#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::new(0x7000ceb310, fn_7000ceb310 as *const (), 1), // +0x5ff310 sub_5ff310
    Lifted::new(0x7000ceb340, fn_7000ceb340 as *const (), 2), // +0x5ff340 sub_5ff340
    Lifted::new(0x7000ceb610, fn_7000ceb610 as *const (), 2), // +0x5ff610 sub_5ff610
    Lifted::new(0x7000ceb900, fn_7000ceb900 as *const (), 2), // +0x5ff900 sub_5ff900
    Lifted::new(0x7000cebc50, fn_7000cebc50 as *const (), 1), // +0x5ffc50 sub_5ffc50
    Lifted::new(0x7000cebc70, fn_7000cebc70 as *const (), 2), // +0x5ffc70 sub_5ffc70
    Lifted::new(0x7000cebdb0, fn_7000cebdb0 as *const (), 2), // +0x5ffdb0 sub_5ffdb0
    Lifted::new(0x7000cec0c0, fn_7000cec0c0 as *const (), 0), // +0x6000c0 sub_6000c0
    Lifted::new(0x7000cec2e0, fn_7000cec2e0 as *const (), 0), // +0x6002e0 sub_6002e0
    Lifted::new(0x7000cec3d0, tpl_1244::<0x3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x6003d0 sub_6003d0
    Lifted::new(0x7000cec4a0, tpl_1245::<0x7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x6004a0 sub_6004a0
    Lifted::new(0x7000cec570, tpl_1246::<0x7001779e20, 0x7000cee230, 0x70017d37c0, 0x70017790e0, 0x7000cec7de> as *const (), 2), // +0x600570 sub_600570
    Lifted::new(0x7000cec880, tpl_1247::<0x7001779e20, 0x7000cee230, 0x70017d37c0, 0x70017790e0, 0x7000cecaff> as *const (), 2), // +0x600880 sub_600880
    Lifted::new(0x7000cecba0, tpl_68 as *const (), 1), // +0x600ba0 sub_600ba0
    Lifted::new(0x7000cecc80, tpl_91::<0x8, 0x10, 0x20, 0x7001779e20, 0x18, 0x30, 0x38, 0x7000cede50> as *const (), 0), // +0x600c80 sub_600c80
    Lifted::new(0x7000cecd90, fn_7000cecd90 as *const (), 0), // +0x600d90 sub_600d90
    Lifted::new(0x7000ceced0, fn_7000ceced0 as *const (), 2), // +0x600ed0 sub_600ed0
    Lifted::new(0x7000ced330, fn_7000ced330 as *const (), 1), // +0x601330 sub_601330
    Lifted::new(0x7000ced350, fn_7000ced350 as *const (), 0), // +0x601350 sub_601350
    Lifted::new(0x7000ced590, fn_7000ced590 as *const (), 0), // +0x601590 sub_601590
    Lifted::new(0x7000ced810, fn_7000ced810 as *const (), 0), // +0x601810 sub_601810
    Lifted::new(0x7000ced910, fn_7000ced910 as *const (), 0), // +0x601910 sub_601910
    Lifted::new(0x7000cedc10, fn_7000cedc10 as *const (), 0), // +0x601c10 sub_601c10
    Lifted::new(0x7000cede50, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000cedf57, 0x7000cecba0, 0x70007f0910, 0x7000cedf73> as *const (), 0), // +0x601e50 sub_601e50
    Lifted::new(0x7000cedfc0, tpl_286::<0x7000cede50, 0x70017d37a0, 0x70017790e0, 0x7000cee1ab> as *const (), 0), // +0x601fc0 sub_601fc0
    Lifted::new(0x7000cee230, tpl_897::<0x10, 0x38, 0x700177c170, 0x70017d37ec, 0x7000cedfc0> as *const (), 2), // +0x602230 sub_602230
    Lifted::new(0x7000cee340, tpl_1248::<0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x602340 sub_602340
    Lifted::new(0x7000cee480, fn_7000cee480 as *const (), 0), // +0x602480 sub_602480
    Lifted::new(0x7000cee4f0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x6024f0 sub_6024f0
    Lifted::new(0x7000cee530, tpl_906::<0x7000d09930, 0x700177c312, 0x7001d54968, 0x7000cee6b4, 0x7000d097f0, 0x7000cee6ba> as *const (), 0), // +0x602530 sub_602530
    Lifted::new(0x7000cee720, tpl_388::<0x7000d099c0, 0x700177c312, 0x7001d54968, 0x7000cee8aa, 0x7000d09810, 0x7000cee8b0> as *const (), 2), // +0x602720 sub_602720
    Lifted::new(0x7000cee920, fn_7000cee920 as *const (), 0), // +0x602920 sub_602920
    Lifted::new(0x7000ceeac0, fn_7000ceeac0 as *const (), 0), // +0x602ac0 sub_602ac0
    Lifted::new(0x7000ceeb80, tpl_1224::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000ceebdd> as *const (), 0), // +0x602b80 sub_602b80
    Lifted::new(0x7000ceec00, tpl_1149::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000ceec79> as *const (), 0), // +0x602c00 sub_602c00
    Lifted::new(0x7000ceeca0, tpl_1149::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000ceed19> as *const (), 0), // +0x602ca0 sub_602ca0
    Lifted::new(0x7000ceed40, tpl_1149::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000ceedb9> as *const (), 0), // +0x602d40 sub_602d40
    Lifted::new(0x7000ceede0, tpl_1149::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000ceee59> as *const (), 0), // +0x602de0 sub_602de0
    Lifted::new(0x7000ceee80, tpl_1149::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000ceeef9> as *const (), 0), // +0x602e80 sub_602e80
    Lifted::new(0x7000ceef20, tpl_1149::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000ceef99> as *const (), 0), // +0x602f20 sub_602f20
    Lifted::new(0x7000ceefc0, tpl_1149::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000cef039> as *const (), 0), // +0x602fc0 sub_602fc0
    Lifted::new(0x7000cef060, tpl_1149::<0x30, 0x40, 0xfffffffffffffff8, 0x700177c312, 0x38, 0x7001d54968, 0x7000cef0d9> as *const (), 0), // +0x603060 sub_603060
    Lifted::new(0x7000cef100, fn_7000cef100 as *const (), 0), // +0x603100 sub_603100
    Lifted::new(0x7000cef1b0, fn_7000cef1b0 as *const (), 0), // +0x6031b0 sub_6031b0
    Lifted::new(0x7000cef310, fn_7000cef310 as *const (), 0), // +0x603310 sub_603310
    Lifted::new(0x7000cef380, fn_7000cef380 as *const (), 2), // +0x603380 sub_603380
    Lifted::new(0x7000cef5b0, fn_7000cef5b0 as *const (), 2), // +0x6035b0 sub_6035b0
    Lifted::new(0x7000cef7a0, fn_7000cef7a0 as *const (), 2), // +0x6037a0 sub_6037a0
    Lifted::new(0x7000cef960, fn_7000cef960 as *const (), 2), // +0x603960 sub_603960
    Lifted::new(0x7000cefb50, fn_7000cefb50 as *const (), 2), // +0x603b50 sub_603b50
    Lifted::new(0x7000cefd00, fn_7000cefd00 as *const (), 0), // +0x603d00 sub_603d00
    Lifted::new(0x7000cefed0, fn_7000cefed0 as *const (), 0), // +0x603ed0 sub_603ed0
    Lifted::new(0x7000ceff30, fn_7000ceff30 as *const (), 0), // +0x603f30 sub_603f30
    Lifted::new(0x7000cf00c0, fn_7000cf00c0 as *const (), 0), // +0x6040c0 sub_6040c0
    Lifted::new(0x7000cf02c0, fn_7000cf02c0 as *const (), 2), // +0x6042c0 sub_6042c0
    Lifted::new(0x7000cf1b50, fn_7000cf1b50 as *const (), 2), // +0x605b50 sub_605b50
    Lifted::new(0x7000cf2d70, fn_7000cf2d70 as *const (), 0), // +0x606d70 sub_606d70
    Lifted::new(0x7000cf2dd0, fn_7000cf2dd0 as *const (), 2), // +0x606dd0 sub_606dd0
    Lifted::new(0x7000cf3240, fn_7000cf3240 as *const (), 0), // +0x607240 sub_607240
    Lifted::new(0x7000cf32f0, fn_7000cf32f0 as *const (), 0), // +0x6072f0 sub_6072f0
    Lifted::new(0x7000cf33d0, fn_7000cf33d0 as *const (), 0), // +0x6073d0 sub_6073d0
    Lifted::new(0x7000cf3450, fn_7000cf3450 as *const (), 2), // +0x607450 sub_607450
    Lifted::new(0x7000cf3580, fn_7000cf3580 as *const (), 0), // +0x607580 sub_607580
    Lifted::new(0x7000cf39a0, fn_7000cf39a0 as *const (), 0), // +0x6079a0 sub_6079a0
    Lifted::new(0x7000cf3a20, fn_7000cf3a20 as *const (), 1), // +0x607a20 sub_607a20
    Lifted::new(0x7000cf3b50, fn_7000cf3b50 as *const (), 0), // +0x607b50 sub_607b50
    Lifted::new(0x7000cf3c70, fn_7000cf3c70 as *const (), 1), // +0x607c70 sub_607c70
    Lifted::new(0x7000cf3d00, fn_7000cf3d00 as *const (), 2), // +0x607d00 sub_607d00
    Lifted::new(0x7000cf3ea0, fn_7000cf3ea0 as *const (), 2), // +0x607ea0 sub_607ea0
    Lifted::new(0x7000cf4230, fn_7000cf4230 as *const (), 0), // +0x608230 sub_608230
    Lifted::new(0x7000cf4690, fn_7000cf4690 as *const (), 0), // +0x608690 sub_608690
    Lifted::new(0x7000cf4b00, fn_7000cf4b00 as *const (), 0), // +0x608b00 sub_608b00
    Lifted::new(0x7000cf5360, fn_7000cf5360 as *const (), 0), // +0x609360 sub_609360
    Lifted::new(0x7000cf5420, fn_7000cf5420 as *const (), 0), // +0x609420 sub_609420
    Lifted::new(0x7000cf59c0, fn_7000cf59c0 as *const (), 2), // +0x6099c0 sub_6099c0
    Lifted::new(0x7000cf5bf0, fn_7000cf5bf0 as *const (), 0), // +0x609bf0 sub_609bf0
    Lifted::new(0x7000cf5cc0, fn_7000cf5cc0 as *const (), 0), // +0x609cc0 sub_609cc0
    Lifted::new(0x7000cf5d50, fn_7000cf5d50 as *const (), 0), // +0x609d50 sub_609d50
    Lifted::new(0x7000cf5de0, tpl_1249::<0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x609de0 sub_609de0
    Lifted::new(0x7000cf5e70, fn_7000cf5e70 as *const (), 0), // +0x609e70 sub_609e70
    Lifted::new(0x7000cf5f00, fn_7000cf5f00 as *const (), 0), // +0x609f00 sub_609f00
    Lifted::new(0x7000cf5f90, fn_7000cf5f90 as *const (), 0), // +0x609f90 sub_609f90
    Lifted::new(0x7000cf63f0, fn_7000cf63f0 as *const (), 0), // +0x60a3f0 sub_60a3f0
    Lifted::new(0x7000cf65f0, fn_7000cf65f0 as *const (), 0), // +0x60a5f0 sub_60a5f0
    Lifted::new(0x7000cf67e0, fn_7000cf67e0 as *const (), 0), // +0x60a7e0 sub_60a7e0
    Lifted::new(0x7000cf6860, fn_7000cf6860 as *const (), 0), // +0x60a860 sub_60a860
    Lifted::new(0x7000cf6a90, fn_7000cf6a90 as *const (), 0), // +0x60aa90 sub_60aa90
    Lifted::new(0x7000cf6ca0, fn_7000cf6ca0 as *const (), 0), // +0x60aca0 sub_60aca0
    Lifted::new(0x7000cf6d30, fn_7000cf6d30 as *const (), 0), // +0x60ad30 sub_60ad30
    Lifted::new(0x7000cf6e20, fn_7000cf6e20 as *const (), 0), // +0x60ae20 sub_60ae20
    Lifted::new(0x7000cf6e90, fn_7000cf6e90 as *const (), 0), // +0x60ae90 sub_60ae90
    Lifted::new(0x7000cf7180, fn_7000cf7180 as *const (), 0), // +0x60b180 sub_60b180
    Lifted::new(0x7000cf7360, fn_7000cf7360 as *const (), 0), // +0x60b360 sub_60b360
    Lifted::new(0x7000cf76e0, fn_7000cf76e0 as *const (), 2), // +0x60b6e0 sub_60b6e0
    Lifted::new(0x7000cf7a50, fn_7000cf7a50 as *const (), 2), // +0x60ba50 sub_60ba50
    Lifted::new(0x7000cf7dc0, fn_7000cf7dc0 as *const (), 2), // +0x60bdc0 sub_60bdc0
    Lifted::new(0x7000cf83d0, fn_7000cf83d0 as *const (), 0), // +0x60c3d0 sub_60c3d0
    Lifted::new(0x7000cf84d0, fn_7000cf84d0 as *const (), 0), // +0x60c4d0 sub_60c4d0
    Lifted::new(0x7000cf8800, fn_7000cf8800 as *const (), 2), // +0x60c800 sub_60c800
    Lifted::new(0x7000cf8be0, fn_7000cf8be0 as *const (), 2), // +0x60cbe0 sub_60cbe0
    Lifted::new(0x7000cf8ec0, fn_7000cf8ec0 as *const (), 2), // +0x60cec0 sub_60cec0
    Lifted::new(0x7000cf9610, fn_7000cf9610 as *const (), 2), // +0x60d610 sub_60d610
    Lifted::new(0x7000cf9910, fn_7000cf9910 as *const (), 0), // +0x60d910 sub_60d910
    Lifted::new(0x7000cf9d40, fn_7000cf9d40 as *const (), 2), // +0x60dd40 sub_60dd40
    Lifted::new(0x7000cfa080, fn_7000cfa080 as *const (), 2), // +0x60e080 sub_60e080
    Lifted::new(0x7000cfa300, fn_7000cfa300 as *const (), 0), // +0x60e300 sub_60e300
    Lifted::new(0x7000cfa570, fn_7000cfa570 as *const (), 2), // +0x60e570 sub_60e570
    Lifted::new(0x7000cfa7d0, fn_7000cfa7d0 as *const (), 2), // +0x60e7d0 sub_60e7d0
    Lifted::new(0x7000cfab70, fn_7000cfab70 as *const (), 0), // +0x60eb70 sub_60eb70
    Lifted::new(0x7000cfb110, fn_7000cfb110 as *const (), 2), // +0x60f110 sub_60f110
    Lifted::new(0x7000cfb4f0, fn_7000cfb4f0 as *const (), 2), // +0x60f4f0 sub_60f4f0
    Lifted::new(0x7000cfb7d0, fn_7000cfb7d0 as *const (), 0), // +0x60f7d0 sub_60f7d0
    Lifted::new(0x7000cfb8a0, fn_7000cfb8a0 as *const (), 0), // +0x60f8a0 sub_60f8a0
    Lifted::new(0x7000cfbcf0, fn_7000cfbcf0 as *const (), 2), // +0x60fcf0 sub_60fcf0
    Lifted::new(0x7000cfc110, fn_7000cfc110 as *const (), 0), // +0x610110 sub_610110
    Lifted::new(0x7000cfc140, fn_7000cfc140 as *const (), 0), // +0x610140 sub_610140
    Lifted::new(0x7000cfc340, fn_7000cfc340 as *const (), 2), // +0x610340 sub_610340
    Lifted::new(0x7000cfc7a0, fn_7000cfc7a0 as *const (), 2), // +0x6107a0 sub_6107a0
    Lifted::new(0x7000cfcae0, fn_7000cfcae0 as *const (), 2), // +0x610ae0 sub_610ae0
    Lifted::new(0x7000cfd010, fn_7000cfd010 as *const (), 2), // +0x611010 sub_611010
    Lifted::new(0x7000cfd4e0, fn_7000cfd4e0 as *const (), 2), // +0x6114e0 sub_6114e0
    Lifted::new(0x7000cfd9b0, fn_7000cfd9b0 as *const (), 2), // +0x6119b0 sub_6119b0
    Lifted::new(0x7000cfe170, fn_7000cfe170 as *const (), 2), // +0x612170 sub_612170
    Lifted::new(0x7000cfe4c0, fn_7000cfe4c0 as *const (), 2), // +0x6124c0 sub_6124c0
    Lifted::new(0x7000cfe970, fn_7000cfe970 as *const (), 2), // +0x612970 sub_612970
    Lifted::new(0x7000cfeed0, fn_7000cfeed0 as *const (), 2), // +0x612ed0 sub_612ed0
    Lifted::new(0x7000cff320, fn_7000cff320 as *const (), 2), // +0x613320 sub_613320
    Lifted::new(0x7000cffc20, fn_7000cffc20 as *const (), 2), // +0x613c20 sub_613c20
    Lifted::new(0x7000d000a0, fn_7000d000a0 as *const (), 2), // +0x6140a0 sub_6140a0
    Lifted::new(0x7000d00650, fn_7000d00650 as *const (), 2), // +0x614650 sub_614650
    Lifted::new(0x7000d00b00, fn_7000d00b00 as *const (), 2), // +0x614b00 sub_614b00
    Lifted::new(0x7000d00ee0, fn_7000d00ee0 as *const (), 2), // +0x614ee0 sub_614ee0
    Lifted::new(0x7000d012c0, fn_7000d012c0 as *const (), 2), // +0x6152c0 sub_6152c0
    Lifted::new(0x7000d01690, fn_7000d01690 as *const (), 2), // +0x615690 sub_615690
    Lifted::new(0x7000d01b90, fn_7000d01b90 as *const (), 2), // +0x615b90 sub_615b90
    Lifted::new(0x7000d022a0, fn_7000d022a0 as *const (), 2), // +0x6162a0 sub_6162a0
    Lifted::new(0x7000d027c0, fn_7000d027c0 as *const (), 2), // +0x6167c0 sub_6167c0
    Lifted::new(0x7000d02c00, fn_7000d02c00 as *const (), 2), // +0x616c00 sub_616c00
    Lifted::new(0x7000d02f00, fn_7000d02f00 as *const (), 2), // +0x616f00 sub_616f00
    Lifted::new(0x7000d034e0, fn_7000d034e0 as *const (), 0), // +0x6174e0 sub_6174e0
    Lifted::new(0x7000d03d90, fn_7000d03d90 as *const (), 0), // +0x617d90 sub_617d90
    Lifted::new(0x7000d042f0, fn_7000d042f0 as *const (), 0), // +0x6182f0 sub_6182f0
    Lifted::new(0x7000d047e0, fn_7000d047e0 as *const (), 0), // +0x6187e0 sub_6187e0
    Lifted::new(0x7000d04b40, fn_7000d04b40 as *const (), 2), // +0x618b40 sub_618b40
    Lifted::new(0x7000d05030, fn_7000d05030 as *const (), 2), // +0x619030 sub_619030
    Lifted::new(0x7000d057f0, fn_7000d057f0 as *const (), 0), // +0x6197f0 sub_6197f0
    Lifted::new(0x7000d058e0, fn_7000d058e0 as *const (), 0), // +0x6198e0 sub_6198e0
    Lifted::new(0x7000d05980, fn_7000d05980 as *const (), 0), // +0x619980 sub_619980
    Lifted::new(0x7000d05a00, fn_7000d05a00 as *const (), 2), // +0x619a00 sub_619a00
    Lifted::new(0x7000d062f0, fn_7000d062f0 as *const (), 0), // +0x61a2f0 sub_61a2f0
    Lifted::new(0x7000d063a0, fn_7000d063a0 as *const (), 2), // +0x61a3a0 sub_61a3a0
    Lifted::new(0x7000d06600, fn_7000d06600 as *const (), 0), // +0x61a600 sub_61a600
    Lifted::new(0x7000d066e0, fn_7000d066e0 as *const (), 0), // +0x61a6e0 sub_61a6e0
    Lifted::new(0x7000d069f0, fn_7000d069f0 as *const (), 2), // +0x61a9f0 sub_61a9f0
    Lifted::new(0x7000d06c40, fn_7000d06c40 as *const (), 2), // +0x61ac40 sub_61ac40
    Lifted::new(0x7000d07510, fn_7000d07510 as *const (), 2), // +0x61b510 sub_61b510
    Lifted::new(0x7000d07780, fn_7000d07780 as *const (), 0), // +0x61b780 sub_61b780
    Lifted::new(0x7000d07860, fn_7000d07860 as *const (), 2), // +0x61b860 sub_61b860
    Lifted::new(0x7000d08590, fn_7000d08590 as *const (), 0), // +0x61c590 sub_61c590
    Lifted::new(0x7000d08650, fn_7000d08650 as *const (), 2), // +0x61c650 sub_61c650
    Lifted::new(0x7000d09640, tpl_999::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000d096f0> as *const (), 0), // +0x61d640 sub_61d640
    Lifted::new(0x7000d09720, tpl_753::<0x7001828a18, 0x8> as *const (), 1), // +0x61d720 sub_61d720
    Lifted::new(0x7000d09740, tpl_754::<0x700177c312> as *const (), 0), // +0x61d740 sub_61d740
    Lifted::new(0x7000d09760, fn_7000d09760 as *const (), 0), // +0x61d760 sub_61d760
    Lifted::new(0x7000d09790, tpl_52::<0x8> as *const (), 1), // +0x61d790 sub_61d790
    Lifted::new(0x7000d097a0, tpl_753::<0x7001828a18, 0x8> as *const (), 1), // +0x61d7a0 sub_61d7a0
    Lifted::new(0x7000d097c0, tpl_315::<0x7001c6bfb0> as *const (), 1), // +0x61d7c0 sub_61d7c0
    Lifted::new(0x7000d097d0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d097e0> as *const (), 0), // +0x61d7d0 sub_61d7d0
    Lifted::new(0x7000d097f0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d09800> as *const (), 0), // +0x61d7f0 sub_61d7f0
    Lifted::new(0x7000d09810, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d09820> as *const (), 0), // +0x61d810 sub_61d810
    Lifted::new(0x7000d09830, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000d09840> as *const (), 0), // +0x61d830 sub_61d830
    Lifted::new(0x7000d09850, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000d09860> as *const (), 0), // +0x61d850 sub_61d850
    Lifted::new(0x7000d09870, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000d09880> as *const (), 0), // +0x61d870 sub_61d870
    Lifted::new(0x7000d09890, tpl_1001::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000d098e8, 0x70007f0910, 0x7000d09903> as *const (), 0), // +0x61d890 sub_61d890
    Lifted::new(0x7000d09930, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000d09984, 0x70007f0910, 0x7000d0999f> as *const (), 0), // +0x61d930 sub_61d930
    Lifted::new(0x7000d099c0, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000d09a0d, 0x70007f0910, 0x7000d09a2b> as *const (), 0), // +0x61d9c0 sub_61d9c0
    Lifted::new(0x7000d09a50, tpl_1002::<0x7000d09890, 0x7000d09640, 0x7000d097d0, 0x7000d09bf8> as *const (), 2), // +0x61da50 sub_61da50
    Lifted::new(0x7000d09cb0, fn_7000d09cb0 as *const (), 0), // +0x61dcb0 sub_61dcb0
    Lifted::new(0x7000d09d80, fn_7000d09d80 as *const (), 0), // +0x61dd80 sub_61dd80
    Lifted::new(0x7000d09dd0, fn_7000d09dd0 as *const (), 1), // +0x61ddd0 sub_61ddd0
    Lifted::new(0x7000d09e30, fn_7000d09e30 as *const (), 0), // +0x61de30 sub_61de30
    Lifted::new(0x7000d09ee0, fn_7000d09ee0 as *const (), 1), // +0x61dee0 sub_61dee0
    Lifted::new(0x7000d09fc0, tpl_1250::<0x3> as *const (), 1), // +0x61dfc0 sub_61dfc0
    Lifted::new(0x7000d09fe0, tpl_1250::<0x3> as *const (), 1), // +0x61dfe0 sub_61dfe0
    Lifted::new(0x7000d0a000, tpl_1251::<0xfffffffffffffee5> as *const (), 1), // +0x61e000 sub_61e000
    Lifted::new(0x7000d0a020, tpl_1251::<0xfffffffffffffee5> as *const (), 1), // +0x61e020 sub_61e020
    Lifted::new(0x7000d0a040, fn_7000d0a040 as *const (), 2), // +0x61e040 sub_61e040
    Lifted::new(0x7000d0a1d0, fn_7000d0a1d0 as *const (), 1), // +0x61e1d0 sub_61e1d0
    Lifted::new(0x7000d0a350, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x61e350 sub_61e350
    Lifted::new(0x7000d0a460, fn_7000d0a460 as *const (), 1), // +0x61e460 sub_61e460
    Lifted::new(0x7000d0a4d0, tpl_580::<0x10, 0x8, 0x7000d127e0> as *const (), 0), // +0x61e4d0 sub_61e4d0
    Lifted::new(0x7000d0a5b0, tpl_581::<0x10, 0x8, 0x7000d127e0> as *const (), 0), // +0x61e5b0 sub_61e5b0
    Lifted::new(0x7000d0a710, tpl_1252::<0x7001779e20, 0x7001828b08, 0x38, 0x7001d54cd8, 0x7001828af8, 0x700070d620, 0x7001828ae0> as *const (), 0), // +0x61e710 sub_61e710
    Lifted::new(0x7000d0a800, tpl_1253::<0x7001779e20, 0x700180c698, 0x7001828b40, 0x700070d620, 0x7001828b28> as *const (), 0), // +0x61e800 sub_61e800
    Lifted::new(0x7000d0a8a0, tpl_1252::<0x7001779e20, 0x7001828b08, 0x38, 0x7001d54cd8, 0x7001828b6c, 0x700070d620, 0x7001828b58> as *const (), 0), // +0x61e8a0 sub_61e8a0
    Lifted::new(0x7000d0a990, tpl_1253::<0x7001779e20, 0x700180c900, 0x7001828b90, 0x700070d620, 0x7001828b78> as *const (), 0), // +0x61e990 sub_61e990
    Lifted::new(0x7000d0aa30, tpl_1253::<0x7001779e20, 0x700180c990, 0x7001828bc8, 0x700070d620, 0x7001828bb0> as *const (), 0), // +0x61ea30 sub_61ea30
    Lifted::new(0x7000d0aad0, tpl_1253::<0x7001779e20, 0x700180d500, 0x7001828bf0, 0x700070d620, 0x7001828bd8> as *const (), 0), // +0x61ead0 sub_61ead0
    Lifted::new(0x7000d0ab70, tpl_562::<0x8, 0x10, 0x7000d13030> as *const (), 0), // +0x61eb70 sub_61eb70
    Lifted::new(0x7000d0aba0, fn_7000d0aba0 as *const (), 1), // +0x61eba0 sub_61eba0
    Lifted::new(0x7000d0abc0, fn_7000d0abc0 as *const (), 0), // +0x61ebc0 sub_61ebc0
    Lifted::new(0x7000d0ac60, fn_7000d0ac60 as *const (), 2), // +0x61ec60 sub_61ec60
    Lifted::new(0x7000d0b090, fn_7000d0b090 as *const (), 2), // +0x61f090 sub_61f090
    Lifted::new(0x7000d0ba10, fn_7000d0ba10 as *const (), 0), // +0x61fa10 sub_61fa10
    Lifted::new(0x7000d0baf0, tpl_34::<0x70017a0080, 0x18, 0x700177c312> as *const (), 0), // +0x61faf0 sub_61faf0
    Lifted::new(0x7000d0bb50, tpl_34::<0x70017a0080, 0x18, 0x700177c312> as *const (), 0), // +0x61fb50 sub_61fb50
    Lifted::new(0x7000d0bbb0, tpl_34::<0x70017a0080, 0x18, 0x700177c312> as *const (), 0), // +0x61fbb0 sub_61fbb0
    Lifted::new(0x7000d0bc10, tpl_34::<0x70017a0080, 0x18, 0x700177c312> as *const (), 0), // +0x61fc10 sub_61fc10
    Lifted::new(0x7000d0bc70, tpl_34::<0x70017a0080, 0x18, 0x700177c312> as *const (), 0), // +0x61fc70 sub_61fc70
    Lifted::new(0x7000d0bcd0, tpl_34::<0x70017a0080, 0x18, 0x700177c312> as *const (), 0), // +0x61fcd0 sub_61fcd0
    Lifted::new(0x7000d0bd30, fn_7000d0bd30 as *const (), 2), // +0x61fd30 sub_61fd30
    Lifted::new(0x7000d0bf10, fn_7000d0bf10 as *const (), 2), // +0x61ff10 sub_61ff10
    Lifted::new(0x7000d0c1a0, fn_7000d0c1a0 as *const (), 0), // +0x6201a0 sub_6201a0
    Lifted::new(0x7000d0c220, tpl_43 as *const (), 1), // +0x620220 sub_620220
    Lifted::new(0x7000d0c230, fn_7000d0c230 as *const (), 1), // +0x620230 sub_620230
    Lifted::new(0x7000d0c290, fn_7000d0c290 as *const (), 2), // +0x620290 sub_620290
    Lifted::new(0x7000d0c8d0, fn_7000d0c8d0 as *const (), 0), // +0x6208d0 sub_6208d0
    Lifted::new(0x7000d0ca70, fn_7000d0ca70 as *const (), 0), // +0x620a70 sub_620a70
    Lifted::new(0x7000d0ce50, fn_7000d0ce50 as *const (), 0), // +0x620e50 sub_620e50
    Lifted::new(0x7000d0d140, fn_7000d0d140 as *const (), 2), // +0x621140 sub_621140
    Lifted::new(0x7000d0d580, fn_7000d0d580 as *const (), 2), // +0x621580 sub_621580
    Lifted::new(0x7000d0e110, fn_7000d0e110 as *const (), 2), // +0x622110 sub_622110
    Lifted::new(0x7000d0e4e0, fn_7000d0e4e0 as *const (), 2), // +0x6224e0 sub_6224e0
    Lifted::new(0x7000d0e790, fn_7000d0e790 as *const (), 0), // +0x622790 sub_622790
    Lifted::new(0x7000d0e860, fn_7000d0e860 as *const (), 0), // +0x622860 sub_622860
    Lifted::new(0x7000d0ea40, fn_7000d0ea40 as *const (), 0), // +0x622a40 sub_622a40
    Lifted::new(0x7000d0eb90, fn_7000d0eb90 as *const (), 0), // +0x622b90 sub_622b90
    Lifted::new(0x7000d0ee80, fn_7000d0ee80 as *const (), 0), // +0x622e80 sub_622e80
    Lifted::new(0x7000d0f060, fn_7000d0f060 as *const (), 2), // +0x623060 sub_623060
    Lifted::new(0x7000d0f5c0, fn_7000d0f5c0 as *const (), 2), // +0x6235c0 sub_6235c0
    Lifted::new(0x7000d10380, fn_7000d10380 as *const (), 2), // +0x624380 sub_624380
    Lifted::new(0x7000d10980, fn_7000d10980 as *const (), 2), // +0x624980 sub_624980
    Lifted::new(0x7000d11210, fn_7000d11210 as *const (), 0), // +0x625210 sub_625210
    Lifted::new(0x7000d11410, fn_7000d11410 as *const (), 2), // +0x625410 sub_625410
    Lifted::new(0x7000d11aa0, fn_7000d11aa0 as *const (), 0), // +0x625aa0 sub_625aa0
    Lifted::new(0x7000d11c10, fn_7000d11c10 as *const (), 2), // +0x625c10 sub_625c10
    Lifted::new(0x7000d11fa0, fn_7000d11fa0 as *const (), 2), // +0x625fa0 sub_625fa0
    Lifted::new(0x7000d124e0, fn_7000d124e0 as *const (), 2), // +0x6264e0 sub_6264e0
    Lifted::new(0x7000d127e0, fn_7000d127e0 as *const (), 0), // +0x6267e0 sub_6267e0
    Lifted::new(0x7000d12990, tpl_15 as *const (), 1), // +0x626990 sub_626990
    Lifted::new(0x7000d129a0, tpl_1254::<0x8, 0x700074a010> as *const (), 0), // +0x6269a0 sub_6269a0
    Lifted::new(0x7000d129d0, tpl_1254::<0x8, 0x700074a010> as *const (), 0), // +0x6269d0 sub_6269d0
    Lifted::new(0x7000d12a00, fn_7000d12a00 as *const (), 0), // +0x626a00 sub_626a00
    Lifted::new(0x7000d12a30, fn_7000d12a30 as *const (), 0), // +0x626a30 sub_626a30
    Lifted::new(0x7000d12be0, fn_7000d12be0 as *const (), 2), // +0x626be0 sub_626be0
    Lifted::new(0x7000d12e50, fn_7000d12e50 as *const (), 0), // +0x626e50 sub_626e50
    Lifted::new(0x7000d13030, tpl_477::<0x7001d55060, 0x8, 0x70017d8de0, 0x7001d54ef8, 0x7000d13063, 0x10, 0x18, 0xffffffffffffffe8, 0x14> as *const (), 0), // +0x627030 sub_627030
    Lifted::new(0x7000d130d0, fn_7000d130d0 as *const (), 1), // +0x6270d0 sub_6270d0
    Lifted::new(0x7000d13120, fn_7000d13120 as *const (), 0), // +0x627120 sub_627120
    Lifted::new(0x7000d136a0, fn_7000d136a0 as *const (), 0), // +0x6276a0 sub_6276a0
    Lifted::new(0x7000d138c0, fn_7000d138c0 as *const (), 2), // +0x6278c0 sub_6278c0
    Lifted::new(0x7000d1fdf0, fn_7000d1fdf0 as *const (), 0), // +0x633df0 sub_633df0
    Lifted::new(0x7000d201c0, fn_7000d201c0 as *const (), 0), // +0x6341c0 sub_6341c0
    Lifted::new(0x7000d20450, fn_7000d20450 as *const (), 0), // +0x634450 sub_634450
    Lifted::new(0x7000d20570, fn_7000d20570 as *const (), 0), // +0x634570 sub_634570
    Lifted::new(0x7000d208a0, fn_7000d208a0 as *const (), 0), // +0x6348a0 sub_6348a0
    Lifted::new(0x7000d20d90, fn_7000d20d90 as *const (), 0), // +0x634d90 sub_634d90
    Lifted::new(0x7000d20eb0, fn_7000d20eb0 as *const (), 0), // +0x634eb0 sub_634eb0
    Lifted::new(0x7000d21110, fn_7000d21110 as *const (), 0), // +0x635110 sub_635110
    Lifted::new(0x7000d21350, fn_7000d21350 as *const (), 2), // +0x635350 sub_635350
    Lifted::new(0x7000d22610, fn_7000d22610 as *const (), 2), // +0x636610 sub_636610
    Lifted::new(0x7000d22880, fn_7000d22880 as *const (), 0), // +0x636880 sub_636880
    Lifted::new(0x7000d22c00, fn_7000d22c00 as *const (), 2), // +0x636c00 sub_636c00
    Lifted::new(0x7000d244e0, fn_7000d244e0 as *const (), 0), // +0x6384e0 sub_6384e0
    Lifted::new(0x7000d24800, fn_7000d24800 as *const (), 0), // +0x638800 sub_638800
    Lifted::new(0x7000d24890, fn_7000d24890 as *const (), 0), // +0x638890 sub_638890
    Lifted::new(0x7000d249a0, fn_7000d249a0 as *const (), 2), // +0x6389a0 sub_6389a0
    Lifted::new(0x7000d251a0, fn_7000d251a0 as *const (), 0), // +0x6391a0 sub_6391a0
    Lifted::new(0x7000d25410, fn_7000d25410 as *const (), 0), // +0x639410 sub_639410
    Lifted::new(0x7000d254c0, fn_7000d254c0 as *const (), 0), // +0x6394c0 sub_6394c0
    Lifted::new(0x7000d25570, fn_7000d25570 as *const (), 0), // +0x639570 sub_639570
    Lifted::new(0x7000d25a00, fn_7000d25a00 as *const (), 0), // +0x639a00 sub_639a00
    Lifted::new(0x7000d25bd0, fn_7000d25bd0 as *const (), 0), // +0x639bd0 sub_639bd0
    Lifted::new(0x7000d25cb0, fn_7000d25cb0 as *const (), 0), // +0x639cb0 sub_639cb0
    Lifted::new(0x7000d25d60, fn_7000d25d60 as *const (), 0), // +0x639d60 sub_639d60
    Lifted::new(0x7000d25e50, fn_7000d25e50 as *const (), 0), // +0x639e50 sub_639e50
    Lifted::new(0x7000d25f20, fn_7000d25f20 as *const (), 2), // +0x639f20 sub_639f20
    Lifted::new(0x7000d261c0, fn_7000d261c0 as *const (), 0), // +0x63a1c0 sub_63a1c0
    Lifted::new(0x7000d262c0, fn_7000d262c0 as *const (), 0), // +0x63a2c0 sub_63a2c0
    Lifted::new(0x7000d26490, fn_7000d26490 as *const (), 0), // +0x63a490 sub_63a490
    Lifted::new(0x7000d264e0, fn_7000d264e0 as *const (), 0), // +0x63a4e0 sub_63a4e0
    Lifted::new(0x7000d26550, fn_7000d26550 as *const (), 0), // +0x63a550 sub_63a550
    Lifted::new(0x7000d26580, fn_7000d26580 as *const (), 0), // +0x63a580 sub_63a580
    Lifted::new(0x7000d26610, fn_7000d26610 as *const (), 0), // +0x63a610 sub_63a610
    Lifted::new(0x7000d26d80, fn_7000d26d80 as *const (), 0), // +0x63ad80 sub_63ad80
    Lifted::new(0x7000d26f60, fn_7000d26f60 as *const (), 0), // +0x63af60 sub_63af60
    Lifted::new(0x7000d27060, fn_7000d27060 as *const (), 2), // +0x63b060 sub_63b060
    Lifted::new(0x7000d27200, tpl_581::<0x10, 0x8, 0x7000d127e0> as *const (), 0), // +0x63b200 sub_63b200
    Lifted::new(0x7000d272e0, fn_7000d272e0 as *const (), 0), // +0x63b2e0 sub_63b2e0
    Lifted::new(0x7000d274b0, fn_7000d274b0 as *const (), 0), // +0x63b4b0 sub_63b4b0
    Lifted::new(0x7000d27530, fn_7000d27530 as *const (), 0), // +0x63b530 sub_63b530
    Lifted::new(0x7000d276c0, fn_7000d276c0 as *const (), 0), // +0x63b6c0 sub_63b6c0
    Lifted::new(0x7000d277b0, fn_7000d277b0 as *const (), 1), // +0x63b7b0 sub_63b7b0
    Lifted::new(0x7000d277c0, fn_7000d277c0 as *const (), 0), // +0x63b7c0 sub_63b7c0
    Lifted::new(0x7000d278e0, fn_7000d278e0 as *const (), 0), // +0x63b8e0 sub_63b8e0
    Lifted::new(0x7000d279d0, fn_7000d279d0 as *const (), 0), // +0x63b9d0 sub_63b9d0
    Lifted::new(0x7000d27aa0, fn_7000d27aa0 as *const (), 0), // +0x63baa0 sub_63baa0
    Lifted::new(0x7000d27ba0, fn_7000d27ba0 as *const (), 0), // +0x63bba0 sub_63bba0
    Lifted::new(0x7000d27be0, fn_7000d27be0 as *const (), 0), // +0x63bbe0 sub_63bbe0
    Lifted::new(0x7000d27c70, fn_7000d27c70 as *const (), 0), // +0x63bc70 sub_63bc70
    Lifted::new(0x7000d27dc0, tpl_244::<0x18> as *const (), 1), // +0x63bdc0 sub_63bdc0
    Lifted::new(0x7000d27dd0, fn_7000d27dd0 as *const (), 0), // +0x63bdd0 sub_63bdd0
    Lifted::new(0x7000d27f10, fn_7000d27f10 as *const (), 0), // +0x63bf10 sub_63bf10
    Lifted::new(0x7000d28c20, fn_7000d28c20 as *const (), 0), // +0x63cc20 sub_63cc20
    Lifted::new(0x7000d28d70, fn_7000d28d70 as *const (), 1), // +0x63cd70 sub_63cd70
    Lifted::new(0x7000d28e20, tpl_237::<0x14b> as *const (), 1), // +0x63ce20 sub_63ce20
    Lifted::new(0x7000d28e50, tpl_237::<0x1b> as *const (), 1), // +0x63ce50 sub_63ce50
    Lifted::new(0x7000d28e80, fn_7000d28e80 as *const (), 1), // +0x63ce80 sub_63ce80
    Lifted::new(0x7000d28f00, tpl_237::<0xffffffffffffffff> as *const (), 1), // +0x63cf00 sub_63cf00
    Lifted::new(0x7000d28f30, tpl_237::<0x237> as *const (), 1), // +0x63cf30 sub_63cf30
    Lifted::new(0x7000d28f60, fn_7000d28f60 as *const (), 1), // +0x63cf60 sub_63cf60
    Lifted::new(0x7000d29050, tpl_1255::<0xaf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d050 sub_63d050
    Lifted::new(0x7000d29100, tpl_1255::<0x35b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d100 sub_63d100
    Lifted::new(0x7000d291b0, tpl_1255::<0x35f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d1b0 sub_63d1b0
    Lifted::new(0x7000d29260, tpl_1255::<0x363, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d260 sub_63d260
    Lifted::new(0x7000d29310, tpl_1255::<0x367, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d310 sub_63d310
    Lifted::new(0x7000d293c0, tpl_1255::<0xcb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d3c0 sub_63d3c0
    Lifted::new(0x7000d29470, tpl_1255::<0x1fb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d470 sub_63d470
    Lifted::new(0x7000d29520, tpl_1255::<0xd7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d520 sub_63d520
    Lifted::new(0x7000d295d0, tpl_1255::<0xdb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d5d0 sub_63d5d0
    Lifted::new(0x7000d29680, tpl_1255::<0xdf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d680 sub_63d680
    Lifted::new(0x7000d29730, tpl_1255::<0xe3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d730 sub_63d730
    Lifted::new(0x7000d297e0, tpl_1255::<0xf3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d7e0 sub_63d7e0
    Lifted::new(0x7000d29890, tpl_377::<0x7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d890 sub_63d890
    Lifted::new(0x7000d29940, fn_7000d29940 as *const (), 0), // +0x63d940 sub_63d940
    Lifted::new(0x7000d29990, tpl_1255::<0x17, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63d990 sub_63d990
    Lifted::new(0x7000d29a30, tpl_1255::<0x1f3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63da30 sub_63da30
    Lifted::new(0x7000d29ae0, tpl_1255::<0x117, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63dae0 sub_63dae0
    Lifted::new(0x7000d29b90, tpl_1255::<0x393, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63db90 sub_63db90
    Lifted::new(0x7000d29c40, tpl_1255::<0x397, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63dc40 sub_63dc40
    Lifted::new(0x7000d29cf0, tpl_1255::<0x43, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63dcf0 sub_63dcf0
    Lifted::new(0x7000d29d90, tpl_1255::<0x39b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63dd90 sub_63dd90
    Lifted::new(0x7000d29e40, tpl_1244::<0x3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63de40 sub_63de40
    Lifted::new(0x7000d29f10, tpl_1255::<0x12f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63df10 sub_63df10
    Lifted::new(0x7000d29fc0, tpl_1255::<0x133, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63dfc0 sub_63dfc0
    Lifted::new(0x7000d2a070, tpl_1255::<0x137, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e070 sub_63e070
    Lifted::new(0x7000d2a120, tpl_1255::<0x153, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e120 sub_63e120
    Lifted::new(0x7000d2a1d0, tpl_1255::<0x29f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e1d0 sub_63e1d0
    Lifted::new(0x7000d2a280, tpl_1255::<0x15f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e280 sub_63e280
    Lifted::new(0x7000d2a330, tpl_1255::<0x163, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e330 sub_63e330
    Lifted::new(0x7000d2a3e0, tpl_1255::<0x167, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e3e0 sub_63e3e0
    Lifted::new(0x7000d2a490, tpl_1255::<0x16b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e490 sub_63e490
    Lifted::new(0x7000d2a540, tpl_1255::<0x16f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e540 sub_63e540
    Lifted::new(0x7000d2a5f0, tpl_1255::<0x173, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e5f0 sub_63e5f0
    Lifted::new(0x7000d2a6a0, tpl_1255::<0x177, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e6a0 sub_63e6a0
    Lifted::new(0x7000d2a750, fn_7000d2a750 as *const (), 1), // +0x63e750 sub_63e750
    Lifted::new(0x7000d2a760, fn_7000d2a760 as *const (), 1), // +0x63e760 sub_63e760
    Lifted::new(0x7000d2a780, fn_7000d2a780 as *const (), 1), // +0x63e780 sub_63e780
    Lifted::new(0x7000d2a7a0, tpl_1255::<0x1a7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e7a0 sub_63e7a0
    Lifted::new(0x7000d2a850, tpl_1255::<0x1c7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e850 sub_63e850
    Lifted::new(0x7000d2a900, tpl_1255::<0x1cb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e900 sub_63e900
    Lifted::new(0x7000d2a9b0, tpl_1255::<0x1d3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63e9b0 sub_63e9b0
    Lifted::new(0x7000d2aa60, fn_7000d2aa60 as *const (), 0), // +0x63ea60 sub_63ea60
    Lifted::new(0x7000d2aa90, tpl_1255::<0x383, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63ea90 sub_63ea90
    Lifted::new(0x7000d2ab40, tpl_1255::<0x387, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63eb40 sub_63eb40
    Lifted::new(0x7000d2abf0, tpl_1255::<0x38b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63ebf0 sub_63ebf0
    Lifted::new(0x7000d2aca0, tpl_1255::<0x38f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63eca0 sub_63eca0
    Lifted::new(0x7000d2ad50, tpl_1255::<0x33, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63ed50 sub_63ed50
    Lifted::new(0x7000d2adf0, tpl_1255::<0x37, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63edf0 sub_63edf0
    Lifted::new(0x7000d2ae90, fn_7000d2ae90 as *const (), 1), // +0x63ee90 sub_63ee90
    Lifted::new(0x7000d2aec0, tpl_549::<0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63eec0 sub_63eec0
    Lifted::new(0x7000d2af90, tpl_377::<0x13, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63ef90 sub_63ef90
    Lifted::new(0x7000d2b040, tpl_1255::<0x27, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f040 sub_63f040
    Lifted::new(0x7000d2b0e0, tpl_1255::<0x39f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f0e0 sub_63f0e0
    Lifted::new(0x7000d2b190, tpl_1255::<0x3a3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f190 sub_63f190
    Lifted::new(0x7000d2b240, tpl_1255::<0x3a7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f240 sub_63f240
    Lifted::new(0x7000d2b2f0, tpl_1255::<0x3ab, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f2f0 sub_63f2f0
    Lifted::new(0x7000d2b3a0, tpl_1255::<0x263, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f3a0 sub_63f3a0
    Lifted::new(0x7000d2b450, tpl_1255::<0x3f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f450 sub_63f450
    Lifted::new(0x7000d2b4f0, tpl_806::<0x1b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f4f0 sub_63f4f0
    Lifted::new(0x7000d2b5a0, tpl_1255::<0x257, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f5a0 sub_63f5a0
    Lifted::new(0x7000d2b650, tpl_1255::<0x2b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f650 sub_63f650
    Lifted::new(0x7000d2b6f0, tpl_1255::<0x2f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f6f0 sub_63f6f0
    Lifted::new(0x7000d2b790, tpl_1255::<0x26b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f790 sub_63f790
    Lifted::new(0x7000d2b840, tpl_1255::<0x273, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f840 sub_63f840
    Lifted::new(0x7000d2b8f0, tpl_1255::<0x277, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f8f0 sub_63f8f0
    Lifted::new(0x7000d2b9a0, tpl_1255::<0x27b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63f9a0 sub_63f9a0
    Lifted::new(0x7000d2ba50, tpl_1255::<0x27f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63fa50 sub_63fa50
    Lifted::new(0x7000d2bb00, tpl_1255::<0x283, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63fb00 sub_63fb00
    Lifted::new(0x7000d2bbb0, tpl_1255::<0x287, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63fbb0 sub_63fbb0
    Lifted::new(0x7000d2bc60, tpl_1255::<0x297, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63fc60 sub_63fc60
    Lifted::new(0x7000d2bd10, tpl_1255::<0x2df, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63fd10 sub_63fd10
    Lifted::new(0x7000d2bdc0, tpl_1255::<0x29b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63fdc0 sub_63fdc0
    Lifted::new(0x7000d2be70, tpl_1255::<0x1f7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63fe70 sub_63fe70
    Lifted::new(0x7000d2bf20, tpl_1255::<0x2a3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63ff20 sub_63ff20
    Lifted::new(0x7000d2bfd0, tpl_1255::<0x2ab, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x63ffd0 sub_63ffd0
    Lifted::new(0x7000d2c080, tpl_1255::<0x3bb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640080 sub_640080
    Lifted::new(0x7000d2c130, tpl_1255::<0x3bf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640130 sub_640130
    Lifted::new(0x7000d2c1e0, tpl_1255::<0x3c3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x6401e0 sub_6401e0
    Lifted::new(0x7000d2c290, tpl_1245::<0x7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640290 sub_640290
    Lifted::new(0x7000d2c360, tpl_1255::<0x2bf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640360 sub_640360
    Lifted::new(0x7000d2c410, tpl_1255::<0x2c3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640410 sub_640410
    Lifted::new(0x7000d2c4c0, fn_7000d2c4c0 as *const (), 0), // +0x6404c0 sub_6404c0
    Lifted::new(0x7000d2c540, tpl_1255::<0x2c7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640540 sub_640540
    Lifted::new(0x7000d2c5f0, tpl_1255::<0x327, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x6405f0 sub_6405f0
    Lifted::new(0x7000d2c6a0, tpl_1255::<0x32b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x6406a0 sub_6406a0
    Lifted::new(0x7000d2c750, tpl_1255::<0x353, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640750 sub_640750
    Lifted::new(0x7000d2c800, tpl_1255::<0x357, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640800 sub_640800
    Lifted::new(0x7000d2c8b0, tpl_549::<0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x6408b0 sub_6408b0
    Lifted::new(0x7000d2c980, tpl_1255::<0x3eb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640980 sub_640980
    Lifted::new(0x7000d2ca30, tpl_1255::<0x3e7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640a30 sub_640a30
    Lifted::new(0x7000d2cae0, tpl_1255::<0x3e3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640ae0 sub_640ae0
    Lifted::new(0x7000d2cb90, tpl_1255::<0x3ef, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x640b90 sub_640b90
    Lifted::new(0x7000d2cc40, tpl_237::<0x2bf> as *const (), 1), // +0x640c40 sub_640c40
    Lifted::new(0x7000d2cc70, tpl_237::<0x323> as *const (), 1), // +0x640c70 sub_640c70
    Lifted::new(0x7000d2cca0, tpl_44 as *const (), 1), // +0x640ca0 sub_640ca0
    Lifted::new(0x7000d2ccb0, tpl_44 as *const (), 1), // +0x640cb0 sub_640cb0
    Lifted::new(0x7000d2ccc0, fn_7000d2ccc0 as *const (), 0), // +0x640cc0 sub_640cc0
    Lifted::new(0x7000d2cdc0, tpl_17::<0x700177c312> as *const (), 0), // +0x640dc0 sub_640dc0
    Lifted::new(0x7000d2cdd0, fn_7000d2cdd0 as *const (), 0), // +0x640dd0 sub_640dd0
    Lifted::new(0x7000d2ce60, tpl_1256::<0x7001c8e33c, 0x7001c73114, 0x7001c730d8, 0x700177a340, 0x70010f0870, 0x700177a2c0> as *const (), 0), // +0x640e60 sub_640e60
    Lifted::new(0x7000d2cf10, fn_7000d2cf10 as *const (), 0), // +0x640f10 sub_640f10
    Lifted::new(0x7000d2cfc0, tpl_15 as *const (), 1), // +0x640fc0 sub_640fc0
    Lifted::new(0x7000d2cfd0, fn_7000d2cfd0 as *const (), 1), // +0x640fd0 sub_640fd0
    Lifted::new(0x7000d2cff0, tpl_455::<0x7001c73118> as *const (), 1), // +0x640ff0 sub_640ff0
    Lifted::new(0x7000d2d000, fn_7000d2d000 as *const (), 0), // +0x641000 sub_641000
    Lifted::new(0x7000d2d060, fn_7000d2d060 as *const (), 0), // +0x641060 sub_641060
    Lifted::new(0x7000d2d0c0, fn_7000d2d0c0 as *const (), 0), // +0x6410c0 sub_6410c0
    Lifted::new(0x7000d2d9a0, fn_7000d2d9a0 as *const (), 0), // +0x6419a0 sub_6419a0
    Lifted::new(0x7000d2da30, tpl_955::<0x7001c73118> as *const (), 1), // +0x641a30 sub_641a30
    Lifted::new(0x7000d2da40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000d2da51> as *const (), 0), // +0x641a40 sub_641a40
    Lifted::new(0x7000d2da60, fn_7000d2da60 as *const (), 0), // +0x641a60 sub_641a60
    Lifted::new(0x7000d2dae0, fn_7000d2dae0 as *const (), 0), // +0x641ae0 sub_641ae0
    Lifted::new(0x7000d2db10, tpl_753::<0x700182acc8, 0x8> as *const (), 1), // +0x641b10 sub_641b10
    Lifted::new(0x7000d2db30, tpl_754::<0x700177c312> as *const (), 0), // +0x641b30 sub_641b30
    Lifted::new(0x7000d2db50, fn_7000d2db50 as *const (), 2), // +0x641b50 sub_641b50
    Lifted::new(0x7000d2db60, tpl_52::<0x8> as *const (), 1), // +0x641b60 sub_641b60
    Lifted::new(0x7000d2db70, tpl_753::<0x700182acc8, 0x8> as *const (), 1), // +0x641b70 sub_641b70
    Lifted::new(0x7000d2db90, tpl_315::<0x7001c6c000> as *const (), 1), // +0x641b90 sub_641b90
    Lifted::new(0x7000d2dba0, fn_7000d2dba0 as *const (), 0), // +0x641ba0 sub_641ba0
    Lifted::new(0x7000d2ddf0, fn_7000d2ddf0 as *const (), 0), // +0x641df0 sub_641df0
    Lifted::new(0x7000d2e060, fn_7000d2e060 as *const (), 0), // +0x642060 sub_642060
    Lifted::new(0x7000d2e340, fn_7000d2e340 as *const (), 0), // +0x642340 sub_642340
    Lifted::new(0x7000d2e590, fn_7000d2e590 as *const (), 0), // +0x642590 sub_642590
    Lifted::new(0x7000d2e800, fn_7000d2e800 as *const (), 0), // +0x642800 sub_642800
    Lifted::new(0x7000d2ea50, fn_7000d2ea50 as *const (), 0), // +0x642a50 sub_642a50
    Lifted::new(0x7000d2eca0, fn_7000d2eca0 as *const (), 0), // +0x642ca0 sub_642ca0
    Lifted::new(0x7000d2ef00, fn_7000d2ef00 as *const (), 0), // +0x642f00 sub_642f00
    Lifted::new(0x7000d2f160, fn_7000d2f160 as *const (), 0), // +0x643160 sub_643160
    Lifted::new(0x7000d2f3f0, fn_7000d2f3f0 as *const (), 0), // +0x6433f0 sub_6433f0
    Lifted::new(0x7000d2f530, fn_7000d2f530 as *const (), 0), // +0x643530 sub_643530
    Lifted::new(0x7000d2f790, fn_7000d2f790 as *const (), 0), // +0x643790 sub_643790
    Lifted::new(0x7000d2f940, fn_7000d2f940 as *const (), 0), // +0x643940 sub_643940
    Lifted::new(0x7000d2fb80, fn_7000d2fb80 as *const (), 0), // +0x643b80 sub_643b80
    Lifted::new(0x7000d2fdd0, fn_7000d2fdd0 as *const (), 0), // +0x643dd0 sub_643dd0
    Lifted::new(0x7000d30070, fn_7000d30070 as *const (), 0), // +0x644070 sub_644070
    Lifted::new(0x7000d302c0, fn_7000d302c0 as *const (), 0), // +0x6442c0 sub_6442c0
    Lifted::new(0x7000d305a0, fn_7000d305a0 as *const (), 0), // +0x6445a0 sub_6445a0
    Lifted::new(0x7000d30840, fn_7000d30840 as *const (), 0), // +0x644840 sub_644840
    Lifted::new(0x7000d30aa0, fn_7000d30aa0 as *const (), 0), // +0x644aa0 sub_644aa0
    Lifted::new(0x7000d30d00, fn_7000d30d00 as *const (), 0), // +0x644d00 sub_644d00
    Lifted::new(0x7000d30f50, fn_7000d30f50 as *const (), 0), // +0x644f50 sub_644f50
    Lifted::new(0x7000d31100, fn_7000d31100 as *const (), 0), // +0x645100 sub_645100
    Lifted::new(0x7000d31370, fn_7000d31370 as *const (), 0), // +0x645370 sub_645370
    Lifted::new(0x7000d315e0, fn_7000d315e0 as *const (), 0), // +0x6455e0 sub_6455e0
    Lifted::new(0x7000d31840, fn_7000d31840 as *const (), 0), // +0x645840 sub_645840
    Lifted::new(0x7000d31ab0, fn_7000d31ab0 as *const (), 0), // +0x645ab0 sub_645ab0
    Lifted::new(0x7000d31d80, fn_7000d31d80 as *const (), 0), // +0x645d80 sub_645d80
    Lifted::new(0x7000d31fc0, fn_7000d31fc0 as *const (), 0), // +0x645fc0 sub_645fc0
    Lifted::new(0x7000d32200, fn_7000d32200 as *const (), 0), // +0x646200 sub_646200
    Lifted::new(0x7000d32440, fn_7000d32440 as *const (), 0), // +0x646440 sub_646440
    Lifted::new(0x7000d32650, fn_7000d32650 as *const (), 0), // +0x646650 sub_646650
    Lifted::new(0x7000d32860, fn_7000d32860 as *const (), 0), // +0x646860 sub_646860
    Lifted::new(0x7000d32ac0, fn_7000d32ac0 as *const (), 0), // +0x646ac0 sub_646ac0
    Lifted::new(0x7000d32d80, fn_7000d32d80 as *const (), 0), // +0x646d80 sub_646d80
    Lifted::new(0x7000d33040, fn_7000d33040 as *const (), 0), // +0x647040 sub_647040
    Lifted::new(0x7000d33300, fn_7000d33300 as *const (), 0), // +0x647300 sub_647300
    Lifted::new(0x7000d33570, fn_7000d33570 as *const (), 0), // +0x647570 sub_647570
    Lifted::new(0x7000d337e0, fn_7000d337e0 as *const (), 0), // +0x6477e0 sub_6477e0
    Lifted::new(0x7000d33ad0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647ad0 sub_647ad0
    Lifted::new(0x7000d33b20, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647b20 sub_647b20
    Lifted::new(0x7000d33b70, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647b70 sub_647b70
    Lifted::new(0x7000d33bc0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647bc0 sub_647bc0
    Lifted::new(0x7000d33c10, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647c10 sub_647c10
    Lifted::new(0x7000d33dc0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647dc0 sub_647dc0
    Lifted::new(0x7000d33e30, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647e30 sub_647e30
    Lifted::new(0x7000d33e80, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647e80 sub_647e80
    Lifted::new(0x7000d33f10, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647f10 sub_647f10
    Lifted::new(0x7000d33f60, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647f60 sub_647f60
    Lifted::new(0x7000d33fb0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x647fb0 sub_647fb0
    Lifted::new(0x7000d34000, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648000 sub_648000
    Lifted::new(0x7000d34090, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648090 sub_648090
    Lifted::new(0x7000d340e0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6480e0 sub_6480e0
    Lifted::new(0x7000d34130, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648130 sub_648130
    Lifted::new(0x7000d34180, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648180 sub_648180
    Lifted::new(0x7000d341d0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6481d0 sub_6481d0
    Lifted::new(0x7000d34220, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648220 sub_648220
    Lifted::new(0x7000d34270, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648270 sub_648270
    Lifted::new(0x7000d342c0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6482c0 sub_6482c0
    Lifted::new(0x7000d34310, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648310 sub_648310
    Lifted::new(0x7000d34360, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648360 sub_648360
    Lifted::new(0x7000d343b0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6483b0 sub_6483b0
    Lifted::new(0x7000d34400, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648400 sub_648400
    Lifted::new(0x7000d34450, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648450 sub_648450
    Lifted::new(0x7000d344a0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6484a0 sub_6484a0
    Lifted::new(0x7000d344f0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6484f0 sub_6484f0
    Lifted::new(0x7000d34540, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648540 sub_648540
    Lifted::new(0x7000d34590, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648590 sub_648590
    Lifted::new(0x7000d345e0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6485e0 sub_6485e0
    Lifted::new(0x7000d34630, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648630 sub_648630
    Lifted::new(0x7000d34680, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648680 sub_648680
    Lifted::new(0x7000d346d0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6486d0 sub_6486d0
    Lifted::new(0x7000d34720, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648720 sub_648720
    Lifted::new(0x7000d34770, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648770 sub_648770
    Lifted::new(0x7000d347c0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6487c0 sub_6487c0
    Lifted::new(0x7000d34810, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648810 sub_648810
    Lifted::new(0x7000d34860, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648860 sub_648860
    Lifted::new(0x7000d348b0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6488b0 sub_6488b0
    Lifted::new(0x7000d34900, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648900 sub_648900
    Lifted::new(0x7000d34950, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648950 sub_648950
    Lifted::new(0x7000d349a0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6489a0 sub_6489a0
    Lifted::new(0x7000d349f0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6489f0 sub_6489f0
    Lifted::new(0x7000d34a40, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648a40 sub_648a40
    Lifted::new(0x7000d34a90, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648a90 sub_648a90
    Lifted::new(0x7000d34cb0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648cb0 sub_648cb0
    Lifted::new(0x7000d34d00, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648d00 sub_648d00
    Lifted::new(0x7000d34d50, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648d50 sub_648d50
    Lifted::new(0x7000d34da0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648da0 sub_648da0
    Lifted::new(0x7000d34df0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648df0 sub_648df0
    Lifted::new(0x7000d34e40, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648e40 sub_648e40
    Lifted::new(0x7000d34e90, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648e90 sub_648e90
    Lifted::new(0x7000d34ee0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648ee0 sub_648ee0
    Lifted::new(0x7000d34f30, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648f30 sub_648f30
    Lifted::new(0x7000d34f80, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648f80 sub_648f80
    Lifted::new(0x7000d34ff0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x648ff0 sub_648ff0
    Lifted::new(0x7000d35040, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x649040 sub_649040
    Lifted::new(0x7000d350b0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6490b0 sub_6490b0
    Lifted::new(0x7000d35140, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x649140 sub_649140
    Lifted::new(0x7000d35390, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649390 sub_649390
    Lifted::new(0x7000d353d0, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x6493d0 sub_6493d0
    Lifted::new(0x7000d35410, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649410 sub_649410
    Lifted::new(0x7000d35450, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649450 sub_649450
    Lifted::new(0x7000d35490, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649490 sub_649490
    Lifted::new(0x7000d354d0, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x6494d0 sub_6494d0
    Lifted::new(0x7000d35510, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649510 sub_649510
    Lifted::new(0x7000d35550, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649550 sub_649550
    Lifted::new(0x7000d35590, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649590 sub_649590
    Lifted::new(0x7000d355d0, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x6495d0 sub_6495d0
    Lifted::new(0x7000d35610, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649610 sub_649610
    Lifted::new(0x7000d356f0, tpl_1257::<0x188, 0x10, 0x700075a560, 0x8, 0x700071c2d0> as *const (), 0), // +0x6496f0 sub_6496f0
    Lifted::new(0x7000d35830, tpl_1258::<0x188, 0x700079f470, 0x10, 0x700075a560> as *const (), 0), // +0x649830 sub_649830
    Lifted::new(0x7000d35870, fn_7000d35870 as *const (), 0), // +0x649870 sub_649870
    Lifted::new(0x7000d359b0, tpl_1259::<0xe4, 0x10, 0x188, 0x700079f470, 0x1c, 0xb8, 0x7000d76200, 0xf0, 0xd8, 0x7000733b00> as *const (), 0), // +0x6499b0 sub_6499b0
    Lifted::new(0x7000d35a70, tpl_1259::<0xe4, 0x10, 0x188, 0x700079f470, 0x1c, 0xb8, 0x7000d76330, 0xf0, 0xd8, 0x70006efc90> as *const (), 0), // +0x649a70 sub_649a70
    Lifted::new(0x7000d35b30, tpl_1260::<0x7000780850> as *const (), 0), // +0x649b30 sub_649b30
    Lifted::new(0x7000d35b60, tpl_1260::<0x7000793f60> as *const (), 0), // +0x649b60 sub_649b60
    Lifted::new(0x7000d35b90, tpl_1261::<0x19, 0x8, 0x10, 0x18, 0x700076bf20, 0x30, 0x20, 0x28, 0x7000d35b90> as *const (), 0), // +0x649b90 sub_649b90
    Lifted::new(0x7000d35cb0, tpl_1261::<0x19, 0x8, 0x10, 0x18, 0x700076bf20, 0x30, 0x20, 0x28, 0x7000d35cb0> as *const (), 0), // +0x649cb0 sub_649cb0
    Lifted::new(0x7000d35dd0, tpl_569::<0x19, 0x10, 0x7000d35dd0> as *const (), 0), // +0x649dd0 sub_649dd0
    Lifted::new(0x7000d35e30, tpl_569::<0x19, 0x10, 0x7000d35e30> as *const (), 0), // +0x649e30 sub_649e30
    Lifted::new(0x7000d35e90, tpl_569::<0x19, 0x10, 0x7000d35e90> as *const (), 0), // +0x649e90 sub_649e90
    Lifted::new(0x7000d35ef0, tpl_569::<0x19, 0x10, 0x7000d35ef0> as *const (), 0), // +0x649ef0 sub_649ef0
    Lifted::new(0x7000d35f50, fn_7000d35f50 as *const (), 0), // +0x649f50 sub_649f50
    Lifted::new(0x7000d36020, fn_7000d36020 as *const (), 0), // +0x64a020 sub_64a020
    Lifted::new(0x7000d36120, fn_7000d36120 as *const (), 1), // +0x64a120 sub_64a120
    Lifted::new(0x7000d36140, fn_7000d36140 as *const (), 0), // +0x64a140 sub_64a140
    Lifted::new(0x7000d36330, fn_7000d36330 as *const (), 1), // +0x64a330 sub_64a330
    Lifted::new(0x7000d36350, fn_7000d36350 as *const (), 1), // +0x64a350 sub_64a350
    Lifted::new(0x7000d36370, fn_7000d36370 as *const (), 0), // +0x64a370 sub_64a370
    Lifted::new(0x7000d363f0, fn_7000d363f0 as *const (), 1), // +0x64a3f0 sub_64a3f0
    Lifted::new(0x7000d36400, fn_7000d36400 as *const (), 1), // +0x64a400 sub_64a400
    Lifted::new(0x7000d36450, fn_7000d36450 as *const (), 1), // +0x64a450 sub_64a450
    Lifted::new(0x7000d367c0, fn_7000d367c0 as *const (), 1), // +0x64a7c0 sub_64a7c0
    Lifted::new(0x7000d36a90, fn_7000d36a90 as *const (), 1), // +0x64aa90 sub_64aa90
    Lifted::new(0x7000d36cb0, fn_7000d36cb0 as *const (), 1), // +0x64acb0 sub_64acb0
    Lifted::new(0x7000d36e40, fn_7000d36e40 as *const (), 1), // +0x64ae40 sub_64ae40
    Lifted::new(0x7000d36f50, fn_7000d36f50 as *const (), 1), // +0x64af50 sub_64af50
    Lifted::new(0x7000d36f70, fn_7000d36f70 as *const (), 2), // +0x64af70 sub_64af70
    Lifted::new(0x7000d371d0, fn_7000d371d0 as *const (), 1), // +0x64b1d0 sub_64b1d0
    Lifted::new(0x7000d37260, fn_7000d37260 as *const (), 1), // +0x64b260 sub_64b260
    Lifted::new(0x7000d37290, fn_7000d37290 as *const (), 1), // +0x64b290 sub_64b290
    Lifted::new(0x7000d372b0, fn_7000d372b0 as *const (), 1), // +0x64b2b0 sub_64b2b0
    Lifted::new(0x7000d373b0, tpl_107::<0x50, 0x58, 0x60> as *const (), 1), // +0x64b3b0 sub_64b3b0
    Lifted::new(0x7000d373d0, fn_7000d373d0 as *const (), 1), // +0x64b3d0 sub_64b3d0
    Lifted::new(0x7000d37410, fn_7000d37410 as *const (), 1), // +0x64b410 sub_64b410
    Lifted::new(0x7000d37430, fn_7000d37430 as *const (), 1), // +0x64b430 sub_64b430
    Lifted::new(0x7000d37460, tpl_21::<0x10> as *const (), 3), // +0x64b460 sub_64b460
    Lifted::new(0x7000d37480, fn_7000d37480 as *const (), 1), // +0x64b480 sub_64b480
    Lifted::new(0x7000d374a0, tpl_19 as *const (), 3), // +0x64b4a0 sub_64b4a0
    Lifted::new(0x7000d374b0, tpl_28 as *const (), 1), // +0x64b4b0 sub_64b4b0
    Lifted::new(0x7000d374c0, tpl_28 as *const (), 1), // +0x64b4c0 sub_64b4c0
    Lifted::new(0x7000d374d0, fn_7000d374d0 as *const (), 1), // +0x64b4d0 sub_64b4d0
    Lifted::new(0x7000d37520, tpl_26::<0x10, 0x20> as *const (), 3), // +0x64b520 sub_64b520
    Lifted::new(0x7000d37550, fn_7000d37550 as *const (), 0), // +0x64b550 sub_64b550
    Lifted::new(0x7000d378c0, fn_7000d378c0 as *const (), 0), // +0x64b8c0 sub_64b8c0
    Lifted::new(0x7000d37910, tpl_1262::<0x7000719830> as *const (), 0), // +0x64b910 sub_64b910
    Lifted::new(0x7000d37930, tpl_33::<0x700177c312> as *const (), 0), // +0x64b930 sub_64b930
    Lifted::new(0x7000d37960, tpl_417::<0x70009d3bb0, 0x7000d3797b> as *const (), 0), // +0x64b960 sub_64b960
    Lifted::new(0x7000d37990, fn_7000d37990 as *const (), 0), // +0x64b990 sub_64b990
    Lifted::new(0x7000d37ad0, tpl_1263::<0x4, 0x8> as *const (), 1), // +0x64bad0 sub_64bad0
    Lifted::new(0x7000d37b30, fn_7000d37b30 as *const (), 0), // +0x64bb30 sub_64bb30
    Lifted::new(0x7000d37b60, fn_7000d37b60 as *const (), 1), // +0x64bb60 sub_64bb60
    Lifted::new(0x7000d37c80, tpl_489::<0x70007854d0> as *const (), 0), // +0x64bc80 sub_64bc80
    Lifted::new(0x7000d37c90, fn_7000d37c90 as *const (), 0), // +0x64bc90 sub_64bc90
    Lifted::new(0x7000d37d40, tpl_489::<0x700078f950> as *const (), 0), // +0x64bd40 sub_64bd40
    Lifted::new(0x7000d37d50, fn_7000d37d50 as *const (), 1), // +0x64bd50 sub_64bd50
    Lifted::new(0x7000d37dc0, fn_7000d37dc0 as *const (), 0), // +0x64bdc0 sub_64bdc0
    Lifted::new(0x7000d38a70, fn_7000d38a70 as *const (), 0), // +0x64ca70 sub_64ca70
    Lifted::new(0x7000d397c0, fn_7000d397c0 as *const (), 0), // +0x64d7c0 sub_64d7c0
    Lifted::new(0x7000d39830, fn_7000d39830 as *const (), 0), // +0x64d830 sub_64d830
    Lifted::new(0x7000d39890, fn_7000d39890 as *const (), 0), // +0x64d890 sub_64d890
    Lifted::new(0x7000d398e0, tpl_1264::<0x7001c6abd8, 0x7000d2dba0, 0x198, 0x19c, 0xf0, 0x70007494a0> as *const (), 0), // +0x64d8e0 sub_64d8e0
    Lifted::new(0x7000d399d0, tpl_1265::<0x7000d2fb80> as *const (), 0), // +0x64d9d0 sub_64d9d0
    Lifted::new(0x7000d39a00, fn_7000d39a00 as *const (), 0), // +0x64da00 sub_64da00
    Lifted::new(0x7000d39b80, fn_7000d39b80 as *const (), 0), // +0x64db80 sub_64db80
    Lifted::new(0x7000d39ba0, fn_7000d39ba0 as *const (), 0), // +0x64dba0 sub_64dba0
    Lifted::new(0x7000d39bc0, fn_7000d39bc0 as *const (), 0), // +0x64dbc0 sub_64dbc0
    Lifted::new(0x7000d39c60, fn_7000d39c60 as *const (), 0), // +0x64dc60 sub_64dc60
    Lifted::new(0x7000d39ca0, fn_7000d39ca0 as *const (), 0), // +0x64dca0 sub_64dca0
    Lifted::new(0x7000d39cd0, fn_7000d39cd0 as *const (), 0), // +0x64dcd0 sub_64dcd0
    Lifted::new(0x7000d39fb0, tpl_1264::<0x7001c6abd8, 0x7000d2ea50, 0x198, 0x19c, 0xf0, 0x70007494a0> as *const (), 0), // +0x64dfb0 sub_64dfb0
    Lifted::new(0x7000d3a0a0, tpl_489::<0x7000735600> as *const (), 0), // +0x64e0a0 sub_64e0a0
    Lifted::new(0x7000d3a0b0, fn_7000d3a0b0 as *const (), 1), // +0x64e0b0 sub_64e0b0
    Lifted::new(0x7000d3a100, fn_7000d3a100 as *const (), 0), // +0x64e100 sub_64e100
    Lifted::new(0x7000d3a340, fn_7000d3a340 as *const (), 0), // +0x64e340 sub_64e340
    Lifted::new(0x7000d3a4f0, fn_7000d3a4f0 as *const (), 0), // +0x64e4f0 sub_64e4f0
    Lifted::new(0x7000d3a510, fn_7000d3a510 as *const (), 0), // +0x64e510 sub_64e510
    Lifted::new(0x7000d3a680, fn_7000d3a680 as *const (), 0), // +0x64e680 sub_64e680
    Lifted::new(0x7000d3b370, fn_7000d3b370 as *const (), 0), // +0x64f370 sub_64f370
    Lifted::new(0x7000d3b590, fn_7000d3b590 as *const (), 0), // +0x64f590 sub_64f590
    Lifted::new(0x7000d3b730, fn_7000d3b730 as *const (), 0), // +0x64f730 sub_64f730
    Lifted::new(0x7000d3b8a0, fn_7000d3b8a0 as *const (), 0), // +0x64f8a0 sub_64f8a0
    Lifted::new(0x7000d3ba10, fn_7000d3ba10 as *const (), 1), // +0x64fa10 sub_64fa10
    Lifted::new(0x7000d3ba30, tpl_1264::<0x7001c6abd8, 0x7000d2eca0, 0x198, 0x19c, 0xf0, 0x70007494a0> as *const (), 0), // +0x64fa30 sub_64fa30
    Lifted::new(0x7000d3bb20, tpl_1264::<0x7001c6abd8, 0x7000d2ef00, 0x198, 0x19c, 0xf0, 0x70007494a0> as *const (), 0), // +0x64fb20 sub_64fb20
    Lifted::new(0x7000d3bc10, fn_7000d3bc10 as *const (), 1), // +0x64fc10 sub_64fc10
    Lifted::new(0x7000d3bc70, fn_7000d3bc70 as *const (), 1), // +0x64fc70 sub_64fc70
    Lifted::new(0x7000d3bd60, fn_7000d3bd60 as *const (), 0), // +0x64fd60 sub_64fd60
    Lifted::new(0x7000d3bd70, fn_7000d3bd70 as *const (), 0), // +0x64fd70 sub_64fd70
    Lifted::new(0x7000d3c030, fn_7000d3c030 as *const (), 0), // +0x650030 sub_650030
    Lifted::new(0x7000d3c190, fn_7000d3c190 as *const (), 0), // +0x650190 sub_650190
    Lifted::new(0x7000d3c390, fn_7000d3c390 as *const (), 0), // +0x650390 sub_650390
    Lifted::new(0x7000d3c540, fn_7000d3c540 as *const (), 0), // +0x650540 sub_650540
    Lifted::new(0x7000d3c770, fn_7000d3c770 as *const (), 0), // +0x650770 sub_650770
    Lifted::new(0x7000d3c970, fn_7000d3c970 as *const (), 0), // +0x650970 sub_650970
    Lifted::new(0x7000d3cb10, fn_7000d3cb10 as *const (), 0), // +0x650b10 sub_650b10
    Lifted::new(0x7000d3cc50, fn_7000d3cc50 as *const (), 0), // +0x650c50 sub_650c50
    Lifted::new(0x7000d3ce50, fn_7000d3ce50 as *const (), 0), // +0x650e50 sub_650e50
    Lifted::new(0x7000d3d050, fn_7000d3d050 as *const (), 0), // +0x651050 sub_651050
    Lifted::new(0x7000d3d250, fn_7000d3d250 as *const (), 0), // +0x651250 sub_651250
    Lifted::new(0x7000d3d450, fn_7000d3d450 as *const (), 0), // +0x651450 sub_651450
    Lifted::new(0x7000d3d680, fn_7000d3d680 as *const (), 0), // +0x651680 sub_651680
    Lifted::new(0x7000d3d710, fn_7000d3d710 as *const (), 0), // +0x651710 sub_651710
    Lifted::new(0x7000d3d850, fn_7000d3d850 as *const (), 0), // +0x651850 sub_651850
    Lifted::new(0x7000d3dbb0, fn_7000d3dbb0 as *const (), 0), // +0x651bb0 sub_651bb0
    Lifted::new(0x7000d3dbd0, fn_7000d3dbd0 as *const (), 0), // +0x651bd0 sub_651bd0
    Lifted::new(0x7000d3dc00, fn_7000d3dc00 as *const (), 0), // +0x651c00 sub_651c00
    Lifted::new(0x7000d3dc20, fn_7000d3dc20 as *const (), 0), // +0x651c20 sub_651c20
    Lifted::new(0x7000d3dc60, tpl_143::<0x8, 0x7000750190> as *const (), 0), // +0x651c60 sub_651c60
    Lifted::new(0x7000d3dc70, tpl_143::<0x8, 0x70006fa730> as *const (), 0), // +0x651c70 sub_651c70
    Lifted::new(0x7000d3dc80, tpl_143::<0x8, 0x7000746ed0> as *const (), 0), // +0x651c80 sub_651c80
    Lifted::new(0x7000d3dc90, tpl_489::<0x7000772030> as *const (), 0), // +0x651c90 sub_651c90
    Lifted::new(0x7000d3dca0, tpl_489::<0x7000741180> as *const (), 0), // +0x651ca0 sub_651ca0
    Lifted::new(0x7000d3dcb0, tpl_489::<0x7000758220> as *const (), 0), // +0x651cb0 sub_651cb0
    Lifted::new(0x7000d3dcc0, fn_7000d3dcc0 as *const (), 1), // +0x651cc0 sub_651cc0
    Lifted::new(0x7000d3dd20, tpl_13::<0x7000d30840> as *const (), 0), // +0x651d20 sub_651d20
    Lifted::new(0x7000d3dd40, fn_7000d3dd40 as *const (), 0), // +0x651d40 sub_651d40
    Lifted::new(0x7000d3df20, fn_7000d3df20 as *const (), 0), // +0x651f20 sub_651f20
    Lifted::new(0x7000d3e010, fn_7000d3e010 as *const (), 0), // +0x652010 sub_652010
    Lifted::new(0x7000d3e250, fn_7000d3e250 as *const (), 0), // +0x652250 sub_652250
    Lifted::new(0x7000d3e430, tpl_489::<0x700073c9b0> as *const (), 0), // +0x652430 sub_652430
    Lifted::new(0x7000d3e440, tpl_489::<0x7000704d70> as *const (), 0), // +0x652440 sub_652440
    Lifted::new(0x7000d3e450, tpl_489::<0x7000728f70> as *const (), 0), // +0x652450 sub_652450
    Lifted::new(0x7000d3e460, tpl_489::<0x70006fedf0> as *const (), 0), // +0x652460 sub_652460
    Lifted::new(0x7000d3e470, fn_7000d3e470 as *const (), 0), // +0x652470 sub_652470
    Lifted::new(0x7000d3e4c0, fn_7000d3e4c0 as *const (), 0), // +0x6524c0 sub_6524c0
    Lifted::new(0x7000d3e500, tpl_13::<0x7000d30aa0> as *const (), 0), // +0x652500 sub_652500
    Lifted::new(0x7000d3e520, fn_7000d3e520 as *const (), 0), // +0x652520 sub_652520
    Lifted::new(0x7000d3e5a0, fn_7000d3e5a0 as *const (), 0), // +0x6525a0 sub_6525a0
    Lifted::new(0x7000d3e5f0, fn_7000d3e5f0 as *const (), 0), // +0x6525f0 sub_6525f0
    Lifted::new(0x7000d3e620, fn_7000d3e620 as *const (), 1), // +0x652620 sub_652620
    Lifted::new(0x7000d3e640, fn_7000d3e640 as *const (), 1), // +0x652640 sub_652640
    Lifted::new(0x7000d3e650, fn_7000d3e650 as *const (), 0), // +0x652650 sub_652650
    Lifted::new(0x7000d3e820, fn_7000d3e820 as *const (), 0), // +0x652820 sub_652820
    Lifted::new(0x7000d3ea10, fn_7000d3ea10 as *const (), 0), // +0x652a10 sub_652a10
    Lifted::new(0x7000d3eb90, tpl_489::<0x70007531a0> as *const (), 0), // +0x652b90 sub_652b90
    Lifted::new(0x7000d3eba0, fn_7000d3eba0 as *const (), 1), // +0x652ba0 sub_652ba0
    Lifted::new(0x7000d3ec00, fn_7000d3ec00 as *const (), 1), // +0x652c00 sub_652c00
    Lifted::new(0x7000d3ecc0, fn_7000d3ecc0 as *const (), 1), // +0x652cc0 sub_652cc0
    Lifted::new(0x7000d3ed10, tpl_489::<0x70007732b0> as *const (), 0), // +0x652d10 sub_652d10
    Lifted::new(0x7000d3ed20, fn_7000d3ed20 as *const (), 0), // +0x652d20 sub_652d20
    Lifted::new(0x7000d3ee90, fn_7000d3ee90 as *const (), 0), // +0x652e90 sub_652e90
    Lifted::new(0x7000d3f0f0, fn_7000d3f0f0 as *const (), 0), // +0x6530f0 sub_6530f0
    Lifted::new(0x7000d3f280, tpl_1266::<0x700074b930, 0x7000784540, 0x7000766710, 0x7000786de0> as *const (), 0), // +0x653280 sub_653280
    Lifted::new(0x7000d3f2e0, tpl_1266::<0x700074b930, 0x700070f280, 0x7000716d00, 0x7000768770> as *const (), 0), // +0x6532e0 sub_6532e0
    Lifted::new(0x7000d3f340, fn_7000d3f340 as *const (), 0), // +0x653340 sub_653340
    Lifted::new(0x7000d3f4d0, fn_7000d3f4d0 as *const (), 0), // +0x6534d0 sub_6534d0
    Lifted::new(0x7000d3f660, fn_7000d3f660 as *const (), 0), // +0x653660 sub_653660
    Lifted::new(0x7000d3f7f0, fn_7000d3f7f0 as *const (), 0), // +0x6537f0 sub_6537f0
    Lifted::new(0x7000d3fa50, fn_7000d3fa50 as *const (), 0), // +0x653a50 sub_653a50
    Lifted::new(0x7000d3fbe0, fn_7000d3fbe0 as *const (), 0), // +0x653be0 sub_653be0
    Lifted::new(0x7000d3fd70, fn_7000d3fd70 as *const (), 0), // +0x653d70 sub_653d70
    Lifted::new(0x7000d3ff50, fn_7000d3ff50 as *const (), 0), // +0x653f50 sub_653f50
    Lifted::new(0x7000d40170, tpl_1267::<0x700078f950, 0x7000d315e0> as *const (), 0), // +0x654170 sub_654170
    Lifted::new(0x7000d401d0, tpl_1268::<0x700071a8a0, 0x700076b980> as *const (), 0), // +0x6541d0 sub_6541d0
    Lifted::new(0x7000d401e0, tpl_1267::<0x7000735600, 0x7000d315e0> as *const (), 0), // +0x6541e0 sub_6541e0
    Lifted::new(0x7000d40240, tpl_13::<0x7000d30f50> as *const (), 0), // +0x654240 sub_654240
    Lifted::new(0x7000d40260, fn_7000d40260 as *const (), 0), // +0x654260 sub_654260
    Lifted::new(0x7000d40590, fn_7000d40590 as *const (), 0), // +0x654590 sub_654590
    Lifted::new(0x7000d406f0, fn_7000d406f0 as *const (), 0), // +0x6546f0 sub_6546f0
    Lifted::new(0x7000d40a40, tpl_1267::<0x70007732b0, 0x7000d315e0> as *const (), 0), // +0x654a40 sub_654a40
    Lifted::new(0x7000d40aa0, fn_7000d40aa0 as *const (), 0), // +0x654aa0 sub_654aa0
    Lifted::new(0x7000d40ce0, fn_7000d40ce0 as *const (), 2), // +0x654ce0 sub_654ce0
    Lifted::new(0x7000d40d80, tpl_1269::<0x7000704d70, 0x7000d30f50> as *const (), 0), // +0x654d80 sub_654d80
    Lifted::new(0x7000d40db0, tpl_1269::<0x7000728f70, 0x7000d30f50> as *const (), 0), // +0x654db0 sub_654db0
    Lifted::new(0x7000d40de0, tpl_1269::<0x70006fedf0, 0x7000d30f50> as *const (), 0), // +0x654de0 sub_654de0
    Lifted::new(0x7000d40e10, fn_7000d40e10 as *const (), 0), // +0x654e10 sub_654e10
    Lifted::new(0x7000d410b0, fn_7000d410b0 as *const (), 0), // +0x6550b0 sub_6550b0
    Lifted::new(0x7000d41480, fn_7000d41480 as *const (), 0), // +0x655480 sub_655480
    Lifted::new(0x7000d41850, fn_7000d41850 as *const (), 0), // +0x655850 sub_655850
    Lifted::new(0x7000d41b10, fn_7000d41b10 as *const (), 0), // +0x655b10 sub_655b10
    Lifted::new(0x7000d41d10, tpl_1270::<0x7000704d70, 0x7000d315e0> as *const (), 0), // +0x655d10 sub_655d10
    Lifted::new(0x7000d41d70, tpl_1270::<0x7000704d70, 0x7000d31840> as *const (), 0), // +0x655d70 sub_655d70
    Lifted::new(0x7000d41dd0, fn_7000d41dd0 as *const (), 0), // +0x655dd0 sub_655dd0
    Lifted::new(0x7000d41f30, fn_7000d41f30 as *const (), 0), // +0x655f30 sub_655f30
    Lifted::new(0x7000d42090, fn_7000d42090 as *const (), 0), // +0x656090 sub_656090
    Lifted::new(0x7000d421f0, fn_7000d421f0 as *const (), 0), // +0x6561f0 sub_6561f0
    Lifted::new(0x7000d42350, fn_7000d42350 as *const (), 1), // +0x656350 sub_656350
    Lifted::new(0x7000d42360, fn_7000d42360 as *const (), 0), // +0x656360 sub_656360
    Lifted::new(0x7000d425a0, fn_7000d425a0 as *const (), 0), // +0x6565a0 sub_6565a0
    Lifted::new(0x7000d42620, fn_7000d42620 as *const (), 0), // +0x656620 sub_656620
    Lifted::new(0x7000d426b0, fn_7000d426b0 as *const (), 0), // +0x6566b0 sub_6566b0
    Lifted::new(0x7000d426f0, fn_7000d426f0 as *const (), 0), // +0x6566f0 sub_6566f0
    Lifted::new(0x7000d42990, tpl_489::<0x70007509a0> as *const (), 0), // +0x656990 sub_656990
    Lifted::new(0x7000d429a0, fn_7000d429a0 as *const (), 0), // +0x6569a0 sub_6569a0
    Lifted::new(0x7000d429b0, fn_7000d429b0 as *const (), 0), // +0x6569b0 sub_6569b0
    Lifted::new(0x7000d429f0, fn_7000d429f0 as *const (), 0), // +0x6569f0 sub_6569f0
    Lifted::new(0x7000d42a40, tpl_182 as *const (), 1), // +0x656a40 sub_656a40
    Lifted::new(0x7000d42a50, fn_7000d42a50 as *const (), 0), // +0x656a50 sub_656a50
    Lifted::new(0x7000d42b30, fn_7000d42b30 as *const (), 0), // +0x656b30 sub_656b30
    Lifted::new(0x7000d42c40, fn_7000d42c40 as *const (), 0), // +0x656c40 sub_656c40
    Lifted::new(0x7000d42da0, fn_7000d42da0 as *const (), 2), // +0x656da0 sub_656da0
    Lifted::new(0x7000d42f30, fn_7000d42f30 as *const (), 0), // +0x656f30 sub_656f30
    Lifted::new(0x7000d43020, fn_7000d43020 as *const (), 0), // +0x657020 sub_657020
    Lifted::new(0x7000d43050, fn_7000d43050 as *const (), 0), // +0x657050 sub_657050
    Lifted::new(0x7000d43220, fn_7000d43220 as *const (), 0), // +0x657220 sub_657220
    Lifted::new(0x7000d43410, fn_7000d43410 as *const (), 0), // +0x657410 sub_657410
    Lifted::new(0x7000d43550, fn_7000d43550 as *const (), 0), // +0x657550 sub_657550
    Lifted::new(0x7000d43580, fn_7000d43580 as *const (), 1), // +0x657580 sub_657580
    Lifted::new(0x7000d43590, fn_7000d43590 as *const (), 1), // +0x657590 sub_657590
    Lifted::new(0x7000d435b0, fn_7000d435b0 as *const (), 1), // +0x6575b0 sub_6575b0
    Lifted::new(0x7000d435c0, fn_7000d435c0 as *const (), 0), // +0x6575c0 sub_6575c0
    Lifted::new(0x7000d43660, fn_7000d43660 as *const (), 0), // +0x657660 sub_657660
    Lifted::new(0x7000d43750, tpl_47::<0x182> as *const (), 1), // +0x657750 sub_657750
    Lifted::new(0x7000d43760, tpl_47::<0x92> as *const (), 1), // +0x657760 sub_657760
    Lifted::new(0x7000d43770, fn_7000d43770 as *const (), 1), // +0x657770 sub_657770
    Lifted::new(0x7000d437d0, fn_7000d437d0 as *const (), 0), // +0x6577d0 sub_6577d0
    Lifted::new(0x7000d438b0, tpl_1265::<0x7000d31ab0> as *const (), 0), // +0x6578b0 sub_6578b0
    Lifted::new(0x7000d438e0, fn_7000d438e0 as *const (), 0), // +0x6578e0 sub_6578e0
    Lifted::new(0x7000d43a20, tpl_489::<0x7000763c20> as *const (), 0), // +0x657a20 sub_657a20
    Lifted::new(0x7000d43a30, fn_7000d43a30 as *const (), 1), // +0x657a30 sub_657a30
    Lifted::new(0x7000d43a40, tpl_1271::<0x190, 0x194> as *const (), 1), // +0x657a40 sub_657a40
    Lifted::new(0x7000d43a60, fn_7000d43a60 as *const (), 1), // +0x657a60 sub_657a60
    Lifted::new(0x7000d43a90, fn_7000d43a90 as *const (), 1), // +0x657a90 sub_657a90
    Lifted::new(0x7000d43ac0, fn_7000d43ac0 as *const (), 1), // +0x657ac0 sub_657ac0
    Lifted::new(0x7000d43bc0, fn_7000d43bc0 as *const (), 0), // +0x657bc0 sub_657bc0
    Lifted::new(0x7000d43d20, fn_7000d43d20 as *const (), 1), // +0x657d20 sub_657d20
    Lifted::new(0x7000d43d40, tpl_1271::<0x190, 0x194> as *const (), 1), // +0x657d40 sub_657d40
    Lifted::new(0x7000d43d60, fn_7000d43d60 as *const (), 1), // +0x657d60 sub_657d60
    Lifted::new(0x7000d43d80, fn_7000d43d80 as *const (), 0), // +0x657d80 sub_657d80
    Lifted::new(0x7000d43e30, tpl_1267::<0x7000735600, 0x7000d33570> as *const (), 0), // +0x657e30 sub_657e30
    Lifted::new(0x7000d43e90, fn_7000d43e90 as *const (), 0), // +0x657e90 sub_657e90
    Lifted::new(0x7000d44070, tpl_1151::<0x7000d32ac0> as *const (), 0), // +0x658070 sub_658070
    Lifted::new(0x7000d440a0, fn_7000d440a0 as *const (), 0), // +0x6580a0 sub_6580a0
    Lifted::new(0x7000d442a0, tpl_13::<0x7000d32d80> as *const (), 0), // +0x6582a0 sub_6582a0
    Lifted::new(0x7000d442c0, tpl_13::<0x7000d33040> as *const (), 0), // +0x6582c0 sub_6582c0
    Lifted::new(0x7000d442e0, fn_7000d442e0 as *const (), 0), // +0x6582e0 sub_6582e0
    Lifted::new(0x7000d444b0, fn_7000d444b0 as *const (), 0), // +0x6584b0 sub_6584b0
    Lifted::new(0x7000d446b0, tpl_1270::<0x7000704d70, 0x7000d33300> as *const (), 0), // +0x6586b0 sub_6586b0
    Lifted::new(0x7000d44710, tpl_1270::<0x7000704d70, 0x7000d33570> as *const (), 0), // +0x658710 sub_658710
    Lifted::new(0x7000d44770, tpl_13::<0x7000d33570> as *const (), 0), // +0x658770 sub_658770
    Lifted::new(0x7000d44790, fn_7000d44790 as *const (), 1), // +0x658790 sub_658790
    Lifted::new(0x7000d44800, tpl_1264::<0x7001c6abd8, 0x7000d2f530, 0x198, 0x19c, 0xf0, 0x70007494a0> as *const (), 0), // +0x658800 sub_658800
    Lifted::new(0x7000d448f0, fn_7000d448f0 as *const (), 0), // +0x6588f0 sub_6588f0
    Lifted::new(0x7000d44b70, fn_7000d44b70 as *const (), 0), // +0x658b70 sub_658b70
    Lifted::new(0x7000d44c60, fn_7000d44c60 as *const (), 0), // +0x658c60 sub_658c60
    Lifted::new(0x7000d44da0, fn_7000d44da0 as *const (), 0), // +0x658da0 sub_658da0
    Lifted::new(0x7000d44f50, fn_7000d44f50 as *const (), 0), // +0x658f50 sub_658f50
    Lifted::new(0x7000d45100, fn_7000d45100 as *const (), 0), // +0x659100 sub_659100
    Lifted::new(0x7000d45240, fn_7000d45240 as *const (), 0), // +0x659240 sub_659240
    Lifted::new(0x7000d45380, fn_7000d45380 as *const (), 0), // +0x659380 sub_659380
    Lifted::new(0x7000d45550, tpl_13::<0x7000d337e0> as *const (), 0), // +0x659550 sub_659550
    Lifted::new(0x7000d45570, fn_7000d45570 as *const (), 0), // +0x659570 sub_659570
    Lifted::new(0x7000d45730, fn_7000d45730 as *const (), 0), // +0x659730 sub_659730
    Lifted::new(0x7000d45900, fn_7000d45900 as *const (), 0), // +0x659900 sub_659900
    Lifted::new(0x7000d45a60, fn_7000d45a60 as *const (), 0), // +0x659a60 sub_659a60
    Lifted::new(0x7000d45bc0, fn_7000d45bc0 as *const (), 0), // +0x659bc0 sub_659bc0
    Lifted::new(0x7000d46000, tpl_1272::<0x4, 0x8, 0xfffffffffffffff8> as *const (), 1), // +0x65a000 sub_65a000
    Lifted::new(0x7000d46060, fn_7000d46060 as *const (), 1), // +0x65a060 sub_65a060
    Lifted::new(0x7000d46120, fn_7000d46120 as *const (), 1), // +0x65a120 sub_65a120
    Lifted::new(0x7000d46210, fn_7000d46210 as *const (), 0), // +0x65a210 sub_65a210
    Lifted::new(0x7000d46290, tpl_1273::<0x198, 0x20, 0x19c, 0x7000715d50> as *const (), 0), // +0x65a290 sub_65a290
    Lifted::new(0x7000d46310, tpl_1273::<0x198, 0x20, 0x19c, 0x7000722460> as *const (), 0), // +0x65a310 sub_65a310
    Lifted::new(0x7000d46390, tpl_1273::<0x198, 0x20, 0x19c, 0x700071d350> as *const (), 0), // +0x65a390 sub_65a390
    Lifted::new(0x7000d46410, fn_7000d46410 as *const (), 2), // +0x65a410 sub_65a410
    Lifted::new(0x7000d46640, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d46650> as *const (), 0), // +0x65a640 sub_65a640
    Lifted::new(0x7000d46660, tpl_180 as *const (), 1), // +0x65a660 sub_65a660
    Lifted::new(0x7000d46670, tpl_50 as *const (), 1), // +0x65a670 sub_65a670
    Lifted::new(0x7000d46680, tpl_349 as *const (), 1), // +0x65a680 sub_65a680
    Lifted::new(0x7000d46690, tpl_52::<0xf0> as *const (), 1), // +0x65a690 sub_65a690
    Lifted::new(0x7000d466a0, tpl_52::<0x10> as *const (), 1), // +0x65a6a0 sub_65a6a0
    Lifted::new(0x7000d466b0, tpl_39::<0x14> as *const (), 1), // +0x65a6b0 sub_65a6b0
    Lifted::new(0x7000d466c0, tpl_52::<0x18> as *const (), 1), // +0x65a6c0 sub_65a6c0
    Lifted::new(0x7000d466d0, tpl_52::<0x18> as *const (), 1), // +0x65a6d0 sub_65a6d0
    Lifted::new(0x7000d466e0, tpl_36::<0x78> as *const (), 1), // +0x65a6e0 sub_65a6e0
    Lifted::new(0x7000d466f0, tpl_30 as *const (), 1), // +0x65a6f0 sub_65a6f0
    Lifted::new(0x7000d46700, tpl_208::<0x10> as *const (), 1), // +0x65a700 sub_65a700
    Lifted::new(0x7000d46710, tpl_36::<0x8> as *const (), 1), // +0x65a710 sub_65a710
    Lifted::new(0x7000d46720, tpl_39::<0xdc> as *const (), 1), // +0x65a720 sub_65a720
    Lifted::new(0x7000d46730, tpl_52::<0xb8> as *const (), 1), // +0x65a730 sub_65a730
    Lifted::new(0x7000d46740, fn_7000d46740 as *const (), 1), // +0x65a740 sub_65a740
    Lifted::new(0x7000d46750, tpl_349 as *const (), 1), // +0x65a750 sub_65a750
    Lifted::new(0x7000d46760, tpl_47::<0x18> as *const (), 1), // +0x65a760 sub_65a760
    Lifted::new(0x7000d46770, tpl_347::<0x4> as *const (), 1), // +0x65a770 sub_65a770
    Lifted::new(0x7000d46780, tpl_36::<0x8> as *const (), 1), // +0x65a780 sub_65a780
    Lifted::new(0x7000d46790, tpl_41 as *const (), 1), // +0x65a790 sub_65a790
    Lifted::new(0x7000d467a0, tpl_39::<0xdc> as *const (), 1), // +0x65a7a0 sub_65a7a0
    Lifted::new(0x7000d467b0, tpl_15 as *const (), 1), // +0x65a7b0 sub_65a7b0
    Lifted::new(0x7000d467c0, tpl_39::<0x1c> as *const (), 1), // +0x65a7c0 sub_65a7c0
    Lifted::new(0x7000d467d0, tpl_30 as *const (), 1), // +0x65a7d0 sub_65a7d0
    Lifted::new(0x7000d467e0, tpl_882::<0x4> as *const (), 1), // +0x65a7e0 sub_65a7e0
    Lifted::new(0x7000d467f0, tpl_39::<0x18> as *const (), 1), // +0x65a7f0 sub_65a7f0
    Lifted::new(0x7000d46800, tpl_47::<0x1c> as *const (), 1), // +0x65a800 sub_65a800
    Lifted::new(0x7000d46810, tpl_52::<0x4> as *const (), 1), // +0x65a810 sub_65a810
    Lifted::new(0x7000d46820, tpl_39::<0xd8> as *const (), 1), // +0x65a820 sub_65a820
    Lifted::new(0x7000d46830, tpl_52::<0xe0> as *const (), 1), // +0x65a830 sub_65a830
    Lifted::new(0x7000d46840, tpl_52::<0xe0> as *const (), 1), // +0x65a840 sub_65a840
    Lifted::new(0x7000d46850, tpl_51::<0x8> as *const (), 1), // +0x65a850 sub_65a850
    Lifted::new(0x7000d46860, tpl_51::<0x20> as *const (), 1), // +0x65a860 sub_65a860
    Lifted::new(0x7000d46870, tpl_51::<0x8> as *const (), 1), // +0x65a870 sub_65a870
    Lifted::new(0x7000d46880, tpl_39::<0x10> as *const (), 1), // +0x65a880 sub_65a880
    Lifted::new(0x7000d46890, tpl_52::<0x20> as *const (), 1), // +0x65a890 sub_65a890
    Lifted::new(0x7000d468a0, tpl_52::<0x38> as *const (), 1), // +0x65a8a0 sub_65a8a0
    Lifted::new(0x7000d468b0, tpl_36::<0x8> as *const (), 1), // +0x65a8b0 sub_65a8b0
    Lifted::new(0x7000d468c0, tpl_39::<0xe4> as *const (), 1), // +0x65a8c0 sub_65a8c0
    Lifted::new(0x7000d468d0, fn_7000d468d0 as *const (), 1), // +0x65a8d0 sub_65a8d0
    Lifted::new(0x7000d469c0, tpl_30 as *const (), 1), // +0x65a9c0 sub_65a9c0
    Lifted::new(0x7000d469d0, tpl_36::<0x98> as *const (), 1), // +0x65a9d0 sub_65a9d0
    Lifted::new(0x7000d469e0, fn_7000d469e0 as *const (), 0), // +0x65a9e0 sub_65a9e0
    Lifted::new(0x7000d46aa0, fn_7000d46aa0 as *const (), 0), // +0x65aaa0 sub_65aaa0
    Lifted::new(0x7000d46b60, fn_7000d46b60 as *const (), 0), // +0x65ab60 sub_65ab60
    Lifted::new(0x7000d46c80, fn_7000d46c80 as *const (), 0), // +0x65ac80 sub_65ac80
    Lifted::new(0x7000d46da0, fn_7000d46da0 as *const (), 0), // +0x65ada0 sub_65ada0
    Lifted::new(0x7000d46e50, fn_7000d46e50 as *const (), 1), // +0x65ae50 sub_65ae50
    Lifted::new(0x7000d46e70, fn_7000d46e70 as *const (), 0), // +0x65ae70 sub_65ae70
    Lifted::new(0x7000d46eb0, fn_7000d46eb0 as *const (), 0), // +0x65aeb0 sub_65aeb0
    Lifted::new(0x7000d46ed0, tpl_20 as *const (), 1), // +0x65aed0 sub_65aed0
    Lifted::new(0x7000d46ee0, tpl_20 as *const (), 1), // +0x65aee0 sub_65aee0
    Lifted::new(0x7000d46ef0, fn_7000d46ef0 as *const (), 1), // +0x65aef0 sub_65aef0
    Lifted::new(0x7000d46f00, fn_7000d46f00 as *const (), 1), // +0x65af00 sub_65af00
    Lifted::new(0x7000d46f30, fn_7000d46f30 as *const (), 0), // +0x65af30 sub_65af30
    Lifted::new(0x7000d46f60, fn_7000d46f60 as *const (), 0), // +0x65af60 sub_65af60
    Lifted::new(0x7000d46fb0, fn_7000d46fb0 as *const (), 1), // +0x65afb0 sub_65afb0
    Lifted::new(0x7000d46fc0, fn_7000d46fc0 as *const (), 1), // +0x65afc0 sub_65afc0
    Lifted::new(0x7000d47010, fn_7000d47010 as *const (), 0), // +0x65b010 sub_65b010
    Lifted::new(0x7000d47050, fn_7000d47050 as *const (), 0), // +0x65b050 sub_65b050
    Lifted::new(0x7000d47090, tpl_1274::<0x18, 0x700182d310, 0x20, 0x70007177d0, 0x700070ac90> as *const (), 0), // +0x65b090 sub_65b090
    Lifted::new(0x7000d47100, fn_7000d47100 as *const (), 0), // +0x65b100 sub_65b100
    Lifted::new(0x7000d47170, fn_7000d47170 as *const (), 0), // +0x65b170 sub_65b170
    Lifted::new(0x7000d472f0, fn_7000d472f0 as *const (), 0), // +0x65b2f0 sub_65b2f0
    Lifted::new(0x7000d47320, tpl_1274::<0x18, 0x700182d310, 0x20, 0x70007177d0, 0x700070ac90> as *const (), 0), // +0x65b320 sub_65b320
    Lifted::new(0x7000d47390, fn_7000d47390 as *const (), 0), // +0x65b390 sub_65b390
    Lifted::new(0x7000d473d0, fn_7000d473d0 as *const (), 0), // +0x65b3d0 sub_65b3d0
    Lifted::new(0x7000d47410, fn_7000d47410 as *const (), 0), // +0x65b410 sub_65b410
    Lifted::new(0x7000d47450, fn_7000d47450 as *const (), 0), // +0x65b450 sub_65b450
    Lifted::new(0x7000d47520, fn_7000d47520 as *const (), 0), // +0x65b520 sub_65b520
    Lifted::new(0x7000d475a0, fn_7000d475a0 as *const (), 0), // +0x65b5a0 sub_65b5a0
    Lifted::new(0x7000d47620, fn_7000d47620 as *const (), 0), // +0x65b620 sub_65b620
    Lifted::new(0x7000d476c0, fn_7000d476c0 as *const (), 0), // +0x65b6c0 sub_65b6c0
    Lifted::new(0x7000d477e0, tpl_1274::<0x18, 0x700182d310, 0x20, 0x70007177d0, 0x700070ac90> as *const (), 0), // +0x65b7e0 sub_65b7e0
    Lifted::new(0x7000d47850, tpl_1275::<0x18, 0x20, 0x70007177d0, 0x7000782d40> as *const (), 0), // +0x65b850 sub_65b850
    Lifted::new(0x7000d478b0, fn_7000d478b0 as *const (), 0), // +0x65b8b0 sub_65b8b0
    Lifted::new(0x7000d47930, fn_7000d47930 as *const (), 0), // +0x65b930 sub_65b930
    Lifted::new(0x7000d47970, tpl_1275::<0x18, 0x20, 0x70007177d0, 0x700070ac90> as *const (), 0), // +0x65b970 sub_65b970
    Lifted::new(0x7000d479d0, fn_7000d479d0 as *const (), 1), // +0x65b9d0 sub_65b9d0
    Lifted::new(0x7000d47a10, fn_7000d47a10 as *const (), 0), // +0x65ba10 sub_65ba10
    Lifted::new(0x7000d47a40, fn_7000d47a40 as *const (), 0), // +0x65ba40 sub_65ba40
    Lifted::new(0x7000d47a70, fn_7000d47a70 as *const (), 1), // +0x65ba70 sub_65ba70
    Lifted::new(0x7000d47b30, fn_7000d47b30 as *const (), 1), // +0x65bb30 sub_65bb30
    Lifted::new(0x7000d47b70, fn_7000d47b70 as *const (), 0), // +0x65bb70 sub_65bb70
    Lifted::new(0x7000d47bd0, fn_7000d47bd0 as *const (), 1), // +0x65bbd0 sub_65bbd0
    Lifted::new(0x7000d47c10, fn_7000d47c10 as *const (), 1), // +0x65bc10 sub_65bc10
    Lifted::new(0x7000d47c40, fn_7000d47c40 as *const (), 0), // +0x65bc40 sub_65bc40
    Lifted::new(0x7000d47ca0, fn_7000d47ca0 as *const (), 1), // +0x65bca0 sub_65bca0
    Lifted::new(0x7000d47e40, tpl_941::<0x700076bf20, 0x7000b95540, 0x7000d47fa1> as *const (), 0), // +0x65be40 sub_65be40
    Lifted::new(0x7000d48000, fn_7000d48000 as *const (), 0), // +0x65c000 sub_65c000
    Lifted::new(0x7000d48120, fn_7000d48120 as *const (), 0), // +0x65c120 sub_65c120
    Lifted::new(0x7000d48140, fn_7000d48140 as *const (), 0), // +0x65c140 sub_65c140
    Lifted::new(0x7000d481a0, fn_7000d481a0 as *const (), 0), // +0x65c1a0 sub_65c1a0
    Lifted::new(0x7000d48200, fn_7000d48200 as *const (), 0), // +0x65c200 sub_65c200
    Lifted::new(0x7000d48250, fn_7000d48250 as *const (), 0), // +0x65c250 sub_65c250
    Lifted::new(0x7000d482a0, fn_7000d482a0 as *const (), 0), // +0x65c2a0 sub_65c2a0
    Lifted::new(0x7000d482f0, fn_7000d482f0 as *const (), 0), // +0x65c2f0 sub_65c2f0
    Lifted::new(0x7000d48330, fn_7000d48330 as *const (), 0), // +0x65c330 sub_65c330
    Lifted::new(0x7000d48360, fn_7000d48360 as *const (), 0), // +0x65c360 sub_65c360
    Lifted::new(0x7000d48470, fn_7000d48470 as *const (), 1), // +0x65c470 sub_65c470
    Lifted::new(0x7000d484a0, fn_7000d484a0 as *const (), 0), // +0x65c4a0 sub_65c4a0
    Lifted::new(0x7000d484e0, tpl_39::<0x50> as *const (), 1), // +0x65c4e0 sub_65c4e0
    Lifted::new(0x7000d484f0, tpl_1276::<0x40, 0x38> as *const (), 1), // +0x65c4f0 sub_65c4f0
    Lifted::new(0x7000d48500, tpl_1277::<0x20, 0x18, 0x78, 0x7000733ce0, 0x700076c8b0, 0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x65c500 sub_65c500
    Lifted::new(0x7000d48630, tpl_1277::<0x20, 0x18, 0x78, 0x700070d410, 0x7000725790, 0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x65c630 sub_65c630
    Lifted::new(0x7000d48760, fn_7000d48760 as *const (), 0), // +0x65c760 sub_65c760
    Lifted::new(0x7000d487b0, fn_7000d487b0 as *const (), 0), // +0x65c7b0 sub_65c7b0
    Lifted::new(0x7000d48800, fn_7000d48800 as *const (), 0), // +0x65c800 sub_65c800
    Lifted::new(0x7000d48970, fn_7000d48970 as *const (), 0), // +0x65c970 sub_65c970
    Lifted::new(0x7000d489a0, fn_7000d489a0 as *const (), 0), // +0x65c9a0 sub_65c9a0
    Lifted::new(0x7000d48a00, fn_7000d48a00 as *const (), 0), // +0x65ca00 sub_65ca00
    Lifted::new(0x7000d48a50, fn_7000d48a50 as *const (), 1), // +0x65ca50 sub_65ca50
    Lifted::new(0x7000d48a70, fn_7000d48a70 as *const (), 0), // +0x65ca70 sub_65ca70
    Lifted::new(0x7000d48aa0, fn_7000d48aa0 as *const (), 0), // +0x65caa0 sub_65caa0
    Lifted::new(0x7000d48ad0, fn_7000d48ad0 as *const (), 0), // +0x65cad0 sub_65cad0
    Lifted::new(0x7000d48c80, fn_7000d48c80 as *const (), 0), // +0x65cc80 sub_65cc80
    Lifted::new(0x7000d48d20, fn_7000d48d20 as *const (), 0), // +0x65cd20 sub_65cd20
    Lifted::new(0x7000d48de0, fn_7000d48de0 as *const (), 0), // +0x65cde0 sub_65cde0
    Lifted::new(0x7000d48e10, fn_7000d48e10 as *const (), 0), // +0x65ce10 sub_65ce10
    Lifted::new(0x7000d48f30, fn_7000d48f30 as *const (), 1), // +0x65cf30 sub_65cf30
    Lifted::new(0x7000d48f40, fn_7000d48f40 as *const (), 0), // +0x65cf40 sub_65cf40
    Lifted::new(0x7000d49170, fn_7000d49170 as *const (), 0), // +0x65d170 sub_65d170
    Lifted::new(0x7000d49270, fn_7000d49270 as *const (), 0), // +0x65d270 sub_65d270
    Lifted::new(0x7000d49380, fn_7000d49380 as *const (), 0), // +0x65d380 sub_65d380
    Lifted::new(0x7000d49450, fn_7000d49450 as *const (), 0), // +0x65d450 sub_65d450
    Lifted::new(0x7000d49550, fn_7000d49550 as *const (), 0), // +0x65d550 sub_65d550
    Lifted::new(0x7000d49580, fn_7000d49580 as *const (), 1), // +0x65d580 sub_65d580
    Lifted::new(0x7000d495a0, fn_7000d495a0 as *const (), 0), // +0x65d5a0 sub_65d5a0
    Lifted::new(0x7000d49620, fn_7000d49620 as *const (), 0), // +0x65d620 sub_65d620
    Lifted::new(0x7000d49670, fn_7000d49670 as *const (), 0), // +0x65d670 sub_65d670
    Lifted::new(0x7000d49720, tpl_1278::<0x92, 0x7000797c20, 0x20, 0x7000710460, 0x24, 0x38, 0x18, 0x700078d010, 0x70007a1250> as *const (), 0), // +0x65d720 sub_65d720
    Lifted::new(0x7000d497e0, tpl_1278::<0x92, 0x7000797c20, 0x20, 0x7000710460, 0x24, 0x38, 0x18, 0x700078d010, 0x7000748a40> as *const (), 0), // +0x65d7e0 sub_65d7e0
    Lifted::new(0x7000d498a0, fn_7000d498a0 as *const (), 0), // +0x65d8a0 sub_65d8a0
    Lifted::new(0x7000d49980, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000d49990> as *const (), 0), // +0x65d980 sub_65d980
    Lifted::new(0x7000d499a0, tpl_1279::<0x700076bf20, 0x70009a4db0, 0x7000d49af0> as *const (), 0), // +0x65d9a0 sub_65d9a0
    Lifted::new(0x7000d49b50, tpl_1279::<0x700076bf20, 0x70009a4db0, 0x7000d49ca0> as *const (), 0), // +0x65db50 sub_65db50
    Lifted::new(0x7000d49d00, fn_7000d49d00 as *const (), 0), // +0x65dd00 sub_65dd00
    Lifted::new(0x7000d4aa60, fn_7000d4aa60 as *const (), 0), // +0x65ea60 sub_65ea60
    Lifted::new(0x7000d4aaf0, fn_7000d4aaf0 as *const (), 0), // +0x65eaf0 sub_65eaf0
    Lifted::new(0x7000d4abc0, fn_7000d4abc0 as *const (), 0), // +0x65ebc0 sub_65ebc0
    Lifted::new(0x7000d4ac40, fn_7000d4ac40 as *const (), 0), // +0x65ec40 sub_65ec40
    Lifted::new(0x7000d4acc0, fn_7000d4acc0 as *const (), 0), // +0x65ecc0 sub_65ecc0
    Lifted::new(0x7000d4c090, tpl_181 as *const (), 1), // +0x660090 sub_660090
    Lifted::new(0x7000d4c0a0, fn_7000d4c0a0 as *const (), 1), // +0x6600a0 sub_6600a0
    Lifted::new(0x7000d4c0b0, fn_7000d4c0b0 as *const (), 1), // +0x6600b0 sub_6600b0
    Lifted::new(0x7000d4c0e0, fn_7000d4c0e0 as *const (), 1), // +0x6600e0 sub_6600e0
    Lifted::new(0x7000d4c0f0, fn_7000d4c0f0 as *const (), 1), // +0x6600f0 sub_6600f0
    Lifted::new(0x7000d4c100, tpl_181 as *const (), 1), // +0x660100 sub_660100
    Lifted::new(0x7000d4c110, fn_7000d4c110 as *const (), 1), // +0x660110 sub_660110
    Lifted::new(0x7000d4c190, tpl_1280::<0x700076df00, 0x70007a0f80, 0x7000af7af0, 0x7000767d50, 0x700074a010, 0x7000d5b450, 0x7000e970f0> as *const (), 2), // +0x660190 sub_660190
    Lifted::new(0x7000d4c760, tpl_1280::<0x70006f52f0, 0x70007a0f80, 0x7000af7dc0, 0x7000767d50, 0x700074a010, 0x7000d5b450, 0x7000e970f0> as *const (), 2), // +0x660760 sub_660760
    Lifted::new(0x7000d4cd30, fn_7000d4cd30 as *const (), 2), // +0x660d30 sub_660d30
    Lifted::new(0x7000d4d0c0, fn_7000d4d0c0 as *const (), 2), // +0x6610c0 sub_6610c0
    Lifted::new(0x7000d4d490, tpl_1281::<0x7000769b10, 0x7000d4eea0, 0x7000784c80, 0x700072a4f0, 0x7000748d80, 0x700182ce58, 0x7000d510b0, 0x7000759790, 0x70006f62a0, 0x7000780480, 0x70007781d0, 0x7000755560, 0x70007008a0, 0x70007551c0, 0x700076f0b0, 0x7000d56690, 0x700071d2f0, 0x70007940a0, 0x7000d52990, 0x700076c980> as *const (), 2), // +0x661490 sub_661490
    Lifted::new(0x7000d4d850, fn_7000d4d850 as *const (), 2), // +0x661850 sub_661850
    Lifted::new(0x7000d4dc00, tpl_1281::<0x7000769b10, 0x7000d4eea0, 0x7000784c80, 0x700072a4f0, 0x7000748d80, 0x700182ce58, 0x7000d51850, 0x7000759790, 0x70006f62a0, 0x7000780480, 0x70007781d0, 0x7000755560, 0x70007008a0, 0x70007551c0, 0x700076f0b0, 0x7000d56690, 0x700071d2f0, 0x70007940a0, 0x7000d52990, 0x700076c980> as *const (), 2), // +0x661c00 sub_661c00
    Lifted::new(0x7000d4dfc0, tpl_1282::<0x7000d4cd30, 0x70007567f0, 0x7001c7201a, 0x70017ea368, 0x7001d543d0, 0x7001d54aa8, 0x7000735a90, 0x70017ea358, 0x700075e790, 0x7001d550d0, 0x7000a47e70, 0x7000dc44c0, 0x700177c312, 0x7001d550c8, 0x7001d54470, 0x7001d54460, 0x7001d54468, 0x7000718890> as *const (), 0), // +0x661fc0 sub_661fc0
    Lifted::new(0x7000d4e1e0, tpl_1282::<0x7000d4d0c0, 0x70007a1600, 0x7001c7201a, 0x70017ea368, 0x7001d543d0, 0x7001d54aa8, 0x7000735a90, 0x70017ea358, 0x700075e790, 0x7001d550d0, 0x7000a47e70, 0x7000dc44c0, 0x700177c312, 0x7001d550c8, 0x7001d54470, 0x7001d54460, 0x7001d54468, 0x700073b5c0> as *const (), 0), // +0x6621e0 sub_6621e0
    Lifted::new(0x7000d4e400, fn_7000d4e400 as *const (), 0), // +0x662400 sub_662400
    Lifted::new(0x7000d4e490, fn_7000d4e490 as *const (), 0), // +0x662490 sub_662490
    Lifted::new(0x7000d4e510, tpl_564::<0x7000a47050, 0x7000a47a50> as *const (), 0), // +0x662510 sub_662510
    Lifted::new(0x7000d4e590, tpl_565::<0x7000a47050, 0x7000a47a50> as *const (), 0), // +0x662590 sub_662590
    Lifted::new(0x7000d4e610, fn_7000d4e610 as *const (), 2), // +0x662610 sub_662610
    Lifted::new(0x7000d4e7e0, tpl_1283::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000d4e96d, 0x7000b5ae00, 0x7000d4e973, 0x70007f0910, 0x7000d4e979> as *const (), 0), // +0x6627e0 sub_6627e0
    Lifted::new(0x7000d4e9e0, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000d4ebb8, 0x7000d6e0a0, 0x7000d4ebbe, 0x70007f0910, 0x7000d4ebc4> as *const (), 2), // +0x6629e0 sub_6629e0
    Lifted::new(0x7000d4ec40, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000d4ee18, 0x7000d6e0c0, 0x7000d4ee1e, 0x70007f0910, 0x7000d4ee24> as *const (), 2), // +0x662c40 sub_662c40
    Lifted::new(0x7000d4eea0, tpl_1284::<0x700076bf20, 0x7000d6e120, 0x7000d4f074> as *const (), 2), // +0x662ea0 sub_662ea0
    Lifted::new(0x7000d4f0f0, tpl_569::<0x19, 0x10, 0x7000d4f0f0> as *const (), 0), // +0x6630f0 sub_6630f0
    Lifted::new(0x7000d4f150, tpl_63::<0x19, 0x10, 0x7000d4f150, 0x700177c312> as *const (), 0), // +0x663150 sub_663150
    Lifted::new(0x7000d4f1c0, fn_7000d4f1c0 as *const (), 2), // +0x6631c0 sub_6631c0
    Lifted::new(0x7000d4f3a0, tpl_394::<0x700076bf20, 0x7000a36e80, 0x7000d4f512> as *const (), 0), // +0x6633a0 sub_6633a0
    Lifted::new(0x7000d4f570, tpl_1285::<0x700076bf20, 0x7000d6e020, 0x7000d4f6d5> as *const (), 2), // +0x663570 sub_663570
    Lifted::new(0x7000d4f730, tpl_1285::<0x700076bf20, 0x7000d6e040, 0x7000d4f895> as *const (), 2), // +0x663730 sub_663730
    Lifted::new(0x7000d4f8f0, tpl_1285::<0x700076bf20, 0x7000d6e060, 0x7000d4fa55> as *const (), 2), // +0x6638f0 sub_6638f0
    Lifted::new(0x7000d4fab0, tpl_1285::<0x700076bf20, 0x7000d6e080, 0x7000d4fc15> as *const (), 2), // +0x663ab0 sub_663ab0
    Lifted::new(0x7000d4fc70, tpl_1285::<0x700076bf20, 0x7000d6e0e0, 0x7000d4fdd5> as *const (), 2), // +0x663c70 sub_663c70
    Lifted::new(0x7000d4fe30, tpl_1285::<0x700076bf20, 0x7000d6e100, 0x7000d4ff95> as *const (), 2), // +0x663e30 sub_663e30
    Lifted::new(0x7000d4fff0, fn_7000d4fff0 as *const (), 0), // +0x663ff0 sub_663ff0
    Lifted::new(0x7000d50090, tpl_88::<0x70006f8da0, 0x700182cda8> as *const (), 0), // +0x664090 sub_664090
    Lifted::new(0x7000d500c0, fn_7000d500c0 as *const (), 0), // +0x6640c0 sub_6640c0
    Lifted::new(0x7000d50170, fn_7000d50170 as *const (), 0), // +0x664170 sub_664170
    Lifted::new(0x7000d50590, fn_7000d50590 as *const (), 0), // +0x664590 sub_664590
    Lifted::new(0x7000d50750, fn_7000d50750 as *const (), 0), // +0x664750 sub_664750
    Lifted::new(0x7000d50820, tpl_1286::<0x700182cd88, 0x8> as *const (), 1), // +0x664820 sub_664820
    Lifted::new(0x7000d50840, fn_7000d50840 as *const (), 0), // +0x664840 sub_664840
    Lifted::new(0x7000d509a0, fn_7000d509a0 as *const (), 0), // +0x6649a0 sub_6649a0
    Lifted::new(0x7000d50b20, fn_7000d50b20 as *const (), 0), // +0x664b20 sub_664b20
    Lifted::new(0x7000d50c80, fn_7000d50c80 as *const (), 0), // +0x664c80 sub_664c80
    Lifted::new(0x7000d50d00, fn_7000d50d00 as *const (), 0), // +0x664d00 sub_664d00
    Lifted::new(0x7000d50e00, fn_7000d50e00 as *const (), 0), // +0x664e00 sub_664e00
    Lifted::new(0x7000d50e50, tpl_15 as *const (), 1), // +0x664e50 sub_664e50
    Lifted::new(0x7000d50e60, fn_7000d50e60 as *const (), 0), // +0x664e60 sub_664e60
    Lifted::new(0x7000d50eb0, tpl_30 as *const (), 1), // +0x664eb0 sub_664eb0
    Lifted::new(0x7000d50ec0, fn_7000d50ec0 as *const (), 0), // +0x664ec0 sub_664ec0
    Lifted::new(0x7000d510b0, fn_7000d510b0 as *const (), 2), // +0x6650b0 sub_6650b0
    Lifted::new(0x7000d51850, fn_7000d51850 as *const (), 0), // +0x665850 sub_665850
    Lifted::new(0x7000d51bb0, fn_7000d51bb0 as *const (), 0), // +0x665bb0 sub_665bb0
    Lifted::new(0x7000d51dd0, tpl_1287::<0xfffffffffffffff8, 0x7000706a40, 0x700177c312> as *const (), 0), // +0x665dd0 sub_665dd0
    Lifted::new(0x7000d51ea0, tpl_1287::<0xfffffffffffffff8, 0x7000706a40, 0x700177c312> as *const (), 0), // +0x665ea0 sub_665ea0
    Lifted::new(0x7000d51f70, fn_7000d51f70 as *const (), 0), // +0x665f70 sub_665f70
    Lifted::new(0x7000d52040, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x666040 sub_666040
    Lifted::new(0x7000d52090, fn_7000d52090 as *const (), 0), // +0x666090 sub_666090
    Lifted::new(0x7000d52160, fn_7000d52160 as *const (), 0), // +0x666160 sub_666160
    Lifted::new(0x7000d52230, fn_7000d52230 as *const (), 0), // +0x666230 sub_666230
    Lifted::new(0x7000d522f0, fn_7000d522f0 as *const (), 0), // +0x6662f0 sub_6662f0
    Lifted::new(0x7000d523b0, tpl_133::<0x7000706a40, 0x700177c312> as *const (), 0), // +0x6663b0 sub_6663b0
    Lifted::new(0x7000d523f0, tpl_133::<0x7000706a40, 0x700177c312> as *const (), 0), // +0x6663f0 sub_6663f0
    Lifted::new(0x7000d52430, tpl_133::<0x70006ffba0, 0x700177c312> as *const (), 0), // +0x666430 sub_666430
    Lifted::new(0x7000d52470, fn_7000d52470 as *const (), 0), // +0x666470 sub_666470
    Lifted::new(0x7000d524f0, tpl_33::<0x700177c312> as *const (), 0), // +0x6664f0 sub_6664f0
    Lifted::new(0x7000d52520, tpl_133::<0x7000740a40, 0x700177c312> as *const (), 0), // +0x666520 sub_666520
    Lifted::new(0x7000d52560, tpl_133::<0x70006f2820, 0x700177c312> as *const (), 0), // +0x666560 sub_666560
    Lifted::new(0x7000d525a0, tpl_133::<0x7000711990, 0x700177c312> as *const (), 0), // +0x6665a0 sub_6665a0
    Lifted::new(0x7000d525e0, tpl_133::<0x700076c980, 0x700177c312> as *const (), 0), // +0x6665e0 sub_6665e0
    Lifted::new(0x7000d52620, fn_7000d52620 as *const (), 0), // +0x666620 sub_666620
    Lifted::new(0x7000d526a0, fn_7000d526a0 as *const (), 0), // +0x6666a0 sub_6666a0
    Lifted::new(0x7000d52790, fn_7000d52790 as *const (), 0), // +0x666790 sub_666790
    Lifted::new(0x7000d52870, fn_7000d52870 as *const (), 0), // +0x666870 sub_666870
    Lifted::new(0x7000d52990, fn_7000d52990 as *const (), 0), // +0x666990 sub_666990
    Lifted::new(0x7000d52bd0, fn_7000d52bd0 as *const (), 2), // +0x666bd0 sub_666bd0
    Lifted::new(0x7000d52c20, tpl_1216::<0x10, 0x7000779880> as *const (), 0), // +0x666c20 sub_666c20
    Lifted::new(0x7000d52c30, tpl_974::<0x10, 0x700072a2a0> as *const (), 0), // +0x666c30 sub_666c30
    Lifted::new(0x7000d52c40, tpl_1216::<0x10, 0x70007248b0> as *const (), 0), // +0x666c40 sub_666c40
    Lifted::new(0x7000d52c50, tpl_974::<0x10, 0x70007a4610> as *const (), 0), // +0x666c50 sub_666c50
    Lifted::new(0x7000d52c60, tpl_1216::<0x10, 0x700070a110> as *const (), 0), // +0x666c60 sub_666c60
    Lifted::new(0x7000d52c70, fn_7000d52c70 as *const (), 0), // +0x666c70 sub_666c70
    Lifted::new(0x7000d53390, fn_7000d53390 as *const (), 0), // +0x667390 sub_667390
    Lifted::new(0x7000d535f0, fn_7000d535f0 as *const (), 0), // +0x6675f0 sub_6675f0
    Lifted::new(0x7000d538a0, fn_7000d538a0 as *const (), 0), // +0x6678a0 sub_6678a0
    Lifted::new(0x7000d538f0, fn_7000d538f0 as *const (), 0), // +0x6678f0 sub_6678f0
    Lifted::new(0x7000d53a60, fn_7000d53a60 as *const (), 2), // +0x667a60 sub_667a60
    Lifted::new(0x7000d54e30, fn_7000d54e30 as *const (), 0), // +0x668e30 sub_668e30
    Lifted::new(0x7000d550e0, fn_7000d550e0 as *const (), 2), // +0x6690e0 sub_6690e0
    Lifted::new(0x7000d55a90, fn_7000d55a90 as *const (), 0), // +0x669a90 sub_669a90
    Lifted::new(0x7000d55b20, fn_7000d55b20 as *const (), 0), // +0x669b20 sub_669b20
    Lifted::new(0x7000d55c70, fn_7000d55c70 as *const (), 0), // +0x669c70 sub_669c70
    Lifted::new(0x7000d55e30, fn_7000d55e30 as *const (), 0), // +0x669e30 sub_669e30
    Lifted::new(0x7000d56470, fn_7000d56470 as *const (), 0), // +0x66a470 sub_66a470
    Lifted::new(0x7000d56690, fn_7000d56690 as *const (), 2), // +0x66a690 sub_66a690
    Lifted::new(0x7000d569e0, fn_7000d569e0 as *const (), 0), // +0x66a9e0 sub_66a9e0
    Lifted::new(0x7000d56bf0, fn_7000d56bf0 as *const (), 0), // +0x66abf0 sub_66abf0
    Lifted::new(0x7000d56c50, fn_7000d56c50 as *const (), 0), // +0x66ac50 sub_66ac50
    Lifted::new(0x7000d57000, fn_7000d57000 as *const (), 0), // +0x66b000 sub_66b000
    Lifted::new(0x7000d570d0, fn_7000d570d0 as *const (), 0), // +0x66b0d0 sub_66b0d0
    Lifted::new(0x7000d57250, fn_7000d57250 as *const (), 0), // +0x66b250 sub_66b250
    Lifted::new(0x7000d572a0, fn_7000d572a0 as *const (), 0), // +0x66b2a0 sub_66b2a0
    Lifted::new(0x7000d57430, fn_7000d57430 as *const (), 0), // +0x66b430 sub_66b430
    Lifted::new(0x7000d57580, fn_7000d57580 as *const (), 0), // +0x66b580 sub_66b580
    Lifted::new(0x7000d577e0, fn_7000d577e0 as *const (), 0), // +0x66b7e0 sub_66b7e0
    Lifted::new(0x7000d578d0, fn_7000d578d0 as *const (), 0), // +0x66b8d0 sub_66b8d0
    Lifted::new(0x7000d57a00, fn_7000d57a00 as *const (), 0), // +0x66ba00 sub_66ba00
    Lifted::new(0x7000d57a60, fn_7000d57a60 as *const (), 0), // +0x66ba60 sub_66ba60
    Lifted::new(0x7000d57b10, fn_7000d57b10 as *const (), 0), // +0x66bb10 sub_66bb10
    Lifted::new(0x7000d57c60, fn_7000d57c60 as *const (), 0), // +0x66bc60 sub_66bc60
    Lifted::new(0x7000d57e10, fn_7000d57e10 as *const (), 0), // +0x66be10 sub_66be10
    Lifted::new(0x7000d57ec0, tpl_1288::<0xf8, 0x2c8, 0x1, 0xfc, 0x100, 0x8, 0x18, 0x7000780480, 0x700072c7b0> as *const (), 0), // +0x66bec0 sub_66bec0
    Lifted::new(0x7000d57fe0, tpl_1288::<0xf8, 0x2c8, 0x1, 0xfc, 0x100, 0x8, 0x18, 0x7000781dc0, 0x7000774be0> as *const (), 0), // +0x66bfe0 sub_66bfe0
    Lifted::new(0x7000d58100, fn_7000d58100 as *const (), 0), // +0x66c100 sub_66c100
    Lifted::new(0x7000d58390, fn_7000d58390 as *const (), 0), // +0x66c390 sub_66c390
    Lifted::new(0x7000d58500, fn_7000d58500 as *const (), 0), // +0x66c500 sub_66c500
    Lifted::new(0x7000d58680, fn_7000d58680 as *const (), 0), // +0x66c680 sub_66c680
    Lifted::new(0x7000d58aa0, fn_7000d58aa0 as *const (), 0), // +0x66caa0 sub_66caa0
    Lifted::new(0x7000d58c40, fn_7000d58c40 as *const (), 0), // +0x66cc40 sub_66cc40
    Lifted::new(0x7000d58d20, fn_7000d58d20 as *const (), 0), // +0x66cd20 sub_66cd20
    Lifted::new(0x7000d58e90, fn_7000d58e90 as *const (), 0), // +0x66ce90 sub_66ce90
    Lifted::new(0x7000d58fc0, fn_7000d58fc0 as *const (), 0), // +0x66cfc0 sub_66cfc0
    Lifted::new(0x7000d590a0, fn_7000d590a0 as *const (), 0), // +0x66d0a0 sub_66d0a0
    Lifted::new(0x7000d59180, tpl_1289::<0x1c8, 0x70007491b0, 0x82, 0xa8, 0x7000d59900> as *const (), 0), // +0x66d180 sub_66d180
    Lifted::new(0x7000d59210, fn_7000d59210 as *const (), 0), // +0x66d210 sub_66d210
    Lifted::new(0x7000d59900, fn_7000d59900 as *const (), 0), // +0x66d900 sub_66d900
    Lifted::new(0x7000d59cd0, fn_7000d59cd0 as *const (), 0), // +0x66dcd0 sub_66dcd0
    Lifted::new(0x7000d59d80, fn_7000d59d80 as *const (), 2), // +0x66dd80 sub_66dd80
    Lifted::new(0x7000d59e70, tpl_1216::<0x48, 0x7000779880> as *const (), 0), // +0x66de70 sub_66de70
    Lifted::new(0x7000d59e80, tpl_1216::<0x48, 0x70007248b0> as *const (), 0), // +0x66de80 sub_66de80
    Lifted::new(0x7000d59e90, tpl_1216::<0x48, 0x700070a110> as *const (), 0), // +0x66de90 sub_66de90
    Lifted::new(0x7000d59ea0, fn_7000d59ea0 as *const (), 0), // +0x66dea0 sub_66dea0
    Lifted::new(0x7000d59f00, fn_7000d59f00 as *const (), 0), // +0x66df00 sub_66df00
    Lifted::new(0x7000d59fb0, fn_7000d59fb0 as *const (), 0), // +0x66dfb0 sub_66dfb0
    Lifted::new(0x7000d5a040, fn_7000d5a040 as *const (), 0), // +0x66e040 sub_66e040
    Lifted::new(0x7000d5a070, fn_7000d5a070 as *const (), 0), // +0x66e070 sub_66e070
    Lifted::new(0x7000d5a130, fn_7000d5a130 as *const (), 0), // +0x66e130 sub_66e130
    Lifted::new(0x7000d5a820, fn_7000d5a820 as *const (), 0), // +0x66e820 sub_66e820
    Lifted::new(0x7000d5b200, fn_7000d5b200 as *const (), 1), // +0x66f200 sub_66f200
    Lifted::new(0x7000d5b280, fn_7000d5b280 as *const (), 1), // +0x66f280 sub_66f280
    Lifted::new(0x7000d5b310, fn_7000d5b310 as *const (), 0), // +0x66f310 sub_66f310
    Lifted::new(0x7000d5b450, fn_7000d5b450 as *const (), 0), // +0x66f450 sub_66f450
    Lifted::new(0x7000d5b550, fn_7000d5b550 as *const (), 0), // +0x66f550 sub_66f550
    Lifted::new(0x7000d5b630, fn_7000d5b630 as *const (), 0), // +0x66f630 sub_66f630
    Lifted::new(0x7000d5b740, fn_7000d5b740 as *const (), 0), // +0x66f740 sub_66f740
    Lifted::new(0x7000d5b8c0, fn_7000d5b8c0 as *const (), 0), // +0x66f8c0 sub_66f8c0
    Lifted::new(0x7000d5b9a0, fn_7000d5b9a0 as *const (), 0), // +0x66f9a0 sub_66f9a0
    Lifted::new(0x7000d5bb60, fn_7000d5bb60 as *const (), 0), // +0x66fb60 sub_66fb60
    Lifted::new(0x7000d5bbe0, fn_7000d5bbe0 as *const (), 2), // +0x66fbe0 sub_66fbe0
    Lifted::new(0x7000d5bc80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000d5bc91> as *const (), 0), // +0x66fc80 sub_66fc80
    Lifted::new(0x7000d5bca0, fn_7000d5bca0 as *const (), 0), // +0x66fca0 sub_66fca0
    Lifted::new(0x7000d5bcd0, fn_7000d5bcd0 as *const (), 2), // +0x66fcd0 sub_66fcd0
    Lifted::new(0x7000d5c030, fn_7000d5c030 as *const (), 0), // +0x670030 sub_670030
    Lifted::new(0x7000d5c0c0, tpl_1290::<0x7000a47050> as *const (), 0), // +0x6700c0 sub_6700c0
    Lifted::new(0x7000d5c0e0, tpl_1291::<0x7000a48290, 0x7000a47a50, 0x700076bf20, 0x7000d4e9e0> as *const (), 0), // +0x6700e0 sub_6700e0
    Lifted::new(0x7000d5c260, tpl_1291::<0x7000a48290, 0x7000a47a50, 0x700076bf20, 0x7000d4ec40> as *const (), 0), // +0x670260 sub_670260
    Lifted::new(0x7000d5c4c0, tpl_14 as *const (), 1), // +0x6704c0 sub_6704c0
    Lifted::new(0x7000d5c4d0, fn_7000d5c4d0 as *const (), 0), // +0x6704d0 sub_6704d0
    Lifted::new(0x7000d5c580, fn_7000d5c580 as *const (), 0), // +0x670580 sub_670580
    Lifted::new(0x7000d5cd30, fn_7000d5cd30 as *const (), 0), // +0x670d30 sub_670d30
    Lifted::new(0x7000d5ce30, fn_7000d5ce30 as *const (), 0), // +0x670e30 sub_670e30
    Lifted::new(0x7000d5d100, tpl_14 as *const (), 1), // +0x671100 sub_671100
    Lifted::new(0x7000d5d110, tpl_552::<0x8, 0x700077aad0, 0x7000d659b0> as *const (), 0), // +0x671110 sub_671110
    Lifted::new(0x7000d5d170, fn_7000d5d170 as *const (), 0), // +0x671170 sub_671170
    Lifted::new(0x7000d5d2d0, fn_7000d5d2d0 as *const (), 0), // +0x6712d0 sub_6712d0
    Lifted::new(0x7000d5d480, fn_7000d5d480 as *const (), 0), // +0x671480 sub_671480
    Lifted::new(0x7000d5d500, fn_7000d5d500 as *const (), 0), // +0x671500 sub_671500
    Lifted::new(0x7000d5d6f0, fn_7000d5d6f0 as *const (), 0), // +0x6716f0 sub_6716f0
    Lifted::new(0x7000d5d730, fn_7000d5d730 as *const (), 0), // +0x671730 sub_671730
    Lifted::new(0x7000d5d7d0, fn_7000d5d7d0 as *const (), 0), // +0x6717d0 sub_6717d0
    Lifted::new(0x7000d5d870, fn_7000d5d870 as *const (), 0), // +0x671870 sub_671870
    Lifted::new(0x7000d5d930, fn_7000d5d930 as *const (), 0), // +0x671930 sub_671930
    Lifted::new(0x7000d5daf0, fn_7000d5daf0 as *const (), 0), // +0x671af0 sub_671af0
    Lifted::new(0x7000d5dc70, fn_7000d5dc70 as *const (), 0), // +0x671c70 sub_671c70
    Lifted::new(0x7000d5dd20, fn_7000d5dd20 as *const (), 0), // +0x671d20 sub_671d20
    Lifted::new(0x7000d5ea00, fn_7000d5ea00 as *const (), 0), // +0x672a00 sub_672a00
    Lifted::new(0x7000d5ec10, fn_7000d5ec10 as *const (), 0), // +0x672c10 sub_672c10
    Lifted::new(0x7000d5ecb0, fn_7000d5ecb0 as *const (), 0), // +0x672cb0 sub_672cb0
    Lifted::new(0x7000d5f1b0, fn_7000d5f1b0 as *const (), 0), // +0x6731b0 sub_6731b0
    Lifted::new(0x7000d5f390, fn_7000d5f390 as *const (), 0), // +0x673390 sub_673390
    Lifted::new(0x7000d5f3d0, fn_7000d5f3d0 as *const (), 0), // +0x6733d0 sub_6733d0
    Lifted::new(0x7000d5f6f0, fn_7000d5f6f0 as *const (), 0), // +0x6736f0 sub_6736f0
    Lifted::new(0x7000d5fe20, fn_7000d5fe20 as *const (), 0), // +0x673e20 sub_673e20
    Lifted::new(0x7000d5ff20, fn_7000d5ff20 as *const (), 0), // +0x673f20 sub_673f20
    Lifted::new(0x7000d5ffd0, fn_7000d5ffd0 as *const (), 0), // +0x673fd0 sub_673fd0
    Lifted::new(0x7000d60b50, fn_7000d60b50 as *const (), 0), // +0x674b50 sub_674b50
    Lifted::new(0x7000d60c00, fn_7000d60c00 as *const (), 0), // +0x674c00 sub_674c00
    Lifted::new(0x7000d61050, fn_7000d61050 as *const (), 0), // +0x675050 sub_675050
    Lifted::new(0x7000d61380, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000d61391> as *const (), 0), // +0x675380 sub_675380
    Lifted::new(0x7000d613e0, fn_7000d613e0 as *const (), 0), // +0x6753e0 sub_6753e0
    Lifted::new(0x7000d614c0, fn_7000d614c0 as *const (), 0), // +0x6754c0 sub_6754c0
    Lifted::new(0x7000d61570, fn_7000d61570 as *const (), 0), // +0x675570 sub_675570
    Lifted::new(0x7000d61a90, fn_7000d61a90 as *const (), 0), // +0x675a90 sub_675a90
    Lifted::new(0x7000d61b40, fn_7000d61b40 as *const (), 0), // +0x675b40 sub_675b40
    Lifted::new(0x7000d61d00, fn_7000d61d00 as *const (), 0), // +0x675d00 sub_675d00
    Lifted::new(0x7000d61d90, fn_7000d61d90 as *const (), 0), // +0x675d90 sub_675d90
    Lifted::new(0x7000d62080, fn_7000d62080 as *const (), 0), // +0x676080 sub_676080
    Lifted::new(0x7000d62200, fn_7000d62200 as *const (), 0), // +0x676200 sub_676200
    Lifted::new(0x7000d623e0, fn_7000d623e0 as *const (), 0), // +0x6763e0 sub_6763e0
    Lifted::new(0x7000d62500, fn_7000d62500 as *const (), 0), // +0x676500 sub_676500
    Lifted::new(0x7000d625d0, fn_7000d625d0 as *const (), 0), // +0x6765d0 sub_6765d0
    Lifted::new(0x7000d62740, fn_7000d62740 as *const (), 0), // +0x676740 sub_676740
    Lifted::new(0x7000d62880, fn_7000d62880 as *const (), 0), // +0x676880 sub_676880
    Lifted::new(0x7000d62ae0, fn_7000d62ae0 as *const (), 0), // +0x676ae0 sub_676ae0
    Lifted::new(0x7000d62b90, fn_7000d62b90 as *const (), 0), // +0x676b90 sub_676b90
    Lifted::new(0x7000d62d10, fn_7000d62d10 as *const (), 0), // +0x676d10 sub_676d10
    Lifted::new(0x7000d63060, fn_7000d63060 as *const (), 0), // +0x677060 sub_677060
    Lifted::new(0x7000d63110, fn_7000d63110 as *const (), 0), // +0x677110 sub_677110
    Lifted::new(0x7000d631d0, fn_7000d631d0 as *const (), 0), // +0x6771d0 sub_6771d0
    Lifted::new(0x7000d63450, fn_7000d63450 as *const (), 2), // +0x677450 sub_677450
    Lifted::new(0x7000d635c0, fn_7000d635c0 as *const (), 0), // +0x6775c0 sub_6775c0
    Lifted::new(0x7000d636c0, fn_7000d636c0 as *const (), 0), // +0x6776c0 sub_6776c0
    Lifted::new(0x7000d63890, tpl_1292::<0x7000a49b90, 0x7000d613e0, 0x18, 0x700071fe20, 0x7000a49bc0, 0x70007034e0, 0x700070f280, 0x1f0, 0x20, 0x70007a4620> as *const (), 0), // +0x677890 sub_677890
    Lifted::new(0x7000d63970, fn_7000d63970 as *const (), 0), // +0x677970 sub_677970
    Lifted::new(0x7000d63b40, tpl_1292::<0x7000a49bc0, 0x7000d613e0, 0x18, 0x700071fe20, 0x7000a49b90, 0x70007034e0, 0x7000784540, 0x1f0, 0x20, 0x70007a4620> as *const (), 0), // +0x677b40 sub_677b40
    Lifted::new(0x7000d63c20, fn_7000d63c20 as *const (), 0), // +0x677c20 sub_677c20
    Lifted::new(0x7000d63cc0, fn_7000d63cc0 as *const (), 0), // +0x677cc0 sub_677cc0
    Lifted::new(0x7000d63e80, fn_7000d63e80 as *const (), 0), // +0x677e80 sub_677e80
    Lifted::new(0x7000d640d0, fn_7000d640d0 as *const (), 0), // +0x6780d0 sub_6780d0
    Lifted::new(0x7000d64280, fn_7000d64280 as *const (), 0), // +0x678280 sub_678280
    Lifted::new(0x7000d64730, fn_7000d64730 as *const (), 0), // +0x678730 sub_678730
    Lifted::new(0x7000d649f0, fn_7000d649f0 as *const (), 0), // +0x6789f0 sub_6789f0
    Lifted::new(0x7000d64b60, fn_7000d64b60 as *const (), 0), // +0x678b60 sub_678b60
    Lifted::new(0x7000d64ec0, fn_7000d64ec0 as *const (), 0), // +0x678ec0 sub_678ec0
    Lifted::new(0x7000d65220, fn_7000d65220 as *const (), 0), // +0x679220 sub_679220
    Lifted::new(0x7000d653a0, fn_7000d653a0 as *const (), 0), // +0x6793a0 sub_6793a0
    Lifted::new(0x7000d65680, fn_7000d65680 as *const (), 0), // +0x679680 sub_679680
    Lifted::new(0x7000d65840, fn_7000d65840 as *const (), 0), // +0x679840 sub_679840
    Lifted::new(0x7000d65900, fn_7000d65900 as *const (), 0), // +0x679900 sub_679900
    Lifted::new(0x7000d659b0, fn_7000d659b0 as *const (), 2), // +0x6799b0 sub_6799b0
    Lifted::new(0x7000d67590, fn_7000d67590 as *const (), 0), // +0x67b590 sub_67b590
    Lifted::new(0x7000d67700, fn_7000d67700 as *const (), 0), // +0x67b700 sub_67b700
    Lifted::new(0x7000d67900, fn_7000d67900 as *const (), 0), // +0x67b900 sub_67b900
    Lifted::new(0x7000d67a30, fn_7000d67a30 as *const (), 2), // +0x67ba30 sub_67ba30
    Lifted::new(0x7000d6a3c0, fn_7000d6a3c0 as *const (), 0), // +0x67e3c0 sub_67e3c0
    Lifted::new(0x7000d6a500, fn_7000d6a500 as *const (), 0), // +0x67e500 sub_67e500
    Lifted::new(0x7000d6a5d0, fn_7000d6a5d0 as *const (), 0), // +0x67e5d0 sub_67e5d0
    Lifted::new(0x7000d6a910, fn_7000d6a910 as *const (), 0), // +0x67e910 sub_67e910
    Lifted::new(0x7000d6a9e0, fn_7000d6a9e0 as *const (), 0), // +0x67e9e0 sub_67e9e0
    Lifted::new(0x7000d6aa40, fn_7000d6aa40 as *const (), 0), // +0x67ea40 sub_67ea40
    Lifted::new(0x7000d6aaa0, fn_7000d6aaa0 as *const (), 0), // +0x67eaa0 sub_67eaa0
    Lifted::new(0x7000d6ac20, fn_7000d6ac20 as *const (), 0), // +0x67ec20 sub_67ec20
    Lifted::new(0x7000d6ad10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000d6ad21> as *const (), 0), // +0x67ed10 sub_67ed10
    Lifted::new(0x7000d6ad50, fn_7000d6ad50 as *const (), 2), // +0x67ed50 sub_67ed50
    Lifted::new(0x7000d6b900, fn_7000d6b900 as *const (), 0), // +0x67f900 sub_67f900
    Lifted::new(0x7000d6bb20, tpl_1289::<0x1c8, 0x70007491b0, 0x82, 0xa8, 0x7000d59900> as *const (), 0), // +0x67fb20 sub_67fb20
    Lifted::new(0x7000d6bb80, fn_7000d6bb80 as *const (), 0), // +0x67fb80 sub_67fb80
    Lifted::new(0x7000d6bbf0, fn_7000d6bbf0 as *const (), 0), // +0x67fbf0 sub_67fbf0
    Lifted::new(0x7000d6bc70, fn_7000d6bc70 as *const (), 0), // +0x67fc70 sub_67fc70
    Lifted::new(0x7000d6be50, fn_7000d6be50 as *const (), 0), // +0x67fe50 sub_67fe50
    Lifted::new(0x7000d6bea0, fn_7000d6bea0 as *const (), 0), // +0x67fea0 sub_67fea0
    Lifted::new(0x7000d6bf10, fn_7000d6bf10 as *const (), 0), // +0x67ff10 sub_67ff10
    Lifted::new(0x7000d6c260, fn_7000d6c260 as *const (), 0), // +0x680260 sub_680260
    Lifted::new(0x7000d6c4e0, fn_7000d6c4e0 as *const (), 0), // +0x6804e0 sub_6804e0
    Lifted::new(0x7000d6c510, fn_7000d6c510 as *const (), 0), // +0x680510 sub_680510
    Lifted::new(0x7000d6c7b0, fn_7000d6c7b0 as *const (), 0), // +0x6807b0 sub_6807b0
    Lifted::new(0x7000d6c920, fn_7000d6c920 as *const (), 0), // +0x680920 sub_680920
    Lifted::new(0x7000d6cda0, fn_7000d6cda0 as *const (), 0), // +0x680da0 sub_680da0
    Lifted::new(0x7000d6d9e0, tpl_287 as *const (), 1), // +0x6819e0 sub_6819e0
    Lifted::new(0x7000d6dd00, tpl_287 as *const (), 1), // +0x681d00 sub_681d00
    Lifted::new(0x7000d6e020, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e030> as *const (), 0), // +0x682020 sub_682020
    Lifted::new(0x7000d6e040, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e050> as *const (), 0), // +0x682040 sub_682040
    Lifted::new(0x7000d6e060, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e070> as *const (), 0), // +0x682060 sub_682060
    Lifted::new(0x7000d6e080, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e090> as *const (), 0), // +0x682080 sub_682080
    Lifted::new(0x7000d6e0a0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e0b0> as *const (), 0), // +0x6820a0 sub_6820a0
    Lifted::new(0x7000d6e0c0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e0d0> as *const (), 0), // +0x6820c0 sub_6820c0
    Lifted::new(0x7000d6e0e0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e0f0> as *const (), 0), // +0x6820e0 sub_6820e0
    Lifted::new(0x7000d6e100, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e110> as *const (), 0), // +0x682100 sub_682100
    Lifted::new(0x7000d6e120, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e130> as *const (), 0), // +0x682120 sub_682120
    Lifted::new(0x7000d6e140, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d6e150> as *const (), 0), // +0x682140 sub_682140
    Lifted::new(0x7000d6e160, fn_7000d6e160 as *const (), 1), // +0x682160 sub_682160
    Lifted::new(0x7000d6e1a0, tpl_52::<0x10> as *const (), 1), // +0x6821a0 sub_6821a0
    Lifted::new(0x7000d6e1b0, tpl_36::<0x8> as *const (), 1), // +0x6821b0 sub_6821b0
    Lifted::new(0x7000d6e1c0, fn_7000d6e1c0 as *const (), 0), // +0x6821c0 sub_6821c0
    Lifted::new(0x7000d6e220, tpl_52::<0x50> as *const (), 1), // +0x682220 sub_682220
    Lifted::new(0x7000d6e230, tpl_36::<0x10> as *const (), 1), // +0x682230 sub_682230
    Lifted::new(0x7000d6e240, tpl_20 as *const (), 1), // +0x682240 sub_682240
    Lifted::new(0x7000d6e250, tpl_1293::<0x8> as *const (), 1), // +0x682250 sub_682250
    Lifted::new(0x7000d6e270, tpl_52::<0x30> as *const (), 1), // +0x682270 sub_682270
    Lifted::new(0x7000d6e280, fn_7000d6e280 as *const (), 0), // +0x682280 sub_682280
    Lifted::new(0x7000d6e2e0, fn_7000d6e2e0 as *const (), 2), // +0x6822e0 sub_6822e0
    Lifted::new(0x7000d6e3a0, tpl_1294::<0x7001d54378> as *const (), 0), // +0x6823a0 sub_6823a0
    Lifted::new(0x7000d6e3c0, fn_7000d6e3c0 as *const (), 1), // +0x6823c0 sub_6823c0
    Lifted::new(0x7000d6e430, fn_7000d6e430 as *const (), 0), // +0x682430 sub_682430
    Lifted::new(0x7000d6e450, fn_7000d6e450 as *const (), 0), // +0x682450 sub_682450
    Lifted::new(0x7000d6e530, fn_7000d6e530 as *const (), 0), // +0x682530 sub_682530
    Lifted::new(0x7000d6e5a0, fn_7000d6e5a0 as *const (), 0), // +0x6825a0 sub_6825a0
    Lifted::new(0x7000d6e630, fn_7000d6e630 as *const (), 0), // +0x682630 sub_682630
    Lifted::new(0x7000d6e810, tpl_55::<0x700177c0d0> as *const (), 0), // +0x682810 sub_682810
    Lifted::new(0x7000d6e850, tpl_898::<0x700177c0e0> as *const (), 0), // +0x682850 sub_682850
    Lifted::new(0x7000d6e890, tpl_392::<0x700177c0e0> as *const (), 0), // +0x682890 sub_682890
    Lifted::new(0x7000d6e8c0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x6828c0 sub_6828c0
    Lifted::new(0x7000d6e900, fn_7000d6e900 as *const (), 0), // +0x682900 sub_682900
    Lifted::new(0x7000d6eb40, fn_7000d6eb40 as *const (), 0), // +0x682b40 sub_682b40
    Lifted::new(0x7000d6ec80, fn_7000d6ec80 as *const (), 0), // +0x682c80 sub_682c80
    Lifted::new(0x7000d6ed10, tpl_33::<0x700177c312> as *const (), 0), // +0x682d10 sub_682d10
    Lifted::new(0x7000d6ed40, fn_7000d6ed40 as *const (), 0), // +0x682d40 sub_682d40
    Lifted::new(0x7000d6ee20, fn_7000d6ee20 as *const (), 1), // +0x682e20 sub_682e20
    Lifted::new(0x7000d6ee70, fn_7000d6ee70 as *const (), 0), // +0x682e70 sub_682e70
    Lifted::new(0x7000d6ef30, fn_7000d6ef30 as *const (), 1), // +0x682f30 sub_682f30
    Lifted::new(0x7000d6ef80, fn_7000d6ef80 as *const (), 0), // +0x682f80 sub_682f80
    Lifted::new(0x7000d6f210, fn_7000d6f210 as *const (), 0), // +0x683210 sub_683210
    Lifted::new(0x7000d6f2c0, fn_7000d6f2c0 as *const (), 0), // +0x6832c0 sub_6832c0
    Lifted::new(0x7000d6f420, fn_7000d6f420 as *const (), 1), // +0x683420 sub_683420
    Lifted::new(0x7000d6f470, fn_7000d6f470 as *const (), 0), // +0x683470 sub_683470
    Lifted::new(0x7000d6f510, fn_7000d6f510 as *const (), 0), // +0x683510 sub_683510
    Lifted::new(0x7000d6f6c0, fn_7000d6f6c0 as *const (), 0), // +0x6836c0 sub_6836c0
    Lifted::new(0x7000d6f730, fn_7000d6f730 as *const (), 0), // +0x683730 sub_683730
    Lifted::new(0x7000d6f7d0, fn_7000d6f7d0 as *const (), 0), // +0x6837d0 sub_6837d0
    Lifted::new(0x7000d6f880, fn_7000d6f880 as *const (), 0), // +0x683880 sub_683880
    Lifted::new(0x7000d6f970, fn_7000d6f970 as *const (), 0), // +0x683970 sub_683970
    Lifted::new(0x7000d6fad0, fn_7000d6fad0 as *const (), 0), // +0x683ad0 sub_683ad0
    Lifted::new(0x7000d6fb90, fn_7000d6fb90 as *const (), 0), // +0x683b90 sub_683b90
    Lifted::new(0x7000d6fc60, fn_7000d6fc60 as *const (), 1), // +0x683c60 sub_683c60
    Lifted::new(0x7000d6fcc0, fn_7000d6fcc0 as *const (), 0), // +0x683cc0 sub_683cc0
    Lifted::new(0x7000d6fde0, tpl_1295::<0x700076bf20, 0x700177c0d0, 0x700177c0e0, 0x7000d70090, 0x7000d6ff67> as *const (), 0), // +0x683de0 sub_683de0
    Lifted::new(0x7000d6ffd0, tpl_1296::<0x30, 0xffffffffffffffff, 0x28, 0x20, 0x18, 0x8> as *const (), 1), // +0x683fd0 sub_683fd0
    Lifted::new(0x7000d70090, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000d700a0> as *const (), 0), // +0x684090 sub_684090
    Lifted::new(0x7000d700b0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d700c0> as *const (), 0), // +0x6840b0 sub_6840b0
    Lifted::new(0x7000d700d0, tpl_1297::<0x700076bf20, 0x700177c0e0, 0x7000d700b0, 0x7000d70227> as *const (), 0), // +0x6840d0 sub_6840d0
    Lifted::new(0x7000d70280, fn_7000d70280 as *const (), 1), // +0x684280 sub_684280
    Lifted::new(0x7000d702d0, fn_7000d702d0 as *const (), 1), // +0x6842d0 sub_6842d0
    Lifted::new(0x7000d702e0, fn_7000d702e0 as *const (), 1), // +0x6842e0 sub_6842e0
    Lifted::new(0x7000d70320, fn_7000d70320 as *const (), 1), // +0x684320 sub_684320
    Lifted::new(0x7000d70330, fn_7000d70330 as *const (), 0), // +0x684330 sub_684330
    Lifted::new(0x7000d70680, fn_7000d70680 as *const (), 1), // +0x684680 sub_684680
    Lifted::new(0x7000d70690, fn_7000d70690 as *const (), 1), // +0x684690 sub_684690
    Lifted::new(0x7000d706a0, fn_7000d706a0 as *const (), 1), // +0x6846a0 sub_6846a0
    Lifted::new(0x7000d706b0, fn_7000d706b0 as *const (), 1), // +0x6846b0 sub_6846b0
    Lifted::new(0x7000d706c0, fn_7000d706c0 as *const (), 1), // +0x6846c0 sub_6846c0
    Lifted::new(0x7000d706d0, fn_7000d706d0 as *const (), 1), // +0x6846d0 sub_6846d0
    Lifted::new(0x7000d706e0, fn_7000d706e0 as *const (), 1), // +0x6846e0 sub_6846e0
    Lifted::new(0x7000d706f0, fn_7000d706f0 as *const (), 1), // +0x6846f0 sub_6846f0
    Lifted::new(0x7000d70700, fn_7000d70700 as *const (), 1), // +0x684700 sub_684700
    Lifted::new(0x7000d70710, fn_7000d70710 as *const (), 2), // +0x684710 sub_684710
    Lifted::new(0x7000d707d0, fn_7000d707d0 as *const (), 1), // +0x6847d0 sub_6847d0
    Lifted::new(0x7000d707e0, fn_7000d707e0 as *const (), 0), // +0x6847e0 sub_6847e0
    Lifted::new(0x7000d70860, fn_7000d70860 as *const (), 0), // +0x684860 sub_684860
    Lifted::new(0x7000d70890, fn_7000d70890 as *const (), 1), // +0x684890 sub_684890
    Lifted::new(0x7000d70900, fn_7000d70900 as *const (), 0), // +0x684900 sub_684900
    Lifted::new(0x7000d709a0, fn_7000d709a0 as *const (), 0), // +0x6849a0 sub_6849a0
    Lifted::new(0x7000d70a00, fn_7000d70a00 as *const (), 0), // +0x684a00 sub_684a00
    Lifted::new(0x7000d70aa0, fn_7000d70aa0 as *const (), 1), // +0x684aa0 sub_684aa0
    Lifted::new(0x7000d70af0, fn_7000d70af0 as *const (), 0), // +0x684af0 sub_684af0
    Lifted::new(0x7000d70b50, fn_7000d70b50 as *const (), 1), // +0x684b50 sub_684b50
    Lifted::new(0x7000d70b70, fn_7000d70b70 as *const (), 1), // +0x684b70 sub_684b70
    Lifted::new(0x7000d70b90, fn_7000d70b90 as *const (), 0), // +0x684b90 sub_684b90
    Lifted::new(0x7000d70bf0, fn_7000d70bf0 as *const (), 1), // +0x684bf0 sub_684bf0
    Lifted::new(0x7000d70c20, fn_7000d70c20 as *const (), 0), // +0x684c20 sub_684c20
    Lifted::new(0x7000d70d40, fn_7000d70d40 as *const (), 0), // +0x684d40 sub_684d40
    Lifted::new(0x7000d70de0, fn_7000d70de0 as *const (), 1), // +0x684de0 sub_684de0
    Lifted::new(0x7000d70e20, fn_7000d70e20 as *const (), 2), // +0x684e20 sub_684e20
    Lifted::new(0x7000d710a0, fn_7000d710a0 as *const (), 0), // +0x6850a0 sub_6850a0
    Lifted::new(0x7000d72050, tpl_1298::<0x18, 0xffffffffffffffff, 0x8, 0x70017d2c10, 0x7001d54ef8, 0x7000d72099, 0x28, 0x30, 0x7000d72810> as *const (), 0), // +0x686050 sub_686050
    Lifted::new(0x7000d72140, tpl_1298::<0x18, 0xffffffffffffffff, 0x8, 0x70017d2c10, 0x7001d54ef8, 0x7000d72189, 0x28, 0x30, 0x7000d72b50> as *const (), 0), // +0x686140 sub_686140
    Lifted::new(0x7000d72290, tpl_1299::<0x70007227b0, 0x7000d72810, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x686290 sub_686290
    Lifted::new(0x7000d72550, tpl_1299::<0x7000718450, 0x7000d72b50, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x686550 sub_686550
    Lifted::new(0x7000d72810, fn_7000d72810 as *const (), 2), // +0x686810 sub_686810
    Lifted::new(0x7000d72b50, fn_7000d72b50 as *const (), 2), // +0x686b50 sub_686b50
    Lifted::new(0x7000d72f20, tpl_1285::<0x700076bf20, 0x7000d75680, 0x7000d73085> as *const (), 2), // +0x686f20 sub_686f20
    Lifted::new(0x7000d730e0, tpl_582::<0x8, 0x10, 0x7000d73cd0> as *const (), 0), // +0x6870e0 sub_6870e0
    Lifted::new(0x7000d73110, fn_7000d73110 as *const (), 0), // +0x687110 sub_687110
    Lifted::new(0x7000d734a0, fn_7000d734a0 as *const (), 0), // +0x6874a0 sub_6874a0
    Lifted::new(0x7000d73520, fn_7000d73520 as *const (), 2), // +0x687520 sub_687520
    Lifted::new(0x7000d735b0, fn_7000d735b0 as *const (), 2), // +0x6875b0 sub_6875b0
    Lifted::new(0x7000d73680, fn_7000d73680 as *const (), 2), // +0x687680 sub_687680
    Lifted::new(0x7000d738c0, fn_7000d738c0 as *const (), 0), // +0x6878c0 sub_6878c0
    Lifted::new(0x7000d739d0, fn_7000d739d0 as *const (), 0), // +0x6879d0 sub_6879d0
    Lifted::new(0x7000d73a50, fn_7000d73a50 as *const (), 0), // +0x687a50 sub_687a50
    Lifted::new(0x7000d73ac0, fn_7000d73ac0 as *const (), 0), // +0x687ac0 sub_687ac0
    Lifted::new(0x7000d73c70, fn_7000d73c70 as *const (), 0), // +0x687c70 sub_687c70
    Lifted::new(0x7000d73cd0, tpl_1300::<0x10, 0x18, 0x700076bf20, 0x8, 0x70017d8de0, 0x7001d54ef8, 0x7000d73d22, 0xfffffffffffffff8, 0x14> as *const (), 0), // +0x687cd0 sub_687cd0
    Lifted::new(0x7000d73d80, fn_7000d73d80 as *const (), 2), // +0x687d80 sub_687d80
    Lifted::new(0x7000d73fe0, fn_7000d73fe0 as *const (), 2), // +0x687fe0 sub_687fe0
    Lifted::new(0x7000d74150, fn_7000d74150 as *const (), 2), // +0x688150 sub_688150
    Lifted::new(0x7000d742d0, fn_7000d742d0 as *const (), 2), // +0x6882d0 sub_6882d0
    Lifted::new(0x7000d74450, fn_7000d74450 as *const (), 0), // +0x688450 sub_688450
    Lifted::new(0x7000d744c0, fn_7000d744c0 as *const (), 2), // +0x6884c0 sub_6884c0
    Lifted::new(0x7000d74590, fn_7000d74590 as *const (), 2), // +0x688590 sub_688590
    Lifted::new(0x7000d74660, fn_7000d74660 as *const (), 2), // +0x688660 sub_688660
    Lifted::new(0x7000d74700, fn_7000d74700 as *const (), 2), // +0x688700 sub_688700
    Lifted::new(0x7000d747d0, fn_7000d747d0 as *const (), 2), // +0x6887d0 sub_6887d0
    Lifted::new(0x7000d748a0, fn_7000d748a0 as *const (), 2), // +0x6888a0 sub_6888a0
    Lifted::new(0x7000d74970, fn_7000d74970 as *const (), 2), // +0x688970 sub_688970
    Lifted::new(0x7000d74a40, fn_7000d74a40 as *const (), 2), // +0x688a40 sub_688a40
    Lifted::new(0x7000d74b10, fn_7000d74b10 as *const (), 2), // +0x688b10 sub_688b10
    Lifted::new(0x7000d74be0, fn_7000d74be0 as *const (), 2), // +0x688be0 sub_688be0
    Lifted::new(0x7000d74cb0, fn_7000d74cb0 as *const (), 0), // +0x688cb0 sub_688cb0
    Lifted::new(0x7000d74d30, fn_7000d74d30 as *const (), 0), // +0x688d30 sub_688d30
    Lifted::new(0x7000d74e10, fn_7000d74e10 as *const (), 0), // +0x688e10 sub_688e10
    Lifted::new(0x7000d74e80, fn_7000d74e80 as *const (), 2), // +0x688e80 sub_688e80
    Lifted::new(0x7000d75040, tpl_287 as *const (), 1), // +0x689040 sub_689040
    Lifted::new(0x7000d75360, tpl_287 as *const (), 1), // +0x689360 sub_689360
    Lifted::new(0x7000d75680, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d75690> as *const (), 0), // +0x689680 sub_689680
    Lifted::new(0x7000d756a0, fn_7000d756a0 as *const (), 1), // +0x6896a0 sub_6896a0
    Lifted::new(0x7000d75700, fn_7000d75700 as *const (), 0), // +0x689700 sub_689700
    Lifted::new(0x7000d75790, fn_7000d75790 as *const (), 0), // +0x689790 sub_689790
    Lifted::new(0x7000d75860, fn_7000d75860 as *const (), 0), // +0x689860 sub_689860
    Lifted::new(0x7000d758a0, fn_7000d758a0 as *const (), 0), // +0x6898a0 sub_6898a0
    Lifted::new(0x7000d758f0, fn_7000d758f0 as *const (), 0), // +0x6898f0 sub_6898f0
    Lifted::new(0x7000d75950, fn_7000d75950 as *const (), 0), // +0x689950 sub_689950
    Lifted::new(0x7000d759d0, fn_7000d759d0 as *const (), 0), // +0x6899d0 sub_6899d0
    Lifted::new(0x7000d75a40, fn_7000d75a40 as *const (), 0), // +0x689a40 sub_689a40
    Lifted::new(0x7000d75a80, tpl_1301::<0x10, 0x8, 0x700079f760> as *const (), 0), // +0x689a80 sub_689a80
    Lifted::new(0x7000d75aa0, tpl_1301::<0x10, 0x8, 0x700079f760> as *const (), 0), // +0x689aa0 sub_689aa0
    Lifted::new(0x7000d75ac0, fn_7000d75ac0 as *const (), 0), // +0x689ac0 sub_689ac0
    Lifted::new(0x7000d75ad0, fn_7000d75ad0 as *const (), 0), // +0x689ad0 sub_689ad0
    Lifted::new(0x7000d75b70, fn_7000d75b70 as *const (), 0), // +0x689b70 sub_689b70
    Lifted::new(0x7000d75be0, fn_7000d75be0 as *const (), 0), // +0x689be0 sub_689be0
    Lifted::new(0x7000d75bf0, fn_7000d75bf0 as *const (), 0), // +0x689bf0 sub_689bf0
    Lifted::new(0x7000d75c60, fn_7000d75c60 as *const (), 0), // +0x689c60 sub_689c60
    Lifted::new(0x7000d75c70, fn_7000d75c70 as *const (), 0), // +0x689c70 sub_689c70
    Lifted::new(0x7000d75cc0, tpl_1302::<0x70007034e0, 0x8, 0x700077d800> as *const (), 0), // +0x689cc0 sub_689cc0
    Lifted::new(0x7000d75cf0, tpl_1303::<0x70007034e0, 0x8, 0x700070f280> as *const (), 0), // +0x689cf0 sub_689cf0
    Lifted::new(0x7000d75d30, tpl_1302::<0x70007034e0, 0x8, 0x7000786de0> as *const (), 0), // +0x689d30 sub_689d30
    Lifted::new(0x7000d75d60, tpl_1303::<0x70007034e0, 0x8, 0x7000784540> as *const (), 0), // +0x689d60 sub_689d60
    Lifted::new(0x7000d75da0, tpl_1302::<0x70007034e0, 0x8, 0x7000766710> as *const (), 0), // +0x689da0 sub_689da0
    Lifted::new(0x7000d75dd0, fn_7000d75dd0 as *const (), 0), // +0x689dd0 sub_689dd0
    Lifted::new(0x7000d75ec0, tpl_1304::<0x18, 0x8, 0x7000779490> as *const (), 0), // +0x689ec0 sub_689ec0
    Lifted::new(0x7000d75ed0, tpl_15 as *const (), 1), // +0x689ed0 sub_689ed0
    Lifted::new(0x7000d75ee0, fn_7000d75ee0 as *const (), 0), // +0x689ee0 sub_689ee0
    Lifted::new(0x7000d75f70, tpl_959::<0x10, 0x8, 0x700079fd10> as *const (), 0), // +0x689f70 sub_689f70
    Lifted::new(0x7000d75f80, fn_7000d75f80 as *const (), 0), // +0x689f80 sub_689f80
    Lifted::new(0x7000d75fd0, fn_7000d75fd0 as *const (), 0), // +0x689fd0 sub_689fd0
    Lifted::new(0x7000d76000, fn_7000d76000 as *const (), 0), // +0x68a000 sub_68a000
    Lifted::new(0x7000d76030, fn_7000d76030 as *const (), 0), // +0x68a030 sub_68a030
    Lifted::new(0x7000d76070, fn_7000d76070 as *const (), 0), // +0x68a070 sub_68a070
    Lifted::new(0x7000d76120, fn_7000d76120 as *const (), 0), // +0x68a120 sub_68a120
    Lifted::new(0x7000d76150, fn_7000d76150 as *const (), 0), // +0x68a150 sub_68a150
    Lifted::new(0x7000d76180, tpl_1304::<0x40, 0x8, 0x7000792350> as *const (), 0), // +0x68a180 sub_68a180
    Lifted::new(0x7000d76190, fn_7000d76190 as *const (), 0), // +0x68a190 sub_68a190
    Lifted::new(0x7000d761e0, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000d761f0> as *const (), 0), // +0x68a1e0 sub_68a1e0
    Lifted::new(0x7000d76200, tpl_1305::<0x700079a910, 0x700075f630, 0x70007273d0, 0x7000775cf0, 0x7000760a60, 0x70007295c0, 0x70007039b0> as *const (), 0), // +0x68a200 sub_68a200
    Lifted::new(0x7000d76330, tpl_1305::<0x700079a910, 0x7000733990, 0x70007273d0, 0x7000775cf0, 0x7000760a60, 0x70007295c0, 0x70007039b0> as *const (), 0), // +0x68a330 sub_68a330
    Lifted::new(0x7000d76460, tpl_1306::<0x700076bf20, 0x7000d76780, 0x7000d765fe> as *const (), 2), // +0x68a460 sub_68a460
    Lifted::new(0x7000d76670, tpl_404::<0x8, 0x10, 0x18> as *const (), 1), // +0x68a670 sub_68a670
    Lifted::new(0x7000d76690, fn_7000d76690 as *const (), 0), // +0x68a690 sub_68a690
    Lifted::new(0x7000d766e0, tpl_1307::<0x8, 0x18> as *const (), 1), // +0x68a6e0 sub_68a6e0
    Lifted::new(0x7000d76700, fn_7000d76700 as *const (), 1), // +0x68a700 sub_68a700
    Lifted::new(0x7000d76720, tpl_1307::<0x8, 0x1c> as *const (), 1), // +0x68a720 sub_68a720
    Lifted::new(0x7000d76740, fn_7000d76740 as *const (), 1), // +0x68a740 sub_68a740
    Lifted::new(0x7000d76760, fn_7000d76760 as *const (), 1), // +0x68a760 sub_68a760
    Lifted::new(0x7000d76780, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d76790> as *const (), 0), // +0x68a780 sub_68a780
    Lifted::new(0x7000d767a0, fn_7000d767a0 as *const (), 0), // +0x68a7a0 sub_68a7a0
    Lifted::new(0x7000d76920, fn_7000d76920 as *const (), 0), // +0x68a920 sub_68a920
    Lifted::new(0x7000d76980, fn_7000d76980 as *const (), 0), // +0x68a980 sub_68a980
    Lifted::new(0x7000d76b10, fn_7000d76b10 as *const (), 0), // +0x68ab10 sub_68ab10
    Lifted::new(0x7000d76f10, fn_7000d76f10 as *const (), 0), // +0x68af10 sub_68af10
    Lifted::new(0x7000d77300, tpl_1294::<0x7001d543b8> as *const (), 0), // +0x68b300 sub_68b300
    Lifted::new(0x7000d77320, tpl_1308::<0x7001779e20, 0x7000d774c0> as *const (), 0), // +0x68b320 sub_68b320
    Lifted::new(0x7000d773c0, tpl_1308::<0x7001779e20, 0x7000d774c0> as *const (), 0), // +0x68b3c0 sub_68b3c0
    Lifted::new(0x7000d77460, fn_7000d77460 as *const (), 0), // +0x68b460 sub_68b460
    Lifted::new(0x7000d774c0, fn_7000d774c0 as *const (), 0), // +0x68b4c0 sub_68b4c0
    Lifted::new(0x7000d77600, fn_7000d77600 as *const (), 0), // +0x68b600 sub_68b600
    Lifted::new(0x7000d77710, tpl_795::<0x1810, 0x700177c312> as *const (), 0), // +0x68b710 sub_68b710
    Lifted::new(0x7000d77760, fn_7000d77760 as *const (), 0), // +0x68b760 sub_68b760
    Lifted::new(0x7000d777e0, tpl_33::<0x700177c312> as *const (), 0), // +0x68b7e0 sub_68b7e0
    Lifted::new(0x7000d77810, fn_7000d77810 as *const (), 2), // +0x68b810 sub_68b810
    Lifted::new(0x7000d77b40, fn_7000d77b40 as *const (), 2), // +0x68bb40 sub_68bb40
    Lifted::new(0x7000d77df0, fn_7000d77df0 as *const (), 2), // +0x68bdf0 sub_68bdf0
    Lifted::new(0x7000d780a0, fn_7000d780a0 as *const (), 0), // +0x68c0a0 sub_68c0a0
    Lifted::new(0x7000d78140, fn_7000d78140 as *const (), 0), // +0x68c140 sub_68c140
    Lifted::new(0x7000d781f0, fn_7000d781f0 as *const (), 0), // +0x68c1f0 sub_68c1f0
    Lifted::new(0x7000d78240, fn_7000d78240 as *const (), 2), // +0x68c240 sub_68c240
    Lifted::new(0x7000d78640, fn_7000d78640 as *const (), 0), // +0x68c640 sub_68c640
    Lifted::new(0x7000d786a0, fn_7000d786a0 as *const (), 2), // +0x68c6a0 sub_68c6a0
    Lifted::new(0x7000d78840, fn_7000d78840 as *const (), 0), // +0x68c840 sub_68c840
    Lifted::new(0x7000d78860, fn_7000d78860 as *const (), 0), // +0x68c860 sub_68c860
    Lifted::new(0x7000d78900, fn_7000d78900 as *const (), 0), // +0x68c900 sub_68c900
    Lifted::new(0x7000d789a0, fn_7000d789a0 as *const (), 0), // +0x68c9a0 sub_68c9a0
    Lifted::new(0x7000d78a20, fn_7000d78a20 as *const (), 0), // +0x68ca20 sub_68ca20
    Lifted::new(0x7000d78ab0, tpl_918::<0x7001831270, 0x8> as *const (), 3), // +0x68cab0 sub_68cab0
    Lifted::new(0x7000d78ad0, tpl_919::<0x700177c312> as *const (), 0), // +0x68cad0 sub_68cad0
    Lifted::new(0x7000d78af0, fn_7000d78af0 as *const (), 0), // +0x68caf0 sub_68caf0
    Lifted::new(0x7000d78c00, tpl_52::<0x8> as *const (), 1), // +0x68cc00 sub_68cc00
    Lifted::new(0x7000d78c10, tpl_918::<0x7001831270, 0x8> as *const (), 3), // +0x68cc10 sub_68cc10
    Lifted::new(0x7000d78c30, tpl_315::<0x7001c6c050> as *const (), 1), // +0x68cc30 sub_68cc30
    Lifted::new(0x7000d78c40, fn_7000d78c40 as *const (), 0), // +0x68cc40 sub_68cc40
    Lifted::new(0x7000d78e60, fn_7000d78e60 as *const (), 0), // +0x68ce60 sub_68ce60
    Lifted::new(0x7000d790b0, fn_7000d790b0 as *const (), 0), // +0x68d0b0 sub_68d0b0
    Lifted::new(0x7000d79380, fn_7000d79380 as *const (), 0), // +0x68d380 sub_68d380
    Lifted::new(0x7000d79660, tpl_1309::<0x7000a3b580, 0x7000d78c40, 0x7000d7a590, 0x7001d550a8, 0x7000c17eb0> as *const (), 2), // +0x68d660 sub_68d660
    Lifted::new(0x7000d797d0, tpl_1310::<0x7000c17560, 0x7000d78e60, 0x7000d7a590, 0x7001d550a8, 0x7000c17eb0> as *const (), 2), // +0x68d7d0 sub_68d7d0
    Lifted::new(0x7000d79940, tpl_1309::<0x7000a3b6d0, 0x7000d790b0, 0x7000d7a840, 0x7001d550a8, 0x7000c17f50> as *const (), 2), // +0x68d940 sub_68d940
    Lifted::new(0x7000d79ab0, tpl_1310::<0x7000a3bcd0, 0x7000d79380, 0x7000d7a840, 0x7001d550a8, 0x7000c17f50> as *const (), 2), // +0x68dab0 sub_68dab0
    Lifted::new(0x7000d79c20, fn_7000d79c20 as *const (), 2), // +0x68dc20 sub_68dc20
    Lifted::new(0x7000d7a0e0, fn_7000d7a0e0 as *const (), 2), // +0x68e0e0 sub_68e0e0
    Lifted::new(0x7000d7a590, fn_7000d7a590 as *const (), 2), // +0x68e590 sub_68e590
    Lifted::new(0x7000d7a840, fn_7000d7a840 as *const (), 2), // +0x68e840 sub_68e840
    Lifted::new(0x7000d7ab40, tpl_1311::<0x7000d850d0, 0x7000d7af20, 0x7000d84dd0, 0x7000d85070, 0x7000d7acc5> as *const (), 0), // +0x68eb40 sub_68eb40
    Lifted::new(0x7000d7ad30, tpl_1311::<0x7000d85170, 0x7000d7afd0, 0x7000d84f20, 0x7000d85090, 0x7000d7aeb5> as *const (), 0), // +0x68ed30 sub_68ed30
    Lifted::new(0x7000d7af20, tpl_1312::<0x18, 0xffffffffffffffe8, 0x28, 0xffffffffffffffc8, 0xffffffffffffffd0, 0xffffffffffffffc0, 0xffffffffffffffd8, 0xffffffffffffffdc, 0xffffffffffffffe0> as *const (), 1), // +0x68ef20 sub_68ef20
    Lifted::new(0x7000d7afd0, tpl_1312::<0x18, 0xffffffffffffffe8, 0x28, 0xffffffffffffffc8, 0xffffffffffffffd0, 0xffffffffffffffc0, 0xffffffffffffffd8, 0xffffffffffffffdc, 0xffffffffffffffe0> as *const (), 1), // +0x68efd0 sub_68efd0
    Lifted::new(0x7000d7b080, fn_7000d7b080 as *const (), 1), // +0x68f080 sub_68f080
    Lifted::new(0x7000d7b0f0, fn_7000d7b0f0 as *const (), 1), // +0x68f0f0 sub_68f0f0
    Lifted::new(0x7000d7b140, fn_7000d7b140 as *const (), 1), // +0x68f140 sub_68f140
    Lifted::new(0x7000d7b1b0, fn_7000d7b1b0 as *const (), 0), // +0x68f1b0 sub_68f1b0
    Lifted::new(0x7000d7b410, fn_7000d7b410 as *const (), 0), // +0x68f410 sub_68f410
    Lifted::new(0x7000d7b7e0, fn_7000d7b7e0 as *const (), 1), // +0x68f7e0 sub_68f7e0
    Lifted::new(0x7000d7b830, fn_7000d7b830 as *const (), 0), // +0x68f830 sub_68f830
    Lifted::new(0x7000d7b890, fn_7000d7b890 as *const (), 0), // +0x68f890 sub_68f890
    Lifted::new(0x7000d7b990, fn_7000d7b990 as *const (), 0), // +0x68f990 sub_68f990
    Lifted::new(0x7000d7ba20, tpl_1313::<0x7000d7b990, 0x18, 0x1c, 0x20> as *const (), 0), // +0x68fa20 sub_68fa20
    Lifted::new(0x7000d7ba80, tpl_1313::<0x7000d7b990, 0x18, 0x1c, 0x20> as *const (), 0), // +0x68fa80 sub_68fa80
    Lifted::new(0x7000d7bae0, fn_7000d7bae0 as *const (), 1), // +0x68fae0 sub_68fae0
    Lifted::new(0x7000d7bbb0, fn_7000d7bbb0 as *const (), 3), // +0x68fbb0 sub_68fbb0
    Lifted::new(0x7000d7bc20, tpl_1314::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0x700177c312> as *const (), 0), // +0x68fc20 sub_68fc20
    Lifted::new(0x7000d7bca0, tpl_1314::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0x700177c312> as *const (), 0), // +0x68fca0 sub_68fca0
    Lifted::new(0x7000d7bda0, tpl_1315::<0x700073b280, 0x70017d8e88, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x68fda0 sub_68fda0
    Lifted::new(0x7000d7c0c0, tpl_1315::<0x700073b280, 0x70017d8e88, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6900c0 sub_6900c0
    Lifted::new(0x7000d7c3e0, fn_7000d7c3e0 as *const (), 2), // +0x6903e0 sub_6903e0
    Lifted::new(0x7000d7d7b0, fn_7000d7d7b0 as *const (), 2), // +0x6917b0 sub_6917b0
    Lifted::new(0x7000d7ebf0, fn_7000d7ebf0 as *const (), 1), // +0x692bf0 sub_692bf0
    Lifted::new(0x7000d7ecb0, fn_7000d7ecb0 as *const (), 1), // +0x692cb0 sub_692cb0
    Lifted::new(0x7000d7ecd0, fn_7000d7ecd0 as *const (), 0), // +0x692cd0 sub_692cd0
    Lifted::new(0x7000d7eda0, fn_7000d7eda0 as *const (), 0), // +0x692da0 sub_692da0
    Lifted::new(0x7000d7ee40, fn_7000d7ee40 as *const (), 0), // +0x692e40 sub_692e40
    Lifted::new(0x7000d7efc0, tpl_243::<0x7000d7efd0> as *const (), 0), // +0x692fc0 sub_692fc0
    Lifted::new(0x7000d7efd0, tpl_1316::<0x8, 0x10, 0x7001d54ce0, 0x7001d55060, 0x70017db8e8, 0x7001d54ef8, 0x7000d7f06b, 0x700177c0b0, 0x18, 0x7001d54ef0> as *const (), 0), // +0x692fd0 sub_692fd0
    Lifted::new(0x7000d7f100, tpl_243::<0x7000d7f110> as *const (), 0), // +0x693100 sub_693100
    Lifted::new(0x7000d7f110, tpl_1317::<0x10, 0x8, 0x7001d54ce0, 0x7001d55060, 0x70017db8e8, 0x7001d54ef8, 0x7000d7f177, 0x700177c0b0, 0x18, 0x7001d54ef0> as *const (), 0), // +0x693110 sub_693110
    Lifted::new(0x7000d7f200, fn_7000d7f200 as *const (), 0), // +0x693200 sub_693200
    Lifted::new(0x7000d7f340, fn_7000d7f340 as *const (), 2), // +0x693340 sub_693340
    Lifted::new(0x7000d7fbb0, fn_7000d7fbb0 as *const (), 2), // +0x693bb0 sub_693bb0
    Lifted::new(0x7000d7ffb0, fn_7000d7ffb0 as *const (), 2), // +0x693fb0 sub_693fb0
    Lifted::new(0x7000d80400, fn_7000d80400 as *const (), 0), // +0x694400 sub_694400
    Lifted::new(0x7000d804d0, fn_7000d804d0 as *const (), 0), // +0x6944d0 sub_6944d0
    Lifted::new(0x7000d80690, fn_7000d80690 as *const (), 0), // +0x694690 sub_694690
    Lifted::new(0x7000d80810, fn_7000d80810 as *const (), 0), // +0x694810 sub_694810
    Lifted::new(0x7000d808f0, fn_7000d808f0 as *const (), 0), // +0x6948f0 sub_6948f0
    Lifted::new(0x7000d80980, fn_7000d80980 as *const (), 2), // +0x694980 sub_694980
    Lifted::new(0x7000d80d90, fn_7000d80d90 as *const (), 2), // +0x694d90 sub_694d90
    Lifted::new(0x7000d811d0, fn_7000d811d0 as *const (), 2), // +0x6951d0 sub_6951d0
    Lifted::new(0x7000d82660, fn_7000d82660 as *const (), 2), // +0x696660 sub_696660
    Lifted::new(0x7000d83bd0, fn_7000d83bd0 as *const (), 2), // +0x697bd0 sub_697bd0
    Lifted::new(0x7000d83da0, fn_7000d83da0 as *const (), 0), // +0x697da0 sub_697da0
    Lifted::new(0x7000d84010, fn_7000d84010 as *const (), 0), // +0x698010 sub_698010
    Lifted::new(0x7000d84280, fn_7000d84280 as *const (), 0), // +0x698280 sub_698280
    Lifted::new(0x7000d84470, fn_7000d84470 as *const (), 0), // +0x698470 sub_698470
    Lifted::new(0x7000d84660, fn_7000d84660 as *const (), 0), // +0x698660 sub_698660
    Lifted::new(0x7000d84890, fn_7000d84890 as *const (), 0), // +0x698890 sub_698890
    Lifted::new(0x7000d84b00, fn_7000d84b00 as *const (), 1), // +0x698b00 sub_698b00
    Lifted::new(0x7000d84b70, fn_7000d84b70 as *const (), 1), // +0x698b70 sub_698b70
    Lifted::new(0x7000d84c40, fn_7000d84c40 as *const (), 1), // +0x698c40 sub_698c40
    Lifted::new(0x7000d84d40, fn_7000d84d40 as *const (), 1), // +0x698d40 sub_698d40
    Lifted::new(0x7000d84d80, fn_7000d84d80 as *const (), 1), // +0x698d80 sub_698d80
    Lifted::new(0x7000d84dd0, tpl_1318::<0x7000745eb0, 0x700177c312, 0x7001d54968, 0x7000d84edc> as *const (), 0), // +0x698dd0 sub_698dd0
    Lifted::new(0x7000d84f20, tpl_1318::<0x7000745eb0, 0x700177c312, 0x7001d54968, 0x7000d8502c> as *const (), 0), // +0x698f20 sub_698f20
    Lifted::new(0x7000d85070, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d85080> as *const (), 0), // +0x699070 sub_699070
    Lifted::new(0x7000d85090, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d850a0> as *const (), 0), // +0x699090 sub_699090
    Lifted::new(0x7000d850d0, tpl_932::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000d85128, 0x70007f0910, 0x7000d85143> as *const (), 0), // +0x6990d0 sub_6990d0
    Lifted::new(0x7000d85170, tpl_932::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000d851c8, 0x70007f0910, 0x7000d851e3> as *const (), 0), // +0x699170 sub_699170
    Lifted::new(0x7000d85390, fn_7000d85390 as *const (), 2), // +0x699390 sub_699390
    Lifted::new(0x7000d85710, fn_7000d85710 as *const (), 0), // +0x699710 sub_699710
    Lifted::new(0x7000d85a60, fn_7000d85a60 as *const (), 0), // +0x699a60 sub_699a60
    Lifted::new(0x7000d85e10, fn_7000d85e10 as *const (), 2), // +0x699e10 sub_699e10
    Lifted::new(0x7000d867a0, fn_7000d867a0 as *const (), 2), // +0x69a7a0 sub_69a7a0
    Lifted::new(0x7000d89f00, fn_7000d89f00 as *const (), 2), // +0x69df00 sub_69df00
    Lifted::new(0x7000d8b9d0, tpl_1319::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000d8bb9a, 0x7000d90690, 0x7000d8bba0, 0x70007f0910, 0x7000d8bba6> as *const (), 2), // +0x69f9d0 sub_69f9d0
    Lifted::new(0x7000d8bc20, tpl_14 as *const (), 1), // +0x69fc20 sub_69fc20
    Lifted::new(0x7000d8bc30, fn_7000d8bc30 as *const (), 1), // +0x69fc30 sub_69fc30
    Lifted::new(0x7000d8bc60, fn_7000d8bc60 as *const (), 0), // +0x69fc60 sub_69fc60
    Lifted::new(0x7000d8bcc0, tpl_70 as *const (), 1), // +0x69fcc0 sub_69fcc0
    Lifted::new(0x7000d8bcd0, tpl_14 as *const (), 1), // +0x69fcd0 sub_69fcd0
    Lifted::new(0x7000d8bce0, tpl_14 as *const (), 1), // +0x69fce0 sub_69fce0
    Lifted::new(0x7000d8bcf0, tpl_14 as *const (), 1), // +0x69fcf0 sub_69fcf0
    Lifted::new(0x7000d8bd00, tpl_1320::<0x38> as *const (), 1), // +0x69fd00 sub_69fd00
    Lifted::new(0x7000d8bd10, fn_7000d8bd10 as *const (), 1), // +0x69fd10 sub_69fd10
    Lifted::new(0x7000d8bd20, tpl_20 as *const (), 1), // +0x69fd20 sub_69fd20
    Lifted::new(0x7000d8bd30, tpl_20 as *const (), 1), // +0x69fd30 sub_69fd30
    Lifted::new(0x7000d8bd40, tpl_20 as *const (), 1), // +0x69fd40 sub_69fd40
    Lifted::new(0x7000d8bd50, tpl_187 as *const (), 1), // +0x69fd50 sub_69fd50
    Lifted::new(0x7000d8bd60, fn_7000d8bd60 as *const (), 1), // +0x69fd60 sub_69fd60
    Lifted::new(0x7000d8bd70, fn_7000d8bd70 as *const (), 0), // +0x69fd70 sub_69fd70
    Lifted::new(0x7000d8bf40, fn_7000d8bf40 as *const (), 0), // +0x69ff40 sub_69ff40
    Lifted::new(0x7000d8c040, fn_7000d8c040 as *const (), 0), // +0x6a0040 sub_6a0040
    Lifted::new(0x7000d8c1e0, fn_7000d8c1e0 as *const (), 0), // +0x6a01e0 sub_6a01e0
    Lifted::new(0x7000d8c640, fn_7000d8c640 as *const (), 0), // +0x6a0640 sub_6a0640
    Lifted::new(0x7000d8c7f0, fn_7000d8c7f0 as *const (), 0), // +0x6a07f0 sub_6a07f0
    Lifted::new(0x7000d8cb20, fn_7000d8cb20 as *const (), 0), // +0x6a0b20 sub_6a0b20
    Lifted::new(0x7000d8cda0, fn_7000d8cda0 as *const (), 0), // +0x6a0da0 sub_6a0da0
    Lifted::new(0x7000d8d300, fn_7000d8d300 as *const (), 0), // +0x6a1300 sub_6a1300
    Lifted::new(0x7000d8d480, tpl_1048 as *const (), 1), // +0x6a1480 sub_6a1480
    Lifted::new(0x7000d8d490, tpl_1321::<0x13> as *const (), 1), // +0x6a1490 sub_6a1490
    Lifted::new(0x7000d8d4b0, fn_7000d8d4b0 as *const (), 1), // +0x6a14b0 sub_6a14b0
    Lifted::new(0x7000d8d4e0, tpl_224::<0xf> as *const (), 1), // +0x6a14e0 sub_6a14e0
    Lifted::new(0x7000d8d520, fn_7000d8d520 as *const (), 0), // +0x6a1520 sub_6a1520
    Lifted::new(0x7000d8d6a0, fn_7000d8d6a0 as *const (), 1), // +0x6a16a0 sub_6a16a0
    Lifted::new(0x7000d8d6d0, fn_7000d8d6d0 as *const (), 2), // +0x6a16d0 sub_6a16d0
    Lifted::new(0x7000d8db90, fn_7000d8db90 as *const (), 2), // +0x6a1b90 sub_6a1b90
    Lifted::new(0x7000d8e230, fn_7000d8e230 as *const (), 0), // +0x6a2230 sub_6a2230
    Lifted::new(0x7000d8e260, fn_7000d8e260 as *const (), 1), // +0x6a2260 sub_6a2260
    Lifted::new(0x7000d8e280, tpl_362 as *const (), 1), // +0x6a2280 sub_6a2280
    Lifted::new(0x7000d8e290, fn_7000d8e290 as *const (), 1), // +0x6a2290 sub_6a2290
    Lifted::new(0x7000d8e2c0, fn_7000d8e2c0 as *const (), 2), // +0x6a22c0 sub_6a22c0
    Lifted::new(0x7000d8e470, fn_7000d8e470 as *const (), 1), // +0x6a2470 sub_6a2470
    Lifted::new(0x7000d8e490, fn_7000d8e490 as *const (), 0), // +0x6a2490 sub_6a2490
    Lifted::new(0x7000d8e930, fn_7000d8e930 as *const (), 2), // +0x6a2930 sub_6a2930
    Lifted::new(0x7000d8eb90, fn_7000d8eb90 as *const (), 2), // +0x6a2b90 sub_6a2b90
    Lifted::new(0x7000d8f5e0, fn_7000d8f5e0 as *const (), 2), // +0x6a35e0 sub_6a35e0
    Lifted::new(0x7000d8fad0, fn_7000d8fad0 as *const (), 0), // +0x6a3ad0 sub_6a3ad0
    Lifted::new(0x7000d90080, fn_7000d90080 as *const (), 0), // +0x6a4080 sub_6a4080
    Lifted::new(0x7000d901d0, fn_7000d901d0 as *const (), 0), // +0x6a41d0 sub_6a41d0
    Lifted::new(0x7000d90360, fn_7000d90360 as *const (), 0), // +0x6a4360 sub_6a4360
    Lifted::new(0x7000d904d0, fn_7000d904d0 as *const (), 0), // +0x6a44d0 sub_6a44d0
    Lifted::new(0x7000d905e0, fn_7000d905e0 as *const (), 3), // +0x6a45e0 sub_6a45e0
    Lifted::new(0x7000d90690, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000d906a0> as *const (), 0), // +0x6a4690 sub_6a4690
    Lifted::new(0x7000d906b0, tpl_187 as *const (), 1), // +0x6a46b0 sub_6a46b0
    Lifted::new(0x7000d906c0, tpl_334::<0x7> as *const (), 1), // +0x6a46c0 sub_6a46c0
    Lifted::new(0x7000d906f0, tpl_191::<0x8> as *const (), 1), // +0x6a46f0 sub_6a46f0
    Lifted::new(0x7000d90700, fn_7000d90700 as *const (), 3), // +0x6a4700 sub_6a4700
    Lifted::new(0x7000d90720, tpl_333::<0x17> as *const (), 1), // +0x6a4720 sub_6a4720
    Lifted::new(0x7000d90740, fn_7000d90740 as *const (), 1), // +0x6a4740 sub_6a4740
    Lifted::new(0x7000d90750, tpl_334::<0xb> as *const (), 1), // +0x6a4750 sub_6a4750
    Lifted::new(0x7000d90780, fn_7000d90780 as *const (), 1), // +0x6a4780 sub_6a4780
    Lifted::new(0x7000d90790, fn_7000d90790 as *const (), 1), // +0x6a4790 sub_6a4790
    Lifted::new(0x7000d907a0, fn_7000d907a0 as *const (), 0), // +0x6a47a0 sub_6a47a0
    Lifted::new(0x7000d90820, fn_7000d90820 as *const (), 0), // +0x6a4820 sub_6a4820
    Lifted::new(0x7000d908c0, fn_7000d908c0 as *const (), 0), // +0x6a48c0 sub_6a48c0
    Lifted::new(0x7000d90920, fn_7000d90920 as *const (), 0), // +0x6a4920 sub_6a4920
    Lifted::new(0x7000d90a50, fn_7000d90a50 as *const (), 0), // +0x6a4a50 sub_6a4a50
    Lifted::new(0x7000d90ad0, fn_7000d90ad0 as *const (), 0), // +0x6a4ad0 sub_6a4ad0
    Lifted::new(0x7000d90bf0, tpl_171::<0x20> as *const (), 1), // +0x6a4bf0 sub_6a4bf0
    Lifted::new(0x7000d90c00, tpl_168::<0x20> as *const (), 1), // +0x6a4c00 sub_6a4c00
    Lifted::new(0x7000d90c10, fn_7000d90c10 as *const (), 2), // +0x6a4c10 sub_6a4c10
    Lifted::new(0x7000d910f0, fn_7000d910f0 as *const (), 0), // +0x6a50f0 sub_6a50f0
    Lifted::new(0x7000d91150, fn_7000d91150 as *const (), 0), // +0x6a5150 sub_6a5150
    Lifted::new(0x7000d91180, fn_7000d91180 as *const (), 0), // +0x6a5180 sub_6a5180
    Lifted::new(0x7000d91220, fn_7000d91220 as *const (), 0), // +0x6a5220 sub_6a5220
    Lifted::new(0x7000d91280, fn_7000d91280 as *const (), 0), // +0x6a5280 sub_6a5280
    Lifted::new(0x7000d912d0, fn_7000d912d0 as *const (), 0), // +0x6a52d0 sub_6a52d0
    Lifted::new(0x7000d912f0, fn_7000d912f0 as *const (), 0), // +0x6a52f0 sub_6a52f0
    Lifted::new(0x7000d91350, fn_7000d91350 as *const (), 0), // +0x6a5350 sub_6a5350
    Lifted::new(0x7000d913a0, fn_7000d913a0 as *const (), 0), // +0x6a53a0 sub_6a53a0
    Lifted::new(0x7000d914a0, fn_7000d914a0 as *const (), 0), // +0x6a54a0 sub_6a54a0
    Lifted::new(0x7000d929f0, fn_7000d929f0 as *const (), 0), // +0x6a69f0 sub_6a69f0
    Lifted::new(0x7000d95130, fn_7000d95130 as *const (), 0), // +0x6a9130 sub_6a9130
    Lifted::new(0x7000d951a0, fn_7000d951a0 as *const (), 0), // +0x6a91a0 sub_6a91a0
    Lifted::new(0x7000d95240, fn_7000d95240 as *const (), 0), // +0x6a9240 sub_6a9240
    Lifted::new(0x7000d952a0, tpl_17::<0x7000d9bd20> as *const (), 0), // +0x6a92a0 sub_6a92a0
    Lifted::new(0x7000d952b0, tpl_17::<0x7000da1500> as *const (), 0), // +0x6a92b0 sub_6a92b0
    Lifted::new(0x7000d952c0, tpl_17::<0x7000da1680> as *const (), 0), // +0x6a92c0 sub_6a92c0
    Lifted::new(0x7000d952d0, tpl_17::<0x7000da3610> as *const (), 0), // +0x6a92d0 sub_6a92d0
    Lifted::new(0x7000d952e0, tpl_17::<0x7000da3850> as *const (), 0), // +0x6a92e0 sub_6a92e0
    Lifted::new(0x7000d952f0, fn_7000d952f0 as *const (), 0), // +0x6a92f0 sub_6a92f0
    Lifted::new(0x7000d953a0, fn_7000d953a0 as *const (), 0), // +0x6a93a0 sub_6a93a0
    Lifted::new(0x7000d95420, fn_7000d95420 as *const (), 0), // +0x6a9420 sub_6a9420
    Lifted::new(0x7000d95530, fn_7000d95530 as *const (), 2), // +0x6a9530 sub_6a9530
    Lifted::new(0x7000d955c0, fn_7000d955c0 as *const (), 2), // +0x6a95c0 sub_6a95c0
    Lifted::new(0x7000d95610, tpl_13::<0x7000d960b0> as *const (), 0), // +0x6a9610 sub_6a9610
    Lifted::new(0x7000d95630, fn_7000d95630 as *const (), 0), // +0x6a9630 sub_6a9630
    Lifted::new(0x7000d95660, fn_7000d95660 as *const (), 0), // +0x6a9660 sub_6a9660
    Lifted::new(0x7000d95890, fn_7000d95890 as *const (), 0), // +0x6a9890 sub_6a9890
    Lifted::new(0x7000d95960, fn_7000d95960 as *const (), 0), // +0x6a9960 sub_6a9960
    Lifted::new(0x7000d95a80, fn_7000d95a80 as *const (), 0), // +0x6a9a80 sub_6a9a80
    Lifted::new(0x7000d95b20, fn_7000d95b20 as *const (), 0), // +0x6a9b20 sub_6a9b20
    Lifted::new(0x7000d95c20, fn_7000d95c20 as *const (), 0), // +0x6a9c20 sub_6a9c20
    Lifted::new(0x7000d95e00, fn_7000d95e00 as *const (), 0), // +0x6a9e00 sub_6a9e00
    Lifted::new(0x7000d95ed0, fn_7000d95ed0 as *const (), 0), // +0x6a9ed0 sub_6a9ed0
    Lifted::new(0x7000d960b0, fn_7000d960b0 as *const (), 2), // +0x6aa0b0 sub_6aa0b0
    Lifted::new(0x7000d96240, fn_7000d96240 as *const (), 2), // +0x6aa240 sub_6aa240
    Lifted::new(0x7000d96430, fn_7000d96430 as *const (), 0), // +0x6aa430 sub_6aa430
    Lifted::new(0x7000d964a0, fn_7000d964a0 as *const (), 0), // +0x6aa4a0 sub_6aa4a0
    Lifted::new(0x7000d96570, fn_7000d96570 as *const (), 0), // +0x6aa570 sub_6aa570
    Lifted::new(0x7000d965c0, tpl_1322::<0x18, 0x10, 0x700180581c, 0x700177c0c0, 0x1, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000d9667e> as *const (), 0), // +0x6aa5c0 sub_6aa5c0
    Lifted::new(0x7000d966b0, tpl_1322::<0x18, 0x10, 0x700183e5fc, 0x700177c0c0, 0x1, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000d9676e> as *const (), 0), // +0x6aa6b0 sub_6aa6b0
    Lifted::new(0x7000d967a0, fn_7000d967a0 as *const (), 0), // +0x6aa7a0 sub_6aa7a0
    Lifted::new(0x7000d968c0, fn_7000d968c0 as *const (), 0), // +0x6aa8c0 sub_6aa8c0
    Lifted::new(0x7000d96da0, fn_7000d96da0 as *const (), 0), // +0x6aada0 sub_6aada0
    Lifted::new(0x7000d96dc0, fn_7000d96dc0 as *const (), 1), // +0x6aadc0 sub_6aadc0
    Lifted::new(0x7000d96de0, fn_7000d96de0 as *const (), 0), // +0x6aade0 sub_6aade0
    Lifted::new(0x7000d96e50, tpl_55::<0x700177c0d0> as *const (), 0), // +0x6aae50 sub_6aae50
    Lifted::new(0x7000d96e90, tpl_1323::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000d97097, 0x7000da6190, 0x7000d9709d, 0x70007f0910, 0x7000d970a3> as *const (), 2), // +0x6aae90 sub_6aae90
    Lifted::new(0x7000d97130, tpl_1319::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000d972fa, 0x7000da6170, 0x7000d97300, 0x70007f0910, 0x7000d97306> as *const (), 2), // +0x6ab130 sub_6ab130
    Lifted::new(0x7000d97380, fn_7000d97380 as *const (), 0), // +0x6ab380 sub_6ab380
    Lifted::new(0x7000d97500, tpl_922::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000da5ac0, 0x70017d37c0, 0x70017790e0, 0x7000d977e1> as *const (), 2), // +0x6ab500 sub_6ab500
    Lifted::new(0x7000d978a0, fn_7000d978a0 as *const (), 2), // +0x6ab8a0 sub_6ab8a0
    Lifted::new(0x7000d978f0, fn_7000d978f0 as *const (), 0), // +0x6ab8f0 sub_6ab8f0
    Lifted::new(0x7000d979b0, fn_7000d979b0 as *const (), 2), // +0x6ab9b0 sub_6ab9b0
    Lifted::new(0x7000d97a60, fn_7000d97a60 as *const (), 0), // +0x6aba60 sub_6aba60
    Lifted::new(0x7000d97be0, fn_7000d97be0 as *const (), 0), // +0x6abbe0 sub_6abbe0
    Lifted::new(0x7000d97c30, fn_7000d97c30 as *const (), 0), // +0x6abc30 sub_6abc30
    Lifted::new(0x7000d97ca0, fn_7000d97ca0 as *const (), 0), // +0x6abca0 sub_6abca0
    Lifted::new(0x7000d97d60, fn_7000d97d60 as *const (), 0), // +0x6abd60 sub_6abd60
    Lifted::new(0x7000d97e80, fn_7000d97e80 as *const (), 0), // +0x6abe80 sub_6abe80
    Lifted::new(0x7000d97ee0, fn_7000d97ee0 as *const (), 1), // +0x6abee0 sub_6abee0
    Lifted::new(0x7000d97f10, fn_7000d97f10 as *const (), 0), // +0x6abf10 sub_6abf10
    Lifted::new(0x7000d97f90, fn_7000d97f90 as *const (), 0), // +0x6abf90 sub_6abf90
    Lifted::new(0x7000d98040, fn_7000d98040 as *const (), 0), // +0x6ac040 sub_6ac040
    Lifted::new(0x7000d98180, fn_7000d98180 as *const (), 0), // +0x6ac180 sub_6ac180
    Lifted::new(0x7000d98760, fn_7000d98760 as *const (), 0), // +0x6ac760 sub_6ac760
    Lifted::new(0x7000d98780, fn_7000d98780 as *const (), 0), // +0x6ac780 sub_6ac780
    Lifted::new(0x7000d987d0, fn_7000d987d0 as *const (), 0), // +0x6ac7d0 sub_6ac7d0
    Lifted::new(0x7000d988e0, fn_7000d988e0 as *const (), 0), // +0x6ac8e0 sub_6ac8e0
    Lifted::new(0x7000d98940, fn_7000d98940 as *const (), 0), // +0x6ac940 sub_6ac940
    Lifted::new(0x7000d98990, fn_7000d98990 as *const (), 0), // +0x6ac990 sub_6ac990
    Lifted::new(0x7000d98a80, fn_7000d98a80 as *const (), 0), // +0x6aca80 sub_6aca80
    Lifted::new(0x7000d98b80, fn_7000d98b80 as *const (), 0), // +0x6acb80 sub_6acb80
    Lifted::new(0x7000d98be0, fn_7000d98be0 as *const (), 0), // +0x6acbe0 sub_6acbe0
    Lifted::new(0x7000d98c50, fn_7000d98c50 as *const (), 0), // +0x6acc50 sub_6acc50
    Lifted::new(0x7000d98cb0, fn_7000d98cb0 as *const (), 0), // +0x6accb0 sub_6accb0
    Lifted::new(0x7000d98de0, fn_7000d98de0 as *const (), 0), // +0x6acde0 sub_6acde0
    Lifted::new(0x7000d98e60, fn_7000d98e60 as *const (), 0), // +0x6ace60 sub_6ace60
    Lifted::new(0x7000d98eb0, tpl_136::<0x7001d54cb8, 0x700177c312> as *const (), 0), // +0x6aceb0 sub_6aceb0
    Lifted::new(0x7000d98ef0, fn_7000d98ef0 as *const (), 0), // +0x6acef0 sub_6acef0
    Lifted::new(0x7000d98f70, fn_7000d98f70 as *const (), 0), // +0x6acf70 sub_6acf70
    Lifted::new(0x7000d99000, fn_7000d99000 as *const (), 2), // +0x6ad000 sub_6ad000
    Lifted::new(0x7000d99090, fn_7000d99090 as *const (), 0), // +0x6ad090 sub_6ad090
    Lifted::new(0x7000d991c0, fn_7000d991c0 as *const (), 0), // +0x6ad1c0 sub_6ad1c0
    Lifted::new(0x7000d992d0, fn_7000d992d0 as *const (), 2), // +0x6ad2d0 sub_6ad2d0
    Lifted::new(0x7000d99420, fn_7000d99420 as *const (), 0), // +0x6ad420 sub_6ad420
    Lifted::new(0x7000d99670, fn_7000d99670 as *const (), 2), // +0x6ad670 sub_6ad670
    Lifted::new(0x7000d99920, fn_7000d99920 as *const (), 0), // +0x6ad920 sub_6ad920
    Lifted::new(0x7000d99a80, fn_7000d99a80 as *const (), 0), // +0x6ada80 sub_6ada80
    Lifted::new(0x7000d99bb0, fn_7000d99bb0 as *const (), 0), // +0x6adbb0 sub_6adbb0
    Lifted::new(0x7000d99d80, tpl_15 as *const (), 1), // +0x6add80 sub_6add80
    Lifted::new(0x7000d99d90, tpl_1324::<0x70017d2c0c, 0x7000d99db0> as *const (), 0), // +0x6add90 sub_6add90
    Lifted::new(0x7000d99db0, fn_7000d99db0 as *const (), 0), // +0x6addb0 sub_6addb0
    Lifted::new(0x7000d99fc0, fn_7000d99fc0 as *const (), 0), // +0x6adfc0 sub_6adfc0
    Lifted::new(0x7000d9a1b0, fn_7000d9a1b0 as *const (), 0), // +0x6ae1b0 sub_6ae1b0
    Lifted::new(0x7000d9a280, fn_7000d9a280 as *const (), 0), // +0x6ae280 sub_6ae280
    Lifted::new(0x7000d9a300, fn_7000d9a300 as *const (), 0), // +0x6ae300 sub_6ae300
    Lifted::new(0x7000d9a410, fn_7000d9a410 as *const (), 0), // +0x6ae410 sub_6ae410
    Lifted::new(0x7000d9a700, tpl_15 as *const (), 1), // +0x6ae700 sub_6ae700
    Lifted::new(0x7000d9a710, fn_7000d9a710 as *const (), 0), // +0x6ae710 sub_6ae710
    Lifted::new(0x7000d9a870, fn_7000d9a870 as *const (), 0), // +0x6ae870 sub_6ae870
    Lifted::new(0x7000d9aa20, fn_7000d9aa20 as *const (), 0), // +0x6aea20 sub_6aea20
    Lifted::new(0x7000d9abe0, fn_7000d9abe0 as *const (), 0), // +0x6aebe0 sub_6aebe0
    Lifted::new(0x7000d9ae50, fn_7000d9ae50 as *const (), 2), // +0x6aee50 sub_6aee50
    Lifted::new(0x7000d9b040, fn_7000d9b040 as *const (), 0), // +0x6af040 sub_6af040
    Lifted::new(0x7000d9b1a0, fn_7000d9b1a0 as *const (), 0), // +0x6af1a0 sub_6af1a0
    Lifted::new(0x7000d9b300, fn_7000d9b300 as *const (), 0), // +0x6af300 sub_6af300
    Lifted::new(0x7000d9b4e0, fn_7000d9b4e0 as *const (), 0), // +0x6af4e0 sub_6af4e0
    Lifted::new(0x7000d9b770, tpl_15 as *const (), 1), // +0x6af770 sub_6af770
    Lifted::new(0x7000d9b780, fn_7000d9b780 as *const (), 0), // +0x6af780 sub_6af780
    Lifted::new(0x7000d9b840, tpl_15 as *const (), 1), // +0x6af840 sub_6af840
    Lifted::new(0x7000d9b850, fn_7000d9b850 as *const (), 0), // +0x6af850 sub_6af850
    Lifted::new(0x7000d9b900, tpl_15 as *const (), 1), // +0x6af900 sub_6af900
    Lifted::new(0x7000d9b910, tpl_15 as *const (), 1), // +0x6af910 sub_6af910
    Lifted::new(0x7000d9b920, fn_7000d9b920 as *const (), 0), // +0x6af920 sub_6af920
    Lifted::new(0x7000d9bab0, tpl_15 as *const (), 1), // +0x6afab0 sub_6afab0
    Lifted::new(0x7000d9bac0, fn_7000d9bac0 as *const (), 2), // +0x6afac0 sub_6afac0
    Lifted::new(0x7000d9bcb0, fn_7000d9bcb0 as *const (), 2), // +0x6afcb0 sub_6afcb0
    Lifted::new(0x7000d9bd20, fn_7000d9bd20 as *const (), 0), // +0x6afd20 sub_6afd20
    Lifted::new(0x7000d9bd90, fn_7000d9bd90 as *const (), 0), // +0x6afd90 sub_6afd90
    Lifted::new(0x7000d9bf20, fn_7000d9bf20 as *const (), 0), // +0x6aff20 sub_6aff20
    Lifted::new(0x7000d9c0d0, fn_7000d9c0d0 as *const (), 0), // +0x6b00d0 sub_6b00d0
    Lifted::new(0x7000d9c1e0, fn_7000d9c1e0 as *const (), 0), // +0x6b01e0 sub_6b01e0
    Lifted::new(0x7000d9c330, tpl_15 as *const (), 1), // +0x6b0330 sub_6b0330
    Lifted::new(0x7000d9c340, fn_7000d9c340 as *const (), 0), // +0x6b0340 sub_6b0340
    Lifted::new(0x7000d9c380, fn_7000d9c380 as *const (), 0), // +0x6b0380 sub_6b0380
    Lifted::new(0x7000d9c3b0, fn_7000d9c3b0 as *const (), 0), // +0x6b03b0 sub_6b03b0
    Lifted::new(0x7000d9c4f0, fn_7000d9c4f0 as *const (), 0), // +0x6b04f0 sub_6b04f0
    Lifted::new(0x7000d9c7b0, fn_7000d9c7b0 as *const (), 0), // +0x6b07b0 sub_6b07b0
    Lifted::new(0x7000d9c8d0, fn_7000d9c8d0 as *const (), 0), // +0x6b08d0 sub_6b08d0
    Lifted::new(0x7000d9c9f0, fn_7000d9c9f0 as *const (), 0), // +0x6b09f0 sub_6b09f0
    Lifted::new(0x7000d9cb10, fn_7000d9cb10 as *const (), 2), // +0x6b0b10 sub_6b0b10
    Lifted::new(0x7000d9cd60, fn_7000d9cd60 as *const (), 2), // +0x6b0d60 sub_6b0d60
    Lifted::new(0x7000d9cdc0, fn_7000d9cdc0 as *const (), 0), // +0x6b0dc0 sub_6b0dc0
    Lifted::new(0x7000d9d300, fn_7000d9d300 as *const (), 2), // +0x6b1300 sub_6b1300
    Lifted::new(0x7000d9d630, fn_7000d9d630 as *const (), 0), // +0x6b1630 sub_6b1630
    Lifted::new(0x7000d9de20, fn_7000d9de20 as *const (), 2), // +0x6b1e20 sub_6b1e20
    Lifted::new(0x7000d9dfc0, fn_7000d9dfc0 as *const (), 2), // +0x6b1fc0 sub_6b1fc0
    Lifted::new(0x7000d9e140, fn_7000d9e140 as *const (), 0), // +0x6b2140 sub_6b2140
    Lifted::new(0x7000d9e280, tpl_15 as *const (), 1), // +0x6b2280 sub_6b2280
    Lifted::new(0x7000d9e290, tpl_1324::<0x70017d90d8, 0x7000d99db0> as *const (), 0), // +0x6b2290 sub_6b2290
    Lifted::new(0x7000d9e2b0, fn_7000d9e2b0 as *const (), 0), // +0x6b22b0 sub_6b22b0
    Lifted::new(0x7000d9e3b0, fn_7000d9e3b0 as *const (), 1), // +0x6b23b0 sub_6b23b0
    Lifted::new(0x7000d9e440, fn_7000d9e440 as *const (), 2), // +0x6b2440 sub_6b2440
    Lifted::new(0x7000d9e810, fn_7000d9e810 as *const (), 0), // +0x6b2810 sub_6b2810
    Lifted::new(0x7000d9e8a0, fn_7000d9e8a0 as *const (), 0), // +0x6b28a0 sub_6b28a0
    Lifted::new(0x7000d9e9b0, fn_7000d9e9b0 as *const (), 0), // +0x6b29b0 sub_6b29b0
    Lifted::new(0x7000d9ec70, fn_7000d9ec70 as *const (), 2), // +0x6b2c70 sub_6b2c70
    Lifted::new(0x7000d9ef50, fn_7000d9ef50 as *const (), 2), // +0x6b2f50 sub_6b2f50
    Lifted::new(0x7000d9f3f0, fn_7000d9f3f0 as *const (), 0), // +0x6b33f0 sub_6b33f0
    Lifted::new(0x7000d9f430, fn_7000d9f430 as *const (), 0), // +0x6b3430 sub_6b3430
    Lifted::new(0x7000d9f6f0, fn_7000d9f6f0 as *const (), 0), // +0x6b36f0 sub_6b36f0
    Lifted::new(0x7000d9f810, tpl_1163::<0x7000d9f6f0> as *const (), 0), // +0x6b3810 sub_6b3810
    Lifted::new(0x7000d9f820, fn_7000d9f820 as *const (), 0), // +0x6b3820 sub_6b3820
    Lifted::new(0x7000d9fcc0, tpl_1163::<0x7000d9f820> as *const (), 0), // +0x6b3cc0 sub_6b3cc0
    Lifted::new(0x7000d9fcd0, fn_7000d9fcd0 as *const (), 0), // +0x6b3cd0 sub_6b3cd0
    Lifted::new(0x7000d9fd80, fn_7000d9fd80 as *const (), 0), // +0x6b3d80 sub_6b3d80
    Lifted::new(0x7000da0360, fn_7000da0360 as *const (), 0), // +0x6b4360 sub_6b4360
    Lifted::new(0x7000da03e0, fn_7000da03e0 as *const (), 2), // +0x6b43e0 sub_6b43e0
    Lifted::new(0x7000da08e0, fn_7000da08e0 as *const (), 2), // +0x6b48e0 sub_6b48e0
    Lifted::new(0x7000da0ad0, fn_7000da0ad0 as *const (), 0), // +0x6b4ad0 sub_6b4ad0
    Lifted::new(0x7000da0bb0, fn_7000da0bb0 as *const (), 0), // +0x6b4bb0 sub_6b4bb0
    Lifted::new(0x7000da0de0, fn_7000da0de0 as *const (), 2), // +0x6b4de0 sub_6b4de0
    Lifted::new(0x7000da1480, tpl_1325::<0x70017ff898, 0x7000d99bb0> as *const (), 0), // +0x6b5480 sub_6b5480
    Lifted::new(0x7000da14a0, tpl_1325::<0x70017d3d40, 0x7000d99bb0> as *const (), 0), // +0x6b54a0 sub_6b54a0
    Lifted::new(0x7000da14c0, fn_7000da14c0 as *const (), 0), // +0x6b54c0 sub_6b54c0
    Lifted::new(0x7000da1500, fn_7000da1500 as *const (), 2), // +0x6b5500 sub_6b5500
    Lifted::new(0x7000da1680, fn_7000da1680 as *const (), 2), // +0x6b5680 sub_6b5680
    Lifted::new(0x7000da1a80, fn_7000da1a80 as *const (), 2), // +0x6b5a80 sub_6b5a80
    Lifted::new(0x7000da1e50, fn_7000da1e50 as *const (), 0), // +0x6b5e50 sub_6b5e50
    Lifted::new(0x7000da1f90, tpl_15 as *const (), 1), // +0x6b5f90 sub_6b5f90
    Lifted::new(0x7000da1fa0, tpl_15 as *const (), 1), // +0x6b5fa0 sub_6b5fa0
    Lifted::new(0x7000da1fb0, fn_7000da1fb0 as *const (), 0), // +0x6b5fb0 sub_6b5fb0
    Lifted::new(0x7000da2100, fn_7000da2100 as *const (), 2), // +0x6b6100 sub_6b6100
    Lifted::new(0x7000da22d0, fn_7000da22d0 as *const (), 2), // +0x6b62d0 sub_6b62d0
    Lifted::new(0x7000da2810, fn_7000da2810 as *const (), 0), // +0x6b6810 sub_6b6810
    Lifted::new(0x7000da2910, fn_7000da2910 as *const (), 2), // +0x6b6910 sub_6b6910
    Lifted::new(0x7000da2c20, fn_7000da2c20 as *const (), 0), // +0x6b6c20 sub_6b6c20
    Lifted::new(0x7000da2d70, fn_7000da2d70 as *const (), 0), // +0x6b6d70 sub_6b6d70
    Lifted::new(0x7000da2eb0, tpl_488::<0x8, 0xc800, 0x7000da2ed0> as *const (), 0), // +0x6b6eb0 sub_6b6eb0
    Lifted::new(0x7000da2ed0, fn_7000da2ed0 as *const (), 0), // +0x6b6ed0 sub_6b6ed0
    Lifted::new(0x7000da3070, fn_7000da3070 as *const (), 2), // +0x6b7070 sub_6b7070
    Lifted::new(0x7000da3220, fn_7000da3220 as *const (), 0), // +0x6b7220 sub_6b7220
    Lifted::new(0x7000da3300, fn_7000da3300 as *const (), 0), // +0x6b7300 sub_6b7300
    Lifted::new(0x7000da3350, fn_7000da3350 as *const (), 0), // +0x6b7350 sub_6b7350
    Lifted::new(0x7000da3460, fn_7000da3460 as *const (), 2), // +0x6b7460 sub_6b7460
    Lifted::new(0x7000da3610, fn_7000da3610 as *const (), 0), // +0x6b7610 sub_6b7610
    Lifted::new(0x7000da3850, fn_7000da3850 as *const (), 0), // +0x6b7850 sub_6b7850
    Lifted::new(0x7000da3a20, fn_7000da3a20 as *const (), 0), // +0x6b7a20 sub_6b7a20
    Lifted::new(0x7000da3be0, fn_7000da3be0 as *const (), 2), // +0x6b7be0 sub_6b7be0
    Lifted::new(0x7000da44e0, tpl_15 as *const (), 1), // +0x6b84e0 sub_6b84e0
    Lifted::new(0x7000da44f0, tpl_1324::<0x70017d9160, 0x7000d99db0> as *const (), 0), // +0x6b84f0 sub_6b84f0
    Lifted::new(0x7000da4510, tpl_15 as *const (), 1), // +0x6b8510 sub_6b8510
    Lifted::new(0x7000da4520, fn_7000da4520 as *const (), 0), // +0x6b8520 sub_6b8520
    Lifted::new(0x7000da4580, fn_7000da4580 as *const (), 0), // +0x6b8580 sub_6b8580
    Lifted::new(0x7000da4660, fn_7000da4660 as *const (), 0), // +0x6b8660 sub_6b8660
    Lifted::new(0x7000da47e0, fn_7000da47e0 as *const (), 0), // +0x6b87e0 sub_6b87e0
    Lifted::new(0x7000da49d0, fn_7000da49d0 as *const (), 2), // +0x6b89d0 sub_6b89d0
    Lifted::new(0x7000da4a30, fn_7000da4a30 as *const (), 0), // +0x6b8a30 sub_6b8a30
    Lifted::new(0x7000da4b60, fn_7000da4b60 as *const (), 0), // +0x6b8b60 sub_6b8b60
    Lifted::new(0x7000da4ba0, fn_7000da4ba0 as *const (), 0), // +0x6b8ba0 sub_6b8ba0
    Lifted::new(0x7000da4bf0, tpl_606::<0x7001c72309, 0x7000da53d0> as *const (), 0), // +0x6b8bf0 sub_6b8bf0
    Lifted::new(0x7000da4c10, fn_7000da4c10 as *const (), 0), // +0x6b8c10 sub_6b8c10
    Lifted::new(0x7000da4d90, fn_7000da4d90 as *const (), 0), // +0x6b8d90 sub_6b8d90
    Lifted::new(0x7000da4ed0, fn_7000da4ed0 as *const (), 0), // +0x6b8ed0 sub_6b8ed0
    Lifted::new(0x7000da5220, fn_7000da5220 as *const (), 0), // +0x6b9220 sub_6b9220
    Lifted::new(0x7000da5260, fn_7000da5260 as *const (), 0), // +0x6b9260 sub_6b9260
    Lifted::new(0x7000da53d0, fn_7000da53d0 as *const (), 0), // +0x6b93d0 sub_6b93d0
    Lifted::new(0x7000da54c0, fn_7000da54c0 as *const (), 0), // +0x6b94c0 sub_6b94c0
    Lifted::new(0x7000da5530, tpl_15 as *const (), 1), // +0x6b9530 sub_6b9530
    Lifted::new(0x7000da5540, tpl_15 as *const (), 1), // +0x6b9540 sub_6b9540
    Lifted::new(0x7000da5550, fn_7000da5550 as *const (), 0), // +0x6b9550 sub_6b9550
    Lifted::new(0x7000da5640, fn_7000da5640 as *const (), 0), // +0x6b9640 sub_6b9640
    Lifted::new(0x7000da5780, fn_7000da5780 as *const (), 0), // +0x6b9780 sub_6b9780
    Lifted::new(0x7000da58c0, fn_7000da58c0 as *const (), 0), // +0x6b98c0 sub_6b98c0
    Lifted::new(0x7000da59e0, fn_7000da59e0 as *const (), 0), // +0x6b99e0 sub_6b99e0
    Lifted::new(0x7000da5ac0, tpl_285::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000da5bf7, 0x7000bb4350, 0x70007f0910, 0x7000da5d86, 0x70017d37a0, 0x70017790e0, 0x7000da5d93> as *const (), 0), // +0x6b9ac0 sub_6b9ac0
    Lifted::new(0x7000da5e50, tpl_287 as *const (), 1), // +0x6b9e50 sub_6b9e50
    Lifted::new(0x7000da6170, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000da6180> as *const (), 0), // +0x6ba170 sub_6ba170
    Lifted::new(0x7000da6190, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000da61a0> as *const (), 0), // +0x6ba190 sub_6ba190
    Lifted::new(0x7000da61b0, tpl_1326::<0x28, 0x7000d96de0> as *const (), 0), // +0x6ba1b0 sub_6ba1b0
    Lifted::new(0x7000da61e0, tpl_44 as *const (), 1), // +0x6ba1e0 sub_6ba1e0
    Lifted::new(0x7000da61f0, fn_7000da61f0 as *const (), 1), // +0x6ba1f0 sub_6ba1f0
    Lifted::new(0x7000da6220, fn_7000da6220 as *const (), 1), // +0x6ba220 sub_6ba220
    Lifted::new(0x7000da6240, fn_7000da6240 as *const (), 1), // +0x6ba240 sub_6ba240
    Lifted::new(0x7000da6260, tpl_55::<0x700177c0d0> as *const (), 0), // +0x6ba260 sub_6ba260
    Lifted::new(0x7000da62a0, tpl_898::<0x700177c0e0> as *const (), 0), // +0x6ba2a0 sub_6ba2a0
    Lifted::new(0x7000da62e0, tpl_392::<0x700177c0e0> as *const (), 0), // +0x6ba2e0 sub_6ba2e0
    Lifted::new(0x7000da6310, fn_7000da6310 as *const (), 0), // +0x6ba310 sub_6ba310
    Lifted::new(0x7000da6440, tpl_55::<0x700177c0d0> as *const (), 0), // +0x6ba440 sub_6ba440
    Lifted::new(0x7000da6480, tpl_1327::<0x10, 0x8, 0xc, 0x700177c312> as *const (), 0), // +0x6ba480 sub_6ba480
    Lifted::new(0x7000da6520, fn_7000da6520 as *const (), 2), // +0x6ba520 sub_6ba520
    Lifted::new(0x7000da6740, tpl_172::<0x28> as *const (), 1), // +0x6ba740 sub_6ba740
    Lifted::new(0x7000da6750, tpl_977::<0x28, 0x60> as *const (), 0), // +0x6ba750 sub_6ba750
    Lifted::new(0x7000da6770, fn_7000da6770 as *const (), 2), // +0x6ba770 sub_6ba770
    Lifted::new(0x7000da6a20, fn_7000da6a20 as *const (), 0), // +0x6baa20 sub_6baa20
    Lifted::new(0x7000da6ba0, tpl_900::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000da6d68, 0x70007f0910, 0x7000da6d6e, 0x7000da6df0, 0x7000da6d77> as *const (), 0), // +0x6baba0 sub_6baba0
    Lifted::new(0x7000da6df0, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000da6e00> as *const (), 0), // +0x6badf0 sub_6badf0
    Lifted::new(0x7000da6e10, tpl_1079::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000da700b, 0x7000da9c60, 0x7000da7011, 0x70007f0910, 0x7000da7017> as *const (), 0), // +0x6bae10 sub_6bae10
    Lifted::new(0x7000da70a0, tpl_996::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000da733f, 0x7000da9c80, 0x7000da7345, 0x70007f0910, 0x7000da734b> as *const (), 2), // +0x6bb0a0 sub_6bb0a0
    Lifted::new(0x7000da7400, fn_7000da7400 as *const (), 3), // +0x6bb400 sub_6bb400
    Lifted::new(0x7000da76c0, fn_7000da76c0 as *const (), 3), // +0x6bb6c0 sub_6bb6c0
    Lifted::new(0x7000da77e0, fn_7000da77e0 as *const (), 2), // +0x6bb7e0 sub_6bb7e0
    Lifted::new(0x7000da7cd0, fn_7000da7cd0 as *const (), 3), // +0x6bbcd0 sub_6bbcd0
    Lifted::new(0x7000da7f00, fn_7000da7f00 as *const (), 2), // +0x6bbf00 sub_6bbf00
    Lifted::new(0x7000da8290, fn_7000da8290 as *const (), 0), // +0x6bc290 sub_6bc290
    Lifted::new(0x7000da8320, fn_7000da8320 as *const (), 2), // +0x6bc320 sub_6bc320
    Lifted::new(0x7000da8430, fn_7000da8430 as *const (), 0), // +0x6bc430 sub_6bc430
    Lifted::new(0x7000da8480, fn_7000da8480 as *const (), 0), // +0x6bc480 sub_6bc480
    Lifted::new(0x7000da8620, tpl_1084::<0x8, 0x700177c312, 0x10, 0xfffffffffffffff8, 0x7001d54968, 0x7000da86b3> as *const (), 0), // +0x6bc620 sub_6bc620
    Lifted::new(0x7000da86e0, fn_7000da86e0 as *const (), 0), // +0x6bc6e0 sub_6bc6e0
    Lifted::new(0x7000da8720, fn_7000da8720 as *const (), 0), // +0x6bc720 sub_6bc720
    Lifted::new(0x7000da8910, fn_7000da8910 as *const (), 0), // +0x6bc910 sub_6bc910
    Lifted::new(0x7000da8950, fn_7000da8950 as *const (), 2), // +0x6bc950 sub_6bc950
    Lifted::new(0x7000da8a30, fn_7000da8a30 as *const (), 0), // +0x6bca30 sub_6bca30
    Lifted::new(0x7000da8aa0, fn_7000da8aa0 as *const (), 0), // +0x6bcaa0 sub_6bcaa0
    Lifted::new(0x7000da8b90, fn_7000da8b90 as *const (), 1), // +0x6bcb90 sub_6bcb90
    Lifted::new(0x7000da8bd0, fn_7000da8bd0 as *const (), 2), // +0x6bcbd0 sub_6bcbd0
    Lifted::new(0x7000da8c40, fn_7000da8c40 as *const (), 0), // +0x6bcc40 sub_6bcc40
    Lifted::new(0x7000da8cd0, fn_7000da8cd0 as *const (), 0), // +0x6bccd0 sub_6bccd0
    Lifted::new(0x7000da8d80, fn_7000da8d80 as *const (), 0), // +0x6bcd80 sub_6bcd80
    Lifted::new(0x7000da8e50, fn_7000da8e50 as *const (), 0), // +0x6bce50 sub_6bce50
    Lifted::new(0x7000da8eb0, fn_7000da8eb0 as *const (), 1), // +0x6bceb0 sub_6bceb0
    Lifted::new(0x7000da8ed0, fn_7000da8ed0 as *const (), 1), // +0x6bced0 sub_6bced0
    Lifted::new(0x7000da8f10, fn_7000da8f10 as *const (), 0), // +0x6bcf10 sub_6bcf10
    Lifted::new(0x7000da8f50, fn_7000da8f50 as *const (), 0), // +0x6bcf50 sub_6bcf50
    Lifted::new(0x7000da8fc0, fn_7000da8fc0 as *const (), 0), // +0x6bcfc0 sub_6bcfc0
    Lifted::new(0x7000da9080, fn_7000da9080 as *const (), 0), // +0x6bd080 sub_6bd080
    Lifted::new(0x7000da90b0, fn_7000da90b0 as *const (), 2), // +0x6bd0b0 sub_6bd0b0
    Lifted::new(0x7000da92c0, fn_7000da92c0 as *const (), 2), // +0x6bd2c0 sub_6bd2c0
    Lifted::new(0x7000da9690, fn_7000da9690 as *const (), 2), // +0x6bd690 sub_6bd690
    Lifted::new(0x7000da97c0, fn_7000da97c0 as *const (), 0), // +0x6bd7c0 sub_6bd7c0
    Lifted::new(0x7000da98c0, tpl_1328::<0x8, 0x10> as *const (), 1), // +0x6bd8c0 sub_6bd8c0
    Lifted::new(0x7000da98d0, fn_7000da98d0 as *const (), 0), // +0x6bd8d0 sub_6bd8d0
    Lifted::new(0x7000da99f0, fn_7000da99f0 as *const (), 2), // +0x6bd9f0 sub_6bd9f0
    Lifted::new(0x7000da9b00, fn_7000da9b00 as *const (), 0), // +0x6bdb00 sub_6bdb00
    Lifted::new(0x7000da9ba0, fn_7000da9ba0 as *const (), 0), // +0x6bdba0 sub_6bdba0
    Lifted::new(0x7000da9c60, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000da9c70> as *const (), 0), // +0x6bdc60 sub_6bdc60
    Lifted::new(0x7000da9c80, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000da9c90> as *const (), 0), // +0x6bdc80 sub_6bdc80
    Lifted::new(0x7000da9ca0, fn_7000da9ca0 as *const (), 0), // +0x6bdca0 sub_6bdca0
    Lifted::new(0x7000da9d30, fn_7000da9d30 as *const (), 0), // +0x6bdd30 sub_6bdd30
    Lifted::new(0x7000da9de0, fn_7000da9de0 as *const (), 2), // +0x6bdde0 sub_6bdde0
    Lifted::new(0x7000da9ff0, fn_7000da9ff0 as *const (), 2), // +0x6bdff0 sub_6bdff0
    Lifted::new(0x7000daa160, tpl_1329::<0x700184a4f0> as *const (), 1), // +0x6be160 sub_6be160
    Lifted::new(0x7000daa360, tpl_1329::<0x700184a4f0> as *const (), 1), // +0x6be360 sub_6be360
    Lifted::new(0x7000daa560, tpl_1330::<0x700184a4f0, 0x7000703840> as *const (), 0), // +0x6be560 sub_6be560
    Lifted::new(0x7000daa810, tpl_1330::<0x700184a4f0, 0x7000703840> as *const (), 0), // +0x6be810 sub_6be810
    Lifted::new(0x7000daaac0, fn_7000daaac0 as *const (), 3), // +0x6beac0 sub_6beac0
    Lifted::new(0x7000daac50, fn_7000daac50 as *const (), 2), // +0x6bec50 sub_6bec50
    Lifted::new(0x7000daadf0, fn_7000daadf0 as *const (), 2), // +0x6bedf0 sub_6bedf0
    Lifted::new(0x7000dab4f0, fn_7000dab4f0 as *const (), 2), // +0x6bf4f0 sub_6bf4f0
    Lifted::new(0x7000dabce0, fn_7000dabce0 as *const (), 2), // +0x6bfce0 sub_6bfce0
    Lifted::new(0x7000dabf90, fn_7000dabf90 as *const (), 2), // +0x6bff90 sub_6bff90
    Lifted::new(0x7000dac230, fn_7000dac230 as *const (), 2), // +0x6c0230 sub_6c0230
    Lifted::new(0x7000dac4e0, fn_7000dac4e0 as *const (), 2), // +0x6c04e0 sub_6c04e0
    Lifted::new(0x7000dac780, fn_7000dac780 as *const (), 2), // +0x6c0780 sub_6c0780
    Lifted::new(0x7000daca20, fn_7000daca20 as *const (), 2), // +0x6c0a20 sub_6c0a20
    Lifted::new(0x7000daccd0, fn_7000daccd0 as *const (), 2), // +0x6c0cd0 sub_6c0cd0
    Lifted::new(0x7000dacf70, fn_7000dacf70 as *const (), 2), // +0x6c0f70 sub_6c0f70
    Lifted::new(0x7000dad240, fn_7000dad240 as *const (), 2), // +0x6c1240 sub_6c1240
    Lifted::new(0x7000dad540, fn_7000dad540 as *const (), 2), // +0x6c1540 sub_6c1540
    Lifted::new(0x7000dad810, fn_7000dad810 as *const (), 2), // +0x6c1810 sub_6c1810
    Lifted::new(0x7000dadb10, fn_7000dadb10 as *const (), 0), // +0x6c1b10 sub_6c1b10
    Lifted::new(0x7000dadcf0, fn_7000dadcf0 as *const (), 0), // +0x6c1cf0 sub_6c1cf0
    Lifted::new(0x7000daded0, tpl_1331::<0x24, 0x28, 0x20, 0x7000dadb10, 0xa4, 0x34, 0x32, 0x700184a4f0> as *const (), 0), // +0x6c1ed0 sub_6c1ed0
    Lifted::new(0x7000dadf90, fn_7000dadf90 as *const (), 2), // +0x6c1f90 sub_6c1f90
    Lifted::new(0x7000dae310, tpl_1332::<0x24, 0x20, 0x28, 0x7000dadcf0, 0xa4, 0x34, 0x32, 0x700184a4f0, 0x7000703840> as *const (), 0), // +0x6c2310 sub_6c2310
    Lifted::new(0x7000dae420, fn_7000dae420 as *const (), 2), // +0x6c2420 sub_6c2420
    Lifted::new(0x7000dae860, fn_7000dae860 as *const (), 0), // +0x6c2860 sub_6c2860
    Lifted::new(0x7000dae900, tpl_1224::<0x78, 0x88, 0xfffffffffffffff8, 0x700177c312, 0x80, 0x7001d54968, 0x7000dae966> as *const (), 0), // +0x6c2900 sub_6c2900
    Lifted::new(0x7000dae980, tpl_1224::<0x78, 0x88, 0xfffffffffffffff8, 0x700177c312, 0x80, 0x7001d54968, 0x7000dae9e6> as *const (), 0), // +0x6c2980 sub_6c2980
    Lifted::new(0x7000daea20, tpl_133::<0x7000dae900, 0x700177c312> as *const (), 0), // +0x6c2a20 sub_6c2a20
    Lifted::new(0x7000daea60, tpl_133::<0x7000dae980, 0x700177c312> as *const (), 0), // +0x6c2a60 sub_6c2a60
    Lifted::new(0x7000daeaa0, tpl_33::<0x700177c312> as *const (), 0), // +0x6c2aa0 sub_6c2aa0
    Lifted::new(0x7000daead0, tpl_33::<0x700177c312> as *const (), 0), // +0x6c2ad0 sub_6c2ad0
    Lifted::new(0x7000daeb00, tpl_33::<0x700177c312> as *const (), 0), // +0x6c2b00 sub_6c2b00
    Lifted::new(0x7000daeb30, fn_7000daeb30 as *const (), 0), // +0x6c2b30 sub_6c2b30
    Lifted::new(0x7000daed40, fn_7000daed40 as *const (), 2), // +0x6c2d40 sub_6c2d40
    Lifted::new(0x7000daf1f0, fn_7000daf1f0 as *const (), 2), // +0x6c31f0 sub_6c31f0
    Lifted::new(0x7000daf300, fn_7000daf300 as *const (), 2), // +0x6c3300 sub_6c3300
    Lifted::new(0x7000daf670, fn_7000daf670 as *const (), 2), // +0x6c3670 sub_6c3670
    Lifted::new(0x7000daf9d0, fn_7000daf9d0 as *const (), 2), // +0x6c39d0 sub_6c39d0
    Lifted::new(0x7000dafeb0, fn_7000dafeb0 as *const (), 0), // +0x6c3eb0 sub_6c3eb0
    Lifted::new(0x7000db00d0, fn_7000db00d0 as *const (), 1), // +0x6c40d0 sub_6c40d0
    Lifted::new(0x7000db0170, fn_7000db0170 as *const (), 2), // +0x6c4170 sub_6c4170
    Lifted::new(0x7000db0370, tpl_1333::<0x70007756d0, 0x7000daa560, 0x7000daa160> as *const (), 0), // +0x6c4370 sub_6c4370
    Lifted::new(0x7000db04c0, tpl_1333::<0x70007756d0, 0x7000daa810, 0x7000daa360> as *const (), 0), // +0x6c44c0 sub_6c44c0
    Lifted::new(0x7000db0610, tpl_1331::<0x24, 0x28, 0x20, 0x7000dadb10, 0xa4, 0x34, 0x32, 0x700184a4f0> as *const (), 0), // +0x6c4610 sub_6c4610
    Lifted::new(0x7000db06d0, tpl_1331::<0x24, 0x28, 0x20, 0x7000dadb10, 0xa4, 0x34, 0x32, 0x700184a4f0> as *const (), 0), // +0x6c46d0 sub_6c46d0
    Lifted::new(0x7000db07a0, tpl_1332::<0x24, 0x20, 0x28, 0x7000dadcf0, 0xa4, 0x34, 0x32, 0x700184a4f0, 0x7000703840> as *const (), 0), // +0x6c47a0 sub_6c47a0
    Lifted::new(0x7000db08b0, tpl_1332::<0x24, 0x20, 0x28, 0x7000dadcf0, 0xa4, 0x34, 0x32, 0x700184a4f0, 0x7000703840> as *const (), 0), // +0x6c48b0 sub_6c48b0
    Lifted::new(0x7000db09d0, fn_7000db09d0 as *const (), 2), // +0x6c49d0 sub_6c49d0
    Lifted::new(0x7000db0ce0, fn_7000db0ce0 as *const (), 2), // +0x6c4ce0 sub_6c4ce0
    Lifted::new(0x7000db0d90, fn_7000db0d90 as *const (), 2), // +0x6c4d90 sub_6c4d90
    Lifted::new(0x7000db0dc0, fn_7000db0dc0 as *const (), 2), // +0x6c4dc0 sub_6c4dc0
    Lifted::new(0x7000db0de0, fn_7000db0de0 as *const (), 2), // +0x6c4de0 sub_6c4de0
    Lifted::new(0x7000db0e00, fn_7000db0e00 as *const (), 2), // +0x6c4e00 sub_6c4e00
    Lifted::new(0x7000db1010, fn_7000db1010 as *const (), 2), // +0x6c5010 sub_6c5010
    Lifted::new(0x7000db1170, fn_7000db1170 as *const (), 2), // +0x6c5170 sub_6c5170
    Lifted::new(0x7000db1460, tpl_1334::<0x700075f630> as *const (), 0), // +0x6c5460 sub_6c5460
    Lifted::new(0x7000db14a0, fn_7000db14a0 as *const (), 2), // +0x6c54a0 sub_6c54a0
    Lifted::new(0x7000db16c0, fn_7000db16c0 as *const (), 2), // +0x6c56c0 sub_6c56c0
    Lifted::new(0x7000db1730, tpl_1255::<0x21f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x6c5730 sub_6c5730
    Lifted::new(0x7000db17e0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x6c57e0 sub_6c57e0
    Lifted::new(0x7000db1820, tpl_1335::<0x8, 0xc> as *const (), 0), // +0x6c5820 sub_6c5820
    Lifted::new(0x7000db18b0, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000db1a3d, 0x7000db5030, 0x7000db1a43, 0x70007f0910, 0x7000db1a49> as *const (), 0), // +0x6c58b0 sub_6c58b0
    Lifted::new(0x7000db1ab0, fn_7000db1ab0 as *const (), 2), // +0x6c5ab0 sub_6c5ab0
    Lifted::new(0x7000db1e80, tpl_68 as *const (), 1), // +0x6c5e80 sub_6c5e80
    Lifted::new(0x7000db1f60, tpl_104::<0x8, 0xc> as *const (), 0), // +0x6c5f60 sub_6c5f60
    Lifted::new(0x7000db1fc0, fn_7000db1fc0 as *const (), 0), // +0x6c5fc0 sub_6c5fc0
    Lifted::new(0x7000db21f0, fn_7000db21f0 as *const (), 0), // +0x6c61f0 sub_6c61f0
    Lifted::new(0x7000db2330, fn_7000db2330 as *const (), 2), // +0x6c6330 sub_6c6330
    Lifted::new(0x7000db2540, fn_7000db2540 as *const (), 0), // +0x6c6540 sub_6c6540
    Lifted::new(0x7000db2770, fn_7000db2770 as *const (), 0), // +0x6c6770 sub_6c6770
    Lifted::new(0x7000db28b0, fn_7000db28b0 as *const (), 0), // +0x6c68b0 sub_6c68b0
    Lifted::new(0x7000db2a50, fn_7000db2a50 as *const (), 0), // +0x6c6a50 sub_6c6a50
    Lifted::new(0x7000db2ba0, fn_7000db2ba0 as *const (), 0), // +0x6c6ba0 sub_6c6ba0
    Lifted::new(0x7000db2c50, fn_7000db2c50 as *const (), 0), // +0x6c6c50 sub_6c6c50
    Lifted::new(0x7000db2d40, fn_7000db2d40 as *const (), 0), // +0x6c6d40 sub_6c6d40
    Lifted::new(0x7000db2dd0, fn_7000db2dd0 as *const (), 0), // +0x6c6dd0 sub_6c6dd0
    Lifted::new(0x7000db2e80, fn_7000db2e80 as *const (), 2), // +0x6c6e80 sub_6c6e80
    Lifted::new(0x7000db2fd0, fn_7000db2fd0 as *const (), 2), // +0x6c6fd0 sub_6c6fd0
    Lifted::new(0x7000db3080, fn_7000db3080 as *const (), 2), // +0x6c7080 sub_6c7080
    Lifted::new(0x7000db35f0, fn_7000db35f0 as *const (), 0), // +0x6c75f0 sub_6c75f0
    Lifted::new(0x7000db3710, fn_7000db3710 as *const (), 2), // +0x6c7710 sub_6c7710
    Lifted::new(0x7000db3920, fn_7000db3920 as *const (), 0), // +0x6c7920 sub_6c7920
    Lifted::new(0x7000db3940, fn_7000db3940 as *const (), 1), // +0x6c7940 sub_6c7940
    Lifted::new(0x7000db3990, fn_7000db3990 as *const (), 0), // +0x6c7990 sub_6c7990
    Lifted::new(0x7000db3a60, fn_7000db3a60 as *const (), 0), // +0x6c7a60 sub_6c7a60
    Lifted::new(0x7000db3b10, fn_7000db3b10 as *const (), 2), // +0x6c7b10 sub_6c7b10
    Lifted::new(0x7000db4050, fn_7000db4050 as *const (), 0), // +0x6c8050 sub_6c8050
    Lifted::new(0x7000db4170, fn_7000db4170 as *const (), 0), // +0x6c8170 sub_6c8170
    Lifted::new(0x7000db4470, fn_7000db4470 as *const (), 2), // +0x6c8470 sub_6c8470
    Lifted::new(0x7000db4740, fn_7000db4740 as *const (), 0), // +0x6c8740 sub_6c8740
    Lifted::new(0x7000db4810, fn_7000db4810 as *const (), 0), // +0x6c8810 sub_6c8810
    Lifted::new(0x7000db48e0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000db49e7, 0x7000db1e80, 0x70007f0910, 0x7000db4a03> as *const (), 0), // +0x6c88e0 sub_6c88e0
    Lifted::new(0x7000db4a50, tpl_980::<0x700184aa90, 0x8, 0x18> as *const (), 3), // +0x6c8a50 sub_6c8a50
    Lifted::new(0x7000db4a80, tpl_918::<0x700184aa10, 0x8> as *const (), 3), // +0x6c8a80 sub_6c8a80
    Lifted::new(0x7000db4aa0, tpl_753::<0x700184aa50, 0x8> as *const (), 1), // +0x6c8aa0 sub_6c8aa0
    Lifted::new(0x7000db4ac0, tpl_918::<0x700184aad0, 0x8> as *const (), 3), // +0x6c8ac0 sub_6c8ac0
    Lifted::new(0x7000db4ae0, tpl_984::<0x700177c312> as *const (), 0), // +0x6c8ae0 sub_6c8ae0
    Lifted::new(0x7000db4b00, tpl_919::<0x700177c312> as *const (), 0), // +0x6c8b00 sub_6c8b00
    Lifted::new(0x7000db4b20, tpl_754::<0x700177c312> as *const (), 0), // +0x6c8b20 sub_6c8b20
    Lifted::new(0x7000db4b40, tpl_919::<0x700177c312> as *const (), 0), // +0x6c8b40 sub_6c8b40
    Lifted::new(0x7000db4b60, fn_7000db4b60 as *const (), 0), // +0x6c8b60 sub_6c8b60
    Lifted::new(0x7000db4bd0, fn_7000db4bd0 as *const (), 0), // +0x6c8bd0 sub_6c8bd0
    Lifted::new(0x7000db4c00, fn_7000db4c00 as *const (), 1), // +0x6c8c00 sub_6c8c00
    Lifted::new(0x7000db4c50, fn_7000db4c50 as *const (), 0), // +0x6c8c50 sub_6c8c50
    Lifted::new(0x7000db4cb0, tpl_286::<0x7000db48e0, 0x70017d37a0, 0x70017790e0, 0x7000db4e9b> as *const (), 0), // +0x6c8cb0 sub_6c8cb0
    Lifted::new(0x7000db4f20, tpl_52::<0x8> as *const (), 1), // +0x6c8f20 sub_6c8f20
    Lifted::new(0x7000db4f30, tpl_52::<0x8> as *const (), 1), // +0x6c8f30 sub_6c8f30
    Lifted::new(0x7000db4f40, tpl_52::<0x8> as *const (), 1), // +0x6c8f40 sub_6c8f40
    Lifted::new(0x7000db4f50, tpl_52::<0x8> as *const (), 1), // +0x6c8f50 sub_6c8f50
    Lifted::new(0x7000db4f60, tpl_980::<0x700184aa90, 0x8, 0x18> as *const (), 3), // +0x6c8f60 sub_6c8f60
    Lifted::new(0x7000db4f90, tpl_918::<0x700184aa10, 0x8> as *const (), 3), // +0x6c8f90 sub_6c8f90
    Lifted::new(0x7000db4fb0, tpl_753::<0x700184aa50, 0x8> as *const (), 1), // +0x6c8fb0 sub_6c8fb0
    Lifted::new(0x7000db4fd0, tpl_918::<0x700184aad0, 0x8> as *const (), 3), // +0x6c8fd0 sub_6c8fd0
    Lifted::new(0x7000db4ff0, tpl_315::<0x7001c6c0f0> as *const (), 1), // +0x6c8ff0 sub_6c8ff0
    Lifted::new(0x7000db5000, tpl_315::<0x7001c6c190> as *const (), 1), // +0x6c9000 sub_6c9000
    Lifted::new(0x7000db5010, tpl_315::<0x7001c6c140> as *const (), 1), // +0x6c9010 sub_6c9010
    Lifted::new(0x7000db5020, tpl_315::<0x7001c6c0a0> as *const (), 1), // +0x6c9020 sub_6c9020
    Lifted::new(0x7000db5030, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000db5040> as *const (), 0), // +0x6c9030 sub_6c9030
    Lifted::new(0x7000db5050, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000db5060> as *const (), 0), // +0x6c9050 sub_6c9050
    Lifted::new(0x7000db5070, fn_7000db5070 as *const (), 0), // +0x6c9070 sub_6c9070
    Lifted::new(0x7000db50d0, fn_7000db50d0 as *const (), 0), // +0x6c90d0 sub_6c90d0
    Lifted::new(0x7000db5130, tpl_1336::<0x7001c73190, 0x7000db2fd0, 0x7001c73198, 0x7001804460, 0x7001d54eb8> as *const (), 2), // +0x6c9130 sub_6c9130
    Lifted::new(0x7000db51c0, fn_7000db51c0 as *const (), 2), // +0x6c91c0 sub_6c91c0
    Lifted::new(0x7000db5300, fn_7000db5300 as *const (), 2), // +0x6c9300 sub_6c9300
    Lifted::new(0x7000db5460, fn_7000db5460 as *const (), 1), // +0x6c9460 sub_6c9460
    Lifted::new(0x7000db5550, tpl_1337::<0x7000db7690> as *const (), 0), // +0x6c9550 sub_6c9550
    Lifted::new(0x7000db5590, tpl_1337::<0x7000db7690> as *const (), 0), // +0x6c9590 sub_6c9590
    Lifted::new(0x7000db55d0, fn_7000db55d0 as *const (), 0), // +0x6c95d0 sub_6c95d0
    Lifted::new(0x7000db56c0, fn_7000db56c0 as *const (), 0), // +0x6c96c0 sub_6c96c0
    Lifted::new(0x7000db5720, tpl_1338::<0x7001c72256, 0x700184ab28, 0x7001d54ef8, 0x7000db5746, 0x700074a850, 0x700079cfd0> as *const (), 0), // +0x6c9720 sub_6c9720
    Lifted::new(0x7000db57c0, tpl_1338::<0x7001c72256, 0x700184ab28, 0x7001d54ef8, 0x7000db57e6, 0x700074a850, 0x700079cfd0> as *const (), 0), // +0x6c97c0 sub_6c97c0
    Lifted::new(0x7000db5860, fn_7000db5860 as *const (), 0), // +0x6c9860 sub_6c9860
    Lifted::new(0x7000db58b0, tpl_1339::<0x7000730e70, 0x3, 0x7000db7690> as *const (), 0), // +0x6c98b0 sub_6c98b0
    Lifted::new(0x7000db5910, tpl_1339::<0x7000756270, 0x3, 0x7000db7690> as *const (), 0), // +0x6c9910 sub_6c9910
    Lifted::new(0x7000db5970, fn_7000db5970 as *const (), 0), // +0x6c9970 sub_6c9970
    Lifted::new(0x7000db5bb0, fn_7000db5bb0 as *const (), 0), // +0x6c9bb0 sub_6c9bb0
    Lifted::new(0x7000db5c70, fn_7000db5c70 as *const (), 0), // +0x6c9c70 sub_6c9c70
    Lifted::new(0x7000db5d30, fn_7000db5d30 as *const (), 0), // +0x6c9d30 sub_6c9d30
    Lifted::new(0x7000db5fd0, tpl_1340::<0x7001c6abd8, 0x700184abd0, 0x7000db5d30, 0x20> as *const (), 0), // +0x6c9fd0 sub_6c9fd0
    Lifted::new(0x7000db6090, fn_7000db6090 as *const (), 0), // +0x6ca090 sub_6ca090
    Lifted::new(0x7000db60f0, fn_7000db60f0 as *const (), 0), // +0x6ca0f0 sub_6ca0f0
    Lifted::new(0x7000db6260, tpl_1340::<0x7001c6abd8, 0x700184ac10, 0x7000db5d30, 0x20> as *const (), 0), // +0x6ca260 sub_6ca260
    Lifted::new(0x7000db6320, fn_7000db6320 as *const (), 2), // +0x6ca320 sub_6ca320
    Lifted::new(0x7000db6530, fn_7000db6530 as *const (), 1), // +0x6ca530 sub_6ca530
    Lifted::new(0x7000db65c0, fn_7000db65c0 as *const (), 0), // +0x6ca5c0 sub_6ca5c0
    Lifted::new(0x7000db6760, fn_7000db6760 as *const (), 1), // +0x6ca760 sub_6ca760
    Lifted::new(0x7000db67e0, fn_7000db67e0 as *const (), 0), // +0x6ca7e0 sub_6ca7e0
    Lifted::new(0x7000db6960, fn_7000db6960 as *const (), 0), // +0x6ca960 sub_6ca960
    Lifted::new(0x7000db6a10, tpl_1341::<0x7000db6a70, 0x7000db7690> as *const (), 0), // +0x6caa10 sub_6caa10
    Lifted::new(0x7000db6a70, fn_7000db6a70 as *const (), 0), // +0x6caa70 sub_6caa70
    Lifted::new(0x7000db6ef0, fn_7000db6ef0 as *const (), 0), // +0x6caef0 sub_6caef0
    Lifted::new(0x7000db6f90, tpl_1341::<0x7000db6ff0, 0x7000db7690> as *const (), 0), // +0x6caf90 sub_6caf90
    Lifted::new(0x7000db6ff0, fn_7000db6ff0 as *const (), 0), // +0x6caff0 sub_6caff0
    Lifted::new(0x7000db72f0, tpl_1341::<0x7000db7350, 0x7000db7690> as *const (), 0), // +0x6cb2f0 sub_6cb2f0
    Lifted::new(0x7000db7350, fn_7000db7350 as *const (), 0), // +0x6cb350 sub_6cb350
    Lifted::new(0x7000db7690, fn_7000db7690 as *const (), 0), // +0x6cb690 sub_6cb690
    Lifted::new(0x7000db7790, fn_7000db7790 as *const (), 3), // +0x6cb790 sub_6cb790
    Lifted::new(0x7000db7940, fn_7000db7940 as *const (), 2), // +0x6cb940 sub_6cb940
    Lifted::new(0x7000db7bb0, fn_7000db7bb0 as *const (), 2), // +0x6cbbb0 sub_6cbbb0
    Lifted::new(0x7000db7c60, fn_7000db7c60 as *const (), 0), // +0x6cbc60 sub_6cbc60
    Lifted::new(0x7000db7d10, fn_7000db7d10 as *const (), 0), // +0x6cbd10 sub_6cbd10
    Lifted::new(0x7000db7ee0, fn_7000db7ee0 as *const (), 0), // +0x6cbee0 sub_6cbee0
    Lifted::new(0x7000db8010, fn_7000db8010 as *const (), 1), // +0x6cc010 sub_6cc010
    Lifted::new(0x7000db8020, fn_7000db8020 as *const (), 2), // +0x6cc020 sub_6cc020
    Lifted::new(0x7000db8430, fn_7000db8430 as *const (), 1), // +0x6cc430 sub_6cc430
    Lifted::new(0x7000db84c0, fn_7000db84c0 as *const (), 2), // +0x6cc4c0 sub_6cc4c0
    Lifted::new(0x7000db8570, fn_7000db8570 as *const (), 0), // +0x6cc570 sub_6cc570
    Lifted::new(0x7000db8690, fn_7000db8690 as *const (), 0), // +0x6cc690 sub_6cc690
    Lifted::new(0x7000db8a10, fn_7000db8a10 as *const (), 0), // +0x6cca10 sub_6cca10
    Lifted::new(0x7000db8ae0, fn_7000db8ae0 as *const (), 2), // +0x6ccae0 sub_6ccae0
    Lifted::new(0x7000db8eb0, fn_7000db8eb0 as *const (), 0), // +0x6cceb0 sub_6cceb0
    Lifted::new(0x7000db90e0, fn_7000db90e0 as *const (), 0), // +0x6cd0e0 sub_6cd0e0
    Lifted::new(0x7000db91c0, fn_7000db91c0 as *const (), 0), // +0x6cd1c0 sub_6cd1c0
    Lifted::new(0x7000db9260, fn_7000db9260 as *const (), 0), // +0x6cd260 sub_6cd260
    Lifted::new(0x7000db9380, fn_7000db9380 as *const (), 1), // +0x6cd380 sub_6cd380
    Lifted::new(0x7000db94a0, fn_7000db94a0 as *const (), 0), // +0x6cd4a0 sub_6cd4a0
    Lifted::new(0x7000db95c0, fn_7000db95c0 as *const (), 0), // +0x6cd5c0 sub_6cd5c0
    Lifted::new(0x7000db9640, fn_7000db9640 as *const (), 0), // +0x6cd640 sub_6cd640
    Lifted::new(0x7000db9900, fn_7000db9900 as *const (), 0), // +0x6cd900 sub_6cd900
    Lifted::new(0x7000db9950, fn_7000db9950 as *const (), 2), // +0x6cd950 sub_6cd950
    Lifted::new(0x7000db9b30, tpl_1342::<0x3, 0x7, 0x70013624f0, 0x7000db7690> as *const (), 2), // +0x6cdb30 sub_6cdb30
    Lifted::new(0x7000db9bf0, fn_7000db9bf0 as *const (), 3), // +0x6cdbf0 sub_6cdbf0
    Lifted::new(0x7000db9d30, tpl_1342::<0x3, 0x7, 0x7001362b30, 0x7000db7690> as *const (), 2), // +0x6cdd30 sub_6cdd30
    Lifted::new(0x7000db9df0, fn_7000db9df0 as *const (), 2), // +0x6cddf0 sub_6cddf0
    Lifted::new(0x7000db9fd0, fn_7000db9fd0 as *const (), 0), // +0x6cdfd0 sub_6cdfd0
    Lifted::new(0x7000dba0a0, fn_7000dba0a0 as *const (), 2), // +0x6ce0a0 sub_6ce0a0
    Lifted::new(0x7000dba430, fn_7000dba430 as *const (), 0), // +0x6ce430 sub_6ce430
    Lifted::new(0x7000dba7a0, fn_7000dba7a0 as *const (), 0), // +0x6ce7a0 sub_6ce7a0
    Lifted::new(0x7000dba830, fn_7000dba830 as *const (), 0), // +0x6ce830 sub_6ce830
    Lifted::new(0x7000dba860, fn_7000dba860 as *const (), 0), // +0x6ce860 sub_6ce860
    Lifted::new(0x7000dba8e0, fn_7000dba8e0 as *const (), 2), // +0x6ce8e0 sub_6ce8e0
    Lifted::new(0x7000dbab00, tpl_1343::<0x700078b720> as *const (), 0), // +0x6ceb00 sub_6ceb00
    Lifted::new(0x7000dbab20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dbab31> as *const (), 0), // +0x6ceb20 sub_6ceb20
    Lifted::new(0x7000dbab40, fn_7000dbab40 as *const (), 2), // +0x6ceb40 sub_6ceb40
    Lifted::new(0x7000dbadd0, fn_7000dbadd0 as *const (), 2), // +0x6cedd0 sub_6cedd0
    Lifted::new(0x7000dbafe0, fn_7000dbafe0 as *const (), 2), // +0x6cefe0 sub_6cefe0
    Lifted::new(0x7000dbb3b0, fn_7000dbb3b0 as *const (), 1), // +0x6cf3b0 sub_6cf3b0
    Lifted::new(0x7000dbb440, fn_7000dbb440 as *const (), 0), // +0x6cf440 sub_6cf440
    Lifted::new(0x7000dbb6f0, fn_7000dbb6f0 as *const (), 0), // +0x6cf6f0 sub_6cf6f0
    Lifted::new(0x7000dbb8b0, fn_7000dbb8b0 as *const (), 0), // +0x6cf8b0 sub_6cf8b0
    Lifted::new(0x7000dbba20, fn_7000dbba20 as *const (), 0), // +0x6cfa20 sub_6cfa20
    Lifted::new(0x7000dbbaa0, fn_7000dbbaa0 as *const (), 1), // +0x6cfaa0 sub_6cfaa0
    Lifted::new(0x7000dbbac0, tpl_923::<0x700184ab50> as *const (), 1), // +0x6cfac0 sub_6cfac0
    Lifted::new(0x7000dbbae0, tpl_923::<0x700184ac10> as *const (), 1), // +0x6cfae0 sub_6cfae0
    Lifted::new(0x7000dbbb00, tpl_923::<0x700184abd0> as *const (), 1), // +0x6cfb00 sub_6cfb00
    Lifted::new(0x7000dbbb20, tpl_923::<0x700184ab90> as *const (), 1), // +0x6cfb20 sub_6cfb20
    Lifted::new(0x7000dbbb40, tpl_754::<0x700177c312> as *const (), 0), // +0x6cfb40 sub_6cfb40
    Lifted::new(0x7000dbbb60, tpl_754::<0x700177c312> as *const (), 0), // +0x6cfb60 sub_6cfb60
    Lifted::new(0x7000dbbb80, tpl_754::<0x700177c312> as *const (), 0), // +0x6cfb80 sub_6cfb80
    Lifted::new(0x7000dbbba0, tpl_754::<0x700177c312> as *const (), 0), // +0x6cfba0 sub_6cfba0
    Lifted::new(0x7000dbbbc0, fn_7000dbbbc0 as *const (), 1), // +0x6cfbc0 sub_6cfbc0
    Lifted::new(0x7000dbbbd0, fn_7000dbbbd0 as *const (), 1), // +0x6cfbd0 sub_6cfbd0
    Lifted::new(0x7000dbbbe0, fn_7000dbbbe0 as *const (), 1), // +0x6cfbe0 sub_6cfbe0
    Lifted::new(0x7000dbbbf0, fn_7000dbbbf0 as *const (), 1), // +0x6cfbf0 sub_6cfbf0
    Lifted::new(0x7000dbbc00, tpl_52::<0x8> as *const (), 1), // +0x6cfc00 sub_6cfc00
    Lifted::new(0x7000dbbc10, tpl_52::<0x8> as *const (), 1), // +0x6cfc10 sub_6cfc10
    Lifted::new(0x7000dbbc20, tpl_52::<0x8> as *const (), 1), // +0x6cfc20 sub_6cfc20
    Lifted::new(0x7000dbbc30, tpl_52::<0x8> as *const (), 1), // +0x6cfc30 sub_6cfc30
    Lifted::new(0x7000dbbc40, tpl_923::<0x700184ab50> as *const (), 1), // +0x6cfc40 sub_6cfc40
    Lifted::new(0x7000dbbc60, tpl_923::<0x700184ac10> as *const (), 1), // +0x6cfc60 sub_6cfc60
    Lifted::new(0x7000dbbc80, tpl_923::<0x700184abd0> as *const (), 1), // +0x6cfc80 sub_6cfc80
    Lifted::new(0x7000dbbca0, tpl_923::<0x700184ab90> as *const (), 1), // +0x6cfca0 sub_6cfca0
    Lifted::new(0x7000dbbcc0, tpl_315::<0x7001c6c2d0> as *const (), 1), // +0x6cfcc0 sub_6cfcc0
    Lifted::new(0x7000dbbcd0, tpl_315::<0x7001c6c1e0> as *const (), 1), // +0x6cfcd0 sub_6cfcd0
    Lifted::new(0x7000dbbce0, tpl_315::<0x7001c6c230> as *const (), 1), // +0x6cfce0 sub_6cfce0
    Lifted::new(0x7000dbbcf0, fn_7000dbbcf0 as *const (), 1), // +0x6cfcf0 sub_6cfcf0
    Lifted::new(0x7000dbbd10, fn_7000dbbd10 as *const (), 1), // +0x6cfd10 sub_6cfd10
    Lifted::new(0x7000dbbd60, fn_7000dbbd60 as *const (), 0), // +0x6cfd60 sub_6cfd60
    Lifted::new(0x7000dbbec0, tpl_13::<0x7000a62410> as *const (), 0), // +0x6cfec0 sub_6cfec0
    Lifted::new(0x7000dbbee0, tpl_13::<0x7000a62410> as *const (), 0), // +0x6cfee0 sub_6cfee0
    Lifted::new(0x7000dbbf00, tpl_13::<0x7000a62410> as *const (), 0), // +0x6cff00 sub_6cff00
    Lifted::new(0x7000dbbf20, tpl_13::<0x7000a62410> as *const (), 0), // +0x6cff20 sub_6cff20
    Lifted::new(0x7000dbbf40, tpl_1344::<0x7000a62410> as *const (), 0), // +0x6cff40 sub_6cff40
    Lifted::new(0x7000dbbf60, tpl_1345::<0x700072fbb0> as *const (), 0), // +0x6cff60 sub_6cff60
    Lifted::new(0x7000dbbf90, tpl_13::<0x7000776920> as *const (), 0), // +0x6cff90 sub_6cff90
    Lifted::new(0x7000dbbfd0, fn_7000dbbfd0 as *const (), 1), // +0x6cffd0 sub_6cffd0
    Lifted::new(0x7000dbc010, fn_7000dbc010 as *const (), 0), // +0x6d0010 sub_6d0010
    Lifted::new(0x7000dbc140, fn_7000dbc140 as *const (), 0), // +0x6d0140 sub_6d0140
    Lifted::new(0x7000dbc200, fn_7000dbc200 as *const (), 2), // +0x6d0200 sub_6d0200
    Lifted::new(0x7000dbc490, fn_7000dbc490 as *const (), 0), // +0x6d0490 sub_6d0490
    Lifted::new(0x7000dbc4e0, fn_7000dbc4e0 as *const (), 0), // +0x6d04e0 sub_6d04e0
    Lifted::new(0x7000dbc620, fn_7000dbc620 as *const (), 0), // +0x6d0620 sub_6d0620
    Lifted::new(0x7000dbc670, fn_7000dbc670 as *const (), 0), // +0x6d0670 sub_6d0670
    Lifted::new(0x7000dbc770, fn_7000dbc770 as *const (), 0), // +0x6d0770 sub_6d0770
    Lifted::new(0x7000dbcf10, fn_7000dbcf10 as *const (), 2), // +0x6d0f10 sub_6d0f10
    Lifted::new(0x7000dbd080, tpl_37::<0x168> as *const (), 1), // +0x6d1080 sub_6d1080
    Lifted::new(0x7000dbd0a0, fn_7000dbd0a0 as *const (), 0), // +0x6d10a0 sub_6d10a0
    Lifted::new(0x7000dbd170, fn_7000dbd170 as *const (), 0), // +0x6d1170 sub_6d1170
    Lifted::new(0x7000dbd190, fn_7000dbd190 as *const (), 0), // +0x6d1190 sub_6d1190
    Lifted::new(0x7000dbd210, fn_7000dbd210 as *const (), 0), // +0x6d1210 sub_6d1210
    Lifted::new(0x7000dbd300, fn_7000dbd300 as *const (), 2), // +0x6d1300 sub_6d1300
    Lifted::new(0x7000dbd430, fn_7000dbd430 as *const (), 1), // +0x6d1430 sub_6d1430
    Lifted::new(0x7000dbd4a0, fn_7000dbd4a0 as *const (), 0), // +0x6d14a0 sub_6d14a0
    Lifted::new(0x7000dbd500, fn_7000dbd500 as *const (), 0), // +0x6d1500 sub_6d1500
    Lifted::new(0x7000dbd560, fn_7000dbd560 as *const (), 1), // +0x6d1560 sub_6d1560
    Lifted::new(0x7000dbd5f0, fn_7000dbd5f0 as *const (), 0), // +0x6d15f0 sub_6d15f0
    Lifted::new(0x7000dbd9b0, fn_7000dbd9b0 as *const (), 0), // +0x6d19b0 sub_6d19b0
    Lifted::new(0x7000dbdcd0, fn_7000dbdcd0 as *const (), 0), // +0x6d1cd0 sub_6d1cd0
    Lifted::new(0x7000dbde30, fn_7000dbde30 as *const (), 1), // +0x6d1e30 sub_6d1e30
    Lifted::new(0x7000dbde40, fn_7000dbde40 as *const (), 1), // +0x6d1e40 sub_6d1e40
    Lifted::new(0x7000dbde60, fn_7000dbde60 as *const (), 0), // +0x6d1e60 sub_6d1e60
    Lifted::new(0x7000dbe0c0, fn_7000dbe0c0 as *const (), 0), // +0x6d20c0 sub_6d20c0
    Lifted::new(0x7000dbe100, fn_7000dbe100 as *const (), 2), // +0x6d2100 sub_6d2100
    Lifted::new(0x7000dbe310, fn_7000dbe310 as *const (), 1), // +0x6d2310 sub_6d2310
    Lifted::new(0x7000dbe3f0, fn_7000dbe3f0 as *const (), 0), // +0x6d23f0 sub_6d23f0
    Lifted::new(0x7000dbe450, fn_7000dbe450 as *const (), 0), // +0x6d2450 sub_6d2450
    Lifted::new(0x7000dbe4d0, tpl_1346::<0x7000730310, 0x13, 0x7000700020, 0x700075c640, 0x7000739b00, 0x1f, 0x70010d2e80> as *const (), 0), // +0x6d24d0 sub_6d24d0
    Lifted::new(0x7000dbe580, fn_7000dbe580 as *const (), 0), // +0x6d2580 sub_6d2580
    Lifted::new(0x7000dbe5c0, fn_7000dbe5c0 as *const (), 0), // +0x6d25c0 sub_6d25c0
    Lifted::new(0x7000dbe620, fn_7000dbe620 as *const (), 0), // +0x6d2620 sub_6d2620
    Lifted::new(0x7000dbe6e0, tpl_1346::<0x7000730310, 0x13, 0x7000700020, 0x700075c640, 0x7000739b00, 0x1f, 0x70010d2ee0> as *const (), 0), // +0x6d26e0 sub_6d26e0
    Lifted::new(0x7000dbe790, fn_7000dbe790 as *const (), 0), // +0x6d2790 sub_6d2790
    Lifted::new(0x7000dbe840, fn_7000dbe840 as *const (), 0), // +0x6d2840 sub_6d2840
    Lifted::new(0x7000dbe8e0, fn_7000dbe8e0 as *const (), 0), // +0x6d28e0 sub_6d28e0
    Lifted::new(0x7000dbeb20, fn_7000dbeb20 as *const (), 0), // +0x6d2b20 sub_6d2b20
    Lifted::new(0x7000dbeb50, fn_7000dbeb50 as *const (), 0), // +0x6d2b50 sub_6d2b50
    Lifted::new(0x7000dbed80, fn_7000dbed80 as *const (), 0), // +0x6d2d80 sub_6d2d80
    Lifted::new(0x7000dbedc0, fn_7000dbedc0 as *const (), 0), // +0x6d2dc0 sub_6d2dc0
    Lifted::new(0x7000dbef30, fn_7000dbef30 as *const (), 1), // +0x6d2f30 sub_6d2f30
    Lifted::new(0x7000dbf010, fn_7000dbf010 as *const (), 0), // +0x6d3010 sub_6d3010
    Lifted::new(0x7000dbf240, fn_7000dbf240 as *const (), 1), // +0x6d3240 sub_6d3240
    Lifted::new(0x7000dbf260, fn_7000dbf260 as *const (), 1), // +0x6d3260 sub_6d3260
    Lifted::new(0x7000dbf270, tpl_1347::<0x2f, 0x2b> as *const (), 1), // +0x6d3270 sub_6d3270
    Lifted::new(0x7000dbf290, tpl_144 as *const (), 1), // +0x6d3290 sub_6d3290
    Lifted::new(0x7000dbf2a0, tpl_1347::<0x2b, 0x27> as *const (), 1), // +0x6d32a0 sub_6d32a0
    Lifted::new(0x7000dbf2c0, fn_7000dbf2c0 as *const (), 1), // +0x6d32c0 sub_6d32c0
    Lifted::new(0x7000dbf2e0, fn_7000dbf2e0 as *const (), 0), // +0x6d32e0 sub_6d32e0
    Lifted::new(0x7000dbf430, tpl_33::<0x700177c312> as *const (), 0), // +0x6d3430 sub_6d3430
    Lifted::new(0x7000dbf460, tpl_33::<0x700177c312> as *const (), 0), // +0x6d3460 sub_6d3460
    Lifted::new(0x7000dbf490, tpl_33::<0x700177c312> as *const (), 0), // +0x6d3490 sub_6d3490
    Lifted::new(0x7000dbf4c0, fn_7000dbf4c0 as *const (), 2), // +0x6d34c0 sub_6d34c0
    Lifted::new(0x7000dbf940, fn_7000dbf940 as *const (), 0), // +0x6d3940 sub_6d3940
    Lifted::new(0x7000dbfbe0, fn_7000dbfbe0 as *const (), 2), // +0x6d3be0 sub_6d3be0
    Lifted::new(0x7000dbfe80, fn_7000dbfe80 as *const (), 0), // +0x6d3e80 sub_6d3e80
    Lifted::new(0x7000dbff50, fn_7000dbff50 as *const (), 2), // +0x6d3f50 sub_6d3f50
    Lifted::new(0x7000dc00a0, fn_7000dc00a0 as *const (), 2), // +0x6d40a0 sub_6d40a0
    Lifted::new(0x7000dc0390, fn_7000dc0390 as *const (), 0), // +0x6d4390 sub_6d4390
    Lifted::new(0x7000dc0520, fn_7000dc0520 as *const (), 0), // +0x6d4520 sub_6d4520
    Lifted::new(0x7000dc0770, fn_7000dc0770 as *const (), 2), // +0x6d4770 sub_6d4770
    Lifted::new(0x7000dc0bc0, fn_7000dc0bc0 as *const (), 2), // +0x6d4bc0 sub_6d4bc0
    Lifted::new(0x7000dc0e50, fn_7000dc0e50 as *const (), 2), // +0x6d4e50 sub_6d4e50
    Lifted::new(0x7000dc1120, fn_7000dc1120 as *const (), 0), // +0x6d5120 sub_6d5120
    Lifted::new(0x7000dc11b0, fn_7000dc11b0 as *const (), 0), // +0x6d51b0 sub_6d51b0
    Lifted::new(0x7000dc1280, fn_7000dc1280 as *const (), 1), // +0x6d5280 sub_6d5280
    Lifted::new(0x7000dc1350, fn_7000dc1350 as *const (), 2), // +0x6d5350 sub_6d5350
    Lifted::new(0x7000dc14c0, fn_7000dc14c0 as *const (), 0), // +0x6d54c0 sub_6d54c0
    Lifted::new(0x7000dc1600, fn_7000dc1600 as *const (), 2), // +0x6d5600 sub_6d5600
    Lifted::new(0x7000dc1870, tpl_146::<0x10f> as *const (), 1), // +0x6d5870 sub_6d5870
    Lifted::new(0x7000dc1880, fn_7000dc1880 as *const (), 0), // +0x6d5880 sub_6d5880
    Lifted::new(0x7000dc18a0, fn_7000dc18a0 as *const (), 2), // +0x6d58a0 sub_6d58a0
    Lifted::new(0x7000dc1a50, fn_7000dc1a50 as *const (), 0), // +0x6d5a50 sub_6d5a50
    Lifted::new(0x7000dc1c40, fn_7000dc1c40 as *const (), 2), // +0x6d5c40 sub_6d5c40
    Lifted::new(0x7000dc2110, fn_7000dc2110 as *const (), 0), // +0x6d6110 sub_6d6110
    Lifted::new(0x7000dc2f00, fn_7000dc2f00 as *const (), 0), // +0x6d6f00 sub_6d6f00
    Lifted::new(0x7000dc2fd0, tpl_243::<0x7000dc3b40> as *const (), 0), // +0x6d6fd0 sub_6d6fd0
    Lifted::new(0x7000dc2fe0, fn_7000dc2fe0 as *const (), 0), // +0x6d6fe0 sub_6d6fe0
    Lifted::new(0x7000dc3390, fn_7000dc3390 as *const (), 0), // +0x6d7390 sub_6d7390
    Lifted::new(0x7000dc35b0, fn_7000dc35b0 as *const (), 1), // +0x6d75b0 sub_6d75b0
    Lifted::new(0x7000dc36a0, fn_7000dc36a0 as *const (), 0), // +0x6d76a0 sub_6d76a0
    Lifted::new(0x7000dc3720, fn_7000dc3720 as *const (), 1), // +0x6d7720 sub_6d7720
    Lifted::new(0x7000dc3790, fn_7000dc3790 as *const (), 1), // +0x6d7790 sub_6d7790
    Lifted::new(0x7000dc3850, fn_7000dc3850 as *const (), 1), // +0x6d7850 sub_6d7850
    Lifted::new(0x7000dc38a0, fn_7000dc38a0 as *const (), 1), // +0x6d78a0 sub_6d78a0
    Lifted::new(0x7000dc38f0, fn_7000dc38f0 as *const (), 0), // +0x6d78f0 sub_6d78f0
    Lifted::new(0x7000dc3a50, fn_7000dc3a50 as *const (), 1), // +0x6d7a50 sub_6d7a50
    Lifted::new(0x7000dc3ab0, fn_7000dc3ab0 as *const (), 1), // +0x6d7ab0 sub_6d7ab0
    Lifted::new(0x7000dc3b40, tpl_1255::<0x10f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x6d7b40 sub_6d7b40
    Lifted::new(0x7000dc3c00, tpl_485::<0x1b> as *const (), 1), // +0x6d7c00 sub_6d7c00
    Lifted::new(0x7000dc3c20, tpl_1348::<0x1b> as *const (), 1), // +0x6d7c20 sub_6d7c20
    Lifted::new(0x7000dc3c40, fn_7000dc3c40 as *const (), 1), // +0x6d7c40 sub_6d7c40
    Lifted::new(0x7000dc3c60, fn_7000dc3c60 as *const (), 2), // +0x6d7c60 sub_6d7c60
    Lifted::new(0x7000dc3e00, fn_7000dc3e00 as *const (), 2), // +0x6d7e00 sub_6d7e00
    Lifted::new(0x7000dc4230, fn_7000dc4230 as *const (), 0), // +0x6d8230 sub_6d8230
    Lifted::new(0x7000dc43c0, fn_7000dc43c0 as *const (), 2), // +0x6d83c0 sub_6d83c0
    Lifted::new(0x7000dc44c0, fn_7000dc44c0 as *const (), 0), // +0x6d84c0 sub_6d84c0
    Lifted::new(0x7000dc4670, fn_7000dc4670 as *const (), 1), // +0x6d8670 sub_6d8670
    Lifted::new(0x7000dc4680, fn_7000dc4680 as *const (), 0), // +0x6d8680 sub_6d8680
    Lifted::new(0x7000dc4820, fn_7000dc4820 as *const (), 0), // +0x6d8820 sub_6d8820
    Lifted::new(0x7000dc49c0, fn_7000dc49c0 as *const (), 1), // +0x6d89c0 sub_6d89c0
    Lifted::new(0x7000dc4a30, fn_7000dc4a30 as *const (), 1), // +0x6d8a30 sub_6d8a30
    Lifted::new(0x7000dc4b80, fn_7000dc4b80 as *const (), 1), // +0x6d8b80 sub_6d8b80
    Lifted::new(0x7000dc4c90, fn_7000dc4c90 as *const (), 0), // +0x6d8c90 sub_6d8c90
    Lifted::new(0x7000dc4d60, fn_7000dc4d60 as *const (), 0), // +0x6d8d60 sub_6d8d60
    Lifted::new(0x7000dc4e30, tpl_486::<0x1b> as *const (), 1), // +0x6d8e30 sub_6d8e30
    Lifted::new(0x7000dc4e40, fn_7000dc4e40 as *const (), 1), // +0x6d8e40 sub_6d8e40
    Lifted::new(0x7000dc4f40, fn_7000dc4f40 as *const (), 0), // +0x6d8f40 sub_6d8f40
    Lifted::new(0x7000dc5000, fn_7000dc5000 as *const (), 1), // +0x6d9000 sub_6d9000
    Lifted::new(0x7000dc5020, fn_7000dc5020 as *const (), 1), // +0x6d9020 sub_6d9020
    Lifted::new(0x7000dc5060, fn_7000dc5060 as *const (), 1), // +0x6d9060 sub_6d9060
    Lifted::new(0x7000dc5090, fn_7000dc5090 as *const (), 1), // +0x6d9090 sub_6d9090
    Lifted::new(0x7000dc50b0, fn_7000dc50b0 as *const (), 1), // +0x6d90b0 sub_6d90b0
    Lifted::new(0x7000dc50d0, fn_7000dc50d0 as *const (), 0), // +0x6d90d0 sub_6d90d0
    Lifted::new(0x7000dc54e0, fn_7000dc54e0 as *const (), 2), // +0x6d94e0 sub_6d94e0
    Lifted::new(0x7000dc59b0, fn_7000dc59b0 as *const (), 1), // +0x6d99b0 sub_6d99b0
    Lifted::new(0x7000dc59e0, tpl_170::<0x7000733d90> as *const (), 0), // +0x6d99e0 sub_6d99e0
    Lifted::new(0x7000dc5a00, fn_7000dc5a00 as *const (), 0), // +0x6d9a00 sub_6d9a00
    Lifted::new(0x7000dc5ac0, fn_7000dc5ac0 as *const (), 0), // +0x6d9ac0 sub_6d9ac0
    Lifted::new(0x7000dc5d70, fn_7000dc5d70 as *const (), 1), // +0x6d9d70 sub_6d9d70
    Lifted::new(0x7000dc5d90, fn_7000dc5d90 as *const (), 0), // +0x6d9d90 sub_6d9d90
    Lifted::new(0x7000dc5df0, fn_7000dc5df0 as *const (), 0), // +0x6d9df0 sub_6d9df0
    Lifted::new(0x7000dc5e40, fn_7000dc5e40 as *const (), 0), // +0x6d9e40 sub_6d9e40
    Lifted::new(0x7000dc5ee0, fn_7000dc5ee0 as *const (), 0), // +0x6d9ee0 sub_6d9ee0
    Lifted::new(0x7000dc5ff0, tpl_1349::<0x70009e4b20> as *const (), 2), // +0x6d9ff0 sub_6d9ff0
    Lifted::new(0x7000dc6090, tpl_1349::<0x70009e4b20> as *const (), 2), // +0x6da090 sub_6da090
    Lifted::new(0x7000dc6130, tpl_1350::<0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dc61f7> as *const (), 2), // +0x6da130 sub_6da130
    Lifted::new(0x7000dc6230, tpl_1350::<0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dc62f7> as *const (), 2), // +0x6da230 sub_6da230
    Lifted::new(0x7000dc6330, tpl_1351::<0x70009e4b20> as *const (), 2), // +0x6da330 sub_6da330
    Lifted::new(0x7000dc63d0, tpl_1351::<0x70009e4b20> as *const (), 2), // +0x6da3d0 sub_6da3d0
    Lifted::new(0x7000dc6470, tpl_1352::<0x70017d8e7c, 0x70017d8e80, 0x70017d8ed8, 0x70017d8ee4, 0x70017d8ee8, 0x70017d8e60, 0x70017d8e70, 0x70017d8ec8> as *const (), 3), // +0x6da470 sub_6da470
    Lifted::new(0x7000dc65c0, tpl_1353::<0x7001855d18, 0x700177c260> as *const (), 2), // +0x6da5c0 sub_6da5c0
    Lifted::new(0x7000dc6690, tpl_1349::<0x70009e4b20> as *const (), 2), // +0x6da690 sub_6da690
    Lifted::new(0x7000dc6730, tpl_1349::<0x70009e4b20> as *const (), 2), // +0x6da730 sub_6da730
    Lifted::new(0x7000dc67d0, tpl_1350::<0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dc6897> as *const (), 2), // +0x6da7d0 sub_6da7d0
    Lifted::new(0x7000dc68d0, tpl_1350::<0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dc6997> as *const (), 2), // +0x6da8d0 sub_6da8d0
    Lifted::new(0x7000dc69d0, tpl_1351::<0x70009e4b20> as *const (), 2), // +0x6da9d0 sub_6da9d0
    Lifted::new(0x7000dc6a70, tpl_1351::<0x70009e4b20> as *const (), 2), // +0x6daa70 sub_6daa70
    Lifted::new(0x7000dc6b10, tpl_1352::<0x70017d8e7c, 0x70017d8e80, 0x70017d8ed8, 0x70017d8ee4, 0x70017d8ee8, 0x70017d8e60, 0x70017d8e70, 0x70017d8ec8> as *const (), 3), // +0x6dab10 sub_6dab10
    Lifted::new(0x7000dc6c60, tpl_1353::<0x7001855d18, 0x700177c260> as *const (), 2), // +0x6dac60 sub_6dac60
    Lifted::new(0x7000dc6d30, tpl_1354::<0x70009e4b20> as *const (), 2), // +0x6dad30 sub_6dad30
    Lifted::new(0x7000dc6e00, tpl_1354::<0x70009e4b20> as *const (), 2), // +0x6dae00 sub_6dae00
    Lifted::new(0x7000dc6ed0, tpl_1355::<0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dc6fa7> as *const (), 2), // +0x6daed0 sub_6daed0
    Lifted::new(0x7000dc6fe0, tpl_1355::<0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dc70b7> as *const (), 2), // +0x6dafe0 sub_6dafe0
    Lifted::new(0x7000dc70f0, tpl_1356::<0x70009e4b20> as *const (), 2), // +0x6db0f0 sub_6db0f0
    Lifted::new(0x7000dc71c0, tpl_1356::<0x70009e4b20> as *const (), 2), // +0x6db1c0 sub_6db1c0
    Lifted::new(0x7000dc7290, tpl_1357::<0x70017d8e60, 0x70017d8e7c, 0x70017d8e80, 0x70017d8ed8, 0x70017d8ee4, 0x70017d8ee8, 0x70017d8e70, 0x70017d8ec8> as *const (), 3), // +0x6db290 sub_6db290
    Lifted::new(0x7000dc7400, tpl_1358::<0x4> as *const (), 3), // +0x6db400 sub_6db400
    Lifted::new(0x7000dc7510, tpl_1359::<0x7001855d18, 0x700177c260> as *const (), 2), // +0x6db510 sub_6db510
    Lifted::new(0x7000dc7610, tpl_1354::<0x70009e4b20> as *const (), 2), // +0x6db610 sub_6db610
    Lifted::new(0x7000dc76e0, tpl_1354::<0x70009e4b20> as *const (), 2), // +0x6db6e0 sub_6db6e0
    Lifted::new(0x7000dc77b0, tpl_1355::<0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dc7887> as *const (), 2), // +0x6db7b0 sub_6db7b0
    Lifted::new(0x7000dc78c0, tpl_1355::<0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dc7997> as *const (), 2), // +0x6db8c0 sub_6db8c0
    Lifted::new(0x7000dc79d0, tpl_1356::<0x70009e4b20> as *const (), 2), // +0x6db9d0 sub_6db9d0
    Lifted::new(0x7000dc7aa0, tpl_1356::<0x70009e4b20> as *const (), 2), // +0x6dbaa0 sub_6dbaa0
    Lifted::new(0x7000dc7b70, tpl_1357::<0x70017d8e60, 0x70017d8e7c, 0x70017d8e80, 0x70017d8ed8, 0x70017d8ee4, 0x70017d8ee8, 0x70017d8e70, 0x70017d8ec8> as *const (), 3), // +0x6dbb70 sub_6dbb70
    Lifted::new(0x7000dc7ce0, tpl_1358::<0x4> as *const (), 3), // +0x6dbce0 sub_6dbce0
    Lifted::new(0x7000dc7df0, tpl_1359::<0x7001855d18, 0x700177c260> as *const (), 2), // +0x6dbdf0 sub_6dbdf0
    Lifted::new(0x7000dc7ef0, tpl_1360::<0x4> as *const (), 3), // +0x6dbef0 sub_6dbef0
    Lifted::new(0x7000dc7fb0, tpl_1360::<0x4> as *const (), 3), // +0x6dbfb0 sub_6dbfb0
    Lifted::new(0x7000dc8070, tpl_1361::<0x4> as *const (), 1), // +0x6dc070 sub_6dc070
    Lifted::new(0x7000dc8140, tpl_1361::<0x4> as *const (), 1), // +0x6dc140 sub_6dc140
    Lifted::new(0x7000dc8210, tpl_1361::<0x4> as *const (), 1), // +0x6dc210 sub_6dc210
    Lifted::new(0x7000dc82e0, tpl_1361::<0x4> as *const (), 1), // +0x6dc2e0 sub_6dc2e0
    Lifted::new(0x7000dc83b0, tpl_1361::<0x4> as *const (), 1), // +0x6dc3b0 sub_6dc3b0
    Lifted::new(0x7000dc8480, tpl_1361::<0x4> as *const (), 1), // +0x6dc480 sub_6dc480
    Lifted::new(0x7000dc8550, tpl_1361::<0x4> as *const (), 1), // +0x6dc550 sub_6dc550
    Lifted::new(0x7000dc8620, tpl_1361::<0x4> as *const (), 1), // +0x6dc620 sub_6dc620
    Lifted::new(0x7000dc86f0, fn_7000dc86f0 as *const (), 3), // +0x6dc6f0 sub_6dc6f0
    Lifted::new(0x7000dc8730, fn_7000dc8730 as *const (), 2), // +0x6dc730 sub_6dc730
    Lifted::new(0x7000dc88c0, fn_7000dc88c0 as *const (), 3), // +0x6dc8c0 sub_6dc8c0
    Lifted::new(0x7000dc8bb0, fn_7000dc8bb0 as *const (), 0), // +0x6dcbb0 sub_6dcbb0
    Lifted::new(0x7000dc8c50, fn_7000dc8c50 as *const (), 2), // +0x6dcc50 sub_6dcc50
    Lifted::new(0x7000dc94b0, fn_7000dc94b0 as *const (), 3), // +0x6dd4b0 sub_6dd4b0
    Lifted::new(0x7000dc9780, fn_7000dc9780 as *const (), 0), // +0x6dd780 sub_6dd780
    Lifted::new(0x7000dc9970, fn_7000dc9970 as *const (), 0), // +0x6dd970 sub_6dd970
    Lifted::new(0x7000dc9a40, fn_7000dc9a40 as *const (), 0), // +0x6dda40 sub_6dda40
    Lifted::new(0x7000dc9b10, fn_7000dc9b10 as *const (), 0), // +0x6ddb10 sub_6ddb10
    Lifted::new(0x7000dc9be0, fn_7000dc9be0 as *const (), 2), // +0x6ddbe0 sub_6ddbe0
    Lifted::new(0x7000dc9cd0, fn_7000dc9cd0 as *const (), 1), // +0x6ddcd0 sub_6ddcd0
    Lifted::new(0x7000dc9cf0, fn_7000dc9cf0 as *const (), 1), // +0x6ddcf0 sub_6ddcf0
    Lifted::new(0x7000dc9d20, tpl_20 as *const (), 1), // +0x6ddd20 sub_6ddd20
    Lifted::new(0x7000dc9d30, fn_7000dc9d30 as *const (), 3), // +0x6ddd30 sub_6ddd30
    Lifted::new(0x7000dc9df0, tpl_33::<0x700177c312> as *const (), 0), // +0x6dddf0 sub_6dddf0
    Lifted::new(0x7000dc9e20, tpl_33::<0x700177c312> as *const (), 0), // +0x6dde20 sub_6dde20
    Lifted::new(0x7000dc9e50, tpl_33::<0x700177c312> as *const (), 0), // +0x6dde50 sub_6dde50
    Lifted::new(0x7000dc9e80, tpl_33::<0x700177c312> as *const (), 0), // +0x6dde80 sub_6dde80
    Lifted::new(0x7000dc9eb0, tpl_33::<0x700177c312> as *const (), 0), // +0x6ddeb0 sub_6ddeb0
    Lifted::new(0x7000dc9ee0, tpl_33::<0x700177c312> as *const (), 0), // +0x6ddee0 sub_6ddee0
    Lifted::new(0x7000dc9f10, tpl_33::<0x700177c312> as *const (), 0), // +0x6ddf10 sub_6ddf10
    Lifted::new(0x7000dc9f40, tpl_33::<0x700177c312> as *const (), 0), // +0x6ddf40 sub_6ddf40
    Lifted::new(0x7000dc9f70, tpl_33::<0x700177c312> as *const (), 0), // +0x6ddf70 sub_6ddf70
    Lifted::new(0x7000dc9fa0, tpl_33::<0x700177c312> as *const (), 0), // +0x6ddfa0 sub_6ddfa0
    Lifted::new(0x7000dc9fd0, tpl_33::<0x700177c312> as *const (), 0), // +0x6ddfd0 sub_6ddfd0
    Lifted::new(0x7000dca000, tpl_33::<0x700177c312> as *const (), 0), // +0x6de000 sub_6de000
    Lifted::new(0x7000dca030, tpl_33::<0x700177c312> as *const (), 0), // +0x6de030 sub_6de030
    Lifted::new(0x7000dca060, tpl_33::<0x700177c312> as *const (), 0), // +0x6de060 sub_6de060
    Lifted::new(0x7000dca090, tpl_33::<0x700177c312> as *const (), 0), // +0x6de090 sub_6de090
    Lifted::new(0x7000dca0c0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de0c0 sub_6de0c0
    Lifted::new(0x7000dca0f0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de0f0 sub_6de0f0
    Lifted::new(0x7000dca120, tpl_33::<0x700177c312> as *const (), 0), // +0x6de120 sub_6de120
    Lifted::new(0x7000dca150, tpl_33::<0x700177c312> as *const (), 0), // +0x6de150 sub_6de150
    Lifted::new(0x7000dca180, tpl_33::<0x700177c312> as *const (), 0), // +0x6de180 sub_6de180
    Lifted::new(0x7000dca1b0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de1b0 sub_6de1b0
    Lifted::new(0x7000dca1e0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de1e0 sub_6de1e0
    Lifted::new(0x7000dca210, tpl_33::<0x700177c312> as *const (), 0), // +0x6de210 sub_6de210
    Lifted::new(0x7000dca240, tpl_33::<0x700177c312> as *const (), 0), // +0x6de240 sub_6de240
    Lifted::new(0x7000dca270, tpl_33::<0x700177c312> as *const (), 0), // +0x6de270 sub_6de270
    Lifted::new(0x7000dca2a0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de2a0 sub_6de2a0
    Lifted::new(0x7000dca2d0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de2d0 sub_6de2d0
    Lifted::new(0x7000dca300, tpl_33::<0x700177c312> as *const (), 0), // +0x6de300 sub_6de300
    Lifted::new(0x7000dca330, tpl_33::<0x700177c312> as *const (), 0), // +0x6de330 sub_6de330
    Lifted::new(0x7000dca360, tpl_33::<0x700177c312> as *const (), 0), // +0x6de360 sub_6de360
    Lifted::new(0x7000dca390, tpl_33::<0x700177c312> as *const (), 0), // +0x6de390 sub_6de390
    Lifted::new(0x7000dca3c0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de3c0 sub_6de3c0
    Lifted::new(0x7000dca3f0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de3f0 sub_6de3f0
    Lifted::new(0x7000dca420, tpl_33::<0x700177c312> as *const (), 0), // +0x6de420 sub_6de420
    Lifted::new(0x7000dca450, tpl_33::<0x700177c312> as *const (), 0), // +0x6de450 sub_6de450
    Lifted::new(0x7000dca480, tpl_33::<0x700177c312> as *const (), 0), // +0x6de480 sub_6de480
    Lifted::new(0x7000dca4b0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de4b0 sub_6de4b0
    Lifted::new(0x7000dca4e0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de4e0 sub_6de4e0
    Lifted::new(0x7000dca510, tpl_33::<0x700177c312> as *const (), 0), // +0x6de510 sub_6de510
    Lifted::new(0x7000dca540, tpl_33::<0x700177c312> as *const (), 0), // +0x6de540 sub_6de540
    Lifted::new(0x7000dca570, tpl_33::<0x700177c312> as *const (), 0), // +0x6de570 sub_6de570
    Lifted::new(0x7000dca5a0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de5a0 sub_6de5a0
    Lifted::new(0x7000dca5d0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de5d0 sub_6de5d0
    Lifted::new(0x7000dca600, tpl_33::<0x700177c312> as *const (), 0), // +0x6de600 sub_6de600
    Lifted::new(0x7000dca630, tpl_33::<0x700177c312> as *const (), 0), // +0x6de630 sub_6de630
    Lifted::new(0x7000dca660, tpl_33::<0x700177c312> as *const (), 0), // +0x6de660 sub_6de660
    Lifted::new(0x7000dca690, tpl_33::<0x700177c312> as *const (), 0), // +0x6de690 sub_6de690
    Lifted::new(0x7000dca6c0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de6c0 sub_6de6c0
    Lifted::new(0x7000dca6f0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de6f0 sub_6de6f0
    Lifted::new(0x7000dca720, tpl_33::<0x700177c312> as *const (), 0), // +0x6de720 sub_6de720
    Lifted::new(0x7000dca750, tpl_33::<0x700177c312> as *const (), 0), // +0x6de750 sub_6de750
    Lifted::new(0x7000dca780, tpl_33::<0x700177c312> as *const (), 0), // +0x6de780 sub_6de780
    Lifted::new(0x7000dca7b0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de7b0 sub_6de7b0
    Lifted::new(0x7000dca7e0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de7e0 sub_6de7e0
    Lifted::new(0x7000dca810, tpl_33::<0x700177c312> as *const (), 0), // +0x6de810 sub_6de810
    Lifted::new(0x7000dca840, tpl_33::<0x700177c312> as *const (), 0), // +0x6de840 sub_6de840
    Lifted::new(0x7000dca870, tpl_33::<0x700177c312> as *const (), 0), // +0x6de870 sub_6de870
    Lifted::new(0x7000dca8a0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de8a0 sub_6de8a0
    Lifted::new(0x7000dca8d0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de8d0 sub_6de8d0
    Lifted::new(0x7000dca900, tpl_33::<0x700177c312> as *const (), 0), // +0x6de900 sub_6de900
    Lifted::new(0x7000dca930, tpl_33::<0x700177c312> as *const (), 0), // +0x6de930 sub_6de930
    Lifted::new(0x7000dca960, tpl_33::<0x700177c312> as *const (), 0), // +0x6de960 sub_6de960
    Lifted::new(0x7000dca990, tpl_33::<0x700177c312> as *const (), 0), // +0x6de990 sub_6de990
    Lifted::new(0x7000dca9c0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de9c0 sub_6de9c0
    Lifted::new(0x7000dca9f0, tpl_33::<0x700177c312> as *const (), 0), // +0x6de9f0 sub_6de9f0
    Lifted::new(0x7000dcaa20, tpl_33::<0x700177c312> as *const (), 0), // +0x6dea20 sub_6dea20
    Lifted::new(0x7000dcaa50, tpl_33::<0x700177c312> as *const (), 0), // +0x6dea50 sub_6dea50
    Lifted::new(0x7000dcaa80, tpl_33::<0x700177c312> as *const (), 0), // +0x6dea80 sub_6dea80
    Lifted::new(0x7000dcaab0, tpl_33::<0x700177c312> as *const (), 0), // +0x6deab0 sub_6deab0
    Lifted::new(0x7000dcaae0, tpl_33::<0x700177c312> as *const (), 0), // +0x6deae0 sub_6deae0
    Lifted::new(0x7000dcab10, tpl_33::<0x700177c312> as *const (), 0), // +0x6deb10 sub_6deb10
    Lifted::new(0x7000dcab40, tpl_33::<0x700177c312> as *const (), 0), // +0x6deb40 sub_6deb40
    Lifted::new(0x7000dcab70, tpl_33::<0x700177c312> as *const (), 0), // +0x6deb70 sub_6deb70
    Lifted::new(0x7000dcaba0, tpl_33::<0x700177c312> as *const (), 0), // +0x6deba0 sub_6deba0
    Lifted::new(0x7000dcabd0, tpl_33::<0x700177c312> as *const (), 0), // +0x6debd0 sub_6debd0
    Lifted::new(0x7000dcac00, tpl_33::<0x700177c312> as *const (), 0), // +0x6dec00 sub_6dec00
    Lifted::new(0x7000dcac30, tpl_33::<0x700177c312> as *const (), 0), // +0x6dec30 sub_6dec30
    Lifted::new(0x7000dcac60, tpl_33::<0x700177c312> as *const (), 0), // +0x6dec60 sub_6dec60
    Lifted::new(0x7000dcac90, tpl_33::<0x700177c312> as *const (), 0), // +0x6dec90 sub_6dec90
    Lifted::new(0x7000dcacc0, tpl_33::<0x700177c312> as *const (), 0), // +0x6decc0 sub_6decc0
    Lifted::new(0x7000dcacf0, tpl_33::<0x700177c312> as *const (), 0), // +0x6decf0 sub_6decf0
    Lifted::new(0x7000dcad20, tpl_33::<0x700177c312> as *const (), 0), // +0x6ded20 sub_6ded20
    Lifted::new(0x7000dcad50, tpl_33::<0x700177c312> as *const (), 0), // +0x6ded50 sub_6ded50
    Lifted::new(0x7000dcad80, tpl_33::<0x700177c312> as *const (), 0), // +0x6ded80 sub_6ded80
    Lifted::new(0x7000dcadb0, tpl_33::<0x700177c312> as *const (), 0), // +0x6dedb0 sub_6dedb0
    Lifted::new(0x7000dcade0, tpl_33::<0x700177c312> as *const (), 0), // +0x6dede0 sub_6dede0
    Lifted::new(0x7000dcae10, tpl_33::<0x700177c312> as *const (), 0), // +0x6dee10 sub_6dee10
    Lifted::new(0x7000dcae40, tpl_33::<0x700177c312> as *const (), 0), // +0x6dee40 sub_6dee40
    Lifted::new(0x7000dcae70, tpl_33::<0x700177c312> as *const (), 0), // +0x6dee70 sub_6dee70
    Lifted::new(0x7000dcaea0, tpl_33::<0x700177c312> as *const (), 0), // +0x6deea0 sub_6deea0
    Lifted::new(0x7000dcaed0, tpl_33::<0x700177c312> as *const (), 0), // +0x6deed0 sub_6deed0
    Lifted::new(0x7000dcaf00, tpl_33::<0x700177c312> as *const (), 0), // +0x6def00 sub_6def00
    Lifted::new(0x7000dcaf30, tpl_33::<0x700177c312> as *const (), 0), // +0x6def30 sub_6def30
    Lifted::new(0x7000dcaf60, tpl_33::<0x700177c312> as *const (), 0), // +0x6def60 sub_6def60
    Lifted::new(0x7000dcaf90, tpl_33::<0x700177c312> as *const (), 0), // +0x6def90 sub_6def90
    Lifted::new(0x7000dcafc0, tpl_33::<0x700177c312> as *const (), 0), // +0x6defc0 sub_6defc0
    Lifted::new(0x7000dcaff0, tpl_33::<0x700177c312> as *const (), 0), // +0x6deff0 sub_6deff0
    Lifted::new(0x7000dcb020, tpl_33::<0x700177c312> as *const (), 0), // +0x6df020 sub_6df020
    Lifted::new(0x7000dcb050, tpl_33::<0x700177c312> as *const (), 0), // +0x6df050 sub_6df050
    Lifted::new(0x7000dcb080, tpl_33::<0x700177c312> as *const (), 0), // +0x6df080 sub_6df080
    Lifted::new(0x7000dcb0b0, tpl_33::<0x700177c312> as *const (), 0), // +0x6df0b0 sub_6df0b0
    Lifted::new(0x7000dcb0e0, tpl_33::<0x700177c312> as *const (), 0), // +0x6df0e0 sub_6df0e0
    Lifted::new(0x7000dcb110, tpl_33::<0x700177c312> as *const (), 0), // +0x6df110 sub_6df110
    Lifted::new(0x7000dcb140, tpl_33::<0x700177c312> as *const (), 0), // +0x6df140 sub_6df140
    Lifted::new(0x7000dcb170, tpl_33::<0x700177c312> as *const (), 0), // +0x6df170 sub_6df170
    Lifted::new(0x7000dcb1a0, tpl_33::<0x700177c312> as *const (), 0), // +0x6df1a0 sub_6df1a0
    Lifted::new(0x7000dcb1d0, tpl_33::<0x700177c312> as *const (), 0), // +0x6df1d0 sub_6df1d0
    Lifted::new(0x7000dcb200, tpl_33::<0x700177c312> as *const (), 0), // +0x6df200 sub_6df200
    Lifted::new(0x7000dcb230, tpl_33::<0x700177c312> as *const (), 0), // +0x6df230 sub_6df230
    Lifted::new(0x7000dcb260, tpl_33::<0x700177c312> as *const (), 0), // +0x6df260 sub_6df260
    Lifted::new(0x7000dcb290, tpl_33::<0x700177c312> as *const (), 0), // +0x6df290 sub_6df290
    Lifted::new(0x7000dcb2c0, tpl_33::<0x700177c312> as *const (), 0), // +0x6df2c0 sub_6df2c0
    Lifted::new(0x7000dcb2f0, tpl_33::<0x700177c312> as *const (), 0), // +0x6df2f0 sub_6df2f0
    Lifted::new(0x7000dcb320, tpl_33::<0x700177c312> as *const (), 0), // +0x6df320 sub_6df320
    Lifted::new(0x7000dcb350, tpl_1362::<0x7000dd3220, 0x7000dcb380> as *const (), 0), // +0x6df350 sub_6df350
    Lifted::new(0x7000dcb390, tpl_1362::<0x7000dd3240, 0x7000dcb3c0> as *const (), 0), // +0x6df390 sub_6df390
    Lifted::new(0x7000dcb3d0, tpl_1362::<0x7000dd3260, 0x7000dcb400> as *const (), 0), // +0x6df3d0 sub_6df3d0
    Lifted::new(0x7000dcb410, tpl_1362::<0x7000dd3280, 0x7000dcb440> as *const (), 0), // +0x6df410 sub_6df410
    Lifted::new(0x7000dcb450, tpl_1362::<0x7000dd32a0, 0x7000dcb480> as *const (), 0), // +0x6df450 sub_6df450
    Lifted::new(0x7000dcb490, tpl_1362::<0x7000dd32c0, 0x7000dcb4c0> as *const (), 0), // +0x6df490 sub_6df490
    Lifted::new(0x7000dcb4d0, tpl_1362::<0x7000dd32e0, 0x7000dcb500> as *const (), 0), // +0x6df4d0 sub_6df4d0
    Lifted::new(0x7000dcb510, tpl_1362::<0x7000dd3300, 0x7000dcb540> as *const (), 0), // +0x6df510 sub_6df510
    Lifted::new(0x7000dcb550, tpl_1362::<0x7000dd3320, 0x7000dcb580> as *const (), 0), // +0x6df550 sub_6df550
    Lifted::new(0x7000dcb590, tpl_1362::<0x7000dd3340, 0x7000dcb5c0> as *const (), 0), // +0x6df590 sub_6df590
    Lifted::new(0x7000dcb5d0, tpl_1362::<0x7000dd3360, 0x7000dcb600> as *const (), 0), // +0x6df5d0 sub_6df5d0
    Lifted::new(0x7000dcb610, tpl_1362::<0x7000dd3380, 0x7000dcb640> as *const (), 0), // +0x6df610 sub_6df610
    Lifted::new(0x7000dcb650, tpl_1362::<0x7000dd33a0, 0x7000dcb680> as *const (), 0), // +0x6df650 sub_6df650
    Lifted::new(0x7000dcb690, tpl_1362::<0x7000dd33c0, 0x7000dcb6c0> as *const (), 0), // +0x6df690 sub_6df690
    Lifted::new(0x7000dcb6d0, tpl_1362::<0x7000dd33e0, 0x7000dcb700> as *const (), 0), // +0x6df6d0 sub_6df6d0
    Lifted::new(0x7000dcb710, tpl_1362::<0x7000dd3400, 0x7000dcb740> as *const (), 0), // +0x6df710 sub_6df710
    Lifted::new(0x7000dcb750, tpl_1362::<0x7000dd3420, 0x7000dcb780> as *const (), 0), // +0x6df750 sub_6df750
    Lifted::new(0x7000dcb790, tpl_1362::<0x7000dd3440, 0x7000dcb7c0> as *const (), 0), // +0x6df790 sub_6df790
    Lifted::new(0x7000dcb7d0, tpl_1362::<0x7000dd3460, 0x7000dcb800> as *const (), 0), // +0x6df7d0 sub_6df7d0
    Lifted::new(0x7000dcb810, tpl_1362::<0x7000dd3480, 0x7000dcb840> as *const (), 0), // +0x6df810 sub_6df810
    Lifted::new(0x7000dcb850, tpl_1362::<0x7000dd34a0, 0x7000dcb880> as *const (), 0), // +0x6df850 sub_6df850
    Lifted::new(0x7000dcb890, tpl_1362::<0x7000dd34c0, 0x7000dcb8c0> as *const (), 0), // +0x6df890 sub_6df890
    Lifted::new(0x7000dcb8d0, tpl_1363::<0x7000dd4730> as *const (), 0), // +0x6df8d0 sub_6df8d0
    Lifted::new(0x7000dcb920, tpl_1363::<0x7000dd34e0> as *const (), 0), // +0x6df920 sub_6df920
    Lifted::new(0x7000dcb970, tpl_1362::<0x7000dd4430, 0x7000dcb9a0> as *const (), 0), // +0x6df970 sub_6df970
    Lifted::new(0x7000dcb9b0, tpl_1362::<0x7000dd4470, 0x7000dcb9e0> as *const (), 0), // +0x6df9b0 sub_6df9b0
    Lifted::new(0x7000dcb9f0, tpl_1363::<0x7000dd37a0> as *const (), 0), // +0x6df9f0 sub_6df9f0
    Lifted::new(0x7000dcba40, tpl_1362::<0x7000dd44b0, 0x7000dcba70> as *const (), 0), // +0x6dfa40 sub_6dfa40
    Lifted::new(0x7000dcba80, tpl_1363::<0x7000dd3a10> as *const (), 0), // +0x6dfa80 sub_6dfa80
    Lifted::new(0x7000dcbad0, tpl_1363::<0x7000dd3c70> as *const (), 0), // +0x6dfad0 sub_6dfad0
    Lifted::new(0x7000dcbb20, tpl_1362::<0x7000dd4450, 0x7000dcbb50> as *const (), 0), // +0x6dfb20 sub_6dfb20
    Lifted::new(0x7000dcbb60, tpl_1362::<0x7000dd4490, 0x7000dcbb90> as *const (), 0), // +0x6dfb60 sub_6dfb60
    Lifted::new(0x7000dcbba0, tpl_1363::<0x7000dd3f50> as *const (), 0), // +0x6dfba0 sub_6dfba0
    Lifted::new(0x7000dcbbf0, tpl_1362::<0x7000dd44d0, 0x7000dcbc20> as *const (), 0), // +0x6dfbf0 sub_6dfbf0
    Lifted::new(0x7000dcbc30, tpl_1363::<0x7000dd41c0> as *const (), 0), // +0x6dfc30 sub_6dfc30
    Lifted::new(0x7000dcbc80, tpl_1363::<0x7000dd49f0> as *const (), 0), // +0x6dfc80 sub_6dfc80
    Lifted::new(0x7000dcbcd0, tpl_1363::<0x7000dd44f0> as *const (), 0), // +0x6dfcd0 sub_6dfcd0
    Lifted::new(0x7000dcbd20, tpl_1363::<0x7000dd4de0> as *const (), 0), // +0x6dfd20 sub_6dfd20
    Lifted::new(0x7000dcbd70, fn_7000dcbd70 as *const (), 0), // +0x6dfd70 sub_6dfd70
    Lifted::new(0x7000dcbdd0, fn_7000dcbdd0 as *const (), 2), // +0x6dfdd0 sub_6dfdd0
    Lifted::new(0x7000dcc130, fn_7000dcc130 as *const (), 2), // +0x6e0130 sub_6e0130
    Lifted::new(0x7000dcc450, fn_7000dcc450 as *const (), 2), // +0x6e0450 sub_6e0450
    Lifted::new(0x7000dcc7a0, fn_7000dcc7a0 as *const (), 2), // +0x6e07a0 sub_6e07a0
    Lifted::new(0x7000dccaf0, fn_7000dccaf0 as *const (), 2), // +0x6e0af0 sub_6e0af0
    Lifted::new(0x7000dcce40, fn_7000dcce40 as *const (), 2), // +0x6e0e40 sub_6e0e40
    Lifted::new(0x7000dcd190, fn_7000dcd190 as *const (), 2), // +0x6e1190 sub_6e1190
    Lifted::new(0x7000dcd530, fn_7000dcd530 as *const (), 2), // +0x6e1530 sub_6e1530
    Lifted::new(0x7000dcd850, fn_7000dcd850 as *const (), 2), // +0x6e1850 sub_6e1850
    Lifted::new(0x7000dcdba0, fn_7000dcdba0 as *const (), 2), // +0x6e1ba0 sub_6e1ba0
    Lifted::new(0x7000dcdf30, fn_7000dcdf30 as *const (), 2), // +0x6e1f30 sub_6e1f30
    Lifted::new(0x7000dce280, fn_7000dce280 as *const (), 2), // +0x6e2280 sub_6e2280
    Lifted::new(0x7000dce610, tpl_1364::<0x7000dd1df0> as *const (), 0), // +0x6e2610 sub_6e2610
    Lifted::new(0x7000dce630, tpl_1364::<0x7000dd1fc0> as *const (), 0), // +0x6e2630 sub_6e2630
    Lifted::new(0x7000dce650, tpl_1364::<0x7000dd2180> as *const (), 0), // +0x6e2650 sub_6e2650
    Lifted::new(0x7000dce670, tpl_1364::<0x7000dd2380> as *const (), 0), // +0x6e2670 sub_6e2670
    Lifted::new(0x7000dce690, tpl_1365::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000a9ecd0, 0x7000e9d370> as *const (), 0), // +0x6e2690 sub_6e2690
    Lifted::new(0x7000dce820, tpl_1365::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9de0, 0x7000e9d370> as *const (), 0), // +0x6e2820 sub_6e2820
    Lifted::new(0x7000dce9b0, tpl_1366::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9cb0, 0x7000e9d370> as *const (), 2), // +0x6e29b0 sub_6e29b0
    Lifted::new(0x7000dceb50, tpl_1367::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9cb0, 0x7000e9d370> as *const (), 2), // +0x6e2b50 sub_6e2b50
    Lifted::new(0x7000dced10, tpl_1364::<0x7000dd2580> as *const (), 0), // +0x6e2d10 sub_6e2d10
    Lifted::new(0x7000dced30, tpl_1368::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000db91c0, 0x7000e9d370> as *const (), 0), // +0x6e2d30 sub_6e2d30
    Lifted::new(0x7000dceee0, tpl_1368::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700077c7f0, 0x7000e9d370> as *const (), 0), // +0x6e2ee0 sub_6e2ee0
    Lifted::new(0x7000dcf090, tpl_1364::<0x7000dd2750> as *const (), 0), // +0x6e3090 sub_6e3090
    Lifted::new(0x7000dcf0b0, tpl_1364::<0x7000dd2920> as *const (), 0), // +0x6e30b0 sub_6e30b0
    Lifted::new(0x7000dcf0d0, tpl_1364::<0x7000dd2ae0> as *const (), 0), // +0x6e30d0 sub_6e30d0
    Lifted::new(0x7000dcf0f0, tpl_1364::<0x7000dd2ce0> as *const (), 0), // +0x6e30f0 sub_6e30f0
    Lifted::new(0x7000dcf110, tpl_1365::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000a9ecd0, 0x7000e9d370> as *const (), 0), // +0x6e3110 sub_6e3110
    Lifted::new(0x7000dcf2a0, tpl_1365::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9de0, 0x7000e9d370> as *const (), 0), // +0x6e32a0 sub_6e32a0
    Lifted::new(0x7000dcf430, tpl_1366::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9cb0, 0x7000e9d370> as *const (), 2), // +0x6e3430 sub_6e3430
    Lifted::new(0x7000dcf5d0, tpl_1367::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9cb0, 0x7000e9d370> as *const (), 2), // +0x6e35d0 sub_6e35d0
    Lifted::new(0x7000dcf790, tpl_1364::<0x7000dd2ee0> as *const (), 0), // +0x6e3790 sub_6e3790
    Lifted::new(0x7000dcf7b0, tpl_1368::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000db91c0, 0x7000e9d370> as *const (), 0), // +0x6e37b0 sub_6e37b0
    Lifted::new(0x7000dcf960, tpl_1368::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700077c7f0, 0x7000e9d370> as *const (), 0), // +0x6e3960 sub_6e3960
    Lifted::new(0x7000dcfb10, tpl_1364::<0x7000dd30b0> as *const (), 0), // +0x6e3b10 sub_6e3b10
    Lifted::new(0x7000dcfb30, fn_7000dcfb30 as *const (), 2), // +0x6e3b30 sub_6e3b30
    Lifted::new(0x7000dcfce0, tpl_1364::<0x7000dd03f0> as *const (), 0), // +0x6e3ce0 sub_6e3ce0
    Lifted::new(0x7000dcfd00, tpl_1364::<0x7000dd05e0> as *const (), 0), // +0x6e3d00 sub_6e3d00
    Lifted::new(0x7000dcfd20, tpl_1364::<0x7000dd07d0> as *const (), 0), // +0x6e3d20 sub_6e3d20
    Lifted::new(0x7000dcfd40, tpl_1364::<0x7000dd09c0> as *const (), 0), // +0x6e3d40 sub_6e3d40
    Lifted::new(0x7000dcfd60, tpl_1364::<0x7000dd0bb0> as *const (), 0), // +0x6e3d60 sub_6e3d60
    Lifted::new(0x7000dcfd80, fn_7000dcfd80 as *const (), 2), // +0x6e3d80 sub_6e3d80
    Lifted::new(0x7000dcff30, tpl_1364::<0x7000dd0da0> as *const (), 0), // +0x6e3f30 sub_6e3f30
    Lifted::new(0x7000dcff50, tpl_1364::<0x7000dd0f90> as *const (), 0), // +0x6e3f50 sub_6e3f50
    Lifted::new(0x7000dcff70, tpl_1364::<0x7000dd1180> as *const (), 0), // +0x6e3f70 sub_6e3f70
    Lifted::new(0x7000dcff90, tpl_1364::<0x7000dd1370> as *const (), 0), // +0x6e3f90 sub_6e3f90
    Lifted::new(0x7000dcffb0, tpl_1364::<0x7000dd1560> as *const (), 0), // +0x6e3fb0 sub_6e3fb0
    Lifted::new(0x7000dcffd0, tpl_1364::<0x7000dd1750> as *const (), 0), // +0x6e3fd0 sub_6e3fd0
    Lifted::new(0x7000dcfff0, tpl_1369::<0x70006f2a70, 0x70006f9a60, 0x70009e75c0, 0x70009c8930, 0x7001d550a8, 0x7000783b70, 0x7000e9d370, 0x7000dd07d0> as *const (), 0), // +0x6e3ff0 sub_6e3ff0
    Lifted::new(0x7000dd01e0, tpl_1364::<0x7000dd1a30> as *const (), 0), // +0x6e41e0 sub_6e41e0
    Lifted::new(0x7000dd0200, tpl_1369::<0x70006f2a70, 0x70006f9a60, 0x70009e75c0, 0x70009c8930, 0x7001d550a8, 0x7000783b70, 0x7000e9d370, 0x7000dd30b0> as *const (), 0), // +0x6e4200 sub_6e4200
    Lifted::new(0x7000dd03f0, fn_7000dd03f0 as *const (), 0), // +0x6e43f0 sub_6e43f0
    Lifted::new(0x7000dd05e0, fn_7000dd05e0 as *const (), 0), // +0x6e45e0 sub_6e45e0
    Lifted::new(0x7000dd07d0, fn_7000dd07d0 as *const (), 0), // +0x6e47d0 sub_6e47d0
    Lifted::new(0x7000dd09c0, fn_7000dd09c0 as *const (), 0), // +0x6e49c0 sub_6e49c0
    Lifted::new(0x7000dd0bb0, fn_7000dd0bb0 as *const (), 0), // +0x6e4bb0 sub_6e4bb0
    Lifted::new(0x7000dd0da0, fn_7000dd0da0 as *const (), 0), // +0x6e4da0 sub_6e4da0
    Lifted::new(0x7000dd0f90, fn_7000dd0f90 as *const (), 0), // +0x6e4f90 sub_6e4f90
    Lifted::new(0x7000dd1180, fn_7000dd1180 as *const (), 0), // +0x6e5180 sub_6e5180
    Lifted::new(0x7000dd1370, fn_7000dd1370 as *const (), 0), // +0x6e5370 sub_6e5370
    Lifted::new(0x7000dd1560, fn_7000dd1560 as *const (), 0), // +0x6e5560 sub_6e5560
    Lifted::new(0x7000dd1750, fn_7000dd1750 as *const (), 0), // +0x6e5750 sub_6e5750
    Lifted::new(0x7000dd1a30, fn_7000dd1a30 as *const (), 0), // +0x6e5a30 sub_6e5a30
    Lifted::new(0x7000dd1df0, tpl_1370::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370> as *const (), 0), // +0x6e5df0 sub_6e5df0
    Lifted::new(0x7000dd1fc0, tpl_1371::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370> as *const (), 0), // +0x6e5fc0 sub_6e5fc0
    Lifted::new(0x7000dd2180, tpl_1372::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dd2313> as *const (), 0), // +0x6e6180 sub_6e6180
    Lifted::new(0x7000dd2380, tpl_1373::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dd2514> as *const (), 0), // +0x6e6380 sub_6e6380
    Lifted::new(0x7000dd2580, tpl_1370::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370> as *const (), 0), // +0x6e6580 sub_6e6580
    Lifted::new(0x7000dd2750, tpl_1370::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370> as *const (), 0), // +0x6e6750 sub_6e6750
    Lifted::new(0x7000dd2920, tpl_1371::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370> as *const (), 0), // +0x6e6920 sub_6e6920
    Lifted::new(0x7000dd2ae0, tpl_1372::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dd2c73> as *const (), 0), // +0x6e6ae0 sub_6e6ae0
    Lifted::new(0x7000dd2ce0, tpl_1373::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dd2e74> as *const (), 0), // +0x6e6ce0 sub_6e6ce0
    Lifted::new(0x7000dd2ee0, tpl_1370::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000e9d370> as *const (), 0), // +0x6e6ee0 sub_6e6ee0
    Lifted::new(0x7000dd30b0, fn_7000dd30b0 as *const (), 0), // +0x6e70b0 sub_6e70b0
    Lifted::new(0x7000dd3220, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3231> as *const (), 0), // +0x6e7220 sub_6e7220
    Lifted::new(0x7000dd3240, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3251> as *const (), 0), // +0x6e7240 sub_6e7240
    Lifted::new(0x7000dd3260, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3271> as *const (), 0), // +0x6e7260 sub_6e7260
    Lifted::new(0x7000dd3280, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3291> as *const (), 0), // +0x6e7280 sub_6e7280
    Lifted::new(0x7000dd32a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd32b1> as *const (), 0), // +0x6e72a0 sub_6e72a0
    Lifted::new(0x7000dd32c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd32d1> as *const (), 0), // +0x6e72c0 sub_6e72c0
    Lifted::new(0x7000dd32e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd32f1> as *const (), 0), // +0x6e72e0 sub_6e72e0
    Lifted::new(0x7000dd3300, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3311> as *const (), 0), // +0x6e7300 sub_6e7300
    Lifted::new(0x7000dd3320, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3331> as *const (), 0), // +0x6e7320 sub_6e7320
    Lifted::new(0x7000dd3340, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3351> as *const (), 0), // +0x6e7340 sub_6e7340
    Lifted::new(0x7000dd3360, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3371> as *const (), 0), // +0x6e7360 sub_6e7360
    Lifted::new(0x7000dd3380, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3391> as *const (), 0), // +0x6e7380 sub_6e7380
    Lifted::new(0x7000dd33a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd33b1> as *const (), 0), // +0x6e73a0 sub_6e73a0
    Lifted::new(0x7000dd33c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd33d1> as *const (), 0), // +0x6e73c0 sub_6e73c0
    Lifted::new(0x7000dd33e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd33f1> as *const (), 0), // +0x6e73e0 sub_6e73e0
    Lifted::new(0x7000dd3400, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3411> as *const (), 0), // +0x6e7400 sub_6e7400
    Lifted::new(0x7000dd3420, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3431> as *const (), 0), // +0x6e7420 sub_6e7420
    Lifted::new(0x7000dd3440, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3451> as *const (), 0), // +0x6e7440 sub_6e7440
    Lifted::new(0x7000dd3460, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3471> as *const (), 0), // +0x6e7460 sub_6e7460
    Lifted::new(0x7000dd3480, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd3491> as *const (), 0), // +0x6e7480 sub_6e7480
    Lifted::new(0x7000dd34a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd34b1> as *const (), 0), // +0x6e74a0 sub_6e74a0
    Lifted::new(0x7000dd34c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd34d1> as *const (), 0), // +0x6e74c0 sub_6e74c0
    Lifted::new(0x7000dd34e0, fn_7000dd34e0 as *const (), 2), // +0x6e74e0 sub_6e74e0
    Lifted::new(0x7000dd37a0, fn_7000dd37a0 as *const (), 0), // +0x6e77a0 sub_6e77a0
    Lifted::new(0x7000dd3a10, fn_7000dd3a10 as *const (), 0), // +0x6e7a10 sub_6e7a10
    Lifted::new(0x7000dd3c70, fn_7000dd3c70 as *const (), 2), // +0x6e7c70 sub_6e7c70
    Lifted::new(0x7000dd3f50, fn_7000dd3f50 as *const (), 0), // +0x6e7f50 sub_6e7f50
    Lifted::new(0x7000dd41c0, fn_7000dd41c0 as *const (), 0), // +0x6e81c0 sub_6e81c0
    Lifted::new(0x7000dd4430, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd4441> as *const (), 0), // +0x6e8430 sub_6e8430
    Lifted::new(0x7000dd4450, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd4461> as *const (), 0), // +0x6e8450 sub_6e8450
    Lifted::new(0x7000dd4470, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd4481> as *const (), 0), // +0x6e8470 sub_6e8470
    Lifted::new(0x7000dd4490, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd44a1> as *const (), 0), // +0x6e8490 sub_6e8490
    Lifted::new(0x7000dd44b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd44c1> as *const (), 0), // +0x6e84b0 sub_6e84b0
    Lifted::new(0x7000dd44d0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000dd44e1> as *const (), 0), // +0x6e84d0 sub_6e84d0
    Lifted::new(0x7000dd44f0, fn_7000dd44f0 as *const (), 0), // +0x6e84f0 sub_6e84f0
    Lifted::new(0x7000dd4730, fn_7000dd4730 as *const (), 0), // +0x6e8730 sub_6e8730
    Lifted::new(0x7000dd49f0, fn_7000dd49f0 as *const (), 0), // +0x6e89f0 sub_6e89f0
    Lifted::new(0x7000dd4de0, fn_7000dd4de0 as *const (), 0), // +0x6e8de0 sub_6e8de0
    Lifted::new(0x7000dd50a0, fn_7000dd50a0 as *const (), 2), // +0x6e90a0 sub_6e90a0
    Lifted::new(0x7000dd5690, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6e9690 sub_6e9690
    Lifted::new(0x7000dd5880, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6e9880 sub_6e9880
    Lifted::new(0x7000dd5a70, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6e9a70 sub_6e9a70
    Lifted::new(0x7000dd5c60, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6e9c60 sub_6e9c60
    Lifted::new(0x7000dd5e50, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6e9e50 sub_6e9e50
    Lifted::new(0x7000dd6040, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ea040 sub_6ea040
    Lifted::new(0x7000dd6230, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ea230 sub_6ea230
    Lifted::new(0x7000dd6420, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ea420 sub_6ea420
    Lifted::new(0x7000dd6610, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ea610 sub_6ea610
    Lifted::new(0x7000dd6800, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ea800 sub_6ea800
    Lifted::new(0x7000dd69f0, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ea9f0 sub_6ea9f0
    Lifted::new(0x7000dd6be0, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eabe0 sub_6eabe0
    Lifted::new(0x7000dd6dd0, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eadd0 sub_6eadd0
    Lifted::new(0x7000dd6fc0, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eafc0 sub_6eafc0
    Lifted::new(0x7000dd71b0, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eb1b0 sub_6eb1b0
    Lifted::new(0x7000dd73a0, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eb3a0 sub_6eb3a0
    Lifted::new(0x7000dd7590, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eb590 sub_6eb590
    Lifted::new(0x7000dd7780, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eb780 sub_6eb780
    Lifted::new(0x7000dd7970, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eb970 sub_6eb970
    Lifted::new(0x7000dd7b60, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ebb60 sub_6ebb60
    Lifted::new(0x7000dd7d50, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ebd50 sub_6ebd50
    Lifted::new(0x7000dd7f40, tpl_1374::<0x7000aeb7a0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ebf40 sub_6ebf40
    Lifted::new(0x7000dd8130, tpl_1375::<0x34, 0x7000dda4a0> as *const (), 0), // +0x6ec130 sub_6ec130
    Lifted::new(0x7000dd8160, tpl_1375::<0x34, 0x7000dd8460> as *const (), 0), // +0x6ec160 sub_6ec160
    Lifted::new(0x7000dd8190, tpl_1375::<0x34, 0x7000dd8690> as *const (), 0), // +0x6ec190 sub_6ec190
    Lifted::new(0x7000dd81c0, tpl_1375::<0x34, 0x7000dd88a0> as *const (), 0), // +0x6ec1c0 sub_6ec1c0
    Lifted::new(0x7000dd81f0, tpl_1375::<0x34, 0x7000dd8ab0> as *const (), 0), // +0x6ec1f0 sub_6ec1f0
    Lifted::new(0x7000dd8220, tpl_1375::<0x34, 0x7000dd8cc0> as *const (), 0), // +0x6ec220 sub_6ec220
    Lifted::new(0x7000dd8250, tpl_1375::<0x34, 0x7000dd8ed0> as *const (), 0), // +0x6ec250 sub_6ec250
    Lifted::new(0x7000dd8280, tpl_1375::<0x34, 0x7000dd90e0> as *const (), 0), // +0x6ec280 sub_6ec280
    Lifted::new(0x7000dd82b0, tpl_1375::<0x34, 0x7000dd92d0> as *const (), 0), // +0x6ec2b0 sub_6ec2b0
    Lifted::new(0x7000dd82e0, tpl_1375::<0x34, 0x7000dd94c0> as *const (), 0), // +0x6ec2e0 sub_6ec2e0
    Lifted::new(0x7000dd8310, tpl_1375::<0x34, 0x7000dd96b0> as *const (), 0), // +0x6ec310 sub_6ec310
    Lifted::new(0x7000dd8340, tpl_1375::<0x34, 0x7000dd98a0> as *const (), 0), // +0x6ec340 sub_6ec340
    Lifted::new(0x7000dd8370, tpl_1375::<0x34, 0x7000dd9a90> as *const (), 0), // +0x6ec370 sub_6ec370
    Lifted::new(0x7000dd83a0, tpl_1375::<0x34, 0x7000dd9c80> as *const (), 0), // +0x6ec3a0 sub_6ec3a0
    Lifted::new(0x7000dd83d0, tpl_1375::<0x34, 0x7000dda1e0> as *const (), 0), // +0x6ec3d0 sub_6ec3d0
    Lifted::new(0x7000dd8400, tpl_1375::<0x34, 0x7000dd9f30> as *const (), 0), // +0x6ec400 sub_6ec400
    Lifted::new(0x7000dd8430, tpl_1375::<0x34, 0x7000dda340> as *const (), 0), // +0x6ec430 sub_6ec430
    Lifted::new(0x7000dd8460, fn_7000dd8460 as *const (), 2), // +0x6ec460 sub_6ec460
    Lifted::new(0x7000dd8690, tpl_1376::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ec690 sub_6ec690
    Lifted::new(0x7000dd88a0, tpl_1376::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ec8a0 sub_6ec8a0
    Lifted::new(0x7000dd8ab0, tpl_1376::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ecab0 sub_6ecab0
    Lifted::new(0x7000dd8cc0, tpl_1376::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eccc0 sub_6eccc0
    Lifted::new(0x7000dd8ed0, tpl_1376::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eced0 sub_6eced0
    Lifted::new(0x7000dd90e0, tpl_1377::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ed0e0 sub_6ed0e0
    Lifted::new(0x7000dd92d0, tpl_1377::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ed2d0 sub_6ed2d0
    Lifted::new(0x7000dd94c0, tpl_1377::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ed4c0 sub_6ed4c0
    Lifted::new(0x7000dd96b0, tpl_1377::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ed6b0 sub_6ed6b0
    Lifted::new(0x7000dd98a0, tpl_1377::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6ed8a0 sub_6ed8a0
    Lifted::new(0x7000dd9a90, tpl_1377::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370> as *const (), 2), // +0x6eda90 sub_6eda90
    Lifted::new(0x7000dd9c80, tpl_1378::<0x700076df00, 0x7000e970f0, 0x70006f2a70, 0x70006f9a60, 0x7000e18a60, 0x7000dc9780, 0x70007309a0, 0x7000e9d6f0> as *const (), 0), // +0x6edc80 sub_6edc80
    Lifted::new(0x7000dd9f30, tpl_1378::<0x700076df00, 0x7000e970f0, 0x70006f2a70, 0x70006f9a60, 0x7000e1bd30, 0x7000dc9780, 0x70007309a0, 0x7000e9d6f0> as *const (), 0), // +0x6edf30 sub_6edf30
    Lifted::new(0x7000dda1e0, tpl_1379::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370, 0x7000dd8ab0> as *const (), 2), // +0x6ee1e0 sub_6ee1e0
    Lifted::new(0x7000dda340, tpl_1379::<0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000e9d370, 0x7000dda4a0> as *const (), 2), // +0x6ee340 sub_6ee340
    Lifted::new(0x7000dda4a0, fn_7000dda4a0 as *const (), 2), // +0x6ee4a0 sub_6ee4a0
    Lifted::new(0x7000dda810, tpl_1380::<0x7000de2060> as *const (), 0), // +0x6ee810 sub_6ee810
    Lifted::new(0x7000dda870, tpl_1380::<0x7000de23f0> as *const (), 0), // +0x6ee870 sub_6ee870
    Lifted::new(0x7000dda8d0, tpl_1380::<0x7000de2780> as *const (), 0), // +0x6ee8d0 sub_6ee8d0
    Lifted::new(0x7000dda930, tpl_1380::<0x7000de2b30> as *const (), 0), // +0x6ee930 sub_6ee930
    Lifted::new(0x7000dda990, tpl_1380::<0x7000de2ed0> as *const (), 0), // +0x6ee990 sub_6ee990
    Lifted::new(0x7000dda9f0, tpl_1380::<0x7000de3230> as *const (), 0), // +0x6ee9f0 sub_6ee9f0
    Lifted::new(0x7000ddaa50, tpl_1380::<0x7000de3590> as *const (), 0), // +0x6eea50 sub_6eea50
    Lifted::new(0x7000ddaab0, tpl_1380::<0x7000de3910> as *const (), 0), // +0x6eeab0 sub_6eeab0
    Lifted::new(0x7000ddab10, tpl_1380::<0x7000de3cb0> as *const (), 0), // +0x6eeb10 sub_6eeb10
    Lifted::new(0x7000ddab70, tpl_1380::<0x7000de4040> as *const (), 0), // +0x6eeb70 sub_6eeb70
    Lifted::new(0x7000ddabd0, tpl_1380::<0x7000de43c0> as *const (), 0), // +0x6eebd0 sub_6eebd0
    Lifted::new(0x7000ddac30, tpl_1380::<0x7000de4740> as *const (), 0), // +0x6eec30 sub_6eec30
    Lifted::new(0x7000ddac90, tpl_1380::<0x7000de4ad0> as *const (), 0), // +0x6eec90 sub_6eec90
    Lifted::new(0x7000ddacf0, tpl_1380::<0x7000de4e60> as *const (), 0), // +0x6eecf0 sub_6eecf0
    Lifted::new(0x7000ddad50, tpl_1380::<0x7000de5210> as *const (), 0), // +0x6eed50 sub_6eed50
    Lifted::new(0x7000ddadb0, tpl_1380::<0x7000de55b0> as *const (), 0), // +0x6eedb0 sub_6eedb0
    Lifted::new(0x7000ddae10, tpl_1380::<0x7000de5910> as *const (), 0), // +0x6eee10 sub_6eee10
    Lifted::new(0x7000ddae70, tpl_1380::<0x7000de5c70> as *const (), 0), // +0x6eee70 sub_6eee70
    Lifted::new(0x7000ddaed0, tpl_1380::<0x7000de5ff0> as *const (), 0), // +0x6eeed0 sub_6eeed0
    Lifted::new(0x7000ddaf30, tpl_1380::<0x7000de6390> as *const (), 0), // +0x6eef30 sub_6eef30
    Lifted::new(0x7000ddaf90, tpl_1380::<0x7000de6720> as *const (), 0), // +0x6eef90 sub_6eef90
    Lifted::new(0x7000ddaff0, tpl_1380::<0x7000de6aa0> as *const (), 0), // +0x6eeff0 sub_6eeff0
    Lifted::new(0x7000ddb050, tpl_1380::<0x7000ddb6b0> as *const (), 0), // +0x6ef050 sub_6ef050
    Lifted::new(0x7000ddb0b0, tpl_1380::<0x7000dde920> as *const (), 0), // +0x6ef0b0 sub_6ef0b0
    Lifted::new(0x7000ddb110, tpl_1380::<0x7000ddefa0> as *const (), 0), // +0x6ef110 sub_6ef110
    Lifted::new(0x7000ddb170, tpl_1380::<0x7000ddf480> as *const (), 0), // +0x6ef170 sub_6ef170
    Lifted::new(0x7000ddb1d0, tpl_1380::<0x7000ddf960> as *const (), 0), // +0x6ef1d0 sub_6ef1d0
    Lifted::new(0x7000ddb230, tpl_1380::<0x7000ddfe40> as *const (), 0), // +0x6ef230 sub_6ef230
    Lifted::new(0x7000ddb290, tpl_1380::<0x7000de0320> as *const (), 0), // +0x6ef290 sub_6ef290
    Lifted::new(0x7000ddb2f0, tpl_1380::<0x7000ddec60> as *const (), 0), // +0x6ef2f0 sub_6ef2f0
    Lifted::new(0x7000ddb350, tpl_1380::<0x7000de0800> as *const (), 0), // +0x6ef350 sub_6ef350
    Lifted::new(0x7000ddb3b0, tpl_1380::<0x7000de0ce0> as *const (), 0), // +0x6ef3b0 sub_6ef3b0
    Lifted::new(0x7000ddb410, tpl_1380::<0x7000de11c0> as *const (), 0), // +0x6ef410 sub_6ef410
    Lifted::new(0x7000ddb470, tpl_1380::<0x7000de16a0> as *const (), 0), // +0x6ef470 sub_6ef470
    Lifted::new(0x7000ddb4d0, tpl_1380::<0x7000de1b80> as *const (), 0), // +0x6ef4d0 sub_6ef4d0
    Lifted::new(0x7000ddb530, tpl_1380::<0x7000ddc150> as *const (), 0), // +0x6ef530 sub_6ef530
    Lifted::new(0x7000ddb590, tpl_1380::<0x7000ddca90> as *const (), 0), // +0x6ef590 sub_6ef590
    Lifted::new(0x7000ddb5f0, tpl_1380::<0x7000ddd470> as *const (), 0), // +0x6ef5f0 sub_6ef5f0
    Lifted::new(0x7000ddb650, tpl_1380::<0x7000ddde70> as *const (), 0), // +0x6ef650 sub_6ef650
    Lifted::new(0x7000ddb6b0, fn_7000ddb6b0 as *const (), 2), // +0x6ef6b0 sub_6ef6b0
    Lifted::new(0x7000ddc150, fn_7000ddc150 as *const (), 2), // +0x6f0150 sub_6f0150
    Lifted::new(0x7000ddca90, fn_7000ddca90 as *const (), 2), // +0x6f0a90 sub_6f0a90
    Lifted::new(0x7000ddd470, fn_7000ddd470 as *const (), 2), // +0x6f1470 sub_6f1470
    Lifted::new(0x7000ddde70, fn_7000ddde70 as *const (), 2), // +0x6f1e70 sub_6f1e70
    Lifted::new(0x7000dde920, tpl_1381::<0x70006f2a70, 0x70006f9a60, 0x70009c9cb0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f2920 sub_6f2920
    Lifted::new(0x7000ddec60, tpl_1381::<0x70006f2a70, 0x70006f9a60, 0x70009c9cb0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f2c60 sub_6f2c60
    Lifted::new(0x7000ddefa0, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f2fa0 sub_6f2fa0
    Lifted::new(0x7000ddf480, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f3480 sub_6f3480
    Lifted::new(0x7000ddf960, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f3960 sub_6f3960
    Lifted::new(0x7000ddfe40, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f3e40 sub_6f3e40
    Lifted::new(0x7000de0320, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f4320 sub_6f4320
    Lifted::new(0x7000de0800, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f4800 sub_6f4800
    Lifted::new(0x7000de0ce0, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f4ce0 sub_6f4ce0
    Lifted::new(0x7000de11c0, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f51c0 sub_6f51c0
    Lifted::new(0x7000de16a0, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f56a0 sub_6f56a0
    Lifted::new(0x7000de1b80, tpl_1382::<0x70006f2a70, 0x70006f9a60, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f5b80 sub_6f5b80
    Lifted::new(0x7000de2060, tpl_1383::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f6060 sub_6f6060
    Lifted::new(0x7000de23f0, tpl_1384::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f63f0 sub_6f63f0
    Lifted::new(0x7000de2780, tpl_1385::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000de2a3c> as *const (), 2), // +0x6f6780 sub_6f6780
    Lifted::new(0x7000de2b30, tpl_1386::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000de2dde> as *const (), 2), // +0x6f6b30 sub_6f6b30
    Lifted::new(0x7000de2ed0, tpl_1387::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000a9ecd0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f6ed0 sub_6f6ed0
    Lifted::new(0x7000de3230, tpl_1387::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9de0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f7230 sub_6f7230
    Lifted::new(0x7000de3590, tpl_1388::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9cb0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f7590 sub_6f7590
    Lifted::new(0x7000de3910, tpl_1389::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9cb0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f7910 sub_6f7910
    Lifted::new(0x7000de3cb0, tpl_1383::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f7cb0 sub_6f7cb0
    Lifted::new(0x7000de4040, tpl_1390::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000db91c0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f8040 sub_6f8040
    Lifted::new(0x7000de43c0, tpl_1390::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700077c7f0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f83c0 sub_6f83c0
    Lifted::new(0x7000de4740, tpl_1383::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f8740 sub_6f8740
    Lifted::new(0x7000de4ad0, tpl_1384::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f8ad0 sub_6f8ad0
    Lifted::new(0x7000de4e60, tpl_1385::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000de511c> as *const (), 2), // +0x6f8e60 sub_6f8e60
    Lifted::new(0x7000de5210, tpl_1386::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000de54be> as *const (), 2), // +0x6f9210 sub_6f9210
    Lifted::new(0x7000de55b0, tpl_1387::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000a9ecd0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f95b0 sub_6f95b0
    Lifted::new(0x7000de5910, tpl_1387::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9de0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f9910 sub_6f9910
    Lifted::new(0x7000de5c70, tpl_1388::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9cb0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f9c70 sub_6f9c70
    Lifted::new(0x7000de5ff0, tpl_1389::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x70009c9cb0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6f9ff0 sub_6f9ff0
    Lifted::new(0x7000de6390, tpl_1383::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6fa390 sub_6fa390
    Lifted::new(0x7000de6720, tpl_1390::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x7000db91c0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6fa720 sub_6fa720
    Lifted::new(0x7000de6aa0, tpl_1390::<0x70006f2a70, 0x70006f9a60, 0x7000aeb7a0, 0x700077c7f0, 0x700072d6b0, 0x700076df00, 0x700079db00, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x6faaa0 sub_6faaa0
    Lifted::new(0x7000de6e20, fn_7000de6e20 as *const (), 2), // +0x6fae20 sub_6fae20
    Lifted::new(0x7000de71a0, tpl_1391::<0x700074a850, 0x700079cfd0, 0x700078a7f0, 0x7000e00ae0> as *const (), 0), // +0x6fb1a0 sub_6fb1a0
    Lifted::new(0x7000de7330, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e00e00> as *const (), 0), // +0x6fb330 sub_6fb330
    Lifted::new(0x7000de74c0, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e00f20> as *const (), 0), // +0x6fb4c0 sub_6fb4c0
    Lifted::new(0x7000de7650, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e01040> as *const (), 0), // +0x6fb650 sub_6fb650
    Lifted::new(0x7000de77e0, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e01160> as *const (), 0), // +0x6fb7e0 sub_6fb7e0
    Lifted::new(0x7000de7970, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e01280> as *const (), 0), // +0x6fb970 sub_6fb970
    Lifted::new(0x7000de7b00, tpl_1391::<0x700074a850, 0x700079cfd0, 0x700078a7f0, 0x7000e00c70> as *const (), 0), // +0x6fbb00 sub_6fbb00
    Lifted::new(0x7000de7c90, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e013a0> as *const (), 0), // +0x6fbc90 sub_6fbc90
    Lifted::new(0x7000de7e20, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e014c0> as *const (), 0), // +0x6fbe20 sub_6fbe20
    Lifted::new(0x7000de7fb0, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e015e0> as *const (), 0), // +0x6fbfb0 sub_6fbfb0
    Lifted::new(0x7000de8140, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e01700> as *const (), 0), // +0x6fc140 sub_6fc140
    Lifted::new(0x7000de82d0, tpl_1392::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000e01820> as *const (), 0), // +0x6fc2d0 sub_6fc2d0
    Lifted::new(0x7000de8460, fn_7000de8460 as *const (), 0), // +0x6fc460 sub_6fc460
    Lifted::new(0x7000de86b0, tpl_1393::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000c399e0, 0x7000de8d90> as *const (), 0), // +0x6fc6b0 sub_6fc6b0
    Lifted::new(0x7000de8900, tpl_1393::<0x700074a850, 0x700079cfd0, 0x700076df00, 0x7000c399e0, 0x7000de8d90> as *const (), 0), // +0x6fc900 sub_6fc900
    Lifted::new(0x7000de8b50, fn_7000de8b50 as *const (), 2), // +0x6fcb50 sub_6fcb50
    Lifted::new(0x7000de8d90, fn_7000de8d90 as *const (), 2), // +0x6fcd90 sub_6fcd90
    Lifted::new(0x7000de90c0, fn_7000de90c0 as *const (), 0), // +0x6fd0c0 sub_6fd0c0
    Lifted::new(0x7000de91e0, fn_7000de91e0 as *const (), 2), // +0x6fd1e0 sub_6fd1e0
    Lifted::new(0x7000de95c0, tpl_1394::<0x7000df8e80> as *const (), 0), // +0x6fd5c0 sub_6fd5c0
    Lifted::new(0x7000de9610, tpl_1395::<0x7000e00800, 0x7000de964b> as *const (), 0), // +0x6fd610 sub_6fd610
    Lifted::new(0x7000de9660, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00800, 0x7000de973d> as *const (), 0), // +0x6fd660 sub_6fd660
    Lifted::new(0x7000de9780, tpl_1394::<0x7000df9110> as *const (), 0), // +0x6fd780 sub_6fd780
    Lifted::new(0x7000de97d0, tpl_1395::<0x7000e00820, 0x7000de980b> as *const (), 0), // +0x6fd7d0 sub_6fd7d0
    Lifted::new(0x7000de9820, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00820, 0x7000de98fd> as *const (), 0), // +0x6fd820 sub_6fd820
    Lifted::new(0x7000de9940, tpl_1394::<0x7000df93a0> as *const (), 0), // +0x6fd940 sub_6fd940
    Lifted::new(0x7000de9990, tpl_1395::<0x7000e00840, 0x7000de99cb> as *const (), 0), // +0x6fd990 sub_6fd990
    Lifted::new(0x7000de99e0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00840, 0x7000de9abd> as *const (), 0), // +0x6fd9e0 sub_6fd9e0
    Lifted::new(0x7000de9b00, tpl_1394::<0x7000df9630> as *const (), 0), // +0x6fdb00 sub_6fdb00
    Lifted::new(0x7000de9b50, tpl_1395::<0x7000e00860, 0x7000de9b8b> as *const (), 0), // +0x6fdb50 sub_6fdb50
    Lifted::new(0x7000de9ba0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00860, 0x7000de9c7d> as *const (), 0), // +0x6fdba0 sub_6fdba0
    Lifted::new(0x7000de9cc0, tpl_1394::<0x7000df98c0> as *const (), 0), // +0x6fdcc0 sub_6fdcc0
    Lifted::new(0x7000de9d10, tpl_1395::<0x7000e00880, 0x7000de9d4b> as *const (), 0), // +0x6fdd10 sub_6fdd10
    Lifted::new(0x7000de9d60, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00880, 0x7000de9e3d> as *const (), 0), // +0x6fdd60 sub_6fdd60
    Lifted::new(0x7000de9e80, tpl_1394::<0x7000df9b50> as *const (), 0), // +0x6fde80 sub_6fde80
    Lifted::new(0x7000de9ed0, tpl_1395::<0x7000e008a0, 0x7000de9f0b> as *const (), 0), // +0x6fded0 sub_6fded0
    Lifted::new(0x7000de9f20, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e008a0, 0x7000de9ffd> as *const (), 0), // +0x6fdf20 sub_6fdf20
    Lifted::new(0x7000dea040, tpl_1394::<0x7000df9de0> as *const (), 0), // +0x6fe040 sub_6fe040
    Lifted::new(0x7000dea090, tpl_1395::<0x7000e008c0, 0x7000dea0cb> as *const (), 0), // +0x6fe090 sub_6fe090
    Lifted::new(0x7000dea0e0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e008c0, 0x7000dea1bd> as *const (), 0), // +0x6fe0e0 sub_6fe0e0
    Lifted::new(0x7000dea200, tpl_1394::<0x7000dfa070> as *const (), 0), // +0x6fe200 sub_6fe200
    Lifted::new(0x7000dea250, tpl_1395::<0x7000e008e0, 0x7000dea28b> as *const (), 0), // +0x6fe250 sub_6fe250
    Lifted::new(0x7000dea2a0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e008e0, 0x7000dea37d> as *const (), 0), // +0x6fe2a0 sub_6fe2a0
    Lifted::new(0x7000dea3c0, tpl_1394::<0x7000dfa300> as *const (), 0), // +0x6fe3c0 sub_6fe3c0
    Lifted::new(0x7000dea410, tpl_1395::<0x7000e00900, 0x7000dea44b> as *const (), 0), // +0x6fe410 sub_6fe410
    Lifted::new(0x7000dea460, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00900, 0x7000dea53d> as *const (), 0), // +0x6fe460 sub_6fe460
    Lifted::new(0x7000dea580, tpl_1394::<0x7000dfa590> as *const (), 0), // +0x6fe580 sub_6fe580
    Lifted::new(0x7000dea5d0, tpl_1395::<0x7000e00920, 0x7000dea60b> as *const (), 0), // +0x6fe5d0 sub_6fe5d0
    Lifted::new(0x7000dea620, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00920, 0x7000dea6fd> as *const (), 0), // +0x6fe620 sub_6fe620
    Lifted::new(0x7000dea740, tpl_1394::<0x7000dfa760> as *const (), 0), // +0x6fe740 sub_6fe740
    Lifted::new(0x7000dea790, tpl_1395::<0x7000e00940, 0x7000dea7cb> as *const (), 0), // +0x6fe790 sub_6fe790
    Lifted::new(0x7000dea7e0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00940, 0x7000dea8bd> as *const (), 0), // +0x6fe7e0 sub_6fe7e0
    Lifted::new(0x7000dea900, tpl_1394::<0x7000dfa930> as *const (), 0), // +0x6fe900 sub_6fe900
    Lifted::new(0x7000dea950, tpl_1395::<0x7000e00960, 0x7000dea98b> as *const (), 0), // +0x6fe950 sub_6fe950
    Lifted::new(0x7000dea9a0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00960, 0x7000deaa7d> as *const (), 0), // +0x6fe9a0 sub_6fe9a0
    Lifted::new(0x7000deaac0, tpl_1394::<0x7000dfabc0> as *const (), 0), // +0x6feac0 sub_6feac0
    Lifted::new(0x7000deab10, tpl_1395::<0x7000e00980, 0x7000deab4b> as *const (), 0), // +0x6feb10 sub_6feb10
    Lifted::new(0x7000deab60, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00980, 0x7000deac3d> as *const (), 0), // +0x6feb60 sub_6feb60
    Lifted::new(0x7000deac80, tpl_1394::<0x7000dfae50> as *const (), 0), // +0x6fec80 sub_6fec80
    Lifted::new(0x7000deacd0, tpl_1395::<0x7000e009a0, 0x7000dead0b> as *const (), 0), // +0x6fecd0 sub_6fecd0
    Lifted::new(0x7000dead20, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e009a0, 0x7000deadfd> as *const (), 0), // +0x6fed20 sub_6fed20
    Lifted::new(0x7000deae40, tpl_1394::<0x7000dfb0e0> as *const (), 0), // +0x6fee40 sub_6fee40
    Lifted::new(0x7000deae90, tpl_1395::<0x7000e009c0, 0x7000deaecb> as *const (), 0), // +0x6fee90 sub_6fee90
    Lifted::new(0x7000deaee0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e009c0, 0x7000deafbd> as *const (), 0), // +0x6feee0 sub_6feee0
    Lifted::new(0x7000deb000, tpl_1394::<0x7000dfb370> as *const (), 0), // +0x6ff000 sub_6ff000
    Lifted::new(0x7000deb050, tpl_1395::<0x7000e009e0, 0x7000deb08b> as *const (), 0), // +0x6ff050 sub_6ff050
    Lifted::new(0x7000deb0a0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e009e0, 0x7000deb17d> as *const (), 0), // +0x6ff0a0 sub_6ff0a0
    Lifted::new(0x7000deb1c0, tpl_1394::<0x7000dfb600> as *const (), 0), // +0x6ff1c0 sub_6ff1c0
    Lifted::new(0x7000deb210, tpl_1395::<0x7000e00a00, 0x7000deb24b> as *const (), 0), // +0x6ff210 sub_6ff210
    Lifted::new(0x7000deb260, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00a00, 0x7000deb33d> as *const (), 0), // +0x6ff260 sub_6ff260
    Lifted::new(0x7000deb380, tpl_1394::<0x7000dfb890> as *const (), 0), // +0x6ff380 sub_6ff380
    Lifted::new(0x7000deb3d0, tpl_1395::<0x7000e00a20, 0x7000deb40b> as *const (), 0), // +0x6ff3d0 sub_6ff3d0
    Lifted::new(0x7000deb420, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00a20, 0x7000deb4fd> as *const (), 0), // +0x6ff420 sub_6ff420
    Lifted::new(0x7000deb540, tpl_1394::<0x7000dfbb20> as *const (), 0), // +0x6ff540 sub_6ff540
    Lifted::new(0x7000deb590, tpl_1395::<0x7000e00a40, 0x7000deb5cb> as *const (), 0), // +0x6ff590 sub_6ff590
    Lifted::new(0x7000deb5e0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00a40, 0x7000deb6bd> as *const (), 0), // +0x6ff5e0 sub_6ff5e0
    Lifted::new(0x7000deb700, tpl_1394::<0x7000dfbdb0> as *const (), 0), // +0x6ff700 sub_6ff700
    Lifted::new(0x7000deb750, tpl_1395::<0x7000e00a60, 0x7000deb78b> as *const (), 0), // +0x6ff750 sub_6ff750
    Lifted::new(0x7000deb7a0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00a60, 0x7000deb87d> as *const (), 0), // +0x6ff7a0 sub_6ff7a0
    Lifted::new(0x7000deb8c0, tpl_1394::<0x7000dfc040> as *const (), 0), // +0x6ff8c0 sub_6ff8c0
    Lifted::new(0x7000deb910, tpl_1395::<0x7000e00a80, 0x7000deb94b> as *const (), 0), // +0x6ff910 sub_6ff910
    Lifted::new(0x7000deb960, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00a80, 0x7000deba3d> as *const (), 0), // +0x6ff960 sub_6ff960
    Lifted::new(0x7000deba80, tpl_1394::<0x7000dfc2d0> as *const (), 0), // +0x6ffa80 sub_6ffa80
    Lifted::new(0x7000debad0, tpl_1395::<0x7000e00aa0, 0x7000debb0b> as *const (), 0), // +0x6ffad0 sub_6ffad0
    Lifted::new(0x7000debb20, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00aa0, 0x7000debbfd> as *const (), 0), // +0x6ffb20 sub_6ffb20
    Lifted::new(0x7000debc40, tpl_1397::<0x7000df8c60, 0x7000debc6a> as *const (), 0), // +0x6ffc40 sub_6ffc40
    Lifted::new(0x7000debc80, tpl_1395::<0x7000e01940, 0x7000debcbb> as *const (), 0), // +0x6ffc80 sub_6ffc80
    Lifted::new(0x7000debcd0, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e01940, 0x7000debdad> as *const (), 0), // +0x6ffcd0 sub_6ffcd0
    Lifted::new(0x7000debdf0, tpl_1397::<0x7000df8c80, 0x7000debe1a> as *const (), 0), // +0x6ffdf0 sub_6ffdf0
    Lifted::new(0x7000debe30, tpl_1398::<0x7000e00ae0> as *const (), 0), // +0x6ffe30 sub_6ffe30
    Lifted::new(0x7000debe80, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e00ae0> as *const (), 0), // +0x6ffe80 sub_6ffe80
    Lifted::new(0x7000debf80, tpl_1397::<0x7000df8ca0, 0x7000debfaa> as *const (), 0), // +0x6fff80 sub_6fff80
    Lifted::new(0x7000debfc0, tpl_1398::<0x7000e00e00> as *const (), 0), // +0x6fffc0 sub_6fffc0
    Lifted::new(0x7000dec010, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e00e00> as *const (), 0), // +0x700010 sub_700010
    Lifted::new(0x7000dec110, tpl_1397::<0x7000df8cc0, 0x7000dec13a> as *const (), 0), // +0x700110 sub_700110
    Lifted::new(0x7000dec150, tpl_1398::<0x7000e00f20> as *const (), 0), // +0x700150 sub_700150
    Lifted::new(0x7000dec1a0, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e00f20> as *const (), 0), // +0x7001a0 sub_7001a0
    Lifted::new(0x7000dec2a0, tpl_1397::<0x7000df8ce0, 0x7000dec2ca> as *const (), 0), // +0x7002a0 sub_7002a0
    Lifted::new(0x7000dec2e0, tpl_1398::<0x7000e01040> as *const (), 0), // +0x7002e0 sub_7002e0
    Lifted::new(0x7000dec330, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e01040> as *const (), 0), // +0x700330 sub_700330
    Lifted::new(0x7000dec430, tpl_1397::<0x7000df8d00, 0x7000dec45a> as *const (), 0), // +0x700430 sub_700430
    Lifted::new(0x7000dec470, tpl_1398::<0x7000e01160> as *const (), 0), // +0x700470 sub_700470
    Lifted::new(0x7000dec4c0, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e01160> as *const (), 0), // +0x7004c0 sub_7004c0
    Lifted::new(0x7000dec5c0, tpl_1397::<0x7000df8d20, 0x7000dec5ea> as *const (), 0), // +0x7005c0 sub_7005c0
    Lifted::new(0x7000dec600, tpl_1398::<0x7000e01280> as *const (), 0), // +0x700600 sub_700600
    Lifted::new(0x7000dec650, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e01280> as *const (), 0), // +0x700650 sub_700650
    Lifted::new(0x7000dec750, tpl_1397::<0x7000df8d40, 0x7000dec77a> as *const (), 0), // +0x700750 sub_700750
    Lifted::new(0x7000dec790, tpl_1398::<0x7000e00c70> as *const (), 0), // +0x700790 sub_700790
    Lifted::new(0x7000dec7e0, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e00c70> as *const (), 0), // +0x7007e0 sub_7007e0
    Lifted::new(0x7000dec8e0, tpl_1397::<0x7000df8d60, 0x7000dec90a> as *const (), 0), // +0x7008e0 sub_7008e0
    Lifted::new(0x7000dec920, tpl_1398::<0x7000e013a0> as *const (), 0), // +0x700920 sub_700920
    Lifted::new(0x7000dec970, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e013a0> as *const (), 0), // +0x700970 sub_700970
    Lifted::new(0x7000deca70, tpl_1397::<0x7000df8d80, 0x7000deca9a> as *const (), 0), // +0x700a70 sub_700a70
    Lifted::new(0x7000decab0, tpl_1398::<0x7000e014c0> as *const (), 0), // +0x700ab0 sub_700ab0
    Lifted::new(0x7000decb00, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e014c0> as *const (), 0), // +0x700b00 sub_700b00
    Lifted::new(0x7000decc00, tpl_1397::<0x7000df8da0, 0x7000decc2a> as *const (), 0), // +0x700c00 sub_700c00
    Lifted::new(0x7000decc40, tpl_1398::<0x7000e015e0> as *const (), 0), // +0x700c40 sub_700c40
    Lifted::new(0x7000decc90, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e015e0> as *const (), 0), // +0x700c90 sub_700c90
    Lifted::new(0x7000decd90, tpl_1397::<0x7000df8dc0, 0x7000decdba> as *const (), 0), // +0x700d90 sub_700d90
    Lifted::new(0x7000decdd0, tpl_1398::<0x7000e01700> as *const (), 0), // +0x700dd0 sub_700dd0
    Lifted::new(0x7000dece20, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e01700> as *const (), 0), // +0x700e20 sub_700e20
    Lifted::new(0x7000decf20, tpl_1397::<0x7000df8de0, 0x7000decf4a> as *const (), 0), // +0x700f20 sub_700f20
    Lifted::new(0x7000decf60, tpl_1398::<0x7000e01820> as *const (), 0), // +0x700f60 sub_700f60
    Lifted::new(0x7000decfb0, tpl_1399::<0xffffffffffffffff, 0x7, 0xb, 0x10, 0x7000e01820> as *const (), 0), // +0x700fb0 sub_700fb0
    Lifted::new(0x7000ded0b0, tpl_1397::<0x7000df8e00, 0x7000ded0da> as *const (), 0), // +0x7010b0 sub_7010b0
    Lifted::new(0x7000ded0f0, fn_7000ded0f0 as *const (), 0), // +0x7010f0 sub_7010f0
    Lifted::new(0x7000ded180, fn_7000ded180 as *const (), 0), // +0x701180 sub_701180
    Lifted::new(0x7000ded250, tpl_1397::<0x7000df8e20, 0x7000ded27a> as *const (), 0), // +0x701250 sub_701250
    Lifted::new(0x7000ded290, tpl_1400::<0x7000de8d90, 0x7000e01e10> as *const (), 0), // +0x701290 sub_701290
    Lifted::new(0x7000ded320, tpl_1401::<0xffffffffffffffff, 0x7, 0x10, 0xffffffffffff6898, 0x7000de8d90, 0x7000e01e10> as *const (), 0), // +0x701320 sub_701320
    Lifted::new(0x7000ded3f0, tpl_1397::<0x7000df8e40, 0x7000ded41a> as *const (), 0), // +0x7013f0 sub_7013f0
    Lifted::new(0x7000ded430, tpl_1395::<0x7000e00ac0, 0x7000ded46b> as *const (), 0), // +0x701430 sub_701430
    Lifted::new(0x7000ded480, tpl_1396::<0x7000e40330, 0xffffffffffffffff, 0x7, 0x7000d90750, 0x7000e5ef90, 0x7000d906c0, 0x7000e25820, 0x7000e00ac0, 0x7000ded55d> as *const (), 0), // +0x701480 sub_701480
    Lifted::new(0x7000ded5a0, tpl_1397::<0x7000df8e60, 0x7000ded5ca> as *const (), 0), // +0x7015a0 sub_7015a0
    Lifted::new(0x7000ded5e0, tpl_1400::<0x7000de8d90, 0x7000e01e10> as *const (), 0), // +0x7015e0 sub_7015e0
    Lifted::new(0x7000ded670, tpl_1401::<0xffffffffffffffff, 0x7, 0x10, 0xffffffffffff6898, 0x7000de8d90, 0x7000e01e10> as *const (), 0), // +0x701670 sub_701670
    Lifted::new(0x7000ded740, fn_7000ded740 as *const (), 2), // +0x701740 sub_701740
    Lifted::new(0x7000dedef0, fn_7000dedef0 as *const (), 2), // +0x701ef0 sub_701ef0
    Lifted::new(0x7000dee6a0, fn_7000dee6a0 as *const (), 2), // +0x7026a0 sub_7026a0
    Lifted::new(0x7000deef90, fn_7000deef90 as *const (), 2), // +0x702f90 sub_702f90
    Lifted::new(0x7000def880, fn_7000def880 as *const (), 0), // +0x703880 sub_703880
    Lifted::new(0x7000df0020, fn_7000df0020 as *const (), 0), // +0x704020 sub_704020
    Lifted::new(0x7000df07c0, fn_7000df07c0 as *const (), 2), // +0x7047c0 sub_7047c0
    Lifted::new(0x7000df10b0, fn_7000df10b0 as *const (), 2), // +0x7050b0 sub_7050b0
    Lifted::new(0x7000df1c40, fn_7000df1c40 as *const (), 0), // +0x705c40 sub_705c40
    Lifted::new(0x7000df2470, fn_7000df2470 as *const (), 2), // +0x706470 sub_706470
    Lifted::new(0x7000df2b20, fn_7000df2b20 as *const (), 2), // +0x706b20 sub_706b20
    Lifted::new(0x7000df31d0, fn_7000df31d0 as *const (), 2), // +0x7071d0 sub_7071d0
    Lifted::new(0x7000df3980, fn_7000df3980 as *const (), 2), // +0x707980 sub_707980
    Lifted::new(0x7000df4130, fn_7000df4130 as *const (), 2), // +0x708130 sub_708130
    Lifted::new(0x7000df4a20, fn_7000df4a20 as *const (), 2), // +0x708a20 sub_708a20
    Lifted::new(0x7000df5310, fn_7000df5310 as *const (), 0), // +0x709310 sub_709310
    Lifted::new(0x7000df5ab0, fn_7000df5ab0 as *const (), 0), // +0x709ab0 sub_709ab0
    Lifted::new(0x7000df6250, fn_7000df6250 as *const (), 2), // +0x70a250 sub_70a250
    Lifted::new(0x7000df6b40, fn_7000df6b40 as *const (), 2), // +0x70ab40 sub_70ab40
    Lifted::new(0x7000df76d0, fn_7000df76d0 as *const (), 0), // +0x70b6d0 sub_70b6d0
    Lifted::new(0x7000df7f00, fn_7000df7f00 as *const (), 2), // +0x70bf00 sub_70bf00
    Lifted::new(0x7000df85b0, fn_7000df85b0 as *const (), 2), // +0x70c5b0 sub_70c5b0
    Lifted::new(0x7000df8c60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8c71> as *const (), 0), // +0x70cc60 sub_70cc60
    Lifted::new(0x7000df8c80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8c91> as *const (), 0), // +0x70cc80 sub_70cc80
    Lifted::new(0x7000df8ca0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8cb1> as *const (), 0), // +0x70cca0 sub_70cca0
    Lifted::new(0x7000df8cc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8cd1> as *const (), 0), // +0x70ccc0 sub_70ccc0
    Lifted::new(0x7000df8ce0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8cf1> as *const (), 0), // +0x70cce0 sub_70cce0
    Lifted::new(0x7000df8d00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8d11> as *const (), 0), // +0x70cd00 sub_70cd00
    Lifted::new(0x7000df8d20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8d31> as *const (), 0), // +0x70cd20 sub_70cd20
    Lifted::new(0x7000df8d40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8d51> as *const (), 0), // +0x70cd40 sub_70cd40
    Lifted::new(0x7000df8d60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8d71> as *const (), 0), // +0x70cd60 sub_70cd60
    Lifted::new(0x7000df8d80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8d91> as *const (), 0), // +0x70cd80 sub_70cd80
    Lifted::new(0x7000df8da0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8db1> as *const (), 0), // +0x70cda0 sub_70cda0
    Lifted::new(0x7000df8dc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8dd1> as *const (), 0), // +0x70cdc0 sub_70cdc0
    Lifted::new(0x7000df8de0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8df1> as *const (), 0), // +0x70cde0 sub_70cde0
    Lifted::new(0x7000df8e00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8e11> as *const (), 0), // +0x70ce00 sub_70ce00
    Lifted::new(0x7000df8e20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8e31> as *const (), 0), // +0x70ce20 sub_70ce20
    Lifted::new(0x7000df8e40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8e51> as *const (), 0), // +0x70ce40 sub_70ce40
    Lifted::new(0x7000df8e60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000df8e71> as *const (), 0), // +0x70ce60 sub_70ce60
    Lifted::new(0x7000df8e80, tpl_1402::<0x7000ded740, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000df8fcc, 0x70017d9fe0, 0x70017d8e50, 0x7000e667f0, 0x7000dfc560> as *const (), 2), // +0x70ce80 sub_70ce80
    Lifted::new(0x7000df9110, tpl_1402::<0x7000dedef0, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000df925c, 0x70017d9fe0, 0x70017d8e50, 0x7000e66c40, 0x7000dfc830> as *const (), 2), // +0x70d110 sub_70d110
    Lifted::new(0x7000df93a0, tpl_1402::<0x7000dee6a0, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000df94ec, 0x70017d9fe0, 0x70017d8e50, 0x7000e67090, 0x7000dfcb00> as *const (), 2), // +0x70d3a0 sub_70d3a0
    Lifted::new(0x7000df9630, tpl_1402::<0x7000deef90, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000df977c, 0x70017d9fe0, 0x70017d8e50, 0x7000e67550, 0x7000dfce20> as *const (), 2), // +0x70d630 sub_70d630
    Lifted::new(0x7000df98c0, tpl_1402::<0x7000def880, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000df9a0c, 0x70017d9fe0, 0x70017d8e50, 0x7000e67a10, 0x7000dfd140> as *const (), 2), // +0x70d8c0 sub_70d8c0
    Lifted::new(0x7000df9b50, tpl_1402::<0x7000df0020, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000df9c9c, 0x70017d9fe0, 0x70017d8e50, 0x7000e67e50, 0x7000dfd430> as *const (), 2), // +0x70db50 sub_70db50
    Lifted::new(0x7000df9de0, tpl_1402::<0x7000df07c0, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000df9f2c, 0x70017d9fe0, 0x70017d8e50, 0x7000e68290, 0x7000dfd720> as *const (), 2), // +0x70dde0 sub_70dde0
    Lifted::new(0x7000dfa070, tpl_1402::<0x7000df10b0, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfa1bc, 0x70017d9fe0, 0x70017d8e50, 0x7000e687c0, 0x7000dfda40> as *const (), 2), // +0x70e070 sub_70e070
    Lifted::new(0x7000dfa300, tpl_1402::<0x7000df1c40, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfa44c, 0x70017d9fe0, 0x70017d8e50, 0x7000e68f90, 0x7000dfdda0> as *const (), 2), // +0x70e300 sub_70e300
    Lifted::new(0x7000dfa590, tpl_1403::<0x7000df2470, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfa6da, 0x7000dfe0b0> as *const (), 0), // +0x70e590 sub_70e590
    Lifted::new(0x7000dfa760, tpl_1403::<0x7000df2b20, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfa8aa, 0x7000dfe390> as *const (), 0), // +0x70e760 sub_70e760
    Lifted::new(0x7000dfa930, tpl_1402::<0x7000df31d0, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfaa7c, 0x70017d9fe0, 0x70017d8e50, 0x7000e69420, 0x7000dfe670> as *const (), 2), // +0x70e930 sub_70e930
    Lifted::new(0x7000dfabc0, tpl_1402::<0x7000df3980, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfad0c, 0x70017d9fe0, 0x70017d8e50, 0x7000e69870, 0x7000dfe940> as *const (), 2), // +0x70ebc0 sub_70ebc0
    Lifted::new(0x7000dfae50, tpl_1402::<0x7000df4130, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfaf9c, 0x70017d9fe0, 0x70017d8e50, 0x7000e69cc0, 0x7000dfec10> as *const (), 2), // +0x70ee50 sub_70ee50
    Lifted::new(0x7000dfb0e0, tpl_1402::<0x7000df4a20, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfb22c, 0x70017d9fe0, 0x70017d8e50, 0x7000e6a180, 0x7000dfef30> as *const (), 2), // +0x70f0e0 sub_70f0e0
    Lifted::new(0x7000dfb370, tpl_1402::<0x7000df5310, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfb4bc, 0x70017d9fe0, 0x70017d8e50, 0x7000e6a640, 0x7000dff250> as *const (), 2), // +0x70f370 sub_70f370
    Lifted::new(0x7000dfb600, tpl_1402::<0x7000df5ab0, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfb74c, 0x70017d9fe0, 0x70017d8e50, 0x7000e6aa80, 0x7000dff540> as *const (), 2), // +0x70f600 sub_70f600
    Lifted::new(0x7000dfb890, tpl_1402::<0x7000df6250, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfb9dc, 0x70017d9fe0, 0x70017d8e50, 0x7000e6aec0, 0x7000dff830> as *const (), 2), // +0x70f890 sub_70f890
    Lifted::new(0x7000dfbb20, tpl_1402::<0x7000df6b40, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfbc6c, 0x70017d9fe0, 0x70017d8e50, 0x7000e6b3f0, 0x7000dffb50> as *const (), 2), // +0x70fb20 sub_70fb20
    Lifted::new(0x7000dfbdb0, tpl_1402::<0x7000df76d0, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfbefc, 0x70017d9fe0, 0x70017d8e50, 0x7000e6bbc0, 0x7000dffeb0> as *const (), 2), // +0x70fdb0 sub_70fdb0
    Lifted::new(0x7000dfc040, tpl_1402::<0x7000df7f00, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfc18c, 0x70017d9fe0, 0x70017d8e50, 0x7000e6c050, 0x7000e001c0> as *const (), 2), // +0x710040 sub_710040
    Lifted::new(0x7000dfc2d0, tpl_1402::<0x7000df85b0, 0x7001855b90, 0x70017a0290, 0x7001d54ef8, 0x7000dfc41c, 0x70017d9fe0, 0x70017d8e50, 0x7000e6c500, 0x7000e004e0> as *const (), 2), // +0x7102d0 sub_7102d0
    Lifted::new(0x7000dfc560, tpl_1404::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfc750, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x710560 sub_710560
    Lifted::new(0x7000dfc830, tpl_1404::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfca20, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x710830 sub_710830
    Lifted::new(0x7000dfcb00, tpl_1405::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dfcccd, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x7000dfcd17, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x710b00 sub_710b00
    Lifted::new(0x7000dfce20, tpl_1405::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dfcfed, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x7000dfd037, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x710e20 sub_710e20
    Lifted::new(0x7000dfd140, tpl_1406::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfd334, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x711140 sub_711140
    Lifted::new(0x7000dfd430, tpl_1406::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfd624, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x711430 sub_711430
    Lifted::new(0x7000dfd720, tpl_1407::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x7000e20420, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfd94d, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x711720 sub_711720
    Lifted::new(0x7000dfda40, tpl_1408::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfdca2, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x711a40 sub_711a40
    Lifted::new(0x7000dfdda0, tpl_1409::<0x7001855d18, 0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x700177c260, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfdfc1, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x711da0 sub_711da0
    Lifted::new(0x7000dfe0b0, tpl_1410::<0x7000710c10, 0x700070f9b0, 0x7000db8eb0, 0x7000db6760, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfe2aa, 0x70007a6400, 0x700079cfd0> as *const (), 0), // +0x7120b0 sub_7120b0
    Lifted::new(0x7000dfe390, tpl_1410::<0x7000710c10, 0x700070f9b0, 0x7000db8eb0, 0x7000757d30, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfe58a, 0x70007a6400, 0x700079cfd0> as *const (), 0), // +0x712390 sub_712390
    Lifted::new(0x7000dfe670, tpl_1404::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfe860, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x712670 sub_712670
    Lifted::new(0x7000dfe940, tpl_1404::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dfeb30, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x712940 sub_712940
    Lifted::new(0x7000dfec10, tpl_1405::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dfeddd, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x7000dfee27, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x712c10 sub_712c10
    Lifted::new(0x7000dfef30, tpl_1405::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000dff0fd, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x7000dff147, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x712f30 sub_712f30
    Lifted::new(0x7000dff250, tpl_1406::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dff444, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x713250 sub_713250
    Lifted::new(0x7000dff540, tpl_1406::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70009e4b20, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dff734, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x713540 sub_713540
    Lifted::new(0x7000dff830, tpl_1407::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x7000e207a0, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dffa5d, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x713830 sub_713830
    Lifted::new(0x7000dffb50, tpl_1408::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000dffdb2, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x713b50 sub_713b50
    Lifted::new(0x7000dffeb0, tpl_1409::<0x7001855d18, 0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x700177c260, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000e000d1, 0x70007a6400, 0x700079cfd0> as *const (), 2), // +0x713eb0 sub_713eb0
    Lifted::new(0x7000e001c0, tpl_1411::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x7000db6760, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000e003f4, 0x70007a6400, 0x700079cfd0> as *const (), 0), // +0x7141c0 sub_7141c0
    Lifted::new(0x7000e004e0, tpl_1411::<0x7000710c10, 0x700070f9b0, 0x7000ed42f0, 0x7000757d30, 0x70017d915c, 0x700071b050, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000e00714, 0x70007a6400, 0x700079cfd0> as *const (), 0), // +0x7144e0 sub_7144e0
    Lifted::new(0x7000e00800, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00811> as *const (), 0), // +0x714800 sub_714800
    Lifted::new(0x7000e00820, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00831> as *const (), 0), // +0x714820 sub_714820
    Lifted::new(0x7000e00840, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00851> as *const (), 0), // +0x714840 sub_714840
    Lifted::new(0x7000e00860, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00871> as *const (), 0), // +0x714860 sub_714860
    Lifted::new(0x7000e00880, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00891> as *const (), 0), // +0x714880 sub_714880
    Lifted::new(0x7000e008a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e008b1> as *const (), 0), // +0x7148a0 sub_7148a0
    Lifted::new(0x7000e008c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e008d1> as *const (), 0), // +0x7148c0 sub_7148c0
    Lifted::new(0x7000e008e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e008f1> as *const (), 0), // +0x7148e0 sub_7148e0
    Lifted::new(0x7000e00900, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00911> as *const (), 0), // +0x714900 sub_714900
    Lifted::new(0x7000e00920, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00931> as *const (), 0), // +0x714920 sub_714920
    Lifted::new(0x7000e00940, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00951> as *const (), 0), // +0x714940 sub_714940
    Lifted::new(0x7000e00960, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00971> as *const (), 0), // +0x714960 sub_714960
    Lifted::new(0x7000e00980, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00991> as *const (), 0), // +0x714980 sub_714980
    Lifted::new(0x7000e009a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e009b1> as *const (), 0), // +0x7149a0 sub_7149a0
    Lifted::new(0x7000e009c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e009d1> as *const (), 0), // +0x7149c0 sub_7149c0
    Lifted::new(0x7000e009e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e009f1> as *const (), 0), // +0x7149e0 sub_7149e0
    Lifted::new(0x7000e00a00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00a11> as *const (), 0), // +0x714a00 sub_714a00
    Lifted::new(0x7000e00a20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00a31> as *const (), 0), // +0x714a20 sub_714a20
    Lifted::new(0x7000e00a40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00a51> as *const (), 0), // +0x714a40 sub_714a40
    Lifted::new(0x7000e00a60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00a71> as *const (), 0), // +0x714a60 sub_714a60
    Lifted::new(0x7000e00a80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00a91> as *const (), 0), // +0x714a80 sub_714a80
    Lifted::new(0x7000e00aa0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00ab1> as *const (), 0), // +0x714aa0 sub_714aa0
    Lifted::new(0x7000e00ac0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e00ad1> as *const (), 0), // +0x714ac0 sub_714ac0
    Lifted::new(0x7000e00ae0, fn_7000e00ae0 as *const (), 0), // +0x714ae0 sub_714ae0
    Lifted::new(0x7000e00c70, fn_7000e00c70 as *const (), 0), // +0x714c70 sub_714c70
    Lifted::new(0x7000e00e00, fn_7000e00e00 as *const (), 0), // +0x714e00 sub_714e00
    Lifted::new(0x7000e00f20, fn_7000e00f20 as *const (), 0), // +0x714f20 sub_714f20
    Lifted::new(0x7000e01040, fn_7000e01040 as *const (), 0), // +0x715040 sub_715040
    Lifted::new(0x7000e01160, fn_7000e01160 as *const (), 0), // +0x715160 sub_715160
    Lifted::new(0x7000e01280, fn_7000e01280 as *const (), 0), // +0x715280 sub_715280
    Lifted::new(0x7000e013a0, fn_7000e013a0 as *const (), 0), // +0x7153a0 sub_7153a0
    Lifted::new(0x7000e014c0, fn_7000e014c0 as *const (), 0), // +0x7154c0 sub_7154c0
    Lifted::new(0x7000e015e0, fn_7000e015e0 as *const (), 0), // +0x7155e0 sub_7155e0
    Lifted::new(0x7000e01700, fn_7000e01700 as *const (), 0), // +0x715700 sub_715700
    Lifted::new(0x7000e01820, fn_7000e01820 as *const (), 0), // +0x715820 sub_715820
    Lifted::new(0x7000e01940, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e01951> as *const (), 0), // +0x715940 sub_715940
    Lifted::new(0x7000e01960, fn_7000e01960 as *const (), 0), // +0x715960 sub_715960
    Lifted::new(0x7000e01c50, fn_7000e01c50 as *const (), 1), // +0x715c50 sub_715c50
    Lifted::new(0x7000e01e10, fn_7000e01e10 as *const (), 0), // +0x715e10 sub_715e10
    Lifted::new(0x7000e01f30, fn_7000e01f30 as *const (), 1), // +0x715f30 sub_715f30
    Lifted::new(0x7000e02040, fn_7000e02040 as *const (), 1), // +0x716040 sub_716040
    Lifted::new(0x7000e021c0, tpl_1412::<0x7000e02a40> as *const (), 0), // +0x7161c0 sub_7161c0
    Lifted::new(0x7000e021e0, tpl_1412::<0x7000e02e90> as *const (), 0), // +0x7161e0 sub_7161e0
    Lifted::new(0x7000e02200, tpl_1412::<0x7000e032e0> as *const (), 0), // +0x716200 sub_716200
    Lifted::new(0x7000e02220, tpl_1412::<0x7000e03830> as *const (), 0), // +0x716220 sub_716220
    Lifted::new(0x7000e02240, tpl_1412::<0x7000e03d80> as *const (), 0), // +0x716240 sub_716240
    Lifted::new(0x7000e02260, tpl_1412::<0x7000e041d0> as *const (), 0), // +0x716260 sub_716260
    Lifted::new(0x7000e02280, tpl_1412::<0x7000e04620> as *const (), 0), // +0x716280 sub_716280
    Lifted::new(0x7000e022a0, tpl_1412::<0x7000e04be0> as *const (), 0), // +0x7162a0 sub_7162a0
    Lifted::new(0x7000e022c0, tpl_1412::<0x7000e05410> as *const (), 0), // +0x7162c0 sub_7162c0
    Lifted::new(0x7000e022e0, tpl_1412::<0x7000e058e0> as *const (), 0), // +0x7162e0 sub_7162e0
    Lifted::new(0x7000e02300, tpl_1412::<0x7000e05c70> as *const (), 0), // +0x716300 sub_716300
    Lifted::new(0x7000e02320, tpl_1412::<0x7000e06000> as *const (), 0), // +0x716320 sub_716320
    Lifted::new(0x7000e02340, tpl_1412::<0x7000e06450> as *const (), 0), // +0x716340 sub_716340
    Lifted::new(0x7000e02360, tpl_1412::<0x7000e068a0> as *const (), 0), // +0x716360 sub_716360
    Lifted::new(0x7000e02380, tpl_1412::<0x7000e06df0> as *const (), 0), // +0x716380 sub_716380
    Lifted::new(0x7000e023a0, tpl_1412::<0x7000e07340> as *const (), 0), // +0x7163a0 sub_7163a0
    Lifted::new(0x7000e023c0, tpl_1412::<0x7000e07790> as *const (), 0), // +0x7163c0 sub_7163c0
    Lifted::new(0x7000e023e0, tpl_1412::<0x7000e07be0> as *const (), 0), // +0x7163e0 sub_7163e0
    Lifted::new(0x7000e02400, tpl_1412::<0x7000e081a0> as *const (), 0), // +0x716400 sub_716400
    Lifted::new(0x7000e02420, tpl_1412::<0x7000e089d0> as *const (), 0), // +0x716420 sub_716420
    Lifted::new(0x7000e02440, tpl_1412::<0x7000e08ea0> as *const (), 0), // +0x716440 sub_716440
    Lifted::new(0x7000e02460, tpl_1412::<0x7000e09230> as *const (), 0), // +0x716460 sub_716460
    Lifted::new(0x7000e02480, tpl_1413::<0x7000e02820, 0x7000e0249d> as *const (), 0), // +0x716480 sub_716480
    Lifted::new(0x7000e024b0, tpl_1413::<0x7000e02840, 0x7000e024cd> as *const (), 0), // +0x7164b0 sub_7164b0
    Lifted::new(0x7000e024e0, tpl_1413::<0x7000e02860, 0x7000e024fd> as *const (), 0), // +0x7164e0 sub_7164e0
    Lifted::new(0x7000e02510, tpl_1413::<0x7000e02880, 0x7000e0252d> as *const (), 0), // +0x716510 sub_716510
    Lifted::new(0x7000e02540, tpl_1413::<0x7000e028a0, 0x7000e0255d> as *const (), 0), // +0x716540 sub_716540
    Lifted::new(0x7000e02570, tpl_1413::<0x7000e028c0, 0x7000e0258d> as *const (), 0), // +0x716570 sub_716570
    Lifted::new(0x7000e025a0, tpl_1413::<0x7000e028e0, 0x7000e025bd> as *const (), 0), // +0x7165a0 sub_7165a0
    Lifted::new(0x7000e025d0, tpl_1413::<0x7000e02900, 0x7000e025ed> as *const (), 0), // +0x7165d0 sub_7165d0
    Lifted::new(0x7000e02600, tpl_1413::<0x7000e02920, 0x7000e0261d> as *const (), 0), // +0x716600 sub_716600
    Lifted::new(0x7000e02630, tpl_1413::<0x7000e02940, 0x7000e0264d> as *const (), 0), // +0x716630 sub_716630
    Lifted::new(0x7000e02660, tpl_1413::<0x7000e02960, 0x7000e0267d> as *const (), 0), // +0x716660 sub_716660
    Lifted::new(0x7000e02690, tpl_1413::<0x7000e02980, 0x7000e026ad> as *const (), 0), // +0x716690 sub_716690
    Lifted::new(0x7000e026c0, tpl_1413::<0x7000e029a0, 0x7000e026dd> as *const (), 0), // +0x7166c0 sub_7166c0
    Lifted::new(0x7000e026f0, tpl_1413::<0x7000e029c0, 0x7000e0270d> as *const (), 0), // +0x7166f0 sub_7166f0
    Lifted::new(0x7000e02720, tpl_1413::<0x7000e029e0, 0x7000e0273d> as *const (), 0), // +0x716720 sub_716720
    Lifted::new(0x7000e02750, tpl_1413::<0x7000e02a00, 0x7000e0276d> as *const (), 0), // +0x716750 sub_716750
    Lifted::new(0x7000e02780, tpl_1413::<0x7000e02a20, 0x7000e0279d> as *const (), 0), // +0x716780 sub_716780
    Lifted::new(0x7000e027b0, fn_7000e027b0 as *const (), 0), // +0x7167b0 sub_7167b0
    Lifted::new(0x7000e02820, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02831> as *const (), 0), // +0x716820 sub_716820
    Lifted::new(0x7000e02840, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02851> as *const (), 0), // +0x716840 sub_716840
    Lifted::new(0x7000e02860, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02871> as *const (), 0), // +0x716860 sub_716860
    Lifted::new(0x7000e02880, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02891> as *const (), 0), // +0x716880 sub_716880
    Lifted::new(0x7000e028a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e028b1> as *const (), 0), // +0x7168a0 sub_7168a0
    Lifted::new(0x7000e028c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e028d1> as *const (), 0), // +0x7168c0 sub_7168c0
    Lifted::new(0x7000e028e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e028f1> as *const (), 0), // +0x7168e0 sub_7168e0
    Lifted::new(0x7000e02900, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02911> as *const (), 0), // +0x716900 sub_716900
    Lifted::new(0x7000e02920, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02931> as *const (), 0), // +0x716920 sub_716920
    Lifted::new(0x7000e02940, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02951> as *const (), 0), // +0x716940 sub_716940
    Lifted::new(0x7000e02960, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02971> as *const (), 0), // +0x716960 sub_716960
    Lifted::new(0x7000e02980, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02991> as *const (), 0), // +0x716980 sub_716980
    Lifted::new(0x7000e029a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e029b1> as *const (), 0), // +0x7169a0 sub_7169a0
    Lifted::new(0x7000e029c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e029d1> as *const (), 0), // +0x7169c0 sub_7169c0
    Lifted::new(0x7000e029e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e029f1> as *const (), 0), // +0x7169e0 sub_7169e0
    Lifted::new(0x7000e02a00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02a11> as *const (), 0), // +0x716a00 sub_716a00
    Lifted::new(0x7000e02a20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e02a31> as *const (), 0), // +0x716a20 sub_716a20
    Lifted::new(0x7000e02a40, fn_7000e02a40 as *const (), 0), // +0x716a40 sub_716a40
    Lifted::new(0x7000e02e90, fn_7000e02e90 as *const (), 0), // +0x716e90 sub_716e90
    Lifted::new(0x7000e032e0, fn_7000e032e0 as *const (), 0), // +0x7172e0 sub_7172e0
    Lifted::new(0x7000e03830, fn_7000e03830 as *const (), 0), // +0x717830 sub_717830
    Lifted::new(0x7000e03d80, fn_7000e03d80 as *const (), 0), // +0x717d80 sub_717d80
    Lifted::new(0x7000e041d0, fn_7000e041d0 as *const (), 0), // +0x7181d0 sub_7181d0
    Lifted::new(0x7000e04620, fn_7000e04620 as *const (), 2), // +0x718620 sub_718620
    Lifted::new(0x7000e04be0, fn_7000e04be0 as *const (), 2), // +0x718be0 sub_718be0
    Lifted::new(0x7000e05410, fn_7000e05410 as *const (), 0), // +0x719410 sub_719410
    Lifted::new(0x7000e058e0, fn_7000e058e0 as *const (), 2), // +0x7198e0 sub_7198e0
    Lifted::new(0x7000e05c70, fn_7000e05c70 as *const (), 2), // +0x719c70 sub_719c70
    Lifted::new(0x7000e06000, fn_7000e06000 as *const (), 0), // +0x71a000 sub_71a000
    Lifted::new(0x7000e06450, fn_7000e06450 as *const (), 0), // +0x71a450 sub_71a450
    Lifted::new(0x7000e068a0, fn_7000e068a0 as *const (), 0), // +0x71a8a0 sub_71a8a0
    Lifted::new(0x7000e06df0, fn_7000e06df0 as *const (), 0), // +0x71adf0 sub_71adf0
    Lifted::new(0x7000e07340, fn_7000e07340 as *const (), 0), // +0x71b340 sub_71b340
    Lifted::new(0x7000e07790, fn_7000e07790 as *const (), 0), // +0x71b790 sub_71b790
    Lifted::new(0x7000e07be0, fn_7000e07be0 as *const (), 2), // +0x71bbe0 sub_71bbe0
    Lifted::new(0x7000e081a0, fn_7000e081a0 as *const (), 2), // +0x71c1a0 sub_71c1a0
    Lifted::new(0x7000e089d0, fn_7000e089d0 as *const (), 0), // +0x71c9d0 sub_71c9d0
    Lifted::new(0x7000e08ea0, fn_7000e08ea0 as *const (), 2), // +0x71cea0 sub_71cea0
    Lifted::new(0x7000e09230, fn_7000e09230 as *const (), 2), // +0x71d230 sub_71d230
    Lifted::new(0x7000e095c0, fn_7000e095c0 as *const (), 0), // +0x71d5c0 sub_71d5c0
    Lifted::new(0x7000e09720, tpl_1414::<0x7000e0c490> as *const (), 0), // +0x71d720 sub_71d720
    Lifted::new(0x7000e09760, tpl_1414::<0x7000e0c6b0> as *const (), 0), // +0x71d760 sub_71d760
    Lifted::new(0x7000e097a0, tpl_1414::<0x7000e0c8e0> as *const (), 0), // +0x71d7a0 sub_71d7a0
    Lifted::new(0x7000e097e0, tpl_1414::<0x7000e0cb10> as *const (), 0), // +0x71d7e0 sub_71d7e0
    Lifted::new(0x7000e09820, tpl_1414::<0x7000e0cd40> as *const (), 0), // +0x71d820 sub_71d820
    Lifted::new(0x7000e09860, tpl_1414::<0x7000e0cf70> as *const (), 0), // +0x71d860 sub_71d860
    Lifted::new(0x7000e098a0, tpl_1414::<0x7000e0d1a0> as *const (), 0), // +0x71d8a0 sub_71d8a0
    Lifted::new(0x7000e098e0, tpl_1414::<0x7000e0d3f0> as *const (), 0), // +0x71d8e0 sub_71d8e0
    Lifted::new(0x7000e09920, tpl_1414::<0x7000e0d640> as *const (), 0), // +0x71d920 sub_71d920
    Lifted::new(0x7000e09960, tpl_1414::<0x7000e0d860> as *const (), 0), // +0x71d960 sub_71d960
    Lifted::new(0x7000e099a0, tpl_1414::<0x7000e0daa0> as *const (), 0), // +0x71d9a0 sub_71d9a0
    Lifted::new(0x7000e099e0, tpl_1414::<0x7000e0dce0> as *const (), 0), // +0x71d9e0 sub_71d9e0
    Lifted::new(0x7000e09a20, tpl_1414::<0x7000e0df00> as *const (), 0), // +0x71da20 sub_71da20
    Lifted::new(0x7000e09a60, tpl_1414::<0x7000e0e130> as *const (), 0), // +0x71da60 sub_71da60
    Lifted::new(0x7000e09aa0, tpl_1414::<0x7000e0e360> as *const (), 0), // +0x71daa0 sub_71daa0
    Lifted::new(0x7000e09ae0, tpl_1414::<0x7000e0e590> as *const (), 0), // +0x71dae0 sub_71dae0
    Lifted::new(0x7000e09b20, tpl_1414::<0x7000e0e7c0> as *const (), 0), // +0x71db20 sub_71db20
    Lifted::new(0x7000e09b60, tpl_1414::<0x7000e0e9f0> as *const (), 0), // +0x71db60 sub_71db60
    Lifted::new(0x7000e09ba0, tpl_1414::<0x7000e0ec40> as *const (), 0), // +0x71dba0 sub_71dba0
    Lifted::new(0x7000e09be0, tpl_1414::<0x7000e0ee90> as *const (), 0), // +0x71dbe0 sub_71dbe0
    Lifted::new(0x7000e09c20, tpl_1414::<0x7000e0f0b0> as *const (), 0), // +0x71dc20 sub_71dc20
    Lifted::new(0x7000e09c60, tpl_1414::<0x7000e0f2f0> as *const (), 0), // +0x71dc60 sub_71dc60
    Lifted::new(0x7000e09ca0, tpl_1415::<0x7000e0a090, 0x7000e09cbd> as *const (), 0), // +0x71dca0 sub_71dca0
    Lifted::new(0x7000e09cd0, tpl_1414::<0x7000e0a130> as *const (), 0), // +0x71dcd0 sub_71dcd0
    Lifted::new(0x7000e09d10, tpl_1414::<0x7000e0a430> as *const (), 0), // +0x71dd10 sub_71dd10
    Lifted::new(0x7000e09d50, tpl_1414::<0x7000e0a720> as *const (), 0), // +0x71dd50 sub_71dd50
    Lifted::new(0x7000e09d90, tpl_1414::<0x7000e0aa10> as *const (), 0), // +0x71dd90 sub_71dd90
    Lifted::new(0x7000e09dd0, tpl_1414::<0x7000e0ad00> as *const (), 0), // +0x71ddd0 sub_71ddd0
    Lifted::new(0x7000e09e10, tpl_1414::<0x7000e0aff0> as *const (), 0), // +0x71de10 sub_71de10
    Lifted::new(0x7000e09e50, tpl_1414::<0x7000e0b2e0> as *const (), 0), // +0x71de50 sub_71de50
    Lifted::new(0x7000e09e90, tpl_1414::<0x7000e0b5e0> as *const (), 0), // +0x71de90 sub_71de90
    Lifted::new(0x7000e09ed0, tpl_1414::<0x7000e0b8d0> as *const (), 0), // +0x71ded0 sub_71ded0
    Lifted::new(0x7000e09f10, tpl_1414::<0x7000e0bbc0> as *const (), 0), // +0x71df10 sub_71df10
    Lifted::new(0x7000e09f50, tpl_1414::<0x7000e0beb0> as *const (), 0), // +0x71df50 sub_71df50
    Lifted::new(0x7000e09f90, tpl_1414::<0x7000e0c1a0> as *const (), 0), // +0x71df90 sub_71df90
    Lifted::new(0x7000e09fd0, tpl_1415::<0x7000e0a0b0, 0x7000e09fed> as *const (), 0), // +0x71dfd0 sub_71dfd0
    Lifted::new(0x7000e0a000, tpl_1415::<0x7000e0a0d0, 0x7000e0a01d> as *const (), 0), // +0x71e000 sub_71e000
    Lifted::new(0x7000e0a030, tpl_1415::<0x7000e0a0f0, 0x7000e0a04d> as *const (), 0), // +0x71e030 sub_71e030
    Lifted::new(0x7000e0a060, tpl_1415::<0x7000e0a110, 0x7000e0a07d> as *const (), 0), // +0x71e060 sub_71e060
    Lifted::new(0x7000e0a090, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e0a0a1> as *const (), 0), // +0x71e090 sub_71e090
    Lifted::new(0x7000e0a0b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e0a0c1> as *const (), 0), // +0x71e0b0 sub_71e0b0
    Lifted::new(0x7000e0a0d0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e0a0e1> as *const (), 0), // +0x71e0d0 sub_71e0d0
    Lifted::new(0x7000e0a0f0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e0a101> as *const (), 0), // +0x71e0f0 sub_71e0f0
    Lifted::new(0x7000e0a110, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e0a121> as *const (), 0), // +0x71e110 sub_71e110
    Lifted::new(0x7000e0a130, tpl_1416::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x70009c9cb0, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71e130 sub_71e130
    Lifted::new(0x7000e0a430, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71e430 sub_71e430
    Lifted::new(0x7000e0a720, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71e720 sub_71e720
    Lifted::new(0x7000e0aa10, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71ea10 sub_71ea10
    Lifted::new(0x7000e0ad00, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71ed00 sub_71ed00
    Lifted::new(0x7000e0aff0, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71eff0 sub_71eff0
    Lifted::new(0x7000e0b2e0, tpl_1416::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x70009c9cb0, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71f2e0 sub_71f2e0
    Lifted::new(0x7000e0b5e0, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71f5e0 sub_71f5e0
    Lifted::new(0x7000e0b8d0, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71f8d0 sub_71f8d0
    Lifted::new(0x7000e0bbc0, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71fbc0 sub_71fbc0
    Lifted::new(0x7000e0beb0, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x71feb0 sub_71feb0
    Lifted::new(0x7000e0c1a0, tpl_1417::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000764750, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x7201a0 sub_7201a0
    Lifted::new(0x7000e0c490, tpl_1418::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x720490 sub_720490
    Lifted::new(0x7000e0c6b0, tpl_1419::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x7206b0 sub_7206b0
    Lifted::new(0x7000e0c8e0, tpl_1420::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e0ca9a> as *const (), 2), // +0x7208e0 sub_7208e0
    Lifted::new(0x7000e0cb10, tpl_1421::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e0cccb> as *const (), 2), // +0x720b10 sub_720b10
    Lifted::new(0x7000e0cd40, tpl_1422::<0x700076df00, 0x7000a9ecd0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x720d40 sub_720d40
    Lifted::new(0x7000e0cf70, tpl_1422::<0x700076df00, 0x70009c9de0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x720f70 sub_720f70
    Lifted::new(0x7000e0d1a0, tpl_1423::<0x700076df00, 0x70009c9cb0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x7211a0 sub_7211a0
    Lifted::new(0x7000e0d3f0, tpl_1424::<0x700076df00, 0x70009c9cb0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x7213f0 sub_7213f0
    Lifted::new(0x7000e0d640, tpl_1418::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x721640 sub_721640
    Lifted::new(0x7000e0d860, tpl_1425::<0x700076df00, 0x7000db91c0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x721860 sub_721860
    Lifted::new(0x7000e0daa0, tpl_1425::<0x700076df00, 0x700077c7f0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x721aa0 sub_721aa0
    Lifted::new(0x7000e0dce0, tpl_1418::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x721ce0 sub_721ce0
    Lifted::new(0x7000e0df00, tpl_1419::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x721f00 sub_721f00
    Lifted::new(0x7000e0e130, tpl_1420::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e0e2ea> as *const (), 2), // +0x722130 sub_722130
    Lifted::new(0x7000e0e360, tpl_1421::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e0e51b> as *const (), 2), // +0x722360 sub_722360
    Lifted::new(0x7000e0e590, tpl_1422::<0x700076df00, 0x7000a9ecd0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x722590 sub_722590
    Lifted::new(0x7000e0e7c0, tpl_1422::<0x700076df00, 0x70009c9de0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x7227c0 sub_7227c0
    Lifted::new(0x7000e0e9f0, tpl_1423::<0x700076df00, 0x70009c9cb0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x7229f0 sub_7229f0
    Lifted::new(0x7000e0ec40, tpl_1424::<0x700076df00, 0x70009c9cb0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x722c40 sub_722c40
    Lifted::new(0x7000e0ee90, tpl_1418::<0x700076df00, 0x70006f2a70, 0x70006f9a60, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x722e90 sub_722e90
    Lifted::new(0x7000e0f0b0, tpl_1425::<0x700076df00, 0x7000db91c0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x7230b0 sub_7230b0
    Lifted::new(0x7000e0f2f0, tpl_1425::<0x700076df00, 0x700077c7f0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x7232f0 sub_7232f0
    Lifted::new(0x7000e0f530, tpl_15 as *const (), 1), // +0x723530 sub_723530
    Lifted::new(0x7000e0f540, tpl_15 as *const (), 1), // +0x723540 sub_723540
    Lifted::new(0x7000e0f550, tpl_15 as *const (), 1), // +0x723550 sub_723550
    Lifted::new(0x7000e0f560, tpl_15 as *const (), 1), // +0x723560 sub_723560
    Lifted::new(0x7000e0f570, tpl_15 as *const (), 1), // +0x723570 sub_723570
    Lifted::new(0x7000e0f580, tpl_15 as *const (), 1), // +0x723580 sub_723580
    Lifted::new(0x7000e0f590, tpl_15 as *const (), 1), // +0x723590 sub_723590
    Lifted::new(0x7000e0f5a0, tpl_15 as *const (), 1), // +0x7235a0 sub_7235a0
    Lifted::new(0x7000e0f5b0, tpl_15 as *const (), 1), // +0x7235b0 sub_7235b0
    Lifted::new(0x7000e0f5c0, tpl_15 as *const (), 1), // +0x7235c0 sub_7235c0
    Lifted::new(0x7000e0f5d0, tpl_15 as *const (), 1), // +0x7235d0 sub_7235d0
    Lifted::new(0x7000e0f5e0, tpl_15 as *const (), 1), // +0x7235e0 sub_7235e0
    Lifted::new(0x7000e0f5f0, tpl_15 as *const (), 1), // +0x7235f0 sub_7235f0
    Lifted::new(0x7000e0f600, tpl_15 as *const (), 1), // +0x723600 sub_723600
    Lifted::new(0x7000e0f610, tpl_15 as *const (), 1), // +0x723610 sub_723610
    Lifted::new(0x7000e0f620, tpl_15 as *const (), 1), // +0x723620 sub_723620
    Lifted::new(0x7000e0f630, tpl_15 as *const (), 1), // +0x723630 sub_723630
    Lifted::new(0x7000e0f640, tpl_15 as *const (), 1), // +0x723640 sub_723640
    Lifted::new(0x7000e0f650, tpl_15 as *const (), 1), // +0x723650 sub_723650
    Lifted::new(0x7000e0f660, tpl_15 as *const (), 1), // +0x723660 sub_723660
    Lifted::new(0x7000e0f670, tpl_15 as *const (), 1), // +0x723670 sub_723670
    Lifted::new(0x7000e0f680, tpl_15 as *const (), 1), // +0x723680 sub_723680
    Lifted::new(0x7000e0f690, tpl_441::<0x7000e12a20> as *const (), 0), // +0x723690 sub_723690
    Lifted::new(0x7000e0f6a0, fn_7000e0f6a0 as *const (), 0), // +0x7236a0 sub_7236a0
    Lifted::new(0x7000e0f770, tpl_1426::<0x7000e125d0, 0x7000e0f77f> as *const (), 0), // +0x723770 sub_723770
    Lifted::new(0x7000e0f790, tpl_1427::<0x7000e84980, 0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e11040> as *const (), 0), // +0x723790 sub_723790
    Lifted::new(0x7000e0f860, tpl_441::<0x7000e12500> as *const (), 0), // +0x723860 sub_723860
    Lifted::new(0x7000e0f870, tpl_1426::<0x7000e12610, 0x7000e0f87f> as *const (), 0), // +0x723870 sub_723870
    Lifted::new(0x7000e0f890, tpl_1427::<0x7000e84980, 0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e11640> as *const (), 0), // +0x723890 sub_723890
    Lifted::new(0x7000e0f960, fn_7000e0f960 as *const (), 0), // +0x723960 sub_723960
    Lifted::new(0x7000e0fa40, tpl_1426::<0x7000e125f0, 0x7000e0fa4f> as *const (), 0), // +0x723a40 sub_723a40
    Lifted::new(0x7000e0fa60, fn_7000e0fa60 as *const (), 0), // +0x723a60 sub_723a60
    Lifted::new(0x7000e0fb40, fn_7000e0fb40 as *const (), 0), // +0x723b40 sub_723b40
    Lifted::new(0x7000e0fc20, tpl_1426::<0x7000e12630, 0x7000e0fc2f> as *const (), 0), // +0x723c20 sub_723c20
    Lifted::new(0x7000e0fc40, fn_7000e0fc40 as *const (), 0), // +0x723c40 sub_723c40
    Lifted::new(0x7000e0fd20, tpl_441::<0x7000e12650> as *const (), 0), // +0x723d20 sub_723d20
    Lifted::new(0x7000e0fd30, tpl_1428::<0xb, 0x7, 0x7000e12500> as *const (), 0), // +0x723d30 sub_723d30
    Lifted::new(0x7000e0fd70, tpl_441::<0x7000e12840> as *const (), 0), // +0x723d70 sub_723d70
    Lifted::new(0x7000e0fd80, tpl_1428::<0xb, 0x7, 0x7000e12a20> as *const (), 0), // +0x723d80 sub_723d80
    Lifted::new(0x7000e0fdc0, tpl_1429::<0x7000767d50, 0x7000748180, 0x700074a010> as *const (), 0), // +0x723dc0 sub_723dc0
    Lifted::new(0x7000e0ffa0, tpl_1430::<0x7000767d50, 0x7000748180, 0x700074a010> as *const (), 0), // +0x723fa0 sub_723fa0
    Lifted::new(0x7000e101a0, tpl_1430::<0x7000767d50, 0x7000748180, 0x700074a010> as *const (), 0), // +0x7241a0 sub_7241a0
    Lifted::new(0x7000e103a0, tpl_1430::<0x7000767d50, 0x7000748180, 0x700074a010> as *const (), 0), // +0x7243a0 sub_7243a0
    Lifted::new(0x7000e105a0, tpl_1429::<0x7000767d50, 0x7000748180, 0x700074a010> as *const (), 0), // +0x7245a0 sub_7245a0
    Lifted::new(0x7000e10780, tpl_1430::<0x7000767d50, 0x7000748180, 0x700074a010> as *const (), 0), // +0x724780 sub_724780
    Lifted::new(0x7000e10980, tpl_1430::<0x7000767d50, 0x7000748180, 0x700074a010> as *const (), 0), // +0x724980 sub_724980
    Lifted::new(0x7000e10b80, tpl_1430::<0x7000767d50, 0x7000748180, 0x700074a010> as *const (), 0), // +0x724b80 sub_724b80
    Lifted::new(0x7000e10d80, tpl_1431::<0x7000e0fdc0, 0x70017d8e30, 0x7001d55098, 0x7000721d60> as *const (), 2), // +0x724d80 sub_724d80
    Lifted::new(0x7000e11040, tpl_1432::<0x7000e0ffa0, 0x70017d8e30, 0x7001d55098, 0x7000721d60> as *const (), 2), // +0x725040 sub_725040
    Lifted::new(0x7000e11340, tpl_1432::<0x7000e101a0, 0x70017d8e30, 0x7001d55098, 0x7000721d60> as *const (), 2), // +0x725340 sub_725340
    Lifted::new(0x7000e11640, tpl_1432::<0x7000e103a0, 0x70017d8e30, 0x7001d55098, 0x7000721d60> as *const (), 2), // +0x725640 sub_725640
    Lifted::new(0x7000e11940, tpl_1431::<0x7000e105a0, 0x70017d8e30, 0x7001d55098, 0x7000721d60> as *const (), 2), // +0x725940 sub_725940
    Lifted::new(0x7000e11c00, tpl_1432::<0x7000e10780, 0x70017d8e30, 0x7001d55098, 0x7000721d60> as *const (), 2), // +0x725c00 sub_725c00
    Lifted::new(0x7000e11f00, tpl_1432::<0x7000e10980, 0x70017d8e30, 0x7001d55098, 0x7000721d60> as *const (), 2), // +0x725f00 sub_725f00
    Lifted::new(0x7000e12200, tpl_1432::<0x7000e10b80, 0x70017d8e30, 0x7001d55098, 0x7000721d60> as *const (), 2), // +0x726200 sub_726200
    Lifted::new(0x7000e12500, fn_7000e12500 as *const (), 0), // +0x726500 sub_726500
    Lifted::new(0x7000e125d0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e125e1> as *const (), 0), // +0x7265d0 sub_7265d0
    Lifted::new(0x7000e125f0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e12601> as *const (), 0), // +0x7265f0 sub_7265f0
    Lifted::new(0x7000e12610, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e12621> as *const (), 0), // +0x726610 sub_726610
    Lifted::new(0x7000e12630, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e12641> as *const (), 0), // +0x726630 sub_726630
    Lifted::new(0x7000e12650, fn_7000e12650 as *const (), 0), // +0x726650 sub_726650
    Lifted::new(0x7000e12840, fn_7000e12840 as *const (), 0), // +0x726840 sub_726840
    Lifted::new(0x7000e12a20, fn_7000e12a20 as *const (), 0), // +0x726a20 sub_726a20
    Lifted::new(0x7000e12ba0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x726ba0 sub_726ba0
    Lifted::new(0x7000e12f60, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x726f60 sub_726f60
    Lifted::new(0x7000e13320, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x727320 sub_727320
    Lifted::new(0x7000e136e0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x7276e0 sub_7276e0
    Lifted::new(0x7000e13aa0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x727aa0 sub_727aa0
    Lifted::new(0x7000e13e60, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x727e60 sub_727e60
    Lifted::new(0x7000e14220, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x728220 sub_728220
    Lifted::new(0x7000e145e0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x7285e0 sub_7285e0
    Lifted::new(0x7000e149a0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x7289a0 sub_7289a0
    Lifted::new(0x7000e14d60, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x728d60 sub_728d60
    Lifted::new(0x7000e15120, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x729120 sub_729120
    Lifted::new(0x7000e154e0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x7294e0 sub_7294e0
    Lifted::new(0x7000e158a0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x7298a0 sub_7298a0
    Lifted::new(0x7000e15c60, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x729c60 sub_729c60
    Lifted::new(0x7000e16020, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x72a020 sub_72a020
    Lifted::new(0x7000e163e0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x72a3e0 sub_72a3e0
    Lifted::new(0x7000e167a0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x72a7a0 sub_72a7a0
    Lifted::new(0x7000e16b60, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x72ab60 sub_72ab60
    Lifted::new(0x7000e16f20, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x72af20 sub_72af20
    Lifted::new(0x7000e172e0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x72b2e0 sub_72b2e0
    Lifted::new(0x7000e176a0, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x72b6a0 sub_72b6a0
    Lifted::new(0x7000e17a60, tpl_1433::<0x7000aeb7a0, 0x700072d6b0, 0x7000767d50, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x700074a010> as *const (), 2), // +0x72ba60 sub_72ba60
    Lifted::new(0x7000e17e20, fn_7000e17e20 as *const (), 2), // +0x72be20 sub_72be20
    Lifted::new(0x7000e18240, tpl_1434::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72c240 sub_72c240
    Lifted::new(0x7000e18650, tpl_1434::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72c650 sub_72c650
    Lifted::new(0x7000e18a60, tpl_1434::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72ca60 sub_72ca60
    Lifted::new(0x7000e18e70, tpl_1434::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72ce70 sub_72ce70
    Lifted::new(0x7000e19280, tpl_1434::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72d280 sub_72d280
    Lifted::new(0x7000e19690, tpl_1435::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72d690 sub_72d690
    Lifted::new(0x7000e19a80, tpl_1435::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72da80 sub_72da80
    Lifted::new(0x7000e19e70, tpl_1435::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72de70 sub_72de70
    Lifted::new(0x7000e1a260, tpl_1435::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72e260 sub_72e260
    Lifted::new(0x7000e1a650, tpl_1435::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72e650 sub_72e650
    Lifted::new(0x7000e1aa40, tpl_1435::<0x700072d6b0, 0x70006f2a70, 0x70006f9a60, 0x700077d9a0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x72ea40 sub_72ea40
    Lifted::new(0x7000e1ae30, fn_7000e1ae30 as *const (), 2), // +0x72ee30 sub_72ee30
    Lifted::new(0x7000e1b2d0, fn_7000e1b2d0 as *const (), 2), // +0x72f2d0 sub_72f2d0
    Lifted::new(0x7000e1b750, tpl_1436::<0x700072d6b0, 0x7000767d50, 0x700074a010, 0x70006f2a70, 0x7000e970f0, 0x70006f9a60, 0x7000e18a60> as *const (), 2), // +0x72f750 sub_72f750
    Lifted::new(0x7000e1ba40, tpl_1436::<0x700072d6b0, 0x7000767d50, 0x700074a010, 0x70006f2a70, 0x7000e970f0, 0x70006f9a60, 0x7000e1bd30> as *const (), 2), // +0x72fa40 sub_72fa40
    Lifted::new(0x7000e1bd30, fn_7000e1bd30 as *const (), 2), // +0x72fd30 sub_72fd30
    Lifted::new(0x7000e1bfd0, tpl_1437::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0x2f, 0x27, 0xb, 0x2b, 0x700177c0e0> as *const (), 2), // +0x72ffd0 sub_72ffd0
    Lifted::new(0x7000e1c0c0, tpl_1437::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0x2f, 0x27, 0xb, 0x2b, 0x700177c0e0> as *const (), 2), // +0x7300c0 sub_7300c0
    Lifted::new(0x7000e1c1b0, tpl_1438::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0xb, 0x2f, 0x27, 0x2b, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e1c26b, 0x7000dc9a40> as *const (), 2), // +0x7301b0 sub_7301b0
    Lifted::new(0x7000e1c2d0, tpl_1438::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0xb, 0x2f, 0x27, 0x2b, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e1c38b, 0x7000dc9970> as *const (), 2), // +0x7302d0 sub_7302d0
    Lifted::new(0x7000e1c3f0, tpl_1439::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0xb, 0x2f, 0x27, 0x2b, 0x7000dc9b10> as *const (), 2), // +0x7303f0 sub_7303f0
    Lifted::new(0x7000e1c4f0, tpl_1439::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0xb, 0x2f, 0x27, 0x2b, 0x7000b930b0> as *const (), 2), // +0x7304f0 sub_7304f0
    Lifted::new(0x7000e1c5f0, tpl_1440::<0x7000e20420, 0x7000dc9be0> as *const (), 2), // +0x7305f0 sub_7305f0
    Lifted::new(0x7000e1c710, tpl_1441 as *const (), 3), // +0x730710 sub_730710
    Lifted::new(0x7000e1c860, tpl_1394::<0x7000e1fe70> as *const (), 0), // +0x730860 sub_730860
    Lifted::new(0x7000e1c8b0, tpl_1442::<0x7000db6760> as *const (), 0), // +0x7308b0 sub_7308b0
    Lifted::new(0x7000e1c9d0, tpl_1442::<0x7000757d30> as *const (), 0), // +0x7309d0 sub_7309d0
    Lifted::new(0x7000e1caf0, tpl_1437::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0x2f, 0x27, 0xb, 0x2b, 0x700177c0e0> as *const (), 2), // +0x730af0 sub_730af0
    Lifted::new(0x7000e1cbe0, tpl_1437::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0x2f, 0x27, 0xb, 0x2b, 0x700177c0e0> as *const (), 2), // +0x730be0 sub_730be0
    Lifted::new(0x7000e1ccd0, tpl_1438::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0xb, 0x2f, 0x27, 0x2b, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e1cd8b, 0x7000dc9a40> as *const (), 2), // +0x730cd0 sub_730cd0
    Lifted::new(0x7000e1cdf0, tpl_1438::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0xb, 0x2f, 0x27, 0x2b, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e1ceab, 0x7000dc9970> as *const (), 2), // +0x730df0 sub_730df0
    Lifted::new(0x7000e1cf10, tpl_1439::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0xb, 0x2f, 0x27, 0x2b, 0x7000dc9b10> as *const (), 2), // +0x730f10 sub_730f10
    Lifted::new(0x7000e1d010, tpl_1439::<0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0xb, 0x2f, 0x27, 0x2b, 0x7000b930b0> as *const (), 2), // +0x731010 sub_731010
    Lifted::new(0x7000e1d110, tpl_1440::<0x7000e207a0, 0x7000dc9be0> as *const (), 2), // +0x731110 sub_731110
    Lifted::new(0x7000e1d230, tpl_1441 as *const (), 3), // +0x731230 sub_731230
    Lifted::new(0x7000e1d380, tpl_1394::<0x7000e1ffb0> as *const (), 0), // +0x731380 sub_731380
    Lifted::new(0x7000e1d3d0, tpl_1442::<0x7000db6760> as *const (), 0), // +0x7313d0 sub_7313d0
    Lifted::new(0x7000e1d4f0, tpl_1442::<0x7000757d30> as *const (), 0), // +0x7314f0 sub_7314f0
    Lifted::new(0x7000e1d610, tpl_1397::<0x7000e1db10, 0x7000e1d63a> as *const (), 0), // +0x731610 sub_731610
    Lifted::new(0x7000e1d650, tpl_1394::<0x7000e1dbb0> as *const (), 0), // +0x731650 sub_731650
    Lifted::new(0x7000e1d6a0, tpl_1394::<0x7000e1de50> as *const (), 0), // +0x7316a0 sub_7316a0
    Lifted::new(0x7000e1d6f0, tpl_1394::<0x7000e1e0a0> as *const (), 0), // +0x7316f0 sub_7316f0
    Lifted::new(0x7000e1d740, tpl_1394::<0x7000e1e3b0> as *const (), 0), // +0x731740 sub_731740
    Lifted::new(0x7000e1d790, tpl_1394::<0x7000e1e620> as *const (), 0), // +0x731790 sub_731790
    Lifted::new(0x7000e1d7e0, tpl_1394::<0x7000e1e930> as *const (), 0), // +0x7317e0 sub_7317e0
    Lifted::new(0x7000e1d830, tpl_1394::<0x7000e1ec50> as *const (), 0), // +0x731830 sub_731830
    Lifted::new(0x7000e1d880, tpl_1394::<0x7000e1ef20> as *const (), 0), // +0x731880 sub_731880
    Lifted::new(0x7000e1d8d0, tpl_1394::<0x7000e1f170> as *const (), 0), // +0x7318d0 sub_7318d0
    Lifted::new(0x7000e1d920, tpl_1394::<0x7000e1f4c0> as *const (), 0), // +0x731920 sub_731920
    Lifted::new(0x7000e1d970, tpl_1394::<0x7000e1f810> as *const (), 0), // +0x731970 sub_731970
    Lifted::new(0x7000e1d9c0, tpl_1394::<0x7000e1fb20> as *const (), 0), // +0x7319c0 sub_7319c0
    Lifted::new(0x7000e1da10, tpl_1397::<0x7000e1db30, 0x7000e1da3a> as *const (), 0), // +0x731a10 sub_731a10
    Lifted::new(0x7000e1da50, tpl_1397::<0x7000e1db50, 0x7000e1da7a> as *const (), 0), // +0x731a50 sub_731a50
    Lifted::new(0x7000e1da90, tpl_1397::<0x7000e1db70, 0x7000e1daba> as *const (), 0), // +0x731a90 sub_731a90
    Lifted::new(0x7000e1dad0, tpl_1397::<0x7000e1db90, 0x7000e1dafa> as *const (), 0), // +0x731ad0 sub_731ad0
    Lifted::new(0x7000e1db10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e1db21> as *const (), 0), // +0x731b10 sub_731b10
    Lifted::new(0x7000e1db30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e1db41> as *const (), 0), // +0x731b30 sub_731b30
    Lifted::new(0x7000e1db50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e1db61> as *const (), 0), // +0x731b50 sub_731b50
    Lifted::new(0x7000e1db70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e1db81> as *const (), 0), // +0x731b70 sub_731b70
    Lifted::new(0x7000e1db90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e1dba1> as *const (), 0), // +0x731b90 sub_731b90
    Lifted::new(0x7000e1dbb0, fn_7000e1dbb0 as *const (), 2), // +0x731bb0 sub_731bb0
    Lifted::new(0x7000e1de50, fn_7000e1de50 as *const (), 0), // +0x731e50 sub_731e50
    Lifted::new(0x7000e1e0a0, fn_7000e1e0a0 as *const (), 2), // +0x7320a0 sub_7320a0
    Lifted::new(0x7000e1e3b0, fn_7000e1e3b0 as *const (), 0), // +0x7323b0 sub_7323b0
    Lifted::new(0x7000e1e620, fn_7000e1e620 as *const (), 2), // +0x732620 sub_732620
    Lifted::new(0x7000e1e930, fn_7000e1e930 as *const (), 2), // +0x732930 sub_732930
    Lifted::new(0x7000e1ec50, fn_7000e1ec50 as *const (), 2), // +0x732c50 sub_732c50
    Lifted::new(0x7000e1ef20, fn_7000e1ef20 as *const (), 0), // +0x732f20 sub_732f20
    Lifted::new(0x7000e1f170, fn_7000e1f170 as *const (), 2), // +0x733170 sub_733170
    Lifted::new(0x7000e1f4c0, fn_7000e1f4c0 as *const (), 2), // +0x7334c0 sub_7334c0
    Lifted::new(0x7000e1f810, fn_7000e1f810 as *const (), 2), // +0x733810 sub_733810
    Lifted::new(0x7000e1fb20, fn_7000e1fb20 as *const (), 2), // +0x733b20 sub_733b20
    Lifted::new(0x7000e1fe70, tpl_1443::<0x7001855d18, 0x700177c260, 0x700177c0e0> as *const (), 2), // +0x733e70 sub_733e70
    Lifted::new(0x7000e1ffb0, tpl_1443::<0x7001855d18, 0x700177c260, 0x700177c0e0> as *const (), 2), // +0x733fb0 sub_733fb0
    Lifted::new(0x7000e200f0, fn_7000e200f0 as *const (), 2), // +0x7340f0 sub_7340f0
    Lifted::new(0x7000e201a0, tpl_1444::<0xffffffffffffffff, 0x7, 0x3, 0x7001855d18, 0x700177c260> as *const (), 2), // +0x7341a0 sub_7341a0
    Lifted::new(0x7000e20220, tpl_1444::<0xffffffffffffffff, 0x7, 0x3, 0x7001855d18, 0x700177c260> as *const (), 2), // +0x734220 sub_734220
    Lifted::new(0x7000e202a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e202b1> as *const (), 0), // +0x7342a0 sub_7342a0
    Lifted::new(0x7000e202c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e202d1> as *const (), 0), // +0x7342c0 sub_7342c0
    Lifted::new(0x7000e202e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e202f1> as *const (), 0), // +0x7342e0 sub_7342e0
    Lifted::new(0x7000e20300, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20311> as *const (), 0), // +0x734300 sub_734300
    Lifted::new(0x7000e20320, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20331> as *const (), 0), // +0x734320 sub_734320
    Lifted::new(0x7000e20340, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20351> as *const (), 0), // +0x734340 sub_734340
    Lifted::new(0x7000e20360, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20371> as *const (), 0), // +0x734360 sub_734360
    Lifted::new(0x7000e20380, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20391> as *const (), 0), // +0x734380 sub_734380
    Lifted::new(0x7000e203a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e203b1> as *const (), 0), // +0x7343a0 sub_7343a0
    Lifted::new(0x7000e203c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e203d1> as *const (), 0), // +0x7343c0 sub_7343c0
    Lifted::new(0x7000e203e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e203f1> as *const (), 0), // +0x7343e0 sub_7343e0
    Lifted::new(0x7000e20400, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20411> as *const (), 0), // +0x734400 sub_734400
    Lifted::new(0x7000e20420, tpl_383::<0x70017d8e60, 0x70017d8e70, 0x70017d8e7c, 0x70017d8e80, 0x70017d8ec8, 0x70017d8ed8, 0x70017d8ee4, 0x70017d8ee8> as *const (), 3), // +0x734420 sub_734420
    Lifted::new(0x7000e204a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e204b1> as *const (), 0), // +0x7344a0 sub_7344a0
    Lifted::new(0x7000e204c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e204d1> as *const (), 0), // +0x7344c0 sub_7344c0
    Lifted::new(0x7000e204e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e204f1> as *const (), 0), // +0x7344e0 sub_7344e0
    Lifted::new(0x7000e20500, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20511> as *const (), 0), // +0x734500 sub_734500
    Lifted::new(0x7000e20520, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20531> as *const (), 0), // +0x734520 sub_734520
    Lifted::new(0x7000e20540, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20551> as *const (), 0), // +0x734540 sub_734540
    Lifted::new(0x7000e20560, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20571> as *const (), 0), // +0x734560 sub_734560
    Lifted::new(0x7000e20580, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20591> as *const (), 0), // +0x734580 sub_734580
    Lifted::new(0x7000e205a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e205b1> as *const (), 0), // +0x7345a0 sub_7345a0
    Lifted::new(0x7000e205c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e205d1> as *const (), 0), // +0x7345c0 sub_7345c0
    Lifted::new(0x7000e205e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e205f1> as *const (), 0), // +0x7345e0 sub_7345e0
    Lifted::new(0x7000e20600, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20611> as *const (), 0), // +0x734600 sub_734600
    Lifted::new(0x7000e20620, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20631> as *const (), 0), // +0x734620 sub_734620
    Lifted::new(0x7000e20640, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20651> as *const (), 0), // +0x734640 sub_734640
    Lifted::new(0x7000e20660, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20671> as *const (), 0), // +0x734660 sub_734660
    Lifted::new(0x7000e20680, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20691> as *const (), 0), // +0x734680 sub_734680
    Lifted::new(0x7000e206a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e206b1> as *const (), 0), // +0x7346a0 sub_7346a0
    Lifted::new(0x7000e206c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e206d1> as *const (), 0), // +0x7346c0 sub_7346c0
    Lifted::new(0x7000e206e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e206f1> as *const (), 0), // +0x7346e0 sub_7346e0
    Lifted::new(0x7000e20700, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20711> as *const (), 0), // +0x734700 sub_734700
    Lifted::new(0x7000e20720, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20731> as *const (), 0), // +0x734720 sub_734720
    Lifted::new(0x7000e20740, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20751> as *const (), 0), // +0x734740 sub_734740
    Lifted::new(0x7000e20760, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20771> as *const (), 0), // +0x734760 sub_734760
    Lifted::new(0x7000e20780, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20791> as *const (), 0), // +0x734780 sub_734780
    Lifted::new(0x7000e207a0, tpl_383::<0x70017d8e60, 0x70017d8e70, 0x70017d8e7c, 0x70017d8e80, 0x70017d8ec8, 0x70017d8ed8, 0x70017d8ee4, 0x70017d8ee8> as *const (), 3), // +0x7347a0 sub_7347a0
    Lifted::new(0x7000e20820, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20831> as *const (), 0), // +0x734820 sub_734820
    Lifted::new(0x7000e20840, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20851> as *const (), 0), // +0x734840 sub_734840
    Lifted::new(0x7000e20860, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20871> as *const (), 0), // +0x734860 sub_734860
    Lifted::new(0x7000e20880, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20891> as *const (), 0), // +0x734880 sub_734880
    Lifted::new(0x7000e208a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e208b1> as *const (), 0), // +0x7348a0 sub_7348a0
    Lifted::new(0x7000e208c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e208d1> as *const (), 0), // +0x7348c0 sub_7348c0
    Lifted::new(0x7000e208e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e208f1> as *const (), 0), // +0x7348e0 sub_7348e0
    Lifted::new(0x7000e20900, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20911> as *const (), 0), // +0x734900 sub_734900
    Lifted::new(0x7000e20920, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20931> as *const (), 0), // +0x734920 sub_734920
    Lifted::new(0x7000e20940, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20951> as *const (), 0), // +0x734940 sub_734940
    Lifted::new(0x7000e20960, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20971> as *const (), 0), // +0x734960 sub_734960
    Lifted::new(0x7000e20980, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e20991> as *const (), 0), // +0x734980 sub_734980
    Lifted::new(0x7000e209a0, tpl_1445::<0x7000e246a0> as *const (), 0), // +0x7349a0 sub_7349a0
    Lifted::new(0x7000e209d0, tpl_1445::<0x7000e24780> as *const (), 0), // +0x7349d0 sub_7349d0
    Lifted::new(0x7000e20a00, tpl_1445::<0x7000e24860> as *const (), 0), // +0x734a00 sub_734a00
    Lifted::new(0x7000e20a30, tpl_1445::<0x7000e24970> as *const (), 0), // +0x734a30 sub_734a30
    Lifted::new(0x7000e20a60, tpl_1445::<0x7000e24a80> as *const (), 0), // +0x734a60 sub_734a60
    Lifted::new(0x7000e20a90, tpl_1445::<0x7000e24b10> as *const (), 0), // +0x734a90 sub_734a90
    Lifted::new(0x7000e20ac0, tpl_1445::<0x7000e24ba0> as *const (), 0), // +0x734ac0 sub_734ac0
    Lifted::new(0x7000e20af0, tpl_1445::<0x7000e24c40> as *const (), 0), // +0x734af0 sub_734af0
    Lifted::new(0x7000e20b20, tpl_1445::<0x7000e24d00> as *const (), 0), // +0x734b20 sub_734b20
    Lifted::new(0x7000e20b50, tpl_1445::<0x7000e24de0> as *const (), 0), // +0x734b50 sub_734b50
    Lifted::new(0x7000e20b80, tpl_1445::<0x7000e24ea0> as *const (), 0), // +0x734b80 sub_734b80
    Lifted::new(0x7000e20bb0, tpl_1445::<0x7000e24f60> as *const (), 0), // +0x734bb0 sub_734bb0
    Lifted::new(0x7000e20be0, tpl_1445::<0x7000e25040> as *const (), 0), // +0x734be0 sub_734be0
    Lifted::new(0x7000e20c10, tpl_1445::<0x7000e25120> as *const (), 0), // +0x734c10 sub_734c10
    Lifted::new(0x7000e20c40, tpl_1445::<0x7000e25230> as *const (), 0), // +0x734c40 sub_734c40
    Lifted::new(0x7000e20c70, tpl_1445::<0x7000e25340> as *const (), 0), // +0x734c70 sub_734c70
    Lifted::new(0x7000e20ca0, tpl_1445::<0x7000e253d0> as *const (), 0), // +0x734ca0 sub_734ca0
    Lifted::new(0x7000e20cd0, tpl_1445::<0x7000e25460> as *const (), 0), // +0x734cd0 sub_734cd0
    Lifted::new(0x7000e20d00, tpl_1445::<0x7000e25500> as *const (), 0), // +0x734d00 sub_734d00
    Lifted::new(0x7000e20d30, tpl_1445::<0x7000e255c0> as *const (), 0), // +0x734d30 sub_734d30
    Lifted::new(0x7000e20d60, tpl_1445::<0x7000e256a0> as *const (), 0), // +0x734d60 sub_734d60
    Lifted::new(0x7000e20d90, tpl_1445::<0x7000e25760> as *const (), 0), // +0x734d90 sub_734d90
    Lifted::new(0x7000e20dc0, tpl_1446::<0x7, 0x10, 0x7000e240f0> as *const (), 0), // +0x734dc0 sub_734dc0
    Lifted::new(0x7000e20e20, tpl_1447::<0x7, 0x10, 0xffffffffffff6940, 0xffffffffffff6898, 0x70009c9cb0> as *const (), 2), // +0x734e20 sub_734e20
    Lifted::new(0x7000e20ec0, tpl_1446::<0x7, 0x10, 0x7000e233e0> as *const (), 0), // +0x734ec0 sub_734ec0
    Lifted::new(0x7000e20f20, tpl_1446::<0x7, 0x10, 0x7000e23490> as *const (), 0), // +0x734f20 sub_734f20
    Lifted::new(0x7000e20f80, tpl_1446::<0x7, 0x10, 0x7000e23540> as *const (), 0), // +0x734f80 sub_734f80
    Lifted::new(0x7000e20fe0, tpl_1446::<0x7, 0x10, 0x7000e235f0> as *const (), 0), // +0x734fe0 sub_734fe0
    Lifted::new(0x7000e21040, tpl_1446::<0x7, 0x10, 0x7000e236a0> as *const (), 0), // +0x735040 sub_735040
    Lifted::new(0x7000e210a0, tpl_1447::<0x7, 0x10, 0xffffffffffff6940, 0xffffffffffff6898, 0x70009c9cb0> as *const (), 2), // +0x7350a0 sub_7350a0
    Lifted::new(0x7000e21140, tpl_1446::<0x7, 0x10, 0x7000e23750> as *const (), 0), // +0x735140 sub_735140
    Lifted::new(0x7000e211a0, tpl_1446::<0x7, 0x10, 0x7000e23800> as *const (), 0), // +0x7351a0 sub_7351a0
    Lifted::new(0x7000e21200, tpl_1446::<0x7, 0x10, 0x7000e238b0> as *const (), 0), // +0x735200 sub_735200
    Lifted::new(0x7000e21260, tpl_1446::<0x7, 0x10, 0x7000e23960> as *const (), 0), // +0x735260 sub_735260
    Lifted::new(0x7000e212c0, tpl_1446::<0x7, 0x10, 0x7000e23a10> as *const (), 0), // +0x7352c0 sub_7352c0
    Lifted::new(0x7000e21320, tpl_1446::<0x7, 0x10, 0x7000e23ac0> as *const (), 0), // +0x735320 sub_735320
    Lifted::new(0x7000e21380, tpl_1445::<0x7000e24320> as *const (), 0), // +0x735380 sub_735380
    Lifted::new(0x7000e213b0, tpl_1446::<0x7, 0x10, 0x7000e23c40> as *const (), 0), // +0x7353b0 sub_7353b0
    Lifted::new(0x7000e21410, tpl_1445::<0x7000e244e0> as *const (), 0), // +0x735410 sub_735410
    Lifted::new(0x7000e21440, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735440 sub_735440
    Lifted::new(0x7000e21470, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735470 sub_735470
    Lifted::new(0x7000e214a0, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x7354a0 sub_7354a0
    Lifted::new(0x7000e214d0, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x7354d0 sub_7354d0
    Lifted::new(0x7000e21500, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735500 sub_735500
    Lifted::new(0x7000e21530, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735530 sub_735530
    Lifted::new(0x7000e21560, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735560 sub_735560
    Lifted::new(0x7000e21590, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735590 sub_735590
    Lifted::new(0x7000e215c0, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x7355c0 sub_7355c0
    Lifted::new(0x7000e215f0, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x7355f0 sub_7355f0
    Lifted::new(0x7000e21620, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735620 sub_735620
    Lifted::new(0x7000e21650, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735650 sub_735650
    Lifted::new(0x7000e21680, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735680 sub_735680
    Lifted::new(0x7000e216b0, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x7356b0 sub_7356b0
    Lifted::new(0x7000e216e0, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x7356e0 sub_7356e0
    Lifted::new(0x7000e21710, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735710 sub_735710
    Lifted::new(0x7000e21740, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735740 sub_735740
    Lifted::new(0x7000e21770, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735770 sub_735770
    Lifted::new(0x7000e217a0, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x7357a0 sub_7357a0
    Lifted::new(0x7000e217d0, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x7357d0 sub_7357d0
    Lifted::new(0x7000e21800, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735800 sub_735800
    Lifted::new(0x7000e21830, tpl_1448::<0x7000aeb7a0> as *const (), 0), // +0x735830 sub_735830
    Lifted::new(0x7000e21860, tpl_1449::<0x3> as *const (), 1), // +0x735860 sub_735860
    Lifted::new(0x7000e21870, tpl_1449::<0x3> as *const (), 1), // +0x735870 sub_735870
    Lifted::new(0x7000e21880, tpl_1449::<0x3> as *const (), 1), // +0x735880 sub_735880
    Lifted::new(0x7000e21890, tpl_1449::<0x3> as *const (), 1), // +0x735890 sub_735890
    Lifted::new(0x7000e218a0, tpl_1449::<0x3> as *const (), 1), // +0x7358a0 sub_7358a0
    Lifted::new(0x7000e218b0, tpl_1449::<0x3> as *const (), 1), // +0x7358b0 sub_7358b0
    Lifted::new(0x7000e218c0, tpl_1449::<0x3> as *const (), 1), // +0x7358c0 sub_7358c0
    Lifted::new(0x7000e218d0, tpl_1449::<0x3> as *const (), 1), // +0x7358d0 sub_7358d0
    Lifted::new(0x7000e218e0, tpl_1449::<0x3> as *const (), 1), // +0x7358e0 sub_7358e0
    Lifted::new(0x7000e218f0, tpl_1449::<0x3> as *const (), 1), // +0x7358f0 sub_7358f0
    Lifted::new(0x7000e21900, tpl_1449::<0x3> as *const (), 1), // +0x735900 sub_735900
    Lifted::new(0x7000e21910, tpl_1449::<0x3> as *const (), 1), // +0x735910 sub_735910
    Lifted::new(0x7000e21920, tpl_1449::<0x3> as *const (), 1), // +0x735920 sub_735920
    Lifted::new(0x7000e21930, tpl_1450::<0xb, 0x3> as *const (), 1), // +0x735930 sub_735930
    Lifted::new(0x7000e21960, tpl_1449::<0x3> as *const (), 1), // +0x735960 sub_735960
    Lifted::new(0x7000e21970, tpl_1450::<0xb, 0x3> as *const (), 1), // +0x735970 sub_735970
    Lifted::new(0x7000e219a0, tpl_1449::<0x3> as *const (), 1), // +0x7359a0 sub_7359a0
    Lifted::new(0x7000e219b0, tpl_1451 as *const (), 1), // +0x7359b0 sub_7359b0
    Lifted::new(0x7000e219c0, tpl_1451 as *const (), 1), // +0x7359c0 sub_7359c0
    Lifted::new(0x7000e219d0, tpl_1451 as *const (), 1), // +0x7359d0 sub_7359d0
    Lifted::new(0x7000e219e0, tpl_1451 as *const (), 1), // +0x7359e0 sub_7359e0
    Lifted::new(0x7000e219f0, tpl_1451 as *const (), 1), // +0x7359f0 sub_7359f0
    Lifted::new(0x7000e21a00, tpl_1451 as *const (), 1), // +0x735a00 sub_735a00
    Lifted::new(0x7000e21a10, tpl_1451 as *const (), 1), // +0x735a10 sub_735a10
    Lifted::new(0x7000e21a20, tpl_1451 as *const (), 1), // +0x735a20 sub_735a20
    Lifted::new(0x7000e21a30, tpl_1451 as *const (), 1), // +0x735a30 sub_735a30
    Lifted::new(0x7000e21a40, tpl_1451 as *const (), 1), // +0x735a40 sub_735a40
    Lifted::new(0x7000e21a50, tpl_1451 as *const (), 1), // +0x735a50 sub_735a50
    Lifted::new(0x7000e21a60, tpl_1451 as *const (), 1), // +0x735a60 sub_735a60
    Lifted::new(0x7000e21a70, tpl_1451 as *const (), 1), // +0x735a70 sub_735a70
    Lifted::new(0x7000e21a80, tpl_1451 as *const (), 1), // +0x735a80 sub_735a80
    Lifted::new(0x7000e21a90, tpl_1451 as *const (), 1), // +0x735a90 sub_735a90
    Lifted::new(0x7000e21aa0, tpl_1451 as *const (), 1), // +0x735aa0 sub_735aa0
    Lifted::new(0x7000e21ab0, tpl_1451 as *const (), 1), // +0x735ab0 sub_735ab0
    Lifted::new(0x7000e21ac0, tpl_1451 as *const (), 1), // +0x735ac0 sub_735ac0
    Lifted::new(0x7000e21ad0, tpl_1451 as *const (), 1), // +0x735ad0 sub_735ad0
    Lifted::new(0x7000e21ae0, tpl_1451 as *const (), 1), // +0x735ae0 sub_735ae0
    Lifted::new(0x7000e21af0, tpl_1451 as *const (), 1), // +0x735af0 sub_735af0
    Lifted::new(0x7000e21b00, tpl_1451 as *const (), 1), // +0x735b00 sub_735b00
    Lifted::new(0x7000e21b10, fn_7000e21b10 as *const (), 0), // +0x735b10 sub_735b10
    Lifted::new(0x7000e21b70, tpl_1451 as *const (), 1), // +0x735b70 sub_735b70
    Lifted::new(0x7000e21b80, tpl_1451 as *const (), 1), // +0x735b80 sub_735b80
    Lifted::new(0x7000e21b90, tpl_1451 as *const (), 1), // +0x735b90 sub_735b90
    Lifted::new(0x7000e21ba0, tpl_1451 as *const (), 1), // +0x735ba0 sub_735ba0
    Lifted::new(0x7000e21bb0, tpl_1451 as *const (), 1), // +0x735bb0 sub_735bb0
    Lifted::new(0x7000e21bc0, tpl_1451 as *const (), 1), // +0x735bc0 sub_735bc0
    Lifted::new(0x7000e21bd0, tpl_1451 as *const (), 1), // +0x735bd0 sub_735bd0
    Lifted::new(0x7000e21be0, tpl_1451 as *const (), 1), // +0x735be0 sub_735be0
    Lifted::new(0x7000e21bf0, tpl_1451 as *const (), 1), // +0x735bf0 sub_735bf0
    Lifted::new(0x7000e21c00, tpl_1451 as *const (), 1), // +0x735c00 sub_735c00
    Lifted::new(0x7000e21c10, tpl_1451 as *const (), 1), // +0x735c10 sub_735c10
    Lifted::new(0x7000e21c20, tpl_1451 as *const (), 1), // +0x735c20 sub_735c20
    Lifted::new(0x7000e21c30, tpl_1451 as *const (), 1), // +0x735c30 sub_735c30
    Lifted::new(0x7000e21c40, fn_7000e21c40 as *const (), 1), // +0x735c40 sub_735c40
    Lifted::new(0x7000e21c70, tpl_1445::<0x7000e21cd0> as *const (), 0), // +0x735c70 sub_735c70
    Lifted::new(0x7000e21ca0, tpl_1445::<0x7000e21d70> as *const (), 0), // +0x735ca0 sub_735ca0
    Lifted::new(0x7000e21cd0, fn_7000e21cd0 as *const (), 0), // +0x735cd0 sub_735cd0
    Lifted::new(0x7000e21d70, fn_7000e21d70 as *const (), 0), // +0x735d70 sub_735d70
    Lifted::new(0x7000e21e00, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x735e00 sub_735e00
    Lifted::new(0x7000e21e50, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x735e50 sub_735e50
    Lifted::new(0x7000e21ea0, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x735ea0 sub_735ea0
    Lifted::new(0x7000e21ef0, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x735ef0 sub_735ef0
    Lifted::new(0x7000e21f40, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x735f40 sub_735f40
    Lifted::new(0x7000e21f90, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x735f90 sub_735f90
    Lifted::new(0x7000e21fe0, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x735fe0 sub_735fe0
    Lifted::new(0x7000e22030, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736030 sub_736030
    Lifted::new(0x7000e22080, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736080 sub_736080
    Lifted::new(0x7000e220d0, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x7360d0 sub_7360d0
    Lifted::new(0x7000e22120, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736120 sub_736120
    Lifted::new(0x7000e22170, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736170 sub_736170
    Lifted::new(0x7000e221c0, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x7361c0 sub_7361c0
    Lifted::new(0x7000e22210, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736210 sub_736210
    Lifted::new(0x7000e22260, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736260 sub_736260
    Lifted::new(0x7000e222b0, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x7362b0 sub_7362b0
    Lifted::new(0x7000e22300, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736300 sub_736300
    Lifted::new(0x7000e22350, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736350 sub_736350
    Lifted::new(0x7000e223a0, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x7363a0 sub_7363a0
    Lifted::new(0x7000e223f0, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x7363f0 sub_7363f0
    Lifted::new(0x7000e22440, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736440 sub_736440
    Lifted::new(0x7000e22490, tpl_1452::<0x7000aeb7a0> as *const (), 0), // +0x736490 sub_736490
    Lifted::new(0x7000e224e0, tpl_1453::<0x7000e23280> as *const (), 0), // +0x7364e0 sub_7364e0
    Lifted::new(0x7000e22530, fn_7000e22530 as *const (), 1), // +0x736530 sub_736530
    Lifted::new(0x7000e225c0, tpl_1454::<0x7000e22bb0> as *const (), 0), // +0x7365c0 sub_7365c0
    Lifted::new(0x7000e22610, tpl_1454::<0x7000e22c70> as *const (), 0), // +0x736610 sub_736610
    Lifted::new(0x7000e22660, tpl_1454::<0x7000e22d30> as *const (), 0), // +0x736660 sub_736660
    Lifted::new(0x7000e226b0, tpl_1454::<0x7000e22df0> as *const (), 0), // +0x7366b0 sub_7366b0
    Lifted::new(0x7000e22700, tpl_1454::<0x7000e22eb0> as *const (), 0), // +0x736700 sub_736700
    Lifted::new(0x7000e22750, tpl_1455::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x736750 sub_736750
    Lifted::new(0x7000e227b0, tpl_1455::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x7367b0 sub_7367b0
    Lifted::new(0x7000e22810, tpl_1455::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x736810 sub_736810
    Lifted::new(0x7000e22870, tpl_1455::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x736870 sub_736870
    Lifted::new(0x7000e228d0, tpl_1455::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x7368d0 sub_7368d0
    Lifted::new(0x7000e22930, tpl_1455::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x736930 sub_736930
    Lifted::new(0x7000e22990, tpl_1453::<0x7000e22f70> as *const (), 0), // +0x736990 sub_736990
    Lifted::new(0x7000e229e0, tpl_1453::<0x7000e23190> as *const (), 0), // +0x7369e0 sub_7369e0
    Lifted::new(0x7000e22a30, tpl_1453::<0x7000e230b0> as *const (), 0), // +0x736a30 sub_736a30
    Lifted::new(0x7000e22a80, fn_7000e22a80 as *const (), 0), // +0x736a80 sub_736a80
    Lifted::new(0x7000e22bb0, tpl_1456::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x38, 0x8, 0xa8> as *const (), 1), // +0x736bb0 sub_736bb0
    Lifted::new(0x7000e22c70, tpl_1456::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x38, 0x8, 0xa8> as *const (), 1), // +0x736c70 sub_736c70
    Lifted::new(0x7000e22d30, tpl_1456::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x38, 0x8, 0xa8> as *const (), 1), // +0x736d30 sub_736d30
    Lifted::new(0x7000e22df0, tpl_1456::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x38, 0x8, 0xa8> as *const (), 1), // +0x736df0 sub_736df0
    Lifted::new(0x7000e22eb0, tpl_1456::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x38, 0x8, 0xa8> as *const (), 1), // +0x736eb0 sub_736eb0
    Lifted::new(0x7000e22f70, fn_7000e22f70 as *const (), 1), // +0x736f70 sub_736f70
    Lifted::new(0x7000e230b0, fn_7000e230b0 as *const (), 0), // +0x7370b0 sub_7370b0
    Lifted::new(0x7000e23190, fn_7000e23190 as *const (), 1), // +0x737190 sub_737190
    Lifted::new(0x7000e23280, fn_7000e23280 as *const (), 0), // +0x737280 sub_737280
    Lifted::new(0x7000e233b0, fn_7000e233b0 as *const (), 1), // +0x7373b0 sub_7373b0
    Lifted::new(0x7000e233e0, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x7373e0 sub_7373e0
    Lifted::new(0x7000e23490, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x737490 sub_737490
    Lifted::new(0x7000e23540, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x737540 sub_737540
    Lifted::new(0x7000e235f0, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x7375f0 sub_7375f0
    Lifted::new(0x7000e236a0, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x7376a0 sub_7376a0
    Lifted::new(0x7000e23750, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x737750 sub_737750
    Lifted::new(0x7000e23800, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x737800 sub_737800
    Lifted::new(0x7000e238b0, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x7378b0 sub_7378b0
    Lifted::new(0x7000e23960, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x737960 sub_737960
    Lifted::new(0x7000e23a10, tpl_1457::<0x8, 0xffffffffffffffff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60> as *const (), 0), // +0x737a10 sub_737a10
    Lifted::new(0x7000e23ac0, fn_7000e23ac0 as *const (), 0), // +0x737ac0 sub_737ac0
    Lifted::new(0x7000e23c40, fn_7000e23c40 as *const (), 0), // +0x737c40 sub_737c40
    Lifted::new(0x7000e23ea0, tpl_1458 as *const (), 1), // +0x737ea0 sub_737ea0
    Lifted::new(0x7000e23eb0, tpl_1458 as *const (), 1), // +0x737eb0 sub_737eb0
    Lifted::new(0x7000e23ec0, tpl_1459::<0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e23ef6> as *const (), 0), // +0x737ec0 sub_737ec0
    Lifted::new(0x7000e23f10, tpl_1459::<0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e23f46> as *const (), 0), // +0x737f10 sub_737f10
    Lifted::new(0x7000e23f60, tpl_1460 as *const (), 1), // +0x737f60 sub_737f60
    Lifted::new(0x7000e23f80, tpl_1460 as *const (), 1), // +0x737f80 sub_737f80
    Lifted::new(0x7000e23fa0, fn_7000e23fa0 as *const (), 3), // +0x737fa0 sub_737fa0
    Lifted::new(0x7000e23fd0, fn_7000e23fd0 as *const (), 3), // +0x737fd0 sub_737fd0
    Lifted::new(0x7000e24040, tpl_1458 as *const (), 1), // +0x738040 sub_738040
    Lifted::new(0x7000e24050, tpl_1461 as *const (), 1), // +0x738050 sub_738050
    Lifted::new(0x7000e240a0, tpl_1461 as *const (), 1), // +0x7380a0 sub_7380a0
    Lifted::new(0x7000e240f0, fn_7000e240f0 as *const (), 0), // +0x7380f0 sub_7380f0
    Lifted::new(0x7000e241b0, fn_7000e241b0 as *const (), 0), // +0x7381b0 sub_7381b0
    Lifted::new(0x7000e24320, tpl_1462::<0x70006f2a70, 0x70006f9a60, 0x70009e75c0, 0x70009c8930, 0x7001d550a8, 0x7000783b70, 0x7000e23540> as *const (), 0), // +0x738320 sub_738320
    Lifted::new(0x7000e244e0, tpl_1462::<0x70006f2a70, 0x70006f9a60, 0x70009e75c0, 0x70009c8930, 0x7001d550a8, 0x7000783b70, 0x7000e240f0> as *const (), 0), // +0x7384e0 sub_7384e0
    Lifted::new(0x7000e246a0, tpl_1463::<0x2f, 0x27, 0x10, 0xb, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x7386a0 sub_7386a0
    Lifted::new(0x7000e24780, tpl_1464::<0x2f, 0x27, 0x10, 0xb, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x738780 sub_738780
    Lifted::new(0x7000e24860, tpl_1465::<0xb, 0x10, 0x2f, 0x27, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e24935> as *const (), 0), // +0x738860 sub_738860
    Lifted::new(0x7000e24970, tpl_1466::<0xb, 0x10, 0x2f, 0x27, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e24a46> as *const (), 0), // +0x738970 sub_738970
    Lifted::new(0x7000e24a80, tpl_1467::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x7000a9ecd0> as *const (), 0), // +0x738a80 sub_738a80
    Lifted::new(0x7000e24b10, tpl_1467::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x70009c9de0> as *const (), 0), // +0x738b10 sub_738b10
    Lifted::new(0x7000e24ba0, tpl_1468::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x70009c9cb0> as *const (), 2), // +0x738ba0 sub_738ba0
    Lifted::new(0x7000e24c40, tpl_1469::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x70009c9cb0> as *const (), 2), // +0x738c40 sub_738c40
    Lifted::new(0x7000e24d00, tpl_1463::<0x2f, 0x27, 0x10, 0xb, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x738d00 sub_738d00
    Lifted::new(0x7000e24de0, tpl_1470::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x7000db91c0> as *const (), 0), // +0x738de0 sub_738de0
    Lifted::new(0x7000e24ea0, tpl_1470::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x700077c7f0> as *const (), 0), // +0x738ea0 sub_738ea0
    Lifted::new(0x7000e24f60, tpl_1463::<0x2f, 0x27, 0x10, 0xb, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x738f60 sub_738f60
    Lifted::new(0x7000e25040, tpl_1464::<0x2f, 0x27, 0x10, 0xb, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x739040 sub_739040
    Lifted::new(0x7000e25120, tpl_1465::<0xb, 0x10, 0x2f, 0x27, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e251f5> as *const (), 0), // +0x739120 sub_739120
    Lifted::new(0x7000e25230, tpl_1466::<0xb, 0x10, 0x2f, 0x27, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e25306> as *const (), 0), // +0x739230 sub_739230
    Lifted::new(0x7000e25340, tpl_1467::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x7000a9ecd0> as *const (), 0), // +0x739340 sub_739340
    Lifted::new(0x7000e253d0, tpl_1467::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x70009c9de0> as *const (), 0), // +0x7393d0 sub_7393d0
    Lifted::new(0x7000e25460, tpl_1468::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x70009c9cb0> as *const (), 2), // +0x739460 sub_739460
    Lifted::new(0x7000e25500, tpl_1469::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x70009c9cb0> as *const (), 2), // +0x739500 sub_739500
    Lifted::new(0x7000e255c0, tpl_1463::<0x2f, 0x27, 0x10, 0xb, 0x2b, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x7395c0 sub_7395c0
    Lifted::new(0x7000e256a0, tpl_1470::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x7000db91c0> as *const (), 0), // +0x7396a0 sub_7396a0
    Lifted::new(0x7000e25760, tpl_1470::<0x10, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffff6898, 0x700077c7f0> as *const (), 0), // +0x739760 sub_739760
    Lifted::new(0x7000e25820, tpl_462::<0x10> as *const (), 1), // +0x739820 sub_739820
    Lifted::new(0x7000e25840, fn_7000e25840 as *const (), 0), // +0x739840 sub_739840
    Lifted::new(0x7000e25af0, fn_7000e25af0 as *const (), 0), // +0x739af0 sub_739af0
    Lifted::new(0x7000e25da0, fn_7000e25da0 as *const (), 0), // +0x739da0 sub_739da0
    Lifted::new(0x7000e26050, fn_7000e26050 as *const (), 0), // +0x73a050 sub_73a050
    Lifted::new(0x7000e26300, fn_7000e26300 as *const (), 0), // +0x73a300 sub_73a300
    Lifted::new(0x7000e265b0, fn_7000e265b0 as *const (), 0), // +0x73a5b0 sub_73a5b0
    Lifted::new(0x7000e26860, fn_7000e26860 as *const (), 0), // +0x73a860 sub_73a860
    Lifted::new(0x7000e26b10, fn_7000e26b10 as *const (), 0), // +0x73ab10 sub_73ab10
    Lifted::new(0x7000e26dc0, fn_7000e26dc0 as *const (), 0), // +0x73adc0 sub_73adc0
    Lifted::new(0x7000e27070, fn_7000e27070 as *const (), 0), // +0x73b070 sub_73b070
    Lifted::new(0x7000e27320, fn_7000e27320 as *const (), 0), // +0x73b320 sub_73b320
    Lifted::new(0x7000e275d0, fn_7000e275d0 as *const (), 0), // +0x73b5d0 sub_73b5d0
    Lifted::new(0x7000e27880, fn_7000e27880 as *const (), 0), // +0x73b880 sub_73b880
    Lifted::new(0x7000e27b30, fn_7000e27b30 as *const (), 0), // +0x73bb30 sub_73bb30
    Lifted::new(0x7000e27de0, fn_7000e27de0 as *const (), 0), // +0x73bde0 sub_73bde0
    Lifted::new(0x7000e28090, fn_7000e28090 as *const (), 0), // +0x73c090 sub_73c090
    Lifted::new(0x7000e28340, fn_7000e28340 as *const (), 0), // +0x73c340 sub_73c340
    Lifted::new(0x7000e285f0, fn_7000e285f0 as *const (), 0), // +0x73c5f0 sub_73c5f0
    Lifted::new(0x7000e288a0, fn_7000e288a0 as *const (), 0), // +0x73c8a0 sub_73c8a0
    Lifted::new(0x7000e28b50, fn_7000e28b50 as *const (), 0), // +0x73cb50 sub_73cb50
    Lifted::new(0x7000e28e00, fn_7000e28e00 as *const (), 0), // +0x73ce00 sub_73ce00
    Lifted::new(0x7000e290b0, fn_7000e290b0 as *const (), 0), // +0x73d0b0 sub_73d0b0
    Lifted::new(0x7000e29360, fn_7000e29360 as *const (), 0), // +0x73d360 sub_73d360
    Lifted::new(0x7000e29610, fn_7000e29610 as *const (), 0), // +0x73d610 sub_73d610
    Lifted::new(0x7000e29820, fn_7000e29820 as *const (), 0), // +0x73d820 sub_73d820
    Lifted::new(0x7000e29a30, fn_7000e29a30 as *const (), 0), // +0x73da30 sub_73da30
    Lifted::new(0x7000e29c40, fn_7000e29c40 as *const (), 0), // +0x73dc40 sub_73dc40
    Lifted::new(0x7000e29e50, fn_7000e29e50 as *const (), 0), // +0x73de50 sub_73de50
    Lifted::new(0x7000e2a060, fn_7000e2a060 as *const (), 0), // +0x73e060 sub_73e060
    Lifted::new(0x7000e2a270, fn_7000e2a270 as *const (), 0), // +0x73e270 sub_73e270
    Lifted::new(0x7000e2a480, fn_7000e2a480 as *const (), 0), // +0x73e480 sub_73e480
    Lifted::new(0x7000e2a690, fn_7000e2a690 as *const (), 0), // +0x73e690 sub_73e690
    Lifted::new(0x7000e2a8a0, fn_7000e2a8a0 as *const (), 0), // +0x73e8a0 sub_73e8a0
    Lifted::new(0x7000e2aab0, fn_7000e2aab0 as *const (), 0), // +0x73eab0 sub_73eab0
    Lifted::new(0x7000e2acc0, fn_7000e2acc0 as *const (), 0), // +0x73ecc0 sub_73ecc0
    Lifted::new(0x7000e2aed0, fn_7000e2aed0 as *const (), 0), // +0x73eed0 sub_73eed0
    Lifted::new(0x7000e2b0e0, fn_7000e2b0e0 as *const (), 0), // +0x73f0e0 sub_73f0e0
    Lifted::new(0x7000e2b2f0, fn_7000e2b2f0 as *const (), 0), // +0x73f2f0 sub_73f2f0
    Lifted::new(0x7000e2b5a0, fn_7000e2b5a0 as *const (), 0), // +0x73f5a0 sub_73f5a0
    Lifted::new(0x7000e2b7b0, tpl_1471::<0x7000e2bef0> as *const (), 0), // +0x73f7b0 sub_73f7b0
    Lifted::new(0x7000e2b7e0, tpl_1471::<0x7000e2c170> as *const (), 0), // +0x73f7e0 sub_73f7e0
    Lifted::new(0x7000e2b810, tpl_1471::<0x7000e2c3f0> as *const (), 0), // +0x73f810 sub_73f810
    Lifted::new(0x7000e2b840, tpl_1471::<0x7000e2c670> as *const (), 0), // +0x73f840 sub_73f840
    Lifted::new(0x7000e2b870, tpl_1471::<0x7000e2c8f0> as *const (), 0), // +0x73f870 sub_73f870
    Lifted::new(0x7000e2b8a0, tpl_1471::<0x7000e2cb70> as *const (), 0), // +0x73f8a0 sub_73f8a0
    Lifted::new(0x7000e2b8d0, tpl_1471::<0x7000e2cdf0> as *const (), 0), // +0x73f8d0 sub_73f8d0
    Lifted::new(0x7000e2b900, tpl_1471::<0x7000e2d070> as *const (), 0), // +0x73f900 sub_73f900
    Lifted::new(0x7000e2b930, tpl_1471::<0x7000e2d2f0> as *const (), 0), // +0x73f930 sub_73f930
    Lifted::new(0x7000e2b960, tpl_1471::<0x7000e2d570> as *const (), 0), // +0x73f960 sub_73f960
    Lifted::new(0x7000e2b990, tpl_1471::<0x7000e2d7f0> as *const (), 0), // +0x73f990 sub_73f990
    Lifted::new(0x7000e2b9c0, tpl_1471::<0x7000e2da70> as *const (), 0), // +0x73f9c0 sub_73f9c0
    Lifted::new(0x7000e2b9f0, tpl_1471::<0x7000e2dcf0> as *const (), 0), // +0x73f9f0 sub_73f9f0
    Lifted::new(0x7000e2ba20, tpl_1471::<0x7000e2df70> as *const (), 0), // +0x73fa20 sub_73fa20
    Lifted::new(0x7000e2ba50, tpl_1471::<0x7000e2e1f0> as *const (), 0), // +0x73fa50 sub_73fa50
    Lifted::new(0x7000e2ba80, tpl_1471::<0x7000e2e470> as *const (), 0), // +0x73fa80 sub_73fa80
    Lifted::new(0x7000e2bab0, tpl_1471::<0x7000e2e6f0> as *const (), 0), // +0x73fab0 sub_73fab0
    Lifted::new(0x7000e2bae0, tpl_1471::<0x7000e2e970> as *const (), 0), // +0x73fae0 sub_73fae0
    Lifted::new(0x7000e2bb10, tpl_1471::<0x7000e2ebf0> as *const (), 0), // +0x73fb10 sub_73fb10
    Lifted::new(0x7000e2bb40, tpl_1471::<0x7000e2ee70> as *const (), 0), // +0x73fb40 sub_73fb40
    Lifted::new(0x7000e2bb70, tpl_1471::<0x7000e2f0f0> as *const (), 0), // +0x73fb70 sub_73fb70
    Lifted::new(0x7000e2bba0, tpl_1471::<0x7000e2f370> as *const (), 0), // +0x73fba0 sub_73fba0
    Lifted::new(0x7000e2bbd0, tpl_1471::<0x7000e2f5f0> as *const (), 0), // +0x73fbd0 sub_73fbd0
    Lifted::new(0x7000e2bc00, tpl_1471::<0x7000e2f870> as *const (), 0), // +0x73fc00 sub_73fc00
    Lifted::new(0x7000e2bc30, tpl_1471::<0x7000e2fa00> as *const (), 0), // +0x73fc30 sub_73fc30
    Lifted::new(0x7000e2bc60, tpl_1471::<0x7000e2fb90> as *const (), 0), // +0x73fc60 sub_73fc60
    Lifted::new(0x7000e2bc90, tpl_1471::<0x7000e2fd20> as *const (), 0), // +0x73fc90 sub_73fc90
    Lifted::new(0x7000e2bcc0, tpl_1471::<0x7000e2feb0> as *const (), 0), // +0x73fcc0 sub_73fcc0
    Lifted::new(0x7000e2bcf0, tpl_1471::<0x7000e30040> as *const (), 0), // +0x73fcf0 sub_73fcf0
    Lifted::new(0x7000e2bd20, tpl_1471::<0x7000e301d0> as *const (), 0), // +0x73fd20 sub_73fd20
    Lifted::new(0x7000e2bd50, tpl_1471::<0x7000e30380> as *const (), 0), // +0x73fd50 sub_73fd50
    Lifted::new(0x7000e2bd80, tpl_1471::<0x7000e30510> as *const (), 0), // +0x73fd80 sub_73fd80
    Lifted::new(0x7000e2bdb0, tpl_1471::<0x7000e306c0> as *const (), 0), // +0x73fdb0 sub_73fdb0
    Lifted::new(0x7000e2bde0, tpl_1471::<0x7000e30870> as *const (), 0), // +0x73fde0 sub_73fde0
    Lifted::new(0x7000e2be10, tpl_1471::<0x7000e30a00> as *const (), 0), // +0x73fe10 sub_73fe10
    Lifted::new(0x7000e2be40, tpl_1471::<0x7000e30ef0> as *const (), 0), // +0x73fe40 sub_73fe40
    Lifted::new(0x7000e2be70, tpl_1471::<0x7000e30bd0> as *const (), 0), // +0x73fe70 sub_73fe70
    Lifted::new(0x7000e2bea0, fn_7000e2bea0 as *const (), 0), // +0x73fea0 sub_73fea0
    Lifted::new(0x7000e2bec0, tpl_1471::<0x7000e30d60> as *const (), 0), // +0x73fec0 sub_73fec0
    Lifted::new(0x7000e2bef0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00800, 0x7000e2c0e3> as *const (), 0), // +0x73fef0 sub_73fef0
    Lifted::new(0x7000e2c170, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00820, 0x7000e2c363> as *const (), 0), // +0x740170 sub_740170
    Lifted::new(0x7000e2c3f0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00840, 0x7000e2c5e3> as *const (), 0), // +0x7403f0 sub_7403f0
    Lifted::new(0x7000e2c670, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00860, 0x7000e2c863> as *const (), 0), // +0x740670 sub_740670
    Lifted::new(0x7000e2c8f0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00880, 0x7000e2cae3> as *const (), 0), // +0x7408f0 sub_7408f0
    Lifted::new(0x7000e2cb70, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e008a0, 0x7000e2cd63> as *const (), 0), // +0x740b70 sub_740b70
    Lifted::new(0x7000e2cdf0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e008c0, 0x7000e2cfe3> as *const (), 0), // +0x740df0 sub_740df0
    Lifted::new(0x7000e2d070, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e008e0, 0x7000e2d263> as *const (), 0), // +0x741070 sub_741070
    Lifted::new(0x7000e2d2f0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00900, 0x7000e2d4e3> as *const (), 0), // +0x7412f0 sub_7412f0
    Lifted::new(0x7000e2d570, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00920, 0x7000e2d763> as *const (), 0), // +0x741570 sub_741570
    Lifted::new(0x7000e2d7f0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00940, 0x7000e2d9e3> as *const (), 0), // +0x7417f0 sub_7417f0
    Lifted::new(0x7000e2da70, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00960, 0x7000e2dc63> as *const (), 0), // +0x741a70 sub_741a70
    Lifted::new(0x7000e2dcf0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00980, 0x7000e2dee3> as *const (), 0), // +0x741cf0 sub_741cf0
    Lifted::new(0x7000e2df70, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e009a0, 0x7000e2e163> as *const (), 0), // +0x741f70 sub_741f70
    Lifted::new(0x7000e2e1f0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e009c0, 0x7000e2e3e3> as *const (), 0), // +0x7421f0 sub_7421f0
    Lifted::new(0x7000e2e470, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e009e0, 0x7000e2e663> as *const (), 0), // +0x742470 sub_742470
    Lifted::new(0x7000e2e6f0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a00, 0x7000e2e8e3> as *const (), 0), // +0x7426f0 sub_7426f0
    Lifted::new(0x7000e2e970, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a20, 0x7000e2eb63> as *const (), 0), // +0x742970 sub_742970
    Lifted::new(0x7000e2ebf0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a40, 0x7000e2ede3> as *const (), 0), // +0x742bf0 sub_742bf0
    Lifted::new(0x7000e2ee70, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a60, 0x7000e2f063> as *const (), 0), // +0x742e70 sub_742e70
    Lifted::new(0x7000e2f0f0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a80, 0x7000e2f2e3> as *const (), 0), // +0x7430f0 sub_7430f0
    Lifted::new(0x7000e2f370, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00aa0, 0x7000e2f563> as *const (), 0), // +0x743370 sub_743370
    Lifted::new(0x7000e2f5f0, tpl_1472::<0x7000778530, 0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e01940, 0x7000e2f7e3> as *const (), 0), // +0x7435f0 sub_7435f0
    Lifted::new(0x7000e2f870, fn_7000e2f870 as *const (), 0), // +0x743870 sub_743870
    Lifted::new(0x7000e2fa00, fn_7000e2fa00 as *const (), 0), // +0x743a00 sub_743a00
    Lifted::new(0x7000e2fb90, fn_7000e2fb90 as *const (), 0), // +0x743b90 sub_743b90
    Lifted::new(0x7000e2fd20, fn_7000e2fd20 as *const (), 0), // +0x743d20 sub_743d20
    Lifted::new(0x7000e2feb0, fn_7000e2feb0 as *const (), 0), // +0x743eb0 sub_743eb0
    Lifted::new(0x7000e30040, fn_7000e30040 as *const (), 0), // +0x744040 sub_744040
    Lifted::new(0x7000e301d0, fn_7000e301d0 as *const (), 0), // +0x7441d0 sub_7441d0
    Lifted::new(0x7000e30380, fn_7000e30380 as *const (), 0), // +0x744380 sub_744380
    Lifted::new(0x7000e30510, fn_7000e30510 as *const (), 0), // +0x744510 sub_744510
    Lifted::new(0x7000e306c0, fn_7000e306c0 as *const (), 0), // +0x7446c0 sub_7446c0
    Lifted::new(0x7000e30870, fn_7000e30870 as *const (), 0), // +0x744870 sub_744870
    Lifted::new(0x7000e30a00, fn_7000e30a00 as *const (), 0), // +0x744a00 sub_744a00
    Lifted::new(0x7000e30bb0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e30bc1> as *const (), 0), // +0x744bb0 sub_744bb0
    Lifted::new(0x7000e30bd0, tpl_1473::<0x70006f2a70, 0x70006f9a60, 0x7000778530, 0x7000de86b0, 0x7000e871d0, 0x7000e5de80, 0x7000e7f6e0> as *const (), 0), // +0x744bd0 sub_744bd0
    Lifted::new(0x7000e30d60, tpl_1473::<0x70006f2a70, 0x70006f9a60, 0x7000778530, 0x7000de8900, 0x7000e871d0, 0x7000e5de80, 0x7000e7f6e0> as *const (), 0), // +0x744d60 sub_744d60
    Lifted::new(0x7000e30ef0, fn_7000e30ef0 as *const (), 0), // +0x744ef0 sub_744ef0
    Lifted::new(0x7000e31180, tpl_144 as *const (), 1), // +0x745180 sub_745180
    Lifted::new(0x7000e31190, tpl_144 as *const (), 1), // +0x745190 sub_745190
    Lifted::new(0x7000e311a0, tpl_144 as *const (), 1), // +0x7451a0 sub_7451a0
    Lifted::new(0x7000e311b0, tpl_144 as *const (), 1), // +0x7451b0 sub_7451b0
    Lifted::new(0x7000e311c0, tpl_144 as *const (), 1), // +0x7451c0 sub_7451c0
    Lifted::new(0x7000e311d0, tpl_144 as *const (), 1), // +0x7451d0 sub_7451d0
    Lifted::new(0x7000e311e0, tpl_144 as *const (), 1), // +0x7451e0 sub_7451e0
    Lifted::new(0x7000e311f0, tpl_144 as *const (), 1), // +0x7451f0 sub_7451f0
    Lifted::new(0x7000e31200, tpl_144 as *const (), 1), // +0x745200 sub_745200
    Lifted::new(0x7000e31210, tpl_144 as *const (), 1), // +0x745210 sub_745210
    Lifted::new(0x7000e31220, tpl_144 as *const (), 1), // +0x745220 sub_745220
    Lifted::new(0x7000e31230, tpl_144 as *const (), 1), // +0x745230 sub_745230
    Lifted::new(0x7000e31240, tpl_144 as *const (), 1), // +0x745240 sub_745240
    Lifted::new(0x7000e31250, tpl_144 as *const (), 1), // +0x745250 sub_745250
    Lifted::new(0x7000e31260, tpl_144 as *const (), 1), // +0x745260 sub_745260
    Lifted::new(0x7000e31270, tpl_144 as *const (), 1), // +0x745270 sub_745270
    Lifted::new(0x7000e31280, tpl_144 as *const (), 1), // +0x745280 sub_745280
    Lifted::new(0x7000e31290, tpl_144 as *const (), 1), // +0x745290 sub_745290
    Lifted::new(0x7000e312a0, tpl_144 as *const (), 1), // +0x7452a0 sub_7452a0
    Lifted::new(0x7000e312b0, tpl_144 as *const (), 1), // +0x7452b0 sub_7452b0
    Lifted::new(0x7000e312c0, tpl_144 as *const (), 1), // +0x7452c0 sub_7452c0
    Lifted::new(0x7000e312d0, tpl_144 as *const (), 1), // +0x7452d0 sub_7452d0
    Lifted::new(0x7000e312e0, tpl_1474::<0x7, 0x7000e31460> as *const (), 0), // +0x7452e0 sub_7452e0
    Lifted::new(0x7000e31310, tpl_144 as *const (), 1), // +0x745310 sub_745310
    Lifted::new(0x7000e31320, tpl_144 as *const (), 1), // +0x745320 sub_745320
    Lifted::new(0x7000e31330, tpl_144 as *const (), 1), // +0x745330 sub_745330
    Lifted::new(0x7000e31340, tpl_144 as *const (), 1), // +0x745340 sub_745340
    Lifted::new(0x7000e31350, tpl_144 as *const (), 1), // +0x745350 sub_745350
    Lifted::new(0x7000e31360, tpl_144 as *const (), 1), // +0x745360 sub_745360
    Lifted::new(0x7000e31370, tpl_144 as *const (), 1), // +0x745370 sub_745370
    Lifted::new(0x7000e31380, tpl_144 as *const (), 1), // +0x745380 sub_745380
    Lifted::new(0x7000e31390, tpl_144 as *const (), 1), // +0x745390 sub_745390
    Lifted::new(0x7000e313a0, tpl_144 as *const (), 1), // +0x7453a0 sub_7453a0
    Lifted::new(0x7000e313b0, tpl_144 as *const (), 1), // +0x7453b0 sub_7453b0
    Lifted::new(0x7000e313c0, tpl_144 as *const (), 1), // +0x7453c0 sub_7453c0
    Lifted::new(0x7000e313d0, tpl_144 as *const (), 1), // +0x7453d0 sub_7453d0
    Lifted::new(0x7000e313e0, tpl_144 as *const (), 1), // +0x7453e0 sub_7453e0
    Lifted::new(0x7000e313f0, fn_7000e313f0 as *const (), 0), // +0x7453f0 sub_7453f0
    Lifted::new(0x7000e31430, tpl_1474::<0x7, 0x7000e31460> as *const (), 0), // +0x745430 sub_745430
    Lifted::new(0x7000e31460, fn_7000e31460 as *const (), 0), // +0x745460 sub_745460
    Lifted::new(0x7000e315a0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7455a0 sub_7455a0
    Lifted::new(0x7000e315e0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7455e0 sub_7455e0
    Lifted::new(0x7000e31620, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745620 sub_745620
    Lifted::new(0x7000e31660, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745660 sub_745660
    Lifted::new(0x7000e316a0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7456a0 sub_7456a0
    Lifted::new(0x7000e316e0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7456e0 sub_7456e0
    Lifted::new(0x7000e31720, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745720 sub_745720
    Lifted::new(0x7000e31760, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745760 sub_745760
    Lifted::new(0x7000e317a0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7457a0 sub_7457a0
    Lifted::new(0x7000e317e0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7457e0 sub_7457e0
    Lifted::new(0x7000e31820, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745820 sub_745820
    Lifted::new(0x7000e31860, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745860 sub_745860
    Lifted::new(0x7000e318a0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7458a0 sub_7458a0
    Lifted::new(0x7000e318e0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7458e0 sub_7458e0
    Lifted::new(0x7000e31920, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745920 sub_745920
    Lifted::new(0x7000e31960, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745960 sub_745960
    Lifted::new(0x7000e319a0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7459a0 sub_7459a0
    Lifted::new(0x7000e319e0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x7459e0 sub_7459e0
    Lifted::new(0x7000e31a20, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745a20 sub_745a20
    Lifted::new(0x7000e31a60, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745a60 sub_745a60
    Lifted::new(0x7000e31aa0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745aa0 sub_745aa0
    Lifted::new(0x7000e31ae0, tpl_1475::<0x7000aeb7a0> as *const (), 0), // +0x745ae0 sub_745ae0
    Lifted::new(0x7000e31b20, tpl_1476::<0x10, 0x7000e23280> as *const (), 0), // +0x745b20 sub_745b20
    Lifted::new(0x7000e31b80, fn_7000e31b80 as *const (), 1), // +0x745b80 sub_745b80
    Lifted::new(0x7000e31c00, tpl_1477::<0x10, 0xffffffffffffffff, 0x7, 0xb, 0x3, 0x8> as *const (), 1), // +0x745c00 sub_745c00
    Lifted::new(0x7000e31cb0, tpl_1477::<0x10, 0xffffffffffffffff, 0x7, 0xb, 0x3, 0x8> as *const (), 1), // +0x745cb0 sub_745cb0
    Lifted::new(0x7000e31d60, tpl_1477::<0x10, 0xffffffffffffffff, 0x7, 0xb, 0x3, 0x8> as *const (), 1), // +0x745d60 sub_745d60
    Lifted::new(0x7000e31e10, tpl_1477::<0x10, 0xffffffffffffffff, 0x7, 0xb, 0x3, 0x8> as *const (), 1), // +0x745e10 sub_745e10
    Lifted::new(0x7000e31ec0, tpl_1477::<0x10, 0xffffffffffffffff, 0x7, 0xb, 0x3, 0x8> as *const (), 1), // +0x745ec0 sub_745ec0
    Lifted::new(0x7000e31f70, tpl_1478::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x745f70 sub_745f70
    Lifted::new(0x7000e31fd0, tpl_1478::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x745fd0 sub_745fd0
    Lifted::new(0x7000e32030, tpl_1478::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x746030 sub_746030
    Lifted::new(0x7000e32090, tpl_1478::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x746090 sub_746090
    Lifted::new(0x7000e320f0, tpl_1478::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x7460f0 sub_7460f0
    Lifted::new(0x7000e32150, tpl_1478::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x746150 sub_746150
    Lifted::new(0x7000e321b0, tpl_1476::<0x10, 0x7000e22f70> as *const (), 0), // +0x7461b0 sub_7461b0
    Lifted::new(0x7000e32210, fn_7000e32210 as *const (), 0), // +0x746210 sub_746210
    Lifted::new(0x7000e32240, fn_7000e32240 as *const (), 0), // +0x746240 sub_746240
    Lifted::new(0x7000e32320, fn_7000e32320 as *const (), 0), // +0x746320 sub_746320
    Lifted::new(0x7000e323c0, fn_7000e323c0 as *const (), 1), // +0x7463c0 sub_7463c0
    Lifted::new(0x7000e32480, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e331b0, 0x7000e324bc> as *const (), 0), // +0x746480 sub_746480
    Lifted::new(0x7000e324d0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e331d0, 0x7000e3250c> as *const (), 0), // +0x7464d0 sub_7464d0
    Lifted::new(0x7000e32520, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e331f0, 0x7000e3255c> as *const (), 0), // +0x746520 sub_746520
    Lifted::new(0x7000e32570, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33210, 0x7000e325ac> as *const (), 0), // +0x746570 sub_746570
    Lifted::new(0x7000e325c0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33230, 0x7000e325fc> as *const (), 0), // +0x7465c0 sub_7465c0
    Lifted::new(0x7000e32610, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33250, 0x7000e3264c> as *const (), 0), // +0x746610 sub_746610
    Lifted::new(0x7000e32660, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33270, 0x7000e3269c> as *const (), 0), // +0x746660 sub_746660
    Lifted::new(0x7000e326b0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33290, 0x7000e326ec> as *const (), 0), // +0x7466b0 sub_7466b0
    Lifted::new(0x7000e32700, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e332b0, 0x7000e3273c> as *const (), 0), // +0x746700 sub_746700
    Lifted::new(0x7000e32750, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e332d0, 0x7000e3278c> as *const (), 0), // +0x746750 sub_746750
    Lifted::new(0x7000e327a0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e332f0, 0x7000e327dc> as *const (), 0), // +0x7467a0 sub_7467a0
    Lifted::new(0x7000e327f0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33310, 0x7000e3282c> as *const (), 0), // +0x7467f0 sub_7467f0
    Lifted::new(0x7000e32840, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33330, 0x7000e3287c> as *const (), 0), // +0x746840 sub_746840
    Lifted::new(0x7000e32890, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33350, 0x7000e328cc> as *const (), 0), // +0x746890 sub_746890
    Lifted::new(0x7000e328e0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33370, 0x7000e3291c> as *const (), 0), // +0x7468e0 sub_7468e0
    Lifted::new(0x7000e32930, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33390, 0x7000e3296c> as *const (), 0), // +0x746930 sub_746930
    Lifted::new(0x7000e32980, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e333b0, 0x7000e329bc> as *const (), 0), // +0x746980 sub_746980
    Lifted::new(0x7000e329d0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e333d0, 0x7000e32a0c> as *const (), 0), // +0x7469d0 sub_7469d0
    Lifted::new(0x7000e32a20, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e333f0, 0x7000e32a5c> as *const (), 0), // +0x746a20 sub_746a20
    Lifted::new(0x7000e32a70, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33410, 0x7000e32aac> as *const (), 0), // +0x746a70 sub_746a70
    Lifted::new(0x7000e32ac0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33430, 0x7000e32afc> as *const (), 0), // +0x746ac0 sub_746ac0
    Lifted::new(0x7000e32b10, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33450, 0x7000e32b4c> as *const (), 0), // +0x746b10 sub_746b10
    Lifted::new(0x7000e32b60, fn_7000e32b60 as *const (), 0), // +0x746b60 sub_746b60
    Lifted::new(0x7000e32bf0, tpl_1480::<0x7> as *const (), 1), // +0x746bf0 sub_746bf0
    Lifted::new(0x7000e32c30, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746c30 sub_746c30
    Lifted::new(0x7000e32c90, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746c90 sub_746c90
    Lifted::new(0x7000e32cf0, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746cf0 sub_746cf0
    Lifted::new(0x7000e32d50, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746d50 sub_746d50
    Lifted::new(0x7000e32db0, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746db0 sub_746db0
    Lifted::new(0x7000e32e10, tpl_1480::<0x7> as *const (), 1), // +0x746e10 sub_746e10
    Lifted::new(0x7000e32e50, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746e50 sub_746e50
    Lifted::new(0x7000e32eb0, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746eb0 sub_746eb0
    Lifted::new(0x7000e32f10, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746f10 sub_746f10
    Lifted::new(0x7000e32f70, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746f70 sub_746f70
    Lifted::new(0x7000e32fd0, tpl_1481::<0x10, 0x8, 0x7, 0xffffffffffff68d0, 0xffffffffffffffff, 0xffffffffffff6940> as *const (), 1), // +0x746fd0 sub_746fd0
    Lifted::new(0x7000e33030, fn_7000e33030 as *const (), 1), // +0x747030 sub_747030
    Lifted::new(0x7000e330d0, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33470, 0x7000e3310c> as *const (), 0), // +0x7470d0 sub_7470d0
    Lifted::new(0x7000e33120, fn_7000e33120 as *const (), 0), // +0x747120 sub_747120
    Lifted::new(0x7000e33160, tpl_1479::<0x7000d906c0, 0x7000e25820, 0x7000e33490, 0x7000e3319c> as *const (), 0), // +0x747160 sub_747160
    Lifted::new(0x7000e331b0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e331c1> as *const (), 0), // +0x7471b0 sub_7471b0
    Lifted::new(0x7000e331d0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e331e1> as *const (), 0), // +0x7471d0 sub_7471d0
    Lifted::new(0x7000e331f0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33201> as *const (), 0), // +0x7471f0 sub_7471f0
    Lifted::new(0x7000e33210, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33221> as *const (), 0), // +0x747210 sub_747210
    Lifted::new(0x7000e33230, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33241> as *const (), 0), // +0x747230 sub_747230
    Lifted::new(0x7000e33250, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33261> as *const (), 0), // +0x747250 sub_747250
    Lifted::new(0x7000e33270, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33281> as *const (), 0), // +0x747270 sub_747270
    Lifted::new(0x7000e33290, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e332a1> as *const (), 0), // +0x747290 sub_747290
    Lifted::new(0x7000e332b0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e332c1> as *const (), 0), // +0x7472b0 sub_7472b0
    Lifted::new(0x7000e332d0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e332e1> as *const (), 0), // +0x7472d0 sub_7472d0
    Lifted::new(0x7000e332f0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33301> as *const (), 0), // +0x7472f0 sub_7472f0
    Lifted::new(0x7000e33310, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33321> as *const (), 0), // +0x747310 sub_747310
    Lifted::new(0x7000e33330, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33341> as *const (), 0), // +0x747330 sub_747330
    Lifted::new(0x7000e33350, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33361> as *const (), 0), // +0x747350 sub_747350
    Lifted::new(0x7000e33370, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33381> as *const (), 0), // +0x747370 sub_747370
    Lifted::new(0x7000e33390, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e333a1> as *const (), 0), // +0x747390 sub_747390
    Lifted::new(0x7000e333b0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e333c1> as *const (), 0), // +0x7473b0 sub_7473b0
    Lifted::new(0x7000e333d0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e333e1> as *const (), 0), // +0x7473d0 sub_7473d0
    Lifted::new(0x7000e333f0, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33401> as *const (), 0), // +0x7473f0 sub_7473f0
    Lifted::new(0x7000e33410, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33421> as *const (), 0), // +0x747410 sub_747410
    Lifted::new(0x7000e33430, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33441> as *const (), 0), // +0x747430 sub_747430
    Lifted::new(0x7000e33450, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33461> as *const (), 0), // +0x747450 sub_747450
    Lifted::new(0x7000e33470, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e33481> as *const (), 0), // +0x747470 sub_747470
    Lifted::new(0x7000e33490, tpl_177::<0x70017d9c58, 0x7001d54ef8, 0x7000e334a1> as *const (), 0), // +0x747490 sub_747490
    Lifted::new(0x7000e334b0, fn_7000e334b0 as *const (), 0), // +0x7474b0 sub_7474b0
    Lifted::new(0x7000e33570, tpl_1482::<0x7000e37400> as *const (), 0), // +0x747570 sub_747570
    Lifted::new(0x7000e335c0, tpl_1482::<0x7000e37600> as *const (), 0), // +0x7475c0 sub_7475c0
    Lifted::new(0x7000e33610, tpl_1482::<0x7000e37810> as *const (), 0), // +0x747610 sub_747610
    Lifted::new(0x7000e33660, tpl_1482::<0x7000e37a40> as *const (), 0), // +0x747660 sub_747660
    Lifted::new(0x7000e336b0, tpl_1482::<0x7000e37c80> as *const (), 0), // +0x7476b0 sub_7476b0
    Lifted::new(0x7000e33700, tpl_1482::<0x7000e37e90> as *const (), 0), // +0x747700 sub_747700
    Lifted::new(0x7000e33750, tpl_1482::<0x7000e380a0> as *const (), 0), // +0x747750 sub_747750
    Lifted::new(0x7000e337a0, tpl_1482::<0x7000e38320> as *const (), 0), // +0x7477a0 sub_7477a0
    Lifted::new(0x7000e337f0, tpl_1482::<0x7000e385f0> as *const (), 0), // +0x7477f0 sub_7477f0
    Lifted::new(0x7000e33840, tpl_1482::<0x7000e387f0> as *const (), 0), // +0x747840 sub_747840
    Lifted::new(0x7000e33890, tpl_1482::<0x7000e389c0> as *const (), 0), // +0x747890 sub_747890
    Lifted::new(0x7000e338e0, tpl_1482::<0x7000e38b90> as *const (), 0), // +0x7478e0 sub_7478e0
    Lifted::new(0x7000e33930, tpl_1482::<0x7000e38d90> as *const (), 0), // +0x747930 sub_747930
    Lifted::new(0x7000e33980, tpl_1482::<0x7000e38fa0> as *const (), 0), // +0x747980 sub_747980
    Lifted::new(0x7000e339d0, tpl_1482::<0x7000e391d0> as *const (), 0), // +0x7479d0 sub_7479d0
    Lifted::new(0x7000e33a20, tpl_1482::<0x7000e39410> as *const (), 0), // +0x747a20 sub_747a20
    Lifted::new(0x7000e33a70, tpl_1482::<0x7000e39620> as *const (), 0), // +0x747a70 sub_747a70
    Lifted::new(0x7000e33ac0, tpl_1482::<0x7000e39830> as *const (), 0), // +0x747ac0 sub_747ac0
    Lifted::new(0x7000e33b10, tpl_1482::<0x7000e39a30> as *const (), 0), // +0x747b10 sub_747b10
    Lifted::new(0x7000e33b60, tpl_1482::<0x7000e39c40> as *const (), 0), // +0x747b60 sub_747b60
    Lifted::new(0x7000e33bb0, tpl_1482::<0x7000e39e40> as *const (), 0), // +0x747bb0 sub_747bb0
    Lifted::new(0x7000e33c00, tpl_1482::<0x7000e3a0e0> as *const (), 0), // +0x747c00 sub_747c00
    Lifted::new(0x7000e33c50, tpl_1482::<0x7000e3a300> as *const (), 0), // +0x747c50 sub_747c50
    Lifted::new(0x7000e33ca0, tpl_1482::<0x7000e34420> as *const (), 0), // +0x747ca0 sub_747ca0
    Lifted::new(0x7000e33cf0, tpl_1482::<0x7000e34690> as *const (), 0), // +0x747cf0 sub_747cf0
    Lifted::new(0x7000e33d40, tpl_1482::<0x7000e349f0> as *const (), 0), // +0x747d40 sub_747d40
    Lifted::new(0x7000e33d90, tpl_1482::<0x7000e34d50> as *const (), 0), // +0x747d90 sub_747d90
    Lifted::new(0x7000e33de0, tpl_1482::<0x7000e350b0> as *const (), 0), // +0x747de0 sub_747de0
    Lifted::new(0x7000e33e30, tpl_1482::<0x7000e35410> as *const (), 0), // +0x747e30 sub_747e30
    Lifted::new(0x7000e33e80, tpl_1482::<0x7000e35660> as *const (), 0), // +0x747e80 sub_747e80
    Lifted::new(0x7000e33ed0, tpl_1482::<0x7000e358d0> as *const (), 0), // +0x747ed0 sub_747ed0
    Lifted::new(0x7000e33f20, tpl_1482::<0x7000e35c30> as *const (), 0), // +0x747f20 sub_747f20
    Lifted::new(0x7000e33f70, tpl_1482::<0x7000e35f90> as *const (), 0), // +0x747f70 sub_747f70
    Lifted::new(0x7000e33fc0, tpl_1482::<0x7000e362f0> as *const (), 0), // +0x747fc0 sub_747fc0
    Lifted::new(0x7000e34010, tpl_1482::<0x7000e36650> as *const (), 0), // +0x748010 sub_748010
    Lifted::new(0x7000e34060, tpl_1482::<0x7000e368a0> as *const (), 0), // +0x748060 sub_748060
    Lifted::new(0x7000e340b0, tpl_1482::<0x7000e3a870> as *const (), 0), // +0x7480b0 sub_7480b0
    Lifted::new(0x7000e34100, tpl_1482::<0x7000e36dd0> as *const (), 0), // +0x748100 sub_748100
    Lifted::new(0x7000e34150, tpl_1482::<0x7000e3a870> as *const (), 0), // +0x748150 sub_748150
    Lifted::new(0x7000e341a0, fn_7000e341a0 as *const (), 2), // +0x7481a0 sub_7481a0
    Lifted::new(0x7000e34420, tpl_1483 as *const (), 3), // +0x748420 sub_748420
    Lifted::new(0x7000e34690, tpl_1484::<0x7000eed560> as *const (), 2), // +0x748690 sub_748690
    Lifted::new(0x7000e349f0, tpl_1484::<0x7000eed560> as *const (), 2), // +0x7489f0 sub_7489f0
    Lifted::new(0x7000e34d50, tpl_1484::<0x7000eed560> as *const (), 2), // +0x748d50 sub_748d50
    Lifted::new(0x7000e350b0, tpl_1484::<0x7000eed560> as *const (), 2), // +0x7490b0 sub_7490b0
    Lifted::new(0x7000e35410, tpl_1485 as *const (), 3), // +0x749410 sub_749410
    Lifted::new(0x7000e35660, tpl_1483 as *const (), 3), // +0x749660 sub_749660
    Lifted::new(0x7000e358d0, tpl_1484::<0x7000eed560> as *const (), 2), // +0x7498d0 sub_7498d0
    Lifted::new(0x7000e35c30, tpl_1484::<0x7000eed560> as *const (), 2), // +0x749c30 sub_749c30
    Lifted::new(0x7000e35f90, tpl_1484::<0x7000eed560> as *const (), 2), // +0x749f90 sub_749f90
    Lifted::new(0x7000e362f0, tpl_1484::<0x7000eed560> as *const (), 2), // +0x74a2f0 sub_74a2f0
    Lifted::new(0x7000e36650, tpl_1485 as *const (), 3), // +0x74a650 sub_74a650
    Lifted::new(0x7000e368a0, fn_7000e368a0 as *const (), 0), // +0x74a8a0 sub_74a8a0
    Lifted::new(0x7000e36dd0, fn_7000e36dd0 as *const (), 0), // +0x74add0 sub_74add0
    Lifted::new(0x7000e37400, tpl_1486::<0x700177c160, 0x7000dc86f0> as *const (), 2), // +0x74b400 sub_74b400
    Lifted::new(0x7000e37600, tpl_1487::<0x700177c160, 0x7001855d68, 0x7001855d08> as *const (), 2), // +0x74b600 sub_74b600
    Lifted::new(0x7000e37810, tpl_1488::<0x700177c160, 0x7000dc8730, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e379c4> as *const (), 2), // +0x74b810 sub_74b810
    Lifted::new(0x7000e37a40, tpl_1489::<0x700177c160, 0x7001855d78, 0x7001855d28, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e37c04> as *const (), 2), // +0x74ba40 sub_74ba40
    Lifted::new(0x7000e37c80, tpl_1490::<0x700177c160, 0x70017dcfb0, 0x70017d8e20> as *const (), 2), // +0x74bc80 sub_74bc80
    Lifted::new(0x7000e37e90, tpl_1491::<0x700177c160, 0x70017d37f0, 0x70017d37e0> as *const (), 2), // +0x74be90 sub_74be90
    Lifted::new(0x7000e380a0, fn_7000e380a0 as *const (), 2), // +0x74c0a0 sub_74c0a0
    Lifted::new(0x7000e38320, fn_7000e38320 as *const (), 2), // +0x74c320 sub_74c320
    Lifted::new(0x7000e385f0, tpl_1486::<0x700177c160, 0x7000dc86f0> as *const (), 2), // +0x74c5f0 sub_74c5f0
    Lifted::new(0x7000e387f0, tpl_1492::<0x7000db6760> as *const (), 0), // +0x74c7f0 sub_74c7f0
    Lifted::new(0x7000e389c0, tpl_1492::<0x7000757d30> as *const (), 0), // +0x74c9c0 sub_74c9c0
    Lifted::new(0x7000e38b90, tpl_1486::<0x700177c160, 0x7000dc86f0> as *const (), 2), // +0x74cb90 sub_74cb90
    Lifted::new(0x7000e38d90, tpl_1487::<0x700177c160, 0x7001855d68, 0x7001855d08> as *const (), 2), // +0x74cd90 sub_74cd90
    Lifted::new(0x7000e38fa0, tpl_1488::<0x700177c160, 0x7000dc8730, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e39154> as *const (), 2), // +0x74cfa0 sub_74cfa0
    Lifted::new(0x7000e391d0, tpl_1489::<0x700177c160, 0x7001855d78, 0x7001855d28, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e39394> as *const (), 2), // +0x74d1d0 sub_74d1d0
    Lifted::new(0x7000e39410, tpl_1490::<0x700177c160, 0x70017dcfb0, 0x70017d8e20> as *const (), 2), // +0x74d410 sub_74d410
    Lifted::new(0x7000e39620, tpl_1491::<0x700177c160, 0x70017d37f0, 0x70017d37e0> as *const (), 2), // +0x74d620 sub_74d620
    Lifted::new(0x7000e39830, fn_7000e39830 as *const (), 2), // +0x74d830 sub_74d830
    Lifted::new(0x7000e39a30, fn_7000e39a30 as *const (), 2), // +0x74da30 sub_74da30
    Lifted::new(0x7000e39c40, tpl_1486::<0x700177c160, 0x7000dc86f0> as *const (), 2), // +0x74dc40 sub_74dc40
    Lifted::new(0x7000e39e40, fn_7000e39e40 as *const (), 2), // +0x74de40 sub_74de40
    Lifted::new(0x7000e3a0e0, fn_7000e3a0e0 as *const (), 2), // +0x74e0e0 sub_74e0e0
    Lifted::new(0x7000e3a300, fn_7000e3a300 as *const (), 0), // +0x74e300 sub_74e300
    Lifted::new(0x7000e3a870, fn_7000e3a870 as *const (), 0), // +0x74e870 sub_74e870
    Lifted::new(0x7000e3a9b0, tpl_1493::<0x700177c160, 0x7000dc86f0> as *const (), 2), // +0x74e9b0 sub_74e9b0
    Lifted::new(0x7000e3ab60, tpl_1494::<0x700177c160, 0x7001855d68, 0x7001855d08> as *const (), 2), // +0x74eb60 sub_74eb60
    Lifted::new(0x7000e3ad20, tpl_1482::<0x7000e3dcc0> as *const (), 0), // +0x74ed20 sub_74ed20
    Lifted::new(0x7000e3ad70, tpl_1482::<0x7000e3de70> as *const (), 0), // +0x74ed70 sub_74ed70
    Lifted::new(0x7000e3adc0, tpl_1482::<0x7000e3e040> as *const (), 0), // +0x74edc0 sub_74edc0
    Lifted::new(0x7000e3ae10, tpl_1482::<0x7000e3e1f0> as *const (), 0), // +0x74ee10 sub_74ee10
    Lifted::new(0x7000e3ae60, tpl_1482::<0x7000e3e3a0> as *const (), 0), // +0x74ee60 sub_74ee60
    Lifted::new(0x7000e3aeb0, tpl_1482::<0x7000e3e560> as *const (), 0), // +0x74eeb0 sub_74eeb0
    Lifted::new(0x7000e3af00, tpl_1493::<0x700177c160, 0x7000dc86f0> as *const (), 2), // +0x74ef00 sub_74ef00
    Lifted::new(0x7000e3b0b0, tpl_1495::<0x7000db6760> as *const (), 0), // +0x74f0b0 sub_74f0b0
    Lifted::new(0x7000e3b230, tpl_1495::<0x7000757d30> as *const (), 0), // +0x74f230 sub_74f230
    Lifted::new(0x7000e3b3b0, tpl_1493::<0x700177c160, 0x7000dc86f0> as *const (), 2), // +0x74f3b0 sub_74f3b0
    Lifted::new(0x7000e3b560, tpl_1494::<0x700177c160, 0x7001855d68, 0x7001855d08> as *const (), 2), // +0x74f560 sub_74f560
    Lifted::new(0x7000e3b720, tpl_1482::<0x7000e3e730> as *const (), 0), // +0x74f720 sub_74f720
    Lifted::new(0x7000e3b770, tpl_1482::<0x7000e3e8e0> as *const (), 0), // +0x74f770 sub_74f770
    Lifted::new(0x7000e3b7c0, tpl_1482::<0x7000e3eab0> as *const (), 0), // +0x74f7c0 sub_74f7c0
    Lifted::new(0x7000e3b810, tpl_1482::<0x7000e3ec60> as *const (), 0), // +0x74f810 sub_74f810
    Lifted::new(0x7000e3b860, tpl_1482::<0x7000e3ee10> as *const (), 0), // +0x74f860 sub_74f860
    Lifted::new(0x7000e3b8b0, tpl_1482::<0x7000e3efc0> as *const (), 0), // +0x74f8b0 sub_74f8b0
    Lifted::new(0x7000e3b900, tpl_1493::<0x700177c160, 0x7000dc86f0> as *const (), 2), // +0x74f900 sub_74f900
    Lifted::new(0x7000e3bab0, tpl_1482::<0x7000e3f170> as *const (), 0), // +0x74fab0 sub_74fab0
    Lifted::new(0x7000e3bb00, tpl_1482::<0x7000e3f3b0> as *const (), 0), // +0x74fb00 sub_74fb00
    Lifted::new(0x7000e3bb50, tpl_1482::<0x7000e3f590> as *const (), 0), // +0x74fb50 sub_74fb50
    Lifted::new(0x7000e3bba0, tpl_1482::<0x7000e3cea0> as *const (), 0), // +0x74fba0 sub_74fba0
    Lifted::new(0x7000e3bbf0, tpl_1496::<0x7000764de0> as *const (), 2), // +0x74fbf0 sub_74fbf0
    Lifted::new(0x7000e3bdb0, tpl_1496::<0x7000764de0> as *const (), 2), // +0x74fdb0 sub_74fdb0
    Lifted::new(0x7000e3bf70, tpl_1496::<0x7000764de0> as *const (), 2), // +0x74ff70 sub_74ff70
    Lifted::new(0x7000e3c130, tpl_1496::<0x7000764de0> as *const (), 2), // +0x750130 sub_750130
    Lifted::new(0x7000e3c2f0, tpl_1497::<0x7000764de0> as *const (), 2), // +0x7502f0 sub_7502f0
    Lifted::new(0x7000e3c480, tpl_1482::<0x7000e3d010> as *const (), 0), // +0x750480 sub_750480
    Lifted::new(0x7000e3c4d0, tpl_1496::<0x7000764de0> as *const (), 2), // +0x7504d0 sub_7504d0
    Lifted::new(0x7000e3c690, tpl_1496::<0x7000764de0> as *const (), 2), // +0x750690 sub_750690
    Lifted::new(0x7000e3c850, tpl_1496::<0x7000764de0> as *const (), 2), // +0x750850 sub_750850
    Lifted::new(0x7000e3ca10, tpl_1496::<0x7000764de0> as *const (), 2), // +0x750a10 sub_750a10
    Lifted::new(0x7000e3cbd0, tpl_1497::<0x7000764de0> as *const (), 2), // +0x750bd0 sub_750bd0
    Lifted::new(0x7000e3cd60, tpl_1482::<0x7000e3d180> as *const (), 0), // +0x750d60 sub_750d60
    Lifted::new(0x7000e3cdb0, tpl_1482::<0x7000e3f980> as *const (), 0), // +0x750db0 sub_750db0
    Lifted::new(0x7000e3ce00, tpl_1482::<0x7000e3d6b0> as *const (), 0), // +0x750e00 sub_750e00
    Lifted::new(0x7000e3ce50, tpl_1482::<0x7000e3f980> as *const (), 0), // +0x750e50 sub_750e50
    Lifted::new(0x7000e3cea0, tpl_1498 as *const (), 3), // +0x750ea0 sub_750ea0
    Lifted::new(0x7000e3d010, tpl_1498 as *const (), 3), // +0x751010 sub_751010
    Lifted::new(0x7000e3d180, fn_7000e3d180 as *const (), 0), // +0x751180 sub_751180
    Lifted::new(0x7000e3d6b0, fn_7000e3d6b0 as *const (), 0), // +0x7516b0 sub_7516b0
    Lifted::new(0x7000e3dcc0, tpl_1499::<0x700177c160, 0x7000dc8730, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e3de14> as *const (), 2), // +0x751cc0 sub_751cc0
    Lifted::new(0x7000e3de70, tpl_1500::<0x700177c160, 0x7001855d78, 0x7001855d28, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e3dfdb> as *const (), 2), // +0x751e70 sub_751e70
    Lifted::new(0x7000e3e040, tpl_1501::<0x700177c160, 0x70017dcfb0, 0x70017d8e20> as *const (), 2), // +0x752040 sub_752040
    Lifted::new(0x7000e3e1f0, tpl_1502::<0x700177c160, 0x70017d37f0, 0x70017d37e0> as *const (), 2), // +0x7521f0 sub_7521f0
    Lifted::new(0x7000e3e3a0, fn_7000e3e3a0 as *const (), 2), // +0x7523a0 sub_7523a0
    Lifted::new(0x7000e3e560, fn_7000e3e560 as *const (), 2), // +0x752560 sub_752560
    Lifted::new(0x7000e3e730, tpl_1499::<0x700177c160, 0x7000dc8730, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e3e884> as *const (), 2), // +0x752730 sub_752730
    Lifted::new(0x7000e3e8e0, tpl_1500::<0x700177c160, 0x7001855d78, 0x7001855d28, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e3ea4b> as *const (), 2), // +0x7528e0 sub_7528e0
    Lifted::new(0x7000e3eab0, tpl_1501::<0x700177c160, 0x70017dcfb0, 0x70017d8e20> as *const (), 2), // +0x752ab0 sub_752ab0
    Lifted::new(0x7000e3ec60, tpl_1502::<0x700177c160, 0x70017d37f0, 0x70017d37e0> as *const (), 2), // +0x752c60 sub_752c60
    Lifted::new(0x7000e3ee10, fn_7000e3ee10 as *const (), 2), // +0x752e10 sub_752e10
    Lifted::new(0x7000e3efc0, fn_7000e3efc0 as *const (), 2), // +0x752fc0 sub_752fc0
    Lifted::new(0x7000e3f170, fn_7000e3f170 as *const (), 2), // +0x753170 sub_753170
    Lifted::new(0x7000e3f3b0, fn_7000e3f3b0 as *const (), 2), // +0x7533b0 sub_7533b0
    Lifted::new(0x7000e3f590, fn_7000e3f590 as *const (), 0), // +0x753590 sub_753590
    Lifted::new(0x7000e3f980, fn_7000e3f980 as *const (), 0), // +0x753980 sub_753980
    Lifted::new(0x7000e3fab0, fn_7000e3fab0 as *const (), 0), // +0x753ab0 sub_753ab0
    Lifted::new(0x7000e40330, fn_7000e40330 as *const (), 1), // +0x754330 sub_754330
    Lifted::new(0x7000e40350, fn_7000e40350 as *const (), 1), // +0x754350 sub_754350
    Lifted::new(0x7000e40370, tpl_1503::<0x700177c160, 0x70017dcfb0, 0x7001855d18> as *const (), 2), // +0x754370 sub_754370
    Lifted::new(0x7000e404f0, tpl_1504::<0x700177c160, 0x7001855d68, 0x7001855d08> as *const (), 2), // +0x7544f0 sub_7544f0
    Lifted::new(0x7000e40660, tpl_1505::<0x700177c160, 0x70017dcfb0, 0x7001855d38, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e407b0> as *const (), 2), // +0x754660 sub_754660
    Lifted::new(0x7000e40810, tpl_1506::<0x700177c160, 0x7001855d78, 0x7001855d28, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e40959> as *const (), 2), // +0x754810 sub_754810
    Lifted::new(0x7000e409b0, tpl_1507::<0x700177c160, 0x70017dcfb0, 0x70017d8e20> as *const (), 2), // +0x7549b0 sub_7549b0
    Lifted::new(0x7000e40b30, tpl_1508::<0x700177c160, 0x70017d37f0, 0x70017d37e0> as *const (), 2), // +0x754b30 sub_754b30
    Lifted::new(0x7000e40cb0, tpl_1509::<0x700177c160, 0x70017d8ec8, 0x70017d8e60> as *const (), 2), // +0x754cb0 sub_754cb0
    Lifted::new(0x7000e40e40, tpl_1510::<0x700177c160> as *const (), 2), // +0x754e40 sub_754e40
    Lifted::new(0x7000e40fc0, tpl_1503::<0x700177c160, 0x70017dcfb0, 0x7001855d18> as *const (), 2), // +0x754fc0 sub_754fc0
    Lifted::new(0x7000e41140, tpl_1511::<0x7000db6760> as *const (), 0), // +0x755140 sub_755140
    Lifted::new(0x7000e412a0, tpl_1511::<0x7000757d30> as *const (), 0), // +0x7552a0 sub_7552a0
    Lifted::new(0x7000e41400, tpl_1503::<0x700177c160, 0x70017dcfb0, 0x7001855d18> as *const (), 2), // +0x755400 sub_755400
    Lifted::new(0x7000e41580, tpl_1504::<0x700177c160, 0x7001855d68, 0x7001855d08> as *const (), 2), // +0x755580 sub_755580
    Lifted::new(0x7000e416f0, tpl_1505::<0x700177c160, 0x70017dcfb0, 0x7001855d38, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e41840> as *const (), 2), // +0x7556f0 sub_7556f0
    Lifted::new(0x7000e418a0, tpl_1506::<0x700177c160, 0x7001855d78, 0x7001855d28, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e419e9> as *const (), 2), // +0x7558a0 sub_7558a0
    Lifted::new(0x7000e41a40, tpl_1507::<0x700177c160, 0x70017dcfb0, 0x70017d8e20> as *const (), 2), // +0x755a40 sub_755a40
    Lifted::new(0x7000e41bc0, tpl_1508::<0x700177c160, 0x70017d37f0, 0x70017d37e0> as *const (), 2), // +0x755bc0 sub_755bc0
    Lifted::new(0x7000e41d40, tpl_1509::<0x700177c160, 0x70017d8ec8, 0x70017d8e60> as *const (), 2), // +0x755d40 sub_755d40
    Lifted::new(0x7000e41ed0, tpl_1510::<0x700177c160> as *const (), 2), // +0x755ed0 sub_755ed0
    Lifted::new(0x7000e42050, tpl_1503::<0x700177c160, 0x70017dcfb0, 0x7001855d18> as *const (), 2), // +0x756050 sub_756050
    Lifted::new(0x7000e421d0, fn_7000e421d0 as *const (), 2), // +0x7561d0 sub_7561d0
    Lifted::new(0x7000e423e0, fn_7000e423e0 as *const (), 2), // +0x7563e0 sub_7563e0
    Lifted::new(0x7000e42590, tpl_1512::<0x7000e428c0, 0x7000e425ad> as *const (), 0), // +0x756590 sub_756590
    Lifted::new(0x7000e425c0, tpl_1512::<0x7000e428e0, 0x7000e425dd> as *const (), 0), // +0x7565c0 sub_7565c0
    Lifted::new(0x7000e425f0, tpl_1512::<0x7000e42900, 0x7000e4260d> as *const (), 0), // +0x7565f0 sub_7565f0
    Lifted::new(0x7000e42620, tpl_1512::<0x7000e42920, 0x7000e4263d> as *const (), 0), // +0x756620 sub_756620
    Lifted::new(0x7000e42650, tpl_1512::<0x7000e42940, 0x7000e4266d> as *const (), 0), // +0x756650 sub_756650
    Lifted::new(0x7000e42680, tpl_1512::<0x7000e42960, 0x7000e4269d> as *const (), 0), // +0x756680 sub_756680
    Lifted::new(0x7000e426b0, tpl_1512::<0x7000e42980, 0x7000e426cd> as *const (), 0), // +0x7566b0 sub_7566b0
    Lifted::new(0x7000e426e0, tpl_1512::<0x7000e429a0, 0x7000e426fd> as *const (), 0), // +0x7566e0 sub_7566e0
    Lifted::new(0x7000e42710, tpl_1512::<0x7000e429c0, 0x7000e4272d> as *const (), 0), // +0x756710 sub_756710
    Lifted::new(0x7000e42740, tpl_1512::<0x7000e429e0, 0x7000e4275d> as *const (), 0), // +0x756740 sub_756740
    Lifted::new(0x7000e42770, tpl_1512::<0x7000e42a00, 0x7000e4278d> as *const (), 0), // +0x756770 sub_756770
    Lifted::new(0x7000e427a0, tpl_1512::<0x7000e42a20, 0x7000e427bd> as *const (), 0), // +0x7567a0 sub_7567a0
    Lifted::new(0x7000e427d0, tpl_1512::<0x7000e42a40, 0x7000e427ed> as *const (), 0), // +0x7567d0 sub_7567d0
    Lifted::new(0x7000e42800, tpl_1512::<0x7000e42a60, 0x7000e4281d> as *const (), 0), // +0x756800 sub_756800
    Lifted::new(0x7000e42830, tpl_1512::<0x7000e42a80, 0x7000e4284d> as *const (), 0), // +0x756830 sub_756830
    Lifted::new(0x7000e42860, tpl_1512::<0x7000e42aa0, 0x7000e4287d> as *const (), 0), // +0x756860 sub_756860
    Lifted::new(0x7000e42890, tpl_1512::<0x7000e42ac0, 0x7000e428ad> as *const (), 0), // +0x756890 sub_756890
    Lifted::new(0x7000e428c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e428d1> as *const (), 0), // +0x7568c0 sub_7568c0
    Lifted::new(0x7000e428e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e428f1> as *const (), 0), // +0x7568e0 sub_7568e0
    Lifted::new(0x7000e42900, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42911> as *const (), 0), // +0x756900 sub_756900
    Lifted::new(0x7000e42920, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42931> as *const (), 0), // +0x756920 sub_756920
    Lifted::new(0x7000e42940, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42951> as *const (), 0), // +0x756940 sub_756940
    Lifted::new(0x7000e42960, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42971> as *const (), 0), // +0x756960 sub_756960
    Lifted::new(0x7000e42980, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42991> as *const (), 0), // +0x756980 sub_756980
    Lifted::new(0x7000e429a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e429b1> as *const (), 0), // +0x7569a0 sub_7569a0
    Lifted::new(0x7000e429c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e429d1> as *const (), 0), // +0x7569c0 sub_7569c0
    Lifted::new(0x7000e429e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e429f1> as *const (), 0), // +0x7569e0 sub_7569e0
    Lifted::new(0x7000e42a00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42a11> as *const (), 0), // +0x756a00 sub_756a00
    Lifted::new(0x7000e42a20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42a31> as *const (), 0), // +0x756a20 sub_756a20
    Lifted::new(0x7000e42a40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42a51> as *const (), 0), // +0x756a40 sub_756a40
    Lifted::new(0x7000e42a60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42a71> as *const (), 0), // +0x756a60 sub_756a60
    Lifted::new(0x7000e42a80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42a91> as *const (), 0), // +0x756a80 sub_756a80
    Lifted::new(0x7000e42aa0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42ab1> as *const (), 0), // +0x756aa0 sub_756aa0
    Lifted::new(0x7000e42ac0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e42ad1> as *const (), 0), // +0x756ac0 sub_756ac0
    Lifted::new(0x7000e42ae0, fn_7000e42ae0 as *const (), 0), // +0x756ae0 sub_756ae0
    Lifted::new(0x7000e42ba0, tpl_1513::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x70006f3b00> as *const (), 0), // +0x756ba0 sub_756ba0
    Lifted::new(0x7000e42d60, tpl_1514::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x756d60 sub_756d60
    Lifted::new(0x7000e43000, tpl_1514::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x757000 sub_757000
    Lifted::new(0x7000e432a0, tpl_1514::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x7572a0 sub_7572a0
    Lifted::new(0x7000e43540, tpl_1514::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x757540 sub_757540
    Lifted::new(0x7000e437e0, tpl_1515::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x7577e0 sub_7577e0
    Lifted::new(0x7000e43a50, tpl_1513::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x70006f3b00> as *const (), 0), // +0x757a50 sub_757a50
    Lifted::new(0x7000e43c10, tpl_1514::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x757c10 sub_757c10
    Lifted::new(0x7000e43eb0, tpl_1514::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x757eb0 sub_757eb0
    Lifted::new(0x7000e44150, tpl_1514::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x758150 sub_758150
    Lifted::new(0x7000e443f0, tpl_1514::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x7583f0 sub_7583f0
    Lifted::new(0x7000e44690, tpl_1515::<0x7000c45b50, 0x700073eb40, 0x7000767d50, 0x700074a010, 0x7000718620, 0x7001c72d48> as *const (), 2), // +0x758690 sub_758690
    Lifted::new(0x7000e44900, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47cf0, 0x7000e44998> as *const (), 0), // +0x758900 sub_758900
    Lifted::new(0x7000e449c0, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47d10, 0x7000e44a58> as *const (), 0), // +0x7589c0 sub_7589c0
    Lifted::new(0x7000e44a80, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47d30, 0x7000e44b18> as *const (), 0), // +0x758a80 sub_758a80
    Lifted::new(0x7000e44b40, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47d50, 0x7000e44bd8> as *const (), 0), // +0x758b40 sub_758b40
    Lifted::new(0x7000e44c00, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47d70, 0x7000e44c98> as *const (), 0), // +0x758c00 sub_758c00
    Lifted::new(0x7000e44cc0, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47d90, 0x7000e44d58> as *const (), 0), // +0x758cc0 sub_758cc0
    Lifted::new(0x7000e44d80, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47db0, 0x7000e44e18> as *const (), 0), // +0x758d80 sub_758d80
    Lifted::new(0x7000e44e40, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47dd0, 0x7000e44ed8> as *const (), 0), // +0x758e40 sub_758e40
    Lifted::new(0x7000e44f00, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47df0, 0x7000e44f98> as *const (), 0), // +0x758f00 sub_758f00
    Lifted::new(0x7000e44fc0, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47e10, 0x7000e45058> as *const (), 0), // +0x758fc0 sub_758fc0
    Lifted::new(0x7000e45080, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47e30, 0x7000e45118> as *const (), 0), // +0x759080 sub_759080
    Lifted::new(0x7000e45140, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47e50, 0x7000e451d8> as *const (), 0), // +0x759140 sub_759140
    Lifted::new(0x7000e45200, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47e70, 0x7000e45298> as *const (), 0), // +0x759200 sub_759200
    Lifted::new(0x7000e452c0, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47e90, 0x7000e45358> as *const (), 0), // +0x7592c0 sub_7592c0
    Lifted::new(0x7000e45380, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47eb0, 0x7000e45418> as *const (), 0), // +0x759380 sub_759380
    Lifted::new(0x7000e45440, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47ed0, 0x7000e454d8> as *const (), 0), // +0x759440 sub_759440
    Lifted::new(0x7000e45500, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47ef0, 0x7000e45598> as *const (), 0), // +0x759500 sub_759500
    Lifted::new(0x7000e455c0, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47f10, 0x7000e45658> as *const (), 0), // +0x7595c0 sub_7595c0
    Lifted::new(0x7000e45680, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47f30, 0x7000e45718> as *const (), 0), // +0x759680 sub_759680
    Lifted::new(0x7000e45740, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47f50, 0x7000e457d8> as *const (), 0), // +0x759740 sub_759740
    Lifted::new(0x7000e45800, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47f70, 0x7000e45898> as *const (), 0), // +0x759800 sub_759800
    Lifted::new(0x7000e458c0, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47f90, 0x7000e45958> as *const (), 0), // +0x7598c0 sub_7598c0
    Lifted::new(0x7000e45980, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47fb0, 0x7000e45a18> as *const (), 0), // +0x759980 sub_759980
    Lifted::new(0x7000e45a40, fn_7000e45a40 as *const (), 2), // +0x759a40 sub_759a40
    Lifted::new(0x7000e45ca0, fn_7000e45ca0 as *const (), 0), // +0x759ca0 sub_759ca0
    Lifted::new(0x7000e45f40, fn_7000e45f40 as *const (), 0), // +0x759f40 sub_759f40
    Lifted::new(0x7000e461e0, tpl_1517::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e48010> as *const (), 0), // +0x75a1e0 sub_75a1e0
    Lifted::new(0x7000e462b0, fn_7000e462b0 as *const (), 0), // +0x75a2b0 sub_75a2b0
    Lifted::new(0x7000e46550, fn_7000e46550 as *const (), 0), // +0x75a550 sub_75a550
    Lifted::new(0x7000e46800, fn_7000e46800 as *const (), 2), // +0x75a800 sub_75a800
    Lifted::new(0x7000e46a60, fn_7000e46a60 as *const (), 0), // +0x75aa60 sub_75aa60
    Lifted::new(0x7000e46d00, fn_7000e46d00 as *const (), 0), // +0x75ad00 sub_75ad00
    Lifted::new(0x7000e46fa0, fn_7000e46fa0 as *const (), 0), // +0x75afa0 sub_75afa0
    Lifted::new(0x7000e47250, fn_7000e47250 as *const (), 0), // +0x75b250 sub_75b250
    Lifted::new(0x7000e474f0, fn_7000e474f0 as *const (), 0), // +0x75b4f0 sub_75b4f0
    Lifted::new(0x7000e477a0, fn_7000e477a0 as *const (), 0), // +0x75b7a0 sub_75b7a0
    Lifted::new(0x7000e478f0, tpl_1517::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e48010> as *const (), 0), // +0x75b8f0 sub_75b8f0
    Lifted::new(0x7000e479c0, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47fd0, 0x7000e47a58> as *const (), 0), // +0x75b9c0 sub_75b9c0
    Lifted::new(0x7000e47a80, tpl_1516::<0x7000d8bd20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e47ff0, 0x7000e47b18> as *const (), 0), // +0x75ba80 sub_75ba80
    Lifted::new(0x7000e47b40, fn_7000e47b40 as *const (), 0), // +0x75bb40 sub_75bb40
    Lifted::new(0x7000e47cf0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47d01> as *const (), 0), // +0x75bcf0 sub_75bcf0
    Lifted::new(0x7000e47d10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47d21> as *const (), 0), // +0x75bd10 sub_75bd10
    Lifted::new(0x7000e47d30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47d41> as *const (), 0), // +0x75bd30 sub_75bd30
    Lifted::new(0x7000e47d50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47d61> as *const (), 0), // +0x75bd50 sub_75bd50
    Lifted::new(0x7000e47d70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47d81> as *const (), 0), // +0x75bd70 sub_75bd70
    Lifted::new(0x7000e47d90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47da1> as *const (), 0), // +0x75bd90 sub_75bd90
    Lifted::new(0x7000e47db0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47dc1> as *const (), 0), // +0x75bdb0 sub_75bdb0
    Lifted::new(0x7000e47dd0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47de1> as *const (), 0), // +0x75bdd0 sub_75bdd0
    Lifted::new(0x7000e47df0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47e01> as *const (), 0), // +0x75bdf0 sub_75bdf0
    Lifted::new(0x7000e47e10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47e21> as *const (), 0), // +0x75be10 sub_75be10
    Lifted::new(0x7000e47e30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47e41> as *const (), 0), // +0x75be30 sub_75be30
    Lifted::new(0x7000e47e50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47e61> as *const (), 0), // +0x75be50 sub_75be50
    Lifted::new(0x7000e47e70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47e81> as *const (), 0), // +0x75be70 sub_75be70
    Lifted::new(0x7000e47e90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47ea1> as *const (), 0), // +0x75be90 sub_75be90
    Lifted::new(0x7000e47eb0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47ec1> as *const (), 0), // +0x75beb0 sub_75beb0
    Lifted::new(0x7000e47ed0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47ee1> as *const (), 0), // +0x75bed0 sub_75bed0
    Lifted::new(0x7000e47ef0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47f01> as *const (), 0), // +0x75bef0 sub_75bef0
    Lifted::new(0x7000e47f10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47f21> as *const (), 0), // +0x75bf10 sub_75bf10
    Lifted::new(0x7000e47f30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47f41> as *const (), 0), // +0x75bf30 sub_75bf30
    Lifted::new(0x7000e47f50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47f61> as *const (), 0), // +0x75bf50 sub_75bf50
    Lifted::new(0x7000e47f70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47f81> as *const (), 0), // +0x75bf70 sub_75bf70
    Lifted::new(0x7000e47f90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47fa1> as *const (), 0), // +0x75bf90 sub_75bf90
    Lifted::new(0x7000e47fb0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47fc1> as *const (), 0), // +0x75bfb0 sub_75bfb0
    Lifted::new(0x7000e47fd0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e47fe1> as *const (), 0), // +0x75bfd0 sub_75bfd0
    Lifted::new(0x7000e47ff0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e48001> as *const (), 0), // +0x75bff0 sub_75bff0
    Lifted::new(0x7000e48010, fn_7000e48010 as *const (), 0), // +0x75c010 sub_75c010
    Lifted::new(0x7000e48240, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c240 sub_75c240
    Lifted::new(0x7000e48270, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c270 sub_75c270
    Lifted::new(0x7000e482a0, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c2a0 sub_75c2a0
    Lifted::new(0x7000e482d0, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c2d0 sub_75c2d0
    Lifted::new(0x7000e48300, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c300 sub_75c300
    Lifted::new(0x7000e48330, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c330 sub_75c330
    Lifted::new(0x7000e48360, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c360 sub_75c360
    Lifted::new(0x7000e48390, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c390 sub_75c390
    Lifted::new(0x7000e483c0, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c3c0 sub_75c3c0
    Lifted::new(0x7000e483f0, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c3f0 sub_75c3f0
    Lifted::new(0x7000e48420, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c420 sub_75c420
    Lifted::new(0x7000e48450, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c450 sub_75c450
    Lifted::new(0x7000e48480, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c480 sub_75c480
    Lifted::new(0x7000e484b0, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c4b0 sub_75c4b0
    Lifted::new(0x7000e484e0, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c4e0 sub_75c4e0
    Lifted::new(0x7000e48510, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c510 sub_75c510
    Lifted::new(0x7000e48540, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c540 sub_75c540
    Lifted::new(0x7000e48570, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c570 sub_75c570
    Lifted::new(0x7000e485a0, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c5a0 sub_75c5a0
    Lifted::new(0x7000e485d0, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c5d0 sub_75c5d0
    Lifted::new(0x7000e48600, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c600 sub_75c600
    Lifted::new(0x7000e48630, tpl_1518::<0x7000aeb7a0> as *const (), 0), // +0x75c630 sub_75c630
    Lifted::new(0x7000e48660, fn_7000e48660 as *const (), 0), // +0x75c660 sub_75c660
    Lifted::new(0x7000e486a0, tpl_1519::<0x7, 0x7000e48af0> as *const (), 0), // +0x75c6a0 sub_75c6a0
    Lifted::new(0x7000e486e0, tpl_1519::<0x7, 0x7000e48b90> as *const (), 0), // +0x75c6e0 sub_75c6e0
    Lifted::new(0x7000e48720, tpl_1519::<0x7, 0x7000e48c50> as *const (), 0), // +0x75c720 sub_75c720
    Lifted::new(0x7000e48760, tpl_1519::<0x7, 0x7000e48d10> as *const (), 0), // +0x75c760 sub_75c760
    Lifted::new(0x7000e487a0, tpl_1519::<0x7, 0x7000e48dd0> as *const (), 0), // +0x75c7a0 sub_75c7a0
    Lifted::new(0x7000e487e0, tpl_1519::<0x7, 0x7000e48e90> as *const (), 0), // +0x75c7e0 sub_75c7e0
    Lifted::new(0x7000e48820, tpl_1520::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x75c820 sub_75c820
    Lifted::new(0x7000e48870, tpl_1519::<0x7, 0x7000e48f50> as *const (), 0), // +0x75c870 sub_75c870
    Lifted::new(0x7000e488b0, tpl_1519::<0x7, 0x7000e49010> as *const (), 0), // +0x75c8b0 sub_75c8b0
    Lifted::new(0x7000e488f0, tpl_1520::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x75c8f0 sub_75c8f0
    Lifted::new(0x7000e48940, tpl_1519::<0x7, 0x7000e490d0> as *const (), 0), // +0x75c940 sub_75c940
    Lifted::new(0x7000e48980, tpl_1520::<0xffffffffffffffff, 0x7, 0xb, 0x3> as *const (), 1), // +0x75c980 sub_75c980
    Lifted::new(0x7000e489d0, tpl_1519::<0x7, 0x7000e49190> as *const (), 0), // +0x75c9d0 sub_75c9d0
    Lifted::new(0x7000e48a10, fn_7000e48a10 as *const (), 0), // +0x75ca10 sub_75ca10
    Lifted::new(0x7000e48a60, tpl_1519::<0x7, 0x7000e49250> as *const (), 0), // +0x75ca60 sub_75ca60
    Lifted::new(0x7000e48aa0, fn_7000e48aa0 as *const (), 0), // +0x75caa0 sub_75caa0
    Lifted::new(0x7000e48af0, fn_7000e48af0 as *const (), 1), // +0x75caf0 sub_75caf0
    Lifted::new(0x7000e48b90, tpl_1521::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x10, 0x8, 0x1> as *const (), 1), // +0x75cb90 sub_75cb90
    Lifted::new(0x7000e48c50, tpl_1521::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x10, 0x8, 0x1> as *const (), 1), // +0x75cc50 sub_75cc50
    Lifted::new(0x7000e48d10, tpl_1521::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x10, 0x8, 0x1> as *const (), 1), // +0x75cd10 sub_75cd10
    Lifted::new(0x7000e48dd0, tpl_1521::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x10, 0x8, 0x1> as *const (), 1), // +0x75cdd0 sub_75cdd0
    Lifted::new(0x7000e48e90, tpl_1521::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x10, 0x8, 0x1> as *const (), 1), // +0x75ce90 sub_75ce90
    Lifted::new(0x7000e48f50, tpl_1521::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x10, 0x8, 0x1> as *const (), 1), // +0x75cf50 sub_75cf50
    Lifted::new(0x7000e49010, tpl_1521::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x10, 0x8, 0x1> as *const (), 1), // +0x75d010 sub_75d010
    Lifted::new(0x7000e490d0, tpl_1521::<0xffffffffffffffff, 0x7, 0xb, 0x3, 0x10, 0x8, 0x1> as *const (), 1), // +0x75d0d0 sub_75d0d0
    Lifted::new(0x7000e49190, fn_7000e49190 as *const (), 0), // +0x75d190 sub_75d190
    Lifted::new(0x7000e49250, fn_7000e49250 as *const (), 0), // +0x75d250 sub_75d250
    Lifted::new(0x7000e49310, tpl_1426::<0x7000e49870, 0x7000e4931f> as *const (), 0), // +0x75d310 sub_75d310
    Lifted::new(0x7000e49330, tpl_1426::<0x7000e49890, 0x7000e4933f> as *const (), 0), // +0x75d330 sub_75d330
    Lifted::new(0x7000e49350, tpl_1426::<0x7000e498b0, 0x7000e4935f> as *const (), 0), // +0x75d350 sub_75d350
    Lifted::new(0x7000e49370, tpl_1426::<0x7000e498d0, 0x7000e4937f> as *const (), 0), // +0x75d370 sub_75d370
    Lifted::new(0x7000e49390, tpl_1426::<0x7000e498f0, 0x7000e4939f> as *const (), 0), // +0x75d390 sub_75d390
    Lifted::new(0x7000e493b0, tpl_1426::<0x7000e49910, 0x7000e493bf> as *const (), 0), // +0x75d3b0 sub_75d3b0
    Lifted::new(0x7000e493d0, tpl_1426::<0x7000e49930, 0x7000e493df> as *const (), 0), // +0x75d3d0 sub_75d3d0
    Lifted::new(0x7000e493f0, tpl_1426::<0x7000e49950, 0x7000e493ff> as *const (), 0), // +0x75d3f0 sub_75d3f0
    Lifted::new(0x7000e49410, tpl_1426::<0x7000e49970, 0x7000e4941f> as *const (), 0), // +0x75d410 sub_75d410
    Lifted::new(0x7000e49430, tpl_1426::<0x7000e49990, 0x7000e4943f> as *const (), 0), // +0x75d430 sub_75d430
    Lifted::new(0x7000e49450, tpl_1426::<0x7000e499b0, 0x7000e4945f> as *const (), 0), // +0x75d450 sub_75d450
    Lifted::new(0x7000e49470, tpl_1426::<0x7000e499d0, 0x7000e4947f> as *const (), 0), // +0x75d470 sub_75d470
    Lifted::new(0x7000e49490, tpl_1426::<0x7000e499f0, 0x7000e4949f> as *const (), 0), // +0x75d490 sub_75d490
    Lifted::new(0x7000e494b0, tpl_1426::<0x7000e49a10, 0x7000e494bf> as *const (), 0), // +0x75d4b0 sub_75d4b0
    Lifted::new(0x7000e494d0, tpl_1426::<0x7000e49a30, 0x7000e494df> as *const (), 0), // +0x75d4d0 sub_75d4d0
    Lifted::new(0x7000e494f0, tpl_1426::<0x7000e49a50, 0x7000e494ff> as *const (), 0), // +0x75d4f0 sub_75d4f0
    Lifted::new(0x7000e49510, tpl_1426::<0x7000e49a70, 0x7000e4951f> as *const (), 0), // +0x75d510 sub_75d510
    Lifted::new(0x7000e49530, tpl_1426::<0x7000e49a90, 0x7000e4953f> as *const (), 0), // +0x75d530 sub_75d530
    Lifted::new(0x7000e49550, tpl_1426::<0x7000e49ab0, 0x7000e4955f> as *const (), 0), // +0x75d550 sub_75d550
    Lifted::new(0x7000e49570, tpl_1426::<0x7000e49ad0, 0x7000e4957f> as *const (), 0), // +0x75d570 sub_75d570
    Lifted::new(0x7000e49590, tpl_1426::<0x7000e49af0, 0x7000e4959f> as *const (), 0), // +0x75d590 sub_75d590
    Lifted::new(0x7000e495b0, tpl_1426::<0x7000e49b10, 0x7000e495bf> as *const (), 0), // +0x75d5b0 sub_75d5b0
    Lifted::new(0x7000e495d0, tpl_1426::<0x7000e49b30, 0x7000e495df> as *const (), 0), // +0x75d5d0 sub_75d5d0
    Lifted::new(0x7000e495f0, tpl_1522::<0x7000e52910> as *const (), 0), // +0x75d5f0 sub_75d5f0
    Lifted::new(0x7000e49620, tpl_1426::<0x7000e49bd0, 0x7000e4962f> as *const (), 0), // +0x75d620 sub_75d620
    Lifted::new(0x7000e49640, tpl_1522::<0x7000e52ae0> as *const (), 0), // +0x75d640 sub_75d640
    Lifted::new(0x7000e49670, tpl_1522::<0x7000e52ce0> as *const (), 0), // +0x75d670 sub_75d670
    Lifted::new(0x7000e496a0, tpl_1426::<0x7000e49c10, 0x7000e496af> as *const (), 0), // +0x75d6a0 sub_75d6a0
    Lifted::new(0x7000e496c0, tpl_1522::<0x7000e52f50> as *const (), 0), // +0x75d6c0 sub_75d6c0
    Lifted::new(0x7000e496f0, tpl_1522::<0x7000e531c0> as *const (), 0), // +0x75d6f0 sub_75d6f0
    Lifted::new(0x7000e49720, tpl_1426::<0x7000e49bf0, 0x7000e4972f> as *const (), 0), // +0x75d720 sub_75d720
    Lifted::new(0x7000e49740, tpl_1522::<0x7000e53370> as *const (), 0), // +0x75d740 sub_75d740
    Lifted::new(0x7000e49770, tpl_1522::<0x7000e53570> as *const (), 0), // +0x75d770 sub_75d770
    Lifted::new(0x7000e497a0, tpl_1426::<0x7000e49c30, 0x7000e497af> as *const (), 0), // +0x75d7a0 sub_75d7a0
    Lifted::new(0x7000e497c0, tpl_1522::<0x7000e537c0> as *const (), 0), // +0x75d7c0 sub_75d7c0
    Lifted::new(0x7000e497f0, tpl_1426::<0x7000e49b50, 0x7000e497ff> as *const (), 0), // +0x75d7f0 sub_75d7f0
    Lifted::new(0x7000e49810, tpl_1426::<0x7000e49b70, 0x7000e4981f> as *const (), 0), // +0x75d810 sub_75d810
    Lifted::new(0x7000e49830, tpl_1426::<0x7000e49b90, 0x7000e4983f> as *const (), 0), // +0x75d830 sub_75d830
    Lifted::new(0x7000e49850, tpl_1426::<0x7000e49bb0, 0x7000e4985f> as *const (), 0), // +0x75d850 sub_75d850
    Lifted::new(0x7000e49870, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49881> as *const (), 0), // +0x75d870 sub_75d870
    Lifted::new(0x7000e49890, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e498a1> as *const (), 0), // +0x75d890 sub_75d890
    Lifted::new(0x7000e498b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e498c1> as *const (), 0), // +0x75d8b0 sub_75d8b0
    Lifted::new(0x7000e498d0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e498e1> as *const (), 0), // +0x75d8d0 sub_75d8d0
    Lifted::new(0x7000e498f0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49901> as *const (), 0), // +0x75d8f0 sub_75d8f0
    Lifted::new(0x7000e49910, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49921> as *const (), 0), // +0x75d910 sub_75d910
    Lifted::new(0x7000e49930, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49941> as *const (), 0), // +0x75d930 sub_75d930
    Lifted::new(0x7000e49950, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49961> as *const (), 0), // +0x75d950 sub_75d950
    Lifted::new(0x7000e49970, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49981> as *const (), 0), // +0x75d970 sub_75d970
    Lifted::new(0x7000e49990, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e499a1> as *const (), 0), // +0x75d990 sub_75d990
    Lifted::new(0x7000e499b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e499c1> as *const (), 0), // +0x75d9b0 sub_75d9b0
    Lifted::new(0x7000e499d0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e499e1> as *const (), 0), // +0x75d9d0 sub_75d9d0
    Lifted::new(0x7000e499f0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49a01> as *const (), 0), // +0x75d9f0 sub_75d9f0
    Lifted::new(0x7000e49a10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49a21> as *const (), 0), // +0x75da10 sub_75da10
    Lifted::new(0x7000e49a30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49a41> as *const (), 0), // +0x75da30 sub_75da30
    Lifted::new(0x7000e49a50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49a61> as *const (), 0), // +0x75da50 sub_75da50
    Lifted::new(0x7000e49a70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49a81> as *const (), 0), // +0x75da70 sub_75da70
    Lifted::new(0x7000e49a90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49aa1> as *const (), 0), // +0x75da90 sub_75da90
    Lifted::new(0x7000e49ab0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49ac1> as *const (), 0), // +0x75dab0 sub_75dab0
    Lifted::new(0x7000e49ad0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49ae1> as *const (), 0), // +0x75dad0 sub_75dad0
    Lifted::new(0x7000e49af0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49b01> as *const (), 0), // +0x75daf0 sub_75daf0
    Lifted::new(0x7000e49b10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49b21> as *const (), 0), // +0x75db10 sub_75db10
    Lifted::new(0x7000e49b30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49b41> as *const (), 0), // +0x75db30 sub_75db30
    Lifted::new(0x7000e49b50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49b61> as *const (), 0), // +0x75db50 sub_75db50
    Lifted::new(0x7000e49b70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49b81> as *const (), 0), // +0x75db70 sub_75db70
    Lifted::new(0x7000e49b90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49ba1> as *const (), 0), // +0x75db90 sub_75db90
    Lifted::new(0x7000e49bb0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49bc1> as *const (), 0), // +0x75dbb0 sub_75dbb0
    Lifted::new(0x7000e49bd0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49be1> as *const (), 0), // +0x75dbd0 sub_75dbd0
    Lifted::new(0x7000e49bf0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49c01> as *const (), 0), // +0x75dbf0 sub_75dbf0
    Lifted::new(0x7000e49c10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49c21> as *const (), 0), // +0x75dc10 sub_75dc10
    Lifted::new(0x7000e49c30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e49c41> as *const (), 0), // +0x75dc30 sub_75dc30
    Lifted::new(0x7000e49c50, tpl_1523::<0x7000e4b700> as *const (), 0), // +0x75dc50 sub_75dc50
    Lifted::new(0x7000e49ca0, tpl_1523::<0x7000e4b910> as *const (), 0), // +0x75dca0 sub_75dca0
    Lifted::new(0x7000e49cf0, tpl_1523::<0x7000e4bb20> as *const (), 0), // +0x75dcf0 sub_75dcf0
    Lifted::new(0x7000e49d40, tpl_1523::<0x7000e4bd30> as *const (), 0), // +0x75dd40 sub_75dd40
    Lifted::new(0x7000e49d90, tpl_1523::<0x7000e4bf40> as *const (), 0), // +0x75dd90 sub_75dd90
    Lifted::new(0x7000e49de0, tpl_1523::<0x7000e4c150> as *const (), 0), // +0x75dde0 sub_75dde0
    Lifted::new(0x7000e49e30, tpl_1523::<0x7000e4c360> as *const (), 0), // +0x75de30 sub_75de30
    Lifted::new(0x7000e49e80, tpl_1523::<0x7000e4c570> as *const (), 0), // +0x75de80 sub_75de80
    Lifted::new(0x7000e49ed0, tpl_1523::<0x7000e4c780> as *const (), 0), // +0x75ded0 sub_75ded0
    Lifted::new(0x7000e49f20, tpl_1523::<0x7000e4c990> as *const (), 0), // +0x75df20 sub_75df20
    Lifted::new(0x7000e49f70, tpl_1523::<0x7000e4cba0> as *const (), 0), // +0x75df70 sub_75df70
    Lifted::new(0x7000e49fc0, tpl_1523::<0x7000e4cdb0> as *const (), 0), // +0x75dfc0 sub_75dfc0
    Lifted::new(0x7000e4a010, tpl_1523::<0x7000e4cfc0> as *const (), 0), // +0x75e010 sub_75e010
    Lifted::new(0x7000e4a060, tpl_1523::<0x7000e4d1d0> as *const (), 0), // +0x75e060 sub_75e060
    Lifted::new(0x7000e4a0b0, tpl_1523::<0x7000e4d3e0> as *const (), 0), // +0x75e0b0 sub_75e0b0
    Lifted::new(0x7000e4a100, tpl_1523::<0x7000e4d5f0> as *const (), 0), // +0x75e100 sub_75e100
    Lifted::new(0x7000e4a150, tpl_1523::<0x7000e4d800> as *const (), 0), // +0x75e150 sub_75e150
    Lifted::new(0x7000e4a1a0, tpl_1523::<0x7000e4da10> as *const (), 0), // +0x75e1a0 sub_75e1a0
    Lifted::new(0x7000e4a1f0, tpl_1523::<0x7000e4dc20> as *const (), 0), // +0x75e1f0 sub_75e1f0
    Lifted::new(0x7000e4a240, tpl_1523::<0x7000e4de30> as *const (), 0), // +0x75e240 sub_75e240
    Lifted::new(0x7000e4a290, tpl_1523::<0x7000e4e040> as *const (), 0), // +0x75e290 sub_75e290
    Lifted::new(0x7000e4a2e0, tpl_1523::<0x7000e4e250> as *const (), 0), // +0x75e2e0 sub_75e2e0
    Lifted::new(0x7000e4a330, tpl_1523::<0x7000e4e460> as *const (), 0), // +0x75e330 sub_75e330
    Lifted::new(0x7000e4a380, tpl_1523::<0x7000e4e890> as *const (), 0), // +0x75e380 sub_75e380
    Lifted::new(0x7000e4a3d0, tpl_1523::<0x7000e4ead0> as *const (), 0), // +0x75e3d0 sub_75e3d0
    Lifted::new(0x7000e4a420, tpl_1523::<0x7000e4ed10> as *const (), 0), // +0x75e420 sub_75e420
    Lifted::new(0x7000e4a470, tpl_1523::<0x7000e4ef50> as *const (), 0), // +0x75e470 sub_75e470
    Lifted::new(0x7000e4a4c0, tpl_1523::<0x7000e4f190> as *const (), 0), // +0x75e4c0 sub_75e4c0
    Lifted::new(0x7000e4a510, tpl_1523::<0x7000e4f3d0> as *const (), 0), // +0x75e510 sub_75e510
    Lifted::new(0x7000e4a560, tpl_1524::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e19690, 0x7000e01e10> as *const (), 0), // +0x75e560 sub_75e560
    Lifted::new(0x7000e4a780, tpl_1524::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e19a80, 0x7000e01e10> as *const (), 0), // +0x75e780 sub_75e780
    Lifted::new(0x7000e4a9a0, tpl_1524::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e19e70, 0x7000e01e10> as *const (), 0), // +0x75e9a0 sub_75e9a0
    Lifted::new(0x7000e4abc0, tpl_1524::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e1a260, 0x7000e01e10> as *const (), 0), // +0x75ebc0 sub_75ebc0
    Lifted::new(0x7000e4ade0, tpl_1524::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e1a650, 0x7000e01e10> as *const (), 0), // +0x75ede0 sub_75ede0
    Lifted::new(0x7000e4b000, tpl_1524::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e1aa40, 0x7000e01e10> as *const (), 0), // +0x75f000 sub_75f000
    Lifted::new(0x7000e4b220, tpl_1523::<0x7000e4f610> as *const (), 0), // +0x75f220 sub_75f220
    Lifted::new(0x7000e4b270, tpl_1524::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e1ae30, 0x7000e01e10> as *const (), 0), // +0x75f270 sub_75f270
    Lifted::new(0x7000e4b490, tpl_1523::<0x7000e4fa30> as *const (), 0), // +0x75f490 sub_75f490
    Lifted::new(0x7000e4b4e0, tpl_1524::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e1b2d0, 0x7000e01e10> as *const (), 0), // +0x75f4e0 sub_75f4e0
    Lifted::new(0x7000e4b700, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e12ba0, 0x7000e01e10> as *const (), 0), // +0x75f700 sub_75f700
    Lifted::new(0x7000e4b910, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e12f60, 0x7000e01e10> as *const (), 0), // +0x75f910 sub_75f910
    Lifted::new(0x7000e4bb20, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e13320, 0x7000e01e10> as *const (), 0), // +0x75fb20 sub_75fb20
    Lifted::new(0x7000e4bd30, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e136e0, 0x7000e01e10> as *const (), 0), // +0x75fd30 sub_75fd30
    Lifted::new(0x7000e4bf40, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e13aa0, 0x7000e01e10> as *const (), 0), // +0x75ff40 sub_75ff40
    Lifted::new(0x7000e4c150, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e13e60, 0x7000e01e10> as *const (), 0), // +0x760150 sub_760150
    Lifted::new(0x7000e4c360, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e14220, 0x7000e01e10> as *const (), 0), // +0x760360 sub_760360
    Lifted::new(0x7000e4c570, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e145e0, 0x7000e01e10> as *const (), 0), // +0x760570 sub_760570
    Lifted::new(0x7000e4c780, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e149a0, 0x7000e01e10> as *const (), 0), // +0x760780 sub_760780
    Lifted::new(0x7000e4c990, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e14d60, 0x7000e01e10> as *const (), 0), // +0x760990 sub_760990
    Lifted::new(0x7000e4cba0, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e15120, 0x7000e01e10> as *const (), 0), // +0x760ba0 sub_760ba0
    Lifted::new(0x7000e4cdb0, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e154e0, 0x7000e01e10> as *const (), 0), // +0x760db0 sub_760db0
    Lifted::new(0x7000e4cfc0, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e158a0, 0x7000e01e10> as *const (), 0), // +0x760fc0 sub_760fc0
    Lifted::new(0x7000e4d1d0, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e15c60, 0x7000e01e10> as *const (), 0), // +0x7611d0 sub_7611d0
    Lifted::new(0x7000e4d3e0, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e16020, 0x7000e01e10> as *const (), 0), // +0x7613e0 sub_7613e0
    Lifted::new(0x7000e4d5f0, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e163e0, 0x7000e01e10> as *const (), 0), // +0x7615f0 sub_7615f0
    Lifted::new(0x7000e4d800, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e167a0, 0x7000e01e10> as *const (), 0), // +0x761800 sub_761800
    Lifted::new(0x7000e4da10, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e16b60, 0x7000e01e10> as *const (), 0), // +0x761a10 sub_761a10
    Lifted::new(0x7000e4dc20, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e16f20, 0x7000e01e10> as *const (), 0), // +0x761c20 sub_761c20
    Lifted::new(0x7000e4de30, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e172e0, 0x7000e01e10> as *const (), 0), // +0x761e30 sub_761e30
    Lifted::new(0x7000e4e040, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e176a0, 0x7000e01e10> as *const (), 0), // +0x762040 sub_762040
    Lifted::new(0x7000e4e250, tpl_1525::<0x7000aeb7a0, 0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x700076df00, 0x7000e17a60, 0x7000e01e10> as *const (), 0), // +0x762250 sub_762250
    Lifted::new(0x7000e4e460, fn_7000e4e460 as *const (), 2), // +0x762460 sub_762460
    Lifted::new(0x7000e4e890, tpl_1526::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x7000e48af0, 0x700076df00, 0x7000e17e20, 0x7000e01e10, 0x7000ef1da0> as *const (), 0), // +0x762890 sub_762890
    Lifted::new(0x7000e4ead0, tpl_1526::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x7000e48b90, 0x700076df00, 0x7000e18240, 0x7000e01e10, 0x7000ef1da0> as *const (), 0), // +0x762ad0 sub_762ad0
    Lifted::new(0x7000e4ed10, tpl_1526::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x7000e48c50, 0x700076df00, 0x7000e18650, 0x7000e01e10, 0x7000ef1da0> as *const (), 0), // +0x762d10 sub_762d10
    Lifted::new(0x7000e4ef50, tpl_1526::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x7000e48d10, 0x700076df00, 0x7000e18a60, 0x7000e01e10, 0x7000ef1da0> as *const (), 0), // +0x762f50 sub_762f50
    Lifted::new(0x7000e4f190, tpl_1526::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x7000e48dd0, 0x700076df00, 0x7000e18e70, 0x7000e01e10, 0x7000ef1da0> as *const (), 0), // +0x763190 sub_763190
    Lifted::new(0x7000e4f3d0, tpl_1526::<0x700074a850, 0x700079cfd0, 0x700077c8e0, 0x7000e48e90, 0x700076df00, 0x7000e19280, 0x7000e01e10, 0x7000ef1da0> as *const (), 0), // +0x7633d0 sub_7633d0
    Lifted::new(0x7000e4f610, fn_7000e4f610 as *const (), 2), // +0x763610 sub_763610
    Lifted::new(0x7000e4fa30, fn_7000e4fa30 as *const (), 2), // +0x763a30 sub_763a30
    Lifted::new(0x7000e4fe50, tpl_1527::<0x7000e50a80, 0x7000e4fe6d> as *const (), 0), // +0x763e50 sub_763e50
    Lifted::new(0x7000e4fe80, tpl_1527::<0x7000e50aa0, 0x7000e4fe9d> as *const (), 0), // +0x763e80 sub_763e80
    Lifted::new(0x7000e4feb0, tpl_1527::<0x7000e50ac0, 0x7000e4fecd> as *const (), 0), // +0x763eb0 sub_763eb0
    Lifted::new(0x7000e4fee0, tpl_1527::<0x7000e50ae0, 0x7000e4fefd> as *const (), 0), // +0x763ee0 sub_763ee0
    Lifted::new(0x7000e4ff10, tpl_1527::<0x7000e50b00, 0x7000e4ff2d> as *const (), 0), // +0x763f10 sub_763f10
    Lifted::new(0x7000e4ff40, tpl_1527::<0x7000e50b20, 0x7000e4ff5d> as *const (), 0), // +0x763f40 sub_763f40
    Lifted::new(0x7000e4ff70, tpl_1527::<0x7000e50b40, 0x7000e4ff8d> as *const (), 0), // +0x763f70 sub_763f70
    Lifted::new(0x7000e4ffa0, tpl_1527::<0x7000e50b60, 0x7000e4ffbd> as *const (), 0), // +0x763fa0 sub_763fa0
    Lifted::new(0x7000e4ffd0, tpl_1527::<0x7000e50b80, 0x7000e4ffed> as *const (), 0), // +0x763fd0 sub_763fd0
    Lifted::new(0x7000e50000, tpl_1527::<0x7000e50ba0, 0x7000e5001d> as *const (), 0), // +0x764000 sub_764000
    Lifted::new(0x7000e50030, tpl_1527::<0x7000e50bc0, 0x7000e5004d> as *const (), 0), // +0x764030 sub_764030
    Lifted::new(0x7000e50060, tpl_1527::<0x7000e50be0, 0x7000e5007d> as *const (), 0), // +0x764060 sub_764060
    Lifted::new(0x7000e50090, tpl_1527::<0x7000e50c00, 0x7000e500ad> as *const (), 0), // +0x764090 sub_764090
    Lifted::new(0x7000e500c0, tpl_1527::<0x7000e50c20, 0x7000e500dd> as *const (), 0), // +0x7640c0 sub_7640c0
    Lifted::new(0x7000e500f0, tpl_1527::<0x7000e50c40, 0x7000e5010d> as *const (), 0), // +0x7640f0 sub_7640f0
    Lifted::new(0x7000e50120, tpl_1527::<0x7000e50c60, 0x7000e5013d> as *const (), 0), // +0x764120 sub_764120
    Lifted::new(0x7000e50150, tpl_1527::<0x7000e50c80, 0x7000e5016d> as *const (), 0), // +0x764150 sub_764150
    Lifted::new(0x7000e50180, tpl_1527::<0x7000e50ca0, 0x7000e5019d> as *const (), 0), // +0x764180 sub_764180
    Lifted::new(0x7000e501b0, tpl_1527::<0x7000e50cc0, 0x7000e501cd> as *const (), 0), // +0x7641b0 sub_7641b0
    Lifted::new(0x7000e501e0, tpl_1527::<0x7000e50ce0, 0x7000e501fd> as *const (), 0), // +0x7641e0 sub_7641e0
    Lifted::new(0x7000e50210, tpl_1527::<0x7000e50d00, 0x7000e5022d> as *const (), 0), // +0x764210 sub_764210
    Lifted::new(0x7000e50240, tpl_1527::<0x7000e50d20, 0x7000e5025d> as *const (), 0), // +0x764240 sub_764240
    Lifted::new(0x7000e50270, tpl_1527::<0x7000e50d40, 0x7000e5028d> as *const (), 0), // +0x764270 sub_764270
    Lifted::new(0x7000e502a0, tpl_1528::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcbdd0> as *const (), 0), // +0x7642a0 sub_7642a0
    Lifted::new(0x7000e503a0, tpl_1527::<0x7000e50de0, 0x7000e503bd> as *const (), 0), // +0x7643a0 sub_7643a0
    Lifted::new(0x7000e503d0, tpl_1527::<0x7000e50e20, 0x7000e503ed> as *const (), 0), // +0x7643d0 sub_7643d0
    Lifted::new(0x7000e50400, tpl_1528::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcc7a0> as *const (), 0), // +0x764400 sub_764400
    Lifted::new(0x7000e50500, tpl_1527::<0x7000e50e60, 0x7000e5051d> as *const (), 0), // +0x764500 sub_764500
    Lifted::new(0x7000e50530, tpl_1528::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcce40> as *const (), 0), // +0x764530 sub_764530
    Lifted::new(0x7000e50630, tpl_1528::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcd190> as *const (), 0), // +0x764630 sub_764630
    Lifted::new(0x7000e50730, tpl_1527::<0x7000e50e00, 0x7000e5074d> as *const (), 0), // +0x764730 sub_764730
    Lifted::new(0x7000e50760, tpl_1527::<0x7000e50e40, 0x7000e5077d> as *const (), 0), // +0x764760 sub_764760
    Lifted::new(0x7000e50790, tpl_1528::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcdba0> as *const (), 0), // +0x764790 sub_764790
    Lifted::new(0x7000e50890, tpl_1527::<0x7000e50e80, 0x7000e508ad> as *const (), 0), // +0x764890 sub_764890
    Lifted::new(0x7000e508c0, tpl_1528::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dce280> as *const (), 0), // +0x7648c0 sub_7648c0
    Lifted::new(0x7000e509c0, tpl_1527::<0x7000e50d60, 0x7000e509dd> as *const (), 0), // +0x7649c0 sub_7649c0
    Lifted::new(0x7000e509f0, tpl_1527::<0x7000e50d80, 0x7000e50a0d> as *const (), 0), // +0x7649f0 sub_7649f0
    Lifted::new(0x7000e50a20, tpl_1527::<0x7000e50da0, 0x7000e50a3d> as *const (), 0), // +0x764a20 sub_764a20
    Lifted::new(0x7000e50a50, tpl_1527::<0x7000e50dc0, 0x7000e50a6d> as *const (), 0), // +0x764a50 sub_764a50
    Lifted::new(0x7000e50a80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50a91> as *const (), 0), // +0x764a80 sub_764a80
    Lifted::new(0x7000e50aa0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50ab1> as *const (), 0), // +0x764aa0 sub_764aa0
    Lifted::new(0x7000e50ac0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50ad1> as *const (), 0), // +0x764ac0 sub_764ac0
    Lifted::new(0x7000e50ae0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50af1> as *const (), 0), // +0x764ae0 sub_764ae0
    Lifted::new(0x7000e50b00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50b11> as *const (), 0), // +0x764b00 sub_764b00
    Lifted::new(0x7000e50b20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50b31> as *const (), 0), // +0x764b20 sub_764b20
    Lifted::new(0x7000e50b40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50b51> as *const (), 0), // +0x764b40 sub_764b40
    Lifted::new(0x7000e50b60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50b71> as *const (), 0), // +0x764b60 sub_764b60
    Lifted::new(0x7000e50b80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50b91> as *const (), 0), // +0x764b80 sub_764b80
    Lifted::new(0x7000e50ba0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50bb1> as *const (), 0), // +0x764ba0 sub_764ba0
    Lifted::new(0x7000e50bc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50bd1> as *const (), 0), // +0x764bc0 sub_764bc0
    Lifted::new(0x7000e50be0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50bf1> as *const (), 0), // +0x764be0 sub_764be0
    Lifted::new(0x7000e50c00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50c11> as *const (), 0), // +0x764c00 sub_764c00
    Lifted::new(0x7000e50c20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50c31> as *const (), 0), // +0x764c20 sub_764c20
    Lifted::new(0x7000e50c40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50c51> as *const (), 0), // +0x764c40 sub_764c40
    Lifted::new(0x7000e50c60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50c71> as *const (), 0), // +0x764c60 sub_764c60
    Lifted::new(0x7000e50c80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50c91> as *const (), 0), // +0x764c80 sub_764c80
    Lifted::new(0x7000e50ca0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50cb1> as *const (), 0), // +0x764ca0 sub_764ca0
    Lifted::new(0x7000e50cc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50cd1> as *const (), 0), // +0x764cc0 sub_764cc0
    Lifted::new(0x7000e50ce0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50cf1> as *const (), 0), // +0x764ce0 sub_764ce0
    Lifted::new(0x7000e50d00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50d11> as *const (), 0), // +0x764d00 sub_764d00
    Lifted::new(0x7000e50d20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50d31> as *const (), 0), // +0x764d20 sub_764d20
    Lifted::new(0x7000e50d40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50d51> as *const (), 0), // +0x764d40 sub_764d40
    Lifted::new(0x7000e50d60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50d71> as *const (), 0), // +0x764d60 sub_764d60
    Lifted::new(0x7000e50d80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50d91> as *const (), 0), // +0x764d80 sub_764d80
    Lifted::new(0x7000e50da0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50db1> as *const (), 0), // +0x764da0 sub_764da0
    Lifted::new(0x7000e50dc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50dd1> as *const (), 0), // +0x764dc0 sub_764dc0
    Lifted::new(0x7000e50de0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50df1> as *const (), 0), // +0x764de0 sub_764de0
    Lifted::new(0x7000e50e00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50e11> as *const (), 0), // +0x764e00 sub_764e00
    Lifted::new(0x7000e50e20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50e31> as *const (), 0), // +0x764e20 sub_764e20
    Lifted::new(0x7000e50e40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50e51> as *const (), 0), // +0x764e40 sub_764e40
    Lifted::new(0x7000e50e60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50e71> as *const (), 0), // +0x764e60 sub_764e60
    Lifted::new(0x7000e50e80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e50e91> as *const (), 0), // +0x764e80 sub_764e80
    Lifted::new(0x7000e50ea0, tpl_1529::<0x7000e51fc0, 0x7000e50ec8> as *const (), 0), // +0x764ea0 sub_764ea0
    Lifted::new(0x7000e50ee0, tpl_1529::<0x7000e51fe0, 0x7000e50f08> as *const (), 0), // +0x764ee0 sub_764ee0
    Lifted::new(0x7000e50f20, tpl_1529::<0x7000e52000, 0x7000e50f48> as *const (), 0), // +0x764f20 sub_764f20
    Lifted::new(0x7000e50f60, tpl_1529::<0x7000e52020, 0x7000e50f88> as *const (), 0), // +0x764f60 sub_764f60
    Lifted::new(0x7000e50fa0, tpl_1529::<0x7000e52040, 0x7000e50fc8> as *const (), 0), // +0x764fa0 sub_764fa0
    Lifted::new(0x7000e50fe0, tpl_1529::<0x7000e52060, 0x7000e51008> as *const (), 0), // +0x764fe0 sub_764fe0
    Lifted::new(0x7000e51020, tpl_1529::<0x7000e52080, 0x7000e51048> as *const (), 0), // +0x765020 sub_765020
    Lifted::new(0x7000e51060, tpl_1529::<0x7000e520a0, 0x7000e51088> as *const (), 0), // +0x765060 sub_765060
    Lifted::new(0x7000e510a0, tpl_1529::<0x7000e520c0, 0x7000e510c8> as *const (), 0), // +0x7650a0 sub_7650a0
    Lifted::new(0x7000e510e0, tpl_1529::<0x7000e520e0, 0x7000e51108> as *const (), 0), // +0x7650e0 sub_7650e0
    Lifted::new(0x7000e51120, tpl_1529::<0x7000e52100, 0x7000e51148> as *const (), 0), // +0x765120 sub_765120
    Lifted::new(0x7000e51160, tpl_1529::<0x7000e52120, 0x7000e51188> as *const (), 0), // +0x765160 sub_765160
    Lifted::new(0x7000e511a0, tpl_1529::<0x7000e52140, 0x7000e511c8> as *const (), 0), // +0x7651a0 sub_7651a0
    Lifted::new(0x7000e511e0, tpl_1529::<0x7000e52160, 0x7000e51208> as *const (), 0), // +0x7651e0 sub_7651e0
    Lifted::new(0x7000e51220, tpl_1529::<0x7000e52180, 0x7000e51248> as *const (), 0), // +0x765220 sub_765220
    Lifted::new(0x7000e51260, tpl_1529::<0x7000e521a0, 0x7000e51288> as *const (), 0), // +0x765260 sub_765260
    Lifted::new(0x7000e512a0, tpl_1529::<0x7000e521c0, 0x7000e512c8> as *const (), 0), // +0x7652a0 sub_7652a0
    Lifted::new(0x7000e512e0, tpl_1529::<0x7000e521e0, 0x7000e51308> as *const (), 0), // +0x7652e0 sub_7652e0
    Lifted::new(0x7000e51320, tpl_1529::<0x7000e52200, 0x7000e51348> as *const (), 0), // +0x765320 sub_765320
    Lifted::new(0x7000e51360, tpl_1529::<0x7000e52220, 0x7000e51388> as *const (), 0), // +0x765360 sub_765360
    Lifted::new(0x7000e513a0, tpl_1529::<0x7000e52240, 0x7000e513c8> as *const (), 0), // +0x7653a0 sub_7653a0
    Lifted::new(0x7000e513e0, tpl_1529::<0x7000e52260, 0x7000e51408> as *const (), 0), // +0x7653e0 sub_7653e0
    Lifted::new(0x7000e51420, tpl_1530::<0x7000e523d0> as *const (), 0), // +0x765420 sub_765420
    Lifted::new(0x7000e51450, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x765450 sub_765450
    Lifted::new(0x7000e51550, tpl_1529::<0x7000e52390, 0x7000e51578> as *const (), 0), // +0x765550 sub_765550
    Lifted::new(0x7000e51590, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x765590 sub_765590
    Lifted::new(0x7000e51690, tpl_1530::<0x7000e52280> as *const (), 0), // +0x765690 sub_765690
    Lifted::new(0x7000e516c0, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x7656c0 sub_7656c0
    Lifted::new(0x7000e517c0, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x7657c0 sub_7657c0
    Lifted::new(0x7000e518c0, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x7658c0 sub_7658c0
    Lifted::new(0x7000e519c0, tpl_1529::<0x7000e523b0, 0x7000e519e8> as *const (), 0), // +0x7659c0 sub_7659c0
    Lifted::new(0x7000e51a00, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x765a00 sub_765a00
    Lifted::new(0x7000e51b00, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x765b00 sub_765b00
    Lifted::new(0x7000e51c00, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x765c00 sub_765c00
    Lifted::new(0x7000e51d00, tpl_1531::<0x7000721d60, 0x10, 0xffffffffffff6918, 0x430, 0xffffffffffff68d0, 0x7, 0x700073e310, 0x7000e523d0> as *const (), 0), // +0x765d00 sub_765d00
    Lifted::new(0x7000e51e00, fn_7000e51e00 as *const (), 0), // +0x765e00 sub_765e00
    Lifted::new(0x7000e51ed0, tpl_1532::<0xb, 0x7, 0x7000e52280> as *const (), 0), // +0x765ed0 sub_765ed0
    Lifted::new(0x7000e51f30, tpl_1530::<0x7000e52490> as *const (), 0), // +0x765f30 sub_765f30
    Lifted::new(0x7000e51f60, tpl_1532::<0xb, 0x7, 0x7000e523d0> as *const (), 0), // +0x765f60 sub_765f60
    Lifted::new(0x7000e51fc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e51fd1> as *const (), 0), // +0x765fc0 sub_765fc0
    Lifted::new(0x7000e51fe0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e51ff1> as *const (), 0), // +0x765fe0 sub_765fe0
    Lifted::new(0x7000e52000, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52011> as *const (), 0), // +0x766000 sub_766000
    Lifted::new(0x7000e52020, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52031> as *const (), 0), // +0x766020 sub_766020
    Lifted::new(0x7000e52040, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52051> as *const (), 0), // +0x766040 sub_766040
    Lifted::new(0x7000e52060, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52071> as *const (), 0), // +0x766060 sub_766060
    Lifted::new(0x7000e52080, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52091> as *const (), 0), // +0x766080 sub_766080
    Lifted::new(0x7000e520a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e520b1> as *const (), 0), // +0x7660a0 sub_7660a0
    Lifted::new(0x7000e520c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e520d1> as *const (), 0), // +0x7660c0 sub_7660c0
    Lifted::new(0x7000e520e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e520f1> as *const (), 0), // +0x7660e0 sub_7660e0
    Lifted::new(0x7000e52100, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52111> as *const (), 0), // +0x766100 sub_766100
    Lifted::new(0x7000e52120, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52131> as *const (), 0), // +0x766120 sub_766120
    Lifted::new(0x7000e52140, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52151> as *const (), 0), // +0x766140 sub_766140
    Lifted::new(0x7000e52160, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52171> as *const (), 0), // +0x766160 sub_766160
    Lifted::new(0x7000e52180, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52191> as *const (), 0), // +0x766180 sub_766180
    Lifted::new(0x7000e521a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e521b1> as *const (), 0), // +0x7661a0 sub_7661a0
    Lifted::new(0x7000e521c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e521d1> as *const (), 0), // +0x7661c0 sub_7661c0
    Lifted::new(0x7000e521e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e521f1> as *const (), 0), // +0x7661e0 sub_7661e0
    Lifted::new(0x7000e52200, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52211> as *const (), 0), // +0x766200 sub_766200
    Lifted::new(0x7000e52220, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52231> as *const (), 0), // +0x766220 sub_766220
    Lifted::new(0x7000e52240, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52251> as *const (), 0), // +0x766240 sub_766240
    Lifted::new(0x7000e52260, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e52271> as *const (), 0), // +0x766260 sub_766260
    Lifted::new(0x7000e52280, fn_7000e52280 as *const (), 0), // +0x766280 sub_766280
    Lifted::new(0x7000e52390, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e523a1> as *const (), 0), // +0x766390 sub_766390
    Lifted::new(0x7000e523b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e523c1> as *const (), 0), // +0x7663b0 sub_7663b0
    Lifted::new(0x7000e523d0, fn_7000e523d0 as *const (), 0), // +0x7663d0 sub_7663d0
    Lifted::new(0x7000e52490, fn_7000e52490 as *const (), 2), // +0x766490 sub_766490
    Lifted::new(0x7000e52910, fn_7000e52910 as *const (), 2), // +0x766910 sub_766910
    Lifted::new(0x7000e52ae0, tpl_1533::<0x70006f2a70, 0x70006f9a60, 0x7000e43000, 0x7000e5c620> as *const (), 0), // +0x766ae0 sub_766ae0
    Lifted::new(0x7000e52ce0, tpl_1534::<0x7000e84980, 0x7000745eb0, 0x70006f2a70, 0x70006f9a60, 0x7000e432a0, 0x7000e5a1e0> as *const (), 0), // +0x766ce0 sub_766ce0
    Lifted::new(0x7000e52f50, tpl_1534::<0x7000e84980, 0x7000745eb0, 0x70006f2a70, 0x70006f9a60, 0x7000e437e0, 0x7000e5a740> as *const (), 0), // +0x766f50 sub_766f50
    Lifted::new(0x7000e531c0, fn_7000e531c0 as *const (), 2), // +0x7671c0 sub_7671c0
    Lifted::new(0x7000e53370, tpl_1533::<0x70006f2a70, 0x70006f9a60, 0x7000e43eb0, 0x7000e5ca50> as *const (), 0), // +0x767370 sub_767370
    Lifted::new(0x7000e53570, tpl_1535::<0x7000e84980, 0x7000745eb0, 0x70006f2a70, 0x70006f9a60, 0x7000e44150, 0x7000e5b120> as *const (), 0), // +0x767570 sub_767570
    Lifted::new(0x7000e537c0, tpl_1535::<0x7000e84980, 0x7000745eb0, 0x70006f2a70, 0x70006f9a60, 0x7000e44690, 0x7000e5b6a0> as *const (), 0), // +0x7677c0 sub_7677c0
    Lifted::new(0x7000e53a10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e53a21> as *const (), 0), // +0x767a10 sub_767a10
    Lifted::new(0x7000e53a30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e53a41> as *const (), 0), // +0x767a30 sub_767a30
    Lifted::new(0x7000e53a50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e53a61> as *const (), 0), // +0x767a50 sub_767a50
    Lifted::new(0x7000e53a70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e53a81> as *const (), 0), // +0x767a70 sub_767a70
    Lifted::new(0x7000e53a90, tpl_1536::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x70017798a0> as *const (), 0), // +0x767a90 sub_767a90
    Lifted::new(0x7000e53b10, tpl_1536::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x70017798a0> as *const (), 0), // +0x767b10 sub_767b10
    Lifted::new(0x7000e53b90, tpl_1537::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e53c10, 0x70017799b0> as *const (), 0), // +0x767b90 sub_767b90
    Lifted::new(0x7000e53c50, tpl_1537::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e53cd0, 0x70017799b0> as *const (), 0), // +0x767c50 sub_767c50
    Lifted::new(0x7000e53d10, tpl_1538::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0x7001779ad0> as *const (), 0), // +0x767d10 sub_767d10
    Lifted::new(0x7000e53da0, tpl_1538::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0x7001779ad0> as *const (), 0), // +0x767da0 sub_767da0
    Lifted::new(0x7000e53e30, tpl_1539::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0xfffffffffffffffc, 0x7001779ad0> as *const (), 2), // +0x767e30 sub_767e30
    Lifted::new(0x7000e53f40, tpl_432::<0x7000e54880> as *const (), 0), // +0x767f40 sub_767f40
    Lifted::new(0x7000e53f50, tpl_1536::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x70017798a0> as *const (), 0), // +0x767f50 sub_767f50
    Lifted::new(0x7000e53fd0, tpl_432::<0x7000e54ac0> as *const (), 0), // +0x767fd0 sub_767fd0
    Lifted::new(0x7000e53fe0, tpl_432::<0x7000e54cb0> as *const (), 0), // +0x767fe0 sub_767fe0
    Lifted::new(0x7000e53ff0, tpl_1536::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x70017798a0> as *const (), 0), // +0x767ff0 sub_767ff0
    Lifted::new(0x7000e54070, tpl_1536::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x70017798a0> as *const (), 0), // +0x768070 sub_768070
    Lifted::new(0x7000e540f0, tpl_1537::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e54170, 0x70017799b0> as *const (), 0), // +0x7680f0 sub_7680f0
    Lifted::new(0x7000e541b0, tpl_1537::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e54230, 0x70017799b0> as *const (), 0), // +0x7681b0 sub_7681b0
    Lifted::new(0x7000e54270, tpl_1538::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0x7001779ad0> as *const (), 0), // +0x768270 sub_768270
    Lifted::new(0x7000e54300, tpl_1538::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0x7001779ad0> as *const (), 0), // +0x768300 sub_768300
    Lifted::new(0x7000e54390, tpl_1539::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0xfffffffffffffffc, 0x7001779ad0> as *const (), 2), // +0x768390 sub_768390
    Lifted::new(0x7000e544a0, tpl_432::<0x7000e54ea0> as *const (), 0), // +0x7684a0 sub_7684a0
    Lifted::new(0x7000e544b0, tpl_1536::<0x1f, 0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x70017798a0> as *const (), 0), // +0x7684b0 sub_7684b0
    Lifted::new(0x7000e54530, tpl_432::<0x7000e550e0> as *const (), 0), // +0x768530 sub_768530
    Lifted::new(0x7000e54540, tpl_432::<0x7000e552d0> as *const (), 0), // +0x768540 sub_768540
    Lifted::new(0x7000e54550, tpl_1540::<0x7000e54660, 0x7000e5455c> as *const (), 0), // +0x768550 sub_768550
    Lifted::new(0x7000e54560, tpl_1540::<0x7000e54680, 0x7000e5456c> as *const (), 0), // +0x768560 sub_768560
    Lifted::new(0x7000e54570, tpl_1540::<0x7000e546a0, 0x7000e5457c> as *const (), 0), // +0x768570 sub_768570
    Lifted::new(0x7000e54580, tpl_1540::<0x7000e546c0, 0x7000e5458c> as *const (), 0), // +0x768580 sub_768580
    Lifted::new(0x7000e54590, tpl_1540::<0x7000e546e0, 0x7000e5459c> as *const (), 0), // +0x768590 sub_768590
    Lifted::new(0x7000e545a0, tpl_1540::<0x7000e54700, 0x7000e545ac> as *const (), 0), // +0x7685a0 sub_7685a0
    Lifted::new(0x7000e545b0, tpl_1540::<0x7000e54720, 0x7000e545bc> as *const (), 0), // +0x7685b0 sub_7685b0
    Lifted::new(0x7000e545c0, tpl_1540::<0x7000e54740, 0x7000e545cc> as *const (), 0), // +0x7685c0 sub_7685c0
    Lifted::new(0x7000e545d0, tpl_1540::<0x7000e54760, 0x7000e545dc> as *const (), 0), // +0x7685d0 sub_7685d0
    Lifted::new(0x7000e545e0, tpl_1540::<0x7000e54780, 0x7000e545ec> as *const (), 0), // +0x7685e0 sub_7685e0
    Lifted::new(0x7000e545f0, tpl_1540::<0x7000e547a0, 0x7000e545fc> as *const (), 0), // +0x7685f0 sub_7685f0
    Lifted::new(0x7000e54600, tpl_1540::<0x7000e547c0, 0x7000e5460c> as *const (), 0), // +0x768600 sub_768600
    Lifted::new(0x7000e54610, tpl_1540::<0x7000e547e0, 0x7000e5461c> as *const (), 0), // +0x768610 sub_768610
    Lifted::new(0x7000e54620, tpl_1540::<0x7000e54800, 0x7000e5462c> as *const (), 0), // +0x768620 sub_768620
    Lifted::new(0x7000e54630, tpl_1540::<0x7000e54820, 0x7000e5463c> as *const (), 0), // +0x768630 sub_768630
    Lifted::new(0x7000e54640, tpl_1540::<0x7000e54840, 0x7000e5464c> as *const (), 0), // +0x768640 sub_768640
    Lifted::new(0x7000e54650, tpl_1540::<0x7000e54860, 0x7000e5465c> as *const (), 0), // +0x768650 sub_768650
    Lifted::new(0x7000e54660, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54671> as *const (), 0), // +0x768660 sub_768660
    Lifted::new(0x7000e54680, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54691> as *const (), 0), // +0x768680 sub_768680
    Lifted::new(0x7000e546a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e546b1> as *const (), 0), // +0x7686a0 sub_7686a0
    Lifted::new(0x7000e546c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e546d1> as *const (), 0), // +0x7686c0 sub_7686c0
    Lifted::new(0x7000e546e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e546f1> as *const (), 0), // +0x7686e0 sub_7686e0
    Lifted::new(0x7000e54700, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54711> as *const (), 0), // +0x768700 sub_768700
    Lifted::new(0x7000e54720, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54731> as *const (), 0), // +0x768720 sub_768720
    Lifted::new(0x7000e54740, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54751> as *const (), 0), // +0x768740 sub_768740
    Lifted::new(0x7000e54760, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54771> as *const (), 0), // +0x768760 sub_768760
    Lifted::new(0x7000e54780, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54791> as *const (), 0), // +0x768780 sub_768780
    Lifted::new(0x7000e547a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e547b1> as *const (), 0), // +0x7687a0 sub_7687a0
    Lifted::new(0x7000e547c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e547d1> as *const (), 0), // +0x7687c0 sub_7687c0
    Lifted::new(0x7000e547e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e547f1> as *const (), 0), // +0x7687e0 sub_7687e0
    Lifted::new(0x7000e54800, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54811> as *const (), 0), // +0x768800 sub_768800
    Lifted::new(0x7000e54820, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54831> as *const (), 0), // +0x768820 sub_768820
    Lifted::new(0x7000e54840, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54851> as *const (), 0), // +0x768840 sub_768840
    Lifted::new(0x7000e54860, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e54871> as *const (), 0), // +0x768860 sub_768860
    Lifted::new(0x7000e54880, tpl_1541 as *const (), 3), // +0x768880 sub_768880
    Lifted::new(0x7000e54ac0, tpl_1542 as *const (), 1), // +0x768ac0 sub_768ac0
    Lifted::new(0x7000e54cb0, tpl_1542 as *const (), 1), // +0x768cb0 sub_768cb0
    Lifted::new(0x7000e54ea0, tpl_1541 as *const (), 3), // +0x768ea0 sub_768ea0
    Lifted::new(0x7000e550e0, tpl_1542 as *const (), 1), // +0x7690e0 sub_7690e0
    Lifted::new(0x7000e552d0, tpl_1542 as *const (), 1), // +0x7692d0 sub_7692d0
    Lifted::new(0x7000e554c0, tpl_1543::<0x7000e566f0> as *const (), 0), // +0x7694c0 sub_7694c0
    Lifted::new(0x7000e554e0, tpl_1543::<0x7000e56790> as *const (), 0), // +0x7694e0 sub_7694e0
    Lifted::new(0x7000e55500, tpl_1543::<0x7000e56830> as *const (), 0), // +0x769500 sub_769500
    Lifted::new(0x7000e55520, tpl_1543::<0x7000e568f0> as *const (), 0), // +0x769520 sub_769520
    Lifted::new(0x7000e55540, tpl_1543::<0x7000e569b0> as *const (), 0), // +0x769540 sub_769540
    Lifted::new(0x7000e55560, tpl_1543::<0x7000e56a50> as *const (), 0), // +0x769560 sub_769560
    Lifted::new(0x7000e55580, tpl_1543::<0x7000e56af0> as *const (), 0), // +0x769580 sub_769580
    Lifted::new(0x7000e555a0, tpl_1543::<0x7000e56bb0> as *const (), 0), // +0x7695a0 sub_7695a0
    Lifted::new(0x7000e555c0, tpl_1543::<0x7000e56c90> as *const (), 0), // +0x7695c0 sub_7695c0
    Lifted::new(0x7000e555e0, tpl_1543::<0x7000e56d60> as *const (), 0), // +0x7695e0 sub_7695e0
    Lifted::new(0x7000e55600, tpl_1543::<0x7000e56e10> as *const (), 0), // +0x769600 sub_769600
    Lifted::new(0x7000e55620, tpl_1543::<0x7000e56ec0> as *const (), 0), // +0x769620 sub_769620
    Lifted::new(0x7000e55640, tpl_1543::<0x7000e56f60> as *const (), 0), // +0x769640 sub_769640
    Lifted::new(0x7000e55660, tpl_1543::<0x7000e57000> as *const (), 0), // +0x769660 sub_769660
    Lifted::new(0x7000e55680, tpl_1543::<0x7000e570c0> as *const (), 0), // +0x769680 sub_769680
    Lifted::new(0x7000e556a0, tpl_1543::<0x7000e57180> as *const (), 0), // +0x7696a0 sub_7696a0
    Lifted::new(0x7000e556c0, tpl_1543::<0x7000e57220> as *const (), 0), // +0x7696c0 sub_7696c0
    Lifted::new(0x7000e556e0, tpl_1543::<0x7000e572c0> as *const (), 0), // +0x7696e0 sub_7696e0
    Lifted::new(0x7000e55700, tpl_1543::<0x7000e57380> as *const (), 0), // +0x769700 sub_769700
    Lifted::new(0x7000e55720, tpl_1543::<0x7000e57460> as *const (), 0), // +0x769720 sub_769720
    Lifted::new(0x7000e55740, tpl_1543::<0x7000e57530> as *const (), 0), // +0x769740 sub_769740
    Lifted::new(0x7000e55760, tpl_1543::<0x7000e575e0> as *const (), 0), // +0x769760 sub_769760
    Lifted::new(0x7000e55780, fn_7000e55780 as *const (), 0), // +0x769780 sub_769780
    Lifted::new(0x7000e557e0, tpl_1544::<0x7, 0x3, 0x70017d8e88> as *const (), 3), // +0x7697e0 sub_7697e0
    Lifted::new(0x7000e55860, tpl_1545::<0x7000e55b60, 0x7000e55875> as *const (), 0), // +0x769860 sub_769860
    Lifted::new(0x7000e55880, tpl_1543::<0x7000e55be0> as *const (), 0), // +0x769880 sub_769880
    Lifted::new(0x7000e558a0, tpl_1546::<0x7, 0x7000e55cb0> as *const (), 0), // +0x7698a0 sub_7698a0
    Lifted::new(0x7000e558d0, tpl_1543::<0x7000e55d70> as *const (), 0), // +0x7698d0 sub_7698d0
    Lifted::new(0x7000e558f0, tpl_1543::<0x7000e55e40> as *const (), 0), // +0x7698f0 sub_7698f0
    Lifted::new(0x7000e55910, tpl_1544::<0x7, 0x3, 0x70017d8e88> as *const (), 3), // +0x769910 sub_769910
    Lifted::new(0x7000e55990, tpl_1545::<0x7000e55ba0, 0x7000e559a5> as *const (), 0), // +0x769990 sub_769990
    Lifted::new(0x7000e559b0, tpl_1543::<0x7000e55f10> as *const (), 0), // +0x7699b0 sub_7699b0
    Lifted::new(0x7000e559d0, tpl_1543::<0x7000e55fe0> as *const (), 0), // +0x7699d0 sub_7699d0
    Lifted::new(0x7000e559f0, tpl_1543::<0x7000e560b0> as *const (), 0), // +0x7699f0 sub_7699f0
    Lifted::new(0x7000e55a10, tpl_1543::<0x7000e56180> as *const (), 0), // +0x769a10 sub_769a10
    Lifted::new(0x7000e55a30, tpl_1546::<0x7, 0x7000e56250> as *const (), 0), // +0x769a30 sub_769a30
    Lifted::new(0x7000e55a60, fn_7000e55a60 as *const (), 0), // +0x769a60 sub_769a60
    Lifted::new(0x7000e55ab0, tpl_1546::<0x7, 0x7000e56490> as *const (), 0), // +0x769ab0 sub_769ab0
    Lifted::new(0x7000e55ae0, fn_7000e55ae0 as *const (), 0), // +0x769ae0 sub_769ae0
    Lifted::new(0x7000e55b60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e55b71> as *const (), 0), // +0x769b60 sub_769b60
    Lifted::new(0x7000e55b80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e55b91> as *const (), 0), // +0x769b80 sub_769b80
    Lifted::new(0x7000e55ba0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e55bb1> as *const (), 0), // +0x769ba0 sub_769ba0
    Lifted::new(0x7000e55bc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e55bd1> as *const (), 0), // +0x769bc0 sub_769bc0
    Lifted::new(0x7000e55be0, tpl_1547::<0x7, 0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x769be0 sub_769be0
    Lifted::new(0x7000e55cb0, fn_7000e55cb0 as *const (), 0), // +0x769cb0 sub_769cb0
    Lifted::new(0x7000e55d70, tpl_1547::<0x7, 0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x769d70 sub_769d70
    Lifted::new(0x7000e55e40, tpl_1547::<0x7, 0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x769e40 sub_769e40
    Lifted::new(0x7000e55f10, tpl_1547::<0x7, 0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x769f10 sub_769f10
    Lifted::new(0x7000e55fe0, tpl_1547::<0x7, 0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x769fe0 sub_769fe0
    Lifted::new(0x7000e560b0, tpl_1547::<0x7, 0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x76a0b0 sub_76a0b0
    Lifted::new(0x7000e56180, tpl_1547::<0x7, 0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x76a180 sub_76a180
    Lifted::new(0x7000e56250, fn_7000e56250 as *const (), 0), // +0x76a250 sub_76a250
    Lifted::new(0x7000e56490, fn_7000e56490 as *const (), 0), // +0x76a490 sub_76a490
    Lifted::new(0x7000e566f0, tpl_1548::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20> as *const (), 2), // +0x76a6f0 sub_76a6f0
    Lifted::new(0x7000e56790, tpl_1548::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20> as *const (), 2), // +0x76a790 sub_76a790
    Lifted::new(0x7000e56830, tpl_1549::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e568c6> as *const (), 2), // +0x76a830 sub_76a830
    Lifted::new(0x7000e568f0, tpl_1549::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e56986> as *const (), 2), // +0x76a8f0 sub_76a8f0
    Lifted::new(0x7000e569b0, tpl_1550::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20> as *const (), 2), // +0x76a9b0 sub_76a9b0
    Lifted::new(0x7000e56a50, tpl_1550::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20> as *const (), 2), // +0x76aa50 sub_76aa50
    Lifted::new(0x7000e56af0, tpl_1551::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x7000e20420> as *const (), 2), // +0x76aaf0 sub_76aaf0
    Lifted::new(0x7000e56bb0, tpl_1552::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x4> as *const (), 3), // +0x76abb0 sub_76abb0
    Lifted::new(0x7000e56c90, tpl_1553::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x7001855d18, 0x700177c260> as *const (), 2), // +0x76ac90 sub_76ac90
    Lifted::new(0x7000e56d60, tpl_1554::<0x2f, 0x27, 0xb, 0x2b, 0x7000db6760, 0x4> as *const (), 0), // +0x76ad60 sub_76ad60
    Lifted::new(0x7000e56e10, tpl_1554::<0x2f, 0x27, 0xb, 0x2b, 0x7000757d30, 0x4> as *const (), 0), // +0x76ae10 sub_76ae10
    Lifted::new(0x7000e56ec0, tpl_1548::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20> as *const (), 2), // +0x76aec0 sub_76aec0
    Lifted::new(0x7000e56f60, tpl_1548::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20> as *const (), 2), // +0x76af60 sub_76af60
    Lifted::new(0x7000e57000, tpl_1549::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e57096> as *const (), 2), // +0x76b000 sub_76b000
    Lifted::new(0x7000e570c0, tpl_1549::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20, 0x7001855bb8, 0x70017a0290, 0x7001d54ef8, 0x7000e57156> as *const (), 2), // +0x76b0c0 sub_76b0c0
    Lifted::new(0x7000e57180, tpl_1550::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20> as *const (), 2), // +0x76b180 sub_76b180
    Lifted::new(0x7000e57220, tpl_1550::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x70009e4b20> as *const (), 2), // +0x76b220 sub_76b220
    Lifted::new(0x7000e572c0, tpl_1551::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x7000e207a0> as *const (), 2), // +0x76b2c0 sub_76b2c0
    Lifted::new(0x7000e57380, tpl_1552::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x4> as *const (), 3), // +0x76b380 sub_76b380
    Lifted::new(0x7000e57460, tpl_1553::<0x2f, 0x27, 0xb, 0x2b, 0xffffffffffffffff, 0x7, 0x3, 0x7001855d18, 0x700177c260> as *const (), 2), // +0x76b460 sub_76b460
    Lifted::new(0x7000e57530, tpl_1554::<0x2f, 0x27, 0xb, 0x2b, 0x7000db6760, 0x4> as *const (), 0), // +0x76b530 sub_76b530
    Lifted::new(0x7000e575e0, tpl_1554::<0x2f, 0x27, 0xb, 0x2b, 0x7000757d30, 0x4> as *const (), 0), // +0x76b5e0 sub_76b5e0
    Lifted::new(0x7000e57690, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d720, 0x7000e5775c> as *const (), 0), // +0x76b690 sub_76b690
    Lifted::new(0x7000e57790, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d740, 0x7000e5785c> as *const (), 0), // +0x76b790 sub_76b790
    Lifted::new(0x7000e57890, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d760, 0x7000e5795c> as *const (), 0), // +0x76b890 sub_76b890
    Lifted::new(0x7000e57990, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d780, 0x7000e57a5c> as *const (), 0), // +0x76b990 sub_76b990
    Lifted::new(0x7000e57a90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d7a0, 0x7000e57b5c> as *const (), 0), // +0x76ba90 sub_76ba90
    Lifted::new(0x7000e57b90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d7c0, 0x7000e57c5c> as *const (), 0), // +0x76bb90 sub_76bb90
    Lifted::new(0x7000e57c90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d7e0, 0x7000e57d5c> as *const (), 0), // +0x76bc90 sub_76bc90
    Lifted::new(0x7000e57d90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d800, 0x7000e57e5c> as *const (), 0), // +0x76bd90 sub_76bd90
    Lifted::new(0x7000e57e90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d820, 0x7000e57f5c> as *const (), 0), // +0x76be90 sub_76be90
    Lifted::new(0x7000e57f90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d840, 0x7000e5805c> as *const (), 0), // +0x76bf90 sub_76bf90
    Lifted::new(0x7000e58090, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d860, 0x7000e5815c> as *const (), 0), // +0x76c090 sub_76c090
    Lifted::new(0x7000e58190, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d880, 0x7000e5825c> as *const (), 0), // +0x76c190 sub_76c190
    Lifted::new(0x7000e58290, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d8a0, 0x7000e5835c> as *const (), 0), // +0x76c290 sub_76c290
    Lifted::new(0x7000e58390, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d8c0, 0x7000e5845c> as *const (), 0), // +0x76c390 sub_76c390
    Lifted::new(0x7000e58490, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d8e0, 0x7000e5855c> as *const (), 0), // +0x76c490 sub_76c490
    Lifted::new(0x7000e58590, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d900, 0x7000e5865c> as *const (), 0), // +0x76c590 sub_76c590
    Lifted::new(0x7000e58690, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d920, 0x7000e5875c> as *const (), 0), // +0x76c690 sub_76c690
    Lifted::new(0x7000e58790, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d940, 0x7000e5885c> as *const (), 0), // +0x76c790 sub_76c790
    Lifted::new(0x7000e58890, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d960, 0x7000e5895c> as *const (), 0), // +0x76c890 sub_76c890
    Lifted::new(0x7000e58990, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d980, 0x7000e58a5c> as *const (), 0), // +0x76c990 sub_76c990
    Lifted::new(0x7000e58a90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d9a0, 0x7000e58b5c> as *const (), 0), // +0x76ca90 sub_76ca90
    Lifted::new(0x7000e58b90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d9c0, 0x7000e58c5c> as *const (), 0), // +0x76cb90 sub_76cb90
    Lifted::new(0x7000e58c90, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5d9e0> as *const (), 0), // +0x76cc90 sub_76cc90
    Lifted::new(0x7000e58d90, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e59d90> as *const (), 0), // +0x76cd90 sub_76cd90
    Lifted::new(0x7000e58e90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5c5e0, 0x7000e58f5c> as *const (), 0), // +0x76ce90 sub_76ce90
    Lifted::new(0x7000e58f90, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5c620> as *const (), 0), // +0x76cf90 sub_76cf90
    Lifted::new(0x7000e59090, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5a1e0> as *const (), 0), // +0x76d090 sub_76d090
    Lifted::new(0x7000e59190, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5ce80> as *const (), 0), // +0x76d190 sub_76d190
    Lifted::new(0x7000e59290, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5a740> as *const (), 0), // +0x76d290 sub_76d290
    Lifted::new(0x7000e59390, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5aca0> as *const (), 0), // +0x76d390 sub_76d390
    Lifted::new(0x7000e59490, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5c600, 0x7000e5955c> as *const (), 0), // +0x76d490 sub_76d490
    Lifted::new(0x7000e59590, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5ca50> as *const (), 0), // +0x76d590 sub_76d590
    Lifted::new(0x7000e59690, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5b120> as *const (), 0), // +0x76d690 sub_76d690
    Lifted::new(0x7000e59790, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5d2b0> as *const (), 0), // +0x76d790 sub_76d790
    Lifted::new(0x7000e59890, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5b6a0> as *const (), 0), // +0x76d890 sub_76d890
    Lifted::new(0x7000e59990, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d6e0, 0x7000e59a5c> as *const (), 0), // +0x76d990 sub_76d990
    Lifted::new(0x7000e59a90, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5bc20> as *const (), 0), // +0x76da90 sub_76da90
    Lifted::new(0x7000e59b90, tpl_1555::<0x7000dc9d20, 0x7000e25820, 0x7000d906c0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7000e5d700, 0x7000e59c5c> as *const (), 0), // +0x76db90 sub_76db90
    Lifted::new(0x7000e59c90, tpl_1556::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000e5c100> as *const (), 0), // +0x76dc90 sub_76dc90
    Lifted::new(0x7000e59d90, fn_7000e59d90 as *const (), 2), // +0x76dd90 sub_76dd90
    Lifted::new(0x7000e5a1e0, fn_7000e5a1e0 as *const (), 2), // +0x76e1e0 sub_76e1e0
    Lifted::new(0x7000e5a740, fn_7000e5a740 as *const (), 2), // +0x76e740 sub_76e740
    Lifted::new(0x7000e5aca0, fn_7000e5aca0 as *const (), 2), // +0x76eca0 sub_76eca0
    Lifted::new(0x7000e5b120, fn_7000e5b120 as *const (), 2), // +0x76f120 sub_76f120
    Lifted::new(0x7000e5b6a0, fn_7000e5b6a0 as *const (), 2), // +0x76f6a0 sub_76f6a0
    Lifted::new(0x7000e5bc20, tpl_1557::<0x7001855b58, 0x70017a0290, 0x7001d54ef8, 0x7000e5bc70, 0x70017d8e30, 0x700073b7a0, 0x7000e241b0, 0x7000748180, 0x7001c72d48, 0x70006f2a70, 0x70006f9a60, 0x7000778530, 0x7000de86b0, 0x7000e871d0, 0x7000e5de80, 0x7000e7f6e0, 0x7000e96450> as *const (), 2), // +0x76fc20 sub_76fc20
    Lifted::new(0x7000e5c100, tpl_1557::<0x7001855b58, 0x70017a0290, 0x7001d54ef8, 0x7000e5c150, 0x70017d8e30, 0x700073b7a0, 0x7000e241b0, 0x7000748180, 0x7001c72d48, 0x70006f2a70, 0x70006f9a60, 0x7000778530, 0x7000de8900, 0x7000e871d0, 0x7000e5de80, 0x7000e7f6e0, 0x7000e96450> as *const (), 2), // +0x770100 sub_770100
    Lifted::new(0x7000e5c5e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5c5f1> as *const (), 0), // +0x7705e0 sub_7705e0
    Lifted::new(0x7000e5c600, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5c611> as *const (), 0), // +0x770600 sub_770600
    Lifted::new(0x7000e5c620, tpl_1558::<0x7001855b58, 0x70017a0290, 0x7001d54ef8, 0x7000e5c673, 0x70017d8e30, 0x7001c73200, 0x70006f2a70, 0x70006f9a60, 0x7001855cc0, 0x7000eb6b00, 0x7000720d30, 0x7000767d50, 0x700074a010, 0x7000737930, 0x7000e91da0, 0x7000e7cae0, 0x7000e5d9e0> as *const (), 2), // +0x770620 sub_770620
    Lifted::new(0x7000e5ca50, tpl_1558::<0x7001855b58, 0x70017a0290, 0x7001d54ef8, 0x7000e5caa3, 0x70017d8e30, 0x7001c73200, 0x70006f2a70, 0x70006f9a60, 0x7001855cc0, 0x7000eb6b00, 0x7000720d30, 0x7000767d50, 0x700074a010, 0x7000737930, 0x7000e91da0, 0x7000e7cae0, 0x7000e5d9e0> as *const (), 2), // +0x770a50 sub_770a50
    Lifted::new(0x7000e5ce80, tpl_1559::<0x7001855b58, 0x70017a0290, 0x7001d54ef8, 0x7000e5ced3, 0x70017d8e30, 0x7001c73200, 0x70006f2a70, 0x70006f9a60, 0x7001855cc0, 0x7000eb6b00, 0x7000720d30, 0x7000767d50, 0x700074a010, 0x7000737930, 0x7000e91da0, 0x7000e7cae0, 0x7000e5d9e0> as *const (), 2), // +0x770e80 sub_770e80
    Lifted::new(0x7000e5d2b0, tpl_1559::<0x7001855b58, 0x70017a0290, 0x7001d54ef8, 0x7000e5d303, 0x70017d8e30, 0x7001c73200, 0x70006f2a70, 0x70006f9a60, 0x7001855cc0, 0x7000eb6b00, 0x7000720d30, 0x7000767d50, 0x700074a010, 0x7000737930, 0x7000e91da0, 0x7000e7cae0, 0x7000e5d9e0> as *const (), 2), // +0x7712b0 sub_7712b0
    Lifted::new(0x7000e5d6e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d6f1> as *const (), 0), // +0x7716e0 sub_7716e0
    Lifted::new(0x7000e5d700, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d711> as *const (), 0), // +0x771700 sub_771700
    Lifted::new(0x7000e5d720, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d731> as *const (), 0), // +0x771720 sub_771720
    Lifted::new(0x7000e5d740, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d751> as *const (), 0), // +0x771740 sub_771740
    Lifted::new(0x7000e5d760, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d771> as *const (), 0), // +0x771760 sub_771760
    Lifted::new(0x7000e5d780, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d791> as *const (), 0), // +0x771780 sub_771780
    Lifted::new(0x7000e5d7a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d7b1> as *const (), 0), // +0x7717a0 sub_7717a0
    Lifted::new(0x7000e5d7c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d7d1> as *const (), 0), // +0x7717c0 sub_7717c0
    Lifted::new(0x7000e5d7e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d7f1> as *const (), 0), // +0x7717e0 sub_7717e0
    Lifted::new(0x7000e5d800, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d811> as *const (), 0), // +0x771800 sub_771800
    Lifted::new(0x7000e5d820, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d831> as *const (), 0), // +0x771820 sub_771820
    Lifted::new(0x7000e5d840, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d851> as *const (), 0), // +0x771840 sub_771840
    Lifted::new(0x7000e5d860, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d871> as *const (), 0), // +0x771860 sub_771860
    Lifted::new(0x7000e5d880, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d891> as *const (), 0), // +0x771880 sub_771880
    Lifted::new(0x7000e5d8a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d8b1> as *const (), 0), // +0x7718a0 sub_7718a0
    Lifted::new(0x7000e5d8c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d8d1> as *const (), 0), // +0x7718c0 sub_7718c0
    Lifted::new(0x7000e5d8e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d8f1> as *const (), 0), // +0x7718e0 sub_7718e0
    Lifted::new(0x7000e5d900, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d911> as *const (), 0), // +0x771900 sub_771900
    Lifted::new(0x7000e5d920, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d931> as *const (), 0), // +0x771920 sub_771920
    Lifted::new(0x7000e5d940, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d951> as *const (), 0), // +0x771940 sub_771940
    Lifted::new(0x7000e5d960, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d971> as *const (), 0), // +0x771960 sub_771960
    Lifted::new(0x7000e5d980, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d991> as *const (), 0), // +0x771980 sub_771980
    Lifted::new(0x7000e5d9a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d9b1> as *const (), 0), // +0x7719a0 sub_7719a0
    Lifted::new(0x7000e5d9c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5d9d1> as *const (), 0), // +0x7719c0 sub_7719c0
    Lifted::new(0x7000e5d9e0, fn_7000e5d9e0 as *const (), 2), // +0x7719e0 sub_7719e0
    Lifted::new(0x7000e5de80, fn_7000e5de80 as *const (), 0), // +0x771e80 sub_771e80
    Lifted::new(0x7000e5df60, tpl_1426::<0x7000e5e4c0, 0x7000e5df6f> as *const (), 0), // +0x771f60 sub_771f60
    Lifted::new(0x7000e5df80, tpl_1426::<0x7000e5e4e0, 0x7000e5df8f> as *const (), 0), // +0x771f80 sub_771f80
    Lifted::new(0x7000e5dfa0, tpl_1426::<0x7000e5e500, 0x7000e5dfaf> as *const (), 0), // +0x771fa0 sub_771fa0
    Lifted::new(0x7000e5dfc0, tpl_1426::<0x7000e5e520, 0x7000e5dfcf> as *const (), 0), // +0x771fc0 sub_771fc0
    Lifted::new(0x7000e5dfe0, tpl_1426::<0x7000e5e540, 0x7000e5dfef> as *const (), 0), // +0x771fe0 sub_771fe0
    Lifted::new(0x7000e5e000, tpl_1426::<0x7000e5e560, 0x7000e5e00f> as *const (), 0), // +0x772000 sub_772000
    Lifted::new(0x7000e5e020, tpl_1426::<0x7000e5e580, 0x7000e5e02f> as *const (), 0), // +0x772020 sub_772020
    Lifted::new(0x7000e5e040, tpl_1426::<0x7000e5e5a0, 0x7000e5e04f> as *const (), 0), // +0x772040 sub_772040
    Lifted::new(0x7000e5e060, tpl_1426::<0x7000e5e5c0, 0x7000e5e06f> as *const (), 0), // +0x772060 sub_772060
    Lifted::new(0x7000e5e080, tpl_1426::<0x7000e5e5e0, 0x7000e5e08f> as *const (), 0), // +0x772080 sub_772080
    Lifted::new(0x7000e5e0a0, tpl_1426::<0x7000e5e600, 0x7000e5e0af> as *const (), 0), // +0x7720a0 sub_7720a0
    Lifted::new(0x7000e5e0c0, tpl_1426::<0x7000e5e620, 0x7000e5e0cf> as *const (), 0), // +0x7720c0 sub_7720c0
    Lifted::new(0x7000e5e0e0, tpl_1426::<0x7000e5e640, 0x7000e5e0ef> as *const (), 0), // +0x7720e0 sub_7720e0
    Lifted::new(0x7000e5e100, tpl_1426::<0x7000e5e660, 0x7000e5e10f> as *const (), 0), // +0x772100 sub_772100
    Lifted::new(0x7000e5e120, tpl_1426::<0x7000e5e680, 0x7000e5e12f> as *const (), 0), // +0x772120 sub_772120
    Lifted::new(0x7000e5e140, tpl_1426::<0x7000e5e6a0, 0x7000e5e14f> as *const (), 0), // +0x772140 sub_772140
    Lifted::new(0x7000e5e160, tpl_1426::<0x7000e5e6c0, 0x7000e5e16f> as *const (), 0), // +0x772160 sub_772160
    Lifted::new(0x7000e5e180, tpl_1426::<0x7000e5e6e0, 0x7000e5e18f> as *const (), 0), // +0x772180 sub_772180
    Lifted::new(0x7000e5e1a0, tpl_1426::<0x7000e5e700, 0x7000e5e1af> as *const (), 0), // +0x7721a0 sub_7721a0
    Lifted::new(0x7000e5e1c0, tpl_1426::<0x7000e5e720, 0x7000e5e1cf> as *const (), 0), // +0x7721c0 sub_7721c0
    Lifted::new(0x7000e5e1e0, tpl_1426::<0x7000e5e740, 0x7000e5e1ef> as *const (), 0), // +0x7721e0 sub_7721e0
    Lifted::new(0x7000e5e200, tpl_1426::<0x7000e5e760, 0x7000e5e20f> as *const (), 0), // +0x772200 sub_772200
    Lifted::new(0x7000e5e220, tpl_1426::<0x7000e5e780, 0x7000e5e22f> as *const (), 0), // +0x772220 sub_772220
    Lifted::new(0x7000e5e240, tpl_1560::<0x7000e52910> as *const (), 0), // +0x772240 sub_772240
    Lifted::new(0x7000e5e270, tpl_1426::<0x7000e5e820, 0x7000e5e27f> as *const (), 0), // +0x772270 sub_772270
    Lifted::new(0x7000e5e290, tpl_1560::<0x7000e52ae0> as *const (), 0), // +0x772290 sub_772290
    Lifted::new(0x7000e5e2c0, tpl_1560::<0x7000e52ce0> as *const (), 0), // +0x7722c0 sub_7722c0
    Lifted::new(0x7000e5e2f0, tpl_1426::<0x7000e5e840, 0x7000e5e2ff> as *const (), 0), // +0x7722f0 sub_7722f0
    Lifted::new(0x7000e5e310, tpl_1560::<0x7000e52f50> as *const (), 0), // +0x772310 sub_772310
    Lifted::new(0x7000e5e340, tpl_1560::<0x7000e531c0> as *const (), 0), // +0x772340 sub_772340
    Lifted::new(0x7000e5e370, tpl_1426::<0x7000e5e860, 0x7000e5e37f> as *const (), 0), // +0x772370 sub_772370
    Lifted::new(0x7000e5e390, tpl_1560::<0x7000e53370> as *const (), 0), // +0x772390 sub_772390
    Lifted::new(0x7000e5e3c0, tpl_1560::<0x7000e53570> as *const (), 0), // +0x7723c0 sub_7723c0
    Lifted::new(0x7000e5e3f0, tpl_1426::<0x7000e5e880, 0x7000e5e3ff> as *const (), 0), // +0x7723f0 sub_7723f0
    Lifted::new(0x7000e5e410, tpl_1560::<0x7000e537c0> as *const (), 0), // +0x772410 sub_772410
    Lifted::new(0x7000e5e440, tpl_1426::<0x7000e5e7a0, 0x7000e5e44f> as *const (), 0), // +0x772440 sub_772440
    Lifted::new(0x7000e5e460, tpl_1426::<0x7000e5e7c0, 0x7000e5e46f> as *const (), 0), // +0x772460 sub_772460
    Lifted::new(0x7000e5e480, tpl_1426::<0x7000e5e7e0, 0x7000e5e48f> as *const (), 0), // +0x772480 sub_772480
    Lifted::new(0x7000e5e4a0, tpl_1426::<0x7000e5e800, 0x7000e5e4af> as *const (), 0), // +0x7724a0 sub_7724a0
    Lifted::new(0x7000e5e4c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e4d1> as *const (), 0), // +0x7724c0 sub_7724c0
    Lifted::new(0x7000e5e4e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e4f1> as *const (), 0), // +0x7724e0 sub_7724e0
    Lifted::new(0x7000e5e500, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e511> as *const (), 0), // +0x772500 sub_772500
    Lifted::new(0x7000e5e520, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e531> as *const (), 0), // +0x772520 sub_772520
    Lifted::new(0x7000e5e540, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e551> as *const (), 0), // +0x772540 sub_772540
    Lifted::new(0x7000e5e560, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e571> as *const (), 0), // +0x772560 sub_772560
    Lifted::new(0x7000e5e580, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e591> as *const (), 0), // +0x772580 sub_772580
    Lifted::new(0x7000e5e5a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e5b1> as *const (), 0), // +0x7725a0 sub_7725a0
    Lifted::new(0x7000e5e5c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e5d1> as *const (), 0), // +0x7725c0 sub_7725c0
    Lifted::new(0x7000e5e5e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e5f1> as *const (), 0), // +0x7725e0 sub_7725e0
    Lifted::new(0x7000e5e600, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e611> as *const (), 0), // +0x772600 sub_772600
    Lifted::new(0x7000e5e620, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e631> as *const (), 0), // +0x772620 sub_772620
    Lifted::new(0x7000e5e640, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e651> as *const (), 0), // +0x772640 sub_772640
    Lifted::new(0x7000e5e660, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e671> as *const (), 0), // +0x772660 sub_772660
    Lifted::new(0x7000e5e680, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e691> as *const (), 0), // +0x772680 sub_772680
    Lifted::new(0x7000e5e6a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e6b1> as *const (), 0), // +0x7726a0 sub_7726a0
    Lifted::new(0x7000e5e6c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e6d1> as *const (), 0), // +0x7726c0 sub_7726c0
    Lifted::new(0x7000e5e6e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e6f1> as *const (), 0), // +0x7726e0 sub_7726e0
    Lifted::new(0x7000e5e700, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e711> as *const (), 0), // +0x772700 sub_772700
    Lifted::new(0x7000e5e720, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e731> as *const (), 0), // +0x772720 sub_772720
    Lifted::new(0x7000e5e740, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e751> as *const (), 0), // +0x772740 sub_772740
    Lifted::new(0x7000e5e760, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e771> as *const (), 0), // +0x772760 sub_772760
    Lifted::new(0x7000e5e780, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e791> as *const (), 0), // +0x772780 sub_772780
    Lifted::new(0x7000e5e7a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e7b1> as *const (), 0), // +0x7727a0 sub_7727a0
    Lifted::new(0x7000e5e7c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e7d1> as *const (), 0), // +0x7727c0 sub_7727c0
    Lifted::new(0x7000e5e7e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e7f1> as *const (), 0), // +0x7727e0 sub_7727e0
    Lifted::new(0x7000e5e800, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e5e811> as *const (), 0), // +0x772800 sub_772800
    Lifted::new(0x7000e5e820, tpl_1561::<0x7000e53a10, 0x7000e5e831> as *const (), 0), // +0x772820 sub_772820
    Lifted::new(0x7000e5e840, tpl_1561::<0x7000e53a50, 0x7000e5e851> as *const (), 0), // +0x772840 sub_772840
    Lifted::new(0x7000e5e860, tpl_1561::<0x7000e53a30, 0x7000e5e871> as *const (), 0), // +0x772860 sub_772860
    Lifted::new(0x7000e5e880, tpl_1561::<0x7000e53a70, 0x7000e5e891> as *const (), 0), // +0x772880 sub_772880
    Lifted::new(0x7000e5e8a0, fn_7000e5e8a0 as *const (), 0), // +0x7728a0 sub_7728a0
    Lifted::new(0x7000e5ea20, fn_7000e5ea20 as *const (), 0), // +0x772a20 sub_772a20
    Lifted::new(0x7000e5ef90, fn_7000e5ef90 as *const (), 1), // +0x772f90 sub_772f90
    Lifted::new(0x7000e5efa0, tpl_1445::<0x7000e5f6d0> as *const (), 0), // +0x772fa0 sub_772fa0
    Lifted::new(0x7000e5efd0, tpl_1445::<0x7000e5fa50> as *const (), 0), // +0x772fd0 sub_772fd0
    Lifted::new(0x7000e5f000, tpl_1445::<0x7000e5fdd0> as *const (), 0), // +0x773000 sub_773000
    Lifted::new(0x7000e5f030, tpl_1445::<0x7000e60150> as *const (), 0), // +0x773030 sub_773030
    Lifted::new(0x7000e5f060, tpl_1445::<0x7000e604d0> as *const (), 0), // +0x773060 sub_773060
    Lifted::new(0x7000e5f090, tpl_1445::<0x7000e60850> as *const (), 0), // +0x773090 sub_773090
    Lifted::new(0x7000e5f0c0, tpl_1445::<0x7000e60bd0> as *const (), 0), // +0x7730c0 sub_7730c0
    Lifted::new(0x7000e5f0f0, tpl_1445::<0x7000e60f50> as *const (), 0), // +0x7730f0 sub_7730f0
    Lifted::new(0x7000e5f120, tpl_1445::<0x7000e612d0> as *const (), 0), // +0x773120 sub_773120
    Lifted::new(0x7000e5f150, tpl_1445::<0x7000e61650> as *const (), 0), // +0x773150 sub_773150
    Lifted::new(0x7000e5f180, tpl_1445::<0x7000e619d0> as *const (), 0), // +0x773180 sub_773180
    Lifted::new(0x7000e5f1b0, tpl_1445::<0x7000e61d50> as *const (), 0), // +0x7731b0 sub_7731b0
    Lifted::new(0x7000e5f1e0, tpl_1445::<0x7000e620d0> as *const (), 0), // +0x7731e0 sub_7731e0
    Lifted::new(0x7000e5f210, tpl_1445::<0x7000e62450> as *const (), 0), // +0x773210 sub_773210
    Lifted::new(0x7000e5f240, tpl_1445::<0x7000e627d0> as *const (), 0), // +0x773240 sub_773240
    Lifted::new(0x7000e5f270, tpl_1445::<0x7000e62b50> as *const (), 0), // +0x773270 sub_773270
    Lifted::new(0x7000e5f2a0, tpl_1445::<0x7000e62ed0> as *const (), 0), // +0x7732a0 sub_7732a0
    Lifted::new(0x7000e5f2d0, tpl_1445::<0x7000e63250> as *const (), 0), // +0x7732d0 sub_7732d0
    Lifted::new(0x7000e5f300, tpl_1445::<0x7000e635d0> as *const (), 0), // +0x773300 sub_773300
    Lifted::new(0x7000e5f330, tpl_1445::<0x7000e63950> as *const (), 0), // +0x773330 sub_773330
    Lifted::new(0x7000e5f360, tpl_1445::<0x7000e63cd0> as *const (), 0), // +0x773360 sub_773360
    Lifted::new(0x7000e5f390, tpl_1445::<0x7000e64050> as *const (), 0), // +0x773390 sub_773390
    Lifted::new(0x7000e5f3c0, tpl_1445::<0x7000e643d0> as *const (), 0), // +0x7733c0 sub_7733c0
    Lifted::new(0x7000e5f3f0, tpl_1445::<0x7000e64750> as *const (), 0), // +0x7733f0 sub_7733f0
    Lifted::new(0x7000e5f420, tpl_1445::<0x7000e649a0> as *const (), 0), // +0x773420 sub_773420
    Lifted::new(0x7000e5f450, tpl_1445::<0x7000e64bf0> as *const (), 0), // +0x773450 sub_773450
    Lifted::new(0x7000e5f480, tpl_1445::<0x7000e64e40> as *const (), 0), // +0x773480 sub_773480
    Lifted::new(0x7000e5f4b0, tpl_1445::<0x7000e65090> as *const (), 0), // +0x7734b0 sub_7734b0
    Lifted::new(0x7000e5f4e0, tpl_1445::<0x7000e652e0> as *const (), 0), // +0x7734e0 sub_7734e0
    Lifted::new(0x7000e5f510, tpl_1445::<0x7000e65530> as *const (), 0), // +0x773510 sub_773510
    Lifted::new(0x7000e5f540, tpl_1445::<0x7000e65780> as *const (), 0), // +0x773540 sub_773540
    Lifted::new(0x7000e5f570, tpl_1445::<0x7000e659d0> as *const (), 0), // +0x773570 sub_773570
    Lifted::new(0x7000e5f5a0, tpl_1445::<0x7000e65c20> as *const (), 0), // +0x7735a0 sub_7735a0
    Lifted::new(0x7000e5f5d0, tpl_1445::<0x7000e65e70> as *const (), 0), // +0x7735d0 sub_7735d0
    Lifted::new(0x7000e5f600, tpl_1445::<0x7000e660c0> as *const (), 0), // +0x773600 sub_773600
    Lifted::new(0x7000e5f630, tpl_1545::<0x7000e667b0, 0x7000e5f645> as *const (), 0), // +0x773630 sub_773630
    Lifted::new(0x7000e5f650, tpl_1445::<0x7000e66310> as *const (), 0), // +0x773650 sub_773650
    Lifted::new(0x7000e5f680, tpl_1545::<0x7000e667d0, 0x7000e5f695> as *const (), 0), // +0x773680 sub_773680
    Lifted::new(0x7000e5f6a0, tpl_1445::<0x7000e66560> as *const (), 0), // +0x7736a0 sub_7736a0
    Lifted::new(0x7000e5f6d0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00800, 0x7000e5f994> as *const (), 0), // +0x7736d0 sub_7736d0
    Lifted::new(0x7000e5fa50, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00820, 0x7000e5fd14> as *const (), 0), // +0x773a50 sub_773a50
    Lifted::new(0x7000e5fdd0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00840, 0x7000e60094> as *const (), 0), // +0x773dd0 sub_773dd0
    Lifted::new(0x7000e60150, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00860, 0x7000e60414> as *const (), 0), // +0x774150 sub_774150
    Lifted::new(0x7000e604d0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00880, 0x7000e60794> as *const (), 0), // +0x7744d0 sub_7744d0
    Lifted::new(0x7000e60850, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e008a0, 0x7000e60b14> as *const (), 0), // +0x774850 sub_774850
    Lifted::new(0x7000e60bd0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e008c0, 0x7000e60e94> as *const (), 0), // +0x774bd0 sub_774bd0
    Lifted::new(0x7000e60f50, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e008e0, 0x7000e61214> as *const (), 0), // +0x774f50 sub_774f50
    Lifted::new(0x7000e612d0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00900, 0x7000e61594> as *const (), 0), // +0x7752d0 sub_7752d0
    Lifted::new(0x7000e61650, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00920, 0x7000e61914> as *const (), 0), // +0x775650 sub_775650
    Lifted::new(0x7000e619d0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00940, 0x7000e61c94> as *const (), 0), // +0x7759d0 sub_7759d0
    Lifted::new(0x7000e61d50, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00960, 0x7000e62014> as *const (), 0), // +0x775d50 sub_775d50
    Lifted::new(0x7000e620d0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00980, 0x7000e62394> as *const (), 0), // +0x7760d0 sub_7760d0
    Lifted::new(0x7000e62450, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e009a0, 0x7000e62714> as *const (), 0), // +0x776450 sub_776450
    Lifted::new(0x7000e627d0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e009c0, 0x7000e62a94> as *const (), 0), // +0x7767d0 sub_7767d0
    Lifted::new(0x7000e62b50, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e009e0, 0x7000e62e14> as *const (), 0), // +0x776b50 sub_776b50
    Lifted::new(0x7000e62ed0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a00, 0x7000e63194> as *const (), 0), // +0x776ed0 sub_776ed0
    Lifted::new(0x7000e63250, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a20, 0x7000e63514> as *const (), 0), // +0x777250 sub_777250
    Lifted::new(0x7000e635d0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a40, 0x7000e63894> as *const (), 0), // +0x7775d0 sub_7775d0
    Lifted::new(0x7000e63950, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a60, 0x7000e63c14> as *const (), 0), // +0x777950 sub_777950
    Lifted::new(0x7000e63cd0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00a80, 0x7000e63f94> as *const (), 0), // +0x777cd0 sub_777cd0
    Lifted::new(0x7000e64050, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e00aa0, 0x7000e64314> as *const (), 0), // +0x778050 sub_778050
    Lifted::new(0x7000e643d0, tpl_1562::<0x70006f2a70, 0x70006f9a60, 0x700074a850, 0x700079cfd0, 0x7000720d30, 0x70007718e0, 0x700076df00, 0x7000e40330, 0x7000d8bd20, 0x7000d90750, 0x7000e5ef90, 0x7000e01940, 0x7000e64694> as *const (), 0), // +0x7783d0 sub_7783d0
    Lifted::new(0x7000e64750, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de71a0, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x778750 sub_778750
    Lifted::new(0x7000e649a0, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de7330, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x7789a0 sub_7789a0
    Lifted::new(0x7000e64bf0, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de74c0, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x778bf0 sub_778bf0
    Lifted::new(0x7000e64e40, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de7650, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x778e40 sub_778e40
    Lifted::new(0x7000e65090, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de77e0, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x779090 sub_779090
    Lifted::new(0x7000e652e0, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de7970, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x7792e0 sub_7792e0
    Lifted::new(0x7000e65530, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de7b00, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x779530 sub_779530
    Lifted::new(0x7000e65780, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de7c90, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x779780 sub_779780
    Lifted::new(0x7000e659d0, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de7e20, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x7799d0 sub_7799d0
    Lifted::new(0x7000e65c20, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de7fb0, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x779c20 sub_779c20
    Lifted::new(0x7000e65e70, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de8140, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x779e70 sub_779e70
    Lifted::new(0x7000e660c0, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de82d0, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x77a0c0 sub_77a0c0
    Lifted::new(0x7000e66310, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de86b0, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x77a310 sub_77a310
    Lifted::new(0x7000e66560, tpl_1563::<0x70006f2a70, 0x70006f9a60, 0x7000de8900, 0x7000e5de80, 0x7000720d30> as *const (), 0), // +0x77a560 sub_77a560
    Lifted::new(0x7000e667b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e667c1> as *const (), 0), // +0x77a7b0 sub_77a7b0
    Lifted::new(0x7000e667d0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e667e1> as *const (), 0), // +0x77a7d0 sub_77a7d0
    Lifted::new(0x7000e667f0, tpl_1564::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e66861, 0x700072e670, 0x70009e4b20> as *const (), 2), // +0x77a7f0 sub_77a7f0
    Lifted::new(0x7000e66c40, tpl_1564::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e66cb1, 0x700072e670, 0x70009e4b20> as *const (), 2), // +0x77ac40 sub_77ac40
    Lifted::new(0x7000e67090, fn_7000e67090 as *const (), 2), // +0x77b090 sub_77b090
    Lifted::new(0x7000e67550, fn_7000e67550 as *const (), 2), // +0x77b550 sub_77b550
    Lifted::new(0x7000e67a10, tpl_1565::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e67a81, 0x700072e670, 0x70009e4b20> as *const (), 2), // +0x77ba10 sub_77ba10
    Lifted::new(0x7000e67e50, tpl_1565::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e67ec1, 0x700072e670, 0x70009e4b20> as *const (), 2), // +0x77be50 sub_77be50
    Lifted::new(0x7000e68290, fn_7000e68290 as *const (), 2), // +0x77c290 sub_77c290
    Lifted::new(0x7000e687c0, fn_7000e687c0 as *const (), 2), // +0x77c7c0 sub_77c7c0
    Lifted::new(0x7000e68f90, tpl_1566::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e69001, 0x700072e670, 0x7000e201a0, 0x7001855d18, 0x700177c260> as *const (), 2), // +0x77cf90 sub_77cf90
    Lifted::new(0x7000e69420, tpl_1564::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e69491, 0x700072e670, 0x70009e4b20> as *const (), 2), // +0x77d420 sub_77d420
    Lifted::new(0x7000e69870, tpl_1564::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e698e1, 0x700072e670, 0x70009e4b20> as *const (), 2), // +0x77d870 sub_77d870
    Lifted::new(0x7000e69cc0, fn_7000e69cc0 as *const (), 2), // +0x77dcc0 sub_77dcc0
    Lifted::new(0x7000e6a180, fn_7000e6a180 as *const (), 2), // +0x77e180 sub_77e180
    Lifted::new(0x7000e6a640, tpl_1565::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e6a6b1, 0x700072e670, 0x70009e4b20> as *const (), 2), // +0x77e640 sub_77e640
    Lifted::new(0x7000e6aa80, tpl_1565::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e6aaf1, 0x700072e670, 0x70009e4b20> as *const (), 2), // +0x77ea80 sub_77ea80
    Lifted::new(0x7000e6aec0, fn_7000e6aec0 as *const (), 2), // +0x77eec0 sub_77eec0
    Lifted::new(0x7000e6b3f0, fn_7000e6b3f0 as *const (), 2), // +0x77f3f0 sub_77f3f0
    Lifted::new(0x7000e6bbc0, tpl_1566::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e6bc31, 0x700072e670, 0x7000e20220, 0x7001855d18, 0x700177c260> as *const (), 2), // +0x77fbc0 sub_77fbc0
    Lifted::new(0x7000e6c050, tpl_1567::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e6c0c5, 0x700072e670, 0x7000db6760, 0x7000b7ead0, 0x7000e5ef90, 0x7000e208e0, 0x7000e6c3ca, 0x7000e6c3eb, 0x7000d90700, 0x7000e20920, 0x7000e6c3fc, 0x7000e6c40d> as *const (), 0), // +0x780050 sub_780050
    Lifted::new(0x7000e6c500, tpl_1567::<0x700072f5d0, 0x7001855b18, 0x70017a0290, 0x7001d54ef8, 0x7000e6c575, 0x700072e670, 0x7000757d30, 0x7000b7ead0, 0x7000e5ef90, 0x7000e20940, 0x7000e6c87a, 0x7000e6c89b, 0x7000d90700, 0x7000e20980, 0x7000e6c8ac, 0x7000e6c8bd> as *const (), 0), // +0x780500 sub_780500
    Lifted::new(0x7000e6c9b0, tpl_1527::<0x7000e6dac0, 0x7000e6c9cd> as *const (), 0), // +0x7809b0 sub_7809b0
    Lifted::new(0x7000e6c9e0, tpl_1527::<0x7000e6dae0, 0x7000e6c9fd> as *const (), 0), // +0x7809e0 sub_7809e0
    Lifted::new(0x7000e6ca10, tpl_1527::<0x7000e6db00, 0x7000e6ca2d> as *const (), 0), // +0x780a10 sub_780a10
    Lifted::new(0x7000e6ca40, tpl_1527::<0x7000e6db20, 0x7000e6ca5d> as *const (), 0), // +0x780a40 sub_780a40
    Lifted::new(0x7000e6ca70, tpl_1527::<0x7000e6db40, 0x7000e6ca8d> as *const (), 0), // +0x780a70 sub_780a70
    Lifted::new(0x7000e6caa0, tpl_1527::<0x7000e6db60, 0x7000e6cabd> as *const (), 0), // +0x780aa0 sub_780aa0
    Lifted::new(0x7000e6cad0, tpl_1527::<0x7000e6db80, 0x7000e6caed> as *const (), 0), // +0x780ad0 sub_780ad0
    Lifted::new(0x7000e6cb00, tpl_1527::<0x7000e6dba0, 0x7000e6cb1d> as *const (), 0), // +0x780b00 sub_780b00
    Lifted::new(0x7000e6cb30, tpl_1527::<0x7000e6dbc0, 0x7000e6cb4d> as *const (), 0), // +0x780b30 sub_780b30
    Lifted::new(0x7000e6cb60, tpl_1527::<0x7000e6dbe0, 0x7000e6cb7d> as *const (), 0), // +0x780b60 sub_780b60
    Lifted::new(0x7000e6cb90, tpl_1527::<0x7000e6dc00, 0x7000e6cbad> as *const (), 0), // +0x780b90 sub_780b90
    Lifted::new(0x7000e6cbc0, tpl_1527::<0x7000e6dc20, 0x7000e6cbdd> as *const (), 0), // +0x780bc0 sub_780bc0
    Lifted::new(0x7000e6cbf0, tpl_1527::<0x7000e6dc40, 0x7000e6cc0d> as *const (), 0), // +0x780bf0 sub_780bf0
    Lifted::new(0x7000e6cc20, tpl_1527::<0x7000e6dc60, 0x7000e6cc3d> as *const (), 0), // +0x780c20 sub_780c20
    Lifted::new(0x7000e6cc50, tpl_1527::<0x7000e6dc80, 0x7000e6cc6d> as *const (), 0), // +0x780c50 sub_780c50
    Lifted::new(0x7000e6cc80, tpl_1527::<0x7000e6dca0, 0x7000e6cc9d> as *const (), 0), // +0x780c80 sub_780c80
    Lifted::new(0x7000e6ccb0, tpl_1527::<0x7000e6dcc0, 0x7000e6cccd> as *const (), 0), // +0x780cb0 sub_780cb0
    Lifted::new(0x7000e6cce0, tpl_1527::<0x7000e6dce0, 0x7000e6ccfd> as *const (), 0), // +0x780ce0 sub_780ce0
    Lifted::new(0x7000e6cd10, tpl_1527::<0x7000e6dd00, 0x7000e6cd2d> as *const (), 0), // +0x780d10 sub_780d10
    Lifted::new(0x7000e6cd40, tpl_1527::<0x7000e6dd20, 0x7000e6cd5d> as *const (), 0), // +0x780d40 sub_780d40
    Lifted::new(0x7000e6cd70, tpl_1527::<0x7000e6dd40, 0x7000e6cd8d> as *const (), 0), // +0x780d70 sub_780d70
    Lifted::new(0x7000e6cda0, tpl_1527::<0x7000e6dd60, 0x7000e6cdbd> as *const (), 0), // +0x780da0 sub_780da0
    Lifted::new(0x7000e6cdd0, tpl_1527::<0x7000e6dd80, 0x7000e6cded> as *const (), 0), // +0x780dd0 sub_780dd0
    Lifted::new(0x7000e6ce00, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcbdd0> as *const (), 0), // +0x780e00 sub_780e00
    Lifted::new(0x7000e6cf00, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcc130> as *const (), 0), // +0x780f00 sub_780f00
    Lifted::new(0x7000e6d000, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcc450> as *const (), 0), // +0x781000 sub_781000
    Lifted::new(0x7000e6d100, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcc7a0> as *const (), 0), // +0x781100 sub_781100
    Lifted::new(0x7000e6d200, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dccaf0> as *const (), 0), // +0x781200 sub_781200
    Lifted::new(0x7000e6d300, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcce40> as *const (), 0), // +0x781300 sub_781300
    Lifted::new(0x7000e6d400, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcd190> as *const (), 0), // +0x781400 sub_781400
    Lifted::new(0x7000e6d500, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcd530> as *const (), 0), // +0x781500 sub_781500
    Lifted::new(0x7000e6d600, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcd850> as *const (), 0), // +0x781600 sub_781600
    Lifted::new(0x7000e6d700, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcdba0> as *const (), 0), // +0x781700 sub_781700
    Lifted::new(0x7000e6d800, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dcdf30> as *const (), 0), // +0x781800 sub_781800
    Lifted::new(0x7000e6d900, tpl_1568::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0x7000dce280> as *const (), 0), // +0x781900 sub_781900
    Lifted::new(0x7000e6da00, tpl_1527::<0x7000e6dda0, 0x7000e6da1d> as *const (), 0), // +0x781a00 sub_781a00
    Lifted::new(0x7000e6da30, tpl_1527::<0x7000e6ddc0, 0x7000e6da4d> as *const (), 0), // +0x781a30 sub_781a30
    Lifted::new(0x7000e6da60, tpl_1527::<0x7000e6dde0, 0x7000e6da7d> as *const (), 0), // +0x781a60 sub_781a60
    Lifted::new(0x7000e6da90, tpl_1527::<0x7000e6de00, 0x7000e6daad> as *const (), 0), // +0x781a90 sub_781a90
    Lifted::new(0x7000e6dac0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dad1> as *const (), 0), // +0x781ac0 sub_781ac0
    Lifted::new(0x7000e6dae0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6daf1> as *const (), 0), // +0x781ae0 sub_781ae0
    Lifted::new(0x7000e6db00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6db11> as *const (), 0), // +0x781b00 sub_781b00
    Lifted::new(0x7000e6db20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6db31> as *const (), 0), // +0x781b20 sub_781b20
    Lifted::new(0x7000e6db40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6db51> as *const (), 0), // +0x781b40 sub_781b40
    Lifted::new(0x7000e6db60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6db71> as *const (), 0), // +0x781b60 sub_781b60
    Lifted::new(0x7000e6db80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6db91> as *const (), 0), // +0x781b80 sub_781b80
    Lifted::new(0x7000e6dba0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dbb1> as *const (), 0), // +0x781ba0 sub_781ba0
    Lifted::new(0x7000e6dbc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dbd1> as *const (), 0), // +0x781bc0 sub_781bc0
    Lifted::new(0x7000e6dbe0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dbf1> as *const (), 0), // +0x781be0 sub_781be0
    Lifted::new(0x7000e6dc00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dc11> as *const (), 0), // +0x781c00 sub_781c00
    Lifted::new(0x7000e6dc20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dc31> as *const (), 0), // +0x781c20 sub_781c20
    Lifted::new(0x7000e6dc40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dc51> as *const (), 0), // +0x781c40 sub_781c40
    Lifted::new(0x7000e6dc60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dc71> as *const (), 0), // +0x781c60 sub_781c60
    Lifted::new(0x7000e6dc80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dc91> as *const (), 0), // +0x781c80 sub_781c80
    Lifted::new(0x7000e6dca0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dcb1> as *const (), 0), // +0x781ca0 sub_781ca0
    Lifted::new(0x7000e6dcc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dcd1> as *const (), 0), // +0x781cc0 sub_781cc0
    Lifted::new(0x7000e6dce0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dcf1> as *const (), 0), // +0x781ce0 sub_781ce0
    Lifted::new(0x7000e6dd00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dd11> as *const (), 0), // +0x781d00 sub_781d00
    Lifted::new(0x7000e6dd20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dd31> as *const (), 0), // +0x781d20 sub_781d20
    Lifted::new(0x7000e6dd40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dd51> as *const (), 0), // +0x781d40 sub_781d40
    Lifted::new(0x7000e6dd60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dd71> as *const (), 0), // +0x781d60 sub_781d60
    Lifted::new(0x7000e6dd80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6dd91> as *const (), 0), // +0x781d80 sub_781d80
    Lifted::new(0x7000e6dda0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6ddb1> as *const (), 0), // +0x781da0 sub_781da0
    Lifted::new(0x7000e6ddc0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6ddd1> as *const (), 0), // +0x781dc0 sub_781dc0
    Lifted::new(0x7000e6dde0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6ddf1> as *const (), 0), // +0x781de0 sub_781de0
    Lifted::new(0x7000e6de00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000e6de11> as *const (), 0), // +0x781e00 sub_781e00
    Lifted::new(0x7000e6de20, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x781e20 sub_781e20
    Lifted::new(0x7000e6de90, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x781e90 sub_781e90
    Lifted::new(0x7000e6df00, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x781f00 sub_781f00
    Lifted::new(0x7000e6df70, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x781f70 sub_781f70
    Lifted::new(0x7000e6dfe0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x781fe0 sub_781fe0
    Lifted::new(0x7000e6e050, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782050 sub_782050
    Lifted::new(0x7000e6e0c0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7820c0 sub_7820c0
    Lifted::new(0x7000e6e130, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782130 sub_782130
    Lifted::new(0x7000e6e1a0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7821a0 sub_7821a0
    Lifted::new(0x7000e6e210, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782210 sub_782210
    Lifted::new(0x7000e6e280, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782280 sub_782280
    Lifted::new(0x7000e6e2f0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7822f0 sub_7822f0
    Lifted::new(0x7000e6e360, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782360 sub_782360
    Lifted::new(0x7000e6e3d0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7823d0 sub_7823d0
    Lifted::new(0x7000e6e440, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782440 sub_782440
    Lifted::new(0x7000e6e4b0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7824b0 sub_7824b0
    Lifted::new(0x7000e6e520, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782520 sub_782520
    Lifted::new(0x7000e6e590, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782590 sub_782590
    Lifted::new(0x7000e6e600, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782600 sub_782600
    Lifted::new(0x7000e6e670, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782670 sub_782670
    Lifted::new(0x7000e6e6e0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7826e0 sub_7826e0
    Lifted::new(0x7000e6e750, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782750 sub_782750
    Lifted::new(0x7000e6e7c0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7827c0 sub_7827c0
    Lifted::new(0x7000e6e830, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782830 sub_782830
    Lifted::new(0x7000e6e8a0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7828a0 sub_7828a0
    Lifted::new(0x7000e6e910, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782910 sub_782910
    Lifted::new(0x7000e6e980, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782980 sub_782980
    Lifted::new(0x7000e6e9f0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x7829f0 sub_7829f0
    Lifted::new(0x7000e6ea60, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782a60 sub_782a60
    Lifted::new(0x7000e6ead0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782ad0 sub_782ad0
    Lifted::new(0x7000e6eb40, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782b40 sub_782b40
    Lifted::new(0x7000e6ebb0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782bb0 sub_782bb0
    Lifted::new(0x7000e6ec20, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782c20 sub_782c20
    Lifted::new(0x7000e6ec90, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782c90 sub_782c90
    Lifted::new(0x7000e6ed00, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782d00 sub_782d00
    Lifted::new(0x7000e6ed70, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782d70 sub_782d70
    Lifted::new(0x7000e6ede0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782de0 sub_782de0
    Lifted::new(0x7000e6ee50, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782e50 sub_782e50
    Lifted::new(0x7000e6eec0, tpl_1569::<0x7, 0xffffffffffffffff> as *const (), 1), // +0x782ec0 sub_782ec0
    Lifted::new(0x7000e6ef30, fn_7000e6ef30 as *const (), 0), // +0x782f30 sub_782f30
    Lifted::new(0x7000e6f070, fn_7000e6f070 as *const (), 0), // +0x783070 sub_783070
    Lifted::new(0x7000e6f1c0, fn_7000e6f1c0 as *const (), 0), // +0x7831c0 sub_7831c0
    Lifted::new(0x7000e6f310, tpl_1570::<0x8, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x70006f4020, 0x28, 0x70006f86f0> as *const (), 0), // +0x783310 sub_783310
    Lifted::new(0x7000e6f430, tpl_1570::<0x8, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x70006f4020, 0x28, 0x70006f86f0> as *const (), 0), // +0x783430 sub_783430
    Lifted::new(0x7000e6f550, tpl_1570::<0x8, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x70006f4020, 0x28, 0x70006f86f0> as *const (), 0), // +0x783550 sub_783550
    Lifted::new(0x7000e6f670, tpl_1570::<0x8, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x70006f4020, 0x28, 0x70006f86f0> as *const (), 0), // +0x783670 sub_783670
    Lifted::new(0x7000e6f790, tpl_389::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000e6fa0f, 0x7000d097d0, 0x7000e6fa15, 0x70007f0910, 0x7000e6fa1b> as *const (), 2), // +0x783790 sub_783790
    Lifted::new(0x7000e6fac0, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000e6fc94, 0x7000e75c00, 0x7000e6fc9a, 0x70007f0910, 0x7000e6fca0> as *const (), 0), // +0x783ac0 sub_783ac0
    Lifted::new(0x7000e6fd20, fn_7000e6fd20 as *const (), 0), // +0x783d20 sub_783d20
    Lifted::new(0x7000e6fe90, fn_7000e6fe90 as *const (), 1), // +0x783e90 sub_783e90
    Lifted::new(0x7000e6ff60, fn_7000e6ff60 as *const (), 3), // +0x783f60 sub_783f60
    Lifted::new(0x7000e70020, fn_7000e70020 as *const (), 1), // +0x784020 sub_784020
    Lifted::new(0x7000e700f0, fn_7000e700f0 as *const (), 0), // +0x7840f0 sub_7840f0
    Lifted::new(0x7000e70370, fn_7000e70370 as *const (), 0), // +0x784370 sub_784370
    Lifted::new(0x7000e705a0, fn_7000e705a0 as *const (), 0), // +0x7845a0 sub_7845a0
    Lifted::new(0x7000e706d0, fn_7000e706d0 as *const (), 0), // +0x7846d0 sub_7846d0
    Lifted::new(0x7000e70720, fn_7000e70720 as *const (), 0), // +0x784720 sub_784720
    Lifted::new(0x7000e70870, fn_7000e70870 as *const (), 0), // +0x784870 sub_784870
    Lifted::new(0x7000e709b0, fn_7000e709b0 as *const (), 0), // +0x7849b0 sub_7849b0
    Lifted::new(0x7000e70ac0, fn_7000e70ac0 as *const (), 0), // +0x784ac0 sub_784ac0
    Lifted::new(0x7000e70af0, fn_7000e70af0 as *const (), 0), // +0x784af0 sub_784af0
    Lifted::new(0x7000e70c20, fn_7000e70c20 as *const (), 0), // +0x784c20 sub_784c20
    Lifted::new(0x7000e70f20, fn_7000e70f20 as *const (), 0), // +0x784f20 sub_784f20
    Lifted::new(0x7000e713e0, fn_7000e713e0 as *const (), 2), // +0x7853e0 sub_7853e0
    Lifted::new(0x7000e71480, fn_7000e71480 as *const (), 2), // +0x785480 sub_785480
    Lifted::new(0x7000e71d00, fn_7000e71d00 as *const (), 0), // +0x785d00 sub_785d00
    Lifted::new(0x7000e71db0, fn_7000e71db0 as *const (), 0), // +0x785db0 sub_785db0
    Lifted::new(0x7000e71e70, fn_7000e71e70 as *const (), 0), // +0x785e70 sub_785e70
    Lifted::new(0x7000e71f30, fn_7000e71f30 as *const (), 0), // +0x785f30 sub_785f30
    Lifted::new(0x7000e72000, fn_7000e72000 as *const (), 0), // +0x786000 sub_786000
    Lifted::new(0x7000e72100, fn_7000e72100 as *const (), 2), // +0x786100 sub_786100
    Lifted::new(0x7000e72490, fn_7000e72490 as *const (), 2), // +0x786490 sub_786490
    Lifted::new(0x7000e728b0, fn_7000e728b0 as *const (), 0), // +0x7868b0 sub_7868b0
    Lifted::new(0x7000e72920, fn_7000e72920 as *const (), 0), // +0x786920 sub_786920
    Lifted::new(0x7000e72b70, fn_7000e72b70 as *const (), 0), // +0x786b70 sub_786b70
    Lifted::new(0x7000e72bc0, fn_7000e72bc0 as *const (), 0), // +0x786bc0 sub_786bc0
    Lifted::new(0x7000e72db0, fn_7000e72db0 as *const (), 0), // +0x786db0 sub_786db0
    Lifted::new(0x7000e72e60, fn_7000e72e60 as *const (), 2), // +0x786e60 sub_786e60
    Lifted::new(0x7000e72f70, fn_7000e72f70 as *const (), 2), // +0x786f70 sub_786f70
    Lifted::new(0x7000e73110, fn_7000e73110 as *const (), 0), // +0x787110 sub_787110
    Lifted::new(0x7000e731d0, fn_7000e731d0 as *const (), 0), // +0x7871d0 sub_7871d0
    Lifted::new(0x7000e732b0, fn_7000e732b0 as *const (), 0), // +0x7872b0 sub_7872b0
    Lifted::new(0x7000e73300, fn_7000e73300 as *const (), 0), // +0x787300 sub_787300
    Lifted::new(0x7000e73350, fn_7000e73350 as *const (), 0), // +0x787350 sub_787350
    Lifted::new(0x7000e73420, fn_7000e73420 as *const (), 0), // +0x787420 sub_787420
    Lifted::new(0x7000e735c0, fn_7000e735c0 as *const (), 0), // +0x7875c0 sub_7875c0
    Lifted::new(0x7000e736a0, fn_7000e736a0 as *const (), 0), // +0x7876a0 sub_7876a0
    Lifted::new(0x7000e736f0, fn_7000e736f0 as *const (), 0), // +0x7876f0 sub_7876f0
    Lifted::new(0x7000e73750, fn_7000e73750 as *const (), 0), // +0x787750 sub_787750
    Lifted::new(0x7000e737d0, fn_7000e737d0 as *const (), 2), // +0x7877d0 sub_7877d0
    Lifted::new(0x7000e73990, fn_7000e73990 as *const (), 2), // +0x787990 sub_787990
    Lifted::new(0x7000e73d20, fn_7000e73d20 as *const (), 1), // +0x787d20 sub_787d20
    Lifted::new(0x7000e73d70, fn_7000e73d70 as *const (), 1), // +0x787d70 sub_787d70
    Lifted::new(0x7000e73de0, fn_7000e73de0 as *const (), 1), // +0x787de0 sub_787de0
    Lifted::new(0x7000e73e50, fn_7000e73e50 as *const (), 0), // +0x787e50 sub_787e50
    Lifted::new(0x7000e73f00, fn_7000e73f00 as *const (), 2), // +0x787f00 sub_787f00
    Lifted::new(0x7000e74250, fn_7000e74250 as *const (), 0), // +0x788250 sub_788250
    Lifted::new(0x7000e742a0, tpl_1451 as *const (), 1), // +0x7882a0 sub_7882a0
    Lifted::new(0x7000e742b0, fn_7000e742b0 as *const (), 2), // +0x7882b0 sub_7882b0
    Lifted::new(0x7000e74640, fn_7000e74640 as *const (), 1), // +0x788640 sub_788640
    Lifted::new(0x7000e74660, fn_7000e74660 as *const (), 0), // +0x788660 sub_788660
    Lifted::new(0x7000e746a0, fn_7000e746a0 as *const (), 1), // +0x7886a0 sub_7886a0
    Lifted::new(0x7000e746c0, fn_7000e746c0 as *const (), 1), // +0x7886c0 sub_7886c0
    Lifted::new(0x7000e74760, fn_7000e74760 as *const (), 0), // +0x788760 sub_788760
    Lifted::new(0x7000e74970, fn_7000e74970 as *const (), 2), // +0x788970 sub_788970
    Lifted::new(0x7000e74b20, fn_7000e74b20 as *const (), 0), // +0x788b20 sub_788b20
    Lifted::new(0x7000e74f70, fn_7000e74f70 as *const (), 0), // +0x788f70 sub_788f70
    Lifted::new(0x7000e751b0, fn_7000e751b0 as *const (), 0), // +0x7891b0 sub_7891b0
    Lifted::new(0x7000e753a0, fn_7000e753a0 as *const (), 0), // +0x7893a0 sub_7893a0
    Lifted::new(0x7000e75460, fn_7000e75460 as *const (), 0), // +0x789460 sub_789460
    Lifted::new(0x7000e75520, fn_7000e75520 as *const (), 0), // +0x789520 sub_789520
    Lifted::new(0x7000e75640, fn_7000e75640 as *const (), 1), // +0x789640 sub_789640
    Lifted::new(0x7000e75660, fn_7000e75660 as *const (), 0), // +0x789660 sub_789660
    Lifted::new(0x7000e75750, fn_7000e75750 as *const (), 1), // +0x789750 sub_789750
    Lifted::new(0x7000e75830, fn_7000e75830 as *const (), 1), // +0x789830 sub_789830
    Lifted::new(0x7000e75850, fn_7000e75850 as *const (), 0), // +0x789850 sub_789850
    Lifted::new(0x7000e759d0, fn_7000e759d0 as *const (), 0), // +0x7899d0 sub_7899d0
    Lifted::new(0x7000e75a90, fn_7000e75a90 as *const (), 0), // +0x789a90 sub_789a90
    Lifted::new(0x7000e75c00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000e75c10> as *const (), 0), // +0x789c00 sub_789c00
    Lifted::new(0x7000e75c20, fn_7000e75c20 as *const (), 0), // +0x789c20 sub_789c20
    Lifted::new(0x7000e761b0, fn_7000e761b0 as *const (), 0), // +0x78a1b0 sub_78a1b0
    Lifted::new(0x7000e762d0, tpl_329 as *const (), 1), // +0x78a2d0 sub_78a2d0
    Lifted::new(0x7000e762e0, fn_7000e762e0 as *const (), 0), // +0x78a2e0 sub_78a2e0
    Lifted::new(0x7000e76370, tpl_187 as *const (), 1), // +0x78a370 sub_78a370
    Lifted::new(0x7000e76380, tpl_14 as *const (), 1), // +0x78a380 sub_78a380
    Lifted::new(0x7000e76390, fn_7000e76390 as *const (), 0), // +0x78a390 sub_78a390
    Lifted::new(0x7000e76420, fn_7000e76420 as *const (), 1), // +0x78a420 sub_78a420
    Lifted::new(0x7000e764b0, fn_7000e764b0 as *const (), 1), // +0x78a4b0 sub_78a4b0
    Lifted::new(0x7000e764f0, fn_7000e764f0 as *const (), 1), // +0x78a4f0 sub_78a4f0
    Lifted::new(0x7000e76500, tpl_1249::<0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x78a500 sub_78a500
    Lifted::new(0x7000e76590, fn_7000e76590 as *const (), 1), // +0x78a590 sub_78a590
    Lifted::new(0x7000e765c0, fn_7000e765c0 as *const (), 1), // +0x78a5c0 sub_78a5c0
    Lifted::new(0x7000e765f0, fn_7000e765f0 as *const (), 1), // +0x78a5f0 sub_78a5f0
    Lifted::new(0x7000e76690, fn_7000e76690 as *const (), 0), // +0x78a690 sub_78a690
    Lifted::new(0x7000e76730, tpl_14 as *const (), 1), // +0x78a730 sub_78a730
    Lifted::new(0x7000e76740, fn_7000e76740 as *const (), 2), // +0x78a740 sub_78a740
    Lifted::new(0x7000e76a00, fn_7000e76a00 as *const (), 2), // +0x78aa00 sub_78aa00
    Lifted::new(0x7000e76e90, fn_7000e76e90 as *const (), 0), // +0x78ae90 sub_78ae90
    Lifted::new(0x7000e770a0, fn_7000e770a0 as *const (), 2), // +0x78b0a0 sub_78b0a0
    Lifted::new(0x7000e771b0, fn_7000e771b0 as *const (), 1), // +0x78b1b0 sub_78b1b0
    Lifted::new(0x7000e77270, fn_7000e77270 as *const (), 2), // +0x78b270 sub_78b270
    Lifted::new(0x7000e77670, fn_7000e77670 as *const (), 0), // +0x78b670 sub_78b670
    Lifted::new(0x7000e77880, fn_7000e77880 as *const (), 1), // +0x78b880 sub_78b880
    Lifted::new(0x7000e77940, fn_7000e77940 as *const (), 0), // +0x78b940 sub_78b940
    Lifted::new(0x7000e77b70, fn_7000e77b70 as *const (), 1), // +0x78bb70 sub_78bb70
    Lifted::new(0x7000e77bb0, fn_7000e77bb0 as *const (), 1), // +0x78bbb0 sub_78bbb0
    Lifted::new(0x7000e77bf0, fn_7000e77bf0 as *const (), 0), // +0x78bbf0 sub_78bbf0
    Lifted::new(0x7000e77cc0, fn_7000e77cc0 as *const (), 0), // +0x78bcc0 sub_78bcc0
    Lifted::new(0x7000e77e20, fn_7000e77e20 as *const (), 0), // +0x78be20 sub_78be20
    Lifted::new(0x7000e77eb0, fn_7000e77eb0 as *const (), 0), // +0x78beb0 sub_78beb0
    Lifted::new(0x7000e77fe0, fn_7000e77fe0 as *const (), 0), // +0x78bfe0 sub_78bfe0
    Lifted::new(0x7000e78050, fn_7000e78050 as *const (), 0), // +0x78c050 sub_78c050
    Lifted::new(0x7000e78130, fn_7000e78130 as *const (), 0), // +0x78c130 sub_78c130
    Lifted::new(0x7000e78210, fn_7000e78210 as *const (), 0), // +0x78c210 sub_78c210
    Lifted::new(0x7000e78340, fn_7000e78340 as *const (), 0), // +0x78c340 sub_78c340
    Lifted::new(0x7000e78590, fn_7000e78590 as *const (), 0), // +0x78c590 sub_78c590
    Lifted::new(0x7000e78690, fn_7000e78690 as *const (), 0), // +0x78c690 sub_78c690
    Lifted::new(0x7000e786f0, fn_7000e786f0 as *const (), 0), // +0x78c6f0 sub_78c6f0
    Lifted::new(0x7000e787c0, fn_7000e787c0 as *const (), 1), // +0x78c7c0 sub_78c7c0
    Lifted::new(0x7000e78880, fn_7000e78880 as *const (), 0), // +0x78c880 sub_78c880
    Lifted::new(0x7000e78980, fn_7000e78980 as *const (), 0), // +0x78c980 sub_78c980
    Lifted::new(0x7000e78d60, fn_7000e78d60 as *const (), 0), // +0x78cd60 sub_78cd60
    Lifted::new(0x7000e79040, fn_7000e79040 as *const (), 0), // +0x78d040 sub_78d040
    Lifted::new(0x7000e79370, fn_7000e79370 as *const (), 0), // +0x78d370 sub_78d370
    Lifted::new(0x7000e797b0, fn_7000e797b0 as *const (), 0), // +0x78d7b0 sub_78d7b0
    Lifted::new(0x7000e79820, fn_7000e79820 as *const (), 0), // +0x78d820 sub_78d820
    Lifted::new(0x7000e798f0, fn_7000e798f0 as *const (), 0), // +0x78d8f0 sub_78d8f0
    Lifted::new(0x7000e79b60, fn_7000e79b60 as *const (), 0), // +0x78db60 sub_78db60
    Lifted::new(0x7000e79d80, fn_7000e79d80 as *const (), 0), // +0x78dd80 sub_78dd80
    Lifted::new(0x7000e7a390, fn_7000e7a390 as *const (), 0), // +0x78e390 sub_78e390
    Lifted::new(0x7000e7a540, fn_7000e7a540 as *const (), 0), // +0x78e540 sub_78e540
    Lifted::new(0x7000e7a6e0, fn_7000e7a6e0 as *const (), 0), // +0x78e6e0 sub_78e6e0
    Lifted::new(0x7000e7a9b0, fn_7000e7a9b0 as *const (), 0), // +0x78e9b0 sub_78e9b0
    Lifted::new(0x7000e7aab0, fn_7000e7aab0 as *const (), 0), // +0x78eab0 sub_78eab0
    Lifted::new(0x7000e7ab80, fn_7000e7ab80 as *const (), 0), // +0x78eb80 sub_78eb80
    Lifted::new(0x7000e7ac50, fn_7000e7ac50 as *const (), 0), // +0x78ec50 sub_78ec50
    Lifted::new(0x7000e7ac80, fn_7000e7ac80 as *const (), 0), // +0x78ec80 sub_78ec80
    Lifted::new(0x7000e7acb0, fn_7000e7acb0 as *const (), 0), // +0x78ecb0 sub_78ecb0
    Lifted::new(0x7000e7af20, fn_7000e7af20 as *const (), 0), // +0x78ef20 sub_78ef20
    Lifted::new(0x7000e7b1b0, fn_7000e7b1b0 as *const (), 0), // +0x78f1b0 sub_78f1b0
    Lifted::new(0x7000e7b2a0, fn_7000e7b2a0 as *const (), 0), // +0x78f2a0 sub_78f2a0
    Lifted::new(0x7000e7b320, fn_7000e7b320 as *const (), 2), // +0x78f320 sub_78f320
    Lifted::new(0x7000e7b5f0, fn_7000e7b5f0 as *const (), 0), // +0x78f5f0 sub_78f5f0
    Lifted::new(0x7000e7b610, fn_7000e7b610 as *const (), 0), // +0x78f610 sub_78f610
    Lifted::new(0x7000e7ba80, fn_7000e7ba80 as *const (), 0), // +0x78fa80 sub_78fa80
    Lifted::new(0x7000e7bc30, fn_7000e7bc30 as *const (), 0), // +0x78fc30 sub_78fc30
    Lifted::new(0x7000e7bdc0, fn_7000e7bdc0 as *const (), 2), // +0x78fdc0 sub_78fdc0
    Lifted::new(0x7000e7c150, fn_7000e7c150 as *const (), 1), // +0x790150 sub_790150
    Lifted::new(0x7000e7c180, fn_7000e7c180 as *const (), 0), // +0x790180 sub_790180
    Lifted::new(0x7000e7c6f0, tpl_1294::<0x7001d542b8> as *const (), 0), // +0x7906f0 sub_7906f0
    Lifted::new(0x7000e7c710, tpl_13::<0x7000a62410> as *const (), 0), // +0x790710 sub_790710
    Lifted::new(0x7000e7c730, tpl_13::<0x7000a62410> as *const (), 0), // +0x790730 sub_790730
    Lifted::new(0x7000e7c750, tpl_1571::<0x7001d54340> as *const (), 0), // +0x790750 sub_790750
    Lifted::new(0x7000e7c780, fn_7000e7c780 as *const (), 0), // +0x790780 sub_790780
    Lifted::new(0x7000e7c980, fn_7000e7c980 as *const (), 1), // +0x790980 sub_790980
    Lifted::new(0x7000e7cae0, fn_7000e7cae0 as *const (), 0), // +0x790ae0 sub_790ae0
    Lifted::new(0x7000e7ccb0, fn_7000e7ccb0 as *const (), 0), // +0x790cb0 sub_790cb0
    Lifted::new(0x7000e7cff0, fn_7000e7cff0 as *const (), 2), // +0x790ff0 sub_790ff0
    Lifted::new(0x7000e7d2a0, fn_7000e7d2a0 as *const (), 2), // +0x7912a0 sub_7912a0
    Lifted::new(0x7000e7d6c0, fn_7000e7d6c0 as *const (), 0), // +0x7916c0 sub_7916c0
    Lifted::new(0x7000e7d870, fn_7000e7d870 as *const (), 0), // +0x791870 sub_791870
    Lifted::new(0x7000e7d9e0, fn_7000e7d9e0 as *const (), 0), // +0x7919e0 sub_7919e0
    Lifted::new(0x7000e7dac0, fn_7000e7dac0 as *const (), 0), // +0x791ac0 sub_791ac0
    Lifted::new(0x7000e7e3f0, fn_7000e7e3f0 as *const (), 0), // +0x7923f0 sub_7923f0
    Lifted::new(0x7000e7ed80, fn_7000e7ed80 as *const (), 0), // +0x792d80 sub_792d80
    Lifted::new(0x7000e7f580, tpl_1572::<0x70007080e0, 0x7000e7d870, 0x70006f2a70, 0x70006f9a60, 0x7000e7cff0> as *const (), 0), // +0x793580 sub_793580
    Lifted::new(0x7000e7f6e0, tpl_1572::<0x70007080e0, 0x7000e7d870, 0x70006f2a70, 0x70006f9a60, 0x7000e7d2a0> as *const (), 0), // +0x7936e0 sub_7936e0
    Lifted::new(0x7000e7f850, fn_7000e7f850 as *const (), 0), // +0x793850 sub_793850
    Lifted::new(0x7000e7f910, fn_7000e7f910 as *const (), 0), // +0x793910 sub_793910
    Lifted::new(0x7000e7f9d0, fn_7000e7f9d0 as *const (), 0), // +0x7939d0 sub_7939d0
    Lifted::new(0x7000e7fa30, fn_7000e7fa30 as *const (), 2), // +0x793a30 sub_793a30
    Lifted::new(0x7000e80180, fn_7000e80180 as *const (), 0), // +0x794180 sub_794180
    Lifted::new(0x7000e80210, fn_7000e80210 as *const (), 0), // +0x794210 sub_794210
    Lifted::new(0x7000e80300, fn_7000e80300 as *const (), 0), // +0x794300 sub_794300
    Lifted::new(0x7000e80500, fn_7000e80500 as *const (), 0), // +0x794500 sub_794500
    Lifted::new(0x7000e805c0, fn_7000e805c0 as *const (), 2), // +0x7945c0 sub_7945c0
    Lifted::new(0x7000e80d40, fn_7000e80d40 as *const (), 0), // +0x794d40 sub_794d40
    Lifted::new(0x7000e80e30, fn_7000e80e30 as *const (), 0), // +0x794e30 sub_794e30
    Lifted::new(0x7000e80f20, fn_7000e80f20 as *const (), 0), // +0x794f20 sub_794f20
    Lifted::new(0x7000e81150, fn_7000e81150 as *const (), 2), // +0x795150 sub_795150
    Lifted::new(0x7000e81300, fn_7000e81300 as *const (), 2), // +0x795300 sub_795300
    Lifted::new(0x7000e81410, fn_7000e81410 as *const (), 0), // +0x795410 sub_795410
    Lifted::new(0x7000e81560, fn_7000e81560 as *const (), 0), // +0x795560 sub_795560
    Lifted::new(0x7000e81670, fn_7000e81670 as *const (), 2), // +0x795670 sub_795670
    Lifted::new(0x7000e816e0, fn_7000e816e0 as *const (), 0), // +0x7956e0 sub_7956e0
    Lifted::new(0x7000e81810, fn_7000e81810 as *const (), 0), // +0x795810 sub_795810
    Lifted::new(0x7000e818f0, fn_7000e818f0 as *const (), 0), // +0x7958f0 sub_7958f0
    Lifted::new(0x7000e81aa0, fn_7000e81aa0 as *const (), 0), // +0x795aa0 sub_795aa0
    Lifted::new(0x7000e81e30, fn_7000e81e30 as *const (), 0), // +0x795e30 sub_795e30
    Lifted::new(0x7000e821d0, fn_7000e821d0 as *const (), 0), // +0x7961d0 sub_7961d0
    Lifted::new(0x7000e82890, fn_7000e82890 as *const (), 2), // +0x796890 sub_796890
    Lifted::new(0x7000e82ab0, fn_7000e82ab0 as *const (), 0), // +0x796ab0 sub_796ab0
    Lifted::new(0x7000e82b90, fn_7000e82b90 as *const (), 2), // +0x796b90 sub_796b90
    Lifted::new(0x7000e83170, fn_7000e83170 as *const (), 0), // +0x797170 sub_797170
    Lifted::new(0x7000e83200, fn_7000e83200 as *const (), 2), // +0x797200 sub_797200
    Lifted::new(0x7000e83750, fn_7000e83750 as *const (), 0), // +0x797750 sub_797750
    Lifted::new(0x7000e83a80, fn_7000e83a80 as *const (), 0), // +0x797a80 sub_797a80
    Lifted::new(0x7000e83b10, fn_7000e83b10 as *const (), 0), // +0x797b10 sub_797b10
    Lifted::new(0x7000e83be0, fn_7000e83be0 as *const (), 2), // +0x797be0 sub_797be0
    Lifted::new(0x7000e840c0, fn_7000e840c0 as *const (), 0), // +0x7980c0 sub_7980c0
    Lifted::new(0x7000e841d0, fn_7000e841d0 as *const (), 2), // +0x7981d0 sub_7981d0
    Lifted::new(0x7000e842a0, fn_7000e842a0 as *const (), 2), // +0x7982a0 sub_7982a0
    Lifted::new(0x7000e84650, fn_7000e84650 as *const (), 0), // +0x798650 sub_798650
    Lifted::new(0x7000e84670, fn_7000e84670 as *const (), 0), // +0x798670 sub_798670
    Lifted::new(0x7000e84980, fn_7000e84980 as *const (), 0), // +0x798980 sub_798980
    Lifted::new(0x7000e84b40, fn_7000e84b40 as *const (), 0), // +0x798b40 sub_798b40
    Lifted::new(0x7000e84bb0, fn_7000e84bb0 as *const (), 0), // +0x798bb0 sub_798bb0
    Lifted::new(0x7000e85630, fn_7000e85630 as *const (), 0), // +0x799630 sub_799630
    Lifted::new(0x7000e85eb0, fn_7000e85eb0 as *const (), 0), // +0x799eb0 sub_799eb0
    Lifted::new(0x7000e86050, fn_7000e86050 as *const (), 0), // +0x79a050 sub_79a050
    Lifted::new(0x7000e86190, fn_7000e86190 as *const (), 0), // +0x79a190 sub_79a190
    Lifted::new(0x7000e86290, fn_7000e86290 as *const (), 2), // +0x79a290 sub_79a290
    Lifted::new(0x7000e86520, fn_7000e86520 as *const (), 2), // +0x79a520 sub_79a520
    Lifted::new(0x7000e86620, fn_7000e86620 as *const (), 0), // +0x79a620 sub_79a620
    Lifted::new(0x7000e86650, fn_7000e86650 as *const (), 0), // +0x79a650 sub_79a650
    Lifted::new(0x7000e86bd0, fn_7000e86bd0 as *const (), 0), // +0x79abd0 sub_79abd0
    Lifted::new(0x7000e86c00, fn_7000e86c00 as *const (), 0), // +0x79ac00 sub_79ac00
    Lifted::new(0x7000e86e90, fn_7000e86e90 as *const (), 0), // +0x79ae90 sub_79ae90
    Lifted::new(0x7000e870b0, fn_7000e870b0 as *const (), 0), // +0x79b0b0 sub_79b0b0
    Lifted::new(0x7000e871d0, fn_7000e871d0 as *const (), 0), // +0x79b1d0 sub_79b1d0
    Lifted::new(0x7000e872d0, fn_7000e872d0 as *const (), 0), // +0x79b2d0 sub_79b2d0
    Lifted::new(0x7000e87520, fn_7000e87520 as *const (), 0), // +0x79b520 sub_79b520
    Lifted::new(0x7000e875a0, fn_7000e875a0 as *const (), 0), // +0x79b5a0 sub_79b5a0
    Lifted::new(0x7000e87780, fn_7000e87780 as *const (), 0), // +0x79b780 sub_79b780
    Lifted::new(0x7000e879d0, fn_7000e879d0 as *const (), 1), // +0x79b9d0 sub_79b9d0
    Lifted::new(0x7000e87b00, fn_7000e87b00 as *const (), 2), // +0x79bb00 sub_79bb00
    Lifted::new(0x7000e87c10, fn_7000e87c10 as *const (), 0), // +0x79bc10 sub_79bc10
    Lifted::new(0x7000e87c60, fn_7000e87c60 as *const (), 0), // +0x79bc60 sub_79bc60
    Lifted::new(0x7000e87e40, fn_7000e87e40 as *const (), 0), // +0x79be40 sub_79be40
    Lifted::new(0x7000e87e90, fn_7000e87e90 as *const (), 0), // +0x79be90 sub_79be90
    Lifted::new(0x7000e87f40, fn_7000e87f40 as *const (), 0), // +0x79bf40 sub_79bf40
    Lifted::new(0x7000e88220, fn_7000e88220 as *const (), 0), // +0x79c220 sub_79c220
    Lifted::new(0x7000e88270, fn_7000e88270 as *const (), 2), // +0x79c270 sub_79c270
    Lifted::new(0x7000e888a0, tpl_1573::<0x7000710c10, 0x700070f9b0> as *const (), 0), // +0x79c8a0 sub_79c8a0
    Lifted::new(0x7000e88930, fn_7000e88930 as *const (), 0), // +0x79c930 sub_79c930
    Lifted::new(0x7000e88b10, fn_7000e88b10 as *const (), 0), // +0x79cb10 sub_79cb10
    Lifted::new(0x7000e88c90, fn_7000e88c90 as *const (), 0), // +0x79cc90 sub_79cc90
    Lifted::new(0x7000e88ce0, fn_7000e88ce0 as *const (), 2), // +0x79cce0 sub_79cce0
    Lifted::new(0x7000e89380, fn_7000e89380 as *const (), 2), // +0x79d380 sub_79d380
    Lifted::new(0x7000e899c0, fn_7000e899c0 as *const (), 0), // +0x79d9c0 sub_79d9c0
    Lifted::new(0x7000e89bb0, fn_7000e89bb0 as *const (), 0), // +0x79dbb0 sub_79dbb0
    Lifted::new(0x7000e89c10, fn_7000e89c10 as *const (), 0), // +0x79dc10 sub_79dc10
    Lifted::new(0x7000e89ec0, fn_7000e89ec0 as *const (), 1), // +0x79dec0 sub_79dec0
    Lifted::new(0x7000e89fd0, fn_7000e89fd0 as *const (), 2), // +0x79dfd0 sub_79dfd0
    Lifted::new(0x7000e8a190, fn_7000e8a190 as *const (), 2), // +0x79e190 sub_79e190
    Lifted::new(0x7000e8a460, fn_7000e8a460 as *const (), 0), // +0x79e460 sub_79e460
    Lifted::new(0x7000e8a740, fn_7000e8a740 as *const (), 0), // +0x79e740 sub_79e740
    Lifted::new(0x7000e8a7e0, fn_7000e8a7e0 as *const (), 0), // +0x79e7e0 sub_79e7e0
    Lifted::new(0x7000e8aad0, fn_7000e8aad0 as *const (), 2), // +0x79ead0 sub_79ead0
    Lifted::new(0x7000e8ad10, fn_7000e8ad10 as *const (), 0), // +0x79ed10 sub_79ed10
    Lifted::new(0x7000e8aee0, fn_7000e8aee0 as *const (), 0), // +0x79eee0 sub_79eee0
    Lifted::new(0x7000e8b000, fn_7000e8b000 as *const (), 0), // +0x79f000 sub_79f000
    Lifted::new(0x7000e8b080, fn_7000e8b080 as *const (), 0), // +0x79f080 sub_79f080
    Lifted::new(0x7000e8b180, fn_7000e8b180 as *const (), 0), // +0x79f180 sub_79f180
    Lifted::new(0x7000e8b230, fn_7000e8b230 as *const (), 0), // +0x79f230 sub_79f230
    Lifted::new(0x7000e8b2f0, fn_7000e8b2f0 as *const (), 0), // +0x79f2f0 sub_79f2f0
    Lifted::new(0x7000e8b590, fn_7000e8b590 as *const (), 0), // +0x79f590 sub_79f590
    Lifted::new(0x7000e8b5d0, fn_7000e8b5d0 as *const (), 0), // +0x79f5d0 sub_79f5d0
    Lifted::new(0x7000e8b840, fn_7000e8b840 as *const (), 1), // +0x79f840 sub_79f840
    Lifted::new(0x7000e8b8d0, fn_7000e8b8d0 as *const (), 1), // +0x79f8d0 sub_79f8d0
    Lifted::new(0x7000e8ba70, fn_7000e8ba70 as *const (), 0), // +0x79fa70 sub_79fa70
    Lifted::new(0x7000e8bad0, fn_7000e8bad0 as *const (), 1), // +0x79fad0 sub_79fad0
    Lifted::new(0x7000e8bb50, fn_7000e8bb50 as *const (), 0), // +0x79fb50 sub_79fb50
    Lifted::new(0x7000e8bd70, fn_7000e8bd70 as *const (), 0), // +0x79fd70 sub_79fd70
    Lifted::new(0x7000e8bde0, fn_7000e8bde0 as *const (), 0), // +0x79fde0 sub_79fde0
    Lifted::new(0x7000e8c100, fn_7000e8c100 as *const (), 0), // +0x7a0100 sub_7a0100
    Lifted::new(0x7000e8c2e0, fn_7000e8c2e0 as *const (), 2), // +0x7a02e0 sub_7a02e0
    Lifted::new(0x7000e8cdd0, fn_7000e8cdd0 as *const (), 2), // +0x7a0dd0 sub_7a0dd0
    Lifted::new(0x7000e8d2b0, fn_7000e8d2b0 as *const (), 0), // +0x7a12b0 sub_7a12b0
    Lifted::new(0x7000e8d5b0, fn_7000e8d5b0 as *const (), 2), // +0x7a15b0 sub_7a15b0
    Lifted::new(0x7000e8e3a0, fn_7000e8e3a0 as *const (), 0), // +0x7a23a0 sub_7a23a0
    Lifted::new(0x7000e8ed00, fn_7000e8ed00 as *const (), 0), // +0x7a2d00 sub_7a2d00
    Lifted::new(0x7000e8ee30, fn_7000e8ee30 as *const (), 2), // +0x7a2e30 sub_7a2e30
    Lifted::new(0x7000e8f8a0, fn_7000e8f8a0 as *const (), 0), // +0x7a38a0 sub_7a38a0
    Lifted::new(0x7000e8fa50, fn_7000e8fa50 as *const (), 2), // +0x7a3a50 sub_7a3a50
    Lifted::new(0x7000e8fbc0, fn_7000e8fbc0 as *const (), 0), // +0x7a3bc0 sub_7a3bc0
    Lifted::new(0x7000e8fd40, fn_7000e8fd40 as *const (), 0), // +0x7a3d40 sub_7a3d40
    Lifted::new(0x7000e90050, fn_7000e90050 as *const (), 0), // +0x7a4050 sub_7a4050
    Lifted::new(0x7000e90170, fn_7000e90170 as *const (), 0), // +0x7a4170 sub_7a4170
    Lifted::new(0x7000e903f0, fn_7000e903f0 as *const (), 0), // +0x7a43f0 sub_7a43f0
    Lifted::new(0x7000e90490, fn_7000e90490 as *const (), 0), // +0x7a4490 sub_7a4490
    Lifted::new(0x7000e90980, fn_7000e90980 as *const (), 2), // +0x7a4980 sub_7a4980
    Lifted::new(0x7000e90bd0, fn_7000e90bd0 as *const (), 0), // +0x7a4bd0 sub_7a4bd0
    Lifted::new(0x7000e90c50, fn_7000e90c50 as *const (), 2), // +0x7a4c50 sub_7a4c50
    Lifted::new(0x7000e90de0, fn_7000e90de0 as *const (), 2), // +0x7a4de0 sub_7a4de0
    Lifted::new(0x7000e91010, fn_7000e91010 as *const (), 0), // +0x7a5010 sub_7a5010
    Lifted::new(0x7000e91500, fn_7000e91500 as *const (), 0), // +0x7a5500 sub_7a5500
    Lifted::new(0x7000e91570, fn_7000e91570 as *const (), 0), // +0x7a5570 sub_7a5570
    Lifted::new(0x7000e91770, fn_7000e91770 as *const (), 0), // +0x7a5770 sub_7a5770
    Lifted::new(0x7000e91b50, fn_7000e91b50 as *const (), 1), // +0x7a5b50 sub_7a5b50
    Lifted::new(0x7000e91b80, fn_7000e91b80 as *const (), 1), // +0x7a5b80 sub_7a5b80
    Lifted::new(0x7000e91c80, fn_7000e91c80 as *const (), 0), // +0x7a5c80 sub_7a5c80
    Lifted::new(0x7000e91d00, fn_7000e91d00 as *const (), 0), // +0x7a5d00 sub_7a5d00
    Lifted::new(0x7000e91da0, fn_7000e91da0 as *const (), 0), // +0x7a5da0 sub_7a5da0
    Lifted::new(0x7000e92040, fn_7000e92040 as *const (), 0), // +0x7a6040 sub_7a6040
    Lifted::new(0x7000e92220, fn_7000e92220 as *const (), 2), // +0x7a6220 sub_7a6220
    Lifted::new(0x7000e924a0, fn_7000e924a0 as *const (), 1), // +0x7a64a0 sub_7a64a0
    Lifted::new(0x7000e92590, fn_7000e92590 as *const (), 0), // +0x7a6590 sub_7a6590
    Lifted::new(0x7000e92660, fn_7000e92660 as *const (), 0), // +0x7a6660 sub_7a6660
    Lifted::new(0x7000e92790, fn_7000e92790 as *const (), 0), // +0x7a6790 sub_7a6790
    Lifted::new(0x7000e92860, fn_7000e92860 as *const (), 2), // +0x7a6860 sub_7a6860
    Lifted::new(0x7000e92db0, fn_7000e92db0 as *const (), 0), // +0x7a6db0 sub_7a6db0
    Lifted::new(0x7000e93260, fn_7000e93260 as *const (), 2), // +0x7a7260 sub_7a7260
    Lifted::new(0x7000e937f0, fn_7000e937f0 as *const (), 0), // +0x7a77f0 sub_7a77f0
    Lifted::new(0x7000e93890, fn_7000e93890 as *const (), 0), // +0x7a7890 sub_7a7890
    Lifted::new(0x7000e93940, fn_7000e93940 as *const (), 0), // +0x7a7940 sub_7a7940
    Lifted::new(0x7000e93ab0, fn_7000e93ab0 as *const (), 0), // +0x7a7ab0 sub_7a7ab0
    Lifted::new(0x7000e93c50, fn_7000e93c50 as *const (), 0), // +0x7a7c50 sub_7a7c50
    Lifted::new(0x7000e93cd0, fn_7000e93cd0 as *const (), 2), // +0x7a7cd0 sub_7a7cd0
    Lifted::new(0x7000e94110, fn_7000e94110 as *const (), 0), // +0x7a8110 sub_7a8110
    Lifted::new(0x7000e946c0, fn_7000e946c0 as *const (), 0), // +0x7a86c0 sub_7a86c0
    Lifted::new(0x7000e94730, fn_7000e94730 as *const (), 2), // +0x7a8730 sub_7a8730
    Lifted::new(0x7000e948b0, fn_7000e948b0 as *const (), 2), // +0x7a88b0 sub_7a88b0
    Lifted::new(0x7000e94940, fn_7000e94940 as *const (), 0), // +0x7a8940 sub_7a8940
    Lifted::new(0x7000e94a10, fn_7000e94a10 as *const (), 2), // +0x7a8a10 sub_7a8a10
    Lifted::new(0x7000e94d70, fn_7000e94d70 as *const (), 0), // +0x7a8d70 sub_7a8d70
    Lifted::new(0x7000e94fb0, fn_7000e94fb0 as *const (), 0), // +0x7a8fb0 sub_7a8fb0
    Lifted::new(0x7000e950d0, fn_7000e950d0 as *const (), 0), // +0x7a90d0 sub_7a90d0
    Lifted::new(0x7000e95140, fn_7000e95140 as *const (), 1), // +0x7a9140 sub_7a9140
    Lifted::new(0x7000e953b0, fn_7000e953b0 as *const (), 1), // +0x7a93b0 sub_7a93b0
    Lifted::new(0x7000e95400, fn_7000e95400 as *const (), 0), // +0x7a9400 sub_7a9400
    Lifted::new(0x7000e95590, fn_7000e95590 as *const (), 0), // +0x7a9590 sub_7a9590
    Lifted::new(0x7000e95810, fn_7000e95810 as *const (), 0), // +0x7a9810 sub_7a9810
    Lifted::new(0x7000e95970, fn_7000e95970 as *const (), 0), // +0x7a9970 sub_7a9970
    Lifted::new(0x7000e95c00, fn_7000e95c00 as *const (), 2), // +0x7a9c00 sub_7a9c00
    Lifted::new(0x7000e95dd0, fn_7000e95dd0 as *const (), 0), // +0x7a9dd0 sub_7a9dd0
    Lifted::new(0x7000e96450, tpl_15 as *const (), 1), // +0x7aa450 sub_7aa450
    Lifted::new(0x7000e96460, fn_7000e96460 as *const (), 0), // +0x7aa460 sub_7aa460
    Lifted::new(0x7000e96580, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000e96590> as *const (), 0), // +0x7aa580 sub_7aa580
    Lifted::new(0x7000e965a0, fn_7000e965a0 as *const (), 1), // +0x7aa5a0 sub_7aa5a0
    Lifted::new(0x7000e965c0, fn_7000e965c0 as *const (), 0), // +0x7aa5c0 sub_7aa5c0
    Lifted::new(0x7000e96e80, fn_7000e96e80 as *const (), 1), // +0x7aae80 sub_7aae80
    Lifted::new(0x7000e96fb0, fn_7000e96fb0 as *const (), 1), // +0x7aafb0 sub_7aafb0
    Lifted::new(0x7000e97060, fn_7000e97060 as *const (), 1), // +0x7ab060 sub_7ab060
    Lifted::new(0x7000e970f0, tpl_1574::<0x8, 0xffffffffffffffff> as *const (), 1), // +0x7ab0f0 sub_7ab0f0
    Lifted::new(0x7000e97110, tpl_367::<0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x7ab110 sub_7ab110
    Lifted::new(0x7000e971e0, tpl_549::<0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x7ab1e0 sub_7ab1e0
    Lifted::new(0x7000e972b0, fn_7000e972b0 as *const (), 0), // +0x7ab2b0 sub_7ab2b0
    Lifted::new(0x7000e972e0, fn_7000e972e0 as *const (), 0), // +0x7ab2e0 sub_7ab2e0
    Lifted::new(0x7000e974a0, fn_7000e974a0 as *const (), 0), // +0x7ab4a0 sub_7ab4a0
    Lifted::new(0x7000e97760, fn_7000e97760 as *const (), 0), // +0x7ab760 sub_7ab760
    Lifted::new(0x7000e97810, fn_7000e97810 as *const (), 2), // +0x7ab810 sub_7ab810
    Lifted::new(0x7000e97fd0, fn_7000e97fd0 as *const (), 1), // +0x7abfd0 sub_7abfd0
    Lifted::new(0x7000e980a0, fn_7000e980a0 as *const (), 1), // +0x7ac0a0 sub_7ac0a0
    Lifted::new(0x7000e980f0, fn_7000e980f0 as *const (), 0), // +0x7ac0f0 sub_7ac0f0
    Lifted::new(0x7000e985d0, fn_7000e985d0 as *const (), 3), // +0x7ac5d0 sub_7ac5d0
    Lifted::new(0x7000e987b0, fn_7000e987b0 as *const (), 2), // +0x7ac7b0 sub_7ac7b0
    Lifted::new(0x7000e989d0, fn_7000e989d0 as *const (), 0), // +0x7ac9d0 sub_7ac9d0
    Lifted::new(0x7000e98ad0, fn_7000e98ad0 as *const (), 2), // +0x7acad0 sub_7acad0
    Lifted::new(0x7000e98e20, fn_7000e98e20 as *const (), 0), // +0x7ace20 sub_7ace20
    Lifted::new(0x7000e98e70, fn_7000e98e70 as *const (), 1), // +0x7ace70 sub_7ace70
    Lifted::new(0x7000e98f10, fn_7000e98f10 as *const (), 0), // +0x7acf10 sub_7acf10
    Lifted::new(0x7000e99020, fn_7000e99020 as *const (), 0), // +0x7ad020 sub_7ad020
    Lifted::new(0x7000e99080, fn_7000e99080 as *const (), 2), // +0x7ad080 sub_7ad080
    Lifted::new(0x7000e99270, fn_7000e99270 as *const (), 1), // +0x7ad270 sub_7ad270
    Lifted::new(0x7000e99320, fn_7000e99320 as *const (), 0), // +0x7ad320 sub_7ad320
    Lifted::new(0x7000e99480, fn_7000e99480 as *const (), 0), // +0x7ad480 sub_7ad480
    Lifted::new(0x7000e994f0, fn_7000e994f0 as *const (), 0), // +0x7ad4f0 sub_7ad4f0
    Lifted::new(0x7000e99970, fn_7000e99970 as *const (), 0), // +0x7ad970 sub_7ad970
    Lifted::new(0x7000e99de0, fn_7000e99de0 as *const (), 0), // +0x7adde0 sub_7adde0
    Lifted::new(0x7000e9a0a0, fn_7000e9a0a0 as *const (), 0), // +0x7ae0a0 sub_7ae0a0
    Lifted::new(0x7000e9a360, fn_7000e9a360 as *const (), 0), // +0x7ae360 sub_7ae360
    Lifted::new(0x7000e9a590, fn_7000e9a590 as *const (), 0), // +0x7ae590 sub_7ae590
    Lifted::new(0x7000e9a7b0, fn_7000e9a7b0 as *const (), 2), // +0x7ae7b0 sub_7ae7b0
    Lifted::new(0x7000e9a9b0, fn_7000e9a9b0 as *const (), 2), // +0x7ae9b0 sub_7ae9b0
    Lifted::new(0x7000e9aba0, tpl_1575::<0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7, 0x168, 0x70006f1220, 0x700076df00, 0x7000e9a7b0> as *const (), 0), // +0x7aeba0 sub_7aeba0
    Lifted::new(0x7000e9acb0, tpl_1575::<0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x7, 0x168, 0x700075fa50, 0x700076df00, 0x7000e9a9b0> as *const (), 0), // +0x7aecb0 sub_7aecb0
    Lifted::new(0x7000e9adc0, tpl_1576::<0xffffffffffffffff, 0x7001c73200, 0xa, 0x7000ea14e0, 0x7000ea3230, 0x7001c7224f, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000e9ae71, 0x3, 0x7001857198, 0x7000735830> as *const (), 0), // +0x7aedc0 sub_7aedc0
    Lifted::new(0x7000e9aef0, tpl_1576::<0xffffffffffffffff, 0x7001c73200, 0xa, 0x7000ea2a50, 0x7000ea3230, 0x7001c7224f, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000e9afa1, 0x3, 0x7001857198, 0x7000735830> as *const (), 0), // +0x7aeef0 sub_7aeef0
    Lifted::new(0x7000e9b050, fn_7000e9b050 as *const (), 0), // +0x7af050 sub_7af050
    Lifted::new(0x7000e9b1e0, fn_7000e9b1e0 as *const (), 1), // +0x7af1e0 sub_7af1e0
    Lifted::new(0x7000e9b450, fn_7000e9b450 as *const (), 1), // +0x7af450 sub_7af450
    Lifted::new(0x7000e9b680, fn_7000e9b680 as *const (), 1), // +0x7af680 sub_7af680
    Lifted::new(0x7000e9b7e0, fn_7000e9b7e0 as *const (), 1), // +0x7af7e0 sub_7af7e0
    Lifted::new(0x7000e9b910, fn_7000e9b910 as *const (), 0), // +0x7af910 sub_7af910
    Lifted::new(0x7000e9bec0, fn_7000e9bec0 as *const (), 0), // +0x7afec0 sub_7afec0
    Lifted::new(0x7000e9c420, fn_7000e9c420 as *const (), 1), // +0x7b0420 sub_7b0420
    Lifted::new(0x7000e9c630, fn_7000e9c630 as *const (), 1), // +0x7b0630 sub_7b0630
    Lifted::new(0x7000e9c800, fn_7000e9c800 as *const (), 1), // +0x7b0800 sub_7b0800
    Lifted::new(0x7000e9c8f0, fn_7000e9c8f0 as *const (), 0), // +0x7b08f0 sub_7b08f0
    Lifted::new(0x7000e9cbd0, fn_7000e9cbd0 as *const (), 0), // +0x7b0bd0 sub_7b0bd0
    Lifted::new(0x7000e9ce90, fn_7000e9ce90 as *const (), 0), // +0x7b0e90 sub_7b0e90
    Lifted::new(0x7000e9d370, fn_7000e9d370 as *const (), 2), // +0x7b1370 sub_7b1370
    Lifted::new(0x7000e9d6f0, fn_7000e9d6f0 as *const (), 0), // +0x7b16f0 sub_7b16f0
    Lifted::new(0x7000e9d790, fn_7000e9d790 as *const (), 0), // +0x7b1790 sub_7b1790
    Lifted::new(0x7000e9d910, fn_7000e9d910 as *const (), 0), // +0x7b1910 sub_7b1910
    Lifted::new(0x7000e9d9a0, fn_7000e9d9a0 as *const (), 0), // +0x7b19a0 sub_7b19a0
    Lifted::new(0x7000e9da20, fn_7000e9da20 as *const (), 0), // +0x7b1a20 sub_7b1a20
    Lifted::new(0x7000e9dab0, fn_7000e9dab0 as *const (), 0), // +0x7b1ab0 sub_7b1ab0
    Lifted::new(0x7000e9dc50, fn_7000e9dc50 as *const (), 0), // +0x7b1c50 sub_7b1c50
    Lifted::new(0x7000e9ddf0, fn_7000e9ddf0 as *const (), 2), // +0x7b1df0 sub_7b1df0
    Lifted::new(0x7000e9e1d0, fn_7000e9e1d0 as *const (), 0), // +0x7b21d0 sub_7b21d0
    Lifted::new(0x7000e9e430, fn_7000e9e430 as *const (), 0), // +0x7b2430 sub_7b2430
    Lifted::new(0x7000e9e5b0, fn_7000e9e5b0 as *const (), 2), // +0x7b25b0 sub_7b25b0
    Lifted::new(0x7000e9f440, fn_7000e9f440 as *const (), 0), // +0x7b3440 sub_7b3440
    Lifted::new(0x7000e9f560, fn_7000e9f560 as *const (), 0), // +0x7b3560 sub_7b3560
    Lifted::new(0x7000e9fa40, fn_7000e9fa40 as *const (), 0), // +0x7b3a40 sub_7b3a40
    Lifted::new(0x7000ea01d0, fn_7000ea01d0 as *const (), 0), // +0x7b41d0 sub_7b41d0
    Lifted::new(0x7000ea0480, fn_7000ea0480 as *const (), 2), // +0x7b4480 sub_7b4480
    Lifted::new(0x7000ea07e0, fn_7000ea07e0 as *const (), 1), // +0x7b47e0 sub_7b47e0
    Lifted::new(0x7000ea08b0, fn_7000ea08b0 as *const (), 0), // +0x7b48b0 sub_7b48b0
    Lifted::new(0x7000ea09c0, fn_7000ea09c0 as *const (), 2), // +0x7b49c0 sub_7b49c0
    Lifted::new(0x7000ea1090, fn_7000ea1090 as *const (), 2), // +0x7b5090 sub_7b5090
    Lifted::new(0x7000ea14e0, fn_7000ea14e0 as *const (), 2), // +0x7b54e0 sub_7b54e0
    Lifted::new(0x7000ea1d80, fn_7000ea1d80 as *const (), 0), // +0x7b5d80 sub_7b5d80
    Lifted::new(0x7000ea1e60, fn_7000ea1e60 as *const (), 0), // +0x7b5e60 sub_7b5e60
    Lifted::new(0x7000ea2050, fn_7000ea2050 as *const (), 2), // +0x7b6050 sub_7b6050
    Lifted::new(0x7000ea2190, fn_7000ea2190 as *const (), 0), // +0x7b6190 sub_7b6190
    Lifted::new(0x7000ea23d0, fn_7000ea23d0 as *const (), 0), // +0x7b63d0 sub_7b63d0
    Lifted::new(0x7000ea2650, fn_7000ea2650 as *const (), 0), // +0x7b6650 sub_7b6650
    Lifted::new(0x7000ea2a50, fn_7000ea2a50 as *const (), 0), // +0x7b6a50 sub_7b6a50
    Lifted::new(0x7000ea2ba0, fn_7000ea2ba0 as *const (), 0), // +0x7b6ba0 sub_7b6ba0
    Lifted::new(0x7000ea2cd0, fn_7000ea2cd0 as *const (), 0), // +0x7b6cd0 sub_7b6cd0
    Lifted::new(0x7000ea2d80, fn_7000ea2d80 as *const (), 0), // +0x7b6d80 sub_7b6d80
    Lifted::new(0x7000ea2dd0, fn_7000ea2dd0 as *const (), 0), // +0x7b6dd0 sub_7b6dd0
    Lifted::new(0x7000ea2e50, fn_7000ea2e50 as *const (), 0), // +0x7b6e50 sub_7b6e50
    Lifted::new(0x7000ea3230, fn_7000ea3230 as *const (), 0), // +0x7b7230 sub_7b7230
    Lifted::new(0x7000ea3340, fn_7000ea3340 as *const (), 0), // +0x7b7340 sub_7b7340
    Lifted::new(0x7000ea36e0, fn_7000ea36e0 as *const (), 0), // +0x7b76e0 sub_7b76e0
    Lifted::new(0x7000ea3900, tpl_1577::<0x7000d0a1d0, 0x7000728210, 0x70006f3040, 0x700072c720, 0x7000ea8400, 0x7000766d20, 0x7000c18dd0, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000ea3abe, 0x7000c2e830, 0x7000ea3b87, 0x7001857218, 0x7000ea8540> as *const (), 2), // +0x7b7900 sub_7b7900
    Lifted::new(0x7000ea3ca0, tpl_1577::<0x7000d0a1d0, 0x7000728210, 0x70006f3040, 0x70006edee0, 0x7000ea8400, 0x7000766d20, 0x7000c18dd0, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000ea3e5e, 0x7000c2e830, 0x7000ea3f27, 0x7001857218, 0x7000ea8540> as *const (), 2), // +0x7b7ca0 sub_7b7ca0
    Lifted::new(0x7000ea4040, fn_7000ea4040 as *const (), 0), // +0x7b8040 sub_7b8040
    Lifted::new(0x7000ea46d0, fn_7000ea46d0 as *const (), 0), // +0x7b86d0 sub_7b86d0
    Lifted::new(0x7000ea4cc0, fn_7000ea4cc0 as *const (), 0), // +0x7b8cc0 sub_7b8cc0
    Lifted::new(0x7000ea52f0, fn_7000ea52f0 as *const (), 0), // +0x7b92f0 sub_7b92f0
    Lifted::new(0x7000ea5980, fn_7000ea5980 as *const (), 0), // +0x7b9980 sub_7b9980
    Lifted::new(0x7000ea5fc0, fn_7000ea5fc0 as *const (), 2), // +0x7b9fc0 sub_7b9fc0
    Lifted::new(0x7000ea6d50, fn_7000ea6d50 as *const (), 2), // +0x7bad50 sub_7bad50
    Lifted::new(0x7000ea7ba0, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x7bbba0 sub_7bbba0
    Lifted::new(0x7000ea7cb0, tpl_1578::<0x8, 0x28, 0x10, 0x70007395d0, 0x70006f4020, 0x70006f86f0> as *const (), 0), // +0x7bbcb0 sub_7bbcb0
    Lifted::new(0x7000ea7db0, fn_7000ea7db0 as *const (), 3), // +0x7bbdb0 sub_7bbdb0
    Lifted::new(0x7000ea7fa0, tpl_1579::<0x7000766d20, 0x70006f3040, 0x7000c18dd0, 0x7000ec4940> as *const (), 2), // +0x7bbfa0 sub_7bbfa0
    Lifted::new(0x7000ea8120, tpl_1579::<0x7000766d20, 0x70006f3040, 0x7000c18dd0, 0x7000ec49e0> as *const (), 2), // +0x7bc120 sub_7bc120
    Lifted::new(0x7000ea82a0, fn_7000ea82a0 as *const (), 0), // +0x7bc2a0 sub_7bc2a0
    Lifted::new(0x7000ea8340, fn_7000ea8340 as *const (), 0), // +0x7bc340 sub_7bc340
    Lifted::new(0x7000ea8400, tpl_1580::<0x7000767d50, 0x700074a010> as *const (), 0), // +0x7bc400 sub_7bc400
    Lifted::new(0x7000ea8540, tpl_1580::<0x7000767d50, 0x700074a010> as *const (), 0), // +0x7bc540 sub_7bc540
    Lifted::new(0x7000ea8680, fn_7000ea8680 as *const (), 1), // +0x7bc680 sub_7bc680
    Lifted::new(0x7000ea86b0, fn_7000ea86b0 as *const (), 2), // +0x7bc6b0 sub_7bc6b0
    Lifted::new(0x7000ea8720, fn_7000ea8720 as *const (), 2), // +0x7bc720 sub_7bc720
    Lifted::new(0x7000ea8790, fn_7000ea8790 as *const (), 2), // +0x7bc790 sub_7bc790
    Lifted::new(0x7000ea8800, fn_7000ea8800 as *const (), 2), // +0x7bc800 sub_7bc800
    Lifted::new(0x7000ea8870, fn_7000ea8870 as *const (), 2), // +0x7bc870 sub_7bc870
    Lifted::new(0x7000ea88e0, fn_7000ea88e0 as *const (), 0), // +0x7bc8e0 sub_7bc8e0
    Lifted::new(0x7000ea8db0, fn_7000ea8db0 as *const (), 0), // +0x7bcdb0 sub_7bcdb0
    Lifted::new(0x7000ea9060, fn_7000ea9060 as *const (), 0), // +0x7bd060 sub_7bd060
    Lifted::new(0x7000ea9350, fn_7000ea9350 as *const (), 0), // +0x7bd350 sub_7bd350
    Lifted::new(0x7000ea96d0, fn_7000ea96d0 as *const (), 0), // +0x7bd6d0 sub_7bd6d0
    Lifted::new(0x7000ea9d80, fn_7000ea9d80 as *const (), 0), // +0x7bdd80 sub_7bdd80
    Lifted::new(0x7000eaa210, tpl_1581::<0x38, 0x18, 0x4, 0xc, 0xffffffffffffffff, 0x7, 0x7000ea9060, 0x7000ea88e0> as *const (), 0), // +0x7be210 sub_7be210
    Lifted::new(0x7000eaa280, tpl_1581::<0x38, 0x18, 0x4, 0xc, 0xffffffffffffffff, 0x7, 0x7000ea9350, 0x7000ea8db0> as *const (), 0), // +0x7be280 sub_7be280
    Lifted::new(0x7000eaa2f0, tpl_1582::<0xc, 0x10, 0x50, 0x70006feb00> as *const (), 0), // +0x7be2f0 sub_7be2f0
    Lifted::new(0x7000eaa310, tpl_1582::<0xc, 0x10, 0x50, 0x700071f080> as *const (), 0), // +0x7be310 sub_7be310
    Lifted::new(0x7000eaa330, fn_7000eaa330 as *const (), 0), // +0x7be330 sub_7be330
    Lifted::new(0x7000eaa3d0, fn_7000eaa3d0 as *const (), 0), // +0x7be3d0 sub_7be3d0
    Lifted::new(0x7000eaa490, tpl_1583::<0x18, 0x40, 0x48, 0x38, 0xffffffffffffffff, 0x7, 0x7000726fc0, 0x8, 0x4, 0x7000ea9060, 0x7000ea88e0, 0x7000ea96d0> as *const (), 0), // +0x7be490 sub_7be490
    Lifted::new(0x7000eaa580, tpl_1583::<0x18, 0x40, 0x48, 0x38, 0xffffffffffffffff, 0x7, 0x7000726fc0, 0x8, 0x4, 0x7000ea9350, 0x7000ea8db0, 0x7000ea9d80> as *const (), 0), // +0x7be580 sub_7be580
    Lifted::new(0x7000eaa670, fn_7000eaa670 as *const (), 0), // +0x7be670 sub_7be670
    Lifted::new(0x7000eaa740, fn_7000eaa740 as *const (), 2), // +0x7be740 sub_7be740
    Lifted::new(0x7000eaa990, fn_7000eaa990 as *const (), 0), // +0x7be990 sub_7be990
    Lifted::new(0x7000eaaff0, fn_7000eaaff0 as *const (), 2), // +0x7beff0 sub_7beff0
    Lifted::new(0x7000eab220, fn_7000eab220 as *const (), 2), // +0x7bf220 sub_7bf220
    Lifted::new(0x7000eab850, tpl_39::<0x50> as *const (), 1), // +0x7bf850 sub_7bf850
    Lifted::new(0x7000eab860, tpl_1123::<0x7000791db0> as *const (), 0), // +0x7bf860 sub_7bf860
    Lifted::new(0x7000eab880, tpl_216::<0x7000791db0> as *const (), 0), // +0x7bf880 sub_7bf880
    Lifted::new(0x7000eab8a0, tpl_39::<0x50> as *const (), 1), // +0x7bf8a0 sub_7bf8a0
    Lifted::new(0x7000eab8b0, fn_7000eab8b0 as *const (), 0), // +0x7bf8b0 sub_7bf8b0
    Lifted::new(0x7000eab900, fn_7000eab900 as *const (), 0), // +0x7bf900 sub_7bf900
    Lifted::new(0x7000eaba20, fn_7000eaba20 as *const (), 0), // +0x7bfa20 sub_7bfa20
    Lifted::new(0x7000eabae0, fn_7000eabae0 as *const (), 0), // +0x7bfae0 sub_7bfae0
    Lifted::new(0x7000eabc00, fn_7000eabc00 as *const (), 0), // +0x7bfc00 sub_7bfc00
    Lifted::new(0x7000eabe00, fn_7000eabe00 as *const (), 0), // +0x7bfe00 sub_7bfe00
    Lifted::new(0x7000eabeb0, fn_7000eabeb0 as *const (), 1), // +0x7bfeb0 sub_7bfeb0
    Lifted::new(0x7000eabee0, fn_7000eabee0 as *const (), 1), // +0x7bfee0 sub_7bfee0
    Lifted::new(0x7000eabf60, fn_7000eabf60 as *const (), 0), // +0x7bff60 sub_7bff60
    Lifted::new(0x7000eac850, fn_7000eac850 as *const (), 2), // +0x7c0850 sub_7c0850
    Lifted::new(0x7000eac9e0, fn_7000eac9e0 as *const (), 2), // +0x7c09e0 sub_7c09e0
    Lifted::new(0x7000eacd30, fn_7000eacd30 as *const (), 0), // +0x7c0d30 sub_7c0d30
    Lifted::new(0x7000eace40, fn_7000eace40 as *const (), 0), // +0x7c0e40 sub_7c0e40
    Lifted::new(0x7000eacf00, fn_7000eacf00 as *const (), 1), // +0x7c0f00 sub_7c0f00
    Lifted::new(0x7000eacf60, fn_7000eacf60 as *const (), 0), // +0x7c0f60 sub_7c0f60
    Lifted::new(0x7000ead000, fn_7000ead000 as *const (), 0), // +0x7c1000 sub_7c1000
    Lifted::new(0x7000ead4f0, fn_7000ead4f0 as *const (), 0), // +0x7c14f0 sub_7c14f0
    Lifted::new(0x7000ead750, fn_7000ead750 as *const (), 2), // +0x7c1750 sub_7c1750
    Lifted::new(0x7000ead980, fn_7000ead980 as *const (), 0), // +0x7c1980 sub_7c1980
    Lifted::new(0x7000eadf60, fn_7000eadf60 as *const (), 2), // +0x7c1f60 sub_7c1f60
    Lifted::new(0x7000eadff0, fn_7000eadff0 as *const (), 2), // +0x7c1ff0 sub_7c1ff0
    Lifted::new(0x7000eae1e0, fn_7000eae1e0 as *const (), 2), // +0x7c21e0 sub_7c21e0
    Lifted::new(0x7000eae640, fn_7000eae640 as *const (), 0), // +0x7c2640 sub_7c2640
    Lifted::new(0x7000eaeb20, fn_7000eaeb20 as *const (), 0), // +0x7c2b20 sub_7c2b20
    Lifted::new(0x7000eaece0, fn_7000eaece0 as *const (), 0), // +0x7c2ce0 sub_7c2ce0
    Lifted::new(0x7000eaee20, fn_7000eaee20 as *const (), 2), // +0x7c2e20 sub_7c2e20
    Lifted::new(0x7000eaeee0, fn_7000eaeee0 as *const (), 2), // +0x7c2ee0 sub_7c2ee0
    Lifted::new(0x7000eaf160, fn_7000eaf160 as *const (), 0), // +0x7c3160 sub_7c3160
    Lifted::new(0x7000eaf1e0, fn_7000eaf1e0 as *const (), 0), // +0x7c31e0 sub_7c31e0
    Lifted::new(0x7000eaf460, fn_7000eaf460 as *const (), 0), // +0x7c3460 sub_7c3460
    Lifted::new(0x7000eaf590, fn_7000eaf590 as *const (), 2), // +0x7c3590 sub_7c3590
    Lifted::new(0x7000eaf7a0, fn_7000eaf7a0 as *const (), 2), // +0x7c37a0 sub_7c37a0
    Lifted::new(0x7000eafb00, fn_7000eafb00 as *const (), 1), // +0x7c3b00 sub_7c3b00
    Lifted::new(0x7000eafb70, fn_7000eafb70 as *const (), 1), // +0x7c3b70 sub_7c3b70
    Lifted::new(0x7000eafc20, fn_7000eafc20 as *const (), 1), // +0x7c3c20 sub_7c3c20
    Lifted::new(0x7000eafcd0, fn_7000eafcd0 as *const (), 0), // +0x7c3cd0 sub_7c3cd0
    Lifted::new(0x7000eafd10, fn_7000eafd10 as *const (), 0), // +0x7c3d10 sub_7c3d10
    Lifted::new(0x7000eafd80, tpl_55::<0x700177c0d0> as *const (), 0), // +0x7c3d80 sub_7c3d80
    Lifted::new(0x7000eafdc0, tpl_898::<0x700177c0e0> as *const (), 0), // +0x7c3dc0 sub_7c3dc0
    Lifted::new(0x7000eafe00, tpl_392::<0x700177c0e0> as *const (), 0), // +0x7c3e00 sub_7c3e00
    Lifted::new(0x7000eafe30, tpl_55::<0x700177c0d0> as *const (), 0), // +0x7c3e30 sub_7c3e30
    Lifted::new(0x7000eafe70, fn_7000eafe70 as *const (), 0), // +0x7c3e70 sub_7c3e70
    Lifted::new(0x7000eaffa0, fn_7000eaffa0 as *const (), 2), // +0x7c3fa0 sub_7c3fa0
    Lifted::new(0x7000eb0f00, fn_7000eb0f00 as *const (), 0), // +0x7c4f00 sub_7c4f00
    Lifted::new(0x7000eb1670, fn_7000eb1670 as *const (), 0), // +0x7c5670 sub_7c5670
    Lifted::new(0x7000eb1750, fn_7000eb1750 as *const (), 0), // +0x7c5750 sub_7c5750
    Lifted::new(0x7000eb1a80, fn_7000eb1a80 as *const (), 0), // +0x7c5a80 sub_7c5a80
    Lifted::new(0x7000eb1ec0, fn_7000eb1ec0 as *const (), 0), // +0x7c5ec0 sub_7c5ec0
    Lifted::new(0x7000eb2200, fn_7000eb2200 as *const (), 0), // +0x7c6200 sub_7c6200
    Lifted::new(0x7000eb2bf0, fn_7000eb2bf0 as *const (), 0), // +0x7c6bf0 sub_7c6bf0
    Lifted::new(0x7000eb2c10, fn_7000eb2c10 as *const (), 0), // +0x7c6c10 sub_7c6c10
    Lifted::new(0x7000eb3210, fn_7000eb3210 as *const (), 0), // +0x7c7210 sub_7c7210
    Lifted::new(0x7000eb32f0, fn_7000eb32f0 as *const (), 1), // +0x7c72f0 sub_7c72f0
    Lifted::new(0x7000eb3390, fn_7000eb3390 as *const (), 0), // +0x7c7390 sub_7c7390
    Lifted::new(0x7000eb3410, fn_7000eb3410 as *const (), 1), // +0x7c7410 sub_7c7410
    Lifted::new(0x7000eb3450, fn_7000eb3450 as *const (), 0), // +0x7c7450 sub_7c7450
    Lifted::new(0x7000eb3590, fn_7000eb3590 as *const (), 0), // +0x7c7590 sub_7c7590
    Lifted::new(0x7000eb3720, fn_7000eb3720 as *const (), 1), // +0x7c7720 sub_7c7720
    Lifted::new(0x7000eb3780, fn_7000eb3780 as *const (), 0), // +0x7c7780 sub_7c7780
    Lifted::new(0x7000eb37e0, fn_7000eb37e0 as *const (), 0), // +0x7c77e0 sub_7c77e0
    Lifted::new(0x7000eb3bc0, fn_7000eb3bc0 as *const (), 0), // +0x7c7bc0 sub_7c7bc0
    Lifted::new(0x7000eb3c90, fn_7000eb3c90 as *const (), 0), // +0x7c7c90 sub_7c7c90
    Lifted::new(0x7000eb4030, fn_7000eb4030 as *const (), 0), // +0x7c8030 sub_7c8030
    Lifted::new(0x7000eb4290, fn_7000eb4290 as *const (), 0), // +0x7c8290 sub_7c8290
    Lifted::new(0x7000eb4320, fn_7000eb4320 as *const (), 0), // +0x7c8320 sub_7c8320
    Lifted::new(0x7000eb4660, fn_7000eb4660 as *const (), 0), // +0x7c8660 sub_7c8660
    Lifted::new(0x7000eb4b10, fn_7000eb4b10 as *const (), 2), // +0x7c8b10 sub_7c8b10
    Lifted::new(0x7000eb4df0, fn_7000eb4df0 as *const (), 0), // +0x7c8df0 sub_7c8df0
    Lifted::new(0x7000eb4e90, fn_7000eb4e90 as *const (), 0), // +0x7c8e90 sub_7c8e90
    Lifted::new(0x7000eb54d0, fn_7000eb54d0 as *const (), 0), // +0x7c94d0 sub_7c94d0
    Lifted::new(0x7000eb5540, fn_7000eb5540 as *const (), 0), // +0x7c9540 sub_7c9540
    Lifted::new(0x7000eb55f0, tpl_900::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000eb57b8, 0x70007f0910, 0x7000eb57be, 0x7000eb5840, 0x7000eb57c7> as *const (), 0), // +0x7c95f0 sub_7c95f0
    Lifted::new(0x7000eb5840, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000eb5850> as *const (), 0), // +0x7c9840 sub_7c9840
    Lifted::new(0x7000eb5860, fn_7000eb5860 as *const (), 0), // +0x7c9860 sub_7c9860
    Lifted::new(0x7000eb5b20, fn_7000eb5b20 as *const (), 0), // +0x7c9b20 sub_7c9b20
    Lifted::new(0x7000eb5ca0, fn_7000eb5ca0 as *const (), 2), // +0x7c9ca0 sub_7c9ca0
    Lifted::new(0x7000eb6240, fn_7000eb6240 as *const (), 0), // +0x7ca240 sub_7ca240
    Lifted::new(0x7000eb6350, fn_7000eb6350 as *const (), 1), // +0x7ca350 sub_7ca350
    Lifted::new(0x7000eb64b0, fn_7000eb64b0 as *const (), 2), // +0x7ca4b0 sub_7ca4b0
    Lifted::new(0x7000eb6700, fn_7000eb6700 as *const (), 0), // +0x7ca700 sub_7ca700
    Lifted::new(0x7000eb6920, fn_7000eb6920 as *const (), 0), // +0x7ca920 sub_7ca920
    Lifted::new(0x7000eb6b00, fn_7000eb6b00 as *const (), 0), // +0x7cab00 sub_7cab00
    Lifted::new(0x7000eb6c80, fn_7000eb6c80 as *const (), 0), // +0x7cac80 sub_7cac80
    Lifted::new(0x7000eb6ef0, fn_7000eb6ef0 as *const (), 0), // +0x7caef0 sub_7caef0
    Lifted::new(0x7000eb7250, fn_7000eb7250 as *const (), 0), // +0x7cb250 sub_7cb250
    Lifted::new(0x7000eb7350, fn_7000eb7350 as *const (), 0), // +0x7cb350 sub_7cb350
    Lifted::new(0x7000eb74b0, fn_7000eb74b0 as *const (), 0), // +0x7cb4b0 sub_7cb4b0
    Lifted::new(0x7000eb7730, fn_7000eb7730 as *const (), 0), // +0x7cb730 sub_7cb730
    Lifted::new(0x7000eb7880, tpl_13::<0x7000eb7ba0> as *const (), 0), // +0x7cb880 sub_7cb880
    Lifted::new(0x7000eb78a0, fn_7000eb78a0 as *const (), 0), // +0x7cb8a0 sub_7cb8a0
    Lifted::new(0x7000eb7ba0, fn_7000eb7ba0 as *const (), 0), // +0x7cbba0 sub_7cbba0
    Lifted::new(0x7000eb7cd0, fn_7000eb7cd0 as *const (), 0), // +0x7cbcd0 sub_7cbcd0
    Lifted::new(0x7000eb7df0, fn_7000eb7df0 as *const (), 0), // +0x7cbdf0 sub_7cbdf0
    Lifted::new(0x7000eb8020, fn_7000eb8020 as *const (), 0), // +0x7cc020 sub_7cc020
    Lifted::new(0x7000eb81b0, fn_7000eb81b0 as *const (), 0), // +0x7cc1b0 sub_7cc1b0
    Lifted::new(0x7000eb8560, fn_7000eb8560 as *const (), 0), // +0x7cc560 sub_7cc560
    Lifted::new(0x7000eb8820, fn_7000eb8820 as *const (), 0), // +0x7cc820 sub_7cc820
    Lifted::new(0x7000eb8b30, fn_7000eb8b30 as *const (), 0), // +0x7ccb30 sub_7ccb30
    Lifted::new(0x7000eb8c40, fn_7000eb8c40 as *const (), 0), // +0x7ccc40 sub_7ccc40
    Lifted::new(0x7000eb9040, fn_7000eb9040 as *const (), 0), // +0x7cd040 sub_7cd040
    Lifted::new(0x7000eb9060, fn_7000eb9060 as *const (), 0), // +0x7cd060 sub_7cd060
    Lifted::new(0x7000eb9130, fn_7000eb9130 as *const (), 0), // +0x7cd130 sub_7cd130
    Lifted::new(0x7000eb93c0, fn_7000eb93c0 as *const (), 0), // +0x7cd3c0 sub_7cd3c0
    Lifted::new(0x7000eb9560, fn_7000eb9560 as *const (), 0), // +0x7cd560 sub_7cd560
    Lifted::new(0x7000eb99d0, fn_7000eb99d0 as *const (), 1), // +0x7cd9d0 sub_7cd9d0
    Lifted::new(0x7000eb9ad0, fn_7000eb9ad0 as *const (), 1), // +0x7cdad0 sub_7cdad0
    Lifted::new(0x7000eb9bc0, fn_7000eb9bc0 as *const (), 0), // +0x7cdbc0 sub_7cdbc0
    Lifted::new(0x7000eb9da0, fn_7000eb9da0 as *const (), 1), // +0x7cdda0 sub_7cdda0
    Lifted::new(0x7000eb9e60, fn_7000eb9e60 as *const (), 3), // +0x7cde60 sub_7cde60
    Lifted::new(0x7000eb9ed0, fn_7000eb9ed0 as *const (), 0), // +0x7cded0 sub_7cded0
    Lifted::new(0x7000eb9f60, fn_7000eb9f60 as *const (), 1), // +0x7cdf60 sub_7cdf60
    Lifted::new(0x7000eba000, fn_7000eba000 as *const (), 0), // +0x7ce000 sub_7ce000
    Lifted::new(0x7000eba6e0, fn_7000eba6e0 as *const (), 0), // +0x7ce6e0 sub_7ce6e0
    Lifted::new(0x7000ebaa80, fn_7000ebaa80 as *const (), 0), // +0x7cea80 sub_7cea80
    Lifted::new(0x7000ebae80, fn_7000ebae80 as *const (), 0), // +0x7cee80 sub_7cee80
    Lifted::new(0x7000ebb020, fn_7000ebb020 as *const (), 0), // +0x7cf020 sub_7cf020
    Lifted::new(0x7000ebb1a0, fn_7000ebb1a0 as *const (), 1), // +0x7cf1a0 sub_7cf1a0
    Lifted::new(0x7000ebb260, fn_7000ebb260 as *const (), 0), // +0x7cf260 sub_7cf260
    Lifted::new(0x7000ebba00, fn_7000ebba00 as *const (), 1), // +0x7cfa00 sub_7cfa00
    Lifted::new(0x7000ebbaf0, fn_7000ebbaf0 as *const (), 0), // +0x7cfaf0 sub_7cfaf0
    Lifted::new(0x7000ebbd10, fn_7000ebbd10 as *const (), 0), // +0x7cfd10 sub_7cfd10
    Lifted::new(0x7000ebbe60, fn_7000ebbe60 as *const (), 0), // +0x7cfe60 sub_7cfe60
    Lifted::new(0x7000ebbf50, fn_7000ebbf50 as *const (), 1), // +0x7cff50 sub_7cff50
    Lifted::new(0x7000ebc060, fn_7000ebc060 as *const (), 0), // +0x7d0060 sub_7d0060
    Lifted::new(0x7000ebc230, fn_7000ebc230 as *const (), 0), // +0x7d0230 sub_7d0230
    Lifted::new(0x7000ebc2b0, fn_7000ebc2b0 as *const (), 0), // +0x7d02b0 sub_7d02b0
    Lifted::new(0x7000ebc550, fn_7000ebc550 as *const (), 0), // +0x7d0550 sub_7d0550
    Lifted::new(0x7000ebc590, fn_7000ebc590 as *const (), 1), // +0x7d0590 sub_7d0590
    Lifted::new(0x7000ebc610, fn_7000ebc610 as *const (), 2), // +0x7d0610 sub_7d0610
    Lifted::new(0x7000ebcaa0, fn_7000ebcaa0 as *const (), 0), // +0x7d0aa0 sub_7d0aa0
    Lifted::new(0x7000ebcaf0, fn_7000ebcaf0 as *const (), 1), // +0x7d0af0 sub_7d0af0
    Lifted::new(0x7000ebcbb0, fn_7000ebcbb0 as *const (), 1), // +0x7d0bb0 sub_7d0bb0
    Lifted::new(0x7000ebcc40, fn_7000ebcc40 as *const (), 1), // +0x7d0c40 sub_7d0c40
    Lifted::new(0x7000ebcc80, fn_7000ebcc80 as *const (), 0), // +0x7d0c80 sub_7d0c80
    Lifted::new(0x7000ebcd40, fn_7000ebcd40 as *const (), 2), // +0x7d0d40 sub_7d0d40
    Lifted::new(0x7000ebcfc0, fn_7000ebcfc0 as *const (), 2), // +0x7d0fc0 sub_7d0fc0
    Lifted::new(0x7000ebd2b0, fn_7000ebd2b0 as *const (), 2), // +0x7d12b0 sub_7d12b0
    Lifted::new(0x7000ebd400, fn_7000ebd400 as *const (), 2), // +0x7d1400 sub_7d1400
    Lifted::new(0x7000ebd580, fn_7000ebd580 as *const (), 2), // +0x7d1580 sub_7d1580
    Lifted::new(0x7000ebd780, fn_7000ebd780 as *const (), 0), // +0x7d1780 sub_7d1780
    Lifted::new(0x7000ebd9b0, fn_7000ebd9b0 as *const (), 1), // +0x7d19b0 sub_7d19b0
    Lifted::new(0x7000ebda00, fn_7000ebda00 as *const (), 1), // +0x7d1a00 sub_7d1a00
    Lifted::new(0x7000ebdc20, fn_7000ebdc20 as *const (), 0), // +0x7d1c20 sub_7d1c20
    Lifted::new(0x7000ebe110, fn_7000ebe110 as *const (), 2), // +0x7d2110 sub_7d2110
    Lifted::new(0x7000ebe830, fn_7000ebe830 as *const (), 2), // +0x7d2830 sub_7d2830
    Lifted::new(0x7000ebef10, fn_7000ebef10 as *const (), 0), // +0x7d2f10 sub_7d2f10
    Lifted::new(0x7000ebef50, fn_7000ebef50 as *const (), 0), // +0x7d2f50 sub_7d2f50
    Lifted::new(0x7000ebf020, fn_7000ebf020 as *const (), 2), // +0x7d3020 sub_7d3020
    Lifted::new(0x7000ebf0a0, fn_7000ebf0a0 as *const (), 0), // +0x7d30a0 sub_7d30a0
    Lifted::new(0x7000ebf4c0, fn_7000ebf4c0 as *const (), 0), // +0x7d34c0 sub_7d34c0
    Lifted::new(0x7000ebf5b0, fn_7000ebf5b0 as *const (), 0), // +0x7d35b0 sub_7d35b0
    Lifted::new(0x7000ebf600, fn_7000ebf600 as *const (), 0), // +0x7d3600 sub_7d3600
    Lifted::new(0x7000ebf670, fn_7000ebf670 as *const (), 0), // +0x7d3670 sub_7d3670
    Lifted::new(0x7000ebf850, fn_7000ebf850 as *const (), 0), // +0x7d3850 sub_7d3850
    Lifted::new(0x7000ebf960, tpl_923::<0x7001857ad8> as *const (), 1), // +0x7d3960 sub_7d3960
    Lifted::new(0x7000ebf980, tpl_753::<0x7001857a58, 0x8> as *const (), 1), // +0x7d3980 sub_7d3980
    Lifted::new(0x7000ebf9a0, tpl_918::<0x7001857a98, 0x8> as *const (), 3), // +0x7d39a0 sub_7d39a0
    Lifted::new(0x7000ebf9c0, tpl_754::<0x700177c312> as *const (), 0), // +0x7d39c0 sub_7d39c0
    Lifted::new(0x7000ebf9e0, tpl_754::<0x700177c312> as *const (), 0), // +0x7d39e0 sub_7d39e0
    Lifted::new(0x7000ebfa00, tpl_919::<0x700177c312> as *const (), 0), // +0x7d3a00 sub_7d3a00
    Lifted::new(0x7000ebfa20, fn_7000ebfa20 as *const (), 1), // +0x7d3a20 sub_7d3a20
    Lifted::new(0x7000ebfa40, fn_7000ebfa40 as *const (), 1), // +0x7d3a40 sub_7d3a40
    Lifted::new(0x7000ebfa80, fn_7000ebfa80 as *const (), 0), // +0x7d3a80 sub_7d3a80
    Lifted::new(0x7000ebfab0, tpl_52::<0x8> as *const (), 1), // +0x7d3ab0 sub_7d3ab0
    Lifted::new(0x7000ebfac0, tpl_52::<0x8> as *const (), 1), // +0x7d3ac0 sub_7d3ac0
    Lifted::new(0x7000ebfad0, tpl_52::<0x8> as *const (), 1), // +0x7d3ad0 sub_7d3ad0
    Lifted::new(0x7000ebfae0, tpl_923::<0x7001857ad8> as *const (), 1), // +0x7d3ae0 sub_7d3ae0
    Lifted::new(0x7000ebfb00, tpl_753::<0x7001857a58, 0x8> as *const (), 1), // +0x7d3b00 sub_7d3b00
    Lifted::new(0x7000ebfb20, tpl_918::<0x7001857a98, 0x8> as *const (), 3), // +0x7d3b20 sub_7d3b20
    Lifted::new(0x7000ebfb40, tpl_315::<0x7001c6c370> as *const (), 1), // +0x7d3b40 sub_7d3b40
    Lifted::new(0x7000ebfb50, tpl_315::<0x7001c6c320> as *const (), 1), // +0x7d3b50 sub_7d3b50
    Lifted::new(0x7000ebfb60, tpl_315::<0x7001c6c3c0> as *const (), 1), // +0x7d3b60 sub_7d3b60
    Lifted::new(0x7000ebfb70, tpl_1584::<0x7001c6abd8> as *const (), 0), // +0x7d3b70 sub_7d3b70
    Lifted::new(0x7000ebfd20, fn_7000ebfd20 as *const (), 0), // +0x7d3d20 sub_7d3d20
    Lifted::new(0x7000ebfde0, fn_7000ebfde0 as *const (), 0), // +0x7d3de0 sub_7d3de0
    Lifted::new(0x7000ec0120, fn_7000ec0120 as *const (), 0), // +0x7d4120 sub_7d4120
    Lifted::new(0x7000ec0260, fn_7000ec0260 as *const (), 0), // +0x7d4260 sub_7d4260
    Lifted::new(0x7000ec0600, fn_7000ec0600 as *const (), 2), // +0x7d4600 sub_7d4600
    Lifted::new(0x7000ec0930, tpl_394::<0x700076bf20, 0x7000ec3770, 0x7000ec0aa2> as *const (), 0), // +0x7d4930 sub_7d4930
    Lifted::new(0x7000ec0b00, tpl_394::<0x700076bf20, 0x7000ec3750, 0x7000ec0c72> as *const (), 0), // +0x7d4b00 sub_7d4b00
    Lifted::new(0x7000ec0cd0, tpl_394::<0x700076bf20, 0x7000ec3770, 0x7000ec0e42> as *const (), 0), // +0x7d4cd0 sub_7d4cd0
    Lifted::new(0x7000ec0ea0, tpl_68 as *const (), 1), // +0x7d4ea0 sub_7d4ea0
    Lifted::new(0x7000ec0f50, tpl_68 as *const (), 1), // +0x7d4f50 sub_7d4f50
    Lifted::new(0x7000ec1060, fn_7000ec1060 as *const (), 0), // +0x7d5060 sub_7d5060
    Lifted::new(0x7000ec1200, fn_7000ec1200 as *const (), 0), // +0x7d5200 sub_7d5200
    Lifted::new(0x7000ec1550, fn_7000ec1550 as *const (), 0), // +0x7d5550 sub_7d5550
    Lifted::new(0x7000ec1690, tpl_334::<0x13> as *const (), 1), // +0x7d5690 sub_7d5690
    Lifted::new(0x7000ec16c0, fn_7000ec16c0 as *const (), 0), // +0x7d56c0 sub_7d56c0
    Lifted::new(0x7000ec1860, fn_7000ec1860 as *const (), 2), // +0x7d5860 sub_7d5860
    Lifted::new(0x7000ec2020, fn_7000ec2020 as *const (), 0), // +0x7d6020 sub_7d6020
    Lifted::new(0x7000ec2200, fn_7000ec2200 as *const (), 0), // +0x7d6200 sub_7d6200
    Lifted::new(0x7000ec2300, fn_7000ec2300 as *const (), 0), // +0x7d6300 sub_7d6300
    Lifted::new(0x7000ec25c0, fn_7000ec25c0 as *const (), 2), // +0x7d65c0 sub_7d65c0
    Lifted::new(0x7000ec2940, fn_7000ec2940 as *const (), 0), // +0x7d6940 sub_7d6940
    Lifted::new(0x7000ec2a80, fn_7000ec2a80 as *const (), 2), // +0x7d6a80 sub_7d6a80
    Lifted::new(0x7000ec2c60, fn_7000ec2c60 as *const (), 0), // +0x7d6c60 sub_7d6c60
    Lifted::new(0x7000ec2d20, fn_7000ec2d20 as *const (), 2), // +0x7d6d20 sub_7d6d20
    Lifted::new(0x7000ec2ea0, fn_7000ec2ea0 as *const (), 0), // +0x7d6ea0 sub_7d6ea0
    Lifted::new(0x7000ec30b0, fn_7000ec30b0 as *const (), 2), // +0x7d70b0 sub_7d70b0
    Lifted::new(0x7000ec31a0, tpl_935::<0xb> as *const (), 1), // +0x7d71a0 sub_7d71a0
    Lifted::new(0x7000ec31c0, tpl_1585::<0x10, 0x8, 0x18, 0x700076bf20, 0x7000ec0ea0> as *const (), 0), // +0x7d71c0 sub_7d71c0
    Lifted::new(0x7000ec32b0, tpl_1585::<0x10, 0x8, 0x18, 0x700076bf20, 0x7000ec0f50> as *const (), 0), // +0x7d72b0 sub_7d72b0
    Lifted::new(0x7000ec33a0, tpl_1586::<0x7000ec31c0, 0x70017d37a0, 0x70017790e0, 0x7000ec3515> as *const (), 0), // +0x7d73a0 sub_7d73a0
    Lifted::new(0x7000ec3580, fn_7000ec3580 as *const (), 0), // +0x7d7580 sub_7d7580
    Lifted::new(0x7000ec3750, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000ec3760> as *const (), 0), // +0x7d7750 sub_7d7750
    Lifted::new(0x7000ec3770, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000ec3780> as *const (), 0), // +0x7d7770 sub_7d7770
    Lifted::new(0x7000ec3790, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ec37a1> as *const (), 0), // +0x7d7790 sub_7d7790
    Lifted::new(0x7000ec37b0, fn_7000ec37b0 as *const (), 2), // +0x7d77b0 sub_7d77b0
    Lifted::new(0x7000ec3a40, fn_7000ec3a40 as *const (), 0), // +0x7d7a40 sub_7d7a40
    Lifted::new(0x7000ec3c00, fn_7000ec3c00 as *const (), 0), // +0x7d7c00 sub_7d7c00
    Lifted::new(0x7000ec3e50, fn_7000ec3e50 as *const (), 0), // +0x7d7e50 sub_7d7e50
    Lifted::new(0x7000ec4040, fn_7000ec4040 as *const (), 2), // +0x7d8040 sub_7d8040
    Lifted::new(0x7000ec42e0, fn_7000ec42e0 as *const (), 0), // +0x7d82e0 sub_7d82e0
    Lifted::new(0x7000ec44b0, fn_7000ec44b0 as *const (), 0), // +0x7d84b0 sub_7d84b0
    Lifted::new(0x7000ec46f0, fn_7000ec46f0 as *const (), 0), // +0x7d86f0 sub_7d86f0
    Lifted::new(0x7000ec48f0, tpl_1587::<0x7000717350> as *const (), 0), // +0x7d88f0 sub_7d88f0
    Lifted::new(0x7000ec4940, tpl_1587::<0x70007a6010> as *const (), 0), // +0x7d8940 sub_7d8940
    Lifted::new(0x7000ec4990, tpl_1587::<0x70007009e0> as *const (), 0), // +0x7d8990 sub_7d8990
    Lifted::new(0x7000ec49e0, tpl_1587::<0x70006f3f20> as *const (), 0), // +0x7d89e0 sub_7d89e0
    Lifted::new(0x7000ec4a30, fn_7000ec4a30 as *const (), 0), // +0x7d8a30 sub_7d8a30
    Lifted::new(0x7000ec4a90, fn_7000ec4a90 as *const (), 0), // +0x7d8a90 sub_7d8a90
    Lifted::new(0x7000ec4b00, fn_7000ec4b00 as *const (), 2), // +0x7d8b00 sub_7d8b00
    Lifted::new(0x7000ec4e10, fn_7000ec4e10 as *const (), 0), // +0x7d8e10 sub_7d8e10
    Lifted::new(0x7000ec5640, fn_7000ec5640 as *const (), 0), // +0x7d9640 sub_7d9640
    Lifted::new(0x7000ec64c0, fn_7000ec64c0 as *const (), 1), // +0x7da4c0 sub_7da4c0
    Lifted::new(0x7000ec6580, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ec6591> as *const (), 0), // +0x7da580 sub_7da580
    Lifted::new(0x7000ec65a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ec65b1> as *const (), 0), // +0x7da5a0 sub_7da5a0
    Lifted::new(0x7000ec65c0, tpl_1588::<0x7000799d20, 0x7000769b00> as *const (), 0), // +0x7da5c0 sub_7da5c0
    Lifted::new(0x7000ec6700, tpl_1588::<0x7000730960, 0x700070e790> as *const (), 0), // +0x7da700 sub_7da700
    Lifted::new(0x7000ec6840, fn_7000ec6840 as *const (), 0), // +0x7da840 sub_7da840
    Lifted::new(0x7000ec6a10, tpl_1588::<0x7000746580, 0x7000735760> as *const (), 0), // +0x7daa10 sub_7daa10
    Lifted::new(0x7000ec6b50, tpl_1588::<0x70006f35f0, 0x70006fd6a0> as *const (), 0), // +0x7dab50 sub_7dab50
    Lifted::new(0x7000ec6c90, tpl_1589::<0x7001d55098, 0x70007a5b20, 0x7000e970f0, 0x7000738880, 0x7001859148, 0x700070b750, 0x7000ec6e02> as *const (), 0), // +0x7dac90 sub_7dac90
    Lifted::new(0x7000ec6e60, tpl_1588::<0x7000738be0, 0x70006f4b10> as *const (), 0), // +0x7dae60 sub_7dae60
    Lifted::new(0x7000ec6fa0, tpl_1588::<0x70007175e0, 0x7000722e90> as *const (), 0), // +0x7dafa0 sub_7dafa0
    Lifted::new(0x7000ec70e0, tpl_1589::<0x7001d55098, 0x70007a5b20, 0x7000e970f0, 0x7000742220, 0x7001859148, 0x700070b750, 0x7000ec7252> as *const (), 0), // +0x7db0e0 sub_7db0e0
    Lifted::new(0x7000ec72b0, tpl_1590::<0x7000728ac0, 0x7000769b00> as *const (), 0), // +0x7db2b0 sub_7db2b0
    Lifted::new(0x7000ec7400, tpl_1590::<0x70007a60d0, 0x700070e790> as *const (), 0), // +0x7db400 sub_7db400
    Lifted::new(0x7000ec7550, tpl_1590::<0x70007a3e10, 0x7000725af0> as *const (), 0), // +0x7db550 sub_7db550
    Lifted::new(0x7000ec76a0, tpl_1590::<0x700077ab80, 0x7000735760> as *const (), 0), // +0x7db6a0 sub_7db6a0
    Lifted::new(0x7000ec77f0, tpl_1590::<0x70006f8390, 0x70006fd6a0> as *const (), 0), // +0x7db7f0 sub_7db7f0
    Lifted::new(0x7000ec7940, tpl_1591::<0x7001d55098, 0x700076dfa0, 0x7000e970f0, 0x7000738880, 0x7001859148, 0x7000732840, 0x7000ec7abc> as *const (), 0), // +0x7db940 sub_7db940
    Lifted::new(0x7000ec7b20, tpl_1590::<0x700073c230, 0x70006f4b10> as *const (), 0), // +0x7dbb20 sub_7dbb20
    Lifted::new(0x7000ec7c70, tpl_1590::<0x7000779480, 0x7000722e90> as *const (), 0), // +0x7dbc70 sub_7dbc70
    Lifted::new(0x7000ec7dc0, tpl_1591::<0x7001d55098, 0x700076dfa0, 0x7000e970f0, 0x7000742220, 0x7001859148, 0x7000732840, 0x7000ec7f3c> as *const (), 0), // +0x7dbdc0 sub_7dbdc0
    Lifted::new(0x7000ec7fa0, fn_7000ec7fa0 as *const (), 0), // +0x7dbfa0 sub_7dbfa0
    Lifted::new(0x7000ec81d0, fn_7000ec81d0 as *const (), 0), // +0x7dc1d0 sub_7dc1d0
    Lifted::new(0x7000ec83f0, tpl_1248::<0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x7dc3f0 sub_7dc3f0
    Lifted::new(0x7000ec8530, fn_7000ec8530 as *const (), 0), // +0x7dc530 sub_7dc530
    Lifted::new(0x7000ec86a0, fn_7000ec86a0 as *const (), 0), // +0x7dc6a0 sub_7dc6a0
    Lifted::new(0x7000ec8830, tpl_1174::<0x7000e87780> as *const (), 0), // +0x7dc830 sub_7dc830
    Lifted::new(0x7000ec89a0, tpl_1175::<0x88, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x7dc9a0 sub_7dc9a0
    Lifted::new(0x7000ec8a20, tpl_1592::<0x700070e500, 0x4, 0x7000e970f0, 0xffffffffffffffff> as *const (), 0), // +0x7dca20 sub_7dca20
    Lifted::new(0x7000ec8a90, tpl_1592::<0x7000746580, 0x4, 0x7000e970f0, 0xffffffffffffffff> as *const (), 0), // +0x7dca90 sub_7dca90
    Lifted::new(0x7000ec8b00, fn_7000ec8b00 as *const (), 0), // +0x7dcb00 sub_7dcb00
    Lifted::new(0x7000ec8c30, tpl_1593::<0x7001d55098, 0x2d8, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1, 0x7001859148, 0x700070b750, 0x7000ec8d1b> as *const (), 0), // +0x7dcc30 sub_7dcc30
    Lifted::new(0x7000ec8d60, fn_7000ec8d60 as *const (), 0), // +0x7dcd60 sub_7dcd60
    Lifted::new(0x7000ec8e90, fn_7000ec8e90 as *const (), 0), // +0x7dce90 sub_7dce90
    Lifted::new(0x7000ec8fc0, fn_7000ec8fc0 as *const (), 0), // +0x7dcfc0 sub_7dcfc0
    Lifted::new(0x7000ec90f0, tpl_1594::<0x7001d55098, 0xf0, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1, 0x7001859148, 0x700070b750, 0x7000ec91d7> as *const (), 0), // +0x7dd0f0 sub_7dd0f0
    Lifted::new(0x7000ec9220, tpl_1593::<0x7001d55098, 0xf0, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1, 0x7001859148, 0x700070b750, 0x7000ec930b> as *const (), 0), // +0x7dd220 sub_7dd220
    Lifted::new(0x7000ec9350, tpl_1593::<0x7001d55098, 0x270, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1, 0x7001859148, 0x700070b750, 0x7000ec943b> as *const (), 0), // +0x7dd350 sub_7dd350
    Lifted::new(0x7000ec9480, tpl_1594::<0x7001d55098, 0xf0, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1, 0x7001859148, 0x700070b750, 0x7000ec9567> as *const (), 0), // +0x7dd480 sub_7dd480
    Lifted::new(0x7000ec95b0, tpl_1592::<0x70007a3e10, 0x4, 0x7000e970f0, 0xffffffffffffffff> as *const (), 0), // +0x7dd5b0 sub_7dd5b0
    Lifted::new(0x7000ec9620, tpl_1592::<0x700077ab80, 0x4, 0x7000e970f0, 0xffffffffffffffff> as *const (), 0), // +0x7dd620 sub_7dd620
    Lifted::new(0x7000ec9690, fn_7000ec9690 as *const (), 0), // +0x7dd690 sub_7dd690
    Lifted::new(0x7000ec97c0, fn_7000ec97c0 as *const (), 0), // +0x7dd7c0 sub_7dd7c0
    Lifted::new(0x7000ec98f0, fn_7000ec98f0 as *const (), 0), // +0x7dd8f0 sub_7dd8f0
    Lifted::new(0x7000ec9a20, fn_7000ec9a20 as *const (), 0), // +0x7dda20 sub_7dda20
    Lifted::new(0x7000ec9b50, fn_7000ec9b50 as *const (), 0), // +0x7ddb50 sub_7ddb50
    Lifted::new(0x7000ec9c80, tpl_1595::<0x7001d55098, 0x3, 0x700076dfa0, 0x7000e970f0, 0x1, 0x7001859148, 0x7000732840, 0x7000ec9d67> as *const (), 0), // +0x7ddc80 sub_7ddc80
    Lifted::new(0x7000ec9db0, fn_7000ec9db0 as *const (), 0), // +0x7dddb0 sub_7dddb0
    Lifted::new(0x7000ec9ee0, fn_7000ec9ee0 as *const (), 0), // +0x7ddee0 sub_7ddee0
    Lifted::new(0x7000eca010, tpl_1595::<0x7001d55098, 0x3, 0x700076dfa0, 0x7000e970f0, 0x1, 0x7001859148, 0x7000732840, 0x7000eca0f7> as *const (), 0), // +0x7de010 sub_7de010
    Lifted::new(0x7000eca140, fn_7000eca140 as *const (), 0), // +0x7de140 sub_7de140
    Lifted::new(0x7000eca220, fn_7000eca220 as *const (), 0), // +0x7de220 sub_7de220
    Lifted::new(0x7000eca2f0, tpl_1596::<0x2d8, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1> as *const (), 0), // +0x7de2f0 sub_7de2f0
    Lifted::new(0x7000eca3c0, tpl_1597::<0x1f0, 0x5, 0x70007a5b20, 0x7000e970f0, 0x1> as *const (), 0), // +0x7de3c0 sub_7de3c0
    Lifted::new(0x7000eca480, fn_7000eca480 as *const (), 0), // +0x7de480 sub_7de480
    Lifted::new(0x7000eca550, fn_7000eca550 as *const (), 0), // +0x7de550 sub_7de550
    Lifted::new(0x7000eca620, tpl_1597::<0xf0, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1> as *const (), 0), // +0x7de620 sub_7de620
    Lifted::new(0x7000eca6e0, tpl_1596::<0xf0, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1> as *const (), 0), // +0x7de6e0 sub_7de6e0
    Lifted::new(0x7000eca7b0, tpl_1596::<0x270, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1> as *const (), 0), // +0x7de7b0 sub_7de7b0
    Lifted::new(0x7000eca880, tpl_1597::<0xf0, 0x3, 0x70007a5b20, 0x7000e970f0, 0x1> as *const (), 0), // +0x7de880 sub_7de880
    Lifted::new(0x7000eca940, fn_7000eca940 as *const (), 0), // +0x7de940 sub_7de940
    Lifted::new(0x7000ecaa10, fn_7000ecaa10 as *const (), 0), // +0x7dea10 sub_7dea10
    Lifted::new(0x7000ecaae0, fn_7000ecaae0 as *const (), 0), // +0x7deae0 sub_7deae0
    Lifted::new(0x7000ecabb0, fn_7000ecabb0 as *const (), 0), // +0x7debb0 sub_7debb0
    Lifted::new(0x7000ecac80, fn_7000ecac80 as *const (), 0), // +0x7dec80 sub_7dec80
    Lifted::new(0x7000ecad50, tpl_1598::<0x3, 0x700076dfa0, 0x7000e970f0, 0x1> as *const (), 0), // +0x7ded50 sub_7ded50
    Lifted::new(0x7000ecae10, fn_7000ecae10 as *const (), 0), // +0x7dee10 sub_7dee10
    Lifted::new(0x7000ecaee0, fn_7000ecaee0 as *const (), 0), // +0x7deee0 sub_7deee0
    Lifted::new(0x7000ecafb0, tpl_1598::<0x3, 0x700076dfa0, 0x7000e970f0, 0x1> as *const (), 0), // +0x7defb0 sub_7defb0
    Lifted::new(0x7000ecb070, fn_7000ecb070 as *const (), 0), // +0x7df070 sub_7df070
    Lifted::new(0x7000ecb370, fn_7000ecb370 as *const (), 0), // +0x7df370 sub_7df370
    Lifted::new(0x7000ecb6c0, fn_7000ecb6c0 as *const (), 0), // +0x7df6c0 sub_7df6c0
    Lifted::new(0x7000ecb700, fn_7000ecb700 as *const (), 0), // +0x7df700 sub_7df700
    Lifted::new(0x7000ecb8c0, fn_7000ecb8c0 as *const (), 0), // +0x7df8c0 sub_7df8c0
    Lifted::new(0x7000ecb9c0, fn_7000ecb9c0 as *const (), 0), // +0x7df9c0 sub_7df9c0
    Lifted::new(0x7000ecbc70, tpl_1228::<0x7000ee1ad0, 0x7000ecbc79> as *const (), 0), // +0x7dfc70 sub_7dfc70
    Lifted::new(0x7000ecbc80, tpl_1228::<0x7000ee1af0, 0x7000ecbc89> as *const (), 0), // +0x7dfc80 sub_7dfc80
    Lifted::new(0x7000ecbc90, tpl_1228::<0x7000ee1b10, 0x7000ecbc99> as *const (), 0), // +0x7dfc90 sub_7dfc90
    Lifted::new(0x7000ecbca0, tpl_1228::<0x7000ee1b30, 0x7000ecbca9> as *const (), 0), // +0x7dfca0 sub_7dfca0
    Lifted::new(0x7000ecbcb0, tpl_1228::<0x7000ee1b50, 0x7000ecbcb9> as *const (), 0), // +0x7dfcb0 sub_7dfcb0
    Lifted::new(0x7000ecbcc0, tpl_1228::<0x7000ee1b70, 0x7000ecbcc9> as *const (), 0), // +0x7dfcc0 sub_7dfcc0
    Lifted::new(0x7000ecbcf0, tpl_14 as *const (), 1), // +0x7dfcf0 sub_7dfcf0
    Lifted::new(0x7000ecbd00, tpl_70 as *const (), 1), // +0x7dfd00 sub_7dfd00
    Lifted::new(0x7000ecbd10, tpl_14 as *const (), 1), // +0x7dfd10 sub_7dfd10
    Lifted::new(0x7000ecbd20, tpl_70 as *const (), 1), // +0x7dfd20 sub_7dfd20
    Lifted::new(0x7000ecbd30, tpl_14 as *const (), 1), // +0x7dfd30 sub_7dfd30
    Lifted::new(0x7000ecbd40, tpl_70 as *const (), 1), // +0x7dfd40 sub_7dfd40
    Lifted::new(0x7000ecbd50, tpl_14 as *const (), 1), // +0x7dfd50 sub_7dfd50
    Lifted::new(0x7000ecbd60, tpl_70 as *const (), 1), // +0x7dfd60 sub_7dfd60
    Lifted::new(0x7000ecbd70, tpl_14 as *const (), 1), // +0x7dfd70 sub_7dfd70
    Lifted::new(0x7000ecbd80, tpl_70 as *const (), 1), // +0x7dfd80 sub_7dfd80
    Lifted::new(0x7000ecbd90, tpl_14 as *const (), 1), // +0x7dfd90 sub_7dfd90
    Lifted::new(0x7000ecbda0, tpl_70 as *const (), 1), // +0x7dfda0 sub_7dfda0
    Lifted::new(0x7000ecbdb0, fn_7000ecbdb0 as *const (), 1), // +0x7dfdb0 sub_7dfdb0
    Lifted::new(0x7000ecbdd0, tpl_28 as *const (), 1), // +0x7dfdd0 sub_7dfdd0
    Lifted::new(0x7000ecbde0, tpl_28 as *const (), 1), // +0x7dfde0 sub_7dfde0
    Lifted::new(0x7000ecbdf0, tpl_28 as *const (), 1), // +0x7dfdf0 sub_7dfdf0
    Lifted::new(0x7000ecbe00, tpl_28 as *const (), 1), // +0x7dfe00 sub_7dfe00
    Lifted::new(0x7000ecbe10, tpl_28 as *const (), 1), // +0x7dfe10 sub_7dfe10
    Lifted::new(0x7000ecbe20, tpl_28 as *const (), 1), // +0x7dfe20 sub_7dfe20
    Lifted::new(0x7000ecbe30, tpl_28 as *const (), 1), // +0x7dfe30 sub_7dfe30
    Lifted::new(0x7000ecbe40, tpl_28 as *const (), 1), // +0x7dfe40 sub_7dfe40
    Lifted::new(0x7000ecbe50, tpl_28 as *const (), 1), // +0x7dfe50 sub_7dfe50
    Lifted::new(0x7000ecbe60, tpl_28 as *const (), 1), // +0x7dfe60 sub_7dfe60
    Lifted::new(0x7000ecbe70, tpl_28 as *const (), 1), // +0x7dfe70 sub_7dfe70
    Lifted::new(0x7000ecbe80, tpl_28 as *const (), 1), // +0x7dfe80 sub_7dfe80
    Lifted::new(0x7000ecbe90, fn_7000ecbe90 as *const (), 0), // +0x7dfe90 sub_7dfe90
    Lifted::new(0x7000ecbf50, fn_7000ecbf50 as *const (), 0), // +0x7dff50 sub_7dff50
    Lifted::new(0x7000ecbf80, fn_7000ecbf80 as *const (), 0), // +0x7dff80 sub_7dff80
    Lifted::new(0x7000ecd5e0, tpl_1599::<0x70017d2c10, 0x7001d54ef8, 0x7000ecd604, 0x7001857ec0, 0x7000a62410, 0x7001857eb0, 0x7001857ea0, 0x7001857ed0> as *const (), 0), // +0x7e15e0 sub_7e15e0
    Lifted::new(0x7000ecd660, tpl_1600::<0x70007292b0, 0x7000717350, 0x7000e970f0> as *const (), 0), // +0x7e1660 sub_7e1660
    Lifted::new(0x7000ecd750, tpl_1600::<0x700072e970, 0x70007a6010, 0x7000e970f0> as *const (), 0), // +0x7e1750 sub_7e1750
    Lifted::new(0x7000ecd840, fn_7000ecd840 as *const (), 2), // +0x7e1840 sub_7e1840
    Lifted::new(0x7000ecda50, fn_7000ecda50 as *const (), 0), // +0x7e1a50 sub_7e1a50
    Lifted::new(0x7000ecdb90, fn_7000ecdb90 as *const (), 2), // +0x7e1b90 sub_7e1b90
    Lifted::new(0x7000ecdd50, fn_7000ecdd50 as *const (), 2), // +0x7e1d50 sub_7e1d50
    Lifted::new(0x7000ece1a0, fn_7000ece1a0 as *const (), 0), // +0x7e21a0 sub_7e21a0
    Lifted::new(0x7000ece470, fn_7000ece470 as *const (), 2), // +0x7e2470 sub_7e2470
    Lifted::new(0x7000ece890, fn_7000ece890 as *const (), 0), // +0x7e2890 sub_7e2890
    Lifted::new(0x7000eceb10, fn_7000eceb10 as *const (), 0), // +0x7e2b10 sub_7e2b10
    Lifted::new(0x7000ecebb0, fn_7000ecebb0 as *const (), 0), // +0x7e2bb0 sub_7e2bb0
    Lifted::new(0x7000ecf190, fn_7000ecf190 as *const (), 0), // +0x7e3190 sub_7e3190
    Lifted::new(0x7000ecf210, fn_7000ecf210 as *const (), 0), // +0x7e3210 sub_7e3210
    Lifted::new(0x7000ecf290, fn_7000ecf290 as *const (), 2), // +0x7e3290 sub_7e3290
    Lifted::new(0x7000ecf5a0, fn_7000ecf5a0 as *const (), 2), // +0x7e35a0 sub_7e35a0
    Lifted::new(0x7000ecf790, fn_7000ecf790 as *const (), 2), // +0x7e3790 sub_7e3790
    Lifted::new(0x7000ecfad0, fn_7000ecfad0 as *const (), 2), // +0x7e3ad0 sub_7e3ad0
    Lifted::new(0x7000ecfe00, tpl_148 as *const (), 1), // +0x7e3e00 sub_7e3e00
    Lifted::new(0x7000ecfe10, fn_7000ecfe10 as *const (), 1), // +0x7e3e10 sub_7e3e10
    Lifted::new(0x7000ecfe30, fn_7000ecfe30 as *const (), 0), // +0x7e3e30 sub_7e3e30
    Lifted::new(0x7000ed0060, fn_7000ed0060 as *const (), 2), // +0x7e4060 sub_7e4060
    Lifted::new(0x7000ed0710, fn_7000ed0710 as *const (), 0), // +0x7e4710 sub_7e4710
    Lifted::new(0x7000ed0b50, fn_7000ed0b50 as *const (), 2), // +0x7e4b50 sub_7e4b50
    Lifted::new(0x7000ed0c70, fn_7000ed0c70 as *const (), 1), // +0x7e4c70 sub_7e4c70
    Lifted::new(0x7000ed0cd0, fn_7000ed0cd0 as *const (), 0), // +0x7e4cd0 sub_7e4cd0
    Lifted::new(0x7000ed0e90, fn_7000ed0e90 as *const (), 0), // +0x7e4e90 sub_7e4e90
    Lifted::new(0x7000ed1070, fn_7000ed1070 as *const (), 2), // +0x7e5070 sub_7e5070
    Lifted::new(0x7000ed12f0, fn_7000ed12f0 as *const (), 2), // +0x7e52f0 sub_7e52f0
    Lifted::new(0x7000ed1550, fn_7000ed1550 as *const (), 2), // +0x7e5550 sub_7e5550
    Lifted::new(0x7000ed16b0, fn_7000ed16b0 as *const (), 0), // +0x7e56b0 sub_7e56b0
    Lifted::new(0x7000ed1710, fn_7000ed1710 as *const (), 0), // +0x7e5710 sub_7e5710
    Lifted::new(0x7000ed1780, fn_7000ed1780 as *const (), 0), // +0x7e5780 sub_7e5780
    Lifted::new(0x7000ed1960, fn_7000ed1960 as *const (), 0), // +0x7e5960 sub_7e5960
    Lifted::new(0x7000ed19f0, fn_7000ed19f0 as *const (), 2), // +0x7e59f0 sub_7e59f0
    Lifted::new(0x7000ed1db0, fn_7000ed1db0 as *const (), 0), // +0x7e5db0 sub_7e5db0
    Lifted::new(0x7000ed1ee0, fn_7000ed1ee0 as *const (), 0), // +0x7e5ee0 sub_7e5ee0
    Lifted::new(0x7000ed1fb0, fn_7000ed1fb0 as *const (), 0), // +0x7e5fb0 sub_7e5fb0
    Lifted::new(0x7000ed2110, fn_7000ed2110 as *const (), 0), // +0x7e6110 sub_7e6110
    Lifted::new(0x7000ed2200, fn_7000ed2200 as *const (), 2), // +0x7e6200 sub_7e6200
    Lifted::new(0x7000ed2360, fn_7000ed2360 as *const (), 2), // +0x7e6360 sub_7e6360
    Lifted::new(0x7000ed25b0, fn_7000ed25b0 as *const (), 2), // +0x7e65b0 sub_7e65b0
    Lifted::new(0x7000ed2710, tpl_1601::<0x70006f2a70, 0x70006f9a60, 0x700071dde0, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x7e6710 sub_7e6710
    Lifted::new(0x7000ed2860, tpl_1601::<0x70006f2a70, 0x70006f9a60, 0x70007432e0, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x7e6860 sub_7e6860
    Lifted::new(0x7000ed29b0, fn_7000ed29b0 as *const (), 0), // +0x7e69b0 sub_7e69b0
    Lifted::new(0x7000ed2a20, fn_7000ed2a20 as *const (), 0), // +0x7e6a20 sub_7e6a20
    Lifted::new(0x7000ed2b30, fn_7000ed2b30 as *const (), 0), // +0x7e6b30 sub_7e6b30
    Lifted::new(0x7000ed2c10, fn_7000ed2c10 as *const (), 1), // +0x7e6c10 sub_7e6c10
    Lifted::new(0x7000ed2c60, fn_7000ed2c60 as *const (), 2), // +0x7e6c60 sub_7e6c60
    Lifted::new(0x7000ed3010, fn_7000ed3010 as *const (), 2), // +0x7e7010 sub_7e7010
    Lifted::new(0x7000ed31c0, fn_7000ed31c0 as *const (), 2), // +0x7e71c0 sub_7e71c0
    Lifted::new(0x7000ed35e0, fn_7000ed35e0 as *const (), 0), // +0x7e75e0 sub_7e75e0
    Lifted::new(0x7000ed3650, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e7650 sub_7e7650
    Lifted::new(0x7000ed36b0, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e76b0 sub_7e76b0
    Lifted::new(0x7000ed3710, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e7710 sub_7e7710
    Lifted::new(0x7000ed3770, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e7770 sub_7e7770
    Lifted::new(0x7000ed37d0, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e77d0 sub_7e77d0
    Lifted::new(0x7000ed3830, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e7830 sub_7e7830
    Lifted::new(0x7000ed3890, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e7890 sub_7e7890
    Lifted::new(0x7000ed38f0, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e78f0 sub_7e78f0
    Lifted::new(0x7000ed3950, tpl_1602::<0x7001d55098> as *const (), 0), // +0x7e7950 sub_7e7950
    Lifted::new(0x7000ed39b0, fn_7000ed39b0 as *const (), 0), // +0x7e79b0 sub_7e79b0
    Lifted::new(0x7000ed3a80, fn_7000ed3a80 as *const (), 0), // +0x7e7a80 sub_7e7a80
    Lifted::new(0x7000ed3bd0, fn_7000ed3bd0 as *const (), 2), // +0x7e7bd0 sub_7e7bd0
    Lifted::new(0x7000ed3e10, fn_7000ed3e10 as *const (), 2), // +0x7e7e10 sub_7e7e10
    Lifted::new(0x7000ed3ee0, fn_7000ed3ee0 as *const (), 2), // +0x7e7ee0 sub_7e7ee0
    Lifted::new(0x7000ed3fb0, fn_7000ed3fb0 as *const (), 2), // +0x7e7fb0 sub_7e7fb0
    Lifted::new(0x7000ed41c0, fn_7000ed41c0 as *const (), 0), // +0x7e81c0 sub_7e81c0
    Lifted::new(0x7000ed42f0, fn_7000ed42f0 as *const (), 0), // +0x7e82f0 sub_7e82f0
    Lifted::new(0x7000ed4580, fn_7000ed4580 as *const (), 2), // +0x7e8580 sub_7e8580
    Lifted::new(0x7000ed47f0, fn_7000ed47f0 as *const (), 0), // +0x7e87f0 sub_7e87f0
    Lifted::new(0x7000ed4aa0, fn_7000ed4aa0 as *const (), 2), // +0x7e8aa0 sub_7e8aa0
    Lifted::new(0x7000ed4c00, fn_7000ed4c00 as *const (), 2), // +0x7e8c00 sub_7e8c00
    Lifted::new(0x7000ed4dd0, fn_7000ed4dd0 as *const (), 2), // +0x7e8dd0 sub_7e8dd0
    Lifted::new(0x7000ed50b0, fn_7000ed50b0 as *const (), 0), // +0x7e90b0 sub_7e90b0
    Lifted::new(0x7000ed5110, fn_7000ed5110 as *const (), 2), // +0x7e9110 sub_7e9110
    Lifted::new(0x7000ed52f0, fn_7000ed52f0 as *const (), 0), // +0x7e92f0 sub_7e92f0
    Lifted::new(0x7000ed5320, fn_7000ed5320 as *const (), 2), // +0x7e9320 sub_7e9320
    Lifted::new(0x7000ed56c0, fn_7000ed56c0 as *const (), 2), // +0x7e96c0 sub_7e96c0
    Lifted::new(0x7000ed5970, fn_7000ed5970 as *const (), 1), // +0x7e9970 sub_7e9970
    Lifted::new(0x7000ed59d0, fn_7000ed59d0 as *const (), 2), // +0x7e99d0 sub_7e99d0
    Lifted::new(0x7000ed6030, fn_7000ed6030 as *const (), 2), // +0x7ea030 sub_7ea030
    Lifted::new(0x7000ed6320, fn_7000ed6320 as *const (), 2), // +0x7ea320 sub_7ea320
    Lifted::new(0x7000ed6740, fn_7000ed6740 as *const (), 2), // +0x7ea740 sub_7ea740
    Lifted::new(0x7000ed6d60, fn_7000ed6d60 as *const (), 0), // +0x7ead60 sub_7ead60
    Lifted::new(0x7000ed6f10, fn_7000ed6f10 as *const (), 0), // +0x7eaf10 sub_7eaf10
    Lifted::new(0x7000ed70b0, tpl_1603::<0x7000737f00, 0x7, 0x7000e970f0, 0xb, 0x70007676c0> as *const (), 0), // +0x7eb0b0 sub_7eb0b0
    Lifted::new(0x7000ed7170, tpl_1603::<0x7000707350, 0x7, 0x7000e970f0, 0xb, 0x7000724470> as *const (), 0), // +0x7eb170 sub_7eb170
    Lifted::new(0x7000ed7230, tpl_1603::<0x700070d8f0, 0x7, 0x7000e970f0, 0xb, 0x700079a0a0> as *const (), 0), // +0x7eb230 sub_7eb230
    Lifted::new(0x7000ed72f0, fn_7000ed72f0 as *const (), 2), // +0x7eb2f0 sub_7eb2f0
    Lifted::new(0x7000ed7730, tpl_1321::<0x1f> as *const (), 1), // +0x7eb730 sub_7eb730
    Lifted::new(0x7000ed7750, fn_7000ed7750 as *const (), 0), // +0x7eb750 sub_7eb750
    Lifted::new(0x7000ed7770, fn_7000ed7770 as *const (), 0), // +0x7eb770 sub_7eb770
    Lifted::new(0x7000ed77b0, fn_7000ed77b0 as *const (), 0), // +0x7eb7b0 sub_7eb7b0
    Lifted::new(0x7000ed77c0, fn_7000ed77c0 as *const (), 0), // +0x7eb7c0 sub_7eb7c0
    Lifted::new(0x7000ed7860, fn_7000ed7860 as *const (), 0), // +0x7eb860 sub_7eb860
    Lifted::new(0x7000ed7960, fn_7000ed7960 as *const (), 0), // +0x7eb960 sub_7eb960
    Lifted::new(0x7000ed79f0, fn_7000ed79f0 as *const (), 0), // +0x7eb9f0 sub_7eb9f0
    Lifted::new(0x7000ed7a90, tpl_1604::<0x700099ba10, 0x70006fc8f0, 0xf> as *const (), 0), // +0x7eba90 sub_7eba90
    Lifted::new(0x7000ed7b70, fn_7000ed7b70 as *const (), 1), // +0x7ebb70 sub_7ebb70
    Lifted::new(0x7000ed7c00, fn_7000ed7c00 as *const (), 1), // +0x7ebc00 sub_7ebc00
    Lifted::new(0x7000ed7c70, tpl_1605::<0x430, 0x3, 0x7, 0xf> as *const (), 3), // +0x7ebc70 sub_7ebc70
    Lifted::new(0x7000ed7d70, tpl_1604::<0x700099ba10, 0x70006fc8f0, 0xf> as *const (), 0), // +0x7ebd70 sub_7ebd70
    Lifted::new(0x7000ed7e50, tpl_1604::<0x700099ba10, 0x70006fc8f0, 0xf> as *const (), 0), // +0x7ebe50 sub_7ebe50
    Lifted::new(0x7000ed7f30, tpl_1605::<0x430, 0x3, 0x7, 0xf> as *const (), 3), // +0x7ebf30 sub_7ebf30
    Lifted::new(0x7000ed8030, fn_7000ed8030 as *const (), 0), // +0x7ec030 sub_7ec030
    Lifted::new(0x7000ed8100, fn_7000ed8100 as *const (), 1), // +0x7ec100 sub_7ec100
    Lifted::new(0x7000ed8110, fn_7000ed8110 as *const (), 1), // +0x7ec110 sub_7ec110
    Lifted::new(0x7000ed8120, tpl_173::<0x3> as *const (), 1), // +0x7ec120 sub_7ec120
    Lifted::new(0x7000ed8130, tpl_160::<0x3> as *const (), 1), // +0x7ec130 sub_7ec130
    Lifted::new(0x7000ed8140, fn_7000ed8140 as *const (), 2), // +0x7ec140 sub_7ec140
    Lifted::new(0x7000ed8a70, tpl_1606::<0x3, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffffffff> as *const (), 1), // +0x7eca70 sub_7eca70
    Lifted::new(0x7000ed8af0, tpl_1606::<0x3, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffffffff> as *const (), 1), // +0x7ecaf0 sub_7ecaf0
    Lifted::new(0x7000ed8b70, fn_7000ed8b70 as *const (), 0), // +0x7ecb70 sub_7ecb70
    Lifted::new(0x7000ed8c60, fn_7000ed8c60 as *const (), 3), // +0x7ecc60 sub_7ecc60
    Lifted::new(0x7000ed8d50, fn_7000ed8d50 as *const (), 3), // +0x7ecd50 sub_7ecd50
    Lifted::new(0x7000ed8e40, fn_7000ed8e40 as *const (), 0), // +0x7ece40 sub_7ece40
    Lifted::new(0x7000ed8f80, tpl_1607::<0x38, 0xf, 0x13, 0xa0, 0xa8> as *const (), 1), // +0x7ecf80 sub_7ecf80
    Lifted::new(0x7000ed9010, tpl_1608::<0xf, 0x13, 0x20, 0x28> as *const (), 1), // +0x7ed010 sub_7ed010
    Lifted::new(0x7000ed9080, tpl_1609::<0x38, 0xf, 0x13, 0xa0, 0xa8> as *const (), 1), // +0x7ed080 sub_7ed080
    Lifted::new(0x7000ed9120, tpl_1610::<0xf, 0x13, 0x20, 0x28> as *const (), 1), // +0x7ed120 sub_7ed120
    Lifted::new(0x7000ed91a0, tpl_1611::<0x38, 0xf, 0x1c, 0xffffffffffffffff, 0xa0, 0xa8> as *const (), 1), // +0x7ed1a0 sub_7ed1a0
    Lifted::new(0x7000ed9240, tpl_1612::<0xf, 0x1c, 0xffffffffffffffff, 0x20, 0x28> as *const (), 1), // +0x7ed240 sub_7ed240
    Lifted::new(0x7000ed92c0, tpl_1607::<0x38, 0xf, 0x1b, 0xa0, 0xa8> as *const (), 1), // +0x7ed2c0 sub_7ed2c0
    Lifted::new(0x7000ed9350, tpl_1608::<0xf, 0x1b, 0x20, 0x28> as *const (), 1), // +0x7ed350 sub_7ed350
    Lifted::new(0x7000ed93c0, fn_7000ed93c0 as *const (), 0), // +0x7ed3c0 sub_7ed3c0
    Lifted::new(0x7000ed9400, fn_7000ed9400 as *const (), 1), // +0x7ed400 sub_7ed400
    Lifted::new(0x7000ed9490, tpl_1611::<0x38, 0xf, 0x14, 0xffffffffffffffff, 0xa0, 0xa8> as *const (), 1), // +0x7ed490 sub_7ed490
    Lifted::new(0x7000ed9530, tpl_1612::<0xf, 0x14, 0xffffffffffffffff, 0x20, 0x28> as *const (), 1), // +0x7ed530 sub_7ed530
    Lifted::new(0x7000ed95b0, tpl_1609::<0x38, 0xf, 0x13, 0xa0, 0xa8> as *const (), 1), // +0x7ed5b0 sub_7ed5b0
    Lifted::new(0x7000ed9650, tpl_1610::<0xf, 0x13, 0x20, 0x28> as *const (), 1), // +0x7ed650 sub_7ed650
    Lifted::new(0x7000ed96d0, tpl_1609::<0x38, 0xf, 0x13, 0xa0, 0xa8> as *const (), 1), // +0x7ed6d0 sub_7ed6d0
    Lifted::new(0x7000ed9770, tpl_1610::<0xf, 0x13, 0x20, 0x28> as *const (), 1), // +0x7ed770 sub_7ed770
    Lifted::new(0x7000ed97f0, tpl_1611::<0x38, 0xf, 0x14, 0xffffffffffffffff, 0xa0, 0xa8> as *const (), 1), // +0x7ed7f0 sub_7ed7f0
    Lifted::new(0x7000ed9890, tpl_1612::<0xf, 0x14, 0xffffffffffffffff, 0x20, 0x28> as *const (), 1), // +0x7ed890 sub_7ed890
    Lifted::new(0x7000ed9910, fn_7000ed9910 as *const (), 2), // +0x7ed910 sub_7ed910
    Lifted::new(0x7000eda0e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000eda0f1> as *const (), 0), // +0x7ee0e0 sub_7ee0e0
    Lifted::new(0x7000eda100, fn_7000eda100 as *const (), 2), // +0x7ee100 sub_7ee100
    Lifted::new(0x7000eda320, tpl_1046::<0x13, 0xffffffffffffffff, 0x1, 0x3, 0x8> as *const (), 1), // +0x7ee320 sub_7ee320
    Lifted::new(0x7000eda390, fn_7000eda390 as *const (), 1), // +0x7ee390 sub_7ee390
    Lifted::new(0x7000eda440, fn_7000eda440 as *const (), 2), // +0x7ee440 sub_7ee440
    Lifted::new(0x7000eda620, fn_7000eda620 as *const (), 1), // +0x7ee620 sub_7ee620
    Lifted::new(0x7000eda7f0, fn_7000eda7f0 as *const (), 1), // +0x7ee7f0 sub_7ee7f0
    Lifted::new(0x7000eda810, fn_7000eda810 as *const (), 2), // +0x7ee810 sub_7ee810
    Lifted::new(0x7000eda850, fn_7000eda850 as *const (), 2), // +0x7ee850 sub_7ee850
    Lifted::new(0x7000eda8e0, fn_7000eda8e0 as *const (), 0), // +0x7ee8e0 sub_7ee8e0
    Lifted::new(0x7000edab70, fn_7000edab70 as *const (), 2), // +0x7eeb70 sub_7eeb70
    Lifted::new(0x7000edaf60, fn_7000edaf60 as *const (), 0), // +0x7eef60 sub_7eef60
    Lifted::new(0x7000edb210, fn_7000edb210 as *const (), 0), // +0x7ef210 sub_7ef210
    Lifted::new(0x7000edb380, fn_7000edb380 as *const (), 2), // +0x7ef380 sub_7ef380
    Lifted::new(0x7000edb3c0, fn_7000edb3c0 as *const (), 2), // +0x7ef3c0 sub_7ef3c0
    Lifted::new(0x7000edb450, tpl_188 as *const (), 1), // +0x7ef450 sub_7ef450
    Lifted::new(0x7000edb460, tpl_188 as *const (), 1), // +0x7ef460 sub_7ef460
    Lifted::new(0x7000edb470, tpl_188 as *const (), 1), // +0x7ef470 sub_7ef470
    Lifted::new(0x7000edb480, tpl_188 as *const (), 1), // +0x7ef480 sub_7ef480
    Lifted::new(0x7000edb490, fn_7000edb490 as *const (), 0), // +0x7ef490 sub_7ef490
    Lifted::new(0x7000edb4b0, fn_7000edb4b0 as *const (), 0), // +0x7ef4b0 sub_7ef4b0
    Lifted::new(0x7000edb640, fn_7000edb640 as *const (), 1), // +0x7ef640 sub_7ef640
    Lifted::new(0x7000edb6c0, fn_7000edb6c0 as *const (), 0), // +0x7ef6c0 sub_7ef6c0
    Lifted::new(0x7000edb760, fn_7000edb760 as *const (), 2), // +0x7ef760 sub_7ef760
    Lifted::new(0x7000edbcc0, fn_7000edbcc0 as *const (), 0), // +0x7efcc0 sub_7efcc0
    Lifted::new(0x7000edbfc0, fn_7000edbfc0 as *const (), 0), // +0x7effc0 sub_7effc0
    Lifted::new(0x7000edc060, fn_7000edc060 as *const (), 0), // +0x7f0060 sub_7f0060
    Lifted::new(0x7000edc270, fn_7000edc270 as *const (), 0), // +0x7f0270 sub_7f0270
    Lifted::new(0x7000edc640, fn_7000edc640 as *const (), 0), // +0x7f0640 sub_7f0640
    Lifted::new(0x7000edc9a0, fn_7000edc9a0 as *const (), 2), // +0x7f09a0 sub_7f09a0
    Lifted::new(0x7000edcad0, fn_7000edcad0 as *const (), 2), // +0x7f0ad0 sub_7f0ad0
    Lifted::new(0x7000edd3b0, fn_7000edd3b0 as *const (), 0), // +0x7f13b0 sub_7f13b0
    Lifted::new(0x7000edd460, fn_7000edd460 as *const (), 0), // +0x7f1460 sub_7f1460
    Lifted::new(0x7000edd890, fn_7000edd890 as *const (), 0), // +0x7f1890 sub_7f1890
    Lifted::new(0x7000edd950, fn_7000edd950 as *const (), 2), // +0x7f1950 sub_7f1950
    Lifted::new(0x7000eddb20, fn_7000eddb20 as *const (), 0), // +0x7f1b20 sub_7f1b20
    Lifted::new(0x7000eddbb0, fn_7000eddbb0 as *const (), 0), // +0x7f1bb0 sub_7f1bb0
    Lifted::new(0x7000eddca0, fn_7000eddca0 as *const (), 2), // +0x7f1ca0 sub_7f1ca0
    Lifted::new(0x7000ede100, fn_7000ede100 as *const (), 0), // +0x7f2100 sub_7f2100
    Lifted::new(0x7000ede1f0, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f21f0 sub_7f21f0
    Lifted::new(0x7000ede250, tpl_1614 as *const (), 1), // +0x7f2250 sub_7f2250
    Lifted::new(0x7000ede290, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f2290 sub_7f2290
    Lifted::new(0x7000ede2f0, tpl_1614 as *const (), 1), // +0x7f22f0 sub_7f22f0
    Lifted::new(0x7000ede330, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f2330 sub_7f2330
    Lifted::new(0x7000ede390, tpl_1614 as *const (), 1), // +0x7f2390 sub_7f2390
    Lifted::new(0x7000ede3d0, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f23d0 sub_7f23d0
    Lifted::new(0x7000ede430, tpl_1614 as *const (), 1), // +0x7f2430 sub_7f2430
    Lifted::new(0x7000ede470, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f2470 sub_7f2470
    Lifted::new(0x7000ede4d0, tpl_1614 as *const (), 1), // +0x7f24d0 sub_7f24d0
    Lifted::new(0x7000ede510, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f2510 sub_7f2510
    Lifted::new(0x7000ede570, tpl_1614 as *const (), 1), // +0x7f2570 sub_7f2570
    Lifted::new(0x7000ede5b0, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f25b0 sub_7f25b0
    Lifted::new(0x7000ede610, tpl_1614 as *const (), 1), // +0x7f2610 sub_7f2610
    Lifted::new(0x7000ede650, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f2650 sub_7f2650
    Lifted::new(0x7000ede6b0, tpl_1614 as *const (), 1), // +0x7f26b0 sub_7f26b0
    Lifted::new(0x7000ede6f0, tpl_1613::<0xb, 0x7, 0xf> as *const (), 1), // +0x7f26f0 sub_7f26f0
    Lifted::new(0x7000ede750, tpl_1614 as *const (), 1), // +0x7f2750 sub_7f2750
    Lifted::new(0x7000ede790, fn_7000ede790 as *const (), 2), // +0x7f2790 sub_7f2790
    Lifted::new(0x7000edea00, fn_7000edea00 as *const (), 0), // +0x7f2a00 sub_7f2a00
    Lifted::new(0x7000ee0180, fn_7000ee0180 as *const (), 1), // +0x7f4180 sub_7f4180
    Lifted::new(0x7000ee01a0, fn_7000ee01a0 as *const (), 1), // +0x7f41a0 sub_7f41a0
    Lifted::new(0x7000ee01c0, tpl_203::<0xfffffffffffffffd> as *const (), 1), // +0x7f41c0 sub_7f41c0
    Lifted::new(0x7000ee01e0, tpl_204::<0xfffffffffffffffd> as *const (), 1), // +0x7f41e0 sub_7f41e0
    Lifted::new(0x7000ee01f0, fn_7000ee01f0 as *const (), 1), // +0x7f41f0 sub_7f41f0
    Lifted::new(0x7000ee0210, fn_7000ee0210 as *const (), 1), // +0x7f4210 sub_7f4210
    Lifted::new(0x7000ee0280, fn_7000ee0280 as *const (), 0), // +0x7f4280 sub_7f4280
    Lifted::new(0x7000ee02c0, tpl_1615::<0x70006fe540, 0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x7f42c0 sub_7f42c0
    Lifted::new(0x7000ee03a0, tpl_1615::<0x7000722c90, 0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x7f43a0 sub_7f43a0
    Lifted::new(0x7000ee0480, fn_7000ee0480 as *const (), 0), // +0x7f4480 sub_7f4480
    Lifted::new(0x7000ee0600, fn_7000ee0600 as *const (), 2), // +0x7f4600 sub_7f4600
    Lifted::new(0x7000ee09c0, fn_7000ee09c0 as *const (), 2), // +0x7f49c0 sub_7f49c0
    Lifted::new(0x7000ee0be0, fn_7000ee0be0 as *const (), 0), // +0x7f4be0 sub_7f4be0
    Lifted::new(0x7000ee0dd0, fn_7000ee0dd0 as *const (), 0), // +0x7f4dd0 sub_7f4dd0
    Lifted::new(0x7000ee1010, fn_7000ee1010 as *const (), 0), // +0x7f5010 sub_7f5010
    Lifted::new(0x7000ee1070, fn_7000ee1070 as *const (), 0), // +0x7f5070 sub_7f5070
    Lifted::new(0x7000ee12c0, fn_7000ee12c0 as *const (), 0), // +0x7f52c0 sub_7f52c0
    Lifted::new(0x7000ee1330, fn_7000ee1330 as *const (), 0), // +0x7f5330 sub_7f5330
    Lifted::new(0x7000ee13d0, fn_7000ee13d0 as *const (), 1), // +0x7f53d0 sub_7f53d0
    Lifted::new(0x7000ee1430, fn_7000ee1430 as *const (), 1), // +0x7f5430 sub_7f5430
    Lifted::new(0x7000ee14d0, fn_7000ee14d0 as *const (), 0), // +0x7f54d0 sub_7f54d0
    Lifted::new(0x7000ee18b0, fn_7000ee18b0 as *const (), 0), // +0x7f58b0 sub_7f58b0
    Lifted::new(0x7000ee1910, fn_7000ee1910 as *const (), 1), // +0x7f5910 sub_7f5910
    Lifted::new(0x7000ee1930, tpl_209::<0x20, 0x28> as *const (), 1), // +0x7f5930 sub_7f5930
    Lifted::new(0x7000ee1950, tpl_209::<0x20, 0x28> as *const (), 1), // +0x7f5950 sub_7f5950
    Lifted::new(0x7000ee1970, tpl_209::<0x20, 0x28> as *const (), 1), // +0x7f5970 sub_7f5970
    Lifted::new(0x7000ee1990, tpl_209::<0x20, 0x28> as *const (), 1), // +0x7f5990 sub_7f5990
    Lifted::new(0x7000ee19b0, fn_7000ee19b0 as *const (), 1), // +0x7f59b0 sub_7f59b0
    Lifted::new(0x7000ee1a30, fn_7000ee1a30 as *const (), 1), // +0x7f5a30 sub_7f5a30
    Lifted::new(0x7000ee1a50, fn_7000ee1a50 as *const (), 0), // +0x7f5a50 sub_7f5a50
    Lifted::new(0x7000ee1ad0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ee1ae1> as *const (), 0), // +0x7f5ad0 sub_7f5ad0
    Lifted::new(0x7000ee1af0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ee1b01> as *const (), 0), // +0x7f5af0 sub_7f5af0
    Lifted::new(0x7000ee1b10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ee1b21> as *const (), 0), // +0x7f5b10 sub_7f5b10
    Lifted::new(0x7000ee1b30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ee1b41> as *const (), 0), // +0x7f5b30 sub_7f5b30
    Lifted::new(0x7000ee1b50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ee1b61> as *const (), 0), // +0x7f5b50 sub_7f5b50
    Lifted::new(0x7000ee1b70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000ee1b81> as *const (), 0), // +0x7f5b70 sub_7f5b70
    Lifted::new(0x7000ee1b90, fn_7000ee1b90 as *const (), 0), // +0x7f5b90 sub_7f5b90
    Lifted::new(0x7000ee1bc0, fn_7000ee1bc0 as *const (), 0), // +0x7f5bc0 sub_7f5bc0
    Lifted::new(0x7000ee1c50, fn_7000ee1c50 as *const (), 0), // +0x7f5c50 sub_7f5c50
    Lifted::new(0x7000ee1cd0, fn_7000ee1cd0 as *const (), 0), // +0x7f5cd0 sub_7f5cd0
    Lifted::new(0x7000ee1d20, fn_7000ee1d20 as *const (), 0), // +0x7f5d20 sub_7f5d20
    Lifted::new(0x7000ee1d70, fn_7000ee1d70 as *const (), 0), // +0x7f5d70 sub_7f5d70
    Lifted::new(0x7000ee1df0, fn_7000ee1df0 as *const (), 0), // +0x7f5df0 sub_7f5df0
    Lifted::new(0x7000ee1e20, tpl_1616::<0x3, 0xffffffffffffffff, 0x13, 0x8, 0x10> as *const (), 0), // +0x7f5e20 sub_7f5e20
    Lifted::new(0x7000ee1e60, tpl_1616::<0x3, 0xffffffffffffffff, 0x13, 0x8, 0x10> as *const (), 0), // +0x7f5e60 sub_7f5e60
    Lifted::new(0x7000ee1ea0, tpl_1616::<0x3, 0xffffffffffffffff, 0x1b, 0x8, 0x10> as *const (), 0), // +0x7f5ea0 sub_7f5ea0
    Lifted::new(0x7000ee1ee0, tpl_1616::<0x3, 0xffffffffffffffff, 0x1b, 0x8, 0x10> as *const (), 0), // +0x7f5ee0 sub_7f5ee0
    Lifted::new(0x7000ee1f20, tpl_1616::<0x3, 0xffffffffffffffff, 0x17, 0x8, 0x10> as *const (), 0), // +0x7f5f20 sub_7f5f20
    Lifted::new(0x7000ee1f60, tpl_1616::<0x3, 0xffffffffffffffff, 0x13, 0x8, 0x10> as *const (), 0), // +0x7f5f60 sub_7f5f60
    Lifted::new(0x7000ee1fa0, tpl_1616::<0x3, 0xffffffffffffffff, 0x13, 0x8, 0x10> as *const (), 0), // +0x7f5fa0 sub_7f5fa0
    Lifted::new(0x7000ee1fe0, tpl_1616::<0x3, 0xffffffffffffffff, 0x13, 0x8, 0x10> as *const (), 0), // +0x7f5fe0 sub_7f5fe0
    Lifted::new(0x7000ee2020, tpl_1616::<0x3, 0xffffffffffffffff, 0x13, 0x8, 0x10> as *const (), 0), // +0x7f6020 sub_7f6020
    Lifted::new(0x7000ee2060, tpl_1617::<0x88, 0x13, 0x3, 0x10> as *const (), 0), // +0x7f6060 sub_7f6060
    Lifted::new(0x7000ee20b0, tpl_1617::<0x88, 0x13, 0x3, 0x10> as *const (), 0), // +0x7f60b0 sub_7f60b0
    Lifted::new(0x7000ee2100, tpl_1617::<0x88, 0x1b, 0x3, 0x10> as *const (), 0), // +0x7f6100 sub_7f6100
    Lifted::new(0x7000ee2150, tpl_1617::<0x88, 0x1b, 0x3, 0x10> as *const (), 0), // +0x7f6150 sub_7f6150
    Lifted::new(0x7000ee21a0, tpl_1617::<0x88, 0x17, 0x3, 0x10> as *const (), 0), // +0x7f61a0 sub_7f61a0
    Lifted::new(0x7000ee21f0, tpl_1617::<0x88, 0x13, 0x3, 0x10> as *const (), 0), // +0x7f61f0 sub_7f61f0
    Lifted::new(0x7000ee2240, tpl_1617::<0x88, 0x13, 0x3, 0x10> as *const (), 0), // +0x7f6240 sub_7f6240
    Lifted::new(0x7000ee2290, tpl_1617::<0x88, 0x13, 0x3, 0x10> as *const (), 0), // +0x7f6290 sub_7f6290
    Lifted::new(0x7000ee22e0, tpl_1617::<0x88, 0x13, 0x3, 0x10> as *const (), 0), // +0x7f62e0 sub_7f62e0
    Lifted::new(0x7000ee2330, fn_7000ee2330 as *const (), 0), // +0x7f6330 sub_7f6330
    Lifted::new(0x7000ee2550, fn_7000ee2550 as *const (), 0), // +0x7f6550 sub_7f6550
    Lifted::new(0x7000ee26e0, fn_7000ee26e0 as *const (), 0), // +0x7f66e0 sub_7f66e0
    Lifted::new(0x7000ee2860, tpl_224::<0x13> as *const (), 1), // +0x7f6860 sub_7f6860
    Lifted::new(0x7000ee28a0, tpl_225::<0x13> as *const (), 1), // +0x7f68a0 sub_7f68a0
    Lifted::new(0x7000ee28e0, tpl_226::<0x13> as *const (), 1), // +0x7f68e0 sub_7f68e0
    Lifted::new(0x7000ee2910, tpl_227::<0x13> as *const (), 1), // +0x7f6910 sub_7f6910
    Lifted::new(0x7000ee2950, fn_7000ee2950 as *const (), 1), // +0x7f6950 sub_7f6950
    Lifted::new(0x7000ee2990, fn_7000ee2990 as *const (), 1), // +0x7f6990 sub_7f6990
    Lifted::new(0x7000ee29d0, fn_7000ee29d0 as *const (), 1), // +0x7f69d0 sub_7f69d0
    Lifted::new(0x7000ee2a00, fn_7000ee2a00 as *const (), 1), // +0x7f6a00 sub_7f6a00
    Lifted::new(0x7000ee2a40, tpl_218::<0x13> as *const (), 1), // +0x7f6a40 sub_7f6a40
    Lifted::new(0x7000ee2a80, tpl_219::<0x13> as *const (), 1), // +0x7f6a80 sub_7f6a80
    Lifted::new(0x7000ee2ac0, tpl_220::<0x13> as *const (), 1), // +0x7f6ac0 sub_7f6ac0
    Lifted::new(0x7000ee2af0, tpl_221::<0x13> as *const (), 1), // +0x7f6af0 sub_7f6af0
    Lifted::new(0x7000ee2b30, tpl_149::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x7f6b30 sub_7f6b30
    Lifted::new(0x7000ee2b70, tpl_222::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x7f6b70 sub_7f6b70
    Lifted::new(0x7000ee2bb0, tpl_150::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x7f6bb0 sub_7f6bb0
    Lifted::new(0x7000ee2be0, tpl_223::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x7f6be0 sub_7f6be0
    Lifted::new(0x7000ee2c20, fn_7000ee2c20 as *const (), 0), // +0x7f6c20 sub_7f6c20
    Lifted::new(0x7000ee2e00, tpl_1618::<0x7001c72d48, 0x8, 0x30, 0x10, 0x700099ba10, 0x70006fc8f0, 0x20, 0x28, 0x7000790d10> as *const (), 0), // +0x7f6e00 sub_7f6e00
    Lifted::new(0x7000ee2f10, tpl_1619::<0x7000790d10> as *const (), 0), // +0x7f6f10 sub_7f6f10
    Lifted::new(0x7000ee2f60, tpl_1620::<0x7001c72d48, 0x8, 0x10, 0x70007343b0, 0x28, 0x18, 0xffffffffffffffff> as *const (), 0), // +0x7f6f60 sub_7f6f60
    Lifted::new(0x7000ee3050, tpl_1618::<0x7001c72d48, 0x8, 0x30, 0x10, 0x700099ba10, 0x70006fc8f0, 0x20, 0x28, 0x7000798560> as *const (), 0), // +0x7f7050 sub_7f7050
    Lifted::new(0x7000ee3160, tpl_1619::<0x7000798560> as *const (), 0), // +0x7f7160 sub_7f7160
    Lifted::new(0x7000ee31b0, tpl_1620::<0x7001c72d48, 0x8, 0x10, 0x70007532f0, 0x28, 0x18, 0xffffffffffffffff> as *const (), 0), // +0x7f71b0 sub_7f71b0
    Lifted::new(0x7000ee32a0, fn_7000ee32a0 as *const (), 1), // +0x7f72a0 sub_7f72a0
    Lifted::new(0x7000ee32b0, fn_7000ee32b0 as *const (), 1), // +0x7f72b0 sub_7f72b0
    Lifted::new(0x7000ee32f0, fn_7000ee32f0 as *const (), 0), // +0x7f72f0 sub_7f72f0
    Lifted::new(0x7000ee33f0, fn_7000ee33f0 as *const (), 0), // +0x7f73f0 sub_7f73f0
    Lifted::new(0x7000ee3510, fn_7000ee3510 as *const (), 0), // +0x7f7510 sub_7f7510
    Lifted::new(0x7000ee3590, fn_7000ee3590 as *const (), 0), // +0x7f7590 sub_7f7590
    Lifted::new(0x7000ee36a0, fn_7000ee36a0 as *const (), 0), // +0x7f76a0 sub_7f76a0
    Lifted::new(0x7000ee3770, fn_7000ee3770 as *const (), 0), // +0x7f7770 sub_7f7770
    Lifted::new(0x7000ee37a0, fn_7000ee37a0 as *const (), 0), // +0x7f77a0 sub_7f77a0
    Lifted::new(0x7000ee3800, fn_7000ee3800 as *const (), 2), // +0x7f7800 sub_7f7800
    Lifted::new(0x7000ee3a20, fn_7000ee3a20 as *const (), 1), // +0x7f7a20 sub_7f7a20
    Lifted::new(0x7000ee3ac0, fn_7000ee3ac0 as *const (), 0), // +0x7f7ac0 sub_7f7ac0
    Lifted::new(0x7000ee3c50, fn_7000ee3c50 as *const (), 0), // +0x7f7c50 sub_7f7c50
    Lifted::new(0x7000ee3d70, fn_7000ee3d70 as *const (), 0), // +0x7f7d70 sub_7f7d70
    Lifted::new(0x7000ee4180, fn_7000ee4180 as *const (), 0), // +0x7f8180 sub_7f8180
    Lifted::new(0x7000ee4d40, fn_7000ee4d40 as *const (), 0), // +0x7f8d40 sub_7f8d40
    Lifted::new(0x7000ee5200, fn_7000ee5200 as *const (), 1), // +0x7f9200 sub_7f9200
    Lifted::new(0x7000ee5340, fn_7000ee5340 as *const (), 1), // +0x7f9340 sub_7f9340
    Lifted::new(0x7000ee5470, fn_7000ee5470 as *const (), 1), // +0x7f9470 sub_7f9470
    Lifted::new(0x7000ee55a0, fn_7000ee55a0 as *const (), 0), // +0x7f95a0 sub_7f95a0
    Lifted::new(0x7000ee56b0, fn_7000ee56b0 as *const (), 0), // +0x7f96b0 sub_7f96b0
    Lifted::new(0x7000ee5800, fn_7000ee5800 as *const (), 0), // +0x7f9800 sub_7f9800
    Lifted::new(0x7000ee5b30, fn_7000ee5b30 as *const (), 0), // +0x7f9b30 sub_7f9b30
    Lifted::new(0x7000ee5b70, fn_7000ee5b70 as *const (), 0), // +0x7f9b70 sub_7f9b70
    Lifted::new(0x7000ee5e40, fn_7000ee5e40 as *const (), 0), // +0x7f9e40 sub_7f9e40
    Lifted::new(0x7000ee5ec0, fn_7000ee5ec0 as *const (), 0), // +0x7f9ec0 sub_7f9ec0
    Lifted::new(0x7000ee62b0, fn_7000ee62b0 as *const (), 1), // +0x7fa2b0 sub_7fa2b0
    Lifted::new(0x7000ee65b0, fn_7000ee65b0 as *const (), 2), // +0x7fa5b0 sub_7fa5b0
    Lifted::new(0x7000ee66f0, fn_7000ee66f0 as *const (), 2), // +0x7fa6f0 sub_7fa6f0
    Lifted::new(0x7000ee6820, tpl_1621::<0x70007329a0, 0x7000767d50, 0x700074a010, 0x7000758610, 0x7001d55098, 0x700075c500, 0x7000730960, 0x700070e790, 0x700076a480> as *const (), 0), // +0x7fa820 sub_7fa820
    Lifted::new(0x7000ee6c40, tpl_1622::<0x10, 0x700099ba10, 0xffffffffffff6898, 0x7000794bd0, 0x700073df10> as *const (), 0), // +0x7fac40 sub_7fac40
    Lifted::new(0x7000ee6d10, tpl_1621::<0x70007329a0, 0x7000767d50, 0x700074a010, 0x70007290c0, 0x7001d55098, 0x700075c500, 0x7000738be0, 0x70006f4b10, 0x7000779ff0> as *const (), 0), // +0x7fad10 sub_7fad10
    Lifted::new(0x7000ee7130, tpl_1622::<0x10, 0x700099ba10, 0xffffffffffff6898, 0x7000794bd0, 0x7000728c40> as *const (), 0), // +0x7fb130 sub_7fb130
    Lifted::new(0x7000ee7200, tpl_240::<0x1c> as *const (), 1), // +0x7fb200 sub_7fb200
    Lifted::new(0x7000ee7230, fn_7000ee7230 as *const (), 1), // +0x7fb230 sub_7fb230
    Lifted::new(0x7000ee7260, fn_7000ee7260 as *const (), 0), // +0x7fb260 sub_7fb260
    Lifted::new(0x7000ee7320, fn_7000ee7320 as *const (), 2), // +0x7fb320 sub_7fb320
    Lifted::new(0x7000ee7720, fn_7000ee7720 as *const (), 0), // +0x7fb720 sub_7fb720
    Lifted::new(0x7000ee7a20, fn_7000ee7a20 as *const (), 2), // +0x7fba20 sub_7fba20
    Lifted::new(0x7000ee7e50, tpl_1623::<0x7001c72d48, 0x700099ba10, 0x70006fc8f0, 0x7000e970f0, 0x70006f0b10> as *const (), 0), // +0x7fbe50 sub_7fbe50
    Lifted::new(0x7000ee8220, fn_7000ee8220 as *const (), 2), // +0x7fc220 sub_7fc220
    Lifted::new(0x7000ee8560, fn_7000ee8560 as *const (), 0), // +0x7fc560 sub_7fc560
    Lifted::new(0x7000ee8860, fn_7000ee8860 as *const (), 2), // +0x7fc860 sub_7fc860
    Lifted::new(0x7000ee8cd0, fn_7000ee8cd0 as *const (), 0), // +0x7fccd0 sub_7fccd0
    Lifted::new(0x7000ee8fc0, fn_7000ee8fc0 as *const (), 2), // +0x7fcfc0 sub_7fcfc0
    Lifted::new(0x7000ee9500, fn_7000ee9500 as *const (), 2), // +0x7fd500 sub_7fd500
    Lifted::new(0x7000ee98f0, fn_7000ee98f0 as *const (), 0), // +0x7fd8f0 sub_7fd8f0
    Lifted::new(0x7000ee9b70, fn_7000ee9b70 as *const (), 0), // +0x7fdb70 sub_7fdb70
    Lifted::new(0x7000ee9f30, fn_7000ee9f30 as *const (), 2), // +0x7fdf30 sub_7fdf30
    Lifted::new(0x7000eea2e0, tpl_1623::<0x7001c72d48, 0x700099ba10, 0x70006fc8f0, 0x7000e970f0, 0x7000717fa0> as *const (), 0), // +0x7fe2e0 sub_7fe2e0
    Lifted::new(0x7000eea6b0, fn_7000eea6b0 as *const (), 2), // +0x7fe6b0 sub_7fe6b0
    Lifted::new(0x7000eeaaa0, fn_7000eeaaa0 as *const (), 2), // +0x7feaa0 sub_7feaa0
    Lifted::new(0x7000eeae80, fn_7000eeae80 as *const (), 0), // +0x7fee80 sub_7fee80
    Lifted::new(0x7000eeb0a0, fn_7000eeb0a0 as *const (), 0), // +0x7ff0a0 sub_7ff0a0
    Lifted::new(0x7000eeb420, tpl_1624::<0x70006f2a70, 0x70006f9a60, 0x700076d780, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000eeb4b3, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x7ff420 sub_7ff420
    Lifted::new(0x7000eeb580, tpl_1624::<0x70006f2a70, 0x70006f9a60, 0x7000716e80, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000eeb613, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x7ff580 sub_7ff580
    Lifted::new(0x7000eeb6e0, fn_7000eeb6e0 as *const (), 2), // +0x7ff6e0 sub_7ff6e0
    Lifted::new(0x7000eeba20, tpl_1625::<0x700099ba10, 0x70006fc8f0, 0x7001c72d48, 0x8, 0x10, 0x70007748f0> as *const (), 0), // +0x7ffa20 sub_7ffa20
    Lifted::new(0x7000eebb40, tpl_1626::<0x7001c72d48, 0x70007329a0, 0x700070ed80, 0x70006eda10> as *const (), 0), // +0x7ffb40 sub_7ffb40
    Lifted::new(0x7000eebcb0, tpl_1625::<0x700099ba10, 0x70006fc8f0, 0x7001c72d48, 0x8, 0x10, 0x700079a630> as *const (), 0), // +0x7ffcb0 sub_7ffcb0
    Lifted::new(0x7000eebdd0, tpl_1626::<0x7001c72d48, 0x70007329a0, 0x7000712180, 0x70007915f0> as *const (), 0), // +0x7ffdd0 sub_7ffdd0
    Lifted::new(0x7000eebf40, fn_7000eebf40 as *const (), 2), // +0x7fff40 sub_7fff40
    Lifted::new(0x7000eec620, tpl_1627::<0x7001c72d48, 0x8, 0x10, 0x14, 0xffffffffffffffff, 0x18, 0x7, 0x7000e970f0, 0xb> as *const (), 0), // +0x800620 sub_800620
    Lifted::new(0x7000eec730, tpl_1627::<0x7001c72d48, 0x8, 0x10, 0x14, 0xffffffffffffffff, 0x18, 0x7, 0x7000e970f0, 0xb> as *const (), 0), // +0x800730 sub_800730
    Lifted::new(0x7000eec840, fn_7000eec840 as *const (), 2), // +0x800840 sub_800840
    Lifted::new(0x7000eeca80, fn_7000eeca80 as *const (), 2), // +0x800a80 sub_800a80
    Lifted::new(0x7000eecb40, fn_7000eecb40 as *const (), 0), // +0x800b40 sub_800b40
    Lifted::new(0x7000eecbe0, fn_7000eecbe0 as *const (), 1), // +0x800be0 sub_800be0
    Lifted::new(0x7000eecc20, fn_7000eecc20 as *const (), 0), // +0x800c20 sub_800c20
    Lifted::new(0x7000eed120, fn_7000eed120 as *const (), 1), // +0x801120 sub_801120
    Lifted::new(0x7000eed140, fn_7000eed140 as *const (), 2), // +0x801140 sub_801140
    Lifted::new(0x7000eed300, fn_7000eed300 as *const (), 2), // +0x801300 sub_801300
    Lifted::new(0x7000eed560, fn_7000eed560 as *const (), 2), // +0x801560 sub_801560
    Lifted::new(0x7000eed760, fn_7000eed760 as *const (), 2), // +0x801760 sub_801760
    Lifted::new(0x7000eed900, fn_7000eed900 as *const (), 0), // +0x801900 sub_801900
    Lifted::new(0x7000eedab0, fn_7000eedab0 as *const (), 0), // +0x801ab0 sub_801ab0
    Lifted::new(0x7000eedbb0, fn_7000eedbb0 as *const (), 0), // +0x801bb0 sub_801bb0
    Lifted::new(0x7000eedbf0, fn_7000eedbf0 as *const (), 0), // +0x801bf0 sub_801bf0
    Lifted::new(0x7000eedd10, fn_7000eedd10 as *const (), 0), // +0x801d10 sub_801d10
    Lifted::new(0x7000eedfe0, tpl_248::<0x7000e970f0> as *const (), 0), // +0x801fe0 sub_801fe0
    Lifted::new(0x7000eee000, tpl_248::<0x7000e970f0> as *const (), 0), // +0x802000 sub_802000
    Lifted::new(0x7000eee020, tpl_248::<0x7000e970f0> as *const (), 0), // +0x802020 sub_802020
    Lifted::new(0x7000eee040, tpl_248::<0x7000e970f0> as *const (), 0), // +0x802040 sub_802040
    Lifted::new(0x7000eee060, fn_7000eee060 as *const (), 0), // +0x802060 sub_802060
    Lifted::new(0x7000eee380, fn_7000eee380 as *const (), 0), // +0x802380 sub_802380
    Lifted::new(0x7000eee450, fn_7000eee450 as *const (), 0), // +0x802450 sub_802450
    Lifted::new(0x7000eee4f0, fn_7000eee4f0 as *const (), 0), // +0x8024f0 sub_8024f0
    Lifted::new(0x7000eee660, fn_7000eee660 as *const (), 0), // +0x802660 sub_802660
    Lifted::new(0x7000eee780, fn_7000eee780 as *const (), 0), // +0x802780 sub_802780
    Lifted::new(0x7000eeea90, fn_7000eeea90 as *const (), 0), // +0x802a90 sub_802a90
    Lifted::new(0x7000eeef20, fn_7000eeef20 as *const (), 0), // +0x802f20 sub_802f20
    Lifted::new(0x7000eeeff0, fn_7000eeeff0 as *const (), 0), // +0x802ff0 sub_802ff0
    Lifted::new(0x7000eef120, fn_7000eef120 as *const (), 0), // +0x803120 sub_803120
    Lifted::new(0x7000eef600, fn_7000eef600 as *const (), 0), // +0x803600 sub_803600
    Lifted::new(0x7000eefc80, fn_7000eefc80 as *const (), 0), // +0x803c80 sub_803c80
    Lifted::new(0x7000eefd00, fn_7000eefd00 as *const (), 0), // +0x803d00 sub_803d00
    Lifted::new(0x7000ef0130, fn_7000ef0130 as *const (), 2), // +0x804130 sub_804130
    Lifted::new(0x7000ef0760, fn_7000ef0760 as *const (), 0), // +0x804760 sub_804760
    Lifted::new(0x7000ef0870, tpl_1628::<0x1f0, 0x70007715d0> as *const (), 0), // +0x804870 sub_804870
    Lifted::new(0x7000ef08b0, tpl_1628::<0x210, 0x70007715d0> as *const (), 0), // +0x8048b0 sub_8048b0
    Lifted::new(0x7000ef08f0, tpl_1628::<0x220, 0x70007715d0> as *const (), 0), // +0x8048f0 sub_8048f0
    Lifted::new(0x7000ef0930, tpl_1628::<0x270, 0x70007715d0> as *const (), 0), // +0x804930 sub_804930
    Lifted::new(0x7000ef0970, tpl_1629::<0x700072fbb0> as *const (), 0), // +0x804970 sub_804970
    Lifted::new(0x7000ef09a0, fn_7000ef09a0 as *const (), 0), // +0x8049a0 sub_8049a0
    Lifted::new(0x7000ef0ab0, tpl_1630::<0x7001c6abd8, 0x70017d94d8, 0x7001d543d0, 0x7001d543c8, 0x70017d94c8, 0x7001d543f8, 0x70017d9438, 0x700072fbb0, 0x70009a6500, 0x70010e2510, 0x7001d54968, 0x7000ef0c88, 0x700177c312, 0x7000a633c0, 0x7001d54460, 0x7001d54468> as *const (), 2), // +0x804ab0 sub_804ab0
    Lifted::new(0x7000ef0da0, tpl_257::<0x7000724470> as *const (), 0), // +0x804da0 sub_804da0
    Lifted::new(0x7000ef0dc0, tpl_257::<0x700079a0a0> as *const (), 0), // +0x804dc0 sub_804dc0
    Lifted::new(0x7000ef0de0, fn_7000ef0de0 as *const (), 0), // +0x804de0 sub_804de0
    Lifted::new(0x7000ef0fa0, fn_7000ef0fa0 as *const (), 0), // +0x804fa0 sub_804fa0
    Lifted::new(0x7000ef1160, fn_7000ef1160 as *const (), 0), // +0x805160 sub_805160
    Lifted::new(0x7000ef1310, fn_7000ef1310 as *const (), 0), // +0x805310 sub_805310
    Lifted::new(0x7000ef14d0, fn_7000ef14d0 as *const (), 0), // +0x8054d0 sub_8054d0
    Lifted::new(0x7000ef1690, tpl_1631::<0x7001d55098, 0x70007a5b20, 0x7000e970f0, 0x7000738880, 0x7001859148, 0x700070b750, 0x7000ef17e6> as *const (), 0), // +0x805690 sub_805690
    Lifted::new(0x7000ef1840, fn_7000ef1840 as *const (), 0), // +0x805840 sub_805840
    Lifted::new(0x7000ef1a00, fn_7000ef1a00 as *const (), 0), // +0x805a00 sub_805a00
    Lifted::new(0x7000ef1bc0, tpl_1631::<0x7001d55098, 0x70007a5b20, 0x7000e970f0, 0x7000742220, 0x7001859148, 0x700070b750, 0x7000ef1d16> as *const (), 0), // +0x805bc0 sub_805bc0
    Lifted::new(0x7000ef1d70, fn_7000ef1d70 as *const (), 0), // +0x805d70 sub_805d70
    Lifted::new(0x7000ef1da0, fn_7000ef1da0 as *const (), 0), // +0x805da0 sub_805da0
    Lifted::new(0x7000ef1e60, fn_7000ef1e60 as *const (), 0), // +0x805e60 sub_805e60
    Lifted::new(0x7000ef2460, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x806460 sub_806460
    Lifted::new(0x7000ef2470, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x806470 sub_806470
    Lifted::new(0x7000ef2480, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x806480 sub_806480
    Lifted::new(0x7000ef2490, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x806490 sub_806490
    Lifted::new(0x7000ef24a0, fn_7000ef24a0 as *const (), 1), // +0x8064a0 sub_8064a0
    Lifted::new(0x7000ef25c0, fn_7000ef25c0 as *const (), 1), // +0x8065c0 sub_8065c0
    Lifted::new(0x7000ef26b0, fn_7000ef26b0 as *const (), 1), // +0x8066b0 sub_8066b0
    Lifted::new(0x7000ef27d0, fn_7000ef27d0 as *const (), 1), // +0x8067d0 sub_8067d0
    Lifted::new(0x7000ef28e0, fn_7000ef28e0 as *const (), 0), // +0x8068e0 sub_8068e0
    Lifted::new(0x7000ef2900, fn_7000ef2900 as *const (), 1), // +0x806900 sub_806900
    Lifted::new(0x7000ef2fa0, fn_7000ef2fa0 as *const (), 0), // +0x806fa0 sub_806fa0
    Lifted::new(0x7000ef3240, fn_7000ef3240 as *const (), 0), // +0x807240 sub_807240
    Lifted::new(0x7000ef32a0, fn_7000ef32a0 as *const (), 0), // +0x8072a0 sub_8072a0
    Lifted::new(0x7000ef3440, fn_7000ef3440 as *const (), 1), // +0x807440 sub_807440
    Lifted::new(0x7000ef34c0, fn_7000ef34c0 as *const (), 1), // +0x8074c0 sub_8074c0
    Lifted::new(0x7000ef3530, fn_7000ef3530 as *const (), 2), // +0x807530 sub_807530
    Lifted::new(0x7000ef3940, fn_7000ef3940 as *const (), 2), // +0x807940 sub_807940
    Lifted::new(0x7000ef3bd0, fn_7000ef3bd0 as *const (), 0), // +0x807bd0 sub_807bd0
    Lifted::new(0x7000ef3d20, fn_7000ef3d20 as *const (), 2), // +0x807d20 sub_807d20
    Lifted::new(0x7000ef4130, fn_7000ef4130 as *const (), 2), // +0x808130 sub_808130
    Lifted::new(0x7000ef4540, tpl_1632::<0x70009bb6f0, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x808540 sub_808540
    Lifted::new(0x7000ef4690, tpl_1633::<0x70009bb6f0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x808690 sub_808690
    Lifted::new(0x7000ef49e0, tpl_1633::<0x70009bb6f0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x8089e0 sub_8089e0
    Lifted::new(0x7000ef4d30, tpl_1632::<0x70009bb6f0, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x808d30 sub_808d30
    Lifted::new(0x7000ef4e80, fn_7000ef4e80 as *const (), 0), // +0x808e80 sub_808e80
    Lifted::new(0x7000ef4fb0, fn_7000ef4fb0 as *const (), 0), // +0x808fb0 sub_808fb0
    Lifted::new(0x7000ef5070, fn_7000ef5070 as *const (), 0), // +0x809070 sub_809070
    Lifted::new(0x7000ef5290, fn_7000ef5290 as *const (), 0), // +0x809290 sub_809290
    Lifted::new(0x7000ef53c0, fn_7000ef53c0 as *const (), 3), // +0x8093c0 sub_8093c0
    Lifted::new(0x7000ef5460, tpl_267::<0x13, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffff6940> as *const (), 1), // +0x809460 sub_809460
    Lifted::new(0x7000ef54e0, tpl_268::<0x13, 0x20, 0x28> as *const (), 1), // +0x8094e0 sub_8094e0
    Lifted::new(0x7000ef5530, fn_7000ef5530 as *const (), 1), // +0x809530 sub_809530
    Lifted::new(0x7000ef55b0, fn_7000ef55b0 as *const (), 1), // +0x8095b0 sub_8095b0
    Lifted::new(0x7000ef5600, tpl_265::<0x13, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffff6940> as *const (), 1), // +0x809600 sub_809600
    Lifted::new(0x7000ef5680, tpl_266::<0x13, 0x20, 0x28> as *const (), 1), // +0x809680 sub_809680
    Lifted::new(0x7000ef56d0, tpl_269::<0x14, 0xffffffffffffffff, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffff6940> as *const (), 1), // +0x8096d0 sub_8096d0
    Lifted::new(0x7000ef5750, tpl_270::<0x14, 0xffffffffffffffff, 0x20, 0x28> as *const (), 1), // +0x809750 sub_809750
    Lifted::new(0x7000ef57a0, fn_7000ef57a0 as *const (), 0), // +0x8097a0 sub_8097a0
    Lifted::new(0x7000ef5840, fn_7000ef5840 as *const (), 0), // +0x809840 sub_809840
    Lifted::new(0x7000ef58d0, fn_7000ef58d0 as *const (), 0), // +0x8098d0 sub_8098d0
    Lifted::new(0x7000ef5bd0, fn_7000ef5bd0 as *const (), 2), // +0x809bd0 sub_809bd0
    Lifted::new(0x7000ef5cd0, fn_7000ef5cd0 as *const (), 2), // +0x809cd0 sub_809cd0
    Lifted::new(0x7000ef6690, fn_7000ef6690 as *const (), 2), // +0x80a690 sub_80a690
    Lifted::new(0x7000ef6820, fn_7000ef6820 as *const (), 3), // +0x80a820 sub_80a820
    Lifted::new(0x7000ef6890, fn_7000ef6890 as *const (), 3), // +0x80a890 sub_80a890
    Lifted::new(0x7000ef6900, fn_7000ef6900 as *const (), 0), // +0x80a900 sub_80a900
    Lifted::new(0x7000ef6b90, tpl_14 as *const (), 1), // +0x80ab90 sub_80ab90
    Lifted::new(0x7000ef6ba0, tpl_14 as *const (), 1), // +0x80aba0 sub_80aba0
    Lifted::new(0x7000ef6bb0, tpl_14 as *const (), 1), // +0x80abb0 sub_80abb0
    Lifted::new(0x7000ef6bc0, fn_7000ef6bc0 as *const (), 0), // +0x80abc0 sub_80abc0
    Lifted::new(0x7000ef6c90, fn_7000ef6c90 as *const (), 1), // +0x80ac90 sub_80ac90
    Lifted::new(0x7000ef6e30, tpl_224::<0x1b> as *const (), 1), // +0x80ae30 sub_80ae30
    Lifted::new(0x7000ef6e70, tpl_226::<0x1b> as *const (), 1), // +0x80ae70 sub_80ae70
    Lifted::new(0x7000ef6ea0, tpl_229::<0x18, 0xffffffffffffffff> as *const (), 1), // +0x80aea0 sub_80aea0
    Lifted::new(0x7000ef6ee0, fn_7000ef6ee0 as *const (), 1), // +0x80aee0 sub_80aee0
    Lifted::new(0x7000ef6f10, tpl_229::<0x18, 0xffffffffffffffff> as *const (), 1), // +0x80af10 sub_80af10
    Lifted::new(0x7000ef6f50, tpl_229::<0x18, 0xffffffffffffffff> as *const (), 1), // +0x80af50 sub_80af50
    Lifted::new(0x7000ef6f90, fn_7000ef6f90 as *const (), 0), // +0x80af90 sub_80af90
    Lifted::new(0x7000ef6fa0, fn_7000ef6fa0 as *const (), 0), // +0x80afa0 sub_80afa0
    Lifted::new(0x7000ef6fb0, fn_7000ef6fb0 as *const (), 2), // +0x80afb0 sub_80afb0
    Lifted::new(0x7000ef7160, fn_7000ef7160 as *const (), 0), // +0x80b160 sub_80b160
    Lifted::new(0x7000ef7220, fn_7000ef7220 as *const (), 0), // +0x80b220 sub_80b220
    Lifted::new(0x7000ef7340, fn_7000ef7340 as *const (), 1), // +0x80b340 sub_80b340
    Lifted::new(0x7000ef7360, fn_7000ef7360 as *const (), 0), // +0x80b360 sub_80b360
    Lifted::new(0x7000ef7390, fn_7000ef7390 as *const (), 0), // +0x80b390 sub_80b390
    Lifted::new(0x7000ef73f0, fn_7000ef73f0 as *const (), 0), // +0x80b3f0 sub_80b3f0
    Lifted::new(0x7000ef7460, fn_7000ef7460 as *const (), 0), // +0x80b460 sub_80b460
    Lifted::new(0x7000ef7520, fn_7000ef7520 as *const (), 1), // +0x80b520 sub_80b520
    Lifted::new(0x7000ef75d0, fn_7000ef75d0 as *const (), 0), // +0x80b5d0 sub_80b5d0
    Lifted::new(0x7000ef7780, fn_7000ef7780 as *const (), 1), // +0x80b780 sub_80b780
    Lifted::new(0x7000ef7790, tpl_1634::<0x7000ef9460, 0x7000e970f0, 0x70009bb620> as *const (), 0), // +0x80b790 sub_80b790
    Lifted::new(0x7000ef7a60, tpl_1634::<0x7000ef9a00, 0x7000e970f0, 0x70009bb620> as *const (), 0), // +0x80ba60 sub_80ba60
    Lifted::new(0x7000ef7d30, fn_7000ef7d30 as *const (), 0), // +0x80bd30 sub_80bd30
    Lifted::new(0x7000ef7eb0, fn_7000ef7eb0 as *const (), 0), // +0x80beb0 sub_80beb0
    Lifted::new(0x7000ef8030, fn_7000ef8030 as *const (), 0), // +0x80c030 sub_80c030
    Lifted::new(0x7000ef81a0, tpl_13::<0x7000ef7d30> as *const (), 0), // +0x80c1a0 sub_80c1a0
    Lifted::new(0x7000ef81c0, tpl_13::<0x7000ef7eb0> as *const (), 0), // +0x80c1c0 sub_80c1c0
    Lifted::new(0x7000ef81e0, fn_7000ef81e0 as *const (), 0), // +0x80c1e0 sub_80c1e0
    Lifted::new(0x7000ef8230, fn_7000ef8230 as *const (), 0), // +0x80c230 sub_80c230
    Lifted::new(0x7000ef83b0, fn_7000ef83b0 as *const (), 0), // +0x80c3b0 sub_80c3b0
    Lifted::new(0x7000ef8550, tpl_1635::<0x7, 0xb, 0xf, 0x7000ef8aa0> as *const (), 0), // +0x80c550 sub_80c550
    Lifted::new(0x7000ef85e0, tpl_1635::<0x7, 0xb, 0xf, 0x7000ef8fb0> as *const (), 0), // +0x80c5e0 sub_80c5e0
    Lifted::new(0x7000ef8670, tpl_1636::<0xb, 0xf, 0x13, 0x7000ef9460, 0x7, 0x7000e970f0> as *const (), 0), // +0x80c670 sub_80c670
    Lifted::new(0x7000ef8740, tpl_1636::<0xb, 0xf, 0x13, 0x7000ef9a00, 0x7, 0x7000e970f0> as *const (), 0), // +0x80c740 sub_80c740
    Lifted::new(0x7000ef8810, tpl_1637 as *const (), 1), // +0x80c810 sub_80c810
    Lifted::new(0x7000ef8940, tpl_1637 as *const (), 1), // +0x80c940 sub_80c940
    Lifted::new(0x7000ef8a70, tpl_1638::<0xf, 0x7000ef8aa0> as *const (), 0), // +0x80ca70 sub_80ca70
    Lifted::new(0x7000ef8aa0, fn_7000ef8aa0 as *const (), 2), // +0x80caa0 sub_80caa0
    Lifted::new(0x7000ef8f80, tpl_1638::<0xf, 0x7000ef8fb0> as *const (), 0), // +0x80cf80 sub_80cf80
    Lifted::new(0x7000ef8fb0, fn_7000ef8fb0 as *const (), 2), // +0x80cfb0 sub_80cfb0
    Lifted::new(0x7000ef9460, fn_7000ef9460 as *const (), 2), // +0x80d460 sub_80d460
    Lifted::new(0x7000ef9990, tpl_1639::<0x7000ef9460, 0x7, 0x7000e970f0> as *const (), 0), // +0x80d990 sub_80d990
    Lifted::new(0x7000ef9a00, fn_7000ef9a00 as *const (), 2), // +0x80da00 sub_80da00
    Lifted::new(0x7000ef9ef0, tpl_1639::<0x7000ef9a00, 0x7, 0x7000e970f0> as *const (), 0), // +0x80def0 sub_80def0
    Lifted::new(0x7000ef9f60, fn_7000ef9f60 as *const (), 0), // +0x80df60 sub_80df60
    Lifted::new(0x7000efa2d0, fn_7000efa2d0 as *const (), 0), // +0x80e2d0 sub_80e2d0
    Lifted::new(0x7000efa3d0, fn_7000efa3d0 as *const (), 0), // +0x80e3d0 sub_80e3d0
    Lifted::new(0x7000efa700, fn_7000efa700 as *const (), 0), // +0x80e700 sub_80e700
    Lifted::new(0x7000efa7f0, fn_7000efa7f0 as *const (), 0), // +0x80e7f0 sub_80e7f0
    Lifted::new(0x7000efa910, fn_7000efa910 as *const (), 2), // +0x80e910 sub_80e910
    Lifted::new(0x7000efac70, fn_7000efac70 as *const (), 0), // +0x80ec70 sub_80ec70
    Lifted::new(0x7000efaee0, fn_7000efaee0 as *const (), 2), // +0x80eee0 sub_80eee0
    Lifted::new(0x7000efb230, fn_7000efb230 as *const (), 0), // +0x80f230 sub_80f230
    Lifted::new(0x7000efb430, fn_7000efb430 as *const (), 0), // +0x80f430 sub_80f430
    Lifted::new(0x7000efb5a0, fn_7000efb5a0 as *const (), 0), // +0x80f5a0 sub_80f5a0
    Lifted::new(0x7000efb7f0, tpl_1640::<0x7000726930, 0x7000ef7d30> as *const (), 0), // +0x80f7f0 sub_80f7f0
    Lifted::new(0x7000efb860, tpl_1640::<0x700071a2f0, 0x7000ef7eb0> as *const (), 0), // +0x80f860 sub_80f860
    Lifted::new(0x7000efb8d0, fn_7000efb8d0 as *const (), 0), // +0x80f8d0 sub_80f8d0
    Lifted::new(0x7000efb970, tpl_454::<0x7000726930> as *const (), 0), // +0x80f970 sub_80f970
    Lifted::new(0x7000efb9a0, tpl_454::<0x700071a2f0> as *const (), 0), // +0x80f9a0 sub_80f9a0
    Lifted::new(0x7000efb9d0, tpl_454::<0x7000782820> as *const (), 0), // +0x80f9d0 sub_80f9d0
    Lifted::new(0x7000efba00, tpl_1641::<0x230, 0x70007a5b20, 0x2, 0x7000e970f0, 0x1> as *const (), 0), // +0x80fa00 sub_80fa00
    Lifted::new(0x7000efbac0, tpl_1641::<0x238, 0x70007a5b20, 0x2, 0x7000e970f0, 0x1> as *const (), 0), // +0x80fac0 sub_80fac0
    Lifted::new(0x7000efbb80, fn_7000efbb80 as *const (), 0), // +0x80fb80 sub_80fb80
    Lifted::new(0x7000efbc40, fn_7000efbc40 as *const (), 0), // +0x80fc40 sub_80fc40
    Lifted::new(0x7000efbc70, fn_7000efbc70 as *const (), 0), // +0x80fc70 sub_80fc70
    Lifted::new(0x7000efbca0, fn_7000efbca0 as *const (), 0), // +0x80fca0 sub_80fca0
    Lifted::new(0x7000efbcf0, fn_7000efbcf0 as *const (), 1), // +0x80fcf0 sub_80fcf0
    Lifted::new(0x7000efbd30, tpl_1642::<0x7000ef7d30, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000efbd7c, 0x7000767d50, 0x700074a010, 0x7000e970f0> as *const (), 0), // +0x80fd30 sub_80fd30
    Lifted::new(0x7000efbe90, tpl_1642::<0x7000ef7eb0, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000efbedc, 0x7000767d50, 0x700074a010, 0x7000e970f0> as *const (), 0), // +0x80fe90 sub_80fe90
    Lifted::new(0x7000efbff0, fn_7000efbff0 as *const (), 0), // +0x80fff0 sub_80fff0
    Lifted::new(0x7000efc150, fn_7000efc150 as *const (), 2), // +0x810150 sub_810150
    Lifted::new(0x7000efc620, fn_7000efc620 as *const (), 2), // +0x810620 sub_810620
    Lifted::new(0x7000efc820, fn_7000efc820 as *const (), 0), // +0x810820 sub_810820
    Lifted::new(0x7000efc9a0, fn_7000efc9a0 as *const (), 2), // +0x8109a0 sub_8109a0
    Lifted::new(0x7000efcc30, fn_7000efcc30 as *const (), 0), // +0x810c30 sub_810c30
    Lifted::new(0x7000efcc80, fn_7000efcc80 as *const (), 0), // +0x810c80 sub_810c80
    Lifted::new(0x7000efccd0, fn_7000efccd0 as *const (), 2), // +0x810cd0 sub_810cd0
    Lifted::new(0x7000efcee0, fn_7000efcee0 as *const (), 2), // +0x810ee0 sub_810ee0
    Lifted::new(0x7000efd070, fn_7000efd070 as *const (), 0), // +0x811070 sub_811070
    Lifted::new(0x7000efd1a0, fn_7000efd1a0 as *const (), 2), // +0x8111a0 sub_8111a0
    Lifted::new(0x7000efd380, fn_7000efd380 as *const (), 2), // +0x811380 sub_811380
    Lifted::new(0x7000efd600, fn_7000efd600 as *const (), 0), // +0x811600 sub_811600
    Lifted::new(0x7000efd680, fn_7000efd680 as *const (), 2), // +0x811680 sub_811680
    Lifted::new(0x7000efd890, fn_7000efd890 as *const (), 1), // +0x811890 sub_811890
    Lifted::new(0x7000efd910, fn_7000efd910 as *const (), 0), // +0x811910 sub_811910
    Lifted::new(0x7000efd9a0, fn_7000efd9a0 as *const (), 0), // +0x8119a0 sub_8119a0
    Lifted::new(0x7000efdc60, fn_7000efdc60 as *const (), 0), // +0x811c60 sub_811c60
    Lifted::new(0x7000efdf20, fn_7000efdf20 as *const (), 0), // +0x811f20 sub_811f20
    Lifted::new(0x7000efe1f0, fn_7000efe1f0 as *const (), 0), // +0x8121f0 sub_8121f0
    Lifted::new(0x7000efe350, fn_7000efe350 as *const (), 0), // +0x812350 sub_812350
    Lifted::new(0x7000efe4b0, fn_7000efe4b0 as *const (), 1), // +0x8124b0 sub_8124b0
    Lifted::new(0x7000efe540, fn_7000efe540 as *const (), 0), // +0x812540 sub_812540
    Lifted::new(0x7000efe630, tpl_1320::<0x380> as *const (), 1), // +0x812630 sub_812630
    Lifted::new(0x7000efe650, tpl_1320::<0x388> as *const (), 1), // +0x812650 sub_812650
    Lifted::new(0x7000efe670, fn_7000efe670 as *const (), 0), // +0x812670 sub_812670
    Lifted::new(0x7000efe6f0, tpl_1643::<0x5, 0x4, 0x700078a720> as *const (), 0), // +0x8126f0 sub_8126f0
    Lifted::new(0x7000efe770, tpl_1643::<0x5, 0x4, 0x70006ee970> as *const (), 0), // +0x812770 sub_812770
    Lifted::new(0x7000efe7f0, fn_7000efe7f0 as *const (), 0), // +0x8127f0 sub_8127f0
    Lifted::new(0x7000efe8a0, tpl_1644::<0x7000757810> as *const (), 0), // +0x8128a0 sub_8128a0
    Lifted::new(0x7000efe8e0, tpl_1644::<0x7000775cb0> as *const (), 0), // +0x8128e0 sub_8128e0
    Lifted::new(0x7000efe920, fn_7000efe920 as *const (), 1), // +0x812920 sub_812920
    Lifted::new(0x7000efea00, fn_7000efea00 as *const (), 0), // +0x812a00 sub_812a00
    Lifted::new(0x7000efea90, fn_7000efea90 as *const (), 0), // +0x812a90 sub_812a90
    Lifted::new(0x7000efeb20, fn_7000efeb20 as *const (), 0), // +0x812b20 sub_812b20
    Lifted::new(0x7000efebf0, tpl_1645::<0x7000efe1f0> as *const (), 0), // +0x812bf0 sub_812bf0
    Lifted::new(0x7000efec20, tpl_1645::<0x7000efe350> as *const (), 0), // +0x812c20 sub_812c20
    Lifted::new(0x7000efece0, tpl_1645::<0x7000efe1f0> as *const (), 0), // +0x812ce0 sub_812ce0
    Lifted::new(0x7000efed10, tpl_1645::<0x7000efe350> as *const (), 0), // +0x812d10 sub_812d10
    Lifted::new(0x7000efed40, fn_7000efed40 as *const (), 0), // +0x812d40 sub_812d40
    Lifted::new(0x7000efeee0, fn_7000efeee0 as *const (), 0), // +0x812ee0 sub_812ee0
    Lifted::new(0x7000eff080, fn_7000eff080 as *const (), 1), // +0x813080 sub_813080
    Lifted::new(0x7000eff0c0, fn_7000eff0c0 as *const (), 0), // +0x8130c0 sub_8130c0
    Lifted::new(0x7000eff180, fn_7000eff180 as *const (), 0), // +0x813180 sub_813180
    Lifted::new(0x7000eff230, fn_7000eff230 as *const (), 0), // +0x813230 sub_813230
    Lifted::new(0x7000eff2f0, fn_7000eff2f0 as *const (), 1), // +0x8132f0 sub_8132f0
    Lifted::new(0x7000eff3c0, fn_7000eff3c0 as *const (), 1), // +0x8133c0 sub_8133c0
    Lifted::new(0x7000eff400, fn_7000eff400 as *const (), 2), // +0x813400 sub_813400
    Lifted::new(0x7000eff750, fn_7000eff750 as *const (), 2), // +0x813750 sub_813750
    Lifted::new(0x7000effa20, fn_7000effa20 as *const (), 2), // +0x813a20 sub_813a20
    Lifted::new(0x7000effd80, tpl_1638::<0xf, 0x7000ef8aa0> as *const (), 0), // +0x813d80 sub_813d80
    Lifted::new(0x7000effdb0, tpl_13::<0x7000ef8aa0> as *const (), 0), // +0x813db0 sub_813db0
    Lifted::new(0x7000effdd0, tpl_1638::<0xf, 0x7000ef8fb0> as *const (), 0), // +0x813dd0 sub_813dd0
    Lifted::new(0x7000effe00, tpl_13::<0x7000ef8fb0> as *const (), 0), // +0x813e00 sub_813e00
    Lifted::new(0x7000effe20, tpl_13::<0x700078a720> as *const (), 0), // +0x813e20 sub_813e20
    Lifted::new(0x7000effe40, tpl_13::<0x70006ee970> as *const (), 0), // +0x813e40 sub_813e40
    Lifted::new(0x7000effe60, fn_7000effe60 as *const (), 0), // +0x813e60 sub_813e60
    Lifted::new(0x7000effea0, tpl_262::<0x1, 0xffffffffffffffff, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x813ea0 sub_813ea0
    Lifted::new(0x7000efff70, fn_7000efff70 as *const (), 0), // +0x813f70 sub_813f70
    Lifted::new(0x7000f00040, fn_7000f00040 as *const (), 0), // +0x814040 sub_814040
    Lifted::new(0x7000f00110, fn_7000f00110 as *const (), 0), // +0x814110 sub_814110
    Lifted::new(0x7000f00190, fn_7000f00190 as *const (), 2), // +0x814190 sub_814190
    Lifted::new(0x7000f00550, fn_7000f00550 as *const (), 0), // +0x814550 sub_814550
    Lifted::new(0x7000f005d0, fn_7000f005d0 as *const (), 0), // +0x8145d0 sub_8145d0
    Lifted::new(0x7000f00640, tpl_1646::<0x7, 0xf, 0x7000ef8aa0, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000f006ab> as *const (), 0), // +0x814640 sub_814640
    Lifted::new(0x7000f006e0, tpl_1646::<0x7, 0xf, 0x7000ef8fb0, 0x70017d3780, 0x70017a0290, 0x7001d54ef8, 0x7000f0074b> as *const (), 0), // +0x8146e0 sub_8146e0
    Lifted::new(0x7000f00780, fn_7000f00780 as *const (), 0), // +0x814780 sub_814780
    Lifted::new(0x7000f00850, tpl_1647::<0x5, 0x3, 0x700078a720> as *const (), 0), // +0x814850 sub_814850
    Lifted::new(0x7000f008c0, tpl_1647::<0x5, 0x3, 0x70006ee970> as *const (), 0), // +0x8148c0 sub_8148c0
    Lifted::new(0x7000f00930, fn_7000f00930 as *const (), 0), // +0x814930 sub_814930
    Lifted::new(0x7000f009c0, fn_7000f009c0 as *const (), 0), // +0x8149c0 sub_8149c0
    Lifted::new(0x7000f00aa0, tpl_1648::<0x7000767d50, 0x700074a010> as *const (), 2), // +0x814aa0 sub_814aa0
    Lifted::new(0x7000f00c30, tpl_1648::<0x7000767d50, 0x700074a010> as *const (), 2), // +0x814c30 sub_814c30
    Lifted::new(0x7000f00dc0, fn_7000f00dc0 as *const (), 1), // +0x814dc0 sub_814dc0
    Lifted::new(0x7000f00e50, fn_7000f00e50 as *const (), 0), // +0x814e50 sub_814e50
    Lifted::new(0x7000f00f50, fn_7000f00f50 as *const (), 2), // +0x814f50 sub_814f50
    Lifted::new(0x7000f012c0, fn_7000f012c0 as *const (), 1), // +0x8152c0 sub_8152c0
    Lifted::new(0x7000f012e0, fn_7000f012e0 as *const (), 0), // +0x8152e0 sub_8152e0
    Lifted::new(0x7000f013c0, fn_7000f013c0 as *const (), 0), // +0x8153c0 sub_8153c0
    Lifted::new(0x7000f01450, fn_7000f01450 as *const (), 0), // +0x815450 sub_815450
    Lifted::new(0x7000f01710, fn_7000f01710 as *const (), 0), // +0x815710 sub_815710
    Lifted::new(0x7000f01840, fn_7000f01840 as *const (), 1), // +0x815840 sub_815840
    Lifted::new(0x7000f01940, fn_7000f01940 as *const (), 1), // +0x815940 sub_815940
    Lifted::new(0x7000f01990, tpl_1649::<0xc, 0xffffffffffffffff> as *const (), 1), // +0x815990 sub_815990
    Lifted::new(0x7000f01a00, fn_7000f01a00 as *const (), 0), // +0x815a00 sub_815a00
    Lifted::new(0x7000f01ac0, tpl_1649::<0x8, 0xffffffffffffffff> as *const (), 1), // +0x815ac0 sub_815ac0
    Lifted::new(0x7000f01b30, fn_7000f01b30 as *const (), 2), // +0x815b30 sub_815b30
    Lifted::new(0x7000f01dd0, fn_7000f01dd0 as *const (), 0), // +0x815dd0 sub_815dd0
    Lifted::new(0x7000f01e50, fn_7000f01e50 as *const (), 2), // +0x815e50 sub_815e50
    Lifted::new(0x7000f021d0, fn_7000f021d0 as *const (), 1), // +0x8161d0 sub_8161d0
    Lifted::new(0x7000f021f0, fn_7000f021f0 as *const (), 1), // +0x8161f0 sub_8161f0
    Lifted::new(0x7000f02270, fn_7000f02270 as *const (), 0), // +0x816270 sub_816270
    Lifted::new(0x7000f02350, fn_7000f02350 as *const (), 0), // +0x816350 sub_816350
    Lifted::new(0x7000f02420, fn_7000f02420 as *const (), 0), // +0x816420 sub_816420
    Lifted::new(0x7000f02540, fn_7000f02540 as *const (), 2), // +0x816540 sub_816540
    Lifted::new(0x7000f02be0, fn_7000f02be0 as *const (), 0), // +0x816be0 sub_816be0
    Lifted::new(0x7000f02f80, fn_7000f02f80 as *const (), 0), // +0x816f80 sub_816f80
    Lifted::new(0x7000f03580, fn_7000f03580 as *const (), 2), // +0x817580 sub_817580
    Lifted::new(0x7000f03850, fn_7000f03850 as *const (), 3), // +0x817850 sub_817850
    Lifted::new(0x7000f03870, fn_7000f03870 as *const (), 3), // +0x817870 sub_817870
    Lifted::new(0x7000f038c0, fn_7000f038c0 as *const (), 1), // +0x8178c0 sub_8178c0
    Lifted::new(0x7000f038e0, fn_7000f038e0 as *const (), 0), // +0x8178e0 sub_8178e0
    Lifted::new(0x7000f039c0, fn_7000f039c0 as *const (), 0), // +0x8179c0 sub_8179c0
    Lifted::new(0x7000f03a80, fn_7000f03a80 as *const (), 0), // +0x817a80 sub_817a80
    Lifted::new(0x7000f03ad0, fn_7000f03ad0 as *const (), 3), // +0x817ad0 sub_817ad0
    Lifted::new(0x7000f03b10, fn_7000f03b10 as *const (), 2), // +0x817b10 sub_817b10
    Lifted::new(0x7000f03bf0, fn_7000f03bf0 as *const (), 3), // +0x817bf0 sub_817bf0
    Lifted::new(0x7000f03ce0, fn_7000f03ce0 as *const (), 2), // +0x817ce0 sub_817ce0
    Lifted::new(0x7000f03d70, fn_7000f03d70 as *const (), 3), // +0x817d70 sub_817d70
    Lifted::new(0x7000f03dc0, fn_7000f03dc0 as *const (), 0), // +0x817dc0 sub_817dc0
    Lifted::new(0x7000f03ff0, fn_7000f03ff0 as *const (), 0), // +0x817ff0 sub_817ff0
    Lifted::new(0x7000f04170, fn_7000f04170 as *const (), 0), // +0x818170 sub_818170
    Lifted::new(0x7000f04f20, fn_7000f04f20 as *const (), 0), // +0x818f20 sub_818f20
    Lifted::new(0x7000f05cd0, tpl_1650::<0x7000716f10, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x819cd0 sub_819cd0
    Lifted::new(0x7000f05ea0, fn_7000f05ea0 as *const (), 2), // +0x819ea0 sub_819ea0
    Lifted::new(0x7000f066a0, tpl_1651::<0x7000716f10, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x81a6a0 sub_81a6a0
    Lifted::new(0x7000f06960, tpl_1650::<0x700077d960, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x81a960 sub_81a960
    Lifted::new(0x7000f06b30, fn_7000f06b30 as *const (), 2), // +0x81ab30 sub_81ab30
    Lifted::new(0x7000f07330, tpl_1651::<0x700077d960, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x81b330 sub_81b330
    Lifted::new(0x7000f075f0, tpl_1599::<0x70017d2c10, 0x7001d54ef8, 0x7000f07614, 0x7001859404, 0x7000a62410, 0x70018593f8, 0x70018593f0, 0x70018593e8> as *const (), 0), // +0x81b5f0 sub_81b5f0
    Lifted::new(0x7000f07670, fn_7000f07670 as *const (), 1), // +0x81b670 sub_81b670
    Lifted::new(0x7000f07680, fn_7000f07680 as *const (), 1), // +0x81b680 sub_81b680
    Lifted::new(0x7000f076a0, fn_7000f076a0 as *const (), 1), // +0x81b6a0 sub_81b6a0
    Lifted::new(0x7000f076c0, fn_7000f076c0 as *const (), 1), // +0x81b6c0 sub_81b6c0
    Lifted::new(0x7000f077a0, fn_7000f077a0 as *const (), 1), // +0x81b7a0 sub_81b7a0
    Lifted::new(0x7000f077d0, fn_7000f077d0 as *const (), 1), // +0x81b7d0 sub_81b7d0
    Lifted::new(0x7000f07810, fn_7000f07810 as *const (), 1), // +0x81b810 sub_81b810
    Lifted::new(0x7000f07840, fn_7000f07840 as *const (), 1), // +0x81b840 sub_81b840
    Lifted::new(0x7000f07870, fn_7000f07870 as *const (), 1), // +0x81b870 sub_81b870
    Lifted::new(0x7000f078a0, fn_7000f078a0 as *const (), 1), // +0x81b8a0 sub_81b8a0
    Lifted::new(0x7000f078f0, fn_7000f078f0 as *const (), 1), // +0x81b8f0 sub_81b8f0
    Lifted::new(0x7000f07930, fn_7000f07930 as *const (), 0), // +0x81b930 sub_81b930
    Lifted::new(0x7000f07c20, fn_7000f07c20 as *const (), 2), // +0x81bc20 sub_81bc20
    Lifted::new(0x7000f08030, fn_7000f08030 as *const (), 0), // +0x81c030 sub_81c030
    Lifted::new(0x7000f08060, fn_7000f08060 as *const (), 0), // +0x81c060 sub_81c060
    Lifted::new(0x7000f080e0, fn_7000f080e0 as *const (), 0), // +0x81c0e0 sub_81c0e0
    Lifted::new(0x7000f08240, tpl_949::<0x7000f07c20> as *const (), 0), // +0x81c240 sub_81c240
    Lifted::new(0x7000f08260, tpl_1320::<0x160> as *const (), 1), // +0x81c260 sub_81c260
    Lifted::new(0x7000f08280, tpl_1652::<0x7000f09c20, 0x8, 0x3> as *const (), 0), // +0x81c280 sub_81c280
    Lifted::new(0x7000f082c0, fn_7000f082c0 as *const (), 0), // +0x81c2c0 sub_81c2c0
    Lifted::new(0x7000f08390, fn_7000f08390 as *const (), 0), // +0x81c390 sub_81c390
    Lifted::new(0x7000f08570, fn_7000f08570 as *const (), 0), // +0x81c570 sub_81c570
    Lifted::new(0x7000f08670, fn_7000f08670 as *const (), 0), // +0x81c670 sub_81c670
    Lifted::new(0x7000f086b0, fn_7000f086b0 as *const (), 1), // +0x81c6b0 sub_81c6b0
    Lifted::new(0x7000f08720, fn_7000f08720 as *const (), 1), // +0x81c720 sub_81c720
    Lifted::new(0x7000f08810, fn_7000f08810 as *const (), 1), // +0x81c810 sub_81c810
    Lifted::new(0x7000f08830, fn_7000f08830 as *const (), 1), // +0x81c830 sub_81c830
    Lifted::new(0x7000f08850, fn_7000f08850 as *const (), 1), // +0x81c850 sub_81c850
    Lifted::new(0x7000f08870, fn_7000f08870 as *const (), 1), // +0x81c870 sub_81c870
    Lifted::new(0x7000f088b0, fn_7000f088b0 as *const (), 1), // +0x81c8b0 sub_81c8b0
    Lifted::new(0x7000f088d0, fn_7000f088d0 as *const (), 1), // +0x81c8d0 sub_81c8d0
    Lifted::new(0x7000f08920, fn_7000f08920 as *const (), 1), // +0x81c920 sub_81c920
    Lifted::new(0x7000f08940, fn_7000f08940 as *const (), 0), // +0x81c940 sub_81c940
    Lifted::new(0x7000f089d0, fn_7000f089d0 as *const (), 0), // +0x81c9d0 sub_81c9d0
    Lifted::new(0x7000f08a40, tpl_1653::<0x7000f09c20, 0x8> as *const (), 0), // +0x81ca40 sub_81ca40
    Lifted::new(0x7000f08a70, fn_7000f08a70 as *const (), 1), // +0x81ca70 sub_81ca70
    Lifted::new(0x7000f08a90, fn_7000f08a90 as *const (), 1), // +0x81ca90 sub_81ca90
    Lifted::new(0x7000f08ab0, fn_7000f08ab0 as *const (), 1), // +0x81cab0 sub_81cab0
    Lifted::new(0x7000f08ad0, fn_7000f08ad0 as *const (), 0), // +0x81cad0 sub_81cad0
    Lifted::new(0x7000f08b90, fn_7000f08b90 as *const (), 0), // +0x81cb90 sub_81cb90
    Lifted::new(0x7000f08c10, fn_7000f08c10 as *const (), 0), // +0x81cc10 sub_81cc10
    Lifted::new(0x7000f08c80, fn_7000f08c80 as *const (), 0), // +0x81cc80 sub_81cc80
    Lifted::new(0x7000f08cf0, fn_7000f08cf0 as *const (), 0), // +0x81ccf0 sub_81ccf0
    Lifted::new(0x7000f08ec0, tpl_1654::<0x7000f08ee0> as *const (), 0), // +0x81cec0 sub_81cec0
    Lifted::new(0x7000f08ee0, fn_7000f08ee0 as *const (), 0), // +0x81cee0 sub_81cee0
    Lifted::new(0x7000f08f60, fn_7000f08f60 as *const (), 0), // +0x81cf60 sub_81cf60
    Lifted::new(0x7000f090b0, fn_7000f090b0 as *const (), 0), // +0x81d0b0 sub_81d0b0
    Lifted::new(0x7000f090e0, tpl_1654::<0x7000f09100> as *const (), 0), // +0x81d0e0 sub_81d0e0
    Lifted::new(0x7000f09100, fn_7000f09100 as *const (), 0), // +0x81d100 sub_81d100
    Lifted::new(0x7000f091a0, tpl_1653::<0x7000bfffa0, 0x8> as *const (), 0), // +0x81d1a0 sub_81d1a0
    Lifted::new(0x7000f091d0, fn_7000f091d0 as *const (), 1), // +0x81d1d0 sub_81d1d0
    Lifted::new(0x7000f091e0, fn_7000f091e0 as *const (), 0), // +0x81d1e0 sub_81d1e0
    Lifted::new(0x7000f09280, fn_7000f09280 as *const (), 0), // +0x81d280 sub_81d280
    Lifted::new(0x7000f092f0, fn_7000f092f0 as *const (), 0), // +0x81d2f0 sub_81d2f0
    Lifted::new(0x7000f09330, fn_7000f09330 as *const (), 1), // +0x81d330 sub_81d330
    Lifted::new(0x7000f09350, fn_7000f09350 as *const (), 1), // +0x81d350 sub_81d350
    Lifted::new(0x7000f093a0, fn_7000f093a0 as *const (), 1), // +0x81d3a0 sub_81d3a0
    Lifted::new(0x7000f09440, fn_7000f09440 as *const (), 0), // +0x81d440 sub_81d440
    Lifted::new(0x7000f09790, fn_7000f09790 as *const (), 1), // +0x81d790 sub_81d790
    Lifted::new(0x7000f097d0, fn_7000f097d0 as *const (), 1), // +0x81d7d0 sub_81d7d0
    Lifted::new(0x7000f09810, fn_7000f09810 as *const (), 1), // +0x81d810 sub_81d810
    Lifted::new(0x7000f098b0, fn_7000f098b0 as *const (), 1), // +0x81d8b0 sub_81d8b0
    Lifted::new(0x7000f09920, fn_7000f09920 as *const (), 0), // +0x81d920 sub_81d920
    Lifted::new(0x7000f09a00, fn_7000f09a00 as *const (), 0), // +0x81da00 sub_81da00
    Lifted::new(0x7000f09ad0, fn_7000f09ad0 as *const (), 0), // +0x81dad0 sub_81dad0
    Lifted::new(0x7000f09b20, fn_7000f09b20 as *const (), 0), // +0x81db20 sub_81db20
    Lifted::new(0x7000f09bc0, tpl_1348::<0x3> as *const (), 1), // +0x81dbc0 sub_81dbc0
    Lifted::new(0x7000f09be0, tpl_1652::<0x7000f09c20, 0x8, 0xffffffffffffffff> as *const (), 0), // +0x81dbe0 sub_81dbe0
    Lifted::new(0x7000f09c20, fn_7000f09c20 as *const (), 0), // +0x81dc20 sub_81dc20
    Lifted::new(0x7000f09d90, fn_7000f09d90 as *const (), 0), // +0x81dd90 sub_81dd90
    Lifted::new(0x7000f09e50, fn_7000f09e50 as *const (), 0), // +0x81de50 sub_81de50
    Lifted::new(0x7000f0a010, fn_7000f0a010 as *const (), 1), // +0x81e010 sub_81e010
    Lifted::new(0x7000f0a030, fn_7000f0a030 as *const (), 1), // +0x81e030 sub_81e030
    Lifted::new(0x7000f0a090, fn_7000f0a090 as *const (), 1), // +0x81e090 sub_81e090
    Lifted::new(0x7000f0a100, fn_7000f0a100 as *const (), 1), // +0x81e100 sub_81e100
    Lifted::new(0x7000f0a110, tpl_913::<0x3> as *const (), 1), // +0x81e110 sub_81e110
    Lifted::new(0x7000f0a130, fn_7000f0a130 as *const (), 1), // +0x81e130 sub_81e130
    Lifted::new(0x7000f0a160, fn_7000f0a160 as *const (), 1), // +0x81e160 sub_81e160
    Lifted::new(0x7000f0a180, fn_7000f0a180 as *const (), 0), // +0x81e180 sub_81e180
    Lifted::new(0x7000f0a1c0, fn_7000f0a1c0 as *const (), 1), // +0x81e1c0 sub_81e1c0
    Lifted::new(0x7000f0a1e0, fn_7000f0a1e0 as *const (), 1), // +0x81e1e0 sub_81e1e0
    Lifted::new(0x7000f0a1f0, fn_7000f0a1f0 as *const (), 1), // +0x81e1f0 sub_81e1f0
    Lifted::new(0x7000f0a200, fn_7000f0a200 as *const (), 1), // +0x81e200 sub_81e200
    Lifted::new(0x7000f0a210, fn_7000f0a210 as *const (), 0), // +0x81e210 sub_81e210
    Lifted::new(0x7000f0a300, tpl_1574::<0x4, 0xffffffffffffffff> as *const (), 1), // +0x81e300 sub_81e300
    Lifted::new(0x7000f0a320, fn_7000f0a320 as *const (), 1), // +0x81e320 sub_81e320
    Lifted::new(0x7000f0a340, fn_7000f0a340 as *const (), 2), // +0x81e340 sub_81e340
    Lifted::new(0x7000f0a880, fn_7000f0a880 as *const (), 2), // +0x81e880 sub_81e880
    Lifted::new(0x7000f0ad70, fn_7000f0ad70 as *const (), 0), // +0x81ed70 sub_81ed70
    Lifted::new(0x7000f0ae60, fn_7000f0ae60 as *const (), 0), // +0x81ee60 sub_81ee60
    Lifted::new(0x7000f0b050, tpl_1655::<0xf, 0xffffffffffffffff, 0x7, 0x7000dc3c00> as *const (), 0), // +0x81f050 sub_81f050
    Lifted::new(0x7000f0b0d0, fn_7000f0b0d0 as *const (), 0), // +0x81f0d0 sub_81f0d0
    Lifted::new(0x7000f0b3d0, fn_7000f0b3d0 as *const (), 0), // +0x81f3d0 sub_81f3d0
    Lifted::new(0x7000f0b4f0, fn_7000f0b4f0 as *const (), 0), // +0x81f4f0 sub_81f4f0
    Lifted::new(0x7000f0b780, fn_7000f0b780 as *const (), 0), // +0x81f780 sub_81f780
    Lifted::new(0x7000f0b9d0, fn_7000f0b9d0 as *const (), 0), // +0x81f9d0 sub_81f9d0
    Lifted::new(0x7000f0bda0, fn_7000f0bda0 as *const (), 0), // +0x81fda0 sub_81fda0
    Lifted::new(0x7000f0bf80, fn_7000f0bf80 as *const (), 0), // +0x81ff80 sub_81ff80
    Lifted::new(0x7000f0c210, fn_7000f0c210 as *const (), 0), // +0x820210 sub_820210
    Lifted::new(0x7000f0c560, fn_7000f0c560 as *const (), 0), // +0x820560 sub_820560
    Lifted::new(0x7000f0c6f0, fn_7000f0c6f0 as *const (), 0), // +0x8206f0 sub_8206f0
    Lifted::new(0x7000f0ced0, tpl_1655::<0xf, 0xffffffffffffffff, 0x7, 0x7000dc4e30> as *const (), 0), // +0x820ed0 sub_820ed0
    Lifted::new(0x7000f0cf50, tpl_1655::<0xf, 0xffffffffffffffff, 0x7, 0x7000dc5000> as *const (), 0), // +0x820f50 sub_820f50
    Lifted::new(0x7000f0cfd0, fn_7000f0cfd0 as *const (), 1), // +0x820fd0 sub_820fd0
    Lifted::new(0x7000f0d0e0, fn_7000f0d0e0 as *const (), 1), // +0x8210e0 sub_8210e0
    Lifted::new(0x7000f0d130, fn_7000f0d130 as *const (), 0), // +0x821130 sub_821130
    Lifted::new(0x7000f0d230, fn_7000f0d230 as *const (), 0), // +0x821230 sub_821230
    Lifted::new(0x7000f0d2c0, fn_7000f0d2c0 as *const (), 2), // +0x8212c0 sub_8212c0
    Lifted::new(0x7000f0d5f0, fn_7000f0d5f0 as *const (), 1), // +0x8215f0 sub_8215f0
    Lifted::new(0x7000f0d6e0, fn_7000f0d6e0 as *const (), 0), // +0x8216e0 sub_8216e0
    Lifted::new(0x7000f0d770, fn_7000f0d770 as *const (), 0), // +0x821770 sub_821770
    Lifted::new(0x7000f0d840, fn_7000f0d840 as *const (), 0), // +0x821840 sub_821840
    Lifted::new(0x7000f0d9a0, fn_7000f0d9a0 as *const (), 0), // +0x8219a0 sub_8219a0
    Lifted::new(0x7000f0dbd0, fn_7000f0dbd0 as *const (), 1), // +0x821bd0 sub_821bd0
    Lifted::new(0x7000f0dc00, fn_7000f0dc00 as *const (), 0), // +0x821c00 sub_821c00
    Lifted::new(0x7000f0dd70, fn_7000f0dd70 as *const (), 0), // +0x821d70 sub_821d70
    Lifted::new(0x7000f0e0c0, fn_7000f0e0c0 as *const (), 0), // +0x8220c0 sub_8220c0
    Lifted::new(0x7000f0e100, fn_7000f0e100 as *const (), 0), // +0x822100 sub_822100
    Lifted::new(0x7000f0e2e0, fn_7000f0e2e0 as *const (), 2), // +0x8222e0 sub_8222e0
    Lifted::new(0x7000f0e4a0, fn_7000f0e4a0 as *const (), 1), // +0x8224a0 sub_8224a0
    Lifted::new(0x7000f0e580, tpl_923::<0x70018594d8> as *const (), 1), // +0x822580 sub_822580
    Lifted::new(0x7000f0e5a0, tpl_754::<0x700177c312> as *const (), 0), // +0x8225a0 sub_8225a0
    Lifted::new(0x7000f0e5c0, tpl_15 as *const (), 1), // +0x8225c0 sub_8225c0
    Lifted::new(0x7000f0e5d0, tpl_52::<0x8> as *const (), 1), // +0x8225d0 sub_8225d0
    Lifted::new(0x7000f0e5e0, tpl_923::<0x70018594d8> as *const (), 1), // +0x8225e0 sub_8225e0
    Lifted::new(0x7000f0e600, tpl_315::<0x7001c6c410> as *const (), 1), // +0x822600 sub_822600
    Lifted::new(0x7000f0e610, tpl_237::<0x3> as *const (), 1), // +0x822610 sub_822610
    Lifted::new(0x7000f0e640, tpl_237::<0x3> as *const (), 1), // +0x822640 sub_822640
    Lifted::new(0x7000f0e670, tpl_237::<0x3> as *const (), 1), // +0x822670 sub_822670
    Lifted::new(0x7000f0e6a0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8226a0 sub_8226a0
    Lifted::new(0x7000f0e6e0, tpl_569::<0x19, 0x10, 0x7000f0e6e0> as *const (), 0), // +0x8226e0 sub_8226e0
    Lifted::new(0x7000f0e740, tpl_1656::<0x7000770a40> as *const (), 0), // +0x822740 sub_822740
    Lifted::new(0x7000f0e870, tpl_1656::<0x7000770a40> as *const (), 0), // +0x822870 sub_822870
    Lifted::new(0x7000f0e9a0, fn_7000f0e9a0 as *const (), 1), // +0x8229a0 sub_8229a0
    Lifted::new(0x7000f0ea40, tpl_898::<0x700177c0e0> as *const (), 0), // +0x822a40 sub_822a40
    Lifted::new(0x7000f0ea80, tpl_392::<0x700177c0e0> as *const (), 0), // +0x822a80 sub_822a80
    Lifted::new(0x7000f0eab0, fn_7000f0eab0 as *const (), 2), // +0x822ab0 sub_822ab0
    Lifted::new(0x7000f0ed80, fn_7000f0ed80 as *const (), 2), // +0x822d80 sub_822d80
    Lifted::new(0x7000f0f0d0, fn_7000f0f0d0 as *const (), 2), // +0x8230d0 sub_8230d0
    Lifted::new(0x7000f0f3b0, fn_7000f0f3b0 as *const (), 2), // +0x8233b0 sub_8233b0
    Lifted::new(0x7000f0f700, tpl_68 as *const (), 1), // +0x823700 sub_823700
    Lifted::new(0x7000f0f7b0, tpl_68 as *const (), 1), // +0x8237b0 sub_8237b0
    Lifted::new(0x7000f0f860, tpl_68 as *const (), 1), // +0x823860 sub_823860
    Lifted::new(0x7000f0f910, tpl_55::<0x700177c0d0> as *const (), 0), // +0x823910 sub_823910
    Lifted::new(0x7000f0f9e0, fn_7000f0f9e0 as *const (), 2), // +0x8239e0 sub_8239e0
    Lifted::new(0x7000f0fb50, fn_7000f0fb50 as *const (), 0), // +0x823b50 sub_823b50
    Lifted::new(0x7000f10030, fn_7000f10030 as *const (), 0), // +0x824030 sub_824030
    Lifted::new(0x7000f10280, fn_7000f10280 as *const (), 2), // +0x824280 sub_824280
    Lifted::new(0x7000f10550, fn_7000f10550 as *const (), 0), // +0x824550 sub_824550
    Lifted::new(0x7000f10690, fn_7000f10690 as *const (), 0), // +0x824690 sub_824690
    Lifted::new(0x7000f10800, fn_7000f10800 as *const (), 0), // +0x824800 sub_824800
    Lifted::new(0x7000f10b70, fn_7000f10b70 as *const (), 2), // +0x824b70 sub_824b70
    Lifted::new(0x7000f11020, fn_7000f11020 as *const (), 0), // +0x825020 sub_825020
    Lifted::new(0x7000f11280, fn_7000f11280 as *const (), 2), // +0x825280 sub_825280
    Lifted::new(0x7000f11c50, fn_7000f11c50 as *const (), 2), // +0x825c50 sub_825c50
    Lifted::new(0x7000f12920, fn_7000f12920 as *const (), 2), // +0x826920 sub_826920
    Lifted::new(0x7000f130b0, fn_7000f130b0 as *const (), 0), // +0x8270b0 sub_8270b0
    Lifted::new(0x7000f131b0, fn_7000f131b0 as *const (), 0), // +0x8271b0 sub_8271b0
    Lifted::new(0x7000f13280, fn_7000f13280 as *const (), 0), // +0x827280 sub_827280
    Lifted::new(0x7000f13420, fn_7000f13420 as *const (), 0), // +0x827420 sub_827420
    Lifted::new(0x7000f134f0, fn_7000f134f0 as *const (), 0), // +0x8274f0 sub_8274f0
    Lifted::new(0x7000f13620, fn_7000f13620 as *const (), 0), // +0x827620 sub_827620
    Lifted::new(0x7000f13730, fn_7000f13730 as *const (), 0), // +0x827730 sub_827730
    Lifted::new(0x7000f13990, fn_7000f13990 as *const (), 2), // +0x827990 sub_827990
    Lifted::new(0x7000f14170, fn_7000f14170 as *const (), 0), // +0x828170 sub_828170
    Lifted::new(0x7000f142c0, fn_7000f142c0 as *const (), 2), // +0x8282c0 sub_8282c0
    Lifted::new(0x7000f14440, fn_7000f14440 as *const (), 2), // +0x828440 sub_828440
    Lifted::new(0x7000f14f10, fn_7000f14f10 as *const (), 2), // +0x828f10 sub_828f10
    Lifted::new(0x7000f152c0, fn_7000f152c0 as *const (), 2), // +0x8292c0 sub_8292c0
    Lifted::new(0x7000f15990, fn_7000f15990 as *const (), 2), // +0x829990 sub_829990
    Lifted::new(0x7000f16080, fn_7000f16080 as *const (), 2), // +0x82a080 sub_82a080
    Lifted::new(0x7000f16400, fn_7000f16400 as *const (), 2), // +0x82a400 sub_82a400
    Lifted::new(0x7000f16790, fn_7000f16790 as *const (), 0), // +0x82a790 sub_82a790
    Lifted::new(0x7000f16900, tpl_1585::<0x10, 0x8, 0x18, 0x700076bf20, 0x7000f0f700> as *const (), 0), // +0x82a900 sub_82a900
    Lifted::new(0x7000f169f0, tpl_1585::<0x10, 0x8, 0x18, 0x700076bf20, 0x7000f0f7b0> as *const (), 0), // +0x82a9f0 sub_82a9f0
    Lifted::new(0x7000f16ae0, tpl_1585::<0x10, 0x8, 0x18, 0x700076bf20, 0x7000f0f860> as *const (), 0), // +0x82aae0 sub_82aae0
    Lifted::new(0x7000f16bd0, tpl_1586::<0x7000f169f0, 0x70017d37a0, 0x70017790e0, 0x7000f16d45> as *const (), 0), // +0x82abd0 sub_82abd0
    Lifted::new(0x7000f16db0, tpl_1657::<0x7000f16ae0, 0x700079a530, 0x7000770a40, 0x70017d37a0, 0x70017790e0, 0x7000f17018> as *const (), 0), // +0x82adb0 sub_82adb0
    Lifted::new(0x7000f170c0, tpl_1657::<0x7000f16900, 0x700079a530, 0x7000770a40, 0x70017d37a0, 0x70017790e0, 0x7000f17328> as *const (), 0), // +0x82b0c0 sub_82b0c0
    Lifted::new(0x7000f173d0, tpl_1295::<0x700076bf20, 0x700177c0d0, 0x700177c0e0, 0x7000f178e0, 0x7000f17557> as *const (), 0), // +0x82b3d0 sub_82b3d0
    Lifted::new(0x7000f175c0, tpl_287 as *const (), 1), // +0x82b5c0 sub_82b5c0
    Lifted::new(0x7000f178e0, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000f178f0> as *const (), 0), // +0x82b8e0 sub_82b8e0
    Lifted::new(0x7000f17900, fn_7000f17900 as *const (), 0), // +0x82b900 sub_82b900
    Lifted::new(0x7000f179d0, tpl_1658::<0x7000f173d0, 0x700076bf20> as *const (), 0), // +0x82b9d0 sub_82b9d0
    Lifted::new(0x7000f17b20, fn_7000f17b20 as *const (), 0), // +0x82bb20 sub_82bb20
    Lifted::new(0x7000f17b90, fn_7000f17b90 as *const (), 1), // +0x82bb90 sub_82bb90
    Lifted::new(0x7000f17d10, fn_7000f17d10 as *const (), 1), // +0x82bd10 sub_82bd10
    Lifted::new(0x7000f17e90, fn_7000f17e90 as *const (), 0), // +0x82be90 sub_82be90
    Lifted::new(0x7000f186a0, fn_7000f186a0 as *const (), 0), // +0x82c6a0 sub_82c6a0
    Lifted::new(0x7000f18cd0, fn_7000f18cd0 as *const (), 2), // +0x82ccd0 sub_82ccd0
    Lifted::new(0x7000f19210, fn_7000f19210 as *const (), 0), // +0x82d210 sub_82d210
    Lifted::new(0x7000f193a0, fn_7000f193a0 as *const (), 2), // +0x82d3a0 sub_82d3a0
    Lifted::new(0x7000f1a020, fn_7000f1a020 as *const (), 0), // +0x82e020 sub_82e020
    Lifted::new(0x7000f1a170, fn_7000f1a170 as *const (), 0), // +0x82e170 sub_82e170
    Lifted::new(0x7000f1a330, fn_7000f1a330 as *const (), 0), // +0x82e330 sub_82e330
    Lifted::new(0x7000f1a4b0, fn_7000f1a4b0 as *const (), 0), // +0x82e4b0 sub_82e4b0
    Lifted::new(0x7000f1a570, fn_7000f1a570 as *const (), 0), // +0x82e570 sub_82e570
    Lifted::new(0x7000f1a7d0, fn_7000f1a7d0 as *const (), 0), // +0x82e7d0 sub_82e7d0
    Lifted::new(0x7000f1a8d0, fn_7000f1a8d0 as *const (), 0), // +0x82e8d0 sub_82e8d0
    Lifted::new(0x7000f1b040, fn_7000f1b040 as *const (), 1), // +0x82f040 sub_82f040
    Lifted::new(0x7000f1b120, fn_7000f1b120 as *const (), 0), // +0x82f120 sub_82f120
    Lifted::new(0x7000f1b290, fn_7000f1b290 as *const (), 0), // +0x82f290 sub_82f290
    Lifted::new(0x7000f1b2d0, tpl_1659::<0x7000f1b040, 0x7, 0x10, 0xffffffffffff6948> as *const (), 0), // +0x82f2d0 sub_82f2d0
    Lifted::new(0x7000f1b360, fn_7000f1b360 as *const (), 0), // +0x82f360 sub_82f360
    Lifted::new(0x7000f1b3e0, fn_7000f1b3e0 as *const (), 0), // +0x82f3e0 sub_82f3e0
    Lifted::new(0x7000f1b4f0, tpl_1659::<0x7000f1b040, 0x37, 0x10, 0xffffffffffff6948> as *const (), 0), // +0x82f4f0 sub_82f4f0
    Lifted::new(0x7000f1b5c0, fn_7000f1b5c0 as *const (), 0), // +0x82f5c0 sub_82f5c0
    Lifted::new(0x7000f1b780, fn_7000f1b780 as *const (), 0), // +0x82f780 sub_82f780
    Lifted::new(0x7000f1b940, tpl_146::<0x7> as *const (), 1), // +0x82f940 sub_82f940
    Lifted::new(0x7000f1b950, tpl_334::<0x3> as *const (), 1), // +0x82f950 sub_82f950
    Lifted::new(0x7000f1b980, fn_7000f1b980 as *const (), 0), // +0x82f980 sub_82f980
    Lifted::new(0x7000f1ba90, fn_7000f1ba90 as *const (), 1), // +0x82fa90 sub_82fa90
    Lifted::new(0x7000f1baf0, fn_7000f1baf0 as *const (), 0), // +0x82faf0 sub_82faf0
    Lifted::new(0x7000f1be40, fn_7000f1be40 as *const (), 0), // +0x82fe40 sub_82fe40
    Lifted::new(0x7000f1c170, fn_7000f1c170 as *const (), 0), // +0x830170 sub_830170
    Lifted::new(0x7000f1c1b0, fn_7000f1c1b0 as *const (), 1), // +0x8301b0 sub_8301b0
    Lifted::new(0x7000f1c290, fn_7000f1c290 as *const (), 0), // +0x830290 sub_830290
    Lifted::new(0x7000f1c2d0, fn_7000f1c2d0 as *const (), 1), // +0x8302d0 sub_8302d0
    Lifted::new(0x7000f1c320, tpl_1660::<0x13, 0x7, 0xf, 0xffffffffffffffff, 0x3a7> as *const (), 1), // +0x830320 sub_830320
    Lifted::new(0x7000f1c390, tpl_1660::<0x13, 0x7, 0xf, 0xffffffffffffffff, 0x3ab> as *const (), 1), // +0x830390 sub_830390
    Lifted::new(0x7000f1c400, fn_7000f1c400 as *const (), 1), // +0x830400 sub_830400
    Lifted::new(0x7000f1c4c0, fn_7000f1c4c0 as *const (), 0), // +0x8304c0 sub_8304c0
    Lifted::new(0x7000f1c590, fn_7000f1c590 as *const (), 0), // +0x830590 sub_830590
    Lifted::new(0x7000f1ce40, fn_7000f1ce40 as *const (), 0), // +0x830e40 sub_830e40
    Lifted::new(0x7000f1cee0, fn_7000f1cee0 as *const (), 0), // +0x830ee0 sub_830ee0
    Lifted::new(0x7000f1cf10, fn_7000f1cf10 as *const (), 0), // +0x830f10 sub_830f10
    Lifted::new(0x7000f1d9a0, fn_7000f1d9a0 as *const (), 0), // +0x8319a0 sub_8319a0
    Lifted::new(0x7000f1dbc0, fn_7000f1dbc0 as *const (), 0), // +0x831bc0 sub_831bc0
    Lifted::new(0x7000f1de00, fn_7000f1de00 as *const (), 0), // +0x831e00 sub_831e00
    Lifted::new(0x7000f1dec0, fn_7000f1dec0 as *const (), 0), // +0x831ec0 sub_831ec0
    Lifted::new(0x7000f1e030, fn_7000f1e030 as *const (), 0), // +0x832030 sub_832030
    Lifted::new(0x7000f1e0f0, tpl_1661::<0x10, 0x38, 0x7000f1fcc0> as *const (), 0), // +0x8320f0 sub_8320f0
    Lifted::new(0x7000f1e1c0, tpl_1661::<0x10, 0x38, 0x7000f1fd80> as *const (), 0), // +0x8321c0 sub_8321c0
    Lifted::new(0x7000f1e290, tpl_1662::<0x7000f1f420, 0x7001d550a8> as *const (), 2), // +0x832290 sub_832290
    Lifted::new(0x7000f1e3e0, tpl_1662::<0x7000f1f870, 0x7001d550a8> as *const (), 2), // +0x8323e0 sub_8323e0
    Lifted::new(0x7000f1e530, fn_7000f1e530 as *const (), 0), // +0x832530 sub_832530
    Lifted::new(0x7000f1e620, fn_7000f1e620 as *const (), 0), // +0x832620 sub_832620
    Lifted::new(0x7000f1e710, tpl_1663::<0x10, 0x90, 0x38, 0x7000f1fe50> as *const (), 0), // +0x832710 sub_832710
    Lifted::new(0x7000f1e7e0, tpl_1663::<0x10, 0x90, 0x38, 0x7000f1ff40> as *const (), 0), // +0x8327e0 sub_8327e0
    Lifted::new(0x7000f1e8b0, fn_7000f1e8b0 as *const (), 0), // +0x8328b0 sub_8328b0
    Lifted::new(0x7000f1e9a0, tpl_1664::<0x10, 0x2, 0x38, 0x7000f1fcc0> as *const (), 0), // +0x8329a0 sub_8329a0
    Lifted::new(0x7000f1eaa0, tpl_1664::<0x10, 0x2, 0x38, 0x7000f1fd80> as *const (), 0), // +0x832aa0 sub_832aa0
    Lifted::new(0x7000f1eba0, tpl_1665::<0x7000f1f420, 0x7001d550a8> as *const (), 2), // +0x832ba0 sub_832ba0
    Lifted::new(0x7000f1ed30, tpl_1665::<0x7000f1f870, 0x7001d550a8> as *const (), 2), // +0x832d30 sub_832d30
    Lifted::new(0x7000f1eec0, fn_7000f1eec0 as *const (), 0), // +0x832ec0 sub_832ec0
    Lifted::new(0x7000f1efe0, fn_7000f1efe0 as *const (), 0), // +0x832fe0 sub_832fe0
    Lifted::new(0x7000f1f100, tpl_1666::<0x10, 0x2, 0x90, 0x38, 0x7000f1fe50> as *const (), 0), // +0x833100 sub_833100
    Lifted::new(0x7000f1f200, tpl_1666::<0x10, 0x2, 0x90, 0x38, 0x7000f1ff40> as *const (), 0), // +0x833200 sub_833200
    Lifted::new(0x7000f1f300, fn_7000f1f300 as *const (), 0), // +0x833300 sub_833300
    Lifted::new(0x7000f1f420, fn_7000f1f420 as *const (), 2), // +0x833420 sub_833420
    Lifted::new(0x7000f1f870, fn_7000f1f870 as *const (), 2), // +0x833870 sub_833870
    Lifted::new(0x7000f1fcc0, fn_7000f1fcc0 as *const (), 2), // +0x833cc0 sub_833cc0
    Lifted::new(0x7000f1fd80, fn_7000f1fd80 as *const (), 2), // +0x833d80 sub_833d80
    Lifted::new(0x7000f1fe50, tpl_1667::<0x8, 0x3, 0xc, 0x7, 0x10, 0x90, 0x7001d550b8, 0x38, 0x7000f1f420, 0x7001d550a8> as *const (), 0), // +0x833e50 sub_833e50
    Lifted::new(0x7000f1ff40, tpl_1667::<0x8, 0x3, 0xc, 0x7, 0x10, 0x90, 0x7001d550b8, 0x38, 0x7000f1f870, 0x7001d550a8> as *const (), 0), // +0x833f40 sub_833f40
    Lifted::new(0x7000f20030, fn_7000f20030 as *const (), 0), // +0x834030 sub_834030
    Lifted::new(0x7000f20330, fn_7000f20330 as *const (), 0), // +0x834330 sub_834330
    Lifted::new(0x7000f206c0, fn_7000f206c0 as *const (), 2), // +0x8346c0 sub_8346c0
    Lifted::new(0x7000f20a60, fn_7000f20a60 as *const (), 2), // +0x834a60 sub_834a60
    Lifted::new(0x7000f20db0, fn_7000f20db0 as *const (), 2), // +0x834db0 sub_834db0
    Lifted::new(0x7000f21160, fn_7000f21160 as *const (), 2), // +0x835160 sub_835160
    Lifted::new(0x7000f214b0, fn_7000f214b0 as *const (), 0), // +0x8354b0 sub_8354b0
    Lifted::new(0x7000f218f0, fn_7000f218f0 as *const (), 0), // +0x8358f0 sub_8358f0
    Lifted::new(0x7000f21c00, fn_7000f21c00 as *const (), 0), // +0x835c00 sub_835c00
    Lifted::new(0x7000f21f20, fn_7000f21f20 as *const (), 2), // +0x835f20 sub_835f20
    Lifted::new(0x7000f22160, fn_7000f22160 as *const (), 2), // +0x836160 sub_836160
    Lifted::new(0x7000f223e0, fn_7000f223e0 as *const (), 0), // +0x8363e0 sub_8363e0
    Lifted::new(0x7000f22430, fn_7000f22430 as *const (), 0), // +0x836430 sub_836430
    Lifted::new(0x7000f224b0, tpl_408::<0x7000f224b0, 0x700078d950> as *const (), 0), // +0x8364b0 sub_8364b0
    Lifted::new(0x7000f224f0, fn_7000f224f0 as *const (), 0), // +0x8364f0 sub_8364f0
    Lifted::new(0x7000f22570, fn_7000f22570 as *const (), 0), // +0x836570 sub_836570
    Lifted::new(0x7000f225b0, tpl_33::<0x700177c312> as *const (), 0), // +0x8365b0 sub_8365b0
    Lifted::new(0x7000f225e0, tpl_147::<0x10> as *const (), 1), // +0x8365e0 sub_8365e0
    Lifted::new(0x7000f225f0, fn_7000f225f0 as *const (), 0), // +0x8365f0 sub_8365f0
    Lifted::new(0x7000f22640, fn_7000f22640 as *const (), 0), // +0x836640 sub_836640
    Lifted::new(0x7000f22770, fn_7000f22770 as *const (), 1), // +0x836770 sub_836770
    Lifted::new(0x7000f227d0, fn_7000f227d0 as *const (), 0), // +0x8367d0 sub_8367d0
    Lifted::new(0x7000f22820, fn_7000f22820 as *const (), 0), // +0x836820 sub_836820
    Lifted::new(0x7000f22870, fn_7000f22870 as *const (), 0), // +0x836870 sub_836870
    Lifted::new(0x7000f228b0, fn_7000f228b0 as *const (), 0), // +0x8368b0 sub_8368b0
    Lifted::new(0x7000f228f0, fn_7000f228f0 as *const (), 0), // +0x8368f0 sub_8368f0
    Lifted::new(0x7000f22a40, fn_7000f22a40 as *const (), 0), // +0x836a40 sub_836a40
    Lifted::new(0x7000f22ae0, fn_7000f22ae0 as *const (), 1), // +0x836ae0 sub_836ae0
    Lifted::new(0x7000f22af0, fn_7000f22af0 as *const (), 0), // +0x836af0 sub_836af0
    Lifted::new(0x7000f22b40, fn_7000f22b40 as *const (), 0), // +0x836b40 sub_836b40
    Lifted::new(0x7000f22cd0, fn_7000f22cd0 as *const (), 0), // +0x836cd0 sub_836cd0
    Lifted::new(0x7000f22e60, fn_7000f22e60 as *const (), 0), // +0x836e60 sub_836e60
    Lifted::new(0x7000f23010, fn_7000f23010 as *const (), 0), // +0x837010 sub_837010
    Lifted::new(0x7000f23170, fn_7000f23170 as *const (), 2), // +0x837170 sub_837170
    Lifted::new(0x7000f233c0, fn_7000f233c0 as *const (), 2), // +0x8373c0 sub_8373c0
    Lifted::new(0x7000f235f0, fn_7000f235f0 as *const (), 0), // +0x8375f0 sub_8375f0
    Lifted::new(0x7000f23730, fn_7000f23730 as *const (), 0), // +0x837730 sub_837730
    Lifted::new(0x7000f238b0, fn_7000f238b0 as *const (), 0), // +0x8378b0 sub_8378b0
    Lifted::new(0x7000f23a20, fn_7000f23a20 as *const (), 0), // +0x837a20 sub_837a20
    Lifted::new(0x7000f23bb0, fn_7000f23bb0 as *const (), 2), // +0x837bb0 sub_837bb0
    Lifted::new(0x7000f23e60, fn_7000f23e60 as *const (), 2), // +0x837e60 sub_837e60
    Lifted::new(0x7000f24090, fn_7000f24090 as *const (), 2), // +0x838090 sub_838090
    Lifted::new(0x7000f242b0, fn_7000f242b0 as *const (), 2), // +0x8382b0 sub_8382b0
    Lifted::new(0x7000f244e0, fn_7000f244e0 as *const (), 0), // +0x8384e0 sub_8384e0
    Lifted::new(0x7000f24680, fn_7000f24680 as *const (), 0), // +0x838680 sub_838680
    Lifted::new(0x7000f24820, tpl_1668::<0x7000716490, 0x7001d550a8> as *const (), 2), // +0x838820 sub_838820
    Lifted::new(0x7000f24870, fn_7000f24870 as *const (), 0), // +0x838870 sub_838870
    Lifted::new(0x7000f24cb0, tpl_1668::<0x70007701e0, 0x7001d550a8> as *const (), 2), // +0x838cb0 sub_838cb0
    Lifted::new(0x7000f24d00, fn_7000f24d00 as *const (), 0), // +0x838d00 sub_838d00
    Lifted::new(0x7000f25150, fn_7000f25150 as *const (), 1), // +0x839150 sub_839150
    Lifted::new(0x7000f25160, fn_7000f25160 as *const (), 1), // +0x839160 sub_839160
    Lifted::new(0x7000f251e0, fn_7000f251e0 as *const (), 0), // +0x8391e0 sub_8391e0
    Lifted::new(0x7000f25280, fn_7000f25280 as *const (), 1), // +0x839280 sub_839280
    Lifted::new(0x7000f252b0, fn_7000f252b0 as *const (), 0), // +0x8392b0 sub_8392b0
    Lifted::new(0x7000f252f0, fn_7000f252f0 as *const (), 0), // +0x8392f0 sub_8392f0
    Lifted::new(0x7000f25500, fn_7000f25500 as *const (), 0), // +0x839500 sub_839500
    Lifted::new(0x7000f25850, fn_7000f25850 as *const (), 0), // +0x839850 sub_839850
    Lifted::new(0x7000f25a20, fn_7000f25a20 as *const (), 2), // +0x839a20 sub_839a20
    Lifted::new(0x7000f25a70, fn_7000f25a70 as *const (), 1), // +0x839a70 sub_839a70
    Lifted::new(0x7000f25d70, fn_7000f25d70 as *const (), 2), // +0x839d70 sub_839d70
    Lifted::new(0x7000f25df0, fn_7000f25df0 as *const (), 1), // +0x839df0 sub_839df0
    Lifted::new(0x7000f25e30, fn_7000f25e30 as *const (), 0), // +0x839e30 sub_839e30
    Lifted::new(0x7000f25f90, fn_7000f25f90 as *const (), 0), // +0x839f90 sub_839f90
    Lifted::new(0x7000f26040, fn_7000f26040 as *const (), 0), // +0x83a040 sub_83a040
    Lifted::new(0x7000f260e0, tpl_1669::<0xffffffffffffffff, 0x7, 0xb, 0x8, 0x70007675e0, 0x28> as *const (), 0), // +0x83a0e0 sub_83a0e0
    Lifted::new(0x7000f26170, tpl_1669::<0xffffffffffffffff, 0x7, 0xb, 0x8, 0x7000785b50, 0x28> as *const (), 0), // +0x83a170 sub_83a170
    Lifted::new(0x7000f26200, fn_7000f26200 as *const (), 0), // +0x83a200 sub_83a200
    Lifted::new(0x7000f26420, fn_7000f26420 as *const (), 0), // +0x83a420 sub_83a420
    Lifted::new(0x7000f264a0, fn_7000f264a0 as *const (), 0), // +0x83a4a0 sub_83a4a0
    Lifted::new(0x7000f26c80, fn_7000f26c80 as *const (), 0), // +0x83ac80 sub_83ac80
    Lifted::new(0x7000f26d00, tpl_212::<0x7000f23bb0, 0x7001d550a8> as *const (), 2), // +0x83ad00 sub_83ad00
    Lifted::new(0x7000f26d70, fn_7000f26d70 as *const (), 0), // +0x83ad70 sub_83ad70
    Lifted::new(0x7000f26fa0, fn_7000f26fa0 as *const (), 2), // +0x83afa0 sub_83afa0
    Lifted::new(0x7000f27220, fn_7000f27220 as *const (), 1), // +0x83b220 sub_83b220
    Lifted::new(0x7000f27250, fn_7000f27250 as *const (), 2), // +0x83b250 sub_83b250
    Lifted::new(0x7000f27880, fn_7000f27880 as *const (), 0), // +0x83b880 sub_83b880
    Lifted::new(0x7000f278e0, fn_7000f278e0 as *const (), 2), // +0x83b8e0 sub_83b8e0
    Lifted::new(0x7000f27be0, fn_7000f27be0 as *const (), 2), // +0x83bbe0 sub_83bbe0
    Lifted::new(0x7000f27ef0, fn_7000f27ef0 as *const (), 0), // +0x83bef0 sub_83bef0
    Lifted::new(0x7000f28160, fn_7000f28160 as *const (), 1), // +0x83c160 sub_83c160
    Lifted::new(0x7000f282d0, fn_7000f282d0 as *const (), 0), // +0x83c2d0 sub_83c2d0
    Lifted::new(0x7000f28330, fn_7000f28330 as *const (), 1), // +0x83c330 sub_83c330
    Lifted::new(0x7000f28440, fn_7000f28440 as *const (), 0), // +0x83c440 sub_83c440
    Lifted::new(0x7000f28550, fn_7000f28550 as *const (), 0), // +0x83c550 sub_83c550
    Lifted::new(0x7000f28780, fn_7000f28780 as *const (), 0), // +0x83c780 sub_83c780
    Lifted::new(0x7000f28b20, fn_7000f28b20 as *const (), 0), // +0x83cb20 sub_83cb20
    Lifted::new(0x7000f28c10, fn_7000f28c10 as *const (), 1), // +0x83cc10 sub_83cc10
    Lifted::new(0x7000f28db0, fn_7000f28db0 as *const (), 0), // +0x83cdb0 sub_83cdb0
    Lifted::new(0x7000f28e80, fn_7000f28e80 as *const (), 0), // +0x83ce80 sub_83ce80
    Lifted::new(0x7000f291f0, fn_7000f291f0 as *const (), 0), // +0x83d1f0 sub_83d1f0
    Lifted::new(0x7000f29580, fn_7000f29580 as *const (), 2), // +0x83d580 sub_83d580
    Lifted::new(0x7000f295d0, fn_7000f295d0 as *const (), 0), // +0x83d5d0 sub_83d5d0
    Lifted::new(0x7000f29950, fn_7000f29950 as *const (), 0), // +0x83d950 sub_83d950
    Lifted::new(0x7000f29df0, fn_7000f29df0 as *const (), 2), // +0x83ddf0 sub_83ddf0
    Lifted::new(0x7000f2a250, fn_7000f2a250 as *const (), 0), // +0x83e250 sub_83e250
    Lifted::new(0x7000f2a560, fn_7000f2a560 as *const (), 0), // +0x83e560 sub_83e560
    Lifted::new(0x7000f2a670, fn_7000f2a670 as *const (), 0), // +0x83e670 sub_83e670
    Lifted::new(0x7000f2a6e0, fn_7000f2a6e0 as *const (), 0), // +0x83e6e0 sub_83e6e0
    Lifted::new(0x7000f2ace0, fn_7000f2ace0 as *const (), 0), // +0x83ece0 sub_83ece0
    Lifted::new(0x7000f2ad50, fn_7000f2ad50 as *const (), 2), // +0x83ed50 sub_83ed50
    Lifted::new(0x7000f2b1c0, fn_7000f2b1c0 as *const (), 3), // +0x83f1c0 sub_83f1c0
    Lifted::new(0x7000f2b260, fn_7000f2b260 as *const (), 0), // +0x83f260 sub_83f260
    Lifted::new(0x7000f2b400, fn_7000f2b400 as *const (), 0), // +0x83f400 sub_83f400
    Lifted::new(0x7000f2b430, fn_7000f2b430 as *const (), 0), // +0x83f430 sub_83f430
    Lifted::new(0x7000f2b460, tpl_237::<0xffffffffffffffff> as *const (), 1), // +0x83f460 sub_83f460
    Lifted::new(0x7000f2b490, fn_7000f2b490 as *const (), 2), // +0x83f490 sub_83f490
    Lifted::new(0x7000f2ba90, fn_7000f2ba90 as *const (), 2), // +0x83fa90 sub_83fa90
    Lifted::new(0x7000f2bfa0, fn_7000f2bfa0 as *const (), 2), // +0x83ffa0 sub_83ffa0
    Lifted::new(0x7000f2c580, fn_7000f2c580 as *const (), 2), // +0x840580 sub_840580
    Lifted::new(0x7000f2caa0, fn_7000f2caa0 as *const (), 2), // +0x840aa0 sub_840aa0
    Lifted::new(0x7000f2cd30, fn_7000f2cd30 as *const (), 0), // +0x840d30 sub_840d30
    Lifted::new(0x7000f2d090, fn_7000f2d090 as *const (), 0), // +0x841090 sub_841090
    Lifted::new(0x7000f2d2c0, fn_7000f2d2c0 as *const (), 2), // +0x8412c0 sub_8412c0
    Lifted::new(0x7000f2d790, fn_7000f2d790 as *const (), 0), // +0x841790 sub_841790
    Lifted::new(0x7000f2d840, fn_7000f2d840 as *const (), 0), // +0x841840 sub_841840
    Lifted::new(0x7000f2da00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f2da10> as *const (), 0), // +0x841a00 sub_841a00
    Lifted::new(0x7000f2da20, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f2da30> as *const (), 0), // +0x841a20 sub_841a20
    Lifted::new(0x7000f2da50, fn_7000f2da50 as *const (), 0), // +0x841a50 sub_841a50
    Lifted::new(0x7000f2dcc0, fn_7000f2dcc0 as *const (), 0), // +0x841cc0 sub_841cc0
    Lifted::new(0x7000f2dcf0, fn_7000f2dcf0 as *const (), 0), // +0x841cf0 sub_841cf0
    Lifted::new(0x7000f2dff0, fn_7000f2dff0 as *const (), 0), // +0x841ff0 sub_841ff0
    Lifted::new(0x7000f2e160, fn_7000f2e160 as *const (), 0), // +0x842160 sub_842160
    Lifted::new(0x7000f2e390, fn_7000f2e390 as *const (), 0), // +0x842390 sub_842390
    Lifted::new(0x7000f2e6c0, tpl_1629::<0x700072fbb0> as *const (), 0), // +0x8426c0 sub_8426c0
    Lifted::new(0x7000f2e6f0, tpl_1670::<0x7001d54aa8, 0x7000735a90, 0x700072fbb0, 0x7001d54470, 0x7001d54460, 0x7001d54468> as *const (), 0), // +0x8426f0 sub_8426f0
    Lifted::new(0x7000f2e790, tpl_1629::<0x700072fbb0> as *const (), 0), // +0x842790 sub_842790
    Lifted::new(0x7000f2e7c0, tpl_1670::<0x7001d54aa8, 0x7000735a90, 0x700072fbb0, 0x7001d54470, 0x7001d54460, 0x7001d54468> as *const (), 0), // +0x8427c0 sub_8427c0
    Lifted::new(0x7000f2e860, tpl_1629::<0x700072fbb0> as *const (), 0), // +0x842860 sub_842860
    Lifted::new(0x7000f2e890, tpl_1671::<0x7000735a90, 0x700072fbb0, 0x7001d54470, 0x7001d54460, 0x7001d54468> as *const (), 0), // +0x842890 sub_842890
    Lifted::new(0x7000f2e910, tpl_1630::<0x7001c6abd8, 0x70017d94d8, 0x7001d543d0, 0x7001d543c8, 0x70017d94c8, 0x7001d543f8, 0x70017d9438, 0x700072fbb0, 0x70009a6500, 0x70010e2510, 0x7001d54968, 0x7000f2eae8, 0x700177c312, 0x7000a633c0, 0x7001d54460, 0x7001d54468> as *const (), 2), // +0x842910 sub_842910
    Lifted::new(0x7000f2ebf0, tpl_1629::<0x700072fbb0> as *const (), 0), // +0x842bf0 sub_842bf0
    Lifted::new(0x7000f2ec20, tpl_1671::<0x7000735a90, 0x700072fbb0, 0x7001d54470, 0x7001d54460, 0x7001d54468> as *const (), 0), // +0x842c20 sub_842c20
    Lifted::new(0x7000f2eca0, tpl_1630::<0x7001c6abd8, 0x70017d94d8, 0x7001d543d0, 0x7001d543c8, 0x70017d94c8, 0x7001d543f8, 0x70017d9438, 0x700072fbb0, 0x70009a6500, 0x70010e2510, 0x7001d54968, 0x7000f2ee78, 0x700177c312, 0x7000a633c0, 0x7001d54460, 0x7001d54468> as *const (), 2), // +0x842ca0 sub_842ca0
    Lifted::new(0x7000f2efa0, fn_7000f2efa0 as *const (), 2), // +0x842fa0 sub_842fa0
    Lifted::new(0x7000f2f500, fn_7000f2f500 as *const (), 2), // +0x843500 sub_843500
    Lifted::new(0x7000f2f6a0, fn_7000f2f6a0 as *const (), 0), // +0x8436a0 sub_8436a0
    Lifted::new(0x7000f2f790, fn_7000f2f790 as *const (), 1), // +0x843790 sub_843790
    Lifted::new(0x7000f2f870, fn_7000f2f870 as *const (), 1), // +0x843870 sub_843870
    Lifted::new(0x7000f2fa20, fn_7000f2fa20 as *const (), 0), // +0x843a20 sub_843a20
    Lifted::new(0x7000f2fcb0, fn_7000f2fcb0 as *const (), 1), // +0x843cb0 sub_843cb0
    Lifted::new(0x7000f2fcd0, fn_7000f2fcd0 as *const (), 1), // +0x843cd0 sub_843cd0
    Lifted::new(0x7000f2fe30, fn_7000f2fe30 as *const (), 2), // +0x843e30 sub_843e30
    Lifted::new(0x7000f300c0, fn_7000f300c0 as *const (), 3), // +0x8440c0 sub_8440c0
    Lifted::new(0x7000f30130, fn_7000f30130 as *const (), 1), // +0x844130 sub_844130
    Lifted::new(0x7000f302b0, fn_7000f302b0 as *const (), 1), // +0x8442b0 sub_8442b0
    Lifted::new(0x7000f30420, fn_7000f30420 as *const (), 1), // +0x844420 sub_844420
    Lifted::new(0x7000f30700, fn_7000f30700 as *const (), 1), // +0x844700 sub_844700
    Lifted::new(0x7000f30760, fn_7000f30760 as *const (), 2), // +0x844760 sub_844760
    Lifted::new(0x7000f30a80, fn_7000f30a80 as *const (), 2), // +0x844a80 sub_844a80
    Lifted::new(0x7000f30da0, fn_7000f30da0 as *const (), 0), // +0x844da0 sub_844da0
    Lifted::new(0x7000f30f20, fn_7000f30f20 as *const (), 0), // +0x844f20 sub_844f20
    Lifted::new(0x7000f31060, fn_7000f31060 as *const (), 0), // +0x845060 sub_845060
    Lifted::new(0x7000f31320, fn_7000f31320 as *const (), 1), // +0x845320 sub_845320
    Lifted::new(0x7000f31370, fn_7000f31370 as *const (), 0), // +0x845370 sub_845370
    Lifted::new(0x7000f314e0, fn_7000f314e0 as *const (), 1), // +0x8454e0 sub_8454e0
    Lifted::new(0x7000f31540, fn_7000f31540 as *const (), 1), // +0x845540 sub_845540
    Lifted::new(0x7000f31570, tpl_243::<0x7000f31580> as *const (), 0), // +0x845570 sub_845570
    Lifted::new(0x7000f31580, tpl_1317::<0x10, 0x8, 0x7001d54ce0, 0x7001d55060, 0x70017db8e8, 0x7001d54ef8, 0x7000f315e7, 0x700177c0b0, 0x18, 0x7001d54ef0> as *const (), 0), // +0x845580 sub_845580
    Lifted::new(0x7000f31670, fn_7000f31670 as *const (), 0), // +0x845670 sub_845670
    Lifted::new(0x7000f316f0, fn_7000f316f0 as *const (), 0), // +0x8456f0 sub_8456f0
    Lifted::new(0x7000f31800, fn_7000f31800 as *const (), 0), // +0x845800 sub_845800
    Lifted::new(0x7000f31860, fn_7000f31860 as *const (), 2), // +0x845860 sub_845860
    Lifted::new(0x7000f32530, fn_7000f32530 as *const (), 1), // +0x846530 sub_846530
    Lifted::new(0x7000f325b0, fn_7000f325b0 as *const (), 1), // +0x8465b0 sub_8465b0
    Lifted::new(0x7000f32620, fn_7000f32620 as *const (), 0), // +0x846620 sub_846620
    Lifted::new(0x7000f32690, fn_7000f32690 as *const (), 0), // +0x846690 sub_846690
    Lifted::new(0x7000f329e0, fn_7000f329e0 as *const (), 0), // +0x8469e0 sub_8469e0
    Lifted::new(0x7000f32ac0, fn_7000f32ac0 as *const (), 0), // +0x846ac0 sub_846ac0
    Lifted::new(0x7000f32b70, fn_7000f32b70 as *const (), 0), // +0x846b70 sub_846b70
    Lifted::new(0x7000f32c30, fn_7000f32c30 as *const (), 1), // +0x846c30 sub_846c30
    Lifted::new(0x7000f32da0, fn_7000f32da0 as *const (), 1), // +0x846da0 sub_846da0
    Lifted::new(0x7000f32f20, fn_7000f32f20 as *const (), 0), // +0x846f20 sub_846f20
    Lifted::new(0x7000f33070, fn_7000f33070 as *const (), 0), // +0x847070 sub_847070
    Lifted::new(0x7000f331d0, fn_7000f331d0 as *const (), 0), // +0x8471d0 sub_8471d0
    Lifted::new(0x7000f33230, fn_7000f33230 as *const (), 0), // +0x847230 sub_847230
    Lifted::new(0x7000f334e0, fn_7000f334e0 as *const (), 0), // +0x8474e0 sub_8474e0
    Lifted::new(0x7000f335d0, fn_7000f335d0 as *const (), 0), // +0x8475d0 sub_8475d0
    Lifted::new(0x7000f33620, fn_7000f33620 as *const (), 2), // +0x847620 sub_847620
    Lifted::new(0x7000f33d30, fn_7000f33d30 as *const (), 0), // +0x847d30 sub_847d30
    Lifted::new(0x7000f340a0, tpl_1672::<0x70017d2c10, 0x7001d54ef8, 0x7000f340c5, 0x7001859f48, 0x7000a62410, 0x7001859f38, 0x7001859f28> as *const (), 0), // +0x8480a0 sub_8480a0
    Lifted::new(0x7000f34110, fn_7000f34110 as *const (), 0), // +0x848110 sub_848110
    Lifted::new(0x7000f34220, fn_7000f34220 as *const (), 0), // +0x848220 sub_848220
    Lifted::new(0x7000f34390, fn_7000f34390 as *const (), 0), // +0x848390 sub_848390
    Lifted::new(0x7000f34420, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x848420 sub_848420
    Lifted::new(0x7000f34530, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x848530 sub_848530
    Lifted::new(0x7000f34640, fn_7000f34640 as *const (), 1), // +0x848640 sub_848640
    Lifted::new(0x7000f346c0, fn_7000f346c0 as *const (), 1), // +0x8486c0 sub_8486c0
    Lifted::new(0x7000f34740, fn_7000f34740 as *const (), 1), // +0x848740 sub_848740
    Lifted::new(0x7000f347c0, fn_7000f347c0 as *const (), 1), // +0x8487c0 sub_8487c0
    Lifted::new(0x7000f34860, fn_7000f34860 as *const (), 0), // +0x848860 sub_848860
    Lifted::new(0x7000f348e0, fn_7000f348e0 as *const (), 0), // +0x8488e0 sub_8488e0
    Lifted::new(0x7000f34980, tpl_1673::<0x7001c6abd8, 0xffffffffffffffff, 0x7000f3b100, 0x8, 0x700177c0b0> as *const (), 0), // +0x848980 sub_848980
    Lifted::new(0x7000f34a30, fn_7000f34a30 as *const (), 0), // +0x848a30 sub_848a30
    Lifted::new(0x7000f34a50, fn_7000f34a50 as *const (), 0), // +0x848a50 sub_848a50
    Lifted::new(0x7000f34aa0, tpl_906::<0x7000f402e0, 0x700177c312, 0x7001d54968, 0x7000f34c24, 0x7000b7fc90, 0x7000f34c2a> as *const (), 0), // +0x848aa0 sub_848aa0
    Lifted::new(0x7000f34c90, tpl_1140::<0x7000f402e0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000f34dcf, 0x7000b7fc90, 0x7000f34df5> as *const (), 0), // +0x848c90 sub_848c90
    Lifted::new(0x7000f34e50, fn_7000f34e50 as *const (), 0), // +0x848e50 sub_848e50
    Lifted::new(0x7000f34ec0, fn_7000f34ec0 as *const (), 0), // +0x848ec0 sub_848ec0
    Lifted::new(0x7000f34f40, fn_7000f34f40 as *const (), 0), // +0x848f40 sub_848f40
    Lifted::new(0x7000f35090, tpl_792::<0x7001859f70, 0x70007844b0, 0x7000770500> as *const (), 0), // +0x849090 sub_849090
    Lifted::new(0x7000f350c0, fn_7000f350c0 as *const (), 0), // +0x8490c0 sub_8490c0
    Lifted::new(0x7000f35100, fn_7000f35100 as *const (), 0), // +0x849100 sub_849100
    Lifted::new(0x7000f351b0, tpl_793::<0x7001859f70, 0x70007844b0, 0x7000770500, 0x700177c312> as *const (), 0), // +0x8491b0 sub_8491b0
    Lifted::new(0x7000f35210, fn_7000f35210 as *const (), 0), // +0x849210 sub_849210
    Lifted::new(0x7000f352a0, fn_7000f352a0 as *const (), 0), // +0x8492a0 sub_8492a0
    Lifted::new(0x7000f353b0, tpl_15 as *const (), 1), // +0x8493b0 sub_8493b0
    Lifted::new(0x7000f353c0, fn_7000f353c0 as *const (), 0), // +0x8493c0 sub_8493c0
    Lifted::new(0x7000f35490, fn_7000f35490 as *const (), 0), // +0x849490 sub_849490
    Lifted::new(0x7000f355e0, fn_7000f355e0 as *const (), 1), // +0x8495e0 sub_8495e0
    Lifted::new(0x7000f35640, fn_7000f35640 as *const (), 0), // +0x849640 sub_849640
    Lifted::new(0x7000f356c0, fn_7000f356c0 as *const (), 2), // +0x8496c0 sub_8496c0
    Lifted::new(0x7000f357c0, fn_7000f357c0 as *const (), 2), // +0x8497c0 sub_8497c0
    Lifted::new(0x7000f35e60, fn_7000f35e60 as *const (), 1), // +0x849e60 sub_849e60
    Lifted::new(0x7000f35ec0, fn_7000f35ec0 as *const (), 0), // +0x849ec0 sub_849ec0
    Lifted::new(0x7000f36100, fn_7000f36100 as *const (), 0), // +0x84a100 sub_84a100
    Lifted::new(0x7000f36240, fn_7000f36240 as *const (), 0), // +0x84a240 sub_84a240
    Lifted::new(0x7000f36360, fn_7000f36360 as *const (), 0), // +0x84a360 sub_84a360
    Lifted::new(0x7000f36510, fn_7000f36510 as *const (), 0), // +0x84a510 sub_84a510
    Lifted::new(0x7000f36850, fn_7000f36850 as *const (), 2), // +0x84a850 sub_84a850
    Lifted::new(0x7000f36a70, fn_7000f36a70 as *const (), 0), // +0x84aa70 sub_84aa70
    Lifted::new(0x7000f36f80, fn_7000f36f80 as *const (), 2), // +0x84af80 sub_84af80
    Lifted::new(0x7000f37380, fn_7000f37380 as *const (), 0), // +0x84b380 sub_84b380
    Lifted::new(0x7000f37640, fn_7000f37640 as *const (), 0), // +0x84b640 sub_84b640
    Lifted::new(0x7000f38250, fn_7000f38250 as *const (), 2), // +0x84c250 sub_84c250
    Lifted::new(0x7000f386c0, fn_7000f386c0 as *const (), 0), // +0x84c6c0 sub_84c6c0
    Lifted::new(0x7000f388d0, fn_7000f388d0 as *const (), 0), // +0x84c8d0 sub_84c8d0
    Lifted::new(0x7000f38cb0, fn_7000f38cb0 as *const (), 0), // +0x84ccb0 sub_84ccb0
    Lifted::new(0x7000f38eb0, fn_7000f38eb0 as *const (), 2), // +0x84ceb0 sub_84ceb0
    Lifted::new(0x7000f398c0, fn_7000f398c0 as *const (), 2), // +0x84d8c0 sub_84d8c0
    Lifted::new(0x7000f3a400, fn_7000f3a400 as *const (), 2), // +0x84e400 sub_84e400
    Lifted::new(0x7000f3a4e0, fn_7000f3a4e0 as *const (), 3), // +0x84e4e0 sub_84e4e0
    Lifted::new(0x7000f3a550, fn_7000f3a550 as *const (), 1), // +0x84e550 sub_84e550
    Lifted::new(0x7000f3a580, fn_7000f3a580 as *const (), 0), // +0x84e580 sub_84e580
    Lifted::new(0x7000f3a8e0, fn_7000f3a8e0 as *const (), 0), // +0x84e8e0 sub_84e8e0
    Lifted::new(0x7000f3a9c0, fn_7000f3a9c0 as *const (), 0), // +0x84e9c0 sub_84e9c0
    Lifted::new(0x7000f3abb0, fn_7000f3abb0 as *const (), 0), // +0x84ebb0 sub_84ebb0
    Lifted::new(0x7000f3acc0, fn_7000f3acc0 as *const (), 1), // +0x84ecc0 sub_84ecc0
    Lifted::new(0x7000f3ad40, fn_7000f3ad40 as *const (), 1), // +0x84ed40 sub_84ed40
    Lifted::new(0x7000f3adc0, fn_7000f3adc0 as *const (), 0), // +0x84edc0 sub_84edc0
    Lifted::new(0x7000f3ae80, fn_7000f3ae80 as *const (), 0), // +0x84ee80 sub_84ee80
    Lifted::new(0x7000f3afd0, fn_7000f3afd0 as *const (), 0), // +0x84efd0 sub_84efd0
    Lifted::new(0x7000f3b0d0, fn_7000f3b0d0 as *const (), 1), // +0x84f0d0 sub_84f0d0
    Lifted::new(0x7000f3b100, fn_7000f3b100 as *const (), 0), // +0x84f100 sub_84f100
    Lifted::new(0x7000f3b1e0, fn_7000f3b1e0 as *const (), 2), // +0x84f1e0 sub_84f1e0
    Lifted::new(0x7000f3b5a0, fn_7000f3b5a0 as *const (), 0), // +0x84f5a0 sub_84f5a0
    Lifted::new(0x7000f3b5d0, fn_7000f3b5d0 as *const (), 0), // +0x84f5d0 sub_84f5d0
    Lifted::new(0x7000f3b740, fn_7000f3b740 as *const (), 0), // +0x84f740 sub_84f740
    Lifted::new(0x7000f3b7b0, fn_7000f3b7b0 as *const (), 0), // +0x84f7b0 sub_84f7b0
    Lifted::new(0x7000f3b800, fn_7000f3b800 as *const (), 0), // +0x84f800 sub_84f800
    Lifted::new(0x7000f3b930, fn_7000f3b930 as *const (), 0), // +0x84f930 sub_84f930
    Lifted::new(0x7000f3b960, fn_7000f3b960 as *const (), 0), // +0x84f960 sub_84f960
    Lifted::new(0x7000f3b9c0, fn_7000f3b9c0 as *const (), 0), // +0x84f9c0 sub_84f9c0
    Lifted::new(0x7000f3ba50, fn_7000f3ba50 as *const (), 2), // +0x84fa50 sub_84fa50
    Lifted::new(0x7000f3ba90, fn_7000f3ba90 as *const (), 0), // +0x84fa90 sub_84fa90
    Lifted::new(0x7000f3bae0, fn_7000f3bae0 as *const (), 2), // +0x84fae0 sub_84fae0
    Lifted::new(0x7000f3bb60, fn_7000f3bb60 as *const (), 0), // +0x84fb60 sub_84fb60
    Lifted::new(0x7000f3bd70, fn_7000f3bd70 as *const (), 2), // +0x84fd70 sub_84fd70
    Lifted::new(0x7000f3c600, fn_7000f3c600 as *const (), 2), // +0x850600 sub_850600
    Lifted::new(0x7000f3c980, fn_7000f3c980 as *const (), 0), // +0x850980 sub_850980
    Lifted::new(0x7000f3cc30, fn_7000f3cc30 as *const (), 2), // +0x850c30 sub_850c30
    Lifted::new(0x7000f3ccf0, fn_7000f3ccf0 as *const (), 2), // +0x850cf0 sub_850cf0
    Lifted::new(0x7000f3d400, fn_7000f3d400 as *const (), 2), // +0x851400 sub_851400
    Lifted::new(0x7000f3d8f0, fn_7000f3d8f0 as *const (), 0), // +0x8518f0 sub_8518f0
    Lifted::new(0x7000f3de60, fn_7000f3de60 as *const (), 2), // +0x851e60 sub_851e60
    Lifted::new(0x7000f3e2d0, fn_7000f3e2d0 as *const (), 0), // +0x8522d0 sub_8522d0
    Lifted::new(0x7000f3e430, fn_7000f3e430 as *const (), 2), // +0x852430 sub_852430
    Lifted::new(0x7000f3e720, fn_7000f3e720 as *const (), 2), // +0x852720 sub_852720
    Lifted::new(0x7000f3f000, fn_7000f3f000 as *const (), 0), // +0x853000 sub_853000
    Lifted::new(0x7000f3f110, fn_7000f3f110 as *const (), 2), // +0x853110 sub_853110
    Lifted::new(0x7000f3f710, fn_7000f3f710 as *const (), 2), // +0x853710 sub_853710
    Lifted::new(0x7000f3fb50, fn_7000f3fb50 as *const (), 0), // +0x853b50 sub_853b50
    Lifted::new(0x7000f3fbd0, fn_7000f3fbd0 as *const (), 0), // +0x853bd0 sub_853bd0
    Lifted::new(0x7000f3fc40, fn_7000f3fc40 as *const (), 0), // +0x853c40 sub_853c40
    Lifted::new(0x7000f3fc90, fn_7000f3fc90 as *const (), 0), // +0x853c90 sub_853c90
    Lifted::new(0x7000f3fcf0, fn_7000f3fcf0 as *const (), 0), // +0x853cf0 sub_853cf0
    Lifted::new(0x7000f3fe90, fn_7000f3fe90 as *const (), 0), // +0x853e90 sub_853e90
    Lifted::new(0x7000f3fed0, fn_7000f3fed0 as *const (), 0), // +0x853ed0 sub_853ed0
    Lifted::new(0x7000f3ff60, tpl_1673::<0x7001c6abd8, 0xffffffffffffffff, 0x7000f3b100, 0x8, 0x700177c0b0> as *const (), 0), // +0x853f60 sub_853f60
    Lifted::new(0x7000f40010, fn_7000f40010 as *const (), 0), // +0x854010 sub_854010
    Lifted::new(0x7000f40190, fn_7000f40190 as *const (), 0), // +0x854190 sub_854190
    Lifted::new(0x7000f402e0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000f40334, 0x70007f0910, 0x7000f4034f> as *const (), 0), // +0x8542e0 sub_8542e0
    Lifted::new(0x7000f40370, fn_7000f40370 as *const (), 0), // +0x854370 sub_854370
    Lifted::new(0x7000f405a0, fn_7000f405a0 as *const (), 0), // +0x8545a0 sub_8545a0
    Lifted::new(0x7000f406b0, tpl_906::<0x7000f41040, 0x700177c312, 0x7001d54968, 0x7000f40834, 0x7000a36e40, 0x7000f4083a> as *const (), 0), // +0x8546b0 sub_8546b0
    Lifted::new(0x7000f408a0, tpl_391::<0x7000b092a0, 0x700177c0e0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000f409cc, 0x7000a36d80, 0x7000f409f4> as *const (), 0), // +0x8548a0 sub_8548a0
    Lifted::new(0x7000f40a50, tpl_392::<0x700177c0e0> as *const (), 0), // +0x854a50 sub_854a50
    Lifted::new(0x7000f40a80, fn_7000f40a80 as *const (), 1), // +0x854a80 sub_854a80
    Lifted::new(0x7000f40ab0, fn_7000f40ab0 as *const (), 2), // +0x854ab0 sub_854ab0
    Lifted::new(0x7000f40c90, fn_7000f40c90 as *const (), 2), // +0x854c90 sub_854c90
    Lifted::new(0x7000f40e50, fn_7000f40e50 as *const (), 0), // +0x854e50 sub_854e50
    Lifted::new(0x7000f40ed0, fn_7000f40ed0 as *const (), 0), // +0x854ed0 sub_854ed0
    Lifted::new(0x7000f40f40, fn_7000f40f40 as *const (), 0), // +0x854f40 sub_854f40
    Lifted::new(0x7000f40fb0, fn_7000f40fb0 as *const (), 0), // +0x854fb0 sub_854fb0
    Lifted::new(0x7000f41040, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000f41094, 0x70007f0910, 0x7000f410af> as *const (), 0), // +0x855040 sub_855040
    Lifted::new(0x7000f410d0, tpl_571::<0x7000a401b0> as *const (), 0), // +0x8550d0 sub_8550d0
    Lifted::new(0x7000f410f0, tpl_1674::<0x10, 0x14, 0x70007a5a10, 0x70007847a0> as *const (), 0), // +0x8550f0 sub_8550f0
    Lifted::new(0x7000f41160, tpl_1674::<0x10, 0x14, 0x7000768180, 0x700075d630> as *const (), 0), // +0x855160 sub_855160
    Lifted::new(0x7000f411d0, fn_7000f411d0 as *const (), 2), // +0x8551d0 sub_8551d0
    Lifted::new(0x7000f41380, fn_7000f41380 as *const (), 0), // +0x855380 sub_855380
    Lifted::new(0x7000f41470, fn_7000f41470 as *const (), 2), // +0x855470 sub_855470
    Lifted::new(0x7000f41530, fn_7000f41530 as *const (), 0), // +0x855530 sub_855530
    Lifted::new(0x7000f41730, fn_7000f41730 as *const (), 0), // +0x855730 sub_855730
    Lifted::new(0x7000f41a00, fn_7000f41a00 as *const (), 0), // +0x855a00 sub_855a00
    Lifted::new(0x7000f41ad0, tpl_569::<0x19, 0x10, 0x7000f41ad0> as *const (), 0), // +0x855ad0 sub_855ad0
    Lifted::new(0x7000f41b60, tpl_1675::<0x7001779e20, 0x700074a340> as *const (), 0), // +0x855b60 sub_855b60
    Lifted::new(0x7000f41bf0, fn_7000f41bf0 as *const (), 0), // +0x855bf0 sub_855bf0
    Lifted::new(0x7000f41c70, fn_7000f41c70 as *const (), 2), // +0x855c70 sub_855c70
    Lifted::new(0x7000f41d70, fn_7000f41d70 as *const (), 2), // +0x855d70 sub_855d70
    Lifted::new(0x7000f41de0, fn_7000f41de0 as *const (), 0), // +0x855de0 sub_855de0
    Lifted::new(0x7000f41f00, fn_7000f41f00 as *const (), 1), // +0x855f00 sub_855f00
    Lifted::new(0x7000f41f60, fn_7000f41f60 as *const (), 0), // +0x855f60 sub_855f60
    Lifted::new(0x7000f42050, fn_7000f42050 as *const (), 0), // +0x856050 sub_856050
    Lifted::new(0x7000f420f0, fn_7000f420f0 as *const (), 0), // +0x8560f0 sub_8560f0
    Lifted::new(0x7000f421e0, tpl_15 as *const (), 1), // +0x8561e0 sub_8561e0
    Lifted::new(0x7000f421f0, fn_7000f421f0 as *const (), 1), // +0x8561f0 sub_8561f0
    Lifted::new(0x7000f42220, fn_7000f42220 as *const (), 2), // +0x856220 sub_856220
    Lifted::new(0x7000f422a0, fn_7000f422a0 as *const (), 0), // +0x8562a0 sub_8562a0
    Lifted::new(0x7000f42500, fn_7000f42500 as *const (), 0), // +0x856500 sub_856500
    Lifted::new(0x7000f42640, fn_7000f42640 as *const (), 0), // +0x856640 sub_856640
    Lifted::new(0x7000f42670, fn_7000f42670 as *const (), 0), // +0x856670 sub_856670
    Lifted::new(0x7000f42740, fn_7000f42740 as *const (), 2), // +0x856740 sub_856740
    Lifted::new(0x7000f427e0, fn_7000f427e0 as *const (), 2), // +0x8567e0 sub_8567e0
    Lifted::new(0x7000f42890, fn_7000f42890 as *const (), 0), // +0x856890 sub_856890
    Lifted::new(0x7000f42a00, tpl_1676::<0x40> as *const (), 0), // +0x856a00 sub_856a00
    Lifted::new(0x7000f42a30, fn_7000f42a30 as *const (), 0), // +0x856a30 sub_856a30
    Lifted::new(0x7000f42ac0, fn_7000f42ac0 as *const (), 1), // +0x856ac0 sub_856ac0
    Lifted::new(0x7000f42bb0, tpl_275::<0x70, 0x28> as *const (), 1), // +0x856bb0 sub_856bb0
    Lifted::new(0x7000f42bc0, tpl_374::<0x40> as *const (), 0), // +0x856bc0 sub_856bc0
    Lifted::new(0x7000f42bf0, tpl_1677::<0x7000f7a6a0, 0x7000767d50, 0x700074a010, 0x7000f7a5e0> as *const (), 2), // +0x856bf0 sub_856bf0
    Lifted::new(0x7000f42da0, tpl_1677::<0x7000f7a700, 0x7000767d50, 0x700074a010, 0x7000f7a640> as *const (), 2), // +0x856da0 sub_856da0
    Lifted::new(0x7000f42f50, tpl_55::<0x700177c0d0> as *const (), 0), // +0x856f50 sub_856f50
    Lifted::new(0x7000f42f90, tpl_55::<0x700177c0d0> as *const (), 0), // +0x856f90 sub_856f90
    Lifted::new(0x7000f42fd0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x856fd0 sub_856fd0
    Lifted::new(0x7000f43010, fn_7000f43010 as *const (), 2), // +0x857010 sub_857010
    Lifted::new(0x7000f431d0, tpl_569::<0x19, 0x10, 0x7000f431d0> as *const (), 0), // +0x8571d0 sub_8571d0
    Lifted::new(0x7000f43230, tpl_397::<0x8, 0xc, 0x10, 0x18, 0x700076bf20> as *const (), 0), // +0x857230 sub_857230
    Lifted::new(0x7000f432b0, fn_7000f432b0 as *const (), 2), // +0x8572b0 sub_8572b0
    Lifted::new(0x7000f436a0, tpl_423::<0xc, 0x8, 0x18, 0x1, 0x10, 0x700076bf20, 0x70009fc7f0> as *const (), 0), // +0x8576a0 sub_8576a0
    Lifted::new(0x7000f43780, tpl_423::<0xc, 0x8, 0x18, 0x1, 0x10, 0x700076bf20, 0x70009fc7f0> as *const (), 0), // +0x857780 sub_857780
    Lifted::new(0x7000f43860, tpl_424::<0xc, 0x8, 0x1, 0x7000a25800> as *const (), 0), // +0x857860 sub_857860
    Lifted::new(0x7000f438e0, tpl_423::<0xc, 0x8, 0x18, 0x1, 0x10, 0x700076bf20, 0x70009fc7f0> as *const (), 0), // +0x8578e0 sub_8578e0
    Lifted::new(0x7000f439c0, fn_7000f439c0 as *const (), 0), // +0x8579c0 sub_8579c0
    Lifted::new(0x7000f43be0, fn_7000f43be0 as *const (), 0), // +0x857be0 sub_857be0
    Lifted::new(0x7000f43c80, fn_7000f43c80 as *const (), 0), // +0x857c80 sub_857c80
    Lifted::new(0x7000f43ce0, fn_7000f43ce0 as *const (), 0), // +0x857ce0 sub_857ce0
    Lifted::new(0x7000f43d50, fn_7000f43d50 as *const (), 0), // +0x857d50 sub_857d50
    Lifted::new(0x7000f43e10, fn_7000f43e10 as *const (), 0), // +0x857e10 sub_857e10
    Lifted::new(0x7000f43ed0, fn_7000f43ed0 as *const (), 0), // +0x857ed0 sub_857ed0
    Lifted::new(0x7000f43f60, fn_7000f43f60 as *const (), 0), // +0x857f60 sub_857f60
    Lifted::new(0x7000f440f0, fn_7000f440f0 as *const (), 0), // +0x8580f0 sub_8580f0
    Lifted::new(0x7000f44250, fn_7000f44250 as *const (), 0), // +0x858250 sub_858250
    Lifted::new(0x7000f446e0, fn_7000f446e0 as *const (), 0), // +0x8586e0 sub_8586e0
    Lifted::new(0x7000f44b90, fn_7000f44b90 as *const (), 2), // +0x858b90 sub_858b90
    Lifted::new(0x7000f44cb0, fn_7000f44cb0 as *const (), 0), // +0x858cb0 sub_858cb0
    Lifted::new(0x7000f44e30, fn_7000f44e30 as *const (), 0), // +0x858e30 sub_858e30
    Lifted::new(0x7000f450a0, fn_7000f450a0 as *const (), 0), // +0x8590a0 sub_8590a0
    Lifted::new(0x7000f45340, fn_7000f45340 as *const (), 0), // +0x859340 sub_859340
    Lifted::new(0x7000f45410, fn_7000f45410 as *const (), 0), // +0x859410 sub_859410
    Lifted::new(0x7000f45680, fn_7000f45680 as *const (), 0), // +0x859680 sub_859680
    Lifted::new(0x7000f45780, fn_7000f45780 as *const (), 0), // +0x859780 sub_859780
    Lifted::new(0x7000f457c0, fn_7000f457c0 as *const (), 0), // +0x8597c0 sub_8597c0
    Lifted::new(0x7000f458f0, fn_7000f458f0 as *const (), 0), // +0x8598f0 sub_8598f0
    Lifted::new(0x7000f45980, fn_7000f45980 as *const (), 0), // +0x859980 sub_859980
    Lifted::new(0x7000f45b20, fn_7000f45b20 as *const (), 0), // +0x859b20 sub_859b20
    Lifted::new(0x7000f45c50, fn_7000f45c50 as *const (), 0), // +0x859c50 sub_859c50
    Lifted::new(0x7000f45dd0, fn_7000f45dd0 as *const (), 0), // +0x859dd0 sub_859dd0
    Lifted::new(0x7000f45ea0, fn_7000f45ea0 as *const (), 0), // +0x859ea0 sub_859ea0
    Lifted::new(0x7000f46110, fn_7000f46110 as *const (), 0), // +0x85a110 sub_85a110
    Lifted::new(0x7000f46160, fn_7000f46160 as *const (), 0), // +0x85a160 sub_85a160
    Lifted::new(0x7000f46300, fn_7000f46300 as *const (), 0), // +0x85a300 sub_85a300
    Lifted::new(0x7000f46460, fn_7000f46460 as *const (), 0), // +0x85a460 sub_85a460
    Lifted::new(0x7000f465a0, fn_7000f465a0 as *const (), 0), // +0x85a5a0 sub_85a5a0
    Lifted::new(0x7000f46640, fn_7000f46640 as *const (), 0), // +0x85a640 sub_85a640
    Lifted::new(0x7000f466a0, fn_7000f466a0 as *const (), 0), // +0x85a6a0 sub_85a6a0
    Lifted::new(0x7000f467f0, fn_7000f467f0 as *const (), 0), // +0x85a7f0 sub_85a7f0
    Lifted::new(0x7000f46c30, fn_7000f46c30 as *const (), 0), // +0x85ac30 sub_85ac30
    Lifted::new(0x7000f46d10, fn_7000f46d10 as *const (), 0), // +0x85ad10 sub_85ad10
    Lifted::new(0x7000f46f90, fn_7000f46f90 as *const (), 0), // +0x85af90 sub_85af90
    Lifted::new(0x7000f485a0, fn_7000f485a0 as *const (), 0), // +0x85c5a0 sub_85c5a0
    Lifted::new(0x7000f49280, fn_7000f49280 as *const (), 0), // +0x85d280 sub_85d280
    Lifted::new(0x7000f49d80, fn_7000f49d80 as *const (), 2), // +0x85dd80 sub_85dd80
    Lifted::new(0x7000f49fd0, fn_7000f49fd0 as *const (), 0), // +0x85dfd0 sub_85dfd0
    Lifted::new(0x7000f4a120, fn_7000f4a120 as *const (), 0), // +0x85e120 sub_85e120
    Lifted::new(0x7000f4a2a0, fn_7000f4a2a0 as *const (), 0), // +0x85e2a0 sub_85e2a0
    Lifted::new(0x7000f4a340, fn_7000f4a340 as *const (), 0), // +0x85e340 sub_85e340
    Lifted::new(0x7000f4a3d0, fn_7000f4a3d0 as *const (), 0), // +0x85e3d0 sub_85e3d0
    Lifted::new(0x7000f4a420, fn_7000f4a420 as *const (), 0), // +0x85e420 sub_85e420
    Lifted::new(0x7000f4a530, fn_7000f4a530 as *const (), 0), // +0x85e530 sub_85e530
    Lifted::new(0x7000f4a600, fn_7000f4a600 as *const (), 0), // +0x85e600 sub_85e600
    Lifted::new(0x7000f4a870, fn_7000f4a870 as *const (), 0), // +0x85e870 sub_85e870
    Lifted::new(0x7000f4a8a0, fn_7000f4a8a0 as *const (), 2), // +0x85e8a0 sub_85e8a0
    Lifted::new(0x7000f4a9f0, fn_7000f4a9f0 as *const (), 0), // +0x85e9f0 sub_85e9f0
    Lifted::new(0x7000f4aaf0, fn_7000f4aaf0 as *const (), 0), // +0x85eaf0 sub_85eaf0
    Lifted::new(0x7000f4ab80, fn_7000f4ab80 as *const (), 0), // +0x85eb80 sub_85eb80
    Lifted::new(0x7000f4ac10, fn_7000f4ac10 as *const (), 0), // +0x85ec10 sub_85ec10
    Lifted::new(0x7000f4ad00, fn_7000f4ad00 as *const (), 0), // +0x85ed00 sub_85ed00
    Lifted::new(0x7000f4adf0, fn_7000f4adf0 as *const (), 0), // +0x85edf0 sub_85edf0
    Lifted::new(0x7000f4ae60, fn_7000f4ae60 as *const (), 0), // +0x85ee60 sub_85ee60
    Lifted::new(0x7000f4aed0, fn_7000f4aed0 as *const (), 0), // +0x85eed0 sub_85eed0
    Lifted::new(0x7000f4af80, fn_7000f4af80 as *const (), 0), // +0x85ef80 sub_85ef80
    Lifted::new(0x7000f4b030, fn_7000f4b030 as *const (), 0), // +0x85f030 sub_85f030
    Lifted::new(0x7000f4b0a0, fn_7000f4b0a0 as *const (), 0), // +0x85f0a0 sub_85f0a0
    Lifted::new(0x7000f4b150, fn_7000f4b150 as *const (), 2), // +0x85f150 sub_85f150
    Lifted::new(0x7000f4b2f0, fn_7000f4b2f0 as *const (), 0), // +0x85f2f0 sub_85f2f0
    Lifted::new(0x7000f4b390, fn_7000f4b390 as *const (), 0), // +0x85f390 sub_85f390
    Lifted::new(0x7000f4b4a0, fn_7000f4b4a0 as *const (), 0), // +0x85f4a0 sub_85f4a0
    Lifted::new(0x7000f4b550, fn_7000f4b550 as *const (), 0), // +0x85f550 sub_85f550
    Lifted::new(0x7000f4b610, fn_7000f4b610 as *const (), 0), // +0x85f610 sub_85f610
    Lifted::new(0x7000f4b6a0, fn_7000f4b6a0 as *const (), 0), // +0x85f6a0 sub_85f6a0
    Lifted::new(0x7000f4b840, fn_7000f4b840 as *const (), 0), // +0x85f840 sub_85f840
    Lifted::new(0x7000f4b9f0, fn_7000f4b9f0 as *const (), 0), // +0x85f9f0 sub_85f9f0
    Lifted::new(0x7000f4baa0, fn_7000f4baa0 as *const (), 0), // +0x85faa0 sub_85faa0
    Lifted::new(0x7000f4bb70, fn_7000f4bb70 as *const (), 0), // +0x85fb70 sub_85fb70
    Lifted::new(0x7000f4bc90, fn_7000f4bc90 as *const (), 0), // +0x85fc90 sub_85fc90
    Lifted::new(0x7000f4bd50, fn_7000f4bd50 as *const (), 0), // +0x85fd50 sub_85fd50
    Lifted::new(0x7000f4be10, fn_7000f4be10 as *const (), 0), // +0x85fe10 sub_85fe10
    Lifted::new(0x7000f4bec0, fn_7000f4bec0 as *const (), 0), // +0x85fec0 sub_85fec0
    Lifted::new(0x7000f4bf70, fn_7000f4bf70 as *const (), 0), // +0x85ff70 sub_85ff70
    Lifted::new(0x7000f4bfe0, fn_7000f4bfe0 as *const (), 0), // +0x85ffe0 sub_85ffe0
    Lifted::new(0x7000f4c220, fn_7000f4c220 as *const (), 0), // +0x860220 sub_860220
    Lifted::new(0x7000f4c450, fn_7000f4c450 as *const (), 0), // +0x860450 sub_860450
    Lifted::new(0x7000f4c4d0, fn_7000f4c4d0 as *const (), 2), // +0x8604d0 sub_8604d0
    Lifted::new(0x7000f4c700, fn_7000f4c700 as *const (), 0), // +0x860700 sub_860700
    Lifted::new(0x7000f4c880, fn_7000f4c880 as *const (), 0), // +0x860880 sub_860880
    Lifted::new(0x7000f4d3a0, fn_7000f4d3a0 as *const (), 0), // +0x8613a0 sub_8613a0
    Lifted::new(0x7000f4d790, fn_7000f4d790 as *const (), 2), // +0x861790 sub_861790
    Lifted::new(0x7000f4dba0, fn_7000f4dba0 as *const (), 0), // +0x861ba0 sub_861ba0
    Lifted::new(0x7000f4e1d0, fn_7000f4e1d0 as *const (), 0), // +0x8621d0 sub_8621d0
    Lifted::new(0x7000f4e830, fn_7000f4e830 as *const (), 0), // +0x862830 sub_862830
    Lifted::new(0x7000f4e960, fn_7000f4e960 as *const (), 0), // +0x862960 sub_862960
    Lifted::new(0x7000f4edb0, fn_7000f4edb0 as *const (), 0), // +0x862db0 sub_862db0
    Lifted::new(0x7000f4f3b0, fn_7000f4f3b0 as *const (), 2), // +0x8633b0 sub_8633b0
    Lifted::new(0x7000f4fd00, fn_7000f4fd00 as *const (), 2), // +0x863d00 sub_863d00
    Lifted::new(0x7000f506c0, fn_7000f506c0 as *const (), 2), // +0x8646c0 sub_8646c0
    Lifted::new(0x7000f50b40, fn_7000f50b40 as *const (), 0), // +0x864b40 sub_864b40
    Lifted::new(0x7000f51220, fn_7000f51220 as *const (), 0), // +0x865220 sub_865220
    Lifted::new(0x7000f512a0, fn_7000f512a0 as *const (), 0), // +0x8652a0 sub_8652a0
    Lifted::new(0x7000f51370, fn_7000f51370 as *const (), 0), // +0x865370 sub_865370
    Lifted::new(0x7000f513c0, fn_7000f513c0 as *const (), 2), // +0x8653c0 sub_8653c0
    Lifted::new(0x7000f51b40, fn_7000f51b40 as *const (), 2), // +0x865b40 sub_865b40
    Lifted::new(0x7000f51cc0, fn_7000f51cc0 as *const (), 2), // +0x865cc0 sub_865cc0
    Lifted::new(0x7000f52150, fn_7000f52150 as *const (), 0), // +0x866150 sub_866150
    Lifted::new(0x7000f526b0, fn_7000f526b0 as *const (), 0), // +0x8666b0 sub_8666b0
    Lifted::new(0x7000f52850, fn_7000f52850 as *const (), 2), // +0x866850 sub_866850
    Lifted::new(0x7000f52990, fn_7000f52990 as *const (), 0), // +0x866990 sub_866990
    Lifted::new(0x7000f529d0, fn_7000f529d0 as *const (), 0), // +0x8669d0 sub_8669d0
    Lifted::new(0x7000f52c20, fn_7000f52c20 as *const (), 0), // +0x866c20 sub_866c20
    Lifted::new(0x7000f52e40, fn_7000f52e40 as *const (), 0), // +0x866e40 sub_866e40
    Lifted::new(0x7000f52f70, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x866f70 sub_866f70
    Lifted::new(0x7000f53010, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x867010 sub_867010
    Lifted::new(0x7000f530b0, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x8670b0 sub_8670b0
    Lifted::new(0x7000f53150, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x867150 sub_867150
    Lifted::new(0x7000f53210, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x867210 sub_867210
    Lifted::new(0x7000f532d0, tpl_515::<0x8, 0x1, 0x7000a25800, 0xc> as *const (), 0), // +0x8672d0 sub_8672d0
    Lifted::new(0x7000f53320, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x867320 sub_867320
    Lifted::new(0x7000f533e0, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x8673e0 sub_8673e0
    Lifted::new(0x7000f534a0, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x8674a0 sub_8674a0
    Lifted::new(0x7000f53560, tpl_515::<0x8, 0x1, 0x7000a25800, 0xc> as *const (), 0), // +0x867560 sub_867560
    Lifted::new(0x7000f535b0, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x8675b0 sub_8675b0
    Lifted::new(0x7000f53670, fn_7000f53670 as *const (), 0), // +0x867670 sub_867670
    Lifted::new(0x7000f53750, fn_7000f53750 as *const (), 0), // +0x867750 sub_867750
    Lifted::new(0x7000f53960, fn_7000f53960 as *const (), 0), // +0x867960 sub_867960
    Lifted::new(0x7000f53ce0, fn_7000f53ce0 as *const (), 0), // +0x867ce0 sub_867ce0
    Lifted::new(0x7000f53e60, fn_7000f53e60 as *const (), 0), // +0x867e60 sub_867e60
    Lifted::new(0x7000f54080, fn_7000f54080 as *const (), 0), // +0x868080 sub_868080
    Lifted::new(0x7000f54360, fn_7000f54360 as *const (), 0), // +0x868360 sub_868360
    Lifted::new(0x7000f54650, fn_7000f54650 as *const (), 0), // +0x868650 sub_868650
    Lifted::new(0x7000f54690, fn_7000f54690 as *const (), 0), // +0x868690 sub_868690
    Lifted::new(0x7000f54770, fn_7000f54770 as *const (), 0), // +0x868770 sub_868770
    Lifted::new(0x7000f54860, fn_7000f54860 as *const (), 0), // +0x868860 sub_868860
    Lifted::new(0x7000f54970, fn_7000f54970 as *const (), 0), // +0x868970 sub_868970
    Lifted::new(0x7000f549a0, fn_7000f549a0 as *const (), 1), // +0x8689a0 sub_8689a0
    Lifted::new(0x7000f549c0, fn_7000f549c0 as *const (), 2), // +0x8689c0 sub_8689c0
    Lifted::new(0x7000f54d80, fn_7000f54d80 as *const (), 2), // +0x868d80 sub_868d80
    Lifted::new(0x7000f55400, fn_7000f55400 as *const (), 0), // +0x869400 sub_869400
    Lifted::new(0x7000f55530, fn_7000f55530 as *const (), 0), // +0x869530 sub_869530
    Lifted::new(0x7000f555e0, fn_7000f555e0 as *const (), 0), // +0x8695e0 sub_8695e0
    Lifted::new(0x7000f55610, fn_7000f55610 as *const (), 0), // +0x869610 sub_869610
    Lifted::new(0x7000f55750, tpl_914::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000f557d7> as *const (), 0), // +0x869750 sub_869750
    Lifted::new(0x7000f55800, tpl_287 as *const (), 1), // +0x869800 sub_869800
    Lifted::new(0x7000f55b20, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f55b30> as *const (), 0), // +0x869b20 sub_869b20
    Lifted::new(0x7000f55b40, fn_7000f55b40 as *const (), 2), // +0x869b40 sub_869b40
    Lifted::new(0x7000f55c50, fn_7000f55c50 as *const (), 0), // +0x869c50 sub_869c50
    Lifted::new(0x7000f55d10, fn_7000f55d10 as *const (), 0), // +0x869d10 sub_869d10
    Lifted::new(0x7000f55fa0, fn_7000f55fa0 as *const (), 0), // +0x869fa0 sub_869fa0
    Lifted::new(0x7000f56240, fn_7000f56240 as *const (), 0), // +0x86a240 sub_86a240
    Lifted::new(0x7000f56260, fn_7000f56260 as *const (), 0), // +0x86a260 sub_86a260
    Lifted::new(0x7000f56480, fn_7000f56480 as *const (), 0), // +0x86a480 sub_86a480
    Lifted::new(0x7000f56510, fn_7000f56510 as *const (), 0), // +0x86a510 sub_86a510
    Lifted::new(0x7000f56560, fn_7000f56560 as *const (), 0), // +0x86a560 sub_86a560
    Lifted::new(0x7000f565b0, fn_7000f565b0 as *const (), 1), // +0x86a5b0 sub_86a5b0
    Lifted::new(0x7000f56600, fn_7000f56600 as *const (), 0), // +0x86a600 sub_86a600
    Lifted::new(0x7000f566a0, fn_7000f566a0 as *const (), 0), // +0x86a6a0 sub_86a6a0
    Lifted::new(0x7000f567a0, fn_7000f567a0 as *const (), 0), // +0x86a7a0 sub_86a7a0
    Lifted::new(0x7000f56840, fn_7000f56840 as *const (), 0), // +0x86a840 sub_86a840
    Lifted::new(0x7000f56870, fn_7000f56870 as *const (), 0), // +0x86a870 sub_86a870
    Lifted::new(0x7000f56960, fn_7000f56960 as *const (), 3), // +0x86a960 sub_86a960
    Lifted::new(0x7000f569c0, fn_7000f569c0 as *const (), 3), // +0x86a9c0 sub_86a9c0
    Lifted::new(0x7000f56a20, fn_7000f56a20 as *const (), 2), // +0x86aa20 sub_86aa20
    Lifted::new(0x7000f56a90, fn_7000f56a90 as *const (), 0), // +0x86aa90 sub_86aa90
    Lifted::new(0x7000f56c40, fn_7000f56c40 as *const (), 0), // +0x86ac40 sub_86ac40
    Lifted::new(0x7000f56cb0, fn_7000f56cb0 as *const (), 0), // +0x86acb0 sub_86acb0
    Lifted::new(0x7000f56d80, fn_7000f56d80 as *const (), 0), // +0x86ad80 sub_86ad80
    Lifted::new(0x7000f56e60, tpl_55::<0x700177c0d0> as *const (), 0), // +0x86ae60 sub_86ae60
    Lifted::new(0x7000f56ea0, tpl_392::<0x700177c0e0> as *const (), 0), // +0x86aea0 sub_86aea0
    Lifted::new(0x7000f56ed0, fn_7000f56ed0 as *const (), 0), // +0x86aed0 sub_86aed0
    Lifted::new(0x7000f56fa0, fn_7000f56fa0 as *const (), 0), // +0x86afa0 sub_86afa0
    Lifted::new(0x7000f57060, fn_7000f57060 as *const (), 0), // +0x86b060 sub_86b060
    Lifted::new(0x7000f570f0, fn_7000f570f0 as *const (), 1), // +0x86b0f0 sub_86b0f0
    Lifted::new(0x7000f57150, fn_7000f57150 as *const (), 0), // +0x86b150 sub_86b150
    Lifted::new(0x7000f571d0, fn_7000f571d0 as *const (), 0), // +0x86b1d0 sub_86b1d0
    Lifted::new(0x7000f573a0, tpl_795::<0x8, 0x700177c312> as *const (), 0), // +0x86b3a0 sub_86b3a0
    Lifted::new(0x7000f573f0, tpl_795::<0x8, 0x700177c312> as *const (), 0), // +0x86b3f0 sub_86b3f0
    Lifted::new(0x7000f57440, tpl_33::<0x700177c312> as *const (), 0), // +0x86b440 sub_86b440
    Lifted::new(0x7000f57470, tpl_795::<0x8, 0x700177c312> as *const (), 0), // +0x86b470 sub_86b470
    Lifted::new(0x7000f574c0, tpl_795::<0x8, 0x700177c312> as *const (), 0), // +0x86b4c0 sub_86b4c0
    Lifted::new(0x7000f57510, fn_7000f57510 as *const (), 1), // +0x86b510 sub_86b510
    Lifted::new(0x7000f57530, fn_7000f57530 as *const (), 0), // +0x86b530 sub_86b530
    Lifted::new(0x7000f57560, fn_7000f57560 as *const (), 0), // +0x86b560 sub_86b560
    Lifted::new(0x7000f57600, tpl_1678::<0x10, 0x700071d970, 0x8, 0x700177c0b0> as *const (), 0), // +0x86b600 sub_86b600
    Lifted::new(0x7000f57670, tpl_1678::<0x10, 0x7000758870, 0x8, 0x700177c0b0> as *const (), 0), // +0x86b670 sub_86b670
    Lifted::new(0x7000f576e0, fn_7000f576e0 as *const (), 0), // +0x86b6e0 sub_86b6e0
    Lifted::new(0x7000f57780, fn_7000f57780 as *const (), 2), // +0x86b780 sub_86b780
    Lifted::new(0x7000f57830, fn_7000f57830 as *const (), 0), // +0x86b830 sub_86b830
    Lifted::new(0x7000f57910, fn_7000f57910 as *const (), 0), // +0x86b910 sub_86b910
    Lifted::new(0x7000f579d0, fn_7000f579d0 as *const (), 0), // +0x86b9d0 sub_86b9d0
    Lifted::new(0x7000f57aa0, tpl_1679::<0x10, 0x18, 0x700076bf20, 0x700185a7a8, 0x8> as *const (), 0), // +0x86baa0 sub_86baa0
    Lifted::new(0x7000f57b20, tpl_1679::<0x10, 0x18, 0x700076bf20, 0x700185a7e8, 0x8> as *const (), 0), // +0x86bb20 sub_86bb20
    Lifted::new(0x7000f57ba0, fn_7000f57ba0 as *const (), 0), // +0x86bba0 sub_86bba0
    Lifted::new(0x7000f57c20, fn_7000f57c20 as *const (), 0), // +0x86bc20 sub_86bc20
    Lifted::new(0x7000f57d50, fn_7000f57d50 as *const (), 0), // +0x86bd50 sub_86bd50
    Lifted::new(0x7000f57e10, fn_7000f57e10 as *const (), 0), // +0x86be10 sub_86be10
    Lifted::new(0x7000f58110, fn_7000f58110 as *const (), 0), // +0x86c110 sub_86c110
    Lifted::new(0x7000f582a0, tpl_1293::<0x20> as *const (), 1), // +0x86c2a0 sub_86c2a0
    Lifted::new(0x7000f582c0, tpl_191::<0x20> as *const (), 1), // +0x86c2c0 sub_86c2c0
    Lifted::new(0x7000f582d0, fn_7000f582d0 as *const (), 1), // +0x86c2d0 sub_86c2d0
    Lifted::new(0x7000f582f0, fn_7000f582f0 as *const (), 1), // +0x86c2f0 sub_86c2f0
    Lifted::new(0x7000f58310, tpl_172::<0x38> as *const (), 1), // +0x86c310 sub_86c310
    Lifted::new(0x7000f58320, fn_7000f58320 as *const (), 0), // +0x86c320 sub_86c320
    Lifted::new(0x7000f583a0, fn_7000f583a0 as *const (), 0), // +0x86c3a0 sub_86c3a0
    Lifted::new(0x7000f58420, fn_7000f58420 as *const (), 0), // +0x86c420 sub_86c420
    Lifted::new(0x7000f58480, tpl_1680::<0x8, 0x7000f58580, 0x10> as *const (), 0), // +0x86c480 sub_86c480
    Lifted::new(0x7000f58500, tpl_1680::<0x8, 0x7000f58870, 0x10> as *const (), 0), // +0x86c500 sub_86c500
    Lifted::new(0x7000f58580, fn_7000f58580 as *const (), 0), // +0x86c580 sub_86c580
    Lifted::new(0x7000f58870, fn_7000f58870 as *const (), 0), // +0x86c870 sub_86c870
    Lifted::new(0x7000f58b90, fn_7000f58b90 as *const (), 0), // +0x86cb90 sub_86cb90
    Lifted::new(0x7000f58c20, fn_7000f58c20 as *const (), 0), // +0x86cc20 sub_86cc20
    Lifted::new(0x7000f58cd0, tpl_1681::<0x18, 0x8, 0x7000f58580> as *const (), 0), // +0x86ccd0 sub_86ccd0
    Lifted::new(0x7000f58d50, tpl_1681::<0x18, 0x8, 0x7000f58870> as *const (), 0), // +0x86cd50 sub_86cd50
    Lifted::new(0x7000f58dd0, fn_7000f58dd0 as *const (), 0), // +0x86cdd0 sub_86cdd0
    Lifted::new(0x7000f58ec0, fn_7000f58ec0 as *const (), 0), // +0x86cec0 sub_86cec0
    Lifted::new(0x7000f59090, fn_7000f59090 as *const (), 1), // +0x86d090 sub_86d090
    Lifted::new(0x7000f59310, fn_7000f59310 as *const (), 0), // +0x86d310 sub_86d310
    Lifted::new(0x7000f59400, fn_7000f59400 as *const (), 0), // +0x86d400 sub_86d400
    Lifted::new(0x7000f59520, fn_7000f59520 as *const (), 0), // +0x86d520 sub_86d520
    Lifted::new(0x7000f595d0, tpl_1326::<0x8, 0x70007286c0> as *const (), 0), // +0x86d5d0 sub_86d5d0
    Lifted::new(0x7000f59600, tpl_1326::<0x8, 0x700073e110> as *const (), 0), // +0x86d600 sub_86d600
    Lifted::new(0x7000f59630, tpl_143::<0x8, 0x700070f5c0> as *const (), 0), // +0x86d630 sub_86d630
    Lifted::new(0x7000f59640, tpl_191::<0x8> as *const (), 1), // +0x86d640 sub_86d640
    Lifted::new(0x7000f59650, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f59661> as *const (), 0), // +0x86d650 sub_86d650
    Lifted::new(0x7000f59670, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f59681> as *const (), 0), // +0x86d670 sub_86d670
    Lifted::new(0x7000f59690, tpl_1682::<0x700071d970, 0x700177c0b0, 0x70007286c0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x86d690 sub_86d690
    Lifted::new(0x7000f59850, tpl_1682::<0x7000758870, 0x700177c0b0, 0x700073e110, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x86d850 sub_86d850
    Lifted::new(0x7000f59a10, fn_7000f59a10 as *const (), 0), // +0x86da10 sub_86da10
    Lifted::new(0x7000f59ad0, tpl_1683::<0x700071d970, 0x700177c0b0, 0x7000f59ad0, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x86dad0 sub_86dad0
    Lifted::new(0x7000f59c90, tpl_1683::<0x7000758870, 0x700177c0b0, 0x7000f59c90, 0x7000767d50, 0x700074a010> as *const (), 2), // +0x86dc90 sub_86dc90
    Lifted::new(0x7000f59e50, tpl_1326::<0x8, 0x7000f59ad0> as *const (), 0), // +0x86de50 sub_86de50
    Lifted::new(0x7000f59e80, tpl_1326::<0x8, 0x7000f59c90> as *const (), 0), // +0x86de80 sub_86de80
    Lifted::new(0x7000f59eb0, tpl_36::<0x8> as *const (), 1), // +0x86deb0 sub_86deb0
    Lifted::new(0x7000f59ec0, fn_7000f59ec0 as *const (), 1), // +0x86dec0 sub_86dec0
    Lifted::new(0x7000f59ef0, tpl_51::<0x8> as *const (), 1), // +0x86def0 sub_86def0
    Lifted::new(0x7000f59f00, fn_7000f59f00 as *const (), 0), // +0x86df00 sub_86df00
    Lifted::new(0x7000f5a020, fn_7000f5a020 as *const (), 1), // +0x86e020 sub_86e020
    Lifted::new(0x7000f5a080, fn_7000f5a080 as *const (), 1), // +0x86e080 sub_86e080
    Lifted::new(0x7000f5a0b0, fn_7000f5a0b0 as *const (), 1), // +0x86e0b0 sub_86e0b0
    Lifted::new(0x7000f5a110, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f5a120> as *const (), 0), // +0x86e110 sub_86e110
    Lifted::new(0x7000f5a130, tpl_611::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000f5a16a, 0x70007f0910, 0x7000f5a188> as *const (), 0), // +0x86e130 sub_86e130
    Lifted::new(0x7000f5a1a0, fn_7000f5a1a0 as *const (), 0), // +0x86e1a0 sub_86e1a0
    Lifted::new(0x7000f5a470, tpl_39::<0x10> as *const (), 1), // +0x86e470 sub_86e470
    Lifted::new(0x7000f5a480, fn_7000f5a480 as *const (), 0), // +0x86e480 sub_86e480
    Lifted::new(0x7000f5a4e0, fn_7000f5a4e0 as *const (), 0), // +0x86e4e0 sub_86e4e0
    Lifted::new(0x7000f5a540, fn_7000f5a540 as *const (), 0), // +0x86e540 sub_86e540
    Lifted::new(0x7000f5a5f0, fn_7000f5a5f0 as *const (), 0), // +0x86e5f0 sub_86e5f0
    Lifted::new(0x7000f5a620, fn_7000f5a620 as *const (), 2), // +0x86e620 sub_86e620
    Lifted::new(0x7000f5a940, fn_7000f5a940 as *const (), 1), // +0x86e940 sub_86e940
    Lifted::new(0x7000f5aaf0, tpl_422::<0x14, 0x10, 0x30, 0x8, 0x38, 0x1c, 0x18, 0x34, 0xc, 0x40> as *const (), 1), // +0x86eaf0 sub_86eaf0
    Lifted::new(0x7000f5ab70, fn_7000f5ab70 as *const (), 0), // +0x86eb70 sub_86eb70
    Lifted::new(0x7000f5abe0, fn_7000f5abe0 as *const (), 0), // +0x86ebe0 sub_86ebe0
    Lifted::new(0x7000f5adf0, fn_7000f5adf0 as *const (), 0), // +0x86edf0 sub_86edf0
    Lifted::new(0x7000f5aea0, fn_7000f5aea0 as *const (), 0), // +0x86eea0 sub_86eea0
    Lifted::new(0x7000f5af90, fn_7000f5af90 as *const (), 0), // +0x86ef90 sub_86ef90
    Lifted::new(0x7000f5b020, fn_7000f5b020 as *const (), 0), // +0x86f020 sub_86f020
    Lifted::new(0x7000f5b160, fn_7000f5b160 as *const (), 2), // +0x86f160 sub_86f160
    Lifted::new(0x7000f5b4c0, fn_7000f5b4c0 as *const (), 2), // +0x86f4c0 sub_86f4c0
    Lifted::new(0x7000f5b6d0, fn_7000f5b6d0 as *const (), 0), // +0x86f6d0 sub_86f6d0
    Lifted::new(0x7000f5b820, fn_7000f5b820 as *const (), 0), // +0x86f820 sub_86f820
    Lifted::new(0x7000f5ba10, fn_7000f5ba10 as *const (), 0), // +0x86fa10 sub_86fa10
    Lifted::new(0x7000f5bd10, fn_7000f5bd10 as *const (), 0), // +0x86fd10 sub_86fd10
    Lifted::new(0x7000f5bea0, fn_7000f5bea0 as *const (), 0), // +0x86fea0 sub_86fea0
    Lifted::new(0x7000f5c200, tpl_445::<0xc8, 0x700070f540, 0x7000f72cf0> as *const (), 0), // +0x870200 sub_870200
    Lifted::new(0x7000f5c250, tpl_446::<0xc8, 0x700070f540, 0x7000f72cf0, 0x28, 0x700075f490, 0x10, 0x38, 0x18, 0x1c, 0xfffffffffffffffe, 0x7000f72bd0> as *const (), 0), // +0x870250 sub_870250
    Lifted::new(0x7000f5c370, tpl_447::<0xc8, 0x18, 0x38, 0x700070f540, 0x4c, 0x10, 0x30, 0x700073c730, 0x82, 0xd0, 0x7000f72b40, 0x7000f72cf0> as *const (), 0), // +0x870370 sub_870370
    Lifted::new(0x7000f5c450, fn_7000f5c450 as *const (), 0), // +0x870450 sub_870450
    Lifted::new(0x7000f5c610, fn_7000f5c610 as *const (), 0), // +0x870610 sub_870610
    Lifted::new(0x7000f5c720, fn_7000f5c720 as *const (), 1), // +0x870720 sub_870720
    Lifted::new(0x7000f5c770, tpl_491::<0x10, 0x28, 0x20, 0x18, 0x4, 0x8, 0x7000a2c5c0> as *const (), 0), // +0x870770 sub_870770
    Lifted::new(0x7000f5c7c0, tpl_497::<0x88, 0x10, 0x18, 0x700076bf20, 0xd8, 0x7000766820, 0x43, 0x28, 0x7000726da0> as *const (), 0), // +0x8707c0 sub_8707c0
    Lifted::new(0x7000f5c8b0, fn_7000f5c8b0 as *const (), 0), // +0x8708b0 sub_8708b0
    Lifted::new(0x7000f5c960, tpl_498::<0x88, 0x10, 0x18, 0x700076bf20, 0xd8, 0x7000766820> as *const (), 0), // +0x870960 sub_870960
    Lifted::new(0x7000f5ca00, fn_7000f5ca00 as *const (), 2), // +0x870a00 sub_870a00
    Lifted::new(0x7000f5cc10, fn_7000f5cc10 as *const (), 2), // +0x870c10 sub_870c10
    Lifted::new(0x7000f5d120, fn_7000f5d120 as *const (), 2), // +0x871120 sub_871120
    Lifted::new(0x7000f5d650, fn_7000f5d650 as *const (), 2), // +0x871650 sub_871650
    Lifted::new(0x7000f5da70, fn_7000f5da70 as *const (), 0), // +0x871a70 sub_871a70
    Lifted::new(0x7000f5dc20, fn_7000f5dc20 as *const (), 0), // +0x871c20 sub_871c20
    Lifted::new(0x7000f5e4f0, fn_7000f5e4f0 as *const (), 0), // +0x8724f0 sub_8724f0
    Lifted::new(0x7000f5e6b0, fn_7000f5e6b0 as *const (), 0), // +0x8726b0 sub_8726b0
    Lifted::new(0x7000f5e7a0, fn_7000f5e7a0 as *const (), 0), // +0x8727a0 sub_8727a0
    Lifted::new(0x7000f5ea70, fn_7000f5ea70 as *const (), 0), // +0x872a70 sub_872a70
    Lifted::new(0x7000f5ed80, fn_7000f5ed80 as *const (), 0), // +0x872d80 sub_872d80
    Lifted::new(0x7000f5f0b0, fn_7000f5f0b0 as *const (), 2), // +0x8730b0 sub_8730b0
    Lifted::new(0x7000f5f4a0, fn_7000f5f4a0 as *const (), 0), // +0x8734a0 sub_8734a0
    Lifted::new(0x7000f5f500, fn_7000f5f500 as *const (), 2), // +0x873500 sub_873500
    Lifted::new(0x7000f5f6b0, fn_7000f5f6b0 as *const (), 2), // +0x8736b0 sub_8736b0
    Lifted::new(0x7000f5f9d0, fn_7000f5f9d0 as *const (), 2), // +0x8739d0 sub_8739d0
    Lifted::new(0x7000f5fd90, fn_7000f5fd90 as *const (), 2), // +0x873d90 sub_873d90
    Lifted::new(0x7000f60980, fn_7000f60980 as *const (), 2), // +0x874980 sub_874980
    Lifted::new(0x7000f60e30, fn_7000f60e30 as *const (), 0), // +0x874e30 sub_874e30
    Lifted::new(0x7000f61030, fn_7000f61030 as *const (), 0), // +0x875030 sub_875030
    Lifted::new(0x7000f61370, fn_7000f61370 as *const (), 0), // +0x875370 sub_875370
    Lifted::new(0x7000f61790, fn_7000f61790 as *const (), 2), // +0x875790 sub_875790
    Lifted::new(0x7000f61ab0, fn_7000f61ab0 as *const (), 0), // +0x875ab0 sub_875ab0
    Lifted::new(0x7000f61b00, fn_7000f61b00 as *const (), 0), // +0x875b00 sub_875b00
    Lifted::new(0x7000f61e70, fn_7000f61e70 as *const (), 2), // +0x875e70 sub_875e70
    Lifted::new(0x7000f62640, fn_7000f62640 as *const (), 0), // +0x876640 sub_876640
    Lifted::new(0x7000f62b70, fn_7000f62b70 as *const (), 2), // +0x876b70 sub_876b70
    Lifted::new(0x7000f63870, fn_7000f63870 as *const (), 0), // +0x877870 sub_877870
    Lifted::new(0x7000f63fe0, fn_7000f63fe0 as *const (), 0), // +0x877fe0 sub_877fe0
    Lifted::new(0x7000f644b0, fn_7000f644b0 as *const (), 2), // +0x8784b0 sub_8784b0
    Lifted::new(0x7000f652a0, fn_7000f652a0 as *const (), 2), // +0x8792a0 sub_8792a0
    Lifted::new(0x7000f657c0, fn_7000f657c0 as *const (), 0), // +0x8797c0 sub_8797c0
    Lifted::new(0x7000f66790, fn_7000f66790 as *const (), 0), // +0x87a790 sub_87a790
    Lifted::new(0x7000f668c0, fn_7000f668c0 as *const (), 2), // +0x87a8c0 sub_87a8c0
    Lifted::new(0x7000f66ad0, fn_7000f66ad0 as *const (), 2), // +0x87aad0 sub_87aad0
    Lifted::new(0x7000f670b0, fn_7000f670b0 as *const (), 2), // +0x87b0b0 sub_87b0b0
    Lifted::new(0x7000f67410, fn_7000f67410 as *const (), 0), // +0x87b410 sub_87b410
    Lifted::new(0x7000f674d0, fn_7000f674d0 as *const (), 0), // +0x87b4d0 sub_87b4d0
    Lifted::new(0x7000f67800, fn_7000f67800 as *const (), 0), // +0x87b800 sub_87b800
    Lifted::new(0x7000f67c20, fn_7000f67c20 as *const (), 0), // +0x87bc20 sub_87bc20
    Lifted::new(0x7000f687c0, fn_7000f687c0 as *const (), 0), // +0x87c7c0 sub_87c7c0
    Lifted::new(0x7000f68a50, fn_7000f68a50 as *const (), 0), // +0x87ca50 sub_87ca50
    Lifted::new(0x7000f68dd0, fn_7000f68dd0 as *const (), 0), // +0x87cdd0 sub_87cdd0
    Lifted::new(0x7000f69260, fn_7000f69260 as *const (), 0), // +0x87d260 sub_87d260
    Lifted::new(0x7000f69500, fn_7000f69500 as *const (), 0), // +0x87d500 sub_87d500
    Lifted::new(0x7000f69640, fn_7000f69640 as *const (), 0), // +0x87d640 sub_87d640
    Lifted::new(0x7000f69820, fn_7000f69820 as *const (), 2), // +0x87d820 sub_87d820
    Lifted::new(0x7000f69d00, fn_7000f69d00 as *const (), 0), // +0x87dd00 sub_87dd00
    Lifted::new(0x7000f6a670, fn_7000f6a670 as *const (), 2), // +0x87e670 sub_87e670
    Lifted::new(0x7000f6aac0, fn_7000f6aac0 as *const (), 0), // +0x87eac0 sub_87eac0
    Lifted::new(0x7000f6abf0, fn_7000f6abf0 as *const (), 2), // +0x87ebf0 sub_87ebf0
    Lifted::new(0x7000f6c1d0, fn_7000f6c1d0 as *const (), 2), // +0x8801d0 sub_8801d0
    Lifted::new(0x7000f6cbb0, fn_7000f6cbb0 as *const (), 0), // +0x880bb0 sub_880bb0
    Lifted::new(0x7000f6cc80, fn_7000f6cc80 as *const (), 2), // +0x880c80 sub_880c80
    Lifted::new(0x7000f6cdf0, fn_7000f6cdf0 as *const (), 0), // +0x880df0 sub_880df0
    Lifted::new(0x7000f6ced0, fn_7000f6ced0 as *const (), 0), // +0x880ed0 sub_880ed0
    Lifted::new(0x7000f6d4d0, fn_7000f6d4d0 as *const (), 0), // +0x8814d0 sub_8814d0
    Lifted::new(0x7000f6d620, fn_7000f6d620 as *const (), 0), // +0x881620 sub_881620
    Lifted::new(0x7000f6db90, fn_7000f6db90 as *const (), 0), // +0x881b90 sub_881b90
    Lifted::new(0x7000f6def0, fn_7000f6def0 as *const (), 0), // +0x881ef0 sub_881ef0
    Lifted::new(0x7000f6ed50, fn_7000f6ed50 as *const (), 0), // +0x882d50 sub_882d50
    Lifted::new(0x7000f6f030, fn_7000f6f030 as *const (), 0), // +0x883030 sub_883030
    Lifted::new(0x7000f6f2f0, fn_7000f6f2f0 as *const (), 0), // +0x8832f0 sub_8832f0
    Lifted::new(0x7000f6f740, fn_7000f6f740 as *const (), 0), // +0x883740 sub_883740
    Lifted::new(0x7000f6fe60, fn_7000f6fe60 as *const (), 0), // +0x883e60 sub_883e60
    Lifted::new(0x7000f70130, fn_7000f70130 as *const (), 0), // +0x884130 sub_884130
    Lifted::new(0x7000f703c0, fn_7000f703c0 as *const (), 0), // +0x8843c0 sub_8843c0
    Lifted::new(0x7000f70740, fn_7000f70740 as *const (), 0), // +0x884740 sub_884740
    Lifted::new(0x7000f70900, fn_7000f70900 as *const (), 2), // +0x884900 sub_884900
    Lifted::new(0x7000f713b0, fn_7000f713b0 as *const (), 2), // +0x8853b0 sub_8853b0
    Lifted::new(0x7000f714d0, fn_7000f714d0 as *const (), 0), // +0x8854d0 sub_8854d0
    Lifted::new(0x7000f717e0, fn_7000f717e0 as *const (), 0), // +0x8857e0 sub_8857e0
    Lifted::new(0x7000f71b10, fn_7000f71b10 as *const (), 0), // +0x885b10 sub_885b10
    Lifted::new(0x7000f71d50, tpl_503::<0xc8, 0x38, 0x18, 0x1c, 0x28, 0x700072add0> as *const (), 0), // +0x885d50 sub_885d50
    Lifted::new(0x7000f71dd0, fn_7000f71dd0 as *const (), 0), // +0x885dd0 sub_885dd0
    Lifted::new(0x7000f72470, fn_7000f72470 as *const (), 0), // +0x886470 sub_886470
    Lifted::new(0x7000f72940, fn_7000f72940 as *const (), 1), // +0x886940 sub_886940
    Lifted::new(0x7000f72980, fn_7000f72980 as *const (), 0), // +0x886980 sub_886980
    Lifted::new(0x7000f729e0, fn_7000f729e0 as *const (), 0), // +0x8869e0 sub_8869e0
    Lifted::new(0x7000f72a90, fn_7000f72a90 as *const (), 0), // +0x886a90 sub_886a90
    Lifted::new(0x7000f72b40, fn_7000f72b40 as *const (), 0), // +0x886b40 sub_886b40
    Lifted::new(0x7000f72bd0, tpl_1684::<0x80, 0x7000f56960, 0xc8, 0x28, 0x30, 0x18, 0x10, 0x38, 0xf0> as *const (), 0), // +0x886bd0 sub_886bd0
    Lifted::new(0x7000f72c60, tpl_1684::<0x80, 0x7000f569c0, 0xc8, 0x28, 0x30, 0x18, 0x10, 0x38, 0xf0> as *const (), 0), // +0x886c60 sub_886c60
    Lifted::new(0x7000f72cf0, fn_7000f72cf0 as *const (), 1), // +0x886cf0 sub_886cf0
    Lifted::new(0x7000f72d60, fn_7000f72d60 as *const (), 0), // +0x886d60 sub_886d60
    Lifted::new(0x7000f72fa0, tpl_521::<0x28, 0x20, 0xffffffffffffffff, 0x18, 0x8> as *const (), 1), // +0x886fa0 sub_886fa0
    Lifted::new(0x7000f73020, fn_7000f73020 as *const (), 0), // +0x887020 sub_887020
    Lifted::new(0x7000f731d0, tpl_528::<0xf0, 0xf8, 0x38, 0x4, 0x30, 0x7000f729e0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x700075ff30> as *const (), 0), // +0x8871d0 sub_8871d0
    Lifted::new(0x7000f732c0, fn_7000f732c0 as *const (), 1), // +0x8872c0 sub_8872c0
    Lifted::new(0x7000f733b0, fn_7000f733b0 as *const (), 0), // +0x8873b0 sub_8873b0
    Lifted::new(0x7000f73570, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f73580> as *const (), 0), // +0x887570 sub_887570
    Lifted::new(0x7000f73590, tpl_541::<0x18, 0x30, 0x700073c730, 0x82, 0xd0> as *const (), 0), // +0x887590 sub_887590
    Lifted::new(0x7000f73610, tpl_542::<0xd0, 0x18, 0x30, 0x700073c730, 0x82, 0xfffffffffffffff4> as *const (), 0), // +0x887610 sub_887610
    Lifted::new(0x7000f73660, fn_7000f73660 as *const (), 0), // +0x887660 sub_887660
    Lifted::new(0x7000f737f0, fn_7000f737f0 as *const (), 0), // +0x8877f0 sub_8877f0
    Lifted::new(0x7000f73890, fn_7000f73890 as *const (), 2), // +0x887890 sub_887890
    Lifted::new(0x7000f73960, fn_7000f73960 as *const (), 0), // +0x887960 sub_887960
    Lifted::new(0x7000f73bf0, fn_7000f73bf0 as *const (), 0), // +0x887bf0 sub_887bf0
    Lifted::new(0x7000f73ca0, fn_7000f73ca0 as *const (), 0), // +0x887ca0 sub_887ca0
    Lifted::new(0x7000f73d00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73d11> as *const (), 0), // +0x887d00 sub_887d00
    Lifted::new(0x7000f73d20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73d31> as *const (), 0), // +0x887d20 sub_887d20
    Lifted::new(0x7000f73d40, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73d51> as *const (), 0), // +0x887d40 sub_887d40
    Lifted::new(0x7000f73d60, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73d71> as *const (), 0), // +0x887d60 sub_887d60
    Lifted::new(0x7000f73d80, fn_7000f73d80 as *const (), 0), // +0x887d80 sub_887d80
    Lifted::new(0x7000f73e10, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73e21> as *const (), 0), // +0x887e10 sub_887e10
    Lifted::new(0x7000f73e30, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73e41> as *const (), 0), // +0x887e30 sub_887e30
    Lifted::new(0x7000f73e50, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73e61> as *const (), 0), // +0x887e50 sub_887e50
    Lifted::new(0x7000f73e70, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73e81> as *const (), 0), // +0x887e70 sub_887e70
    Lifted::new(0x7000f73e90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73ea1> as *const (), 0), // +0x887e90 sub_887e90
    Lifted::new(0x7000f73eb0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73ec1> as *const (), 0), // +0x887eb0 sub_887eb0
    Lifted::new(0x7000f73ed0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73ee1> as *const (), 0), // +0x887ed0 sub_887ed0
    Lifted::new(0x7000f73f00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73f11> as *const (), 0), // +0x887f00 sub_887f00
    Lifted::new(0x7000f73f30, tpl_1685::<0x53, 0x8, 0x7000f73ca0, 0x7000f73660, 0x52> as *const (), 0), // +0x887f30 sub_887f30
    Lifted::new(0x7000f73fa0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f73fb1> as *const (), 0), // +0x887fa0 sub_887fa0
    Lifted::new(0x7000f73fd0, fn_7000f73fd0 as *const (), 0), // +0x887fd0 sub_887fd0
    Lifted::new(0x7000f74030, tpl_1685::<0x53, 0x8, 0x7000f73ca0, 0x7000f73660, 0x52> as *const (), 0), // +0x888030 sub_888030
    Lifted::new(0x7000f740a0, tpl_1685::<0x53, 0x8, 0x7000f73ca0, 0x7000f73660, 0x52> as *const (), 0), // +0x8880a0 sub_8880a0
    Lifted::new(0x7000f74110, tpl_1685::<0x53, 0x8, 0x7000f73ca0, 0x7000f73660, 0x52> as *const (), 0), // +0x888110 sub_888110
    Lifted::new(0x7000f74180, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74191> as *const (), 0), // +0x888180 sub_888180
    Lifted::new(0x7000f741a0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f741b1> as *const (), 0), // +0x8881a0 sub_8881a0
    Lifted::new(0x7000f741c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f741d1> as *const (), 0), // +0x8881c0 sub_8881c0
    Lifted::new(0x7000f741e0, fn_7000f741e0 as *const (), 0), // +0x8881e0 sub_8881e0
    Lifted::new(0x7000f74290, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f742a1> as *const (), 0), // +0x888290 sub_888290
    Lifted::new(0x7000f742d0, tpl_1685::<0x53, 0x8, 0x7000f73ca0, 0x7000f73660, 0x52> as *const (), 0), // +0x8882d0 sub_8882d0
    Lifted::new(0x7000f74340, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74351> as *const (), 0), // +0x888340 sub_888340
    Lifted::new(0x7000f74360, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74371> as *const (), 0), // +0x888360 sub_888360
    Lifted::new(0x7000f74380, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74391> as *const (), 0), // +0x888380 sub_888380
    Lifted::new(0x7000f743a0, fn_7000f743a0 as *const (), 0), // +0x8883a0 sub_8883a0
    Lifted::new(0x7000f74ac0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74ad1> as *const (), 0), // +0x888ac0 sub_888ac0
    Lifted::new(0x7000f74ae0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74af1> as *const (), 0), // +0x888ae0 sub_888ae0
    Lifted::new(0x7000f74b00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74b11> as *const (), 0), // +0x888b00 sub_888b00
    Lifted::new(0x7000f74b20, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74b31> as *const (), 0), // +0x888b20 sub_888b20
    Lifted::new(0x7000f74b50, fn_7000f74b50 as *const (), 0), // +0x888b50 sub_888b50
    Lifted::new(0x7000f74b90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74ba1> as *const (), 0), // +0x888b90 sub_888b90
    Lifted::new(0x7000f74bb0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74bc1> as *const (), 0), // +0x888bb0 sub_888bb0
    Lifted::new(0x7000f74bd0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74be1> as *const (), 0), // +0x888bd0 sub_888bd0
    Lifted::new(0x7000f74bf0, fn_7000f74bf0 as *const (), 0), // +0x888bf0 sub_888bf0
    Lifted::new(0x7000f74c90, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74ca1> as *const (), 0), // +0x888c90 sub_888c90
    Lifted::new(0x7000f74cb0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74cc1> as *const (), 0), // +0x888cb0 sub_888cb0
    Lifted::new(0x7000f74cd0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f74ce1> as *const (), 0), // +0x888cd0 sub_888cd0
    Lifted::new(0x7000f74cf0, fn_7000f74cf0 as *const (), 0), // +0x888cf0 sub_888cf0
    Lifted::new(0x7000f74da0, fn_7000f74da0 as *const (), 0), // +0x888da0 sub_888da0
    Lifted::new(0x7000f75050, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f75061> as *const (), 0), // +0x889050 sub_889050
    Lifted::new(0x7000f75070, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f75081> as *const (), 0), // +0x889070 sub_889070
    Lifted::new(0x7000f75090, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f750a1> as *const (), 0), // +0x889090 sub_889090
    Lifted::new(0x7000f750b0, tpl_1685::<0x53, 0x8, 0x7000f73ca0, 0x7000f73660, 0x52> as *const (), 0), // +0x8890b0 sub_8890b0
    Lifted::new(0x7000f75120, fn_7000f75120 as *const (), 0), // +0x889120 sub_889120
    Lifted::new(0x7000f75190, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f751a1> as *const (), 0), // +0x889190 sub_889190
    Lifted::new(0x7000f751b0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f751c1> as *const (), 0), // +0x8891b0 sub_8891b0
    Lifted::new(0x7000f751d0, tpl_20 as *const (), 1), // +0x8891d0 sub_8891d0
    Lifted::new(0x7000f751e0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8891e0 sub_8891e0
    Lifted::new(0x7000f75220, tpl_1686::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f7549d, 0x7000f78b90, 0x7000f754a3, 0x70007f0910, 0x7000f754a9> as *const (), 2), // +0x889220 sub_889220
    Lifted::new(0x7000f75550, tpl_1686::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f757cd, 0x7000f78bb0, 0x7000f757d3, 0x70007f0910, 0x7000f757d9> as *const (), 2), // +0x889550 sub_889550
    Lifted::new(0x7000f75880, fn_7000f75880 as *const (), 2), // +0x889880 sub_889880
    Lifted::new(0x7000f75b30, fn_7000f75b30 as *const (), 0), // +0x889b30 sub_889b30
    Lifted::new(0x7000f75b60, fn_7000f75b60 as *const (), 0), // +0x889b60 sub_889b60
    Lifted::new(0x7000f75b90, tpl_1687::<0x7000f75ce0, 0x700185b060, 0x38, 0x7000f75b30, 0x7000f75bbc> as *const (), 0), // +0x889b90 sub_889b90
    Lifted::new(0x7000f75bd0, tpl_1687::<0x7000f75ce0, 0x700185aba8, 0x38, 0x7000f75b60, 0x7000f75bfc> as *const (), 0), // +0x889bd0 sub_889bd0
    Lifted::new(0x7000f75c10, fn_7000f75c10 as *const (), 1), // +0x889c10 sub_889c10
    Lifted::new(0x7000f75c70, fn_7000f75c70 as *const (), 0), // +0x889c70 sub_889c70
    Lifted::new(0x7000f75ce0, fn_7000f75ce0 as *const (), 1), // +0x889ce0 sub_889ce0
    Lifted::new(0x7000f75d10, tpl_1688::<0x700185aeb8, 0x18, 0x10, 0x700177c312, 0x20, 0xfffffffffffffff8, 0x7001d54968, 0x7000f75dd1> as *const (), 0), // +0x889d10 sub_889d10
    Lifted::new(0x7000f75e10, tpl_1688::<0x700185aef8, 0x18, 0x10, 0x700177c312, 0x20, 0xfffffffffffffff8, 0x7001d54968, 0x7000f75ed1> as *const (), 0), // +0x889e10 sub_889e10
    Lifted::new(0x7000f75f10, tpl_937::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000f75f8a> as *const (), 0), // +0x889f10 sub_889f10
    Lifted::new(0x7000f75fc0, tpl_1689::<0x7000f75d10, 0x700177c312> as *const (), 0), // +0x889fc0 sub_889fc0
    Lifted::new(0x7000f76010, tpl_1690::<0x438, 0x20, 0x700177c312> as *const (), 0), // +0x88a010 sub_88a010
    Lifted::new(0x7000f76060, tpl_33::<0x700177c312> as *const (), 0), // +0x88a060 sub_88a060
    Lifted::new(0x7000f76090, tpl_33::<0x700177c312> as *const (), 0), // +0x88a090 sub_88a090
    Lifted::new(0x7000f760c0, fn_7000f760c0 as *const (), 0), // +0x88a0c0 sub_88a0c0
    Lifted::new(0x7000f76100, tpl_1690::<0x38, 0x20, 0x700177c312> as *const (), 0), // +0x88a100 sub_88a100
    Lifted::new(0x7000f76150, tpl_33::<0x700177c312> as *const (), 0), // +0x88a150 sub_88a150
    Lifted::new(0x7000f76180, tpl_33::<0x700177c312> as *const (), 0), // +0x88a180 sub_88a180
    Lifted::new(0x7000f761b0, tpl_33::<0x700177c312> as *const (), 0), // +0x88a1b0 sub_88a1b0
    Lifted::new(0x7000f761e0, fn_7000f761e0 as *const (), 0), // +0x88a1e0 sub_88a1e0
    Lifted::new(0x7000f76250, tpl_33::<0x700177c312> as *const (), 0), // +0x88a250 sub_88a250
    Lifted::new(0x7000f76280, fn_7000f76280 as *const (), 0), // +0x88a280 sub_88a280
    Lifted::new(0x7000f76330, tpl_1689::<0x7000f75d10, 0x700177c312> as *const (), 0), // +0x88a330 sub_88a330
    Lifted::new(0x7000f76380, tpl_995::<0x700185af08, 0x70017a0290, 0x7001d54ef8, 0x7000f76398> as *const (), 0), // +0x88a380 sub_88a380
    Lifted::new(0x7000f763a0, fn_7000f763a0 as *const (), 0), // +0x88a3a0 sub_88a3a0
    Lifted::new(0x7000f76470, tpl_995::<0x700185af08, 0x70017a0290, 0x7001d54ef8, 0x7000f76488> as *const (), 0), // +0x88a470 sub_88a470
    Lifted::new(0x7000f76490, fn_7000f76490 as *const (), 2), // +0x88a490 sub_88a490
    Lifted::new(0x7000f76520, fn_7000f76520 as *const (), 0), // +0x88a520 sub_88a520
    Lifted::new(0x7000f76570, fn_7000f76570 as *const (), 0), // +0x88a570 sub_88a570
    Lifted::new(0x7000f76630, fn_7000f76630 as *const (), 0), // +0x88a630 sub_88a630
    Lifted::new(0x7000f76680, fn_7000f76680 as *const (), 2), // +0x88a680 sub_88a680
    Lifted::new(0x7000f76710, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000f76721> as *const (), 0), // +0x88a710 sub_88a710
    Lifted::new(0x7000f76730, tpl_995::<0x700185af08, 0x70017a0290, 0x7001d54ef8, 0x7000f76748> as *const (), 0), // +0x88a730 sub_88a730
    Lifted::new(0x7000f76750, fn_7000f76750 as *const (), 2), // +0x88a750 sub_88a750
    Lifted::new(0x7000f76890, fn_7000f76890 as *const (), 0), // +0x88a890 sub_88a890
    Lifted::new(0x7000f769c0, fn_7000f769c0 as *const (), 0), // +0x88a9c0 sub_88a9c0
    Lifted::new(0x7000f76ec0, tpl_1691::<0x7001c73174, 0x70007081a0, 0x700074eed0, 0x70017d2c10, 0x7001d54ef8, 0x7000f770af> as *const (), 2), // +0x88aec0 sub_88aec0
    Lifted::new(0x7000f77150, tpl_1691::<0x7001c73174, 0x70007081a0, 0x700074eed0, 0x70017d2c10, 0x7001d54ef8, 0x7000f7733f> as *const (), 2), // +0x88b150 sub_88b150
    Lifted::new(0x7000f773e0, fn_7000f773e0 as *const (), 2), // +0x88b3e0 sub_88b3e0
    Lifted::new(0x7000f77630, fn_7000f77630 as *const (), 0), // +0x88b630 sub_88b630
    Lifted::new(0x7000f77650, fn_7000f77650 as *const (), 0), // +0x88b650 sub_88b650
    Lifted::new(0x7000f77b30, fn_7000f77b30 as *const (), 0), // +0x88bb30 sub_88bb30
    Lifted::new(0x7000f77c00, fn_7000f77c00 as *const (), 0), // +0x88bc00 sub_88bc00
    Lifted::new(0x7000f77cc0, fn_7000f77cc0 as *const (), 0), // +0x88bcc0 sub_88bcc0
    Lifted::new(0x7000f77d70, fn_7000f77d70 as *const (), 0), // +0x88bd70 sub_88bd70
    Lifted::new(0x7000f77de0, fn_7000f77de0 as *const (), 0), // +0x88bde0 sub_88bde0
    Lifted::new(0x7000f77e50, fn_7000f77e50 as *const (), 0), // +0x88be50 sub_88be50
    Lifted::new(0x7000f77f50, tpl_1692::<0x10, 0x38, 0x8, 0x20, 0x448, 0x440, 0x18, 0x7000a3b6d0> as *const (), 0), // +0x88bf50 sub_88bf50
    Lifted::new(0x7000f78010, fn_7000f78010 as *const (), 0), // +0x88c010 sub_88c010
    Lifted::new(0x7000f780e0, tpl_1692::<0x10, 0x38, 0x8, 0x20, 0x440, 0x438, 0x18, 0x7000a3b6d0> as *const (), 0), // +0x88c0e0 sub_88c0e0
    Lifted::new(0x7000f781a0, fn_7000f781a0 as *const (), 0), // +0x88c1a0 sub_88c1a0
    Lifted::new(0x7000f78240, tpl_1693::<0x10, 0x8, 0x20, 0x48, 0x40, 0x18> as *const (), 1), // +0x88c240 sub_88c240
    Lifted::new(0x7000f782a0, fn_7000f782a0 as *const (), 1), // +0x88c2a0 sub_88c2a0
    Lifted::new(0x7000f78330, tpl_1693::<0x10, 0x8, 0x20, 0x40, 0x38, 0x18> as *const (), 1), // +0x88c330 sub_88c330
    Lifted::new(0x7000f78390, fn_7000f78390 as *const (), 0), // +0x88c390 sub_88c390
    Lifted::new(0x7000f783f0, fn_7000f783f0 as *const (), 0), // +0x88c3f0 sub_88c3f0
    Lifted::new(0x7000f78510, fn_7000f78510 as *const (), 2), // +0x88c510 sub_88c510
    Lifted::new(0x7000f787c0, fn_7000f787c0 as *const (), 1), // +0x88c7c0 sub_88c7c0
    Lifted::new(0x7000f78b10, fn_7000f78b10 as *const (), 1), // +0x88cb10 sub_88cb10
    Lifted::new(0x7000f78b90, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f78ba0> as *const (), 0), // +0x88cb90 sub_88cb90
    Lifted::new(0x7000f78bb0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f78bc0> as *const (), 0), // +0x88cbb0 sub_88cbb0
    Lifted::new(0x7000f78bd0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f78be0> as *const (), 0), // +0x88cbd0 sub_88cbd0
    Lifted::new(0x7000f78bf0, tpl_144 as *const (), 1), // +0x88cbf0 sub_88cbf0
    Lifted::new(0x7000f78c00, tpl_144 as *const (), 1), // +0x88cc00 sub_88cc00
    Lifted::new(0x7000f78c10, tpl_44 as *const (), 1), // +0x88cc10 sub_88cc10
    Lifted::new(0x7000f78c20, tpl_144 as *const (), 1), // +0x88cc20 sub_88cc20
    Lifted::new(0x7000f78c30, tpl_144 as *const (), 1), // +0x88cc30 sub_88cc30
    Lifted::new(0x7000f78c40, tpl_144 as *const (), 1), // +0x88cc40 sub_88cc40
    Lifted::new(0x7000f78c50, tpl_44 as *const (), 1), // +0x88cc50 sub_88cc50
    Lifted::new(0x7000f78c60, tpl_144 as *const (), 1), // +0x88cc60 sub_88cc60
    Lifted::new(0x7000f78c70, tpl_144 as *const (), 1), // +0x88cc70 sub_88cc70
    Lifted::new(0x7000f78c80, tpl_144 as *const (), 1), // +0x88cc80 sub_88cc80
    Lifted::new(0x7000f78c90, tpl_144 as *const (), 1), // +0x88cc90 sub_88cc90
    Lifted::new(0x7000f78ca0, tpl_44 as *const (), 1), // +0x88cca0 sub_88cca0
    Lifted::new(0x7000f78cb0, tpl_144 as *const (), 1), // +0x88ccb0 sub_88ccb0
    Lifted::new(0x7000f78cc0, tpl_44 as *const (), 1), // +0x88ccc0 sub_88ccc0
    Lifted::new(0x7000f78cd0, tpl_144 as *const (), 1), // +0x88ccd0 sub_88ccd0
    Lifted::new(0x7000f78ce0, tpl_44 as *const (), 1), // +0x88cce0 sub_88cce0
    Lifted::new(0x7000f78cf0, tpl_144 as *const (), 1), // +0x88ccf0 sub_88ccf0
    Lifted::new(0x7000f78d00, tpl_44 as *const (), 1), // +0x88cd00 sub_88cd00
    Lifted::new(0x7000f78d10, tpl_144 as *const (), 1), // +0x88cd10 sub_88cd10
    Lifted::new(0x7000f78d20, tpl_144 as *const (), 1), // +0x88cd20 sub_88cd20
    Lifted::new(0x7000f78d30, fn_7000f78d30 as *const (), 0), // +0x88cd30 sub_88cd30
    Lifted::new(0x7000f78e10, fn_7000f78e10 as *const (), 0), // +0x88ce10 sub_88ce10
    Lifted::new(0x7000f78e80, fn_7000f78e80 as *const (), 0), // +0x88ce80 sub_88ce80
    Lifted::new(0x7000f79240, fn_7000f79240 as *const (), 0), // +0x88d240 sub_88d240
    Lifted::new(0x7000f796c0, fn_7000f796c0 as *const (), 0), // +0x88d6c0 sub_88d6c0
    Lifted::new(0x7000f79870, fn_7000f79870 as *const (), 0), // +0x88d870 sub_88d870
    Lifted::new(0x7000f79a20, fn_7000f79a20 as *const (), 0), // +0x88da20 sub_88da20
    Lifted::new(0x7000f79b60, fn_7000f79b60 as *const (), 0), // +0x88db60 sub_88db60
    Lifted::new(0x7000f79ca0, fn_7000f79ca0 as *const (), 0), // +0x88dca0 sub_88dca0
    Lifted::new(0x7000f79d80, fn_7000f79d80 as *const (), 0), // +0x88dd80 sub_88dd80
    Lifted::new(0x7000f79ec0, fn_7000f79ec0 as *const (), 0), // +0x88dec0 sub_88dec0
    Lifted::new(0x7000f79fb0, fn_7000f79fb0 as *const (), 0), // +0x88dfb0 sub_88dfb0
    Lifted::new(0x7000f7a2d0, fn_7000f7a2d0 as *const (), 0), // +0x88e2d0 sub_88e2d0
    Lifted::new(0x7000f7a4a0, fn_7000f7a4a0 as *const (), 0), // +0x88e4a0 sub_88e4a0
    Lifted::new(0x7000f7a5e0, tpl_1694::<0x158, 0x15c, 0x7000f410f0> as *const (), 0), // +0x88e5e0 sub_88e5e0
    Lifted::new(0x7000f7a640, tpl_1694::<0x158, 0x15c, 0x7000f41160> as *const (), 0), // +0x88e640 sub_88e640
    Lifted::new(0x7000f7a6a0, tpl_1695::<0x140, 0x144, 0x7000f410f0> as *const (), 0), // +0x88e6a0 sub_88e6a0
    Lifted::new(0x7000f7a700, tpl_1695::<0x140, 0x144, 0x7000f41160> as *const (), 0), // +0x88e700 sub_88e700
    Lifted::new(0x7000f7a7c0, fn_7000f7a7c0 as *const (), 3), // +0x88e7c0 sub_88e7c0
    Lifted::new(0x7000f7a9c0, fn_7000f7a9c0 as *const (), 0), // +0x88e9c0 sub_88e9c0
    Lifted::new(0x7000f7aa60, fn_7000f7aa60 as *const (), 0), // +0x88ea60 sub_88ea60
    Lifted::new(0x7000f7ab00, tpl_1696::<0x10, 0x34, 0x20, 0x30, 0x7000749090, 0x7000a40a20> as *const (), 0), // +0x88eb00 sub_88eb00
    Lifted::new(0x7000f7ab60, tpl_1696::<0x10, 0x1c, 0x8, 0x18, 0x7000749090, 0x7000a40a20> as *const (), 0), // +0x88eb60 sub_88eb60
    Lifted::new(0x7000f7abc0, fn_7000f7abc0 as *const (), 2), // +0x88ebc0 sub_88ebc0
    Lifted::new(0x7000f7ac00, fn_7000f7ac00 as *const (), 1), // +0x88ec00 sub_88ec00
    Lifted::new(0x7000f7acb0, fn_7000f7acb0 as *const (), 1), // +0x88ecb0 sub_88ecb0
    Lifted::new(0x7000f7acc0, fn_7000f7acc0 as *const (), 1), // +0x88ecc0 sub_88ecc0
    Lifted::new(0x7000f7acd0, fn_7000f7acd0 as *const (), 0), // +0x88ecd0 sub_88ecd0
    Lifted::new(0x7000f7beb0, tpl_1697::<0xffffffffffffffd0> as *const (), 1), // +0x88feb0 sub_88feb0
    Lifted::new(0x7000f7bec0, fn_7000f7bec0 as *const (), 1), // +0x88fec0 sub_88fec0
    Lifted::new(0x7000f7bed0, fn_7000f7bed0 as *const (), 1), // +0x88fed0 sub_88fed0
    Lifted::new(0x7000f7bef0, fn_7000f7bef0 as *const (), 0), // +0x88fef0 sub_88fef0
    Lifted::new(0x7000f7bf20, fn_7000f7bf20 as *const (), 1), // +0x88ff20 sub_88ff20
    Lifted::new(0x7000f7bf30, fn_7000f7bf30 as *const (), 1), // +0x88ff30 sub_88ff30
    Lifted::new(0x7000f7bf40, fn_7000f7bf40 as *const (), 0), // +0x88ff40 sub_88ff40
    Lifted::new(0x7000f7d140, fn_7000f7d140 as *const (), 0), // +0x891140 sub_891140
    Lifted::new(0x7000f7d1a0, fn_7000f7d1a0 as *const (), 0), // +0x8911a0 sub_8911a0
    Lifted::new(0x7000f7e3e0, fn_7000f7e3e0 as *const (), 0), // +0x8923e0 sub_8923e0
    Lifted::new(0x7000f7f5a0, fn_7000f7f5a0 as *const (), 0), // +0x8935a0 sub_8935a0
    Lifted::new(0x7000f80760, tpl_1698::<0x700077b2d0, 0x7000777f90> as *const (), 0), // +0x894760 sub_894760
    Lifted::new(0x7000f80780, fn_7000f80780 as *const (), 0), // +0x894780 sub_894780
    Lifted::new(0x7000f808d0, fn_7000f808d0 as *const (), 0), // +0x8948d0 sub_8948d0
    Lifted::new(0x7000f80b20, fn_7000f80b20 as *const (), 0), // +0x894b20 sub_894b20
    Lifted::new(0x7000f80cb0, fn_7000f80cb0 as *const (), 0), // +0x894cb0 sub_894cb0
    Lifted::new(0x7000f80f10, tpl_1698::<0x70006fc300, 0x7000777f90> as *const (), 0), // +0x894f10 sub_894f10
    Lifted::new(0x7000f80f30, fn_7000f80f30 as *const (), 0), // +0x894f30 sub_894f30
    Lifted::new(0x7000f81060, fn_7000f81060 as *const (), 0), // +0x895060 sub_895060
    Lifted::new(0x7000f81360, fn_7000f81360 as *const (), 0), // +0x895360 sub_895360
    Lifted::new(0x7000f81650, fn_7000f81650 as *const (), 0), // +0x895650 sub_895650
    Lifted::new(0x7000f81ad0, fn_7000f81ad0 as *const (), 0), // +0x895ad0 sub_895ad0
    Lifted::new(0x7000f81bc0, fn_7000f81bc0 as *const (), 0), // +0x895bc0 sub_895bc0
    Lifted::new(0x7000f81d30, fn_7000f81d30 as *const (), 0), // +0x895d30 sub_895d30
    Lifted::new(0x7000f825f0, tpl_1698::<0x700075a9c0, 0x7000777f90> as *const (), 0), // +0x8965f0 sub_8965f0
    Lifted::new(0x7000f82610, fn_7000f82610 as *const (), 0), // +0x896610 sub_896610
    Lifted::new(0x7000f82a90, fn_7000f82a90 as *const (), 2), // +0x896a90 sub_896a90
    Lifted::new(0x7000f82e00, fn_7000f82e00 as *const (), 0), // +0x896e00 sub_896e00
    Lifted::new(0x7000f831e0, fn_7000f831e0 as *const (), 0), // +0x8971e0 sub_8971e0
    Lifted::new(0x7000f83300, fn_7000f83300 as *const (), 0), // +0x897300 sub_897300
    Lifted::new(0x7000f844c0, fn_7000f844c0 as *const (), 0), // +0x8984c0 sub_8984c0
    Lifted::new(0x7000f847a0, fn_7000f847a0 as *const (), 0), // +0x8987a0 sub_8987a0
    Lifted::new(0x7000f84d20, fn_7000f84d20 as *const (), 0), // +0x898d20 sub_898d20
    Lifted::new(0x7000f85fb0, fn_7000f85fb0 as *const (), 0), // +0x899fb0 sub_899fb0
    Lifted::new(0x7000f87270, tpl_51::<0x8> as *const (), 1), // +0x89b270 sub_89b270
    Lifted::new(0x7000f87280, fn_7000f87280 as *const (), 0), // +0x89b280 sub_89b280
    Lifted::new(0x7000f875a0, fn_7000f875a0 as *const (), 0), // +0x89b5a0 sub_89b5a0
    Lifted::new(0x7000f87600, fn_7000f87600 as *const (), 0), // +0x89b600 sub_89b600
    Lifted::new(0x7000f876e0, fn_7000f876e0 as *const (), 0), // +0x89b6e0 sub_89b6e0
    Lifted::new(0x7000f87730, fn_7000f87730 as *const (), 0), // +0x89b730 sub_89b730
    Lifted::new(0x7000f878b0, fn_7000f878b0 as *const (), 2), // +0x89b8b0 sub_89b8b0
    Lifted::new(0x7000f87e20, tpl_580::<0x10, 0x8, 0x7000d127e0> as *const (), 0), // +0x89be20 sub_89be20
    Lifted::new(0x7000f87f00, tpl_581::<0x10, 0x8, 0x7000d127e0> as *const (), 0), // +0x89bf00 sub_89bf00
    Lifted::new(0x7000f87fe0, tpl_54::<0x8, 0x7000f88170, 0x19, 0x10> as *const (), 0), // +0x89bfe0 sub_89bfe0
    Lifted::new(0x7000f880b0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x89c0b0 sub_89c0b0
    Lifted::new(0x7000f880f0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x89c0f0 sub_89c0f0
    Lifted::new(0x7000f88130, tpl_55::<0x700177c0d0> as *const (), 0), // +0x89c130 sub_89c130
    Lifted::new(0x7000f88170, fn_7000f88170 as *const (), 2), // +0x89c170 sub_89c170
    Lifted::new(0x7000f88270, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000f883fd, 0x7000f8ae80, 0x7000f88403, 0x70007f0910, 0x7000f88409> as *const (), 0), // +0x89c270 sub_89c270
    Lifted::new(0x7000f88470, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000f885fd, 0x7000f8aea0, 0x7000f88603, 0x70007f0910, 0x7000f88609> as *const (), 0), // +0x89c470 sub_89c470
    Lifted::new(0x7000f88670, fn_7000f88670 as *const (), 0), // +0x89c670 sub_89c670
    Lifted::new(0x7000f88700, tpl_963::<0x19, 0x10, 0x7000f88700, 0x700177c312> as *const (), 0), // +0x89c700 sub_89c700
    Lifted::new(0x7000f88770, tpl_963::<0x19, 0x10, 0x7000f88770, 0x700177c312> as *const (), 0), // +0x89c770 sub_89c770
    Lifted::new(0x7000f887e0, tpl_395::<0x7000f8aee0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000f88925, 0x7000f8aec0, 0x7000f8892b> as *const (), 0), // +0x89c7e0 sub_89c7e0
    Lifted::new(0x7000f88980, fn_7000f88980 as *const (), 2), // +0x89c980 sub_89c980
    Lifted::new(0x7000f88ac0, tpl_85::<0x8, 0x38, 0x7001779e20, 0x10, 0x18> as *const (), 0), // +0x89cac0 sub_89cac0
    Lifted::new(0x7000f88b40, tpl_1699::<0x8, 0x38, 0x7001779e20, 0x10, 0x18, 0x7000f87fe0> as *const (), 0), // +0x89cb40 sub_89cb40
    Lifted::new(0x7000f88bb0, tpl_86::<0x8, 0x38, 0x7001779e20, 0x10, 0x18> as *const (), 0), // +0x89cbb0 sub_89cbb0
    Lifted::new(0x7000f88c80, fn_7000f88c80 as *const (), 0), // +0x89cc80 sub_89cc80
    Lifted::new(0x7000f88eb0, fn_7000f88eb0 as *const (), 0), // +0x89ceb0 sub_89ceb0
    Lifted::new(0x7000f88f20, tpl_1700::<0x7000f88670> as *const (), 0), // +0x89cf20 sub_89cf20
    Lifted::new(0x7000f88f30, fn_7000f88f30 as *const (), 0), // +0x89cf30 sub_89cf30
    Lifted::new(0x7000f89010, fn_7000f89010 as *const (), 0), // +0x89d010 sub_89d010
    Lifted::new(0x7000f89200, fn_7000f89200 as *const (), 0), // +0x89d200 sub_89d200
    Lifted::new(0x7000f89260, fn_7000f89260 as *const (), 0), // +0x89d260 sub_89d260
    Lifted::new(0x7000f892c0, fn_7000f892c0 as *const (), 0), // +0x89d2c0 sub_89d2c0
    Lifted::new(0x7000f895f0, fn_7000f895f0 as *const (), 0), // +0x89d5f0 sub_89d5f0
    Lifted::new(0x7000f89740, fn_7000f89740 as *const (), 2), // +0x89d740 sub_89d740
    Lifted::new(0x7000f897d0, fn_7000f897d0 as *const (), 2), // +0x89d7d0 sub_89d7d0
    Lifted::new(0x7000f89c30, fn_7000f89c30 as *const (), 0), // +0x89dc30 sub_89dc30
    Lifted::new(0x7000f89d40, fn_7000f89d40 as *const (), 1), // +0x89dd40 sub_89dd40
    Lifted::new(0x7000f89d90, fn_7000f89d90 as *const (), 0), // +0x89dd90 sub_89dd90
    Lifted::new(0x7000f89dc0, fn_7000f89dc0 as *const (), 0), // +0x89ddc0 sub_89ddc0
    Lifted::new(0x7000f89e70, fn_7000f89e70 as *const (), 0), // +0x89de70 sub_89de70
    Lifted::new(0x7000f8a050, fn_7000f8a050 as *const (), 0), // +0x89e050 sub_89e050
    Lifted::new(0x7000f8a140, fn_7000f8a140 as *const (), 0), // +0x89e140 sub_89e140
    Lifted::new(0x7000f8a230, fn_7000f8a230 as *const (), 0), // +0x89e230 sub_89e230
    Lifted::new(0x7000f8a290, fn_7000f8a290 as *const (), 2), // +0x89e290 sub_89e290
    Lifted::new(0x7000f8a520, fn_7000f8a520 as *const (), 0), // +0x89e520 sub_89e520
    Lifted::new(0x7000f8a600, fn_7000f8a600 as *const (), 0), // +0x89e600 sub_89e600
    Lifted::new(0x7000f8aa80, tpl_287 as *const (), 1), // +0x89ea80 sub_89ea80
    Lifted::new(0x7000f8ada0, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x89eda0 sub_89eda0
    Lifted::new(0x7000f8ae10, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x89ee10 sub_89ee10
    Lifted::new(0x7000f8ae80, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f8ae90> as *const (), 0), // +0x89ee80 sub_89ee80
    Lifted::new(0x7000f8aea0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f8aeb0> as *const (), 0), // +0x89eea0 sub_89eea0
    Lifted::new(0x7000f8aec0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f8aed0> as *const (), 0), // +0x89eec0 sub_89eec0
    Lifted::new(0x7000f8aee0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000f8af34, 0x70007f0910, 0x7000f8af4f> as *const (), 0), // +0x89eee0 sub_89eee0
    Lifted::new(0x7000f8af70, tpl_1701::<0x8, 0x19, 0x10, 0x7000f88700, 0x700177c312> as *const (), 0), // +0x89ef70 sub_89ef70
    Lifted::new(0x7000f8b000, fn_7000f8b000 as *const (), 0), // +0x89f000 sub_89f000
    Lifted::new(0x7000f8b0f0, fn_7000f8b0f0 as *const (), 1), // +0x89f0f0 sub_89f0f0
    Lifted::new(0x7000f8b210, fn_7000f8b210 as *const (), 2), // +0x89f210 sub_89f210
    Lifted::new(0x7000f8b520, fn_7000f8b520 as *const (), 1), // +0x89f520 sub_89f520
    Lifted::new(0x7000f8b650, tpl_68 as *const (), 1), // +0x89f650 sub_89f650
    Lifted::new(0x7000f8b700, tpl_1145::<0x7001779e20, 0x700077d380> as *const (), 0), // +0x89f700 sub_89f700
    Lifted::new(0x7000f8b830, fn_7000f8b830 as *const (), 0), // +0x89f830 sub_89f830
    Lifted::new(0x7000f8b910, fn_7000f8b910 as *const (), 0), // +0x89f910 sub_89f910
    Lifted::new(0x7000f8ba80, fn_7000f8ba80 as *const (), 0), // +0x89fa80 sub_89fa80
    Lifted::new(0x7000f8bb00, fn_7000f8bb00 as *const (), 0), // +0x89fb00 sub_89fb00
    Lifted::new(0x7000f8bce0, fn_7000f8bce0 as *const (), 0), // +0x89fce0 sub_89fce0
    Lifted::new(0x7000f8bd90, fn_7000f8bd90 as *const (), 0), // +0x89fd90 sub_89fd90
    Lifted::new(0x7000f8bf10, fn_7000f8bf10 as *const (), 0), // +0x89ff10 sub_89ff10
    Lifted::new(0x7000f8c200, fn_7000f8c200 as *const (), 0), // +0x8a0200 sub_8a0200
    Lifted::new(0x7000f8c310, fn_7000f8c310 as *const (), 0), // +0x8a0310 sub_8a0310
    Lifted::new(0x7000f8c360, fn_7000f8c360 as *const (), 0), // +0x8a0360 sub_8a0360
    Lifted::new(0x7000f8c3c0, fn_7000f8c3c0 as *const (), 0), // +0x8a03c0 sub_8a03c0
    Lifted::new(0x7000f8c3e0, fn_7000f8c3e0 as *const (), 0), // +0x8a03e0 sub_8a03e0
    Lifted::new(0x7000f8c4c0, fn_7000f8c4c0 as *const (), 0), // +0x8a04c0 sub_8a04c0
    Lifted::new(0x7000f8c590, tpl_1702::<0x7000f8c4c0> as *const (), 0), // +0x8a0590 sub_8a0590
    Lifted::new(0x7000f8c5a0, fn_7000f8c5a0 as *const (), 0), // +0x8a05a0 sub_8a05a0
    Lifted::new(0x7000f8c710, tpl_1702::<0x7000f8c5a0> as *const (), 0), // +0x8a0710 sub_8a0710
    Lifted::new(0x7000f8c720, tpl_33::<0x700177c312> as *const (), 0), // +0x8a0720 sub_8a0720
    Lifted::new(0x7000f8c750, tpl_133::<0x7000794090, 0x700177c312> as *const (), 0), // +0x8a0750 sub_8a0750
    Lifted::new(0x7000f8c790, fn_7000f8c790 as *const (), 0), // +0x8a0790 sub_8a0790
    Lifted::new(0x7000f8c7e0, tpl_133::<0x7000749df0, 0x700177c312> as *const (), 0), // +0x8a07e0 sub_8a07e0
    Lifted::new(0x7000f8c820, fn_7000f8c820 as *const (), 0), // +0x8a0820 sub_8a0820
    Lifted::new(0x7000f8c8b0, fn_7000f8c8b0 as *const (), 2), // +0x8a08b0 sub_8a08b0
    Lifted::new(0x7000f8ca30, fn_7000f8ca30 as *const (), 2), // +0x8a0a30 sub_8a0a30
    Lifted::new(0x7000f8cb90, fn_7000f8cb90 as *const (), 0), // +0x8a0b90 sub_8a0b90
    Lifted::new(0x7000f8cc10, fn_7000f8cc10 as *const (), 0), // +0x8a0c10 sub_8a0c10
    Lifted::new(0x7000f8cc50, tpl_411::<0x7000772c80, 0x18, 0x7000727810> as *const (), 0), // +0x8a0c50 sub_8a0c50
    Lifted::new(0x7000f8cc80, fn_7000f8cc80 as *const (), 0), // +0x8a0c80 sub_8a0c80
    Lifted::new(0x7000f8cca0, fn_7000f8cca0 as *const (), 0), // +0x8a0ca0 sub_8a0ca0
    Lifted::new(0x7000f8cd70, fn_7000f8cd70 as *const (), 2), // +0x8a0d70 sub_8a0d70
    Lifted::new(0x7000f8d0e0, fn_7000f8d0e0 as *const (), 0), // +0x8a10e0 sub_8a10e0
    Lifted::new(0x7000f8d100, fn_7000f8d100 as *const (), 0), // +0x8a1100 sub_8a1100
    Lifted::new(0x7000f8d1b0, tpl_1326::<0x48, 0x700075e9f0> as *const (), 0), // +0x8a11b0 sub_8a11b0
    Lifted::new(0x7000f8d1e0, fn_7000f8d1e0 as *const (), 0), // +0x8a11e0 sub_8a11e0
    Lifted::new(0x7000f8d2e0, fn_7000f8d2e0 as *const (), 0), // +0x8a12e0 sub_8a12e0
    Lifted::new(0x7000f8d310, fn_7000f8d310 as *const (), 0), // +0x8a1310 sub_8a1310
    Lifted::new(0x7000f8d360, fn_7000f8d360 as *const (), 0), // +0x8a1360 sub_8a1360
    Lifted::new(0x7000f8d410, fn_7000f8d410 as *const (), 0), // +0x8a1410 sub_8a1410
    Lifted::new(0x7000f8d5b0, fn_7000f8d5b0 as *const (), 0), // +0x8a15b0 sub_8a15b0
    Lifted::new(0x7000f8d6b0, fn_7000f8d6b0 as *const (), 2), // +0x8a16b0 sub_8a16b0
    Lifted::new(0x7000f8d800, fn_7000f8d800 as *const (), 1), // +0x8a1800 sub_8a1800
    Lifted::new(0x7000f8d850, fn_7000f8d850 as *const (), 0), // +0x8a1850 sub_8a1850
    Lifted::new(0x7000f8d890, fn_7000f8d890 as *const (), 2), // +0x8a1890 sub_8a1890
    Lifted::new(0x7000f8d9a0, fn_7000f8d9a0 as *const (), 1), // +0x8a19a0 sub_8a19a0
    Lifted::new(0x7000f8d9c0, fn_7000f8d9c0 as *const (), 0), // +0x8a19c0 sub_8a19c0
    Lifted::new(0x7000f8db00, fn_7000f8db00 as *const (), 2), // +0x8a1b00 sub_8a1b00
    Lifted::new(0x7000f8dc00, fn_7000f8dc00 as *const (), 2), // +0x8a1c00 sub_8a1c00
    Lifted::new(0x7000f8def0, fn_7000f8def0 as *const (), 0), // +0x8a1ef0 sub_8a1ef0
    Lifted::new(0x7000f8df80, fn_7000f8df80 as *const (), 0), // +0x8a1f80 sub_8a1f80
    Lifted::new(0x7000f8e140, fn_7000f8e140 as *const (), 0), // +0x8a2140 sub_8a2140
    Lifted::new(0x7000f8e2c0, fn_7000f8e2c0 as *const (), 0), // +0x8a22c0 sub_8a22c0
    Lifted::new(0x7000f8e390, fn_7000f8e390 as *const (), 0), // +0x8a2390 sub_8a2390
    Lifted::new(0x7000f8e510, fn_7000f8e510 as *const (), 2), // +0x8a2510 sub_8a2510
    Lifted::new(0x7000f8e7d0, fn_7000f8e7d0 as *const (), 0), // +0x8a27d0 sub_8a27d0
    Lifted::new(0x7000f8e8d0, fn_7000f8e8d0 as *const (), 1), // +0x8a28d0 sub_8a28d0
    Lifted::new(0x7000f8e960, fn_7000f8e960 as *const (), 0), // +0x8a2960 sub_8a2960
    Lifted::new(0x7000f8ea10, fn_7000f8ea10 as *const (), 0), // +0x8a2a10 sub_8a2a10
    Lifted::new(0x7000f8ead0, tpl_1703::<0x48, 0x7000754b50, 0x70007394b0> as *const (), 0), // +0x8a2ad0 sub_8a2ad0
    Lifted::new(0x7000f8eb00, fn_7000f8eb00 as *const (), 0), // +0x8a2b00 sub_8a2b00
    Lifted::new(0x7000f8eb60, fn_7000f8eb60 as *const (), 0), // +0x8a2b60 sub_8a2b60
    Lifted::new(0x7000f8ec50, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f8ed57, 0x7000f8b650, 0x70007f0910, 0x7000f8ed73> as *const (), 0), // +0x8a2c50 sub_8a2c50
    Lifted::new(0x7000f8edc0, tpl_286::<0x7000f8ec50, 0x70017d37a0, 0x70017790e0, 0x7000f8efab> as *const (), 0), // +0x8a2dc0 sub_8a2dc0
    Lifted::new(0x7000f8f030, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000f8f040> as *const (), 0), // +0x8a3030 sub_8a3030
    Lifted::new(0x7000f8f050, tpl_30 as *const (), 1), // +0x8a3050 sub_8a3050
    Lifted::new(0x7000f8f060, tpl_51::<0x18> as *const (), 1), // +0x8a3060 sub_8a3060
    Lifted::new(0x7000f8f070, tpl_49::<0x10> as *const (), 1), // +0x8a3070 sub_8a3070
    Lifted::new(0x7000f8f080, fn_7000f8f080 as *const (), 0), // +0x8a3080 sub_8a3080
    Lifted::new(0x7000f8f330, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f8f508, 0x7000f914e0, 0x7000f8f50e, 0x70007f0910, 0x7000f8f514> as *const (), 2), // +0x8a3330 sub_8a3330
    Lifted::new(0x7000f8f590, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f8f764, 0x7000f91500, 0x7000f8f76a, 0x70007f0910, 0x7000f8f770> as *const (), 0), // +0x8a3590 sub_8a3590
    Lifted::new(0x7000f8f7f0, fn_7000f8f7f0 as *const (), 0), // +0x8a37f0 sub_8a37f0
    Lifted::new(0x7000f8f8d0, tpl_1704::<0x8, 0x7000f8fcd0, 0x700177c312, 0x10, 0xfffffffffffffff8, 0x7001d54968, 0x7000f8f980> as *const (), 0), // +0x8a38d0 sub_8a38d0
    Lifted::new(0x7000f8f9b0, fn_7000f8f9b0 as *const (), 0), // +0x8a39b0 sub_8a39b0
    Lifted::new(0x7000f8fba0, fn_7000f8fba0 as *const (), 0), // +0x8a3ba0 sub_8a3ba0
    Lifted::new(0x7000f8fcd0, fn_7000f8fcd0 as *const (), 0), // +0x8a3cd0 sub_8a3cd0
    Lifted::new(0x7000f900b0, fn_7000f900b0 as *const (), 0), // +0x8a40b0 sub_8a40b0
    Lifted::new(0x7000f90200, fn_7000f90200 as *const (), 0), // +0x8a4200 sub_8a4200
    Lifted::new(0x7000f90250, tpl_575::<0x30, 0x7000f897d0> as *const (), 0), // +0x8a4250 sub_8a4250
    Lifted::new(0x7000f90270, fn_7000f90270 as *const (), 0), // +0x8a4270 sub_8a4270
    Lifted::new(0x7000f902e0, fn_7000f902e0 as *const (), 0), // +0x8a42e0 sub_8a42e0
    Lifted::new(0x7000f90360, fn_7000f90360 as *const (), 0), // +0x8a4360 sub_8a4360
    Lifted::new(0x7000f903f0, fn_7000f903f0 as *const (), 0), // +0x8a43f0 sub_8a43f0
    Lifted::new(0x7000f90560, tpl_1705::<0x48, 0x7000fb7810> as *const (), 0), // +0x8a4560 sub_8a4560
    Lifted::new(0x7000f90580, fn_7000f90580 as *const (), 0), // +0x8a4580 sub_8a4580
    Lifted::new(0x7000f905c0, tpl_451::<0x18, 0x10, 0x7000f91540, 0x7000f905e9> as *const (), 0), // +0x8a45c0 sub_8a45c0
    Lifted::new(0x7000f90600, fn_7000f90600 as *const (), 0), // +0x8a4600 sub_8a4600
    Lifted::new(0x7000f90650, fn_7000f90650 as *const (), 0), // +0x8a4650 sub_8a4650
    Lifted::new(0x7000f90670, tpl_452::<0x18, 0x10> as *const (), 1), // +0x8a4670 sub_8a4670
    Lifted::new(0x7000f90680, tpl_47::<0x39> as *const (), 1), // +0x8a4680 sub_8a4680
    Lifted::new(0x7000f90690, fn_7000f90690 as *const (), 0), // +0x8a4690 sub_8a4690
    Lifted::new(0x7000f90720, fn_7000f90720 as *const (), 0), // +0x8a4720 sub_8a4720
    Lifted::new(0x7000f907d0, tpl_143::<0x8, 0x7000fa1b00> as *const (), 0), // +0x8a47d0 sub_8a47d0
    Lifted::new(0x7000f907e0, fn_7000f907e0 as *const (), 2), // +0x8a47e0 sub_8a47e0
    Lifted::new(0x7000f90ce0, tpl_992::<0x58, 0x50, 0x8, 0xfffffffffffffff0, 0xfffffffffffffff8> as *const (), 1), // +0x8a4ce0 sub_8a4ce0
    Lifted::new(0x7000f90d60, fn_7000f90d60 as *const (), 0), // +0x8a4d60 sub_8a4d60
    Lifted::new(0x7000f90e70, tpl_255::<0x68, 0x70> as *const (), 1), // +0x8a4e70 sub_8a4e70
    Lifted::new(0x7000f90e80, fn_7000f90e80 as *const (), 0), // +0x8a4e80 sub_8a4e80
    Lifted::new(0x7000f90f50, fn_7000f90f50 as *const (), 0), // +0x8a4f50 sub_8a4f50
    Lifted::new(0x7000f91010, fn_7000f91010 as *const (), 0), // +0x8a5010 sub_8a5010
    Lifted::new(0x7000f910f0, fn_7000f910f0 as *const (), 0), // +0x8a50f0 sub_8a50f0
    Lifted::new(0x7000f911b0, fn_7000f911b0 as *const (), 2), // +0x8a51b0 sub_8a51b0
    Lifted::new(0x7000f914d0, tpl_143::<0x8, 0x7000fa64e0> as *const (), 0), // +0x8a54d0 sub_8a54d0
    Lifted::new(0x7000f914e0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f914f0> as *const (), 0), // +0x8a54e0 sub_8a54e0
    Lifted::new(0x7000f91500, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f91510> as *const (), 0), // +0x8a5500 sub_8a5500
    Lifted::new(0x7000f91520, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000f91530> as *const (), 0), // +0x8a5520 sub_8a5520
    Lifted::new(0x7000f91540, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000f91550> as *const (), 0), // +0x8a5540 sub_8a5540
    Lifted::new(0x7000f91570, fn_7000f91570 as *const (), 1), // +0x8a5570 sub_8a5570
    Lifted::new(0x7000f91590, tpl_1706::<0x7, 0xb, 0x700078e020, 0x70007429b0> as *const (), 0), // +0x8a5590 sub_8a5590
    Lifted::new(0x7000f91690, tpl_1706::<0x3, 0x7, 0x700078e020, 0x70007429b0> as *const (), 0), // +0x8a5690 sub_8a5690
    Lifted::new(0x7000f91790, tpl_560::<0x7000732f50> as *const (), 0), // +0x8a5790 sub_8a5790
    Lifted::new(0x7000f919b0, tpl_580::<0x10, 0x8, 0x7000d127e0> as *const (), 0), // +0x8a59b0 sub_8a59b0
    Lifted::new(0x7000f91a90, tpl_564::<0x7000f91c70, 0x7000f9d9c0> as *const (), 0), // +0x8a5a90 sub_8a5a90
    Lifted::new(0x7000f91b10, tpl_581::<0x10, 0x8, 0x7000d127e0> as *const (), 0), // +0x8a5b10 sub_8a5b10
    Lifted::new(0x7000f91bf0, tpl_565::<0x7000f91c70, 0x7000f9d9c0> as *const (), 0), // +0x8a5bf0 sub_8a5bf0
    Lifted::new(0x7000f91c70, fn_7000f91c70 as *const (), 0), // +0x8a5c70 sub_8a5c70
    Lifted::new(0x7000f91d30, fn_7000f91d30 as *const (), 0), // +0x8a5d30 sub_8a5d30
    Lifted::new(0x7000f91f00, fn_7000f91f00 as *const (), 0), // +0x8a5f00 sub_8a5f00
    Lifted::new(0x7000f920d0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8a60d0 sub_8a60d0
    Lifted::new(0x7000f92110, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8a6110 sub_8a6110
    Lifted::new(0x7000f92150, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8a6150 sub_8a6150
    Lifted::new(0x7000f92190, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8a6190 sub_8a6190
    Lifted::new(0x7000f921d0, fn_7000f921d0 as *const (), 2), // +0x8a61d0 sub_8a61d0
    Lifted::new(0x7000f924e0, tpl_1319::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f926aa, 0x7000fa9170, 0x7000f926b0, 0x70007f0910, 0x7000f926b6> as *const (), 2), // +0x8a64e0 sub_8a64e0
    Lifted::new(0x7000f92730, fn_7000f92730 as *const (), 2), // +0x8a6730 sub_8a6730
    Lifted::new(0x7000f929b0, fn_7000f929b0 as *const (), 2), // +0x8a69b0 sub_8a69b0
    Lifted::new(0x7000f92bf0, tpl_1707::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f92e69, 0x7000fa91d0, 0x7000f92e6f, 0x70007f0910, 0x7000f92e75> as *const (), 2), // +0x8a6bf0 sub_8a6bf0
    Lifted::new(0x7000f92f20, tpl_1323::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000f93127, 0x7000fa9190, 0x7000f9312d, 0x70007f0910, 0x7000f93133> as *const (), 2), // +0x8a6f20 sub_8a6f20
    Lifted::new(0x7000f931c0, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f93394, 0x7000fa9230, 0x7000f9339a, 0x70007f0910, 0x7000f933a0> as *const (), 0), // +0x8a71c0 sub_8a71c0
    Lifted::new(0x7000f93420, tpl_1080::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000f93621, 0x7000fa9250, 0x7000f93627, 0x70007f0910, 0x7000f9362d> as *const (), 0), // +0x8a7420 sub_8a7420
    Lifted::new(0x7000f937d0, tpl_391::<0x7000fa9290, 0x700177c0e0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000f938fc, 0x7000fa9150, 0x7000f93924> as *const (), 0), // +0x8a77d0 sub_8a77d0
    Lifted::new(0x7000f93980, tpl_898::<0x700177c0e0> as *const (), 0), // +0x8a7980 sub_8a7980
    Lifted::new(0x7000f939c0, tpl_898::<0x700177c0e0> as *const (), 0), // +0x8a79c0 sub_8a79c0
    Lifted::new(0x7000f93a00, tpl_392::<0x700177c0e0> as *const (), 0), // +0x8a7a00 sub_8a7a00
    Lifted::new(0x7000f93a30, tpl_392::<0x700177c0e0> as *const (), 0), // +0x8a7a30 sub_8a7a30
    Lifted::new(0x7000f93a60, tpl_1708::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000fa7e30, 0x70017d37c0, 0x70017790e0, 0x7000f93d5a> as *const (), 2), // +0x8a7a60 sub_8a7a60
    Lifted::new(0x7000f93e20, tpl_1144::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000fa8a20, 0x70017d37c0, 0x70017790e0, 0x7000f940ec> as *const (), 2), // +0x8a7e20 sub_8a7e20
    Lifted::new(0x7000f941a0, tpl_1708::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000fa87b0, 0x70017d37c0, 0x70017790e0, 0x7000f9449a> as *const (), 2), // +0x8a81a0 sub_8a81a0
    Lifted::new(0x7000f94560, tpl_1709::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000fa79d0, 0x70017d37c0, 0x70017790e0, 0x7000f94821> as *const (), 2), // +0x8a8560 sub_8a8560
    Lifted::new(0x7000f948e0, tpl_1709::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000fa7c00, 0x70017d37c0, 0x70017790e0, 0x7000f94ba1> as *const (), 2), // +0x8a88e0 sub_8a88e0
    Lifted::new(0x7000f94c60, fn_7000f94c60 as *const (), 2), // +0x8a8c60 sub_8a8c60
    Lifted::new(0x7000f95000, tpl_1710::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000fa8310, 0x70017d37c0, 0x70017790e0, 0x7000f952dd> as *const (), 2), // +0x8a9000 sub_8a9000
    Lifted::new(0x7000f953a0, tpl_1710::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000fa8560, 0x70017d37c0, 0x70017790e0, 0x7000f9567d> as *const (), 2), // +0x8a93a0 sub_8a93a0
    Lifted::new(0x7000f95740, tpl_68 as *const (), 1), // +0x8a9740 sub_8a9740
    Lifted::new(0x7000f957f0, tpl_68 as *const (), 1), // +0x8a97f0 sub_8a97f0
    Lifted::new(0x7000f958a0, tpl_68 as *const (), 1), // +0x8a98a0 sub_8a98a0
    Lifted::new(0x7000f95950, tpl_68 as *const (), 1), // +0x8a9950 sub_8a9950
    Lifted::new(0x7000f95a00, tpl_68 as *const (), 1), // +0x8a9a00 sub_8a9a00
    Lifted::new(0x7000f95ab0, tpl_68 as *const (), 1), // +0x8a9ab0 sub_8a9ab0
    Lifted::new(0x7000f95b60, tpl_68 as *const (), 1), // +0x8a9b60 sub_8a9b60
    Lifted::new(0x7000f95c10, tpl_68 as *const (), 1), // +0x8a9c10 sub_8a9c10
    Lifted::new(0x7000f95cc0, tpl_896::<0x1, 0x30, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x18, 0x10> as *const (), 1), // +0x8a9cc0 sub_8a9cc0
    Lifted::new(0x7000f95dd0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8a9dd0 sub_8a9dd0
    Lifted::new(0x7000f95e10, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8a9e10 sub_8a9e10
    Lifted::new(0x7000f960b0, fn_7000f960b0 as *const (), 0), // +0x8aa0b0 sub_8aa0b0
    Lifted::new(0x7000f96270, fn_7000f96270 as *const (), 0), // +0x8aa270 sub_8aa270
    Lifted::new(0x7000f96440, fn_7000f96440 as *const (), 0), // +0x8aa440 sub_8aa440
    Lifted::new(0x7000f96670, fn_7000f96670 as *const (), 0), // +0x8aa670 sub_8aa670
    Lifted::new(0x7000f96730, fn_7000f96730 as *const (), 0), // +0x8aa730 sub_8aa730
    Lifted::new(0x7000f96970, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000f96a13> as *const (), 0), // +0x8aa970 sub_8aa970
    Lifted::new(0x7000f96a40, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000f96ae3> as *const (), 0), // +0x8aaa40 sub_8aaa40
    Lifted::new(0x7000f96b10, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000f96bb3> as *const (), 0), // +0x8aab10 sub_8aab10
    Lifted::new(0x7000f96be0, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000f96c83> as *const (), 0), // +0x8aabe0 sub_8aabe0
    Lifted::new(0x7000f96cb0, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000f96d53> as *const (), 0), // +0x8aacb0 sub_8aacb0
    Lifted::new(0x7000f96d80, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000f96e23> as *const (), 0), // +0x8aad80 sub_8aad80
    Lifted::new(0x7000f96e50, tpl_100::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000f96eca> as *const (), 0), // +0x8aae50 sub_8aae50
    Lifted::new(0x7000f96ef0, tpl_1185::<0x8, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000f96f88> as *const (), 0), // +0x8aaef0 sub_8aaef0
    Lifted::new(0x7000f96fb0, fn_7000f96fb0 as *const (), 0), // +0x8aafb0 sub_8aafb0
    Lifted::new(0x7000f97190, tpl_33::<0x700177c312> as *const (), 0), // +0x8ab190 sub_8ab190
    Lifted::new(0x7000f971c0, fn_7000f971c0 as *const (), 0), // +0x8ab1c0 sub_8ab1c0
    Lifted::new(0x7000f97210, tpl_1149::<0x10, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x18, 0x7001d54968, 0x7000f97289> as *const (), 0), // +0x8ab210 sub_8ab210
    Lifted::new(0x7000f972b0, tpl_33::<0x700177c312> as *const (), 0), // +0x8ab2b0 sub_8ab2b0
    Lifted::new(0x7000f972e0, fn_7000f972e0 as *const (), 0), // +0x8ab2e0 sub_8ab2e0
    Lifted::new(0x7000f97360, tpl_33::<0x700177c312> as *const (), 0), // +0x8ab360 sub_8ab360
    Lifted::new(0x7000f97390, tpl_33::<0x700177c312> as *const (), 0), // +0x8ab390 sub_8ab390
    Lifted::new(0x7000f973c0, tpl_33::<0x700177c312> as *const (), 0), // +0x8ab3c0 sub_8ab3c0
    Lifted::new(0x7000f973f0, tpl_33::<0x700177c312> as *const (), 0), // +0x8ab3f0 sub_8ab3f0
    Lifted::new(0x7000f97420, tpl_133::<0x700070c370, 0x700177c312> as *const (), 0), // +0x8ab420 sub_8ab420
    Lifted::new(0x7000f97460, tpl_33::<0x700177c312> as *const (), 0), // +0x8ab460 sub_8ab460
    Lifted::new(0x7000f97490, fn_7000f97490 as *const (), 0), // +0x8ab490 sub_8ab490
    Lifted::new(0x7000f974f0, fn_7000f974f0 as *const (), 0), // +0x8ab4f0 sub_8ab4f0
    Lifted::new(0x7000f97680, fn_7000f97680 as *const (), 0), // +0x8ab680 sub_8ab680
    Lifted::new(0x7000f97cf0, fn_7000f97cf0 as *const (), 0), // +0x8abcf0 sub_8abcf0
    Lifted::new(0x7000f97d90, fn_7000f97d90 as *const (), 0), // +0x8abd90 sub_8abd90
    Lifted::new(0x7000f97e50, tpl_1711::<0x700185e168, 0x3, 0x9, 0x7000f974f0, 0x10> as *const (), 0), // +0x8abe50 sub_8abe50
    Lifted::new(0x7000f97e90, fn_7000f97e90 as *const (), 0), // +0x8abe90 sub_8abe90
    Lifted::new(0x7000f97f00, fn_7000f97f00 as *const (), 0), // +0x8abf00 sub_8abf00
    Lifted::new(0x7000f97f90, fn_7000f97f90 as *const (), 0), // +0x8abf90 sub_8abf90
    Lifted::new(0x7000f98090, fn_7000f98090 as *const (), 0), // +0x8ac090 sub_8ac090
    Lifted::new(0x7000f98100, fn_7000f98100 as *const (), 2), // +0x8ac100 sub_8ac100
    Lifted::new(0x7000f98230, tpl_1711::<0x70017d2c0c, 0x1, 0x9, 0x7000f974f0, 0x8> as *const (), 0), // +0x8ac230 sub_8ac230
    Lifted::new(0x7000f98270, fn_7000f98270 as *const (), 2), // +0x8ac270 sub_8ac270
    Lifted::new(0x7000f98570, fn_7000f98570 as *const (), 0), // +0x8ac570 sub_8ac570
    Lifted::new(0x7000f98670, fn_7000f98670 as *const (), 0), // +0x8ac670 sub_8ac670
    Lifted::new(0x7000f98770, tpl_196 as *const (), 1), // +0x8ac770 sub_8ac770
    Lifted::new(0x7000f98780, fn_7000f98780 as *const (), 0), // +0x8ac780 sub_8ac780
    Lifted::new(0x7000f98830, tpl_500::<0x7000f98833, 0x7000f98834, 0x7000f98835, 0x7000f98836, 0x7000f98837, 0x7000f98838, 0x7000f98839, 0x7000f9883a, 0x7000f9883b, 0x7000f9883c, 0x7000f9883d, 0x7000f9883e, 0x7000f9883f, 0x700073c900> as *const (), 0), // +0x8ac830 sub_8ac830
    Lifted::new(0x7000f98850, fn_7000f98850 as *const (), 0), // +0x8ac850 sub_8ac850
    Lifted::new(0x7000f988d0, fn_7000f988d0 as *const (), 2), // +0x8ac8d0 sub_8ac8d0
    Lifted::new(0x7000f98a50, fn_7000f98a50 as *const (), 0), // +0x8aca50 sub_8aca50
    Lifted::new(0x7000f98a60, fn_7000f98a60 as *const (), 0), // +0x8aca60 sub_8aca60
    Lifted::new(0x7000f98af0, fn_7000f98af0 as *const (), 0), // +0x8acaf0 sub_8acaf0
    Lifted::new(0x7000f98b90, fn_7000f98b90 as *const (), 0), // +0x8acb90 sub_8acb90
    Lifted::new(0x7000f98c30, fn_7000f98c30 as *const (), 0), // +0x8acc30 sub_8acc30
    Lifted::new(0x7000f98f50, fn_7000f98f50 as *const (), 0), // +0x8acf50 sub_8acf50
    Lifted::new(0x7000f990a0, fn_7000f990a0 as *const (), 0), // +0x8ad0a0 sub_8ad0a0
    Lifted::new(0x7000f992c0, fn_7000f992c0 as *const (), 0), // +0x8ad2c0 sub_8ad2c0
    Lifted::new(0x7000f99470, fn_7000f99470 as *const (), 0), // +0x8ad470 sub_8ad470
    Lifted::new(0x7000f99510, fn_7000f99510 as *const (), 0), // +0x8ad510 sub_8ad510
    Lifted::new(0x7000f99630, fn_7000f99630 as *const (), 0), // +0x8ad630 sub_8ad630
    Lifted::new(0x7000f99910, fn_7000f99910 as *const (), 0), // +0x8ad910 sub_8ad910
    Lifted::new(0x7000f99ee0, fn_7000f99ee0 as *const (), 0), // +0x8adee0 sub_8adee0
    Lifted::new(0x7000f9a050, fn_7000f9a050 as *const (), 0), // +0x8ae050 sub_8ae050
    Lifted::new(0x7000f9a2d0, fn_7000f9a2d0 as *const (), 0), // +0x8ae2d0 sub_8ae2d0
    Lifted::new(0x7000f9a5a0, fn_7000f9a5a0 as *const (), 0), // +0x8ae5a0 sub_8ae5a0
    Lifted::new(0x7000f9a680, fn_7000f9a680 as *const (), 0), // +0x8ae680 sub_8ae680
    Lifted::new(0x7000f9a730, fn_7000f9a730 as *const (), 0), // +0x8ae730 sub_8ae730
    Lifted::new(0x7000f9a7f0, fn_7000f9a7f0 as *const (), 0), // +0x8ae7f0 sub_8ae7f0
    Lifted::new(0x7000f9a9a0, fn_7000f9a9a0 as *const (), 0), // +0x8ae9a0 sub_8ae9a0
    Lifted::new(0x7000f9aa90, tpl_1712::<0xb, 0x8, 0xffffffffffffffff, 0x7, 0x358, 0xe8, 0x3b8, 0xf0, 0x60, 0xa8, 0xb0, 0x98, 0x70007a6e40, 0x70017dac2c, 0x7000fa5b60> as *const (), 0), // +0x8aea90 sub_8aea90
    Lifted::new(0x7000f9abb0, fn_7000f9abb0 as *const (), 0), // +0x8aebb0 sub_8aebb0
    Lifted::new(0x7000f9adf0, tpl_1712::<0xb, 0x8, 0xffffffffffffffff, 0x7, 0x358, 0xe8, 0x3b8, 0xf0, 0x60, 0xa8, 0xb0, 0x98, 0x70007a6e40, 0x7001860238, 0x7000fa5b60> as *const (), 0), // +0x8aedf0 sub_8aedf0
    Lifted::new(0x7000f9af10, fn_7000f9af10 as *const (), 0), // +0x8aef10 sub_8aef10
    Lifted::new(0x7000f9b570, tpl_1712::<0xb, 0x8, 0xffffffffffffffff, 0x7, 0x358, 0xe8, 0x3b8, 0xf0, 0x60, 0xa8, 0xb0, 0x98, 0x70007a6e40, 0x70018608d0, 0x7000fa5b60> as *const (), 0), // +0x8af570 sub_8af570
    Lifted::new(0x7000f9b690, tpl_1712::<0xb, 0x8, 0xffffffffffffffff, 0x7, 0x358, 0xe8, 0x3b8, 0xf0, 0x60, 0xa8, 0xb0, 0x98, 0x70007a6e40, 0x70017dac2c, 0x7000fa5b60> as *const (), 0), // +0x8af690 sub_8af690
    Lifted::new(0x7000f9b7b0, fn_7000f9b7b0 as *const (), 0), // +0x8af7b0 sub_8af7b0
    Lifted::new(0x7000f9b870, fn_7000f9b870 as *const (), 2), // +0x8af870 sub_8af870
    Lifted::new(0x7000f9ba50, fn_7000f9ba50 as *const (), 0), // +0x8afa50 sub_8afa50
    Lifted::new(0x7000f9c0a0, fn_7000f9c0a0 as *const (), 2), // +0x8b00a0 sub_8b00a0
    Lifted::new(0x7000f9c190, fn_7000f9c190 as *const (), 0), // +0x8b0190 sub_8b0190
    Lifted::new(0x7000f9c2e0, fn_7000f9c2e0 as *const (), 0), // +0x8b02e0 sub_8b02e0
    Lifted::new(0x7000f9c900, fn_7000f9c900 as *const (), 0), // +0x8b0900 sub_8b0900
    Lifted::new(0x7000f9cfe0, fn_7000f9cfe0 as *const (), 0), // +0x8b0fe0 sub_8b0fe0
    Lifted::new(0x7000f9d2b0, fn_7000f9d2b0 as *const (), 0), // +0x8b12b0 sub_8b12b0
    Lifted::new(0x7000f9d5d0, fn_7000f9d5d0 as *const (), 0), // +0x8b15d0 sub_8b15d0
    Lifted::new(0x7000f9d7f0, fn_7000f9d7f0 as *const (), 0), // +0x8b17f0 sub_8b17f0
    Lifted::new(0x7000f9d890, fn_7000f9d890 as *const (), 0), // +0x8b1890 sub_8b1890
    Lifted::new(0x7000f9d9c0, fn_7000f9d9c0 as *const (), 0), // +0x8b19c0 sub_8b19c0
    Lifted::new(0x7000f9db70, fn_7000f9db70 as *const (), 0), // +0x8b1b70 sub_8b1b70
    Lifted::new(0x7000f9dbc0, fn_7000f9dbc0 as *const (), 0), // +0x8b1bc0 sub_8b1bc0
    Lifted::new(0x7000f9dcd0, fn_7000f9dcd0 as *const (), 1), // +0x8b1cd0 sub_8b1cd0
    Lifted::new(0x7000f9de00, fn_7000f9de00 as *const (), 0), // +0x8b1e00 sub_8b1e00
    Lifted::new(0x7000f9de60, fn_7000f9de60 as *const (), 2), // +0x8b1e60 sub_8b1e60
    Lifted::new(0x7000f9e1e0, fn_7000f9e1e0 as *const (), 0), // +0x8b21e0 sub_8b21e0
    Lifted::new(0x7000f9e270, fn_7000f9e270 as *const (), 0), // +0x8b2270 sub_8b2270
    Lifted::new(0x7000f9e650, fn_7000f9e650 as *const (), 0), // +0x8b2650 sub_8b2650
    Lifted::new(0x7000f9e760, fn_7000f9e760 as *const (), 0), // +0x8b2760 sub_8b2760
    Lifted::new(0x7000f9e8a0, fn_7000f9e8a0 as *const (), 0), // +0x8b28a0 sub_8b28a0
    Lifted::new(0x7000f9ea20, fn_7000f9ea20 as *const (), 2), // +0x8b2a20 sub_8b2a20
    Lifted::new(0x7000f9ed20, fn_7000f9ed20 as *const (), 0), // +0x8b2d20 sub_8b2d20
    Lifted::new(0x7000f9ee60, fn_7000f9ee60 as *const (), 0), // +0x8b2e60 sub_8b2e60
    Lifted::new(0x7000f9f040, fn_7000f9f040 as *const (), 0), // +0x8b3040 sub_8b3040
    Lifted::new(0x7000fa0340, fn_7000fa0340 as *const (), 0), // +0x8b4340 sub_8b4340
    Lifted::new(0x7000fa0380, tpl_144 as *const (), 1), // +0x8b4380 sub_8b4380
    Lifted::new(0x7000fa0390, fn_7000fa0390 as *const (), 1), // +0x8b4390 sub_8b4390
    Lifted::new(0x7000fa0430, fn_7000fa0430 as *const (), 1), // +0x8b4430 sub_8b4430
    Lifted::new(0x7000fa04e0, fn_7000fa04e0 as *const (), 2), // +0x8b44e0 sub_8b44e0
    Lifted::new(0x7000fa0920, fn_7000fa0920 as *const (), 2), // +0x8b4920 sub_8b4920
    Lifted::new(0x7000fa0f70, tpl_566::<0x7000f91c70, 0x7000f9d9c0> as *const (), 0), // +0x8b4f70 sub_8b4f70
    Lifted::new(0x7000fa0ff0, fn_7000fa0ff0 as *const (), 0), // +0x8b4ff0 sub_8b4ff0
    Lifted::new(0x7000fa1060, fn_7000fa1060 as *const (), 1), // +0x8b5060 sub_8b5060
    Lifted::new(0x7000fa10a0, fn_7000fa10a0 as *const (), 0), // +0x8b50a0 sub_8b50a0
    Lifted::new(0x7000fa12d0, fn_7000fa12d0 as *const (), 0), // +0x8b52d0 sub_8b52d0
    Lifted::new(0x7000fa1570, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000fa1581> as *const (), 0), // +0x8b5570 sub_8b5570
    Lifted::new(0x7000fa15d0, fn_7000fa15d0 as *const (), 2), // +0x8b55d0 sub_8b55d0
    Lifted::new(0x7000fa1a30, fn_7000fa1a30 as *const (), 0), // +0x8b5a30 sub_8b5a30
    Lifted::new(0x7000fa1a70, fn_7000fa1a70 as *const (), 0), // +0x8b5a70 sub_8b5a70
    Lifted::new(0x7000fa1af0, tpl_202::<0x200> as *const (), 1), // +0x8b5af0 sub_8b5af0
    Lifted::new(0x7000fa1b00, fn_7000fa1b00 as *const (), 2), // +0x8b5b00 sub_8b5b00
    Lifted::new(0x7000fa2050, tpl_587 as *const (), 3), // +0x8b6050 sub_8b6050
    Lifted::new(0x7000fa21a0, fn_7000fa21a0 as *const (), 2), // +0x8b61a0 sub_8b61a0
    Lifted::new(0x7000fa28b0, fn_7000fa28b0 as *const (), 0), // +0x8b68b0 sub_8b68b0
    Lifted::new(0x7000fa29e0, fn_7000fa29e0 as *const (), 0), // +0x8b69e0 sub_8b69e0
    Lifted::new(0x7000fa2ab0, fn_7000fa2ab0 as *const (), 0), // +0x8b6ab0 sub_8b6ab0
    Lifted::new(0x7000fa2d20, fn_7000fa2d20 as *const (), 0), // +0x8b6d20 sub_8b6d20
    Lifted::new(0x7000fa3020, fn_7000fa3020 as *const (), 0), // +0x8b7020 sub_8b7020
    Lifted::new(0x7000fa3580, fn_7000fa3580 as *const (), 0), // +0x8b7580 sub_8b7580
    Lifted::new(0x7000fa3850, fn_7000fa3850 as *const (), 0), // +0x8b7850 sub_8b7850
    Lifted::new(0x7000fa3960, fn_7000fa3960 as *const (), 0), // +0x8b7960 sub_8b7960
    Lifted::new(0x7000fa3e80, fn_7000fa3e80 as *const (), 0), // +0x8b7e80 sub_8b7e80
    Lifted::new(0x7000fa3f20, fn_7000fa3f20 as *const (), 0), // +0x8b7f20 sub_8b7f20
    Lifted::new(0x7000fa41a0, fn_7000fa41a0 as *const (), 0), // +0x8b81a0 sub_8b81a0
    Lifted::new(0x7000fa4280, fn_7000fa4280 as *const (), 0), // +0x8b8280 sub_8b8280
    Lifted::new(0x7000fa4960, fn_7000fa4960 as *const (), 0), // +0x8b8960 sub_8b8960
    Lifted::new(0x7000fa4b20, fn_7000fa4b20 as *const (), 0), // +0x8b8b20 sub_8b8b20
    Lifted::new(0x7000fa4f10, fn_7000fa4f10 as *const (), 0), // +0x8b8f10 sub_8b8f10
    Lifted::new(0x7000fa53f0, fn_7000fa53f0 as *const (), 0), // +0x8b93f0 sub_8b93f0
    Lifted::new(0x7000fa54b0, fn_7000fa54b0 as *const (), 0), // +0x8b94b0 sub_8b94b0
    Lifted::new(0x7000fa5520, fn_7000fa5520 as *const (), 0), // +0x8b9520 sub_8b9520
    Lifted::new(0x7000fa5570, fn_7000fa5570 as *const (), 0), // +0x8b9570 sub_8b9570
    Lifted::new(0x7000fa55a0, fn_7000fa55a0 as *const (), 0), // +0x8b95a0 sub_8b95a0
    Lifted::new(0x7000fa5790, fn_7000fa5790 as *const (), 0), // +0x8b9790 sub_8b9790
    Lifted::new(0x7000fa5840, fn_7000fa5840 as *const (), 0), // +0x8b9840 sub_8b9840
    Lifted::new(0x7000fa5950, fn_7000fa5950 as *const (), 0), // +0x8b9950 sub_8b9950
    Lifted::new(0x7000fa5a20, fn_7000fa5a20 as *const (), 0), // +0x8b9a20 sub_8b9a20
    Lifted::new(0x7000fa5ae0, fn_7000fa5ae0 as *const (), 0), // +0x8b9ae0 sub_8b9ae0
    Lifted::new(0x7000fa5b60, fn_7000fa5b60 as *const (), 0), // +0x8b9b60 sub_8b9b60
    Lifted::new(0x7000fa5c70, fn_7000fa5c70 as *const (), 0), // +0x8b9c70 sub_8b9c70
    Lifted::new(0x7000fa5cc0, fn_7000fa5cc0 as *const (), 0), // +0x8b9cc0 sub_8b9cc0
    Lifted::new(0x7000fa5e50, fn_7000fa5e50 as *const (), 0), // +0x8b9e50 sub_8b9e50
    Lifted::new(0x7000fa5e80, fn_7000fa5e80 as *const (), 0), // +0x8b9e80 sub_8b9e80
    Lifted::new(0x7000fa5ef0, fn_7000fa5ef0 as *const (), 0), // +0x8b9ef0 sub_8b9ef0
    Lifted::new(0x7000fa5fc0, fn_7000fa5fc0 as *const (), 0), // +0x8b9fc0 sub_8b9fc0
    Lifted::new(0x7000fa6080, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000fa6091> as *const (), 0), // +0x8ba080 sub_8ba080
    Lifted::new(0x7000fa60d0, tpl_322::<0x38, 0x40> as *const (), 1), // +0x8ba0d0 sub_8ba0d0
    Lifted::new(0x7000fa60e0, fn_7000fa60e0 as *const (), 0), // +0x8ba0e0 sub_8ba0e0
    Lifted::new(0x7000fa6160, fn_7000fa6160 as *const (), 0), // +0x8ba160 sub_8ba160
    Lifted::new(0x7000fa62d0, fn_7000fa62d0 as *const (), 0), // +0x8ba2d0 sub_8ba2d0
    Lifted::new(0x7000fa64e0, tpl_623::<0x7000f9de60> as *const (), 0), // +0x8ba4e0 sub_8ba4e0
    Lifted::new(0x7000fa64f0, fn_7000fa64f0 as *const (), 0), // +0x8ba4f0 sub_8ba4f0
    Lifted::new(0x7000fa65c0, tpl_995::<0x700181a358, 0x70017a0290, 0x7001d54ef8, 0x7000fa65d8> as *const (), 0), // +0x8ba5c0 sub_8ba5c0
    Lifted::new(0x7000fa65e0, fn_7000fa65e0 as *const (), 0), // +0x8ba5e0 sub_8ba5e0
    Lifted::new(0x7000fa6690, fn_7000fa6690 as *const (), 0), // +0x8ba690 sub_8ba690
    Lifted::new(0x7000fa6730, fn_7000fa6730 as *const (), 0), // +0x8ba730 sub_8ba730
    Lifted::new(0x7000fa6820, fn_7000fa6820 as *const (), 0), // +0x8ba820 sub_8ba820
    Lifted::new(0x7000fa6980, tpl_260::<0x8> as *const (), 0), // +0x8ba980 sub_8ba980
    Lifted::new(0x7000fa6990, fn_7000fa6990 as *const (), 0), // +0x8ba990 sub_8ba990
    Lifted::new(0x7000fa6a80, fn_7000fa6a80 as *const (), 0), // +0x8baa80 sub_8baa80
    Lifted::new(0x7000fa6b00, fn_7000fa6b00 as *const (), 0), // +0x8bab00 sub_8bab00
    Lifted::new(0x7000fa6d70, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fa6e77, 0x7000f95740, 0x70007f0910, 0x7000fa6e93> as *const (), 0), // +0x8bad70 sub_8bad70
    Lifted::new(0x7000fa6ee0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fa6fe7, 0x7000f957f0, 0x70007f0910, 0x7000fa7003> as *const (), 0), // +0x8baee0 sub_8baee0
    Lifted::new(0x7000fa7050, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fa7157, 0x7000f958a0, 0x70007f0910, 0x7000fa7173> as *const (), 0), // +0x8bb050 sub_8bb050
    Lifted::new(0x7000fa71c0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fa72c7, 0x7000f95950, 0x70007f0910, 0x7000fa72e3> as *const (), 0), // +0x8bb1c0 sub_8bb1c0
    Lifted::new(0x7000fa7330, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fa7437, 0x7000f95a00, 0x70007f0910, 0x7000fa7453> as *const (), 0), // +0x8bb330 sub_8bb330
    Lifted::new(0x7000fa74a0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fa75a7, 0x7000f95ab0, 0x70007f0910, 0x7000fa75c3> as *const (), 0), // +0x8bb4a0 sub_8bb4a0
    Lifted::new(0x7000fa7610, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fa7717, 0x7000f95b60, 0x70007f0910, 0x7000fa7733> as *const (), 0), // +0x8bb610 sub_8bb610
    Lifted::new(0x7000fa7780, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fa7887, 0x7000f95c10, 0x70007f0910, 0x7000fa78a3> as *const (), 0), // +0x8bb780 sub_8bb780
    Lifted::new(0x7000fa78f0, tpl_999::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000fa79a0> as *const (), 0), // +0x8bb8f0 sub_8bb8f0
    Lifted::new(0x7000fa79d0, tpl_284::<0x7000fa6ee0, 0x70017d37a0, 0x70017790e0, 0x7000fa7b8d> as *const (), 0), // +0x8bb9d0 sub_8bb9d0
    Lifted::new(0x7000fa7c00, tpl_284::<0x7000fa7050, 0x70017d37a0, 0x70017790e0, 0x7000fa7dbd> as *const (), 0), // +0x8bbc00 sub_8bbc00
    Lifted::new(0x7000fa7e30, tpl_286::<0x7000fa7610, 0x70017d37a0, 0x70017790e0, 0x7000fa801b> as *const (), 0), // +0x8bbe30 sub_8bbe30
    Lifted::new(0x7000fa80a0, tpl_286::<0x7000fa7780, 0x70017d37a0, 0x70017790e0, 0x7000fa828b> as *const (), 0), // +0x8bc0a0 sub_8bc0a0
    Lifted::new(0x7000fa8310, tpl_1159::<0x7000fa71c0, 0x70017d37a0, 0x70017790e0, 0x7000fa84de> as *const (), 0), // +0x8bc310 sub_8bc310
    Lifted::new(0x7000fa8560, tpl_1159::<0x7000fa7330, 0x70017d37a0, 0x70017790e0, 0x7000fa872e> as *const (), 0), // +0x8bc560 sub_8bc560
    Lifted::new(0x7000fa87b0, tpl_286::<0x7000fa74a0, 0x70017d37a0, 0x70017790e0, 0x7000fa899b> as *const (), 0), // +0x8bc7b0 sub_8bc7b0
    Lifted::new(0x7000fa8a20, tpl_1159::<0x7000fa6d70, 0x70017d37a0, 0x70017790e0, 0x7000fa8bee> as *const (), 0), // +0x8bca20 sub_8bca20
    Lifted::new(0x7000fa8c70, fn_7000fa8c70 as *const (), 0), // +0x8bcc70 sub_8bcc70
    Lifted::new(0x7000fa8ec0, tpl_1713::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000fa9085, 0x70007f0910, 0x7000fa908b, 0x7000fa9130, 0x7000fa9094> as *const (), 0), // +0x8bcec0 sub_8bcec0
    Lifted::new(0x7000fa9110, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000fa9120> as *const (), 0), // +0x8bd110 sub_8bd110
    Lifted::new(0x7000fa9130, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000fa9140> as *const (), 0), // +0x8bd130 sub_8bd130
    Lifted::new(0x7000fa9150, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa9160> as *const (), 0), // +0x8bd150 sub_8bd150
    Lifted::new(0x7000fa9170, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa9180> as *const (), 0), // +0x8bd170 sub_8bd170
    Lifted::new(0x7000fa9190, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa91a0> as *const (), 0), // +0x8bd190 sub_8bd190
    Lifted::new(0x7000fa91b0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa91c0> as *const (), 0), // +0x8bd1b0 sub_8bd1b0
    Lifted::new(0x7000fa91d0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa91e0> as *const (), 0), // +0x8bd1d0 sub_8bd1d0
    Lifted::new(0x7000fa91f0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa9200> as *const (), 0), // +0x8bd1f0 sub_8bd1f0
    Lifted::new(0x7000fa9210, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa9220> as *const (), 0), // +0x8bd210 sub_8bd210
    Lifted::new(0x7000fa9230, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa9240> as *const (), 0), // +0x8bd230 sub_8bd230
    Lifted::new(0x7000fa9250, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fa9260> as *const (), 0), // +0x8bd250 sub_8bd250
    Lifted::new(0x7000fa9270, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000fa9280> as *const (), 0), // +0x8bd270 sub_8bd270
    Lifted::new(0x7000fa9290, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000fa92e4, 0x70007f0910, 0x7000fa92ff> as *const (), 0), // +0x8bd290 sub_8bd290
    Lifted::new(0x7000fa9320, tpl_1001::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000fa9378, 0x70007f0910, 0x7000fa9393> as *const (), 0), // +0x8bd320 sub_8bd320
    Lifted::new(0x7000fa93c0, fn_7000fa93c0 as *const (), 0), // +0x8bd3c0 sub_8bd3c0
    Lifted::new(0x7000fa9420, fn_7000fa9420 as *const (), 2), // +0x8bd420 sub_8bd420
    Lifted::new(0x7000fa9800, fn_7000fa9800 as *const (), 2), // +0x8bd800 sub_8bd800
    Lifted::new(0x7000fa9ae0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8bdae0 sub_8bdae0
    Lifted::new(0x7000fa9b20, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8bdb20 sub_8bdb20
    Lifted::new(0x7000fa9b60, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8bdb60 sub_8bdb60
    Lifted::new(0x7000fa9ba0, fn_7000fa9ba0 as *const (), 0), // +0x8bdba0 sub_8bdba0
    Lifted::new(0x7000fa9cd0, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000fa9e5d, 0x7000fb0e10, 0x7000fa9e63, 0x70007f0910, 0x7000fa9e69> as *const (), 0), // +0x8bdcd0 sub_8bdcd0
    Lifted::new(0x7000fa9ed0, fn_7000fa9ed0 as *const (), 0), // +0x8bded0 sub_8bded0
    Lifted::new(0x7000faa100, fn_7000faa100 as *const (), 2), // +0x8be100 sub_8be100
    Lifted::new(0x7000faa3a0, tpl_1205::<0x7000fb0e90, 0x7000fa9ba0, 0x700177c312, 0x7001d54968, 0x7000faa544, 0x7000fb0e50, 0x7000faa54a> as *const (), 0), // +0x8be3a0 sub_8be3a0
    Lifted::new(0x7000faa5c0, tpl_963::<0x19, 0x10, 0x7000faa5c0, 0x700177c312> as *const (), 0), // +0x8be5c0 sub_8be5c0
    Lifted::new(0x7000faa630, fn_7000faa630 as *const (), 3), // +0x8be630 sub_8be630
    Lifted::new(0x7000faa730, fn_7000faa730 as *const (), 0), // +0x8be730 sub_8be730
    Lifted::new(0x7000faa7f0, fn_7000faa7f0 as *const (), 0), // +0x8be7f0 sub_8be7f0
    Lifted::new(0x7000faa880, tpl_1141::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000faffb0, 0x70017d37c0, 0x70017790e0, 0x7000faab31> as *const (), 2), // +0x8be880 sub_8be880
    Lifted::new(0x7000faabe0, fn_7000faabe0 as *const (), 2), // +0x8bebe0 sub_8bebe0
    Lifted::new(0x7000faaf90, tpl_898::<0x700177c0e0> as *const (), 0), // +0x8bef90 sub_8bef90
    Lifted::new(0x7000faafd0, tpl_392::<0x700177c0e0> as *const (), 0), // +0x8befd0 sub_8befd0
    Lifted::new(0x7000fab000, tpl_68 as *const (), 1), // +0x8bf000 sub_8bf000
    Lifted::new(0x7000fab0b0, tpl_68 as *const (), 1), // +0x8bf0b0 sub_8bf0b0
    Lifted::new(0x7000fab160, tpl_1714::<0x7001779e20, 0x7001861030, 0x8> as *const (), 0), // +0x8bf160 sub_8bf160
    Lifted::new(0x7000fab1e0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x8bf1e0 sub_8bf1e0
    Lifted::new(0x7000fab280, fn_7000fab280 as *const (), 0), // +0x8bf280 sub_8bf280
    Lifted::new(0x7000fab2e0, fn_7000fab2e0 as *const (), 2), // +0x8bf2e0 sub_8bf2e0
    Lifted::new(0x7000fab640, fn_7000fab640 as *const (), 0), // +0x8bf640 sub_8bf640
    Lifted::new(0x7000fab6a0, fn_7000fab6a0 as *const (), 0), // +0x8bf6a0 sub_8bf6a0
    Lifted::new(0x7000fab740, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000fab7e3> as *const (), 0), // +0x8bf740 sub_8bf740
    Lifted::new(0x7000fab810, tpl_1715::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x28, 0x7000faa730, 0x7001d54968, 0x7000fab882> as *const (), 0), // +0x8bf810 sub_8bf810
    Lifted::new(0x7000fab8a0, tpl_1716::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000fab943> as *const (), 0), // +0x8bf8a0 sub_8bf8a0
    Lifted::new(0x7000fab970, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000faba13> as *const (), 0), // +0x8bf970 sub_8bf970
    Lifted::new(0x7000faba40, fn_7000faba40 as *const (), 0), // +0x8bfa40 sub_8bfa40
    Lifted::new(0x7000fabb40, fn_7000fabb40 as *const (), 0), // +0x8bfb40 sub_8bfb40
    Lifted::new(0x7000fabc00, tpl_1208::<0x8, 0x7000fa9ba0, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x7000fabc6a> as *const (), 0), // +0x8bfc00 sub_8bfc00
    Lifted::new(0x7000fabc90, fn_7000fabc90 as *const (), 0), // +0x8bfc90 sub_8bfc90
    Lifted::new(0x7000fabd10, fn_7000fabd10 as *const (), 0), // +0x8bfd10 sub_8bfd10
    Lifted::new(0x7000fabdd0, fn_7000fabdd0 as *const (), 0), // +0x8bfdd0 sub_8bfdd0
    Lifted::new(0x7000fabe50, tpl_1717::<0x8, 0x700177c312> as *const (), 0), // +0x8bfe50 sub_8bfe50
    Lifted::new(0x7000fabea0, fn_7000fabea0 as *const (), 2), // +0x8bfea0 sub_8bfea0
    Lifted::new(0x7000fabfc0, fn_7000fabfc0 as *const (), 2), // +0x8bffc0 sub_8bffc0
    Lifted::new(0x7000fac250, fn_7000fac250 as *const (), 0), // +0x8c0250 sub_8c0250
    Lifted::new(0x7000fac330, fn_7000fac330 as *const (), 0), // +0x8c0330 sub_8c0330
    Lifted::new(0x7000fac450, fn_7000fac450 as *const (), 2), // +0x8c0450 sub_8c0450
    Lifted::new(0x7000fac600, fn_7000fac600 as *const (), 1), // +0x8c0600 sub_8c0600
    Lifted::new(0x7000fac620, fn_7000fac620 as *const (), 1), // +0x8c0620 sub_8c0620
    Lifted::new(0x7000fac650, fn_7000fac650 as *const (), 0), // +0x8c0650 sub_8c0650
    Lifted::new(0x7000fac720, fn_7000fac720 as *const (), 0), // +0x8c0720 sub_8c0720
    Lifted::new(0x7000fac940, fn_7000fac940 as *const (), 0), // +0x8c0940 sub_8c0940
    Lifted::new(0x7000faca60, fn_7000faca60 as *const (), 0), // +0x8c0a60 sub_8c0a60
    Lifted::new(0x7000facb70, fn_7000facb70 as *const (), 2), // +0x8c0b70 sub_8c0b70
    Lifted::new(0x7000face30, fn_7000face30 as *const (), 0), // +0x8c0e30 sub_8c0e30
    Lifted::new(0x7000facf40, fn_7000facf40 as *const (), 2), // +0x8c0f40 sub_8c0f40
    Lifted::new(0x7000fad080, fn_7000fad080 as *const (), 1), // +0x8c1080 sub_8c1080
    Lifted::new(0x7000fad160, fn_7000fad160 as *const (), 2), // +0x8c1160 sub_8c1160
    Lifted::new(0x7000fad330, fn_7000fad330 as *const (), 2), // +0x8c1330 sub_8c1330
    Lifted::new(0x7000fad560, fn_7000fad560 as *const (), 0), // +0x8c1560 sub_8c1560
    Lifted::new(0x7000fad630, fn_7000fad630 as *const (), 2), // +0x8c1630 sub_8c1630
    Lifted::new(0x7000fad7a0, fn_7000fad7a0 as *const (), 1), // +0x8c17a0 sub_8c17a0
    Lifted::new(0x7000fad8d0, fn_7000fad8d0 as *const (), 0), // +0x8c18d0 sub_8c18d0
    Lifted::new(0x7000fad9e0, fn_7000fad9e0 as *const (), 1), // +0x8c19e0 sub_8c19e0
    Lifted::new(0x7000fada80, fn_7000fada80 as *const (), 1), // +0x8c1a80 sub_8c1a80
    Lifted::new(0x7000fadaf0, tpl_1705::<0x30, 0x7000734360> as *const (), 0), // +0x8c1af0 sub_8c1af0
    Lifted::new(0x7000fadb10, fn_7000fadb10 as *const (), 1), // +0x8c1b10 sub_8c1b10
    Lifted::new(0x7000fadbb0, fn_7000fadbb0 as *const (), 1), // +0x8c1bb0 sub_8c1bb0
    Lifted::new(0x7000fadbd0, fn_7000fadbd0 as *const (), 0), // +0x8c1bd0 sub_8c1bd0
    Lifted::new(0x7000fadd00, fn_7000fadd00 as *const (), 1), // +0x8c1d00 sub_8c1d00
    Lifted::new(0x7000fadd60, fn_7000fadd60 as *const (), 1), // +0x8c1d60 sub_8c1d60
    Lifted::new(0x7000faddb0, fn_7000faddb0 as *const (), 2), // +0x8c1db0 sub_8c1db0
    Lifted::new(0x7000fae0d0, fn_7000fae0d0 as *const (), 0), // +0x8c20d0 sub_8c20d0
    Lifted::new(0x7000fae1a0, fn_7000fae1a0 as *const (), 0), // +0x8c21a0 sub_8c21a0
    Lifted::new(0x7000fae200, fn_7000fae200 as *const (), 2), // +0x8c2200 sub_8c2200
    Lifted::new(0x7000fae480, fn_7000fae480 as *const (), 0), // +0x8c2480 sub_8c2480
    Lifted::new(0x7000fae4e0, fn_7000fae4e0 as *const (), 0), // +0x8c24e0 sub_8c24e0
    Lifted::new(0x7000fae640, fn_7000fae640 as *const (), 0), // +0x8c2640 sub_8c2640
    Lifted::new(0x7000fae680, tpl_618::<0x8> as *const (), 0), // +0x8c2680 sub_8c2680
    Lifted::new(0x7000fae690, fn_7000fae690 as *const (), 1), // +0x8c2690 sub_8c2690
    Lifted::new(0x7000fae6b0, fn_7000fae6b0 as *const (), 0), // +0x8c26b0 sub_8c26b0
    Lifted::new(0x7000fae980, fn_7000fae980 as *const (), 0), // +0x8c2980 sub_8c2980
    Lifted::new(0x7000fae9f0, fn_7000fae9f0 as *const (), 0), // +0x8c29f0 sub_8c29f0
    Lifted::new(0x7000faec60, fn_7000faec60 as *const (), 2), // +0x8c2c60 sub_8c2c60
    Lifted::new(0x7000faefa0, fn_7000faefa0 as *const (), 0), // +0x8c2fa0 sub_8c2fa0
    Lifted::new(0x7000faf730, fn_7000faf730 as *const (), 0), // +0x8c3730 sub_8c3730
    Lifted::new(0x7000faf7c0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000faf8c7, 0x7000fab000, 0x70007f0910, 0x7000faf8e3> as *const (), 0), // +0x8c37c0 sub_8c37c0
    Lifted::new(0x7000faf930, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000fafa37, 0x7000fab0b0, 0x70007f0910, 0x7000fafa53> as *const (), 0), // +0x8c3930 sub_8c3930
    Lifted::new(0x7000fafaa0, fn_7000fafaa0 as *const (), 0), // +0x8c3aa0 sub_8c3aa0
    Lifted::new(0x7000fafb30, fn_7000fafb30 as *const (), 0), // +0x8c3b30 sub_8c3b30
    Lifted::new(0x7000faffb0, tpl_284::<0x7000faf7c0, 0x70017d37a0, 0x70017790e0, 0x7000fb016d> as *const (), 0), // +0x8c3fb0 sub_8c3fb0
    Lifted::new(0x7000fb01e0, fn_7000fb01e0 as *const (), 2), // +0x8c41e0 sub_8c41e0
    Lifted::new(0x7000fb0510, tpl_1713::<0x7001779e20, 0x700177c0d0, 0x700177c0e0, 0x700177c312, 0x7001d54968, 0x7000fb06d5, 0x70007f0910, 0x7000fb06db, 0x7000fb0df0, 0x7000fb06e4> as *const (), 0), // +0x8c4510 sub_8c4510
    Lifted::new(0x7000fb0760, tpl_287 as *const (), 1), // +0x8c4760 sub_8c4760
    Lifted::new(0x7000fb0a80, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x8c4a80 sub_8c4a80
    Lifted::new(0x7000fb0af0, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x8c4af0 sub_8c4af0
    Lifted::new(0x7000fb0b60, fn_7000fb0b60 as *const (), 0), // +0x8c4b60 sub_8c4b60
    Lifted::new(0x7000fb0df0, tpl_292::<0x70017fdca0, 0x70017790e0, 0x7000fb0e00> as *const (), 0), // +0x8c4df0 sub_8c4df0
    Lifted::new(0x7000fb0e10, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fb0e20> as *const (), 0), // +0x8c4e10 sub_8c4e10
    Lifted::new(0x7000fb0e30, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fb0e40> as *const (), 0), // +0x8c4e30 sub_8c4e30
    Lifted::new(0x7000fb0e50, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fb0e60> as *const (), 0), // +0x8c4e50 sub_8c4e50
    Lifted::new(0x7000fb0e70, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000fb0e80> as *const (), 0), // +0x8c4e70 sub_8c4e70
    Lifted::new(0x7000fb0e90, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000fb0ee4, 0x70007f0910, 0x7000fb0eff> as *const (), 0), // +0x8c4e90 sub_8c4e90
    Lifted::new(0x7000fb0f20, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000fb0f6d, 0x70007f0910, 0x7000fb0f8b> as *const (), 0), // +0x8c4f20 sub_8c4f20
];
