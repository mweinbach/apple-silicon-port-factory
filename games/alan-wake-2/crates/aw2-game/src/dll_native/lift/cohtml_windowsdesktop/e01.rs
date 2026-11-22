#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::new(0x70002a1ce0, fn_70002a1ce0 as *const (), 0), // +0x2a1ce0 sub_2a1ce0
    Lifted::new(0x70002a1dc0, fn_70002a1dc0 as *const (), 0), // +0x2a1dc0 sub_2a1dc0
    Lifted::new(0x70002a1e70, fn_70002a1e70 as *const (), 2), // +0x2a1e70 sub_2a1e70
    Lifted::new(0x70002a2030, fn_70002a2030 as *const (), 0), // +0x2a2030 sub_2a2030
    Lifted::new(0x70002a2150, fn_70002a2150 as *const (), 0), // +0x2a2150 sub_2a2150
    Lifted::new(0x70002a2230, fn_70002a2230 as *const (), 2), // +0x2a2230 sub_2a2230
    Lifted::new(0x70002a2340, fn_70002a2340 as *const (), 2), // +0x2a2340 sub_2a2340
    Lifted::new(0x70002a2510, fn_70002a2510 as *const (), 2), // +0x2a2510 sub_2a2510
    Lifted::new(0x70002a26a0, fn_70002a26a0 as *const (), 2), // +0x2a26a0 sub_2a26a0
    Lifted::new(0x70002a2960, fn_70002a2960 as *const (), 0), // +0x2a2960 sub_2a2960
    Lifted::new(0x70002a29c0, fn_70002a29c0 as *const (), 0), // +0x2a29c0 sub_2a29c0
    Lifted::new(0x70002a2b80, fn_70002a2b80 as *const (), 0), // +0x2a2b80 sub_2a2b80
    Lifted::new(0x70002a2c20, fn_70002a2c20 as *const (), 0), // +0x2a2c20 sub_2a2c20
    Lifted::new(0x70002a2c70, fn_70002a2c70 as *const (), 0), // +0x2a2c70 sub_2a2c70
    Lifted::new(0x70002a2cd0, fn_70002a2cd0 as *const (), 0), // +0x2a2cd0 sub_2a2cd0
    Lifted::new(0x70002a2f30, fn_70002a2f30 as *const (), 0), // +0x2a2f30 sub_2a2f30
    Lifted::new(0x70002a3090, fn_70002a3090 as *const (), 0), // +0x2a3090 sub_2a3090
    Lifted::new(0x70002a3250, fn_70002a3250 as *const (), 0), // +0x2a3250 sub_2a3250
    Lifted::new(0x70002a33b0, fn_70002a33b0 as *const (), 0), // +0x2a33b0 sub_2a33b0
    Lifted::new(0x70002a3490, fn_70002a3490 as *const (), 0), // +0x2a3490 sub_2a3490
    Lifted::new(0x70002a35f0, fn_70002a35f0 as *const (), 0), // +0x2a35f0 sub_2a35f0
    Lifted::new(0x70002a3750, fn_70002a3750 as *const (), 0), // +0x2a3750 sub_2a3750
    Lifted::new(0x70002a37b0, fn_70002a37b0 as *const (), 0), // +0x2a37b0 sub_2a37b0
    Lifted::new(0x70002a3840, fn_70002a3840 as *const (), 0), // +0x2a3840 sub_2a3840
    Lifted::new(0x70002a38b0, fn_70002a38b0 as *const (), 0), // +0x2a38b0 sub_2a38b0
    Lifted::new(0x70002a3920, fn_70002a3920 as *const (), 2), // +0x2a3920 sub_2a3920
    Lifted::new(0x70002a3af0, tpl_189::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x70002a3be0> as *const (), 2), // +0x2a3af0 sub_2a3af0
    Lifted::new(0x70002a3be0, fn_70002a3be0 as *const (), 0), // +0x2a3be0 sub_2a3be0
    Lifted::new(0x70002a3c90, tpl_225::<0x8, 0x18, 0x60> as *const (), 0), // +0x2a3c90 sub_2a3c90
    Lifted::new(0x70002a3cd0, fn_70002a3cd0 as *const (), 0), // +0x2a3cd0 sub_2a3cd0
    Lifted::new(0x70002a3d90, fn_70002a3d90 as *const (), 0), // +0x2a3d90 sub_2a3d90
    Lifted::new(0x70002a3dd0, fn_70002a3dd0 as *const (), 2), // +0x2a3dd0 sub_2a3dd0
    Lifted::new(0x70002a4080, fn_70002a4080 as *const (), 0), // +0x2a4080 sub_2a4080
    Lifted::new(0x70002a4300, fn_70002a4300 as *const (), 0), // +0x2a4300 sub_2a4300
    Lifted::new(0x70002a4670, fn_70002a4670 as *const (), 2), // +0x2a4670 sub_2a4670
    Lifted::new(0x70002a4c00, fn_70002a4c00 as *const (), 0), // +0x2a4c00 sub_2a4c00
    Lifted::new(0x70002a4c80, fn_70002a4c80 as *const (), 0), // +0x2a4c80 sub_2a4c80
    Lifted::new(0x70002a59e0, fn_70002a59e0 as *const (), 2), // +0x2a59e0 sub_2a59e0
    Lifted::new(0x70002a6290, fn_70002a6290 as *const (), 0), // +0x2a6290 sub_2a6290
    Lifted::new(0x70002a64b0, fn_70002a64b0 as *const (), 0), // +0x2a64b0 sub_2a64b0
    Lifted::new(0x70002a66d0, fn_70002a66d0 as *const (), 0), // +0x2a66d0 sub_2a66d0
    Lifted::new(0x70002a6800, fn_70002a6800 as *const (), 0), // +0x2a6800 sub_2a6800
    Lifted::new(0x70002a6990, fn_70002a6990 as *const (), 0), // +0x2a6990 sub_2a6990
    Lifted::new(0x70002a6d00, fn_70002a6d00 as *const (), 0), // +0x2a6d00 sub_2a6d00
    Lifted::new(0x70002a6e70, fn_70002a6e70 as *const (), 2), // +0x2a6e70 sub_2a6e70
    Lifted::new(0x70002a7010, fn_70002a7010 as *const (), 0), // +0x2a7010 sub_2a7010
    Lifted::new(0x70002a7190, fn_70002a7190 as *const (), 0), // +0x2a7190 sub_2a7190
    Lifted::new(0x70002a7410, tpl_199::<0x700024bc60, 0x70002a7490> as *const (), 0), // +0x2a7410 sub_2a7410
    Lifted::new(0x70002a7430, fn_70002a7430 as *const (), 0), // +0x2a7430 sub_2a7430
    Lifted::new(0x70002a7490, fn_70002a7490 as *const (), 2), // +0x2a7490 sub_2a7490
    Lifted::new(0x70002a7760, tpl_197::<0x1d0, 0x1d1, 0x1e0, 0x70003d9710, 0x70002a7490> as *const (), 0), // +0x2a7760 sub_2a7760
    Lifted::new(0x70002a77a0, fn_70002a77a0 as *const (), 0), // +0x2a77a0 sub_2a77a0
    Lifted::new(0x70002a7a70, fn_70002a7a70 as *const (), 0), // +0x2a7a70 sub_2a7a70
    Lifted::new(0x70002a7b60, tpl_217::<0x138, 0xf8, 0xf9, 0x108> as *const (), 1), // +0x2a7b60 sub_2a7b60
    Lifted::new(0x70002a7bd0, fn_70002a7bd0 as *const (), 2), // +0x2a7bd0 sub_2a7bd0
    Lifted::new(0x70002a7d20, fn_70002a7d20 as *const (), 0), // +0x2a7d20 sub_2a7d20
    Lifted::new(0x70002a7e70, tpl_218::<0x1a0, 0x8, 0x198, 0x19c, 0x4> as *const (), 1), // +0x2a7e70 sub_2a7e70
    Lifted::new(0x70002a7e90, fn_70002a7e90 as *const (), 0), // +0x2a7e90 sub_2a7e90
    Lifted::new(0x70002a7ee0, fn_70002a7ee0 as *const (), 0), // +0x2a7ee0 sub_2a7ee0
    Lifted::new(0x70002a80a0, tpl_279::<0x240, 0x1d0, 0x7000255b50, 0x70002a8150> as *const (), 0), // +0x2a80a0 sub_2a80a0
    Lifted::new(0x70002a80e0, tpl_279::<0x240, 0x1d0, 0x7000255da0, 0x70002a8150> as *const (), 0), // +0x2a80e0 sub_2a80e0
    Lifted::new(0x70002a8120, fn_70002a8120 as *const (), 0), // +0x2a8120 sub_2a8120
    Lifted::new(0x70002a8150, fn_70002a8150 as *const (), 0), // +0x2a8150 sub_2a8150
    Lifted::new(0x70002a8200, fn_70002a8200 as *const (), 0), // +0x2a8200 sub_2a8200
    Lifted::new(0x70002a8240, fn_70002a8240 as *const (), 0), // +0x2a8240 sub_2a8240
    Lifted::new(0x70002a82c0, fn_70002a82c0 as *const (), 0), // +0x2a82c0 sub_2a82c0
    Lifted::new(0x70002a8300, fn_70002a8300 as *const (), 2), // +0x2a8300 sub_2a8300
    Lifted::new(0x70002a8380, fn_70002a8380 as *const (), 0), // +0x2a8380 sub_2a8380
    Lifted::new(0x70002a83e0, fn_70002a83e0 as *const (), 0), // +0x2a83e0 sub_2a83e0
    Lifted::new(0x70002a84a0, fn_70002a84a0 as *const (), 0), // +0x2a84a0 sub_2a84a0
    Lifted::new(0x70002a8550, fn_70002a8550 as *const (), 0), // +0x2a8550 sub_2a8550
    Lifted::new(0x70002a89c0, fn_70002a89c0 as *const (), 0), // +0x2a89c0 sub_2a89c0
    Lifted::new(0x70002a8a30, fn_70002a8a30 as *const (), 0), // +0x2a8a30 sub_2a8a30
    Lifted::new(0x70002a8b10, fn_70002a8b10 as *const (), 0), // +0x2a8b10 sub_2a8b10
    Lifted::new(0x70002a8c60, fn_70002a8c60 as *const (), 0), // +0x2a8c60 sub_2a8c60
    Lifted::new(0x70002a8ca0, tpl_224::<0x1d0, 0x70001a4530> as *const (), 0), // +0x2a8ca0 sub_2a8ca0
    Lifted::new(0x70002a8cd0, fn_70002a8cd0 as *const (), 2), // +0x2a8cd0 sub_2a8cd0
    Lifted::new(0x70002a8d20, fn_70002a8d20 as *const (), 2), // +0x2a8d20 sub_2a8d20
    Lifted::new(0x70002a8f20, fn_70002a8f20 as *const (), 2), // +0x2a8f20 sub_2a8f20
    Lifted::new(0x70002a90c0, fn_70002a90c0 as *const (), 2), // +0x2a90c0 sub_2a90c0
    Lifted::new(0x70002a9310, fn_70002a9310 as *const (), 0), // +0x2a9310 sub_2a9310
    Lifted::new(0x70002a93b0, fn_70002a93b0 as *const (), 0), // +0x2a93b0 sub_2a93b0
    Lifted::new(0x70002a9484, tpl_185::<0x70002a7010> as *const (), 0), // +0x2a9484 sub_2a9484
    Lifted::new(0x70002a9490, fn_70002a9490 as *const (), 0), // +0x2a9490 sub_2a9490
    Lifted::new(0x70002a9ca0, fn_70002a9ca0 as *const (), 0), // +0x2a9ca0 sub_2a9ca0
    Lifted::new(0x70002a9e50, fn_70002a9e50 as *const (), 0), // +0x2a9e50 sub_2a9e50
    Lifted::new(0x70002a9f50, fn_70002a9f50 as *const (), 0), // +0x2a9f50 sub_2a9f50
    Lifted::new(0x70002aa080, tpl_180::<0x8, 0x18, 0x88> as *const (), 0), // +0x2aa080 sub_2aa080
    Lifted::new(0x70002aa0c0, fn_70002aa0c0 as *const (), 0), // +0x2aa0c0 sub_2aa0c0
    Lifted::new(0x70002aa3e0, fn_70002aa3e0 as *const (), 0), // +0x2aa3e0 sub_2aa3e0
    Lifted::new(0x70002aa420, fn_70002aa420 as *const (), 2), // +0x2aa420 sub_2aa420
    Lifted::new(0x70002aa4d0, fn_70002aa4d0 as *const (), 2), // +0x2aa4d0 sub_2aa4d0
    Lifted::new(0x70002aa620, fn_70002aa620 as *const (), 0), // +0x2aa620 sub_2aa620
    Lifted::new(0x70002aa680, fn_70002aa680 as *const (), 2), // +0x2aa680 sub_2aa680
    Lifted::new(0x70002aa950, fn_70002aa950 as *const (), 2), // +0x2aa950 sub_2aa950
    Lifted::new(0x70002aacd0, fn_70002aacd0 as *const (), 0), // +0x2aacd0 sub_2aacd0
    Lifted::new(0x70002aacf0, fn_70002aacf0 as *const (), 0), // +0x2aacf0 sub_2aacf0
    Lifted::new(0x70002aade0, fn_70002aade0 as *const (), 0), // +0x2aade0 sub_2aade0
    Lifted::new(0x70002aae00, fn_70002aae00 as *const (), 0), // +0x2aae00 sub_2aae00
    Lifted::new(0x70002ab0e0, fn_70002ab0e0 as *const (), 0), // +0x2ab0e0 sub_2ab0e0
    Lifted::new(0x70002ab220, fn_70002ab220 as *const (), 2), // +0x2ab220 sub_2ab220
    Lifted::new(0x70002ab3d0, fn_70002ab3d0 as *const (), 2), // +0x2ab3d0 sub_2ab3d0
    Lifted::new(0x70002ab490, fn_70002ab490 as *const (), 2), // +0x2ab490 sub_2ab490
    Lifted::new(0x70002ab700, fn_70002ab700 as *const (), 0), // +0x2ab700 sub_2ab700
    Lifted::new(0x70002ab860, fn_70002ab860 as *const (), 0), // +0x2ab860 sub_2ab860
    Lifted::new(0x70002aba10, fn_70002aba10 as *const (), 0), // +0x2aba10 sub_2aba10
    Lifted::new(0x70002abb10, tpl_177::<0x58, 0x700039fda0> as *const (), 0), // +0x2abb10 sub_2abb10
    Lifted::new(0x70002abb20, tpl_280::<0x58, 0x70006974b8, 0x70005fe558, 0x70005f84a0, 0x7000130980, 0x3010> as *const (), 0), // +0x2abb20 sub_2abb20
    Lifted::new(0x70002abb80, fn_70002abb80 as *const (), 2), // +0x2abb80 sub_2abb80
    Lifted::new(0x70002abdb0, tpl_281::<0x8, 0x19, 0x10, 0x7000256eb0, 0x70002abed0, 0x7000697150> as *const (), 0), // +0x2abdb0 sub_2abdb0
    Lifted::new(0x70002abe50, fn_70002abe50 as *const (), 0), // +0x2abe50 sub_2abe50
    Lifted::new(0x70002abed0, tpl_25::<0x7000694974, 0x700069a858, 0x70005a4174, 0x70005a6180, 0x700069a860, 0x700069a878, 0x700069a890, 0x700069a8a8, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2abed0 sub_2abed0
    Lifted::new(0x70002abf50, tpl_29::<0x18, 0x8, 0x70002ac150> as *const (), 0), // +0x2abf50 sub_2abf50
    Lifted::new(0x70002abfe0, fn_70002abfe0 as *const (), 2), // +0x2abfe0 sub_2abfe0
    Lifted::new(0x70002ac150, fn_70002ac150 as *const (), 2), // +0x2ac150 sub_2ac150
    Lifted::new(0x70002ac400, fn_70002ac400 as *const (), 2), // +0x2ac400 sub_2ac400
    Lifted::new(0x70002ac6c0, fn_70002ac6c0 as *const (), 0), // +0x2ac6c0 sub_2ac6c0
    Lifted::new(0x70002ac700, tpl_134::<0x8, 0x19, 0x10, 0xfffffffffffffff8, 0x7000697150, 0x7000245240> as *const (), 0), // +0x2ac700 sub_2ac700
    Lifted::new(0x70002ac7c0, fn_70002ac7c0 as *const (), 0), // +0x2ac7c0 sub_2ac7c0
    Lifted::new(0x70002ac800, fn_70002ac800 as *const (), 0), // +0x2ac800 sub_2ac800
    Lifted::new(0x70002ac940, fn_70002ac940 as *const (), 0), // +0x2ac940 sub_2ac940
    Lifted::new(0x70002acb70, fn_70002acb70 as *const (), 0), // +0x2acb70 sub_2acb70
    Lifted::new(0x70002accf0, fn_70002accf0 as *const (), 0), // +0x2accf0 sub_2accf0
    Lifted::new(0x70002acd70, fn_70002acd70 as *const (), 0), // +0x2acd70 sub_2acd70
    Lifted::new(0x70002ace70, fn_70002ace70 as *const (), 0), // +0x2ace70 sub_2ace70
    Lifted::new(0x70002ad0b0, fn_70002ad0b0 as *const (), 0), // +0x2ad0b0 sub_2ad0b0
    Lifted::new(0x70002ad240, fn_70002ad240 as *const (), 0), // +0x2ad240 sub_2ad240
    Lifted::new(0x70002ad2c0, fn_70002ad2c0 as *const (), 2), // +0x2ad2c0 sub_2ad2c0
    Lifted::new(0x70002ad3d0, fn_70002ad3d0 as *const (), 2), // +0x2ad3d0 sub_2ad3d0
    Lifted::new(0x70002ad500, fn_70002ad500 as *const (), 0), // +0x2ad500 sub_2ad500
    Lifted::new(0x70002ad530, fn_70002ad530 as *const (), 2), // +0x2ad530 sub_2ad530
    Lifted::new(0x70002ad690, fn_70002ad690 as *const (), 0), // +0x2ad690 sub_2ad690
    Lifted::new(0x70002ad790, fn_70002ad790 as *const (), 0), // +0x2ad790 sub_2ad790
    Lifted::new(0x70002ad870, fn_70002ad870 as *const (), 2), // +0x2ad870 sub_2ad870
    Lifted::new(0x70002ae1b0, fn_70002ae1b0 as *const (), 2), // +0x2ae1b0 sub_2ae1b0
    Lifted::new(0x70002ae3d0, fn_70002ae3d0 as *const (), 0), // +0x2ae3d0 sub_2ae3d0
    Lifted::new(0x70002ae4d0, fn_70002ae4d0 as *const (), 0), // +0x2ae4d0 sub_2ae4d0
    Lifted::new(0x70002ae6d0, fn_70002ae6d0 as *const (), 2), // +0x2ae6d0 sub_2ae6d0
    Lifted::new(0x70002aeba0, fn_70002aeba0 as *const (), 0), // +0x2aeba0 sub_2aeba0
    Lifted::new(0x70002aeed0, fn_70002aeed0 as *const (), 0), // +0x2aeed0 sub_2aeed0
    Lifted::new(0x70002aef60, tpl_282::<0x70002af610, 0x8, 0x70002af670, 0xffffffffffffffff, 0x7000697150, 0x10> as *const (), 0), // +0x2aef60 sub_2aef60
    Lifted::new(0x70002aefc0, fn_70002aefc0 as *const (), 0), // +0x2aefc0 sub_2aefc0
    Lifted::new(0x70002af020, fn_70002af020 as *const (), 2), // +0x2af020 sub_2af020
    Lifted::new(0x70002af1e0, fn_70002af1e0 as *const (), 0), // +0x2af1e0 sub_2af1e0
    Lifted::new(0x70002af280, fn_70002af280 as *const (), 0), // +0x2af280 sub_2af280
    Lifted::new(0x70002af310, fn_70002af310 as *const (), 0), // +0x2af310 sub_2af310
    Lifted::new(0x70002af540, fn_70002af540 as *const (), 0), // +0x2af540 sub_2af540
    Lifted::new(0x70002af550, fn_70002af550 as *const (), 0), // +0x2af550 sub_2af550
    Lifted::new(0x70002af5b0, fn_70002af5b0 as *const (), 0), // +0x2af5b0 sub_2af5b0
    Lifted::new(0x70002af610, tpl_283::<0xffffffffffffffff, 0x70002af850, 0x7000697150, 0x10> as *const (), 0), // +0x2af610 sub_2af610
    Lifted::new(0x70002af670, tpl_283::<0xffffffffffffffff, 0x7000237760, 0x7000697150, 0x10> as *const (), 0), // +0x2af670 sub_2af670
    Lifted::new(0x70002af6d0, fn_70002af6d0 as *const (), 1), // +0x2af6d0 sub_2af6d0
    Lifted::new(0x70002af770, fn_70002af770 as *const (), 0), // +0x2af770 sub_2af770
    Lifted::new(0x70002af7d0, tpl_202::<0x7000609cf0, 0x70002aef60, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x2af7d0 sub_2af7d0
    Lifted::new(0x70002af850, fn_70002af850 as *const (), 0), // +0x2af850 sub_2af850
    Lifted::new(0x70002af8c0, fn_70002af8c0 as *const (), 2), // +0x2af8c0 sub_2af8c0
    Lifted::new(0x70002afa10, tpl_212::<0x70002aeba0> as *const (), 0), // +0x2afa10 sub_2afa10
    Lifted::new(0x70002afa20, fn_70002afa20 as *const (), 2), // +0x2afa20 sub_2afa20
    Lifted::new(0x70002afb40, fn_70002afb40 as *const (), 2), // +0x2afb40 sub_2afb40
    Lifted::new(0x70002afce0, fn_70002afce0 as *const (), 0), // +0x2afce0 sub_2afce0
    Lifted::new(0x70002aff80, fn_70002aff80 as *const (), 0), // +0x2aff80 sub_2aff80
    Lifted::new(0x70002b0130, fn_70002b0130 as *const (), 0), // +0x2b0130 sub_2b0130
    Lifted::new(0x70002b0300, fn_70002b0300 as *const (), 0), // +0x2b0300 sub_2b0300
    Lifted::new(0x70002b0370, fn_70002b0370 as *const (), 0), // +0x2b0370 sub_2b0370
    Lifted::new(0x70002b0600, fn_70002b0600 as *const (), 0), // +0x2b0600 sub_2b0600
    Lifted::new(0x70002b0670, fn_70002b0670 as *const (), 0), // +0x2b0670 sub_2b0670
    Lifted::new(0x70002b0860, fn_70002b0860 as *const (), 0), // +0x2b0860 sub_2b0860
    Lifted::new(0x70002b08f0, fn_70002b08f0 as *const (), 0), // +0x2b08f0 sub_2b08f0
    Lifted::new(0x70002b0c50, tpl_284::<0x8, 0x70001e4150, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x2b0c50 sub_2b0c50
    Lifted::new(0x70002b0cc0, fn_70002b0cc0 as *const (), 0), // +0x2b0cc0 sub_2b0cc0
    Lifted::new(0x70002b0d40, fn_70002b0d40 as *const (), 2), // +0x2b0d40 sub_2b0d40
    Lifted::new(0x70002b0e50, tpl_39::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70002b0ee0> as *const (), 2), // +0x2b0e50 sub_2b0e50
    Lifted::new(0x70002b0ee0, fn_70002b0ee0 as *const (), 0), // +0x2b0ee0 sub_2b0ee0
    Lifted::new(0x70002b0fc0, fn_70002b0fc0 as *const (), 2), // +0x2b0fc0 sub_2b0fc0
    Lifted::new(0x70002b10b0, tpl_285::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x70002b11a0> as *const (), 2), // +0x2b10b0 sub_2b10b0
    Lifted::new(0x70002b11a0, tpl_284::<0x8, 0x70002b1c60, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x2b11a0 sub_2b11a0
    Lifted::new(0x70002b1210, fn_70002b1210 as *const (), 0), // +0x2b1210 sub_2b1210
    Lifted::new(0x70002b1350, fn_70002b1350 as *const (), 0), // +0x2b1350 sub_2b1350
    Lifted::new(0x70002b1400, fn_70002b1400 as *const (), 0), // +0x2b1400 sub_2b1400
    Lifted::new(0x70002b1480, fn_70002b1480 as *const (), 0), // +0x2b1480 sub_2b1480
    Lifted::new(0x70002b14e0, tpl_23::<0x7000697150, 0x8, 0x70002a0b20, 0xc> as *const (), 0), // +0x2b14e0 sub_2b14e0
    Lifted::new(0x70002b1570, tpl_39::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70002b14e0> as *const (), 2), // +0x2b1570 sub_2b1570
    Lifted::new(0x70002b1600, fn_70002b1600 as *const (), 0), // +0x2b1600 sub_2b1600
    Lifted::new(0x70002b1870, tpl_23::<0x7000697150, 0x8, 0x70002b1bc0, 0xc> as *const (), 0), // +0x2b1870 sub_2b1870
    Lifted::new(0x70002b1900, tpl_39::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70002b1870> as *const (), 2), // +0x2b1900 sub_2b1900
    Lifted::new(0x70002b1990, fn_70002b1990 as *const (), 2), // +0x2b1990 sub_2b1990
    Lifted::new(0x70002b1bc0, fn_70002b1bc0 as *const (), 0), // +0x2b1bc0 sub_2b1bc0
    Lifted::new(0x70002b1c60, fn_70002b1c60 as *const (), 0), // +0x2b1c60 sub_2b1c60
    Lifted::new(0x70002b1d40, fn_70002b1d40 as *const (), 0), // +0x2b1d40 sub_2b1d40
    Lifted::new(0x70002b2120, fn_70002b2120 as *const (), 0), // +0x2b2120 sub_2b2120
    Lifted::new(0x70002b28a0, fn_70002b28a0 as *const (), 0), // +0x2b28a0 sub_2b28a0
    Lifted::new(0x70002b2ba0, fn_70002b2ba0 as *const (), 0), // +0x2b2ba0 sub_2b2ba0
    Lifted::new(0x70002b2ec0, fn_70002b2ec0 as *const (), 2), // +0x2b2ec0 sub_2b2ec0
    Lifted::new(0x70002b2fa0, fn_70002b2fa0 as *const (), 2), // +0x2b2fa0 sub_2b2fa0
    Lifted::new(0x70002b30e0, fn_70002b30e0 as *const (), 0), // +0x2b30e0 sub_2b30e0
    Lifted::new(0x70002b3350, fn_70002b3350 as *const (), 0), // +0x2b3350 sub_2b3350
    Lifted::new(0x70002b3450, fn_70002b3450 as *const (), 2), // +0x2b3450 sub_2b3450
    Lifted::new(0x70002b36f0, fn_70002b36f0 as *const (), 0), // +0x2b36f0 sub_2b36f0
    Lifted::new(0x70002b38a0, fn_70002b38a0 as *const (), 0), // +0x2b38a0 sub_2b38a0
    Lifted::new(0x70002b39f0, fn_70002b39f0 as *const (), 0), // +0x2b39f0 sub_2b39f0
    Lifted::new(0x70002b3a50, fn_70002b3a50 as *const (), 0), // +0x2b3a50 sub_2b3a50
    Lifted::new(0x70002b3b00, fn_70002b3b00 as *const (), 0), // +0x2b3b00 sub_2b3b00
    Lifted::new(0x70002b3b50, tpl_286::<0x70005a8240, 0x700060a310, 0x70002445b0, 0x70005a8230> as *const (), 0), // +0x2b3b50 sub_2b3b50
    Lifted::new(0x70002b3c90, fn_70002b3c90 as *const (), 0), // +0x2b3c90 sub_2b3c90
    Lifted::new(0x70002b40d0, fn_70002b40d0 as *const (), 0), // +0x2b40d0 sub_2b40d0
    Lifted::new(0x70002b4240, fn_70002b4240 as *const (), 2), // +0x2b4240 sub_2b4240
    Lifted::new(0x70002b4390, fn_70002b4390 as *const (), 0), // +0x2b4390 sub_2b4390
    Lifted::new(0x70002b4500, tpl_235::<0x8, 0x19, 0x10, 0xfffffffffffffff8, 0x7000697150, 0x70002b4830> as *const (), 0), // +0x2b4500 sub_2b4500
    Lifted::new(0x70002b45c0, fn_70002b45c0 as *const (), 0), // +0x2b45c0 sub_2b45c0
    Lifted::new(0x70002b4760, fn_70002b4760 as *const (), 0), // +0x2b4760 sub_2b4760
    Lifted::new(0x70002b4830, tpl_25::<0x7000694974, 0x7000698c88, 0x70005a4174, 0x70005a6310, 0x7000698be8, 0x7000698c10, 0x7000698c38, 0x7000698c60, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2b4830 sub_2b4830
    Lifted::new(0x70002b48b0, fn_70002b48b0 as *const (), 2), // +0x2b48b0 sub_2b48b0
    Lifted::new(0x70002b4a90, fn_70002b4a90 as *const (), 2), // +0x2b4a90 sub_2b4a90
    Lifted::new(0x70002b4bf0, tpl_240::<0x70002b4390, 0x7000697150, 0x700062e690, 0x7000262b00, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2b4bf0 sub_2b4bf0
    Lifted::new(0x70002b4db0, fn_70002b4db0 as *const (), 2), // +0x2b4db0 sub_2b4db0
    Lifted::new(0x70002b5020, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x70002b5480> as *const (), 0), // +0x2b5020 sub_2b5020
    Lifted::new(0x70002b50f0, fn_70002b50f0 as *const (), 0), // +0x2b50f0 sub_2b50f0
    Lifted::new(0x70002b52e0, fn_70002b52e0 as *const (), 2), // +0x2b52e0 sub_2b52e0
    Lifted::new(0x70002b5400, fn_70002b5400 as *const (), 0), // +0x2b5400 sub_2b5400
    Lifted::new(0x70002b5480, tpl_237::<0x700004de60, 0x7000697150, 0x70002b58a0, 0x70002b5020> as *const (), 2), // +0x2b5480 sub_2b5480
    Lifted::new(0x70002b5730, fn_70002b5730 as *const (), 0), // +0x2b5730 sub_2b5730
    Lifted::new(0x70002b58a0, tpl_236::<0x70005a4d32, 0x700062e490, 0x70002b4830, 0x7000697150, 0x70005f84c0, 0x70002b5020> as *const (), 2), // +0x2b58a0 sub_2b58a0
    Lifted::new(0x70002b5b80, fn_70002b5b80 as *const (), 2), // +0x2b5b80 sub_2b5b80
    Lifted::new(0x70002b5d00, fn_70002b5d00 as *const (), 2), // +0x2b5d00 sub_2b5d00
    Lifted::new(0x70002b5e40, fn_70002b5e40 as *const (), 2), // +0x2b5e40 sub_2b5e40
    Lifted::new(0x70002b5fb0, fn_70002b5fb0 as *const (), 2), // +0x2b5fb0 sub_2b5fb0
    Lifted::new(0x70002b6110, fn_70002b6110 as *const (), 0), // +0x2b6110 sub_2b6110
    Lifted::new(0x70002b6630, fn_70002b6630 as *const (), 0), // +0x2b6630 sub_2b6630
    Lifted::new(0x70002b6770, fn_70002b6770 as *const (), 0), // +0x2b6770 sub_2b6770
    Lifted::new(0x70002b67f0, fn_70002b67f0 as *const (), 0), // +0x2b67f0 sub_2b67f0
    Lifted::new(0x70002b6840, fn_70002b6840 as *const (), 0), // +0x2b6840 sub_2b6840
    Lifted::new(0x70002b6b10, fn_70002b6b10 as *const (), 2), // +0x2b6b10 sub_2b6b10
    Lifted::new(0x70002b70c0, fn_70002b70c0 as *const (), 0), // +0x2b70c0 sub_2b70c0
    Lifted::new(0x70002b71a0, fn_70002b71a0 as *const (), 0), // +0x2b71a0 sub_2b71a0
    Lifted::new(0x70002b7420, fn_70002b7420 as *const (), 0), // +0x2b7420 sub_2b7420
    Lifted::new(0x70002b7470, fn_70002b7470 as *const (), 0), // +0x2b7470 sub_2b7470
    Lifted::new(0x70002b7610, fn_70002b7610 as *const (), 2), // +0x2b7610 sub_2b7610
    Lifted::new(0x70002b7f50, fn_70002b7f50 as *const (), 0), // +0x2b7f50 sub_2b7f50
    Lifted::new(0x70002b7f80, fn_70002b7f80 as *const (), 0), // +0x2b7f80 sub_2b7f80
    Lifted::new(0x70002b7fc0, fn_70002b7fc0 as *const (), 2), // +0x2b7fc0 sub_2b7fc0
    Lifted::new(0x70002b83a0, fn_70002b83a0 as *const (), 0), // +0x2b83a0 sub_2b83a0
    Lifted::new(0x70002b8400, fn_70002b8400 as *const (), 0), // +0x2b8400 sub_2b8400
    Lifted::new(0x70002b84a0, fn_70002b84a0 as *const (), 0), // +0x2b84a0 sub_2b84a0
    Lifted::new(0x70002b8690, fn_70002b8690 as *const (), 0), // +0x2b8690 sub_2b8690
    Lifted::new(0x70002b8840, fn_70002b8840 as *const (), 0), // +0x2b8840 sub_2b8840
    Lifted::new(0x70002b88b0, tpl_287::<0x18, 0x70002bc020, 0x10, 0x8, 0x7000697150> as *const (), 0), // +0x2b88b0 sub_2b88b0
    Lifted::new(0x70002b8920, fn_70002b8920 as *const (), 0), // +0x2b8920 sub_2b8920
    Lifted::new(0x70002b89e0, fn_70002b89e0 as *const (), 0), // +0x2b89e0 sub_2b89e0
    Lifted::new(0x70002b8ab0, fn_70002b8ab0 as *const (), 0), // +0x2b8ab0 sub_2b8ab0
    Lifted::new(0x70002b8b30, tpl_287::<0x18, 0x70002b4500, 0x10, 0x8, 0x7000697150> as *const (), 0), // +0x2b8b30 sub_2b8b30
    Lifted::new(0x70002b8ba0, tpl_287::<0x18, 0x70002bc1a0, 0x10, 0x8, 0x7000697150> as *const (), 0), // +0x2b8ba0 sub_2b8ba0
    Lifted::new(0x70002b8c10, fn_70002b8c10 as *const (), 0), // +0x2b8c10 sub_2b8c10
    Lifted::new(0x70002b8cc0, tpl_287::<0x18, 0x700027e000, 0x10, 0x8, 0x7000697150> as *const (), 0), // +0x2b8cc0 sub_2b8cc0
    Lifted::new(0x70002b8d30, fn_70002b8d30 as *const (), 0), // +0x2b8d30 sub_2b8d30
    Lifted::new(0x70002b8e00, tpl_25::<0x7000694974, 0x700069a8c0, 0x70005a4174, 0x70005a63a0, 0x700069a8d0, 0x700069a8e8, 0x700069a900, 0x700069a918, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2b8e00 sub_2b8e00
    Lifted::new(0x70002b8e80, fn_70002b8e80 as *const (), 0), // +0x2b8e80 sub_2b8e80
    Lifted::new(0x70002b90b0, tpl_288::<0x700060a3a8, 0x7000606d4c, 0x7000266260, 0x7000021150, 0x70002bbdc0, 0x7000263120, 0x7000275ea0, 0x70002636c0, 0x7000697150, 0x70005f7edc, 0x70002b5e40, 0x70002b5b80> as *const (), 2), // +0x2b90b0 sub_2b90b0
    Lifted::new(0x70002b9400, tpl_289::<0x700060a3c0, 0x7000606d4c, 0x7000266260, 0x7000021150, 0x70002bbdc0, 0x7000263120, 0x7000275ea0, 0x70002636c0, 0x7000697150, 0x70005f7edc, 0x70002b5e40, 0x70002b5b80> as *const (), 2), // +0x2b9400 sub_2b9400
    Lifted::new(0x70002b9750, tpl_288::<0x700060a3c8, 0x7000606d4c, 0x7000266260, 0x7000021150, 0x70002bbdc0, 0x7000263120, 0x7000275ea0, 0x70002636c0, 0x7000697150, 0x70005f7edc, 0x70002b5e40, 0x70002b5b80> as *const (), 2), // +0x2b9750 sub_2b9750
    Lifted::new(0x70002b9aa0, tpl_288::<0x700060a3b4, 0x7000606d4c, 0x7000266260, 0x7000021150, 0x70002bbdc0, 0x7000263120, 0x7000275ea0, 0x70002636c0, 0x7000697150, 0x70005f7edc, 0x70002b5e40, 0x70002b5b80> as *const (), 2), // +0x2b9aa0 sub_2b9aa0
    Lifted::new(0x70002b9df0, fn_70002b9df0 as *const (), 2), // +0x2b9df0 sub_2b9df0
    Lifted::new(0x70002ba140, tpl_289::<0x700060a3e8, 0x7000606d4c, 0x7000266260, 0x7000021150, 0x70002bbdc0, 0x7000263120, 0x7000275ea0, 0x70002636c0, 0x7000697150, 0x70005f7edc, 0x70002b5e40, 0x70002b5b80> as *const (), 2), // +0x2ba140 sub_2ba140
    Lifted::new(0x70002ba490, tpl_289::<0x700060a3f0, 0x7000606d4c, 0x7000266260, 0x7000021150, 0x70002bbdc0, 0x7000263120, 0x7000275ea0, 0x70002636c0, 0x7000697150, 0x70005f7edc, 0x70002b5e40, 0x70002b5b80> as *const (), 2), // +0x2ba490 sub_2ba490
    Lifted::new(0x70002ba7e0, fn_70002ba7e0 as *const (), 2), // +0x2ba7e0 sub_2ba7e0
    Lifted::new(0x70002bab30, fn_70002bab30 as *const (), 2), // +0x2bab30 sub_2bab30
    Lifted::new(0x70002bae70, fn_70002bae70 as *const (), 2), // +0x2bae70 sub_2bae70
    Lifted::new(0x70002bb1c0, fn_70002bb1c0 as *const (), 0), // +0x2bb1c0 sub_2bb1c0
    Lifted::new(0x70002bb320, tpl_290::<0x70005a8240, 0x700060a918, 0x70002445b0, 0x700025c890, 0x70002bc930, 0x7000697150> as *const (), 0), // +0x2bb320 sub_2bb320
    Lifted::new(0x70002bb4b0, fn_70002bb4b0 as *const (), 0), // +0x2bb4b0 sub_2bb4b0
    Lifted::new(0x70002bb6f0, tpl_290::<0x70005a8240, 0x700060a9f8, 0x70002445b0, 0x700025c890, 0x70002bc930, 0x7000697150> as *const (), 0), // +0x2bb6f0 sub_2bb6f0
    Lifted::new(0x70002bb880, tpl_114::<0x18, 0x8, 0x70002bca20> as *const (), 0), // +0x2bb880 sub_2bb880
    Lifted::new(0x70002bb910, fn_70002bb910 as *const (), 0), // +0x2bb910 sub_2bb910
    Lifted::new(0x70002bb9e0, fn_70002bb9e0 as *const (), 0), // +0x2bb9e0 sub_2bb9e0
    Lifted::new(0x70002bba80, fn_70002bba80 as *const (), 0), // +0x2bba80 sub_2bba80
    Lifted::new(0x70002bbb00, fn_70002bbb00 as *const (), 0), // +0x2bbb00 sub_2bbb00
    Lifted::new(0x70002bbcb0, fn_70002bbcb0 as *const (), 0), // +0x2bbcb0 sub_2bbcb0
    Lifted::new(0x70002bbdc0, fn_70002bbdc0 as *const (), 2), // +0x2bbdc0 sub_2bbdc0
    Lifted::new(0x70002bc020, tpl_235::<0x8, 0x19, 0x10, 0xfffffffffffffff8, 0x7000697150, 0x70002bc8b0> as *const (), 0), // +0x2bc020 sub_2bc020
    Lifted::new(0x70002bc0e0, tpl_235::<0x8, 0x19, 0x10, 0xfffffffffffffff8, 0x7000697150, 0x70002bc830> as *const (), 0), // +0x2bc0e0 sub_2bc0e0
    Lifted::new(0x70002bc1a0, tpl_235::<0x8, 0x19, 0x10, 0xfffffffffffffff8, 0x7000697150, 0x70002bc7b0> as *const (), 0), // +0x2bc1a0 sub_2bc1a0
    Lifted::new(0x70002bc260, tpl_235::<0x8, 0x19, 0x10, 0xfffffffffffffff8, 0x7000697150, 0x70002bc730> as *const (), 0), // +0x2bc260 sub_2bc260
    Lifted::new(0x70002bc320, fn_70002bc320 as *const (), 0), // +0x2bc320 sub_2bc320
    Lifted::new(0x70002bc3e0, tpl_291::<0x8, 0x19, 0x10, 0x20, 0x70002bc630, 0x7000697150> as *const (), 0), // +0x2bc3e0 sub_2bc3e0
    Lifted::new(0x70002bc450, fn_70002bc450 as *const (), 0), // +0x2bc450 sub_2bc450
    Lifted::new(0x70002bc4c0, tpl_291::<0x8, 0x19, 0x10, 0x20, 0x7000271900, 0x7000697150> as *const (), 0), // +0x2bc4c0 sub_2bc4c0
    Lifted::new(0x70002bc530, tpl_292::<0x8, 0x70002bcf70, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x2bc530 sub_2bc530
    Lifted::new(0x70002bc5b0, tpl_25::<0x7000694974, 0x7000699020, 0x70005a4174, 0x70005a6410, 0x7000699028, 0x7000699034, 0x7000699040, 0x700069904c, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2bc5b0 sub_2bc5b0
    Lifted::new(0x70002bc630, tpl_25::<0x7000694974, 0x7000699024, 0x70005a4174, 0x70005a6400, 0x7000698f30, 0x7000698f48, 0x7000698f60, 0x7000698f78, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2bc630 sub_2bc630
    Lifted::new(0x70002bc6b0, tpl_25::<0x7000694974, 0x7000698f20, 0x70005a4174, 0x70005a63f0, 0x7000698fa0, 0x7000698fc0, 0x7000698fe0, 0x7000699000, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2bc6b0 sub_2bc6b0
    Lifted::new(0x70002bc730, tpl_25::<0x7000694974, 0x7000698f90, 0x70005a4174, 0x70005a63e0, 0x7000698dd8, 0x7000698e00, 0x7000698e28, 0x7000698e50, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2bc730 sub_2bc730
    Lifted::new(0x70002bc7b0, tpl_25::<0x7000694974, 0x7000698dd0, 0x70005a4174, 0x70005a63d0, 0x7000698e80, 0x7000698ea8, 0x7000698ed0, 0x7000698ef8, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2bc7b0 sub_2bc7b0
    Lifted::new(0x70002bc830, tpl_25::<0x7000694974, 0x7000698e78, 0x70005a4174, 0x70005a63c0, 0x7000698c90, 0x7000698cb8, 0x7000698ce0, 0x7000698d08, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2bc830 sub_2bc830
    Lifted::new(0x70002bc8b0, tpl_25::<0x7000694974, 0x7000698c8c, 0x70005a4174, 0x70005a63b0, 0x7000698d30, 0x7000698d58, 0x7000698d80, 0x7000698da8, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2bc8b0 sub_2bc8b0
    Lifted::new(0x70002bc930, fn_70002bc930 as *const (), 0), // +0x2bc930 sub_2bc930
    Lifted::new(0x70002bca20, fn_70002bca20 as *const (), 2), // +0x2bca20 sub_2bca20
    Lifted::new(0x70002bcca0, fn_70002bcca0 as *const (), 2), // +0x2bcca0 sub_2bcca0
    Lifted::new(0x70002bcf70, tpl_293::<0x20, 0x30, 0x28, 0x7000697150, 0x10, 0x8> as *const (), 0), // +0x2bcf70 sub_2bcf70
    Lifted::new(0x70002bcfe0, fn_70002bcfe0 as *const (), 2), // +0x2bcfe0 sub_2bcfe0
    Lifted::new(0x70002bd1b0, fn_70002bd1b0 as *const (), 2), // +0x2bd1b0 sub_2bd1b0
    Lifted::new(0x70002bd840, fn_70002bd840 as *const (), 2), // +0x2bd840 sub_2bd840
    Lifted::new(0x70002bdb70, fn_70002bdb70 as *const (), 2), // +0x2bdb70 sub_2bdb70
    Lifted::new(0x70002be2b0, fn_70002be2b0 as *const (), 1), // +0x2be2b0 sub_2be2b0
    Lifted::new(0x70002be320, fn_70002be320 as *const (), 2), // +0x2be320 sub_2be320
    Lifted::new(0x70002c23c0, fn_70002c23c0 as *const (), 2), // +0x2c23c0 sub_2c23c0
    Lifted::new(0x70002c5d10, fn_70002c5d10 as *const (), 0), // +0x2c5d10 sub_2c5d10
    Lifted::new(0x70002c5e20, fn_70002c5e20 as *const (), 0), // +0x2c5e20 sub_2c5e20
    Lifted::new(0x70002c5ea0, fn_70002c5ea0 as *const (), 2), // +0x2c5ea0 sub_2c5ea0
    Lifted::new(0x70002c5f50, fn_70002c5f50 as *const (), 2), // +0x2c5f50 sub_2c5f50
    Lifted::new(0x70002c7020, tpl_293::<0x18, 0x28, 0x20, 0x7000697150, 0x10, 0x8> as *const (), 0), // +0x2c7020 sub_2c7020
    Lifted::new(0x70002c7090, fn_70002c7090 as *const (), 0), // +0x2c7090 sub_2c7090
    Lifted::new(0x70002c7120, fn_70002c7120 as *const (), 2), // +0x2c7120 sub_2c7120
    Lifted::new(0x70002c7b20, fn_70002c7b20 as *const (), 2), // +0x2c7b20 sub_2c7b20
    Lifted::new(0x70002c7d30, fn_70002c7d30 as *const (), 2), // +0x2c7d30 sub_2c7d30
    Lifted::new(0x70002c7fe0, fn_70002c7fe0 as *const (), 0), // +0x2c7fe0 sub_2c7fe0
    Lifted::new(0x70002c81a0, fn_70002c81a0 as *const (), 0), // +0x2c81a0 sub_2c81a0
    Lifted::new(0x70002c8220, fn_70002c8220 as *const (), 0), // +0x2c8220 sub_2c8220
    Lifted::new(0x70002c82a0, fn_70002c82a0 as *const (), 2), // +0x2c82a0 sub_2c82a0
    Lifted::new(0x70002c83a0, fn_70002c83a0 as *const (), 0), // +0x2c83a0 sub_2c83a0
    Lifted::new(0x70002c8420, fn_70002c8420 as *const (), 0), // +0x2c8420 sub_2c8420
    Lifted::new(0x70002c8850, fn_70002c8850 as *const (), 2), // +0x2c8850 sub_2c8850
    Lifted::new(0x70002c9210, fn_70002c9210 as *const (), 2), // +0x2c9210 sub_2c9210
    Lifted::new(0x70002c9ab0, fn_70002c9ab0 as *const (), 2), // +0x2c9ab0 sub_2c9ab0
    Lifted::new(0x70002ca030, fn_70002ca030 as *const (), 0), // +0x2ca030 sub_2ca030
    Lifted::new(0x70002ca5f0, fn_70002ca5f0 as *const (), 2), // +0x2ca5f0 sub_2ca5f0
    Lifted::new(0x70002cbf30, fn_70002cbf30 as *const (), 2), // +0x2cbf30 sub_2cbf30
    Lifted::new(0x70002cca40, fn_70002cca40 as *const (), 2), // +0x2cca40 sub_2cca40
    Lifted::new(0x70002ccc40, fn_70002ccc40 as *const (), 2), // +0x2ccc40 sub_2ccc40
    Lifted::new(0x70002cd130, fn_70002cd130 as *const (), 2), // +0x2cd130 sub_2cd130
    Lifted::new(0x70002cd510, fn_70002cd510 as *const (), 2), // +0x2cd510 sub_2cd510
    Lifted::new(0x70002cd670, fn_70002cd670 as *const (), 2), // +0x2cd670 sub_2cd670
    Lifted::new(0x70002cd830, fn_70002cd830 as *const (), 2), // +0x2cd830 sub_2cd830
    Lifted::new(0x70002cdbb0, fn_70002cdbb0 as *const (), 2), // +0x2cdbb0 sub_2cdbb0
    Lifted::new(0x70002cdc80, fn_70002cdc80 as *const (), 2), // +0x2cdc80 sub_2cdc80
    Lifted::new(0x70002cdee0, fn_70002cdee0 as *const (), 0), // +0x2cdee0 sub_2cdee0
    Lifted::new(0x70002ce050, fn_70002ce050 as *const (), 1), // +0x2ce050 sub_2ce050
    Lifted::new(0x70002ce060, fn_70002ce060 as *const (), 0), // +0x2ce060 sub_2ce060
    Lifted::new(0x70002ce080, fn_70002ce080 as *const (), 0), // +0x2ce080 sub_2ce080
    Lifted::new(0x70002ce110, fn_70002ce110 as *const (), 2), // +0x2ce110 sub_2ce110
    Lifted::new(0x70002ce1b0, fn_70002ce1b0 as *const (), 0), // +0x2ce1b0 sub_2ce1b0
    Lifted::new(0x70002ce220, fn_70002ce220 as *const (), 2), // +0x2ce220 sub_2ce220
    Lifted::new(0x70002ce360, fn_70002ce360 as *const (), 2), // +0x2ce360 sub_2ce360
    Lifted::new(0x70002ce490, fn_70002ce490 as *const (), 0), // +0x2ce490 sub_2ce490
    Lifted::new(0x70002ce530, fn_70002ce530 as *const (), 2), // +0x2ce530 sub_2ce530
    Lifted::new(0x70002ce6e0, fn_70002ce6e0 as *const (), 0), // +0x2ce6e0 sub_2ce6e0
    Lifted::new(0x70002ce760, fn_70002ce760 as *const (), 0), // +0x2ce760 sub_2ce760
    Lifted::new(0x70002ce7d0, fn_70002ce7d0 as *const (), 0), // +0x2ce7d0 sub_2ce7d0
    Lifted::new(0x70002ce8a0, fn_70002ce8a0 as *const (), 0), // +0x2ce8a0 sub_2ce8a0
    Lifted::new(0x70002ce950, fn_70002ce950 as *const (), 2), // +0x2ce950 sub_2ce950
    Lifted::new(0x70002cea90, fn_70002cea90 as *const (), 0), // +0x2cea90 sub_2cea90
    Lifted::new(0x70002ceb20, fn_70002ceb20 as *const (), 2), // +0x2ceb20 sub_2ceb20
    Lifted::new(0x70002cec30, fn_70002cec30 as *const (), 2), // +0x2cec30 sub_2cec30
    Lifted::new(0x70002ced20, fn_70002ced20 as *const (), 0), // +0x2ced20 sub_2ced20
    Lifted::new(0x70002ced90, fn_70002ced90 as *const (), 0), // +0x2ced90 sub_2ced90
    Lifted::new(0x70002cee70, fn_70002cee70 as *const (), 2), // +0x2cee70 sub_2cee70
    Lifted::new(0x70002cf040, tpl_241::<0x70002cca40, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2cf040 sub_2cf040
    Lifted::new(0x70002cf190, tpl_248::<0x70005e061c, 0x7000266260, 0x70006974b8, 0x7000606bc8, 0x7000246600, 0x70002c23c0, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2cf190 sub_2cf190
    Lifted::new(0x70002cf360, tpl_248::<0x70005e061c, 0x7000266260, 0x70006974b8, 0x7000606bc8, 0x7000246600, 0x70002c7120, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2cf360 sub_2cf360
    Lifted::new(0x70002cf530, tpl_248::<0x70005e061c, 0x7000266260, 0x70006974b8, 0x7000606bc8, 0x7000246600, 0x70002c8850, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2cf530 sub_2cf530
    Lifted::new(0x70002cf700, tpl_248::<0x70005e061c, 0x7000266260, 0x70006974b8, 0x7000606bc8, 0x7000246600, 0x70002c9210, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2cf700 sub_2cf700
    Lifted::new(0x70002cf8d0, tpl_294::<0x70005e0438, 0x70002da4d0, 0x70002c9ab0, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2cf8d0 sub_2cf8d0
    Lifted::new(0x70002cfa70, fn_70002cfa70 as *const (), 2), // +0x2cfa70 sub_2cfa70
    Lifted::new(0x70002cfc40, fn_70002cfc40 as *const (), 2), // +0x2cfc40 sub_2cfc40
    Lifted::new(0x70002cfe60, fn_70002cfe60 as *const (), 2), // +0x2cfe60 sub_2cfe60
    Lifted::new(0x70002cff40, fn_70002cff40 as *const (), 2), // +0x2cff40 sub_2cff40
    Lifted::new(0x70002d0020, fn_70002d0020 as *const (), 2), // +0x2d0020 sub_2d0020
    Lifted::new(0x70002d0100, fn_70002d0100 as *const (), 2), // +0x2d0100 sub_2d0100
    Lifted::new(0x70002d01e0, fn_70002d01e0 as *const (), 2), // +0x2d01e0 sub_2d01e0
    Lifted::new(0x70002d02c0, fn_70002d02c0 as *const (), 2), // +0x2d02c0 sub_2d02c0
    Lifted::new(0x70002d03f0, fn_70002d03f0 as *const (), 2), // +0x2d03f0 sub_2d03f0
    Lifted::new(0x70002d0520, fn_70002d0520 as *const (), 2), // +0x2d0520 sub_2d0520
    Lifted::new(0x70002d0600, fn_70002d0600 as *const (), 2), // +0x2d0600 sub_2d0600
    Lifted::new(0x70002d06e0, fn_70002d06e0 as *const (), 2), // +0x2d06e0 sub_2d06e0
    Lifted::new(0x70002d07f0, fn_70002d07f0 as *const (), 2), // +0x2d07f0 sub_2d07f0
    Lifted::new(0x70002d0900, fn_70002d0900 as *const (), 2), // +0x2d0900 sub_2d0900
    Lifted::new(0x70002d0a10, fn_70002d0a10 as *const (), 2), // +0x2d0a10 sub_2d0a10
    Lifted::new(0x70002d0b20, fn_70002d0b20 as *const (), 2), // +0x2d0b20 sub_2d0b20
    Lifted::new(0x70002d0c00, fn_70002d0c00 as *const (), 2), // +0x2d0c00 sub_2d0c00
    Lifted::new(0x70002d0ce0, fn_70002d0ce0 as *const (), 2), // +0x2d0ce0 sub_2d0ce0
    Lifted::new(0x70002d0dc0, fn_70002d0dc0 as *const (), 2), // +0x2d0dc0 sub_2d0dc0
    Lifted::new(0x70002d0ea0, fn_70002d0ea0 as *const (), 2), // +0x2d0ea0 sub_2d0ea0
    Lifted::new(0x70002d0f80, fn_70002d0f80 as *const (), 2), // +0x2d0f80 sub_2d0f80
    Lifted::new(0x70002d1060, fn_70002d1060 as *const (), 2), // +0x2d1060 sub_2d1060
    Lifted::new(0x70002d1140, fn_70002d1140 as *const (), 2), // +0x2d1140 sub_2d1140
    Lifted::new(0x70002d1220, fn_70002d1220 as *const (), 2), // +0x2d1220 sub_2d1220
    Lifted::new(0x70002d1310, fn_70002d1310 as *const (), 2), // +0x2d1310 sub_2d1310
    Lifted::new(0x70002d1400, fn_70002d1400 as *const (), 2), // +0x2d1400 sub_2d1400
    Lifted::new(0x70002d14f0, fn_70002d14f0 as *const (), 2), // +0x2d14f0 sub_2d14f0
    Lifted::new(0x70002d15e0, fn_70002d15e0 as *const (), 2), // +0x2d15e0 sub_2d15e0
    Lifted::new(0x70002d16d0, fn_70002d16d0 as *const (), 2), // +0x2d16d0 sub_2d16d0
    Lifted::new(0x70002d17c0, fn_70002d17c0 as *const (), 2), // +0x2d17c0 sub_2d17c0
    Lifted::new(0x70002d18b0, fn_70002d18b0 as *const (), 2), // +0x2d18b0 sub_2d18b0
    Lifted::new(0x70002d19a0, fn_70002d19a0 as *const (), 2), // +0x2d19a0 sub_2d19a0
    Lifted::new(0x70002d1a90, fn_70002d1a90 as *const (), 2), // +0x2d1a90 sub_2d1a90
    Lifted::new(0x70002d1b80, fn_70002d1b80 as *const (), 2), // +0x2d1b80 sub_2d1b80
    Lifted::new(0x70002d1c70, fn_70002d1c70 as *const (), 2), // +0x2d1c70 sub_2d1c70
    Lifted::new(0x70002d1d60, fn_70002d1d60 as *const (), 2), // +0x2d1d60 sub_2d1d60
    Lifted::new(0x70002d1e40, fn_70002d1e40 as *const (), 2), // +0x2d1e40 sub_2d1e40
    Lifted::new(0x70002d1f20, fn_70002d1f20 as *const (), 2), // +0x2d1f20 sub_2d1f20
    Lifted::new(0x70002d2000, fn_70002d2000 as *const (), 2), // +0x2d2000 sub_2d2000
    Lifted::new(0x70002d20f0, fn_70002d20f0 as *const (), 2), // +0x2d20f0 sub_2d20f0
    Lifted::new(0x70002d21e0, fn_70002d21e0 as *const (), 2), // +0x2d21e0 sub_2d21e0
    Lifted::new(0x70002d22d0, fn_70002d22d0 as *const (), 2), // +0x2d22d0 sub_2d22d0
    Lifted::new(0x70002d23b0, fn_70002d23b0 as *const (), 2), // +0x2d23b0 sub_2d23b0
    Lifted::new(0x70002d24a0, fn_70002d24a0 as *const (), 2), // +0x2d24a0 sub_2d24a0
    Lifted::new(0x70002d2590, fn_70002d2590 as *const (), 2), // +0x2d2590 sub_2d2590
    Lifted::new(0x70002d26b0, fn_70002d26b0 as *const (), 2), // +0x2d26b0 sub_2d26b0
    Lifted::new(0x70002d27d0, fn_70002d27d0 as *const (), 2), // +0x2d27d0 sub_2d27d0
    Lifted::new(0x70002d2900, fn_70002d2900 as *const (), 2), // +0x2d2900 sub_2d2900
    Lifted::new(0x70002d29e0, fn_70002d29e0 as *const (), 2), // +0x2d29e0 sub_2d29e0
    Lifted::new(0x70002d2ac0, fn_70002d2ac0 as *const (), 2), // +0x2d2ac0 sub_2d2ac0
    Lifted::new(0x70002d2bc0, fn_70002d2bc0 as *const (), 2), // +0x2d2bc0 sub_2d2bc0
    Lifted::new(0x70002d2ce0, fn_70002d2ce0 as *const (), 2), // +0x2d2ce0 sub_2d2ce0
    Lifted::new(0x70002d2e00, tpl_295::<0x10, 0x70002159f0, 0xc, 0x8, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x1, 0x70002ce7d0, 0x70005fab48, 0x700026c5d0, 0x18> as *const (), 2), // +0x2d2e00 sub_2d2e00
    Lifted::new(0x70002d2ee0, fn_70002d2ee0 as *const (), 2), // +0x2d2ee0 sub_2d2ee0
    Lifted::new(0x70002d2fc0, tpl_296::<0x10, 0x70002114f0, 0xc, 0x8, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x1, 0x70002ce7d0, 0x70005f7cf8, 0x700026c5d0, 0x18> as *const (), 2), // +0x2d2fc0 sub_2d2fc0
    Lifted::new(0x70002d30a0, fn_70002d30a0 as *const (), 2), // +0x2d30a0 sub_2d30a0
    Lifted::new(0x70002d31c0, fn_70002d31c0 as *const (), 2), // +0x2d31c0 sub_2d31c0
    Lifted::new(0x70002d32a0, fn_70002d32a0 as *const (), 2), // +0x2d32a0 sub_2d32a0
    Lifted::new(0x70002d3380, fn_70002d3380 as *const (), 2), // +0x2d3380 sub_2d3380
    Lifted::new(0x70002d3490, fn_70002d3490 as *const (), 2), // +0x2d3490 sub_2d3490
    Lifted::new(0x70002d35a0, fn_70002d35a0 as *const (), 2), // +0x2d35a0 sub_2d35a0
    Lifted::new(0x70002d36c0, fn_70002d36c0 as *const (), 2), // +0x2d36c0 sub_2d36c0
    Lifted::new(0x70002d37a0, fn_70002d37a0 as *const (), 2), // +0x2d37a0 sub_2d37a0
    Lifted::new(0x70002d38d0, fn_70002d38d0 as *const (), 2), // +0x2d38d0 sub_2d38d0
    Lifted::new(0x70002d39b0, fn_70002d39b0 as *const (), 2), // +0x2d39b0 sub_2d39b0
    Lifted::new(0x70002d3ac0, fn_70002d3ac0 as *const (), 2), // +0x2d3ac0 sub_2d3ac0
    Lifted::new(0x70002d3bf0, fn_70002d3bf0 as *const (), 2), // +0x2d3bf0 sub_2d3bf0
    Lifted::new(0x70002d3d20, fn_70002d3d20 as *const (), 2), // +0x2d3d20 sub_2d3d20
    Lifted::new(0x70002d3e40, fn_70002d3e40 as *const (), 2), // +0x2d3e40 sub_2d3e40
    Lifted::new(0x70002d3f20, fn_70002d3f20 as *const (), 2), // +0x2d3f20 sub_2d3f20
    Lifted::new(0x70002d4000, tpl_297::<0x10, 0x70002114f0, 0xc, 0x8, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x1, 0x70002ce7d0, 0x70005f8c30, 0x700026c5d0, 0x18> as *const (), 2), // +0x2d4000 sub_2d4000
    Lifted::new(0x70002d40e0, fn_70002d40e0 as *const (), 2), // +0x2d40e0 sub_2d40e0
    Lifted::new(0x70002d41c0, fn_70002d41c0 as *const (), 2), // +0x2d41c0 sub_2d41c0
    Lifted::new(0x70002d42a0, fn_70002d42a0 as *const (), 2), // +0x2d42a0 sub_2d42a0
    Lifted::new(0x70002d43a0, fn_70002d43a0 as *const (), 2), // +0x2d43a0 sub_2d43a0
    Lifted::new(0x70002d44a0, fn_70002d44a0 as *const (), 2), // +0x2d44a0 sub_2d44a0
    Lifted::new(0x70002d45b0, fn_70002d45b0 as *const (), 2), // +0x2d45b0 sub_2d45b0
    Lifted::new(0x70002d46c0, tpl_295::<0x20, 0x7000216640, 0xc, 0x8, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x1, 0x70002ce7d0, 0x70005fae80, 0x700026c5d0, 0x18> as *const (), 2), // +0x2d46c0 sub_2d46c0
    Lifted::new(0x70002d47a0, fn_70002d47a0 as *const (), 2), // +0x2d47a0 sub_2d47a0
    Lifted::new(0x70002d4880, fn_70002d4880 as *const (), 2), // +0x2d4880 sub_2d4880
    Lifted::new(0x70002d4960, fn_70002d4960 as *const (), 2), // +0x2d4960 sub_2d4960
    Lifted::new(0x70002d4a40, fn_70002d4a40 as *const (), 2), // +0x2d4a40 sub_2d4a40
    Lifted::new(0x70002d4b20, fn_70002d4b20 as *const (), 2), // +0x2d4b20 sub_2d4b20
    Lifted::new(0x70002d4c00, tpl_297::<0x20, 0x7000216d30, 0xc, 0x8, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x1, 0x70002ce7d0, 0x70005fafb8, 0x700026c5d0, 0x18> as *const (), 2), // +0x2d4c00 sub_2d4c00
    Lifted::new(0x70002d4ce0, fn_70002d4ce0 as *const (), 2), // +0x2d4ce0 sub_2d4ce0
    Lifted::new(0x70002d4dc0, tpl_295::<0x28, 0x7000217c30, 0xc, 0x8, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x1, 0x70002ce7d0, 0x70005fb030, 0x700026c5d0, 0x18> as *const (), 2), // +0x2d4dc0 sub_2d4dc0
    Lifted::new(0x70002d4ea0, tpl_296::<0x28, 0x7000216750, 0xc, 0x8, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x1, 0x70002ce7d0, 0x70005fb068, 0x700026c5d0, 0x18> as *const (), 2), // +0x2d4ea0 sub_2d4ea0
    Lifted::new(0x70002d4f80, fn_70002d4f80 as *const (), 2), // +0x2d4f80 sub_2d4f80
    Lifted::new(0x70002d5060, fn_70002d5060 as *const (), 2), // +0x2d5060 sub_2d5060
    Lifted::new(0x70002d5140, fn_70002d5140 as *const (), 2), // +0x2d5140 sub_2d5140
    Lifted::new(0x70002d5220, fn_70002d5220 as *const (), 2), // +0x2d5220 sub_2d5220
    Lifted::new(0x70002d5300, fn_70002d5300 as *const (), 2), // +0x2d5300 sub_2d5300
    Lifted::new(0x70002d53e0, fn_70002d53e0 as *const (), 2), // +0x2d53e0 sub_2d53e0
    Lifted::new(0x70002d54c0, fn_70002d54c0 as *const (), 2), // +0x2d54c0 sub_2d54c0
    Lifted::new(0x70002d55b0, fn_70002d55b0 as *const (), 2), // +0x2d55b0 sub_2d55b0
    Lifted::new(0x70002d5690, fn_70002d5690 as *const (), 2), // +0x2d5690 sub_2d5690
    Lifted::new(0x70002d5770, fn_70002d5770 as *const (), 2), // +0x2d5770 sub_2d5770
    Lifted::new(0x70002d5860, fn_70002d5860 as *const (), 2), // +0x2d5860 sub_2d5860
    Lifted::new(0x70002d5950, fn_70002d5950 as *const (), 2), // +0x2d5950 sub_2d5950
    Lifted::new(0x70002d5a40, fn_70002d5a40 as *const (), 2), // +0x2d5a40 sub_2d5a40
    Lifted::new(0x70002d5b20, fn_70002d5b20 as *const (), 2), // +0x2d5b20 sub_2d5b20
    Lifted::new(0x70002d5c10, fn_70002d5c10 as *const (), 2), // +0x2d5c10 sub_2d5c10
    Lifted::new(0x70002d5d10, fn_70002d5d10 as *const (), 2), // +0x2d5d10 sub_2d5d10
    Lifted::new(0x70002d5e00, fn_70002d5e00 as *const (), 2), // +0x2d5e00 sub_2d5e00
    Lifted::new(0x70002d5ee0, fn_70002d5ee0 as *const (), 2), // +0x2d5ee0 sub_2d5ee0
    Lifted::new(0x70002d5fc0, fn_70002d5fc0 as *const (), 2), // +0x2d5fc0 sub_2d5fc0
    Lifted::new(0x70002d60a0, fn_70002d60a0 as *const (), 2), // +0x2d60a0 sub_2d60a0
    Lifted::new(0x70002d6180, fn_70002d6180 as *const (), 2), // +0x2d6180 sub_2d6180
    Lifted::new(0x70002d6260, fn_70002d6260 as *const (), 2), // +0x2d6260 sub_2d6260
    Lifted::new(0x70002d6340, fn_70002d6340 as *const (), 2), // +0x2d6340 sub_2d6340
    Lifted::new(0x70002d6420, fn_70002d6420 as *const (), 2), // +0x2d6420 sub_2d6420
    Lifted::new(0x70002d6500, fn_70002d6500 as *const (), 2), // +0x2d6500 sub_2d6500
    Lifted::new(0x70002d65e0, fn_70002d65e0 as *const (), 2), // +0x2d65e0 sub_2d65e0
    Lifted::new(0x70002d66c0, fn_70002d66c0 as *const (), 2), // +0x2d66c0 sub_2d66c0
    Lifted::new(0x70002d67a0, fn_70002d67a0 as *const (), 2), // +0x2d67a0 sub_2d67a0
    Lifted::new(0x70002d68c0, fn_70002d68c0 as *const (), 2), // +0x2d68c0 sub_2d68c0
    Lifted::new(0x70002d69b0, fn_70002d69b0 as *const (), 2), // +0x2d69b0 sub_2d69b0
    Lifted::new(0x70002d6a90, fn_70002d6a90 as *const (), 2), // +0x2d6a90 sub_2d6a90
    Lifted::new(0x70002d6b80, fn_70002d6b80 as *const (), 2), // +0x2d6b80 sub_2d6b80
    Lifted::new(0x70002d6c60, fn_70002d6c60 as *const (), 2), // +0x2d6c60 sub_2d6c60
    Lifted::new(0x70002d6d40, fn_70002d6d40 as *const (), 2), // +0x2d6d40 sub_2d6d40
    Lifted::new(0x70002d6e50, fn_70002d6e50 as *const (), 2), // +0x2d6e50 sub_2d6e50
    Lifted::new(0x70002d6f40, fn_70002d6f40 as *const (), 2), // +0x2d6f40 sub_2d6f40
    Lifted::new(0x70002d7030, fn_70002d7030 as *const (), 2), // +0x2d7030 sub_2d7030
    Lifted::new(0x70002d7150, fn_70002d7150 as *const (), 2), // +0x2d7150 sub_2d7150
    Lifted::new(0x70002d7270, fn_70002d7270 as *const (), 2), // +0x2d7270 sub_2d7270
    Lifted::new(0x70002d7380, fn_70002d7380 as *const (), 2), // +0x2d7380 sub_2d7380
    Lifted::new(0x70002d7490, fn_70002d7490 as *const (), 2), // +0x2d7490 sub_2d7490
    Lifted::new(0x70002d75a0, fn_70002d75a0 as *const (), 2), // +0x2d75a0 sub_2d75a0
    Lifted::new(0x70002d76b0, fn_70002d76b0 as *const (), 2), // +0x2d76b0 sub_2d76b0
    Lifted::new(0x70002d77c0, fn_70002d77c0 as *const (), 2), // +0x2d77c0 sub_2d77c0
    Lifted::new(0x70002d78e0, fn_70002d78e0 as *const (), 2), // +0x2d78e0 sub_2d78e0
    Lifted::new(0x70002d79f0, fn_70002d79f0 as *const (), 0), // +0x2d79f0 sub_2d79f0
    Lifted::new(0x70002d7aa0, fn_70002d7aa0 as *const (), 2), // +0x2d7aa0 sub_2d7aa0
    Lifted::new(0x70002d7c30, fn_70002d7c30 as *const (), 0), // +0x2d7c30 sub_2d7c30
    Lifted::new(0x70002d7e00, fn_70002d7e00 as *const (), 2), // +0x2d7e00 sub_2d7e00
    Lifted::new(0x70002d7f40, fn_70002d7f40 as *const (), 2), // +0x2d7f40 sub_2d7f40
    Lifted::new(0x70002d8130, fn_70002d8130 as *const (), 2), // +0x2d8130 sub_2d8130
    Lifted::new(0x70002d81b0, fn_70002d81b0 as *const (), 0), // +0x2d81b0 sub_2d81b0
    Lifted::new(0x70002d8240, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x70002da220> as *const (), 0), // +0x2d8240 sub_2d8240
    Lifted::new(0x70002d8310, fn_70002d8310 as *const (), 2), // +0x2d8310 sub_2d8310
    Lifted::new(0x70002d9310, fn_70002d9310 as *const (), 2), // +0x2d9310 sub_2d9310
    Lifted::new(0x70002d9380, fn_70002d9380 as *const (), 2), // +0x2d9380 sub_2d9380
    Lifted::new(0x70002d94c0, fn_70002d94c0 as *const (), 2), // +0x2d94c0 sub_2d94c0
    Lifted::new(0x70002d9630, fn_70002d9630 as *const (), 0), // +0x2d9630 sub_2d9630
    Lifted::new(0x70002d96a0, tpl_292::<0x8, 0x70002daef0, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x2d96a0 sub_2d96a0
    Lifted::new(0x70002d9720, fn_70002d9720 as *const (), 2), // +0x2d9720 sub_2d9720
    Lifted::new(0x70002d99d0, fn_70002d99d0 as *const (), 2), // +0x2d99d0 sub_2d99d0
    Lifted::new(0x70002d9ca0, fn_70002d9ca0 as *const (), 2), // +0x2d9ca0 sub_2d9ca0
    Lifted::new(0x70002d9ec0, fn_70002d9ec0 as *const (), 0), // +0x2d9ec0 sub_2d9ec0
    Lifted::new(0x70002da220, tpl_237::<0x700004de60, 0x7000697150, 0x70002da5c0, 0x70002d8240> as *const (), 2), // +0x2da220 sub_2da220
    Lifted::new(0x70002da4d0, tpl_298::<0x7000266260, 0x8, 0x70006974b8, 0x7000606bc8, 0x7000246600, 0x10, 0xe, 0x700026c6d0> as *const (), 0), // +0x2da4d0 sub_2da4d0
    Lifted::new(0x70002da590, tpl_74::<0x10, 0x2, 0x70002da8a0> as *const (), 0), // +0x2da590 sub_2da590
    Lifted::new(0x70002da5c0, tpl_236::<0x70005a4d32, 0x700062e490, 0x70002bc730, 0x7000697150, 0x70005f84c0, 0x70002d8240> as *const (), 2), // +0x2da5c0 sub_2da5c0
    Lifted::new(0x70002da8a0, fn_70002da8a0 as *const (), 2), // +0x2da8a0 sub_2da8a0
    Lifted::new(0x70002dabb0, fn_70002dabb0 as *const (), 2), // +0x2dabb0 sub_2dabb0
    Lifted::new(0x70002dae60, fn_70002dae60 as *const (), 2), // +0x2dae60 sub_2dae60
    Lifted::new(0x70002daef0, tpl_293::<0x28, 0x38, 0x30, 0x7000697150, 0x10, 0x8> as *const (), 0), // +0x2daef0 sub_2daef0
    Lifted::new(0x70002daf60, tpl_114::<0x18, 0x8, 0x70002db730> as *const (), 0), // +0x2daf60 sub_2daf60
    Lifted::new(0x70002daff0, fn_70002daff0 as *const (), 2), // +0x2daff0 sub_2daff0
    Lifted::new(0x70002db2d0, fn_70002db2d0 as *const (), 2), // +0x2db2d0 sub_2db2d0
    Lifted::new(0x70002db600, fn_70002db600 as *const (), 2), // +0x2db600 sub_2db600
    Lifted::new(0x70002db730, fn_70002db730 as *const (), 2), // +0x2db730 sub_2db730
    Lifted::new(0x70002db920, tpl_299::<0x7000000000, 0x7000694974, 0x700004f420, 0x70005a4c62, 0x700062e498, 0x700062e490> as *const (), 2), // +0x2db920 sub_2db920
    Lifted::new(0x70002dbca0, fn_70002dbca0 as *const (), 3), // +0x2dbca0 sub_2dbca0
    Lifted::new(0x70002dbda0, fn_70002dbda0 as *const (), 0), // +0x2dbda0 sub_2dbda0
    Lifted::new(0x70002dbf70, fn_70002dbf70 as *const (), 0), // +0x2dbf70 sub_2dbf70
    Lifted::new(0x70002dc190, fn_70002dc190 as *const (), 0), // +0x2dc190 sub_2dc190
    Lifted::new(0x70002dc7d0, fn_70002dc7d0 as *const (), 0), // +0x2dc7d0 sub_2dc7d0
    Lifted::new(0x70002dca60, fn_70002dca60 as *const (), 0), // +0x2dca60 sub_2dca60
    Lifted::new(0x70002dcca0, fn_70002dcca0 as *const (), 0), // +0x2dcca0 sub_2dcca0
    Lifted::new(0x70002dcf30, fn_70002dcf30 as *const (), 0), // +0x2dcf30 sub_2dcf30
    Lifted::new(0x70002dd110, fn_70002dd110 as *const (), 0), // +0x2dd110 sub_2dd110
    Lifted::new(0x70002dd770, fn_70002dd770 as *const (), 2), // +0x2dd770 sub_2dd770
    Lifted::new(0x70002ddd50, fn_70002ddd50 as *const (), 2), // +0x2ddd50 sub_2ddd50
    Lifted::new(0x70002ddf80, fn_70002ddf80 as *const (), 2), // +0x2ddf80 sub_2ddf80
    Lifted::new(0x70002de450, fn_70002de450 as *const (), 0), // +0x2de450 sub_2de450
    Lifted::new(0x70002ded90, fn_70002ded90 as *const (), 0), // +0x2ded90 sub_2ded90
    Lifted::new(0x70002deff0, fn_70002deff0 as *const (), 2), // +0x2deff0 sub_2deff0
    Lifted::new(0x70002df980, fn_70002df980 as *const (), 0), // +0x2df980 sub_2df980
    Lifted::new(0x70002dfb80, fn_70002dfb80 as *const (), 2), // +0x2dfb80 sub_2dfb80
    Lifted::new(0x70002e05f0, fn_70002e05f0 as *const (), 2), // +0x2e05f0 sub_2e05f0
    Lifted::new(0x70002e0c90, fn_70002e0c90 as *const (), 2), // +0x2e0c90 sub_2e0c90
    Lifted::new(0x70002e0e90, fn_70002e0e90 as *const (), 2), // +0x2e0e90 sub_2e0e90
    Lifted::new(0x70002e1150, fn_70002e1150 as *const (), 2), // +0x2e1150 sub_2e1150
    Lifted::new(0x70002e1480, fn_70002e1480 as *const (), 2), // +0x2e1480 sub_2e1480
    Lifted::new(0x70002e15c0, fn_70002e15c0 as *const (), 2), // +0x2e15c0 sub_2e15c0
    Lifted::new(0x70002e1bb0, fn_70002e1bb0 as *const (), 0), // +0x2e1bb0 sub_2e1bb0
    Lifted::new(0x70002e1d10, fn_70002e1d10 as *const (), 2), // +0x2e1d10 sub_2e1d10
    Lifted::new(0x70002e2230, fn_70002e2230 as *const (), 2), // +0x2e2230 sub_2e2230
    Lifted::new(0x70002e25e0, fn_70002e25e0 as *const (), 2), // +0x2e25e0 sub_2e25e0
    Lifted::new(0x70002e2e30, fn_70002e2e30 as *const (), 2), // +0x2e2e30 sub_2e2e30
    Lifted::new(0x70002e3530, fn_70002e3530 as *const (), 2), // +0x2e3530 sub_2e3530
    Lifted::new(0x70002e3930, fn_70002e3930 as *const (), 2), // +0x2e3930 sub_2e3930
    Lifted::new(0x70002e3cb0, fn_70002e3cb0 as *const (), 2), // +0x2e3cb0 sub_2e3cb0
    Lifted::new(0x70002e4030, fn_70002e4030 as *const (), 2), // +0x2e4030 sub_2e4030
    Lifted::new(0x70002e4350, fn_70002e4350 as *const (), 2), // +0x2e4350 sub_2e4350
    Lifted::new(0x70002e4840, fn_70002e4840 as *const (), 2), // +0x2e4840 sub_2e4840
    Lifted::new(0x70002e4bf0, fn_70002e4bf0 as *const (), 2), // +0x2e4bf0 sub_2e4bf0
    Lifted::new(0x70002e4de0, fn_70002e4de0 as *const (), 2), // +0x2e4de0 sub_2e4de0
    Lifted::new(0x70002e5100, fn_70002e5100 as *const (), 2), // +0x2e5100 sub_2e5100
    Lifted::new(0x70002e5260, fn_70002e5260 as *const (), 2), // +0x2e5260 sub_2e5260
    Lifted::new(0x70002e5420, tpl_241::<0x70002e0c90, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e5420 sub_2e5420
    Lifted::new(0x70002e5570, tpl_300::<0x70005e061c, 0x70002e8680, 0x70002dd770, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e5570 sub_2e5570
    Lifted::new(0x70002e56e0, tpl_241::<0x70002ddd50, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e56e0 sub_2e56e0
    Lifted::new(0x70002e5830, fn_70002e5830 as *const (), 2), // +0x2e5830 sub_2e5830
    Lifted::new(0x70002e5a70, tpl_301::<0x70005e061c, 0x70002e8680, 0x70005e04e8, 0x70002da4d0, 0x70002deff0, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e5a70 sub_2e5a70
    Lifted::new(0x70002e5c40, fn_70002e5c40 as *const (), 2), // +0x2e5c40 sub_2e5c40
    Lifted::new(0x70002e5ed0, fn_70002e5ed0 as *const (), 2), // +0x2e5ed0 sub_2e5ed0
    Lifted::new(0x70002e60c0, tpl_302::<0x70005e061c, 0x70002e8680, 0x70002df980, 0x7000697150, 0x700062e690, 0x7000262b00, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e60c0 sub_2e60c0
    Lifted::new(0x70002e62a0, fn_70002e62a0 as *const (), 2), // +0x2e62a0 sub_2e62a0
    Lifted::new(0x70002e64e0, fn_70002e64e0 as *const (), 2), // +0x2e64e0 sub_2e64e0
    Lifted::new(0x70002e6770, tpl_300::<0x70005e061c, 0x70002e8680, 0x70002e15c0, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e6770 sub_2e6770
    Lifted::new(0x70002e68e0, tpl_301::<0x70005e061c, 0x70002e8680, 0x70005e0420, 0x70002da4d0, 0x70002e25e0, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e68e0 sub_2e68e0
    Lifted::new(0x70002e6ab0, tpl_301::<0x70005e061c, 0x70002e8680, 0x70005e0420, 0x70002da4d0, 0x70002e2e30, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e6ab0 sub_2e6ab0
    Lifted::new(0x70002e6c80, tpl_302::<0x70005e04c8, 0x70002e8680, 0x70002e1bb0, 0x7000697150, 0x700062e690, 0x7000262b00, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2e6c80 sub_2e6c80
    Lifted::new(0x70002e6e60, fn_70002e6e60 as *const (), 2), // +0x2e6e60 sub_2e6e60
    Lifted::new(0x70002e7020, fn_70002e7020 as *const (), 2), // +0x2e7020 sub_2e7020
    Lifted::new(0x70002e7240, tpl_201::<0x70005a8240, 0x700060afc0, 0x70002445b0, 0x70005a8238, 0x700060af98, 0x70005a8230> as *const (), 0), // +0x2e7240 sub_2e7240
    Lifted::new(0x70002e73a0, tpl_201::<0x70005a8240, 0x700060b0d8, 0x70002445b0, 0x70005a8238, 0x700060af98, 0x70005a8230> as *const (), 0), // +0x2e73a0 sub_2e73a0
    Lifted::new(0x70002e7500, tpl_201::<0x70005a8240, 0x700060b1d8, 0x70002445b0, 0x70005a8238, 0x700060af98, 0x70005a8230> as *const (), 0), // +0x2e7500 sub_2e7500
    Lifted::new(0x70002e7660, tpl_201::<0x70005a8240, 0x700060b1f8, 0x70002445b0, 0x70005a8238, 0x700060af98, 0x70005a8230> as *const (), 0), // +0x2e7660 sub_2e7660
    Lifted::new(0x70002e77c0, fn_70002e77c0 as *const (), 0), // +0x2e77c0 sub_2e77c0
    Lifted::new(0x70002e7870, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x70002e83d0> as *const (), 0), // +0x2e7870 sub_2e7870
    Lifted::new(0x70002e7940, fn_70002e7940 as *const (), 0), // +0x2e7940 sub_2e7940
    Lifted::new(0x70002e7c40, fn_70002e7c40 as *const (), 2), // +0x2e7c40 sub_2e7c40
    Lifted::new(0x70002e7d90, fn_70002e7d90 as *const (), 0), // +0x2e7d90 sub_2e7d90
    Lifted::new(0x70002e7df0, fn_70002e7df0 as *const (), 2), // +0x2e7df0 sub_2e7df0
    Lifted::new(0x70002e80a0, fn_70002e80a0 as *const (), 2), // +0x2e80a0 sub_2e80a0
    Lifted::new(0x70002e83d0, tpl_237::<0x700004de60, 0x7000697150, 0x70002e8730, 0x70002e7870> as *const (), 2), // +0x2e83d0 sub_2e83d0
    Lifted::new(0x70002e8680, fn_70002e8680 as *const (), 0), // +0x2e8680 sub_2e8680
    Lifted::new(0x70002e8700, tpl_74::<0x10, 0x2, 0x70002e8a10> as *const (), 0), // +0x2e8700 sub_2e8700
    Lifted::new(0x70002e8730, tpl_236::<0x70005a4d32, 0x700062e490, 0x70002bc7b0, 0x7000697150, 0x70005f84c0, 0x70002e7870> as *const (), 2), // +0x2e8730 sub_2e8730
    Lifted::new(0x70002e8a10, fn_70002e8a10 as *const (), 2), // +0x2e8a10 sub_2e8a10
    Lifted::new(0x70002e8d40, tpl_245::<0x18, 0x8, 0x20, 0x70002e8dd0> as *const (), 0), // +0x2e8d40 sub_2e8d40
    Lifted::new(0x70002e8dd0, fn_70002e8dd0 as *const (), 2), // +0x2e8dd0 sub_2e8dd0
    Lifted::new(0x70002e90c0, fn_70002e90c0 as *const (), 2), // +0x2e90c0 sub_2e90c0
    Lifted::new(0x70002e9420, fn_70002e9420 as *const (), 0), // +0x2e9420 sub_2e9420
    Lifted::new(0x70002e94a0, fn_70002e94a0 as *const (), 2), // +0x2e94a0 sub_2e94a0
    Lifted::new(0x70002e9a10, fn_70002e9a10 as *const (), 0), // +0x2e9a10 sub_2e9a10
    Lifted::new(0x70002e9d20, fn_70002e9d20 as *const (), 2), // +0x2e9d20 sub_2e9d20
    Lifted::new(0x70002e9f20, fn_70002e9f20 as *const (), 0), // +0x2e9f20 sub_2e9f20
    Lifted::new(0x70002ea0f0, fn_70002ea0f0 as *const (), 2), // +0x2ea0f0 sub_2ea0f0
    Lifted::new(0x70002ea4b0, fn_70002ea4b0 as *const (), 2), // +0x2ea4b0 sub_2ea4b0
    Lifted::new(0x70002ea770, fn_70002ea770 as *const (), 2), // +0x2ea770 sub_2ea770
    Lifted::new(0x70002ea870, fn_70002ea870 as *const (), 2), // +0x2ea870 sub_2ea870
    Lifted::new(0x70002eaa30, tpl_241::<0x70002e9d20, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2eaa30 sub_2eaa30
    Lifted::new(0x70002eab80, tpl_252::<0x70002ebdc0, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2eab80 sub_2eab80
    Lifted::new(0x70002eace0, fn_70002eace0 as *const (), 2), // +0x2eace0 sub_2eace0
    Lifted::new(0x70002eae40, fn_70002eae40 as *const (), 2), // +0x2eae40 sub_2eae40
    Lifted::new(0x70002eb0a0, fn_70002eb0a0 as *const (), 2), // +0x2eb0a0 sub_2eb0a0
    Lifted::new(0x70002eb260, tpl_252::<0x70002eb920, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2eb260 sub_2eb260
    Lifted::new(0x70002eb3c0, tpl_303::<0x70005a8240, 0x700060b500, 0x70002445b0, 0x70005a81b0, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x2eb3c0 sub_2eb3c0
    Lifted::new(0x70002eb540, fn_70002eb540 as *const (), 0), // +0x2eb540 sub_2eb540
    Lifted::new(0x70002eb6d0, tpl_303::<0x70005a8240, 0x700060b560, 0x70002445b0, 0x70005a81b0, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x2eb6d0 sub_2eb6d0
    Lifted::new(0x70002eb850, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x70002ebfc0> as *const (), 0), // +0x2eb850 sub_2eb850
    Lifted::new(0x70002eb920, fn_70002eb920 as *const (), 2), // +0x2eb920 sub_2eb920
    Lifted::new(0x70002ebae0, fn_70002ebae0 as *const (), 2), // +0x2ebae0 sub_2ebae0
    Lifted::new(0x70002ebdc0, fn_70002ebdc0 as *const (), 2), // +0x2ebdc0 sub_2ebdc0
    Lifted::new(0x70002ebfc0, tpl_237::<0x700004de60, 0x7000697150, 0x70002ec340, 0x70002eb850> as *const (), 2), // +0x2ebfc0 sub_2ebfc0
    Lifted::new(0x70002ec270, fn_70002ec270 as *const (), 0), // +0x2ec270 sub_2ec270
    Lifted::new(0x70002ec340, tpl_236::<0x70005a4d32, 0x700062e490, 0x70002bc8b0, 0x7000697150, 0x70005f84c0, 0x70002eb850> as *const (), 2), // +0x2ec340 sub_2ec340
    Lifted::new(0x70002ec620, fn_70002ec620 as *const (), 0), // +0x2ec620 sub_2ec620
    Lifted::new(0x70002ec800, fn_70002ec800 as *const (), 2), // +0x2ec800 sub_2ec800
    Lifted::new(0x70002ec8f0, fn_70002ec8f0 as *const (), 2), // +0x2ec8f0 sub_2ec8f0
    Lifted::new(0x70002ecb30, fn_70002ecb30 as *const (), 0), // +0x2ecb30 sub_2ecb30
    Lifted::new(0x70002ece20, fn_70002ece20 as *const (), 0), // +0x2ece20 sub_2ece20
    Lifted::new(0x70002ed160, fn_70002ed160 as *const (), 0), // +0x2ed160 sub_2ed160
    Lifted::new(0x70002ed490, fn_70002ed490 as *const (), 2), // +0x2ed490 sub_2ed490
    Lifted::new(0x70002ed690, fn_70002ed690 as *const (), 2), // +0x2ed690 sub_2ed690
    Lifted::new(0x70002ed900, fn_70002ed900 as *const (), 2), // +0x2ed900 sub_2ed900
    Lifted::new(0x70002ee290, fn_70002ee290 as *const (), 2), // +0x2ee290 sub_2ee290
    Lifted::new(0x70002ee410, fn_70002ee410 as *const (), 2), // +0x2ee410 sub_2ee410
    Lifted::new(0x70002eeff0, fn_70002eeff0 as *const (), 0), // +0x2eeff0 sub_2eeff0
    Lifted::new(0x70002ef280, fn_70002ef280 as *const (), 2), // +0x2ef280 sub_2ef280
    Lifted::new(0x70002ef740, fn_70002ef740 as *const (), 2), // +0x2ef740 sub_2ef740
    Lifted::new(0x70002ef900, fn_70002ef900 as *const (), 2), // +0x2ef900 sub_2ef900
    Lifted::new(0x70002efbe0, fn_70002efbe0 as *const (), 2), // +0x2efbe0 sub_2efbe0
    Lifted::new(0x70002efdd0, fn_70002efdd0 as *const (), 2), // +0x2efdd0 sub_2efdd0
    Lifted::new(0x70002eff20, fn_70002eff20 as *const (), 2), // +0x2eff20 sub_2eff20
    Lifted::new(0x70002f00e0, tpl_241::<0x70002ed490, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2f00e0 sub_2f00e0
    Lifted::new(0x70002f0230, tpl_294::<0x70005e05e8, 0x70002da4d0, 0x70002ed690, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2f0230 sub_2f0230
    Lifted::new(0x70002f03d0, tpl_241::<0x70002ee290, 0x7000697150, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2f03d0 sub_2f03d0
    Lifted::new(0x70002f0520, fn_70002f0520 as *const (), 2), // +0x2f0520 sub_2f0520
    Lifted::new(0x70002f0870, fn_70002f0870 as *const (), 2), // +0x2f0870 sub_2f0870
    Lifted::new(0x70002f0c60, fn_70002f0c60 as *const (), 2), // +0x2f0c60 sub_2f0c60
    Lifted::new(0x70002f0e50, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x70002f0fc0> as *const (), 0), // +0x2f0e50 sub_2f0e50
    Lifted::new(0x70002f0f20, tpl_210::<0x70002ef280> as *const (), 0), // +0x2f0f20 sub_2f0f20
    Lifted::new(0x70002f0f40, tpl_202::<0x700060b8a8, 0x700028ccf0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x2f0f40 sub_2f0f40
    Lifted::new(0x70002f0fc0, tpl_237::<0x700004de60, 0x7000697150, 0x70002f1270, 0x70002f0e50> as *const (), 2), // +0x2f0fc0 sub_2f0fc0
    Lifted::new(0x70002f1270, tpl_236::<0x70005a4d32, 0x700062e490, 0x70002bc830, 0x7000697150, 0x70005f84c0, 0x70002f0e50> as *const (), 2), // +0x2f1270 sub_2f1270
    Lifted::alias(0x70002f1550, fn_70002eeff0 as *const (), 0, 0x2eeff0), // +0x2f1550 sub_2f1550 (thunk -> +0x2eeff0)
    Lifted::new(0x70002f1560, tpl_251::<0x70006939a8, 0x70005a83d0, 0x70006939a0, 0x8, 0x10> as *const (), 2), // +0x2f1560 sub_2f1560
    Lifted::new(0x70002f15f0, fn_70002f15f0 as *const (), 2), // +0x2f15f0 sub_2f15f0
    Lifted::new(0x70002f1720, fn_70002f1720 as *const (), 2), // +0x2f1720 sub_2f1720
    Lifted::new(0x70002f18c0, fn_70002f18c0 as *const (), 0), // +0x2f18c0 sub_2f18c0
    Lifted::new(0x70002f1980, fn_70002f1980 as *const (), 2), // +0x2f1980 sub_2f1980
    Lifted::new(0x70002f2520, fn_70002f2520 as *const (), 2), // +0x2f2520 sub_2f2520
    Lifted::new(0x70002f3550, fn_70002f3550 as *const (), 2), // +0x2f3550 sub_2f3550
    Lifted::new(0x70002f3c50, fn_70002f3c50 as *const (), 0), // +0x2f3c50 sub_2f3c50
    Lifted::new(0x70002f3d10, fn_70002f3d10 as *const (), 0), // +0x2f3d10 sub_2f3d10
    Lifted::new(0x70002f3fa0, fn_70002f3fa0 as *const (), 0), // +0x2f3fa0 sub_2f3fa0
    Lifted::new(0x70002f4040, tpl_235::<0x8, 0x19, 0x10, 0xfffffffffffffff8, 0x7000697150, 0x70002f4330> as *const (), 0), // +0x2f4040 sub_2f4040
    Lifted::new(0x70002f4100, fn_70002f4100 as *const (), 0), // +0x2f4100 sub_2f4100
    Lifted::new(0x70002f41b0, fn_70002f41b0 as *const (), 2), // +0x2f41b0 sub_2f41b0
    Lifted::new(0x70002f42d0, fn_70002f42d0 as *const (), 0), // +0x2f42d0 sub_2f42d0
    Lifted::new(0x70002f4330, tpl_25::<0x7000694974, 0x700069a930, 0x70005a4174, 0x70005a6720, 0x700069a938, 0x700069a960, 0x700069a988, 0x700069a9b0, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2f4330 sub_2f4330
    Lifted::new(0x70002f43b0, fn_70002f43b0 as *const (), 2), // +0x2f43b0 sub_2f43b0
    Lifted::new(0x70002f4650, tpl_240::<0x70002f3550, 0x7000697150, 0x700062e690, 0x7000262b00, 0x7000021150, 0x70002636c0> as *const (), 2), // +0x2f4650 sub_2f4650
    Lifted::new(0x70002f4810, fn_70002f4810 as *const (), 2), // +0x2f4810 sub_2f4810
    Lifted::new(0x70002f4a30, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x70002f4b00> as *const (), 0), // +0x2f4a30 sub_2f4a30
    Lifted::new(0x70002f4b00, tpl_237::<0x700004de60, 0x7000697150, 0x70002f4e70, 0x70002f4a30> as *const (), 2), // +0x2f4b00 sub_2f4b00
    Lifted::new(0x70002f4db0, tpl_298::<0x7000266260, 0x8, 0x70006974b8, 0x7000606bc8, 0x7000246600, 0x10, 0xe, 0x70001a7a40> as *const (), 0), // +0x2f4db0 sub_2f4db0
    Lifted::new(0x70002f4e70, tpl_236::<0x70005a4d32, 0x700062e490, 0x70002f4330, 0x7000697150, 0x70005f84c0, 0x70002f4a30> as *const (), 2), // +0x2f4e70 sub_2f4e70
    Lifted::new(0x70002f5150, fn_70002f5150 as *const (), 0), // +0x2f5150 sub_2f5150
    Lifted::new(0x70002f5190, fn_70002f5190 as *const (), 0), // +0x2f5190 sub_2f5190
    Lifted::new(0x70002f51d0, fn_70002f51d0 as *const (), 0), // +0x2f51d0 sub_2f51d0
    Lifted::new(0x70002f5350, tpl_304::<0x700048c2c0> as *const (), 0), // +0x2f5350 sub_2f5350
    Lifted::new(0x70002f5380, fn_70002f5380 as *const (), 0), // +0x2f5380 sub_2f5380
    Lifted::new(0x70002f5440, fn_70002f5440 as *const (), 2), // +0x2f5440 sub_2f5440
    Lifted::new(0x70002f56f0, tpl_305::<0x88, 0x4, 0x700060c438, 0xffffffffffffff78, 0xffffffffffffff80, 0xffffffffffffff74, 0x70002f6ad0, 0xffffffffffffff88, 0x70005a8150, 0x70005a8138> as *const (), 0), // +0x2f56f0 sub_2f56f0
    Lifted::new(0x70002f5750, fn_70002f5750 as *const (), 0), // +0x2f5750 sub_2f5750
    Lifted::new(0x70002f5850, fn_70002f5850 as *const (), 2), // +0x2f5850 sub_2f5850
    Lifted::new(0x70002f5890, tpl_138::<0x8> as *const (), 1), // +0x2f5890 sub_2f5890
    Lifted::new(0x70002f58a0, fn_70002f58a0 as *const (), 0), // +0x2f58a0 sub_2f58a0
    Lifted::new(0x70002f58e0, fn_70002f58e0 as *const (), 2), // +0x2f58e0 sub_2f58e0
    Lifted::new(0x70002f5bd0, fn_70002f5bd0 as *const (), 0), // +0x2f5bd0 sub_2f5bd0
    Lifted::new(0x70002f5e00, fn_70002f5e00 as *const (), 2), // +0x2f5e00 sub_2f5e00
    Lifted::new(0x70002f6180, fn_70002f6180 as *const (), 2), // +0x2f6180 sub_2f6180
    Lifted::new(0x70002f64e0, fn_70002f64e0 as *const (), 2), // +0x2f64e0 sub_2f64e0
    Lifted::new(0x70002f6950, fn_70002f6950 as *const (), 0), // +0x2f6950 sub_2f6950
    Lifted::new(0x70002f6ad0, fn_70002f6ad0 as *const (), 0), // +0x2f6ad0 sub_2f6ad0
    Lifted::new(0x70002f6b50, fn_70002f6b50 as *const (), 0), // +0x2f6b50 sub_2f6b50
    Lifted::new(0x70002f6bf0, fn_70002f6bf0 as *const (), 0), // +0x2f6bf0 sub_2f6bf0
    Lifted::new(0x70002f6cd0, tpl_306::<0x70002f6ad0, 0x700048c2c0> as *const (), 0), // +0x2f6cd0 sub_2f6cd0
    Lifted::new(0x70002f6d10, tpl_203::<0xffffffffffffff78, 0x70002f56f0, 0x700048c2c0> as *const (), 0), // +0x2f6d10 sub_2f6d10
    Lifted::new(0x70002f6d50, fn_70002f6d50 as *const (), 0), // +0x2f6d50 sub_2f6d50
    Lifted::new(0x70002f6db0, fn_70002f6db0 as *const (), 2), // +0x2f6db0 sub_2f6db0
    Lifted::new(0x70002f6f00, tpl_25::<0x7000694974, 0x700069a9d8, 0x70005a4174, 0x70005a67b0, 0x700069a9e0, 0x700069aa00, 0x700069aa20, 0x700069aa40, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2f6f00 sub_2f6f00
    Lifted::new(0x70002f6f80, tpl_245::<0x18, 0x8, 0x20, 0x70002f76d0> as *const (), 0), // +0x2f6f80 sub_2f6f80
    Lifted::new(0x70002f7010, fn_70002f7010 as *const (), 2), // +0x2f7010 sub_2f7010
    Lifted::new(0x70002f70f0, fn_70002f70f0 as *const (), 0), // +0x2f70f0 sub_2f70f0
    Lifted::new(0x70002f76d0, fn_70002f76d0 as *const (), 2), // +0x2f76d0 sub_2f76d0
    Lifted::new(0x70002f7b80, fn_70002f7b80 as *const (), 2), // +0x2f7b80 sub_2f7b80
    Lifted::new(0x70002f7e58, tpl_204::<0xfffffffffffffffc, 0x70002f6d10> as *const (), 0), // +0x2f7e58 sub_2f7e58
    Lifted::new(0x70002f7e70, fn_70002f7e70 as *const (), 0), // +0x2f7e70 sub_2f7e70
    Lifted::new(0x70002f7ef0, fn_70002f7ef0 as *const (), 2), // +0x2f7ef0 sub_2f7ef0
    Lifted::new(0x70002f97d0, fn_70002f97d0 as *const (), 2), // +0x2f97d0 sub_2f97d0
    Lifted::new(0x70002fa1f0, fn_70002fa1f0 as *const (), 2), // +0x2fa1f0 sub_2fa1f0
    Lifted::new(0x70002fa5b0, tpl_307::<0x8, 0x700029bac0, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x2fa5b0 sub_2fa5b0
    Lifted::new(0x70002fa620, tpl_135::<0x7000233000, 0x10, 0x8, 0x18> as *const (), 0), // +0x2fa620 sub_2fa620
    Lifted::new(0x70002fa690, tpl_308::<0x70005a8240, 0x700060c5c0, 0x70002445b0, 0x70005a8230> as *const (), 0), // +0x2fa690 sub_2fa690
    Lifted::new(0x70002fa7b0, tpl_115::<0x18, 0x8, 0x20, 0x70002fa860> as *const (), 0), // +0x2fa7b0 sub_2fa7b0
    Lifted::new(0x70002fa860, fn_70002fa860 as *const (), 2), // +0x2fa860 sub_2fa860
    Lifted::new(0x70002fac30, fn_70002fac30 as *const (), 2), // +0x2fac30 sub_2fac30
    Lifted::new(0x70002faef0, fn_70002faef0 as *const (), 2), // +0x2faef0 sub_2faef0
    Lifted::new(0x70002fb7d0, fn_70002fb7d0 as *const (), 2), // +0x2fb7d0 sub_2fb7d0
    Lifted::new(0x70002fc1a0, fn_70002fc1a0 as *const (), 2), // +0x2fc1a0 sub_2fc1a0
    Lifted::new(0x70002fcc30, fn_70002fcc30 as *const (), 0), // +0x2fcc30 sub_2fcc30
    Lifted::new(0x70002fcd90, fn_70002fcd90 as *const (), 0), // +0x2fcd90 sub_2fcd90
    Lifted::new(0x70002fcef0, fn_70002fcef0 as *const (), 0), // +0x2fcef0 sub_2fcef0
    Lifted::new(0x70002fcfe0, tpl_74::<0x10, 0x2, 0x70002fd010> as *const (), 0), // +0x2fcfe0 sub_2fcfe0
    Lifted::new(0x70002fd010, fn_70002fd010 as *const (), 2), // +0x2fd010 sub_2fd010
    Lifted::new(0x70002fd330, tpl_137::<0x70005a4d1a, 0x7000694974, 0x700004f420> as *const (), 0), // +0x2fd330 sub_2fd330
    Lifted::new(0x70002fd510, fn_70002fd510 as *const (), 1), // +0x2fd510 sub_2fd510
    Lifted::new(0x70002fd5b0, tpl_25::<0x7000694974, 0x7000699058, 0x70005a4174, 0x70005a67c0, 0x7000699060, 0x7000699080, 0x70006990a0, 0x70006990c0, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x2fd5b0 sub_2fd5b0
    Lifted::new(0x70002fd630, tpl_67::<0x18, 0x8, 0x20, 0x70002fd940> as *const (), 0), // +0x2fd630 sub_2fd630
    Lifted::new(0x70002fd6c0, fn_70002fd6c0 as *const (), 1), // +0x2fd6c0 sub_2fd6c0
    Lifted::new(0x70002fd790, fn_70002fd790 as *const (), 1), // +0x2fd790 sub_2fd790
    Lifted::new(0x70002fd870, fn_70002fd870 as *const (), 3), // +0x2fd870 sub_2fd870
    Lifted::new(0x70002fd940, fn_70002fd940 as *const (), 2), // +0x2fd940 sub_2fd940
    Lifted::new(0x70002fdca0, fn_70002fdca0 as *const (), 2), // +0x2fdca0 sub_2fdca0
    Lifted::new(0x70002fe010, fn_70002fe010 as *const (), 0), // +0x2fe010 sub_2fe010
    Lifted::new(0x70002fe070, fn_70002fe070 as *const (), 2), // +0x2fe070 sub_2fe070
    Lifted::new(0x70002fe9e0, fn_70002fe9e0 as *const (), 2), // +0x2fe9e0 sub_2fe9e0
    Lifted::new(0x70002fede0, fn_70002fede0 as *const (), 2), // +0x2fede0 sub_2fede0
    Lifted::new(0x70002fef50, fn_70002fef50 as *const (), 3), // +0x2fef50 sub_2fef50
    Lifted::new(0x70002fefe0, fn_70002fefe0 as *const (), 2), // +0x2fefe0 sub_2fefe0
    Lifted::new(0x70002ff100, fn_70002ff100 as *const (), 1), // +0x2ff100 sub_2ff100
    Lifted::new(0x70002ffc80, fn_70002ffc80 as *const (), 0), // +0x2ffc80 sub_2ffc80
    Lifted::new(0x70002ffe90, fn_70002ffe90 as *const (), 2), // +0x2ffe90 sub_2ffe90
    Lifted::new(0x7000300120, fn_7000300120 as *const (), 0), // +0x300120 sub_300120
    Lifted::new(0x7000300870, fn_7000300870 as *const (), 2), // +0x300870 sub_300870
    Lifted::new(0x70003009f0, fn_70003009f0 as *const (), 0), // +0x3009f0 sub_3009f0
    Lifted::new(0x7000300b40, tpl_242::<0x1, 0x10, 0x700010a310, 0x70001c3390> as *const (), 0), // +0x300b40 sub_300b40
    Lifted::new(0x7000300c10, fn_7000300c10 as *const (), 0), // +0x300c10 sub_300c10
    Lifted::new(0x7000300f70, tpl_76::<0x8, 0x1, 0x10, 0x70003010b0> as *const (), 0), // +0x300f70 sub_300f70
    Lifted::new(0x7000301040, tpl_77::<0x10, 0x70003010b0> as *const (), 0), // +0x301040 sub_301040
    Lifted::new(0x70003010b0, fn_70003010b0 as *const (), 2), // +0x3010b0 sub_3010b0
    Lifted::new(0x7000301470, fn_7000301470 as *const (), 1), // +0x301470 sub_301470
    Lifted::new(0x70003014a0, fn_70003014a0 as *const (), 0), // +0x3014a0 sub_3014a0
    Lifted::new(0x7000301740, fn_7000301740 as *const (), 0), // +0x301740 sub_301740
    Lifted::new(0x7000301990, tpl_309::<0x1, 0x10, 0x700010a310, 0x7000301040> as *const (), 0), // +0x301990 sub_301990
    Lifted::new(0x7000301a10, fn_7000301a10 as *const (), 0), // +0x301a10 sub_301a10
    Lifted::new(0x7000301a80, fn_7000301a80 as *const (), 2), // +0x301a80 sub_301a80
    Lifted::new(0x7000301b40, tpl_139::<0x38, 0x40, 0x48, 0x50, 0x18, 0x20, 0x28, 0x30, 0x7000302100, 0x70003021a0> as *const (), 0), // +0x301b40 sub_301b40
    Lifted::new(0x7000301bb0, fn_7000301bb0 as *const (), 0), // +0x301bb0 sub_301bb0
    Lifted::new(0x7000301d00, fn_7000301d00 as *const (), 0), // +0x301d00 sub_301d00
    Lifted::new(0x7000301df0, fn_7000301df0 as *const (), 0), // +0x301df0 sub_301df0
    Lifted::new(0x7000301eb0, fn_7000301eb0 as *const (), 2), // +0x301eb0 sub_301eb0
    Lifted::new(0x7000302030, fn_7000302030 as *const (), 0), // +0x302030 sub_302030
    Lifted::new(0x7000302100, fn_7000302100 as *const (), 0), // +0x302100 sub_302100
    Lifted::new(0x70003021a0, fn_70003021a0 as *const (), 0), // +0x3021a0 sub_3021a0
    Lifted::new(0x7000302220, fn_7000302220 as *const (), 1), // +0x302220 sub_302220
    Lifted::new(0x7000302240, fn_7000302240 as *const (), 0), // +0x302240 sub_302240
    Lifted::new(0x70003022a0, fn_70003022a0 as *const (), 0), // +0x3022a0 sub_3022a0
    Lifted::new(0x7000302ea0, fn_7000302ea0 as *const (), 0), // +0x302ea0 sub_302ea0
    Lifted::new(0x7000303080, fn_7000303080 as *const (), 0), // +0x303080 sub_303080
    Lifted::new(0x70003031b0, fn_70003031b0 as *const (), 0), // +0x3031b0 sub_3031b0
    Lifted::new(0x70003032e0, fn_70003032e0 as *const (), 1), // +0x3032e0 sub_3032e0
    Lifted::new(0x70003032f0, fn_70003032f0 as *const (), 1), // +0x3032f0 sub_3032f0
    Lifted::new(0x7000303390, fn_7000303390 as *const (), 2), // +0x303390 sub_303390
    Lifted::new(0x7000303680, fn_7000303680 as *const (), 0), // +0x303680 sub_303680
    Lifted::new(0x70003036e0, fn_70003036e0 as *const (), 0), // +0x3036e0 sub_3036e0
    Lifted::new(0x7000303920, fn_7000303920 as *const (), 2), // +0x303920 sub_303920
    Lifted::new(0x7000303d70, fn_7000303d70 as *const (), 2), // +0x303d70 sub_303d70
    Lifted::new(0x7000304310, fn_7000304310 as *const (), 2), // +0x304310 sub_304310
    Lifted::new(0x7000304d00, fn_7000304d00 as *const (), 0), // +0x304d00 sub_304d00
    Lifted::new(0x7000304e90, fn_7000304e90 as *const (), 2), // +0x304e90 sub_304e90
    Lifted::new(0x70003057b0, fn_70003057b0 as *const (), 2), // +0x3057b0 sub_3057b0
    Lifted::new(0x7000305860, fn_7000305860 as *const (), 0), // +0x305860 sub_305860
    Lifted::new(0x70003058e0, tpl_209::<0x8, 0x7000697150, 0x10> as *const (), 0), // +0x3058e0 sub_3058e0
    Lifted::new(0x7000305940, fn_7000305940 as *const (), 0), // +0x305940 sub_305940
    Lifted::new(0x70003059d0, fn_70003059d0 as *const (), 0), // +0x3059d0 sub_3059d0
    Lifted::new(0x7000305a40, fn_7000305a40 as *const (), 2), // +0x305a40 sub_305a40
    Lifted::new(0x7000305f60, fn_7000305f60 as *const (), 2), // +0x305f60 sub_305f60
    Lifted::new(0x7000306370, fn_7000306370 as *const (), 2), // +0x306370 sub_306370
    Lifted::new(0x7000306440, fn_7000306440 as *const (), 2), // +0x306440 sub_306440
    Lifted::new(0x70003066e0, fn_70003066e0 as *const (), 0), // +0x3066e0 sub_3066e0
    Lifted::new(0x7000306740, fn_7000306740 as *const (), 2), // +0x306740 sub_306740
    Lifted::new(0x7000306a00, fn_7000306a00 as *const (), 0), // +0x306a00 sub_306a00
    Lifted::new(0x7000306c70, fn_7000306c70 as *const (), 2), // +0x306c70 sub_306c70
    Lifted::new(0x7000307240, fn_7000307240 as *const (), 2), // +0x307240 sub_307240
    Lifted::new(0x7000307650, fn_7000307650 as *const (), 0), // +0x307650 sub_307650
    Lifted::new(0x70003076c0, fn_70003076c0 as *const (), 2), // +0x3076c0 sub_3076c0
    Lifted::new(0x7000307a40, fn_7000307a40 as *const (), 0), // +0x307a40 sub_307a40
    Lifted::new(0x7000307d50, fn_7000307d50 as *const (), 2), // +0x307d50 sub_307d50
    Lifted::new(0x7000308620, fn_7000308620 as *const (), 0), // +0x308620 sub_308620
    Lifted::new(0x70003089b0, fn_70003089b0 as *const (), 0), // +0x3089b0 sub_3089b0
    Lifted::new(0x7000308d50, fn_7000308d50 as *const (), 2), // +0x308d50 sub_308d50
    Lifted::new(0x7000309100, fn_7000309100 as *const (), 0), // +0x309100 sub_309100
    Lifted::new(0x7000309180, fn_7000309180 as *const (), 0), // +0x309180 sub_309180
    Lifted::new(0x7000309250, fn_7000309250 as *const (), 2), // +0x309250 sub_309250
    Lifted::new(0x7000309890, fn_7000309890 as *const (), 0), // +0x309890 sub_309890
    Lifted::new(0x7000309b30, fn_7000309b30 as *const (), 0), // +0x309b30 sub_309b30
    Lifted::new(0x7000309bb0, tpl_310::<0x700030a620, 0x700004de60> as *const (), 0), // +0x309bb0 sub_309bb0
    Lifted::new(0x7000309cd0, fn_7000309cd0 as *const (), 0), // +0x309cd0 sub_309cd0
    Lifted::new(0x7000309d40, fn_7000309d40 as *const (), 2), // +0x309d40 sub_309d40
    Lifted::new(0x7000309e30, fn_7000309e30 as *const (), 2), // +0x309e30 sub_309e30
    Lifted::new(0x700030a020, tpl_256::<0x8, 0x19, 0x10, 0x7000307650, 0x20> as *const (), 0), // +0x30a020 sub_30a020
    Lifted::new(0x700030a0b0, fn_700030a0b0 as *const (), 0), // +0x30a0b0 sub_30a0b0
    Lifted::new(0x700030a170, tpl_311::<0x8, 0x700030afe0> as *const (), 2), // +0x30a170 sub_30a170
    Lifted::new(0x700030a230, fn_700030a230 as *const (), 0), // +0x30a230 sub_30a230
    Lifted::new(0x700030a340, fn_700030a340 as *const (), 0), // +0x30a340 sub_30a340
    Lifted::new(0x700030a400, fn_700030a400 as *const (), 2), // +0x30a400 sub_30a400
    Lifted::new(0x700030a500, fn_700030a500 as *const (), 0), // +0x30a500 sub_30a500
    Lifted::new(0x700030a5a0, fn_700030a5a0 as *const (), 0), // +0x30a5a0 sub_30a5a0
    Lifted::new(0x700030a620, tpl_312::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x700004de60, 0x700030a5a0> as *const (), 2), // +0x30a620 sub_30a620
    Lifted::new(0x700030a780, fn_700030a780 as *const (), 2), // +0x30a780 sub_30a780
    Lifted::new(0x700030a870, fn_700030a870 as *const (), 0), // +0x30a870 sub_30a870
    Lifted::new(0x700030a8f0, fn_700030a8f0 as *const (), 2), // +0x30a8f0 sub_30a8f0
    Lifted::new(0x700030a9f0, fn_700030a9f0 as *const (), 0), // +0x30a9f0 sub_30a9f0
    Lifted::new(0x700030aa90, tpl_24::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x700030ab30> as *const (), 2), // +0x30aa90 sub_30aa90
    Lifted::new(0x700030ab30, fn_700030ab30 as *const (), 0), // +0x30ab30 sub_30ab30
    Lifted::new(0x700030abe0, fn_700030abe0 as *const (), 2), // +0x30abe0 sub_30abe0
    Lifted::new(0x700030ace0, fn_700030ace0 as *const (), 0), // +0x30ace0 sub_30ace0
    Lifted::new(0x700030ad60, fn_700030ad60 as *const (), 2), // +0x30ad60 sub_30ad60
    Lifted::new(0x700030adf0, tpl_41::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x700030b760> as *const (), 2), // +0x30adf0 sub_30adf0
    Lifted::new(0x700030aea0, fn_700030aea0 as *const (), 0), // +0x30aea0 sub_30aea0
    Lifted::new(0x700030af30, fn_700030af30 as *const (), 0), // +0x30af30 sub_30af30
    Lifted::new(0x700030afe0, fn_700030afe0 as *const (), 2), // +0x30afe0 sub_30afe0
    Lifted::new(0x700030b0f0, fn_700030b0f0 as *const (), 2), // +0x30b0f0 sub_30b0f0
    Lifted::new(0x700030b200, fn_700030b200 as *const (), 0), // +0x30b200 sub_30b200
    Lifted::new(0x700030b2a0, tpl_243::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x700030b340> as *const (), 2), // +0x30b2a0 sub_30b2a0
    Lifted::new(0x700030b340, fn_700030b340 as *const (), 0), // +0x30b340 sub_30b340
    Lifted::new(0x700030b3e0, tpl_25::<0x7000694974, 0x700069aa60, 0x70005a4174, 0x70005a6980, 0x700069aa68, 0x700069aa74, 0x700069aa80, 0x700069aa8c, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x30b3e0 sub_30b3e0
    Lifted::new(0x700030b460, tpl_25::<0x7000694974, 0x700069aa98, 0x70005a4174, 0x70005a6970, 0x700069aaa0, 0x700069ab28, 0x700069abb0, 0x700069ac38, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x30b460 sub_30b460
    Lifted::new(0x700030b4e0, tpl_25::<0x7000694974, 0x700069acc0, 0x70005a4174, 0x70005a6960, 0x700069acc8, 0x700069ad80, 0x700069ae38, 0x700069aef0, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x30b4e0 sub_30b4e0
    Lifted::new(0x700030b560, tpl_25::<0x7000694974, 0x700069afa8, 0x70005a4174, 0x70005a6950, 0x700069afb0, 0x700069afd8, 0x700069b000, 0x700069b028, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x30b560 sub_30b560
    Lifted::new(0x700030b5e0, fn_700030b5e0 as *const (), 2), // +0x30b5e0 sub_30b5e0
    Lifted::new(0x700030b760, fn_700030b760 as *const (), 0), // +0x30b760 sub_30b760
    Lifted::new(0x700030b7e0, fn_700030b7e0 as *const (), 0), // +0x30b7e0 sub_30b7e0
    Lifted::new(0x700030ba10, tpl_313::<0x70005a8240, 0x700060c7f8, 0x70002445b0, 0x700019fc10, 0x70005a81b0, 0x70005a8210, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x30ba10 sub_30ba10
    Lifted::new(0x700030bc20, fn_700030bc20 as *const (), 0), // +0x30bc20 sub_30bc20
    Lifted::new(0x700030bdc0, tpl_228::<0x70005a8240, 0x70005f84a0, 0x70002445b0, 0x700024f970, 0x70005a8210, 0x70005a8230> as *const (), 0), // +0x30bdc0 sub_30bdc0
    Lifted::new(0x700030bf20, fn_700030bf20 as *const (), 0), // +0x30bf20 sub_30bf20
    Lifted::new(0x700030c080, fn_700030c080 as *const (), 0), // +0x30c080 sub_30c080
    Lifted::new(0x700030c240, fn_700030c240 as *const (), 0), // +0x30c240 sub_30c240
    Lifted::new(0x700030c3f0, fn_700030c3f0 as *const (), 2), // +0x30c3f0 sub_30c3f0
    Lifted::new(0x700030c6b0, fn_700030c6b0 as *const (), 0), // +0x30c6b0 sub_30c6b0
    Lifted::new(0x700030c800, fn_700030c800 as *const (), 0), // +0x30c800 sub_30c800
    Lifted::new(0x700030c970, fn_700030c970 as *const (), 0), // +0x30c970 sub_30c970
    Lifted::new(0x700030caf0, fn_700030caf0 as *const (), 0), // +0x30caf0 sub_30caf0
    Lifted::new(0x700030cc70, tpl_201::<0x70005a8240, 0x700060cf20, 0x70002445b0, 0x70005a8210, 0x700060cf10, 0x70005a8230> as *const (), 0), // +0x30cc70 sub_30cc70
    Lifted::new(0x700030cdd0, fn_700030cdd0 as *const (), 0), // +0x30cdd0 sub_30cdd0
    Lifted::new(0x700030cf50, fn_700030cf50 as *const (), 0), // +0x30cf50 sub_30cf50
    Lifted::new(0x700030d0b0, fn_700030d0b0 as *const (), 0), // +0x30d0b0 sub_30d0b0
    Lifted::new(0x700030d230, fn_700030d230 as *const (), 0), // +0x30d230 sub_30d230
    Lifted::new(0x700030d410, fn_700030d410 as *const (), 0), // +0x30d410 sub_30d410
    Lifted::new(0x700030d500, fn_700030d500 as *const (), 0), // +0x30d500 sub_30d500
    Lifted::new(0x700030d590, fn_700030d590 as *const (), 3), // +0x30d590 sub_30d590
    Lifted::new(0x700030d640, tpl_262::<0x18, 0x8, 0x88, 0x700030e5f0> as *const (), 0), // +0x30d640 sub_30d640
    Lifted::new(0x700030d6d0, fn_700030d6d0 as *const (), 0), // +0x30d6d0 sub_30d6d0
    Lifted::new(0x700030d760, tpl_73::<0x7000007030, 0x700012fa40, 0x700030ea90> as *const (), 0), // +0x30d760 sub_30d760
    Lifted::new(0x700030d850, fn_700030d850 as *const (), 2), // +0x30d850 sub_30d850
    Lifted::new(0x700030d8f0, fn_700030d8f0 as *const (), 2), // +0x30d8f0 sub_30d8f0
    Lifted::new(0x700030da10, fn_700030da10 as *const (), 0), // +0x30da10 sub_30da10
    Lifted::new(0x700030dae0, fn_700030dae0 as *const (), 0), // +0x30dae0 sub_30dae0
    Lifted::new(0x700030db50, fn_700030db50 as *const (), 0), // +0x30db50 sub_30db50
    Lifted::new(0x700030dbe0, fn_700030dbe0 as *const (), 2), // +0x30dbe0 sub_30dbe0
    Lifted::new(0x700030dcd0, fn_700030dcd0 as *const (), 0), // +0x30dcd0 sub_30dcd0
    Lifted::new(0x700030dde0, fn_700030dde0 as *const (), 0), // +0x30dde0 sub_30dde0
    Lifted::new(0x700030de10, fn_700030de10 as *const (), 2), // +0x30de10 sub_30de10
    Lifted::new(0x700030dea0, fn_700030dea0 as *const (), 0), // +0x30dea0 sub_30dea0
    Lifted::new(0x700030ded0, tpl_210::<0x7000304d00> as *const (), 0), // +0x30ded0 sub_30ded0
    Lifted::new(0x700030def0, tpl_314::<0x700060d398, 0x700030ace0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x30def0 sub_30def0
    Lifted::new(0x700030df70, fn_700030df70 as *const (), 0), // +0x30df70 sub_30df70
    Lifted::new(0x700030e050, fn_700030e050 as *const (), 0), // +0x30e050 sub_30e050
    Lifted::new(0x700030e0f0, fn_700030e0f0 as *const (), 0), // +0x30e0f0 sub_30e0f0
    Lifted::new(0x700030e150, tpl_314::<0x700060d3d8, 0x700019f140, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x30e150 sub_30e150
    Lifted::new(0x700030e1d0, tpl_202::<0x700060d3e8, 0x70003058e0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x30e1d0 sub_30e1d0
    Lifted::new(0x700030e250, tpl_202::<0x700060d3f8, 0x70003059d0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x30e250 sub_30e250
    Lifted::new(0x700030e2d0, fn_700030e2d0 as *const (), 0), // +0x30e2d0 sub_30e2d0
    Lifted::new(0x700030e350, tpl_314::<0x700060d418, 0x700048beb0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x30e350 sub_30e350
    Lifted::new(0x700030e3d0, fn_700030e3d0 as *const (), 2), // +0x30e3d0 sub_30e3d0
    Lifted::new(0x700030e5f0, fn_700030e5f0 as *const (), 2), // +0x30e5f0 sub_30e5f0
    Lifted::new(0x700030e840, fn_700030e840 as *const (), 2), // +0x30e840 sub_30e840
    Lifted::new(0x700030ea90, fn_700030ea90 as *const (), 2), // +0x30ea90 sub_30ea90
    Lifted::new(0x700030ed30, tpl_74::<0x10, 0x2, 0x700030f590> as *const (), 0), // +0x30ed30 sub_30ed30
    Lifted::new(0x700030ed60, fn_700030ed60 as *const (), 2), // +0x30ed60 sub_30ed60
    Lifted::new(0x700030f010, tpl_261::<0x70005a4d32, 0x700062e490, 0x700030b460, 0x7000697150, 0x70005f84c0, 0x700030d640, 0x7000307650> as *const (), 2), // +0x30f010 sub_30f010
    Lifted::new(0x700030f2d0, fn_700030f2d0 as *const (), 2), // +0x30f2d0 sub_30f2d0
    Lifted::new(0x700030f590, fn_700030f590 as *const (), 2), // +0x30f590 sub_30f590
    Lifted::new(0x700030f870, fn_700030f870 as *const (), 0), // +0x30f870 sub_30f870
    Lifted::new(0x700030fae0, fn_700030fae0 as *const (), 2), // +0x30fae0 sub_30fae0
    Lifted::new(0x700030fd50, fn_700030fd50 as *const (), 2), // +0x30fd50 sub_30fd50
    Lifted::new(0x700030fea0, fn_700030fea0 as *const (), 0), // +0x30fea0 sub_30fea0
    Lifted::new(0x700030ff90, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x7000310640> as *const (), 0), // +0x30ff90 sub_30ff90
    Lifted::new(0x7000310060, fn_7000310060 as *const (), 3), // +0x310060 sub_310060
    Lifted::new(0x70003100d0, fn_70003100d0 as *const (), 0), // +0x3100d0 sub_3100d0
    Lifted::alias(0x7000310150, fn_7000304e90 as *const (), 2, 0x304e90), // +0x310150 sub_310150 (thunk -> +0x304e90)
    Lifted::new(0x7000310160, tpl_251::<0x70006939e8, 0x70005a83d0, 0x70006939e0, 0x8, 0x10> as *const (), 2), // +0x310160 sub_310160
    Lifted::new(0x70003101f0, fn_70003101f0 as *const (), 0), // +0x3101f0 sub_3101f0
    Lifted::new(0x7000310360, fn_7000310360 as *const (), 0), // +0x310360 sub_310360
    Lifted::new(0x7000310640, fn_7000310640 as *const (), 2), // +0x310640 sub_310640
    Lifted::new(0x70003108a0, fn_70003108a0 as *const (), 2), // +0x3108a0 sub_3108a0
    Lifted::new(0x7000310960, fn_7000310960 as *const (), 2), // +0x310960 sub_310960
    Lifted::new(0x7000310aa0, fn_7000310aa0 as *const (), 1), // +0x310aa0 sub_310aa0
    Lifted::new(0x7000310b60, fn_7000310b60 as *const (), 1), // +0x310b60 sub_310b60
    Lifted::new(0x7000310ca0, fn_7000310ca0 as *const (), 1), // +0x310ca0 sub_310ca0
    Lifted::new(0x7000310dc0, fn_7000310dc0 as *const (), 1), // +0x310dc0 sub_310dc0
    Lifted::new(0x7000310e70, tpl_176::<0x7000603388, 0x700048c2c0> as *const (), 0), // +0x310e70 sub_310e70
    Lifted::new(0x7000310ea0, fn_7000310ea0 as *const (), 2), // +0x310ea0 sub_310ea0
    Lifted::new(0x7000310f40, tpl_9::<0x700060d440> as *const (), 1), // +0x310f40 sub_310f40
    Lifted::new(0x7000310f50, fn_7000310f50 as *const (), 0), // +0x310f50 sub_310f50
    Lifted::new(0x7000310f90, tpl_214::<0x700060d448, 0x8, 0x7000603350> as *const (), 1), // +0x310f90 sub_310f90
    Lifted::new(0x7000310fb0, fn_7000310fb0 as *const (), 0), // +0x310fb0 sub_310fb0
    Lifted::new(0x7000310fd0, fn_7000310fd0 as *const (), 2), // +0x310fd0 sub_310fd0
    Lifted::new(0x7000311130, fn_7000311130 as *const (), 1), // +0x311130 sub_311130
    Lifted::new(0x7000311150, fn_7000311150 as *const (), 1), // +0x311150 sub_311150
    Lifted::new(0x7000311160, fn_7000311160 as *const (), 0), // +0x311160 sub_311160
    Lifted::new(0x70003111a0, fn_70003111a0 as *const (), 1), // +0x3111a0 sub_3111a0
    Lifted::new(0x70003111c0, fn_70003111c0 as *const (), 2), // +0x3111c0 sub_3111c0
    Lifted::new(0x70003111f0, fn_70003111f0 as *const (), 0), // +0x3111f0 sub_3111f0
    Lifted::new(0x70003112e0, fn_70003112e0 as *const (), 0), // +0x3112e0 sub_3112e0
    Lifted::new(0x7000311660, fn_7000311660 as *const (), 2), // +0x311660 sub_311660
    Lifted::new(0x7000311b50, fn_7000311b50 as *const (), 2), // +0x311b50 sub_311b50
    Lifted::new(0x7000311d30, fn_7000311d30 as *const (), 0), // +0x311d30 sub_311d30
    Lifted::new(0x7000312090, fn_7000312090 as *const (), 2), // +0x312090 sub_312090
    Lifted::new(0x7000312630, fn_7000312630 as *const (), 0), // +0x312630 sub_312630
    Lifted::new(0x70003128f0, fn_70003128f0 as *const (), 0), // +0x3128f0 sub_3128f0
    Lifted::new(0x7000312bd0, fn_7000312bd0 as *const (), 0), // +0x312bd0 sub_312bd0
    Lifted::new(0x7000312c40, fn_7000312c40 as *const (), 2), // +0x312c40 sub_312c40
    Lifted::new(0x7000312ec0, fn_7000312ec0 as *const (), 2), // +0x312ec0 sub_312ec0
    Lifted::new(0x7000313330, fn_7000313330 as *const (), 0), // +0x313330 sub_313330
    Lifted::new(0x70003135a0, fn_70003135a0 as *const (), 0), // +0x3135a0 sub_3135a0
    Lifted::new(0x7000313730, fn_7000313730 as *const (), 0), // +0x313730 sub_313730
    Lifted::new(0x7000313880, fn_7000313880 as *const (), 0), // +0x313880 sub_313880
    Lifted::new(0x70003139c0, fn_70003139c0 as *const (), 0), // +0x3139c0 sub_3139c0
    Lifted::new(0x7000313a50, fn_7000313a50 as *const (), 0), // +0x313a50 sub_313a50
    Lifted::new(0x7000313b00, fn_7000313b00 as *const (), 0), // +0x313b00 sub_313b00
    Lifted::new(0x7000313be0, fn_7000313be0 as *const (), 0), // +0x313be0 sub_313be0
    Lifted::new(0x7000313c80, tpl_311::<0x8, 0x7000313f80> as *const (), 2), // +0x313c80 sub_313c80
    Lifted::new(0x7000313d40, fn_7000313d40 as *const (), 0), // +0x313d40 sub_313d40
    Lifted::new(0x7000313dc0, fn_7000313dc0 as *const (), 2), // +0x313dc0 sub_313dc0
    Lifted::new(0x7000313ed0, fn_7000313ed0 as *const (), 0), // +0x313ed0 sub_313ed0
    Lifted::new(0x7000313f80, fn_7000313f80 as *const (), 2), // +0x313f80 sub_313f80
    Lifted::new(0x7000314090, tpl_25::<0x7000694974, 0x700069b050, 0x70005a4174, 0x70005a69a0, 0x700069b060, 0x700069b078, 0x700069b090, 0x700069b0a8, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x314090 sub_314090
    Lifted::new(0x7000314110, tpl_25::<0x7000694974, 0x700069b0c0, 0x70005a4174, 0x70005a6990, 0x700069b0d0, 0x700069b0f8, 0x700069b120, 0x700069b148, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x314110 sub_314110
    Lifted::new(0x7000314190, fn_7000314190 as *const (), 0), // +0x314190 sub_314190
    Lifted::new(0x7000314300, tpl_313::<0x70005a8240, 0x700060d490, 0x70002445b0, 0x700019fc10, 0x70005a81b0, 0x70005a8210, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x314300 sub_314300
    Lifted::new(0x7000314510, fn_7000314510 as *const (), 0), // +0x314510 sub_314510
    Lifted::new(0x70003146a0, fn_70003146a0 as *const (), 0), // +0x3146a0 sub_3146a0
    Lifted::new(0x70003147f0, tpl_315::<0x70005a8240, 0x700060d660, 0x70002445b0, 0x700060d618, 0x70005a8230> as *const (), 0), // +0x3147f0 sub_3147f0
    Lifted::new(0x7000314940, tpl_273::<0x7000692760, 0x7000007030, 0x700048bd10, 0x7000314d60> as *const (), 0), // +0x314940 sub_314940
    Lifted::new(0x7000314a30, tpl_316::<0x18, 0x8, 0x7000315010> as *const (), 0), // +0x314a30 sub_314a30
    Lifted::new(0x7000314ae0, tpl_244::<0x18, 0x8, 0x7000315330> as *const (), 0), // +0x314ae0 sub_314ae0
    Lifted::new(0x7000314b80, fn_7000314b80 as *const (), 2), // +0x314b80 sub_314b80
    Lifted::new(0x7000314ca0, tpl_210::<0x70003128f0> as *const (), 0), // +0x314ca0 sub_314ca0
    Lifted::new(0x7000314cc0, tpl_210::<0x7000311d30> as *const (), 0), // +0x314cc0 sub_314cc0
    Lifted::new(0x7000314ce0, tpl_202::<0x700060d680, 0x70003058e0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x314ce0 sub_314ce0
    Lifted::new(0x7000314d60, fn_7000314d60 as *const (), 2), // +0x314d60 sub_314d60
    Lifted::new(0x7000315010, fn_7000315010 as *const (), 2), // +0x315010 sub_315010
    Lifted::new(0x7000315330, fn_7000315330 as *const (), 2), // +0x315330 sub_315330
    Lifted::new(0x70003155f0, fn_70003155f0 as *const (), 2), // +0x3155f0 sub_3155f0
    Lifted::new(0x70003158d0, fn_70003158d0 as *const (), 2), // +0x3158d0 sub_3158d0
    Lifted::new(0x7000315b90, fn_7000315b90 as *const (), 0), // +0x315b90 sub_315b90
    Lifted::alias(0x7000315ca0, fn_7000312630 as *const (), 0, 0x312630), // +0x315ca0 sub_315ca0 (thunk -> +0x312630)
    Lifted::new(0x7000315cb0, tpl_251::<0x7000693a28, 0x70005a83d0, 0x7000693a20, 0x8, 0x10> as *const (), 2), // +0x315cb0 sub_315cb0
    Lifted::new(0x7000315d40, fn_7000315d40 as *const (), 0), // +0x315d40 sub_315d40
    Lifted::new(0x7000315e10, fn_7000315e10 as *const (), 0), // +0x315e10 sub_315e10
    Lifted::new(0x7000315f80, fn_7000315f80 as *const (), 2), // +0x315f80 sub_315f80
    Lifted::new(0x70003163d0, fn_70003163d0 as *const (), 0), // +0x3163d0 sub_3163d0
    Lifted::new(0x7000316760, fn_7000316760 as *const (), 0), // +0x316760 sub_316760
    Lifted::new(0x70003168b0, fn_70003168b0 as *const (), 0), // +0x3168b0 sub_3168b0
    Lifted::new(0x70003169b0, fn_70003169b0 as *const (), 0), // +0x3169b0 sub_3169b0
    Lifted::new(0x7000316a60, fn_7000316a60 as *const (), 2), // +0x316a60 sub_316a60
    Lifted::new(0x7000316e40, tpl_266::<0x8, 0x7000697150, 0x10> as *const (), 0), // +0x316e40 sub_316e40
    Lifted::new(0x7000316eb0, fn_7000316eb0 as *const (), 2), // +0x316eb0 sub_316eb0
    Lifted::new(0x7000317390, fn_7000317390 as *const (), 0), // +0x317390 sub_317390
    Lifted::new(0x7000317410, fn_7000317410 as *const (), 2), // +0x317410 sub_317410
    Lifted::new(0x70003177e0, fn_70003177e0 as *const (), 2), // +0x3177e0 sub_3177e0
    Lifted::new(0x7000317b90, fn_7000317b90 as *const (), 0), // +0x317b90 sub_317b90
    Lifted::new(0x7000317d90, fn_7000317d90 as *const (), 0), // +0x317d90 sub_317d90
    Lifted::new(0x7000317e10, fn_7000317e10 as *const (), 0), // +0x317e10 sub_317e10
    Lifted::new(0x7000318000, fn_7000318000 as *const (), 0), // +0x318000 sub_318000
    Lifted::new(0x7000318120, fn_7000318120 as *const (), 2), // +0x318120 sub_318120
    Lifted::new(0x70003185e0, fn_70003185e0 as *const (), 0), // +0x3185e0 sub_3185e0
    Lifted::new(0x7000318640, fn_7000318640 as *const (), 2), // +0x318640 sub_318640
    Lifted::new(0x7000318940, fn_7000318940 as *const (), 0), // +0x318940 sub_318940
    Lifted::new(0x70003189f0, fn_70003189f0 as *const (), 0), // +0x3189f0 sub_3189f0
    Lifted::new(0x7000318a90, fn_7000318a90 as *const (), 2), // +0x318a90 sub_318a90
    Lifted::new(0x70003191f0, fn_70003191f0 as *const (), 0), // +0x3191f0 sub_3191f0
    Lifted::new(0x7000319410, fn_7000319410 as *const (), 2), // +0x319410 sub_319410
    Lifted::new(0x7000319820, fn_7000319820 as *const (), 2), // +0x319820 sub_319820
    Lifted::new(0x7000319c50, fn_7000319c50 as *const (), 0), // +0x319c50 sub_319c50
    Lifted::new(0x7000319db0, fn_7000319db0 as *const (), 0), // +0x319db0 sub_319db0
    Lifted::new(0x7000319e30, fn_7000319e30 as *const (), 0), // +0x319e30 sub_319e30
    Lifted::new(0x7000319ec0, fn_7000319ec0 as *const (), 0), // +0x319ec0 sub_319ec0
    Lifted::new(0x7000319f50, fn_7000319f50 as *const (), 0), // +0x319f50 sub_319f50
    Lifted::new(0x700031a030, tpl_25::<0x7000694974, 0x700069b170, 0x70005a4174, 0x70005a69d0, 0x700069b180, 0x700069b1a0, 0x700069b1c0, 0x700069b1e0, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x31a030 sub_31a030
    Lifted::new(0x700031a0b0, tpl_25::<0x7000694974, 0x700069b200, 0x70005a4174, 0x70005a69c0, 0x700069b210, 0x700069b230, 0x700069b250, 0x700069b270, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x31a0b0 sub_31a0b0
    Lifted::new(0x700031a130, tpl_25::<0x7000694974, 0x700069b290, 0x70005a4174, 0x70005a69b0, 0x700069b2a0, 0x700069b2b8, 0x700069b2d0, 0x700069b2e8, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x31a130 sub_31a130
    Lifted::new(0x700031a1b0, fn_700031a1b0 as *const (), 0), // +0x31a1b0 sub_31a1b0
    Lifted::new(0x700031a2d0, tpl_317::<0x70005a8240, 0x700060d858, 0x70002445b0, 0x700060d848, 0x70005a8230> as *const (), 0), // +0x31a2d0 sub_31a2d0
    Lifted::new(0x700031a440, tpl_317::<0x70005a8240, 0x700060d8d8, 0x70002445b0, 0x700060d848, 0x70005a8230> as *const (), 0), // +0x31a440 sub_31a440
    Lifted::new(0x700031a5b0, fn_700031a5b0 as *const (), 0), // +0x31a5b0 sub_31a5b0
    Lifted::new(0x700031a740, tpl_272::<0x70005a8240, 0x700060d9b0, 0x70002445b0, 0x70005a81b0, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x31a740 sub_31a740
    Lifted::new(0x700031a8c0, tpl_272::<0x70005a8240, 0x700060d958, 0x70002445b0, 0x70005a81b0, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x31a8c0 sub_31a8c0
    Lifted::new(0x700031aa40, tpl_272::<0x70005a8240, 0x700060d9d0, 0x70002445b0, 0x70005a81b0, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x31aa40 sub_31aa40
    Lifted::new(0x700031abc0, tpl_272::<0x70005a8240, 0x700060da00, 0x70002445b0, 0x70005a81b0, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x31abc0 sub_31abc0
    Lifted::new(0x700031ad40, tpl_195::<0x7000692760, 0x7000007030, 0x700048bd10, 0x700031c130> as *const (), 0), // +0x31ad40 sub_31ad40
    Lifted::new(0x700031ae30, tpl_195::<0x7000692760, 0x7000007030, 0x700048bd10, 0x700031c4f0> as *const (), 0), // +0x31ae30 sub_31ae30
    Lifted::new(0x700031af20, tpl_29::<0x18, 0x8, 0x700031cb40> as *const (), 0), // +0x31af20 sub_31af20
    Lifted::new(0x700031afb0, fn_700031afb0 as *const (), 2), // +0x31afb0 sub_31afb0
    Lifted::new(0x700031b030, fn_700031b030 as *const (), 0), // +0x31b030 sub_31b030
    Lifted::new(0x700031b1a0, fn_700031b1a0 as *const (), 0), // +0x31b1a0 sub_31b1a0
    Lifted::new(0x700031b260, tpl_318::<0x48, 0x8, 0x18, 0x58, 0x50, 0x70003191f0, 0x700010b4f0> as *const (), 0), // +0x31b260 sub_31b260
    Lifted::new(0x700031b2d0, tpl_259::<0x7000317b90> as *const (), 2), // +0x31b2d0 sub_31b2d0
    Lifted::new(0x700031b2f0, tpl_318::<0x48, 0x8, 0x18, 0x50, 0x58, 0x70003191f0, 0x700010b4f0> as *const (), 0), // +0x31b2f0 sub_31b2f0
    Lifted::new(0x700031b360, fn_700031b360 as *const (), 0), // +0x31b360 sub_31b360
    Lifted::new(0x700031b420, fn_700031b420 as *const (), 0), // +0x31b420 sub_31b420
    Lifted::new(0x700031b490, fn_700031b490 as *const (), 0), // +0x31b490 sub_31b490
    Lifted::new(0x700031b530, tpl_319::<0x70001c1260, 0x70001c13d0, 0x700010b4f0, 0x7000697150, 0x70001c1200> as *const (), 2), // +0x31b530 sub_31b530
    Lifted::new(0x700031b6a0, tpl_320::<0x700060d6e0, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x31b6a0 sub_31b6a0
    Lifted::new(0x700031b750, tpl_192::<0x700060d6f0, 0x7000318940, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x31b750 sub_31b750
    Lifted::new(0x700031b7d0, fn_700031b7d0 as *const (), 0), // +0x31b7d0 sub_31b7d0
    Lifted::new(0x700031b880, tpl_192::<0x700060d710, 0x7000317d90, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x31b880 sub_31b880
    Lifted::new(0x700031b900, tpl_191::<0x700060d720, 0x70002972e0, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x31b900 sub_31b900
    Lifted::new(0x700031b9b0, tpl_320::<0x700060d730, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x31b9b0 sub_31b9b0
    Lifted::new(0x700031ba60, tpl_227::<0x700060d740, 0x70003169b0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x31ba60 sub_31ba60
    Lifted::new(0x700031bae0, tpl_163::<0x700060d750, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x31bae0 sub_31bae0
    Lifted::new(0x700031bb90, fn_700031bb90 as *const (), 0), // +0x31bb90 sub_31bb90
    Lifted::new(0x700031bcd0, fn_700031bcd0 as *const (), 2), // +0x31bcd0 sub_31bcd0
    Lifted::new(0x700031be30, fn_700031be30 as *const (), 2), // +0x31be30 sub_31be30
    Lifted::new(0x700031c130, fn_700031c130 as *const (), 2), // +0x31c130 sub_31c130
    Lifted::new(0x700031c4f0, fn_700031c4f0 as *const (), 2), // +0x31c4f0 sub_31c4f0
    Lifted::new(0x700031c8b0, fn_700031c8b0 as *const (), 2), // +0x31c8b0 sub_31c8b0
    Lifted::new(0x700031cb40, fn_700031cb40 as *const (), 2), // +0x31cb40 sub_31cb40
    Lifted::new(0x700031cdf0, fn_700031cdf0 as *const (), 2), // +0x31cdf0 sub_31cdf0
    Lifted::new(0x700031d0a0, tpl_193::<0x70005a4d32, 0x700062e490, 0x700031a0b0, 0x7000697150, 0x70005f84c0, 0x700031ae30, 0x7000316e40, 0x700048beb0> as *const (), 2), // +0x31d0a0 sub_31d0a0
    Lifted::new(0x700031d350, fn_700031d350 as *const (), 2), // +0x31d350 sub_31d350
    Lifted::new(0x700031d630, fn_700031d630 as *const (), 2), // +0x31d630 sub_31d630
    Lifted::new(0x700031d730, fn_700031d730 as *const (), 2), // +0x31d730 sub_31d730
    Lifted::new(0x700031d8c0, fn_700031d8c0 as *const (), 0), // +0x31d8c0 sub_31d8c0
    Lifted::new(0x700031da60, fn_700031da60 as *const (), 0), // +0x31da60 sub_31da60
    Lifted::new(0x700031dac0, fn_700031dac0 as *const (), 0), // +0x31dac0 sub_31dac0
    Lifted::new(0x700031dc10, tpl_212::<0x7000318640> as *const (), 0), // +0x31dc10 sub_31dc10
    Lifted::new(0x700031dc20, fn_700031dc20 as *const (), 2), // +0x31dc20 sub_31dc20
    Lifted::new(0x700031dd50, fn_700031dd50 as *const (), 2), // +0x31dd50 sub_31dd50
    Lifted::new(0x700031de40, fn_700031de40 as *const (), 0), // +0x31de40 sub_31de40
    Lifted::new(0x700031df40, fn_700031df40 as *const (), 2), // +0x31df40 sub_31df40
    Lifted::new(0x700031dff0, fn_700031dff0 as *const (), 2), // +0x31dff0 sub_31dff0
    Lifted::new(0x700031e110, fn_700031e110 as *const (), 2), // +0x31e110 sub_31e110
    Lifted::new(0x700031e460, fn_700031e460 as *const (), 2), // +0x31e460 sub_31e460
    Lifted::new(0x700031e5a0, fn_700031e5a0 as *const (), 2), // +0x31e5a0 sub_31e5a0
    Lifted::new(0x700031e710, fn_700031e710 as *const (), 0), // +0x31e710 sub_31e710
    Lifted::new(0x700031e8d0, fn_700031e8d0 as *const (), 0), // +0x31e8d0 sub_31e8d0
    Lifted::new(0x700031ea90, fn_700031ea90 as *const (), 1), // +0x31ea90 sub_31ea90
    Lifted::new(0x700031ed70, fn_700031ed70 as *const (), 0), // +0x31ed70 sub_31ed70
    Lifted::new(0x700031edd0, fn_700031edd0 as *const (), 0), // +0x31edd0 sub_31edd0
    Lifted::new(0x700031eef0, fn_700031eef0 as *const (), 0), // +0x31eef0 sub_31eef0
    Lifted::new(0x700031f280, fn_700031f280 as *const (), 0), // +0x31f280 sub_31f280
    Lifted::new(0x700031f700, fn_700031f700 as *const (), 0), // +0x31f700 sub_31f700
    Lifted::new(0x700031f8a0, fn_700031f8a0 as *const (), 0), // +0x31f8a0 sub_31f8a0
    Lifted::new(0x700031f970, fn_700031f970 as *const (), 0), // +0x31f970 sub_31f970
    Lifted::new(0x700031f9f0, fn_700031f9f0 as *const (), 0), // +0x31f9f0 sub_31f9f0
    Lifted::new(0x700031fa20, tpl_143::<0x10> as *const (), 1), // +0x31fa20 sub_31fa20
    Lifted::new(0x700031fa30, tpl_321::<0x7000697150, 0x10> as *const (), 0), // +0x31fa30 sub_31fa30
    Lifted::new(0x700031fa60, tpl_322::<0x700062deb8, 0x70005a8300, 0x700048c2c0> as *const (), 0), // +0x31fa60 sub_31fa60
    Lifted::new(0x700031faa0, fn_700031faa0 as *const (), 2), // +0x31faa0 sub_31faa0
    Lifted::new(0x700031fe90, fn_700031fe90 as *const (), 0), // +0x31fe90 sub_31fe90
    Lifted::new(0x700031ff90, fn_700031ff90 as *const (), 0), // +0x31ff90 sub_31ff90
    Lifted::new(0x7000320080, fn_7000320080 as *const (), 0), // +0x320080 sub_320080
    Lifted::new(0x7000320250, fn_7000320250 as *const (), 0), // +0x320250 sub_320250
    Lifted::new(0x70003202c0, tpl_215::<0x50, 0x48, 0x700031fe90> as *const (), 0), // +0x3202c0 sub_3202c0
    Lifted::new(0x70003202d0, tpl_215::<0x50, 0x48, 0x700031f970> as *const (), 0), // +0x3202d0 sub_3202d0
    Lifted::new(0x70003202e0, tpl_215::<0x50, 0x48, 0x700031f8a0> as *const (), 0), // +0x3202e0 sub_3202e0
    Lifted::new(0x70003202f0, tpl_323::<0x700060dc98, 0x700024f790, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x3202f0 sub_3202f0
    Lifted::new(0x7000320370, tpl_323::<0x700060dca8, 0x700024f790, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x320370 sub_320370
    Lifted::new(0x70003203f0, tpl_219::<0x700060dcb8, 0x700028ccf0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x3203f0 sub_3203f0
    Lifted::new(0x7000320470, tpl_219::<0x700060dcc8, 0x700028ccf0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x320470 sub_320470
    Lifted::new(0x70003204f0, fn_70003204f0 as *const (), 0), // +0x3204f0 sub_3204f0
    Lifted::new(0x70003205c0, fn_70003205c0 as *const (), 2), // +0x3205c0 sub_3205c0
    Lifted::new(0x7000320660, fn_7000320660 as *const (), 0), // +0x320660 sub_320660
    Lifted::new(0x70003207d0, fn_70003207d0 as *const (), 0), // +0x3207d0 sub_3207d0
    Lifted::new(0x70003208b0, fn_70003208b0 as *const (), 0), // +0x3208b0 sub_3208b0
    Lifted::new(0x7000320c70, fn_7000320c70 as *const (), 0), // +0x320c70 sub_320c70
    Lifted::new(0x7000320c90, fn_7000320c90 as *const (), 0), // +0x320c90 sub_320c90
    Lifted::new(0x7000322f10, fn_7000322f10 as *const (), 0), // +0x322f10 sub_322f10
    Lifted::new(0x70003230f0, fn_70003230f0 as *const (), 0), // +0x3230f0 sub_3230f0
    Lifted::new(0x7000323290, fn_7000323290 as *const (), 0), // +0x323290 sub_323290
    Lifted::new(0x7000323320, fn_7000323320 as *const (), 2), // +0x323320 sub_323320
    Lifted::new(0x7000323430, fn_7000323430 as *const (), 2), // +0x323430 sub_323430
    Lifted::new(0x7000323580, fn_7000323580 as *const (), 2), // +0x323580 sub_323580
    Lifted::new(0x7000323870, fn_7000323870 as *const (), 2), // +0x323870 sub_323870
    Lifted::new(0x7000323bf0, fn_7000323bf0 as *const (), 0), // +0x323bf0 sub_323bf0
    Lifted::new(0x7000323e30, fn_7000323e30 as *const (), 2), // +0x323e30 sub_323e30
    Lifted::new(0x70003240e0, fn_70003240e0 as *const (), 0), // +0x3240e0 sub_3240e0
    Lifted::new(0x7000324300, fn_7000324300 as *const (), 2), // +0x324300 sub_324300
    Lifted::new(0x7000324520, fn_7000324520 as *const (), 2), // +0x324520 sub_324520
    Lifted::new(0x7000324650, fn_7000324650 as *const (), 0), // +0x324650 sub_324650
    Lifted::new(0x70003247a0, fn_70003247a0 as *const (), 0), // +0x3247a0 sub_3247a0
    Lifted::new(0x7000324a00, fn_7000324a00 as *const (), 2), // +0x324a00 sub_324a00
    Lifted::new(0x7000324bd0, fn_7000324bd0 as *const (), 0), // +0x324bd0 sub_324bd0
    Lifted::new(0x7000324f20, fn_7000324f20 as *const (), 0), // +0x324f20 sub_324f20
    Lifted::new(0x7000324f50, fn_7000324f50 as *const (), 0), // +0x324f50 sub_324f50
    Lifted::new(0x70003250f0, fn_70003250f0 as *const (), 0), // +0x3250f0 sub_3250f0
    Lifted::new(0x7000326400, fn_7000326400 as *const (), 1), // +0x326400 sub_326400
    Lifted::new(0x7000326410, fn_7000326410 as *const (), 2), // +0x326410 sub_326410
    Lifted::new(0x7000326b40, fn_7000326b40 as *const (), 2), // +0x326b40 sub_326b40
    Lifted::new(0x70003271f0, fn_70003271f0 as *const (), 2), // +0x3271f0 sub_3271f0
    Lifted::new(0x7000327940, fn_7000327940 as *const (), 2), // +0x327940 sub_327940
    Lifted::new(0x7000328030, fn_7000328030 as *const (), 2), // +0x328030 sub_328030
    Lifted::new(0x7000328730, fn_7000328730 as *const (), 2), // +0x328730 sub_328730
    Lifted::new(0x7000328df0, fn_7000328df0 as *const (), 2), // +0x328df0 sub_328df0
    Lifted::new(0x70003294e0, fn_70003294e0 as *const (), 2), // +0x3294e0 sub_3294e0
    Lifted::new(0x7000329be0, fn_7000329be0 as *const (), 2), // +0x329be0 sub_329be0
    Lifted::new(0x700032a310, fn_700032a310 as *const (), 2), // +0x32a310 sub_32a310
    Lifted::new(0x700032a9c0, fn_700032a9c0 as *const (), 2), // +0x32a9c0 sub_32a9c0
    Lifted::new(0x700032b110, fn_700032b110 as *const (), 2), // +0x32b110 sub_32b110
    Lifted::new(0x700032b7f0, fn_700032b7f0 as *const (), 2), // +0x32b7f0 sub_32b7f0
    Lifted::new(0x700032bed0, fn_700032bed0 as *const (), 2), // +0x32bed0 sub_32bed0
    Lifted::new(0x700032c5e0, fn_700032c5e0 as *const (), 2), // +0x32c5e0 sub_32c5e0
    Lifted::new(0x700032cca0, fn_700032cca0 as *const (), 2), // +0x32cca0 sub_32cca0
    Lifted::new(0x700032d380, fn_700032d380 as *const (), 2), // +0x32d380 sub_32d380
    Lifted::new(0x700032da60, fn_700032da60 as *const (), 2), // +0x32da60 sub_32da60
    Lifted::new(0x700032e230, fn_700032e230 as *const (), 2), // +0x32e230 sub_32e230
    Lifted::new(0x700032e940, fn_700032e940 as *const (), 2), // +0x32e940 sub_32e940
    Lifted::new(0x700032f040, fn_700032f040 as *const (), 2), // +0x32f040 sub_32f040
    Lifted::new(0x700032f720, fn_700032f720 as *const (), 2), // +0x32f720 sub_32f720
    Lifted::new(0x700032fe50, fn_700032fe50 as *const (), 2), // +0x32fe50 sub_32fe50
    Lifted::new(0x7000330600, fn_7000330600 as *const (), 2), // +0x330600 sub_330600
    Lifted::new(0x7000330cf0, fn_7000330cf0 as *const (), 2), // +0x330cf0 sub_330cf0
    Lifted::new(0x70003313e0, fn_70003313e0 as *const (), 2), // +0x3313e0 sub_3313e0
    Lifted::new(0x7000331b00, fn_7000331b00 as *const (), 2), // +0x331b00 sub_331b00
    Lifted::new(0x7000332260, fn_7000332260 as *const (), 2), // +0x332260 sub_332260
    Lifted::new(0x7000332970, fn_7000332970 as *const (), 2), // +0x332970 sub_332970
    Lifted::new(0x7000333050, fn_7000333050 as *const (), 2), // +0x333050 sub_333050
    Lifted::new(0x7000333750, fn_7000333750 as *const (), 2), // +0x333750 sub_333750
    Lifted::new(0x7000333e10, fn_7000333e10 as *const (), 2), // +0x333e10 sub_333e10
    Lifted::new(0x7000334520, fn_7000334520 as *const (), 2), // +0x334520 sub_334520
    Lifted::new(0x7000334c00, fn_7000334c00 as *const (), 2), // +0x334c00 sub_334c00
    Lifted::new(0x70003352c0, fn_70003352c0 as *const (), 2), // +0x3352c0 sub_3352c0
    Lifted::new(0x7000335980, fn_7000335980 as *const (), 2), // +0x335980 sub_335980
    Lifted::new(0x7000336050, fn_7000336050 as *const (), 2), // +0x336050 sub_336050
    Lifted::new(0x7000336700, fn_7000336700 as *const (), 2), // +0x336700 sub_336700
    Lifted::new(0x7000336db0, fn_7000336db0 as *const (), 2), // +0x336db0 sub_336db0
    Lifted::new(0x7000337470, fn_7000337470 as *const (), 2), // +0x337470 sub_337470
    Lifted::new(0x7000337b40, fn_7000337b40 as *const (), 2), // +0x337b40 sub_337b40
    Lifted::new(0x7000338220, fn_7000338220 as *const (), 2), // +0x338220 sub_338220
    Lifted::new(0x7000338900, fn_7000338900 as *const (), 2), // +0x338900 sub_338900
    Lifted::new(0x7000338ff0, fn_7000338ff0 as *const (), 2), // +0x338ff0 sub_338ff0
    Lifted::new(0x70003396e0, fn_70003396e0 as *const (), 2), // +0x3396e0 sub_3396e0
    Lifted::new(0x7000339df0, fn_7000339df0 as *const (), 2), // +0x339df0 sub_339df0
    Lifted::new(0x700033a4a0, fn_700033a4a0 as *const (), 2), // +0x33a4a0 sub_33a4a0
    Lifted::new(0x700033ab50, fn_700033ab50 as *const (), 2), // +0x33ab50 sub_33ab50
    Lifted::new(0x700033b220, fn_700033b220 as *const (), 2), // +0x33b220 sub_33b220
    Lifted::new(0x700033b910, fn_700033b910 as *const (), 2), // +0x33b910 sub_33b910
    Lifted::new(0x700033c010, fn_700033c010 as *const (), 2), // +0x33c010 sub_33c010
    Lifted::new(0x700033c750, fn_700033c750 as *const (), 2), // +0x33c750 sub_33c750
    Lifted::new(0x700033ce30, fn_700033ce30 as *const (), 2), // +0x33ce30 sub_33ce30
    Lifted::new(0x700033d500, fn_700033d500 as *const (), 2), // +0x33d500 sub_33d500
    Lifted::new(0x700033dc00, fn_700033dc00 as *const (), 2), // +0x33dc00 sub_33dc00
    Lifted::new(0x700033e300, fn_700033e300 as *const (), 2), // +0x33e300 sub_33e300
    Lifted::new(0x700033e9e0, fn_700033e9e0 as *const (), 2), // +0x33e9e0 sub_33e9e0
    Lifted::new(0x700033f090, fn_700033f090 as *const (), 2), // +0x33f090 sub_33f090
    Lifted::new(0x700033f790, fn_700033f790 as *const (), 2), // +0x33f790 sub_33f790
    Lifted::new(0x700033fe40, fn_700033fe40 as *const (), 2), // +0x33fe40 sub_33fe40
    Lifted::new(0x7000340560, fn_7000340560 as *const (), 2), // +0x340560 sub_340560
    Lifted::new(0x7000340c80, fn_7000340c80 as *const (), 2), // +0x340c80 sub_340c80
    Lifted::new(0x70003413a0, fn_70003413a0 as *const (), 2), // +0x3413a0 sub_3413a0
    Lifted::new(0x7000341ac0, fn_7000341ac0 as *const (), 2), // +0x341ac0 sub_341ac0
    Lifted::new(0x70003421b0, fn_70003421b0 as *const (), 2), // +0x3421b0 sub_3421b0
    Lifted::new(0x7000342890, fn_7000342890 as *const (), 2), // +0x342890 sub_342890
    Lifted::new(0x7000342f80, fn_7000342f80 as *const (), 2), // +0x342f80 sub_342f80
    Lifted::new(0x7000343670, fn_7000343670 as *const (), 2), // +0x343670 sub_343670
    Lifted::new(0x7000343d50, fn_7000343d50 as *const (), 2), // +0x343d50 sub_343d50
    Lifted::new(0x7000344430, fn_7000344430 as *const (), 2), // +0x344430 sub_344430
    Lifted::new(0x7000344b00, fn_7000344b00 as *const (), 2), // +0x344b00 sub_344b00
    Lifted::new(0x7000345240, fn_7000345240 as *const (), 2), // +0x345240 sub_345240
    Lifted::new(0x7000345930, fn_7000345930 as *const (), 2), // +0x345930 sub_345930
    Lifted::new(0x7000346070, fn_7000346070 as *const (), 2), // +0x346070 sub_346070
    Lifted::new(0x70003467b0, fn_70003467b0 as *const (), 2), // +0x3467b0 sub_3467b0
    Lifted::new(0x7000346e90, fn_7000346e90 as *const (), 2), // +0x346e90 sub_346e90
    Lifted::new(0x70003475c0, fn_70003475c0 as *const (), 2), // +0x3475c0 sub_3475c0
    Lifted::new(0x7000347d10, fn_7000347d10 as *const (), 2), // +0x347d10 sub_347d10
    Lifted::new(0x70003483f0, fn_70003483f0 as *const (), 2), // +0x3483f0 sub_3483f0
    Lifted::new(0x7000348ac0, fn_7000348ac0 as *const (), 2), // +0x348ac0 sub_348ac0
    Lifted::new(0x70003491a0, fn_70003491a0 as *const (), 2), // +0x3491a0 sub_3491a0
    Lifted::new(0x7000349940, fn_7000349940 as *const (), 2), // +0x349940 sub_349940
    Lifted::new(0x700034a0d0, fn_700034a0d0 as *const (), 2), // +0x34a0d0 sub_34a0d0
    Lifted::new(0x700034a820, fn_700034a820 as *const (), 2), // +0x34a820 sub_34a820
    Lifted::new(0x700034b050, fn_700034b050 as *const (), 2), // +0x34b050 sub_34b050
    Lifted::new(0x700034b730, fn_700034b730 as *const (), 2), // +0x34b730 sub_34b730
    Lifted::new(0x700034be10, fn_700034be10 as *const (), 2), // +0x34be10 sub_34be10
    Lifted::new(0x700034c4d0, fn_700034c4d0 as *const (), 2), // +0x34c4d0 sub_34c4d0
    Lifted::new(0x700034cbc0, fn_700034cbc0 as *const (), 2), // +0x34cbc0 sub_34cbc0
    Lifted::new(0x700034d2f0, fn_700034d2f0 as *const (), 2), // +0x34d2f0 sub_34d2f0
    Lifted::new(0x700034d9c0, fn_700034d9c0 as *const (), 2), // +0x34d9c0 sub_34d9c0
    Lifted::new(0x700034e0a0, fn_700034e0a0 as *const (), 2), // +0x34e0a0 sub_34e0a0
    Lifted::new(0x700034e790, fn_700034e790 as *const (), 2), // +0x34e790 sub_34e790
    Lifted::new(0x700034ee90, fn_700034ee90 as *const (), 2), // +0x34ee90 sub_34ee90
    Lifted::new(0x700034f5a0, fn_700034f5a0 as *const (), 2), // +0x34f5a0 sub_34f5a0
    Lifted::new(0x700034fca0, fn_700034fca0 as *const (), 2), // +0x34fca0 sub_34fca0
    Lifted::new(0x7000350390, fn_7000350390 as *const (), 2), // +0x350390 sub_350390
    Lifted::new(0x7000350a60, fn_7000350a60 as *const (), 2), // +0x350a60 sub_350a60
    Lifted::new(0x7000350ae0, fn_7000350ae0 as *const (), 0), // +0x350ae0 sub_350ae0
    Lifted::new(0x7000350b60, fn_7000350b60 as *const (), 2), // +0x350b60 sub_350b60
    Lifted::new(0x7000350c10, fn_7000350c10 as *const (), 2), // +0x350c10 sub_350c10
    Lifted::new(0x7000350c90, fn_7000350c90 as *const (), 0), // +0x350c90 sub_350c90
    Lifted::new(0x7000350d10, fn_7000350d10 as *const (), 0), // +0x350d10 sub_350d10
    Lifted::new(0x7000350d80, fn_7000350d80 as *const (), 0), // +0x350d80 sub_350d80
    Lifted::new(0x7000350e20, fn_7000350e20 as *const (), 2), // +0x350e20 sub_350e20
    Lifted::new(0x7000350f50, fn_7000350f50 as *const (), 1), // +0x350f50 sub_350f50
    Lifted::new(0x7000350f70, fn_7000350f70 as *const (), 0), // +0x350f70 sub_350f70
    Lifted::new(0x70003512f0, fn_70003512f0 as *const (), 0), // +0x3512f0 sub_3512f0
    Lifted::new(0x70003514a0, fn_70003514a0 as *const (), 2), // +0x3514a0 sub_3514a0
    Lifted::new(0x7000352160, fn_7000352160 as *const (), 0), // +0x352160 sub_352160
    Lifted::new(0x7000352190, tpl_65::<0x1, 0x10, 0x70003522c0, 0x70005a4d14> as *const (), 0), // +0x352190 sub_352190
    Lifted::new(0x7000352240, tpl_63::<0x70003522c0, 0x1, 0x10, 0x70005a4d14> as *const (), 0), // +0x352240 sub_352240
    Lifted::new(0x70003522c0, fn_70003522c0 as *const (), 0), // +0x3522c0 sub_3522c0
    Lifted::new(0x7000352440, tpl_226::<0x70005a8240, 0x700060e2b0, 0x70002445b0, 0x70005fb764, 0x70005a8230> as *const (), 0), // +0x352440 sub_352440
    Lifted::new(0x7000352590, tpl_84::<0x70005a8240, 0x700060e310, 0x70002445b0, 0x70005a81b0, 0x70005a8230> as *const (), 0), // +0x352590 sub_352590
    Lifted::new(0x70003526f0, fn_70003526f0 as *const (), 0), // +0x3526f0 sub_3526f0
    Lifted::new(0x7000352900, fn_7000352900 as *const (), 0), // +0x352900 sub_352900
    Lifted::new(0x70003529b0, fn_70003529b0 as *const (), 2), // +0x3529b0 sub_3529b0
    Lifted::new(0x7000352aa0, fn_7000352aa0 as *const (), 0), // +0x352aa0 sub_352aa0
    Lifted::new(0x7000352c40, fn_7000352c40 as *const (), 0), // +0x352c40 sub_352c40
    Lifted::new(0x7000352df0, fn_7000352df0 as *const (), 2), // +0x352df0 sub_352df0
    Lifted::new(0x7000353100, fn_7000353100 as *const (), 0), // +0x353100 sub_353100
    Lifted::new(0x7000353130, fn_7000353130 as *const (), 2), // +0x353130 sub_353130
    Lifted::new(0x7000353410, tpl_25::<0x7000694974, 0x70006990e0, 0x70005a4174, 0x70005a6a60, 0x70006990e8, 0x7000699110, 0x7000699138, 0x7000699160, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x353410 sub_353410
    Lifted::new(0x7000353490, fn_7000353490 as *const (), 0), // +0x353490 sub_353490
    Lifted::new(0x7000353520, fn_7000353520 as *const (), 2), // +0x353520 sub_353520
    Lifted::new(0x70003537c0, fn_70003537c0 as *const (), 0), // +0x3537c0 sub_3537c0
    Lifted::new(0x70003538b0, fn_70003538b0 as *const (), 0), // +0x3538b0 sub_3538b0
    Lifted::new(0x7000353960, fn_7000353960 as *const (), 0), // +0x353960 sub_353960
    Lifted::new(0x7000353a70, fn_7000353a70 as *const (), 0), // +0x353a70 sub_353a70
    Lifted::new(0x7000353b60, fn_7000353b60 as *const (), 0), // +0x353b60 sub_353b60
    Lifted::new(0x7000353c90, fn_7000353c90 as *const (), 0), // +0x353c90 sub_353c90
    Lifted::new(0x7000353d30, fn_7000353d30 as *const (), 0), // +0x353d30 sub_353d30
    Lifted::new(0x7000353de0, fn_7000353de0 as *const (), 0), // +0x353de0 sub_353de0
    Lifted::new(0x7000353ea0, fn_7000353ea0 as *const (), 0), // +0x353ea0 sub_353ea0
    Lifted::new(0x7000353f30, fn_7000353f30 as *const (), 1), // +0x353f30 sub_353f30
    Lifted::new(0x7000353f60, tpl_117::<0x30> as *const (), 1), // +0x353f60 sub_353f60
    Lifted::new(0x7000353f70, tpl_117::<0x50> as *const (), 1), // +0x353f70 sub_353f70
    Lifted::new(0x7000353f80, tpl_117::<0x90> as *const (), 1), // +0x353f80 sub_353f80
    Lifted::new(0x7000353f90, tpl_117::<0xb0> as *const (), 1), // +0x353f90 sub_353f90
    Lifted::new(0x7000353fa0, tpl_324::<0xd0> as *const (), 1), // +0x353fa0 sub_353fa0
    Lifted::new(0x7000353fb0, fn_7000353fb0 as *const (), 0), // +0x353fb0 sub_353fb0
    Lifted::new(0x7000354000, fn_7000354000 as *const (), 0), // +0x354000 sub_354000
    Lifted::new(0x7000354140, fn_7000354140 as *const (), 0), // +0x354140 sub_354140
    Lifted::new(0x7000356460, fn_7000356460 as *const (), 0), // +0x356460 sub_356460
    Lifted::new(0x7000357390, fn_7000357390 as *const (), 0), // +0x357390 sub_357390
    Lifted::new(0x70003577b0, fn_70003577b0 as *const (), 1), // +0x3577b0 sub_3577b0
    Lifted::new(0x70003578a0, tpl_140::<0x28, 0x18, 0x70003579f0, 0x8, 0x7000357950, 0x70001ea920, 0x1c, 0x34, 0x10, 0x40, 0x80, 0x30> as *const (), 0), // +0x3578a0 sub_3578a0
    Lifted::new(0x7000357950, fn_7000357950 as *const (), 0), // +0x357950 sub_357950
    Lifted::new(0x70003579f0, fn_70003579f0 as *const (), 2), // +0x3579f0 sub_3579f0
    Lifted::new(0x7000357ac0, fn_7000357ac0 as *const (), 0), // +0x357ac0 sub_357ac0
    Lifted::new(0x7000357c30, fn_7000357c30 as *const (), 0), // +0x357c30 sub_357c30
    Lifted::new(0x7000357e00, fn_7000357e00 as *const (), 2), // +0x357e00 sub_357e00
    Lifted::new(0x7000357f90, fn_7000357f90 as *const (), 2), // +0x357f90 sub_357f90
    Lifted::new(0x70003581f0, fn_70003581f0 as *const (), 2), // +0x3581f0 sub_3581f0
    Lifted::new(0x7000358460, fn_7000358460 as *const (), 0), // +0x358460 sub_358460
    Lifted::new(0x70003586a0, fn_70003586a0 as *const (), 0), // +0x3586a0 sub_3586a0
    Lifted::new(0x7000358830, fn_7000358830 as *const (), 0), // +0x358830 sub_358830
    Lifted::new(0x70003589c0, tpl_71::<0x700060e450, 0x700048c2c0> as *const (), 0), // +0x3589c0 sub_3589c0
    Lifted::new(0x70003589f0, fn_70003589f0 as *const (), 0), // +0x3589f0 sub_3589f0
    Lifted::new(0x7000358a30, fn_7000358a30 as *const (), 2), // +0x358a30 sub_358a30
    Lifted::new(0x7000358d00, fn_7000358d00 as *const (), 2), // +0x358d00 sub_358d00
    Lifted::new(0x7000358e90, tpl_117::<0x190> as *const (), 1), // +0x358e90 sub_358e90
    Lifted::new(0x7000358ea0, fn_7000358ea0 as *const (), 2), // +0x358ea0 sub_358ea0
    Lifted::new(0x70003591d0, fn_70003591d0 as *const (), 2), // +0x3591d0 sub_3591d0
    Lifted::new(0x7000359530, fn_7000359530 as *const (), 3), // +0x359530 sub_359530
    Lifted::new(0x70003595a0, tpl_186::<0x700035ae10, 0x700048c2c0> as *const (), 0), // +0x3595a0 sub_3595a0
    Lifted::new(0x70003595e0, fn_70003595e0 as *const (), 2), // +0x3595e0 sub_3595e0
    Lifted::new(0x7000359900, tpl_117::<0x194> as *const (), 1), // +0x359900 sub_359900
    Lifted::new(0x7000359910, fn_7000359910 as *const (), 2), // +0x359910 sub_359910
    Lifted::new(0x7000359ad0, fn_7000359ad0 as *const (), 2), // +0x359ad0 sub_359ad0
    Lifted::new(0x7000359e00, fn_7000359e00 as *const (), 2), // +0x359e00 sub_359e00
    Lifted::new(0x700035a160, fn_700035a160 as *const (), 0), // +0x35a160 sub_35a160
    Lifted::new(0x700035a1a0, fn_700035a1a0 as *const (), 0), // +0x35a1a0 sub_35a1a0
    Lifted::new(0x700035a1e0, fn_700035a1e0 as *const (), 2), // +0x35a1e0 sub_35a1e0
    Lifted::new(0x700035a590, fn_700035a590 as *const (), 2), // +0x35a590 sub_35a590
    Lifted::new(0x700035a820, fn_700035a820 as *const (), 2), // +0x35a820 sub_35a820
    Lifted::new(0x700035a910, fn_700035a910 as *const (), 2), // +0x35a910 sub_35a910
    Lifted::new(0x700035ac70, fn_700035ac70 as *const (), 3), // +0x35ac70 sub_35ac70
    Lifted::new(0x700035ad80, fn_700035ad80 as *const (), 0), // +0x35ad80 sub_35ad80
    Lifted::new(0x700035ae10, fn_700035ae10 as *const (), 0), // +0x35ae10 sub_35ae10
    Lifted::new(0x700035aea0, fn_700035aea0 as *const (), 2), // +0x35aea0 sub_35aea0
    Lifted::new(0x700035af20, fn_700035af20 as *const (), 2), // +0x35af20 sub_35af20
    Lifted::new(0x700035aff0, fn_700035aff0 as *const (), 2), // +0x35aff0 sub_35aff0
    Lifted::new(0x700035b890, fn_700035b890 as *const (), 2), // +0x35b890 sub_35b890
    Lifted::new(0x700035b9c0, fn_700035b9c0 as *const (), 2), // +0x35b9c0 sub_35b9c0
    Lifted::new(0x700035c4c0, fn_700035c4c0 as *const (), 2), // +0x35c4c0 sub_35c4c0
    Lifted::new(0x700035c530, tpl_325::<0xffffffffffffffff, 0x700035d220, 0x7000694974, 0x70005a4254, 0x30, 0x700002f700> as *const (), 0), // +0x35c530 sub_35c530
    Lifted::new(0x700035c5c0, tpl_325::<0xffffffffffffffff, 0x700035d090, 0x7000694974, 0x70005a4254, 0x20, 0x700002f440> as *const (), 0), // +0x35c5c0 sub_35c5c0
    Lifted::new(0x700035c650, fn_700035c650 as *const (), 0), // +0x35c650 sub_35c650
    Lifted::new(0x700035c720, tpl_325::<0xffffffffffffffff, 0x700035d010, 0x7000694974, 0x70005a4254, 0x10, 0x700002f180> as *const (), 0), // +0x35c720 sub_35c720
    Lifted::new(0x700035c7b0, tpl_325::<0xffffffffffffffff, 0x700035cd70, 0x7000694974, 0x70005a4254, 0x8, 0x700002f020> as *const (), 0), // +0x35c7b0 sub_35c7b0
    Lifted::new(0x700035c840, fn_700035c840 as *const (), 0), // +0x35c840 sub_35c840
    Lifted::new(0x700035c8f0, tpl_155::<0x8, 0x700029bac0, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x35c8f0 sub_35c8f0
    Lifted::new(0x700035c960, tpl_168::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x700035c8f0> as *const (), 2), // +0x35c960 sub_35c960
    Lifted::new(0x700035ca50, fn_700035ca50 as *const (), 0), // +0x35ca50 sub_35ca50
    Lifted::new(0x700035cb80, fn_700035cb80 as *const (), 0), // +0x35cb80 sub_35cb80
    Lifted::new(0x700035cc40, tpl_25::<0x7000694974, 0x7000699188, 0x70005a4174, 0x70005a6a70, 0x7000699190, 0x70006991b8, 0x70006991e0, 0x7000699208, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x35cc40 sub_35cc40
    Lifted::new(0x700035ccc0, fn_700035ccc0 as *const (), 0), // +0x35ccc0 sub_35ccc0
    Lifted::new(0x700035cd70, fn_700035cd70 as *const (), 0), // +0x35cd70 sub_35cd70
    Lifted::new(0x700035d010, fn_700035d010 as *const (), 0), // +0x35d010 sub_35d010
    Lifted::new(0x700035d090, fn_700035d090 as *const (), 0), // +0x35d090 sub_35d090
    Lifted::new(0x700035d220, fn_700035d220 as *const (), 0), // +0x35d220 sub_35d220
    Lifted::new(0x700035d3e0, fn_700035d3e0 as *const (), 0), // +0x35d3e0 sub_35d3e0
    Lifted::new(0x700035d460, fn_700035d460 as *const (), 1), // +0x35d460 sub_35d460
    Lifted::new(0x700035d480, fn_700035d480 as *const (), 0), // +0x35d480 sub_35d480
    Lifted::new(0x700035d530, fn_700035d530 as *const (), 2), // +0x35d530 sub_35d530
    Lifted::new(0x700035d940, tpl_117::<0x1b8> as *const (), 1), // +0x35d940 sub_35d940
    Lifted::new(0x700035d950, fn_700035d950 as *const (), 2), // +0x35d950 sub_35d950
    Lifted::new(0x700035db30, fn_700035db30 as *const (), 0), // +0x35db30 sub_35db30
    Lifted::new(0x700035dbb0, fn_700035dbb0 as *const (), 2), // +0x35dbb0 sub_35dbb0
    Lifted::new(0x700035dca0, fn_700035dca0 as *const (), 2), // +0x35dca0 sub_35dca0
    Lifted::new(0x700035dfd0, fn_700035dfd0 as *const (), 2), // +0x35dfd0 sub_35dfd0
    Lifted::new(0x700035e340, fn_700035e340 as *const (), 0), // +0x35e340 sub_35e340
    Lifted::new(0x700035e3b0, fn_700035e3b0 as *const (), 2), // +0x35e3b0 sub_35e3b0
    Lifted::new(0x700035e940, fn_700035e940 as *const (), 2), // +0x35e940 sub_35e940
    Lifted::new(0x700035eb60, fn_700035eb60 as *const (), 1), // +0x35eb60 sub_35eb60
    Lifted::new(0x700035eb80, fn_700035eb80 as *const (), 2), // +0x35eb80 sub_35eb80
    Lifted::new(0x700035ef80, fn_700035ef80 as *const (), 2), // +0x35ef80 sub_35ef80
    Lifted::new(0x700035f420, fn_700035f420 as *const (), 2), // +0x35f420 sub_35f420
    Lifted::new(0x700035f7a0, fn_700035f7a0 as *const (), 0), // +0x35f7a0 sub_35f7a0
    Lifted::new(0x700035f7f0, fn_700035f7f0 as *const (), 0), // +0x35f7f0 sub_35f7f0
    Lifted::new(0x700035f870, fn_700035f870 as *const (), 2), // +0x35f870 sub_35f870
    Lifted::new(0x700035fa40, fn_700035fa40 as *const (), 2), // +0x35fa40 sub_35fa40
    Lifted::new(0x700035fd70, fn_700035fd70 as *const (), 2), // +0x35fd70 sub_35fd70
    Lifted::new(0x7000360010, fn_7000360010 as *const (), 2), // +0x360010 sub_360010
    Lifted::new(0x70003603c0, fn_70003603c0 as *const (), 2), // +0x3603c0 sub_3603c0
    Lifted::new(0x70003604a0, tpl_117::<0x198> as *const (), 1), // +0x3604a0 sub_3604a0
    Lifted::new(0x70003604b0, tpl_172::<0x8, 0x700035f7f0, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x3604b0 sub_3604b0
    Lifted::new(0x7000360530, fn_7000360530 as *const (), 2), // +0x360530 sub_360530
    Lifted::new(0x7000360640, fn_7000360640 as *const (), 2), // +0x360640 sub_360640
    Lifted::new(0x70003606d0, fn_70003606d0 as *const (), 1), // +0x3606d0 sub_3606d0
    Lifted::new(0x70003606f0, fn_70003606f0 as *const (), 3), // +0x3606f0 sub_3606f0
    Lifted::new(0x70003607e0, fn_70003607e0 as *const (), 2), // +0x3607e0 sub_3607e0
    Lifted::new(0x7000360be0, fn_7000360be0 as *const (), 0), // +0x360be0 sub_360be0
    Lifted::new(0x7000360c30, fn_7000360c30 as *const (), 2), // +0x360c30 sub_360c30
    Lifted::new(0x7000361070, fn_7000361070 as *const (), 2), // +0x361070 sub_361070
    Lifted::new(0x70003615d0, fn_70003615d0 as *const (), 2), // +0x3615d0 sub_3615d0
    Lifted::new(0x7000361900, fn_7000361900 as *const (), 2), // +0x361900 sub_361900
    Lifted::new(0x7000361c60, fn_7000361c60 as *const (), 2), // +0x361c60 sub_361c60
    Lifted::new(0x7000361fa0, fn_7000361fa0 as *const (), 2), // +0x361fa0 sub_361fa0
    Lifted::new(0x7000362310, fn_7000362310 as *const (), 2), // +0x362310 sub_362310
    Lifted::new(0x7000362680, fn_7000362680 as *const (), 2), // +0x362680 sub_362680
    Lifted::new(0x7000362730, fn_7000362730 as *const (), 2), // +0x362730 sub_362730
    Lifted::new(0x7000362980, tpl_117::<0x180> as *const (), 1), // +0x362980 sub_362980
    Lifted::new(0x7000362990, fn_7000362990 as *const (), 0), // +0x362990 sub_362990
    Lifted::new(0x7000362a50, tpl_152::<0x10c, 0x10, 0xc, 0x700062e214, 0x700062e9b0, 0x70001f3aa0, 0x7000362ae0, 0x7000391800> as *const (), 2), // +0x362a50 sub_362a50
    Lifted::new(0x7000362ae0, fn_7000362ae0 as *const (), 0), // +0x362ae0 sub_362ae0
    Lifted::new(0x7000362b10, fn_7000362b10 as *const (), 2), // +0x362b10 sub_362b10
    Lifted::new(0x7000362e50, fn_7000362e50 as *const (), 2), // +0x362e50 sub_362e50
    Lifted::new(0x7000362f10, fn_7000362f10 as *const (), 2), // +0x362f10 sub_362f10
    Lifted::new(0x70003630b0, fn_70003630b0 as *const (), 2), // +0x3630b0 sub_3630b0
    Lifted::new(0x70003631c0, fn_70003631c0 as *const (), 2), // +0x3631c0 sub_3631c0
    Lifted::new(0x7000363d30, fn_7000363d30 as *const (), 2), // +0x363d30 sub_363d30
    Lifted::new(0x70003645c0, fn_70003645c0 as *const (), 2), // +0x3645c0 sub_3645c0
    Lifted::new(0x7000364e50, fn_7000364e50 as *const (), 2), // +0x364e50 sub_364e50
    Lifted::new(0x7000364f50, fn_7000364f50 as *const (), 0), // +0x364f50 sub_364f50
    Lifted::new(0x7000365070, fn_7000365070 as *const (), 2), // +0x365070 sub_365070
    Lifted::new(0x7000365150, fn_7000365150 as *const (), 2), // +0x365150 sub_365150
    Lifted::new(0x7000365ab0, fn_7000365ab0 as *const (), 2), // +0x365ab0 sub_365ab0
    Lifted::new(0x7000365b80, fn_7000365b80 as *const (), 2), // +0x365b80 sub_365b80
    Lifted::new(0x70003660a0, fn_70003660a0 as *const (), 2), // +0x3660a0 sub_3660a0
    Lifted::new(0x7000366600, fn_7000366600 as *const (), 2), // +0x366600 sub_366600
    Lifted::new(0x7000367000, fn_7000367000 as *const (), 2), // +0x367000 sub_367000
    Lifted::new(0x7000367b30, fn_7000367b30 as *const (), 2), // +0x367b30 sub_367b30
    Lifted::new(0x7000368080, fn_7000368080 as *const (), 2), // +0x368080 sub_368080
    Lifted::new(0x7000368170, fn_7000368170 as *const (), 2), // +0x368170 sub_368170
    Lifted::new(0x7000368280, fn_7000368280 as *const (), 2), // +0x368280 sub_368280
    Lifted::new(0x70003683b0, fn_70003683b0 as *const (), 2), // +0x3683b0 sub_3683b0
    Lifted::new(0x7000368540, fn_7000368540 as *const (), 2), // +0x368540 sub_368540
    Lifted::new(0x7000368690, fn_7000368690 as *const (), 2), // +0x368690 sub_368690
    Lifted::new(0x7000368800, fn_7000368800 as *const (), 2), // +0x368800 sub_368800
    Lifted::new(0x7000368e10, fn_7000368e10 as *const (), 2), // +0x368e10 sub_368e10
    Lifted::new(0x70003695a0, fn_70003695a0 as *const (), 2), // +0x3695a0 sub_3695a0
    Lifted::new(0x7000369860, fn_7000369860 as *const (), 2), // +0x369860 sub_369860
    Lifted::new(0x7000369c10, fn_7000369c10 as *const (), 2), // +0x369c10 sub_369c10
    Lifted::new(0x7000369ca0, fn_7000369ca0 as *const (), 2), // +0x369ca0 sub_369ca0
    Lifted::new(0x7000369da0, fn_7000369da0 as *const (), 2), // +0x369da0 sub_369da0
    Lifted::new(0x700036aa30, fn_700036aa30 as *const (), 1), // +0x36aa30 sub_36aa30
    Lifted::new(0x700036aad0, fn_700036aad0 as *const (), 0), // +0x36aad0 sub_36aad0
    Lifted::new(0x700036acb0, fn_700036acb0 as *const (), 1), // +0x36acb0 sub_36acb0
    Lifted::new(0x700036ada0, fn_700036ada0 as *const (), 3), // +0x36ada0 sub_36ada0
    Lifted::new(0x700036aeb0, fn_700036aeb0 as *const (), 0), // +0x36aeb0 sub_36aeb0
    Lifted::new(0x700036af00, fn_700036af00 as *const (), 2), // +0x36af00 sub_36af00
    Lifted::new(0x700036b2c0, tpl_117::<0x200> as *const (), 1), // +0x36b2c0 sub_36b2c0
    Lifted::new(0x700036b2d0, fn_700036b2d0 as *const (), 2), // +0x36b2d0 sub_36b2d0
    Lifted::new(0x700036b4d0, fn_700036b4d0 as *const (), 0), // +0x36b4d0 sub_36b4d0
    Lifted::new(0x700036b600, fn_700036b600 as *const (), 2), // +0x36b600 sub_36b600
    Lifted::new(0x700036b9c0, fn_700036b9c0 as *const (), 2), // +0x36b9c0 sub_36b9c0
    Lifted::new(0x700036bac0, fn_700036bac0 as *const (), 2), // +0x36bac0 sub_36bac0
    Lifted::new(0x700036bcd0, fn_700036bcd0 as *const (), 0), // +0x36bcd0 sub_36bcd0
    Lifted::new(0x700036bdd0, fn_700036bdd0 as *const (), 2), // +0x36bdd0 sub_36bdd0
    Lifted::new(0x700036c340, fn_700036c340 as *const (), 0), // +0x36c340 sub_36c340
    Lifted::new(0x700036c450, fn_700036c450 as *const (), 0), // +0x36c450 sub_36c450
    Lifted::new(0x700036c7d0, tpl_326::<0x70005fd8b0, 0x70005ff3d8> as *const (), 1), // +0x36c7d0 sub_36c7d0
    Lifted::new(0x700036c900, tpl_326::<0x70005ff490, 0x70005ff478> as *const (), 1), // +0x36c900 sub_36c900
    Lifted::new(0x700036ca30, tpl_327::<0x70005ff3bc, 0x70005ff3b4, 0x70005febc0, 0x70005f8320> as *const (), 1), // +0x36ca30 sub_36ca30
    Lifted::new(0x700036cc60, tpl_327::<0x70005ff3ac, 0x70005febc0, 0x70005ff3a4, 0x70005f8320> as *const (), 1), // +0x36cc60 sub_36cc60
    Lifted::new(0x700036ce90, fn_700036ce90 as *const (), 1), // +0x36ce90 sub_36ce90
    Lifted::new(0x700036d050, fn_700036d050 as *const (), 2), // +0x36d050 sub_36d050
    Lifted::new(0x700036d1d0, fn_700036d1d0 as *const (), 2), // +0x36d1d0 sub_36d1d0
    Lifted::new(0x700036ed20, fn_700036ed20 as *const (), 0), // +0x36ed20 sub_36ed20
    Lifted::new(0x700036f030, fn_700036f030 as *const (), 0), // +0x36f030 sub_36f030
    Lifted::new(0x700036f2d0, fn_700036f2d0 as *const (), 0), // +0x36f2d0 sub_36f2d0
    Lifted::new(0x700036f6e0, tpl_327::<0x70005f8344, 0x70005ff4a0, 0x70005f834c, 0x70005f8320> as *const (), 1), // +0x36f6e0 sub_36f6e0
    Lifted::new(0x700036f910, fn_700036f910 as *const (), 1), // +0x36f910 sub_36f910
    Lifted::new(0x700036fa40, fn_700036fa40 as *const (), 0), // +0x36fa40 sub_36fa40
    Lifted::new(0x700036fbe0, fn_700036fbe0 as *const (), 0), // +0x36fbe0 sub_36fbe0
    Lifted::new(0x700036fdc0, fn_700036fdc0 as *const (), 0), // +0x36fdc0 sub_36fdc0
    Lifted::new(0x700036ffa0, fn_700036ffa0 as *const (), 0), // +0x36ffa0 sub_36ffa0
    Lifted::new(0x70003701a0, fn_70003701a0 as *const (), 0), // +0x3701a0 sub_3701a0
    Lifted::new(0x7000370270, fn_7000370270 as *const (), 0), // +0x370270 sub_370270
    Lifted::new(0x7000370280, tpl_328::<0x7000693c28, 0x70005a83d0, 0x7000693c20, 0x8> as *const (), 0), // +0x370280 sub_370280
    Lifted::new(0x7000370330, fn_7000370330 as *const (), 1), // +0x370330 sub_370330
    Lifted::new(0x7000370350, tpl_263::<0x7000693be8, 0x70005a83d0, 0x7000693be0, 0x8> as *const (), 0), // +0x370350 sub_370350
    Lifted::new(0x70003703c0, fn_70003703c0 as *const (), 1), // +0x3703c0 sub_3703c0
    Lifted::new(0x70003703d0, tpl_329::<0x7000693ba8, 0x70005a83d0, 0x7000693ba0, 0x8> as *const (), 0), // +0x3703d0 sub_3703d0
    Lifted::new(0x7000370450, tpl_329::<0x7000693b68, 0x70005a83d0, 0x7000693b60, 0x8> as *const (), 0), // +0x370450 sub_370450
    Lifted::new(0x70003704d0, fn_70003704d0 as *const (), 2), // +0x3704d0 sub_3704d0
    Lifted::new(0x7000370590, fn_7000370590 as *const (), 2), // +0x370590 sub_370590
    Lifted::new(0x70003706b0, fn_70003706b0 as *const (), 2), // +0x3706b0 sub_3706b0
    Lifted::new(0x70003707e0, fn_70003707e0 as *const (), 1), // +0x3707e0 sub_3707e0
    Lifted::new(0x7000370800, fn_7000370800 as *const (), 2), // +0x370800 sub_370800
    Lifted::new(0x7000370b20, fn_7000370b20 as *const (), 2), // +0x370b20 sub_370b20
    Lifted::new(0x7000371100, fn_7000371100 as *const (), 2), // +0x371100 sub_371100
    Lifted::new(0x7000371a20, fn_7000371a20 as *const (), 2), // +0x371a20 sub_371a20
    Lifted::new(0x7000372d00, fn_7000372d00 as *const (), 2), // +0x372d00 sub_372d00
    Lifted::new(0x7000372da0, fn_7000372da0 as *const (), 0), // +0x372da0 sub_372da0
    Lifted::new(0x7000372ee0, fn_7000372ee0 as *const (), 0), // +0x372ee0 sub_372ee0
    Lifted::new(0x7000373a60, fn_7000373a60 as *const (), 1), // +0x373a60 sub_373a60
    Lifted::new(0x7000373af0, fn_7000373af0 as *const (), 1), // +0x373af0 sub_373af0
    Lifted::new(0x7000373bc0, fn_7000373bc0 as *const (), 0), // +0x373bc0 sub_373bc0
    Lifted::new(0x7000373c30, fn_7000373c30 as *const (), 0), // +0x373c30 sub_373c30
    Lifted::new(0x7000373cb0, fn_7000373cb0 as *const (), 0), // +0x373cb0 sub_373cb0
    Lifted::new(0x7000373d80, fn_7000373d80 as *const (), 0), // +0x373d80 sub_373d80
    Lifted::new(0x7000373da0, fn_7000373da0 as *const (), 0), // +0x373da0 sub_373da0
    Lifted::new(0x7000373dc0, fn_7000373dc0 as *const (), 0), // +0x373dc0 sub_373dc0
    Lifted::new(0x7000374090, tpl_69::<0x7000378060> as *const (), 0), // +0x374090 sub_374090
    Lifted::new(0x70003740b0, fn_70003740b0 as *const (), 0), // +0x3740b0 sub_3740b0
    Lifted::new(0x7000374110, fn_7000374110 as *const (), 0), // +0x374110 sub_374110
    Lifted::new(0x70003748e0, fn_70003748e0 as *const (), 0), // +0x3748e0 sub_3748e0
    Lifted::new(0x7000374aa0, fn_7000374aa0 as *const (), 0), // +0x374aa0 sub_374aa0
    Lifted::new(0x7000374c60, fn_7000374c60 as *const (), 2), // +0x374c60 sub_374c60
    Lifted::new(0x7000375080, fn_7000375080 as *const (), 0), // +0x375080 sub_375080
    Lifted::new(0x7000375120, fn_7000375120 as *const (), 2), // +0x375120 sub_375120
    Lifted::new(0x7000375b80, fn_7000375b80 as *const (), 0), // +0x375b80 sub_375b80
    Lifted::new(0x7000375ec0, fn_7000375ec0 as *const (), 2), // +0x375ec0 sub_375ec0
    Lifted::new(0x7000376180, fn_7000376180 as *const (), 2), // +0x376180 sub_376180
    Lifted::new(0x7000377150, fn_7000377150 as *const (), 0), // +0x377150 sub_377150
    Lifted::new(0x70003771b0, fn_70003771b0 as *const (), 0), // +0x3771b0 sub_3771b0
    Lifted::new(0x7000377250, fn_7000377250 as *const (), 0), // +0x377250 sub_377250
    Lifted::new(0x70003772c0, tpl_330::<0x7000697150, 0x7000383130, 0x7000375080> as *const (), 0), // +0x3772c0 sub_3772c0
    Lifted::new(0x7000377330, tpl_330::<0x7000697150, 0x700037b3d0, 0x7000375080> as *const (), 0), // +0x377330 sub_377330
    Lifted::new(0x70003773a0, tpl_330::<0x7000697150, 0x70003825d0, 0x7000375080> as *const (), 0), // +0x3773a0 sub_3773a0
    Lifted::new(0x7000377410, tpl_330::<0x7000697150, 0x7000381ff0, 0x7000375080> as *const (), 0), // +0x377410 sub_377410
    Lifted::new(0x7000377480, tpl_330::<0x7000697150, 0x7000382490, 0x7000375080> as *const (), 0), // +0x377480 sub_377480
    Lifted::new(0x70003774f0, tpl_330::<0x7000697150, 0x7000382d10, 0x7000375080> as *const (), 0), // +0x3774f0 sub_3774f0
    Lifted::new(0x7000377560, tpl_330::<0x7000697150, 0x7000382e80, 0x7000375080> as *const (), 0), // +0x377560 sub_377560
    Lifted::new(0x70003775d0, tpl_331::<0x7000697150, 0x70003822e0, 0x7000375080> as *const (), 0), // +0x3775d0 sub_3775d0
    Lifted::new(0x7000377640, tpl_331::<0x7000697150, 0x7000382ff0, 0x7000375080> as *const (), 0), // +0x377640 sub_377640
    Lifted::new(0x70003776b0, fn_70003776b0 as *const (), 0), // +0x3776b0 sub_3776b0
    Lifted::new(0x7000377750, fn_7000377750 as *const (), 0), // +0x377750 sub_377750
    Lifted::new(0x70003777f0, tpl_330::<0x7000697150, 0x70003821a0, 0x7000375080> as *const (), 0), // +0x3777f0 sub_3777f0
    Lifted::new(0x7000377860, tpl_330::<0x7000697150, 0x700029b630, 0x7000375080> as *const (), 0), // +0x377860 sub_377860
    Lifted::new(0x70003778d0, tpl_330::<0x7000697150, 0x700029bcd0, 0x7000375080> as *const (), 0), // +0x3778d0 sub_3778d0
    Lifted::new(0x7000377940, fn_7000377940 as *const (), 1), // +0x377940 sub_377940
    Lifted::new(0x7000377950, fn_7000377950 as *const (), 0), // +0x377950 sub_377950
    Lifted::new(0x70003779d0, fn_70003779d0 as *const (), 0), // +0x3779d0 sub_3779d0
    Lifted::new(0x7000377ae0, fn_7000377ae0 as *const (), 0), // +0x377ae0 sub_377ae0
    Lifted::new(0x7000377b60, fn_7000377b60 as *const (), 2), // +0x377b60 sub_377b60
    Lifted::new(0x7000377ca0, fn_7000377ca0 as *const (), 0), // +0x377ca0 sub_377ca0
    Lifted::new(0x7000377d60, fn_7000377d60 as *const (), 0), // +0x377d60 sub_377d60
    Lifted::new(0x7000377de0, fn_7000377de0 as *const (), 2), // +0x377de0 sub_377de0
    Lifted::new(0x7000377ef0, fn_7000377ef0 as *const (), 0), // +0x377ef0 sub_377ef0
    Lifted::new(0x7000378060, fn_7000378060 as *const (), 2), // +0x378060 sub_378060
    Lifted::new(0x7000378340, fn_7000378340 as *const (), 0), // +0x378340 sub_378340
    Lifted::new(0x70003785c0, tpl_332::<0x7000697150, 0x700029d030, 0x7000375080, 0x70001a4330, 0x70005a4d14> as *const (), 0), // +0x3785c0 sub_3785c0
    Lifted::new(0x7000378700, tpl_332::<0x7000697150, 0x700029bbc0, 0x7000375080, 0x70001a4330, 0x70005a4d14> as *const (), 0), // +0x378700 sub_378700
    Lifted::new(0x7000378840, tpl_332::<0x7000697150, 0x7000384b60, 0x7000375080, 0x70001a4330, 0x70005a4d14> as *const (), 0), // +0x378840 sub_378840
    Lifted::new(0x7000378980, fn_7000378980 as *const (), 0), // +0x378980 sub_378980
    Lifted::new(0x70003789d0, fn_70003789d0 as *const (), 0), // +0x3789d0 sub_3789d0
    Lifted::new(0x7000378ad0, fn_7000378ad0 as *const (), 1), // +0x378ad0 sub_378ad0
    Lifted::new(0x7000378af0, fn_7000378af0 as *const (), 2), // +0x378af0 sub_378af0
    Lifted::new(0x7000378c50, fn_7000378c50 as *const (), 2), // +0x378c50 sub_378c50
    Lifted::new(0x7000378d80, fn_7000378d80 as *const (), 0), // +0x378d80 sub_378d80
    Lifted::new(0x7000378f40, fn_7000378f40 as *const (), 0), // +0x378f40 sub_378f40
    Lifted::new(0x7000379260, fn_7000379260 as *const (), 0), // +0x379260 sub_379260
    Lifted::new(0x70003795c0, fn_70003795c0 as *const (), 2), // +0x3795c0 sub_3795c0
    Lifted::new(0x7000379ff0, fn_7000379ff0 as *const (), 0), // +0x379ff0 sub_379ff0
    Lifted::new(0x700037a060, fn_700037a060 as *const (), 0), // +0x37a060 sub_37a060
    Lifted::new(0x700037a1a0, tpl_141::<0x10, 0x8, 0x18> as *const (), 1), // +0x37a1a0 sub_37a1a0
    Lifted::new(0x700037a240, fn_700037a240 as *const (), 0), // +0x37a240 sub_37a240
    Lifted::new(0x700037a3b0, tpl_333::<0x10, 0x8, 0x18> as *const (), 3), // +0x37a3b0 sub_37a3b0
    Lifted::new(0x700037a470, fn_700037a470 as *const (), 0), // +0x37a470 sub_37a470
    Lifted::new(0x700037a540, fn_700037a540 as *const (), 0), // +0x37a540 sub_37a540
    Lifted::new(0x700037a7a0, fn_700037a7a0 as *const (), 0), // +0x37a7a0 sub_37a7a0
    Lifted::new(0x700037a9c0, fn_700037a9c0 as *const (), 0), // +0x37a9c0 sub_37a9c0
    Lifted::new(0x700037ac20, fn_700037ac20 as *const (), 0), // +0x37ac20 sub_37ac20
    Lifted::new(0x700037ae80, fn_700037ae80 as *const (), 0), // +0x37ae80 sub_37ae80
    Lifted::new(0x700037af60, fn_700037af60 as *const (), 0), // +0x37af60 sub_37af60
    Lifted::new(0x700037b1c0, fn_700037b1c0 as *const (), 0), // +0x37b1c0 sub_37b1c0
    Lifted::new(0x700037b200, fn_700037b200 as *const (), 0), // +0x37b200 sub_37b200
    Lifted::new(0x700037b270, fn_700037b270 as *const (), 0), // +0x37b270 sub_37b270
    Lifted::new(0x700037b300, fn_700037b300 as *const (), 0), // +0x37b300 sub_37b300
    Lifted::new(0x700037b3d0, fn_700037b3d0 as *const (), 0), // +0x37b3d0 sub_37b3d0
    Lifted::new(0x700037b490, tpl_275::<0x7000697150, 0x30, 0x700037b3d0, 0x70003801e0> as *const (), 0), // +0x37b490 sub_37b490
    Lifted::new(0x700037b510, fn_700037b510 as *const (), 0), // +0x37b510 sub_37b510
    Lifted::new(0x700037b5d0, tpl_182::<0x10, 0x700060fcc2, 0x700060fcb8, 0x7000231e90> as *const (), 0), // +0x37b5d0 sub_37b5d0
    Lifted::new(0x700037b610, fn_700037b610 as *const (), 2), // +0x37b610 sub_37b610
    Lifted::new(0x700037f6b0, fn_700037f6b0 as *const (), 0), // +0x37f6b0 sub_37f6b0
    Lifted::new(0x700037f7f0, fn_700037f7f0 as *const (), 0), // +0x37f7f0 sub_37f7f0
    Lifted::new(0x700037f9c0, fn_700037f9c0 as *const (), 0), // +0x37f9c0 sub_37f9c0
    Lifted::new(0x700037f9f0, fn_700037f9f0 as *const (), 0), // +0x37f9f0 sub_37f9f0
    Lifted::new(0x700037fce0, fn_700037fce0 as *const (), 0), // +0x37fce0 sub_37fce0
    Lifted::new(0x700037fdb0, fn_700037fdb0 as *const (), 0), // +0x37fdb0 sub_37fdb0
    Lifted::new(0x700037fe60, fn_700037fe60 as *const (), 0), // +0x37fe60 sub_37fe60
    Lifted::new(0x700037fe80, fn_700037fe80 as *const (), 0), // +0x37fe80 sub_37fe80
    Lifted::new(0x7000380060, fn_7000380060 as *const (), 0), // +0x380060 sub_380060
    Lifted::new(0x70003801e0, fn_70003801e0 as *const (), 0), // +0x3801e0 sub_3801e0
    Lifted::new(0x70003802c0, fn_70003802c0 as *const (), 0), // +0x3802c0 sub_3802c0
    Lifted::new(0x7000380590, fn_7000380590 as *const (), 0), // +0x380590 sub_380590
    Lifted::new(0x70003805f0, fn_70003805f0 as *const (), 0), // +0x3805f0 sub_3805f0
    Lifted::new(0x7000380770, fn_7000380770 as *const (), 2), // +0x380770 sub_380770
    Lifted::new(0x70003808a0, tpl_334::<0x70003c6d80, 0x70001dca70, 0xf0, 0x7000429920, 0x30, 0x10, 0x18, 0x700039fa20, 0x7000132560, 0x700038094d> as *const (), 0), // +0x3808a0 sub_3808a0
    Lifted::new(0x7000380950, tpl_334::<0x70003c6d80, 0x70001c1630, 0xf0, 0x7000429920, 0x30, 0x10, 0x18, 0x700039fa20, 0x7000132560, 0x70003809fd> as *const (), 0), // +0x380950 sub_380950
    Lifted::new(0x7000380a00, tpl_334::<0x70003c6d80, 0x70001c1950, 0xf0, 0x7000429920, 0x30, 0x10, 0x18, 0x700039fa20, 0x7000132560, 0x7000380aad> as *const (), 0), // +0x380a00 sub_380a00
    Lifted::new(0x7000380ab0, tpl_334::<0x70003c6d80, 0x70001c17b0, 0xf0, 0x7000429920, 0x30, 0x10, 0x18, 0x700039fa20, 0x7000132560, 0x7000380b5d> as *const (), 0), // +0x380ab0 sub_380ab0
    Lifted::new(0x7000380b60, tpl_334::<0x70003c6d80, 0x70001dd820, 0xf0, 0x7000429920, 0x30, 0x10, 0x18, 0x700039fa20, 0x7000132560, 0x7000380c0d> as *const (), 0), // +0x380b60 sub_380b60
    Lifted::new(0x7000380c10, tpl_334::<0x70003c6d80, 0x70001dd530, 0xf0, 0x7000429920, 0x30, 0x10, 0x18, 0x700039fa20, 0x7000132560, 0x7000380cbd> as *const (), 0), // +0x380c10 sub_380c10
    Lifted::new(0x7000380cc0, fn_7000380cc0 as *const (), 0), // +0x380cc0 sub_380cc0
    Lifted::new(0x7000380d50, tpl_334::<0x70003c6d80, 0x70001c1ad0, 0xf0, 0x7000429920, 0x30, 0x10, 0x18, 0x700039fa20, 0x7000132560, 0x7000380dfd> as *const (), 0), // +0x380d50 sub_380d50
    Lifted::new(0x7000380e00, fn_7000380e00 as *const (), 0), // +0x380e00 sub_380e00
    Lifted::new(0x7000380e90, fn_7000380e90 as *const (), 0), // +0x380e90 sub_380e90
    Lifted::new(0x7000380f20, fn_7000380f20 as *const (), 0), // +0x380f20 sub_380f20
    Lifted::new(0x7000380fb0, tpl_226::<0x70005a8240, 0x700060fc30, 0x70002445b0, 0x700060fc40, 0x70005a8230> as *const (), 0), // +0x380fb0 sub_380fb0
    Lifted::new(0x7000381100, tpl_93::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x70001a86c0, 0x70003814d0> as *const (), 0), // +0x381100 sub_381100
    Lifted::new(0x70003811d0, fn_70003811d0 as *const (), 2), // +0x3811d0 sub_3811d0
    Lifted::new(0x7000381350, fn_7000381350 as *const (), 2), // +0x381350 sub_381350
    Lifted::new(0x70003814d0, fn_70003814d0 as *const (), 2), // +0x3814d0 sub_3814d0
    Lifted::new(0x70003817e0, fn_70003817e0 as *const (), 2), // +0x3817e0 sub_3817e0
    Lifted::new(0x7000381ad0, tpl_25::<0x7000694974, 0x70006992d8, 0x70005a4174, 0x70005a6a80, 0x7000699238, 0x7000699260, 0x7000699288, 0x70006992b0, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x381ad0 sub_381ad0
    Lifted::new(0x7000381b50, fn_7000381b50 as *const (), 0), // +0x381b50 sub_381b50
    Lifted::new(0x7000381c90, fn_7000381c90 as *const (), 0), // +0x381c90 sub_381c90
    Lifted::new(0x7000381d60, fn_7000381d60 as *const (), 0), // +0x381d60 sub_381d60
    Lifted::new(0x7000381f10, fn_7000381f10 as *const (), 1), // +0x381f10 sub_381f10
    Lifted::new(0x7000381ff0, fn_7000381ff0 as *const (), 0), // +0x381ff0 sub_381ff0
    Lifted::new(0x7000382120, tpl_275::<0x7000697150, 0x30, 0x7000381ff0, 0x70003801e0> as *const (), 0), // +0x382120 sub_382120
    Lifted::new(0x70003821a0, fn_70003821a0 as *const (), 0), // +0x3821a0 sub_3821a0
    Lifted::new(0x7000382260, tpl_275::<0x7000697150, 0x30, 0x70003821a0, 0x70003801e0> as *const (), 0), // +0x382260 sub_382260
    Lifted::new(0x70003822e0, fn_70003822e0 as *const (), 0), // +0x3822e0 sub_3822e0
    Lifted::new(0x70003823c0, fn_70003823c0 as *const (), 0), // +0x3823c0 sub_3823c0
    Lifted::new(0x7000382400, tpl_335::<0x7000697150, 0x30, 0x70003822e0, 0x70003801e0, 0x258> as *const (), 0), // +0x382400 sub_382400
    Lifted::new(0x7000382480, tpl_185::<0x70003823c0> as *const (), 0), // +0x382480 sub_382480
    Lifted::new(0x7000382490, fn_7000382490 as *const (), 0), // +0x382490 sub_382490
    Lifted::new(0x7000382550, tpl_275::<0x7000697150, 0x30, 0x7000382490, 0x70003801e0> as *const (), 0), // +0x382550 sub_382550
    Lifted::new(0x70003825d0, fn_70003825d0 as *const (), 0), // +0x3825d0 sub_3825d0
    Lifted::new(0x70003826b0, tpl_275::<0x7000697150, 0x30, 0x70003825d0, 0x70003801e0> as *const (), 0), // +0x3826b0 sub_3826b0
    Lifted::new(0x7000382730, fn_7000382730 as *const (), 0), // +0x382730 sub_382730
    Lifted::new(0x70003828f0, fn_70003828f0 as *const (), 0), // +0x3828f0 sub_3828f0
    Lifted::new(0x7000382940, fn_7000382940 as *const (), 2), // +0x382940 sub_382940
    Lifted::new(0x7000382b60, fn_7000382b60 as *const (), 0), // +0x382b60 sub_382b60
    Lifted::new(0x7000382cfc, tpl_185::<0x70003828f0> as *const (), 0), // +0x382cfc sub_382cfc
    Lifted::new(0x7000382d10, fn_7000382d10 as *const (), 0), // +0x382d10 sub_382d10
    Lifted::new(0x7000382e00, tpl_275::<0x7000697150, 0x30, 0x7000382d10, 0x70003801e0> as *const (), 0), // +0x382e00 sub_382e00
    Lifted::new(0x7000382e80, fn_7000382e80 as *const (), 0), // +0x382e80 sub_382e80
    Lifted::new(0x7000382f70, tpl_275::<0x7000697150, 0x30, 0x7000382e80, 0x70003801e0> as *const (), 0), // +0x382f70 sub_382f70
    Lifted::new(0x7000382ff0, fn_7000382ff0 as *const (), 0), // +0x382ff0 sub_382ff0
    Lifted::new(0x70003830b0, tpl_335::<0x7000697150, 0x30, 0x7000382ff0, 0x70003801e0, 0x258> as *const (), 0), // +0x3830b0 sub_3830b0
    Lifted::new(0x7000383130, fn_7000383130 as *const (), 0), // +0x383130 sub_383130
    Lifted::new(0x7000383200, tpl_275::<0x7000697150, 0x30, 0x7000383130, 0x70003801e0> as *const (), 0), // +0x383200 sub_383200
    Lifted::new(0x7000383280, fn_7000383280 as *const (), 0), // +0x383280 sub_383280
    Lifted::new(0x7000383320, fn_7000383320 as *const (), 2), // +0x383320 sub_383320
    Lifted::new(0x7000383600, fn_7000383600 as *const (), 0), // +0x383600 sub_383600
    Lifted::new(0x7000383770, fn_7000383770 as *const (), 0), // +0x383770 sub_383770
    Lifted::new(0x70003838a0, fn_70003838a0 as *const (), 0), // +0x3838a0 sub_3838a0
    Lifted::new(0x7000383920, fn_7000383920 as *const (), 2), // +0x383920 sub_383920
    Lifted::new(0x7000383b50, fn_7000383b50 as *const (), 2), // +0x383b50 sub_383b50
    Lifted::new(0x7000383d40, fn_7000383d40 as *const (), 2), // +0x383d40 sub_383d40
    Lifted::new(0x7000383e00, fn_7000383e00 as *const (), 0), // +0x383e00 sub_383e00
    Lifted::new(0x7000383ec0, fn_7000383ec0 as *const (), 0), // +0x383ec0 sub_383ec0
    Lifted::new(0x7000383f50, fn_7000383f50 as *const (), 0), // +0x383f50 sub_383f50
    Lifted::new(0x7000383fd0, fn_7000383fd0 as *const (), 0), // +0x383fd0 sub_383fd0
    Lifted::new(0x70003840f0, fn_70003840f0 as *const (), 0), // +0x3840f0 sub_3840f0
    Lifted::new(0x70003841d0, fn_70003841d0 as *const (), 2), // +0x3841d0 sub_3841d0
    Lifted::new(0x7000384260, tpl_227::<0x7000611480, 0x7000383ec0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x384260 sub_384260
    Lifted::new(0x70003842e0, fn_70003842e0 as *const (), 2), // +0x3842e0 sub_3842e0
    Lifted::new(0x7000384610, fn_7000384610 as *const (), 2), // +0x384610 sub_384610
    Lifted::new(0x7000384780, fn_7000384780 as *const (), 0), // +0x384780 sub_384780
    Lifted::new(0x70003847e0, fn_70003847e0 as *const (), 2), // +0x3847e0 sub_3847e0
    Lifted::new(0x7000384b54, tpl_185::<0x7000383600> as *const (), 0), // +0x384b54 sub_384b54
    Lifted::new(0x7000384b60, fn_7000384b60 as *const (), 0), // +0x384b60 sub_384b60
    Lifted::new(0x7000384cb0, tpl_275::<0x7000697150, 0x30, 0x7000384b60, 0x70003801e0> as *const (), 0), // +0x384cb0 sub_384cb0
    Lifted::new(0x7000384d30, fn_7000384d30 as *const (), 0), // +0x384d30 sub_384d30
    Lifted::new(0x7000384d50, fn_7000384d50 as *const (), 2), // +0x384d50 sub_384d50
    Lifted::new(0x7000384dd0, fn_7000384dd0 as *const (), 2), // +0x384dd0 sub_384dd0
    Lifted::new(0x7000384fa0, fn_7000384fa0 as *const (), 2), // +0x384fa0 sub_384fa0
    Lifted::new(0x7000385140, fn_7000385140 as *const (), 2), // +0x385140 sub_385140
    Lifted::new(0x7000385440, fn_7000385440 as *const (), 2), // +0x385440 sub_385440
    Lifted::new(0x7000385610, fn_7000385610 as *const (), 0), // +0x385610 sub_385610
    Lifted::new(0x7000385710, fn_7000385710 as *const (), 2), // +0x385710 sub_385710
    Lifted::new(0x7000385890, fn_7000385890 as *const (), 2), // +0x385890 sub_385890
    Lifted::new(0x7000385950, fn_7000385950 as *const (), 2), // +0x385950 sub_385950
    Lifted::new(0x70003859f0, tpl_24::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000385a90> as *const (), 2), // +0x3859f0 sub_3859f0
    Lifted::new(0x7000385a90, fn_7000385a90 as *const (), 2), // +0x385a90 sub_385a90
    Lifted::new(0x7000385b20, fn_7000385b20 as *const (), 2), // +0x385b20 sub_385b20
    Lifted::new(0x7000385cf0, fn_7000385cf0 as *const (), 0), // +0x385cf0 sub_385cf0
    Lifted::new(0x7000385e30, fn_7000385e30 as *const (), 2), // +0x385e30 sub_385e30
    Lifted::new(0x70003860f0, fn_70003860f0 as *const (), 3), // +0x3860f0 sub_3860f0
    Lifted::new(0x7000386230, fn_7000386230 as *const (), 2), // +0x386230 sub_386230
    Lifted::new(0x70003865a0, fn_70003865a0 as *const (), 2), // +0x3865a0 sub_3865a0
    Lifted::new(0x7000386850, fn_7000386850 as *const (), 1), // +0x386850 sub_386850
    Lifted::new(0x7000386a20, fn_7000386a20 as *const (), 0), // +0x386a20 ??2IEventHandler@cohtml@@SAPEAX_K@Z
    Lifted::new(0x7000386a40, fn_7000386a40 as *const (), 0), // +0x386a40 ??3IEventHandler@cohtml@@SAXPEAX@Z
    Lifted::new(0x7000386a60, tpl_68::<0x7000611970, 0x70005a82d0, 0x7000386a71> as *const (), 0), // +0x386a60 sub_386a60
    Lifted::new(0x7000386a80, fn_7000386a80 as *const (), 0), // +0x386a80 sub_386a80
    Lifted::new(0x7000386e10, fn_7000386e10 as *const (), 0), // +0x386e10 sub_386e10
    Lifted::new(0x7000387000, fn_7000387000 as *const (), 0), // +0x387000 sub_387000
    Lifted::new(0x7000387200, fn_7000387200 as *const (), 0), // +0x387200 sub_387200
    Lifted::new(0x70003872e0, fn_70003872e0 as *const (), 0), // +0x3872e0 sub_3872e0
    Lifted::new(0x7000387440, fn_7000387440 as *const (), 0), // +0x387440 sub_387440
    Lifted::new(0x70003874e0, fn_70003874e0 as *const (), 2), // +0x3874e0 sub_3874e0
    Lifted::new(0x70003875e0, fn_70003875e0 as *const (), 2), // +0x3875e0 sub_3875e0
    Lifted::new(0x7000387700, fn_7000387700 as *const (), 0), // +0x387700 sub_387700
    Lifted::new(0x7000387880, fn_7000387880 as *const (), 0), // +0x387880 sub_387880
    Lifted::new(0x7000387980, fn_7000387980 as *const (), 1), // +0x387980 sub_387980
    Lifted::new(0x7000387e30, fn_7000387e30 as *const (), 1), // +0x387e30 sub_387e30
    Lifted::new(0x7000387fe0, fn_7000387fe0 as *const (), 1), // +0x387fe0 sub_387fe0
    Lifted::new(0x7000388350, fn_7000388350 as *const (), 0), // +0x388350 sub_388350
    Lifted::new(0x70003883d0, fn_70003883d0 as *const (), 0), // +0x3883d0 sub_3883d0
    Lifted::new(0x70003885b0, fn_70003885b0 as *const (), 1), // +0x3885b0 sub_3885b0
    Lifted::new(0x7000388810, fn_7000388810 as *const (), 0), // +0x388810 sub_388810
    Lifted::new(0x7000388bc0, fn_7000388bc0 as *const (), 0), // +0x388bc0 sub_388bc0
    Lifted::new(0x7000388c60, fn_7000388c60 as *const (), 0), // +0x388c60 sub_388c60
    Lifted::new(0x7000388d00, tpl_25::<0x7000694974, 0x7000699360, 0x70005a4174, 0x70005a6aa0, 0x7000699370, 0x7000699388, 0x70006993a0, 0x70006993b8, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x388d00 sub_388d00
    Lifted::new(0x7000388d80, tpl_25::<0x7000694974, 0x70006992dc, 0x70005a4174, 0x70005a6a90, 0x70006992e0, 0x7000699300, 0x7000699320, 0x7000699340, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x388d80 sub_388d80
    Lifted::new(0x7000388e00, tpl_244::<0x18, 0x8, 0x7000388f50> as *const (), 0), // +0x388e00 sub_388e00
    Lifted::new(0x7000388ea0, tpl_116::<0x18, 0x8, 0x10, 0x70003891d0> as *const (), 0), // +0x388ea0 sub_388ea0
    Lifted::new(0x7000388f50, fn_7000388f50 as *const (), 2), // +0x388f50 sub_388f50
    Lifted::new(0x70003891d0, fn_70003891d0 as *const (), 2), // +0x3891d0 sub_3891d0
    Lifted::new(0x7000389450, fn_7000389450 as *const (), 2), // +0x389450 sub_389450
    Lifted::new(0x7000389720, fn_7000389720 as *const (), 2), // +0x389720 sub_389720
    Lifted::new(0x70003899e0, fn_70003899e0 as *const (), 0), // +0x3899e0 ?Invoke@?$CoherentToString@PEBD@cohtml@@SA_NPEBDPEADPEA_K@Z
    Lifted::new(0x7000389a50, fn_7000389a50 as *const (), 0), // +0x389a50 ?Invoke@?$CoherentToString@PEAD@cohtml@@SA_NPEAD0PEA_K@Z
    Lifted::new(0x7000389ac0, fn_7000389ac0 as *const (), 0), // +0x389ac0 ?Invoke@?$CoherentToString@F@cohtml@@SA_NFPEADPEA_K@Z
    Lifted::new(0x7000389b00, fn_7000389b00 as *const (), 0), // +0x389b00 ?Invoke@?$CoherentToString@G@cohtml@@SA_NGPEADPEA_K@Z
    Lifted::new(0x7000389b40, tpl_336::<0x70005fe728, 0x7000231b20> as *const (), 0), // +0x389b40 ?Invoke@?$CoherentToString@H@cohtml@@SA_NHPEADPEA_K@Z
    Lifted::new(0x7000389b80, tpl_336::<0x70005fb9c4, 0x7000231b20> as *const (), 0), // +0x389b80 ?Invoke@?$CoherentToString@I@cohtml@@SA_NIPEADPEA_K@Z
    Lifted::new(0x7000389bc0, tpl_337::<0x7000611a08, 0x7000231b20> as *const (), 0), // +0x389bc0 ?Invoke@?$CoherentToString@_J@cohtml@@SA_N_JPEADPEA_K@Z
    Lifted::new(0x7000389c00, tpl_337::<0x700060c720, 0x7000231b20> as *const (), 0), // +0x389c00 ?Invoke@?$CoherentToString@_K@cohtml@@SA_N_KPEADPEA_K@Z
    Lifted::new(0x7000389c40, tpl_336::<0x7000611a04, 0x7000231b20> as *const (), 0), // +0x389c40 ?Invoke@?$CoherentToString@J@cohtml@@SA_NJPEADPEA_K@Z
    Lifted::new(0x7000389c80, tpl_336::<0x7000611a1c, 0x7000231b20> as *const (), 0), // +0x389c80 ?Invoke@?$CoherentToString@K@cohtml@@SA_NKPEADPEA_K@Z
    Lifted::new(0x7000389cc0, fn_7000389cc0 as *const (), 2), // +0x389cc0 ?Invoke@?$CoherentToString@M@cohtml@@SA_NMPEADPEA_K@Z
    Lifted::new(0x7000389d00, fn_7000389d00 as *const (), 2), // +0x389d00 ?Invoke@?$CoherentToString@N@cohtml@@SA_NNPEADPEA_K@Z
    Lifted::new(0x7000389d40, fn_7000389d40 as *const (), 0), // +0x389d40 ?Invoke@?$CoherentToString@_N@cohtml@@SA_N_NPEADPEA_K@Z
    Lifted::new(0x7000389d90, fn_7000389d90 as *const (), 0), // +0x389d90 sub_389d90
    Lifted::new(0x7000389e10, fn_7000389e10 as *const (), 3), // +0x389e10 sub_389e10
    Lifted::new(0x7000389ec0, fn_7000389ec0 as *const (), 2), // +0x389ec0 sub_389ec0
    Lifted::new(0x700038a2a0, fn_700038a2a0 as *const (), 3), // +0x38a2a0 sub_38a2a0
    Lifted::new(0x700038a3d0, fn_700038a3d0 as *const (), 1), // +0x38a3d0 sub_38a3d0
    Lifted::new(0x700038a400, fn_700038a400 as *const (), 2), // +0x38a400 sub_38a400
    Lifted::new(0x700038a650, fn_700038a650 as *const (), 2), // +0x38a650 sub_38a650
    Lifted::new(0x700038a9b0, fn_700038a9b0 as *const (), 3), // +0x38a9b0 sub_38a9b0
    Lifted::new(0x700038aa50, fn_700038aa50 as *const (), 0), // +0x38aa50 sub_38aa50
    Lifted::new(0x700038aa90, fn_700038aa90 as *const (), 0), // +0x38aa90 sub_38aa90
    Lifted::new(0x700038ab00, fn_700038ab00 as *const (), 2), // +0x38ab00 sub_38ab00
    Lifted::new(0x700038b2d0, fn_700038b2d0 as *const (), 0), // +0x38b2d0 sub_38b2d0
    Lifted::new(0x700038b3c0, fn_700038b3c0 as *const (), 0), // +0x38b3c0 sub_38b3c0
    Lifted::new(0x700038b400, fn_700038b400 as *const (), 2), // +0x38b400 sub_38b400
    Lifted::new(0x700038b490, fn_700038b490 as *const (), 2), // +0x38b490 sub_38b490
    Lifted::new(0x700038b790, fn_700038b790 as *const (), 2), // +0x38b790 sub_38b790
    Lifted::new(0x700038be90, fn_700038be90 as *const (), 2), // +0x38be90 sub_38be90
    Lifted::new(0x700038c4b0, fn_700038c4b0 as *const (), 2), // +0x38c4b0 sub_38c4b0
    Lifted::new(0x700038c720, fn_700038c720 as *const (), 2), // +0x38c720 sub_38c720
    Lifted::new(0x700038d0c0, fn_700038d0c0 as *const (), 2), // +0x38d0c0 sub_38d0c0
    Lifted::new(0x700038d3f0, fn_700038d3f0 as *const (), 0), // +0x38d3f0 sub_38d3f0
    Lifted::new(0x700038d520, fn_700038d520 as *const (), 2), // +0x38d520 sub_38d520
    Lifted::new(0x700038db00, fn_700038db00 as *const (), 2), // +0x38db00 sub_38db00
    Lifted::new(0x700038f1d0, fn_700038f1d0 as *const (), 2), // +0x38f1d0 sub_38f1d0
    Lifted::new(0x700038f860, fn_700038f860 as *const (), 2), // +0x38f860 sub_38f860
    Lifted::new(0x7000390940, fn_7000390940 as *const (), 2), // +0x390940 sub_390940
    Lifted::new(0x7000390fc0, fn_7000390fc0 as *const (), 2), // +0x390fc0 sub_390fc0
    Lifted::new(0x7000391800, fn_7000391800 as *const (), 2), // +0x391800 sub_391800
    Lifted::new(0x7000393190, fn_7000393190 as *const (), 2), // +0x393190 sub_393190
    Lifted::new(0x70003935a0, fn_70003935a0 as *const (), 2), // +0x3935a0 sub_3935a0
    Lifted::new(0x7000393790, fn_7000393790 as *const (), 2), // +0x393790 sub_393790
    Lifted::new(0x7000393f50, fn_7000393f50 as *const (), 0), // +0x393f50 sub_393f50
    Lifted::new(0x7000394030, tpl_338::<0xc, 0x8, 0x70003943f0> as *const (), 0), // +0x394030 sub_394030
    Lifted::new(0x70003940f0, fn_70003940f0 as *const (), 2), // +0x3940f0 sub_3940f0
    Lifted::new(0x70003941f0, fn_70003941f0 as *const (), 2), // +0x3941f0 sub_3941f0
    Lifted::new(0x70003942e0, fn_70003942e0 as *const (), 2), // +0x3942e0 sub_3942e0
    Lifted::new(0x70003943f0, tpl_133::<0xc, 0x8, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000394570> as *const (), 2), // +0x3943f0 sub_3943f0
    Lifted::new(0x70003944b0, fn_70003944b0 as *const (), 0), // +0x3944b0 sub_3944b0
    Lifted::new(0x7000394570, fn_7000394570 as *const (), 0), // +0x394570 sub_394570
    Lifted::new(0x7000394660, fn_7000394660 as *const (), 1), // +0x394660 sub_394660
    Lifted::new(0x7000394710, fn_7000394710 as *const (), 3), // +0x394710 sub_394710
    Lifted::new(0x70003947b0, fn_70003947b0 as *const (), 0), // +0x3947b0 sub_3947b0
    Lifted::new(0x7000394890, fn_7000394890 as *const (), 2), // +0x394890 sub_394890
    Lifted::new(0x7000394da0, fn_7000394da0 as *const (), 0), // +0x394da0 sub_394da0
    Lifted::new(0x7000394f10, tpl_282::<0x70001f4230, 0x8, 0x7000395690, 0xffffffffffffffff, 0x7000697150, 0x10> as *const (), 0), // +0x394f10 sub_394f10
    Lifted::new(0x7000394f70, fn_7000394f70 as *const (), 2), // +0x394f70 sub_394f70
    Lifted::new(0x7000394fc0, tpl_159::<0x10> as *const (), 1), // +0x394fc0 sub_394fc0
    Lifted::new(0x7000394fe0, fn_7000394fe0 as *const (), 2), // +0x394fe0 sub_394fe0
    Lifted::new(0x70003953d0, fn_70003953d0 as *const (), 2), // +0x3953d0 sub_3953d0
    Lifted::new(0x7000395690, fn_7000395690 as *const (), 0), // +0x395690 sub_395690
    Lifted::new(0x7000395740, tpl_39::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70001f91c0> as *const (), 2), // +0x395740 sub_395740
    Lifted::new(0x70003957d0, fn_70003957d0 as *const (), 0), // +0x3957d0 sub_3957d0
    Lifted::new(0x7000395950, fn_7000395950 as *const (), 0), // +0x395950 sub_395950
    Lifted::new(0x70003959e0, tpl_210::<0x7000394da0> as *const (), 0), // +0x3959e0 sub_3959e0
    Lifted::new(0x7000395a00, tpl_162::<0x7000611bb0, 0x70001fa8c0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x395a00 sub_395a00
    Lifted::new(0x7000395a80, tpl_202::<0x7000611bc0, 0x7000394f10, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x395a80 sub_395a80
    Lifted::new(0x7000395b00, fn_7000395b00 as *const (), 2), // +0x395b00 sub_395b00
    Lifted::new(0x7000395c80, fn_7000395c80 as *const (), 2), // +0x395c80 sub_395c80
    Lifted::new(0x7000395f70, fn_7000395f70 as *const (), 3), // +0x395f70 sub_395f70
    Lifted::new(0x7000396210, fn_7000396210 as *const (), 2), // +0x396210 sub_396210
    Lifted::new(0x70003962f0, fn_70003962f0 as *const (), 2), // +0x3962f0 sub_3962f0
    Lifted::new(0x70003963e0, fn_70003963e0 as *const (), 2), // +0x3963e0 sub_3963e0
    Lifted::new(0x7000396490, fn_7000396490 as *const (), 0), // +0x396490 sub_396490
    Lifted::new(0x7000396580, tpl_156::<0x7000611c70, 0x700038aa90, 0x700048c2c0> as *const (), 0), // +0x396580 sub_396580
    Lifted::new(0x70003965c0, tpl_152::<0x10c, 0x10, 0xc, 0x700062e214, 0x700062e9b0, 0x70001f3aa0, 0x7000396650, 0x7000391800> as *const (), 2), // +0x3965c0 sub_3965c0
    Lifted::new(0x7000396650, fn_7000396650 as *const (), 2), // +0x396650 sub_396650
    Lifted::new(0x7000396780, tpl_183::<0x7000611c98, 0x7000611e60, 0xd8, 0x700024baa0, 0x700048c2c0> as *const (), 0), // +0x396780 sub_396780
    Lifted::new(0x70003967d0, tpl_339::<0x30, 0x2bd0, 0x700024bc60> as *const (), 0), // +0x3967d0 sub_3967d0
    Lifted::new(0x70003967e0, tpl_206::<0x70003c7150, 0x30, 0x2bd0> as *const (), 0), // +0x3967e0 sub_3967e0
    Lifted::new(0x7000396810, tpl_340::<0xa0, 0x30, 0xc4, 0x70003f7d00, 0x18, 0x10, 0x1a4, 0x1a8, 0x70003a0c10, 0x700010b4f0, 0x2bc8, 0x90> as *const (), 2), // +0x396810 sub_396810
    Lifted::new(0x70003968f0, tpl_340::<0xa0, 0x30, 0xc0, 0x70003f7d00, 0x18, 0x10, 0x1a0, 0x1a9, 0x70003a0c10, 0x700010b4f0, 0x2bc8, 0x98> as *const (), 2), // +0x3968f0 sub_3968f0
    Lifted::new(0x70003969d0, tpl_341::<0x30, 0x2bc8, 0x88> as *const (), 2), // +0x3969d0 sub_3969d0
    Lifted::new(0x70003969f0, tpl_341::<0x30, 0x2bc8, 0x80> as *const (), 2), // +0x3969f0 sub_3969f0
    Lifted::new(0x7000396a10, tpl_184::<0x7000697150, 0x25, 0x30, 0x70003bfea0, 0x20, 0x7000611c98, 0x7000611e60, 0x1c8, 0xd8, 0x70003c3f40, 0x7000602bd8, 0x70001c30e0> as *const (), 0), // +0x396a10 sub_396a10
    Lifted::new(0x7000396aec, tpl_185::<0x7000396780> as *const (), 0), // +0x396aec sub_396aec
    Lifted::new(0x7000396b00, tpl_184::<0x7000697150, 0x25, 0x30, 0x70003bfea0, 0x20, 0x7000611ee8, 0x7000602bf8, 0x1c8, 0xd8, 0x70003c3f40, 0x7000602bd8, 0x70001c30e0> as *const (), 0), // +0x396b00 sub_396b00
    Lifted::new(0x7000396be0, fn_7000396be0 as *const (), 2), // +0x396be0 sub_396be0
    Lifted::new(0x7000397990, fn_7000397990 as *const (), 2), // +0x397990 sub_397990
    Lifted::new(0x7000397e10, fn_7000397e10 as *const (), 2), // +0x397e10 sub_397e10
    Lifted::new(0x7000398130, fn_7000398130 as *const (), 0), // +0x398130 sub_398130
    Lifted::new(0x7000398bb0, fn_7000398bb0 as *const (), 0), // +0x398bb0 sub_398bb0
    Lifted::new(0x7000398cc0, fn_7000398cc0 as *const (), 0), // +0x398cc0 sub_398cc0
    Lifted::new(0x7000398db0, fn_7000398db0 as *const (), 2), // +0x398db0 sub_398db0
    Lifted::new(0x7000398f10, fn_7000398f10 as *const (), 0), // +0x398f10 sub_398f10
    Lifted::new(0x7000399c30, tpl_183::<0x7000612138, 0x7000612300, 0xd8, 0x700024baa0, 0x700048c2c0> as *const (), 0), // +0x399c30 sub_399c30
    Lifted::new(0x7000399c80, tpl_184::<0x7000697150, 0x25, 0x30, 0x70003bfea0, 0x20, 0x7000612138, 0x7000612300, 0x1c8, 0xd8, 0x70003c3f40, 0x7000602bd8, 0x70001c30e0> as *const (), 0), // +0x399c80 sub_399c80
    Lifted::new(0x7000399d5c, tpl_185::<0x7000399c30> as *const (), 0), // +0x399d5c sub_399d5c
    Lifted::new(0x7000399d70, fn_7000399d70 as *const (), 0), // +0x399d70 sub_399d70
    Lifted::new(0x7000399e00, tpl_342::<0x7000613c50, 0x700048c2c0> as *const (), 0), // +0x399e00 sub_399e00
    Lifted::new(0x7000399e30, tpl_180::<0x8, 0x18, 0x58> as *const (), 0), // +0x399e30 sub_399e30
    Lifted::new(0x7000399e70, tpl_343::<0x700010b4f0, 0x7000613c50, 0x700048c2c0> as *const (), 0), // +0x399e70 sub_399e70
    Lifted::new(0x7000399ec0, fn_7000399ec0 as *const (), 0), // +0x399ec0 sub_399ec0
    Lifted::new(0x7000399f10, fn_7000399f10 as *const (), 0), // +0x399f10 sub_399f10
    Lifted::new(0x7000399fb0, tpl_343::<0x70001e4150, 0x7000613c50, 0x700048c2c0> as *const (), 0), // +0x399fb0 sub_399fb0
    Lifted::new(0x700039a000, fn_700039a000 as *const (), 0), // +0x39a000 sub_39a000
    Lifted::new(0x700039a060, fn_700039a060 as *const (), 0), // +0x39a060 sub_39a060
    Lifted::new(0x700039a0a0, fn_700039a0a0 as *const (), 0), // +0x39a0a0 sub_39a0a0
    Lifted::new(0x700039a140, fn_700039a140 as *const (), 0), // +0x39a140 sub_39a140
    Lifted::new(0x700039a340, fn_700039a340 as *const (), 0), // +0x39a340 sub_39a340
    Lifted::new(0x700039a380, fn_700039a380 as *const (), 0), // +0x39a380 sub_39a380
    Lifted::new(0x700039a3c0, fn_700039a3c0 as *const (), 0), // +0x39a3c0 sub_39a3c0
    Lifted::new(0x700039adb0, fn_700039adb0 as *const (), 0), // +0x39adb0 sub_39adb0
    Lifted::new(0x700039adf0, fn_700039adf0 as *const (), 0), // +0x39adf0 sub_39adf0
    Lifted::new(0x700039af00, fn_700039af00 as *const (), 0), // +0x39af00 sub_39af00
    Lifted::new(0x700039b370, tpl_139::<0x38, 0x40, 0x48, 0x50, 0x18, 0x20, 0x28, 0x30, 0x70003a5300, 0x70001ec150> as *const (), 0), // +0x39b370 sub_39b370
    Lifted::new(0x700039b3e0, fn_700039b3e0 as *const (), 0), // +0x39b3e0 sub_39b3e0
    Lifted::new(0x700039b890, fn_700039b890 as *const (), 0), // +0x39b890 sub_39b890
    Lifted::new(0x700039bc20, fn_700039bc20 as *const (), 0), // +0x39bc20 sub_39bc20
    Lifted::new(0x700039bca0, fn_700039bca0 as *const (), 0), // +0x39bca0 sub_39bca0
    Lifted::new(0x700039be10, tpl_182::<0x10, 0x7000607010, 0x7000607008, 0x7000231e90> as *const (), 0), // +0x39be10 sub_39be10
    Lifted::new(0x700039be50, fn_700039be50 as *const (), 0), // +0x39be50 sub_39be50
    Lifted::new(0x700039be80, fn_700039be80 as *const (), 0), // +0x39be80 sub_39be80
    Lifted::new(0x700039ce20, fn_700039ce20 as *const (), 0), // +0x39ce20 sub_39ce20
    Lifted::new(0x700039d0d0, fn_700039d0d0 as *const (), 0), // +0x39d0d0 sub_39d0d0
    Lifted::new(0x700039d410, tpl_344::<0x70001a4330, 0x70005a4d14, 0x7000697150, 0x70005b52e4, 0x7000613088, 0x70001a4530, 0x70003cea90> as *const (), 0), // +0x39d410 sub_39d410
    Lifted::new(0x700039d6c0, tpl_344::<0x70001a4330, 0x70005a4d14, 0x7000697150, 0x70005b52e4, 0x70006130c0, 0x70001a4530, 0x70003cea90> as *const (), 0), // +0x39d6c0 sub_39d6c0
    Lifted::new(0x700039d970, fn_700039d970 as *const (), 2), // +0x39d970 sub_39d970
    Lifted::new(0x700039db80, fn_700039db80 as *const (), 0), // +0x39db80 sub_39db80
    Lifted::new(0x700039dd30, fn_700039dd30 as *const (), 0), // +0x39dd30 sub_39dd30
    Lifted::new(0x700039dde0, fn_700039dde0 as *const (), 2), // +0x39dde0 sub_39dde0
    Lifted::new(0x700039e0b0, fn_700039e0b0 as *const (), 0), // +0x39e0b0 sub_39e0b0
    Lifted::new(0x700039e9b0, fn_700039e9b0 as *const (), 0), // +0x39e9b0 sub_39e9b0
    Lifted::new(0x700039eb40, fn_700039eb40 as *const (), 0), // +0x39eb40 sub_39eb40
    Lifted::new(0x700039ee00, fn_700039ee00 as *const (), 0), // +0x39ee00 sub_39ee00
    Lifted::new(0x700039efe0, fn_700039efe0 as *const (), 0), // +0x39efe0 sub_39efe0
    Lifted::new(0x700039f0c0, fn_700039f0c0 as *const (), 0), // +0x39f0c0 sub_39f0c0
    Lifted::new(0x700039f230, fn_700039f230 as *const (), 0), // +0x39f230 sub_39f230
    Lifted::new(0x700039f3c0, fn_700039f3c0 as *const (), 0), // +0x39f3c0 sub_39f3c0
    Lifted::new(0x700039fa20, fn_700039fa20 as *const (), 2), // +0x39fa20 sub_39fa20
    Lifted::new(0x700039fce0, fn_700039fce0 as *const (), 0), // +0x39fce0 sub_39fce0
    Lifted::new(0x700039fda0, fn_700039fda0 as *const (), 2), // +0x39fda0 sub_39fda0
    Lifted::new(0x700039fe90, fn_700039fe90 as *const (), 0), // +0x39fe90 sub_39fe90
    Lifted::new(0x700039ff00, fn_700039ff00 as *const (), 0), // +0x39ff00 sub_39ff00
    Lifted::new(0x70003a0000, fn_70003a0000 as *const (), 2), // +0x3a0000 sub_3a0000
    Lifted::new(0x70003a0310, tpl_305::<0x88, 0x4, 0x7000612bc0, 0xffffffffffffff78, 0xffffffffffffff80, 0xffffffffffffff74, 0x70003a4730, 0xffffffffffffff88, 0x70005a8150, 0x70005a8138> as *const (), 0), // +0x3a0310 sub_3a0310
    Lifted::new(0x70003a0370, fn_70003a0370 as *const (), 2), // +0x3a0370 sub_3a0370
    Lifted::new(0x70003a07a0, fn_70003a07a0 as *const (), 0), // +0x3a07a0 sub_3a07a0
    Lifted::new(0x70003a08e0, fn_70003a08e0 as *const (), 0), // +0x3a08e0 sub_3a08e0
    Lifted::new(0x70003a0ab0, fn_70003a0ab0 as *const (), 2), // +0x3a0ab0 sub_3a0ab0
    Lifted::new(0x70003a0c10, fn_70003a0c10 as *const (), 2), // +0x3a0c10 sub_3a0c10
    Lifted::new(0x70003a0d40, fn_70003a0d40 as *const (), 2), // +0x3a0d40 sub_3a0d40
    Lifted::new(0x70003a16f0, fn_70003a16f0 as *const (), 0), // +0x3a16f0 sub_3a16f0
    Lifted::new(0x70003a17d0, fn_70003a17d0 as *const (), 0), // +0x3a17d0 sub_3a17d0
    Lifted::new(0x70003a18c0, fn_70003a18c0 as *const (), 2), // +0x3a18c0 sub_3a18c0
    Lifted::new(0x70003a1c80, fn_70003a1c80 as *const (), 2), // +0x3a1c80 sub_3a1c80
    Lifted::new(0x70003a1ee0, fn_70003a1ee0 as *const (), 0), // +0x3a1ee0 sub_3a1ee0
    Lifted::new(0x70003a20a0, fn_70003a20a0 as *const (), 2), // +0x3a20a0 sub_3a20a0
    Lifted::new(0x70003a2290, fn_70003a2290 as *const (), 2), // +0x3a2290 sub_3a2290
    Lifted::new(0x70003a2580, fn_70003a2580 as *const (), 2), // +0x3a2580 sub_3a2580
    Lifted::new(0x70003a27a0, fn_70003a27a0 as *const (), 2), // +0x3a27a0 sub_3a27a0
    Lifted::new(0x70003a2b70, fn_70003a2b70 as *const (), 0), // +0x3a2b70 sub_3a2b70
    Lifted::new(0x70003a30f0, fn_70003a30f0 as *const (), 0), // +0x3a30f0 sub_3a30f0
    Lifted::new(0x70003a3340, fn_70003a3340 as *const (), 2), // +0x3a3340 sub_3a3340
    Lifted::new(0x70003a36e0, fn_70003a36e0 as *const (), 0), // +0x3a36e0 sub_3a36e0
    Lifted::new(0x70003a3980, fn_70003a3980 as *const (), 0), // +0x3a3980 sub_3a3980
    Lifted::new(0x70003a3a10, fn_70003a3a10 as *const (), 2), // +0x3a3a10 sub_3a3a10
    Lifted::new(0x70003a3f70, fn_70003a3f70 as *const (), 0), // +0x3a3f70 sub_3a3f70
    Lifted::new(0x70003a4020, fn_70003a4020 as *const (), 2), // +0x3a4020 sub_3a4020
    Lifted::new(0x70003a4210, fn_70003a4210 as *const (), 0), // +0x3a4210 sub_3a4210
    Lifted::new(0x70003a4370, fn_70003a4370 as *const (), 0), // +0x3a4370 sub_3a4370
    Lifted::new(0x70003a44c0, fn_70003a44c0 as *const (), 0), // +0x3a44c0 sub_3a44c0
    Lifted::new(0x70003a45b0, fn_70003a45b0 as *const (), 0), // +0x3a45b0 sub_3a45b0
    Lifted::new(0x70003a4730, fn_70003a4730 as *const (), 0), // +0x3a4730 sub_3a4730
    Lifted::new(0x70003a47b0, fn_70003a47b0 as *const (), 0), // +0x3a47b0 sub_3a47b0
    Lifted::new(0x70003a4820, fn_70003a4820 as *const (), 0), // +0x3a4820 sub_3a4820
    Lifted::new(0x70003a4880, fn_70003a4880 as *const (), 0), // +0x3a4880 sub_3a4880
    Lifted::new(0x70003a4970, tpl_97::<0x8, 0x19, 0x10, 0x18, 0x700010b4f0, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x7000697150, 0x20> as *const (), 0), // +0x3a4970 sub_3a4970
    Lifted::new(0x70003a4a20, fn_70003a4a20 as *const (), 0), // +0x3a4a20 sub_3a4a20
    Lifted::new(0x70003a4a90, tpl_345::<0x700062e254, 0x70005ded38, 0x70005ded30, 0x700010b4f0, 0x700004da80, 0x700004da10> as *const (), 2), // +0x3a4a90 sub_3a4a90
    Lifted::new(0x70003a4c20, tpl_306::<0x70003a4730, 0x700048c2c0> as *const (), 0), // +0x3a4c20 sub_3a4c20
    Lifted::new(0x70003a4c60, tpl_203::<0xffffffffffffff78, 0x70003a0310, 0x700048c2c0> as *const (), 0), // +0x3a4c60 sub_3a4c60
    Lifted::new(0x70003a4ca0, fn_70003a4ca0 as *const (), 2), // +0x3a4ca0 sub_3a4ca0
    Lifted::new(0x70003a4df0, fn_70003a4df0 as *const (), 0), // +0x3a4df0 sub_3a4df0
    Lifted::new(0x70003a4e90, tpl_24::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70003a5900> as *const (), 2), // +0x3a4e90 sub_3a4e90
    Lifted::new(0x70003a4f30, fn_70003a4f30 as *const (), 0), // +0x3a4f30 sub_3a4f30
    Lifted::new(0x70003a4f90, fn_70003a4f90 as *const (), 0), // +0x3a4f90 sub_3a4f90
    Lifted::new(0x70003a5180, tpl_346::<0x8, 0x19, 0x10, 0x70003b6e10, 0x7000697150, 0x20> as *const (), 0), // +0x3a5180 sub_3a5180
    Lifted::new(0x70003a5240, tpl_346::<0x8, 0x19, 0x10, 0x70003b6ed0, 0x7000697150, 0x20> as *const (), 0), // +0x3a5240 sub_3a5240
    Lifted::new(0x70003a5300, tpl_135::<0x700010b4f0, 0x10, 0x8, 0x18> as *const (), 0), // +0x3a5300 sub_3a5300
    Lifted::new(0x70003a5370, tpl_278::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x700004da10> as *const (), 2), // +0x3a5370 sub_3a5370
    Lifted::new(0x70003a5460, fn_70003a5460 as *const (), 0), // +0x3a5460 sub_3a5460
    Lifted::new(0x70003a54e0, fn_70003a54e0 as *const (), 0), // +0x3a54e0 sub_3a54e0
    Lifted::new(0x70003a5590, fn_70003a5590 as *const (), 2), // +0x3a5590 sub_3a5590
    Lifted::new(0x70003a5680, tpl_99::<0x8, 0x70003b7430, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x3a5680 sub_3a5680
    Lifted::new(0x70003a5700, tpl_25::<0x7000694974, 0x7000699620, 0x70005a4174, 0x70005a6be0, 0x7000699630, 0x7000699648, 0x7000699660, 0x7000699678, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3a5700 sub_3a5700
    Lifted::new(0x70003a5780, tpl_25::<0x7000694974, 0x700069b308, 0x70005a4174, 0x70005a6bd0, 0x700069b310, 0x700069b338, 0x700069b360, 0x700069b388, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3a5780 sub_3a5780
    Lifted::new(0x70003a5800, tpl_25::<0x7000694974, 0x70006995b8, 0x70005a4174, 0x70005a6bc0, 0x70006995c0, 0x70006995d8, 0x70006995f0, 0x7000699608, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3a5800 sub_3a5800
    Lifted::new(0x70003a5880, tpl_25::<0x7000694974, 0x700069b3b0, 0x70005a4174, 0x70005a6bb0, 0x700069b3c0, 0x700069b3d8, 0x700069b3f0, 0x700069b408, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3a5880 sub_3a5880
    Lifted::new(0x70003a5900, fn_70003a5900 as *const (), 0), // +0x3a5900 sub_3a5900
    Lifted::new(0x70003a59b0, fn_70003a59b0 as *const (), 0), // +0x3a59b0 sub_3a59b0
    Lifted::new(0x70003a5b30, fn_70003a5b30 as *const (), 2), // +0x3a5b30 sub_3a5b30
    Lifted::new(0x70003a5e00, fn_70003a5e00 as *const (), 2), // +0x3a5e00 sub_3a5e00
    Lifted::new(0x70003a60d0, fn_70003a60d0 as *const (), 2), // +0x3a60d0 sub_3a60d0
    Lifted::new(0x70003a63a0, fn_70003a63a0 as *const (), 2), // +0x3a63a0 sub_3a63a0
    Lifted::new(0x70003a6670, fn_70003a6670 as *const (), 2), // +0x3a6670 sub_3a6670
    Lifted::new(0x70003a6940, fn_70003a6940 as *const (), 2), // +0x3a6940 sub_3a6940
    Lifted::new(0x70003a6c10, fn_70003a6c10 as *const (), 2), // +0x3a6c10 sub_3a6c10
    Lifted::new(0x70003a6ee0, fn_70003a6ee0 as *const (), 2), // +0x3a6ee0 sub_3a6ee0
    Lifted::new(0x70003a71b0, fn_70003a71b0 as *const (), 2), // +0x3a71b0 sub_3a71b0
    Lifted::new(0x70003a7480, fn_70003a7480 as *const (), 2), // +0x3a7480 sub_3a7480
    Lifted::new(0x70003a7750, fn_70003a7750 as *const (), 2), // +0x3a7750 sub_3a7750
    Lifted::new(0x70003a7970, fn_70003a7970 as *const (), 2), // +0x3a7970 sub_3a7970
    Lifted::new(0x70003a7b90, fn_70003a7b90 as *const (), 2), // +0x3a7b90 sub_3a7b90
    Lifted::new(0x70003a7db0, fn_70003a7db0 as *const (), 2), // +0x3a7db0 sub_3a7db0
    Lifted::new(0x70003a7fd0, fn_70003a7fd0 as *const (), 2), // +0x3a7fd0 sub_3a7fd0
    Lifted::new(0x70003a82a0, fn_70003a82a0 as *const (), 2), // +0x3a82a0 sub_3a82a0
    Lifted::new(0x70003a8570, fn_70003a8570 as *const (), 2), // +0x3a8570 sub_3a8570
    Lifted::new(0x70003a8840, fn_70003a8840 as *const (), 2), // +0x3a8840 sub_3a8840
    Lifted::new(0x70003a8b10, fn_70003a8b10 as *const (), 2), // +0x3a8b10 sub_3a8b10
    Lifted::new(0x70003a8de0, fn_70003a8de0 as *const (), 2), // +0x3a8de0 sub_3a8de0
    Lifted::new(0x70003a90b0, fn_70003a90b0 as *const (), 2), // +0x3a90b0 sub_3a90b0
    Lifted::new(0x70003a9380, fn_70003a9380 as *const (), 2), // +0x3a9380 sub_3a9380
    Lifted::new(0x70003a9650, fn_70003a9650 as *const (), 2), // +0x3a9650 sub_3a9650
    Lifted::new(0x70003a9880, fn_70003a9880 as *const (), 2), // +0x3a9880 sub_3a9880
    Lifted::new(0x70003a9aa0, fn_70003a9aa0 as *const (), 2), // +0x3a9aa0 sub_3a9aa0
    Lifted::new(0x70003a9d80, fn_70003a9d80 as *const (), 2), // +0x3a9d80 sub_3a9d80
    Lifted::new(0x70003aa080, fn_70003aa080 as *const (), 2), // +0x3aa080 sub_3aa080
    Lifted::new(0x70003aa380, fn_70003aa380 as *const (), 2), // +0x3aa380 sub_3aa380
    Lifted::new(0x70003aa5b0, fn_70003aa5b0 as *const (), 0), // +0x3aa5b0 sub_3aa5b0
    Lifted::new(0x70003aa930, fn_70003aa930 as *const (), 0), // +0x3aa930 sub_3aa930
    Lifted::new(0x70003aacb0, fn_70003aacb0 as *const (), 2), // +0x3aacb0 sub_3aacb0
    Lifted::new(0x70003aaef0, fn_70003aaef0 as *const (), 2), // +0x3aaef0 sub_3aaef0
    Lifted::new(0x70003ab130, fn_70003ab130 as *const (), 2), // +0x3ab130 sub_3ab130
    Lifted::new(0x70003ab3e0, fn_70003ab3e0 as *const (), 2), // +0x3ab3e0 sub_3ab3e0
    Lifted::new(0x70003ab620, fn_70003ab620 as *const (), 2), // +0x3ab620 sub_3ab620
    Lifted::new(0x70003ab8c0, fn_70003ab8c0 as *const (), 2), // +0x3ab8c0 sub_3ab8c0
    Lifted::new(0x70003abb70, fn_70003abb70 as *const (), 2), // +0x3abb70 sub_3abb70
    Lifted::new(0x70003abe20, fn_70003abe20 as *const (), 2), // +0x3abe20 sub_3abe20
    Lifted::new(0x70003ac0d0, fn_70003ac0d0 as *const (), 2), // +0x3ac0d0 sub_3ac0d0
    Lifted::new(0x70003ac380, fn_70003ac380 as *const (), 2), // +0x3ac380 sub_3ac380
    Lifted::new(0x70003ac5c0, fn_70003ac5c0 as *const (), 2), // +0x3ac5c0 sub_3ac5c0
    Lifted::new(0x70003ac800, fn_70003ac800 as *const (), 2), // +0x3ac800 sub_3ac800
    Lifted::new(0x70003aca40, fn_70003aca40 as *const (), 2), // +0x3aca40 sub_3aca40
    Lifted::new(0x70003acc80, fn_70003acc80 as *const (), 2), // +0x3acc80 sub_3acc80
    Lifted::new(0x70003aceb0, fn_70003aceb0 as *const (), 2), // +0x3aceb0 sub_3aceb0
    Lifted::new(0x70003ad0e0, fn_70003ad0e0 as *const (), 2), // +0x3ad0e0 sub_3ad0e0
    Lifted::new(0x70003ad310, fn_70003ad310 as *const (), 2), // +0x3ad310 sub_3ad310
    Lifted::new(0x70003ad540, fn_70003ad540 as *const (), 2), // +0x3ad540 sub_3ad540
    Lifted::new(0x70003ad810, fn_70003ad810 as *const (), 2), // +0x3ad810 sub_3ad810
    Lifted::new(0x70003adac0, fn_70003adac0 as *const (), 2), // +0x3adac0 sub_3adac0
    Lifted::new(0x70003add90, fn_70003add90 as *const (), 2), // +0x3add90 sub_3add90
    Lifted::new(0x70003adfb0, fn_70003adfb0 as *const (), 2), // +0x3adfb0 sub_3adfb0
    Lifted::new(0x70003ae1d0, fn_70003ae1d0 as *const (), 2), // +0x3ae1d0 sub_3ae1d0
    Lifted::new(0x70003ae3f0, fn_70003ae3f0 as *const (), 2), // +0x3ae3f0 sub_3ae3f0
    Lifted::new(0x70003ae6a0, fn_70003ae6a0 as *const (), 2), // +0x3ae6a0 sub_3ae6a0
    Lifted::new(0x70003ae970, fn_70003ae970 as *const (), 2), // +0x3ae970 sub_3ae970
    Lifted::new(0x70003aec40, fn_70003aec40 as *const (), 2), // +0x3aec40 sub_3aec40
    Lifted::new(0x70003aee80, fn_70003aee80 as *const (), 2), // +0x3aee80 sub_3aee80
    Lifted::new(0x70003af0a0, fn_70003af0a0 as *const (), 2), // +0x3af0a0 sub_3af0a0
    Lifted::new(0x70003af350, fn_70003af350 as *const (), 2), // +0x3af350 sub_3af350
    Lifted::new(0x70003af590, fn_70003af590 as *const (), 2), // +0x3af590 sub_3af590
    Lifted::new(0x70003af8a0, fn_70003af8a0 as *const (), 2), // +0x3af8a0 sub_3af8a0
    Lifted::new(0x70003afac0, fn_70003afac0 as *const (), 2), // +0x3afac0 sub_3afac0
    Lifted::new(0x70003afdb0, fn_70003afdb0 as *const (), 2), // +0x3afdb0 sub_3afdb0
    Lifted::new(0x70003b00a0, fn_70003b00a0 as *const (), 2), // +0x3b00a0 sub_3b00a0
    Lifted::new(0x70003b0390, fn_70003b0390 as *const (), 0), // +0x3b0390 sub_3b0390
    Lifted::new(0x70003b0710, fn_70003b0710 as *const (), 0), // +0x3b0710 sub_3b0710
    Lifted::new(0x70003b0990, fn_70003b0990 as *const (), 2), // +0x3b0990 sub_3b0990
    Lifted::new(0x70003b0bc0, fn_70003b0bc0 as *const (), 2), // +0x3b0bc0 sub_3b0bc0
    Lifted::new(0x70003b0df0, fn_70003b0df0 as *const (), 2), // +0x3b0df0 sub_3b0df0
    Lifted::new(0x70003b1020, fn_70003b1020 as *const (), 2), // +0x3b1020 sub_3b1020
    Lifted::new(0x70003b1240, fn_70003b1240 as *const (), 2), // +0x3b1240 sub_3b1240
    Lifted::new(0x70003b1470, fn_70003b1470 as *const (), 2), // +0x3b1470 sub_3b1470
    Lifted::new(0x70003b16b0, fn_70003b16b0 as *const (), 2), // +0x3b16b0 sub_3b16b0
    Lifted::new(0x70003b18d0, fn_70003b18d0 as *const (), 2), // +0x3b18d0 sub_3b18d0
    Lifted::new(0x70003b1af0, fn_70003b1af0 as *const (), 2), // +0x3b1af0 sub_3b1af0
    Lifted::new(0x70003b1d10, fn_70003b1d10 as *const (), 2), // +0x3b1d10 sub_3b1d10
    Lifted::new(0x70003b1f30, fn_70003b1f30 as *const (), 2), // +0x3b1f30 sub_3b1f30
    Lifted::new(0x70003b2150, fn_70003b2150 as *const (), 2), // +0x3b2150 sub_3b2150
    Lifted::new(0x70003b2370, fn_70003b2370 as *const (), 2), // +0x3b2370 sub_3b2370
    Lifted::new(0x70003b2590, fn_70003b2590 as *const (), 2), // +0x3b2590 sub_3b2590
    Lifted::new(0x70003b27b0, fn_70003b27b0 as *const (), 2), // +0x3b27b0 sub_3b27b0
    Lifted::new(0x70003b29d0, fn_70003b29d0 as *const (), 2), // +0x3b29d0 sub_3b29d0
    Lifted::new(0x70003b2bf0, fn_70003b2bf0 as *const (), 2), // +0x3b2bf0 sub_3b2bf0
    Lifted::new(0x70003b2e10, fn_70003b2e10 as *const (), 2), // +0x3b2e10 sub_3b2e10
    Lifted::new(0x70003b3050, fn_70003b3050 as *const (), 2), // +0x3b3050 sub_3b3050
    Lifted::new(0x70003b3280, fn_70003b3280 as *const (), 2), // +0x3b3280 sub_3b3280
    Lifted::new(0x70003b34a0, fn_70003b34a0 as *const (), 2), // +0x3b34a0 sub_3b34a0
    Lifted::new(0x70003b36d0, fn_70003b36d0 as *const (), 2), // +0x3b36d0 sub_3b36d0
    Lifted::new(0x70003b3980, fn_70003b3980 as *const (), 2), // +0x3b3980 sub_3b3980
    Lifted::new(0x70003b3ba0, fn_70003b3ba0 as *const (), 2), // +0x3b3ba0 sub_3b3ba0
    Lifted::new(0x70003b3dc0, fn_70003b3dc0 as *const (), 2), // +0x3b3dc0 sub_3b3dc0
    Lifted::new(0x70003b3fe0, tpl_317::<0x70005a8240, 0x70006125e0, 0x70002445b0, 0x7000612610, 0x70005a8230> as *const (), 0), // +0x3b3fe0 sub_3b3fe0
    Lifted::new(0x70003b4150, fn_70003b4150 as *const (), 2), // +0x3b4150 sub_3b4150
    Lifted::new(0x70003b4b80, tpl_347::<0x70002452e0, 0x70005a8b00, 0x7000269680, 0x7000697150, 0x70001328c0, 0x70003a3980, 0x70006974b8, 0x7000612938, 0x70005f84a0, 0x7000130980, 0x70003a4a90, 0x7000432f50, 0x70003a2290, 0x700042f260, 0x7000432d30, 0x70003b6650, 0x70003b7910, 0x70003b59a0, 0x70003b7ac0, 0x700010b4f0, 0x700004da10, 0x70003b68c0, 0x70001327c0, 0x70003b6760> as *const (), 2), // +0x3b4b80 sub_3b4b80
    Lifted::new(0x70003b5200, tpl_347::<0x70002452e0, 0x70005a8b00, 0x7000269680, 0x7000697150, 0x70001328c0, 0x70003a3980, 0x70006974b8, 0x7000612938, 0x70005f84a0, 0x7000130980, 0x70003a4a90, 0x7000433440, 0x70003a2290, 0x700042f260, 0x7000433170, 0x70003b6650, 0x70003b7910, 0x70003b59a0, 0x70003b7ac0, 0x700010b4f0, 0x700004da10, 0x70003b68c0, 0x70001327c0, 0x70003b6760> as *const (), 2), // +0x3b5200 sub_3b5200
    Lifted::new(0x70003b5880, fn_70003b5880 as *const (), 2), // +0x3b5880 sub_3b5880
    Lifted::new(0x70003b59a0, fn_70003b59a0 as *const (), 0), // +0x3b59a0 sub_3b59a0
    Lifted::new(0x70003b5b50, tpl_226::<0x70005a8240, 0x7000612748, 0x70002445b0, 0x7000612620, 0x70005a8230> as *const (), 0), // +0x3b5b50 sub_3b5b50
    Lifted::new(0x70003b5ca0, fn_70003b5ca0 as *const (), 2), // +0x3b5ca0 sub_3b5ca0
    Lifted::new(0x70003b5ee0, tpl_91::<0x7000007030, 0x70001a86c0, 0x70003b8030> as *const (), 0), // +0x3b5ee0 sub_3b5ee0
    Lifted::new(0x70003b5fd0, fn_70003b5fd0 as *const (), 0), // +0x3b5fd0 sub_3b5fd0
    Lifted::new(0x70003b60d0, tpl_226::<0x70005a8240, 0x7000612820, 0x70002445b0, 0x7000612840, 0x70005a8230> as *const (), 0), // +0x3b60d0 sub_3b60d0
    Lifted::new(0x70003b6220, fn_70003b6220 as *const (), 0), // +0x3b6220 sub_3b6220
    Lifted::new(0x70003b62d0, tpl_244::<0x18, 0x8, 0x70003b8740> as *const (), 0), // +0x3b62d0 sub_3b62d0
    Lifted::new(0x70003b6370, tpl_244::<0x18, 0x8, 0x70003b8c30> as *const (), 0), // +0x3b6370 sub_3b6370
    Lifted::new(0x70003b6410, fn_70003b6410 as *const (), 0), // +0x3b6410 sub_3b6410
    Lifted::new(0x70003b64d0, fn_70003b64d0 as *const (), 0), // +0x3b64d0 sub_3b64d0
    Lifted::new(0x70003b6530, fn_70003b6530 as *const (), 0), // +0x3b6530 sub_3b6530
    Lifted::new(0x70003b65c0, fn_70003b65c0 as *const (), 0), // +0x3b65c0 sub_3b65c0
    Lifted::new(0x70003b6650, fn_70003b6650 as *const (), 0), // +0x3b6650 sub_3b6650
    Lifted::new(0x70003b6760, fn_70003b6760 as *const (), 0), // +0x3b6760 sub_3b6760
    Lifted::new(0x70003b68c0, tpl_96::<0x8, 0x19, 0x10, 0x20, 0x70003b75f0, 0x7000697150> as *const (), 0), // +0x3b68c0 sub_3b68c0
    Lifted::new(0x70003b6930, fn_70003b6930 as *const (), 0), // +0x3b6930 sub_3b6930
    Lifted::new(0x70003b6a40, fn_70003b6a40 as *const (), 0), // +0x3b6a40 sub_3b6a40
    Lifted::new(0x70003b6b10, fn_70003b6b10 as *const (), 2), // +0x3b6b10 sub_3b6b10
    Lifted::new(0x70003b6ca0, tpl_319::<0x70001c1260, 0x70001c13d0, 0x70001e4150, 0x7000697150, 0x70001c1200> as *const (), 2), // +0x3b6ca0 sub_3b6ca0
    Lifted::new(0x70003b6e10, fn_70003b6e10 as *const (), 0), // +0x3b6e10 sub_3b6e10
    Lifted::new(0x70003b6ed0, tpl_281::<0x8, 0x19, 0x10, 0x70001e4150, 0x70003b74f0, 0x7000697150> as *const (), 0), // +0x3b6ed0 sub_3b6ed0
    Lifted::new(0x70003b6f70, fn_70003b6f70 as *const (), 0), // +0x3b6f70 sub_3b6f70
    Lifted::new(0x70003b7090, fn_70003b7090 as *const (), 2), // +0x3b7090 sub_3b7090
    Lifted::new(0x70003b7220, tpl_320::<0x70006129c8, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3b7220 sub_3b7220
    Lifted::new(0x70003b72d0, tpl_320::<0x7000612a08, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3b72d0 sub_3b72d0
    Lifted::new(0x70003b7380, tpl_320::<0x7000612a18, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3b7380 sub_3b7380
    Lifted::new(0x70003b7430, fn_70003b7430 as *const (), 0), // +0x3b7430 sub_3b7430
    Lifted::new(0x70003b74f0, tpl_25::<0x7000694974, 0x70006994a0, 0x70005a4174, 0x70005a6c20, 0x70006994b0, 0x70006994c8, 0x70006994e0, 0x70006994f8, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3b74f0 sub_3b74f0
    Lifted::new(0x70003b7570, tpl_25::<0x7000694974, 0x7000699510, 0x70005a4174, 0x70005a6c10, 0x7000699518, 0x7000699540, 0x7000699568, 0x7000699590, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3b7570 sub_3b7570
    Lifted::new(0x70003b75f0, tpl_25::<0x7000694974, 0x700069b420, 0x70005a4174, 0x70005a6c00, 0x700069b430, 0x700069b448, 0x700069b460, 0x700069b478, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3b75f0 sub_3b75f0
    Lifted::new(0x70003b7670, tpl_25::<0x7000694974, 0x70006993d0, 0x70005a4174, 0x70005a6bf0, 0x70006993e0, 0x7000699410, 0x7000699440, 0x7000699470, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3b7670 sub_3b7670
    Lifted::new(0x70003b76f0, fn_70003b76f0 as *const (), 0), // +0x3b76f0 sub_3b76f0
    Lifted::new(0x70003b7770, fn_70003b7770 as *const (), 0), // +0x3b7770 sub_3b7770
    Lifted::new(0x70003b7910, fn_70003b7910 as *const (), 0), // +0x3b7910 sub_3b7910
    Lifted::new(0x70003b7ac0, fn_70003b7ac0 as *const (), 2), // +0x3b7ac0 sub_3b7ac0
    Lifted::new(0x70003b7c00, fn_70003b7c00 as *const (), 2), // +0x3b7c00 sub_3b7c00
    Lifted::new(0x70003b7d40, fn_70003b7d40 as *const (), 2), // +0x3b7d40 sub_3b7d40
    Lifted::new(0x70003b8030, fn_70003b8030 as *const (), 2), // +0x3b8030 sub_3b8030
    Lifted::new(0x70003b8340, fn_70003b8340 as *const (), 2), // +0x3b8340 sub_3b8340
    Lifted::new(0x70003b8740, tpl_246::<0x70003b9b60, 0x70003b62d0> as *const (), 2), // +0x3b8740 sub_3b8740
    Lifted::new(0x70003b8970, fn_70003b8970 as *const (), 2), // +0x3b8970 sub_3b8970
    Lifted::new(0x70003b8c30, fn_70003b8c30 as *const (), 2), // +0x3b8c30 sub_3b8c30
    Lifted::new(0x70003b8ee0, fn_70003b8ee0 as *const (), 0), // +0x3b8ee0 sub_3b8ee0
    Lifted::new(0x70003b9b00, tpl_74::<0x10, 0x2, 0x70003b9e10> as *const (), 0), // +0x3b9b00 sub_3b9b00
    Lifted::new(0x70003b9b30, tpl_74::<0x10, 0x2, 0x70003ba170> as *const (), 0), // +0x3b9b30 sub_3b9b30
    Lifted::new(0x70003b9b60, tpl_120::<0x70005a4d32, 0x700062e490, 0x70003b75f0, 0x7000697150, 0x70005f84c0, 0x70003b62d0> as *const (), 2), // +0x3b9b60 sub_3b9b60
    Lifted::new(0x70003b9e10, fn_70003b9e10 as *const (), 2), // +0x3b9e10 sub_3b9e10
    Lifted::new(0x70003ba170, tpl_121::<0x70005a4d32, 0x700062e490, 0x70003a5780, 0x7000697150, 0x70005f84c0, 0x70003ba880, 0x700010b4f0> as *const (), 2), // +0x3ba170 sub_3ba170
    Lifted::new(0x70003ba460, tpl_267::<0x70005a4d32, 0x700062e490, 0x70003a5880, 0x7000697150, 0x70005f84c0, 0x70003b6370, 0x700039fe90> as *const (), 2), // +0x3ba460 sub_3ba460
    Lifted::new(0x70003ba720, fn_70003ba720 as *const (), 2), // +0x3ba720 sub_3ba720
    Lifted::new(0x70003ba880, tpl_93::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x70001a86c0, 0x70003bac20> as *const (), 0), // +0x3ba880 sub_3ba880
    Lifted::new(0x70003ba950, fn_70003ba950 as *const (), 0), // +0x3ba950 sub_3ba950
    Lifted::new(0x70003ba9f0, fn_70003ba9f0 as *const (), 0), // +0x3ba9f0 sub_3ba9f0
    Lifted::new(0x70003baa10, tpl_328::<0x7000693c48, 0x70005a83d0, 0x7000693c40, 0x8> as *const (), 0), // +0x3baa10 sub_3baa10
    Lifted::new(0x70003baac0, fn_70003baac0 as *const (), 0), // +0x3baac0 sub_3baac0
    Lifted::new(0x70003bac20, tpl_132::<0x700004de60, 0x700010b4f0, 0x7000697150, 0x70003b9b30, 0x70003ba880> as *const (), 2), // +0x3bac20 sub_3bac20
    Lifted::new(0x70003baef0, fn_70003baef0 as *const (), 2), // +0x3baef0 sub_3baef0
    Lifted::new(0x70003bb250, fn_70003bb250 as *const (), 2), // +0x3bb250 sub_3bb250
    Lifted::new(0x70003bb4e0, fn_70003bb4e0 as *const (), 0), // +0x3bb4e0 sub_3bb4e0
    Lifted::new(0x70003bb5e4, tpl_185::<0x700039adb0> as *const (), 0), // +0x3bb5e4 sub_3bb5e4
    Lifted::new(0x70003bb5f0, tpl_204::<0xfffffffffffffffc, 0x70003a4c60> as *const (), 0), // +0x3bb5f0 sub_3bb5f0
    Lifted::new(0x70003bb600, tpl_98::<0x8, 0x70001e4150, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x3bb600 sub_3bb600
    Lifted::new(0x70003bb680, tpl_348::<0x8, 0x70003bb600> as *const (), 0), // +0x3bb680 sub_3bb680
    Lifted::new(0x70003bb6e0, tpl_182::<0x10, 0x7000612a34, 0x7000612a28, 0x7000231e90> as *const (), 0), // +0x3bb6e0 sub_3bb6e0
    Lifted::new(0x70003bb720, fn_70003bb720 as *const (), 0), // +0x3bb720 sub_3bb720
    Lifted::new(0x70003bb7b0, fn_70003bb7b0 as *const (), 2), // +0x3bb7b0 sub_3bb7b0
    Lifted::new(0x70003bba20, tpl_305::<0x88, 0x4, 0x70006134b0, 0xffffffffffffff78, 0xffffffffffffff80, 0xffffffffffffff74, 0x70003bf6f0, 0xffffffffffffff88, 0x70005a8150, 0x70005a8138> as *const (), 0), // +0x3bba20 sub_3bba20
    Lifted::new(0x70003bba80, fn_70003bba80 as *const (), 2), // +0x3bba80 sub_3bba80
    Lifted::new(0x70003bbce0, fn_70003bbce0 as *const (), 2), // +0x3bbce0 sub_3bbce0
    Lifted::new(0x70003bbf00, tpl_349::<0x7000601528, 0x70005a8258, 0x70005a80e8, 0x70006134b0, 0x70005a8278, 0x70006134b8, 0x7000613118, 0x7000613198, 0x7000613148, 0x7000613158, 0x7000613164, 0x70002445b0, 0x70003bf7f0, 0x70003bf940, 0x70005a4d14, 0x7000697150, 0x700062e610, 0x70003bf6f0, 0x70005a8150, 0x70005a8138> as *const (), 2), // +0x3bbf00 sub_3bbf00
    Lifted::new(0x70003bc100, fn_70003bc100 as *const (), 2), // +0x3bc100 sub_3bc100
    Lifted::new(0x70003bc2e0, tpl_349::<0x7000601528, 0x70005a8258, 0x70005a80e8, 0x70006134b0, 0x70005a8278, 0x70006134b8, 0x7000613118, 0x7000613120, 0x7000609610, 0x7000613188, 0x7000613190, 0x70002445b0, 0x70003bf7f0, 0x70003bf940, 0x70005a4d14, 0x7000697150, 0x700062e610, 0x70003bf6f0, 0x70005a8150, 0x70005a8138> as *const (), 2), // +0x3bc2e0 sub_3bc2e0
    Lifted::new(0x70003bc4e0, fn_70003bc4e0 as *const (), 2), // +0x3bc4e0 sub_3bc4e0
    Lifted::new(0x70003be290, fn_70003be290 as *const (), 2), // +0x3be290 sub_3be290
    Lifted::new(0x70003be880, fn_70003be880 as *const (), 0), // +0x3be880 sub_3be880
    Lifted::new(0x70003beb60, fn_70003beb60 as *const (), 0), // +0x3beb60 sub_3beb60
    Lifted::new(0x70003bedd0, fn_70003bedd0 as *const (), 0), // +0x3bedd0 sub_3bedd0
    Lifted::new(0x70003bef00, fn_70003bef00 as *const (), 0), // +0x3bef00 sub_3bef00
    Lifted::new(0x70003bf0e0, fn_70003bf0e0 as *const (), 0), // +0x3bf0e0 sub_3bf0e0
    Lifted::new(0x70003bf570, fn_70003bf570 as *const (), 0), // +0x3bf570 sub_3bf570
    Lifted::new(0x70003bf6f0, fn_70003bf6f0 as *const (), 0), // +0x3bf6f0 sub_3bf6f0
    Lifted::new(0x70003bf770, tpl_306::<0x70003bf6f0, 0x700048c2c0> as *const (), 0), // +0x3bf770 sub_3bf770
    Lifted::new(0x70003bf7b0, tpl_203::<0xffffffffffffff78, 0x70003bba20, 0x700048c2c0> as *const (), 0), // +0x3bf7b0 sub_3bf7b0
    Lifted::new(0x70003bf7f0, fn_70003bf7f0 as *const (), 2), // +0x3bf7f0 sub_3bf7f0
    Lifted::new(0x70003bf940, fn_70003bf940 as *const (), 0), // +0x3bf940 sub_3bf940
    Lifted::new(0x70003bfabc, tpl_204::<0xfffffffffffffffc, 0x70003bf7b0> as *const (), 0), // +0x3bfabc sub_3bfabc
    Lifted::new(0x70003bfad0, fn_70003bfad0 as *const (), 2), // +0x3bfad0 sub_3bfad0
    Lifted::new(0x70003bfc90, tpl_160::<0x700062e070> as *const (), 1), // +0x3bfc90 ??0ILocalizationManager@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003bfca0, tpl_161::<0xfffffffffffffff8, 0x700062e070, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3bfca0 sub_3bfca0
    Lifted::new(0x70003bfd30, fn_70003bfd30 as *const (), 0), // +0x3bfd30 sub_3bfd30
    Lifted::new(0x70003bfea0, fn_70003bfea0 as *const (), 2), // +0x3bfea0 sub_3bfea0
    Lifted::new(0x70003bff70, fn_70003bff70 as *const (), 0), // +0x3bff70 sub_3bff70
    Lifted::new(0x70003bffb0, fn_70003bffb0 as *const (), 0), // +0x3bffb0 sub_3bffb0
    Lifted::new(0x70003c0240, fn_70003c0240 as *const (), 0), // +0x3c0240 sub_3c0240
    Lifted::new(0x70003c0460, fn_70003c0460 as *const (), 0), // +0x3c0460 sub_3c0460
    Lifted::new(0x70003c04d0, fn_70003c04d0 as *const (), 0), // +0x3c04d0 sub_3c04d0
    Lifted::new(0x70003c05d0, fn_70003c05d0 as *const (), 0), // +0x3c05d0 sub_3c05d0
    Lifted::new(0x70003c0720, fn_70003c0720 as *const (), 0), // +0x3c0720 sub_3c0720
    Lifted::new(0x70003c07d0, fn_70003c07d0 as *const (), 0), // +0x3c07d0 sub_3c07d0
    Lifted::new(0x70003c0a80, fn_70003c0a80 as *const (), 0), // +0x3c0a80 sub_3c0a80
    Lifted::new(0x70003c0c80, fn_70003c0c80 as *const (), 0), // +0x3c0c80 sub_3c0c80
    Lifted::new(0x70003c0ef0, fn_70003c0ef0 as *const (), 0), // +0x3c0ef0 sub_3c0ef0
    Lifted::new(0x70003c0fc0, fn_70003c0fc0 as *const (), 0), // +0x3c0fc0 sub_3c0fc0
    Lifted::new(0x70003c12c0, fn_70003c12c0 as *const (), 2), // +0x3c12c0 sub_3c12c0
    Lifted::new(0x70003c1650, fn_70003c1650 as *const (), 0), // +0x3c1650 sub_3c1650
    Lifted::new(0x70003c1960, fn_70003c1960 as *const (), 2), // +0x3c1960 sub_3c1960
    Lifted::new(0x70003c1c50, fn_70003c1c50 as *const (), 2), // +0x3c1c50 sub_3c1c50
    Lifted::new(0x70003c1f40, fn_70003c1f40 as *const (), 0), // +0x3c1f40 sub_3c1f40
    Lifted::new(0x70003c2080, fn_70003c2080 as *const (), 0), // +0x3c2080 sub_3c2080
    Lifted::new(0x70003c2100, fn_70003c2100 as *const (), 0), // +0x3c2100 sub_3c2100
    Lifted::new(0x70003c21a0, fn_70003c21a0 as *const (), 0), // +0x3c21a0 sub_3c21a0
    Lifted::new(0x70003c22c0, fn_70003c22c0 as *const (), 0), // +0x3c22c0 sub_3c22c0
    Lifted::new(0x70003c26a0, fn_70003c26a0 as *const (), 0), // +0x3c26a0 sub_3c26a0
    Lifted::new(0x70003c2880, fn_70003c2880 as *const (), 0), // +0x3c2880 sub_3c2880
    Lifted::new(0x70003c2e70, fn_70003c2e70 as *const (), 0), // +0x3c2e70 sub_3c2e70
    Lifted::new(0x70003c2f90, fn_70003c2f90 as *const (), 2), // +0x3c2f90 sub_3c2f90
    Lifted::new(0x70003c3110, fn_70003c3110 as *const (), 2), // +0x3c3110 sub_3c3110
    Lifted::new(0x70003c3290, fn_70003c3290 as *const (), 0), // +0x3c3290 sub_3c3290
    Lifted::new(0x70003c3440, fn_70003c3440 as *const (), 0), // +0x3c3440 sub_3c3440
    Lifted::new(0x70003c3660, fn_70003c3660 as *const (), 0), // +0x3c3660 sub_3c3660
    Lifted::new(0x70003c3a80, fn_70003c3a80 as *const (), 0), // +0x3c3a80 sub_3c3a80
    Lifted::new(0x70003c3c90, fn_70003c3c90 as *const (), 0), // +0x3c3c90 sub_3c3c90
    Lifted::new(0x70003c3ec0, fn_70003c3ec0 as *const (), 0), // +0x3c3ec0 sub_3c3ec0
    Lifted::new(0x70003c3f40, fn_70003c3f40 as *const (), 0), // +0x3c3f40 sub_3c3f40
    Lifted::new(0x70003c4310, fn_70003c4310 as *const (), 0), // +0x3c4310 sub_3c4310
    Lifted::new(0x70003c4710, fn_70003c4710 as *const (), 0), // +0x3c4710 sub_3c4710
    Lifted::new(0x70003c4e10, fn_70003c4e10 as *const (), 0), // +0x3c4e10 sub_3c4e10
    Lifted::new(0x70003c4e30, fn_70003c4e30 as *const (), 0), // +0x3c4e30 sub_3c4e30
    Lifted::new(0x70003c4e60, fn_70003c4e60 as *const (), 0), // +0x3c4e60 sub_3c4e60
    Lifted::new(0x70003c4eb0, fn_70003c4eb0 as *const (), 0), // +0x3c4eb0 sub_3c4eb0
    Lifted::new(0x70003c5b80, fn_70003c5b80 as *const (), 0), // +0x3c5b80 sub_3c5b80
    Lifted::new(0x70003c5cc0, fn_70003c5cc0 as *const (), 0), // +0x3c5cc0 sub_3c5cc0
    Lifted::new(0x70003c5fb0, fn_70003c5fb0 as *const (), 0), // +0x3c5fb0 sub_3c5fb0
    Lifted::new(0x70003c6220, fn_70003c6220 as *const (), 0), // +0x3c6220 sub_3c6220
    Lifted::new(0x70003c6440, fn_70003c6440 as *const (), 0), // +0x3c6440 sub_3c6440
    Lifted::new(0x70003c64f0, fn_70003c64f0 as *const (), 0), // +0x3c64f0 sub_3c64f0
    Lifted::new(0x70003c6560, fn_70003c6560 as *const (), 0), // +0x3c6560 sub_3c6560
    Lifted::new(0x70003c6600, fn_70003c6600 as *const (), 0), // +0x3c6600 sub_3c6600
    Lifted::new(0x70003c6630, fn_70003c6630 as *const (), 2), // +0x3c6630 sub_3c6630
    Lifted::new(0x70003c6b90, fn_70003c6b90 as *const (), 0), // +0x3c6b90 sub_3c6b90
    Lifted::new(0x70003c6c50, fn_70003c6c50 as *const (), 2), // +0x3c6c50 sub_3c6c50
    Lifted::new(0x70003c6cc0, fn_70003c6cc0 as *const (), 0), // +0x3c6cc0 sub_3c6cc0
    Lifted::new(0x70003c6d80, fn_70003c6d80 as *const (), 0), // +0x3c6d80 sub_3c6d80
    Lifted::new(0x70003c6e20, fn_70003c6e20 as *const (), 0), // +0x3c6e20 sub_3c6e20
    Lifted::new(0x70003c6fc0, fn_70003c6fc0 as *const (), 0), // +0x3c6fc0 sub_3c6fc0
    Lifted::new(0x70003c70e0, fn_70003c70e0 as *const (), 2), // +0x3c70e0 sub_3c70e0
    Lifted::new(0x70003c7150, fn_70003c7150 as *const (), 0), // +0x3c7150 sub_3c7150
    Lifted::new(0x70003c7260, fn_70003c7260 as *const (), 1), // +0x3c7260 sub_3c7260
    Lifted::new(0x70003c7270, tpl_350::<0x20, 0x70005b52e4, 0x118, 0x70005f5738, 0x160, 0x30, 0x1c8, 0x1ad0, 0x7000322f10, 0x28> as *const (), 0), // +0x3c7270 sub_3c7270
    Lifted::new(0x70003c7320, fn_70003c7320 as *const (), 0), // +0x3c7320 sub_3c7320
    Lifted::alias(0x70003c7390, tpl_350::<0x20, 0x70005b52e4, 0x119, 0x70005f5748, 0x160, 0x30, 0x1c8, 0x1ad0, 0x7000322f10, 0x28> as *const (), 0, 0x3c73a0), // +0x3c7390 sub_3c7390 (thunk -> +0x3c73a0)
    Lifted::new(0x70003c73a0, tpl_350::<0x20, 0x70005b52e4, 0x119, 0x70005f5748, 0x160, 0x30, 0x1c8, 0x1ad0, 0x7000322f10, 0x28> as *const (), 0), // +0x3c73a0 sub_3c73a0
    Lifted::new(0x70003c7450, tpl_11::<0xc0> as *const (), 1), // +0x3c7450 sub_3c7450
    Lifted::new(0x70003c7460, tpl_143::<0xd0> as *const (), 1), // +0x3c7460 sub_3c7460
    Lifted::new(0x70003c7470, fn_70003c7470 as *const (), 1), // +0x3c7470 sub_3c7470
    Lifted::new(0x70003c74a0, tpl_324::<0xffffffffffffff4d> as *const (), 1), // +0x3c74a0 sub_3c74a0
    Lifted::new(0x70003c74b0, fn_70003c74b0 as *const (), 0), // +0x3c74b0 sub_3c74b0
    Lifted::new(0x70003c7580, fn_70003c7580 as *const (), 0), // +0x3c7580 sub_3c7580
    Lifted::new(0x70003c7630, fn_70003c7630 as *const (), 0), // +0x3c7630 sub_3c7630
    Lifted::new(0x70003c76e0, fn_70003c76e0 as *const (), 0), // +0x3c76e0 sub_3c76e0
    Lifted::new(0x70003c7a00, fn_70003c7a00 as *const (), 0), // +0x3c7a00 sub_3c7a00
    Lifted::new(0x70003c7b00, fn_70003c7b00 as *const (), 0), // +0x3c7b00 sub_3c7b00
    Lifted::new(0x70003c7c40, fn_70003c7c40 as *const (), 0), // +0x3c7c40 sub_3c7c40
    Lifted::new(0x70003c7c70, fn_70003c7c70 as *const (), 0), // +0x3c7c70 sub_3c7c70
    Lifted::new(0x70003c7f40, fn_70003c7f40 as *const (), 0), // +0x3c7f40 sub_3c7f40
    Lifted::new(0x70003c8240, fn_70003c8240 as *const (), 0), // +0x3c8240 sub_3c8240
    Lifted::new(0x70003c8330, fn_70003c8330 as *const (), 2), // +0x3c8330 sub_3c8330
    Lifted::new(0x70003c85e0, fn_70003c85e0 as *const (), 2), // +0x3c85e0 sub_3c85e0
    Lifted::new(0x70003c88b0, fn_70003c88b0 as *const (), 0), // +0x3c88b0 sub_3c88b0
    Lifted::new(0x70003c8ac0, fn_70003c8ac0 as *const (), 2), // +0x3c8ac0 sub_3c8ac0
    Lifted::new(0x70003c8c20, fn_70003c8c20 as *const (), 0), // +0x3c8c20 sub_3c8c20
    Lifted::new(0x70003c8cf0, fn_70003c8cf0 as *const (), 2), // +0x3c8cf0 sub_3c8cf0
    Lifted::new(0x70003c8fe0, fn_70003c8fe0 as *const (), 0), // +0x3c8fe0 sub_3c8fe0
    Lifted::new(0x70003c9220, fn_70003c9220 as *const (), 0), // +0x3c9220 sub_3c9220
    Lifted::new(0x70003c95b0, fn_70003c95b0 as *const (), 0), // +0x3c95b0 sub_3c95b0
    Lifted::new(0x70003c9650, fn_70003c9650 as *const (), 2), // +0x3c9650 sub_3c9650
    Lifted::new(0x70003c9930, fn_70003c9930 as *const (), 0), // +0x3c9930 sub_3c9930
    Lifted::new(0x70003c9ab0, fn_70003c9ab0 as *const (), 0), // +0x3c9ab0 sub_3c9ab0
    Lifted::new(0x70003c9b50, tpl_66::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70002a0c80> as *const (), 2), // +0x3c9b50 sub_3c9b50
    Lifted::new(0x70003c9be0, tpl_41::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70003c9c90> as *const (), 2), // +0x3c9be0 sub_3c9be0
    Lifted::new(0x70003c9c90, fn_70003c9c90 as *const (), 0), // +0x3c9c90 sub_3c9c90
    Lifted::new(0x70003c9d10, tpl_316::<0x18, 0x8, 0x70003ca150> as *const (), 0), // +0x3c9d10 sub_3c9d10
    Lifted::new(0x70003c9dc0, tpl_316::<0x18, 0x8, 0x70003ca450> as *const (), 0), // +0x3c9dc0 sub_3c9dc0
    Lifted::new(0x70003c9e70, tpl_351::<0x8, 0x18, 0x70003ca750> as *const (), 0), // +0x3c9e70 sub_3c9e70
    Lifted::new(0x70003c9f20, tpl_91::<0x7000007030, 0x70001a86c0, 0x70003caa60> as *const (), 0), // +0x3c9f20 sub_3c9f20
    Lifted::new(0x70003ca010, fn_70003ca010 as *const (), 0), // +0x3ca010 sub_3ca010
    Lifted::new(0x70003ca150, tpl_352::<0x70003b6ed0, 0x7000697150, 0x70003cb550, 0x70003cc390, 0x70003c9d10> as *const (), 2), // +0x3ca150 sub_3ca150
    Lifted::new(0x70003ca450, tpl_352::<0x70003b6e10, 0x7000697150, 0x70003cb200, 0x70003cc5f0, 0x70003c9dc0> as *const (), 2), // +0x3ca450 sub_3ca450
    Lifted::new(0x70003ca750, fn_70003ca750 as *const (), 2), // +0x3ca750 sub_3ca750
    Lifted::new(0x70003caa60, fn_70003caa60 as *const (), 2), // +0x3caa60 sub_3caa60
    Lifted::new(0x70003cad80, fn_70003cad80 as *const (), 0), // +0x3cad80 sub_3cad80
    Lifted::new(0x70003cb080, fn_70003cb080 as *const (), 0), // +0x3cb080 sub_3cb080
    Lifted::new(0x70003cb140, tpl_74::<0x10, 0x2, 0x70003cb200> as *const (), 0), // +0x3cb140 sub_3cb140
    Lifted::new(0x70003cb170, tpl_74::<0x10, 0x2, 0x70003cb550> as *const (), 0), // +0x3cb170 sub_3cb170
    Lifted::new(0x70003cb1a0, tpl_74::<0x10, 0x2, 0x70003cb8a0> as *const (), 0), // +0x3cb1a0 sub_3cb1a0
    Lifted::new(0x70003cb1d0, tpl_74::<0x10, 0x2, 0x70003cbb90> as *const (), 0), // +0x3cb1d0 sub_3cb1d0
    Lifted::new(0x70003cb200, tpl_353::<0x70005a4d32, 0x700062e490, 0x70003a5800, 0x7000697150, 0x70005f84c0, 0x70003cc5f0, 0x70003b6e10> as *const (), 2), // +0x3cb200 sub_3cb200
    Lifted::new(0x70003cb550, tpl_353::<0x70005a4d32, 0x700062e490, 0x70003a5700, 0x7000697150, 0x70005f84c0, 0x70003cc390, 0x70003b6ed0> as *const (), 2), // +0x3cb550 sub_3cb550
    Lifted::new(0x70003cb8a0, tpl_121::<0x70005a4d32, 0x700062e490, 0x70003b7570, 0x7000697150, 0x70005f84c0, 0x70003cc070, 0x70001e4150> as *const (), 2), // +0x3cb8a0 sub_3cb8a0
    Lifted::new(0x70003cbb90, fn_70003cbb90 as *const (), 2), // +0x3cbb90 sub_3cbb90
    Lifted::new(0x70003cbea0, tpl_244::<0x18, 0x8, 0x70003cc390> as *const (), 0), // +0x3cbea0 sub_3cbea0
    Lifted::new(0x70003cbf40, tpl_244::<0x18, 0x8, 0x70003cc5f0> as *const (), 0), // +0x3cbf40 sub_3cbf40
    Lifted::new(0x70003cbfe0, tpl_354::<0x18, 0x8, 0x70003cc850> as *const (), 0), // +0x3cbfe0 sub_3cbfe0
    Lifted::new(0x70003cc070, tpl_93::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x70001a86c0, 0x70003ccae0> as *const (), 0), // +0x3cc070 sub_3cc070
    Lifted::new(0x70003cc140, fn_70003cc140 as *const (), 0), // +0x3cc140 sub_3cc140
    Lifted::new(0x70003cc390, tpl_355::<0x70003b6ed0, 0x7000697150, 0x70003cb170, 0x70003cbea0> as *const (), 2), // +0x3cc390 sub_3cc390
    Lifted::new(0x70003cc5f0, tpl_355::<0x70003b6e10, 0x7000697150, 0x70003cb140, 0x70003cbf40> as *const (), 2), // +0x3cc5f0 sub_3cc5f0
    Lifted::new(0x70003cc850, fn_70003cc850 as *const (), 2), // +0x3cc850 sub_3cc850
    Lifted::new(0x70003ccae0, fn_70003ccae0 as *const (), 2), // +0x3ccae0 sub_3ccae0
    Lifted::new(0x70003ccdd0, fn_70003ccdd0 as *const (), 0), // +0x3ccdd0 sub_3ccdd0
    Lifted::new(0x70003cd060, fn_70003cd060 as *const (), 0), // +0x3cd060 sub_3cd060
    Lifted::new(0x70003cd640, fn_70003cd640 as *const (), 0), // +0x3cd640 sub_3cd640
    Lifted::new(0x70003cd7f0, fn_70003cd7f0 as *const (), 0), // +0x3cd7f0 sub_3cd7f0
    Lifted::new(0x70003cd950, fn_70003cd950 as *const (), 0), // +0x3cd950 sub_3cd950
    Lifted::new(0x70003cdae0, tpl_185::<0x70003bff70> as *const (), 0), // +0x3cdae0 sub_3cdae0
    Lifted::new(0x70003cdaf0, fn_70003cdaf0 as *const (), 0), // +0x3cdaf0 sub_3cdaf0
    Lifted::new(0x70003cdb20, fn_70003cdb20 as *const (), 0), // +0x3cdb20 sub_3cdb20
    Lifted::new(0x70003cdb90, fn_70003cdb90 as *const (), 2), // +0x3cdb90 sub_3cdb90
    Lifted::new(0x70003cde10, fn_70003cde10 as *const (), 0), // +0x3cde10 sub_3cde10
    Lifted::new(0x70003cdef0, fn_70003cdef0 as *const (), 0), // +0x3cdef0 sub_3cdef0
    Lifted::new(0x70003cdfe0, fn_70003cdfe0 as *const (), 0), // +0x3cdfe0 sub_3cdfe0
    Lifted::new(0x70003ce160, fn_70003ce160 as *const (), 0), // +0x3ce160 sub_3ce160
    Lifted::new(0x70003ce200, fn_70003ce200 as *const (), 0), // +0x3ce200 sub_3ce200
    Lifted::new(0x70003ce270, tpl_78::<0x70006139a0, 0x700048c2c0> as *const (), 0), // +0x3ce270 sub_3ce270
    Lifted::new(0x70003ce2a0, fn_70003ce2a0 as *const (), 2), // +0x3ce2a0 sub_3ce2a0
    Lifted::new(0x70003ce350, fn_70003ce350 as *const (), 0), // +0x3ce350 sub_3ce350
    Lifted::new(0x70003ce3b0, fn_70003ce3b0 as *const (), 0), // +0x3ce3b0 sub_3ce3b0
    Lifted::new(0x70003ce3f0, fn_70003ce3f0 as *const (), 0), // +0x3ce3f0 sub_3ce3f0
    Lifted::new(0x70003ce7d0, fn_70003ce7d0 as *const (), 0), // +0x3ce7d0 sub_3ce7d0
    Lifted::new(0x70003cea90, fn_70003cea90 as *const (), 0), // +0x3cea90 sub_3cea90
    Lifted::new(0x70003ceb60, fn_70003ceb60 as *const (), 0), // +0x3ceb60 sub_3ceb60
    Lifted::new(0x70003cf980, fn_70003cf980 as *const (), 0), // +0x3cf980 sub_3cf980
    Lifted::new(0x70003cfb60, fn_70003cfb60 as *const (), 2), // +0x3cfb60 sub_3cfb60
    Lifted::new(0x70003cfcf0, fn_70003cfcf0 as *const (), 0), // +0x3cfcf0 sub_3cfcf0
    Lifted::new(0x70003cfda0, fn_70003cfda0 as *const (), 0), // +0x3cfda0 sub_3cfda0
    Lifted::new(0x70003cfe10, fn_70003cfe10 as *const (), 0), // +0x3cfe10 sub_3cfe10
    Lifted::new(0x70003cfeb0, fn_70003cfeb0 as *const (), 2), // +0x3cfeb0 sub_3cfeb0
    Lifted::new(0x70003d0030, tpl_24::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70003d00d0> as *const (), 2), // +0x3d0030 sub_3d0030
    Lifted::new(0x70003d00d0, fn_70003d00d0 as *const (), 0), // +0x3d00d0 sub_3d00d0
    Lifted::new(0x70003d0170, fn_70003d0170 as *const (), 0), // +0x3d0170 sub_3d0170
    Lifted::new(0x70003d0270, fn_70003d0270 as *const (), 0), // +0x3d0270 sub_3d0270
    Lifted::new(0x70003d04c0, tpl_183::<0x7000613d90, 0x7000613d08, 0xd8, 0x700024baa0, 0x700048c2c0> as *const (), 0), // +0x3d04c0 sub_3d04c0
    Lifted::new(0x70003d0510, tpl_339::<0x30, 0x2bd8, 0x700024bc60> as *const (), 0), // +0x3d0510 sub_3d0510
    Lifted::new(0x70003d0520, tpl_206::<0x70003c7150, 0x30, 0x2bd8> as *const (), 0), // +0x3d0520 sub_3d0520
    Lifted::new(0x70003d0550, tpl_184::<0x7000697150, 0x25, 0x30, 0x70003bfea0, 0x20, 0x7000613d90, 0x7000613d08, 0x1c8, 0xd8, 0x70003c3f40, 0x7000602bd8, 0x70001c30e0> as *const (), 0), // +0x3d0550 sub_3d0550
    Lifted::new(0x70003d062c, tpl_185::<0x70003d04c0> as *const (), 0), // +0x3d062c sub_3d062c
    Lifted::new(0x70003d0640, fn_70003d0640 as *const (), 0), // +0x3d0640 sub_3d0640
    Lifted::new(0x70003d3460, fn_70003d3460 as *const (), 0), // +0x3d3460 sub_3d3460
    Lifted::new(0x70003d3590, fn_70003d3590 as *const (), 0), // +0x3d3590 sub_3d3590
    Lifted::new(0x70003d36d0, fn_70003d36d0 as *const (), 0), // +0x3d36d0 sub_3d36d0
    Lifted::new(0x70003d3750, fn_70003d3750 as *const (), 0), // +0x3d3750 sub_3d3750
    Lifted::new(0x70003d3760, fn_70003d3760 as *const (), 0), // +0x3d3760 sub_3d3760
    Lifted::new(0x70003d37b0, fn_70003d37b0 as *const (), 2), // +0x3d37b0 sub_3d37b0
    Lifted::new(0x70003d3990, tpl_199::<0x70003c7150, 0x70003d3a00> as *const (), 0), // +0x3d3990 sub_3d3990
    Lifted::new(0x70003d39b0, fn_70003d39b0 as *const (), 0), // +0x3d39b0 sub_3d39b0
    Lifted::new(0x70003d3a00, fn_70003d3a00 as *const (), 0), // +0x3d3a00 sub_3d3a00
    Lifted::new(0x70003d3a90, fn_70003d3a90 as *const (), 0), // +0x3d3a90 sub_3d3a90
    Lifted::new(0x70003d3c80, fn_70003d3c80 as *const (), 2), // +0x3d3c80 sub_3d3c80
    Lifted::new(0x70003d3f40, fn_70003d3f40 as *const (), 0), // +0x3d3f40 sub_3d3f40
    Lifted::new(0x70003d4000, fn_70003d4000 as *const (), 2), // +0x3d4000 sub_3d4000
    Lifted::new(0x70003d45f0, fn_70003d45f0 as *const (), 2), // +0x3d45f0 sub_3d45f0
    Lifted::new(0x70003d4a00, fn_70003d4a00 as *const (), 0), // +0x3d4a00 sub_3d4a00
    Lifted::new(0x70003d4c60, fn_70003d4c60 as *const (), 2), // +0x3d4c60 sub_3d4c60
    Lifted::new(0x70003d5500, fn_70003d5500 as *const (), 2), // +0x3d5500 sub_3d5500
    Lifted::new(0x70003d5940, fn_70003d5940 as *const (), 0), // +0x3d5940 sub_3d5940
    Lifted::new(0x70003d59d0, fn_70003d59d0 as *const (), 0), // +0x3d59d0 sub_3d59d0
    Lifted::new(0x70003d5a30, fn_70003d5a30 as *const (), 0), // +0x3d5a30 sub_3d5a30
    Lifted::new(0x70003d5a90, fn_70003d5a90 as *const (), 1), // +0x3d5a90 sub_3d5a90
    Lifted::new(0x70003d5ab0, fn_70003d5ab0 as *const (), 2), // +0x3d5ab0 sub_3d5ab0
    Lifted::new(0x70003d5b80, fn_70003d5b80 as *const (), 0), // +0x3d5b80 sub_3d5b80
    Lifted::new(0x70003d5be0, fn_70003d5be0 as *const (), 0), // +0x3d5be0 sub_3d5be0
    Lifted::new(0x70003d5c40, fn_70003d5c40 as *const (), 0), // +0x3d5c40 sub_3d5c40
    Lifted::new(0x70003d5ca0, fn_70003d5ca0 as *const (), 1), // +0x3d5ca0 sub_3d5ca0
    Lifted::new(0x70003d5cd0, fn_70003d5cd0 as *const (), 1), // +0x3d5cd0 sub_3d5cd0
    Lifted::new(0x70003d5cf0, fn_70003d5cf0 as *const (), 1), // +0x3d5cf0 sub_3d5cf0
    Lifted::new(0x70003d5d30, tpl_162::<0x7000614080, 0x70001f4230, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x3d5d30 sub_3d5d30
    Lifted::new(0x70003d5db0, tpl_320::<0x7000614090, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3d5db0 sub_3d5db0
    Lifted::new(0x70003d5e60, tpl_320::<0x70006140a0, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3d5e60 sub_3d5e60
    Lifted::new(0x70003d5f10, tpl_163::<0x70006140b0, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3d5f10 sub_3d5f10
    Lifted::new(0x70003d5fc0, tpl_163::<0x70006140c0, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3d5fc0 sub_3d5fc0
    Lifted::new(0x70003d6070, tpl_356::<0x70006140d0, 0xa4, 0x7000697150, 0x98, 0x10, 0x48, 0x70001f4230, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3d6070 sub_3d6070
    Lifted::new(0x70003d6120, tpl_356::<0x70006140e0, 0xa4, 0x7000697150, 0x98, 0x10, 0x48, 0x70001f4230, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3d6120 sub_3d6120
    Lifted::new(0x70003d61d0, tpl_357::<0x70006140f0, 0xa4, 0x7000697150, 0x98, 0x10, 0x48, 0x70001f4230, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3d61d0 sub_3d61d0
    Lifted::new(0x70003d6280, fn_70003d6280 as *const (), 0), // +0x3d6280 sub_3d6280
    Lifted::new(0x70003d62d0, fn_70003d62d0 as *const (), 0), // +0x3d62d0 sub_3d62d0
    Lifted::new(0x70003d6354, tpl_185::<0x70003d36d0> as *const (), 0), // +0x3d6354 sub_3d6354
    Lifted::new(0x70003d6360, fn_70003d6360 as *const (), 0), // +0x3d6360 sub_3d6360
    Lifted::new(0x70003d63d0, fn_70003d63d0 as *const (), 0), // +0x3d63d0 sub_3d63d0
    Lifted::new(0x70003d64d0, fn_70003d64d0 as *const (), 0), // +0x3d64d0 sub_3d64d0
    Lifted::new(0x70003d65e0, tpl_199::<0x700024bc60, 0x70003d6b10> as *const (), 0), // +0x3d65e0 sub_3d65e0
    Lifted::new(0x70003d6600, fn_70003d6600 as *const (), 0), // +0x3d6600 sub_3d6600
    Lifted::new(0x70003d69a0, fn_70003d69a0 as *const (), 0), // +0x3d69a0 sub_3d69a0
    Lifted::new(0x70003d69f0, fn_70003d69f0 as *const (), 0), // +0x3d69f0 sub_3d69f0
    Lifted::new(0x70003d6b10, fn_70003d6b10 as *const (), 0), // +0x3d6b10 sub_3d6b10
    Lifted::new(0x70003d6d80, fn_70003d6d80 as *const (), 0), // +0x3d6d80 sub_3d6d80
    Lifted::new(0x70003d7060, tpl_276::<0x8, 0x70003d70e0, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x3d7060 sub_3d7060
    Lifted::new(0x70003d70e0, tpl_277::<0x10, 0x70002a4c00, 0x1c, 0x7000697150, 0xc> as *const (), 0), // +0x3d70e0 sub_3d70e0
    Lifted::new(0x70003d7160, fn_70003d7160 as *const (), 2), // +0x3d7160 sub_3d7160
    Lifted::new(0x70003d7220, fn_70003d7220 as *const (), 2), // +0x3d7220 sub_3d7220
    Lifted::new(0x70003d72d0, fn_70003d72d0 as *const (), 0), // +0x3d72d0 sub_3d72d0
    Lifted::new(0x70003d734c, tpl_185::<0x70003d63d0> as *const (), 0), // +0x3d734c sub_3d734c
    Lifted::new(0x70003d7360, fn_70003d7360 as *const (), 2), // +0x3d7360 sub_3d7360
    Lifted::new(0x70003d77d0, tpl_160::<0x700062dc70> as *const (), 1), // +0x3d77d0 ??0ITextTransformationManager@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003d77e0, tpl_161::<0xfffffffffffffff8, 0x700062dc70, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3d77e0 sub_3d77e0
    Lifted::new(0x70003d7870, fn_70003d7870 as *const (), 0), // +0x3d7870 sub_3d7870
    Lifted::new(0x70003d7910, fn_70003d7910 as *const (), 0), // +0x3d7910 sub_3d7910
    Lifted::new(0x70003d7a00, fn_70003d7a00 as *const (), 0), // +0x3d7a00 sub_3d7a00
    Lifted::new(0x70003d7af0, fn_70003d7af0 as *const (), 0), // +0x3d7af0 sub_3d7af0
    Lifted::new(0x70003d7bf0, fn_70003d7bf0 as *const (), 0), // +0x3d7bf0 sub_3d7bf0
    Lifted::new(0x70003d7d40, fn_70003d7d40 as *const (), 0), // +0x3d7d40 sub_3d7d40
    Lifted::new(0x70003d7fe0, fn_70003d7fe0 as *const (), 0), // +0x3d7fe0 sub_3d7fe0
    Lifted::new(0x70003d8190, fn_70003d8190 as *const (), 0), // +0x3d8190 sub_3d8190
    Lifted::new(0x70003d8220, fn_70003d8220 as *const (), 0), // +0x3d8220 sub_3d8220
    Lifted::new(0x70003d8350, fn_70003d8350 as *const (), 0), // +0x3d8350 sub_3d8350
    Lifted::new(0x70003d8400, fn_70003d8400 as *const (), 0), // +0x3d8400 sub_3d8400
    Lifted::new(0x70003d8420, fn_70003d8420 as *const (), 0), // +0x3d8420 sub_3d8420
    Lifted::new(0x70003d8520, fn_70003d8520 as *const (), 0), // +0x3d8520 sub_3d8520
    Lifted::new(0x70003d8db0, fn_70003d8db0 as *const (), 0), // +0x3d8db0 sub_3d8db0
    Lifted::new(0x70003d9140, fn_70003d9140 as *const (), 0), // +0x3d9140 sub_3d9140
    Lifted::new(0x70003d9460, fn_70003d9460 as *const (), 0), // +0x3d9460 sub_3d9460
    Lifted::new(0x70003d96a0, fn_70003d96a0 as *const (), 0), // +0x3d96a0 sub_3d96a0
    Lifted::new(0x70003d96e0, tpl_94::<0x70006041a8, 0x700048c2c0> as *const (), 0), // +0x3d96e0 sub_3d96e0
    Lifted::new(0x70003d9710, fn_70003d9710 as *const (), 0), // +0x3d9710 sub_3d9710
    Lifted::new(0x70003d9a90, fn_70003d9a90 as *const (), 0), // +0x3d9a90 sub_3d9a90
    Lifted::new(0x70003d9dc0, fn_70003d9dc0 as *const (), 0), // +0x3d9dc0 sub_3d9dc0
    Lifted::new(0x70003d9f30, fn_70003d9f30 as *const (), 0), // +0x3d9f30 sub_3d9f30
    Lifted::new(0x70003da2d0, fn_70003da2d0 as *const (), 2), // +0x3da2d0 sub_3da2d0
    Lifted::new(0x70003dac80, fn_70003dac80 as *const (), 0), // +0x3dac80 sub_3dac80
    Lifted::new(0x70003dae40, fn_70003dae40 as *const (), 0), // +0x3dae40 sub_3dae40
    Lifted::new(0x70003db0e0, fn_70003db0e0 as *const (), 0), // +0x3db0e0 sub_3db0e0
    Lifted::new(0x70003db350, fn_70003db350 as *const (), 0), // +0x3db350 sub_3db350
    Lifted::new(0x70003db460, fn_70003db460 as *const (), 0), // +0x3db460 sub_3db460
    Lifted::new(0x70003db500, fn_70003db500 as *const (), 0), // +0x3db500 sub_3db500
    Lifted::new(0x70003db540, fn_70003db540 as *const (), 0), // +0x3db540 sub_3db540
    Lifted::new(0x70003db660, fn_70003db660 as *const (), 0), // +0x3db660 sub_3db660
    Lifted::new(0x70003db6e0, fn_70003db6e0 as *const (), 0), // +0x3db6e0 sub_3db6e0
    Lifted::new(0x70003db8f0, tpl_177::<0x30, 0x700039fda0> as *const (), 0), // +0x3db8f0 sub_3db8f0
    Lifted::new(0x70003db900, fn_70003db900 as *const (), 0), // +0x3db900 sub_3db900
    Lifted::new(0x70003dbb40, tpl_280::<0x30, 0x70006974b8, 0x70005fe558, 0x70005f84a0, 0x7000130980, 0x3010> as *const (), 0), // +0x3dbb40 sub_3dbb40
    Lifted::new(0x70003dbba0, fn_70003dbba0 as *const (), 2), // +0x3dbba0 sub_3dbba0
    Lifted::new(0x70003dbcc0, fn_70003dbcc0 as *const (), 0), // +0x3dbcc0 sub_3dbcc0
    Lifted::new(0x70003dbe40, fn_70003dbe40 as *const (), 0), // +0x3dbe40 sub_3dbe40
    Lifted::new(0x70003dbea0, fn_70003dbea0 as *const (), 0), // +0x3dbea0 sub_3dbea0
    Lifted::new(0x70003dc1b0, tpl_358::<0x30, 0x1c8, 0x90, 0x7000496870, 0xa0, 0x18, 0x1a4, 0xc4> as *const (), 2), // +0x3dc1b0 sub_3dc1b0
    Lifted::new(0x70003dc220, tpl_358::<0x30, 0x1c8, 0x90, 0x7000496870, 0xa0, 0x18, 0x1a0, 0xc0> as *const (), 2), // +0x3dc220 sub_3dc220
    Lifted::new(0x70003dc290, tpl_359::<0xa0, 0xc4, 0x30, 0x70003f7d00, 0x18, 0x10, 0x1a4, 0x1a8, 0x70003a0c10, 0x700010b4f0> as *const (), 2), // +0x3dc290 sub_3dc290
    Lifted::new(0x70003dc360, tpl_359::<0xa0, 0xc0, 0x30, 0x70003f7d00, 0x18, 0x10, 0x1a0, 0x1a9, 0x70003a0c10, 0x700010b4f0> as *const (), 2), // +0x3dc360 sub_3dc360
    Lifted::new(0x70003dc430, fn_70003dc430 as *const (), 0), // +0x3dc430 sub_3dc430
    Lifted::new(0x70003dc5b0, fn_70003dc5b0 as *const (), 0), // +0x3dc5b0 sub_3dc5b0
    Lifted::new(0x70003dc7f0, fn_70003dc7f0 as *const (), 0), // +0x3dc7f0 sub_3dc7f0
    Lifted::new(0x70003dc890, fn_70003dc890 as *const (), 0), // +0x3dc890 sub_3dc890
    Lifted::new(0x70003dc920, fn_70003dc920 as *const (), 0), // +0x3dc920 sub_3dc920
    Lifted::new(0x70003dcad0, fn_70003dcad0 as *const (), 0), // +0x3dcad0 sub_3dcad0
    Lifted::new(0x70003dcca0, tpl_360::<0x30, 0x1c8, 0x90, 0x7000496870, 0xa0, 0x18, 0xf0> as *const (), 0), // +0x3dcca0 sub_3dcca0
    Lifted::new(0x70003dcd00, tpl_360::<0x30, 0x1c8, 0x90, 0x7000496870, 0xa0, 0x18, 0xf4> as *const (), 0), // +0x3dcd00 sub_3dcd00
    Lifted::new(0x70003dcd60, fn_70003dcd60 as *const (), 2), // +0x3dcd60 sub_3dcd60
    Lifted::new(0x70003dcfd0, fn_70003dcfd0 as *const (), 2), // +0x3dcfd0 sub_3dcfd0
    Lifted::new(0x70003dd130, fn_70003dd130 as *const (), 2), // +0x3dd130 sub_3dd130
    Lifted::new(0x70003dd2b0, fn_70003dd2b0 as *const (), 0), // +0x3dd2b0 sub_3dd2b0
    Lifted::new(0x70003dd320, fn_70003dd320 as *const (), 0), // +0x3dd320 sub_3dd320
    Lifted::new(0x70003dd750, fn_70003dd750 as *const (), 0), // +0x3dd750 sub_3dd750
    Lifted::new(0x70003dd8c0, fn_70003dd8c0 as *const (), 2), // +0x3dd8c0 sub_3dd8c0
    Lifted::new(0x70003dda90, fn_70003dda90 as *const (), 0), // +0x3dda90 sub_3dda90
    Lifted::new(0x70003ddba0, fn_70003ddba0 as *const (), 0), // +0x3ddba0 sub_3ddba0
    Lifted::new(0x70003ddc40, fn_70003ddc40 as *const (), 0), // +0x3ddc40 sub_3ddc40
    Lifted::new(0x70003ddce0, tpl_39::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70003b76f0> as *const (), 2), // +0x3ddce0 sub_3ddce0
    Lifted::new(0x70003ddd70, fn_70003ddd70 as *const (), 0), // +0x3ddd70 sub_3ddd70
    Lifted::new(0x70003ddea0, fn_70003ddea0 as *const (), 0), // +0x3ddea0 sub_3ddea0
    Lifted::new(0x70003ddfd0, fn_70003ddfd0 as *const (), 0), // +0x3ddfd0 sub_3ddfd0
    Lifted::new(0x70003de060, fn_70003de060 as *const (), 2), // +0x3de060 sub_3de060
    Lifted::new(0x70003de150, fn_70003de150 as *const (), 0), // +0x3de150 sub_3de150
    Lifted::new(0x70003de160, fn_70003de160 as *const (), 0), // +0x3de160 sub_3de160
    Lifted::new(0x70003de1b0, fn_70003de1b0 as *const (), 0), // +0x3de1b0 sub_3de1b0
    Lifted::new(0x70003de2b0, fn_70003de2b0 as *const (), 0), // +0x3de2b0 sub_3de2b0
    Lifted::new(0x70003de360, fn_70003de360 as *const (), 0), // +0x3de360 sub_3de360
    Lifted::new(0x70003de400, tpl_219::<0x7000614d60, 0x70001f4230, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x3de400 sub_3de400
    Lifted::new(0x70003de480, tpl_219::<0x7000614d70, 0x70001f4230, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x3de480 sub_3de480
    Lifted::new(0x70003de500, tpl_320::<0x7000614d80, 0x48, 0xffffffffffffffff, 0x7000697150, 0x10, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3de500 sub_3de500
    Lifted::new(0x70003de5b0, fn_70003de5b0 as *const (), 0), // +0x3de5b0 sub_3de5b0
    Lifted::new(0x70003de640, tpl_357::<0x7000614da0, 0xac, 0x7000697150, 0xa0, 0x10, 0x48, 0x70001f4230, 0x70005fe088, 0x3c, 0x30, 0x700048c2c0> as *const (), 0), // +0x3de640 sub_3de640
    Lifted::new(0x70003de6f0, fn_70003de6f0 as *const (), 0), // +0x3de6f0 sub_3de6f0
    Lifted::new(0x70003de780, fn_70003de780 as *const (), 0), // +0x3de780 sub_3de780
    Lifted::new(0x70003dea80, fn_70003dea80 as *const (), 0), // +0x3dea80 sub_3dea80
    Lifted::new(0x70003deb50, fn_70003deb50 as *const (), 2), // +0x3deb50 sub_3deb50
    Lifted::new(0x70003dedd0, fn_70003dedd0 as *const (), 0), // +0x3dedd0 sub_3dedd0
    Lifted::new(0x70003defa0, fn_70003defa0 as *const (), 0), // +0x3defa0 sub_3defa0
    Lifted::new(0x70003dfe80, fn_70003dfe80 as *const (), 0), // +0x3dfe80 sub_3dfe80
    Lifted::new(0x70003e0ad0, tpl_361::<0x7000697150, 0x700037b3d0> as *const (), 0), // +0x3e0ad0 sub_3e0ad0
    Lifted::new(0x70003e0b20, fn_70003e0b20 as *const (), 0), // +0x3e0b20 sub_3e0b20
    Lifted::new(0x70003e0ba0, tpl_361::<0x7000697150, 0x7000381ff0> as *const (), 0), // +0x3e0ba0 sub_3e0ba0
    Lifted::new(0x70003e0bf0, tpl_361::<0x7000697150, 0x70003821a0> as *const (), 0), // +0x3e0bf0 sub_3e0bf0
    Lifted::new(0x70003e0c40, tpl_361::<0x7000697150, 0x7000382490> as *const (), 0), // +0x3e0c40 sub_3e0c40
    Lifted::new(0x70003e0c90, tpl_362::<0x7000697150, 0x70003822e0> as *const (), 0), // +0x3e0c90 sub_3e0c90
    Lifted::new(0x70003e0ce0, tpl_361::<0x7000697150, 0x70003825d0> as *const (), 0), // +0x3e0ce0 sub_3e0ce0
    Lifted::new(0x70003e0d30, tpl_361::<0x7000697150, 0x7000382d10> as *const (), 0), // +0x3e0d30 sub_3e0d30
    Lifted::new(0x70003e0d80, tpl_361::<0x7000697150, 0x7000382e80> as *const (), 0), // +0x3e0d80 sub_3e0d80
    Lifted::new(0x70003e0dd0, tpl_362::<0x7000697150, 0x7000382ff0> as *const (), 0), // +0x3e0dd0 sub_3e0dd0
    Lifted::new(0x70003e0e20, tpl_361::<0x7000697150, 0x7000383130> as *const (), 0), // +0x3e0e20 sub_3e0e20
    Lifted::new(0x70003e0e70, fn_70003e0e70 as *const (), 0), // +0x3e0e70 sub_3e0e70
    Lifted::new(0x70003e0ef0, fn_70003e0ef0 as *const (), 0), // +0x3e0ef0 sub_3e0ef0
    Lifted::new(0x70003e0f70, tpl_361::<0x7000697150, 0x7000384b60> as *const (), 0), // +0x3e0f70 sub_3e0f70
    Lifted::new(0x70003e0fc0, fn_70003e0fc0 as *const (), 0), // +0x3e0fc0 sub_3e0fc0
    Lifted::new(0x70003e1010, tpl_361::<0x7000697150, 0x700029b630> as *const (), 0), // +0x3e1010 sub_3e1010
    Lifted::new(0x70003e1060, tpl_361::<0x7000697150, 0x700029bcd0> as *const (), 0), // +0x3e1060 sub_3e1060
    Lifted::new(0x70003e10b0, fn_70003e10b0 as *const (), 0), // +0x3e10b0 sub_3e10b0
    Lifted::new(0x70003e1110, fn_70003e1110 as *const (), 0), // +0x3e1110 sub_3e1110
    Lifted::new(0x70003e11a8, tpl_185::<0x70003dea80> as *const (), 0), // +0x3e11a8 sub_3e11a8
    Lifted::new(0x70003e11c0, tpl_183::<0x70006151c8, 0x7000615390, 0xd8, 0x700024baa0, 0x700048c2c0> as *const (), 0), // +0x3e11c0 sub_3e11c0
    Lifted::new(0x70003e1210, fn_70003e1210 as *const (), 0), // +0x3e1210 sub_3e1210
    Lifted::new(0x70003e13b0, tpl_184::<0x7000697150, 0x25, 0x30, 0x70003bfea0, 0x20, 0x70006151c8, 0x7000615390, 0x1c8, 0xd8, 0x70003c3f40, 0x7000602bd8, 0x70001c30e0> as *const (), 0), // +0x3e13b0 sub_3e13b0
    Lifted::new(0x70003e148c, tpl_185::<0x70003e11c0> as *const (), 0), // +0x3e148c sub_3e148c
    Lifted::new(0x70003e14a0, fn_70003e14a0 as *const (), 0), // +0x3e14a0 sub_3e14a0
    Lifted::new(0x70003e1510, fn_70003e1510 as *const (), 0), // +0x3e1510 sub_3e1510
    Lifted::new(0x70003e1590, fn_70003e1590 as *const (), 0), // +0x3e1590 sub_3e1590
    Lifted::new(0x70003e1710, tpl_199::<0x700024bc60, 0x70003e1dd0> as *const (), 0), // +0x3e1710 sub_3e1710
    Lifted::new(0x70003e1730, fn_70003e1730 as *const (), 0), // +0x3e1730 sub_3e1730
    Lifted::new(0x70003e18b0, fn_70003e18b0 as *const (), 0), // +0x3e18b0 sub_3e18b0
    Lifted::new(0x70003e1a80, fn_70003e1a80 as *const (), 0), // +0x3e1a80 sub_3e1a80
    Lifted::new(0x70003e1cb0, fn_70003e1cb0 as *const (), 0), // +0x3e1cb0 sub_3e1cb0
    Lifted::new(0x70003e1dd0, fn_70003e1dd0 as *const (), 0), // +0x3e1dd0 sub_3e1dd0
    Lifted::new(0x70003e2040, fn_70003e2040 as *const (), 0), // +0x3e2040 sub_3e2040
    Lifted::new(0x70003e2104, tpl_185::<0x70003e1510> as *const (), 0), // +0x3e2104 sub_3e2104
    Lifted::new(0x70003e2110, tpl_183::<0x70006156f0, 0x7000615668, 0xd8, 0x700024baa0, 0x700048c2c0> as *const (), 0), // +0x3e2110 sub_3e2110
    Lifted::new(0x70003e2160, tpl_184::<0x7000697150, 0x25, 0x30, 0x70003bfea0, 0x20, 0x70006156f0, 0x7000615668, 0x1c8, 0xd8, 0x70003c3f40, 0x7000602bd8, 0x70001c30e0> as *const (), 0), // +0x3e2160 sub_3e2160
    Lifted::new(0x70003e223c, tpl_185::<0x70003e2110> as *const (), 0), // +0x3e223c sub_3e223c
    Lifted::new(0x70003e2250, fn_70003e2250 as *const (), 0), // +0x3e2250 sub_3e2250
    Lifted::new(0x70003e23d0, fn_70003e23d0 as *const (), 0), // +0x3e23d0 sub_3e23d0
    Lifted::new(0x70003e2430, fn_70003e2430 as *const (), 0), // +0x3e2430 sub_3e2430
    Lifted::new(0x70003e2480, fn_70003e2480 as *const (), 0), // +0x3e2480 sub_3e2480
    Lifted::new(0x70003e25a0, tpl_199::<0x700024bc60, 0x70003e2480> as *const (), 0), // +0x3e25a0 sub_3e25a0
    Lifted::new(0x70003e25c0, fn_70003e25c0 as *const (), 0), // +0x3e25c0 sub_3e25c0
    Lifted::new(0x70003e26bc, tpl_185::<0x70003e23d0> as *const (), 0), // +0x3e26bc sub_3e26bc
    Lifted::new(0x70003e26d0, fn_70003e26d0 as *const (), 0), // +0x3e26d0 sub_3e26d0
    Lifted::new(0x70003e2750, fn_70003e2750 as *const (), 0), // +0x3e2750 sub_3e2750
    Lifted::new(0x70003e2870, fn_70003e2870 as *const (), 0), // +0x3e2870 sub_3e2870
    Lifted::new(0x70003e28b0, fn_70003e28b0 as *const (), 0), // +0x3e28b0 sub_3e28b0
    Lifted::new(0x70003e2970, fn_70003e2970 as *const (), 0), // +0x3e2970 sub_3e2970
    Lifted::new(0x70003e2c00, fn_70003e2c00 as *const (), 2), // +0x3e2c00 sub_3e2c00
    Lifted::alias(0x70003e31c0, fn_700023a6b0 as *const (), 0, 0x23a6b0), // +0x3e31c0 sub_3e31c0 (thunk -> +0x23a6b0)
    Lifted::new(0x70003e31d0, fn_70003e31d0 as *const (), 0), // +0x3e31d0 sub_3e31d0
    Lifted::new(0x70003e32d0, tpl_183::<0x7000615d08, 0x7000615ed0, 0xd8, 0x700024baa0, 0x700048c2c0> as *const (), 0), // +0x3e32d0 sub_3e32d0
    Lifted::new(0x70003e3320, fn_70003e3320 as *const (), 0), // +0x3e3320 sub_3e3320
    Lifted::new(0x70003e33fc, tpl_185::<0x70003e32d0> as *const (), 0), // +0x3e33fc sub_3e33fc
    Lifted::new(0x70003e3410, fn_70003e3410 as *const (), 0), // +0x3e3410 sub_3e3410
    Lifted::new(0x70003e35c0, fn_70003e35c0 as *const (), 0), // +0x3e35c0 sub_3e35c0
    Lifted::new(0x70003e35d0, tpl_207::<0x8, 0x10> as *const (), 0), // +0x3e35d0 sub_3e35d0
    Lifted::new(0x70003e35e0, fn_70003e35e0 as *const (), 0), // +0x3e35e0 sub_3e35e0
    Lifted::new(0x70003e3620, tpl_322::<0x7000616188, 0x70005a8300, 0x700048c2c0> as *const (), 0), // +0x3e3620 sub_3e3620
    Lifted::new(0x70003e3660, fn_70003e3660 as *const (), 0), // +0x3e3660 sub_3e3660
    Lifted::new(0x70003e36a0, tpl_322::<0x7000616168, 0x70005a8300, 0x700048c2c0> as *const (), 0), // +0x3e36a0 sub_3e36a0
    Lifted::new(0x70003e36e0, fn_70003e36e0 as *const (), 2), // +0x3e36e0 sub_3e36e0
    Lifted::new(0x70003e3900, fn_70003e3900 as *const (), 0), // +0x3e3900 sub_3e3900
    Lifted::new(0x70003e3c00, fn_70003e3c00 as *const (), 1), // +0x3e3c00 sub_3e3c00
    Lifted::new(0x70003e3d50, fn_70003e3d50 as *const (), 2), // +0x3e3d50 sub_3e3d50
    Lifted::new(0x70003e41a0, fn_70003e41a0 as *const (), 2), // +0x3e41a0 sub_3e41a0
    Lifted::new(0x70003e44d0, fn_70003e44d0 as *const (), 2), // +0x3e44d0 sub_3e44d0
    Lifted::new(0x70003e4760, fn_70003e4760 as *const (), 2), // +0x3e4760 sub_3e4760
    Lifted::new(0x70003e4b40, fn_70003e4b40 as *const (), 2), // +0x3e4b40 sub_3e4b40
    Lifted::new(0x70003e4be0, fn_70003e4be0 as *const (), 2), // +0x3e4be0 sub_3e4be0
    Lifted::new(0x70003e4e90, fn_70003e4e90 as *const (), 0), // +0x3e4e90 sub_3e4e90
    Lifted::new(0x70003e5020, fn_70003e5020 as *const (), 0), // +0x3e5020 sub_3e5020
    Lifted::new(0x70003e5260, tpl_34::<0x8, 0x70003e54e0> as *const (), 0), // +0x3e5260 sub_3e5260
    Lifted::new(0x70003e52e0, fn_70003e52e0 as *const (), 0), // +0x3e52e0 sub_3e52e0
    Lifted::new(0x70003e53a0, fn_70003e53a0 as *const (), 0), // +0x3e53a0 sub_3e53a0
    Lifted::new(0x70003e5470, fn_70003e5470 as *const (), 0), // +0x3e5470 sub_3e5470
    Lifted::new(0x70003e54e0, tpl_285::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x70003e57e0> as *const (), 2), // +0x3e54e0 sub_3e54e0
    Lifted::new(0x70003e55d0, fn_70003e55d0 as *const (), 0), // +0x3e55d0 sub_3e55d0
    Lifted::new(0x70003e5660, tpl_25::<0x7000694974, 0x700069b490, 0x70005a4174, 0x70005a6d50, 0x700069b4a0, 0x700069b4d8, 0x700069b510, 0x700069b548, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3e5660 sub_3e5660
    Lifted::new(0x70003e56e0, tpl_25::<0x7000694974, 0x700069b580, 0x70005a4174, 0x70005a6d40, 0x700069b590, 0x700069b5b8, 0x700069b5e0, 0x700069b608, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3e56e0 sub_3e56e0
    Lifted::new(0x70003e5760, tpl_25::<0x7000694974, 0x700069b630, 0x70005a4174, 0x70005a6d30, 0x700069b640, 0x700069b660, 0x700069b680, 0x700069b6a0, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3e5760 sub_3e5760
    Lifted::new(0x70003e57e0, fn_70003e57e0 as *const (), 0), // +0x3e57e0 sub_3e57e0
    Lifted::new(0x70003e5890, fn_70003e5890 as *const (), 2), // +0x3e5890 sub_3e5890
    Lifted::new(0x70003e59a0, tpl_363::<0x8, 0x38, 0x70003e6300> as *const (), 0), // +0x3e59a0 sub_3e59a0
    Lifted::new(0x70003e5a50, fn_70003e5a50 as *const (), 0), // +0x3e5a50 sub_3e5a50
    Lifted::new(0x70003e5be0, tpl_315::<0x70005a8240, 0x7000616058, 0x70002445b0, 0x7000616068, 0x70005a8230> as *const (), 0), // +0x3e5be0 sub_3e5be0
    Lifted::new(0x70003e5d30, fn_70003e5d30 as *const (), 0), // +0x3e5d30 sub_3e5d30
    Lifted::new(0x70003e5e00, tpl_363::<0x8, 0x38, 0x70003e6920> as *const (), 0), // +0x3e5e00 sub_3e5e00
    Lifted::new(0x70003e5eb0, fn_70003e5eb0 as *const (), 0), // +0x3e5eb0 sub_3e5eb0
    Lifted::new(0x70003e5f70, tpl_148::<0x70003e56e0, 0x8, 0x10, 0x20, 0x18, 0x1c, 0x19> as *const (), 2), // +0x3e5f70 sub_3e5f70
    Lifted::new(0x70003e6010, tpl_210::<0x70003e5020> as *const (), 0), // +0x3e6010 sub_3e6010
    Lifted::new(0x70003e6030, fn_70003e6030 as *const (), 0), // +0x3e6030 sub_3e6030
    Lifted::new(0x70003e6050, fn_70003e6050 as *const (), 0), // +0x3e6050 sub_3e6050
    Lifted::new(0x70003e6070, tpl_162::<0x7000616100, 0x70003a4f30, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x3e6070 sub_3e6070
    Lifted::new(0x70003e60f0, fn_70003e60f0 as *const (), 0), // +0x3e60f0 sub_3e60f0
    Lifted::new(0x70003e6150, fn_70003e6150 as *const (), 0), // +0x3e6150 sub_3e6150
    Lifted::new(0x70003e61b0, fn_70003e61b0 as *const (), 0), // +0x3e61b0 sub_3e61b0
    Lifted::new(0x70003e6300, fn_70003e6300 as *const (), 2), // +0x3e6300 sub_3e6300
    Lifted::new(0x70003e6640, fn_70003e6640 as *const (), 2), // +0x3e6640 sub_3e6640
    Lifted::new(0x70003e6920, fn_70003e6920 as *const (), 2), // +0x3e6920 sub_3e6920
    Lifted::new(0x70003e6c60, fn_70003e6c60 as *const (), 2), // +0x3e6c60 sub_3e6c60
    Lifted::new(0x70003e6f30, fn_70003e6f30 as *const (), 2), // +0x3e6f30 sub_3e6f30
    Lifted::new(0x70003e7180, tpl_74::<0x10, 0x2, 0x70003e7540> as *const (), 0), // +0x3e7180 sub_3e7180
    Lifted::new(0x70003e71b0, fn_70003e71b0 as *const (), 1), // +0x3e71b0 sub_3e71b0
    Lifted::new(0x70003e7280, fn_70003e7280 as *const (), 2), // +0x3e7280 sub_3e7280
    Lifted::new(0x70003e7540, fn_70003e7540 as *const (), 2), // +0x3e7540 sub_3e7540
    Lifted::new(0x70003e7880, fn_70003e7880 as *const (), 2), // +0x3e7880 sub_3e7880
    Lifted::new(0x70003e7b20, fn_70003e7b20 as *const (), 0), // +0x3e7b20 sub_3e7b20
    Lifted::new(0x70003e7bb0, fn_70003e7bb0 as *const (), 2), // +0x3e7bb0 sub_3e7bb0
    Lifted::new(0x70003e7e60, fn_70003e7e60 as *const (), 0), // +0x3e7e60 sub_3e7e60
    Lifted::new(0x70003e7f20, tpl_212::<0x70003e4760> as *const (), 0), // +0x3e7f20 sub_3e7f20
    Lifted::new(0x70003e7f30, fn_70003e7f30 as *const (), 2), // +0x3e7f30 sub_3e7f30
    Lifted::new(0x70003e8050, tpl_149::<0x70003e56e0, 0x70003e55d0, 0x7000697150> as *const (), 2), // +0x3e8050 sub_3e8050
    Lifted::new(0x70003e81d0, fn_70003e81d0 as *const (), 2), // +0x3e81d0 sub_3e81d0
    Lifted::new(0x70003e8450, fn_70003e8450 as *const (), 2), // +0x3e8450 sub_3e8450
    Lifted::new(0x70003e8760, fn_70003e8760 as *const (), 2), // +0x3e8760 sub_3e8760
    Lifted::new(0x70003e8ac0, fn_70003e8ac0 as *const (), 2), // +0x3e8ac0 sub_3e8ac0
    Lifted::new(0x70003e8e60, fn_70003e8e60 as *const (), 2), // +0x3e8e60 sub_3e8e60
    Lifted::new(0x70003e9c30, fn_70003e9c30 as *const (), 2), // +0x3e9c30 sub_3e9c30
    Lifted::new(0x70003ea0a0, fn_70003ea0a0 as *const (), 2), // +0x3ea0a0 sub_3ea0a0
    Lifted::new(0x70003ea6c0, fn_70003ea6c0 as *const (), 2), // +0x3ea6c0 sub_3ea6c0
    Lifted::new(0x70003ea930, fn_70003ea930 as *const (), 2), // +0x3ea930 sub_3ea930
    Lifted::new(0x70003eaa00, fn_70003eaa00 as *const (), 2), // +0x3eaa00 sub_3eaa00
    Lifted::new(0x70003eb550, fn_70003eb550 as *const (), 2), // +0x3eb550 sub_3eb550
    Lifted::new(0x70003eb820, fn_70003eb820 as *const (), 2), // +0x3eb820 sub_3eb820
    Lifted::new(0x70003ebea0, fn_70003ebea0 as *const (), 2), // +0x3ebea0 sub_3ebea0
    Lifted::new(0x70003ec200, fn_70003ec200 as *const (), 2), // +0x3ec200 sub_3ec200
    Lifted::new(0x70003ee7d0, fn_70003ee7d0 as *const (), 2), // +0x3ee7d0 sub_3ee7d0
    Lifted::new(0x70003ee9f0, fn_70003ee9f0 as *const (), 0), // +0x3ee9f0 sub_3ee9f0
    Lifted::new(0x70003eea80, fn_70003eea80 as *const (), 0), // +0x3eea80 sub_3eea80
    Lifted::new(0x70003eeb00, tpl_364::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x70005a4d1a> as *const (), 2), // +0x3eeb00 sub_3eeb00
    Lifted::new(0x70003eebf0, fn_70003eebf0 as *const (), 2), // +0x3eebf0 sub_3eebf0
    Lifted::new(0x70003eece0, tpl_364::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x70005a4d1a> as *const (), 2), // +0x3eece0 sub_3eece0
    Lifted::new(0x70003eedd0, fn_70003eedd0 as *const (), 2), // +0x3eedd0 sub_3eedd0
    Lifted::new(0x70003eeed0, tpl_307::<0x8, 0x7000233000, 0xc, 0x7000697150, 0x10> as *const (), 0), // +0x3eeed0 sub_3eeed0
    Lifted::new(0x70003eef40, fn_70003eef40 as *const (), 0), // +0x3eef40 sub_3eef40
    Lifted::new(0x70003ef100, fn_70003ef100 as *const (), 0), // +0x3ef100 sub_3ef100
    Lifted::new(0x70003ef1a0, fn_70003ef1a0 as *const (), 3), // +0x3ef1a0 sub_3ef1a0
    Lifted::new(0x70003ef2d0, fn_70003ef2d0 as *const (), 2), // +0x3ef2d0 sub_3ef2d0
    Lifted::new(0x70003ef430, fn_70003ef430 as *const (), 3), // +0x3ef430 sub_3ef430
    Lifted::new(0x70003ef4e0, fn_70003ef4e0 as *const (), 0), // +0x3ef4e0 sub_3ef4e0
    Lifted::new(0x70003ef8c0, fn_70003ef8c0 as *const (), 2), // +0x3ef8c0 sub_3ef8c0
    Lifted::new(0x70003efd10, fn_70003efd10 as *const (), 0), // +0x3efd10 sub_3efd10
    Lifted::new(0x70003eff50, fn_70003eff50 as *const (), 0), // +0x3eff50 sub_3eff50
    Lifted::new(0x70003f0190, fn_70003f0190 as *const (), 2), // +0x3f0190 sub_3f0190
    Lifted::new(0x70003f04f0, fn_70003f04f0 as *const (), 0), // +0x3f04f0 sub_3f04f0
    Lifted::new(0x70003f0790, fn_70003f0790 as *const (), 2), // +0x3f0790 sub_3f0790
    Lifted::new(0x70003f0a20, fn_70003f0a20 as *const (), 0), // +0x3f0a20 sub_3f0a20
    Lifted::new(0x70003f0c60, fn_70003f0c60 as *const (), 2), // +0x3f0c60 sub_3f0c60
    Lifted::new(0x70003f0e80, fn_70003f0e80 as *const (), 0), // +0x3f0e80 sub_3f0e80
    Lifted::new(0x70003f1180, tpl_75::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x700012fa40, 0x70003f1480> as *const (), 0), // +0x3f1180 sub_3f1180
    Lifted::new(0x70003f1250, fn_70003f1250 as *const (), 0), // +0x3f1250 sub_3f1250
    Lifted::new(0x70003f12e0, fn_70003f12e0 as *const (), 2), // +0x3f12e0 sub_3f12e0
    Lifted::new(0x70003f1480, fn_70003f1480 as *const (), 2), // +0x3f1480 sub_3f1480
    Lifted::new(0x70003f1730, fn_70003f1730 as *const (), 2), // +0x3f1730 sub_3f1730
    Lifted::new(0x70003f1a10, fn_70003f1a10 as *const (), 3), // +0x3f1a10 sub_3f1a10
    Lifted::new(0x70003f1ad0, tpl_165::<0x18, 0x8, 0x70003f3470> as *const (), 0), // +0x3f1ad0 sub_3f1ad0
    Lifted::new(0x70003f1b60, fn_70003f1b60 as *const (), 3), // +0x3f1b60 sub_3f1b60
    Lifted::new(0x70003f1f90, fn_70003f1f90 as *const (), 2), // +0x3f1f90 sub_3f1f90
    Lifted::new(0x70003f25b0, fn_70003f25b0 as *const (), 3), // +0x3f25b0 sub_3f25b0
    Lifted::new(0x70003f2b70, fn_70003f2b70 as *const (), 2), // +0x3f2b70 sub_3f2b70
    Lifted::new(0x70003f2ca0, fn_70003f2ca0 as *const (), 2), // +0x3f2ca0 sub_3f2ca0
    Lifted::new(0x70003f2e40, fn_70003f2e40 as *const (), 2), // +0x3f2e40 sub_3f2e40
    Lifted::new(0x70003f31e0, fn_70003f31e0 as *const (), 3), // +0x3f31e0 sub_3f31e0
    Lifted::new(0x70003f3470, fn_70003f3470 as *const (), 2), // +0x3f3470 sub_3f3470
    Lifted::new(0x70003f37d0, fn_70003f37d0 as *const (), 0), // +0x3f37d0 sub_3f37d0
    Lifted::new(0x70003f38f0, fn_70003f38f0 as *const (), 1), // +0x3f38f0 sub_3f38f0
    Lifted::new(0x70003f3910, fn_70003f3910 as *const (), 0), // +0x3f3910 sub_3f3910
    Lifted::new(0x70003f3960, fn_70003f3960 as *const (), 2), // +0x3f3960 sub_3f3960
    Lifted::new(0x70003f3b50, fn_70003f3b50 as *const (), 2), // +0x3f3b50 sub_3f3b50
    Lifted::new(0x70003f3d40, fn_70003f3d40 as *const (), 2), // +0x3f3d40 sub_3f3d40
    Lifted::new(0x70003f3f30, fn_70003f3f30 as *const (), 2), // +0x3f3f30 sub_3f3f30
    Lifted::new(0x70003f4120, fn_70003f4120 as *const (), 2), // +0x3f4120 sub_3f4120
    Lifted::new(0x70003f4310, fn_70003f4310 as *const (), 2), // +0x3f4310 sub_3f4310
    Lifted::new(0x70003f4500, fn_70003f4500 as *const (), 2), // +0x3f4500 sub_3f4500
    Lifted::new(0x70003f46f0, fn_70003f46f0 as *const (), 2), // +0x3f46f0 sub_3f46f0
    Lifted::new(0x70003f49e0, fn_70003f49e0 as *const (), 3), // +0x3f49e0 sub_3f49e0
    Lifted::new(0x70003f4b90, fn_70003f4b90 as *const (), 0), // +0x3f4b90 sub_3f4b90
    Lifted::new(0x70003f4db0, fn_70003f4db0 as *const (), 1), // +0x3f4db0 sub_3f4db0
    Lifted::new(0x70003f4e40, fn_70003f4e40 as *const (), 0), // +0x3f4e40 sub_3f4e40
    Lifted::new(0x70003f4fc0, fn_70003f4fc0 as *const (), 2), // +0x3f4fc0 sub_3f4fc0
    Lifted::new(0x70003f5220, fn_70003f5220 as *const (), 0), // +0x3f5220 sub_3f5220
    Lifted::new(0x70003f5430, fn_70003f5430 as *const (), 2), // +0x3f5430 sub_3f5430
    Lifted::new(0x70003f5e90, fn_70003f5e90 as *const (), 0), // +0x3f5e90 sub_3f5e90
    Lifted::new(0x70003f5f90, fn_70003f5f90 as *const (), 2), // +0x3f5f90 sub_3f5f90
    Lifted::new(0x70003f6030, fn_70003f6030 as *const (), 0), // +0x3f6030 sub_3f6030
    Lifted::new(0x70003f6450, fn_70003f6450 as *const (), 0), // +0x3f6450 sub_3f6450
    Lifted::new(0x70003f6720, fn_70003f6720 as *const (), 0), // +0x3f6720 sub_3f6720
    Lifted::new(0x70003f68d0, fn_70003f68d0 as *const (), 2), // +0x3f68d0 sub_3f68d0
    Lifted::new(0x70003f6d40, fn_70003f6d40 as *const (), 2), // +0x3f6d40 sub_3f6d40
    Lifted::new(0x70003f7150, tpl_345::<0x700062e254, 0x70005ded38, 0x70005ded30, 0x7000233000, 0x70003f7c70, 0x70003eeed0> as *const (), 2), // +0x3f7150 sub_3f7150
    Lifted::new(0x70003f72e0, fn_70003f72e0 as *const (), 0), // +0x3f72e0 sub_3f72e0
    Lifted::new(0x70003f7360, fn_70003f7360 as *const (), 0), // +0x3f7360 sub_3f7360
    Lifted::new(0x70003f7580, fn_70003f7580 as *const (), 0), // +0x3f7580 sub_3f7580
    Lifted::new(0x70003f77a0, fn_70003f77a0 as *const (), 0), // +0x3f77a0 sub_3f77a0
    Lifted::new(0x70003f7a00, tpl_66::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70003f7c70> as *const (), 2), // +0x3f7a00 sub_3f7a00
    Lifted::new(0x70003f7a90, fn_70003f7a90 as *const (), 0), // +0x3f7a90 sub_3f7a90
    Lifted::new(0x70003f7c70, fn_70003f7c70 as *const (), 0), // +0x3f7c70 sub_3f7c70
    Lifted::new(0x70003f7d00, fn_70003f7d00 as *const (), 2), // +0x3f7d00 sub_3f7d00
    Lifted::new(0x70003f7e40, fn_70003f7e40 as *const (), 0), // +0x3f7e40 sub_3f7e40
    Lifted::new(0x70003f7ef0, fn_70003f7ef0 as *const (), 0), // +0x3f7ef0 sub_3f7ef0
    Lifted::new(0x70003f8030, fn_70003f8030 as *const (), 0), // +0x3f8030 sub_3f8030
    Lifted::new(0x70003f81e0, tpl_174::<0x70003f8030, 0x70003f8cc0, 0x70003f81e0, 0x70003f8e50> as *const (), 0), // +0x3f81e0 sub_3f81e0
    Lifted::new(0x70003f82e0, fn_70003f82e0 as *const (), 2), // +0x3f82e0 sub_3f82e0
    Lifted::new(0x70003f85f0, fn_70003f85f0 as *const (), 3), // +0x3f85f0 sub_3f85f0
    Lifted::new(0x70003f86b0, fn_70003f86b0 as *const (), 2), // +0x3f86b0 sub_3f86b0
    Lifted::new(0x70003f88b0, tpl_74::<0x10, 0x2, 0x70003f88e0> as *const (), 0), // +0x3f88b0 sub_3f88b0
    Lifted::new(0x70003f88e0, fn_70003f88e0 as *const (), 2), // +0x3f88e0 sub_3f88e0
    Lifted::new(0x70003f8c40, tpl_25::<0x7000694974, 0x7000699690, 0x70005a4174, 0x70005a6d60, 0x7000699698, 0x70006996c0, 0x70006996e8, 0x7000699710, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x3f8c40 sub_3f8c40
    Lifted::new(0x70003f8cc0, fn_70003f8cc0 as *const (), 0), // +0x3f8cc0 sub_3f8cc0
    Lifted::new(0x70003f8e50, fn_70003f8e50 as *const (), 0), // +0x3f8e50 sub_3f8e50
    Lifted::new(0x70003f9170, fn_70003f9170 as *const (), 0), // +0x3f9170 sub_3f9170
    Lifted::new(0x70003f9210, fn_70003f9210 as *const (), 2), // +0x3f9210 sub_3f9210
    Lifted::new(0x70003f9470, fn_70003f9470 as *const (), 0), // +0x3f9470 sub_3f9470
    Lifted::new(0x70003f9590, fn_70003f9590 as *const (), 0), // +0x3f9590 sub_3f9590
    Lifted::new(0x70003f96b0, fn_70003f96b0 as *const (), 0), // +0x3f96b0 sub_3f96b0
    Lifted::new(0x70003f97f0, fn_70003f97f0 as *const (), 0), // +0x3f97f0 sub_3f97f0
    Lifted::new(0x70003f9940, fn_70003f9940 as *const (), 1), // +0x3f9940 sub_3f9940
    Lifted::new(0x70003f9a30, fn_70003f9a30 as *const (), 0), // +0x3f9a30 sub_3f9a30
    Lifted::new(0x70003f9c00, fn_70003f9c00 as *const (), 1), // +0x3f9c00 sub_3f9c00
    Lifted::new(0x70003f9ce0, fn_70003f9ce0 as *const (), 3), // +0x3f9ce0 sub_3f9ce0
    Lifted::new(0x70003f9ec0, tpl_365::<0x70005a8310, 0x700048c2c0> as *const (), 0), // +0x3f9ec0 sub_3f9ec0
    Lifted::new(0x70003f9f00, tpl_365::<0x70005a8328, 0x700048c2c0> as *const (), 0), // +0x3f9f00 sub_3f9f00
    Lifted::new(0x70003f9f40, tpl_365::<0x70005a8338, 0x700048c2c0> as *const (), 0), // +0x3f9f40 sub_3f9f40
    Lifted::new(0x70003f9f80, fn_70003f9f80 as *const (), 0), // +0x3f9f80 sub_3f9f80
    Lifted::new(0x70003f9f90, fn_70003f9f90 as *const (), 0), // +0x3f9f90 sub_3f9f90
    Lifted::new(0x70003f9fa0, tpl_366::<0x70005a8448> as *const (), 0), // +0x3f9fa0 sub_3f9fa0
    Lifted::new(0x70003f9fb0, fn_70003f9fb0 as *const (), 0), // +0x3f9fb0 sub_3f9fb0
    Lifted::new(0x70003f9fc0, tpl_366::<0x70005a8460> as *const (), 0), // +0x3f9fc0 sub_3f9fc0
    Lifted::new(0x70003f9fd0, fn_70003f9fd0 as *const (), 0), // +0x3f9fd0 sub_3f9fd0
    Lifted::new(0x70003fa060, fn_70003fa060 as *const (), 0), // +0x3fa060 sub_3fa060
    Lifted::new(0x70003fa0a0, tpl_78::<0x700062dda8, 0x700048c2c0> as *const (), 0), // +0x3fa0a0 sub_3fa0a0
    Lifted::new(0x70003fa0d0, fn_70003fa0d0 as *const (), 0), // +0x3fa0d0 sub_3fa0d0
    Lifted::new(0x70003fa0f0, fn_70003fa0f0 as *const (), 0), // +0x3fa0f0 sub_3fa0f0
    Lifted::new(0x70003fa100, fn_70003fa100 as *const (), 0), // +0x3fa100 sub_3fa100
    Lifted::new(0x70003fa120, tpl_367::<0x8, 0x10> as *const (), 0), // +0x3fa120 sub_3fa120
    Lifted::new(0x70003fa140, fn_70003fa140 as *const (), 0), // +0x3fa140 sub_3fa140
    Lifted::new(0x70003fa160, tpl_367::<0x8, 0x20> as *const (), 0), // +0x3fa160 sub_3fa160
    Lifted::new(0x70003fa180, fn_70003fa180 as *const (), 0), // +0x3fa180 sub_3fa180
    Lifted::new(0x70003fa2e0, fn_70003fa2e0 as *const (), 0), // +0x3fa2e0 sub_3fa2e0
    Lifted::new(0x70003fa440, fn_70003fa440 as *const (), 0), // +0x3fa440 sub_3fa440
    Lifted::new(0x70003fa540, tpl_164::<0x700062dd50> as *const (), 1), // +0x3fa540 ??1Library@cohtml@@UEAA@XZ
    Lifted::new(0x70003fa550, fn_70003fa550 as *const (), 0), // +0x3fa550 ?Initialize@Library@cohtml@@SAPEAV12@PEBDAEBULibraryParams@2@@Z
    Lifted::new(0x70003fa570, fn_70003fa570 as *const (), 0), // +0x3fa570 ?InitializeWithRenderingLibrary@Library@cohtml@@SAPEAV12@PEBDAEBULibraryParamsWithExternalRenderingLibrary@2@PEAVIRender
    Lifted::new(0x70003fa5b0, fn_70003fa5b0 as *const (), 0), // +0x3fa5b0 sub_3fa5b0
    Lifted::new(0x70003fa6e0, tpl_9::<0x70006163b0> as *const (), 1), // +0x3fa6e0 ?GetDefaultUserAgent@Library@cohtml@@SAPEBDXZ
    Lifted::new(0x70003fa6f0, tpl_142::<0x70003fa730, 0x700048c2c0> as *const (), 0), // +0x3fa6f0 sub_3fa6f0
    Lifted::new(0x70003fa730, fn_70003fa730 as *const (), 0), // +0x3fa730 sub_3fa730
    Lifted::new(0x70003fa8b0, fn_70003fa8b0 as *const (), 2), // +0x3fa8b0 sub_3fa8b0
    Lifted::new(0x70003fb5d0, fn_70003fb5d0 as *const (), 0), // +0x3fb5d0 sub_3fb5d0
    Lifted::new(0x70003fb740, fn_70003fb740 as *const (), 0), // +0x3fb740 sub_3fb740
    Lifted::new(0x70003fba50, fn_70003fba50 as *const (), 2), // +0x3fba50 sub_3fba50
    Lifted::new(0x70003fbb30, fn_70003fbb30 as *const (), 0), // +0x3fbb30 sub_3fbb30
    Lifted::new(0x70003fbc20, tpl_78::<0x70005ffba8, 0x700048c2c0> as *const (), 0), // +0x3fbc20 sub_3fbc20
    Lifted::new(0x70003fbc50, fn_70003fbc50 as *const (), 0), // +0x3fbc50 sub_3fbc50
    Lifted::new(0x70003fbd60, tpl_207::<0x50, 0xf0> as *const (), 0), // +0x3fbd60 sub_3fbd60
    Lifted::new(0x70003fbd70, fn_70003fbd70 as *const (), 0), // +0x3fbd70 sub_3fbd70
    Lifted::new(0x70003fbf20, fn_70003fbf20 as *const (), 0), // +0x3fbf20 ?HintThreadUsage@Library@cohtml@@SAXW4WorkType@2@@Z
    Lifted::new(0x70003fbf50, fn_70003fbf50 as *const (), 0), // +0x3fbf50 ?DecodeURLString@Library@cohtml@@SAXPEBDIPEADPEAI@Z
    Lifted::new(0x70003fc050, tpl_274::<0x50, 0x140> as *const (), 0), // +0x3fc050 sub_3fc050
    Lifted::new(0x70003fc070, tpl_368::<0x50, 0x148> as *const (), 0), // +0x3fc070 sub_3fc070
    Lifted::new(0x70003fc090, tpl_9::<0x70006166a8> as *const (), 1), // +0x3fc090 ?GetBuildSDKVersion@Library@cohtml@@SAPEBDXZ
    Lifted::new(0x70003fc0a0, tpl_9::<0x70006167f0> as *const (), 1), // +0x3fc0a0 ?GetLibraryVersion@Library@cohtml@@SAPEBDXZ
    Lifted::new(0x70003fc0b0, fn_70003fc0b0 as *const (), 0), // +0x3fc0b0 ?SetScriptEngineAllocator@Library@cohtml@@SAXPEAVIAllocator@2@@Z
    Lifted::new(0x70003fc100, fn_70003fc100 as *const (), 0), // +0x3fc100 ?AdaptRenderingAllocatorAndLoggerFromCohtml@cohtml@@YAXPEAVIAllocator@1@PEAVILogHandler@Logging@1@PEAPEAV2renoir@@PEAPEA
    Lifted::new(0x70003fc1a0, fn_70003fc1a0 as *const (), 0), // +0x3fc1a0 ?DestroyAdaptedRenderingAllocatorAndLogger@cohtml@@YAXPEAVIAllocator@1@PEAV2renoir@@PEAVILogHandler@Logging@3@@Z
    Lifted::new(0x70003fc210, fn_70003fc210 as *const (), 0), // +0x3fc210 sub_3fc210
    Lifted::new(0x70003fc270, fn_70003fc270 as *const (), 0), // +0x3fc270 sub_3fc270
    Lifted::new(0x70003fc320, fn_70003fc320 as *const (), 0), // +0x3fc320 sub_3fc320
    Lifted::new(0x70003fc4c0, fn_70003fc4c0 as *const (), 0), // +0x3fc4c0 sub_3fc4c0
    Lifted::new(0x70003fc520, tpl_365::<0x70005a8398, 0x700048c2c0> as *const (), 0), // +0x3fc520 sub_3fc520
    Lifted::new(0x70003fc560, fn_70003fc560 as *const (), 0), // +0x3fc560 sub_3fc560
    Lifted::new(0x70003fc640, fn_70003fc640 as *const (), 0), // +0x3fc640 sub_3fc640
    Lifted::new(0x70003fc7c0, tpl_160::<0x700062dd50> as *const (), 1), // +0x3fc7c0 ??0Library@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fc7d0, tpl_161::<0xfffffffffffffff8, 0x700062dd50, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fc7d0 sub_3fc7d0
    Lifted::new(0x70003fc860, tpl_160::<0x700062dfd0> as *const (), 1), // +0x3fc860 ??0ILogHandler@Logging@cohtml@@QEAA@AEBV012@@Z
    Lifted::new(0x70003fc870, tpl_161::<0xfffffffffffffff8, 0x700062dfd0, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fc870 sub_3fc870
    Lifted::new(0x70003fc900, tpl_160::<0x700062df60> as *const (), 1), // +0x3fc900 ??0IPerformanceHandler@Profile@cohtml@@QEAA@AEBV012@@Z
    Lifted::new(0x70003fc910, tpl_161::<0xfffffffffffffff8, 0x700062df60, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fc910 sub_3fc910
    Lifted::new(0x70003fc9a0, tpl_160::<0x700062dfe8> as *const (), 1), // +0x3fc9a0 ??0IDataStorageRequest@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fc9b0, tpl_161::<0xfffffffffffffff8, 0x700062dfe8, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fc9b0 sub_3fc9b0
    Lifted::new(0x70003fca40, tpl_160::<0x700062dfb0> as *const (), 1), // +0x3fca40 ??0IDataStorageResponse@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fca50, tpl_161::<0xfffffffffffffff8, 0x700062dfb0, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fca50 sub_3fca50
    Lifted::new(0x70003fcae0, tpl_160::<0x700062df00> as *const (), 1), // +0x3fcae0 ??0IDataStorageHandler@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fcaf0, tpl_161::<0xfffffffffffffff8, 0x700062df00, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fcaf0 sub_3fcaf0
    Lifted::new(0x70003fcb80, tpl_160::<0x700062ddf0> as *const (), 1), // +0x3fcb80 ??0IAsyncResourceRequest@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fcb90, tpl_161::<0xfffffffffffffff8, 0x700062ddf0, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fcb90 sub_3fcb90
    Lifted::new(0x70003fcc20, tpl_160::<0x700062de68> as *const (), 1), // +0x3fcc20 ??0IAsyncResourceResponse@cohtml@@IEAA@XZ
    Lifted::new(0x70003fcc30, tpl_161::<0xfffffffffffffff8, 0x700062de68, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fcc30 sub_3fcc30
    Lifted::new(0x70003fccc0, tpl_160::<0x700062df90> as *const (), 1), // +0x3fccc0 ??0IAsyncResourceStreamResponse@cohtml@@IEAA@XZ
    Lifted::new(0x70003fccd0, tpl_161::<0xfffffffffffffff8, 0x700062df90, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fccd0 sub_3fccd0
    Lifted::new(0x70003fcd60, tpl_160::<0x700062df38> as *const (), 1), // +0x3fcd60 ??0IAsyncResourceHandler@cohtml@@IEAA@XZ
    Lifted::new(0x70003fcd70, tpl_161::<0xfffffffffffffff8, 0x700062df38, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fcd70 sub_3fcd70
    Lifted::new(0x70003fce00, tpl_160::<0x700062deb8> as *const (), 1), // +0x3fce00 ??0ISyncStreamReader@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fce10, tpl_369::<0xfffffffffffffff8, 0x700062deb8, 0x70005a8300, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fce10 sub_3fce10
    Lifted::new(0x70003fcee0, tpl_160::<0x700062e048> as *const (), 1), // +0x3fcee0 ??0IFileSystemReader@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fcef0, tpl_369::<0xfffffffffffffff8, 0x700062e048, 0x70005a8390, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fcef0 sub_3fcef0
    Lifted::new(0x70003fcfc0, tpl_160::<0x700062dee0> as *const (), 1), // +0x3fcfc0 ??0ISyncStreamWriter@cohtml@@IEAA@XZ
    Lifted::new(0x70003fcfd0, tpl_161::<0xfffffffffffffff8, 0x700062dee0, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fcfd0 sub_3fcfd0
    Lifted::new(0x70003fd060, tpl_160::<0x700062df78> as *const (), 1), // +0x3fd060 ??0IFileSystemWriter@cohtml@@IEAA@XZ
    Lifted::new(0x70003fd070, tpl_161::<0xfffffffffffffff8, 0x700062df78, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fd070 sub_3fd070
    Lifted::new(0x70003fd100, tpl_160::<0x700062dda8> as *const (), 1), // +0x3fd100 ??0IAllocator@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fd110, tpl_161::<0xfffffffffffffff8, 0x700062dda8, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fd110 sub_3fd110
    Lifted::new(0x70003fd1a0, tpl_160::<0x700062e018> as *const (), 1), // +0x3fd1a0 ??0IVirtualAllocator@cohtml@@QEAA@AEBV01@@Z
    Lifted::new(0x70003fd1b0, tpl_161::<0xfffffffffffffff8, 0x700062e018, 0x8, 0x7000021150, 0x700048c2c0> as *const (), 0), // +0x3fd1b0 sub_3fd1b0
    Lifted::new(0x70003fd240, fn_70003fd240 as *const (), 0), // +0x3fd240 sub_3fd240
    Lifted::new(0x70003fd2e0, fn_70003fd2e0 as *const (), 0), // +0x3fd2e0 sub_3fd2e0
    Lifted::new(0x70003fd560, fn_70003fd560 as *const (), 0), // +0x3fd560 sub_3fd560
    Lifted::new(0x70003fd5a0, fn_70003fd5a0 as *const (), 0), // +0x3fd5a0 sub_3fd5a0
    Lifted::new(0x70003fd5f0, fn_70003fd5f0 as *const (), 0), // +0x3fd5f0 sub_3fd5f0
    Lifted::new(0x70003fd654, tpl_204::<0xfffffffffffffffc, 0x70003fd5f0> as *const (), 0), // +0x3fd654 sub_3fd654
    Lifted::new(0x70003fd660, fn_70003fd660 as *const (), 0), // +0x3fd660 sub_3fd660
    Lifted::new(0x70003fd6e0, fn_70003fd6e0 as *const (), 2), // +0x3fd6e0 sub_3fd6e0
    Lifted::new(0x70003fd9b0, fn_70003fd9b0 as *const (), 0), // +0x3fd9b0 sub_3fd9b0
    Lifted::new(0x70003fdb50, fn_70003fdb50 as *const (), 0), // +0x3fdb50 sub_3fdb50
    Lifted::new(0x70003fdc40, fn_70003fdc40 as *const (), 0), // +0x3fdc40 sub_3fdc40
    Lifted::new(0x70003fdd10, fn_70003fdd10 as *const (), 0), // +0x3fdd10 sub_3fdd10
    Lifted::new(0x70003fdd20, fn_70003fdd20 as *const (), 2), // +0x3fdd20 sub_3fdd20
    Lifted::new(0x70003fe200, tpl_101::<0x70005a8240, 0x7000616a68, 0x70002445b0, 0x70005a81b0, 0x70005a8230, 0x7000697150> as *const (), 0), // +0x3fe200 sub_3fe200
    Lifted::new(0x70003fe380, fn_70003fe380 as *const (), 0), // +0x3fe380 sub_3fe380
    Lifted::new(0x70003fe3e0, fn_70003fe3e0 as *const (), 0), // +0x3fe3e0 sub_3fe3e0
    Lifted::new(0x70003fe7d0, tpl_370::<0x18, 0xc0, 0x7000692760, 0x10> as *const (), 1), // +0x3fe7d0 sub_3fe7d0
    Lifted::new(0x70003fe800, tpl_370::<0x20, 0xc0, 0x7000692760, 0x10> as *const (), 1), // +0x3fe800 sub_3fe800
    Lifted::new(0x70003fe830, fn_70003fe830 as *const (), 0), // +0x3fe830 sub_3fe830
    Lifted::new(0x70003fe840, tpl_371::<0x30, 0x38, 0x39, 0x48> as *const (), 1), // +0x3fe840 sub_3fe840
    Lifted::new(0x70003fe860, tpl_371::<0x50, 0x58, 0x59, 0x68> as *const (), 1), // +0x3fe860 sub_3fe860
    Lifted::new(0x70003fe880, fn_70003fe880 as *const (), 0), // +0x3fe880 sub_3fe880
    Lifted::new(0x70003fe9b0, tpl_143::<0x98> as *const (), 1), // +0x3fe9b0 sub_3fe9b0
    Lifted::new(0x70003fe9c0, fn_70003fe9c0 as *const (), 0), // +0x3fe9c0 sub_3fe9c0
    Lifted::new(0x70003fea10, fn_70003fea10 as *const (), 0), // +0x3fea10 sub_3fea10
    Lifted::new(0x70003fea70, fn_70003fea70 as *const (), 1), // +0x3fea70 sub_3fea70
    Lifted::new(0x70003fea90, fn_70003fea90 as *const (), 1), // +0x3fea90 sub_3fea90
    Lifted::new(0x70003feab0, fn_70003feab0 as *const (), 2), // +0x3feab0 sub_3feab0
    Lifted::new(0x70003fec30, fn_70003fec30 as *const (), 0), // +0x3fec30 sub_3fec30
    Lifted::new(0x70003fed20, fn_70003fed20 as *const (), 0), // +0x3fed20 sub_3fed20
    Lifted::new(0x70003feec0, fn_70003feec0 as *const (), 2), // +0x3feec0 sub_3feec0
    Lifted::new(0x70003feff0, fn_70003feff0 as *const (), 0), // +0x3feff0 sub_3feff0
    Lifted::new(0x70003ff080, fn_70003ff080 as *const (), 0), // +0x3ff080 sub_3ff080
    Lifted::new(0x70003ff140, fn_70003ff140 as *const (), 1), // +0x3ff140 sub_3ff140
    Lifted::new(0x70003ff150, fn_70003ff150 as *const (), 0), // +0x3ff150 sub_3ff150
    Lifted::new(0x70003ff160, fn_70003ff160 as *const (), 0), // +0x3ff160 sub_3ff160
    Lifted::new(0x70003ff3d0, fn_70003ff3d0 as *const (), 2), // +0x3ff3d0 sub_3ff3d0
    Lifted::new(0x70003ff880, fn_70003ff880 as *const (), 2), // +0x3ff880 sub_3ff880
    Lifted::new(0x70003ffa80, fn_70003ffa80 as *const (), 0), // +0x3ffa80 sub_3ffa80
    Lifted::new(0x70003ffb00, fn_70003ffb00 as *const (), 0), // +0x3ffb00 sub_3ffb00
    Lifted::new(0x70003ffbb0, tpl_372::<0xc0> as *const (), 1), // +0x3ffbb0 sub_3ffbb0
    Lifted::new(0x70003ffbc0, fn_70003ffbc0 as *const (), 2), // +0x3ffbc0 sub_3ffbc0
    Lifted::new(0x70004000b0, tpl_373::<0x1c, 0x7000697150, 0x10, 0x700030b200> as *const (), 0), // +0x4000b0 sub_4000b0
    Lifted::new(0x7000400110, fn_7000400110 as *const (), 2), // +0x400110 sub_400110
    Lifted::new(0x7000401140, fn_7000401140 as *const (), 2), // +0x401140 sub_401140
    Lifted::new(0x70004018b0, fn_70004018b0 as *const (), 0), // +0x4018b0 sub_4018b0
    Lifted::new(0x7000401990, fn_7000401990 as *const (), 0), // +0x401990 sub_401990
    Lifted::new(0x70004019c0, fn_70004019c0 as *const (), 0), // +0x4019c0 sub_4019c0
    Lifted::new(0x7000401a80, fn_7000401a80 as *const (), 0), // +0x401a80 sub_401a80
    Lifted::new(0x7000401af0, fn_7000401af0 as *const (), 2), // +0x401af0 sub_401af0
    Lifted::new(0x7000401ef0, fn_7000401ef0 as *const (), 0), // +0x401ef0 sub_401ef0
    Lifted::new(0x70004020c0, tpl_374::<0x8, 0x70004023d0, 0x3, 0x70002dae60> as *const (), 0), // +0x4020c0 sub_4020c0
    Lifted::new(0x7000402170, tpl_374::<0x8, 0x70004023d0, 0x3, 0x70002cec30> as *const (), 0), // +0x402170 sub_402170
    Lifted::new(0x7000402220, fn_7000402220 as *const (), 0), // +0x402220 sub_402220
    Lifted::new(0x70004022a0, fn_70004022a0 as *const (), 0), // +0x4022a0 sub_4022a0
    Lifted::new(0x7000402330, fn_7000402330 as *const (), 2), // +0x402330 sub_402330
    Lifted::new(0x70004023d0, tpl_243::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70004025a0> as *const (), 2), // +0x4023d0 sub_4023d0
    Lifted::new(0x7000402470, fn_7000402470 as *const (), 0), // +0x402470 sub_402470
    Lifted::new(0x70004025a0, fn_70004025a0 as *const (), 0), // +0x4025a0 sub_4025a0
    Lifted::new(0x7000402650, fn_7000402650 as *const (), 0), // +0x402650 sub_402650
    Lifted::new(0x70004026f0, tpl_271::<0x18, 0x8, 0x70004027a0> as *const (), 0), // +0x4026f0 sub_4026f0
    Lifted::new(0x70004027a0, fn_70004027a0 as *const (), 2), // +0x4027a0 sub_4027a0
    Lifted::new(0x7000402b00, fn_7000402b00 as *const (), 0), // +0x402b00 sub_402b00
    Lifted::new(0x7000402c10, fn_7000402c10 as *const (), 0), // +0x402c10 sub_402c10
    Lifted::new(0x7000402c80, fn_7000402c80 as *const (), 0), // +0x402c80 sub_402c80
    Lifted::new(0x7000402ca0, fn_7000402ca0 as *const (), 1), // +0x402ca0 sub_402ca0
    Lifted::new(0x7000402cc0, fn_7000402cc0 as *const (), 2), // +0x402cc0 sub_402cc0
    Lifted::new(0x7000406500, fn_7000406500 as *const (), 2), // +0x406500 sub_406500
    Lifted::new(0x7000406760, fn_7000406760 as *const (), 0), // +0x406760 sub_406760
    Lifted::new(0x7000406930, fn_7000406930 as *const (), 0), // +0x406930 sub_406930
    Lifted::new(0x7000406d00, fn_7000406d00 as *const (), 1), // +0x406d00 sub_406d00
    Lifted::new(0x7000406dc0, fn_7000406dc0 as *const (), 2), // +0x406dc0 sub_406dc0
    Lifted::new(0x7000406e50, fn_7000406e50 as *const (), 0), // +0x406e50 sub_406e50
    Lifted::new(0x7000407050, fn_7000407050 as *const (), 2), // +0x407050 sub_407050
    Lifted::new(0x7000407440, fn_7000407440 as *const (), 2), // +0x407440 sub_407440
    Lifted::new(0x7000407560, fn_7000407560 as *const (), 2), // +0x407560 sub_407560
    Lifted::new(0x70004076c0, fn_70004076c0 as *const (), 1), // +0x4076c0 sub_4076c0
    Lifted::new(0x70004076f0, fn_70004076f0 as *const (), 1), // +0x4076f0 sub_4076f0
    Lifted::new(0x7000407710, fn_7000407710 as *const (), 3), // +0x407710 sub_407710
    Lifted::new(0x7000407740, fn_7000407740 as *const (), 3), // +0x407740 sub_407740
    Lifted::new(0x7000407770, fn_7000407770 as *const (), 3), // +0x407770 sub_407770
    Lifted::new(0x70004077a0, fn_70004077a0 as *const (), 3), // +0x4077a0 sub_4077a0
    Lifted::new(0x70004077d0, fn_70004077d0 as *const (), 3), // +0x4077d0 sub_4077d0
    Lifted::new(0x7000407800, fn_7000407800 as *const (), 1), // +0x407800 sub_407800
    Lifted::new(0x7000407830, fn_7000407830 as *const (), 1), // +0x407830 sub_407830
    Lifted::new(0x7000407860, fn_7000407860 as *const (), 3), // +0x407860 sub_407860
    Lifted::new(0x7000407890, fn_7000407890 as *const (), 3), // +0x407890 sub_407890
    Lifted::new(0x70004078c0, fn_70004078c0 as *const (), 3), // +0x4078c0 sub_4078c0
    Lifted::new(0x70004078f0, fn_70004078f0 as *const (), 3), // +0x4078f0 sub_4078f0
    Lifted::new(0x7000407920, fn_7000407920 as *const (), 1), // +0x407920 sub_407920
    Lifted::new(0x7000407940, fn_7000407940 as *const (), 1), // +0x407940 sub_407940
    Lifted::new(0x7000407960, fn_7000407960 as *const (), 1), // +0x407960 sub_407960
    Lifted::new(0x7000407980, fn_7000407980 as *const (), 1), // +0x407980 sub_407980
    Lifted::new(0x70004079a0, fn_70004079a0 as *const (), 2), // +0x4079a0 sub_4079a0
    Lifted::new(0x7000407a30, fn_7000407a30 as *const (), 2), // +0x407a30 sub_407a30
    Lifted::new(0x7000407ac0, fn_7000407ac0 as *const (), 2), // +0x407ac0 sub_407ac0
    Lifted::new(0x7000407b50, fn_7000407b50 as *const (), 2), // +0x407b50 sub_407b50
    Lifted::new(0x7000407be0, fn_7000407be0 as *const (), 3), // +0x407be0 sub_407be0
    Lifted::new(0x7000407c00, fn_7000407c00 as *const (), 1), // +0x407c00 sub_407c00
    Lifted::new(0x7000407c30, fn_7000407c30 as *const (), 3), // +0x407c30 sub_407c30
    Lifted::new(0x7000407c50, fn_7000407c50 as *const (), 3), // +0x407c50 sub_407c50
    Lifted::new(0x7000407c80, fn_7000407c80 as *const (), 0), // +0x407c80 sub_407c80
    Lifted::new(0x7000407cb0, fn_7000407cb0 as *const (), 0), // +0x407cb0 sub_407cb0
    Lifted::new(0x7000407ce0, fn_7000407ce0 as *const (), 1), // +0x407ce0 sub_407ce0
    Lifted::new(0x7000407d10, fn_7000407d10 as *const (), 1), // +0x407d10 sub_407d10
    Lifted::new(0x7000407d40, fn_7000407d40 as *const (), 3), // +0x407d40 sub_407d40
    Lifted::new(0x7000407d70, fn_7000407d70 as *const (), 1), // +0x407d70 sub_407d70
    Lifted::new(0x7000407da0, fn_7000407da0 as *const (), 1), // +0x407da0 sub_407da0
    Lifted::new(0x7000407dd0, fn_7000407dd0 as *const (), 1), // +0x407dd0 sub_407dd0
    Lifted::new(0x7000407df0, fn_7000407df0 as *const (), 1), // +0x407df0 sub_407df0
    Lifted::new(0x7000407e10, fn_7000407e10 as *const (), 2), // +0x407e10 sub_407e10
    Lifted::new(0x70004081d0, fn_70004081d0 as *const (), 2), // +0x4081d0 sub_4081d0
    Lifted::new(0x70004083e0, fn_70004083e0 as *const (), 0), // +0x4083e0 sub_4083e0
    Lifted::new(0x7000408510, fn_7000408510 as *const (), 0), // +0x408510 sub_408510
    Lifted::new(0x7000408540, fn_7000408540 as *const (), 1), // +0x408540 sub_408540
    Lifted::new(0x7000408560, tpl_42::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000292c00> as *const (), 2), // +0x408560 sub_408560
    Lifted::new(0x7000408610, fn_7000408610 as *const (), 0), // +0x408610 sub_408610
    Lifted::new(0x70004086a0, fn_70004086a0 as *const (), 0), // +0x4086a0 sub_4086a0
    Lifted::new(0x7000408850, tpl_375::<0x7000467d00, 0x70001df430, 0x7000697150, 0x10> as *const (), 0), // +0x408850 sub_408850
    Lifted::new(0x70004088d0, fn_70004088d0 as *const (), 2), // +0x4088d0 sub_4088d0
    Lifted::new(0x7000408a80, tpl_375::<0x7000467880, 0x70001df430, 0x7000697150, 0x10> as *const (), 0), // +0x408a80 sub_408a80
    Lifted::new(0x7000408b00, tpl_376::<0x7000466f50, 0x70001df170, 0x7000697150, 0x10> as *const (), 0), // +0x408b00 sub_408b00
    Lifted::new(0x7000408b80, fn_7000408b80 as *const (), 2), // +0x408b80 sub_408b80
    Lifted::new(0x7000408c70, tpl_375::<0x7000469090, 0x70001df430, 0x7000697150, 0x10> as *const (), 0), // +0x408c70 sub_408c70
    Lifted::new(0x7000408cf0, fn_7000408cf0 as *const (), 0), // +0x408cf0 sub_408cf0
    Lifted::new(0x7000408e80, fn_7000408e80 as *const (), 2), // +0x408e80 sub_408e80
    Lifted::new(0x7000408f20, fn_7000408f20 as *const (), 0), // +0x408f20 sub_408f20
    Lifted::new(0x70004090e0, tpl_377::<0x700045c700, 0x8, 0x1> as *const (), 0), // +0x4090e0 sub_4090e0
    Lifted::new(0x7000409120, tpl_376::<0x700047d7f0, 0x70001dea00, 0x7000697150, 0x10> as *const (), 0), // +0x409120 sub_409120
    Lifted::new(0x70004091a0, fn_70004091a0 as *const (), 2), // +0x4091a0 sub_4091a0
    Lifted::new(0x7000409240, fn_7000409240 as *const (), 2), // +0x409240 sub_409240
    Lifted::new(0x7000409400, fn_7000409400 as *const (), 0), // +0x409400 sub_409400
    Lifted::new(0x7000409460, fn_7000409460 as *const (), 2), // +0x409460 sub_409460
    Lifted::new(0x7000409500, tpl_378::<0x700047cf70, 0x3, 0x1, 0x7000697150, 0xffffffffffffffd8, 0x10, 0x20, 0x8> as *const (), 2), // +0x409500 sub_409500
    Lifted::new(0x7000409590, tpl_377::<0x700045b0f0, 0x8, 0x1> as *const (), 0), // +0x409590 sub_409590
    Lifted::new(0x70004095d0, fn_70004095d0 as *const (), 2), // +0x4095d0 sub_4095d0
    Lifted::new(0x7000409660, tpl_378::<0x700047ca30, 0x3, 0x1, 0x7000697150, 0xffffffffffffffd8, 0x10, 0x20, 0x8> as *const (), 2), // +0x409660 sub_409660
    Lifted::new(0x70004096f0, tpl_379::<0x7000456a10, 0x1, 0x8> as *const (), 2), // +0x4096f0 sub_4096f0
    Lifted::new(0x7000409730, tpl_380::<0x7000455050, 0x8, 0x1> as *const (), 0), // +0x409730 sub_409730
    Lifted::new(0x7000409770, fn_7000409770 as *const (), 2), // +0x409770 sub_409770
    Lifted::new(0x7000409820, tpl_381::<0x70004576f0, 0x70001deb80, 0x7000697150, 0x10> as *const (), 0), // +0x409820 sub_409820
    Lifted::new(0x7000409890, fn_7000409890 as *const (), 2), // +0x409890 sub_409890
    Lifted::new(0x70004099a0, fn_70004099a0 as *const (), 0), // +0x4099a0 sub_4099a0
    Lifted::new(0x7000409a00, fn_7000409a00 as *const (), 0), // +0x409a00 sub_409a00
    Lifted::new(0x7000409a60, tpl_382::<0x7000692ff0, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x409a60 sub_409a60
    Lifted::new(0x7000409ad0, tpl_383::<0x7000693080, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x409ad0 sub_409ad0
    Lifted::new(0x7000409b40, fn_7000409b40 as *const (), 2), // +0x409b40 sub_409b40
    Lifted::new(0x7000409be0, tpl_384::<0x7000692f10, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x409be0 sub_409be0
    Lifted::new(0x7000409c40, tpl_382::<0x7000693050, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x409c40 sub_409c40
    Lifted::new(0x7000409cb0, tpl_383::<0x70006930d0, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x409cb0 sub_409cb0
    Lifted::new(0x7000409d20, tpl_383::<0x70006930a8, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x409d20 sub_409d20
    Lifted::new(0x7000409d90, fn_7000409d90 as *const (), 2), // +0x409d90 sub_409d90
    Lifted::new(0x700040a600, fn_700040a600 as *const (), 2), // +0x40a600 sub_40a600
    Lifted::new(0x700040a640, tpl_380::<0x70004699a0, 0x8, 0x1> as *const (), 0), // +0x40a640 sub_40a640
    Lifted::new(0x700040a680, fn_700040a680 as *const (), 0), // +0x40a680 sub_40a680
    Lifted::new(0x700040a750, fn_700040a750 as *const (), 2), // +0x40a750 sub_40a750
    Lifted::new(0x700040a880, fn_700040a880 as *const (), 2), // +0x40a880 sub_40a880
    Lifted::new(0x700040a930, tpl_381::<0x7000459240, 0x70001ded40, 0x7000697150, 0x10> as *const (), 0), // +0x40a930 sub_40a930
    Lifted::new(0x700040a9a0, fn_700040a9a0 as *const (), 0), // +0x40a9a0 sub_40a9a0
    Lifted::new(0x700040a9e0, tpl_376::<0x700045e320, 0x70001df170, 0x7000697150, 0x10> as *const (), 0), // +0x40a9e0 sub_40a9e0
    Lifted::new(0x700040aa60, tpl_384::<0x7000692d88, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40aa60 sub_40aa60
    Lifted::new(0x700040aac0, fn_700040aac0 as *const (), 0), // +0x40aac0 sub_40aac0
    Lifted::new(0x700040ab20, fn_700040ab20 as *const (), 0), // +0x40ab20 sub_40ab20
    Lifted::new(0x700040ab60, tpl_383::<0x7000692ac8, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40ab60 sub_40ab60
    Lifted::new(0x700040abd0, fn_700040abd0 as *const (), 2), // +0x40abd0 sub_40abd0
    Lifted::new(0x700040ad20, fn_700040ad20 as *const (), 2), // +0x40ad20 sub_40ad20
    Lifted::new(0x700040ae80, tpl_385::<0x7000692d68, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40ae80 sub_40ae80
    Lifted::new(0x700040aee0, tpl_386::<0x7000454530, 0x8, 0x1> as *const (), 0), // +0x40aee0 sub_40aee0
    Lifted::new(0x700040af20, fn_700040af20 as *const (), 2), // +0x40af20 sub_40af20
    Lifted::new(0x700040afc0, fn_700040afc0 as *const (), 2), // +0x40afc0 sub_40afc0
    Lifted::new(0x700040b010, fn_700040b010 as *const (), 0), // +0x40b010 sub_40b010
    Lifted::new(0x700040b090, fn_700040b090 as *const (), 0), // +0x40b090 sub_40b090
    Lifted::new(0x700040b120, fn_700040b120 as *const (), 2), // +0x40b120 sub_40b120
    Lifted::new(0x700040b190, tpl_383::<0x7000692b80, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40b190 sub_40b190
    Lifted::new(0x700040b200, fn_700040b200 as *const (), 2), // +0x40b200 sub_40b200
    Lifted::new(0x700040b2b0, tpl_379::<0x70004667a0, 0x1, 0x8> as *const (), 2), // +0x40b2b0 sub_40b2b0
    Lifted::new(0x700040b2f0, tpl_387::<0x7000692d38, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40b2f0 sub_40b2f0
    Lifted::new(0x700040b360, fn_700040b360 as *const (), 0), // +0x40b360 sub_40b360
    Lifted::new(0x700040b4e0, fn_700040b4e0 as *const (), 0), // +0x40b4e0 sub_40b4e0
    Lifted::new(0x700040b550, tpl_388::<0x7000692af0, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40b550 sub_40b550
    Lifted::new(0x700040b5c0, tpl_381::<0x70004581f0, 0x70001dec60, 0x7000697150, 0x10> as *const (), 0), // +0x40b5c0 sub_40b5c0
    Lifted::new(0x700040b630, tpl_382::<0x7000692ba8, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40b630 sub_40b630
    Lifted::new(0x700040b6a0, fn_700040b6a0 as *const (), 2), // +0x40b6a0 sub_40b6a0
    Lifted::new(0x700040b750, fn_700040b750 as *const (), 2), // +0x40b750 sub_40b750
    Lifted::new(0x700040b7e0, tpl_385::<0x7000692e68, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40b7e0 sub_40b7e0
    Lifted::new(0x700040b840, fn_700040b840 as *const (), 0), // +0x40b840 sub_40b840
    Lifted::new(0x700040b8c0, fn_700040b8c0 as *const (), 0), // +0x40b8c0 sub_40b8c0
    Lifted::new(0x700040b930, tpl_388::<0x7000692b60, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40b930 sub_40b930
    Lifted::new(0x700040b9a0, tpl_382::<0x7000692bd8, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40b9a0 sub_40b9a0
    Lifted::new(0x700040ba10, tpl_386::<0x700045e780, 0x8, 0x1> as *const (), 0), // +0x40ba10 sub_40ba10
    Lifted::new(0x700040ba50, tpl_384::<0x7000692ab0, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40ba50 sub_40ba50
    Lifted::new(0x700040bab0, fn_700040bab0 as *const (), 0), // +0x40bab0 sub_40bab0
    Lifted::new(0x700040baf0, fn_700040baf0 as *const (), 0), // +0x40baf0 sub_40baf0
    Lifted::new(0x700040bb30, fn_700040bb30 as *const (), 0), // +0x40bb30 sub_40bb30
    Lifted::new(0x700040bb90, tpl_387::<0x7000692b10, 0x70002af6d0, 0x1, 0x8> as *const (), 0), // +0x40bb90 sub_40bb90
    Lifted::new(0x700040bc00, fn_700040bc00 as *const (), 0), // +0x40bc00 sub_40bc00
    Lifted::new(0x700040bc40, fn_700040bc40 as *const (), 2), // +0x40bc40 sub_40bc40
    Lifted::new(0x700040bcc0, tpl_389::<0x1, 0x10, 0x700036cc60, 0x8> as *const (), 0), // +0x40bcc0 sub_40bcc0
    Lifted::new(0x700040bd10, tpl_389::<0x1, 0x10, 0x700036ca30, 0x8> as *const (), 0), // +0x40bd10 sub_40bd10
    Lifted::new(0x700040bd60, fn_700040bd60 as *const (), 2), // +0x40bd60 sub_40bd60
    Lifted::new(0x700040bde0, tpl_390::<0x1, 0x10, 0x700036c7d0, 0x8> as *const (), 0), // +0x40bde0 sub_40bde0
    Lifted::new(0x700040be20, fn_700040be20 as *const (), 0), // +0x40be20 sub_40be20
    Lifted::new(0x700040be70, tpl_391::<0x1, 0x10, 0x700036bac0, 0x70001df760, 0x7000697150> as *const (), 0), // +0x40be70 sub_40be70
    Lifted::new(0x700040bee0, tpl_391::<0x1, 0x10, 0x700036bcd0, 0x70001df760, 0x7000697150> as *const (), 0), // +0x40bee0 sub_40bee0
    Lifted::new(0x700040bf50, fn_700040bf50 as *const (), 0), // +0x40bf50 sub_40bf50
    Lifted::new(0x700040bfa0, fn_700040bfa0 as *const (), 2), // +0x40bfa0 sub_40bfa0
    Lifted::new(0x700040c030, tpl_390::<0x1, 0x10, 0x700036c900, 0x8> as *const (), 0), // +0x40c030 sub_40c030
    Lifted::new(0x700040c070, fn_700040c070 as *const (), 2), // +0x40c070 sub_40c070
    Lifted::new(0x700040c110, fn_700040c110 as *const (), 2), // +0x40c110 sub_40c110
    Lifted::new(0x700040c1c0, fn_700040c1c0 as *const (), 0), // +0x40c1c0 sub_40c1c0
    Lifted::new(0x700040c2f0, tpl_389::<0x1, 0x10, 0x700036f6e0, 0x8> as *const (), 0), // +0x40c2f0 sub_40c2f0
    Lifted::new(0x700040c340, tpl_390::<0x1, 0x10, 0x700036f910, 0x8> as *const (), 0), // +0x40c340 sub_40c340
    Lifted::new(0x700040c380, fn_700040c380 as *const (), 2), // +0x40c380 sub_40c380
    Lifted::new(0x700040c450, fn_700040c450 as *const (), 0), // +0x40c450 sub_40c450
    Lifted::new(0x700040c490, fn_700040c490 as *const (), 0), // +0x40c490 sub_40c490
    Lifted::new(0x700040c4d0, fn_700040c4d0 as *const (), 2), // +0x40c4d0 sub_40c4d0
    Lifted::new(0x700040c530, fn_700040c530 as *const (), 2), // +0x40c530 sub_40c530
    Lifted::new(0x700040c5a0, fn_700040c5a0 as *const (), 0), // +0x40c5a0 sub_40c5a0
    Lifted::new(0x700040c5e0, tpl_392::<0x7000472a30, 0x7000473990> as *const (), 2), // +0x40c5e0 sub_40c5e0
    Lifted::new(0x700040c620, tpl_392::<0x7000472a30, 0x7000472fd0> as *const (), 2), // +0x40c620 sub_40c620
    Lifted::new(0x700040c660, tpl_392::<0x7000472a30, 0x7000473310> as *const (), 2), // +0x40c660 sub_40c660
    Lifted::new(0x700040c6a0, tpl_392::<0x7000472a30, 0x7000473650> as *const (), 2), // +0x40c6a0 sub_40c6a0
    Lifted::new(0x700040c6e0, tpl_392::<0x7000472a30, 0x7000473cd0> as *const (), 2), // +0x40c6e0 sub_40c6e0
    Lifted::new(0x700040c720, fn_700040c720 as *const (), 0), // +0x40c720 sub_40c720
    Lifted::new(0x700040c760, fn_700040c760 as *const (), 0), // +0x40c760 sub_40c760
    Lifted::new(0x700040c810, fn_700040c810 as *const (), 0), // +0x40c810 sub_40c810
    Lifted::new(0x700040c8c0, fn_700040c8c0 as *const (), 0), // +0x40c8c0 sub_40c8c0
    Lifted::new(0x700040c940, fn_700040c940 as *const (), 0), // +0x40c940 sub_40c940
    Lifted::new(0x700040c9b0, fn_700040c9b0 as *const (), 2), // +0x40c9b0 sub_40c9b0
    Lifted::new(0x700040ca60, fn_700040ca60 as *const (), 0), // +0x40ca60 sub_40ca60
    Lifted::new(0x700040cb20, fn_700040cb20 as *const (), 2), // +0x40cb20 sub_40cb20
    Lifted::new(0x700040cb60, fn_700040cb60 as *const (), 2), // +0x40cb60 sub_40cb60
    Lifted::new(0x700040cc00, fn_700040cc00 as *const (), 2), // +0x40cc00 sub_40cc00
    Lifted::new(0x700040cc50, fn_700040cc50 as *const (), 0), // +0x40cc50 sub_40cc50
    Lifted::new(0x7000417090, fn_7000417090 as *const (), 0), // +0x417090 sub_417090
    Lifted::new(0x70004172b0, fn_70004172b0 as *const (), 2), // +0x4172b0 sub_4172b0
    Lifted::new(0x70004177e0, fn_70004177e0 as *const (), 0), // +0x4177e0 sub_4177e0
    Lifted::new(0x7000417a20, fn_7000417a20 as *const (), 0), // +0x417a20 sub_417a20
    Lifted::new(0x7000417ef0, fn_7000417ef0 as *const (), 0), // +0x417ef0 sub_417ef0
    Lifted::new(0x7000418b20, fn_7000418b20 as *const (), 0), // +0x418b20 sub_418b20
    Lifted::new(0x7000418df0, fn_7000418df0 as *const (), 2), // +0x418df0 sub_418df0
    Lifted::new(0x7000419590, fn_7000419590 as *const (), 0), // +0x419590 sub_419590
    Lifted::new(0x7000419bd0, fn_7000419bd0 as *const (), 0), // +0x419bd0 sub_419bd0
    Lifted::new(0x7000419d30, fn_7000419d30 as *const (), 2), // +0x419d30 sub_419d30
    Lifted::new(0x7000419f00, fn_7000419f00 as *const (), 2), // +0x419f00 sub_419f00
    Lifted::new(0x700041a130, fn_700041a130 as *const (), 2), // +0x41a130 sub_41a130
    Lifted::new(0x700041a370, fn_700041a370 as *const (), 2), // +0x41a370 sub_41a370
    Lifted::new(0x700041a7e0, fn_700041a7e0 as *const (), 0), // +0x41a7e0 sub_41a7e0
    Lifted::new(0x700041aae0, fn_700041aae0 as *const (), 0), // +0x41aae0 sub_41aae0
    Lifted::new(0x700041afb0, fn_700041afb0 as *const (), 2), // +0x41afb0 sub_41afb0
    Lifted::new(0x700041b070, fn_700041b070 as *const (), 2), // +0x41b070 sub_41b070
    Lifted::new(0x700041caa0, fn_700041caa0 as *const (), 0), // +0x41caa0 sub_41caa0
    Lifted::new(0x700041cb70, tpl_181::<0x7000353410, 0x8, 0x10, 0x20, 0x18, 0x1c> as *const (), 0), // +0x41cb70 sub_41cb70
    Lifted::new(0x700041cbb0, tpl_393::<0x7000697190, 0x7000353490> as *const (), 0), // +0x41cbb0 sub_41cbb0
    Lifted::new(0x700041cbd0, tpl_20::<0x1, 0x70003522c0, 0x10, 0x70005a4d14> as *const (), 0), // +0x41cbd0 sub_41cbd0
    Lifted::new(0x700041cc60, tpl_181::<0x700041db40, 0x8, 0x10, 0x20, 0x18, 0x1c> as *const (), 0), // +0x41cc60 sub_41cc60
    Lifted::new(0x700041cca0, tpl_393::<0x70006971a0, 0x700041e980> as *const (), 0), // +0x41cca0 sub_41cca0
    Lifted::new(0x700041ccc0, fn_700041ccc0 as *const (), 0), // +0x41ccc0 sub_41ccc0
    Lifted::new(0x700041cd90, tpl_181::<0x700041da40, 0x8, 0x10, 0x20, 0x18, 0x1c> as *const (), 0), // +0x41cd90 sub_41cd90
    Lifted::new(0x700041cdd0, tpl_393::<0x7000697188, 0x700041eb00> as *const (), 0), // +0x41cdd0 sub_41cdd0
    Lifted::new(0x700041cdf0, tpl_181::<0x700041d9c0, 0x8, 0x10, 0x20, 0x18, 0x1c> as *const (), 0), // +0x41cdf0 sub_41cdf0
    Lifted::new(0x700041ce30, tpl_393::<0x7000697180, 0x700041ec40> as *const (), 0), // +0x41ce30 sub_41ce30
    Lifted::new(0x700041ce50, tpl_374::<0x8, 0x70002ce220, 0x3, 0x70002dae60> as *const (), 0), // +0x41ce50 sub_41ce50
    Lifted::new(0x700041cf00, fn_700041cf00 as *const (), 0), // +0x41cf00 sub_41cf00
    Lifted::new(0x700041d030, fn_700041d030 as *const (), 0), // +0x41d030 sub_41d030
    Lifted::new(0x700041d160, fn_700041d160 as *const (), 1), // +0x41d160 sub_41d160
    Lifted::new(0x700041d170, tpl_17::<0x8, 0x700041d930> as *const (), 0), // +0x41d170 sub_41d170
    Lifted::new(0x700041d1e0, fn_700041d1e0 as *const (), 2), // +0x41d1e0 sub_41d1e0
    Lifted::new(0x700041d390, tpl_181::<0x700041dac0, 0x8, 0x10, 0x20, 0x18, 0x1c> as *const (), 0), // +0x41d390 sub_41d390
    Lifted::new(0x700041d3d0, fn_700041d3d0 as *const (), 2), // +0x41d3d0 sub_41d3d0
    Lifted::new(0x700041d4c0, fn_700041d4c0 as *const (), 0), // +0x41d4c0 sub_41d4c0
    Lifted::new(0x700041d580, fn_700041d580 as *const (), 0), // +0x41d580 sub_41d580
    Lifted::new(0x700041d640, tpl_394::<0x8, 0x19, 0x10, 0x20, 0x700041da40, 0x7000697150> as *const (), 0), // +0x41d640 sub_41d640
    Lifted::new(0x700041d6b0, tpl_394::<0x8, 0x19, 0x10, 0x20, 0x700041d9c0, 0x7000697150> as *const (), 0), // +0x41d6b0 sub_41d6b0
    Lifted::new(0x700041d720, fn_700041d720 as *const (), 2), // +0x41d720 sub_41d720
    Lifted::new(0x700041d890, fn_700041d890 as *const (), 2), // +0x41d890 sub_41d890
    Lifted::new(0x700041d930, tpl_21::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x700041dcc0> as *const (), 2), // +0x41d930 sub_41d930
    Lifted::new(0x700041d9c0, tpl_25::<0x7000694974, 0x700069b6c8, 0x70005a4174, 0x70005a78f0, 0x700069b6d0, 0x700069b6e8, 0x700069b700, 0x700069b718, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x41d9c0 sub_41d9c0
    Lifted::new(0x700041da40, tpl_25::<0x7000694974, 0x700069b730, 0x70005a4174, 0x70005a78e0, 0x700069b740, 0x700069b758, 0x700069b770, 0x700069b788, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x41da40 sub_41da40
    Lifted::new(0x700041dac0, tpl_25::<0x7000694974, 0x700069b7a0, 0x70005a4174, 0x70005a78d0, 0x700069b7b0, 0x700069b7f0, 0x700069b830, 0x700069b870, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x41dac0 sub_41dac0
    Lifted::new(0x700041db40, tpl_25::<0x7000694974, 0x700069b8b0, 0x70005a4174, 0x70005a78c0, 0x700069b8b8, 0x700069b8e0, 0x700069b908, 0x700069b930, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x41db40 sub_41db40
    Lifted::new(0x700041dbc0, fn_700041dbc0 as *const (), 0), // +0x41dbc0 sub_41dbc0
    Lifted::new(0x700041dcc0, fn_700041dcc0 as *const (), 0), // +0x41dcc0 sub_41dcc0
    Lifted::new(0x700041dd40, fn_700041dd40 as *const (), 1), // +0x41dd40 sub_41dd40
    Lifted::new(0x700041dd50, fn_700041dd50 as *const (), 1), // +0x41dd50 sub_41dd50
    Lifted::new(0x700041dd60, fn_700041dd60 as *const (), 0), // +0x41dd60 sub_41dd60
    Lifted::new(0x700041dd90, fn_700041dd90 as *const (), 1), // +0x41dd90 sub_41dd90
    Lifted::new(0x700041ddb0, fn_700041ddb0 as *const (), 1), // +0x41ddb0 sub_41ddb0
    Lifted::new(0x700041ddc0, fn_700041ddc0 as *const (), 0), // +0x41ddc0 sub_41ddc0
    Lifted::new(0x700041ddf0, tpl_395::<0x70005f9d10, 0x8> as *const (), 1), // +0x41ddf0 sub_41ddf0
    Lifted::new(0x700041de10, tpl_395::<0x70005f8338, 0x8> as *const (), 1), // +0x41de10 sub_41de10
    Lifted::new(0x700041de30, fn_700041de30 as *const (), 2), // +0x41de30 sub_41de30
    Lifted::new(0x700041df40, tpl_269::<0x70005a8240, 0x7000617058, 0x70002445b0, 0x70005a81b0, 0x70005a8230> as *const (), 0), // +0x41df40 sub_41df40
    Lifted::new(0x700041e090, tpl_269::<0x70005a8240, 0x7000617030, 0x70002445b0, 0x70005a81b0, 0x70005a8230> as *const (), 0), // +0x41e090 sub_41e090
    Lifted::new(0x700041e1e0, tpl_269::<0x70005a8240, 0x70006170e0, 0x70002445b0, 0x70005a81b0, 0x70005a8230> as *const (), 0), // +0x41e1e0 sub_41e1e0
    Lifted::new(0x700041e330, tpl_269::<0x70005a8240, 0x70006170c0, 0x70002445b0, 0x70005a81b0, 0x70005a8230> as *const (), 0), // +0x41e330 sub_41e330
    Lifted::new(0x700041e480, fn_700041e480 as *const (), 2), // +0x41e480 sub_41e480
    Lifted::new(0x700041e700, fn_700041e700 as *const (), 0), // +0x41e700 sub_41e700
    Lifted::new(0x700041e8d0, fn_700041e8d0 as *const (), 0), // +0x41e8d0 sub_41e8d0
    Lifted::new(0x700041e980, tpl_93::<0x1, 0x10, 0x7000007030, 0x18, 0x8, 0x70001a86c0, 0x700041f0a0> as *const (), 0), // +0x41e980 sub_41e980
    Lifted::new(0x700041ea50, tpl_351::<0x8, 0x18, 0x700041f350> as *const (), 0), // +0x41ea50 sub_41ea50
    Lifted::new(0x700041eb00, tpl_354::<0x18, 0x8, 0x700041f620> as *const (), 0), // +0x41eb00 sub_41eb00
    Lifted::new(0x700041eb90, tpl_351::<0x8, 0x18, 0x700041f850> as *const (), 0), // +0x41eb90 sub_41eb90
    Lifted::new(0x700041ec40, tpl_354::<0x18, 0x8, 0x700041fb20> as *const (), 0), // +0x41ec40 sub_41ec40
    Lifted::new(0x700041ecd0, fn_700041ecd0 as *const (), 0), // +0x41ecd0 sub_41ecd0
    Lifted::new(0x700041ed60, fn_700041ed60 as *const (), 2), // +0x41ed60 sub_41ed60
    Lifted::new(0x700041f0a0, fn_700041f0a0 as *const (), 2), // +0x41f0a0 sub_41f0a0
    Lifted::new(0x700041f350, tpl_396::<0x70004203b0, 0x700041f620, 0x700041ea50> as *const (), 2), // +0x41f350 sub_41f350
    Lifted::new(0x700041f620, tpl_397::<0x70004203b0, 0x700041eb00> as *const (), 2), // +0x41f620 sub_41f620
    Lifted::new(0x700041f850, tpl_396::<0x7000420660, 0x700041fb20, 0x700041eb90> as *const (), 2), // +0x41f850 sub_41f850
    Lifted::new(0x700041fb20, tpl_397::<0x7000420660, 0x700041ec40> as *const (), 2), // +0x41fb20 sub_41fb20
    Lifted::new(0x700041fd50, tpl_74::<0x10, 0x2, 0x700041fd80> as *const (), 0), // +0x41fd50 sub_41fd50
    Lifted::new(0x700041fd80, fn_700041fd80 as *const (), 2), // +0x41fd80 sub_41fd80
    Lifted::new(0x70004200d0, fn_70004200d0 as *const (), 2), // +0x4200d0 sub_4200d0
    Lifted::new(0x70004203b0, tpl_398::<0x70005a4d32, 0x700062e490, 0x700041da40, 0x7000697150, 0x70005f84c0, 0x700041eb00> as *const (), 2), // +0x4203b0 sub_4203b0
    Lifted::new(0x7000420660, tpl_398::<0x70005a4d32, 0x700062e490, 0x700041d9c0, 0x7000697150, 0x70005f84c0, 0x700041ec40> as *const (), 2), // +0x420660 sub_420660
    Lifted::new(0x7000420910, fn_7000420910 as *const (), 0), // +0x420910 sub_420910
    Lifted::new(0x70004209a0, fn_70004209a0 as *const (), 1), // +0x4209a0 sub_4209a0
    Lifted::new(0x7000420a40, fn_7000420a40 as *const (), 0), // +0x420a40 sub_420a40
    Lifted::new(0x7000420ba0, fn_7000420ba0 as *const (), 2), // +0x420ba0 sub_420ba0
    Lifted::new(0x7000420e70, fn_7000420e70 as *const (), 1), // +0x420e70 sub_420e70
    Lifted::new(0x7000420f20, tpl_45::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000402b00> as *const (), 2), // +0x420f20 sub_420f20
    Lifted::new(0x7000420fd0, fn_7000420fd0 as *const (), 0), // +0x420fd0 sub_420fd0
    Lifted::new(0x7000421100, tpl_399::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000402c10> as *const (), 2), // +0x421100 sub_421100
    Lifted::new(0x70004211a0, fn_70004211a0 as *const (), 0), // +0x4211a0 sub_4211a0
    Lifted::new(0x7000421230, fn_7000421230 as *const (), 0), // +0x421230 sub_421230
    Lifted::new(0x70004212a0, fn_70004212a0 as *const (), 0), // +0x4212a0 sub_4212a0
    Lifted::new(0x7000421650, fn_7000421650 as *const (), 0), // +0x421650 sub_421650
    Lifted::new(0x70004216e0, fn_70004216e0 as *const (), 0), // +0x4216e0 sub_4216e0
    Lifted::new(0x7000421850, fn_7000421850 as *const (), 2), // +0x421850 sub_421850
    Lifted::new(0x7000423620, fn_7000423620 as *const (), 2), // +0x423620 sub_423620
    Lifted::new(0x7000423840, fn_7000423840 as *const (), 0), // +0x423840 sub_423840
    Lifted::new(0x70004238f0, fn_70004238f0 as *const (), 2), // +0x4238f0 sub_4238f0
    Lifted::new(0x7000423bb0, fn_7000423bb0 as *const (), 0), // +0x423bb0 sub_423bb0
    Lifted::new(0x7000423d20, fn_7000423d20 as *const (), 0), // +0x423d20 sub_423d20
    Lifted::new(0x7000423f80, fn_7000423f80 as *const (), 0), // +0x423f80 sub_423f80
    Lifted::new(0x7000424070, fn_7000424070 as *const (), 0), // +0x424070 sub_424070
    Lifted::new(0x70004242c0, fn_70004242c0 as *const (), 0), // +0x4242c0 sub_4242c0
    Lifted::new(0x70004247a0, fn_70004247a0 as *const (), 2), // +0x4247a0 sub_4247a0
    Lifted::new(0x7000424940, fn_7000424940 as *const (), 1), // +0x424940 sub_424940
    Lifted::new(0x7000424ab0, fn_7000424ab0 as *const (), 2), // +0x424ab0 sub_424ab0
    Lifted::new(0x70004252b0, fn_70004252b0 as *const (), 0), // +0x4252b0 sub_4252b0
    Lifted::new(0x7000425390, fn_7000425390 as *const (), 0), // +0x425390 sub_425390
    Lifted::new(0x7000425550, fn_7000425550 as *const (), 2), // +0x425550 sub_425550
    Lifted::new(0x7000425790, fn_7000425790 as *const (), 0), // +0x425790 sub_425790
    Lifted::new(0x7000425900, fn_7000425900 as *const (), 0), // +0x425900 sub_425900
    Lifted::new(0x7000425a70, fn_7000425a70 as *const (), 0), // +0x425a70 sub_425a70
    Lifted::new(0x7000425c30, fn_7000425c30 as *const (), 0), // +0x425c30 sub_425c30
    Lifted::new(0x70004261c0, fn_70004261c0 as *const (), 0), // +0x4261c0 sub_4261c0
    Lifted::new(0x7000426420, fn_7000426420 as *const (), 0), // +0x426420 sub_426420
    Lifted::new(0x70004264b0, fn_70004264b0 as *const (), 0), // +0x4264b0 sub_4264b0
    Lifted::new(0x7000426540, fn_7000426540 as *const (), 2), // +0x426540 sub_426540
    Lifted::new(0x7000426650, fn_7000426650 as *const (), 2), // +0x426650 sub_426650
    Lifted::new(0x70004268b0, fn_70004268b0 as *const (), 2), // +0x4268b0 sub_4268b0
    Lifted::new(0x7000426b20, fn_7000426b20 as *const (), 0), // +0x426b20 sub_426b20
    Lifted::new(0x7000426ba0, fn_7000426ba0 as *const (), 2), // +0x426ba0 sub_426ba0
    Lifted::new(0x70004271f0, fn_70004271f0 as *const (), 0), // +0x4271f0 sub_4271f0
    Lifted::new(0x7000427320, fn_7000427320 as *const (), 2), // +0x427320 sub_427320
    Lifted::new(0x7000427450, fn_7000427450 as *const (), 2), // +0x427450 sub_427450
    Lifted::new(0x7000427780, fn_7000427780 as *const (), 0), // +0x427780 sub_427780
    Lifted::new(0x70004279b0, fn_70004279b0 as *const (), 2), // +0x4279b0 sub_4279b0
    Lifted::new(0x7000427df0, fn_7000427df0 as *const (), 0), // +0x427df0 sub_427df0
    Lifted::new(0x7000427f80, fn_7000427f80 as *const (), 0), // +0x427f80 sub_427f80
    Lifted::new(0x7000428060, fn_7000428060 as *const (), 0), // +0x428060 sub_428060
    Lifted::new(0x70004281e0, fn_70004281e0 as *const (), 0), // +0x4281e0 sub_4281e0
    Lifted::new(0x70004282d0, fn_70004282d0 as *const (), 0), // +0x4282d0 sub_4282d0
    Lifted::new(0x70004284d0, fn_70004284d0 as *const (), 1), // +0x4284d0 sub_4284d0
    Lifted::new(0x7000428590, tpl_400::<0x7000428fa0> as *const (), 0), // +0x428590 sub_428590
    Lifted::new(0x70004286b0, tpl_400::<0x7000429180> as *const (), 0), // +0x4286b0 sub_4286b0
    Lifted::new(0x70004287d0, tpl_401::<0x18, 0x8, 0x20, 0x700042ae90, 0x10> as *const (), 0), // +0x4287d0 sub_4287d0
    Lifted::new(0x7000428860, tpl_400::<0x7000429300> as *const (), 0), // +0x428860 sub_428860
    Lifted::new(0x7000428980, tpl_401::<0x18, 0x8, 0x20, 0x700042b1d0, 0x10> as *const (), 0), // +0x428980 sub_428980
    Lifted::new(0x7000428a10, fn_7000428a10 as *const (), 0), // +0x428a10 sub_428a10
    Lifted::new(0x7000428a90, fn_7000428a90 as *const (), 0), // +0x428a90 sub_428a90
    Lifted::new(0x7000428b80, fn_7000428b80 as *const (), 0), // +0x428b80 sub_428b80
    Lifted::new(0x7000428be0, fn_7000428be0 as *const (), 0), // +0x428be0 sub_428be0
    Lifted::new(0x7000428d00, fn_7000428d00 as *const (), 0), // +0x428d00 sub_428d00
    Lifted::new(0x7000428e50, fn_7000428e50 as *const (), 2), // +0x428e50 sub_428e50
    Lifted::new(0x7000428f40, tpl_402::<0x8, 0x700029cbd0, 0xc> as *const (), 0), // +0x428f40 sub_428f40
    Lifted::new(0x7000428fa0, tpl_403::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000694974, 0x700004f420, 0x7000428f40> as *const (), 2), // +0x428fa0 sub_428fa0
    Lifted::new(0x7000429120, tpl_402::<0x8, 0x700029cb70, 0xc> as *const (), 0), // +0x429120 sub_429120
    Lifted::new(0x7000429180, tpl_403::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000694974, 0x700004f420, 0x7000429120> as *const (), 2), // +0x429180 sub_429180
    Lifted::new(0x7000429300, tpl_404::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x700029c700> as *const (), 2), // +0x429300 sub_429300
    Lifted::new(0x7000429460, fn_7000429460 as *const (), 2), // +0x429460 sub_429460
    Lifted::new(0x7000429610, fn_7000429610 as *const (), 2), // +0x429610 sub_429610
    Lifted::new(0x7000429790, fn_7000429790 as *const (), 2), // +0x429790 sub_429790
    Lifted::new(0x7000429920, fn_7000429920 as *const (), 0), // +0x429920 sub_429920
    Lifted::new(0x7000429a20, fn_7000429a20 as *const (), 2), // +0x429a20 sub_429a20
    Lifted::new(0x7000429b90, fn_7000429b90 as *const (), 2), // +0x429b90 sub_429b90
    Lifted::new(0x7000429cc0, fn_7000429cc0 as *const (), 0), // +0x429cc0 sub_429cc0
    Lifted::new(0x7000429d70, tpl_405::<0x18, 0x8, 0x20, 0x700042ae90> as *const (), 0), // +0x429d70 sub_429d70
    Lifted::new(0x7000429e20, tpl_405::<0x18, 0x8, 0x20, 0x700042b1d0> as *const (), 0), // +0x429e20 sub_429e20
    Lifted::new(0x7000429ed0, fn_7000429ed0 as *const (), 0), // +0x429ed0 sub_429ed0
    Lifted::new(0x7000429f40, fn_7000429f40 as *const (), 0), // +0x429f40 sub_429f40
    Lifted::new(0x7000429fb0, tpl_158::<0x70001c13d0> as *const (), 2), // +0x429fb0 sub_429fb0
    Lifted::new(0x700042a150, fn_700042a150 as *const (), 2), // +0x42a150 sub_42a150
    Lifted::new(0x700042a420, fn_700042a420 as *const (), 2), // +0x42a420 sub_42a420
    Lifted::new(0x700042a670, fn_700042a670 as *const (), 2), // +0x42a670 sub_42a670
    Lifted::new(0x700042a830, fn_700042a830 as *const (), 2), // +0x42a830 sub_42a830
    Lifted::new(0x700042ab50, fn_700042ab50 as *const (), 2), // +0x42ab50 sub_42ab50
    Lifted::new(0x700042ae90, fn_700042ae90 as *const (), 2), // +0x42ae90 sub_42ae90
    Lifted::new(0x700042b1d0, fn_700042b1d0 as *const (), 2), // +0x42b1d0 sub_42b1d0
    Lifted::new(0x700042b520, tpl_74::<0x10, 0x2, 0x700042b5b0> as *const (), 0), // +0x42b520 sub_42b520
    Lifted::new(0x700042b550, tpl_74::<0x10, 0x2, 0x700042b8e0> as *const (), 0), // +0x42b550 sub_42b550
    Lifted::new(0x700042b580, tpl_74::<0x10, 0x2, 0x700042bd30> as *const (), 0), // +0x42b580 sub_42b580
    Lifted::new(0x700042b5b0, fn_700042b5b0 as *const (), 2), // +0x42b5b0 sub_42b5b0
    Lifted::new(0x700042b8e0, fn_700042b8e0 as *const (), 2), // +0x42b8e0 sub_42b8e0
    Lifted::new(0x700042bc10, tpl_400::<0x700042c070> as *const (), 0), // +0x42bc10 sub_42bc10
    Lifted::new(0x700042bd30, fn_700042bd30 as *const (), 2), // +0x42bd30 sub_42bd30
    Lifted::new(0x700042c070, tpl_404::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x700029c680> as *const (), 2), // +0x42c070 sub_42c070
    Lifted::new(0x700042c1d0, fn_700042c1d0 as *const (), 2), // +0x42c1d0 sub_42c1d0
    Lifted::new(0x700042c3d0, fn_700042c3d0 as *const (), 2), // +0x42c3d0 sub_42c3d0
    Lifted::new(0x700042c5f0, fn_700042c5f0 as *const (), 2), // +0x42c5f0 sub_42c5f0
    Lifted::new(0x700042cd60, fn_700042cd60 as *const (), 0), // +0x42cd60 sub_42cd60
    Lifted::new(0x700042cdf0, tpl_268::<0x18, 0x8, 0x20, 0x700042d890> as *const (), 0), // +0x42cdf0 sub_42cdf0
    Lifted::new(0x700042ce80, tpl_268::<0x18, 0x8, 0x20, 0x700042db80> as *const (), 0), // +0x42ce80 sub_42ce80
    Lifted::new(0x700042cf10, tpl_131::<0x70005a4d38, 0x700062e4c0> as *const (), 2), // +0x42cf10 sub_42cf10
    Lifted::new(0x700042cf90, fn_700042cf90 as *const (), 3), // +0x42cf90 sub_42cf90
    Lifted::new(0x700042d0b0, fn_700042d0b0 as *const (), 3), // +0x42d0b0 sub_42d0b0
    Lifted::new(0x700042d370, fn_700042d370 as *const (), 3), // +0x42d370 sub_42d370
    Lifted::new(0x700042d5a0, fn_700042d5a0 as *const (), 2), // +0x42d5a0 sub_42d5a0
    Lifted::new(0x700042d890, fn_700042d890 as *const (), 2), // +0x42d890 sub_42d890
    Lifted::new(0x700042db80, fn_700042db80 as *const (), 2), // +0x42db80 sub_42db80
    Lifted::new(0x700042de90, tpl_299::<0x7000000000, 0x7000694974, 0x700004f420, 0x70005a4c62, 0x700062e498, 0x700062e490> as *const (), 2), // +0x42de90 sub_42de90
    Lifted::new(0x700042e210, fn_700042e210 as *const (), 3), // +0x42e210 sub_42e210
    Lifted::new(0x700042e420, fn_700042e420 as *const (), 0), // +0x42e420 sub_42e420
    Lifted::new(0x700042e550, fn_700042e550 as *const (), 0), // +0x42e550 sub_42e550
    Lifted::new(0x700042e700, fn_700042e700 as *const (), 1), // +0x42e700 sub_42e700
    Lifted::new(0x700042e7a0, fn_700042e7a0 as *const (), 0), // +0x42e7a0 sub_42e7a0
    Lifted::new(0x700042e850, fn_700042e850 as *const (), 2), // +0x42e850 sub_42e850
    Lifted::new(0x700042ed80, fn_700042ed80 as *const (), 2), // +0x42ed80 sub_42ed80
    Lifted::new(0x700042f010, fn_700042f010 as *const (), 0), // +0x42f010 sub_42f010
    Lifted::new(0x700042f260, fn_700042f260 as *const (), 0), // +0x42f260 sub_42f260
    Lifted::new(0x700042f300, fn_700042f300 as *const (), 0), // +0x42f300 sub_42f300
    Lifted::new(0x700042f7c0, fn_700042f7c0 as *const (), 2), // +0x42f7c0 sub_42f7c0
    Lifted::new(0x700042fd60, fn_700042fd60 as *const (), 0), // +0x42fd60 sub_42fd60
    Lifted::new(0x700042fe10, fn_700042fe10 as *const (), 2), // +0x42fe10 sub_42fe10
    Lifted::new(0x700042ff10, fn_700042ff10 as *const (), 2), // +0x42ff10 sub_42ff10
    Lifted::new(0x7000430420, fn_7000430420 as *const (), 2), // +0x430420 sub_430420
    Lifted::new(0x70004308e0, fn_70004308e0 as *const (), 0), // +0x4308e0 sub_4308e0
    Lifted::new(0x7000430b80, fn_7000430b80 as *const (), 0), // +0x430b80 sub_430b80
    Lifted::new(0x7000430d50, fn_7000430d50 as *const (), 0), // +0x430d50 sub_430d50
    Lifted::new(0x7000430eb0, fn_7000430eb0 as *const (), 2), // +0x430eb0 sub_430eb0
    Lifted::new(0x7000431c60, fn_7000431c60 as *const (), 2), // +0x431c60 sub_431c60
    Lifted::new(0x7000431fa0, fn_7000431fa0 as *const (), 2), // +0x431fa0 sub_431fa0
    Lifted::new(0x7000432d30, fn_7000432d30 as *const (), 0), // +0x432d30 sub_432d30
    Lifted::new(0x7000432f50, fn_7000432f50 as *const (), 0), // +0x432f50 sub_432f50
    Lifted::new(0x7000433170, fn_7000433170 as *const (), 0), // +0x433170 sub_433170
    Lifted::new(0x7000433440, fn_7000433440 as *const (), 0), // +0x433440 sub_433440
    Lifted::new(0x70004336f0, fn_70004336f0 as *const (), 0), // +0x4336f0 sub_4336f0
    Lifted::new(0x7000433820, fn_7000433820 as *const (), 2), // +0x433820 sub_433820
    Lifted::new(0x7000433be0, fn_7000433be0 as *const (), 2), // +0x433be0 sub_433be0
    Lifted::new(0x7000433fb0, fn_7000433fb0 as *const (), 0), // +0x433fb0 sub_433fb0
    Lifted::new(0x70004342a0, fn_70004342a0 as *const (), 2), // +0x4342a0 sub_4342a0
    Lifted::new(0x7000435110, fn_7000435110 as *const (), 0), // +0x435110 sub_435110
    Lifted::new(0x70004358e0, fn_70004358e0 as *const (), 2), // +0x4358e0 sub_4358e0
    Lifted::new(0x7000435dc0, fn_7000435dc0 as *const (), 0), // +0x435dc0 sub_435dc0
    Lifted::new(0x7000435fc0, fn_7000435fc0 as *const (), 0), // +0x435fc0 sub_435fc0
    Lifted::new(0x7000436a20, fn_7000436a20 as *const (), 2), // +0x436a20 sub_436a20
    Lifted::new(0x7000438710, fn_7000438710 as *const (), 0), // +0x438710 sub_438710
    Lifted::new(0x7000439280, fn_7000439280 as *const (), 0), // +0x439280 sub_439280
    Lifted::new(0x7000439410, fn_7000439410 as *const (), 0), // +0x439410 sub_439410
    Lifted::new(0x7000439510, fn_7000439510 as *const (), 2), // +0x439510 sub_439510
    Lifted::new(0x7000439d90, fn_7000439d90 as *const (), 0), // +0x439d90 sub_439d90
    Lifted::new(0x700043a070, fn_700043a070 as *const (), 0), // +0x43a070 sub_43a070
    Lifted::new(0x700043adb0, fn_700043adb0 as *const (), 2), // +0x43adb0 sub_43adb0
    Lifted::new(0x700043b010, fn_700043b010 as *const (), 2), // +0x43b010 sub_43b010
    Lifted::new(0x700043b440, fn_700043b440 as *const (), 0), // +0x43b440 sub_43b440
    Lifted::new(0x700043b550, fn_700043b550 as *const (), 0), // +0x43b550 sub_43b550
    Lifted::new(0x700043b830, fn_700043b830 as *const (), 0), // +0x43b830 sub_43b830
    Lifted::new(0x700043b920, fn_700043b920 as *const (), 0), // +0x43b920 sub_43b920
    Lifted::new(0x700043ba20, fn_700043ba20 as *const (), 0), // +0x43ba20 sub_43ba20
    Lifted::new(0x700043bb20, fn_700043bb20 as *const (), 2), // +0x43bb20 sub_43bb20
    Lifted::new(0x700043be00, fn_700043be00 as *const (), 0), // +0x43be00 sub_43be00
    Lifted::new(0x700043c070, fn_700043c070 as *const (), 0), // +0x43c070 sub_43c070
    Lifted::new(0x700043c2e0, fn_700043c2e0 as *const (), 0), // +0x43c2e0 sub_43c2e0
    Lifted::new(0x700043c560, fn_700043c560 as *const (), 0), // +0x43c560 sub_43c560
    Lifted::new(0x700043c660, fn_700043c660 as *const (), 2), // +0x43c660 sub_43c660
    Lifted::new(0x700043c870, fn_700043c870 as *const (), 0), // +0x43c870 sub_43c870
    Lifted::new(0x700043ca50, fn_700043ca50 as *const (), 0), // +0x43ca50 sub_43ca50
    Lifted::new(0x700043cb30, fn_700043cb30 as *const (), 0), // +0x43cb30 sub_43cb30
    Lifted::new(0x700043cc10, fn_700043cc10 as *const (), 2), // +0x43cc10 sub_43cc10
    Lifted::new(0x700043cec0, fn_700043cec0 as *const (), 2), // +0x43cec0 sub_43cec0
    Lifted::new(0x700043d170, fn_700043d170 as *const (), 2), // +0x43d170 sub_43d170
    Lifted::new(0x700043d420, fn_700043d420 as *const (), 2), // +0x43d420 sub_43d420
    Lifted::new(0x700043d6b0, fn_700043d6b0 as *const (), 2), // +0x43d6b0 sub_43d6b0
    Lifted::new(0x700043d9d0, fn_700043d9d0 as *const (), 2), // +0x43d9d0 sub_43d9d0
    Lifted::new(0x700043dcf0, fn_700043dcf0 as *const (), 2), // +0x43dcf0 sub_43dcf0
    Lifted::new(0x700043e010, fn_700043e010 as *const (), 2), // +0x43e010 sub_43e010
    Lifted::new(0x700043e330, fn_700043e330 as *const (), 2), // +0x43e330 sub_43e330
    Lifted::new(0x700043e440, fn_700043e440 as *const (), 2), // +0x43e440 sub_43e440
    Lifted::new(0x700043e550, fn_700043e550 as *const (), 2), // +0x43e550 sub_43e550
    Lifted::new(0x700043e660, fn_700043e660 as *const (), 0), // +0x43e660 sub_43e660
    Lifted::new(0x700043e750, fn_700043e750 as *const (), 0), // +0x43e750 sub_43e750
    Lifted::new(0x700043e9a0, fn_700043e9a0 as *const (), 0), // +0x43e9a0 sub_43e9a0
    Lifted::new(0x700043ec00, fn_700043ec00 as *const (), 0), // +0x43ec00 sub_43ec00
    Lifted::new(0x700043ee70, fn_700043ee70 as *const (), 2), // +0x43ee70 sub_43ee70
    Lifted::new(0x700043f060, fn_700043f060 as *const (), 2), // +0x43f060 sub_43f060
    Lifted::new(0x700043f260, fn_700043f260 as *const (), 0), // +0x43f260 sub_43f260
    Lifted::new(0x700043f340, fn_700043f340 as *const (), 0), // +0x43f340 sub_43f340
    Lifted::new(0x700043f420, fn_700043f420 as *const (), 0), // +0x43f420 sub_43f420
    Lifted::new(0x700043f510, fn_700043f510 as *const (), 0), // +0x43f510 sub_43f510
    Lifted::new(0x700043f790, fn_700043f790 as *const (), 0), // +0x43f790 sub_43f790
    Lifted::new(0x700043fa10, fn_700043fa10 as *const (), 2), // +0x43fa10 sub_43fa10
    Lifted::new(0x700043fb20, fn_700043fb20 as *const (), 0), // +0x43fb20 sub_43fb20
    Lifted::new(0x700043fc10, fn_700043fc10 as *const (), 0), // +0x43fc10 sub_43fc10
    Lifted::new(0x700043fd00, fn_700043fd00 as *const (), 0), // +0x43fd00 sub_43fd00
    Lifted::new(0x700043fe00, fn_700043fe00 as *const (), 0), // +0x43fe00 sub_43fe00
    Lifted::new(0x700043fef0, fn_700043fef0 as *const (), 0), // +0x43fef0 sub_43fef0
    Lifted::new(0x7000440000, fn_7000440000 as *const (), 2), // +0x440000 sub_440000
    Lifted::new(0x7000440210, fn_7000440210 as *const (), 2), // +0x440210 sub_440210
    Lifted::new(0x7000440420, fn_7000440420 as *const (), 2), // +0x440420 sub_440420
    Lifted::new(0x7000440630, fn_7000440630 as *const (), 2), // +0x440630 sub_440630
    Lifted::new(0x7000440840, fn_7000440840 as *const (), 2), // +0x440840 sub_440840
    Lifted::new(0x7000440930, fn_7000440930 as *const (), 2), // +0x440930 sub_440930
    Lifted::new(0x7000440a20, fn_7000440a20 as *const (), 2), // +0x440a20 sub_440a20
    Lifted::new(0x7000440b10, fn_7000440b10 as *const (), 2), // +0x440b10 sub_440b10
    Lifted::new(0x7000440c00, fn_7000440c00 as *const (), 2), // +0x440c00 sub_440c00
    Lifted::new(0x7000440cf0, fn_7000440cf0 as *const (), 2), // +0x440cf0 sub_440cf0
    Lifted::new(0x7000440de0, fn_7000440de0 as *const (), 2), // +0x440de0 sub_440de0
    Lifted::new(0x7000440ed0, fn_7000440ed0 as *const (), 2), // +0x440ed0 sub_440ed0
    Lifted::new(0x7000440fc0, fn_7000440fc0 as *const (), 0), // +0x440fc0 sub_440fc0
    Lifted::new(0x70004410c0, fn_70004410c0 as *const (), 0), // +0x4410c0 sub_4410c0
    Lifted::new(0x70004411c0, fn_70004411c0 as *const (), 0), // +0x4411c0 sub_4411c0
    Lifted::new(0x70004412d0, fn_70004412d0 as *const (), 0), // +0x4412d0 sub_4412d0
    Lifted::new(0x70004413c0, fn_70004413c0 as *const (), 0), // +0x4413c0 sub_4413c0
    Lifted::new(0x70004414c0, fn_70004414c0 as *const (), 0), // +0x4414c0 sub_4414c0
    Lifted::new(0x70004416a0, fn_70004416a0 as *const (), 2), // +0x4416a0 sub_4416a0
    Lifted::new(0x7000441970, fn_7000441970 as *const (), 2), // +0x441970 sub_441970
    Lifted::new(0x7000441c00, fn_7000441c00 as *const (), 0), // +0x441c00 sub_441c00
    Lifted::new(0x7000441cf0, fn_7000441cf0 as *const (), 2), // +0x441cf0 sub_441cf0
    Lifted::new(0x7000441f90, fn_7000441f90 as *const (), 0), // +0x441f90 sub_441f90
    Lifted::new(0x7000442180, fn_7000442180 as *const (), 0), // +0x442180 sub_442180
    Lifted::new(0x7000442260, fn_7000442260 as *const (), 0), // +0x442260 sub_442260
    Lifted::new(0x7000442340, fn_7000442340 as *const (), 0), // +0x442340 sub_442340
    Lifted::new(0x7000442420, fn_7000442420 as *const (), 0), // +0x442420 sub_442420
    Lifted::new(0x7000442500, fn_7000442500 as *const (), 0), // +0x442500 sub_442500
    Lifted::new(0x70004425e0, fn_70004425e0 as *const (), 0), // +0x4425e0 sub_4425e0
    Lifted::new(0x70004426c0, fn_70004426c0 as *const (), 0), // +0x4426c0 sub_4426c0
    Lifted::new(0x70004427a0, fn_70004427a0 as *const (), 0), // +0x4427a0 sub_4427a0
    Lifted::new(0x7000442880, fn_7000442880 as *const (), 0), // +0x442880 sub_442880
    Lifted::new(0x7000442970, fn_7000442970 as *const (), 0), // +0x442970 sub_442970
    Lifted::new(0x7000442a60, fn_7000442a60 as *const (), 2), // +0x442a60 sub_442a60
    Lifted::new(0x7000442b90, fn_7000442b90 as *const (), 2), // +0x442b90 sub_442b90
    Lifted::new(0x7000442e10, fn_7000442e10 as *const (), 2), // +0x442e10 sub_442e10
    Lifted::new(0x7000443090, fn_7000443090 as *const (), 2), // +0x443090 sub_443090
    Lifted::new(0x7000443310, fn_7000443310 as *const (), 2), // +0x443310 sub_443310
    Lifted::new(0x7000443590, fn_7000443590 as *const (), 2), // +0x443590 sub_443590
    Lifted::new(0x70004437c0, fn_70004437c0 as *const (), 2), // +0x4437c0 sub_4437c0
    Lifted::new(0x70004439f0, fn_70004439f0 as *const (), 2), // +0x4439f0 sub_4439f0
    Lifted::new(0x7000443c20, fn_7000443c20 as *const (), 2), // +0x443c20 sub_443c20
    Lifted::new(0x7000443e40, fn_7000443e40 as *const (), 2), // +0x443e40 sub_443e40
    Lifted::new(0x70004440a0, fn_70004440a0 as *const (), 2), // +0x4440a0 sub_4440a0
    Lifted::new(0x7000444320, fn_7000444320 as *const (), 2), // +0x444320 sub_444320
    Lifted::new(0x70004445a0, fn_70004445a0 as *const (), 2), // +0x4445a0 sub_4445a0
    Lifted::new(0x7000444820, fn_7000444820 as *const (), 0), // +0x444820 sub_444820
    Lifted::new(0x7000444910, fn_7000444910 as *const (), 0), // +0x444910 sub_444910
    Lifted::new(0x7000444a00, fn_7000444a00 as *const (), 0), // +0x444a00 sub_444a00
    Lifted::new(0x7000444af0, fn_7000444af0 as *const (), 0), // +0x444af0 sub_444af0
    Lifted::new(0x7000444be0, fn_7000444be0 as *const (), 2), // +0x444be0 sub_444be0
    Lifted::new(0x7000444e30, fn_7000444e30 as *const (), 2), // +0x444e30 sub_444e30
    Lifted::new(0x7000445080, fn_7000445080 as *const (), 2), // +0x445080 sub_445080
    Lifted::new(0x7000445310, fn_7000445310 as *const (), 2), // +0x445310 sub_445310
    Lifted::new(0x70004455a0, fn_70004455a0 as *const (), 2), // +0x4455a0 sub_4455a0
    Lifted::new(0x7000445830, fn_7000445830 as *const (), 2), // +0x445830 sub_445830
    Lifted::new(0x7000445ac0, fn_7000445ac0 as *const (), 2), // +0x445ac0 sub_445ac0
    Lifted::new(0x7000445d50, fn_7000445d50 as *const (), 2), // +0x445d50 sub_445d50
    Lifted::new(0x7000445fe0, fn_7000445fe0 as *const (), 2), // +0x445fe0 sub_445fe0
    Lifted::new(0x7000446270, fn_7000446270 as *const (), 2), // +0x446270 sub_446270
    Lifted::new(0x7000446500, fn_7000446500 as *const (), 0), // +0x446500 sub_446500
    Lifted::new(0x70004465f0, fn_70004465f0 as *const (), 0), // +0x4465f0 sub_4465f0
    Lifted::new(0x70004466f0, tpl_406::<0x8, 0x7f, 0x24, 0x1, 0x7000450fb0, 0x3, 0x7000691780, 0x20, 0x70001e0c50, 0x70003f04f0, 0x10, 0x70001dfae0> as *const (), 0), // +0x4466f0 sub_4466f0
    Lifted::new(0x70004467c0, fn_70004467c0 as *const (), 0), // +0x4467c0 sub_4467c0
    Lifted::new(0x7000446890, fn_7000446890 as *const (), 0), // +0x446890 sub_446890
    Lifted::new(0x7000446960, fn_7000446960 as *const (), 0), // +0x446960 sub_446960
    Lifted::new(0x7000446a30, fn_7000446a30 as *const (), 0), // +0x446a30 sub_446a30
    Lifted::new(0x7000446b00, fn_7000446b00 as *const (), 0), // +0x446b00 sub_446b00
    Lifted::new(0x7000446bd0, fn_7000446bd0 as *const (), 0), // +0x446bd0 sub_446bd0
    Lifted::new(0x7000446ca0, fn_7000446ca0 as *const (), 0), // +0x446ca0 sub_446ca0
    Lifted::new(0x7000446d70, tpl_406::<0x8, 0x87, 0x24, 0x1, 0x7000451230, 0x3, 0x7000691808, 0x28, 0x70001e0890, 0x70003f0790, 0x10, 0x70001dfd60> as *const (), 0), // +0x446d70 sub_446d70
    Lifted::new(0x7000446e40, fn_7000446e40 as *const (), 0), // +0x446e40 sub_446e40
    Lifted::new(0x7000446f10, fn_7000446f10 as *const (), 0), // +0x446f10 sub_446f10
    Lifted::new(0x7000446fe0, fn_7000446fe0 as *const (), 0), // +0x446fe0 sub_446fe0
    Lifted::new(0x70004470b0, fn_70004470b0 as *const (), 2), // +0x4470b0 sub_4470b0
    Lifted::new(0x7000447380, fn_7000447380 as *const (), 0), // +0x447380 sub_447380
    Lifted::new(0x7000447480, fn_7000447480 as *const (), 0), // +0x447480 sub_447480
    Lifted::new(0x7000447570, fn_7000447570 as *const (), 0), // +0x447570 sub_447570
    Lifted::new(0x7000447660, fn_7000447660 as *const (), 2), // +0x447660 sub_447660
    Lifted::new(0x70004478f0, fn_70004478f0 as *const (), 2), // +0x4478f0 sub_4478f0
    Lifted::new(0x7000447b80, fn_7000447b80 as *const (), 0), // +0x447b80 sub_447b80
    Lifted::new(0x7000447c60, fn_7000447c60 as *const (), 0), // +0x447c60 sub_447c60
    Lifted::new(0x7000447d40, fn_7000447d40 as *const (), 0), // +0x447d40 sub_447d40
    Lifted::new(0x7000447e30, fn_7000447e30 as *const (), 0), // +0x447e30 sub_447e30
    Lifted::new(0x7000447f20, fn_7000447f20 as *const (), 2), // +0x447f20 sub_447f20
    Lifted::new(0x7000448150, fn_7000448150 as *const (), 2), // +0x448150 sub_448150
    Lifted::new(0x7000448430, fn_7000448430 as *const (), 2), // +0x448430 sub_448430
    Lifted::new(0x7000448740, fn_7000448740 as *const (), 0), // +0x448740 sub_448740
    Lifted::new(0x7000448820, fn_7000448820 as *const (), 2), // +0x448820 sub_448820
    Lifted::new(0x7000448930, fn_7000448930 as *const (), 0), // +0x448930 sub_448930
    Lifted::new(0x7000448a20, fn_7000448a20 as *const (), 0), // +0x448a20 sub_448a20
    Lifted::new(0x7000448b10, fn_7000448b10 as *const (), 2), // +0x448b10 sub_448b10
    Lifted::new(0x7000448d20, fn_7000448d20 as *const (), 0), // +0x448d20 sub_448d20
    Lifted::new(0x7000448f90, fn_7000448f90 as *const (), 2), // +0x448f90 sub_448f90
    Lifted::new(0x70004492d0, fn_70004492d0 as *const (), 0), // +0x4492d0 sub_4492d0
    Lifted::new(0x70004494e0, fn_70004494e0 as *const (), 2), // +0x4494e0 sub_4494e0
    Lifted::new(0x7000449600, fn_7000449600 as *const (), 0), // +0x449600 sub_449600
    Lifted::new(0x70004496f0, fn_70004496f0 as *const (), 2), // +0x4496f0 sub_4496f0
    Lifted::new(0x7000449800, fn_7000449800 as *const (), 2), // +0x449800 sub_449800
    Lifted::new(0x7000449aa0, fn_7000449aa0 as *const (), 2), // +0x449aa0 sub_449aa0
    Lifted::new(0x7000449cb0, fn_7000449cb0 as *const (), 2), // +0x449cb0 sub_449cb0
    Lifted::new(0x7000449f50, fn_7000449f50 as *const (), 2), // +0x449f50 sub_449f50
    Lifted::new(0x700044a160, fn_700044a160 as *const (), 2), // +0x44a160 sub_44a160
    Lifted::new(0x700044a400, fn_700044a400 as *const (), 2), // +0x44a400 sub_44a400
    Lifted::new(0x700044a610, fn_700044a610 as *const (), 2), // +0x44a610 sub_44a610
    Lifted::new(0x700044a840, fn_700044a840 as *const (), 0), // +0x44a840 sub_44a840
    Lifted::new(0x700044a930, fn_700044a930 as *const (), 0), // +0x44a930 sub_44a930
    Lifted::new(0x700044aa20, fn_700044aa20 as *const (), 2), // +0x44aa20 sub_44aa20
    Lifted::new(0x700044ab10, fn_700044ab10 as *const (), 0), // +0x44ab10 sub_44ab10
    Lifted::new(0x700044ac00, fn_700044ac00 as *const (), 2), // +0x44ac00 sub_44ac00
    Lifted::new(0x700044ae10, fn_700044ae10 as *const (), 2), // +0x44ae10 sub_44ae10
    Lifted::new(0x700044b150, fn_700044b150 as *const (), 0), // +0x44b150 sub_44b150
    Lifted::new(0x700044b240, fn_700044b240 as *const (), 0), // +0x44b240 sub_44b240
    Lifted::new(0x700044b330, fn_700044b330 as *const (), 2), // +0x44b330 sub_44b330
    Lifted::new(0x700044b560, fn_700044b560 as *const (), 2), // +0x44b560 sub_44b560
    Lifted::new(0x700044b790, fn_700044b790 as *const (), 2), // +0x44b790 sub_44b790
    Lifted::new(0x700044b9c0, fn_700044b9c0 as *const (), 2), // +0x44b9c0 sub_44b9c0
    Lifted::new(0x700044bbf0, fn_700044bbf0 as *const (), 2), // +0x44bbf0 sub_44bbf0
    Lifted::new(0x700044be20, fn_700044be20 as *const (), 2), // +0x44be20 sub_44be20
    Lifted::new(0x700044c050, fn_700044c050 as *const (), 2), // +0x44c050 sub_44c050
    Lifted::new(0x700044c270, fn_700044c270 as *const (), 2), // +0x44c270 sub_44c270
    Lifted::new(0x700044c4a0, fn_700044c4a0 as *const (), 2), // +0x44c4a0 sub_44c4a0
    Lifted::new(0x700044c6d0, fn_700044c6d0 as *const (), 2), // +0x44c6d0 sub_44c6d0
    Lifted::new(0x700044c900, fn_700044c900 as *const (), 2), // +0x44c900 sub_44c900
    Lifted::new(0x700044cb30, fn_700044cb30 as *const (), 2), // +0x44cb30 sub_44cb30
    Lifted::new(0x700044cd00, fn_700044cd00 as *const (), 0), // +0x44cd00 sub_44cd00
    Lifted::new(0x700044ce20, fn_700044ce20 as *const (), 2), // +0x44ce20 sub_44ce20
    Lifted::new(0x700044d060, fn_700044d060 as *const (), 2), // +0x44d060 sub_44d060
    Lifted::new(0x700044d280, fn_700044d280 as *const (), 0), // +0x44d280 sub_44d280
    Lifted::new(0x700044d380, fn_700044d380 as *const (), 0), // +0x44d380 sub_44d380
    Lifted::new(0x700044d470, fn_700044d470 as *const (), 2), // +0x44d470 sub_44d470
    Lifted::new(0x700044d6a0, fn_700044d6a0 as *const (), 2), // +0x44d6a0 sub_44d6a0
    Lifted::new(0x700044d980, fn_700044d980 as *const (), 2), // +0x44d980 sub_44d980
    Lifted::new(0x700044dba0, fn_700044dba0 as *const (), 0), // +0x44dba0 sub_44dba0
    Lifted::new(0x700044dd60, fn_700044dd60 as *const (), 2), // +0x44dd60 sub_44dd60
    Lifted::new(0x700044dea0, fn_700044dea0 as *const (), 2), // +0x44dea0 sub_44dea0
    Lifted::new(0x700044e0d0, fn_700044e0d0 as *const (), 0), // +0x44e0d0 sub_44e0d0
    Lifted::new(0x700044e1c0, fn_700044e1c0 as *const (), 0), // +0x44e1c0 sub_44e1c0
    Lifted::new(0x700044e2b0, fn_700044e2b0 as *const (), 0), // +0x44e2b0 sub_44e2b0
    Lifted::new(0x700044e3a0, fn_700044e3a0 as *const (), 0), // +0x44e3a0 sub_44e3a0
    Lifted::new(0x700044e590, fn_700044e590 as *const (), 0), // +0x44e590 sub_44e590
    Lifted::new(0x700044e780, fn_700044e780 as *const (), 0), // +0x44e780 sub_44e780
    Lifted::new(0x700044eac0, fn_700044eac0 as *const (), 1), // +0x44eac0 sub_44eac0
    Lifted::new(0x700044ebc0, fn_700044ebc0 as *const (), 0), // +0x44ebc0 sub_44ebc0
    Lifted::new(0x700044ec10, fn_700044ec10 as *const (), 0), // +0x44ec10 sub_44ec10
    Lifted::new(0x700044ec60, fn_700044ec60 as *const (), 0), // +0x44ec60 sub_44ec60
    Lifted::new(0x700044ecb0, fn_700044ecb0 as *const (), 0), // +0x44ecb0 sub_44ecb0
    Lifted::new(0x700044ed00, fn_700044ed00 as *const (), 0), // +0x44ed00 sub_44ed00
    Lifted::new(0x700044ed50, fn_700044ed50 as *const (), 0), // +0x44ed50 sub_44ed50
    Lifted::new(0x700044eda0, fn_700044eda0 as *const (), 0), // +0x44eda0 sub_44eda0
    Lifted::new(0x700044edf0, fn_700044edf0 as *const (), 0), // +0x44edf0 sub_44edf0
    Lifted::new(0x700044ee40, fn_700044ee40 as *const (), 0), // +0x44ee40 sub_44ee40
    Lifted::new(0x700044ee90, fn_700044ee90 as *const (), 0), // +0x44ee90 sub_44ee90
    Lifted::new(0x700044eee0, fn_700044eee0 as *const (), 0), // +0x44eee0 sub_44eee0
    Lifted::new(0x700044ef30, fn_700044ef30 as *const (), 0), // +0x44ef30 sub_44ef30
    Lifted::new(0x700044ef80, fn_700044ef80 as *const (), 0), // +0x44ef80 sub_44ef80
    Lifted::new(0x700044efd0, fn_700044efd0 as *const (), 0), // +0x44efd0 sub_44efd0
    Lifted::new(0x700044f020, fn_700044f020 as *const (), 0), // +0x44f020 sub_44f020
    Lifted::new(0x700044f070, fn_700044f070 as *const (), 0), // +0x44f070 sub_44f070
    Lifted::new(0x700044f0c0, fn_700044f0c0 as *const (), 0), // +0x44f0c0 sub_44f0c0
    Lifted::new(0x700044f110, fn_700044f110 as *const (), 0), // +0x44f110 sub_44f110
    Lifted::new(0x700044f160, fn_700044f160 as *const (), 0), // +0x44f160 sub_44f160
    Lifted::new(0x700044f1b0, fn_700044f1b0 as *const (), 0), // +0x44f1b0 sub_44f1b0
    Lifted::new(0x700044f200, fn_700044f200 as *const (), 0), // +0x44f200 sub_44f200
    Lifted::new(0x700044f250, fn_700044f250 as *const (), 0), // +0x44f250 sub_44f250
    Lifted::new(0x700044f2a0, fn_700044f2a0 as *const (), 0), // +0x44f2a0 sub_44f2a0
    Lifted::new(0x700044f2f0, fn_700044f2f0 as *const (), 0), // +0x44f2f0 sub_44f2f0
    Lifted::new(0x700044f340, fn_700044f340 as *const (), 0), // +0x44f340 sub_44f340
    Lifted::new(0x700044f390, fn_700044f390 as *const (), 0), // +0x44f390 sub_44f390
    Lifted::new(0x700044f3e0, fn_700044f3e0 as *const (), 0), // +0x44f3e0 sub_44f3e0
    Lifted::new(0x700044f430, fn_700044f430 as *const (), 0), // +0x44f430 sub_44f430
    Lifted::new(0x700044f480, fn_700044f480 as *const (), 3), // +0x44f480 sub_44f480
    Lifted::new(0x700044f660, fn_700044f660 as *const (), 0), // +0x44f660 sub_44f660
    Lifted::new(0x700044f6b0, fn_700044f6b0 as *const (), 3), // +0x44f6b0 sub_44f6b0
    Lifted::new(0x700044f8e0, fn_700044f8e0 as *const (), 0), // +0x44f8e0 sub_44f8e0
    Lifted::new(0x700044f930, fn_700044f930 as *const (), 0), // +0x44f930 sub_44f930
    Lifted::new(0x700044f980, fn_700044f980 as *const (), 0), // +0x44f980 sub_44f980
    Lifted::new(0x700044f9d0, fn_700044f9d0 as *const (), 0), // +0x44f9d0 sub_44f9d0
    Lifted::new(0x700044fa20, fn_700044fa20 as *const (), 0), // +0x44fa20 sub_44fa20
    Lifted::new(0x700044fa70, fn_700044fa70 as *const (), 0), // +0x44fa70 sub_44fa70
    Lifted::new(0x700044fac0, fn_700044fac0 as *const (), 0), // +0x44fac0 sub_44fac0
    Lifted::new(0x700044fb10, fn_700044fb10 as *const (), 0), // +0x44fb10 sub_44fb10
    Lifted::new(0x700044fb60, fn_700044fb60 as *const (), 0), // +0x44fb60 sub_44fb60
    Lifted::new(0x700044fbb0, fn_700044fbb0 as *const (), 0), // +0x44fbb0 sub_44fbb0
    Lifted::new(0x700044fc00, fn_700044fc00 as *const (), 0), // +0x44fc00 sub_44fc00
    Lifted::new(0x700044fc50, fn_700044fc50 as *const (), 0), // +0x44fc50 sub_44fc50
    Lifted::new(0x700044fca0, fn_700044fca0 as *const (), 0), // +0x44fca0 sub_44fca0
    Lifted::new(0x700044fcf0, fn_700044fcf0 as *const (), 0), // +0x44fcf0 sub_44fcf0
    Lifted::new(0x700044fd40, fn_700044fd40 as *const (), 0), // +0x44fd40 sub_44fd40
    Lifted::new(0x700044fd90, fn_700044fd90 as *const (), 0), // +0x44fd90 sub_44fd90
    Lifted::new(0x700044fde0, fn_700044fde0 as *const (), 0), // +0x44fde0 sub_44fde0
    Lifted::new(0x700044fe30, fn_700044fe30 as *const (), 0), // +0x44fe30 sub_44fe30
    Lifted::new(0x700044fe80, fn_700044fe80 as *const (), 0), // +0x44fe80 sub_44fe80
    Lifted::new(0x700044fed0, fn_700044fed0 as *const (), 0), // +0x44fed0 sub_44fed0
    Lifted::new(0x700044ff20, fn_700044ff20 as *const (), 0), // +0x44ff20 sub_44ff20
    Lifted::new(0x700044ff70, fn_700044ff70 as *const (), 0), // +0x44ff70 sub_44ff70
    Lifted::new(0x700044ffc0, fn_700044ffc0 as *const (), 0), // +0x44ffc0 sub_44ffc0
    Lifted::new(0x7000450010, fn_7000450010 as *const (), 0), // +0x450010 sub_450010
    Lifted::new(0x7000450060, fn_7000450060 as *const (), 0), // +0x450060 sub_450060
    Lifted::new(0x70004500b0, fn_70004500b0 as *const (), 0), // +0x4500b0 sub_4500b0
    Lifted::new(0x7000450100, fn_7000450100 as *const (), 0), // +0x450100 sub_450100
    Lifted::new(0x7000450150, fn_7000450150 as *const (), 0), // +0x450150 sub_450150
    Lifted::new(0x70004501a0, fn_70004501a0 as *const (), 0), // +0x4501a0 sub_4501a0
    Lifted::new(0x70004501f0, fn_70004501f0 as *const (), 0), // +0x4501f0 sub_4501f0
    Lifted::new(0x7000450240, fn_7000450240 as *const (), 0), // +0x450240 sub_450240
    Lifted::new(0x7000450290, fn_7000450290 as *const (), 0), // +0x450290 sub_450290
    Lifted::new(0x70004502e0, fn_70004502e0 as *const (), 0), // +0x4502e0 sub_4502e0
    Lifted::new(0x7000450330, fn_7000450330 as *const (), 0), // +0x450330 sub_450330
    Lifted::new(0x7000450380, fn_7000450380 as *const (), 0), // +0x450380 sub_450380
    Lifted::new(0x70004503d0, fn_70004503d0 as *const (), 0), // +0x4503d0 sub_4503d0
    Lifted::new(0x7000450420, fn_7000450420 as *const (), 0), // +0x450420 sub_450420
    Lifted::new(0x7000450470, fn_7000450470 as *const (), 0), // +0x450470 sub_450470
    Lifted::new(0x70004504c0, fn_70004504c0 as *const (), 0), // +0x4504c0 sub_4504c0
    Lifted::new(0x7000450510, fn_7000450510 as *const (), 0), // +0x450510 sub_450510
    Lifted::new(0x7000450560, fn_7000450560 as *const (), 0), // +0x450560 sub_450560
    Lifted::new(0x70004505b0, fn_70004505b0 as *const (), 0), // +0x4505b0 sub_4505b0
    Lifted::new(0x7000450600, fn_7000450600 as *const (), 0), // +0x450600 sub_450600
    Lifted::new(0x7000450650, fn_7000450650 as *const (), 0), // +0x450650 sub_450650
    Lifted::new(0x70004506a0, fn_70004506a0 as *const (), 0), // +0x4506a0 sub_4506a0
    Lifted::new(0x70004506f0, fn_70004506f0 as *const (), 0), // +0x4506f0 sub_4506f0
    Lifted::new(0x7000450740, fn_7000450740 as *const (), 0), // +0x450740 sub_450740
    Lifted::new(0x7000450790, fn_7000450790 as *const (), 0), // +0x450790 sub_450790
    Lifted::new(0x70004507e0, fn_70004507e0 as *const (), 0), // +0x4507e0 sub_4507e0
    Lifted::new(0x7000450830, fn_7000450830 as *const (), 0), // +0x450830 sub_450830
    Lifted::new(0x7000450880, fn_7000450880 as *const (), 0), // +0x450880 sub_450880
    Lifted::new(0x70004508d0, fn_70004508d0 as *const (), 0), // +0x4508d0 sub_4508d0
    Lifted::new(0x7000450920, fn_7000450920 as *const (), 0), // +0x450920 sub_450920
    Lifted::new(0x7000450970, fn_7000450970 as *const (), 0), // +0x450970 sub_450970
    Lifted::new(0x70004509c0, fn_70004509c0 as *const (), 0), // +0x4509c0 sub_4509c0
    Lifted::new(0x7000450a10, fn_7000450a10 as *const (), 0), // +0x450a10 sub_450a10
    Lifted::new(0x7000450a60, fn_7000450a60 as *const (), 0), // +0x450a60 sub_450a60
    Lifted::new(0x7000450ab0, fn_7000450ab0 as *const (), 0), // +0x450ab0 sub_450ab0
    Lifted::new(0x7000450b00, fn_7000450b00 as *const (), 0), // +0x450b00 sub_450b00
    Lifted::new(0x7000450b50, fn_7000450b50 as *const (), 0), // +0x450b50 sub_450b50
    Lifted::new(0x7000450ba0, fn_7000450ba0 as *const (), 0), // +0x450ba0 sub_450ba0
    Lifted::new(0x7000450bf0, fn_7000450bf0 as *const (), 0), // +0x450bf0 sub_450bf0
    Lifted::new(0x7000450c40, fn_7000450c40 as *const (), 0), // +0x450c40 sub_450c40
    Lifted::new(0x7000450c90, fn_7000450c90 as *const (), 0), // +0x450c90 sub_450c90
    Lifted::new(0x7000450ce0, fn_7000450ce0 as *const (), 0), // +0x450ce0 sub_450ce0
    Lifted::new(0x7000450d30, fn_7000450d30 as *const (), 0), // +0x450d30 sub_450d30
    Lifted::new(0x7000450d80, fn_7000450d80 as *const (), 0), // +0x450d80 sub_450d80
    Lifted::new(0x7000450dd0, fn_7000450dd0 as *const (), 0), // +0x450dd0 sub_450dd0
    Lifted::new(0x7000450e20, fn_7000450e20 as *const (), 0), // +0x450e20 sub_450e20
    Lifted::new(0x7000450e70, fn_7000450e70 as *const (), 0), // +0x450e70 sub_450e70
    Lifted::new(0x7000450ec0, fn_7000450ec0 as *const (), 0), // +0x450ec0 sub_450ec0
    Lifted::new(0x7000450f10, fn_7000450f10 as *const (), 0), // +0x450f10 sub_450f10
    Lifted::new(0x7000450f60, fn_7000450f60 as *const (), 0), // +0x450f60 sub_450f60
    Lifted::new(0x7000450fb0, fn_7000450fb0 as *const (), 0), // +0x450fb0 sub_450fb0
    Lifted::new(0x7000451000, fn_7000451000 as *const (), 0), // +0x451000 sub_451000
    Lifted::new(0x7000451050, fn_7000451050 as *const (), 0), // +0x451050 sub_451050
    Lifted::new(0x70004510a0, fn_70004510a0 as *const (), 0), // +0x4510a0 sub_4510a0
    Lifted::new(0x70004510f0, fn_70004510f0 as *const (), 0), // +0x4510f0 sub_4510f0
    Lifted::new(0x7000451140, fn_7000451140 as *const (), 0), // +0x451140 sub_451140
    Lifted::new(0x7000451190, fn_7000451190 as *const (), 0), // +0x451190 sub_451190
    Lifted::new(0x70004511e0, fn_70004511e0 as *const (), 0), // +0x4511e0 sub_4511e0
    Lifted::new(0x7000451230, fn_7000451230 as *const (), 0), // +0x451230 sub_451230
    Lifted::new(0x7000451280, fn_7000451280 as *const (), 0), // +0x451280 sub_451280
    Lifted::new(0x70004512d0, fn_70004512d0 as *const (), 0), // +0x4512d0 sub_4512d0
    Lifted::new(0x7000451320, fn_7000451320 as *const (), 0), // +0x451320 sub_451320
    Lifted::new(0x7000451370, fn_7000451370 as *const (), 0), // +0x451370 sub_451370
    Lifted::new(0x70004513c0, fn_70004513c0 as *const (), 0), // +0x4513c0 sub_4513c0
    Lifted::new(0x7000451410, fn_7000451410 as *const (), 0), // +0x451410 sub_451410
    Lifted::new(0x7000451460, fn_7000451460 as *const (), 0), // +0x451460 sub_451460
    Lifted::new(0x70004514b0, fn_70004514b0 as *const (), 0), // +0x4514b0 sub_4514b0
    Lifted::new(0x7000451500, fn_7000451500 as *const (), 0), // +0x451500 sub_451500
    Lifted::new(0x7000451550, fn_7000451550 as *const (), 0), // +0x451550 sub_451550
    Lifted::new(0x70004515a0, fn_70004515a0 as *const (), 0), // +0x4515a0 sub_4515a0
    Lifted::new(0x70004515f0, fn_70004515f0 as *const (), 0), // +0x4515f0 sub_4515f0
    Lifted::new(0x7000451640, fn_7000451640 as *const (), 0), // +0x451640 sub_451640
    Lifted::new(0x7000451690, fn_7000451690 as *const (), 0), // +0x451690 sub_451690
    Lifted::new(0x70004516e0, fn_70004516e0 as *const (), 0), // +0x4516e0 sub_4516e0
    Lifted::new(0x7000451730, fn_7000451730 as *const (), 0), // +0x451730 sub_451730
    Lifted::new(0x7000451780, fn_7000451780 as *const (), 0), // +0x451780 sub_451780
    Lifted::new(0x70004517d0, fn_70004517d0 as *const (), 0), // +0x4517d0 sub_4517d0
    Lifted::new(0x7000451820, fn_7000451820 as *const (), 0), // +0x451820 sub_451820
    Lifted::new(0x7000451870, fn_7000451870 as *const (), 0), // +0x451870 sub_451870
    Lifted::new(0x70004518c0, fn_70004518c0 as *const (), 0), // +0x4518c0 sub_4518c0
    Lifted::new(0x7000451910, fn_7000451910 as *const (), 0), // +0x451910 sub_451910
    Lifted::new(0x7000451960, fn_7000451960 as *const (), 0), // +0x451960 sub_451960
    Lifted::new(0x70004519c0, fn_70004519c0 as *const (), 0), // +0x4519c0 sub_4519c0
    Lifted::new(0x7000451a10, fn_7000451a10 as *const (), 0), // +0x451a10 sub_451a10
    Lifted::new(0x7000451a60, fn_7000451a60 as *const (), 0), // +0x451a60 sub_451a60
    Lifted::new(0x7000451ab0, fn_7000451ab0 as *const (), 0), // +0x451ab0 sub_451ab0
    Lifted::new(0x7000451b00, fn_7000451b00 as *const (), 0), // +0x451b00 sub_451b00
    Lifted::new(0x7000451b50, fn_7000451b50 as *const (), 0), // +0x451b50 sub_451b50
    Lifted::new(0x7000451ba0, fn_7000451ba0 as *const (), 0), // +0x451ba0 sub_451ba0
    Lifted::new(0x7000451bf0, fn_7000451bf0 as *const (), 0), // +0x451bf0 sub_451bf0
    Lifted::new(0x7000451c40, fn_7000451c40 as *const (), 0), // +0x451c40 sub_451c40
    Lifted::new(0x7000451c90, fn_7000451c90 as *const (), 0), // +0x451c90 sub_451c90
    Lifted::new(0x7000451ce0, fn_7000451ce0 as *const (), 0), // +0x451ce0 sub_451ce0
    Lifted::new(0x7000451d30, fn_7000451d30 as *const (), 0), // +0x451d30 sub_451d30
    Lifted::new(0x7000451d80, fn_7000451d80 as *const (), 0), // +0x451d80 sub_451d80
    Lifted::new(0x7000451dd0, fn_7000451dd0 as *const (), 0), // +0x451dd0 sub_451dd0
    Lifted::new(0x7000451e20, fn_7000451e20 as *const (), 0), // +0x451e20 sub_451e20
    Lifted::new(0x7000451e70, fn_7000451e70 as *const (), 0), // +0x451e70 sub_451e70
    Lifted::new(0x7000451ec0, fn_7000451ec0 as *const (), 0), // +0x451ec0 sub_451ec0
    Lifted::new(0x7000451f10, fn_7000451f10 as *const (), 0), // +0x451f10 sub_451f10
    Lifted::new(0x7000451f60, fn_7000451f60 as *const (), 0), // +0x451f60 sub_451f60
    Lifted::new(0x7000451fb0, fn_7000451fb0 as *const (), 0), // +0x451fb0 sub_451fb0
    Lifted::new(0x7000452000, fn_7000452000 as *const (), 0), // +0x452000 sub_452000
    Lifted::new(0x7000452050, fn_7000452050 as *const (), 0), // +0x452050 sub_452050
    Lifted::new(0x70004520a0, fn_70004520a0 as *const (), 0), // +0x4520a0 sub_4520a0
    Lifted::new(0x70004520f0, fn_70004520f0 as *const (), 0), // +0x4520f0 sub_4520f0
    Lifted::new(0x7000452140, fn_7000452140 as *const (), 0), // +0x452140 sub_452140
    Lifted::new(0x7000452190, fn_7000452190 as *const (), 0), // +0x452190 sub_452190
    Lifted::new(0x70004521e0, fn_70004521e0 as *const (), 0), // +0x4521e0 sub_4521e0
    Lifted::new(0x7000452230, fn_7000452230 as *const (), 0), // +0x452230 sub_452230
    Lifted::new(0x7000452280, fn_7000452280 as *const (), 0), // +0x452280 sub_452280
    Lifted::new(0x70004522d0, fn_70004522d0 as *const (), 0), // +0x4522d0 sub_4522d0
    Lifted::new(0x7000452320, fn_7000452320 as *const (), 0), // +0x452320 sub_452320
    Lifted::new(0x7000452370, fn_7000452370 as *const (), 2), // +0x452370 sub_452370
    Lifted::new(0x7000452550, fn_7000452550 as *const (), 0), // +0x452550 sub_452550
    Lifted::new(0x70004525a0, fn_70004525a0 as *const (), 0), // +0x4525a0 sub_4525a0
    Lifted::new(0x70004525f0, fn_70004525f0 as *const (), 0), // +0x4525f0 sub_4525f0
    Lifted::new(0x7000452640, fn_7000452640 as *const (), 0), // +0x452640 sub_452640
    Lifted::new(0x7000452690, fn_7000452690 as *const (), 0), // +0x452690 sub_452690
    Lifted::new(0x70004526e0, fn_70004526e0 as *const (), 0), // +0x4526e0 sub_4526e0
    Lifted::new(0x7000452730, fn_7000452730 as *const (), 0), // +0x452730 sub_452730
    Lifted::new(0x7000452780, fn_7000452780 as *const (), 0), // +0x452780 sub_452780
    Lifted::new(0x70004527d0, fn_70004527d0 as *const (), 0), // +0x4527d0 sub_4527d0
    Lifted::new(0x7000452820, fn_7000452820 as *const (), 0), // +0x452820 sub_452820
    Lifted::new(0x70004529d0, fn_70004529d0 as *const (), 0), // +0x4529d0 sub_4529d0
    Lifted::new(0x7000452a20, fn_7000452a20 as *const (), 0), // +0x452a20 sub_452a20
    Lifted::new(0x7000452a70, fn_7000452a70 as *const (), 0), // +0x452a70 sub_452a70
    Lifted::new(0x7000452ac0, fn_7000452ac0 as *const (), 0), // +0x452ac0 sub_452ac0
    Lifted::new(0x7000452b10, fn_7000452b10 as *const (), 0), // +0x452b10 sub_452b10
    Lifted::new(0x7000452b60, fn_7000452b60 as *const (), 0), // +0x452b60 sub_452b60
    Lifted::new(0x7000452bb0, fn_7000452bb0 as *const (), 0), // +0x452bb0 sub_452bb0
    Lifted::new(0x7000452c00, fn_7000452c00 as *const (), 1), // +0x452c00 sub_452c00
    Lifted::new(0x7000452d40, fn_7000452d40 as *const (), 1), // +0x452d40 sub_452d40
    Lifted::new(0x7000452dd0, fn_7000452dd0 as *const (), 1), // +0x452dd0 sub_452dd0
    Lifted::new(0x7000452e80, fn_7000452e80 as *const (), 2), // +0x452e80 sub_452e80
    Lifted::new(0x7000452fb0, fn_7000452fb0 as *const (), 2), // +0x452fb0 sub_452fb0
    Lifted::new(0x7000453ba0, fn_7000453ba0 as *const (), 2), // +0x453ba0 sub_453ba0
    Lifted::new(0x7000453fb0, fn_7000453fb0 as *const (), 2), // +0x453fb0 sub_453fb0
    Lifted::new(0x70004542b0, fn_70004542b0 as *const (), 1), // +0x4542b0 sub_4542b0
    Lifted::new(0x7000454430, fn_7000454430 as *const (), 3), // +0x454430 sub_454430
    Lifted::new(0x7000454530, fn_7000454530 as *const (), 2), // +0x454530 sub_454530
    Lifted::new(0x70004547e0, fn_70004547e0 as *const (), 0), // +0x4547e0 sub_4547e0
    Lifted::new(0x70004549f0, fn_70004549f0 as *const (), 2), // +0x4549f0 sub_4549f0
    Lifted::new(0x7000454d40, fn_7000454d40 as *const (), 2), // +0x454d40 sub_454d40
    Lifted::new(0x7000454e90, fn_7000454e90 as *const (), 2), // +0x454e90 sub_454e90
    Lifted::new(0x7000455050, fn_7000455050 as *const (), 0), // +0x455050 sub_455050
    Lifted::new(0x70004551e0, fn_70004551e0 as *const (), 2), // +0x4551e0 sub_4551e0
    Lifted::new(0x7000456410, fn_7000456410 as *const (), 0), // +0x456410 sub_456410
    Lifted::new(0x7000456600, fn_7000456600 as *const (), 2), // +0x456600 sub_456600
    Lifted::new(0x70004566b0, fn_70004566b0 as *const (), 2), // +0x4566b0 sub_4566b0
    Lifted::new(0x7000456760, fn_7000456760 as *const (), 0), // +0x456760 sub_456760
    Lifted::new(0x70004567e0, fn_70004567e0 as *const (), 1), // +0x4567e0 sub_4567e0
    Lifted::new(0x70004568f0, fn_70004568f0 as *const (), 1), // +0x4568f0 sub_4568f0
    Lifted::new(0x7000456a10, fn_7000456a10 as *const (), 0), // +0x456a10 sub_456a10
    Lifted::new(0x7000456e90, fn_7000456e90 as *const (), 2), // +0x456e90 sub_456e90
    Lifted::new(0x7000457540, fn_7000457540 as *const (), 1), // +0x457540 sub_457540
    Lifted::new(0x70004576f0, fn_70004576f0 as *const (), 2), // +0x4576f0 sub_4576f0
    Lifted::new(0x7000457c40, fn_7000457c40 as *const (), 2), // +0x457c40 sub_457c40
    Lifted::new(0x70004581f0, fn_70004581f0 as *const (), 2), // +0x4581f0 sub_4581f0
    Lifted::new(0x70004587c0, fn_70004587c0 as *const (), 2), // +0x4587c0 sub_4587c0
    Lifted::new(0x70004588c0, fn_70004588c0 as *const (), 3), // +0x4588c0 sub_4588c0
    Lifted::new(0x7000458a20, fn_7000458a20 as *const (), 0), // +0x458a20 sub_458a20
    Lifted::new(0x7000458e20, fn_7000458e20 as *const (), 2), // +0x458e20 sub_458e20
    Lifted::new(0x7000459240, fn_7000459240 as *const (), 2), // +0x459240 sub_459240
    Lifted::new(0x700045aa40, fn_700045aa40 as *const (), 0), // +0x45aa40 sub_45aa40
    Lifted::new(0x700045b0f0, fn_700045b0f0 as *const (), 0), // +0x45b0f0 sub_45b0f0
    Lifted::new(0x700045b6a0, tpl_407::<0x70006931a0, 0x70002af6d0, 0x7000693258, 0x7000454530, 0x700062e470, 0x700062e45c> as *const (), 2), // +0x45b6a0 sub_45b6a0
    Lifted::new(0x700045b7c0, tpl_407::<0x7000693218, 0x70002af6d0, 0x7000693258, 0x7000454530, 0x700062e470, 0x700062e45c> as *const (), 2), // +0x45b7c0 sub_45b7c0
    Lifted::new(0x700045b8e0, fn_700045b8e0 as *const (), 2), // +0x45b8e0 sub_45b8e0
    Lifted::new(0x700045bec0, fn_700045bec0 as *const (), 2), // +0x45bec0 sub_45bec0
    Lifted::new(0x700045c580, fn_700045c580 as *const (), 0), // +0x45c580 sub_45c580
    Lifted::new(0x700045c700, fn_700045c700 as *const (), 0), // +0x45c700 sub_45c700
    Lifted::new(0x700045cea0, fn_700045cea0 as *const (), 2), // +0x45cea0 sub_45cea0
    Lifted::new(0x700045d510, fn_700045d510 as *const (), 2), // +0x45d510 sub_45d510
    Lifted::new(0x700045d750, fn_700045d750 as *const (), 2), // +0x45d750 sub_45d750
    Lifted::new(0x700045d7d0, fn_700045d7d0 as *const (), 2), // +0x45d7d0 sub_45d7d0
    Lifted::new(0x700045e230, fn_700045e230 as *const (), 0), // +0x45e230 sub_45e230
    Lifted::new(0x700045e320, fn_700045e320 as *const (), 0), // +0x45e320 sub_45e320
    Lifted::new(0x700045e780, fn_700045e780 as *const (), 0), // +0x45e780 sub_45e780
    Lifted::new(0x700045e8a0, fn_700045e8a0 as *const (), 2), // +0x45e8a0 sub_45e8a0
    Lifted::new(0x700045f0c0, fn_700045f0c0 as *const (), 2), // +0x45f0c0 sub_45f0c0
    Lifted::new(0x700045f9e0, fn_700045f9e0 as *const (), 2), // +0x45f9e0 sub_45f9e0
    Lifted::new(0x70004605a0, fn_70004605a0 as *const (), 2), // +0x4605a0 sub_4605a0
    Lifted::new(0x7000460a60, fn_7000460a60 as *const (), 2), // +0x460a60 sub_460a60
    Lifted::new(0x7000461040, fn_7000461040 as *const (), 2), // +0x461040 sub_461040
    Lifted::new(0x7000461680, fn_7000461680 as *const (), 2), // +0x461680 sub_461680
    Lifted::new(0x70004622d0, fn_70004622d0 as *const (), 2), // +0x4622d0 sub_4622d0
    Lifted::new(0x7000462990, fn_7000462990 as *const (), 2), // +0x462990 sub_462990
    Lifted::new(0x7000465c50, fn_7000465c50 as *const (), 0), // +0x465c50 sub_465c50
    Lifted::new(0x7000465f00, fn_7000465f00 as *const (), 0), // +0x465f00 sub_465f00
    Lifted::new(0x7000466160, fn_7000466160 as *const (), 0), // +0x466160 sub_466160
    Lifted::new(0x70004663c0, fn_70004663c0 as *const (), 0), // +0x4663c0 sub_4663c0
    Lifted::new(0x70004667a0, fn_70004667a0 as *const (), 0), // +0x4667a0 sub_4667a0
    Lifted::new(0x7000466910, fn_7000466910 as *const (), 2), // +0x466910 sub_466910
    Lifted::new(0x7000466d80, fn_7000466d80 as *const (), 2), // +0x466d80 sub_466d80
    Lifted::new(0x7000466f50, fn_7000466f50 as *const (), 2), // +0x466f50 sub_466f50
    Lifted::new(0x7000467480, fn_7000467480 as *const (), 2), // +0x467480 sub_467480
    Lifted::new(0x7000467880, fn_7000467880 as *const (), 0), // +0x467880 sub_467880
    Lifted::new(0x7000467d00, fn_7000467d00 as *const (), 0), // +0x467d00 sub_467d00
    Lifted::new(0x7000468180, fn_7000468180 as *const (), 2), // +0x468180 sub_468180
    Lifted::new(0x7000469090, fn_7000469090 as *const (), 0), // +0x469090 sub_469090
    Lifted::new(0x7000469510, fn_7000469510 as *const (), 0), // +0x469510 sub_469510
    Lifted::new(0x70004699a0, fn_70004699a0 as *const (), 0), // +0x4699a0 sub_4699a0
    Lifted::new(0x7000469e10, fn_7000469e10 as *const (), 0), // +0x469e10 sub_469e10
    Lifted::new(0x700046a380, fn_700046a380 as *const (), 0), // +0x46a380 sub_46a380
    Lifted::new(0x700046cbf0, fn_700046cbf0 as *const (), 2), // +0x46cbf0 sub_46cbf0
    Lifted::new(0x700046e600, fn_700046e600 as *const (), 2), // +0x46e600 sub_46e600
    Lifted::new(0x700046ed90, fn_700046ed90 as *const (), 0), // +0x46ed90 sub_46ed90
    Lifted::new(0x700046eed0, fn_700046eed0 as *const (), 1), // +0x46eed0 sub_46eed0
    Lifted::new(0x700046f0f0, fn_700046f0f0 as *const (), 0), // +0x46f0f0 sub_46f0f0
    Lifted::new(0x700046f350, fn_700046f350 as *const (), 2), // +0x46f350 sub_46f350
    Lifted::new(0x700046f970, fn_700046f970 as *const (), 2), // +0x46f970 sub_46f970
    Lifted::new(0x700046fe40, fn_700046fe40 as *const (), 0), // +0x46fe40 sub_46fe40
    Lifted::new(0x7000470130, fn_7000470130 as *const (), 2), // +0x470130 sub_470130
    Lifted::new(0x7000470340, fn_7000470340 as *const (), 2), // +0x470340 sub_470340
    Lifted::new(0x7000470fc0, fn_7000470fc0 as *const (), 2), // +0x470fc0 sub_470fc0
    Lifted::new(0x7000471820, fn_7000471820 as *const (), 0), // +0x471820 sub_471820
    Lifted::new(0x7000471bf0, fn_7000471bf0 as *const (), 2), // +0x471bf0 sub_471bf0
    Lifted::new(0x7000472020, fn_7000472020 as *const (), 2), // +0x472020 sub_472020
    Lifted::new(0x7000472530, fn_7000472530 as *const (), 2), // +0x472530 sub_472530
    Lifted::new(0x7000472a30, fn_7000472a30 as *const (), 2), // +0x472a30 sub_472a30
    Lifted::new(0x7000472fd0, fn_7000472fd0 as *const (), 2), // +0x472fd0 sub_472fd0
    Lifted::new(0x7000473310, fn_7000473310 as *const (), 2), // +0x473310 sub_473310
    Lifted::new(0x7000473650, fn_7000473650 as *const (), 2), // +0x473650 sub_473650
    Lifted::new(0x7000473990, fn_7000473990 as *const (), 2), // +0x473990 sub_473990
    Lifted::new(0x7000473cd0, fn_7000473cd0 as *const (), 2), // +0x473cd0 sub_473cd0
    Lifted::new(0x7000473e90, fn_7000473e90 as *const (), 0), // +0x473e90 sub_473e90
    Lifted::new(0x70004742b0, fn_70004742b0 as *const (), 2), // +0x4742b0 sub_4742b0
    Lifted::new(0x7000474690, fn_7000474690 as *const (), 2), // +0x474690 sub_474690
    Lifted::new(0x7000475a90, fn_7000475a90 as *const (), 2), // +0x475a90 sub_475a90
    Lifted::new(0x7000476000, fn_7000476000 as *const (), 2), // +0x476000 sub_476000
    Lifted::new(0x7000476430, fn_7000476430 as *const (), 0), // +0x476430 sub_476430
    Lifted::new(0x7000476810, fn_7000476810 as *const (), 2), // +0x476810 sub_476810
    Lifted::new(0x7000477af0, fn_7000477af0 as *const (), 2), // +0x477af0 sub_477af0
    Lifted::new(0x7000478130, fn_7000478130 as *const (), 0), // +0x478130 sub_478130
    Lifted::new(0x7000478350, fn_7000478350 as *const (), 0), // +0x478350 sub_478350
    Lifted::new(0x7000478b90, fn_7000478b90 as *const (), 2), // +0x478b90 sub_478b90
    Lifted::new(0x70004798e0, fn_70004798e0 as *const (), 2), // +0x4798e0 sub_4798e0
    Lifted::new(0x7000479fb0, fn_7000479fb0 as *const (), 0), // +0x479fb0 sub_479fb0
    Lifted::new(0x700047a120, fn_700047a120 as *const (), 0), // +0x47a120 sub_47a120
    Lifted::new(0x700047a3c0, fn_700047a3c0 as *const (), 2), // +0x47a3c0 sub_47a3c0
    Lifted::new(0x700047ad80, fn_700047ad80 as *const (), 2), // +0x47ad80 sub_47ad80
    Lifted::new(0x700047b2d0, fn_700047b2d0 as *const (), 2), // +0x47b2d0 sub_47b2d0
    Lifted::new(0x700047b860, fn_700047b860 as *const (), 2), // +0x47b860 sub_47b860
    Lifted::new(0x700047bc40, fn_700047bc40 as *const (), 2), // +0x47bc40 sub_47bc40
    Lifted::new(0x700047bce0, fn_700047bce0 as *const (), 2), // +0x47bce0 sub_47bce0
    Lifted::new(0x700047bd70, tpl_338::<0xc, 0x8, 0x70001dff80> as *const (), 0), // +0x47bd70 sub_47bd70
    Lifted::new(0x700047be30, fn_700047be30 as *const (), 2), // +0x47be30 sub_47be30
    Lifted::new(0x700047bf70, tpl_242::<0x1, 0x10, 0x700010a310, 0x7000407e10> as *const (), 0), // +0x47bf70 sub_47bf70
    Lifted::new(0x700047c040, fn_700047c040 as *const (), 0), // +0x47c040 sub_47c040
    Lifted::new(0x700047c0d0, fn_700047c0d0 as *const (), 0), // +0x47c0d0 sub_47c0d0
    Lifted::new(0x700047c170, fn_700047c170 as *const (), 0), // +0x47c170 sub_47c170
    Lifted::new(0x700047c200, fn_700047c200 as *const (), 2), // +0x47c200 sub_47c200
    Lifted::new(0x700047c2f0, fn_700047c2f0 as *const (), 2), // +0x47c2f0 sub_47c2f0
    Lifted::new(0x700047c380, fn_700047c380 as *const (), 0), // +0x47c380 sub_47c380
    Lifted::new(0x700047c420, fn_700047c420 as *const (), 2), // +0x47c420 sub_47c420
    Lifted::new(0x700047c4c0, fn_700047c4c0 as *const (), 0), // +0x47c4c0 sub_47c4c0
    Lifted::new(0x700047c610, fn_700047c610 as *const (), 2), // +0x47c610 sub_47c610
    Lifted::new(0x700047ca30, fn_700047ca30 as *const (), 2), // +0x47ca30 sub_47ca30
    Lifted::new(0x700047cf70, fn_700047cf70 as *const (), 2), // +0x47cf70 sub_47cf70
    Lifted::new(0x700047d410, fn_700047d410 as *const (), 0), // +0x47d410 sub_47d410
    Lifted::new(0x700047d5b0, fn_700047d5b0 as *const (), 0), // +0x47d5b0 sub_47d5b0
    Lifted::new(0x700047d7f0, fn_700047d7f0 as *const (), 0), // +0x47d7f0 sub_47d7f0
    Lifted::new(0x700047d9e0, fn_700047d9e0 as *const (), 0), // +0x47d9e0 sub_47d9e0
    Lifted::new(0x700047dbc0, fn_700047dbc0 as *const (), 0), // +0x47dbc0 sub_47dbc0
    Lifted::new(0x700047dd90, fn_700047dd90 as *const (), 0), // +0x47dd90 sub_47dd90
    Lifted::new(0x700047e130, fn_700047e130 as *const (), 0), // +0x47e130 sub_47e130
    Lifted::new(0x700047e370, tpl_408::<0x7000480890, 0x7000250520, 0x7000489c20, 0x700047e4fd> as *const (), 0), // +0x47e370 sub_47e370
    Lifted::new(0x700047e500, fn_700047e500 as *const (), 0), // +0x47e500 sub_47e500
    Lifted::new(0x700047e690, tpl_408::<0x70004809e0, 0x7000250520, 0x7000489c20, 0x700047e81d> as *const (), 0), // +0x47e690 sub_47e690
    Lifted::new(0x700047e820, fn_700047e820 as *const (), 0), // +0x47e820 sub_47e820
    Lifted::new(0x700047ea10, fn_700047ea10 as *const (), 0), // +0x47ea10 sub_47ea10
    Lifted::new(0x700047ebd0, fn_700047ebd0 as *const (), 0), // +0x47ebd0 sub_47ebd0
    Lifted::new(0x700047ed80, fn_700047ed80 as *const (), 0), // +0x47ed80 sub_47ed80
    Lifted::new(0x700047f120, fn_700047f120 as *const (), 2), // +0x47f120 sub_47f120
    Lifted::new(0x700047f640, fn_700047f640 as *const (), 0), // +0x47f640 sub_47f640
    Lifted::new(0x700047f9d0, fn_700047f9d0 as *const (), 2), // +0x47f9d0 sub_47f9d0
    Lifted::new(0x700047fee0, fn_700047fee0 as *const (), 0), // +0x47fee0 sub_47fee0
    Lifted::new(0x7000480270, fn_7000480270 as *const (), 2), // +0x480270 sub_480270
    Lifted::new(0x7000480560, fn_7000480560 as *const (), 2), // +0x480560 sub_480560
    Lifted::new(0x7000480890, fn_7000480890 as *const (), 2), // +0x480890 sub_480890
    Lifted::new(0x70004809e0, fn_70004809e0 as *const (), 2), // +0x4809e0 sub_4809e0
    Lifted::new(0x7000480b50, fn_7000480b50 as *const (), 1), // +0x480b50 sub_480b50
    Lifted::new(0x7000480b60, tpl_263::<0x7000693e28, 0x70005a83d0, 0x7000693e20, 0x8> as *const (), 0), // +0x480b60 sub_480b60
    Lifted::new(0x7000480bd0, tpl_263::<0x7000693de8, 0x70005a83d0, 0x7000693de0, 0x8> as *const (), 0), // +0x480bd0 sub_480bd0
    Lifted::new(0x7000480c40, fn_7000480c40 as *const (), 2), // +0x480c40 sub_480c40
    Lifted::new(0x7000480dd0, tpl_329::<0x7000693d28, 0x70005a83d0, 0x7000693d20, 0x8> as *const (), 0), // +0x480dd0 sub_480dd0
    Lifted::new(0x7000480e50, tpl_329::<0x7000693da8, 0x70005a83d0, 0x7000693da0, 0x8> as *const (), 0), // +0x480e50 sub_480e50
    Lifted::new(0x7000480ed0, tpl_329::<0x7000693d68, 0x70005a83d0, 0x7000693d60, 0x8> as *const (), 0), // +0x480ed0 sub_480ed0
    Lifted::new(0x7000480f50, fn_7000480f50 as *const (), 0), // +0x480f50 sub_480f50
    Lifted::new(0x7000480fa0, fn_7000480fa0 as *const (), 0), // +0x480fa0 sub_480fa0
    Lifted::new(0x7000481010, fn_7000481010 as *const (), 3), // +0x481010 sub_481010
    Lifted::new(0x7000481020, tpl_321::<0x7000697150, 0x10> as *const (), 0), // +0x481020 sub_481020
    Lifted::new(0x7000481050, tpl_78::<0x7000618580, 0x700048c2c0> as *const (), 0), // +0x481050 sub_481050
    Lifted::new(0x7000481080, tpl_78::<0x7000618568, 0x700048c2c0> as *const (), 0), // +0x481080 sub_481080
    Lifted::new(0x70004810b0, fn_70004810b0 as *const (), 2), // +0x4810b0 sub_4810b0
    Lifted::new(0x7000481b00, fn_7000481b00 as *const (), 0), // +0x481b00 sub_481b00
    Lifted::new(0x7000481b40, fn_7000481b40 as *const (), 0), // +0x481b40 sub_481b40
    Lifted::new(0x7000481bc0, fn_7000481bc0 as *const (), 0), // +0x481bc0 sub_481bc0
    Lifted::new(0x7000481da0, tpl_143::<0x698> as *const (), 1), // +0x481da0 sub_481da0
    Lifted::new(0x7000481db0, fn_7000481db0 as *const (), 1), // +0x481db0 sub_481db0
    Lifted::new(0x7000481dc0, fn_7000481dc0 as *const (), 2), // +0x481dc0 sub_481dc0
    Lifted::new(0x7000482080, fn_7000482080 as *const (), 0), // +0x482080 sub_482080
    Lifted::new(0x7000482380, fn_7000482380 as *const (), 0), // +0x482380 sub_482380
    Lifted::new(0x7000482410, fn_7000482410 as *const (), 0), // +0x482410 sub_482410
    Lifted::new(0x7000482460, tpl_409::<0x70003fc640, 0x70001328c0, 0x690, 0x70002b0130, 0x70001327c0, 0x70003fc560> as *const (), 0), // +0x482460 sub_482460
    Lifted::new(0x70004824c0, fn_70004824c0 as *const (), 2), // +0x4824c0 sub_4824c0
    Lifted::new(0x7000482660, fn_7000482660 as *const (), 0), // +0x482660 sub_482660
    Lifted::new(0x70004826d0, fn_70004826d0 as *const (), 0), // +0x4826d0 sub_4826d0
    Lifted::new(0x70004828d0, fn_70004828d0 as *const (), 0), // +0x4828d0 sub_4828d0
    Lifted::new(0x7000482cd0, fn_7000482cd0 as *const (), 0), // +0x482cd0 sub_482cd0
    Lifted::new(0x7000482d90, fn_7000482d90 as *const (), 0), // +0x482d90 sub_482d90
    Lifted::new(0x7000483090, fn_7000483090 as *const (), 0), // +0x483090 sub_483090
    Lifted::new(0x7000483540, fn_7000483540 as *const (), 0), // +0x483540 sub_483540
    Lifted::new(0x7000483590, fn_7000483590 as *const (), 0), // +0x483590 sub_483590
    Lifted::new(0x7000483740, fn_7000483740 as *const (), 0), // +0x483740 sub_483740
    Lifted::new(0x70004838e0, fn_70004838e0 as *const (), 0), // +0x4838e0 sub_4838e0
    Lifted::new(0x7000483940, fn_7000483940 as *const (), 2), // +0x483940 sub_483940
    Lifted::new(0x7000483de0, fn_7000483de0 as *const (), 2), // +0x483de0 sub_483de0
    Lifted::new(0x7000484570, fn_7000484570 as *const (), 2), // +0x484570 sub_484570
    Lifted::new(0x7000484830, fn_7000484830 as *const (), 2), // +0x484830 sub_484830
    Lifted::new(0x7000484b90, tpl_409::<0x70003fc640, 0x70001328c0, 0x6a0, 0x70003e4e90, 0x70001327c0, 0x70003fc560> as *const (), 0), // +0x484b90 sub_484b90
    Lifted::new(0x7000484bf0, fn_7000484bf0 as *const (), 0), // +0x484bf0 sub_484bf0
    Lifted::new(0x7000484c90, fn_7000484c90 as *const (), 2), // +0x484c90 sub_484c90
    Lifted::new(0x7000484df0, fn_7000484df0 as *const (), 2), // +0x484df0 sub_484df0
    Lifted::new(0x7000484ef0, fn_7000484ef0 as *const (), 0), // +0x484ef0 sub_484ef0
    Lifted::new(0x7000484fd0, tpl_373::<0x1c, 0x7000697150, 0x10, 0x70002b11a0> as *const (), 0), // +0x484fd0 sub_484fd0
    Lifted::new(0x7000485030, tpl_310::<0x70004855c0, 0x700004de60> as *const (), 0), // +0x485030 sub_485030
    Lifted::new(0x7000485150, fn_7000485150 as *const (), 2), // +0x485150 sub_485150
    Lifted::new(0x7000485250, fn_7000485250 as *const (), 0), // +0x485250 sub_485250
    Lifted::new(0x70004853b0, fn_70004853b0 as *const (), 0), // +0x4853b0 sub_4853b0
    Lifted::new(0x7000485510, fn_7000485510 as *const (), 0), // +0x485510 sub_485510
    Lifted::new(0x70004855c0, tpl_312::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000697150, 0x700004de60, 0x7000485510> as *const (), 2), // +0x4855c0 sub_4855c0
    Lifted::new(0x7000485720, tpl_286::<0x70005a8240, 0x7000618378, 0x70002445b0, 0x70005a8230> as *const (), 0), // +0x485720 sub_485720
    Lifted::new(0x7000485860, tpl_259::<0x7000484570> as *const (), 2), // +0x485860 sub_485860
    Lifted::new(0x7000485880, fn_7000485880 as *const (), 0), // +0x485880 sub_485880
    Lifted::new(0x70004858a0, fn_70004858a0 as *const (), 0), // +0x4858a0 sub_4858a0
    Lifted::new(0x7000485920, fn_7000485920 as *const (), 0), // +0x485920 sub_485920
    Lifted::new(0x70004859a0, tpl_78::<0x70006188d8, 0x700048c2c0> as *const (), 0), // +0x4859a0 sub_4859a0
    Lifted::new(0x70004859d0, tpl_176::<0x70006188d8, 0x700048c2c0> as *const (), 0), // +0x4859d0 sub_4859d0
    Lifted::new(0x7000485a00, fn_7000485a00 as *const (), 2), // +0x485a00 sub_485a00
    Lifted::new(0x7000485b40, fn_7000485b40 as *const (), 0), // +0x485b40 sub_485b40
    Lifted::new(0x7000485b70, fn_7000485b70 as *const (), 0), // +0x485b70 sub_485b70
    Lifted::new(0x7000485cd0, fn_7000485cd0 as *const (), 0), // +0x485cd0 sub_485cd0
    Lifted::new(0x7000485d60, fn_7000485d60 as *const (), 2), // +0x485d60 sub_485d60
    Lifted::new(0x7000485e50, fn_7000485e50 as *const (), 0), // +0x485e50 sub_485e50
    Lifted::new(0x7000485fd0, fn_7000485fd0 as *const (), 2), // +0x485fd0 sub_485fd0
    Lifted::new(0x70004867b0, fn_70004867b0 as *const (), 0), // +0x4867b0 sub_4867b0
    Lifted::new(0x7000486970, fn_7000486970 as *const (), 1), // +0x486970 sub_486970
    Lifted::new(0x7000486990, fn_7000486990 as *const (), 2), // +0x486990 sub_486990
    Lifted::new(0x7000486c10, tpl_139::<0x38, 0x40, 0x48, 0x50, 0x18, 0x20, 0x28, 0x30, 0x7000302100, 0x7000487120> as *const (), 0), // +0x486c10 sub_486c10
    Lifted::new(0x7000486c80, fn_7000486c80 as *const (), 0), // +0x486c80 sub_486c80
    Lifted::new(0x7000486f00, fn_7000486f00 as *const (), 0), // +0x486f00 sub_486f00
    Lifted::new(0x7000486fb0, fn_7000486fb0 as *const (), 2), // +0x486fb0 sub_486fb0
    Lifted::new(0x7000487120, fn_7000487120 as *const (), 0), // +0x487120 sub_487120
    Lifted::new(0x70004871a0, fn_70004871a0 as *const (), 0), // +0x4871a0 sub_4871a0
    Lifted::new(0x7000487320, fn_7000487320 as *const (), 0), // +0x487320 sub_487320
    Lifted::new(0x70004873a0, fn_70004873a0 as *const (), 1), // +0x4873a0 sub_4873a0
    Lifted::new(0x7000487460, fn_7000487460 as *const (), 0), // +0x487460 sub_487460
    Lifted::new(0x7000488040, fn_7000488040 as *const (), 0), // +0x488040 sub_488040
    Lifted::new(0x7000488220, fn_7000488220 as *const (), 2), // +0x488220 sub_488220
    Lifted::new(0x7000488640, fn_7000488640 as *const (), 0), // +0x488640 sub_488640
    Lifted::new(0x70004886b0, fn_70004886b0 as *const (), 0), // +0x4886b0 sub_4886b0
    Lifted::new(0x70004887d0, fn_70004887d0 as *const (), 0), // +0x4887d0 sub_4887d0
    Lifted::new(0x7000488c80, fn_7000488c80 as *const (), 0), // +0x488c80 sub_488c80
    Lifted::new(0x7000488d60, fn_7000488d60 as *const (), 0), // +0x488d60 sub_488d60
    Lifted::new(0x7000488ec0, fn_7000488ec0 as *const (), 0), // +0x488ec0 sub_488ec0
    Lifted::new(0x7000488f50, fn_7000488f50 as *const (), 2), // +0x488f50 sub_488f50
    Lifted::new(0x70004890a0, fn_70004890a0 as *const (), 0), // +0x4890a0 sub_4890a0
    Lifted::new(0x7000489160, fn_7000489160 as *const (), 2), // +0x489160 sub_489160
    Lifted::new(0x70004892b0, fn_70004892b0 as *const (), 0), // +0x4892b0 sub_4892b0
    Lifted::new(0x7000489410, fn_7000489410 as *const (), 0), // +0x489410 sub_489410
    Lifted::new(0x70004894c0, fn_70004894c0 as *const (), 2), // +0x4894c0 sub_4894c0
    Lifted::new(0x7000489670, fn_7000489670 as *const (), 0), // +0x489670 sub_489670
    Lifted::new(0x70004896b0, fn_70004896b0 as *const (), 2), // +0x4896b0 sub_4896b0
    Lifted::new(0x7000489840, fn_7000489840 as *const (), 0), // +0x489840 sub_489840
    Lifted::new(0x7000489860, tpl_250::<0x7000693e68, 0x70005a83d0, 0x7000693e60, 0x8, 0x10> as *const (), 2), // +0x489860 sub_489860
    Lifted::new(0x70004898f0, fn_70004898f0 as *const (), 0), // +0x4898f0 sub_4898f0
    Lifted::new(0x7000489940, tpl_198::<0x7000694010, 0x8, 0x7000694018, 0x70005a83d0> as *const (), 0), // +0x489940 sub_489940
    Lifted::new(0x70004899c0, fn_70004899c0 as *const (), 0), // +0x4899c0 sub_4899c0
    Lifted::new(0x7000489a00, tpl_198::<0x7000693fd0, 0x8, 0x7000693fd8, 0x70005a83d0> as *const (), 0), // +0x489a00 sub_489a00
    Lifted::new(0x7000489a80, fn_7000489a80 as *const (), 2), // +0x489a80 sub_489a80
    Lifted::new(0x7000489c20, fn_7000489c20 as *const (), 0), // +0x489c20 sub_489c20
    Lifted::new(0x7000489c50, fn_7000489c50 as *const (), 0), // +0x489c50 sub_489c50
    Lifted::new(0x7000489cb0, fn_7000489cb0 as *const (), 0), // +0x489cb0 sub_489cb0
    Lifted::new(0x7000489e60, fn_7000489e60 as *const (), 2), // +0x489e60 sub_489e60
    Lifted::new(0x700048a380, fn_700048a380 as *const (), 0), // +0x48a380 sub_48a380
    Lifted::new(0x700048a480, fn_700048a480 as *const (), 0), // +0x48a480 sub_48a480
    Lifted::new(0x700048a5a0, fn_700048a5a0 as *const (), 0), // +0x48a5a0 sub_48a5a0
    Lifted::new(0x700048a880, fn_700048a880 as *const (), 0), // +0x48a880 sub_48a880
    Lifted::new(0x700048aaa0, fn_700048aaa0 as *const (), 0), // +0x48aaa0 sub_48aaa0
    Lifted::new(0x700048ac30, fn_700048ac30 as *const (), 0), // +0x48ac30 sub_48ac30
    Lifted::new(0x700048acb0, fn_700048acb0 as *const (), 2), // +0x48acb0 sub_48acb0
    Lifted::new(0x700048ae40, fn_700048ae40 as *const (), 0), // +0x48ae40 sub_48ae40
    Lifted::new(0x700048b030, fn_700048b030 as *const (), 2), // +0x48b030 sub_48b030
    Lifted::new(0x700048bd10, fn_700048bd10 as *const (), 1), // +0x48bd10 sub_48bd10
    Lifted::new(0x700048be30, tpl_309::<0x1, 0x10, 0x700010a310, 0x70001a06c0> as *const (), 0), // +0x48be30 sub_48be30
    Lifted::new(0x700048beb0, fn_700048beb0 as *const (), 0), // +0x48beb0 sub_48beb0
    Lifted::new(0x700048bf50, fn_700048bf50 as *const (), 0), // +0x48bf50 sub_48bf50
    Lifted::new(0x700048c180, fn_700048c180 as *const (), 0), // +0x48c180 sub_48c180
    Lifted::new(0x700048c1c0, fn_700048c1c0 as *const (), 0), // +0x48c1c0 sub_48c1c0
    Lifted::new(0x700048c220, fn_700048c220 as *const (), 0), // +0x48c220 sub_48c220
    Lifted::new(0x700048c250, tpl_207::<0x10, 0x18> as *const (), 0), // +0x48c250 sub_48c250
    Lifted::new(0x700048c260, tpl_207::<0x10, 0x20> as *const (), 0), // +0x48c260 sub_48c260
    Lifted::new(0x700048c270, fn_700048c270 as *const (), 0), // +0x48c270 sub_48c270
    Lifted::new(0x700048c280, fn_700048c280 as *const (), 0), // +0x48c280 sub_48c280
    Lifted::new(0x700048c290, fn_700048c290 as *const (), 0), // +0x48c290 sub_48c290
    Lifted::new(0x700048c2c0, fn_700048c2c0 as *const (), 0), // +0x48c2c0 sub_48c2c0
    Lifted::new(0x700048c2e0, tpl_143::<0x170> as *const (), 1), // +0x48c2e0 sub_48c2e0
    Lifted::new(0x700048c2f0, tpl_143::<0x174> as *const (), 1), // +0x48c2f0 sub_48c2f0
    Lifted::new(0x700048c300, tpl_410::<0x1b7> as *const (), 1), // +0x48c300 sub_48c300
    Lifted::new(0x700048c310, tpl_410::<0x221> as *const (), 1), // +0x48c310 sub_48c310
    Lifted::new(0x700048c320, tpl_411::<0x224, 0x700049cce0> as *const (), 0), // +0x48c320 sub_48c320
    Lifted::new(0x700048c330, tpl_222::<0x1b0> as *const (), 3), // +0x48c330 sub_48c330
    Lifted::new(0x700048c340, fn_700048c340 as *const (), 1), // +0x48c340 sub_48c340
    Lifted::new(0x700048c350, fn_700048c350 as *const (), 2), // +0x48c350 sub_48c350
    Lifted::new(0x700048c380, fn_700048c380 as *const (), 2), // +0x48c380 sub_48c380
    Lifted::new(0x700048c470, tpl_71::<0x70005fe508, 0x700048c2c0> as *const (), 0), // +0x48c470 sub_48c470
    Lifted::new(0x700048c4a0, tpl_180::<0x8, 0x18, 0x30> as *const (), 0), // +0x48c4a0 sub_48c4a0
    Lifted::new(0x700048c4e0, fn_700048c4e0 as *const (), 0), // +0x48c4e0 sub_48c4e0
    Lifted::new(0x700048c520, fn_700048c520 as *const (), 2), // +0x48c520 sub_48c520
    Lifted::new(0x700048c760, fn_700048c760 as *const (), 3), // +0x48c760 sub_48c760
    Lifted::new(0x700048c8d0, fn_700048c8d0 as *const (), 2), // +0x48c8d0 sub_48c8d0
    Lifted::new(0x700048cec0, fn_700048cec0 as *const (), 2), // +0x48cec0 sub_48cec0
    Lifted::new(0x700048d440, fn_700048d440 as *const (), 2), // +0x48d440 sub_48d440
    Lifted::new(0x700048db10, fn_700048db10 as *const (), 2), // +0x48db10 sub_48db10
    Lifted::new(0x700048dc20, fn_700048dc20 as *const (), 2), // +0x48dc20 sub_48dc20
    Lifted::new(0x700048dee0, fn_700048dee0 as *const (), 0), // +0x48dee0 sub_48dee0
    Lifted::new(0x700048dfa0, fn_700048dfa0 as *const (), 2), // +0x48dfa0 sub_48dfa0
    Lifted::new(0x700048e2a0, fn_700048e2a0 as *const (), 2), // +0x48e2a0 sub_48e2a0
    Lifted::new(0x700048ec00, fn_700048ec00 as *const (), 2), // +0x48ec00 sub_48ec00
    Lifted::new(0x700048f080, fn_700048f080 as *const (), 2), // +0x48f080 sub_48f080
    Lifted::new(0x700048f1f0, fn_700048f1f0 as *const (), 2), // +0x48f1f0 sub_48f1f0
    Lifted::new(0x70004900b0, tpl_71::<0x700061a3b8, 0x700048c2c0> as *const (), 0), // +0x4900b0 sub_4900b0
    Lifted::new(0x70004900e0, tpl_78::<0x700061a160, 0x700048c2c0> as *const (), 0), // +0x4900e0 sub_4900e0
    Lifted::new(0x7000490110, fn_7000490110 as *const (), 2), // +0x490110 sub_490110
    Lifted::new(0x70004914a0, fn_70004914a0 as *const (), 0), // +0x4914a0 sub_4914a0
    Lifted::new(0x70004914e0, fn_70004914e0 as *const (), 0), // +0x4914e0 sub_4914e0
    Lifted::new(0x7000491650, fn_7000491650 as *const (), 0), // +0x491650 sub_491650
    Lifted::new(0x7000491770, fn_7000491770 as *const (), 0), // +0x491770 sub_491770
    Lifted::new(0x70004917f0, fn_70004917f0 as *const (), 0), // +0x4917f0 sub_4917f0
    Lifted::new(0x7000491b90, fn_7000491b90 as *const (), 0), // +0x491b90 sub_491b90
    Lifted::new(0x7000491bc0, tpl_143::<0x90> as *const (), 1), // +0x491bc0 sub_491bc0
    Lifted::new(0x7000491bd0, tpl_143::<0x350> as *const (), 1), // +0x491bd0 sub_491bd0
    Lifted::new(0x7000491be0, fn_7000491be0 as *const (), 2), // +0x491be0 sub_491be0
    Lifted::new(0x7000491cf0, fn_7000491cf0 as *const (), 3), // +0x491cf0 sub_491cf0
    Lifted::new(0x7000491d60, fn_7000491d60 as *const (), 1), // +0x491d60 sub_491d60
    Lifted::new(0x7000491d80, fn_7000491d80 as *const (), 2), // +0x491d80 sub_491d80
    Lifted::new(0x7000491de0, fn_7000491de0 as *const (), 2), // +0x491de0 sub_491de0
    Lifted::new(0x7000492870, fn_7000492870 as *const (), 2), // +0x492870 sub_492870
    Lifted::new(0x7000492a60, fn_7000492a60 as *const (), 0), // +0x492a60 sub_492a60
    Lifted::new(0x7000492c40, fn_7000492c40 as *const (), 2), // +0x492c40 sub_492c40
    Lifted::new(0x7000492fa0, fn_7000492fa0 as *const (), 0), // +0x492fa0 sub_492fa0
    Lifted::new(0x70004935c0, tpl_118::<0x158, 0x700023d100> as *const (), 0), // +0x4935c0 sub_4935c0
    Lifted::new(0x70004935d0, fn_70004935d0 as *const (), 0), // +0x4935d0 sub_4935d0
    Lifted::new(0x7000493a20, fn_7000493a20 as *const (), 2), // +0x493a20 sub_493a20
    Lifted::new(0x7000493da0, fn_7000493da0 as *const (), 1), // +0x493da0 sub_493da0
    Lifted::new(0x7000493ec0, fn_7000493ec0 as *const (), 0), // +0x493ec0 sub_493ec0
    Lifted::new(0x7000493ff0, fn_7000493ff0 as *const (), 0), // +0x493ff0 sub_493ff0
    Lifted::new(0x7000494110, fn_7000494110 as *const (), 0), // +0x494110 sub_494110
    Lifted::new(0x7000494150, tpl_176::<0x7000607820, 0x700048c2c0> as *const (), 0), // +0x494150 sub_494150
    Lifted::new(0x7000494180, fn_7000494180 as *const (), 0), // +0x494180 sub_494180
    Lifted::new(0x70004941c0, fn_70004941c0 as *const (), 1), // +0x4941c0 sub_4941c0
    Lifted::new(0x7000494270, fn_7000494270 as *const (), 2), // +0x494270 sub_494270
    Lifted::new(0x70004942f0, fn_70004942f0 as *const (), 0), // +0x4942f0 sub_4942f0
    Lifted::new(0x7000494340, fn_7000494340 as *const (), 0), // +0x494340 sub_494340
    Lifted::new(0x7000494410, fn_7000494410 as *const (), 2), // +0x494410 sub_494410
    Lifted::new(0x70004945d0, fn_70004945d0 as *const (), 2), // +0x4945d0 sub_4945d0
    Lifted::new(0x70004947b0, fn_70004947b0 as *const (), 2), // +0x4947b0 sub_4947b0
    Lifted::new(0x7000494bc0, fn_7000494bc0 as *const (), 2), // +0x494bc0 sub_494bc0
    Lifted::new(0x7000494f50, fn_7000494f50 as *const (), 0), // +0x494f50 sub_494f50
    Lifted::new(0x7000495100, fn_7000495100 as *const (), 2), // +0x495100 sub_495100
    Lifted::new(0x70004952a0, fn_70004952a0 as *const (), 2), // +0x4952a0 sub_4952a0
    Lifted::new(0x7000495660, fn_7000495660 as *const (), 2), // +0x495660 sub_495660
    Lifted::new(0x7000496230, fn_7000496230 as *const (), 2), // +0x496230 sub_496230
    Lifted::new(0x7000496490, fn_7000496490 as *const (), 0), // +0x496490 sub_496490
    Lifted::new(0x7000496580, fn_7000496580 as *const (), 0), // +0x496580 sub_496580
    Lifted::new(0x70004966b0, fn_70004966b0 as *const (), 0), // +0x4966b0 sub_4966b0
    Lifted::new(0x7000496870, fn_7000496870 as *const (), 2), // +0x496870 sub_496870
    Lifted::new(0x7000496d70, fn_7000496d70 as *const (), 0), // +0x496d70 sub_496d70
    Lifted::new(0x7000496f30, fn_7000496f30 as *const (), 2), // +0x496f30 sub_496f30
    Lifted::new(0x7000498080, fn_7000498080 as *const (), 0), // +0x498080 sub_498080
    Lifted::new(0x7000498300, fn_7000498300 as *const (), 0), // +0x498300 sub_498300
    Lifted::new(0x70004984c0, fn_70004984c0 as *const (), 2), // +0x4984c0 sub_4984c0
    Lifted::new(0x70004987d0, fn_70004987d0 as *const (), 2), // +0x4987d0 sub_4987d0
    Lifted::new(0x70004989e0, fn_70004989e0 as *const (), 0), // +0x4989e0 sub_4989e0
    Lifted::new(0x7000498a60, fn_7000498a60 as *const (), 0), // +0x498a60 sub_498a60
    Lifted::new(0x7000498d30, fn_7000498d30 as *const (), 2), // +0x498d30 sub_498d30
    Lifted::new(0x7000498f00, fn_7000498f00 as *const (), 2), // +0x498f00 sub_498f00
    Lifted::new(0x7000499310, fn_7000499310 as *const (), 2), // +0x499310 sub_499310
    Lifted::new(0x7000499660, fn_7000499660 as *const (), 2), // +0x499660 sub_499660
    Lifted::new(0x7000499ad0, fn_7000499ad0 as *const (), 2), // +0x499ad0 sub_499ad0
    Lifted::new(0x7000499eb0, fn_7000499eb0 as *const (), 0), // +0x499eb0 sub_499eb0
    Lifted::new(0x7000499fa0, fn_7000499fa0 as *const (), 0), // +0x499fa0 sub_499fa0
    Lifted::new(0x700049a120, fn_700049a120 as *const (), 2), // +0x49a120 sub_49a120
    Lifted::new(0x700049a850, fn_700049a850 as *const (), 2), // +0x49a850 sub_49a850
    Lifted::new(0x700049b250, fn_700049b250 as *const (), 2), // +0x49b250 sub_49b250
    Lifted::new(0x700049ba60, fn_700049ba60 as *const (), 0), // +0x49ba60 sub_49ba60
    Lifted::new(0x700049bb20, fn_700049bb20 as *const (), 2), // +0x49bb20 sub_49bb20
    Lifted::new(0x700049bf10, fn_700049bf10 as *const (), 2), // +0x49bf10 sub_49bf10
    Lifted::new(0x700049c180, fn_700049c180 as *const (), 0), // +0x49c180 sub_49c180
    Lifted::new(0x700049c240, fn_700049c240 as *const (), 0), // +0x49c240 sub_49c240
    Lifted::new(0x700049c300, fn_700049c300 as *const (), 0), // +0x49c300 sub_49c300
    Lifted::new(0x700049c3b0, fn_700049c3b0 as *const (), 2), // +0x49c3b0 sub_49c3b0
    Lifted::new(0x700049c570, fn_700049c570 as *const (), 0), // +0x49c570 sub_49c570
    Lifted::new(0x700049c700, fn_700049c700 as *const (), 2), // +0x49c700 sub_49c700
    Lifted::new(0x700049c9c0, fn_700049c9c0 as *const (), 2), // +0x49c9c0 sub_49c9c0
    Lifted::new(0x700049cce0, fn_700049cce0 as *const (), 2), // +0x49cce0 sub_49cce0
    Lifted::new(0x700049cd50, fn_700049cd50 as *const (), 2), // +0x49cd50 sub_49cd50
    Lifted::new(0x700049cf20, fn_700049cf20 as *const (), 0), // +0x49cf20 sub_49cf20
    Lifted::new(0x700049d0e0, fn_700049d0e0 as *const (), 0), // +0x49d0e0 sub_49d0e0
    Lifted::new(0x700049d160, fn_700049d160 as *const (), 0), // +0x49d160 sub_49d160
    Lifted::new(0x700049d190, fn_700049d190 as *const (), 0), // +0x49d190 sub_49d190
    Lifted::new(0x700049d210, tpl_410::<0x1b79> as *const (), 1), // +0x49d210 sub_49d210
    Lifted::new(0x700049d220, tpl_324::<0x1b79> as *const (), 1), // +0x49d220 sub_49d220
    Lifted::new(0x700049d230, tpl_411::<0x35c, 0x700049cce0> as *const (), 0), // +0x49d230 sub_49d230
    Lifted::new(0x700049d240, fn_700049d240 as *const (), 0), // +0x49d240 sub_49d240
    Lifted::new(0x700049d2b0, fn_700049d2b0 as *const (), 0), // +0x49d2b0 sub_49d2b0
    Lifted::new(0x700049d3f0, fn_700049d3f0 as *const (), 0), // +0x49d3f0 sub_49d3f0
    Lifted::new(0x700049d540, fn_700049d540 as *const (), 1), // +0x49d540 sub_49d540
    Lifted::new(0x700049d570, fn_700049d570 as *const (), 1), // +0x49d570 sub_49d570
    Lifted::new(0x700049d590, fn_700049d590 as *const (), 1), // +0x49d590 sub_49d590
    Lifted::new(0x700049d5b0, fn_700049d5b0 as *const (), 1), // +0x49d5b0 sub_49d5b0
    Lifted::new(0x700049d5e0, fn_700049d5e0 as *const (), 1), // +0x49d5e0 sub_49d5e0
    Lifted::new(0x700049d600, fn_700049d600 as *const (), 0), // +0x49d600 sub_49d600
    Lifted::new(0x700049d7b0, fn_700049d7b0 as *const (), 0), // +0x49d7b0 sub_49d7b0
    Lifted::new(0x700049d940, fn_700049d940 as *const (), 0), // +0x49d940 sub_49d940
    Lifted::new(0x700049dd30, fn_700049dd30 as *const (), 0), // +0x49dd30 sub_49dd30
    Lifted::new(0x700049df00, fn_700049df00 as *const (), 1), // +0x49df00 sub_49df00
    Lifted::new(0x700049df10, fn_700049df10 as *const (), 0), // +0x49df10 sub_49df10
    Lifted::new(0x700049e010, fn_700049e010 as *const (), 0), // +0x49e010 sub_49e010
    Lifted::new(0x700049e190, fn_700049e190 as *const (), 0), // +0x49e190 sub_49e190
    Lifted::new(0x700049e3d0, fn_700049e3d0 as *const (), 0), // +0x49e3d0 sub_49e3d0
    Lifted::new(0x700049e530, fn_700049e530 as *const (), 0), // +0x49e530 sub_49e530
    Lifted::new(0x700049e730, fn_700049e730 as *const (), 0), // +0x49e730 sub_49e730
    Lifted::new(0x700049e7c0, fn_700049e7c0 as *const (), 0), // +0x49e7c0 sub_49e7c0
    Lifted::new(0x700049e840, fn_700049e840 as *const (), 2), // +0x49e840 sub_49e840
    Lifted::new(0x700049e900, fn_700049e900 as *const (), 2), // +0x49e900 sub_49e900
    Lifted::new(0x700049ee90, tpl_274::<0x290, 0xd8> as *const (), 0), // +0x49ee90 sub_49ee90
    Lifted::new(0x700049eeb0, tpl_368::<0x290, 0xe0> as *const (), 0), // +0x49eeb0 sub_49eeb0
    Lifted::new(0x700049eed0, fn_700049eed0 as *const (), 0), // +0x49eed0 sub_49eed0
    Lifted::new(0x700049ef90, fn_700049ef90 as *const (), 0), // +0x49ef90 sub_49ef90
    Lifted::new(0x700049f170, fn_700049f170 as *const (), 0), // +0x49f170 sub_49f170
    Lifted::new(0x700049f1f0, fn_700049f1f0 as *const (), 0), // +0x49f1f0 sub_49f1f0
    Lifted::new(0x700049f270, fn_700049f270 as *const (), 1), // +0x49f270 sub_49f270
    Lifted::new(0x700049f290, tpl_412::<0x70003fc640, 0x70001328c0, 0x150, 0x70006974b8, 0x7000619ad8, 0x70005f84a0, 0x7000130980, 0x3018, 0x70001b3320, 0x98, 0x90, 0x20, 0x7000485fd0, 0x70001327c0, 0x70003fc560> as *const (), 0), // +0x49f290 sub_49f290
    Lifted::new(0x700049f350, fn_700049f350 as *const (), 0), // +0x49f350 sub_49f350
    Lifted::new(0x700049f3e0, fn_700049f3e0 as *const (), 0), // +0x49f3e0 sub_49f3e0
    Lifted::new(0x700049f5b0, tpl_412::<0x70003fc640, 0x70001328c0, 0x150, 0x70006974b8, 0x7000619ad8, 0x70005f84a0, 0x7000130980, 0x3018, 0x70001b37a0, 0x98, 0x90, 0x20, 0x7000485fd0, 0x70001327c0, 0x70003fc560> as *const (), 0), // +0x49f5b0 sub_49f5b0
    Lifted::new(0x700049f670, fn_700049f670 as *const (), 0), // +0x49f670 sub_49f670
    Lifted::new(0x700049f6b0, fn_700049f6b0 as *const (), 0), // +0x49f6b0 sub_49f6b0
    Lifted::new(0x700049f750, fn_700049f750 as *const (), 1), // +0x49f750 sub_49f750
    Lifted::new(0x700049f770, fn_700049f770 as *const (), 1), // +0x49f770 sub_49f770
    Lifted::new(0x700049f7a0, fn_700049f7a0 as *const (), 0), // +0x49f7a0 sub_49f7a0
    Lifted::new(0x700049f960, fn_700049f960 as *const (), 2), // +0x49f960 sub_49f960
    Lifted::new(0x700049fa50, fn_700049fa50 as *const (), 0), // +0x49fa50 sub_49fa50
    Lifted::new(0x700049fc10, fn_700049fc10 as *const (), 0), // +0x49fc10 sub_49fc10
    Lifted::new(0x700049fdc0, fn_700049fdc0 as *const (), 0), // +0x49fdc0 sub_49fdc0
    Lifted::new(0x700049fe60, fn_700049fe60 as *const (), 0), // +0x49fe60 sub_49fe60
    Lifted::new(0x700049fee0, fn_700049fee0 as *const (), 0), // +0x49fee0 sub_49fee0
    Lifted::new(0x700049ff70, fn_700049ff70 as *const (), 0), // +0x49ff70 sub_49ff70
    Lifted::new(0x70004a0050, fn_70004a0050 as *const (), 0), // +0x4a0050 sub_4a0050
    Lifted::new(0x70004a0130, fn_70004a0130 as *const (), 0), // +0x4a0130 sub_4a0130
    Lifted::new(0x70004a0230, fn_70004a0230 as *const (), 0), // +0x4a0230 sub_4a0230
    Lifted::new(0x70004a0330, fn_70004a0330 as *const (), 0), // +0x4a0330 sub_4a0330
    Lifted::new(0x70004a0430, fn_70004a0430 as *const (), 0), // +0x4a0430 sub_4a0430
    Lifted::new(0x70004a0510, fn_70004a0510 as *const (), 0), // +0x4a0510 sub_4a0510
    Lifted::new(0x70004a05f0, fn_70004a05f0 as *const (), 0), // +0x4a05f0 sub_4a05f0
    Lifted::new(0x70004a06f0, fn_70004a06f0 as *const (), 0), // +0x4a06f0 sub_4a06f0
    Lifted::new(0x70004a07f0, fn_70004a07f0 as *const (), 0), // +0x4a07f0 sub_4a07f0
    Lifted::new(0x70004a0970, fn_70004a0970 as *const (), 0), // +0x4a0970 sub_4a0970
    Lifted::new(0x70004a0aa0, fn_70004a0aa0 as *const (), 0), // +0x4a0aa0 sub_4a0aa0
    Lifted::new(0x70004a0b30, fn_70004a0b30 as *const (), 2), // +0x4a0b30 sub_4a0b30
    Lifted::new(0x70004a0c30, fn_70004a0c30 as *const (), 0), // +0x4a0c30 sub_4a0c30
    Lifted::new(0x70004a0d60, fn_70004a0d60 as *const (), 0), // +0x4a0d60 sub_4a0d60
    Lifted::new(0x70004a0e10, fn_70004a0e10 as *const (), 0), // +0x4a0e10 sub_4a0e10
    Lifted::new(0x70004a0e80, fn_70004a0e80 as *const (), 2), // +0x4a0e80 sub_4a0e80
    Lifted::new(0x70004a0fe0, tpl_39::<0x8, 0xc, 0x700062e338, 0x70005a4d32, 0x700062e490, 0x70004a1a50> as *const (), 2), // +0x4a0fe0 sub_4a0fe0
    Lifted::new(0x70004a1070, fn_70004a1070 as *const (), 0), // +0x4a1070 sub_4a1070
    Lifted::new(0x70004a1110, tpl_38::<0x700062e338, 0x70005a4d32, 0x700062e490, 0x7000694974, 0x700004f420, 0x70004a1070> as *const (), 2), // +0x4a1110 sub_4a1110
    Lifted::new(0x70004a1220, fn_70004a1220 as *const (), 0), // +0x4a1220 sub_4a1220
    Lifted::new(0x70004a12a0, fn_70004a12a0 as *const (), 0), // +0x4a12a0 sub_4a12a0
    Lifted::new(0x70004a1340, fn_70004a1340 as *const (), 2), // +0x4a1340 sub_4a1340
    Lifted::new(0x70004a13e0, fn_70004a13e0 as *const (), 2), // +0x4a13e0 sub_4a13e0
    Lifted::new(0x70004a14c0, fn_70004a14c0 as *const (), 0), // +0x4a14c0 sub_4a14c0
    Lifted::new(0x70004a15b0, fn_70004a15b0 as *const (), 2), // +0x4a15b0 sub_4a15b0
    Lifted::new(0x70004a16d0, fn_70004a16d0 as *const (), 0), // +0x4a16d0 sub_4a16d0
    Lifted::new(0x70004a1740, fn_70004a1740 as *const (), 2), // +0x4a1740 sub_4a1740
    Lifted::new(0x70004a1890, fn_70004a1890 as *const (), 0), // +0x4a1890 sub_4a1890
    Lifted::new(0x70004a1940, fn_70004a1940 as *const (), 2), // +0x4a1940 sub_4a1940
    Lifted::new(0x70004a1a50, fn_70004a1a50 as *const (), 0), // +0x4a1a50 sub_4a1a50
    Lifted::new(0x70004a1b30, fn_70004a1b30 as *const (), 0), // +0x4a1b30 sub_4a1b30
    Lifted::new(0x70004a1bb0, fn_70004a1bb0 as *const (), 2), // +0x4a1bb0 sub_4a1bb0
    Lifted::new(0x70004a1dd0, fn_70004a1dd0 as *const (), 2), // +0x4a1dd0 sub_4a1dd0
    Lifted::new(0x70004a3380, fn_70004a3380 as *const (), 2), // +0x4a3380 sub_4a3380
    Lifted::new(0x70004a3980, fn_70004a3980 as *const (), 2), // +0x4a3980 sub_4a3980
    Lifted::new(0x70004a3c60, fn_70004a3c60 as *const (), 2), // +0x4a3c60 sub_4a3c60
    Lifted::new(0x70004a5060, fn_70004a5060 as *const (), 2), // +0x4a5060 sub_4a5060
    Lifted::new(0x70004a6660, fn_70004a6660 as *const (), 2), // +0x4a6660 sub_4a6660
    Lifted::new(0x70004a7a80, fn_70004a7a80 as *const (), 0), // +0x4a7a80 sub_4a7a80
    Lifted::new(0x70004a7bf0, fn_70004a7bf0 as *const (), 0), // +0x4a7bf0 sub_4a7bf0
    Lifted::new(0x70004a7d60, fn_70004a7d60 as *const (), 0), // +0x4a7d60 sub_4a7d60
    Lifted::new(0x70004a7eb0, tpl_231::<0x70005a8240, 0x70002445b0, 0x70005a8210, 0x70005a8230> as *const (), 0), // +0x4a7eb0 sub_4a7eb0
    Lifted::new(0x70004a8000, fn_70004a8000 as *const (), 0), // +0x4a8000 sub_4a8000
    Lifted::new(0x70004a8140, fn_70004a8140 as *const (), 0), // +0x4a8140 sub_4a8140
    Lifted::new(0x70004a8240, fn_70004a8240 as *const (), 0), // +0x4a8240 sub_4a8240
    Lifted::new(0x70004a82d0, fn_70004a82d0 as *const (), 2), // +0x4a82d0 sub_4a82d0
    Lifted::new(0x70004a8440, tpl_308::<0x70005a8240, 0x70006193d0, 0x70002445b0, 0x70005a8230> as *const (), 0), // +0x4a8440 sub_4a8440
    Lifted::new(0x70004a8560, tpl_115::<0x18, 0x8, 0x20, 0x70004aa4c0> as *const (), 0), // +0x4a8560 sub_4a8560
    Lifted::new(0x70004a8610, fn_70004a8610 as *const (), 3), // +0x4a8610 sub_4a8610
    Lifted::new(0x70004a8680, fn_70004a8680 as *const (), 2), // +0x4a8680 sub_4a8680
    Lifted::new(0x70004a86f0, fn_70004a86f0 as *const (), 0), // +0x4a86f0 sub_4a86f0
    Lifted::new(0x70004a8750, tpl_210::<0x7000495660> as *const (), 0), // +0x4a8750 sub_4a8750
    Lifted::new(0x70004a8770, tpl_259::<0x7000496230> as *const (), 2), // +0x4a8770 sub_4a8770
    Lifted::new(0x70004a8790, fn_70004a8790 as *const (), 0), // +0x4a8790 sub_4a8790
    Lifted::new(0x70004a8800, fn_70004a8800 as *const (), 0), // +0x4a8800 sub_4a8800
    Lifted::new(0x70004a8870, fn_70004a8870 as *const (), 0), // +0x4a8870 sub_4a8870
    Lifted::new(0x70004a88c0, fn_70004a88c0 as *const (), 0), // +0x4a88c0 sub_4a88c0
    Lifted::new(0x70004a8a10, fn_70004a8a10 as *const (), 0), // +0x4a8a10 sub_4a8a10
    Lifted::new(0x70004a8a90, fn_70004a8a90 as *const (), 2), // +0x4a8a90 sub_4a8a90
    Lifted::new(0x70004a8c00, fn_70004a8c00 as *const (), 0), // +0x4a8c00 sub_4a8c00
    Lifted::new(0x70004a8de0, fn_70004a8de0 as *const (), 2), // +0x4a8de0 sub_4a8de0
    Lifted::new(0x70004a8e40, fn_70004a8e40 as *const (), 1), // +0x4a8e40 sub_4a8e40
    Lifted::new(0x70004a8e50, tpl_118::<0x48, 0x7000301df0> as *const (), 0), // +0x4a8e50 sub_4a8e50
    Lifted::new(0x70004a8e60, fn_70004a8e60 as *const (), 0), // +0x4a8e60 sub_4a8e60
    Lifted::new(0x70004a8e90, tpl_413::<0x50, 0x48, 0x70004ae680> as *const (), 0), // +0x4a8e90 sub_4a8e90
    Lifted::new(0x70004a8ea0, fn_70004a8ea0 as *const (), 0), // +0x4a8ea0 sub_4a8ea0
    Lifted::new(0x70004a8ef0, fn_70004a8ef0 as *const (), 0), // +0x4a8ef0 sub_4a8ef0
    Lifted::new(0x70004a9060, tpl_413::<0x50, 0x48, 0x70004af110> as *const (), 0), // +0x4a9060 sub_4a9060
    Lifted::new(0x70004a9070, fn_70004a9070 as *const (), 0), // +0x4a9070 sub_4a9070
    Lifted::new(0x70004a9100, fn_70004a9100 as *const (), 0), // +0x4a9100 sub_4a9100
    Lifted::new(0x70004a9180, fn_70004a9180 as *const (), 0), // +0x4a9180 sub_4a9180
    Lifted::new(0x70004a9260, fn_70004a9260 as *const (), 3), // +0x4a9260 sub_4a9260
    Lifted::new(0x70004a9290, fn_70004a9290 as *const (), 2), // +0x4a9290 sub_4a9290
    Lifted::new(0x70004a9420, fn_70004a9420 as *const (), 2), // +0x4a9420 sub_4a9420
    Lifted::new(0x70004a94d0, tpl_323::<0x7000619d68, 0x70004a0aa0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x4a94d0 sub_4a94d0
    Lifted::new(0x70004a9550, tpl_192::<0x7000619d78, 0x70004a14c0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x4a9550 sub_4a9550
    Lifted::new(0x70004a95d0, fn_70004a95d0 as *const (), 0), // +0x4a95d0 sub_4a95d0
    Lifted::new(0x70004a9650, fn_70004a9650 as *const (), 0), // +0x4a9650 sub_4a9650
    Lifted::new(0x70004a96f0, fn_70004a96f0 as *const (), 0), // +0x4a96f0 sub_4a96f0
    Lifted::new(0x70004a97a0, tpl_216::<0x7000619dc8, 0x70004a0aa0, 0x48, 0x70004a14c0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x4a97a0 sub_4a97a0
    Lifted::new(0x70004a9830, tpl_162::<0x7000619dd8, 0x70004a0aa0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x4a9830 sub_4a9830
    Lifted::new(0x70004a98b0, fn_70004a98b0 as *const (), 0), // +0x4a98b0 sub_4a98b0
    Lifted::new(0x70004a9940, fn_70004a9940 as *const (), 0), // +0x4a9940 sub_4a9940
    Lifted::new(0x70004a99d0, fn_70004a99d0 as *const (), 0), // +0x4a99d0 sub_4a99d0
    Lifted::new(0x70004a9a50, tpl_162::<0x7000619e28, 0x70004a16d0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x4a9a50 sub_4a9a50
    Lifted::new(0x70004a9ad0, fn_70004a9ad0 as *const (), 0), // +0x4a9ad0 sub_4a9ad0
    Lifted::new(0x70004a9b30, fn_70004a9b30 as *const (), 0), // +0x4a9b30 sub_4a9b30
    Lifted::new(0x70004a9bd0, fn_70004a9bd0 as *const (), 0), // +0x4a9bd0 sub_4a9bd0
    Lifted::new(0x70004a9c50, fn_70004a9c50 as *const (), 0), // +0x4a9c50 sub_4a9c50
    Lifted::new(0x70004a9d00, fn_70004a9d00 as *const (), 0), // +0x4a9d00 sub_4a9d00
    Lifted::new(0x70004a9db0, tpl_323::<0x7000619e98, 0x700048beb0, 0x70005fe088, 0x3c, 0x7000697150, 0x30, 0x10, 0x700048c2c0> as *const (), 0), // +0x4a9db0 sub_4a9db0
    Lifted::new(0x70004a9e30, fn_70004a9e30 as *const (), 0), // +0x4a9e30 sub_4a9e30
    Lifted::new(0x70004a9ee0, fn_70004a9ee0 as *const (), 0), // +0x4a9ee0 sub_4a9ee0
    Lifted::new(0x70004a9f60, fn_70004a9f60 as *const (), 2), // +0x4a9f60 sub_4a9f60
    Lifted::new(0x70004aa0c0, fn_70004aa0c0 as *const (), 2), // +0x4aa0c0 sub_4aa0c0
    Lifted::new(0x70004aa1c0, tpl_308::<0x70005a8240, 0x7000619ba0, 0x70002445b0, 0x70005a8230> as *const (), 0), // +0x4aa1c0 sub_4aa1c0
    Lifted::new(0x70004aa2e0, fn_70004aa2e0 as *const (), 0), // +0x4aa2e0 sub_4aa2e0
    Lifted::new(0x70004aa430, fn_70004aa430 as *const (), 0), // +0x4aa430 sub_4aa430
    Lifted::new(0x70004aa4c0, fn_70004aa4c0 as *const (), 2), // +0x4aa4c0 sub_4aa4c0
    Lifted::new(0x70004aa8c0, fn_70004aa8c0 as *const (), 2), // +0x4aa8c0 sub_4aa8c0
    Lifted::new(0x70004aabb0, fn_70004aabb0 as *const (), 2), // +0x4aabb0 sub_4aabb0
    Lifted::new(0x70004aae70, fn_70004aae70 as *const (), 2), // +0x4aae70 sub_4aae70
    Lifted::new(0x70004ab140, fn_70004ab140 as *const (), 2), // +0x4ab140 sub_4ab140
    Lifted::new(0x70004ab330, fn_70004ab330 as *const (), 0), // +0x4ab330 sub_4ab330
    Lifted::new(0x70004ab480, fn_70004ab480 as *const (), 0), // +0x4ab480 sub_4ab480
    Lifted::new(0x70004ab580, tpl_250::<0x7000694058, 0x70005a83d0, 0x7000694050, 0x8, 0x10> as *const (), 2), // +0x4ab580 sub_4ab580
    Lifted::new(0x70004ab610, fn_70004ab610 as *const (), 2), // +0x4ab610 sub_4ab610
    Lifted::new(0x70004ab750, fn_70004ab750 as *const (), 2), // +0x4ab750 sub_4ab750
    Lifted::new(0x70004aba20, tpl_414::<0x70004abf10, 0x70004aba20, 0x70003f8030, 0x70004acee0> as *const (), 0), // +0x4aba20 sub_4aba20
    Lifted::new(0x70004abbb0, tpl_414::<0x70004ac7d0, 0x70004abbb0, 0x70004ac620, 0x70004ad000> as *const (), 0), // +0x4abbb0 sub_4abbb0
    Lifted::new(0x70004abd40, fn_70004abd40 as *const (), 0), // +0x4abd40 sub_4abd40
    Lifted::new(0x70004abf10, fn_70004abf10 as *const (), 0), // +0x4abf10 sub_4abf10
    Lifted::new(0x70004ac620, fn_70004ac620 as *const (), 0), // +0x4ac620 sub_4ac620
    Lifted::new(0x70004ac7d0, fn_70004ac7d0 as *const (), 0), // +0x4ac7d0 sub_4ac7d0
    Lifted::new(0x70004acee0, fn_70004acee0 as *const (), 0), // +0x4acee0 sub_4acee0
    Lifted::new(0x70004ad000, fn_70004ad000 as *const (), 0), // +0x4ad000 sub_4ad000
    Lifted::new(0x70004ad120, fn_70004ad120 as *const (), 1), // +0x4ad120 sub_4ad120
    Lifted::new(0x70004ad220, fn_70004ad220 as *const (), 1), // +0x4ad220 sub_4ad220
    Lifted::new(0x70004ad3e0, fn_70004ad3e0 as *const (), 1), // +0x4ad3e0 sub_4ad3e0
    Lifted::new(0x70004ad4e0, fn_70004ad4e0 as *const (), 1), // +0x4ad4e0 sub_4ad4e0
    Lifted::new(0x70004ad6a0, fn_70004ad6a0 as *const (), 2), // +0x4ad6a0 sub_4ad6a0
    Lifted::new(0x70004ad7d0, fn_70004ad7d0 as *const (), 0), // +0x4ad7d0 sub_4ad7d0
    Lifted::new(0x70004ad8b0, fn_70004ad8b0 as *const (), 0), // +0x4ad8b0 sub_4ad8b0
    Lifted::new(0x70004ad930, tpl_78::<0x700061a618, 0x700048c2c0> as *const (), 0), // +0x4ad930 sub_4ad930
    Lifted::new(0x70004ad960, fn_70004ad960 as *const (), 0), // +0x4ad960 sub_4ad960
    Lifted::new(0x70004ada00, fn_70004ada00 as *const (), 2), // +0x4ada00 sub_4ada00
    Lifted::new(0x70004ae1d0, fn_70004ae1d0 as *const (), 2), // +0x4ae1d0 sub_4ae1d0
    Lifted::new(0x70004ae2c0, fn_70004ae2c0 as *const (), 2), // +0x4ae2c0 sub_4ae2c0
    Lifted::new(0x70004ae3a0, fn_70004ae3a0 as *const (), 0), // +0x4ae3a0 sub_4ae3a0
    Lifted::new(0x70004ae680, fn_70004ae680 as *const (), 0), // +0x4ae680 sub_4ae680
    Lifted::new(0x70004ae7a0, fn_70004ae7a0 as *const (), 2), // +0x4ae7a0 sub_4ae7a0
    Lifted::new(0x70004af110, fn_70004af110 as *const (), 2), // +0x4af110 sub_4af110
    Lifted::new(0x70004af910, fn_70004af910 as *const (), 0), // +0x4af910 sub_4af910
    Lifted::new(0x70004afa40, fn_70004afa40 as *const (), 2), // +0x4afa40 sub_4afa40
    Lifted::new(0x70004afd60, fn_70004afd60 as *const (), 2), // +0x4afd60 sub_4afd60
    Lifted::new(0x70004b0030, fn_70004b0030 as *const (), 0), // +0x4b0030 sub_4b0030
    Lifted::new(0x70004b0240, fn_70004b0240 as *const (), 0), // +0x4b0240 sub_4b0240
    Lifted::new(0x70004b0430, fn_70004b0430 as *const (), 0), // +0x4b0430 sub_4b0430
    Lifted::new(0x70004b04e0, fn_70004b04e0 as *const (), 0), // +0x4b04e0 sub_4b04e0
    Lifted::new(0x70004b05e0, fn_70004b05e0 as *const (), 0), // +0x4b05e0 sub_4b05e0
    Lifted::new(0x70004b06e0, fn_70004b06e0 as *const (), 0), // +0x4b06e0 sub_4b06e0
    Lifted::new(0x70004b0750, fn_70004b0750 as *const (), 2), // +0x4b0750 sub_4b0750
    Lifted::new(0x70004b0860, tpl_25::<0x7000694974, 0x700069b958, 0x70005a4174, 0x70005a7ae0, 0x700069b960, 0x700069b980, 0x700069b9a0, 0x700069b9c0, 0x70005a40e8, 0x70005a4108> as *const (), 0), // +0x4b0860 sub_4b0860
    Lifted::new(0x70004b08e0, fn_70004b08e0 as *const (), 0), // +0x4b08e0 sub_4b08e0
    Lifted::new(0x70004b0990, tpl_210::<0x70004afd60> as *const (), 0), // +0x4b0990 sub_4b0990
    Lifted::new(0x70004b09b0, fn_70004b09b0 as *const (), 0), // +0x4b09b0 sub_4b09b0
    Lifted::new(0x70004b0a40, fn_70004b0a40 as *const (), 2), // +0x4b0a40 sub_4b0a40
    Lifted::new(0x70004b0d10, tpl_74::<0x10, 0x2, 0x70004b0d40> as *const (), 0), // +0x4b0d10 sub_4b0d10
    Lifted::new(0x70004b0d40, fn_70004b0d40 as *const (), 2), // +0x4b0d40 sub_4b0d40
    Lifted::new(0x70004b1040, fn_70004b1040 as *const (), 0), // +0x4b1040 sub_4b1040
    Lifted::new(0x70004b10d0, fn_70004b10d0 as *const (), 2), // +0x4b10d0 sub_4b10d0
    Lifted::new(0x70004b12e0, fn_70004b12e0 as *const (), 0), // +0x4b12e0 sub_4b12e0
    Lifted::new(0x70004b1370, fn_70004b1370 as *const (), 0), // +0x4b1370 sub_4b1370
    Lifted::new(0x70004b1620, fn_70004b1620 as *const (), 0), // +0x4b1620 sub_4b1620
    Lifted::new(0x70004b17d0, fn_70004b17d0 as *const (), 0), // +0x4b17d0 sub_4b17d0
    Lifted::new(0x70004b18f0, fn_70004b18f0 as *const (), 0), // +0x4b18f0 sub_4b18f0
    Lifted::new(0x70004b1aa0, fn_70004b1aa0 as *const (), 0), // +0x4b1aa0 sub_4b1aa0
    Lifted::new(0x70004b1ca0, fn_70004b1ca0 as *const (), 0), // +0x4b1ca0 sub_4b1ca0
    Lifted::new(0x70004b1e00, fn_70004b1e00 as *const (), 0), // +0x4b1e00 sub_4b1e00
    Lifted::new(0x70004b1f60, fn_70004b1f60 as *const (), 0), // +0x4b1f60 sub_4b1f60
    Lifted::new(0x70004b2280, fn_70004b2280 as *const (), 0), // +0x4b2280 sub_4b2280
    Lifted::new(0x70004b24e0, fn_70004b24e0 as *const (), 2), // +0x4b24e0 sub_4b24e0
    Lifted::new(0x70004b2690, fn_70004b2690 as *const (), 0), // +0x4b2690 sub_4b2690
    Lifted::new(0x70004b2840, fn_70004b2840 as *const (), 0), // +0x4b2840 sub_4b2840
    Lifted::new(0x70004b29f0, fn_70004b29f0 as *const (), 0), // +0x4b29f0 sub_4b29f0
    Lifted::new(0x70004b2b70, fn_70004b2b70 as *const (), 0), // +0x4b2b70 sub_4b2b70
    Lifted::new(0x70004b2bd0, fn_70004b2bd0 as *const (), 0), // +0x4b2bd0 sub_4b2bd0
    Lifted::new(0x70004b2c30, fn_70004b2c30 as *const (), 2), // +0x4b2c30 sub_4b2c30
    Lifted::new(0x70004b34f0, fn_70004b34f0 as *const (), 0), // +0x4b34f0 sub_4b34f0
    Lifted::new(0x70004b3620, fn_70004b3620 as *const (), 0), // +0x4b3620 sub_4b3620
    Lifted::new(0x70004b3750, fn_70004b3750 as *const (), 0), // +0x4b3750 sub_4b3750
    Lifted::new(0x70004b3820, fn_70004b3820 as *const (), 0), // +0x4b3820 sub_4b3820
    Lifted::new(0x70004b3a00, fn_70004b3a00 as *const (), 1), // +0x4b3a00 sub_4b3a00
    Lifted::new(0x70004b3ab0, fn_70004b3ab0 as *const (), 0), // +0x4b3ab0 sub_4b3ab0
    Lifted::new(0x70004b3b40, fn_70004b3b40 as *const (), 0), // +0x4b3b40 sub_4b3b40
    Lifted::new(0x70004b3bd0, fn_70004b3bd0 as *const (), 0), // +0x4b3bd0 sub_4b3bd0
    Lifted::new(0x70004b3c50, fn_70004b3c50 as *const (), 0), // +0x4b3c50 sub_4b3c50
    Lifted::new(0x70004b3f90, fn_70004b3f90 as *const (), 0), // +0x4b3f90 sub_4b3f90
    Lifted::new(0x70004b4150, fn_70004b4150 as *const (), 0), // +0x4b4150 sub_4b4150
    Lifted::new(0x70004b4170, fn_70004b4170 as *const (), 0), // +0x4b4170 sub_4b4170
    Lifted::new(0x70004b41e0, fn_70004b41e0 as *const (), 0), // +0x4b41e0 sub_4b41e0
    Lifted::new(0x70004b4840, fn_70004b4840 as *const (), 0), // +0x4b4840 sub_4b4840
    Lifted::new(0x70004b48a0, fn_70004b48a0 as *const (), 2), // +0x4b48a0 sub_4b48a0
    Lifted::new(0x70004b49b0, fn_70004b49b0 as *const (), 0), // +0x4b49b0 sub_4b49b0
    Lifted::new(0x70004b4c00, fn_70004b4c00 as *const (), 0), // +0x4b4c00 sub_4b4c00
    Lifted::new(0x70004b4c80, fn_70004b4c80 as *const (), 0), // +0x4b4c80 sub_4b4c80
    Lifted::new(0x70004b4ce0, fn_70004b4ce0 as *const (), 1), // +0x4b4ce0 sub_4b4ce0
    Lifted::new(0x70004b4d00, fn_70004b4d00 as *const (), 0), // +0x4b4d00 sub_4b4d00
    Lifted::new(0x70004b4d90, fn_70004b4d90 as *const (), 2), // +0x4b4d90 sub_4b4d90
    Lifted::new(0x70004b5010, fn_70004b5010 as *const (), 0), // +0x4b5010 sub_4b5010
    Lifted::new(0x70004b5120, tpl_205::<0xffffffffffffffff, 0x7, 0x17, 0x70005a8c68, 0xb, 0x70005a8c60, 0x10> as *const (), 0), // +0x4b5120 sub_4b5120
    Lifted::new(0x70004b51d0, fn_70004b51d0 as *const (), 0), // +0x4b51d0 sub_4b51d0
    Lifted::new(0x70004b5280, fn_70004b5280 as *const (), 0), // +0x4b5280 sub_4b5280
    Lifted::new(0x70004b5370, fn_70004b5370 as *const (), 0), // +0x4b5370 sub_4b5370
    Lifted::new(0x70004b5560, fn_70004b5560 as *const (), 0), // +0x4b5560 sub_4b5560
    Lifted::new(0x70004b5660, fn_70004b5660 as *const (), 0), // +0x4b5660 sub_4b5660
    Lifted::new(0x70004b5f50, fn_70004b5f50 as *const (), 0), // +0x4b5f50 sub_4b5f50
    Lifted::new(0x70004b67c0, fn_70004b67c0 as *const (), 0), // +0x4b67c0 sub_4b67c0
    Lifted::new(0x70004b6ab0, fn_70004b6ab0 as *const (), 0), // +0x4b6ab0 sub_4b6ab0
    Lifted::new(0x70004b6cb0, fn_70004b6cb0 as *const (), 0), // +0x4b6cb0 sub_4b6cb0
    Lifted::new(0x70004b6f00, fn_70004b6f00 as *const (), 2), // +0x4b6f00 sub_4b6f00
    Lifted::new(0x70004b73a0, fn_70004b73a0 as *const (), 0), // +0x4b73a0 sub_4b73a0
    Lifted::new(0x70004b7800, tpl_415::<0x70005a8c30, 0x70004b5370, 0x700061abb0, 0x70005a8c70, 0x70005a8b30, 0x70005a8e58, 0x70005a8878, 0x70005a8c40, 0x70001b9110, 0x700004de60, 0x7000697150, 0x700061ab00, 0x700061ab10, 0x70005a8990> as *const (), 2), // +0x4b7800 sub_4b7800
    Lifted::new(0x70004b7b60, tpl_415::<0x70005a8c30, 0x70004b5370, 0x700061ab20, 0x70005a8c70, 0x70005a8b30, 0x70005a8e58, 0x70005a8878, 0x70005a8c40, 0x70001b9110, 0x700004de60, 0x7000697150, 0x700061ab00, 0x700061ab10, 0x70005a8990> as *const (), 2), // +0x4b7b60 sub_4b7b60
    Lifted::new(0x70004b7ec0, fn_70004b7ec0 as *const (), 0), // +0x4b7ec0 sub_4b7ec0
    Lifted::new(0x70004b8090, fn_70004b8090 as *const (), 2), // +0x4b8090 sub_4b8090
    Lifted::new(0x70004b8420, fn_70004b8420 as *const (), 0), // +0x4b8420 sub_4b8420
    Lifted::new(0x70004b8500, fn_70004b8500 as *const (), 0), // +0x4b8500 sub_4b8500
    Lifted::new(0x70004b8730, fn_70004b8730 as *const (), 0), // +0x4b8730 sub_4b8730
    Lifted::new(0x70004b8940, fn_70004b8940 as *const (), 0), // +0x4b8940 sub_4b8940
    Lifted::new(0x70004b8c70, fn_70004b8c70 as *const (), 0), // +0x4b8c70 sub_4b8c70
    Lifted::new(0x70004b8de0, fn_70004b8de0 as *const (), 0), // +0x4b8de0 sub_4b8de0
    Lifted::new(0x70004b93f0, fn_70004b93f0 as *const (), 0), // +0x4b93f0 sub_4b93f0
    Lifted::new(0x70004b9560, fn_70004b9560 as *const (), 0), // +0x4b9560 sub_4b9560
    Lifted::new(0x70004b95f0, tpl_416::<0x70004b93f0, 0x70005a8e58, 0x70005a8970, 0x70005a8868> as *const (), 0), // +0x4b95f0 sub_4b95f0
    Lifted::new(0x70004b9670, tpl_416::<0x70004b93f0, 0x70005a8e58, 0x70005a8970, 0x70005a8860> as *const (), 0), // +0x4b9670 sub_4b9670
    Lifted::new(0x70004b96f0, fn_70004b96f0 as *const (), 2), // +0x4b96f0 sub_4b96f0
    Lifted::new(0x70004b97a0, fn_70004b97a0 as *const (), 2), // +0x4b97a0 sub_4b97a0
    Lifted::new(0x70004b9890, fn_70004b9890 as *const (), 2), // +0x4b9890 sub_4b9890
    Lifted::new(0x70004b9940, fn_70004b9940 as *const (), 2), // +0x4b9940 sub_4b9940
    Lifted::new(0x70004b9a00, tpl_417::<0x700061abc0, 0x700057eb50> as *const (), 0), // +0x4b9a00 sub_4b9a00
    Lifted::new(0x70004b9a10, tpl_418::<0x70004b2bd0, 0x700060dea0, 0x700061abc0, 0x700057ec20> as *const (), 0), // +0x4b9a10 sub_4b9a10
    Lifted::new(0x70004b9a40, tpl_419::<0x10, 0x8, 0x700061abd0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x30, 0x18, 0x38, 0x7000042360, 0x700010b4f0> as *const (), 0), // +0x4b9a40 sub_4b9a40
    Lifted::new(0x70004b9b50, tpl_419::<0x10, 0x8, 0x700061ac30, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x30, 0x18, 0x38, 0x7000043100, 0x700010b4f0> as *const (), 0), // +0x4b9b50 sub_4b9b50
    Lifted::new(0x70004b9c60, fn_70004b9c60 as *const (), 0), // +0x4b9c60 sub_4b9c60
    Lifted::new(0x70004b9f50, fn_70004b9f50 as *const (), 2), // +0x4b9f50 sub_4b9f50
    Lifted::new(0x70004ba090, fn_70004ba090 as *const (), 0), // +0x4ba090 sub_4ba090
    Lifted::new(0x70004ba140, fn_70004ba140 as *const (), 0), // +0x4ba140 sub_4ba140
    Lifted::new(0x70004ba1a0, fn_70004ba1a0 as *const (), 0), // +0x4ba1a0 sub_4ba1a0
    Lifted::new(0x70004ba840, fn_70004ba840 as *const (), 0), // +0x4ba840 sub_4ba840
    Lifted::new(0x70004ba8d0, tpl_420::<0x70004b2bd0, 0x58, 0x8, 0x59, 0x68, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x4ba8d0 sub_4ba8d0
    Lifted::new(0x70004ba950, tpl_421::<0x70004b2bd0, 0x88, 0x8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x4ba950 sub_4ba950
    Lifted::new(0x70004ba9a0, tpl_420::<0x70004b2bd0, 0x70, 0x8, 0x71, 0x80, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x4ba9a0 sub_4ba9a0
    Lifted::new(0x70004baa30, tpl_422::<0x70005a8848, 0x70005a8e58, 0x70005a8e28, 0x70005a8910, 0x700061a940, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70005a8c68, 0x70005a8c60, 0x70004b7800, 0x70004b5010, 0x7000697150, 0x7000613c50, 0x7000613088, 0x70001a4530> as *const (), 2), // +0x4baa30 sub_4baa30
    Lifted::new(0x70004bae90, fn_70004bae90 as *const (), 0), // +0x4bae90 sub_4bae90
    Lifted::new(0x70004baf20, fn_70004baf20 as *const (), 0), // +0x4baf20 sub_4baf20
    Lifted::new(0x70004bb2a0, fn_70004bb2a0 as *const (), 0), // +0x4bb2a0 sub_4bb2a0
    Lifted::new(0x70004bb330, tpl_420::<0x70004b2bd0, 0xd8, 0x8, 0xd9, 0xe8, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x4bb330 sub_4bb330
    Lifted::new(0x70004bb3c0, tpl_420::<0x70004b2bd0, 0xf0, 0x8, 0xf1, 0x100, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x4bb3c0 sub_4bb3c0
    Lifted::new(0x70004bb450, tpl_420::<0x70004b2bd0, 0x108, 0x8, 0x109, 0x118, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x4bb450 sub_4bb450
    Lifted::new(0x70004bb4e0, tpl_420::<0x70004b2bd0, 0x120, 0x8, 0x121, 0x130, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x4bb4e0 sub_4bb4e0
    Lifted::new(0x70004bb570, tpl_423::<0x70004b2bd0, 0x8, 0x7000237d20, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x4bb570 sub_4bb570
    Lifted::new(0x70004bb640, fn_70004bb640 as *const (), 0), // +0x4bb640 sub_4bb640
    Lifted::new(0x70004bb6f0, fn_70004bb6f0 as *const (), 0), // +0x4bb6f0 sub_4bb6f0
    Lifted::new(0x70004bbc10, fn_70004bbc10 as *const (), 0), // +0x4bbc10 sub_4bbc10
    Lifted::new(0x70004bbca0, tpl_424::<0x70004b2bd0, 0x50, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x4bbca0 sub_4bbca0
    Lifted::new(0x70004bbcf0, tpl_420::<0x70004b2bd0, 0x38, 0x8, 0x39, 0x48, 0x70005a8c70, 0x70005a8b30, 0x10, 0x18> as *const (), 0), // +0x4bbcf0 sub_4bbcf0
    Lifted::new(0x70004bbd70, fn_70004bbd70 as *const (), 0), // +0x4bbd70 sub_4bbd70
    Lifted::new(0x70004bc200, fn_70004bc200 as *const (), 0), // +0x4bc200 sub_4bc200
    Lifted::new(0x70004bc910, tpl_425::<0x18, 0x8, 0x70005a8840, 0x10, 0x28, 0x70005a88b0, 0x70001e4150> as *const (), 0), // +0x4bc910 sub_4bc910
    Lifted::new(0x70004bc970, fn_70004bc970 as *const (), 0), // +0x4bc970 sub_4bc970
    Lifted::new(0x70004bccc0, fn_70004bccc0 as *const (), 0), // +0x4bccc0 sub_4bccc0
    Lifted::new(0x70004bcd50, fn_70004bcd50 as *const (), 0), // +0x4bcd50 sub_4bcd50
    Lifted::new(0x70004bcdd0, fn_70004bcdd0 as *const (), 0), // +0x4bcdd0 sub_4bcdd0
    Lifted::new(0x70004bce60, fn_70004bce60 as *const (), 0), // +0x4bce60 sub_4bce60
    Lifted::new(0x70004bcf00, fn_70004bcf00 as *const (), 0), // +0x4bcf00 sub_4bcf00
    Lifted::new(0x70004bd1b0, fn_70004bd1b0 as *const (), 0), // +0x4bd1b0 sub_4bd1b0
    Lifted::new(0x70004bd240, fn_70004bd240 as *const (), 2), // +0x4bd240 sub_4bd240
    Lifted::new(0x70004bd3b0, fn_70004bd3b0 as *const (), 0), // +0x4bd3b0 sub_4bd3b0
    Lifted::new(0x70004bd5f0, fn_70004bd5f0 as *const (), 0), // +0x4bd5f0 sub_4bd5f0
    Lifted::new(0x70004bd680, fn_70004bd680 as *const (), 0), // +0x4bd680 sub_4bd680
    Lifted::new(0x70004bd710, fn_70004bd710 as *const (), 0), // +0x4bd710 sub_4bd710
    Lifted::new(0x70004bd770, tpl_426::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xe8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x4bd770 sub_4bd770
    Lifted::new(0x70004bd7e0, tpl_427::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x38, 0x30, 0xe8> as *const (), 2), // +0x4bd7e0 sub_4bd7e0
    Lifted::new(0x70004bd870, fn_70004bd870 as *const (), 0), // +0x4bd870 sub_4bd870
    Lifted::new(0x70004bd900, fn_70004bd900 as *const (), 0), // +0x4bd900 sub_4bd900
    Lifted::new(0x70004bd9a0, fn_70004bd9a0 as *const (), 0), // +0x4bd9a0 sub_4bd9a0
    Lifted::new(0x70004bda00, fn_70004bda00 as *const (), 0), // +0x4bda00 sub_4bda00
    Lifted::new(0x70004bda80, fn_70004bda80 as *const (), 0), // +0x4bda80 sub_4bda80
    Lifted::new(0x70004bdae0, fn_70004bdae0 as *const (), 0), // +0x4bdae0 sub_4bdae0
    Lifted::new(0x70004bdb60, tpl_426::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xe4, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x4bdb60 sub_4bdb60
    Lifted::new(0x70004bdbd0, tpl_427::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x38, 0x30, 0xe4> as *const (), 2), // +0x4bdbd0 sub_4bdbd0
    Lifted::new(0x70004bdc60, tpl_428::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xc8, 0x70006013d8, 0x70001a7da0, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x4bdc60 sub_4bdc60
    Lifted::new(0x70004bdd50, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x7000242230, 0x7000697150> as *const (), 0), // +0x4bdd50 sub_4bdd50
    Lifted::new(0x70004bdde0, tpl_428::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xcc, 0x7000601390, 0x70001a7da0, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x4bdde0 sub_4bdde0
    Lifted::new(0x70004bded0, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x7000242380, 0x7000697150> as *const (), 0), // +0x4bded0 sub_4bded0
    Lifted::new(0x70004bdf60, tpl_426::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xf8, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x4bdf60 sub_4bdf60
    Lifted::new(0x70004bdfd0, tpl_427::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x38, 0x30, 0xf8> as *const (), 2), // +0x4bdfd0 sub_4bdfd0
    Lifted::new(0x70004be060, tpl_423::<0x70004b2bd0, 0x8, 0x7000241b20, 0x1, 0x10, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x18> as *const (), 0), // +0x4be060 sub_4be060
    Lifted::new(0x70004be130, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x70002419a0, 0x7000697150> as *const (), 0), // +0x4be130 sub_4be130
    Lifted::new(0x70004be1c0, tpl_428::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xc0, 0x70006013f0, 0x70001a7da0, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x4be1c0 sub_4be1c0
    Lifted::new(0x70004be2b0, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x7000241dd0, 0x7000697150> as *const (), 0), // +0x4be2b0 sub_4be2b0
    Lifted::new(0x70004be340, tpl_428::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xc4, 0x70006013a8, 0x70001a7da0, 0x70005a8c70, 0x70005a8b30, 0x7000697150, 0x10, 0x18> as *const (), 0), // +0x4be340 sub_4be340
    Lifted::new(0x70004be430, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x7000241fd0, 0x7000697150> as *const (), 0), // +0x4be430 sub_4be430
    Lifted::new(0x70004be4c0, tpl_426::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xec, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x4be4c0 sub_4be4c0
    Lifted::new(0x70004be530, tpl_427::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x38, 0x30, 0xec> as *const (), 2), // +0x4be530 sub_4be530
    Lifted::new(0x70004be5c0, tpl_426::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xf0, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x4be5c0 sub_4be5c0
    Lifted::new(0x70004be630, tpl_427::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x38, 0x30, 0xf0> as *const (), 2), // +0x4be630 sub_4be630
    Lifted::new(0x70004be6c0, tpl_426::<0x70004b2bd0, 0x38, 0x30, 0x8, 0xf4, 0x70005a8b10, 0x10, 0x18> as *const (), 2), // +0x4be6c0 sub_4be6c0
    Lifted::new(0x70004be730, tpl_427::<0x70004b2bd0, 0x10, 0x8, 0x70005a8e58, 0x70005a8990, 0x38, 0x30, 0xf4> as *const (), 2), // +0x4be730 sub_4be730
    Lifted::new(0x70004be7c0, fn_70004be7c0 as *const (), 2), // +0x4be7c0 sub_4be7c0
    Lifted::new(0x70004be980, fn_70004be980 as *const (), 2), // +0x4be980 sub_4be980
    Lifted::new(0x70004beaf0, fn_70004beaf0 as *const (), 0), // +0x4beaf0 sub_4beaf0
    Lifted::new(0x70004beb50, fn_70004beb50 as *const (), 0), // +0x4beb50 sub_4beb50
    Lifted::new(0x70004bebc0, fn_70004bebc0 as *const (), 2), // +0x4bebc0 sub_4bebc0
    Lifted::new(0x70004befe0, fn_70004befe0 as *const (), 2), // +0x4befe0 sub_4befe0
    Lifted::new(0x70004bf360, fn_70004bf360 as *const (), 0), // +0x4bf360 sub_4bf360
    Lifted::new(0x70004bf480, fn_70004bf480 as *const (), 2), // +0x4bf480 sub_4bf480
    Lifted::new(0x70004bf8a0, fn_70004bf8a0 as *const (), 2), // +0x4bf8a0 sub_4bf8a0
    Lifted::new(0x70004bfc10, fn_70004bfc10 as *const (), 0), // +0x4bfc10 sub_4bfc10
    Lifted::new(0x70004bfce0, fn_70004bfce0 as *const (), 0), // +0x4bfce0 sub_4bfce0
    Lifted::new(0x70004bfdd0, fn_70004bfdd0 as *const (), 2), // +0x4bfdd0 sub_4bfdd0
    Lifted::new(0x70004c0150, fn_70004c0150 as *const (), 2), // +0x4c0150 sub_4c0150
    Lifted::new(0x70004c05b0, fn_70004c05b0 as *const (), 0), // +0x4c05b0 sub_4c05b0
    Lifted::new(0x70004c0740, tpl_430::<0x10, 0x8, 0x700061b2f0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70002412f0> as *const (), 0), // +0x4c0740 sub_4c0740
    Lifted::new(0x70004c0810, fn_70004c0810 as *const (), 2), // +0x4c0810 sub_4c0810
    Lifted::new(0x70004c0a80, tpl_431::<0x700061b3d0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e58, 0x70005a8990, 0x70001b9110, 0x700023fed0, 0x7000697150> as *const (), 2), // +0x4c0a80 sub_4c0a80
    Lifted::new(0x70004c0d50, fn_70004c0d50 as *const (), 2), // +0x4c0d50 sub_4c0d50
    Lifted::new(0x70004c0f30, tpl_432::<0x700061b850, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x70002402e0, 0x7000697150> as *const (), 0), // +0x4c0f30 sub_4c0f30
    Lifted::new(0x70004c1120, fn_70004c1120 as *const (), 2), // +0x4c1120 sub_4c1120
    Lifted::new(0x70004c1300, fn_70004c1300 as *const (), 2), // +0x4c1300 sub_4c1300
    Lifted::new(0x70004c1600, fn_70004c1600 as *const (), 2), // +0x4c1600 sub_4c1600
    Lifted::new(0x70004c18e0, fn_70004c18e0 as *const (), 0), // +0x4c18e0 sub_4c18e0
    Lifted::new(0x70004c19f0, fn_70004c19f0 as *const (), 2), // +0x4c19f0 sub_4c19f0
    Lifted::new(0x70004c1b20, fn_70004c1b20 as *const (), 2), // +0x4c1b20 sub_4c1b20
    Lifted::new(0x70004c1c90, fn_70004c1c90 as *const (), 2), // +0x4c1c90 sub_4c1c90
    Lifted::new(0x70004c1e70, fn_70004c1e70 as *const (), 2), // +0x4c1e70 sub_4c1e70
    Lifted::new(0x70004c2290, tpl_430::<0x10, 0x8, 0x700061bc40, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x7000241570> as *const (), 0), // +0x4c2290 sub_4c2290
    Lifted::new(0x70004c2360, fn_70004c2360 as *const (), 2), // +0x4c2360 sub_4c2360
    Lifted::new(0x70004c2750, tpl_431::<0x700061b9a0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e58, 0x70005a8990, 0x70001b9110, 0x70002400c0, 0x7000697150> as *const (), 2), // +0x4c2750 sub_4c2750
    Lifted::new(0x70004c2a20, fn_70004c2a20 as *const (), 2), // +0x4c2a20 sub_4c2a20
    Lifted::new(0x70004c2e40, fn_70004c2e40 as *const (), 2), // +0x4c2e40 sub_4c2e40
    Lifted::new(0x70004c3020, fn_70004c3020 as *const (), 0), // +0x4c3020 sub_4c3020
    Lifted::new(0x70004c3460, fn_70004c3460 as *const (), 2), // +0x4c3460 sub_4c3460
    Lifted::new(0x70004c4a60, fn_70004c4a60 as *const (), 0), // +0x4c4a60 sub_4c4a60
    Lifted::new(0x70004c6d10, fn_70004c6d10 as *const (), 0), // +0x4c6d10 sub_4c6d10
    Lifted::new(0x70004c6da0, fn_70004c6da0 as *const (), 0), // +0x4c6da0 sub_4c6da0
    Lifted::new(0x70004c6e20, tpl_424::<0x70004b2bd0, 0x20, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x4c6e20 sub_4c6e20
    Lifted::new(0x70004c6e70, fn_70004c6e70 as *const (), 0), // +0x4c6e70 sub_4c6e70
    Lifted::new(0x70004c7170, fn_70004c7170 as *const (), 0), // +0x4c7170 sub_4c7170
    Lifted::new(0x70004c7200, tpl_429::<0x70004b2bd0, 0x10, 0x8, 0x70001b9110, 0x700023a6b0, 0x7000697150> as *const (), 0), // +0x4c7200 sub_4c7200
    Lifted::new(0x70004c7290, tpl_424::<0x70004b2bd0, 0xf0, 0x8, 0x70005a8958, 0x10, 0x18> as *const (), 0), // +0x4c7290 sub_4c7290
    Lifted::new(0x70004c72e0, tpl_433::<0x70004b2bd0, 0x70003d7a00, 0x7000697150> as *const (), 0), // +0x4c72e0 sub_4c72e0
    Lifted::new(0x70004c7460, tpl_433::<0x70004b2bd0, 0x70003d7af0, 0x7000697150> as *const (), 0), // +0x4c7460 sub_4c7460
    Lifted::new(0x70004c75e0, fn_70004c75e0 as *const (), 0), // +0x4c75e0 sub_4c75e0
    Lifted::new(0x70004c7720, fn_70004c7720 as *const (), 0), // +0x4c7720 sub_4c7720
    Lifted::new(0x70004c78c0, fn_70004c78c0 as *const (), 0), // +0x4c78c0 sub_4c78c0
    Lifted::new(0x70004c7a30, fn_70004c7a30 as *const (), 0), // +0x4c7a30 sub_4c7a30
    Lifted::new(0x70004c7c20, fn_70004c7c20 as *const (), 0), // +0x4c7c20 sub_4c7c20
    Lifted::new(0x70004c7e60, fn_70004c7e60 as *const (), 0), // +0x4c7e60 sub_4c7e60
    Lifted::new(0x70004c7f50, fn_70004c7f50 as *const (), 0), // +0x4c7f50 sub_4c7f50
    Lifted::new(0x70004c86c0, fn_70004c86c0 as *const (), 0), // +0x4c86c0 sub_4c86c0
    Lifted::new(0x70004c8750, tpl_434::<0x700061c2c0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70001b9110, 0x700020f7f0, 0x7000697150> as *const (), 0), // +0x4c8750 sub_4c8750
    Lifted::new(0x70004c8870, fn_70004c8870 as *const (), 0), // +0x4c8870 sub_4c8870
    Lifted::new(0x70004c8ab0, fn_70004c8ab0 as *const (), 0), // +0x4c8ab0 sub_4c8ab0
    Lifted::new(0x70004c8b40, fn_70004c8b40 as *const (), 0), // +0x4c8b40 sub_4c8b40
    Lifted::new(0x70004c8e70, fn_70004c8e70 as *const (), 0), // +0x4c8e70 sub_4c8e70
    Lifted::new(0x70004c8f00, fn_70004c8f00 as *const (), 0), // +0x4c8f00 sub_4c8f00
    Lifted::new(0x70004c9080, tpl_435::<0x700061c510, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e30, 0x70001328c0, 0x7000694974, 0x700004f420, 0x70005a8ca0, 0x70005705b0, 0x70005a8af8, 0x7000242e70, 0x70004c8f00, 0x70001327c0, 0x70005a8e38> as *const (), 2), // +0x4c9080 sub_4c9080
    Lifted::new(0x70004c9350, tpl_435::<0x700061c570, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e30, 0x70001328c0, 0x7000694974, 0x700004f420, 0x70005a8ca0, 0x70005705b0, 0x70005a8af8, 0x7000242c90, 0x70004c8f00, 0x70001327c0, 0x70005a8e38> as *const (), 2), // +0x4c9350 sub_4c9350
    Lifted::new(0x70004c9620, tpl_435::<0x700061c5d0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e30, 0x70001328c0, 0x7000694974, 0x700004f420, 0x70005a8ca0, 0x70005705b0, 0x70005a8af8, 0x7000242dd0, 0x70004c8f00, 0x70001327c0, 0x70005a8e38> as *const (), 2), // +0x4c9620 sub_4c9620
    Lifted::new(0x70004c98f0, tpl_435::<0x700061c630, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e30, 0x70001328c0, 0x7000694974, 0x700004f420, 0x70005a8ca0, 0x70005705b0, 0x70005a8af8, 0x7000242bf0, 0x70004c8f00, 0x70001327c0, 0x70005a8e38> as *const (), 2), // +0x4c98f0 sub_4c98f0
    Lifted::new(0x70004c9bc0, tpl_435::<0x700061c390, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e30, 0x70001328c0, 0x7000694974, 0x700004f420, 0x70005a8ca0, 0x70005705b0, 0x70005a8af8, 0x7000242bf0, 0x70004c8f00, 0x70001327c0, 0x70005a8e38> as *const (), 2), // +0x4c9bc0 sub_4c9bc0
    Lifted::new(0x70004c9e90, tpl_435::<0x700061c3f0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e30, 0x70001328c0, 0x7000694974, 0x700004f420, 0x70005a8ca0, 0x70005705b0, 0x70005a8af8, 0x70002431e0, 0x70004c8f00, 0x70001327c0, 0x70005a8e38> as *const (), 2), // +0x4c9e90 sub_4c9e90
    Lifted::new(0x70004ca160, tpl_435::<0x700061c450, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e30, 0x70001328c0, 0x7000694974, 0x700004f420, 0x70005a8ca0, 0x70005705b0, 0x70005a8af8, 0x70002435f0, 0x70004c8f00, 0x70001327c0, 0x70005a8e38> as *const (), 2), // +0x4ca160 sub_4ca160
    Lifted::new(0x70004ca430, tpl_435::<0x700061c4b0, 0x7000231b20, 0x70005a8c70, 0x70005a8b30, 0x70005a88d0, 0x70005a8950, 0x70004b2bd0, 0x70005a8e30, 0x70001328c0, 0x7000694974, 0x700004f420, 0x70005a8ca0, 0x70005705b0, 0x70005a8af8, 0x7000242d30, 0x70004c8f00, 0x70001327c0, 0x70005a8e38> as *const (), 2), // +0x4ca430 sub_4ca430
    Lifted::new(0x70004ca700, fn_70004ca700 as *const (), 0), // +0x4ca700 sub_4ca700
    Lifted::new(0x70004cad20, fn_70004cad20 as *const (), 0), // +0x4cad20 sub_4cad20
    Lifted::new(0x70004cadb0, fn_70004cadb0 as *const (), 0), // +0x4cadb0 sub_4cadb0
    Lifted::new(0x70004cae00, fn_70004cae00 as *const (), 0), // +0x4cae00 sub_4cae00
    Lifted::new(0x70004caea0, fn_70004caea0 as *const (), 0), // +0x4caea0 sub_4caea0
    Lifted::new(0x70004cb0d0, fn_70004cb0d0 as *const (), 0), // +0x4cb0d0 sub_4cb0d0
    Lifted::new(0x70004cb160, tpl_436::<0x70006974b8, 0x70005a8240, 0x700061c8e0, 0x70002445b0, 0x70005a8230> as *const (), 0), // +0x4cb160 sub_4cb160
    Lifted::new(0x70004cb290, tpl_436::<0x70006974b8, 0x70005a8240, 0x700061c940, 0x70002445b0, 0x70005a8230> as *const (), 0), // +0x4cb290 sub_4cb290
    Lifted::new(0x70004cb3c0, fn_70004cb3c0 as *const (), 0), // +0x4cb3c0 sub_4cb3c0
    Lifted::new(0x70004cb470, fn_70004cb470 as *const (), 0), // +0x4cb470 sub_4cb470
    Lifted::new(0x70004cb530, fn_70004cb530 as *const (), 0), // +0x4cb530 sub_4cb530
    Lifted::new(0x70004cb610, fn_70004cb610 as *const (), 0), // +0x4cb610 sub_4cb610
    Lifted::new(0x70004cb6d0, fn_70004cb6d0 as *const (), 0), // +0x4cb6d0 sub_4cb6d0
    Lifted::new(0x70004cb7d0, fn_70004cb7d0 as *const (), 0), // +0x4cb7d0 sub_4cb7d0
    Lifted::new(0x70004cb890, fn_70004cb890 as *const (), 0), // +0x4cb890 sub_4cb890
    Lifted::new(0x70004cb960, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000467480, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cb960 sub_4cb960
    Lifted::new(0x70004cbb30, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000467d00, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cbb30 sub_4cbb30
    Lifted::new(0x70004cbd00, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000466d80, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cbd00 sub_4cbd00
    Lifted::new(0x70004cbed0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000467880, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cbed0 sub_4cbed0
    Lifted::new(0x70004cc0a0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000466f50, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cc0a0 sub_4cc0a0
    Lifted::new(0x70004cc270, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000466910, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cc270 sub_4cc270
    Lifted::new(0x70004cc440, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000469090, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cc440 sub_4cc440
    Lifted::new(0x70004cc610, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000468180, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cc610 sub_4cc610
    Lifted::new(0x70004cc7e0, tpl_438::<0x7000573620, 0x7000570b30, 0x7000697150, 0x700045b8e0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cc7e0 sub_4cc7e0
    Lifted::new(0x70004cc9b0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047d5b0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cc9b0 sub_4cc9b0
    Lifted::new(0x70004ccb80, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700045c700, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ccb80 sub_4ccb80
    Lifted::new(0x70004ccd50, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047d7f0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ccd50 sub_4ccd50
    Lifted::new(0x70004ccf20, tpl_438::<0x7000573980, 0x7000570b30, 0x7000697150, 0x700045bec0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ccf20 sub_4ccf20
    Lifted::new(0x70004cd0f0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700045c580, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cd0f0 sub_4cd0f0
    Lifted::new(0x70004cd2c0, fn_70004cd2c0 as *const (), 0), // +0x4cd2c0 sub_4cd2c0
    Lifted::new(0x70004cd4c0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000456410, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cd4c0 sub_4cd4c0
    Lifted::new(0x70004cd690, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047cf70, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cd690 sub_4cd690
    Lifted::new(0x70004cd860, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700045b0f0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cd860 sub_4cd860
    Lifted::new(0x70004cda30, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700045aa40, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cda30 sub_4cda30
    Lifted::new(0x70004cdc00, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047ca30, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cdc00 sub_4cdc00
    Lifted::new(0x70004cddd0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000456a10, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cddd0 sub_4cddd0
    Lifted::new(0x70004cdfa0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000455050, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cdfa0 sub_4cdfa0
    Lifted::new(0x70004ce170, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x70004576f0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ce170 sub_4ce170
    Lifted::new(0x70004ce340, tpl_439::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692f28, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ce340 sub_4ce340
    Lifted::new(0x70004ce540, tpl_440::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692f48, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ce540 sub_4ce540
    Lifted::new(0x70004ce740, tpl_441::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692ff0, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ce740 sub_4ce740
    Lifted::new(0x70004ce940, tpl_440::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000693080, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ce940 sub_4ce940
    Lifted::new(0x70004ceb40, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700045d7d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ceb40 sub_4ceb40
    Lifted::new(0x70004ced10, tpl_442::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692f10, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4ced10 sub_4ced10
    Lifted::new(0x70004cef10, tpl_441::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000693050, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cef10 sub_4cef10
    Lifted::new(0x70004cf110, tpl_440::<0x7000573610, 0x7000570b30, 0x7000697150, 0x70006930d0, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cf110 sub_4cf110
    Lifted::new(0x70004cf310, tpl_440::<0x7000573610, 0x7000570b30, 0x7000697150, 0x70006930a8, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cf310 sub_4cf310
    Lifted::new(0x70004cf510, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x70004551e0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cf510 sub_4cf510
    Lifted::new(0x70004cf6e0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x70004699a0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cf6e0 sub_4cf6e0
    Lifted::new(0x70004cf8b0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000469e10, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cf8b0 sub_4cf8b0
    Lifted::new(0x70004cfa80, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700046e600, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cfa80 sub_4cfa80
    Lifted::new(0x70004cfc50, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047c420, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cfc50 sub_4cfc50
    Lifted::new(0x70004cfe20, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700045e320, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cfe20 sub_4cfe20
    Lifted::new(0x70004cfff0, tpl_442::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692d88, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4cfff0 sub_4cfff0
    Lifted::new(0x70004d01f0, tpl_439::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692fd0, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d01f0 sub_4d01f0
    Lifted::new(0x70004d03f0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700045e230, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d03f0 sub_4d03f0
    Lifted::new(0x70004d05c0, tpl_440::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692ac8, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d05c0 sub_4d05c0
    Lifted::new(0x70004d07c0, fn_70004d07c0 as *const (), 0), // +0x4d07c0 sub_4d07c0
    Lifted::new(0x70004d09a0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700045d510, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d09a0 sub_4d09a0
    Lifted::new(0x70004d0b70, fn_70004d0b70 as *const (), 0), // +0x4d0b70 sub_4d0b70
    Lifted::new(0x70004d0d70, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000454530, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d0d70 sub_4d0d70
    Lifted::new(0x70004d0f40, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x70004587c0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d0f40 sub_4d0f40
    Lifted::new(0x70004d1110, tpl_440::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692b80, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d1110 sub_4d1110
    Lifted::new(0x70004d1310, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x70004667a0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d1310 sub_4d1310
    Lifted::new(0x70004d14e0, tpl_442::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692d38, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d14e0 sub_4d14e0
    Lifted::new(0x70004d16e0, fn_70004d16e0 as *const (), 0), // +0x4d16e0 sub_4d16e0
    Lifted::new(0x70004d18c0, tpl_441::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000693298, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d18c0 sub_4d18c0
    Lifted::new(0x70004d1ac0, tpl_439::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692af0, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d1ac0 sub_4d1ac0
    Lifted::new(0x70004d1cc0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x70004581f0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d1cc0 sub_4d1cc0
    Lifted::new(0x70004d1e90, tpl_441::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692ba8, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d1e90 sub_4d1e90
    Lifted::new(0x70004d2090, tpl_438::<0x7000573cf0, 0x7000570b30, 0x7000697150, 0x7000465c50, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d2090 sub_4d2090
    Lifted::new(0x70004d2260, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700046a380, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d2260 sub_4d2260
    Lifted::new(0x70004d2430, tpl_442::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692d50, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d2430 sub_4d2430
    Lifted::new(0x70004d2630, tpl_439::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692b60, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d2630 sub_4d2630
    Lifted::new(0x70004d2830, tpl_441::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692bd8, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d2830 sub_4d2830
    Lifted::new(0x70004d2a30, tpl_442::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692ab0, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d2a30 sub_4d2a30
    Lifted::new(0x70004d2c30, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700046ed90, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d2c30 sub_4d2c30
    Lifted::new(0x70004d2e00, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700046f0f0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d2e00 sub_4d2e00
    Lifted::new(0x70004d2fd0, fn_70004d2fd0 as *const (), 0), // +0x4d2fd0 sub_4d2fd0
    Lifted::new(0x70004d31d0, tpl_442::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000692b10, 0x70002af6d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d31d0 sub_4d31d0
    Lifted::new(0x70004d33d0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000470340, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d33d0 sub_4d33d0
    Lifted::new(0x70004d35a0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000478b90, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d35a0 sub_4d35a0
    Lifted::new(0x70004d3770, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000479fb0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d3770 sub_4d3770
    Lifted::new(0x70004d3940, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000472a30, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d3940 sub_4d3940
    Lifted::new(0x70004d3b10, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000472020, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d3b10 sub_4d3b10
    Lifted::new(0x70004d3ce0, fn_70004d3ce0 as *const (), 0), // +0x4d3ce0 sub_4d3ce0
    Lifted::new(0x70004d3eb0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000473e90, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d3eb0 sub_4d3eb0
    Lifted::new(0x70004d4080, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047ed80, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d4080 sub_4d4080
    Lifted::new(0x70004d4250, fn_70004d4250 as *const (), 2), // +0x4d4250 sub_4d4250
    Lifted::new(0x70004d4440, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700046f350, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d4440 sub_4d4440
    Lifted::new(0x70004d4610, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047f9d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d4610 sub_4d4610
    Lifted::new(0x70004d47e0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x7000476810, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d47e0 sub_4d47e0
    Lifted::new(0x70004d49b0, fn_70004d49b0 as *const (), 0), // +0x4d49b0 sub_4d49b0
    Lifted::new(0x70004d4bc0, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047fee0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d4bc0 sub_4d4bc0
    Lifted::new(0x70004d4d90, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700046fe40, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d4d90 sub_4d4d90
    Lifted::new(0x70004d4f60, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047a3c0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d4f60 sub_4d4f60
    Lifted::new(0x70004d5130, tpl_437::<0x7000573610, 0x7000570b30, 0x7000697150, 0x700047b2d0, 0x700060e440, 0x70005a83d8, 0x700060e238, 0x700004de60> as *const (), 0), // +0x4d5130 sub_4d5130
    Lifted::new(0x70004d5300, fn_70004d5300 as *const (), 2), // +0x4d5300 sub_4d5300
    Lifted::new(0x70004d53c0, fn_70004d53c0 as *const (), 0), // +0x4d53c0 sub_4d53c0
    Lifted::new(0x70004d5510, fn_70004d5510 as *const (), 0), // +0x4d5510 sub_4d5510
    Lifted::new(0x70004d5650, fn_70004d5650 as *const (), 0), // +0x4d5650 sub_4d5650
    Lifted::new(0x70004d5700, fn_70004d5700 as *const (), 0), // +0x4d5700 sub_4d5700
    Lifted::new(0x70004d59c0, fn_70004d59c0 as *const (), 0), // +0x4d59c0 sub_4d59c0
    Lifted::new(0x70004d5a70, fn_70004d5a70 as *const (), 0), // +0x4d5a70 sub_4d5a70
    Lifted::new(0x70004d5d30, fn_70004d5d30 as *const (), 0), // +0x4d5d30 sub_4d5d30
    Lifted::new(0x70004d5de0, fn_70004d5de0 as *const (), 0), // +0x4d5de0 sub_4d5de0
    Lifted::new(0x70004d60a0, fn_70004d60a0 as *const (), 0), // +0x4d60a0 sub_4d60a0
    Lifted::new(0x70004d6150, fn_70004d6150 as *const (), 0), // +0x4d6150 sub_4d6150
    Lifted::new(0x70004d6410, fn_70004d6410 as *const (), 2), // +0x4d6410 sub_4d6410
    Lifted::new(0x70004d6540, tpl_443::<0x70004b2bd0, 0x70004d4250, 0x70006974b8, 0x700061ca50, 0x700004f620, 0x70001328c0, 0x7000471bf0, 0x70003c6fc0, 0x70001a2990, 0x70001327c0, 0x7000132a00, 0x70005a4d14, 0x700061caf0, 0x7000572bb0, 0x70003c6e20, 0x700061ca90, 0x7000576ad0, 0x7000697150> as *const (), 0), // +0x4d6540 sub_4d6540
    Lifted::new(0x70004d6800, fn_70004d6800 as *const (), 0), // +0x4d6800 sub_4d6800
    Lifted::new(0x70004d6910, fn_70004d6910 as *const (), 2), // +0x4d6910 sub_4d6910
    Lifted::new(0x70004d6be0, fn_70004d6be0 as *const (), 0), // +0x4d6be0 sub_4d6be0
    Lifted::new(0x70004d6cf0, fn_70004d6cf0 as *const (), 2), // +0x4d6cf0 sub_4d6cf0
    Lifted::new(0x70004d6fc0, fn_70004d6fc0 as *const (), 0), // +0x4d6fc0 sub_4d6fc0
    Lifted::new(0x70004d70d0, fn_70004d70d0 as *const (), 2), // +0x4d70d0 sub_4d70d0
    Lifted::new(0x70004d73a0, fn_70004d73a0 as *const (), 0), // +0x4d73a0 sub_4d73a0
    Lifted::new(0x70004d74b0, fn_70004d74b0 as *const (), 2), // +0x4d74b0 sub_4d74b0
    Lifted::new(0x70004d7780, fn_70004d7780 as *const (), 2), // +0x4d7780 sub_4d7780
    Lifted::new(0x70004d78f0, fn_70004d78f0 as *const (), 2), // +0x4d78f0 sub_4d78f0
    Lifted::new(0x70004d7be0, fn_70004d7be0 as *const (), 0), // +0x4d7be0 sub_4d7be0
    Lifted::new(0x70004d7c90, fn_70004d7c90 as *const (), 0), // +0x4d7c90 sub_4d7c90
    Lifted::new(0x70004d7f50, fn_70004d7f50 as *const (), 0), // +0x4d7f50 sub_4d7f50
    Lifted::new(0x70004d8000, fn_70004d8000 as *const (), 0), // +0x4d8000 sub_4d8000
    Lifted::new(0x70004d82c0, fn_70004d82c0 as *const (), 0), // +0x4d82c0 sub_4d82c0
    Lifted::new(0x70004d8370, fn_70004d8370 as *const (), 0), // +0x4d8370 sub_4d8370
    Lifted::new(0x70004d8630, fn_70004d8630 as *const (), 0), // +0x4d8630 sub_4d8630
    Lifted::new(0x70004d86e0, fn_70004d86e0 as *const (), 0), // +0x4d86e0 sub_4d86e0
    Lifted::new(0x70004d89a0, fn_70004d89a0 as *const (), 0), // +0x4d89a0 sub_4d89a0
    Lifted::new(0x70004d8b60, fn_70004d8b60 as *const (), 0), // +0x4d8b60 sub_4d8b60
    Lifted::new(0x70004d8e10, fn_70004d8e10 as *const (), 2), // +0x4d8e10 sub_4d8e10
    Lifted::new(0x70004d8f40, tpl_444::<0x70004b2bd0, 0x70004d3940, 0x70006974b8, 0x700061d218, 0x700004f620, 0x70001328c0, 0x7000473990, 0x70003c6fc0, 0x70001a2990, 0x70001327c0, 0x7000132a00, 0x70005a4d14, 0x700061d2b0, 0x7000572bb0, 0x70003c6e20, 0x700061d250, 0x7000576ad0, 0x7000697150> as *const (), 2), // +0x4d8f40 sub_4d8f40
    Lifted::new(0x70004d9200, fn_70004d9200 as *const (), 2), // +0x4d9200 sub_4d9200
    Lifted::new(0x70004d9320, tpl_444::<0x70004b2bd0, 0x70004d3940, 0x70006974b8, 0x700061d578, 0x700004f620, 0x70001328c0, 0x7000472fd0, 0x70003c6fc0, 0x70001a2990, 0x70001327c0, 0x7000132a00, 0x70005a4d14, 0x700061d610, 0x7000572bb0, 0x70003c6e20, 0x700061d5b0, 0x7000576ad0, 0x7000697150> as *const (), 2), // +0x4d9320 sub_4d9320
    Lifted::new(0x70004d95e0, fn_70004d95e0 as *const (), 2), // +0x4d95e0 sub_4d95e0
    Lifted::new(0x70004d9710, tpl_444::<0x70004b2bd0, 0x70004d3940, 0x70006974b8, 0x700061d668, 0x700004f620, 0x70001328c0, 0x7000473310, 0x70003c6fc0, 0x70001a2990, 0x70001327c0, 0x7000132a00, 0x70005a4d14, 0x700061d490, 0x7000572bb0, 0x70003c6e20, 0x700061d430, 0x7000576ad0, 0x7000697150> as *const (), 2), // +0x4d9710 sub_4d9710
    Lifted::new(0x70004d99d0, fn_70004d99d0 as *const (), 2), // +0x4d99d0 sub_4d99d0
    Lifted::new(0x70004d9af0, tpl_444::<0x70004b2bd0, 0x70004d3940, 0x70006974b8, 0x700061d4e8, 0x700004f620, 0x70001328c0, 0x7000473650, 0x70003c6fc0, 0x70001a2990, 0x70001327c0, 0x7000132a00, 0x70005a4d14, 0x700061d7c0, 0x7000572bb0, 0x70003c6e20, 0x700061d520, 0x7000576ad0, 0x7000697150> as *const (), 2), // +0x4d9af0 sub_4d9af0
    Lifted::new(0x70004d9db0, fn_70004d9db0 as *const (), 2), // +0x4d9db0 sub_4d9db0
    Lifted::new(0x70004d9ea0, tpl_444::<0x70004b2bd0, 0x70004d3940, 0x70006974b8, 0x700061d810, 0x700004f620, 0x70001328c0, 0x7000473cd0, 0x70003c6fc0, 0x70001a2990, 0x70001327c0, 0x7000132a00, 0x70005a4d14, 0x700061d890, 0x7000572bb0, 0x70003c6e20, 0x700061d840, 0x7000576ad0, 0x7000697150> as *const (), 2), // +0x4d9ea0 sub_4d9ea0
    Lifted::new(0x70004da160, fn_70004da160 as *const (), 0), // +0x4da160 sub_4da160
    Lifted::new(0x70004da2c0, fn_70004da2c0 as *const (), 0), // +0x4da2c0 sub_4da2c0
    Lifted::new(0x70004da580, fn_70004da580 as *const (), 0), // +0x4da580 sub_4da580
    Lifted::new(0x70004da6d0, fn_70004da6d0 as *const (), 0), // +0x4da6d0 sub_4da6d0
    Lifted::new(0x70004da990, fn_70004da990 as *const (), 0), // +0x4da990 sub_4da990
    Lifted::new(0x70004daaa0, fn_70004daaa0 as *const (), 2), // +0x4daaa0 sub_4daaa0
    Lifted::new(0x70004dad70, fn_70004dad70 as *const (), 0), // +0x4dad70 sub_4dad70
    Lifted::new(0x70004dae80, fn_70004dae80 as *const (), 2), // +0x4dae80 sub_4dae80
    Lifted::new(0x70004db150, fn_70004db150 as *const (), 0), // +0x4db150 sub_4db150
    Lifted::new(0x70004db2b0, fn_70004db2b0 as *const (), 0), // +0x4db2b0 sub_4db2b0
    Lifted::new(0x70004db590, fn_70004db590 as *const (), 0), // +0x4db590 sub_4db590
    Lifted::new(0x70004db640, fn_70004db640 as *const (), 0), // +0x4db640 sub_4db640
    Lifted::new(0x70004db900, fn_70004db900 as *const (), 0), // +0x4db900 sub_4db900
    Lifted::new(0x70004db9b0, fn_70004db9b0 as *const (), 0), // +0x4db9b0 sub_4db9b0
    Lifted::new(0x70004dbc70, fn_70004dbc70 as *const (), 0), // +0x4dbc70 sub_4dbc70
    Lifted::new(0x70004dbd20, fn_70004dbd20 as *const (), 0), // +0x4dbd20 sub_4dbd20
    Lifted::new(0x70004dbfe0, fn_70004dbfe0 as *const (), 0), // +0x4dbfe0 sub_4dbfe0
    Lifted::new(0x70004dc090, fn_70004dc090 as *const (), 0), // +0x4dc090 sub_4dc090
    Lifted::new(0x70004dc350, fn_70004dc350 as *const (), 2), // +0x4dc350 sub_4dc350
    Lifted::new(0x70004dc480, tpl_443::<0x70004b2bd0, 0x70004d4610, 0x70006974b8, 0x700061dba8, 0x700004f620, 0x70001328c0, 0x7000476000, 0x70003c6fc0, 0x70001a2990, 0x70001327c0, 0x7000132a00, 0x70005a4d14, 0x700061de80, 0x7000572bb0, 0x70003c6e20, 0x700061dbe0, 0x7000576ad0, 0x7000697150> as *const (), 0), // +0x4dc480 sub_4dc480
    Lifted::new(0x70004dc740, fn_70004dc740 as *const (), 0), // +0x4dc740 sub_4dc740
    Lifted::new(0x70004dc850, fn_70004dc850 as *const (), 0), // +0x4dc850 sub_4dc850
    Lifted::new(0x70004dcb10, fn_70004dcb10 as *const (), 0), // +0x4dcb10 sub_4dcb10
    Lifted::new(0x70004dcc20, fn_70004dcc20 as *const (), 0), // +0x4dcc20 sub_4dcc20
    Lifted::new(0x70004dcee0, fn_70004dcee0 as *const (), 0), // +0x4dcee0 sub_4dcee0
    Lifted::new(0x70004dcff0, fn_70004dcff0 as *const (), 0), // +0x4dcff0 sub_4dcff0
    Lifted::new(0x70004dd2b0, fn_70004dd2b0 as *const (), 0), // +0x4dd2b0 sub_4dd2b0
    Lifted::new(0x70004dd3c0, fn_70004dd3c0 as *const (), 0), // +0x4dd3c0 sub_4dd3c0
    Lifted::new(0x70004dd680, fn_70004dd680 as *const (), 2), // +0x4dd680 sub_4dd680
    Lifted::new(0x70004dd780, fn_70004dd780 as *const (), 0), // +0x4dd780 sub_4dd780
    Lifted::new(0x70004dda40, fn_70004dda40 as *const (), 0), // +0x4dda40 sub_4dda40
    Lifted::new(0x70004ddb40, fn_70004ddb40 as *const (), 0), // +0x4ddb40 sub_4ddb40
    Lifted::new(0x70004dde40, fn_70004dde40 as *const (), 0), // +0x4dde40 sub_4dde40
    Lifted::new(0x70004ddef0, fn_70004ddef0 as *const (), 0), // +0x4ddef0 sub_4ddef0
    Lifted::new(0x70004de1b0, fn_70004de1b0 as *const (), 0), // +0x4de1b0 sub_4de1b0
    Lifted::new(0x70004de260, fn_70004de260 as *const (), 0), // +0x4de260 sub_4de260
    Lifted::new(0x70004de520, fn_70004de520 as *const (), 0), // +0x4de520 sub_4de520
    Lifted::new(0x70004de5d0, fn_70004de5d0 as *const (), 0), // +0x4de5d0 sub_4de5d0
    Lifted::new(0x70004de890, fn_70004de890 as *const (), 0), // +0x4de890 sub_4de890
    Lifted::new(0x70004de940, fn_70004de940 as *const (), 0), // +0x4de940 sub_4de940
    Lifted::new(0x70004dec00, fn_70004dec00 as *const (), 2), // +0x4dec00 sub_4dec00
    Lifted::new(0x70004dedd0, fn_70004dedd0 as *const (), 0), // +0x4dedd0 sub_4dedd0
    Lifted::new(0x70004deeb0, fn_70004deeb0 as *const (), 2), // +0x4deeb0 sub_4deeb0
    Lifted::new(0x70004df080, fn_70004df080 as *const (), 0), // +0x4df080 sub_4df080
    Lifted::new(0x70004df160, fn_70004df160 as *const (), 2), // +0x4df160 sub_4df160
    Lifted::new(0x70004df330, fn_70004df330 as *const (), 0), // +0x4df330 sub_4df330
    Lifted::new(0x70004df410, fn_70004df410 as *const (), 2), // +0x4df410 sub_4df410
    Lifted::new(0x70004df5e0, fn_70004df5e0 as *const (), 0), // +0x4df5e0 sub_4df5e0
    Lifted::new(0x70004df6c0, fn_70004df6c0 as *const (), 2), // +0x4df6c0 sub_4df6c0
    Lifted::new(0x70004df890, fn_70004df890 as *const (), 2), // +0x4df890 sub_4df890
    Lifted::new(0x70004df980, fn_70004df980 as *const (), 2), // +0x4df980 sub_4df980
    Lifted::new(0x70004dfb50, fn_70004dfb50 as *const (), 2), // +0x4dfb50 sub_4dfb50
    Lifted::new(0x70004dfc40, fn_70004dfc40 as *const (), 2), // +0x4dfc40 sub_4dfc40
    Lifted::new(0x70004dfe10, fn_70004dfe10 as *const (), 2), // +0x4dfe10 sub_4dfe10
    Lifted::new(0x70004dff00, fn_70004dff00 as *const (), 2), // +0x4dff00 sub_4dff00
    Lifted::new(0x70004e00d0, fn_70004e00d0 as *const (), 2), // +0x4e00d0 sub_4e00d0
    Lifted::new(0x70004e01c0, fn_70004e01c0 as *const (), 2), // +0x4e01c0 sub_4e01c0
    Lifted::new(0x70004e0390, fn_70004e0390 as *const (), 2), // +0x4e0390 sub_4e0390
    Lifted::new(0x70004e0480, fn_70004e0480 as *const (), 2), // +0x4e0480 sub_4e0480
    Lifted::new(0x70004e0650, fn_70004e0650 as *const (), 2), // +0x4e0650 sub_4e0650
    Lifted::new(0x70004e0740, fn_70004e0740 as *const (), 2), // +0x4e0740 sub_4e0740
    Lifted::new(0x70004e0910, fn_70004e0910 as *const (), 2), // +0x4e0910 sub_4e0910
    Lifted::new(0x70004e0a00, fn_70004e0a00 as *const (), 2), // +0x4e0a00 sub_4e0a00
    Lifted::new(0x70004e0bd0, fn_70004e0bd0 as *const (), 2), // +0x4e0bd0 sub_4e0bd0
    Lifted::new(0x70004e0cc0, fn_70004e0cc0 as *const (), 2), // +0x4e0cc0 sub_4e0cc0
    Lifted::new(0x70004e0e40, fn_70004e0e40 as *const (), 2), // +0x4e0e40 sub_4e0e40
    Lifted::new(0x70004e0f30, fn_70004e0f30 as *const (), 2), // +0x4e0f30 sub_4e0f30
    Lifted::new(0x70004e10b0, fn_70004e10b0 as *const (), 2), // +0x4e10b0 sub_4e10b0
    Lifted::new(0x70004e11a0, fn_70004e11a0 as *const (), 2), // +0x4e11a0 sub_4e11a0
    Lifted::new(0x70004e1320, fn_70004e1320 as *const (), 2), // +0x4e1320 sub_4e1320
    Lifted::new(0x70004e1410, fn_70004e1410 as *const (), 2), // +0x4e1410 sub_4e1410
    Lifted::new(0x70004e1590, fn_70004e1590 as *const (), 2), // +0x4e1590 sub_4e1590
    Lifted::new(0x70004e1680, fn_70004e1680 as *const (), 2), // +0x4e1680 sub_4e1680
    Lifted::new(0x70004e1800, fn_70004e1800 as *const (), 2), // +0x4e1800 sub_4e1800
    Lifted::new(0x70004e18f0, fn_70004e18f0 as *const (), 2), // +0x4e18f0 sub_4e18f0
    Lifted::new(0x70004e1a70, fn_70004e1a70 as *const (), 2), // +0x4e1a70 sub_4e1a70
    Lifted::new(0x70004e1b60, fn_70004e1b60 as *const (), 2), // +0x4e1b60 sub_4e1b60
    Lifted::new(0x70004e1ce0, fn_70004e1ce0 as *const (), 2), // +0x4e1ce0 sub_4e1ce0
    Lifted::new(0x70004e1dd0, fn_70004e1dd0 as *const (), 2), // +0x4e1dd0 sub_4e1dd0
    Lifted::new(0x70004e1f50, fn_70004e1f50 as *const (), 2), // +0x4e1f50 sub_4e1f50
    Lifted::new(0x70004e2040, fn_70004e2040 as *const (), 0), // +0x4e2040 sub_4e2040
    Lifted::new(0x70004e20f0, fn_70004e20f0 as *const (), 0), // +0x4e20f0 sub_4e20f0
    Lifted::new(0x70004e23b0, fn_70004e23b0 as *const (), 0), // +0x4e23b0 sub_4e23b0
    Lifted::new(0x70004e2460, fn_70004e2460 as *const (), 0), // +0x4e2460 sub_4e2460
    Lifted::new(0x70004e2720, fn_70004e2720 as *const (), 0), // +0x4e2720 sub_4e2720
    Lifted::new(0x70004e27d0, fn_70004e27d0 as *const (), 0), // +0x4e27d0 sub_4e27d0
    Lifted::new(0x70004e2a90, fn_70004e2a90 as *const (), 0), // +0x4e2a90 sub_4e2a90
    Lifted::new(0x70004e2b40, fn_70004e2b40 as *const (), 0), // +0x4e2b40 sub_4e2b40
    Lifted::new(0x70004e2e00, fn_70004e2e00 as *const (), 0), // +0x4e2e00 sub_4e2e00
    Lifted::new(0x70004e2eb0, fn_70004e2eb0 as *const (), 0), // +0x4e2eb0 sub_4e2eb0
    Lifted::new(0x70004e3170, fn_70004e3170 as *const (), 0), // +0x4e3170 sub_4e3170
    Lifted::new(0x70004e3220, fn_70004e3220 as *const (), 0), // +0x4e3220 sub_4e3220
    Lifted::new(0x70004e34e0, fn_70004e34e0 as *const (), 2), // +0x4e34e0 sub_4e34e0
    Lifted::new(0x70004e36b0, fn_70004e36b0 as *const (), 0), // +0x4e36b0 sub_4e36b0
    Lifted::new(0x70004e3790, fn_70004e3790 as *const (), 2), // +0x4e3790 sub_4e3790
    Lifted::new(0x70004e3960, fn_70004e3960 as *const (), 0), // +0x4e3960 sub_4e3960
    Lifted::new(0x70004e3a40, fn_70004e3a40 as *const (), 2), // +0x4e3a40 sub_4e3a40
    Lifted::new(0x70004e3c10, fn_70004e3c10 as *const (), 0), // +0x4e3c10 sub_4e3c10
    Lifted::new(0x70004e3cf0, fn_70004e3cf0 as *const (), 2), // +0x4e3cf0 sub_4e3cf0
    Lifted::new(0x70004e3ec0, fn_70004e3ec0 as *const (), 0), // +0x4e3ec0 sub_4e3ec0
    Lifted::new(0x70004e3fa0, fn_70004e3fa0 as *const (), 2), // +0x4e3fa0 sub_4e3fa0
    Lifted::new(0x70004e4170, fn_70004e4170 as *const (), 0), // +0x4e4170 sub_4e4170
    Lifted::new(0x70004e4250, fn_70004e4250 as *const (), 2), // +0x4e4250 sub_4e4250
    Lifted::new(0x70004e4420, fn_70004e4420 as *const (), 0), // +0x4e4420 sub_4e4420
    Lifted::new(0x70004e4500, fn_70004e4500 as *const (), 2), // +0x4e4500 sub_4e4500
    Lifted::new(0x70004e46d0, fn_70004e46d0 as *const (), 2), // +0x4e46d0 sub_4e46d0
    Lifted::new(0x70004e47c0, fn_70004e47c0 as *const (), 2), // +0x4e47c0 sub_4e47c0
    Lifted::new(0x70004e4990, fn_70004e4990 as *const (), 2), // +0x4e4990 sub_4e4990
    Lifted::new(0x70004e4a80, fn_70004e4a80 as *const (), 2), // +0x4e4a80 sub_4e4a80
    Lifted::new(0x70004e4c50, fn_70004e4c50 as *const (), 2), // +0x4e4c50 sub_4e4c50
    Lifted::new(0x70004e4d40, fn_70004e4d40 as *const (), 2), // +0x4e4d40 sub_4e4d40
    Lifted::new(0x70004e4f10, fn_70004e4f10 as *const (), 2), // +0x4e4f10 sub_4e4f10
    Lifted::new(0x70004e5000, fn_70004e5000 as *const (), 2), // +0x4e5000 sub_4e5000
    Lifted::new(0x70004e51d0, fn_70004e51d0 as *const (), 2), // +0x4e51d0 sub_4e51d0
    Lifted::new(0x70004e52c0, fn_70004e52c0 as *const (), 2), // +0x4e52c0 sub_4e52c0
    Lifted::new(0x70004e5490, fn_70004e5490 as *const (), 2), // +0x4e5490 sub_4e5490
    Lifted::new(0x70004e5580, fn_70004e5580 as *const (), 2), // +0x4e5580 sub_4e5580
    Lifted::new(0x70004e5750, fn_70004e5750 as *const (), 2), // +0x4e5750 sub_4e5750
    Lifted::new(0x70004e5840, fn_70004e5840 as *const (), 2), // +0x4e5840 sub_4e5840
    Lifted::new(0x70004e5a10, fn_70004e5a10 as *const (), 2), // +0x4e5a10 sub_4e5a10
    Lifted::new(0x70004e5b00, fn_70004e5b00 as *const (), 2), // +0x4e5b00 sub_4e5b00
    Lifted::new(0x70004e5cd0, fn_70004e5cd0 as *const (), 2), // +0x4e5cd0 sub_4e5cd0
    Lifted::new(0x70004e5dc0, fn_70004e5dc0 as *const (), 2), // +0x4e5dc0 sub_4e5dc0
    Lifted::new(0x70004e5f90, fn_70004e5f90 as *const (), 2), // +0x4e5f90 sub_4e5f90
    Lifted::new(0x70004e6080, fn_70004e6080 as *const (), 2), // +0x4e6080 sub_4e6080
    Lifted::new(0x70004e6250, fn_70004e6250 as *const (), 2), // +0x4e6250 sub_4e6250
    Lifted::new(0x70004e6340, fn_70004e6340 as *const (), 2), // +0x4e6340 sub_4e6340
    Lifted::new(0x70004e6510, fn_70004e6510 as *const (), 2), // +0x4e6510 sub_4e6510
    Lifted::new(0x70004e6600, fn_70004e6600 as *const (), 2), // +0x4e6600 sub_4e6600
    Lifted::new(0x70004e6780, fn_70004e6780 as *const (), 2), // +0x4e6780 sub_4e6780
    Lifted::new(0x70004e6870, fn_70004e6870 as *const (), 2), // +0x4e6870 sub_4e6870
    Lifted::new(0x70004e69f0, fn_70004e69f0 as *const (), 2), // +0x4e69f0 sub_4e69f0
    Lifted::new(0x70004e6ae0, fn_70004e6ae0 as *const (), 2), // +0x4e6ae0 sub_4e6ae0
    Lifted::new(0x70004e6c60, fn_70004e6c60 as *const (), 2), // +0x4e6c60 sub_4e6c60
    Lifted::new(0x70004e6d50, fn_70004e6d50 as *const (), 2), // +0x4e6d50 sub_4e6d50
    Lifted::new(0x70004e6ed0, fn_70004e6ed0 as *const (), 2), // +0x4e6ed0 sub_4e6ed0
    Lifted::new(0x70004e6fc0, fn_70004e6fc0 as *const (), 2), // +0x4e6fc0 sub_4e6fc0
    Lifted::new(0x70004e7140, fn_70004e7140 as *const (), 2), // +0x4e7140 sub_4e7140
    Lifted::new(0x70004e7230, fn_70004e7230 as *const (), 2), // +0x4e7230 sub_4e7230
    Lifted::new(0x70004e73b0, fn_70004e73b0 as *const (), 2), // +0x4e73b0 sub_4e73b0
    Lifted::new(0x70004e74a0, fn_70004e74a0 as *const (), 2), // +0x4e74a0 sub_4e74a0
    Lifted::new(0x70004e7620, fn_70004e7620 as *const (), 2), // +0x4e7620 sub_4e7620
    Lifted::new(0x70004e7710, fn_70004e7710 as *const (), 2), // +0x4e7710 sub_4e7710
    Lifted::new(0x70004e7890, fn_70004e7890 as *const (), 2), // +0x4e7890 sub_4e7890
    Lifted::new(0x70004e7980, fn_70004e7980 as *const (), 2), // +0x4e7980 sub_4e7980
    Lifted::new(0x70004e7b00, fn_70004e7b00 as *const (), 2), // +0x4e7b00 sub_4e7b00
    Lifted::new(0x70004e7bf0, fn_70004e7bf0 as *const (), 2), // +0x4e7bf0 sub_4e7bf0
    Lifted::new(0x70004e7d70, fn_70004e7d70 as *const (), 2), // +0x4e7d70 sub_4e7d70
    Lifted::new(0x70004e7e60, fn_70004e7e60 as *const (), 2), // +0x4e7e60 sub_4e7e60
    Lifted::new(0x70004e7fe0, fn_70004e7fe0 as *const (), 2), // +0x4e7fe0 sub_4e7fe0
    Lifted::new(0x70004e80d0, fn_70004e80d0 as *const (), 2), // +0x4e80d0 sub_4e80d0
    Lifted::new(0x70004e8250, fn_70004e8250 as *const (), 2), // +0x4e8250 sub_4e8250
    Lifted::new(0x70004e8340, fn_70004e8340 as *const (), 0), // +0x4e8340 sub_4e8340
    Lifted::new(0x70004e8440, fn_70004e8440 as *const (), 2), // +0x4e8440 sub_4e8440
    Lifted::new(0x70004e8740, fn_70004e8740 as *const (), 0), // +0x4e8740 sub_4e8740
    Lifted::new(0x70004e8840, fn_70004e8840 as *const (), 2), // +0x4e8840 sub_4e8840
    Lifted::new(0x70004e8b20, fn_70004e8b20 as *const (), 0), // +0x4e8b20 sub_4e8b20
    Lifted::new(0x70004e8c20, fn_70004e8c20 as *const (), 0), // +0x4e8c20 sub_4e8c20
    Lifted::new(0x70004e8f20, fn_70004e8f20 as *const (), 0), // +0x4e8f20 sub_4e8f20
    Lifted::new(0x70004e9020, fn_70004e9020 as *const (), 0), // +0x4e9020 sub_4e9020
    Lifted::new(0x70004e9320, fn_70004e9320 as *const (), 2), // +0x4e9320 sub_4e9320
    Lifted::new(0x70004e9490, fn_70004e9490 as *const (), 0), // +0x4e9490 sub_4e9490
    Lifted::new(0x70004e97d0, fn_70004e97d0 as *const (), 0), // +0x4e97d0 sub_4e97d0
    Lifted::new(0x70004e98d0, fn_70004e98d0 as *const (), 0), // +0x4e98d0 sub_4e98d0
    Lifted::new(0x70004e9bd0, fn_70004e9bd0 as *const (), 0), // +0x4e9bd0 sub_4e9bd0
    Lifted::new(0x70004e9c80, fn_70004e9c80 as *const (), 0), // +0x4e9c80 sub_4e9c80
    Lifted::new(0x70004e9f40, fn_70004e9f40 as *const (), 0), // +0x4e9f40 sub_4e9f40
    Lifted::new(0x70004e9ff0, fn_70004e9ff0 as *const (), 0), // +0x4e9ff0 sub_4e9ff0
    Lifted::new(0x70004ea2b0, fn_70004ea2b0 as *const (), 0), // +0x4ea2b0 sub_4ea2b0
    Lifted::new(0x70004ea360, fn_70004ea360 as *const (), 0), // +0x4ea360 sub_4ea360
    Lifted::new(0x70004ea620, fn_70004ea620 as *const (), 0), // +0x4ea620 sub_4ea620
    Lifted::new(0x70004ea6d0, fn_70004ea6d0 as *const (), 0), // +0x4ea6d0 sub_4ea6d0
    Lifted::new(0x70004ea990, fn_70004ea990 as *const (), 2), // +0x4ea990 sub_4ea990
    Lifted::new(0x70004eaaa0, fn_70004eaaa0 as *const (), 0), // +0x4eaaa0 sub_4eaaa0
    Lifted::new(0x70004ead60, fn_70004ead60 as *const (), 0), // +0x4ead60 sub_4ead60
    Lifted::new(0x70004eae10, fn_70004eae10 as *const (), 2), // +0x4eae10 sub_4eae10
    Lifted::new(0x70004eb170, fn_70004eb170 as *const (), 0), // +0x4eb170 sub_4eb170
    Lifted::new(0x70004eb280, fn_70004eb280 as *const (), 0), // +0x4eb280 sub_4eb280
    Lifted::new(0x70004eb540, fn_70004eb540 as *const (), 0), // +0x4eb540 sub_4eb540
    Lifted::new(0x70004eb650, fn_70004eb650 as *const (), 0), // +0x4eb650 sub_4eb650
    Lifted::new(0x70004eb910, fn_70004eb910 as *const (), 0), // +0x4eb910 sub_4eb910
    Lifted::new(0x70004eba20, fn_70004eba20 as *const (), 0), // +0x4eba20 sub_4eba20
    Lifted::new(0x70004ebce0, fn_70004ebce0 as *const (), 0), // +0x4ebce0 sub_4ebce0
    Lifted::new(0x70004ebdf0, fn_70004ebdf0 as *const (), 0), // +0x4ebdf0 sub_4ebdf0
    Lifted::new(0x70004ec0b0, fn_70004ec0b0 as *const (), 0), // +0x4ec0b0 sub_4ec0b0
    Lifted::new(0x70004ec1d0, fn_70004ec1d0 as *const (), 0), // +0x4ec1d0 sub_4ec1d0
    Lifted::new(0x70004ec4f0, fn_70004ec4f0 as *const (), 0), // +0x4ec4f0 sub_4ec4f0
    Lifted::new(0x70004ec5f0, fn_70004ec5f0 as *const (), 0), // +0x4ec5f0 sub_4ec5f0
    Lifted::new(0x70004ec8e0, fn_70004ec8e0 as *const (), 0), // +0x4ec8e0 sub_4ec8e0
    Lifted::new(0x70004ec9e0, fn_70004ec9e0 as *const (), 0), // +0x4ec9e0 sub_4ec9e0
    Lifted::new(0x70004eccd0, fn_70004eccd0 as *const (), 0), // +0x4eccd0 sub_4eccd0
    Lifted::new(0x70004ecde0, fn_70004ecde0 as *const (), 0), // +0x4ecde0 sub_4ecde0
    Lifted::new(0x70004ed0a0, fn_70004ed0a0 as *const (), 0), // +0x4ed0a0 sub_4ed0a0
    Lifted::new(0x70004ed1b0, fn_70004ed1b0 as *const (), 2), // +0x4ed1b0 sub_4ed1b0
    Lifted::new(0x70004ed480, fn_70004ed480 as *const (), 2), // +0x4ed480 sub_4ed480
    Lifted::new(0x70004ed610, fn_70004ed610 as *const (), 2), // +0x4ed610 sub_4ed610
    Lifted::new(0x70004ed8d0, fn_70004ed8d0 as *const (), 0), // +0x4ed8d0 sub_4ed8d0
    Lifted::new(0x70004eda30, fn_70004eda30 as *const (), 0), // +0x4eda30 sub_4eda30
    Lifted::new(0x70004edcf0, fn_70004edcf0 as *const (), 0), // +0x4edcf0 sub_4edcf0
    Lifted::new(0x70004ede40, fn_70004ede40 as *const (), 0), // +0x4ede40 sub_4ede40
    Lifted::new(0x70004ee100, fn_70004ee100 as *const (), 0), // +0x4ee100 sub_4ee100
    Lifted::new(0x70004ee250, fn_70004ee250 as *const (), 0), // +0x4ee250 sub_4ee250
    Lifted::new(0x70004ee510, fn_70004ee510 as *const (), 0), // +0x4ee510 sub_4ee510
    Lifted::new(0x70004ee610, fn_70004ee610 as *const (), 0), // +0x4ee610 sub_4ee610
    Lifted::new(0x70004ee900, fn_70004ee900 as *const (), 0), // +0x4ee900 sub_4ee900
    Lifted::new(0x70004eea50, fn_70004eea50 as *const (), 0), // +0x4eea50 sub_4eea50
    Lifted::new(0x70004eed10, fn_70004eed10 as *const (), 0), // +0x4eed10 sub_4eed10
    Lifted::new(0x70004eee70, fn_70004eee70 as *const (), 0), // +0x4eee70 sub_4eee70
    Lifted::new(0x70004ef130, fn_70004ef130 as *const (), 0), // +0x4ef130 sub_4ef130
    Lifted::new(0x70004ef280, fn_70004ef280 as *const (), 0), // +0x4ef280 sub_4ef280
    Lifted::new(0x70004ef540, fn_70004ef540 as *const (), 0), // +0x4ef540 sub_4ef540
    Lifted::new(0x70004ef5f0, fn_70004ef5f0 as *const (), 0), // +0x4ef5f0 sub_4ef5f0
    Lifted::new(0x70004ef8b0, fn_70004ef8b0 as *const (), 0), // +0x4ef8b0 sub_4ef8b0
    Lifted::new(0x70004ef960, fn_70004ef960 as *const (), 0), // +0x4ef960 sub_4ef960
    Lifted::new(0x70004efc20, fn_70004efc20 as *const (), 0), // +0x4efc20 sub_4efc20
    Lifted::new(0x70004efd60, fn_70004efd60 as *const (), 0), // +0x4efd60 sub_4efd60
    Lifted::new(0x70004f0020, fn_70004f0020 as *const (), 0), // +0x4f0020 sub_4f0020
    Lifted::new(0x70004f0120, fn_70004f0120 as *const (), 0), // +0x4f0120 sub_4f0120
    Lifted::new(0x70004f03f0, fn_70004f03f0 as *const (), 0), // +0x4f03f0 sub_4f03f0
    Lifted::new(0x70004f0520, fn_70004f0520 as *const (), 0), // +0x4f0520 sub_4f0520
    Lifted::new(0x70004f07e0, fn_70004f07e0 as *const (), 2), // +0x4f07e0 sub_4f07e0
    Lifted::new(0x70004f0900, fn_70004f0900 as *const (), 0), // +0x4f0900 sub_4f0900
    Lifted::new(0x70004f0bc0, fn_70004f0bc0 as *const (), 0), // +0x4f0bc0 sub_4f0bc0
    Lifted::new(0x70004f0d10, fn_70004f0d10 as *const (), 0), // +0x4f0d10 sub_4f0d10
    Lifted::new(0x70004f0fd0, fn_70004f0fd0 as *const (), 2), // +0x4f0fd0 sub_4f0fd0
    Lifted::new(0x70004f1180, fn_70004f1180 as *const (), 0), // +0x4f1180 sub_4f1180
    Lifted::new(0x70004f1460, fn_70004f1460 as *const (), 0), // +0x4f1460 sub_4f1460
    Lifted::new(0x70004f1610, fn_70004f1610 as *const (), 0), // +0x4f1610 sub_4f1610
    Lifted::new(0x70004f1910, fn_70004f1910 as *const (), 0), // +0x4f1910 sub_4f1910
    Lifted::new(0x70004f1a20, fn_70004f1a20 as *const (), 0), // +0x4f1a20 sub_4f1a20
    Lifted::new(0x70004f1ce0, fn_70004f1ce0 as *const (), 0), // +0x4f1ce0 sub_4f1ce0
    Lifted::new(0x70004f1e90, fn_70004f1e90 as *const (), 0), // +0x4f1e90 sub_4f1e90
    Lifted::new(0x70004f2150, fn_70004f2150 as *const (), 0), // +0x4f2150 sub_4f2150
    Lifted::new(0x70004f22a0, fn_70004f22a0 as *const (), 0), // +0x4f22a0 sub_4f22a0
    Lifted::new(0x70004f2560, fn_70004f2560 as *const (), 0), // +0x4f2560 sub_4f2560
    Lifted::new(0x70004f26a0, fn_70004f26a0 as *const (), 0), // +0x4f26a0 sub_4f26a0
    Lifted::new(0x70004f2960, fn_70004f2960 as *const (), 0), // +0x4f2960 sub_4f2960
    Lifted::new(0x70004f2ac0, fn_70004f2ac0 as *const (), 0), // +0x4f2ac0 sub_4f2ac0
    Lifted::new(0x70004f2d80, fn_70004f2d80 as *const (), 0), // +0x4f2d80 sub_4f2d80
    Lifted::new(0x70004f2ed0, fn_70004f2ed0 as *const (), 0), // +0x4f2ed0 sub_4f2ed0
    Lifted::new(0x70004f3190, fn_70004f3190 as *const (), 0), // +0x4f3190 sub_4f3190
    Lifted::new(0x70004f32f0, fn_70004f32f0 as *const (), 0), // +0x4f32f0 sub_4f32f0
    Lifted::new(0x70004f35b0, fn_70004f35b0 as *const (), 0), // +0x4f35b0 sub_4f35b0
    Lifted::new(0x70004f3700, fn_70004f3700 as *const (), 0), // +0x4f3700 sub_4f3700
    Lifted::new(0x70004f39c0, fn_70004f39c0 as *const (), 0), // +0x4f39c0 sub_4f39c0
    Lifted::new(0x70004f3b00, fn_70004f3b00 as *const (), 0), // +0x4f3b00 sub_4f3b00
    Lifted::new(0x70004f3dc0, fn_70004f3dc0 as *const (), 0), // +0x4f3dc0 sub_4f3dc0
    Lifted::new(0x70004f3ec0, fn_70004f3ec0 as *const (), 0), // +0x4f3ec0 sub_4f3ec0
    Lifted::new(0x70004f4190, fn_70004f4190 as *const (), 0), // +0x4f4190 sub_4f4190
    Lifted::new(0x70004f4240, fn_70004f4240 as *const (), 0), // +0x4f4240 sub_4f4240
    Lifted::new(0x70004f4500, fn_70004f4500 as *const (), 0), // +0x4f4500 sub_4f4500
    Lifted::new(0x70004f4640, fn_70004f4640 as *const (), 0), // +0x4f4640 sub_4f4640
    Lifted::new(0x70004f4900, fn_70004f4900 as *const (), 0), // +0x4f4900 sub_4f4900
    Lifted::new(0x70004f4a10, fn_70004f4a10 as *const (), 2), // +0x4f4a10 sub_4f4a10
    Lifted::new(0x70004f4ce0, fn_70004f4ce0 as *const (), 0), // +0x4f4ce0 sub_4f4ce0
    Lifted::new(0x70004f4d90, fn_70004f4d90 as *const (), 0), // +0x4f4d90 sub_4f4d90
    Lifted::new(0x70004f5050, fn_70004f5050 as *const (), 0), // +0x4f5050 sub_4f5050
    Lifted::new(0x70004f5180, fn_70004f5180 as *const (), 0), // +0x4f5180 sub_4f5180
    Lifted::new(0x70004f5440, fn_70004f5440 as *const (), 2), // +0x4f5440 sub_4f5440
    Lifted::new(0x70004f5570, fn_70004f5570 as *const (), 2), // +0x4f5570 sub_4f5570
    Lifted::new(0x70004f5830, fn_70004f5830 as *const (), 2), // +0x4f5830 sub_4f5830
    Lifted::new(0x70004f5a00, fn_70004f5a00 as *const (), 0), // +0x4f5a00 sub_4f5a00
    Lifted::new(0x70004f5ae0, fn_70004f5ae0 as *const (), 2), // +0x4f5ae0 sub_4f5ae0
    Lifted::new(0x70004f5cb0, fn_70004f5cb0 as *const (), 2), // +0x4f5cb0 sub_4f5cb0
    Lifted::new(0x70004f5da0, fn_70004f5da0 as *const (), 2), // +0x4f5da0 sub_4f5da0
    Lifted::new(0x70004f5f70, fn_70004f5f70 as *const (), 2), // +0x4f5f70 sub_4f5f70
    Lifted::new(0x70004f6060, fn_70004f6060 as *const (), 2), // +0x4f6060 sub_4f6060
    Lifted::new(0x70004f61e0, fn_70004f61e0 as *const (), 2), // +0x4f61e0 sub_4f61e0
    Lifted::new(0x70004f62d0, fn_70004f62d0 as *const (), 2), // +0x4f62d0 sub_4f62d0
    Lifted::new(0x70004f6450, fn_70004f6450 as *const (), 2), // +0x4f6450 sub_4f6450
    Lifted::new(0x70004f6540, fn_70004f6540 as *const (), 2), // +0x4f6540 sub_4f6540
    Lifted::new(0x70004f6710, fn_70004f6710 as *const (), 0), // +0x4f6710 sub_4f6710
    Lifted::new(0x70004f67f0, fn_70004f67f0 as *const (), 2), // +0x4f67f0 sub_4f67f0
    Lifted::new(0x70004f69c0, fn_70004f69c0 as *const (), 2), // +0x4f69c0 sub_4f69c0
    Lifted::new(0x70004f6ab0, fn_70004f6ab0 as *const (), 2), // +0x4f6ab0 sub_4f6ab0
    Lifted::new(0x70004f6c80, fn_70004f6c80 as *const (), 2), // +0x4f6c80 sub_4f6c80
    Lifted::new(0x70004f6d70, fn_70004f6d70 as *const (), 2), // +0x4f6d70 sub_4f6d70
    Lifted::new(0x70004f6ef0, fn_70004f6ef0 as *const (), 2), // +0x4f6ef0 sub_4f6ef0
    Lifted::new(0x70004f6fe0, fn_70004f6fe0 as *const (), 2), // +0x4f6fe0 sub_4f6fe0
    Lifted::new(0x70004f7160, fn_70004f7160 as *const (), 2), // +0x4f7160 sub_4f7160
    Lifted::new(0x70004f7250, fn_70004f7250 as *const (), 0), // +0x4f7250 sub_4f7250
    Lifted::new(0x70004f7350, fn_70004f7350 as *const (), 2), // +0x4f7350 sub_4f7350
    Lifted::new(0x70004f76a0, fn_70004f76a0 as *const (), 0), // +0x4f76a0 sub_4f76a0
    Lifted::new(0x70004f7840, fn_70004f7840 as *const (), 0), // +0x4f7840 sub_4f7840
    Lifted::new(0x70004f7b00, fn_70004f7b00 as *const (), 2), // +0x4f7b00 sub_4f7b00
    Lifted::new(0x70004f7c40, fn_70004f7c40 as *const (), 2), // +0x4f7c40 sub_4f7c40
    Lifted::new(0x70004f7f90, fn_70004f7f90 as *const (), 0), // +0x4f7f90 sub_4f7f90
    Lifted::new(0x70004f80a0, fn_70004f80a0 as *const (), 2), // +0x4f80a0 sub_4f80a0
    Lifted::new(0x70004f83b0, fn_70004f83b0 as *const (), 0), // +0x4f83b0 sub_4f83b0
    Lifted::new(0x70004f84c0, fn_70004f84c0 as *const (), 2), // +0x4f84c0 sub_4f84c0
    Lifted::new(0x70004f87c0, fn_70004f87c0 as *const (), 0), // +0x4f87c0 sub_4f87c0
    Lifted::new(0x70004f88d0, fn_70004f88d0 as *const (), 0), // +0x4f88d0 sub_4f88d0
    Lifted::new(0x70004f8bd0, fn_70004f8bd0 as *const (), 0), // +0x4f8bd0 sub_4f8bd0
    Lifted::new(0x70004f8ce0, fn_70004f8ce0 as *const (), 0), // +0x4f8ce0 sub_4f8ce0
    Lifted::new(0x70004f8fd0, fn_70004f8fd0 as *const (), 0), // +0x4f8fd0 sub_4f8fd0
    Lifted::new(0x70004f90e0, fn_70004f90e0 as *const (), 0), // +0x4f90e0 sub_4f90e0
    Lifted::new(0x70004f93e0, fn_70004f93e0 as *const (), 0), // +0x4f93e0 sub_4f93e0
    Lifted::new(0x70004f94f0, fn_70004f94f0 as *const (), 0), // +0x4f94f0 sub_4f94f0
    Lifted::new(0x70004f97f0, fn_70004f97f0 as *const (), 0), // +0x4f97f0 sub_4f97f0
    Lifted::new(0x70004f9900, fn_70004f9900 as *const (), 0), // +0x4f9900 sub_4f9900
    Lifted::new(0x70004f9c00, fn_70004f9c00 as *const (), 0), // +0x4f9c00 sub_4f9c00
    Lifted::new(0x70004f9d10, fn_70004f9d10 as *const (), 0), // +0x4f9d10 sub_4f9d10
    Lifted::new(0x70004fa010, fn_70004fa010 as *const (), 0), // +0x4fa010 sub_4fa010
    Lifted::new(0x70004fa110, fn_70004fa110 as *const (), 0), // +0x4fa110 sub_4fa110
    Lifted::new(0x70004fa3d0, fn_70004fa3d0 as *const (), 0), // +0x4fa3d0 sub_4fa3d0
    Lifted::new(0x70004fa510, fn_70004fa510 as *const (), 0), // +0x4fa510 sub_4fa510
    Lifted::new(0x70004fa800, fn_70004fa800 as *const (), 0), // +0x4fa800 sub_4fa800
    Lifted::new(0x70004fa8b0, fn_70004fa8b0 as *const (), 0), // +0x4fa8b0 sub_4fa8b0
    Lifted::new(0x70004fab70, fn_70004fab70 as *const (), 0), // +0x4fab70 sub_4fab70
    Lifted::new(0x70004fac20, fn_70004fac20 as *const (), 0), // +0x4fac20 sub_4fac20
    Lifted::new(0x70004faee0, fn_70004faee0 as *const (), 0), // +0x4faee0 sub_4faee0
    Lifted::new(0x70004fb030, fn_70004fb030 as *const (), 0), // +0x4fb030 sub_4fb030
    Lifted::new(0x70004fb2e0, fn_70004fb2e0 as *const (), 0), // +0x4fb2e0 sub_4fb2e0
    Lifted::new(0x70004fb430, fn_70004fb430 as *const (), 0), // +0x4fb430 sub_4fb430
    Lifted::new(0x70004fb6f0, fn_70004fb6f0 as *const (), 0), // +0x4fb6f0 sub_4fb6f0
    Lifted::new(0x70004fb800, fn_70004fb800 as *const (), 0), // +0x4fb800 sub_4fb800
    Lifted::new(0x70004fbac0, fn_70004fbac0 as *const (), 2), // +0x4fbac0 sub_4fbac0
    Lifted::new(0x70004fbc00, fn_70004fbc00 as *const (), 2), // +0x4fbc00 sub_4fbc00
    Lifted::new(0x70004fbf40, fn_70004fbf40 as *const (), 0), // +0x4fbf40 sub_4fbf40
    Lifted::new(0x70004fbff0, fn_70004fbff0 as *const (), 2), // +0x4fbff0 sub_4fbff0
    Lifted::new(0x70004fc350, fn_70004fc350 as *const (), 0), // +0x4fc350 sub_4fc350
    Lifted::new(0x70004fc4d0, fn_70004fc4d0 as *const (), 0), // +0x4fc4d0 sub_4fc4d0
    Lifted::new(0x70004fc790, fn_70004fc790 as *const (), 0), // +0x4fc790 sub_4fc790
    Lifted::new(0x70004fc8a0, fn_70004fc8a0 as *const (), 0), // +0x4fc8a0 sub_4fc8a0
    Lifted::new(0x70004fcb60, fn_70004fcb60 as *const (), 0), // +0x4fcb60 sub_4fcb60
    Lifted::new(0x70004fcc70, fn_70004fcc70 as *const (), 0), // +0x4fcc70 sub_4fcc70
    Lifted::new(0x70004fcf30, fn_70004fcf30 as *const (), 2), // +0x4fcf30 sub_4fcf30
    Lifted::new(0x70004fd050, fn_70004fd050 as *const (), 0), // +0x4fd050 sub_4fd050
    Lifted::new(0x70004fd360, fn_70004fd360 as *const (), 0), // +0x4fd360 sub_4fd360
    Lifted::new(0x70004fd490, fn_70004fd490 as *const (), 0), // +0x4fd490 sub_4fd490
    Lifted::new(0x70004fd750, fn_70004fd750 as *const (), 0), // +0x4fd750 sub_4fd750
    Lifted::new(0x70004fd8a0, fn_70004fd8a0 as *const (), 0), // +0x4fd8a0 sub_4fd8a0
    Lifted::new(0x70004fdb60, fn_70004fdb60 as *const (), 0), // +0x4fdb60 sub_4fdb60
    Lifted::new(0x70004fdc80, fn_70004fdc80 as *const (), 0), // +0x4fdc80 sub_4fdc80
    Lifted::new(0x70004fdf40, fn_70004fdf40 as *const (), 0), // +0x4fdf40 sub_4fdf40
    Lifted::new(0x70004fe060, fn_70004fe060 as *const (), 0), // +0x4fe060 sub_4fe060
    Lifted::new(0x70004fe320, fn_70004fe320 as *const (), 0), // +0x4fe320 sub_4fe320
    Lifted::new(0x70004fe430, fn_70004fe430 as *const (), 0), // +0x4fe430 sub_4fe430
    Lifted::new(0x70004fe730, fn_70004fe730 as *const (), 0), // +0x4fe730 sub_4fe730
    Lifted::new(0x70004fe840, fn_70004fe840 as *const (), 2), // +0x4fe840 sub_4fe840
    Lifted::new(0x70004feb40, fn_70004feb40 as *const (), 0), // +0x4feb40 sub_4feb40
    Lifted::new(0x70004fec50, fn_70004fec50 as *const (), 0), // +0x4fec50 sub_4fec50
    Lifted::new(0x70004fef50, fn_70004fef50 as *const (), 0), // +0x4fef50 sub_4fef50
    Lifted::new(0x70004ff060, fn_70004ff060 as *const (), 0), // +0x4ff060 sub_4ff060
    Lifted::new(0x70004ff350, fn_70004ff350 as *const (), 2), // +0x4ff350 sub_4ff350
    Lifted::new(0x70004ff510, fn_70004ff510 as *const (), 2), // +0x4ff510 sub_4ff510
    Lifted::new(0x70004ff810, fn_70004ff810 as *const (), 0), // +0x4ff810 sub_4ff810
    Lifted::new(0x70004ff960, fn_70004ff960 as *const (), 2), // +0x4ff960 sub_4ff960
    Lifted::new(0x70004ffc30, fn_70004ffc30 as *const (), 0), // +0x4ffc30 sub_4ffc30
    Lifted::new(0x70004ffd70, fn_70004ffd70 as *const (), 0), // +0x4ffd70 sub_4ffd70
    Lifted::new(0x7000500030, fn_7000500030 as *const (), 0), // +0x500030 sub_500030
    Lifted::new(0x7000500170, fn_7000500170 as *const (), 0), // +0x500170 sub_500170
    Lifted::new(0x7000500430, fn_7000500430 as *const (), 0), // +0x500430 sub_500430
    Lifted::new(0x7000500580, fn_7000500580 as *const (), 0), // +0x500580 sub_500580
    Lifted::new(0x7000500840, fn_7000500840 as *const (), 0), // +0x500840 sub_500840
    Lifted::new(0x7000500960, fn_7000500960 as *const (), 0), // +0x500960 sub_500960
    Lifted::new(0x7000500c50, fn_7000500c50 as *const (), 2), // +0x500c50 sub_500c50
    Lifted::new(0x7000500d70, fn_7000500d70 as *const (), 0), // +0x500d70 sub_500d70
    Lifted::new(0x7000501030, fn_7000501030 as *const (), 2), // +0x501030 sub_501030
    Lifted::new(0x7000501150, fn_7000501150 as *const (), 0), // +0x501150 sub_501150
    Lifted::new(0x7000501410, fn_7000501410 as *const (), 2), // +0x501410 sub_501410
    Lifted::new(0x7000501530, fn_7000501530 as *const (), 0), // +0x501530 sub_501530
    Lifted::new(0x70005017f0, fn_70005017f0 as *const (), 2), // +0x5017f0 sub_5017f0
    Lifted::new(0x7000501910, fn_7000501910 as *const (), 0), // +0x501910 sub_501910
    Lifted::new(0x7000501bd0, fn_7000501bd0 as *const (), 2), // +0x501bd0 sub_501bd0
    Lifted::new(0x7000501cf0, fn_7000501cf0 as *const (), 0), // +0x501cf0 sub_501cf0
    Lifted::new(0x7000501fb0, fn_7000501fb0 as *const (), 2), // +0x501fb0 sub_501fb0
    Lifted::new(0x70005020d0, fn_70005020d0 as *const (), 0), // +0x5020d0 sub_5020d0
    Lifted::new(0x7000502390, fn_7000502390 as *const (), 2), // +0x502390 sub_502390
    Lifted::new(0x70005024b0, fn_70005024b0 as *const (), 0), // +0x5024b0 sub_5024b0
    Lifted::new(0x7000502770, fn_7000502770 as *const (), 2), // +0x502770 sub_502770
    Lifted::new(0x7000502890, fn_7000502890 as *const (), 0), // +0x502890 sub_502890
    Lifted::new(0x7000502b50, fn_7000502b50 as *const (), 2), // +0x502b50 sub_502b50
    Lifted::new(0x7000502c70, fn_7000502c70 as *const (), 0), // +0x502c70 sub_502c70
    Lifted::new(0x7000502f30, fn_7000502f30 as *const (), 2), // +0x502f30 sub_502f30
    Lifted::new(0x7000503050, fn_7000503050 as *const (), 0), // +0x503050 sub_503050
    Lifted::new(0x7000503310, fn_7000503310 as *const (), 2), // +0x503310 sub_503310
    Lifted::new(0x7000503430, fn_7000503430 as *const (), 0), // +0x503430 sub_503430
    Lifted::new(0x70005036f0, fn_70005036f0 as *const (), 2), // +0x5036f0 sub_5036f0
    Lifted::new(0x7000503810, fn_7000503810 as *const (), 0), // +0x503810 sub_503810
    Lifted::new(0x7000503ad0, fn_7000503ad0 as *const (), 0), // +0x503ad0 sub_503ad0
    Lifted::new(0x7000503bf0, fn_7000503bf0 as *const (), 0), // +0x503bf0 sub_503bf0
    Lifted::new(0x7000503ee0, fn_7000503ee0 as *const (), 0), // +0x503ee0 sub_503ee0
    Lifted::new(0x7000504000, fn_7000504000 as *const (), 0), // +0x504000 sub_504000
    Lifted::new(0x70005042f0, fn_70005042f0 as *const (), 0), // +0x5042f0 sub_5042f0
    Lifted::new(0x7000504410, fn_7000504410 as *const (), 0), // +0x504410 sub_504410
    Lifted::new(0x7000504700, fn_7000504700 as *const (), 0), // +0x504700 sub_504700
    Lifted::new(0x7000504800, fn_7000504800 as *const (), 0), // +0x504800 sub_504800
    Lifted::new(0x7000504ad0, fn_7000504ad0 as *const (), 0), // +0x504ad0 sub_504ad0
    Lifted::new(0x7000504ca0, fn_7000504ca0 as *const (), 0), // +0x504ca0 sub_504ca0
    Lifted::new(0x7000505150, fn_7000505150 as *const (), 0), // +0x505150 sub_505150
    Lifted::new(0x7000513000, fn_7000513000 as *const (), 0), // +0x513000 sub_513000
];
