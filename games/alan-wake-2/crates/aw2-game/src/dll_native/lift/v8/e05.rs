#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::new(0x70007f0b80, tpl_36::<0x8> as *const (), 1), // +0x104b80 sub_104b80
    Lifted::new(0x70007f0b90, tpl_49::<0x30> as *const (), 1), // +0x104b90 sub_104b90
    Lifted::new(0x70007f0ba0, tpl_50 as *const (), 1), // +0x104ba0 sub_104ba0
    Lifted::new(0x70007f0bb0, tpl_51::<0x18> as *const (), 1), // +0x104bb0 sub_104bb0
    Lifted::new(0x70007f0bc0, tpl_51::<0x20> as *const (), 1), // +0x104bc0 sub_104bc0
    Lifted::new(0x70007f0bd0, tpl_51::<0x8> as *const (), 1), // +0x104bd0 sub_104bd0
    Lifted::new(0x70007f0be0, tpl_51::<0x10> as *const (), 1), // +0x104be0 sub_104be0
    Lifted::new(0x70007f0bf0, tpl_51::<0x28> as *const (), 1), // +0x104bf0 sub_104bf0
    Lifted::new(0x70007f0c00, tpl_36::<0x18> as *const (), 1), // +0x104c00 sub_104c00
    Lifted::new(0x70007f0c10, tpl_36::<0x10> as *const (), 1), // +0x104c10 sub_104c10
    Lifted::new(0x70007f0c20, tpl_52::<0x10> as *const (), 1), // +0x104c20 sub_104c20
    Lifted::new(0x70007f0c30, tpl_36::<0x18> as *const (), 1), // +0x104c30 sub_104c30
    Lifted::new(0x70007f0c40, tpl_30 as *const (), 1), // +0x104c40 sub_104c40
    Lifted::new(0x70007f0c50, tpl_36::<0x8> as *const (), 1), // +0x104c50 sub_104c50
    Lifted::new(0x70007f0c60, tpl_36::<0x10> as *const (), 1), // +0x104c60 sub_104c60
    Lifted::new(0x70007f0c70, tpl_36::<0x28> as *const (), 1), // +0x104c70 sub_104c70
    Lifted::new(0x70007f0c80, tpl_36::<0x18> as *const (), 1), // +0x104c80 sub_104c80
    Lifted::new(0x70007f0c90, tpl_36::<0x60> as *const (), 1), // +0x104c90 sub_104c90
    Lifted::new(0x70007f0ca0, tpl_30 as *const (), 1), // +0x104ca0 sub_104ca0
    Lifted::new(0x70007f0cb0, tpl_36::<0x8> as *const (), 1), // +0x104cb0 sub_104cb0
    Lifted::new(0x70007f0cc0, tpl_36::<0x10> as *const (), 1), // +0x104cc0 sub_104cc0
    Lifted::new(0x70007f0cd0, tpl_36::<0x68> as *const (), 1), // +0x104cd0 sub_104cd0
    Lifted::new(0x70007f0ce0, tpl_36::<0x20> as *const (), 1), // +0x104ce0 sub_104ce0
    Lifted::new(0x70007f0cf0, fn_70007f0cf0 as *const (), 1), // +0x104cf0 sub_104cf0
    Lifted::new(0x70007f0d10, tpl_53 as *const (), 1), // +0x104d10 sub_104d10
    Lifted::new(0x70007f0d20, fn_70007f0d20 as *const (), 1), // +0x104d20 sub_104d20
    Lifted::new(0x70007f0d40, fn_70007f0d40 as *const (), 1), // +0x104d40 sub_104d40
    Lifted::new(0x70007f0d60, tpl_15 as *const (), 1), // +0x104d60 sub_104d60
    Lifted::new(0x70007f0d70, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x104d70 sub_104d70
    Lifted::new(0x70007f0dc0, tpl_33::<0x700177c312> as *const (), 0), // +0x104dc0 sub_104dc0
    Lifted::new(0x70007f0df0, fn_70007f0df0 as *const (), 0), // +0x104df0 sub_104df0
    Lifted::new(0x70007f0e20, fn_70007f0e20 as *const (), 0), // +0x104e20 sub_104e20
    Lifted::new(0x7000988480, tpl_20 as *const (), 1), // +0x29c480 sub_29c480
    Lifted::new(0x7000988490, tpl_20 as *const (), 1), // +0x29c490 sub_29c490
    Lifted::new(0x70009884a0, tpl_20 as *const (), 1), // +0x29c4a0 sub_29c4a0
    Lifted::new(0x70009884b0, tpl_20 as *const (), 1), // +0x29c4b0 sub_29c4b0
    Lifted::new(0x70009884c0, fn_70009884c0 as *const (), 0), // +0x29c4c0 sub_29c4c0
    Lifted::new(0x7000988520, fn_7000988520 as *const (), 2), // +0x29c520 sub_29c520
    Lifted::new(0x70009889e0, fn_70009889e0 as *const (), 2), // +0x29c9e0 sub_29c9e0
    Lifted::new(0x7000988c90, fn_7000988c90 as *const (), 0), // +0x29cc90 sub_29cc90
    Lifted::new(0x7000988ed0, fn_7000988ed0 as *const (), 0), // +0x29ced0 sub_29ced0
    Lifted::new(0x7000989260, tpl_54::<0x8, 0x70009894d0, 0x19, 0x10> as *const (), 0), // +0x29d260 sub_29d260
    Lifted::new(0x7000989350, tpl_55::<0x700177c0d0> as *const (), 0), // +0x29d350 sub_29d350
    Lifted::new(0x7000989390, tpl_55::<0x700177c0d0> as *const (), 0), // +0x29d390 sub_29d390
    Lifted::new(0x70009893d0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x29d3d0 sub_29d3d0
    Lifted::new(0x7000989410, tpl_55::<0x700177c0d0> as *const (), 0), // +0x29d410 sub_29d410
    Lifted::new(0x7000989450, tpl_55::<0x700177c0d0> as *const (), 0), // +0x29d450 sub_29d450
    Lifted::new(0x7000989490, tpl_55::<0x700177c0d0> as *const (), 0), // +0x29d490 sub_29d490
    Lifted::new(0x70009894d0, tpl_56::<0x19, 0x7001779e20, 0x20, 0x10, 0x18, 0x8, 0x70009894d0> as *const (), 0), // +0x29d4d0 sub_29d4d0
    Lifted::new(0x70009895c0, tpl_57::<0x8, 0x70006f32a0> as *const (), 0), // +0x29d5c0 sub_29d5c0
    Lifted::new(0x7000989650, tpl_57::<0x8, 0x70006f32a0> as *const (), 0), // +0x29d650 sub_29d650
    Lifted::new(0x70009896e0, tpl_58::<0x18, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x700177c312, 0xffffffffffffffe8, 0x7001d54968, 0x7000989767> as *const (), 0), // +0x29d6e0 sub_29d6e0
    Lifted::new(0x7000989790, fn_7000989790 as *const (), 0), // +0x29d790 sub_29d790
    Lifted::new(0x7000989840, tpl_59::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x70009899b1, 0x70009a4dd0, 0x70009899b7, 0x70007f0910, 0x70009899bd> as *const (), 0), // +0x29d840 sub_29d840
    Lifted::new(0x7000989a20, tpl_60::<0x70009a5480, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000989b62, 0x70006fcc10, 0x7000989b68> as *const (), 2), // +0x29da20 sub_29da20
    Lifted::new(0x7000989bc0, fn_7000989bc0 as *const (), 0), // +0x29dbc0 sub_29dbc0
    Lifted::new(0x7000989d80, tpl_60::<0x70009a5480, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x7000989ec2, 0x70006fcc10, 0x7000989ec8> as *const (), 2), // +0x29dd80 sub_29dd80
    Lifted::new(0x7000989f20, fn_7000989f20 as *const (), 0), // +0x29df20 sub_29df20
    Lifted::new(0x700098a100, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x700098a2d4, 0x70009a4e50, 0x700098a2da, 0x70007f0910, 0x700098a2e0> as *const (), 0), // +0x29e100 sub_29e100
    Lifted::new(0x700098a360, tpl_62::<0x8, 0x19, 0x10, 0x700098a460, 0x700177c312> as *const (), 0), // +0x29e360 sub_29e360
    Lifted::new(0x700098a3f0, tpl_63::<0x19, 0x10, 0x700098a3f0, 0x700177c312> as *const (), 0), // +0x29e3f0 sub_29e3f0
    Lifted::new(0x700098a460, tpl_64::<0x19, 0x10, 0x700098a460, 0x700177c312> as *const (), 0), // +0x29e460 sub_29e460
    Lifted::new(0x700098a4d0, tpl_64::<0x19, 0x10, 0x700098a4d0, 0x700177c312> as *const (), 0), // +0x29e4d0 sub_29e4d0
    Lifted::new(0x700098a540, fn_700098a540 as *const (), 3), // +0x29e540 sub_29e540
    Lifted::new(0x700098ab00, fn_700098ab00 as *const (), 0), // +0x29eb00 sub_29eb00
    Lifted::new(0x700098ad70, fn_700098ad70 as *const (), 0), // +0x29ed70 sub_29ed70
    Lifted::new(0x700098afb0, fn_700098afb0 as *const (), 0), // +0x29efb0 sub_29efb0
    Lifted::new(0x700098b050, tpl_57::<0x8, 0x700073db00> as *const (), 0), // +0x29f050 sub_29f050
    Lifted::new(0x700098b0e0, fn_700098b0e0 as *const (), 2), // +0x29f0e0 sub_29f0e0
    Lifted::new(0x700098b2d0, fn_700098b2d0 as *const (), 0), // +0x29f2d0 sub_29f2d0
    Lifted::new(0x700098b500, fn_700098b500 as *const (), 0), // +0x29f500 sub_29f500
    Lifted::new(0x700098b6c0, fn_700098b6c0 as *const (), 2), // +0x29f6c0 sub_29f6c0
    Lifted::new(0x700098b870, tpl_65::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x70009a34d0, 0x70017d37c0, 0x70017790e0, 0x700098bb5a> as *const (), 2), // +0x29f870 sub_29f870
    Lifted::new(0x700098bc20, fn_700098bc20 as *const (), 0), // +0x29fc20 sub_29fc20
    Lifted::new(0x700098bcd0, fn_700098bcd0 as *const (), 0), // +0x29fcd0 sub_29fcd0
    Lifted::new(0x700098bd80, fn_700098bd80 as *const (), 0), // +0x29fd80 sub_29fd80
    Lifted::new(0x700098be30, tpl_66::<0x7001779100, 0x8, 0x7001779110> as *const (), 0), // +0x29fe30 sub_29fe30
    Lifted::new(0x700098bee0, fn_700098bee0 as *const (), 0), // +0x29fee0 sub_29fee0
    Lifted::new(0x700098bf30, tpl_66::<0x7001779100, 0x8, 0x7001779110> as *const (), 0), // +0x29ff30 sub_29ff30
    Lifted::new(0x700098bfe0, fn_700098bfe0 as *const (), 0), // +0x29ffe0 sub_29ffe0
    Lifted::new(0x700098c030, fn_700098c030 as *const (), 0), // +0x2a0030 sub_2a0030
    Lifted::new(0x700098c090, fn_700098c090 as *const (), 2), // +0x2a0090 sub_2a0090
    Lifted::new(0x700098c380, fn_700098c380 as *const (), 0), // +0x2a0380 sub_2a0380
    Lifted::new(0x700098c550, tpl_67::<0x70009a53f0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x700098c693, 0x70009a4df0, 0x700098c699> as *const (), 0), // +0x2a0550 sub_2a0550
    Lifted::new(0x700098c6f0, tpl_67::<0x70009a53f0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x700098c833, 0x70009a4df0, 0x700098c839> as *const (), 0), // +0x2a06f0 sub_2a06f0
    Lifted::new(0x700098c890, fn_700098c890 as *const (), 0), // +0x2a0890 sub_2a0890
    Lifted::new(0x700098c920, tpl_68 as *const (), 1), // +0x2a0920 sub_2a0920
    Lifted::new(0x700098c9d0, tpl_68 as *const (), 1), // +0x2a09d0 sub_2a09d0
    Lifted::new(0x700098ca80, tpl_68 as *const (), 1), // +0x2a0a80 sub_2a0a80
    Lifted::new(0x700098cb30, tpl_68 as *const (), 1), // +0x2a0b30 sub_2a0b30
    Lifted::new(0x700098cbe0, fn_700098cbe0 as *const (), 2), // +0x2a0be0 sub_2a0be0
    Lifted::new(0x700098ced0, tpl_69::<0x7001779e20, 0x70009a31b0, 0x70009a3860, 0x70017d37c0, 0x70017790e0, 0x700098d14e> as *const (), 2), // +0x2a0ed0 sub_2a0ed0
    Lifted::new(0x700098d330, tpl_14 as *const (), 1), // +0x2a1330 sub_2a1330
    Lifted::new(0x700098d340, tpl_70 as *const (), 1), // +0x2a1340 sub_2a1340
    Lifted::new(0x700098d350, tpl_14 as *const (), 1), // +0x2a1350 sub_2a1350
    Lifted::new(0x700098d360, tpl_70 as *const (), 1), // +0x2a1360 sub_2a1360
    Lifted::new(0x700098d370, tpl_14 as *const (), 1), // +0x2a1370 sub_2a1370
    Lifted::new(0x700098d380, tpl_70 as *const (), 1), // +0x2a1380 sub_2a1380
    Lifted::new(0x700098d390, tpl_14 as *const (), 1), // +0x2a1390 sub_2a1390
    Lifted::new(0x700098d3a0, tpl_70 as *const (), 1), // +0x2a13a0 sub_2a13a0
    Lifted::new(0x700098d3b0, tpl_14 as *const (), 1), // +0x2a13b0 sub_2a13b0
    Lifted::new(0x700098d3c0, tpl_70 as *const (), 1), // +0x2a13c0 sub_2a13c0
    Lifted::new(0x700098d3d0, tpl_14 as *const (), 1), // +0x2a13d0 sub_2a13d0
    Lifted::new(0x700098d3e0, tpl_70 as *const (), 1), // +0x2a13e0 sub_2a13e0
    Lifted::new(0x700098d3f0, tpl_14 as *const (), 1), // +0x2a13f0 sub_2a13f0
    Lifted::new(0x700098d400, tpl_70 as *const (), 1), // +0x2a1400 sub_2a1400
    Lifted::new(0x700098d410, tpl_14 as *const (), 1), // +0x2a1410 sub_2a1410
    Lifted::new(0x700098d420, tpl_70 as *const (), 1), // +0x2a1420 sub_2a1420
    Lifted::new(0x700098d430, tpl_14 as *const (), 1), // +0x2a1430 sub_2a1430
    Lifted::new(0x700098d440, tpl_70 as *const (), 1), // +0x2a1440 sub_2a1440
    Lifted::new(0x700098d450, tpl_14 as *const (), 1), // +0x2a1450 sub_2a1450
    Lifted::new(0x700098d460, tpl_70 as *const (), 1), // +0x2a1460 sub_2a1460
    Lifted::new(0x700098d470, tpl_14 as *const (), 1), // +0x2a1470 sub_2a1470
    Lifted::new(0x700098d480, tpl_70 as *const (), 1), // +0x2a1480 sub_2a1480
    Lifted::new(0x700098d490, tpl_14 as *const (), 1), // +0x2a1490 sub_2a1490
    Lifted::new(0x700098d4a0, tpl_70 as *const (), 1), // +0x2a14a0 sub_2a14a0
    Lifted::new(0x700098d4b0, tpl_14 as *const (), 1), // +0x2a14b0 sub_2a14b0
    Lifted::new(0x700098d4c0, tpl_70 as *const (), 1), // +0x2a14c0 sub_2a14c0
    Lifted::new(0x700098d4d0, tpl_14 as *const (), 1), // +0x2a14d0 sub_2a14d0
    Lifted::new(0x700098d4e0, fn_700098d4e0 as *const (), 1), // +0x2a14e0 sub_2a14e0
    Lifted::new(0x700098d500, tpl_71::<0x10, 0x8> as *const (), 1), // +0x2a1500 sub_2a1500
    Lifted::new(0x700098d540, tpl_72::<0x8, 0x10> as *const (), 1), // +0x2a1540 sub_2a1540
    Lifted::new(0x700098d580, tpl_73::<0x8, 0x10, 0x70009a5480, 0x700177c0d0> as *const (), 0), // +0x2a1580 sub_2a1580
    Lifted::new(0x700098d640, tpl_73::<0x8, 0x10, 0x70009a5480, 0x700177c0d0> as *const (), 0), // +0x2a1640 sub_2a1640
    Lifted::new(0x700098d700, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2a1700 sub_2a1700
    Lifted::new(0x700098d720, fn_700098d720 as *const (), 0), // +0x2a1720 sub_2a1720
    Lifted::new(0x700098d7f0, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2a17f0 sub_2a17f0
    Lifted::new(0x700098d810, fn_700098d810 as *const (), 2), // +0x2a1810 sub_2a1810
    Lifted::new(0x700098d8c0, fn_700098d8c0 as *const (), 0), // +0x2a18c0 sub_2a18c0
    Lifted::new(0x700098d970, tpl_71::<0x10, 0x8> as *const (), 1), // +0x2a1970 sub_2a1970
    Lifted::new(0x700098d9b0, tpl_72::<0x8, 0x10> as *const (), 1), // +0x2a19b0 sub_2a19b0
    Lifted::new(0x700098d9f0, tpl_75::<0x8, 0x10, 0x70009a5510, 0x700098bcd0> as *const (), 0), // +0x2a19f0 sub_2a19f0
    Lifted::new(0x700098daa0, tpl_75::<0x8, 0x10, 0x70009a5510, 0x700098bcd0> as *const (), 0), // +0x2a1aa0 sub_2a1aa0
    Lifted::new(0x700098db50, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2a1b50 sub_2a1b50
    Lifted::new(0x700098db70, fn_700098db70 as *const (), 0), // +0x2a1b70 sub_2a1b70
    Lifted::new(0x700098dc60, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2a1c60 sub_2a1c60
    Lifted::new(0x700098dc80, fn_700098dc80 as *const (), 0), // +0x2a1c80 sub_2a1c80
    Lifted::new(0x700098dd50, fn_700098dd50 as *const (), 2), // +0x2a1d50 sub_2a1d50
    Lifted::new(0x700098de10, fn_700098de10 as *const (), 0), // +0x2a1e10 sub_2a1e10
    Lifted::new(0x700098ded0, tpl_7::<0x70017d2f90> as *const (), 1), // +0x2a1ed0 sub_2a1ed0
    Lifted::new(0x700098def0, tpl_7::<0x70017d2f90> as *const (), 1), // +0x2a1ef0 sub_2a1ef0
    Lifted::new(0x700098df10, tpl_7::<0x70017d2fa8> as *const (), 1), // +0x2a1f10 sub_2a1f10
    Lifted::new(0x700098df30, tpl_7::<0x70017d2fa8> as *const (), 1), // +0x2a1f30 sub_2a1f30
    Lifted::new(0x700098df50, fn_700098df50 as *const (), 1), // +0x2a1f50 sub_2a1f50
    Lifted::new(0x700098dff0, fn_700098dff0 as *const (), 1), // +0x2a1ff0 sub_2a1ff0
    Lifted::new(0x700098e020, tpl_76::<0x8, 0x10> as *const (), 1), // +0x2a2020 sub_2a2020
    Lifted::new(0x700098e050, tpl_77::<0x8, 0x10> as *const (), 1), // +0x2a2050 sub_2a2050
    Lifted::new(0x700098e090, tpl_78::<0x8, 0x10, 0x7001779e20, 0x18, 0x20, 0x28, 0x70009a2260, 0x30, 0x38> as *const (), 0), // +0x2a2090 sub_2a2090
    Lifted::new(0x700098e1b0, tpl_79::<0x8, 0x10, 0x18, 0x7001779e20, 0x30, 0x38, 0x20, 0x70009a2260, 0x700098cbe0> as *const (), 0), // +0x2a21b0 sub_2a21b0
    Lifted::new(0x700098e270, tpl_80::<0x8, 0x10, 0x18, 0x7001779e20, 0x30, 0x38, 0x70009a2260> as *const (), 0), // +0x2a2270 sub_2a2270
    Lifted::new(0x700098e300, fn_700098e300 as *const (), 0), // +0x2a2300 sub_2a2300
    Lifted::new(0x700098e330, fn_700098e330 as *const (), 1), // +0x2a2330 sub_2a2330
    Lifted::new(0x700098e350, tpl_6::<0x8> as *const (), 1), // +0x2a2350 sub_2a2350
    Lifted::new(0x700098e360, fn_700098e360 as *const (), 0), // +0x2a2360 sub_2a2360
    Lifted::new(0x700098e3b0, fn_700098e3b0 as *const (), 1), // +0x2a23b0 sub_2a23b0
    Lifted::new(0x700098e3f0, tpl_81::<0x700098de10, 0x18> as *const (), 0), // +0x2a23f0 sub_2a23f0
    Lifted::new(0x700098e430, fn_700098e430 as *const (), 1), // +0x2a2430 sub_2a2430
    Lifted::new(0x700098e450, fn_700098e450 as *const (), 0), // +0x2a2450 sub_2a2450
    Lifted::new(0x700098e4f0, fn_700098e4f0 as *const (), 1), // +0x2a24f0 sub_2a24f0
    Lifted::new(0x700098e520, fn_700098e520 as *const (), 0), // +0x2a2520 sub_2a2520
    Lifted::new(0x700098e580, tpl_82::<0x8> as *const (), 1), // +0x2a2580 sub_2a2580
    Lifted::new(0x700098e5b0, tpl_82::<0x8> as *const (), 1), // +0x2a25b0 sub_2a25b0
    Lifted::new(0x700098e5e0, fn_700098e5e0 as *const (), 1), // +0x2a25e0 sub_2a25e0
    Lifted::new(0x700098e620, tpl_83::<0x700074f8d0> as *const (), 0), // +0x2a2620 sub_2a2620
    Lifted::new(0x700098e650, fn_700098e650 as *const (), 1), // +0x2a2650 sub_2a2650
    Lifted::new(0x700098e670, tpl_84::<0x4, 0x8, 0x10, 0x700070d1a0> as *const (), 0), // +0x2a2670 sub_2a2670
    Lifted::new(0x700098e6c0, tpl_84::<0x4, 0x8, 0x10, 0x700070d1a0> as *const (), 0), // +0x2a26c0 sub_2a26c0
    Lifted::new(0x700098e710, fn_700098e710 as *const (), 2), // +0x2a2710 sub_2a2710
    Lifted::new(0x700098e7d0, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2a27d0 sub_2a27d0
    Lifted::new(0x700098e7f0, tpl_7::<0x70017d2f48> as *const (), 1), // +0x2a27f0 sub_2a27f0
    Lifted::new(0x700098e810, tpl_7::<0x70017d2f48> as *const (), 1), // +0x2a2810 sub_2a2810
    Lifted::new(0x700098e830, tpl_85::<0x8, 0x30, 0x7001779e20, 0x10, 0x18> as *const (), 0), // +0x2a2830 sub_2a2830
    Lifted::new(0x700098e8b0, fn_700098e8b0 as *const (), 2), // +0x2a28b0 sub_2a28b0
    Lifted::new(0x700098ea00, tpl_86::<0x8, 0x30, 0x7001779e20, 0x10, 0x18> as *const (), 0), // +0x2a2a00 sub_2a2a00
    Lifted::new(0x700098ea50, tpl_7::<0x70017d2fc8> as *const (), 1), // +0x2a2a50 sub_2a2a50
    Lifted::new(0x700098ea70, tpl_7::<0x70017d2fc8> as *const (), 1), // +0x2a2a70 sub_2a2a70
    Lifted::new(0x700098ea90, tpl_76::<0x8, 0x10> as *const (), 1), // +0x2a2a90 sub_2a2a90
    Lifted::new(0x700098eac0, tpl_77::<0x8, 0x10> as *const (), 1), // +0x2a2ac0 sub_2a2ac0
    Lifted::new(0x700098eb00, tpl_14 as *const (), 1), // +0x2a2b00 sub_2a2b00
    Lifted::new(0x700098eb10, tpl_70 as *const (), 1), // +0x2a2b10 sub_2a2b10
    Lifted::new(0x700098eb20, fn_700098eb20 as *const (), 0), // +0x2a2b20 sub_2a2b20
    Lifted::new(0x700098eb70, fn_700098eb70 as *const (), 1), // +0x2a2b70 sub_2a2b70
    Lifted::new(0x700098ebc0, fn_700098ebc0 as *const (), 0), // +0x2a2bc0 sub_2a2bc0
    Lifted::new(0x700098ecc0, fn_700098ecc0 as *const (), 0), // +0x2a2cc0 sub_2a2cc0
    Lifted::new(0x700098ed90, tpl_87::<0x70017d2c68, 0x8, 0x10, 0x70017d2cb0, 0x18, 0x20, 0x24, 0x28> as *const (), 1), // +0x2a2d90 sub_2a2d90
    Lifted::new(0x700098edf0, tpl_87::<0x70017d2c68, 0x8, 0x10, 0x70017d2cb0, 0x18, 0x20, 0x24, 0x28> as *const (), 1), // +0x2a2df0 sub_2a2df0
    Lifted::new(0x700098ee50, tpl_88::<0x700078d460, 0x70017d3040> as *const (), 0), // +0x2a2e50 sub_2a2e50
    Lifted::new(0x700098ee80, tpl_88::<0x700078d460, 0x70017d3098> as *const (), 0), // +0x2a2e80 sub_2a2e80
    Lifted::new(0x700098eeb0, fn_700098eeb0 as *const (), 1), // +0x2a2eb0 sub_2a2eb0
    Lifted::new(0x700098ef30, tpl_76::<0x8, 0x10> as *const (), 1), // +0x2a2f30 sub_2a2f30
    Lifted::new(0x700098ef60, tpl_77::<0x8, 0x10> as *const (), 1), // +0x2a2f60 sub_2a2f60
    Lifted::new(0x700098efa0, tpl_14 as *const (), 1), // +0x2a2fa0 sub_2a2fa0
    Lifted::new(0x700098efb0, tpl_70 as *const (), 1), // +0x2a2fb0 sub_2a2fb0
    Lifted::new(0x700098efc0, tpl_14 as *const (), 1), // +0x2a2fc0 sub_2a2fc0
    Lifted::new(0x700098efd0, tpl_70 as *const (), 1), // +0x2a2fd0 sub_2a2fd0
    Lifted::new(0x700098efe0, tpl_13::<0x7000733240> as *const (), 0), // +0x2a2fe0 sub_2a2fe0
    Lifted::new(0x700098f000, fn_700098f000 as *const (), 1), // +0x2a3000 sub_2a3000
    Lifted::new(0x700098f040, fn_700098f040 as *const (), 1), // +0x2a3040 sub_2a3040
    Lifted::new(0x700098f060, fn_700098f060 as *const (), 0), // +0x2a3060 sub_2a3060
    Lifted::new(0x700098f160, fn_700098f160 as *const (), 0), // +0x2a3160 sub_2a3160
    Lifted::new(0x700098f210, fn_700098f210 as *const (), 0), // +0x2a3210 sub_2a3210
    Lifted::new(0x700098f2c0, fn_700098f2c0 as *const (), 0), // +0x2a32c0 sub_2a32c0
    Lifted::new(0x700098f370, fn_700098f370 as *const (), 0), // +0x2a3370 sub_2a3370
    Lifted::new(0x700098f430, fn_700098f430 as *const (), 0), // +0x2a3430 sub_2a3430
    Lifted::new(0x700098f460, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2a3460 sub_2a3460
    Lifted::new(0x700098f480, fn_700098f480 as *const (), 0), // +0x2a3480 sub_2a3480
    Lifted::new(0x700098f6f0, fn_700098f6f0 as *const (), 0), // +0x2a36f0 sub_2a36f0
    Lifted::new(0x700098f740, fn_700098f740 as *const (), 0), // +0x2a3740 sub_2a3740
    Lifted::new(0x700098f780, fn_700098f780 as *const (), 0), // +0x2a3780 sub_2a3780
    Lifted::new(0x700098f7c0, fn_700098f7c0 as *const (), 0), // +0x2a37c0 sub_2a37c0
    Lifted::new(0x700098f800, fn_700098f800 as *const (), 0), // +0x2a3800 sub_2a3800
    Lifted::new(0x700098f840, fn_700098f840 as *const (), 0), // +0x2a3840 sub_2a3840
    Lifted::new(0x700098f880, fn_700098f880 as *const (), 0), // +0x2a3880 sub_2a3880
    Lifted::new(0x700098f930, fn_700098f930 as *const (), 1), // +0x2a3930 sub_2a3930
    Lifted::new(0x700098f950, tpl_19 as *const (), 3), // +0x2a3950 sub_2a3950
    Lifted::new(0x700098f960, tpl_14 as *const (), 1), // +0x2a3960 sub_2a3960
    Lifted::new(0x700098f970, tpl_70 as *const (), 1), // +0x2a3970 sub_2a3970
    Lifted::new(0x700098f980, tpl_89::<0x8, 0x10, 0x7001779e20, 0x18, 0x20, 0x28, 0x70009a2540, 0x30, 0x38> as *const (), 0), // +0x2a3980 sub_2a3980
    Lifted::new(0x700098faa0, tpl_90::<0x8, 0x10, 0x20, 0x7001779e20, 0x18, 0x30, 0x38, 0x70009a2540, 0x700098ced0> as *const (), 0), // +0x2a3aa0 sub_2a3aa0
    Lifted::new(0x700098fb60, tpl_91::<0x8, 0x10, 0x20, 0x7001779e20, 0x18, 0x30, 0x38, 0x70009a2540> as *const (), 0), // +0x2a3b60 sub_2a3b60
    Lifted::new(0x700098fbf0, tpl_92::<0x8> as *const (), 1), // +0x2a3bf0 sub_2a3bf0
    Lifted::new(0x700098fc10, tpl_93::<0x8> as *const (), 1), // +0x2a3c10 sub_2a3c10
    Lifted::new(0x700098fc20, fn_700098fc20 as *const (), 0), // +0x2a3c20 sub_2a3c20
    Lifted::new(0x700098fc70, tpl_14 as *const (), 1), // +0x2a3c70 sub_2a3c70
    Lifted::new(0x700098fc80, tpl_70 as *const (), 1), // +0x2a3c80 sub_2a3c80
    Lifted::new(0x700098fc90, tpl_14 as *const (), 1), // +0x2a3c90 sub_2a3c90
    Lifted::new(0x700098fca0, tpl_70 as *const (), 1), // +0x2a3ca0 sub_2a3ca0
    Lifted::new(0x700098fcb0, tpl_14 as *const (), 1), // +0x2a3cb0 sub_2a3cb0
    Lifted::new(0x700098fcc0, tpl_70 as *const (), 1), // +0x2a3cc0 sub_2a3cc0
    Lifted::new(0x700098fcd0, tpl_7::<0x70017d3180> as *const (), 1), // +0x2a3cd0 sub_2a3cd0
    Lifted::new(0x700098fcf0, tpl_7::<0x70017d3180> as *const (), 1), // +0x2a3cf0 sub_2a3cf0
    Lifted::new(0x700098fd10, fn_700098fd10 as *const (), 0), // +0x2a3d10 sub_2a3d10
    Lifted::new(0x700098fd50, tpl_14 as *const (), 1), // +0x2a3d50 sub_2a3d50
    Lifted::new(0x700098fd60, tpl_70 as *const (), 1), // +0x2a3d60 sub_2a3d60
    Lifted::new(0x700098fd70, tpl_14 as *const (), 1), // +0x2a3d70 sub_2a3d70
    Lifted::new(0x700098fd80, tpl_70 as *const (), 1), // +0x2a3d80 sub_2a3d80
    Lifted::new(0x700098fd90, tpl_14 as *const (), 1), // +0x2a3d90 sub_2a3d90
    Lifted::new(0x700098fda0, tpl_70 as *const (), 1), // +0x2a3da0 sub_2a3da0
    Lifted::new(0x700098fdb0, tpl_7::<0x70017d2f60> as *const (), 1), // +0x2a3db0 sub_2a3db0
    Lifted::new(0x700098fdd0, tpl_7::<0x70017d2f60> as *const (), 1), // +0x2a3dd0 sub_2a3dd0
    Lifted::new(0x700098fdf0, fn_700098fdf0 as *const (), 1), // +0x2a3df0 sub_2a3df0
    Lifted::new(0x700098fe10, tpl_94::<0x70017d32f8, 0x8, 0x10, 0x18, 0x20, 0x30> as *const (), 3), // +0x2a3e10 sub_2a3e10
    Lifted::new(0x700098fe60, tpl_94::<0x70017d32f8, 0x8, 0x10, 0x18, 0x20, 0x30> as *const (), 3), // +0x2a3e60 sub_2a3e60
    Lifted::new(0x700098feb0, fn_700098feb0 as *const (), 1), // +0x2a3eb0 sub_2a3eb0
    Lifted::new(0x700098ff40, tpl_7::<0x70017d2e38> as *const (), 1), // +0x2a3f40 sub_2a3f40
    Lifted::new(0x700098ff60, tpl_7::<0x70017d2e38> as *const (), 1), // +0x2a3f60 sub_2a3f60
    Lifted::new(0x700098ff80, tpl_7::<0x70017d2b90> as *const (), 1), // +0x2a3f80 sub_2a3f80
    Lifted::new(0x700098ffa0, tpl_7::<0x70017d2b90> as *const (), 1), // +0x2a3fa0 sub_2a3fa0
    Lifted::new(0x700098ffc0, fn_700098ffc0 as *const (), 1), // +0x2a3fc0 sub_2a3fc0
    Lifted::new(0x700098ffd0, tpl_7::<0x70017d2d50> as *const (), 1), // +0x2a3fd0 sub_2a3fd0
    Lifted::new(0x700098fff0, tpl_77::<0x8, 0x10> as *const (), 1), // +0x2a3ff0 sub_2a3ff0
    Lifted::new(0x7000990030, tpl_77::<0x8, 0x10> as *const (), 1), // +0x2a4030 sub_2a4030
    Lifted::new(0x7000990070, fn_7000990070 as *const (), 1), // +0x2a4070 sub_2a4070
    Lifted::new(0x7000990080, fn_7000990080 as *const (), 0), // +0x2a4080 sub_2a4080
    Lifted::new(0x70009901a0, fn_70009901a0 as *const (), 0), // +0x2a41a0 sub_2a41a0
    Lifted::new(0x7000990230, fn_7000990230 as *const (), 1), // +0x2a4230 sub_2a4230
    Lifted::new(0x7000990250, fn_7000990250 as *const (), 1), // +0x2a4250 sub_2a4250
    Lifted::new(0x7000990290, fn_7000990290 as *const (), 1), // +0x2a4290 sub_2a4290
    Lifted::new(0x70009902c0, fn_70009902c0 as *const (), 3), // +0x2a42c0 sub_2a42c0
    Lifted::new(0x7000990320, fn_7000990320 as *const (), 2), // +0x2a4320 sub_2a4320
    Lifted::new(0x7000990440, fn_7000990440 as *const (), 2), // +0x2a4440 sub_2a4440
    Lifted::new(0x70009905f0, fn_70009905f0 as *const (), 1), // +0x2a45f0 sub_2a45f0
    Lifted::new(0x7000990620, fn_7000990620 as *const (), 1), // +0x2a4620 sub_2a4620
    Lifted::new(0x7000990660, tpl_7::<0x70017d2dc0> as *const (), 1), // +0x2a4660 sub_2a4660
    Lifted::new(0x7000990680, tpl_14 as *const (), 1), // +0x2a4680 sub_2a4680
    Lifted::new(0x7000990690, tpl_70 as *const (), 1), // +0x2a4690 sub_2a4690
    Lifted::new(0x70009906a0, fn_70009906a0 as *const (), 0), // +0x2a46a0 sub_2a46a0
    Lifted::new(0x70009907b0, fn_70009907b0 as *const (), 0), // +0x2a47b0 sub_2a47b0
    Lifted::new(0x70009908c0, tpl_82::<0x8> as *const (), 1), // +0x2a48c0 sub_2a48c0
    Lifted::new(0x70009908f0, tpl_82::<0x8> as *const (), 1), // +0x2a48f0 sub_2a48f0
    Lifted::new(0x7000990920, tpl_95::<0x70017d3020, 0x8, 0x10, 0x70017d3030, 0x18> as *const (), 1), // +0x2a4920 sub_2a4920
    Lifted::new(0x7000990960, tpl_95::<0x70017d3020, 0x8, 0x10, 0x70017d3030, 0x18> as *const (), 1), // +0x2a4960 sub_2a4960
    Lifted::new(0x70009909a0, fn_70009909a0 as *const (), 1), // +0x2a49a0 sub_2a49a0
    Lifted::new(0x70009909d0, tpl_96::<0x70017d3020, 0x8, 0x10> as *const (), 1), // +0x2a49d0 sub_2a49d0
    Lifted::new(0x7000990a00, fn_7000990a00 as *const (), 1), // +0x2a4a00 sub_2a4a00
    Lifted::new(0x7000990a20, fn_7000990a20 as *const (), 2), // +0x2a4a20 sub_2a4a20
    Lifted::new(0x7000990be0, fn_7000990be0 as *const (), 1), // +0x2a4be0 sub_2a4be0
    Lifted::new(0x7000990bf0, tpl_14 as *const (), 1), // +0x2a4bf0 sub_2a4bf0
    Lifted::new(0x7000990c00, tpl_70 as *const (), 1), // +0x2a4c00 sub_2a4c00
    Lifted::new(0x7000990c10, tpl_14 as *const (), 1), // +0x2a4c10 sub_2a4c10
    Lifted::new(0x7000990c20, tpl_70 as *const (), 1), // +0x2a4c20 sub_2a4c20
    Lifted::new(0x7000990c30, fn_7000990c30 as *const (), 1), // +0x2a4c30 sub_2a4c30
    Lifted::new(0x7000990d10, fn_7000990d10 as *const (), 0), // +0x2a4d10 sub_2a4d10
    Lifted::new(0x7000990f40, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000990fe3> as *const (), 0), // +0x2a4f40 sub_2a4f40
    Lifted::new(0x7000991010, tpl_98::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x70009910b3> as *const (), 0), // +0x2a5010 sub_2a5010
    Lifted::new(0x70009910e0, tpl_99::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000991183> as *const (), 0), // +0x2a50e0 sub_2a50e0
    Lifted::new(0x70009911b0, tpl_97::<0x18, 0x20, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x7001d54968, 0x7000991253> as *const (), 0), // +0x2a51b0 sub_2a51b0
    Lifted::new(0x7000991280, fn_7000991280 as *const (), 0), // +0x2a5280 sub_2a5280
    Lifted::new(0x7000991360, tpl_100::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x70009913da> as *const (), 0), // +0x2a5360 sub_2a5360
    Lifted::new(0x7000991400, tpl_101::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x700099145b> as *const (), 0), // +0x2a5400 sub_2a5400
    Lifted::new(0x7000991490, tpl_15 as *const (), 1), // +0x2a5490 sub_2a5490
    Lifted::new(0x70009914a0, fn_70009914a0 as *const (), 0), // +0x2a54a0 sub_2a54a0
    Lifted::new(0x70009914d0, tpl_15 as *const (), 1), // +0x2a54d0 sub_2a54d0
    Lifted::new(0x70009914e0, fn_70009914e0 as *const (), 0), // +0x2a54e0 sub_2a54e0
    Lifted::new(0x70009915a0, tpl_15 as *const (), 1), // +0x2a55a0 sub_2a55a0
    Lifted::new(0x70009915b0, tpl_102::<0x10, 0x8, 0xc> as *const (), 0), // +0x2a55b0 sub_2a55b0
    Lifted::new(0x7000991630, tpl_103::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x700099168b> as *const (), 0), // +0x2a5630 sub_2a5630
    Lifted::new(0x70009916b0, fn_70009916b0 as *const (), 0), // +0x2a56b0 sub_2a56b0
    Lifted::new(0x7000991780, tpl_15 as *const (), 1), // +0x2a5780 sub_2a5780
    Lifted::new(0x7000991790, fn_7000991790 as *const (), 0), // +0x2a5790 sub_2a5790
    Lifted::new(0x70009918f0, tpl_104::<0x8, 0xc> as *const (), 0), // +0x2a58f0 sub_2a58f0
    Lifted::new(0x7000991950, tpl_105::<0x8, 0x18, 0xfffffffffffffff8, 0x700177c312, 0x10, 0x7001d54968, 0x70009919ad> as *const (), 0), // +0x2a5950 sub_2a5950
    Lifted::new(0x70009919d0, tpl_106::<0x10, 0x7000751790> as *const (), 0), // +0x2a59d0 sub_2a59d0
    Lifted::new(0x7000991a00, tpl_15 as *const (), 1), // +0x2a5a00 sub_2a5a00
    Lifted::new(0x7000991a10, fn_7000991a10 as *const (), 0), // +0x2a5a10 sub_2a5a10
    Lifted::new(0x7000991a90, tpl_15 as *const (), 1), // +0x2a5a90 sub_2a5a90
    Lifted::new(0x7000991aa0, tpl_102::<0x10, 0x8, 0xc> as *const (), 0), // +0x2a5aa0 sub_2a5aa0
    Lifted::new(0x7000991b00, fn_7000991b00 as *const (), 0), // +0x2a5b00 sub_2a5b00
    Lifted::new(0x7000991b10, tpl_107::<0x8, 0x10, 0x18> as *const (), 1), // +0x2a5b10 sub_2a5b10
    Lifted::new(0x7000991b30, fn_7000991b30 as *const (), 0), // +0x2a5b30 sub_2a5b30
    Lifted::new(0x7000991b80, tpl_102::<0x10, 0x8, 0xc> as *const (), 0), // +0x2a5b80 sub_2a5b80
    Lifted::new(0x7000991be0, fn_7000991be0 as *const (), 0), // +0x2a5be0 sub_2a5be0
    Lifted::new(0x7000991cf0, fn_7000991cf0 as *const (), 0), // +0x2a5cf0 sub_2a5cf0
    Lifted::new(0x7000991d00, fn_7000991d00 as *const (), 0), // +0x2a5d00 sub_2a5d00
    Lifted::new(0x7000991d10, tpl_108::<0x70017d3198, 0x7000780b90, 0x108, 0x7001d550c0, 0xf8, 0x7001d54ea0, 0x700070d940> as *const (), 0), // +0x2a5d10 sub_2a5d10
    Lifted::new(0x7000991d60, tpl_109::<0x7000991280, 0x7001d54ea0> as *const (), 0), // +0x2a5d60 sub_2a5d60
    Lifted::new(0x7000991da0, fn_7000991da0 as *const (), 0), // +0x2a5da0 sub_2a5da0
    Lifted::new(0x7000991ea0, fn_7000991ea0 as *const (), 0), // +0x2a5ea0 sub_2a5ea0
    Lifted::new(0x7000991ee0, tpl_15 as *const (), 1), // +0x2a5ee0 sub_2a5ee0
    Lifted::new(0x7000991ef0, fn_7000991ef0 as *const (), 0), // +0x2a5ef0 sub_2a5ef0
    Lifted::new(0x7000991f20, tpl_15 as *const (), 1), // +0x2a5f20 sub_2a5f20
    Lifted::new(0x7000991f30, tpl_108::<0x70017d3310, 0x70006f0c20, 0x150, 0x7001d550c0, 0x138, 0x7001d54ea0, 0x700070d940> as *const (), 0), // +0x2a5f30 sub_2a5f30
    Lifted::new(0x7000991f80, tpl_15 as *const (), 1), // +0x2a5f80 sub_2a5f80
    Lifted::new(0x7000991f90, tpl_15 as *const (), 1), // +0x2a5f90 sub_2a5f90
    Lifted::new(0x7000991fa0, tpl_15 as *const (), 1), // +0x2a5fa0 sub_2a5fa0
    Lifted::new(0x7000991fb0, fn_7000991fb0 as *const (), 0), // +0x2a5fb0 sub_2a5fb0
    Lifted::new(0x7000991ff0, tpl_15 as *const (), 1), // +0x2a5ff0 sub_2a5ff0
    Lifted::new(0x7000992000, tpl_102::<0x10, 0x8, 0xc> as *const (), 0), // +0x2a6000 sub_2a6000
    Lifted::new(0x7000992060, tpl_102::<0x10, 0x8, 0xc> as *const (), 0), // +0x2a6060 sub_2a6060
    Lifted::new(0x70009920e0, tpl_110::<0x7000991010> as *const (), 0), // +0x2a60e0 sub_2a60e0
    Lifted::new(0x70009920f0, fn_70009920f0 as *const (), 0), // +0x2a60f0 sub_2a60f0
    Lifted::new(0x7000992110, tpl_107::<0x10, 0x18, 0x20> as *const (), 1), // +0x2a6110 sub_2a6110
    Lifted::new(0x7000992130, fn_7000992130 as *const (), 0), // +0x2a6130 sub_2a6130
    Lifted::new(0x70009921b0, tpl_15 as *const (), 1), // +0x2a61b0 sub_2a61b0
    Lifted::new(0x70009921c0, fn_70009921c0 as *const (), 0), // +0x2a61c0 sub_2a61c0
    Lifted::new(0x70009921e0, fn_70009921e0 as *const (), 0), // +0x2a61e0 sub_2a61e0
    Lifted::new(0x70009922e0, tpl_104::<0x8, 0xc> as *const (), 0), // +0x2a62e0 sub_2a62e0
    Lifted::new(0x7000992350, fn_7000992350 as *const (), 0), // +0x2a6350 sub_2a6350
    Lifted::new(0x7000992400, fn_7000992400 as *const (), 0), // +0x2a6400 sub_2a6400
    Lifted::new(0x7000992540, tpl_111 as *const (), 1), // +0x2a6540 sub_2a6540
    Lifted::new(0x7000992550, tpl_112::<0x7001d55090, 0x700099255a> as *const (), 0), // +0x2a6550 sub_2a6550
    Lifted::new(0x7000992570, tpl_112::<0x7001d55090, 0x700099257a> as *const (), 0), // +0x2a6570 sub_2a6570
    Lifted::new(0x7000992580, tpl_28 as *const (), 1), // +0x2a6580 sub_2a6580
    Lifted::new(0x7000992590, tpl_28 as *const (), 1), // +0x2a6590 sub_2a6590
    Lifted::new(0x70009925a0, tpl_28 as *const (), 1), // +0x2a65a0 sub_2a65a0
    Lifted::new(0x70009925b0, tpl_28 as *const (), 1), // +0x2a65b0 sub_2a65b0
    Lifted::new(0x70009925c0, tpl_20 as *const (), 1), // +0x2a65c0 sub_2a65c0
    Lifted::new(0x70009925d0, tpl_20 as *const (), 1), // +0x2a65d0 sub_2a65d0
    Lifted::new(0x70009925e0, tpl_20 as *const (), 1), // +0x2a65e0 sub_2a65e0
    Lifted::new(0x70009925f0, tpl_20 as *const (), 1), // +0x2a65f0 sub_2a65f0
    Lifted::new(0x7000992600, tpl_20 as *const (), 1), // +0x2a6600 sub_2a6600
    Lifted::new(0x7000992610, tpl_20 as *const (), 1), // +0x2a6610 sub_2a6610
    Lifted::new(0x7000992620, tpl_28 as *const (), 1), // +0x2a6620 sub_2a6620
    Lifted::new(0x7000992630, tpl_28 as *const (), 1), // +0x2a6630 sub_2a6630
    Lifted::new(0x7000992640, tpl_28 as *const (), 1), // +0x2a6640 sub_2a6640
    Lifted::new(0x7000992650, tpl_28 as *const (), 1), // +0x2a6650 sub_2a6650
    Lifted::new(0x7000992660, tpl_20 as *const (), 1), // +0x2a6660 sub_2a6660
    Lifted::new(0x7000992670, tpl_20 as *const (), 1), // +0x2a6670 sub_2a6670
    Lifted::new(0x7000992680, tpl_20 as *const (), 1), // +0x2a6680 sub_2a6680
    Lifted::new(0x7000992690, tpl_20 as *const (), 1), // +0x2a6690 sub_2a6690
    Lifted::new(0x70009926a0, tpl_28 as *const (), 1), // +0x2a66a0 sub_2a66a0
    Lifted::new(0x70009926b0, tpl_28 as *const (), 1), // +0x2a66b0 sub_2a66b0
    Lifted::new(0x70009926c0, tpl_28 as *const (), 1), // +0x2a66c0 sub_2a66c0
    Lifted::new(0x70009926d0, tpl_28 as *const (), 1), // +0x2a66d0 sub_2a66d0
    Lifted::new(0x70009926e0, tpl_28 as *const (), 1), // +0x2a66e0 sub_2a66e0
    Lifted::new(0x70009926f0, tpl_28 as *const (), 1), // +0x2a66f0 sub_2a66f0
    Lifted::new(0x7000992700, tpl_28 as *const (), 1), // +0x2a6700 sub_2a6700
    Lifted::new(0x7000992710, tpl_28 as *const (), 1), // +0x2a6710 sub_2a6710
    Lifted::new(0x7000992720, tpl_28 as *const (), 1), // +0x2a6720 sub_2a6720
    Lifted::new(0x7000992730, tpl_28 as *const (), 1), // +0x2a6730 sub_2a6730
    Lifted::new(0x7000992740, tpl_28 as *const (), 1), // +0x2a6740 sub_2a6740
    Lifted::new(0x7000992750, tpl_28 as *const (), 1), // +0x2a6750 sub_2a6750
    Lifted::new(0x7000992760, tpl_28 as *const (), 1), // +0x2a6760 sub_2a6760
    Lifted::new(0x7000992770, tpl_28 as *const (), 1), // +0x2a6770 sub_2a6770
    Lifted::new(0x7000992780, tpl_28 as *const (), 1), // +0x2a6780 sub_2a6780
    Lifted::new(0x7000992790, tpl_28 as *const (), 1), // +0x2a6790 sub_2a6790
    Lifted::new(0x70009927a0, tpl_28 as *const (), 1), // +0x2a67a0 sub_2a67a0
    Lifted::new(0x70009927b0, tpl_28 as *const (), 1), // +0x2a67b0 sub_2a67b0
    Lifted::new(0x70009927c0, tpl_28 as *const (), 1), // +0x2a67c0 sub_2a67c0
    Lifted::new(0x70009927d0, tpl_28 as *const (), 1), // +0x2a67d0 sub_2a67d0
    Lifted::new(0x70009927e0, tpl_20 as *const (), 1), // +0x2a67e0 sub_2a67e0
    Lifted::new(0x70009927f0, tpl_20 as *const (), 1), // +0x2a67f0 sub_2a67f0
    Lifted::new(0x7000992800, tpl_20 as *const (), 1), // +0x2a6800 sub_2a6800
    Lifted::new(0x7000992810, tpl_20 as *const (), 1), // +0x2a6810 sub_2a6810
    Lifted::new(0x7000992820, tpl_20 as *const (), 1), // +0x2a6820 sub_2a6820
    Lifted::new(0x7000992830, tpl_20 as *const (), 1), // +0x2a6830 sub_2a6830
    Lifted::new(0x7000992840, tpl_20 as *const (), 1), // +0x2a6840 sub_2a6840
    Lifted::new(0x7000992850, tpl_20 as *const (), 1), // +0x2a6850 sub_2a6850
    Lifted::new(0x7000992860, tpl_20 as *const (), 1), // +0x2a6860 sub_2a6860
    Lifted::new(0x7000992870, tpl_20 as *const (), 1), // +0x2a6870 sub_2a6870
    Lifted::new(0x7000992880, tpl_113::<0x8, 0xc> as *const (), 0), // +0x2a6880 sub_2a6880
    Lifted::new(0x7000992920, tpl_113::<0x8, 0xc> as *const (), 0), // +0x2a6920 sub_2a6920
    Lifted::new(0x70009929c0, tpl_114::<0x700073db00> as *const (), 0), // +0x2a69c0 sub_2a69c0
    Lifted::new(0x70009929f0, tpl_114::<0x70006f32a0> as *const (), 0), // +0x2a69f0 sub_2a69f0
    Lifted::new(0x7000992a20, fn_7000992a20 as *const (), 0), // +0x2a6a20 sub_2a6a20
    Lifted::new(0x7000992ab0, fn_7000992ab0 as *const (), 0), // +0x2a6ab0 sub_2a6ab0
    Lifted::new(0x7000992b10, tpl_114::<0x700077b670> as *const (), 0), // +0x2a6b10 sub_2a6b10
    Lifted::new(0x7000992b40, tpl_115::<0x8, 0x7000988ed0> as *const (), 0), // +0x2a6b40 sub_2a6b40
    Lifted::new(0x7000992b70, tpl_116::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000992c0a> as *const (), 0), // +0x2a6b70 sub_2a6b70
    Lifted::new(0x7000992c40, fn_7000992c40 as *const (), 0), // +0x2a6c40 sub_2a6c40
    Lifted::new(0x7000992ce0, tpl_20 as *const (), 1), // +0x2a6ce0 sub_2a6ce0
    Lifted::new(0x7000992cf0, tpl_20 as *const (), 1), // +0x2a6cf0 sub_2a6cf0
    Lifted::new(0x7000992d00, tpl_20 as *const (), 1), // +0x2a6d00 sub_2a6d00
    Lifted::new(0x7000992d10, tpl_20 as *const (), 1), // +0x2a6d10 sub_2a6d10
    Lifted::new(0x7000992d20, tpl_20 as *const (), 1), // +0x2a6d20 sub_2a6d20
    Lifted::new(0x7000992d30, tpl_20 as *const (), 1), // +0x2a6d30 sub_2a6d30
    Lifted::new(0x7000992d40, tpl_83::<0x7000992880> as *const (), 0), // +0x2a6d40 sub_2a6d40
    Lifted::new(0x7000992d70, tpl_117::<0x70009a8c40, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const (), 0), // +0x2a6d70 sub_2a6d70
    Lifted::new(0x7000992e40, fn_7000992e40 as *const (), 0), // +0x2a6e40 sub_2a6e40
    Lifted::new(0x7000992ff0, fn_7000992ff0 as *const (), 3), // +0x2a6ff0 sub_2a6ff0
    Lifted::new(0x7000993010, tpl_19 as *const (), 3), // +0x2a7010 sub_2a7010
    Lifted::new(0x7000993020, fn_7000993020 as *const (), 3), // +0x2a7020 sub_2a7020
    Lifted::new(0x7000993050, tpl_25::<0x10> as *const (), 3), // +0x2a7050 sub_2a7050
    Lifted::new(0x7000993070, tpl_20 as *const (), 1), // +0x2a7070 sub_2a7070
    Lifted::new(0x7000993080, tpl_20 as *const (), 1), // +0x2a7080 sub_2a7080
    Lifted::new(0x7000993090, tpl_18::<0x8> as *const (), 1), // +0x2a7090 sub_2a7090
    Lifted::new(0x70009930b0, tpl_19 as *const (), 3), // +0x2a70b0 sub_2a70b0
    Lifted::new(0x70009930c0, tpl_20 as *const (), 1), // +0x2a70c0 sub_2a70c0
    Lifted::new(0x70009930d0, tpl_20 as *const (), 1), // +0x2a70d0 sub_2a70d0
    Lifted::new(0x70009930e0, tpl_20 as *const (), 1), // +0x2a70e0 sub_2a70e0
    Lifted::new(0x70009930f0, tpl_20 as *const (), 1), // +0x2a70f0 sub_2a70f0
    Lifted::new(0x7000993100, tpl_81::<0x7000992b70, 0x18> as *const (), 0), // +0x2a7100 sub_2a7100
    Lifted::new(0x7000993140, tpl_81::<0x7000992c40, 0x18> as *const (), 0), // +0x2a7140 sub_2a7140
    Lifted::new(0x7000993180, fn_7000993180 as *const (), 1), // +0x2a7180 sub_2a7180
    Lifted::new(0x70009931a0, tpl_20 as *const (), 1), // +0x2a71a0 sub_2a71a0
    Lifted::new(0x70009931b0, tpl_20 as *const (), 1), // +0x2a71b0 sub_2a71b0
    Lifted::new(0x70009931c0, tpl_20 as *const (), 1), // +0x2a71c0 sub_2a71c0
    Lifted::new(0x70009931d0, tpl_13::<0x7000992880> as *const (), 0), // +0x2a71d0 sub_2a71d0
    Lifted::new(0x70009931f0, tpl_20 as *const (), 1), // +0x2a71f0 sub_2a71f0
    Lifted::new(0x7000993200, tpl_20 as *const (), 1), // +0x2a7200 sub_2a7200
    Lifted::new(0x7000993210, tpl_118::<0x700073db00> as *const (), 0), // +0x2a7210 sub_2a7210
    Lifted::new(0x7000993250, tpl_118::<0x70006f32a0> as *const (), 0), // +0x2a7250 sub_2a7250
    Lifted::new(0x7000993290, tpl_20 as *const (), 1), // +0x2a7290 sub_2a7290
    Lifted::new(0x70009932a0, tpl_20 as *const (), 1), // +0x2a72a0 sub_2a72a0
    Lifted::new(0x70009932b0, tpl_119::<0x4, 0x8, 0x10, 0x7000751790, 0x700070d1a0> as *const (), 0), // +0x2a72b0 sub_2a72b0
    Lifted::new(0x7000993320, tpl_119::<0x4, 0x8, 0x10, 0x7000751790, 0x700070d1a0> as *const (), 0), // +0x2a7320 sub_2a7320
    Lifted::new(0x7000993390, fn_7000993390 as *const (), 0), // +0x2a7390 sub_2a7390
    Lifted::new(0x7000993400, fn_7000993400 as *const (), 3), // +0x2a7400 sub_2a7400
    Lifted::new(0x7000993420, tpl_25::<0x10> as *const (), 3), // +0x2a7420 sub_2a7420
    Lifted::new(0x7000993440, tpl_23::<0x8, 0x10> as *const (), 1), // +0x2a7440 sub_2a7440
    Lifted::new(0x7000993460, tpl_21::<0x10> as *const (), 3), // +0x2a7460 sub_2a7460
    Lifted::new(0x7000993480, tpl_20 as *const (), 1), // +0x2a7480 sub_2a7480
    Lifted::new(0x7000993490, fn_7000993490 as *const (), 0), // +0x2a7490 sub_2a7490
    Lifted::new(0x7000993560, tpl_20 as *const (), 1), // +0x2a7560 sub_2a7560
    Lifted::new(0x7000993570, tpl_20 as *const (), 1), // +0x2a7570 sub_2a7570
    Lifted::new(0x7000993580, tpl_20 as *const (), 1), // +0x2a7580 sub_2a7580
    Lifted::new(0x7000993590, tpl_20 as *const (), 1), // +0x2a7590 sub_2a7590
    Lifted::new(0x70009935a0, tpl_20 as *const (), 1), // +0x2a75a0 sub_2a75a0
    Lifted::new(0x70009935b0, tpl_83::<0x7000992920> as *const (), 0), // +0x2a75b0 sub_2a75b0
    Lifted::new(0x70009935e0, tpl_28 as *const (), 1), // +0x2a75e0 sub_2a75e0
    Lifted::new(0x70009935f0, tpl_28 as *const (), 1), // +0x2a75f0 sub_2a75f0
    Lifted::new(0x7000993600, tpl_21::<0x10> as *const (), 3), // +0x2a7600 sub_2a7600
    Lifted::new(0x7000993620, tpl_20 as *const (), 1), // +0x2a7620 sub_2a7620
    Lifted::new(0x7000993630, tpl_20 as *const (), 1), // +0x2a7630 sub_2a7630
    Lifted::new(0x7000993640, fn_7000993640 as *const (), 1), // +0x2a7640 sub_2a7640
    Lifted::new(0x7000993680, tpl_26::<0x10, 0x20> as *const (), 3), // +0x2a7680 sub_2a7680
    Lifted::new(0x70009936b0, fn_70009936b0 as *const (), 0), // +0x2a76b0 sub_2a76b0
    Lifted::new(0x7000993820, fn_7000993820 as *const (), 0), // +0x2a7820 sub_2a7820
    Lifted::new(0x7000993940, tpl_20 as *const (), 1), // +0x2a7940 sub_2a7940
    Lifted::new(0x7000993950, tpl_20 as *const (), 1), // +0x2a7950 sub_2a7950
    Lifted::new(0x7000993960, tpl_120::<0x8, 0x10, 0x18, 0x20, 0x24, 0x28> as *const (), 1), // +0x2a7960 sub_2a7960
    Lifted::new(0x70009939a0, tpl_120::<0x8, 0x10, 0x18, 0x20, 0x24, 0x28> as *const (), 1), // +0x2a79a0 sub_2a79a0
    Lifted::new(0x70009939e0, tpl_83::<0x7000992880> as *const (), 0), // +0x2a79e0 sub_2a79e0
    Lifted::new(0x7000993a10, tpl_28 as *const (), 1), // +0x2a7a10 sub_2a7a10
    Lifted::new(0x7000993a20, tpl_28 as *const (), 1), // +0x2a7a20 sub_2a7a20
    Lifted::new(0x7000993a30, tpl_20 as *const (), 1), // +0x2a7a30 sub_2a7a30
    Lifted::new(0x7000993a40, tpl_20 as *const (), 1), // +0x2a7a40 sub_2a7a40
    Lifted::new(0x7000993a50, tpl_28 as *const (), 1), // +0x2a7a50 sub_2a7a50
    Lifted::new(0x7000993a60, tpl_28 as *const (), 1), // +0x2a7a60 sub_2a7a60
    Lifted::new(0x7000993a70, tpl_20 as *const (), 1), // +0x2a7a70 sub_2a7a70
    Lifted::new(0x7000993a80, tpl_20 as *const (), 1), // +0x2a7a80 sub_2a7a80
    Lifted::new(0x7000993a90, tpl_20 as *const (), 1), // +0x2a7a90 sub_2a7a90
    Lifted::new(0x7000993aa0, tpl_20 as *const (), 1), // +0x2a7aa0 sub_2a7aa0
    Lifted::new(0x7000993ab0, tpl_20 as *const (), 1), // +0x2a7ab0 sub_2a7ab0
    Lifted::new(0x7000993ac0, tpl_20 as *const (), 1), // +0x2a7ac0 sub_2a7ac0
    Lifted::new(0x7000993ad0, tpl_20 as *const (), 1), // +0x2a7ad0 sub_2a7ad0
    Lifted::new(0x7000993ae0, tpl_20 as *const (), 1), // +0x2a7ae0 sub_2a7ae0
    Lifted::new(0x7000993af0, tpl_28 as *const (), 1), // +0x2a7af0 sub_2a7af0
    Lifted::new(0x7000993b00, tpl_28 as *const (), 1), // +0x2a7b00 sub_2a7b00
    Lifted::new(0x7000993b10, tpl_20 as *const (), 1), // +0x2a7b10 sub_2a7b10
    Lifted::new(0x7000993b20, tpl_20 as *const (), 1), // +0x2a7b20 sub_2a7b20
    Lifted::new(0x7000993b30, tpl_20 as *const (), 1), // +0x2a7b30 sub_2a7b30
    Lifted::new(0x7000993b40, tpl_20 as *const (), 1), // +0x2a7b40 sub_2a7b40
    Lifted::new(0x7000993b50, fn_7000993b50 as *const (), 0), // +0x2a7b50 sub_2a7b50
    Lifted::new(0x7000993c00, fn_7000993c00 as *const (), 0), // +0x2a7c00 sub_2a7c00
    Lifted::new(0x7000993ca0, fn_7000993ca0 as *const (), 0), // +0x2a7ca0 sub_2a7ca0
    Lifted::new(0x7000993d50, fn_7000993d50 as *const (), 0), // +0x2a7d50 sub_2a7d50
    Lifted::new(0x7000993df0, tpl_27::<0x4> as *const (), 1), // +0x2a7df0 sub_2a7df0
    Lifted::new(0x7000993e10, tpl_28 as *const (), 1), // +0x2a7e10 sub_2a7e10
    Lifted::new(0x7000993e20, fn_7000993e20 as *const (), 1), // +0x2a7e20 sub_2a7e20
    Lifted::new(0x7000993e50, tpl_21::<0x10> as *const (), 3), // +0x2a7e50 sub_2a7e50
    Lifted::new(0x7000993e70, tpl_28 as *const (), 1), // +0x2a7e70 sub_2a7e70
    Lifted::new(0x7000993e80, tpl_28 as *const (), 1), // +0x2a7e80 sub_2a7e80
    Lifted::new(0x7000993e90, fn_7000993e90 as *const (), 1), // +0x2a7e90 sub_2a7e90
    Lifted::new(0x7000993eb0, tpl_121::<0x8> as *const (), 3), // +0x2a7eb0 sub_2a7eb0
    Lifted::new(0x7000993ed0, tpl_23::<0x8, 0x10> as *const (), 1), // +0x2a7ed0 sub_2a7ed0
    Lifted::new(0x7000993ef0, tpl_21::<0x10> as *const (), 3), // +0x2a7ef0 sub_2a7ef0
    Lifted::new(0x7000993f10, tpl_20 as *const (), 1), // +0x2a7f10 sub_2a7f10
    Lifted::new(0x7000993f20, tpl_20 as *const (), 1), // +0x2a7f20 sub_2a7f20
    Lifted::new(0x7000993f30, tpl_20 as *const (), 1), // +0x2a7f30 sub_2a7f30
    Lifted::new(0x7000993f40, tpl_20 as *const (), 1), // +0x2a7f40 sub_2a7f40
    Lifted::new(0x7000993f50, fn_7000993f50 as *const (), 1), // +0x2a7f50 sub_2a7f50
    Lifted::new(0x7000993f80, tpl_25::<0x10> as *const (), 3), // +0x2a7f80 sub_2a7f80
    Lifted::new(0x7000993fa0, tpl_19 as *const (), 3), // +0x2a7fa0 sub_2a7fa0
    Lifted::new(0x7000993fb0, tpl_19 as *const (), 3), // +0x2a7fb0 sub_2a7fb0
    Lifted::new(0x7000993fc0, tpl_28 as *const (), 1), // +0x2a7fc0 sub_2a7fc0
    Lifted::new(0x7000993fd0, tpl_28 as *const (), 1), // +0x2a7fd0 sub_2a7fd0
    Lifted::new(0x7000993fe0, tpl_20 as *const (), 1), // +0x2a7fe0 sub_2a7fe0
    Lifted::new(0x7000993ff0, tpl_20 as *const (), 1), // +0x2a7ff0 sub_2a7ff0
    Lifted::new(0x7000994000, tpl_117::<0x70009a8d30, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const (), 0), // +0x2a8000 sub_2a8000
    Lifted::new(0x70009940d0, tpl_122::<0x7001779e20, 0x700177c312, 0x70009a31b0, 0x70009a3860> as *const (), 2), // +0x2a80d0 sub_2a80d0
    Lifted::new(0x7000994280, tpl_28 as *const (), 1), // +0x2a8280 sub_2a8280
    Lifted::new(0x7000994290, tpl_28 as *const (), 1), // +0x2a8290 sub_2a8290
    Lifted::new(0x70009942a0, tpl_28 as *const (), 1), // +0x2a82a0 sub_2a82a0
    Lifted::new(0x70009942b0, tpl_28 as *const (), 1), // +0x2a82b0 sub_2a82b0
    Lifted::new(0x70009942c0, tpl_28 as *const (), 1), // +0x2a82c0 sub_2a82c0
    Lifted::new(0x70009942d0, tpl_28 as *const (), 1), // +0x2a82d0 sub_2a82d0
    Lifted::new(0x70009942e0, tpl_20 as *const (), 1), // +0x2a82e0 sub_2a82e0
    Lifted::new(0x70009942f0, tpl_28 as *const (), 1), // +0x2a82f0 sub_2a82f0
    Lifted::new(0x7000994300, tpl_28 as *const (), 1), // +0x2a8300 sub_2a8300
    Lifted::new(0x7000994310, tpl_20 as *const (), 1), // +0x2a8310 sub_2a8310
    Lifted::new(0x7000994320, tpl_20 as *const (), 1), // +0x2a8320 sub_2a8320
    Lifted::new(0x7000994330, tpl_28 as *const (), 1), // +0x2a8330 sub_2a8330
    Lifted::new(0x7000994340, tpl_28 as *const (), 1), // +0x2a8340 sub_2a8340
    Lifted::new(0x7000994350, tpl_20 as *const (), 1), // +0x2a8350 sub_2a8350
    Lifted::new(0x7000994360, tpl_20 as *const (), 1), // +0x2a8360 sub_2a8360
    Lifted::new(0x7000994370, tpl_28 as *const (), 1), // +0x2a8370 sub_2a8370
    Lifted::new(0x7000994380, tpl_28 as *const (), 1), // +0x2a8380 sub_2a8380
    Lifted::new(0x7000994390, tpl_20 as *const (), 1), // +0x2a8390 sub_2a8390
    Lifted::new(0x70009943a0, tpl_20 as *const (), 1), // +0x2a83a0 sub_2a83a0
    Lifted::new(0x70009943b0, tpl_20 as *const (), 1), // +0x2a83b0 sub_2a83b0
    Lifted::new(0x70009943c0, tpl_29::<0x8> as *const (), 1), // +0x2a83c0 sub_2a83c0
    Lifted::new(0x70009943e0, tpl_19 as *const (), 3), // +0x2a83e0 sub_2a83e0
    Lifted::new(0x70009943f0, tpl_123::<0x8, 0x10, 0x18, 0x20, 0x30> as *const (), 3), // +0x2a83f0 sub_2a83f0
    Lifted::new(0x7000994430, tpl_123::<0x8, 0x10, 0x18, 0x20, 0x30> as *const (), 3), // +0x2a8430 sub_2a8430
    Lifted::new(0x7000994470, tpl_20 as *const (), 1), // +0x2a8470 sub_2a8470
    Lifted::new(0x7000994480, tpl_20 as *const (), 1), // +0x2a8480 sub_2a8480
    Lifted::new(0x7000994490, tpl_124 as *const (), 1), // +0x2a8490 sub_2a8490
    Lifted::new(0x70009944a0, tpl_124 as *const (), 1), // +0x2a84a0 sub_2a84a0
    Lifted::new(0x70009944b0, tpl_20 as *const (), 1), // +0x2a84b0 sub_2a84b0
    Lifted::new(0x70009944c0, tpl_20 as *const (), 1), // +0x2a84c0 sub_2a84c0
    Lifted::new(0x70009944d0, tpl_83::<0x7000992880> as *const (), 0), // +0x2a84d0 sub_2a84d0
    Lifted::new(0x7000994500, tpl_83::<0x7000992920> as *const (), 0), // +0x2a8500 sub_2a8500
    Lifted::new(0x7000994530, fn_7000994530 as *const (), 0), // +0x2a8530 sub_2a8530
    Lifted::new(0x70009946b0, fn_70009946b0 as *const (), 2), // +0x2a86b0 sub_2a86b0
    Lifted::new(0x7000994850, fn_7000994850 as *const (), 2), // +0x2a8850 sub_2a8850
    Lifted::new(0x70009949a0, fn_70009949a0 as *const (), 3), // +0x2a89a0 sub_2a89a0
    Lifted::new(0x70009949f0, fn_70009949f0 as *const (), 3), // +0x2a89f0 sub_2a89f0
    Lifted::new(0x7000994a20, fn_7000994a20 as *const (), 1), // +0x2a8a20 sub_2a8a20
    Lifted::new(0x7000994a40, tpl_20 as *const (), 1), // +0x2a8a40 sub_2a8a40
    Lifted::new(0x7000994a50, tpl_20 as *const (), 1), // +0x2a8a50 sub_2a8a50
    Lifted::new(0x7000994a60, tpl_28 as *const (), 1), // +0x2a8a60 sub_2a8a60
    Lifted::new(0x7000994a70, tpl_28 as *const (), 1), // +0x2a8a70 sub_2a8a70
    Lifted::new(0x7000994a80, tpl_20 as *const (), 1), // +0x2a8a80 sub_2a8a80
    Lifted::new(0x7000994a90, tpl_20 as *const (), 1), // +0x2a8a90 sub_2a8a90
    Lifted::new(0x7000994aa0, tpl_20 as *const (), 1), // +0x2a8aa0 sub_2a8aa0
    Lifted::new(0x7000994ab0, tpl_20 as *const (), 1), // +0x2a8ab0 sub_2a8ab0
    Lifted::new(0x7000994ac0, tpl_13::<0x7000992920> as *const (), 0), // +0x2a8ac0 sub_2a8ac0
    Lifted::new(0x7000994ae0, tpl_125::<0x8, 0x10, 0x18> as *const (), 1), // +0x2a8ae0 sub_2a8ae0
    Lifted::new(0x7000994b10, tpl_125::<0x8, 0x10, 0x18> as *const (), 1), // +0x2a8b10 sub_2a8b10
    Lifted::new(0x7000994b40, fn_7000994b40 as *const (), 1), // +0x2a8b40 sub_2a8b40
    Lifted::new(0x7000994b60, fn_7000994b60 as *const (), 1), // +0x2a8b60 sub_2a8b60
    Lifted::new(0x7000994b90, tpl_121::<0x8> as *const (), 3), // +0x2a8b90 sub_2a8b90
    Lifted::new(0x7000994bb0, tpl_20 as *const (), 1), // +0x2a8bb0 sub_2a8bb0
    Lifted::new(0x7000994bc0, tpl_20 as *const (), 1), // +0x2a8bc0 sub_2a8bc0
    Lifted::new(0x7000994bd0, fn_7000994bd0 as *const (), 1), // +0x2a8bd0 sub_2a8bd0
    Lifted::new(0x7000994bf0, tpl_121::<0x8> as *const (), 3), // +0x2a8bf0 sub_2a8bf0
    Lifted::new(0x7000994c10, fn_7000994c10 as *const (), 1), // +0x2a8c10 sub_2a8c10
    Lifted::new(0x7000994c30, tpl_121::<0x8> as *const (), 3), // +0x2a8c30 sub_2a8c30
    Lifted::new(0x7000994c50, tpl_28 as *const (), 1), // +0x2a8c50 sub_2a8c50
    Lifted::new(0x7000994c60, tpl_28 as *const (), 1), // +0x2a8c60 sub_2a8c60
    Lifted::new(0x7000994c70, tpl_28 as *const (), 1), // +0x2a8c70 sub_2a8c70
    Lifted::new(0x7000994c80, tpl_28 as *const (), 1), // +0x2a8c80 sub_2a8c80
    Lifted::new(0x7000994c90, fn_7000994c90 as *const (), 0), // +0x2a8c90 sub_2a8c90
    Lifted::new(0x7000994d50, fn_7000994d50 as *const (), 0), // +0x2a8d50 sub_2a8d50
    Lifted::new(0x7000994dd0, tpl_20 as *const (), 1), // +0x2a8dd0 sub_2a8dd0
    Lifted::new(0x7000994de0, tpl_20 as *const (), 1), // +0x2a8de0 sub_2a8de0
    Lifted::new(0x7000994df0, tpl_20 as *const (), 1), // +0x2a8df0 sub_2a8df0
    Lifted::new(0x7000994e00, tpl_20 as *const (), 1), // +0x2a8e00 sub_2a8e00
    Lifted::new(0x7000994e10, tpl_20 as *const (), 1), // +0x2a8e10 sub_2a8e10
    Lifted::new(0x7000994e20, tpl_20 as *const (), 1), // +0x2a8e20 sub_2a8e20
    Lifted::new(0x7000994e30, fn_7000994e30 as *const (), 1), // +0x2a8e30 sub_2a8e30
    Lifted::new(0x7000994e60, tpl_25::<0x10> as *const (), 3), // +0x2a8e60 sub_2a8e60
    Lifted::new(0x7000994e80, fn_7000994e80 as *const (), 1), // +0x2a8e80 sub_2a8e80
    Lifted::new(0x7000994e90, fn_7000994e90 as *const (), 1), // +0x2a8e90 sub_2a8e90
    Lifted::new(0x7000994ea0, tpl_126 as *const (), 1), // +0x2a8ea0 sub_2a8ea0
    Lifted::new(0x7000994eb0, tpl_126 as *const (), 1), // +0x2a8eb0 sub_2a8eb0
    Lifted::new(0x7000994ec0, tpl_127 as *const (), 1), // +0x2a8ec0 sub_2a8ec0
    Lifted::new(0x7000994ed0, tpl_127 as *const (), 1), // +0x2a8ed0 sub_2a8ed0
    Lifted::new(0x7000994ee0, fn_7000994ee0 as *const (), 1), // +0x2a8ee0 sub_2a8ee0
    Lifted::new(0x7000994ef0, fn_7000994ef0 as *const (), 1), // +0x2a8ef0 sub_2a8ef0
    Lifted::new(0x7000994f00, fn_7000994f00 as *const (), 1), // +0x2a8f00 sub_2a8f00
    Lifted::new(0x7000994f10, fn_7000994f10 as *const (), 1), // +0x2a8f10 sub_2a8f10
    Lifted::new(0x7000994f20, fn_7000994f20 as *const (), 0), // +0x2a8f20 sub_2a8f20
    Lifted::new(0x7000994f60, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a8f60 sub_2a8f60
    Lifted::new(0x7000994fb0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a8fb0 sub_2a8fb0
    Lifted::new(0x7000995000, tpl_128::<0xfffffffffffffff8, 0x700177c312, 0x7000701460> as *const (), 0), // +0x2a9000 sub_2a9000
    Lifted::new(0x7000995050, tpl_129::<0xfffffffffffffff8, 0x7000714fd0, 0x700177c312> as *const (), 0), // +0x2a9050 sub_2a9050
    Lifted::new(0x7000995120, fn_7000995120 as *const (), 0), // +0x2a9120 sub_2a9120
    Lifted::new(0x70009952b0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a92b0 sub_2a92b0
    Lifted::new(0x7000995300, fn_7000995300 as *const (), 0), // +0x2a9300 sub_2a9300
    Lifted::new(0x70009953d0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a93d0 sub_2a93d0
    Lifted::new(0x7000995420, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a9420 sub_2a9420
    Lifted::new(0x7000995470, fn_7000995470 as *const (), 0), // +0x2a9470 sub_2a9470
    Lifted::new(0x7000995540, tpl_130::<0xfffffffffffffff8, 0x700072b260, 0x700177c312> as *const (), 0), // +0x2a9540 sub_2a9540
    Lifted::new(0x7000995610, tpl_130::<0xfffffffffffffff8, 0x700072b260, 0x700177c312> as *const (), 0), // +0x2a9610 sub_2a9610
    Lifted::new(0x70009956e0, tpl_131::<0xfffffffffffffff8, 0x7000770500, 0x700177c312> as *const (), 0), // +0x2a96e0 sub_2a96e0
    Lifted::new(0x7000995790, fn_7000995790 as *const (), 0), // +0x2a9790 sub_2a9790
    Lifted::new(0x70009958c0, fn_70009958c0 as *const (), 0), // +0x2a98c0 sub_2a98c0
    Lifted::new(0x7000995a80, fn_7000995a80 as *const (), 0), // +0x2a9a80 sub_2a9a80
    Lifted::new(0x7000995b40, tpl_128::<0xfffffffffffffff8, 0x700177c312, 0x7000701460> as *const (), 0), // +0x2a9b40 sub_2a9b40
    Lifted::new(0x7000995b90, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a9b90 sub_2a9b90
    Lifted::new(0x7000995be0, fn_7000995be0 as *const (), 0), // +0x2a9be0 sub_2a9be0
    Lifted::new(0x7000995d10, tpl_128::<0xfffffffffffffff8, 0x700177c312, 0x7000701460> as *const (), 0), // +0x2a9d10 sub_2a9d10
    Lifted::new(0x7000995d60, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a9d60 sub_2a9d60
    Lifted::new(0x7000995db0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a9db0 sub_2a9db0
    Lifted::new(0x7000995e00, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2a9e00 sub_2a9e00
    Lifted::new(0x7000995e50, fn_7000995e50 as *const (), 0), // +0x2a9e50 sub_2a9e50
    Lifted::new(0x7000995f50, fn_7000995f50 as *const (), 0), // +0x2a9f50 sub_2a9f50
    Lifted::new(0x70009960e0, fn_70009960e0 as *const (), 0), // +0x2aa0e0 sub_2aa0e0
    Lifted::new(0x7000996190, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2aa190 sub_2aa190
    Lifted::new(0x70009961e0, tpl_129::<0xfffffffffffffff8, 0x700071c6d0, 0x700177c312> as *const (), 0), // +0x2aa1e0 sub_2aa1e0
    Lifted::new(0x70009962b0, fn_70009962b0 as *const (), 0), // +0x2aa2b0 sub_2aa2b0
    Lifted::new(0x7000996360, fn_7000996360 as *const (), 0), // +0x2aa360 sub_2aa360
    Lifted::new(0x7000996390, fn_7000996390 as *const (), 1), // +0x2aa390 sub_2aa390
    Lifted::new(0x70009963a0, fn_70009963a0 as *const (), 0), // +0x2aa3a0 sub_2aa3a0
    Lifted::new(0x70009963d0, tpl_132::<0x30, 0x70017d2c68, 0x8, 0x10, 0xc8d0, 0x700177c312> as *const (), 0), // +0x2aa3d0 sub_2aa3d0
    Lifted::new(0x7000996430, tpl_132::<0x38, 0x70017d2c68, 0x8, 0x10, 0xc8d0, 0x700177c312> as *const (), 0), // +0x2aa430 sub_2aa430
    Lifted::new(0x7000996490, tpl_33::<0x700177c312> as *const (), 0), // +0x2aa490 sub_2aa490
    Lifted::new(0x70009964c0, tpl_33::<0x700177c312> as *const (), 0), // +0x2aa4c0 sub_2aa4c0
    Lifted::new(0x70009964f0, tpl_33::<0x7000701460> as *const (), 0), // +0x2aa4f0 sub_2aa4f0
    Lifted::new(0x7000996520, tpl_133::<0x7000714fd0, 0x700177c312> as *const (), 0), // +0x2aa520 sub_2aa520
    Lifted::new(0x7000996560, tpl_133::<0x700071eb70, 0x7000701460> as *const (), 0), // +0x2aa560 sub_2aa560
    Lifted::new(0x70009965a0, tpl_33::<0x700177c312> as *const (), 0), // +0x2aa5a0 sub_2aa5a0
    Lifted::new(0x70009965d0, tpl_134::<0x8, 0x70017d2c68, 0x10, 0xc8d0, 0x700177c312> as *const (), 0), // +0x2aa5d0 sub_2aa5d0
    Lifted::new(0x7000996620, tpl_133::<0x70007128a0, 0x700177c312> as *const (), 0), // +0x2aa620 sub_2aa620
    Lifted::new(0x7000996660, tpl_33::<0x700177c312> as *const (), 0), // +0x2aa660 sub_2aa660
    Lifted::new(0x7000996690, tpl_33::<0x700177c312> as *const (), 0), // +0x2aa690 sub_2aa690
    Lifted::new(0x70009966c0, tpl_134::<0x8, 0x70017d2c68, 0x10, 0xc8d0, 0x700177c312> as *const (), 0), // +0x2aa6c0 sub_2aa6c0
    Lifted::new(0x7000996710, tpl_133::<0x700072b260, 0x700177c312> as *const (), 0), // +0x2aa710 sub_2aa710
    Lifted::new(0x7000996750, tpl_133::<0x700072b260, 0x700177c312> as *const (), 0), // +0x2aa750 sub_2aa750
    Lifted::new(0x7000996790, tpl_132::<0x30, 0x70017d2c68, 0x8, 0x10, 0xc8d0, 0x700177c312> as *const (), 0), // +0x2aa790 sub_2aa790
    Lifted::new(0x70009967f0, tpl_133::<0x7000770500, 0x700177c312> as *const (), 0), // +0x2aa7f0 sub_2aa7f0
    Lifted::new(0x7000996830, tpl_135::<0x70017d3198, 0x7000780b90, 0x108, 0x7001d550c0, 0xf8, 0x7001d54ea0, 0x700070d940, 0x7000701460> as *const (), 0), // +0x2aa830 sub_2aa830
    Lifted::new(0x70009968b0, tpl_133::<0x700070bc20, 0x7000701460> as *const (), 0), // +0x2aa8b0 sub_2aa8b0
    Lifted::new(0x70009968f0, tpl_136::<0x7001d54470, 0x700177c312> as *const (), 0), // +0x2aa8f0 sub_2aa8f0
    Lifted::new(0x7000996930, tpl_33::<0x7000701460> as *const (), 0), // +0x2aa930 sub_2aa930
    Lifted::new(0x7000996960, tpl_33::<0x700177c312> as *const (), 0), // +0x2aa960 sub_2aa960
    Lifted::new(0x7000996990, tpl_135::<0x70017d3310, 0x70006f0c20, 0x150, 0x7001d550c0, 0x138, 0x7001d54ea0, 0x700070d940, 0x7000701460> as *const (), 0), // +0x2aa990 sub_2aa990
    Lifted::new(0x7000996a10, tpl_33::<0x7000701460> as *const (), 0), // +0x2aaa10 sub_2aaa10
    Lifted::new(0x7000996a40, tpl_132::<0x38, 0x70017d2c68, 0x8, 0x10, 0xc8d0, 0x700177c312> as *const (), 0), // +0x2aaa40 sub_2aaa40
    Lifted::new(0x7000996aa0, tpl_33::<0x700177c312> as *const (), 0), // +0x2aaaa0 sub_2aaaa0
    Lifted::new(0x7000996ad0, tpl_33::<0x700177c312> as *const (), 0), // +0x2aaad0 sub_2aaad0
    Lifted::new(0x7000996b00, tpl_134::<0x8, 0x70017d2c68, 0x10, 0xc8d0, 0x700177c312> as *const (), 0), // +0x2aab00 sub_2aab00
    Lifted::new(0x7000996b50, tpl_33::<0x700177c312> as *const (), 0), // +0x2aab50 sub_2aab50
    Lifted::new(0x7000996b80, tpl_137::<0x14, 0x70017d3570, 0x8, 0x700177c312> as *const (), 0), // +0x2aab80 sub_2aab80
    Lifted::new(0x7000996be0, tpl_137::<0x14, 0x70017d3570, 0x8, 0x700177c312> as *const (), 0), // +0x2aabe0 sub_2aabe0
    Lifted::new(0x7000996c40, tpl_133::<0x700070d940, 0x7000701460> as *const (), 0), // +0x2aac40 sub_2aac40
    Lifted::new(0x7000996c80, tpl_133::<0x7000720400, 0x7000701460> as *const (), 0), // +0x2aac80 sub_2aac80
    Lifted::new(0x7000996cc0, tpl_33::<0x700177c312> as *const (), 0), // +0x2aacc0 sub_2aacc0
    Lifted::new(0x7000996cf0, tpl_133::<0x700071c6d0, 0x700177c312> as *const (), 0), // +0x2aacf0 sub_2aacf0
    Lifted::new(0x7000996d30, tpl_133::<0x700071c6d0, 0x700177c312> as *const (), 0), // +0x2aad30 sub_2aad30
    Lifted::new(0x7000996d70, fn_7000996d70 as *const (), 0), // +0x2aad70 sub_2aad70
    Lifted::new(0x7000996dd0, tpl_112::<0x7001d55090, 0x7000996dda> as *const (), 0), // +0x2aadd0 sub_2aadd0
    Lifted::new(0x7000996de0, tpl_112::<0x7001d55090, 0x7000996dea> as *const (), 0), // +0x2aade0 sub_2aade0
    Lifted::new(0x7000996df0, tpl_138::<0x18, 0x20> as *const (), 1), // +0x2aadf0 sub_2aadf0
    Lifted::new(0x7000996e20, tpl_139::<0x18> as *const (), 1), // +0x2aae20 sub_2aae20
    Lifted::new(0x7000996e30, fn_7000996e30 as *const (), 0), // +0x2aae30 sub_2aae30
    Lifted::new(0x7000996e80, fn_7000996e80 as *const (), 1), // +0x2aae80 sub_2aae80
    Lifted::new(0x7000996e90, fn_7000996e90 as *const (), 0), // +0x2aae90 sub_2aae90
    Lifted::new(0x7000996eb0, fn_7000996eb0 as *const (), 0), // +0x2aaeb0 sub_2aaeb0
    Lifted::new(0x7000996ed0, fn_7000996ed0 as *const (), 1), // +0x2aaed0 sub_2aaed0
    Lifted::new(0x7000996ef0, fn_7000996ef0 as *const (), 0), // +0x2aaef0 sub_2aaef0
    Lifted::new(0x7000996f10, fn_7000996f10 as *const (), 0), // +0x2aaf10 sub_2aaf10
    Lifted::new(0x7000996f30, fn_7000996f30 as *const (), 0), // +0x2aaf30 sub_2aaf30
    Lifted::new(0x7000996f50, tpl_140::<0x20, 0x1> as *const (), 1), // +0x2aaf50 sub_2aaf50
    Lifted::new(0x7000996f60, fn_7000996f60 as *const (), 0), // +0x2aaf60 sub_2aaf60
    Lifted::new(0x7000996f80, fn_7000996f80 as *const (), 0), // +0x2aaf80 sub_2aaf80
    Lifted::new(0x7000996fa0, fn_7000996fa0 as *const (), 0), // +0x2aafa0 sub_2aafa0
    Lifted::new(0x70009970a0, fn_70009970a0 as *const (), 0), // +0x2ab0a0 sub_2ab0a0
    Lifted::new(0x70009970c0, fn_70009970c0 as *const (), 0), // +0x2ab0c0 sub_2ab0c0
    Lifted::new(0x70009970e0, fn_70009970e0 as *const (), 0), // +0x2ab0e0 sub_2ab0e0
    Lifted::new(0x7000997120, fn_7000997120 as *const (), 1), // +0x2ab120 sub_2ab120
    Lifted::new(0x7000997140, fn_7000997140 as *const (), 0), // +0x2ab140 sub_2ab140
    Lifted::new(0x7000997160, fn_7000997160 as *const (), 0), // +0x2ab160 sub_2ab160
    Lifted::new(0x7000997190, fn_7000997190 as *const (), 0), // +0x2ab190 sub_2ab190
    Lifted::new(0x70009971b0, tpl_15 as *const (), 1), // +0x2ab1b0 sub_2ab1b0
    Lifted::new(0x70009971c0, tpl_15 as *const (), 1), // +0x2ab1c0 sub_2ab1c0
    Lifted::new(0x70009971d0, tpl_15 as *const (), 1), // +0x2ab1d0 sub_2ab1d0
    Lifted::new(0x70009971e0, tpl_15 as *const (), 1), // +0x2ab1e0 sub_2ab1e0
    Lifted::new(0x70009971f0, tpl_15 as *const (), 1), // +0x2ab1f0 sub_2ab1f0
    Lifted::new(0x7000997200, tpl_15 as *const (), 1), // +0x2ab200 sub_2ab200
    Lifted::new(0x7000997210, tpl_15 as *const (), 1), // +0x2ab210 sub_2ab210
    Lifted::new(0x7000997220, tpl_15 as *const (), 1), // +0x2ab220 sub_2ab220
    Lifted::new(0x7000997230, tpl_15 as *const (), 1), // +0x2ab230 sub_2ab230
    Lifted::new(0x7000997240, tpl_15 as *const (), 1), // +0x2ab240 sub_2ab240
    Lifted::new(0x7000997250, fn_7000997250 as *const (), 0), // +0x2ab250 sub_2ab250
    Lifted::new(0x7000997270, fn_7000997270 as *const (), 0), // +0x2ab270 sub_2ab270
    Lifted::new(0x70009972b0, fn_70009972b0 as *const (), 0), // +0x2ab2b0 sub_2ab2b0
    Lifted::new(0x70009972f0, fn_70009972f0 as *const (), 0), // +0x2ab2f0 sub_2ab2f0
    Lifted::new(0x7000997310, fn_7000997310 as *const (), 0), // +0x2ab310 sub_2ab310
    Lifted::new(0x7000997330, tpl_15 as *const (), 1), // +0x2ab330 sub_2ab330
    Lifted::new(0x7000997340, tpl_141::<0x7001d550d0, 0x8, 0x700077c660, 0xf8, 0x7001d550c8> as *const (), 0), // +0x2ab340 sub_2ab340
    Lifted::new(0x7000997390, fn_7000997390 as *const (), 0), // +0x2ab390 sub_2ab390
    Lifted::new(0x70009973d0, fn_70009973d0 as *const (), 0), // +0x2ab3d0 sub_2ab3d0
    Lifted::new(0x70009974c0, fn_70009974c0 as *const (), 0), // +0x2ab4c0 sub_2ab4c0
    Lifted::new(0x70009975f0, fn_70009975f0 as *const (), 0), // +0x2ab5f0 sub_2ab5f0
    Lifted::new(0x7000997720, tpl_142::<0x4c0> as *const (), 1), // +0x2ab720 sub_2ab720
    Lifted::new(0x7000997740, fn_7000997740 as *const (), 0), // +0x2ab740 sub_2ab740
    Lifted::new(0x7000997850, fn_7000997850 as *const (), 0), // +0x2ab850 sub_2ab850
    Lifted::new(0x7000997940, fn_7000997940 as *const (), 0), // +0x2ab940 sub_2ab940
    Lifted::new(0x70009979d0, fn_70009979d0 as *const (), 0), // +0x2ab9d0 sub_2ab9d0
    Lifted::new(0x7000997a40, fn_7000997a40 as *const (), 0), // +0x2aba40 sub_2aba40
    Lifted::new(0x7000997bf0, fn_7000997bf0 as *const (), 0), // +0x2abbf0 sub_2abbf0
    Lifted::new(0x7000997d60, fn_7000997d60 as *const (), 0), // +0x2abd60 sub_2abd60
    Lifted::new(0x7000997f00, fn_7000997f00 as *const (), 0), // +0x2abf00 sub_2abf00
    Lifted::new(0x70009980d0, fn_70009980d0 as *const (), 0), // +0x2ac0d0 sub_2ac0d0
    Lifted::new(0x7000998220, fn_7000998220 as *const (), 0), // +0x2ac220 sub_2ac220
    Lifted::new(0x70009982f0, fn_70009982f0 as *const (), 0), // +0x2ac2f0 sub_2ac2f0
    Lifted::new(0x7000998310, fn_7000998310 as *const (), 1), // +0x2ac310 sub_2ac310
    Lifted::new(0x7000998320, fn_7000998320 as *const (), 1), // +0x2ac320 sub_2ac320
    Lifted::new(0x7000998330, tpl_39::<0x118> as *const (), 1), // +0x2ac330 sub_2ac330
    Lifted::new(0x7000998340, fn_7000998340 as *const (), 1), // +0x2ac340 sub_2ac340
    Lifted::new(0x7000998390, fn_7000998390 as *const (), 1), // +0x2ac390 sub_2ac390
    Lifted::new(0x70009983d0, tpl_143::<0x8, 0x700070eae0> as *const (), 0), // +0x2ac3d0 sub_2ac3d0
    Lifted::new(0x70009983e0, tpl_142::<0x4e8> as *const (), 1), // +0x2ac3e0 sub_2ac3e0
    Lifted::new(0x7000998400, tpl_142::<0x510> as *const (), 1), // +0x2ac400 sub_2ac400
    Lifted::new(0x7000998420, tpl_142::<0x518> as *const (), 1), // +0x2ac420 sub_2ac420
    Lifted::new(0x7000998440, tpl_142::<0x500> as *const (), 1), // +0x2ac440 sub_2ac440
    Lifted::new(0x7000998460, tpl_30 as *const (), 1), // +0x2ac460 sub_2ac460
    Lifted::new(0x7000998470, tpl_20 as *const (), 1), // +0x2ac470 sub_2ac470
    Lifted::new(0x7000998480, tpl_36::<0x90> as *const (), 1), // +0x2ac480 sub_2ac480
    Lifted::new(0x7000998490, tpl_36::<0xd920> as *const (), 1), // +0x2ac490 sub_2ac490
    Lifted::new(0x70009984a0, tpl_20 as *const (), 1), // +0x2ac4a0 sub_2ac4a0
    Lifted::new(0x70009984b0, tpl_15 as *const (), 1), // +0x2ac4b0 sub_2ac4b0
    Lifted::new(0x70009984c0, fn_70009984c0 as *const (), 0), // +0x2ac4c0 sub_2ac4c0
    Lifted::new(0x7000998550, fn_7000998550 as *const (), 1), // +0x2ac550 sub_2ac550
    Lifted::new(0x7000998560, tpl_142::<0x558> as *const (), 1), // +0x2ac560 sub_2ac560
    Lifted::new(0x7000998580, tpl_142::<0x548> as *const (), 1), // +0x2ac580 sub_2ac580
    Lifted::new(0x70009985a0, tpl_142::<0x560> as *const (), 1), // +0x2ac5a0 sub_2ac5a0
    Lifted::new(0x70009985c0, tpl_142::<0x570> as *const (), 1), // +0x2ac5c0 sub_2ac5c0
    Lifted::new(0x70009985e0, fn_70009985e0 as *const (), 1), // +0x2ac5e0 sub_2ac5e0
    Lifted::new(0x7000998600, tpl_15 as *const (), 1), // +0x2ac600 sub_2ac600
    Lifted::new(0x7000998610, tpl_144 as *const (), 1), // +0x2ac610 sub_2ac610
    Lifted::new(0x7000998620, fn_7000998620 as *const (), 1), // +0x2ac620 sub_2ac620
    Lifted::new(0x7000998640, tpl_145::<0xf> as *const (), 1), // +0x2ac640 sub_2ac640
    Lifted::new(0x7000998650, tpl_145::<0xf> as *const (), 1), // +0x2ac650 sub_2ac650
    Lifted::new(0x7000998660, tpl_145::<0x13> as *const (), 1), // +0x2ac660 sub_2ac660
    Lifted::new(0x7000998670, tpl_146::<0xf> as *const (), 1), // +0x2ac670 sub_2ac670
    Lifted::new(0x7000998680, fn_7000998680 as *const (), 0), // +0x2ac680 sub_2ac680
    Lifted::new(0x70009986b0, tpl_36::<0x120> as *const (), 1), // +0x2ac6b0 sub_2ac6b0
    Lifted::new(0x70009986c0, tpl_147::<0x7> as *const (), 1), // +0x2ac6c0 sub_2ac6c0
    Lifted::new(0x70009986d0, fn_70009986d0 as *const (), 1), // +0x2ac6d0 sub_2ac6d0
    Lifted::new(0x70009986f0, fn_70009986f0 as *const (), 0), // +0x2ac6f0 sub_2ac6f0
    Lifted::new(0x7000998730, tpl_148 as *const (), 1), // +0x2ac730 sub_2ac730
    Lifted::new(0x7000998740, tpl_20 as *const (), 1), // +0x2ac740 sub_2ac740
    Lifted::new(0x7000998750, tpl_20 as *const (), 1), // +0x2ac750 sub_2ac750
    Lifted::new(0x7000998760, fn_7000998760 as *const (), 1), // +0x2ac760 sub_2ac760
    Lifted::new(0x7000998770, tpl_149::<0x1c, 0xffffffffffffffff> as *const (), 1), // +0x2ac770 sub_2ac770
    Lifted::new(0x70009987b0, tpl_150::<0x1c, 0xffffffffffffffff> as *const (), 1), // +0x2ac7b0 sub_2ac7b0
    Lifted::new(0x70009987e0, fn_70009987e0 as *const (), 1), // +0x2ac7e0 sub_2ac7e0
    Lifted::new(0x70009987f0, fn_70009987f0 as *const (), 1), // +0x2ac7f0 sub_2ac7f0
    Lifted::new(0x7000998800, tpl_151::<0x700071fe90> as *const (), 0), // +0x2ac800 sub_2ac800
    Lifted::new(0x7000998810, tpl_152::<0x700071fe90> as *const (), 0), // +0x2ac810 sub_2ac810
    Lifted::new(0x7000998820, tpl_47::<0x8> as *const (), 1), // +0x2ac820 sub_2ac820
    Lifted::new(0x7000998830, tpl_153::<0x700071fe90> as *const (), 0), // +0x2ac830 sub_2ac830
    Lifted::new(0x7000998840, tpl_154::<0x700071fe90> as *const (), 0), // +0x2ac840 sub_2ac840
    Lifted::new(0x7000998850, tpl_155::<0x700071fe90> as *const (), 0), // +0x2ac850 sub_2ac850
    Lifted::new(0x7000998860, tpl_156::<0x700071fe90> as *const (), 0), // +0x2ac860 sub_2ac860
    Lifted::new(0x7000998870, tpl_157::<0x700071fe90> as *const (), 0), // +0x2ac870 sub_2ac870
    Lifted::new(0x7000998880, tpl_158::<0x700071fe90> as *const (), 0), // +0x2ac880 sub_2ac880
    Lifted::new(0x7000998890, fn_7000998890 as *const (), 1), // +0x2ac890 sub_2ac890
    Lifted::new(0x70009988a0, fn_70009988a0 as *const (), 0), // +0x2ac8a0 sub_2ac8a0
    Lifted::new(0x70009988c0, tpl_151::<0x7000739f20> as *const (), 0), // +0x2ac8c0 sub_2ac8c0
    Lifted::new(0x70009988d0, fn_70009988d0 as *const (), 0), // +0x2ac8d0 sub_2ac8d0
    Lifted::new(0x7000998940, tpl_152::<0x7000739f20> as *const (), 0), // +0x2ac940 sub_2ac940
    Lifted::new(0x7000998950, fn_7000998950 as *const (), 2), // +0x2ac950 sub_2ac950
    Lifted::new(0x7000998ac0, fn_7000998ac0 as *const (), 0), // +0x2acac0 sub_2acac0
    Lifted::new(0x7000998bc0, fn_7000998bc0 as *const (), 0), // +0x2acbc0 sub_2acbc0
    Lifted::new(0x7000998c20, tpl_153::<0x7000739f20> as *const (), 0), // +0x2acc20 sub_2acc20
    Lifted::new(0x7000998c30, tpl_154::<0x7000739f20> as *const (), 0), // +0x2acc30 sub_2acc30
    Lifted::new(0x7000998c40, tpl_155::<0x7000739f20> as *const (), 0), // +0x2acc40 sub_2acc40
    Lifted::new(0x7000998c50, tpl_156::<0x7000739f20> as *const (), 0), // +0x2acc50 sub_2acc50
    Lifted::new(0x7000998c60, tpl_157::<0x7000739f20> as *const (), 0), // +0x2acc60 sub_2acc60
    Lifted::new(0x7000998c70, tpl_158::<0x7000739f20> as *const (), 0), // +0x2acc70 sub_2acc70
    Lifted::new(0x7000998c80, fn_7000998c80 as *const (), 1), // +0x2acc80 sub_2acc80
    Lifted::new(0x7000998c90, tpl_15 as *const (), 1), // +0x2acc90 sub_2acc90
    Lifted::new(0x7000998ca0, tpl_15 as *const (), 1), // +0x2acca0 sub_2acca0
    Lifted::new(0x7000998cb0, tpl_15 as *const (), 1), // +0x2accb0 sub_2accb0
    Lifted::new(0x7000998cc0, fn_7000998cc0 as *const (), 0), // +0x2accc0 sub_2accc0
    Lifted::new(0x7000998cf0, tpl_36::<0x18> as *const (), 1), // +0x2accf0 sub_2accf0
    Lifted::new(0x7000998d00, fn_7000998d00 as *const (), 0), // +0x2acd00 sub_2acd00
    Lifted::new(0x7000998d50, fn_7000998d50 as *const (), 1), // +0x2acd50 sub_2acd50
    Lifted::new(0x7000998d80, tpl_142::<0x5a8> as *const (), 1), // +0x2acd80 sub_2acd80
    Lifted::new(0x7000998da0, fn_7000998da0 as *const (), 0), // +0x2acda0 sub_2acda0
    Lifted::new(0x7000998dd0, fn_7000998dd0 as *const (), 1), // +0x2acdd0 sub_2acdd0
    Lifted::new(0x7000998e10, fn_7000998e10 as *const (), 1), // +0x2ace10 sub_2ace10
    Lifted::new(0x7000998e40, fn_7000998e40 as *const (), 1), // +0x2ace40 sub_2ace40
    Lifted::new(0x7000998e70, fn_7000998e70 as *const (), 1), // +0x2ace70 sub_2ace70
    Lifted::new(0x7000998ea0, tpl_159::<0x40> as *const (), 1), // +0x2acea0 sub_2acea0
    Lifted::new(0x7000998ec0, fn_7000998ec0 as *const (), 1), // +0x2acec0 sub_2acec0
    Lifted::new(0x7000998f30, tpl_36::<0x48> as *const (), 1), // +0x2acf30 sub_2acf30
    Lifted::new(0x7000998f40, fn_7000998f40 as *const (), 1), // +0x2acf40 sub_2acf40
    Lifted::new(0x7000998f60, tpl_160::<0x10> as *const (), 1), // +0x2acf60 sub_2acf60
    Lifted::new(0x7000998f70, tpl_161::<0x10> as *const (), 1), // +0x2acf70 sub_2acf70
    Lifted::new(0x7000998f80, tpl_162::<0x7001c71e94, 0x7001d55088> as *const (), 0), // +0x2acf80 sub_2acf80
    Lifted::new(0x7000998f90, tpl_162::<0x7001c71e90, 0x7001d55088> as *const (), 0), // +0x2acf90 sub_2acf90
    Lifted::new(0x7000998fa0, tpl_161::<0x10> as *const (), 1), // +0x2acfa0 sub_2acfa0
    Lifted::new(0x7000998fb0, tpl_163 as *const (), 1), // +0x2acfb0 sub_2acfb0
    Lifted::new(0x7000998fc0, tpl_164 as *const (), 1), // +0x2acfc0 sub_2acfc0
    Lifted::new(0x7000998fd0, tpl_142::<0x618> as *const (), 1), // +0x2acfd0 sub_2acfd0
    Lifted::new(0x7000998ff0, fn_7000998ff0 as *const (), 0), // +0x2acff0 sub_2acff0
    Lifted::new(0x7000999020, fn_7000999020 as *const (), 1), // +0x2ad020 sub_2ad020
    Lifted::new(0x7000999040, fn_7000999040 as *const (), 1), // +0x2ad040 sub_2ad040
    Lifted::new(0x7000999050, tpl_139::<0x130> as *const (), 1), // +0x2ad050 sub_2ad050
    Lifted::new(0x7000999060, tpl_139::<0x120> as *const (), 1), // +0x2ad060 sub_2ad060
    Lifted::new(0x7000999070, fn_7000999070 as *const (), 1), // +0x2ad070 sub_2ad070
    Lifted::new(0x70009990a0, tpl_165::<0x40> as *const (), 1), // +0x2ad0a0 sub_2ad0a0
    Lifted::new(0x70009990b0, tpl_165::<0x44> as *const (), 1), // +0x2ad0b0 sub_2ad0b0
    Lifted::new(0x70009990c0, fn_70009990c0 as *const (), 1), // +0x2ad0c0 sub_2ad0c0
    Lifted::new(0x7000999100, fn_7000999100 as *const (), 1), // +0x2ad100 sub_2ad100
    Lifted::new(0x7000999130, fn_7000999130 as *const (), 1), // +0x2ad130 sub_2ad130
    Lifted::new(0x7000999180, tpl_166::<0x7, 0x1f> as *const (), 1), // +0x2ad180 sub_2ad180
    Lifted::new(0x70009991b0, tpl_166::<0x7, 0x1f> as *const (), 1), // +0x2ad1b0 sub_2ad1b0
    Lifted::new(0x70009991e0, fn_70009991e0 as *const (), 0), // +0x2ad1e0 sub_2ad1e0
    Lifted::new(0x70009992a0, fn_70009992a0 as *const (), 0), // +0x2ad2a0 sub_2ad2a0
    Lifted::new(0x70009992e0, fn_70009992e0 as *const (), 0), // +0x2ad2e0 sub_2ad2e0
    Lifted::new(0x7000999310, tpl_167::<0x7, 0x1f> as *const (), 1), // +0x2ad310 sub_2ad310
    Lifted::new(0x7000999330, tpl_167::<0x7, 0x1f> as *const (), 1), // +0x2ad330 sub_2ad330
    Lifted::new(0x7000999350, fn_7000999350 as *const (), 1), // +0x2ad350 sub_2ad350
    Lifted::new(0x7000999370, tpl_168::<0xc7e6> as *const (), 1), // +0x2ad370 sub_2ad370
    Lifted::new(0x7000999380, tpl_169::<0x30> as *const (), 1), // +0x2ad380 sub_2ad380
    Lifted::new(0x7000999390, fn_7000999390 as *const (), 0), // +0x2ad390 sub_2ad390
    Lifted::new(0x70009993e0, fn_70009993e0 as *const (), 0), // +0x2ad3e0 sub_2ad3e0
    Lifted::new(0x7000999400, fn_7000999400 as *const (), 0), // +0x2ad400 sub_2ad400
    Lifted::new(0x7000999460, tpl_170::<0x70007602f0> as *const (), 0), // +0x2ad460 sub_2ad460
    Lifted::new(0x7000999480, fn_7000999480 as *const (), 0), // +0x2ad480 sub_2ad480
    Lifted::new(0x70009994b0, fn_70009994b0 as *const (), 0), // +0x2ad4b0 sub_2ad4b0
    Lifted::new(0x7000999570, fn_7000999570 as *const (), 0), // +0x2ad570 sub_2ad570
    Lifted::new(0x70009995d0, tpl_44 as *const (), 1), // +0x2ad5d0 sub_2ad5d0
    Lifted::new(0x70009995e0, tpl_171::<0xc7e6> as *const (), 1), // +0x2ad5e0 sub_2ad5e0
    Lifted::new(0x70009995f0, tpl_172::<0x10> as *const (), 1), // +0x2ad5f0 sub_2ad5f0
    Lifted::new(0x7000999600, fn_7000999600 as *const (), 0), // +0x2ad600 sub_2ad600
    Lifted::new(0x7000999630, fn_7000999630 as *const (), 1), // +0x2ad630 sub_2ad630
    Lifted::new(0x7000999690, tpl_173::<0x3> as *const (), 1), // +0x2ad690 sub_2ad690
    Lifted::new(0x70009996a0, tpl_160::<0x5> as *const (), 1), // +0x2ad6a0 sub_2ad6a0
    Lifted::new(0x70009996b0, fn_70009996b0 as *const (), 1), // +0x2ad6b0 sub_2ad6b0
    Lifted::new(0x70009996c0, tpl_160::<0x3> as *const (), 1), // +0x2ad6c0 sub_2ad6c0
    Lifted::new(0x70009996d0, tpl_173::<0x3> as *const (), 1), // +0x2ad6d0 sub_2ad6d0
    Lifted::new(0x70009996e0, fn_70009996e0 as *const (), 1), // +0x2ad6e0 sub_2ad6e0
    Lifted::new(0x7000999700, tpl_174::<0xf> as *const (), 1), // +0x2ad700 sub_2ad700
    Lifted::new(0x7000999720, tpl_175::<0x13> as *const (), 1), // +0x2ad720 sub_2ad720
    Lifted::new(0x7000999740, fn_7000999740 as *const (), 1), // +0x2ad740 sub_2ad740
    Lifted::new(0x7000999760, tpl_174::<0xf> as *const (), 1), // +0x2ad760 sub_2ad760
    Lifted::new(0x7000999780, tpl_175::<0x13> as *const (), 1), // +0x2ad780 sub_2ad780
    Lifted::new(0x70009997a0, tpl_173::<0x4> as *const (), 1), // +0x2ad7a0 sub_2ad7a0
    Lifted::new(0x70009997b0, tpl_173::<0x4> as *const (), 1), // +0x2ad7b0 sub_2ad7b0
    Lifted::new(0x70009997c0, tpl_142::<0x6c0> as *const (), 1), // +0x2ad7c0 sub_2ad7c0
    Lifted::new(0x70009997e0, tpl_142::<0x6e0> as *const (), 1), // +0x2ad7e0 sub_2ad7e0
    Lifted::new(0x7000999800, fn_7000999800 as *const (), 0), // +0x2ad800 sub_2ad800
    Lifted::new(0x70009999c0, fn_70009999c0 as *const (), 0), // +0x2ad9c0 sub_2ad9c0
    Lifted::new(0x7000999ac0, fn_7000999ac0 as *const (), 1), // +0x2adac0 sub_2adac0
    Lifted::new(0x7000999b20, fn_7000999b20 as *const (), 0), // +0x2adb20 sub_2adb20
    Lifted::new(0x7000999b90, fn_7000999b90 as *const (), 0), // +0x2adb90 sub_2adb90
    Lifted::new(0x7000999bc0, tpl_176::<0xd0> as *const (), 1), // +0x2adbc0 sub_2adbc0
    Lifted::new(0x7000999be0, fn_7000999be0 as *const (), 0), // +0x2adbe0 sub_2adbe0
    Lifted::new(0x7000999c00, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000999c11> as *const (), 0), // +0x2adc00 sub_2adc00
    Lifted::new(0x7000999c20, tpl_142::<0x6f8> as *const (), 1), // +0x2adc20 sub_2adc20
    Lifted::new(0x7000999c40, tpl_178::<0xf, 0x20, 0x13, 0x70007329a0> as *const (), 0), // +0x2adc40 sub_2adc40
    Lifted::new(0x7000999d30, fn_7000999d30 as *const (), 1), // +0x2add30 sub_2add30
    Lifted::new(0x7000999df0, fn_7000999df0 as *const (), 1), // +0x2addf0 sub_2addf0
    Lifted::new(0x7000999eb0, fn_7000999eb0 as *const (), 0), // +0x2adeb0 sub_2adeb0
    Lifted::new(0x7000999fa0, tpl_178::<0xf, 0x20, 0x13, 0x70007329a0> as *const (), 0), // +0x2adfa0 sub_2adfa0
    Lifted::new(0x700099a090, fn_700099a090 as *const (), 3), // +0x2ae090 sub_2ae090
    Lifted::new(0x700099a1b0, tpl_179::<0x70007191e0> as *const (), 0), // +0x2ae1b0 sub_2ae1b0
    Lifted::new(0x700099a1e0, fn_700099a1e0 as *const (), 0), // +0x2ae1e0 sub_2ae1e0
    Lifted::new(0x700099a240, fn_700099a240 as *const (), 0), // +0x2ae240 sub_2ae240
    Lifted::new(0x700099a260, tpl_36::<0xc0> as *const (), 1), // +0x2ae260 sub_2ae260
    Lifted::new(0x700099a270, fn_700099a270 as *const (), 1), // +0x2ae270 sub_2ae270
    Lifted::new(0x700099a290, tpl_142::<0x708> as *const (), 1), // +0x2ae290 sub_2ae290
    Lifted::new(0x700099a2b0, tpl_142::<0x710> as *const (), 1), // +0x2ae2b0 sub_2ae2b0
    Lifted::new(0x700099a2d0, fn_700099a2d0 as *const (), 0), // +0x2ae2d0 sub_2ae2d0
    Lifted::new(0x700099a380, tpl_15 as *const (), 1), // +0x2ae380 sub_2ae380
    Lifted::new(0x700099a390, fn_700099a390 as *const (), 0), // +0x2ae390 sub_2ae390
    Lifted::new(0x700099a3a0, tpl_180 as *const (), 1), // +0x2ae3a0 sub_2ae3a0
    Lifted::new(0x700099a3b0, tpl_20 as *const (), 1), // +0x2ae3b0 sub_2ae3b0
    Lifted::new(0x700099a3c0, tpl_93::<0x8> as *const (), 1), // +0x2ae3c0 sub_2ae3c0
    Lifted::new(0x700099a3d0, tpl_181 as *const (), 1), // +0x2ae3d0 sub_2ae3d0
    Lifted::new(0x700099a3e0, fn_700099a3e0 as *const (), 0), // +0x2ae3e0 sub_2ae3e0
    Lifted::new(0x700099a4d0, fn_700099a4d0 as *const (), 1), // +0x2ae4d0 sub_2ae4d0
    Lifted::new(0x700099a510, tpl_52::<0xffffffffffff6898> as *const (), 1), // +0x2ae510 sub_2ae510
    Lifted::new(0x700099a520, tpl_52::<0xfffffffffffffff8> as *const (), 1), // +0x2ae520 sub_2ae520
    Lifted::new(0x700099a530, tpl_92::<0x8> as *const (), 1), // +0x2ae530 sub_2ae530
    Lifted::new(0x700099a550, tpl_182 as *const (), 1), // +0x2ae550 sub_2ae550
    Lifted::new(0x700099a560, tpl_20 as *const (), 1), // +0x2ae560 sub_2ae560
    Lifted::new(0x700099a570, fn_700099a570 as *const (), 1), // +0x2ae570 sub_2ae570
    Lifted::new(0x700099a590, tpl_183::<0x8, 0x350> as *const (), 1), // +0x2ae590 sub_2ae590
    Lifted::new(0x700099a5f0, tpl_142::<0x720> as *const (), 1), // +0x2ae5f0 sub_2ae5f0
    Lifted::new(0x700099a610, tpl_142::<0x740> as *const (), 1), // +0x2ae610 sub_2ae610
    Lifted::new(0x700099a630, fn_700099a630 as *const (), 1), // +0x2ae630 sub_2ae630
    Lifted::new(0x700099a650, fn_700099a650 as *const (), 1), // +0x2ae650 sub_2ae650
    Lifted::new(0x700099a770, tpl_30 as *const (), 1), // +0x2ae770 sub_2ae770
    Lifted::new(0x700099a780, fn_700099a780 as *const (), 0), // +0x2ae780 sub_2ae780
    Lifted::new(0x700099a7d0, fn_700099a7d0 as *const (), 1), // +0x2ae7d0 sub_2ae7d0
    Lifted::new(0x700099a7e0, tpl_184 as *const (), 1), // +0x2ae7e0 sub_2ae7e0
    Lifted::new(0x700099a7f0, fn_700099a7f0 as *const (), 1), // +0x2ae7f0 sub_2ae7f0
    Lifted::new(0x700099a820, tpl_36::<0x8> as *const (), 1), // +0x2ae820 sub_2ae820
    Lifted::new(0x700099a830, fn_700099a830 as *const (), 1), // +0x2ae830 sub_2ae830
    Lifted::new(0x700099a840, tpl_185::<0x18> as *const (), 1), // +0x2ae840 sub_2ae840
    Lifted::new(0x700099a850, fn_700099a850 as *const (), 0), // +0x2ae850 sub_2ae850
    Lifted::new(0x700099a900, fn_700099a900 as *const (), 0), // +0x2ae900 sub_2ae900
    Lifted::new(0x700099a930, fn_700099a930 as *const (), 1), // +0x2ae930 sub_2ae930
    Lifted::new(0x700099a960, fn_700099a960 as *const (), 0), // +0x2ae960 sub_2ae960
    Lifted::new(0x700099a990, fn_700099a990 as *const (), 2), // +0x2ae990 sub_2ae990
    Lifted::new(0x700099abf0, fn_700099abf0 as *const (), 1), // +0x2aebf0 sub_2aebf0
    Lifted::new(0x700099ac20, fn_700099ac20 as *const (), 1), // +0x2aec20 sub_2aec20
    Lifted::new(0x700099ac50, fn_700099ac50 as *const (), 3), // +0x2aec50 sub_2aec50
    Lifted::new(0x700099ac90, fn_700099ac90 as *const (), 1), // +0x2aec90 sub_2aec90
    Lifted::new(0x700099ad50, tpl_186::<0x700182e5b0> as *const (), 1), // +0x2aed50 sub_2aed50
    Lifted::new(0x700099ad70, fn_700099ad70 as *const (), 1), // +0x2aed70 sub_2aed70
    Lifted::new(0x700099aea0, fn_700099aea0 as *const (), 0), // +0x2aeea0 sub_2aeea0
    Lifted::new(0x700099afe0, tpl_36::<0x10> as *const (), 1), // +0x2aefe0 sub_2aefe0
    Lifted::new(0x700099aff0, fn_700099aff0 as *const (), 1), // +0x2aeff0 sub_2aeff0
    Lifted::new(0x700099b030, tpl_187 as *const (), 1), // +0x2af030 sub_2af030
    Lifted::new(0x700099b040, fn_700099b040 as *const (), 0), // +0x2af040 sub_2af040
    Lifted::new(0x700099b130, fn_700099b130 as *const (), 1), // +0x2af130 sub_2af130
    Lifted::new(0x700099b150, fn_700099b150 as *const (), 0), // +0x2af150 sub_2af150
    Lifted::new(0x700099b180, fn_700099b180 as *const (), 0), // +0x2af180 sub_2af180
    Lifted::new(0x700099b1c0, tpl_188 as *const (), 1), // +0x2af1c0 sub_2af1c0
    Lifted::new(0x700099b1d0, tpl_188 as *const (), 1), // +0x2af1d0 sub_2af1d0
    Lifted::new(0x700099b1e0, tpl_188 as *const (), 1), // +0x2af1e0 sub_2af1e0
    Lifted::new(0x700099b1f0, tpl_188 as *const (), 1), // +0x2af1f0 sub_2af1f0
    Lifted::new(0x700099b200, tpl_188 as *const (), 1), // +0x2af200 sub_2af200
    Lifted::new(0x700099b210, fn_700099b210 as *const (), 1), // +0x2af210 sub_2af210
    Lifted::new(0x700099b230, fn_700099b230 as *const (), 1), // +0x2af230 sub_2af230
    Lifted::new(0x700099b250, fn_700099b250 as *const (), 1), // +0x2af250 sub_2af250
    Lifted::new(0x700099b270, fn_700099b270 as *const (), 1), // +0x2af270 sub_2af270
    Lifted::new(0x700099b290, fn_700099b290 as *const (), 1), // +0x2af290 sub_2af290
    Lifted::new(0x700099b2b0, fn_700099b2b0 as *const (), 1), // +0x2af2b0 sub_2af2b0
    Lifted::new(0x700099b2d0, tpl_36::<0x10> as *const (), 1), // +0x2af2d0 sub_2af2d0
    Lifted::new(0x700099b2e0, fn_700099b2e0 as *const (), 1), // +0x2af2e0 sub_2af2e0
    Lifted::new(0x700099b340, tpl_39::<0x40> as *const (), 1), // +0x2af340 sub_2af340
    Lifted::new(0x700099b350, fn_700099b350 as *const (), 0), // +0x2af350 sub_2af350
    Lifted::new(0x700099b3c0, fn_700099b3c0 as *const (), 1), // +0x2af3c0 sub_2af3c0
    Lifted::new(0x700099b4f0, tpl_52::<0x158> as *const (), 1), // +0x2af4f0 sub_2af4f0
    Lifted::new(0x700099b500, fn_700099b500 as *const (), 0), // +0x2af500 sub_2af500
    Lifted::new(0x700099b5a0, fn_700099b5a0 as *const (), 0), // +0x2af5a0 sub_2af5a0
    Lifted::new(0x700099b610, fn_700099b610 as *const (), 1), // +0x2af610 sub_2af610
    Lifted::new(0x700099b640, fn_700099b640 as *const (), 1), // +0x2af640 sub_2af640
    Lifted::new(0x700099b660, tpl_189::<0x700182d310> as *const (), 1), // +0x2af660 sub_2af660
    Lifted::new(0x700099b680, fn_700099b680 as *const (), 0), // +0x2af680 sub_2af680
    Lifted::new(0x700099b7f0, tpl_143::<0x9760, 0x700074eb60> as *const (), 0), // +0x2af7f0 sub_2af7f0
    Lifted::new(0x700099b800, tpl_143::<0x9760, 0x700074eb60> as *const (), 0), // +0x2af800 sub_2af800
    Lifted::new(0x700099b810, tpl_190::<0x700075e0c0, 0x8> as *const (), 0), // +0x2af810 sub_2af810
    Lifted::new(0x700099b830, fn_700099b830 as *const (), 1), // +0x2af830 sub_2af830
    Lifted::new(0x700099b880, tpl_191::<0x30> as *const (), 1), // +0x2af880 sub_2af880
    Lifted::new(0x700099b890, fn_700099b890 as *const (), 0), // +0x2af890 sub_2af890
    Lifted::new(0x700099b950, fn_700099b950 as *const (), 1), // +0x2af950 sub_2af950
    Lifted::new(0x700099b990, fn_700099b990 as *const (), 0), // +0x2af990 sub_2af990
    Lifted::new(0x700099ba10, fn_700099ba10 as *const (), 2), // +0x2afa10 sub_2afa10
    Lifted::new(0x700099bda0, fn_700099bda0 as *const (), 1), // +0x2afda0 sub_2afda0
    Lifted::new(0x700099bde0, fn_700099bde0 as *const (), 1), // +0x2afde0 sub_2afde0
    Lifted::new(0x700099be40, fn_700099be40 as *const (), 0), // +0x2afe40 sub_2afe40
    Lifted::new(0x700099bed0, fn_700099bed0 as *const (), 3), // +0x2afed0 sub_2afed0
    Lifted::new(0x700099bf00, fn_700099bf00 as *const (), 1), // +0x2aff00 sub_2aff00
    Lifted::new(0x700099bf20, fn_700099bf20 as *const (), 0), // +0x2aff20 sub_2aff20
    Lifted::new(0x700099bfd0, fn_700099bfd0 as *const (), 1), // +0x2affd0 sub_2affd0
    Lifted::new(0x700099c020, fn_700099c020 as *const (), 1), // +0x2b0020 sub_2b0020
    Lifted::new(0x700099c030, fn_700099c030 as *const (), 1), // +0x2b0030 sub_2b0030
    Lifted::new(0x700099c040, tpl_36::<0x8> as *const (), 1), // +0x2b0040 sub_2b0040
    Lifted::new(0x700099c050, tpl_192::<0x8> as *const (), 3), // +0x2b0050 sub_2b0050
    Lifted::new(0x700099c060, tpl_15 as *const (), 1), // +0x2b0060 sub_2b0060
    Lifted::new(0x700099c070, fn_700099c070 as *const (), 0), // +0x2b0070 sub_2b0070
    Lifted::new(0x700099c0b0, fn_700099c0b0 as *const (), 0), // +0x2b00b0 sub_2b00b0
    Lifted::new(0x700099c180, fn_700099c180 as *const (), 0), // +0x2b0180 sub_2b0180
    Lifted::new(0x700099c1c0, fn_700099c1c0 as *const (), 1), // +0x2b01c0 sub_2b01c0
    Lifted::new(0x700099c1d0, tpl_172::<0xd9d8> as *const (), 1), // +0x2b01d0 sub_2b01d0
    Lifted::new(0x700099c1e0, tpl_193::<0xd918> as *const (), 1), // +0x2b01e0 sub_2b01e0
    Lifted::new(0x700099c1f0, tpl_194 as *const (), 1), // +0x2b01f0 sub_2b01f0
    Lifted::new(0x700099c200, fn_700099c200 as *const (), 1), // +0x2b0200 sub_2b0200
    Lifted::new(0x700099c210, tpl_195::<0x44> as *const (), 1), // +0x2b0210 sub_2b0210
    Lifted::new(0x700099c220, fn_700099c220 as *const (), 1), // +0x2b0220 sub_2b0220
    Lifted::new(0x700099c250, tpl_172::<0xc0> as *const (), 1), // +0x2b0250 sub_2b0250
    Lifted::new(0x700099c260, fn_700099c260 as *const (), 0), // +0x2b0260 sub_2b0260
    Lifted::new(0x700099c330, tpl_146::<0x17> as *const (), 1), // +0x2b0330 sub_2b0330
    Lifted::new(0x700099c340, tpl_146::<0x17> as *const (), 1), // +0x2b0340 sub_2b0340
    Lifted::new(0x700099c350, tpl_146::<0x7> as *const (), 1), // +0x2b0350 sub_2b0350
    Lifted::new(0x700099c360, tpl_147::<0x3> as *const (), 1), // +0x2b0360 sub_2b0360
    Lifted::new(0x700099c370, fn_700099c370 as *const (), 1), // +0x2b0370 sub_2b0370
    Lifted::new(0x700099c390, tpl_196 as *const (), 1), // +0x2b0390 sub_2b0390
    Lifted::new(0x700099c3a0, tpl_197 as *const (), 1), // +0x2b03a0 sub_2b03a0
    Lifted::new(0x700099c3b0, tpl_197 as *const (), 1), // +0x2b03b0 sub_2b03b0
    Lifted::new(0x700099c3c0, tpl_43 as *const (), 1), // +0x2b03c0 sub_2b03c0
    Lifted::new(0x700099c3d0, tpl_198 as *const (), 1), // +0x2b03d0 sub_2b03d0
    Lifted::new(0x700099c3e0, tpl_198 as *const (), 1), // +0x2b03e0 sub_2b03e0
    Lifted::new(0x700099c3f0, fn_700099c3f0 as *const (), 1), // +0x2b03f0 sub_2b03f0
    Lifted::new(0x700099c400, tpl_199::<0xf> as *const (), 1), // +0x2b0400 sub_2b0400
    Lifted::new(0x700099c410, tpl_199::<0xf> as *const (), 1), // +0x2b0410 sub_2b0410
    Lifted::new(0x700099c420, tpl_199::<0x13> as *const (), 1), // +0x2b0420 sub_2b0420
    Lifted::new(0x700099c430, tpl_200::<0xf, 0x14, 0xffffffffffffffff> as *const (), 1), // +0x2b0430 sub_2b0430
    Lifted::new(0x700099c480, tpl_200::<0xf, 0x14, 0xffffffffffffffff> as *const (), 1), // +0x2b0480 sub_2b0480
    Lifted::new(0x700099c4d0, tpl_200::<0x13, 0x18, 0xffffffffffffffff> as *const (), 1), // +0x2b04d0 sub_2b04d0
    Lifted::new(0x700099c520, fn_700099c520 as *const (), 1), // +0x2b0520 sub_2b0520
    Lifted::new(0x700099c530, tpl_201::<0x130> as *const (), 1), // +0x2b0530 sub_2b0530
    Lifted::new(0x700099c540, tpl_138::<0x120, 0x128> as *const (), 1), // +0x2b0540 sub_2b0540
    Lifted::new(0x700099c580, fn_700099c580 as *const (), 1), // +0x2b0580 sub_2b0580
    Lifted::new(0x700099c5b0, fn_700099c5b0 as *const (), 0), // +0x2b05b0 sub_2b05b0
    Lifted::new(0x700099c630, tpl_202::<0x40> as *const (), 1), // +0x2b0630 sub_2b0630
    Lifted::new(0x700099c640, tpl_202::<0x44> as *const (), 1), // +0x2b0640 sub_2b0640
    Lifted::new(0x700099c650, fn_700099c650 as *const (), 1), // +0x2b0650 sub_2b0650
    Lifted::new(0x700099c660, fn_700099c660 as *const (), 1), // +0x2b0660 sub_2b0660
    Lifted::new(0x700099c670, tpl_203::<0xfffffffffffffffd> as *const (), 1), // +0x2b0670 sub_2b0670
    Lifted::new(0x700099c690, tpl_204::<0xfffffffffffffffb> as *const (), 1), // +0x2b0690 sub_2b0690
    Lifted::new(0x700099c6a0, fn_700099c6a0 as *const (), 1), // +0x2b06a0 sub_2b06a0
    Lifted::new(0x700099c6c0, tpl_204::<0xfffffffffffffffd> as *const (), 1), // +0x2b06c0 sub_2b06c0
    Lifted::new(0x700099c6d0, tpl_203::<0xfffffffffffffffd> as *const (), 1), // +0x2b06d0 sub_2b06d0
    Lifted::new(0x700099c6f0, fn_700099c6f0 as *const (), 0), // +0x2b06f0 sub_2b06f0
    Lifted::new(0x700099c720, tpl_36::<0x228> as *const (), 1), // +0x2b0720 sub_2b0720
    Lifted::new(0x700099c730, fn_700099c730 as *const (), 0), // +0x2b0730 sub_2b0730
    Lifted::new(0x700099c770, tpl_142::<0x7d8> as *const (), 1), // +0x2b0770 sub_2b0770
    Lifted::new(0x700099c790, tpl_142::<0x7e0> as *const (), 1), // +0x2b0790 sub_2b0790
    Lifted::new(0x700099c7b0, tpl_142::<0x7e8> as *const (), 1), // +0x2b07b0 sub_2b07b0
    Lifted::new(0x700099c7d0, fn_700099c7d0 as *const (), 0), // +0x2b07d0 sub_2b07d0
    Lifted::new(0x700099c850, fn_700099c850 as *const (), 0), // +0x2b0850 sub_2b0850
    Lifted::new(0x700099c8c0, fn_700099c8c0 as *const (), 2), // +0x2b08c0 sub_2b08c0
    Lifted::new(0x700099c910, fn_700099c910 as *const (), 0), // +0x2b0910 sub_2b0910
    Lifted::new(0x700099c960, fn_700099c960 as *const (), 1), // +0x2b0960 sub_2b0960
    Lifted::new(0x700099c990, fn_700099c990 as *const (), 1), // +0x2b0990 sub_2b0990
    Lifted::new(0x700099c9c0, fn_700099c9c0 as *const (), 1), // +0x2b09c0 sub_2b09c0
    Lifted::new(0x700099c9f0, fn_700099c9f0 as *const (), 1), // +0x2b09f0 sub_2b09f0
    Lifted::new(0x700099ca00, tpl_169::<0x30> as *const (), 1), // +0x2b0a00 sub_2b0a00
    Lifted::new(0x700099ca10, tpl_205::<0x7000746980> as *const (), 0), // +0x2b0a10 sub_2b0a10
    Lifted::new(0x700099ca20, fn_700099ca20 as *const (), 1), // +0x2b0a20 sub_2b0a20
    Lifted::new(0x700099ca40, tpl_206::<0x220, 0x218> as *const (), 1), // +0x2b0a40 sub_2b0a40
    Lifted::new(0x700099ca60, fn_700099ca60 as *const (), 0), // +0x2b0a60 sub_2b0a60
    Lifted::new(0x700099ccb0, fn_700099ccb0 as *const (), 1), // +0x2b0cb0 sub_2b0cb0
    Lifted::new(0x700099ccd0, fn_700099ccd0 as *const (), 1), // +0x2b0cd0 sub_2b0cd0
    Lifted::new(0x700099ccf0, fn_700099ccf0 as *const (), 0), // +0x2b0cf0 sub_2b0cf0
    Lifted::new(0x700099cd20, fn_700099cd20 as *const (), 1), // +0x2b0d20 sub_2b0d20
    Lifted::new(0x700099cd40, fn_700099cd40 as *const (), 1), // +0x2b0d40 sub_2b0d40
    Lifted::new(0x700099cd50, fn_700099cd50 as *const (), 1), // +0x2b0d50 sub_2b0d50
    Lifted::new(0x700099cd60, fn_700099cd60 as *const (), 1), // +0x2b0d60 sub_2b0d60
    Lifted::new(0x700099cd70, fn_700099cd70 as *const (), 1), // +0x2b0d70 sub_2b0d70
    Lifted::new(0x700099cd80, fn_700099cd80 as *const (), 1), // +0x2b0d80 sub_2b0d80
    Lifted::new(0x700099cd90, fn_700099cd90 as *const (), 1), // +0x2b0d90 sub_2b0d90
    Lifted::new(0x700099cda0, tpl_47::<0xc7e2> as *const (), 1), // +0x2b0da0 sub_2b0da0
    Lifted::new(0x700099cdb0, fn_700099cdb0 as *const (), 1), // +0x2b0db0 sub_2b0db0
    Lifted::new(0x700099cdc0, tpl_44 as *const (), 1), // +0x2b0dc0 sub_2b0dc0
    Lifted::new(0x700099cdd0, fn_700099cdd0 as *const (), 1), // +0x2b0dd0 sub_2b0dd0
    Lifted::new(0x700099ce30, fn_700099ce30 as *const (), 1), // +0x2b0e30 sub_2b0e30
    Lifted::new(0x700099ce60, tpl_207 as *const (), 1), // +0x2b0e60 sub_2b0e60
    Lifted::new(0x700099ce70, tpl_208::<0x8> as *const (), 1), // +0x2b0e70 sub_2b0e70
    Lifted::new(0x700099ce80, tpl_208::<0xc0> as *const (), 1), // +0x2b0e80 sub_2b0e80
    Lifted::new(0x700099ce90, fn_700099ce90 as *const (), 1), // +0x2b0e90 sub_2b0e90
    Lifted::new(0x700099cea0, fn_700099cea0 as *const (), 1), // +0x2b0ea0 sub_2b0ea0
    Lifted::new(0x700099ceb0, fn_700099ceb0 as *const (), 1), // +0x2b0eb0 sub_2b0eb0
    Lifted::new(0x700099cec0, fn_700099cec0 as *const (), 1), // +0x2b0ec0 sub_2b0ec0
    Lifted::new(0x700099ced0, fn_700099ced0 as *const (), 1), // +0x2b0ed0 sub_2b0ed0
    Lifted::new(0x700099cee0, fn_700099cee0 as *const (), 1), // +0x2b0ee0 sub_2b0ee0
    Lifted::new(0x700099cef0, fn_700099cef0 as *const (), 1), // +0x2b0ef0 sub_2b0ef0
    Lifted::new(0x700099cf10, fn_700099cf10 as *const (), 1), // +0x2b0f10 sub_2b0f10
    Lifted::new(0x700099cf20, fn_700099cf20 as *const (), 1), // +0x2b0f20 sub_2b0f20
    Lifted::new(0x700099cf30, fn_700099cf30 as *const (), 1), // +0x2b0f30 sub_2b0f30
    Lifted::new(0x700099cf40, fn_700099cf40 as *const (), 1), // +0x2b0f40 sub_2b0f40
    Lifted::new(0x700099cf50, fn_700099cf50 as *const (), 1), // +0x2b0f50 sub_2b0f50
    Lifted::new(0x700099cf60, tpl_172::<0xd840> as *const (), 1), // +0x2b0f60 sub_2b0f60
    Lifted::new(0x700099cf70, fn_700099cf70 as *const (), 0), // +0x2b0f70 sub_2b0f70
    Lifted::new(0x700099cf90, tpl_172::<0xa590> as *const (), 1), // +0x2b0f90 sub_2b0f90
    Lifted::new(0x700099cfa0, tpl_47::<0xc7e5> as *const (), 1), // +0x2b0fa0 sub_2b0fa0
    Lifted::new(0x700099cfb0, fn_700099cfb0 as *const (), 1), // +0x2b0fb0 sub_2b0fb0
    Lifted::new(0x700099cfc0, fn_700099cfc0 as *const (), 1), // +0x2b0fc0 sub_2b0fc0
    Lifted::new(0x700099cfd0, fn_700099cfd0 as *const (), 1), // +0x2b0fd0 sub_2b0fd0
    Lifted::new(0x700099cfe0, fn_700099cfe0 as *const (), 1), // +0x2b0fe0 sub_2b0fe0
    Lifted::new(0x700099d000, fn_700099d000 as *const (), 1), // +0x2b1000 sub_2b1000
    Lifted::new(0x700099d020, tpl_209::<0x20, 0x28> as *const (), 1), // +0x2b1020 sub_2b1020
    Lifted::new(0x700099d040, tpl_209::<0x20, 0x28> as *const (), 1), // +0x2b1040 sub_2b1040
    Lifted::new(0x700099d060, tpl_209::<0x20, 0x28> as *const (), 1), // +0x2b1060 sub_2b1060
    Lifted::new(0x700099d080, tpl_209::<0x20, 0x28> as *const (), 1), // +0x2b1080 sub_2b1080
    Lifted::new(0x700099d0a0, tpl_209::<0x20, 0x28> as *const (), 1), // +0x2b10a0 sub_2b10a0
    Lifted::new(0x700099d0c0, tpl_210::<0x28> as *const (), 1), // +0x2b10c0 sub_2b10c0
    Lifted::new(0x700099d0d0, tpl_210::<0x28> as *const (), 1), // +0x2b10d0 sub_2b10d0
    Lifted::new(0x700099d0e0, tpl_210::<0x28> as *const (), 1), // +0x2b10e0 sub_2b10e0
    Lifted::new(0x700099d0f0, tpl_210::<0x28> as *const (), 1), // +0x2b10f0 sub_2b10f0
    Lifted::new(0x700099d100, fn_700099d100 as *const (), 1), // +0x2b1100 sub_2b1100
    Lifted::new(0x700099d120, fn_700099d120 as *const (), 1), // +0x2b1120 sub_2b1120
    Lifted::new(0x700099d130, fn_700099d130 as *const (), 1), // +0x2b1130 sub_2b1130
    Lifted::new(0x700099d140, fn_700099d140 as *const (), 1), // +0x2b1140 sub_2b1140
    Lifted::new(0x700099d150, fn_700099d150 as *const (), 0), // +0x2b1150 sub_2b1150
    Lifted::new(0x700099d170, tpl_47::<0xc7e6> as *const (), 1), // +0x2b1170 sub_2b1170
    Lifted::new(0x700099d180, tpl_211::<0x7> as *const (), 1), // +0x2b1180 sub_2b1180
    Lifted::new(0x700099d190, tpl_211::<0x7> as *const (), 1), // +0x2b1190 sub_2b1190
    Lifted::new(0x700099d1a0, tpl_211::<0xb> as *const (), 1), // +0x2b11a0 sub_2b11a0
    Lifted::new(0x700099d1b0, tpl_212::<0x70009889e0, 0x7001d550a8> as *const (), 2), // +0x2b11b0 sub_2b11b0
    Lifted::new(0x700099d220, tpl_47::<0x58> as *const (), 1), // +0x2b1220 sub_2b1220
    Lifted::new(0x700099d230, fn_700099d230 as *const (), 1), // +0x2b1230 sub_2b1230
    Lifted::new(0x700099d240, tpl_213::<0x10> as *const (), 1), // +0x2b1240 sub_2b1240
    Lifted::new(0x700099d250, fn_700099d250 as *const (), 1), // +0x2b1250 sub_2b1250
    Lifted::new(0x700099d270, fn_700099d270 as *const (), 1), // +0x2b1270 sub_2b1270
    Lifted::new(0x700099d290, tpl_47::<0x4c> as *const (), 1), // +0x2b1290 sub_2b1290
    Lifted::new(0x700099d2a0, fn_700099d2a0 as *const (), 1), // +0x2b12a0 sub_2b12a0
    Lifted::new(0x700099d2b0, fn_700099d2b0 as *const (), 1), // +0x2b12b0 sub_2b12b0
    Lifted::new(0x700099d2c0, fn_700099d2c0 as *const (), 1), // +0x2b12c0 sub_2b12c0
    Lifted::new(0x700099d2e0, fn_700099d2e0 as *const (), 1), // +0x2b12e0 sub_2b12e0
    Lifted::new(0x700099d2f0, fn_700099d2f0 as *const (), 0), // +0x2b12f0 sub_2b12f0
    Lifted::new(0x700099d310, tpl_47::<0x60> as *const (), 1), // +0x2b1310 sub_2b1310
    Lifted::new(0x700099d320, fn_700099d320 as *const (), 1), // +0x2b1320 sub_2b1320
    Lifted::new(0x700099d340, fn_700099d340 as *const (), 1), // +0x2b1340 sub_2b1340
    Lifted::new(0x700099d390, tpl_214::<0x30> as *const (), 1), // +0x2b1390 sub_2b1390
    Lifted::new(0x700099d3a0, tpl_215::<0x7, 0xb, 0x8> as *const (), 1), // +0x2b13a0 sub_2b13a0
    Lifted::new(0x700099d3d0, tpl_215::<0x7, 0xb, 0x8> as *const (), 1), // +0x2b13d0 sub_2b13d0
    Lifted::new(0x700099d400, tpl_215::<0xb, 0xf, 0x8> as *const (), 1), // +0x2b1400 sub_2b1400
    Lifted::new(0x700099d430, fn_700099d430 as *const (), 1), // +0x2b1430 sub_2b1430
    Lifted::new(0x700099d450, tpl_216::<0x7000781230> as *const (), 0), // +0x2b1450 sub_2b1450
    Lifted::new(0x700099d470, fn_700099d470 as *const (), 0), // +0x2b1470 sub_2b1470
    Lifted::new(0x700099d550, tpl_217::<0x48, 0x18, 0x7, 0x10> as *const (), 0), // +0x2b1550 sub_2b1550
    Lifted::new(0x700099d590, tpl_217::<0x50, 0x18, 0x7, 0x10> as *const (), 0), // +0x2b1590 sub_2b1590
    Lifted::new(0x700099d5d0, tpl_15 as *const (), 1), // +0x2b15d0 sub_2b15d0
    Lifted::new(0x700099d5e0, tpl_218::<0x13> as *const (), 1), // +0x2b15e0 sub_2b15e0
    Lifted::new(0x700099d620, tpl_219::<0x13> as *const (), 1), // +0x2b1620 sub_2b1620
    Lifted::new(0x700099d660, tpl_220::<0x13> as *const (), 1), // +0x2b1660 sub_2b1660
    Lifted::new(0x700099d690, tpl_221::<0x13> as *const (), 1), // +0x2b1690 sub_2b1690
    Lifted::new(0x700099d6d0, tpl_149::<0x1c, 0xffffffffffffffff> as *const (), 1), // +0x2b16d0 sub_2b16d0
    Lifted::new(0x700099d710, tpl_222::<0x1c, 0xffffffffffffffff> as *const (), 1), // +0x2b1710 sub_2b1710
    Lifted::new(0x700099d750, tpl_150::<0x1c, 0xffffffffffffffff> as *const (), 1), // +0x2b1750 sub_2b1750
    Lifted::new(0x700099d780, tpl_223::<0x1c, 0xffffffffffffffff> as *const (), 1), // +0x2b1780 sub_2b1780
    Lifted::new(0x700099d7c0, tpl_224::<0x1b> as *const (), 1), // +0x2b17c0 sub_2b17c0
    Lifted::new(0x700099d800, tpl_225::<0x1b> as *const (), 1), // +0x2b1800 sub_2b1800
    Lifted::new(0x700099d840, tpl_226::<0x1b> as *const (), 1), // +0x2b1840 sub_2b1840
    Lifted::new(0x700099d870, tpl_227::<0x1b> as *const (), 1), // +0x2b1870 sub_2b1870
    Lifted::new(0x700099d8b0, tpl_149::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x2b18b0 sub_2b18b0
    Lifted::new(0x700099d8f0, tpl_222::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x2b18f0 sub_2b18f0
    Lifted::new(0x700099d930, tpl_150::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x2b1930 sub_2b1930
    Lifted::new(0x700099d960, tpl_223::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x2b1960 sub_2b1960
    Lifted::new(0x700099d9a0, tpl_218::<0x13> as *const (), 1), // +0x2b19a0 sub_2b19a0
    Lifted::new(0x700099d9e0, tpl_219::<0x13> as *const (), 1), // +0x2b19e0 sub_2b19e0
    Lifted::new(0x700099da20, tpl_220::<0x13> as *const (), 1), // +0x2b1a20 sub_2b1a20
    Lifted::new(0x700099da50, tpl_221::<0x13> as *const (), 1), // +0x2b1a50 sub_2b1a50
    Lifted::new(0x700099da90, fn_700099da90 as *const (), 1), // +0x2b1a90 sub_2b1a90
    Lifted::new(0x700099daf0, fn_700099daf0 as *const (), 1), // +0x2b1af0 sub_2b1af0
    Lifted::new(0x700099db50, tpl_228::<0x13, 0x18, 0xffffffffffffffff> as *const (), 1), // +0x2b1b50 sub_2b1b50
    Lifted::new(0x700099dbb0, tpl_229::<0x10, 0xffffffffffffffff> as *const (), 1), // +0x2b1bb0 sub_2b1bb0
    Lifted::new(0x700099dbf0, tpl_229::<0x10, 0xffffffffffffffff> as *const (), 1), // +0x2b1bf0 sub_2b1bf0
    Lifted::new(0x700099dc30, tpl_230 as *const (), 1), // +0x2b1c30 sub_2b1c30
    Lifted::new(0x700099dc40, tpl_142::<0x838> as *const (), 1), // +0x2b1c40 sub_2b1c40
    Lifted::new(0x700099dc60, fn_700099dc60 as *const (), 1), // +0x2b1c60 sub_2b1c60
    Lifted::new(0x700099dca0, fn_700099dca0 as *const (), 1), // +0x2b1ca0 sub_2b1ca0
    Lifted::new(0x700099dcd0, tpl_36::<0x8> as *const (), 1), // +0x2b1cd0 sub_2b1cd0
    Lifted::new(0x700099dce0, fn_700099dce0 as *const (), 1), // +0x2b1ce0 sub_2b1ce0
    Lifted::new(0x700099dcf0, fn_700099dcf0 as *const (), 2), // +0x2b1cf0 sub_2b1cf0
    Lifted::new(0x700099dd60, fn_700099dd60 as *const (), 0), // +0x2b1d60 sub_2b1d60
    Lifted::new(0x700099dd90, tpl_142::<0x850> as *const (), 1), // +0x2b1d90 sub_2b1d90
    Lifted::new(0x700099ddb0, tpl_142::<0x848> as *const (), 1), // +0x2b1db0 sub_2b1db0
    Lifted::new(0x700099ddd0, fn_700099ddd0 as *const (), 1), // +0x2b1dd0 sub_2b1dd0
    Lifted::new(0x700099de00, tpl_15 as *const (), 1), // +0x2b1e00 sub_2b1e00
    Lifted::new(0x700099de10, fn_700099de10 as *const (), 2), // +0x2b1e10 sub_2b1e10
    Lifted::new(0x700099df50, tpl_231::<0x8, 0x10, 0x70007154a0> as *const (), 0), // +0x2b1f50 sub_2b1f50
    Lifted::new(0x700099df90, fn_700099df90 as *const (), 0), // +0x2b1f90 sub_2b1f90
    Lifted::new(0x700099dfc0, tpl_231::<0x8, 0x10, 0x7000767d50> as *const (), 0), // +0x2b1fc0 sub_2b1fc0
    Lifted::new(0x700099e000, fn_700099e000 as *const (), 0), // +0x2b2000 sub_2b2000
    Lifted::new(0x700099e050, fn_700099e050 as *const (), 0), // +0x2b2050 sub_2b2050
    Lifted::new(0x700099e090, fn_700099e090 as *const (), 0), // +0x2b2090 sub_2b2090
    Lifted::new(0x700099e0d0, fn_700099e0d0 as *const (), 1), // +0x2b20d0 sub_2b20d0
    Lifted::new(0x700099e0e0, fn_700099e0e0 as *const (), 1), // +0x2b20e0 sub_2b20e0
    Lifted::new(0x700099e0f0, fn_700099e0f0 as *const (), 1), // +0x2b20f0 sub_2b20f0
    Lifted::new(0x700099e100, fn_700099e100 as *const (), 1), // +0x2b2100 sub_2b2100
    Lifted::new(0x700099e110, fn_700099e110 as *const (), 1), // +0x2b2110 sub_2b2110
    Lifted::new(0x700099e130, fn_700099e130 as *const (), 1), // +0x2b2130 sub_2b2130
    Lifted::new(0x700099e150, tpl_36::<0x220> as *const (), 1), // +0x2b2150 sub_2b2150
    Lifted::new(0x700099e160, tpl_36::<0x20> as *const (), 1), // +0x2b2160 sub_2b2160
    Lifted::new(0x700099e170, fn_700099e170 as *const (), 0), // +0x2b2170 sub_2b2170
    Lifted::new(0x700099e1c0, tpl_232::<0x8, 0x10, 0xffffffffffff7498> as *const (), 1), // +0x2b21c0 sub_2b21c0
    Lifted::new(0x700099e200, tpl_232::<0x8, 0x10, 0xffffffffffff74a0> as *const (), 1), // +0x2b2200 sub_2b2200
    Lifted::new(0x700099e240, fn_700099e240 as *const (), 0), // +0x2b2240 sub_2b2240
    Lifted::new(0x700099e380, tpl_233 as *const (), 1), // +0x2b2380 sub_2b2380
    Lifted::new(0x700099e390, fn_700099e390 as *const (), 1), // +0x2b2390 sub_2b2390
    Lifted::new(0x700099e3e0, fn_700099e3e0 as *const (), 1), // +0x2b23e0 sub_2b23e0
    Lifted::new(0x700099e410, tpl_142::<0x880> as *const (), 1), // +0x2b2410 sub_2b2410
    Lifted::new(0x700099e430, fn_700099e430 as *const (), 1), // +0x2b2430 sub_2b2430
    Lifted::new(0x700099e470, fn_700099e470 as *const (), 0), // +0x2b2470 sub_2b2470
    Lifted::new(0x700099e4d0, fn_700099e4d0 as *const (), 0), // +0x2b24d0 sub_2b24d0
    Lifted::new(0x700099e520, fn_700099e520 as *const (), 0), // +0x2b2520 sub_2b2520
    Lifted::new(0x700099e570, tpl_142::<0x8c0> as *const (), 1), // +0x2b2570 sub_2b2570
    Lifted::new(0x700099e590, tpl_142::<0x8c8> as *const (), 1), // +0x2b2590 sub_2b2590
    Lifted::new(0x700099e5b0, tpl_142::<0x8d0> as *const (), 1), // +0x2b25b0 sub_2b25b0
    Lifted::new(0x700099e5d0, tpl_142::<0x8d8> as *const (), 1), // +0x2b25d0 sub_2b25d0
    Lifted::new(0x700099e5f0, tpl_142::<0x898> as *const (), 1), // +0x2b25f0 sub_2b25f0
    Lifted::new(0x700099e610, tpl_234::<0x1c, 0xffffffffffffffff, 0x3> as *const (), 1), // +0x2b2610 sub_2b2610
    Lifted::new(0x700099e650, tpl_235::<0x1c, 0xffffffffffffffff, 0x3> as *const (), 1), // +0x2b2650 sub_2b2650
    Lifted::new(0x700099e680, tpl_224::<0x1b> as *const (), 1), // +0x2b2680 sub_2b2680
    Lifted::new(0x700099e6c0, tpl_226::<0x1b> as *const (), 1), // +0x2b26c0 sub_2b26c0
    Lifted::new(0x700099e6f0, tpl_228::<0x13, 0x18, 0xffffffffffffffff> as *const (), 1), // +0x2b26f0 sub_2b26f0
    Lifted::new(0x700099e750, tpl_229::<0x10, 0xffffffffffffffff> as *const (), 1), // +0x2b2750 sub_2b2750
    Lifted::new(0x700099e790, tpl_15 as *const (), 1), // +0x2b2790 sub_2b2790
    Lifted::new(0x700099e7a0, fn_700099e7a0 as *const (), 1), // +0x2b27a0 sub_2b27a0
    Lifted::new(0x700099e7d0, fn_700099e7d0 as *const (), 0), // +0x2b27d0 sub_2b27d0
    Lifted::new(0x700099e860, tpl_216::<0x700078a940> as *const (), 0), // +0x2b2860 sub_2b2860
    Lifted::new(0x700099e880, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x700099e891> as *const (), 0), // +0x2b2880 sub_2b2880
    Lifted::new(0x700099e8a0, fn_700099e8a0 as *const (), 0), // +0x2b28a0 sub_2b28a0
    Lifted::new(0x700099e930, fn_700099e930 as *const (), 0), // +0x2b2930 sub_2b2930
    Lifted::new(0x700099e970, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x700099e981> as *const (), 0), // +0x2b2970 sub_2b2970
    Lifted::new(0x700099e990, fn_700099e990 as *const (), 0), // +0x2b2990 sub_2b2990
    Lifted::new(0x700099ea10, tpl_236::<0x700078c0b0, 0x3, 0x20> as *const (), 0), // +0x2b2a10 sub_2b2a10
    Lifted::new(0x700099eab0, tpl_236::<0x70006faef0, 0x3, 0x20> as *const (), 0), // +0x2b2ab0 sub_2b2ab0
    Lifted::new(0x700099eb50, fn_700099eb50 as *const (), 0), // +0x2b2b50 sub_2b2b50
    Lifted::new(0x700099eba0, fn_700099eba0 as *const (), 0), // +0x2b2ba0 sub_2b2ba0
    Lifted::new(0x700099ecb0, fn_700099ecb0 as *const (), 0), // +0x2b2cb0 sub_2b2cb0
    Lifted::new(0x700099edc0, fn_700099edc0 as *const (), 1), // +0x2b2dc0 sub_2b2dc0
    Lifted::new(0x700099ede0, fn_700099ede0 as *const (), 1), // +0x2b2de0 sub_2b2de0
    Lifted::new(0x700099ee30, fn_700099ee30 as *const (), 1), // +0x2b2e30 sub_2b2e30
    Lifted::new(0x700099ee80, fn_700099ee80 as *const (), 1), // +0x2b2e80 sub_2b2e80
    Lifted::new(0x700099eed0, fn_700099eed0 as *const (), 0), // +0x2b2ed0 sub_2b2ed0
    Lifted::new(0x700099ef20, fn_700099ef20 as *const (), 1), // +0x2b2f20 sub_2b2f20
    Lifted::new(0x700099ef60, fn_700099ef60 as *const (), 1), // +0x2b2f60 sub_2b2f60
    Lifted::new(0x700099ef80, tpl_237::<0x7> as *const (), 1), // +0x2b2f80 sub_2b2f80
    Lifted::new(0x700099efb0, tpl_237::<0x7> as *const (), 1), // +0x2b2fb0 sub_2b2fb0
    Lifted::new(0x700099efe0, tpl_237::<0xb> as *const (), 1), // +0x2b2fe0 sub_2b2fe0
    Lifted::new(0x700099f010, tpl_196 as *const (), 1), // +0x2b3010 sub_2b3010
    Lifted::new(0x700099f020, tpl_196 as *const (), 1), // +0x2b3020 sub_2b3020
    Lifted::new(0x700099f030, tpl_53 as *const (), 1), // +0x2b3030 sub_2b3030
    Lifted::new(0x700099f040, tpl_148 as *const (), 1), // +0x2b3040 sub_2b3040
    Lifted::new(0x700099f050, tpl_148 as *const (), 1), // +0x2b3050 sub_2b3050
    Lifted::new(0x700099f060, tpl_233 as *const (), 1), // +0x2b3060 sub_2b3060
    Lifted::new(0x700099f070, fn_700099f070 as *const (), 1), // +0x2b3070 sub_2b3070
    Lifted::new(0x700099f080, tpl_15 as *const (), 1), // +0x2b3080 sub_2b3080
    Lifted::new(0x700099f090, tpl_146::<0xf> as *const (), 1), // +0x2b3090 sub_2b3090
    Lifted::new(0x700099f0a0, tpl_146::<0xf> as *const (), 1), // +0x2b30a0 sub_2b30a0
    Lifted::new(0x700099f0b0, tpl_146::<0x13> as *const (), 1), // +0x2b30b0 sub_2b30b0
    Lifted::new(0x700099f0c0, tpl_238 as *const (), 1), // +0x2b30c0 sub_2b30c0
    Lifted::new(0x700099f0d0, tpl_238 as *const (), 1), // +0x2b30d0 sub_2b30d0
    Lifted::new(0x700099f0e0, tpl_197 as *const (), 1), // +0x2b30e0 sub_2b30e0
    Lifted::new(0x700099f0f0, tpl_164 as *const (), 1), // +0x2b30f0 sub_2b30f0
    Lifted::new(0x700099f100, tpl_164 as *const (), 1), // +0x2b3100 sub_2b3100
    Lifted::new(0x700099f110, tpl_198 as *const (), 1), // +0x2b3110 sub_2b3110
    Lifted::new(0x700099f120, tpl_146::<0xb> as *const (), 1), // +0x2b3120 sub_2b3120
    Lifted::new(0x700099f130, tpl_146::<0xb> as *const (), 1), // +0x2b3130 sub_2b3130
    Lifted::new(0x700099f140, tpl_146::<0xf> as *const (), 1), // +0x2b3140 sub_2b3140
    Lifted::new(0x700099f150, tpl_146::<0xb> as *const (), 1), // +0x2b3150 sub_2b3150
    Lifted::new(0x700099f160, fn_700099f160 as *const (), 1), // +0x2b3160 sub_2b3160
    Lifted::new(0x700099f1b0, tpl_53 as *const (), 1), // +0x2b31b0 sub_2b31b0
    Lifted::new(0x700099f1c0, tpl_53 as *const (), 1), // +0x2b31c0 sub_2b31c0
    Lifted::new(0x700099f1d0, tpl_238 as *const (), 1), // +0x2b31d0 sub_2b31d0
    Lifted::new(0x700099f1e0, tpl_233 as *const (), 1), // +0x2b31e0 sub_2b31e0
    Lifted::new(0x700099f1f0, tpl_233 as *const (), 1), // +0x2b31f0 sub_2b31f0
    Lifted::new(0x700099f200, tpl_164 as *const (), 1), // +0x2b3200 sub_2b3200
    Lifted::new(0x700099f210, tpl_146::<0x7> as *const (), 1), // +0x2b3210 sub_2b3210
    Lifted::new(0x700099f220, tpl_146::<0x7> as *const (), 1), // +0x2b3220 sub_2b3220
    Lifted::new(0x700099f230, tpl_146::<0xb> as *const (), 1), // +0x2b3230 sub_2b3230
    Lifted::new(0x700099f240, tpl_146::<0x7> as *const (), 1), // +0x2b3240 sub_2b3240
    Lifted::new(0x700099f250, fn_700099f250 as *const (), 1), // +0x2b3250 sub_2b3250
    Lifted::new(0x700099f2a0, tpl_196 as *const (), 1), // +0x2b32a0 sub_2b32a0
    Lifted::new(0x700099f2b0, tpl_196 as *const (), 1), // +0x2b32b0 sub_2b32b0
    Lifted::new(0x700099f2c0, tpl_53 as *const (), 1), // +0x2b32c0 sub_2b32c0
    Lifted::new(0x700099f2d0, tpl_148 as *const (), 1), // +0x2b32d0 sub_2b32d0
    Lifted::new(0x700099f2e0, tpl_148 as *const (), 1), // +0x2b32e0 sub_2b32e0
    Lifted::new(0x700099f2f0, tpl_233 as *const (), 1), // +0x2b32f0 sub_2b32f0
    Lifted::new(0x700099f300, fn_700099f300 as *const (), 1), // +0x2b3300 sub_2b3300
    Lifted::new(0x700099f320, tpl_142::<0x8f8> as *const (), 1), // +0x2b3320 sub_2b3320
    Lifted::new(0x700099f340, tpl_142::<0x910> as *const (), 1), // +0x2b3340 sub_2b3340
    Lifted::new(0x700099f360, fn_700099f360 as *const (), 1), // +0x2b3360 sub_2b3360
    Lifted::new(0x700099f370, fn_700099f370 as *const (), 1), // +0x2b3370 sub_2b3370
    Lifted::new(0x700099f3a0, fn_700099f3a0 as *const (), 1), // +0x2b33a0 sub_2b33a0
    Lifted::new(0x700099f3c0, fn_700099f3c0 as *const (), 1), // +0x2b33c0 sub_2b33c0
    Lifted::new(0x700099f3e0, fn_700099f3e0 as *const (), 1), // +0x2b33e0 sub_2b33e0
    Lifted::new(0x700099f3f0, fn_700099f3f0 as *const (), 0), // +0x2b33f0 sub_2b33f0
    Lifted::new(0x700099f430, tpl_39::<0xe8> as *const (), 1), // +0x2b3430 sub_2b3430
    Lifted::new(0x700099f440, fn_700099f440 as *const (), 0), // +0x2b3440 sub_2b3440
    Lifted::new(0x700099f460, fn_700099f460 as *const (), 1), // +0x2b3460 sub_2b3460
    Lifted::new(0x700099f480, fn_700099f480 as *const (), 0), // +0x2b3480 sub_2b3480
    Lifted::new(0x700099f4a0, tpl_15 as *const (), 1), // +0x2b34a0 sub_2b34a0
    Lifted::new(0x700099f4b0, fn_700099f4b0 as *const (), 0), // +0x2b34b0 sub_2b34b0
    Lifted::new(0x700099f500, tpl_196 as *const (), 1), // +0x2b3500 sub_2b3500
    Lifted::new(0x700099f510, tpl_196 as *const (), 1), // +0x2b3510 sub_2b3510
    Lifted::new(0x700099f520, tpl_196 as *const (), 1), // +0x2b3520 sub_2b3520
    Lifted::new(0x700099f530, tpl_43 as *const (), 1), // +0x2b3530 sub_2b3530
    Lifted::new(0x700099f540, tpl_142::<0x970> as *const (), 1), // +0x2b3540 sub_2b3540
    Lifted::new(0x700099f560, fn_700099f560 as *const (), 1), // +0x2b3560 sub_2b3560
    Lifted::new(0x700099f580, fn_700099f580 as *const (), 1), // +0x2b3580 sub_2b3580
    Lifted::new(0x700099f590, tpl_142::<0x990> as *const (), 1), // +0x2b3590 sub_2b3590
    Lifted::new(0x700099f5b0, fn_700099f5b0 as *const (), 1), // +0x2b35b0 sub_2b35b0
    Lifted::new(0x700099f600, tpl_142::<0x9a0> as *const (), 1), // +0x2b3600 sub_2b3600
    Lifted::new(0x700099f620, fn_700099f620 as *const (), 1), // +0x2b3620 sub_2b3620
    Lifted::new(0x700099f640, tpl_239::<0x20> as *const (), 1), // +0x2b3640 sub_2b3640
    Lifted::new(0x700099f670, tpl_240::<0x20> as *const (), 1), // +0x2b3670 sub_2b3670
    Lifted::new(0x700099f6a0, fn_700099f6a0 as *const (), 1), // +0x2b36a0 sub_2b36a0
    Lifted::new(0x700099f720, tpl_241::<0x700182e5b0> as *const (), 1), // +0x2b3720 sub_2b3720
    Lifted::new(0x700099f740, tpl_142::<0x9b0> as *const (), 1), // +0x2b3740 sub_2b3740
    Lifted::new(0x700099f760, tpl_142::<0x9b8> as *const (), 1), // +0x2b3760 sub_2b3760
    Lifted::new(0x700099f780, tpl_142::<0x9c0> as *const (), 1), // +0x2b3780 sub_2b3780
    Lifted::new(0x700099f7a0, tpl_142::<0x9c8> as *const (), 1), // +0x2b37a0 sub_2b37a0
    Lifted::new(0x700099f7c0, tpl_15 as *const (), 1), // +0x2b37c0 sub_2b37c0
    Lifted::new(0x700099f7d0, tpl_15 as *const (), 1), // +0x2b37d0 sub_2b37d0
    Lifted::new(0x700099f7e0, tpl_51::<0xd960> as *const (), 1), // +0x2b37e0 sub_2b37e0
    Lifted::new(0x700099f7f0, fn_700099f7f0 as *const (), 1), // +0x2b37f0 sub_2b37f0
    Lifted::new(0x700099f800, fn_700099f800 as *const (), 1), // +0x2b3800 sub_2b3800
    Lifted::new(0x700099f810, fn_700099f810 as *const (), 0), // +0x2b3810 sub_2b3810
    Lifted::new(0x700099f8b0, fn_700099f8b0 as *const (), 0), // +0x2b38b0 sub_2b38b0
    Lifted::new(0x700099f900, tpl_141::<0x7001d550d0, 0x8, 0x700071acc0, 0xf8, 0x7001d550c8> as *const (), 0), // +0x2b3900 sub_2b3900
    Lifted::new(0x700099f950, tpl_197 as *const (), 1), // +0x2b3950 sub_2b3950
    Lifted::new(0x700099f960, tpl_197 as *const (), 1), // +0x2b3960 sub_2b3960
    Lifted::new(0x700099f970, tpl_43 as *const (), 1), // +0x2b3970 sub_2b3970
    Lifted::new(0x700099f980, tpl_242::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x2b3980 sub_2b3980
    Lifted::new(0x700099f9a0, tpl_242::<0x14, 0xffffffffffffffff> as *const (), 1), // +0x2b39a0 sub_2b39a0
    Lifted::new(0x700099f9c0, tpl_242::<0x18, 0xffffffffffffffff> as *const (), 1), // +0x2b39c0 sub_2b39c0
    Lifted::new(0x700099f9e0, fn_700099f9e0 as *const (), 1), // +0x2b39e0 sub_2b39e0
    Lifted::new(0x700099f9f0, tpl_151::<0x700076e890> as *const (), 0), // +0x2b39f0 sub_2b39f0
    Lifted::new(0x700099fa00, tpl_152::<0x700076e890> as *const (), 0), // +0x2b3a00 sub_2b3a00
    Lifted::new(0x700099fa10, tpl_153::<0x700076e890> as *const (), 0), // +0x2b3a10 sub_2b3a10
    Lifted::new(0x700099fa20, tpl_154::<0x700076e890> as *const (), 0), // +0x2b3a20 sub_2b3a20
    Lifted::new(0x700099fa30, tpl_155::<0x700076e890> as *const (), 0), // +0x2b3a30 sub_2b3a30
    Lifted::new(0x700099fa40, tpl_156::<0x700076e890> as *const (), 0), // +0x2b3a40 sub_2b3a40
    Lifted::new(0x700099fa50, tpl_157::<0x700076e890> as *const (), 0), // +0x2b3a50 sub_2b3a50
    Lifted::new(0x700099fa60, tpl_158::<0x700076e890> as *const (), 0), // +0x2b3a60 sub_2b3a60
    Lifted::new(0x700099fa70, tpl_243::<0x700073fe90> as *const (), 0), // +0x2b3a70 sub_2b3a70
    Lifted::new(0x700099fa80, fn_700099fa80 as *const (), 1), // +0x2b3a80 sub_2b3a80
    Lifted::new(0x700099fa90, fn_700099fa90 as *const (), 0), // +0x2b3a90 sub_2b3a90
    Lifted::new(0x700099fab0, fn_700099fab0 as *const (), 1), // +0x2b3ab0 sub_2b3ab0
    Lifted::new(0x700099fba0, tpl_244::<0x100> as *const (), 1), // +0x2b3ba0 sub_2b3ba0
    Lifted::new(0x700099fbc0, tpl_245::<0x10> as *const (), 1), // +0x2b3bc0 sub_2b3bc0
    Lifted::new(0x700099fbd0, fn_700099fbd0 as *const (), 0), // +0x2b3bd0 sub_2b3bd0
    Lifted::new(0x700099fbf0, fn_700099fbf0 as *const (), 3), // +0x2b3bf0 sub_2b3bf0
    Lifted::new(0x700099fc20, tpl_36::<0x8> as *const (), 1), // +0x2b3c20 sub_2b3c20
    Lifted::new(0x700099fc30, tpl_20 as *const (), 1), // +0x2b3c30 sub_2b3c30
    Lifted::new(0x700099fc40, fn_700099fc40 as *const (), 1), // +0x2b3c40 sub_2b3c40
    Lifted::new(0x700099fc60, fn_700099fc60 as *const (), 1), // +0x2b3c60 sub_2b3c60
    Lifted::new(0x700099fc80, tpl_246::<0xfffffffffffffff0, 0xffffffffffffffe0, 0x8> as *const (), 0), // +0x2b3c80 sub_2b3c80
    Lifted::new(0x700099fcb0, tpl_142::<0x9f8> as *const (), 1), // +0x2b3cb0 sub_2b3cb0
    Lifted::new(0x700099fcd0, fn_700099fcd0 as *const (), 0), // +0x2b3cd0 sub_2b3cd0
    Lifted::new(0x700099fce0, fn_700099fce0 as *const (), 1), // +0x2b3ce0 sub_2b3ce0
    Lifted::new(0x700099fd20, tpl_247 as *const (), 1), // +0x2b3d20 sub_2b3d20
    Lifted::new(0x700099fd50, tpl_142::<0xa08> as *const (), 1), // +0x2b3d50 sub_2b3d50
    Lifted::new(0x700099fd70, tpl_171::<0x39> as *const (), 1), // +0x2b3d70 sub_2b3d70
    Lifted::new(0x700099fd80, fn_700099fd80 as *const (), 1), // +0x2b3d80 sub_2b3d80
    Lifted::new(0x700099fde0, fn_700099fde0 as *const (), 1), // +0x2b3de0 sub_2b3de0
    Lifted::new(0x700099fe50, tpl_248::<0x7000e970f0> as *const (), 0), // +0x2b3e50 sub_2b3e50
    Lifted::new(0x700099fe70, tpl_248::<0x7000e970f0> as *const (), 0), // +0x2b3e70 sub_2b3e70
    Lifted::new(0x700099fe90, tpl_248::<0x7000e970f0> as *const (), 0), // +0x2b3e90 sub_2b3e90
    Lifted::new(0x700099feb0, tpl_248::<0x7000e970f0> as *const (), 0), // +0x2b3eb0 sub_2b3eb0
    Lifted::new(0x700099fed0, tpl_248::<0x7000e970f0> as *const (), 0), // +0x2b3ed0 sub_2b3ed0
    Lifted::new(0x700099fef0, tpl_249::<0x7> as *const (), 1), // +0x2b3ef0 sub_2b3ef0
    Lifted::new(0x700099ff00, fn_700099ff00 as *const (), 1), // +0x2b3f00 sub_2b3f00
    Lifted::new(0x700099ff50, fn_700099ff50 as *const (), 1), // +0x2b3f50 sub_2b3f50
    Lifted::new(0x700099ff60, fn_700099ff60 as *const (), 0), // +0x2b3f60 sub_2b3f60
    Lifted::new(0x700099ff80, fn_700099ff80 as *const (), 0), // +0x2b3f80 sub_2b3f80
    Lifted::new(0x700099ffd0, fn_700099ffd0 as *const (), 0), // +0x2b3fd0 sub_2b3fd0
    Lifted::new(0x70009a0090, fn_70009a0090 as *const (), 0), // +0x2b4090 sub_2b4090
    Lifted::new(0x70009a0150, fn_70009a0150 as *const (), 0), // +0x2b4150 sub_2b4150
    Lifted::new(0x70009a01a0, fn_70009a01a0 as *const (), 1), // +0x2b41a0 sub_2b41a0
    Lifted::new(0x70009a0200, fn_70009a0200 as *const (), 0), // +0x2b4200 sub_2b4200
    Lifted::new(0x70009a0300, fn_70009a0300 as *const (), 0), // +0x2b4300 sub_2b4300
    Lifted::new(0x70009a0320, tpl_250::<0x7000e970f0> as *const (), 0), // +0x2b4320 sub_2b4320
    Lifted::new(0x70009a0340, tpl_250::<0x7000e970f0> as *const (), 0), // +0x2b4340 sub_2b4340
    Lifted::new(0x70009a0360, tpl_251::<0x7000e970f0> as *const (), 0), // +0x2b4360 sub_2b4360
    Lifted::new(0x70009a0380, tpl_249::<0x3> as *const (), 1), // +0x2b4380 sub_2b4380
    Lifted::new(0x70009a0390, fn_70009a0390 as *const (), 0), // +0x2b4390 sub_2b4390
    Lifted::new(0x70009a03e0, tpl_142::<0xa40> as *const (), 1), // +0x2b43e0 sub_2b43e0
    Lifted::new(0x70009a0400, fn_70009a0400 as *const (), 0), // +0x2b4400 sub_2b4400
    Lifted::new(0x70009a0410, fn_70009a0410 as *const (), 1), // +0x2b4410 sub_2b4410
    Lifted::new(0x70009a0470, tpl_252::<0x70009bb620> as *const (), 0), // +0x2b4470 sub_2b4470
    Lifted::new(0x70009a0480, tpl_252::<0x70009bb620> as *const (), 0), // +0x2b4480 sub_2b4480
    Lifted::new(0x70009a0490, fn_70009a0490 as *const (), 0), // +0x2b4490 sub_2b4490
    Lifted::new(0x70009a04a0, tpl_248::<0x7000e970f0> as *const (), 0), // +0x2b44a0 sub_2b44a0
    Lifted::new(0x70009a04c0, tpl_248::<0x7000e970f0> as *const (), 0), // +0x2b44c0 sub_2b44c0
    Lifted::new(0x70009a04e0, tpl_253::<0x7000e970f0> as *const (), 0), // +0x2b44e0 sub_2b44e0
    Lifted::new(0x70009a0500, tpl_254::<0x7000e970f0> as *const (), 0), // +0x2b4500 sub_2b4500
    Lifted::new(0x70009a0520, tpl_254::<0x7000e970f0> as *const (), 0), // +0x2b4520 sub_2b4520
    Lifted::new(0x70009a0540, tpl_248::<0x7000e970f0> as *const (), 0), // +0x2b4540 sub_2b4540
    Lifted::new(0x70009a0560, tpl_254::<0x7000e970f0> as *const (), 0), // +0x2b4560 sub_2b4560
    Lifted::new(0x70009a0580, fn_70009a0580 as *const (), 1), // +0x2b4580 sub_2b4580
    Lifted::new(0x70009a05d0, tpl_251::<0x7000e970f0> as *const (), 0), // +0x2b45d0 sub_2b45d0
    Lifted::new(0x70009a05f0, tpl_251::<0x7000e970f0> as *const (), 0), // +0x2b45f0 sub_2b45f0
    Lifted::new(0x70009a0610, tpl_254::<0x7000e970f0> as *const (), 0), // +0x2b4610 sub_2b4610
    Lifted::new(0x70009a0630, tpl_251::<0x7000e970f0> as *const (), 0), // +0x2b4630 sub_2b4630
    Lifted::new(0x70009a0650, fn_70009a0650 as *const (), 1), // +0x2b4650 sub_2b4650
    Lifted::new(0x70009a06a0, tpl_255::<0x30, 0x60> as *const (), 1), // +0x2b46a0 sub_2b46a0
    Lifted::new(0x70009a06b0, tpl_256::<0x7000e970f0> as *const (), 0), // +0x2b46b0 sub_2b46b0
    Lifted::new(0x70009a06d0, tpl_256::<0x7000e970f0> as *const (), 0), // +0x2b46d0 sub_2b46d0
    Lifted::new(0x70009a06f0, fn_70009a06f0 as *const (), 0), // +0x2b46f0 sub_2b46f0
    Lifted::new(0x70009a0710, fn_70009a0710 as *const (), 1), // +0x2b4710 sub_2b4710
    Lifted::new(0x70009a0730, fn_70009a0730 as *const (), 1), // +0x2b4730 sub_2b4730
    Lifted::new(0x70009a0750, fn_70009a0750 as *const (), 0), // +0x2b4750 sub_2b4750
    Lifted::new(0x70009a0770, fn_70009a0770 as *const (), 0), // +0x2b4770 sub_2b4770
    Lifted::new(0x70009a07b0, fn_70009a07b0 as *const (), 0), // +0x2b47b0 sub_2b47b0
    Lifted::new(0x70009a07f0, tpl_49::<0x5c> as *const (), 1), // +0x2b47f0 sub_2b47f0
    Lifted::new(0x70009a0800, tpl_142::<0xa20> as *const (), 1), // +0x2b4800 sub_2b4800
    Lifted::new(0x70009a0820, tpl_15 as *const (), 1), // +0x2b4820 sub_2b4820
    Lifted::new(0x70009a0830, tpl_142::<0xa50> as *const (), 1), // +0x2b4830 sub_2b4830
    Lifted::new(0x70009a0850, tpl_15 as *const (), 1), // +0x2b4850 sub_2b4850
    Lifted::new(0x70009a0860, fn_70009a0860 as *const (), 1), // +0x2b4860 sub_2b4860
    Lifted::new(0x70009a0880, tpl_257::<0x700073f9e0> as *const (), 0), // +0x2b4880 sub_2b4880
    Lifted::new(0x70009a08a0, tpl_257::<0x70007676c0> as *const (), 0), // +0x2b48a0 sub_2b48a0
    Lifted::new(0x70009a08c0, tpl_171::<0xc7e2> as *const (), 1), // +0x2b48c0 sub_2b48c0
    Lifted::new(0x70009a08d0, fn_70009a08d0 as *const (), 1), // +0x2b48d0 sub_2b48d0
    Lifted::new(0x70009a0900, tpl_36::<0xe0> as *const (), 1), // +0x2b4900 sub_2b4900
    Lifted::new(0x70009a0910, tpl_258::<0x240, 0x18, 0x7000707a60, 0x210, 0xe8> as *const (), 0), // +0x2b4910 sub_2b4910
    Lifted::new(0x70009a0980, tpl_36::<0x130> as *const (), 1), // +0x2b4980 sub_2b4980
    Lifted::new(0x70009a0990, tpl_259::<0x2> as *const (), 1), // +0x2b4990 sub_2b4990
    Lifted::new(0x70009a09a0, fn_70009a09a0 as *const (), 0), // +0x2b49a0 sub_2b49a0
    Lifted::new(0x70009a09d0, fn_70009a09d0 as *const (), 1), // +0x2b49d0 sub_2b49d0
    Lifted::new(0x70009a09e0, tpl_247 as *const (), 1), // +0x2b49e0 sub_2b49e0
    Lifted::new(0x70009a0a10, tpl_36::<0xf0> as *const (), 1), // +0x2b4a10 sub_2b4a10
    Lifted::new(0x70009a0a20, tpl_258::<0x240, 0x18, 0x7000707a60, 0x210, 0xe8> as *const (), 0), // +0x2b4a20 sub_2b4a20
    Lifted::new(0x70009a0a90, tpl_260::<0x18> as *const (), 0), // +0x2b4a90 sub_2b4a90
    Lifted::new(0x70009a0aa0, fn_70009a0aa0 as *const (), 1), // +0x2b4aa0 sub_2b4aa0
    Lifted::new(0x70009a0ad0, fn_70009a0ad0 as *const (), 0), // +0x2b4ad0 sub_2b4ad0
    Lifted::new(0x70009a0b10, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x2b4b10 sub_2b4b10
    Lifted::new(0x70009a0b20, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x2b4b20 sub_2b4b20
    Lifted::new(0x70009a0b30, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x2b4b30 sub_2b4b30
    Lifted::new(0x70009a0b40, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x2b4b40 sub_2b4b40
    Lifted::new(0x70009a0b50, tpl_261::<0xfffffffffffffff8> as *const (), 1), // +0x2b4b50 sub_2b4b50
    Lifted::new(0x70009a0b60, fn_70009a0b60 as *const (), 0), // +0x2b4b60 sub_2b4b60
    Lifted::new(0x70009a0b70, tpl_15 as *const (), 1), // +0x2b4b70 sub_2b4b70
    Lifted::new(0x70009a0b80, tpl_51::<0x8> as *const (), 1), // +0x2b4b80 sub_2b4b80
    Lifted::new(0x70009a0b90, fn_70009a0b90 as *const (), 1), // +0x2b4b90 sub_2b4b90
    Lifted::new(0x70009a0ba0, tpl_262::<0x2, 0xffffffffffffffff, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2b4ba0 sub_2b4ba0
    Lifted::new(0x70009a0c70, fn_70009a0c70 as *const (), 1), // +0x2b4c70 sub_2b4c70
    Lifted::new(0x70009a0c90, tpl_142::<0xa80> as *const (), 1), // +0x2b4c90 sub_2b4c90
    Lifted::new(0x70009a0cb0, tpl_44 as *const (), 1), // +0x2b4cb0 sub_2b4cb0
    Lifted::new(0x70009a0cc0, tpl_263::<0x114> as *const (), 1), // +0x2b4cc0 sub_2b4cc0
    Lifted::new(0x70009a0cd0, tpl_263::<0x114> as *const (), 1), // +0x2b4cd0 sub_2b4cd0
    Lifted::new(0x70009a0ce0, tpl_142::<0xaa0> as *const (), 1), // +0x2b4ce0 sub_2b4ce0
    Lifted::new(0x70009a0d00, tpl_142::<0xab0> as *const (), 1), // +0x2b4d00 sub_2b4d00
    Lifted::new(0x70009a0d20, tpl_264::<0x700079cfd0> as *const (), 0), // +0x2b4d20 sub_2b4d20
    Lifted::new(0x70009a0d40, fn_70009a0d40 as *const (), 1), // +0x2b4d40 sub_2b4d40
    Lifted::new(0x70009a0d60, fn_70009a0d60 as *const (), 0), // +0x2b4d60 sub_2b4d60
    Lifted::new(0x70009a0db0, tpl_181 as *const (), 1), // +0x2b4db0 sub_2b4db0
    Lifted::new(0x70009a0dc0, fn_70009a0dc0 as *const (), 2), // +0x2b4dc0 sub_2b4dc0
    Lifted::new(0x70009a0e30, tpl_265::<0x13, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffff6940> as *const (), 1), // +0x2b4e30 sub_2b4e30
    Lifted::new(0x70009a0eb0, tpl_266::<0x13, 0x20, 0x28> as *const (), 1), // +0x2b4eb0 sub_2b4eb0
    Lifted::new(0x70009a0f00, fn_70009a0f00 as *const (), 1), // +0x2b4f00 sub_2b4f00
    Lifted::new(0x70009a0f90, fn_70009a0f90 as *const (), 1), // +0x2b4f90 sub_2b4f90
    Lifted::new(0x70009a0ff0, tpl_267::<0x1b, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffff6940> as *const (), 1), // +0x2b4ff0 sub_2b4ff0
    Lifted::new(0x70009a1070, tpl_268::<0x1b, 0x20, 0x28> as *const (), 1), // +0x2b5070 sub_2b5070
    Lifted::new(0x70009a10c0, tpl_269::<0x14, 0xffffffffffffffff, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffff6940> as *const (), 1), // +0x2b50c0 sub_2b50c0
    Lifted::new(0x70009a1140, tpl_270::<0x14, 0xffffffffffffffff, 0x20, 0x28> as *const (), 1), // +0x2b5140 sub_2b5140
    Lifted::new(0x70009a1190, tpl_265::<0x13, 0x7001c72d48, 0x8, 0x58, 0x10, 0xffffffffffff6940> as *const (), 1), // +0x2b5190 sub_2b5190
    Lifted::new(0x70009a1210, tpl_266::<0x13, 0x20, 0x28> as *const (), 1), // +0x2b5210 sub_2b5210
    Lifted::new(0x70009a1260, fn_70009a1260 as *const (), 1), // +0x2b5260 sub_2b5260
    Lifted::new(0x70009a12e0, fn_70009a12e0 as *const (), 1), // +0x2b52e0 sub_2b52e0
    Lifted::new(0x70009a1360, fn_70009a1360 as *const (), 1), // +0x2b5360 sub_2b5360
    Lifted::new(0x70009a13e0, tpl_271::<0x10, 0xffffffffffffffff, 0x28> as *const (), 1), // +0x2b53e0 sub_2b53e0
    Lifted::new(0x70009a1430, tpl_271::<0x10, 0xffffffffffffffff, 0x28> as *const (), 1), // +0x2b5430 sub_2b5430
    Lifted::new(0x70009a1480, fn_70009a1480 as *const (), 1), // +0x2b5480 sub_2b5480
    Lifted::new(0x70009a1490, tpl_183::<0x8, 0x238> as *const (), 1), // +0x2b5490 sub_2b5490
    Lifted::new(0x70009a14f0, fn_70009a14f0 as *const (), 1), // +0x2b54f0 sub_2b54f0
    Lifted::new(0x70009a1530, tpl_36::<0x218> as *const (), 1), // +0x2b5530 sub_2b5530
    Lifted::new(0x70009a1540, tpl_272::<0x7001c73184, 0x18> as *const (), 0), // +0x2b5540 sub_2b5540
    Lifted::new(0x70009a1560, tpl_15 as *const (), 1), // +0x2b5560 sub_2b5560
    Lifted::new(0x70009a1570, tpl_272::<0x7001c73184, 0x8> as *const (), 0), // +0x2b5570 sub_2b5570
    Lifted::new(0x70009a1590, tpl_15 as *const (), 1), // +0x2b5590 sub_2b5590
    Lifted::new(0x70009a15a0, tpl_272::<0x7001c73184, 0x10> as *const (), 0), // +0x2b55a0 sub_2b55a0
    Lifted::new(0x70009a15c0, tpl_15 as *const (), 1), // +0x2b55c0 sub_2b55c0
    Lifted::new(0x70009a15d0, fn_70009a15d0 as *const (), 0), // +0x2b55d0 sub_2b55d0
    Lifted::new(0x70009a1670, fn_70009a1670 as *const (), 1), // +0x2b5670 sub_2b5670
    Lifted::new(0x70009a16c0, tpl_162::<0x7001c71e94, 0x7001d55088> as *const (), 0), // +0x2b56c0 sub_2b56c0
    Lifted::new(0x70009a16d0, fn_70009a16d0 as *const (), 1), // +0x2b56d0 sub_2b56d0
    Lifted::new(0x70009a1700, fn_70009a1700 as *const (), 1), // +0x2b5700 sub_2b5700
    Lifted::new(0x70009a1730, fn_70009a1730 as *const (), 3), // +0x2b5730 sub_2b5730
    Lifted::new(0x70009a17a0, fn_70009a17a0 as *const (), 3), // +0x2b57a0 sub_2b57a0
    Lifted::new(0x70009a1810, fn_70009a1810 as *const (), 3), // +0x2b5810 sub_2b5810
    Lifted::new(0x70009a1870, tpl_142::<0xb08> as *const (), 1), // +0x2b5870 sub_2b5870
    Lifted::new(0x70009a1890, tpl_142::<0xb48> as *const (), 1), // +0x2b5890 sub_2b5890
    Lifted::new(0x70009a18b0, tpl_142::<0xb10> as *const (), 1), // +0x2b58b0 sub_2b58b0
    Lifted::new(0x70009a18d0, tpl_142::<0xb18> as *const (), 1), // +0x2b58d0 sub_2b58d0
    Lifted::new(0x70009a18f0, fn_70009a18f0 as *const (), 0), // +0x2b58f0 sub_2b58f0
    Lifted::new(0x70009a1910, tpl_142::<0xb20> as *const (), 1), // +0x2b5910 sub_2b5910
    Lifted::new(0x70009a1930, tpl_142::<0xb28> as *const (), 1), // +0x2b5930 sub_2b5930
    Lifted::new(0x70009a1950, fn_70009a1950 as *const (), 0), // +0x2b5950 sub_2b5950
    Lifted::new(0x70009a1990, fn_70009a1990 as *const (), 0), // +0x2b5990 sub_2b5990
    Lifted::new(0x70009a19c0, tpl_232::<0x8, 0x10, 0xffffffffffff7540> as *const (), 1), // +0x2b59c0 sub_2b59c0
    Lifted::new(0x70009a1a00, fn_70009a1a00 as *const (), 0), // +0x2b5a00 sub_2b5a00
    Lifted::new(0x70009a1aa0, fn_70009a1aa0 as *const (), 0), // +0x2b5aa0 sub_2b5aa0
    Lifted::new(0x70009a1ac0, tpl_14 as *const (), 1), // +0x2b5ac0 sub_2b5ac0
    Lifted::new(0x70009a1ad0, tpl_14 as *const (), 1), // +0x2b5ad0 sub_2b5ad0
    Lifted::new(0x70009a1ae0, fn_70009a1ae0 as *const (), 1), // +0x2b5ae0 sub_2b5ae0
    Lifted::new(0x70009a1b00, fn_70009a1b00 as *const (), 1), // +0x2b5b00 sub_2b5b00
    Lifted::new(0x70009a1b30, fn_70009a1b30 as *const (), 0), // +0x2b5b30 sub_2b5b30
    Lifted::new(0x70009a1ba0, fn_70009a1ba0 as *const (), 0), // +0x2b5ba0 sub_2b5ba0
    Lifted::new(0x70009a1c40, fn_70009a1c40 as *const (), 0), // +0x2b5c40 sub_2b5c40
    Lifted::new(0x70009a1c60, fn_70009a1c60 as *const (), 2), // +0x2b5c60 sub_2b5c60
    Lifted::new(0x70009a1d60, fn_70009a1d60 as *const (), 1), // +0x2b5d60 sub_2b5d60
    Lifted::new(0x70009a1d80, tpl_273::<0x7, 0xb> as *const (), 1), // +0x2b5d80 sub_2b5d80
    Lifted::new(0x70009a1da0, tpl_273::<0x7, 0xb> as *const (), 1), // +0x2b5da0 sub_2b5da0
    Lifted::new(0x70009a1dc0, tpl_273::<0xb, 0xf> as *const (), 1), // +0x2b5dc0 sub_2b5dc0
    Lifted::new(0x70009a1de0, fn_70009a1de0 as *const (), 1), // +0x2b5de0 sub_2b5de0
    Lifted::new(0x70009a1e40, tpl_149::<0x20, 0xffffffffffffffff> as *const (), 1), // +0x2b5e40 sub_2b5e40
    Lifted::new(0x70009a1e80, tpl_150::<0x20, 0xffffffffffffffff> as *const (), 1), // +0x2b5e80 sub_2b5e80
    Lifted::new(0x70009a1eb0, tpl_224::<0x1f> as *const (), 1), // +0x2b5eb0 sub_2b5eb0
    Lifted::new(0x70009a1ef0, tpl_226::<0x1f> as *const (), 1), // +0x2b5ef0 sub_2b5ef0
    Lifted::new(0x70009a1f20, tpl_234::<0x1c, 0xffffffffffffffff, 0xb> as *const (), 1), // +0x2b5f20 sub_2b5f20
    Lifted::new(0x70009a1f60, tpl_235::<0x1c, 0xffffffffffffffff, 0xb> as *const (), 1), // +0x2b5f60 sub_2b5f60
    Lifted::new(0x70009a1f90, fn_70009a1f90 as *const (), 1), // +0x2b5f90 sub_2b5f90
    Lifted::new(0x70009a2000, tpl_228::<0x13, 0x1c, 0xffffffffffffffff> as *const (), 1), // +0x2b6000 sub_2b6000
    Lifted::new(0x70009a2060, tpl_218::<0x13> as *const (), 1), // +0x2b6060 sub_2b6060
    Lifted::new(0x70009a20a0, fn_70009a20a0 as *const (), 0), // +0x2b60a0 sub_2b60a0
    Lifted::new(0x70009a20b0, fn_70009a20b0 as *const (), 0), // +0x2b60b0 sub_2b60b0
    Lifted::new(0x70009a20c0, fn_70009a20c0 as *const (), 0), // +0x2b60c0 sub_2b60c0
    Lifted::new(0x70009a20d0, fn_70009a20d0 as *const (), 0), // +0x2b60d0 sub_2b60d0
    Lifted::new(0x70009a20f0, tpl_274::<0x7000784d10> as *const (), 0), // +0x2b60f0 sub_2b60f0
    Lifted::new(0x70009a2110, tpl_274::<0x7000784d10> as *const (), 0), // +0x2b6110 sub_2b6110
    Lifted::new(0x70009a2130, tpl_218::<0x13> as *const (), 1), // +0x2b6130 sub_2b6130
    Lifted::new(0x70009a2170, tpl_275::<0xd8, 0x18> as *const (), 1), // +0x2b6170 sub_2b6170
    Lifted::new(0x70009a2180, tpl_15 as *const (), 1), // +0x2b6180 sub_2b6180
    Lifted::new(0x70009a2190, tpl_142::<0xb60> as *const (), 1), // +0x2b6190 sub_2b6190
    Lifted::new(0x70009a21b0, tpl_142::<0xb68> as *const (), 1), // +0x2b61b0 sub_2b61b0
    Lifted::new(0x70009a21d0, tpl_142::<0xb70> as *const (), 1), // +0x2b61d0 sub_2b61d0
    Lifted::new(0x70009a21f0, tpl_142::<0xa00> as *const (), 1), // +0x2b61f0 sub_2b61f0
    Lifted::new(0x70009a2210, tpl_53 as *const (), 1), // +0x2b6210 sub_2b6210
    Lifted::new(0x70009a2220, tpl_276::<0x700182e5b0> as *const (), 1), // +0x2b6220 sub_2b6220
    Lifted::new(0x70009a2240, fn_70009a2240 as *const (), 1), // +0x2b6240 sub_2b6240
    Lifted::new(0x70009a2260, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x70009a2367, 0x700098c920, 0x70007f0910, 0x70009a2383> as *const (), 0), // +0x2b6260 sub_2b6260
    Lifted::new(0x70009a23d0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x70009a24d7, 0x700098c9d0, 0x70007f0910, 0x70009a24f3> as *const (), 0), // +0x2b63d0 sub_2b63d0
    Lifted::new(0x70009a2540, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x70009a2647, 0x700098ca80, 0x70007f0910, 0x70009a2663> as *const (), 0), // +0x2b6540 sub_2b6540
    Lifted::new(0x70009a26b0, fn_70009a26b0 as *const (), 0), // +0x2b66b0 sub_2b66b0
    Lifted::new(0x70009a2710, fn_70009a2710 as *const (), 0), // +0x2b6710 sub_2b6710
    Lifted::new(0x70009a2770, fn_70009a2770 as *const (), 0), // +0x2b6770 sub_2b6770
    Lifted::new(0x70009a27b0, fn_70009a27b0 as *const (), 0), // +0x2b67b0 sub_2b67b0
    Lifted::new(0x70009a27f0, fn_70009a27f0 as *const (), 0), // +0x2b67f0 sub_2b67f0
    Lifted::new(0x70009a28a0, fn_70009a28a0 as *const (), 1), // +0x2b68a0 sub_2b68a0
    Lifted::new(0x70009a28e0, fn_70009a28e0 as *const (), 1), // +0x2b68e0 sub_2b68e0
    Lifted::new(0x70009a2920, tpl_278::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x70009a29ad> as *const (), 0), // +0x2b6920 sub_2b6920
    Lifted::new(0x70009a29e0, fn_70009a29e0 as *const (), 0), // +0x2b69e0 sub_2b69e0
    Lifted::new(0x70009a2ab0, tpl_279::<0x8, 0x18, 0x10, 0x7, 0x700076bf20, 0x70009a4db0, 0x70009a2b51> as *const (), 0), // +0x2b6ab0 sub_2b6ab0
    Lifted::new(0x70009a2b80, fn_70009a2b80 as *const (), 0), // +0x2b6b80 sub_2b6b80
    Lifted::new(0x70009a2c70, fn_70009a2c70 as *const (), 0), // +0x2b6c70 sub_2b6c70
    Lifted::new(0x70009a2d80, tpl_279::<0x8, 0x18, 0x10, 0x7, 0x700076bf20, 0x70009a4e90, 0x70009a2e21> as *const (), 0), // +0x2b6d80 sub_2b6d80
    Lifted::new(0x70009a2e50, tpl_280::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x70009a55a0, 0x7001d54968, 0x70009a2f08, 0x70009a4eb0, 0x70009a2f0e> as *const (), 0), // +0x2b6e50 sub_2b6e50
    Lifted::new(0x70009a2f40, tpl_281::<0x70006f32a0> as *const (), 0), // +0x2b6f40 sub_2b6f40
    Lifted::new(0x70009a2f50, tpl_282::<0x8, 0x10, 0x70007a48c0, 0x700177c0d0> as *const (), 0), // +0x2b6f50 sub_2b6f50
    Lifted::new(0x70009a2fe0, tpl_281::<0x700077b670> as *const (), 0), // +0x2b6fe0 sub_2b6fe0
    Lifted::new(0x70009a2ff0, fn_70009a2ff0 as *const (), 0), // +0x2b6ff0 sub_2b6ff0
    Lifted::new(0x70009a30f0, tpl_283::<0x38, 0x700177c170, 0x70017d37ec> as *const (), 2), // +0x2b70f0 sub_2b70f0
    Lifted::new(0x70009a31b0, tpl_283::<0x38, 0x700177c170, 0x70017d37ec> as *const (), 2), // +0x2b71b0 sub_2b71b0
    Lifted::new(0x70009a3270, tpl_15 as *const (), 1), // +0x2b7270 sub_2b7270
    Lifted::new(0x70009a3280, fn_70009a3280 as *const (), 0), // +0x2b7280 sub_2b7280
    Lifted::new(0x70009a32a0, tpl_284::<0x70009a2260, 0x70017d37a0, 0x70017790e0, 0x70009a345d> as *const (), 0), // +0x2b72a0 sub_2b72a0
    Lifted::new(0x70009a34d0, tpl_285::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x70009a3607, 0x700098cb30, 0x70007f0910, 0x70009a3796, 0x70017d37a0, 0x70017790e0, 0x70009a37a3> as *const (), 0), // +0x2b74d0 sub_2b74d0
    Lifted::new(0x70009a3860, tpl_286::<0x70009a2540, 0x70017d37a0, 0x70017790e0, 0x70009a3a4b> as *const (), 0), // +0x2b7860 sub_2b7860
    Lifted::new(0x70009a3ad0, tpl_20 as *const (), 1), // +0x2b7ad0 sub_2b7ad0
    Lifted::new(0x70009a3ae0, tpl_20 as *const (), 1), // +0x2b7ae0 sub_2b7ae0
    Lifted::new(0x70009a3af0, tpl_20 as *const (), 1), // +0x2b7af0 sub_2b7af0
    Lifted::new(0x70009a3b00, tpl_20 as *const (), 1), // +0x2b7b00 sub_2b7b00
    Lifted::new(0x70009a3b10, tpl_287 as *const (), 1), // +0x2b7b10 sub_2b7b10
    Lifted::new(0x70009a3e30, tpl_288 as *const (), 1), // +0x2b7e30 sub_2b7e30
    Lifted::new(0x70009a3e40, tpl_288 as *const (), 1), // +0x2b7e40 sub_2b7e40
    Lifted::new(0x70009a3e50, fn_70009a3e50 as *const (), 1), // +0x2b7e50 sub_2b7e50
    Lifted::new(0x70009a3e70, fn_70009a3e70 as *const (), 1), // +0x2b7e70 sub_2b7e70
    Lifted::new(0x70009a3e90, tpl_289::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x70009a3f22> as *const (), 0), // +0x2b7e90 sub_2b7e90
    Lifted::new(0x70009a3f50, tpl_289::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x70009a3fe2> as *const (), 0), // +0x2b7f50 sub_2b7f50
    Lifted::new(0x70009a4010, tpl_289::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x70009a40a2> as *const (), 0), // +0x2b8010 sub_2b8010
    Lifted::new(0x70009a40d0, tpl_290::<0x7000783fb0, 0x8, 0x10> as *const (), 0), // +0x2b80d0 sub_2b80d0
    Lifted::new(0x70009a4130, tpl_290::<0x7000783fb0, 0x8, 0x10> as *const (), 0), // +0x2b8130 sub_2b8130
    Lifted::new(0x70009a4190, tpl_290::<0x7000783fb0, 0x8, 0x10> as *const (), 0), // +0x2b8190 sub_2b8190
    Lifted::new(0x70009a41f0, tpl_291::<0x8, 0x10> as *const (), 1), // +0x2b81f0 sub_2b81f0
    Lifted::new(0x70009a4220, tpl_291::<0x8, 0x10> as *const (), 1), // +0x2b8220 sub_2b8220
    Lifted::new(0x70009a4250, tpl_291::<0x8, 0x10> as *const (), 1), // +0x2b8250 sub_2b8250
    Lifted::new(0x70009a4280, tpl_291::<0x8, 0x10> as *const (), 1), // +0x2b8280 sub_2b8280
    Lifted::new(0x70009a42b0, tpl_15 as *const (), 1), // +0x2b82b0 sub_2b82b0
    Lifted::new(0x70009a42c0, tpl_15 as *const (), 1), // +0x2b82c0 sub_2b82c0
    Lifted::new(0x70009a42d0, fn_70009a42d0 as *const (), 0), // +0x2b82d0 sub_2b82d0
    Lifted::new(0x70009a4350, fn_70009a4350 as *const (), 0), // +0x2b8350 sub_2b8350
    Lifted::new(0x70009a43d0, tpl_292::<0x70017d2c40, 0x70017790e0, 0x70009a43e0> as *const (), 0), // +0x2b83d0 sub_2b83d0
    Lifted::new(0x70009a43f0, tpl_101::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x70009a444b> as *const (), 0), // +0x2b83f0 sub_2b83f0
    Lifted::new(0x70009a4470, tpl_293::<0x8, 0x70009896e0, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x70009a44da> as *const (), 0), // +0x2b8470 sub_2b8470
    Lifted::new(0x70009a4500, tpl_293::<0x8, 0x7000989790, 0x10, 0xfffffffffffffff8, 0x700177c312, 0x7001d54968, 0x70009a456a> as *const (), 0), // +0x2b8500 sub_2b8500
    Lifted::new(0x70009a4590, fn_70009a4590 as *const (), 3), // +0x2b8590 sub_2b8590
    Lifted::new(0x70009a45c0, fn_70009a45c0 as *const (), 0), // +0x2b85c0 sub_2b85c0
    Lifted::new(0x70009a4620, fn_70009a4620 as *const (), 0), // +0x2b8620 sub_2b8620
    Lifted::new(0x70009a46c0, fn_70009a46c0 as *const (), 2), // +0x2b86c0 sub_2b86c0
    Lifted::new(0x70009a4740, fn_70009a4740 as *const (), 0), // +0x2b8740 sub_2b8740
    Lifted::new(0x70009a4780, tpl_294::<0x700098bd80> as *const (), 0), // +0x2b8780 sub_2b8780
    Lifted::new(0x70009a47a0, tpl_295::<0x700177c0d0> as *const (), 0), // +0x2b87a0 sub_2b87a0
    Lifted::new(0x70009a47b0, tpl_295::<0x700177c0d0> as *const (), 0), // +0x2b87b0 sub_2b87b0
    Lifted::new(0x70009a47c0, tpl_294::<0x700098bd80> as *const (), 0), // +0x2b87c0 sub_2b87c0
    Lifted::new(0x70009a47e0, tpl_294::<0x700098bcd0> as *const (), 0), // +0x2b87e0 sub_2b87e0
    Lifted::new(0x70009a4800, tpl_295::<0x700177c0d0> as *const (), 0), // +0x2b8800 sub_2b8800
    Lifted::new(0x70009a4810, tpl_294::<0x700098bd80> as *const (), 0), // +0x2b8810 sub_2b8810
    Lifted::new(0x70009a4830, tpl_30 as *const (), 1), // +0x2b8830 sub_2b8830
    Lifted::new(0x70009a4840, tpl_30 as *const (), 1), // +0x2b8840 sub_2b8840
    Lifted::new(0x70009a4850, tpl_30 as *const (), 1), // +0x2b8850 sub_2b8850
    Lifted::new(0x70009a4860, tpl_30 as *const (), 1), // +0x2b8860 sub_2b8860
    Lifted::new(0x70009a4870, tpl_36::<0x8> as *const (), 1), // +0x2b8870 sub_2b8870
    Lifted::new(0x70009a4880, tpl_36::<0x8> as *const (), 1), // +0x2b8880 sub_2b8880
    Lifted::new(0x70009a4890, tpl_36::<0x8> as *const (), 1), // +0x2b8890 sub_2b8890
    Lifted::new(0x70009a48a0, tpl_36::<0x8> as *const (), 1), // +0x2b88a0 sub_2b88a0
    Lifted::new(0x70009a48b0, fn_70009a48b0 as *const (), 0), // +0x2b88b0 sub_2b88b0
    Lifted::new(0x70009a4ae0, tpl_296::<0x700177c312> as *const (), 0), // +0x2b8ae0 sub_2b8ae0
    Lifted::new(0x70009a4d90, tpl_292::<0x70017d2b78, 0x70017790e0, 0x70009a4da0> as *const (), 0), // +0x2b8d90 sub_2b8d90
    Lifted::new(0x70009a4db0, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4dc0> as *const (), 0), // +0x2b8db0 sub_2b8db0
    Lifted::new(0x70009a4dd0, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4de0> as *const (), 0), // +0x2b8dd0 sub_2b8dd0
    Lifted::new(0x70009a4df0, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4e00> as *const (), 0), // +0x2b8df0 sub_2b8df0
    Lifted::new(0x70009a4e10, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4e20> as *const (), 0), // +0x2b8e10 sub_2b8e10
    Lifted::new(0x70009a4e30, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4e40> as *const (), 0), // +0x2b8e30 sub_2b8e30
    Lifted::new(0x70009a4e50, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4e60> as *const (), 0), // +0x2b8e50 sub_2b8e50
    Lifted::new(0x70009a4e70, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4e80> as *const (), 0), // +0x2b8e70 sub_2b8e70
    Lifted::new(0x70009a4e90, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4ea0> as *const (), 0), // +0x2b8e90 sub_2b8e90
    Lifted::new(0x70009a4eb0, tpl_292::<0x70017d3768, 0x70017790e0, 0x70009a4ec0> as *const (), 0), // +0x2b8eb0 sub_2b8eb0
    Lifted::new(0x70009a4ed0, tpl_292::<0x70017d3748, 0x70017790f0, 0x70009a4ee0> as *const (), 0), // +0x2b8ed0 sub_2b8ed0
    Lifted::new(0x70009a4ef0, tpl_292::<0x70017d3748, 0x70017790f0, 0x70009a4f00> as *const (), 0), // +0x2b8ef0 sub_2b8ef0
    Lifted::new(0x70009a4f10, tpl_292::<0x70017d3748, 0x70017790f0, 0x70009a4f20> as *const (), 0), // +0x2b8f10 sub_2b8f10
    Lifted::new(0x70009a4f30, tpl_142::<0xea8> as *const (), 1), // +0x2b8f30 sub_2b8f30
    Lifted::new(0x70009a4f50, tpl_142::<0xeb0> as *const (), 1), // +0x2b8f50 sub_2b8f50
    Lifted::new(0x70009a4f70, tpl_142::<0xd58> as *const (), 1), // +0x2b8f70 sub_2b8f70
    Lifted::new(0x70009a4f90, tpl_142::<0xd60> as *const (), 1), // +0x2b8f90 sub_2b8f90
    Lifted::new(0x70009a4fb0, tpl_142::<0xd68> as *const (), 1), // +0x2b8fb0 sub_2b8fb0
    Lifted::new(0x70009a4fd0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x9f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2b8fd0 sub_2b8fd0
    Lifted::new(0x70009a50a0, tpl_52::<0x130> as *const (), 1), // +0x2b90a0 sub_2b90a0
    Lifted::new(0x70009a50b0, fn_70009a50b0 as *const (), 0), // +0x2b90b0 sub_2b90b0
    Lifted::new(0x70009a5150, fn_70009a5150 as *const (), 0), // +0x2b9150 sub_2b9150
    Lifted::new(0x70009a5210, tpl_142::<0x4b8> as *const (), 1), // +0x2b9210 sub_2b9210
    Lifted::new(0x70009a5230, fn_70009a5230 as *const (), 0), // +0x2b9230 sub_2b9230
    Lifted::new(0x70009a52d0, tpl_52::<0x18> as *const (), 1), // +0x2b92d0 sub_2b92d0
    Lifted::new(0x70009a52e0, tpl_52::<0x8> as *const (), 1), // +0x2b92e0 sub_2b92e0
    Lifted::new(0x70009a52f0, tpl_36::<0x348> as *const (), 1), // +0x2b92f0 sub_2b92f0
    Lifted::new(0x70009a5300, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x377, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2b9300 sub_2b9300
    Lifted::new(0x70009a53d0, tpl_142::<0xd70> as *const (), 1), // +0x2b93d0 sub_2b93d0
    Lifted::new(0x70009a53f0, tpl_298::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x70009a5444, 0x70007f0910, 0x70009a545f> as *const (), 0), // +0x2b93f0 sub_2b93f0
    Lifted::new(0x70009a5480, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x70009a54cd, 0x70007f0910, 0x70009a54eb> as *const (), 0), // +0x2b9480 sub_2b9480
    Lifted::new(0x70009a5510, tpl_300::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x70009a555d, 0x70007f0910, 0x70009a557b> as *const (), 0), // +0x2b9510 sub_2b9510
    Lifted::new(0x70009a55a0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x70009a55f4, 0x70007f0910, 0x70009a560f> as *const (), 0), // +0x2b95a0 sub_2b95a0
    Lifted::new(0x70009a5630, fn_70009a5630 as *const (), 0), // +0x2b9630 sub_2b9630
    Lifted::new(0x70009a5660, tpl_142::<0xd78> as *const (), 1), // +0x2b9660 sub_2b9660
    Lifted::new(0x70009a5680, tpl_142::<0xed8> as *const (), 1), // +0x2b9680 sub_2b9680
    Lifted::new(0x70009a56a0, tpl_142::<0xee0> as *const (), 1), // +0x2b96a0 sub_2b96a0
    Lifted::new(0x70009a56c0, fn_70009a56c0 as *const (), 1), // +0x2b96c0 sub_2b96c0
    Lifted::new(0x70009a56f0, tpl_30 as *const (), 1), // +0x2b96f0 sub_2b96f0
    Lifted::new(0x70009a5700, tpl_36::<0xa708> as *const (), 1), // +0x2b9700 sub_2b9700
    Lifted::new(0x70009a5710, tpl_275::<0x10, 0x8> as *const (), 1), // +0x2b9710 sub_2b9710
    Lifted::new(0x70009a5720, tpl_36::<0x8> as *const (), 1), // +0x2b9720 sub_2b9720
    Lifted::new(0x70009a5730, tpl_36::<0x20> as *const (), 1), // +0x2b9730 sub_2b9730
    Lifted::new(0x70009a5740, tpl_47::<0xd930> as *const (), 1), // +0x2b9740 sub_2b9740
    Lifted::new(0x70009a5750, tpl_36::<0xc878> as *const (), 1), // +0x2b9750 sub_2b9750
    Lifted::new(0x70009a5760, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xa3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2b9760 sub_2b9760
    Lifted::new(0x70009a5830, tpl_193::<0x78> as *const (), 1), // +0x2b9830 sub_2b9830
    Lifted::new(0x70009a5840, tpl_302 as *const (), 1), // +0x2b9840 sub_2b9840
    Lifted::new(0x70009a5850, tpl_303 as *const (), 1), // +0x2b9850 sub_2b9850
    Lifted::new(0x70009a5860, tpl_304 as *const (), 1), // +0x2b9860 sub_2b9860
    Lifted::new(0x70009a5870, tpl_193::<0x2> as *const (), 1), // +0x2b9870 sub_2b9870
    Lifted::new(0x70009a5880, tpl_36::<0xc890> as *const (), 1), // +0x2b9880 sub_2b9880
    Lifted::new(0x70009a5890, tpl_142::<0x4c8> as *const (), 1), // +0x2b9890 sub_2b9890
    Lifted::new(0x70009a58b0, tpl_142::<0x4d0> as *const (), 1), // +0x2b98b0 sub_2b98b0
    Lifted::new(0x70009a58d0, tpl_142::<0x4d8> as *const (), 1), // +0x2b98d0 sub_2b98d0
    Lifted::new(0x70009a58f0, tpl_47::<0x6a8> as *const (), 1), // +0x2b98f0 sub_2b98f0
    Lifted::new(0x70009a5900, tpl_36::<0xc8e8> as *const (), 1), // +0x2b9900 sub_2b9900
    Lifted::new(0x70009a5910, tpl_142::<0x1108> as *const (), 1), // +0x2b9910 sub_2b9910
    Lifted::new(0x70009a5930, tpl_142::<0x1100> as *const (), 1), // +0x2b9930 sub_2b9930
    Lifted::new(0x70009a5950, fn_70009a5950 as *const (), 0), // +0x2b9950 sub_2b9950
    Lifted::new(0x70009a5b00, tpl_142::<0x4e0> as *const (), 1), // +0x2b9b00 sub_2b9b00
    Lifted::new(0x70009a5b20, tpl_142::<0xf20> as *const (), 1), // +0x2b9b20 sub_2b9b20
    Lifted::new(0x70009a5b40, tpl_142::<0x178> as *const (), 1), // +0x2b9b40 sub_2b9b40
    Lifted::new(0x70009a5b60, tpl_142::<0x390> as *const (), 1), // +0x2b9b60 sub_2b9b60
    Lifted::new(0x70009a5b80, tpl_142::<0x4f0> as *const (), 1), // +0x2b9b80 sub_2b9b80
    Lifted::new(0x70009a5ba0, tpl_142::<0x4f8> as *const (), 1), // +0x2b9ba0 sub_2b9ba0
    Lifted::new(0x70009a5bc0, tpl_142::<0xd80> as *const (), 1), // +0x2b9bc0 sub_2b9bc0
    Lifted::new(0x70009a5be0, tpl_36::<0xd868> as *const (), 1), // +0x2b9be0 sub_2b9be0
    Lifted::new(0x70009a5bf0, tpl_305::<0xd878, 0x8, 0xd870> as *const (), 1), // +0x2b9bf0 sub_2b9bf0
    Lifted::new(0x70009a5c30, tpl_142::<0xff0> as *const (), 1), // +0x2b9c30 sub_2b9c30
    Lifted::new(0x70009a5c50, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xa7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2b9c50 sub_2b9c50
    Lifted::new(0x70009a5d20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xab, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2b9d20 sub_2b9d20
    Lifted::new(0x70009a5df0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xaf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2b9df0 sub_2b9df0
    Lifted::new(0x70009a5ec0, tpl_142::<0xb98> as *const (), 1), // +0x2b9ec0 sub_2b9ec0
    Lifted::new(0x70009a5ee0, tpl_142::<0xfa0> as *const (), 1), // +0x2b9ee0 sub_2b9ee0
    Lifted::new(0x70009a5f00, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x35b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2b9f00 sub_2b9f00
    Lifted::new(0x70009a5fd0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x35f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2b9fd0 sub_2b9fd0
    Lifted::new(0x70009a60a0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xb3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ba0a0 sub_2ba0a0
    Lifted::new(0x70009a6170, tpl_39::<0x48> as *const (), 1), // +0x2ba170 sub_2ba170
    Lifted::new(0x70009a6180, tpl_142::<0xfe8> as *const (), 1), // +0x2ba180 sub_2ba180
    Lifted::new(0x70009a61a0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xb7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ba1a0 sub_2ba1a0
    Lifted::new(0x70009a6270, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x363, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ba270 sub_2ba270
    Lifted::new(0x70009a6340, tpl_142::<0xf28> as *const (), 1), // +0x2ba340 sub_2ba340
    Lifted::new(0x70009a6360, tpl_142::<0x1b8> as *const (), 1), // +0x2ba360 sub_2ba360
    Lifted::new(0x70009a6380, tpl_142::<0xfc0> as *const (), 1), // +0x2ba380 sub_2ba380
    Lifted::new(0x70009a63a0, tpl_142::<0x508> as *const (), 1), // +0x2ba3a0 sub_2ba3a0
    Lifted::new(0x70009a63c0, tpl_306::<0x68> as *const (), 1), // +0x2ba3c0 sub_2ba3c0
    Lifted::new(0x70009a63d0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x367, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ba3d0 sub_2ba3d0
    Lifted::new(0x70009a64a0, tpl_142::<0x520> as *const (), 1), // +0x2ba4a0 sub_2ba4a0
    Lifted::new(0x70009a64c0, tpl_142::<0xd88> as *const (), 1), // +0x2ba4c0 sub_2ba4c0
    Lifted::new(0x70009a64e0, tpl_142::<0x528> as *const (), 1), // +0x2ba4e0 sub_2ba4e0
    Lifted::new(0x70009a6500, fn_70009a6500 as *const (), 0), // +0x2ba500 sub_2ba500
    Lifted::new(0x70009a66c0, tpl_282::<0x8, 0x10, 0x70007a48c0, 0x700177c0d0> as *const (), 0), // +0x2ba6c0 sub_2ba6c0
    Lifted::new(0x70009a6750, fn_70009a6750 as *const (), 2), // +0x2ba750 sub_2ba750
    Lifted::new(0x70009a6860, tpl_307::<0x8, 0x7000988ed0> as *const (), 0), // +0x2ba860 sub_2ba860
    Lifted::new(0x70009a6880, fn_70009a6880 as *const (), 0), // +0x2ba880 sub_2ba880
    Lifted::new(0x70009a6960, tpl_36::<0xc808> as *const (), 1), // +0x2ba960 sub_2ba960
    Lifted::new(0x70009a6970, tpl_308::<0x10> as *const (), 1), // +0x2ba970 sub_2ba970
    Lifted::new(0x70009a6980, tpl_36::<0x10> as *const (), 1), // +0x2ba980 sub_2ba980
    Lifted::new(0x70009a6990, tpl_36::<0x50> as *const (), 1), // +0x2ba990 sub_2ba990
    Lifted::new(0x70009a69a0, tpl_52::<0xa628> as *const (), 1), // +0x2ba9a0 sub_2ba9a0
    Lifted::new(0x70009a69b0, tpl_52::<0xd910> as *const (), 1), // +0x2ba9b0 sub_2ba9b0
    Lifted::new(0x70009a69c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xbb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ba9c0 sub_2ba9c0
    Lifted::new(0x70009a6a90, tpl_142::<0x1048> as *const (), 1), // +0x2baa90 sub_2baa90
    Lifted::new(0x70009a6ab0, tpl_142::<0x1050> as *const (), 1), // +0x2baab0 sub_2baab0
    Lifted::new(0x70009a6ad0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xbf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2baad0 sub_2baad0
    Lifted::new(0x70009a6ba0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x307, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2baba0 sub_2baba0
    Lifted::new(0x70009a6c70, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xc3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bac70 sub_2bac70
    Lifted::new(0x70009a6d40, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x30b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bad40 sub_2bad40
    Lifted::new(0x70009a6e10, tpl_142::<0x1058> as *const (), 1), // +0x2bae10 sub_2bae10
    Lifted::new(0x70009a6e30, tpl_142::<0x1060> as *const (), 1), // +0x2bae30 sub_2bae30
    Lifted::new(0x70009a6e50, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xc7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bae50 sub_2bae50
    Lifted::new(0x70009a6f20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x317, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2baf20 sub_2baf20
    Lifted::new(0x70009a6ff0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x31b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2baff0 sub_2baff0
    Lifted::new(0x70009a70c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x12b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bb0c0 sub_2bb0c0
    Lifted::new(0x70009a7190, tpl_142::<0xd90> as *const (), 1), // +0x2bb190 sub_2bb190
    Lifted::new(0x70009a71b0, tpl_142::<0x1078> as *const (), 1), // +0x2bb1b0 sub_2bb1b0
    Lifted::new(0x70009a71d0, tpl_142::<0x1080> as *const (), 1), // +0x2bb1d0 sub_2bb1d0
    Lifted::new(0x70009a71f0, tpl_142::<0x1070> as *const (), 1), // +0x2bb1f0 sub_2bb1f0
    Lifted::new(0x70009a7210, tpl_142::<0x1068> as *const (), 1), // +0x2bb210 sub_2bb210
    Lifted::new(0x70009a7230, tpl_142::<0xcb0> as *const (), 1), // +0x2bb230 sub_2bb230
    Lifted::new(0x70009a7250, tpl_142::<0x1088> as *const (), 1), // +0x2bb250 sub_2bb250
    Lifted::new(0x70009a7270, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x23, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bb270 sub_2bb270
    Lifted::new(0x70009a7340, tpl_142::<0x530> as *const (), 1), // +0x2bb340 sub_2bb340
    Lifted::new(0x70009a7360, tpl_309::<0x8, 0x7000766110, 0x70009a7388> as *const (), 0), // +0x2bb360 sub_2bb360
    Lifted::new(0x70009a73a0, tpl_309::<0x8, 0x7000766110, 0x70009a73c8> as *const (), 0), // +0x2bb3a0 sub_2bb3a0
    Lifted::new(0x70009a73e0, tpl_310::<0x8, 0x70006fe730, 0x70009a7409> as *const (), 0), // +0x2bb3e0 sub_2bb3e0
    Lifted::new(0x70009a7420, tpl_310::<0x8, 0x70006fe730, 0x70009a7449> as *const (), 0), // +0x2bb420 sub_2bb420
    Lifted::new(0x70009a7460, tpl_52::<0xb1> as *const (), 1), // +0x2bb460 sub_2bb460
    Lifted::new(0x70009a7470, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xcb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bb470 sub_2bb470
    Lifted::new(0x70009a7540, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2d7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bb540 sub_2bb540
    Lifted::new(0x70009a7610, tpl_142::<0x538> as *const (), 1), // +0x2bb610 sub_2bb610
    Lifted::new(0x70009a7630, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1fb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bb630 sub_2bb630
    Lifted::new(0x70009a7700, tpl_142::<0x540> as *const (), 1), // +0x2bb700 sub_2bb700
    Lifted::new(0x70009a7720, tpl_311::<0x8> as *const (), 1), // +0x2bb720 sub_2bb720
    Lifted::new(0x70009a7730, tpl_311::<0x8> as *const (), 1), // +0x2bb730 sub_2bb730
    Lifted::new(0x70009a7740, tpl_312::<0x8> as *const (), 1), // +0x2bb740 sub_2bb740
    Lifted::new(0x70009a7750, tpl_312::<0x8> as *const (), 1), // +0x2bb750 sub_2bb750
    Lifted::new(0x70009a7760, tpl_52::<0xcb78> as *const (), 1), // +0x2bb760 sub_2bb760
    Lifted::new(0x70009a7770, tpl_193::<0x5c> as *const (), 1), // +0x2bb770 sub_2bb770
    Lifted::new(0x70009a7780, tpl_36::<0xc830> as *const (), 1), // +0x2bb780 sub_2bb780
    Lifted::new(0x70009a7790, tpl_36::<0x1a0> as *const (), 1), // +0x2bb790 sub_2bb790
    Lifted::new(0x70009a77a0, tpl_142::<0xd98> as *const (), 1), // +0x2bb7a0 sub_2bb7a0
    Lifted::new(0x70009a77c0, tpl_142::<0x478> as *const (), 1), // +0x2bb7c0 sub_2bb7c0
    Lifted::new(0x70009a77e0, tpl_142::<0x3c0> as *const (), 1), // +0x2bb7e0 sub_2bb7e0
    Lifted::new(0x70009a7800, tpl_142::<0x1170> as *const (), 1), // +0x2bb800 sub_2bb800
    Lifted::new(0x70009a7820, tpl_187 as *const (), 1), // +0x2bb820 sub_2bb820
    Lifted::new(0x70009a7830, tpl_187 as *const (), 1), // +0x2bb830 sub_2bb830
    Lifted::new(0x70009a7840, tpl_187 as *const (), 1), // +0x2bb840 sub_2bb840
    Lifted::new(0x70009a7850, tpl_187 as *const (), 1), // +0x2bb850 sub_2bb850
    Lifted::new(0x70009a7860, tpl_191::<0xc0> as *const (), 1), // +0x2bb860 sub_2bb860
    Lifted::new(0x70009a7880, tpl_191::<0x1f0> as *const (), 1), // +0x2bb880 sub_2bb880
    Lifted::new(0x70009a78a0, tpl_191::<0x1f0> as *const (), 1), // +0x2bb8a0 sub_2bb8a0
    Lifted::new(0x70009a78c0, tpl_191::<0xc0> as *const (), 1), // +0x2bb8c0 sub_2bb8c0
    Lifted::new(0x70009a78e0, tpl_191::<0xc0> as *const (), 1), // +0x2bb8e0 sub_2bb8e0
    Lifted::new(0x70009a7900, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x6f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bb900 sub_2bb900
    Lifted::new(0x70009a79d0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xcf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bb9d0 sub_2bb9d0
    Lifted::new(0x70009a7aa0, tpl_142::<0x1c0> as *const (), 1), // +0x2bbaa0 sub_2bbaa0
    Lifted::new(0x70009a7ac0, tpl_36::<0xa730> as *const (), 1), // +0x2bbac0 sub_2bbac0
    Lifted::new(0x70009a7ad0, fn_70009a7ad0 as *const (), 0), // +0x2bbad0 sub_2bbad0
    Lifted::new(0x70009a7b00, tpl_142::<0x550> as *const (), 1), // +0x2bbb00 sub_2bbb00
    Lifted::new(0x70009a7b20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x6b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bbb20 sub_2bbb20
    Lifted::new(0x70009a7bf0, tpl_142::<0x568> as *const (), 1), // +0x2bbbf0 sub_2bbbf0
    Lifted::new(0x70009a7c10, tpl_47::<0x5b> as *const (), 1), // +0x2bbc10 sub_2bbc10
    Lifted::new(0x70009a7c20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1ff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bbc20 sub_2bbc20
    Lifted::new(0x70009a7cf0, tpl_313 as *const (), 1), // +0x2bbcf0 sub_2bbcf0
    Lifted::new(0x70009a7d00, tpl_52::<0xa688> as *const (), 1), // +0x2bbd00 sub_2bbd00
    Lifted::new(0x70009a7d10, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xd3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bbd10 sub_2bbd10
    Lifted::new(0x70009a7de0, tpl_142::<0x380> as *const (), 1), // +0x2bbde0 sub_2bbde0
    Lifted::new(0x70009a7e00, tpl_142::<0x578> as *const (), 1), // +0x2bbe00 sub_2bbe00
    Lifted::new(0x70009a7e20, tpl_142::<0x580> as *const (), 1), // +0x2bbe20 sub_2bbe20
    Lifted::new(0x70009a7e40, tpl_36::<0xa610> as *const (), 1), // +0x2bbe40 sub_2bbe40
    Lifted::new(0x70009a7e50, tpl_142::<0x588> as *const (), 1), // +0x2bbe50 sub_2bbe50
    Lifted::new(0x70009a7e70, tpl_142::<0xf30> as *const (), 1), // +0x2bbe70 sub_2bbe70
    Lifted::new(0x70009a7e90, tpl_142::<0xf38> as *const (), 1), // +0x2bbe90 sub_2bbe90
    Lifted::new(0x70009a7eb0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xd7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bbeb0 sub_2bbeb0
    Lifted::new(0x70009a7f80, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xdb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bbf80 sub_2bbf80
    Lifted::new(0x70009a8050, tpl_47::<0x180> as *const (), 1), // +0x2bc050 sub_2bc050
    Lifted::new(0x70009a8060, tpl_52::<0xa638> as *const (), 1), // +0x2bc060 sub_2bc060
    Lifted::new(0x70009a8070, tpl_47::<0xc> as *const (), 1), // +0x2bc070 sub_2bc070
    Lifted::new(0x70009a8080, tpl_39::<0x48> as *const (), 1), // +0x2bc080 sub_2bc080
    Lifted::new(0x70009a8090, tpl_39::<0x8> as *const (), 1), // +0x2bc090 sub_2bc090
    Lifted::new(0x70009a80a0, tpl_314::<0xf3> as *const (), 1), // +0x2bc0a0 sub_2bc0a0
    Lifted::new(0x70009a80b0, tpl_47::<0x7c> as *const (), 1), // +0x2bc0b0 sub_2bc0b0
    Lifted::new(0x70009a80c0, tpl_142::<0xda8> as *const (), 1), // +0x2bc0c0 sub_2bc0c0
    Lifted::new(0x70009a80e0, tpl_142::<0xda0> as *const (), 1), // +0x2bc0e0 sub_2bc0e0
    Lifted::new(0x70009a8100, tpl_47::<0xd> as *const (), 1), // +0x2bc100 sub_2bc100
    Lifted::new(0x70009a8110, tpl_30 as *const (), 1), // +0x2bc110 sub_2bc110
    Lifted::new(0x70009a8120, tpl_142::<0x590> as *const (), 1), // +0x2bc120 sub_2bc120
    Lifted::new(0x70009a8140, tpl_52::<0x3098> as *const (), 1), // +0x2bc140 sub_2bc140
    Lifted::new(0x70009a8150, tpl_52::<0xaf40> as *const (), 1), // +0x2bc150 sub_2bc150
    Lifted::new(0x70009a8160, tpl_142::<0x1040> as *const (), 1), // +0x2bc160 sub_2bc160
    Lifted::new(0x70009a8180, tpl_36::<0xd840> as *const (), 1), // +0x2bc180 sub_2bc180
    Lifted::new(0x70009a8190, tpl_52::<0x63f0> as *const (), 1), // +0x2bc190 sub_2bc190
    Lifted::new(0x70009a81a0, tpl_142::<0xd8> as *const (), 1), // +0x2bc1a0 sub_2bc1a0
    Lifted::new(0x70009a81c0, tpl_36::<0x18> as *const (), 1), // +0x2bc1c0 sub_2bc1c0
    Lifted::new(0x70009a81d0, fn_70009a81d0 as *const (), 0), // +0x2bc1d0 sub_2bc1d0
    Lifted::new(0x70009a8220, tpl_142::<0x598> as *const (), 1), // +0x2bc220 sub_2bc220
    Lifted::new(0x70009a8240, tpl_142::<0x5a0> as *const (), 1), // +0x2bc240 sub_2bc240
    Lifted::new(0x70009a8260, tpl_39::<0xc92c> as *const (), 1), // +0x2bc260 sub_2bc260
    Lifted::new(0x70009a8270, tpl_187 as *const (), 1), // +0x2bc270 sub_2bc270
    Lifted::new(0x70009a8280, tpl_142::<0x1d0> as *const (), 1), // +0x2bc280 sub_2bc280
    Lifted::new(0x70009a82a0, tpl_39::<0x10> as *const (), 1), // +0x2bc2a0 sub_2bc2a0
    Lifted::new(0x70009a82b0, tpl_315::<0x700182e6a0> as *const (), 1), // +0x2bc2b0 sub_2bc2b0
    Lifted::new(0x70009a82c0, tpl_36::<0x70> as *const (), 1), // +0x2bc2c0 sub_2bc2c0
    Lifted::new(0x70009a82d0, tpl_52::<0x3048> as *const (), 1), // +0x2bc2d0 sub_2bc2d0
    Lifted::new(0x70009a82e0, fn_70009a82e0 as *const (), 1), // +0x2bc2e0 sub_2bc2e0
    Lifted::new(0x70009a82f0, tpl_36::<0x3058> as *const (), 1), // +0x2bc2f0 sub_2bc2f0
    Lifted::new(0x70009a8300, tpl_52::<0x3058> as *const (), 1), // +0x2bc300 sub_2bc300
    Lifted::new(0x70009a8310, tpl_142::<0xdb0> as *const (), 1), // +0x2bc310 sub_2bc310
    Lifted::new(0x70009a8330, tpl_36::<0x38> as *const (), 1), // +0x2bc330 sub_2bc330
    Lifted::new(0x70009a8340, tpl_275::<0x90, 0x38> as *const (), 1), // +0x2bc340 sub_2bc340
    Lifted::new(0x70009a8350, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xdf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bc350 sub_2bc350
    Lifted::new(0x70009a8420, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xe3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bc420 sub_2bc420
    Lifted::new(0x70009a84f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xe7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bc4f0 sub_2bc4f0
    Lifted::new(0x70009a85c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xeb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bc5c0 sub_2bc5c0
    Lifted::new(0x70009a8690, tpl_142::<0xba0> as *const (), 1), // +0x2bc690 sub_2bc690
    Lifted::new(0x70009a86b0, tpl_142::<0xd30> as *const (), 1), // +0x2bc6b0 sub_2bc6b0
    Lifted::new(0x70009a86d0, tpl_142::<0xd38> as *const (), 1), // +0x2bc6d0 sub_2bc6d0
    Lifted::new(0x70009a86f0, tpl_142::<0x5b0> as *const (), 1), // +0x2bc6f0 sub_2bc6f0
    Lifted::new(0x70009a8710, tpl_142::<0x5b8> as *const (), 1), // +0x2bc710 sub_2bc710
    Lifted::new(0x70009a8730, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xef, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bc730 sub_2bc730
    Lifted::new(0x70009a8800, tpl_208::<0x8> as *const (), 1), // +0x2bc800 sub_2bc800
    Lifted::new(0x70009a8810, tpl_36::<0xd900> as *const (), 1), // +0x2bc810 sub_2bc810
    Lifted::new(0x70009a8820, tpl_47::<0x58> as *const (), 1), // +0x2bc820 sub_2bc820
    Lifted::new(0x70009a8830, tpl_47::<0x10> as *const (), 1), // +0x2bc830 sub_2bc830
    Lifted::new(0x70009a8840, tpl_316::<0x10> as *const (), 1), // +0x2bc840 sub_2bc840
    Lifted::new(0x70009a8850, tpl_317::<0x10> as *const (), 1), // +0x2bc850 sub_2bc850
    Lifted::new(0x70009a8860, tpl_36::<0x8> as *const (), 1), // +0x2bc860 sub_2bc860
    Lifted::new(0x70009a8870, tpl_39::<0x20> as *const (), 1), // +0x2bc870 sub_2bc870
    Lifted::new(0x70009a8880, tpl_36::<0x10> as *const (), 1), // +0x2bc880 sub_2bc880
    Lifted::new(0x70009a8890, tpl_14 as *const (), 1), // +0x2bc890 sub_2bc890
    Lifted::new(0x70009a88a0, tpl_14 as *const (), 1), // +0x2bc8a0 sub_2bc8a0
    Lifted::new(0x70009a88b0, tpl_14 as *const (), 1), // +0x2bc8b0 sub_2bc8b0
    Lifted::new(0x70009a88c0, tpl_14 as *const (), 1), // +0x2bc8c0 sub_2bc8c0
    Lifted::new(0x70009a88d0, tpl_14 as *const (), 1), // +0x2bc8d0 sub_2bc8d0
    Lifted::new(0x70009a88e0, tpl_14 as *const (), 1), // +0x2bc8e0 sub_2bc8e0
    Lifted::new(0x70009a88f0, tpl_14 as *const (), 1), // +0x2bc8f0 sub_2bc8f0
    Lifted::new(0x70009a8900, tpl_14 as *const (), 1), // +0x2bc900 sub_2bc900
    Lifted::new(0x70009a8910, tpl_14 as *const (), 1), // +0x2bc910 sub_2bc910
    Lifted::new(0x70009a8920, tpl_14 as *const (), 1), // +0x2bc920 sub_2bc920
    Lifted::new(0x70009a8930, tpl_14 as *const (), 1), // +0x2bc930 sub_2bc930
    Lifted::new(0x70009a8940, tpl_14 as *const (), 1), // +0x2bc940 sub_2bc940
    Lifted::new(0x70009a8950, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x203, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bc950 sub_2bc950
    Lifted::new(0x70009a8a20, tpl_142::<0x5c0> as *const (), 1), // +0x2bca20 sub_2bca20
    Lifted::new(0x70009a8a40, tpl_187 as *const (), 1), // +0x2bca40 sub_2bca40
    Lifted::new(0x70009a8a50, tpl_187 as *const (), 1), // +0x2bca50 sub_2bca50
    Lifted::new(0x70009a8a60, tpl_142::<0x128> as *const (), 1), // +0x2bca60 sub_2bca60
    Lifted::new(0x70009a8a80, tpl_20 as *const (), 1), // +0x2bca80 sub_2bca80
    Lifted::new(0x70009a8a90, tpl_191::<0x8> as *const (), 1), // +0x2bca90 sub_2bca90
    Lifted::new(0x70009a8aa0, tpl_191::<0x8> as *const (), 1), // +0x2bcaa0 sub_2bcaa0
    Lifted::new(0x70009a8ab0, tpl_36::<0x40> as *const (), 1), // +0x2bcab0 sub_2bcab0
    Lifted::new(0x70009a8ac0, tpl_142::<0x5c8> as *const (), 1), // +0x2bcac0 sub_2bcac0
    Lifted::new(0x70009a8ae0, tpl_194 as *const (), 1), // +0x2bcae0 sub_2bcae0
    Lifted::new(0x70009a8af0, tpl_318 as *const (), 1), // +0x2bcaf0 sub_2bcaf0
    Lifted::new(0x70009a8b00, tpl_142::<0xbb8> as *const (), 1), // +0x2bcb00 sub_2bcb00
    Lifted::new(0x70009a8b20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x31f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bcb20 sub_2bcb20
    Lifted::new(0x70009a8bf0, tpl_142::<0xdb8> as *const (), 1), // +0x2bcbf0 sub_2bcbf0
    Lifted::new(0x70009a8c10, tpl_142::<0xbc0> as *const (), 1), // +0x2bcc10 sub_2bcc10
    Lifted::new(0x70009a8c30, tpl_319 as *const (), 1), // +0x2bcc30 sub_2bcc30
    Lifted::new(0x70009a8c40, tpl_320::<0x10, 0x38, 0x8, 0x70009a48b0, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x2bcc40 sub_2bcc40
    Lifted::new(0x70009a8d30, tpl_321::<0x10, 0x38, 0x8, 0x70009a4ae0, 0x700177c312, 0x18, 0x20> as *const (), 0), // +0x2bcd30 sub_2bcd30
    Lifted::new(0x70009a8e20, fn_70009a8e20 as *const (), 0), // +0x2bce20 sub_2bce20
    Lifted::new(0x70009a8eb0, fn_70009a8eb0 as *const (), 1), // +0x2bceb0 sub_2bceb0
    Lifted::new(0x70009a8ec0, fn_70009a8ec0 as *const (), 0), // +0x2bcec0 sub_2bcec0
    Lifted::new(0x70009a8ef0, tpl_15 as *const (), 1), // +0x2bcef0 sub_2bcef0
    Lifted::new(0x70009a8f00, tpl_322::<0xa8, 0x2ff8> as *const (), 1), // +0x2bcf00 sub_2bcf00
    Lifted::new(0x70009a8f20, tpl_322::<0xa8, 0x3030> as *const (), 1), // +0x2bcf20 sub_2bcf20
    Lifted::new(0x70009a8f40, tpl_322::<0xa8, 0x3040> as *const (), 1), // +0x2bcf40 sub_2bcf40
    Lifted::new(0x70009a8f60, tpl_244::<0x70> as *const (), 1), // +0x2bcf60 sub_2bcf60
    Lifted::new(0x70009a8f70, tpl_52::<0xd9d0> as *const (), 1), // +0x2bcf70 sub_2bcf70
    Lifted::new(0x70009a8f80, tpl_36::<0x20> as *const (), 1), // +0x2bcf80 sub_2bcf80
    Lifted::new(0x70009a8f90, tpl_142::<0x1a8> as *const (), 1), // +0x2bcf90 sub_2bcf90
    Lifted::new(0x70009a8fb0, tpl_142::<0x5d0> as *const (), 1), // +0x2bcfb0 sub_2bcfb0
    Lifted::new(0x70009a8fd0, tpl_39::<0xc928> as *const (), 1), // +0x2bcfd0 sub_2bcfd0
    Lifted::new(0x70009a8fe0, tpl_39::<0x48> as *const (), 1), // +0x2bcfe0 sub_2bcfe0
    Lifted::new(0x70009a8ff0, tpl_39::<0xc7c0> as *const (), 1), // +0x2bcff0 sub_2bcff0
    Lifted::new(0x70009a9000, tpl_142::<0x1d8> as *const (), 1), // +0x2bd000 sub_2bd000
    Lifted::new(0x70009a9020, tpl_36::<0xc800> as *const (), 1), // +0x2bd020 sub_2bd020
    Lifted::new(0x70009a9030, tpl_142::<0x120> as *const (), 1), // +0x2bd030 sub_2bd030
    Lifted::new(0x70009a9050, tpl_39::<0x38> as *const (), 1), // +0x2bd050 sub_2bd050
    Lifted::new(0x70009a9060, tpl_39::<0x20> as *const (), 1), // +0x2bd060 sub_2bd060
    Lifted::new(0x70009a9070, tpl_142::<0x5d8> as *const (), 1), // +0x2bd070 sub_2bd070
    Lifted::new(0x70009a9090, tpl_36::<0xc910> as *const (), 1), // +0x2bd090 sub_2bd090
    Lifted::new(0x70009a90a0, tpl_323 as *const (), 1), // +0x2bd0a0 sub_2bd0a0
    Lifted::new(0x70009a90b0, tpl_324 as *const (), 1), // +0x2bd0b0 sub_2bd0b0
    Lifted::new(0x70009a90c0, tpl_142::<0x5e0> as *const (), 1), // +0x2bd0c0 sub_2bd0c0
    Lifted::new(0x70009a90e0, tpl_36::<0x70> as *const (), 1), // +0x2bd0e0 sub_2bd0e0
    Lifted::new(0x70009a90f0, tpl_36::<0x8> as *const (), 1), // +0x2bd0f0 sub_2bd0f0
    Lifted::new(0x70009a9100, tpl_39::<0x114> as *const (), 1), // +0x2bd100 sub_2bd100
    Lifted::new(0x70009a9110, tpl_47::<0xc975> as *const (), 1), // +0x2bd110 sub_2bd110
    Lifted::new(0x70009a9120, tpl_36::<0xa620> as *const (), 1), // +0x2bd120 sub_2bd120
    Lifted::new(0x70009a9130, tpl_52::<0x78> as *const (), 1), // +0x2bd130 sub_2bd130
    Lifted::new(0x70009a9140, tpl_36::<0xe0> as *const (), 1), // +0x2bd140 sub_2bd140
    Lifted::new(0x70009a9150, tpl_52::<0x10> as *const (), 1), // +0x2bd150 sub_2bd150
    Lifted::new(0x70009a9160, tpl_36::<0xc818> as *const (), 1), // +0x2bd160 sub_2bd160
    Lifted::new(0x70009a9170, tpl_142::<0x5e8> as *const (), 1), // +0x2bd170 sub_2bd170
    Lifted::new(0x70009a9190, tpl_172::<0xd758> as *const (), 1), // +0x2bd190 sub_2bd190
    Lifted::new(0x70009a91a0, tpl_52::<0x48> as *const (), 1), // +0x2bd1a0 sub_2bd1a0
    Lifted::new(0x70009a91b0, tpl_142::<0x5f0> as *const (), 1), // +0x2bd1b0 sub_2bd1b0
    Lifted::new(0x70009a91d0, tpl_142::<0x5f8> as *const (), 1), // +0x2bd1d0 sub_2bd1d0
    Lifted::new(0x70009a91f0, tpl_142::<0x2f8> as *const (), 1), // +0x2bd1f0 sub_2bd1f0
    Lifted::new(0x70009a9210, tpl_142::<0x300> as *const (), 1), // +0x2bd210 sub_2bd210
    Lifted::new(0x70009a9230, tpl_142::<0xba8> as *const (), 1), // +0x2bd230 sub_2bd230
    Lifted::new(0x70009a9250, tpl_142::<0xbb0> as *const (), 1), // +0x2bd250 sub_2bd250
    Lifted::new(0x70009a9270, tpl_36::<0xd908> as *const (), 1), // +0x2bd270 sub_2bd270
    Lifted::new(0x70009a9280, tpl_39::<0x34> as *const (), 1), // +0x2bd280 sub_2bd280
    Lifted::new(0x70009a9290, fn_70009a9290 as *const (), 1), // +0x2bd290 sub_2bd290
    Lifted::new(0x70009a92a0, tpl_142::<0x600> as *const (), 1), // +0x2bd2a0 sub_2bd2a0
    Lifted::new(0x70009a92c0, tpl_142::<0x1198> as *const (), 1), // +0x2bd2c0 sub_2bd2c0
    Lifted::new(0x70009a92e0, tpl_142::<0x11a0> as *const (), 1), // +0x2bd2e0 sub_2bd2e0
    Lifted::new(0x70009a9300, tpl_142::<0x608> as *const (), 1), // +0x2bd300 sub_2bd300
    Lifted::new(0x70009a9320, tpl_36::<0x48> as *const (), 1), // +0x2bd320 sub_2bd320
    Lifted::new(0x70009a9330, fn_70009a9330 as *const (), 1), // +0x2bd330 sub_2bd330
    Lifted::new(0x70009a9360, tpl_325::<0x78> as *const (), 1), // +0x2bd360 sub_2bd360
    Lifted::new(0x70009a9370, tpl_191::<0x2fe8> as *const (), 1), // +0x2bd370 sub_2bd370
    Lifted::new(0x70009a9390, tpl_191::<0x8> as *const (), 1), // +0x2bd390 sub_2bd390
    Lifted::new(0x70009a93a0, tpl_52::<0x2fe8> as *const (), 1), // +0x2bd3a0 sub_2bd3a0
    Lifted::new(0x70009a93b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xf3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bd3b0 sub_2bd3b0
    Lifted::new(0x70009a9480, tpl_52::<0x48> as *const (), 1), // +0x2bd480 sub_2bd480
    Lifted::new(0x70009a9490, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bd490 sub_2bd490
    Lifted::new(0x70009a9560, tpl_36::<0xa628> as *const (), 1), // +0x2bd560 sub_2bd560
    Lifted::new(0x70009a9570, tpl_326 as *const (), 1), // +0x2bd570 sub_2bd570
    Lifted::new(0x70009a9580, tpl_142::<0x1e0> as *const (), 1), // +0x2bd580 sub_2bd580
    Lifted::new(0x70009a95a0, tpl_191::<0x8> as *const (), 1), // +0x2bd5a0 sub_2bd5a0
    Lifted::new(0x70009a95b0, tpl_191::<0x8> as *const (), 1), // +0x2bd5b0 sub_2bd5b0
    Lifted::new(0x70009a95c0, tpl_39::<0x20> as *const (), 1), // +0x2bd5c0 sub_2bd5c0
    Lifted::new(0x70009a95d0, tpl_187 as *const (), 1), // +0x2bd5d0 sub_2bd5d0
    Lifted::new(0x70009a95e0, tpl_187 as *const (), 1), // +0x2bd5e0 sub_2bd5e0
    Lifted::new(0x70009a95f0, tpl_327::<0x18> as *const (), 1), // +0x2bd5f0 sub_2bd5f0
    Lifted::new(0x70009a9600, tpl_39::<0x28> as *const (), 1), // +0x2bd600 sub_2bd600
    Lifted::new(0x70009a9610, tpl_142::<0x1150> as *const (), 1), // +0x2bd610 sub_2bd610
    Lifted::new(0x70009a9630, fn_70009a9630 as *const (), 1), // +0x2bd630 sub_2bd630
    Lifted::new(0x70009a9640, tpl_47::<0x20> as *const (), 1), // +0x2bd640 sub_2bd640
    Lifted::new(0x70009a9650, tpl_36::<0x20> as *const (), 1), // +0x2bd650 sub_2bd650
    Lifted::new(0x70009a9660, tpl_36::<0x18> as *const (), 1), // +0x2bd660 sub_2bd660
    Lifted::new(0x70009a9670, tpl_142::<0x610> as *const (), 1), // +0x2bd670 sub_2bd670
    Lifted::new(0x70009a9690, tpl_39::<0x3080> as *const (), 1), // +0x2bd690 sub_2bd690
    Lifted::new(0x70009a96a0, tpl_30 as *const (), 1), // +0x2bd6a0 sub_2bd6a0
    Lifted::new(0x70009a96b0, tpl_30 as *const (), 1), // +0x2bd6b0 sub_2bd6b0
    Lifted::new(0x70009a96c0, tpl_30 as *const (), 1), // +0x2bd6c0 sub_2bd6c0
    Lifted::new(0x70009a96d0, tpl_30 as *const (), 1), // +0x2bd6d0 sub_2bd6d0
    Lifted::new(0x70009a96e0, tpl_36::<0x8> as *const (), 1), // +0x2bd6e0 sub_2bd6e0
    Lifted::new(0x70009a96f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xf7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bd6f0 sub_2bd6f0
    Lifted::new(0x70009a97c0, tpl_328::<0x9f> as *const (), 1), // +0x2bd7c0 sub_2bd7c0
    Lifted::new(0x70009a97d0, tpl_328::<0x97> as *const (), 1), // +0x2bd7d0 sub_2bd7d0
    Lifted::new(0x70009a97e0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xfb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bd7e0 sub_2bd7e0
    Lifted::new(0x70009a98b0, tpl_36::<0xc778> as *const (), 1), // +0x2bd8b0 sub_2bd8b0
    Lifted::new(0x70009a98c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0xff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bd8c0 sub_2bd8c0
    Lifted::new(0x70009a9990, tpl_142::<0x620> as *const (), 1), // +0x2bd990 sub_2bd990
    Lifted::new(0x70009a99b0, tpl_36::<0xc7f0> as *const (), 1), // +0x2bd9b0 sub_2bd9b0
    Lifted::new(0x70009a99c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x103, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bd9c0 sub_2bd9c0
    Lifted::new(0x70009a9a90, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x20b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bda90 sub_2bda90
    Lifted::new(0x70009a9b60, tpl_47::<0xc941> as *const (), 1), // +0x2bdb60 sub_2bdb60
    Lifted::new(0x70009a9b70, tpl_52::<0xc941> as *const (), 1), // +0x2bdb70 sub_2bdb70
    Lifted::new(0x70009a9b80, tpl_142::<0xdc0> as *const (), 1), // +0x2bdb80 sub_2bdb80
    Lifted::new(0x70009a9ba0, tpl_36::<0xc900> as *const (), 1), // +0x2bdba0 sub_2bdba0
    Lifted::new(0x70009a9bb0, tpl_142::<0x628> as *const (), 1), // +0x2bdbb0 sub_2bdbb0
    Lifted::new(0x70009a9bd0, tpl_142::<0x630> as *const (), 1), // +0x2bdbd0 sub_2bdbd0
    Lifted::new(0x70009a9bf0, tpl_142::<0x638> as *const (), 1), // +0x2bdbf0 sub_2bdbf0
    Lifted::new(0x70009a9c10, tpl_329 as *const (), 1), // +0x2bdc10 sub_2bdc10
    Lifted::new(0x70009a9c20, tpl_47::<0xc974> as *const (), 1), // +0x2bdc20 sub_2bdc20
    Lifted::new(0x70009a9c30, tpl_47::<0xa6b0> as *const (), 1), // +0x2bdc30 sub_2bdc30
    Lifted::new(0x70009a9c40, tpl_142::<0xe58> as *const (), 1), // +0x2bdc40 sub_2bdc40
    Lifted::new(0x70009a9c60, tpl_36::<0xa6d0> as *const (), 1), // +0x2bdc60 sub_2bdc60
    Lifted::new(0x70009a9c70, tpl_191::<0x50> as *const (), 1), // +0x2bdc70 sub_2bdc70
    Lifted::new(0x70009a9c80, tpl_314::<0xc6c> as *const (), 1), // +0x2bdc80 sub_2bdc80
    Lifted::new(0x70009a9c90, tpl_330::<0x8, 0x10> as *const (), 1), // +0x2bdc90 sub_2bdc90
    Lifted::new(0x70009a9cb0, tpl_142::<0x1120> as *const (), 1), // +0x2bdcb0 sub_2bdcb0
    Lifted::new(0x70009a9cd0, tpl_47::<0xd750> as *const (), 1), // +0x2bdcd0 sub_2bdcd0
    Lifted::new(0x70009a9ce0, tpl_142::<0xbc8> as *const (), 1), // +0x2bdce0 sub_2bdce0
    Lifted::new(0x70009a9d00, tpl_191::<0x50> as *const (), 1), // +0x2bdd00 sub_2bdd00
    Lifted::new(0x70009a9d10, tpl_47::<0xc970> as *const (), 1), // +0x2bdd10 sub_2bdd10
    Lifted::new(0x70009a9d20, tpl_47::<0xc948> as *const (), 1), // +0x2bdd20 sub_2bdd20
    Lifted::new(0x70009a9d30, tpl_142::<0x640> as *const (), 1), // +0x2bdd30 sub_2bdd30
    Lifted::new(0x70009a9d50, fn_70009a9d50 as *const (), 1), // +0x2bdd50 sub_2bdd50
    Lifted::new(0x70009a9d60, fn_70009a9d60 as *const (), 1), // +0x2bdd60 sub_2bdd60
    Lifted::new(0x70009a9d70, tpl_47::<0x18> as *const (), 1), // +0x2bdd70 sub_2bdd70
    Lifted::new(0x70009a9d80, tpl_169::<0x18> as *const (), 1), // +0x2bdd80 sub_2bdd80
    Lifted::new(0x70009a9d90, tpl_142::<0x648> as *const (), 1), // +0x2bdd90 sub_2bdd90
    Lifted::new(0x70009a9db0, tpl_142::<0x6a8> as *const (), 1), // +0x2bddb0 sub_2bddb0
    Lifted::new(0x70009a9dd0, tpl_142::<0x650> as *const (), 1), // +0x2bddd0 sub_2bddd0
    Lifted::new(0x70009a9df0, tpl_142::<0x658> as *const (), 1), // +0x2bddf0 sub_2bddf0
    Lifted::new(0x70009a9e10, tpl_142::<0x660> as *const (), 1), // +0x2bde10 sub_2bde10
    Lifted::new(0x70009a9e30, tpl_142::<0x668> as *const (), 1), // +0x2bde30 sub_2bde30
    Lifted::new(0x70009a9e50, tpl_142::<0x670> as *const (), 1), // +0x2bde50 sub_2bde50
    Lifted::new(0x70009a9e70, tpl_142::<0x678> as *const (), 1), // +0x2bde70 sub_2bde70
    Lifted::new(0x70009a9e90, tpl_142::<0x688> as *const (), 1), // +0x2bde90 sub_2bde90
    Lifted::new(0x70009a9eb0, tpl_142::<0x680> as *const (), 1), // +0x2bdeb0 sub_2bdeb0
    Lifted::new(0x70009a9ed0, tpl_142::<0x690> as *const (), 1), // +0x2bded0 sub_2bded0
    Lifted::new(0x70009a9ef0, tpl_142::<0x698> as *const (), 1), // +0x2bdef0 sub_2bdef0
    Lifted::new(0x70009a9f10, tpl_142::<0x6a0> as *const (), 1), // +0x2bdf10 sub_2bdf10
    Lifted::new(0x70009a9f30, tpl_328::<0xa7> as *const (), 1), // +0x2bdf30 sub_2bdf30
    Lifted::new(0x70009a9f40, tpl_142::<0x6b8> as *const (), 1), // +0x2bdf40 sub_2bdf40
    Lifted::new(0x70009a9f60, tpl_36::<0xd950> as *const (), 1), // +0x2bdf60 sub_2bdf60
    Lifted::new(0x70009a9f70, tpl_142::<0xbd0> as *const (), 1), // +0x2bdf70 sub_2bdf70
    Lifted::new(0x70009a9f90, tpl_52::<0x30> as *const (), 1), // +0x2bdf90 sub_2bdf90
    Lifted::new(0x70009a9fa0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x17, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bdfa0 sub_2bdfa0
    Lifted::new(0x70009aa070, tpl_142::<0x2e0> as *const (), 1), // +0x2be070 sub_2be070
    Lifted::new(0x70009aa090, tpl_39::<0xc960> as *const (), 1), // +0x2be090 sub_2be090
    Lifted::new(0x70009aa0a0, tpl_39::<0xc95c> as *const (), 1), // +0x2be0a0 sub_2be0a0
    Lifted::new(0x70009aa0b0, tpl_331::<0x8> as *const (), 1), // +0x2be0b0 sub_2be0b0
    Lifted::new(0x70009aa0c0, tpl_331::<0x8> as *const (), 1), // +0x2be0c0 sub_2be0c0
    Lifted::new(0x70009aa0d0, tpl_208::<0x10> as *const (), 1), // +0x2be0d0 sub_2be0d0
    Lifted::new(0x70009aa0e0, tpl_263::<0x1c> as *const (), 1), // +0x2be0e0 sub_2be0e0
    Lifted::new(0x70009aa0f0, tpl_142::<0x3e8> as *const (), 1), // +0x2be0f0 sub_2be0f0
    Lifted::new(0x70009aa110, tpl_142::<0x3d8> as *const (), 1), // +0x2be110 sub_2be110
    Lifted::new(0x70009aa130, tpl_142::<0x3f0> as *const (), 1), // +0x2be130 sub_2be130
    Lifted::new(0x70009aa150, tpl_142::<0x170> as *const (), 1), // +0x2be150 sub_2be150
    Lifted::new(0x70009aa170, tpl_70 as *const (), 1), // +0x2be170 sub_2be170
    Lifted::new(0x70009aa180, tpl_142::<0x3c8> as *const (), 1), // +0x2be180 sub_2be180
    Lifted::new(0x70009aa1a0, tpl_142::<0x410> as *const (), 1), // +0x2be1a0 sub_2be1a0
    Lifted::new(0x70009aa1c0, tpl_142::<0x168> as *const (), 1), // +0x2be1c0 sub_2be1c0
    Lifted::new(0x70009aa1e0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x107, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2be1e0 sub_2be1e0
    Lifted::new(0x70009aa2b0, tpl_142::<0x4a0> as *const (), 1), // +0x2be2b0 sub_2be2b0
    Lifted::new(0x70009aa2d0, tpl_142::<0x3e0> as *const (), 1), // +0x2be2d0 sub_2be2d0
    Lifted::new(0x70009aa2f0, tpl_142::<0x400> as *const (), 1), // +0x2be2f0 sub_2be2f0
    Lifted::new(0x70009aa310, tpl_142::<0x408> as *const (), 1), // +0x2be310 sub_2be310
    Lifted::new(0x70009aa330, tpl_142::<0x420> as *const (), 1), // +0x2be330 sub_2be330
    Lifted::new(0x70009aa350, tpl_142::<0x3d0> as *const (), 1), // +0x2be350 sub_2be350
    Lifted::new(0x70009aa370, tpl_142::<0x418> as *const (), 1), // +0x2be370 sub_2be370
    Lifted::new(0x70009aa390, tpl_142::<0x160> as *const (), 1), // +0x2be390 sub_2be390
    Lifted::new(0x70009aa3b0, tpl_142::<0xf88> as *const (), 1), // +0x2be3b0 sub_2be3b0
    Lifted::new(0x70009aa3d0, fn_70009aa3d0 as *const (), 1), // +0x2be3d0 sub_2be3d0
    Lifted::new(0x70009aa400, tpl_142::<0x3f8> as *const (), 1), // +0x2be400 sub_2be400
    Lifted::new(0x70009aa420, tpl_142::<0xc8> as *const (), 1), // +0x2be420 sub_2be420
    Lifted::new(0x70009aa440, tpl_142::<0x428> as *const (), 1), // +0x2be440 sub_2be440
    Lifted::new(0x70009aa460, tpl_142::<0x440> as *const (), 1), // +0x2be460 sub_2be460
    Lifted::new(0x70009aa480, tpl_142::<0x438> as *const (), 1), // +0x2be480 sub_2be480
    Lifted::new(0x70009aa4a0, tpl_171::<0xc7e1> as *const (), 1), // +0x2be4a0 sub_2be4a0
    Lifted::new(0x70009aa4b0, tpl_52::<0x98> as *const (), 1), // +0x2be4b0 sub_2be4b0
    Lifted::new(0x70009aa4c0, tpl_39::<0x20> as *const (), 1), // +0x2be4c0 sub_2be4c0
    Lifted::new(0x70009aa4d0, tpl_191::<0x8> as *const (), 1), // +0x2be4d0 sub_2be4d0
    Lifted::new(0x70009aa4e0, tpl_191::<0x8> as *const (), 1), // +0x2be4e0 sub_2be4e0
    Lifted::new(0x70009aa4f0, tpl_191::<0x8> as *const (), 1), // +0x2be4f0 sub_2be4f0
    Lifted::new(0x70009aa500, tpl_191::<0x8> as *const (), 1), // +0x2be500 sub_2be500
    Lifted::new(0x70009aa510, tpl_42 as *const (), 1), // +0x2be510 sub_2be510
    Lifted::new(0x70009aa520, tpl_332::<0x8> as *const (), 1), // +0x2be520 sub_2be520
    Lifted::new(0x70009aa530, tpl_42 as *const (), 1), // +0x2be530 sub_2be530
    Lifted::new(0x70009aa540, tpl_42 as *const (), 1), // +0x2be540 sub_2be540
    Lifted::new(0x70009aa550, tpl_42 as *const (), 1), // +0x2be550 sub_2be550
    Lifted::new(0x70009aa560, tpl_42 as *const (), 1), // +0x2be560 sub_2be560
    Lifted::new(0x70009aa570, tpl_39::<0xc> as *const (), 1), // +0x2be570 sub_2be570
    Lifted::new(0x70009aa580, fn_70009aa580 as *const (), 1), // +0x2be580 sub_2be580
    Lifted::new(0x70009aa590, tpl_142::<0xdc8> as *const (), 1), // +0x2be590 sub_2be590
    Lifted::new(0x70009aa5b0, tpl_142::<0x6b0> as *const (), 1), // +0x2be5b0 sub_2be5b0
    Lifted::new(0x70009aa5d0, tpl_142::<0x2d8> as *const (), 1), // +0x2be5d0 sub_2be5d0
    Lifted::new(0x70009aa5f0, fn_70009aa5f0 as *const (), 0), // +0x2be5f0 sub_2be5f0
    Lifted::new(0x70009aa680, fn_70009aa680 as *const (), 0), // +0x2be680 sub_2be680
    Lifted::new(0x70009aa6e0, fn_70009aa6e0 as *const (), 0), // +0x2be6e0 sub_2be6e0
    Lifted::new(0x70009aa780, fn_70009aa780 as *const (), 0), // +0x2be780 sub_2be780
    Lifted::new(0x70009aa850, tpl_142::<0xbd8> as *const (), 1), // +0x2be850 sub_2be850
    Lifted::new(0x70009aa870, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2be870 sub_2be870
    Lifted::new(0x70009aa940, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x10b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2be940 sub_2be940
    Lifted::new(0x70009aaa10, tpl_142::<0xbe0> as *const (), 1), // +0x2bea10 sub_2bea10
    Lifted::new(0x70009aaa30, tpl_142::<0xf40> as *const (), 1), // +0x2bea30 sub_2bea30
    Lifted::new(0x70009aaa50, tpl_142::<0xbe8> as *const (), 1), // +0x2bea50 sub_2bea50
    Lifted::new(0x70009aaa70, tpl_142::<0x6d0> as *const (), 1), // +0x2bea70 sub_2bea70
    Lifted::new(0x70009aaa90, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bea90 sub_2bea90
    Lifted::new(0x70009aab60, tpl_142::<0x6c8> as *const (), 1), // +0x2beb60 sub_2beb60
    Lifted::new(0x70009aab80, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x10f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2beb80 sub_2beb80
    Lifted::new(0x70009aac50, tpl_36::<0xa720> as *const (), 1), // +0x2bec50 sub_2bec50
    Lifted::new(0x70009aac60, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1f3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bec60 sub_2bec60
    Lifted::new(0x70009aad30, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x373, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bed30 sub_2bed30
    Lifted::new(0x70009aae00, tpl_142::<0x6d8> as *const (), 1), // +0x2bee00 sub_2bee00
    Lifted::new(0x70009aae20, tpl_36::<0xc870> as *const (), 1), // +0x2bee20 sub_2bee20
    Lifted::new(0x70009aae30, tpl_142::<0x180> as *const (), 1), // +0x2bee30 sub_2bee30
    Lifted::new(0x70009aae50, tpl_36::<0xc860> as *const (), 1), // +0x2bee50 sub_2bee50
    Lifted::new(0x70009aae60, tpl_142::<0x398> as *const (), 1), // +0x2bee60 sub_2bee60
    Lifted::new(0x70009aae80, fn_70009aae80 as *const (), 1), // +0x2bee80 sub_2bee80
    Lifted::new(0x70009aae90, tpl_142::<0x6e8> as *const (), 1), // +0x2bee90 sub_2bee90
    Lifted::new(0x70009aaeb0, tpl_39::<0x110> as *const (), 1), // +0x2beeb0 sub_2beeb0
    Lifted::new(0x70009aaec0, tpl_142::<0xec8> as *const (), 1), // +0x2beec0 sub_2beec0
    Lifted::new(0x70009aaee0, tpl_142::<0xe98> as *const (), 1), // +0x2beee0 sub_2beee0
    Lifted::new(0x70009aaf00, tpl_142::<0xea0> as *const (), 1), // +0x2bef00 sub_2bef00
    Lifted::new(0x70009aaf20, tpl_333::<0xc3> as *const (), 1), // +0x2bef20 sub_2bef20
    Lifted::new(0x70009aaf40, tpl_334::<0xc3> as *const (), 1), // +0x2bef40 sub_2bef40
    Lifted::new(0x70009aaf70, tpl_36::<0x20> as *const (), 1), // +0x2bef70 sub_2bef70
    Lifted::new(0x70009aaf80, tpl_36::<0x3078> as *const (), 1), // +0x2bef80 sub_2bef80
    Lifted::new(0x70009aaf90, tpl_47::<0x3039> as *const (), 1), // +0x2bef90 sub_2bef90
    Lifted::new(0x70009aafa0, tpl_52::<0x3039> as *const (), 1), // +0x2befa0 sub_2befa0
    Lifted::new(0x70009aafb0, tpl_142::<0x340> as *const (), 1), // +0x2befb0 sub_2befb0
    Lifted::new(0x70009aafd0, tpl_142::<0xf78> as *const (), 1), // +0x2befd0 sub_2befd0
    Lifted::new(0x70009aaff0, tpl_142::<0x348> as *const (), 1), // +0x2beff0 sub_2beff0
    Lifted::new(0x70009ab010, tpl_142::<0x328> as *const (), 1), // +0x2bf010 sub_2bf010
    Lifted::new(0x70009ab030, tpl_36::<0xc8f0> as *const (), 1), // +0x2bf030 sub_2bf030
    Lifted::new(0x70009ab040, tpl_52::<0x11b0> as *const (), 1), // +0x2bf040 sub_2bf040
    Lifted::new(0x70009ab050, tpl_39::<0xc930> as *const (), 1), // +0x2bf050 sub_2bf050
    Lifted::new(0x70009ab060, tpl_142::<0x320> as *const (), 1), // +0x2bf060 sub_2bf060
    Lifted::new(0x70009ab080, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x113, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bf080 sub_2bf080
    Lifted::new(0x70009ab150, tpl_20 as *const (), 1), // +0x2bf150 sub_2bf150
    Lifted::new(0x70009ab160, tpl_20 as *const (), 1), // +0x2bf160 sub_2bf160
    Lifted::new(0x70009ab170, tpl_20 as *const (), 1), // +0x2bf170 sub_2bf170
    Lifted::new(0x70009ab180, tpl_20 as *const (), 1), // +0x2bf180 sub_2bf180
    Lifted::new(0x70009ab190, tpl_36::<0x18> as *const (), 1), // +0x2bf190 sub_2bf190
    Lifted::new(0x70009ab1a0, tpl_142::<0x6f0> as *const (), 1), // +0x2bf1a0 sub_2bf1a0
    Lifted::new(0x70009ab1c0, tpl_142::<0xc0> as *const (), 1), // +0x2bf1c0 sub_2bf1c0
    Lifted::new(0x70009ab1e0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x117, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bf1e0 sub_2bf1e0
    Lifted::new(0x70009ab2b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x11b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bf2b0 sub_2bf2b0
    Lifted::new(0x70009ab380, tpl_52::<0x30> as *const (), 1), // +0x2bf380 sub_2bf380
    Lifted::new(0x70009ab390, tpl_142::<0x1b0> as *const (), 1), // +0x2bf390 sub_2bf390
    Lifted::new(0x70009ab3b0, tpl_52::<0xa660> as *const (), 1), // +0x2bf3b0 sub_2bf3b0
    Lifted::new(0x70009ab3c0, tpl_142::<0x158> as *const (), 1), // +0x2bf3c0 sub_2bf3c0
    Lifted::new(0x70009ab3e0, tpl_142::<0x1130> as *const (), 1), // +0x2bf3e0 sub_2bf3e0
    Lifted::new(0x70009ab400, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x393, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bf400 sub_2bf400
    Lifted::new(0x70009ab4d0, tpl_47::<0x5c> as *const (), 1), // +0x2bf4d0 sub_2bf4d0
    Lifted::new(0x70009ab4e0, tpl_52::<0xb0> as *const (), 1), // +0x2bf4e0 sub_2bf4e0
    Lifted::new(0x70009ab4f0, tpl_275::<0x1f0, 0x18> as *const (), 1), // +0x2bf4f0 sub_2bf4f0
    Lifted::new(0x70009ab500, fn_70009ab500 as *const (), 0), // +0x2bf500 sub_2bf500
    Lifted::new(0x70009ab540, tpl_36::<0xc0> as *const (), 1), // +0x2bf540 sub_2bf540
    Lifted::new(0x70009ab550, tpl_36::<0x1f0> as *const (), 1), // +0x2bf550 sub_2bf550
    Lifted::new(0x70009ab560, tpl_36::<0xc0> as *const (), 1), // +0x2bf560 sub_2bf560
    Lifted::new(0x70009ab570, tpl_36::<0xc0> as *const (), 1), // +0x2bf570 sub_2bf570
    Lifted::new(0x70009ab580, tpl_36::<0xc0> as *const (), 1), // +0x2bf580 sub_2bf580
    Lifted::new(0x70009ab590, tpl_36::<0xc0> as *const (), 1), // +0x2bf590 sub_2bf590
    Lifted::new(0x70009ab5a0, tpl_259::<0x3e> as *const (), 1), // +0x2bf5a0 sub_2bf5a0
    Lifted::new(0x70009ab5b0, tpl_142::<0xe0> as *const (), 1), // +0x2bf5b0 sub_2bf5b0
    Lifted::new(0x70009ab5d0, tpl_142::<0xe8> as *const (), 1), // +0x2bf5d0 sub_2bf5d0
    Lifted::new(0x70009ab5f0, tpl_142::<0x1e8> as *const (), 1), // +0x2bf5f0 sub_2bf5f0
    Lifted::new(0x70009ab610, fn_70009ab610 as *const (), 3), // +0x2bf610 sub_2bf610
    Lifted::new(0x70009ab630, tpl_20 as *const (), 1), // +0x2bf630 sub_2bf630
    Lifted::new(0x70009ab640, tpl_142::<0x700> as *const (), 1), // +0x2bf640 sub_2bf640
    Lifted::new(0x70009ab660, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x5f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bf660 sub_2bf660
    Lifted::new(0x70009ab730, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x63, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bf730 sub_2bf730
    Lifted::new(0x70009ab800, fn_70009ab800 as *const (), 1), // +0x2bf800 sub_2bf800
    Lifted::new(0x70009ab810, tpl_47::<0xd76c> as *const (), 1), // +0x2bf810 sub_2bf810
    Lifted::new(0x70009ab820, tpl_52::<0xd76c> as *const (), 1), // +0x2bf820 sub_2bf820
    Lifted::new(0x70009ab830, tpl_142::<0x138> as *const (), 1), // +0x2bf830 sub_2bf830
    Lifted::new(0x70009ab850, tpl_47::<0xc940> as *const (), 1), // +0x2bf850 sub_2bf850
    Lifted::new(0x70009ab860, tpl_36::<0xd8> as *const (), 1), // +0x2bf860 sub_2bf860
    Lifted::new(0x70009ab870, tpl_335::<0x30> as *const (), 1), // +0x2bf870 sub_2bf870
    Lifted::new(0x70009ab880, tpl_142::<0x80> as *const (), 1), // +0x2bf880 sub_2bf880
    Lifted::new(0x70009ab8a0, tpl_196 as *const (), 1), // +0x2bf8a0 sub_2bf8a0
    Lifted::new(0x70009ab8b0, tpl_52::<0x248> as *const (), 1), // +0x2bf8b0 sub_2bf8b0
    Lifted::new(0x70009ab8c0, tpl_142::<0x718> as *const (), 1), // +0x2bf8c0 sub_2bf8c0
    Lifted::new(0x70009ab8e0, tpl_30 as *const (), 1), // +0x2bf8e0 sub_2bf8e0
    Lifted::new(0x70009ab8f0, tpl_30 as *const (), 1), // +0x2bf8f0 sub_2bf8f0
    Lifted::new(0x70009ab900, tpl_30 as *const (), 1), // +0x2bf900 sub_2bf900
    Lifted::new(0x70009ab910, tpl_30 as *const (), 1), // +0x2bf910 sub_2bf910
    Lifted::new(0x70009ab920, tpl_142::<0xbf0> as *const (), 1), // +0x2bf920 sub_2bf920
    Lifted::new(0x70009ab940, tpl_52::<0xa670> as *const (), 1), // +0x2bf940 sub_2bf940
    Lifted::new(0x70009ab950, tpl_142::<0xf48> as *const (), 1), // +0x2bf950 sub_2bf950
    Lifted::new(0x70009ab970, tpl_142::<0xf50> as *const (), 1), // +0x2bf970 sub_2bf970
    Lifted::new(0x70009ab990, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1eb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bf990 sub_2bf990
    Lifted::new(0x70009aba60, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x11f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bfa60 sub_2bfa60
    Lifted::new(0x70009abb30, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x397, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bfb30 sub_2bfb30
    Lifted::new(0x70009abc00, tpl_47::<0x8> as *const (), 1), // +0x2bfc00 sub_2bfc00
    Lifted::new(0x70009abc10, tpl_142::<0xf60> as *const (), 1), // +0x2bfc10 sub_2bfc10
    Lifted::new(0x70009abc30, tpl_36::<0x58> as *const (), 1), // +0x2bfc30 sub_2bfc30
    Lifted::new(0x70009abc40, tpl_142::<0xf58> as *const (), 1), // +0x2bfc40 sub_2bfc40
    Lifted::new(0x70009abc60, tpl_142::<0x728> as *const (), 1), // +0x2bfc60 sub_2bfc60
    Lifted::new(0x70009abc80, tpl_142::<0xf68> as *const (), 1), // +0x2bfc80 sub_2bfc80
    Lifted::new(0x70009abca0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x43, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bfca0 sub_2bfca0
    Lifted::new(0x70009abd70, tpl_142::<0x730> as *const (), 1), // +0x2bfd70 sub_2bfd70
    Lifted::new(0x70009abd90, tpl_47::<0x9> as *const (), 1), // +0x2bfd90 sub_2bfd90
    Lifted::new(0x70009abda0, tpl_142::<0xd48> as *const (), 1), // +0x2bfda0 sub_2bfda0
    Lifted::new(0x70009abdc0, tpl_142::<0xdd0> as *const (), 1), // +0x2bfdc0 sub_2bfdc0
    Lifted::new(0x70009abde0, tpl_142::<0x738> as *const (), 1), // +0x2bfde0 sub_2bfde0
    Lifted::new(0x70009abe00, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x39b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bfe00 sub_2bfe00
    Lifted::new(0x70009abed0, tpl_52::<0xd880> as *const (), 1), // +0x2bfed0 sub_2bfed0
    Lifted::new(0x70009abee0, tpl_36::<0x78> as *const (), 1), // +0x2bfee0 sub_2bfee0
    Lifted::new(0x70009abef0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x123, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bfef0 sub_2bfef0
    Lifted::new(0x70009abfc0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x30f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2bffc0 sub_2bffc0
    Lifted::new(0x70009ac090, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x313, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c0090 sub_2c0090
    Lifted::new(0x70009ac160, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c0160 sub_2c0160
    Lifted::new(0x70009ac230, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x127, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c0230 sub_2c0230
    Lifted::new(0x70009ac300, fn_70009ac300 as *const (), 0), // +0x2c0300 sub_2c0300
    Lifted::new(0x70009ac340, tpl_142::<0x758> as *const (), 1), // +0x2c0340 sub_2c0340
    Lifted::new(0x70009ac360, tpl_142::<0x760> as *const (), 1), // +0x2c0360 sub_2c0360
    Lifted::new(0x70009ac380, tpl_111 as *const (), 1), // +0x2c0380 sub_2c0380
    Lifted::new(0x70009ac390, tpl_111 as *const (), 1), // +0x2c0390 sub_2c0390
    Lifted::new(0x70009ac3a0, tpl_142::<0x748> as *const (), 1), // +0x2c03a0 sub_2c03a0
    Lifted::new(0x70009ac3c0, tpl_142::<0x750> as *const (), 1), // +0x2c03c0 sub_2c03c0
    Lifted::new(0x70009ac3e0, tpl_36::<0x9758> as *const (), 1), // +0x2c03e0 sub_2c03e0
    Lifted::new(0x70009ac3f0, tpl_142::<0x770> as *const (), 1), // +0x2c03f0 sub_2c03f0
    Lifted::new(0x70009ac410, tpl_142::<0x1f0> as *const (), 1), // +0x2c0410 sub_2c0410
    Lifted::new(0x70009ac430, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x37b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c0430 sub_2c0430
    Lifted::new(0x70009ac500, tpl_36::<0xa718> as *const (), 1), // +0x2c0500 sub_2c0500
    Lifted::new(0x70009ac510, tpl_336::<0x2fe8, 0x700075d990, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c0510 sub_2c0510
    Lifted::new(0x70009ac5c0, tpl_142::<0x130> as *const (), 1), // +0x2c05c0 sub_2c05c0
    Lifted::new(0x70009ac5e0, tpl_336::<0x2fe8, 0x70007606d0, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c05e0 sub_2c05e0
    Lifted::new(0x70009ac690, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x37f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c0690 sub_2c0690
    Lifted::new(0x70009ac760, fn_70009ac760 as *const (), 0), // +0x2c0760 sub_2c0760
    Lifted::new(0x70009ac830, tpl_142::<0x768> as *const (), 1), // +0x2c0830 sub_2c0830
    Lifted::new(0x70009ac850, tpl_142::<0x498> as *const (), 1), // +0x2c0850 sub_2c0850
    Lifted::new(0x70009ac870, tpl_328::<0x4f> as *const (), 1), // +0x2c0870 sub_2c0870
    Lifted::new(0x70009ac880, tpl_52::<0xcf78> as *const (), 1), // +0x2c0880 sub_2c0880
    Lifted::new(0x70009ac890, tpl_142::<0x778> as *const (), 1), // +0x2c0890 sub_2c0890
    Lifted::new(0x70009ac8b0, tpl_142::<0x780> as *const (), 1), // +0x2c08b0 sub_2c08b0
    Lifted::new(0x70009ac8d0, tpl_52::<0xa6d8> as *const (), 1), // +0x2c08d0 sub_2c08d0
    Lifted::new(0x70009ac8e0, tpl_36::<0xa6f8> as *const (), 1), // +0x2c08e0 sub_2c08e0
    Lifted::new(0x70009ac8f0, tpl_52::<0xa6f8> as *const (), 1), // +0x2c08f0 sub_2c08f0
    Lifted::new(0x70009ac900, tpl_191::<0x10> as *const (), 1), // +0x2c0900 sub_2c0900
    Lifted::new(0x70009ac910, tpl_36::<0x78> as *const (), 1), // +0x2c0910 sub_2c0910
    Lifted::new(0x70009ac920, tpl_52::<0x3050> as *const (), 1), // +0x2c0920 sub_2c0920
    Lifted::new(0x70009ac930, tpl_39::<0x44> as *const (), 1), // +0x2c0930 sub_2c0930
    Lifted::new(0x70009ac940, tpl_36::<0x28> as *const (), 1), // +0x2c0940 sub_2c0940
    Lifted::new(0x70009ac950, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x70009ac961> as *const (), 0), // +0x2c0950 sub_2c0950
    Lifted::new(0x70009ac970, fn_70009ac970 as *const (), 1), // +0x2c0970 sub_2c0970
    Lifted::new(0x70009ac9b0, tpl_172::<0x10> as *const (), 1), // +0x2c09b0 sub_2c09b0
    Lifted::new(0x70009ac9c0, tpl_337::<0x30> as *const (), 1), // +0x2c09c0 sub_2c09c0
    Lifted::new(0x70009ac9d0, tpl_338::<0x7001c72d48, 0xd7, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c09d0 sub_2c09d0
    Lifted::new(0x70009aca30, tpl_339::<0x7001c72d48, 0xd7, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0a30 sub_2c0a30
    Lifted::new(0x70009aca80, tpl_142::<0x790> as *const (), 1), // +0x2c0a80 sub_2c0a80
    Lifted::new(0x70009acaa0, tpl_338::<0x7001c72d48, 0xf, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0aa0 sub_2c0aa0
    Lifted::new(0x70009acaf0, tpl_339::<0x7001c72d48, 0xf, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0af0 sub_2c0af0
    Lifted::new(0x70009acb40, tpl_338::<0x7001c72d48, 0xdf, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0b40 sub_2c0b40
    Lifted::new(0x70009acba0, tpl_339::<0x7001c72d48, 0xdf, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0ba0 sub_2c0ba0
    Lifted::new(0x70009acbf0, tpl_142::<0xd08> as *const (), 1), // +0x2c0bf0 sub_2c0bf0
    Lifted::new(0x70009acc10, tpl_338::<0x7001c72d48, 0xe3, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0c10 sub_2c0c10
    Lifted::new(0x70009acc70, tpl_339::<0x7001c72d48, 0xe3, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0c70 sub_2c0c70
    Lifted::new(0x70009accc0, tpl_338::<0x7001c72d48, 0xcb, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0cc0 sub_2c0cc0
    Lifted::new(0x70009acd20, tpl_339::<0x7001c72d48, 0xcb, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0d20 sub_2c0d20
    Lifted::new(0x70009acd70, tpl_340::<0xa8, 0x2ff8> as *const (), 1), // +0x2c0d70 sub_2c0d70
    Lifted::new(0x70009acd90, tpl_144 as *const (), 1), // +0x2c0d90 sub_2c0d90
    Lifted::new(0x70009acda0, tpl_172::<0x30> as *const (), 1), // +0x2c0da0 sub_2c0da0
    Lifted::new(0x70009acdb0, tpl_340::<0xa8, 0x3030> as *const (), 1), // +0x2c0db0 sub_2c0db0
    Lifted::new(0x70009acdd0, tpl_340::<0xa8, 0x3040> as *const (), 1), // +0x2c0dd0 sub_2c0dd0
    Lifted::new(0x70009acdf0, fn_70009acdf0 as *const (), 1), // +0x2c0df0 sub_2c0df0
    Lifted::new(0x70009ace30, tpl_341 as *const (), 1), // +0x2c0e30 sub_2c0e30
    Lifted::new(0x70009ace40, tpl_142::<0x788> as *const (), 1), // +0x2c0e40 sub_2c0e40
    Lifted::new(0x70009ace60, fn_70009ace60 as *const (), 1), // +0x2c0e60 sub_2c0e60
    Lifted::new(0x70009acea0, tpl_195::<0x70> as *const (), 1), // +0x2c0ea0 sub_2c0ea0
    Lifted::new(0x70009aceb0, tpl_338::<0x7001c72d48, 0xdb, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0eb0 sub_2c0eb0
    Lifted::new(0x70009acf10, tpl_339::<0x7001c72d48, 0xdb, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0f10 sub_2c0f10
    Lifted::new(0x70009acf60, tpl_338::<0x7001c72d48, 0xd3, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0f60 sub_2c0f60
    Lifted::new(0x70009acfc0, tpl_339::<0x7001c72d48, 0xd3, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c0fc0 sub_2c0fc0
    Lifted::new(0x70009ad010, tpl_338::<0x7001c72d48, 0xe7, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c1010 sub_2c1010
    Lifted::new(0x70009ad070, tpl_339::<0x7001c72d48, 0xe7, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c1070 sub_2c1070
    Lifted::new(0x70009ad0c0, tpl_342::<0x50> as *const (), 1), // +0x2c10c0 sub_2c10c0
    Lifted::new(0x70009ad0d0, fn_70009ad0d0 as *const (), 1), // +0x2c10d0 sub_2c10d0
    Lifted::new(0x70009ad0f0, tpl_338::<0x7001c72d48, 0xcf, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c10f0 sub_2c10f0
    Lifted::new(0x70009ad150, tpl_339::<0x7001c72d48, 0xcf, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c1150 sub_2c1150
    Lifted::new(0x70009ad1a0, tpl_338::<0x7001c72d48, 0xeb, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c11a0 sub_2c11a0
    Lifted::new(0x70009ad200, tpl_339::<0x7001c72d48, 0xeb, 0x8, 0x50, 0x10> as *const (), 1), // +0x2c1200 sub_2c1200
    Lifted::new(0x70009ad250, tpl_142::<0x4b0> as *const (), 1), // +0x2c1250 sub_2c1250
    Lifted::new(0x70009ad270, tpl_343::<0x10> as *const (), 1), // +0x2c1270 sub_2c1270
    Lifted::new(0x70009ad280, tpl_30 as *const (), 1), // +0x2c1280 sub_2c1280
    Lifted::new(0x70009ad290, tpl_142::<0xf0> as *const (), 1), // +0x2c1290 sub_2c1290
    Lifted::new(0x70009ad2b0, tpl_36::<0x8> as *const (), 1), // +0x2c12b0 sub_2c12b0
    Lifted::new(0x70009ad2c0, tpl_30 as *const (), 1), // +0x2c12c0 sub_2c12c0
    Lifted::new(0x70009ad2d0, tpl_52::<0x9768> as *const (), 1), // +0x2c12d0 sub_2c12d0
    Lifted::new(0x70009ad2e0, tpl_52::<0x9768> as *const (), 1), // +0x2c12e0 sub_2c12e0
    Lifted::new(0x70009ad2f0, tpl_30 as *const (), 1), // +0x2c12f0 sub_2c12f0
    Lifted::new(0x70009ad300, tpl_52::<0x8> as *const (), 1), // +0x2c1300 sub_2c1300
    Lifted::new(0x70009ad310, fn_70009ad310 as *const (), 1), // +0x2c1310 sub_2c1310
    Lifted::new(0x70009ad320, tpl_142::<0x140> as *const (), 1), // +0x2c1320 sub_2c1320
    Lifted::new(0x70009ad340, tpl_36::<0xc7f8> as *const (), 1), // +0x2c1340 sub_2c1340
    Lifted::new(0x70009ad350, tpl_142::<0x450> as *const (), 1), // +0x2c1350 sub_2c1350
    Lifted::new(0x70009ad370, tpl_52::<0x9> as *const (), 1), // +0x2c1370 sub_2c1370
    Lifted::new(0x70009ad380, tpl_328::<0xaf> as *const (), 1), // +0x2c1380 sub_2c1380
    Lifted::new(0x70009ad390, tpl_36::<0x18> as *const (), 1), // +0x2c1390 sub_2c1390
    Lifted::new(0x70009ad3a0, tpl_39::<0xa588> as *const (), 1), // +0x2c13a0 sub_2c13a0
    Lifted::new(0x70009ad3b0, tpl_39::<0x10> as *const (), 1), // +0x2c13b0 sub_2c13b0
    Lifted::new(0x70009ad3c0, tpl_142::<0x798> as *const (), 1), // +0x2c13c0 sub_2c13c0
    Lifted::new(0x70009ad3e0, fn_70009ad3e0 as *const (), 1), // +0x2c13e0 sub_2c13e0
    Lifted::new(0x70009ad410, tpl_142::<0x7a0> as *const (), 1), // +0x2c1410 sub_2c1410
    Lifted::new(0x70009ad430, tpl_142::<0x7a8> as *const (), 1), // +0x2c1430 sub_2c1430
    Lifted::new(0x70009ad450, tpl_20 as *const (), 1), // +0x2c1450 sub_2c1450
    Lifted::new(0x70009ad460, tpl_20 as *const (), 1), // +0x2c1460 sub_2c1460
    Lifted::new(0x70009ad470, tpl_36::<0xe8> as *const (), 1), // +0x2c1470 sub_2c1470
    Lifted::new(0x70009ad480, tpl_333::<0xb> as *const (), 1), // +0x2c1480 sub_2c1480
    Lifted::new(0x70009ad4a0, tpl_334::<0xb> as *const (), 1), // +0x2c14a0 sub_2c14a0
    Lifted::new(0x70009ad4d0, tpl_328::<0x37> as *const (), 1), // +0x2c14d0 sub_2c14d0
    Lifted::new(0x70009ad4e0, tpl_328::<0x57> as *const (), 1), // +0x2c14e0 sub_2c14e0
    Lifted::new(0x70009ad4f0, tpl_333::<0xd7> as *const (), 1), // +0x2c14f0 sub_2c14f0
    Lifted::new(0x70009ad510, tpl_334::<0xd7> as *const (), 1), // +0x2c1510 sub_2c1510
    Lifted::new(0x70009ad540, fn_70009ad540 as *const (), 1), // +0x2c1540 sub_2c1540
    Lifted::new(0x70009ad550, tpl_52::<0x60> as *const (), 1), // +0x2c1550 sub_2c1550
    Lifted::new(0x70009ad560, tpl_36::<0x48> as *const (), 1), // +0x2c1560 sub_2c1560
    Lifted::new(0x70009ad570, tpl_41 as *const (), 1), // +0x2c1570 sub_2c1570
    Lifted::new(0x70009ad580, tpl_39::<0x30> as *const (), 1), // +0x2c1580 sub_2c1580
    Lifted::new(0x70009ad590, tpl_142::<0x7b0> as *const (), 1), // +0x2c1590 sub_2c1590
    Lifted::new(0x70009ad5b0, tpl_142::<0x7b8> as *const (), 1), // +0x2c15b0 sub_2c15b0
    Lifted::new(0x70009ad5d0, tpl_333::<0xf> as *const (), 1), // +0x2c15d0 sub_2c15d0
    Lifted::new(0x70009ad5f0, tpl_334::<0xf> as *const (), 1), // +0x2c15f0 sub_2c15f0
    Lifted::new(0x70009ad620, tpl_328::<0x47> as *const (), 1), // +0x2c1620 sub_2c1620
    Lifted::new(0x70009ad630, tpl_147::<0x13> as *const (), 1), // +0x2c1630 sub_2c1630
    Lifted::new(0x70009ad640, tpl_328::<0x3f> as *const (), 1), // +0x2c1640 sub_2c1640
    Lifted::new(0x70009ad650, tpl_333::<0xdf> as *const (), 1), // +0x2c1650 sub_2c1650
    Lifted::new(0x70009ad670, tpl_334::<0xdf> as *const (), 1), // +0x2c1670 sub_2c1670
    Lifted::new(0x70009ad6a0, tpl_142::<0x7c8> as *const (), 1), // +0x2c16a0 sub_2c16a0
    Lifted::new(0x70009ad6c0, tpl_142::<0x458> as *const (), 1), // +0x2c16c0 sub_2c16c0
    Lifted::new(0x70009ad6e0, tpl_36::<0x68> as *const (), 1), // +0x2c16e0 sub_2c16e0
    Lifted::new(0x70009ad6f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x12f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c16f0 sub_2c16f0
    Lifted::new(0x70009ad7c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x133, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c17c0 sub_2c17c0
    Lifted::new(0x70009ad890, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x137, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1890 sub_2c1890
    Lifted::new(0x70009ad960, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x147, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1960 sub_2c1960
    Lifted::new(0x70009ada30, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x143, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1a30 sub_2c1a30
    Lifted::new(0x70009adb00, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x13b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1b00 sub_2c1b00
    Lifted::new(0x70009adbd0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x13f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1bd0 sub_2c1bd0
    Lifted::new(0x70009adca0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x14b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1ca0 sub_2c1ca0
    Lifted::new(0x70009add70, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x14f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1d70 sub_2c1d70
    Lifted::new(0x70009ade40, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x153, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1e40 sub_2c1e40
    Lifted::new(0x70009adf10, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x157, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1f10 sub_2c1f10
    Lifted::new(0x70009adfe0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x29f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c1fe0 sub_2c1fe0
    Lifted::new(0x70009ae0b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x15b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c20b0 sub_2c20b0
    Lifted::new(0x70009ae180, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x15f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c2180 sub_2c2180
    Lifted::new(0x70009ae250, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x163, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c2250 sub_2c2250
    Lifted::new(0x70009ae320, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x167, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c2320 sub_2c2320
    Lifted::new(0x70009ae3f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x16b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c23f0 sub_2c23f0
    Lifted::new(0x70009ae4c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x16f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c24c0 sub_2c24c0
    Lifted::new(0x70009ae590, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x173, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c2590 sub_2c2590
    Lifted::new(0x70009ae660, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x177, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c2660 sub_2c2660
    Lifted::new(0x70009ae730, tpl_47::<0xc7e3> as *const (), 1), // +0x2c2730 sub_2c2730
    Lifted::new(0x70009ae740, tpl_36::<0xa710> as *const (), 1), // +0x2c2740 sub_2c2740
    Lifted::new(0x70009ae750, tpl_142::<0x7d0> as *const (), 1), // +0x2c2750 sub_2c2750
    Lifted::new(0x70009ae770, tpl_344::<0x8, 0x10, 0xfffffffffffffff0, 0x700177c0d0, 0x7000989d80> as *const (), 2), // +0x2c2770 sub_2c2770
    Lifted::new(0x70009ae830, tpl_344::<0x8, 0x10, 0xfffffffffffffff0, 0x700177c0d0, 0x7000989a20> as *const (), 2), // +0x2c2830 sub_2c2830
    Lifted::new(0x70009ae8f0, fn_70009ae8f0 as *const (), 0), // +0x2c28f0 sub_2c28f0
    Lifted::new(0x70009ae960, fn_70009ae960 as *const (), 2), // +0x2c2960 sub_2c2960
    Lifted::new(0x70009aeca0, tpl_216::<0x700098c380> as *const (), 0), // +0x2c2ca0 sub_2c2ca0
    Lifted::new(0x70009aecc0, fn_70009aecc0 as *const (), 0), // +0x2c2cc0 sub_2c2cc0
    Lifted::new(0x70009aee50, fn_70009aee50 as *const (), 0), // +0x2c2e50 sub_2c2e50
    Lifted::new(0x70009aeec0, fn_70009aeec0 as *const (), 0), // +0x2c2ec0 sub_2c2ec0
    Lifted::new(0x70009af1e0, fn_70009af1e0 as *const (), 0), // +0x2c31e0 sub_2c31e0
    Lifted::new(0x70009af210, tpl_36::<0xc950> as *const (), 1), // +0x2c3210 sub_2c3210
    Lifted::new(0x70009af220, tpl_36::<0x8> as *const (), 1), // +0x2c3220 sub_2c3220
    Lifted::new(0x70009af230, fn_70009af230 as *const (), 1), // +0x2c3230 sub_2c3230
    Lifted::new(0x70009af250, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x53, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3250 sub_2c3250
    Lifted::new(0x70009af320, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x5b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3320 sub_2c3320
    Lifted::new(0x70009af3f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x4b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c33f0 sub_2c33f0
    Lifted::new(0x70009af4c0, tpl_142::<0xdd8> as *const (), 1), // +0x2c34c0 sub_2c34c0
    Lifted::new(0x70009af4e0, tpl_142::<0xe80> as *const (), 1), // +0x2c34e0 sub_2c34e0
    Lifted::new(0x70009af500, tpl_142::<0xeb8> as *const (), 1), // +0x2c3500 sub_2c3500
    Lifted::new(0x70009af520, tpl_142::<0xec0> as *const (), 1), // +0x2c3520 sub_2c3520
    Lifted::new(0x70009af540, tpl_52::<0xa668> as *const (), 1), // +0x2c3540 sub_2c3540
    Lifted::new(0x70009af550, fn_70009af550 as *const (), 1), // +0x2c3550 sub_2c3550
    Lifted::new(0x70009af570, tpl_142::<0x338> as *const (), 1), // +0x2c3570 sub_2c3570
    Lifted::new(0x70009af590, tpl_36::<0xc810> as *const (), 1), // +0x2c3590 sub_2c3590
    Lifted::new(0x70009af5a0, tpl_142::<0xde0> as *const (), 1), // +0x2c35a0 sub_2c35a0
    Lifted::new(0x70009af5c0, tpl_142::<0x11a8> as *const (), 1), // +0x2c35c0 sub_2c35c0
    Lifted::new(0x70009af5e0, tpl_142::<0x1160> as *const (), 1), // +0x2c35e0 sub_2c35e0
    Lifted::new(0x70009af600, tpl_142::<0xbf8> as *const (), 1), // +0x2c3600 sub_2c3600
    Lifted::new(0x70009af620, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x17b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3620 sub_2c3620
    Lifted::new(0x70009af6f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x17f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c36f0 sub_2c36f0
    Lifted::new(0x70009af7c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x183, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c37c0 sub_2c37c0
    Lifted::new(0x70009af890, tpl_142::<0xcc0> as *const (), 1), // +0x2c3890 sub_2c3890
    Lifted::new(0x70009af8b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x18f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c38b0 sub_2c38b0
    Lifted::new(0x70009af980, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x18b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3980 sub_2c3980
    Lifted::new(0x70009afa50, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x187, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3a50 sub_2c3a50
    Lifted::new(0x70009afb20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x193, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3b20 sub_2c3b20
    Lifted::new(0x70009afbf0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x197, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3bf0 sub_2c3bf0
    Lifted::new(0x70009afcc0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1a3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3cc0 sub_2c3cc0
    Lifted::new(0x70009afd90, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x19b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3d90 sub_2c3d90
    Lifted::new(0x70009afe60, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x19f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c3e60 sub_2c3e60
    Lifted::new(0x70009aff30, tpl_142::<0xf98> as *const (), 1), // +0x2c3f30 sub_2c3f30
    Lifted::new(0x70009aff50, fn_70009aff50 as *const (), 1), // +0x2c3f50 sub_2c3f50
    Lifted::new(0x70009aff60, tpl_142::<0x7f0> as *const (), 1), // +0x2c3f60 sub_2c3f60
    Lifted::new(0x70009aff80, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x9f> as *const (), 1), // +0x2c3f80 sub_2c3f80
    Lifted::new(0x70009affc0, tpl_47::<0x8> as *const (), 1), // +0x2c3fc0 sub_2c3fc0
    Lifted::new(0x70009affd0, tpl_52::<0x8> as *const (), 1), // +0x2c3fd0 sub_2c3fd0
    Lifted::new(0x70009affe0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x377> as *const (), 1), // +0x2c3fe0 sub_2c3fe0
    Lifted::new(0x70009b0020, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xa3> as *const (), 1), // +0x2c4020 sub_2c4020
    Lifted::new(0x70009b0060, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xa7> as *const (), 1), // +0x2c4060 sub_2c4060
    Lifted::new(0x70009b00a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xab> as *const (), 1), // +0x2c40a0 sub_2c40a0
    Lifted::new(0x70009b00e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xaf> as *const (), 1), // +0x2c40e0 sub_2c40e0
    Lifted::new(0x70009b0120, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x35b> as *const (), 1), // +0x2c4120 sub_2c4120
    Lifted::new(0x70009b0160, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x35f> as *const (), 1), // +0x2c4160 sub_2c4160
    Lifted::new(0x70009b01a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xb3> as *const (), 1), // +0x2c41a0 sub_2c41a0
    Lifted::new(0x70009b01e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xb7> as *const (), 1), // +0x2c41e0 sub_2c41e0
    Lifted::new(0x70009b0220, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x363> as *const (), 1), // +0x2c4220 sub_2c4220
    Lifted::new(0x70009b0260, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x367> as *const (), 1), // +0x2c4260 sub_2c4260
    Lifted::new(0x70009b02a0, fn_70009b02a0 as *const (), 1), // +0x2c42a0 sub_2c42a0
    Lifted::new(0x70009b02b0, tpl_346 as *const (), 1), // +0x2c42b0 sub_2c42b0
    Lifted::new(0x70009b02c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xbb> as *const (), 1), // +0x2c42c0 sub_2c42c0
    Lifted::new(0x70009b0300, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xbf> as *const (), 1), // +0x2c4300 sub_2c4300
    Lifted::new(0x70009b0340, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x307> as *const (), 1), // +0x2c4340 sub_2c4340
    Lifted::new(0x70009b0380, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xc3> as *const (), 1), // +0x2c4380 sub_2c4380
    Lifted::new(0x70009b03c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x30b> as *const (), 1), // +0x2c43c0 sub_2c43c0
    Lifted::new(0x70009b0400, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xc7> as *const (), 1), // +0x2c4400 sub_2c4400
    Lifted::new(0x70009b0440, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x317> as *const (), 1), // +0x2c4440 sub_2c4440
    Lifted::new(0x70009b0480, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x31b> as *const (), 1), // +0x2c4480 sub_2c4480
    Lifted::new(0x70009b04c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x12b> as *const (), 1), // +0x2c44c0 sub_2c44c0
    Lifted::new(0x70009b0500, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x23> as *const (), 1), // +0x2c4500 sub_2c4500
    Lifted::new(0x70009b0540, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xcb> as *const (), 1), // +0x2c4540 sub_2c4540
    Lifted::new(0x70009b0580, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2d7> as *const (), 1), // +0x2c4580 sub_2c4580
    Lifted::new(0x70009b05c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1fb> as *const (), 1), // +0x2c45c0 sub_2c45c0
    Lifted::new(0x70009b0600, fn_70009b0600 as *const (), 1), // +0x2c4600 sub_2c4600
    Lifted::new(0x70009b0610, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x6f> as *const (), 1), // +0x2c4610 sub_2c4610
    Lifted::new(0x70009b0650, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xcf> as *const (), 1), // +0x2c4650 sub_2c4650
    Lifted::new(0x70009b0690, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x6b> as *const (), 1), // +0x2c4690 sub_2c4690
    Lifted::new(0x70009b06d0, fn_70009b06d0 as *const (), 1), // +0x2c46d0 sub_2c46d0
    Lifted::new(0x70009b0700, fn_70009b0700 as *const (), 1), // +0x2c4700 sub_2c4700
    Lifted::new(0x70009b0710, fn_70009b0710 as *const (), 1), // +0x2c4710 sub_2c4710
    Lifted::new(0x70009b0740, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1ff> as *const (), 1), // +0x2c4740 sub_2c4740
    Lifted::new(0x70009b0780, fn_70009b0780 as *const (), 1), // +0x2c4780 sub_2c4780
    Lifted::new(0x70009b0790, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xd3> as *const (), 1), // +0x2c4790 sub_2c4790
    Lifted::new(0x70009b07d0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xd7> as *const (), 1), // +0x2c47d0 sub_2c47d0
    Lifted::new(0x70009b0810, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xdb> as *const (), 1), // +0x2c4810 sub_2c4810
    Lifted::new(0x70009b0850, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xdf> as *const (), 1), // +0x2c4850 sub_2c4850
    Lifted::new(0x70009b0890, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xe3> as *const (), 1), // +0x2c4890 sub_2c4890
    Lifted::new(0x70009b08d0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xe7> as *const (), 1), // +0x2c48d0 sub_2c48d0
    Lifted::new(0x70009b0910, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xeb> as *const (), 1), // +0x2c4910 sub_2c4910
    Lifted::new(0x70009b0950, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xef> as *const (), 1), // +0x2c4950 sub_2c4950
    Lifted::new(0x70009b0990, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x203> as *const (), 1), // +0x2c4990 sub_2c4990
    Lifted::new(0x70009b09d0, tpl_210::<0x188> as *const (), 1), // +0x2c49d0 sub_2c49d0
    Lifted::new(0x70009b09e0, fn_70009b09e0 as *const (), 0), // +0x2c49e0 sub_2c49e0
    Lifted::new(0x70009b0b90, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x31f> as *const (), 1), // +0x2c4b90 sub_2c4b90
    Lifted::new(0x70009b0bd0, tpl_347::<0xc944> as *const (), 1), // +0x2c4bd0 sub_2c4bd0
    Lifted::new(0x70009b0be0, tpl_195::<0x114> as *const (), 1), // +0x2c4be0 sub_2c4be0
    Lifted::new(0x70009b0bf0, tpl_142::<0xfb8> as *const (), 1), // +0x2c4bf0 sub_2c4bf0
    Lifted::new(0x70009b0c10, tpl_142::<0xd10> as *const (), 1), // +0x2c4c10 sub_2c4c10
    Lifted::new(0x70009b0c30, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xf3> as *const (), 1), // +0x2c4c30 sub_2c4c30
    Lifted::new(0x70009b0c70, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1b> as *const (), 1), // +0x2c4c70 sub_2c4c70
    Lifted::new(0x70009b0cb0, fn_70009b0cb0 as *const (), 1), // +0x2c4cb0 sub_2c4cb0
    Lifted::new(0x70009b0ce0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xf7> as *const (), 1), // +0x2c4ce0 sub_2c4ce0
    Lifted::new(0x70009b0d20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xfb> as *const (), 1), // +0x2c4d20 sub_2c4d20
    Lifted::new(0x70009b0d60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0xff> as *const (), 1), // +0x2c4d60 sub_2c4d60
    Lifted::new(0x70009b0da0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x103> as *const (), 1), // +0x2c4da0 sub_2c4da0
    Lifted::new(0x70009b0de0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x20b> as *const (), 1), // +0x2c4de0 sub_2c4de0
    Lifted::new(0x70009b0e20, fn_70009b0e20 as *const (), 1), // +0x2c4e20 sub_2c4e20
    Lifted::new(0x70009b0e50, tpl_318 as *const (), 1), // +0x2c4e50 sub_2c4e50
    Lifted::new(0x70009b0e60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x17> as *const (), 1), // +0x2c4e60 sub_2c4e60
    Lifted::new(0x70009b0ea0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x107> as *const (), 1), // +0x2c4ea0 sub_2c4ea0
    Lifted::new(0x70009b0ee0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36b> as *const (), 1), // +0x2c4ee0 sub_2c4ee0
    Lifted::new(0x70009b0f20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x10b> as *const (), 1), // +0x2c4f20 sub_2c4f20
    Lifted::new(0x70009b0f60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36f> as *const (), 1), // +0x2c4f60 sub_2c4f60
    Lifted::new(0x70009b0fa0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x10f> as *const (), 1), // +0x2c4fa0 sub_2c4fa0
    Lifted::new(0x70009b0fe0, tpl_348 as *const (), 1), // +0x2c4fe0 sub_2c4fe0
    Lifted::new(0x70009b0ff0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1f3> as *const (), 1), // +0x2c4ff0 sub_2c4ff0
    Lifted::new(0x70009b1030, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x373> as *const (), 1), // +0x2c5030 sub_2c5030
    Lifted::new(0x70009b1070, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x113> as *const (), 1), // +0x2c5070 sub_2c5070
    Lifted::new(0x70009b10b0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x117> as *const (), 1), // +0x2c50b0 sub_2c50b0
    Lifted::new(0x70009b10f0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x11b> as *const (), 1), // +0x2c50f0 sub_2c50f0
    Lifted::new(0x70009b1130, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x393> as *const (), 1), // +0x2c5130 sub_2c5130
    Lifted::new(0x70009b1170, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x5f> as *const (), 1), // +0x2c5170 sub_2c5170
    Lifted::new(0x70009b11b0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x63> as *const (), 1), // +0x2c51b0 sub_2c51b0
    Lifted::new(0x70009b11f0, tpl_349 as *const (), 1), // +0x2c51f0 sub_2c51f0
    Lifted::new(0x70009b1200, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1eb> as *const (), 1), // +0x2c5200 sub_2c5200
    Lifted::new(0x70009b1240, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x11f> as *const (), 1), // +0x2c5240 sub_2c5240
    Lifted::new(0x70009b1280, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x397> as *const (), 1), // +0x2c5280 sub_2c5280
    Lifted::new(0x70009b12c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x43> as *const (), 1), // +0x2c52c0 sub_2c52c0
    Lifted::new(0x70009b1300, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x39b> as *const (), 1), // +0x2c5300 sub_2c5300
    Lifted::new(0x70009b1340, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x123> as *const (), 1), // +0x2c5340 sub_2c5340
    Lifted::new(0x70009b1380, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x30f> as *const (), 1), // +0x2c5380 sub_2c5380
    Lifted::new(0x70009b13c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x313> as *const (), 1), // +0x2c53c0 sub_2c53c0
    Lifted::new(0x70009b1400, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1f> as *const (), 1), // +0x2c5400 sub_2c5400
    Lifted::new(0x70009b1440, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x127> as *const (), 1), // +0x2c5440 sub_2c5440
    Lifted::new(0x70009b1480, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x37b> as *const (), 1), // +0x2c5480 sub_2c5480
    Lifted::new(0x70009b14c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x37f> as *const (), 1), // +0x2c54c0 sub_2c54c0
    Lifted::new(0x70009b1500, fn_70009b1500 as *const (), 1), // +0x2c5500 sub_2c5500
    Lifted::new(0x70009b1540, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x12f> as *const (), 1), // +0x2c5540 sub_2c5540
    Lifted::new(0x70009b1580, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x133> as *const (), 1), // +0x2c5580 sub_2c5580
    Lifted::new(0x70009b15c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x137> as *const (), 1), // +0x2c55c0 sub_2c55c0
    Lifted::new(0x70009b1600, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x147> as *const (), 1), // +0x2c5600 sub_2c5600
    Lifted::new(0x70009b1640, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x143> as *const (), 1), // +0x2c5640 sub_2c5640
    Lifted::new(0x70009b1680, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x13b> as *const (), 1), // +0x2c5680 sub_2c5680
    Lifted::new(0x70009b16c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x13f> as *const (), 1), // +0x2c56c0 sub_2c56c0
    Lifted::new(0x70009b1700, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x14b> as *const (), 1), // +0x2c5700 sub_2c5700
    Lifted::new(0x70009b1740, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x14f> as *const (), 1), // +0x2c5740 sub_2c5740
    Lifted::new(0x70009b1780, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x153> as *const (), 1), // +0x2c5780 sub_2c5780
    Lifted::new(0x70009b17c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x157> as *const (), 1), // +0x2c57c0 sub_2c57c0
    Lifted::new(0x70009b1800, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x29f> as *const (), 1), // +0x2c5800 sub_2c5800
    Lifted::new(0x70009b1840, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x15b> as *const (), 1), // +0x2c5840 sub_2c5840
    Lifted::new(0x70009b1880, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x15f> as *const (), 1), // +0x2c5880 sub_2c5880
    Lifted::new(0x70009b18c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x163> as *const (), 1), // +0x2c58c0 sub_2c58c0
    Lifted::new(0x70009b1900, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x167> as *const (), 1), // +0x2c5900 sub_2c5900
    Lifted::new(0x70009b1940, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x16b> as *const (), 1), // +0x2c5940 sub_2c5940
    Lifted::new(0x70009b1980, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x16f> as *const (), 1), // +0x2c5980 sub_2c5980
    Lifted::new(0x70009b19c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x173> as *const (), 1), // +0x2c59c0 sub_2c59c0
    Lifted::new(0x70009b1a00, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x177> as *const (), 1), // +0x2c5a00 sub_2c5a00
    Lifted::new(0x70009b1a40, fn_70009b1a40 as *const (), 1), // +0x2c5a40 sub_2c5a40
    Lifted::new(0x70009b1a60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x53> as *const (), 1), // +0x2c5a60 sub_2c5a60
    Lifted::new(0x70009b1aa0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x5b> as *const (), 1), // +0x2c5aa0 sub_2c5aa0
    Lifted::new(0x70009b1ae0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x4b> as *const (), 1), // +0x2c5ae0 sub_2c5ae0
    Lifted::new(0x70009b1b20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x17b> as *const (), 1), // +0x2c5b20 sub_2c5b20
    Lifted::new(0x70009b1b60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x17f> as *const (), 1), // +0x2c5b60 sub_2c5b60
    Lifted::new(0x70009b1ba0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x183> as *const (), 1), // +0x2c5ba0 sub_2c5ba0
    Lifted::new(0x70009b1be0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x18f> as *const (), 1), // +0x2c5be0 sub_2c5be0
    Lifted::new(0x70009b1c20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x18b> as *const (), 1), // +0x2c5c20 sub_2c5c20
    Lifted::new(0x70009b1c60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x187> as *const (), 1), // +0x2c5c60 sub_2c5c60
    Lifted::new(0x70009b1ca0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x193> as *const (), 1), // +0x2c5ca0 sub_2c5ca0
    Lifted::new(0x70009b1ce0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x197> as *const (), 1), // +0x2c5ce0 sub_2c5ce0
    Lifted::new(0x70009b1d20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1a3> as *const (), 1), // +0x2c5d20 sub_2c5d20
    Lifted::new(0x70009b1d60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x19b> as *const (), 1), // +0x2c5d60 sub_2c5d60
    Lifted::new(0x70009b1da0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x19f> as *const (), 1), // +0x2c5da0 sub_2c5da0
    Lifted::new(0x70009b1de0, tpl_47::<0x40> as *const (), 1), // +0x2c5de0 sub_2c5de0
    Lifted::new(0x70009b1df0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1a7> as *const (), 1), // +0x2c5df0 sub_2c5df0
    Lifted::new(0x70009b1e30, fn_70009b1e30 as *const (), 1), // +0x2c5e30 sub_2c5e30
    Lifted::new(0x70009b1e50, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1bf> as *const (), 1), // +0x2c5e50 sub_2c5e50
    Lifted::new(0x70009b1e90, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1b7> as *const (), 1), // +0x2c5e90 sub_2c5e90
    Lifted::new(0x70009b1ed0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1af> as *const (), 1), // +0x2c5ed0 sub_2c5ed0
    Lifted::new(0x70009b1f10, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1bb> as *const (), 1), // +0x2c5f10 sub_2c5f10
    Lifted::new(0x70009b1f50, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1b3> as *const (), 1), // +0x2c5f50 sub_2c5f50
    Lifted::new(0x70009b1f90, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1ab> as *const (), 1), // +0x2c5f90 sub_2c5f90
    Lifted::new(0x70009b1fd0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1e3> as *const (), 1), // +0x2c5fd0 sub_2c5fd0
    Lifted::new(0x70009b2010, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1c3> as *const (), 1), // +0x2c6010 sub_2c6010
    Lifted::new(0x70009b2050, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1c7> as *const (), 1), // +0x2c6050 sub_2c6050
    Lifted::new(0x70009b2090, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1cb> as *const (), 1), // +0x2c6090 sub_2c6090
    Lifted::new(0x70009b20d0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1cf> as *const (), 1), // +0x2c60d0 sub_2c60d0
    Lifted::new(0x70009b2110, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1d3> as *const (), 1), // +0x2c6110 sub_2c6110
    Lifted::new(0x70009b2150, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1d7> as *const (), 1), // +0x2c6150 sub_2c6150
    Lifted::new(0x70009b2190, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1df> as *const (), 1), // +0x2c6190 sub_2c6190
    Lifted::new(0x70009b21d0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1db> as *const (), 1), // +0x2c61d0 sub_2c61d0
    Lifted::new(0x70009b2210, tpl_350 as *const (), 1), // +0x2c6210 sub_2c6210
    Lifted::new(0x70009b2220, fn_70009b2220 as *const (), 1), // +0x2c6220 sub_2c6220
    Lifted::new(0x70009b2230, tpl_47::<0x8> as *const (), 1), // +0x2c6230 sub_2c6230
    Lifted::new(0x70009b2240, tpl_47::<0x10> as *const (), 1), // +0x2c6240 sub_2c6240
    Lifted::new(0x70009b2250, tpl_47::<0x10> as *const (), 1), // +0x2c6250 sub_2c6250
    Lifted::new(0x70009b2260, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x20f> as *const (), 1), // +0x2c6260 sub_2c6260
    Lifted::new(0x70009b22a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x383> as *const (), 1), // +0x2c62a0 sub_2c62a0
    Lifted::new(0x70009b22e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x387> as *const (), 1), // +0x2c62e0 sub_2c62e0
    Lifted::new(0x70009b2320, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x38b> as *const (), 1), // +0x2c6320 sub_2c6320
    Lifted::new(0x70009b2360, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x213> as *const (), 1), // +0x2c6360 sub_2c6360
    Lifted::new(0x70009b23a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x217> as *const (), 1), // +0x2c63a0 sub_2c63a0
    Lifted::new(0x70009b23e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x38f> as *const (), 1), // +0x2c63e0 sub_2c63e0
    Lifted::new(0x70009b2420, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x21b> as *const (), 1), // +0x2c6420 sub_2c6420
    Lifted::new(0x70009b2460, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x33> as *const (), 1), // +0x2c6460 sub_2c6460
    Lifted::new(0x70009b24a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x37> as *const (), 1), // +0x2c64a0 sub_2c64a0
    Lifted::new(0x70009b24e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x227> as *const (), 1), // +0x2c64e0 sub_2c64e0
    Lifted::new(0x70009b2520, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x21f> as *const (), 1), // +0x2c6520 sub_2c6520
    Lifted::new(0x70009b2560, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x223> as *const (), 1), // +0x2c6560 sub_2c6560
    Lifted::new(0x70009b25a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x22b> as *const (), 1), // +0x2c65a0 sub_2c65a0
    Lifted::new(0x70009b25e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x303> as *const (), 1), // +0x2c65e0 sub_2c65e0
    Lifted::new(0x70009b2620, tpl_351 as *const (), 1), // +0x2c6620 sub_2c6620
    Lifted::new(0x70009b2630, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1ef> as *const (), 1), // +0x2c6630 sub_2c6630
    Lifted::new(0x70009b2670, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1e7> as *const (), 1), // +0x2c6670 sub_2c6670
    Lifted::new(0x70009b26b0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x22f> as *const (), 1), // +0x2c66b0 sub_2c66b0
    Lifted::new(0x70009b26f0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x233> as *const (), 1), // +0x2c66f0 sub_2c66f0
    Lifted::new(0x70009b2730, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x27> as *const (), 1), // +0x2c6730 sub_2c6730
    Lifted::new(0x70009b2770, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x237> as *const (), 1), // +0x2c6770 sub_2c6770
    Lifted::new(0x70009b27b0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x23b> as *const (), 1), // +0x2c67b0 sub_2c67b0
    Lifted::new(0x70009b27f0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x39f> as *const (), 1), // +0x2c67f0 sub_2c67f0
    Lifted::new(0x70009b2830, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3a3> as *const (), 1), // +0x2c6830 sub_2c6830
    Lifted::new(0x70009b2870, tpl_144 as *const (), 1), // +0x2c6870 sub_2c6870
    Lifted::new(0x70009b2880, fn_70009b2880 as *const (), 1), // +0x2c6880 sub_2c6880
    Lifted::new(0x70009b2890, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3f7> as *const (), 1), // +0x2c6890 sub_2c6890
    Lifted::new(0x70009b28d0, fn_70009b28d0 as *const (), 1), // +0x2c68d0 sub_2c68d0
    Lifted::new(0x70009b28e0, fn_70009b28e0 as *const (), 1), // +0x2c68e0 sub_2c68e0
    Lifted::new(0x70009b28f0, fn_70009b28f0 as *const (), 1), // +0x2c68f0 sub_2c68f0
    Lifted::new(0x70009b2900, tpl_47::<0xc7e0> as *const (), 1), // +0x2c6900 sub_2c6900
    Lifted::new(0x70009b2910, tpl_52::<0xc7e0> as *const (), 1), // +0x2c6910 sub_2c6910
    Lifted::new(0x70009b2920, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3a7> as *const (), 1), // +0x2c6920 sub_2c6920
    Lifted::new(0x70009b2960, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3ab> as *const (), 1), // +0x2c6960 sub_2c6960
    Lifted::new(0x70009b29a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3af> as *const (), 1), // +0x2c69a0 sub_2c69a0
    Lifted::new(0x70009b29e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x247> as *const (), 1), // +0x2c69e0 sub_2c69e0
    Lifted::new(0x70009b2a20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x243> as *const (), 1), // +0x2c6a20 sub_2c6a20
    Lifted::new(0x70009b2a60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x23f> as *const (), 1), // +0x2c6a60 sub_2c6a60
    Lifted::new(0x70009b2aa0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x24b> as *const (), 1), // +0x2c6aa0 sub_2c6aa0
    Lifted::new(0x70009b2ae0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3b> as *const (), 1), // +0x2c6ae0 sub_2c6ae0
    Lifted::new(0x70009b2b20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x263> as *const (), 1), // +0x2c6b20 sub_2c6b20
    Lifted::new(0x70009b2b60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3f> as *const (), 1), // +0x2c6b60 sub_2c6b60
    Lifted::new(0x70009b2ba0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x24f> as *const (), 1), // +0x2c6ba0 sub_2c6ba0
    Lifted::new(0x70009b2be0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x253> as *const (), 1), // +0x2c6be0 sub_2c6be0
    Lifted::new(0x70009b2c20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x257> as *const (), 1), // +0x2c6c20 sub_2c6c20
    Lifted::new(0x70009b2c60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x25b> as *const (), 1), // +0x2c6c60 sub_2c6c60
    Lifted::new(0x70009b2ca0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x25f> as *const (), 1), // +0x2c6ca0 sub_2c6ca0
    Lifted::new(0x70009b2ce0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x9b> as *const (), 1), // +0x2c6ce0 sub_2c6ce0
    Lifted::new(0x70009b2d20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x97> as *const (), 1), // +0x2c6d20 sub_2c6d20
    Lifted::new(0x70009b2d60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x8b> as *const (), 1), // +0x2c6d60 sub_2c6d60
    Lifted::new(0x70009b2da0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x8f> as *const (), 1), // +0x2c6da0 sub_2c6da0
    Lifted::new(0x70009b2de0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x7f> as *const (), 1), // +0x2c6de0 sub_2c6de0
    Lifted::new(0x70009b2e20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x87> as *const (), 1), // +0x2c6e20 sub_2c6e20
    Lifted::new(0x70009b2e60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x77> as *const (), 1), // +0x2c6e60 sub_2c6e60
    Lifted::new(0x70009b2ea0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x7b> as *const (), 1), // +0x2c6ea0 sub_2c6ea0
    Lifted::new(0x70009b2ee0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x83> as *const (), 1), // +0x2c6ee0 sub_2c6ee0
    Lifted::new(0x70009b2f20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x73> as *const (), 1), // +0x2c6f20 sub_2c6f20
    Lifted::new(0x70009b2f60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x93> as *const (), 1), // +0x2c6f60 sub_2c6f60
    Lifted::new(0x70009b2fa0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3b3> as *const (), 1), // +0x2c6fa0 sub_2c6fa0
    Lifted::new(0x70009b2fe0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x267> as *const (), 1), // +0x2c6fe0 sub_2c6fe0
    Lifted::new(0x70009b3020, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3b7> as *const (), 1), // +0x2c7020 sub_2c7020
    Lifted::new(0x70009b3060, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2b> as *const (), 1), // +0x2c7060 sub_2c7060
    Lifted::new(0x70009b30a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2f> as *const (), 1), // +0x2c70a0 sub_2c70a0
    Lifted::new(0x70009b30e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x26b> as *const (), 1), // +0x2c70e0 sub_2c70e0
    Lifted::new(0x70009b3120, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x26f> as *const (), 1), // +0x2c7120 sub_2c7120
    Lifted::new(0x70009b3160, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x273> as *const (), 1), // +0x2c7160 sub_2c7160
    Lifted::new(0x70009b31a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x277> as *const (), 1), // +0x2c71a0 sub_2c71a0
    Lifted::new(0x70009b31e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x27b> as *const (), 1), // +0x2c71e0 sub_2c71e0
    Lifted::new(0x70009b3220, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x27f> as *const (), 1), // +0x2c7220 sub_2c7220
    Lifted::new(0x70009b3260, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x283> as *const (), 1), // +0x2c7260 sub_2c7260
    Lifted::new(0x70009b32a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x287> as *const (), 1), // +0x2c72a0 sub_2c72a0
    Lifted::new(0x70009b32e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x293> as *const (), 1), // +0x2c72e0 sub_2c72e0
    Lifted::new(0x70009b3320, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x28b> as *const (), 1), // +0x2c7320 sub_2c7320
    Lifted::new(0x70009b3360, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x28f> as *const (), 1), // +0x2c7360 sub_2c7360
    Lifted::new(0x70009b33a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x297> as *const (), 1), // +0x2c73a0 sub_2c73a0
    Lifted::new(0x70009b33e0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2df> as *const (), 1), // +0x2c73e0 sub_2c73e0
    Lifted::new(0x70009b3420, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x29b> as *const (), 1), // +0x2c7420 sub_2c7420
    Lifted::new(0x70009b3460, tpl_352 as *const (), 1), // +0x2c7460 sub_2c7460
    Lifted::new(0x70009b3470, tpl_325::<0x30> as *const (), 1), // +0x2c7470 sub_2c7470
    Lifted::new(0x70009b3480, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3f3> as *const (), 1), // +0x2c7480 sub_2c7480
    Lifted::new(0x70009b34c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1f7> as *const (), 1), // +0x2c74c0 sub_2c74c0
    Lifted::new(0x70009b3500, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2a3> as *const (), 1), // +0x2c7500 sub_2c7500
    Lifted::new(0x70009b3540, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2a7> as *const (), 1), // +0x2c7540 sub_2c7540
    Lifted::new(0x70009b3580, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2ab> as *const (), 1), // +0x2c7580 sub_2c7580
    Lifted::new(0x70009b35c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2af> as *const (), 1), // +0x2c75c0 sub_2c75c0
    Lifted::new(0x70009b3600, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3bb> as *const (), 1), // +0x2c7600 sub_2c7600
    Lifted::new(0x70009b3640, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3bf> as *const (), 1), // +0x2c7640 sub_2c7640
    Lifted::new(0x70009b3680, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3c3> as *const (), 1), // +0x2c7680 sub_2c7680
    Lifted::new(0x70009b36c0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2b7> as *const (), 1), // +0x2c76c0 sub_2c76c0
    Lifted::new(0x70009b3700, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2b3> as *const (), 1), // +0x2c7700 sub_2c7700
    Lifted::new(0x70009b3740, tpl_193::<0x30> as *const (), 1), // +0x2c7740 sub_2c7740
    Lifted::new(0x70009b3750, tpl_47::<0xd9c0> as *const (), 1), // +0x2c7750 sub_2c7750
    Lifted::new(0x70009b3760, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2bb> as *const (), 1), // +0x2c7760 sub_2c7760
    Lifted::new(0x70009b37a0, fn_70009b37a0 as *const (), 1), // +0x2c77a0 sub_2c77a0
    Lifted::new(0x70009b37b0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2bf> as *const (), 1), // +0x2c77b0 sub_2c77b0
    Lifted::new(0x70009b37f0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2e3> as *const (), 1), // +0x2c77f0 sub_2c77f0
    Lifted::new(0x70009b3830, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2e7> as *const (), 1), // +0x2c7830 sub_2c7830
    Lifted::new(0x70009b3870, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2ef> as *const (), 1), // +0x2c7870 sub_2c7870
    Lifted::new(0x70009b38b0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2eb> as *const (), 1), // +0x2c78b0 sub_2c78b0
    Lifted::new(0x70009b38f0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2c3> as *const (), 1), // +0x2c78f0 sub_2c78f0
    Lifted::new(0x70009b3930, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2cb> as *const (), 1), // +0x2c7930 sub_2c7930
    Lifted::new(0x70009b3970, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2cf> as *const (), 1), // +0x2c7970 sub_2c7970
    Lifted::new(0x70009b39b0, fn_70009b39b0 as *const (), 0), // +0x2c79b0 sub_2c79b0
    Lifted::new(0x70009b3a20, tpl_47::<0x30> as *const (), 1), // +0x2c7a20 sub_2c7a20
    Lifted::new(0x70009b3a30, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2c7> as *const (), 1), // +0x2c7a30 sub_2c7a30
    Lifted::new(0x70009b3a70, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2f3> as *const (), 1), // +0x2c7a70 sub_2c7a70
    Lifted::new(0x70009b3ab0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2f7> as *const (), 1), // +0x2c7ab0 sub_2c7ab0
    Lifted::new(0x70009b3af0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2fb> as *const (), 1), // +0x2c7af0 sub_2c7af0
    Lifted::new(0x70009b3b30, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2ff> as *const (), 1), // +0x2c7b30 sub_2c7b30
    Lifted::new(0x70009b3b70, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x323> as *const (), 1), // +0x2c7b70 sub_2c7b70
    Lifted::new(0x70009b3bb0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x327> as *const (), 1), // +0x2c7bb0 sub_2c7bb0
    Lifted::new(0x70009b3bf0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x32b> as *const (), 1), // +0x2c7bf0 sub_2c7bf0
    Lifted::new(0x70009b3c30, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3c7> as *const (), 1), // +0x2c7c30 sub_2c7c30
    Lifted::new(0x70009b3c70, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x34f> as *const (), 1), // +0x2c7c70 sub_2c7c70
    Lifted::new(0x70009b3cb0, tpl_194 as *const (), 1), // +0x2c7cb0 sub_2c7cb0
    Lifted::new(0x70009b3cc0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3cb> as *const (), 1), // +0x2c7cc0 sub_2c7cc0
    Lifted::new(0x70009b3d00, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x353> as *const (), 1), // +0x2c7d00 sub_2c7d00
    Lifted::new(0x70009b3d40, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x357> as *const (), 1), // +0x2c7d40 sub_2c7d40
    Lifted::new(0x70009b3d80, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x4f> as *const (), 1), // +0x2c7d80 sub_2c7d80
    Lifted::new(0x70009b3dc0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x57> as *const (), 1), // +0x2c7dc0 sub_2c7dc0
    Lifted::new(0x70009b3e00, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x47> as *const (), 1), // +0x2c7e00 sub_2c7e00
    Lifted::new(0x70009b3e40, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x67> as *const (), 1), // +0x2c7e40 sub_2c7e40
    Lifted::new(0x70009b3e80, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3cf> as *const (), 1), // +0x2c7e80 sub_2c7e80
    Lifted::new(0x70009b3ec0, fn_70009b3ec0 as *const (), 1), // +0x2c7ec0 sub_2c7ec0
    Lifted::new(0x70009b3ed0, fn_70009b3ed0 as *const (), 1), // +0x2c7ed0 sub_2c7ed0
    Lifted::new(0x70009b3ee0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3d3> as *const (), 1), // +0x2c7ee0 sub_2c7ee0
    Lifted::new(0x70009b3f20, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2db> as *const (), 1), // +0x2c7f20 sub_2c7f20
    Lifted::new(0x70009b3f60, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x337> as *const (), 1), // +0x2c7f60 sub_2c7f60
    Lifted::new(0x70009b3fa0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3df> as *const (), 1), // +0x2c7fa0 sub_2c7fa0
    Lifted::new(0x70009b3fe0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x32f> as *const (), 1), // +0x2c7fe0 sub_2c7fe0
    Lifted::new(0x70009b4020, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x33b> as *const (), 1), // +0x2c8020 sub_2c8020
    Lifted::new(0x70009b4060, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x33f> as *const (), 1), // +0x2c8060 sub_2c8060
    Lifted::new(0x70009b40a0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3d7> as *const (), 1), // +0x2c80a0 sub_2c80a0
    Lifted::new(0x70009b40e0, tpl_353::<0x30> as *const (), 1), // +0x2c80e0 sub_2c80e0
    Lifted::new(0x70009b40f0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x343> as *const (), 1), // +0x2c80f0 sub_2c80f0
    Lifted::new(0x70009b4130, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x347> as *const (), 1), // +0x2c8130 sub_2c8130
    Lifted::new(0x70009b4170, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3db> as *const (), 1), // +0x2c8170 sub_2c8170
    Lifted::new(0x70009b41b0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x34b> as *const (), 1), // +0x2c81b0 sub_2c81b0
    Lifted::new(0x70009b41f0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x333> as *const (), 1), // +0x2c81f0 sub_2c81f0
    Lifted::new(0x70009b4230, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3eb> as *const (), 1), // +0x2c8230 sub_2c8230
    Lifted::new(0x70009b4270, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3e7> as *const (), 1), // +0x2c8270 sub_2c8270
    Lifted::new(0x70009b42b0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3e3> as *const (), 1), // +0x2c82b0 sub_2c82b0
    Lifted::new(0x70009b42f0, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3ef> as *const (), 1), // +0x2c82f0 sub_2c82f0
    Lifted::new(0x70009b4330, tpl_345::<0x2fe8, 0xffffffffffffffff, 0x13, 0x207> as *const (), 1), // +0x2c8330 sub_2c8330
    Lifted::new(0x70009b4370, tpl_354::<0x9> as *const (), 1), // +0x2c8370 sub_2c8370
    Lifted::new(0x70009b4380, tpl_20 as *const (), 1), // +0x2c8380 sub_2c8380
    Lifted::new(0x70009b4390, tpl_20 as *const (), 1), // +0x2c8390 sub_2c8390
    Lifted::new(0x70009b43a0, tpl_20 as *const (), 1), // +0x2c83a0 sub_2c83a0
    Lifted::new(0x70009b43b0, tpl_30 as *const (), 1), // +0x2c83b0 sub_2c83b0
    Lifted::new(0x70009b43c0, tpl_20 as *const (), 1), // +0x2c83c0 sub_2c83c0
    Lifted::new(0x70009b43d0, tpl_36::<0x18> as *const (), 1), // +0x2c83d0 sub_2c83d0
    Lifted::new(0x70009b43e0, tpl_30 as *const (), 1), // +0x2c83e0 sub_2c83e0
    Lifted::new(0x70009b43f0, tpl_36::<0x7800> as *const (), 1), // +0x2c83f0 sub_2c83f0
    Lifted::new(0x70009b4400, tpl_20 as *const (), 1), // +0x2c8400 sub_2c8400
    Lifted::new(0x70009b4410, tpl_20 as *const (), 1), // +0x2c8410 sub_2c8410
    Lifted::new(0x70009b4420, tpl_30 as *const (), 1), // +0x2c8420 sub_2c8420
    Lifted::new(0x70009b4430, tpl_20 as *const (), 1), // +0x2c8430 sub_2c8430
    Lifted::new(0x70009b4440, tpl_328::<0x5f> as *const (), 1), // +0x2c8440 sub_2c8440
    Lifted::new(0x70009b4450, tpl_196 as *const (), 1), // +0x2c8450 sub_2c8450
    Lifted::new(0x70009b4460, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1a7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8460 sub_2c8460
    Lifted::new(0x70009b4530, tpl_142::<0xcb8> as *const (), 1), // +0x2c8530 sub_2c8530
    Lifted::new(0x70009b4550, tpl_47::<0xc971> as *const (), 1), // +0x2c8550 sub_2c8550
    Lifted::new(0x70009b4560, tpl_52::<0xc971> as *const (), 1), // +0x2c8560 sub_2c8560
    Lifted::new(0x70009b4570, tpl_47::<0xc973> as *const (), 1), // +0x2c8570 sub_2c8570
    Lifted::new(0x70009b4580, tpl_47::<0xc972> as *const (), 1), // +0x2c8580 sub_2c8580
    Lifted::new(0x70009b4590, tpl_52::<0xc8> as *const (), 1), // +0x2c8590 sub_2c8590
    Lifted::new(0x70009b45a0, tpl_142::<0x7f8> as *const (), 1), // +0x2c85a0 sub_2c85a0
    Lifted::new(0x70009b45c0, tpl_142::<0x800> as *const (), 1), // +0x2c85c0 sub_2c85c0
    Lifted::new(0x70009b45e0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1bf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c85e0 sub_2c85e0
    Lifted::new(0x70009b46b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1b7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c86b0 sub_2c86b0
    Lifted::new(0x70009b4780, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1af, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8780 sub_2c8780
    Lifted::new(0x70009b4850, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1bb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8850 sub_2c8850
    Lifted::new(0x70009b4920, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1b3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8920 sub_2c8920
    Lifted::new(0x70009b49f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1ab, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c89f0 sub_2c89f0
    Lifted::new(0x70009b4ac0, tpl_36::<0x3070> as *const (), 1), // +0x2c8ac0 sub_2c8ac0
    Lifted::new(0x70009b4ad0, tpl_52::<0x3070> as *const (), 1), // +0x2c8ad0 sub_2c8ad0
    Lifted::new(0x70009b4ae0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1e3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8ae0 sub_2c8ae0
    Lifted::new(0x70009b4bb0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1c3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8bb0 sub_2c8bb0
    Lifted::new(0x70009b4c80, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1c7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8c80 sub_2c8c80
    Lifted::new(0x70009b4d50, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1cb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8d50 sub_2c8d50
    Lifted::new(0x70009b4e20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1cf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8e20 sub_2c8e20
    Lifted::new(0x70009b4ef0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1d3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8ef0 sub_2c8ef0
    Lifted::new(0x70009b4fc0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1d7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c8fc0 sub_2c8fc0
    Lifted::new(0x70009b5090, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1df, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9090 sub_2c9090
    Lifted::new(0x70009b5160, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1db, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9160 sub_2c9160
    Lifted::new(0x70009b5230, tpl_36::<0x18> as *const (), 1), // +0x2c9230 sub_2c9230
    Lifted::new(0x70009b5240, tpl_52::<0xb758> as *const (), 1), // +0x2c9240 sub_2c9240
    Lifted::new(0x70009b5250, tpl_52::<0xc978> as *const (), 1), // +0x2c9250 sub_2c9250
    Lifted::new(0x70009b5260, tpl_52::<0xaf58> as *const (), 1), // +0x2c9260 sub_2c9260
    Lifted::new(0x70009b5270, tpl_355::<0xc8, 0x8> as *const (), 1), // +0x2c9270 sub_2c9270
    Lifted::new(0x70009b5290, tpl_328::<0x67> as *const (), 1), // +0x2c9290 sub_2c9290
    Lifted::new(0x70009b52a0, fn_70009b52a0 as *const (), 0), // +0x2c92a0 sub_2c92a0
    Lifted::new(0x70009b52e0, tpl_142::<0x808> as *const (), 1), // +0x2c92e0 sub_2c92e0
    Lifted::new(0x70009b5300, tpl_47::<0x14> as *const (), 1), // +0x2c9300 sub_2c9300
    Lifted::new(0x70009b5310, fn_70009b5310 as *const (), 1), // +0x2c9310 sub_2c9310
    Lifted::new(0x70009b5320, tpl_47::<0x100> as *const (), 1), // +0x2c9320 sub_2c9320
    Lifted::new(0x70009b5330, tpl_353::<0x78> as *const (), 1), // +0x2c9330 sub_2c9330
    Lifted::new(0x70009b5340, tpl_142::<0x810> as *const (), 1), // +0x2c9340 sub_2c9340
    Lifted::new(0x70009b5360, tpl_39::<0xc94c> as *const (), 1), // +0x2c9360 sub_2c9360
    Lifted::new(0x70009b5370, tpl_142::<0x1188> as *const (), 1), // +0x2c9370 sub_2c9370
    Lifted::new(0x70009b5390, tpl_36::<0x1f8> as *const (), 1), // +0x2c9390 sub_2c9390
    Lifted::new(0x70009b53a0, tpl_36::<0xc8> as *const (), 1), // +0x2c93a0 sub_2c93a0
    Lifted::new(0x70009b53b0, tpl_36::<0xc8> as *const (), 1), // +0x2c93b0 sub_2c93b0
    Lifted::new(0x70009b53c0, tpl_142::<0x1180> as *const (), 1), // +0x2c93c0 sub_2c93c0
    Lifted::new(0x70009b53e0, tpl_47::<0x4c> as *const (), 1), // +0x2c93e0 sub_2c93e0
    Lifted::new(0x70009b53f0, tpl_36::<0xc828> as *const (), 1), // +0x2c93f0 sub_2c93f0
    Lifted::new(0x70009b5400, tpl_47::<0x181> as *const (), 1), // +0x2c9400 sub_2c9400
    Lifted::new(0x70009b5410, tpl_39::<0x10> as *const (), 1), // +0x2c9410 sub_2c9410
    Lifted::new(0x70009b5420, tpl_30 as *const (), 1), // +0x2c9420 sub_2c9420
    Lifted::new(0x70009b5430, tpl_39::<0x24> as *const (), 1), // +0x2c9430 sub_2c9430
    Lifted::new(0x70009b5440, tpl_356::<0x8> as *const (), 1), // +0x2c9440 sub_2c9440
    Lifted::new(0x70009b5450, tpl_142::<0x818> as *const (), 1), // +0x2c9450 sub_2c9450
    Lifted::new(0x70009b5470, tpl_142::<0x820> as *const (), 1), // +0x2c9470 sub_2c9470
    Lifted::new(0x70009b5490, tpl_36::<0x188> as *const (), 1), // +0x2c9490 sub_2c9490
    Lifted::new(0x70009b54a0, tpl_36::<0x190> as *const (), 1), // +0x2c94a0 sub_2c94a0
    Lifted::new(0x70009b54b0, tpl_142::<0x828> as *const (), 1), // +0x2c94b0 sub_2c94b0
    Lifted::new(0x70009b54d0, tpl_142::<0x830> as *const (), 1), // +0x2c94d0 sub_2c94d0
    Lifted::new(0x70009b54f0, tpl_36::<0x70> as *const (), 1), // +0x2c94f0 sub_2c94f0
    Lifted::new(0x70009b5500, tpl_142::<0xee8> as *const (), 1), // +0x2c9500 sub_2c9500
    Lifted::new(0x70009b5520, tpl_142::<0xef0> as *const (), 1), // +0x2c9520 sub_2c9520
    Lifted::new(0x70009b5540, tpl_142::<0xef8> as *const (), 1), // +0x2c9540 sub_2c9540
    Lifted::new(0x70009b5560, tpl_36::<0xa698> as *const (), 1), // +0x2c9560 sub_2c9560
    Lifted::new(0x70009b5570, tpl_275::<0x8, 0x2068> as *const (), 1), // +0x2c9570 sub_2c9570
    Lifted::new(0x70009b5580, fn_70009b5580 as *const (), 1), // +0x2c9580 sub_2c9580
    Lifted::new(0x70009b5590, tpl_36::<0xa690> as *const (), 1), // +0x2c9590 sub_2c9590
    Lifted::new(0x70009b55a0, tpl_36::<0x98> as *const (), 1), // +0x2c95a0 sub_2c95a0
    Lifted::new(0x70009b55b0, tpl_275::<0x10, 0x48> as *const (), 1), // +0x2c95b0 sub_2c95b0
    Lifted::new(0x70009b55c0, tpl_36::<0x48> as *const (), 1), // +0x2c95c0 sub_2c95c0
    Lifted::new(0x70009b55d0, tpl_142::<0x840> as *const (), 1), // +0x2c95d0 sub_2c95d0
    Lifted::new(0x70009b55f0, tpl_191::<0x40> as *const (), 1), // +0x2c95f0 sub_2c95f0
    Lifted::new(0x70009b5600, tpl_36::<0xd920> as *const (), 1), // +0x2c9600 sub_2c9600
    Lifted::new(0x70009b5610, tpl_333::<0xe3> as *const (), 1), // +0x2c9610 sub_2c9610
    Lifted::new(0x70009b5630, tpl_334::<0xe3> as *const (), 1), // +0x2c9630 sub_2c9630
    Lifted::new(0x70009b5660, tpl_333::<0xef> as *const (), 1), // +0x2c9660 sub_2c9660
    Lifted::new(0x70009b5680, tpl_334::<0xef> as *const (), 1), // +0x2c9680 sub_2c9680
    Lifted::new(0x70009b56b0, tpl_142::<0xf90> as *const (), 1), // +0x2c96b0 sub_2c96b0
    Lifted::new(0x70009b56d0, tpl_142::<0x1f8> as *const (), 1), // +0x2c96d0 sub_2c96d0
    Lifted::new(0x70009b56f0, tpl_191::<0x8> as *const (), 1), // +0x2c96f0 sub_2c96f0
    Lifted::new(0x70009b5700, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x20f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9700 sub_2c9700
    Lifted::new(0x70009b57d0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x383, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c97d0 sub_2c97d0
    Lifted::new(0x70009b58a0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x387, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c98a0 sub_2c98a0
    Lifted::new(0x70009b5970, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x38b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9970 sub_2c9970
    Lifted::new(0x70009b5a40, tpl_142::<0x1008> as *const (), 1), // +0x2c9a40 sub_2c9a40
    Lifted::new(0x70009b5a60, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x213, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9a60 sub_2c9a60
    Lifted::new(0x70009b5b30, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x217, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9b30 sub_2c9b30
    Lifted::new(0x70009b5c00, tpl_357::<0x70017d2c10, 0x7001d54ef8, 0x70009b5c1c, 0x8> as *const (), 0), // +0x2c9c00 sub_2c9c00
    Lifted::new(0x70009b5c40, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x38f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9c40 sub_2c9c40
    Lifted::new(0x70009b5d10, tpl_52::<0xa680> as *const (), 1), // +0x2c9d10 sub_2c9d10
    Lifted::new(0x70009b5d20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x21b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9d20 sub_2c9d20
    Lifted::new(0x70009b5df0, tpl_36::<0x38> as *const (), 1), // +0x2c9df0 sub_2c9df0
    Lifted::new(0x70009b5e00, tpl_52::<0xb0> as *const (), 1), // +0x2c9e00 sub_2c9e00
    Lifted::new(0x70009b5e10, tpl_142::<0xcc8> as *const (), 1), // +0x2c9e10 sub_2c9e10
    Lifted::new(0x70009b5e30, tpl_142::<0xcd0> as *const (), 1), // +0x2c9e30 sub_2c9e30
    Lifted::new(0x70009b5e50, tpl_36::<0xa6b8> as *const (), 1), // +0x2c9e50 sub_2c9e50
    Lifted::new(0x70009b5e60, tpl_142::<0x1118> as *const (), 1), // +0x2c9e60 sub_2c9e60
    Lifted::new(0x70009b5e80, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x33, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9e80 sub_2c9e80
    Lifted::new(0x70009b5f50, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x37, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2c9f50 sub_2c9f50
    Lifted::new(0x70009b6020, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x227, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ca020 sub_2ca020
    Lifted::new(0x70009b60f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x21f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ca0f0 sub_2ca0f0
    Lifted::new(0x70009b61c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x223, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ca1c0 sub_2ca1c0
    Lifted::new(0x70009b6290, tpl_36::<0x10> as *const (), 1), // +0x2ca290 sub_2ca290
    Lifted::new(0x70009b62a0, tpl_142::<0x858> as *const (), 1), // +0x2ca2a0 sub_2ca2a0
    Lifted::new(0x70009b62c0, tpl_39::<0x3c> as *const (), 1), // +0x2ca2c0 sub_2ca2c0
    Lifted::new(0x70009b62d0, tpl_39::<0x4> as *const (), 1), // +0x2ca2d0 sub_2ca2d0
    Lifted::new(0x70009b62e0, fn_70009b62e0 as *const (), 1), // +0x2ca2e0 sub_2ca2e0
    Lifted::new(0x70009b62f0, fn_70009b62f0 as *const (), 1), // +0x2ca2f0 sub_2ca2f0
    Lifted::new(0x70009b6300, tpl_142::<0x860> as *const (), 1), // +0x2ca300 sub_2ca300
    Lifted::new(0x70009b6320, tpl_142::<0x200> as *const (), 1), // +0x2ca320 sub_2ca320
    Lifted::new(0x70009b6340, tpl_142::<0xfb0> as *const (), 1), // +0x2ca340 sub_2ca340
    Lifted::new(0x70009b6360, tpl_142::<0xc00> as *const (), 1), // +0x2ca360 sub_2ca360
    Lifted::new(0x70009b6380, tpl_142::<0xc08> as *const (), 1), // +0x2ca380 sub_2ca380
    Lifted::new(0x70009b63a0, tpl_52::<0xc0> as *const (), 1), // +0x2ca3a0 sub_2ca3a0
    Lifted::new(0x70009b63b0, tpl_328::<0x27> as *const (), 1), // +0x2ca3b0 sub_2ca3b0
    Lifted::new(0x70009b63c0, tpl_333::<0xcb> as *const (), 1), // +0x2ca3c0 sub_2ca3c0
    Lifted::new(0x70009b63e0, tpl_334::<0xcb> as *const (), 1), // +0x2ca3e0 sub_2ca3e0
    Lifted::new(0x70009b6410, tpl_328::<0x1f> as *const (), 1), // +0x2ca410 sub_2ca410
    Lifted::new(0x70009b6420, tpl_328::<0x17> as *const (), 1), // +0x2ca420 sub_2ca420
    Lifted::new(0x70009b6430, tpl_142::<0x1148> as *const (), 1), // +0x2ca430 sub_2ca430
    Lifted::new(0x70009b6450, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x22b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ca450 sub_2ca450
    Lifted::new(0x70009b6520, tpl_142::<0xf80> as *const (), 1), // +0x2ca520 sub_2ca520
    Lifted::new(0x70009b6540, tpl_142::<0x868> as *const (), 1), // +0x2ca540 sub_2ca540
    Lifted::new(0x70009b6560, tpl_142::<0xd0> as *const (), 1), // +0x2ca560 sub_2ca560
    Lifted::new(0x70009b6580, tpl_142::<0x870> as *const (), 1), // +0x2ca580 sub_2ca580
    Lifted::new(0x70009b65a0, tpl_333::<0xb> as *const (), 1), // +0x2ca5a0 sub_2ca5a0
    Lifted::new(0x70009b65c0, tpl_334::<0xb> as *const (), 1), // +0x2ca5c0 sub_2ca5c0
    Lifted::new(0x70009b65f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x303, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ca5f0 sub_2ca5f0
    Lifted::new(0x70009b66c0, tpl_52::<0xd7b0> as *const (), 1), // +0x2ca6c0 sub_2ca6c0
    Lifted::new(0x70009b66d0, tpl_39::<0x48> as *const (), 1), // +0x2ca6d0 sub_2ca6d0
    Lifted::new(0x70009b66e0, tpl_358 as *const (), 1), // +0x2ca6e0 sub_2ca6e0
    Lifted::new(0x70009b66f0, tpl_142::<0x878> as *const (), 1), // +0x2ca6f0 sub_2ca6f0
    Lifted::new(0x70009b6710, tpl_142::<0x468> as *const (), 1), // +0x2ca710 sub_2ca710
    Lifted::new(0x70009b6730, tpl_142::<0x460> as *const (), 1), // +0x2ca730 sub_2ca730
    Lifted::new(0x70009b6750, tpl_41 as *const (), 1), // +0x2ca750 sub_2ca750
    Lifted::new(0x70009b6760, fn_70009b6760 as *const (), 1), // +0x2ca760 sub_2ca760
    Lifted::new(0x70009b6770, tpl_36::<0xc888> as *const (), 1), // +0x2ca770 sub_2ca770
    Lifted::new(0x70009b6780, tpl_36::<0xc880> as *const (), 1), // +0x2ca780 sub_2ca780
    Lifted::new(0x70009b6790, tpl_36::<0xa0> as *const (), 1), // +0x2ca790 sub_2ca790
    Lifted::new(0x70009b67a0, tpl_359 as *const (), 3), // +0x2ca7a0 sub_2ca7a0
    Lifted::new(0x70009b67b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1ef, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ca7b0 sub_2ca7b0
    Lifted::new(0x70009b6880, tpl_142::<0xe20> as *const (), 1), // +0x2ca880 sub_2ca880
    Lifted::new(0x70009b68a0, tpl_142::<0x208> as *const (), 1), // +0x2ca8a0 sub_2ca8a0
    Lifted::new(0x70009b68c0, tpl_333::<0xbf> as *const (), 1), // +0x2ca8c0 sub_2ca8c0
    Lifted::new(0x70009b68e0, tpl_334::<0xbf> as *const (), 1), // +0x2ca8e0 sub_2ca8e0
    Lifted::new(0x70009b6910, tpl_142::<0xde8> as *const (), 1), // +0x2ca910 sub_2ca910
    Lifted::new(0x70009b6930, tpl_142::<0x888> as *const (), 1), // +0x2ca930 sub_2ca930
    Lifted::new(0x70009b6950, tpl_52::<0x138> as *const (), 1), // +0x2ca950 sub_2ca950
    Lifted::new(0x70009b6960, fn_70009b6960 as *const (), 0), // +0x2ca960 sub_2ca960
    Lifted::new(0x70009b6970, tpl_191::<0x20> as *const (), 1), // +0x2ca970 sub_2ca970
    Lifted::new(0x70009b6980, tpl_36::<0x30> as *const (), 1), // +0x2ca980 sub_2ca980
    Lifted::new(0x70009b6990, tpl_142::<0x210> as *const (), 1), // +0x2ca990 sub_2ca990
    Lifted::new(0x70009b69b0, tpl_142::<0x890> as *const (), 1), // +0x2ca9b0 sub_2ca9b0
    Lifted::new(0x70009b69d0, tpl_142::<0x448> as *const (), 1), // +0x2ca9d0 sub_2ca9d0
    Lifted::new(0x70009b69f0, tpl_142::<0x8a0> as *const (), 1), // +0x2ca9f0 sub_2ca9f0
    Lifted::new(0x70009b6a10, fn_70009b6a10 as *const (), 0), // +0x2caa10 sub_2caa10
    Lifted::new(0x70009b6ac0, tpl_333::<0xc7> as *const (), 1), // +0x2caac0 sub_2caac0
    Lifted::new(0x70009b6ae0, tpl_334::<0xc7> as *const (), 1), // +0x2caae0 sub_2caae0
    Lifted::new(0x70009b6b10, tpl_142::<0xc10> as *const (), 1), // +0x2cab10 sub_2cab10
    Lifted::new(0x70009b6b30, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1e7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cab30 sub_2cab30
    Lifted::new(0x70009b6c00, tpl_30 as *const (), 1), // +0x2cac00 sub_2cac00
    Lifted::new(0x70009b6c10, tpl_142::<0x4a8> as *const (), 1), // +0x2cac10 sub_2cac10
    Lifted::new(0x70009b6c30, tpl_142::<0x8a8> as *const (), 1), // +0x2cac30 sub_2cac30
    Lifted::new(0x70009b6c50, tpl_47::<0x9> as *const (), 1), // +0x2cac50 sub_2cac50
    Lifted::new(0x70009b6c60, tpl_328::<0x6f> as *const (), 1), // +0x2cac60 sub_2cac60
    Lifted::new(0x70009b6c70, tpl_328::<0x77> as *const (), 1), // +0x2cac70 sub_2cac70
    Lifted::new(0x70009b6c80, tpl_142::<0x8b0> as *const (), 1), // +0x2cac80 sub_2cac80
    Lifted::new(0x70009b6ca0, tpl_36::<0x30> as *const (), 1), // +0x2caca0 sub_2caca0
    Lifted::new(0x70009b6cb0, tpl_142::<0x268> as *const (), 1), // +0x2cacb0 sub_2cacb0
    Lifted::new(0x70009b6cd0, tpl_146::<0x13> as *const (), 1), // +0x2cacd0 sub_2cacd0
    Lifted::new(0x70009b6ce0, tpl_146::<0x13> as *const (), 1), // +0x2cace0 sub_2cace0
    Lifted::new(0x70009b6cf0, tpl_142::<0x8b8> as *const (), 1), // +0x2cacf0 sub_2cacf0
    Lifted::new(0x70009b6d10, tpl_142::<0x1190> as *const (), 1), // +0x2cad10 sub_2cad10
    Lifted::new(0x70009b6d30, tpl_47::<0xc958> as *const (), 1), // +0x2cad30 sub_2cad30
    Lifted::new(0x70009b6d40, tpl_142::<0x218> as *const (), 1), // +0x2cad40 sub_2cad40
    Lifted::new(0x70009b6d60, tpl_142::<0xfa8> as *const (), 1), // +0x2cad60 sub_2cad60
    Lifted::new(0x70009b6d80, tpl_47::<0xc976> as *const (), 1), // +0x2cad80 sub_2cad80
    Lifted::new(0x70009b6d90, tpl_36::<0xc968> as *const (), 1), // +0x2cad90 sub_2cad90
    Lifted::new(0x70009b6da0, tpl_39::<0x4> as *const (), 1), // +0x2cada0 sub_2cada0
    Lifted::new(0x70009b6db0, tpl_52::<0xb2> as *const (), 1), // +0x2cadb0 sub_2cadb0
    Lifted::new(0x70009b6dc0, tpl_142::<0xc18> as *const (), 1), // +0x2cadc0 sub_2cadc0
    Lifted::new(0x70009b6de0, tpl_142::<0x430> as *const (), 1), // +0x2cade0 sub_2cade0
    Lifted::new(0x70009b6e00, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x22f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cae00 sub_2cae00
    Lifted::new(0x70009b6ed0, tpl_142::<0x8e0> as *const (), 1), // +0x2caed0 sub_2caed0
    Lifted::new(0x70009b6ef0, tpl_142::<0xc20> as *const (), 1), // +0x2caef0 sub_2caef0
    Lifted::new(0x70009b6f10, tpl_142::<0x378> as *const (), 1), // +0x2caf10 sub_2caf10
    Lifted::new(0x70009b6f30, tpl_142::<0x8e8> as *const (), 1), // +0x2caf30 sub_2caf30
    Lifted::new(0x70009b6f50, tpl_142::<0x8f0> as *const (), 1), // +0x2caf50 sub_2caf50
    Lifted::new(0x70009b6f70, tpl_142::<0xb0> as *const (), 1), // +0x2caf70 sub_2caf70
    Lifted::new(0x70009b6f90, tpl_36::<0xc938> as *const (), 1), // +0x2caf90 sub_2caf90
    Lifted::new(0x70009b6fa0, fn_70009b6fa0 as *const (), 1), // +0x2cafa0 sub_2cafa0
    Lifted::new(0x70009b6fc0, tpl_360::<0xa0, 0x44> as *const (), 1), // +0x2cafc0 sub_2cafc0
    Lifted::new(0x70009b6fd0, tpl_36::<0xf0> as *const (), 1), // +0x2cafd0 sub_2cafd0
    Lifted::new(0x70009b6fe0, tpl_328::<0xb7> as *const (), 1), // +0x2cafe0 sub_2cafe0
    Lifted::new(0x70009b6ff0, tpl_259::<0x3c> as *const (), 1), // +0x2caff0 sub_2caff0
    Lifted::new(0x70009b7000, tpl_142::<0x220> as *const (), 1), // +0x2cb000 sub_2cb000
    Lifted::new(0x70009b7020, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x233, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cb020 sub_2cb020
    Lifted::new(0x70009b70f0, tpl_142::<0x900> as *const (), 1), // +0x2cb0f0 sub_2cb0f0
    Lifted::new(0x70009b7110, tpl_142::<0x10f0> as *const (), 1), // +0x2cb110 sub_2cb110
    Lifted::new(0x70009b7130, tpl_142::<0x908> as *const (), 1), // +0x2cb130 sub_2cb130
    Lifted::new(0x70009b7150, tpl_142::<0x1c8> as *const (), 1), // +0x2cb150 sub_2cb150
    Lifted::new(0x70009b7170, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x27, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cb170 sub_2cb170
    Lifted::new(0x70009b7240, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x237, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cb240 sub_2cb240
    Lifted::new(0x70009b7310, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x23b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cb310 sub_2cb310
    Lifted::new(0x70009b73e0, tpl_142::<0x918> as *const (), 1), // +0x2cb3e0 sub_2cb3e0
    Lifted::new(0x70009b7400, tpl_142::<0xd50> as *const (), 1), // +0x2cb400 sub_2cb400
    Lifted::new(0x70009b7420, tpl_142::<0x920> as *const (), 1), // +0x2cb420 sub_2cb420
    Lifted::new(0x70009b7440, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x39f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cb440 sub_2cb440
    Lifted::new(0x70009b7510, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3a3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cb510 sub_2cb510
    Lifted::new(0x70009b75e0, tpl_142::<0x928> as *const (), 1), // +0x2cb5e0 sub_2cb5e0
    Lifted::new(0x70009b7600, tpl_142::<0x480> as *const (), 1), // +0x2cb600 sub_2cb600
    Lifted::new(0x70009b7620, tpl_142::<0x930> as *const (), 1), // +0x2cb620 sub_2cb620
    Lifted::new(0x70009b7640, tpl_328::<0x7f> as *const (), 1), // +0x2cb640 sub_2cb640
    Lifted::new(0x70009b7650, tpl_328::<0x87> as *const (), 1), // +0x2cb650 sub_2cb650
    Lifted::new(0x70009b7660, tpl_52::<0x58> as *const (), 1), // +0x2cb660 sub_2cb660
    Lifted::new(0x70009b7670, tpl_142::<0xe78> as *const (), 1), // +0x2cb670 sub_2cb670
    Lifted::new(0x70009b7690, tpl_52::<0x18> as *const (), 1), // +0x2cb690 sub_2cb690
    Lifted::new(0x70009b76a0, tpl_142::<0x108> as *const (), 1), // +0x2cb6a0 sub_2cb6a0
    Lifted::new(0x70009b76c0, tpl_142::<0x100> as *const (), 1), // +0x2cb6c0 sub_2cb6c0
    Lifted::new(0x70009b76e0, tpl_142::<0x228> as *const (), 1), // +0x2cb6e0 sub_2cb6e0
    Lifted::new(0x70009b7700, tpl_142::<0x88> as *const (), 1), // +0x2cb700 sub_2cb700
    Lifted::new(0x70009b7720, tpl_142::<0x938> as *const (), 1), // +0x2cb720 sub_2cb720
    Lifted::new(0x70009b7740, tpl_47::<0xc959> as *const (), 1), // +0x2cb740 sub_2cb740
    Lifted::new(0x70009b7750, tpl_36::<0xc868> as *const (), 1), // +0x2cb750 sub_2cb750
    Lifted::new(0x70009b7760, tpl_142::<0x190> as *const (), 1), // +0x2cb760 sub_2cb760
    Lifted::new(0x70009b7780, tpl_142::<0x3a8> as *const (), 1), // +0x2cb780 sub_2cb780
    Lifted::new(0x70009b77a0, tpl_36::<0xd758> as *const (), 1), // +0x2cb7a0 sub_2cb7a0
    Lifted::new(0x70009b77b0, tpl_142::<0x230> as *const (), 1), // +0x2cb7b0 sub_2cb7b0
    Lifted::new(0x70009b77d0, tpl_142::<0x238> as *const (), 1), // +0x2cb7d0 sub_2cb7d0
    Lifted::new(0x70009b77f0, tpl_142::<0x240> as *const (), 1), // +0x2cb7f0 sub_2cb7f0
    Lifted::new(0x70009b7810, tpl_36::<0x148> as *const (), 1), // +0x2cb810 sub_2cb810
    Lifted::new(0x70009b7820, tpl_36::<0x120> as *const (), 1), // +0x2cb820 sub_2cb820
    Lifted::new(0x70009b7830, tpl_36::<0x140> as *const (), 1), // +0x2cb830 sub_2cb830
    Lifted::new(0x70009b7840, tpl_36::<0x118> as *const (), 1), // +0x2cb840 sub_2cb840
    Lifted::new(0x70009b7850, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3f7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cb850 sub_2cb850
    Lifted::new(0x70009b7920, tpl_142::<0x940> as *const (), 1), // +0x2cb920 sub_2cb920
    Lifted::new(0x70009b7940, tpl_361 as *const (), 1), // +0x2cb940 sub_2cb940
    Lifted::new(0x70009b7950, tpl_142::<0x948> as *const (), 1), // +0x2cb950 sub_2cb950
    Lifted::new(0x70009b7970, tpl_36::<0x8> as *const (), 1), // +0x2cb970 sub_2cb970
    Lifted::new(0x70009b7980, tpl_275::<0x10, 0x50> as *const (), 1), // +0x2cb980 sub_2cb980
    Lifted::new(0x70009b7990, tpl_36::<0x50> as *const (), 1), // +0x2cb990 sub_2cb990
    Lifted::new(0x70009b79a0, tpl_39::<0x38> as *const (), 1), // +0x2cb9a0 sub_2cb9a0
    Lifted::new(0x70009b79b0, tpl_36::<0x8> as *const (), 1), // +0x2cb9b0 sub_2cb9b0
    Lifted::new(0x70009b79c0, tpl_362 as *const (), 1), // +0x2cb9c0 sub_2cb9c0
    Lifted::new(0x70009b79d0, tpl_36::<0x70> as *const (), 1), // +0x2cb9d0 sub_2cb9d0
    Lifted::new(0x70009b79e0, tpl_52::<0x108> as *const (), 1), // +0x2cb9e0 sub_2cb9e0
    Lifted::new(0x70009b79f0, tpl_52::<0x128> as *const (), 1), // +0x2cb9f0 sub_2cb9f0
    Lifted::new(0x70009b7a00, tpl_52::<0x150> as *const (), 1), // +0x2cba00 sub_2cba00
    Lifted::new(0x70009b7a10, tpl_363::<0x10> as *const (), 1), // +0x2cba10 sub_2cba10
    Lifted::new(0x70009b7a20, tpl_52::<0x18> as *const (), 1), // +0x2cba20 sub_2cba20
    Lifted::new(0x70009b7a30, tpl_52::<0x18> as *const (), 1), // +0x2cba30 sub_2cba30
    Lifted::new(0x70009b7a40, fn_70009b7a40 as *const (), 0), // +0x2cba40 sub_2cba40
    Lifted::new(0x70009b7a90, tpl_52::<0x2ff8> as *const (), 1), // +0x2cba90 sub_2cba90
    Lifted::new(0x70009b7aa0, tpl_52::<0x3010> as *const (), 1), // +0x2cbaa0 sub_2cbaa0
    Lifted::new(0x70009b7ab0, tpl_52::<0x3000> as *const (), 1), // +0x2cbab0 sub_2cbab0
    Lifted::new(0x70009b7ac0, tpl_52::<0x3008> as *const (), 1), // +0x2cbac0 sub_2cbac0
    Lifted::new(0x70009b7ad0, tpl_52::<0x3018> as *const (), 1), // +0x2cbad0 sub_2cbad0
    Lifted::new(0x70009b7ae0, tpl_52::<0x3020> as *const (), 1), // +0x2cbae0 sub_2cbae0
    Lifted::new(0x70009b7af0, tpl_191::<0x3030> as *const (), 1), // +0x2cbaf0 sub_2cbaf0
    Lifted::new(0x70009b7b10, tpl_52::<0x3030> as *const (), 1), // +0x2cbb10 sub_2cbb10
    Lifted::new(0x70009b7b20, tpl_364::<0x90> as *const (), 1), // +0x2cbb20 sub_2cbb20
    Lifted::new(0x70009b7b40, tpl_36::<0x100> as *const (), 1), // +0x2cbb40 sub_2cbb40
    Lifted::new(0x70009b7b50, tpl_142::<0x1168> as *const (), 1), // +0x2cbb50 sub_2cbb50
    Lifted::new(0x70009b7b70, tpl_142::<0x950> as *const (), 1), // +0x2cbb70 sub_2cbb70
    Lifted::new(0x70009b7b90, tpl_36::<0xd760> as *const (), 1), // +0x2cbb90 sub_2cbb90
    Lifted::new(0x70009b7ba0, fn_70009b7ba0 as *const (), 1), // +0x2cbba0 sub_2cbba0
    Lifted::new(0x70009b7bb0, fn_70009b7bb0 as *const (), 0), // +0x2cbbb0 sub_2cbbb0
    Lifted::new(0x70009b7c50, fn_70009b7c50 as *const (), 1), // +0x2cbc50 sub_2cbc50
    Lifted::new(0x70009b7c60, tpl_39::<0xc> as *const (), 1), // +0x2cbc60 sub_2cbc60
    Lifted::new(0x70009b7c70, tpl_142::<0x958> as *const (), 1), // +0x2cbc70 sub_2cbc70
    Lifted::new(0x70009b7c90, tpl_142::<0x248> as *const (), 1), // +0x2cbc90 sub_2cbc90
    Lifted::new(0x70009b7cb0, tpl_36::<0x38> as *const (), 1), // +0x2cbcb0 sub_2cbcb0
    Lifted::new(0x70009b7cc0, tpl_142::<0x960> as *const (), 1), // +0x2cbcc0 sub_2cbcc0
    Lifted::new(0x70009b7ce0, tpl_142::<0x968> as *const (), 1), // +0x2cbce0 sub_2cbce0
    Lifted::new(0x70009b7d00, fn_70009b7d00 as *const (), 1), // +0x2cbd00 sub_2cbd00
    Lifted::new(0x70009b7d30, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3a7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cbd30 sub_2cbd30
    Lifted::new(0x70009b7e00, tpl_142::<0x1090> as *const (), 1), // +0x2cbe00 sub_2cbe00
    Lifted::new(0x70009b7e20, tpl_142::<0x10a0> as *const (), 1), // +0x2cbe20 sub_2cbe20
    Lifted::new(0x70009b7e40, tpl_142::<0x1098> as *const (), 1), // +0x2cbe40 sub_2cbe40
    Lifted::new(0x70009b7e60, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3ab, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cbe60 sub_2cbe60
    Lifted::new(0x70009b7f30, tpl_142::<0x10a8> as *const (), 1), // +0x2cbf30 sub_2cbf30
    Lifted::new(0x70009b7f50, tpl_142::<0x10b0> as *const (), 1), // +0x2cbf50 sub_2cbf50
    Lifted::new(0x70009b7f70, tpl_142::<0x10b8> as *const (), 1), // +0x2cbf70 sub_2cbf70
    Lifted::new(0x70009b7f90, tpl_142::<0xdf0> as *const (), 1), // +0x2cbf90 sub_2cbf90
    Lifted::new(0x70009b7fb0, tpl_142::<0x10c0> as *const (), 1), // +0x2cbfb0 sub_2cbfb0
    Lifted::new(0x70009b7fd0, tpl_142::<0xc28> as *const (), 1), // +0x2cbfd0 sub_2cbfd0
    Lifted::new(0x70009b7ff0, tpl_142::<0xc30> as *const (), 1), // +0x2cbff0 sub_2cbff0
    Lifted::new(0x70009b8010, tpl_142::<0xc38> as *const (), 1), // +0x2cc010 sub_2cc010
    Lifted::new(0x70009b8030, tpl_142::<0xd20> as *const (), 1), // +0x2cc030 sub_2cc030
    Lifted::new(0x70009b8050, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3af, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc050 sub_2cc050
    Lifted::new(0x70009b8120, tpl_142::<0x10c8> as *const (), 1), // +0x2cc120 sub_2cc120
    Lifted::new(0x70009b8140, tpl_142::<0xc40> as *const (), 1), // +0x2cc140 sub_2cc140
    Lifted::new(0x70009b8160, tpl_52::<0xc7a8> as *const (), 1), // +0x2cc160 sub_2cc160
    Lifted::new(0x70009b8170, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x247, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc170 sub_2cc170
    Lifted::new(0x70009b8240, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x243, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc240 sub_2cc240
    Lifted::new(0x70009b8310, tpl_52::<0xd918> as *const (), 1), // +0x2cc310 sub_2cc310
    Lifted::new(0x70009b8320, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x23f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc320 sub_2cc320
    Lifted::new(0x70009b83f0, tpl_142::<0xff8> as *const (), 1), // +0x2cc3f0 sub_2cc3f0
    Lifted::new(0x70009b8410, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x24b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc410 sub_2cc410
    Lifted::new(0x70009b84e0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc4e0 sub_2cc4e0
    Lifted::new(0x70009b85b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x263, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc5b0 sub_2cc5b0
    Lifted::new(0x70009b8680, tpl_142::<0xdf8> as *const (), 1), // +0x2cc680 sub_2cc680
    Lifted::new(0x70009b86a0, tpl_36::<0xc918> as *const (), 1), // +0x2cc6a0 sub_2cc6a0
    Lifted::new(0x70009b86b0, tpl_142::<0xd28> as *const (), 1), // +0x2cc6b0 sub_2cc6b0
    Lifted::new(0x70009b86d0, tpl_142::<0x1000> as *const (), 1), // +0x2cc6d0 sub_2cc6d0
    Lifted::new(0x70009b86f0, tpl_142::<0xd40> as *const (), 1), // +0x2cc6f0 sub_2cc6f0
    Lifted::new(0x70009b8710, tpl_142::<0xfd0> as *const (), 1), // +0x2cc710 sub_2cc710
    Lifted::new(0x70009b8730, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc730 sub_2cc730
    Lifted::new(0x70009b8800, tpl_142::<0x10d0> as *const (), 1), // +0x2cc800 sub_2cc800
    Lifted::new(0x70009b8820, tpl_142::<0x1010> as *const (), 1), // +0x2cc820 sub_2cc820
    Lifted::new(0x70009b8840, tpl_142::<0x10d8> as *const (), 1), // +0x2cc840 sub_2cc840
    Lifted::new(0x70009b8860, tpl_142::<0x10e0> as *const (), 1), // +0x2cc860 sub_2cc860
    Lifted::new(0x70009b8880, tpl_142::<0x250> as *const (), 1), // +0x2cc880 sub_2cc880
    Lifted::new(0x70009b88a0, fn_70009b88a0 as *const (), 1), // +0x2cc8a0 sub_2cc8a0
    Lifted::new(0x70009b88b0, tpl_142::<0xe00> as *const (), 1), // +0x2cc8b0 sub_2cc8b0
    Lifted::new(0x70009b88d0, tpl_185::<0x10> as *const (), 1), // +0x2cc8d0 sub_2cc8d0
    Lifted::new(0x70009b88e0, fn_70009b88e0 as *const (), 1), // +0x2cc8e0 sub_2cc8e0
    Lifted::new(0x70009b8900, fn_70009b8900 as *const (), 1), // +0x2cc900 sub_2cc900
    Lifted::new(0x70009b8920, tpl_142::<0x978> as *const (), 1), // +0x2cc920 sub_2cc920
    Lifted::new(0x70009b8940, tpl_142::<0xe08> as *const (), 1), // +0x2cc940 sub_2cc940
    Lifted::new(0x70009b8960, tpl_142::<0x980> as *const (), 1), // +0x2cc960 sub_2cc960
    Lifted::new(0x70009b8980, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x24f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cc980 sub_2cc980
    Lifted::new(0x70009b8a50, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x253, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cca50 sub_2cca50
    Lifted::new(0x70009b8b20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x257, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ccb20 sub_2ccb20
    Lifted::new(0x70009b8bf0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x25b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ccbf0 sub_2ccbf0
    Lifted::new(0x70009b8cc0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x25f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cccc0 sub_2cccc0
    Lifted::new(0x70009b8d90, tpl_142::<0x10e8> as *const (), 1), // +0x2ccd90 sub_2ccd90
    Lifted::new(0x70009b8db0, tpl_142::<0x988> as *const (), 1), // +0x2ccdb0 sub_2ccdb0
    Lifted::new(0x70009b8dd0, tpl_142::<0x10f8> as *const (), 1), // +0x2ccdd0 sub_2ccdd0
    Lifted::new(0x70009b8df0, tpl_365::<0x8, 0x10, 0x7000989d80> as *const (), 2), // +0x2ccdf0 sub_2ccdf0
    Lifted::new(0x70009b8e20, tpl_365::<0x8, 0x10, 0x7000989a20> as *const (), 2), // +0x2cce20 sub_2cce20
    Lifted::new(0x70009b8e50, fn_70009b8e50 as *const (), 0), // +0x2cce50 sub_2cce50
    Lifted::new(0x70009b8eb0, fn_70009b8eb0 as *const (), 0), // +0x2cceb0 sub_2cceb0
    Lifted::new(0x70009b8f10, tpl_142::<0x998> as *const (), 1), // +0x2ccf10 sub_2ccf10
    Lifted::new(0x70009b8f30, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x9b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ccf30 sub_2ccf30
    Lifted::new(0x70009b9000, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x97, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd000 sub_2cd000
    Lifted::new(0x70009b90d0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x8b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd0d0 sub_2cd0d0
    Lifted::new(0x70009b91a0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x8f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd1a0 sub_2cd1a0
    Lifted::new(0x70009b9270, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x7f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd270 sub_2cd270
    Lifted::new(0x70009b9340, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x87, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd340 sub_2cd340
    Lifted::new(0x70009b9410, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x77, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd410 sub_2cd410
    Lifted::new(0x70009b94e0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x7b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd4e0 sub_2cd4e0
    Lifted::new(0x70009b95b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x83, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd5b0 sub_2cd5b0
    Lifted::new(0x70009b9680, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x73, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd680 sub_2cd680
    Lifted::new(0x70009b9750, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x93, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd750 sub_2cd750
    Lifted::new(0x70009b9820, tpl_39::<0xc790> as *const (), 1), // +0x2cd820 sub_2cd820
    Lifted::new(0x70009b9830, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3b3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cd830 sub_2cd830
    Lifted::new(0x70009b9900, fn_70009b9900 as *const (), 1), // +0x2cd900 sub_2cd900
    Lifted::new(0x70009b9940, tpl_142::<0x9a8> as *const (), 1), // +0x2cd940 sub_2cd940
    Lifted::new(0x70009b9960, tpl_39::<0x3c> as *const (), 1), // +0x2cd960 sub_2cd960
    Lifted::new(0x70009b9970, tpl_191::<0x8> as *const (), 1), // +0x2cd970 sub_2cd970
    Lifted::new(0x70009b9980, tpl_191::<0x8> as *const (), 1), // +0x2cd980 sub_2cd980
    Lifted::new(0x70009b9990, tpl_191::<0x8> as *const (), 1), // +0x2cd990 sub_2cd990
    Lifted::new(0x70009b99a0, tpl_191::<0x8> as *const (), 1), // +0x2cd9a0 sub_2cd9a0
    Lifted::new(0x70009b99b0, tpl_36::<0xa568> as *const (), 1), // +0x2cd9b0 sub_2cd9b0
    Lifted::new(0x70009b99c0, tpl_275::<0x90, 0xa568> as *const (), 1), // +0x2cd9c0 sub_2cd9c0
    Lifted::new(0x70009b99e0, tpl_142::<0x80> as *const (), 1), // +0x2cd9e0 sub_2cd9e0
    Lifted::new(0x70009b9a00, tpl_187 as *const (), 1), // +0x2cda00 sub_2cda00
    Lifted::new(0x70009b9a10, tpl_142::<0x80> as *const (), 1), // +0x2cda10 sub_2cda10
    Lifted::new(0x70009b9a30, tpl_187 as *const (), 1), // +0x2cda30 sub_2cda30
    Lifted::new(0x70009b9a40, tpl_328::<0x148> as *const (), 1), // +0x2cda40 sub_2cda40
    Lifted::new(0x70009b9a50, tpl_36::<0x10> as *const (), 1), // +0x2cda50 sub_2cda50
    Lifted::new(0x70009b9a60, tpl_36::<0x8> as *const (), 1), // +0x2cda60 sub_2cda60
    Lifted::new(0x70009b9a70, tpl_328::<0x8f> as *const (), 1), // +0x2cda70 sub_2cda70
    Lifted::new(0x70009b9a80, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x267, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cda80 sub_2cda80
    Lifted::new(0x70009b9b50, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3b7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cdb50 sub_2cdb50
    Lifted::new(0x70009b9c20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cdc20 sub_2cdc20
    Lifted::new(0x70009b9cf0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cdcf0 sub_2cdcf0
    Lifted::new(0x70009b9dc0, tpl_142::<0xe10> as *const (), 1), // +0x2cddc0 sub_2cddc0
    Lifted::new(0x70009b9de0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x26b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cdde0 sub_2cdde0
    Lifted::new(0x70009b9eb0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x26f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cdeb0 sub_2cdeb0
    Lifted::new(0x70009b9f80, tpl_52::<0xc760> as *const (), 1), // +0x2cdf80 sub_2cdf80
    Lifted::new(0x70009b9f90, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x273, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cdf90 sub_2cdf90
    Lifted::new(0x70009ba060, tpl_52::<0xbf58> as *const (), 1), // +0x2ce060 sub_2ce060
    Lifted::new(0x70009ba070, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x277, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce070 sub_2ce070
    Lifted::new(0x70009ba140, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x27b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce140 sub_2ce140
    Lifted::new(0x70009ba210, tpl_142::<0x1038> as *const (), 1), // +0x2ce210 sub_2ce210
    Lifted::new(0x70009ba230, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x27f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce230 sub_2ce230
    Lifted::new(0x70009ba300, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x283, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce300 sub_2ce300
    Lifted::new(0x70009ba3d0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x287, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce3d0 sub_2ce3d0
    Lifted::new(0x70009ba4a0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x293, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce4a0 sub_2ce4a0
    Lifted::new(0x70009ba570, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x28b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce570 sub_2ce570
    Lifted::new(0x70009ba640, tpl_142::<0xc48> as *const (), 1), // +0x2ce640 sub_2ce640
    Lifted::new(0x70009ba660, tpl_142::<0xc50> as *const (), 1), // +0x2ce660 sub_2ce660
    Lifted::new(0x70009ba680, tpl_142::<0xc58> as *const (), 1), // +0x2ce680 sub_2ce680
    Lifted::new(0x70009ba6a0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x28f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce6a0 sub_2ce6a0
    Lifted::new(0x70009ba770, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x297, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce770 sub_2ce770
    Lifted::new(0x70009ba840, tpl_142::<0xfd8> as *const (), 1), // +0x2ce840 sub_2ce840
    Lifted::new(0x70009ba860, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2df, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce860 sub_2ce860
    Lifted::new(0x70009ba930, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x29b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ce930 sub_2ce930
    Lifted::new(0x70009baa00, tpl_36::<0xc758> as *const (), 1), // +0x2cea00 sub_2cea00
    Lifted::new(0x70009baa10, tpl_142::<0x9d0> as *const (), 1), // +0x2cea10 sub_2cea10
    Lifted::new(0x70009baa30, tpl_20 as *const (), 1), // +0x2cea30 sub_2cea30
    Lifted::new(0x70009baa40, fn_70009baa40 as *const (), 1), // +0x2cea40 sub_2cea40
    Lifted::new(0x70009baa50, fn_70009baa50 as *const (), 1), // +0x2cea50 sub_2cea50
    Lifted::new(0x70009baa70, tpl_36::<0xc8d0> as *const (), 1), // +0x2cea70 sub_2cea70
    Lifted::new(0x70009baa80, tpl_187 as *const (), 1), // +0x2cea80 sub_2cea80
    Lifted::new(0x70009baa90, tpl_187 as *const (), 1), // +0x2cea90 sub_2cea90
    Lifted::new(0x70009baaa0, tpl_187 as *const (), 1), // +0x2ceaa0 sub_2ceaa0
    Lifted::new(0x70009baab0, tpl_187 as *const (), 1), // +0x2ceab0 sub_2ceab0
    Lifted::new(0x70009baac0, tpl_142::<0xcd8> as *const (), 1), // +0x2ceac0 sub_2ceac0
    Lifted::new(0x70009baae0, fn_70009baae0 as *const (), 1), // +0x2ceae0 sub_2ceae0
    Lifted::new(0x70009bab00, tpl_39::<0x78> as *const (), 1), // +0x2ceb00 sub_2ceb00
    Lifted::new(0x70009bab10, tpl_366 as *const (), 1), // +0x2ceb10 sub_2ceb10
    Lifted::new(0x70009bab20, fn_70009bab20 as *const (), 0), // +0x2ceb20 sub_2ceb20
    Lifted::new(0x70009bab70, fn_70009bab70 as *const (), 0), // +0x2ceb70 sub_2ceb70
    Lifted::new(0x70009babc0, fn_70009babc0 as *const (), 1), // +0x2cebc0 sub_2cebc0
    Lifted::new(0x70009babd0, tpl_142::<0x9d8> as *const (), 1), // +0x2cebd0 sub_2cebd0
    Lifted::new(0x70009babf0, fn_70009babf0 as *const (), 0), // +0x2cebf0 sub_2cebf0
    Lifted::new(0x70009bac20, fn_70009bac20 as *const (), 2), // +0x2cec20 sub_2cec20
    Lifted::new(0x70009bad40, tpl_142::<0x9e0> as *const (), 1), // +0x2ced40 sub_2ced40
    Lifted::new(0x70009bad60, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3f3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ced60 sub_2ced60
    Lifted::new(0x70009bae30, tpl_142::<0x1128> as *const (), 1), // +0x2cee30 sub_2cee30
    Lifted::new(0x70009bae50, tpl_142::<0x9e8> as *const (), 1), // +0x2cee50 sub_2cee50
    Lifted::new(0x70009bae70, tpl_191::<0x68> as *const (), 1), // +0x2cee70 sub_2cee70
    Lifted::new(0x70009bae80, tpl_142::<0x9f0> as *const (), 1), // +0x2cee80 sub_2cee80
    Lifted::new(0x70009baea0, fn_70009baea0 as *const (), 1), // +0x2ceea0 sub_2ceea0
    Lifted::new(0x70009baec0, fn_70009baec0 as *const (), 1), // +0x2ceec0 sub_2ceec0
    Lifted::new(0x70009baee0, fn_70009baee0 as *const (), 1), // +0x2ceee0 sub_2ceee0
    Lifted::new(0x70009baf00, fn_70009baf00 as *const (), 1), // +0x2cef00 sub_2cef00
    Lifted::new(0x70009baf20, tpl_36::<0xc8f8> as *const (), 1), // +0x2cef20 sub_2cef20
    Lifted::new(0x70009baf30, fn_70009baf30 as *const (), 1), // +0x2cef30 sub_2cef30
    Lifted::new(0x70009baf50, tpl_52::<0x80> as *const (), 1), // +0x2cef50 sub_2cef50
    Lifted::new(0x70009baf60, tpl_52::<0x80> as *const (), 1), // +0x2cef60 sub_2cef60
    Lifted::new(0x70009baf70, tpl_36::<0xb0> as *const (), 1), // +0x2cef70 sub_2cef70
    Lifted::new(0x70009baf80, tpl_36::<0x60> as *const (), 1), // +0x2cef80 sub_2cef80
    Lifted::new(0x70009baf90, tpl_36::<0xa618> as *const (), 1), // +0x2cef90 sub_2cef90
    Lifted::new(0x70009bafa0, tpl_52::<0xa738> as *const (), 1), // +0x2cefa0 sub_2cefa0
    Lifted::new(0x70009bafb0, tpl_39::<0x40> as *const (), 1), // +0x2cefb0 sub_2cefb0
    Lifted::new(0x70009bafc0, tpl_39::<0x8> as *const (), 1), // +0x2cefc0 sub_2cefc0
    Lifted::new(0x70009bafd0, tpl_191::<0x3040> as *const (), 1), // +0x2cefd0 sub_2cefd0
    Lifted::new(0x70009baff0, tpl_52::<0x3040> as *const (), 1), // +0x2ceff0 sub_2ceff0
    Lifted::new(0x70009bb000, tpl_142::<0x110> as *const (), 1), // +0x2cf000 sub_2cf000
    Lifted::new(0x70009bb020, tpl_187 as *const (), 1), // +0x2cf020 sub_2cf020
    Lifted::new(0x70009bb030, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x1f7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cf030 sub_2cf030
    Lifted::new(0x70009bb100, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2a3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cf100 sub_2cf100
    Lifted::new(0x70009bb1d0, tpl_142::<0x1a0> as *const (), 1), // +0x2cf1d0 sub_2cf1d0
    Lifted::new(0x70009bb1f0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2a7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cf1f0 sub_2cf1f0
    Lifted::new(0x70009bb2c0, tpl_39::<0x4> as *const (), 1), // +0x2cf2c0 sub_2cf2c0
    Lifted::new(0x70009bb2d0, tpl_142::<0x1110> as *const (), 1), // +0x2cf2d0 sub_2cf2d0
    Lifted::new(0x70009bb2f0, tpl_142::<0xe18> as *const (), 1), // +0x2cf2f0 sub_2cf2f0
    Lifted::new(0x70009bb310, tpl_36::<0x28> as *const (), 1), // +0x2cf310 sub_2cf310
    Lifted::new(0x70009bb320, tpl_142::<0xa10> as *const (), 1), // +0x2cf320 sub_2cf320
    Lifted::new(0x70009bb340, tpl_142::<0xc60> as *const (), 1), // +0x2cf340 sub_2cf340
    Lifted::new(0x70009bb360, tpl_142::<0xce0> as *const (), 1), // +0x2cf360 sub_2cf360
    Lifted::new(0x70009bb380, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2ab, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cf380 sub_2cf380
    Lifted::new(0x70009bb450, tpl_36::<0x8> as *const (), 1), // +0x2cf450 sub_2cf450
    Lifted::new(0x70009bb460, tpl_142::<0x470> as *const (), 1), // +0x2cf460 sub_2cf460
    Lifted::new(0x70009bb480, tpl_142::<0x3b8> as *const (), 1), // +0x2cf480 sub_2cf480
    Lifted::new(0x70009bb4a0, tpl_142::<0xa28> as *const (), 1), // +0x2cf4a0 sub_2cf4a0
    Lifted::new(0x70009bb4c0, tpl_142::<0x1140> as *const (), 1), // +0x2cf4c0 sub_2cf4c0
    Lifted::new(0x70009bb4e0, tpl_142::<0x1138> as *const (), 1), // +0x2cf4e0 sub_2cf4e0
    Lifted::new(0x70009bb500, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2af, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cf500 sub_2cf500
    Lifted::new(0x70009bb5d0, tpl_47::<0xc7e1> as *const (), 1), // +0x2cf5d0 sub_2cf5d0
    Lifted::new(0x70009bb5e0, fn_70009bb5e0 as *const (), 0), // +0x2cf5e0 sub_2cf5e0
    Lifted::new(0x70009bb620, tpl_367::<0x8, 0xffffffffffffffff, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2cf620 sub_2cf620
    Lifted::new(0x70009bb6f0, fn_70009bb6f0 as *const (), 0), // +0x2cf6f0 sub_2cf6f0
    Lifted::new(0x70009bb7e0, tpl_142::<0xa48> as *const (), 1), // +0x2cf7e0 sub_2cf7e0
    Lifted::new(0x70009bb800, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3bb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cf800 sub_2cf800
    Lifted::new(0x70009bb8d0, fn_70009bb8d0 as *const (), 0), // +0x2cf8d0 sub_2cf8d0
    Lifted::new(0x70009bb8e0, tpl_368::<0x28> as *const (), 1), // +0x2cf8e0 sub_2cf8e0
    Lifted::new(0x70009bb8f0, tpl_49::<0xd930> as *const (), 1), // +0x2cf8f0 sub_2cf8f0
    Lifted::new(0x70009bb900, tpl_51::<0xc878> as *const (), 1), // +0x2cf900 sub_2cf900
    Lifted::new(0x70009bb910, tpl_369::<0x78> as *const (), 1), // +0x2cf910 sub_2cf910
    Lifted::new(0x70009bb920, fn_70009bb920 as *const (), 1), // +0x2cf920 sub_2cf920
    Lifted::new(0x70009bb940, fn_70009bb940 as *const (), 1), // +0x2cf940 sub_2cf940
    Lifted::new(0x70009bb960, fn_70009bb960 as *const (), 1), // +0x2cf960 sub_2cf960
    Lifted::new(0x70009bb980, fn_70009bb980 as *const (), 1), // +0x2cf980 sub_2cf980
    Lifted::new(0x70009bb9a0, tpl_51::<0xc890> as *const (), 1), // +0x2cf9a0 sub_2cf9a0
    Lifted::new(0x70009bb9b0, fn_70009bb9b0 as *const (), 1), // +0x2cf9b0 sub_2cf9b0
    Lifted::new(0x70009bb9c0, tpl_51::<0xc8e8> as *const (), 1), // +0x2cf9c0 sub_2cf9c0
    Lifted::new(0x70009bb9d0, tpl_51::<0xd868> as *const (), 1), // +0x2cf9d0 sub_2cf9d0
    Lifted::new(0x70009bb9e0, fn_70009bb9e0 as *const (), 0), // +0x2cf9e0 sub_2cf9e0
    Lifted::new(0x70009bbac0, tpl_370::<0xd910, 0x70007a1040> as *const (), 0), // +0x2cfac0 sub_2cfac0
    Lifted::new(0x70009bbad0, fn_70009bbad0 as *const (), 1), // +0x2cfad0 sub_2cfad0
    Lifted::new(0x70009bbaf0, tpl_371::<0xf3> as *const (), 1), // +0x2cfaf0 sub_2cfaf0
    Lifted::new(0x70009bbb00, tpl_49::<0xd> as *const (), 1), // +0x2cfb00 sub_2cfb00
    Lifted::new(0x70009bbb10, tpl_372::<0x20> as *const (), 1), // +0x2cfb10 sub_2cfb10
    Lifted::new(0x70009bbb20, tpl_245::<0xc92c> as *const (), 1), // +0x2cfb20 sub_2cfb20
    Lifted::new(0x70009bbb30, tpl_373::<0x30> as *const (), 1), // +0x2cfb30 sub_2cfb30
    Lifted::new(0x70009bbb40, fn_70009bbb40 as *const (), 1), // +0x2cfb40 sub_2cfb40
    Lifted::new(0x70009bbb60, tpl_245::<0xc928> as *const (), 1), // +0x2cfb60 sub_2cfb60
    Lifted::new(0x70009bbb70, tpl_245::<0xc7c0> as *const (), 1), // +0x2cfb70 sub_2cfb70
    Lifted::new(0x70009bbb80, tpl_51::<0xc910> as *const (), 1), // +0x2cfb80 sub_2cfb80
    Lifted::new(0x70009bbb90, fn_70009bbb90 as *const (), 1), // +0x2cfb90 sub_2cfb90
    Lifted::new(0x70009bbbb0, fn_70009bbbb0 as *const (), 1), // +0x2cfbb0 sub_2cfbb0
    Lifted::new(0x70009bbbd0, tpl_49::<0xc975> as *const (), 1), // +0x2cfbd0 sub_2cfbd0
    Lifted::new(0x70009bbbe0, tpl_255::<0xc818, 0xc820> as *const (), 1), // +0x2cfbe0 sub_2cfbe0
    Lifted::new(0x70009bbc00, tpl_51::<0xd908> as *const (), 1), // +0x2cfc00 sub_2cfc00
    Lifted::new(0x70009bbc10, tpl_374::<0x48> as *const (), 0), // +0x2cfc10 sub_2cfc10
    Lifted::new(0x70009bbc40, tpl_375::<0x78> as *const (), 1), // +0x2cfc40 sub_2cfc40
    Lifted::new(0x70009bbc50, tpl_51::<0x2fe8> as *const (), 1), // +0x2cfc50 sub_2cfc50
    Lifted::new(0x70009bbc60, fn_70009bbc60 as *const (), 1), // +0x2cfc60 sub_2cfc60
    Lifted::new(0x70009bbc80, tpl_51::<0xa6a8> as *const (), 1), // +0x2cfc80 sub_2cfc80
    Lifted::new(0x70009bbc90, tpl_245::<0x3080> as *const (), 1), // +0x2cfc90 sub_2cfc90
    Lifted::new(0x70009bbca0, tpl_376::<0x9f> as *const (), 1), // +0x2cfca0 sub_2cfca0
    Lifted::new(0x70009bbcb0, tpl_376::<0x97> as *const (), 1), // +0x2cfcb0 sub_2cfcb0
    Lifted::new(0x70009bbcc0, tpl_49::<0xc941> as *const (), 1), // +0x2cfcc0 sub_2cfcc0
    Lifted::new(0x70009bbcd0, tpl_51::<0xc900> as *const (), 1), // +0x2cfcd0 sub_2cfcd0
    Lifted::new(0x70009bbce0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3bf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cfce0 sub_2cfce0
    Lifted::new(0x70009bbdb0, tpl_49::<0xc974> as *const (), 1), // +0x2cfdb0 sub_2cfdb0
    Lifted::new(0x70009bbdc0, tpl_373::<0x38> as *const (), 1), // +0x2cfdc0 sub_2cfdc0
    Lifted::new(0x70009bbdd0, tpl_49::<0xa6b0> as *const (), 1), // +0x2cfdd0 sub_2cfdd0
    Lifted::new(0x70009bbde0, tpl_49::<0xc970> as *const (), 1), // +0x2cfde0 sub_2cfde0
    Lifted::new(0x70009bbdf0, tpl_49::<0xc948> as *const (), 1), // +0x2cfdf0 sub_2cfdf0
    Lifted::new(0x70009bbe00, tpl_376::<0xa7> as *const (), 1), // +0x2cfe00 sub_2cfe00
    Lifted::new(0x70009bbe10, tpl_51::<0xd950> as *const (), 1), // +0x2cfe10 sub_2cfe10
    Lifted::new(0x70009bbe20, tpl_245::<0xc960> as *const (), 1), // +0x2cfe20 sub_2cfe20
    Lifted::new(0x70009bbe30, tpl_245::<0xc95c> as *const (), 1), // +0x2cfe30 sub_2cfe30
    Lifted::new(0x70009bbe40, fn_70009bbe40 as *const (), 1), // +0x2cfe40 sub_2cfe40
    Lifted::new(0x70009bbe50, tpl_51::<0xc870> as *const (), 1), // +0x2cfe50 sub_2cfe50
    Lifted::new(0x70009bbe60, tpl_51::<0xc860> as *const (), 1), // +0x2cfe60 sub_2cfe60
    Lifted::new(0x70009bbe70, tpl_377::<0xc3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2cfe70 sub_2cfe70
    Lifted::new(0x70009bbf30, tpl_51::<0x20> as *const (), 1), // +0x2cff30 sub_2cff30
    Lifted::new(0x70009bbf40, tpl_51::<0x3078> as *const (), 1), // +0x2cff40 sub_2cff40
    Lifted::new(0x70009bbf50, tpl_49::<0x3039> as *const (), 1), // +0x2cff50 sub_2cff50
    Lifted::new(0x70009bbf60, tpl_51::<0xc8f0> as *const (), 1), // +0x2cff60 sub_2cff60
    Lifted::new(0x70009bbf70, tpl_245::<0xc930> as *const (), 1), // +0x2cff70 sub_2cff70
    Lifted::new(0x70009bbf80, tpl_373::<0x20> as *const (), 1), // +0x2cff80 sub_2cff80
    Lifted::new(0x70009bbf90, tpl_49::<0xd76c> as *const (), 1), // +0x2cff90 sub_2cff90
    Lifted::new(0x70009bbfa0, tpl_49::<0xc940> as *const (), 1), // +0x2cffa0 sub_2cffa0
    Lifted::new(0x70009bbfb0, tpl_368::<0x8> as *const (), 1), // +0x2cffb0 sub_2cffb0
    Lifted::new(0x70009bbfc0, tpl_51::<0x58> as *const (), 1), // +0x2cffc0 sub_2cffc0
    Lifted::new(0x70009bbfd0, tpl_368::<0x9> as *const (), 1), // +0x2cffd0 sub_2cffd0
    Lifted::new(0x70009bbfe0, tpl_376::<0x4f> as *const (), 1), // +0x2cffe0 sub_2cffe0
    Lifted::new(0x70009bbff0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3c3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2cfff0 sub_2cfff0
    Lifted::new(0x70009bc0c0, fn_70009bc0c0 as *const (), 1), // +0x2d00c0 sub_2d00c0
    Lifted::new(0x70009bc0e0, tpl_376::<0xaf> as *const (), 1), // +0x2d00e0 sub_2d00e0
    Lifted::new(0x70009bc0f0, tpl_377::<0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d00f0 sub_2d00f0
    Lifted::new(0x70009bc1a0, tpl_376::<0x37> as *const (), 1), // +0x2d01a0 sub_2d01a0
    Lifted::new(0x70009bc1b0, tpl_376::<0x57> as *const (), 1), // +0x2d01b0 sub_2d01b0
    Lifted::new(0x70009bc1c0, tpl_377::<0xd7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d01c0 sub_2d01c0
    Lifted::new(0x70009bc280, tpl_377::<0xf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d0280 sub_2d0280
    Lifted::new(0x70009bc330, tpl_376::<0x47> as *const (), 1), // +0x2d0330 sub_2d0330
    Lifted::new(0x70009bc340, tpl_249::<0x13> as *const (), 1), // +0x2d0340 sub_2d0340
    Lifted::new(0x70009bc350, tpl_376::<0x3f> as *const (), 1), // +0x2d0350 sub_2d0350
    Lifted::new(0x70009bc360, tpl_377::<0xdf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d0360 sub_2d0360
    Lifted::new(0x70009bc420, tpl_372::<0x24> as *const (), 1), // +0x2d0420 sub_2d0420
    Lifted::new(0x70009bc430, tpl_51::<0xc950> as *const (), 1), // +0x2d0430 sub_2d0430
    Lifted::new(0x70009bc440, fn_70009bc440 as *const (), 1), // +0x2d0440 sub_2d0440
    Lifted::new(0x70009bc460, fn_70009bc460 as *const (), 1), // +0x2d0460 sub_2d0460
    Lifted::new(0x70009bc480, fn_70009bc480 as *const (), 1), // +0x2d0480 sub_2d0480
    Lifted::new(0x70009bc4a0, tpl_368::<0x48> as *const (), 1), // +0x2d04a0 sub_2d04a0
    Lifted::new(0x70009bc4b0, fn_70009bc4b0 as *const (), 1), // +0x2d04b0 sub_2d04b0
    Lifted::new(0x70009bc4d0, fn_70009bc4d0 as *const (), 1), // +0x2d04d0 sub_2d04d0
    Lifted::new(0x70009bc4f0, fn_70009bc4f0 as *const (), 1), // +0x2d04f0 sub_2d04f0
    Lifted::new(0x70009bc510, fn_70009bc510 as *const (), 1), // +0x2d0510 sub_2d0510
    Lifted::new(0x70009bc520, fn_70009bc520 as *const (), 1), // +0x2d0520 sub_2d0520
    Lifted::new(0x70009bc530, tpl_376::<0x5f> as *const (), 1), // +0x2d0530 sub_2d0530
    Lifted::new(0x70009bc540, tpl_142::<0x1018> as *const (), 1), // +0x2d0540 sub_2d0540
    Lifted::new(0x70009bc560, tpl_49::<0xc971> as *const (), 1), // +0x2d0560 sub_2d0560
    Lifted::new(0x70009bc570, tpl_49::<0xc973> as *const (), 1), // +0x2d0570 sub_2d0570
    Lifted::new(0x70009bc580, tpl_49::<0xc972> as *const (), 1), // +0x2d0580 sub_2d0580
    Lifted::new(0x70009bc590, fn_70009bc590 as *const (), 0), // +0x2d0590 sub_2d0590
    Lifted::new(0x70009bc640, fn_70009bc640 as *const (), 0), // +0x2d0640 sub_2d0640
    Lifted::new(0x70009bc720, fn_70009bc720 as *const (), 0), // +0x2d0720 sub_2d0720
    Lifted::new(0x70009bc800, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2b7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d0800 sub_2d0800
    Lifted::new(0x70009bc8d0, tpl_372::<0x28> as *const (), 1), // +0x2d08d0 sub_2d08d0
    Lifted::new(0x70009bc8e0, tpl_378::<0x78> as *const (), 1), // +0x2d08e0 sub_2d08e0
    Lifted::new(0x70009bc8f0, tpl_245::<0xc94c> as *const (), 1), // +0x2d08f0 sub_2d08f0
    Lifted::new(0x70009bc900, tpl_49::<0x181> as *const (), 1), // +0x2d0900 sub_2d0900
    Lifted::new(0x70009bc910, tpl_51::<0x70> as *const (), 1), // +0x2d0910 sub_2d0910
    Lifted::new(0x70009bc920, tpl_377::<0xe3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d0920 sub_2d0920
    Lifted::new(0x70009bc9e0, tpl_377::<0xef, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d09e0 sub_2d09e0
    Lifted::new(0x70009bcaa0, tpl_373::<0x18> as *const (), 1), // +0x2d0aa0 sub_2d0aa0
    Lifted::new(0x70009bcab0, tpl_245::<0x3c> as *const (), 1), // +0x2d0ab0 sub_2d0ab0
    Lifted::new(0x70009bcac0, tpl_376::<0x27> as *const (), 1), // +0x2d0ac0 sub_2d0ac0
    Lifted::new(0x70009bcad0, tpl_377::<0xcb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d0ad0 sub_2d0ad0
    Lifted::new(0x70009bcb90, tpl_376::<0x1f> as *const (), 1), // +0x2d0b90 sub_2d0b90
    Lifted::new(0x70009bcba0, tpl_376::<0x17> as *const (), 1), // +0x2d0ba0 sub_2d0ba0
    Lifted::new(0x70009bcbb0, tpl_377::<0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d0bb0 sub_2d0bb0
    Lifted::new(0x70009bcc60, tpl_245::<0x48> as *const (), 1), // +0x2d0c60 sub_2d0c60
    Lifted::new(0x70009bcc70, fn_70009bcc70 as *const (), 1), // +0x2d0c70 sub_2d0c70
    Lifted::new(0x70009bcc90, tpl_51::<0xc888> as *const (), 1), // +0x2d0c90 sub_2d0c90
    Lifted::new(0x70009bcca0, tpl_51::<0xc880> as *const (), 1), // +0x2d0ca0 sub_2d0ca0
    Lifted::new(0x70009bccb0, tpl_377::<0xbf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d0cb0 sub_2d0cb0
    Lifted::new(0x70009bcd70, tpl_377::<0xc7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d0d70 sub_2d0d70
    Lifted::new(0x70009bce30, tpl_376::<0x6f> as *const (), 1), // +0x2d0e30 sub_2d0e30
    Lifted::new(0x70009bce40, tpl_376::<0x77> as *const (), 1), // +0x2d0e40 sub_2d0e40
    Lifted::new(0x70009bce50, tpl_253::<0x7000e970f0> as *const (), 0), // +0x2d0e50 sub_2d0e50
    Lifted::new(0x70009bce70, tpl_253::<0x7000e970f0> as *const (), 0), // +0x2d0e70 sub_2d0e70
    Lifted::new(0x70009bce90, tpl_49::<0xc958> as *const (), 1), // +0x2d0e90 sub_2d0e90
    Lifted::new(0x70009bcea0, tpl_49::<0xc976> as *const (), 1), // +0x2d0ea0 sub_2d0ea0
    Lifted::new(0x70009bceb0, tpl_51::<0xc968> as *const (), 1), // +0x2d0eb0 sub_2d0eb0
    Lifted::new(0x70009bcec0, tpl_51::<0xc938> as *const (), 1), // +0x2d0ec0 sub_2d0ec0
    Lifted::new(0x70009bced0, tpl_376::<0xb7> as *const (), 1), // +0x2d0ed0 sub_2d0ed0
    Lifted::new(0x70009bcee0, tpl_376::<0x7f> as *const (), 1), // +0x2d0ee0 sub_2d0ee0
    Lifted::new(0x70009bcef0, tpl_376::<0x87> as *const (), 1), // +0x2d0ef0 sub_2d0ef0
    Lifted::new(0x70009bcf00, tpl_49::<0xc959> as *const (), 1), // +0x2d0f00 sub_2d0f00
    Lifted::new(0x70009bcf10, tpl_51::<0xc868> as *const (), 1), // +0x2d0f10 sub_2d0f10
    Lifted::new(0x70009bcf20, fn_70009bcf20 as *const (), 1), // +0x2d0f20 sub_2d0f20
    Lifted::new(0x70009bcf40, tpl_245::<0x38> as *const (), 1), // +0x2d0f40 sub_2d0f40
    Lifted::new(0x70009bcf50, fn_70009bcf50 as *const (), 1), // +0x2d0f50 sub_2d0f50
    Lifted::new(0x70009bcf70, tpl_51::<0x2ff8> as *const (), 1), // +0x2d0f70 sub_2d0f70
    Lifted::new(0x70009bcf80, tpl_51::<0x3030> as *const (), 1), // +0x2d0f80 sub_2d0f80
    Lifted::new(0x70009bcf90, tpl_373::<0x40> as *const (), 1), // +0x2d0f90 sub_2d0f90
    Lifted::new(0x70009bcfa0, tpl_51::<0xc918> as *const (), 1), // +0x2d0fa0 sub_2d0fa0
    Lifted::new(0x70009bcfb0, tpl_368::<0x49> as *const (), 1), // +0x2d0fb0 sub_2d0fb0
    Lifted::new(0x70009bcfc0, tpl_51::<0xa568> as *const (), 1), // +0x2d0fc0 sub_2d0fc0
    Lifted::new(0x70009bcfd0, tpl_376::<0x8f> as *const (), 1), // +0x2d0fd0 sub_2d0fd0
    Lifted::new(0x70009bcfe0, tpl_51::<0xc8d0> as *const (), 1), // +0x2d0fe0 sub_2d0fe0
    Lifted::new(0x70009bcff0, fn_70009bcff0 as *const (), 1), // +0x2d0ff0 sub_2d0ff0
    Lifted::new(0x70009bd010, tpl_51::<0x68> as *const (), 1), // +0x2d1010 sub_2d1010
    Lifted::new(0x70009bd020, tpl_51::<0xc8f8> as *const (), 1), // +0x2d1020 sub_2d1020
    Lifted::new(0x70009bd030, tpl_51::<0x3040> as *const (), 1), // +0x2d1030 sub_2d1030
    Lifted::new(0x70009bd040, tpl_372::<0x4> as *const (), 1), // +0x2d1040 sub_2d1040
    Lifted::new(0x70009bd050, tpl_51::<0x28> as *const (), 1), // +0x2d1050 sub_2d1050
    Lifted::new(0x70009bd060, tpl_373::<0x18> as *const (), 1), // +0x2d1060 sub_2d1060
    Lifted::new(0x70009bd070, tpl_51::<0xc8a8> as *const (), 1), // +0x2d1070 sub_2d1070
    Lifted::new(0x70009bd080, tpl_51::<0xc920> as *const (), 1), // +0x2d1080 sub_2d1080
    Lifted::new(0x70009bd090, tpl_373::<0x30> as *const (), 1), // +0x2d1090 sub_2d1090
    Lifted::new(0x70009bd0a0, tpl_51::<0x68> as *const (), 1), // +0x2d10a0 sub_2d10a0
    Lifted::new(0x70009bd0b0, tpl_142::<0xa30> as *const (), 1), // +0x2d10b0 sub_2d10b0
    Lifted::new(0x70009bd0d0, tpl_376::<0x2f> as *const (), 1), // +0x2d10d0 sub_2d10d0
    Lifted::new(0x70009bd0e0, tpl_372::<0x4c> as *const (), 1), // +0x2d10e0 sub_2d10e0
    Lifted::new(0x70009bd0f0, tpl_142::<0xa38> as *const (), 1), // +0x2d10f0 sub_2d10f0
    Lifted::new(0x70009bd110, tpl_51::<0xc8e0> as *const (), 1), // +0x2d1110 sub_2d1110
    Lifted::new(0x70009bd120, tpl_51::<0xc8d8> as *const (), 1), // +0x2d1120 sub_2d1120
    Lifted::new(0x70009bd130, tpl_377::<0xdb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d1130 sub_2d1130
    Lifted::new(0x70009bd1f0, tpl_377::<0xd3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d11f0 sub_2d11f0
    Lifted::new(0x70009bd2b0, tpl_377::<0xe7, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d12b0 sub_2d12b0
    Lifted::new(0x70009bd370, tpl_245::<0x2ff0> as *const (), 1), // +0x2d1370 sub_2d1370
    Lifted::new(0x70009bd380, tpl_51::<0x9758> as *const (), 1), // +0x2d1380 sub_2d1380
    Lifted::new(0x70009bd390, tpl_51::<0xd968> as *const (), 1), // +0x2d1390 sub_2d1390
    Lifted::new(0x70009bd3a0, fn_70009bd3a0 as *const (), 0), // +0x2d13a0 sub_2d13a0
    Lifted::new(0x70009bd3d0, tpl_51::<0xc908> as *const (), 1), // +0x2d13d0 sub_2d13d0
    Lifted::new(0x70009bd3e0, tpl_245::<0xc944> as *const (), 1), // +0x2d13e0 sub_2d13e0
    Lifted::new(0x70009bd3f0, tpl_377::<0xcf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d13f0 sub_2d13f0
    Lifted::new(0x70009bd4b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2b3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d14b0 sub_2d14b0
    Lifted::new(0x70009bd580, tpl_51::<0xc8c8> as *const (), 1), // +0x2d1580 sub_2d1580
    Lifted::new(0x70009bd590, tpl_377::<0xeb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x2d1590 sub_2d1590
    Lifted::new(0x70009bd650, tpl_51::<0xc8a0> as *const (), 1), // +0x2d1650 sub_2d1650
    Lifted::new(0x70009bd660, tpl_51::<0xc8b8> as *const (), 1), // +0x2d1660 sub_2d1660
    Lifted::new(0x70009bd670, tpl_51::<0xc898> as *const (), 1), // +0x2d1670 sub_2d1670
    Lifted::new(0x70009bd680, tpl_51::<0xc8c0> as *const (), 1), // +0x2d1680 sub_2d1680
    Lifted::new(0x70009bd690, tpl_51::<0xc8b0> as *const (), 1), // +0x2d1690 sub_2d1690
    Lifted::new(0x70009bd6a0, tpl_20 as *const (), 1), // +0x2d16a0 sub_2d16a0
    Lifted::new(0x70009bd6b0, tpl_191::<0x18> as *const (), 1), // +0x2d16b0 sub_2d16b0
    Lifted::new(0x70009bd6c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2bb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d16c0 sub_2d16c0
    Lifted::new(0x70009bd790, tpl_52::<0xa678> as *const (), 1), // +0x2d1790 sub_2d1790
    Lifted::new(0x70009bd7a0, tpl_142::<0x118> as *const (), 1), // +0x2d17a0 sub_2d17a0
    Lifted::new(0x70009bd7c0, tpl_36::<0xd9c8> as *const (), 1), // +0x2d17c0 sub_2d17c0
    Lifted::new(0x70009bd7d0, tpl_305::<0xa8, 0x8, 0xa0> as *const (), 1), // +0x2d17d0 sub_2d17d0
    Lifted::new(0x70009bd810, tpl_142::<0x1178> as *const (), 1), // +0x2d1810 sub_2d1810
    Lifted::new(0x70009bd830, tpl_36::<0xc8a8> as *const (), 1), // +0x2d1830 sub_2d1830
    Lifted::new(0x70009bd840, tpl_142::<0xa18> as *const (), 1), // +0x2d1840 sub_2d1840
    Lifted::new(0x70009bd860, fn_70009bd860 as *const (), 0), // +0x2d1860 sub_2d1860
    Lifted::new(0x70009bd890, fn_70009bd890 as *const (), 0), // +0x2d1890 sub_2d1890
    Lifted::new(0x70009bd8c0, tpl_142::<0x258> as *const (), 1), // +0x2d18c0 sub_2d18c0
    Lifted::new(0x70009bd8e0, tpl_142::<0x260> as *const (), 1), // +0x2d18e0 sub_2d18e0
    Lifted::new(0x70009bd900, tpl_306::<0x68> as *const (), 1), // +0x2d1900 sub_2d1900
    Lifted::new(0x70009bd910, tpl_142::<0x270> as *const (), 1), // +0x2d1910 sub_2d1910
    Lifted::new(0x70009bd930, tpl_142::<0x1028> as *const (), 1), // +0x2d1930 sub_2d1930
    Lifted::new(0x70009bd950, tpl_316::<0x8> as *const (), 1), // +0x2d1950 sub_2d1950
    Lifted::new(0x70009bd960, tpl_317::<0x8> as *const (), 1), // +0x2d1960 sub_2d1960
    Lifted::new(0x70009bd970, tpl_36::<0x10> as *const (), 1), // +0x2d1970 sub_2d1970
    Lifted::new(0x70009bd980, tpl_39::<0x8> as *const (), 1), // +0x2d1980 sub_2d1980
    Lifted::new(0x70009bd990, tpl_39::<0x1c> as *const (), 1), // +0x2d1990 sub_2d1990
    Lifted::new(0x70009bd9a0, tpl_36::<0x8> as *const (), 1), // +0x2d19a0 sub_2d19a0
    Lifted::new(0x70009bd9b0, tpl_36::<0x18> as *const (), 1), // +0x2d19b0 sub_2d19b0
    Lifted::new(0x70009bd9c0, tpl_142::<0x318> as *const (), 1), // +0x2d19c0 sub_2d19c0
    Lifted::new(0x70009bd9e0, tpl_142::<0x310> as *const (), 1), // +0x2d19e0 sub_2d19e0
    Lifted::new(0x70009bda00, tpl_142::<0xe50> as *const (), 1), // +0x2d1a00 sub_2d1a00
    Lifted::new(0x70009bda20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2bf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d1a20 sub_2d1a20
    Lifted::new(0x70009bdaf0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2e3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d1af0 sub_2d1af0
    Lifted::new(0x70009bdbc0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2e7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d1bc0 sub_2d1bc0
    Lifted::new(0x70009bdc90, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2ef, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d1c90 sub_2d1c90
    Lifted::new(0x70009bdd60, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2eb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d1d60 sub_2d1d60
    Lifted::new(0x70009bde30, tpl_185::<0x10> as *const (), 1), // +0x2d1e30 sub_2d1e30
    Lifted::new(0x70009bde40, tpl_379::<0x10, 0x8> as *const (), 1), // +0x2d1e40 sub_2d1e40
    Lifted::new(0x70009bde50, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2c3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d1e50 sub_2d1e50
    Lifted::new(0x70009bdf20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2cb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d1f20 sub_2d1f20
    Lifted::new(0x70009bdff0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2cf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d1ff0 sub_2d1ff0
    Lifted::new(0x70009be0c0, fn_70009be0c0 as *const (), 0), // +0x2d20c0 sub_2d20c0
    Lifted::new(0x70009be1a0, tpl_142::<0x278> as *const (), 1), // +0x2d21a0 sub_2d21a0
    Lifted::new(0x70009be1c0, tpl_142::<0x280> as *const (), 1), // +0x2d21c0 sub_2d21c0
    Lifted::new(0x70009be1e0, tpl_142::<0x288> as *const (), 1), // +0x2d21e0 sub_2d21e0
    Lifted::new(0x70009be200, tpl_142::<0xe90> as *const (), 1), // +0x2d2200 sub_2d2200
    Lifted::new(0x70009be220, tpl_142::<0xe88> as *const (), 1), // +0x2d2220 sub_2d2220
    Lifted::new(0x70009be240, fn_70009be240 as *const (), 1), // +0x2d2240 sub_2d2240
    Lifted::new(0x70009be260, tpl_36::<0xc920> as *const (), 1), // +0x2d2260 sub_2d2260
    Lifted::new(0x70009be270, tpl_144 as *const (), 1), // +0x2d2270 sub_2d2270
    Lifted::new(0x70009be280, tpl_44 as *const (), 1), // +0x2d2280 sub_2d2280
    Lifted::new(0x70009be290, tpl_142::<0xed0> as *const (), 1), // +0x2d2290 sub_2d2290
    Lifted::new(0x70009be2b0, tpl_142::<0xa60> as *const (), 1), // +0x2d22b0 sub_2d22b0
    Lifted::new(0x70009be2d0, tpl_191::<0x28> as *const (), 1), // +0x2d22d0 sub_2d22d0
    Lifted::new(0x70009be2e0, fn_70009be2e0 as *const (), 1), // +0x2d22e0 sub_2d22e0
    Lifted::new(0x70009be310, tpl_36::<0x68> as *const (), 1), // +0x2d2310 sub_2d2310
    Lifted::new(0x70009be320, tpl_142::<0xa58> as *const (), 1), // +0x2d2320 sub_2d2320
    Lifted::new(0x70009be340, tpl_142::<0x290> as *const (), 1), // +0x2d2340 sub_2d2340
    Lifted::new(0x70009be360, tpl_142::<0xce8> as *const (), 1), // +0x2d2360 sub_2d2360
    Lifted::new(0x70009be380, tpl_142::<0xcf0> as *const (), 1), // +0x2d2380 sub_2d2380
    Lifted::new(0x70009be3a0, tpl_142::<0xa70> as *const (), 1), // +0x2d23a0 sub_2d23a0
    Lifted::new(0x70009be3c0, tpl_142::<0xe28> as *const (), 1), // +0x2d23c0 sub_2d23c0
    Lifted::new(0x70009be3e0, tpl_52::<0x48> as *const (), 1), // +0x2d23e0 sub_2d23e0
    Lifted::new(0x70009be3f0, tpl_36::<0xa8> as *const (), 1), // +0x2d23f0 sub_2d23f0
    Lifted::new(0x70009be400, tpl_36::<0x30> as *const (), 1), // +0x2d2400 sub_2d2400
    Lifted::new(0x70009be410, tpl_328::<0x2f> as *const (), 1), // +0x2d2410 sub_2d2410
    Lifted::new(0x70009be420, tpl_39::<0x38> as *const (), 1), // +0x2d2420 sub_2d2420
    Lifted::new(0x70009be430, tpl_142::<0xa68> as *const (), 1), // +0x2d2430 sub_2d2430
    Lifted::new(0x70009be450, tpl_142::<0xc68> as *const (), 1), // +0x2d2450 sub_2d2450
    Lifted::new(0x70009be470, tpl_142::<0x198> as *const (), 1), // +0x2d2470 sub_2d2470
    Lifted::new(0x70009be490, tpl_142::<0x3b0> as *const (), 1), // +0x2d2490 sub_2d2490
    Lifted::new(0x70009be4b0, tpl_36::<0x18> as *const (), 1), // +0x2d24b0 sub_2d24b0
    Lifted::new(0x70009be4c0, tpl_30 as *const (), 1), // +0x2d24c0 sub_2d24c0
    Lifted::new(0x70009be4d0, tpl_39::<0x8> as *const (), 1), // +0x2d24d0 sub_2d24d0
    Lifted::new(0x70009be4e0, tpl_52::<0xd828> as *const (), 1), // +0x2d24e0 sub_2d24e0
    Lifted::new(0x70009be4f0, tpl_39::<0x8> as *const (), 1), // +0x2d24f0 sub_2d24f0
    Lifted::new(0x70009be500, tpl_47::<0x58> as *const (), 1), // +0x2d2500 sub_2d2500
    Lifted::new(0x70009be510, tpl_39::<0x4> as *const (), 1), // +0x2d2510 sub_2d2510
    Lifted::new(0x70009be520, tpl_36::<0x10> as *const (), 1), // +0x2d2520 sub_2d2520
    Lifted::new(0x70009be530, tpl_39::<0xc> as *const (), 1), // +0x2d2530 sub_2d2530
    Lifted::new(0x70009be540, tpl_39::<0x10> as *const (), 1), // +0x2d2540 sub_2d2540
    Lifted::new(0x70009be550, tpl_142::<0xa78> as *const (), 1), // +0x2d2550 sub_2d2550
    Lifted::new(0x70009be570, tpl_142::<0xf00> as *const (), 1), // +0x2d2570 sub_2d2570
    Lifted::new(0x70009be590, tpl_142::<0xf08> as *const (), 1), // +0x2d2590 sub_2d2590
    Lifted::new(0x70009be5b0, tpl_142::<0xf10> as *const (), 1), // +0x2d25b0 sub_2d25b0
    Lifted::new(0x70009be5d0, tpl_142::<0xf18> as *const (), 1), // +0x2d25d0 sub_2d25d0
    Lifted::new(0x70009be5f0, tpl_36::<0xa6a0> as *const (), 1), // +0x2d25f0 sub_2d25f0
    Lifted::new(0x70009be600, tpl_52::<0xd768> as *const (), 1), // +0x2d2600 sub_2d2600
    Lifted::new(0x70009be610, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2c7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d2610 sub_2d2610
    Lifted::new(0x70009be6e0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2f3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d26e0 sub_2d26e0
    Lifted::new(0x70009be7b0, tpl_142::<0xc70> as *const (), 1), // +0x2d27b0 sub_2d27b0
    Lifted::new(0x70009be7d0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2f7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d27d0 sub_2d27d0
    Lifted::new(0x70009be8a0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2fb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d28a0 sub_2d28a0
    Lifted::new(0x70009be970, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2ff, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d2970 sub_2d2970
    Lifted::new(0x70009bea40, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x323, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d2a40 sub_2d2a40
    Lifted::new(0x70009beb10, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x327, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d2b10 sub_2d2b10
    Lifted::new(0x70009bebe0, tpl_142::<0x1020> as *const (), 1), // +0x2d2be0 sub_2d2be0
    Lifted::new(0x70009bec00, tpl_142::<0xf70> as *const (), 1), // +0x2d2c00 sub_2d2c00
    Lifted::new(0x70009bec20, tpl_142::<0xfe0> as *const (), 1), // +0x2d2c20 sub_2d2c20
    Lifted::new(0x70009bec40, tpl_142::<0x2f0> as *const (), 1), // +0x2d2c40 sub_2d2c40
    Lifted::new(0x70009bec60, tpl_142::<0x1030> as *const (), 1), // +0x2d2c60 sub_2d2c60
    Lifted::new(0x70009bec80, tpl_191::<0xc8e0> as *const (), 1), // +0x2d2c80 sub_2d2c80
    Lifted::new(0x70009beca0, tpl_36::<0xc8d8> as *const (), 1), // +0x2d2ca0 sub_2d2ca0
    Lifted::new(0x70009becb0, tpl_142::<0xa88> as *const (), 1), // +0x2d2cb0 sub_2d2cb0
    Lifted::new(0x70009becd0, tpl_36::<0xa580> as *const (), 1), // +0x2d2cd0 sub_2d2cd0
    Lifted::new(0x70009bece0, tpl_275::<0x90, 0xa580> as *const (), 1), // +0x2d2ce0 sub_2d2ce0
    Lifted::new(0x70009bed00, tpl_142::<0xa90> as *const (), 1), // +0x2d2d00 sub_2d2d00
    Lifted::new(0x70009bed20, tpl_142::<0xe60> as *const (), 1), // +0x2d2d20 sub_2d2d20
    Lifted::new(0x70009bed40, tpl_306::<0x68> as *const (), 1), // +0x2d2d40 sub_2d2d40
    Lifted::new(0x70009bed50, tpl_52::<0xd364> as *const (), 1), // +0x2d2d50 sub_2d2d50
    Lifted::new(0x70009bed60, tpl_144 as *const (), 1), // +0x2d2d60 sub_2d2d60
    Lifted::new(0x70009bed70, tpl_52::<0x70> as *const (), 1), // +0x2d2d70 sub_2d2d70
    Lifted::new(0x70009bed80, tpl_380::<0x8, 0x10> as *const (), 1), // +0x2d2d80 sub_2d2d80
    Lifted::new(0x70009bedd0, tpl_380::<0x8, 0x10> as *const (), 1), // +0x2d2dd0 sub_2d2dd0
    Lifted::new(0x70009bee20, tpl_142::<0x298> as *const (), 1), // +0x2d2e20 sub_2d2e20
    Lifted::new(0x70009bee40, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x32b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d2e40 sub_2d2e40
    Lifted::new(0x70009bef10, tpl_142::<0xf8> as *const (), 1), // +0x2d2f10 sub_2d2f10
    Lifted::new(0x70009bef30, tpl_142::<0xa98> as *const (), 1), // +0x2d2f30 sub_2d2f30
    Lifted::new(0x70009bef50, tpl_142::<0xaa8> as *const (), 1), // +0x2d2f50 sub_2d2f50
    Lifted::new(0x70009bef70, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3c7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d2f70 sub_2d2f70
    Lifted::new(0x70009bf040, tpl_142::<0x2a0> as *const (), 1), // +0x2d3040 sub_2d3040
    Lifted::new(0x70009bf060, tpl_333::<0xdb> as *const (), 1), // +0x2d3060 sub_2d3060
    Lifted::new(0x70009bf080, tpl_334::<0xdb> as *const (), 1), // +0x2d3080 sub_2d3080
    Lifted::new(0x70009bf0b0, tpl_333::<0xd3> as *const (), 1), // +0x2d30b0 sub_2d30b0
    Lifted::new(0x70009bf0d0, tpl_334::<0xd3> as *const (), 1), // +0x2d30d0 sub_2d30d0
    Lifted::new(0x70009bf100, tpl_333::<0xe7> as *const (), 1), // +0x2d3100 sub_2d3100
    Lifted::new(0x70009bf120, tpl_334::<0xe7> as *const (), 1), // +0x2d3120 sub_2d3120
    Lifted::new(0x70009bf150, tpl_142::<0xab8> as *const (), 1), // +0x2d3150 sub_2d3150
    Lifted::new(0x70009bf170, tpl_142::<0xe30> as *const (), 1), // +0x2d3170 sub_2d3170
    Lifted::new(0x70009bf190, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x34f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d3190 sub_2d3190
    Lifted::new(0x70009bf260, tpl_142::<0x188> as *const (), 1), // +0x2d3260 sub_2d3260
    Lifted::new(0x70009bf280, tpl_142::<0x3a0> as *const (), 1), // +0x2d3280 sub_2d3280
    Lifted::new(0x70009bf2a0, tpl_142::<0x370> as *const (), 1), // +0x2d32a0 sub_2d32a0
    Lifted::new(0x70009bf2c0, tpl_142::<0xa8> as *const (), 1), // +0x2d32c0 sub_2d32c0
    Lifted::new(0x70009bf2e0, tpl_142::<0xac0> as *const (), 1), // +0x2d32e0 sub_2d32e0
    Lifted::new(0x70009bf300, tpl_142::<0x150> as *const (), 1), // +0x2d3300 sub_2d3300
    Lifted::new(0x70009bf320, tpl_142::<0x308> as *const (), 1), // +0x2d3320 sub_2d3320
    Lifted::new(0x70009bf340, tpl_142::<0xac8> as *const (), 1), // +0x2d3340 sub_2d3340
    Lifted::new(0x70009bf360, tpl_142::<0xad0> as *const (), 1), // +0x2d3360 sub_2d3360
    Lifted::new(0x70009bf380, tpl_185::<0x2ff0> as *const (), 1), // +0x2d3380 sub_2d3380
    Lifted::new(0x70009bf390, tpl_185::<0xa0> as *const (), 1), // +0x2d3390 sub_2d3390
    Lifted::new(0x70009bf3a0, tpl_52::<0x3090> as *const (), 1), // +0x2d33a0 sub_2d33a0
    Lifted::new(0x70009bf3b0, fn_70009bf3b0 as *const (), 1), // +0x2d33b0 sub_2d33b0
    Lifted::new(0x70009bf3c0, tpl_52::<0x2fd8> as *const (), 1), // +0x2d33c0 sub_2d33c0
    Lifted::new(0x70009bf3d0, tpl_52::<0x2fd8> as *const (), 1), // +0x2d33d0 sub_2d33d0
    Lifted::new(0x70009bf3e0, tpl_36::<0xa728> as *const (), 1), // +0x2d33e0 sub_2d33e0
    Lifted::new(0x70009bf3f0, tpl_142::<0xad8> as *const (), 1), // +0x2d33f0 sub_2d33f0
    Lifted::new(0x70009bf410, fn_70009bf410 as *const (), 1), // +0x2d3410 sub_2d3410
    Lifted::new(0x70009bf420, tpl_36::<0x1a8> as *const (), 1), // +0x2d3420 sub_2d3420
    Lifted::new(0x70009bf430, fn_70009bf430 as *const (), 2), // +0x2d3430 sub_2d3430
    Lifted::new(0x70009bf460, tpl_142::<0xae0> as *const (), 1), // +0x2d3460 sub_2d3460
    Lifted::new(0x70009bf480, tpl_142::<0xae8> as *const (), 1), // +0x2d3480 sub_2d3480
    Lifted::new(0x70009bf4a0, tpl_142::<0xaf0> as *const (), 1), // +0x2d34a0 sub_2d34a0
    Lifted::new(0x70009bf4c0, tpl_142::<0xcf8> as *const (), 1), // +0x2d34c0 sub_2d34c0
    Lifted::new(0x70009bf4e0, tpl_52::<0x130> as *const (), 1), // +0x2d34e0 sub_2d34e0
    Lifted::new(0x70009bf4f0, tpl_142::<0xd18> as *const (), 1), // +0x2d34f0 sub_2d34f0
    Lifted::new(0x70009bf510, tpl_36::<0xd968> as *const (), 1), // +0x2d3510 sub_2d3510
    Lifted::new(0x70009bf520, tpl_36::<0xd948> as *const (), 1), // +0x2d3520 sub_2d3520
    Lifted::new(0x70009bf530, tpl_142::<0xb00> as *const (), 1), // +0x2d3530 sub_2d3530
    Lifted::new(0x70009bf550, tpl_142::<0x490> as *const (), 1), // +0x2d3550 sub_2d3550
    Lifted::new(0x70009bf570, tpl_142::<0x488> as *const (), 1), // +0x2d3570 sub_2d3570
    Lifted::new(0x70009bf590, tpl_142::<0x148> as *const (), 1), // +0x2d3590 sub_2d3590
    Lifted::new(0x70009bf5b0, tpl_191::<0x28> as *const (), 1), // +0x2d35b0 sub_2d35b0
    Lifted::new(0x70009bf5c0, tpl_191::<0x28> as *const (), 1), // +0x2d35c0 sub_2d35c0
    Lifted::new(0x70009bf5d0, tpl_191::<0x18> as *const (), 1), // +0x2d35d0 sub_2d35d0
    Lifted::new(0x70009bf5e0, tpl_381::<0x50, 0x70017d36e8, 0x70017a0290, 0x7001d54ef8, 0x70009bf5ff> as *const (), 0), // +0x2d35e0 sub_2d35e0
    Lifted::new(0x70009bf610, tpl_142::<0xaf8> as *const (), 1), // +0x2d3610 sub_2d3610
    Lifted::new(0x70009bf630, tpl_142::<0xb8> as *const (), 1), // +0x2d3630 sub_2d3630
    Lifted::new(0x70009bf650, tpl_36::<0x2fd8> as *const (), 1), // +0x2d3650 sub_2d3650
    Lifted::new(0x70009bf660, tpl_142::<0xe38> as *const (), 1), // +0x2d3660 sub_2d3660
    Lifted::new(0x70009bf680, tpl_36::<0xc908> as *const (), 1), // +0x2d3680 sub_2d3680
    Lifted::new(0x70009bf690, tpl_36::<0x198> as *const (), 1), // +0x2d3690 sub_2d3690
    Lifted::new(0x70009bf6a0, tpl_142::<0x90> as *const (), 1), // +0x2d36a0 sub_2d36a0
    Lifted::new(0x70009bf6c0, tpl_39::<0xc> as *const (), 1), // +0x2d36c0 sub_2d36c0
    Lifted::new(0x70009bf6d0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3cb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d36d0 sub_2d36d0
    Lifted::new(0x70009bf7a0, tpl_39::<0xc944> as *const (), 1), // +0x2d37a0 sub_2d37a0
    Lifted::new(0x70009bf7b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x353, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d37b0 sub_2d37b0
    Lifted::new(0x70009bf880, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x357, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d3880 sub_2d3880
    Lifted::new(0x70009bf950, tpl_142::<0xfc8> as *const (), 1), // +0x2d3950 sub_2d3950
    Lifted::new(0x70009bf970, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x4f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d3970 sub_2d3970
    Lifted::new(0x70009bfa40, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x57, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d3a40 sub_2d3a40
    Lifted::new(0x70009bfb10, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x47, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d3b10 sub_2d3b10
    Lifted::new(0x70009bfbe0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x67, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d3be0 sub_2d3be0
    Lifted::new(0x70009bfcb0, tpl_142::<0x350> as *const (), 1), // +0x2d3cb0 sub_2d3cb0
    Lifted::new(0x70009bfcd0, tpl_142::<0x358> as *const (), 1), // +0x2d3cd0 sub_2d3cd0
    Lifted::new(0x70009bfcf0, tpl_142::<0x360> as *const (), 1), // +0x2d3cf0 sub_2d3cf0
    Lifted::new(0x70009bfd10, tpl_142::<0x330> as *const (), 1), // +0x2d3d10 sub_2d3d10
    Lifted::new(0x70009bfd30, tpl_14 as *const (), 1), // +0x2d3d30 sub_2d3d30
    Lifted::new(0x70009bfd40, tpl_14 as *const (), 1), // +0x2d3d40 sub_2d3d40
    Lifted::new(0x70009bfd50, tpl_14 as *const (), 1), // +0x2d3d50 sub_2d3d50
    Lifted::new(0x70009bfd60, tpl_14 as *const (), 1), // +0x2d3d60 sub_2d3d60
    Lifted::new(0x70009bfd70, tpl_14 as *const (), 1), // +0x2d3d70 sub_2d3d70
    Lifted::new(0x70009bfd80, tpl_14 as *const (), 1), // +0x2d3d80 sub_2d3d80
    Lifted::new(0x70009bfd90, tpl_14 as *const (), 1), // +0x2d3d90 sub_2d3d90
    Lifted::new(0x70009bfda0, tpl_14 as *const (), 1), // +0x2d3da0 sub_2d3da0
    Lifted::new(0x70009bfdb0, tpl_14 as *const (), 1), // +0x2d3db0 sub_2d3db0
    Lifted::new(0x70009bfdc0, tpl_14 as *const (), 1), // +0x2d3dc0 sub_2d3dc0
    Lifted::new(0x70009bfdd0, tpl_14 as *const (), 1), // +0x2d3dd0 sub_2d3dd0
    Lifted::new(0x70009bfde0, tpl_14 as *const (), 1), // +0x2d3de0 sub_2d3de0
    Lifted::new(0x70009bfdf0, tpl_142::<0xe70> as *const (), 1), // +0x2d3df0 sub_2d3df0
    Lifted::new(0x70009bfe10, tpl_142::<0xe68> as *const (), 1), // +0x2d3e10 sub_2d3e10
    Lifted::new(0x70009bfe30, tpl_142::<0x368> as *const (), 1), // +0x2d3e30 sub_2d3e30
    Lifted::new(0x70009bfe50, tpl_142::<0xb30> as *const (), 1), // +0x2d3e50 sub_2d3e50
    Lifted::new(0x70009bfe70, tpl_142::<0xb38> as *const (), 1), // +0x2d3e70 sub_2d3e70
    Lifted::new(0x70009bfe90, tpl_142::<0xa0> as *const (), 1), // +0x2d3e90 sub_2d3e90
    Lifted::new(0x70009bfeb0, tpl_36::<0xa700> as *const (), 1), // +0x2d3eb0 sub_2d3eb0
    Lifted::new(0x70009bfec0, tpl_142::<0xb40> as *const (), 1), // +0x2d3ec0 sub_2d3ec0
    Lifted::new(0x70009bfee0, tpl_142::<0x388> as *const (), 1), // +0x2d3ee0 sub_2d3ee0
    Lifted::new(0x70009bff00, tpl_142::<0xca8> as *const (), 1), // +0x2d3f00 sub_2d3f00
    Lifted::new(0x70009bff20, tpl_142::<0x98> as *const (), 1), // +0x2d3f20 sub_2d3f20
    Lifted::new(0x70009bff40, tpl_39::<0x4c> as *const (), 1), // +0x2d3f40 sub_2d3f40
    Lifted::new(0x70009bff50, tpl_142::<0xd00> as *const (), 1), // +0x2d3f50 sub_2d3f50
    Lifted::new(0x70009bff70, tpl_333::<0xcf> as *const (), 1), // +0x2d3f70 sub_2d3f70
    Lifted::new(0x70009bff90, tpl_334::<0xcf> as *const (), 1), // +0x2d3f90 sub_2d3f90
    Lifted::new(0x70009bffc0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3cf, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d3fc0 sub_2d3fc0
    Lifted::new(0x70009c0090, tpl_39::<0x8> as *const (), 1), // +0x2d4090 sub_2d4090
    Lifted::new(0x70009c00a0, tpl_142::<0xb58> as *const (), 1), // +0x2d40a0 sub_2d40a0
    Lifted::new(0x70009c00c0, tpl_357::<0x70017d2c10, 0x7001d54ef8, 0x70009c00dc, 0x4> as *const (), 0), // +0x2d40c0 sub_2d40c0
    Lifted::new(0x70009c0100, tpl_142::<0xb50> as *const (), 1), // +0x2d4100 sub_2d4100
    Lifted::new(0x70009c0120, fn_70009c0120 as *const (), 1), // +0x2d4120 sub_2d4120
    Lifted::new(0x70009c0150, tpl_187 as *const (), 1), // +0x2d4150 sub_2d4150
    Lifted::new(0x70009c0160, tpl_142::<0x2a8> as *const (), 1), // +0x2d4160 sub_2d4160
    Lifted::new(0x70009c0180, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3d3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4180 sub_2d4180
    Lifted::new(0x70009c0250, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x2db, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4250 sub_2d4250
    Lifted::new(0x70009c0320, tpl_142::<0xc98> as *const (), 1), // +0x2d4320 sub_2d4320
    Lifted::new(0x70009c0340, tpl_142::<0xca0> as *const (), 1), // +0x2d4340 sub_2d4340
    Lifted::new(0x70009c0360, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x337, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4360 sub_2d4360
    Lifted::new(0x70009c0430, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3df, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4430 sub_2d4430
    Lifted::new(0x70009c0500, tpl_142::<0xe40> as *const (), 1), // +0x2d4500 sub_2d4500
    Lifted::new(0x70009c0520, tpl_142::<0xc78> as *const (), 1), // +0x2d4520 sub_2d4520
    Lifted::new(0x70009c0540, tpl_142::<0xc80> as *const (), 1), // +0x2d4540 sub_2d4540
    Lifted::new(0x70009c0560, tpl_36::<0xc8c8> as *const (), 1), // +0x2d4560 sub_2d4560
    Lifted::new(0x70009c0570, tpl_142::<0x2b0> as *const (), 1), // +0x2d4570 sub_2d4570
    Lifted::new(0x70009c0590, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x32f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4590 sub_2d4590
    Lifted::new(0x70009c0660, tpl_333::<0xeb> as *const (), 1), // +0x2d4660 sub_2d4660
    Lifted::new(0x70009c0680, tpl_334::<0xeb> as *const (), 1), // +0x2d4680 sub_2d4680
    Lifted::new(0x70009c06b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x33b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d46b0 sub_2d46b0
    Lifted::new(0x70009c0780, tpl_142::<0xe48> as *const (), 1), // +0x2d4780 sub_2d4780
    Lifted::new(0x70009c07a0, tpl_36::<0xc8a0> as *const (), 1), // +0x2d47a0 sub_2d47a0
    Lifted::new(0x70009c07b0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x33f, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d47b0 sub_2d47b0
    Lifted::new(0x70009c0880, tpl_142::<0x2b8> as *const (), 1), // +0x2d4880 sub_2d4880
    Lifted::new(0x70009c08a0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3d7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d48a0 sub_2d48a0
    Lifted::new(0x70009c0970, tpl_36::<0xc8b8> as *const (), 1), // +0x2d4970 sub_2d4970
    Lifted::new(0x70009c0980, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x343, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4980 sub_2d4980
    Lifted::new(0x70009c0a50, tpl_36::<0xc898> as *const (), 1), // +0x2d4a50 sub_2d4a50
    Lifted::new(0x70009c0a60, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x347, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4a60 sub_2d4a60
    Lifted::new(0x70009c0b30, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3db, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4b30 sub_2d4b30
    Lifted::new(0x70009c0c00, tpl_36::<0xc8c0> as *const (), 1), // +0x2d4c00 sub_2d4c00
    Lifted::new(0x70009c0c10, tpl_36::<0xc8b0> as *const (), 1), // +0x2d4c10 sub_2d4c10
    Lifted::new(0x70009c0c20, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x34b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4c20 sub_2d4c20
    Lifted::new(0x70009c0cf0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x333, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4cf0 sub_2d4cf0
    Lifted::new(0x70009c0dc0, tpl_142::<0x2c0> as *const (), 1), // +0x2d4dc0 sub_2d4dc0
    Lifted::new(0x70009c0de0, tpl_142::<0xc88> as *const (), 1), // +0x2d4de0 sub_2d4de0
    Lifted::new(0x70009c0e00, tpl_142::<0xc90> as *const (), 1), // +0x2d4e00 sub_2d4e00
    Lifted::new(0x70009c0e20, tpl_142::<0x2d0> as *const (), 1), // +0x2d4e20 sub_2d4e20
    Lifted::new(0x70009c0e40, tpl_142::<0x2e8> as *const (), 1), // +0x2d4e40 sub_2d4e40
    Lifted::new(0x70009c0e60, tpl_142::<0x2c8> as *const (), 1), // +0x2d4e60 sub_2d4e60
    Lifted::new(0x70009c0e80, tpl_142::<0x1158> as *const (), 1), // +0x2d4e80 sub_2d4e80
    Lifted::new(0x70009c0ea0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3eb, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4ea0 sub_2d4ea0
    Lifted::new(0x70009c0f70, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3e7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d4f70 sub_2d4f70
    Lifted::new(0x70009c1040, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3e3, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d5040 sub_2d5040
    Lifted::new(0x70009c1110, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x3ef, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d5110 sub_2d5110
    Lifted::new(0x70009c11e0, tpl_142::<0xb78> as *const (), 1), // +0x2d51e0 sub_2d51e0
    Lifted::new(0x70009c1200, fn_70009c1200 as *const (), 0), // +0x2d5200 sub_2d5200
    Lifted::new(0x70009c12c0, tpl_297::<0x2fe8, 0xffffffffffffffff, 0x13, 0x207, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2d52c0 sub_2d52c0
    Lifted::new(0x70009c1390, tpl_142::<0xb80> as *const (), 1), // +0x2d5390 sub_2d5390
    Lifted::new(0x70009c13b0, tpl_142::<0xb88> as *const (), 1), // +0x2d53b0 sub_2d53b0
    Lifted::new(0x70009c13d0, tpl_142::<0xb90> as *const (), 1), // +0x2d53d0 sub_2d53d0
    Lifted::new(0x70009c13f0, tpl_36::<0x18> as *const (), 1), // +0x2d53f0 sub_2d53f0
    Lifted::new(0x70009c1400, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x2d5400 sub_2d5400
    Lifted::new(0x70009c1510, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x2d5510 sub_2d5510
    Lifted::new(0x70009c1620, fn_70009c1620 as *const (), 2), // +0x2d5620 sub_2d5620
    Lifted::new(0x70009c1fc0, fn_70009c1fc0 as *const (), 2), // +0x2d5fc0 sub_2d5fc0
    Lifted::new(0x70009c2ba0, fn_70009c2ba0 as *const (), 0), // +0x2d6ba0 sub_2d6ba0
    Lifted::new(0x70009c2ce0, fn_70009c2ce0 as *const (), 0), // +0x2d6ce0 sub_2d6ce0
    Lifted::new(0x70009c2e00, fn_70009c2e00 as *const (), 0), // +0x2d6e00 sub_2d6e00
    Lifted::new(0x70009c2f50, fn_70009c2f50 as *const (), 0), // +0x2d6f50 sub_2d6f50
    Lifted::new(0x70009c3040, fn_70009c3040 as *const (), 0), // +0x2d7040 sub_2d7040
    Lifted::new(0x70009c31b0, fn_70009c31b0 as *const (), 0), // +0x2d71b0 sub_2d71b0
    Lifted::new(0x70009c3320, fn_70009c3320 as *const (), 0), // +0x2d7320 sub_2d7320
    Lifted::new(0x70009c34b0, fn_70009c34b0 as *const (), 2), // +0x2d74b0 sub_2d74b0
    Lifted::new(0x70009c3880, fn_70009c3880 as *const (), 2), // +0x2d7880 sub_2d7880
    Lifted::new(0x70009c3f80, fn_70009c3f80 as *const (), 0), // +0x2d7f80 sub_2d7f80
    Lifted::new(0x70009c41c0, fn_70009c41c0 as *const (), 0), // +0x2d81c0 sub_2d81c0
    Lifted::new(0x70009c4440, fn_70009c4440 as *const (), 0), // +0x2d8440 sub_2d8440
    Lifted::new(0x70009c4680, fn_70009c4680 as *const (), 0), // +0x2d8680 sub_2d8680
    Lifted::new(0x70009c48a0, fn_70009c48a0 as *const (), 0), // +0x2d88a0 sub_2d88a0
    Lifted::new(0x70009c4990, fn_70009c4990 as *const (), 2), // +0x2d8990 sub_2d8990
    Lifted::new(0x70009c4ff0, fn_70009c4ff0 as *const (), 0), // +0x2d8ff0 sub_2d8ff0
    Lifted::new(0x70009c50b0, fn_70009c50b0 as *const (), 0), // +0x2d90b0 sub_2d90b0
    Lifted::new(0x70009c5180, fn_70009c5180 as *const (), 2), // +0x2d9180 sub_2d9180
    Lifted::new(0x70009c5710, fn_70009c5710 as *const (), 0), // +0x2d9710 sub_2d9710
    Lifted::new(0x70009c57d0, fn_70009c57d0 as *const (), 2), // +0x2d97d0 sub_2d97d0
    Lifted::new(0x70009c5ad0, fn_70009c5ad0 as *const (), 0), // +0x2d9ad0 sub_2d9ad0
    Lifted::new(0x70009c5d60, fn_70009c5d60 as *const (), 2), // +0x2d9d60 sub_2d9d60
    Lifted::new(0x70009c6070, tpl_70 as *const (), 1), // +0x2da070 sub_2da070
    Lifted::new(0x70009c6080, fn_70009c6080 as *const (), 0), // +0x2da080 sub_2da080
    Lifted::new(0x70009c6170, fn_70009c6170 as *const (), 2), // +0x2da170 sub_2da170
    Lifted::new(0x70009c6590, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x2da590 sub_2da590
    Lifted::new(0x70009c66a0, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x2da6a0 sub_2da6a0
    Lifted::new(0x70009c67b0, tpl_382::<0x10, 0x8, 0xa6e8, 0xa6d8, 0xa6e0, 0x7000745eb0, 0xa6f0, 0x70006f2a70, 0x70006f9a60> as *const (), 0), // +0x2da7b0 sub_2da7b0
    Lifted::new(0x70009c68e0, tpl_383::<0x70017d8e60, 0x70017d8e70, 0x70017d8e7c, 0x70017d8e80, 0x70017d8ec8, 0x70017d8ed8, 0x70017d8ee4, 0x70017d8ee8> as *const (), 3), // +0x2da8e0 sub_2da8e0
    Lifted::new(0x70009c6960, tpl_384::<0x70009c6963, 0x70009c6964, 0x70009c6965, 0x70009c6966, 0x70009c6967, 0x70009c6968, 0x70009c6969, 0x70009c696a, 0x70009c696b, 0x70009c696c, 0x70009c696d, 0x70009c696e, 0x70009c696f, 0x70009e4c50, 0x70009c6975, 0x70009c6976, 0x70009c6977, 0x70009c6978, 0x70009c6979, 0x70009c697a, 0x70009c697b, 0x70009c697c, 0x70009c697d, 0x70009c697e, 0x70009c697f> as *const (), 0), // +0x2da960 sub_2da960
    Lifted::new(0x70009c6990, fn_70009c6990 as *const (), 3), // +0x2da990 sub_2da990
    Lifted::new(0x70009c69c0, fn_70009c69c0 as *const (), 2), // +0x2da9c0 sub_2da9c0
    Lifted::new(0x70009c7240, fn_70009c7240 as *const (), 2), // +0x2db240 sub_2db240
    Lifted::new(0x70009c7a60, fn_70009c7a60 as *const (), 2), // +0x2dba60 sub_2dba60
    Lifted::new(0x70009c81b0, fn_70009c81b0 as *const (), 2), // +0x2dc1b0 sub_2dc1b0
    Lifted::new(0x70009c8930, fn_70009c8930 as *const (), 0), // +0x2dc930 sub_2dc930
    Lifted::new(0x70009c8bf0, fn_70009c8bf0 as *const (), 2), // +0x2dcbf0 sub_2dcbf0
    Lifted::new(0x70009c9210, fn_70009c9210 as *const (), 2), // +0x2dd210 sub_2dd210
    Lifted::new(0x70009c9830, tpl_385::<0x700076bf20> as *const (), 0), // +0x2dd830 sub_2dd830
    Lifted::new(0x70009c9950, tpl_385::<0x700076bf20> as *const (), 0), // +0x2dd950 sub_2dd950
    Lifted::new(0x70009c9a70, fn_70009c9a70 as *const (), 0), // +0x2dda70 sub_2dda70
    Lifted::new(0x70009c9b00, fn_70009c9b00 as *const (), 0), // +0x2ddb00 sub_2ddb00
    Lifted::new(0x70009c9ba0, tpl_386::<0x70018da5bc, 0x700177aa00, 0x7000724770, 0x18, 0x70017d8c38, 0x7000723490, 0x70009c9bf1> as *const (), 0), // +0x2ddba0 sub_2ddba0
    Lifted::new(0x70009c9c10, fn_70009c9c10 as *const (), 0), // +0x2ddc10 sub_2ddc10
    Lifted::new(0x70009c9cb0, fn_70009c9cb0 as *const (), 2), // +0x2ddcb0 sub_2ddcb0
    Lifted::new(0x70009c9de0, fn_70009c9de0 as *const (), 2), // +0x2ddde0 sub_2ddde0
    Lifted::new(0x70009c9ed0, fn_70009c9ed0 as *const (), 2), // +0x2dded0 sub_2dded0
    Lifted::new(0x70009ca320, fn_70009ca320 as *const (), 0), // +0x2de320 sub_2de320
    Lifted::new(0x70009ca350, fn_70009ca350 as *const (), 0), // +0x2de350 sub_2de350
    Lifted::new(0x70009ca520, fn_70009ca520 as *const (), 0), // +0x2de520 sub_2de520
    Lifted::new(0x70009ca700, fn_70009ca700 as *const (), 2), // +0x2de700 sub_2de700
    Lifted::new(0x70009caa40, fn_70009caa40 as *const (), 0), // +0x2dea40 sub_2dea40
    Lifted::new(0x70009caf70, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2def70 sub_2def70
    Lifted::new(0x70009cafb0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2defb0 sub_2defb0
    Lifted::new(0x70009caff0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2deff0 sub_2deff0
    Lifted::new(0x70009cb030, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df030 sub_2df030
    Lifted::new(0x70009cb070, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df070 sub_2df070
    Lifted::new(0x70009cb0b0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df0b0 sub_2df0b0
    Lifted::new(0x70009cb0f0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df0f0 sub_2df0f0
    Lifted::new(0x70009cb130, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df130 sub_2df130
    Lifted::new(0x70009cb170, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df170 sub_2df170
    Lifted::new(0x70009cb1b0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df1b0 sub_2df1b0
    Lifted::new(0x70009cb1f0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df1f0 sub_2df1f0
    Lifted::new(0x70009cb230, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df230 sub_2df230
    Lifted::new(0x70009cb270, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df270 sub_2df270
    Lifted::new(0x70009cb2b0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df2b0 sub_2df2b0
    Lifted::new(0x70009cb2f0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df2f0 sub_2df2f0
    Lifted::new(0x70009cb330, tpl_55::<0x700177c0d0> as *const (), 0), // +0x2df330 sub_2df330
    Lifted::new(0x70009cb370, fn_70009cb370 as *const (), 0), // +0x2df370 sub_2df370
    Lifted::new(0x70009cb3a0, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x70009cb52d, 0x7000a36d80, 0x70009cb533, 0x70007f0910, 0x70009cb539> as *const (), 0), // +0x2df3a0 sub_2df3a0
    Lifted::new(0x70009cb5a0, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x70009cb72d, 0x7000a36dc0, 0x70009cb733, 0x70007f0910, 0x70009cb739> as *const (), 0), // +0x2df5a0 sub_2df5a0
    Lifted::new(0x70009cb7a0, tpl_387::<0x7001779e20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x70009cb92d, 0x7000a36de0, 0x70009cb933, 0x70007f0910, 0x70009cb939> as *const (), 0), // +0x2df7a0 sub_2df7a0
    Lifted::new(0x70009cb9a0, tpl_388::<0x7000a36fb0, 0x700177c312, 0x7001d54968, 0x70009cbb2a, 0x7000a36e00, 0x70009cbb30> as *const (), 2), // +0x2df9a0 sub_2df9a0
    Lifted::new(0x70009cbba0, tpl_389::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x70009cbe1f, 0x7000a36e20, 0x70009cbe25, 0x70007f0910, 0x70009cbe2b> as *const (), 2), // +0x2dfba0 sub_2dfba0
    Lifted::new(0x70009cbed0, tpl_388::<0x7000a37040, 0x700177c312, 0x7001d54968, 0x70009cc05a, 0x7000a36e60, 0x70009cc060> as *const (), 2), // +0x2dfed0 sub_2dfed0
    Lifted::new(0x70009cc0d0, tpl_390::<0x7001c6abd8, 0x18, 0x10, 0x70009a6500> as *const (), 0), // +0x2e00d0 sub_2e00d0
    Lifted::new(0x70009cc1f0, fn_70009cc1f0 as *const (), 0), // +0x2e01f0 sub_2e01f0
    Lifted::new(0x70009cc410, tpl_391::<0x7000a36f20, 0x700177c0e0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x70009cc53c, 0x7000a36da0, 0x70009cc564> as *const (), 0), // +0x2e0410 sub_2e0410
    Lifted::new(0x70009cc5c0, fn_70009cc5c0 as *const (), 2), // +0x2e05c0 sub_2e05c0
    Lifted::new(0x70009cc7c0, fn_70009cc7c0 as *const (), 0), // +0x2e07c0 sub_2e07c0
    Lifted::new(0x70009cca00, tpl_20 as *const (), 1), // +0x2e0a00 sub_2e0a00
    Lifted::new(0x70009cca10, tpl_20 as *const (), 1), // +0x2e0a10 sub_2e0a10
    Lifted::new(0x70009cca20, tpl_392::<0x700177c0e0> as *const (), 0), // +0x2e0a20 sub_2e0a20
    Lifted::new(0x70009cca50, fn_70009cca50 as *const (), 0), // +0x2e0a50 sub_2e0a50
    Lifted::new(0x70009cca80, tpl_393::<0x7001779e20, 0x7001d54968, 0x70009ccb47, 0x700177c0d0, 0x7000a36670, 0x70009a4dd0, 0x70009ccbcb, 0x70007f0910, 0x70009ccbd1> as *const (), 0), // +0x2e0a80 sub_2e0a80
    Lifted::new(0x70009ccc30, tpl_394::<0x700076bf20, 0x7000a36e80, 0x70009ccda2> as *const (), 0), // +0x2e0c30 sub_2e0c30
    Lifted::new(0x70009cce00, tpl_395::<0x70009a55a0, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x70009ccf45, 0x70009a4eb0, 0x70009ccf4b> as *const (), 0), // +0x2e0e00 sub_2e0e00
    Lifted::new(0x70009ccfa0, tpl_395::<0x7000a36f20, 0x700177c0d0, 0x700177c312, 0x7001d54968, 0x70009cd0e5, 0x7000a36da0, 0x70009cd0eb> as *const (), 0), // +0x2e0fa0 sub_2e0fa0
    Lifted::new(0x70009cd140, tpl_20 as *const (), 1), // +0x2e1140 sub_2e1140
    Lifted::new(0x70009cd150, tpl_20 as *const (), 1), // +0x2e1150 sub_2e1150
    Lifted::new(0x70009cd160, tpl_20 as *const (), 1), // +0x2e1160 sub_2e1160
    Lifted::new(0x70009cd170, fn_70009cd170 as *const (), 0), // +0x2e1170 sub_2e1170
    Lifted::new(0x70009cd1a0, tpl_396::<0x7001779e20, 0x70007031a0> as *const (), 0), // +0x2e11a0 sub_2e11a0
    Lifted::new(0x70009cd220, fn_70009cd220 as *const (), 0), // +0x2e1220 sub_2e1220
    Lifted::new(0x70009cd3a0, fn_70009cd3a0 as *const (), 0), // +0x2e13a0 sub_2e13a0
    Lifted::new(0x70009cd560, fn_70009cd560 as *const (), 0), // +0x2e1560 sub_2e1560
    Lifted::new(0x70009cd710, fn_70009cd710 as *const (), 1), // +0x2e1710 sub_2e1710
    Lifted::new(0x70009cd830, tpl_93::<0x8> as *const (), 1), // +0x2e1830 sub_2e1830
    Lifted::new(0x70009cd840, tpl_6::<0x8> as *const (), 1), // +0x2e1840 sub_2e1840
    Lifted::new(0x70009cd850, tpl_93::<0x8> as *const (), 1), // +0x2e1850 sub_2e1850
    Lifted::new(0x70009cd860, fn_70009cd860 as *const (), 2), // +0x2e1860 sub_2e1860
    Lifted::new(0x70009cdbe0, fn_70009cdbe0 as *const (), 2), // +0x2e1be0 sub_2e1be0
    Lifted::new(0x70009cdf40, tpl_70 as *const (), 1), // +0x2e1f40 sub_2e1f40
    Lifted::new(0x70009cdf50, fn_70009cdf50 as *const (), 0), // +0x2e1f50 sub_2e1f50
    Lifted::new(0x70009ce010, fn_70009ce010 as *const (), 2), // +0x2e2010 sub_2e2010
    Lifted::new(0x70009ce340, tpl_397::<0x8, 0xc, 0x10, 0x18, 0x700076bf20> as *const (), 0), // +0x2e2340 sub_2e2340
    Lifted::new(0x70009ce3c0, fn_70009ce3c0 as *const (), 2), // +0x2e23c0 sub_2e23c0
    Lifted::new(0x70009ce6f0, tpl_397::<0x8, 0xc, 0x10, 0x18, 0x700076bf20> as *const (), 0), // +0x2e26f0 sub_2e26f0
    Lifted::new(0x70009ce770, fn_70009ce770 as *const (), 2), // +0x2e2770 sub_2e2770
    Lifted::new(0x70009ceaa0, tpl_397::<0x8, 0xc, 0x10, 0x18, 0x700076bf20> as *const (), 0), // +0x2e2aa0 sub_2e2aa0
    Lifted::new(0x70009ceb20, fn_70009ceb20 as *const (), 2), // +0x2e2b20 sub_2e2b20
    Lifted::new(0x70009cee60, fn_70009cee60 as *const (), 0), // +0x2e2e60 sub_2e2e60
    Lifted::new(0x70009ceec0, fn_70009ceec0 as *const (), 0), // +0x2e2ec0 sub_2e2ec0
    Lifted::new(0x70009cf000, fn_70009cf000 as *const (), 0), // +0x2e3000 sub_2e3000
    Lifted::new(0x70009cf070, fn_70009cf070 as *const (), 0), // +0x2e3070 sub_2e3070
    Lifted::new(0x70009cf130, fn_70009cf130 as *const (), 1), // +0x2e3130 sub_2e3130
    Lifted::new(0x70009cf160, fn_70009cf160 as *const (), 0), // +0x2e3160 sub_2e3160
    Lifted::new(0x70009cf1c0, fn_70009cf1c0 as *const (), 1), // +0x2e31c0 sub_2e31c0
    Lifted::new(0x70009cf1e0, fn_70009cf1e0 as *const (), 1), // +0x2e31e0 sub_2e31e0
    Lifted::new(0x70009cf200, fn_70009cf200 as *const (), 0), // +0x2e3200 sub_2e3200
    Lifted::new(0x70009cf4f0, fn_70009cf4f0 as *const (), 0), // +0x2e34f0 sub_2e34f0
    Lifted::new(0x70009cf580, tpl_13::<0x7000707700> as *const (), 0), // +0x2e3580 sub_2e3580
    Lifted::new(0x70009cf5a0, fn_70009cf5a0 as *const (), 0), // +0x2e35a0 sub_2e35a0
    Lifted::new(0x70009cf620, tpl_7::<0x70017d4230> as *const (), 1), // +0x2e3620 sub_2e3620
    Lifted::new(0x70009cf640, tpl_7::<0x70017d4230> as *const (), 1), // +0x2e3640 sub_2e3640
    Lifted::new(0x70009cf660, fn_70009cf660 as *const (), 0), // +0x2e3660 sub_2e3660
    Lifted::new(0x70009cf750, fn_70009cf750 as *const (), 0), // +0x2e3750 sub_2e3750
    Lifted::new(0x70009cf7a0, tpl_93::<0x8> as *const (), 1), // +0x2e37a0 sub_2e37a0
    Lifted::new(0x70009cf7b0, tpl_7::<0x70017d3d08> as *const (), 1), // +0x2e37b0 sub_2e37b0
    Lifted::new(0x70009cf7d0, tpl_7::<0x70017d3d08> as *const (), 1), // +0x2e37d0 sub_2e37d0
    Lifted::new(0x70009cf7f0, fn_70009cf7f0 as *const (), 0), // +0x2e37f0 sub_2e37f0
    Lifted::new(0x70009cf870, fn_70009cf870 as *const (), 1), // +0x2e3870 sub_2e3870
    Lifted::new(0x70009cf8e0, fn_70009cf8e0 as *const (), 3), // +0x2e38e0 sub_2e38e0
    Lifted::new(0x70009cfa40, fn_70009cfa40 as *const (), 0), // +0x2e3a40 sub_2e3a40
    Lifted::new(0x70009cfcf0, tpl_11::<0x70017d3f50, 0x8> as *const (), 1), // +0x2e3cf0 sub_2e3cf0
    Lifted::new(0x70009cfd10, tpl_11::<0x70017d3f50, 0x8> as *const (), 1), // +0x2e3d10 sub_2e3d10
    Lifted::new(0x70009cfd30, tpl_9::<0x70017d3f50, 0x8> as *const (), 1), // +0x2e3d30 sub_2e3d30
    Lifted::new(0x70009cfd50, fn_70009cfd50 as *const (), 0), // +0x2e3d50 sub_2e3d50
    Lifted::new(0x70009cfde0, fn_70009cfde0 as *const (), 0), // +0x2e3de0 sub_2e3de0
    Lifted::new(0x70009cfe20, tpl_70 as *const (), 1), // +0x2e3e20 sub_2e3e20
    Lifted::new(0x70009cfe30, tpl_398::<0x10, 0x18, 0x28, 0x8, 0x30> as *const (), 1), // +0x2e3e30 sub_2e3e30
    Lifted::new(0x70009cfe60, fn_70009cfe60 as *const (), 0), // +0x2e3e60 sub_2e3e60
    Lifted::new(0x70009cfee0, fn_70009cfee0 as *const (), 0), // +0x2e3ee0 sub_2e3ee0
    Lifted::new(0x70009cffd0, fn_70009cffd0 as *const (), 2), // +0x2e3fd0 sub_2e3fd0
    Lifted::new(0x70009d0190, fn_70009d0190 as *const (), 3), // +0x2e4190 sub_2e4190
    Lifted::new(0x70009d01c0, fn_70009d01c0 as *const (), 3), // +0x2e41c0 sub_2e41c0
    Lifted::new(0x70009d0250, tpl_13::<0x7000778cd0> as *const (), 0), // +0x2e4250 sub_2e4250
    Lifted::new(0x70009d0270, tpl_7::<0x70017d3c98> as *const (), 1), // +0x2e4270 sub_2e4270
    Lifted::new(0x70009d0290, tpl_7::<0x70017d3c98> as *const (), 1), // +0x2e4290 sub_2e4290
    Lifted::new(0x70009d02b0, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2e42b0 sub_2e42b0
    Lifted::new(0x70009d02d0, tpl_399::<0x8, 0x10, 0x18> as *const (), 1), // +0x2e42d0 sub_2e42d0
    Lifted::new(0x70009d02f0, tpl_400::<0x8, 0x10, 0x18, 0x20> as *const (), 1), // +0x2e42f0 sub_2e42f0
    Lifted::new(0x70009d0310, fn_70009d0310 as *const (), 1), // +0x2e4310 sub_2e4310
    Lifted::new(0x70009d0360, tpl_7::<0x70017d4250> as *const (), 1), // +0x2e4360 sub_2e4360
    Lifted::new(0x70009d0380, tpl_7::<0x70017d4250> as *const (), 1), // +0x2e4380 sub_2e4380
    Lifted::new(0x70009d03a0, fn_70009d03a0 as *const (), 3), // +0x2e43a0 sub_2e43a0
    Lifted::new(0x70009d0410, fn_70009d0410 as *const (), 1), // +0x2e4410 sub_2e4410
    Lifted::new(0x70009d0480, fn_70009d0480 as *const (), 1), // +0x2e4480 sub_2e4480
    Lifted::new(0x70009d04c0, fn_70009d04c0 as *const (), 1), // +0x2e44c0 sub_2e44c0
    Lifted::new(0x70009d04f0, fn_70009d04f0 as *const (), 0), // +0x2e44f0 sub_2e44f0
    Lifted::new(0x70009d0510, fn_70009d0510 as *const (), 0), // +0x2e4510 sub_2e4510
    Lifted::new(0x70009d05f0, tpl_7::<0x70017d3c58> as *const (), 1), // +0x2e45f0 sub_2e45f0
    Lifted::new(0x70009d0610, tpl_7::<0x70017d3c58> as *const (), 1), // +0x2e4610 sub_2e4610
    Lifted::new(0x70009d0630, fn_70009d0630 as *const (), 2), // +0x2e4630 sub_2e4630
    Lifted::new(0x70009d0770, fn_70009d0770 as *const (), 0), // +0x2e4770 sub_2e4770
    Lifted::new(0x70009d09b0, fn_70009d09b0 as *const (), 0), // +0x2e49b0 sub_2e49b0
    Lifted::new(0x70009d0a30, fn_70009d0a30 as *const (), 0), // +0x2e4a30 sub_2e4a30
    Lifted::new(0x70009d0a90, fn_70009d0a90 as *const (), 0), // +0x2e4a90 sub_2e4a90
    Lifted::new(0x70009d0b20, fn_70009d0b20 as *const (), 0), // +0x2e4b20 sub_2e4b20
    Lifted::new(0x70009d0b80, fn_70009d0b80 as *const (), 0), // +0x2e4b80 sub_2e4b80
    Lifted::new(0x70009d0ca0, fn_70009d0ca0 as *const (), 0), // +0x2e4ca0 sub_2e4ca0
    Lifted::new(0x70009d0d60, tpl_400::<0x8, 0x10, 0x18, 0x20> as *const (), 1), // +0x2e4d60 sub_2e4d60
    Lifted::new(0x70009d0d80, fn_70009d0d80 as *const (), 0), // +0x2e4d80 sub_2e4d80
    Lifted::new(0x70009d0dc0, tpl_401::<0xc958, 0x8> as *const (), 1), // +0x2e4dc0 sub_2e4dc0
    Lifted::new(0x70009d0de0, tpl_398::<0x10, 0x18, 0x28, 0x8, 0x30> as *const (), 1), // +0x2e4de0 sub_2e4de0
    Lifted::new(0x70009d0e10, fn_70009d0e10 as *const (), 1), // +0x2e4e10 sub_2e4e10
    Lifted::new(0x70009d1030, fn_70009d1030 as *const (), 1), // +0x2e5030 sub_2e5030
    Lifted::new(0x70009d1080, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2e5080 sub_2e5080
    Lifted::new(0x70009d10a0, fn_70009d10a0 as *const (), 0), // +0x2e50a0 sub_2e50a0
    Lifted::new(0x70009d11e0, fn_70009d11e0 as *const (), 0), // +0x2e51e0 sub_2e51e0
    Lifted::new(0x70009d1240, tpl_402::<0x8, 0x10> as *const (), 1), // +0x2e5240 sub_2e5240
    Lifted::new(0x70009d1260, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2e5260 sub_2e5260
    Lifted::new(0x70009d1280, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2e5280 sub_2e5280
    Lifted::new(0x70009d12a0, fn_70009d12a0 as *const (), 0), // +0x2e52a0 sub_2e52a0
    Lifted::new(0x70009d1330, fn_70009d1330 as *const (), 2), // +0x2e5330 sub_2e5330
    Lifted::new(0x70009d13b0, fn_70009d13b0 as *const (), 1), // +0x2e53b0 sub_2e53b0
    Lifted::new(0x70009d13f0, tpl_7::<0x70017d4280> as *const (), 1), // +0x2e53f0 sub_2e53f0
    Lifted::new(0x70009d1410, tpl_7::<0x70017d4280> as *const (), 1), // +0x2e5410 sub_2e5410
    Lifted::new(0x70009d1430, fn_70009d1430 as *const (), 1), // +0x2e5430 sub_2e5430
    Lifted::new(0x70009d1470, fn_70009d1470 as *const (), 1), // +0x2e5470 sub_2e5470
    Lifted::new(0x70009d14c0, fn_70009d14c0 as *const (), 0), // +0x2e54c0 sub_2e54c0
    Lifted::new(0x70009d1530, fn_70009d1530 as *const (), 0), // +0x2e5530 sub_2e5530
    Lifted::new(0x70009d16f0, fn_70009d16f0 as *const (), 0), // +0x2e56f0 sub_2e56f0
    Lifted::new(0x70009d1930, fn_70009d1930 as *const (), 0), // +0x2e5930 sub_2e5930
    Lifted::new(0x70009d1960, fn_70009d1960 as *const (), 0), // +0x2e5960 sub_2e5960
    Lifted::new(0x70009d1a80, fn_70009d1a80 as *const (), 0), // +0x2e5a80 sub_2e5a80
    Lifted::new(0x70009d1b10, fn_70009d1b10 as *const (), 0), // +0x2e5b10 sub_2e5b10
    Lifted::new(0x70009d1b90, tpl_7::<0x70017d3ca8> as *const (), 1), // +0x2e5b90 sub_2e5b90
    Lifted::new(0x70009d1bb0, tpl_7::<0x70017d3ca8> as *const (), 1), // +0x2e5bb0 sub_2e5bb0
    Lifted::new(0x70009d1bd0, fn_70009d1bd0 as *const (), 3), // +0x2e5bd0 sub_2e5bd0
    Lifted::new(0x70009d1c20, tpl_403::<0x70007ed7e0> as *const (), 0), // +0x2e5c20 sub_2e5c20
    Lifted::new(0x70009d1c50, fn_70009d1c50 as *const (), 2), // +0x2e5c50 sub_2e5c50
    Lifted::new(0x70009d1db0, fn_70009d1db0 as *const (), 3), // +0x2e5db0 sub_2e5db0
    Lifted::new(0x70009d1e00, fn_70009d1e00 as *const (), 1), // +0x2e5e00 sub_2e5e00
    Lifted::new(0x70009d1e20, tpl_399::<0x8, 0x10, 0x18> as *const (), 1), // +0x2e5e20 sub_2e5e20
    Lifted::new(0x70009d1e40, fn_70009d1e40 as *const (), 0), // +0x2e5e40 sub_2e5e40
    Lifted::new(0x70009d1f80, tpl_404::<0x8, 0x10, 0x18> as *const (), 1), // +0x2e5f80 sub_2e5f80
    Lifted::new(0x70009d1fa0, fn_70009d1fa0 as *const (), 0), // +0x2e5fa0 sub_2e5fa0
    Lifted::new(0x70009d20b0, fn_70009d20b0 as *const (), 0), // +0x2e60b0 sub_2e60b0
    Lifted::new(0x70009d2160, fn_70009d2160 as *const (), 0), // +0x2e6160 sub_2e6160
    Lifted::new(0x70009d2270, tpl_405::<0xb0, 0xc0, 0xfffffffffffffff8, 0x700177c312, 0xb8, 0x98, 0xa8, 0xa0, 0x7001d54968, 0x70009d233a> as *const (), 0), // +0x2e6270 sub_2e6270
    Lifted::new(0x70009d2370, tpl_406::<0x70009dca60, 0x8, 0x18, 0xfffffffffffffff8, 0x700177c312, 0x10, 0x7001d54968, 0x70009d23d2> as *const (), 0), // +0x2e6370 sub_2e6370
    Lifted::new(0x70009d2440, tpl_104::<0x8, 0xc> as *const (), 0), // +0x2e6440 sub_2e6440
    Lifted::new(0x70009d24a0, fn_70009d24a0 as *const (), 0), // +0x2e64a0 sub_2e64a0
    Lifted::new(0x70009d24c0, tpl_407::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x70009d2539> as *const (), 0), // +0x2e64c0 sub_2e64c0
    Lifted::new(0x70009d2560, fn_70009d2560 as *const (), 0), // +0x2e6560 sub_2e6560
    Lifted::new(0x70009d2670, fn_70009d2670 as *const (), 0), // +0x2e6670 sub_2e6670
    Lifted::new(0x70009d26e0, fn_70009d26e0 as *const (), 1), // +0x2e66e0 sub_2e66e0
    Lifted::new(0x70009d2710, fn_70009d2710 as *const (), 0), // +0x2e6710 sub_2e6710
    Lifted::new(0x70009d2730, tpl_16::<0x70009d2733, 0x70009d2734, 0x70009d2735, 0x70009d2736, 0x70009d2737, 0x70009d2738, 0x70009d2739, 0x70009d273a, 0x70009d273b, 0x70009d273c, 0x70009d273d, 0x70009d273e, 0x70009d273f, 0x7000733100> as *const (), 0), // +0x2e6730 sub_2e6730
    Lifted::new(0x70009d2750, fn_70009d2750 as *const (), 0), // +0x2e6750 sub_2e6750
    Lifted::new(0x70009d2790, tpl_15 as *const (), 1), // +0x2e6790 sub_2e6790
    Lifted::new(0x70009d27a0, tpl_408::<0x7000744f50, 0x700177c312> as *const (), 0), // +0x2e67a0 sub_2e67a0
    Lifted::new(0x70009d27e0, tpl_16::<0x70009d27e3, 0x70009d27e4, 0x70009d27e5, 0x70009d27e6, 0x70009d27e7, 0x70009d27e8, 0x70009d27e9, 0x70009d27ea, 0x70009d27eb, 0x70009d27ec, 0x70009d27ed, 0x70009d27ee, 0x70009d27ef, 0x70009d2560> as *const (), 0), // +0x2e67e0 sub_2e67e0
    Lifted::new(0x70009d2800, fn_70009d2800 as *const (), 0), // +0x2e6800 sub_2e6800
    Lifted::new(0x70009d2840, tpl_102::<0x70, 0x8, 0xc> as *const (), 0), // +0x2e6840 sub_2e6840
    Lifted::new(0x70009d28a0, fn_70009d28a0 as *const (), 0), // +0x2e68a0 sub_2e68a0
    Lifted::new(0x70009d28f0, tpl_15 as *const (), 1), // +0x2e68f0 sub_2e68f0
    Lifted::new(0x70009d2900, tpl_15 as *const (), 1), // +0x2e6900 sub_2e6900
    Lifted::new(0x70009d2910, fn_70009d2910 as *const (), 0), // +0x2e6910 sub_2e6910
    Lifted::new(0x70009d2980, fn_70009d2980 as *const (), 1), // +0x2e6980 sub_2e6980
    Lifted::new(0x70009d2990, fn_70009d2990 as *const (), 2), // +0x2e6990 sub_2e6990
    Lifted::new(0x70009d2b40, tpl_409::<0x20, 0x30, 0xfffffffffffffff8, 0x700177c312, 0x28, 0x8, 0x18, 0x10, 0x7001d54968, 0x70009d2bec> as *const (), 0), // +0x2e6b40 sub_2e6b40
    Lifted::new(0x70009d2c40, fn_70009d2c40 as *const (), 0), // +0x2e6c40 sub_2e6c40
    Lifted::new(0x70009d2c80, tpl_410::<0x70009d2c83, 0x70009d2c84, 0x70009d2c85, 0x70009d2c86, 0x70009d2c87, 0x70009d2c88, 0x70009d2c89, 0x70009d2c8a, 0x70009d2c8b, 0x70009d2c8c, 0x70009d2c8d, 0x70009d2c8e, 0x70009d2c8f> as *const (), 1), // +0x2e6c80 sub_2e6c80
    Lifted::new(0x70009d2ca0, fn_70009d2ca0 as *const (), 0), // +0x2e6ca0 sub_2e6ca0
    Lifted::new(0x70009d2cd0, fn_70009d2cd0 as *const (), 1), // +0x2e6cd0 sub_2e6cd0
    Lifted::new(0x70009d2cf0, fn_70009d2cf0 as *const (), 0), // +0x2e6cf0 sub_2e6cf0
    Lifted::new(0x70009d2d50, fn_70009d2d50 as *const (), 0), // +0x2e6d50 sub_2e6d50
    Lifted::new(0x70009d2e60, fn_70009d2e60 as *const (), 0), // +0x2e6e60 sub_2e6e60
    Lifted::new(0x70009d2fc0, tpl_15 as *const (), 1), // +0x2e6fc0 sub_2e6fc0
    Lifted::new(0x70009d2fd0, tpl_411::<0x7000748160, 0x18, 0x7000722bd0> as *const (), 0), // +0x2e6fd0 sub_2e6fd0
    Lifted::new(0x70009d3000, tpl_17::<0x700177c312> as *const (), 0), // +0x2e7000 sub_2e7000
    Lifted::new(0x70009d3010, fn_70009d3010 as *const (), 0), // +0x2e7010 sub_2e7010
    Lifted::new(0x70009d3040, tpl_412::<0x8, 0xc958> as *const (), 1), // +0x2e7040 sub_2e7040
    Lifted::new(0x70009d3060, fn_70009d3060 as *const (), 0), // +0x2e7060 sub_2e7060
    Lifted::new(0x70009d3110, tpl_15 as *const (), 1), // +0x2e7110 sub_2e7110
    Lifted::new(0x70009d3120, fn_70009d3120 as *const (), 1), // +0x2e7120 sub_2e7120
    Lifted::new(0x70009d3140, fn_70009d3140 as *const (), 0), // +0x2e7140 sub_2e7140
    Lifted::new(0x70009d3210, tpl_15 as *const (), 1), // +0x2e7210 sub_2e7210
    Lifted::new(0x70009d3220, tpl_408::<0x7000afb3c0, 0x700177c312> as *const (), 0), // +0x2e7220 sub_2e7220
    Lifted::new(0x70009d3260, fn_70009d3260 as *const (), 0), // +0x2e7260 sub_2e7260
    Lifted::new(0x70009d3310, tpl_15 as *const (), 1), // +0x2e7310 sub_2e7310
    Lifted::new(0x70009d3320, fn_70009d3320 as *const (), 1), // +0x2e7320 sub_2e7320
    Lifted::new(0x70009d3340, fn_70009d3340 as *const (), 0), // +0x2e7340 sub_2e7340
    Lifted::new(0x70009d3390, fn_70009d3390 as *const (), 0), // +0x2e7390 sub_2e7390
    Lifted::new(0x70009d3590, tpl_17::<0x700177c312> as *const (), 0), // +0x2e7590 sub_2e7590
    Lifted::new(0x70009d35a0, fn_70009d35a0 as *const (), 0), // +0x2e75a0 sub_2e75a0
    Lifted::new(0x70009d38b0, tpl_17::<0x700177c312> as *const (), 0), // +0x2e78b0 sub_2e78b0
    Lifted::new(0x70009d38c0, tpl_413::<0x8, 0x7000f350c0, 0x700177c312> as *const (), 0), // +0x2e78c0 sub_2e78c0
    Lifted::new(0x70009d3900, tpl_413::<0x8, 0x7000f35100, 0x700177c312> as *const (), 0), // +0x2e7900 sub_2e7900
    Lifted::new(0x70009d3940, tpl_15 as *const (), 1), // +0x2e7940 sub_2e7940
    Lifted::new(0x70009d3950, tpl_414::<0x20, 0x8, 0xfffffffffffffff8, 0x700177c312, 0x18, 0x7001d54968, 0x70009d39b0> as *const (), 0), // +0x2e7950 sub_2e7950
    Lifted::new(0x70009d39d0, fn_70009d39d0 as *const (), 0), // +0x2e79d0 sub_2e79d0
    Lifted::new(0x70009d3a50, tpl_406::<0x7000772c80, 0x8, 0x18, 0xfffffffffffffff8, 0x700177c312, 0x10, 0x7001d54968, 0x70009d3ab2> as *const (), 0), // +0x2e7a50 sub_2e7a50
    Lifted::new(0x70009d3ae0, fn_70009d3ae0 as *const (), 0), // +0x2e7ae0 sub_2e7ae0
    Lifted::new(0x70009d3b00, tpl_112::<0x7001d55090, 0x70009d3b0a> as *const (), 0), // +0x2e7b00 sub_2e7b00
    Lifted::new(0x70009d3b10, tpl_112::<0x7001d55090, 0x70009d3b1a> as *const (), 0), // +0x2e7b10 sub_2e7b10
    Lifted::new(0x70009d3b20, tpl_112::<0x7001d55090, 0x70009d3b2a> as *const (), 0), // +0x2e7b20 sub_2e7b20
    Lifted::new(0x70009d3b30, tpl_112::<0x7001d55090, 0x70009d3b3a> as *const (), 0), // +0x2e7b30 sub_2e7b30
    Lifted::new(0x70009d3b40, tpl_112::<0x7001d55090, 0x70009d3b4a> as *const (), 0), // +0x2e7b40 sub_2e7b40
    Lifted::new(0x70009d3b60, tpl_112::<0x7001d55090, 0x70009d3b6a> as *const (), 0), // +0x2e7b60 sub_2e7b60
    Lifted::new(0x70009d3b70, tpl_112::<0x7001d55090, 0x70009d3b7a> as *const (), 0), // +0x2e7b70 sub_2e7b70
    Lifted::new(0x70009d3b80, tpl_112::<0x7001d55090, 0x70009d3b8a> as *const (), 0), // +0x2e7b80 sub_2e7b80
    Lifted::new(0x70009d3b90, tpl_112::<0x7001d55090, 0x70009d3b9a> as *const (), 0), // +0x2e7b90 sub_2e7b90
    Lifted::new(0x70009d3ba0, tpl_112::<0x7001d55090, 0x70009d3baa> as *const (), 0), // +0x2e7ba0 sub_2e7ba0
    Lifted::new(0x70009d3bb0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x70009d3bc1> as *const (), 0), // +0x2e7bb0 sub_2e7bb0
    Lifted::new(0x70009d3bd0, tpl_18::<0x8> as *const (), 1), // +0x2e7bd0 sub_2e7bd0
    Lifted::new(0x70009d3bf0, tpl_19 as *const (), 3), // +0x2e7bf0 sub_2e7bf0
    Lifted::new(0x70009d3c00, fn_70009d3c00 as *const (), 2), // +0x2e7c00 sub_2e7c00
    Lifted::new(0x70009d3cb0, tpl_415::<0x7001c6abd8> as *const (), 0), // +0x2e7cb0 sub_2e7cb0
    Lifted::new(0x70009d3ea0, tpl_20 as *const (), 1), // +0x2e7ea0 sub_2e7ea0
    Lifted::new(0x70009d3eb0, tpl_20 as *const (), 1), // +0x2e7eb0 sub_2e7eb0
    Lifted::new(0x70009d3ec0, tpl_20 as *const (), 1), // +0x2e7ec0 sub_2e7ec0
    Lifted::new(0x70009d3ed0, tpl_20 as *const (), 1), // +0x2e7ed0 sub_2e7ed0
    Lifted::new(0x70009d3ee0, fn_70009d3ee0 as *const (), 3), // +0x2e7ee0 sub_2e7ee0
    Lifted::new(0x70009d41b0, tpl_22::<0x8> as *const (), 1), // +0x2e81b0 sub_2e81b0
    Lifted::new(0x70009d41c0, tpl_22::<0x8> as *const (), 1), // +0x2e81c0 sub_2e81c0
    Lifted::new(0x70009d41d0, tpl_28 as *const (), 1), // +0x2e81d0 sub_2e81d0
    Lifted::new(0x70009d41e0, tpl_28 as *const (), 1), // +0x2e81e0 sub_2e81e0
    Lifted::new(0x70009d41f0, tpl_20 as *const (), 1), // +0x2e81f0 sub_2e81f0
    Lifted::new(0x70009d4200, tpl_20 as *const (), 1), // +0x2e8200 sub_2e8200
    Lifted::new(0x70009d4210, tpl_20 as *const (), 1), // +0x2e8210 sub_2e8210
    Lifted::new(0x70009d4220, tpl_20 as *const (), 1), // +0x2e8220 sub_2e8220
    Lifted::new(0x70009d4230, tpl_20 as *const (), 1), // +0x2e8230 sub_2e8230
    Lifted::new(0x70009d4240, tpl_20 as *const (), 1), // +0x2e8240 sub_2e8240
    Lifted::new(0x70009d4250, tpl_20 as *const (), 1), // +0x2e8250 sub_2e8250
    Lifted::new(0x70009d4260, tpl_20 as *const (), 1), // +0x2e8260 sub_2e8260
    Lifted::new(0x70009d4270, tpl_20 as *const (), 1), // +0x2e8270 sub_2e8270
    Lifted::new(0x70009d4280, tpl_20 as *const (), 1), // +0x2e8280 sub_2e8280
    Lifted::new(0x70009d4290, tpl_20 as *const (), 1), // +0x2e8290 sub_2e8290
    Lifted::new(0x70009d42a0, tpl_20 as *const (), 1), // +0x2e82a0 sub_2e82a0
    Lifted::new(0x70009d42b0, tpl_20 as *const (), 1), // +0x2e82b0 sub_2e82b0
    Lifted::new(0x70009d42c0, tpl_20 as *const (), 1), // +0x2e82c0 sub_2e82c0
    Lifted::new(0x70009d42d0, tpl_20 as *const (), 1), // +0x2e82d0 sub_2e82d0
    Lifted::new(0x70009d42e0, fn_70009d42e0 as *const (), 0), // +0x2e82e0 sub_2e82e0
    Lifted::new(0x70009d4390, tpl_20 as *const (), 1), // +0x2e8390 sub_2e8390
    Lifted::new(0x70009d43a0, tpl_20 as *const (), 1), // +0x2e83a0 sub_2e83a0
    Lifted::new(0x70009d43b0, tpl_23::<0x8, 0x10> as *const (), 1), // +0x2e83b0 sub_2e83b0
    Lifted::new(0x70009d43d0, tpl_21::<0x10> as *const (), 3), // +0x2e83d0 sub_2e83d0
    Lifted::new(0x70009d43f0, fn_70009d43f0 as *const (), 2), // +0x2e83f0 sub_2e83f0
    Lifted::new(0x70009d4460, tpl_20 as *const (), 1), // +0x2e8460 sub_2e8460
    Lifted::new(0x70009d4470, fn_70009d4470 as *const (), 1), // +0x2e8470 sub_2e8470
    Lifted::new(0x70009d4560, fn_70009d4560 as *const (), 0), // +0x2e8560 sub_2e8560
    Lifted::new(0x70009d4590, tpl_20 as *const (), 1), // +0x2e8590 sub_2e8590
    Lifted::new(0x70009d45a0, tpl_20 as *const (), 1), // +0x2e85a0 sub_2e85a0
    Lifted::new(0x70009d45b0, tpl_20 as *const (), 1), // +0x2e85b0 sub_2e85b0
    Lifted::new(0x70009d45c0, tpl_20 as *const (), 1), // +0x2e85c0 sub_2e85c0
    Lifted::new(0x70009d45d0, tpl_20 as *const (), 1), // +0x2e85d0 sub_2e85d0
    Lifted::new(0x70009d45e0, tpl_20 as *const (), 1), // +0x2e85e0 sub_2e85e0
    Lifted::new(0x70009d45f0, tpl_20 as *const (), 1), // +0x2e85f0 sub_2e85f0
    Lifted::new(0x70009d4600, tpl_403::<0x70009d3c00> as *const (), 0), // +0x2e8600 sub_2e8600
    Lifted::new(0x70009d4630, fn_70009d4630 as *const (), 0), // +0x2e8630 sub_2e8630
    Lifted::new(0x70009d4680, tpl_20 as *const (), 1), // +0x2e8680 sub_2e8680
    Lifted::new(0x70009d4690, tpl_20 as *const (), 1), // +0x2e8690 sub_2e8690
    Lifted::new(0x70009d46a0, tpl_20 as *const (), 1), // +0x2e86a0 sub_2e86a0
    Lifted::new(0x70009d46b0, tpl_20 as *const (), 1), // +0x2e86b0 sub_2e86b0
    Lifted::new(0x70009d46c0, tpl_20 as *const (), 1), // +0x2e86c0 sub_2e86c0
    Lifted::new(0x70009d46d0, tpl_20 as *const (), 1), // +0x2e86d0 sub_2e86d0
    Lifted::new(0x70009d46e0, fn_70009d46e0 as *const (), 0), // +0x2e86e0 sub_2e86e0
    Lifted::new(0x70009d47e0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2e87e0 sub_2e87e0
    Lifted::new(0x70009d4830, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2e8830 sub_2e8830
    Lifted::new(0x70009d4880, fn_70009d4880 as *const (), 0), // +0x2e8880 sub_2e8880
    Lifted::new(0x70009d49c0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2e89c0 sub_2e89c0
    Lifted::new(0x70009d4a10, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2e8a10 sub_2e8a10
    Lifted::new(0x70009d4a60, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2e8a60 sub_2e8a60
    Lifted::new(0x70009d4ab0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2e8ab0 sub_2e8ab0
    Lifted::new(0x70009d4b00, fn_70009d4b00 as *const (), 0), // +0x2e8b00 sub_2e8b00
    Lifted::new(0x70009d4c60, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2e8c60 sub_2e8c60
    Lifted::new(0x70009d4cb0, tpl_31::<0xfffffffffffffff8, 0x700177c312> as *const (), 0), // +0x2e8cb0 sub_2e8cb0
    Lifted::new(0x70009d4d00, tpl_33::<0x700177c312> as *const (), 0), // +0x2e8d00 sub_2e8d00
    Lifted::new(0x70009d4d30, tpl_33::<0x700177c312> as *const (), 0), // +0x2e8d30 sub_2e8d30
    Lifted::new(0x70009d4d60, tpl_33::<0x700177c312> as *const (), 0), // +0x2e8d60 sub_2e8d60
    Lifted::new(0x70009d4d90, tpl_416::<0x70009d3bb0, 0x70009d4dab> as *const (), 0), // +0x2e8d90 sub_2e8d90
    Lifted::new(0x70009d4dc0, tpl_417::<0x70009d3bb0, 0x70009d4ddb> as *const (), 0), // +0x2e8dc0 sub_2e8dc0
    Lifted::new(0x70009d4df0, tpl_417::<0x70009d3bb0, 0x70009d4e0b> as *const (), 0), // +0x2e8df0 sub_2e8df0
    Lifted::new(0x70009d4e20, tpl_417::<0x70009d3bb0, 0x70009d4e3b> as *const (), 0), // +0x2e8e20 sub_2e8e20
    Lifted::new(0x70009d4e50, tpl_34::<0x70017d3d18, 0x8, 0x700177c312> as *const (), 0), // +0x2e8e50 sub_2e8e50
    Lifted::new(0x70009d4eb0, tpl_33::<0x700177c312> as *const (), 0), // +0x2e8eb0 sub_2e8eb0
    Lifted::new(0x70009d4ee0, tpl_418::<0x70009d3bb0, 0x70009d4efb> as *const (), 0), // +0x2e8ee0 sub_2e8ee0
    Lifted::new(0x70009d4f10, tpl_417::<0x70009d3bb0, 0x70009d4f2b> as *const (), 0), // +0x2e8f10 sub_2e8f10
    Lifted::new(0x70009d4f40, tpl_33::<0x700177c312> as *const (), 0), // +0x2e8f40 sub_2e8f40
    Lifted::new(0x70009d4f70, fn_70009d4f70 as *const (), 0), // +0x2e8f70 sub_2e8f70
    Lifted::new(0x70009d4fe0, tpl_33::<0x700177c312> as *const (), 0), // +0x2e8fe0 sub_2e8fe0
    Lifted::new(0x70009d5010, tpl_417::<0x70009d3bb0, 0x70009d502b> as *const (), 0), // +0x2e9010 sub_2e9010
    Lifted::new(0x70009d5040, tpl_33::<0x700177c312> as *const (), 0), // +0x2e9040 sub_2e9040
    Lifted::new(0x70009d5070, tpl_416::<0x70009d3bb0, 0x70009d508b> as *const (), 0), // +0x2e9070 sub_2e9070
    Lifted::new(0x70009d50a0, tpl_33::<0x700177c312> as *const (), 0), // +0x2e90a0 sub_2e90a0
    Lifted::new(0x70009d50d0, tpl_419::<0x70009d3bb0, 0x70009d50eb> as *const (), 0), // +0x2e90d0 sub_2e90d0
    Lifted::new(0x70009d5100, tpl_33::<0x700177c312> as *const (), 0), // +0x2e9100 sub_2e9100
    Lifted::new(0x70009d5130, tpl_420::<0x20, 0x70017d3d30, 0x8, 0x700075ca60, 0x700177c312> as *const (), 0), // +0x2e9130 sub_2e9130
    Lifted::new(0x70009d5190, tpl_418::<0x70009d3bb0, 0x70009d51ab> as *const (), 0), // +0x2e9190 sub_2e9190
    Lifted::new(0x70009d51c0, tpl_417::<0x70009d3bb0, 0x70009d51db> as *const (), 0), // +0x2e91c0 sub_2e91c0
    Lifted::new(0x70009d51f0, tpl_133::<0x70007450d0, 0x700177c312> as *const (), 0), // +0x2e91f0 sub_2e91f0
    Lifted::new(0x70009d5230, tpl_421::<0x10, 0x18, 0x20, 0x70009d3bb0, 0x70009d5260> as *const (), 0), // +0x2e9230 sub_2e9230
    Lifted::new(0x70009d5270, tpl_33::<0x700177c312> as *const (), 0), // +0x2e9270 sub_2e9270
    Lifted::new(0x70009d52a0, tpl_133::<0x70006edb20, 0x700177c312> as *const (), 0), // +0x2e92a0 sub_2e92a0
    Lifted::new(0x70009d52e0, tpl_33::<0x700177c312> as *const (), 0), // +0x2e92e0 sub_2e92e0
    Lifted::new(0x70009d5310, tpl_417::<0x70009d3bb0, 0x70009d532b> as *const (), 0), // +0x2e9310 sub_2e9310
    Lifted::new(0x70009d5340, tpl_417::<0x70009d3bb0, 0x70009d535b> as *const (), 0), // +0x2e9340 sub_2e9340
    Lifted::new(0x70009d5370, tpl_417::<0x70009d3bb0, 0x70009d538b> as *const (), 0), // +0x2e9370 sub_2e9370
    Lifted::new(0x70009d53a0, tpl_418::<0x70009d3bb0, 0x70009d53bb> as *const (), 0), // +0x2e93a0 sub_2e93a0
    Lifted::new(0x70009d53d0, tpl_418::<0x70009d3bb0, 0x70009d53eb> as *const (), 0), // +0x2e93d0 sub_2e93d0
    Lifted::new(0x70009d5400, tpl_33::<0x700177c312> as *const (), 0), // +0x2e9400 sub_2e9400
    Lifted::new(0x70009d5430, tpl_112::<0x7001d55090, 0x70009d543a> as *const (), 0), // +0x2e9430 sub_2e9430
    Lifted::new(0x70009d5440, tpl_112::<0x7001d55090, 0x70009d544a> as *const (), 0), // +0x2e9440 sub_2e9440
    Lifted::new(0x70009d5450, tpl_112::<0x7001d55090, 0x70009d545a> as *const (), 0), // +0x2e9450 sub_2e9450
    Lifted::new(0x70009d5460, tpl_112::<0x7001d55090, 0x70009d546a> as *const (), 0), // +0x2e9460 sub_2e9460
    Lifted::new(0x70009d5470, tpl_112::<0x7001d55090, 0x70009d547a> as *const (), 0), // +0x2e9470 sub_2e9470
    Lifted::new(0x70009d5480, tpl_112::<0x7001d55090, 0x70009d548a> as *const (), 0), // +0x2e9480 sub_2e9480
    Lifted::new(0x70009d5490, tpl_112::<0x7001d55090, 0x70009d549a> as *const (), 0), // +0x2e9490 sub_2e9490
    Lifted::new(0x70009d54a0, tpl_112::<0x7001d55090, 0x70009d54aa> as *const (), 0), // +0x2e94a0 sub_2e94a0
    Lifted::new(0x70009d54b0, tpl_112::<0x7001d55090, 0x70009d54ba> as *const (), 0), // +0x2e94b0 sub_2e94b0
    Lifted::new(0x70009d54c0, tpl_112::<0x7001d55090, 0x70009d54ca> as *const (), 0), // +0x2e94c0 sub_2e94c0
    Lifted::new(0x70009d54d0, tpl_15 as *const (), 1), // +0x2e94d0 sub_2e94d0
    Lifted::new(0x70009d54e0, tpl_422::<0x14, 0x10, 0x30, 0x8, 0x38, 0x1c, 0x18, 0x34, 0xc, 0x40> as *const (), 1), // +0x2e94e0 sub_2e94e0
    Lifted::new(0x70009d5560, tpl_423::<0xc, 0x8, 0x18, 0x1, 0x10, 0x700076bf20, 0x70009fc7f0> as *const (), 0), // +0x2e9560 sub_2e9560
    Lifted::new(0x70009d5640, tpl_424::<0xc, 0x8, 0x1, 0x7000a25940> as *const (), 0), // +0x2e9640 sub_2e9640
    Lifted::new(0x70009d56c0, tpl_424::<0xc, 0x8, 0x1, 0x7000a259e0> as *const (), 0), // +0x2e96c0 sub_2e96c0
    Lifted::new(0x70009d5740, fn_70009d5740 as *const (), 2), // +0x2e9740 sub_2e9740
    Lifted::new(0x70009d5c00, tpl_425::<0x8, 0xc, 0x7000a25800, 0x700177c0b0> as *const (), 0), // +0x2e9c00 sub_2e9c00
    Lifted::new(0x70009d5c90, fn_70009d5c90 as *const (), 0), // +0x2e9c90 sub_2e9c90
    Lifted::new(0x70009d5d90, tpl_425::<0x8, 0xc, 0x7000a25940, 0x700177c0b0> as *const (), 0), // +0x2e9d90 sub_2e9d90
    Lifted::new(0x70009d5e20, tpl_426::<0xc, 0x8, 0x7000a259e0, 0x700177c0b0> as *const (), 0), // +0x2e9e20 sub_2e9e20
    Lifted::new(0x70009d5eb0, tpl_425::<0x8, 0xc, 0x7000a259e0, 0x700177c0b0> as *const (), 0), // +0x2e9eb0 sub_2e9eb0
    Lifted::new(0x70009d5f40, fn_70009d5f40 as *const (), 0), // +0x2e9f40 sub_2e9f40
    Lifted::new(0x70009d5f60, tpl_427::<0x700072d690> as *const (), 0), // +0x2e9f60 sub_2e9f60
    Lifted::new(0x70009d5f70, fn_70009d5f70 as *const (), 2), // +0x2e9f70 sub_2e9f70
    Lifted::new(0x70009d6050, fn_70009d6050 as *const (), 0), // +0x2ea050 sub_2ea050
    Lifted::new(0x70009d6310, fn_70009d6310 as *const (), 0), // +0x2ea310 sub_2ea310
    Lifted::new(0x70009d6510, fn_70009d6510 as *const (), 0), // +0x2ea510 sub_2ea510
    Lifted::new(0x70009d6610, fn_70009d6610 as *const (), 0), // +0x2ea610 sub_2ea610
    Lifted::new(0x70009d6660, tpl_428::<0x70009d8a90, 0x7000c44590> as *const (), 0), // +0x2ea660 sub_2ea660
    Lifted::new(0x70009d6680, tpl_429::<0x7000c44590> as *const (), 0), // +0x2ea680 sub_2ea680
    Lifted::new(0x70009d66a0, tpl_428::<0x70009d8a90, 0x7000c44600> as *const (), 0), // +0x2ea6a0 sub_2ea6a0
    Lifted::new(0x70009d66c0, tpl_429::<0x7000c44600> as *const (), 0), // +0x2ea6c0 sub_2ea6c0
    Lifted::new(0x70009d66e0, fn_70009d66e0 as *const (), 1), // +0x2ea6e0 sub_2ea6e0
    Lifted::new(0x70009d6700, fn_70009d6700 as *const (), 0), // +0x2ea700 sub_2ea700
    Lifted::new(0x70009d6750, fn_70009d6750 as *const (), 0), // +0x2ea750 sub_2ea750
    Lifted::new(0x70009d67e0, fn_70009d67e0 as *const (), 0), // +0x2ea7e0 sub_2ea7e0
    Lifted::new(0x70009d68c0, fn_70009d68c0 as *const (), 0), // +0x2ea8c0 sub_2ea8c0
    Lifted::new(0x70009d68e0, fn_70009d68e0 as *const (), 2), // +0x2ea8e0 sub_2ea8e0
    Lifted::new(0x70009d6b90, tpl_143::<0xc900, 0x700074b850> as *const (), 0), // +0x2eab90 sub_2eab90
    Lifted::new(0x70009d6ba0, tpl_430::<0x7000717f40> as *const (), 0), // +0x2eaba0 sub_2eaba0
    Lifted::new(0x70009d6bb0, fn_70009d6bb0 as *const (), 0), // +0x2eabb0 sub_2eabb0
    Lifted::new(0x70009d6c40, fn_70009d6c40 as *const (), 0), // +0x2eac40 sub_2eac40
    Lifted::new(0x70009d6d40, fn_70009d6d40 as *const (), 0), // +0x2ead40 sub_2ead40
    Lifted::new(0x70009d6d70, tpl_431::<0x7001d55060> as *const (), 0), // +0x2ead70 sub_2ead70
    Lifted::new(0x70009d6d80, fn_70009d6d80 as *const (), 0), // +0x2ead80 sub_2ead80
    Lifted::new(0x70009d6e90, fn_70009d6e90 as *const (), 0), // +0x2eae90 sub_2eae90
    Lifted::new(0x70009d6ef0, fn_70009d6ef0 as *const (), 0), // +0x2eaef0 sub_2eaef0
    Lifted::new(0x70009d6fa0, fn_70009d6fa0 as *const (), 0), // +0x2eafa0 sub_2eafa0
    Lifted::new(0x70009d7020, fn_70009d7020 as *const (), 1), // +0x2eb020 sub_2eb020
    Lifted::new(0x70009d7030, fn_70009d7030 as *const (), 3), // +0x2eb030 sub_2eb030
    Lifted::new(0x70009d7130, fn_70009d7130 as *const (), 1), // +0x2eb130 sub_2eb130
    Lifted::new(0x70009d7140, fn_70009d7140 as *const (), 2), // +0x2eb140 sub_2eb140
    Lifted::new(0x70009d72a0, fn_70009d72a0 as *const (), 2), // +0x2eb2a0 sub_2eb2a0
    Lifted::new(0x70009d7400, fn_70009d7400 as *const (), 1), // +0x2eb400 sub_2eb400
    Lifted::new(0x70009d7430, fn_70009d7430 as *const (), 1), // +0x2eb430 sub_2eb430
    Lifted::new(0x70009d7450, tpl_432::<0x7000a474e0> as *const (), 0), // +0x2eb450 sub_2eb450
    Lifted::new(0x70009d7460, tpl_20 as *const (), 1), // +0x2eb460 sub_2eb460
    Lifted::new(0x70009d7470, tpl_20 as *const (), 1), // +0x2eb470 sub_2eb470
    Lifted::new(0x70009d7480, tpl_430::<0x700072e8c0> as *const (), 0), // +0x2eb480 sub_2eb480
    Lifted::new(0x70009d7490, tpl_430::<0x700075d400> as *const (), 0), // +0x2eb490 sub_2eb490
    Lifted::new(0x70009d74a0, fn_70009d74a0 as *const (), 1), // +0x2eb4a0 sub_2eb4a0
    Lifted::new(0x70009d74c0, tpl_160::<0x10> as *const (), 1), // +0x2eb4c0 sub_2eb4c0
    Lifted::new(0x70009d74d0, fn_70009d74d0 as *const (), 0), // +0x2eb4d0 sub_2eb4d0
    Lifted::new(0x70009d75f0, tpl_433::<0x7000783200> as *const (), 0), // +0x2eb5f0 sub_2eb5f0
    Lifted::new(0x70009d7620, fn_70009d7620 as *const (), 0), // +0x2eb620 sub_2eb620
    Lifted::new(0x70009d77c0, tpl_434::<0xb, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x2eb7c0 sub_2eb7c0
    Lifted::new(0x70009d7870, fn_70009d7870 as *const (), 0), // +0x2eb870 sub_2eb870
    Lifted::new(0x70009d7a30, tpl_328::<0xb> as *const (), 1), // +0x2eba30 sub_2eba30
    Lifted::new(0x70009d7a40, tpl_435::<0xb, 0x2b, 0x17> as *const (), 1), // +0x2eba40 sub_2eba40
    Lifted::new(0x70009d7a80, tpl_36::<0x8> as *const (), 1), // +0x2eba80 sub_2eba80
    Lifted::new(0x70009d7a90, tpl_328::<0xb> as *const (), 1), // +0x2eba90 sub_2eba90
    Lifted::new(0x70009d7aa0, tpl_435::<0xb, 0x2b, 0xf> as *const (), 1), // +0x2ebaa0 sub_2ebaa0
    Lifted::new(0x70009d7ae0, fn_70009d7ae0 as *const (), 0), // +0x2ebae0 sub_2ebae0
    Lifted::new(0x70009d7b90, fn_70009d7b90 as *const (), 2), // +0x2ebb90 sub_2ebb90
    Lifted::new(0x70009d80e0, fn_70009d80e0 as *const (), 2), // +0x2ec0e0 sub_2ec0e0
    Lifted::new(0x70009d85c0, fn_70009d85c0 as *const (), 2), // +0x2ec5c0 sub_2ec5c0
    Lifted::new(0x70009d8a90, fn_70009d8a90 as *const (), 0), // +0x2eca90 sub_2eca90
    Lifted::new(0x70009d8ab0, tpl_325::<0x28> as *const (), 1), // +0x2ecab0 sub_2ecab0
    Lifted::new(0x70009d8ac0, fn_70009d8ac0 as *const (), 1), // +0x2ecac0 sub_2ecac0
    Lifted::new(0x70009d8b00, fn_70009d8b00 as *const (), 0), // +0x2ecb00 sub_2ecb00
    Lifted::new(0x70009d8ba0, fn_70009d8ba0 as *const (), 0), // +0x2ecba0 sub_2ecba0
    Lifted::new(0x70009d8bd0, fn_70009d8bd0 as *const (), 2), // +0x2ecbd0 sub_2ecbd0
    Lifted::new(0x70009d90a0, fn_70009d90a0 as *const (), 0), // +0x2ed0a0 sub_2ed0a0
    Lifted::new(0x70009d90d0, fn_70009d90d0 as *const (), 0), // +0x2ed0d0 sub_2ed0d0
    Lifted::new(0x70009d91f0, fn_70009d91f0 as *const (), 1), // +0x2ed1f0 sub_2ed1f0
    Lifted::new(0x70009d9220, tpl_15 as *const (), 1), // +0x2ed220 sub_2ed220
    Lifted::new(0x70009d9230, tpl_15 as *const (), 1), // +0x2ed230 sub_2ed230
    Lifted::new(0x70009d9240, tpl_436::<0xffffffffffffffff, 0x7, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d7178, 0x70017d71a0, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009d92d1> as *const (), 0), // +0x2ed240 sub_2ed240
    Lifted::new(0x70009d9300, fn_70009d9300 as *const (), 0), // +0x2ed300 sub_2ed300
    Lifted::new(0x70009d93c0, fn_70009d93c0 as *const (), 0), // +0x2ed3c0 sub_2ed3c0
    Lifted::new(0x70009d9490, fn_70009d9490 as *const (), 0), // +0x2ed490 sub_2ed490
    Lifted::new(0x70009d9550, fn_70009d9550 as *const (), 0), // +0x2ed550 sub_2ed550
    Lifted::new(0x70009d9630, fn_70009d9630 as *const (), 0), // +0x2ed630 sub_2ed630
    Lifted::new(0x70009d96f0, fn_70009d96f0 as *const (), 0), // +0x2ed6f0 sub_2ed6f0
    Lifted::new(0x70009d97d0, fn_70009d97d0 as *const (), 0), // +0x2ed7d0 sub_2ed7d0
    Lifted::new(0x70009d98b0, fn_70009d98b0 as *const (), 0), // +0x2ed8b0 sub_2ed8b0
    Lifted::new(0x70009d9980, fn_70009d9980 as *const (), 0), // +0x2ed980 sub_2ed980
    Lifted::new(0x70009d9a70, fn_70009d9a70 as *const (), 0), // +0x2eda70 sub_2eda70
    Lifted::new(0x70009d9b30, fn_70009d9b30 as *const (), 0), // +0x2edb30 sub_2edb30
    Lifted::new(0x70009d9bf0, fn_70009d9bf0 as *const (), 0), // +0x2edbf0 sub_2edbf0
    Lifted::new(0x70009d9cb0, fn_70009d9cb0 as *const (), 0), // +0x2edcb0 sub_2edcb0
    Lifted::new(0x70009d9da0, fn_70009d9da0 as *const (), 0), // +0x2edda0 sub_2edda0
    Lifted::new(0x70009d9e80, fn_70009d9e80 as *const (), 0), // +0x2ede80 sub_2ede80
    Lifted::new(0x70009d9f60, fn_70009d9f60 as *const (), 0), // +0x2edf60 sub_2edf60
    Lifted::new(0x70009da020, tpl_436::<0xffffffffffffffff, 0x7, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d70f8, 0x70017d7120, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009da0b1> as *const (), 0), // +0x2ee020 sub_2ee020
    Lifted::new(0x70009da0e0, fn_70009da0e0 as *const (), 0), // +0x2ee0e0 sub_2ee0e0
    Lifted::new(0x70009da1c0, tpl_437::<0x7000716790, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d5c98, 0x70017d5cc8, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009da234> as *const (), 0), // +0x2ee1c0 sub_2ee1c0
    Lifted::new(0x70009da260, fn_70009da260 as *const (), 0), // +0x2ee260 sub_2ee260
    Lifted::new(0x70009da340, fn_70009da340 as *const (), 0), // +0x2ee340 sub_2ee340
    Lifted::new(0x70009da420, tpl_438::<0xffffffffffffffff, 0x7, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d5c60, 0x70017d5c80, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009da4b5> as *const (), 0), // +0x2ee420 sub_2ee420
    Lifted::new(0x70009da4e0, fn_70009da4e0 as *const (), 0), // +0x2ee4e0 sub_2ee4e0
    Lifted::new(0x70009da5a0, fn_70009da5a0 as *const (), 0), // +0x2ee5a0 sub_2ee5a0
    Lifted::new(0x70009da660, fn_70009da660 as *const (), 0), // +0x2ee660 sub_2ee660
    Lifted::new(0x70009da720, fn_70009da720 as *const (), 0), // +0x2ee720 sub_2ee720
    Lifted::new(0x70009da7e0, tpl_438::<0xffffffffffffffff, 0x7, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d5c28, 0x70017d5c48, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009da875> as *const (), 0), // +0x2ee7e0 sub_2ee7e0
    Lifted::new(0x70009da8a0, fn_70009da8a0 as *const (), 0), // +0x2ee8a0 sub_2ee8a0
    Lifted::new(0x70009da990, fn_70009da990 as *const (), 0), // +0x2ee990 sub_2ee990
    Lifted::new(0x70009daa50, fn_70009daa50 as *const (), 0), // +0x2eea50 sub_2eea50
    Lifted::new(0x70009dab10, fn_70009dab10 as *const (), 0), // +0x2eeb10 sub_2eeb10
    Lifted::new(0x70009dabe0, tpl_439::<0xffffffffffffffff, 0x7, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d5e20, 0x70017d5e40, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009dac75> as *const (), 0), // +0x2eebe0 sub_2eebe0
    Lifted::new(0x70009daca0, fn_70009daca0 as *const (), 0), // +0x2eeca0 sub_2eeca0
    Lifted::new(0x70009dad60, fn_70009dad60 as *const (), 0), // +0x2eed60 sub_2eed60
    Lifted::new(0x70009dae20, tpl_439::<0xffffffffffffffff, 0x7, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d5e58, 0x70017d5e80, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009daeb5> as *const (), 0), // +0x2eee20 sub_2eee20
    Lifted::new(0x70009daee0, fn_70009daee0 as *const (), 0), // +0x2eeee0 sub_2eeee0
    Lifted::new(0x70009dafa0, fn_70009dafa0 as *const (), 0), // +0x2eefa0 sub_2eefa0
    Lifted::new(0x70009db070, tpl_436::<0xffffffffffffffff, 0x7, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d7140, 0x70017d7160, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009db101> as *const (), 0), // +0x2ef070 sub_2ef070
    Lifted::new(0x70009db130, fn_70009db130 as *const (), 0), // +0x2ef130 sub_2ef130
    Lifted::new(0x70009db1f0, fn_70009db1f0 as *const (), 0), // +0x2ef1f0 sub_2ef1f0
    Lifted::new(0x70009db2d0, fn_70009db2d0 as *const (), 0), // +0x2ef2d0 sub_2ef2d0
    Lifted::new(0x70009db390, fn_70009db390 as *const (), 0), // +0x2ef390 sub_2ef390
    Lifted::new(0x70009db470, fn_70009db470 as *const (), 0), // +0x2ef470 sub_2ef470
    Lifted::new(0x70009db530, fn_70009db530 as *const (), 0), // +0x2ef530 sub_2ef530
    Lifted::new(0x70009db610, tpl_437::<0x70007031c0, 0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d5ce0, 0x70017d5d10, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009db684> as *const (), 0), // +0x2ef610 sub_2ef610
    Lifted::new(0x70009db6b0, fn_70009db6b0 as *const (), 0), // +0x2ef6b0 sub_2ef6b0
    Lifted::new(0x70009db790, fn_70009db790 as *const (), 0), // +0x2ef790 sub_2ef790
    Lifted::new(0x70009db870, fn_70009db870 as *const (), 0), // +0x2ef870 sub_2ef870
    Lifted::new(0x70009db950, fn_70009db950 as *const (), 0), // +0x2ef950 sub_2ef950
    Lifted::new(0x70009dba50, fn_70009dba50 as *const (), 0), // +0x2efa50 sub_2efa50
    Lifted::new(0x70009dbb10, fn_70009dbb10 as *const (), 0), // +0x2efb10 sub_2efb10
    Lifted::new(0x70009dbbd0, fn_70009dbbd0 as *const (), 0), // +0x2efbd0 sub_2efbd0
    Lifted::new(0x70009dbc30, fn_70009dbc30 as *const (), 0), // +0x2efc30 sub_2efc30
    Lifted::new(0x70009dbce0, fn_70009dbce0 as *const (), 0), // +0x2efce0 sub_2efce0
    Lifted::new(0x70009dbd80, fn_70009dbd80 as *const (), 2), // +0x2efd80 sub_2efd80
    Lifted::new(0x70009dbf10, fn_70009dbf10 as *const (), 0), // +0x2eff10 sub_2eff10
    Lifted::new(0x70009dbfb0, fn_70009dbfb0 as *const (), 0), // +0x2effb0 sub_2effb0
    Lifted::new(0x70009dc020, fn_70009dc020 as *const (), 0), // +0x2f0020 sub_2f0020
    Lifted::new(0x70009dc0a0, fn_70009dc0a0 as *const (), 0), // +0x2f00a0 sub_2f00a0
    Lifted::new(0x70009dc140, tpl_15 as *const (), 1), // +0x2f0140 sub_2f0140
    Lifted::new(0x70009dc150, fn_70009dc150 as *const (), 0), // +0x2f0150 sub_2f0150
    Lifted::new(0x70009dc1f0, fn_70009dc1f0 as *const (), 0), // +0x2f01f0 sub_2f01f0
    Lifted::new(0x70009dc230, fn_70009dc230 as *const (), 0), // +0x2f0230 sub_2f0230
    Lifted::new(0x70009dc2e0, fn_70009dc2e0 as *const (), 0), // +0x2f02e0 sub_2f02e0
    Lifted::new(0x70009dc360, fn_70009dc360 as *const (), 0), // +0x2f0360 sub_2f0360
    Lifted::new(0x70009dc3b0, fn_70009dc3b0 as *const (), 2), // +0x2f03b0 sub_2f03b0
    Lifted::new(0x70009dc670, fn_70009dc670 as *const (), 0), // +0x2f0670 sub_2f0670
    Lifted::new(0x70009dc6d0, fn_70009dc6d0 as *const (), 0), // +0x2f06d0 sub_2f06d0
    Lifted::new(0x70009dc8f0, fn_70009dc8f0 as *const (), 0), // +0x2f08f0 sub_2f08f0
    Lifted::new(0x70009dca60, fn_70009dca60 as *const (), 0), // +0x2f0a60 sub_2f0a60
    Lifted::new(0x70009dcb50, fn_70009dcb50 as *const (), 2), // +0x2f0b50 sub_2f0b50
    Lifted::new(0x70009dcc60, fn_70009dcc60 as *const (), 2), // +0x2f0c60 sub_2f0c60
    Lifted::new(0x70009dcd70, fn_70009dcd70 as *const (), 0), // +0x2f0d70 sub_2f0d70
    Lifted::new(0x70009dcdd0, fn_70009dcdd0 as *const (), 0), // +0x2f0dd0 sub_2f0dd0
    Lifted::new(0x70009dce50, tpl_15 as *const (), 1), // +0x2f0e50 sub_2f0e50
    Lifted::new(0x70009dce60, tpl_16::<0x70009dce63, 0x70009dce64, 0x70009dce65, 0x70009dce66, 0x70009dce67, 0x70009dce68, 0x70009dce69, 0x70009dce6a, 0x70009dce6b, 0x70009dce6c, 0x70009dce6d, 0x70009dce6e, 0x70009dce6f, 0x7000783940> as *const (), 0), // +0x2f0e60 sub_2f0e60
    Lifted::new(0x70009dce80, fn_70009dce80 as *const (), 0), // +0x2f0e80 sub_2f0e80
    Lifted::new(0x70009dcfa0, fn_70009dcfa0 as *const (), 2), // +0x2f0fa0 sub_2f0fa0
    Lifted::new(0x70009dd050, fn_70009dd050 as *const (), 0), // +0x2f1050 sub_2f1050
    Lifted::new(0x70009dd190, fn_70009dd190 as *const (), 2), // +0x2f1190 sub_2f1190
    Lifted::new(0x70009dd7e0, fn_70009dd7e0 as *const (), 0), // +0x2f17e0 sub_2f17e0
    Lifted::new(0x70009dd810, fn_70009dd810 as *const (), 2), // +0x2f1810 sub_2f1810
    Lifted::new(0x70009de3f0, fn_70009de3f0 as *const (), 0), // +0x2f23f0 sub_2f23f0
    Lifted::new(0x70009de590, fn_70009de590 as *const (), 2), // +0x2f2590 sub_2f2590
    Lifted::new(0x70009dec30, fn_70009dec30 as *const (), 0), // +0x2f2c30 sub_2f2c30
    Lifted::new(0x70009ded10, fn_70009ded10 as *const (), 2), // +0x2f2d10 sub_2f2d10
    Lifted::new(0x70009df620, fn_70009df620 as *const (), 0), // +0x2f3620 sub_2f3620
    Lifted::new(0x70009df720, fn_70009df720 as *const (), 2), // +0x2f3720 sub_2f3720
    Lifted::new(0x70009df890, fn_70009df890 as *const (), 0), // +0x2f3890 sub_2f3890
    Lifted::new(0x70009df910, fn_70009df910 as *const (), 0), // +0x2f3910 sub_2f3910
    Lifted::new(0x70009dfa00, tpl_143::<0xf8, 0x700070f540> as *const (), 0), // +0x2f3a00 sub_2f3a00
    Lifted::new(0x70009dfa10, fn_70009dfa10 as *const (), 1), // +0x2f3a10 sub_2f3a10
    Lifted::new(0x70009dfa50, fn_70009dfa50 as *const (), 2), // +0x2f3a50 sub_2f3a50
    Lifted::new(0x70009dfca0, fn_70009dfca0 as *const (), 0), // +0x2f3ca0 sub_2f3ca0
    Lifted::new(0x70009dfe00, fn_70009dfe00 as *const (), 1), // +0x2f3e00 sub_2f3e00
    Lifted::new(0x70009dfe30, fn_70009dfe30 as *const (), 0), // +0x2f3e30 sub_2f3e30
    Lifted::new(0x70009dff90, fn_70009dff90 as *const (), 0), // +0x2f3f90 sub_2f3f90
    Lifted::new(0x70009e0170, fn_70009e0170 as *const (), 0), // +0x2f4170 sub_2f4170
    Lifted::new(0x70009e0290, fn_70009e0290 as *const (), 2), // +0x2f4290 sub_2f4290
    Lifted::new(0x70009e0340, fn_70009e0340 as *const (), 0), // +0x2f4340 sub_2f4340
    Lifted::new(0x70009e04b0, fn_70009e04b0 as *const (), 0), // +0x2f44b0 sub_2f44b0
    Lifted::new(0x70009e04f0, tpl_440::<0x328> as *const (), 1), // +0x2f44f0 sub_2f44f0
    Lifted::new(0x70009e0500, fn_70009e0500 as *const (), 2), // +0x2f4500 sub_2f4500
    Lifted::new(0x70009e13d0, fn_70009e13d0 as *const (), 0), // +0x2f53d0 sub_2f53d0
    Lifted::new(0x70009e1510, fn_70009e1510 as *const (), 2), // +0x2f5510 sub_2f5510
    Lifted::new(0x70009e1960, fn_70009e1960 as *const (), 2), // +0x2f5960 sub_2f5960
    Lifted::new(0x70009e1db0, fn_70009e1db0 as *const (), 0), // +0x2f5db0 sub_2f5db0
    Lifted::new(0x70009e1e00, fn_70009e1e00 as *const (), 0), // +0x2f5e00 sub_2f5e00
    Lifted::new(0x70009e1f60, fn_70009e1f60 as *const (), 2), // +0x2f5f60 sub_2f5f60
    Lifted::new(0x70009e2200, fn_70009e2200 as *const (), 0), // +0x2f6200 sub_2f6200
    Lifted::new(0x70009e2250, fn_70009e2250 as *const (), 0), // +0x2f6250 sub_2f6250
    Lifted::new(0x70009e22a0, tpl_74::<0x8, 0x10> as *const (), 1), // +0x2f62a0 sub_2f62a0
    Lifted::new(0x70009e22c0, fn_70009e22c0 as *const (), 1), // +0x2f62c0 sub_2f62c0
    Lifted::new(0x70009e22d0, fn_70009e22d0 as *const (), 0), // +0x2f62d0 sub_2f62d0
    Lifted::new(0x70009e2340, tpl_30 as *const (), 1), // +0x2f6340 sub_2f6340
    Lifted::new(0x70009e2350, fn_70009e2350 as *const (), 2), // +0x2f6350 sub_2f6350
    Lifted::new(0x70009e2450, fn_70009e2450 as *const (), 1), // +0x2f6450 sub_2f6450
    Lifted::new(0x70009e2490, fn_70009e2490 as *const (), 0), // +0x2f6490 sub_2f6490
    Lifted::new(0x70009e26a0, fn_70009e26a0 as *const (), 0), // +0x2f66a0 sub_2f66a0
    Lifted::new(0x70009e2740, tpl_441::<0x7000737b40> as *const (), 0), // +0x2f6740 sub_2f6740
    Lifted::new(0x70009e2750, fn_70009e2750 as *const (), 0), // +0x2f6750 sub_2f6750
    Lifted::new(0x70009e2880, tpl_143::<0xc0, 0x70009e2490> as *const (), 0), // +0x2f6880 sub_2f6880
    Lifted::new(0x70009e2890, fn_70009e2890 as *const (), 0), // +0x2f6890 sub_2f6890
    Lifted::new(0x70009e2b90, tpl_111 as *const (), 1), // +0x2f6b90 sub_2f6b90
    Lifted::new(0x70009e2ba0, fn_70009e2ba0 as *const (), 1), // +0x2f6ba0 sub_2f6ba0
    Lifted::new(0x70009e2bc0, fn_70009e2bc0 as *const (), 1), // +0x2f6bc0 sub_2f6bc0
    Lifted::new(0x70009e2bd0, fn_70009e2bd0 as *const (), 0), // +0x2f6bd0 sub_2f6bd0
    Lifted::new(0x70009e2ce0, tpl_15 as *const (), 1), // +0x2f6ce0 sub_2f6ce0
    Lifted::new(0x70009e2cf0, tpl_144 as *const (), 1), // +0x2f6cf0 sub_2f6cf0
    Lifted::new(0x70009e2d00, fn_70009e2d00 as *const (), 2), // +0x2f6d00 sub_2f6d00
    Lifted::new(0x70009e30e0, fn_70009e30e0 as *const (), 2), // +0x2f70e0 sub_2f70e0
    Lifted::new(0x70009e3320, tpl_442::<0xa0, 0x700073aaf0> as *const (), 0), // +0x2f7320 sub_2f7320
    Lifted::new(0x70009e3340, fn_70009e3340 as *const (), 0), // +0x2f7340 sub_2f7340
    Lifted::new(0x70009e33c0, fn_70009e33c0 as *const (), 2), // +0x2f73c0 sub_2f73c0
    Lifted::new(0x70009e36d0, fn_70009e36d0 as *const (), 2), // +0x2f76d0 sub_2f76d0
    Lifted::new(0x70009e3940, fn_70009e3940 as *const (), 2), // +0x2f7940 sub_2f7940
    Lifted::new(0x70009e3d30, fn_70009e3d30 as *const (), 2), // +0x2f7d30 sub_2f7d30
    Lifted::new(0x70009e4050, fn_70009e4050 as *const (), 2), // +0x2f8050 sub_2f8050
    Lifted::new(0x70009e42c0, fn_70009e42c0 as *const (), 0), // +0x2f82c0 sub_2f82c0
    Lifted::new(0x70009e4360, fn_70009e4360 as *const (), 0), // +0x2f8360 sub_2f8360
    Lifted::new(0x70009e43d0, fn_70009e43d0 as *const (), 2), // +0x2f83d0 sub_2f83d0
    Lifted::new(0x70009e4570, tpl_430::<0x70006f4a10> as *const (), 0), // +0x2f8570 sub_2f8570
    Lifted::new(0x70009e4580, fn_70009e4580 as *const (), 0), // +0x2f8580 sub_2f8580
    Lifted::new(0x70009e45e0, fn_70009e45e0 as *const (), 0), // +0x2f85e0 sub_2f85e0
    Lifted::new(0x70009e4700, tpl_441::<0x70007a4a00> as *const (), 0), // +0x2f8700 sub_2f8700
    Lifted::new(0x70009e4710, fn_70009e4710 as *const (), 0), // +0x2f8710 sub_2f8710
    Lifted::new(0x70009e4720, tpl_432::<0x700070d0c0> as *const (), 0), // +0x2f8720 sub_2f8720
    Lifted::new(0x70009e4730, tpl_17::<0x700070b9b0> as *const (), 0), // +0x2f8730 sub_2f8730
    Lifted::new(0x70009e4740, tpl_143::<0x8, 0x7000da4b60> as *const (), 0), // +0x2f8740 sub_2f8740
    Lifted::new(0x70009e4750, fn_70009e4750 as *const (), 0), // +0x2f8750 sub_2f8750
    Lifted::new(0x70009e4770, tpl_443::<0x18> as *const (), 0), // +0x2f8770 sub_2f8770
    Lifted::new(0x70009e4780, fn_70009e4780 as *const (), 0), // +0x2f8780 sub_2f8780
    Lifted::new(0x70009e47b0, tpl_15 as *const (), 1), // +0x2f87b0 sub_2f87b0
    Lifted::new(0x70009e47c0, fn_70009e47c0 as *const (), 0), // +0x2f87c0 sub_2f87c0
    Lifted::new(0x70009e4860, fn_70009e4860 as *const (), 0), // +0x2f8860 sub_2f8860
    Lifted::new(0x70009e48a0, fn_70009e48a0 as *const (), 0), // +0x2f88a0 sub_2f88a0
    Lifted::new(0x70009e4b20, fn_70009e4b20 as *const (), 2), // +0x2f8b20 sub_2f8b20
    Lifted::new(0x70009e4c50, fn_70009e4c50 as *const (), 2), // +0x2f8c50 sub_2f8c50
    Lifted::new(0x70009e4da0, fn_70009e4da0 as *const (), 0), // +0x2f8da0 sub_2f8da0
    Lifted::new(0x70009e5060, tpl_15 as *const (), 1), // +0x2f9060 sub_2f9060
    Lifted::new(0x70009e5070, tpl_444::<0x60, 0x28, 0x120> as *const (), 1), // +0x2f9070 sub_2f9070
    Lifted::new(0x70009e5090, tpl_442::<0x8, 0x7000da4a30> as *const (), 0), // +0x2f9090 sub_2f9090
    Lifted::new(0x70009e50a0, fn_70009e50a0 as *const (), 0), // +0x2f90a0 sub_2f90a0
    Lifted::new(0x70009e50c0, fn_70009e50c0 as *const (), 0), // +0x2f90c0 sub_2f90c0
    Lifted::new(0x70009e5210, fn_70009e5210 as *const (), 1), // +0x2f9210 sub_2f9210
    Lifted::new(0x70009e5240, fn_70009e5240 as *const (), 0), // +0x2f9240 sub_2f9240
    Lifted::new(0x70009e5260, fn_70009e5260 as *const (), 0), // +0x2f9260 sub_2f9260
    Lifted::new(0x70009e5350, fn_70009e5350 as *const (), 0), // +0x2f9350 sub_2f9350
    Lifted::new(0x70009e5390, fn_70009e5390 as *const (), 0), // +0x2f9390 sub_2f9390
    Lifted::new(0x70009e5550, fn_70009e5550 as *const (), 0), // +0x2f9550 sub_2f9550
    Lifted::new(0x70009e5620, fn_70009e5620 as *const (), 0), // +0x2f9620 sub_2f9620
    Lifted::new(0x70009e56c0, fn_70009e56c0 as *const (), 0), // +0x2f96c0 sub_2f96c0
    Lifted::new(0x70009e5870, fn_70009e5870 as *const (), 2), // +0x2f9870 sub_2f9870
    Lifted::new(0x70009e5ae0, fn_70009e5ae0 as *const (), 0), // +0x2f9ae0 sub_2f9ae0
    Lifted::new(0x70009e5ba0, fn_70009e5ba0 as *const (), 0), // +0x2f9ba0 sub_2f9ba0
    Lifted::new(0x70009e5cc0, fn_70009e5cc0 as *const (), 2), // +0x2f9cc0 sub_2f9cc0
    Lifted::new(0x70009e5f20, tpl_146::<0x2b> as *const (), 1), // +0x2f9f20 sub_2f9f20
    Lifted::new(0x70009e5f30, fn_70009e5f30 as *const (), 0), // +0x2f9f30 sub_2f9f30
    Lifted::new(0x70009e6020, fn_70009e6020 as *const (), 0), // +0x2fa020 sub_2fa020
    Lifted::new(0x70009e6090, fn_70009e6090 as *const (), 2), // +0x2fa090 sub_2fa090
    Lifted::new(0x70009e6620, fn_70009e6620 as *const (), 0), // +0x2fa620 sub_2fa620
    Lifted::new(0x70009e66f0, fn_70009e66f0 as *const (), 2), // +0x2fa6f0 sub_2fa6f0
    Lifted::new(0x70009e6b20, fn_70009e6b20 as *const (), 0), // +0x2fab20 sub_2fab20
    Lifted::new(0x70009e6c50, tpl_445::<0xf8, 0x700070f540, 0x7000709910> as *const (), 0), // +0x2fac50 sub_2fac50
    Lifted::new(0x70009e6ca0, tpl_446::<0xf8, 0x700070f540, 0x7000709910, 0x60, 0x700075f490, 0x10, 0x38, 0x18, 0x1c, 0xfffffffffffffffe, 0x7000706220> as *const (), 0), // +0x2faca0 sub_2faca0
    Lifted::new(0x70009e6dc0, tpl_447::<0xf8, 0x18, 0x38, 0x700070f540, 0x4c, 0x10, 0x30, 0x700073c730, 0x82, 0x100, 0x70007a5440, 0x7000709910> as *const (), 0), // +0x2fadc0 sub_2fadc0
    Lifted::new(0x70009e6ea0, fn_70009e6ea0 as *const (), 0), // +0x2faea0 sub_2faea0
    Lifted::new(0x70009e6f00, fn_70009e6f00 as *const (), 0), // +0x2faf00 sub_2faf00
    Lifted::new(0x70009e6fb0, tpl_36::<0x88> as *const (), 1), // +0x2fafb0 sub_2fafb0
    Lifted::new(0x70009e6fc0, fn_70009e6fc0 as *const (), 0), // +0x2fafc0 sub_2fafc0
    Lifted::new(0x70009e7300, fn_70009e7300 as *const (), 0), // +0x2fb300 sub_2fb300
    Lifted::new(0x70009e7310, tpl_36::<0x8> as *const (), 1), // +0x2fb310 sub_2fb310
    Lifted::new(0x70009e7320, fn_70009e7320 as *const (), 0), // +0x2fb320 sub_2fb320
    Lifted::new(0x70009e7360, fn_70009e7360 as *const (), 0), // +0x2fb360 sub_2fb360
    Lifted::new(0x70009e7570, fn_70009e7570 as *const (), 0), // +0x2fb570 sub_2fb570
    Lifted::new(0x70009e75b0, tpl_15 as *const (), 1), // +0x2fb5b0 sub_2fb5b0
    Lifted::new(0x70009e75c0, fn_70009e75c0 as *const (), 0), // +0x2fb5c0 sub_2fb5c0
    Lifted::new(0x70009e77a0, fn_70009e77a0 as *const (), 0), // +0x2fb7a0 sub_2fb7a0
    Lifted::new(0x70009e77f0, fn_70009e77f0 as *const (), 0), // +0x2fb7f0 sub_2fb7f0
    Lifted::new(0x70009e7840, fn_70009e7840 as *const (), 0), // +0x2fb840 sub_2fb840
    Lifted::new(0x70009e7890, tpl_448::<0x79> as *const (), 1), // +0x2fb890 sub_2fb890
    Lifted::new(0x70009e78a0, fn_70009e78a0 as *const (), 1), // +0x2fb8a0 sub_2fb8a0
    Lifted::new(0x70009e78d0, fn_70009e78d0 as *const (), 0), // +0x2fb8d0 sub_2fb8d0
    Lifted::new(0x70009e7920, tpl_47::<0x128> as *const (), 1), // +0x2fb920 sub_2fb920
    Lifted::new(0x70009e7930, tpl_431::<0x7001d54ef0> as *const (), 0), // +0x2fb930 sub_2fb930
    Lifted::new(0x70009e7940, tpl_431::<0x7001d54ef0> as *const (), 0), // +0x2fb940 sub_2fb940
    Lifted::new(0x70009e7950, fn_70009e7950 as *const (), 0), // +0x2fb950 sub_2fb950
    Lifted::new(0x70009e79f0, fn_70009e79f0 as *const (), 0), // +0x2fb9f0 sub_2fb9f0
    Lifted::new(0x70009e7aa0, tpl_449::<0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d4660, 0x70017d4680, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x70009e7b04> as *const (), 0), // +0x2fbaa0 sub_2fbaa0
    Lifted::new(0x70009e7b20, fn_70009e7b20 as *const (), 2), // +0x2fbb20 sub_2fbb20
    Lifted::new(0x70009e7bf0, fn_70009e7bf0 as *const (), 1), // +0x2fbbf0 sub_2fbbf0
    Lifted::new(0x70009e7c40, fn_70009e7c40 as *const (), 1), // +0x2fbc40 sub_2fbc40
    Lifted::new(0x70009e7c80, fn_70009e7c80 as *const (), 1), // +0x2fbc80 sub_2fbc80
    Lifted::new(0x70009e7cd0, tpl_30 as *const (), 1), // +0x2fbcd0 sub_2fbcd0
    Lifted::new(0x70009e7ce0, fn_70009e7ce0 as *const (), 1), // +0x2fbce0 sub_2fbce0
    Lifted::new(0x70009e7d10, fn_70009e7d10 as *const (), 1), // +0x2fbd10 sub_2fbd10
    Lifted::new(0x70009e7d40, fn_70009e7d40 as *const (), 2), // +0x2fbd40 sub_2fbd40
    Lifted::new(0x70009e81f0, fn_70009e81f0 as *const (), 2), // +0x2fc1f0 sub_2fc1f0
    Lifted::new(0x70009e8500, fn_70009e8500 as *const (), 0), // +0x2fc500 sub_2fc500
    Lifted::new(0x70009e8560, tpl_15 as *const (), 1), // +0x2fc560 sub_2fc560
    Lifted::new(0x70009e8570, tpl_15 as *const (), 1), // +0x2fc570 sub_2fc570
    Lifted::new(0x70009e8580, fn_70009e8580 as *const (), 0), // +0x2fc580 sub_2fc580
    Lifted::new(0x70009e8670, fn_70009e8670 as *const (), 2), // +0x2fc670 sub_2fc670
    Lifted::new(0x70009e86b0, fn_70009e86b0 as *const (), 2), // +0x2fc6b0 sub_2fc6b0
    Lifted::new(0x70009e8b70, fn_70009e8b70 as *const (), 0), // +0x2fcb70 sub_2fcb70
    Lifted::new(0x70009e8de0, fn_70009e8de0 as *const (), 2), // +0x2fcde0 sub_2fcde0
    Lifted::new(0x70009e9240, fn_70009e9240 as *const (), 2), // +0x2fd240 sub_2fd240
    Lifted::new(0x70009e9660, fn_70009e9660 as *const (), 0), // +0x2fd660 sub_2fd660
    Lifted::new(0x70009e97f0, fn_70009e97f0 as *const (), 1), // +0x2fd7f0 sub_2fd7f0
    Lifted::new(0x70009e9810, tpl_36::<0xd868> as *const (), 1), // +0x2fd810 sub_2fd810
    Lifted::new(0x70009e9820, tpl_37::<0xcb0> as *const (), 1), // +0x2fd820 sub_2fd820
    Lifted::new(0x70009e9840, fn_70009e9840 as *const (), 0), // +0x2fd840 sub_2fd840
    Lifted::new(0x70009e99a0, fn_70009e99a0 as *const (), 0), // +0x2fd9a0 sub_2fd9a0
    Lifted::new(0x70009e9b00, fn_70009e9b00 as *const (), 1), // +0x2fdb00 sub_2fdb00
    Lifted::new(0x70009e9b20, fn_70009e9b20 as *const (), 0), // +0x2fdb20 sub_2fdb20
    Lifted::new(0x70009e9c40, tpl_450::<0x7001c67534> as *const (), 1), // +0x2fdc40 sub_2fdc40
    Lifted::new(0x70009e9c50, tpl_451::<0x68, 0x60, 0x7000a36ee0, 0x70009e9c79> as *const (), 0), // +0x2fdc50 sub_2fdc50
    Lifted::new(0x70009e9c90, fn_70009e9c90 as *const (), 1), // +0x2fdc90 sub_2fdc90
    Lifted::new(0x70009e9d00, tpl_452::<0x68, 0x60> as *const (), 1), // +0x2fdd00 sub_2fdd00
    Lifted::new(0x70009e9d10, fn_70009e9d10 as *const (), 1), // +0x2fdd10 sub_2fdd10
    Lifted::new(0x70009e9db0, tpl_17::<0x7000737ca0> as *const (), 0), // +0x2fddb0 sub_2fddb0
    Lifted::new(0x70009e9dc0, fn_70009e9dc0 as *const (), 0), // +0x2fddc0 sub_2fddc0
    Lifted::new(0x70009e9f10, fn_70009e9f10 as *const (), 1), // +0x2fdf10 sub_2fdf10
    Lifted::new(0x70009e9f60, tpl_36::<0x10> as *const (), 1), // +0x2fdf60 sub_2fdf60
    Lifted::new(0x70009e9f70, tpl_36::<0x8> as *const (), 1), // +0x2fdf70 sub_2fdf70
    Lifted::new(0x70009e9f80, tpl_453::<0x30, 0x70009e9f84, 0x70009e9f85, 0x70009e9f86, 0x70009e9f87, 0x70009e9f88, 0x70009e9f89, 0x70009e9f8a, 0x70009e9f8b, 0x70009e9f8c, 0x70009e9f8d, 0x70009e9f8e, 0x70009e9f8f, 0x7000f1a020> as *const (), 0), // +0x2fdf80 sub_2fdf80
    Lifted::new(0x70009e9fa0, tpl_360::<0x8, 0x24> as *const (), 1), // +0x2fdfa0 sub_2fdfa0
    Lifted::new(0x70009e9fb0, tpl_36::<0x38> as *const (), 1), // +0x2fdfb0 sub_2fdfb0
    Lifted::new(0x70009e9fc0, fn_70009e9fc0 as *const (), 0), // +0x2fdfc0 sub_2fdfc0
    Lifted::new(0x70009ea140, tpl_143::<0x20, 0x7000781580> as *const (), 0), // +0x2fe140 sub_2fe140
    Lifted::new(0x70009ea150, fn_70009ea150 as *const (), 0), // +0x2fe150 sub_2fe150
    Lifted::new(0x70009ea180, fn_70009ea180 as *const (), 0), // +0x2fe180 sub_2fe180
    Lifted::new(0x70009ea240, tpl_454::<0x70007343a0> as *const (), 0), // +0x2fe240 sub_2fe240
    Lifted::new(0x70009ea270, fn_70009ea270 as *const (), 0), // +0x2fe270 sub_2fe270
    Lifted::new(0x70009ea340, fn_70009ea340 as *const (), 0), // +0x2fe340 sub_2fe340
    Lifted::new(0x70009ea410, tpl_36::<0xa0c8> as *const (), 1), // +0x2fe410 sub_2fe410
    Lifted::new(0x70009ea420, fn_70009ea420 as *const (), 0), // +0x2fe420 sub_2fe420
    Lifted::new(0x70009ea480, tpl_162::<0x7001c71e94, 0x7001d55088> as *const (), 0), // +0x2fe480 sub_2fe480
    Lifted::new(0x70009ea490, fn_70009ea490 as *const (), 0), // +0x2fe490 sub_2fe490
    Lifted::new(0x70009ea590, fn_70009ea590 as *const (), 0), // +0x2fe590 sub_2fe590
    Lifted::new(0x70009ea5c0, fn_70009ea5c0 as *const (), 0), // +0x2fe5c0 sub_2fe5c0
    Lifted::new(0x70009ea610, fn_70009ea610 as *const (), 0), // +0x2fe610 sub_2fe610
    Lifted::new(0x70009ea630, fn_70009ea630 as *const (), 2), // +0x2fe630 sub_2fe630
    Lifted::new(0x70009ea670, fn_70009ea670 as *const (), 0), // +0x2fe670 sub_2fe670
    Lifted::new(0x70009ea790, tpl_17::<0x70007a3b40> as *const (), 0), // +0x2fe790 sub_2fe790
    Lifted::new(0x70009ea7a0, fn_70009ea7a0 as *const (), 1), // +0x2fe7a0 sub_2fe7a0
    Lifted::new(0x70009ea7d0, fn_70009ea7d0 as *const (), 1), // +0x2fe7d0 sub_2fe7d0
    Lifted::new(0x70009ea7f0, tpl_52::<0xc8> as *const (), 1), // +0x2fe7f0 sub_2fe7f0
    Lifted::new(0x70009ea800, fn_70009ea800 as *const (), 0), // +0x2fe800 sub_2fe800
    Lifted::new(0x70009ea860, tpl_455::<0x7001c67540> as *const (), 1), // +0x2fe860 sub_2fe860
    Lifted::new(0x70009ea870, tpl_430::<0x7000c4fe00> as *const (), 0), // +0x2fe870 sub_2fe870
    Lifted::new(0x70009ea880, tpl_456::<0x7000758fd0, 0x4> as *const (), 0), // +0x2fe880 sub_2fe880
    Lifted::new(0x70009ea8b0, fn_70009ea8b0 as *const (), 0), // +0x2fe8b0 sub_2fe8b0
    Lifted::new(0x70009eaa90, fn_70009eaa90 as *const (), 0), // +0x2fea90 sub_2fea90
    Lifted::new(0x70009eac50, tpl_457::<0x38, 0x7001d54d08> as *const (), 0), // +0x2fec50 sub_2fec50
    Lifted::new(0x70009eac80, fn_70009eac80 as *const (), 0), // +0x2fec80 sub_2fec80
    Lifted::new(0x70009ead50, fn_70009ead50 as *const (), 0), // +0x2fed50 sub_2fed50
    Lifted::new(0x70009eaea0, fn_70009eaea0 as *const (), 1), // +0x2feea0 sub_2feea0
    Lifted::new(0x70009eaf30, fn_70009eaf30 as *const (), 1), // +0x2fef30 sub_2fef30
    Lifted::new(0x70009eafe0, fn_70009eafe0 as *const (), 1), // +0x2fefe0 sub_2fefe0
    Lifted::new(0x70009eb050, tpl_458::<0x113, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ff050 sub_2ff050
    Lifted::new(0x70009eb110, fn_70009eb110 as *const (), 1), // +0x2ff110 sub_2ff110
    Lifted::new(0x70009eb130, fn_70009eb130 as *const (), 1), // +0x2ff130 sub_2ff130
    Lifted::new(0x70009eb140, fn_70009eb140 as *const (), 0), // +0x2ff140 sub_2ff140
    Lifted::new(0x70009eb1f0, tpl_146::<0x3> as *const (), 1), // +0x2ff1f0 sub_2ff1f0
    Lifted::new(0x70009eb200, fn_70009eb200 as *const (), 1), // +0x2ff200 sub_2ff200
    Lifted::new(0x70009eb240, fn_70009eb240 as *const (), 2), // +0x2ff240 sub_2ff240
    Lifted::new(0x70009eb660, tpl_191::<0x18> as *const (), 1), // +0x2ff660 sub_2ff660
    Lifted::new(0x70009eb670, tpl_459::<0x8, 0x10, 0x20, 0x70006f24a0> as *const (), 0), // +0x2ff670 sub_2ff670
    Lifted::new(0x70009eb6e0, fn_70009eb6e0 as *const (), 0), // +0x2ff6e0 sub_2ff6e0
    Lifted::new(0x70009eb760, tpl_275::<0x8, 0x10> as *const (), 1), // +0x2ff760 sub_2ff760
    Lifted::new(0x70009eb770, tpl_434::<0xb, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ff770 sub_2ff770
    Lifted::new(0x70009eb820, tpl_37::<0xd08> as *const (), 1), // +0x2ff820 sub_2ff820
    Lifted::new(0x70009eb840, fn_70009eb840 as *const (), 0), // +0x2ff840 sub_2ff840
    Lifted::new(0x70009eb8c0, fn_70009eb8c0 as *const (), 0), // +0x2ff8c0 sub_2ff8c0
    Lifted::new(0x70009eb990, fn_70009eb990 as *const (), 0), // +0x2ff990 sub_2ff990
    Lifted::new(0x70009eb9b0, fn_70009eb9b0 as *const (), 0), // +0x2ff9b0 sub_2ff9b0
    Lifted::new(0x70009ebab0, fn_70009ebab0 as *const (), 0), // +0x2ffab0 sub_2ffab0
    Lifted::new(0x70009ebc20, tpl_39::<0x10> as *const (), 1), // +0x2ffc20 sub_2ffc20
    Lifted::new(0x70009ebc30, tpl_39::<0x98> as *const (), 1), // +0x2ffc30 sub_2ffc30
    Lifted::new(0x70009ebc40, tpl_39::<0x98> as *const (), 1), // +0x2ffc40 sub_2ffc40
    Lifted::new(0x70009ebc50, tpl_460::<0x10, 0x3080, 0x3b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x2ffc50 sub_2ffc50
    Lifted::new(0x70009ebd30, tpl_39::<0x28> as *const (), 1), // +0x2ffd30 sub_2ffd30
    Lifted::new(0x70009ebd40, fn_70009ebd40 as *const (), 2), // +0x2ffd40 sub_2ffd40
    Lifted::new(0x70009ebe60, tpl_461::<0x60, 0xf8, 0x700075f490> as *const (), 0), // +0x2ffe60 sub_2ffe60
    Lifted::new(0x70009ebe80, tpl_146::<0x7> as *const (), 1), // +0x2ffe80 sub_2ffe80
    Lifted::new(0x70009ebe90, fn_70009ebe90 as *const (), 0), // +0x2ffe90 sub_2ffe90
    Lifted::new(0x70009ebed0, fn_70009ebed0 as *const (), 0), // +0x2ffed0 sub_2ffed0
    Lifted::new(0x70009ebf80, tpl_434::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x2fff80 sub_2fff80
    Lifted::new(0x70009ec030, fn_70009ec030 as *const (), 0), // +0x300030 sub_300030
    Lifted::new(0x70009ec060, fn_70009ec060 as *const (), 0), // +0x300060 sub_300060
    Lifted::new(0x70009ec190, tpl_37::<0xd10> as *const (), 1), // +0x300190 sub_300190
    Lifted::new(0x70009ec1b0, tpl_462::<0x10> as *const (), 1), // +0x3001b0 sub_3001b0
    Lifted::new(0x70009ec1d0, tpl_462::<0x10> as *const (), 1), // +0x3001d0 sub_3001d0
    Lifted::new(0x70009ec1f0, tpl_462::<0x10> as *const (), 1), // +0x3001f0 sub_3001f0
    Lifted::new(0x70009ec210, tpl_328::<0x8> as *const (), 1), // +0x300210 sub_300210
    Lifted::new(0x70009ec220, tpl_37::<0xcb8> as *const (), 1), // +0x300220 sub_300220
    Lifted::new(0x70009ec240, fn_70009ec240 as *const (), 0), // +0x300240 sub_300240
    Lifted::new(0x70009ec3a0, tpl_39::<0x10c> as *const (), 1), // +0x3003a0 sub_3003a0
    Lifted::new(0x70009ec3b0, tpl_463::<0x58, 0x8, 0x20> as *const (), 1), // +0x3003b0 sub_3003b0
    Lifted::new(0x70009ec3d0, fn_70009ec3d0 as *const (), 0), // +0x3003d0 sub_3003d0
    Lifted::new(0x70009ec5c0, fn_70009ec5c0 as *const (), 2), // +0x3005c0 sub_3005c0
    Lifted::new(0x70009ec7f0, tpl_450::<0x7001c6752c> as *const (), 1), // +0x3007f0 sub_3007f0
    Lifted::new(0x70009ec800, tpl_37::<0xcd0> as *const (), 1), // +0x300800 sub_300800
    Lifted::new(0x70009ec820, tpl_39::<0x1a0> as *const (), 1), // +0x300820 sub_300820
    Lifted::new(0x70009ec830, fn_70009ec830 as *const (), 0), // +0x300830 sub_300830
    Lifted::new(0x70009ec900, tpl_360::<0xc900, 0x48> as *const (), 1), // +0x300900 sub_300900
    Lifted::new(0x70009ec910, tpl_450::<0x7001c67530> as *const (), 1), // +0x300910 sub_300910
    Lifted::new(0x70009ec920, fn_70009ec920 as *const (), 0), // +0x300920 sub_300920
    Lifted::new(0x70009eca10, fn_70009eca10 as *const (), 0), // +0x300a10 sub_300a10
    Lifted::new(0x70009ece20, fn_70009ece20 as *const (), 2), // +0x300e20 sub_300e20
    Lifted::new(0x70009ed320, fn_70009ed320 as *const (), 0), // +0x301320 sub_301320
    Lifted::new(0x70009ed510, fn_70009ed510 as *const (), 0), // +0x301510 sub_301510
    Lifted::new(0x70009ed640, fn_70009ed640 as *const (), 0), // +0x301640 sub_301640
    Lifted::new(0x70009ed7e0, fn_70009ed7e0 as *const (), 0), // +0x3017e0 sub_3017e0
    Lifted::new(0x70009ed910, fn_70009ed910 as *const (), 0), // +0x301910 sub_301910
    Lifted::new(0x70009ed990, tpl_143::<0x20, 0x70007a6a50> as *const (), 0), // +0x301990 sub_301990
    Lifted::new(0x70009ed9a0, tpl_143::<0x20, 0x7000788eb0> as *const (), 0), // +0x3019a0 sub_3019a0
    Lifted::new(0x70009ed9b0, tpl_143::<0x20, 0x7000754b50> as *const (), 0), // +0x3019b0 sub_3019b0
    Lifted::new(0x70009ed9c0, tpl_464::<0x10c> as *const (), 1), // +0x3019c0 sub_3019c0
    Lifted::new(0x70009ed9e0, tpl_461::<0x60, 0xf8, 0x700072add0> as *const (), 0), // +0x3019e0 sub_3019e0
    Lifted::new(0x70009eda00, fn_70009eda00 as *const (), 0), // +0x301a00 sub_301a00
    Lifted::new(0x70009eda70, tpl_39::<0x80> as *const (), 1), // +0x301a70 sub_301a70
    Lifted::new(0x70009eda80, tpl_39::<0x100> as *const (), 1), // +0x301a80 sub_301a80
    Lifted::new(0x70009eda90, fn_70009eda90 as *const (), 0), // +0x301a90 sub_301a90
    Lifted::new(0x70009edb40, fn_70009edb40 as *const (), 0), // +0x301b40 sub_301b40
    Lifted::new(0x70009edc50, fn_70009edc50 as *const (), 2), // +0x301c50 sub_301c50
    Lifted::new(0x70009ee0c0, fn_70009ee0c0 as *const (), 0), // +0x3020c0 sub_3020c0
    Lifted::new(0x70009ee100, fn_70009ee100 as *const (), 0), // +0x302100 sub_302100
    Lifted::new(0x70009ee140, tpl_36::<0x78> as *const (), 1), // +0x302140 sub_302140
    Lifted::new(0x70009ee150, tpl_450::<0x7001c67538> as *const (), 1), // +0x302150 sub_302150
    Lifted::new(0x70009ee160, tpl_36::<0x40> as *const (), 1), // +0x302160 sub_302160
    Lifted::new(0x70009ee170, tpl_216::<0x70006f9350> as *const (), 0), // +0x302170 sub_302170
    Lifted::new(0x70009ee190, tpl_465 as *const (), 1), // +0x302190 sub_302190
    Lifted::new(0x70009ee1a0, tpl_42 as *const (), 1), // +0x3021a0 sub_3021a0
    Lifted::new(0x70009ee1b0, fn_70009ee1b0 as *const (), 2), // +0x3021b0 sub_3021b0
    Lifted::new(0x70009ee500, fn_70009ee500 as *const (), 0), // +0x302500 sub_302500
    Lifted::new(0x70009ee550, fn_70009ee550 as *const (), 2), // +0x302550 sub_302550
    Lifted::new(0x70009eea40, fn_70009eea40 as *const (), 0), // +0x302a40 sub_302a40
    Lifted::new(0x70009eeb50, tpl_191::<0x8> as *const (), 1), // +0x302b50 sub_302b50
    Lifted::new(0x70009eeb60, tpl_191::<0x8> as *const (), 1), // +0x302b60 sub_302b60
    Lifted::new(0x70009eeb70, fn_70009eeb70 as *const (), 0), // +0x302b70 sub_302b70
    Lifted::new(0x70009eebc0, fn_70009eebc0 as *const (), 1), // +0x302bc0 sub_302bc0
    Lifted::new(0x70009eebe0, tpl_191::<0x8> as *const (), 1), // +0x302be0 sub_302be0
    Lifted::new(0x70009eebf0, fn_70009eebf0 as *const (), 1), // +0x302bf0 sub_302bf0
    Lifted::new(0x70009eec10, fn_70009eec10 as *const (), 0), // +0x302c10 sub_302c10
    Lifted::new(0x70009eece0, fn_70009eece0 as *const (), 0), // +0x302ce0 sub_302ce0
    Lifted::new(0x70009eed80, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x70009eed91> as *const (), 0), // +0x302d80 sub_302d80
    Lifted::new(0x70009eeda0, fn_70009eeda0 as *const (), 0), // +0x302da0 sub_302da0
    Lifted::new(0x70009eee40, fn_70009eee40 as *const (), 0), // +0x302e40 sub_302e40
    Lifted::new(0x70009eeee0, fn_70009eeee0 as *const (), 1), // +0x302ee0 sub_302ee0
    Lifted::new(0x70009eeef0, fn_70009eeef0 as *const (), 1), // +0x302ef0 sub_302ef0
    Lifted::new(0x70009eef10, tpl_111 as *const (), 1), // +0x302f10 sub_302f10
    Lifted::new(0x70009eef20, fn_70009eef20 as *const (), 2), // +0x302f20 sub_302f20
    Lifted::new(0x70009ef370, fn_70009ef370 as *const (), 2), // +0x303370 sub_303370
    Lifted::new(0x70009ef620, fn_70009ef620 as *const (), 2), // +0x303620 sub_303620
    Lifted::new(0x70009ef9f0, fn_70009ef9f0 as *const (), 2), // +0x3039f0 sub_3039f0
    Lifted::new(0x70009eff20, tpl_17::<0x70007491b0> as *const (), 0), // +0x303f20 sub_303f20
    Lifted::new(0x70009eff30, tpl_37::<0xcd8> as *const (), 1), // +0x303f30 sub_303f30
    Lifted::new(0x70009eff50, tpl_460::<0x10, 0x3080, 0x7, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8> as *const (), 0), // +0x303f50 sub_303f50
    Lifted::new(0x70009f0030, tpl_36::<0x8> as *const (), 1), // +0x304030 sub_304030
    Lifted::new(0x70009f0040, fn_70009f0040 as *const (), 1), // +0x304040 sub_304040
    Lifted::new(0x70009f0070, fn_70009f0070 as *const (), 0), // +0x304070 sub_304070
    Lifted::new(0x70009f00c0, tpl_39::<0x60> as *const (), 1), // +0x3040c0 sub_3040c0
    Lifted::new(0x70009f00d0, tpl_39::<0x2c> as *const (), 1), // +0x3040d0 sub_3040d0
    Lifted::new(0x70009f00e0, tpl_466::<0x70006f2a70, 0x70006f9a60, 0x7000783fa0, 0x7000eda850> as *const (), 0), // +0x3040e0 sub_3040e0
    Lifted::new(0x70009f02b0, tpl_360::<0x8, 0x28> as *const (), 1), // +0x3042b0 sub_3042b0
    Lifted::new(0x70009f02c0, tpl_433::<0x7000f1b290> as *const (), 0), // +0x3042c0 sub_3042c0
    Lifted::new(0x70009f02f0, tpl_39::<0x28> as *const (), 1), // +0x3042f0 sub_3042f0
    Lifted::new(0x70009f0300, tpl_466::<0x70006f2a70, 0x70006f9a60, 0x7000783fa0, 0x7000709e70> as *const (), 0), // +0x304300 sub_304300
    Lifted::new(0x70009f04d0, tpl_191::<0x20> as *const (), 1), // +0x3044d0 sub_3044d0
    Lifted::new(0x70009f04e0, tpl_467::<0x10, 0x7000f1b2d0, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0xffffffffffffffff, 0x7> as *const (), 0), // +0x3044e0 sub_3044e0
    Lifted::new(0x70009f05f0, fn_70009f05f0 as *const (), 2), // +0x3045f0 sub_3045f0
    Lifted::new(0x70009f0800, tpl_467::<0x10, 0x7000f1b360, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0xffffffffffffffff, 0x7> as *const (), 0), // +0x304800 sub_304800
    Lifted::new(0x70009f0910, fn_70009f0910 as *const (), 0), // +0x304910 sub_304910
    Lifted::new(0x70009f0b10, fn_70009f0b10 as *const (), 0), // +0x304b10 sub_304b10
    Lifted::new(0x70009f0c00, fn_70009f0c00 as *const (), 0), // +0x304c00 sub_304c00
    Lifted::new(0x70009f0e00, tpl_459::<0x8, 0x18, 0x20, 0x70006f24a0> as *const (), 0), // +0x304e00 sub_304e00
    Lifted::new(0x70009f0e70, fn_70009f0e70 as *const (), 0), // +0x304e70 sub_304e70
    Lifted::new(0x70009f0ea0, tpl_275::<0x8, 0x18> as *const (), 1), // +0x304ea0 sub_304ea0
    Lifted::new(0x70009f0eb0, tpl_467::<0x10, 0x7000f1b3e0, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0xffffffffffffffff, 0x7> as *const (), 0), // +0x304eb0 sub_304eb0
    Lifted::new(0x70009f0fc0, tpl_467::<0x10, 0x7000f1b4f0, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8, 0xffffffffffffffff, 0x7> as *const (), 0), // +0x304fc0 sub_304fc0
    Lifted::new(0x70009f10d0, tpl_37::<0xce0> as *const (), 1), // +0x3050d0 sub_3050d0
    Lifted::new(0x70009f10f0, tpl_458::<0x2ab, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x3050f0 sub_3050f0
    Lifted::new(0x70009f11b0, fn_70009f11b0 as *const (), 0), // +0x3051b0 sub_3051b0
    Lifted::new(0x70009f1310, fn_70009f1310 as *const (), 1), // +0x305310 sub_305310
    Lifted::new(0x70009f1330, tpl_468::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30, 0x7000774720> as *const (), 0), // +0x305330 sub_305330
    Lifted::new(0x70009f1450, tpl_469::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30, 0x7000774720> as *const (), 0), // +0x305450 sub_305450
    Lifted::new(0x70009f1590, fn_70009f1590 as *const (), 0), // +0x305590 sub_305590
    Lifted::new(0x70009f1860, tpl_434::<0xf, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x305860 sub_305860
    Lifted::new(0x70009f1910, fn_70009f1910 as *const (), 0), // +0x305910 sub_305910
    Lifted::new(0x70009f1ba0, fn_70009f1ba0 as *const (), 2), // +0x305ba0 sub_305ba0
    Lifted::new(0x70009f1db0, fn_70009f1db0 as *const (), 0), // +0x305db0 sub_305db0
    Lifted::new(0x70009f1dd0, fn_70009f1dd0 as *const (), 2), // +0x305dd0 sub_305dd0
    Lifted::new(0x70009f1fe0, tpl_434::<0x3, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x305fe0 sub_305fe0
    Lifted::new(0x70009f2090, tpl_37::<0xcf0> as *const (), 1), // +0x306090 sub_306090
    Lifted::new(0x70009f20b0, fn_70009f20b0 as *const (), 0), // +0x3060b0 sub_3060b0
    Lifted::new(0x70009f21d0, fn_70009f21d0 as *const (), 0), // +0x3061d0 sub_3061d0
    Lifted::new(0x70009f22a0, fn_70009f22a0 as *const (), 0), // +0x3062a0 sub_3062a0
    Lifted::new(0x70009f25b0, tpl_456::<0x70006eeb20, 0x4> as *const (), 0), // +0x3065b0 sub_3065b0
    Lifted::new(0x70009f25e0, fn_70009f25e0 as *const (), 0), // +0x3065e0 sub_3065e0
    Lifted::new(0x70009f27b0, fn_70009f27b0 as *const (), 0), // +0x3067b0 sub_3067b0
    Lifted::new(0x70009f2970, tpl_457::<0x30, 0x7001d54d08> as *const (), 0), // +0x306970 sub_306970
    Lifted::new(0x70009f29a0, fn_70009f29a0 as *const (), 0), // +0x3069a0 sub_3069a0
    Lifted::new(0x70009f2a60, tpl_461::<0x60, 0xf8, 0x700075f490> as *const (), 0), // +0x306a60 sub_306a60
    Lifted::new(0x70009f2a80, tpl_434::<0x7, 0x10, 0xf88, 0x70006f2a70, 0xf70, 0xf78, 0xffffffffffff6898, 0x70006f9a60, 0x8> as *const (), 0), // +0x306a80 sub_306a80
    Lifted::new(0x70009f2b30, fn_70009f2b30 as *const (), 0), // +0x306b30 sub_306b30
    Lifted::new(0x70009f2ba0, tpl_36::<0x8> as *const (), 1), // +0x306ba0 sub_306ba0
    Lifted::new(0x70009f2bb0, tpl_37::<0xcf8> as *const (), 1), // +0x306bb0 sub_306bb0
    Lifted::new(0x70009f2bd0, tpl_37::<0xd18> as *const (), 1), // +0x306bd0 sub_306bd0
    Lifted::new(0x70009f2bf0, tpl_36::<0x98> as *const (), 1), // +0x306bf0 sub_306bf0
    Lifted::new(0x70009f2c00, tpl_470 as *const (), 1), // +0x306c00 sub_306c00
    Lifted::new(0x70009f2c10, fn_70009f2c10 as *const (), 1), // +0x306c10 sub_306c10
    Lifted::new(0x70009f2c20, fn_70009f2c20 as *const (), 0), // +0x306c20 sub_306c20
    Lifted::new(0x70009f2d80, fn_70009f2d80 as *const (), 0), // +0x306d80 sub_306d80
    Lifted::new(0x70009f2e40, tpl_37::<0xd00> as *const (), 1), // +0x306e40 sub_306e40
    Lifted::new(0x70009f2e60, fn_70009f2e60 as *const (), 1), // +0x306e60 sub_306e60
    Lifted::new(0x70009f2e90, tpl_455::<0x7001c67560> as *const (), 1), // +0x306e90 sub_306e90
    Lifted::new(0x70009f2ea0, tpl_455::<0x7001c67560> as *const (), 1), // +0x306ea0 sub_306ea0
    Lifted::new(0x70009f2eb0, fn_70009f2eb0 as *const (), 0), // +0x306eb0 sub_306eb0
    Lifted::new(0x70009f3130, tpl_468::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30, 0x7000774720> as *const (), 0), // +0x307130 sub_307130
    Lifted::new(0x70009f3250, fn_70009f3250 as *const (), 1), // +0x307250 sub_307250
    Lifted::new(0x70009f3260, fn_70009f3260 as *const (), 0), // +0x307260 sub_307260
    Lifted::new(0x70009f3330, fn_70009f3330 as *const (), 0), // +0x307330 sub_307330
    Lifted::new(0x70009f3370, fn_70009f3370 as *const (), 1), // +0x307370 sub_307370
    Lifted::new(0x70009f33c0, fn_70009f33c0 as *const (), 1), // +0x3073c0 sub_3073c0
    Lifted::new(0x70009f3410, fn_70009f3410 as *const (), 0), // +0x307410 sub_307410
    Lifted::new(0x70009f3610, fn_70009f3610 as *const (), 2), // +0x307610 sub_307610
    Lifted::new(0x70009f3700, fn_70009f3700 as *const (), 2), // +0x307700 sub_307700
    Lifted::new(0x70009f3820, fn_70009f3820 as *const (), 0), // +0x307820 sub_307820
    Lifted::new(0x70009f3870, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x70009f3881> as *const (), 0), // +0x307870 sub_307870
    Lifted::new(0x70009f3890, tpl_15 as *const (), 1), // +0x307890 sub_307890
    Lifted::new(0x70009f38a0, fn_70009f38a0 as *const (), 2), // +0x3078a0 sub_3078a0
    Lifted::new(0x70009f3bb0, fn_70009f3bb0 as *const (), 2), // +0x307bb0 sub_307bb0
    Lifted::new(0x70009f3e60, fn_70009f3e60 as *const (), 2), // +0x307e60 sub_307e60
    Lifted::new(0x70009f4200, fn_70009f4200 as *const (), 2), // +0x308200 sub_308200
    Lifted::new(0x70009f4510, fn_70009f4510 as *const (), 1), // +0x308510 sub_308510
    Lifted::new(0x70009f4560, fn_70009f4560 as *const (), 1), // +0x308560 sub_308560
    Lifted::new(0x70009f4580, fn_70009f4580 as *const (), 0), // +0x308580 sub_308580
    Lifted::new(0x70009f45b0, fn_70009f45b0 as *const (), 1), // +0x3085b0 sub_3085b0
    Lifted::new(0x70009f45d0, fn_70009f45d0 as *const (), 2), // +0x3085d0 sub_3085d0
    Lifted::new(0x70009f4700, fn_70009f4700 as *const (), 1), // +0x308700 sub_308700
    Lifted::new(0x70009f4750, fn_70009f4750 as *const (), 0), // +0x308750 sub_308750
    Lifted::new(0x70009f48c0, fn_70009f48c0 as *const (), 0), // +0x3088c0 sub_3088c0
    Lifted::new(0x70009f48f0, tpl_471::<0x18, 0x4c> as *const (), 1), // +0x3088f0 sub_3088f0
    Lifted::new(0x70009f4900, fn_70009f4900 as *const (), 1), // +0x308900 sub_308900
    Lifted::new(0x70009f4950, fn_70009f4950 as *const (), 2), // +0x308950 sub_308950
    Lifted::new(0x70009f4c90, fn_70009f4c90 as *const (), 2), // +0x308c90 sub_308c90
    Lifted::new(0x70009f4ef0, tpl_472::<0x700076b0b0, 0x7000748aa0> as *const (), 0), // +0x308ef0 sub_308ef0
    Lifted::new(0x70009f4f20, tpl_216::<0x700074b8e0> as *const (), 0), // +0x308f20 sub_308f20
    Lifted::new(0x70009f4f40, fn_70009f4f40 as *const (), 0), // +0x308f40 sub_308f40
    Lifted::new(0x70009f4ff0, tpl_473 as *const (), 1), // +0x308ff0 sub_308ff0
    Lifted::new(0x70009f5000, tpl_474 as *const (), 1), // +0x309000 sub_309000
    Lifted::new(0x70009f5010, fn_70009f5010 as *const (), 1), // +0x309010 sub_309010
    Lifted::new(0x70009f5030, fn_70009f5030 as *const (), 1), // +0x309030 sub_309030
    Lifted::new(0x70009f5040, tpl_473 as *const (), 1), // +0x309040 sub_309040
    Lifted::new(0x70009f5050, tpl_474 as *const (), 1), // +0x309050 sub_309050
    Lifted::new(0x70009f5060, fn_70009f5060 as *const (), 1), // +0x309060 sub_309060
    Lifted::new(0x70009f5080, fn_70009f5080 as *const (), 1), // +0x309080 sub_309080
    Lifted::new(0x70009f5090, tpl_144 as *const (), 1), // +0x309090 sub_309090
    Lifted::new(0x70009f50a0, tpl_475::<0xffffffffffffffff> as *const (), 1), // +0x3090a0 sub_3090a0
    Lifted::new(0x70009f50b0, fn_70009f50b0 as *const (), 1), // +0x3090b0 sub_3090b0
    Lifted::new(0x70009f50c0, fn_70009f50c0 as *const (), 2), // +0x3090c0 sub_3090c0
    Lifted::new(0x70009f5360, fn_70009f5360 as *const (), 2), // +0x309360 sub_309360
    Lifted::new(0x70009f5600, fn_70009f5600 as *const (), 2), // +0x309600 sub_309600
    Lifted::new(0x70009f58a0, tpl_335::<0x28> as *const (), 1), // +0x3098a0 sub_3098a0
    Lifted::new(0x70009f58b0, fn_70009f58b0 as *const (), 0), // +0x3098b0 sub_3098b0
    Lifted::new(0x70009f5960, fn_70009f5960 as *const (), 1), // +0x309960 sub_309960
    Lifted::new(0x70009f5970, fn_70009f5970 as *const (), 0), // +0x309970 sub_309970
    Lifted::new(0x70009f5990, tpl_195::<0x2fe8> as *const (), 1), // +0x309990 sub_309990
    Lifted::new(0x70009f59a0, tpl_172::<0x8> as *const (), 1), // +0x3099a0 sub_3099a0
    Lifted::new(0x70009f59b0, fn_70009f59b0 as *const (), 0), // +0x3099b0 sub_3099b0
    Lifted::new(0x70009f5a10, fn_70009f5a10 as *const (), 0), // +0x309a10 sub_309a10
    Lifted::new(0x70009f5a60, tpl_15 as *const (), 1), // +0x309a60 sub_309a60
    Lifted::new(0x70009f5a70, tpl_476::<0x10> as *const (), 1), // +0x309a70 sub_309a70
    Lifted::new(0x70009f5a80, tpl_202::<0x10> as *const (), 1), // +0x309a80 sub_309a80
    Lifted::new(0x70009f5a90, fn_70009f5a90 as *const (), 0), // +0x309a90 sub_309a90
    Lifted::new(0x70009f5ab0, fn_70009f5ab0 as *const (), 0), // +0x309ab0 sub_309ab0
    Lifted::new(0x70009f5d60, fn_70009f5d60 as *const (), 0), // +0x309d60 sub_309d60
    Lifted::new(0x70009f5e10, tpl_477::<0x7001d55060, 0x8, 0x70017d8de0, 0x7001d54ef8, 0x70009f5e43, 0x10, 0x18, 0xffffffffffffffe8, 0x14> as *const (), 0), // +0x309e10 sub_309e10
    Lifted::new(0x70009f5eb0, fn_70009f5eb0 as *const (), 0), // +0x309eb0 sub_309eb0
    Lifted::new(0x70009f5fb0, fn_70009f5fb0 as *const (), 0), // +0x309fb0 sub_309fb0
    Lifted::new(0x70009f62c0, fn_70009f62c0 as *const (), 0), // +0x30a2c0 sub_30a2c0
    Lifted::new(0x70009f6390, fn_70009f6390 as *const (), 0), // +0x30a390 sub_30a390
    Lifted::new(0x70009f63d0, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f64c9> as *const (), 2), // +0x30a3d0 sub_30a3d0
    Lifted::new(0x70009f6510, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f6609> as *const (), 2), // +0x30a510 sub_30a510
    Lifted::new(0x70009f6650, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f6749> as *const (), 2), // +0x30a650 sub_30a650
    Lifted::new(0x70009f6790, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f6889> as *const (), 2), // +0x30a790 sub_30a790
    Lifted::new(0x70009f68d0, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f69c9> as *const (), 2), // +0x30a8d0 sub_30a8d0
    Lifted::new(0x70009f6a10, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f6b09> as *const (), 2), // +0x30aa10 sub_30aa10
    Lifted::new(0x70009f6b50, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f6c49> as *const (), 2), // +0x30ab50 sub_30ab50
    Lifted::new(0x70009f6c90, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f6d89> as *const (), 2), // +0x30ac90 sub_30ac90
    Lifted::new(0x70009f6dd0, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f6ec9> as *const (), 2), // +0x30add0 sub_30add0
    Lifted::new(0x70009f6f10, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f7009> as *const (), 2), // +0x30af10 sub_30af10
    Lifted::new(0x70009f7050, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f7149> as *const (), 2), // +0x30b050 sub_30b050
    Lifted::new(0x70009f7190, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f7289> as *const (), 2), // +0x30b190 sub_30b190
    Lifted::new(0x70009f72d0, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f73c9> as *const (), 2), // +0x30b2d0 sub_30b2d0
    Lifted::new(0x70009f7410, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f7509> as *const (), 2), // +0x30b410 sub_30b410
    Lifted::new(0x70009f7550, tpl_478::<0x700076bf20, 0x7000a36860, 0x70009a43d0, 0x70009f7649> as *const (), 2), // +0x30b550 sub_30b550
    Lifted::new(0x70009f7690, fn_70009f7690 as *const (), 0), // +0x30b690 sub_30b690
    Lifted::new(0x70009f7740, fn_70009f7740 as *const (), 2), // +0x30b740 sub_30b740
    Lifted::new(0x70009f79a0, fn_70009f79a0 as *const (), 0), // +0x30b9a0 sub_30b9a0
    Lifted::new(0x70009f7bf0, fn_70009f7bf0 as *const (), 2), // +0x30bbf0 sub_30bbf0
    Lifted::new(0x70009f7e50, fn_70009f7e50 as *const (), 2), // +0x30be50 sub_30be50
    Lifted::new(0x70009f80b0, fn_70009f80b0 as *const (), 2), // +0x30c0b0 sub_30c0b0
    Lifted::new(0x70009f8400, tpl_433::<0x7000757d30> as *const (), 0), // +0x30c400 sub_30c400
    Lifted::new(0x70009f8430, fn_70009f8430 as *const (), 2), // +0x30c430 sub_30c430
    Lifted::new(0x70009f87d0, fn_70009f87d0 as *const (), 0), // +0x30c7d0 sub_30c7d0
    Lifted::new(0x70009f88b0, fn_70009f88b0 as *const (), 1), // +0x30c8b0 sub_30c8b0
    Lifted::new(0x70009f88d0, fn_70009f88d0 as *const (), 0), // +0x30c8d0 sub_30c8d0
    Lifted::new(0x70009f8a30, fn_70009f8a30 as *const (), 0), // +0x30ca30 sub_30ca30
    Lifted::new(0x70009f8ac0, tpl_479 as *const (), 1), // +0x30cac0 sub_30cac0
    Lifted::new(0x70009f8ad0, fn_70009f8ad0 as *const (), 1), // +0x30cad0 sub_30cad0
    Lifted::new(0x70009f8ae0, fn_70009f8ae0 as *const (), 2), // +0x30cae0 sub_30cae0
    Lifted::new(0x70009f8bd0, fn_70009f8bd0 as *const (), 2), // +0x30cbd0 sub_30cbd0
    Lifted::new(0x70009f8e30, fn_70009f8e30 as *const (), 2), // +0x30ce30 sub_30ce30
    Lifted::new(0x70009f8f10, tpl_480 as *const (), 1), // +0x30cf10 sub_30cf10
    Lifted::new(0x70009f8f20, fn_70009f8f20 as *const (), 1), // +0x30cf20 sub_30cf20
    Lifted::new(0x70009f8f30, tpl_481::<0x7001c79f74> as *const (), 1), // +0x30cf30 sub_30cf30
    Lifted::new(0x70009f8f40, tpl_433::<0x7000e8b840> as *const (), 0), // +0x30cf40 sub_30cf40
    Lifted::new(0x70009f8f70, tpl_482::<0x60, 0x28, 0x70> as *const (), 1), // +0x30cf70 sub_30cf70
    Lifted::new(0x70009f8f90, fn_70009f8f90 as *const (), 1), // +0x30cf90 sub_30cf90
    Lifted::new(0x70009f8fd0, fn_70009f8fd0 as *const (), 1), // +0x30cfd0 sub_30cfd0
    Lifted::new(0x70009f9010, fn_70009f9010 as *const (), 1), // +0x30d010 sub_30d010
    Lifted::new(0x70009f9060, fn_70009f9060 as *const (), 0), // +0x30d060 sub_30d060
    Lifted::new(0x70009f90c0, fn_70009f90c0 as *const (), 1), // +0x30d0c0 sub_30d0c0
    Lifted::new(0x70009f90d0, fn_70009f90d0 as *const (), 1), // +0x30d0d0 sub_30d0d0
    Lifted::new(0x70009f9140, fn_70009f9140 as *const (), 1), // +0x30d140 sub_30d140
    Lifted::new(0x70009f9150, tpl_482::<0x60, 0x28, 0x88> as *const (), 1), // +0x30d150 sub_30d150
    Lifted::new(0x70009f9170, fn_70009f9170 as *const (), 1), // +0x30d170 sub_30d170
    Lifted::new(0x70009f91d0, fn_70009f91d0 as *const (), 1), // +0x30d1d0 sub_30d1d0
    Lifted::new(0x70009f91e0, fn_70009f91e0 as *const (), 0), // +0x30d1e0 sub_30d1e0
    Lifted::new(0x70009f9250, fn_70009f9250 as *const (), 1), // +0x30d250 sub_30d250
    Lifted::new(0x70009f9290, fn_70009f9290 as *const (), 1), // +0x30d290 sub_30d290
    Lifted::new(0x70009f92f0, fn_70009f92f0 as *const (), 0), // +0x30d2f0 sub_30d2f0
    Lifted::new(0x70009f9360, fn_70009f9360 as *const (), 1), // +0x30d360 sub_30d360
    Lifted::new(0x70009f9370, fn_70009f9370 as *const (), 1), // +0x30d370 sub_30d370
    Lifted::new(0x70009f9390, fn_70009f9390 as *const (), 1), // +0x30d390 sub_30d390
    Lifted::new(0x70009f93a0, fn_70009f93a0 as *const (), 1), // +0x30d3a0 sub_30d3a0
    Lifted::new(0x70009f93f0, fn_70009f93f0 as *const (), 1), // +0x30d3f0 sub_30d3f0
    Lifted::new(0x70009f9460, fn_70009f9460 as *const (), 1), // +0x30d460 sub_30d460
    Lifted::new(0x70009f9490, fn_70009f9490 as *const (), 1), // +0x30d490 sub_30d490
    Lifted::new(0x70009f94a0, tpl_483::<0x7001c75304> as *const (), 1), // +0x30d4a0 sub_30d4a0
    Lifted::new(0x70009f94b0, fn_70009f94b0 as *const (), 1), // +0x30d4b0 sub_30d4b0
    Lifted::new(0x70009f94e0, fn_70009f94e0 as *const (), 2), // +0x30d4e0 sub_30d4e0
    Lifted::new(0x70009f9650, tpl_484::<0x1b> as *const (), 1), // +0x30d650 sub_30d650
    Lifted::new(0x70009f9660, fn_70009f9660 as *const (), 1), // +0x30d660 sub_30d660
    Lifted::new(0x70009f9670, fn_70009f9670 as *const (), 1), // +0x30d670 sub_30d670
    Lifted::new(0x70009f96a0, tpl_482::<0x60, 0x28, 0xb8> as *const (), 1), // +0x30d6a0 sub_30d6a0
    Lifted::new(0x70009f96c0, tpl_485::<0x13> as *const (), 1), // +0x30d6c0 sub_30d6c0
    Lifted::new(0x70009f96e0, fn_70009f96e0 as *const (), 1), // +0x30d6e0 sub_30d6e0
    Lifted::new(0x70009f9730, fn_70009f9730 as *const (), 1), // +0x30d730 sub_30d730
    Lifted::new(0x70009f9740, fn_70009f9740 as *const (), 1), // +0x30d740 sub_30d740
    Lifted::new(0x70009f9780, fn_70009f9780 as *const (), 1), // +0x30d780 sub_30d780
    Lifted::new(0x70009f97c0, tpl_47::<0xc7e2> as *const (), 1), // +0x30d7c0 sub_30d7c0
    Lifted::new(0x70009f97d0, tpl_475::<0xfffffffffffffffc> as *const (), 1), // +0x30d7d0 sub_30d7d0
    Lifted::new(0x70009f97e0, fn_70009f97e0 as *const (), 1), // +0x30d7e0 sub_30d7e0
    Lifted::new(0x70009f97f0, fn_70009f97f0 as *const (), 1), // +0x30d7f0 sub_30d7f0
    Lifted::new(0x70009f9800, tpl_482::<0x60, 0x28, 0x128> as *const (), 1), // +0x30d800 sub_30d800
    Lifted::new(0x70009f9820, tpl_433::<0x7000f1c170> as *const (), 0), // +0x30d820 sub_30d820
    Lifted::new(0x70009f9850, fn_70009f9850 as *const (), 1), // +0x30d850 sub_30d850
    Lifted::new(0x70009f9880, fn_70009f9880 as *const (), 1), // +0x30d880 sub_30d880
    Lifted::new(0x70009f98b0, fn_70009f98b0 as *const (), 1), // +0x30d8b0 sub_30d8b0
    Lifted::new(0x70009f98e0, fn_70009f98e0 as *const (), 0), // +0x30d8e0 sub_30d8e0
    Lifted::new(0x70009f9960, fn_70009f9960 as *const (), 1), // +0x30d960 sub_30d960
    Lifted::new(0x70009f9990, fn_70009f9990 as *const (), 1), // +0x30d990 sub_30d990
    Lifted::new(0x70009f99c0, fn_70009f99c0 as *const (), 1), // +0x30d9c0 sub_30d9c0
    Lifted::new(0x70009f9a20, fn_70009f9a20 as *const (), 0), // +0x30da20 sub_30da20
    Lifted::new(0x70009f9a90, fn_70009f9a90 as *const (), 0), // +0x30da90 sub_30da90
    Lifted::new(0x70009f9b00, fn_70009f9b00 as *const (), 1), // +0x30db00 sub_30db00
    Lifted::new(0x70009f9b40, fn_70009f9b40 as *const (), 1), // +0x30db40 sub_30db40
    Lifted::new(0x70009f9b80, fn_70009f9b80 as *const (), 1), // +0x30db80 sub_30db80
    Lifted::new(0x70009f9be0, fn_70009f9be0 as *const (), 1), // +0x30dbe0 sub_30dbe0
    Lifted::new(0x70009f9c30, fn_70009f9c30 as *const (), 0), // +0x30dc30 sub_30dc30
    Lifted::new(0x70009f9d00, tpl_144 as *const (), 1), // +0x30dd00 sub_30dd00
    Lifted::new(0x70009f9d10, fn_70009f9d10 as *const (), 0), // +0x30dd10 sub_30dd10
    Lifted::new(0x70009f9f10, fn_70009f9f10 as *const (), 1), // +0x30df10 sub_30df10
    Lifted::new(0x70009f9f40, tpl_486::<0x1b> as *const (), 1), // +0x30df40 sub_30df40
    Lifted::new(0x70009f9f50, tpl_172::<0xa590> as *const (), 1), // +0x30df50 sub_30df50
    Lifted::new(0x70009f9f60, fn_70009f9f60 as *const (), 0), // +0x30df60 sub_30df60
    Lifted::new(0x70009f9fd0, fn_70009f9fd0 as *const (), 3), // +0x30dfd0 sub_30dfd0
    Lifted::new(0x70009fa080, fn_70009fa080 as *const (), 0), // +0x30e080 sub_30e080
    Lifted::new(0x70009fa0f0, fn_70009fa0f0 as *const (), 0), // +0x30e0f0 sub_30e0f0
    Lifted::new(0x70009fa160, fn_70009fa160 as *const (), 0), // +0x30e160 sub_30e160
    Lifted::new(0x70009fa180, tpl_263::<0x8> as *const (), 1), // +0x30e180 sub_30e180
    Lifted::new(0x70009fa190, tpl_241::<0x7001c65d80> as *const (), 1), // +0x30e190 sub_30e190
    Lifted::new(0x70009fa1b0, tpl_487::<0x7000f2fcd0> as *const (), 0), // +0x30e1b0 sub_30e1b0
    Lifted::new(0x70009fa1e0, fn_70009fa1e0 as *const (), 1), // +0x30e1e0 sub_30e1e0
    Lifted::new(0x70009fa250, tpl_482::<0x60, 0x28, 0x158> as *const (), 1), // +0x30e250 sub_30e250
    Lifted::new(0x70009fa270, fn_70009fa270 as *const (), 1), // +0x30e270 sub_30e270
    Lifted::new(0x70009fa280, fn_70009fa280 as *const (), 1), // +0x30e280 sub_30e280
    Lifted::new(0x70009fa290, fn_70009fa290 as *const (), 1), // +0x30e290 sub_30e290
    Lifted::new(0x70009fa2d0, fn_70009fa2d0 as *const (), 1), // +0x30e2d0 sub_30e2d0
    Lifted::new(0x70009fa320, fn_70009fa320 as *const (), 1), // +0x30e320 sub_30e320
    Lifted::new(0x70009fa330, fn_70009fa330 as *const (), 1), // +0x30e330 sub_30e330
    Lifted::new(0x70009fa380, fn_70009fa380 as *const (), 1), // +0x30e380 sub_30e380
    Lifted::new(0x70009fa3c0, tpl_482::<0x60, 0x28, 0x168> as *const (), 1), // +0x30e3c0 sub_30e3c0
    Lifted::new(0x70009fa3e0, fn_70009fa3e0 as *const (), 1), // +0x30e3e0 sub_30e3e0
    Lifted::new(0x70009fa420, fn_70009fa420 as *const (), 1), // +0x30e420 sub_30e420
    Lifted::new(0x70009fa460, fn_70009fa460 as *const (), 1), // +0x30e460 sub_30e460
    Lifted::new(0x70009fa4a0, fn_70009fa4a0 as *const (), 0), // +0x30e4a0 sub_30e4a0
    Lifted::new(0x70009fa5b0, fn_70009fa5b0 as *const (), 1), // +0x30e5b0 sub_30e5b0
    Lifted::new(0x70009fa5f0, fn_70009fa5f0 as *const (), 1), // +0x30e5f0 sub_30e5f0
    Lifted::new(0x70009fa660, fn_70009fa660 as *const (), 1), // +0x30e660 sub_30e660
    Lifted::new(0x70009fa6a0, fn_70009fa6a0 as *const (), 1), // +0x30e6a0 sub_30e6a0
    Lifted::new(0x70009fa6e0, fn_70009fa6e0 as *const (), 1), // +0x30e6e0 sub_30e6e0
    Lifted::new(0x70009fa710, fn_70009fa710 as *const (), 1), // +0x30e710 sub_30e710
    Lifted::new(0x70009fa770, fn_70009fa770 as *const (), 1), // +0x30e770 sub_30e770
    Lifted::new(0x70009fa780, tpl_193::<0x80> as *const (), 1), // +0x30e780 sub_30e780
    Lifted::new(0x70009fa790, tpl_144 as *const (), 1), // +0x30e790 sub_30e790
    Lifted::new(0x70009fa7a0, fn_70009fa7a0 as *const (), 1), // +0x30e7a0 sub_30e7a0
    Lifted::new(0x70009fa7e0, fn_70009fa7e0 as *const (), 0), // +0x30e7e0 sub_30e7e0
    Lifted::new(0x70009fa840, fn_70009fa840 as *const (), 1), // +0x30e840 sub_30e840
    Lifted::new(0x70009fa880, fn_70009fa880 as *const (), 1), // +0x30e880 sub_30e880
    Lifted::new(0x70009fa890, tpl_276::<0x7001c65d80> as *const (), 1), // +0x30e890 sub_30e890
    Lifted::new(0x70009fa8b0, fn_70009fa8b0 as *const (), 1), // +0x30e8b0 sub_30e8b0
    Lifted::new(0x70009fa8c0, fn_70009fa8c0 as *const (), 1), // +0x30e8c0 sub_30e8c0
    Lifted::new(0x70009fa900, fn_70009fa900 as *const (), 1), // +0x30e900 sub_30e900
    Lifted::new(0x70009fa940, fn_70009fa940 as *const (), 1), // +0x30e940 sub_30e940
    Lifted::new(0x70009fa980, tpl_44 as *const (), 1), // +0x30e980 sub_30e980
    Lifted::new(0x70009fa990, tpl_44 as *const (), 1), // +0x30e990 sub_30e990
    Lifted::new(0x70009fa9a0, tpl_471::<0xc900, 0x4c> as *const (), 1), // +0x30e9a0 sub_30e9a0
    Lifted::new(0x70009fa9b0, fn_70009fa9b0 as *const (), 1), // +0x30e9b0 sub_30e9b0
    Lifted::new(0x70009fa9c0, fn_70009fa9c0 as *const (), 1), // +0x30e9c0 sub_30e9c0
    Lifted::new(0x70009faa00, fn_70009faa00 as *const (), 1), // +0x30ea00 sub_30ea00
    Lifted::new(0x70009faa50, tpl_193::<0x30> as *const (), 1), // +0x30ea50 sub_30ea50
    Lifted::new(0x70009faa60, fn_70009faa60 as *const (), 1), // +0x30ea60 sub_30ea60
    Lifted::new(0x70009faab0, fn_70009faab0 as *const (), 1), // +0x30eab0 sub_30eab0
    Lifted::new(0x70009fab10, fn_70009fab10 as *const (), 1), // +0x30eb10 sub_30eb10
    Lifted::new(0x70009fab20, fn_70009fab20 as *const (), 1), // +0x30eb20 sub_30eb20
    Lifted::new(0x70009fab50, fn_70009fab50 as *const (), 1), // +0x30eb50 sub_30eb50
    Lifted::new(0x70009fab60, fn_70009fab60 as *const (), 1), // +0x30eb60 sub_30eb60
    Lifted::new(0x70009fabc0, fn_70009fabc0 as *const (), 1), // +0x30ebc0 sub_30ebc0
    Lifted::new(0x70009fac20, fn_70009fac20 as *const (), 1), // +0x30ec20 sub_30ec20
    Lifted::new(0x70009fac30, fn_70009fac30 as *const (), 1), // +0x30ec30 sub_30ec30
    Lifted::new(0x70009fac70, fn_70009fac70 as *const (), 1), // +0x30ec70 sub_30ec70
    Lifted::new(0x70009facd0, fn_70009facd0 as *const (), 1), // +0x30ecd0 sub_30ecd0
    Lifted::new(0x70009fad00, tpl_480 as *const (), 1), // +0x30ed00 sub_30ed00
    Lifted::new(0x70009fad10, fn_70009fad10 as *const (), 1), // +0x30ed10 sub_30ed10
    Lifted::new(0x70009fad50, fn_70009fad50 as *const (), 1), // +0x30ed50 sub_30ed50
    Lifted::new(0x70009fadb0, fn_70009fadb0 as *const (), 1), // +0x30edb0 sub_30edb0
    Lifted::new(0x70009fadf0, fn_70009fadf0 as *const (), 0), // +0x30edf0 sub_30edf0
    Lifted::new(0x70009fae60, fn_70009fae60 as *const (), 3), // +0x30ee60 sub_30ee60
    Lifted::new(0x70009faf60, fn_70009faf60 as *const (), 0), // +0x30ef60 sub_30ef60
    Lifted::new(0x70009fafd0, fn_70009fafd0 as *const (), 0), // +0x30efd0 sub_30efd0
    Lifted::new(0x70009fb040, fn_70009fb040 as *const (), 0), // +0x30f040 sub_30f040
    Lifted::new(0x70009fb0b0, fn_70009fb0b0 as *const (), 1), // +0x30f0b0 sub_30f0b0
    Lifted::new(0x70009fb0c0, fn_70009fb0c0 as *const (), 1), // +0x30f0c0 sub_30f0c0
    Lifted::new(0x70009fb0d0, fn_70009fb0d0 as *const (), 1), // +0x30f0d0 sub_30f0d0
    Lifted::new(0x70009fb100, tpl_433::<0x7000f1c4c0> as *const (), 0), // +0x30f100 sub_30f100
    Lifted::new(0x70009fb140, fn_70009fb140 as *const (), 1), // +0x30f140 sub_30f140
    Lifted::new(0x70009fb150, fn_70009fb150 as *const (), 1), // +0x30f150 sub_30f150
    Lifted::new(0x70009fb1a0, fn_70009fb1a0 as *const (), 1), // +0x30f1a0 sub_30f1a0
    Lifted::new(0x70009fb1e0, fn_70009fb1e0 as *const (), 1), // +0x30f1e0 sub_30f1e0
    Lifted::new(0x70009fb260, fn_70009fb260 as *const (), 1), // +0x30f260 sub_30f260
    Lifted::new(0x70009fb2e0, tpl_193::<0x28> as *const (), 1), // +0x30f2e0 sub_30f2e0
    Lifted::new(0x70009fb2f0, tpl_347::<0x8> as *const (), 1), // +0x30f2f0 sub_30f2f0
    Lifted::new(0x70009fb300, tpl_486::<0x13> as *const (), 1), // +0x30f300 sub_30f300
    Lifted::new(0x70009fb310, fn_70009fb310 as *const (), 1), // +0x30f310 sub_30f310
    Lifted::new(0x70009fb350, fn_70009fb350 as *const (), 1), // +0x30f350 sub_30f350
    Lifted::new(0x70009fb390, fn_70009fb390 as *const (), 1), // +0x30f390 sub_30f390
    Lifted::new(0x70009fb3d0, fn_70009fb3d0 as *const (), 1), // +0x30f3d0 sub_30f3d0
    Lifted::new(0x70009fb410, fn_70009fb410 as *const (), 0), // +0x30f410 sub_30f410
    Lifted::new(0x70009fb450, fn_70009fb450 as *const (), 2), // +0x30f450 sub_30f450
    Lifted::new(0x70009fb480, fn_70009fb480 as *const (), 0), // +0x30f480 sub_30f480
    Lifted::new(0x70009fb4b0, fn_70009fb4b0 as *const (), 0), // +0x30f4b0 sub_30f4b0
    Lifted::new(0x70009fb640, tpl_488::<0x8, 0xa718, 0x7000785640> as *const (), 0), // +0x30f640 sub_30f640
    Lifted::new(0x70009fb660, fn_70009fb660 as *const (), 1), // +0x30f660 sub_30f660
    Lifted::new(0x70009fb6a0, tpl_146::<0x3> as *const (), 1), // +0x30f6a0 sub_30f6a0
    Lifted::new(0x70009fb6b0, tpl_146::<0x3> as *const (), 1), // +0x30f6b0 sub_30f6b0
    Lifted::new(0x70009fb6c0, tpl_147::<0x7> as *const (), 1), // +0x30f6c0 sub_30f6c0
    Lifted::new(0x70009fb6d0, tpl_435::<0xb, 0x2b, 0x1f> as *const (), 1), // +0x30f6d0 sub_30f6d0
    Lifted::new(0x70009fb710, fn_70009fb710 as *const (), 1), // +0x30f710 sub_30f710
    Lifted::new(0x70009fb770, tpl_36::<0x10> as *const (), 1), // +0x30f770 sub_30f770
    Lifted::new(0x70009fb780, fn_70009fb780 as *const (), 1), // +0x30f780 sub_30f780
    Lifted::new(0x70009fb7c0, tpl_260::<0x10> as *const (), 0), // +0x30f7c0 sub_30f7c0
    Lifted::new(0x70009fb7d0, fn_70009fb7d0 as *const (), 2), // +0x30f7d0 sub_30f7d0
    Lifted::new(0x70009fb870, fn_70009fb870 as *const (), 2), // +0x30f870 sub_30f870
    Lifted::new(0x70009fb930, fn_70009fb930 as *const (), 1), // +0x30f930 sub_30f930
    Lifted::new(0x70009fb9a0, fn_70009fb9a0 as *const (), 0), // +0x30f9a0 sub_30f9a0
    Lifted::new(0x70009fba50, fn_70009fba50 as *const (), 0), // +0x30fa50 sub_30fa50
    Lifted::new(0x70009fbad0, tpl_489::<0x700078e810> as *const (), 0), // +0x30fad0 sub_30fad0
    Lifted::new(0x70009fbae0, fn_70009fbae0 as *const (), 0), // +0x30fae0 sub_30fae0
    Lifted::new(0x70009fbb40, tpl_489::<0x700078e810> as *const (), 0), // +0x30fb40 sub_30fb40
    Lifted::new(0x70009fbb50, fn_70009fbb50 as *const (), 2), // +0x30fb50 sub_30fb50
    Lifted::new(0x70009fbdb0, tpl_490::<0xffffffffffffffff, 0x7, 0xb, 0x7000f2a560, 0x10, 0xffffffffffff9918, 0x28, 0x70007478b0, 0x70017d7390, 0x70017a0290, 0x7001d54ef8, 0x70009fbe89> as *const (), 0), // +0x30fdb0 sub_30fdb0
    Lifted::new(0x70009fbec0, tpl_490::<0xffffffffffffffff, 0x7, 0xb, 0x7000f2a560, 0x10, 0xffffffffffff9918, 0x28, 0x7000722cb0, 0x70017d7390, 0x70017a0290, 0x7001d54ef8, 0x70009fbf99> as *const (), 0), // +0x30fec0 sub_30fec0
    Lifted::new(0x70009fbfd0, fn_70009fbfd0 as *const (), 0), // +0x30ffd0 sub_30ffd0
    Lifted::new(0x70009fc0b0, fn_70009fc0b0 as *const (), 2), // +0x3100b0 sub_3100b0
    Lifted::new(0x70009fc560, fn_70009fc560 as *const (), 1), // +0x310560 sub_310560
    Lifted::new(0x70009fc580, fn_70009fc580 as *const (), 1), // +0x310580 sub_310580
    Lifted::new(0x70009fc5a0, fn_70009fc5a0 as *const (), 0), // +0x3105a0 sub_3105a0
    Lifted::new(0x70009fc710, tpl_491::<0x10, 0x28, 0x20, 0x18, 0x4, 0x8, 0x7000a2c5c0> as *const (), 0), // +0x310710 sub_310710
    Lifted::new(0x70009fc760, fn_70009fc760 as *const (), 1), // +0x310760 sub_310760
    Lifted::new(0x70009fc780, fn_70009fc780 as *const (), 0), // +0x310780 sub_310780
    Lifted::new(0x70009fc7f0, fn_70009fc7f0 as *const (), 2), // +0x3107f0 sub_3107f0
    Lifted::new(0x70009fc9a0, fn_70009fc9a0 as *const (), 0), // +0x3109a0 sub_3109a0
    Lifted::new(0x70009fca20, fn_70009fca20 as *const (), 0), // +0x310a20 sub_310a20
    Lifted::new(0x70009fcb00, fn_70009fcb00 as *const (), 0), // +0x310b00 sub_310b00
    Lifted::new(0x70009fcb50, fn_70009fcb50 as *const (), 0), // +0x310b50 sub_310b50
    Lifted::new(0x70009fcb90, fn_70009fcb90 as *const (), 0), // +0x310b90 sub_310b90
    Lifted::new(0x70009fcd60, tpl_189::<0x700185ba80> as *const (), 1), // +0x310d60 sub_310d60
    Lifted::new(0x70009fcd80, tpl_492::<0x74> as *const (), 1), // +0x310d80 sub_310d80
    Lifted::new(0x70009fcd90, tpl_493 as *const (), 1), // +0x310d90 sub_310d90
    Lifted::new(0x70009fcda0, fn_70009fcda0 as *const (), 2), // +0x310da0 sub_310da0
    Lifted::new(0x70009fcfa0, fn_70009fcfa0 as *const (), 2), // +0x310fa0 sub_310fa0
    Lifted::new(0x70009fd280, fn_70009fd280 as *const (), 2), // +0x311280 sub_311280
    Lifted::new(0x70009fd3e0, fn_70009fd3e0 as *const (), 2), // +0x3113e0 sub_3113e0
    Lifted::new(0x70009fd6b0, fn_70009fd6b0 as *const (), 2), // +0x3116b0 sub_3116b0
    Lifted::new(0x70009fd800, fn_70009fd800 as *const (), 2), // +0x311800 sub_311800
    Lifted::new(0x70009fd9f0, fn_70009fd9f0 as *const (), 2), // +0x3119f0 sub_3119f0
    Lifted::new(0x70009fdc10, tpl_494::<0x3080, 0x700077c7f0> as *const (), 0), // +0x311c10 sub_311c10
    Lifted::new(0x70009fdc80, fn_70009fdc80 as *const (), 2), // +0x311c80 sub_311c80
    Lifted::new(0x70009fde10, fn_70009fde10 as *const (), 2), // +0x311e10 sub_311e10
    Lifted::new(0x70009fe000, fn_70009fe000 as *const (), 2), // +0x312000 sub_312000
    Lifted::new(0x70009fe220, fn_70009fe220 as *const (), 2), // +0x312220 sub_312220
    Lifted::new(0x70009fe410, fn_70009fe410 as *const (), 2), // +0x312410 sub_312410
    Lifted::new(0x70009fe480, fn_70009fe480 as *const (), 0), // +0x312480 sub_312480
    Lifted::new(0x70009fe4f0, fn_70009fe4f0 as *const (), 2), // +0x3124f0 sub_3124f0
    Lifted::new(0x70009fe650, fn_70009fe650 as *const (), 2), // +0x312650 sub_312650
    Lifted::new(0x70009fe7e0, fn_70009fe7e0 as *const (), 2), // +0x3127e0 sub_3127e0
    Lifted::new(0x70009fed50, fn_70009fed50 as *const (), 2), // +0x312d50 sub_312d50
    Lifted::new(0x70009fee70, fn_70009fee70 as *const (), 2), // +0x312e70 sub_312e70
    Lifted::new(0x70009ff060, fn_70009ff060 as *const (), 2), // +0x313060 sub_313060
    Lifted::new(0x70009ff280, fn_70009ff280 as *const (), 2), // +0x313280 sub_313280
    Lifted::new(0x70009ff470, fn_70009ff470 as *const (), 2), // +0x313470 sub_313470
    Lifted::new(0x70009ff690, fn_70009ff690 as *const (), 2), // +0x313690 sub_313690
    Lifted::new(0x70009ff980, fn_70009ff980 as *const (), 2), // +0x313980 sub_313980
    Lifted::new(0x70009ffb70, fn_70009ffb70 as *const (), 2), // +0x313b70 sub_313b70
    Lifted::new(0x70009ffd60, fn_70009ffd60 as *const (), 2), // +0x313d60 sub_313d60
    Lifted::new(0x70009fff80, fn_70009fff80 as *const (), 2), // +0x313f80 sub_313f80
    Lifted::new(0x7000a00170, fn_7000a00170 as *const (), 2), // +0x314170 sub_314170
    Lifted::new(0x7000a00390, fn_7000a00390 as *const (), 2), // +0x314390 sub_314390
    Lifted::new(0x7000a00580, fn_7000a00580 as *const (), 2), // +0x314580 sub_314580
    Lifted::new(0x7000a007a0, fn_7000a007a0 as *const (), 2), // +0x3147a0 sub_3147a0
    Lifted::new(0x7000a00880, fn_7000a00880 as *const (), 0), // +0x314880 sub_314880
    Lifted::new(0x7000a008c0, fn_7000a008c0 as *const (), 2), // +0x3148c0 sub_3148c0
    Lifted::new(0x7000a009d0, fn_7000a009d0 as *const (), 0), // +0x3149d0 sub_3149d0
    Lifted::new(0x7000a00a50, fn_7000a00a50 as *const (), 2), // +0x314a50 sub_314a50
    Lifted::new(0x7000a00ad0, fn_7000a00ad0 as *const (), 2), // +0x314ad0 sub_314ad0
    Lifted::new(0x7000a00c80, fn_7000a00c80 as *const (), 2), // +0x314c80 sub_314c80
    Lifted::new(0x7000a00e30, fn_7000a00e30 as *const (), 2), // +0x314e30 sub_314e30
    Lifted::new(0x7000a010f0, tpl_257::<0x7000a096a0> as *const (), 0), // +0x3150f0 sub_3150f0
    Lifted::new(0x7000a01110, tpl_257::<0x7000a096a0> as *const (), 0), // +0x315110 sub_315110
    Lifted::new(0x7000a01130, fn_7000a01130 as *const (), 0), // +0x315130 sub_315130
    Lifted::new(0x7000a01220, fn_7000a01220 as *const (), 2), // +0x315220 sub_315220
    Lifted::new(0x7000a01400, fn_7000a01400 as *const (), 2), // +0x315400 sub_315400
    Lifted::new(0x7000a01820, fn_7000a01820 as *const (), 2), // +0x315820 sub_315820
    Lifted::new(0x7000a01c40, fn_7000a01c40 as *const (), 2), // +0x315c40 sub_315c40
    Lifted::new(0x7000a02050, fn_7000a02050 as *const (), 2), // +0x316050 sub_316050
    Lifted::new(0x7000a02160, fn_7000a02160 as *const (), 2), // +0x316160 sub_316160
    Lifted::new(0x7000a02400, fn_7000a02400 as *const (), 2), // +0x316400 sub_316400
    Lifted::new(0x7000a02600, tpl_493 as *const (), 1), // +0x316600 sub_316600
    Lifted::new(0x7000a02610, fn_7000a02610 as *const (), 2), // +0x316610 sub_316610
    Lifted::new(0x7000a02790, fn_7000a02790 as *const (), 2), // +0x316790 sub_316790
    Lifted::new(0x7000a02970, fn_7000a02970 as *const (), 2), // +0x316970 sub_316970
    Lifted::new(0x7000a02af0, fn_7000a02af0 as *const (), 2), // +0x316af0 sub_316af0
    Lifted::new(0x7000a02ce0, fn_7000a02ce0 as *const (), 2), // +0x316ce0 sub_316ce0
    Lifted::new(0x7000a02f00, fn_7000a02f00 as *const (), 2), // +0x316f00 sub_316f00
    Lifted::new(0x7000a030f0, fn_7000a030f0 as *const (), 2), // +0x3170f0 sub_3170f0
    Lifted::new(0x7000a03310, fn_7000a03310 as *const (), 2), // +0x317310 sub_317310
    Lifted::new(0x7000a03500, fn_7000a03500 as *const (), 2), // +0x317500 sub_317500
    Lifted::new(0x7000a03720, fn_7000a03720 as *const (), 2), // +0x317720 sub_317720
    Lifted::new(0x7000a03910, fn_7000a03910 as *const (), 2), // +0x317910 sub_317910
    Lifted::new(0x7000a03b30, fn_7000a03b30 as *const (), 0), // +0x317b30 sub_317b30
    Lifted::new(0x7000a03d00, fn_7000a03d00 as *const (), 2), // +0x317d00 sub_317d00
    Lifted::new(0x7000a03ea0, fn_7000a03ea0 as *const (), 0), // +0x317ea0 sub_317ea0
    Lifted::new(0x7000a03f50, fn_7000a03f50 as *const (), 2), // +0x317f50 sub_317f50
    Lifted::new(0x7000a040f0, fn_7000a040f0 as *const (), 0), // +0x3180f0 sub_3180f0
    Lifted::new(0x7000a041a0, fn_7000a041a0 as *const (), 0), // +0x3181a0 sub_3181a0
    Lifted::new(0x7000a04250, fn_7000a04250 as *const (), 0), // +0x318250 sub_318250
    Lifted::new(0x7000a04300, fn_7000a04300 as *const (), 0), // +0x318300 sub_318300
    Lifted::new(0x7000a043a0, fn_7000a043a0 as *const (), 0), // +0x3183a0 sub_3183a0
    Lifted::new(0x7000a043c0, fn_7000a043c0 as *const (), 0), // +0x3183c0 sub_3183c0
    Lifted::new(0x7000a044e0, fn_7000a044e0 as *const (), 0), // +0x3184e0 sub_3184e0
    Lifted::new(0x7000a045e0, fn_7000a045e0 as *const (), 0), // +0x3185e0 sub_3185e0
    Lifted::new(0x7000a046b0, tpl_495::<0xb8, 0x70009c9830> as *const (), 0), // +0x3186b0 sub_3186b0
    Lifted::new(0x7000a046f0, fn_7000a046f0 as *const (), 0), // +0x3186f0 sub_3186f0
    Lifted::new(0x7000a047a0, tpl_495::<0xb8, 0x70009c9950> as *const (), 0), // +0x3187a0 sub_3187a0
    Lifted::new(0x7000a047e0, fn_7000a047e0 as *const (), 2), // +0x3187e0 sub_3187e0
    Lifted::new(0x7000a04b80, fn_7000a04b80 as *const (), 0), // +0x318b80 sub_318b80
    Lifted::new(0x7000a04c30, fn_7000a04c30 as *const (), 0), // +0x318c30 sub_318c30
    Lifted::new(0x7000a04c60, fn_7000a04c60 as *const (), 0), // +0x318c60 sub_318c60
    Lifted::new(0x7000a04cd0, fn_7000a04cd0 as *const (), 0), // +0x318cd0 sub_318cd0
    Lifted::new(0x7000a04d70, tpl_496::<0xb8, 0x10, 0x18, 0x700076bf20, 0x70009ce340> as *const (), 0), // +0x318d70 sub_318d70
    Lifted::new(0x7000a04df0, fn_7000a04df0 as *const (), 0), // +0x318df0 sub_318df0
    Lifted::new(0x7000a04e80, fn_7000a04e80 as *const (), 2), // +0x318e80 sub_318e80
    Lifted::new(0x7000a05070, fn_7000a05070 as *const (), 2), // +0x319070 sub_319070
    Lifted::new(0x7000a05250, fn_7000a05250 as *const (), 0), // +0x319250 sub_319250
    Lifted::new(0x7000a05350, fn_7000a05350 as *const (), 2), // +0x319350 sub_319350
    Lifted::new(0x7000a05550, fn_7000a05550 as *const (), 2), // +0x319550 sub_319550
    Lifted::new(0x7000a05760, tpl_494::<0x3080, 0x7000db91c0> as *const (), 0), // +0x319760 sub_319760
    Lifted::new(0x7000a057d0, fn_7000a057d0 as *const (), 2), // +0x3197d0 sub_3197d0
    Lifted::new(0x7000a05870, fn_7000a05870 as *const (), 2), // +0x319870 sub_319870
    Lifted::new(0x7000a05a50, fn_7000a05a50 as *const (), 2), // +0x319a50 sub_319a50
    Lifted::new(0x7000a05be0, fn_7000a05be0 as *const (), 2), // +0x319be0 sub_319be0
    Lifted::new(0x7000a06000, tpl_497::<0xb8, 0x10, 0x18, 0x700076bf20, 0xd8, 0x7000766820, 0x43, 0x60, 0x7000726da0> as *const (), 0), // +0x31a000 sub_31a000
    Lifted::new(0x7000a060f0, fn_7000a060f0 as *const (), 0), // +0x31a0f0 sub_31a0f0
    Lifted::new(0x7000a06190, fn_7000a06190 as *const (), 0), // +0x31a190 sub_31a190
    Lifted::new(0x7000a061d0, fn_7000a061d0 as *const (), 2), // +0x31a1d0 sub_31a1d0
    Lifted::new(0x7000a065f0, fn_7000a065f0 as *const (), 2), // +0x31a5f0 sub_31a5f0
    Lifted::new(0x7000a06d60, fn_7000a06d60 as *const (), 0), // +0x31ad60 sub_31ad60
    Lifted::new(0x7000a06df0, fn_7000a06df0 as *const (), 0), // +0x31adf0 sub_31adf0
    Lifted::new(0x7000a06ea0, tpl_496::<0xb8, 0x10, 0x18, 0x700076bf20, 0x70009ce6f0> as *const (), 0), // +0x31aea0 sub_31aea0
    Lifted::new(0x7000a06f20, fn_7000a06f20 as *const (), 0), // +0x31af20 sub_31af20
    Lifted::new(0x7000a06fd0, fn_7000a06fd0 as *const (), 0), // +0x31afd0 sub_31afd0
    Lifted::new(0x7000a07070, fn_7000a07070 as *const (), 2), // +0x31b070 sub_31b070
    Lifted::new(0x7000a074e0, fn_7000a074e0 as *const (), 2), // +0x31b4e0 sub_31b4e0
    Lifted::new(0x7000a07930, fn_7000a07930 as *const (), 0), // +0x31b930 sub_31b930
    Lifted::new(0x7000a079e0, fn_7000a079e0 as *const (), 0), // +0x31b9e0 sub_31b9e0
    Lifted::new(0x7000a079f0, fn_7000a079f0 as *const (), 0), // +0x31b9f0 sub_31b9f0
    Lifted::new(0x7000a07aa0, fn_7000a07aa0 as *const (), 0), // +0x31baa0 sub_31baa0
    Lifted::new(0x7000a07b40, fn_7000a07b40 as *const (), 0), // +0x31bb40 sub_31bb40
    Lifted::new(0x7000a07bf0, tpl_496::<0xb8, 0x10, 0x18, 0x700076bf20, 0x70009ceaa0> as *const (), 0), // +0x31bbf0 sub_31bbf0
    Lifted::new(0x7000a07c70, fn_7000a07c70 as *const (), 0), // +0x31bc70 sub_31bc70
    Lifted::new(0x7000a07d00, tpl_17::<0x700077ec40> as *const (), 0), // +0x31bd00 sub_31bd00
    Lifted::new(0x7000a07d10, fn_7000a07d10 as *const (), 0), // +0x31bd10 sub_31bd10
    Lifted::new(0x7000a07d80, fn_7000a07d80 as *const (), 0), // +0x31bd80 sub_31bd80
    Lifted::new(0x7000a07e90, fn_7000a07e90 as *const (), 0), // +0x31be90 sub_31be90
    Lifted::new(0x7000a07f20, fn_7000a07f20 as *const (), 0), // +0x31bf20 sub_31bf20
    Lifted::new(0x7000a07f60, fn_7000a07f60 as *const (), 0), // +0x31bf60 sub_31bf60
    Lifted::new(0x7000a07fa0, fn_7000a07fa0 as *const (), 0), // +0x31bfa0 sub_31bfa0
    Lifted::new(0x7000a07fd0, fn_7000a07fd0 as *const (), 0), // +0x31bfd0 sub_31bfd0
    Lifted::new(0x7000a08000, fn_7000a08000 as *const (), 0), // +0x31c000 sub_31c000
    Lifted::new(0x7000a080c0, fn_7000a080c0 as *const (), 0), // +0x31c0c0 sub_31c0c0
    Lifted::new(0x7000a08110, fn_7000a08110 as *const (), 0), // +0x31c110 sub_31c110
    Lifted::new(0x7000a08160, tpl_498::<0xb8, 0x10, 0x18, 0x700076bf20, 0xd8, 0x7000766820> as *const (), 0), // +0x31c160 sub_31c160
    Lifted::new(0x7000a08200, fn_7000a08200 as *const (), 2), // +0x31c200 sub_31c200
    Lifted::new(0x7000a083c0, fn_7000a083c0 as *const (), 0), // +0x31c3c0 sub_31c3c0
    Lifted::new(0x7000a08460, fn_7000a08460 as *const (), 2), // +0x31c460 sub_31c460
    Lifted::new(0x7000a08970, fn_7000a08970 as *const (), 2), // +0x31c970 sub_31c970
    Lifted::new(0x7000a08b10, fn_7000a08b10 as *const (), 2), // +0x31cb10 sub_31cb10
    Lifted::new(0x7000a08cc0, fn_7000a08cc0 as *const (), 0), // +0x31ccc0 sub_31ccc0
    Lifted::new(0x7000a08d10, tpl_499::<0x8, 0x70017d8b28, 0x70017a0290, 0x7001d54ef8, 0x7000a08d31, 0xa0a8, 0x7000708ea0> as *const (), 0), // +0x31cd10 sub_31cd10
    Lifted::new(0x7000a08d50, tpl_196 as *const (), 1), // +0x31cd50 sub_31cd50
    Lifted::new(0x7000a08d60, tpl_196 as *const (), 1), // +0x31cd60 sub_31cd60
    Lifted::new(0x7000a08d70, tpl_196 as *const (), 1), // +0x31cd70 sub_31cd70
    Lifted::new(0x7000a08d80, tpl_196 as *const (), 1), // +0x31cd80 sub_31cd80
    Lifted::new(0x7000a08d90, tpl_196 as *const (), 1), // +0x31cd90 sub_31cd90
    Lifted::new(0x7000a08da0, tpl_196 as *const (), 1), // +0x31cda0 sub_31cda0
    Lifted::new(0x7000a08db0, tpl_500::<0x7000a08db3, 0x7000a08db4, 0x7000a08db5, 0x7000a08db6, 0x7000a08db7, 0x7000a08db8, 0x7000a08db9, 0x7000a08dba, 0x7000a08dbb, 0x7000a08dbc, 0x7000a08dbd, 0x7000a08dbe, 0x7000a08dbf, 0x70006f62f0> as *const (), 0), // +0x31cdb0 sub_31cdb0
    Lifted::new(0x7000a08dd0, tpl_148 as *const (), 1), // +0x31cdd0 sub_31cdd0
    Lifted::new(0x7000a08de0, fn_7000a08de0 as *const (), 1), // +0x31cde0 sub_31cde0
    Lifted::new(0x7000a08e00, tpl_430::<0x7000c57d60> as *const (), 0), // +0x31ce00 sub_31ce00
    Lifted::new(0x7000a08e10, fn_7000a08e10 as *const (), 3), // +0x31ce10 sub_31ce10
    Lifted::new(0x7000a08e80, fn_7000a08e80 as *const (), 2), // +0x31ce80 sub_31ce80
    Lifted::new(0x7000a091f0, fn_7000a091f0 as *const (), 2), // +0x31d1f0 sub_31d1f0
    Lifted::new(0x7000a096a0, fn_7000a096a0 as *const (), 2), // +0x31d6a0 sub_31d6a0
    Lifted::new(0x7000a09890, tpl_15 as *const (), 1), // +0x31d890 sub_31d890
    Lifted::new(0x7000a098a0, fn_7000a098a0 as *const (), 2), // +0x31d8a0 sub_31d8a0
    Lifted::new(0x7000a09e10, fn_7000a09e10 as *const (), 0), // +0x31de10 sub_31de10
    Lifted::new(0x7000a0a020, fn_7000a0a020 as *const (), 2), // +0x31e020 sub_31e020
    Lifted::new(0x7000a0a550, fn_7000a0a550 as *const (), 2), // +0x31e550 sub_31e550
    Lifted::new(0x7000a0abe0, fn_7000a0abe0 as *const (), 2), // +0x31ebe0 sub_31ebe0
    Lifted::new(0x7000a0b790, fn_7000a0b790 as *const (), 0), // +0x31f790 sub_31f790
    Lifted::new(0x7000a0b940, fn_7000a0b940 as *const (), 0), // +0x31f940 sub_31f940
    Lifted::new(0x7000a0bb60, fn_7000a0bb60 as *const (), 0), // +0x31fb60 sub_31fb60
    Lifted::new(0x7000a0c660, fn_7000a0c660 as *const (), 0), // +0x320660 sub_320660
    Lifted::new(0x7000a0cb20, fn_7000a0cb20 as *const (), 0), // +0x320b20 sub_320b20
    Lifted::new(0x7000a0cbf0, fn_7000a0cbf0 as *const (), 0), // +0x320bf0 sub_320bf0
    Lifted::new(0x7000a0ce80, fn_7000a0ce80 as *const (), 0), // +0x320e80 sub_320e80
    Lifted::new(0x7000a0d290, fn_7000a0d290 as *const (), 0), // +0x321290 sub_321290
    Lifted::new(0x7000a0d7d0, fn_7000a0d7d0 as *const (), 0), // +0x3217d0 sub_3217d0
    Lifted::new(0x7000a0d9b0, fn_7000a0d9b0 as *const (), 0), // +0x3219b0 sub_3219b0
    Lifted::new(0x7000a0dd00, fn_7000a0dd00 as *const (), 0), // +0x321d00 sub_321d00
    Lifted::new(0x7000a0dd40, fn_7000a0dd40 as *const (), 0), // +0x321d40 sub_321d40
    Lifted::new(0x7000a0e050, fn_7000a0e050 as *const (), 0), // +0x322050 sub_322050
    Lifted::new(0x7000a0e3d0, fn_7000a0e3d0 as *const (), 0), // +0x3223d0 sub_3223d0
    Lifted::new(0x7000a0e770, fn_7000a0e770 as *const (), 2), // +0x322770 sub_322770
    Lifted::new(0x7000a0f2a0, fn_7000a0f2a0 as *const (), 0), // +0x3232a0 sub_3232a0
    Lifted::new(0x7000a0f890, fn_7000a0f890 as *const (), 0), // +0x323890 sub_323890
    Lifted::new(0x7000a0fa90, fn_7000a0fa90 as *const (), 0), // +0x323a90 sub_323a90
    Lifted::new(0x7000a0fe80, fn_7000a0fe80 as *const (), 0), // +0x323e80 sub_323e80
    Lifted::new(0x7000a103b0, fn_7000a103b0 as *const (), 0), // +0x3243b0 sub_3243b0
    Lifted::new(0x7000a10410, fn_7000a10410 as *const (), 0), // +0x324410 sub_324410
    Lifted::new(0x7000a107f0, fn_7000a107f0 as *const (), 0), // +0x3247f0 sub_3247f0
    Lifted::new(0x7000a10890, fn_7000a10890 as *const (), 0), // +0x324890 sub_324890
    Lifted::new(0x7000a10d20, fn_7000a10d20 as *const (), 0), // +0x324d20 sub_324d20
    Lifted::new(0x7000a10f60, fn_7000a10f60 as *const (), 2), // +0x324f60 sub_324f60
    Lifted::new(0x7000a112f0, fn_7000a112f0 as *const (), 0), // +0x3252f0 sub_3252f0
    Lifted::new(0x7000a11890, fn_7000a11890 as *const (), 2), // +0x325890 sub_325890
    Lifted::new(0x7000a123d0, fn_7000a123d0 as *const (), 0), // +0x3263d0 sub_3263d0
    Lifted::new(0x7000a12bc0, fn_7000a12bc0 as *const (), 0), // +0x326bc0 sub_326bc0
    Lifted::new(0x7000a13140, fn_7000a13140 as *const (), 2), // +0x327140 sub_327140
    Lifted::new(0x7000a13a50, fn_7000a13a50 as *const (), 0), // +0x327a50 sub_327a50
    Lifted::new(0x7000a13f40, fn_7000a13f40 as *const (), 0), // +0x327f40 sub_327f40
    Lifted::new(0x7000a146f0, fn_7000a146f0 as *const (), 0), // +0x3286f0 sub_3286f0
    Lifted::new(0x7000a15410, fn_7000a15410 as *const (), 0), // +0x329410 sub_329410
    Lifted::new(0x7000a15520, fn_7000a15520 as *const (), 0), // +0x329520 sub_329520
    Lifted::new(0x7000a15710, fn_7000a15710 as *const (), 0), // +0x329710 sub_329710
    Lifted::new(0x7000a15b00, fn_7000a15b00 as *const (), 0), // +0x329b00 sub_329b00
    Lifted::new(0x7000a15ba0, fn_7000a15ba0 as *const (), 0), // +0x329ba0 sub_329ba0
    Lifted::new(0x7000a15c90, fn_7000a15c90 as *const (), 0), // +0x329c90 sub_329c90
    Lifted::new(0x7000a15da0, fn_7000a15da0 as *const (), 0), // +0x329da0 sub_329da0
    Lifted::new(0x7000a16220, fn_7000a16220 as *const (), 0), // +0x32a220 sub_32a220
    Lifted::new(0x7000a16530, fn_7000a16530 as *const (), 0), // +0x32a530 sub_32a530
    Lifted::new(0x7000a171b0, fn_7000a171b0 as *const (), 0), // +0x32b1b0 sub_32b1b0
    Lifted::new(0x7000a17220, fn_7000a17220 as *const (), 0), // +0x32b220 sub_32b220
    Lifted::new(0x7000a17460, fn_7000a17460 as *const (), 0), // +0x32b460 sub_32b460
    Lifted::new(0x7000a174b0, fn_7000a174b0 as *const (), 0), // +0x32b4b0 sub_32b4b0
    Lifted::new(0x7000a174e0, fn_7000a174e0 as *const (), 0), // +0x32b4e0 sub_32b4e0
    Lifted::new(0x7000a178b0, fn_7000a178b0 as *const (), 0), // +0x32b8b0 sub_32b8b0
    Lifted::new(0x7000a17bc0, fn_7000a17bc0 as *const (), 0), // +0x32bbc0 sub_32bbc0
    Lifted::new(0x7000a17e90, fn_7000a17e90 as *const (), 0), // +0x32be90 sub_32be90
    Lifted::new(0x7000a17fc0, fn_7000a17fc0 as *const (), 0), // +0x32bfc0 sub_32bfc0
    Lifted::new(0x7000a18130, fn_7000a18130 as *const (), 2), // +0x32c130 sub_32c130
    Lifted::new(0x7000a187e0, fn_7000a187e0 as *const (), 0), // +0x32c7e0 sub_32c7e0
    Lifted::new(0x7000a19350, fn_7000a19350 as *const (), 0), // +0x32d350 sub_32d350
    Lifted::new(0x7000a197e0, fn_7000a197e0 as *const (), 0), // +0x32d7e0 sub_32d7e0
    Lifted::new(0x7000a19970, fn_7000a19970 as *const (), 0), // +0x32d970 sub_32d970
    Lifted::new(0x7000a19a30, fn_7000a19a30 as *const (), 0), // +0x32da30 sub_32da30
    Lifted::new(0x7000a1acd0, fn_7000a1acd0 as *const (), 2), // +0x32ecd0 sub_32ecd0
    Lifted::new(0x7000a1b770, fn_7000a1b770 as *const (), 0), // +0x32f770 sub_32f770
    Lifted::new(0x7000a1b840, fn_7000a1b840 as *const (), 0), // +0x32f840 sub_32f840
    Lifted::new(0x7000a1b8c0, fn_7000a1b8c0 as *const (), 0), // +0x32f8c0 sub_32f8c0
    Lifted::new(0x7000a1ba70, fn_7000a1ba70 as *const (), 0), // +0x32fa70 sub_32fa70
    Lifted::new(0x7000a1bbd0, fn_7000a1bbd0 as *const (), 0), // +0x32fbd0 sub_32fbd0
    Lifted::new(0x7000a1c220, fn_7000a1c220 as *const (), 0), // +0x330220 sub_330220
    Lifted::new(0x7000a1c3d0, fn_7000a1c3d0 as *const (), 0), // +0x3303d0 sub_3303d0
    Lifted::new(0x7000a1ca50, fn_7000a1ca50 as *const (), 0), // +0x330a50 sub_330a50
    Lifted::new(0x7000a1cd20, tpl_501::<0x70006f4a00> as *const (), 0), // +0x330d20 sub_330d20
    Lifted::new(0x7000a1cd30, fn_7000a1cd30 as *const (), 2), // +0x330d30 sub_330d30
    Lifted::new(0x7000a1de70, fn_7000a1de70 as *const (), 0), // +0x331e70 sub_331e70
    Lifted::new(0x7000a1e1a0, fn_7000a1e1a0 as *const (), 0), // +0x3321a0 sub_3321a0
    Lifted::new(0x7000a1e310, fn_7000a1e310 as *const (), 0), // +0x332310 sub_332310
    Lifted::new(0x7000a1e6a0, fn_7000a1e6a0 as *const (), 0), // +0x3326a0 sub_3326a0
    Lifted::new(0x7000a1f140, fn_7000a1f140 as *const (), 0), // +0x333140 sub_333140
    Lifted::new(0x7000a1f520, fn_7000a1f520 as *const (), 0), // +0x333520 sub_333520
    Lifted::new(0x7000a1f8b0, fn_7000a1f8b0 as *const (), 2), // +0x3338b0 sub_3338b0
    Lifted::new(0x7000a1ff50, fn_7000a1ff50 as *const (), 0), // +0x333f50 sub_333f50
    Lifted::new(0x7000a20070, fn_7000a20070 as *const (), 0), // +0x334070 sub_334070
    Lifted::new(0x7000a20400, fn_7000a20400 as *const (), 0), // +0x334400 sub_334400
    Lifted::new(0x7000a20620, fn_7000a20620 as *const (), 2), // +0x334620 sub_334620
    Lifted::new(0x7000a21360, fn_7000a21360 as *const (), 2), // +0x335360 sub_335360
    Lifted::new(0x7000a21470, fn_7000a21470 as *const (), 0), // +0x335470 sub_335470
    Lifted::new(0x7000a218a0, fn_7000a218a0 as *const (), 0), // +0x3358a0 sub_3358a0
    Lifted::new(0x7000a21c50, fn_7000a21c50 as *const (), 0), // +0x335c50 sub_335c50
    Lifted::new(0x7000a220d0, tpl_502::<0x518> as *const (), 1), // +0x3360d0 sub_3360d0
    Lifted::new(0x7000a220e0, fn_7000a220e0 as *const (), 1), // +0x3360e0 sub_3360e0
    Lifted::new(0x7000a22100, fn_7000a22100 as *const (), 0), // +0x336100 sub_336100
    Lifted::new(0x7000a22150, tpl_143::<0xf8, 0x700077bb20> as *const (), 0), // +0x336150 sub_336150
    Lifted::new(0x7000a22160, tpl_503::<0xf8, 0x38, 0x18, 0x1c, 0x60, 0x700072add0> as *const (), 0), // +0x336160 sub_336160
    Lifted::new(0x7000a221e0, fn_7000a221e0 as *const (), 0), // +0x3361e0 sub_3361e0
    Lifted::new(0x7000a22230, tpl_504::<0xc900, 0x4c, 0x40, 0x44, 0x700070e9f0> as *const (), 0), // +0x336230 sub_336230
    Lifted::new(0x7000a22260, tpl_504::<0xc900, 0x4c, 0x40, 0x44, 0x700070e9f0> as *const (), 0), // +0x336260 sub_336260
    Lifted::new(0x7000a22290, fn_7000a22290 as *const (), 1), // +0x336290 sub_336290
    Lifted::new(0x7000a222b0, fn_7000a222b0 as *const (), 0), // +0x3362b0 sub_3362b0
    Lifted::new(0x7000a222f0, fn_7000a222f0 as *const (), 1), // +0x3362f0 sub_3362f0
    Lifted::new(0x7000a22310, fn_7000a22310 as *const (), 0), // +0x336310 sub_336310
    Lifted::new(0x7000a22600, tpl_505::<0x30, 0x700074bc80> as *const (), 0), // +0x336600 sub_336600
    Lifted::new(0x7000a22610, fn_7000a22610 as *const (), 0), // +0x336610 sub_336610
    Lifted::new(0x7000a22650, fn_7000a22650 as *const (), 0), // +0x336650 sub_336650
    Lifted::new(0x7000a22670, fn_7000a22670 as *const (), 0), // +0x336670 sub_336670
    Lifted::new(0x7000a227f0, fn_7000a227f0 as *const (), 0), // +0x3367f0 sub_3367f0
    Lifted::new(0x7000a22870, tpl_506::<0x7000f40e50> as *const (), 0), // +0x336870 sub_336870
    Lifted::new(0x7000a22880, tpl_506::<0x7000f40ed0> as *const (), 0), // +0x336880 sub_336880
    Lifted::new(0x7000a22890, fn_7000a22890 as *const (), 0), // +0x336890 sub_336890
    Lifted::new(0x7000a22910, tpl_506::<0x7000f40f40> as *const (), 0), // +0x336910 sub_336910
    Lifted::new(0x7000a22920, fn_7000a22920 as *const (), 0), // +0x336920 sub_336920
    Lifted::new(0x7000a22950, fn_7000a22950 as *const (), 2), // +0x336950 sub_336950
    Lifted::new(0x7000a22bb0, fn_7000a22bb0 as *const (), 1), // +0x336bb0 sub_336bb0
    Lifted::new(0x7000a22bf0, tpl_507::<0x7000f35e60> as *const (), 0), // +0x336bf0 sub_336bf0
    Lifted::new(0x7000a22c00, fn_7000a22c00 as *const (), 2), // +0x336c00 sub_336c00
    Lifted::new(0x7000a22f40, tpl_468::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30, 0x7000774720> as *const (), 0), // +0x336f40 sub_336f40
    Lifted::new(0x7000a23060, fn_7000a23060 as *const (), 0), // +0x337060 sub_337060
    Lifted::new(0x7000a23150, tpl_507::<0x7000f3a550> as *const (), 0), // +0x337150 sub_337150
    Lifted::new(0x7000a23160, tpl_507::<0x7000f3acc0> as *const (), 0), // +0x337160 sub_337160
    Lifted::new(0x7000a23170, tpl_507::<0x7000f3ad40> as *const (), 0), // +0x337170 sub_337170
    Lifted::new(0x7000a23180, fn_7000a23180 as *const (), 2), // +0x337180 sub_337180
    Lifted::new(0x7000a235b0, fn_7000a235b0 as *const (), 0), // +0x3375b0 sub_3375b0
    Lifted::new(0x7000a23680, fn_7000a23680 as *const (), 0), // +0x337680 sub_337680
    Lifted::new(0x7000a236e0, fn_7000a236e0 as *const (), 2), // +0x3376e0 sub_3376e0
    Lifted::new(0x7000a238e0, fn_7000a238e0 as *const (), 0), // +0x3378e0 sub_3378e0
    Lifted::new(0x7000a23900, fn_7000a23900 as *const (), 0), // +0x337900 sub_337900
    Lifted::new(0x7000a239c0, tpl_508::<0x318, 0xb8, 0x10, 0x18, 0x700076bf20, 0x70017d4000, 0x8, 0x70009f6510> as *const (), 0), // +0x3379c0 sub_3379c0
    Lifted::new(0x7000a23a70, tpl_509::<0x318, 0xb8, 0x10, 0x18, 0x700076bf20, 0x70017d4028, 0x8, 0x70009f6650> as *const (), 0), // +0x337a70 sub_337a70
    Lifted::new(0x7000a23b20, fn_7000a23b20 as *const (), 0), // +0x337b20 sub_337b20
    Lifted::new(0x7000a23b90, fn_7000a23b90 as *const (), 1), // +0x337b90 sub_337b90
    Lifted::new(0x7000a23bc0, fn_7000a23bc0 as *const (), 0), // +0x337bc0 sub_337bc0
    Lifted::new(0x7000a23c90, tpl_509::<0x318, 0xb8, 0x10, 0x18, 0x700076bf20, 0x70017d4140, 0x8, 0x70009f68d0> as *const (), 0), // +0x337c90 sub_337c90
    Lifted::new(0x7000a23d40, fn_7000a23d40 as *const (), 0), // +0x337d40 sub_337d40
    Lifted::new(0x7000a23dd0, fn_7000a23dd0 as *const (), 0), // +0x337dd0 sub_337dd0
    Lifted::new(0x7000a23e40, fn_7000a23e40 as *const (), 1), // +0x337e40 sub_337e40
    Lifted::new(0x7000a23e80, tpl_510::<0x318, 0xb8, 0x10, 0x18, 0x700076bf20, 0x70017d40c8, 0x8, 0x70009f6c90> as *const (), 0), // +0x337e80 sub_337e80
    Lifted::new(0x7000a23f30, tpl_508::<0x318, 0xb8, 0x10, 0x18, 0x700076bf20, 0x70017d40f0, 0x8, 0x70009f6dd0> as *const (), 0), // +0x337f30 sub_337f30
    Lifted::new(0x7000a23fe0, fn_7000a23fe0 as *const (), 0), // +0x337fe0 sub_337fe0
    Lifted::new(0x7000a24040, fn_7000a24040 as *const (), 0), // +0x338040 sub_338040
    Lifted::new(0x7000a24060, tpl_508::<0x318, 0xb8, 0x10, 0x18, 0x700076bf20, 0x70017d4168, 0x8, 0x70009f7050> as *const (), 0), // +0x338060 sub_338060
    Lifted::new(0x7000a24110, fn_7000a24110 as *const (), 0), // +0x338110 sub_338110
    Lifted::new(0x7000a241d0, fn_7000a241d0 as *const (), 0), // +0x3381d0 sub_3381d0
    Lifted::new(0x7000a24290, tpl_510::<0x318, 0xb8, 0x10, 0x18, 0x700076bf20, 0x70017d41e0, 0x8, 0x70009f7410> as *const (), 0), // +0x338290 sub_338290
    Lifted::new(0x7000a24340, tpl_510::<0x318, 0xb8, 0x10, 0x18, 0x700076bf20, 0x70017d4208, 0x8, 0x70009f7550> as *const (), 0), // +0x338340 sub_338340
    Lifted::new(0x7000a243f0, fn_7000a243f0 as *const (), 2), // +0x3383f0 sub_3383f0
    Lifted::new(0x7000a24650, tpl_48::<0x7000ba9450, 0x638> as *const (), 0), // +0x338650 sub_338650
    Lifted::new(0x7000a24680, fn_7000a24680 as *const (), 0), // +0x338680 sub_338680
    Lifted::new(0x7000a24700, fn_7000a24700 as *const (), 0), // +0x338700 sub_338700
    Lifted::new(0x7000a24720, fn_7000a24720 as *const (), 0), // +0x338720 sub_338720
    Lifted::new(0x7000a247d0, fn_7000a247d0 as *const (), 0), // +0x3387d0 sub_3387d0
    Lifted::new(0x7000a24800, tpl_15 as *const (), 1), // +0x338800 sub_338800
    Lifted::new(0x7000a24810, fn_7000a24810 as *const (), 2), // +0x338810 sub_338810
    Lifted::new(0x7000a24a90, fn_7000a24a90 as *const (), 0), // +0x338a90 sub_338a90
    Lifted::new(0x7000a24af0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a24b01> as *const (), 0), // +0x338af0 sub_338af0
    Lifted::new(0x7000a24b10, fn_7000a24b10 as *const (), 1), // +0x338b10 sub_338b10
    Lifted::new(0x7000a24b20, tpl_168::<0x18> as *const (), 1), // +0x338b20 sub_338b20
    Lifted::new(0x7000a24b30, tpl_168::<0x10> as *const (), 1), // +0x338b30 sub_338b30
    Lifted::new(0x7000a24b40, tpl_168::<0x10> as *const (), 1), // +0x338b40 sub_338b40
    Lifted::new(0x7000a24b50, tpl_168::<0x10> as *const (), 1), // +0x338b50 sub_338b50
    Lifted::new(0x7000a24b60, tpl_511::<0x70009d8a90, 0x7000c5d120> as *const (), 0), // +0x338b60 sub_338b60
    Lifted::new(0x7000a24b80, tpl_430::<0x7000c5d120> as *const (), 0), // +0x338b80 sub_338b80
    Lifted::new(0x7000a24b90, tpl_511::<0x70009d8a90, 0x7000c5d1e0> as *const (), 0), // +0x338b90 sub_338b90
    Lifted::new(0x7000a24bb0, tpl_430::<0x7000c5d1e0> as *const (), 0), // +0x338bb0 sub_338bb0
    Lifted::new(0x7000a24bc0, fn_7000a24bc0 as *const (), 1), // +0x338bc0 sub_338bc0
    Lifted::new(0x7000a24c10, fn_7000a24c10 as *const (), 2), // +0x338c10 sub_338c10
    Lifted::new(0x7000a24e90, tpl_143::<0xc900, 0x7000738e20> as *const (), 0), // +0x338e90 sub_338e90
    Lifted::new(0x7000a24ea0, tpl_430::<0x70006f3af0> as *const (), 0), // +0x338ea0 sub_338ea0
    Lifted::new(0x7000a24eb0, fn_7000a24eb0 as *const (), 0), // +0x338eb0 sub_338eb0
    Lifted::new(0x7000a24fa0, fn_7000a24fa0 as *const (), 0), // +0x338fa0 sub_338fa0
    Lifted::new(0x7000a25050, fn_7000a25050 as *const (), 0), // +0x339050 sub_339050
    Lifted::new(0x7000a250e0, fn_7000a250e0 as *const (), 0), // +0x3390e0 sub_3390e0
    Lifted::new(0x7000a25100, fn_7000a25100 as *const (), 0), // +0x339100 sub_339100
    Lifted::new(0x7000a251b0, fn_7000a251b0 as *const (), 0), // +0x3391b0 sub_3391b0
    Lifted::new(0x7000a25250, tpl_512::<0xb0, 0x7000f56960, 0x170, 0x30, 0x178, 0x18, 0x10, 0x180, 0xf0, 0x38> as *const (), 0), // +0x339250 sub_339250
    Lifted::new(0x7000a252e0, tpl_512::<0xb0, 0x7000f569c0, 0x170, 0x30, 0x178, 0x18, 0x10, 0x180, 0xf0, 0x38> as *const (), 0), // +0x3392e0 sub_3392e0
    Lifted::new(0x7000a25370, fn_7000a25370 as *const (), 0), // +0x339370 sub_339370
    Lifted::new(0x7000a25420, fn_7000a25420 as *const (), 1), // +0x339420 sub_339420
    Lifted::new(0x7000a25450, fn_7000a25450 as *const (), 1), // +0x339450 sub_339450
    Lifted::new(0x7000a25490, fn_7000a25490 as *const (), 0), // +0x339490 sub_339490
    Lifted::new(0x7000a254c0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a254d1> as *const (), 0), // +0x3394c0 sub_3394c0
    Lifted::new(0x7000a254e0, fn_7000a254e0 as *const (), 0), // +0x3394e0 sub_3394e0
    Lifted::new(0x7000a255e0, fn_7000a255e0 as *const (), 0), // +0x3395e0 sub_3395e0
    Lifted::new(0x7000a25640, fn_7000a25640 as *const (), 1), // +0x339640 sub_339640
    Lifted::new(0x7000a25650, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a25661> as *const (), 0), // +0x339650 sub_339650
    Lifted::new(0x7000a25670, fn_7000a25670 as *const (), 1), // +0x339670 sub_339670
    Lifted::new(0x7000a25690, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a256a1> as *const (), 0), // +0x339690 sub_339690
    Lifted::new(0x7000a256b0, fn_7000a256b0 as *const (), 0), // +0x3396b0 sub_3396b0
    Lifted::new(0x7000a25760, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x339760 sub_339760
    Lifted::new(0x7000a25800, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x339800 sub_339800
    Lifted::new(0x7000a258a0, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x3398a0 sub_3398a0
    Lifted::new(0x7000a25940, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x339940 sub_339940
    Lifted::new(0x7000a259e0, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x3399e0 sub_3399e0
    Lifted::new(0x7000a25a80, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x339a80 sub_339a80
    Lifted::new(0x7000a25b40, tpl_515::<0x8, 0x1, 0x7000a25940, 0xc> as *const (), 0), // +0x339b40 sub_339b40
    Lifted::new(0x7000a25b90, tpl_515::<0x8, 0x1, 0x7000a259e0, 0xc> as *const (), 0), // +0x339b90 sub_339b90
    Lifted::new(0x7000a25be0, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x339be0 sub_339be0
    Lifted::new(0x7000a25ca0, tpl_515::<0x8, 0x1, 0x7000a25940, 0xc> as *const (), 0), // +0x339ca0 sub_339ca0
    Lifted::new(0x7000a25cf0, tpl_515::<0x8, 0x1, 0x7000a259e0, 0xc> as *const (), 0), // +0x339cf0 sub_339cf0
    Lifted::new(0x7000a25d40, fn_7000a25d40 as *const (), 2), // +0x339d40 sub_339d40
    Lifted::new(0x7000a25fc0, tpl_15 as *const (), 1), // +0x339fc0 sub_339fc0
    Lifted::new(0x7000a25fd0, fn_7000a25fd0 as *const (), 3), // +0x339fd0 sub_339fd0
    Lifted::new(0x7000a26070, fn_7000a26070 as *const (), 2), // +0x33a070 sub_33a070
    Lifted::new(0x7000a26270, fn_7000a26270 as *const (), 0), // +0x33a270 sub_33a270
    Lifted::new(0x7000a26480, tpl_17::<0x700078ae90> as *const (), 0), // +0x33a480 sub_33a480
    Lifted::new(0x7000a26490, fn_7000a26490 as *const (), 2), // +0x33a490 sub_33a490
    Lifted::new(0x7000a26cc0, fn_7000a26cc0 as *const (), 0), // +0x33acc0 sub_33acc0
    Lifted::new(0x7000a26cd0, tpl_487::<0x70007329a0> as *const (), 0), // +0x33acd0 sub_33acd0
    Lifted::new(0x7000a26d10, fn_7000a26d10 as *const (), 1), // +0x33ad10 sub_33ad10
    Lifted::new(0x7000a26de0, fn_7000a26de0 as *const (), 0), // +0x33ade0 sub_33ade0
    Lifted::new(0x7000a26ec0, fn_7000a26ec0 as *const (), 0), // +0x33aec0 sub_33aec0
    Lifted::new(0x7000a26fe0, fn_7000a26fe0 as *const (), 0), // +0x33afe0 sub_33afe0
    Lifted::new(0x7000a27070, fn_7000a27070 as *const (), 2), // +0x33b070 sub_33b070
    Lifted::new(0x7000a27360, fn_7000a27360 as *const (), 0), // +0x33b360 sub_33b360
    Lifted::new(0x7000a27560, fn_7000a27560 as *const (), 2), // +0x33b560 sub_33b560
    Lifted::new(0x7000a27a30, fn_7000a27a30 as *const (), 2), // +0x33ba30 sub_33ba30
    Lifted::new(0x7000a27cf0, fn_7000a27cf0 as *const (), 2), // +0x33bcf0 sub_33bcf0
    Lifted::new(0x7000a27fa0, fn_7000a27fa0 as *const (), 0), // +0x33bfa0 sub_33bfa0
    Lifted::new(0x7000a28140, fn_7000a28140 as *const (), 0), // +0x33c140 sub_33c140
    Lifted::new(0x7000a28370, fn_7000a28370 as *const (), 0), // +0x33c370 sub_33c370
    Lifted::new(0x7000a28510, fn_7000a28510 as *const (), 0), // +0x33c510 sub_33c510
    Lifted::new(0x7000a28620, fn_7000a28620 as *const (), 2), // +0x33c620 sub_33c620
    Lifted::new(0x7000a28a00, fn_7000a28a00 as *const (), 2), // +0x33ca00 sub_33ca00
    Lifted::new(0x7000a29020, tpl_516::<0x38, 0x23, 0xa0, 0x7000f2f6a0, 0x1f, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x33d020 sub_33d020
    Lifted::new(0x7000a29100, fn_7000a29100 as *const (), 0), // +0x33d100 sub_33d100
    Lifted::new(0x7000a291c0, tpl_517::<0x7001c7220f, 0x70009c8bf0, 0x70017d3ea0, 0x70017a0290, 0x7001d54ef8, 0x7000a292c0, 0x70009c31b0, 0x7000745eb0> as *const (), 0), // +0x33d1c0 sub_33d1c0
    Lifted::new(0x7000a29380, tpl_517::<0x7001c7220f, 0x70009c9210, 0x70017d3ea0, 0x70017a0290, 0x7001d54ef8, 0x7000a29480, 0x70009c31b0, 0x7000745eb0> as *const (), 0), // +0x33d380 sub_33d380
    Lifted::new(0x7000a29540, fn_7000a29540 as *const (), 0), // +0x33d540 sub_33d540
    Lifted::new(0x7000a29660, fn_7000a29660 as *const (), 0), // +0x33d660 sub_33d660
    Lifted::new(0x7000a29750, tpl_518::<0xa628, 0xdd08> as *const (), 1), // +0x33d750 sub_33d750
    Lifted::new(0x7000a29770, fn_7000a29770 as *const (), 0), // +0x33d770 sub_33d770
    Lifted::new(0x7000a29870, fn_7000a29870 as *const (), 0), // +0x33d870 sub_33d870
    Lifted::new(0x7000a29940, fn_7000a29940 as *const (), 0), // +0x33d940 sub_33d940
    Lifted::new(0x7000a29ba0, tpl_49::<0xd930> as *const (), 1), // +0x33dba0 sub_33dba0
    Lifted::new(0x7000a29bb0, tpl_51::<0xc890> as *const (), 1), // +0x33dbb0 sub_33dbb0
    Lifted::new(0x7000a29bc0, fn_7000a29bc0 as *const (), 2), // +0x33dbc0 sub_33dbc0
    Lifted::new(0x7000a29ff0, tpl_144 as *const (), 1), // +0x33dff0 sub_33dff0
    Lifted::new(0x7000a2a000, fn_7000a2a000 as *const (), 2), // +0x33e000 sub_33e000
    Lifted::new(0x7000a2a580, fn_7000a2a580 as *const (), 2), // +0x33e580 sub_33e580
    Lifted::new(0x7000a2aea0, fn_7000a2aea0 as *const (), 0), // +0x33eea0 sub_33eea0
    Lifted::new(0x7000a2aec0, fn_7000a2aec0 as *const (), 0), // +0x33eec0 sub_33eec0
    Lifted::new(0x7000a2b050, fn_7000a2b050 as *const (), 1), // +0x33f050 sub_33f050
    Lifted::new(0x7000a2b0a0, fn_7000a2b0a0 as *const (), 3), // +0x33f0a0 sub_33f0a0
    Lifted::new(0x7000a2b0b0, fn_7000a2b0b0 as *const (), 0), // +0x33f0b0 sub_33f0b0
    Lifted::new(0x7000a2b1c0, tpl_143::<0xa628, 0x7000d914a0> as *const (), 0), // +0x33f1c0 sub_33f1c0
    Lifted::new(0x7000a2b1d0, fn_7000a2b1d0 as *const (), 0), // +0x33f1d0 sub_33f1d0
    Lifted::new(0x7000a2b1f0, fn_7000a2b1f0 as *const (), 2), // +0x33f1f0 sub_33f1f0
    Lifted::new(0x7000a2b2b0, fn_7000a2b2b0 as *const (), 2), // +0x33f2b0 sub_33f2b0
    Lifted::new(0x7000a2b3f0, tpl_430::<0x70007665a0> as *const (), 0), // +0x33f3f0 sub_33f3f0
    Lifted::new(0x7000a2b400, tpl_430::<0x70007189b0> as *const (), 0), // +0x33f400 sub_33f400
    Lifted::new(0x7000a2b410, fn_7000a2b410 as *const (), 0), // +0x33f410 sub_33f410
    Lifted::new(0x7000a2b430, tpl_519::<0x10b, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x33f430 sub_33f430
    Lifted::new(0x7000a2b500, fn_7000a2b500 as *const (), 0), // +0x33f500 sub_33f500
    Lifted::new(0x7000a2b530, tpl_520::<0xc860, 0x7000a2b538, 0x7000a2b539, 0x7000a2b53a, 0x7000a2b53b, 0x7000a2b53c, 0x7000a2b53d, 0x7000a2b53e, 0x7000a2b53f, 0x70007916d0> as *const (), 0), // +0x33f530 sub_33f530
    Lifted::new(0x7000a2b550, fn_7000a2b550 as *const (), 2), // +0x33f550 sub_33f550
    Lifted::new(0x7000a2b590, fn_7000a2b590 as *const (), 0), // +0x33f590 sub_33f590
    Lifted::new(0x7000a2b5c0, fn_7000a2b5c0 as *const (), 0), // +0x33f5c0 sub_33f5c0
    Lifted::new(0x7000a2b5f0, tpl_51::<0x9e00> as *const (), 1), // +0x33f5f0 sub_33f5f0
    Lifted::new(0x7000a2b600, fn_7000a2b600 as *const (), 0), // +0x33f600 sub_33f600
    Lifted::new(0x7000a2b800, fn_7000a2b800 as *const (), 0), // +0x33f800 sub_33f800
    Lifted::new(0x7000a2ba70, fn_7000a2ba70 as *const (), 0), // +0x33fa70 sub_33fa70
    Lifted::new(0x7000a2bac0, fn_7000a2bac0 as *const (), 1), // +0x33fac0 sub_33fac0
    Lifted::new(0x7000a2bb10, tpl_516::<0x38, 0x23, 0xa0, 0x7000f2f6a0, 0x13, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x33fb10 sub_33fb10
    Lifted::new(0x7000a2bbf0, tpl_521::<0x28, 0x20, 0xffffffffffffffff, 0x18, 0x8> as *const (), 1), // +0x33fbf0 sub_33fbf0
    Lifted::new(0x7000a2bc70, tpl_516::<0x38, 0x23, 0xa0, 0x7000f2f6a0, 0x17, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x33fc70 sub_33fc70
    Lifted::new(0x7000a2bd50, fn_7000a2bd50 as *const (), 2), // +0x33fd50 sub_33fd50
    Lifted::new(0x7000a2bfc0, fn_7000a2bfc0 as *const (), 2), // +0x33ffc0 sub_33ffc0
    Lifted::new(0x7000a2c120, fn_7000a2c120 as *const (), 0), // +0x340120 sub_340120
    Lifted::new(0x7000a2c260, fn_7000a2c260 as *const (), 0), // +0x340260 sub_340260
    Lifted::new(0x7000a2c330, fn_7000a2c330 as *const (), 1), // +0x340330 sub_340330
    Lifted::new(0x7000a2c350, fn_7000a2c350 as *const (), 0), // +0x340350 sub_340350
    Lifted::new(0x7000a2c3a0, tpl_369::<0x79> as *const (), 1), // +0x3403a0 sub_3403a0
    Lifted::new(0x7000a2c3b0, fn_7000a2c3b0 as *const (), 0), // +0x3403b0 sub_3403b0
    Lifted::new(0x7000a2c450, fn_7000a2c450 as *const (), 0), // +0x340450 sub_340450
    Lifted::new(0x7000a2c4c0, fn_7000a2c4c0 as *const (), 0), // +0x3404c0 sub_3404c0
    Lifted::new(0x7000a2c5c0, fn_7000a2c5c0 as *const (), 0), // +0x3405c0 sub_3405c0
    Lifted::new(0x7000a2c640, fn_7000a2c640 as *const (), 0), // +0x340640 sub_340640
    Lifted::new(0x7000a2c660, fn_7000a2c660 as *const (), 1), // +0x340660 sub_340660
    Lifted::new(0x7000a2c670, tpl_51::<0xc888> as *const (), 1), // +0x340670 sub_340670
    Lifted::new(0x7000a2c680, fn_7000a2c680 as *const (), 0), // +0x340680 sub_340680
    Lifted::new(0x7000a2c6d0, fn_7000a2c6d0 as *const (), 1), // +0x3406d0 sub_3406d0
    Lifted::new(0x7000a2c700, fn_7000a2c700 as *const (), 0), // +0x340700 sub_340700
    Lifted::new(0x7000a2c880, tpl_516::<0x38, 0x23, 0xa0, 0x7000f2f6a0, 0xf, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x340880 sub_340880
    Lifted::new(0x7000a2c960, fn_7000a2c960 as *const (), 0), // +0x340960 sub_340960
    Lifted::new(0x7000a2ca10, fn_7000a2ca10 as *const (), 0), // +0x340a10 sub_340a10
    Lifted::new(0x7000a2ca90, fn_7000a2ca90 as *const (), 0), // +0x340a90 sub_340a90
    Lifted::new(0x7000a2cc50, tpl_522::<0x79> as *const (), 1), // +0x340c50 sub_340c50
    Lifted::new(0x7000a2cc60, tpl_520::<0xc868, 0x7000a2cc68, 0x7000a2cc69, 0x7000a2cc6a, 0x7000a2cc6b, 0x7000a2cc6c, 0x7000a2cc6d, 0x7000a2cc6e, 0x7000a2cc6f, 0x7000761c30> as *const (), 0), // +0x340c60 sub_340c60
    Lifted::new(0x7000a2cc80, fn_7000a2cc80 as *const (), 2), // +0x340c80 sub_340c80
    Lifted::new(0x7000a2cfe0, fn_7000a2cfe0 as *const (), 2), // +0x340fe0 sub_340fe0
    Lifted::new(0x7000a2d410, tpl_427::<0x70007215f0> as *const (), 0), // +0x341410 sub_341410
    Lifted::new(0x7000a2d420, fn_7000a2d420 as *const (), 2), // +0x341420 sub_341420
    Lifted::new(0x7000a2d7b0, fn_7000a2d7b0 as *const (), 0), // +0x3417b0 sub_3417b0
    Lifted::new(0x7000a2dab0, tpl_516::<0x38, 0x23, 0xa0, 0x7000f2f6a0, 0x3, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x341ab0 sub_341ab0
    Lifted::new(0x7000a2dbb0, fn_7000a2dbb0 as *const (), 0), // +0x341bb0 sub_341bb0
    Lifted::new(0x7000a2dbc0, tpl_15 as *const (), 1), // +0x341bc0 sub_341bc0
    Lifted::new(0x7000a2dbd0, tpl_51::<0x60> as *const (), 1), // +0x341bd0 sub_341bd0
    Lifted::new(0x7000a2dbe0, tpl_519::<0x2ab, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x341be0 sub_341be0
    Lifted::new(0x7000a2dcb0, tpl_520::<0xc8a8, 0x7000a2dcb8, 0x7000a2dcb9, 0x7000a2dcba, 0x7000a2dcbb, 0x7000a2dcbc, 0x7000a2dcbd, 0x7000a2dcbe, 0x7000a2dcbf, 0x7000d2da40> as *const (), 0), // +0x341cb0 sub_341cb0
    Lifted::new(0x7000a2dcd0, fn_7000a2dcd0 as *const (), 0), // +0x341cd0 sub_341cd0
    Lifted::new(0x7000a2dd10, tpl_499::<0x8, 0x70017d8b28, 0x70017a0290, 0x7001d54ef8, 0x7000a2dd31, 0xa718, 0x700079cfc0> as *const (), 0), // +0x341d10 sub_341d10
    Lifted::new(0x7000a2dd50, fn_7000a2dd50 as *const (), 1), // +0x341d50 sub_341d50
    Lifted::new(0x7000a2dd70, tpl_371::<0x41> as *const (), 1), // +0x341d70 sub_341d70
    Lifted::new(0x7000a2dd80, fn_7000a2dd80 as *const (), 2), // +0x341d80 sub_341d80
    Lifted::new(0x7000a2e080, tpl_51::<0x8> as *const (), 1), // +0x342080 sub_342080
    Lifted::new(0x7000a2e090, fn_7000a2e090 as *const (), 0), // +0x342090 sub_342090
    Lifted::new(0x7000a2e0b0, fn_7000a2e0b0 as *const (), 1), // +0x3420b0 sub_3420b0
    Lifted::new(0x7000a2e0d0, fn_7000a2e0d0 as *const (), 0), // +0x3420d0 sub_3420d0
    Lifted::new(0x7000a2e110, tpl_369::<0x28> as *const (), 1), // +0x342110 sub_342110
    Lifted::new(0x7000a2e120, tpl_51::<0xc8c8> as *const (), 1), // +0x342120 sub_342120
    Lifted::new(0x7000a2e130, tpl_51::<0xc8a0> as *const (), 1), // +0x342130 sub_342130
    Lifted::new(0x7000a2e140, tpl_51::<0xc8b8> as *const (), 1), // +0x342140 sub_342140
    Lifted::new(0x7000a2e150, tpl_51::<0xc898> as *const (), 1), // +0x342150 sub_342150
    Lifted::new(0x7000a2e160, tpl_51::<0xc8c0> as *const (), 1), // +0x342160 sub_342160
    Lifted::new(0x7000a2e170, tpl_51::<0xc8b0> as *const (), 1), // +0x342170 sub_342170
    Lifted::new(0x7000a2e180, fn_7000a2e180 as *const (), 3), // +0x342180 sub_342180
    Lifted::new(0x7000a2e1a0, tpl_15 as *const (), 1), // +0x3421a0 sub_3421a0
    Lifted::new(0x7000a2e1b0, fn_7000a2e1b0 as *const (), 1), // +0x3421b0 sub_3421b0
    Lifted::new(0x7000a2e1c0, fn_7000a2e1c0 as *const (), 0), // +0x3421c0 sub_3421c0
    Lifted::new(0x7000a2e210, fn_7000a2e210 as *const (), 0), // +0x342210 sub_342210
    Lifted::new(0x7000a2e250, tpl_523::<0xb, 0x7> as *const (), 1), // +0x342250 sub_342250
    Lifted::new(0x7000a2e280, tpl_523::<0xb, 0x7> as *const (), 1), // +0x342280 sub_342280
    Lifted::new(0x7000a2e2b0, tpl_476::<0x10c> as *const (), 1), // +0x3422b0 sub_3422b0
    Lifted::new(0x7000a2e2c0, fn_7000a2e2c0 as *const (), 0), // +0x3422c0 sub_3422c0
    Lifted::new(0x7000a2e3d0, fn_7000a2e3d0 as *const (), 0), // +0x3423d0 sub_3423d0
    Lifted::new(0x7000a2e4a0, fn_7000a2e4a0 as *const (), 0), // +0x3424a0 sub_3424a0
    Lifted::new(0x7000a2e5c0, fn_7000a2e5c0 as *const (), 0), // +0x3425c0 sub_3425c0
    Lifted::new(0x7000a2e720, fn_7000a2e720 as *const (), 2), // +0x342720 sub_342720
    Lifted::new(0x7000a2e930, fn_7000a2e930 as *const (), 0), // +0x342930 sub_342930
    Lifted::new(0x7000a2ea10, fn_7000a2ea10 as *const (), 2), // +0x342a10 sub_342a10
    Lifted::new(0x7000a2efa0, fn_7000a2efa0 as *const (), 0), // +0x342fa0 sub_342fa0
    Lifted::new(0x7000a2f100, fn_7000a2f100 as *const (), 0), // +0x343100 sub_343100
    Lifted::new(0x7000a2f1e0, fn_7000a2f1e0 as *const (), 0), // +0x3431e0 sub_3431e0
    Lifted::new(0x7000a2f240, fn_7000a2f240 as *const (), 0), // +0x343240 sub_343240
    Lifted::new(0x7000a2f2a0, fn_7000a2f2a0 as *const (), 0), // +0x3432a0 sub_3432a0
    Lifted::new(0x7000a2f3b0, fn_7000a2f3b0 as *const (), 2), // +0x3433b0 sub_3433b0
    Lifted::new(0x7000a2f4a0, tpl_432::<0x700073a760> as *const (), 0), // +0x3434a0 sub_3434a0
    Lifted::new(0x7000a2f4b0, fn_7000a2f4b0 as *const (), 0), // +0x3434b0 sub_3434b0
    Lifted::new(0x7000a2f4e0, tpl_487::<0x7000764de0> as *const (), 0), // +0x3434e0 sub_3434e0
    Lifted::new(0x7000a2f510, tpl_189::<0x700185bef0> as *const (), 1), // +0x343510 sub_343510
    Lifted::new(0x7000a2f530, fn_7000a2f530 as *const (), 0), // +0x343530 sub_343530
    Lifted::new(0x7000a2f5c0, fn_7000a2f5c0 as *const (), 0), // +0x3435c0 sub_3435c0
    Lifted::new(0x7000a2f610, tpl_186::<0x7001c733c0> as *const (), 1), // +0x343610 sub_343610
    Lifted::new(0x7000a2f630, fn_7000a2f630 as *const (), 2), // +0x343630 sub_343630
    Lifted::new(0x7000a2fab0, tpl_524 as *const (), 1), // +0x343ab0 sub_343ab0
    Lifted::new(0x7000a2fae0, fn_7000a2fae0 as *const (), 0), // +0x343ae0 sub_343ae0
    Lifted::new(0x7000a2fb10, fn_7000a2fb10 as *const (), 2), // +0x343b10 sub_343b10
    Lifted::new(0x7000a2fd70, tpl_525::<0x7000f911b0> as *const (), 0), // +0x343d70 sub_343d70
    Lifted::new(0x7000a2fd90, fn_7000a2fd90 as *const (), 1), // +0x343d90 sub_343d90
    Lifted::new(0x7000a2fdd0, fn_7000a2fdd0 as *const (), 1), // +0x343dd0 sub_343dd0
    Lifted::new(0x7000a2fe30, fn_7000a2fe30 as *const (), 0), // +0x343e30 sub_343e30
    Lifted::new(0x7000a2fe50, fn_7000a2fe50 as *const (), 2), // +0x343e50 sub_343e50
    Lifted::new(0x7000a30320, fn_7000a30320 as *const (), 2), // +0x344320 sub_344320
    Lifted::new(0x7000a307e0, fn_7000a307e0 as *const (), 0), // +0x3447e0 sub_3447e0
    Lifted::new(0x7000a30890, fn_7000a30890 as *const (), 0), // +0x344890 sub_344890
    Lifted::new(0x7000a30930, fn_7000a30930 as *const (), 0), // +0x344930 sub_344930
    Lifted::new(0x7000a309b0, fn_7000a309b0 as *const (), 1), // +0x3449b0 sub_3449b0
    Lifted::new(0x7000a309c0, fn_7000a309c0 as *const (), 0), // +0x3449c0 sub_3449c0
    Lifted::new(0x7000a30a00, fn_7000a30a00 as *const (), 2), // +0x344a00 sub_344a00
    Lifted::new(0x7000a30f70, fn_7000a30f70 as *const (), 2), // +0x344f70 sub_344f70
    Lifted::new(0x7000a313e0, fn_7000a313e0 as *const (), 0), // +0x3453e0 sub_3453e0
    Lifted::new(0x7000a31440, fn_7000a31440 as *const (), 2), // +0x345440 sub_345440
    Lifted::new(0x7000a31850, fn_7000a31850 as *const (), 2), // +0x345850 sub_345850
    Lifted::new(0x7000a31cb0, fn_7000a31cb0 as *const (), 2), // +0x345cb0 sub_345cb0
    Lifted::new(0x7000a32110, fn_7000a32110 as *const (), 0), // +0x346110 sub_346110
    Lifted::new(0x7000a32140, tpl_449::<0x7001c71e94, 0x7001d55088, 0xc860, 0x70017d4690, 0x70017d46a8, 0xc7e2, 0x70017d43e8, 0x7001d54d30, 0x7001d55090, 0x7000a321a4> as *const (), 0), // +0x346140 sub_346140
    Lifted::new(0x7000a321c0, fn_7000a321c0 as *const (), 2), // +0x3461c0 sub_3461c0
    Lifted::new(0x7000a32680, fn_7000a32680 as *const (), 2), // +0x346680 sub_346680
    Lifted::new(0x7000a32b40, fn_7000a32b40 as *const (), 2), // +0x346b40 sub_346b40
    Lifted::new(0x7000a32ff0, fn_7000a32ff0 as *const (), 0), // +0x346ff0 sub_346ff0
    Lifted::new(0x7000a33020, fn_7000a33020 as *const (), 0), // +0x347020 sub_347020
    Lifted::new(0x7000a33100, fn_7000a33100 as *const (), 2), // +0x347100 sub_347100
    Lifted::new(0x7000a33550, tpl_433::<0x7000dbb3b0> as *const (), 0), // +0x347550 sub_347550
    Lifted::new(0x7000a33580, tpl_526::<0x40, 0x70017d5810, 0x70017a0290, 0x7001d54ef8, 0x7000a335a1, 0x7000f3b800> as *const (), 0), // +0x347580 sub_347580
    Lifted::new(0x7000a335c0, tpl_507::<0x7000f3b930> as *const (), 0), // +0x3475c0 sub_3475c0
    Lifted::new(0x7000a335d0, tpl_526::<0x40, 0x70017d5810, 0x70017a0290, 0x7001d54ef8, 0x7000a335f1, 0x7000f3b800> as *const (), 0), // +0x3475d0 sub_3475d0
    Lifted::new(0x7000a33610, fn_7000a33610 as *const (), 1), // +0x347610 sub_347610
    Lifted::new(0x7000a33650, fn_7000a33650 as *const (), 0), // +0x347650 sub_347650
    Lifted::new(0x7000a33670, fn_7000a33670 as *const (), 2), // +0x347670 sub_347670
    Lifted::new(0x7000a338d0, fn_7000a338d0 as *const (), 2), // +0x3478d0 sub_3478d0
    Lifted::new(0x7000a339f0, fn_7000a339f0 as *const (), 2), // +0x3479f0 sub_3479f0
    Lifted::new(0x7000a33d30, tpl_433::<0x7000db6760> as *const (), 0), // +0x347d30 sub_347d30
    Lifted::new(0x7000a33d60, fn_7000a33d60 as *const (), 0), // +0x347d60 sub_347d60
    Lifted::new(0x7000a33de0, tpl_527::<0x28, 0x8> as *const (), 0), // +0x347de0 sub_347de0
    Lifted::new(0x7000a33e00, tpl_527::<0x28, 0x8> as *const (), 0), // +0x347e00 sub_347e00
    Lifted::new(0x7000a33e20, fn_7000a33e20 as *const (), 0), // +0x347e20 sub_347e20
    Lifted::new(0x7000a33f80, fn_7000a33f80 as *const (), 0), // +0x347f80 sub_347f80
    Lifted::new(0x7000a34060, fn_7000a34060 as *const (), 0), // +0x348060 sub_348060
    Lifted::new(0x7000a34090, fn_7000a34090 as *const (), 0), // +0x348090 sub_348090
    Lifted::new(0x7000a34140, fn_7000a34140 as *const (), 0), // +0x348140 sub_348140
    Lifted::new(0x7000a34170, fn_7000a34170 as *const (), 2), // +0x348170 sub_348170
    Lifted::new(0x7000a34370, fn_7000a34370 as *const (), 0), // +0x348370 sub_348370
    Lifted::new(0x7000a344c0, tpl_528::<0x120, 0x128, 0x38, 0x4, 0x30, 0x7000a24fa0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x700075ff30> as *const (), 0), // +0x3484c0 sub_3484c0
    Lifted::new(0x7000a345b0, fn_7000a345b0 as *const (), 0), // +0x3485b0 sub_3485b0
    Lifted::new(0x7000a34610, tpl_15 as *const (), 1), // +0x348610 sub_348610
    Lifted::new(0x7000a34620, fn_7000a34620 as *const (), 0), // +0x348620 sub_348620
    Lifted::new(0x7000a34770, fn_7000a34770 as *const (), 1), // +0x348770 sub_348770
    Lifted::new(0x7000a347c0, fn_7000a347c0 as *const (), 0), // +0x3487c0 sub_3487c0
    Lifted::new(0x7000a34890, fn_7000a34890 as *const (), 1), // +0x348890 sub_348890
    Lifted::new(0x7000a348b0, fn_7000a348b0 as *const (), 1), // +0x3488b0 sub_3488b0
    Lifted::new(0x7000a348d0, fn_7000a348d0 as *const (), 3), // +0x3488d0 sub_3488d0
    Lifted::new(0x7000a34900, fn_7000a34900 as *const (), 1), // +0x348900 sub_348900
    Lifted::new(0x7000a34920, fn_7000a34920 as *const (), 2), // +0x348920 sub_348920
    Lifted::new(0x7000a34a80, fn_7000a34a80 as *const (), 2), // +0x348a80 sub_348a80
    Lifted::new(0x7000a34b70, fn_7000a34b70 as *const (), 2), // +0x348b70 sub_348b70
    Lifted::new(0x7000a34ca0, fn_7000a34ca0 as *const (), 2), // +0x348ca0 sub_348ca0
    Lifted::new(0x7000a34dd0, fn_7000a34dd0 as *const (), 2), // +0x348dd0 sub_348dd0
    Lifted::new(0x7000a34f30, fn_7000a34f30 as *const (), 2), // +0x348f30 sub_348f30
    Lifted::new(0x7000a35090, fn_7000a35090 as *const (), 0), // +0x349090 sub_349090
    Lifted::new(0x7000a350c0, fn_7000a350c0 as *const (), 1), // +0x3490c0 sub_3490c0
    Lifted::new(0x7000a351a0, fn_7000a351a0 as *const (), 0), // +0x3491a0 sub_3491a0
    Lifted::new(0x7000a35250, fn_7000a35250 as *const (), 0), // +0x349250 sub_349250
    Lifted::new(0x7000a35370, tpl_15 as *const (), 1), // +0x349370 sub_349370
    Lifted::new(0x7000a35380, tpl_15 as *const (), 1), // +0x349380 sub_349380
    Lifted::new(0x7000a35390, tpl_430::<0x7000c622d0> as *const (), 0), // +0x349390 sub_349390
    Lifted::new(0x7000a353a0, tpl_143::<0xa718, 0x700070fc90> as *const (), 0), // +0x3493a0 sub_3493a0
    Lifted::new(0x7000a353b0, tpl_15 as *const (), 1), // +0x3493b0 sub_3493b0
    Lifted::new(0x7000a353c0, fn_7000a353c0 as *const (), 3), // +0x3493c0 sub_3493c0
    Lifted::new(0x7000a35550, tpl_15 as *const (), 1), // +0x349550 sub_349550
    Lifted::new(0x7000a35560, tpl_15 as *const (), 1), // +0x349560 sub_349560
    Lifted::new(0x7000a35570, tpl_143::<0xa718, 0x70006f10f0> as *const (), 0), // +0x349570 sub_349570
    Lifted::new(0x7000a35580, tpl_16::<0x7000a35583, 0x7000a35584, 0x7000a35585, 0x7000a35586, 0x7000a35587, 0x7000a35588, 0x7000a35589, 0x7000a3558a, 0x7000a3558b, 0x7000a3558c, 0x7000a3558d, 0x7000a3558e, 0x7000a3558f, 0x7000bb0a20> as *const (), 0), // +0x349580 sub_349580
    Lifted::new(0x7000a355a0, fn_7000a355a0 as *const (), 2), // +0x3495a0 sub_3495a0
    Lifted::new(0x7000a35800, fn_7000a35800 as *const (), 2), // +0x349800 sub_349800
    Lifted::new(0x7000a35a60, fn_7000a35a60 as *const (), 2), // +0x349a60 sub_349a60
    Lifted::new(0x7000a35cc0, fn_7000a35cc0 as *const (), 1), // +0x349cc0 sub_349cc0
    Lifted::new(0x7000a35d00, tpl_433::<0x7000dbbaa0> as *const (), 0), // +0x349d00 sub_349d00
    Lifted::new(0x7000a35d30, fn_7000a35d30 as *const (), 2), // +0x349d30 sub_349d30
    Lifted::new(0x7000a35f00, tpl_507::<0x7000f3ba50> as *const (), 0), // +0x349f00 sub_349f00
    Lifted::new(0x7000a35f10, tpl_507::<0x7000f3ba90> as *const (), 0), // +0x349f10 sub_349f10
    Lifted::new(0x7000a35f20, tpl_469::<0x2fe8, 0xffffffffffffffff, 0x13, 0x36b, 0xa6f0, 0x70006f2a70, 0xa6d8, 0xa6e0, 0x70006f9a60, 0x8, 0x70006fae30, 0x7000774720> as *const (), 0), // +0x349f20 sub_349f20
    Lifted::new(0x7000a36040, fn_7000a36040 as *const (), 2), // +0x34a040 sub_34a040
    Lifted::new(0x7000a36210, tpl_507::<0x7000f3fc40> as *const (), 0), // +0x34a210 sub_34a210
    Lifted::new(0x7000a36220, tpl_507::<0x7000f348e0> as *const (), 0), // +0x34a220 sub_34a220
    Lifted::new(0x7000a36230, tpl_507::<0x7000f3ff60> as *const (), 0), // +0x34a230 sub_34a230
    Lifted::new(0x7000a36240, fn_7000a36240 as *const (), 2), // +0x34a240 sub_34a240
    Lifted::new(0x7000a36400, fn_7000a36400 as *const (), 2), // +0x34a400 sub_34a400
    Lifted::new(0x7000a36670, tpl_529::<0x10, 0xfffffffffffffff8, 0x700177c312, 0x8, 0x7001d54968, 0x7000a366f3> as *const (), 0), // +0x34a670 sub_34a670
    Lifted::new(0x7000a36720, tpl_530::<0x10> as *const (), 0), // +0x34a720 sub_34a720
    Lifted::new(0x7000a36740, tpl_530::<0x10> as *const (), 0), // +0x34a740 sub_34a740
    Lifted::new(0x7000a36760, tpl_531::<0x10, 0x7000773c40, 0x700177c312> as *const (), 0), // +0x34a760 sub_34a760
    Lifted::new(0x7000a367a0, tpl_531::<0x10, 0x7000773c40, 0x700177c312> as *const (), 0), // +0x34a7a0 sub_34a7a0
    Lifted::new(0x7000a367e0, tpl_532::<0x8, 0x7001c6b048, 0x700177c100, 0x10> as *const (), 0), // +0x34a7e0 sub_34a7e0
    Lifted::new(0x7000a36820, tpl_532::<0x8, 0x7001c6aff8, 0x700177c100, 0x10> as *const (), 0), // +0x34a820 sub_34a820
    Lifted::new(0x7000a36860, tpl_287 as *const (), 1), // +0x34a860 sub_34a860
    Lifted::new(0x7000a36b80, fn_7000a36b80 as *const (), 0), // +0x34ab80 sub_34ab80
    Lifted::new(0x7000a36d80, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36d90> as *const (), 0), // +0x34ad80 sub_34ad80
    Lifted::new(0x7000a36da0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36db0> as *const (), 0), // +0x34ada0 sub_34ada0
    Lifted::new(0x7000a36dc0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36dd0> as *const (), 0), // +0x34adc0 sub_34adc0
    Lifted::new(0x7000a36de0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36df0> as *const (), 0), // +0x34ade0 sub_34ade0
    Lifted::new(0x7000a36e00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36e10> as *const (), 0), // +0x34ae00 sub_34ae00
    Lifted::new(0x7000a36e20, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36e30> as *const (), 0), // +0x34ae20 sub_34ae20
    Lifted::new(0x7000a36e40, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36e50> as *const (), 0), // +0x34ae40 sub_34ae40
    Lifted::new(0x7000a36e60, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36e70> as *const (), 0), // +0x34ae60 sub_34ae60
    Lifted::new(0x7000a36e80, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36e90> as *const (), 0), // +0x34ae80 sub_34ae80
    Lifted::new(0x7000a36ea0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a36eb0> as *const (), 0), // +0x34aea0 sub_34aea0
    Lifted::new(0x7000a36ec0, tpl_292::<0x70017d8bf8, 0x70017790f0, 0x7000a36ed0> as *const (), 0), // +0x34aec0 sub_34aec0
    Lifted::new(0x7000a36ee0, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000a36ef0> as *const (), 0), // +0x34aee0 sub_34aee0
    Lifted::new(0x7000a36f00, tpl_292::<0x70017d3748, 0x70017790f0, 0x7000a36f10> as *const (), 0), // +0x34af00 sub_34af00
    Lifted::new(0x7000a36f20, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000a36f74, 0x70007f0910, 0x7000a36f8f> as *const (), 0), // +0x34af20 sub_34af20
    Lifted::new(0x7000a36fb0, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000a36ffd, 0x70007f0910, 0x7000a3701b> as *const (), 0), // +0x34afb0 sub_34afb0
    Lifted::new(0x7000a37040, tpl_299::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000a3708d, 0x70007f0910, 0x7000a370ab> as *const (), 0), // +0x34b040 sub_34b040
    Lifted::new(0x7000a370d0, tpl_301::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000a37124, 0x70007f0910, 0x7000a3713f> as *const (), 0), // +0x34b0d0 sub_34b0d0
    Lifted::new(0x7000a37160, tpl_36::<0x30> as *const (), 1), // +0x34b160 sub_34b160
    Lifted::new(0x7000a37170, tpl_20 as *const (), 1), // +0x34b170 sub_34b170
    Lifted::new(0x7000a37180, tpl_47::<0x139> as *const (), 1), // +0x34b180 sub_34b180
    Lifted::new(0x7000a37190, tpl_36::<0xc0> as *const (), 1), // +0x34b190 sub_34b190
    Lifted::new(0x7000a371a0, tpl_36::<0x60> as *const (), 1), // +0x34b1a0 sub_34b1a0
    Lifted::new(0x7000a371b0, fn_7000a371b0 as *const (), 1), // +0x34b1b0 sub_34b1b0
    Lifted::new(0x7000a371d0, fn_7000a371d0 as *const (), 1), // +0x34b1d0 sub_34b1d0
    Lifted::new(0x7000a37210, tpl_52::<0x60> as *const (), 1), // +0x34b210 sub_34b210
    Lifted::new(0x7000a37220, tpl_36::<0x88> as *const (), 1), // +0x34b220 sub_34b220
    Lifted::new(0x7000a37230, fn_7000a37230 as *const (), 1), // +0x34b230 sub_34b230
    Lifted::new(0x7000a37240, fn_7000a37240 as *const (), 1), // +0x34b240 sub_34b240
    Lifted::new(0x7000a37270, tpl_244::<0x50> as *const (), 1), // +0x34b270 sub_34b270
    Lifted::new(0x7000a37280, tpl_36::<0x10> as *const (), 1), // +0x34b280 sub_34b280
    Lifted::new(0x7000a37290, tpl_52::<0x20> as *const (), 1), // +0x34b290 sub_34b290
    Lifted::new(0x7000a372a0, tpl_36::<0x28> as *const (), 1), // +0x34b2a0 sub_34b2a0
    Lifted::new(0x7000a372b0, tpl_36::<0x10> as *const (), 1), // +0x34b2b0 sub_34b2b0
    Lifted::new(0x7000a372c0, tpl_52::<0x18> as *const (), 1), // +0x34b2c0 sub_34b2c0
    Lifted::new(0x7000a372d0, tpl_363::<0x40> as *const (), 1), // +0x34b2d0 sub_34b2d0
    Lifted::new(0x7000a372e0, fn_7000a372e0 as *const (), 1), // +0x34b2e0 sub_34b2e0
    Lifted::new(0x7000a372f0, tpl_36::<0x10> as *const (), 1), // +0x34b2f0 sub_34b2f0
    Lifted::new(0x7000a37300, tpl_471::<0x10, 0x38> as *const (), 1), // +0x34b300 sub_34b300
    Lifted::new(0x7000a37310, tpl_30 as *const (), 1), // +0x34b310 sub_34b310
    Lifted::new(0x7000a37320, tpl_36::<0x10> as *const (), 1), // +0x34b320 sub_34b320
    Lifted::new(0x7000a37330, tpl_450::<0x7001c6fa64> as *const (), 1), // +0x34b330 sub_34b330
    Lifted::new(0x7000a37340, tpl_52::<0x50> as *const (), 1), // +0x34b340 sub_34b340
    Lifted::new(0x7000a37350, tpl_47::<0x110> as *const (), 1), // +0x34b350 sub_34b350
    Lifted::new(0x7000a37360, tpl_52::<0x10> as *const (), 1), // +0x34b360 sub_34b360
    Lifted::new(0x7000a37370, tpl_52::<0xc8> as *const (), 1), // +0x34b370 sub_34b370
    Lifted::new(0x7000a37380, tpl_335::<0x7a> as *const (), 1), // +0x34b380 sub_34b380
    Lifted::new(0x7000a37390, tpl_172::<0x70> as *const (), 1), // +0x34b390 sub_34b390
    Lifted::new(0x7000a373a0, tpl_208::<0x638> as *const (), 1), // +0x34b3a0 sub_34b3a0
    Lifted::new(0x7000a373b0, tpl_263::<0x14> as *const (), 1), // +0x34b3b0 sub_34b3b0
    Lifted::new(0x7000a373c0, tpl_208::<0x18> as *const (), 1), // +0x34b3c0 sub_34b3c0
    Lifted::new(0x7000a373d0, tpl_533::<0xf8, 0x10, 0x4> as *const (), 1), // +0x34b3d0 sub_34b3d0
    Lifted::new(0x7000a373f0, tpl_39::<0x6c> as *const (), 1), // +0x34b3f0 sub_34b3f0
    Lifted::new(0x7000a37400, tpl_36::<0x8> as *const (), 1), // +0x34b400 sub_34b400
    Lifted::new(0x7000a37410, fn_7000a37410 as *const (), 1), // +0x34b410 sub_34b410
    Lifted::new(0x7000a37420, tpl_195::<0x10> as *const (), 1), // +0x34b420 sub_34b420
    Lifted::new(0x7000a37430, tpl_39::<0x16c> as *const (), 1), // +0x34b430 sub_34b430
    Lifted::new(0x7000a37440, tpl_52::<0x170> as *const (), 1), // +0x34b440 sub_34b440
    Lifted::new(0x7000a37450, fn_7000a37450 as *const (), 1), // +0x34b450 sub_34b450
    Lifted::new(0x7000a37470, tpl_36::<0xc0> as *const (), 1), // +0x34b470 sub_34b470
    Lifted::new(0x7000a37480, tpl_275::<0x140, 0x20> as *const (), 1), // +0x34b480 sub_34b480
    Lifted::new(0x7000a37490, tpl_52::<0x68> as *const (), 1), // +0x34b490 sub_34b490
    Lifted::new(0x7000a374a0, tpl_52::<0x100> as *const (), 1), // +0x34b4a0 sub_34b4a0
    Lifted::new(0x7000a374b0, tpl_36::<0x638> as *const (), 1), // +0x34b4b0 sub_34b4b0
    Lifted::new(0x7000a374c0, tpl_47::<0x82> as *const (), 1), // +0x34b4c0 sub_34b4c0
    Lifted::new(0x7000a374d0, tpl_36::<0xb0> as *const (), 1), // +0x34b4d0 sub_34b4d0
    Lifted::new(0x7000a374e0, fn_7000a374e0 as *const (), 1), // +0x34b4e0 sub_34b4e0
    Lifted::new(0x7000a37500, tpl_534::<0x10> as *const (), 1), // +0x34b500 sub_34b500
    Lifted::new(0x7000a37520, tpl_353::<0x81> as *const (), 1), // +0x34b520 sub_34b520
    Lifted::new(0x7000a37530, fn_7000a37530 as *const (), 1), // +0x34b530 sub_34b530
    Lifted::new(0x7000a37540, fn_7000a37540 as *const (), 1), // +0x34b540 sub_34b540
    Lifted::new(0x7000a37550, tpl_535::<0xf8, 0x28, 0x30> as *const (), 1), // +0x34b550 sub_34b550
    Lifted::new(0x7000a37570, tpl_195::<0x16c> as *const (), 1), // +0x34b570 sub_34b570
    Lifted::new(0x7000a37580, tpl_172::<0x18> as *const (), 1), // +0x34b580 sub_34b580
    Lifted::new(0x7000a37590, tpl_536::<0x79> as *const (), 1), // +0x34b590 sub_34b590
    Lifted::new(0x7000a375a0, tpl_537::<0x10> as *const (), 1), // +0x34b5a0 sub_34b5a0
    Lifted::new(0x7000a375b0, tpl_325::<0x81> as *const (), 1), // +0x34b5b0 sub_34b5b0
    Lifted::new(0x7000a375c0, fn_7000a375c0 as *const (), 1), // +0x34b5c0 sub_34b5c0
    Lifted::new(0x7000a375d0, tpl_471::<0x28, 0x30> as *const (), 1), // +0x34b5d0 sub_34b5d0
    Lifted::new(0x7000a375e0, tpl_537::<0x18> as *const (), 1), // +0x34b5e0 sub_34b5e0
    Lifted::new(0x7000a375f0, tpl_193::<0x80> as *const (), 1), // +0x34b5f0 sub_34b5f0
    Lifted::new(0x7000a37600, tpl_47::<0x90> as *const (), 1), // +0x34b600 sub_34b600
    Lifted::new(0x7000a37610, tpl_335::<0x81> as *const (), 1), // +0x34b610 sub_34b610
    Lifted::new(0x7000a37620, tpl_538::<0x81> as *const (), 1), // +0x34b620 sub_34b620
    Lifted::new(0x7000a37630, tpl_537::<0x8> as *const (), 1), // +0x34b630 sub_34b630
    Lifted::new(0x7000a37640, fn_7000a37640 as *const (), 1), // +0x34b640 sub_34b640
    Lifted::new(0x7000a37650, tpl_450::<0x7001c6fa60> as *const (), 1), // +0x34b650 sub_34b650
    Lifted::new(0x7000a37660, tpl_39::<0x80> as *const (), 1), // +0x34b660 sub_34b660
    Lifted::new(0x7000a37670, tpl_20 as *const (), 1), // +0x34b670 sub_34b670
    Lifted::new(0x7000a37680, tpl_20 as *const (), 1), // +0x34b680 sub_34b680
    Lifted::new(0x7000a37690, fn_7000a37690 as *const (), 0), // +0x34b690 sub_34b690
    Lifted::new(0x7000a37830, tpl_36::<0x140> as *const (), 1), // +0x34b830 sub_34b830
    Lifted::new(0x7000a37840, tpl_36::<0x8> as *const (), 1), // +0x34b840 sub_34b840
    Lifted::new(0x7000a37850, tpl_539::<0x79> as *const (), 1), // +0x34b850 sub_34b850
    Lifted::new(0x7000a37860, tpl_540::<0x10, 0x40> as *const (), 1), // +0x34b860 sub_34b860
    Lifted::new(0x7000a37880, tpl_360::<0x10, 0x3c> as *const (), 1), // +0x34b880 sub_34b880
    Lifted::new(0x7000a37890, tpl_47::<0x92> as *const (), 1), // +0x34b890 sub_34b890
    Lifted::new(0x7000a378a0, fn_7000a378a0 as *const (), 1), // +0x34b8a0 sub_34b8a0
    Lifted::new(0x7000a378c0, tpl_325::<0x80> as *const (), 1), // +0x34b8c0 sub_34b8c0
    Lifted::new(0x7000a378d0, fn_7000a378d0 as *const (), 0), // +0x34b8d0 sub_34b8d0
    Lifted::new(0x7000a37900, fn_7000a37900 as *const (), 0), // +0x34b900 sub_34b900
    Lifted::new(0x7000a37930, tpl_541::<0x18, 0x30, 0x700073c730, 0x82, 0x100> as *const (), 0), // +0x34b930 sub_34b930
    Lifted::new(0x7000a379b0, tpl_542::<0x100, 0x18, 0x30, 0x700073c730, 0x82, 0xfffffffffffffff4> as *const (), 0), // +0x34b9b0 sub_34b9b0
    Lifted::new(0x7000a37a00, fn_7000a37a00 as *const (), 1), // +0x34ba00 sub_34ba00
    Lifted::new(0x7000a37a20, tpl_539::<0x7a> as *const (), 1), // +0x34ba20 sub_34ba20
    Lifted::new(0x7000a37a30, tpl_543::<0x78> as *const (), 1), // +0x34ba30 sub_34ba30
    Lifted::new(0x7000a37a40, tpl_544::<0x78> as *const (), 1), // +0x34ba40 sub_34ba40
    Lifted::new(0x7000a37a50, tpl_337::<0x79> as *const (), 1), // +0x34ba50 sub_34ba50
    Lifted::new(0x7000a37a60, tpl_193::<0x7a> as *const (), 1), // +0x34ba60 sub_34ba60
    Lifted::new(0x7000a37a70, tpl_545::<0x78> as *const (), 1), // +0x34ba70 sub_34ba70
    Lifted::new(0x7000a37a80, fn_7000a37a80 as *const (), 1), // +0x34ba80 sub_34ba80
    Lifted::new(0x7000a37a90, fn_7000a37a90 as *const (), 0), // +0x34ba90 sub_34ba90
    Lifted::new(0x7000a37ac0, tpl_335::<0x79> as *const (), 1), // +0x34bac0 sub_34bac0
    Lifted::new(0x7000a37ad0, fn_7000a37ad0 as *const (), 1), // +0x34bad0 sub_34bad0
    Lifted::new(0x7000a37af0, fn_7000a37af0 as *const (), 0), // +0x34baf0 sub_34baf0
    Lifted::new(0x7000a37b20, tpl_471::<0x10, 0x1c> as *const (), 1), // +0x34bb20 sub_34bb20
    Lifted::new(0x7000a37b30, fn_7000a37b30 as *const (), 1), // +0x34bb30 sub_34bb30
    Lifted::new(0x7000a37b40, tpl_471::<0x18, 0x1c> as *const (), 1), // +0x34bb40 sub_34bb40
    Lifted::new(0x7000a37b50, tpl_538::<0x79> as *const (), 1), // +0x34bb50 sub_34bb50
    Lifted::new(0x7000a37b60, tpl_47::<0x70> as *const (), 1), // +0x34bb60 sub_34bb60
    Lifted::new(0x7000a37b70, tpl_471::<0x10, 0x34> as *const (), 1), // +0x34bb70 sub_34bb70
    Lifted::new(0x7000a37b80, tpl_538::<0x7a> as *const (), 1), // +0x34bb80 sub_34bb80
    Lifted::new(0x7000a37b90, fn_7000a37b90 as *const (), 0), // +0x34bb90 sub_34bb90
    Lifted::new(0x7000a37be0, tpl_354::<0x78> as *const (), 1), // +0x34bbe0 sub_34bbe0
    Lifted::new(0x7000a37bf0, tpl_193::<0x81> as *const (), 1), // +0x34bbf0 sub_34bbf0
    Lifted::new(0x7000a37c00, fn_7000a37c00 as *const (), 0), // +0x34bc00 sub_34bc00
    Lifted::new(0x7000a37c30, fn_7000a37c30 as *const (), 1), // +0x34bc30 sub_34bc30
    Lifted::new(0x7000a37c40, tpl_30 as *const (), 1), // +0x34bc40 sub_34bc40
    Lifted::new(0x7000a37c50, tpl_30 as *const (), 1), // +0x34bc50 sub_34bc50
    Lifted::new(0x7000a37c60, tpl_328::<0x20> as *const (), 1), // +0x34bc60 sub_34bc60
    Lifted::new(0x7000a37c70, tpl_36::<0x20> as *const (), 1), // +0x34bc70 sub_34bc70
    Lifted::new(0x7000a37c80, tpl_36::<0x638> as *const (), 1), // +0x34bc80 sub_34bc80
    Lifted::new(0x7000a37c90, fn_7000a37c90 as *const (), 1), // +0x34bc90 sub_34bc90
    Lifted::new(0x7000a37ca0, tpl_193::<0x79> as *const (), 1), // +0x34bca0 sub_34bca0
    Lifted::new(0x7000a37cb0, tpl_36::<0x18> as *const (), 1), // +0x34bcb0 sub_34bcb0
    Lifted::new(0x7000a37cc0, tpl_463::<0x58, 0x8, 0x20> as *const (), 1), // +0x34bcc0 sub_34bcc0
    Lifted::new(0x7000a37ce0, fn_7000a37ce0 as *const (), 1), // +0x34bce0 sub_34bce0
    Lifted::new(0x7000a37d20, fn_7000a37d20 as *const (), 1), // +0x34bd20 sub_34bd20
    Lifted::new(0x7000a37d40, fn_7000a37d40 as *const (), 1), // +0x34bd40 sub_34bd40
    Lifted::new(0x7000a37d70, tpl_52::<0x30> as *const (), 1), // +0x34bd70 sub_34bd70
    Lifted::new(0x7000a37d80, tpl_36::<0x10> as *const (), 1), // +0x34bd80 sub_34bd80
    Lifted::new(0x7000a37d90, tpl_546::<0x18> as *const (), 1), // +0x34bd90 sub_34bd90
    Lifted::new(0x7000a37da0, tpl_275::<0x60, 0x438> as *const (), 1), // +0x34bda0 sub_34bda0
    Lifted::new(0x7000a37db0, tpl_52::<0x8> as *const (), 1), // +0x34bdb0 sub_34bdb0
    Lifted::new(0x7000a37dc0, tpl_52::<0x8> as *const (), 1), // +0x34bdc0 sub_34bdc0
    Lifted::new(0x7000a37dd0, fn_7000a37dd0 as *const (), 0), // +0x34bdd0 sub_34bdd0
    Lifted::new(0x7000a37df0, tpl_353::<0x7a> as *const (), 1), // +0x34bdf0 sub_34bdf0
    Lifted::new(0x7000a37e00, tpl_337::<0x7a> as *const (), 1), // +0x34be00 sub_34be00
    Lifted::new(0x7000a37e10, tpl_337::<0x81> as *const (), 1), // +0x34be10 sub_34be10
    Lifted::new(0x7000a37e20, tpl_36::<0xb8> as *const (), 1), // +0x34be20 sub_34be20
    Lifted::new(0x7000a37e30, tpl_36::<0x18> as *const (), 1), // +0x34be30 sub_34be30
    Lifted::new(0x7000a37e40, tpl_36::<0x18> as *const (), 1), // +0x34be40 sub_34be40
    Lifted::new(0x7000a37e50, fn_7000a37e50 as *const (), 1), // +0x34be50 sub_34be50
    Lifted::new(0x7000a37e90, fn_7000a37e90 as *const (), 1), // +0x34be90 sub_34be90
    Lifted::new(0x7000a37eb0, fn_7000a37eb0 as *const (), 1), // +0x34beb0 sub_34beb0
    Lifted::new(0x7000a37ee0, tpl_36::<0x20> as *const (), 1), // +0x34bee0 sub_34bee0
    Lifted::new(0x7000a37ef0, tpl_140::<0x18, 0x1> as *const (), 1), // +0x34bef0 sub_34bef0
    Lifted::new(0x7000a37f00, tpl_39::<0x74> as *const (), 1), // +0x34bf00 sub_34bf00
    Lifted::new(0x7000a37f10, tpl_39::<0x84> as *const (), 1), // +0x34bf10 sub_34bf10
    Lifted::new(0x7000a37f20, tpl_39::<0x70> as *const (), 1), // +0x34bf20 sub_34bf20
    Lifted::new(0x7000a37f30, tpl_39::<0x2c> as *const (), 1), // +0x34bf30 sub_34bf30
    Lifted::new(0x7000a37f40, tpl_39::<0x168> as *const (), 1), // +0x34bf40 sub_34bf40
    Lifted::new(0x7000a37f50, tpl_191::<0x160> as *const (), 1), // +0x34bf50 sub_34bf50
    Lifted::new(0x7000a37f70, tpl_41 as *const (), 1), // +0x34bf70 sub_34bf70
    Lifted::new(0x7000a37f80, tpl_30 as *const (), 1), // +0x34bf80 sub_34bf80
    Lifted::new(0x7000a37f90, tpl_176::<0x88> as *const (), 1), // +0x34bf90 sub_34bf90
    Lifted::new(0x7000a37fb0, tpl_36::<0x78> as *const (), 1), // +0x34bfb0 sub_34bfb0
    Lifted::new(0x7000a37fc0, tpl_263::<0x308> as *const (), 1), // +0x34bfc0 sub_34bfc0
    Lifted::new(0x7000a37fd0, tpl_535::<0xf8, 0x18, 0x38> as *const (), 1), // +0x34bfd0 sub_34bfd0
    Lifted::new(0x7000a37ff0, tpl_471::<0x18, 0x38> as *const (), 1), // +0x34bff0 sub_34bff0
    Lifted::new(0x7000a38000, fn_7000a38000 as *const (), 1), // +0x34c000 sub_34c000
    Lifted::new(0x7000a38020, tpl_533::<0xf8, 0x18, 0x4> as *const (), 1), // +0x34c020 sub_34c020
    Lifted::new(0x7000a38040, tpl_36::<0x18> as *const (), 1), // +0x34c040 sub_34c040
    Lifted::new(0x7000a38050, tpl_547::<0xf8, 0x18> as *const (), 1), // +0x34c050 sub_34c050
    Lifted::new(0x7000a38070, tpl_36::<0xb0> as *const (), 1), // +0x34c070 sub_34c070
    Lifted::new(0x7000a38080, tpl_36::<0xb0> as *const (), 1), // +0x34c080 sub_34c080
    Lifted::new(0x7000a38090, fn_7000a38090 as *const (), 1), // +0x34c090 sub_34c090
    Lifted::new(0x7000a380a0, tpl_191::<0x80788> as *const (), 1), // +0x34c0a0 sub_34c0a0
    Lifted::new(0x7000a380c0, tpl_52::<0xc8> as *const (), 1), // +0x34c0c0 sub_34c0c0
    Lifted::new(0x7000a380d0, tpl_547::<0xf8, 0x10> as *const (), 1), // +0x34c0d0 sub_34c0d0
    Lifted::new(0x7000a380f0, tpl_52::<0x500> as *const (), 1), // +0x34c0f0 sub_34c0f0
    Lifted::new(0x7000a38100, tpl_36::<0xc8> as *const (), 1), // +0x34c100 sub_34c100
    Lifted::new(0x7000a38110, tpl_325::<0x7a> as *const (), 1), // +0x34c110 sub_34c110
    Lifted::new(0x7000a38120, tpl_36::<0x58> as *const (), 1), // +0x34c120 sub_34c120
    Lifted::new(0x7000a38130, tpl_36::<0x50> as *const (), 1), // +0x34c130 sub_34c130
    Lifted::new(0x7000a38140, tpl_36::<0x60> as *const (), 1), // +0x34c140 sub_34c140
    Lifted::new(0x7000a38150, tpl_52::<0x20> as *const (), 1), // +0x34c150 sub_34c150
    Lifted::new(0x7000a38160, fn_7000a38160 as *const (), 1), // +0x34c160 sub_34c160
    Lifted::new(0x7000a381a0, fn_7000a381a0 as *const (), 1), // +0x34c1a0 sub_34c1a0
    Lifted::new(0x7000a381c0, fn_7000a381c0 as *const (), 1), // +0x34c1c0 sub_34c1c0
    Lifted::new(0x7000a381f0, tpl_36::<0xa8> as *const (), 1), // +0x34c1f0 sub_34c1f0
    Lifted::new(0x7000a38200, tpl_36::<0x8> as *const (), 1), // +0x34c200 sub_34c200
    Lifted::new(0x7000a38210, fn_7000a38210 as *const (), 1), // +0x34c210 sub_34c210
    Lifted::new(0x7000a38220, fn_7000a38220 as *const (), 1), // +0x34c220 sub_34c220
    Lifted::new(0x7000a38250, fn_7000a38250 as *const (), 2), // +0x34c250 sub_34c250
    Lifted::new(0x7000a384e0, tpl_36::<0x30> as *const (), 1), // +0x34c4e0 sub_34c4e0
    Lifted::new(0x7000a384f0, tpl_47::<0x50> as *const (), 1), // +0x34c4f0 sub_34c4f0
    Lifted::new(0x7000a38500, tpl_36::<0x80780> as *const (), 1), // +0x34c500 sub_34c500
    Lifted::new(0x7000a38510, tpl_191::<0x18> as *const (), 1), // +0x34c510 sub_34c510
    Lifted::new(0x7000a38520, tpl_36::<0xf8> as *const (), 1), // +0x34c520 sub_34c520
    Lifted::new(0x7000a38530, tpl_30 as *const (), 1), // +0x34c530 sub_34c530
    Lifted::new(0x7000a38540, tpl_191::<0x60> as *const (), 1), // +0x34c540 sub_34c540
    Lifted::new(0x7000a38550, tpl_47::<0x78> as *const (), 1), // +0x34c550 sub_34c550
    Lifted::new(0x7000a38560, tpl_39::<0x10> as *const (), 1), // +0x34c560 sub_34c560
    Lifted::new(0x7000a38570, tpl_534::<0x18> as *const (), 1), // +0x34c570 sub_34c570
    Lifted::new(0x7000a38590, tpl_49::<0x139> as *const (), 1), // +0x34c590 sub_34c590
    Lifted::new(0x7000a385a0, fn_7000a385a0 as *const (), 1), // +0x34c5a0 sub_34c5a0
    Lifted::new(0x7000a385c0, tpl_51::<0x38> as *const (), 1), // +0x34c5c0 sub_34c5c0
    Lifted::new(0x7000a385d0, fn_7000a385d0 as *const (), 0), // +0x34c5d0 sub_34c5d0
    Lifted::new(0x7000a386a0, tpl_49::<0x110> as *const (), 1), // +0x34c6a0 sub_34c6a0
    Lifted::new(0x7000a386b0, tpl_548::<0x7a> as *const (), 1), // +0x34c6b0 sub_34c6b0
    Lifted::new(0x7000a386c0, tpl_245::<0x6c> as *const (), 1), // +0x34c6c0 sub_34c6c0
    Lifted::new(0x7000a386d0, fn_7000a386d0 as *const (), 1), // +0x34c6d0 sub_34c6d0
    Lifted::new(0x7000a386f0, tpl_549::<0xb, 0x8, 0x10, 0x7000767d50, 0x700074a010> as *const (), 0), // +0x34c6f0 sub_34c6f0
    Lifted::new(0x7000a387c0, tpl_378::<0x81> as *const (), 1), // +0x34c7c0 sub_34c7c0
    Lifted::new(0x7000a387e0, tpl_375::<0x81> as *const (), 1), // +0x34c7e0 sub_34c7e0
    Lifted::new(0x7000a38800, tpl_522::<0x81> as *const (), 1), // +0x34c800 sub_34c800
    Lifted::new(0x7000a38810, fn_7000a38810 as *const (), 1), // +0x34c810 sub_34c810
    Lifted::new(0x7000a38820, tpl_550::<0x7a> as *const (), 1), // +0x34c820 sub_34c820
    Lifted::new(0x7000a38830, tpl_551::<0x79> as *const (), 1), // +0x34c830 sub_34c830
    Lifted::new(0x7000a38840, tpl_548::<0x79> as *const (), 1), // +0x34c840 sub_34c840
    Lifted::new(0x7000a38850, tpl_522::<0x7a> as *const (), 1), // +0x34c850 sub_34c850
    Lifted::new(0x7000a38860, tpl_369::<0x81> as *const (), 1), // +0x34c860 sub_34c860
    Lifted::new(0x7000a38880, tpl_369::<0x79> as *const (), 1), // +0x34c880 sub_34c880
    Lifted::new(0x7000a38890, tpl_551::<0x7a> as *const (), 1), // +0x34c890 sub_34c890
    Lifted::new(0x7000a388a0, tpl_51::<0x18> as *const (), 1), // +0x34c8a0 sub_34c8a0
    Lifted::new(0x7000a388b0, fn_7000a388b0 as *const (), 1), // +0x34c8b0 sub_34c8b0
    Lifted::new(0x7000a38900, tpl_51::<0xc8> as *const (), 1), // +0x34c900 sub_34c900
    Lifted::new(0x7000a38910, tpl_51::<0x50> as *const (), 1), // +0x34c910 sub_34c910
    Lifted::new(0x7000a38920, tpl_171::<0x93> as *const (), 1), // +0x34c920 sub_34c920
    Lifted::new(0x7000a38930, tpl_51::<0xa8> as *const (), 1), // +0x34c930 sub_34c930
    Lifted::new(0x7000a38940, fn_7000a38940 as *const (), 1), // +0x34c940 sub_34c940
    Lifted::new(0x7000a389b0, tpl_245::<0x68> as *const (), 1), // +0x34c9b0 sub_34c9b0
    Lifted::new(0x7000a389c0, fn_7000a389c0 as *const (), 0), // +0x34c9c0 sub_34c9c0
    Lifted::new(0x7000a38a20, fn_7000a38a20 as *const (), 1), // +0x34ca20 sub_34ca20
    Lifted::new(0x7000a38a40, fn_7000a38a40 as *const (), 1), // +0x34ca40 sub_34ca40
    Lifted::new(0x7000a38a70, tpl_36::<0x10> as *const (), 1), // +0x34ca70 sub_34ca70
    Lifted::new(0x7000a38a80, tpl_36::<0x8> as *const (), 1), // +0x34ca80 sub_34ca80
    Lifted::new(0x7000a38a90, tpl_36::<0x8> as *const (), 1), // +0x34ca90 sub_34ca90
    Lifted::new(0x7000a38aa0, tpl_353::<0x79> as *const (), 1), // +0x34caa0 sub_34caa0
    Lifted::new(0x7000a38ab0, tpl_360::<0x10, 0x48> as *const (), 1), // +0x34cab0 sub_34cab0
    Lifted::new(0x7000a38ac0, fn_7000a38ac0 as *const (), 1), // +0x34cac0 sub_34cac0
    Lifted::new(0x7000a38ae0, tpl_36::<0xa8> as *const (), 1), // +0x34cae0 sub_34cae0
    Lifted::new(0x7000a38af0, tpl_471::<0xb0, 0x1> as *const (), 1), // +0x34caf0 sub_34caf0
    Lifted::new(0x7000a38b00, tpl_191::<0x8> as *const (), 1), // +0x34cb00 sub_34cb00
    Lifted::new(0x7000a38b10, tpl_52::<0x8> as *const (), 1), // +0x34cb10 sub_34cb10
    Lifted::new(0x7000a38b20, tpl_453::<0x68, 0x7000a38b24, 0x7000a38b25, 0x7000a38b26, 0x7000a38b27, 0x7000a38b28, 0x7000a38b29, 0x7000a38b2a, 0x7000a38b2b, 0x7000a38b2c, 0x7000a38b2d, 0x7000a38b2e, 0x7000a38b2f, 0x700177c0f0> as *const (), 0), // +0x34cb20 sub_34cb20
    Lifted::new(0x7000a38b40, tpl_36::<0x28> as *const (), 1), // +0x34cb40 sub_34cb40
    Lifted::new(0x7000a38b50, tpl_20 as *const (), 1), // +0x34cb50 sub_34cb50
    Lifted::new(0x7000a38b60, tpl_36::<0x50> as *const (), 1), // +0x34cb60 sub_34cb60
    Lifted::new(0x7000a38b70, tpl_275::<0x18, 0x38> as *const (), 1), // +0x34cb70 sub_34cb70
    Lifted::new(0x7000a38b80, fn_7000a38b80 as *const (), 1), // +0x34cb80 sub_34cb80
    Lifted::new(0x7000a38ba0, tpl_390::<0x7001c6abd8, 0x18, 0x10, 0x70009a6500> as *const (), 0), // +0x34cba0 sub_34cba0
    Lifted::new(0x7000a38cc0, fn_7000a38cc0 as *const (), 1), // +0x34ccc0 sub_34ccc0
    Lifted::new(0x7000a38cf0, tpl_36::<0x28> as *const (), 1), // +0x34ccf0 sub_34ccf0
    Lifted::new(0x7000a38d00, tpl_337::<0x80> as *const (), 1), // +0x34cd00 sub_34cd00
    Lifted::new(0x7000a38d10, tpl_534::<0x8> as *const (), 1), // +0x34cd10 sub_34cd10
    Lifted::new(0x7000a38d30, tpl_52::<0xe0> as *const (), 1), // +0x34cd30 sub_34cd30
    Lifted::new(0x7000a38d40, tpl_536::<0x80> as *const (), 1), // +0x34cd40 sub_34cd40
    Lifted::new(0x7000a38d50, fn_7000a38d50 as *const (), 1), // +0x34cd50 sub_34cd50
    Lifted::new(0x7000a38d60, tpl_52::<0x30> as *const (), 1), // +0x34cd60 sub_34cd60
    Lifted::new(0x7000a38d70, fn_7000a38d70 as *const (), 1), // +0x34cd70 sub_34cd70
    Lifted::new(0x7000a38d80, tpl_36::<0xb8> as *const (), 1), // +0x34cd80 sub_34cd80
    Lifted::new(0x7000a38d90, fn_7000a38d90 as *const (), 1), // +0x34cd90 sub_34cd90
    Lifted::new(0x7000a38dc0, fn_7000a38dc0 as *const (), 1), // +0x34cdc0 sub_34cdc0
    Lifted::new(0x7000a38de0, tpl_15 as *const (), 1), // +0x34cde0 sub_34cde0
    Lifted::new(0x7000a38df0, tpl_552::<0x8, 0x700077aad0, 0x7000a39d60> as *const (), 0), // +0x34cdf0 sub_34cdf0
    Lifted::new(0x7000a38e50, tpl_552::<0x8, 0x700077aad0, 0x7000a39d60> as *const (), 0), // +0x34ce50 sub_34ce50
    Lifted::new(0x7000a38eb0, tpl_553::<0x24, 0x18, 0x7000a38e50, 0x8> as *const (), 0), // +0x34ceb0 sub_34ceb0
    Lifted::new(0x7000a38f40, tpl_554::<0x18, 0x8, 0x7000a38e50, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x34cf40 sub_34cf40
    Lifted::new(0x7000a38fa0, tpl_555::<0x18, 0x8, 0x7000a38e50> as *const (), 0), // +0x34cfa0 sub_34cfa0
    Lifted::new(0x7000a38fd0, tpl_554::<0x18, 0x8, 0x7000a38e50, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x34cfd0 sub_34cfd0
    Lifted::new(0x7000a39030, fn_7000a39030 as *const (), 0), // +0x34d030 sub_34d030
    Lifted::new(0x7000a39110, tpl_556::<0x18, 0x8, 0x7000a38e50, 0x1> as *const (), 0), // +0x34d110 sub_34d110
    Lifted::new(0x7000a391c0, tpl_556::<0x18, 0x8, 0x7000a38e50, 0x1> as *const (), 0), // +0x34d1c0 sub_34d1c0
    Lifted::new(0x7000a39270, tpl_553::<0x24, 0x18, 0x7000a38e50, 0x8> as *const (), 0), // +0x34d270 sub_34d270
    Lifted::new(0x7000a39300, fn_7000a39300 as *const (), 0), // +0x34d300 sub_34d300
    Lifted::new(0x7000a39450, tpl_554::<0x18, 0x8, 0x7000a38e50, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x34d450 sub_34d450
    Lifted::new(0x7000a394b0, tpl_554::<0x18, 0x8, 0x7000a38e50, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x34d4b0 sub_34d4b0
    Lifted::new(0x7000a39510, fn_7000a39510 as *const (), 0), // +0x34d510 sub_34d510
    Lifted::new(0x7000a395a0, tpl_555::<0x18, 0x8, 0x7000a38e50> as *const (), 0), // +0x34d5a0 sub_34d5a0
    Lifted::new(0x7000a395e0, fn_7000a395e0 as *const (), 0), // +0x34d5e0 sub_34d5e0
    Lifted::new(0x7000a39640, tpl_557::<0x8, 0x7000a38e50, 0x10> as *const (), 0), // +0x34d640 sub_34d640
    Lifted::new(0x7000a396c0, tpl_558::<0x10, 0x7000a38e50, 0x8, 0x18> as *const (), 0), // +0x34d6c0 sub_34d6c0
    Lifted::new(0x7000a39720, tpl_558::<0x10, 0x7000a38e50, 0x8, 0x18> as *const (), 0), // +0x34d720 sub_34d720
    Lifted::new(0x7000a39780, fn_7000a39780 as *const (), 0), // +0x34d780 sub_34d780
    Lifted::new(0x7000a39820, fn_7000a39820 as *const (), 0), // +0x34d820 sub_34d820
    Lifted::new(0x7000a39910, fn_7000a39910 as *const (), 0), // +0x34d910 sub_34d910
    Lifted::new(0x7000a39a10, fn_7000a39a10 as *const (), 0), // +0x34da10 sub_34da10
    Lifted::new(0x7000a39a70, fn_7000a39a70 as *const (), 0), // +0x34da70 sub_34da70
    Lifted::new(0x7000a39ae0, fn_7000a39ae0 as *const (), 0), // +0x34dae0 sub_34dae0
    Lifted::new(0x7000a39ba0, fn_7000a39ba0 as *const (), 0), // +0x34dba0 sub_34dba0
    Lifted::new(0x7000a39c80, fn_7000a39c80 as *const (), 0), // +0x34dc80 sub_34dc80
    Lifted::new(0x7000a39d60, fn_7000a39d60 as *const (), 0), // +0x34dd60 sub_34dd60
    Lifted::new(0x7000a3a8e0, fn_7000a3a8e0 as *const (), 0), // +0x34e8e0 sub_34e8e0
    Lifted::new(0x7000a3a9b0, tpl_555::<0x18, 0x8, 0x7000a38e50> as *const (), 0), // +0x34e9b0 sub_34e9b0
    Lifted::new(0x7000a3a9e0, tpl_554::<0x18, 0x8, 0x7000a38e50, 0xffffffffffffffff, 0x10> as *const (), 0), // +0x34e9e0 sub_34e9e0
    Lifted::new(0x7000a3aa70, tpl_555::<0x18, 0x10, 0x7000a38e50> as *const (), 0), // +0x34ea70 sub_34ea70
    Lifted::new(0x7000a3aaa0, tpl_559::<0xc, 0x7000a38e50> as *const (), 0), // +0x34eaa0 sub_34eaa0
    Lifted::new(0x7000a3ab40, fn_7000a3ab40 as *const (), 0), // +0x34eb40 sub_34eb40
    Lifted::new(0x7000a3ac50, fn_7000a3ac50 as *const (), 0), // +0x34ec50 sub_34ec50
    Lifted::new(0x7000a3ace0, tpl_555::<0x18, 0x8, 0x7000a38e50> as *const (), 0), // +0x34ece0 sub_34ece0
    Lifted::new(0x7000a3ad10, tpl_557::<0x8, 0x7000a38e50, 0x18> as *const (), 0), // +0x34ed10 sub_34ed10
    Lifted::new(0x7000a3ad60, tpl_557::<0x8, 0x7000a38e50, 0x10> as *const (), 0), // +0x34ed60 sub_34ed60
    Lifted::new(0x7000a3adb0, tpl_555::<0x18, 0x8, 0x7000a38e50> as *const (), 0), // +0x34edb0 sub_34edb0
    Lifted::new(0x7000a3ae00, fn_7000a3ae00 as *const (), 0), // +0x34ee00 sub_34ee00
    Lifted::new(0x7000a3ae50, fn_7000a3ae50 as *const (), 0), // +0x34ee50 sub_34ee50
    Lifted::new(0x7000a3aea0, tpl_555::<0x18, 0x8, 0x7000a38e50> as *const (), 0), // +0x34eea0 sub_34eea0
    Lifted::new(0x7000a3aed0, tpl_555::<0x18, 0x8, 0x7000a38e50> as *const (), 0), // +0x34eed0 sub_34eed0
    Lifted::new(0x7000a3af00, fn_7000a3af00 as *const (), 0), // +0x34ef00 sub_34ef00
    Lifted::new(0x7000a3afd0, fn_7000a3afd0 as *const (), 0), // +0x34efd0 sub_34efd0
    Lifted::new(0x7000a3b0a0, fn_7000a3b0a0 as *const (), 2), // +0x34f0a0 sub_34f0a0
    Lifted::new(0x7000a3b310, fn_7000a3b310 as *const (), 2), // +0x34f310 sub_34f310
    Lifted::new(0x7000a3b580, fn_7000a3b580 as *const (), 0), // +0x34f580 sub_34f580
    Lifted::new(0x7000a3b6d0, fn_7000a3b6d0 as *const (), 0), // +0x34f6d0 sub_34f6d0
    Lifted::new(0x7000a3bcd0, fn_7000a3bcd0 as *const (), 0), // +0x34fcd0 sub_34fcd0
    Lifted::new(0x7000a3be30, tpl_560::<0x7000732f50> as *const (), 0), // +0x34fe30 sub_34fe30
    Lifted::new(0x7000a3c050, fn_7000a3c050 as *const (), 0), // +0x350050 sub_350050
    Lifted::new(0x7000a3c280, fn_7000a3c280 as *const (), 2), // +0x350280 sub_350280
    Lifted::new(0x7000a3c360, fn_7000a3c360 as *const (), 2), // +0x350360 sub_350360
    Lifted::new(0x7000a3c4d0, fn_7000a3c4d0 as *const (), 2), // +0x3504d0 sub_3504d0
    Lifted::new(0x7000a3c5b0, fn_7000a3c5b0 as *const (), 2), // +0x3505b0 sub_3505b0
    Lifted::new(0x7000a3c720, fn_7000a3c720 as *const (), 2), // +0x350720 sub_350720
    Lifted::new(0x7000a3c940, tpl_561::<0x700177c0c0> as *const (), 0), // +0x350940 sub_350940
    Lifted::new(0x7000a3cad0, tpl_561::<0x700177c0c0> as *const (), 0), // +0x350ad0 sub_350ad0
    Lifted::new(0x7000a3cda0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x350da0 sub_350da0
    Lifted::new(0x7000a3cde0, tpl_55::<0x700177c0d0> as *const (), 0), // +0x350de0 sub_350de0
    Lifted::new(0x7000a3ce20, tpl_562::<0x8, 0x10, 0x7000a40b60> as *const (), 0), // +0x350e20 sub_350e20
    Lifted::new(0x7000a3ce50, fn_7000a3ce50 as *const (), 2), // +0x350e50 sub_350e50
    Lifted::new(0x7000a3fd40, tpl_33::<0x700177c312> as *const (), 0), // +0x353d40 sub_353d40
    Lifted::new(0x7000a3fd70, tpl_33::<0x700177c312> as *const (), 0), // +0x353d70 sub_353d70
    Lifted::new(0x7000a3fda0, tpl_33::<0x700177c312> as *const (), 0), // +0x353da0 sub_353da0
    Lifted::new(0x7000a3fdd0, fn_7000a3fdd0 as *const (), 2), // +0x353dd0 sub_353dd0
    Lifted::new(0x7000a3fe60, fn_7000a3fe60 as *const (), 1), // +0x353e60 sub_353e60
    Lifted::new(0x7000a3fe80, fn_7000a3fe80 as *const (), 1), // +0x353e80 sub_353e80
    Lifted::new(0x7000a3fea0, fn_7000a3fea0 as *const (), 1), // +0x353ea0 sub_353ea0
    Lifted::new(0x7000a3fec0, fn_7000a3fec0 as *const (), 1), // +0x353ec0 sub_353ec0
    Lifted::new(0x7000a3fee0, fn_7000a3fee0 as *const (), 1), // +0x353ee0 sub_353ee0
    Lifted::new(0x7000a3ff00, fn_7000a3ff00 as *const (), 3), // +0x353f00 sub_353f00
    Lifted::new(0x7000a40040, fn_7000a40040 as *const (), 1), // +0x354040 sub_354040
    Lifted::new(0x7000a40060, fn_7000a40060 as *const (), 1), // +0x354060 sub_354060
    Lifted::new(0x7000a40080, fn_7000a40080 as *const (), 1), // +0x354080 sub_354080
    Lifted::new(0x7000a400a0, fn_7000a400a0 as *const (), 1), // +0x3540a0 sub_3540a0
    Lifted::new(0x7000a40100, fn_7000a40100 as *const (), 2), // +0x354100 sub_354100
    Lifted::new(0x7000a401b0, fn_7000a401b0 as *const (), 0), // +0x3541b0 sub_3541b0
    Lifted::new(0x7000a40320, fn_7000a40320 as *const (), 1), // +0x354320 sub_354320
    Lifted::new(0x7000a40340, fn_7000a40340 as *const (), 1), // +0x354340 sub_354340
    Lifted::new(0x7000a40360, fn_7000a40360 as *const (), 0), // +0x354360 sub_354360
    Lifted::new(0x7000a40460, fn_7000a40460 as *const (), 1), // +0x354460 sub_354460
    Lifted::new(0x7000a40480, fn_7000a40480 as *const (), 1), // +0x354480 sub_354480
    Lifted::new(0x7000a404a0, fn_7000a404a0 as *const (), 0), // +0x3544a0 sub_3544a0
    Lifted::new(0x7000a405c0, fn_7000a405c0 as *const (), 1), // +0x3545c0 sub_3545c0
    Lifted::new(0x7000a405e0, fn_7000a405e0 as *const (), 1), // +0x3545e0 sub_3545e0
    Lifted::new(0x7000a40600, fn_7000a40600 as *const (), 1), // +0x354600 sub_354600
    Lifted::new(0x7000a40620, fn_7000a40620 as *const (), 1), // +0x354620 sub_354620
    Lifted::new(0x7000a40640, fn_7000a40640 as *const (), 1), // +0x354640 sub_354640
    Lifted::new(0x7000a40660, fn_7000a40660 as *const (), 1), // +0x354660 sub_354660
    Lifted::new(0x7000a40680, fn_7000a40680 as *const (), 2), // +0x354680 sub_354680
    Lifted::new(0x7000a40830, fn_7000a40830 as *const (), 2), // +0x354830 sub_354830
    Lifted::new(0x7000a408c0, fn_7000a408c0 as *const (), 2), // +0x3548c0 sub_3548c0
    Lifted::new(0x7000a40a20, fn_7000a40a20 as *const (), 2), // +0x354a20 sub_354a20
    Lifted::new(0x7000a40b40, fn_7000a40b40 as *const (), 1), // +0x354b40 sub_354b40
    Lifted::new(0x7000a40b60, fn_7000a40b60 as *const (), 0), // +0x354b60 sub_354b60
    Lifted::new(0x7000a40c00, fn_7000a40c00 as *const (), 0), // +0x354c00 sub_354c00
    Lifted::new(0x7000a40c60, fn_7000a40c60 as *const (), 1), // +0x354c60 sub_354c60
    Lifted::new(0x7000a40c80, fn_7000a40c80 as *const (), 1), // +0x354c80 sub_354c80
    Lifted::new(0x7000a40ca0, fn_7000a40ca0 as *const (), 1), // +0x354ca0 sub_354ca0
    Lifted::new(0x7000a40cd0, fn_7000a40cd0 as *const (), 0), // +0x354cd0 sub_354cd0
    Lifted::new(0x7000a40d20, fn_7000a40d20 as *const (), 1), // +0x354d20 sub_354d20
    Lifted::new(0x7000a40d40, fn_7000a40d40 as *const (), 1), // +0x354d40 sub_354d40
    Lifted::new(0x7000a40d60, fn_7000a40d60 as *const (), 1), // +0x354d60 sub_354d60
    Lifted::new(0x7000a40d80, fn_7000a40d80 as *const (), 1), // +0x354d80 sub_354d80
    Lifted::new(0x7000a40da0, fn_7000a40da0 as *const (), 1), // +0x354da0 sub_354da0
    Lifted::new(0x7000a40dc0, fn_7000a40dc0 as *const (), 1), // +0x354dc0 sub_354dc0
    Lifted::new(0x7000a40de0, fn_7000a40de0 as *const (), 1), // +0x354de0 sub_354de0
    Lifted::new(0x7000a40e00, fn_7000a40e00 as *const (), 1), // +0x354e00 sub_354e00
    Lifted::new(0x7000a40e20, fn_7000a40e20 as *const (), 1), // +0x354e20 sub_354e20
    Lifted::new(0x7000a40e40, fn_7000a40e40 as *const (), 0), // +0x354e40 sub_354e40
    Lifted::new(0x7000a40f00, fn_7000a40f00 as *const (), 0), // +0x354f00 sub_354f00
    Lifted::new(0x7000a40f90, fn_7000a40f90 as *const (), 0), // +0x354f90 sub_354f90
    Lifted::new(0x7000a41000, fn_7000a41000 as *const (), 1), // +0x355000 sub_355000
    Lifted::new(0x7000a41020, fn_7000a41020 as *const (), 1), // +0x355020 sub_355020
    Lifted::new(0x7000a41040, fn_7000a41040 as *const (), 1), // +0x355040 sub_355040
    Lifted::new(0x7000a41060, fn_7000a41060 as *const (), 1), // +0x355060 sub_355060
    Lifted::new(0x7000a41080, fn_7000a41080 as *const (), 1), // +0x355080 sub_355080
    Lifted::new(0x7000a410a0, fn_7000a410a0 as *const (), 1), // +0x3550a0 sub_3550a0
    Lifted::new(0x7000a410c0, fn_7000a410c0 as *const (), 1), // +0x3550c0 sub_3550c0
    Lifted::new(0x7000a410e0, fn_7000a410e0 as *const (), 1), // +0x3550e0 sub_3550e0
    Lifted::new(0x7000a41100, fn_7000a41100 as *const (), 1), // +0x355100 sub_355100
    Lifted::new(0x7000a41120, fn_7000a41120 as *const (), 0), // +0x355120 sub_355120
    Lifted::new(0x7000a411e0, fn_7000a411e0 as *const (), 1), // +0x3551e0 sub_3551e0
    Lifted::new(0x7000a41200, fn_7000a41200 as *const (), 1), // +0x355200 sub_355200
    Lifted::new(0x7000a41220, fn_7000a41220 as *const (), 1), // +0x355220 sub_355220
    Lifted::new(0x7000a41240, fn_7000a41240 as *const (), 1), // +0x355240 sub_355240
    Lifted::new(0x7000a41260, fn_7000a41260 as *const (), 1), // +0x355260 sub_355260
    Lifted::new(0x7000a41280, fn_7000a41280 as *const (), 1), // +0x355280 sub_355280
    Lifted::new(0x7000a412a0, fn_7000a412a0 as *const (), 1), // +0x3552a0 sub_3552a0
    Lifted::new(0x7000a412c0, fn_7000a412c0 as *const (), 1), // +0x3552c0 sub_3552c0
    Lifted::new(0x7000a412e0, fn_7000a412e0 as *const (), 0), // +0x3552e0 sub_3552e0
    Lifted::new(0x7000a41390, fn_7000a41390 as *const (), 1), // +0x355390 sub_355390
    Lifted::new(0x7000a413b0, fn_7000a413b0 as *const (), 1), // +0x3553b0 sub_3553b0
    Lifted::new(0x7000a413d0, fn_7000a413d0 as *const (), 1), // +0x3553d0 sub_3553d0
    Lifted::new(0x7000a413f0, fn_7000a413f0 as *const (), 1), // +0x3553f0 sub_3553f0
    Lifted::new(0x7000a41410, fn_7000a41410 as *const (), 1), // +0x355410 sub_355410
    Lifted::new(0x7000a41430, fn_7000a41430 as *const (), 1), // +0x355430 sub_355430
    Lifted::new(0x7000a41450, fn_7000a41450 as *const (), 1), // +0x355450 sub_355450
    Lifted::new(0x7000a41470, fn_7000a41470 as *const (), 1), // +0x355470 sub_355470
    Lifted::new(0x7000a41490, fn_7000a41490 as *const (), 1), // +0x355490 sub_355490
    Lifted::new(0x7000a414b0, fn_7000a414b0 as *const (), 1), // +0x3554b0 sub_3554b0
    Lifted::new(0x7000a414d0, fn_7000a414d0 as *const (), 1), // +0x3554d0 sub_3554d0
    Lifted::new(0x7000a414f0, fn_7000a414f0 as *const (), 1), // +0x3554f0 sub_3554f0
    Lifted::new(0x7000a41510, fn_7000a41510 as *const (), 1), // +0x355510 sub_355510
    Lifted::new(0x7000a41530, fn_7000a41530 as *const (), 1), // +0x355530 sub_355530
    Lifted::new(0x7000a41550, fn_7000a41550 as *const (), 1), // +0x355550 sub_355550
    Lifted::new(0x7000a41570, fn_7000a41570 as *const (), 1), // +0x355570 sub_355570
    Lifted::new(0x7000a41590, fn_7000a41590 as *const (), 1), // +0x355590 sub_355590
    Lifted::new(0x7000a415b0, fn_7000a415b0 as *const (), 1), // +0x3555b0 sub_3555b0
    Lifted::new(0x7000a415d0, fn_7000a415d0 as *const (), 1), // +0x3555d0 sub_3555d0
    Lifted::new(0x7000a415f0, fn_7000a415f0 as *const (), 1), // +0x3555f0 sub_3555f0
    Lifted::new(0x7000a41610, fn_7000a41610 as *const (), 1), // +0x355610 sub_355610
    Lifted::new(0x7000a41630, fn_7000a41630 as *const (), 1), // +0x355630 sub_355630
    Lifted::new(0x7000a41650, fn_7000a41650 as *const (), 1), // +0x355650 sub_355650
    Lifted::new(0x7000a41670, fn_7000a41670 as *const (), 1), // +0x355670 sub_355670
    Lifted::new(0x7000a41690, fn_7000a41690 as *const (), 1), // +0x355690 sub_355690
    Lifted::new(0x7000a416b0, fn_7000a416b0 as *const (), 1), // +0x3556b0 sub_3556b0
    Lifted::new(0x7000a416d0, fn_7000a416d0 as *const (), 1), // +0x3556d0 sub_3556d0
    Lifted::new(0x7000a416f0, fn_7000a416f0 as *const (), 1), // +0x3556f0 sub_3556f0
    Lifted::new(0x7000a41710, fn_7000a41710 as *const (), 1), // +0x355710 sub_355710
    Lifted::new(0x7000a41730, fn_7000a41730 as *const (), 1), // +0x355730 sub_355730
    Lifted::new(0x7000a41750, fn_7000a41750 as *const (), 1), // +0x355750 sub_355750
    Lifted::new(0x7000a41770, fn_7000a41770 as *const (), 1), // +0x355770 sub_355770
    Lifted::new(0x7000a41790, fn_7000a41790 as *const (), 1), // +0x355790 sub_355790
    Lifted::new(0x7000a417b0, fn_7000a417b0 as *const (), 1), // +0x3557b0 sub_3557b0
    Lifted::new(0x7000a417d0, fn_7000a417d0 as *const (), 1), // +0x3557d0 sub_3557d0
    Lifted::new(0x7000a417f0, fn_7000a417f0 as *const (), 1), // +0x3557f0 sub_3557f0
    Lifted::new(0x7000a41810, fn_7000a41810 as *const (), 1), // +0x355810 sub_355810
    Lifted::new(0x7000a41830, fn_7000a41830 as *const (), 1), // +0x355830 sub_355830
    Lifted::new(0x7000a41850, fn_7000a41850 as *const (), 1), // +0x355850 sub_355850
    Lifted::new(0x7000a41870, fn_7000a41870 as *const (), 1), // +0x355870 sub_355870
    Lifted::new(0x7000a41890, fn_7000a41890 as *const (), 1), // +0x355890 sub_355890
    Lifted::new(0x7000a418b0, fn_7000a418b0 as *const (), 1), // +0x3558b0 sub_3558b0
    Lifted::new(0x7000a418d0, fn_7000a418d0 as *const (), 1), // +0x3558d0 sub_3558d0
    Lifted::new(0x7000a418f0, fn_7000a418f0 as *const (), 1), // +0x3558f0 sub_3558f0
    Lifted::new(0x7000a41910, fn_7000a41910 as *const (), 1), // +0x355910 sub_355910
    Lifted::new(0x7000a41930, fn_7000a41930 as *const (), 1), // +0x355930 sub_355930
    Lifted::new(0x7000a41950, fn_7000a41950 as *const (), 1), // +0x355950 sub_355950
    Lifted::new(0x7000a41970, fn_7000a41970 as *const (), 1), // +0x355970 sub_355970
    Lifted::new(0x7000a41990, fn_7000a41990 as *const (), 1), // +0x355990 sub_355990
    Lifted::new(0x7000a419b0, fn_7000a419b0 as *const (), 1), // +0x3559b0 sub_3559b0
    Lifted::new(0x7000a419d0, fn_7000a419d0 as *const (), 1), // +0x3559d0 sub_3559d0
    Lifted::new(0x7000a419f0, fn_7000a419f0 as *const (), 1), // +0x3559f0 sub_3559f0
    Lifted::new(0x7000a41a10, fn_7000a41a10 as *const (), 1), // +0x355a10 sub_355a10
    Lifted::new(0x7000a41a30, fn_7000a41a30 as *const (), 1), // +0x355a30 sub_355a30
    Lifted::new(0x7000a41a50, fn_7000a41a50 as *const (), 1), // +0x355a50 sub_355a50
    Lifted::new(0x7000a41a70, fn_7000a41a70 as *const (), 1), // +0x355a70 sub_355a70
    Lifted::new(0x7000a41a90, fn_7000a41a90 as *const (), 1), // +0x355a90 sub_355a90
    Lifted::new(0x7000a41ab0, fn_7000a41ab0 as *const (), 1), // +0x355ab0 sub_355ab0
    Lifted::new(0x7000a41ad0, fn_7000a41ad0 as *const (), 1), // +0x355ad0 sub_355ad0
    Lifted::new(0x7000a41af0, fn_7000a41af0 as *const (), 1), // +0x355af0 sub_355af0
    Lifted::new(0x7000a41b10, fn_7000a41b10 as *const (), 1), // +0x355b10 sub_355b10
    Lifted::new(0x7000a41b30, fn_7000a41b30 as *const (), 1), // +0x355b30 sub_355b30
    Lifted::new(0x7000a41b50, fn_7000a41b50 as *const (), 1), // +0x355b50 sub_355b50
    Lifted::new(0x7000a41b70, fn_7000a41b70 as *const (), 1), // +0x355b70 sub_355b70
    Lifted::new(0x7000a41b90, fn_7000a41b90 as *const (), 1), // +0x355b90 sub_355b90
    Lifted::new(0x7000a41bb0, fn_7000a41bb0 as *const (), 1), // +0x355bb0 sub_355bb0
    Lifted::new(0x7000a41bd0, fn_7000a41bd0 as *const (), 1), // +0x355bd0 sub_355bd0
    Lifted::new(0x7000a41bf0, fn_7000a41bf0 as *const (), 1), // +0x355bf0 sub_355bf0
    Lifted::new(0x7000a41c10, fn_7000a41c10 as *const (), 1), // +0x355c10 sub_355c10
    Lifted::new(0x7000a41c30, fn_7000a41c30 as *const (), 1), // +0x355c30 sub_355c30
    Lifted::new(0x7000a41c50, fn_7000a41c50 as *const (), 1), // +0x355c50 sub_355c50
    Lifted::new(0x7000a41c70, fn_7000a41c70 as *const (), 1), // +0x355c70 sub_355c70
    Lifted::new(0x7000a41c90, fn_7000a41c90 as *const (), 1), // +0x355c90 sub_355c90
    Lifted::new(0x7000a41cb0, fn_7000a41cb0 as *const (), 1), // +0x355cb0 sub_355cb0
    Lifted::new(0x7000a41cd0, fn_7000a41cd0 as *const (), 1), // +0x355cd0 sub_355cd0
    Lifted::new(0x7000a41cf0, fn_7000a41cf0 as *const (), 1), // +0x355cf0 sub_355cf0
    Lifted::new(0x7000a41d10, fn_7000a41d10 as *const (), 1), // +0x355d10 sub_355d10
    Lifted::new(0x7000a41d30, fn_7000a41d30 as *const (), 1), // +0x355d30 sub_355d30
    Lifted::new(0x7000a41d50, fn_7000a41d50 as *const (), 1), // +0x355d50 sub_355d50
    Lifted::new(0x7000a41d70, fn_7000a41d70 as *const (), 1), // +0x355d70 sub_355d70
    Lifted::new(0x7000a41d90, fn_7000a41d90 as *const (), 1), // +0x355d90 sub_355d90
    Lifted::new(0x7000a41db0, fn_7000a41db0 as *const (), 1), // +0x355db0 sub_355db0
    Lifted::new(0x7000a41dd0, fn_7000a41dd0 as *const (), 1), // +0x355dd0 sub_355dd0
    Lifted::new(0x7000a41df0, fn_7000a41df0 as *const (), 1), // +0x355df0 sub_355df0
    Lifted::new(0x7000a41e10, fn_7000a41e10 as *const (), 1), // +0x355e10 sub_355e10
    Lifted::new(0x7000a41e30, fn_7000a41e30 as *const (), 1), // +0x355e30 sub_355e30
    Lifted::new(0x7000a41e50, fn_7000a41e50 as *const (), 1), // +0x355e50 sub_355e50
    Lifted::new(0x7000a41e70, fn_7000a41e70 as *const (), 1), // +0x355e70 sub_355e70
    Lifted::new(0x7000a41e90, fn_7000a41e90 as *const (), 1), // +0x355e90 sub_355e90
    Lifted::new(0x7000a41eb0, fn_7000a41eb0 as *const (), 1), // +0x355eb0 sub_355eb0
    Lifted::new(0x7000a41ed0, fn_7000a41ed0 as *const (), 1), // +0x355ed0 sub_355ed0
    Lifted::new(0x7000a41ef0, fn_7000a41ef0 as *const (), 1), // +0x355ef0 sub_355ef0
    Lifted::new(0x7000a41f10, fn_7000a41f10 as *const (), 1), // +0x355f10 sub_355f10
    Lifted::new(0x7000a41f30, fn_7000a41f30 as *const (), 1), // +0x355f30 sub_355f30
    Lifted::new(0x7000a41f50, fn_7000a41f50 as *const (), 1), // +0x355f50 sub_355f50
    Lifted::new(0x7000a41f70, fn_7000a41f70 as *const (), 1), // +0x355f70 sub_355f70
    Lifted::new(0x7000a41f90, fn_7000a41f90 as *const (), 1), // +0x355f90 sub_355f90
    Lifted::new(0x7000a41fb0, fn_7000a41fb0 as *const (), 1), // +0x355fb0 sub_355fb0
    Lifted::new(0x7000a41fd0, fn_7000a41fd0 as *const (), 1), // +0x355fd0 sub_355fd0
    Lifted::new(0x7000a41ff0, fn_7000a41ff0 as *const (), 1), // +0x355ff0 sub_355ff0
    Lifted::new(0x7000a42010, fn_7000a42010 as *const (), 1), // +0x356010 sub_356010
    Lifted::new(0x7000a42030, fn_7000a42030 as *const (), 1), // +0x356030 sub_356030
    Lifted::new(0x7000a42050, fn_7000a42050 as *const (), 1), // +0x356050 sub_356050
    Lifted::new(0x7000a42070, fn_7000a42070 as *const (), 1), // +0x356070 sub_356070
    Lifted::new(0x7000a42090, fn_7000a42090 as *const (), 1), // +0x356090 sub_356090
    Lifted::new(0x7000a420b0, fn_7000a420b0 as *const (), 1), // +0x3560b0 sub_3560b0
    Lifted::new(0x7000a420d0, fn_7000a420d0 as *const (), 1), // +0x3560d0 sub_3560d0
    Lifted::new(0x7000a420f0, fn_7000a420f0 as *const (), 1), // +0x3560f0 sub_3560f0
    Lifted::new(0x7000a42110, fn_7000a42110 as *const (), 1), // +0x356110 sub_356110
    Lifted::new(0x7000a42130, fn_7000a42130 as *const (), 1), // +0x356130 sub_356130
    Lifted::new(0x7000a42150, fn_7000a42150 as *const (), 1), // +0x356150 sub_356150
    Lifted::new(0x7000a42170, fn_7000a42170 as *const (), 1), // +0x356170 sub_356170
    Lifted::new(0x7000a42190, fn_7000a42190 as *const (), 1), // +0x356190 sub_356190
    Lifted::new(0x7000a421b0, fn_7000a421b0 as *const (), 1), // +0x3561b0 sub_3561b0
    Lifted::new(0x7000a421d0, fn_7000a421d0 as *const (), 1), // +0x3561d0 sub_3561d0
    Lifted::new(0x7000a421f0, fn_7000a421f0 as *const (), 1), // +0x3561f0 sub_3561f0
    Lifted::new(0x7000a42210, fn_7000a42210 as *const (), 1), // +0x356210 sub_356210
    Lifted::new(0x7000a42230, fn_7000a42230 as *const (), 1), // +0x356230 sub_356230
    Lifted::new(0x7000a42250, fn_7000a42250 as *const (), 1), // +0x356250 sub_356250
    Lifted::new(0x7000a42270, fn_7000a42270 as *const (), 1), // +0x356270 sub_356270
    Lifted::new(0x7000a42290, fn_7000a42290 as *const (), 1), // +0x356290 sub_356290
    Lifted::new(0x7000a422b0, fn_7000a422b0 as *const (), 1), // +0x3562b0 sub_3562b0
    Lifted::new(0x7000a422d0, fn_7000a422d0 as *const (), 1), // +0x3562d0 sub_3562d0
    Lifted::new(0x7000a422f0, fn_7000a422f0 as *const (), 1), // +0x3562f0 sub_3562f0
    Lifted::new(0x7000a42310, fn_7000a42310 as *const (), 1), // +0x356310 sub_356310
    Lifted::new(0x7000a42330, fn_7000a42330 as *const (), 1), // +0x356330 sub_356330
    Lifted::new(0x7000a42350, fn_7000a42350 as *const (), 1), // +0x356350 sub_356350
    Lifted::new(0x7000a42370, fn_7000a42370 as *const (), 1), // +0x356370 sub_356370
    Lifted::new(0x7000a42390, fn_7000a42390 as *const (), 1), // +0x356390 sub_356390
    Lifted::new(0x7000a423b0, fn_7000a423b0 as *const (), 1), // +0x3563b0 sub_3563b0
    Lifted::new(0x7000a423d0, fn_7000a423d0 as *const (), 1), // +0x3563d0 sub_3563d0
    Lifted::new(0x7000a423f0, fn_7000a423f0 as *const (), 1), // +0x3563f0 sub_3563f0
    Lifted::new(0x7000a42410, fn_7000a42410 as *const (), 1), // +0x356410 sub_356410
    Lifted::new(0x7000a42430, fn_7000a42430 as *const (), 1), // +0x356430 sub_356430
    Lifted::new(0x7000a42450, fn_7000a42450 as *const (), 1), // +0x356450 sub_356450
    Lifted::new(0x7000a42470, fn_7000a42470 as *const (), 1), // +0x356470 sub_356470
    Lifted::new(0x7000a42490, fn_7000a42490 as *const (), 1), // +0x356490 sub_356490
    Lifted::new(0x7000a424b0, fn_7000a424b0 as *const (), 1), // +0x3564b0 sub_3564b0
    Lifted::new(0x7000a424d0, fn_7000a424d0 as *const (), 1), // +0x3564d0 sub_3564d0
    Lifted::new(0x7000a424f0, fn_7000a424f0 as *const (), 1), // +0x3564f0 sub_3564f0
    Lifted::new(0x7000a42510, fn_7000a42510 as *const (), 1), // +0x356510 sub_356510
    Lifted::new(0x7000a42530, fn_7000a42530 as *const (), 1), // +0x356530 sub_356530
    Lifted::new(0x7000a42550, fn_7000a42550 as *const (), 1), // +0x356550 sub_356550
    Lifted::new(0x7000a42570, fn_7000a42570 as *const (), 1), // +0x356570 sub_356570
    Lifted::new(0x7000a42590, fn_7000a42590 as *const (), 1), // +0x356590 sub_356590
    Lifted::new(0x7000a425b0, fn_7000a425b0 as *const (), 1), // +0x3565b0 sub_3565b0
    Lifted::new(0x7000a425d0, fn_7000a425d0 as *const (), 1), // +0x3565d0 sub_3565d0
    Lifted::new(0x7000a425f0, fn_7000a425f0 as *const (), 1), // +0x3565f0 sub_3565f0
    Lifted::new(0x7000a42610, fn_7000a42610 as *const (), 1), // +0x356610 sub_356610
    Lifted::new(0x7000a42630, fn_7000a42630 as *const (), 1), // +0x356630 sub_356630
    Lifted::new(0x7000a42650, fn_7000a42650 as *const (), 1), // +0x356650 sub_356650
    Lifted::new(0x7000a42670, fn_7000a42670 as *const (), 1), // +0x356670 sub_356670
    Lifted::new(0x7000a42690, fn_7000a42690 as *const (), 1), // +0x356690 sub_356690
    Lifted::new(0x7000a426b0, fn_7000a426b0 as *const (), 1), // +0x3566b0 sub_3566b0
    Lifted::new(0x7000a426d0, fn_7000a426d0 as *const (), 1), // +0x3566d0 sub_3566d0
    Lifted::new(0x7000a426f0, fn_7000a426f0 as *const (), 1), // +0x3566f0 sub_3566f0
    Lifted::new(0x7000a42710, fn_7000a42710 as *const (), 1), // +0x356710 sub_356710
    Lifted::new(0x7000a42730, fn_7000a42730 as *const (), 1), // +0x356730 sub_356730
    Lifted::new(0x7000a42750, fn_7000a42750 as *const (), 1), // +0x356750 sub_356750
    Lifted::new(0x7000a42770, fn_7000a42770 as *const (), 1), // +0x356770 sub_356770
    Lifted::new(0x7000a42790, fn_7000a42790 as *const (), 1), // +0x356790 sub_356790
    Lifted::new(0x7000a427b0, fn_7000a427b0 as *const (), 1), // +0x3567b0 sub_3567b0
    Lifted::new(0x7000a427d0, fn_7000a427d0 as *const (), 1), // +0x3567d0 sub_3567d0
    Lifted::new(0x7000a427f0, tpl_187 as *const (), 1), // +0x3567f0 sub_3567f0
    Lifted::new(0x7000a42800, fn_7000a42800 as *const (), 1), // +0x356800 sub_356800
    Lifted::new(0x7000a42820, fn_7000a42820 as *const (), 1), // +0x356820 sub_356820
    Lifted::new(0x7000a42840, fn_7000a42840 as *const (), 1), // +0x356840 sub_356840
    Lifted::new(0x7000a42860, fn_7000a42860 as *const (), 1), // +0x356860 sub_356860
    Lifted::new(0x7000a42880, fn_7000a42880 as *const (), 1), // +0x356880 sub_356880
    Lifted::new(0x7000a428a0, fn_7000a428a0 as *const (), 1), // +0x3568a0 sub_3568a0
    Lifted::new(0x7000a428c0, fn_7000a428c0 as *const (), 1), // +0x3568c0 sub_3568c0
    Lifted::new(0x7000a428e0, fn_7000a428e0 as *const (), 1), // +0x3568e0 sub_3568e0
    Lifted::new(0x7000a42900, fn_7000a42900 as *const (), 1), // +0x356900 sub_356900
    Lifted::new(0x7000a42920, fn_7000a42920 as *const (), 1), // +0x356920 sub_356920
    Lifted::new(0x7000a42940, fn_7000a42940 as *const (), 1), // +0x356940 sub_356940
    Lifted::new(0x7000a42960, fn_7000a42960 as *const (), 1), // +0x356960 sub_356960
    Lifted::new(0x7000a42980, fn_7000a42980 as *const (), 1), // +0x356980 sub_356980
    Lifted::new(0x7000a429a0, fn_7000a429a0 as *const (), 1), // +0x3569a0 sub_3569a0
    Lifted::new(0x7000a429c0, fn_7000a429c0 as *const (), 1), // +0x3569c0 sub_3569c0
    Lifted::new(0x7000a429e0, fn_7000a429e0 as *const (), 1), // +0x3569e0 sub_3569e0
    Lifted::new(0x7000a42a00, fn_7000a42a00 as *const (), 1), // +0x356a00 sub_356a00
    Lifted::new(0x7000a42a20, fn_7000a42a20 as *const (), 1), // +0x356a20 sub_356a20
    Lifted::new(0x7000a42a40, fn_7000a42a40 as *const (), 1), // +0x356a40 sub_356a40
    Lifted::new(0x7000a42a60, fn_7000a42a60 as *const (), 1), // +0x356a60 sub_356a60
    Lifted::new(0x7000a42a80, fn_7000a42a80 as *const (), 1), // +0x356a80 sub_356a80
    Lifted::new(0x7000a42aa0, fn_7000a42aa0 as *const (), 1), // +0x356aa0 sub_356aa0
    Lifted::new(0x7000a42ac0, fn_7000a42ac0 as *const (), 1), // +0x356ac0 sub_356ac0
    Lifted::new(0x7000a42ae0, fn_7000a42ae0 as *const (), 1), // +0x356ae0 sub_356ae0
    Lifted::new(0x7000a42b00, fn_7000a42b00 as *const (), 1), // +0x356b00 sub_356b00
    Lifted::new(0x7000a42b20, fn_7000a42b20 as *const (), 1), // +0x356b20 sub_356b20
    Lifted::new(0x7000a42b40, fn_7000a42b40 as *const (), 1), // +0x356b40 sub_356b40
    Lifted::new(0x7000a42b60, fn_7000a42b60 as *const (), 1), // +0x356b60 sub_356b60
    Lifted::new(0x7000a42b80, fn_7000a42b80 as *const (), 1), // +0x356b80 sub_356b80
    Lifted::new(0x7000a42ba0, fn_7000a42ba0 as *const (), 1), // +0x356ba0 sub_356ba0
    Lifted::new(0x7000a42bc0, fn_7000a42bc0 as *const (), 1), // +0x356bc0 sub_356bc0
    Lifted::new(0x7000a42be0, fn_7000a42be0 as *const (), 1), // +0x356be0 sub_356be0
    Lifted::new(0x7000a42c00, fn_7000a42c00 as *const (), 1), // +0x356c00 sub_356c00
    Lifted::new(0x7000a42c20, fn_7000a42c20 as *const (), 1), // +0x356c20 sub_356c20
    Lifted::new(0x7000a42c40, fn_7000a42c40 as *const (), 1), // +0x356c40 sub_356c40
    Lifted::new(0x7000a42c60, fn_7000a42c60 as *const (), 1), // +0x356c60 sub_356c60
    Lifted::new(0x7000a42c80, fn_7000a42c80 as *const (), 1), // +0x356c80 sub_356c80
    Lifted::new(0x7000a42ca0, fn_7000a42ca0 as *const (), 1), // +0x356ca0 sub_356ca0
    Lifted::new(0x7000a42cc0, fn_7000a42cc0 as *const (), 1), // +0x356cc0 sub_356cc0
    Lifted::new(0x7000a42ce0, fn_7000a42ce0 as *const (), 1), // +0x356ce0 sub_356ce0
    Lifted::new(0x7000a42d00, fn_7000a42d00 as *const (), 1), // +0x356d00 sub_356d00
    Lifted::new(0x7000a42d20, fn_7000a42d20 as *const (), 1), // +0x356d20 sub_356d20
    Lifted::new(0x7000a42d40, fn_7000a42d40 as *const (), 1), // +0x356d40 sub_356d40
    Lifted::new(0x7000a42d60, fn_7000a42d60 as *const (), 1), // +0x356d60 sub_356d60
    Lifted::new(0x7000a42d80, fn_7000a42d80 as *const (), 1), // +0x356d80 sub_356d80
    Lifted::new(0x7000a42da0, fn_7000a42da0 as *const (), 1), // +0x356da0 sub_356da0
    Lifted::new(0x7000a42dc0, fn_7000a42dc0 as *const (), 1), // +0x356dc0 sub_356dc0
    Lifted::new(0x7000a42de0, fn_7000a42de0 as *const (), 1), // +0x356de0 sub_356de0
    Lifted::new(0x7000a42e00, fn_7000a42e00 as *const (), 1), // +0x356e00 sub_356e00
    Lifted::new(0x7000a42e20, fn_7000a42e20 as *const (), 1), // +0x356e20 sub_356e20
    Lifted::new(0x7000a42e40, fn_7000a42e40 as *const (), 1), // +0x356e40 sub_356e40
    Lifted::new(0x7000a42e60, fn_7000a42e60 as *const (), 1), // +0x356e60 sub_356e60
    Lifted::new(0x7000a42e80, fn_7000a42e80 as *const (), 1), // +0x356e80 sub_356e80
    Lifted::new(0x7000a42ea0, fn_7000a42ea0 as *const (), 1), // +0x356ea0 sub_356ea0
    Lifted::new(0x7000a42ec0, fn_7000a42ec0 as *const (), 1), // +0x356ec0 sub_356ec0
    Lifted::new(0x7000a42ee0, fn_7000a42ee0 as *const (), 1), // +0x356ee0 sub_356ee0
    Lifted::new(0x7000a42f00, fn_7000a42f00 as *const (), 1), // +0x356f00 sub_356f00
    Lifted::new(0x7000a42f20, fn_7000a42f20 as *const (), 1), // +0x356f20 sub_356f20
    Lifted::new(0x7000a42f40, fn_7000a42f40 as *const (), 1), // +0x356f40 sub_356f40
    Lifted::new(0x7000a42f60, fn_7000a42f60 as *const (), 1), // +0x356f60 sub_356f60
    Lifted::new(0x7000a42f80, fn_7000a42f80 as *const (), 1), // +0x356f80 sub_356f80
    Lifted::new(0x7000a42fa0, fn_7000a42fa0 as *const (), 1), // +0x356fa0 sub_356fa0
    Lifted::new(0x7000a42fc0, fn_7000a42fc0 as *const (), 1), // +0x356fc0 sub_356fc0
    Lifted::new(0x7000a42fe0, fn_7000a42fe0 as *const (), 1), // +0x356fe0 sub_356fe0
    Lifted::new(0x7000a43000, fn_7000a43000 as *const (), 1), // +0x357000 sub_357000
    Lifted::new(0x7000a43020, fn_7000a43020 as *const (), 1), // +0x357020 sub_357020
    Lifted::new(0x7000a43040, fn_7000a43040 as *const (), 1), // +0x357040 sub_357040
    Lifted::new(0x7000a43060, fn_7000a43060 as *const (), 1), // +0x357060 sub_357060
    Lifted::new(0x7000a43080, fn_7000a43080 as *const (), 1), // +0x357080 sub_357080
    Lifted::new(0x7000a430a0, fn_7000a430a0 as *const (), 1), // +0x3570a0 sub_3570a0
    Lifted::new(0x7000a430c0, fn_7000a430c0 as *const (), 1), // +0x3570c0 sub_3570c0
    Lifted::new(0x7000a430e0, fn_7000a430e0 as *const (), 1), // +0x3570e0 sub_3570e0
    Lifted::new(0x7000a43100, fn_7000a43100 as *const (), 1), // +0x357100 sub_357100
    Lifted::new(0x7000a43120, fn_7000a43120 as *const (), 1), // +0x357120 sub_357120
    Lifted::new(0x7000a43140, fn_7000a43140 as *const (), 1), // +0x357140 sub_357140
    Lifted::new(0x7000a43160, fn_7000a43160 as *const (), 1), // +0x357160 sub_357160
    Lifted::new(0x7000a43180, fn_7000a43180 as *const (), 1), // +0x357180 sub_357180
    Lifted::new(0x7000a431a0, fn_7000a431a0 as *const (), 1), // +0x3571a0 sub_3571a0
    Lifted::new(0x7000a431c0, fn_7000a431c0 as *const (), 1), // +0x3571c0 sub_3571c0
    Lifted::new(0x7000a431e0, fn_7000a431e0 as *const (), 1), // +0x3571e0 sub_3571e0
    Lifted::new(0x7000a43200, fn_7000a43200 as *const (), 1), // +0x357200 sub_357200
    Lifted::new(0x7000a43220, fn_7000a43220 as *const (), 1), // +0x357220 sub_357220
    Lifted::new(0x7000a43240, fn_7000a43240 as *const (), 1), // +0x357240 sub_357240
    Lifted::new(0x7000a43260, fn_7000a43260 as *const (), 1), // +0x357260 sub_357260
    Lifted::new(0x7000a43280, fn_7000a43280 as *const (), 1), // +0x357280 sub_357280
    Lifted::new(0x7000a432a0, fn_7000a432a0 as *const (), 1), // +0x3572a0 sub_3572a0
    Lifted::new(0x7000a432c0, fn_7000a432c0 as *const (), 1), // +0x3572c0 sub_3572c0
    Lifted::new(0x7000a432e0, fn_7000a432e0 as *const (), 1), // +0x3572e0 sub_3572e0
    Lifted::new(0x7000a43300, fn_7000a43300 as *const (), 1), // +0x357300 sub_357300
    Lifted::new(0x7000a43320, fn_7000a43320 as *const (), 1), // +0x357320 sub_357320
    Lifted::new(0x7000a43340, fn_7000a43340 as *const (), 1), // +0x357340 sub_357340
    Lifted::new(0x7000a43360, fn_7000a43360 as *const (), 1), // +0x357360 sub_357360
    Lifted::new(0x7000a43380, fn_7000a43380 as *const (), 1), // +0x357380 sub_357380
    Lifted::new(0x7000a433a0, fn_7000a433a0 as *const (), 1), // +0x3573a0 sub_3573a0
    Lifted::new(0x7000a433c0, fn_7000a433c0 as *const (), 1), // +0x3573c0 sub_3573c0
    Lifted::new(0x7000a433e0, fn_7000a433e0 as *const (), 1), // +0x3573e0 sub_3573e0
    Lifted::new(0x7000a43400, fn_7000a43400 as *const (), 1), // +0x357400 sub_357400
    Lifted::new(0x7000a43420, fn_7000a43420 as *const (), 1), // +0x357420 sub_357420
    Lifted::new(0x7000a43440, fn_7000a43440 as *const (), 1), // +0x357440 sub_357440
    Lifted::new(0x7000a43460, fn_7000a43460 as *const (), 1), // +0x357460 sub_357460
    Lifted::new(0x7000a43480, fn_7000a43480 as *const (), 1), // +0x357480 sub_357480
    Lifted::new(0x7000a434a0, fn_7000a434a0 as *const (), 1), // +0x3574a0 sub_3574a0
    Lifted::new(0x7000a434c0, fn_7000a434c0 as *const (), 1), // +0x3574c0 sub_3574c0
    Lifted::new(0x7000a434e0, fn_7000a434e0 as *const (), 1), // +0x3574e0 sub_3574e0
    Lifted::new(0x7000a43500, fn_7000a43500 as *const (), 1), // +0x357500 sub_357500
    Lifted::new(0x7000a43520, fn_7000a43520 as *const (), 1), // +0x357520 sub_357520
    Lifted::new(0x7000a43540, fn_7000a43540 as *const (), 1), // +0x357540 sub_357540
    Lifted::new(0x7000a43560, fn_7000a43560 as *const (), 1), // +0x357560 sub_357560
    Lifted::new(0x7000a43580, fn_7000a43580 as *const (), 1), // +0x357580 sub_357580
    Lifted::new(0x7000a435a0, fn_7000a435a0 as *const (), 1), // +0x3575a0 sub_3575a0
    Lifted::new(0x7000a435c0, fn_7000a435c0 as *const (), 1), // +0x3575c0 sub_3575c0
    Lifted::new(0x7000a435e0, fn_7000a435e0 as *const (), 1), // +0x3575e0 sub_3575e0
    Lifted::new(0x7000a43600, fn_7000a43600 as *const (), 1), // +0x357600 sub_357600
    Lifted::new(0x7000a43620, fn_7000a43620 as *const (), 1), // +0x357620 sub_357620
    Lifted::new(0x7000a43640, fn_7000a43640 as *const (), 1), // +0x357640 sub_357640
    Lifted::new(0x7000a43660, fn_7000a43660 as *const (), 1), // +0x357660 sub_357660
    Lifted::new(0x7000a43680, fn_7000a43680 as *const (), 1), // +0x357680 sub_357680
    Lifted::new(0x7000a436a0, fn_7000a436a0 as *const (), 1), // +0x3576a0 sub_3576a0
    Lifted::new(0x7000a436c0, fn_7000a436c0 as *const (), 1), // +0x3576c0 sub_3576c0
    Lifted::new(0x7000a436e0, fn_7000a436e0 as *const (), 1), // +0x3576e0 sub_3576e0
    Lifted::new(0x7000a43700, fn_7000a43700 as *const (), 1), // +0x357700 sub_357700
    Lifted::new(0x7000a43720, fn_7000a43720 as *const (), 1), // +0x357720 sub_357720
    Lifted::new(0x7000a43740, fn_7000a43740 as *const (), 1), // +0x357740 sub_357740
    Lifted::new(0x7000a43760, fn_7000a43760 as *const (), 1), // +0x357760 sub_357760
    Lifted::new(0x7000a43780, fn_7000a43780 as *const (), 1), // +0x357780 sub_357780
    Lifted::new(0x7000a437a0, fn_7000a437a0 as *const (), 1), // +0x3577a0 sub_3577a0
    Lifted::new(0x7000a437c0, fn_7000a437c0 as *const (), 1), // +0x3577c0 sub_3577c0
    Lifted::new(0x7000a437e0, fn_7000a437e0 as *const (), 1), // +0x3577e0 sub_3577e0
    Lifted::new(0x7000a43800, fn_7000a43800 as *const (), 1), // +0x357800 sub_357800
    Lifted::new(0x7000a43820, fn_7000a43820 as *const (), 1), // +0x357820 sub_357820
    Lifted::new(0x7000a43840, fn_7000a43840 as *const (), 1), // +0x357840 sub_357840
    Lifted::new(0x7000a43860, fn_7000a43860 as *const (), 1), // +0x357860 sub_357860
    Lifted::new(0x7000a43880, fn_7000a43880 as *const (), 1), // +0x357880 sub_357880
    Lifted::new(0x7000a438a0, fn_7000a438a0 as *const (), 1), // +0x3578a0 sub_3578a0
    Lifted::new(0x7000a438c0, fn_7000a438c0 as *const (), 1), // +0x3578c0 sub_3578c0
    Lifted::new(0x7000a438e0, fn_7000a438e0 as *const (), 1), // +0x3578e0 sub_3578e0
    Lifted::new(0x7000a43900, fn_7000a43900 as *const (), 1), // +0x357900 sub_357900
    Lifted::new(0x7000a43920, fn_7000a43920 as *const (), 1), // +0x357920 sub_357920
    Lifted::new(0x7000a43940, fn_7000a43940 as *const (), 1), // +0x357940 sub_357940
    Lifted::new(0x7000a43960, fn_7000a43960 as *const (), 1), // +0x357960 sub_357960
    Lifted::new(0x7000a43980, fn_7000a43980 as *const (), 1), // +0x357980 sub_357980
    Lifted::new(0x7000a439a0, fn_7000a439a0 as *const (), 1), // +0x3579a0 sub_3579a0
    Lifted::new(0x7000a439c0, fn_7000a439c0 as *const (), 1), // +0x3579c0 sub_3579c0
    Lifted::new(0x7000a439e0, fn_7000a439e0 as *const (), 1), // +0x3579e0 sub_3579e0
    Lifted::new(0x7000a43a00, fn_7000a43a00 as *const (), 1), // +0x357a00 sub_357a00
    Lifted::new(0x7000a43a20, fn_7000a43a20 as *const (), 1), // +0x357a20 sub_357a20
    Lifted::new(0x7000a43a40, fn_7000a43a40 as *const (), 1), // +0x357a40 sub_357a40
    Lifted::new(0x7000a43a60, fn_7000a43a60 as *const (), 1), // +0x357a60 sub_357a60
    Lifted::new(0x7000a43a80, fn_7000a43a80 as *const (), 1), // +0x357a80 sub_357a80
    Lifted::new(0x7000a43aa0, fn_7000a43aa0 as *const (), 1), // +0x357aa0 sub_357aa0
    Lifted::new(0x7000a43ac0, fn_7000a43ac0 as *const (), 1), // +0x357ac0 sub_357ac0
    Lifted::new(0x7000a43ae0, fn_7000a43ae0 as *const (), 1), // +0x357ae0 sub_357ae0
    Lifted::new(0x7000a43b00, fn_7000a43b00 as *const (), 1), // +0x357b00 sub_357b00
    Lifted::new(0x7000a43b20, fn_7000a43b20 as *const (), 1), // +0x357b20 sub_357b20
    Lifted::new(0x7000a43b40, fn_7000a43b40 as *const (), 1), // +0x357b40 sub_357b40
    Lifted::new(0x7000a43b60, fn_7000a43b60 as *const (), 1), // +0x357b60 sub_357b60
    Lifted::new(0x7000a43b80, fn_7000a43b80 as *const (), 1), // +0x357b80 sub_357b80
    Lifted::new(0x7000a43ba0, fn_7000a43ba0 as *const (), 1), // +0x357ba0 sub_357ba0
    Lifted::new(0x7000a43bc0, fn_7000a43bc0 as *const (), 1), // +0x357bc0 sub_357bc0
    Lifted::new(0x7000a43be0, fn_7000a43be0 as *const (), 1), // +0x357be0 sub_357be0
    Lifted::new(0x7000a43c00, fn_7000a43c00 as *const (), 1), // +0x357c00 sub_357c00
    Lifted::new(0x7000a43c20, fn_7000a43c20 as *const (), 1), // +0x357c20 sub_357c20
    Lifted::new(0x7000a43c40, fn_7000a43c40 as *const (), 1), // +0x357c40 sub_357c40
    Lifted::new(0x7000a43c60, fn_7000a43c60 as *const (), 1), // +0x357c60 sub_357c60
    Lifted::new(0x7000a43c80, fn_7000a43c80 as *const (), 1), // +0x357c80 sub_357c80
    Lifted::new(0x7000a43ca0, fn_7000a43ca0 as *const (), 1), // +0x357ca0 sub_357ca0
    Lifted::new(0x7000a43cc0, fn_7000a43cc0 as *const (), 1), // +0x357cc0 sub_357cc0
    Lifted::new(0x7000a43ce0, fn_7000a43ce0 as *const (), 1), // +0x357ce0 sub_357ce0
    Lifted::new(0x7000a43d00, fn_7000a43d00 as *const (), 1), // +0x357d00 sub_357d00
    Lifted::new(0x7000a43d20, fn_7000a43d20 as *const (), 1), // +0x357d20 sub_357d20
    Lifted::new(0x7000a43d40, fn_7000a43d40 as *const (), 1), // +0x357d40 sub_357d40
    Lifted::new(0x7000a43d60, fn_7000a43d60 as *const (), 1), // +0x357d60 sub_357d60
    Lifted::new(0x7000a43d80, fn_7000a43d80 as *const (), 1), // +0x357d80 sub_357d80
    Lifted::new(0x7000a43da0, fn_7000a43da0 as *const (), 1), // +0x357da0 sub_357da0
    Lifted::new(0x7000a43dc0, fn_7000a43dc0 as *const (), 1), // +0x357dc0 sub_357dc0
    Lifted::new(0x7000a43de0, fn_7000a43de0 as *const (), 1), // +0x357de0 sub_357de0
    Lifted::new(0x7000a43e00, fn_7000a43e00 as *const (), 1), // +0x357e00 sub_357e00
    Lifted::new(0x7000a43e20, fn_7000a43e20 as *const (), 1), // +0x357e20 sub_357e20
    Lifted::new(0x7000a43e40, fn_7000a43e40 as *const (), 1), // +0x357e40 sub_357e40
    Lifted::new(0x7000a43e60, fn_7000a43e60 as *const (), 1), // +0x357e60 sub_357e60
    Lifted::new(0x7000a43e80, fn_7000a43e80 as *const (), 1), // +0x357e80 sub_357e80
    Lifted::new(0x7000a43ea0, fn_7000a43ea0 as *const (), 1), // +0x357ea0 sub_357ea0
    Lifted::new(0x7000a43ec0, fn_7000a43ec0 as *const (), 1), // +0x357ec0 sub_357ec0
    Lifted::new(0x7000a43ee0, fn_7000a43ee0 as *const (), 1), // +0x357ee0 sub_357ee0
    Lifted::new(0x7000a43f00, fn_7000a43f00 as *const (), 1), // +0x357f00 sub_357f00
    Lifted::new(0x7000a43f20, fn_7000a43f20 as *const (), 1), // +0x357f20 sub_357f20
    Lifted::new(0x7000a43f40, fn_7000a43f40 as *const (), 1), // +0x357f40 sub_357f40
    Lifted::new(0x7000a43f60, fn_7000a43f60 as *const (), 1), // +0x357f60 sub_357f60
    Lifted::new(0x7000a43f80, fn_7000a43f80 as *const (), 1), // +0x357f80 sub_357f80
    Lifted::new(0x7000a43fa0, fn_7000a43fa0 as *const (), 1), // +0x357fa0 sub_357fa0
    Lifted::new(0x7000a43fc0, fn_7000a43fc0 as *const (), 1), // +0x357fc0 sub_357fc0
    Lifted::new(0x7000a43fe0, fn_7000a43fe0 as *const (), 1), // +0x357fe0 sub_357fe0
    Lifted::new(0x7000a44000, fn_7000a44000 as *const (), 1), // +0x358000 sub_358000
    Lifted::new(0x7000a44020, fn_7000a44020 as *const (), 1), // +0x358020 sub_358020
    Lifted::new(0x7000a44040, fn_7000a44040 as *const (), 1), // +0x358040 sub_358040
    Lifted::new(0x7000a44060, fn_7000a44060 as *const (), 1), // +0x358060 sub_358060
    Lifted::new(0x7000a44080, fn_7000a44080 as *const (), 1), // +0x358080 sub_358080
    Lifted::new(0x7000a440a0, fn_7000a440a0 as *const (), 1), // +0x3580a0 sub_3580a0
    Lifted::new(0x7000a440c0, fn_7000a440c0 as *const (), 1), // +0x3580c0 sub_3580c0
    Lifted::new(0x7000a440e0, fn_7000a440e0 as *const (), 1), // +0x3580e0 sub_3580e0
    Lifted::new(0x7000a44100, fn_7000a44100 as *const (), 1), // +0x358100 sub_358100
    Lifted::new(0x7000a44120, fn_7000a44120 as *const (), 1), // +0x358120 sub_358120
    Lifted::new(0x7000a44140, fn_7000a44140 as *const (), 1), // +0x358140 sub_358140
    Lifted::new(0x7000a44160, fn_7000a44160 as *const (), 1), // +0x358160 sub_358160
    Lifted::new(0x7000a44180, fn_7000a44180 as *const (), 1), // +0x358180 sub_358180
    Lifted::new(0x7000a441a0, fn_7000a441a0 as *const (), 1), // +0x3581a0 sub_3581a0
    Lifted::new(0x7000a441c0, fn_7000a441c0 as *const (), 1), // +0x3581c0 sub_3581c0
    Lifted::new(0x7000a441e0, fn_7000a441e0 as *const (), 1), // +0x3581e0 sub_3581e0
    Lifted::new(0x7000a44200, fn_7000a44200 as *const (), 1), // +0x358200 sub_358200
    Lifted::new(0x7000a44220, fn_7000a44220 as *const (), 1), // +0x358220 sub_358220
    Lifted::new(0x7000a44240, fn_7000a44240 as *const (), 1), // +0x358240 sub_358240
    Lifted::new(0x7000a44260, fn_7000a44260 as *const (), 1), // +0x358260 sub_358260
    Lifted::new(0x7000a44280, fn_7000a44280 as *const (), 1), // +0x358280 sub_358280
    Lifted::new(0x7000a442a0, fn_7000a442a0 as *const (), 1), // +0x3582a0 sub_3582a0
    Lifted::new(0x7000a442c0, fn_7000a442c0 as *const (), 1), // +0x3582c0 sub_3582c0
    Lifted::new(0x7000a442e0, fn_7000a442e0 as *const (), 1), // +0x3582e0 sub_3582e0
    Lifted::new(0x7000a44300, fn_7000a44300 as *const (), 1), // +0x358300 sub_358300
    Lifted::new(0x7000a44320, fn_7000a44320 as *const (), 1), // +0x358320 sub_358320
    Lifted::new(0x7000a44340, fn_7000a44340 as *const (), 1), // +0x358340 sub_358340
    Lifted::new(0x7000a44360, fn_7000a44360 as *const (), 1), // +0x358360 sub_358360
    Lifted::new(0x7000a44380, fn_7000a44380 as *const (), 1), // +0x358380 sub_358380
    Lifted::new(0x7000a443a0, fn_7000a443a0 as *const (), 1), // +0x3583a0 sub_3583a0
    Lifted::new(0x7000a443c0, fn_7000a443c0 as *const (), 1), // +0x3583c0 sub_3583c0
    Lifted::new(0x7000a443e0, fn_7000a443e0 as *const (), 1), // +0x3583e0 sub_3583e0
    Lifted::new(0x7000a44400, fn_7000a44400 as *const (), 1), // +0x358400 sub_358400
    Lifted::new(0x7000a44420, fn_7000a44420 as *const (), 1), // +0x358420 sub_358420
    Lifted::new(0x7000a44440, fn_7000a44440 as *const (), 1), // +0x358440 sub_358440
    Lifted::new(0x7000a44460, fn_7000a44460 as *const (), 1), // +0x358460 sub_358460
    Lifted::new(0x7000a44480, fn_7000a44480 as *const (), 1), // +0x358480 sub_358480
    Lifted::new(0x7000a444a0, fn_7000a444a0 as *const (), 1), // +0x3584a0 sub_3584a0
    Lifted::new(0x7000a444c0, fn_7000a444c0 as *const (), 1), // +0x3584c0 sub_3584c0
    Lifted::new(0x7000a444e0, fn_7000a444e0 as *const (), 1), // +0x3584e0 sub_3584e0
    Lifted::new(0x7000a44500, fn_7000a44500 as *const (), 1), // +0x358500 sub_358500
    Lifted::new(0x7000a44520, fn_7000a44520 as *const (), 1), // +0x358520 sub_358520
    Lifted::new(0x7000a44540, fn_7000a44540 as *const (), 1), // +0x358540 sub_358540
    Lifted::new(0x7000a44560, fn_7000a44560 as *const (), 1), // +0x358560 sub_358560
    Lifted::new(0x7000a44580, fn_7000a44580 as *const (), 1), // +0x358580 sub_358580
    Lifted::new(0x7000a445a0, fn_7000a445a0 as *const (), 1), // +0x3585a0 sub_3585a0
    Lifted::new(0x7000a445c0, fn_7000a445c0 as *const (), 1), // +0x3585c0 sub_3585c0
    Lifted::new(0x7000a445e0, fn_7000a445e0 as *const (), 1), // +0x3585e0 sub_3585e0
    Lifted::new(0x7000a44600, fn_7000a44600 as *const (), 1), // +0x358600 sub_358600
    Lifted::new(0x7000a44620, fn_7000a44620 as *const (), 1), // +0x358620 sub_358620
    Lifted::new(0x7000a44640, fn_7000a44640 as *const (), 1), // +0x358640 sub_358640
    Lifted::new(0x7000a44660, fn_7000a44660 as *const (), 1), // +0x358660 sub_358660
    Lifted::new(0x7000a44680, fn_7000a44680 as *const (), 1), // +0x358680 sub_358680
    Lifted::new(0x7000a446a0, fn_7000a446a0 as *const (), 1), // +0x3586a0 sub_3586a0
    Lifted::new(0x7000a446c0, fn_7000a446c0 as *const (), 1), // +0x3586c0 sub_3586c0
    Lifted::new(0x7000a446e0, fn_7000a446e0 as *const (), 1), // +0x3586e0 sub_3586e0
    Lifted::new(0x7000a44700, fn_7000a44700 as *const (), 1), // +0x358700 sub_358700
    Lifted::new(0x7000a44720, fn_7000a44720 as *const (), 1), // +0x358720 sub_358720
    Lifted::new(0x7000a44740, fn_7000a44740 as *const (), 1), // +0x358740 sub_358740
    Lifted::new(0x7000a44760, fn_7000a44760 as *const (), 1), // +0x358760 sub_358760
    Lifted::new(0x7000a44780, fn_7000a44780 as *const (), 1), // +0x358780 sub_358780
    Lifted::new(0x7000a447a0, fn_7000a447a0 as *const (), 1), // +0x3587a0 sub_3587a0
    Lifted::new(0x7000a447c0, fn_7000a447c0 as *const (), 1), // +0x3587c0 sub_3587c0
    Lifted::new(0x7000a447e0, fn_7000a447e0 as *const (), 1), // +0x3587e0 sub_3587e0
    Lifted::new(0x7000a44800, fn_7000a44800 as *const (), 1), // +0x358800 sub_358800
    Lifted::new(0x7000a44820, fn_7000a44820 as *const (), 1), // +0x358820 sub_358820
    Lifted::new(0x7000a44840, fn_7000a44840 as *const (), 1), // +0x358840 sub_358840
    Lifted::new(0x7000a44860, fn_7000a44860 as *const (), 1), // +0x358860 sub_358860
    Lifted::new(0x7000a44880, fn_7000a44880 as *const (), 1), // +0x358880 sub_358880
    Lifted::new(0x7000a448a0, fn_7000a448a0 as *const (), 1), // +0x3588a0 sub_3588a0
    Lifted::new(0x7000a448c0, fn_7000a448c0 as *const (), 1), // +0x3588c0 sub_3588c0
    Lifted::new(0x7000a448e0, fn_7000a448e0 as *const (), 1), // +0x3588e0 sub_3588e0
    Lifted::new(0x7000a44900, fn_7000a44900 as *const (), 1), // +0x358900 sub_358900
    Lifted::new(0x7000a44920, fn_7000a44920 as *const (), 1), // +0x358920 sub_358920
    Lifted::new(0x7000a44940, fn_7000a44940 as *const (), 1), // +0x358940 sub_358940
    Lifted::new(0x7000a44960, fn_7000a44960 as *const (), 2), // +0x358960 sub_358960
    Lifted::new(0x7000a44e70, fn_7000a44e70 as *const (), 2), // +0x358e70 sub_358e70
    Lifted::new(0x7000a45460, fn_7000a45460 as *const (), 2), // +0x359460 sub_359460
    Lifted::new(0x7000a459a0, fn_7000a459a0 as *const (), 2), // +0x3599a0 sub_3599a0
    Lifted::new(0x7000a45fa0, tpl_563::<0x4, 0x7000a44960, 0x7000a44e70> as *const (), 0), // +0x359fa0 sub_359fa0
    Lifted::new(0x7000a45fc0, tpl_563::<0x4, 0x7000a45460, 0x7000a459a0> as *const (), 0), // +0x359fc0 sub_359fc0
    Lifted::new(0x7000a45fe0, fn_7000a45fe0 as *const (), 2), // +0x359fe0 sub_359fe0
    Lifted::new(0x7000a462a0, fn_7000a462a0 as *const (), 2), // +0x35a2a0 sub_35a2a0
    Lifted::new(0x7000a46610, fn_7000a46610 as *const (), 0), // +0x35a610 sub_35a610
    Lifted::new(0x7000a466e0, fn_7000a466e0 as *const (), 0), // +0x35a6e0 sub_35a6e0
    Lifted::new(0x7000a467b0, fn_7000a467b0 as *const (), 2), // +0x35a7b0 sub_35a7b0
    Lifted::new(0x7000a46b80, fn_7000a46b80 as *const (), 2), // +0x35ab80 sub_35ab80
    Lifted::new(0x7000a46f50, tpl_564::<0x7000a47050, 0x7000a47a50> as *const (), 0), // +0x35af50 sub_35af50
    Lifted::new(0x7000a46fd0, tpl_565::<0x7000a47050, 0x7000a47a50> as *const (), 0), // +0x35afd0 sub_35afd0
    Lifted::new(0x7000a47050, fn_7000a47050 as *const (), 0), // +0x35b050 sub_35b050
    Lifted::new(0x7000a47110, tpl_55::<0x700177c0d0> as *const (), 0), // +0x35b110 sub_35b110
    Lifted::new(0x7000a47150, fn_7000a47150 as *const (), 0), // +0x35b150 sub_35b150
    Lifted::new(0x7000a47190, fn_7000a47190 as *const (), 1), // +0x35b190 sub_35b190
    Lifted::new(0x7000a471d0, fn_7000a471d0 as *const (), 0), // +0x35b1d0 sub_35b1d0
    Lifted::new(0x7000a47230, fn_7000a47230 as *const (), 1), // +0x35b230 sub_35b230
    Lifted::new(0x7000a47270, fn_7000a47270 as *const (), 1), // +0x35b270 sub_35b270
    Lifted::new(0x7000a47340, fn_7000a47340 as *const (), 1), // +0x35b340 sub_35b340
    Lifted::new(0x7000a47370, fn_7000a47370 as *const (), 0), // +0x35b370 sub_35b370
    Lifted::new(0x7000a473d0, fn_7000a473d0 as *const (), 1), // +0x35b3d0 sub_35b3d0
    Lifted::new(0x7000a47410, tpl_143::<0x28, 0x7000789380> as *const (), 0), // +0x35b410 sub_35b410
    Lifted::new(0x7000a47420, fn_7000a47420 as *const (), 2), // +0x35b420 sub_35b420
    Lifted::new(0x7000a474e0, fn_7000a474e0 as *const (), 1), // +0x35b4e0 sub_35b4e0
    Lifted::new(0x7000a47540, fn_7000a47540 as *const (), 0), // +0x35b540 sub_35b540
    Lifted::new(0x7000a47570, fn_7000a47570 as *const (), 2), // +0x35b570 sub_35b570
    Lifted::new(0x7000a479d0, fn_7000a479d0 as *const (), 1), // +0x35b9d0 sub_35b9d0
    Lifted::new(0x7000a47a50, fn_7000a47a50 as *const (), 0), // +0x35ba50 sub_35ba50
    Lifted::new(0x7000a47c40, fn_7000a47c40 as *const (), 0), // +0x35bc40 sub_35bc40
    Lifted::new(0x7000a47e70, fn_7000a47e70 as *const (), 0), // +0x35be70 sub_35be70
    Lifted::new(0x7000a48290, fn_7000a48290 as *const (), 2), // +0x35c290 sub_35c290
    Lifted::new(0x7000a48450, fn_7000a48450 as *const (), 0), // +0x35c450 sub_35c450
    Lifted::new(0x7000a486b0, fn_7000a486b0 as *const (), 0), // +0x35c6b0 sub_35c6b0
    Lifted::new(0x7000a486e0, fn_7000a486e0 as *const (), 2), // +0x35c6e0 sub_35c6e0
    Lifted::new(0x7000a48c20, fn_7000a48c20 as *const (), 0), // +0x35cc20 sub_35cc20
    Lifted::new(0x7000a48ce0, fn_7000a48ce0 as *const (), 1), // +0x35cce0 sub_35cce0
    Lifted::new(0x7000a48d10, fn_7000a48d10 as *const (), 1), // +0x35cd10 sub_35cd10
    Lifted::new(0x7000a48d50, fn_7000a48d50 as *const (), 1), // +0x35cd50 sub_35cd50
    Lifted::new(0x7000a48da0, fn_7000a48da0 as *const (), 1), // +0x35cda0 sub_35cda0
    Lifted::new(0x7000a48e10, fn_7000a48e10 as *const (), 1), // +0x35ce10 sub_35ce10
    Lifted::new(0x7000a48e50, fn_7000a48e50 as *const (), 1), // +0x35ce50 sub_35ce50
    Lifted::new(0x7000a48e70, fn_7000a48e70 as *const (), 1), // +0x35ce70 sub_35ce70
    Lifted::new(0x7000a48ea0, fn_7000a48ea0 as *const (), 1), // +0x35cea0 sub_35cea0
    Lifted::new(0x7000a48ed0, fn_7000a48ed0 as *const (), 1), // +0x35ced0 sub_35ced0
    Lifted::new(0x7000a48f60, fn_7000a48f60 as *const (), 0), // +0x35cf60 sub_35cf60
    Lifted::new(0x7000a49030, fn_7000a49030 as *const (), 0), // +0x35d030 sub_35d030
    Lifted::new(0x7000a490b0, fn_7000a490b0 as *const (), 1), // +0x35d0b0 sub_35d0b0
    Lifted::new(0x7000a490e0, fn_7000a490e0 as *const (), 1), // +0x35d0e0 sub_35d0e0
    Lifted::new(0x7000a49100, fn_7000a49100 as *const (), 0), // +0x35d100 sub_35d100
    Lifted::new(0x7000a49170, fn_7000a49170 as *const (), 0), // +0x35d170 sub_35d170
    Lifted::new(0x7000a491c0, fn_7000a491c0 as *const (), 0), // +0x35d1c0 sub_35d1c0
    Lifted::new(0x7000a49220, fn_7000a49220 as *const (), 1), // +0x35d220 sub_35d220
    Lifted::new(0x7000a49240, fn_7000a49240 as *const (), 1), // +0x35d240 sub_35d240
    Lifted::new(0x7000a492f0, fn_7000a492f0 as *const (), 1), // +0x35d2f0 sub_35d2f0
    Lifted::new(0x7000a49320, fn_7000a49320 as *const (), 1), // +0x35d320 sub_35d320
    Lifted::new(0x7000a49350, fn_7000a49350 as *const (), 0), // +0x35d350 sub_35d350
    Lifted::new(0x7000a49410, tpl_566::<0x7000a47050, 0x7000a47a50> as *const (), 0), // +0x35d410 sub_35d410
    Lifted::new(0x7000a49490, fn_7000a49490 as *const (), 2), // +0x35d490 sub_35d490
    Lifted::new(0x7000a496f0, fn_7000a496f0 as *const (), 1), // +0x35d6f0 sub_35d6f0
    Lifted::new(0x7000a49780, fn_7000a49780 as *const (), 0), // +0x35d780 sub_35d780
    Lifted::new(0x7000a49870, fn_7000a49870 as *const (), 1), // +0x35d870 sub_35d870
    Lifted::new(0x7000a498f0, fn_7000a498f0 as *const (), 2), // +0x35d8f0 sub_35d8f0
    Lifted::new(0x7000a49a50, fn_7000a49a50 as *const (), 0), // +0x35da50 sub_35da50
    Lifted::new(0x7000a49b60, tpl_143::<0x28, 0x70007103f0> as *const (), 0), // +0x35db60 sub_35db60
    Lifted::new(0x7000a49b70, fn_7000a49b70 as *const (), 1), // +0x35db70 sub_35db70
    Lifted::new(0x7000a49b90, fn_7000a49b90 as *const (), 0), // +0x35db90 sub_35db90
    Lifted::new(0x7000a49bc0, fn_7000a49bc0 as *const (), 0), // +0x35dbc0 sub_35dbc0
    Lifted::new(0x7000a49bf0, fn_7000a49bf0 as *const (), 2), // +0x35dbf0 sub_35dbf0
    Lifted::new(0x7000a49da0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a49db0> as *const (), 0), // +0x35dda0 sub_35dda0
    Lifted::new(0x7000a49de0, tpl_47::<0x11> as *const (), 1), // +0x35dde0 sub_35dde0
    Lifted::new(0x7000a49df0, tpl_360::<0x28, 0x6c> as *const (), 1), // +0x35ddf0 sub_35ddf0
    Lifted::new(0x7000a49e00, tpl_471::<0x28, 0x82> as *const (), 1), // +0x35de00 sub_35de00
    Lifted::new(0x7000a49e10, fn_7000a49e10 as *const (), 1), // +0x35de10 sub_35de10
    Lifted::new(0x7000a49e40, tpl_567::<0x28, 0x48, 0x40, 0x81> as *const (), 1), // +0x35de40 sub_35de40
    Lifted::new(0x7000a49e60, tpl_567::<0x28, 0x40, 0x48, 0x81> as *const (), 1), // +0x35de60 sub_35de60
    Lifted::new(0x7000a49e80, tpl_360::<0x28, 0x68> as *const (), 1), // +0x35de80 sub_35de80
    Lifted::new(0x7000a49e90, tpl_568::<0x700076df00, 0x7000767d50, 0x700074a010, 0x7000e970f0, 0x7000f05cd0> as *const (), 2), // +0x35de90 sub_35de90
    Lifted::new(0x7000a4a1f0, fn_7000a4a1f0 as *const (), 0), // +0x35e1f0 sub_35e1f0
    Lifted::new(0x7000a4a2d0, tpl_568::<0x70006f52f0, 0x7000767d50, 0x700074a010, 0x7000e970f0, 0x7000f06960> as *const (), 2), // +0x35e2d0 sub_35e2d0
    Lifted::new(0x7000a4a630, fn_7000a4a630 as *const (), 0), // +0x35e630 sub_35e630
    Lifted::new(0x7000a4a710, fn_7000a4a710 as *const (), 2), // +0x35e710 sub_35e710
    Lifted::new(0x7000a4abc0, fn_7000a4abc0 as *const (), 2), // +0x35ebc0 sub_35ebc0
    Lifted::new(0x7000a4b080, tpl_569::<0x19, 0x10, 0x7000a4b080> as *const (), 0), // +0x35f080 sub_35f080
    Lifted::new(0x7000a4b0e0, fn_7000a4b0e0 as *const (), 0), // +0x35f0e0 sub_35f0e0
    Lifted::new(0x7000a4b320, tpl_394::<0x700076bf20, 0x7000a4d9e0, 0x7000a4b492> as *const (), 0), // +0x35f320 sub_35f320
    Lifted::new(0x7000a4b4f0, tpl_570::<0x8, 0x10, 0x18, 0x700076bf20, 0x700177c0e0, 0x7000a4da00, 0x7000a4b5a3> as *const (), 0), // +0x35f4f0 sub_35f4f0
    Lifted::new(0x7000a4b5e0, tpl_571::<0x7000a401b0> as *const (), 0), // +0x35f5e0 sub_35f5e0
    Lifted::new(0x7000a4b600, fn_7000a4b600 as *const (), 0), // +0x35f600 sub_35f600
    Lifted::new(0x7000a4b7f0, fn_7000a4b7f0 as *const (), 0), // +0x35f7f0 sub_35f7f0
    Lifted::new(0x7000a4b900, fn_7000a4b900 as *const (), 2), // +0x35f900 sub_35f900
    Lifted::new(0x7000a4bac0, fn_7000a4bac0 as *const (), 2), // +0x35fac0 sub_35fac0
    Lifted::new(0x7000a4bce0, fn_7000a4bce0 as *const (), 2), // +0x35fce0 sub_35fce0
    Lifted::new(0x7000a4be90, fn_7000a4be90 as *const (), 0), // +0x35fe90 sub_35fe90
    Lifted::new(0x7000a4bf80, fn_7000a4bf80 as *const (), 0), // +0x35ff80 sub_35ff80
    Lifted::new(0x7000a4c200, fn_7000a4c200 as *const (), 2), // +0x360200 sub_360200
    Lifted::new(0x7000a4c5d0, fn_7000a4c5d0 as *const (), 1), // +0x3605d0 sub_3605d0
    Lifted::new(0x7000a4c5f0, fn_7000a4c5f0 as *const (), 0), // +0x3605f0 sub_3605f0
    Lifted::new(0x7000a4c820, fn_7000a4c820 as *const (), 0), // +0x360820 sub_360820
    Lifted::new(0x7000a4cb20, fn_7000a4cb20 as *const (), 0), // +0x360b20 sub_360b20
    Lifted::new(0x7000a4cfa0, tpl_287 as *const (), 1), // +0x360fa0 sub_360fa0
    Lifted::new(0x7000a4d2c0, tpl_287 as *const (), 1), // +0x3612c0 sub_3612c0
    Lifted::new(0x7000a4d5e0, tpl_287 as *const (), 1), // +0x3615e0 sub_3615e0
    Lifted::new(0x7000a4d900, tpl_572::<0x10, 0x19, 0x8> as *const (), 1), // +0x361900 sub_361900
    Lifted::new(0x7000a4d970, tpl_573::<0x10, 0x19, 0x8> as *const (), 1), // +0x361970 sub_361970
    Lifted::new(0x7000a4d9e0, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a4d9f0> as *const (), 0), // +0x3619e0 sub_3619e0
    Lifted::new(0x7000a4da00, tpl_292::<0x70017d3768, 0x70017790e0, 0x7000a4da10> as *const (), 0), // +0x361a00 sub_361a00
    Lifted::new(0x7000a4da20, tpl_574::<0x700076bf20, 0x700177c0e0, 0x7000a4da00, 0x7000a4db66> as *const (), 0), // +0x361a20 sub_361a20
    Lifted::new(0x7000a4dbc0, fn_7000a4dbc0 as *const (), 0), // +0x361bc0 sub_361bc0
    Lifted::new(0x7000a4dc90, tpl_575::<0x10, 0x700177c312> as *const (), 0), // +0x361c90 sub_361c90
    Lifted::new(0x7000a4dcb0, fn_7000a4dcb0 as *const (), 0), // +0x361cb0 sub_361cb0
    Lifted::new(0x7000a4dd80, fn_7000a4dd80 as *const (), 0), // +0x361d80 sub_361d80
    Lifted::new(0x7000a4de00, fn_7000a4de00 as *const (), 0), // +0x361e00 sub_361e00
    Lifted::new(0x7000a4de80, fn_7000a4de80 as *const (), 1), // +0x361e80 sub_361e80
    Lifted::new(0x7000a4ded0, fn_7000a4ded0 as *const (), 0), // +0x361ed0 sub_361ed0
    Lifted::new(0x7000a4dff0, fn_7000a4dff0 as *const (), 0), // +0x361ff0 sub_361ff0
    Lifted::new(0x7000a4e0c0, fn_7000a4e0c0 as *const (), 0), // +0x3620c0 sub_3620c0
    Lifted::new(0x7000a4e3f0, fn_7000a4e3f0 as *const (), 0), // +0x3623f0 sub_3623f0
    Lifted::new(0x7000a4e6c0, fn_7000a4e6c0 as *const (), 0), // +0x3626c0 sub_3626c0
    Lifted::new(0x7000a4e830, tpl_576::<0x70017d92a0, 0x7000a4ded0, 0x8, 0x7000a4dcb0, 0x70017d92b0, 0x4, 0x700185bef0, 0x10, 0x70017d929c> as *const (), 0), // +0x362830 sub_362830
    Lifted::new(0x7000a4e8e0, fn_7000a4e8e0 as *const (), 0), // +0x3628e0 sub_3628e0
    Lifted::new(0x7000a4e970, fn_7000a4e970 as *const (), 0), // +0x362970 sub_362970
    Lifted::new(0x7000a4ebc0, fn_7000a4ebc0 as *const (), 0), // +0x362bc0 sub_362bc0
    Lifted::new(0x7000a4ed20, fn_7000a4ed20 as *const (), 0), // +0x362d20 sub_362d20
    Lifted::new(0x7000a4eda0, fn_7000a4eda0 as *const (), 0), // +0x362da0 sub_362da0
    Lifted::new(0x7000a4ee90, tpl_576::<0x70017d92a0, 0x7000a4ded0, 0x8, 0x7000a4dcb0, 0x70017d92b0, 0x4, 0x700185bef0, 0x10, 0x70017d929c> as *const (), 0), // +0x362e90 sub_362e90
    Lifted::new(0x7000a4ef50, tpl_577::<0x8, 0x7000a4dcb0, 0x10, 0x18> as *const (), 0), // +0x362f50 sub_362f50
    Lifted::new(0x7000a4efc0, fn_7000a4efc0 as *const (), 0), // +0x362fc0 sub_362fc0
    Lifted::new(0x7000a4f090, tpl_578::<0x8, 0x7000a4dcb0, 0x10> as *const (), 0), // +0x363090 sub_363090
    Lifted::new(0x7000a4f0e0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a4f0f1> as *const (), 0), // +0x3630e0 sub_3630e0
    Lifted::new(0x7000a4f120, tpl_577::<0x10, 0x7000a4dcb0, 0x18, 0x8> as *const (), 0), // +0x363120 sub_363120
    Lifted::new(0x7000a4f180, fn_7000a4f180 as *const (), 0), // +0x363180 sub_363180
    Lifted::new(0x7000a4f230, fn_7000a4f230 as *const (), 0), // +0x363230 sub_363230
    Lifted::new(0x7000a4f2c0, fn_7000a4f2c0 as *const (), 0), // +0x3632c0 sub_3632c0
    Lifted::new(0x7000a4f370, fn_7000a4f370 as *const (), 0), // +0x363370 sub_363370
    Lifted::new(0x7000a4f3e0, fn_7000a4f3e0 as *const (), 0), // +0x3633e0 sub_3633e0
    Lifted::new(0x7000a4f450, fn_7000a4f450 as *const (), 0), // +0x363450 sub_363450
    Lifted::new(0x7000a4f4e0, fn_7000a4f4e0 as *const (), 0), // +0x3634e0 sub_3634e0
    Lifted::new(0x7000a4f570, fn_7000a4f570 as *const (), 0), // +0x363570 sub_363570
    Lifted::new(0x7000a4f5b0, fn_7000a4f5b0 as *const (), 0), // +0x3635b0 sub_3635b0
    Lifted::new(0x7000a4f930, fn_7000a4f930 as *const (), 0), // +0x363930 sub_363930
    Lifted::new(0x7000a50c70, fn_7000a50c70 as *const (), 0), // +0x364c70 sub_364c70
    Lifted::new(0x7000a50df0, fn_7000a50df0 as *const (), 0), // +0x364df0 sub_364df0
    Lifted::new(0x7000a50f70, fn_7000a50f70 as *const (), 0), // +0x364f70 sub_364f70
    Lifted::new(0x7000a510b0, fn_7000a510b0 as *const (), 0), // +0x3650b0 sub_3650b0
    Lifted::new(0x7000a51130, fn_7000a51130 as *const (), 0), // +0x365130 sub_365130
    Lifted::new(0x7000a51210, fn_7000a51210 as *const (), 0), // +0x365210 sub_365210
    Lifted::new(0x7000a512a0, tpl_578::<0x8, 0x7000a4dcb0, 0x18> as *const (), 0), // +0x3652a0 sub_3652a0
    Lifted::new(0x7000a512f0, tpl_578::<0x8, 0x7000a4dcb0, 0x10> as *const (), 0), // +0x3652f0 sub_3652f0
    Lifted::new(0x7000a51340, fn_7000a51340 as *const (), 0), // +0x365340 sub_365340
    Lifted::new(0x7000a51450, tpl_578::<0x10, 0x7000a4dcb0, 0x8> as *const (), 0), // +0x365450 sub_365450
    Lifted::new(0x7000a514a0, tpl_578::<0x10, 0x7000a4dcb0, 0x18> as *const (), 0), // +0x3654a0 sub_3654a0
    Lifted::new(0x7000a51500, fn_7000a51500 as *const (), 0), // +0x365500 sub_365500
    Lifted::new(0x7000a51580, tpl_20 as *const (), 1), // +0x365580 sub_365580
    Lifted::new(0x7000a51590, fn_7000a51590 as *const (), 0), // +0x365590 sub_365590
    Lifted::new(0x7000a51660, fn_7000a51660 as *const (), 0), // +0x365660 sub_365660
    Lifted::new(0x7000a51740, tpl_579::<0x78, 0x74, 0x18, 0x70007a3d30, 0x60, 0x8, 0x80, 0x70007083e0, 0x10> as *const (), 0), // +0x365740 sub_365740
    Lifted::new(0x7000a517f0, tpl_579::<0x78, 0x74, 0x18, 0x70006f4370, 0x60, 0x8, 0x80, 0x700073a060, 0x10> as *const (), 0), // +0x3657f0 sub_3657f0
    Lifted::new(0x7000a518a0, fn_7000a518a0 as *const (), 0), // +0x3658a0 sub_3658a0
    Lifted::new(0x7000a51ad0, fn_7000a51ad0 as *const (), 0), // +0x365ad0 sub_365ad0
    Lifted::new(0x7000a51d40, fn_7000a51d40 as *const (), 0), // +0x365d40 sub_365d40
    Lifted::new(0x7000a51fd0, fn_7000a51fd0 as *const (), 0), // +0x365fd0 sub_365fd0
    Lifted::new(0x7000a52200, tpl_580::<0x10, 0x8, 0x7000a57430> as *const (), 0), // +0x366200 sub_366200
    Lifted::new(0x7000a522e0, tpl_581::<0x10, 0x8, 0x7000a57430> as *const (), 0), // +0x3662e0 sub_3662e0
    Lifted::new(0x7000a523c0, fn_7000a523c0 as *const (), 0), // +0x3663c0 sub_3663c0
    Lifted::new(0x7000a52600, tpl_55::<0x700177c0d0> as *const (), 0), // +0x366600 sub_366600
    Lifted::new(0x7000a52640, tpl_569::<0x19, 0x10, 0x7000a52640> as *const (), 0), // +0x366640 sub_366640
    Lifted::new(0x7000a526a0, tpl_569::<0x19, 0x10, 0x7000a526a0> as *const (), 0), // +0x3666a0 sub_3666a0
    Lifted::new(0x7000a52700, tpl_582::<0x8, 0x10, 0x7000a581d0> as *const (), 0), // +0x366700 sub_366700
    Lifted::new(0x7000a52730, tpl_583::<0x10, 0x8, 0x14, 0x18, 0x700076bf20, 0x700177c0b0> as *const (), 0), // +0x366730 sub_366730
    Lifted::new(0x7000a527d0, fn_7000a527d0 as *const (), 0), // +0x3667d0 sub_3667d0
    Lifted::new(0x7000a52af0, fn_7000a52af0 as *const (), 0), // +0x366af0 sub_366af0
    Lifted::new(0x7000a52b60, fn_7000a52b60 as *const (), 0), // +0x366b60 sub_366b60
    Lifted::new(0x7000a52cb0, fn_7000a52cb0 as *const (), 0), // +0x366cb0 sub_366cb0
    Lifted::new(0x7000a52d80, fn_7000a52d80 as *const (), 0), // +0x366d80 sub_366d80
    Lifted::new(0x7000a52e40, fn_7000a52e40 as *const (), 0), // +0x366e40 sub_366e40
    Lifted::new(0x7000a52ec0, fn_7000a52ec0 as *const (), 0), // +0x366ec0 sub_366ec0
    Lifted::new(0x7000a52f10, fn_7000a52f10 as *const (), 1), // +0x366f10 sub_366f10
    Lifted::new(0x7000a52f70, fn_7000a52f70 as *const (), 0), // +0x366f70 sub_366f70
    Lifted::new(0x7000a530e0, fn_7000a530e0 as *const (), 0), // +0x3670e0 sub_3670e0
    Lifted::new(0x7000a53270, fn_7000a53270 as *const (), 0), // +0x367270 sub_367270
    Lifted::new(0x7000a53310, fn_7000a53310 as *const (), 0), // +0x367310 sub_367310
    Lifted::new(0x7000a53420, tpl_583::<0x10, 0x8, 0x14, 0x18, 0x700076bf20, 0x700177c0b0> as *const (), 0), // +0x367420 sub_367420
    Lifted::new(0x7000a534c0, fn_7000a534c0 as *const (), 0), // +0x3674c0 sub_3674c0
    Lifted::new(0x7000a535a0, tpl_20 as *const (), 1), // +0x3675a0 sub_3675a0
    Lifted::new(0x7000a535b0, tpl_20 as *const (), 1), // +0x3675b0 sub_3675b0
    Lifted::new(0x7000a535c0, fn_7000a535c0 as *const (), 0), // +0x3675c0 sub_3675c0
    Lifted::new(0x7000a53c90, tpl_423::<0xc, 0x8, 0x18, 0x1, 0x10, 0x700076bf20, 0x70009fc7f0> as *const (), 0), // +0x367c90 sub_367c90
    Lifted::new(0x7000a53d70, fn_7000a53d70 as *const (), 0), // +0x367d70 sub_367d70
    Lifted::new(0x7000a53db0, tpl_584::<0x38, 0x18> as *const (), 1), // +0x367db0 sub_367db0
    Lifted::new(0x7000a53dd0, fn_7000a53dd0 as *const (), 1), // +0x367dd0 sub_367dd0
    Lifted::new(0x7000a53e10, fn_7000a53e10 as *const (), 0), // +0x367e10 sub_367e10
    Lifted::new(0x7000a53fc0, fn_7000a53fc0 as *const (), 1), // +0x367fc0 sub_367fc0
    Lifted::new(0x7000a53ff0, fn_7000a53ff0 as *const (), 0), // +0x367ff0 sub_367ff0
    Lifted::new(0x7000a540b0, fn_7000a540b0 as *const (), 1), // +0x3680b0 sub_3680b0
    Lifted::new(0x7000a54330, fn_7000a54330 as *const (), 0), // +0x368330 sub_368330
    Lifted::new(0x7000a543f0, fn_7000a543f0 as *const (), 0), // +0x3683f0 sub_3683f0
    Lifted::new(0x7000a54650, fn_7000a54650 as *const (), 0), // +0x368650 sub_368650
    Lifted::new(0x7000a54760, fn_7000a54760 as *const (), 0), // +0x368760 sub_368760
    Lifted::new(0x7000a54a20, fn_7000a54a20 as *const (), 0), // +0x368a20 sub_368a20
    Lifted::new(0x7000a54b40, fn_7000a54b40 as *const (), 1), // +0x368b40 sub_368b40
    Lifted::new(0x7000a54ba0, fn_7000a54ba0 as *const (), 0), // +0x368ba0 sub_368ba0
    Lifted::new(0x7000a54c80, fn_7000a54c80 as *const (), 0), // +0x368c80 sub_368c80
    Lifted::new(0x7000a54d00, fn_7000a54d00 as *const (), 1), // +0x368d00 sub_368d00
    Lifted::new(0x7000a54d30, fn_7000a54d30 as *const (), 1), // +0x368d30 sub_368d30
    Lifted::new(0x7000a54d70, fn_7000a54d70 as *const (), 2), // +0x368d70 sub_368d70
    Lifted::new(0x7000a54ed0, fn_7000a54ed0 as *const (), 2), // +0x368ed0 sub_368ed0
    Lifted::new(0x7000a55070, fn_7000a55070 as *const (), 2), // +0x369070 sub_369070
    Lifted::new(0x7000a550d0, tpl_20 as *const (), 1), // +0x3690d0 sub_3690d0
    Lifted::new(0x7000a550e0, tpl_20 as *const (), 1), // +0x3690e0 sub_3690e0
    Lifted::new(0x7000a550f0, tpl_20 as *const (), 1), // +0x3690f0 sub_3690f0
    Lifted::new(0x7000a55100, tpl_20 as *const (), 1), // +0x369100 sub_369100
    Lifted::new(0x7000a55110, tpl_20 as *const (), 1), // +0x369110 sub_369110
    Lifted::new(0x7000a55120, tpl_20 as *const (), 1), // +0x369120 sub_369120
    Lifted::new(0x7000a55130, fn_7000a55130 as *const (), 0), // +0x369130 sub_369130
    Lifted::new(0x7000a55450, fn_7000a55450 as *const (), 0), // +0x369450 sub_369450
    Lifted::new(0x7000a554a0, fn_7000a554a0 as *const (), 0), // +0x3694a0 sub_3694a0
    Lifted::new(0x7000a554d0, fn_7000a554d0 as *const (), 0), // +0x3694d0 sub_3694d0
    Lifted::new(0x7000a55540, fn_7000a55540 as *const (), 1), // +0x369540 sub_369540
    Lifted::new(0x7000a55580, fn_7000a55580 as *const (), 1), // +0x369580 sub_369580
    Lifted::new(0x7000a555c0, fn_7000a555c0 as *const (), 1), // +0x3695c0 sub_3695c0
    Lifted::new(0x7000a55660, fn_7000a55660 as *const (), 0), // +0x369660 sub_369660
    Lifted::new(0x7000a557b0, fn_7000a557b0 as *const (), 0), // +0x3697b0 sub_3697b0
    Lifted::new(0x7000a55860, fn_7000a55860 as *const (), 0), // +0x369860 sub_369860
    Lifted::new(0x7000a559e0, fn_7000a559e0 as *const (), 0), // +0x3699e0 sub_3699e0
    Lifted::new(0x7000a55a50, fn_7000a55a50 as *const (), 0), // +0x369a50 sub_369a50
    Lifted::new(0x7000a55b00, fn_7000a55b00 as *const (), 0), // +0x369b00 sub_369b00
    Lifted::new(0x7000a55cc0, fn_7000a55cc0 as *const (), 0), // +0x369cc0 sub_369cc0
    Lifted::new(0x7000a55d20, fn_7000a55d20 as *const (), 0), // +0x369d20 sub_369d20
    Lifted::new(0x7000a55ea0, fn_7000a55ea0 as *const (), 0), // +0x369ea0 sub_369ea0
    Lifted::new(0x7000a55f50, tpl_585::<0x28, 0xf0, 0x7000a55660, 0x38, 0x18> as *const (), 0), // +0x369f50 sub_369f50
    Lifted::new(0x7000a55fe0, fn_7000a55fe0 as *const (), 0), // +0x369fe0 sub_369fe0
    Lifted::new(0x7000a560c0, fn_7000a560c0 as *const (), 0), // +0x36a0c0 sub_36a0c0
    Lifted::new(0x7000a562b0, fn_7000a562b0 as *const (), 0), // +0x36a2b0 sub_36a2b0
    Lifted::new(0x7000a564a0, tpl_584::<0xa0, 0x18> as *const (), 1), // +0x36a4a0 sub_36a4a0
    Lifted::new(0x7000a564c0, tpl_585::<0x28, 0x108, 0x7000a55660, 0x38, 0x18> as *const (), 0), // +0x36a4c0 sub_36a4c0
    Lifted::new(0x7000a56550, fn_7000a56550 as *const (), 0), // +0x36a550 sub_36a550
    Lifted::new(0x7000a56640, fn_7000a56640 as *const (), 0), // +0x36a640 sub_36a640
    Lifted::new(0x7000a56830, fn_7000a56830 as *const (), 0), // +0x36a830 sub_36a830
    Lifted::new(0x7000a56910, fn_7000a56910 as *const (), 1), // +0x36a910 sub_36a910
    Lifted::new(0x7000a56920, fn_7000a56920 as *const (), 0), // +0x36a920 sub_36a920
    Lifted::new(0x7000a569c0, fn_7000a569c0 as *const (), 0), // +0x36a9c0 sub_36a9c0
    Lifted::new(0x7000a57430, fn_7000a57430 as *const (), 0), // +0x36b430 sub_36b430
    Lifted::new(0x7000a575c0, fn_7000a575c0 as *const (), 1), // +0x36b5c0 sub_36b5c0
    Lifted::new(0x7000a57700, fn_7000a57700 as *const (), 1), // +0x36b700 sub_36b700
    Lifted::new(0x7000a57820, tpl_586::<0x7a, 0x78> as *const (), 1), // +0x36b820 sub_36b820
    Lifted::new(0x7000a57850, tpl_586::<0x7a, 0x78> as *const (), 1), // +0x36b850 sub_36b850
    Lifted::new(0x7000a57880, fn_7000a57880 as *const (), 1), // +0x36b880 sub_36b880
    Lifted::new(0x7000a578a0, fn_7000a578a0 as *const (), 1), // +0x36b8a0 sub_36b8a0
    Lifted::new(0x7000a57920, fn_7000a57920 as *const (), 1), // +0x36b920 sub_36b920
    Lifted::new(0x7000a57940, fn_7000a57940 as *const (), 1), // +0x36b940 sub_36b940
    Lifted::new(0x7000a57960, fn_7000a57960 as *const (), 1), // +0x36b960 sub_36b960
    Lifted::new(0x7000a57990, fn_7000a57990 as *const (), 1), // +0x36b990 sub_36b990
    Lifted::new(0x7000a579b0, fn_7000a579b0 as *const (), 1), // +0x36b9b0 sub_36b9b0
    Lifted::new(0x7000a579e0, fn_7000a579e0 as *const (), 1), // +0x36b9e0 sub_36b9e0
    Lifted::new(0x7000a57a20, fn_7000a57a20 as *const (), 0), // +0x36ba20 sub_36ba20
    Lifted::new(0x7000a57a90, fn_7000a57a90 as *const (), 0), // +0x36ba90 sub_36ba90
    Lifted::new(0x7000a581d0, fn_7000a581d0 as *const (), 0), // +0x36c1d0 sub_36c1d0
    Lifted::new(0x7000a58290, fn_7000a58290 as *const (), 1), // +0x36c290 sub_36c290
    Lifted::new(0x7000a582e0, fn_7000a582e0 as *const (), 1), // +0x36c2e0 sub_36c2e0
    Lifted::new(0x7000a58300, fn_7000a58300 as *const (), 1), // +0x36c300 sub_36c300
    Lifted::new(0x7000a58390, fn_7000a58390 as *const (), 1), // +0x36c390 sub_36c390
    Lifted::new(0x7000a58400, fn_7000a58400 as *const (), 0), // +0x36c400 sub_36c400
    Lifted::new(0x7000a585b0, fn_7000a585b0 as *const (), 1), // +0x36c5b0 sub_36c5b0
    Lifted::new(0x7000a58640, tpl_581::<0x10, 0x8, 0x7000a57430> as *const (), 0), // +0x36c640 sub_36c640
    Lifted::new(0x7000a58720, fn_7000a58720 as *const (), 0), // +0x36c720 sub_36c720
    Lifted::new(0x7000a588a0, fn_7000a588a0 as *const (), 0), // +0x36c8a0 sub_36c8a0
    Lifted::new(0x7000a58960, fn_7000a58960 as *const (), 0), // +0x36c960 sub_36c960
    Lifted::new(0x7000a58b70, fn_7000a58b70 as *const (), 0), // +0x36cb70 sub_36cb70
    Lifted::new(0x7000a58ce0, fn_7000a58ce0 as *const (), 0), // +0x36cce0 sub_36cce0
    Lifted::new(0x7000a58ee0, fn_7000a58ee0 as *const (), 0), // +0x36cee0 sub_36cee0
    Lifted::new(0x7000a58f70, fn_7000a58f70 as *const (), 1), // +0x36cf70 sub_36cf70
    Lifted::new(0x7000a58fc0, fn_7000a58fc0 as *const (), 1), // +0x36cfc0 sub_36cfc0
    Lifted::new(0x7000a58fe0, fn_7000a58fe0 as *const (), 0), // +0x36cfe0 sub_36cfe0
    Lifted::new(0x7000a59140, fn_7000a59140 as *const (), 0), // +0x36d140 sub_36d140
    Lifted::new(0x7000a59190, fn_7000a59190 as *const (), 0), // +0x36d190 sub_36d190
    Lifted::new(0x7000a59200, fn_7000a59200 as *const (), 1), // +0x36d200 sub_36d200
    Lifted::new(0x7000a59280, fn_7000a59280 as *const (), 1), // +0x36d280 sub_36d280
    Lifted::new(0x7000a592d0, fn_7000a592d0 as *const (), 1), // +0x36d2d0 sub_36d2d0
    Lifted::new(0x7000a59300, tpl_587 as *const (), 3), // +0x36d300 sub_36d300
    Lifted::new(0x7000a59450, fn_7000a59450 as *const (), 1), // +0x36d450 sub_36d450
    Lifted::new(0x7000a59660, fn_7000a59660 as *const (), 3), // +0x36d660 sub_36d660
    Lifted::new(0x7000a59780, tpl_588::<0x7000a59450> as *const (), 0), // +0x36d780 sub_36d780
    Lifted::new(0x7000a59790, fn_7000a59790 as *const (), 0), // +0x36d790 sub_36d790
    Lifted::new(0x7000a59960, fn_7000a59960 as *const (), 1), // +0x36d960 sub_36d960
    Lifted::new(0x7000a599f0, fn_7000a599f0 as *const (), 0), // +0x36d9f0 sub_36d9f0
    Lifted::new(0x7000a59b50, fn_7000a59b50 as *const (), 1), // +0x36db50 sub_36db50
    Lifted::new(0x7000a59b90, fn_7000a59b90 as *const (), 0), // +0x36db90 sub_36db90
    Lifted::new(0x7000a59c70, tpl_513::<0x10, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0, 0x8> as *const (), 0), // +0x36dc70 sub_36dc70
    Lifted::new(0x7000a59d10, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x36dd10 sub_36dd10
    Lifted::new(0x7000a59dd0, tpl_514::<0x8, 0x10, 0x1, 0x18, 0x700076bf20, 0xc, 0x70009fc7f0> as *const (), 0), // +0x36ddd0 sub_36ddd0
    Lifted::new(0x7000a59e90, fn_7000a59e90 as *const (), 0), // +0x36de90 sub_36de90
    Lifted::new(0x7000a59fb0, fn_7000a59fb0 as *const (), 0), // +0x36dfb0 sub_36dfb0
    Lifted::new(0x7000a5a150, fn_7000a5a150 as *const (), 0), // +0x36e150 sub_36e150
    Lifted::new(0x7000a5a520, fn_7000a5a520 as *const (), 0), // +0x36e520 sub_36e520
    Lifted::new(0x7000a5a550, fn_7000a5a550 as *const (), 0), // +0x36e550 sub_36e550
    Lifted::new(0x7000a5a5b0, fn_7000a5a5b0 as *const (), 0), // +0x36e5b0 sub_36e5b0
    Lifted::new(0x7000a5a7b0, fn_7000a5a7b0 as *const (), 0), // +0x36e7b0 sub_36e7b0
    Lifted::new(0x7000a5a860, fn_7000a5a860 as *const (), 0), // +0x36e860 sub_36e860
    Lifted::new(0x7000a5a910, fn_7000a5a910 as *const (), 0), // +0x36e910 sub_36e910
    Lifted::new(0x7000a5a940, fn_7000a5a940 as *const (), 1), // +0x36e940 sub_36e940
    Lifted::new(0x7000a5a9f0, fn_7000a5a9f0 as *const (), 1), // +0x36e9f0 sub_36e9f0
    Lifted::new(0x7000a5aa70, fn_7000a5aa70 as *const (), 1), // +0x36ea70 sub_36ea70
    Lifted::new(0x7000a5aab0, fn_7000a5aab0 as *const (), 1), // +0x36eab0 sub_36eab0
    Lifted::new(0x7000a5ab80, fn_7000a5ab80 as *const (), 1), // +0x36eb80 sub_36eb80
    Lifted::new(0x7000a5aba0, tpl_589::<0x80> as *const (), 1), // +0x36eba0 sub_36eba0
    Lifted::new(0x7000a5abb0, fn_7000a5abb0 as *const (), 1), // +0x36ebb0 sub_36ebb0
    Lifted::new(0x7000a5ac90, tpl_590::<0x7001779e20, 0x700177c170, 0x70017d37ec, 0x7000a62120, 0x70017d37c0, 0x70017790e0, 0x7000a5af3a> as *const (), 2), // +0x36ec90 sub_36ec90
    Lifted::new(0x7000a5aff0, tpl_68 as *const (), 1), // +0x36eff0 sub_36eff0
    Lifted::new(0x7000a5b0f0, fn_7000a5b0f0 as *const (), 0), // +0x36f0f0 sub_36f0f0
    Lifted::new(0x7000a5b190, fn_7000a5b190 as *const (), 0), // +0x36f190 sub_36f190
    Lifted::new(0x7000a5b320, tpl_552::<0x8, 0x700077aad0, 0x7000a5e910> as *const (), 0), // +0x36f320 sub_36f320
    Lifted::new(0x7000a5b380, tpl_591::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x36f380 sub_36f380
    Lifted::new(0x7000a5b590, tpl_592::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x36f590 sub_36f590
    Lifted::new(0x7000a5b770, tpl_593::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x36f770 sub_36f770
    Lifted::new(0x7000a5b930, tpl_592::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x36f930 sub_36f930
    Lifted::new(0x7000a5bb10, fn_7000a5bb10 as *const (), 0), // +0x36fb10 sub_36fb10
    Lifted::new(0x7000a5bc50, fn_7000a5bc50 as *const (), 0), // +0x36fc50 sub_36fc50
    Lifted::new(0x7000a5be30, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x36fe30 sub_36fe30
    Lifted::new(0x7000a5bfe0, tpl_595::<0x7000a5f6e0, 0x18, 0x7000a5b320, 0x8, 0x1> as *const (), 0), // +0x36ffe0 sub_36ffe0
    Lifted::new(0x7000a5c0a0, tpl_595::<0x7000a5f6e0, 0x18, 0x7000a5b320, 0x8, 0x1> as *const (), 0), // +0x3700a0 sub_3700a0
    Lifted::new(0x7000a5c160, tpl_591::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x370160 sub_370160
    Lifted::new(0x7000a5c370, fn_7000a5c370 as *const (), 0), // +0x370370 sub_370370
    Lifted::new(0x7000a5c590, tpl_592::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x370590 sub_370590
    Lifted::new(0x7000a5c780, fn_7000a5c780 as *const (), 0), // +0x370780 sub_370780
    Lifted::new(0x7000a5c810, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x370810 sub_370810
    Lifted::new(0x7000a5c9c0, tpl_593::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x3709c0 sub_3709c0
    Lifted::new(0x7000a5cb80, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x370b80 sub_370b80
    Lifted::new(0x7000a5cd30, fn_7000a5cd30 as *const (), 0), // +0x370d30 sub_370d30
    Lifted::new(0x7000a5cdc0, tpl_596::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x370dc0 sub_370dc0
    Lifted::new(0x7000a5cf90, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x370f90 sub_370f90
    Lifted::new(0x7000a5d150, tpl_597::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x371150 sub_371150
    Lifted::new(0x7000a5d310, tpl_598::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x371310 sub_371310
    Lifted::new(0x7000a5d4f0, tpl_598::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x3714f0 sub_3714f0
    Lifted::new(0x7000a5d6d0, fn_7000a5d6d0 as *const (), 0), // +0x3716d0 sub_3716d0
    Lifted::new(0x7000a5d770, fn_7000a5d770 as *const (), 0), // +0x371770 sub_371770
    Lifted::new(0x7000a5d930, fn_7000a5d930 as *const (), 0), // +0x371930 sub_371930
    Lifted::new(0x7000a5d9f0, fn_7000a5d9f0 as *const (), 0), // +0x3719f0 sub_3719f0
    Lifted::new(0x7000a5dad0, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x371ad0 sub_371ad0
    Lifted::new(0x7000a5dc80, fn_7000a5dc80 as *const (), 0), // +0x371c80 sub_371c80
    Lifted::new(0x7000a5de60, fn_7000a5de60 as *const (), 0), // +0x371e60 sub_371e60
    Lifted::new(0x7000a5e050, fn_7000a5e050 as *const (), 0), // +0x372050 sub_372050
    Lifted::new(0x7000a5e270, fn_7000a5e270 as *const (), 0), // +0x372270 sub_372270
    Lifted::new(0x7000a5e360, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x372360 sub_372360
    Lifted::new(0x7000a5e510, fn_7000a5e510 as *const (), 0), // +0x372510 sub_372510
    Lifted::new(0x7000a5e760, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x372760 sub_372760
    Lifted::new(0x7000a5e910, fn_7000a5e910 as *const (), 0), // +0x372910 sub_372910
    Lifted::new(0x7000a5f6e0, fn_7000a5f6e0 as *const (), 0), // +0x3736e0 sub_3736e0
    Lifted::new(0x7000a5f890, fn_7000a5f890 as *const (), 0), // +0x373890 sub_373890
    Lifted::new(0x7000a5f970, tpl_593::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x373970 sub_373970
    Lifted::new(0x7000a5fb30, tpl_592::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x373b30 sub_373b30
    Lifted::new(0x7000a5fd10, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x373d10 sub_373d10
    Lifted::new(0x7000a5fec0, tpl_597::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x373ec0 sub_373ec0
    Lifted::new(0x7000a60080, fn_7000a60080 as *const (), 0), // +0x374080 sub_374080
    Lifted::new(0x7000a60240, fn_7000a60240 as *const (), 0), // +0x374240 sub_374240
    Lifted::new(0x7000a60400, tpl_559::<0xc, 0x7000a5b320> as *const (), 0), // +0x374400 sub_374400
    Lifted::new(0x7000a60480, fn_7000a60480 as *const (), 0), // +0x374480 sub_374480
    Lifted::new(0x7000a604f0, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x3744f0 sub_3744f0
    Lifted::new(0x7000a606a0, fn_7000a606a0 as *const (), 0), // +0x3746a0 sub_3746a0
    Lifted::new(0x7000a607c0, fn_7000a607c0 as *const (), 0), // +0x3747c0 sub_3747c0
    Lifted::new(0x7000a60930, fn_7000a60930 as *const (), 0), // +0x374930 sub_374930
    Lifted::new(0x7000a60b30, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x374b30 sub_374b30
    Lifted::new(0x7000a60ce0, tpl_593::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x374ce0 sub_374ce0
    Lifted::new(0x7000a60ea0, fn_7000a60ea0 as *const (), 0), // +0x374ea0 sub_374ea0
    Lifted::new(0x7000a61070, fn_7000a61070 as *const (), 0), // +0x375070 sub_375070
    Lifted::new(0x7000a61140, tpl_596::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x375140 sub_375140
    Lifted::new(0x7000a61310, fn_7000a61310 as *const (), 0), // +0x375310 sub_375310
    Lifted::new(0x7000a61370, tpl_593::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x375370 sub_375370
    Lifted::new(0x7000a61530, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x375530 sub_375530
    Lifted::new(0x7000a616e0, tpl_594::<0x7000a5ac90> as *const (), 0), // +0x3756e0 sub_3756e0
    Lifted::new(0x7000a61890, fn_7000a61890 as *const (), 0), // +0x375890 sub_375890
    Lifted::new(0x7000a61a60, fn_7000a61a60 as *const (), 0), // +0x375a60 sub_375a60
    Lifted::new(0x7000a61c30, tpl_593::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x375c30 sub_375c30
    Lifted::new(0x7000a61df0, tpl_593::<0x7000a5ac90, 0x7000a5b320> as *const (), 0), // +0x375df0 sub_375df0
    Lifted::new(0x7000a61fb0, tpl_277::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000a620b7, 0x7000a5aff0, 0x70007f0910, 0x7000a620d3> as *const (), 0), // +0x375fb0 sub_375fb0
    Lifted::new(0x7000a62120, tpl_284::<0x7000a61fb0, 0x70017d37a0, 0x70017790e0, 0x7000a622dd> as *const (), 0), // +0x376120 sub_376120
    Lifted::new(0x7000a62350, fn_7000a62350 as *const (), 1), // +0x376350 sub_376350
    Lifted::new(0x7000a62390, fn_7000a62390 as *const (), 1), // +0x376390 sub_376390
    Lifted::new(0x7000a623e0, fn_7000a623e0 as *const (), 1), // +0x3763e0 sub_3763e0
    Lifted::new(0x7000a62410, fn_7000a62410 as *const (), 0), // +0x376410 sub_376410
    Lifted::new(0x7000a62630, tpl_63::<0x19, 0x10, 0x7000a62630, 0x700177c312> as *const (), 0), // +0x376630 sub_376630
    Lifted::new(0x7000a626a0, tpl_599::<0x19, 0x10, 0x7000a626a0, 0x28, 0x38, 0xfffffffffffffff8, 0x700177c312, 0x30, 0x7001d54968, 0x7000a6275e> as *const (), 0), // +0x3766a0 sub_3766a0
    Lifted::new(0x7000a62790, fn_7000a62790 as *const (), 1), // +0x376790 sub_376790
    Lifted::new(0x7000a627c0, fn_7000a627c0 as *const (), 1), // +0x3767c0 sub_3767c0
    Lifted::new(0x7000a627e0, tpl_20 as *const (), 1), // +0x3767e0 sub_3767e0
    Lifted::new(0x7000a627f0, fn_7000a627f0 as *const (), 0), // +0x3767f0 sub_3767f0
    Lifted::new(0x7000a62ac0, tpl_600::<0x7000799f60, 0x70017d9580> as *const (), 0), // +0x376ac0 sub_376ac0
    Lifted::new(0x7000a62b10, tpl_601::<0x7000799f60, 0x70017d9580> as *const (), 0), // +0x376b10 sub_376b10
    Lifted::new(0x7000a62b80, tpl_88::<0x700077d6c0, 0x70017d9580> as *const (), 0), // +0x376b80 sub_376b80
    Lifted::new(0x7000a62bb0, tpl_602::<0x70006fda20, 0x70017d9580> as *const (), 0), // +0x376bb0 sub_376bb0
    Lifted::new(0x7000a62c20, fn_7000a62c20 as *const (), 1), // +0x376c20 sub_376c20
    Lifted::new(0x7000a62c40, fn_7000a62c40 as *const (), 1), // +0x376c40 sub_376c40
    Lifted::new(0x7000a62cf0, fn_7000a62cf0 as *const (), 1), // +0x376cf0 sub_376cf0
    Lifted::new(0x7000a62e00, fn_7000a62e00 as *const (), 1), // +0x376e00 sub_376e00
    Lifted::new(0x7000a62e40, fn_7000a62e40 as *const (), 0), // +0x376e40 sub_376e40
    Lifted::new(0x7000a62ed0, tpl_600::<0x7000799f60, 0x70017d9500> as *const (), 0), // +0x376ed0 sub_376ed0
    Lifted::new(0x7000a62f20, tpl_601::<0x7000799f60, 0x70017d9500> as *const (), 0), // +0x376f20 sub_376f20
    Lifted::new(0x7000a62f90, fn_7000a62f90 as *const (), 2), // +0x376f90 sub_376f90
    Lifted::new(0x7000a63030, tpl_602::<0x70006fda20, 0x70017d9500> as *const (), 0), // +0x377030 sub_377030
    Lifted::new(0x7000a630a0, tpl_600::<0x7000799f60, 0x70017d9540> as *const (), 0), // +0x3770a0 sub_3770a0
    Lifted::new(0x7000a630f0, tpl_601::<0x7000799f60, 0x70017d9540> as *const (), 0), // +0x3770f0 sub_3770f0
    Lifted::new(0x7000a63160, tpl_88::<0x700077d6c0, 0x70017d9540> as *const (), 0), // +0x377160 sub_377160
    Lifted::new(0x7000a63190, tpl_602::<0x70006fda20, 0x70017d9540> as *const (), 0), // +0x377190 sub_377190
    Lifted::new(0x7000a63200, fn_7000a63200 as *const (), 2), // +0x377200 sub_377200
    Lifted::new(0x7000a632d0, fn_7000a632d0 as *const (), 0), // +0x3772d0 sub_3772d0
    Lifted::new(0x7000a63340, fn_7000a63340 as *const (), 0), // +0x377340 sub_377340
    Lifted::new(0x7000a633c0, fn_7000a633c0 as *const (), 0), // +0x3773c0 sub_3773c0
    Lifted::new(0x7000a634b0, tpl_603::<0x20, 0xffffffffffffffff, 0x18, 0x10, 0x8, 0x700177c312, 0xfffffffffffffff8, 0x7001d54968, 0x7000a63573> as *const (), 0), // +0x3774b0 sub_3774b0
    Lifted::new(0x7000a635b0, fn_7000a635b0 as *const (), 0), // +0x3775b0 sub_3775b0
    Lifted::new(0x7000a63720, tpl_106::<0x8, 0x700072ea80> as *const (), 0), // +0x377720 sub_377720
    Lifted::new(0x7000a63750, tpl_16::<0x7000a63753, 0x7000a63754, 0x7000a63755, 0x7000a63756, 0x7000a63757, 0x7000a63758, 0x7000a63759, 0x7000a6375a, 0x7000a6375b, 0x7000a6375c, 0x7000a6375d, 0x7000a6375e, 0x7000a6375f, 0x7000718060> as *const (), 0), // +0x377750 sub_377750
    Lifted::new(0x7000a63770, fn_7000a63770 as *const (), 0), // +0x377770 sub_377770
    Lifted::new(0x7000a637f0, fn_7000a637f0 as *const (), 1), // +0x3777f0 sub_3777f0
    Lifted::new(0x7000a63860, tpl_21::<0x10> as *const (), 3), // +0x377860 sub_377860
    Lifted::new(0x7000a63880, fn_7000a63880 as *const (), 1), // +0x377880 sub_377880
    Lifted::new(0x7000a638a0, tpl_21::<0x10> as *const (), 3), // +0x3778a0 sub_3778a0
    Lifted::new(0x7000a638c0, tpl_20 as *const (), 1), // +0x3778c0 sub_3778c0
    Lifted::new(0x7000a638d0, tpl_121::<0x8> as *const (), 3), // +0x3778d0 sub_3778d0
    Lifted::new(0x7000a638f0, tpl_604::<0xfffffffffffffffc, 0x7000a63b70> as *const (), 0), // +0x3778f0 sub_3778f0
    Lifted::new(0x7000a63900, tpl_605::<0xfffffffffffffff8, 0x7000718060, 0x700177c312, 0x7000701460> as *const (), 0), // +0x377900 sub_377900
    Lifted::new(0x7000a639d0, tpl_605::<0xfffffffffffffff8, 0x7000718060, 0x700177c312, 0x7000701460> as *const (), 0), // +0x3779d0 sub_3779d0
    Lifted::new(0x7000a63aa0, tpl_605::<0xfffffffffffffff8, 0x7000718060, 0x700177c312, 0x7000701460> as *const (), 0), // +0x377aa0 sub_377aa0
    Lifted::new(0x7000a63b70, fn_7000a63b70 as *const (), 0), // +0x377b70 sub_377b70
    Lifted::new(0x7000a63c20, tpl_133::<0x7000a633c0, 0x700177c312> as *const (), 0), // +0x377c20 sub_377c20
    Lifted::new(0x7000a63c60, tpl_133::<0x7000718060, 0x7000701460> as *const (), 0), // +0x377c60 sub_377c60
    Lifted::new(0x7000a63ca0, tpl_133::<0x7000718060, 0x7000701460> as *const (), 0), // +0x377ca0 sub_377ca0
    Lifted::new(0x7000a63ce0, tpl_133::<0x7000718060, 0x7000701460> as *const (), 0), // +0x377ce0 sub_377ce0
    Lifted::new(0x7000a63d20, tpl_15 as *const (), 1), // +0x377d20 sub_377d20
    Lifted::new(0x7000a63d30, fn_7000a63d30 as *const (), 0), // +0x377d30 sub_377d30
    Lifted::new(0x7000a63d70, fn_7000a63d70 as *const (), 0), // +0x377d70 sub_377d70
    Lifted::new(0x7000a63dc0, fn_7000a63dc0 as *const (), 0), // +0x377dc0 sub_377dc0
    Lifted::new(0x7000a63e20, tpl_15 as *const (), 1), // +0x377e20 sub_377e20
    Lifted::new(0x7000a63e30, tpl_191::<0x1b8> as *const (), 1), // +0x377e30 sub_377e30
    Lifted::new(0x7000a63e50, fn_7000a63e50 as *const (), 1), // +0x377e50 sub_377e50
    Lifted::new(0x7000a63e70, fn_7000a63e70 as *const (), 2), // +0x377e70 sub_377e70
    Lifted::new(0x7000a64230, tpl_606::<0x7001c7224c, 0x7000701070> as *const (), 0), // +0x378230 sub_378230
    Lifted::new(0x7000a64250, fn_7000a64250 as *const (), 1), // +0x378250 sub_378250
    Lifted::new(0x7000a64270, fn_7000a64270 as *const (), 0), // +0x378270 sub_378270
    Lifted::new(0x7000a64720, fn_7000a64720 as *const (), 0), // +0x378720 sub_378720
    Lifted::new(0x7000a647d0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a647e1> as *const (), 0), // +0x3787d0 sub_3787d0
    Lifted::new(0x7000a647f0, tpl_15 as *const (), 1), // +0x3787f0 sub_3787f0
    Lifted::new(0x7000a64800, tpl_16::<0x7000a64803, 0x7000a64804, 0x7000a64805, 0x7000a64806, 0x7000a64807, 0x7000a64808, 0x7000a64809, 0x7000a6480a, 0x7000a6480b, 0x7000a6480c, 0x7000a6480d, 0x7000a6480e, 0x7000a6480f, 0x7000763f70> as *const (), 0), // +0x378800 sub_378800
    Lifted::new(0x7000a64820, fn_7000a64820 as *const (), 1), // +0x378820 sub_378820
    Lifted::new(0x7000a64930, fn_7000a64930 as *const (), 0), // +0x378930 sub_378930
    Lifted::new(0x7000a64950, tpl_606::<0x7001c7224c, 0x70007692c0> as *const (), 0), // +0x378950 sub_378950
    Lifted::new(0x7000a64970, fn_7000a64970 as *const (), 0), // +0x378970 sub_378970
    Lifted::new(0x7000a649d0, fn_7000a649d0 as *const (), 1), // +0x3789d0 sub_3789d0
    Lifted::new(0x7000a649f0, tpl_607::<0x8, 0x20> as *const (), 0), // +0x3789f0 sub_3789f0
    Lifted::new(0x7000a64a00, tpl_608::<0x700077ddc0, 0x3, 0x700079e7f0> as *const (), 0), // +0x378a00 sub_378a00
    Lifted::new(0x7000a64a70, tpl_608::<0x700077ddc0, 0xb, 0x700079e7f0> as *const (), 0), // +0x378a70 sub_378a70
    Lifted::new(0x7000a64ae0, fn_7000a64ae0 as *const (), 2), // +0x378ae0 sub_378ae0
    Lifted::new(0x7000a64b40, fn_7000a64b40 as *const (), 0), // +0x378b40 sub_378b40
    Lifted::new(0x7000a64b90, fn_7000a64b90 as *const (), 2), // +0x378b90 sub_378b90
    Lifted::new(0x7000a64bf0, fn_7000a64bf0 as *const (), 0), // +0x378bf0 sub_378bf0
    Lifted::new(0x7000a64c40, fn_7000a64c40 as *const (), 0), // +0x378c40 sub_378c40
    Lifted::new(0x7000a64c50, tpl_609::<0x70017d94f4, 0x7000780ab0, 0x8, 0x2, 0x70007a5dd0> as *const (), 2), // +0x378c50 sub_378c50
    Lifted::new(0x7000a64cc0, fn_7000a64cc0 as *const (), 0), // +0x378cc0 sub_378cc0
    Lifted::new(0x7000a64e10, tpl_15 as *const (), 1), // +0x378e10 sub_378e10
    Lifted::new(0x7000a64e20, fn_7000a64e20 as *const (), 2), // +0x378e20 sub_378e20
    Lifted::new(0x7000a64e90, tpl_610::<0x8, 0x7000733ea0> as *const (), 2), // +0x378e90 sub_378e90
    Lifted::new(0x7000a64ed0, fn_7000a64ed0 as *const (), 2), // +0x378ed0 sub_378ed0
    Lifted::new(0x7000a64fa0, fn_7000a64fa0 as *const (), 2), // +0x378fa0 sub_378fa0
    Lifted::new(0x7000a64fe0, fn_7000a64fe0 as *const (), 2), // +0x378fe0 sub_378fe0
    Lifted::new(0x7000a65000, fn_7000a65000 as *const (), 2), // +0x379000 sub_379000
    Lifted::new(0x7000a65020, fn_7000a65020 as *const (), 0), // +0x379020 sub_379020
    Lifted::new(0x7000a65040, fn_7000a65040 as *const (), 0), // +0x379040 sub_379040
    Lifted::new(0x7000a65070, tpl_15 as *const (), 1), // +0x379070 sub_379070
    Lifted::new(0x7000a65080, tpl_15 as *const (), 1), // +0x379080 sub_379080
    Lifted::new(0x7000a65090, tpl_432::<0x7000a63340> as *const (), 0), // +0x379090 sub_379090
    Lifted::new(0x7000a650a0, tpl_15 as *const (), 1), // +0x3790a0 sub_3790a0
    Lifted::new(0x7000a650b0, fn_7000a650b0 as *const (), 1), // +0x3790b0 sub_3790b0
    Lifted::new(0x7000a650d0, fn_7000a650d0 as *const (), 0), // +0x3790d0 sub_3790d0
    Lifted::new(0x7000a651f0, fn_7000a651f0 as *const (), 1), // +0x3791f0 sub_3791f0
    Lifted::new(0x7000a65220, fn_7000a65220 as *const (), 1), // +0x379220 sub_379220
    Lifted::new(0x7000a65250, fn_7000a65250 as *const (), 0), // +0x379250 sub_379250
    Lifted::new(0x7000a652a0, fn_7000a652a0 as *const (), 0), // +0x3792a0 sub_3792a0
    Lifted::new(0x7000a652d0, fn_7000a652d0 as *const (), 2), // +0x3792d0 sub_3792d0
    Lifted::new(0x7000a65310, fn_7000a65310 as *const (), 0), // +0x379310 sub_379310
    Lifted::new(0x7000a65330, fn_7000a65330 as *const (), 2), // +0x379330 sub_379330
    Lifted::new(0x7000a65370, fn_7000a65370 as *const (), 1), // +0x379370 sub_379370
    Lifted::new(0x7000a65380, fn_7000a65380 as *const (), 0), // +0x379380 sub_379380
    Lifted::new(0x7000a653b0, fn_7000a653b0 as *const (), 2), // +0x3793b0 sub_3793b0
    Lifted::new(0x7000a653f0, fn_7000a653f0 as *const (), 0), // +0x3793f0 sub_3793f0
    Lifted::new(0x7000a65420, fn_7000a65420 as *const (), 2), // +0x379420 sub_379420
    Lifted::new(0x7000a65460, tpl_611::<0x27, 0x7001779e20, 0xfffffffffffffff8, 0x7001d54968, 0x7000a6549a, 0x70007f0910, 0x7000a654b8> as *const (), 0), // +0x379460 sub_379460
    Lifted::new(0x7000a654d0, fn_7000a654d0 as *const (), 0), // +0x3794d0 sub_3794d0
    Lifted::new(0x7000a654e0, tpl_612::<0x700074b810> as *const (), 0), // +0x3794e0 sub_3794e0
    Lifted::new(0x7000a65510, tpl_613::<0x8, 0x700071d060> as *const (), 2), // +0x379510 sub_379510
    Lifted::new(0x7000a65550, fn_7000a65550 as *const (), 0), // +0x379550 sub_379550
    Lifted::new(0x7000a65580, fn_7000a65580 as *const (), 2), // +0x379580 sub_379580
    Lifted::new(0x7000a655c0, fn_7000a655c0 as *const (), 0), // +0x3795c0 sub_3795c0
    Lifted::new(0x7000a655e0, fn_7000a655e0 as *const (), 2), // +0x3795e0 sub_3795e0
    Lifted::new(0x7000a65620, tpl_612::<0x7000780530> as *const (), 0), // +0x379620 sub_379620
    Lifted::new(0x7000a65650, tpl_613::<0x8, 0x70006f1e30> as *const (), 2), // +0x379650 sub_379650
    Lifted::new(0x7000a65690, fn_7000a65690 as *const (), 0), // +0x379690 sub_379690
    Lifted::new(0x7000a656c0, fn_7000a656c0 as *const (), 2), // +0x3796c0 sub_3796c0
    Lifted::new(0x7000a65700, fn_7000a65700 as *const (), 0), // +0x379700 sub_379700
    Lifted::new(0x7000a65720, fn_7000a65720 as *const (), 2), // +0x379720 sub_379720
    Lifted::new(0x7000a65760, tpl_379::<0x118, 0x20> as *const (), 1), // +0x379760 sub_379760
    Lifted::new(0x7000a65770, tpl_614::<0x700074b810> as *const (), 0), // +0x379770 sub_379770
    Lifted::new(0x7000a65790, tpl_615::<0x8, 0x700071d060> as *const (), 2), // +0x379790 sub_379790
    Lifted::new(0x7000a657d0, tpl_614::<0x7000780530> as *const (), 0), // +0x3797d0 sub_3797d0
    Lifted::new(0x7000a657f0, tpl_615::<0x8, 0x70006f1e30> as *const (), 2), // +0x3797f0 sub_3797f0
    Lifted::new(0x7000a65830, fn_7000a65830 as *const (), 0), // +0x379830 sub_379830
    Lifted::new(0x7000a65860, fn_7000a65860 as *const (), 2), // +0x379860 sub_379860
    Lifted::new(0x7000a658b0, fn_7000a658b0 as *const (), 0), // +0x3798b0 sub_3798b0
    Lifted::new(0x7000a658e0, fn_7000a658e0 as *const (), 2), // +0x3798e0 sub_3798e0
    Lifted::new(0x7000a65930, tpl_616::<0x70017d94ec, 0x700074b810> as *const (), 0), // +0x379930 sub_379930
    Lifted::new(0x7000a65960, tpl_617::<0x8, 0x70017d94ec, 0x700071d060> as *const (), 2), // +0x379960 sub_379960
    Lifted::new(0x7000a659b0, tpl_616::<0x70017d94ec, 0x7000780530> as *const (), 0), // +0x3799b0 sub_3799b0
    Lifted::new(0x7000a659e0, tpl_617::<0x8, 0x70017d94ec, 0x70006f1e30> as *const (), 2), // +0x3799e0 sub_3799e0
    Lifted::new(0x7000a65a30, tpl_616::<0x70017d94e8, 0x700074b810> as *const (), 0), // +0x379a30 sub_379a30
    Lifted::new(0x7000a65a60, tpl_617::<0x8, 0x70017d94e8, 0x700071d060> as *const (), 2), // +0x379a60 sub_379a60
    Lifted::new(0x7000a65ab0, tpl_616::<0x70017d94e8, 0x7000780530> as *const (), 0), // +0x379ab0 sub_379ab0
    Lifted::new(0x7000a65ae0, tpl_617::<0x8, 0x70017d94e8, 0x70006f1e30> as *const (), 2), // +0x379ae0 sub_379ae0
    Lifted::new(0x7000a65b30, tpl_616::<0x70017d94e4, 0x700074b810> as *const (), 0), // +0x379b30 sub_379b30
    Lifted::new(0x7000a65b60, tpl_617::<0x8, 0x70017d94e4, 0x700071d060> as *const (), 2), // +0x379b60 sub_379b60
    Lifted::new(0x7000a65bb0, tpl_616::<0x70017d94e4, 0x7000780530> as *const (), 0), // +0x379bb0 sub_379bb0
    Lifted::new(0x7000a65be0, tpl_617::<0x8, 0x70017d94e4, 0x70006f1e30> as *const (), 2), // +0x379be0 sub_379be0
    Lifted::new(0x7000a65c30, fn_7000a65c30 as *const (), 1), // +0x379c30 sub_379c30
    Lifted::new(0x7000a65c50, tpl_607::<0x8, 0x10> as *const (), 0), // +0x379c50 sub_379c50
    Lifted::new(0x7000a65c60, tpl_618::<0x8> as *const (), 0), // +0x379c60 sub_379c60
    Lifted::new(0x7000a65c70, fn_7000a65c70 as *const (), 1), // +0x379c70 sub_379c70
    Lifted::new(0x7000a65c80, tpl_619::<0x7000755920> as *const (), 0), // +0x379c80 sub_379c80
    Lifted::new(0x7000a65cb0, tpl_620::<0x8, 0x7000725050> as *const (), 2), // +0x379cb0 sub_379cb0
    Lifted::new(0x7000a65d00, tpl_619::<0x70007049c0> as *const (), 0), // +0x379d00 sub_379d00
    Lifted::new(0x7000a65d30, tpl_620::<0x8, 0x700070b220> as *const (), 2), // +0x379d30 sub_379d30
    Lifted::new(0x7000a65d80, tpl_621::<0x8, 0x7000753d70> as *const (), 2), // +0x379d80 sub_379d80
    Lifted::new(0x7000a65dc0, fn_7000a65dc0 as *const (), 2), // +0x379dc0 sub_379dc0
    Lifted::new(0x7000a65df0, fn_7000a65df0 as *const (), 0), // +0x379df0 sub_379df0
    Lifted::new(0x7000a65e00, tpl_622::<0x700077aac0> as *const (), 0), // +0x379e00 sub_379e00
    Lifted::new(0x7000a65e10, fn_7000a65e10 as *const (), 2), // +0x379e10 sub_379e10
    Lifted::new(0x7000a65e50, tpl_623::<0x7000722390> as *const (), 0), // +0x379e50 sub_379e50
    Lifted::new(0x7000a65e60, tpl_624::<0x8, 0x7000792f70> as *const (), 2), // +0x379e60 sub_379e60
    Lifted::new(0x7000a65ea0, tpl_623::<0x70007a19c0> as *const (), 0), // +0x379ea0 sub_379ea0
    Lifted::new(0x7000a65eb0, tpl_624::<0x8, 0x700078a870> as *const (), 2), // +0x379eb0 sub_379eb0
    Lifted::new(0x7000a65ef0, tpl_625::<0x7000722390> as *const (), 0), // +0x379ef0 sub_379ef0
    Lifted::new(0x7000a65f00, tpl_626::<0x8, 0x7000792f70> as *const (), 2), // +0x379f00 sub_379f00
    Lifted::new(0x7000a65f40, tpl_625::<0x70007a19c0> as *const (), 0), // +0x379f40 sub_379f40
    Lifted::new(0x7000a65f50, tpl_626::<0x8, 0x700078a870> as *const (), 2), // +0x379f50 sub_379f50
    Lifted::new(0x7000a65f90, tpl_627::<0x7000722390> as *const (), 0), // +0x379f90 sub_379f90
    Lifted::new(0x7000a65fa0, tpl_628::<0x8, 0x7000792f70> as *const (), 2), // +0x379fa0 sub_379fa0
    Lifted::new(0x7000a65fe0, tpl_627::<0x70007a19c0> as *const (), 0), // +0x379fe0 sub_379fe0
    Lifted::new(0x7000a65ff0, tpl_628::<0x8, 0x700078a870> as *const (), 2), // +0x379ff0 sub_379ff0
    Lifted::new(0x7000a66030, tpl_629::<0x7000722390> as *const (), 0), // +0x37a030 sub_37a030
    Lifted::new(0x7000a66040, tpl_630::<0x8, 0x7000792f70> as *const (), 2), // +0x37a040 sub_37a040
    Lifted::new(0x7000a66080, tpl_629::<0x70007a19c0> as *const (), 0), // +0x37a080 sub_37a080
    Lifted::new(0x7000a66090, tpl_630::<0x8, 0x700078a870> as *const (), 2), // +0x37a090 sub_37a090
    Lifted::new(0x7000a660d0, tpl_631::<0x7000722390> as *const (), 0), // +0x37a0d0 sub_37a0d0
    Lifted::new(0x7000a660e0, tpl_632::<0x8, 0x7000792f70> as *const (), 2), // +0x37a0e0 sub_37a0e0
    Lifted::new(0x7000a66120, tpl_631::<0x70007a19c0> as *const (), 0), // +0x37a120 sub_37a120
    Lifted::new(0x7000a66130, tpl_632::<0x8, 0x700078a870> as *const (), 2), // +0x37a130 sub_37a130
    Lifted::new(0x7000a66170, tpl_633::<0x7000722390> as *const (), 0), // +0x37a170 sub_37a170
    Lifted::new(0x7000a66180, tpl_634::<0x8, 0x7000792f70> as *const (), 2), // +0x37a180 sub_37a180
    Lifted::new(0x7000a661c0, tpl_633::<0x70007a19c0> as *const (), 0), // +0x37a1c0 sub_37a1c0
    Lifted::new(0x7000a661d0, tpl_634::<0x8, 0x700078a870> as *const (), 2), // +0x37a1d0 sub_37a1d0
    Lifted::new(0x7000a66210, tpl_635::<0x7000722390> as *const (), 0), // +0x37a210 sub_37a210
    Lifted::new(0x7000a66220, tpl_636::<0x8, 0x7000792f70> as *const (), 2), // +0x37a220 sub_37a220
    Lifted::new(0x7000a66260, tpl_635::<0x70007a19c0> as *const (), 0), // +0x37a260 sub_37a260
    Lifted::new(0x7000a66270, tpl_636::<0x8, 0x700078a870> as *const (), 2), // +0x37a270 sub_37a270
    Lifted::new(0x7000a662b0, tpl_621::<0x8, 0x7000755020> as *const (), 2), // +0x37a2b0 sub_37a2b0
    Lifted::new(0x7000a662f0, fn_7000a662f0 as *const (), 2), // +0x37a2f0 sub_37a2f0
    Lifted::new(0x7000a66320, fn_7000a66320 as *const (), 0), // +0x37a320 sub_37a320
    Lifted::new(0x7000a66330, tpl_622::<0x7000782ef0> as *const (), 0), // +0x37a330 sub_37a330
    Lifted::new(0x7000a66340, fn_7000a66340 as *const (), 2), // +0x37a340 sub_37a340
    Lifted::new(0x7000a66380, tpl_637::<0x8, 0x28> as *const (), 0), // +0x37a380 sub_37a380
    Lifted::new(0x7000a663b0, fn_7000a663b0 as *const (), 0), // +0x37a3b0 sub_37a3b0
    Lifted::new(0x7000a663d0, fn_7000a663d0 as *const (), 2), // +0x37a3d0 sub_37a3d0
    Lifted::new(0x7000a66410, fn_7000a66410 as *const (), 0), // +0x37a410 sub_37a410
    Lifted::new(0x7000a66440, fn_7000a66440 as *const (), 2), // +0x37a440 sub_37a440
    Lifted::new(0x7000a66480, fn_7000a66480 as *const (), 0), // +0x37a480 sub_37a480
    Lifted::new(0x7000a664b0, fn_7000a664b0 as *const (), 2), // +0x37a4b0 sub_37a4b0
    Lifted::new(0x7000a664f0, fn_7000a664f0 as *const (), 0), // +0x37a4f0 sub_37a4f0
    Lifted::new(0x7000a66510, fn_7000a66510 as *const (), 2), // +0x37a510 sub_37a510
    Lifted::new(0x7000a66550, fn_7000a66550 as *const (), 0), // +0x37a550 sub_37a550
    Lifted::new(0x7000a66580, fn_7000a66580 as *const (), 2), // +0x37a580 sub_37a580
    Lifted::new(0x7000a665c0, fn_7000a665c0 as *const (), 0), // +0x37a5c0 sub_37a5c0
    Lifted::new(0x7000a665f0, fn_7000a665f0 as *const (), 2), // +0x37a5f0 sub_37a5f0
    Lifted::new(0x7000a66630, tpl_20 as *const (), 1), // +0x37a630 sub_37a630
    Lifted::new(0x7000a66640, fn_7000a66640 as *const (), 0), // +0x37a640 sub_37a640
    Lifted::new(0x7000a66690, fn_7000a66690 as *const (), 0), // +0x37a690 sub_37a690
    Lifted::new(0x7000a666c0, fn_7000a666c0 as *const (), 2), // +0x37a6c0 sub_37a6c0
    Lifted::new(0x7000a66700, fn_7000a66700 as *const (), 0), // +0x37a700 sub_37a700
    Lifted::new(0x7000a66720, fn_7000a66720 as *const (), 2), // +0x37a720 sub_37a720
    Lifted::new(0x7000a66760, fn_7000a66760 as *const (), 0), // +0x37a760 sub_37a760
    Lifted::new(0x7000a66790, fn_7000a66790 as *const (), 2), // +0x37a790 sub_37a790
    Lifted::new(0x7000a667d0, fn_7000a667d0 as *const (), 0), // +0x37a7d0 sub_37a7d0
    Lifted::new(0x7000a66800, fn_7000a66800 as *const (), 2), // +0x37a800 sub_37a800
    Lifted::new(0x7000a66840, fn_7000a66840 as *const (), 0), // +0x37a840 sub_37a840
    Lifted::new(0x7000a66860, fn_7000a66860 as *const (), 1), // +0x37a860 sub_37a860
    Lifted::new(0x7000a66870, fn_7000a66870 as *const (), 2), // +0x37a870 sub_37a870
    Lifted::new(0x7000a668b0, fn_7000a668b0 as *const (), 2), // +0x37a8b0 sub_37a8b0
    Lifted::new(0x7000a668f0, fn_7000a668f0 as *const (), 0), // +0x37a8f0 sub_37a8f0
    Lifted::new(0x7000a66920, fn_7000a66920 as *const (), 0), // +0x37a920 sub_37a920
    Lifted::new(0x7000a66950, fn_7000a66950 as *const (), 2), // +0x37a950 sub_37a950
    Lifted::new(0x7000a66990, fn_7000a66990 as *const (), 2), // +0x37a990 sub_37a990
    Lifted::new(0x7000a669d0, fn_7000a669d0 as *const (), 2), // +0x37a9d0 sub_37a9d0
    Lifted::new(0x7000a66a10, fn_7000a66a10 as *const (), 0), // +0x37aa10 sub_37aa10
    Lifted::new(0x7000a66a40, fn_7000a66a40 as *const (), 0), // +0x37aa40 sub_37aa40
    Lifted::new(0x7000a66a70, fn_7000a66a70 as *const (), 2), // +0x37aa70 sub_37aa70
    Lifted::new(0x7000a66ab0, fn_7000a66ab0 as *const (), 2), // +0x37aab0 sub_37aab0
    Lifted::new(0x7000a66af0, fn_7000a66af0 as *const (), 2), // +0x37aaf0 sub_37aaf0
    Lifted::new(0x7000a66b30, fn_7000a66b30 as *const (), 0), // +0x37ab30 sub_37ab30
    Lifted::new(0x7000a66b60, fn_7000a66b60 as *const (), 0), // +0x37ab60 sub_37ab60
    Lifted::new(0x7000a66b90, fn_7000a66b90 as *const (), 2), // +0x37ab90 sub_37ab90
    Lifted::new(0x7000a66bd0, tpl_638::<0x20> as *const (), 1), // +0x37abd0 sub_37abd0
    Lifted::new(0x7000a66bf0, tpl_638::<0x20> as *const (), 1), // +0x37abf0 sub_37abf0
    Lifted::new(0x7000a66c10, tpl_639::<0x8, 0x700079b8f0> as *const (), 2), // +0x37ac10 sub_37ac10
    Lifted::new(0x7000a66c50, fn_7000a66c50 as *const (), 2), // +0x37ac50 sub_37ac50
    Lifted::new(0x7000a66c90, fn_7000a66c90 as *const (), 2), // +0x37ac90 sub_37ac90
    Lifted::new(0x7000a66cd0, fn_7000a66cd0 as *const (), 0), // +0x37acd0 sub_37acd0
    Lifted::new(0x7000a66d00, fn_7000a66d00 as *const (), 0), // +0x37ad00 sub_37ad00
    Lifted::new(0x7000a66d30, fn_7000a66d30 as *const (), 2), // +0x37ad30 sub_37ad30
    Lifted::new(0x7000a66d70, fn_7000a66d70 as *const (), 1), // +0x37ad70 sub_37ad70
    Lifted::new(0x7000a66d90, fn_7000a66d90 as *const (), 1), // +0x37ad90 sub_37ad90
    Lifted::new(0x7000a66da0, fn_7000a66da0 as *const (), 0), // +0x37ada0 sub_37ada0
    Lifted::new(0x7000a66dd0, fn_7000a66dd0 as *const (), 2), // +0x37add0 sub_37add0
    Lifted::new(0x7000a66e10, fn_7000a66e10 as *const (), 2), // +0x37ae10 sub_37ae10
    Lifted::new(0x7000a66e50, fn_7000a66e50 as *const (), 0), // +0x37ae50 sub_37ae50
    Lifted::new(0x7000a66e80, fn_7000a66e80 as *const (), 0), // +0x37ae80 sub_37ae80
    Lifted::new(0x7000a66eb0, fn_7000a66eb0 as *const (), 2), // +0x37aeb0 sub_37aeb0
    Lifted::new(0x7000a66ef0, tpl_640::<0x8, 0x700074a4a0> as *const (), 2), // +0x37aef0 sub_37aef0
    Lifted::new(0x7000a66f30, fn_7000a66f30 as *const (), 1), // +0x37af30 sub_37af30
    Lifted::new(0x7000a66f40, fn_7000a66f40 as *const (), 1), // +0x37af40 sub_37af40
    Lifted::new(0x7000a66f50, fn_7000a66f50 as *const (), 2), // +0x37af50 sub_37af50
    Lifted::new(0x7000a66f90, fn_7000a66f90 as *const (), 2), // +0x37af90 sub_37af90
    Lifted::new(0x7000a66fd0, fn_7000a66fd0 as *const (), 0), // +0x37afd0 sub_37afd0
    Lifted::new(0x7000a67020, fn_7000a67020 as *const (), 0), // +0x37b020 sub_37b020
    Lifted::new(0x7000a67050, fn_7000a67050 as *const (), 2), // +0x37b050 sub_37b050
    Lifted::new(0x7000a67090, tpl_36::<0x100> as *const (), 1), // +0x37b090 sub_37b090
    Lifted::new(0x7000a670a0, fn_7000a670a0 as *const (), 2), // +0x37b0a0 sub_37b0a0
    Lifted::new(0x7000a670f0, fn_7000a670f0 as *const (), 0), // +0x37b0f0 sub_37b0f0
    Lifted::new(0x7000a67120, tpl_47::<0x1c8> as *const (), 1), // +0x37b120 sub_37b120
    Lifted::new(0x7000a67130, tpl_379::<0x20, 0x10> as *const (), 1), // +0x37b130 sub_37b130
    Lifted::new(0x7000a67140, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a67151> as *const (), 0), // +0x37b140 sub_37b140
    Lifted::new(0x7000a67160, tpl_36::<0x1b0> as *const (), 1), // +0x37b160 sub_37b160
    Lifted::new(0x7000a67170, tpl_36::<0x110> as *const (), 1), // +0x37b170 sub_37b170
    Lifted::new(0x7000a67180, fn_7000a67180 as *const (), 1), // +0x37b180 sub_37b180
    Lifted::new(0x7000a67190, tpl_641::<0x10> as *const (), 1), // +0x37b190 sub_37b190
    Lifted::new(0x7000a671a0, fn_7000a671a0 as *const (), 0), // +0x37b1a0 sub_37b1a0
    Lifted::new(0x7000a671d0, fn_7000a671d0 as *const (), 2), // +0x37b1d0 sub_37b1d0
    Lifted::new(0x7000a67210, fn_7000a67210 as *const (), 0), // +0x37b210 sub_37b210
    Lifted::new(0x7000a67230, fn_7000a67230 as *const (), 2), // +0x37b230 sub_37b230
    Lifted::new(0x7000a67270, fn_7000a67270 as *const (), 0), // +0x37b270 sub_37b270
    Lifted::new(0x7000a672a0, fn_7000a672a0 as *const (), 2), // +0x37b2a0 sub_37b2a0
    Lifted::new(0x7000a672e0, fn_7000a672e0 as *const (), 0), // +0x37b2e0 sub_37b2e0
    Lifted::new(0x7000a67310, fn_7000a67310 as *const (), 2), // +0x37b310 sub_37b310
    Lifted::new(0x7000a67350, fn_7000a67350 as *const (), 0), // +0x37b350 sub_37b350
    Lifted::new(0x7000a67380, fn_7000a67380 as *const (), 2), // +0x37b380 sub_37b380
    Lifted::new(0x7000a673c0, fn_7000a673c0 as *const (), 0), // +0x37b3c0 sub_37b3c0
    Lifted::new(0x7000a673e0, fn_7000a673e0 as *const (), 2), // +0x37b3e0 sub_37b3e0
    Lifted::new(0x7000a67420, fn_7000a67420 as *const (), 0), // +0x37b420 sub_37b420
    Lifted::new(0x7000a67450, fn_7000a67450 as *const (), 2), // +0x37b450 sub_37b450
    Lifted::new(0x7000a67490, fn_7000a67490 as *const (), 0), // +0x37b490 sub_37b490
    Lifted::new(0x7000a674c0, fn_7000a674c0 as *const (), 2), // +0x37b4c0 sub_37b4c0
    Lifted::new(0x7000a67500, fn_7000a67500 as *const (), 0), // +0x37b500 sub_37b500
    Lifted::new(0x7000a67520, fn_7000a67520 as *const (), 0), // +0x37b520 sub_37b520
    Lifted::new(0x7000a67540, tpl_642::<0x7000724960> as *const (), 2), // +0x37b540 sub_37b540
    Lifted::new(0x7000a67580, tpl_642::<0x7000724960> as *const (), 2), // +0x37b580 sub_37b580
    Lifted::new(0x7000a675c0, fn_7000a675c0 as *const (), 0), // +0x37b5c0 sub_37b5c0
    Lifted::new(0x7000a675f0, fn_7000a675f0 as *const (), 2), // +0x37b5f0 sub_37b5f0
    Lifted::new(0x7000a67630, fn_7000a67630 as *const (), 0), // +0x37b630 sub_37b630
    Lifted::new(0x7000a67650, fn_7000a67650 as *const (), 2), // +0x37b650 sub_37b650
    Lifted::new(0x7000a67690, fn_7000a67690 as *const (), 0), // +0x37b690 sub_37b690
    Lifted::new(0x7000a676c0, fn_7000a676c0 as *const (), 2), // +0x37b6c0 sub_37b6c0
    Lifted::new(0x7000a67700, fn_7000a67700 as *const (), 0), // +0x37b700 sub_37b700
    Lifted::new(0x7000a67730, fn_7000a67730 as *const (), 2), // +0x37b730 sub_37b730
    Lifted::new(0x7000a67770, tpl_643::<0x7000755920> as *const (), 0), // +0x37b770 sub_37b770
    Lifted::new(0x7000a677a0, tpl_644::<0x8, 0x7000725050> as *const (), 2), // +0x37b7a0 sub_37b7a0
    Lifted::new(0x7000a677f0, tpl_643::<0x70007049c0> as *const (), 0), // +0x37b7f0 sub_37b7f0
    Lifted::new(0x7000a67820, tpl_644::<0x8, 0x700070b220> as *const (), 2), // +0x37b820 sub_37b820
    Lifted::new(0x7000a67870, tpl_52::<0xe8> as *const (), 1), // +0x37b870 sub_37b870
    Lifted::new(0x7000a67880, fn_7000a67880 as *const (), 0), // +0x37b880 sub_37b880
    Lifted::new(0x7000a678b0, fn_7000a678b0 as *const (), 2), // +0x37b8b0 sub_37b8b0
    Lifted::new(0x7000a678f0, fn_7000a678f0 as *const (), 0), // +0x37b8f0 sub_37b8f0
    Lifted::new(0x7000a67910, fn_7000a67910 as *const (), 2), // +0x37b910 sub_37b910
    Lifted::new(0x7000a67950, fn_7000a67950 as *const (), 0), // +0x37b950 sub_37b950
    Lifted::new(0x7000a67b60, fn_7000a67b60 as *const (), 0), // +0x37bb60 sub_37bb60
    Lifted::new(0x7000a67b90, fn_7000a67b90 as *const (), 2), // +0x37bb90 sub_37bb90
    Lifted::new(0x7000a67be0, fn_7000a67be0 as *const (), 0), // +0x37bbe0 sub_37bbe0
    Lifted::new(0x7000a67c10, fn_7000a67c10 as *const (), 2), // +0x37bc10 sub_37bc10
    Lifted::new(0x7000a67c60, fn_7000a67c60 as *const (), 0), // +0x37bc60 sub_37bc60
    Lifted::new(0x7000a67c90, fn_7000a67c90 as *const (), 2), // +0x37bc90 sub_37bc90
    Lifted::new(0x7000a67ce0, fn_7000a67ce0 as *const (), 0), // +0x37bce0 sub_37bce0
    Lifted::new(0x7000a67d10, fn_7000a67d10 as *const (), 2), // +0x37bd10 sub_37bd10
    Lifted::new(0x7000a67d50, fn_7000a67d50 as *const (), 0), // +0x37bd50 sub_37bd50
    Lifted::new(0x7000a67d80, fn_7000a67d80 as *const (), 2), // +0x37bd80 sub_37bd80
    Lifted::new(0x7000a67dc0, fn_7000a67dc0 as *const (), 0), // +0x37bdc0 sub_37bdc0
    Lifted::new(0x7000a67df0, fn_7000a67df0 as *const (), 2), // +0x37bdf0 sub_37bdf0
    Lifted::new(0x7000a67e40, fn_7000a67e40 as *const (), 0), // +0x37be40 sub_37be40
    Lifted::new(0x7000a67e70, fn_7000a67e70 as *const (), 2), // +0x37be70 sub_37be70
    Lifted::new(0x7000a67eb0, fn_7000a67eb0 as *const (), 0), // +0x37beb0 sub_37beb0
    Lifted::new(0x7000a67ee0, fn_7000a67ee0 as *const (), 2), // +0x37bee0 sub_37bee0
    Lifted::new(0x7000a67f20, fn_7000a67f20 as *const (), 0), // +0x37bf20 sub_37bf20
    Lifted::new(0x7000a67f50, fn_7000a67f50 as *const (), 2), // +0x37bf50 sub_37bf50
    Lifted::new(0x7000a67f90, fn_7000a67f90 as *const (), 0), // +0x37bf90 sub_37bf90
    Lifted::new(0x7000a67fc0, fn_7000a67fc0 as *const (), 2), // +0x37bfc0 sub_37bfc0
    Lifted::new(0x7000a68000, fn_7000a68000 as *const (), 0), // +0x37c000 sub_37c000
    Lifted::new(0x7000a68030, fn_7000a68030 as *const (), 2), // +0x37c030 sub_37c030
    Lifted::new(0x7000a68070, fn_7000a68070 as *const (), 0), // +0x37c070 sub_37c070
    Lifted::new(0x7000a680a0, fn_7000a680a0 as *const (), 2), // +0x37c0a0 sub_37c0a0
    Lifted::new(0x7000a680e0, fn_7000a680e0 as *const (), 0), // +0x37c0e0 sub_37c0e0
    Lifted::new(0x7000a68110, fn_7000a68110 as *const (), 2), // +0x37c110 sub_37c110
    Lifted::new(0x7000a68150, fn_7000a68150 as *const (), 0), // +0x37c150 sub_37c150
    Lifted::new(0x7000a68180, fn_7000a68180 as *const (), 2), // +0x37c180 sub_37c180
    Lifted::new(0x7000a681c0, fn_7000a681c0 as *const (), 0), // +0x37c1c0 sub_37c1c0
    Lifted::new(0x7000a681f0, fn_7000a681f0 as *const (), 2), // +0x37c1f0 sub_37c1f0
    Lifted::new(0x7000a68230, fn_7000a68230 as *const (), 0), // +0x37c230 sub_37c230
    Lifted::new(0x7000a68260, fn_7000a68260 as *const (), 2), // +0x37c260 sub_37c260
    Lifted::new(0x7000a682a0, fn_7000a682a0 as *const (), 0), // +0x37c2a0 sub_37c2a0
    Lifted::new(0x7000a682d0, fn_7000a682d0 as *const (), 2), // +0x37c2d0 sub_37c2d0
    Lifted::new(0x7000a68310, fn_7000a68310 as *const (), 0), // +0x37c310 sub_37c310
    Lifted::new(0x7000a68340, fn_7000a68340 as *const (), 2), // +0x37c340 sub_37c340
    Lifted::new(0x7000a68380, fn_7000a68380 as *const (), 0), // +0x37c380 sub_37c380
    Lifted::new(0x7000a683b0, fn_7000a683b0 as *const (), 2), // +0x37c3b0 sub_37c3b0
    Lifted::new(0x7000a683f0, fn_7000a683f0 as *const (), 1), // +0x37c3f0 sub_37c3f0
    Lifted::new(0x7000a68460, fn_7000a68460 as *const (), 0), // +0x37c460 sub_37c460
    Lifted::new(0x7000a68490, fn_7000a68490 as *const (), 2), // +0x37c490 sub_37c490
    Lifted::new(0x7000a684e0, tpl_379::<0x20, 0x10> as *const (), 1), // +0x37c4e0 sub_37c4e0
    Lifted::new(0x7000a684f0, tpl_379::<0x20, 0x10> as *const (), 1), // +0x37c4f0 sub_37c4f0
    Lifted::new(0x7000a68500, fn_7000a68500 as *const (), 0), // +0x37c500 sub_37c500
    Lifted::new(0x7000a68530, fn_7000a68530 as *const (), 2), // +0x37c530 sub_37c530
    Lifted::new(0x7000a68570, fn_7000a68570 as *const (), 0), // +0x37c570 sub_37c570
    Lifted::new(0x7000a685a0, fn_7000a685a0 as *const (), 2), // +0x37c5a0 sub_37c5a0
    Lifted::new(0x7000a685e0, fn_7000a685e0 as *const (), 0), // +0x37c5e0 sub_37c5e0
    Lifted::new(0x7000a68610, fn_7000a68610 as *const (), 2), // +0x37c610 sub_37c610
    Lifted::new(0x7000a68660, fn_7000a68660 as *const (), 0), // +0x37c660 sub_37c660
    Lifted::new(0x7000a68690, fn_7000a68690 as *const (), 2), // +0x37c690 sub_37c690
    Lifted::new(0x7000a686d0, fn_7000a686d0 as *const (), 0), // +0x37c6d0 sub_37c6d0
    Lifted::new(0x7000a68700, fn_7000a68700 as *const (), 2), // +0x37c700 sub_37c700
    Lifted::new(0x7000a68740, fn_7000a68740 as *const (), 0), // +0x37c740 sub_37c740
    Lifted::new(0x7000a68770, fn_7000a68770 as *const (), 2), // +0x37c770 sub_37c770
    Lifted::new(0x7000a687b0, fn_7000a687b0 as *const (), 0), // +0x37c7b0 sub_37c7b0
    Lifted::new(0x7000a687e0, fn_7000a687e0 as *const (), 2), // +0x37c7e0 sub_37c7e0
    Lifted::new(0x7000a68830, fn_7000a68830 as *const (), 0), // +0x37c830 sub_37c830
    Lifted::new(0x7000a68860, fn_7000a68860 as *const (), 2), // +0x37c860 sub_37c860
    Lifted::new(0x7000a688a0, tpl_645::<0x7000755920> as *const (), 0), // +0x37c8a0 sub_37c8a0
    Lifted::new(0x7000a688d0, tpl_646::<0x8, 0x7000725050> as *const (), 2), // +0x37c8d0 sub_37c8d0
    Lifted::new(0x7000a68920, tpl_645::<0x70007049c0> as *const (), 0), // +0x37c920 sub_37c920
    Lifted::new(0x7000a68950, tpl_646::<0x8, 0x700070b220> as *const (), 2), // +0x37c950 sub_37c950
    Lifted::new(0x7000a689a0, tpl_647::<0x7000755920> as *const (), 0), // +0x37c9a0 sub_37c9a0
    Lifted::new(0x7000a689d0, tpl_648::<0x8, 0x7000725050> as *const (), 2), // +0x37c9d0 sub_37c9d0
    Lifted::new(0x7000a68a20, tpl_647::<0x70007049c0> as *const (), 0), // +0x37ca20 sub_37ca20
    Lifted::new(0x7000a68a50, tpl_648::<0x8, 0x700070b220> as *const (), 2), // +0x37ca50 sub_37ca50
    Lifted::new(0x7000a68aa0, fn_7000a68aa0 as *const (), 2), // +0x37caa0 sub_37caa0
    Lifted::new(0x7000a68af0, fn_7000a68af0 as *const (), 0), // +0x37caf0 sub_37caf0
    Lifted::new(0x7000a68b20, fn_7000a68b20 as *const (), 2), // +0x37cb20 sub_37cb20
    Lifted::new(0x7000a68b70, fn_7000a68b70 as *const (), 0), // +0x37cb70 sub_37cb70
    Lifted::new(0x7000a68ba0, fn_7000a68ba0 as *const (), 2), // +0x37cba0 sub_37cba0
    Lifted::new(0x7000a68bf0, fn_7000a68bf0 as *const (), 0), // +0x37cbf0 sub_37cbf0
    Lifted::new(0x7000a68c20, fn_7000a68c20 as *const (), 0), // +0x37cc20 sub_37cc20
    Lifted::new(0x7000a68c50, fn_7000a68c50 as *const (), 2), // +0x37cc50 sub_37cc50
    Lifted::new(0x7000a68ca0, fn_7000a68ca0 as *const (), 0), // +0x37cca0 sub_37cca0
    Lifted::new(0x7000a68cd0, fn_7000a68cd0 as *const (), 2), // +0x37ccd0 sub_37ccd0
    Lifted::new(0x7000a68d20, fn_7000a68d20 as *const (), 0), // +0x37cd20 sub_37cd20
    Lifted::new(0x7000a68d50, fn_7000a68d50 as *const (), 2), // +0x37cd50 sub_37cd50
    Lifted::new(0x7000a68da0, fn_7000a68da0 as *const (), 0), // +0x37cda0 sub_37cda0
    Lifted::new(0x7000a68dd0, fn_7000a68dd0 as *const (), 2), // +0x37cdd0 sub_37cdd0
    Lifted::new(0x7000a68e10, fn_7000a68e10 as *const (), 0), // +0x37ce10 sub_37ce10
    Lifted::new(0x7000a68e40, fn_7000a68e40 as *const (), 2), // +0x37ce40 sub_37ce40
    Lifted::new(0x7000a68e90, fn_7000a68e90 as *const (), 0), // +0x37ce90 sub_37ce90
    Lifted::new(0x7000a68ec0, fn_7000a68ec0 as *const (), 2), // +0x37cec0 sub_37cec0
    Lifted::new(0x7000a68f10, fn_7000a68f10 as *const (), 0), // +0x37cf10 sub_37cf10
    Lifted::new(0x7000a68f40, fn_7000a68f40 as *const (), 2), // +0x37cf40 sub_37cf40
    Lifted::new(0x7000a68f80, fn_7000a68f80 as *const (), 0), // +0x37cf80 sub_37cf80
    Lifted::new(0x7000a68fb0, fn_7000a68fb0 as *const (), 2), // +0x37cfb0 sub_37cfb0
    Lifted::new(0x7000a68ff0, fn_7000a68ff0 as *const (), 0), // +0x37cff0 sub_37cff0
    Lifted::new(0x7000a69020, fn_7000a69020 as *const (), 2), // +0x37d020 sub_37d020
    Lifted::new(0x7000a69070, fn_7000a69070 as *const (), 0), // +0x37d070 sub_37d070
    Lifted::new(0x7000a690a0, fn_7000a690a0 as *const (), 2), // +0x37d0a0 sub_37d0a0
    Lifted::new(0x7000a690f0, fn_7000a690f0 as *const (), 0), // +0x37d0f0 sub_37d0f0
    Lifted::new(0x7000a69120, fn_7000a69120 as *const (), 2), // +0x37d120 sub_37d120
    Lifted::new(0x7000a69170, fn_7000a69170 as *const (), 0), // +0x37d170 sub_37d170
    Lifted::new(0x7000a691a0, fn_7000a691a0 as *const (), 2), // +0x37d1a0 sub_37d1a0
    Lifted::new(0x7000a691f0, fn_7000a691f0 as *const (), 0), // +0x37d1f0 sub_37d1f0
    Lifted::new(0x7000a69220, fn_7000a69220 as *const (), 2), // +0x37d220 sub_37d220
    Lifted::new(0x7000a69260, fn_7000a69260 as *const (), 0), // +0x37d260 sub_37d260
    Lifted::new(0x7000a69290, fn_7000a69290 as *const (), 2), // +0x37d290 sub_37d290
    Lifted::new(0x7000a692d0, fn_7000a692d0 as *const (), 0), // +0x37d2d0 sub_37d2d0
    Lifted::new(0x7000a69300, fn_7000a69300 as *const (), 2), // +0x37d300 sub_37d300
    Lifted::new(0x7000a69350, fn_7000a69350 as *const (), 0), // +0x37d350 sub_37d350
    Lifted::new(0x7000a69380, fn_7000a69380 as *const (), 2), // +0x37d380 sub_37d380
    Lifted::new(0x7000a693d0, fn_7000a693d0 as *const (), 0), // +0x37d3d0 sub_37d3d0
    Lifted::new(0x7000a69400, fn_7000a69400 as *const (), 2), // +0x37d400 sub_37d400
    Lifted::new(0x7000a69450, fn_7000a69450 as *const (), 0), // +0x37d450 sub_37d450
    Lifted::new(0x7000a69480, fn_7000a69480 as *const (), 2), // +0x37d480 sub_37d480
    Lifted::new(0x7000a694d0, fn_7000a694d0 as *const (), 0), // +0x37d4d0 sub_37d4d0
    Lifted::new(0x7000a69500, fn_7000a69500 as *const (), 2), // +0x37d500 sub_37d500
    Lifted::new(0x7000a69550, fn_7000a69550 as *const (), 0), // +0x37d550 sub_37d550
    Lifted::new(0x7000a69580, fn_7000a69580 as *const (), 2), // +0x37d580 sub_37d580
    Lifted::new(0x7000a695d0, fn_7000a695d0 as *const (), 0), // +0x37d5d0 sub_37d5d0
    Lifted::new(0x7000a69600, fn_7000a69600 as *const (), 2), // +0x37d600 sub_37d600
    Lifted::new(0x7000a69650, fn_7000a69650 as *const (), 0), // +0x37d650 sub_37d650
    Lifted::new(0x7000a69680, fn_7000a69680 as *const (), 2), // +0x37d680 sub_37d680
    Lifted::new(0x7000a696d0, fn_7000a696d0 as *const (), 0), // +0x37d6d0 sub_37d6d0
    Lifted::new(0x7000a69700, fn_7000a69700 as *const (), 2), // +0x37d700 sub_37d700
    Lifted::new(0x7000a69750, fn_7000a69750 as *const (), 0), // +0x37d750 sub_37d750
    Lifted::new(0x7000a69780, fn_7000a69780 as *const (), 2), // +0x37d780 sub_37d780
    Lifted::new(0x7000a697d0, fn_7000a697d0 as *const (), 0), // +0x37d7d0 sub_37d7d0
    Lifted::new(0x7000a69800, fn_7000a69800 as *const (), 2), // +0x37d800 sub_37d800
    Lifted::new(0x7000a69840, fn_7000a69840 as *const (), 0), // +0x37d840 sub_37d840
    Lifted::new(0x7000a69870, fn_7000a69870 as *const (), 2), // +0x37d870 sub_37d870
    Lifted::new(0x7000a698b0, fn_7000a698b0 as *const (), 0), // +0x37d8b0 sub_37d8b0
    Lifted::new(0x7000a698e0, fn_7000a698e0 as *const (), 2), // +0x37d8e0 sub_37d8e0
    Lifted::new(0x7000a69930, fn_7000a69930 as *const (), 0), // +0x37d930 sub_37d930
    Lifted::new(0x7000a69960, fn_7000a69960 as *const (), 2), // +0x37d960 sub_37d960
    Lifted::new(0x7000a699a0, fn_7000a699a0 as *const (), 0), // +0x37d9a0 sub_37d9a0
    Lifted::new(0x7000a699d0, fn_7000a699d0 as *const (), 2), // +0x37d9d0 sub_37d9d0
    Lifted::new(0x7000a69a10, fn_7000a69a10 as *const (), 0), // +0x37da10 sub_37da10
    Lifted::new(0x7000a69a40, fn_7000a69a40 as *const (), 2), // +0x37da40 sub_37da40
    Lifted::new(0x7000a69a80, tpl_47::<0x108> as *const (), 1), // +0x37da80 sub_37da80
    Lifted::new(0x7000a69a90, fn_7000a69a90 as *const (), 0), // +0x37da90 sub_37da90
    Lifted::new(0x7000a69ac0, fn_7000a69ac0 as *const (), 2), // +0x37dac0 sub_37dac0
    Lifted::new(0x7000a69b10, fn_7000a69b10 as *const (), 0), // +0x37db10 sub_37db10
    Lifted::new(0x7000a69b40, fn_7000a69b40 as *const (), 2), // +0x37db40 sub_37db40
    Lifted::new(0x7000a69b90, fn_7000a69b90 as *const (), 0), // +0x37db90 sub_37db90
    Lifted::new(0x7000a69bc0, fn_7000a69bc0 as *const (), 2), // +0x37dbc0 sub_37dbc0
    Lifted::new(0x7000a69c10, fn_7000a69c10 as *const (), 0), // +0x37dc10 sub_37dc10
    Lifted::new(0x7000a69c40, fn_7000a69c40 as *const (), 2), // +0x37dc40 sub_37dc40
    Lifted::new(0x7000a69c90, fn_7000a69c90 as *const (), 0), // +0x37dc90 sub_37dc90
    Lifted::new(0x7000a69cc0, fn_7000a69cc0 as *const (), 0), // +0x37dcc0 sub_37dcc0
    Lifted::new(0x7000a69d20, fn_7000a69d20 as *const (), 2), // +0x37dd20 sub_37dd20
    Lifted::new(0x7000a69d60, fn_7000a69d60 as *const (), 0), // +0x37dd60 sub_37dd60
    Lifted::new(0x7000a69d90, fn_7000a69d90 as *const (), 0), // +0x37dd90 sub_37dd90
    Lifted::new(0x7000a69df0, fn_7000a69df0 as *const (), 2), // +0x37ddf0 sub_37ddf0
    Lifted::new(0x7000a69e30, fn_7000a69e30 as *const (), 0), // +0x37de30 sub_37de30
    Lifted::new(0x7000a69e60, fn_7000a69e60 as *const (), 0), // +0x37de60 sub_37de60
    Lifted::new(0x7000a69ec0, fn_7000a69ec0 as *const (), 2), // +0x37dec0 sub_37dec0
    Lifted::new(0x7000a69f00, fn_7000a69f00 as *const (), 0), // +0x37df00 sub_37df00
    Lifted::new(0x7000a69f30, fn_7000a69f30 as *const (), 0), // +0x37df30 sub_37df30
    Lifted::new(0x7000a69f90, fn_7000a69f90 as *const (), 2), // +0x37df90 sub_37df90
    Lifted::new(0x7000a69fd0, fn_7000a69fd0 as *const (), 0), // +0x37dfd0 sub_37dfd0
    Lifted::new(0x7000a6a000, fn_7000a6a000 as *const (), 0), // +0x37e000 sub_37e000
    Lifted::new(0x7000a6a060, fn_7000a6a060 as *const (), 2), // +0x37e060 sub_37e060
    Lifted::new(0x7000a6a0a0, fn_7000a6a0a0 as *const (), 0), // +0x37e0a0 sub_37e0a0
    Lifted::new(0x7000a6a0d0, fn_7000a6a0d0 as *const (), 0), // +0x37e0d0 sub_37e0d0
    Lifted::new(0x7000a6a130, fn_7000a6a130 as *const (), 2), // +0x37e130 sub_37e130
    Lifted::new(0x7000a6a170, fn_7000a6a170 as *const (), 0), // +0x37e170 sub_37e170
    Lifted::new(0x7000a6a1a0, fn_7000a6a1a0 as *const (), 0), // +0x37e1a0 sub_37e1a0
    Lifted::new(0x7000a6a200, fn_7000a6a200 as *const (), 2), // +0x37e200 sub_37e200
    Lifted::new(0x7000a6a240, fn_7000a6a240 as *const (), 0), // +0x37e240 sub_37e240
    Lifted::new(0x7000a6a270, fn_7000a6a270 as *const (), 0), // +0x37e270 sub_37e270
    Lifted::new(0x7000a6a2d0, fn_7000a6a2d0 as *const (), 2), // +0x37e2d0 sub_37e2d0
    Lifted::new(0x7000a6a310, fn_7000a6a310 as *const (), 0), // +0x37e310 sub_37e310
    Lifted::new(0x7000a6a340, fn_7000a6a340 as *const (), 2), // +0x37e340 sub_37e340
    Lifted::new(0x7000a6a380, fn_7000a6a380 as *const (), 0), // +0x37e380 sub_37e380
    Lifted::new(0x7000a6a3b0, fn_7000a6a3b0 as *const (), 2), // +0x37e3b0 sub_37e3b0
    Lifted::new(0x7000a6a3f0, fn_7000a6a3f0 as *const (), 0), // +0x37e3f0 sub_37e3f0
    Lifted::new(0x7000a6a420, fn_7000a6a420 as *const (), 2), // +0x37e420 sub_37e420
    Lifted::new(0x7000a6a460, fn_7000a6a460 as *const (), 0), // +0x37e460 sub_37e460
    Lifted::new(0x7000a6a490, fn_7000a6a490 as *const (), 2), // +0x37e490 sub_37e490
    Lifted::new(0x7000a6a4d0, fn_7000a6a4d0 as *const (), 0), // +0x37e4d0 sub_37e4d0
    Lifted::new(0x7000a6a500, fn_7000a6a500 as *const (), 2), // +0x37e500 sub_37e500
    Lifted::new(0x7000a6a540, fn_7000a6a540 as *const (), 0), // +0x37e540 sub_37e540
    Lifted::new(0x7000a6a570, fn_7000a6a570 as *const (), 2), // +0x37e570 sub_37e570
    Lifted::new(0x7000a6a5b0, fn_7000a6a5b0 as *const (), 0), // +0x37e5b0 sub_37e5b0
    Lifted::new(0x7000a6a5e0, fn_7000a6a5e0 as *const (), 2), // +0x37e5e0 sub_37e5e0
    Lifted::new(0x7000a6a620, fn_7000a6a620 as *const (), 0), // +0x37e620 sub_37e620
    Lifted::new(0x7000a6a650, fn_7000a6a650 as *const (), 2), // +0x37e650 sub_37e650
    Lifted::new(0x7000a6a690, fn_7000a6a690 as *const (), 0), // +0x37e690 sub_37e690
    Lifted::new(0x7000a6a6c0, fn_7000a6a6c0 as *const (), 2), // +0x37e6c0 sub_37e6c0
    Lifted::new(0x7000a6a710, fn_7000a6a710 as *const (), 0), // +0x37e710 sub_37e710
    Lifted::new(0x7000a6a740, fn_7000a6a740 as *const (), 2), // +0x37e740 sub_37e740
    Lifted::new(0x7000a6a780, fn_7000a6a780 as *const (), 0), // +0x37e780 sub_37e780
    Lifted::new(0x7000a6a7b0, fn_7000a6a7b0 as *const (), 2), // +0x37e7b0 sub_37e7b0
    Lifted::new(0x7000a6a7f0, fn_7000a6a7f0 as *const (), 0), // +0x37e7f0 sub_37e7f0
    Lifted::new(0x7000a6a820, fn_7000a6a820 as *const (), 2), // +0x37e820 sub_37e820
    Lifted::new(0x7000a6a860, fn_7000a6a860 as *const (), 0), // +0x37e860 sub_37e860
    Lifted::new(0x7000a6a890, fn_7000a6a890 as *const (), 2), // +0x37e890 sub_37e890
    Lifted::new(0x7000a6a8d0, fn_7000a6a8d0 as *const (), 0), // +0x37e8d0 sub_37e8d0
    Lifted::new(0x7000a6a900, fn_7000a6a900 as *const (), 2), // +0x37e900 sub_37e900
    Lifted::new(0x7000a6a940, fn_7000a6a940 as *const (), 0), // +0x37e940 sub_37e940
    Lifted::new(0x7000a6a970, fn_7000a6a970 as *const (), 2), // +0x37e970 sub_37e970
    Lifted::new(0x7000a6a9b0, fn_7000a6a9b0 as *const (), 0), // +0x37e9b0 sub_37e9b0
    Lifted::new(0x7000a6a9e0, fn_7000a6a9e0 as *const (), 2), // +0x37e9e0 sub_37e9e0
    Lifted::new(0x7000a6aa20, fn_7000a6aa20 as *const (), 0), // +0x37ea20 sub_37ea20
    Lifted::new(0x7000a6aa50, fn_7000a6aa50 as *const (), 2), // +0x37ea50 sub_37ea50
    Lifted::new(0x7000a6aa90, fn_7000a6aa90 as *const (), 0), // +0x37ea90 sub_37ea90
    Lifted::new(0x7000a6aac0, fn_7000a6aac0 as *const (), 2), // +0x37eac0 sub_37eac0
    Lifted::new(0x7000a6ab00, fn_7000a6ab00 as *const (), 2), // +0x37eb00 sub_37eb00
    Lifted::new(0x7000a6ab40, fn_7000a6ab40 as *const (), 0), // +0x37eb40 sub_37eb40
    Lifted::new(0x7000a6ab70, tpl_649::<0x8, 0xfffffffffffffffe, 0x700070e210> as *const (), 2), // +0x37eb70 sub_37eb70
    Lifted::new(0x7000a6abb0, tpl_650::<0xfffffffffffffffe, 0x70007198f0> as *const (), 0), // +0x37ebb0 sub_37ebb0
    Lifted::new(0x7000a6abd0, fn_7000a6abd0 as *const (), 2), // +0x37ebd0 sub_37ebd0
    Lifted::new(0x7000a6ac10, fn_7000a6ac10 as *const (), 0), // +0x37ec10 sub_37ec10
    Lifted::new(0x7000a6ac40, tpl_651::<0x8, 0xfffffffffffffffa, 0x700070e210> as *const (), 2), // +0x37ec40 sub_37ec40
    Lifted::new(0x7000a6ac80, tpl_652::<0xfffffffffffffffa, 0x70007198f0> as *const (), 0), // +0x37ec80 sub_37ec80
    Lifted::new(0x7000a6aca0, fn_7000a6aca0 as *const (), 0), // +0x37eca0 sub_37eca0
    Lifted::new(0x7000a6acc0, fn_7000a6acc0 as *const (), 2), // +0x37ecc0 sub_37ecc0
    Lifted::new(0x7000a6ad00, fn_7000a6ad00 as *const (), 2), // +0x37ed00 sub_37ed00
    Lifted::new(0x7000a6ad40, fn_7000a6ad40 as *const (), 0), // +0x37ed40 sub_37ed40
    Lifted::new(0x7000a6ad70, tpl_649::<0x8, 0xffffffffffffffff, 0x700070e210> as *const (), 2), // +0x37ed70 sub_37ed70
    Lifted::new(0x7000a6adb0, tpl_650::<0xffffffffffffffff, 0x70007198f0> as *const (), 0), // +0x37edb0 sub_37edb0
    Lifted::new(0x7000a6add0, fn_7000a6add0 as *const (), 2), // +0x37edd0 sub_37edd0
    Lifted::new(0x7000a6ae10, fn_7000a6ae10 as *const (), 0), // +0x37ee10 sub_37ee10
    Lifted::new(0x7000a6ae40, tpl_651::<0x8, 0xfffffffffffffffb, 0x700070e210> as *const (), 2), // +0x37ee40 sub_37ee40
    Lifted::new(0x7000a6ae80, tpl_652::<0xfffffffffffffffb, 0x70007198f0> as *const (), 0), // +0x37ee80 sub_37ee80
    Lifted::new(0x7000a6aea0, tpl_155::<0x700077b930> as *const (), 0), // +0x37eea0 sub_37eea0
    Lifted::new(0x7000a6aeb0, tpl_151::<0x700077b930> as *const (), 0), // +0x37eeb0 sub_37eeb0
    Lifted::new(0x7000a6aec0, tpl_52::<0x10> as *const (), 1), // +0x37eec0 sub_37eec0
    Lifted::new(0x7000a6aed0, fn_7000a6aed0 as *const (), 2), // +0x37eed0 sub_37eed0
    Lifted::new(0x7000a6af10, fn_7000a6af10 as *const (), 0), // +0x37ef10 sub_37ef10
    Lifted::new(0x7000a6af30, fn_7000a6af30 as *const (), 2), // +0x37ef30 sub_37ef30
    Lifted::new(0x7000a6af70, fn_7000a6af70 as *const (), 0), // +0x37ef70 sub_37ef70
    Lifted::new(0x7000a6af90, fn_7000a6af90 as *const (), 2), // +0x37ef90 sub_37ef90
    Lifted::new(0x7000a6afd0, fn_7000a6afd0 as *const (), 0), // +0x37efd0 sub_37efd0
    Lifted::new(0x7000a6aff0, fn_7000a6aff0 as *const (), 2), // +0x37eff0 sub_37eff0
    Lifted::new(0x7000a6b030, fn_7000a6b030 as *const (), 0), // +0x37f030 sub_37f030
    Lifted::new(0x7000a6b050, tpl_47::<0x1c0> as *const (), 1), // +0x37f050 sub_37f050
    Lifted::new(0x7000a6b060, fn_7000a6b060 as *const (), 2), // +0x37f060 sub_37f060
    Lifted::new(0x7000a6b0a0, fn_7000a6b0a0 as *const (), 0), // +0x37f0a0 sub_37f0a0
    Lifted::new(0x7000a6b0d0, tpl_649::<0x8, 0xfffffffffffffffd, 0x700070e210> as *const (), 2), // +0x37f0d0 sub_37f0d0
    Lifted::new(0x7000a6b110, tpl_650::<0xfffffffffffffffd, 0x70007198f0> as *const (), 0), // +0x37f110 sub_37f110
    Lifted::new(0x7000a6b130, fn_7000a6b130 as *const (), 2), // +0x37f130 sub_37f130
    Lifted::new(0x7000a6b170, fn_7000a6b170 as *const (), 0), // +0x37f170 sub_37f170
    Lifted::new(0x7000a6b1a0, tpl_651::<0x8, 0xfffffffffffffff9, 0x700070e210> as *const (), 2), // +0x37f1a0 sub_37f1a0
    Lifted::new(0x7000a6b1e0, tpl_652::<0xfffffffffffffff9, 0x70007198f0> as *const (), 0), // +0x37f1e0 sub_37f1e0
    Lifted::new(0x7000a6b200, fn_7000a6b200 as *const (), 0), // +0x37f200 sub_37f200
    Lifted::new(0x7000a6b220, fn_7000a6b220 as *const (), 2), // +0x37f220 sub_37f220
    Lifted::new(0x7000a6b260, fn_7000a6b260 as *const (), 2), // +0x37f260 sub_37f260
    Lifted::new(0x7000a6b2a0, fn_7000a6b2a0 as *const (), 0), // +0x37f2a0 sub_37f2a0
    Lifted::new(0x7000a6b2d0, tpl_649::<0x8, 0x3, 0x700070e210> as *const (), 2), // +0x37f2d0 sub_37f2d0
    Lifted::new(0x7000a6b310, tpl_650::<0x3, 0x70007198f0> as *const (), 0), // +0x37f310 sub_37f310
    Lifted::new(0x7000a6b330, fn_7000a6b330 as *const (), 2), // +0x37f330 sub_37f330
    Lifted::new(0x7000a6b370, fn_7000a6b370 as *const (), 0), // +0x37f370 sub_37f370
    Lifted::new(0x7000a6b3a0, tpl_651::<0x8, 0xffffffffffffffff, 0x700070e210> as *const (), 2), // +0x37f3a0 sub_37f3a0
    Lifted::new(0x7000a6b3e0, tpl_652::<0xffffffffffffffff, 0x70007198f0> as *const (), 0), // +0x37f3e0 sub_37f3e0
    Lifted::new(0x7000a6b400, tpl_653::<0x7000755920> as *const (), 0), // +0x37f400 sub_37f400
    Lifted::new(0x7000a6b430, tpl_654::<0x8, 0x7000725050> as *const (), 2), // +0x37f430 sub_37f430
    Lifted::new(0x7000a6b480, tpl_653::<0x70007049c0> as *const (), 0), // +0x37f480 sub_37f480
    Lifted::new(0x7000a6b4b0, tpl_654::<0x8, 0x700070b220> as *const (), 2), // +0x37f4b0 sub_37f4b0
    Lifted::new(0x7000a6b500, fn_7000a6b500 as *const (), 1), // +0x37f500 sub_37f500
    Lifted::new(0x7000a6b6b0, fn_7000a6b6b0 as *const (), 1), // +0x37f6b0 sub_37f6b0
    Lifted::new(0x7000a6b7e0, tpl_49::<0x1c2> as *const (), 1), // +0x37f7e0 sub_37f7e0
    Lifted::new(0x7000a6b7f0, tpl_245::<0x1c4> as *const (), 1), // +0x37f7f0 sub_37f7f0
    Lifted::new(0x7000a6b800, fn_7000a6b800 as *const (), 1), // +0x37f800 sub_37f800
    Lifted::new(0x7000a6b810, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a6b821> as *const (), 0), // +0x37f810 sub_37f810
    Lifted::new(0x7000a6b830, fn_7000a6b830 as *const (), 1), // +0x37f830 sub_37f830
    Lifted::new(0x7000a6b840, fn_7000a6b840 as *const (), 1), // +0x37f840 sub_37f840
    Lifted::new(0x7000a6b860, fn_7000a6b860 as *const (), 1), // +0x37f860 sub_37f860
    Lifted::new(0x7000a6b870, tpl_51::<0x100> as *const (), 1), // +0x37f870 sub_37f870
    Lifted::new(0x7000a6b880, tpl_49::<0x1c8> as *const (), 1), // +0x37f880 sub_37f880
    Lifted::new(0x7000a6b890, tpl_51::<0x110> as *const (), 1), // +0x37f890 sub_37f890
    Lifted::new(0x7000a6b8a0, fn_7000a6b8a0 as *const (), 1), // +0x37f8a0 sub_37f8a0
    Lifted::new(0x7000a6b8c0, tpl_49::<0x108> as *const (), 1), // +0x37f8c0 sub_37f8c0
    Lifted::new(0x7000a6b8d0, tpl_49::<0x1c0> as *const (), 1), // +0x37f8d0 sub_37f8d0
    Lifted::new(0x7000a6b8e0, fn_7000a6b8e0 as *const (), 1), // +0x37f8e0 sub_37f8e0
    Lifted::new(0x7000a6b930, fn_7000a6b930 as *const (), 2), // +0x37f930 sub_37f930
    Lifted::new(0x7000a6b970, fn_7000a6b970 as *const (), 0), // +0x37f970 sub_37f970
    Lifted::new(0x7000a6b9a0, fn_7000a6b9a0 as *const (), 2), // +0x37f9a0 sub_37f9a0
    Lifted::new(0x7000a6b9e0, fn_7000a6b9e0 as *const (), 0), // +0x37f9e0 sub_37f9e0
    Lifted::new(0x7000a6ba00, fn_7000a6ba00 as *const (), 2), // +0x37fa00 sub_37fa00
    Lifted::new(0x7000a6ba40, fn_7000a6ba40 as *const (), 0), // +0x37fa40 sub_37fa40
    Lifted::new(0x7000a6ba70, tpl_651::<0x8, 0xfffffffffffffffc, 0x700070e210> as *const (), 2), // +0x37fa70 sub_37fa70
    Lifted::new(0x7000a6bab0, tpl_652::<0xfffffffffffffffc, 0x70007198f0> as *const (), 0), // +0x37fab0 sub_37fab0
    Lifted::new(0x7000a6bad0, tpl_655::<0x7000755920> as *const (), 0), // +0x37fad0 sub_37fad0
    Lifted::new(0x7000a6bb00, tpl_656::<0x8, 0x7000725050> as *const (), 2), // +0x37fb00 sub_37fb00
    Lifted::new(0x7000a6bb50, tpl_655::<0x70007049c0> as *const (), 0), // +0x37fb50 sub_37fb50
    Lifted::new(0x7000a6bb80, tpl_656::<0x8, 0x700070b220> as *const (), 2), // +0x37fb80 sub_37fb80
    Lifted::new(0x7000a6bbd0, tpl_47::<0x1c2> as *const (), 1), // +0x37fbd0 sub_37fbd0
    Lifted::new(0x7000a6bbe0, fn_7000a6bbe0 as *const (), 2), // +0x37fbe0 sub_37fbe0
    Lifted::new(0x7000a6bc20, fn_7000a6bc20 as *const (), 0), // +0x37fc20 sub_37fc20
    Lifted::new(0x7000a6bc50, tpl_649::<0x8, 0x1, 0x700070e210> as *const (), 2), // +0x37fc50 sub_37fc50
    Lifted::new(0x7000a6bc90, tpl_650::<0x1, 0x70007198f0> as *const (), 0), // +0x37fc90 sub_37fc90
    Lifted::new(0x7000a6bcb0, fn_7000a6bcb0 as *const (), 2), // +0x37fcb0 sub_37fcb0
    Lifted::new(0x7000a6bcf0, fn_7000a6bcf0 as *const (), 0), // +0x37fcf0 sub_37fcf0
    Lifted::new(0x7000a6bd20, tpl_651::<0x8, 0xfffffffffffffffd, 0x700070e210> as *const (), 2), // +0x37fd20 sub_37fd20
    Lifted::new(0x7000a6bd60, tpl_652::<0xfffffffffffffffd, 0x70007198f0> as *const (), 0), // +0x37fd60 sub_37fd60
    Lifted::new(0x7000a6bd80, tpl_657::<0x7000755920> as *const (), 0), // +0x37fd80 sub_37fd80
    Lifted::new(0x7000a6bdb0, tpl_658::<0x8, 0x7000725050> as *const (), 2), // +0x37fdb0 sub_37fdb0
    Lifted::new(0x7000a6be00, tpl_657::<0x70007049c0> as *const (), 0), // +0x37fe00 sub_37fe00
    Lifted::new(0x7000a6be30, tpl_658::<0x8, 0x700070b220> as *const (), 2), // +0x37fe30 sub_37fe30
    Lifted::new(0x7000a6be80, fn_7000a6be80 as *const (), 0), // +0x37fe80 sub_37fe80
    Lifted::new(0x7000a6beb0, fn_7000a6beb0 as *const (), 2), // +0x37feb0 sub_37feb0
    Lifted::new(0x7000a6bef0, fn_7000a6bef0 as *const (), 0), // +0x37fef0 sub_37fef0
    Lifted::new(0x7000a6bf10, fn_7000a6bf10 as *const (), 2), // +0x37ff10 sub_37ff10
    Lifted::new(0x7000a6bf50, fn_7000a6bf50 as *const (), 0), // +0x37ff50 sub_37ff50
    Lifted::new(0x7000a6bf80, fn_7000a6bf80 as *const (), 2), // +0x37ff80 sub_37ff80
    Lifted::new(0x7000a6bfc0, fn_7000a6bfc0 as *const (), 0), // +0x37ffc0 sub_37ffc0
    Lifted::new(0x7000a6bff0, fn_7000a6bff0 as *const (), 2), // +0x37fff0 sub_37fff0
    Lifted::new(0x7000a6c030, fn_7000a6c030 as *const (), 0), // +0x380030 sub_380030
    Lifted::new(0x7000a6c090, fn_7000a6c090 as *const (), 0), // +0x380090 sub_380090
    Lifted::new(0x7000a6c0a0, fn_7000a6c0a0 as *const (), 0), // +0x3800a0 sub_3800a0
    Lifted::new(0x7000a6c0d0, fn_7000a6c0d0 as *const (), 2), // +0x3800d0 sub_3800d0
    Lifted::new(0x7000a6c110, fn_7000a6c110 as *const (), 0), // +0x380110 sub_380110
    Lifted::new(0x7000a6c130, fn_7000a6c130 as *const (), 2), // +0x380130 sub_380130
    Lifted::new(0x7000a6c170, fn_7000a6c170 as *const (), 0), // +0x380170 sub_380170
    Lifted::new(0x7000a6c1a0, fn_7000a6c1a0 as *const (), 2), // +0x3801a0 sub_3801a0
    Lifted::new(0x7000a6c1e0, fn_7000a6c1e0 as *const (), 0), // +0x3801e0 sub_3801e0
    Lifted::new(0x7000a6c210, fn_7000a6c210 as *const (), 2), // +0x380210 sub_380210
    Lifted::new(0x7000a6c250, tpl_640::<0x8, 0x7000782e10> as *const (), 2), // +0x380250 sub_380250
    Lifted::new(0x7000a6c290, tpl_640::<0x8, 0x70007973f0> as *const (), 2), // +0x380290 sub_380290
    Lifted::new(0x7000a6c2d0, tpl_47::<0x1c1> as *const (), 1), // +0x3802d0 sub_3802d0
    Lifted::new(0x7000a6c2e0, fn_7000a6c2e0 as *const (), 0), // +0x3802e0 sub_3802e0
    Lifted::new(0x7000a6c310, fn_7000a6c310 as *const (), 2), // +0x380310 sub_380310
    Lifted::new(0x7000a6c350, fn_7000a6c350 as *const (), 1), // +0x380350 sub_380350
    Lifted::new(0x7000a6c3d0, fn_7000a6c3d0 as *const (), 0), // +0x3803d0 sub_3803d0
    Lifted::new(0x7000a6c3f0, fn_7000a6c3f0 as *const (), 2), // +0x3803f0 sub_3803f0
    Lifted::new(0x7000a6c430, fn_7000a6c430 as *const (), 0), // +0x380430 sub_380430
    Lifted::new(0x7000a6c480, fn_7000a6c480 as *const (), 2), // +0x380480 sub_380480
    Lifted::new(0x7000a6c4f0, fn_7000a6c4f0 as *const (), 0), // +0x3804f0 sub_3804f0
    Lifted::new(0x7000a6c520, fn_7000a6c520 as *const (), 2), // +0x380520 sub_380520
    Lifted::new(0x7000a6c560, fn_7000a6c560 as *const (), 0), // +0x380560 sub_380560
    Lifted::new(0x7000a6c5b0, fn_7000a6c5b0 as *const (), 2), // +0x3805b0 sub_3805b0
    Lifted::new(0x7000a6c620, fn_7000a6c620 as *const (), 0), // +0x380620 sub_380620
    Lifted::new(0x7000a6c670, fn_7000a6c670 as *const (), 2), // +0x380670 sub_380670
    Lifted::new(0x7000a6c6e0, fn_7000a6c6e0 as *const (), 0), // +0x3806e0 sub_3806e0
    Lifted::new(0x7000a6c730, fn_7000a6c730 as *const (), 2), // +0x380730 sub_380730
    Lifted::new(0x7000a6c7a0, fn_7000a6c7a0 as *const (), 0), // +0x3807a0 sub_3807a0
    Lifted::new(0x7000a6c7d0, fn_7000a6c7d0 as *const (), 2), // +0x3807d0 sub_3807d0
    Lifted::new(0x7000a6c810, fn_7000a6c810 as *const (), 0), // +0x380810 sub_380810
    Lifted::new(0x7000a6c860, fn_7000a6c860 as *const (), 2), // +0x380860 sub_380860
    Lifted::new(0x7000a6c8d0, fn_7000a6c8d0 as *const (), 0), // +0x3808d0 sub_3808d0
    Lifted::new(0x7000a6c900, fn_7000a6c900 as *const (), 2), // +0x380900 sub_380900
    Lifted::new(0x7000a6c940, fn_7000a6c940 as *const (), 0), // +0x380940 sub_380940
    Lifted::new(0x7000a6c9b0, fn_7000a6c9b0 as *const (), 0), // +0x3809b0 sub_3809b0
    Lifted::new(0x7000a6ca20, tpl_659::<0x70007023f0, 0x20> as *const (), 0), // +0x380a20 sub_380a20
    Lifted::new(0x7000a6ca60, tpl_660::<0x8, 0x700070b100, 0x20> as *const (), 2), // +0x380a60 sub_380a60
    Lifted::new(0x7000a6cac0, tpl_659::<0x70006f9900, 0x20> as *const (), 0), // +0x380ac0 sub_380ac0
    Lifted::new(0x7000a6cb00, tpl_660::<0x8, 0x70007573d0, 0x20> as *const (), 2), // +0x380b00 sub_380b00
    Lifted::new(0x7000a6cb60, tpl_661::<0x70007023f0, 0x20> as *const (), 0), // +0x380b60 sub_380b60
    Lifted::new(0x7000a6cba0, tpl_662::<0x8, 0x700070b100, 0x20> as *const (), 2), // +0x380ba0 sub_380ba0
    Lifted::new(0x7000a6cc00, tpl_661::<0x70006f9900, 0x20> as *const (), 0), // +0x380c00 sub_380c00
    Lifted::new(0x7000a6cc40, tpl_662::<0x8, 0x70007573d0, 0x20> as *const (), 2), // +0x380c40 sub_380c40
    Lifted::new(0x7000a6cca0, tpl_663::<0x70007023f0, 0x20> as *const (), 0), // +0x380ca0 sub_380ca0
    Lifted::new(0x7000a6cce0, tpl_664::<0x8, 0x700070b100, 0x20> as *const (), 2), // +0x380ce0 sub_380ce0
    Lifted::new(0x7000a6cd40, tpl_663::<0x70006f9900, 0x20> as *const (), 0), // +0x380d40 sub_380d40
    Lifted::new(0x7000a6cd80, tpl_664::<0x8, 0x70007573d0, 0x20> as *const (), 2), // +0x380d80 sub_380d80
    Lifted::new(0x7000a6cde0, tpl_665::<0x70007023f0, 0x20> as *const (), 0), // +0x380de0 sub_380de0
    Lifted::new(0x7000a6ce20, tpl_666::<0x8, 0x700070b100, 0x20> as *const (), 2), // +0x380e20 sub_380e20
    Lifted::new(0x7000a6ce80, tpl_665::<0x70006f9900, 0x20> as *const (), 0), // +0x380e80 sub_380e80
    Lifted::new(0x7000a6cec0, tpl_666::<0x8, 0x70007573d0, 0x20> as *const (), 2), // +0x380ec0 sub_380ec0
    Lifted::new(0x7000a6cf20, tpl_667::<0x70007023f0, 0x20> as *const (), 0), // +0x380f20 sub_380f20
    Lifted::new(0x7000a6cf60, tpl_668::<0x8, 0x700070b100, 0x20> as *const (), 2), // +0x380f60 sub_380f60
    Lifted::new(0x7000a6cfc0, tpl_667::<0x70006f9900, 0x20> as *const (), 0), // +0x380fc0 sub_380fc0
    Lifted::new(0x7000a6d000, tpl_668::<0x8, 0x70007573d0, 0x20> as *const (), 2), // +0x381000 sub_381000
    Lifted::new(0x7000a6d060, tpl_669::<0x70007023f0, 0x20> as *const (), 0), // +0x381060 sub_381060
    Lifted::new(0x7000a6d0a0, tpl_670::<0x8, 0x700070b100, 0x20> as *const (), 2), // +0x3810a0 sub_3810a0
    Lifted::new(0x7000a6d100, tpl_669::<0x70006f9900, 0x20> as *const (), 0), // +0x381100 sub_381100
    Lifted::new(0x7000a6d140, tpl_670::<0x8, 0x70007573d0, 0x20> as *const (), 2), // +0x381140 sub_381140
    Lifted::new(0x7000a6d1a0, tpl_671::<0x70007023f0, 0x20> as *const (), 0), // +0x3811a0 sub_3811a0
    Lifted::new(0x7000a6d1e0, tpl_672::<0x8, 0x700070b100, 0x20> as *const (), 2), // +0x3811e0 sub_3811e0
    Lifted::new(0x7000a6d240, tpl_671::<0x70006f9900, 0x20> as *const (), 0), // +0x381240 sub_381240
    Lifted::new(0x7000a6d280, tpl_672::<0x8, 0x70007573d0, 0x20> as *const (), 2), // +0x381280 sub_381280
    Lifted::new(0x7000a6d2e0, tpl_673::<0x70007023f0, 0x20> as *const (), 0), // +0x3812e0 sub_3812e0
    Lifted::new(0x7000a6d320, tpl_674::<0x8, 0x700070b100, 0x20> as *const (), 2), // +0x381320 sub_381320
    Lifted::new(0x7000a6d380, tpl_673::<0x70006f9900, 0x20> as *const (), 0), // +0x381380 sub_381380
    Lifted::new(0x7000a6d3c0, tpl_674::<0x8, 0x70007573d0, 0x20> as *const (), 2), // +0x3813c0 sub_3813c0
    Lifted::new(0x7000a6d420, fn_7000a6d420 as *const (), 0), // +0x381420 sub_381420
    Lifted::new(0x7000a6d470, tpl_675::<0x70017d94f0, 0x70006f9900> as *const (), 0), // +0x381470 sub_381470
    Lifted::new(0x7000a6d4a0, tpl_676::<0x8, 0x70017d94f0, 0x70007573d0> as *const (), 2), // +0x3814a0 sub_3814a0
    Lifted::new(0x7000a6d4f0, fn_7000a6d4f0 as *const (), 2), // +0x3814f0 sub_3814f0
    Lifted::new(0x7000a6d560, fn_7000a6d560 as *const (), 0), // +0x381560 sub_381560
    Lifted::new(0x7000a6d5b0, fn_7000a6d5b0 as *const (), 2), // +0x3815b0 sub_3815b0
    Lifted::new(0x7000a6d620, fn_7000a6d620 as *const (), 0), // +0x381620 sub_381620
    Lifted::new(0x7000a6d670, tpl_677::<0x70017d94f0, 0x70007023f0> as *const (), 0), // +0x381670 sub_381670
    Lifted::new(0x7000a6d6a0, tpl_678::<0x8, 0x70017d94f0, 0x700070b100> as *const (), 2), // +0x3816a0 sub_3816a0
    Lifted::new(0x7000a6d6f0, tpl_675::<0x70017d94f0, 0x70007023f0> as *const (), 0), // +0x3816f0 sub_3816f0
    Lifted::new(0x7000a6d720, tpl_676::<0x8, 0x70017d94f0, 0x700070b100> as *const (), 2), // +0x381720 sub_381720
    Lifted::new(0x7000a6d770, tpl_677::<0x70017d94f0, 0x70006f9900> as *const (), 0), // +0x381770 sub_381770
    Lifted::new(0x7000a6d7a0, tpl_678::<0x8, 0x70017d94f0, 0x70007573d0> as *const (), 2), // +0x3817a0 sub_3817a0
    Lifted::new(0x7000a6d7f0, fn_7000a6d7f0 as *const (), 2), // +0x3817f0 sub_3817f0
    Lifted::new(0x7000a6d860, fn_7000a6d860 as *const (), 0), // +0x381860 sub_381860
    Lifted::new(0x7000a6d8b0, fn_7000a6d8b0 as *const (), 2), // +0x3818b0 sub_3818b0
    Lifted::new(0x7000a6d920, fn_7000a6d920 as *const (), 0), // +0x381920 sub_381920
    Lifted::new(0x7000a6d970, fn_7000a6d970 as *const (), 0), // +0x381970 sub_381970
    Lifted::new(0x7000a6d9c0, fn_7000a6d9c0 as *const (), 0), // +0x3819c0 sub_3819c0
    Lifted::new(0x7000a6da10, fn_7000a6da10 as *const (), 2), // +0x381a10 sub_381a10
    Lifted::new(0x7000a6da80, fn_7000a6da80 as *const (), 0), // +0x381a80 sub_381a80
    Lifted::new(0x7000a6dad0, fn_7000a6dad0 as *const (), 2), // +0x381ad0 sub_381ad0
    Lifted::new(0x7000a6db40, fn_7000a6db40 as *const (), 0), // +0x381b40 sub_381b40
    Lifted::new(0x7000a6db70, fn_7000a6db70 as *const (), 2), // +0x381b70 sub_381b70
    Lifted::new(0x7000a6dbc0, fn_7000a6dbc0 as *const (), 0), // +0x381bc0 sub_381bc0
    Lifted::new(0x7000a6dc10, fn_7000a6dc10 as *const (), 2), // +0x381c10 sub_381c10
    Lifted::new(0x7000a6dc80, fn_7000a6dc80 as *const (), 0), // +0x381c80 sub_381c80
    Lifted::new(0x7000a6dcd0, fn_7000a6dcd0 as *const (), 2), // +0x381cd0 sub_381cd0
    Lifted::new(0x7000a6dd40, fn_7000a6dd40 as *const (), 0), // +0x381d40 sub_381d40
    Lifted::new(0x7000a6dd90, fn_7000a6dd90 as *const (), 2), // +0x381d90 sub_381d90
    Lifted::new(0x7000a6de00, fn_7000a6de00 as *const (), 0), // +0x381e00 sub_381e00
    Lifted::new(0x7000a6de50, fn_7000a6de50 as *const (), 2), // +0x381e50 sub_381e50
    Lifted::new(0x7000a6dec0, fn_7000a6dec0 as *const (), 0), // +0x381ec0 sub_381ec0
    Lifted::new(0x7000a6df10, fn_7000a6df10 as *const (), 0), // +0x381f10 sub_381f10
    Lifted::new(0x7000a6df60, fn_7000a6df60 as *const (), 2), // +0x381f60 sub_381f60
    Lifted::new(0x7000a6dfd0, fn_7000a6dfd0 as *const (), 0), // +0x381fd0 sub_381fd0
    Lifted::new(0x7000a6e000, fn_7000a6e000 as *const (), 2), // +0x382000 sub_382000
    Lifted::new(0x7000a6e040, fn_7000a6e040 as *const (), 0), // +0x382040 sub_382040
    Lifted::new(0x7000a6e090, fn_7000a6e090 as *const (), 2), // +0x382090 sub_382090
    Lifted::new(0x7000a6e100, fn_7000a6e100 as *const (), 0), // +0x382100 sub_382100
    Lifted::new(0x7000a6e150, fn_7000a6e150 as *const (), 2), // +0x382150 sub_382150
    Lifted::new(0x7000a6e1c0, fn_7000a6e1c0 as *const (), 2), // +0x3821c0 sub_3821c0
    Lifted::new(0x7000a6e250, fn_7000a6e250 as *const (), 0), // +0x382250 sub_382250
    Lifted::new(0x7000a6e2c0, fn_7000a6e2c0 as *const (), 0), // +0x3822c0 sub_3822c0
    Lifted::new(0x7000a6e310, fn_7000a6e310 as *const (), 2), // +0x382310 sub_382310
    Lifted::new(0x7000a6e380, fn_7000a6e380 as *const (), 0), // +0x382380 sub_382380
    Lifted::new(0x7000a6e3d0, fn_7000a6e3d0 as *const (), 2), // +0x3823d0 sub_3823d0
    Lifted::new(0x7000a6e430, fn_7000a6e430 as *const (), 0), // +0x382430 sub_382430
    Lifted::new(0x7000a6e480, fn_7000a6e480 as *const (), 2), // +0x382480 sub_382480
    Lifted::new(0x7000a6e4f0, fn_7000a6e4f0 as *const (), 0), // +0x3824f0 sub_3824f0
    Lifted::new(0x7000a6e540, fn_7000a6e540 as *const (), 2), // +0x382540 sub_382540
    Lifted::new(0x7000a6e5a0, fn_7000a6e5a0 as *const (), 0), // +0x3825a0 sub_3825a0
    Lifted::new(0x7000a6e5f0, fn_7000a6e5f0 as *const (), 2), // +0x3825f0 sub_3825f0
    Lifted::new(0x7000a6e660, fn_7000a6e660 as *const (), 0), // +0x382660 sub_382660
    Lifted::new(0x7000a6e6b0, fn_7000a6e6b0 as *const (), 2), // +0x3826b0 sub_3826b0
    Lifted::new(0x7000a6e710, fn_7000a6e710 as *const (), 0), // +0x382710 sub_382710
    Lifted::new(0x7000a6e760, fn_7000a6e760 as *const (), 2), // +0x382760 sub_382760
    Lifted::new(0x7000a6e7d0, fn_7000a6e7d0 as *const (), 0), // +0x3827d0 sub_3827d0
    Lifted::new(0x7000a6e820, fn_7000a6e820 as *const (), 2), // +0x382820 sub_382820
    Lifted::new(0x7000a6e880, fn_7000a6e880 as *const (), 0), // +0x382880 sub_382880
    Lifted::new(0x7000a6e8d0, fn_7000a6e8d0 as *const (), 2), // +0x3828d0 sub_3828d0
    Lifted::new(0x7000a6e940, fn_7000a6e940 as *const (), 0), // +0x382940 sub_382940
    Lifted::new(0x7000a6e990, fn_7000a6e990 as *const (), 2), // +0x382990 sub_382990
    Lifted::new(0x7000a6e9f0, fn_7000a6e9f0 as *const (), 0), // +0x3829f0 sub_3829f0
    Lifted::new(0x7000a6ea40, fn_7000a6ea40 as *const (), 2), // +0x382a40 sub_382a40
    Lifted::new(0x7000a6eab0, fn_7000a6eab0 as *const (), 0), // +0x382ab0 sub_382ab0
    Lifted::new(0x7000a6eb00, fn_7000a6eb00 as *const (), 2), // +0x382b00 sub_382b00
    Lifted::new(0x7000a6eb60, fn_7000a6eb60 as *const (), 0), // +0x382b60 sub_382b60
    Lifted::new(0x7000a6ebb0, fn_7000a6ebb0 as *const (), 2), // +0x382bb0 sub_382bb0
    Lifted::new(0x7000a6ec20, fn_7000a6ec20 as *const (), 0), // +0x382c20 sub_382c20
    Lifted::new(0x7000a6ec70, fn_7000a6ec70 as *const (), 2), // +0x382c70 sub_382c70
    Lifted::new(0x7000a6ecd0, fn_7000a6ecd0 as *const (), 0), // +0x382cd0 sub_382cd0
    Lifted::new(0x7000a6ed20, fn_7000a6ed20 as *const (), 2), // +0x382d20 sub_382d20
    Lifted::new(0x7000a6ed90, fn_7000a6ed90 as *const (), 0), // +0x382d90 sub_382d90
    Lifted::new(0x7000a6ede0, fn_7000a6ede0 as *const (), 2), // +0x382de0 sub_382de0
    Lifted::new(0x7000a6ee40, fn_7000a6ee40 as *const (), 0), // +0x382e40 sub_382e40
    Lifted::new(0x7000a6ee90, fn_7000a6ee90 as *const (), 2), // +0x382e90 sub_382e90
    Lifted::new(0x7000a6ef00, fn_7000a6ef00 as *const (), 0), // +0x382f00 sub_382f00
    Lifted::new(0x7000a6ef50, fn_7000a6ef50 as *const (), 2), // +0x382f50 sub_382f50
    Lifted::new(0x7000a6efb0, fn_7000a6efb0 as *const (), 0), // +0x382fb0 sub_382fb0
    Lifted::new(0x7000a6f000, fn_7000a6f000 as *const (), 2), // +0x383000 sub_383000
    Lifted::new(0x7000a6f070, fn_7000a6f070 as *const (), 0), // +0x383070 sub_383070
    Lifted::new(0x7000a6f0c0, fn_7000a6f0c0 as *const (), 2), // +0x3830c0 sub_3830c0
    Lifted::new(0x7000a6f120, fn_7000a6f120 as *const (), 0), // +0x383120 sub_383120
    Lifted::new(0x7000a6f170, fn_7000a6f170 as *const (), 2), // +0x383170 sub_383170
    Lifted::new(0x7000a6f1e0, fn_7000a6f1e0 as *const (), 0), // +0x3831e0 sub_3831e0
    Lifted::new(0x7000a6f230, fn_7000a6f230 as *const (), 2), // +0x383230 sub_383230
    Lifted::new(0x7000a6f290, fn_7000a6f290 as *const (), 0), // +0x383290 sub_383290
    Lifted::new(0x7000a6f2e0, fn_7000a6f2e0 as *const (), 2), // +0x3832e0 sub_3832e0
    Lifted::new(0x7000a6f350, fn_7000a6f350 as *const (), 0), // +0x383350 sub_383350
    Lifted::new(0x7000a6f3a0, fn_7000a6f3a0 as *const (), 2), // +0x3833a0 sub_3833a0
    Lifted::new(0x7000a6f400, fn_7000a6f400 as *const (), 0), // +0x383400 sub_383400
    Lifted::new(0x7000a6f450, fn_7000a6f450 as *const (), 2), // +0x383450 sub_383450
    Lifted::new(0x7000a6f4c0, fn_7000a6f4c0 as *const (), 0), // +0x3834c0 sub_3834c0
    Lifted::new(0x7000a6f510, fn_7000a6f510 as *const (), 2), // +0x383510 sub_383510
    Lifted::new(0x7000a6f570, fn_7000a6f570 as *const (), 0), // +0x383570 sub_383570
    Lifted::new(0x7000a6f5c0, fn_7000a6f5c0 as *const (), 2), // +0x3835c0 sub_3835c0
    Lifted::new(0x7000a6f630, fn_7000a6f630 as *const (), 0), // +0x383630 sub_383630
    Lifted::new(0x7000a6f680, fn_7000a6f680 as *const (), 2), // +0x383680 sub_383680
    Lifted::new(0x7000a6f6e0, fn_7000a6f6e0 as *const (), 0), // +0x3836e0 sub_3836e0
    Lifted::new(0x7000a6f730, fn_7000a6f730 as *const (), 2), // +0x383730 sub_383730
    Lifted::new(0x7000a6f7a0, fn_7000a6f7a0 as *const (), 0), // +0x3837a0 sub_3837a0
    Lifted::new(0x7000a6f810, fn_7000a6f810 as *const (), 2), // +0x383810 sub_383810
    Lifted::new(0x7000a6f8a0, fn_7000a6f8a0 as *const (), 2), // +0x3838a0 sub_3838a0
    Lifted::new(0x7000a6f910, fn_7000a6f910 as *const (), 0), // +0x383910 sub_383910
    Lifted::new(0x7000a6f960, fn_7000a6f960 as *const (), 2), // +0x383960 sub_383960
    Lifted::new(0x7000a6f9d0, fn_7000a6f9d0 as *const (), 0), // +0x3839d0 sub_3839d0
    Lifted::new(0x7000a6fa00, fn_7000a6fa00 as *const (), 2), // +0x383a00 sub_383a00
    Lifted::new(0x7000a6fa40, fn_7000a6fa40 as *const (), 0), // +0x383a40 sub_383a40
    Lifted::new(0x7000a6fa90, fn_7000a6fa90 as *const (), 2), // +0x383a90 sub_383a90
    Lifted::new(0x7000a6fb00, fn_7000a6fb00 as *const (), 0), // +0x383b00 sub_383b00
    Lifted::new(0x7000a6fb50, fn_7000a6fb50 as *const (), 2), // +0x383b50 sub_383b50
    Lifted::new(0x7000a6fbc0, fn_7000a6fbc0 as *const (), 0), // +0x383bc0 sub_383bc0
    Lifted::new(0x7000a6fc10, fn_7000a6fc10 as *const (), 2), // +0x383c10 sub_383c10
    Lifted::new(0x7000a6fc80, fn_7000a6fc80 as *const (), 0), // +0x383c80 sub_383c80
    Lifted::new(0x7000a6fcb0, fn_7000a6fcb0 as *const (), 2), // +0x383cb0 sub_383cb0
    Lifted::new(0x7000a6fcf0, fn_7000a6fcf0 as *const (), 0), // +0x383cf0 sub_383cf0
    Lifted::new(0x7000a6fd40, fn_7000a6fd40 as *const (), 2), // +0x383d40 sub_383d40
    Lifted::new(0x7000a6fdb0, fn_7000a6fdb0 as *const (), 0), // +0x383db0 sub_383db0
    Lifted::new(0x7000a6fe00, fn_7000a6fe00 as *const (), 2), // +0x383e00 sub_383e00
    Lifted::new(0x7000a6fe70, tpl_679::<0x70017d94f0, 0x70007023f0> as *const (), 0), // +0x383e70 sub_383e70
    Lifted::new(0x7000a6fea0, tpl_679::<0x70017d94f0, 0x70006f9900> as *const (), 0), // +0x383ea0 sub_383ea0
    Lifted::new(0x7000a6fed0, fn_7000a6fed0 as *const (), 2), // +0x383ed0 sub_383ed0
    Lifted::new(0x7000a6ff20, fn_7000a6ff20 as *const (), 0), // +0x383f20 sub_383f20
    Lifted::new(0x7000a6ff70, fn_7000a6ff70 as *const (), 0), // +0x383f70 sub_383f70
    Lifted::new(0x7000a6ffc0, tpl_680::<0x70017d94f0, 0x70007023f0> as *const (), 0), // +0x383fc0 sub_383fc0
    Lifted::new(0x7000a6fff0, tpl_680::<0x70017d94f0, 0x70006f9900> as *const (), 0), // +0x383ff0 sub_383ff0
    Lifted::new(0x7000a70020, fn_7000a70020 as *const (), 2), // +0x384020 sub_384020
    Lifted::new(0x7000a70090, fn_7000a70090 as *const (), 0), // +0x384090 sub_384090
    Lifted::new(0x7000a700e0, fn_7000a700e0 as *const (), 2), // +0x3840e0 sub_3840e0
    Lifted::new(0x7000a70150, tpl_681::<0x8, 0x70017d94f0, 0x7000733210> as *const (), 2), // +0x384150 sub_384150
    Lifted::new(0x7000a70190, fn_7000a70190 as *const (), 0), // +0x384190 sub_384190
    Lifted::new(0x7000a701c0, tpl_682::<0x8, 0x70017d94f0, 0x7000733210> as *const (), 2), // +0x3841c0 sub_3841c0
    Lifted::new(0x7000a70210, tpl_681::<0x8, 0x70017d94f0, 0x700070b100> as *const (), 2), // +0x384210 sub_384210
    Lifted::new(0x7000a70250, tpl_682::<0x8, 0x70017d94f0, 0x700070b100> as *const (), 2), // +0x384250 sub_384250
    Lifted::new(0x7000a702a0, tpl_681::<0x8, 0x70017d94f0, 0x70007573d0> as *const (), 2), // +0x3842a0 sub_3842a0
    Lifted::new(0x7000a702e0, fn_7000a702e0 as *const (), 0), // +0x3842e0 sub_3842e0
    Lifted::new(0x7000a70310, tpl_682::<0x8, 0x70017d94f0, 0x70007573d0> as *const (), 2), // +0x384310 sub_384310
    Lifted::new(0x7000a70360, fn_7000a70360 as *const (), 0), // +0x384360 sub_384360
    Lifted::new(0x7000a703b0, fn_7000a703b0 as *const (), 2), // +0x3843b0 sub_3843b0
    Lifted::new(0x7000a70420, fn_7000a70420 as *const (), 0), // +0x384420 sub_384420
    Lifted::new(0x7000a70450, fn_7000a70450 as *const (), 2), // +0x384450 sub_384450
    Lifted::new(0x7000a70490, fn_7000a70490 as *const (), 0), // +0x384490 sub_384490
    Lifted::new(0x7000a704e0, fn_7000a704e0 as *const (), 2), // +0x3844e0 sub_3844e0
    Lifted::new(0x7000a70550, fn_7000a70550 as *const (), 0), // +0x384550 sub_384550
    Lifted::new(0x7000a705a0, fn_7000a705a0 as *const (), 2), // +0x3845a0 sub_3845a0
    Lifted::new(0x7000a70610, fn_7000a70610 as *const (), 0), // +0x384610 sub_384610
    Lifted::new(0x7000a70660, fn_7000a70660 as *const (), 2), // +0x384660 sub_384660
    Lifted::new(0x7000a706d0, fn_7000a706d0 as *const (), 0), // +0x3846d0 sub_3846d0
    Lifted::new(0x7000a70700, fn_7000a70700 as *const (), 2), // +0x384700 sub_384700
    Lifted::new(0x7000a70740, fn_7000a70740 as *const (), 0), // +0x384740 sub_384740
    Lifted::new(0x7000a70790, fn_7000a70790 as *const (), 2), // +0x384790 sub_384790
    Lifted::new(0x7000a70800, fn_7000a70800 as *const (), 0), // +0x384800 sub_384800
    Lifted::new(0x7000a70850, fn_7000a70850 as *const (), 2), // +0x384850 sub_384850
    Lifted::new(0x7000a708c0, fn_7000a708c0 as *const (), 0), // +0x3848c0 sub_3848c0
    Lifted::new(0x7000a70910, fn_7000a70910 as *const (), 2), // +0x384910 sub_384910
    Lifted::new(0x7000a70980, fn_7000a70980 as *const (), 0), // +0x384980 sub_384980
    Lifted::new(0x7000a709d0, fn_7000a709d0 as *const (), 2), // +0x3849d0 sub_3849d0
    Lifted::new(0x7000a70a40, fn_7000a70a40 as *const (), 0), // +0x384a40 sub_384a40
    Lifted::new(0x7000a70a90, fn_7000a70a90 as *const (), 2), // +0x384a90 sub_384a90
    Lifted::new(0x7000a70b00, fn_7000a70b00 as *const (), 0), // +0x384b00 sub_384b00
    Lifted::new(0x7000a70b50, fn_7000a70b50 as *const (), 2), // +0x384b50 sub_384b50
    Lifted::new(0x7000a70bc0, fn_7000a70bc0 as *const (), 0), // +0x384bc0 sub_384bc0
    Lifted::new(0x7000a70c10, fn_7000a70c10 as *const (), 2), // +0x384c10 sub_384c10
    Lifted::new(0x7000a70c80, fn_7000a70c80 as *const (), 0), // +0x384c80 sub_384c80
    Lifted::new(0x7000a70cd0, fn_7000a70cd0 as *const (), 2), // +0x384cd0 sub_384cd0
    Lifted::new(0x7000a70d40, fn_7000a70d40 as *const (), 0), // +0x384d40 sub_384d40
    Lifted::new(0x7000a70d90, fn_7000a70d90 as *const (), 2), // +0x384d90 sub_384d90
    Lifted::new(0x7000a70e00, fn_7000a70e00 as *const (), 0), // +0x384e00 sub_384e00
    Lifted::new(0x7000a70e50, fn_7000a70e50 as *const (), 2), // +0x384e50 sub_384e50
    Lifted::new(0x7000a70ec0, fn_7000a70ec0 as *const (), 0), // +0x384ec0 sub_384ec0
    Lifted::new(0x7000a70f10, fn_7000a70f10 as *const (), 2), // +0x384f10 sub_384f10
    Lifted::new(0x7000a70f80, fn_7000a70f80 as *const (), 0), // +0x384f80 sub_384f80
    Lifted::new(0x7000a70fd0, fn_7000a70fd0 as *const (), 2), // +0x384fd0 sub_384fd0
    Lifted::new(0x7000a71040, fn_7000a71040 as *const (), 0), // +0x385040 sub_385040
    Lifted::new(0x7000a71090, fn_7000a71090 as *const (), 2), // +0x385090 sub_385090
    Lifted::new(0x7000a71100, fn_7000a71100 as *const (), 0), // +0x385100 sub_385100
    Lifted::new(0x7000a71150, fn_7000a71150 as *const (), 2), // +0x385150 sub_385150
    Lifted::new(0x7000a711c0, fn_7000a711c0 as *const (), 0), // +0x3851c0 sub_3851c0
    Lifted::new(0x7000a71210, fn_7000a71210 as *const (), 2), // +0x385210 sub_385210
    Lifted::new(0x7000a71280, fn_7000a71280 as *const (), 0), // +0x385280 sub_385280
    Lifted::new(0x7000a712f0, fn_7000a712f0 as *const (), 2), // +0x3852f0 sub_3852f0
    Lifted::new(0x7000a71380, fn_7000a71380 as *const (), 0), // +0x385380 sub_385380
    Lifted::new(0x7000a713d0, fn_7000a713d0 as *const (), 2), // +0x3853d0 sub_3853d0
    Lifted::new(0x7000a71440, fn_7000a71440 as *const (), 0), // +0x385440 sub_385440
    Lifted::new(0x7000a71490, fn_7000a71490 as *const (), 2), // +0x385490 sub_385490
    Lifted::new(0x7000a71500, fn_7000a71500 as *const (), 0), // +0x385500 sub_385500
    Lifted::new(0x7000a71550, fn_7000a71550 as *const (), 2), // +0x385550 sub_385550
    Lifted::new(0x7000a715c0, fn_7000a715c0 as *const (), 0), // +0x3855c0 sub_3855c0
    Lifted::new(0x7000a71610, fn_7000a71610 as *const (), 2), // +0x385610 sub_385610
    Lifted::new(0x7000a71680, fn_7000a71680 as *const (), 0), // +0x385680 sub_385680
    Lifted::new(0x7000a716f0, fn_7000a716f0 as *const (), 0), // +0x3856f0 sub_3856f0
    Lifted::new(0x7000a71760, fn_7000a71760 as *const (), 2), // +0x385760 sub_385760
    Lifted::new(0x7000a717f0, fn_7000a717f0 as *const (), 0), // +0x3857f0 sub_3857f0
    Lifted::new(0x7000a71840, fn_7000a71840 as *const (), 2), // +0x385840 sub_385840
    Lifted::new(0x7000a718b0, fn_7000a718b0 as *const (), 0), // +0x3858b0 sub_3858b0
    Lifted::new(0x7000a71900, fn_7000a71900 as *const (), 2), // +0x385900 sub_385900
    Lifted::new(0x7000a71970, fn_7000a71970 as *const (), 0), // +0x385970 sub_385970
    Lifted::new(0x7000a719c0, fn_7000a719c0 as *const (), 2), // +0x3859c0 sub_3859c0
    Lifted::new(0x7000a71a30, fn_7000a71a30 as *const (), 0), // +0x385a30 sub_385a30
    Lifted::new(0x7000a71a80, fn_7000a71a80 as *const (), 2), // +0x385a80 sub_385a80
    Lifted::new(0x7000a71af0, fn_7000a71af0 as *const (), 0), // +0x385af0 sub_385af0
    Lifted::new(0x7000a71b40, fn_7000a71b40 as *const (), 2), // +0x385b40 sub_385b40
    Lifted::new(0x7000a71bb0, fn_7000a71bb0 as *const (), 0), // +0x385bb0 sub_385bb0
    Lifted::new(0x7000a71c00, fn_7000a71c00 as *const (), 2), // +0x385c00 sub_385c00
    Lifted::new(0x7000a71c70, fn_7000a71c70 as *const (), 0), // +0x385c70 sub_385c70
    Lifted::new(0x7000a71cc0, fn_7000a71cc0 as *const (), 2), // +0x385cc0 sub_385cc0
    Lifted::new(0x7000a71d30, fn_7000a71d30 as *const (), 0), // +0x385d30 sub_385d30
    Lifted::new(0x7000a71d80, fn_7000a71d80 as *const (), 2), // +0x385d80 sub_385d80
    Lifted::new(0x7000a71df0, fn_7000a71df0 as *const (), 0), // +0x385df0 sub_385df0
    Lifted::new(0x7000a71e40, fn_7000a71e40 as *const (), 2), // +0x385e40 sub_385e40
    Lifted::new(0x7000a71eb0, fn_7000a71eb0 as *const (), 0), // +0x385eb0 sub_385eb0
    Lifted::new(0x7000a71f00, fn_7000a71f00 as *const (), 2), // +0x385f00 sub_385f00
    Lifted::new(0x7000a71f70, fn_7000a71f70 as *const (), 0), // +0x385f70 sub_385f70
    Lifted::new(0x7000a71fc0, fn_7000a71fc0 as *const (), 2), // +0x385fc0 sub_385fc0
    Lifted::new(0x7000a72030, fn_7000a72030 as *const (), 2), // +0x386030 sub_386030
    Lifted::new(0x7000a720c0, fn_7000a720c0 as *const (), 0), // +0x3860c0 sub_3860c0
    Lifted::new(0x7000a72130, fn_7000a72130 as *const (), 2), // +0x386130 sub_386130
    Lifted::new(0x7000a721c0, fn_7000a721c0 as *const (), 0), // +0x3861c0 sub_3861c0
    Lifted::new(0x7000a72230, fn_7000a72230 as *const (), 0), // +0x386230 sub_386230
    Lifted::new(0x7000a722a0, fn_7000a722a0 as *const (), 2), // +0x3862a0 sub_3862a0
    Lifted::new(0x7000a72330, fn_7000a72330 as *const (), 0), // +0x386330 sub_386330
    Lifted::new(0x7000a723a0, fn_7000a723a0 as *const (), 0), // +0x3863a0 sub_3863a0
    Lifted::new(0x7000a723f0, fn_7000a723f0 as *const (), 2), // +0x3863f0 sub_3863f0
    Lifted::new(0x7000a72460, fn_7000a72460 as *const (), 0), // +0x386460 sub_386460
    Lifted::new(0x7000a724b0, fn_7000a724b0 as *const (), 2), // +0x3864b0 sub_3864b0
    Lifted::new(0x7000a72520, fn_7000a72520 as *const (), 2), // +0x386520 sub_386520
    Lifted::new(0x7000a725b0, fn_7000a725b0 as *const (), 0), // +0x3865b0 sub_3865b0
    Lifted::new(0x7000a72620, fn_7000a72620 as *const (), 2), // +0x386620 sub_386620
    Lifted::new(0x7000a726b0, fn_7000a726b0 as *const (), 0), // +0x3866b0 sub_3866b0
    Lifted::new(0x7000a72720, fn_7000a72720 as *const (), 2), // +0x386720 sub_386720
    Lifted::new(0x7000a727b0, fn_7000a727b0 as *const (), 0), // +0x3867b0 sub_3867b0
    Lifted::new(0x7000a72820, fn_7000a72820 as *const (), 2), // +0x386820 sub_386820
    Lifted::new(0x7000a728b0, fn_7000a728b0 as *const (), 0), // +0x3868b0 sub_3868b0
    Lifted::new(0x7000a72920, fn_7000a72920 as *const (), 0), // +0x386920 sub_386920
    Lifted::new(0x7000a72970, fn_7000a72970 as *const (), 2), // +0x386970 sub_386970
    Lifted::new(0x7000a729e0, fn_7000a729e0 as *const (), 0), // +0x3869e0 sub_3869e0
    Lifted::new(0x7000a72a30, fn_7000a72a30 as *const (), 2), // +0x386a30 sub_386a30
    Lifted::new(0x7000a72aa0, fn_7000a72aa0 as *const (), 0), // +0x386aa0 sub_386aa0
    Lifted::new(0x7000a72af0, fn_7000a72af0 as *const (), 2), // +0x386af0 sub_386af0
    Lifted::new(0x7000a72b60, fn_7000a72b60 as *const (), 0), // +0x386b60 sub_386b60
    Lifted::new(0x7000a72bb0, fn_7000a72bb0 as *const (), 2), // +0x386bb0 sub_386bb0
    Lifted::new(0x7000a72c20, fn_7000a72c20 as *const (), 0), // +0x386c20 sub_386c20
    Lifted::new(0x7000a72c70, fn_7000a72c70 as *const (), 2), // +0x386c70 sub_386c70
    Lifted::new(0x7000a72ce0, fn_7000a72ce0 as *const (), 0), // +0x386ce0 sub_386ce0
    Lifted::new(0x7000a72d30, fn_7000a72d30 as *const (), 2), // +0x386d30 sub_386d30
    Lifted::new(0x7000a72da0, fn_7000a72da0 as *const (), 0), // +0x386da0 sub_386da0
    Lifted::new(0x7000a72df0, fn_7000a72df0 as *const (), 2), // +0x386df0 sub_386df0
    Lifted::new(0x7000a72e60, fn_7000a72e60 as *const (), 0), // +0x386e60 sub_386e60
    Lifted::new(0x7000a72eb0, fn_7000a72eb0 as *const (), 2), // +0x386eb0 sub_386eb0
    Lifted::new(0x7000a72f20, fn_7000a72f20 as *const (), 0), // +0x386f20 sub_386f20
    Lifted::new(0x7000a72f70, fn_7000a72f70 as *const (), 2), // +0x386f70 sub_386f70
    Lifted::new(0x7000a72fe0, fn_7000a72fe0 as *const (), 0), // +0x386fe0 sub_386fe0
    Lifted::new(0x7000a73030, fn_7000a73030 as *const (), 2), // +0x387030 sub_387030
    Lifted::new(0x7000a730a0, fn_7000a730a0 as *const (), 0), // +0x3870a0 sub_3870a0
    Lifted::new(0x7000a730f0, fn_7000a730f0 as *const (), 2), // +0x3870f0 sub_3870f0
    Lifted::new(0x7000a73160, fn_7000a73160 as *const (), 0), // +0x387160 sub_387160
    Lifted::new(0x7000a731b0, fn_7000a731b0 as *const (), 2), // +0x3871b0 sub_3871b0
    Lifted::new(0x7000a73220, fn_7000a73220 as *const (), 0), // +0x387220 sub_387220
    Lifted::new(0x7000a73270, fn_7000a73270 as *const (), 2), // +0x387270 sub_387270
    Lifted::new(0x7000a732e0, fn_7000a732e0 as *const (), 0), // +0x3872e0 sub_3872e0
    Lifted::new(0x7000a73330, fn_7000a73330 as *const (), 2), // +0x387330 sub_387330
    Lifted::new(0x7000a733a0, fn_7000a733a0 as *const (), 0), // +0x3873a0 sub_3873a0
    Lifted::new(0x7000a733f0, fn_7000a733f0 as *const (), 2), // +0x3873f0 sub_3873f0
    Lifted::new(0x7000a73460, fn_7000a73460 as *const (), 0), // +0x387460 sub_387460
    Lifted::new(0x7000a734b0, fn_7000a734b0 as *const (), 2), // +0x3874b0 sub_3874b0
    Lifted::new(0x7000a73520, fn_7000a73520 as *const (), 0), // +0x387520 sub_387520
    Lifted::new(0x7000a73570, fn_7000a73570 as *const (), 2), // +0x387570 sub_387570
    Lifted::new(0x7000a735e0, fn_7000a735e0 as *const (), 0), // +0x3875e0 sub_3875e0
    Lifted::new(0x7000a73630, fn_7000a73630 as *const (), 2), // +0x387630 sub_387630
    Lifted::new(0x7000a736a0, fn_7000a736a0 as *const (), 0), // +0x3876a0 sub_3876a0
    Lifted::new(0x7000a736f0, fn_7000a736f0 as *const (), 2), // +0x3876f0 sub_3876f0
    Lifted::new(0x7000a73760, fn_7000a73760 as *const (), 0), // +0x387760 sub_387760
    Lifted::new(0x7000a737b0, fn_7000a737b0 as *const (), 2), // +0x3877b0 sub_3877b0
    Lifted::new(0x7000a73820, fn_7000a73820 as *const (), 0), // +0x387820 sub_387820
    Lifted::new(0x7000a73870, fn_7000a73870 as *const (), 2), // +0x387870 sub_387870
    Lifted::new(0x7000a738e0, fn_7000a738e0 as *const (), 0), // +0x3878e0 sub_3878e0
    Lifted::new(0x7000a73930, fn_7000a73930 as *const (), 2), // +0x387930 sub_387930
    Lifted::new(0x7000a739a0, fn_7000a739a0 as *const (), 0), // +0x3879a0 sub_3879a0
    Lifted::new(0x7000a739f0, fn_7000a739f0 as *const (), 2), // +0x3879f0 sub_3879f0
    Lifted::new(0x7000a73a60, fn_7000a73a60 as *const (), 0), // +0x387a60 sub_387a60
    Lifted::new(0x7000a73ab0, fn_7000a73ab0 as *const (), 2), // +0x387ab0 sub_387ab0
    Lifted::new(0x7000a73b20, fn_7000a73b20 as *const (), 0), // +0x387b20 sub_387b20
    Lifted::new(0x7000a73b70, fn_7000a73b70 as *const (), 2), // +0x387b70 sub_387b70
    Lifted::new(0x7000a73be0, fn_7000a73be0 as *const (), 0), // +0x387be0 sub_387be0
    Lifted::new(0x7000a73c30, fn_7000a73c30 as *const (), 2), // +0x387c30 sub_387c30
    Lifted::new(0x7000a73ca0, fn_7000a73ca0 as *const (), 0), // +0x387ca0 sub_387ca0
    Lifted::new(0x7000a73cf0, fn_7000a73cf0 as *const (), 2), // +0x387cf0 sub_387cf0
    Lifted::new(0x7000a73d60, fn_7000a73d60 as *const (), 0), // +0x387d60 sub_387d60
    Lifted::new(0x7000a73db0, fn_7000a73db0 as *const (), 2), // +0x387db0 sub_387db0
    Lifted::new(0x7000a73e20, fn_7000a73e20 as *const (), 0), // +0x387e20 sub_387e20
    Lifted::new(0x7000a73e70, fn_7000a73e70 as *const (), 2), // +0x387e70 sub_387e70
    Lifted::new(0x7000a73ee0, fn_7000a73ee0 as *const (), 0), // +0x387ee0 sub_387ee0
    Lifted::new(0x7000a73f60, fn_7000a73f60 as *const (), 2), // +0x387f60 sub_387f60
    Lifted::new(0x7000a74000, fn_7000a74000 as *const (), 0), // +0x388000 sub_388000
    Lifted::new(0x7000a74080, tpl_683::<0x8, 0x70017d94f0, 0x70007921c0, 0x20> as *const (), 2), // +0x388080 sub_388080
    Lifted::new(0x7000a74120, fn_7000a74120 as *const (), 0), // +0x388120 sub_388120
    Lifted::new(0x7000a741a0, tpl_683::<0x8, 0x70017d94f0, 0x70007921c0, 0x20> as *const (), 2), // +0x3881a0 sub_3881a0
    Lifted::new(0x7000a74240, fn_7000a74240 as *const (), 0), // +0x388240 sub_388240
    Lifted::new(0x7000a74290, fn_7000a74290 as *const (), 2), // +0x388290 sub_388290
    Lifted::new(0x7000a74300, fn_7000a74300 as *const (), 0), // +0x388300 sub_388300
    Lifted::new(0x7000a74350, fn_7000a74350 as *const (), 2), // +0x388350 sub_388350
    Lifted::new(0x7000a743c0, fn_7000a743c0 as *const (), 0), // +0x3883c0 sub_3883c0
    Lifted::new(0x7000a74410, fn_7000a74410 as *const (), 2), // +0x388410 sub_388410
    Lifted::new(0x7000a74480, fn_7000a74480 as *const (), 0), // +0x388480 sub_388480
    Lifted::new(0x7000a744d0, fn_7000a744d0 as *const (), 0), // +0x3884d0 sub_3884d0
    Lifted::new(0x7000a74550, fn_7000a74550 as *const (), 2), // +0x388550 sub_388550
    Lifted::new(0x7000a745c0, fn_7000a745c0 as *const (), 0), // +0x3885c0 sub_3885c0
    Lifted::new(0x7000a74610, fn_7000a74610 as *const (), 0), // +0x388610 sub_388610
    Lifted::new(0x7000a74690, fn_7000a74690 as *const (), 2), // +0x388690 sub_388690
    Lifted::new(0x7000a74700, fn_7000a74700 as *const (), 0), // +0x388700 sub_388700
    Lifted::new(0x7000a74750, fn_7000a74750 as *const (), 0), // +0x388750 sub_388750
    Lifted::new(0x7000a747d0, fn_7000a747d0 as *const (), 2), // +0x3887d0 sub_3887d0
    Lifted::new(0x7000a74840, fn_7000a74840 as *const (), 0), // +0x388840 sub_388840
    Lifted::new(0x7000a74890, fn_7000a74890 as *const (), 0), // +0x388890 sub_388890
    Lifted::new(0x7000a74910, fn_7000a74910 as *const (), 2), // +0x388910 sub_388910
    Lifted::new(0x7000a74980, fn_7000a74980 as *const (), 0), // +0x388980 sub_388980
    Lifted::new(0x7000a749d0, fn_7000a749d0 as *const (), 0), // +0x3889d0 sub_3889d0
    Lifted::new(0x7000a74a50, fn_7000a74a50 as *const (), 2), // +0x388a50 sub_388a50
    Lifted::new(0x7000a74ac0, fn_7000a74ac0 as *const (), 0), // +0x388ac0 sub_388ac0
    Lifted::new(0x7000a74b10, fn_7000a74b10 as *const (), 0), // +0x388b10 sub_388b10
    Lifted::new(0x7000a74b90, fn_7000a74b90 as *const (), 2), // +0x388b90 sub_388b90
    Lifted::new(0x7000a74c00, fn_7000a74c00 as *const (), 0), // +0x388c00 sub_388c00
    Lifted::new(0x7000a74c50, fn_7000a74c50 as *const (), 0), // +0x388c50 sub_388c50
    Lifted::new(0x7000a74cd0, fn_7000a74cd0 as *const (), 2), // +0x388cd0 sub_388cd0
    Lifted::new(0x7000a74d40, fn_7000a74d40 as *const (), 0), // +0x388d40 sub_388d40
    Lifted::new(0x7000a74d90, fn_7000a74d90 as *const (), 0), // +0x388d90 sub_388d90
    Lifted::new(0x7000a74e10, fn_7000a74e10 as *const (), 2), // +0x388e10 sub_388e10
    Lifted::new(0x7000a74e80, fn_7000a74e80 as *const (), 0), // +0x388e80 sub_388e80
    Lifted::new(0x7000a74ed0, fn_7000a74ed0 as *const (), 2), // +0x388ed0 sub_388ed0
    Lifted::new(0x7000a74f40, fn_7000a74f40 as *const (), 0), // +0x388f40 sub_388f40
    Lifted::new(0x7000a74f90, fn_7000a74f90 as *const (), 2), // +0x388f90 sub_388f90
    Lifted::new(0x7000a75000, fn_7000a75000 as *const (), 0), // +0x389000 sub_389000
    Lifted::new(0x7000a75050, fn_7000a75050 as *const (), 2), // +0x389050 sub_389050
    Lifted::new(0x7000a750c0, fn_7000a750c0 as *const (), 0), // +0x3890c0 sub_3890c0
    Lifted::new(0x7000a75110, fn_7000a75110 as *const (), 2), // +0x389110 sub_389110
    Lifted::new(0x7000a75180, fn_7000a75180 as *const (), 0), // +0x389180 sub_389180
    Lifted::new(0x7000a751d0, fn_7000a751d0 as *const (), 2), // +0x3891d0 sub_3891d0
    Lifted::new(0x7000a75240, fn_7000a75240 as *const (), 0), // +0x389240 sub_389240
    Lifted::new(0x7000a75290, fn_7000a75290 as *const (), 2), // +0x389290 sub_389290
    Lifted::new(0x7000a75300, fn_7000a75300 as *const (), 0), // +0x389300 sub_389300
    Lifted::new(0x7000a75350, fn_7000a75350 as *const (), 2), // +0x389350 sub_389350
    Lifted::new(0x7000a753c0, fn_7000a753c0 as *const (), 0), // +0x3893c0 sub_3893c0
    Lifted::new(0x7000a75410, fn_7000a75410 as *const (), 2), // +0x389410 sub_389410
    Lifted::new(0x7000a75480, fn_7000a75480 as *const (), 0), // +0x389480 sub_389480
    Lifted::new(0x7000a754d0, fn_7000a754d0 as *const (), 2), // +0x3894d0 sub_3894d0
    Lifted::new(0x7000a75540, fn_7000a75540 as *const (), 0), // +0x389540 sub_389540
    Lifted::new(0x7000a75590, fn_7000a75590 as *const (), 2), // +0x389590 sub_389590
    Lifted::new(0x7000a75600, fn_7000a75600 as *const (), 0), // +0x389600 sub_389600
    Lifted::new(0x7000a75650, fn_7000a75650 as *const (), 2), // +0x389650 sub_389650
    Lifted::new(0x7000a756c0, fn_7000a756c0 as *const (), 0), // +0x3896c0 sub_3896c0
    Lifted::new(0x7000a75710, fn_7000a75710 as *const (), 2), // +0x389710 sub_389710
    Lifted::new(0x7000a75780, fn_7000a75780 as *const (), 0), // +0x389780 sub_389780
    Lifted::new(0x7000a757d0, fn_7000a757d0 as *const (), 2), // +0x3897d0 sub_3897d0
    Lifted::new(0x7000a75840, fn_7000a75840 as *const (), 0), // +0x389840 sub_389840
    Lifted::new(0x7000a75890, fn_7000a75890 as *const (), 2), // +0x389890 sub_389890
    Lifted::new(0x7000a75900, fn_7000a75900 as *const (), 0), // +0x389900 sub_389900
    Lifted::new(0x7000a75950, fn_7000a75950 as *const (), 2), // +0x389950 sub_389950
    Lifted::new(0x7000a759c0, fn_7000a759c0 as *const (), 0), // +0x3899c0 sub_3899c0
    Lifted::new(0x7000a75a10, fn_7000a75a10 as *const (), 2), // +0x389a10 sub_389a10
    Lifted::new(0x7000a75a80, fn_7000a75a80 as *const (), 0), // +0x389a80 sub_389a80
    Lifted::new(0x7000a75ad0, fn_7000a75ad0 as *const (), 2), // +0x389ad0 sub_389ad0
    Lifted::new(0x7000a75b40, fn_7000a75b40 as *const (), 0), // +0x389b40 sub_389b40
    Lifted::new(0x7000a75b90, fn_7000a75b90 as *const (), 2), // +0x389b90 sub_389b90
    Lifted::new(0x7000a75c00, fn_7000a75c00 as *const (), 0), // +0x389c00 sub_389c00
    Lifted::new(0x7000a75c30, fn_7000a75c30 as *const (), 2), // +0x389c30 sub_389c30
    Lifted::new(0x7000a75c80, fn_7000a75c80 as *const (), 0), // +0x389c80 sub_389c80
    Lifted::new(0x7000a75d00, fn_7000a75d00 as *const (), 0), // +0x389d00 sub_389d00
    Lifted::new(0x7000a75d80, fn_7000a75d80 as *const (), 0), // +0x389d80 sub_389d80
    Lifted::new(0x7000a75df0, fn_7000a75df0 as *const (), 0), // +0x389df0 sub_389df0
    Lifted::new(0x7000a75e60, fn_7000a75e60 as *const (), 0), // +0x389e60 sub_389e60
    Lifted::new(0x7000a75e90, fn_7000a75e90 as *const (), 2), // +0x389e90 sub_389e90
    Lifted::new(0x7000a75ee0, fn_7000a75ee0 as *const (), 0), // +0x389ee0 sub_389ee0
    Lifted::new(0x7000a75f30, fn_7000a75f30 as *const (), 2), // +0x389f30 sub_389f30
    Lifted::new(0x7000a75fa0, fn_7000a75fa0 as *const (), 0), // +0x389fa0 sub_389fa0
    Lifted::new(0x7000a75fd0, tpl_684::<0x70017d94f0, 0x70007023f0> as *const (), 0), // +0x389fd0 sub_389fd0
    Lifted::new(0x7000a76000, tpl_685::<0x8, 0x70017d94f0, 0x700070b100> as *const (), 2), // +0x38a000 sub_38a000
    Lifted::new(0x7000a76050, tpl_684::<0x70017d94f0, 0x70006f9900> as *const (), 0), // +0x38a050 sub_38a050
    Lifted::new(0x7000a76080, tpl_685::<0x8, 0x70017d94f0, 0x70007573d0> as *const (), 2), // +0x38a080 sub_38a080
    Lifted::new(0x7000a760d0, fn_7000a760d0 as *const (), 0), // +0x38a0d0 sub_38a0d0
    Lifted::new(0x7000a76120, fn_7000a76120 as *const (), 2), // +0x38a120 sub_38a120
    Lifted::new(0x7000a76190, fn_7000a76190 as *const (), 0), // +0x38a190 sub_38a190
    Lifted::new(0x7000a761e0, fn_7000a761e0 as *const (), 2), // +0x38a1e0 sub_38a1e0
    Lifted::new(0x7000a76250, fn_7000a76250 as *const (), 0), // +0x38a250 sub_38a250
    Lifted::new(0x7000a762a0, fn_7000a762a0 as *const (), 2), // +0x38a2a0 sub_38a2a0
    Lifted::new(0x7000a76310, fn_7000a76310 as *const (), 0), // +0x38a310 sub_38a310
    Lifted::new(0x7000a76340, fn_7000a76340 as *const (), 2), // +0x38a340 sub_38a340
    Lifted::new(0x7000a76380, fn_7000a76380 as *const (), 0), // +0x38a380 sub_38a380
    Lifted::new(0x7000a763d0, fn_7000a763d0 as *const (), 2), // +0x38a3d0 sub_38a3d0
    Lifted::new(0x7000a76440, fn_7000a76440 as *const (), 0), // +0x38a440 sub_38a440
    Lifted::new(0x7000a76490, fn_7000a76490 as *const (), 2), // +0x38a490 sub_38a490
    Lifted::new(0x7000a76500, fn_7000a76500 as *const (), 0), // +0x38a500 sub_38a500
    Lifted::new(0x7000a76530, fn_7000a76530 as *const (), 2), // +0x38a530 sub_38a530
    Lifted::new(0x7000a76580, fn_7000a76580 as *const (), 0), // +0x38a580 sub_38a580
    Lifted::new(0x7000a765b0, fn_7000a765b0 as *const (), 2), // +0x38a5b0 sub_38a5b0
    Lifted::new(0x7000a765f0, fn_7000a765f0 as *const (), 0), // +0x38a5f0 sub_38a5f0
    Lifted::new(0x7000a76640, fn_7000a76640 as *const (), 2), // +0x38a640 sub_38a640
    Lifted::new(0x7000a766b0, fn_7000a766b0 as *const (), 0), // +0x38a6b0 sub_38a6b0
    Lifted::new(0x7000a766e0, fn_7000a766e0 as *const (), 2), // +0x38a6e0 sub_38a6e0
    Lifted::new(0x7000a76720, fn_7000a76720 as *const (), 0), // +0x38a720 sub_38a720
    Lifted::new(0x7000a76750, fn_7000a76750 as *const (), 2), // +0x38a750 sub_38a750
    Lifted::new(0x7000a76790, fn_7000a76790 as *const (), 0), // +0x38a790 sub_38a790
    Lifted::new(0x7000a767b0, fn_7000a767b0 as *const (), 2), // +0x38a7b0 sub_38a7b0
    Lifted::new(0x7000a767f0, fn_7000a767f0 as *const (), 0), // +0x38a7f0 sub_38a7f0
    Lifted::new(0x7000a76960, fn_7000a76960 as *const (), 0), // +0x38a960 sub_38a960
    Lifted::new(0x7000a76b00, fn_7000a76b00 as *const (), 0), // +0x38ab00 sub_38ab00
    Lifted::new(0x7000a76cb0, fn_7000a76cb0 as *const (), 0), // +0x38acb0 sub_38acb0
    Lifted::new(0x7000a76da0, fn_7000a76da0 as *const (), 0), // +0x38ada0 sub_38ada0
    Lifted::new(0x7000a76e90, fn_7000a76e90 as *const (), 0), // +0x38ae90 sub_38ae90
    Lifted::new(0x7000a77000, fn_7000a77000 as *const (), 0), // +0x38b000 sub_38b000
    Lifted::new(0x7000a771a0, fn_7000a771a0 as *const (), 0), // +0x38b1a0 sub_38b1a0
    Lifted::new(0x7000a77350, fn_7000a77350 as *const (), 0), // +0x38b350 sub_38b350
    Lifted::new(0x7000a774d0, fn_7000a774d0 as *const (), 0), // +0x38b4d0 sub_38b4d0
    Lifted::new(0x7000a77560, fn_7000a77560 as *const (), 0), // +0x38b560 sub_38b560
    Lifted::new(0x7000a77600, fn_7000a77600 as *const (), 0), // +0x38b600 sub_38b600
    Lifted::new(0x7000a776b0, fn_7000a776b0 as *const (), 0), // +0x38b6b0 sub_38b6b0
    Lifted::new(0x7000a776e0, fn_7000a776e0 as *const (), 0), // +0x38b6e0 sub_38b6e0
    Lifted::new(0x7000a77780, fn_7000a77780 as *const (), 0), // +0x38b780 sub_38b780
    Lifted::new(0x7000a77830, fn_7000a77830 as *const (), 0), // +0x38b830 sub_38b830
    Lifted::new(0x7000a778f0, fn_7000a778f0 as *const (), 0), // +0x38b8f0 sub_38b8f0
    Lifted::new(0x7000a77930, fn_7000a77930 as *const (), 0), // +0x38b930 sub_38b930
    Lifted::new(0x7000a77960, fn_7000a77960 as *const (), 0), // +0x38b960 sub_38b960
    Lifted::new(0x7000a779a0, tpl_686::<0x7001c6abd8, 0x7000a83ad0, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38b9a0 sub_38b9a0
    Lifted::new(0x7000a77b50, tpl_687::<0x7001c6abd8, 0x7000a83a00, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38bb50 sub_38bb50
    Lifted::new(0x7000a77d00, fn_7000a77d00 as *const (), 0), // +0x38bd00 sub_38bd00
    Lifted::new(0x7000a77dc0, fn_7000a77dc0 as *const (), 0), // +0x38bdc0 sub_38bdc0
    Lifted::new(0x7000a77e40, fn_7000a77e40 as *const (), 0), // +0x38be40 sub_38be40
    Lifted::new(0x7000a77ee0, fn_7000a77ee0 as *const (), 0), // +0x38bee0 sub_38bee0
    Lifted::new(0x7000a77fb0, fn_7000a77fb0 as *const (), 0), // +0x38bfb0 sub_38bfb0
    Lifted::new(0x7000a78050, fn_7000a78050 as *const (), 0), // +0x38c050 sub_38c050
    Lifted::new(0x7000a780f0, fn_7000a780f0 as *const (), 0), // +0x38c0f0 sub_38c0f0
    Lifted::new(0x7000a78190, fn_7000a78190 as *const (), 0), // +0x38c190 sub_38c190
    Lifted::new(0x7000a781d0, fn_7000a781d0 as *const (), 0), // +0x38c1d0 sub_38c1d0
    Lifted::new(0x7000a78210, fn_7000a78210 as *const (), 0), // +0x38c210 sub_38c210
    Lifted::new(0x7000a78240, fn_7000a78240 as *const (), 0), // +0x38c240 sub_38c240
    Lifted::new(0x7000a78270, fn_7000a78270 as *const (), 0), // +0x38c270 sub_38c270
    Lifted::new(0x7000a782f0, fn_7000a782f0 as *const (), 0), // +0x38c2f0 sub_38c2f0
    Lifted::new(0x7000a78370, fn_7000a78370 as *const (), 0), // +0x38c370 sub_38c370
    Lifted::new(0x7000a783a0, fn_7000a783a0 as *const (), 0), // +0x38c3a0 sub_38c3a0
    Lifted::new(0x7000a783d0, fn_7000a783d0 as *const (), 0), // +0x38c3d0 sub_38c3d0
    Lifted::new(0x7000a78470, fn_7000a78470 as *const (), 0), // +0x38c470 sub_38c470
    Lifted::new(0x7000a78510, fn_7000a78510 as *const (), 0), // +0x38c510 sub_38c510
    Lifted::new(0x7000a78570, fn_7000a78570 as *const (), 0), // +0x38c570 sub_38c570
    Lifted::new(0x7000a78650, tpl_688::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c650 sub_38c650
    Lifted::new(0x7000a78680, tpl_688::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c680 sub_38c680
    Lifted::new(0x7000a786b0, tpl_689::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c6b0 sub_38c6b0
    Lifted::new(0x7000a786e0, tpl_689::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c6e0 sub_38c6e0
    Lifted::new(0x7000a78710, tpl_690::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c710 sub_38c710
    Lifted::new(0x7000a78740, tpl_690::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c740 sub_38c740
    Lifted::new(0x7000a78770, tpl_691::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c770 sub_38c770
    Lifted::new(0x7000a787a0, tpl_691::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c7a0 sub_38c7a0
    Lifted::new(0x7000a787d0, tpl_692::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c7d0 sub_38c7d0
    Lifted::new(0x7000a78800, tpl_692::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c800 sub_38c800
    Lifted::new(0x7000a78830, tpl_693::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c830 sub_38c830
    Lifted::new(0x7000a78860, tpl_693::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c860 sub_38c860
    Lifted::new(0x7000a78890, tpl_694::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c890 sub_38c890
    Lifted::new(0x7000a788c0, tpl_694::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c8c0 sub_38c8c0
    Lifted::new(0x7000a788f0, tpl_695::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c8f0 sub_38c8f0
    Lifted::new(0x7000a78920, tpl_695::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c920 sub_38c920
    Lifted::new(0x7000a78950, tpl_696::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c950 sub_38c950
    Lifted::new(0x7000a78980, tpl_696::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c980 sub_38c980
    Lifted::new(0x7000a789b0, tpl_697::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38c9b0 sub_38c9b0
    Lifted::new(0x7000a789e0, tpl_697::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38c9e0 sub_38c9e0
    Lifted::new(0x7000a78a10, tpl_698::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38ca10 sub_38ca10
    Lifted::new(0x7000a78a40, tpl_698::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38ca40 sub_38ca40
    Lifted::new(0x7000a78a70, tpl_699::<0x1f0, 0x7000a85ea0, 0x7000a82130> as *const (), 0), // +0x38ca70 sub_38ca70
    Lifted::new(0x7000a78aa0, tpl_699::<0x1f0, 0x7000a85f50, 0x7000a82130> as *const (), 0), // +0x38caa0 sub_38caa0
    Lifted::new(0x7000a78ad0, fn_7000a78ad0 as *const (), 0), // +0x38cad0 sub_38cad0
    Lifted::new(0x7000a78b00, fn_7000a78b00 as *const (), 0), // +0x38cb00 sub_38cb00
    Lifted::new(0x7000a78b30, fn_7000a78b30 as *const (), 0), // +0x38cb30 sub_38cb30
    Lifted::new(0x7000a78b60, fn_7000a78b60 as *const (), 0), // +0x38cb60 sub_38cb60
    Lifted::new(0x7000a78b90, fn_7000a78b90 as *const (), 0), // +0x38cb90 sub_38cb90
    Lifted::new(0x7000a78bc0, fn_7000a78bc0 as *const (), 0), // +0x38cbc0 sub_38cbc0
    Lifted::new(0x7000a78bf0, fn_7000a78bf0 as *const (), 0), // +0x38cbf0 sub_38cbf0
    Lifted::new(0x7000a78c20, fn_7000a78c20 as *const (), 0), // +0x38cc20 sub_38cc20
    Lifted::new(0x7000a78c50, fn_7000a78c50 as *const (), 0), // +0x38cc50 sub_38cc50
    Lifted::new(0x7000a78c80, fn_7000a78c80 as *const (), 0), // +0x38cc80 sub_38cc80
    Lifted::new(0x7000a78cb0, fn_7000a78cb0 as *const (), 0), // +0x38ccb0 sub_38ccb0
    Lifted::new(0x7000a78ce0, fn_7000a78ce0 as *const (), 0), // +0x38cce0 sub_38cce0
    Lifted::new(0x7000a78d40, fn_7000a78d40 as *const (), 2), // +0x38cd40 sub_38cd40
    Lifted::new(0x7000a78e10, tpl_700::<0x7001c6abd8, 0x7000a83610, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38ce10 sub_38ce10
    Lifted::new(0x7000a78fc0, tpl_701::<0x7001c6abd8, 0x7000a83540, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38cfc0 sub_38cfc0
    Lifted::new(0x7000a79170, tpl_702::<0x7001c6abd8, 0x7000a836e0, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38d170 sub_38d170
    Lifted::new(0x7000a79320, tpl_703::<0x7001c6abd8, 0x7000a833a0, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38d320 sub_38d320
    Lifted::new(0x7000a794d0, tpl_704::<0x7001c6abd8, 0x7000a83470, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38d4d0 sub_38d4d0
    Lifted::new(0x7000a79680, fn_7000a79680 as *const (), 0), // +0x38d680 sub_38d680
    Lifted::new(0x7000a79790, fn_7000a79790 as *const (), 0), // +0x38d790 sub_38d790
    Lifted::new(0x7000a797f0, fn_7000a797f0 as *const (), 0), // +0x38d7f0 sub_38d7f0
    Lifted::new(0x7000a79870, fn_7000a79870 as *const (), 0), // +0x38d870 sub_38d870
    Lifted::new(0x7000a79910, fn_7000a79910 as *const (), 0), // +0x38d910 sub_38d910
    Lifted::new(0x7000a799b0, fn_7000a799b0 as *const (), 2), // +0x38d9b0 sub_38d9b0
    Lifted::new(0x7000a79ac0, fn_7000a79ac0 as *const (), 2), // +0x38dac0 sub_38dac0
    Lifted::new(0x7000a79bd0, fn_7000a79bd0 as *const (), 2), // +0x38dbd0 sub_38dbd0
    Lifted::new(0x7000a79c80, fn_7000a79c80 as *const (), 2), // +0x38dc80 sub_38dc80
    Lifted::new(0x7000a79d90, tpl_705::<0x7000784c80, 0x1f0, 0x7000a85230, 0x7000a82130> as *const (), 0), // +0x38dd90 sub_38dd90
    Lifted::new(0x7000a79de0, fn_7000a79de0 as *const (), 0), // +0x38dde0 sub_38dde0
    Lifted::new(0x7000a79ed0, fn_7000a79ed0 as *const (), 0), // +0x38ded0 sub_38ded0
    Lifted::new(0x7000a79f10, fn_7000a79f10 as *const (), 0), // +0x38df10 sub_38df10
    Lifted::new(0x7000a79f70, fn_7000a79f70 as *const (), 0), // +0x38df70 sub_38df70
    Lifted::new(0x7000a79fd0, fn_7000a79fd0 as *const (), 2), // +0x38dfd0 sub_38dfd0
    Lifted::new(0x7000a7a080, fn_7000a7a080 as *const (), 2), // +0x38e080 sub_38e080
    Lifted::new(0x7000a7a190, fn_7000a7a190 as *const (), 2), // +0x38e190 sub_38e190
    Lifted::new(0x7000a7a2f0, fn_7000a7a2f0 as *const (), 0), // +0x38e2f0 sub_38e2f0
    Lifted::new(0x7000a7a390, tpl_706::<0x1f0, 0x7000a85230, 0x7000a82130> as *const (), 0), // +0x38e390 sub_38e390
    Lifted::new(0x7000a7a3d0, fn_7000a7a3d0 as *const (), 2), // +0x38e3d0 sub_38e3d0
    Lifted::new(0x7000a7a440, fn_7000a7a440 as *const (), 2), // +0x38e440 sub_38e440
    Lifted::new(0x7000a7a4b0, fn_7000a7a4b0 as *const (), 2), // +0x38e4b0 sub_38e4b0
    Lifted::new(0x7000a7a520, fn_7000a7a520 as *const (), 0), // +0x38e520 sub_38e520
    Lifted::new(0x7000a7a640, fn_7000a7a640 as *const (), 0), // +0x38e640 sub_38e640
    Lifted::new(0x7000a7a690, fn_7000a7a690 as *const (), 0), // +0x38e690 sub_38e690
    Lifted::new(0x7000a7a6c0, fn_7000a7a6c0 as *const (), 0), // +0x38e6c0 sub_38e6c0
    Lifted::new(0x7000a7a6f0, fn_7000a7a6f0 as *const (), 0), // +0x38e6f0 sub_38e6f0
    Lifted::new(0x7000a7a750, fn_7000a7a750 as *const (), 0), // +0x38e750 sub_38e750
    Lifted::new(0x7000a7a810, tpl_707::<0x7001c6abd8, 0x7000a83930, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38e810 sub_38e810
    Lifted::new(0x7000a7a9c0, fn_7000a7a9c0 as *const (), 0), // +0x38e9c0 sub_38e9c0
    Lifted::new(0x7000a7aa70, tpl_708::<0x7001c6abd8, 0x7000a83130, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38ea70 sub_38ea70
    Lifted::new(0x7000a7ac20, tpl_709::<0x7001c6abd8, 0x7000a83200, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38ec20 sub_38ec20
    Lifted::new(0x7000a7add0, tpl_710::<0x7001c6abd8, 0x7000a832d0, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38edd0 sub_38edd0
    Lifted::new(0x7000a7af80, tpl_711::<0x7001c6abd8, 0x7000a82f90, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38ef80 sub_38ef80
    Lifted::new(0x7000a7b130, tpl_712::<0x7001c6abd8, 0x7000a83060, 0x70006f9a50, 0x7000784c80, 0x7000a85230, 0x7000a82130> as *const (), 2), // +0x38f130 sub_38f130
    Lifted::new(0x7000a7b2e0, fn_7000a7b2e0 as *const (), 0), // +0x38f2e0 sub_38f2e0
    Lifted::new(0x7000a7b360, fn_7000a7b360 as *const (), 0), // +0x38f360 sub_38f360
    Lifted::new(0x7000a7b400, fn_7000a7b400 as *const (), 0), // +0x38f400 sub_38f400
    Lifted::new(0x7000a7b4b0, fn_7000a7b4b0 as *const (), 0), // +0x38f4b0 sub_38f4b0
    Lifted::new(0x7000a7b550, fn_7000a7b550 as *const (), 0), // +0x38f550 sub_38f550
    Lifted::new(0x7000a7b5f0, fn_7000a7b5f0 as *const (), 0), // +0x38f5f0 sub_38f5f0
    Lifted::new(0x7000a7b6a0, fn_7000a7b6a0 as *const (), 0), // +0x38f6a0 sub_38f6a0
    Lifted::new(0x7000a7b760, fn_7000a7b760 as *const (), 0), // +0x38f760 sub_38f760
    Lifted::new(0x7000a7b810, fn_7000a7b810 as *const (), 0), // +0x38f810 sub_38f810
    Lifted::new(0x7000a7b880, tpl_713::<0x7000784c80, 0x1f0, 0x7000a85230, 0x70007026c0, 0x7000762bf0, 0x1, 0x7000720700> as *const (), 0), // +0x38f880 sub_38f880
    Lifted::new(0x7000a7b910, tpl_714::<0x7000784c80, 0x1f0, 0x7000a85230, 0x70007026c0, 0x700070de90, 0x7000762bf0, 0x2, 0x7000720700> as *const (), 0), // +0x38f910 sub_38f910
    Lifted::new(0x7000a7b9d0, tpl_713::<0x7000784c80, 0x1f0, 0x7000a85230, 0x70007026c0, 0x7000762bf0, 0x1, 0x7000720700> as *const (), 0), // +0x38f9d0 sub_38f9d0
    Lifted::new(0x7000a7ba60, tpl_714::<0x7000784c80, 0x1f0, 0x7000a85230, 0x70007026c0, 0x700070de90, 0x7000762bf0, 0x2, 0x7000720700> as *const (), 0), // +0x38fa60 sub_38fa60
    Lifted::new(0x7000a7bb20, fn_7000a7bb20 as *const (), 2), // +0x38fb20 sub_38fb20
    Lifted::new(0x7000a7bbd0, tpl_715::<0x7000784c80, 0x7000a85230, 0x8, 0x70007a0620, 0x70007162e0, 0x70007026c0, 0x70006ffd10, 0x7000762bf0, 0x4, 0x7000720700> as *const (), 2), // +0x38fbd0 sub_38fbd0
    Lifted::new(0x7000a7bcc0, tpl_715::<0x7000784c80, 0x7000a85230, 0x8, 0x70007a0620, 0x70007162e0, 0x70007026c0, 0x70006ffd10, 0x7000762bf0, 0x4, 0x7000720700> as *const (), 2), // +0x38fcc0 sub_38fcc0
    Lifted::new(0x7000a7bdb0, fn_7000a7bdb0 as *const (), 0), // +0x38fdb0 sub_38fdb0
    Lifted::new(0x7000a7be40, fn_7000a7be40 as *const (), 2), // +0x38fe40 sub_38fe40
    Lifted::new(0x7000a7bf10, fn_7000a7bf10 as *const (), 2), // +0x38ff10 sub_38ff10
    Lifted::new(0x7000a7c020, fn_7000a7c020 as *const (), 0), // +0x390020 sub_390020
    Lifted::new(0x7000a7c0e0, fn_7000a7c0e0 as *const (), 0), // +0x3900e0 sub_3900e0
    Lifted::new(0x7000a7c180, tpl_713::<0x7000784c80, 0x1f0, 0x7000a85230, 0x70006ffd10, 0x7000762bf0, 0x1, 0x7000720700> as *const (), 0), // +0x390180 sub_390180
    Lifted::new(0x7000a7c210, fn_7000a7c210 as *const (), 2), // +0x390210 sub_390210
    Lifted::new(0x7000a7cc20, tpl_716::<0x7000d46c80> as *const (), 0), // +0x390c20 sub_390c20
    Lifted::new(0x7000a7cc60, tpl_716::<0x7000d46c80> as *const (), 0), // +0x390c60 sub_390c60
    Lifted::new(0x7000a7cca0, tpl_716::<0x7000d46c80> as *const (), 0), // +0x390ca0 sub_390ca0
    Lifted::new(0x7000a7cce0, tpl_716::<0x7000d46c80> as *const (), 0), // +0x390ce0 sub_390ce0
    Lifted::new(0x7000a7cd20, tpl_716::<0x7000d46c80> as *const (), 0), // +0x390d20 sub_390d20
    Lifted::new(0x7000a7cd60, fn_7000a7cd60 as *const (), 0), // +0x390d60 sub_390d60
    Lifted::new(0x7000a7cdc0, fn_7000a7cdc0 as *const (), 0), // +0x390dc0 sub_390dc0
    Lifted::new(0x7000a7ce60, fn_7000a7ce60 as *const (), 0), // +0x390e60 sub_390e60
    Lifted::new(0x7000a7cf10, fn_7000a7cf10 as *const (), 0), // +0x390f10 sub_390f10
    Lifted::new(0x7000a7cfe0, fn_7000a7cfe0 as *const (), 0), // +0x390fe0 sub_390fe0
    Lifted::new(0x7000a7d090, fn_7000a7d090 as *const (), 0), // +0x391090 sub_391090
    Lifted::new(0x7000a7d150, fn_7000a7d150 as *const (), 0), // +0x391150 sub_391150
    Lifted::new(0x7000a7d250, tpl_717::<0x7001c6abd8, 0x7000a83ad0, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x391250 sub_391250
    Lifted::new(0x7000a7d3f0, tpl_717::<0x7001c6abd8, 0x7000a83a00, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x3913f0 sub_3913f0
    Lifted::new(0x7000a7d590, fn_7000a7d590 as *const (), 0), // +0x391590 sub_391590
    Lifted::new(0x7000a7d660, tpl_718::<0x7001c6abd8, 0x70006f3410, 0x70007507b0> as *const (), 0), // +0x391660 sub_391660
    Lifted::new(0x7000a7d6f0, fn_7000a7d6f0 as *const (), 0), // +0x3916f0 sub_3916f0
    Lifted::new(0x7000a7d7a0, fn_7000a7d7a0 as *const (), 0), // +0x3917a0 sub_3917a0
    Lifted::new(0x7000a7d870, fn_7000a7d870 as *const (), 0), // +0x391870 sub_391870
    Lifted::new(0x7000a7d920, fn_7000a7d920 as *const (), 0), // +0x391920 sub_391920
    Lifted::new(0x7000a7d9d0, fn_7000a7d9d0 as *const (), 0), // +0x3919d0 sub_3919d0
    Lifted::new(0x7000a7dac0, tpl_719::<0x7001c6abd8, 0x7000705210, 0x70007507b0> as *const (), 0), // +0x391ac0 sub_391ac0
    Lifted::new(0x7000a7db40, tpl_719::<0x7001c6abd8, 0x7000705210, 0x70007507b0> as *const (), 0), // +0x391b40 sub_391b40
    Lifted::new(0x7000a7dbe0, fn_7000a7dbe0 as *const (), 0), // +0x391be0 sub_391be0
    Lifted::new(0x7000a7dc90, fn_7000a7dc90 as *const (), 0), // +0x391c90 sub_391c90
    Lifted::new(0x7000a7dd40, tpl_720::<0x7001c6abd8, 0x70006f3410, 0x7000784c80, 0x7000a85230> as *const (), 0), // +0x391d40 sub_391d40
    Lifted::new(0x7000a7ddb0, fn_7000a7ddb0 as *const (), 0), // +0x391db0 sub_391db0
    Lifted::new(0x7000a7e0d0, fn_7000a7e0d0 as *const (), 0), // +0x3920d0 sub_3920d0
    Lifted::new(0x7000a7e140, fn_7000a7e140 as *const (), 2), // +0x392140 sub_392140
    Lifted::new(0x7000a7e220, tpl_717::<0x7001c6abd8, 0x7000a83610, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x392220 sub_392220
    Lifted::new(0x7000a7e3c0, tpl_717::<0x7001c6abd8, 0x7000a83540, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x3923c0 sub_3923c0
    Lifted::new(0x7000a7e560, tpl_717::<0x7001c6abd8, 0x7000a836e0, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x392560 sub_392560
    Lifted::new(0x7000a7e700, tpl_717::<0x7001c6abd8, 0x7000a833a0, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x392700 sub_392700
    Lifted::new(0x7000a7e8a0, tpl_717::<0x7001c6abd8, 0x7000a83470, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x3928a0 sub_3928a0
    Lifted::new(0x7000a7ea40, fn_7000a7ea40 as *const (), 0), // +0x392a40 sub_392a40
    Lifted::new(0x7000a7eb50, tpl_720::<0x7001c6abd8, 0x70006f3410, 0x7000784c80, 0x7000a85230> as *const (), 0), // +0x392b50 sub_392b50
    Lifted::new(0x7000a7ebc0, fn_7000a7ebc0 as *const (), 0), // +0x392bc0 sub_392bc0
    Lifted::new(0x7000a7ec50, tpl_721::<0x7001c6abd8, 0x7000705210, 0x70007507b0, 0x7000784c80, 0x7000a85230> as *const (), 0), // +0x392c50 sub_392c50
    Lifted::new(0x7000a7ed00, tpl_721::<0x7001c6abd8, 0x7000705210, 0x70007507b0, 0x7000784c80, 0x7000a85230> as *const (), 0), // +0x392d00 sub_392d00
    Lifted::new(0x7000a7edb0, tpl_722::<0x7001c6abd8, 0x8, 0x70006f9a50, 0x70007507b0, 0x7000705210, 0x70007942a0, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x392db0 sub_392db0
    Lifted::new(0x7000a7eed0, tpl_722::<0x7001c6abd8, 0x8, 0x70006f9a50, 0x70007507b0, 0x7000705210, 0x70007942a0, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x392ed0 sub_392ed0
    Lifted::new(0x7000a7eff0, tpl_723::<0x7001c6abd8, 0x8, 0x70006f9a50, 0x70007507b0, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x392ff0 sub_392ff0
    Lifted::new(0x7000a7f0b0, tpl_722::<0x7001c6abd8, 0x8, 0x70006f9a50, 0x70007507b0, 0x7000705210, 0x70007942a0, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x3930b0 sub_3930b0
    Lifted::new(0x7000a7f1d0, tpl_724::<0x7000784c80, 0x7000a85230> as *const (), 0), // +0x3931d0 sub_3931d0
    Lifted::new(0x7000a7f210, fn_7000a7f210 as *const (), 0), // +0x393210 sub_393210
    Lifted::new(0x7000a7f310, fn_7000a7f310 as *const (), 0), // +0x393310 sub_393310
    Lifted::new(0x7000a7f350, tpl_720::<0x7001c6abd8, 0x70006f3410, 0x7000784c80, 0x7000a85230> as *const (), 0), // +0x393350 sub_393350
    Lifted::new(0x7000a7f3c0, tpl_720::<0x7001c6abd8, 0x70006f3410, 0x7000784c80, 0x7000a85230> as *const (), 0), // +0x3933c0 sub_3933c0
    Lifted::new(0x7000a7f430, tpl_723::<0x7001c6abd8, 0x8, 0x70006f9a50, 0x70007942a0, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x393430 sub_393430
    Lifted::new(0x7000a7f4f0, fn_7000a7f4f0 as *const (), 2), // +0x3934f0 sub_3934f0
    Lifted::new(0x7000a7f610, fn_7000a7f610 as *const (), 2), // +0x393610 sub_393610
    Lifted::new(0x7000a7f770, fn_7000a7f770 as *const (), 0), // +0x393770 sub_393770
    Lifted::new(0x7000a7f820, fn_7000a7f820 as *const (), 0), // +0x393820 sub_393820
    Lifted::new(0x7000a7f860, tpl_725::<0x8, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x393860 sub_393860
    Lifted::new(0x7000a7f8c0, tpl_725::<0x8, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x3938c0 sub_3938c0
    Lifted::new(0x7000a7f920, tpl_725::<0x8, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x393920 sub_393920
    Lifted::new(0x7000a7f980, fn_7000a7f980 as *const (), 0), // +0x393980 sub_393980
    Lifted::new(0x7000a7fab0, tpl_724::<0x7000784c80, 0x7000a85230> as *const (), 0), // +0x393ab0 sub_393ab0
    Lifted::new(0x7000a7faf0, fn_7000a7faf0 as *const (), 0), // +0x393af0 sub_393af0
    Lifted::new(0x7000a7fb80, fn_7000a7fb80 as *const (), 0), // +0x393b80 sub_393b80
    Lifted::new(0x7000a7fbd0, fn_7000a7fbd0 as *const (), 0), // +0x393bd0 sub_393bd0
    Lifted::new(0x7000a7fca0, tpl_717::<0x7001c6abd8, 0x7000a83930, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x393ca0 sub_393ca0
    Lifted::new(0x7000a7fe40, fn_7000a7fe40 as *const (), 0), // +0x393e40 sub_393e40
    Lifted::new(0x7000a7ff00, tpl_717::<0x7001c6abd8, 0x7000a83130, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x393f00 sub_393f00
    Lifted::new(0x7000a800a0, tpl_717::<0x7001c6abd8, 0x7000a83200, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x3940a0 sub_3940a0
    Lifted::new(0x7000a80240, tpl_717::<0x7001c6abd8, 0x7000a832d0, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x394240 sub_394240
    Lifted::new(0x7000a803e0, tpl_717::<0x7001c6abd8, 0x7000a82f90, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x3943e0 sub_3943e0
    Lifted::new(0x7000a80580, tpl_717::<0x7001c6abd8, 0x7000a83060, 0x70006f9a50, 0x7000784c80, 0x7000a85230> as *const (), 2), // +0x394580 sub_394580
    Lifted::new(0x7000a80720, fn_7000a80720 as *const (), 0), // +0x394720 sub_394720
    Lifted::new(0x7000a807b0, fn_7000a807b0 as *const (), 0), // +0x3947b0 sub_3947b0
    Lifted::new(0x7000a80850, fn_7000a80850 as *const (), 0), // +0x394850 sub_394850
    Lifted::new(0x7000a80900, fn_7000a80900 as *const (), 0), // +0x394900 sub_394900
    Lifted::new(0x7000a809a0, fn_7000a809a0 as *const (), 0), // +0x3949a0 sub_3949a0
    Lifted::new(0x7000a80a50, fn_7000a80a50 as *const (), 0), // +0x394a50 sub_394a50
    Lifted::new(0x7000a80b10, fn_7000a80b10 as *const (), 0), // +0x394b10 sub_394b10
    Lifted::new(0x7000a80be0, fn_7000a80be0 as *const (), 0), // +0x394be0 sub_394be0
    Lifted::new(0x7000a80ca0, fn_7000a80ca0 as *const (), 0), // +0x394ca0 sub_394ca0
    Lifted::new(0x7000a80ce0, fn_7000a80ce0 as *const (), 0), // +0x394ce0 sub_394ce0
    Lifted::new(0x7000a80d20, fn_7000a80d20 as *const (), 0), // +0x394d20 sub_394d20
    Lifted::new(0x7000a80d60, fn_7000a80d60 as *const (), 2), // +0x394d60 sub_394d60
    Lifted::new(0x7000a80e60, tpl_726::<0x8, 0x10, 0x3d8, 0x700075f630, 0x70009fc7f0> as *const (), 0), // +0x394e60 sub_394e60
    Lifted::new(0x7000a80f00, tpl_61::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000a810d4, 0x7000a97110, 0x7000a810da, 0x70007f0910, 0x7000a810e0> as *const (), 0), // +0x394f00 sub_394f00
    Lifted::new(0x7000a81160, tpl_727::<0x7001779e20, 0x700177c312, 0x7001d54968, 0x7000a81338, 0x7000a97130, 0x7000a8133e, 0x70007f0910, 0x7000a81344> as *const (), 2), // +0x395160 sub_395160
    Lifted::new(0x7000a813c0, tpl_728::<0x7001779e20, 0x700177c0e0> as *const (), 0), // +0x3953c0 sub_3953c0
    Lifted::new(0x7000a81460, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395460 sub_395460
    Lifted::new(0x7000a81480, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395480 sub_395480
    Lifted::new(0x7000a814a0, tpl_729::<0x7001c55d20> as *const (), 1), // +0x3954a0 sub_3954a0
    Lifted::new(0x7000a814c0, tpl_729::<0x7001c55d20> as *const (), 1), // +0x3954c0 sub_3954c0
    Lifted::new(0x7000a814e0, tpl_729::<0x7001c55d20> as *const (), 1), // +0x3954e0 sub_3954e0
    Lifted::new(0x7000a81500, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395500 sub_395500
    Lifted::new(0x7000a81520, fn_7000a81520 as *const (), 0), // +0x395520 sub_395520
    Lifted::new(0x7000a818e0, tpl_729::<0x7001c55d20> as *const (), 1), // +0x3958e0 sub_3958e0
    Lifted::new(0x7000a81900, tpl_7::<0x7001c55e88> as *const (), 1), // +0x395900 sub_395900
    Lifted::new(0x7000a81920, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395920 sub_395920
    Lifted::new(0x7000a81940, tpl_7::<0x7001c55eb0> as *const (), 1), // +0x395940 sub_395940
    Lifted::new(0x7000a81960, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395960 sub_395960
    Lifted::new(0x7000a81980, tpl_70 as *const (), 1), // +0x395980 sub_395980
    Lifted::new(0x7000a81990, fn_7000a81990 as *const (), 1), // +0x395990 sub_395990
    Lifted::new(0x7000a819c0, tpl_14 as *const (), 1), // +0x3959c0 sub_3959c0
    Lifted::new(0x7000a819d0, fn_7000a819d0 as *const (), 0), // +0x3959d0 sub_3959d0
    Lifted::new(0x7000a81a10, fn_7000a81a10 as *const (), 0), // +0x395a10 sub_395a10
    Lifted::new(0x7000a81a50, fn_7000a81a50 as *const (), 0), // +0x395a50 sub_395a50
    Lifted::new(0x7000a81a90, fn_7000a81a90 as *const (), 0), // +0x395a90 sub_395a90
    Lifted::new(0x7000a81ac0, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395ac0 sub_395ac0
    Lifted::new(0x7000a81ae0, tpl_7::<0x7001c56518> as *const (), 1), // +0x395ae0 sub_395ae0
    Lifted::new(0x7000a81b00, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395b00 sub_395b00
    Lifted::new(0x7000a81b20, tpl_7::<0x7001c56540> as *const (), 1), // +0x395b20 sub_395b20
    Lifted::new(0x7000a81b40, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395b40 sub_395b40
    Lifted::new(0x7000a81b60, tpl_7::<0x7001c565b8> as *const (), 1), // +0x395b60 sub_395b60
    Lifted::new(0x7000a81b80, tpl_729::<0x7001c55d20> as *const (), 1), // +0x395b80 sub_395b80
    Lifted::new(0x7000a81ba0, tpl_7::<0x7001c56bf8> as *const (), 1), // +0x395ba0 sub_395ba0
    Lifted::new(0x7000a81bc0, tpl_28 as *const (), 1), // +0x395bc0 sub_395bc0
    Lifted::new(0x7000a81bd0, tpl_28 as *const (), 1), // +0x395bd0 sub_395bd0
    Lifted::new(0x7000a81be0, tpl_28 as *const (), 1), // +0x395be0 sub_395be0
    Lifted::new(0x7000a81bf0, tpl_28 as *const (), 1), // +0x395bf0 sub_395bf0
    Lifted::new(0x7000a81c00, tpl_28 as *const (), 1), // +0x395c00 sub_395c00
    Lifted::new(0x7000a81c10, tpl_28 as *const (), 1), // +0x395c10 sub_395c10
    Lifted::new(0x7000a81c20, tpl_28 as *const (), 1), // +0x395c20 sub_395c20
    Lifted::new(0x7000a81c30, tpl_28 as *const (), 1), // +0x395c30 sub_395c30
    Lifted::new(0x7000a81c40, tpl_28 as *const (), 1), // +0x395c40 sub_395c40
    Lifted::new(0x7000a81c50, tpl_28 as *const (), 1), // +0x395c50 sub_395c50
    Lifted::new(0x7000a81c60, tpl_28 as *const (), 1), // +0x395c60 sub_395c60
    Lifted::new(0x7000a81c70, tpl_28 as *const (), 1), // +0x395c70 sub_395c70
    Lifted::new(0x7000a81c80, tpl_28 as *const (), 1), // +0x395c80 sub_395c80
    Lifted::new(0x7000a81c90, tpl_28 as *const (), 1), // +0x395c90 sub_395c90
    Lifted::new(0x7000a81ca0, tpl_28 as *const (), 1), // +0x395ca0 sub_395ca0
    Lifted::new(0x7000a81cb0, tpl_28 as *const (), 1), // +0x395cb0 sub_395cb0
    Lifted::new(0x7000a81cc0, tpl_20 as *const (), 1), // +0x395cc0 sub_395cc0
    Lifted::new(0x7000a81cd0, tpl_20 as *const (), 1), // +0x395cd0 sub_395cd0
    Lifted::new(0x7000a81ce0, tpl_28 as *const (), 1), // +0x395ce0 sub_395ce0
    Lifted::new(0x7000a81cf0, tpl_20 as *const (), 1), // +0x395cf0 sub_395cf0
    Lifted::new(0x7000a81d00, tpl_20 as *const (), 1), // +0x395d00 sub_395d00
    Lifted::new(0x7000a81d10, tpl_28 as *const (), 1), // +0x395d10 sub_395d10
    Lifted::new(0x7000a81d20, tpl_28 as *const (), 1), // +0x395d20 sub_395d20
    Lifted::new(0x7000a81d30, tpl_28 as *const (), 1), // +0x395d30 sub_395d30
    Lifted::new(0x7000a81d40, tpl_28 as *const (), 1), // +0x395d40 sub_395d40
    Lifted::new(0x7000a81d50, tpl_28 as *const (), 1), // +0x395d50 sub_395d50
    Lifted::new(0x7000a81d60, tpl_28 as *const (), 1), // +0x395d60 sub_395d60
    Lifted::new(0x7000a81d70, tpl_28 as *const (), 1), // +0x395d70 sub_395d70
    Lifted::new(0x7000a81d80, tpl_28 as *const (), 1), // +0x395d80 sub_395d80
    Lifted::new(0x7000a81d90, tpl_730 as *const (), 1), // +0x395d90 sub_395d90
    Lifted::new(0x7000a81da0, tpl_177::<0x70017d2c10, 0x7001d54ef8, 0x7000a81db1> as *const (), 0), // +0x395da0 sub_395da0
    Lifted::new(0x7000a81dc0, fn_7000a81dc0 as *const (), 0), // +0x395dc0 sub_395dc0
    Lifted::new(0x7000a81e80, tpl_144 as *const (), 1), // +0x395e80 sub_395e80
    Lifted::new(0x7000a81e90, tpl_144 as *const (), 1), // +0x395e90 sub_395e90
    Lifted::new(0x7000a81ea0, tpl_144 as *const (), 1), // +0x395ea0 sub_395ea0
    Lifted::new(0x7000a81eb0, tpl_144 as *const (), 1), // +0x395eb0 sub_395eb0
    Lifted::new(0x7000a81ec0, tpl_144 as *const (), 1), // +0x395ec0 sub_395ec0
    Lifted::new(0x7000a81ed0, tpl_144 as *const (), 1), // +0x395ed0 sub_395ed0
    Lifted::new(0x7000a81ee0, fn_7000a81ee0 as *const (), 1), // +0x395ee0 sub_395ee0
    Lifted::new(0x7000a81ef0, fn_7000a81ef0 as *const (), 0), // +0x395ef0 sub_395ef0
    Lifted::new(0x7000a81f30, fn_7000a81f30 as *const (), 2), // +0x395f30 sub_395f30
    Lifted::new(0x7000a82080, fn_7000a82080 as *const (), 0), // +0x396080 sub_396080
    Lifted::new(0x7000a82130, fn_7000a82130 as *const (), 2), // +0x396130 sub_396130
    Lifted::new(0x7000a821d0, fn_7000a821d0 as *const (), 1), // +0x3961d0 sub_3961d0
    Lifted::new(0x7000a821e0, tpl_288 as *const (), 1), // +0x3961e0 sub_3961e0
    Lifted::new(0x7000a821f0, fn_7000a821f0 as *const (), 0), // +0x3961f0 sub_3961f0
    Lifted::new(0x7000a82290, tpl_15 as *const (), 1), // +0x396290 sub_396290
    Lifted::new(0x7000a822a0, tpl_731::<0x70007224a0> as *const (), 0), // +0x3962a0 sub_3962a0
    Lifted::new(0x7000a822d0, tpl_732 as *const (), 1), // +0x3962d0 sub_3962d0
    Lifted::new(0x7000a822e0, fn_7000a822e0 as *const (), 1), // +0x3962e0 sub_3962e0
    Lifted::new(0x7000a82330, fn_7000a82330 as *const (), 3), // +0x396330 sub_396330
    Lifted::new(0x7000a823a0, fn_7000a823a0 as *const (), 0), // +0x3963a0 sub_3963a0
    Lifted::new(0x7000a82460, fn_7000a82460 as *const (), 1), // +0x396460 sub_396460
    Lifted::new(0x7000a824a0, fn_7000a824a0 as *const (), 1), // +0x3964a0 sub_3964a0
    Lifted::new(0x7000a825c0, tpl_733::<0x7000751d80> as *const (), 0), // +0x3965c0 sub_3965c0
    Lifted::new(0x7000a82600, fn_7000a82600 as *const (), 2), // +0x396600 sub_396600
    Lifted::new(0x7000a82a90, fn_7000a82a90 as *const (), 1), // +0x396a90 sub_396a90
    Lifted::new(0x7000a82ac0, tpl_53 as *const (), 1), // +0x396ac0 sub_396ac0
    Lifted::new(0x7000a82ad0, tpl_53 as *const (), 1), // +0x396ad0 sub_396ad0
    Lifted::new(0x7000a82ae0, tpl_238 as *const (), 1), // +0x396ae0 sub_396ae0
    Lifted::new(0x7000a82af0, tpl_53 as *const (), 1), // +0x396af0 sub_396af0
    Lifted::new(0x7000a82b00, tpl_43 as *const (), 1), // +0x396b00 sub_396b00
    Lifted::new(0x7000a82b10, tpl_196 as *const (), 1), // +0x396b10 sub_396b10
    Lifted::new(0x7000a82b20, tpl_147::<0x8> as *const (), 1), // +0x396b20 sub_396b20
    Lifted::new(0x7000a82b30, fn_7000a82b30 as *const (), 1), // +0x396b30 sub_396b30
    Lifted::new(0x7000a82b60, tpl_734::<0x7001c6abd8> as *const (), 3), // +0x396b60 sub_396b60
    Lifted::new(0x7000a82be0, tpl_734::<0x7001c6abd8> as *const (), 3), // +0x396be0 sub_396be0
    Lifted::new(0x7000a82c60, tpl_734::<0x7001c6abd8> as *const (), 3), // +0x396c60 sub_396c60
    Lifted::new(0x7000a82ce0, tpl_734::<0x7001c6abd8> as *const (), 3), // +0x396ce0 sub_396ce0
    Lifted::new(0x7000a82d60, tpl_734::<0x7001c6abd8> as *const (), 3), // +0x396d60 sub_396d60
    Lifted::new(0x7000a82de0, tpl_734::<0x7001c6abd8> as *const (), 3), // +0x396de0 sub_396de0
    Lifted::new(0x7000a82e60, tpl_734::<0x7001c6abd8> as *const (), 3), // +0x396e60 sub_396e60
    Lifted::new(0x7000a82ee0, fn_7000a82ee0 as *const (), 3), // +0x396ee0 sub_396ee0
    Lifted::new(0x7000a82f50, fn_7000a82f50 as *const (), 0), // +0x396f50 sub_396f50
    Lifted::new(0x7000a82f70, fn_7000a82f70 as *const (), 1), // +0x396f70 sub_396f70
    Lifted::new(0x7000a82f90, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x396f90 sub_396f90
    Lifted::new(0x7000a83060, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397060 sub_397060
    Lifted::new(0x7000a83130, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397130 sub_397130
    Lifted::new(0x7000a83200, tpl_736::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397200 sub_397200
    Lifted::new(0x7000a832d0, tpl_736::<0x7001c6abd8, 0x4> as *const (), 1), // +0x3972d0 sub_3972d0
    Lifted::new(0x7000a833a0, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x3973a0 sub_3973a0
    Lifted::new(0x7000a83470, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397470 sub_397470
    Lifted::new(0x7000a83540, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397540 sub_397540
    Lifted::new(0x7000a83610, tpl_736::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397610 sub_397610
    Lifted::new(0x7000a836e0, tpl_736::<0x7001c6abd8, 0x4> as *const (), 1), // +0x3976e0 sub_3976e0
    Lifted::new(0x7000a837b0, tpl_737::<0x7001c6abd8, 0x4> as *const (), 1), // +0x3977b0 sub_3977b0
    Lifted::new(0x7000a83870, tpl_737::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397870 sub_397870
    Lifted::new(0x7000a83930, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397930 sub_397930
    Lifted::new(0x7000a83a00, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397a00 sub_397a00
    Lifted::new(0x7000a83ad0, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397ad0 sub_397ad0
    Lifted::new(0x7000a83ba0, tpl_735::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397ba0 sub_397ba0
    Lifted::new(0x7000a83c70, tpl_737::<0x7001c6abd8, 0x4> as *const (), 1), // +0x397c70 sub_397c70
    Lifted::new(0x7000a83d30, fn_7000a83d30 as *const (), 1), // +0x397d30 sub_397d30
    Lifted::new(0x7000a83e00, fn_7000a83e00 as *const (), 1), // +0x397e00 sub_397e00
    Lifted::new(0x7000a83e70, tpl_738 as *const (), 1), // +0x397e70 sub_397e70
    Lifted::new(0x7000a83e80, tpl_238 as *const (), 1), // +0x397e80 sub_397e80
    Lifted::new(0x7000a83e90, tpl_53 as *const (), 1), // +0x397e90 sub_397e90
    Lifted::new(0x7000a83ea0, tpl_53 as *const (), 1), // +0x397ea0 sub_397ea0
    Lifted::new(0x7000a83eb0, tpl_53 as *const (), 1), // +0x397eb0 sub_397eb0
    Lifted::new(0x7000a83ec0, tpl_53 as *const (), 1), // +0x397ec0 sub_397ec0
    Lifted::new(0x7000a83ed0, tpl_53 as *const (), 1), // +0x397ed0 sub_397ed0
    Lifted::new(0x7000a83ee0, tpl_147::<0x4> as *const (), 1), // +0x397ee0 sub_397ee0
    Lifted::new(0x7000a83ef0, fn_7000a83ef0 as *const (), 1), // +0x397ef0 sub_397ef0
    Lifted::new(0x7000a83f10, tpl_147::<0x10> as *const (), 1), // +0x397f10 sub_397f10
    Lifted::new(0x7000a83f20, tpl_739::<0x7001c6abd8, 0x2> as *const (), 1), // +0x397f20 sub_397f20
    Lifted::new(0x7000a83ff0, tpl_739::<0x7001c6abd8, 0x2> as *const (), 1), // +0x397ff0 sub_397ff0
    Lifted::new(0x7000a840c0, fn_7000a840c0 as *const (), 0), // +0x3980c0 sub_3980c0
    Lifted::new(0x7000a840e0, tpl_739::<0x7001c6abd8, 0x2> as *const (), 1), // +0x3980e0 sub_3980e0
    Lifted::new(0x7000a841b0, fn_7000a841b0 as *const (), 0), // +0x3981b0 sub_3981b0
    Lifted::new(0x7000a841d0, fn_7000a841d0 as *const (), 1), // +0x3981d0 sub_3981d0
    Lifted::new(0x7000a84250, fn_7000a84250 as *const (), 1), // +0x398250 sub_398250
    Lifted::new(0x7000a84260, tpl_44 as *const (), 1), // +0x398260 sub_398260
    Lifted::new(0x7000a84270, tpl_44 as *const (), 1), // +0x398270 sub_398270
    Lifted::new(0x7000a84280, tpl_144 as *const (), 1), // +0x398280 sub_398280
    Lifted::new(0x7000a84290, tpl_144 as *const (), 1), // +0x398290 sub_398290
    Lifted::new(0x7000a842a0, tpl_44 as *const (), 1), // +0x3982a0 sub_3982a0
    Lifted::new(0x7000a842b0, tpl_44 as *const (), 1), // +0x3982b0 sub_3982b0
    Lifted::new(0x7000a842c0, fn_7000a842c0 as *const (), 0), // +0x3982c0 sub_3982c0
    Lifted::new(0x7000a842e0, tpl_733::<0x70007a46f0> as *const (), 0), // +0x3982e0 sub_3982e0
    Lifted::new(0x7000a84320, tpl_740::<0x70007a46f0> as *const (), 0), // +0x398320 sub_398320
    Lifted::new(0x7000a84360, fn_7000a84360 as *const (), 0), // +0x398360 sub_398360
    Lifted::new(0x7000a84400, fn_7000a84400 as *const (), 0), // +0x398400 sub_398400
    Lifted::new(0x7000a84470, fn_7000a84470 as *const (), 0), // +0x398470 sub_398470
    Lifted::new(0x7000a84510, fn_7000a84510 as *const (), 0), // +0x398510 sub_398510
    Lifted::new(0x7000a84580, fn_7000a84580 as *const (), 0), // +0x398580 sub_398580
    Lifted::new(0x7000a84630, fn_7000a84630 as *const (), 0), // +0x398630 sub_398630
    Lifted::new(0x7000a846f0, tpl_32::<0x7000707c70> as *const (), 0), // +0x3986f0 sub_3986f0
    Lifted::new(0x7000a84700, tpl_32::<0x7000707c70> as *const (), 0), // +0x398700 sub_398700
    Lifted::new(0x7000a84710, tpl_32::<0x7000707c70> as *const (), 0), // +0x398710 sub_398710
    Lifted::new(0x7000a84720, tpl_32::<0x7000707c70> as *const (), 0), // +0x398720 sub_398720
    Lifted::new(0x7000a84730, tpl_32::<0x7000707c70> as *const (), 0), // +0x398730 sub_398730
    Lifted::new(0x7000a84740, tpl_32::<0x7000707c70> as *const (), 0), // +0x398740 sub_398740
    Lifted::new(0x7000a84750, fn_7000a84750 as *const (), 0), // +0x398750 sub_398750
    Lifted::new(0x7000a847b0, fn_7000a847b0 as *const (), 0), // +0x3987b0 sub_3987b0
    Lifted::new(0x7000a847e0, fn_7000a847e0 as *const (), 0), // +0x3987e0 sub_3987e0
    Lifted::new(0x7000a84840, fn_7000a84840 as *const (), 0), // +0x398840 sub_398840
    Lifted::new(0x7000a84870, fn_7000a84870 as *const (), 0), // +0x398870 sub_398870
    Lifted::new(0x7000a848d0, fn_7000a848d0 as *const (), 0), // +0x3988d0 sub_3988d0
    Lifted::new(0x7000a84900, tpl_741::<0x700073c720> as *const (), 0), // +0x398900 sub_398900
    Lifted::new(0x7000a84950, fn_7000a84950 as *const (), 0), // +0x398950 sub_398950
    Lifted::new(0x7000a84980, tpl_741::<0x700078d260> as *const (), 0), // +0x398980 sub_398980
    Lifted::new(0x7000a849d0, tpl_741::<0x70006fa410> as *const (), 0), // +0x3989d0 sub_3989d0
];
