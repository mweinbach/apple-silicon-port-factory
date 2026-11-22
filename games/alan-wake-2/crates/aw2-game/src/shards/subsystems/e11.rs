#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1415bd160, "unknown_1415bd160", tpl_1674::<0x14268a678> as *const ()), // sub_1415bd160
    Replacement::draft(0x1415bd170, "unknown_1415bd170", tpl_22865::<0x14268a678, 0x141834d30> as *const ()), // sub_1415bd170
    Replacement::draft(0x1415bd1a0, "unknown_1415bd1a0", fn_1415bd1a0 as *const ()), // sub_1415bd1a0
    Replacement::draft(0x1415bd1d0, "unknown_1415bd1d0", fn_1415bd1d0 as *const ()), // sub_1415bd1d0
    Replacement::draft(0x1415bd2d0, "unknown_1415bd2d0", fn_1415bd2d0 as *const ()), // sub_1415bd2d0
    Replacement::draft(0x1415bd2f0, "unknown_1415bd2f0", tpl_1674::<0x14268a698> as *const ()), // sub_1415bd2f0
    Replacement::draft(0x1415bd300, "unknown_1415bd300", tpl_27062::<0x14268a698, 0x141834d30> as *const ()), // sub_1415bd300
    Replacement::draft(0x1415bd330, "unknown_1415bd330", fn_1415bd330 as *const ()), // sub_1415bd330
    Replacement::draft(0x1415bd5c0, "unknown_1415bd5c0", fn_1415bd5c0 as *const ()), // sub_1415bd5c0
    Replacement::draft(0x1415bd700, "unknown_1415bd700", fn_1415bd700 as *const ()), // sub_1415bd700
    Replacement::draft(0x1415bd7f0, "unknown_1415bd7f0", fn_1415bd7f0 as *const ()), // sub_1415bd7f0
    Replacement::draft(0x1415bd8a0, "unknown_1415bd8a0", fn_1415bd8a0 as *const ()), // sub_1415bd8a0
    Replacement::draft(0x1415bd980, "unknown_1415bd980", tpl_28527::<0x28, 0x30, 0x8> as *const ()), // sub_1415bd980
    Replacement::draft(0x1415bd9c0, "unknown_1415bd9c0", fn_1415bd9c0 as *const ()), // sub_1415bd9c0
    Replacement::draft(0x1415bda10, "unknown_1415bda10", fn_1415bda10 as *const ()), // sub_1415bda10
    Replacement::draft(0x1415bda50, "unknown_1415bda50", fn_1415bda50 as *const ()), // sub_1415bda50
    Replacement::draft(0x1415bdd20, "unknown_1415bdd20", fn_1415bdd20 as *const ()), // sub_1415bdd20
    Replacement::draft(0x1415bddf0, "unknown_1415bddf0", fn_1415bddf0 as *const ()), // sub_1415bddf0
    Replacement::draft(0x1415be080, "unknown_1415be080", fn_1415be080 as *const ()), // sub_1415be080
    Replacement::draft(0x1415be1c0, "unknown_1415be1c0", fn_1415be1c0 as *const ()), // sub_1415be1c0
    Replacement::draft(0x1415be300, "unknown_1415be300", fn_1415be300 as *const ()), // sub_1415be300
    Replacement::draft(0x1415be410, "unknown_1415be410", fn_1415be410 as *const ()), // sub_1415be410
    Replacement::draft(0x1415be610, "unknown_1415be610", fn_1415be610 as *const ()), // sub_1415be610
    Replacement::draft(0x1415be700, "unknown_1415be700", fn_1415be700 as *const ()), // sub_1415be700
    Replacement::draft(0x1415beac0, "unknown_1415beac0", fn_1415beac0 as *const ()), // sub_1415beac0
    Replacement::draft(0x1415bead0, "unknown_1415bead0", tpl_22426::<0x142689798, 0x1415bd160> as *const ()), // sub_1415bead0
    Replacement::draft(0x1415beae0, "unknown_1415beae0", fn_1415beae0 as *const ()), // sub_1415beae0
    Replacement::draft(0x1415beb50, "unknown_1415beb50", tpl_29629::<0x1415bd2d0, 0x14268a718, 0x10> as *const ()), // sub_1415beb50
    Replacement::draft(0x1415beb80, "unknown_1415beb80", fn_1415beb80 as *const ()), // sub_1415beb80
    Replacement::draft(0x1415bec10, "unknown_1415bec10", fn_1415bec10 as *const ()), // sub_1415bec10
    Replacement::draft(0x1415becc0, "unknown_1415becc0", fn_1415becc0 as *const ()), // sub_1415becc0
    Replacement::draft(0x1415bed30, "unknown_1415bed30", fn_1415bed30 as *const ()), // sub_1415bed30
    Replacement::draft(0x1415bed50, "unknown_1415bed50", fn_1415bed50 as *const ()), // sub_1415bed50
    Replacement::draft(0x1415bed80, "unknown_1415bed80", fn_1415bed80 as *const ()), // sub_1415bed80
    Replacement::draft(0x1415beee0, "unknown_1415beee0", fn_1415beee0 as *const ()), // sub_1415beee0
    Replacement::draft(0x1415bef40, "unknown_1415bef40", fn_1415bef40 as *const ()), // sub_1415bef40
    Replacement::draft(0x1415befc0, "unknown_1415befc0", fn_1415befc0 as *const ()), // sub_1415befc0
    Replacement::draft(0x1415bf020, "unknown_1415bf020", fn_1415bf020 as *const ()), // sub_1415bf020
    Replacement::draft(0x1415bf070, "unknown_1415bf070", fn_1415bf070 as *const ()), // sub_1415bf070
    Replacement::draft(0x1415bf090, "unknown_1415bf090", fn_1415bf090 as *const ()), // sub_1415bf090
    Replacement::draft(0x1415bf0f0, "unknown_1415bf0f0", fn_1415bf0f0 as *const ()), // sub_1415bf0f0
    Replacement::draft(0x1415bf1b0, "unknown_1415bf1b0", fn_1415bf1b0 as *const ()), // sub_1415bf1b0
    Replacement::draft(0x1415bf22c, "unknown_1415bf22c", tpl_28392::<0x1415bf300> as *const ()), // sub_1415bf22c
    Replacement::draft(0x1415bf238, "unknown_1415bf238", tpl_28393::<0x1415bf300> as *const ()), // sub_1415bf238
    Replacement::draft(0x1415bf250, "unknown_1415bf250", fn_1415bf250 as *const ()), // sub_1415bf250
    Replacement::draft(0x1415bf300, "unknown_1415bf300", tpl_29642::<0x1415bf0f0, 0x141834d30> as *const ()), // sub_1415bf300
    Replacement::draft(0x1415bf340, "unknown_1415bf340", fn_1415bf340 as *const ()), // sub_1415bf340
    Replacement::draft(0x1415bf3c0, "unknown_1415bf3c0", fn_1415bf3c0 as *const ()), // sub_1415bf3c0
    Replacement::draft(0x1415bf4b0, "unknown_1415bf4b0", fn_1415bf4b0 as *const ()), // sub_1415bf4b0
    Replacement::draft(0x1415bf570, "unknown_1415bf570", fn_1415bf570 as *const ()), // sub_1415bf570
    Replacement::draft(0x1415bf650, "unknown_1415bf650", fn_1415bf650 as *const ()), // sub_1415bf650
    Replacement::draft(0x1415bf7c0, "unknown_1415bf7c0", fn_1415bf7c0 as *const ()), // sub_1415bf7c0
    Replacement::draft(0x1415bf880, "unknown_1415bf880", fn_1415bf880 as *const ()), // sub_1415bf880
    Replacement::draft(0x1415bfa60, "unknown_1415bfa60", fn_1415bfa60 as *const ()), // sub_1415bfa60
    Replacement::draft(0x1415bfb00, "unknown_1415bfb00", fn_1415bfb00 as *const ()), // sub_1415bfb00
    Replacement::draft(0x1415bfb60, "unknown_1415bfb60", fn_1415bfb60 as *const ()), // sub_1415bfb60
    Replacement::draft(0x1415c0040, "std_1415c0040", fn_1415c0040 as *const ()), // sub_1415c0040
    Replacement::draft(0x1415c0930, "std_1415c0930", fn_1415c0930 as *const ()), // sub_1415c0930
    Replacement::draft(0x1415c0a50, "std_1415c0a50", fn_1415c0a50 as *const ()), // sub_1415c0a50
    Replacement::draft(0x1415c0b00, "unknown_1415c0b00", fn_1415c0b00 as *const ()), // sub_1415c0b00
    Replacement::draft(0x1415c0b50, "unknown_1415c0b50", fn_1415c0b50 as *const ()), // sub_1415c0b50
    Replacement::draft(0x1415c0b70, "unknown_1415c0b70", fn_1415c0b70 as *const ()), // sub_1415c0b70
    Replacement::draft(0x1415c0bf0, "unknown_1415c0bf0", fn_1415c0bf0 as *const ()), // sub_1415c0bf0
    Replacement::draft(0x1415c0c30, "unknown_1415c0c30", fn_1415c0c30 as *const ()), // sub_1415c0c30
    Replacement::draft(0x1415c0d60, "unknown_1415c0d60", fn_1415c0d60 as *const ()), // sub_1415c0d60
    Replacement::draft(0x1415c0fe0, "unknown_1415c0fe0", tpl_29282::<0x143931d40> as *const ()), // sub_1415c0fe0
    Replacement::draft(0x1415c0ff0, "unknown_1415c0ff0", fn_1415c0ff0 as *const ()), // sub_1415c0ff0
    Replacement::draft(0x1415c1150, "unknown_1415c1150", fn_1415c1150 as *const ()), // sub_1415c1150
    Replacement::draft(0x1415c11a0, "unknown_1415c11a0", fn_1415c11a0 as *const ()), // sub_1415c11a0
    Replacement::draft(0x1415c11f0, "unknown_1415c11f0", fn_1415c11f0 as *const ()), // sub_1415c11f0
    Replacement::draft(0x1415c1600, "unknown_1415c1600", fn_1415c1600 as *const ()), // sub_1415c1600
    Replacement::draft(0x1415c1640, "unknown_1415c1640", fn_1415c1640 as *const ()), // sub_1415c1640
    Replacement::draft(0x1415c17d0, "unknown_1415c17d0", fn_1415c17d0 as *const ()), // sub_1415c17d0
    Replacement::draft(0x1415c1900, "unknown_1415c1900", fn_1415c1900 as *const ()), // sub_1415c1900
    Replacement::draft(0x1415c19a0, "unknown_1415c19a0", fn_1415c19a0 as *const ()), // sub_1415c19a0
    Replacement::draft(0x1415c1a50, "unknown_1415c1a50", fn_1415c1a50 as *const ()), // sub_1415c1a50
    Replacement::draft(0x1415c1ac0, "std_1415c1ac0", fn_1415c1ac0 as *const ()), // sub_1415c1ac0
    Replacement::draft(0x1415c1c40, "unknown_1415c1c40", fn_1415c1c40 as *const ()), // sub_1415c1c40
    Replacement::draft(0x1415c1d60, "unknown_1415c1d60", fn_1415c1d60 as *const ()), // sub_1415c1d60
    Replacement::draft(0x1415c1ec0, "unknown_1415c1ec0", fn_1415c1ec0 as *const ()), // sub_1415c1ec0
    Replacement::draft(0x1415c1f30, "unknown_1415c1f30", fn_1415c1f30 as *const ()), // sub_1415c1f30
    Replacement::draft(0x1415c1f80, "unknown_1415c1f80", fn_1415c1f80 as *const ()), // sub_1415c1f80
    Replacement::draft(0x1415c2070, "unknown_1415c2070", fn_1415c2070 as *const ()), // sub_1415c2070
    Replacement::draft(0x1415c20b0, "unknown_1415c20b0", fn_1415c20b0 as *const ()), // sub_1415c20b0
    Replacement::draft(0x1415c2140, "unknown_1415c2140", fn_1415c2140 as *const ()), // sub_1415c2140
    Replacement::draft(0x1415c2230, "unknown_1415c2230", fn_1415c2230 as *const ()), // sub_1415c2230
    Replacement::draft(0x1415c2360, "unknown_1415c2360", fn_1415c2360 as *const ()), // sub_1415c2360
    Replacement::draft(0x1415c2440, "unknown_1415c2440", fn_1415c2440 as *const ()), // sub_1415c2440
    Replacement::draft(0x1415c2480, "unknown_1415c2480", fn_1415c2480 as *const ()), // sub_1415c2480
    Replacement::draft(0x1415c2600, "unknown_1415c2600", fn_1415c2600 as *const ()), // sub_1415c2600
    Replacement::draft(0x1415c2680, "unknown_1415c2680", fn_1415c2680 as *const ()), // sub_1415c2680
    Replacement::draft(0x1415c2730, "unknown_1415c2730", fn_1415c2730 as *const ()), // sub_1415c2730
    Replacement::draft(0x1415c27c0, "unknown_1415c27c0", fn_1415c27c0 as *const ()), // sub_1415c27c0
    Replacement::draft(0x1415c2850, "unknown_1415c2850", fn_1415c2850 as *const ()), // sub_1415c2850
    Replacement::draft(0x1415c2870, "unknown_1415c2870", fn_1415c2870 as *const ()), // sub_1415c2870
    Replacement::draft(0x1415c28a0, "unknown_1415c28a0", fn_1415c28a0 as *const ()), // sub_1415c28a0
    Replacement::draft(0x1415c2900, "unknown_1415c2900", fn_1415c2900 as *const ()), // sub_1415c2900
    Replacement::draft(0x1415c2990, "unknown_1415c2990", fn_1415c2990 as *const ()), // sub_1415c2990
    Replacement::draft(0x1415c29c0, "unknown_1415c29c0", fn_1415c29c0 as *const ()), // sub_1415c29c0
    Replacement::draft(0x1415c2af0, "unknown_1415c2af0", fn_1415c2af0 as *const ()), // sub_1415c2af0
    Replacement::draft(0x1415c2b30, "unknown_1415c2b30", fn_1415c2b30 as *const ()), // sub_1415c2b30
    Replacement::draft(0x1415c2bf0, "unknown_1415c2bf0", fn_1415c2bf0 as *const ()), // sub_1415c2bf0
    Replacement::draft(0x1415c2cc0, "unknown_1415c2cc0", fn_1415c2cc0 as *const ()), // sub_1415c2cc0
    Replacement::draft(0x1415c2db0, "unknown_1415c2db0", fn_1415c2db0 as *const ()), // sub_1415c2db0
    Replacement::draft(0x1415c2e20, "unknown_1415c2e20", fn_1415c2e20 as *const ()), // sub_1415c2e20
    Replacement::draft(0x1415c2f80, "unknown_1415c2f80", fn_1415c2f80 as *const ()), // sub_1415c2f80
    Replacement::draft(0x1415c3010, "unknown_1415c3010", fn_1415c3010 as *const ()), // sub_1415c3010
    Replacement::draft(0x1415c30b0, "unknown_1415c30b0", fn_1415c30b0 as *const ()), // sub_1415c30b0
    Replacement::draft(0x1415c31e0, "unknown_1415c31e0", fn_1415c31e0 as *const ()), // sub_1415c31e0
    Replacement::draft(0x1415c3250, "unknown_1415c3250", fn_1415c3250 as *const ()), // sub_1415c3250
    Replacement::draft(0x1415c32f0, "unknown_1415c32f0", fn_1415c32f0 as *const ()), // sub_1415c32f0
    Replacement::draft(0x1415c3390, "unknown_1415c3390", fn_1415c3390 as *const ()), // sub_1415c3390
    Replacement::draft(0x1415c33f0, "unknown_1415c33f0", fn_1415c33f0 as *const ()), // sub_1415c33f0
    Replacement::draft(0x1415c3470, "unknown_1415c3470", fn_1415c3470 as *const ()), // sub_1415c3470
    Replacement::draft(0x1415c3550, "unknown_1415c3550", fn_1415c3550 as *const ()), // sub_1415c3550
    Replacement::draft(0x1415c3780, "unknown_1415c3780", fn_1415c3780 as *const ()), // sub_1415c3780
    Replacement::draft(0x1415c3840, "unknown_1415c3840", fn_1415c3840 as *const ()), // sub_1415c3840
    Replacement::draft(0x1415c39c0, "unknown_1415c39c0", fn_1415c39c0 as *const ()), // sub_1415c39c0
    Replacement::draft(0x1415c3a80, "unknown_1415c3a80", fn_1415c3a80 as *const ()), // sub_1415c3a80
    Replacement::draft(0x1415c3c70, "unknown_1415c3c70", fn_1415c3c70 as *const ()), // sub_1415c3c70
    Replacement::draft(0x1415c3e60, "unknown_1415c3e60", fn_1415c3e60 as *const ()), // sub_1415c3e60
    Replacement::draft(0x1415c3f50, "unknown_1415c3f50", fn_1415c3f50 as *const ()), // sub_1415c3f50
    Replacement::draft(0x1415c3fb0, "unknown_1415c3fb0", fn_1415c3fb0 as *const ()), // sub_1415c3fb0
    Replacement::draft(0x1415c4160, "unknown_1415c4160", fn_1415c4160 as *const ()), // sub_1415c4160
    Replacement::draft(0x1415c4320, "unknown_1415c4320", fn_1415c4320 as *const ()), // sub_1415c4320
    Replacement::draft(0x1415c43f0, "unknown_1415c43f0", fn_1415c43f0 as *const ()), // sub_1415c43f0
    Replacement::draft(0x1415c4450, "unknown_1415c4450", fn_1415c4450 as *const ()), // sub_1415c4450
    Replacement::draft(0x1415c4520, "unknown_1415c4520", fn_1415c4520 as *const ()), // sub_1415c4520
    Replacement::draft(0x1415c45f0, "unknown_1415c45f0", fn_1415c45f0 as *const ()), // sub_1415c45f0
    Replacement::draft(0x1415c4670, "unknown_1415c4670", fn_1415c4670 as *const ()), // sub_1415c4670
    Replacement::draft(0x1415c4750, "unknown_1415c4750", fn_1415c4750 as *const ()), // sub_1415c4750
    Replacement::draft(0x1415c4950, "unknown_1415c4950", fn_1415c4950 as *const ()), // sub_1415c4950
    Replacement::draft(0x1415c4a10, "unknown_1415c4a10", tpl_88388::<0x143930f88, 0x1426452b0, 0x1426452a8, 0x142645560, 0x141616110> as *const ()), // sub_1415c4a10
    Replacement::draft(0x1415c4b70, "unknown_1415c4b70", fn_1415c4b70 as *const ()), // sub_1415c4b70
    Replacement::draft(0x1415c4bf0, "unknown_1415c4bf0", fn_1415c4bf0 as *const ()), // sub_1415c4bf0
    Replacement::draft(0x1415c4d70, "unknown_1415c4d70", fn_1415c4d70 as *const ()), // sub_1415c4d70
    Replacement::draft(0x1415c4e80, "unknown_1415c4e80", fn_1415c4e80 as *const ()), // sub_1415c4e80
    Replacement::draft(0x1415c4f10, "unknown_1415c4f10", fn_1415c4f10 as *const ()), // sub_1415c4f10
    Replacement::draft(0x1415c4f90, "unknown_1415c4f90", fn_1415c4f90 as *const ()), // sub_1415c4f90
    Replacement::draft(0x1415c5080, "ak_1415c5080", fn_1415c5080 as *const ()), // sub_1415c5080
    Replacement::draft(0x1415c5200, "unknown_1415c5200", fn_1415c5200 as *const ()), // sub_1415c5200
    Replacement::draft(0x1415c53e0, "unknown_1415c53e0", fn_1415c53e0 as *const ()), // sub_1415c53e0
    Replacement::draft(0x1415c5470, "unknown_1415c5470", tpl_88388::<0x143930f88, 0x1426452b0, 0x1426452a8, 0x142645560, 0x141618d30> as *const ()), // sub_1415c5470
    Replacement::draft(0x1415c55d0, "unknown_1415c55d0", fn_1415c55d0 as *const ()), // sub_1415c55d0
    Replacement::draft(0x1415c5690, "unknown_1415c5690", fn_1415c5690 as *const ()), // sub_1415c5690
    Replacement::draft(0x1415c5720, "unknown_1415c5720", fn_1415c5720 as *const ()), // sub_1415c5720
    Replacement::draft(0x1415c5750, "unknown_1415c5750", fn_1415c5750 as *const ()), // sub_1415c5750
    Replacement::draft(0x1415c57b0, "unknown_1415c57b0", fn_1415c57b0 as *const ()), // sub_1415c57b0
    Replacement::draft(0x1415c5820, "unknown_1415c5820", fn_1415c5820 as *const ()), // sub_1415c5820
    Replacement::draft(0x1415c5840, "unknown_1415c5840", fn_1415c5840 as *const ()), // sub_1415c5840
    Replacement::draft(0x1415c5860, "unknown_1415c5860", fn_1415c5860 as *const ()), // sub_1415c5860
    Replacement::draft(0x1415c58f0, "unknown_1415c58f0", fn_1415c58f0 as *const ()), // sub_1415c58f0
    Replacement::draft(0x1415c5930, "unknown_1415c5930", fn_1415c5930 as *const ()), // sub_1415c5930
    Replacement::draft(0x1415c5a40, "unknown_1415c5a40", fn_1415c5a40 as *const ()), // sub_1415c5a40
    Replacement::draft(0x1415c5b60, "unknown_1415c5b60", fn_1415c5b60 as *const ()), // sub_1415c5b60
    Replacement::draft(0x1415c5d20, "unknown_1415c5d20", fn_1415c5d20 as *const ()), // sub_1415c5d20
    Replacement::draft(0x1415c5db0, "unknown_1415c5db0", fn_1415c5db0 as *const ()), // sub_1415c5db0
    Replacement::draft(0x1415c5f60, "unknown_1415c5f60", fn_1415c5f60 as *const ()), // sub_1415c5f60
    Replacement::draft(0x1415c60e0, "unknown_1415c60e0", fn_1415c60e0 as *const ()), // sub_1415c60e0
    Replacement::draft(0x1415c6110, "unknown_1415c6110", fn_1415c6110 as *const ()), // sub_1415c6110
    Replacement::draft(0x1415c6160, "unknown_1415c6160", fn_1415c6160 as *const ()), // sub_1415c6160
    Replacement::draft(0x1415c61c0, "unknown_1415c61c0", fn_1415c61c0 as *const ()), // sub_1415c61c0
    Replacement::draft(0x1415c61f0, "unknown_1415c61f0", fn_1415c61f0 as *const ()), // sub_1415c61f0
    Replacement::draft(0x1415c6220, "unknown_1415c6220", fn_1415c6220 as *const ()), // sub_1415c6220
    Replacement::draft(0x1415c6280, "unknown_1415c6280", fn_1415c6280 as *const ()), // sub_1415c6280
    Replacement::draft(0x1415c63b0, "unknown_1415c63b0", fn_1415c63b0 as *const ()), // sub_1415c63b0
    Replacement::draft(0x1415c6420, "unknown_1415c6420", fn_1415c6420 as *const ()), // sub_1415c6420
    Replacement::draft(0x1415c6480, "unknown_1415c6480", fn_1415c6480 as *const ()), // sub_1415c6480
    Replacement::draft(0x1415c64c0, "unknown_1415c64c0", fn_1415c64c0 as *const ()), // sub_1415c64c0
    Replacement::draft(0x1415c64e0, "unknown_1415c64e0", fn_1415c64e0 as *const ()), // sub_1415c64e0
    Replacement::draft(0x1415c6640, "unknown_1415c6640", fn_1415c6640 as *const ()), // sub_1415c6640
    Replacement::draft(0x1415c664c, "unknown_1415c664c", fn_1415c664c as *const ()), // sub_1415c664c
    Replacement::draft(0x1415c6660, "unknown_1415c6660", fn_1415c6660 as *const ()), // sub_1415c6660
    Replacement::draft(0x1415c6800, "unknown_1415c6800", fn_1415c6800 as *const ()), // sub_1415c6800
    Replacement::draft(0x1415c6840, "unknown_1415c6840", fn_1415c6840 as *const ()), // sub_1415c6840
    Replacement::draft(0x1415c6850, "unknown_1415c6850", fn_1415c6850 as *const ()), // sub_1415c6850
    Replacement::draft(0x1415c69d0, "unknown_1415c69d0", tpl_913::<0xd8> as *const ()), // sub_1415c69d0
    Replacement::draft(0x1415c69e0, "unknown_1415c69e0", fn_1415c69e0 as *const ()), // sub_1415c69e0
    Replacement::draft(0x1415c69f0, "unknown_1415c69f0", tpl_29742::<0xe0, 0x20> as *const ()), // sub_1415c69f0
    Replacement::draft(0x1415c6a10, "unknown_1415c6a10", fn_1415c6a10 as *const ()), // sub_1415c6a10
    Replacement::draft(0x1415c6d30, "unknown_1415c6d30", fn_1415c6d30 as *const ()), // sub_1415c6d30
    Replacement::draft(0x1415c6dc0, "unknown_1415c6dc0", tpl_29744::<0xf8> as *const ()), // sub_1415c6dc0
    Replacement::draft(0x1415c6dd0, "unknown_1415c6dd0", fn_1415c6dd0 as *const ()), // sub_1415c6dd0
    Replacement::draft(0x1415c7000, "unknown_1415c7000", fn_1415c7000 as *const ()), // sub_1415c7000
    Replacement::draft(0x1415c70b0, "unknown_1415c70b0", fn_1415c70b0 as *const ()), // sub_1415c70b0
    Replacement::draft(0x1415c70f0, "unknown_1415c70f0", fn_1415c70f0 as *const ()), // sub_1415c70f0
    Replacement::draft(0x1415c7130, "unknown_1415c7130", fn_1415c7130 as *const ()), // sub_1415c7130
    Replacement::draft(0x1415c71e0, "unknown_1415c71e0", fn_1415c71e0 as *const ()), // sub_1415c71e0
    Replacement::draft(0x1415c7460, "unknown_1415c7460", tpl_29748::<0x141612300> as *const ()), // sub_1415c7460
    Replacement::draft(0x1415c7470, "unknown_1415c7470", tpl_29748::<0x141612370> as *const ()), // sub_1415c7470
    Replacement::draft(0x1415c7480, "unknown_1415c7480", fn_1415c7480 as *const ()), // sub_1415c7480
    Replacement::draft(0x1415c7600, "unknown_1415c7600", fn_1415c7600 as *const ()), // sub_1415c7600
    Replacement::draft(0x1415c7720, "unknown_1415c7720", tpl_28480::<0x14268ac10, 0xd8, 0x142677588, 0x8, 0x14268ae70, 0x10, 0x14268ae98, 0x28, 0x1415242e0, 0x14, 0x14159c0b0> as *const ()), // sub_1415c7720
    Replacement::draft(0x1415c77a0, "unknown_1415c77a0", fn_1415c77a0 as *const ()), // sub_1415c77a0
    Replacement::draft(0x1415c7aac, "unknown_1415c7aac", tpl_28392::<0x1415c7b00> as *const ()), // sub_1415c7aac
    Replacement::draft(0x1415c7ab8, "unknown_1415c7ab8", tpl_28393::<0x1415c7b00> as *const ()), // sub_1415c7ab8
    Replacement::draft(0x1415c7ac4, "unknown_1415c7ac4", tpl_28392::<0x1415c7b60> as *const ()), // sub_1415c7ac4
    Replacement::draft(0x1415c7ad0, "unknown_1415c7ad0", tpl_28393::<0x1415c7b60> as *const ()), // sub_1415c7ad0
    Replacement::draft(0x1415c7adc, "unknown_1415c7adc", tpl_28392::<0x1415c7ba0> as *const ()), // sub_1415c7adc
    Replacement::draft(0x1415c7ae8, "unknown_1415c7ae8", tpl_28393::<0x1415c7ba0> as *const ()), // sub_1415c7ae8
    Replacement::draft(0x1415c7b00, "concurrency_1415c7b00", tpl_29466::<0x14268af18, 0x142677588, 0x8, 0x14268b178, 0x10, 0x14268b1a0, 0x28, 0x1415c7720, 0x141834d30> as *const ()), // sub_1415c7b00
    Replacement::draft(0x1415c7b60, "unknown_1415c7b60", tpl_29642::<0x1415c7720, 0x141834d30> as *const ()), // sub_1415c7b60
    Replacement::draft(0x1415c7ba0, "unknown_1415c7ba0", fn_1415c7ba0 as *const ()), // sub_1415c7ba0
    Replacement::draft(0x1415c7be0, "unknown_1415c7be0", tpl_28395::<0x258, 0x20, 0xd8, 0x1415353d0, 0x1, 0x24> as *const ()), // sub_1415c7be0
    Replacement::draft(0x1415c7c60, "unknown_1415c7c60", fn_1415c7c60 as *const ()), // sub_1415c7c60
    Replacement::draft(0x1415c7d20, "unknown_1415c7d20", fn_1415c7d20 as *const ()), // sub_1415c7d20
    Replacement::draft(0x1415c7f60, "unknown_1415c7f60", fn_1415c7f60 as *const ()), // sub_1415c7f60
    Replacement::draft(0x1415c8020, "unknown_1415c8020", fn_1415c8020 as *const ()), // sub_1415c8020
    Replacement::draft(0x1415c80a0, "unknown_1415c80a0", fn_1415c80a0 as *const ()), // sub_1415c80a0
    Replacement::draft(0x1415c80d0, "unknown_1415c80d0", fn_1415c80d0 as *const ()), // sub_1415c80d0
    Replacement::draft(0x1415c8160, "unknown_1415c8160", fn_1415c8160 as *const ()), // sub_1415c8160
    Replacement::draft(0x1415c8200, "unknown_1415c8200", fn_1415c8200 as *const ()), // sub_1415c8200
    Replacement::draft(0x1415c8290, "unknown_1415c8290", tpl_26849::<0xe8> as *const ()), // sub_1415c8290
    Replacement::draft(0x1415c82a0, "unknown_1415c82a0", fn_1415c82a0 as *const ()), // sub_1415c82a0
    Replacement::draft(0x1415c82b0, "unknown_1415c82b0", fn_1415c82b0 as *const ()), // sub_1415c82b0
    Replacement::draft(0x1415c83a0, "unknown_1415c83a0", fn_1415c83a0 as *const ()), // sub_1415c83a0
    Replacement::draft(0x1415c8470, "unknown_1415c8470", fn_1415c8470 as *const ()), // sub_1415c8470
    Replacement::draft(0x1415c86a0, "unknown_1415c86a0", fn_1415c86a0 as *const ()), // sub_1415c86a0
    Replacement::draft(0x1415c8820, "unknown_1415c8820", fn_1415c8820 as *const ()), // sub_1415c8820
    Replacement::draft(0x1415c8860, "unknown_1415c8860", fn_1415c8860 as *const ()), // sub_1415c8860
    Replacement::draft(0x1415c8a10, "unknown_1415c8a10", fn_1415c8a10 as *const ()), // sub_1415c8a10
    Replacement::draft(0x1415c8c30, "unknown_1415c8c30", fn_1415c8c30 as *const ()), // sub_1415c8c30
    Replacement::draft(0x1415c8c50, "unknown_1415c8c50", fn_1415c8c50 as *const ()), // sub_1415c8c50
    Replacement::draft(0x1415c8cf0, "unknown_1415c8cf0", fn_1415c8cf0 as *const ()), // sub_1415c8cf0
    Replacement::draft(0x1415c8dc0, "unknown_1415c8dc0", fn_1415c8dc0 as *const ()), // sub_1415c8dc0
    Replacement::draft(0x1415c8e90, "unknown_1415c8e90", fn_1415c8e90 as *const ()), // sub_1415c8e90
    Replacement::draft(0x1415c8fa0, "unknown_1415c8fa0", tpl_5167::<0x90> as *const ()), // sub_1415c8fa0
    Replacement::draft(0x1415c8fb0, "unknown_1415c8fb0", fn_1415c8fb0 as *const ()), // sub_1415c8fb0
    Replacement::draft(0x1415c90d0, "unknown_1415c90d0", fn_1415c90d0 as *const ()), // sub_1415c90d0
    Replacement::draft(0x1415c91d0, "unknown_1415c91d0", tpl_86720::<0xe0, 0xe8, 0x68> as *const ()), // sub_1415c91d0
    Replacement::draft(0x1415c92a0, "unknown_1415c92a0", fn_1415c92a0 as *const ()), // sub_1415c92a0
    Replacement::draft(0x1415c9530, "unknown_1415c9530", fn_1415c9530 as *const ()), // sub_1415c9530
    Replacement::draft(0x1415c9610, "unknown_1415c9610", fn_1415c9610 as *const ()), // sub_1415c9610
    Replacement::draft(0x1415c9a50, "unknown_1415c9a50", fn_1415c9a50 as *const ()), // sub_1415c9a50
    Replacement::draft(0x1415c9aa0, "unknown_1415c9aa0", fn_1415c9aa0 as *const ()), // sub_1415c9aa0
    Replacement::draft(0x1415c9ba0, "unknown_1415c9ba0", fn_1415c9ba0 as *const ()), // sub_1415c9ba0
    Replacement::draft(0x1415c9c20, "unknown_1415c9c20", fn_1415c9c20 as *const ()), // sub_1415c9c20
    Replacement::draft(0x1415c9c80, "unknown_1415c9c80", fn_1415c9c80 as *const ()), // sub_1415c9c80
    Replacement::draft(0x1415c9f00, "unknown_1415c9f00", fn_1415c9f00 as *const ()), // sub_1415c9f00
    Replacement::draft(0x1415ca170, "unknown_1415ca170", fn_1415ca170 as *const ()), // sub_1415ca170
    Replacement::draft(0x1415ca1a0, "unknown_1415ca1a0", fn_1415ca1a0 as *const ()), // sub_1415ca1a0
    Replacement::draft(0x1415ca360, "unknown_1415ca360", fn_1415ca360 as *const ()), // sub_1415ca360
    Replacement::draft(0x1415ca3f0, "unknown_1415ca3f0", fn_1415ca3f0 as *const ()), // sub_1415ca3f0
    Replacement::draft(0x1415ca460, "unknown_1415ca460", fn_1415ca460 as *const ()), // sub_1415ca460
    Replacement::draft(0x1415ca4a0, "unknown_1415ca4a0", fn_1415ca4a0 as *const ()), // sub_1415ca4a0
    Replacement::draft(0x1415ca590, "ak_1415ca590", tpl_28185::<0x1439318d8> as *const ()), // sub_1415ca590
    Replacement::draft(0x1415ca5a0, "ak_1415ca5a0", fn_1415ca5a0 as *const ()), // sub_1415ca5a0
    Replacement::draft(0x1415ca6a0, "ak_1415ca6a0", fn_1415ca6a0 as *const ()), // sub_1415ca6a0
    Replacement::draft(0x1415ca6c0, "ak_1415ca6c0", tpl_29778::<0x80> as *const ()), // sub_1415ca6c0
    Replacement::draft(0x1415ca6d0, "ak_1415ca6d0", fn_1415ca6d0 as *const ()), // sub_1415ca6d0
    Replacement::draft(0x1415ca720, "ak_1415ca720", fn_1415ca720 as *const ()), // sub_1415ca720
    Replacement::draft(0x1415ca7e0, "ak_1415ca7e0", fn_1415ca7e0 as *const ()), // sub_1415ca7e0
    Replacement::draft(0x1415ca8e0, "ak_1415ca8e0", tpl_29781::<0x20, 0x1415ee770> as *const ()), // sub_1415ca8e0
    Replacement::draft(0x1415ca8f0, "ak_1415ca8f0", tpl_29781::<0x20, 0x1415f1230> as *const ()), // sub_1415ca8f0
    Replacement::draft(0x1415ca900, "ak_1415ca900", fn_1415ca900 as *const ()), // sub_1415ca900
    Replacement::draft(0x1415ca930, "ak_1415ca930", fn_1415ca930 as *const ()), // sub_1415ca930
    Replacement::draft(0x1415ca970, "ak_1415ca970", fn_1415ca970 as *const ()), // sub_1415ca970
    Replacement::draft(0x1415cabc0, "ak_1415cabc0", tpl_28412::<0xe0, 0xe8, 0xa0> as *const ()), // sub_1415cabc0
    Replacement::draft(0x1415cac50, "ak_1415cac50", fn_1415cac50 as *const ()), // sub_1415cac50
    Replacement::draft(0x1415cad10, "ak_1415cad10", tpl_5167::<0xc8> as *const ()), // sub_1415cad10
    Replacement::draft(0x1415cad20, "ak_1415cad20", tpl_86840::<0xe0, 0xe8, 0x128, 0x130> as *const ()), // sub_1415cad20
    Replacement::draft(0x1415cade0, "ak_1415cade0", fn_1415cade0 as *const ()), // sub_1415cade0
    Replacement::draft(0x1415cae00, "ak_1415cae00", fn_1415cae00 as *const ()), // sub_1415cae00
    Replacement::draft(0x1415cae30, "ak_1415cae30", fn_1415cae30 as *const ()), // sub_1415cae30
    Replacement::draft(0x1415caf30, "ak_1415caf30", fn_1415caf30 as *const ()), // sub_1415caf30
    Replacement::draft(0x1415cafa0, "ak_1415cafa0", tpl_28416::<0x78, 0x20, 0xd8, 0x1415355f0, 0xe8, 0xe0, 0x8, 0x14183acc8, 0x10, 0x14, 0x18> as *const ()), // sub_1415cafa0
    Replacement::draft(0x1415cb030, "ak_1415cb030", fn_1415cb030 as *const ()), // sub_1415cb030
    Replacement::draft(0x1415cb070, "ak_1415cb070", fn_1415cb070 as *const ()), // sub_1415cb070
    Replacement::draft(0x1415cb150, "ak_1415cb150", fn_1415cb150 as *const ()), // sub_1415cb150
    Replacement::draft(0x1415cb1a0, "ak_1415cb1a0", tpl_29791::<0x1415bd1d0> as *const ()), // sub_1415cb1a0
    Replacement::draft(0x1415cb1b0, "ak_1415cb1b0", fn_1415cb1b0 as *const ()), // sub_1415cb1b0
    Replacement::draft(0x1415cb210, "ak_1415cb210", fn_1415cb210 as *const ()), // sub_1415cb210
    Replacement::draft(0x1415cb290, "ak_1415cb290", fn_1415cb290 as *const ()), // sub_1415cb290
    Replacement::draft(0x1415cb390, "ak_1415cb390", fn_1415cb390 as *const ()), // sub_1415cb390
    Replacement::draft(0x1415cb5a0, "ak_1415cb5a0", fn_1415cb5a0 as *const ()), // sub_1415cb5a0
    Replacement::draft(0x1415cb610, "ak_1415cb610", fn_1415cb610 as *const ()), // sub_1415cb610
    Replacement::draft(0x1415cb690, "std_1415cb690", fn_1415cb690 as *const ()), // sub_1415cb690
    Replacement::draft(0x1415cb700, "ak_1415cb700", fn_1415cb700 as *const ()), // sub_1415cb700
    Replacement::draft(0x1415cb7e0, "ak_1415cb7e0", fn_1415cb7e0 as *const ()), // sub_1415cb7e0
    Replacement::draft(0x1415cbae0, "ak_1415cbae0", fn_1415cbae0 as *const ()), // sub_1415cbae0
    Replacement::draft(0x1415cbf20, "ak_1415cbf20", fn_1415cbf20 as *const ()), // sub_1415cbf20
    Replacement::draft(0x1415cc020, "ak_1415cc020", fn_1415cc020 as *const ()), // sub_1415cc020
    Replacement::draft(0x1415cc0a0, "ak_1415cc0a0", fn_1415cc0a0 as *const ()), // sub_1415cc0a0
    Replacement::draft(0x1415cc1e0, "ak_1415cc1e0", fn_1415cc1e0 as *const ()), // sub_1415cc1e0
    Replacement::draft(0x1415cc2d0, "ak_1415cc2d0", fn_1415cc2d0 as *const ()), // sub_1415cc2d0
    Replacement::draft(0x1415cc380, "ak_1415cc380", tpl_28501::<0xe0, 0xe8, 0x130> as *const ()), // sub_1415cc380
    Replacement::draft(0x1415cc3e0, "ak_1415cc3e0", tpl_365::<0x1415a37f0> as *const ()), // sub_1415cc3e0
    Replacement::draft(0x1415cc3f0, "ak_1415cc3f0", fn_1415cc3f0 as *const ()), // sub_1415cc3f0
    Replacement::draft(0x1415cc410, "ak_1415cc410", fn_1415cc410 as *const ()), // sub_1415cc410
    Replacement::draft(0x1415cc530, "ak_1415cc530", fn_1415cc530 as *const ()), // sub_1415cc530
    Replacement::draft(0x1415cc860, "ak_1415cc860", tpl_29781::<0x20, 0x1415f2930> as *const ()), // sub_1415cc860
    Replacement::draft(0x1415cc870, "ak_1415cc870", fn_1415cc870 as *const ()), // sub_1415cc870
    Replacement::draft(0x1415cc8f0, "ak_1415cc8f0", fn_1415cc8f0 as *const ()), // sub_1415cc8f0
    Replacement::draft(0x1415cc980, "ak_1415cc980", fn_1415cc980 as *const ()), // sub_1415cc980
    Replacement::draft(0x1415cca50, "ak_1415cca50", fn_1415cca50 as *const ()), // sub_1415cca50
    Replacement::draft(0x1415ccba0, "ak_1415ccba0", fn_1415ccba0 as *const ()), // sub_1415ccba0
    Replacement::draft(0x1415ccc10, "ak_1415ccc10", fn_1415ccc10 as *const ()), // sub_1415ccc10
    Replacement::draft(0x1415cccc0, "ak_1415cccc0", fn_1415cccc0 as *const ()), // sub_1415cccc0
    Replacement::draft(0x1415ccd20, "ak_1415ccd20", fn_1415ccd20 as *const ()), // sub_1415ccd20
    Replacement::draft(0x1415ccd60, "ak_1415ccd60", fn_1415ccd60 as *const ()), // sub_1415ccd60
    Replacement::draft(0x1415ccd80, "ak_1415ccd80", tpl_893::<0x14268b538, 0x141834d30> as *const ()), // sub_1415ccd80
    Replacement::draft(0x1415ccdb0, "ak_1415ccdb0", fn_1415ccdb0 as *const ()), // sub_1415ccdb0
    Replacement::draft(0x1415cce30, "ak_1415cce30", tpl_27062::<0x14268b538, 0x141834d30> as *const ()), // sub_1415cce30
    Replacement::draft(0x1415cce60, "ak_1415cce60", fn_1415cce60 as *const ()), // sub_1415cce60
    Replacement::draft(0x1415cd010, "ak_1415cd010", fn_1415cd010 as *const ()), // sub_1415cd010
    Replacement::draft(0x1415cd060, "ak_1415cd060", fn_1415cd060 as *const ()), // sub_1415cd060
    Replacement::draft(0x1415cd260, "ak_1415cd260", fn_1415cd260 as *const ()), // sub_1415cd260
    Replacement::draft(0x1415cd2c0, "ak_1415cd2c0", fn_1415cd2c0 as *const ()), // sub_1415cd2c0
    Replacement::draft(0x1415cd2f0, "ak_1415cd2f0", tpl_29814::<0x30> as *const ()), // sub_1415cd2f0
    Replacement::draft(0x1415cd310, "ak_1415cd310", fn_1415cd310 as *const ()), // sub_1415cd310
    Replacement::draft(0x1415cd330, "ak_1415cd330", tpl_29814::<0x28> as *const ()), // sub_1415cd330
    Replacement::draft(0x1415cd350, "ak_1415cd350", fn_1415cd350 as *const ()), // sub_1415cd350
    Replacement::draft(0x1415cd380, "ak_1415cd380", fn_1415cd380 as *const ()), // sub_1415cd380
    Replacement::draft(0x1415cd430, "ak_1415cd430", fn_1415cd430 as *const ()), // sub_1415cd430
    Replacement::draft(0x1415cd4e0, "ak_1415cd4e0", tpl_29819::<0x30> as *const ()), // sub_1415cd4e0
    Replacement::draft(0x1415cd500, "ak_1415cd500", tpl_29819::<0x28> as *const ()), // sub_1415cd500
    Replacement::draft(0x1415cd520, "ak_1415cd520", fn_1415cd520 as *const ()), // sub_1415cd520
    Replacement::draft(0x1415cd560, "ak_1415cd560", fn_1415cd560 as *const ()), // sub_1415cd560
    Replacement::draft(0x1415cd6c0, "ak_1415cd6c0", tpl_29822::<0x14268b5c8, 0x8, 0x10, 0x18> as *const ()), // AK::ReadBytesMem::AK::ReadBytesMem::ReadBytesMem
    Replacement::draft(0x1415cd6e0, "ak_1415cd6e0", tpl_29823::<0x14268b5c8, 0x8, 0x10, 0x18> as *const ()), // AK::ReadBytesMem::AK::ReadBytesMem::ReadBytesMem
    Replacement::draft(0x1415cd700, "ak_1415cd700", tpl_28296::<0x14268b618, 0x8, 0x10> as *const ()), // AK::WriteBytesBuffer::AK::WriteBytesBuffer::WriteBytesBuffer
    Replacement::draft(0x1415cd720, "ak_1415cd720", fn_1415cd720 as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::WriteBytesMem
    Replacement::draft(0x1415cd750, "ak_1415cd750", tpl_1674::<0x14268b5c8> as *const ()), // AK::ReadBytesMem::AK::ReadBytesMem::~ReadBytesMem
    Replacement::draft(0x1415cd760, "ak_1415cd760", tpl_1674::<0x14268b618> as *const ()), // AK::WriteBytesBuffer::AK::WriteBytesBuffer::~WriteBytesBuffer
    Replacement::draft(0x1415cd770, "ak_1415cd770", fn_1415cd770 as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::~WriteBytesMem
    Replacement::draft(0x1415cd790, "ak_1415cd790", tpl_88537::<0xfffffffffffffff8, 0x14268b5c8, 0xffffffffffffffe0, 0x141834ee0, 0x141834d30> as *const ()), // sub_1415cd790
    Replacement::draft(0x1415cd820, "ak_1415cd820", fn_1415cd820 as *const ()), // sub_1415cd820
    Replacement::draft(0x1415cd8b0, "ak_1415cd8b0", fn_1415cd8b0 as *const ()), // sub_1415cd8b0
    Replacement::draft(0x1415cd990, "ak_1415cd990", fn_1415cd990 as *const ()), // AK::ReadBytesMem::AK::ReadBytesMem::Attach
    Replacement::draft(0x1415cd9a0, "ak_1415cd9a0", tpl_3047::<0x14> as *const ()), // AK::WriteBytesBuffer::AK::WriteBytesBuffer::Clear
    Replacement::draft(0x1415cd9b0, "ak_1415cd9b0", tpl_3047::<0x18> as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::Clear
    Replacement::draft(0x1415cd9c0, "ak_1415cd9c0", fn_1415cd9c0 as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::Detach
    Replacement::draft(0x1415cd9e0, "ak_1415cd9e0", fn_1415cd9e0 as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::HasValidMemPool
    Replacement::draft(0x1415cd9f0, "ak_1415cd9f0", fn_1415cd9f0 as *const ()), // AK::ReadBytesMem::AK::ReadBytesMem::ReadBytes
    Replacement::draft(0x1415cda70, "ak_1415cda70", fn_1415cda70 as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::Reserve
    Replacement::draft(0x1415cdad0, "ak_1415cdad0", tpl_29831::<0x8, 0x10> as *const ()), // AK::WriteBytesBuffer::AK::WriteBytesBuffer::SetBuffer
    Replacement::draft(0x1415cdae0, "ak_1415cdae0", tpl_28365::<0x18> as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::SetCount
    Replacement::draft(0x1415cdaf0, "ak_1415cdaf0", fn_1415cdaf0 as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::SetMemPool
    Replacement::draft(0x1415cdb30, "ak_1415cdb30", fn_1415cdb30 as *const ()), // AK::WriteBytesBuffer::AK::WriteBytesBuffer::WriteBytes
    Replacement::draft(0x1415cdb90, "ak_1415cdb90", fn_1415cdb90 as *const ()), // AK::WriteBytesMem::AK::WriteBytesMem::WriteBytes
    Replacement::draft(0x1415cdc40, "ak_1415cdc40", tpl_29822::<0x14268b628, 0x8, 0x10, 0x18> as *const ()), // AK::ReadBytesSkip::AK::ReadBytesSkip::ReadBytesSkip
    Replacement::draft(0x1415cdc60, "ak_1415cdc60", tpl_29823::<0x14268b628, 0x8, 0x10, 0x18> as *const ()), // AK::ReadBytesSkip::AK::ReadBytesSkip::ReadBytesSkip
    Replacement::draft(0x1415cdc80, "ak_1415cdc80", tpl_29835::<0x14268b638, 0x8> as *const ()), // AK::WriteBytesCount::AK::WriteBytesCount::WriteBytesCount
    Replacement::draft(0x1415cdca0, "ak_1415cdca0", tpl_1674::<0x14268b628> as *const ()), // AK::ReadBytesSkip::AK::ReadBytesSkip::~ReadBytesSkip
    Replacement::draft(0x1415cdcb0, "ak_1415cdcb0", tpl_1674::<0x14268b638> as *const ()), // AK::WriteBytesCount::AK::WriteBytesCount::~WriteBytesCount
    Replacement::draft(0x1415cdcc0, "ak_1415cdcc0", tpl_88537::<0xfffffffffffffff8, 0x14268b628, 0xffffffffffffffe0, 0x141834ee0, 0x141834d30> as *const ()), // sub_1415cdcc0
    Replacement::draft(0x1415cdd50, "ak_1415cdd50", fn_1415cdd50 as *const ()), // sub_1415cdd50
    Replacement::draft(0x1415cdde0, "ak_1415cdde0", fn_1415cdde0 as *const ()), // AK::ReadBytesSkip::AK::ReadBytesSkip::ReadBytes
    Replacement::draft(0x1415cde10, "ak_1415cde10", tpl_28365::<0x8> as *const ()), // AK::WriteBytesCount::AK::WriteBytesCount::SetCount
    Replacement::draft(0x1415cde20, "ak_1415cde20", fn_1415cde20 as *const ()), // AK::WriteBytesCount::AK::WriteBytesCount::WriteBytes
    Replacement::draft(0x1415cde30, "ak_1415cde30", fn_1415cde30 as *const ()), // sub_1415cde30
    Replacement::draft(0x1415cded0, "ak_1415cded0", fn_1415cded0 as *const ()), // sub_1415cded0
    Replacement::draft(0x1415cdf20, "ak_1415cdf20", tpl_27062::<0x14268b678, 0x141834d30> as *const ()), // sub_1415cdf20
    Replacement::draft(0x1415cdf50, "ak_1415cdf50", fn_1415cdf50 as *const ()), // sub_1415cdf50
    Replacement::draft(0x1415cdf80, "ak_1415cdf80", fn_1415cdf80 as *const ()), // sub_1415cdf80
    Replacement::draft(0x1415cdfb0, "ak_1415cdfb0", fn_1415cdfb0 as *const ()), // sub_1415cdfb0
    Replacement::draft(0x1415ce1b0, "ak_1415ce1b0", fn_1415ce1b0 as *const ()), // sub_1415ce1b0
    Replacement::draft(0x1415ce1f0, "ak_1415ce1f0", fn_1415ce1f0 as *const ()), // sub_1415ce1f0
    Replacement::draft(0x1415ce210, "ak_1415ce210", fn_1415ce210 as *const ()), // sub_1415ce210
    Replacement::draft(0x1415ce240, "ak_1415ce240", tpl_29846::<0x8, 0x38> as *const ()), // sub_1415ce240
    Replacement::draft(0x1415ce260, "ak_1415ce260", fn_1415ce260 as *const ()), // sub_1415ce260
    Replacement::draft(0x1415ce390, "ak_1415ce390", fn_1415ce390 as *const ()), // sub_1415ce390
    Replacement::draft(0x1415ce5b0, "ak_1415ce5b0", fn_1415ce5b0 as *const ()), // sub_1415ce5b0
    Replacement::draft(0x1415ce5d0, "ak_1415ce5d0", tpl_497::<0x10> as *const ()), // sub_1415ce5d0
    Replacement::draft(0x1415ce5e0, "ak_1415ce5e0", tpl_29360::<0xd4> as *const ()), // sub_1415ce5e0
    Replacement::draft(0x1415ce5f0, "ak_1415ce5f0", fn_1415ce5f0 as *const ()), // sub_1415ce5f0
    Replacement::draft(0x1415ce680, "ak_1415ce680", fn_1415ce680 as *const ()), // sub_1415ce680
    Replacement::draft(0x1415ce700, "ak_1415ce700", fn_1415ce700 as *const ()), // sub_1415ce700
    Replacement::draft(0x1415ce720, "ak_1415ce720", tpl_29853::<0x1415d2530> as *const ()), // sub_1415ce720
    Replacement::draft(0x1415ce730, "ak_1415ce730", fn_1415ce730 as *const ()), // sub_1415ce730
    Replacement::draft(0x1415ce7b0, "ak_1415ce7b0", tpl_29855::<0x8, 0x30> as *const ()), // sub_1415ce7b0
    Replacement::draft(0x1415ce7d0, "ak_1415ce7d0", fn_1415ce7d0 as *const ()), // sub_1415ce7d0
    Replacement::draft(0x1415ce9d0, "ak_1415ce9d0", fn_1415ce9d0 as *const ()), // sub_1415ce9d0
    Replacement::draft(0x1415cea00, "ak_1415cea00", fn_1415cea00 as *const ()), // sub_1415cea00
    Replacement::draft(0x1415cee20, "ak_1415cee20", fn_1415cee20 as *const ()), // sub_1415cee20
    Replacement::draft(0x1415cee70, "ak_1415cee70", fn_1415cee70 as *const ()), // sub_1415cee70
    Replacement::draft(0x1415cef30, "ak_1415cef30", fn_1415cef30 as *const ()), // sub_1415cef30
    Replacement::draft(0x1415cef40, "ak_1415cef40", fn_1415cef40 as *const ()), // sub_1415cef40
    Replacement::draft(0x1415cf010, "ak_1415cf010", fn_1415cf010 as *const ()), // sub_1415cf010
    Replacement::draft(0x1415cf0f0, "ak_1415cf0f0", fn_1415cf0f0 as *const ()), // sub_1415cf0f0
    Replacement::draft(0x1415cf1e0, "ak_1415cf1e0", tpl_27963::<0x8, 0x28> as *const ()), // sub_1415cf1e0
    Replacement::draft(0x1415cf200, "ak_1415cf200", fn_1415cf200 as *const ()), // sub_1415cf200
    Replacement::draft(0x1415cf2a0, "ak_1415cf2a0", tpl_22867::<0x8, 0x20> as *const ()), // sub_1415cf2a0
    Replacement::draft(0x1415cf2c0, "ak_1415cf2c0", fn_1415cf2c0 as *const ()), // sub_1415cf2c0
    Replacement::draft(0x1415cf2d0, "ak_1415cf2d0", fn_1415cf2d0 as *const ()), // sub_1415cf2d0
    Replacement::draft(0x1415cf2f0, "ak_1415cf2f0", fn_1415cf2f0 as *const ()), // sub_1415cf2f0
    Replacement::draft(0x1415cf330, "ak_1415cf330", fn_1415cf330 as *const ()), // sub_1415cf330
    Replacement::draft(0x1415cf3c0, "ak_1415cf3c0", fn_1415cf3c0 as *const ()), // sub_1415cf3c0
    Replacement::draft(0x1415cf520, "ak_1415cf520", fn_1415cf520 as *const ()), // sub_1415cf520
    Replacement::draft(0x1415cf540, "ak_1415cf540", fn_1415cf540 as *const ()), // sub_1415cf540
    Replacement::draft(0x1415cf5e0, "ak_1415cf5e0", fn_1415cf5e0 as *const ()), // sub_1415cf5e0
    Replacement::draft(0x1415cf650, "unknown_1415cf650", fn_1415cf650 as *const ()), // sub_1415cf650
    Replacement::draft(0x1415cf9e0, "unknown_1415cf9e0", fn_1415cf9e0 as *const ()), // sub_1415cf9e0
    Replacement::draft(0x1415cfa10, "unknown_1415cfa10", tpl_23357::<0x14268b678, 0x141834d30> as *const ()), // sub_1415cfa10
    Replacement::draft(0x1415cfa40, "unknown_1415cfa40", fn_1415cfa40 as *const ()), // sub_1415cfa40
    Replacement::draft(0x1415cfad0, "unknown_1415cfad0", fn_1415cfad0 as *const ()), // sub_1415cfad0
    Replacement::draft(0x1415cfb10, "unknown_1415cfb10", fn_1415cfb10 as *const ()), // sub_1415cfb10
    Replacement::draft(0x1415cfb50, "unknown_1415cfb50", fn_1415cfb50 as *const ()), // sub_1415cfb50
    Replacement::draft(0x1415cfb90, "unknown_1415cfb90", fn_1415cfb90 as *const ()), // sub_1415cfb90
    Replacement::draft(0x1415cfbb0, "unknown_1415cfbb0", fn_1415cfbb0 as *const ()), // sub_1415cfbb0
    Replacement::draft(0x1415cfce0, "unknown_1415cfce0", tpl_27317::<0x18, 0x141595eb0> as *const ()), // sub_1415cfce0
    Replacement::draft(0x1415cfcf0, "unknown_1415cfcf0", fn_1415cfcf0 as *const ()), // sub_1415cfcf0
    Replacement::draft(0x1415cfd50, "unknown_1415cfd50", fn_1415cfd50 as *const ()), // sub_1415cfd50
    Replacement::draft(0x1415cfdf0, "unknown_1415cfdf0", fn_1415cfdf0 as *const ()), // sub_1415cfdf0
    Replacement::draft(0x1415cfe60, "unknown_1415cfe60", fn_1415cfe60 as *const ()), // sub_1415cfe60
    Replacement::draft(0x1415cfe90, "concurrency_1415cfe90", fn_1415cfe90 as *const ()), // Concurrency::details::UMSFreeThreadProxyFactory::Concurrency::details::UMSFreeThreadProxyFactory::`scalar_deleting_destr
    Replacement::draft(0x1415cfee0, "unknown_1415cfee0", fn_1415cfee0 as *const ()), // sub_1415cfee0
    Replacement::draft(0x1415d0020, "unknown_1415d0020", fn_1415d0020 as *const ()), // sub_1415d0020
    Replacement::draft(0x1415d00a0, "unknown_1415d00a0", fn_1415d00a0 as *const ()), // sub_1415d00a0
    Replacement::draft(0x1415d0290, "unknown_1415d0290", tpl_660::<0x18, 0x2a0> as *const ()), // sub_1415d0290
    Replacement::draft(0x1415d02a0, "unknown_1415d02a0", fn_1415d02a0 as *const ()), // sub_1415d02a0
    Replacement::draft(0x1415d0310, "unknown_1415d0310", tpl_28121::<0x54> as *const ()), // sub_1415d0310
    Replacement::draft(0x1415d0320, "unknown_1415d0320", tpl_29853::<0x14162c460> as *const ()), // sub_1415d0320
    Replacement::draft(0x1415d0330, "unknown_1415d0330", fn_1415d0330 as *const ()), // sub_1415d0330
    Replacement::draft(0x1415d0410, "unknown_1415d0410", fn_1415d0410 as *const ()), // sub_1415d0410
    Replacement::draft(0x1415d04f0, "unknown_1415d04f0", fn_1415d04f0 as *const ()), // sub_1415d04f0
    Replacement::draft(0x1415d0610, "ak_1415d0610", fn_1415d0610 as *const ()), // sub_1415d0610
    Replacement::draft(0x1415d06c0, "ak_1415d06c0", fn_1415d06c0 as *const ()), // sub_1415d06c0
    Replacement::draft(0x1415d08f0, "ak_1415d08f0", fn_1415d08f0 as *const ()), // sub_1415d08f0
    Replacement::draft(0x1415d09c0, "ak_1415d09c0", fn_1415d09c0 as *const ()), // sub_1415d09c0
    Replacement::draft(0x1415d0c50, "ak_1415d0c50", fn_1415d0c50 as *const ()), // sub_1415d0c50
    Replacement::draft(0x1415d0c90, "ak_1415d0c90", fn_1415d0c90 as *const ()), // sub_1415d0c90
    Replacement::draft(0x1415d0d30, "ak_1415d0d30", fn_1415d0d30 as *const ()), // sub_1415d0d30
    Replacement::draft(0x1415d0df0, "ak_1415d0df0", fn_1415d0df0 as *const ()), // sub_1415d0df0
    Replacement::draft(0x1415d0e90, "ak_1415d0e90", fn_1415d0e90 as *const ()), // sub_1415d0e90
    Replacement::draft(0x1415d0ee0, "ak_1415d0ee0", fn_1415d0ee0 as *const ()), // sub_1415d0ee0
    Replacement::draft(0x1415d1020, "ak_1415d1020", fn_1415d1020 as *const ()), // sub_1415d1020
    Replacement::draft(0x1415d11a0, "ak_1415d11a0", fn_1415d11a0 as *const ()), // sub_1415d11a0
    Replacement::draft(0x1415d11f0, "ak_1415d11f0", fn_1415d11f0 as *const ()), // sub_1415d11f0
    Replacement::draft(0x1415d1250, "ak_1415d1250", fn_1415d1250 as *const ()), // sub_1415d1250
    Replacement::draft(0x1415d12d0, "ak_1415d12d0", fn_1415d12d0 as *const ()), // sub_1415d12d0
    Replacement::draft(0x1415d1340, "ak_1415d1340", fn_1415d1340 as *const ()), // sub_1415d1340
    Replacement::draft(0x1415d1520, "ak_1415d1520", fn_1415d1520 as *const ()), // sub_1415d1520
    Replacement::draft(0x1415d15e0, "ak_1415d15e0", fn_1415d15e0 as *const ()), // sub_1415d15e0
    Replacement::draft(0x1415d1600, "ak_1415d1600", fn_1415d1600 as *const ()), // sub_1415d1600
    Replacement::draft(0x1415d1690, "ak_1415d1690", fn_1415d1690 as *const ()), // sub_1415d1690
    Replacement::draft(0x1415d16b0, "ak_1415d16b0", fn_1415d16b0 as *const ()), // sub_1415d16b0
    Replacement::draft(0x1415d1840, "ak_1415d1840", fn_1415d1840 as *const ()), // sub_1415d1840
    Replacement::draft(0x1415d1990, "ak_1415d1990", fn_1415d1990 as *const ()), // sub_1415d1990
    Replacement::draft(0x1415d19d0, "ak_1415d19d0", fn_1415d19d0 as *const ()), // sub_1415d19d0
    Replacement::draft(0x1415d1a50, "ak_1415d1a50", fn_1415d1a50 as *const ()), // sub_1415d1a50
    Replacement::draft(0x1415d1b10, "ak_1415d1b10", fn_1415d1b10 as *const ()), // sub_1415d1b10
    Replacement::draft(0x1415d1b90, "ak_1415d1b90", fn_1415d1b90 as *const ()), // sub_1415d1b90
    Replacement::draft(0x1415d1bf0, "ak_1415d1bf0", fn_1415d1bf0 as *const ()), // sub_1415d1bf0
    Replacement::draft(0x1415d1c20, "ak_1415d1c20", fn_1415d1c20 as *const ()), // sub_1415d1c20
    Replacement::draft(0x1415d1ce0, "ak_1415d1ce0", fn_1415d1ce0 as *const ()), // sub_1415d1ce0
    Replacement::draft(0x1415d1d10, "ak_1415d1d10", fn_1415d1d10 as *const ()), // sub_1415d1d10
    Replacement::draft(0x1415d1d90, "ak_1415d1d90", fn_1415d1d90 as *const ()), // sub_1415d1d90
    Replacement::draft(0x1415d1e60, "ak_1415d1e60", fn_1415d1e60 as *const ()), // sub_1415d1e60
    Replacement::draft(0x1415d1ed0, "ak_1415d1ed0", fn_1415d1ed0 as *const ()), // sub_1415d1ed0
    Replacement::draft(0x1415d2000, "ak_1415d2000", fn_1415d2000 as *const ()), // sub_1415d2000
    Replacement::draft(0x1415d2150, "ak_1415d2150", fn_1415d2150 as *const ()), // sub_1415d2150
    Replacement::draft(0x1415d2230, "ak_1415d2230", fn_1415d2230 as *const ()), // sub_1415d2230
    Replacement::draft(0x1415d2480, "ak_1415d2480", fn_1415d2480 as *const ()), // sub_1415d2480
    Replacement::draft(0x1415d2530, "ak_1415d2530", fn_1415d2530 as *const ()), // sub_1415d2530
    Replacement::draft(0x1415d2580, "ak_1415d2580", fn_1415d2580 as *const ()), // sub_1415d2580
    Replacement::draft(0x1415d26b0, "ak_1415d26b0", fn_1415d26b0 as *const ()), // sub_1415d26b0
    Replacement::draft(0x1415d2710, "ak_1415d2710", fn_1415d2710 as *const ()), // sub_1415d2710
    Replacement::draft(0x1415d27a0, "ak_1415d27a0", fn_1415d27a0 as *const ()), // sub_1415d27a0
    Replacement::draft(0x1415d27d0, "ak_1415d27d0", fn_1415d27d0 as *const ()), // sub_1415d27d0
    Replacement::draft(0x1415d2870, "ak_1415d2870", fn_1415d2870 as *const ()), // sub_1415d2870
    Replacement::draft(0x1415d2980, "ak_1415d2980", fn_1415d2980 as *const ()), // sub_1415d2980
    Replacement::draft(0x1415d29d0, "ak_1415d29d0", fn_1415d29d0 as *const ()), // sub_1415d29d0
    Replacement::draft(0x1415d2a30, "ak_1415d2a30", fn_1415d2a30 as *const ()), // sub_1415d2a30
    Replacement::draft(0x1415d2b50, "ak_1415d2b50", fn_1415d2b50 as *const ()), // sub_1415d2b50
    Replacement::draft(0x1415d2c50, "ak_1415d2c50", fn_1415d2c50 as *const ()), // sub_1415d2c50
    Replacement::draft(0x1415d2c60, "ak_1415d2c60", fn_1415d2c60 as *const ()), // sub_1415d2c60
    Replacement::draft(0x1415d2ce0, "ak_1415d2ce0", fn_1415d2ce0 as *const ()), // sub_1415d2ce0
    Replacement::draft(0x1415d2d10, "ak_1415d2d10", fn_1415d2d10 as *const ()), // sub_1415d2d10
    Replacement::draft(0x1415d2d30, "ak_1415d2d30", fn_1415d2d30 as *const ()), // sub_1415d2d30
    Replacement::draft(0x1415d2d60, "ak_1415d2d60", fn_1415d2d60 as *const ()), // sub_1415d2d60
    Replacement::draft(0x1415d2da0, "ak_1415d2da0", fn_1415d2da0 as *const ()), // sub_1415d2da0
    Replacement::draft(0x1415d2dc0, "ak_1415d2dc0", fn_1415d2dc0 as *const ()), // sub_1415d2dc0
    Replacement::draft_throws(0x1415d2e40, "ak_1415d2e40", fn_1415d2e40 as *const (), Throws::Discard), // sub_1415d2e40
    Replacement::draft(0x1415d2f10, "ak_1415d2f10", fn_1415d2f10 as *const ()), // sub_1415d2f10
    Replacement::draft(0x1415d2f70, "ak_1415d2f70", tpl_25145::<0x142645560> as *const ()), // sub_1415d2f70
    Replacement::draft(0x1415d2f80, "ak_1415d2f80", fn_1415d2f80 as *const ()), // sub_1415d2f80
    Replacement::draft(0x1415d3010, "ak_1415d3010", fn_1415d3010 as *const ()), // sub_1415d3010
    Replacement::draft(0x1415d30f0, "ak_1415d30f0", fn_1415d30f0 as *const ()), // sub_1415d30f0
    Replacement::draft(0x1415d3290, "ak_1415d3290", fn_1415d3290 as *const ()), // sub_1415d3290
    Replacement::draft(0x1415d3350, "ak_1415d3350", fn_1415d3350 as *const ()), // sub_1415d3350
    Replacement::draft(0x1415d3d00, "ak_1415d3d00", fn_1415d3d00 as *const ()), // sub_1415d3d00
    Replacement::draft(0x1415d4390, "ak_1415d4390", fn_1415d4390 as *const ()), // sub_1415d4390
    Replacement::draft(0x1415d44d0, "ak_1415d44d0", fn_1415d44d0 as *const ()), // sub_1415d44d0
    Replacement::draft(0x1415d4610, "ak_1415d4610", tpl_29129::<0x10, 0x14268bb38, 0x18, 0x14392d470, 0x14392d468, 0x14392d460, 0x142688f88, 0x141834d30> as *const ()), // sub_1415d4610
    Replacement::draft(0x1415d46a0, "ak_1415d46a0", fn_1415d46a0 as *const ()), // sub_1415d46a0
    Replacement::draft(0x1415d4800, "ak_1415d4800", fn_1415d4800 as *const ()), // sub_1415d4800
    Replacement::draft(0x1415d5240, "ak_1415d5240", tpl_365::<0x1415d3350> as *const ()), // sub_1415d5240
    Replacement::draft(0x1415d5250, "ak_1415d5250", fn_1415d5250 as *const ()), // sub_1415d5250
    Replacement::draft(0x1415d55a0, "ak_1415d55a0", fn_1415d55a0 as *const ()), // sub_1415d55a0
    Replacement::draft(0x1415d5a00, "ak_1415d5a00", fn_1415d5a00 as *const ()), // sub_1415d5a00
    Replacement::draft(0x1415d6100, "ak_1415d6100", fn_1415d6100 as *const ()), // sub_1415d6100
    Replacement::draft(0x1415d63c0, "ak_1415d63c0", fn_1415d63c0 as *const ()), // sub_1415d63c0
    Replacement::draft(0x1415d6570, "ak_1415d6570", fn_1415d6570 as *const ()), // sub_1415d6570
    Replacement::draft(0x1415d75c0, "ak_1415d75c0", fn_1415d75c0 as *const ()), // sub_1415d75c0
    Replacement::draft(0x1415d7890, "ak_1415d7890", tpl_365::<0x1415d3d00> as *const ()), // sub_1415d7890
    Replacement::draft(0x1415d78a0, "ak_1415d78a0", fn_1415d78a0 as *const ()), // sub_1415d78a0
    Replacement::draft(0x1415d7950, "ak_1415d7950", fn_1415d7950 as *const ()), // sub_1415d7950
    Replacement::draft(0x1415d7980, "ak_1415d7980", fn_1415d7980 as *const ()), // sub_1415d7980
    Replacement::draft(0x1415d79c0, "ak_1415d79c0", fn_1415d79c0 as *const ()), // sub_1415d79c0
    Replacement::draft(0x1415d7a40, "ak_1415d7a40", fn_1415d7a40 as *const ()), // sub_1415d7a40
    Replacement::draft(0x1415d7aa0, "ak_1415d7aa0", fn_1415d7aa0 as *const ()), // sub_1415d7aa0
    Replacement::draft(0x1415d7b20, "ak_1415d7b20", fn_1415d7b20 as *const ()), // sub_1415d7b20
    Replacement::draft(0x1415d8010, "ak_1415d8010", fn_1415d8010 as *const ()), // sub_1415d8010
    Replacement::draft(0x1415d8040, "ak_1415d8040", fn_1415d8040 as *const ()), // sub_1415d8040
    Replacement::draft(0x1415d8100, "ak_1415d8100", fn_1415d8100 as *const ()), // sub_1415d8100
    Replacement::draft(0x1415d8150, "ak_1415d8150", fn_1415d8150 as *const ()), // sub_1415d8150
    Replacement::draft(0x1415d8270, "ak_1415d8270", fn_1415d8270 as *const ()), // sub_1415d8270
    Replacement::draft(0x1415d87e0, "ak_1415d87e0", fn_1415d87e0 as *const ()), // sub_1415d87e0
    Replacement::draft(0x1415d8820, "ak_1415d8820", fn_1415d8820 as *const ()), // sub_1415d8820
    Replacement::draft(0x1415d8850, "ak_1415d8850", fn_1415d8850 as *const ()), // sub_1415d8850
    Replacement::draft(0x1415d8920, "ak_1415d8920", fn_1415d8920 as *const ()), // sub_1415d8920
    Replacement::draft(0x1415d89a0, "ak_1415d89a0", fn_1415d89a0 as *const ()), // sub_1415d89a0
    Replacement::draft(0x1415d8ec0, "ak_1415d8ec0", fn_1415d8ec0 as *const ()), // sub_1415d8ec0
    Replacement::draft(0x1415d90a0, "ak_1415d90a0", fn_1415d90a0 as *const ()), // sub_1415d90a0
    Replacement::draft(0x1415d91e0, "ak_1415d91e0", fn_1415d91e0 as *const ()), // sub_1415d91e0
    Replacement::draft(0x1415d9400, "ak_1415d9400", fn_1415d9400 as *const ()), // sub_1415d9400
    Replacement::draft(0x1415d9470, "ak_1415d9470", fn_1415d9470 as *const ()), // sub_1415d9470
    Replacement::draft(0x1415d9480, "ak_1415d9480", fn_1415d9480 as *const ()), // sub_1415d9480
    Replacement::draft(0x1415d95c0, "ak_1415d95c0", fn_1415d95c0 as *const ()), // sub_1415d95c0
    Replacement::draft(0x1415d9610, "ak_1415d9610", fn_1415d9610 as *const ()), // sub_1415d9610
    Replacement::draft(0x1415d9620, "ak_1415d9620", fn_1415d9620 as *const ()), // sub_1415d9620
    Replacement::draft(0x1415d9820, "ak_1415d9820", fn_1415d9820 as *const ()), // sub_1415d9820
    Replacement::draft(0x1415d99c0, "ak_1415d99c0", fn_1415d99c0 as *const ()), // sub_1415d99c0
    Replacement::draft(0x1415d9aa0, "ak_1415d9aa0", fn_1415d9aa0 as *const ()), // sub_1415d9aa0
    Replacement::draft(0x1415d9b90, "ak_1415d9b90", fn_1415d9b90 as *const ()), // sub_1415d9b90
    Replacement::draft(0x1415d9ce0, "ak_1415d9ce0", fn_1415d9ce0 as *const ()), // sub_1415d9ce0
    Replacement::draft(0x1415d9f00, "ak_1415d9f00", fn_1415d9f00 as *const ()), // sub_1415d9f00
    Replacement::draft(0x1415da060, "ak_1415da060", fn_1415da060 as *const ()), // sub_1415da060
    Replacement::draft(0x1415da460, "ak_1415da460", fn_1415da460 as *const ()), // sub_1415da460
    Replacement::draft(0x1415da800, "ak_1415da800", fn_1415da800 as *const ()), // sub_1415da800
    Replacement::draft(0x1415da860, "ak_1415da860", fn_1415da860 as *const ()), // sub_1415da860
    Replacement::draft(0x1415da8d0, "ak_1415da8d0", fn_1415da8d0 as *const ()), // sub_1415da8d0
    Replacement::draft(0x1415da9d0, "ak_1415da9d0", fn_1415da9d0 as *const ()), // sub_1415da9d0
    Replacement::draft(0x1415db790, "ak_1415db790", fn_1415db790 as *const ()), // sub_1415db790
    Replacement::draft(0x1415db9a0, "ak_1415db9a0", fn_1415db9a0 as *const ()), // sub_1415db9a0
    Replacement::draft(0x1415dbba0, "ak_1415dbba0", fn_1415dbba0 as *const ()), // sub_1415dbba0
    Replacement::draft(0x1415dbd30, "ak_1415dbd30", fn_1415dbd30 as *const ()), // sub_1415dbd30
    Replacement::draft(0x1415dc3f0, "ak_1415dc3f0", fn_1415dc3f0 as *const ()), // sub_1415dc3f0
    Replacement::draft(0x1415dc7d0, "ak_1415dc7d0", fn_1415dc7d0 as *const ()), // sub_1415dc7d0
    Replacement::draft(0x1415dc9f0, "ak_1415dc9f0", fn_1415dc9f0 as *const ()), // sub_1415dc9f0
    Replacement::draft(0x1415dcc10, "ak_1415dcc10", fn_1415dcc10 as *const ()), // sub_1415dcc10
    Replacement::draft(0x1415dd640, "ak_1415dd640", fn_1415dd640 as *const ()), // sub_1415dd640
    Replacement::draft(0x1415ddee0, "ak_1415ddee0", tpl_88721::<0x14268da80, 0x14268da20> as *const ()), // sub_1415ddee0
    Replacement::draft(0x1415de040, "ak_1415de040", tpl_88721::<0x14268da40, 0x14268d9ec> as *const ()), // sub_1415de040
    Replacement::draft(0x1415de1a0, "ak_1415de1a0", fn_1415de1a0 as *const ()), // sub_1415de1a0
    Replacement::draft(0x1415de330, "ak_1415de330", fn_1415de330 as *const ()), // sub_1415de330
    Replacement::draft(0x1415dea90, "ak_1415dea90", fn_1415dea90 as *const ()), // sub_1415dea90
    Replacement::draft(0x1415ded80, "ak_1415ded80", fn_1415ded80 as *const ()), // sub_1415ded80
    Replacement::draft(0x1415def20, "ak_1415def20", fn_1415def20 as *const ()), // sub_1415def20
    Replacement::draft(0x1415df380, "ak_1415df380", fn_1415df380 as *const ()), // sub_1415df380
    Replacement::draft(0x1415df500, "ak_1415df500", fn_1415df500 as *const ()), // sub_1415df500
    Replacement::draft(0x1415e0010, "ak_1415e0010", fn_1415e0010 as *const ()), // sub_1415e0010
    Replacement::draft(0x1415e0180, "ak_1415e0180", fn_1415e0180 as *const ()), // sub_1415e0180
    Replacement::draft(0x1415e0380, "ak_1415e0380", fn_1415e0380 as *const ()), // sub_1415e0380
    Replacement::draft(0x1415e0400, "ak_1415e0400", fn_1415e0400 as *const ()), // CAkSpeakerPan::CAkSpeakerPan::EncodeVector
    Replacement::draft(0x1415e0680, "ak_1415e0680", fn_1415e0680 as *const ()), // sub_1415e0680
    Replacement::draft(0x1415e0780, "ak_1415e0780", fn_1415e0780 as *const ()), // sub_1415e0780
    Replacement::draft(0x1415e07e0, "ak_1415e07e0", fn_1415e07e0 as *const ()), // sub_1415e07e0
    Replacement::draft(0x1415e0870, "ak_1415e0870", fn_1415e0870 as *const ()), // sub_1415e0870
    Replacement::draft(0x1415e0910, "ak_1415e0910", fn_1415e0910 as *const ()), // CAkSpeakerPan::CAkSpeakerPan::GetDecodingMatrixShape
    Replacement::draft(0x1415e0940, "ak_1415e0940", fn_1415e0940 as *const ()), // sub_1415e0940
    Replacement::draft(0x1415e0960, "ak_1415e0960", fn_1415e0960 as *const ()), // sub_1415e0960
    Replacement::draft(0x1415e0db0, "ak_1415e0db0", fn_1415e0db0 as *const ()), // sub_1415e0db0
    Replacement::draft(0x1415e0e50, "ak_1415e0e50", fn_1415e0e50 as *const ()), // sub_1415e0e50
    Replacement::draft(0x1415e0e60, "ak_1415e0e60", fn_1415e0e60 as *const ()), // sub_1415e0e60
    Replacement::draft(0x1415e1190, "ak_1415e1190", fn_1415e1190 as *const ()), // sub_1415e1190
    Replacement::draft(0x1415e1380, "ak_1415e1380", fn_1415e1380 as *const ()), // sub_1415e1380
    Replacement::draft(0x1415e1400, "ak_1415e1400", fn_1415e1400 as *const ()), // sub_1415e1400
    Replacement::draft(0x1415e1730, "ak_1415e1730", fn_1415e1730 as *const ()), // sub_1415e1730
    Replacement::draft(0x1415e1980, "ak_1415e1980", fn_1415e1980 as *const ()), // sub_1415e1980
    Replacement::draft(0x1415e3d40, "ak_1415e3d40", fn_1415e3d40 as *const ()), // sub_1415e3d40
    Replacement::draft(0x1415e3d80, "ak_1415e3d80", fn_1415e3d80 as *const ()), // sub_1415e3d80
    Replacement::draft(0x1415e3fb0, "ak_1415e3fb0", fn_1415e3fb0 as *const ()), // sub_1415e3fb0
    Replacement::draft(0x1415e4060, "ak_1415e4060", fn_1415e4060 as *const ()), // sub_1415e4060
    Replacement::draft(0x1415e4130, "ak_1415e4130", fn_1415e4130 as *const ()), // sub_1415e4130
    Replacement::draft(0x1415e4220, "ak_1415e4220", fn_1415e4220 as *const ()), // sub_1415e4220
    Replacement::draft(0x1415e4300, "ak_1415e4300", fn_1415e4300 as *const ()), // sub_1415e4300
    Replacement::draft(0x1415e4370, "ak_1415e4370", fn_1415e4370 as *const ()), // sub_1415e4370
    Replacement::draft(0x1415e43c0, "ak_1415e43c0", fn_1415e43c0 as *const ()), // sub_1415e43c0
    Replacement::draft(0x1415e4800, "ak_1415e4800", fn_1415e4800 as *const ()), // sub_1415e4800
    Replacement::draft(0x1415e4b40, "ak_1415e4b40", fn_1415e4b40 as *const ()), // sub_1415e4b40
    Replacement::draft(0x1415e4c10, "ak_1415e4c10", fn_1415e4c10 as *const ()), // sub_1415e4c10
    Replacement::draft(0x1415e5030, "ak_1415e5030", fn_1415e5030 as *const ()), // sub_1415e5030
    Replacement::draft(0x1415e58a0, "ak_1415e58a0", fn_1415e58a0 as *const ()), // sub_1415e58a0
    Replacement::draft(0x1415e5c40, "ak_1415e5c40", fn_1415e5c40 as *const ()), // sub_1415e5c40
    Replacement::draft(0x1415e5fa0, "ak_1415e5fa0", fn_1415e5fa0 as *const ()), // sub_1415e5fa0
    Replacement::draft(0x1415e6040, "ak_1415e6040", fn_1415e6040 as *const ()), // sub_1415e6040
    Replacement::draft(0x1415e6080, "ak_1415e6080", fn_1415e6080 as *const ()), // sub_1415e6080
    Replacement::draft(0x1415e60f0, "ak_1415e60f0", fn_1415e60f0 as *const ()), // sub_1415e60f0
    Replacement::draft(0x1415e68b0, "ak_1415e68b0", fn_1415e68b0 as *const ()), // sub_1415e68b0
    Replacement::draft(0x1415e6c90, "ak_1415e6c90", fn_1415e6c90 as *const ()), // sub_1415e6c90
    Replacement::draft(0x1415e6d20, "ak_1415e6d20", fn_1415e6d20 as *const ()), // sub_1415e6d20
    Replacement::draft(0x1415e6e10, "ak_1415e6e10", fn_1415e6e10 as *const ()), // sub_1415e6e10
    Replacement::draft(0x1415e6ea0, "ak_1415e6ea0", fn_1415e6ea0 as *const ()), // sub_1415e6ea0
    Replacement::draft(0x1415e6f80, "ak_1415e6f80", fn_1415e6f80 as *const ()), // sub_1415e6f80
    Replacement::draft(0x1415e7080, "ak_1415e7080", fn_1415e7080 as *const ()), // sub_1415e7080
    Replacement::draft(0x1415e71a0, "ak_1415e71a0", fn_1415e71a0 as *const ()), // sub_1415e71a0
    Replacement::draft(0x1415e7330, "ak_1415e7330", fn_1415e7330 as *const ()), // sub_1415e7330
    Replacement::draft(0x1415e7860, "ak_1415e7860", fn_1415e7860 as *const ()), // sub_1415e7860
    Replacement::draft(0x1415e78e0, "ak_1415e78e0", fn_1415e78e0 as *const ()), // sub_1415e78e0
    Replacement::draft(0x1415e7ac0, "ak_1415e7ac0", fn_1415e7ac0 as *const ()), // sub_1415e7ac0
    Replacement::draft(0x1415e7c50, "ak_1415e7c50", fn_1415e7c50 as *const ()), // sub_1415e7c50
    Replacement::draft(0x1415e7cb0, "ak_1415e7cb0", fn_1415e7cb0 as *const ()), // sub_1415e7cb0
    Replacement::draft(0x1415e7cf0, "ak_1415e7cf0", fn_1415e7cf0 as *const ()), // sub_1415e7cf0
    Replacement::draft(0x1415e7d40, "ak_1415e7d40", tpl_6019::<0x14268dac0, 0x141834d30> as *const ()), // sub_1415e7d40
    Replacement::draft(0x1415e7d70, "ak_1415e7d70", fn_1415e7d70 as *const ()), // sub_1415e7d70
    Replacement::draft(0x1415e7dd0, "ak_1415e7dd0", tpl_893::<0x14268dac0, 0x141834d30> as *const ()), // sub_1415e7dd0
    Replacement::draft(0x1415e7e00, "ak_1415e7e00", fn_1415e7e00 as *const ()), // sub_1415e7e00
    Replacement::draft(0x1415e7e50, "ak_1415e7e50", tpl_22865::<0x14268dac0, 0x141834d30> as *const ()), // sub_1415e7e50
    Replacement::draft(0x1415e7e80, "ak_1415e7e80", fn_1415e7e80 as *const ()), // sub_1415e7e80
    Replacement::draft(0x1415e7ed0, "ak_1415e7ed0", fn_1415e7ed0 as *const ()), // sub_1415e7ed0
    Replacement::draft(0x1415e7f70, "ak_1415e7f70", fn_1415e7f70 as *const ()), // sub_1415e7f70
    Replacement::draft(0x1415e8060, "ak_1415e8060", fn_1415e8060 as *const ()), // sub_1415e8060
    Replacement::draft(0x1415e80f0, "ak_1415e80f0", fn_1415e80f0 as *const ()), // sub_1415e80f0
    Replacement::draft(0x1415e81d0, "ak_1415e81d0", fn_1415e81d0 as *const ()), // sub_1415e81d0
    Replacement::draft(0x1415e81f0, "ak_1415e81f0", fn_1415e81f0 as *const ()), // sub_1415e81f0
    Replacement::draft(0x1415e8210, "ak_1415e8210", fn_1415e8210 as *const ()), // sub_1415e8210
    Replacement::draft(0x1415e8260, "ak_1415e8260", fn_1415e8260 as *const ()), // sub_1415e8260
    Replacement::draft(0x1415e82e0, "ak_1415e82e0", fn_1415e82e0 as *const ()), // sub_1415e82e0
    Replacement::draft(0x1415e8370, "ak_1415e8370", fn_1415e8370 as *const ()), // sub_1415e8370
    Replacement::draft(0x1415e83a0, "ak_1415e83a0", fn_1415e83a0 as *const ()), // sub_1415e83a0
    Replacement::draft(0x1415e84f0, "ak_1415e84f0", fn_1415e84f0 as *const ()), // sub_1415e84f0
    Replacement::draft(0x1415e8580, "ak_1415e8580", fn_1415e8580 as *const ()), // sub_1415e8580
    Replacement::draft(0x1415e87c0, "ak_1415e87c0", fn_1415e87c0 as *const ()), // sub_1415e87c0
    Replacement::draft(0x1415e8ba0, "ak_1415e8ba0", fn_1415e8ba0 as *const ()), // sub_1415e8ba0
    Replacement::draft(0x1415e9010, "ak_1415e9010", fn_1415e9010 as *const ()), // sub_1415e9010
    Replacement::draft(0x1415e9070, "ak_1415e9070", fn_1415e9070 as *const ()), // sub_1415e9070
    Replacement::draft(0x1415e9090, "ak_1415e9090", fn_1415e9090 as *const ()), // sub_1415e9090
    Replacement::draft(0x1415e90b0, "ak_1415e90b0", tpl_365::<0x1415e9510> as *const ()), // sub_1415e90b0
    Replacement::draft(0x1415e90c0, "ak_1415e90c0", fn_1415e90c0 as *const ()), // sub_1415e90c0
    Replacement::draft(0x1415e9220, "ak_1415e9220", fn_1415e9220 as *const ()), // sub_1415e9220
    Replacement::draft(0x1415e9250, "ak_1415e9250", fn_1415e9250 as *const ()), // sub_1415e9250
    Replacement::draft(0x1415e9380, "ak_1415e9380", fn_1415e9380 as *const ()), // sub_1415e9380
    Replacement::draft(0x1415e93b0, "ak_1415e93b0", fn_1415e93b0 as *const ()), // sub_1415e93b0
    Replacement::draft(0x1415e9430, "ak_1415e9430", fn_1415e9430 as *const ()), // sub_1415e9430
    Replacement::draft(0x1415e9510, "unknown_1415e9510", fn_1415e9510 as *const ()), // sub_1415e9510
    Replacement::draft(0x1415e95b0, "unknown_1415e95b0", fn_1415e95b0 as *const ()), // sub_1415e95b0
    Replacement::draft(0x1415e9740, "unknown_1415e9740", tpl_28964::<0x143931bd8> as *const ()), // sub_1415e9740
    Replacement::draft(0x1415e9750, "unknown_1415e9750", fn_1415e9750 as *const ()), // sub_1415e9750
    Replacement::draft(0x1415e9770, "unknown_1415e9770", fn_1415e9770 as *const ()), // sub_1415e9770
    Replacement::draft(0x1415e97a0, "unknown_1415e97a0", fn_1415e97a0 as *const ()), // sub_1415e97a0
    Replacement::draft(0x1415e97b0, "unknown_1415e97b0", fn_1415e97b0 as *const ()), // sub_1415e97b0
    Replacement::draft(0x1415e97c0, "unknown_1415e97c0", fn_1415e97c0 as *const ()), // sub_1415e97c0
    Replacement::draft(0x1415e97d0, "unknown_1415e97d0", fn_1415e97d0 as *const ()), // sub_1415e97d0
    Replacement::draft(0x1415e9810, "j_ak_1415e9810", fn_1415e9810 as *const ()), // sub_1415e9810
    Replacement::draft(0x1415e9830, "j_ak_1415e9830", fn_1415e9830 as *const ()), // sub_1415e9830
    Replacement::draft(0x1415e9a10, "j_ak_1415e9a10", fn_1415e9a10 as *const ()), // sub_1415e9a10
    Replacement::draft_throws(0x1415e9a50, "unknown_1415e9a50", fn_1415e9a50 as *const (), Throws::Discard), // sub_1415e9a50
    Replacement::draft(0x1415e9c80, "unknown_1415e9c80", fn_1415e9c80 as *const ()), // sub_1415e9c80
    Replacement::draft(0x1415e9d20, "unknown_1415e9d20", fn_1415e9d20 as *const ()), // sub_1415e9d20
    Replacement::draft(0x1415ea130, "unknown_1415ea130", fn_1415ea130 as *const ()), // sub_1415ea130
    Replacement::draft(0x1415ea220, "unknown_1415ea220", tpl_28191::<0x1415ea270> as *const ()), // sub_1415ea220
    Replacement::draft(0x1415ea230, "ak_1415ea230", fn_1415ea230 as *const ()), // sub_1415ea230
    Replacement::draft(0x1415ea250, "unknown_1415ea250", tpl_30049::<0x10, 0x565> as *const ()), // sub_1415ea250
    Replacement::draft(0x1415ea260, "unknown_1415ea260", tpl_497::<0x50> as *const ()), // sub_1415ea260
    Replacement::draft(0x1415ea270, "unknown_1415ea270", fn_1415ea270 as *const ()), // sub_1415ea270
    Replacement::draft(0x1415ea780, "unknown_1415ea780", fn_1415ea780 as *const ()), // sub_1415ea780
    Replacement::draft(0x1415eaa00, "unknown_1415eaa00", fn_1415eaa00 as *const ()), // sub_1415eaa00
    Replacement::draft(0x1415eac20, "unknown_1415eac20", fn_1415eac20 as *const ()), // sub_1415eac20
    Replacement::draft(0x1415eac50, "unknown_1415eac50", fn_1415eac50 as *const ()), // sub_1415eac50
    Replacement::draft(0x1415ead30, "unknown_1415ead30", fn_1415ead30 as *const ()), // sub_1415ead30
    Replacement::draft(0x1415eae50, "unknown_1415eae50", fn_1415eae50 as *const ()), // sub_1415eae50
    Replacement::draft(0x1415eae70, "unknown_1415eae70", fn_1415eae70 as *const ()), // sub_1415eae70
    Replacement::draft(0x1415eafb0, "unknown_1415eafb0", fn_1415eafb0 as *const ()), // sub_1415eafb0
    Replacement::draft(0x1415eb170, "unknown_1415eb170", fn_1415eb170 as *const ()), // sub_1415eb170
    Replacement::draft(0x1415eb190, "unknown_1415eb190", fn_1415eb190 as *const ()), // sub_1415eb190
    Replacement::draft(0x1415eb200, "unknown_1415eb200", fn_1415eb200 as *const ()), // sub_1415eb200
    Replacement::draft(0x1415eb3c0, "unknown_1415eb3c0", fn_1415eb3c0 as *const ()), // sub_1415eb3c0
    Replacement::draft(0x1415eb450, "unknown_1415eb450", tpl_30063::<0x1415e58a0> as *const ()), // sub_1415eb450
    Replacement::draft(0x1415eb460, "unknown_1415eb460", fn_1415eb460 as *const ()), // sub_1415eb460
    Replacement::draft(0x1415eb4a0, "unknown_1415eb4a0", fn_1415eb4a0 as *const ()), // sub_1415eb4a0
    Replacement::draft(0x1415eb630, "unknown_1415eb630", fn_1415eb630 as *const ()), // sub_1415eb630
    Replacement::draft(0x1415eb7a0, "unknown_1415eb7a0", fn_1415eb7a0 as *const ()), // sub_1415eb7a0
    Replacement::draft(0x1415eb820, "unknown_1415eb820", fn_1415eb820 as *const ()), // sub_1415eb820
    Replacement::draft(0x1415eb8d0, "unknown_1415eb8d0", fn_1415eb8d0 as *const ()), // sub_1415eb8d0
    Replacement::draft(0x1415eb9b0, "unknown_1415eb9b0", fn_1415eb9b0 as *const ()), // sub_1415eb9b0
    Replacement::draft(0x1415eba50, "unknown_1415eba50", fn_1415eba50 as *const ()), // sub_1415eba50
    Replacement::draft(0x1415ebc30, "unknown_1415ebc30", fn_1415ebc30 as *const ()), // sub_1415ebc30
    Replacement::draft(0x1415ebcd0, "unknown_1415ebcd0", fn_1415ebcd0 as *const ()), // sub_1415ebcd0
    Replacement::draft(0x1415ebeb0, "unknown_1415ebeb0", fn_1415ebeb0 as *const ()), // sub_1415ebeb0
    Replacement::draft(0x1415ec240, "unknown_1415ec240", fn_1415ec240 as *const ()), // sub_1415ec240
    Replacement::draft(0x1415ec610, "unknown_1415ec610", fn_1415ec610 as *const ()), // sub_1415ec610
    Replacement::draft(0x1415ec900, "unknown_1415ec900", fn_1415ec900 as *const ()), // sub_1415ec900
    Replacement::draft(0x1415ec9d0, "unknown_1415ec9d0", fn_1415ec9d0 as *const ()), // sub_1415ec9d0
    Replacement::draft(0x1415ecaa0, "unknown_1415ecaa0", tpl_30071::<0x68> as *const ()), // sub_1415ecaa0
    Replacement::draft(0x1415ecab0, "unknown_1415ecab0", fn_1415ecab0 as *const ()), // sub_1415ecab0
    Replacement::draft(0x1415ecaf0, "unknown_1415ecaf0", fn_1415ecaf0 as *const ()), // sub_1415ecaf0
    Replacement::draft(0x1415ecd00, "unknown_1415ecd00", fn_1415ecd00 as *const ()), // sub_1415ecd00
    Replacement::draft(0x1415ecf90, "ak_1415ecf90", fn_1415ecf90 as *const ()), // sub_1415ecf90
    Replacement::draft(0x1415ed030, "unknown_1415ed030", fn_1415ed030 as *const ()), // sub_1415ed030
    Replacement::draft(0x1415ed0b0, "ak_1415ed0b0", fn_1415ed0b0 as *const ()), // sub_1415ed0b0
    Replacement::draft(0x1415ed0d0, "unknown_1415ed0d0", fn_1415ed0d0 as *const ()), // sub_1415ed0d0
    Replacement::draft(0x1415ed220, "unknown_1415ed220", fn_1415ed220 as *const ()), // sub_1415ed220
    Replacement::draft(0x1415ed340, "ak_1415ed340", fn_1415ed340 as *const ()), // sub_1415ed340
    Replacement::draft(0x1415ed3d0, "unknown_1415ed3d0", fn_1415ed3d0 as *const ()), // sub_1415ed3d0
    Replacement::draft(0x1415ed460, "unknown_1415ed460", tpl_27517::<0x1426452b0> as *const ()), // sub_1415ed460
    Replacement::draft(0x1415ed470, "unknown_1415ed470", fn_1415ed470 as *const ()), // sub_1415ed470
    Replacement::draft(0x1415ed4d0, "unknown_1415ed4d0", fn_1415ed4d0 as *const ()), // sub_1415ed4d0
    Replacement::draft(0x1415ed4f0, "unknown_1415ed4f0", fn_1415ed4f0 as *const ()), // sub_1415ed4f0
    Replacement::draft(0x1415ed510, "unknown_1415ed510", fn_1415ed510 as *const ()), // sub_1415ed510
    Replacement::draft(0x1415ed530, "unknown_1415ed530", fn_1415ed530 as *const ()), // sub_1415ed530
    Replacement::draft(0x1415ed630, "unknown_1415ed630", fn_1415ed630 as *const ()), // sub_1415ed630
    Replacement::draft(0x1415eda90, "unknown_1415eda90", fn_1415eda90 as *const ()), // sub_1415eda90
    Replacement::draft(0x1415edaf0, "unknown_1415edaf0", fn_1415edaf0 as *const ()), // sub_1415edaf0
    Replacement::draft(0x1415edb30, "unknown_1415edb30", fn_1415edb30 as *const ()), // sub_1415edb30
    Replacement::draft(0x1415edbf0, "unknown_1415edbf0", tpl_27517::<0x1426452a8> as *const ()), // sub_1415edbf0
    Replacement::draft(0x1415edc00, "ak_1415edc00", fn_1415edc00 as *const ()), // sub_1415edc00
    Replacement::draft(0x1415edca0, "unknown_1415edca0", fn_1415edca0 as *const ()), // sub_1415edca0
    Replacement::draft(0x1415edcc0, "unknown_1415edcc0", fn_1415edcc0 as *const ()), // sub_1415edcc0
    Replacement::draft(0x1415edd00, "unknown_1415edd00", tpl_365::<0x1415ea220> as *const ()), // sub_1415edd00
    Replacement::draft(0x1415edd10, "ak_1415edd10", fn_1415edd10 as *const ()), // sub_1415edd10
    Replacement::draft(0x1415ede90, "unknown_1415ede90", fn_1415ede90 as *const ()), // sub_1415ede90
    Replacement::draft(0x1415edf00, "unknown_1415edf00", fn_1415edf00 as *const ()), // sub_1415edf00
    Replacement::draft(0x1415edf30, "unknown_1415edf30", fn_1415edf30 as *const ()), // sub_1415edf30
    Replacement::draft(0x1415edf70, "unknown_1415edf70", fn_1415edf70 as *const ()), // sub_1415edf70
    Replacement::draft(0x1415ee0d0, "unknown_1415ee0d0", fn_1415ee0d0 as *const ()), // sub_1415ee0d0
    Replacement::draft(0x1415ee260, "unknown_1415ee260", fn_1415ee260 as *const ()), // sub_1415ee260
    Replacement::draft(0x1415ee770, "unknown_1415ee770", fn_1415ee770 as *const ()), // sub_1415ee770
    Replacement::draft(0x1415ee820, "unknown_1415ee820", fn_1415ee820 as *const ()), // sub_1415ee820
    Replacement::draft(0x1415eebd0, "unknown_1415eebd0", fn_1415eebd0 as *const ()), // sub_1415eebd0
    Replacement::draft(0x1415eee40, "unknown_1415eee40", fn_1415eee40 as *const ()), // sub_1415eee40
    Replacement::draft(0x1415ef1b0, "unknown_1415ef1b0", fn_1415ef1b0 as *const ()), // sub_1415ef1b0
    Replacement::draft(0x1415ef2c0, "unknown_1415ef2c0", fn_1415ef2c0 as *const ()), // sub_1415ef2c0
    Replacement::draft(0x1415ef3a0, "unknown_1415ef3a0", fn_1415ef3a0 as *const ()), // sub_1415ef3a0
    Replacement::draft(0x1415ef710, "unknown_1415ef710", fn_1415ef710 as *const ()), // sub_1415ef710
    Replacement::draft(0x1415efb50, "unknown_1415efb50", fn_1415efb50 as *const ()), // sub_1415efb50
    Replacement::draft(0x1415efd80, "unknown_1415efd80", fn_1415efd80 as *const ()), // sub_1415efd80
    Replacement::draft(0x1415eff20, "unknown_1415eff20", fn_1415eff20 as *const ()), // sub_1415eff20
    Replacement::draft(0x1415f00c0, "unknown_1415f00c0", fn_1415f00c0 as *const ()), // sub_1415f00c0
    Replacement::draft(0x1415f0150, "unknown_1415f0150", fn_1415f0150 as *const ()), // sub_1415f0150
    Replacement::draft(0x1415f01e0, "unknown_1415f01e0", fn_1415f01e0 as *const ()), // sub_1415f01e0
    Replacement::draft(0x1415f03c0, "unknown_1415f03c0", fn_1415f03c0 as *const ()), // sub_1415f03c0
    Replacement::draft(0x1415f0430, "unknown_1415f0430", fn_1415f0430 as *const ()), // sub_1415f0430
    Replacement::draft(0x1415f0550, "unknown_1415f0550", fn_1415f0550 as *const ()), // sub_1415f0550
    Replacement::draft(0x1415f0580, "unknown_1415f0580", fn_1415f0580 as *const ()), // sub_1415f0580
    Replacement::draft(0x1415f06d0, "unknown_1415f06d0", fn_1415f06d0 as *const ()), // sub_1415f06d0
    Replacement::draft(0x1415f06f0, "unknown_1415f06f0", fn_1415f06f0 as *const ()), // sub_1415f06f0
    Replacement::draft(0x1415f0710, "unknown_1415f0710", tpl_29421::<0x1415b27a0, 0x8, 0x1415edf70, 0x1415b1990> as *const ()), // sub_1415f0710
    Replacement::draft(0x1415f0750, "unknown_1415f0750", fn_1415f0750 as *const ()), // sub_1415f0750
    Replacement::draft(0x1415f08e0, "unknown_1415f08e0", fn_1415f08e0 as *const ()), // sub_1415f08e0
    Replacement::draft(0x1415f0950, "unknown_1415f0950", fn_1415f0950 as *const ()), // sub_1415f0950
    Replacement::draft(0x1415f09a0, "unknown_1415f09a0", fn_1415f09a0 as *const ()), // sub_1415f09a0
    Replacement::draft(0x1415f0aa0, "unknown_1415f0aa0", fn_1415f0aa0 as *const ()), // sub_1415f0aa0
    Replacement::draft(0x1415f0c50, "unknown_1415f0c50", fn_1415f0c50 as *const ()), // sub_1415f0c50
    Replacement::draft(0x1415f0d10, "unknown_1415f0d10", fn_1415f0d10 as *const ()), // sub_1415f0d10
    Replacement::draft(0x1415f0e00, "unknown_1415f0e00", fn_1415f0e00 as *const ()), // sub_1415f0e00
    Replacement::draft(0x1415f0ec0, "unknown_1415f0ec0", fn_1415f0ec0 as *const ()), // sub_1415f0ec0
    Replacement::draft(0x1415f1080, "unknown_1415f1080", fn_1415f1080 as *const ()), // sub_1415f1080
    Replacement::draft(0x1415f10a0, "unknown_1415f10a0", fn_1415f10a0 as *const ()), // sub_1415f10a0
    Replacement::draft(0x1415f1190, "unknown_1415f1190", fn_1415f1190 as *const ()), // sub_1415f1190
    Replacement::draft(0x1415f1230, "unknown_1415f1230", fn_1415f1230 as *const ()), // sub_1415f1230
    Replacement::draft(0x1415f12f0, "unknown_1415f12f0", fn_1415f12f0 as *const ()), // sub_1415f12f0
    Replacement::draft(0x1415f1390, "unknown_1415f1390", fn_1415f1390 as *const ()), // sub_1415f1390
    Replacement::draft(0x1415f1800, "unknown_1415f1800", fn_1415f1800 as *const ()), // sub_1415f1800
    Replacement::draft(0x1415f1a90, "unknown_1415f1a90", fn_1415f1a90 as *const ()), // sub_1415f1a90
    Replacement::draft(0x1415f1ab0, "unknown_1415f1ab0", fn_1415f1ab0 as *const ()), // sub_1415f1ab0
    Replacement::draft(0x1415f1bc0, "unknown_1415f1bc0", fn_1415f1bc0 as *const ()), // sub_1415f1bc0
    Replacement::draft(0x1415f1c50, "unknown_1415f1c50", fn_1415f1c50 as *const ()), // sub_1415f1c50
    Replacement::draft(0x1415f1ce0, "unknown_1415f1ce0", fn_1415f1ce0 as *const ()), // sub_1415f1ce0
    Replacement::draft(0x1415f1e90, "unknown_1415f1e90", fn_1415f1e90 as *const ()), // sub_1415f1e90
    Replacement::draft_throws(0x1415f1f40, "unknown_1415f1f40", fn_1415f1f40 as *const (), Throws::Discard), // sub_1415f1f40
    Replacement::draft(0x1415f20f0, "unknown_1415f20f0", fn_1415f20f0 as *const ()), // sub_1415f20f0
    Replacement::draft(0x1415f21a0, "unknown_1415f21a0", fn_1415f21a0 as *const ()), // sub_1415f21a0
    Replacement::draft(0x1415f2390, "unknown_1415f2390", fn_1415f2390 as *const ()), // sub_1415f2390
    Replacement::draft(0x1415f27a0, "ak_1415f27a0", fn_1415f27a0 as *const ()), // sub_1415f27a0
    Replacement::draft(0x1415f2880, "unknown_1415f2880", fn_1415f2880 as *const ()), // sub_1415f2880
    Replacement::draft(0x1415f28a0, "unknown_1415f28a0", fn_1415f28a0 as *const ()), // sub_1415f28a0
    Replacement::draft(0x1415f2900, "unknown_1415f2900", fn_1415f2900 as *const ()), // sub_1415f2900
    Replacement::draft(0x1415f2930, "unknown_1415f2930", fn_1415f2930 as *const ()), // sub_1415f2930
    Replacement::draft(0x1415f29c0, "unknown_1415f29c0", fn_1415f29c0 as *const ()), // sub_1415f29c0
    Replacement::draft(0x1415f2b00, "unknown_1415f2b00", fn_1415f2b00 as *const ()), // sub_1415f2b00
    Replacement::draft(0x1415f2bb0, "unknown_1415f2bb0", fn_1415f2bb0 as *const ()), // sub_1415f2bb0
    Replacement::draft(0x1415f2d10, "unknown_1415f2d10", fn_1415f2d10 as *const ()), // sub_1415f2d10
    Replacement::draft(0x1415f2db0, "unknown_1415f2db0", fn_1415f2db0 as *const ()), // sub_1415f2db0
    Replacement::draft(0x1415f2e80, "unknown_1415f2e80", fn_1415f2e80 as *const ()), // sub_1415f2e80
    Replacement::draft(0x1415f2f20, "unknown_1415f2f20", fn_1415f2f20 as *const ()), // sub_1415f2f20
    Replacement::draft(0x1415f2fc0, "unknown_1415f2fc0", fn_1415f2fc0 as *const ()), // sub_1415f2fc0
    Replacement::draft(0x1415f3060, "unknown_1415f3060", fn_1415f3060 as *const ()), // sub_1415f3060
    Replacement::draft(0x1415f3150, "unknown_1415f3150", fn_1415f3150 as *const ()), // sub_1415f3150
    Replacement::draft(0x1415f31f0, "unknown_1415f31f0", tpl_30142::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const ()), // sub_1415f31f0
    Replacement::draft(0x1415f3220, "unknown_1415f3220", tpl_29129::<0x10, 0x14268dfb8, 0x18, 0x14392d488, 0x14392d480, 0x14392d478, 0x142688f88, 0x141834d30> as *const ()), // sub_1415f3220
    Replacement::draft(0x1415f32b0, "unknown_1415f32b0", tpl_23741::<0x141605000, 0x141834d30> as *const ()), // sub_1415f32b0
    Replacement::draft(0x1415f32f0, "unknown_1415f32f0", fn_1415f32f0 as *const ()), // sub_1415f32f0
    Replacement::draft(0x1415f33b0, "unknown_1415f33b0", fn_1415f33b0 as *const ()), // sub_1415f33b0
    Replacement::draft(0x1415f34d0, "unknown_1415f34d0", fn_1415f34d0 as *const ()), // sub_1415f34d0
    Replacement::draft(0x1415f35b0, "unknown_1415f35b0", fn_1415f35b0 as *const ()), // sub_1415f35b0
    Replacement::draft(0x1415f3640, "ak_1415f3640", fn_1415f3640 as *const ()), // sub_1415f3640
    Replacement::draft(0x1415f3730, "unknown_1415f3730", fn_1415f3730 as *const ()), // sub_1415f3730
    Replacement::draft(0x1415f3790, "ak_1415f3790", fn_1415f3790 as *const ()), // CAkRegistryMgr::CAkRegistryMgr::GetObj
    Replacement::draft(0x1415f37e0, "ak_1415f37e0", fn_1415f37e0 as *const ()), // CAkRegistryMgr::CAkRegistryMgr::GetObjAndAddref
    Replacement::draft(0x1415f3870, "ak_1415f3870", fn_1415f3870 as *const ()), // sub_1415f3870
    Replacement::draft(0x1415f38d0, "ak_1415f38d0", fn_1415f38d0 as *const ()), // sub_1415f38d0
    Replacement::draft(0x1415f39b0, "ak_1415f39b0", fn_1415f39b0 as *const ()), // sub_1415f39b0
    Replacement::draft(0x1415f3a10, "ak_1415f3a10", fn_1415f3a10 as *const ()), // sub_1415f3a10
    Replacement::draft(0x1415f3aa0, "ak_1415f3aa0", fn_1415f3aa0 as *const ()), // sub_1415f3aa0
    Replacement::draft(0x1415f3db0, "ak_1415f3db0", fn_1415f3db0 as *const ()), // sub_1415f3db0
    Replacement::draft(0x1415f3eb0, "ak_1415f3eb0", fn_1415f3eb0 as *const ()), // sub_1415f3eb0
    Replacement::draft(0x1415f3f50, "ak_1415f3f50", tpl_29288::<0x14268df40, 0x141524330, 0x1415242e0> as *const ()), // sub_1415f3f50
    Replacement::draft(0x1415f40d0, "ak_1415f40d0", fn_1415f40d0 as *const ()), // sub_1415f40d0
    Replacement::draft(0x1415f4270, "ak_1415f4270", fn_1415f4270 as *const ()), // sub_1415f4270
    Replacement::draft(0x1415f4330, "ak_1415f4330", fn_1415f4330 as *const ()), // sub_1415f4330
    Replacement::draft(0x1415f4430, "ak_1415f4430", fn_1415f4430 as *const ()), // sub_1415f4430
    Replacement::draft(0x1415f4570, "ak_1415f4570", fn_1415f4570 as *const ()), // sub_1415f4570
    Replacement::draft(0x1415f4690, "ak_1415f4690", fn_1415f4690 as *const ()), // sub_1415f4690
    Replacement::draft(0x1415f4720, "ak_1415f4720", fn_1415f4720 as *const ()), // sub_1415f4720
    Replacement::draft(0x1415f47f0, "ak_1415f47f0", fn_1415f47f0 as *const ()), // sub_1415f47f0
    Replacement::draft(0x1415f48a0, "ak_1415f48a0", fn_1415f48a0 as *const ()), // sub_1415f48a0
    Replacement::draft(0x1415f4970, "ak_1415f4970", fn_1415f4970 as *const ()), // sub_1415f4970
    Replacement::draft(0x1415f4a50, "ak_1415f4a50", fn_1415f4a50 as *const ()), // sub_1415f4a50
    Replacement::draft(0x1415f4ab0, "ak_1415f4ab0", fn_1415f4ab0 as *const ()), // sub_1415f4ab0
    Replacement::draft(0x1415f4cb0, "ak_1415f4cb0", fn_1415f4cb0 as *const ()), // sub_1415f4cb0
    Replacement::draft(0x1415f4d20, "ak_1415f4d20", fn_1415f4d20 as *const ()), // sub_1415f4d20
    Replacement::draft(0x1415f4ef0, "ak_1415f4ef0", fn_1415f4ef0 as *const ()), // sub_1415f4ef0
    Replacement::draft(0x1415f4fa0, "ak_1415f4fa0", fn_1415f4fa0 as *const ()), // sub_1415f4fa0
    Replacement::draft(0x1415f5070, "ak_1415f5070", fn_1415f5070 as *const ()), // sub_1415f5070
    Replacement::draft(0x1415f50f0, "ak_1415f50f0", fn_1415f50f0 as *const ()), // sub_1415f50f0
    Replacement::draft(0x1415f52c0, "ak_1415f52c0", fn_1415f52c0 as *const ()), // sub_1415f52c0
    Replacement::draft(0x1415f5370, "ak_1415f5370", fn_1415f5370 as *const ()), // sub_1415f5370
    Replacement::draft(0x1415f5480, "ak_1415f5480", fn_1415f5480 as *const ()), // sub_1415f5480
    Replacement::draft(0x1415f54d0, "ak_1415f54d0", fn_1415f54d0 as *const ()), // sub_1415f54d0
    Replacement::draft(0x1415f5610, "ak_1415f5610", fn_1415f5610 as *const ()), // sub_1415f5610
    Replacement::draft(0x1415f5720, "ak_1415f5720", fn_1415f5720 as *const ()), // sub_1415f5720
    Replacement::draft(0x1415f5750, "ak_1415f5750", fn_1415f5750 as *const ()), // sub_1415f5750
    Replacement::draft(0x1415f5820, "ak_1415f5820", fn_1415f5820 as *const ()), // sub_1415f5820
    Replacement::draft(0x1415f5890, "ak_1415f5890", fn_1415f5890 as *const ()), // sub_1415f5890
    Replacement::draft(0x1415f5900, "ak_1415f5900", fn_1415f5900 as *const ()), // sub_1415f5900
    Replacement::draft(0x1415f59c0, "ak_1415f59c0", fn_1415f59c0 as *const ()), // sub_1415f59c0
    Replacement::draft(0x1415f5a30, "ak_1415f5a30", fn_1415f5a30 as *const ()), // sub_1415f5a30
    Replacement::draft(0x1415f5a90, "ak_1415f5a90", fn_1415f5a90 as *const ()), // sub_1415f5a90
    Replacement::draft(0x1415f5d60, "ak_1415f5d60", fn_1415f5d60 as *const ()), // sub_1415f5d60
    Replacement::draft(0x1415f5f30, "ak_1415f5f30", fn_1415f5f30 as *const ()), // sub_1415f5f30
    Replacement::draft(0x1415f5f40, "ak_1415f5f40", fn_1415f5f40 as *const ()), // sub_1415f5f40
    Replacement::draft(0x1415f6210, "ak_1415f6210", fn_1415f6210 as *const ()), // sub_1415f6210
    Replacement::draft(0x1415f6230, "ak_1415f6230", fn_1415f6230 as *const ()), // sub_1415f6230
    Replacement::draft(0x1415f64f0, "ak_1415f64f0", fn_1415f64f0 as *const ()), // sub_1415f64f0
    Replacement::draft(0x1415f6530, "ak_1415f6530", fn_1415f6530 as *const ()), // sub_1415f6530
    Replacement::draft(0x1415f6550, "ak_1415f6550", fn_1415f6550 as *const ()), // sub_1415f6550
    Replacement::draft(0x1415f65b0, "ak_1415f65b0", fn_1415f65b0 as *const ()), // sub_1415f65b0
    Replacement::draft(0x1415f66e0, "ak_1415f66e0", fn_1415f66e0 as *const ()), // sub_1415f66e0
    Replacement::draft(0x1415f6ad0, "ak_1415f6ad0", fn_1415f6ad0 as *const ()), // sub_1415f6ad0
    Replacement::draft(0x1415f6d80, "ak_1415f6d80", fn_1415f6d80 as *const ()), // sub_1415f6d80
    Replacement::draft(0x1415f6e00, "ak_1415f6e00", fn_1415f6e00 as *const ()), // sub_1415f6e00
    Replacement::draft(0x1415f6e60, "ak_1415f6e60", fn_1415f6e60 as *const ()), // sub_1415f6e60
    Replacement::draft(0x1415f6e90, "ak_1415f6e90", tpl_29344::<0x30, 0x1, 0x4> as *const ()), // sub_1415f6e90
    Replacement::draft(0x1415f6ed0, "ak_1415f6ed0", fn_1415f6ed0 as *const ()), // sub_1415f6ed0
    Replacement::draft(0x1415f6ee0, "ak_1415f6ee0", fn_1415f6ee0 as *const ()), // sub_1415f6ee0
    Replacement::draft(0x1415f7050, "ak_1415f7050", fn_1415f7050 as *const ()), // sub_1415f7050
    Replacement::draft(0x1415f7260, "ak_1415f7260", fn_1415f7260 as *const ()), // sub_1415f7260
    Replacement::draft(0x1415f73d0, "ak_1415f73d0", fn_1415f73d0 as *const ()), // sub_1415f73d0
    Replacement::draft(0x1415f7410, "ak_1415f7410", fn_1415f7410 as *const ()), // sub_1415f7410
    Replacement::draft(0x1415f7480, "ak_1415f7480", fn_1415f7480 as *const ()), // sub_1415f7480
    Replacement::draft(0x1415f7540, "ak_1415f7540", fn_1415f7540 as *const ()), // sub_1415f7540
    Replacement::draft(0x1415f75e0, "ak_1415f75e0", fn_1415f75e0 as *const ()), // sub_1415f75e0
    Replacement::draft(0x1415f7660, "ak_1415f7660", fn_1415f7660 as *const ()), // sub_1415f7660
    Replacement::draft(0x1415f76e0, "ak_1415f76e0", fn_1415f76e0 as *const ()), // sub_1415f76e0
    Replacement::draft(0x1415f78a0, "ak_1415f78a0", fn_1415f78a0 as *const ()), // sub_1415f78a0
    Replacement::draft(0x1415f7b20, "ak_1415f7b20", fn_1415f7b20 as *const ()), // sub_1415f7b20
    Replacement::draft(0x1415f7d90, "ak_1415f7d90", fn_1415f7d90 as *const ()), // sub_1415f7d90
    Replacement::draft(0x1415f7e10, "ak_1415f7e10", fn_1415f7e10 as *const ()), // sub_1415f7e10
    Replacement::draft(0x1415f7ea0, "ak_1415f7ea0", fn_1415f7ea0 as *const ()), // sub_1415f7ea0
    Replacement::draft(0x1415f8000, "ak_1415f8000", fn_1415f8000 as *const ()), // sub_1415f8000
    Replacement::draft(0x1415f8060, "ak_1415f8060", fn_1415f8060 as *const ()), // sub_1415f8060
    Replacement::draft(0x1415f80b0, "ak_1415f80b0", fn_1415f80b0 as *const ()), // sub_1415f80b0
    Replacement::draft(0x1415f80f0, "ak_1415f80f0", fn_1415f80f0 as *const ()), // sub_1415f80f0
    Replacement::draft(0x1415f8150, "ak_1415f8150", fn_1415f8150 as *const ()), // sub_1415f8150
    Replacement::draft(0x1415f81d0, "ak_1415f81d0", fn_1415f81d0 as *const ()), // sub_1415f81d0
    Replacement::draft(0x1415f8290, "ak_1415f8290", fn_1415f8290 as *const ()), // sub_1415f8290
    Replacement::draft(0x1415f8370, "ak_1415f8370", fn_1415f8370 as *const ()), // sub_1415f8370
    Replacement::draft(0x1415f83b0, "ak_1415f83b0", fn_1415f83b0 as *const ()), // sub_1415f83b0
    Replacement::draft(0x1415f83f0, "ak_1415f83f0", fn_1415f83f0 as *const ()), // sub_1415f83f0
    Replacement::draft(0x1415f84c0, "ak_1415f84c0", fn_1415f84c0 as *const ()), // sub_1415f84c0
    Replacement::draft(0x1415f8550, "ak_1415f8550", fn_1415f8550 as *const ()), // sub_1415f8550
    Replacement::draft(0x1415f87a0, "ak_1415f87a0", fn_1415f87a0 as *const ()), // sub_1415f87a0
    Replacement::draft(0x1415f89a0, "ak_1415f89a0", tpl_27091::<0x14397d1e0, 0x143931c80, 0x143815af8, 0x141834a78, 0x1426443b0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1415f89a0
    Replacement::draft(0x1415f8a10, "ak_1415f8a10", fn_1415f8a10 as *const ()), // sub_1415f8a10
    Replacement::draft(0x1415f8a60, "ak_1415f8a60", fn_1415f8a60 as *const ()), // sub_1415f8a60
    Replacement::draft(0x1415f8bb0, "ak_1415f8bb0", fn_1415f8bb0 as *const ()), // sub_1415f8bb0
    Replacement::draft(0x1415f8c80, "ak_1415f8c80", fn_1415f8c80 as *const ()), // sub_1415f8c80
    Replacement::draft(0x1415f8cf0, "ak_1415f8cf0", fn_1415f8cf0 as *const ()), // sub_1415f8cf0
    Replacement::draft(0x1415f8d40, "ak_1415f8d40", fn_1415f8d40 as *const ()), // sub_1415f8d40
    Replacement::draft(0x1415f8db0, "ak_1415f8db0", tpl_30209::<0x141613860> as *const ()), // sub_1415f8db0
    Replacement::draft(0x1415f8dc0, "ak_1415f8dc0", fn_1415f8dc0 as *const ()), // sub_1415f8dc0
    Replacement::draft_throws(0x1415f8f00, "ak_1415f8f00", fn_1415f8f00 as *const (), Throws::Discard), // sub_1415f8f00
    Replacement::draft(0x1415f90b0, "ak_1415f90b0", fn_1415f90b0 as *const ()), // sub_1415f90b0
    Replacement::draft(0x1415f90e0, "ak_1415f90e0", tpl_365::<0x1415b7630> as *const ()), // sub_1415f90e0
    Replacement::draft(0x1415f90f0, "ak_1415f90f0", fn_1415f90f0 as *const ()), // sub_1415f90f0
    Replacement::draft(0x1415f9310, "ak_1415f9310", fn_1415f9310 as *const ()), // sub_1415f9310
    Replacement::draft(0x1415f9380, "ak_1415f9380", fn_1415f9380 as *const ()), // sub_1415f9380
    Replacement::draft(0x1415f9520, "ak_1415f9520", fn_1415f9520 as *const ()), // sub_1415f9520
    Replacement::draft(0x1415f9590, "ak_1415f9590", fn_1415f9590 as *const ()), // sub_1415f9590
    Replacement::draft(0x1415f9624, "ak_1415f9624", tpl_28392::<0x1415f9640> as *const ()), // sub_1415f9624
    Replacement::draft(0x1415f9630, "ak_1415f9630", tpl_28393::<0x1415f9640> as *const ()), // sub_1415f9630
    Replacement::draft(0x1415f9640, "ak_1415f9640", fn_1415f9640 as *const ()), // sub_1415f9640
    Replacement::draft(0x1415f9720, "ak_1415f9720", fn_1415f9720 as *const ()), // sub_1415f9720
    Replacement::draft(0x1415f98b0, "ak_1415f98b0", fn_1415f98b0 as *const ()), // sub_1415f98b0
    Replacement::draft(0x1415f9930, "ak_1415f9930", fn_1415f9930 as *const ()), // sub_1415f9930
    Replacement::draft(0x1415f9b00, "ak_1415f9b00", fn_1415f9b00 as *const ()), // sub_1415f9b00
    Replacement::draft(0x1415f9b80, "ak_1415f9b80", fn_1415f9b80 as *const ()), // sub_1415f9b80
    Replacement::draft(0x1415f9c10, "ak_1415f9c10", fn_1415f9c10 as *const ()), // sub_1415f9c10
    Replacement::draft(0x1415f9c40, "ak_1415f9c40", fn_1415f9c40 as *const ()), // sub_1415f9c40
    Replacement::draft(0x1415f9e60, "ak_1415f9e60", fn_1415f9e60 as *const ()), // sub_1415f9e60
    Replacement::draft(0x1415f9ee0, "ak_1415f9ee0", fn_1415f9ee0 as *const ()), // sub_1415f9ee0
    Replacement::draft(0x1415fa0e0, "ak_1415fa0e0", fn_1415fa0e0 as *const ()), // sub_1415fa0e0
    Replacement::draft(0x1415fa180, "ak_1415fa180", fn_1415fa180 as *const ()), // sub_1415fa180
    Replacement::draft(0x1415fa260, "ak_1415fa260", fn_1415fa260 as *const ()), // sub_1415fa260
    Replacement::draft(0x1415fa360, "ak_1415fa360", fn_1415fa360 as *const ()), // sub_1415fa360
    Replacement::draft(0x1415fa780, "ak_1415fa780", fn_1415fa780 as *const ()), // sub_1415fa780
    Replacement::draft(0x1415fab60, "ak_1415fab60", tpl_28648::<0x108, 0x18> as *const ()), // sub_1415fab60
    Replacement::draft(0x1415fab80, "ak_1415fab80", fn_1415fab80 as *const ()), // sub_1415fab80
    Replacement::draft(0x1415fac80, "ak_1415fac80", fn_1415fac80 as *const ()), // sub_1415fac80
    Replacement::draft(0x1415faca0, "ak_1415faca0", fn_1415faca0 as *const ()), // sub_1415faca0
    Replacement::draft(0x1415fad70, "ak_1415fad70", fn_1415fad70 as *const ()), // sub_1415fad70
    Replacement::draft(0x1415fb0e0, "ak_1415fb0e0", fn_1415fb0e0 as *const ()), // sub_1415fb0e0
    Replacement::draft(0x1415fb140, "ak_1415fb140", fn_1415fb140 as *const ()), // sub_1415fb140
    Replacement::draft(0x1415fb340, "ak_1415fb340", fn_1415fb340 as *const ()), // sub_1415fb340
    Replacement::draft(0x1415fb3a0, "ak_1415fb3a0", fn_1415fb3a0 as *const ()), // sub_1415fb3a0
    Replacement::draft(0x1415fb720, "ak_1415fb720", fn_1415fb720 as *const ()), // sub_1415fb720
    Replacement::draft(0x1415fb810, "ak_1415fb810", fn_1415fb810 as *const ()), // sub_1415fb810
    Replacement::draft(0x1415fb990, "ak_1415fb990", fn_1415fb990 as *const ()), // sub_1415fb990
    Replacement::draft(0x1415fba90, "ak_1415fba90", fn_1415fba90 as *const ()), // sub_1415fba90
    Replacement::draft(0x1415fbeb0, "ak_1415fbeb0", fn_1415fbeb0 as *const ()), // sub_1415fbeb0
    Replacement::draft(0x1415fbfb0, "ak_1415fbfb0", fn_1415fbfb0 as *const ()), // sub_1415fbfb0
    Replacement::draft(0x1415fbfc0, "ak_1415fbfc0", fn_1415fbfc0 as *const ()), // sub_1415fbfc0
    Replacement::draft(0x1415fc110, "ak_1415fc110", fn_1415fc110 as *const ()), // sub_1415fc110
    Replacement::draft(0x1415fc1a0, "ak_1415fc1a0", fn_1415fc1a0 as *const ()), // sub_1415fc1a0
    Replacement::draft(0x1415fc1f0, "ak_1415fc1f0", fn_1415fc1f0 as *const ()), // sub_1415fc1f0
    Replacement::draft(0x1415fc320, "ak_1415fc320", fn_1415fc320 as *const ()), // sub_1415fc320
    Replacement::draft(0x1415fc410, "ak_1415fc410", fn_1415fc410 as *const ()), // sub_1415fc410
    Replacement::draft(0x1415fc980, "ak_1415fc980", tpl_26845::<0x1415bd160, 0x141834d30> as *const ()), // sub_1415fc980
    Replacement::draft(0x1415fc9c0, "ak_1415fc9c0", fn_1415fc9c0 as *const ()), // sub_1415fc9c0
    Replacement::draft(0x1415fcc70, "ak_1415fcc70", fn_1415fcc70 as *const ()), // sub_1415fcc70
    Replacement::draft(0x1415fcd90, "ak_1415fcd90", fn_1415fcd90 as *const ()), // sub_1415fcd90
    Replacement::draft(0x1415fcda0, "ak_1415fcda0", fn_1415fcda0 as *const ()), // sub_1415fcda0
    Replacement::draft(0x1415fce00, "ak_1415fce00", fn_1415fce00 as *const ()), // sub_1415fce00
    Replacement::draft(0x1415fce10, "ak_1415fce10", fn_1415fce10 as *const ()), // sub_1415fce10
    Replacement::draft(0x1415fcfa0, "ak_1415fcfa0", fn_1415fcfa0 as *const ()), // sub_1415fcfa0
    Replacement::draft(0x1415fd100, "ak_1415fd100", fn_1415fd100 as *const ()), // sub_1415fd100
    Replacement::draft(0x1415fd170, "ak_1415fd170", fn_1415fd170 as *const ()), // sub_1415fd170
    Replacement::draft(0x1415fd2c0, "ak_1415fd2c0", fn_1415fd2c0 as *const ()), // sub_1415fd2c0
    Replacement::draft(0x1415fd450, "ak_1415fd450", fn_1415fd450 as *const ()), // sub_1415fd450
    Replacement::draft(0x1415fd9f0, "ak_1415fd9f0", fn_1415fd9f0 as *const ()), // sub_1415fd9f0
    Replacement::draft(0x1415fdba0, "ak_1415fdba0", fn_1415fdba0 as *const ()), // sub_1415fdba0
    Replacement::draft(0x1415fdc10, "ak_1415fdc10", fn_1415fdc10 as *const ()), // sub_1415fdc10
    Replacement::draft(0x1415fde80, "ak_1415fde80", fn_1415fde80 as *const ()), // sub_1415fde80
    Replacement::draft(0x1415fe0c0, "ak_1415fe0c0", fn_1415fe0c0 as *const ()), // sub_1415fe0c0
    Replacement::draft(0x1415fe280, "ak_1415fe280", fn_1415fe280 as *const ()), // sub_1415fe280
    Replacement::draft(0x1415fe5d0, "ak_1415fe5d0", fn_1415fe5d0 as *const ()), // sub_1415fe5d0
    Replacement::draft(0x1415fe6c0, "ak_1415fe6c0", fn_1415fe6c0 as *const ()), // sub_1415fe6c0
    Replacement::draft(0x1415fe7c0, "ak_1415fe7c0", fn_1415fe7c0 as *const ()), // sub_1415fe7c0
    Replacement::draft(0x1415fe850, "ak_1415fe850", fn_1415fe850 as *const ()), // sub_1415fe850
    Replacement::draft(0x1415fe900, "ak_1415fe900", fn_1415fe900 as *const ()), // sub_1415fe900
    Replacement::draft(0x1415fea00, "ak_1415fea00", fn_1415fea00 as *const ()), // sub_1415fea00
    Replacement::draft(0x1415feb10, "ak_1415feb10", fn_1415feb10 as *const ()), // sub_1415feb10
    Replacement::draft(0x1415fece0, "ak_1415fece0", fn_1415fece0 as *const ()), // sub_1415fece0
    Replacement::draft(0x1415fedb0, "ak_1415fedb0", tpl_30254::<0x14163cb20> as *const ()), // sub_1415fedb0
    Replacement::draft(0x1415fedd0, "ak_1415fedd0", tpl_30254::<0x14163cb30> as *const ()), // sub_1415fedd0
    Replacement::draft(0x1415fedf0, "ak_1415fedf0", fn_1415fedf0 as *const ()), // sub_1415fedf0
    Replacement::draft(0x1415fefd0, "ak_1415fefd0", fn_1415fefd0 as *const ()), // sub_1415fefd0
    Replacement::draft(0x1415ff000, "ak_1415ff000", fn_1415ff000 as *const ()), // sub_1415ff000
    Replacement::draft(0x1415ff070, "ak_1415ff070", fn_1415ff070 as *const ()), // sub_1415ff070
    Replacement::draft(0x1415ff0b0, "ak_1415ff0b0", fn_1415ff0b0 as *const ()), // sub_1415ff0b0
    Replacement::draft(0x1415ff1e0, "ak_1415ff1e0", fn_1415ff1e0 as *const ()), // sub_1415ff1e0
    Replacement::draft(0x1415ff250, "ak_1415ff250", fn_1415ff250 as *const ()), // sub_1415ff250
    Replacement::draft(0x1415ff260, "ak_1415ff260", tpl_27091::<0x14397d1e0, 0x143931d10, 0x143815b30, 0x141834a78, 0x1426443e0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1415ff260
    Replacement::draft(0x1415ff2d0, "ak_1415ff2d0", tpl_27091::<0x14397d1e0, 0x143931d08, 0x143815b20, 0x141834a78, 0x1426443f0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1415ff2d0
    Replacement::draft(0x1415ff340, "ak_1415ff340", tpl_27091::<0x14397d1e0, 0x143931d0c, 0x143815b28, 0x141834a78, 0x142644400, 0x141834d18, 0x141834a0c> as *const ()), // sub_1415ff340
    Replacement::draft(0x1415ff3b0, "ak_1415ff3b0", tpl_27091::<0x14397d1e0, 0x143931d00, 0x143815b10, 0x141834a78, 0x142644410, 0x141834d18, 0x141834a0c> as *const ()), // sub_1415ff3b0
    Replacement::draft(0x1415ff420, "ak_1415ff420", tpl_27091::<0x14397d1e0, 0x143931d04, 0x143815b18, 0x141834a78, 0x142644420, 0x141834d18, 0x141834a0c> as *const ()), // sub_1415ff420
    Replacement::draft(0x1415ff490, "unknown_1415ff490", fn_1415ff490 as *const ()), // sub_1415ff490
    Replacement::draft(0x1415ff4a0, "unknown_1415ff4a0", fn_1415ff4a0 as *const ()), // sub_1415ff4a0
    Replacement::draft(0x1415ff4d0, "unknown_1415ff4d0", fn_1415ff4d0 as *const ()), // sub_1415ff4d0
    Replacement::draft(0x1415ff4e0, "unknown_1415ff4e0", fn_1415ff4e0 as *const ()), // sub_1415ff4e0
    Replacement::draft(0x1415ff590, "unknown_1415ff590", fn_1415ff590 as *const ()), // sub_1415ff590
    Replacement::draft(0x1415ff5d0, "unknown_1415ff5d0", fn_1415ff5d0 as *const ()), // sub_1415ff5d0
    Replacement::draft(0x1415ff640, "unknown_1415ff640", fn_1415ff640 as *const ()), // sub_1415ff640
    Replacement::draft(0x1415ff660, "unknown_1415ff660", fn_1415ff660 as *const ()), // sub_1415ff660
    Replacement::draft(0x1415ff690, "unknown_1415ff690", fn_1415ff690 as *const ()), // sub_1415ff690
    Replacement::draft(0x1415ff740, "unknown_1415ff740", fn_1415ff740 as *const ()), // sub_1415ff740
    Replacement::draft(0x1415ff810, "unknown_1415ff810", fn_1415ff810 as *const ()), // sub_1415ff810
    Replacement::draft(0x1415ff978, "unknown_1415ff978", tpl_29183::<0x1415ffb60> as *const ()), // sub_1415ff978
    Replacement::draft(0x1415ff990, "opentelemetry_1415ff990", tpl_26965::<0x1416440f0, 0x141834d30> as *const ()), // sub_1415ff990
    Replacement::draft(0x1415ffa20, "unknown_1415ffa20", fn_1415ffa20 as *const ()), // sub_1415ffa20
    Replacement::draft(0x1415ffa70, "unknown_1415ffa70", fn_1415ffa70 as *const ()), // sub_1415ffa70
    Replacement::draft(0x1415ffac0, "unknown_1415ffac0", fn_1415ffac0 as *const ()), // sub_1415ffac0
    Replacement::draft(0x1415ffb60, "unknown_1415ffb60", fn_1415ffb60 as *const ()), // sub_1415ffb60
    Replacement::draft(0x1415ffbe0, "unknown_1415ffbe0", tpl_495::<0x141643b60, 0x141834d30> as *const ()), // sub_1415ffbe0
    Replacement::draft(0x1415ffc20, "unknown_1415ffc20", tpl_893::<0x14268e6d0, 0x141834d30> as *const ()), // sub_1415ffc20
    Replacement::draft(0x1415ffc50, "unknown_1415ffc50", tpl_25002::<0x14158fa60, 0x141834d30> as *const ()), // sub_1415ffc50
    Replacement::draft(0x1415ffc90, "unknown_1415ffc90", fn_1415ffc90 as *const ()), // sub_1415ffc90
    Replacement::draft(0x141600270, "unknown_141600270", fn_141600270 as *const ()), // sub_141600270
    Replacement::draft(0x1416003b0, "unknown_1416003b0", fn_1416003b0 as *const ()), // sub_1416003b0
    Replacement::draft(0x1416006a0, "unknown_1416006a0", fn_1416006a0 as *const ()), // sub_1416006a0
    Replacement::draft(0x141600720, "unknown_141600720", fn_141600720 as *const ()), // sub_141600720
    Replacement::draft(0x141600860, "unknown_141600860", fn_141600860 as *const ()), // sub_141600860
    Replacement::draft(0x141600a30, "unknown_141600a30", fn_141600a30 as *const ()), // sub_141600a30
    Replacement::draft(0x141600ad0, "unknown_141600ad0", fn_141600ad0 as *const ()), // sub_141600ad0
    Replacement::draft(0x141600b70, "unknown_141600b70", fn_141600b70 as *const ()), // sub_141600b70
    Replacement::draft(0x141600c40, "unknown_141600c40", fn_141600c40 as *const ()), // sub_141600c40
    Replacement::draft(0x141600c70, "unknown_141600c70", fn_141600c70 as *const ()), // sub_141600c70
    Replacement::draft(0x141600ea0, "unknown_141600ea0", tpl_27317::<0x8, 0x141590fd0> as *const ()), // sub_141600ea0
    Replacement::draft(0x141600eb0, "unknown_141600eb0", tpl_27317::<0x8, 0x141591060> as *const ()), // sub_141600eb0
    Replacement::draft(0x141600ec0, "unknown_141600ec0", fn_141600ec0 as *const ()), // sub_141600ec0
    Replacement::draft(0x141600ed0, "unknown_141600ed0", fn_141600ed0 as *const ()), // sub_141600ed0
    Replacement::draft(0x141600ef0, "unknown_141600ef0", tpl_27317::<0x8, 0x141591100> as *const ()), // sub_141600ef0
    Replacement::draft(0x141600f00, "unknown_141600f00", fn_141600f00 as *const ()), // sub_141600f00
    Replacement::draft(0x141600f20, "unknown_141600f20", tpl_27317::<0x8, 0x141591230> as *const ()), // sub_141600f20
    Replacement::draft(0x141600f30, "unknown_141600f30", fn_141600f30 as *const ()), // sub_141600f30
    Replacement::draft(0x141600fb0, "unknown_141600fb0", fn_141600fb0 as *const ()), // sub_141600fb0
    Replacement::draft(0x141600fc0, "unknown_141600fc0", fn_141600fc0 as *const ()), // sub_141600fc0
    Replacement::draft(0x141600fe0, "unknown_141600fe0", fn_141600fe0 as *const ()), // sub_141600fe0
    Replacement::draft(0x141601030, "unknown_141601030", fn_141601030 as *const ()), // sub_141601030
    Replacement::draft(0x141601050, "unknown_141601050", tpl_913::<0xd0> as *const ()), // sub_141601050
    Replacement::draft(0x141601060, "unknown_141601060", fn_141601060 as *const ()), // sub_141601060
    Replacement::draft(0x141601080, "unknown_141601080", fn_141601080 as *const ()), // sub_141601080
    Replacement::draft(0x1416010f0, "unknown_1416010f0", fn_1416010f0 as *const ()), // sub_1416010f0
    Replacement::draft(0x141601180, "unknown_141601180", fn_141601180 as *const ()), // sub_141601180
    Replacement::draft(0x141601240, "unknown_141601240", fn_141601240 as *const ()), // sub_141601240
    Replacement::draft(0x141601300, "unknown_141601300", fn_141601300 as *const ()), // sub_141601300
    Replacement::draft(0x141601360, "unknown_141601360", fn_141601360 as *const ()), // sub_141601360
    Replacement::draft(0x141601420, "unknown_141601420", fn_141601420 as *const ()), // sub_141601420
    Replacement::draft(0x141601450, "unknown_141601450", fn_141601450 as *const ()), // sub_141601450
    Replacement::draft(0x141601470, "unknown_141601470", fn_141601470 as *const ()), // sub_141601470
    Replacement::draft(0x1416014e0, "ak_1416014e0", fn_1416014e0 as *const ()), // sub_1416014e0
    Replacement::draft(0x1416015e0, "ak_1416015e0", fn_1416015e0 as *const ()), // sub_1416015e0
    Replacement::draft(0x1416015f0, "ak_1416015f0", fn_1416015f0 as *const ()), // sub_1416015f0
    Replacement::draft(0x141601670, "ak_141601670", fn_141601670 as *const ()), // sub_141601670
    Replacement::draft(0x141601720, "ak_141601720", fn_141601720 as *const ()), // sub_141601720
    Replacement::draft(0x1416017b0, "ak_1416017b0", fn_1416017b0 as *const ()), // sub_1416017b0
    Replacement::draft(0x141601820, "ak_141601820", fn_141601820 as *const ()), // sub_141601820
    Replacement::draft(0x141601880, "ak_141601880", fn_141601880 as *const ()), // sub_141601880
    Replacement::draft(0x1416026f0, "ak_1416026f0", fn_1416026f0 as *const ()), // sub_1416026f0
    Replacement::draft(0x141602720, "ak_141602720", fn_141602720 as *const ()), // sub_141602720
    Replacement::draft(0x1416027d0, "ak_1416027d0", fn_1416027d0 as *const ()), // sub_1416027d0
    Replacement::draft(0x1416029d0, "ak_1416029d0", fn_1416029d0 as *const ()), // sub_1416029d0
    Replacement::draft(0x141602a80, "ak_141602a80", fn_141602a80 as *const ()), // sub_141602a80
    Replacement::draft(0x141602c00, "ak_141602c00", fn_141602c00 as *const ()), // sub_141602c00
    Replacement::draft(0x141602c20, "ak_141602c20", fn_141602c20 as *const ()), // sub_141602c20
    Replacement::draft(0x141603110, "ak_141603110", fn_141603110 as *const ()), // sub_141603110
    Replacement::draft(0x1416031f0, "ak_1416031f0", fn_1416031f0 as *const ()), // sub_1416031f0
    Replacement::draft(0x141603250, "ak_141603250", tpl_30308::<0x68> as *const ()), // sub_141603250
    Replacement::draft(0x141603260, "ak_141603260", fn_141603260 as *const ()), // sub_141603260
    Replacement::draft(0x141603270, "ak_141603270", tpl_29781::<0x8, 0x141524320> as *const ()), // sub_141603270
    Replacement::draft(0x141603280, "ak_141603280", fn_141603280 as *const ()), // sub_141603280
    Replacement::draft(0x141603290, "ak_141603290", tpl_29781::<0x8, 0x141524350> as *const ()), // sub_141603290
    Replacement::draft(0x1416032a0, "ak_1416032a0", tpl_29781::<0x8, 0x1415242e0> as *const ()), // sub_1416032a0
    Replacement::draft(0x1416032b0, "ak_1416032b0", fn_1416032b0 as *const ()), // sub_1416032b0
    Replacement::draft(0x141603420, "ak_141603420", fn_141603420 as *const ()), // sub_141603420
    Replacement::draft(0x1416034e0, "ak_1416034e0", fn_1416034e0 as *const ()), // sub_1416034e0
    Replacement::draft(0x141603630, "ak_141603630", fn_141603630 as *const ()), // sub_141603630
    Replacement::draft(0x141603710, "ak_141603710", fn_141603710 as *const ()), // sub_141603710
    Replacement::draft(0x141603800, "ak_141603800", fn_141603800 as *const ()), // sub_141603800
    Replacement::draft(0x1416038d0, "ak_1416038d0", fn_1416038d0 as *const ()), // sub_1416038d0
    Replacement::draft(0x141603910, "ak_141603910", fn_141603910 as *const ()), // sub_141603910
    Replacement::draft(0x141603ac0, "ak_141603ac0", fn_141603ac0 as *const ()), // sub_141603ac0
    Replacement::draft(0x141603d50, "ak_141603d50", fn_141603d50 as *const ()), // sub_141603d50
    Replacement::draft(0x141603dd0, "ak_141603dd0", fn_141603dd0 as *const ()), // sub_141603dd0
    Replacement::draft(0x141603fb0, "ak_141603fb0", fn_141603fb0 as *const ()), // sub_141603fb0
    Replacement::draft(0x141603fd0, "ak_141603fd0", fn_141603fd0 as *const ()), // sub_141603fd0
    Replacement::draft(0x141604140, "ak_141604140", fn_141604140 as *const ()), // sub_141604140
    Replacement::draft(0x1416042c0, "ak_1416042c0", fn_1416042c0 as *const ()), // sub_1416042c0
    Replacement::draft(0x141604370, "ak_141604370", fn_141604370 as *const ()), // sub_141604370
    Replacement::draft(0x1416043c0, "ak_1416043c0", fn_1416043c0 as *const ()), // sub_1416043c0
    Replacement::draft(0x1416044a0, "ak_1416044a0", fn_1416044a0 as *const ()), // sub_1416044a0
    Replacement::draft(0x1416046a0, "ak_1416046a0", fn_1416046a0 as *const ()), // sub_1416046a0
    Replacement::draft(0x1416046e0, "ak_1416046e0", fn_1416046e0 as *const ()), // sub_1416046e0
    Replacement::draft(0x141604890, "ak_141604890", fn_141604890 as *const ()), // sub_141604890
    Replacement::draft(0x141604950, "ak_141604950", fn_141604950 as *const ()), // sub_141604950
    Replacement::draft(0x1416049d0, "ak_1416049d0", fn_1416049d0 as *const ()), // sub_1416049d0
    Replacement::draft(0x141604a30, "ak_141604a30", fn_141604a30 as *const ()), // sub_141604a30
    Replacement::draft(0x141604ac0, "ak_141604ac0", fn_141604ac0 as *const ()), // sub_141604ac0
    Replacement::draft(0x141604b30, "ak_141604b30", fn_141604b30 as *const ()), // sub_141604b30
    Replacement::draft(0x141604bd0, "ak_141604bd0", fn_141604bd0 as *const ()), // CAkEmitter::CAkEmitter::~CAkEmitter
    Replacement::draft(0x141604cb0, "unknown_141604cb0", fn_141604cb0 as *const ()), // sub_141604cb0
    Replacement::draft(0x141605000, "unknown_141605000", fn_141605000 as *const ()), // sub_141605000
    Replacement::draft(0x141605080, "unknown_141605080", fn_141605080 as *const ()), // sub_141605080
    Replacement::draft(0x141605140, "unknown_141605140", tpl_29129::<0x10, 0x14268ebe8, 0x18, 0x14392d4a0, 0x14392d498, 0x14392d490, 0x142688f88, 0x141834d30> as *const ()), // sub_141605140
    Replacement::draft(0x1416051d0, "unknown_1416051d0", tpl_29129::<0x10, 0x14268ec08, 0x18, 0x14392d4b8, 0x14392d4b0, 0x14392d4a8, 0x142688f88, 0x141834d30> as *const ()), // sub_1416051d0
    Replacement::draft(0x141605260, "opentelemetry_141605260", tpl_27533::<0x141604cb0, 0x141834d30> as *const ()), // sub_141605260
    Replacement::draft(0x1416052a0, "unknown_1416052a0", fn_1416052a0 as *const ()), // sub_1416052a0
    Replacement::draft(0x141605340, "unknown_141605340", fn_141605340 as *const ()), // sub_141605340
    Replacement::draft(0x141605350, "unknown_141605350", fn_141605350 as *const ()), // sub_141605350
    Replacement::draft(0x141605370, "unknown_141605370", fn_141605370 as *const ()), // sub_141605370
    Replacement::draft(0x1416053b0, "unknown_1416053b0", fn_1416053b0 as *const ()), // sub_1416053b0
    Replacement::draft(0x141605490, "unknown_141605490", fn_141605490 as *const ()), // sub_141605490
    Replacement::draft(0x141605660, "unknown_141605660", fn_141605660 as *const ()), // sub_141605660
    Replacement::draft(0x1416057b0, "unknown_1416057b0", fn_1416057b0 as *const ()), // sub_1416057b0
    Replacement::draft(0x141605870, "unknown_141605870", fn_141605870 as *const ()), // sub_141605870
    Replacement::draft(0x141605890, "unknown_141605890", fn_141605890 as *const ()), // sub_141605890
    Replacement::draft(0x1416058a0, "unknown_1416058a0", fn_1416058a0 as *const ()), // sub_1416058a0
    Replacement::draft(0x141605960, "unknown_141605960", fn_141605960 as *const ()), // sub_141605960
    Replacement::draft(0x1416059a0, "unknown_1416059a0", fn_1416059a0 as *const ()), // sub_1416059a0
    Replacement::draft(0x141605b80, "unknown_141605b80", fn_141605b80 as *const ()), // sub_141605b80
    Replacement::draft(0x141605cb0, "unknown_141605cb0", fn_141605cb0 as *const ()), // sub_141605cb0
    Replacement::draft(0x141605d20, "unknown_141605d20", fn_141605d20 as *const ()), // sub_141605d20
    Replacement::draft(0x141605d50, "unknown_141605d50", fn_141605d50 as *const ()), // sub_141605d50
    Replacement::draft(0x141605de0, "ak_141605de0", fn_141605de0 as *const ()), // sub_141605de0
    Replacement::draft(0x141605f50, "ak_141605f50", fn_141605f50 as *const ()), // sub_141605f50
    Replacement::draft(0x141605fa0, "ak_141605fa0", fn_141605fa0 as *const ()), // sub_141605fa0
    Replacement::draft(0x141606080, "ak_141606080", fn_141606080 as *const ()), // sub_141606080
    Replacement::draft(0x1416061d0, "ak_1416061d0", fn_1416061d0 as *const ()), // sub_1416061d0
    Replacement::draft(0x1416061e0, "ak_1416061e0", fn_1416061e0 as *const ()), // sub_1416061e0
    Replacement::draft(0x141606280, "ak_141606280", fn_141606280 as *const ()), // sub_141606280
    Replacement::draft(0x1416063a0, "ak_1416063a0", tpl_22865::<0x142689028, 0x141834d30> as *const ()), // sub_1416063a0
    Replacement::draft(0x1416063d0, "ak_1416063d0", tpl_21550::<0x8> as *const ()), // sub_1416063d0
    Replacement::draft(0x1416063e0, "ak_1416063e0", tpl_1691::<0xc> as *const ()), // sub_1416063e0
    Replacement::draft(0x1416063f0, "ak_1416063f0", tpl_30345::<0x8> as *const ()), // sub_1416063f0
    Replacement::draft(0x141606400, "ak_141606400", fn_141606400 as *const ()), // sub_141606400
    Replacement::draft(0x1416064f0, "ak_1416064f0", fn_1416064f0 as *const ()), // AK::AkRuntimeEnvironmentMgr::AK::AkRuntimeEnvironmentMgr::Instance
    Replacement::draft(0x141606570, "unknown_141606570", fn_141606570 as *const ()), // sub_141606570
    Replacement::draft(0x141606590, "unknown_141606590", tpl_6019::<0x142679628, 0x141834d30> as *const ()), // sub_141606590
    Replacement::draft(0x1416065c0, "unknown_1416065c0", fn_1416065c0 as *const ()), // sub_1416065c0
    Replacement::draft(0x1416065e0, "unknown_1416065e0", fn_1416065e0 as *const ()), // sub_1416065e0
    Replacement::draft(0x141606760, "unknown_141606760", fn_141606760 as *const ()), // sub_141606760
    Replacement::draft(0x141606870, "unknown_141606870", fn_141606870 as *const ()), // sub_141606870
    Replacement::draft(0x1416068c0, "unknown_1416068c0", fn_1416068c0 as *const ()), // sub_1416068c0
    Replacement::draft(0x141606910, "unknown_141606910", fn_141606910 as *const ()), // sub_141606910
    Replacement::draft(0x141606940, "unknown_141606940", fn_141606940 as *const ()), // sub_141606940
    Replacement::draft(0x141606990, "unknown_141606990", fn_141606990 as *const ()), // sub_141606990
    Replacement::draft(0x1416069f0, "unknown_1416069f0", fn_1416069f0 as *const ()), // sub_1416069f0
    Replacement::draft(0x141606aa0, "unknown_141606aa0", fn_141606aa0 as *const ()), // sub_141606aa0
    Replacement::draft(0x141606b80, "unknown_141606b80", fn_141606b80 as *const ()), // sub_141606b80
    Replacement::draft(0x141606d40, "unknown_141606d40", fn_141606d40 as *const ()), // sub_141606d40
    Replacement::draft(0x141606ea0, "unknown_141606ea0", fn_141606ea0 as *const ()), // sub_141606ea0
    Replacement::draft(0x141606eb0, "unknown_141606eb0", fn_141606eb0 as *const ()), // sub_141606eb0
    Replacement::draft(0x141607110, "unknown_141607110", fn_141607110 as *const ()), // sub_141607110
    Replacement::draft(0x141607120, "unknown_141607120", fn_141607120 as *const ()), // sub_141607120
    Replacement::draft(0x1416071f0, "unknown_1416071f0", fn_1416071f0 as *const ()), // sub_1416071f0
    Replacement::draft(0x1416072c0, "unknown_1416072c0", fn_1416072c0 as *const ()), // sub_1416072c0
    Replacement::draft(0x1416072d0, "unknown_1416072d0", tpl_30363::<0x2d> as *const ()), // sub_1416072d0
    Replacement::draft(0x1416072e0, "unknown_1416072e0", fn_1416072e0 as *const ()), // sub_1416072e0
    Replacement::draft(0x1416074d0, "unknown_1416074d0", tpl_28365::<0x80> as *const ()), // sub_1416074d0
    Replacement::draft(0x1416074e0, "unknown_1416074e0", tpl_28365::<0x7c> as *const ()), // sub_1416074e0
    Replacement::draft(0x1416074f0, "unknown_1416074f0", fn_1416074f0 as *const ()), // sub_1416074f0
    Replacement::draft(0x141607700, "unknown_141607700", fn_141607700 as *const ()), // sub_141607700
    Replacement::draft(0x1416077b0, "unknown_1416077b0", fn_1416077b0 as *const ()), // sub_1416077b0
    Replacement::draft(0x1416078b0, "unknown_1416078b0", fn_1416078b0 as *const ()), // sub_1416078b0
    Replacement::draft(0x141607930, "unknown_141607930", fn_141607930 as *const ()), // sub_141607930
    Replacement::draft(0x141607960, "unknown_141607960", tpl_524::<0x18> as *const ()), // sub_141607960
    Replacement::draft(0x141607970, "unknown_141607970", fn_141607970 as *const ()), // sub_141607970
    Replacement::draft(0x1416079b0, "unknown_1416079b0", fn_1416079b0 as *const ()), // sub_1416079b0
    Replacement::draft(0x141607a50, "unknown_141607a50", fn_141607a50 as *const ()), // sub_141607a50
    Replacement::draft(0x141607ad0, "unknown_141607ad0", fn_141607ad0 as *const ()), // sub_141607ad0
    Replacement::draft(0x141607bc0, "unknown_141607bc0", fn_141607bc0 as *const ()), // sub_141607bc0
    Replacement::draft(0x141607d90, "unknown_141607d90", fn_141607d90 as *const ()), // sub_141607d90
    Replacement::draft(0x141607dd0, "unknown_141607dd0", fn_141607dd0 as *const ()), // sub_141607dd0
    Replacement::draft(0x141607e30, "unknown_141607e30", fn_141607e30 as *const ()), // sub_141607e30
    Replacement::draft(0x141608040, "unknown_141608040", fn_141608040 as *const ()), // sub_141608040
    Replacement::draft(0x1416080c0, "unknown_1416080c0", fn_1416080c0 as *const ()), // sub_1416080c0
    Replacement::draft(0x14160817c, "unknown_14160817c", tpl_28319::<0x141608200> as *const ()), // sub_14160817c
    Replacement::draft(0x141608188, "unknown_141608188", fn_141608188 as *const ()), // sub_141608188
    Replacement::draft(0x141608194, "unknown_141608194", tpl_28319::<0x141608280> as *const ()), // sub_141608194
    Replacement::draft(0x1416081a0, "unknown_1416081a0", tpl_28319::<0x1416082c0> as *const ()), // sub_1416081a0
    Replacement::draft(0x1416081ac, "unknown_1416081ac", tpl_28319::<0x141608310> as *const ()), // sub_1416081ac
    Replacement::draft(0x1416081c0, "unknown_1416081c0", tpl_495::<0x1415bd160, 0x141834d30> as *const ()), // sub_1416081c0
    Replacement::draft(0x141608200, "unknown_141608200", fn_141608200 as *const ()), // sub_141608200
    Replacement::draft(0x141608280, "unknown_141608280", fn_141608280 as *const ()), // sub_141608280
    Replacement::draft(0x1416082c0, "concurrency_1416082c0", tpl_30380::<0x14268ee60, 0x14268ee90, 0x18, 0x1416080c0, 0x141834d30> as *const ()), // sub_1416082c0
    Replacement::draft(0x141608310, "concurrency_141608310", tpl_30380::<0x14268edb0, 0x14268ede0, 0x18, 0x1416080c0, 0x141834d30> as *const ()), // sub_141608310
    Replacement::draft(0x141608360, "unknown_141608360", fn_141608360 as *const ()), // sub_141608360
    Replacement::draft(0x1416084f0, "unknown_1416084f0", fn_1416084f0 as *const ()), // sub_1416084f0
    Replacement::draft(0x141608780, "unknown_141608780", fn_141608780 as *const ()), // sub_141608780
    Replacement::draft(0x1416087b0, "unknown_1416087b0", fn_1416087b0 as *const ()), // sub_1416087b0
    Replacement::draft(0x1416087d0, "unknown_1416087d0", fn_1416087d0 as *const ()), // sub_1416087d0
    Replacement::draft(0x141608850, "unknown_141608850", tpl_30385::<0x141524330, 0x141608040, 0x14268ee60, 0x14268ee90, 0x18> as *const ()), // sub_141608850
    Replacement::draft(0x1416088a0, "unknown_1416088a0", tpl_30385::<0x141524330, 0x141608040, 0x14268edb0, 0x14268ede0, 0x18> as *const ()), // sub_1416088a0
    Replacement::draft(0x1416088f0, "unknown_1416088f0", fn_1416088f0 as *const ()), // sub_1416088f0
    Replacement::draft(0x141608940, "unknown_141608940", fn_141608940 as *const ()), // sub_141608940
    Replacement::draft(0x141608970, "unknown_141608970", fn_141608970 as *const ()), // sub_141608970
    Replacement::draft(0x1416089a0, "unknown_1416089a0", tpl_29742::<0x58, 0x30> as *const ()), // sub_1416089a0
    Replacement::draft(0x1416089b0, "unknown_1416089b0", fn_1416089b0 as *const ()), // sub_1416089b0
    Replacement::draft(0x141608b60, "unknown_141608b60", fn_141608b60 as *const ()), // sub_141608b60
    Replacement::draft(0x141608b80, "unknown_141608b80", fn_141608b80 as *const ()), // sub_141608b80
    Replacement::draft(0x141608d60, "unknown_141608d60", fn_141608d60 as *const ()), // sub_141608d60
    Replacement::draft(0x141608f20, "unknown_141608f20", fn_141608f20 as *const ()), // sub_141608f20
    Replacement::draft(0x1416090f0, "unknown_1416090f0", fn_1416090f0 as *const ()), // sub_1416090f0
    Replacement::draft(0x141609190, "unknown_141609190", fn_141609190 as *const ()), // sub_141609190
    Replacement::draft(0x1416093a0, "unknown_1416093a0", fn_1416093a0 as *const ()), // sub_1416093a0
    Replacement::draft(0x141609670, "unknown_141609670", fn_141609670 as *const ()), // sub_141609670
    Replacement::draft(0x141609700, "unknown_141609700", fn_141609700 as *const ()), // sub_141609700
    Replacement::draft(0x141609710, "unknown_141609710", fn_141609710 as *const ()), // sub_141609710
    Replacement::draft(0x141609720, "unknown_141609720", fn_141609720 as *const ()), // sub_141609720
    Replacement::draft(0x141609730, "unknown_141609730", fn_141609730 as *const ()), // sub_141609730
    Replacement::draft(0x141609810, "unknown_141609810", fn_141609810 as *const ()), // sub_141609810
    Replacement::draft(0x141609950, "unknown_141609950", fn_141609950 as *const ()), // sub_141609950
    Replacement::draft(0x141609a10, "unknown_141609a10", fn_141609a10 as *const ()), // sub_141609a10
    Replacement::draft(0x141609a70, "unknown_141609a70", fn_141609a70 as *const ()), // sub_141609a70
    Replacement::draft(0x141609d30, "unknown_141609d30", fn_141609d30 as *const ()), // sub_141609d30
    Replacement::draft(0x141609de0, "unknown_141609de0", fn_141609de0 as *const ()), // sub_141609de0
    Replacement::draft(0x141609ea0, "unknown_141609ea0", fn_141609ea0 as *const ()), // sub_141609ea0
    Replacement::draft(0x141609f50, "unknown_141609f50", fn_141609f50 as *const ()), // sub_141609f50
    Replacement::draft(0x141609f90, "unknown_141609f90", fn_141609f90 as *const ()), // sub_141609f90
    Replacement::draft(0x14160a050, "unknown_14160a050", tpl_30403::<0x88, 0x143815b50, 0x8, 0x14160a090> as *const ()), // sub_14160a050
    Replacement::draft(0x14160a090, "unknown_14160a090", fn_14160a090 as *const ()), // sub_14160a090
    Replacement::draft(0x14160a1d0, "unknown_14160a1d0", fn_14160a1d0 as *const ()), // sub_14160a1d0
    Replacement::draft(0x14160a220, "unknown_14160a220", fn_14160a220 as *const ()), // sub_14160a220
    Replacement::draft(0x14160a240, "unknown_14160a240", fn_14160a240 as *const ()), // sub_14160a240
    Replacement::draft(0x14160a410, "unknown_14160a410", fn_14160a410 as *const ()), // sub_14160a410
    Replacement::draft(0x14160a710, "unknown_14160a710", fn_14160a710 as *const ()), // sub_14160a710
    Replacement::draft(0x14160a980, "unknown_14160a980", fn_14160a980 as *const ()), // sub_14160a980
    Replacement::draft(0x14160aa40, "unknown_14160aa40", fn_14160aa40 as *const ()), // sub_14160aa40
    Replacement::draft(0x14160af00, "unknown_14160af00", fn_14160af00 as *const ()), // sub_14160af00
    Replacement::draft(0x14160b040, "unknown_14160b040", fn_14160b040 as *const ()), // sub_14160b040
    Replacement::draft(0x14160b200, "unknown_14160b200", fn_14160b200 as *const ()), // sub_14160b200
    Replacement::draft(0x14160b2c0, "unknown_14160b2c0", fn_14160b2c0 as *const ()), // sub_14160b2c0
    Replacement::draft(0x14160b460, "unknown_14160b460", fn_14160b460 as *const ()), // sub_14160b460
    Replacement::draft(0x14160b590, "unknown_14160b590", fn_14160b590 as *const ()), // sub_14160b590
    Replacement::draft(0x14160b740, "unknown_14160b740", fn_14160b740 as *const ()), // sub_14160b740
    Replacement::draft(0x14160b950, "unknown_14160b950", fn_14160b950 as *const ()), // sub_14160b950
    Replacement::draft(0x14160b9f0, "unknown_14160b9f0", tpl_6019::<0x14268efe0, 0x141834d30> as *const ()), // sub_14160b9f0
    Replacement::draft(0x14160ba20, "unknown_14160ba20", tpl_27062::<0x14268efe0, 0x141834d30> as *const ()), // sub_14160ba20
    Replacement::draft(0x14160ba50, "unknown_14160ba50", fn_14160ba50 as *const ()), // sub_14160ba50
    Replacement::draft(0x14160bb90, "unknown_14160bb90", fn_14160bb90 as *const ()), // sub_14160bb90
    Replacement::draft(0x14160bba0, "unknown_14160bba0", fn_14160bba0 as *const ()), // sub_14160bba0
    Replacement::draft(0x14160bc90, "unknown_14160bc90", fn_14160bc90 as *const ()), // sub_14160bc90
    Replacement::draft(0x14160bcb0, "unknown_14160bcb0", tpl_29742::<0x8, 0x114> as *const ()), // sub_14160bcb0
    Replacement::draft(0x14160bcd0, "unknown_14160bcd0", fn_14160bcd0 as *const ()), // sub_14160bcd0
    Replacement::draft(0x14160bd70, "unknown_14160bd70", fn_14160bd70 as *const ()), // sub_14160bd70
    Replacement::draft(0x14160bda0, "unknown_14160bda0", fn_14160bda0 as *const ()), // sub_14160bda0
    Replacement::draft(0x14160bdd0, "unknown_14160bdd0", tpl_30417::<0x8, 0x14160b950, 0x14164d950> as *const ()), // sub_14160bdd0
    Replacement::draft(0x14160be20, "unknown_14160be20", tpl_30417::<0x8, 0x14160b950, 0x14164d910> as *const ()), // sub_14160be20
    Replacement::draft(0x14160be70, "unknown_14160be70", fn_14160be70 as *const ()), // sub_14160be70
    Replacement::draft(0x14160bea0, "unknown_14160bea0", tpl_30419::<0x8> as *const ()), // sub_14160bea0
    Replacement::draft(0x14160beb0, "unknown_14160beb0", fn_14160beb0 as *const ()), // sub_14160beb0
    Replacement::draft(0x14160bee0, "unknown_14160bee0", tpl_30421::<0x8, 0x38, 0x14164daf0> as *const ()), // sub_14160bee0
    Replacement::draft(0x14160bf10, "unknown_14160bf10", tpl_30421::<0x8, 0x38, 0x14164da90> as *const ()), // sub_14160bf10
    Replacement::draft(0x14160bf40, "unknown_14160bf40", fn_14160bf40 as *const ()), // sub_14160bf40
    Replacement::draft(0x14160bf70, "unknown_14160bf70", fn_14160bf70 as *const ()), // sub_14160bf70
    Replacement::draft(0x14160c070, "unknown_14160c070", fn_14160c070 as *const ()), // sub_14160c070
    Replacement::draft(0x14160c0e0, "unknown_14160c0e0", fn_14160c0e0 as *const ()), // sub_14160c0e0
    Replacement::draft(0x14160c1c0, "unknown_14160c1c0", fn_14160c1c0 as *const ()), // sub_14160c1c0
    Replacement::draft(0x14160c240, "unknown_14160c240", fn_14160c240 as *const ()), // sub_14160c240
    Replacement::draft(0x14160c320, "unknown_14160c320", fn_14160c320 as *const ()), // sub_14160c320
    Replacement::draft(0x14160c3a0, "unknown_14160c3a0", fn_14160c3a0 as *const ()), // sub_14160c3a0
    Replacement::draft(0x14160c570, "unknown_14160c570", fn_14160c570 as *const ()), // sub_14160c570
    Replacement::draft(0x14160c7b0, "unknown_14160c7b0", fn_14160c7b0 as *const ()), // sub_14160c7b0
    Replacement::draft(0x14160cae0, "unknown_14160cae0", fn_14160cae0 as *const ()), // sub_14160cae0
    Replacement::draft(0x14160cca0, "unknown_14160cca0", fn_14160cca0 as *const ()), // sub_14160cca0
    Replacement::draft(0x14160cdc0, "unknown_14160cdc0", fn_14160cdc0 as *const ()), // sub_14160cdc0
    Replacement::draft(0x14160ce10, "unknown_14160ce10", fn_14160ce10 as *const ()), // sub_14160ce10
    Replacement::draft(0x14160d040, "unknown_14160d040", fn_14160d040 as *const ()), // sub_14160d040
    Replacement::draft(0x14160d050, "unknown_14160d050", fn_14160d050 as *const ()), // sub_14160d050
    Replacement::draft(0x14160d080, "unknown_14160d080", fn_14160d080 as *const ()), // sub_14160d080
    Replacement::draft(0x14160d150, "unknown_14160d150", fn_14160d150 as *const ()), // sub_14160d150
    Replacement::draft(0x14160d370, "unknown_14160d370", fn_14160d370 as *const ()), // sub_14160d370
    Replacement::draft(0x14160d400, "unknown_14160d400", fn_14160d400 as *const ()), // sub_14160d400
    Replacement::draft(0x14160d430, "unknown_14160d430", fn_14160d430 as *const ()), // sub_14160d430
    Replacement::draft(0x14160d630, "unknown_14160d630", fn_14160d630 as *const ()), // sub_14160d630
    Replacement::draft(0x14160d690, "unknown_14160d690", fn_14160d690 as *const ()), // sub_14160d690
    Replacement::draft(0x14160db00, "unknown_14160db00", fn_14160db00 as *const ()), // sub_14160db00
    Replacement::draft(0x14160db80, "unknown_14160db80", fn_14160db80 as *const ()), // sub_14160db80
    Replacement::draft(0x14160dc20, "unknown_14160dc20", fn_14160dc20 as *const ()), // sub_14160dc20
    Replacement::draft(0x14160df00, "unknown_14160df00", fn_14160df00 as *const ()), // sub_14160df00
    Replacement::draft(0x14160dfc0, "unknown_14160dfc0", fn_14160dfc0 as *const ()), // sub_14160dfc0
    Replacement::draft(0x14160e090, "unknown_14160e090", fn_14160e090 as *const ()), // sub_14160e090
    Replacement::draft(0x14160e140, "unknown_14160e140", fn_14160e140 as *const ()), // sub_14160e140
    Replacement::draft(0x14160e340, "unknown_14160e340", fn_14160e340 as *const ()), // sub_14160e340
    Replacement::draft(0x14160e370, "unknown_14160e370", fn_14160e370 as *const ()), // sub_14160e370
    Replacement::draft(0x14160e5c0, "unknown_14160e5c0", fn_14160e5c0 as *const ()), // sub_14160e5c0
    Replacement::draft(0x14160e660, "unknown_14160e660", fn_14160e660 as *const ()), // sub_14160e660
    Replacement::draft(0x14160e7d0, "unknown_14160e7d0", fn_14160e7d0 as *const ()), // sub_14160e7d0
    Replacement::draft(0x14160e910, "unknown_14160e910", fn_14160e910 as *const ()), // sub_14160e910
    Replacement::draft(0x14160e9b0, "unknown_14160e9b0", fn_14160e9b0 as *const ()), // sub_14160e9b0
    Replacement::draft(0x14160eaa0, "unknown_14160eaa0", fn_14160eaa0 as *const ()), // sub_14160eaa0
    Replacement::draft(0x14160eb20, "unknown_14160eb20", fn_14160eb20 as *const ()), // sub_14160eb20
    Replacement::draft(0x14160eca0, "unknown_14160eca0", fn_14160eca0 as *const ()), // sub_14160eca0
    Replacement::draft(0x14160ee30, "unknown_14160ee30", fn_14160ee30 as *const ()), // sub_14160ee30
    Replacement::draft(0x14160f0e0, "unknown_14160f0e0", fn_14160f0e0 as *const ()), // sub_14160f0e0
    Replacement::draft(0x14160f380, "unknown_14160f380", fn_14160f380 as *const ()), // sub_14160f380
    Replacement::draft(0x14160f420, "unknown_14160f420", fn_14160f420 as *const ()), // sub_14160f420
    Replacement::draft(0x14160f4a0, "unknown_14160f4a0", fn_14160f4a0 as *const ()), // sub_14160f4a0
    Replacement::draft(0x14160f4f0, "unknown_14160f4f0", fn_14160f4f0 as *const ()), // sub_14160f4f0
    Replacement::draft(0x14160f560, "unknown_14160f560", fn_14160f560 as *const ()), // sub_14160f560
    Replacement::draft(0x14160f710, "unknown_14160f710", fn_14160f710 as *const ()), // sub_14160f710
    Replacement::draft(0x14160f8e0, "unknown_14160f8e0", fn_14160f8e0 as *const ()), // sub_14160f8e0
    Replacement::draft(0x14160f9a0, "unknown_14160f9a0", fn_14160f9a0 as *const ()), // sub_14160f9a0
    Replacement::draft(0x14160fa60, "unknown_14160fa60", fn_14160fa60 as *const ()), // sub_14160fa60
    Replacement::draft(0x14160fa70, "unknown_14160fa70", fn_14160fa70 as *const ()), // sub_14160fa70
    Replacement::draft(0x14160fa80, "unknown_14160fa80", fn_14160fa80 as *const ()), // sub_14160fa80
    Replacement::draft(0x14160fbb0, "unknown_14160fbb0", fn_14160fbb0 as *const ()), // sub_14160fbb0
    Replacement::draft(0x14160fce0, "unknown_14160fce0", fn_14160fce0 as *const ()), // sub_14160fce0
    Replacement::draft(0x14160fe00, "unknown_14160fe00", fn_14160fe00 as *const ()), // sub_14160fe00
    Replacement::draft(0x14160ff20, "unknown_14160ff20", fn_14160ff20 as *const ()), // sub_14160ff20
    Replacement::draft(0x14160ff40, "unknown_14160ff40", fn_14160ff40 as *const ()), // sub_14160ff40
    Replacement::draft(0x141610260, "unknown_141610260", fn_141610260 as *const ()), // sub_141610260
    Replacement::draft(0x141610270, "unknown_141610270", tpl_28392::<0x141610290> as *const ()), // sub_141610270
    Replacement::draft(0x14161027c, "unknown_14161027c", tpl_28393::<0x141610290> as *const ()), // sub_14161027c
    Replacement::draft(0x141610290, "unknown_141610290", fn_141610290 as *const ()), // sub_141610290
    Replacement::draft(0x141610300, "unknown_141610300", fn_141610300 as *const ()), // sub_141610300
    Replacement::draft(0x1416103b0, "unknown_1416103b0", tpl_365::<0x141610420> as *const ()), // sub_1416103b0
    Replacement::draft(0x1416103c0, "unknown_1416103c0", fn_1416103c0 as *const ()), // sub_1416103c0
    Replacement::draft(0x141610420, "unknown_141610420", fn_141610420 as *const ()), // sub_141610420
    Replacement::draft(0x141610560, "unknown_141610560", fn_141610560 as *const ()), // sub_141610560
    Replacement::draft(0x1416105b0, "unknown_1416105b0", fn_1416105b0 as *const ()), // sub_1416105b0
    Replacement::draft(0x141610650, "unknown_141610650", fn_141610650 as *const ()), // sub_141610650
    Replacement::draft(0x141610710, "unknown_141610710", fn_141610710 as *const ()), // sub_141610710
    Replacement::draft(0x141610960, "unknown_141610960", tpl_30459::<0xf0> as *const ()), // sub_141610960
    Replacement::draft(0x141610970, "unknown_141610970", fn_141610970 as *const ()), // sub_141610970
    Replacement::draft(0x1416109b0, "unknown_1416109b0", tpl_30209::<0x1415c61f0> as *const ()), // sub_1416109b0
    Replacement::draft(0x1416109c0, "unknown_1416109c0", fn_1416109c0 as *const ()), // sub_1416109c0
    Replacement::draft(0x141610a90, "unknown_141610a90", tpl_30209::<0x1415c64c0> as *const ()), // sub_141610a90
    Replacement::draft(0x141610aa0, "unknown_141610aa0", tpl_30461::<0x14268f478, 0x1416513c0, 0x141834d30> as *const ()), // sub_141610aa0
    Replacement::draft(0x141610ae0, "unknown_141610ae0", fn_141610ae0 as *const ()), // sub_141610ae0
    Replacement::draft(0x141610b40, "unknown_141610b40", fn_141610b40 as *const ()), // sub_141610b40
    Replacement::draft(0x141610cd0, "unknown_141610cd0", fn_141610cd0 as *const ()), // sub_141610cd0
    Replacement::draft(0x141610cf0, "unknown_141610cf0", fn_141610cf0 as *const ()), // sub_141610cf0
    Replacement::draft(0x141610d20, "unknown_141610d20", fn_141610d20 as *const ()), // sub_141610d20
    Replacement::draft(0x141610eb0, "unknown_141610eb0", tpl_26964::<0x141610d20, 0x141834d30> as *const ()), // sub_141610eb0
    Replacement::draft(0x141610ef0, "unknown_141610ef0", fn_141610ef0 as *const ()), // sub_141610ef0
    Replacement::draft(0x141610fc0, "unknown_141610fc0", tpl_30467::<0x1415c2140> as *const ()), // sub_141610fc0
    Replacement::draft(0x141610fd0, "unknown_141610fd0", fn_141610fd0 as *const ()), // sub_141610fd0
    Replacement::draft(0x141610ff0, "unknown_141610ff0", fn_141610ff0 as *const ()), // sub_141610ff0
    Replacement::draft(0x1416111e0, "unknown_1416111e0", fn_1416111e0 as *const ()), // sub_1416111e0
    Replacement::draft(0x141611300, "unknown_141611300", fn_141611300 as *const ()), // sub_141611300
    Replacement::draft(0x1416116f0, "unknown_1416116f0", fn_1416116f0 as *const ()), // sub_1416116f0
    Replacement::draft(0x141611720, "unknown_141611720", tpl_30467::<0x1415c2870> as *const ()), // sub_141611720
    Replacement::draft(0x141611730, "unknown_141611730", tpl_4544::<0xc4> as *const ()), // sub_141611730
    Replacement::draft(0x141611740, "unknown_141611740", fn_141611740 as *const ()), // sub_141611740
    Replacement::draft(0x141611790, "unknown_141611790", fn_141611790 as *const ()), // sub_141611790
    Replacement::draft(0x1416117b0, "unknown_1416117b0", fn_1416117b0 as *const ()), // sub_1416117b0
    Replacement::draft(0x1416117f0, "unknown_1416117f0", tpl_28365::<0xbc> as *const ()), // sub_1416117f0
    Replacement::draft(0x141611800, "unknown_141611800", fn_141611800 as *const ()), // sub_141611800
    Replacement::draft(0x141611810, "unknown_141611810", tpl_30363::<0xc5> as *const ()), // sub_141611810
    Replacement::draft(0x141611820, "unknown_141611820", fn_141611820 as *const ()), // sub_141611820
    Replacement::draft(0x1416119b0, "unknown_1416119b0", tpl_30476::<0xd8, 0x48, 0x143930f80, 0x1415adef0, 0x1415ad990> as *const ()), // sub_1416119b0
    Replacement::draft(0x141611a20, "unknown_141611a20", tpl_30476::<0xd8, 0x40, 0x143930f80, 0x1415adef0, 0x1415ad990> as *const ()), // sub_141611a20
    Replacement::draft(0x141611a90, "unknown_141611a90", tpl_28365::<0xc0> as *const ()), // sub_141611a90
    Replacement::draft(0x141611aa0, "unknown_141611aa0", fn_141611aa0 as *const ()), // sub_141611aa0
    Replacement::draft(0x141611ab0, "unknown_141611ab0", fn_141611ab0 as *const ()), // sub_141611ab0
    Replacement::draft(0x141611ad0, "unknown_141611ad0", fn_141611ad0 as *const ()), // sub_141611ad0
    Replacement::draft(0x141611b90, "unknown_141611b90", fn_141611b90 as *const ()), // sub_141611b90
    Replacement::draft(0x141611c80, "unknown_141611c80", fn_141611c80 as *const ()), // sub_141611c80
    Replacement::draft(0x141611f20, "unknown_141611f20", fn_141611f20 as *const ()), // sub_141611f20
    Replacement::draft(0x141611fb0, "unknown_141611fb0", fn_141611fb0 as *const ()), // sub_141611fb0
    Replacement::draft(0x141612060, "unknown_141612060", fn_141612060 as *const ()), // sub_141612060
    Replacement::draft(0x141612110, "unknown_141612110", fn_141612110 as *const ()), // sub_141612110
    Replacement::draft(0x1416121e0, "unknown_1416121e0", fn_1416121e0 as *const ()), // sub_1416121e0
    Replacement::draft(0x141612300, "unknown_141612300", tpl_30071::<0x4c> as *const ()), // sub_141612300
    Replacement::draft(0x141612310, "unknown_141612310", fn_141612310 as *const ()), // sub_141612310
    Replacement::draft(0x141612350, "unknown_141612350", fn_141612350 as *const ()), // sub_141612350
    Replacement::draft(0x141612370, "unknown_141612370", fn_141612370 as *const ()), // sub_141612370
    Replacement::draft(0x1416123b0, "unknown_1416123b0", fn_1416123b0 as *const ()), // sub_1416123b0
    Replacement::draft(0x1416123e0, "unknown_1416123e0", fn_1416123e0 as *const ()), // sub_1416123e0
    Replacement::draft(0x141612550, "unknown_141612550", fn_141612550 as *const ()), // sub_141612550
    Replacement::draft(0x141612570, "unknown_141612570", fn_141612570 as *const ()), // sub_141612570
    Replacement::draft(0x141612750, "unknown_141612750", fn_141612750 as *const ()), // sub_141612750
    Replacement::draft(0x141612930, "unknown_141612930", fn_141612930 as *const ()), // sub_141612930
    Replacement::draft(0x1416129e0, "unknown_1416129e0", fn_1416129e0 as *const ()), // sub_1416129e0
    Replacement::draft(0x141612a20, "unknown_141612a20", fn_141612a20 as *const ()), // sub_141612a20
    Replacement::draft(0x141612b20, "unknown_141612b20", fn_141612b20 as *const ()), // sub_141612b20
    Replacement::draft(0x141612b30, "unknown_141612b30", fn_141612b30 as *const ()), // sub_141612b30
    Replacement::draft(0x141612c40, "unknown_141612c40", fn_141612c40 as *const ()), // sub_141612c40
    Replacement::draft(0x141612c90, "unknown_141612c90", tpl_30499::<0x18, 0x20, 0x40, 0x10, 0x2c, 0x8, 0x14160db00, 0x30> as *const ()), // sub_141612c90
    Replacement::draft(0x141612d50, "unknown_141612d50", tpl_30499::<0x18, 0x20, 0x40, 0x10, 0x2c, 0x8, 0x14160db80, 0x30> as *const ()), // sub_141612d50
    Replacement::draft(0x141612e10, "unknown_141612e10", fn_141612e10 as *const ()), // sub_141612e10
    Replacement::draft(0x141613100, "unknown_141613100", fn_141613100 as *const ()), // sub_141613100
    Replacement::draft(0x1416131c0, "unknown_1416131c0", fn_1416131c0 as *const ()), // sub_1416131c0
    Replacement::draft(0x1416133c0, "unknown_1416133c0", fn_1416133c0 as *const ()), // sub_1416133c0
    Replacement::draft(0x141613400, "unknown_141613400", fn_141613400 as *const ()), // sub_141613400
    Replacement::draft(0x1416135c0, "unknown_1416135c0", fn_1416135c0 as *const ()), // sub_1416135c0
    Replacement::draft(0x1416135f0, "unknown_1416135f0", fn_1416135f0 as *const ()), // sub_1416135f0
    Replacement::draft(0x141613740, "unknown_141613740", fn_141613740 as *const ()), // sub_141613740
    Replacement::draft(0x141613770, "unknown_141613770", fn_141613770 as *const ()), // sub_141613770
    Replacement::draft(0x141613820, "unknown_141613820", fn_141613820 as *const ()), // sub_141613820
    Replacement::draft(0x141613840, "unknown_141613840", fn_141613840 as *const ()), // sub_141613840
    Replacement::draft(0x141613850, "unknown_141613850", fn_141613850 as *const ()), // sub_141613850
    Replacement::draft(0x141613860, "unknown_141613860", tpl_365::<0x141651a00> as *const ()), // sub_141613860
    Replacement::draft(0x141613870, "unknown_141613870", fn_141613870 as *const ()), // sub_141613870
    Replacement::draft(0x1416138e0, "unknown_1416138e0", fn_1416138e0 as *const ()), // sub_1416138e0
    Replacement::draft(0x141613980, "unknown_141613980", fn_141613980 as *const ()), // sub_141613980
    Replacement::draft(0x1416139f0, "unknown_1416139f0", fn_1416139f0 as *const ()), // sub_1416139f0
    Replacement::draft(0x141613aa0, "unknown_141613aa0", fn_141613aa0 as *const ()), // sub_141613aa0
    Replacement::draft(0x141613c30, "unknown_141613c30", fn_141613c30 as *const ()), // sub_141613c30
    Replacement::draft(0x141613d50, "unknown_141613d50", fn_141613d50 as *const ()), // sub_141613d50
    Replacement::draft(0x141613f80, "unknown_141613f80", fn_141613f80 as *const ()), // sub_141613f80
    Replacement::draft(0x141614030, "unknown_141614030", fn_141614030 as *const ()), // sub_141614030
    Replacement::draft(0x1416140e0, "unknown_1416140e0", fn_1416140e0 as *const ()), // sub_1416140e0
    Replacement::draft(0x1416140f0, "unknown_1416140f0", fn_1416140f0 as *const ()), // sub_1416140f0
    Replacement::draft(0x1416144d0, "unknown_1416144d0", fn_1416144d0 as *const ()), // sub_1416144d0
    Replacement::draft(0x141614590, "unknown_141614590", fn_141614590 as *const ()), // sub_141614590
    Replacement::draft(0x141614f90, "unknown_141614f90", fn_141614f90 as *const ()), // sub_141614f90
    Replacement::draft(0x141615120, "unknown_141615120", fn_141615120 as *const ()), // sub_141615120
    Replacement::draft(0x141615340, "unknown_141615340", fn_141615340 as *const ()), // sub_141615340
    Replacement::draft(0x141615420, "unknown_141615420", fn_141615420 as *const ()), // sub_141615420
    Replacement::draft(0x141615700, "unknown_141615700", fn_141615700 as *const ()), // sub_141615700
    Replacement::draft(0x1416158d0, "unknown_1416158d0", fn_1416158d0 as *const ()), // sub_1416158d0
    Replacement::draft(0x141615b30, "unknown_141615b30", fn_141615b30 as *const ()), // sub_141615b30
    Replacement::draft(0x141615c60, "unknown_141615c60", fn_141615c60 as *const ()), // sub_141615c60
    Replacement::draft(0x141615d20, "unknown_141615d20", fn_141615d20 as *const ()), // sub_141615d20
    Replacement::draft(0x141615e10, "unknown_141615e10", fn_141615e10 as *const ()), // sub_141615e10
    Replacement::draft(0x141615e90, "unknown_141615e90", fn_141615e90 as *const ()), // sub_141615e90
    Replacement::draft(0x141615f10, "unknown_141615f10", fn_141615f10 as *const ()), // sub_141615f10
    Replacement::draft(0x141615f90, "unknown_141615f90", fn_141615f90 as *const ()), // sub_141615f90
    Replacement::draft(0x141616010, "opentelemetry_141616010", tpl_27686::<0x141615d20, 0x141834d30> as *const ()), // sub_141616010
    Replacement::draft(0x141616050, "unknown_141616050", fn_141616050 as *const ()), // sub_141616050
    Replacement::draft(0x141616110, "unknown_141616110", fn_141616110 as *const ()), // sub_141616110
    Replacement::draft(0x141616190, "unknown_141616190", fn_141616190 as *const ()), // sub_141616190
    Replacement::draft(0x141616220, "unknown_141616220", fn_141616220 as *const ()), // sub_141616220
    Replacement::draft(0x1416162b0, "unknown_1416162b0", fn_1416162b0 as *const ()), // sub_1416162b0
    Replacement::draft(0x141616340, "unknown_141616340", fn_141616340 as *const ()), // sub_141616340
    Replacement::draft(0x141616430, "unknown_141616430", fn_141616430 as *const ()), // sub_141616430
    Replacement::draft(0x1416164e0, "unknown_1416164e0", fn_1416164e0 as *const ()), // sub_1416164e0
    Replacement::draft(0x141616590, "unknown_141616590", fn_141616590 as *const ()), // sub_141616590
    Replacement::draft(0x141616640, "unknown_141616640", fn_141616640 as *const ()), // sub_141616640
    Replacement::draft(0x1416167c0, "unknown_1416167c0", fn_1416167c0 as *const ()), // sub_1416167c0
    Replacement::draft(0x141616940, "unknown_141616940", fn_141616940 as *const ()), // sub_141616940
    Replacement::draft(0x141616a20, "unknown_141616a20", fn_141616a20 as *const ()), // sub_141616a20
    Replacement::draft(0x141616a30, "unknown_141616a30", fn_141616a30 as *const ()), // sub_141616a30
    Replacement::draft(0x1416173c0, "unknown_1416173c0", fn_1416173c0 as *const ()), // sub_1416173c0
    Replacement::draft(0x141617de0, "unknown_141617de0", fn_141617de0 as *const ()), // sub_141617de0
    Replacement::draft(0x141618380, "unknown_141618380", fn_141618380 as *const ()), // sub_141618380
    Replacement::draft(0x141618400, "unknown_141618400", fn_141618400 as *const ()), // sub_141618400
    Replacement::draft(0x1416184d0, "unknown_1416184d0", tpl_89472::<0x141524340, 0x141524330, 0x141615e10, 0x1415242e0> as *const ()), // sub_1416184d0
    Replacement::draft(0x1416185e0, "unknown_1416185e0", fn_1416185e0 as *const ()), // sub_1416185e0
    Replacement::draft(0x1416186f0, "unknown_1416186f0", tpl_89472::<0x141524340, 0x141524330, 0x141615f10, 0x1415242e0> as *const ()), // sub_1416186f0
    Replacement::draft(0x141618800, "unknown_141618800", fn_141618800 as *const ()), // sub_141618800
    Replacement::draft(0x141618910, "unknown_141618910", fn_141618910 as *const ()), // sub_141618910
    Replacement::draft(0x141618a40, "unknown_141618a40", fn_141618a40 as *const ()), // sub_141618a40
    Replacement::draft(0x141618a90, "unknown_141618a90", fn_141618a90 as *const ()), // sub_141618a90
    Replacement::draft(0x141618ad0, "unknown_141618ad0", fn_141618ad0 as *const ()), // sub_141618ad0
    Replacement::draft(0x141618d20, "unknown_141618d20", fn_141618d20 as *const ()), // sub_141618d20
    Replacement::draft(0x141618d30, "unknown_141618d30", fn_141618d30 as *const ()), // sub_141618d30
    Replacement::draft(0x141618db0, "unknown_141618db0", tpl_89481::<0x141616190, 0x1416184d0, 0x14183acc8> as *const ()), // sub_141618db0
    Replacement::draft(0x141618f00, "unknown_141618f00", tpl_89481::<0x141616190, 0x1416186f0, 0x14183acc8> as *const ()), // sub_141618f00
    Replacement::draft(0x141619050, "unknown_141619050", fn_141619050 as *const ()), // sub_141619050
    Replacement::draft(0x141619190, "unknown_141619190", fn_141619190 as *const ()), // sub_141619190
    Replacement::draft(0x1416192e0, "unknown_1416192e0", fn_1416192e0 as *const ()), // sub_1416192e0
    Replacement::draft(0x141619440, "luau_141619440", fn_141619440 as *const ()), // sub_141619440
    Replacement::draft(0x1416195b0, "unknown_1416195b0", fn_1416195b0 as *const ()), // sub_1416195b0
    Replacement::draft(0x141619630, "unknown_141619630", fn_141619630 as *const ()), // sub_141619630
    Replacement::draft(0x1416196d0, "unknown_1416196d0", fn_1416196d0 as *const ()), // sub_1416196d0
    Replacement::draft(0x1416196f0, "unknown_1416196f0", fn_1416196f0 as *const ()), // sub_1416196f0
    Replacement::draft(0x141619780, "unknown_141619780", fn_141619780 as *const ()), // sub_141619780
    Replacement::draft(0x141619790, "unknown_141619790", fn_141619790 as *const ()), // sub_141619790
    Replacement::draft(0x1416198b0, "unknown_1416198b0", fn_1416198b0 as *const ()), // sub_1416198b0
    Replacement::draft(0x141619960, "unknown_141619960", fn_141619960 as *const ()), // sub_141619960
    Replacement::draft(0x141619b80, "unknown_141619b80", fn_141619b80 as *const ()), // sub_141619b80
    Replacement::draft(0x141619d00, "unknown_141619d00", fn_141619d00 as *const ()), // sub_141619d00
    Replacement::draft(0x14161a3b0, "unknown_14161a3b0", fn_14161a3b0 as *const ()), // sub_14161a3b0
    Replacement::draft(0x14161a540, "unknown_14161a540", fn_14161a540 as *const ()), // sub_14161a540
    Replacement::draft(0x14161a710, "unknown_14161a710", fn_14161a710 as *const ()), // sub_14161a710
    Replacement::draft(0x14161a8f0, "unknown_14161a8f0", fn_14161a8f0 as *const ()), // sub_14161a8f0
    Replacement::draft(0x14161a940, "unknown_14161a940", fn_14161a940 as *const ()), // sub_14161a940
    Replacement::draft(0x14161a990, "unknown_14161a990", tpl_1738::<0x20> as *const ()), // sub_14161a990
    Replacement::draft(0x14161a9a0, "unknown_14161a9a0", fn_14161a9a0 as *const ()), // sub_14161a9a0
    Replacement::draft(0x14161a9b0, "unknown_14161a9b0", fn_14161a9b0 as *const ()), // sub_14161a9b0
    Replacement::draft(0x14161aa80, "unknown_14161aa80", fn_14161aa80 as *const ()), // sub_14161aa80
    Replacement::draft(0x14161aab0, "unknown_14161aab0", fn_14161aab0 as *const ()), // sub_14161aab0
    Replacement::draft(0x14161ab30, "unknown_14161ab30", fn_14161ab30 as *const ()), // sub_14161ab30
    Replacement::draft(0x14161abe0, "unknown_14161abe0", fn_14161abe0 as *const ()), // sub_14161abe0
    Replacement::draft(0x14161acfc, "unknown_14161acfc", tpl_28392::<0x14161ad20> as *const ()), // sub_14161acfc
    Replacement::draft(0x14161ad08, "unknown_14161ad08", tpl_28393::<0x14161ad20> as *const ()), // sub_14161ad08
    Replacement::draft(0x14161ad20, "concurrency_14161ad20", fn_14161ad20 as *const ()), // sub_14161ad20
    Replacement::draft(0x14161ad80, "concurrency_14161ad80", fn_14161ad80 as *const ()), // sub_14161ad80
    Replacement::draft(0x14161ae20, "concurrency_14161ae20", fn_14161ae20 as *const ()), // sub_14161ae20
    Replacement::draft(0x14161b1c0, "concurrency_14161b1c0", fn_14161b1c0 as *const ()), // sub_14161b1c0
    Replacement::draft(0x14161b20c, "concurrency_14161b20c", tpl_28392::<0x14161b230> as *const ()), // sub_14161b20c
    Replacement::draft(0x14161b218, "concurrency_14161b218", tpl_28393::<0x14161b230> as *const ()), // sub_14161b218
    Replacement::draft(0x14161b230, "concurrency_14161b230", fn_14161b230 as *const ()), // sub_14161b230
    Replacement::draft(0x14161b2b0, "concurrency_14161b2b0", fn_14161b2b0 as *const ()), // sub_14161b2b0
    Replacement::draft(0x14161b370, "concurrency_14161b370", fn_14161b370 as *const ()), // sub_14161b370
    Replacement::draft(0x14161b4e0, "concurrency_14161b4e0", fn_14161b4e0 as *const ()), // sub_14161b4e0
    Replacement::draft(0x14161b590, "concurrency_14161b590", fn_14161b590 as *const ()), // sub_14161b590
    Replacement::draft(0x14161b700, "concurrency_14161b700", tpl_89516::<0x24, 0x14161baf0, 0x70, 0xd0, 0xd4, 0x100, 0xf8, 0xfffffffffffffff8, 0x80, 0x10, 0x14, 0x18> as *const ()), // sub_14161b700
    Replacement::draft(0x14161b7d0, "concurrency_14161b7d0", fn_14161b7d0 as *const ()), // sub_14161b7d0
    Replacement::draft(0x14161baf0, "concurrency_14161baf0", fn_14161baf0 as *const ()), // sub_14161baf0
    Replacement::draft(0x14161bd80, "concurrency_14161bd80", tpl_30566::<0x24, 0x14161baf0, 0x1f, 0x1d, 0x28, 0x1415bdd20, 0x10, 0x14, 0x18> as *const ()), // sub_14161bd80
    Replacement::draft(0x14161bdf0, "concurrency_14161bdf0", fn_14161bdf0 as *const ()), // sub_14161bdf0
    Replacement::draft(0x14161c2e0, "concurrency_14161c2e0", tpl_548::<0x30> as *const ()), // sub_14161c2e0
    Replacement::draft(0x14161c2f0, "concurrency_14161c2f0", fn_14161c2f0 as *const ()), // sub_14161c2f0
    Replacement::draft(0x14161c3f0, "concurrency_14161c3f0", tpl_29360::<0xd6> as *const ()), // sub_14161c3f0
    Replacement::draft(0x14161c400, "concurrency_14161c400", fn_14161c400 as *const ()), // sub_14161c400
    Replacement::draft(0x14161c420, "concurrency_14161c420", fn_14161c420 as *const ()), // sub_14161c420
    Replacement::draft(0x14161c480, "concurrency_14161c480", fn_14161c480 as *const ()), // sub_14161c480
    Replacement::draft(0x14161c510, "concurrency_14161c510", fn_14161c510 as *const ()), // sub_14161c510
    Replacement::draft(0x14161c570, "concurrency_14161c570", fn_14161c570 as *const ()), // sub_14161c570
    Replacement::draft(0x14161c810, "concurrency_14161c810", fn_14161c810 as *const ()), // sub_14161c810
    Replacement::draft(0x14161d110, "concurrency_14161d110", fn_14161d110 as *const ()), // sub_14161d110
    Replacement::draft(0x14161d500, "concurrency_14161d500", fn_14161d500 as *const ()), // sub_14161d500
    Replacement::draft(0x14161d720, "concurrency_14161d720", fn_14161d720 as *const ()), // sub_14161d720
    Replacement::draft(0x14161d7c0, "concurrency_14161d7c0", fn_14161d7c0 as *const ()), // sub_14161d7c0
    Replacement::draft(0x14161d8c0, "concurrency_14161d8c0", fn_14161d8c0 as *const ()), // sub_14161d8c0
    Replacement::draft(0x14161de80, "concurrency_14161de80", fn_14161de80 as *const ()), // sub_14161de80
    Replacement::draft(0x14161e050, "concurrency_14161e050", fn_14161e050 as *const ()), // sub_14161e050
    Replacement::draft(0x14161e2a0, "concurrency_14161e2a0", fn_14161e2a0 as *const ()), // sub_14161e2a0
    Replacement::draft(0x14161e400, "concurrency_14161e400", fn_14161e400 as *const ()), // sub_14161e400
    Replacement::draft(0x14161e450, "concurrency_14161e450", fn_14161e450 as *const ()), // sub_14161e450
    Replacement::draft(0x14161e7e8, "concurrency_14161e7e8", tpl_28392::<0x14161e800> as *const ()), // sub_14161e7e8
    Replacement::draft(0x14161e7f4, "concurrency_14161e7f4", tpl_28393::<0x14161e800> as *const ()), // sub_14161e7f4
    Replacement::draft(0x14161e800, "concurrency_14161e800", fn_14161e800 as *const ()), // sub_14161e800
    Replacement::draft(0x14161e900, "concurrency_14161e900", fn_14161e900 as *const ()), // sub_14161e900
    Replacement::draft(0x14161e970, "concurrency_14161e970", fn_14161e970 as *const ()), // sub_14161e970
    Replacement::draft(0x14161ea50, "concurrency_14161ea50", fn_14161ea50 as *const ()), // sub_14161ea50
    Replacement::draft(0x14161eb40, "concurrency_14161eb40", fn_14161eb40 as *const ()), // sub_14161eb40
    Replacement::draft(0x14161ecb0, "concurrency_14161ecb0", fn_14161ecb0 as *const ()), // sub_14161ecb0
    Replacement::draft(0x14161ee60, "concurrency_14161ee60", fn_14161ee60 as *const ()), // sub_14161ee60
    Replacement::draft(0x14161eeb0, "concurrency_14161eeb0", fn_14161eeb0 as *const ()), // sub_14161eeb0
    Replacement::draft(0x14161efd0, "concurrency_14161efd0", fn_14161efd0 as *const ()), // sub_14161efd0
    Replacement::draft(0x14161f040, "concurrency_14161f040", tpl_89516::<0x24, 0x14161f270, 0x70, 0xd0, 0xd4, 0x100, 0xf8, 0xfffffffffffffff8, 0x80, 0x10, 0x14, 0x18> as *const ()), // sub_14161f040
    Replacement::draft(0x14161f110, "concurrency_14161f110", fn_14161f110 as *const ()), // sub_14161f110
    Replacement::draft(0x14161f270, "concurrency_14161f270", fn_14161f270 as *const ()), // sub_14161f270
    Replacement::draft(0x14161f310, "concurrency_14161f310", tpl_30566::<0x24, 0x14161f270, 0x1f, 0x1d, 0x28, 0x1415bdd20, 0x10, 0x14, 0x18> as *const ()), // sub_14161f310
    Replacement::draft(0x14161f380, "concurrency_14161f380", fn_14161f380 as *const ()), // sub_14161f380
    Replacement::draft(0x14161f3b0, "concurrency_14161f3b0", fn_14161f3b0 as *const ()), // sub_14161f3b0
    Replacement::draft(0x14161f580, "concurrency_14161f580", fn_14161f580 as *const ()), // sub_14161f580
    Replacement::draft(0x14161f890, "concurrency_14161f890", tpl_30049::<0x8, 0x138> as *const ()), // sub_14161f890
    Replacement::draft(0x14161f8a0, "concurrency_14161f8a0", fn_14161f8a0 as *const ()), // sub_14161f8a0
    Replacement::draft(0x14161f990, "concurrency_14161f990", fn_14161f990 as *const ()), // sub_14161f990
    Replacement::draft(0x14161fd60, "concurrency_14161fd60", fn_14161fd60 as *const ()), // sub_14161fd60
    Replacement::draft(0x14161ff50, "concurrency_14161ff50", fn_14161ff50 as *const ()), // sub_14161ff50
    Replacement::draft(0x1416203f0, "concurrency_1416203f0", fn_1416203f0 as *const ()), // sub_1416203f0
    Replacement::draft(0x1416204c0, "concurrency_1416204c0", fn_1416204c0 as *const ()), // sub_1416204c0
    Replacement::draft(0x1416208e0, "concurrency_1416208e0", fn_1416208e0 as *const ()), // sub_1416208e0
    Replacement::draft(0x141620a50, "concurrency_141620a50", fn_141620a50 as *const ()), // sub_141620a50
    Replacement::draft(0x141620b90, "concurrency_141620b90", fn_141620b90 as *const ()), // sub_141620b90
    Replacement::draft(0x141620bb0, "concurrency_141620bb0", fn_141620bb0 as *const ()), // sub_141620bb0
    Replacement::draft(0x141620c30, "concurrency_141620c30", fn_141620c30 as *const ()), // sub_141620c30
    Replacement::draft(0x141620d00, "concurrency_141620d00", tpl_28335::<0x10> as *const ()), // sub_141620d00
    Replacement::draft(0x141620d10, "concurrency_141620d10", fn_141620d10 as *const ()), // sub_141620d10
    Replacement::draft(0x141620d50, "concurrency_141620d50", fn_141620d50 as *const ()), // sub_141620d50
    Replacement::draft(0x141620e80, "concurrency_141620e80", fn_141620e80 as *const ()), // sub_141620e80
    Replacement::draft(0x141620ef0, "concurrency_141620ef0", tpl_28390::<0x1416491f0, 0x1426903f8, 0x142677588, 0x8, 0x142690678, 0x10, 0x1426906a0, 0x28, 0x108, 0x110> as *const ()), // sub_141620ef0
    Replacement::draft(0x141620f50, "concurrency_141620f50", fn_141620f50 as *const ()), // sub_141620f50
    Replacement::draft(0x141621010, "concurrency_141621010", fn_141621010 as *const ()), // sub_141621010
    Replacement::draft(0x141621060, "concurrency_141621060", tpl_29465::<0x1426903f8, 0x142677588, 0x8, 0x142690678, 0x10, 0x1426906a0, 0x28, 0x141649250> as *const ()), // sub_141621060
    Replacement::draft(0x141621090, "concurrency_141621090", tpl_28392::<0x1416210b0> as *const ()), // sub_141621090
    Replacement::draft(0x14162109c, "concurrency_14162109c", tpl_28393::<0x1416210b0> as *const ()), // sub_14162109c
    Replacement::draft(0x1416210b0, "concurrency_1416210b0", fn_1416210b0 as *const ()), // sub_1416210b0
    Replacement::draft(0x141621110, "unknown_141621110", fn_141621110 as *const ()), // sub_141621110
    Replacement::draft(0x141621270, "unknown_141621270", fn_141621270 as *const ()), // sub_141621270
    Replacement::draft(0x141621340, "unknown_141621340", fn_141621340 as *const ()), // sub_141621340
    Replacement::draft(0x141621390, "unknown_141621390", fn_141621390 as *const ()), // sub_141621390
    Replacement::draft(0x141621420, "unknown_141621420", fn_141621420 as *const ()), // sub_141621420
    Replacement::draft(0x1416214f0, "unknown_1416214f0", fn_1416214f0 as *const ()), // sub_1416214f0
    Replacement::draft(0x141621530, "unknown_141621530", fn_141621530 as *const ()), // sub_141621530
    Replacement::draft(0x141621850, "unknown_141621850", fn_141621850 as *const ()), // sub_141621850
    Replacement::draft(0x141621970, "unknown_141621970", tpl_29408::<0x108, 0x110, 0x1415242e0, 0x114> as *const ()), // sub_141621970
    Replacement::draft(0x1416219c0, "unknown_1416219c0", fn_1416219c0 as *const ()), // sub_1416219c0
    Replacement::draft(0x141621be0, "unknown_141621be0", fn_141621be0 as *const ()), // sub_141621be0
    Replacement::draft(0x141621e80, "unknown_141621e80", fn_141621e80 as *const ()), // sub_141621e80
    Replacement::draft(0x141622040, "unknown_141622040", fn_141622040 as *const ()), // sub_141622040
    Replacement::draft(0x1416221c0, "unknown_1416221c0", fn_1416221c0 as *const ()), // sub_1416221c0
    Replacement::draft(0x1416222e0, "unknown_1416222e0", fn_1416222e0 as *const ()), // sub_1416222e0
    Replacement::draft(0x141622350, "unknown_141622350", fn_141622350 as *const ()), // sub_141622350
    Replacement::draft(0x141622380, "unknown_141622380", fn_141622380 as *const ()), // sub_141622380
    Replacement::draft(0x1416227b0, "unknown_1416227b0", fn_1416227b0 as *const ()), // sub_1416227b0
    Replacement::draft(0x1416227d0, "unknown_1416227d0", fn_1416227d0 as *const ()), // sub_1416227d0
    Replacement::draft(0x1416229d0, "unknown_1416229d0", fn_1416229d0 as *const ()), // sub_1416229d0
    Replacement::draft(0x141622a80, "unknown_141622a80", fn_141622a80 as *const ()), // sub_141622a80
    Replacement::draft(0x141622c50, "unknown_141622c50", fn_141622c50 as *const ()), // sub_141622c50
    Replacement::draft(0x141622db0, "unknown_141622db0", fn_141622db0 as *const ()), // sub_141622db0
    Replacement::draft(0x141622df0, "unknown_141622df0", fn_141622df0 as *const ()), // sub_141622df0
    Replacement::draft(0x1416231d0, "unknown_1416231d0", fn_1416231d0 as *const ()), // sub_1416231d0
    Replacement::draft(0x141623260, "unknown_141623260", fn_141623260 as *const ()), // sub_141623260
    Replacement::draft(0x141623270, "unknown_141623270", fn_141623270 as *const ()), // sub_141623270
    Replacement::draft(0x141623370, "unknown_141623370", fn_141623370 as *const ()), // sub_141623370
    Replacement::draft(0x141623520, "unknown_141623520", fn_141623520 as *const ()), // sub_141623520
    Replacement::draft(0x141623820, "unknown_141623820", fn_141623820 as *const ()), // sub_141623820
    Replacement::draft(0x141623960, "unknown_141623960", fn_141623960 as *const ()), // sub_141623960
    Replacement::draft(0x141623a40, "unknown_141623a40", fn_141623a40 as *const ()), // sub_141623a40
    Replacement::draft(0x141623ad0, "unknown_141623ad0", fn_141623ad0 as *const ()), // sub_141623ad0
    Replacement::draft(0x141623b50, "unknown_141623b50", fn_141623b50 as *const ()), // sub_141623b50
    Replacement::draft(0x141623b90, "unknown_141623b90", fn_141623b90 as *const ()), // sub_141623b90
    Replacement::draft(0x141623d40, "unknown_141623d40", fn_141623d40 as *const ()), // sub_141623d40
    Replacement::draft(0x141623df0, "unknown_141623df0", fn_141623df0 as *const ()), // sub_141623df0
    Replacement::draft(0x141623f80, "unknown_141623f80", fn_141623f80 as *const ()), // sub_141623f80
    Replacement::draft(0x141624010, "unknown_141624010", tpl_27054::<0x141623f80, 0x141834d30> as *const ()), // sub_141624010
    Replacement::draft(0x141624050, "unknown_141624050", fn_141624050 as *const ()), // sub_141624050
    Replacement::draft(0x141624090, "unknown_141624090", fn_141624090 as *const ()), // sub_141624090
    Replacement::draft(0x1416240d0, "unknown_1416240d0", fn_1416240d0 as *const ()), // sub_1416240d0
    Replacement::draft(0x141624150, "unknown_141624150", fn_141624150 as *const ()), // sub_141624150
    Replacement::draft(0x141624190, "unknown_141624190", fn_141624190 as *const ()), // sub_141624190
    Replacement::draft(0x1416241e0, "unknown_1416241e0", fn_1416241e0 as *const ()), // sub_1416241e0
    Replacement::draft(0x141624210, "unknown_141624210", fn_141624210 as *const ()), // sub_141624210
    Replacement::draft(0x141624330, "unknown_141624330", fn_141624330 as *const ()), // sub_141624330
    Replacement::draft(0x141624380, "unknown_141624380", fn_141624380 as *const ()), // sub_141624380
    Replacement::draft(0x1416243c0, "unknown_1416243c0", fn_1416243c0 as *const ()), // sub_1416243c0
    Replacement::draft(0x141624420, "unknown_141624420", fn_141624420 as *const ()), // sub_141624420
    Replacement::draft(0x141624450, "unknown_141624450", fn_141624450 as *const ()), // sub_141624450
    Replacement::draft(0x141624560, "unknown_141624560", fn_141624560 as *const ()), // sub_141624560
    Replacement::draft(0x141624720, "unknown_141624720", fn_141624720 as *const ()), // sub_141624720
    Replacement::draft(0x141625380, "unknown_141625380", fn_141625380 as *const ()), // sub_141625380
    Replacement::draft(0x1416254d0, "unknown_1416254d0", fn_1416254d0 as *const ()), // sub_1416254d0
    Replacement::draft(0x1416255e0, "unknown_1416255e0", fn_1416255e0 as *const ()), // sub_1416255e0
    Replacement::draft(0x141625700, "unknown_141625700", fn_141625700 as *const ()), // sub_141625700
    Replacement::draft(0x141625720, "unknown_141625720", fn_141625720 as *const ()), // sub_141625720
    Replacement::draft(0x1416258c0, "unknown_1416258c0", fn_1416258c0 as *const ()), // sub_1416258c0
    Replacement::draft(0x141625950, "unknown_141625950", fn_141625950 as *const ()), // sub_141625950
    Replacement::draft(0x1416259a0, "unknown_1416259a0", fn_1416259a0 as *const ()), // sub_1416259a0
    Replacement::draft(0x141625a40, "unknown_141625a40", fn_141625a40 as *const ()), // sub_141625a40
    Replacement::draft(0x141625b60, "unknown_141625b60", fn_141625b60 as *const ()), // sub_141625b60
    Replacement::draft(0x141625ba0, "unknown_141625ba0", fn_141625ba0 as *const ()), // sub_141625ba0
    Replacement::draft(0x141625c50, "unknown_141625c50", fn_141625c50 as *const ()), // sub_141625c50
    Replacement::draft(0x141625d70, "unknown_141625d70", fn_141625d70 as *const ()), // sub_141625d70
    Replacement::draft(0x141625ef0, "unknown_141625ef0", fn_141625ef0 as *const ()), // sub_141625ef0
    Replacement::draft(0x141626080, "unknown_141626080", fn_141626080 as *const ()), // sub_141626080
    Replacement::draft(0x141626220, "unknown_141626220", fn_141626220 as *const ()), // sub_141626220
    Replacement::draft(0x1416264d0, "unknown_1416264d0", fn_1416264d0 as *const ()), // sub_1416264d0
    Replacement::draft(0x141626720, "unknown_141626720", fn_141626720 as *const ()), // sub_141626720
    Replacement::draft(0x141626910, "unknown_141626910", fn_141626910 as *const ()), // sub_141626910
    Replacement::draft(0x141626c10, "unknown_141626c10", fn_141626c10 as *const ()), // sub_141626c10
    Replacement::draft(0x141627040, "unknown_141627040", fn_141627040 as *const ()), // sub_141627040
    Replacement::draft(0x141627430, "unknown_141627430", fn_141627430 as *const ()), // sub_141627430
    Replacement::draft(0x141627790, "unknown_141627790", fn_141627790 as *const ()), // sub_141627790
    Replacement::draft(0x1416279b0, "unknown_1416279b0", fn_1416279b0 as *const ()), // sub_1416279b0
    Replacement::draft(0x141627a20, "unknown_141627a20", fn_141627a20 as *const ()), // sub_141627a20
    Replacement::draft(0x141627ae0, "unknown_141627ae0", fn_141627ae0 as *const ()), // sub_141627ae0
    Replacement::draft(0x141627b60, "unknown_141627b60", fn_141627b60 as *const ()), // sub_141627b60
    Replacement::draft(0x141627be0, "unknown_141627be0", fn_141627be0 as *const ()), // sub_141627be0
    Replacement::draft(0x141627c60, "unknown_141627c60", fn_141627c60 as *const ()), // sub_141627c60
    Replacement::draft(0x141627ce0, "unknown_141627ce0", fn_141627ce0 as *const ()), // sub_141627ce0
    Replacement::draft(0x141627d70, "unknown_141627d70", fn_141627d70 as *const ()), // sub_141627d70
    Replacement::draft(0x141627de0, "unknown_141627de0", fn_141627de0 as *const ()), // sub_141627de0
    Replacement::draft(0x141627e90, "unknown_141627e90", fn_141627e90 as *const ()), // sub_141627e90
    Replacement::draft(0x141627f00, "unknown_141627f00", fn_141627f00 as *const ()), // sub_141627f00
    Replacement::draft(0x141627f90, "unknown_141627f90", fn_141627f90 as *const ()), // sub_141627f90
    Replacement::draft(0x141628020, "unknown_141628020", fn_141628020 as *const ()), // sub_141628020
    Replacement::draft(0x1416280b0, "unknown_1416280b0", fn_1416280b0 as *const ()), // sub_1416280b0
    Replacement::draft(0x141628140, "unknown_141628140", fn_141628140 as *const ()), // sub_141628140
    Replacement::draft(0x141628280, "unknown_141628280", fn_141628280 as *const ()), // sub_141628280
    Replacement::draft(0x1416283d0, "unknown_1416283d0", fn_1416283d0 as *const ()), // sub_1416283d0
    Replacement::draft(0x141628590, "unknown_141628590", fn_141628590 as *const ()), // sub_141628590
    Replacement::draft(0x1416285e0, "unknown_1416285e0", fn_1416285e0 as *const ()), // sub_1416285e0
    Replacement::draft(0x141628690, "unknown_141628690", fn_141628690 as *const ()), // sub_141628690
    Replacement::draft(0x141628740, "unknown_141628740", fn_141628740 as *const ()), // sub_141628740
    Replacement::draft(0x1416287e0, "unknown_1416287e0", fn_1416287e0 as *const ()), // sub_1416287e0
    Replacement::draft(0x141628950, "unknown_141628950", fn_141628950 as *const ()), // sub_141628950
    Replacement::draft(0x141628a50, "unknown_141628a50", fn_141628a50 as *const ()), // sub_141628a50
    Replacement::draft(0x141628bd0, "unknown_141628bd0", fn_141628bd0 as *const ()), // sub_141628bd0
    Replacement::draft(0x141628d50, "unknown_141628d50", fn_141628d50 as *const ()), // sub_141628d50
    Replacement::draft(0x141628f50, "unknown_141628f50", fn_141628f50 as *const ()), // sub_141628f50
    Replacement::draft(0x141628fd0, "unknown_141628fd0", fn_141628fd0 as *const ()), // sub_141628fd0
    Replacement::draft(0x141629020, "unknown_141629020", fn_141629020 as *const ()), // sub_141629020
    Replacement::draft(0x1416290f0, "unknown_1416290f0", tpl_89668::<0x141524340, 0x141524330, 0x141627ae0, 0x1415242e0> as *const ()), // sub_1416290f0
    Replacement::draft(0x141629200, "unknown_141629200", fn_141629200 as *const ()), // sub_141629200
    Replacement::draft(0x141629310, "unknown_141629310", tpl_89668::<0x141524340, 0x141524330, 0x141627be0, 0x1415242e0> as *const ()), // sub_141629310
    Replacement::draft(0x141629420, "unknown_141629420", fn_141629420 as *const ()), // sub_141629420
    Replacement::draft(0x141629530, "unknown_141629530", fn_141629530 as *const ()), // sub_141629530
    Replacement::draft(0x141629660, "unknown_141629660", fn_141629660 as *const ()), // sub_141629660
    Replacement::draft(0x1416296e0, "unknown_1416296e0", fn_1416296e0 as *const ()), // sub_1416296e0
    Replacement::draft(0x141629740, "unknown_141629740", fn_141629740 as *const ()), // sub_141629740
    Replacement::draft(0x1416298e0, "unknown_1416298e0", fn_1416298e0 as *const ()), // sub_1416298e0
    Replacement::draft(0x141629a40, "unknown_141629a40", fn_141629a40 as *const ()), // sub_141629a40
    Replacement::draft(0x141629ba0, "unknown_141629ba0", fn_141629ba0 as *const ()), // sub_141629ba0
    Replacement::draft(0x141629ce0, "unknown_141629ce0", tpl_89678::<0x141627f00, 0x1416290f0, 0x14183acc8> as *const ()), // sub_141629ce0
    Replacement::draft(0x141629e30, "unknown_141629e30", tpl_89678::<0x141627f00, 0x141629310, 0x14183acc8> as *const ()), // sub_141629e30
    Replacement::draft(0x141629f80, "unknown_141629f80", fn_141629f80 as *const ()), // sub_141629f80
    Replacement::draft(0x14162a0c0, "unknown_14162a0c0", fn_14162a0c0 as *const ()), // sub_14162a0c0
    Replacement::draft(0x14162a1d0, "unknown_14162a1d0", tpl_30667::<0x8> as *const ()), // sub_14162a1d0
    Replacement::draft(0x14162a1e0, "unknown_14162a1e0", fn_14162a1e0 as *const ()), // sub_14162a1e0
    Replacement::draft(0x14162a420, "unknown_14162a420", fn_14162a420 as *const ()), // sub_14162a420
    Replacement::draft(0x14162a660, "unknown_14162a660", fn_14162a660 as *const ()), // sub_14162a660
    Replacement::draft(0x14162a6a0, "unknown_14162a6a0", fn_14162a6a0 as *const ()), // sub_14162a6a0
    Replacement::draft(0x14162a710, "unknown_14162a710", fn_14162a710 as *const ()), // sub_14162a710
    Replacement::draft(0x14162a750, "unknown_14162a750", fn_14162a750 as *const ()), // sub_14162a750
    Replacement::draft(0x14162a870, "unknown_14162a870", fn_14162a870 as *const ()), // sub_14162a870
    Replacement::draft(0x14162a920, "unknown_14162a920", fn_14162a920 as *const ()), // sub_14162a920
    Replacement::draft(0x14162ab40, "unknown_14162ab40", fn_14162ab40 as *const ()), // sub_14162ab40
    Replacement::draft(0x14162acc0, "unknown_14162acc0", fn_14162acc0 as *const ()), // sub_14162acc0
    Replacement::draft(0x14162ae30, "unknown_14162ae30", fn_14162ae30 as *const ()), // sub_14162ae30
    Replacement::draft(0x14162aeb0, "unknown_14162aeb0", fn_14162aeb0 as *const ()), // sub_14162aeb0
    Replacement::draft(0x14162af40, "unknown_14162af40", fn_14162af40 as *const ()), // sub_14162af40
    Replacement::draft(0x14162b060, "unknown_14162b060", fn_14162b060 as *const ()), // sub_14162b060
    Replacement::draft(0x14162b200, "unknown_14162b200", tpl_30674::<0x1416285e0, 0x8, 0x18, 0x14183acc8> as *const ()), // sub_14162b200
    Replacement::draft(0x14162b270, "unknown_14162b270", tpl_30674::<0x141628690, 0x8, 0x18, 0x14183acc8> as *const ()), // sub_14162b270
    Replacement::draft(0x14162b2e0, "unknown_14162b2e0", fn_14162b2e0 as *const ()), // sub_14162b2e0
    Replacement::draft(0x14162b330, "unknown_14162b330", fn_14162b330 as *const ()), // sub_14162b330
    Replacement::draft(0x14162b390, "unknown_14162b390", fn_14162b390 as *const ()), // sub_14162b390
    Replacement::draft(0x14162b440, "unknown_14162b440", fn_14162b440 as *const ()), // sub_14162b440
    Replacement::draft(0x14162b4b0, "unknown_14162b4b0", fn_14162b4b0 as *const ()), // sub_14162b4b0
    Replacement::draft(0x14162b500, "unknown_14162b500", tpl_1674::<0x142690888> as *const ()), // sub_14162b500
    Replacement::draft(0x14162b540, "unknown_14162b540", fn_14162b540 as *const ()), // sub_14162b540
    Replacement::draft(0x14162b590, "unknown_14162b590", fn_14162b590 as *const ()), // sub_14162b590
    Replacement::draft(0x14162b5b0, "unknown_14162b5b0", fn_14162b5b0 as *const ()), // sub_14162b5b0
    Replacement::draft(0x14162b6c0, "unknown_14162b6c0", fn_14162b6c0 as *const ()), // sub_14162b6c0
    Replacement::draft(0x14162b890, "unknown_14162b890", fn_14162b890 as *const ()), // sub_14162b890
    Replacement::draft(0x14162b8e0, "unknown_14162b8e0", fn_14162b8e0 as *const ()), // sub_14162b8e0
    Replacement::draft(0x14162b9f0, "unknown_14162b9f0", fn_14162b9f0 as *const ()), // sub_14162b9f0
    Replacement::draft(0x14162bc20, "unknown_14162bc20", fn_14162bc20 as *const ()), // sub_14162bc20
    Replacement::draft(0x14162bc50, "unknown_14162bc50", fn_14162bc50 as *const ()), // sub_14162bc50
    Replacement::draft(0x14162bd60, "unknown_14162bd60", fn_14162bd60 as *const ()), // sub_14162bd60
    Replacement::draft(0x14162beb0, "unknown_14162beb0", fn_14162beb0 as *const ()), // sub_14162beb0
    Replacement::draft(0x14162bf60, "unknown_14162bf60", fn_14162bf60 as *const ()), // sub_14162bf60
    Replacement::draft(0x14162bf80, "unknown_14162bf80", fn_14162bf80 as *const ()), // sub_14162bf80
    Replacement::draft(0x14162bfc0, "unknown_14162bfc0", tpl_26762::<0x14160f4a0, 0x142690898> as *const ()), // sub_14162bfc0
    Replacement::draft(0x14162bff0, "unknown_14162bff0", tpl_30692::<0x142690898, 0x28, 0x1415242e0, 0x20, 0x1415bd160, 0x141834d30> as *const ()), // sub_14162bff0
    Replacement::draft(0x14162c070, "unknown_14162c070", fn_14162c070 as *const ()), // sub_14162c070
    Replacement::draft(0x14162c0b0, "unknown_14162c0b0", fn_14162c0b0 as *const ()), // sub_14162c0b0
    Replacement::draft(0x14162c0d0, "unknown_14162c0d0", fn_14162c0d0 as *const ()), // sub_14162c0d0
    Replacement::draft(0x14162c110, "unknown_14162c110", fn_14162c110 as *const ()), // sub_14162c110
    Replacement::draft(0x14162c1f0, "unknown_14162c1f0", fn_14162c1f0 as *const ()), // sub_14162c1f0
    Replacement::draft(0x14162c200, "unknown_14162c200", fn_14162c200 as *const ()), // sub_14162c200
    Replacement::draft(0x14162c240, "unknown_14162c240", fn_14162c240 as *const ()), // sub_14162c240
    Replacement::draft(0x14162c460, "unknown_14162c460", fn_14162c460 as *const ()), // sub_14162c460
    Replacement::draft(0x14162c500, "unknown_14162c500", fn_14162c500 as *const ()), // sub_14162c500
    Replacement::draft(0x14162c560, "unknown_14162c560", fn_14162c560 as *const ()), // sub_14162c560
    Replacement::draft(0x14162c5f0, "unknown_14162c5f0", fn_14162c5f0 as *const ()), // sub_14162c5f0
    Replacement::draft(0x14162c700, "unknown_14162c700", fn_14162c700 as *const ()), // sub_14162c700
    Replacement::draft(0x14162c790, "unknown_14162c790", fn_14162c790 as *const ()), // sub_14162c790
    Replacement::draft(0x14162c820, "unknown_14162c820", fn_14162c820 as *const ()), // sub_14162c820
    Replacement::draft(0x14162c8a0, "unknown_14162c8a0", fn_14162c8a0 as *const ()), // sub_14162c8a0
    Replacement::draft(0x14162c988, "unknown_14162c988", tpl_28393::<0x14162c9b0> as *const ()), // sub_14162c988
    Replacement::draft(0x14162c994, "unknown_14162c994", fn_14162c994 as *const ()), // sub_14162c994
    Replacement::draft(0x14162c9a0, "unknown_14162c9a0", fn_14162c9a0 as *const ()), // sub_14162c9a0
    Replacement::draft(0x14162c9b0, "unknown_14162c9b0", fn_14162c9b0 as *const ()), // sub_14162c9b0
    Replacement::draft(0x14162ca80, "opentelemetry_14162ca80", tpl_27686::<0x14158fa60, 0x141834d30> as *const ()), // sub_14162ca80
    Replacement::draft(0x14162cac0, "unknown_14162cac0", fn_14162cac0 as *const ()), // sub_14162cac0
    Replacement::draft(0x14162cad0, "unknown_14162cad0", fn_14162cad0 as *const ()), // sub_14162cad0
    Replacement::draft(0x14162cb80, "unknown_14162cb80", tpl_27317::<0xffffffffffffffe0, 0x141593cb0> as *const ()), // sub_14162cb80
    Replacement::draft(0x14162cb90, "unknown_14162cb90", tpl_30707::<0x14158ead0> as *const ()), // sub_14162cb90
    Replacement::draft(0x14162cba0, "unknown_14162cba0", fn_14162cba0 as *const ()), // sub_14162cba0
    Replacement::draft(0x14162cc50, "unknown_14162cc50", tpl_30709::<0xfffffffffffffff0, 0x141591570> as *const ()), // sub_14162cc50
    Replacement::draft(0x14162cc60, "unknown_14162cc60", fn_14162cc60 as *const ()), // sub_14162cc60
    Replacement::draft(0x14162cd70, "unknown_14162cd70", fn_14162cd70 as *const ()), // sub_14162cd70
    Replacement::draft(0x14162cda0, "unknown_14162cda0", tpl_30709::<0xffffffffffffffe8, 0x141590fd0> as *const ()), // sub_14162cda0
    Replacement::draft(0x14162cdb0, "unknown_14162cdb0", tpl_29846::<0xf0, 0x38> as *const ()), // sub_14162cdb0
    Replacement::draft(0x14162cdd0, "unknown_14162cdd0", tpl_30709::<0xffffffffffffffe8, 0x141591060> as *const ()), // sub_14162cdd0
    Replacement::draft(0x14162cde0, "unknown_14162cde0", tpl_28467::<0xffffffffffffffe8, 0x20, 0x58> as *const ()), // sub_14162cde0
    Replacement::draft(0x14162cdf0, "unknown_14162cdf0", tpl_30711::<0xffffffffffffffd8, 0x8> as *const ()), // sub_14162cdf0
    Replacement::draft(0x14162ce10, "unknown_14162ce10", fn_14162ce10 as *const ()), // sub_14162ce10
    Replacement::draft(0x14162ce30, "unknown_14162ce30", tpl_30709::<0xffffffffffffffe8, 0x141591100> as *const ()), // sub_14162ce30
    Replacement::draft(0x14162ce40, "unknown_14162ce40", fn_14162ce40 as *const ()), // sub_14162ce40
    Replacement::draft(0x14162ce60, "unknown_14162ce60", tpl_30709::<0xffffffffffffffe8, 0x141591230> as *const ()), // sub_14162ce60
    Replacement::draft(0x14162ce70, "unknown_14162ce70", fn_14162ce70 as *const ()), // sub_14162ce70
    Replacement::draft(0x14162cef0, "unknown_14162cef0", fn_14162cef0 as *const ()), // sub_14162cef0
    Replacement::draft(0x14162cf10, "unknown_14162cf10", tpl_30716::<0xffffffffffffffe8, 0x20, 0x141605370> as *const ()), // sub_14162cf10
    Replacement::draft(0x14162cf20, "unknown_14162cf20", fn_14162cf20 as *const ()), // sub_14162cf20
    Replacement::draft(0x14162cf40, "unknown_14162cf40", fn_14162cf40 as *const ()), // sub_14162cf40
    Replacement::draft(0x14162cf50, "unknown_14162cf50", fn_14162cf50 as *const ()), // sub_14162cf50
    Replacement::draft(0x14162cfb0, "unknown_14162cfb0", fn_14162cfb0 as *const ()), // sub_14162cfb0
    Replacement::draft(0x14162cfc0, "unknown_14162cfc0", tpl_660::<0xffffffffffffffe0, 0x270> as *const ()), // sub_14162cfc0
    Replacement::draft(0x14162cfd0, "unknown_14162cfd0", fn_14162cfd0 as *const ()), // sub_14162cfd0
    Replacement::draft(0x14162d1c0, "unknown_14162d1c0", fn_14162d1c0 as *const ()), // sub_14162d1c0
    Replacement::draft(0x14162d1d0, "unknown_14162d1d0", fn_14162d1d0 as *const ()), // sub_14162d1d0
    Replacement::draft(0x14162d200, "unknown_14162d200", tpl_30711::<0xffffffffffffffd8, 0x10> as *const ()), // sub_14162d200
    Replacement::draft(0x14162d220, "unknown_14162d220", fn_14162d220 as *const ()), // sub_14162d220
    Replacement::draft(0x14162d2b0, "unknown_14162d2b0", fn_14162d2b0 as *const ()), // sub_14162d2b0
    Replacement::draft(0x14162d340, "unknown_14162d340", fn_14162d340 as *const ()), // sub_14162d340
    Replacement::draft(0x14162d370, "unknown_14162d370", fn_14162d370 as *const ()), // sub_14162d370
    Replacement::draft(0x14162d3e0, "unknown_14162d3e0", fn_14162d3e0 as *const ()), // sub_14162d3e0
    Replacement::draft(0x14162d680, "unknown_14162d680", tpl_29855::<0xf0, 0x48> as *const ()), // sub_14162d680
    Replacement::draft(0x14162d6a0, "unknown_14162d6a0", fn_14162d6a0 as *const ()), // sub_14162d6a0
    Replacement::draft(0x14162d6c0, "unknown_14162d6c0", tpl_30403::<0x90, 0x143815b50, 0x8, 0x14160a090> as *const ()), // sub_14162d6c0
    Replacement::draft(0x14162d700, "unknown_14162d700", fn_14162d700 as *const ()), // sub_14162d700
    Replacement::draft(0x14162d750, "unknown_14162d750", fn_14162d750 as *const ()), // sub_14162d750
    Replacement::draft(0x14162d770, "unknown_14162d770", fn_14162d770 as *const ()), // sub_14162d770
    Replacement::draft(0x14162d7b0, "unknown_14162d7b0", fn_14162d7b0 as *const ()), // sub_14162d7b0
    Replacement::draft(0x14162d830, "unknown_14162d830", fn_14162d830 as *const ()), // sub_14162d830
    Replacement::draft(0x14162d840, "unknown_14162d840", fn_14162d840 as *const ()), // sub_14162d840
    Replacement::draft(0x14162d870, "unknown_14162d870", fn_14162d870 as *const ()), // sub_14162d870
    Replacement::draft(0x14162dd50, "unknown_14162dd50", fn_14162dd50 as *const ()), // sub_14162dd50
    Replacement::draft(0x14162dfe0, "unknown_14162dfe0", fn_14162dfe0 as *const ()), // sub_14162dfe0
    Replacement::draft(0x14162e020, "unknown_14162e020", fn_14162e020 as *const ()), // sub_14162e020
    Replacement::draft(0x14162e050, "unknown_14162e050", fn_14162e050 as *const ()), // sub_14162e050
    Replacement::draft(0x14162e180, "unknown_14162e180", fn_14162e180 as *const ()), // sub_14162e180
    Replacement::draft(0x14162e1f0, "unknown_14162e1f0", fn_14162e1f0 as *const ()), // sub_14162e1f0
    Replacement::draft(0x14162e3a0, "unknown_14162e3a0", tpl_893::<0x142690c78, 0x141834d30> as *const ()), // sub_14162e3a0
    Replacement::draft(0x14162e3d0, "unknown_14162e3d0", fn_14162e3d0 as *const ()), // sub_14162e3d0
    Replacement::draft(0x14162e500, "unknown_14162e500", fn_14162e500 as *const ()), // sub_14162e500
    Replacement::draft(0x14162e630, "unknown_14162e630", fn_14162e630 as *const ()), // sub_14162e630
    Replacement::draft(0x14162e7a0, "unknown_14162e7a0", fn_14162e7a0 as *const ()), // sub_14162e7a0
    Replacement::draft(0x14162eab0, "unknown_14162eab0", fn_14162eab0 as *const ()), // sub_14162eab0
    Replacement::draft(0x14162ebb0, "unknown_14162ebb0", tpl_30741::<0x150, 0x8, 0x18c> as *const ()), // sub_14162ebb0
    Replacement::draft(0x14162ebd0, "unknown_14162ebd0", fn_14162ebd0 as *const ()), // sub_14162ebd0
    Replacement::draft(0x14162ebf0, "unknown_14162ebf0", fn_14162ebf0 as *const ()), // sub_14162ebf0
    Replacement::draft(0x14162ec10, "unknown_14162ec10", tpl_1691::<0x194> as *const ()), // sub_14162ec10
    Replacement::draft(0x14162ec20, "unknown_14162ec20", tpl_89773::<0xffffffffffffffe8, 0x8, 0x10, 0x18, 0x170, 0xffffffffffffffc8, 0xffffffffffffffd8, 0xffffffffffffffe0, 0x14162e3d0, 0x150, 0x1415914d0, 0x1415d7950, 0x1415d7a40, 0x1415907e0, 0x1434264e4, 0x143426abc, 0x1415242e0> as *const ()), // sub_14162ec20
    Replacement::draft(0x14162ed10, "unknown_14162ed10", tpl_913::<0x150> as *const ()), // sub_14162ed10
    Replacement::draft(0x14162ed20, "unknown_14162ed20", fn_14162ed20 as *const ()), // sub_14162ed20
    Replacement::draft(0x14162edc0, "unknown_14162edc0", fn_14162edc0 as *const ()), // sub_14162edc0
    Replacement::draft(0x14162ede0, "unknown_14162ede0", fn_14162ede0 as *const ()), // sub_14162ede0
    Replacement::draft(0x14162ee70, "unknown_14162ee70", tpl_913::<0x158> as *const ()), // sub_14162ee70
    Replacement::draft(0x14162ee80, "unknown_14162ee80", fn_14162ee80 as *const ()), // sub_14162ee80
    Replacement::draft(0x14162ee90, "unknown_14162ee90", fn_14162ee90 as *const ()), // sub_14162ee90
    Replacement::draft(0x14162eeb0, "unknown_14162eeb0", fn_14162eeb0 as *const ()), // sub_14162eeb0
    Replacement::draft(0x14162ef40, "unknown_14162ef40", fn_14162ef40 as *const ()), // sub_14162ef40
    Replacement::draft(0x14162f030, "unknown_14162f030", tpl_30741::<0x150, 0x8, 0x160> as *const ()), // sub_14162f030
    Replacement::draft(0x14162f050, "unknown_14162f050", tpl_89773::<0xffffffffffffffe8, 0x8, 0x10, 0x18, 0x170, 0xffffffffffffffc8, 0xffffffffffffffd8, 0xffffffffffffffe0, 0x14162e3d0, 0x150, 0x1415914d0, 0x1415d7950, 0x1415d7a40, 0x141590880, 0x1434264e4, 0x143426abc, 0x1415242e0> as *const ()), // sub_14162f050
    Replacement::draft(0x14162f140, "unknown_14162f140", tpl_913::<0x168> as *const ()), // sub_14162f140
    Replacement::draft(0x14162f150, "unknown_14162f150", tpl_27067::<0x150, 0x58> as *const ()), // sub_14162f150
    Replacement::draft(0x14162f160, "unknown_14162f160", fn_14162f160 as *const ()), // sub_14162f160
    Replacement::draft(0x14162f2c0, "unknown_14162f2c0", fn_14162f2c0 as *const ()), // sub_14162f2c0
    Replacement::draft(0x14162f2e0, "unknown_14162f2e0", tpl_1738::<0x168> as *const ()), // sub_14162f2e0
    Replacement::draft(0x14162f2f0, "unknown_14162f2f0", fn_14162f2f0 as *const ()), // sub_14162f2f0
    Replacement::draft(0x14162f410, "unknown_14162f410", fn_14162f410 as *const ()), // sub_14162f410
    Replacement::draft(0x14162f510, "unknown_14162f510", fn_14162f510 as *const ()), // sub_14162f510
    Replacement::draft(0x14162f730, "unknown_14162f730", fn_14162f730 as *const ()), // sub_14162f730
    Replacement::draft(0x14162f910, "unknown_14162f910", fn_14162f910 as *const ()), // sub_14162f910
    Replacement::draft(0x14162fb20, "unknown_14162fb20", fn_14162fb20 as *const ()), // sub_14162fb20
    Replacement::draft(0x14162fdf0, "unknown_14162fdf0", fn_14162fdf0 as *const ()), // sub_14162fdf0
    Replacement::draft(0x141630060, "unknown_141630060", fn_141630060 as *const ()), // sub_141630060
    Replacement::draft(0x141630640, "unknown_141630640", fn_141630640 as *const ()), // sub_141630640
    Replacement::draft(0x141630b60, "unknown_141630b60", fn_141630b60 as *const ()), // sub_141630b60
    Replacement::draft(0x141630b90, "luau_141630b90", fn_141630b90 as *const ()), // sub_141630b90
    Replacement::draft(0x141630e40, "unknown_141630e40", fn_141630e40 as *const ()), // sub_141630e40
    Replacement::draft(0x141631060, "unknown_141631060", fn_141631060 as *const ()), // sub_141631060
    Replacement::draft(0x141631140, "unknown_141631140", fn_141631140 as *const ()), // sub_141631140
    Replacement::draft(0x141631440, "unknown_141631440", fn_141631440 as *const ()), // sub_141631440
    Replacement::draft(0x141631760, "unknown_141631760", fn_141631760 as *const ()), // sub_141631760
    Replacement::draft(0x141631840, "unknown_141631840", fn_141631840 as *const ()), // sub_141631840
    Replacement::draft(0x141632050, "unknown_141632050", fn_141632050 as *const ()), // sub_141632050
    Replacement::draft(0x141632150, "unknown_141632150", fn_141632150 as *const ()), // sub_141632150
    Replacement::draft(0x141632370, "unknown_141632370", fn_141632370 as *const ()), // sub_141632370
    Replacement::draft(0x141632390, "ak_141632390", fn_141632390 as *const ()), // sub_141632390
    Replacement::draft(0x141632490, "ak_141632490", fn_141632490 as *const ()), // sub_141632490
    Replacement::draft(0x1416325a0, "ak_1416325a0", fn_1416325a0 as *const ()), // sub_1416325a0
    Replacement::draft(0x141632760, "ak_141632760", fn_141632760 as *const ()), // sub_141632760
    Replacement::draft(0x141632970, "ak_141632970", fn_141632970 as *const ()), // sub_141632970
    Replacement::draft(0x141632a70, "ak_141632a70", fn_141632a70 as *const ()), // sub_141632a70
    Replacement::draft(0x141632cf0, "ak_141632cf0", fn_141632cf0 as *const ()), // sub_141632cf0
    Replacement::draft(0x141632f70, "ak_141632f70", fn_141632f70 as *const ()), // sub_141632f70
    Replacement::draft(0x1416331b0, "ak_1416331b0", fn_1416331b0 as *const ()), // sub_1416331b0
    Replacement::draft(0x141633590, "ak_141633590", fn_141633590 as *const ()), // sub_141633590
    Replacement::draft(0x141633c00, "unknown_141633c00", fn_141633c00 as *const ()), // sub_141633c00
    Replacement::draft(0x141633c80, "unknown_141633c80", fn_141633c80 as *const ()), // sub_141633c80
    Replacement::draft(0x141633cb0, "unknown_141633cb0", fn_141633cb0 as *const ()), // sub_141633cb0
    Replacement::draft(0x141633d10, "unknown_141633d10", fn_141633d10 as *const ()), // sub_141633d10
    Replacement::draft(0x141633d20, "unknown_141633d20", tpl_30771::<0x8, 0x10> as *const ()), // sub_141633d20
    Replacement::draft(0x141633d40, "unknown_141633d40", fn_141633d40 as *const ()), // sub_141633d40
    Replacement::draft(0x141633dc0, "unknown_141633dc0", fn_141633dc0 as *const ()), // sub_141633dc0
    Replacement::draft(0x141633e70, "unknown_141633e70", fn_141633e70 as *const ()), // sub_141633e70
    Replacement::draft(0x141634020, "unknown_141634020", tpl_29425::<0x10, 0x58> as *const ()), // sub_141634020
    Replacement::draft(0x141634040, "unknown_141634040", fn_141634040 as *const ()), // sub_141634040
    Replacement::draft(0x141634070, "unknown_141634070", fn_141634070 as *const ()), // sub_141634070
    Replacement::draft(0x1416340d0, "unknown_1416340d0", fn_1416340d0 as *const ()), // sub_1416340d0
    Replacement::draft(0x141634110, "unknown_141634110", fn_141634110 as *const ()), // sub_141634110
    Replacement::draft(0x141634170, "unknown_141634170", fn_141634170 as *const ()), // sub_141634170
    Replacement::draft(0x141634190, "unknown_141634190", fn_141634190 as *const ()), // sub_141634190
    Replacement::draft(0x1416341b0, "unknown_1416341b0", tpl_30780::<0x20> as *const ()), // sub_1416341b0
    Replacement::draft(0x1416341c0, "unknown_1416341c0", fn_1416341c0 as *const ()), // sub_1416341c0
    Replacement::draft(0x1416341f0, "unknown_1416341f0", fn_1416341f0 as *const ()), // sub_1416341f0
    Replacement::draft(0x141634230, "unknown_141634230", fn_141634230 as *const ()), // sub_141634230
    Replacement::draft(0x141634250, "unknown_141634250", fn_141634250 as *const ()), // sub_141634250
    Replacement::draft(0x141634310, "unknown_141634310", fn_141634310 as *const ()), // sub_141634310
    Replacement::draft(0x1416344c0, "ak_1416344c0", fn_1416344c0 as *const ()), // sub_1416344c0
    Replacement::draft(0x1416345c0, "unknown_1416345c0", fn_1416345c0 as *const ()), // sub_1416345c0
    Replacement::draft(0x141634c40, "unknown_141634c40", fn_141634c40 as *const ()), // sub_141634c40
    Replacement::draft(0x141634d40, "unknown_141634d40", fn_141634d40 as *const ()), // sub_141634d40
    Replacement::draft(0x141634dd0, "unknown_141634dd0", fn_141634dd0 as *const ()), // sub_141634dd0
    Replacement::draft(0x141634e90, "unknown_141634e90", fn_141634e90 as *const ()), // sub_141634e90
    Replacement::draft(0x141634f20, "unknown_141634f20", fn_141634f20 as *const ()), // sub_141634f20
    Replacement::draft(0x141634f50, "unknown_141634f50", fn_141634f50 as *const ()), // sub_141634f50
    Replacement::draft(0x141634fd0, "unknown_141634fd0", fn_141634fd0 as *const ()), // sub_141634fd0
    Replacement::draft(0x141635060, "unknown_141635060", fn_141635060 as *const ()), // sub_141635060
    Replacement::draft(0x1416352a0, "unknown_1416352a0", fn_1416352a0 as *const ()), // sub_1416352a0
    Replacement::draft(0x1416352f0, "unknown_1416352f0", fn_1416352f0 as *const ()), // sub_1416352f0
    Replacement::draft(0x141635510, "unknown_141635510", fn_141635510 as *const ()), // sub_141635510
    Replacement::draft(0x141635540, "unknown_141635540", fn_141635540 as *const ()), // sub_141635540
    Replacement::draft(0x141635690, "unknown_141635690", fn_141635690 as *const ()), // sub_141635690
    Replacement::draft(0x141635750, "unknown_141635750", tpl_30799::<0x141635840> as *const ()), // sub_141635750
    Replacement::draft(0x14163575c, "unknown_14163575c", tpl_30799::<0x1416358d0> as *const ()), // sub_14163575c
    Replacement::draft(0x141635770, "unknown_141635770", fn_141635770 as *const ()), // sub_141635770
    Replacement::draft(0x141635840, "concurrency_141635840", tpl_27058::<0x141635540, 0x141834d30> as *const ()), // sub_141635840
    Replacement::draft(0x141635880, "unknown_141635880", fn_141635880 as *const ()), // sub_141635880
    Replacement::draft(0x1416358d0, "unknown_1416358d0", fn_1416358d0 as *const ()), // sub_1416358d0
    Replacement::draft(0x1416359c0, "unknown_1416359c0", fn_1416359c0 as *const ()), // sub_1416359c0
    Replacement::draft(0x141635a00, "unknown_141635a00", fn_141635a00 as *const ()), // sub_141635a00
    Replacement::draft(0x141635b80, "unknown_141635b80", fn_141635b80 as *const ()), // sub_141635b80
    Replacement::draft(0x141635be0, "unknown_141635be0", fn_141635be0 as *const ()), // sub_141635be0
    Replacement::draft(0x141635e90, "unknown_141635e90", fn_141635e90 as *const ()), // sub_141635e90
    Replacement::draft(0x1416361f0, "unknown_1416361f0", fn_1416361f0 as *const ()), // sub_1416361f0
    Replacement::draft(0x141636370, "unknown_141636370", fn_141636370 as *const ()), // sub_141636370
    Replacement::draft(0x141636450, "unknown_141636450", fn_141636450 as *const ()), // sub_141636450
    Replacement::draft(0x141636600, "unknown_141636600", fn_141636600 as *const ()), // sub_141636600
    Replacement::draft(0x141636660, "unknown_141636660", fn_141636660 as *const ()), // sub_141636660
    Replacement::draft(0x1416366b0, "unknown_1416366b0", fn_1416366b0 as *const ()), // sub_1416366b0
    Replacement::draft(0x141636700, "unknown_141636700", fn_141636700 as *const ()), // sub_141636700
    Replacement::draft(0x141636820, "unknown_141636820", fn_141636820 as *const ()), // sub_141636820
    Replacement::draft(0x141636850, "unknown_141636850", fn_141636850 as *const ()), // sub_141636850
    Replacement::draft(0x1416368c0, "unknown_1416368c0", fn_1416368c0 as *const ()), // sub_1416368c0
    Replacement::draft(0x141636bb0, "unknown_141636bb0", fn_141636bb0 as *const ()), // sub_141636bb0
    Replacement::draft(0x141636f70, "unknown_141636f70", fn_141636f70 as *const ()), // sub_141636f70
    Replacement::draft(0x1416371e0, "unknown_1416371e0", fn_1416371e0 as *const ()), // sub_1416371e0
    Replacement::draft(0x141637700, "unknown_141637700", fn_141637700 as *const ()), // sub_141637700
    Replacement::draft(0x1416377d0, "unknown_1416377d0", fn_1416377d0 as *const ()), // sub_1416377d0
    Replacement::draft(0x141637880, "unknown_141637880", fn_141637880 as *const ()), // sub_141637880
    Replacement::draft(0x141637d40, "unknown_141637d40", fn_141637d40 as *const ()), // sub_141637d40
    Replacement::draft(0x141637e50, "unknown_141637e50", fn_141637e50 as *const ()), // sub_141637e50
    Replacement::draft(0x141637fd0, "unknown_141637fd0", fn_141637fd0 as *const ()), // sub_141637fd0
    Replacement::draft(0x1416380f0, "unknown_1416380f0", fn_1416380f0 as *const ()), // sub_1416380f0
    Replacement::draft(0x1416382b0, "unknown_1416382b0", fn_1416382b0 as *const ()), // sub_1416382b0
    Replacement::draft(0x1416382e0, "unknown_1416382e0", fn_1416382e0 as *const ()), // sub_1416382e0
    Replacement::draft(0x141638380, "unknown_141638380", fn_141638380 as *const ()), // sub_141638380
    Replacement::draft(0x1416383a0, "unknown_1416383a0", fn_1416383a0 as *const ()), // sub_1416383a0
    Replacement::draft(0x1416384a0, "unknown_1416384a0", fn_1416384a0 as *const ()), // sub_1416384a0
    Replacement::draft(0x141638510, "unknown_141638510", fn_141638510 as *const ()), // sub_141638510
    Replacement::draft(0x141638530, "unknown_141638530", fn_141638530 as *const ()), // sub_141638530
    Replacement::draft(0x1416385f0, "unknown_1416385f0", fn_1416385f0 as *const ()), // sub_1416385f0
    Replacement::draft(0x141638990, "unknown_141638990", tpl_29742::<0xffffffffffffffd0, 0x20> as *const ()), // sub_141638990
    Replacement::draft(0x1416389a0, "unknown_1416389a0", fn_1416389a0 as *const ()), // sub_1416389a0
    Replacement::draft(0x141638a20, "unknown_141638a20", fn_141638a20 as *const ()), // sub_141638a20
    Replacement::draft(0x141638a30, "unknown_141638a30", tpl_548::<0xffffffffffffff60> as *const ()), // sub_141638a30
    Replacement::draft(0x141638a40, "unknown_141638a40", fn_141638a40 as *const ()), // sub_141638a40
    Replacement::draft(0x141638ae0, "unknown_141638ae0", fn_141638ae0 as *const ()), // sub_141638ae0
    Replacement::draft(0x141638b30, "unknown_141638b30", fn_141638b30 as *const ()), // sub_141638b30
    Replacement::draft(0x141638bb0, "unknown_141638bb0", fn_141638bb0 as *const ()), // sub_141638bb0
    Replacement::draft(0x141638c40, "unknown_141638c40", fn_141638c40 as *const ()), // sub_141638c40
    Replacement::draft(0x141638cd0, "unknown_141638cd0", fn_141638cd0 as *const ()), // sub_141638cd0
    Replacement::draft(0x141638d60, "unknown_141638d60", fn_141638d60 as *const ()), // sub_141638d60
    Replacement::draft(0x141638e10, "unknown_141638e10", fn_141638e10 as *const ()), // sub_141638e10
    Replacement::draft(0x141638e60, "unknown_141638e60", fn_141638e60 as *const ()), // sub_141638e60
    Replacement::draft(0x141638ec0, "unknown_141638ec0", fn_141638ec0 as *const ()), // sub_141638ec0
    Replacement::draft(0x141639040, "unknown_141639040", fn_141639040 as *const ()), // sub_141639040
    Replacement::draft(0x141639670, "unknown_141639670", fn_141639670 as *const ()), // sub_141639670
    Replacement::draft(0x1416399c0, "unknown_1416399c0", fn_1416399c0 as *const ()), // sub_1416399c0
    Replacement::draft(0x141639b30, "ak_141639b30", fn_141639b30 as *const ()), // sub_141639b30
    Replacement::draft(0x141639c00, "ak_141639c00", fn_141639c00 as *const ()), // sub_141639c00
    Replacement::draft(0x14163a030, "ak_14163a030", fn_14163a030 as *const ()), // sub_14163a030
    Replacement::draft(0x14163a310, "ak_14163a310", fn_14163a310 as *const ()), // sub_14163a310
    Replacement::draft(0x14163ad80, "ak_14163ad80", fn_14163ad80 as *const ()), // sub_14163ad80
    Replacement::draft(0x14163b0f0, "ak_14163b0f0", fn_14163b0f0 as *const ()), // sub_14163b0f0
    Replacement::draft(0x14163b220, "ak_14163b220", fn_14163b220 as *const ()), // sub_14163b220
    Replacement::draft(0x14163b2e0, "ak_14163b2e0", fn_14163b2e0 as *const ()), // sub_14163b2e0
    Replacement::draft(0x14163b610, "ak_14163b610", fn_14163b610 as *const ()), // sub_14163b610
    Replacement::draft(0x14163b6d0, "ak_14163b6d0", fn_14163b6d0 as *const ()), // sub_14163b6d0
    Replacement::draft(0x14163b890, "ak_14163b890", fn_14163b890 as *const ()), // sub_14163b890
    Replacement::draft(0x14163bd40, "ak_14163bd40", fn_14163bd40 as *const ()), // sub_14163bd40
    Replacement::draft(0x14163bee0, "ak_14163bee0", fn_14163bee0 as *const ()), // sub_14163bee0
    Replacement::draft(0x14163c230, "ak_14163c230", fn_14163c230 as *const ()), // sub_14163c230
    Replacement::draft(0x14163c440, "ak_14163c440", fn_14163c440 as *const ()), // sub_14163c440
    Replacement::draft(0x14163c590, "ak_14163c590", fn_14163c590 as *const ()), // sub_14163c590
    Replacement::draft(0x14163c5e0, "ak_14163c5e0", fn_14163c5e0 as *const ()), // sub_14163c5e0
    Replacement::draft(0x14163c6c0, "ak_14163c6c0", fn_14163c6c0 as *const ()), // sub_14163c6c0
    Replacement::draft(0x14163c730, "ak_14163c730", fn_14163c730 as *const ()), // sub_14163c730
    Replacement::draft(0x14163c8a0, "ak_14163c8a0", fn_14163c8a0 as *const ()), // sub_14163c8a0
    Replacement::draft(0x14163c990, "ak_14163c990", fn_14163c990 as *const ()), // sub_14163c990
    Replacement::draft(0x14163c9e0, "ak_14163c9e0", fn_14163c9e0 as *const ()), // sub_14163c9e0
    Replacement::draft(0x14163ca00, "ak_14163ca00", tpl_22865::<0x1426711d8, 0x141834d30> as *const ()), // sub_14163ca00
    Replacement::draft(0x14163ca30, "ak_14163ca30", fn_14163ca30 as *const ()), // sub_14163ca30
    Replacement::draft(0x14163ca70, "ak_14163ca70", fn_14163ca70 as *const ()), // sub_14163ca70
    Replacement::draft(0x14163ca80, "ak_14163ca80", fn_14163ca80 as *const ()), // sub_14163ca80
    Replacement::draft(0x14163caa0, "ak_14163caa0", fn_14163caa0 as *const ()), // sub_14163caa0
    Replacement::draft(0x14163cb10, "ak_14163cb10", fn_14163cb10 as *const ()), // sub_14163cb10
    Replacement::draft(0x14163cb20, "ak_14163cb20", fn_14163cb20 as *const ()), // AkMixer::AkMixer::ApplyGain
    Replacement::draft(0x14163cb30, "ak_14163cb30", fn_14163cb30 as *const ()), // AkMixer::AkMixer::ApplyGainAndInterleave
    Replacement::draft(0x14163cbd0, "ak_14163cbd0", fn_14163cbd0 as *const ()), // sub_14163cbd0
    Replacement::draft(0x14163ccd0, "ak_14163ccd0", fn_14163ccd0 as *const ()), // AkMixer::AkMixer::MixChannel
    Replacement::draft(0x14163ccf0, "ak_14163ccf0", fn_14163ccf0 as *const ()), // AkMixer::AkMixer::MixNinNChannels
    Replacement::draft(0x14163cd10, "ak_14163cd10", fn_14163cd10 as *const ()), // AkMixer::AkMixer::MixOneInNChannels
    Replacement::draft(0x14163cd30, "ak_14163cd30", fn_14163cd30 as *const ()), // AkMixer::AkMixer::ProcessBiquadFilter
    Replacement::draft(0x14163cee0, "ak_14163cee0", fn_14163cee0 as *const ()), // AkMixer::AkMixer::ProcessInterpBiquadFilter
    Replacement::draft(0x14163cf00, "ak_14163cf00", fn_14163cf00 as *const ()), // AkMixer::AkMixer::ProcessInterpBiquadFilterAndMix
    Replacement::draft(0x14163cf20, "ak_14163cf20", fn_14163cf20 as *const ()), // AkMixer::AkMixer::ProcessMix
    Replacement::draft(0x14163cf40, "ak_14163cf40", fn_14163cf40 as *const ()), // AkMixer::AkMixer::ProcessPairedBiquadFilter
    Replacement::draft(0x14163d1d0, "ak_14163d1d0", fn_14163d1d0 as *const ()), // AkMixer::AkMixer::ProcessPairedInterpBiquadFilter
    Replacement::draft(0x14163d1f0, "ak_14163d1f0", fn_14163d1f0 as *const ()), // AkMixer::AkMixer::ProcessPairedInterpBiquadFilterAndMix
    Replacement::draft(0x14163d210, "ak_14163d210", fn_14163d210 as *const ()), // sub_14163d210
    Replacement::draft(0x14163d250, "ak_14163d250", fn_14163d250 as *const ()), // sub_14163d250
    Replacement::draft(0x14163d2b0, "ak_14163d2b0", fn_14163d2b0 as *const ()), // sub_14163d2b0
    Replacement::draft(0x14163d320, "ak_14163d320", fn_14163d320 as *const ()), // sub_14163d320
    Replacement::draft(0x14163d380, "ak_14163d380", fn_14163d380 as *const ()), // sub_14163d380
    Replacement::draft(0x14163d400, "unknown_14163d400", fn_14163d400 as *const ()), // sub_14163d400
    Replacement::draft(0x14163d4f0, "unknown_14163d4f0", fn_14163d4f0 as *const ()), // sub_14163d4f0
    Replacement::draft(0x14163d540, "unknown_14163d540", fn_14163d540 as *const ()), // sub_14163d540
    Replacement::draft(0x14163d940, "unknown_14163d940", fn_14163d940 as *const ()), // sub_14163d940
    Replacement::draft(0x14163dbc0, "unknown_14163dbc0", fn_14163dbc0 as *const ()), // sub_14163dbc0
    Replacement::draft(0x14163e350, "unknown_14163e350", fn_14163e350 as *const ()), // sub_14163e350
    Replacement::draft(0x14163ea90, "unknown_14163ea90", fn_14163ea90 as *const ()), // sub_14163ea90
    Replacement::draft(0x14163f340, "unknown_14163f340", fn_14163f340 as *const ()), // sub_14163f340
    Replacement::draft(0x14163fa80, "unknown_14163fa80", fn_14163fa80 as *const ()), // sub_14163fa80
    Replacement::draft(0x141640040, "unknown_141640040", fn_141640040 as *const ()), // sub_141640040
    Replacement::draft(0x1416404c0, "unknown_1416404c0", fn_1416404c0 as *const ()), // sub_1416404c0
    Replacement::draft(0x141640b70, "unknown_141640b70", fn_141640b70 as *const ()), // sub_141640b70
    Replacement::draft(0x141641040, "unknown_141641040", fn_141641040 as *const ()), // sub_141641040
    Replacement::draft(0x141641400, "unknown_141641400", fn_141641400 as *const ()), // sub_141641400
    Replacement::draft(0x141641680, "unknown_141641680", fn_141641680 as *const ()), // sub_141641680
    Replacement::draft(0x141641820, "unknown_141641820", fn_141641820 as *const ()), // sub_141641820
    Replacement::draft(0x1416419b0, "unknown_1416419b0", fn_1416419b0 as *const ()), // sub_1416419b0
    Replacement::draft(0x1416419e0, "unknown_1416419e0", fn_1416419e0 as *const ()), // sub_1416419e0
    Replacement::draft(0x141641c60, "unknown_141641c60", fn_141641c60 as *const ()), // sub_141641c60
    Replacement::draft(0x141641ee0, "unknown_141641ee0", fn_141641ee0 as *const ()), // sub_141641ee0
    Replacement::draft(0x141641f90, "unknown_141641f90", fn_141641f90 as *const ()), // sub_141641f90
    Replacement::draft(0x141642330, "unknown_141642330", fn_141642330 as *const ()), // sub_141642330
    Replacement::draft(0x141642740, "unknown_141642740", fn_141642740 as *const ()), // sub_141642740
    Replacement::draft(0x1416427d0, "unknown_1416427d0", fn_1416427d0 as *const ()), // sub_1416427d0
    Replacement::draft(0x141642c40, "unknown_141642c40", fn_141642c40 as *const ()), // sub_141642c40
    Replacement::draft(0x141642e70, "unknown_141642e70", fn_141642e70 as *const ()), // sub_141642e70
    Replacement::draft(0x141642f70, "unknown_141642f70", fn_141642f70 as *const ()), // sub_141642f70
    Replacement::draft(0x141642fc0, "unknown_141642fc0", fn_141642fc0 as *const ()), // sub_141642fc0
    Replacement::draft(0x141642ff0, "unknown_141642ff0", fn_141642ff0 as *const ()), // sub_141642ff0
    Replacement::draft(0x141643050, "unknown_141643050", fn_141643050 as *const ()), // sub_141643050
    Replacement::draft(0x141643100, "unknown_141643100", fn_141643100 as *const ()), // sub_141643100
    Replacement::draft(0x141643130, "unknown_141643130", fn_141643130 as *const ()), // sub_141643130
    Replacement::draft(0x1416431e0, "unknown_1416431e0", fn_1416431e0 as *const ()), // sub_1416431e0
    Replacement::draft(0x141643200, "unknown_141643200", fn_141643200 as *const ()), // sub_141643200
    Replacement::draft(0x141643450, "unknown_141643450", fn_141643450 as *const ()), // sub_141643450
    Replacement::draft(0x141643610, "unknown_141643610", fn_141643610 as *const ()), // sub_141643610
    Replacement::draft(0x141643690, "unknown_141643690", tpl_30883::<0x28> as *const ()), // sub_141643690
    Replacement::draft(0x1416436a0, "unknown_1416436a0", fn_1416436a0 as *const ()), // sub_1416436a0
    Replacement::draft(0x141643700, "unknown_141643700", fn_141643700 as *const ()), // sub_141643700
    Replacement::draft(0x1416438a0, "unknown_1416438a0", fn_1416438a0 as *const ()), // sub_1416438a0
    Replacement::draft(0x141643930, "unknown_141643930", fn_141643930 as *const ()), // sub_141643930
    Replacement::draft(0x141643b10, "unknown_141643b10", fn_141643b10 as *const ()), // sub_141643b10
    Replacement::draft(0x141643b60, "unknown_141643b60", tpl_1674::<0x14268b678> as *const ()), // sub_141643b60
    Replacement::draft(0x141643b70, "unknown_141643b70", fn_141643b70 as *const ()), // sub_141643b70
    Replacement::draft(0x141643e80, "unknown_141643e80", fn_141643e80 as *const ()), // sub_141643e80
    Replacement::draft(0x141643ea0, "unknown_141643ea0", fn_141643ea0 as *const ()), // sub_141643ea0
    Replacement::draft(0x141644040, "unknown_141644040", tpl_30419::<0x10> as *const ()), // sub_141644040
    Replacement::draft(0x141644050, "unknown_141644050", tpl_22867::<0x8, 0x10> as *const ()), // sub_141644050
    Replacement::draft(0x141644070, "unknown_141644070", fn_141644070 as *const ()), // sub_141644070
    Replacement::draft(0x1416440f0, "unknown_1416440f0", fn_1416440f0 as *const ()), // sub_1416440f0
    Replacement::draft(0x141644270, "unknown_141644270", fn_141644270 as *const ()), // sub_141644270
    Replacement::draft(0x141644400, "unknown_141644400", fn_141644400 as *const ()), // sub_141644400
    Replacement::draft(0x141644820, "unknown_141644820", fn_141644820 as *const ()), // sub_141644820
    Replacement::draft(0x141644a90, "unknown_141644a90", fn_141644a90 as *const ()), // sub_141644a90
    Replacement::draft(0x141644f50, "unknown_141644f50", fn_141644f50 as *const ()), // sub_141644f50
    Replacement::draft(0x141645320, "unknown_141645320", fn_141645320 as *const ()), // sub_141645320
    Replacement::draft(0x1416454a0, "unknown_1416454a0", fn_1416454a0 as *const ()), // sub_1416454a0
    Replacement::draft(0x1416454d0, "unknown_1416454d0", fn_1416454d0 as *const ()), // sub_1416454d0
    Replacement::draft(0x141645510, "unknown_141645510", fn_141645510 as *const ()), // sub_141645510
    Replacement::draft(0x141645880, "unknown_141645880", fn_141645880 as *const ()), // sub_141645880
    Replacement::draft(0x141645b90, "unknown_141645b90", fn_141645b90 as *const ()), // sub_141645b90
    Replacement::draft(0x141645bf0, "unknown_141645bf0", fn_141645bf0 as *const ()), // sub_141645bf0
    Replacement::draft(0x141645cf0, "unknown_141645cf0", fn_141645cf0 as *const ()), // sub_141645cf0
    Replacement::draft(0x141645e30, "unknown_141645e30", fn_141645e30 as *const ()), // sub_141645e30
    Replacement::draft(0x141645f90, "unknown_141645f90", fn_141645f90 as *const ()), // sub_141645f90
    Replacement::draft(0x141646770, "unknown_141646770", fn_141646770 as *const ()), // sub_141646770
    Replacement::draft(0x141646900, "unknown_141646900", fn_141646900 as *const ()), // sub_141646900
    Replacement::draft(0x141646930, "unknown_141646930", fn_141646930 as *const ()), // sub_141646930
    Replacement::draft(0x141646950, "unknown_141646950", fn_141646950 as *const ()), // sub_141646950
    Replacement::draft(0x141646970, "unknown_141646970", fn_141646970 as *const ()), // sub_141646970
    Replacement::draft(0x141646a00, "unknown_141646a00", fn_141646a00 as *const ()), // sub_141646a00
    Replacement::draft(0x141646a30, "concurrency_141646a30", fn_141646a30 as *const ()), // Concurrency::details::UMSFreeThreadProxyFactory::Concurrency::details::UMSFreeThreadProxyFactory::`scalar_deleting_destr
    Replacement::draft(0x141646a80, "concurrency_141646a80", tpl_16123::<0x142691448, 0x141646970, 0x14268dac0, 0x141834d30> as *const ()), // Concurrency::details::UMSFreeThreadProxyFactory::Concurrency::details::UMSFreeThreadProxyFactory::`scalar_deleting_destr
    Replacement::draft(0x141646ad0, "concurrency_141646ad0", fn_141646ad0 as *const ()), // Concurrency::details::UMSFreeThreadProxyFactory::Concurrency::details::UMSFreeThreadProxyFactory::`scalar_deleting_destr
    Replacement::draft(0x141646b20, "concurrency_141646b20", fn_141646b20 as *const ()), // sub_141646b20
    Replacement::draft(0x141646c50, "concurrency_141646c50", fn_141646c50 as *const ()), // sub_141646c50
    Replacement::draft(0x141646c70, "concurrency_141646c70", fn_141646c70 as *const ()), // sub_141646c70
    Replacement::draft(0x141646c90, "concurrency_141646c90", fn_141646c90 as *const ()), // sub_141646c90
    Replacement::draft(0x141646d40, "concurrency_141646d40", tpl_30716::<0x20, 0x8, 0x141591570> as *const ()), // sub_141646d40
    Replacement::draft(0x141646d50, "concurrency_141646d50", fn_141646d50 as *const ()), // sub_141646d50
    Replacement::draft(0x141646dd0, "concurrency_141646dd0", tpl_28401::<0x20, 0xa0> as *const ()), // sub_141646dd0
    Replacement::draft(0x141646df0, "concurrency_141646df0", tpl_30911::<0x20, 0x14163c990> as *const ()), // sub_141646df0
    Replacement::draft(0x141646e10, "concurrency_141646e10", fn_141646e10 as *const ()), // sub_141646e10
    Replacement::draft(0x141646e20, "concurrency_141646e20", fn_141646e20 as *const ()), // sub_141646e20
    Replacement::draft(0x141646e30, "concurrency_141646e30", tpl_30914::<0x20, 0x50> as *const ()), // sub_141646e30
    Replacement::draft(0x141646e50, "concurrency_141646e50", fn_141646e50 as *const ()), // sub_141646e50
    Replacement::draft(0x141646ea0, "concurrency_141646ea0", fn_141646ea0 as *const ()), // sub_141646ea0
    Replacement::draft(0x141646ec0, "concurrency_141646ec0", fn_141646ec0 as *const ()), // sub_141646ec0
    Replacement::draft(0x141646f70, "concurrency_141646f70", fn_141646f70 as *const ()), // sub_141646f70
    Replacement::draft(0x141647050, "concurrency_141647050", tpl_30914::<0x20, 0x68> as *const ()), // sub_141647050
    Replacement::draft(0x141647070, "concurrency_141647070", fn_141647070 as *const ()), // sub_141647070
    Replacement::draft(0x1416470e0, "concurrency_1416470e0", tpl_28401::<0x20, 0x68> as *const ()), // sub_1416470e0
    Replacement::draft(0x141647100, "concurrency_141647100", fn_141647100 as *const ()), // sub_141647100
    Replacement::draft(0x141647250, "concurrency_141647250", tpl_29831::<0x20, 0x8> as *const ()), // sub_141647250
    Replacement::draft(0x141647260, "concurrency_141647260", tpl_27317::<0x28, 0x1416341c0> as *const ()), // sub_141647260
    Replacement::draft(0x141647270, "concurrency_141647270", fn_141647270 as *const ()), // sub_141647270
    Replacement::draft(0x141647310, "concurrency_141647310", fn_141647310 as *const ()), // sub_141647310
    Replacement::draft(0x141647390, "concurrency_141647390", fn_141647390 as *const ()), // sub_141647390
    Replacement::draft(0x1416473c0, "concurrency_1416473c0", tpl_27317::<0x20, 0x14163c730> as *const ()), // sub_1416473c0
    Replacement::draft(0x1416473d0, "concurrency_1416473d0", fn_1416473d0 as *const ()), // sub_1416473d0
    Replacement::draft(0x141647410, "concurrency_141647410", fn_141647410 as *const ()), // sub_141647410
    Replacement::draft(0x1416474c0, "concurrency_1416474c0", fn_1416474c0 as *const ()), // sub_1416474c0
    Replacement::draft(0x141647570, "concurrency_141647570", fn_141647570 as *const ()), // sub_141647570
    Replacement::draft(0x1416475b0, "concurrency_1416475b0", fn_1416475b0 as *const ()), // sub_1416475b0
    Replacement::draft(0x141647630, "concurrency_141647630", fn_141647630 as *const ()), // sub_141647630
    Replacement::draft(0x1416476e0, "concurrency_1416476e0", fn_1416476e0 as *const ()), // sub_1416476e0
    Replacement::draft(0x141647710, "concurrency_141647710", fn_141647710 as *const ()), // sub_141647710
    Replacement::draft(0x141647754, "concurrency_141647754", tpl_29183::<0x141647760> as *const ()), // sub_141647754
    Replacement::draft(0x141647760, "concurrency_141647760", fn_141647760 as *const ()), // sub_141647760
    Replacement::draft(0x1416477b0, "concurrency_1416477b0", fn_1416477b0 as *const ()), // sub_1416477b0
    Replacement::draft(0x1416477d0, "concurrency_1416477d0", fn_1416477d0 as *const ()), // sub_1416477d0
    Replacement::draft(0x141647860, "concurrency_141647860", fn_141647860 as *const ()), // sub_141647860
    Replacement::draft(0x141647940, "concurrency_141647940", fn_141647940 as *const ()), // sub_141647940
    Replacement::draft(0x141647960, "concurrency_141647960", fn_141647960 as *const ()), // sub_141647960
    Replacement::draft(0x141647a20, "concurrency_141647a20", fn_141647a20 as *const ()), // sub_141647a20
    Replacement::draft(0x141647cf0, "concurrency_141647cf0", fn_141647cf0 as *const ()), // sub_141647cf0
    Replacement::draft(0x141647d40, "concurrency_141647d40", fn_141647d40 as *const ()), // sub_141647d40
    Replacement::draft(0x141647e10, "concurrency_141647e10", fn_141647e10 as *const ()), // sub_141647e10
    Replacement::draft(0x141648090, "concurrency_141648090", fn_141648090 as *const ()), // sub_141648090
    Replacement::draft(0x141648290, "concurrency_141648290", fn_141648290 as *const ()), // sub_141648290
    Replacement::draft(0x141648490, "concurrency_141648490", fn_141648490 as *const ()), // sub_141648490
    Replacement::draft(0x1416486c0, "concurrency_1416486c0", fn_1416486c0 as *const ()), // sub_1416486c0
    Replacement::draft(0x1416488e0, "concurrency_1416488e0", fn_1416488e0 as *const ()), // sub_1416488e0
    Replacement::draft(0x141648920, "concurrency_141648920", fn_141648920 as *const ()), // sub_141648920
    Replacement::draft(0x141648930, "concurrency_141648930", tpl_30942::<0x141648490> as *const ()), // sub_141648930
    Replacement::draft(0x141648950, "concurrency_141648950", tpl_30942::<0x1416486c0> as *const ()), // sub_141648950
    Replacement::draft(0x141648970, "concurrency_141648970", fn_141648970 as *const ()), // sub_141648970
    Replacement::draft(0x1416489f4, "concurrency_1416489f4", tpl_28319::<0x141648a10> as *const ()), // sub_1416489f4
    Replacement::draft(0x141648a00, "concurrency_141648a00", tpl_28339::<0x141648a10> as *const ()), // sub_141648a00
    Replacement::draft(0x141648a10, "concurrency_141648a10", fn_141648a10 as *const ()), // sub_141648a10
    Replacement::draft(0x141648a80, "unknown_141648a80", fn_141648a80 as *const ()), // sub_141648a80
    Replacement::draft(0x141648e20, "unknown_141648e20", fn_141648e20 as *const ()), // sub_141648e20
    Replacement::draft(0x141648f40, "unknown_141648f40", fn_141648f40 as *const ()), // sub_141648f40
    Replacement::draft(0x141648f90, "unknown_141648f90", fn_141648f90 as *const ()), // sub_141648f90
    Replacement::draft(0x141648fd0, "unknown_141648fd0", fn_141648fd0 as *const ()), // sub_141648fd0
    Replacement::draft(0x141649010, "unknown_141649010", tpl_1::<0x143815ed8> as *const ()), // sub_141649010
    Replacement::draft(0x141649020, "unknown_141649020", fn_141649020 as *const ()), // sub_141649020
    Replacement::draft(0x141649050, "unknown_141649050", fn_141649050 as *const ()), // sub_141649050
    Replacement::draft(0x141649140, "unknown_141649140", fn_141649140 as *const ()), // sub_141649140
    Replacement::draft(0x1416491f0, "unknown_1416491f0", fn_1416491f0 as *const ()), // sub_1416491f0
    Replacement::draft(0x141649250, "unknown_141649250", tpl_29465::<0x142677c50, 0x142677588, 0x8, 0x142677ec8, 0x10, 0x142677ef0, 0x28, 0x14153b0e0> as *const ()), // sub_141649250
    Replacement::draft(0x141649280, "unknown_141649280", fn_141649280 as *const ()), // sub_141649280
    Replacement::draft(0x141649330, "unknown_141649330", tpl_28296::<0x142691af8, 0x8, 0x10> as *const ()), // sub_141649330
    Replacement::draft(0x141649350, "unknown_141649350", tpl_28296::<0x142691b60, 0x8, 0x10> as *const ()), // sub_141649350
    Replacement::draft(0x141649370, "unknown_141649370", tpl_22865::<0x14268e6d0, 0x141834d30> as *const ()), // sub_141649370
    Replacement::draft(0x1416493a0, "unknown_1416493a0", fn_1416493a0 as *const ()), // sub_1416493a0
    Replacement::draft(0x141649420, "unknown_141649420", fn_141649420 as *const ()), // sub_141649420
    Replacement::draft(0x141649490, "unknown_141649490", fn_141649490 as *const ()), // sub_141649490
    Replacement::draft(0x1416494b0, "unknown_1416494b0", fn_1416494b0 as *const ()), // sub_1416494b0
    Replacement::draft(0x141649510, "unknown_141649510", fn_141649510 as *const ()), // sub_141649510
    Replacement::draft(0x141649540, "unknown_141649540", fn_141649540 as *const ()), // sub_141649540
    Replacement::draft(0x141649570, "unknown_141649570", fn_141649570 as *const ()), // sub_141649570
    Replacement::draft(0x1416495a0, "unknown_1416495a0", fn_1416495a0 as *const ()), // sub_1416495a0
    Replacement::draft(0x1416495e0, "unknown_1416495e0", fn_1416495e0 as *const ()), // sub_1416495e0
    Replacement::draft(0x1416495f0, "unknown_1416495f0", fn_1416495f0 as *const ()), // sub_1416495f0
    Replacement::draft(0x141649600, "unknown_141649600", fn_141649600 as *const ()), // sub_141649600
    Replacement::draft(0x141649610, "unknown_141649610", fn_141649610 as *const ()), // sub_141649610
    Replacement::draft(0x141649620, "unknown_141649620", fn_141649620 as *const ()), // sub_141649620
    Replacement::draft(0x1416496b0, "unknown_1416496b0", fn_1416496b0 as *const ()), // sub_1416496b0
    Replacement::draft(0x141649710, "unknown_141649710", tpl_3047::<0x10> as *const ()), // sub_141649710
    Replacement::draft(0x141649720, "unknown_141649720", fn_141649720 as *const ()), // sub_141649720
    Replacement::draft(0x141649730, "unknown_141649730", fn_141649730 as *const ()), // sub_141649730
    Replacement::draft(0x1416498d0, "unknown_1416498d0", fn_1416498d0 as *const ()), // sub_1416498d0
    Replacement::draft(0x14164994c, "unknown_14164994c", tpl_28319::<0x141649970> as *const ()), // sub_14164994c
    Replacement::draft(0x141649958, "unknown_141649958", tpl_28339::<0x141649970> as *const ()), // sub_141649958
    Replacement::draft(0x141649970, "unknown_141649970", fn_141649970 as *const ()), // sub_141649970
    Replacement::draft(0x141649a10, "unknown_141649a10", fn_141649a10 as *const ()), // sub_141649a10
    Replacement::draft(0x141649ac0, "unknown_141649ac0", fn_141649ac0 as *const ()), // sub_141649ac0
    Replacement::draft(0x141649bd0, "unknown_141649bd0", fn_141649bd0 as *const ()), // sub_141649bd0
    Replacement::draft(0x141649c20, "unknown_141649c20", fn_141649c20 as *const ()), // sub_141649c20
    Replacement::draft(0x141649ee0, "unknown_141649ee0", fn_141649ee0 as *const ()), // sub_141649ee0
    Replacement::draft(0x14164a1a0, "unknown_14164a1a0", fn_14164a1a0 as *const ()), // sub_14164a1a0
    Replacement::draft(0x14164a3c0, "unknown_14164a3c0", fn_14164a3c0 as *const ()), // sub_14164a3c0
    Replacement::draft(0x14164a410, "unknown_14164a410", fn_14164a410 as *const ()), // sub_14164a410
    Replacement::draft(0x14164a450, "unknown_14164a450", fn_14164a450 as *const ()), // sub_14164a450
    Replacement::draft(0x14164a6e0, "unknown_14164a6e0", fn_14164a6e0 as *const ()), // sub_14164a6e0
    Replacement::draft(0x14164a700, "unknown_14164a700", fn_14164a700 as *const ()), // sub_14164a700
    Replacement::draft(0x14164ab50, "unknown_14164ab50", fn_14164ab50 as *const ()), // sub_14164ab50
    Replacement::draft(0x14164abb0, "unknown_14164abb0", fn_14164abb0 as *const ()), // sub_14164abb0
    Replacement::draft(0x14164ac20, "unknown_14164ac20", fn_14164ac20 as *const ()), // sub_14164ac20
    Replacement::draft(0x14164acd0, "unknown_14164acd0", fn_14164acd0 as *const ()), // sub_14164acd0
    Replacement::draft(0x14164ad60, "unknown_14164ad60", tpl_913::<0x58> as *const ()), // sub_14164ad60
    Replacement::draft(0x14164ad70, "unknown_14164ad70", tpl_497::<0x68> as *const ()), // sub_14164ad70
    Replacement::draft(0x14164ad80, "unknown_14164ad80", fn_14164ad80 as *const ()), // sub_14164ad80
    Replacement::draft(0x14164add0, "unknown_14164add0", fn_14164add0 as *const ()), // sub_14164add0
    Replacement::draft(0x14164adf0, "unknown_14164adf0", fn_14164adf0 as *const ()), // sub_14164adf0
    Replacement::draft(0x14164aef0, "unknown_14164aef0", fn_14164aef0 as *const ()), // sub_14164aef0
    Replacement::draft(0x14164af20, "unknown_14164af20", fn_14164af20 as *const ()), // sub_14164af20
    Replacement::draft(0x14164b030, "unknown_14164b030", fn_14164b030 as *const ()), // sub_14164b030
    Replacement::draft(0x14164b060, "unknown_14164b060", fn_14164b060 as *const ()), // sub_14164b060
    Replacement::draft(0x14164b0a0, "unknown_14164b0a0", tpl_27062::<0x142691d38, 0x141834d30> as *const ()), // sub_14164b0a0
    Replacement::draft(0x14164b0d0, "unknown_14164b0d0", tpl_27317::<0x8, 0x141634310> as *const ()), // sub_14164b0d0
    Replacement::draft(0x14164b0e0, "unknown_14164b0e0", fn_14164b0e0 as *const ()), // sub_14164b0e0
    Replacement::draft(0x14164b170, "unknown_14164b170", fn_14164b170 as *const ()), // sub_14164b170
    Replacement::draft(0x14164b1e0, "unknown_14164b1e0", fn_14164b1e0 as *const ()), // sub_14164b1e0
    Replacement::draft(0x14164b230, "unknown_14164b230", fn_14164b230 as *const ()), // sub_14164b230
    Replacement::draft(0x14164b270, "unknown_14164b270", fn_14164b270 as *const ()), // sub_14164b270
    Replacement::draft(0x14164b600, "unknown_14164b600", fn_14164b600 as *const ()), // sub_14164b600
    Replacement::draft(0x14164b6a0, "unknown_14164b6a0", fn_14164b6a0 as *const ()), // sub_14164b6a0
    Replacement::draft(0x14164b6d0, "unknown_14164b6d0", fn_14164b6d0 as *const ()), // sub_14164b6d0
    Replacement::draft(0x14164b710, "unknown_14164b710", tpl_30994::<0x9c, 0x98, 0xcc, 0xc8, 0x8, 0x40> as *const ()), // sub_14164b710
    Replacement::draft(0x14164b740, "unknown_14164b740", fn_14164b740 as *const ()), // sub_14164b740
    Replacement::draft(0x14164b7b0, "unknown_14164b7b0", fn_14164b7b0 as *const ()), // sub_14164b7b0
    Replacement::draft(0x14164b830, "unknown_14164b830", fn_14164b830 as *const ()), // sub_14164b830
    Replacement::draft(0x14164b850, "unknown_14164b850", fn_14164b850 as *const ()), // sub_14164b850
    Replacement::draft(0x14164b8e0, "unknown_14164b8e0", fn_14164b8e0 as *const ()), // sub_14164b8e0
    Replacement::draft(0x14164b950, "unknown_14164b950", fn_14164b950 as *const ()), // sub_14164b950
    Replacement::draft(0x14164b9a0, "unknown_14164b9a0", fn_14164b9a0 as *const ()), // sub_14164b9a0
    Replacement::draft(0x14164bb30, "unknown_14164bb30", fn_14164bb30 as *const ()), // sub_14164bb30
    Replacement::draft(0x14164bb60, "unknown_14164bb60", fn_14164bb60 as *const ()), // sub_14164bb60
    Replacement::draft(0x14164bc10, "unknown_14164bc10", fn_14164bc10 as *const ()), // sub_14164bc10
    Replacement::draft(0x14164bcc0, "unknown_14164bcc0", fn_14164bcc0 as *const ()), // sub_14164bcc0
    Replacement::draft(0x14164bcd0, "unknown_14164bcd0", fn_14164bcd0 as *const ()), // sub_14164bcd0
    Replacement::draft(0x14164bcf0, "unknown_14164bcf0", fn_14164bcf0 as *const ()), // sub_14164bcf0
    Replacement::draft(0x14164bd10, "unknown_14164bd10", fn_14164bd10 as *const ()), // sub_14164bd10
    Replacement::draft(0x14164be10, "unknown_14164be10", fn_14164be10 as *const ()), // sub_14164be10
    Replacement::draft(0x14164be30, "unknown_14164be30", tpl_500::<0xe0> as *const ()), // sub_14164be30
    Replacement::draft(0x14164be40, "unknown_14164be40", fn_14164be40 as *const ()), // sub_14164be40
    Replacement::draft(0x14164bed0, "unknown_14164bed0", fn_14164bed0 as *const ()), // sub_14164bed0
    Replacement::draft(0x14164bf20, "unknown_14164bf20", fn_14164bf20 as *const ()), // sub_14164bf20
    Replacement::draft(0x14164bf80, "unknown_14164bf80", fn_14164bf80 as *const ()), // sub_14164bf80
    Replacement::draft(0x14164bfb0, "unknown_14164bfb0", fn_14164bfb0 as *const ()), // sub_14164bfb0
    Replacement::draft(0x14164bfd0, "unknown_14164bfd0", fn_14164bfd0 as *const ()), // sub_14164bfd0
    Replacement::draft(0x14164c060, "unknown_14164c060", fn_14164c060 as *const ()), // sub_14164c060
    Replacement::draft(0x14164c0a0, "unknown_14164c0a0", fn_14164c0a0 as *const ()), // sub_14164c0a0
    Replacement::draft(0x14164c280, "unknown_14164c280", fn_14164c280 as *const ()), // sub_14164c280
    Replacement::draft(0x14164c2a0, "unknown_14164c2a0", fn_14164c2a0 as *const ()), // sub_14164c2a0
    Replacement::draft(0x14164c5e0, "unknown_14164c5e0", fn_14164c5e0 as *const ()), // sub_14164c5e0
    Replacement::draft(0x14164c620, "unknown_14164c620", tpl_500::<0x120> as *const ()), // sub_14164c620
    Replacement::draft(0x14164c630, "unknown_14164c630", fn_14164c630 as *const ()), // sub_14164c630
    Replacement::draft(0x14164c750, "unknown_14164c750", tpl_30994::<0xfc, 0xf8, 0x134, 0x130, 0x8, 0x40> as *const ()), // sub_14164c750
    Replacement::draft(0x14164c780, "unknown_14164c780", fn_14164c780 as *const ()), // sub_14164c780
    Replacement::draft(0x14164c9b0, "unknown_14164c9b0", fn_14164c9b0 as *const ()), // sub_14164c9b0
    Replacement::draft(0x14164ca30, "unknown_14164ca30", fn_14164ca30 as *const ()), // sub_14164ca30
    Replacement::draft(0x14164caa0, "unknown_14164caa0", fn_14164caa0 as *const ()), // sub_14164caa0
    Replacement::draft(0x14164cae0, "unknown_14164cae0", fn_14164cae0 as *const ()), // sub_14164cae0
    Replacement::draft(0x14164cb60, "unknown_14164cb60", fn_14164cb60 as *const ()), // sub_14164cb60
    Replacement::draft(0x14164cbc0, "unknown_14164cbc0", fn_14164cbc0 as *const ()), // sub_14164cbc0
    Replacement::draft(0x14164cc60, "unknown_14164cc60", fn_14164cc60 as *const ()), // sub_14164cc60
    Replacement::draft(0x14164cca0, "unknown_14164cca0", tpl_29629::<0x1415bd2d0, 0x142691d58, 0x10> as *const ()), // sub_14164cca0
    Replacement::draft(0x14164ccd0, "unknown_14164ccd0", fn_14164ccd0 as *const ()), // sub_14164ccd0
    Replacement::draft(0x14164cd60, "unknown_14164cd60", fn_14164cd60 as *const ()), // sub_14164cd60
    Replacement::draft(0x14164ce10, "unknown_14164ce10", fn_14164ce10 as *const ()), // sub_14164ce10
    Replacement::draft(0x14164ce70, "unknown_14164ce70", fn_14164ce70 as *const ()), // sub_14164ce70
    Replacement::draft(0x14164ce90, "unknown_14164ce90", fn_14164ce90 as *const ()), // sub_14164ce90
    Replacement::draft(0x14164cf20, "unknown_14164cf20", fn_14164cf20 as *const ()), // sub_14164cf20
    Replacement::draft(0x14164cfd0, "unknown_14164cfd0", fn_14164cfd0 as *const ()), // sub_14164cfd0
    Replacement::draft(0x14164d0d0, "unknown_14164d0d0", tpl_29129::<0x10, 0x142691dd8, 0x18, 0x143931588, 0x143931580, 0x143931578, 0x142688f88, 0x141834d30> as *const ()), // sub_14164d0d0
    Replacement::draft(0x14164d160, "unknown_14164d160", fn_14164d160 as *const ()), // sub_14164d160
    Replacement::draft(0x14164d2c0, "unknown_14164d2c0", fn_14164d2c0 as *const ()), // sub_14164d2c0
    Replacement::draft(0x14164d350, "unknown_14164d350", fn_14164d350 as *const ()), // sub_14164d350
    Replacement::draft(0x14164d400, "unknown_14164d400", fn_14164d400 as *const ()), // sub_14164d400
    Replacement::draft(0x14164d550, "unknown_14164d550", fn_14164d550 as *const ()), // sub_14164d550
    Replacement::draft(0x14164d6b0, "unknown_14164d6b0", fn_14164d6b0 as *const ()), // sub_14164d6b0
    Replacement::draft(0x14164d790, "unknown_14164d790", fn_14164d790 as *const ()), // sub_14164d790
    Replacement::draft(0x14164d7e0, "unknown_14164d7e0", fn_14164d7e0 as *const ()), // sub_14164d7e0
    Replacement::draft(0x14164d910, "unknown_14164d910", fn_14164d910 as *const ()), // sub_14164d910
    Replacement::draft(0x14164d950, "unknown_14164d950", fn_14164d950 as *const ()), // sub_14164d950
    Replacement::draft(0x14164da20, "unknown_14164da20", fn_14164da20 as *const ()), // sub_14164da20
    Replacement::draft(0x14164da90, "unknown_14164da90", fn_14164da90 as *const ()), // sub_14164da90
    Replacement::draft(0x14164daf0, "unknown_14164daf0", fn_14164daf0 as *const ()), // sub_14164daf0
    Replacement::draft(0x14164dbd0, "unknown_14164dbd0", fn_14164dbd0 as *const ()), // sub_14164dbd0
    Replacement::draft(0x14164dd20, "unknown_14164dd20", fn_14164dd20 as *const ()), // sub_14164dd20
    Replacement::draft(0x14164dd50, "unknown_14164dd50", fn_14164dd50 as *const ()), // sub_14164dd50
    Replacement::draft(0x14164ddb0, "unknown_14164ddb0", tpl_31043::<0x142691df8, 0x28, 0x1415242e0, 0x20, 0x1415bd160, 0x141834d30> as *const ()), // sub_14164ddb0
    Replacement::draft(0x14164de30, "unknown_14164de30", tpl_31044::<0xffffffffffffffc1, 0x141524330, 0x14160f4a0, 0x142691df8, 0x38> as *const ()), // sub_14164de30
    Replacement::draft(0x14164de90, "unknown_14164de90", fn_14164de90 as *const ()), // sub_14164de90
    Replacement::draft(0x14164e180, "unknown_14164e180", fn_14164e180 as *const ()), // sub_14164e180
    Replacement::draft(0x14164e1d0, "unknown_14164e1d0", tpl_30461::<0x142691e58, 0x1416513c0, 0x141834d30> as *const ()), // sub_14164e1d0
    Replacement::draft(0x14164e210, "unknown_14164e210", fn_14164e210 as *const ()), // sub_14164e210
    Replacement::draft(0x14164e270, "unknown_14164e270", fn_14164e270 as *const ()), // sub_14164e270
    Replacement::draft(0x14164e380, "unknown_14164e380", fn_14164e380 as *const ()), // sub_14164e380
    Replacement::draft(0x14164e3a0, "unknown_14164e3a0", fn_14164e3a0 as *const ()), // sub_14164e3a0
    Replacement::draft(0x14164e3c0, "unknown_14164e3c0", tpl_30461::<0x142691ec8, 0x1416513c0, 0x141834d30> as *const ()), // sub_14164e3c0
    Replacement::draft(0x14164e400, "unknown_14164e400", fn_14164e400 as *const ()), // sub_14164e400
    Replacement::draft(0x14164e450, "unknown_14164e450", fn_14164e450 as *const ()), // sub_14164e450
    Replacement::draft(0x14164e560, "unknown_14164e560", fn_14164e560 as *const ()), // sub_14164e560
    Replacement::draft(0x14164e590, "unknown_14164e590", tpl_30692::<0x142691f38, 0x28, 0x1415242e0, 0x20, 0x1415bd160, 0x141834d30> as *const ()), // sub_14164e590
    Replacement::draft(0x14164e610, "unknown_14164e610", tpl_31053::<0xffffffffffffffc9, 0x141524330, 0x14160f4a0, 0x142691f38> as *const ()), // sub_14164e610
    Replacement::draft(0x14164e660, "unknown_14164e660", fn_14164e660 as *const ()), // sub_14164e660
    Replacement::draft(0x14164e720, "unknown_14164e720", tpl_31055::<0x142691f88, 0x141669010, 0x141834d30> as *const ()), // sub_14164e720
    Replacement::draft(0x14164e760, "unknown_14164e760", tpl_31056::<0xffffffffffffffb9, 0x141524330, 0x141668fe0, 0x142691f88> as *const ()), // sub_14164e760
    Replacement::draft(0x14164e7b0, "unknown_14164e7b0", fn_14164e7b0 as *const ()), // sub_14164e7b0
    Replacement::draft(0x14164e800, "unknown_14164e800", fn_14164e800 as *const ()), // sub_14164e800
    Replacement::draft(0x14164e860, "unknown_14164e860", fn_14164e860 as *const ()), // sub_14164e860
    Replacement::draft(0x14164e8e0, "unknown_14164e8e0", fn_14164e8e0 as *const ()), // sub_14164e8e0
    Replacement::draft(0x14164e970, "unknown_14164e970", fn_14164e970 as *const ()), // sub_14164e970
    Replacement::draft(0x14164e9c0, "unknown_14164e9c0", fn_14164e9c0 as *const ()), // sub_14164e9c0
    Replacement::draft(0x14164ea20, "unknown_14164ea20", tpl_31063::<0x142692020, 0x141669010, 0x141834d30> as *const ()), // sub_14164ea20
    Replacement::draft(0x14164ea60, "unknown_14164ea60", fn_14164ea60 as *const ()), // sub_14164ea60
    Replacement::draft(0x14164ead0, "unknown_14164ead0", fn_14164ead0 as *const ()), // sub_14164ead0
    Replacement::draft(0x14164eb30, "unknown_14164eb30", fn_14164eb30 as *const ()), // sub_14164eb30
    Replacement::draft(0x14164ebb0, "unknown_14164ebb0", fn_14164ebb0 as *const ()), // sub_14164ebb0
    Replacement::draft(0x14164ebf0, "unknown_14164ebf0", fn_14164ebf0 as *const ()), // sub_14164ebf0
    Replacement::draft(0x14164ec30, "unknown_14164ec30", fn_14164ec30 as *const ()), // sub_14164ec30
    Replacement::draft(0x14164ed10, "unknown_14164ed10", fn_14164ed10 as *const ()), // sub_14164ed10
    Replacement::draft(0x14164ed90, "unknown_14164ed90", fn_14164ed90 as *const ()), // sub_14164ed90
    Replacement::draft(0x14164ee10, "unknown_14164ee10", fn_14164ee10 as *const ()), // sub_14164ee10
    Replacement::draft(0x14164ee50, "unknown_14164ee50", tpl_31043::<0x1426920b8, 0x28, 0x1415242e0, 0x20, 0x1415bd160, 0x141834d30> as *const ()), // sub_14164ee50
    Replacement::draft(0x14164eed0, "unknown_14164eed0", tpl_31044::<0xffffffffffffffc1, 0x141524330, 0x14160f4a0, 0x1426920b8, 0x38> as *const ()), // sub_14164eed0
    Replacement::draft(0x14164ef30, "unknown_14164ef30", fn_14164ef30 as *const ()), // sub_14164ef30
    Replacement::draft(0x14164ef60, "unknown_14164ef60", fn_14164ef60 as *const ()), // sub_14164ef60
    Replacement::draft(0x14164ef90, "unknown_14164ef90", tpl_31043::<0x142692108, 0x28, 0x1415242e0, 0x20, 0x1415bd160, 0x141834d30> as *const ()), // sub_14164ef90
    Replacement::draft(0x14164f010, "unknown_14164f010", tpl_31044::<0xffffffffffffffc1, 0x141524330, 0x14160f4a0, 0x142692108, 0x38> as *const ()), // sub_14164f010
    Replacement::draft(0x14164f070, "unknown_14164f070", fn_14164f070 as *const ()), // sub_14164f070
    Replacement::draft(0x14164f100, "unknown_14164f100", fn_14164f100 as *const ()), // sub_14164f100
    Replacement::draft(0x14164f140, "unknown_14164f140", fn_14164f140 as *const ()), // sub_14164f140
    Replacement::draft(0x14164f1b0, "unknown_14164f1b0", fn_14164f1b0 as *const ()), // sub_14164f1b0
    Replacement::draft(0x14164f260, "unknown_14164f260", fn_14164f260 as *const ()), // sub_14164f260
    Replacement::draft(0x14164f310, "unknown_14164f310", fn_14164f310 as *const ()), // sub_14164f310
    Replacement::draft(0x14164f420, "unknown_14164f420", fn_14164f420 as *const ()), // sub_14164f420
    Replacement::draft(0x14164f540, "unknown_14164f540", fn_14164f540 as *const ()), // sub_14164f540
    Replacement::draft(0x14164f590, "unknown_14164f590", tpl_30692::<0x1426921f0, 0x28, 0x1415242e0, 0x20, 0x1415bd160, 0x141834d30> as *const ()), // sub_14164f590
    Replacement::draft(0x14164f610, "unknown_14164f610", tpl_31053::<0xffffffffffffffc9, 0x141524330, 0x14160f4a0, 0x1426921f0> as *const ()), // sub_14164f610
    Replacement::draft(0x14164f660, "unknown_14164f660", fn_14164f660 as *const ()), // sub_14164f660
    Replacement::draft(0x14164f6c0, "unknown_14164f6c0", tpl_30461::<0x142692240, 0x141651000, 0x141834d30> as *const ()), // sub_14164f6c0
    Replacement::draft(0x14164f700, "unknown_14164f700", fn_14164f700 as *const ()), // sub_14164f700
    Replacement::draft(0x14164f760, "unknown_14164f760", fn_14164f760 as *const ()), // sub_14164f760
    Replacement::draft(0x14164fb20, "unknown_14164fb20", fn_14164fb20 as *const ()), // sub_14164fb20
    Replacement::draft(0x14164fda0, "unknown_14164fda0", fn_14164fda0 as *const ()), // sub_14164fda0
    Replacement::draft(0x14164fdd0, "unknown_14164fdd0", tpl_30692::<0x1426922a8, 0x28, 0x1415242e0, 0x20, 0x1415bd160, 0x141834d30> as *const ()), // sub_14164fdd0
    Replacement::draft(0x14164fe50, "unknown_14164fe50", tpl_31053::<0xffffffffffffffc9, 0x141524330, 0x14160f4a0, 0x1426922a8> as *const ()), // sub_14164fe50
    Replacement::draft(0x14164fea0, "unknown_14164fea0", fn_14164fea0 as *const ()), // sub_14164fea0
    Replacement::draft(0x14164fed0, "unknown_14164fed0", fn_14164fed0 as *const ()), // sub_14164fed0
    Replacement::draft(0x14164ff30, "unknown_14164ff30", tpl_31063::<0x1426922f8, 0x141651000, 0x141834d30> as *const ()), // sub_14164ff30
    Replacement::draft(0x14164ff70, "unknown_14164ff70", fn_14164ff70 as *const ()), // sub_14164ff70
    Replacement::draft(0x14164ffd0, "unknown_14164ffd0", fn_14164ffd0 as *const ()), // sub_14164ffd0
    Replacement::draft(0x1416501a0, "unknown_1416501a0", fn_1416501a0 as *const ()), // sub_1416501a0
    Replacement::draft(0x141650200, "unknown_141650200", tpl_31055::<0x142692360, 0x1416513c0, 0x141834d30> as *const ()), // sub_141650200
    Replacement::draft(0x141650240, "unknown_141650240", tpl_31056::<0xffffffffffffffb9, 0x141524330, 0x141651390, 0x142692360> as *const ()), // sub_141650240
    Replacement::draft(0x141650290, "unknown_141650290", fn_141650290 as *const ()), // sub_141650290
    Replacement::draft(0x141650340, "unknown_141650340", tpl_30692::<0x1426923d0, 0x28, 0x1415242e0, 0x20, 0x1415bd160, 0x141834d30> as *const ()), // sub_141650340
    Replacement::draft(0x1416503c0, "unknown_1416503c0", tpl_31053::<0xffffffffffffffc9, 0x141524330, 0x14160f4a0, 0x1426923d0> as *const ()), // sub_1416503c0
    Replacement::draft(0x141650410, "unknown_141650410", fn_141650410 as *const ()), // sub_141650410
    Replacement::draft(0x1416505a0, "unknown_1416505a0", tpl_31055::<0x142692420, 0x1416513c0, 0x141834d30> as *const ()), // sub_1416505a0
    Replacement::draft(0x1416505e0, "unknown_1416505e0", tpl_31056::<0xffffffffffffffb9, 0x141524330, 0x141651390, 0x142692420> as *const ()), // sub_1416505e0
    Replacement::draft(0x141650630, "unknown_141650630", fn_141650630 as *const ()), // sub_141650630
    Replacement::draft(0x1416506b0, "unknown_1416506b0", fn_1416506b0 as *const ()), // sub_1416506b0
    Replacement::draft(0x1416507f0, "unknown_1416507f0", tpl_31063::<0x142692490, 0x141651000, 0x141834d30> as *const ()), // sub_1416507f0
    Replacement::draft(0x141650830, "unknown_141650830", fn_141650830 as *const ()), // sub_141650830
    Replacement::draft(0x141650890, "unknown_141650890", fn_141650890 as *const ()), // sub_141650890
    Replacement::draft(0x141650a10, "unknown_141650a10", fn_141650a10 as *const ()), // sub_141650a10
    Replacement::draft(0x141650cf0, "unknown_141650cf0", fn_141650cf0 as *const ()), // sub_141650cf0
    Replacement::draft(0x141650f70, "unknown_141650f70", fn_141650f70 as *const ()), // sub_141650f70
    Replacement::draft(0x141650fd0, "unknown_141650fd0", fn_141650fd0 as *const ()), // sub_141650fd0
    Replacement::draft(0x141651000, "unknown_141651000", fn_141651000 as *const ()), // sub_141651000
    Replacement::draft(0x141651080, "unknown_141651080", fn_141651080 as *const ()), // sub_141651080
    Replacement::draft(0x141651120, "unknown_141651120", fn_141651120 as *const ()), // sub_141651120
    Replacement::draft(0x1416511e0, "unknown_1416511e0", tpl_3047::<0x40> as *const ()), // sub_1416511e0
    Replacement::draft(0x1416511f0, "unknown_1416511f0", fn_1416511f0 as *const ()), // sub_1416511f0
    Replacement::draft(0x141651260, "unknown_141651260", fn_141651260 as *const ()), // sub_141651260
    Replacement::draft(0x141651390, "unknown_141651390", tpl_26762::<0x141650fd0, 0x142692560> as *const ()), // sub_141651390
    Replacement::draft(0x1416513c0, "unknown_1416513c0", tpl_22426::<0x142692560, 0x141651000> as *const ()), // sub_1416513c0
    Replacement::draft(0x1416513d0, "unknown_1416513d0", tpl_31055::<0x142692560, 0x141651000, 0x141834d30> as *const ()), // sub_1416513d0
    Replacement::draft(0x141651410, "unknown_141651410", fn_141651410 as *const ()), // sub_141651410
    Replacement::draft(0x1416514c0, "unknown_1416514c0", fn_1416514c0 as *const ()), // sub_1416514c0
    Replacement::draft(0x141651590, "unknown_141651590", fn_141651590 as *const ()), // sub_141651590
    Replacement::draft(0x141651790, "unknown_141651790", fn_141651790 as *const ()), // sub_141651790
    Replacement::draft(0x1416517f0, "unknown_1416517f0", fn_1416517f0 as *const ()), // sub_1416517f0
    Replacement::draft(0x1416518c0, "unknown_1416518c0", fn_1416518c0 as *const ()), // sub_1416518c0
    Replacement::draft(0x141651900, "unknown_141651900", fn_141651900 as *const ()), // sub_141651900
    Replacement::draft(0x1416519a0, "unknown_1416519a0", fn_1416519a0 as *const ()), // sub_1416519a0
    Replacement::draft(0x141651a00, "unknown_141651a00", fn_141651a00 as *const ()), // sub_141651a00
    Replacement::draft(0x141651a60, "unknown_141651a60", fn_141651a60 as *const ()), // sub_141651a60
    Replacement::draft(0x141651ad0, "unknown_141651ad0", fn_141651ad0 as *const ()), // sub_141651ad0
    Replacement::draft(0x141651d10, "unknown_141651d10", fn_141651d10 as *const ()), // sub_141651d10
    Replacement::draft(0x141651f00, "unknown_141651f00", fn_141651f00 as *const ()), // sub_141651f00
    Replacement::draft(0x141651fc0, "unknown_141651fc0", fn_141651fc0 as *const ()), // sub_141651fc0
    Replacement::draft(0x141652000, "ak_141652000", fn_141652000 as *const ()), // sub_141652000
    Replacement::draft(0x141652050, "ak_141652050", fn_141652050 as *const ()), // sub_141652050
    Replacement::draft(0x141652090, "ak_141652090", fn_141652090 as *const ()), // sub_141652090
    Replacement::draft(0x1416520c0, "ak_1416520c0", fn_1416520c0 as *const ()), // sub_1416520c0
    Replacement::draft(0x1416520f0, "ak_1416520f0", fn_1416520f0 as *const ()), // sub_1416520f0
    Replacement::draft(0x141652100, "ak_141652100", fn_141652100 as *const ()), // sub_141652100
    Replacement::draft(0x141652310, "ak_141652310", fn_141652310 as *const ()), // sub_141652310
    Replacement::draft(0x141652390, "ak_141652390", fn_141652390 as *const ()), // sub_141652390
    Replacement::draft(0x141652480, "ak_141652480", fn_141652480 as *const ()), // sub_141652480
    Replacement::draft(0x1416524c0, "ak_1416524c0", tpl_28737::<0x1416545a0> as *const ()), // sub_1416524c0
    Replacement::draft(0x1416524d0, "ak_1416524d0", fn_1416524d0 as *const ()), // sub_1416524d0
    Replacement::draft(0x1416527d0, "ak_1416527d0", fn_1416527d0 as *const ()), // sub_1416527d0
    Replacement::draft(0x141652b50, "ak_141652b50", fn_141652b50 as *const ()), // sub_141652b50
    Replacement::draft(0x141652b80, "ak_141652b80", fn_141652b80 as *const ()), // sub_141652b80
    Replacement::draft_throws(0x141652ce0, "ak_141652ce0", fn_141652ce0 as *const (), Throws::Discard), // sub_141652ce0
    Replacement::draft(0x141653340, "ak_141653340", fn_141653340 as *const ()), // sub_141653340
    Replacement::draft(0x1416538d0, "ak_1416538d0", fn_1416538d0 as *const ()), // sub_1416538d0
    Replacement::draft(0x141653de0, "ak_141653de0", fn_141653de0 as *const ()), // sub_141653de0
    Replacement::draft(0x141653ea0, "ak_141653ea0", tpl_4544::<0x266> as *const ()), // sub_141653ea0
    Replacement::draft(0x141653eb0, "ak_141653eb0", fn_141653eb0 as *const ()), // sub_141653eb0
    Replacement::draft(0x141653fc0, "ak_141653fc0", fn_141653fc0 as *const ()), // sub_141653fc0
    Replacement::draft(0x141654310, "ak_141654310", fn_141654310 as *const ()), // sub_141654310
    Replacement::draft(0x141654590, "ak_141654590", tpl_31123::<0x266> as *const ()), // sub_141654590
    Replacement::draft(0x1416545a0, "ak_1416545a0", fn_1416545a0 as *const ()), // sub_1416545a0
    Replacement::draft(0x141654b40, "ak_141654b40", fn_141654b40 as *const ()), // sub_141654b40
    Replacement::draft(0x141654be0, "ak_141654be0", fn_141654be0 as *const ()), // sub_141654be0
    Replacement::draft(0x141654e50, "ak_141654e50", fn_141654e50 as *const ()), // sub_141654e50
    Replacement::draft(0x141654f10, "ak_141654f10", fn_141654f10 as *const ()), // sub_141654f10
    Replacement::draft(0x1416552b0, "ak_1416552b0", fn_1416552b0 as *const ()), // sub_1416552b0
    Replacement::draft(0x141655680, "ak_141655680", fn_141655680 as *const ()), // sub_141655680
    Replacement::draft(0x141655d20, "ak_141655d20", fn_141655d20 as *const ()), // sub_141655d20
    Replacement::draft(0x1416563e0, "ak_1416563e0", fn_1416563e0 as *const ()), // sub_1416563e0
    Replacement::draft(0x1416564c0, "ak_1416564c0", fn_1416564c0 as *const ()), // sub_1416564c0
    Replacement::draft(0x141656680, "ak_141656680", fn_141656680 as *const ()), // sub_141656680
    Replacement::draft(0x141656870, "ak_141656870", fn_141656870 as *const ()), // sub_141656870
    Replacement::draft(0x141656950, "ak_141656950", fn_141656950 as *const ()), // sub_141656950
    Replacement::draft(0x141656b90, "ak_141656b90", fn_141656b90 as *const ()), // sub_141656b90
    Replacement::draft(0x141656c50, "ak_141656c50", fn_141656c50 as *const ()), // sub_141656c50
    Replacement::draft(0x141656cf0, "ak_141656cf0", fn_141656cf0 as *const ()), // sub_141656cf0
    Replacement::draft(0x141656db0, "ak_141656db0", fn_141656db0 as *const ()), // sub_141656db0
    Replacement::draft(0x141656f00, "ak_141656f00", fn_141656f00 as *const ()), // sub_141656f00
    Replacement::draft(0x1416570a0, "ak_1416570a0", fn_1416570a0 as *const ()), // sub_1416570a0
    Replacement::draft(0x141657150, "ak_141657150", fn_141657150 as *const ()), // sub_141657150
    Replacement::draft(0x1416572b0, "ak_1416572b0", fn_1416572b0 as *const ()), // sub_1416572b0
    Replacement::draft(0x141657300, "ak_141657300", fn_141657300 as *const ()), // sub_141657300
    Replacement::draft(0x141657660, "ak_141657660", fn_141657660 as *const ()), // sub_141657660
    Replacement::draft(0x141657850, "ak_141657850", fn_141657850 as *const ()), // sub_141657850
    Replacement::draft(0x141657bc0, "ak_141657bc0", fn_141657bc0 as *const ()), // sub_141657bc0
    Replacement::draft(0x1416586a0, "ak_1416586a0", fn_1416586a0 as *const ()), // sub_1416586a0
    Replacement::draft(0x141658790, "ak_141658790", fn_141658790 as *const ()), // sub_141658790
    Replacement::draft(0x141658c80, "ak_141658c80", fn_141658c80 as *const ()), // sub_141658c80
    Replacement::draft(0x141658fc0, "ak_141658fc0", fn_141658fc0 as *const ()), // sub_141658fc0
    Replacement::draft(0x141659450, "ak_141659450", fn_141659450 as *const ()), // sub_141659450
    Replacement::draft(0x1416595b0, "ak_1416595b0", fn_1416595b0 as *const ()), // sub_1416595b0
    Replacement::draft(0x141659a30, "ak_141659a30", fn_141659a30 as *const ()), // sub_141659a30
    Replacement::draft(0x141659cb0, "ak_141659cb0", fn_141659cb0 as *const ()), // sub_141659cb0
    Replacement::draft(0x141659f60, "ak_141659f60", fn_141659f60 as *const ()), // sub_141659f60
    Replacement::draft(0x14165a2d0, "ak_14165a2d0", fn_14165a2d0 as *const ()), // sub_14165a2d0
    Replacement::draft(0x14165a530, "ak_14165a530", fn_14165a530 as *const ()), // sub_14165a530
    Replacement::draft(0x14165a9d0, "ak_14165a9d0", fn_14165a9d0 as *const ()), // sub_14165a9d0
    Replacement::draft(0x14165ae90, "ak_14165ae90", fn_14165ae90 as *const ()), // sub_14165ae90
    Replacement::draft(0x14165b3d0, "ak_14165b3d0", fn_14165b3d0 as *const ()), // sub_14165b3d0
    Replacement::draft(0x14165b9a0, "ak_14165b9a0", fn_14165b9a0 as *const ()), // sub_14165b9a0
    Replacement::draft(0x14165bff0, "ak_14165bff0", fn_14165bff0 as *const ()), // sub_14165bff0
    Replacement::draft(0x14165c670, "ak_14165c670", fn_14165c670 as *const ()), // sub_14165c670
    Replacement::draft(0x14165cdf0, "ak_14165cdf0", fn_14165cdf0 as *const ()), // sub_14165cdf0
    Replacement::draft(0x14165d590, "ak_14165d590", fn_14165d590 as *const ()), // sub_14165d590
    Replacement::draft(0x14165ddd0, "ak_14165ddd0", fn_14165ddd0 as *const ()), // sub_14165ddd0
    Replacement::draft(0x14165e6c0, "ak_14165e6c0", fn_14165e6c0 as *const ()), // sub_14165e6c0
    Replacement::draft(0x14165f060, "ak_14165f060", fn_14165f060 as *const ()), // sub_14165f060
    Replacement::draft(0x14165fb00, "ak_14165fb00", fn_14165fb00 as *const ()), // sub_14165fb00
    Replacement::draft(0x14165fc50, "ak_14165fc50", fn_14165fc50 as *const ()), // sub_14165fc50
    Replacement::draft(0x14165fca0, "ak_14165fca0", fn_14165fca0 as *const ()), // sub_14165fca0
    Replacement::draft(0x141660020, "ak_141660020", fn_141660020 as *const ()), // sub_141660020
    Replacement::draft(0x141660230, "ak_141660230", fn_141660230 as *const ()), // sub_141660230
    Replacement::draft(0x141660540, "ak_141660540", fn_141660540 as *const ()), // sub_141660540
    Replacement::draft(0x141660ee0, "ak_141660ee0", fn_141660ee0 as *const ()), // sub_141660ee0
    Replacement::draft(0x1416612b0, "ak_1416612b0", fn_1416612b0 as *const ()), // sub_1416612b0
    Replacement::draft(0x141661430, "ak_141661430", fn_141661430 as *const ()), // sub_141661430
    Replacement::draft(0x1416618b0, "ak_1416618b0", fn_1416618b0 as *const ()), // sub_1416618b0
    Replacement::draft(0x141661ac0, "ak_141661ac0", fn_141661ac0 as *const ()), // sub_141661ac0
    Replacement::draft(0x141661ee0, "ak_141661ee0", fn_141661ee0 as *const ()), // sub_141661ee0
    Replacement::draft(0x141662100, "ak_141662100", fn_141662100 as *const ()), // sub_141662100
    Replacement::draft(0x141662290, "ak_141662290", fn_141662290 as *const ()), // sub_141662290
    Replacement::draft(0x1416624b0, "ak_1416624b0", fn_1416624b0 as *const ()), // sub_1416624b0
    Replacement::draft(0x1416628c0, "ak_1416628c0", fn_1416628c0 as *const ()), // sub_1416628c0
    Replacement::draft(0x141662c30, "ak_141662c30", fn_141662c30 as *const ()), // sub_141662c30
    Replacement::draft(0x141663090, "ak_141663090", fn_141663090 as *const ()), // sub_141663090
    Replacement::draft(0x141663920, "ak_141663920", fn_141663920 as *const ()), // sub_141663920
    Replacement::draft(0x141663c60, "ak_141663c60", fn_141663c60 as *const ()), // sub_141663c60
    Replacement::draft(0x141664140, "ak_141664140", fn_141664140 as *const ()), // sub_141664140
    Replacement::draft(0x141664b10, "ak_141664b10", fn_141664b10 as *const ()), // sub_141664b10
    Replacement::draft(0x141664d60, "ak_141664d60", fn_141664d60 as *const ()), // sub_141664d60
    Replacement::draft(0x141665040, "ak_141665040", fn_141665040 as *const ()), // sub_141665040
    Replacement::draft(0x141665280, "ak_141665280", fn_141665280 as *const ()), // sub_141665280
    Replacement::draft(0x1416655a0, "ak_1416655a0", fn_1416655a0 as *const ()), // sub_1416655a0
    Replacement::draft(0x141665710, "ak_141665710", fn_141665710 as *const ()), // sub_141665710
    Replacement::draft(0x141665990, "ak_141665990", fn_141665990 as *const ()), // sub_141665990
    Replacement::draft(0x141665df0, "ak_141665df0", fn_141665df0 as *const ()), // sub_141665df0
    Replacement::draft(0x141666200, "ak_141666200", fn_141666200 as *const ()), // sub_141666200
    Replacement::draft(0x141666710, "ak_141666710", fn_141666710 as *const ()), // sub_141666710
    Replacement::draft(0x1416670e0, "ak_1416670e0", fn_1416670e0 as *const ()), // sub_1416670e0
    Replacement::draft(0x141667550, "ak_141667550", fn_141667550 as *const ()), // sub_141667550
    Replacement::draft(0x141667b90, "ak_141667b90", fn_141667b90 as *const ()), // sub_141667b90
    Replacement::draft(0x141668640, "ak_141668640", fn_141668640 as *const ()), // sub_141668640
    Replacement::draft(0x1416687a0, "ak_1416687a0", fn_1416687a0 as *const ()), // sub_1416687a0
    Replacement::draft(0x1416688c0, "ak_1416688c0", fn_1416688c0 as *const ()), // sub_1416688c0
    Replacement::draft(0x141668bf0, "ak_141668bf0", fn_141668bf0 as *const ()), // sub_141668bf0
    Replacement::draft(0x141668fe0, "ak_141668fe0", tpl_26762::<0x141650fd0, 0x142693200> as *const ()), // sub_141668fe0
    Replacement::draft(0x141669010, "ak_141669010", tpl_22426::<0x142693200, 0x141651000> as *const ()), // sub_141669010
    Replacement::draft(0x141669020, "ak_141669020", tpl_31055::<0x142693200, 0x141651000, 0x141834d30> as *const ()), // sub_141669020
    Replacement::draft(0x141669060, "ak_141669060", fn_141669060 as *const ()), // sub_141669060
    Replacement::draft(0x141669590, "ak_141669590", fn_141669590 as *const ()), // sub_141669590
    Replacement::draft(0x1416697e0, "ak_1416697e0", fn_1416697e0 as *const ()), // sub_1416697e0
    Replacement::draft(0x141669830, "ak_141669830", fn_141669830 as *const ()), // sub_141669830
    Replacement::draft(0x141669be0, "ak_141669be0", fn_141669be0 as *const ()), // sub_141669be0
    Replacement::draft(0x141669d20, "ak_141669d20", fn_141669d20 as *const ()), // sub_141669d20
    Replacement::draft(0x141669eb0, "ak_141669eb0", fn_141669eb0 as *const ()), // sub_141669eb0
    Replacement::draft(0x141669f50, "ak_141669f50", fn_141669f50 as *const ()), // sub_141669f50
    Replacement::draft(0x14166a210, "ak_14166a210", fn_14166a210 as *const ()), // sub_14166a210
    Replacement::draft(0x14166a380, "ak_14166a380", fn_14166a380 as *const ()), // sub_14166a380
    Replacement::draft(0x14166a430, "ak_14166a430", fn_14166a430 as *const ()), // sub_14166a430
    Replacement::draft(0x14166a760, "ak_14166a760", fn_14166a760 as *const ()), // sub_14166a760
    Replacement::draft(0x14166a8c0, "ak_14166a8c0", fn_14166a8c0 as *const ()), // sub_14166a8c0
    Replacement::draft(0x14166abd0, "ak_14166abd0", fn_14166abd0 as *const ()), // sub_14166abd0
    Replacement::draft(0x14166ad70, "ak_14166ad70", fn_14166ad70 as *const ()), // sub_14166ad70
    Replacement::draft(0x14166b010, "ak_14166b010", fn_14166b010 as *const ()), // sub_14166b010
    Replacement::draft(0x14166d210, "ak_14166d210", fn_14166d210 as *const ()), // sub_14166d210
    Replacement::draft(0x14166d530, "ak_14166d530", fn_14166d530 as *const ()), // sub_14166d530
    Replacement::draft(0x14166d740, "ak_14166d740", fn_14166d740 as *const ()), // sub_14166d740
    Replacement::draft(0x14166e860, "ak_14166e860", fn_14166e860 as *const ()), // sub_14166e860
    Replacement::draft(0x14166e9c0, "ak_14166e9c0", fn_14166e9c0 as *const ()), // sub_14166e9c0
    Replacement::draft(0x14166ebf0, "ak_14166ebf0", fn_14166ebf0 as *const ()), // sub_14166ebf0
    Replacement::draft(0x14166ed00, "ak_14166ed00", fn_14166ed00 as *const ()), // sub_14166ed00
    Replacement::draft(0x14166ed30, "ak_14166ed30", fn_14166ed30 as *const ()), // sub_14166ed30
    Replacement::draft(0x14166ee50, "ak_14166ee50", fn_14166ee50 as *const ()), // sub_14166ee50
    Replacement::draft(0x14166eea0, "ak_14166eea0", fn_14166eea0 as *const ()), // sub_14166eea0
    Replacement::draft(0x14166f010, "ak_14166f010", fn_14166f010 as *const ()), // sub_14166f010
    Replacement::draft(0x14166f460, "ak_14166f460", fn_14166f460 as *const ()), // sub_14166f460
    Replacement::draft(0x14166ff80, "ak_14166ff80", fn_14166ff80 as *const ()), // sub_14166ff80
    Replacement::draft(0x1416705f0, "ak_1416705f0", fn_1416705f0 as *const ()), // sub_1416705f0
    Replacement::draft(0x141670c60, "ak_141670c60", fn_141670c60 as *const ()), // sub_141670c60
    Replacement::draft(0x141671080, "ak_141671080", fn_141671080 as *const ()), // sub_141671080
    Replacement::draft(0x141671350, "ak_141671350", fn_141671350 as *const ()), // sub_141671350
    Replacement::draft(0x141671360, "ak_141671360", fn_141671360 as *const ()), // sub_141671360
    Replacement::draft(0x141671450, "ak_141671450", fn_141671450 as *const ()), // sub_141671450
    Replacement::draft(0x141671610, "ak_141671610", fn_141671610 as *const ()), // sub_141671610
    Replacement::draft(0x141671770, "ak_141671770", fn_141671770 as *const ()), // sub_141671770
    Replacement::draft(0x1416717a0, "ak_1416717a0", fn_1416717a0 as *const ()), // sub_1416717a0
    Replacement::draft(0x141671950, "ak_141671950", fn_141671950 as *const ()), // sub_141671950
    Replacement::draft(0x141671c40, "ak_141671c40", fn_141671c40 as *const ()), // sub_141671c40
    Replacement::draft(0x141671dc0, "ak_141671dc0", fn_141671dc0 as *const ()), // sub_141671dc0
    Replacement::draft(0x141671e00, "ak_141671e00", fn_141671e00 as *const ()), // sub_141671e00
    Replacement::draft(0x141671e70, "ak_141671e70", fn_141671e70 as *const ()), // sub_141671e70
    Replacement::draft(0x141671f50, "ak_141671f50", fn_141671f50 as *const ()), // sub_141671f50
    Replacement::draft(0x141671ff0, "ak_141671ff0", fn_141671ff0 as *const ()), // sub_141671ff0
    Replacement::draft(0x141672010, "ak_141672010", fn_141672010 as *const ()), // sub_141672010
    Replacement::draft(0x1416722f0, "ak_1416722f0", fn_1416722f0 as *const ()), // sub_1416722f0
    Replacement::draft(0x141672430, "ak_141672430", fn_141672430 as *const ()), // sub_141672430
    Replacement::draft(0x141672520, "ak_141672520", fn_141672520 as *const ()), // sub_141672520
    Replacement::draft(0x141672650, "ak_141672650", fn_141672650 as *const ()), // sub_141672650
    Replacement::draft(0x141672c30, "ak_141672c30", fn_141672c30 as *const ()), // sub_141672c30
    Replacement::draft(0x141672c70, "ak_141672c70", fn_141672c70 as *const ()), // sub_141672c70
    Replacement::draft(0x141672ce0, "ak_141672ce0", fn_141672ce0 as *const ()), // sub_141672ce0
    Replacement::draft(0x141673410, "ak_141673410", fn_141673410 as *const ()), // sub_141673410
    Replacement::draft(0x141673570, "ak_141673570", fn_141673570 as *const ()), // sub_141673570
    Replacement::draft(0x1416737a0, "ak_1416737a0", fn_1416737a0 as *const ()), // sub_1416737a0
    Replacement::draft(0x1416738b0, "ak_1416738b0", fn_1416738b0 as *const ()), // AK::StreamMgr::AK::StreamMgr::AddLanguageChangeObserver
    Replacement::draft(0x141673950, "ak_141673950", fn_141673950 as *const ()), // sub_141673950
    Replacement::draft(0x1416739b0, "ak_1416739b0", fn_1416739b0 as *const ()), // sub_1416739b0
    Replacement::draft(0x141673a30, "ak_141673a30", fn_141673a30 as *const ()), // sub_141673a30
    Replacement::draft(0x141673b20, "ak_141673b20", fn_141673b20 as *const ()), // AK::StreamMgr::AK::StreamMgr::Create
    Replacement::draft(0x141673ba0, "ak_141673ba0", fn_141673ba0 as *const ()), // sub_141673ba0
    Replacement::draft(0x141673dd0, "ak_141673dd0", fn_141673dd0 as *const ()), // sub_141673dd0
    Replacement::draft(0x141673e90, "ak_141673e90", fn_141673e90 as *const ()), // sub_141673e90
    Replacement::draft(0x1416740c0, "ak_1416740c0", fn_1416740c0 as *const ()), // sub_1416740c0
    Replacement::draft(0x1416742c0, "ak_1416742c0", fn_1416742c0 as *const ()), // AK::StreamMgr::AK::StreamMgr::CreateDevice
    Replacement::draft(0x141674430, "ak_141674430", fn_141674430 as *const ()), // sub_141674430
    Replacement::draft(0x1416745e0, "ak_1416745e0", fn_1416745e0 as *const ()), // sub_1416745e0
    Replacement::draft(0x1416747b0, "ak_1416747b0", fn_1416747b0 as *const ()), // sub_1416747b0
    Replacement::draft(0x1416747f0, "ak_1416747f0", fn_1416747f0 as *const ()), // sub_1416747f0
    Replacement::draft(0x1416748d0, "ak_1416748d0", fn_1416748d0 as *const ()), // AK::StreamMgr::AK::StreamMgr::DestroyDevice
    Replacement::draft(0x141674930, "ak_141674930", fn_141674930 as *const ()), // sub_141674930
    Replacement::draft(0x141674a20, "ak_141674a20", fn_141674a20 as *const ()), // AK::StreamMgr::AK::StreamMgr::FlushAllCaches
    Replacement::draft(0x141674a70, "ak_141674a70", fn_141674a70 as *const ()), // sub_141674a70
    Replacement::draft(0x141674ae0, "ak_141674ae0", fn_141674ae0 as *const ()), // sub_141674ae0
    Replacement::draft(0x141674b40, "ak_141674b40", fn_141674b40 as *const ()), // sub_141674b40
    Replacement::draft(0x141674c90, "ak_141674c90", tpl_1::<0x143932060> as *const ()), // AK::StreamMgr::AK::StreamMgr::GetCurrentLanguage
    Replacement::draft(0x141674ca0, "ak_141674ca0", fn_141674ca0 as *const ()), // AK::StreamMgr::AK::StreamMgr::GetDefaultDeviceSettings
    Replacement::draft(0x141674cf0, "ak_141674cf0", tpl_366::<0x143932038> as *const ()), // AK::StreamMgr::AK::StreamMgr::GetFileLocationResolver
    Replacement::draft(0x141674d00, "ak_141674d00", fn_141674d00 as *const ()), // sub_141674d00
    Replacement::draft(0x141674e00, "ak_141674e00", fn_141674e00 as *const ()), // sub_141674e00
    Replacement::draft(0x141674ec0, "ak_141674ec0", fn_141674ec0 as *const ()), // sub_141674ec0
    Replacement::draft(0x141674fb0, "ak_141674fb0", fn_141674fb0 as *const ()), // sub_141674fb0
    Replacement::draft(0x141675080, "ak_141675080", fn_141675080 as *const ()), // sub_141675080
    Replacement::draft(0x1416752b0, "ak_1416752b0", tpl_31205::<0x1416776c0> as *const ()), // sub_1416752b0
    Replacement::draft(0x1416752c0, "ak_1416752c0", fn_1416752c0 as *const ()), // AK::StreamMgr::AK::StreamMgr::RemoveLanguageChangeObserver
    Replacement::draft(0x141675350, "ak_141675350", fn_141675350 as *const ()), // AK::StreamMgr::AK::StreamMgr::SetCurrentLanguage
    Replacement::draft(0x141675450, "ak_141675450", tpl_28926::<0x143932038> as *const ()), // AK::StreamMgr::AK::StreamMgr::SetFileLocationResolver
    Replacement::draft(0x141675460, "unknown_141675460", fn_141675460 as *const ()), // sub_141675460
    Replacement::draft(0x1416755a0, "unknown_1416755a0", fn_1416755a0 as *const ()), // sub_1416755a0
    Replacement::draft(0x141675620, "unknown_141675620", fn_141675620 as *const ()), // sub_141675620
    Replacement::draft(0x1416756f0, "global_1416756f0", tpl_83353::<0x1405eb9e0, 0x142646828> as *const ()), // FID_conflict:sprintf_s
    Replacement::draft(0x141675750, "unknown_141675750", fn_141675750 as *const ()), // sub_141675750
    Replacement::draft(0x141675800, "unknown_141675800", fn_141675800 as *const ()), // sub_141675800
    Replacement::draft(0x141675840, "unknown_141675840", fn_141675840 as *const ()), // sub_141675840
    Replacement::draft(0x1416758c0, "unknown_1416758c0", fn_1416758c0 as *const ()), // sub_1416758c0
    Replacement::draft(0x141675970, "unknown_141675970", tpl_31214::<0x7a, 0x142693d38, 0x142693dc8, 0x80, 0x60, 0x14167c3a0, 0x142693b10, 0x141675a50> as *const ()), // sub_141675970
    Replacement::draft(0x1416759c0, "unknown_1416759c0", fn_1416759c0 as *const ()), // sub_1416759c0
    Replacement::draft(0x141675a00, "unknown_141675a00", tpl_31214::<0x7a, 0x142693c60, 0x142693cc8, 0x80, 0x60, 0x14167c7e0, 0x14268dac0, 0x141675a50> as *const ()), // sub_141675a00
    Replacement::draft(0x141675a50, "unknown_141675a50", fn_141675a50 as *const ()), // sub_141675a50
    Replacement::draft(0x141675adc, "unknown_141675adc", tpl_28339::<0x141675b00> as *const ()), // sub_141675adc
    Replacement::draft(0x141675ae8, "unknown_141675ae8", tpl_28339::<0x141675c30> as *const ()), // sub_141675ae8
    Replacement::draft(0x141675b00, "unknown_141675b00", fn_141675b00 as *const ()), // sub_141675b00
    Replacement::draft(0x141675b70, "unknown_141675b70", fn_141675b70 as *const ()), // sub_141675b70
    Replacement::draft(0x141675bd0, "unknown_141675bd0", fn_141675bd0 as *const ()), // sub_141675bd0
    Replacement::draft(0x141675c30, "unknown_141675c30", fn_141675c30 as *const ()), // sub_141675c30
    Replacement::draft(0x141675ca0, "opentelemetry_141675ca0", tpl_27762::<0x141675a50, 0x141834d30> as *const ()), // sub_141675ca0
    Replacement::draft(0x141675ce0, "unknown_141675ce0", tpl_893::<0x142693b10, 0x141834d30> as *const ()), // sub_141675ce0
    Replacement::draft(0x141675d10, "unknown_141675d10", fn_141675d10 as *const ()), // sub_141675d10
    Replacement::draft(0x141675dd0, "unknown_141675dd0", fn_141675dd0 as *const ()), // sub_141675dd0
    Replacement::draft(0x141675ec0, "unknown_141675ec0", fn_141675ec0 as *const ()), // sub_141675ec0
    Replacement::draft(0x141675f10, "unknown_141675f10", fn_141675f10 as *const ()), // sub_141675f10
    Replacement::draft(0x141675fd0, "unknown_141675fd0", fn_141675fd0 as *const ()), // sub_141675fd0
    Replacement::draft(0x141676060, "unknown_141676060", fn_141676060 as *const ()), // sub_141676060
    Replacement::draft(0x141676120, "unknown_141676120", fn_141676120 as *const ()), // sub_141676120
    Replacement::draft(0x1416761a0, "unknown_1416761a0", fn_1416761a0 as *const ()), // sub_1416761a0
    Replacement::draft(0x141676260, "unknown_141676260", fn_141676260 as *const ()), // sub_141676260
    Replacement::draft(0x141676290, "unknown_141676290", fn_141676290 as *const ()), // sub_141676290
    Replacement::draft(0x1416762e0, "unknown_1416762e0", fn_1416762e0 as *const ()), // sub_1416762e0
    Replacement::draft(0x141676360, "unknown_141676360", fn_141676360 as *const ()), // sub_141676360
    Replacement::draft(0x141676380, "unknown_141676380", fn_141676380 as *const ()), // sub_141676380
    Replacement::draft(0x1416763e0, "unknown_1416763e0", fn_1416763e0 as *const ()), // sub_1416763e0
    Replacement::draft(0x141676450, "unknown_141676450", fn_141676450 as *const ()), // sub_141676450
    Replacement::draft(0x1416764c0, "unknown_1416764c0", fn_1416764c0 as *const ()), // sub_1416764c0
    Replacement::draft(0x141676780, "unknown_141676780", fn_141676780 as *const ()), // sub_141676780
    Replacement::draft(0x1416768e0, "ak_1416768e0", fn_1416768e0 as *const ()), // sub_1416768e0
    Replacement::draft(0x141676920, "unknown_141676920", fn_141676920 as *const ()), // sub_141676920
    Replacement::draft(0x141676980, "unknown_141676980", fn_141676980 as *const ()), // sub_141676980
    Replacement::draft(0x141676ab0, "unknown_141676ab0", tpl_1691::<0xfffffffffffffff0> as *const ()), // sub_141676ab0
    Replacement::draft(0x141676ac0, "unknown_141676ac0", fn_141676ac0 as *const ()), // sub_141676ac0
    Replacement::draft(0x141676c30, "unknown_141676c30", fn_141676c30 as *const ()), // sub_141676c30
    Replacement::draft(0x141676c50, "unknown_141676c50", fn_141676c50 as *const ()), // sub_141676c50
    Replacement::draft(0x141676c60, "unknown_141676c60", tpl_913::<0xffffffffffffffa8> as *const ()), // sub_141676c60
    Replacement::draft(0x141676c70, "unknown_141676c70", fn_141676c70 as *const ()), // sub_141676c70
    Replacement::draft(0x141676c90, "unknown_141676c90", fn_141676c90 as *const ()), // sub_141676c90
    Replacement::draft(0x141676cc0, "unknown_141676cc0", fn_141676cc0 as *const ()), // sub_141676cc0
    Replacement::draft(0x141676ce0, "unknown_141676ce0", fn_141676ce0 as *const ()), // sub_141676ce0
    Replacement::draft(0x141676d14, "unknown_141676d14", tpl_28339::<0x141676ce0> as *const ()), // sub_141676d14
    Replacement::draft(0x141676d20, "unknown_141676d20", fn_141676d20 as *const ()), // sub_141676d20
    Replacement::draft(0x141676d30, "unknown_141676d30", fn_141676d30 as *const ()), // sub_141676d30
    Replacement::draft(0x141676db0, "unknown_141676db0", tpl_913::<0x30> as *const ()), // sub_141676db0
    Replacement::draft(0x141676dc0, "unknown_141676dc0", fn_141676dc0 as *const ()), // sub_141676dc0
    Replacement::draft(0x141676dd0, "unknown_141676dd0", fn_141676dd0 as *const ()), // sub_141676dd0
    Replacement::draft(0x141676ef0, "unknown_141676ef0", tpl_1691::<0x38> as *const ()), // sub_141676ef0
    Replacement::draft(0x141676f00, "unknown_141676f00", tpl_1691::<0xa8> as *const ()), // sub_141676f00
    Replacement::draft(0x141676f10, "unknown_141676f10", fn_141676f10 as *const ()), // sub_141676f10
    Replacement::draft_throws(0x141677050, "unknown_141677050", fn_141677050 as *const (), Throws::Discard), // sub_141677050
    Replacement::draft(0x1416770f0, "unknown_1416770f0", fn_1416770f0 as *const ()), // sub_1416770f0
    Replacement::draft(0x141677180, "unknown_141677180", fn_141677180 as *const ()), // sub_141677180
    Replacement::draft(0x1416771e0, "unknown_1416771e0", fn_1416771e0 as *const ()), // sub_1416771e0
    Replacement::draft(0x141677280, "unknown_141677280", fn_141677280 as *const ()), // sub_141677280
    Replacement::draft(0x1416772e0, "unknown_1416772e0", tpl_1743::<0x79> as *const ()), // sub_1416772e0
    Replacement::draft(0x1416772f0, "unknown_1416772f0", fn_1416772f0 as *const ()), // sub_1416772f0
    Replacement::draft(0x141677300, "unknown_141677300", fn_141677300 as *const ()), // sub_141677300
    Replacement::draft(0x141677410, "unknown_141677410", fn_141677410 as *const ()), // sub_141677410
    Replacement::draft(0x141677420, "unknown_141677420", fn_141677420 as *const ()), // sub_141677420
    Replacement::draft(0x141677470, "unknown_141677470", fn_141677470 as *const ()), // sub_141677470
    Replacement::draft(0x141677540, "unknown_141677540", fn_141677540 as *const ()), // sub_141677540
    Replacement::draft(0x141677560, "unknown_141677560", fn_141677560 as *const ()), // sub_141677560
    Replacement::draft(0x1416776c0, "unknown_1416776c0", tpl_30419::<0x20> as *const ()), // sub_1416776c0
    Replacement::draft(0x1416776d0, "unknown_1416776d0", fn_1416776d0 as *const ()), // sub_1416776d0
    Replacement::draft(0x141677910, "unknown_141677910", fn_141677910 as *const ()), // sub_141677910
    Replacement::draft(0x141677ae0, "unknown_141677ae0", fn_141677ae0 as *const ()), // sub_141677ae0
    Replacement::draft(0x141677d90, "unknown_141677d90", fn_141677d90 as *const ()), // sub_141677d90
    Replacement::draft(0x141677f00, "unknown_141677f00", fn_141677f00 as *const ()), // sub_141677f00
    Replacement::draft(0x141677f30, "unknown_141677f30", fn_141677f30 as *const ()), // sub_141677f30
    Replacement::draft(0x141677f70, "unknown_141677f70", fn_141677f70 as *const ()), // sub_141677f70
    Replacement::draft(0x141677fb0, "unknown_141677fb0", fn_141677fb0 as *const ()), // sub_141677fb0
    Replacement::draft(0x141678280, "unknown_141678280", fn_141678280 as *const ()), // sub_141678280
    Replacement::draft(0x141678290, "unknown_141678290", tpl_31257::<0x18> as *const ()), // sub_141678290
    Replacement::draft(0x1416782a0, "unknown_1416782a0", tpl_31257::<0x1c> as *const ()), // sub_1416782a0
    Replacement::draft(0x1416782b0, "unknown_1416782b0", fn_1416782b0 as *const ()), // sub_1416782b0
    Replacement::draft(0x1416783a0, "unknown_1416783a0", fn_1416783a0 as *const ()), // sub_1416783a0
    Replacement::draft(0x1416784f0, "unknown_1416784f0", fn_1416784f0 as *const ()), // sub_1416784f0
    Replacement::draft(0x141678530, "unknown_141678530", fn_141678530 as *const ()), // sub_141678530
    Replacement::draft(0x141678600, "unknown_141678600", fn_141678600 as *const ()), // sub_141678600
    Replacement::draft(0x141678680, "unknown_141678680", tpl_31262::<0xffffffffffffffe8, 0x1415242e0, 0x141524330, 0x1415ede90> as *const ()), // sub_141678680
    Replacement::draft(0x141678710, "unknown_141678710", tpl_31262::<0x40, 0x1415242e0, 0x141524330, 0x1415ede90> as *const ()), // sub_141678710
    Replacement::draft(0x1416787a0, "unknown_1416787a0", fn_1416787a0 as *const ()), // sub_1416787a0
    Replacement::draft(0x141678830, "unknown_141678830", fn_141678830 as *const ()), // sub_141678830
    Replacement::draft(0x141678850, "unknown_141678850", fn_141678850 as *const ()), // sub_141678850
    Replacement::draft(0x1416788a0, "unknown_1416788a0", fn_1416788a0 as *const ()), // sub_1416788a0
    Replacement::draft(0x141678900, "unknown_141678900", fn_141678900 as *const ()), // sub_141678900
    Replacement::draft(0x141678970, "unknown_141678970", fn_141678970 as *const ()), // sub_141678970
    Replacement::draft(0x141678a70, "unknown_141678a70", fn_141678a70 as *const ()), // sub_141678a70
    Replacement::draft(0x141678ac0, "unknown_141678ac0", fn_141678ac0 as *const ()), // sub_141678ac0
    Replacement::draft(0x141678b60, "unknown_141678b60", fn_141678b60 as *const ()), // sub_141678b60
    Replacement::draft(0x141678bd0, "unknown_141678bd0", fn_141678bd0 as *const ()), // sub_141678bd0
    Replacement::draft(0x141678c20, "ak_141678c20", tpl_26762::<0x141675840, 0x142693f28> as *const ()), // sub_141678c20
    Replacement::draft(0x141678c44, "unknown_141678c44", tpl_28339::<0x141678c60> as *const ()), // sub_141678c44
    Replacement::draft(0x141678c50, "unknown_141678c50", tpl_28339::<0x141678cf0> as *const ()), // sub_141678c50
    Replacement::draft(0x141678c60, "unknown_141678c60", fn_141678c60 as *const ()), // sub_141678c60
    Replacement::draft(0x141678cb0, "unknown_141678cb0", fn_141678cb0 as *const ()), // sub_141678cb0
    Replacement::draft(0x141678cf0, "unknown_141678cf0", fn_141678cf0 as *const ()), // sub_141678cf0
    Replacement::draft(0x141678d40, "unknown_141678d40", tpl_31272::<0x30, 0x1426452b0, 0xd8, 0xe0, 0x1426452a8> as *const ()), // sub_141678d40
    Replacement::draft(0x141678d90, "unknown_141678d90", tpl_31272::<0x30, 0x1426452b0, 0xc0, 0xc8, 0x1426452a8> as *const ()), // sub_141678d90
    Replacement::draft(0x141678de0, "unknown_141678de0", fn_141678de0 as *const ()), // sub_141678de0
    Replacement::draft(0x141678e60, "unknown_141678e60", fn_141678e60 as *const ()), // sub_141678e60
    Replacement::draft(0x141678e70, "unknown_141678e70", fn_141678e70 as *const ()), // sub_141678e70
    Replacement::draft(0x141678ef0, "unknown_141678ef0", fn_141678ef0 as *const ()), // sub_141678ef0
    Replacement::draft(0x141678f20, "unknown_141678f20", fn_141678f20 as *const ()), // sub_141678f20
    Replacement::draft(0x141679140, "unknown_141679140", fn_141679140 as *const ()), // sub_141679140
    Replacement::draft(0x1416791d0, "unknown_1416791d0", fn_1416791d0 as *const ()), // sub_1416791d0
    Replacement::draft(0x141679310, "unknown_141679310", fn_141679310 as *const ()), // sub_141679310
    Replacement::draft(0x1416794b0, "unknown_1416794b0", fn_1416794b0 as *const ()), // sub_1416794b0
    Replacement::draft_throws(0x1416794f0, "unknown_1416794f0", tpl_838::<0x141677050> as *const (), Throws::Discard), // sub_1416794f0
    Replacement::draft(0x141679500, "unknown_141679500", fn_141679500 as *const ()), // sub_141679500
    Replacement::draft(0x141679540, "unknown_141679540", fn_141679540 as *const ()), // sub_141679540
    Replacement::draft(0x1416796c0, "unknown_1416796c0", fn_1416796c0 as *const ()), // sub_1416796c0
    Replacement::draft(0x1416796f0, "unknown_1416796f0", fn_1416796f0 as *const ()), // sub_1416796f0
    Replacement::draft(0x1416798b0, "unknown_1416798b0", fn_1416798b0 as *const ()), // sub_1416798b0
    Replacement::draft(0x141679a70, "unknown_141679a70", fn_141679a70 as *const ()), // sub_141679a70
    Replacement::draft(0x141679ad0, "unknown_141679ad0", fn_141679ad0 as *const ()), // sub_141679ad0
    Replacement::draft(0x141679bb0, "unknown_141679bb0", fn_141679bb0 as *const ()), // sub_141679bb0
    Replacement::draft(0x141679c70, "unknown_141679c70", fn_141679c70 as *const ()), // sub_141679c70
    Replacement::draft(0x141679d10, "unknown_141679d10", fn_141679d10 as *const ()), // sub_141679d10
    Replacement::draft(0x141679e40, "unknown_141679e40", fn_141679e40 as *const ()), // sub_141679e40
    Replacement::draft(0x141679f30, "ak_141679f30", fn_141679f30 as *const ()), // sub_141679f30
    Replacement::draft(0x141679f70, "unknown_141679f70", tpl_28339::<0x141679f90> as *const ()), // sub_141679f70
    Replacement::draft(0x141679f7c, "unknown_141679f7c", tpl_28339::<0x14167a000> as *const ()), // sub_141679f7c
    Replacement::draft(0x141679f90, "unknown_141679f90", fn_141679f90 as *const ()), // sub_141679f90
    Replacement::draft(0x14167a000, "unknown_14167a000", fn_14167a000 as *const ()), // sub_14167a000
    Replacement::draft(0x14167a070, "unknown_14167a070", fn_14167a070 as *const ()), // sub_14167a070
    Replacement::draft(0x14167a0b0, "unknown_14167a0b0", fn_14167a0b0 as *const ()), // sub_14167a0b0
    Replacement::draft(0x14167a0f0, "unknown_14167a0f0", tpl_31292::<0x30, 0x1426452b0, 0xf0, 0x1426452a8> as *const ()), // sub_14167a0f0
    Replacement::draft(0x14167a130, "unknown_14167a130", tpl_31292::<0x30, 0x1426452b0, 0xd8, 0x1426452a8> as *const ()), // sub_14167a130
    Replacement::draft(0x14167a170, "unknown_14167a170", fn_14167a170 as *const ()), // sub_14167a170
    Replacement::draft(0x14167a210, "unknown_14167a210", fn_14167a210 as *const ()), // sub_14167a210
    Replacement::draft(0x14167a230, "unknown_14167a230", fn_14167a230 as *const ()), // sub_14167a230
    Replacement::draft(0x14167a410, "unknown_14167a410", fn_14167a410 as *const ()), // sub_14167a410
    Replacement::draft(0x14167a550, "unknown_14167a550", fn_14167a550 as *const ()), // sub_14167a550
    Replacement::draft(0x14167a5d0, "unknown_14167a5d0", fn_14167a5d0 as *const ()), // sub_14167a5d0
    Replacement::draft(0x14167a870, "unknown_14167a870", fn_14167a870 as *const ()), // sub_14167a870
    Replacement::draft(0x14167a8e0, "unknown_14167a8e0", fn_14167a8e0 as *const ()), // sub_14167a8e0
    Replacement::draft(0x14167a980, "unknown_14167a980", fn_14167a980 as *const ()), // sub_14167a980
    Replacement::draft(0x14167a9d0, "unknown_14167a9d0", fn_14167a9d0 as *const ()), // sub_14167a9d0
    Replacement::draft(0x14167acc0, "unknown_14167acc0", tpl_31302::<0xe0, 0x14> as *const ()), // sub_14167acc0
    Replacement::draft(0x14167ace0, "unknown_14167ace0", tpl_31302::<0xc8, 0x14> as *const ()), // sub_14167ace0
    Replacement::draft(0x14167ad00, "unknown_14167ad00", fn_14167ad00 as *const ()), // sub_14167ad00
    Replacement::draft_throws(0x14167ad50, "unknown_14167ad50", fn_14167ad50 as *const (), Throws::Discard), // sub_14167ad50
    Replacement::draft(0x14167ae20, "unknown_14167ae20", fn_14167ae20 as *const ()), // sub_14167ae20
    Replacement::draft(0x14167ae70, "unknown_14167ae70", tpl_5167::<0x8> as *const ()), // sub_14167ae70
    Replacement::draft(0x14167ae80, "unknown_14167ae80", fn_14167ae80 as *const ()), // sub_14167ae80
    Replacement::draft(0x14167b170, "unknown_14167b170", tpl_31307::<0x14, 0xe0, 0xd8, 0xf0> as *const ()), // sub_14167b170
    Replacement::draft(0x14167b240, "unknown_14167b240", tpl_31307::<0x14, 0xc8, 0xc0, 0xd8> as *const ()), // sub_14167b240
    Replacement::draft(0x14167b310, "unknown_14167b310", fn_14167b310 as *const ()), // sub_14167b310
    Replacement::draft(0x14167b4e0, "unknown_14167b4e0", fn_14167b4e0 as *const ()), // sub_14167b4e0
    Replacement::draft(0x14167b760, "unknown_14167b760", fn_14167b760 as *const ()), // sub_14167b760
    Replacement::draft(0x14167b910, "unknown_14167b910", fn_14167b910 as *const ()), // sub_14167b910
    Replacement::draft(0x14167ba00, "unknown_14167ba00", fn_14167ba00 as *const ()), // sub_14167ba00
    Replacement::draft(0x14167bb00, "unknown_14167bb00", fn_14167bb00 as *const ()), // sub_14167bb00
    Replacement::draft(0x14167bc00, "unknown_14167bc00", fn_14167bc00 as *const ()), // sub_14167bc00
    Replacement::draft(0x14167bd30, "unknown_14167bd30", tpl_90506::<0xe0, 0x14, 0xd8, 0xf0, 0x141675d10, 0x60, 0x14167c430, 0x14167acc0> as *const ()), // sub_14167bd30
    Replacement::draft(0x14167be10, "unknown_14167be10", tpl_90506::<0xc8, 0x14, 0xc0, 0xd8, 0x141675dd0, 0x60, 0x14167c430, 0x14167ace0> as *const ()), // sub_14167be10
    Replacement::draft(0x14167bef0, "unknown_14167bef0", fn_14167bef0 as *const ()), // sub_14167bef0
    Replacement::draft(0x14167c000, "unknown_14167c000", fn_14167c000 as *const ()), // sub_14167c000
    Replacement::draft(0x14167c130, "unknown_14167c130", fn_14167c130 as *const ()), // sub_14167c130
    Replacement::draft(0x14167c230, "unknown_14167c230", fn_14167c230 as *const ()), // sub_14167c230
    Replacement::draft(0x14167c250, "unknown_14167c250", fn_14167c250 as *const ()), // sub_14167c250
    Replacement::draft(0x14167c2a0, "unknown_14167c2a0", fn_14167c2a0 as *const ()), // sub_14167c2a0
    Replacement::draft(0x14167c2d0, "unknown_14167c2d0", fn_14167c2d0 as *const ()), // sub_14167c2d0
    Replacement::draft(0x14167c2f0, "unknown_14167c2f0", fn_14167c2f0 as *const ()), // sub_14167c2f0
    Replacement::draft(0x14167c350, "unknown_14167c350", fn_14167c350 as *const ()), // sub_14167c350
    Replacement::draft(0x14167c3a0, "unknown_14167c3a0", tpl_31324::<0x1426452b0, 0x64, 0x58, 0x142645550, 0x18, 0x1426452a8> as *const ()), // sub_14167c3a0
    Replacement::draft(0x14167c3e0, "unknown_14167c3e0", fn_14167c3e0 as *const ()), // sub_14167c3e0
    Replacement::draft(0x14167c430, "unknown_14167c430", fn_14167c430 as *const ()), // sub_14167c430
    Replacement::draft(0x14167c480, "unknown_14167c480", fn_14167c480 as *const ()), // sub_14167c480
    Replacement::draft(0x14167c5b0, "unknown_14167c5b0", fn_14167c5b0 as *const ()), // sub_14167c5b0
    Replacement::draft_throws(0x14167c600, "unknown_14167c600", fn_14167c600 as *const (), Throws::Discard), // sub_14167c600
    Replacement::draft(0x14167c740, "unknown_14167c740", fn_14167c740 as *const ()), // sub_14167c740
    Replacement::draft(0x14167c760, "unknown_14167c760", fn_14167c760 as *const ()), // sub_14167c760
    Replacement::draft(0x14167c780, "unknown_14167c780", fn_14167c780 as *const ()), // sub_14167c780
    Replacement::draft(0x14167c7d0, "unknown_14167c7d0", fn_14167c7d0 as *const ()), // sub_14167c7d0
    Replacement::draft(0x14167c7e0, "unknown_14167c7e0", tpl_31324::<0x1426452b0, 0x60, 0x50, 0x142645550, 0x18, 0x1426452a8> as *const ()), // sub_14167c7e0
    Replacement::draft(0x14167c820, "unknown_14167c820", fn_14167c820 as *const ()), // sub_14167c820
    Replacement::draft(0x14167c860, "unknown_14167c860", fn_14167c860 as *const ()), // sub_14167c860
    Replacement::draft(0x14167c920, "unknown_14167c920", fn_14167c920 as *const ()), // sub_14167c920
    Replacement::draft(0x14167c930, "unknown_14167c930", fn_14167c930 as *const ()), // sub_14167c930
    Replacement::draft(0x14167cbb0, "unknown_14167cbb0", fn_14167cbb0 as *const ()), // sub_14167cbb0
    Replacement::draft(0x14167cc00, "unknown_14167cc00", tpl_1674::<0x1426945d0> as *const ()), // sub_14167cc00
    Replacement::draft(0x14167cc10, "unknown_14167cc10", fn_14167cc10 as *const ()), // sub_14167cc10
    Replacement::draft(0x14167cc40, "unknown_14167cc40", fn_14167cc40 as *const ()), // sub_14167cc40
    Replacement::draft(0x14167ccb0, "unknown_14167ccb0", fn_14167ccb0 as *const ()), // sub_14167ccb0
    Replacement::draft(0x14167ce40, "unknown_14167ce40", fn_14167ce40 as *const ()), // sub_14167ce40
    Replacement::draft(0x14167d050, "unknown_14167d050", fn_14167d050 as *const ()), // sub_14167d050
    Replacement::draft(0x14167d070, "unknown_14167d070", fn_14167d070 as *const ()), // sub_14167d070
    Replacement::draft(0x14167d130, "unknown_14167d130", fn_14167d130 as *const ()), // sub_14167d130
    Replacement::draft(0x14167d330, "unknown_14167d330", fn_14167d330 as *const ()), // sub_14167d330
    Replacement::draft(0x14167d560, "unknown_14167d560", fn_14167d560 as *const ()), // sub_14167d560
    Replacement::draft(0x14167d610, "unknown_14167d610", fn_14167d610 as *const ()), // sub_14167d610
    Replacement::draft(0x14167d780, "unknown_14167d780", fn_14167d780 as *const ()), // sub_14167d780
    Replacement::draft(0x14167d860, "unknown_14167d860", fn_14167d860 as *const ()), // sub_14167d860
    Replacement::draft(0x14167d920, "unknown_14167d920", fn_14167d920 as *const ()), // sub_14167d920
    Replacement::draft(0x14167d9d0, "unknown_14167d9d0", fn_14167d9d0 as *const ()), // sub_14167d9d0
    Replacement::draft(0x14167db20, "unknown_14167db20", fn_14167db20 as *const ()), // sub_14167db20
    Replacement::draft(0x14167de10, "unknown_14167de10", fn_14167de10 as *const ()), // sub_14167de10
    Replacement::draft(0x14167df40, "unknown_14167df40", fn_14167df40 as *const ()), // sub_14167df40
    Replacement::draft(0x14167e020, "unknown_14167e020", fn_14167e020 as *const ()), // sub_14167e020
    Replacement::draft(0x14167e110, "unknown_14167e110", fn_14167e110 as *const ()), // sub_14167e110
    Replacement::draft(0x14167e260, "unknown_14167e260", fn_14167e260 as *const ()), // sub_14167e260
    Replacement::draft(0x14167e370, "concurrency_14167e370", fn_14167e370 as *const ()), // Concurrency::details::UMSFreeThreadProxyFactory::Concurrency::details::UMSFreeThreadProxyFactory::`scalar_deleting_destr
    Replacement::draft(0x14167e3c0, "ak_14167e3c0", fn_14167e3c0 as *const ()), // sub_14167e3c0
    Replacement::draft(0x14167e430, "ak_14167e430", fn_14167e430 as *const ()), // sub_14167e430
    Replacement::draft(0x14167e510, "ak_14167e510", fn_14167e510 as *const ()), // sub_14167e510
    Replacement::draft(0x14167e5e0, "ak_14167e5e0", fn_14167e5e0 as *const ()), // sub_14167e5e0
    Replacement::draft(0x14167e600, "ak_14167e600", fn_14167e600 as *const ()), // sub_14167e600
    Replacement::draft(0x14167e670, "ak_14167e670", tpl_28029::<0x1426946e0, 0x3e, 0x8, 0x1426946b0> as *const ()), // sub_14167e670
    Replacement::draft(0x14167e6b0, "ak_14167e6b0", fn_14167e6b0 as *const ()), // sub_14167e6b0
    Replacement::draft(0x14167e700, "ak_14167e700", fn_14167e700 as *const ()), // sub_14167e700
    Replacement::draft(0x14167e8d0, "ak_14167e8d0", fn_14167e8d0 as *const ()), // sub_14167e8d0
    Replacement::draft(0x14167e970, "ak_14167e970", fn_14167e970 as *const ()), // sub_14167e970
    Replacement::draft(0x14167eb60, "ak_14167eb60", fn_14167eb60 as *const ()), // sub_14167eb60
    Replacement::draft(0x14167ee30, "ak_14167ee30", fn_14167ee30 as *const ()), // sub_14167ee30
    Replacement::draft(0x14167f120, "ak_14167f120", fn_14167f120 as *const ()), // sub_14167f120
    Replacement::draft(0x14167f4d0, "ak_14167f4d0", fn_14167f4d0 as *const ()), // sub_14167f4d0
    Replacement::draft(0x14167f5e0, "ak_14167f5e0", fn_14167f5e0 as *const ()), // sub_14167f5e0
    Replacement::draft(0x14167f750, "ak_14167f750", fn_14167f750 as *const ()), // sub_14167f750
    Replacement::draft(0x14167f7b0, "ak_14167f7b0", fn_14167f7b0 as *const ()), // sub_14167f7b0
    Replacement::draft(0x14167f820, "ak_14167f820", fn_14167f820 as *const ()), // sub_14167f820
    Replacement::draft(0x14167f9f0, "ak_14167f9f0", fn_14167f9f0 as *const ()), // sub_14167f9f0
    Replacement::draft(0x14167fa90, "ak_14167fa90", fn_14167fa90 as *const ()), // sub_14167fa90
    Replacement::draft(0x14167fb90, "ak_14167fb90", fn_14167fb90 as *const ()), // sub_14167fb90
    Replacement::draft(0x14167fd50, "ak_14167fd50", fn_14167fd50 as *const ()), // sub_14167fd50
    Replacement::draft(0x14167fe00, "ak_14167fe00", fn_14167fe00 as *const ()), // sub_14167fe00
    Replacement::draft(0x14167fee0, "ak_14167fee0", fn_14167fee0 as *const ()), // sub_14167fee0
    Replacement::draft(0x14167ffc0, "ak_14167ffc0", fn_14167ffc0 as *const ()), // sub_14167ffc0
    Replacement::draft(0x1416803f0, "ak_1416803f0", fn_1416803f0 as *const ()), // sub_1416803f0
    Replacement::draft(0x1416806a0, "ak_1416806a0", tpl_31368::<0xb0, 0xc, 0x14167e970, 0x8, 0x14167fa90, 0x3c, 0x14167f9f0> as *const ()), // sub_1416806a0
    Replacement::draft(0x141680750, "ak_141680750", fn_141680750 as *const ()), // sub_141680750
    Replacement::draft(0x141680b00, "ak_141680b00", tpl_31368::<0xb0, 0xc, 0x14167ee30, 0x8, 0x14167fa90, 0x3c, 0x14167f9f0> as *const ()), // sub_141680b00
    Replacement::draft(0x141680bb0, "ak_141680bb0", tpl_31368::<0xb0, 0xc, 0x14167f120, 0x8, 0x14167fa90, 0x3c, 0x14167f9f0> as *const ()), // sub_141680bb0
    Replacement::draft(0x141680c60, "ak_141680c60", tpl_31368::<0xb0, 0xc, 0x14167eb60, 0x8, 0x14167fa90, 0x3c, 0x14167f9f0> as *const ()), // sub_141680c60
    Replacement::draft(0x141680d10, "ak_141680d10", fn_141680d10 as *const ()), // sub_141680d10
    Replacement::draft(0x141680df0, "ak_141680df0", fn_141680df0 as *const ()), // sub_141680df0
    Replacement::draft(0x141681010, "ak_141681010", fn_141681010 as *const ()), // sub_141681010
    Replacement::draft(0x141681060, "ak_141681060", fn_141681060 as *const ()), // sub_141681060
    Replacement::draft(0x141681150, "ak_141681150", fn_141681150 as *const ()), // sub_141681150
    Replacement::draft(0x1416812a0, "ak_1416812a0", fn_1416812a0 as *const ()), // sub_1416812a0
    Replacement::draft(0x141681300, "ak_141681300", fn_141681300 as *const ()), // sub_141681300
    Replacement::draft(0x141681400, "ak_141681400", fn_141681400 as *const ()), // sub_141681400
    Replacement::draft(0x1416814f0, "ak_1416814f0", fn_1416814f0 as *const ()), // sub_1416814f0
    Replacement::draft(0x141681820, "ak_141681820", fn_141681820 as *const ()), // sub_141681820
    Replacement::draft(0x141681840, "ak_141681840", fn_141681840 as *const ()), // sub_141681840
    Replacement::draft(0x141681a00, "ak_141681a00", fn_141681a00 as *const ()), // sub_141681a00
    Replacement::draft(0x141681a30, "ak_141681a30", fn_141681a30 as *const ()), // sub_141681a30
    Replacement::draft(0x141681a90, "ak_141681a90", fn_141681a90 as *const ()), // sub_141681a90
    Replacement::draft(0x141681b40, "ak_141681b40", fn_141681b40 as *const ()), // sub_141681b40
    Replacement::draft(0x141681bd0, "ak_141681bd0", fn_141681bd0 as *const ()), // sub_141681bd0
    Replacement::draft(0x141681c30, "ak_141681c30", fn_141681c30 as *const ()), // sub_141681c30
    Replacement::draft(0x141681c50, "ak_141681c50", fn_141681c50 as *const ()), // sub_141681c50
    Replacement::draft(0x141681db0, "ak_141681db0", fn_141681db0 as *const ()), // sub_141681db0
    Replacement::draft(0x141681e70, "ak_141681e70", fn_141681e70 as *const ()), // sub_141681e70
    Replacement::draft(0x141681ea0, "ak_141681ea0", fn_141681ea0 as *const ()), // sub_141681ea0
    Replacement::draft(0x141681ed0, "ak_141681ed0", fn_141681ed0 as *const ()), // sub_141681ed0
    Replacement::draft(0x1416827c0, "ak_1416827c0", fn_1416827c0 as *const ()), // sub_1416827c0
    Replacement::draft(0x141683510, "ak_141683510", fn_141683510 as *const ()), // sub_141683510
    Replacement::draft(0x141683790, "ak_141683790", fn_141683790 as *const ()), // sub_141683790
    Replacement::draft(0x1416837f0, "ak_1416837f0", fn_1416837f0 as *const ()), // sub_1416837f0
    Replacement::draft(0x1416838d0, "ak_1416838d0", fn_1416838d0 as *const ()), // sub_1416838d0
    Replacement::draft(0x141683920, "ak_141683920", fn_141683920 as *const ()), // sub_141683920
    Replacement::draft(0x141683a00, "ak_141683a00", fn_141683a00 as *const ()), // sub_141683a00
    Replacement::draft(0x141683a60, "ak_141683a60", fn_141683a60 as *const ()), // sub_141683a60
    Replacement::draft(0x141683ac0, "ak_141683ac0", tpl_28103::<0x141685df0, 0x14342698c, 0x143427094, 0x143426c2c, 0x1434268d0, 0x1434266b0, 0x143428720> as *const ()), // sub_141683ac0
    Replacement::draft(0x141683d10, "ak_141683d10", tpl_28104::<0x141683ac0> as *const ()), // sub_141683d10
    Replacement::draft(0x141683d20, "ak_141683d20", fn_141683d20 as *const ()), // sub_141683d20
    Replacement::draft(0x141685950, "ak_141685950", fn_141685950 as *const ()), // sub_141685950
    Replacement::draft(0x1416859a0, "ak_1416859a0", fn_1416859a0 as *const ()), // sub_1416859a0
    Replacement::draft(0x141685a20, "ak_141685a20", fn_141685a20 as *const ()), // sub_141685a20
    Replacement::draft(0x141685af0, "ak_141685af0", fn_141685af0 as *const ()), // sub_141685af0
    Replacement::draft(0x141685bc0, "ak_141685bc0", fn_141685bc0 as *const ()), // sub_141685bc0
    Replacement::draft(0x141685c50, "ak_141685c50", fn_141685c50 as *const ()), // sub_141685c50
    Replacement::draft(0x141685df0, "ak_141685df0", fn_141685df0 as *const ()), // sub_141685df0
    Replacement::draft(0x141685fd0, "ak_141685fd0", tpl_28109::<0x1416861c0, 0x14342698c, 0x14267564c, 0x143428720, 0x142675648, 0x142675644, 0x142675640> as *const ()), // sub_141685fd0
    Replacement::draft(0x1416861b0, "ak_1416861b0", tpl_28104::<0x141685fd0> as *const ()), // sub_1416861b0
    Replacement::draft(0x1416861c0, "ak_1416861c0", fn_1416861c0 as *const ()), // sub_1416861c0
    Replacement::draft(0x141686450, "ak_141686450", fn_141686450 as *const ()), // sub_141686450
    Replacement::draft(0x141686bb0, "ak_141686bb0", fn_141686bb0 as *const ()), // sub_141686bb0
    Replacement::draft(0x141686c00, "ak_141686c00", fn_141686c00 as *const ()), // sub_141686c00
    Replacement::draft(0x141686c80, "ak_141686c80", fn_141686c80 as *const ()), // sub_141686c80
    Replacement::draft(0x141686dd0, "ak_141686dd0", fn_141686dd0 as *const ()), // sub_141686dd0
    Replacement::draft(0x141686e10, "ak_141686e10", fn_141686e10 as *const ()), // sub_141686e10
    Replacement::draft(0x141686e90, "ak_141686e90", fn_141686e90 as *const ()), // sub_141686e90
    Replacement::draft(0x141686ec0, "ak_141686ec0", tpl_586::<0x7c> as *const ()), // sub_141686ec0
    Replacement::draft(0x141686ed0, "ak_141686ed0", fn_141686ed0 as *const ()), // sub_141686ed0
    Replacement::draft(0x141687610, "ak_141687610", fn_141687610 as *const ()), // sub_141687610
    Replacement::draft(0x1416876f0, "ak_1416876f0", fn_1416876f0 as *const ()), // sub_1416876f0
    Replacement::draft(0x141687720, "ak_141687720", fn_141687720 as *const ()), // sub_141687720
    Replacement::draft(0x141687780, "ak_141687780", tpl_27770::<0x142694bc0> as *const ()), // sub_141687780
    Replacement::draft(0x141687790, "ak_141687790", fn_141687790 as *const ()), // sub_141687790
    Replacement::draft(0x141687800, "ak_141687800", fn_141687800 as *const ()), // sub_141687800
    Replacement::draft(0x141687850, "ak_141687850", fn_141687850 as *const ()), // sub_141687850
    Replacement::draft(0x1416879d0, "ak_1416879d0", fn_1416879d0 as *const ()), // sub_1416879d0
    Replacement::draft(0x141687a80, "ak_141687a80", tpl_6310::<0x14, 0x1c> as *const ()), // sub_141687a80
    Replacement::draft(0x141687a90, "ak_141687a90", fn_141687a90 as *const ()), // sub_141687a90
    Replacement::draft(0x141687b20, "ak_141687b20", fn_141687b20 as *const ()), // sub_141687b20
    Replacement::draft(0x141688170, "ak_141688170", fn_141688170 as *const ()), // sub_141688170
    Replacement::draft(0x141688810, "ak_141688810", fn_141688810 as *const ()), // sub_141688810
    Replacement::draft(0x141688de0, "ak_141688de0", fn_141688de0 as *const ()), // sub_141688de0
    Replacement::draft(0x141689000, "ak_141689000", fn_141689000 as *const ()), // sub_141689000
    Replacement::draft(0x1416892c0, "ak_1416892c0", fn_1416892c0 as *const ()), // sub_1416892c0
    Replacement::draft(0x1416895b0, "ak_1416895b0", fn_1416895b0 as *const ()), // sub_1416895b0
    Replacement::draft(0x1416897e0, "ak_1416897e0", fn_1416897e0 as *const ()), // sub_1416897e0
    Replacement::draft(0x141689990, "ak_141689990", fn_141689990 as *const ()), // sub_141689990
    Replacement::draft(0x141689a10, "ak_141689a10", fn_141689a10 as *const ()), // sub_141689a10
    Replacement::draft(0x141689a80, "ak_141689a80", fn_141689a80 as *const ()), // sub_141689a80
    Replacement::draft(0x141689d80, "ak_141689d80", fn_141689d80 as *const ()), // sub_141689d80
    Replacement::draft(0x141689e20, "ak_141689e20", fn_141689e20 as *const ()), // sub_141689e20
    Replacement::draft(0x14168be30, "ak_14168be30", fn_14168be30 as *const ()), // sub_14168be30
    Replacement::draft(0x14168bef0, "ak_14168bef0", fn_14168bef0 as *const ()), // sub_14168bef0
    Replacement::draft(0x14168bf60, "ak_14168bf60", tpl_28029::<0x142694d50, 0x26, 0x8, 0x142694d20> as *const ()), // sub_14168bf60
    Replacement::draft(0x14168bfa0, "unknown_14168bfa0", fn_14168bfa0 as *const ()), // sub_14168bfa0
    Replacement::draft(0x14168bff0, "unknown_14168bff0", tpl_28031::<0x32, 0x2c> as *const ()), // sub_14168bff0
    Replacement::draft(0x14168c000, "unknown_14168c000", fn_14168c000 as *const ()), // sub_14168c000
    Replacement::draft(0x14168c210, "unknown_14168c210", fn_14168c210 as *const ()), // sub_14168c210
    Replacement::draft(0x14168c300, "unknown_14168c300", tpl_28639::<0x141524330, 0x14168d460, 0x1415cde30> as *const ()), // sub_14168c300
    Replacement::draft(0x14168c340, "unknown_14168c340", tpl_28638::<0xfffffffffffffffd, 0x141524330, 0x142694dc8> as *const ()), // sub_14168c340
    Replacement::draft(0x14168c370, "unknown_14168c370", fn_14168c370 as *const ()), // sub_14168c370
    Replacement::draft(0x14168c3a0, "unknown_14168c3a0", fn_14168c3a0 as *const ()), // sub_14168c3a0
    Replacement::draft(0x14168c410, "unknown_14168c410", fn_14168c410 as *const ()), // sub_14168c410
    Replacement::draft(0x14168c470, "unknown_14168c470", fn_14168c470 as *const ()), // sub_14168c470
    Replacement::draft(0x14168c4f0, "unknown_14168c4f0", fn_14168c4f0 as *const ()), // sub_14168c4f0
    Replacement::draft(0x14168c580, "unknown_14168c580", fn_14168c580 as *const ()), // sub_14168c580
    Replacement::draft(0x14168c660, "unknown_14168c660", fn_14168c660 as *const ()), // sub_14168c660
    Replacement::draft(0x14168c6c0, "unknown_14168c6c0", fn_14168c6c0 as *const ()), // sub_14168c6c0
    Replacement::draft(0x14168c710, "unknown_14168c710", fn_14168c710 as *const ()), // sub_14168c710
    Replacement::draft(0x14168c740, "unknown_14168c740", fn_14168c740 as *const ()), // sub_14168c740
    Replacement::draft(0x14168c7e0, "unknown_14168c7e0", tpl_3047::<0x58> as *const ()), // sub_14168c7e0
    Replacement::draft(0x14168c7f0, "unknown_14168c7f0", tpl_31429::<0x8, 0x10> as *const ()), // sub_14168c7f0
    Replacement::draft(0x14168c800, "unknown_14168c800", fn_14168c800 as *const ()), // sub_14168c800
    Replacement::draft(0x14168c820, "unknown_14168c820", fn_14168c820 as *const ()), // sub_14168c820
    Replacement::draft(0x14168c8a0, "unknown_14168c8a0", tpl_31429::<0x8, 0x60> as *const ()), // sub_14168c8a0
    Replacement::draft(0x14168c8b0, "unknown_14168c8b0", fn_14168c8b0 as *const ()), // sub_14168c8b0
    Replacement::draft(0x14168c920, "unknown_14168c920", fn_14168c920 as *const ()), // sub_14168c920
    Replacement::draft(0x14168cc20, "unknown_14168cc20", fn_14168cc20 as *const ()), // sub_14168cc20
    Replacement::draft(0x14168cc40, "unknown_14168cc40", fn_14168cc40 as *const ()), // sub_14168cc40
    Replacement::draft(0x14168ccc0, "unknown_14168ccc0", fn_14168ccc0 as *const ()), // sub_14168ccc0
    Replacement::draft(0x14168ccd0, "unknown_14168ccd0", fn_14168ccd0 as *const ()), // sub_14168ccd0
    Replacement::draft(0x14168ce00, "unknown_14168ce00", fn_14168ce00 as *const ()), // sub_14168ce00
    Replacement::draft(0x14168ce50, "unknown_14168ce50", fn_14168ce50 as *const ()), // sub_14168ce50
    Replacement::draft(0x14168cf00, "unknown_14168cf00", fn_14168cf00 as *const ()), // sub_14168cf00
    Replacement::draft(0x14168cf40, "unknown_14168cf40", fn_14168cf40 as *const ()), // sub_14168cf40
    Replacement::draft(0x14168d430, "unknown_14168d430", fn_14168d430 as *const ()), // sub_14168d430
    Replacement::draft(0x14168d460, "unknown_14168d460", fn_14168d460 as *const ()), // sub_14168d460
    Replacement::draft(0x14168d510, "unknown_14168d510", fn_14168d510 as *const ()), // sub_14168d510
    Replacement::draft(0x14168d5d0, "unknown_14168d5d0", fn_14168d5d0 as *const ()), // sub_14168d5d0
    Replacement::draft(0x14168d850, "unknown_14168d850", fn_14168d850 as *const ()), // sub_14168d850
    Replacement::draft(0x14168daa0, "unknown_14168daa0", fn_14168daa0 as *const ()), // sub_14168daa0
    Replacement::draft(0x14168dc40, "unknown_14168dc40", fn_14168dc40 as *const ()), // sub_14168dc40
    Replacement::draft(0x14168dcf0, "unknown_14168dcf0", fn_14168dcf0 as *const ()), // sub_14168dcf0
    Replacement::draft(0x14168dd10, "unknown_14168dd10", fn_14168dd10 as *const ()), // sub_14168dd10
    Replacement::draft(0x14168ddd0, "unknown_14168ddd0", fn_14168ddd0 as *const ()), // sub_14168ddd0
    Replacement::draft(0x14168de10, "unknown_14168de10", fn_14168de10 as *const ()), // sub_14168de10
    Replacement::draft(0x14168de90, "unknown_14168de90", fn_14168de90 as *const ()), // sub_14168de90
    Replacement::draft(0x14168e0e0, "unknown_14168e0e0", fn_14168e0e0 as *const ()), // sub_14168e0e0
    Replacement::draft(0x14168e140, "unknown_14168e140", fn_14168e140 as *const ()), // sub_14168e140
    Replacement::draft(0x14168e1a0, "unknown_14168e1a0", fn_14168e1a0 as *const ()), // sub_14168e1a0
    Replacement::draft(0x14168e2a0, "unknown_14168e2a0", fn_14168e2a0 as *const ()), // sub_14168e2a0
    Replacement::draft(0x14168e420, "unknown_14168e420", fn_14168e420 as *const ()), // sub_14168e420
    Replacement::draft(0x14168e440, "unknown_14168e440", fn_14168e440 as *const ()), // sub_14168e440
    Replacement::draft(0x14168e620, "unknown_14168e620", fn_14168e620 as *const ()), // sub_14168e620
    Replacement::draft(0x14168e6a0, "unknown_14168e6a0", fn_14168e6a0 as *const ()), // sub_14168e6a0
    Replacement::draft(0x14168e790, "unknown_14168e790", fn_14168e790 as *const ()), // sub_14168e790
    Replacement::draft(0x14168ea70, "unknown_14168ea70", fn_14168ea70 as *const ()), // sub_14168ea70
    Replacement::draft(0x14168eca0, "unknown_14168eca0", fn_14168eca0 as *const ()), // sub_14168eca0
    Replacement::draft(0x14168ede0, "unknown_14168ede0", fn_14168ede0 as *const ()), // sub_14168ede0
    Replacement::draft(0x14168ef60, "unknown_14168ef60", fn_14168ef60 as *const ()), // sub_14168ef60
    Replacement::draft(0x14168f020, "unknown_14168f020", fn_14168f020 as *const ()), // sub_14168f020
    Replacement::draft(0x14168f040, "unknown_14168f040", fn_14168f040 as *const ()), // sub_14168f040
    Replacement::draft(0x14168f4b0, "unknown_14168f4b0", fn_14168f4b0 as *const ()), // sub_14168f4b0
    Replacement::draft(0x14168f9b0, "unknown_14168f9b0", fn_14168f9b0 as *const ()), // sub_14168f9b0
    Replacement::draft(0x14168fca0, "unknown_14168fca0", fn_14168fca0 as *const ()), // sub_14168fca0
    Replacement::draft(0x14168fd10, "unknown_14168fd10", fn_14168fd10 as *const ()), // sub_14168fd10
    Replacement::draft(0x1416906e0, "unknown_1416906e0", fn_1416906e0 as *const ()), // sub_1416906e0
    Replacement::draft(0x141692b80, "unknown_141692b80", fn_141692b80 as *const ()), // sub_141692b80
    Replacement::draft(0x141692be0, "unknown_141692be0", fn_141692be0 as *const ()), // sub_141692be0
    Replacement::draft(0x141693740, "unknown_141693740", fn_141693740 as *const ()), // sub_141693740
    Replacement::draft(0x141693b30, "unknown_141693b30", fn_141693b30 as *const ()), // sub_141693b30
    Replacement::draft(0x141693c90, "unknown_141693c90", fn_141693c90 as *const ()), // sub_141693c90
    Replacement::draft(0x141693e10, "unknown_141693e10", fn_141693e10 as *const ()), // sub_141693e10
    Replacement::draft(0x141693fd0, "unknown_141693fd0", fn_141693fd0 as *const ()), // sub_141693fd0
    Replacement::draft(0x1416941e0, "unknown_1416941e0", fn_1416941e0 as *const ()), // sub_1416941e0
    Replacement::draft(0x141694340, "unknown_141694340", fn_141694340 as *const ()), // sub_141694340
    Replacement::draft(0x1416944e0, "unknown_1416944e0", fn_1416944e0 as *const ()), // sub_1416944e0
    Replacement::draft(0x1416946a0, "unknown_1416946a0", fn_1416946a0 as *const ()), // sub_1416946a0
    Replacement::draft(0x1416948c0, "unknown_1416948c0", fn_1416948c0 as *const ()), // sub_1416948c0
    Replacement::draft(0x141694a20, "unknown_141694a20", fn_141694a20 as *const ()), // sub_141694a20
    Replacement::draft(0x141694bb0, "unknown_141694bb0", fn_141694bb0 as *const ()), // sub_141694bb0
    Replacement::draft(0x141694d70, "unknown_141694d70", fn_141694d70 as *const ()), // sub_141694d70
    Replacement::draft(0x141694f80, "unknown_141694f80", fn_141694f80 as *const ()), // sub_141694f80
    Replacement::draft(0x141695100, "unknown_141695100", fn_141695100 as *const ()), // sub_141695100
    Replacement::draft(0x1416952a0, "unknown_1416952a0", fn_1416952a0 as *const ()), // sub_1416952a0
    Replacement::draft(0x141695470, "unknown_141695470", fn_141695470 as *const ()), // sub_141695470
    Replacement::draft(0x141695690, "unknown_141695690", fn_141695690 as *const ()), // sub_141695690
    Replacement::draft(0x1416958c0, "unknown_1416958c0", fn_1416958c0 as *const ()), // sub_1416958c0
    Replacement::draft(0x141695a50, "unknown_141695a50", fn_141695a50 as *const ()), // sub_141695a50
    Replacement::draft(0x141695ca0, "unknown_141695ca0", fn_141695ca0 as *const ()), // sub_141695ca0
    Replacement::draft(0x141696250, "unknown_141696250", fn_141696250 as *const ()), // sub_141696250
    Replacement::draft(0x1416964b0, "unknown_1416964b0", fn_1416964b0 as *const ()), // sub_1416964b0
    Replacement::draft(0x1416966a0, "unknown_1416966a0", fn_1416966a0 as *const ()), // sub_1416966a0
    Replacement::draft(0x1416968c0, "unknown_1416968c0", fn_1416968c0 as *const ()), // sub_1416968c0
    Replacement::draft(0x141696930, "unknown_141696930", fn_141696930 as *const ()), // sub_141696930
    Replacement::draft(0x141696c40, "unknown_141696c40", fn_141696c40 as *const ()), // sub_141696c40
    Replacement::draft(0x141697100, "unknown_141697100", fn_141697100 as *const ()), // sub_141697100
    Replacement::draft(0x141697240, "unknown_141697240", fn_141697240 as *const ()), // sub_141697240
    Replacement::draft(0x1416973c0, "unknown_1416973c0", fn_1416973c0 as *const ()), // sub_1416973c0
    Replacement::draft(0x141697890, "luau_141697890", fn_141697890 as *const ()), // sub_141697890
    Replacement::draft(0x1416980a0, "unknown_1416980a0", fn_1416980a0 as *const ()), // sub_1416980a0
    Replacement::draft(0x1416983a0, "unknown_1416983a0", tpl_365::<0x1416980a0> as *const ()), // sub_1416983a0
    Replacement::draft(0x141698690, "unknown_141698690", fn_141698690 as *const ()), // sub_141698690
    Replacement::draft(0x1416986ad, "crt_1416986ad", tpl_906::<0x142646c38> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnCreateWebSocket
    Replacement::draft(0x1416986b3, "crt_1416986b3", tpl_906::<0x142646c30> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnClipboardTextGet
    Replacement::draft(0x1416986b9, "crt_1416986b9", tpl_906::<0x142646c28> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnClipboardTextSet
    Replacement::draft(0x1416986bf, "crt_1416986bf", tpl_906::<0x142646c10> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnCursorChanged
    Replacement::draft(0x1416986c5, "crt_1416986c5", tpl_906::<0x142646c08> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnCaretRectChanged
    Replacement::draft(0x1416986cb, "crt_1416986cb", tpl_906::<0x142646bc8> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnTextInputTypeChanged
    Replacement::draft(0x1416986d1, "crt_1416986d1", tpl_906::<0x142646bc0> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnAudioStreamVolumeChanged
    Replacement::draft(0x1416986d7, "crt_1416986d7", tpl_906::<0x142646bb8> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnAudioStreamEnded
    Replacement::draft(0x1416986dd, "crt_1416986dd", tpl_906::<0x142646bb0> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnAudioDataReceived
    Replacement::draft(0x1416986e3, "crt_1416986e3", tpl_906::<0x142646ba8> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnAudioStreamPause
    Replacement::draft(0x1416986e9, "crt_1416986e9", tpl_906::<0x142646ba0> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnAudioStreamPlay
    Replacement::draft(0x1416986ef, "crt_1416986ef", tpl_906::<0x142646b98> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnAudioStreamClosed
    Replacement::draft(0x1416986f5, "crt_1416986f5", tpl_906::<0x142646b90> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnAudioStreamCreated
    Replacement::draft(0x1416986fb, "crt_1416986fb", tpl_906::<0x142646b88> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnNodeKeyEvent
    Replacement::draft(0x141698701, "crt_141698701", tpl_906::<0x142646b80> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnNodeTouched
    Replacement::draft(0x141698707, "crt_141698707", tpl_906::<0x142646b78> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnScreenInfoRequested
    Replacement::draft(0x14169870d, "crt_14169870d", tpl_906::<0x142646b70> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnBindingsReleased
    Replacement::draft(0x141698713, "crt_141698713", tpl_906::<0x142646bd0> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnLoadFailed
    Replacement::draft(0x141698719, "crt_141698719", tpl_906::<0x142646bd8> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnDOMBuilt
    Replacement::draft(0x14169871f, "crt_14169871f", tpl_906::<0x142646c18> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnScriptContextCreated
    Replacement::draft(0x141698725, "crt_141698725", tpl_906::<0x142646c20> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnResourceRequestIntercepted
    Replacement::draft(0x14169872b, "crt_14169872b", tpl_906::<0x142646c40> as *const ()), // j_cohtml::IViewListener::j_cohtml::IViewListener::OnNavigateTo
    Replacement::draft(0x141698731, "crt_141698731", tpl_906::<0x142646c60> as *const ()), // j_cohtml::ILocalizationManager::j_cohtml::ILocalizationManager::Translate
    Replacement::draft(0x141698737, "crt_141698737", tpl_906::<0x142646ca0> as *const ()), // j_cohtml::ISyncStreamReader::j_cohtml::ISyncStreamReader::~ISyncStreamReader
    Replacement::draft(0x14169873d, "crt_14169873d", tpl_906::<0x142646cf0> as *const ()), // j_cohtml::IAllocator::j_cohtml::IAllocator::~IAllocator
    Replacement::draft(0x141698743, "crt_141698743", tpl_906::<0x142646d20> as *const ()), // j_cohtml::IInternationalizationManager::j_cohtml::IInternationalizationManager::~IInternationalizationManager
    Replacement::draft(0x141698749, "crt_141698749", tpl_906::<0x142646d50> as *const ()), // j_cohtml::i18n::IBreakIterator::j_cohtml::i18n::IBreakIterator::SetText
    Replacement::draft(0x14169874f, "crt_14169874f", tpl_906::<0x142646d48> as *const ()), // j_cohtml::i18n::IBreakIterator::j_cohtml::i18n::IBreakIterator::ResetToEnd
    Replacement::draft(0x141698755, "crt_141698755", tpl_906::<0x142646ce0> as *const ()), // j_cohtml::i18n::IBreakIterator::j_cohtml::i18n::IBreakIterator::ResetToBeginning
    Replacement::draft(0x14169875b, "crt_14169875b", tpl_906::<0x142646cd0> as *const ()), // j_cohtml::i18n::IBreakIterator::j_cohtml::i18n::IBreakIterator::MoveToPrevious
    Replacement::draft(0x141698761, "crt_141698761", tpl_906::<0x142646c58> as *const ()), // j_cohtml::i18n::IBreakIterator::j_cohtml::i18n::IBreakIterator::MoveToNext
    Replacement::draft(0x141698767, "crt_141698767", tpl_906::<0x142646be8> as *const ()), // j_cohtml::i18n::IBreakIterator::j_cohtml::i18n::IBreakIterator::MoveToCandidateAfter
    Replacement::draft(0x14169876d, "crt_14169876d", tpl_906::<0x142646be0> as *const ()), // j_cohtml::i18n::IBreakIterator::j_cohtml::i18n::IBreakIterator::GetCurrentPosition
    Replacement::draft(0x141698773, "crt_141698773", tpl_906::<0x142646d28> as *const ()), // j_cohtml::i18n::IBreakIterator::j_cohtml::i18n::IBreakIterator::ClearText
    Replacement::draft(0x141698779, "crt_141698779", tpl_906::<0x142646b68> as *const ()), // j_cohtml::IInternationalizationManager::j_cohtml::IInternationalizationManager::ComputeTextDirection
    Replacement::draft(0x14169877f, "crt_14169877f", tpl_906::<0x142646bf0> as *const ()), // j_cohtml::IInternationalizationManager::j_cohtml::IInternationalizationManager::ComputeBaseDirection
    Replacement::draft(0x141698790, "unknown_141698790", fn_141698790 as *const ()), // sub_141698790
    Replacement::draft(0x141698f50, "curl_141698f50", fn_141698f50 as *const ()), // sub_141698f50
    Replacement::draft(0x141699320, "unknown_141699320", fn_141699320 as *const ()), // sub_141699320
    Replacement::draft(0x141699510, "unknown_141699510", fn_141699510 as *const ()), // sub_141699510
    Replacement::draft(0x141699d80, "unknown_141699d80", fn_141699d80 as *const ()), // sub_141699d80
    Replacement::draft(0x141699f20, "unknown_141699f20", fn_141699f20 as *const ()), // sub_141699f20
    Replacement::draft(0x141699fb0, "unknown_141699fb0", fn_141699fb0 as *const ()), // sub_141699fb0
    Replacement::draft(0x14169a060, "unknown_14169a060", fn_14169a060 as *const ()), // sub_14169a060
    Replacement::draft(0x14169a500, "unknown_14169a500", fn_14169a500 as *const ()), // sub_14169a500
    Replacement::draft(0x14169a5c0, "unknown_14169a5c0", fn_14169a5c0 as *const ()), // sub_14169a5c0
    Replacement::draft(0x14169a640, "unknown_14169a640", fn_14169a640 as *const ()), // sub_14169a640
    Replacement::draft(0x14169a6a0, "unknown_14169a6a0", fn_14169a6a0 as *const ()), // sub_14169a6a0
    Replacement::draft(0x14169a720, "unknown_14169a720", fn_14169a720 as *const ()), // sub_14169a720
    Replacement::draft(0x14169a7a0, "unknown_14169a7a0", fn_14169a7a0 as *const ()), // sub_14169a7a0
    Replacement::draft(0x14169a840, "unknown_14169a840", fn_14169a840 as *const ()), // sub_14169a840
    Replacement::draft(0x14169a8a0, "unknown_14169a8a0", fn_14169a8a0 as *const ()), // sub_14169a8a0
    Replacement::draft(0x14169a960, "unknown_14169a960", fn_14169a960 as *const ()), // sub_14169a960
    Replacement::draft(0x14169aa20, "unknown_14169aa20", fn_14169aa20 as *const ()), // sub_14169aa20
    Replacement::draft(0x14169aad0, "unknown_14169aad0", fn_14169aad0 as *const ()), // sub_14169aad0
    Replacement::draft(0x14169abf0, "unknown_14169abf0", fn_14169abf0 as *const ()), // sub_14169abf0
    Replacement::draft(0x14169ac30, "unknown_14169ac30", fn_14169ac30 as *const ()), // sub_14169ac30
    Replacement::draft(0x14169ad70, "unknown_14169ad70", fn_14169ad70 as *const ()), // sub_14169ad70
    Replacement::draft(0x14169b620, "unknown_14169b620", fn_14169b620 as *const ()), // sub_14169b620
    Replacement::draft(0x14169bbc0, "unknown_14169bbc0", fn_14169bbc0 as *const ()), // sub_14169bbc0
    Replacement::draft(0x14169bcb0, "unknown_14169bcb0", fn_14169bcb0 as *const ()), // sub_14169bcb0
    Replacement::draft(0x14169bd10, "unknown_14169bd10", fn_14169bd10 as *const ()), // sub_14169bd10
    Replacement::draft(0x14169bd50, "unknown_14169bd50", fn_14169bd50 as *const ()), // sub_14169bd50
    Replacement::draft_throws(0x14169bd70, "unknown_14169bd70", fn_14169bd70 as *const (), Throws::Discard), // sub_14169bd70
    Replacement::draft(0x14169bdc0, "unknown_14169bdc0", fn_14169bdc0 as *const ()), // sub_14169bdc0
    Replacement::draft(0x14169c130, "nlohmann_14169c130", fn_14169c130 as *const ()), // sub_14169c130
    Replacement::draft_throws(0x14169c360, "unknown_14169c360", fn_14169c360 as *const (), Throws::Discard), // sub_14169c360
    Replacement::draft_throws(0x14169c760, "unknown_14169c760", fn_14169c760 as *const (), Throws::Discard), // sub_14169c760
    Replacement::draft_throws(0x14169cac0, "unknown_14169cac0", fn_14169cac0 as *const (), Throws::Discard), // sub_14169cac0
    Replacement::draft(0x14169cd60, "unknown_14169cd60", fn_14169cd60 as *const ()), // sub_14169cd60
    Replacement::draft(0x14169cdf0, "unknown_14169cdf0", fn_14169cdf0 as *const ()), // sub_14169cdf0
    Replacement::draft(0x14169d070, "unknown_14169d070", fn_14169d070 as *const ()), // sub_14169d070
    Replacement::draft(0x14169d0e0, "unknown_14169d0e0", fn_14169d0e0 as *const ()), // sub_14169d0e0
    Replacement::draft(0x14169d2a0, "unknown_14169d2a0", fn_14169d2a0 as *const ()), // sub_14169d2a0
    Replacement::draft(0x14169d560, "unknown_14169d560", fn_14169d560 as *const ()), // sub_14169d560
    Replacement::draft(0x14169da10, "unknown_14169da10", fn_14169da10 as *const ()), // sub_14169da10
    Replacement::draft(0x14169dcd0, "unknown_14169dcd0", fn_14169dcd0 as *const ()), // sub_14169dcd0
    Replacement::draft(0x14169e0b0, "unknown_14169e0b0", fn_14169e0b0 as *const ()), // sub_14169e0b0
    Replacement::draft(0x14169e110, "unknown_14169e110", fn_14169e110 as *const ()), // sub_14169e110
    Replacement::draft(0x14169e1a0, "unknown_14169e1a0", fn_14169e1a0 as *const ()), // sub_14169e1a0
    Replacement::draft(0x14169e230, "unknown_14169e230", fn_14169e230 as *const ()), // sub_14169e230
    Replacement::draft(0x14169e2e0, "unknown_14169e2e0", tpl_6190::<0x141833c49> as *const ()), // sub_14169e2e0
    Replacement::draft(0x14169e2f0, "unknown_14169e2f0", fn_14169e2f0 as *const ()), // sub_14169e2f0
    Replacement::draft(0x14169e340, "unknown_14169e340", fn_14169e340 as *const ()), // sub_14169e340
    Replacement::draft(0x14169e3a0, "unknown_14169e3a0", tpl_5167::<0x50> as *const ()), // sub_14169e3a0
    Replacement::draft(0x14169e3b0, "unknown_14169e3b0", fn_14169e3b0 as *const ()), // sub_14169e3b0
    Replacement::draft(0x14169e460, "unknown_14169e460", fn_14169e460 as *const ()), // sub_14169e460
    Replacement::draft(0x14169e530, "unknown_14169e530", fn_14169e530 as *const ()), // sub_14169e530
    Replacement::draft(0x14169e5d0, "unknown_14169e5d0", fn_14169e5d0 as *const ()), // sub_14169e5d0
    Replacement::draft(0x14169e600, "unknown_14169e600", fn_14169e600 as *const ()), // sub_14169e600
    Replacement::draft(0x14169e740, "unknown_14169e740", fn_14169e740 as *const ()), // sub_14169e740
    Replacement::draft(0x14169e850, "unknown_14169e850", fn_14169e850 as *const ()), // sub_14169e850
    Replacement::draft(0x14169e980, "unknown_14169e980", fn_14169e980 as *const ()), // sub_14169e980
    Replacement::draft(0x14169ea80, "unknown_14169ea80", fn_14169ea80 as *const ()), // sub_14169ea80
    Replacement::draft(0x14169eb00, "unknown_14169eb00", fn_14169eb00 as *const ()), // sub_14169eb00
    Replacement::draft(0x14169eba0, "unknown_14169eba0", fn_14169eba0 as *const ()), // sub_14169eba0
    Replacement::draft(0x14169eca0, "unknown_14169eca0", fn_14169eca0 as *const ()), // sub_14169eca0
    Replacement::draft(0x14169ed00, "unknown_14169ed00", fn_14169ed00 as *const ()), // sub_14169ed00
    Replacement::draft(0x14169ed80, "kaim_14169ed80", tpl_1674::<0x1433e1370> as *const ()), // sub_14169ed80
    Replacement::draft(0x14169ed90, "kaim_14169ed90", tpl_1674::<0x1433e1448> as *const ()), // sub_14169ed90
    Replacement::draft(0x14169eda0, "kaim_14169eda0", fn_14169eda0 as *const ()), // sub_14169eda0
    Replacement::draft(0x14169edb0, "kaim_14169edb0", fn_14169edb0 as *const ()), // sub_14169edb0
    Replacement::draft(0x14169edd0, "kaim_14169edd0", fn_14169edd0 as *const ()), // sub_14169edd0
    Replacement::draft(0x14169edf0, "kaim_14169edf0", fn_14169edf0 as *const ()), // sub_14169edf0
    Replacement::draft(0x14169ee10, "kaim_14169ee10", fn_14169ee10 as *const ()), // sub_14169ee10
    Replacement::draft(0x14169ee90, "kaim_14169ee90", tpl_31525::<0x14169ee10, 0x18, 0x143932190, 0x60> as *const ()), // sub_14169ee90
    Replacement::draft(0x14169eee0, "kaim_14169eee0", tpl_1674::<0x1426aa560> as *const ()), // sub_14169eee0
    Replacement::draft(0x14169eef0, "kaim_14169eef0", fn_14169eef0 as *const ()), // sub_14169eef0
    Replacement::draft(0x14169ef90, "kaim_14169ef90", fn_14169ef90 as *const ()), // sub_14169ef90
    Replacement::draft(0x14169efe0, "kaim_14169efe0", fn_14169efe0 as *const ()), // sub_14169efe0
    Replacement::draft(0x14169f030, "kaim_14169f030", fn_14169f030 as *const ()), // sub_14169f030
    Replacement::draft(0x14169f1f0, "kaim_14169f1f0", fn_14169f1f0 as *const ()), // sub_14169f1f0
    Replacement::draft(0x14169f340, "kaim_14169f340", fn_14169f340 as *const ()), // sub_14169f340
    Replacement::draft(0x14169f430, "kaim_14169f430", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aa5a0> as *const ()), // sub_14169f430
    Replacement::draft(0x14169f490, "kaim_14169f490", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aa620, 0x8> as *const ()), // sub_14169f490
    Replacement::draft(0x14169f4d0, "kaim_14169f4d0", fn_14169f4d0 as *const ()), // Kaim::NavData::Kaim::NavData::vfunc_3
    Replacement::draft(0x14169f510, "kaim_14169f510", tpl_22446::<0x1416a0670> as *const ()), // sub_14169f510
    Replacement::draft(0x14169f520, "kaim_14169f520", fn_14169f520 as *const ()), // sub_14169f520
    Replacement::draft(0x14169f550, "kaim_14169f550", tpl_1::<0x1426aa588> as *const ()), // sub_14169f550
    Replacement::draft(0x14169f560, "kaim_14169f560", fn_14169f560 as *const ()), // sub_14169f560
    Replacement::draft(0x14169f640, "kaim_14169f640", fn_14169f640 as *const ()), // sub_14169f640
    Replacement::draft(0x14169f670, "kaim_14169f670", fn_14169f670 as *const ()), // sub_14169f670
    Replacement::draft(0x14169f6a0, "kaim_14169f6a0", tpl_548::<0xb0> as *const ()), // sub_14169f6a0
    Replacement::draft(0x14169f6b0, "kaim_14169f6b0", fn_14169f6b0 as *const ()), // sub_14169f6b0
    Replacement::draft(0x14169f7a0, "kaim_14169f7a0", fn_14169f7a0 as *const ()), // sub_14169f7a0
    Replacement::draft(0x14169fa50, "kaim_14169fa50", fn_14169fa50 as *const ()), // sub_14169fa50
    Replacement::draft(0x14169fb10, "kaim_14169fb10", fn_14169fb10 as *const ()), // sub_14169fb10
    Replacement::draft(0x14169fc30, "kaim_14169fc30", tpl_31538::<0x8, 0x1416a0330, 0x14169eda0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_14169fc30
    Replacement::draft(0x14169fcb0, "kaim_14169fcb0", fn_14169fcb0 as *const ()), // sub_14169fcb0
    Replacement::draft(0x1416a0000, "kaim_1416a0000", tpl_90795::<0x8, 0x18, 0x14169eda0, 0x14169edb0, 0x1416a0330, 0x14183acc2> as *const ()), // sub_1416a0000
    Replacement::draft(0x1416a00c0, "kaim_1416a00c0", fn_1416a00c0 as *const ()), // sub_1416a00c0
    Replacement::draft(0x1416a01d0, "kaim_1416a01d0", fn_1416a01d0 as *const ()), // sub_1416a01d0
    Replacement::draft(0x1416a0280, "kaim_1416a0280", fn_1416a0280 as *const ()), // sub_1416a0280
    Replacement::draft(0x1416a0330, "kaim_1416a0330", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x1416a01d0> as *const ()), // sub_1416a0330
    Replacement::draft(0x1416a03d0, "kaim_1416a03d0", fn_1416a03d0 as *const ()), // sub_1416a03d0
    Replacement::draft(0x1416a0670, "kaim_1416a0670", fn_1416a0670 as *const ()), // sub_1416a0670
    Replacement::draft(0x1416a08e0, "kaim_1416a08e0", fn_1416a08e0 as *const ()), // sub_1416a08e0
    Replacement::draft(0x1416a0960, "kaim_1416a0960", fn_1416a0960 as *const ()), // sub_1416a0960
    Replacement::draft(0x1416a0ac0, "kaim_1416a0ac0", tpl_1674::<0x1426aa688> as *const ()), // sub_1416a0ac0
    Replacement::draft(0x1416a0ad0, "kaim_1416a0ad0", tpl_22448::<0x1426aa688, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416a0ad0
    Replacement::draft(0x1416a0b20, "kaim_1416a0b20", fn_1416a0b20 as *const ()), // sub_1416a0b20
    Replacement::draft(0x1416a0b70, "kaim_1416a0b70", fn_1416a0b70 as *const ()), // sub_1416a0b70
    Replacement::draft(0x1416a0c40, "kaim_1416a0c40", fn_1416a0c40 as *const ()), // sub_1416a0c40
    Replacement::draft(0x1416a0df0, "kaim_1416a0df0", fn_1416a0df0 as *const ()), // sub_1416a0df0
    Replacement::draft(0x1416a0e20, "kaim_1416a0e20", fn_1416a0e20 as *const ()), // sub_1416a0e20
    Replacement::draft(0x1416a0e90, "kaim_1416a0e90", tpl_1::<0x1426aa6c8> as *const ()), // sub_1416a0e90
    Replacement::draft(0x1416a0ea0, "kaim_1416a0ea0", fn_1416a0ea0 as *const ()), // sub_1416a0ea0
    Replacement::draft(0x1416a0ed0, "kaim_1416a0ed0", fn_1416a0ed0 as *const ()), // sub_1416a0ed0
    Replacement::draft(0x1416a1200, "kaim_1416a1200", fn_1416a1200 as *const ()), // sub_1416a1200
    Replacement::draft(0x1416a1400, "kaim_1416a1400", fn_1416a1400 as *const ()), // sub_1416a1400
    Replacement::draft(0x1416a14f0, "kaim_1416a14f0", fn_1416a14f0 as *const ()), // sub_1416a14f0
    Replacement::draft(0x1416a1620, "kaim_1416a1620", fn_1416a1620 as *const ()), // sub_1416a1620
    Replacement::draft(0x1416a1640, "kaim_1416a1640", fn_1416a1640 as *const ()), // sub_1416a1640
    Replacement::draft(0x1416a16e0, "kaim_1416a16e0", fn_1416a16e0 as *const ()), // sub_1416a16e0
    Replacement::draft(0x1416a1880, "kaim_1416a1880", fn_1416a1880 as *const ()), // sub_1416a1880
    Replacement::draft(0x1416a18a0, "kaim_1416a18a0", tpl_31538::<0x8, 0x1416a1c10, 0x14169eda0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_1416a18a0
    Replacement::draft(0x1416a1920, "kaim_1416a1920", tpl_90795::<0x8, 0x18, 0x14169eda0, 0x14169edb0, 0x1416a1c10, 0x14183acc2> as *const ()), // sub_1416a1920
    Replacement::draft(0x1416a19e0, "kaim_1416a19e0", fn_1416a19e0 as *const ()), // sub_1416a19e0
    Replacement::draft(0x1416a1b40, "kaim_1416a1b40", fn_1416a1b40 as *const ()), // sub_1416a1b40
    Replacement::draft(0x1416a1bf0, "kaim_1416a1bf0", fn_1416a1bf0 as *const ()), // sub_1416a1bf0
    Replacement::draft(0x1416a1c10, "kaim_1416a1c10", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x1416a1b40> as *const ()), // sub_1416a1c10
    Replacement::draft(0x1416a1cb0, "kaim_1416a1cb0", fn_1416a1cb0 as *const ()), // sub_1416a1cb0
    Replacement::draft(0x1416a1d60, "kaim_1416a1d60", fn_1416a1d60 as *const ()), // sub_1416a1d60
    Replacement::draft(0x1416a20d0, "kaim_1416a20d0", fn_1416a20d0 as *const ()), // sub_1416a20d0
    Replacement::draft(0x1416a2110, "kaim_1416a2110", fn_1416a2110 as *const ()), // sub_1416a2110
    Replacement::draft(0x1416a21e0, "kaim_1416a21e0", fn_1416a21e0 as *const ()), // sub_1416a21e0
    Replacement::draft(0x1416a2220, "kaim_1416a2220", fn_1416a2220 as *const ()), // sub_1416a2220
    Replacement::draft(0x1416a22f0, "kaim_1416a22f0", fn_1416a22f0 as *const ()), // sub_1416a22f0
    Replacement::draft(0x1416a2400, "kaim_1416a2400", fn_1416a2400 as *const ()), // sub_1416a2400
    Replacement::draft(0x1416a24c0, "kaim_1416a24c0", fn_1416a24c0 as *const ()), // sub_1416a24c0
    Replacement::draft(0x1416a2510, "unknown_1416a2510", fn_1416a2510 as *const ()), // sub_1416a2510
    Replacement::draft(0x1416a2610, "kaim_1416a2610", fn_1416a2610 as *const ()), // sub_1416a2610
    Replacement::draft(0x1416a26a0, "kaim_1416a26a0", fn_1416a26a0 as *const ()), // sub_1416a26a0
    Replacement::draft(0x1416a27c0, "unknown_1416a27c0", fn_1416a27c0 as *const ()), // sub_1416a27c0
    Replacement::draft(0x1416a2870, "unknown_1416a2870", fn_1416a2870 as *const ()), // sub_1416a2870
    Replacement::draft(0x1416a2e90, "unknown_1416a2e90", fn_1416a2e90 as *const ()), // sub_1416a2e90
    Replacement::draft(0x1416a2ea0, "unknown_1416a2ea0", fn_1416a2ea0 as *const ()), // sub_1416a2ea0
    Replacement::draft(0x1416a2f50, "unknown_1416a2f50", fn_1416a2f50 as *const ()), // sub_1416a2f50
    Replacement::draft(0x1416a3000, "unknown_1416a3000", fn_1416a3000 as *const ()), // sub_1416a3000
    Replacement::draft(0x1416a30b0, "unknown_1416a30b0", fn_1416a30b0 as *const ()), // sub_1416a30b0
    Replacement::draft(0x1416a30e0, "unknown_1416a30e0", fn_1416a30e0 as *const ()), // sub_1416a30e0
    Replacement::draft(0x1416a3130, "unknown_1416a3130", fn_1416a3130 as *const ()), // sub_1416a3130
    Replacement::draft(0x1416a3160, "kaim_1416a3160", fn_1416a3160 as *const ()), // sub_1416a3160
    Replacement::draft(0x1416a31a0, "kaim_1416a31a0", fn_1416a31a0 as *const ()), // sub_1416a31a0
    Replacement::draft(0x1416a3230, "kaim_1416a3230", fn_1416a3230 as *const ()), // sub_1416a3230
    Replacement::draft(0x1416a32e0, "kaim_1416a32e0", fn_1416a32e0 as *const ()), // sub_1416a32e0
    Replacement::draft(0x1416a3360, "kaim_1416a3360", tpl_22564::<0x74, 0xffffffffffffffff, 0x1416a33d0, 0x143932190, 0x60> as *const ()), // sub_1416a3360
    Replacement::draft(0x1416a33d0, "kaim_1416a33d0", fn_1416a33d0 as *const ()), // sub_1416a33d0
    Replacement::draft(0x1416a35b0, "kaim_1416a35b0", fn_1416a35b0 as *const ()), // sub_1416a35b0
    Replacement::draft(0x1416a41b0, "kaim_1416a41b0", fn_1416a41b0 as *const ()), // sub_1416a41b0
    Replacement::draft(0x1416a4570, "kaim_1416a4570", fn_1416a4570 as *const ()), // sub_1416a4570
    Replacement::draft(0x1416a4590, "kaim_1416a4590", fn_1416a4590 as *const ()), // sub_1416a4590
    Replacement::draft(0x1416a4620, "kaim_1416a4620", fn_1416a4620 as *const ()), // sub_1416a4620
    Replacement::draft(0x1416a46f0, "kaim_1416a46f0", fn_1416a46f0 as *const ()), // sub_1416a46f0
    Replacement::draft(0x1416a4bc0, "kaim_1416a4bc0", fn_1416a4bc0 as *const ()), // sub_1416a4bc0
    Replacement::draft(0x1416a4cb0, "kaim_1416a4cb0", fn_1416a4cb0 as *const ()), // sub_1416a4cb0
    Replacement::draft(0x1416a4ee0, "kaim_1416a4ee0", fn_1416a4ee0 as *const ()), // sub_1416a4ee0
    Replacement::draft(0x1416a4f90, "kaim_1416a4f90", fn_1416a4f90 as *const ()), // sub_1416a4f90
    Replacement::draft(0x1416a5000, "kaim_1416a5000", fn_1416a5000 as *const ()), // sub_1416a5000
    Replacement::draft(0x1416a5130, "kaim_1416a5130", fn_1416a5130 as *const ()), // sub_1416a5130
    Replacement::draft(0x1416a5240, "kaim_1416a5240", fn_1416a5240 as *const ()), // sub_1416a5240
    Replacement::draft(0x1416a53f0, "kaim_1416a53f0", fn_1416a53f0 as *const ()), // sub_1416a53f0
    Replacement::draft(0x1416a5820, "kaim_1416a5820", fn_1416a5820 as *const ()), // sub_1416a5820
    Replacement::draft(0x1416a5b80, "kaim_1416a5b80", fn_1416a5b80 as *const ()), // sub_1416a5b80
    Replacement::draft(0x1416a5d50, "kaim_1416a5d50", fn_1416a5d50 as *const ()), // sub_1416a5d50
    Replacement::draft(0x1416a5dd0, "kaim_1416a5dd0", fn_1416a5dd0 as *const ()), // sub_1416a5dd0
    Replacement::draft(0x1416a5e60, "kaim_1416a5e60", fn_1416a5e60 as *const ()), // sub_1416a5e60
    Replacement::draft(0x1416a5ee0, "kaim_1416a5ee0", fn_1416a5ee0 as *const ()), // sub_1416a5ee0
    Replacement::draft(0x1416a5f60, "kaim_1416a5f60", fn_1416a5f60 as *const ()), // sub_1416a5f60
    Replacement::draft(0x1416a5fe0, "kaim_1416a5fe0", fn_1416a5fe0 as *const ()), // sub_1416a5fe0
    Replacement::draft(0x1416a6070, "kaim_1416a6070", tpl_365::<0x1416a5dd0> as *const ()), // sub_1416a6070
    Replacement::draft(0x1416a6080, "kaim_1416a6080", tpl_365::<0x1416a5ee0> as *const ()), // sub_1416a6080
    Replacement::draft(0x1416a6090, "kaim_1416a6090", tpl_22608::<0x8, 0x1416a7790, 0x143932190, 0x60> as *const ()), // sub_1416a6090
    Replacement::draft(0x1416a60d0, "kaim_1416a60d0", tpl_31603::<0x143932190, 0x60> as *const ()), // sub_1416a60d0
    Replacement::draft(0x1416a60f0, "kaim_1416a60f0", fn_1416a60f0 as *const ()), // sub_1416a60f0
    Replacement::draft(0x1416a6120, "kaim_1416a6120", fn_1416a6120 as *const ()), // sub_1416a6120
    Replacement::draft(0x1416a62c0, "kaim_1416a62c0", fn_1416a62c0 as *const ()), // sub_1416a62c0
    Replacement::draft(0x1416a6350, "kaim_1416a6350", fn_1416a6350 as *const ()), // sub_1416a6350
    Replacement::draft(0x1416a63c0, "kaim_1416a63c0", fn_1416a63c0 as *const ()), // sub_1416a63c0
    Replacement::draft(0x1416a6700, "kaim_1416a6700", fn_1416a6700 as *const ()), // sub_1416a6700
    Replacement::draft(0x1416a6760, "kaim_1416a6760", fn_1416a6760 as *const ()), // sub_1416a6760
    Replacement::draft(0x1416a67b0, "kaim_1416a67b0", fn_1416a67b0 as *const ()), // sub_1416a67b0
    Replacement::draft(0x1416a6870, "kaim_1416a6870", fn_1416a6870 as *const ()), // sub_1416a6870
    Replacement::draft(0x1416a68c0, "kaim_1416a68c0", tpl_1674::<0x1426aa730> as *const ()), // sub_1416a68c0
    Replacement::draft(0x1416a68d0, "kaim_1416a68d0", fn_1416a68d0 as *const ()), // sub_1416a68d0
    Replacement::draft(0x1416a6930, "kaim_1416a6930", tpl_31614::<0xd0, 0x143932190, 0x60, 0xb8, 0xa0, 0x88, 0x10> as *const ()), // sub_1416a6930
    Replacement::draft(0x1416a69c0, "kaim_1416a69c0", tpl_31525::<0x1416a5d50, 0x18, 0x143932190, 0x60> as *const ()), // sub_1416a69c0
    Replacement::draft(0x1416a6a10, "kaim_1416a6a10", fn_1416a6a10 as *const ()), // sub_1416a6a10
    Replacement::draft(0x1416a6bc0, "kaim_1416a6bc0", fn_1416a6bc0 as *const ()), // sub_1416a6bc0
    Replacement::draft(0x1416a6d30, "kaim_1416a6d30", tpl_893::<0x1426aa730, 0x141834d30> as *const ()), // sub_1416a6d30
    Replacement::draft(0x1416a6d60, "kaim_1416a6d60", fn_1416a6d60 as *const ()), // sub_1416a6d60
    Replacement::draft(0x1416a6e90, "kaim_1416a6e90", fn_1416a6e90 as *const ()), // sub_1416a6e90
    Replacement::draft(0x1416a6fd0, "kaim_1416a6fd0", fn_1416a6fd0 as *const ()), // sub_1416a6fd0
    Replacement::draft(0x1416a7100, "kaim_1416a7100", fn_1416a7100 as *const ()), // sub_1416a7100
    Replacement::draft(0x1416a71e0, "kaim_1416a71e0", fn_1416a71e0 as *const ()), // sub_1416a71e0
    Replacement::draft(0x1416a7360, "kaim_1416a7360", fn_1416a7360 as *const ()), // sub_1416a7360
    Replacement::draft(0x1416a7590, "kaim_1416a7590", fn_1416a7590 as *const ()), // sub_1416a7590
    Replacement::draft(0x1416a75e0, "kaim_1416a75e0", fn_1416a75e0 as *const ()), // sub_1416a75e0
    Replacement::draft(0x1416a7630, "kaim_1416a7630", fn_1416a7630 as *const ()), // sub_1416a7630
    Replacement::draft(0x1416a7720, "kaim_1416a7720", fn_1416a7720 as *const ()), // sub_1416a7720
    Replacement::draft(0x1416a7790, "kaim_1416a7790", fn_1416a7790 as *const ()), // sub_1416a7790
    Replacement::draft(0x1416a7860, "kaim_1416a7860", fn_1416a7860 as *const ()), // sub_1416a7860
    Replacement::draft(0x1416a78a0, "kaim_1416a78a0", fn_1416a78a0 as *const ()), // sub_1416a78a0
    Replacement::draft(0x1416a7970, "kaim_1416a7970", fn_1416a7970 as *const ()), // sub_1416a7970
    Replacement::draft(0x1416a7a00, "kaim_1416a7a00", fn_1416a7a00 as *const ()), // sub_1416a7a00
    Replacement::draft(0x1416a7b20, "kaim_1416a7b20", fn_1416a7b20 as *const ()), // sub_1416a7b20
    Replacement::draft(0x1416a7cb0, "kaim_1416a7cb0", fn_1416a7cb0 as *const ()), // sub_1416a7cb0
    Replacement::draft(0x1416a7cf0, "kaim_1416a7cf0", fn_1416a7cf0 as *const ()), // sub_1416a7cf0
    Replacement::draft(0x1416a7f00, "kaim_1416a7f00", fn_1416a7f00 as *const ()), // sub_1416a7f00
    Replacement::draft(0x1416a7fe0, "kaim_1416a7fe0", fn_1416a7fe0 as *const ()), // sub_1416a7fe0
    Replacement::draft(0x1416a8090, "kaim_1416a8090", fn_1416a8090 as *const ()), // sub_1416a8090
    Replacement::draft(0x1416a8220, "kaim_1416a8220", fn_1416a8220 as *const ()), // sub_1416a8220
    Replacement::draft(0x1416a82a0, "kaim_1416a82a0", fn_1416a82a0 as *const ()), // sub_1416a82a0
    Replacement::draft(0x1416a8460, "kaim_1416a8460", fn_1416a8460 as *const ()), // sub_1416a8460
    Replacement::draft(0x1416a85b0, "kaim_1416a85b0", fn_1416a85b0 as *const ()), // sub_1416a85b0
    Replacement::draft(0x1416a8ba0, "kaim_1416a8ba0", fn_1416a8ba0 as *const ()), // sub_1416a8ba0
    Replacement::draft(0x1416a8f50, "kaim_1416a8f50", fn_1416a8f50 as *const ()), // sub_1416a8f50
    Replacement::draft(0x1416a90f0, "kaim_1416a90f0", fn_1416a90f0 as *const ()), // sub_1416a90f0
    Replacement::draft(0x1416a9330, "kaim_1416a9330", fn_1416a9330 as *const ()), // sub_1416a9330
    Replacement::draft(0x1416a9740, "kaim_1416a9740", fn_1416a9740 as *const ()), // sub_1416a9740
    Replacement::draft(0x1416a9d70, "kaim_1416a9d70", fn_1416a9d70 as *const ()), // sub_1416a9d70
    Replacement::draft(0x1416a9e60, "kaim_1416a9e60", fn_1416a9e60 as *const ()), // sub_1416a9e60
    Replacement::draft(0x1416a9fa0, "kaim_1416a9fa0", fn_1416a9fa0 as *const ()), // sub_1416a9fa0
    Replacement::draft(0x1416aa050, "kaim_1416aa050", fn_1416aa050 as *const ()), // sub_1416aa050
    Replacement::draft(0x1416aa0d0, "kaim_1416aa0d0", fn_1416aa0d0 as *const ()), // sub_1416aa0d0
    Replacement::draft(0x1416aa100, "kaim_1416aa100", fn_1416aa100 as *const ()), // sub_1416aa100
    Replacement::draft(0x1416aa200, "kaim_1416aa200", fn_1416aa200 as *const ()), // sub_1416aa200
    Replacement::draft(0x1416aa800, "kaim_1416aa800", fn_1416aa800 as *const ()), // sub_1416aa800
    Replacement::draft(0x1416aad80, "kaim_1416aad80", fn_1416aad80 as *const ()), // sub_1416aad80
    Replacement::draft(0x1416aaf20, "kaim_1416aaf20", fn_1416aaf20 as *const ()), // sub_1416aaf20
    Replacement::draft(0x1416ab080, "kaim_1416ab080", fn_1416ab080 as *const ()), // sub_1416ab080
    Replacement::draft(0x1416ab3f0, "kaim_1416ab3f0", fn_1416ab3f0 as *const ()), // sub_1416ab3f0
    Replacement::draft(0x1416ab5a0, "kaim_1416ab5a0", fn_1416ab5a0 as *const ()), // sub_1416ab5a0
    Replacement::draft(0x1416ab850, "kaim_1416ab850", fn_1416ab850 as *const ()), // sub_1416ab850
    Replacement::draft(0x1416ab940, "kaim_1416ab940", fn_1416ab940 as *const ()), // sub_1416ab940
    Replacement::draft(0x1416aba70, "kaim_1416aba70", fn_1416aba70 as *const ()), // sub_1416aba70
    Replacement::draft(0x1416abb90, "kaim_1416abb90", fn_1416abb90 as *const ()), // sub_1416abb90
    Replacement::draft(0x1416abd40, "kaim_1416abd40", fn_1416abd40 as *const ()), // sub_1416abd40
    Replacement::draft(0x1416abe80, "kaim_1416abe80", fn_1416abe80 as *const ()), // sub_1416abe80
    Replacement::draft(0x1416abf10, "kaim_1416abf10", fn_1416abf10 as *const ()), // sub_1416abf10
    Replacement::draft(0x1416abf50, "kaim_1416abf50", fn_1416abf50 as *const ()), // sub_1416abf50
    Replacement::draft(0x1416abfe0, "kaim_1416abfe0", fn_1416abfe0 as *const ()), // sub_1416abfe0
    Replacement::draft(0x1416ac070, "kaim_1416ac070", fn_1416ac070 as *const ()), // sub_1416ac070
    Replacement::draft(0x1416ac100, "kaim_1416ac100", fn_1416ac100 as *const ()), // sub_1416ac100
    Replacement::draft(0x1416ac150, "kaim_1416ac150", fn_1416ac150 as *const ()), // sub_1416ac150
    Replacement::draft(0x1416ac240, "kaim_1416ac240", fn_1416ac240 as *const ()), // Kaim::DiskCollisionQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::DiskCollisionQue
    Replacement::draft(0x1416ac320, "kaim_1416ac320", fn_1416ac320 as *const ()), // sub_1416ac320
    Replacement::draft(0x1416ac340, "kaim_1416ac340", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aa818> as *const ()), // sub_1416ac340
    Replacement::draft(0x1416ac3a0, "kaim_1416ac3a0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aa850, 0x8> as *const ()), // sub_1416ac3a0
    Replacement::draft(0x1416ac3e0, "kaim_1416ac3e0", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aa818> as *const ()), // sub_1416ac3e0
    Replacement::draft(0x1416ac440, "kaim_1416ac440", fn_1416ac440 as *const ()), // sub_1416ac440
    Replacement::draft(0x1416ac650, "kaim_1416ac650", fn_1416ac650 as *const ()), // sub_1416ac650
    Replacement::draft(0x1416ac840, "kaim_1416ac840", fn_1416ac840 as *const ()), // sub_1416ac840
    Replacement::draft(0x1416ac9b0, "kaim_1416ac9b0", fn_1416ac9b0 as *const ()), // sub_1416ac9b0
    Replacement::draft(0x1416acaa0, "kaim_1416acaa0", fn_1416acaa0 as *const ()), // sub_1416acaa0
    Replacement::draft(0x1416acc80, "kaim_1416acc80", fn_1416acc80 as *const ()), // sub_1416acc80
    Replacement::draft(0x1416ad4f0, "kaim_1416ad4f0", tpl_22446::<0x1416ad7d0> as *const ()), // sub_1416ad4f0
    Replacement::draft(0x1416ad500, "kaim_1416ad500", fn_1416ad500 as *const ()), // sub_1416ad500
    Replacement::draft(0x1416ad510, "kaim_1416ad510", tpl_1::<0x1426aa7f0> as *const ()), // sub_1416ad510
    Replacement::draft(0x1416ad520, "kaim_1416ad520", fn_1416ad520 as *const ()), // Kaim::DiskCollisionQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::DiskCollisionQue
    Replacement::draft(0x1416ad700, "kaim_1416ad700", fn_1416ad700 as *const ()), // sub_1416ad700
    Replacement::draft(0x1416ad7d0, "kaim_1416ad7d0", fn_1416ad7d0 as *const ()), // sub_1416ad7d0
    Replacement::draft(0x1416ad880, "kaim_1416ad880", fn_1416ad880 as *const ()), // sub_1416ad880
    Replacement::draft(0x1416ad910, "kaim_1416ad910", fn_1416ad910 as *const ()), // sub_1416ad910
    Replacement::draft(0x1416ad990, "kaim_1416ad990", fn_1416ad990 as *const ()), // sub_1416ad990
    Replacement::draft(0x1416adac0, "kaim_1416adac0", fn_1416adac0 as *const ()), // sub_1416adac0
    Replacement::draft(0x1416adae0, "kaim_1416adae0", tpl_31685::<0x14169edb0, 0x8> as *const ()), // sub_1416adae0
    Replacement::draft(0x1416adb10, "kaim_1416adb10", tpl_31685::<0x14169edb0, 0x8> as *const ()), // sub_1416adb10
    Replacement::draft(0x1416adb40, "kaim_1416adb40", fn_1416adb40 as *const ()), // sub_1416adb40
    Replacement::draft(0x1416adc00, "kaim_1416adc00", fn_1416adc00 as *const ()), // sub_1416adc00
    Replacement::draft(0x1416adcb0, "kaim_1416adcb0", fn_1416adcb0 as *const ()), // sub_1416adcb0
    Replacement::draft(0x1416adf40, "kaim_1416adf40", fn_1416adf40 as *const ()), // sub_1416adf40
    Replacement::draft(0x1416ae010, "kaim_1416ae010", fn_1416ae010 as *const ()), // sub_1416ae010
    Replacement::draft(0x1416ae0a0, "kaim_1416ae0a0", fn_1416ae0a0 as *const ()), // sub_1416ae0a0
    Replacement::draft(0x1416ae170, "kaim_1416ae170", fn_1416ae170 as *const ()), // sub_1416ae170
    Replacement::draft(0x1416ae230, "kaim_1416ae230", fn_1416ae230 as *const ()), // sub_1416ae230
    Replacement::draft(0x1416ae2d0, "kaim_1416ae2d0", fn_1416ae2d0 as *const ()), // sub_1416ae2d0
    Replacement::draft(0x1416ae430, "kaim_1416ae430", fn_1416ae430 as *const ()), // sub_1416ae430
    Replacement::draft(0x1416ae5e0, "kaim_1416ae5e0", fn_1416ae5e0 as *const ()), // sub_1416ae5e0
    Replacement::draft(0x1416ae6e0, "kaim_1416ae6e0", fn_1416ae6e0 as *const ()), // sub_1416ae6e0
    Replacement::draft(0x1416ae7e0, "kaim_1416ae7e0", fn_1416ae7e0 as *const ()), // sub_1416ae7e0
    Replacement::draft(0x1416ae830, "kaim_1416ae830", fn_1416ae830 as *const ()), // sub_1416ae830
    Replacement::draft(0x1416aed30, "kaim_1416aed30", fn_1416aed30 as *const ()), // sub_1416aed30
    Replacement::draft(0x1416aedc0, "kaim_1416aedc0", tpl_31701::<0x1416dde40, 0x1416ddb00, 0x1416e3e20, 0x141200150, 0x1416e4010, 0x14123af00, 0x1416ddde0, 0x140000000> as *const ()), // sub_1416aedc0
    Replacement::draft(0x1416af170, "kaim_1416af170", fn_1416af170 as *const ()), // sub_1416af170
    Replacement::draft(0x1416af1b0, "kaim_1416af1b0", fn_1416af1b0 as *const ()), // sub_1416af1b0
    Replacement::draft(0x1416af270, "kaim_1416af270", tpl_22465::<0x1416af8e0, 0x10> as *const ()), // sub_1416af270
    Replacement::draft(0x1416af290, "kaim_1416af290", fn_1416af290 as *const ()), // sub_1416af290
    Replacement::draft(0x1416af2e0, "kaim_1416af2e0", fn_1416af2e0 as *const ()), // sub_1416af2e0
    Replacement::draft(0x1416af340, "kaim_1416af340", fn_1416af340 as *const ()), // sub_1416af340
    Replacement::draft(0x1416af380, "kaim_1416af380", fn_1416af380 as *const ()), // sub_1416af380
    Replacement::draft(0x1416af3e0, "kaim_1416af3e0", fn_1416af3e0 as *const ()), // sub_1416af3e0
    Replacement::draft(0x1416af440, "kaim_1416af440", fn_1416af440 as *const ()), // sub_1416af440
    Replacement::draft(0x1416af4d0, "kaim_1416af4d0", fn_1416af4d0 as *const ()), // sub_1416af4d0
    Replacement::draft(0x1416af580, "kaim_1416af580", fn_1416af580 as *const ()), // sub_1416af580
    Replacement::draft(0x1416af600, "kaim_1416af600", fn_1416af600 as *const ()), // sub_1416af600
    Replacement::draft(0x1416af680, "kaim_1416af680", fn_1416af680 as *const ()), // sub_1416af680
    Replacement::draft(0x1416af700, "kaim_1416af700", fn_1416af700 as *const ()), // sub_1416af700
    Replacement::draft(0x1416af780, "kaim_1416af780", fn_1416af780 as *const ()), // sub_1416af780
    Replacement::draft(0x1416af7c0, "kaim_1416af7c0", fn_1416af7c0 as *const ()), // sub_1416af7c0
    Replacement::draft(0x1416af850, "kaim_1416af850", fn_1416af850 as *const ()), // sub_1416af850
    Replacement::draft(0x1416af890, "kaim_1416af890", fn_1416af890 as *const ()), // sub_1416af890
    Replacement::draft(0x1416af8e0, "kaim_1416af8e0", tpl_90980::<0xbc, 0x20, 0x68, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x98, 0x7c, 0x1416af9b0> as *const ()), // sub_1416af8e0
    Replacement::draft(0x1416af9b0, "kaim_1416af9b0", tpl_230870::<0x1416b1060, 0x1416a41b0, 0x1416b12a0, 0x1434268d0, 0x1416ae0a0, 0x1416ae7e0, 0x1416ae2d0, 0x140000000, 0x1416aedc0, 0x1416ae430, 0x1416ae830, 0x1416dd200, 0x1416aed30, 0x1416a35b0> as *const ()), // sub_1416af9b0
    Replacement::draft(0x1416b02d0, "kaim_1416b02d0", fn_1416b02d0 as *const ()), // sub_1416b02d0
    Replacement::draft(0x1416b0360, "kaim_1416b0360", fn_1416b0360 as *const ()), // sub_1416b0360
    Replacement::draft(0x1416b0390, "kaim_1416b0390", fn_1416b0390 as *const ()), // sub_1416b0390
    Replacement::draft(0x1416b05a0, "kaim_1416b05a0", fn_1416b05a0 as *const ()), // sub_1416b05a0
    Replacement::draft(0x1416b0700, "kaim_1416b0700", fn_1416b0700 as *const ()), // sub_1416b0700
    Replacement::draft(0x1416b0940, "kaim_1416b0940", fn_1416b0940 as *const ()), // sub_1416b0940
    Replacement::draft(0x1416b0a90, "kaim_1416b0a90", fn_1416b0a90 as *const ()), // sub_1416b0a90
    Replacement::draft(0x1416b0c80, "kaim_1416b0c80", fn_1416b0c80 as *const ()), // sub_1416b0c80
    Replacement::draft(0x1416b0d50, "kaim_1416b0d50", fn_1416b0d50 as *const ()), // sub_1416b0d50
    Replacement::draft(0x1416b0df0, "kaim_1416b0df0", fn_1416b0df0 as *const ()), // sub_1416b0df0
    Replacement::draft(0x1416b0e60, "kaim_1416b0e60", fn_1416b0e60 as *const ()), // sub_1416b0e60
    Replacement::draft(0x1416b1060, "kaim_1416b1060", fn_1416b1060 as *const ()), // sub_1416b1060
    Replacement::draft(0x1416b1080, "kaim_1416b1080", fn_1416b1080 as *const ()), // sub_1416b1080
    Replacement::draft(0x1416b12a0, "kaim_1416b12a0", fn_1416b12a0 as *const ()), // sub_1416b12a0
    Replacement::draft(0x1416b14c0, "kaim_1416b14c0", fn_1416b14c0 as *const ()), // sub_1416b14c0
    Replacement::draft(0x1416b14e0, "kaim_1416b14e0", fn_1416b14e0 as *const ()), // sub_1416b14e0
    Replacement::draft(0x1416b1580, "kaim_1416b1580", fn_1416b1580 as *const ()), // sub_1416b1580
    Replacement::draft(0x1416b1650, "kaim_1416b1650", fn_1416b1650 as *const ()), // sub_1416b1650
    Replacement::draft(0x1416b1680, "kaim_1416b1680", fn_1416b1680 as *const ()), // sub_1416b1680
    Replacement::draft(0x1416b1740, "kaim_1416b1740", fn_1416b1740 as *const ()), // sub_1416b1740
    Replacement::draft(0x1416b1860, "kaim_1416b1860", fn_1416b1860 as *const ()), // sub_1416b1860
    Replacement::draft(0x1416b1a20, "kaim_1416b1a20", fn_1416b1a20 as *const ()), // Kaim::RayCastQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::RayCastQuery<Kaim::Loc
    Replacement::draft(0x1416b1b10, "kaim_1416b1b10", fn_1416b1b10 as *const ()), // sub_1416b1b10
    Replacement::draft(0x1416b1e80, "kaim_1416b1e80", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aa940> as *const ()), // sub_1416b1e80
    Replacement::draft(0x1416b1ee0, "kaim_1416b1ee0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aa978, 0x8> as *const ()), // sub_1416b1ee0
    Replacement::draft(0x1416b1f20, "kaim_1416b1f20", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aa940> as *const ()), // sub_1416b1f20
    Replacement::draft(0x1416b1f80, "kaim_1416b1f80", fn_1416b1f80 as *const ()), // sub_1416b1f80
    Replacement::draft(0x1416b2060, "kaim_1416b2060", fn_1416b2060 as *const ()), // sub_1416b2060
    Replacement::draft(0x1416b2220, "kaim_1416b2220", fn_1416b2220 as *const ()), // sub_1416b2220
    Replacement::draft(0x1416b25e0, "kaim_1416b25e0", tpl_22446::<0x1416b2d20> as *const ()), // sub_1416b25e0
    Replacement::draft(0x1416b25f0, "kaim_1416b25f0", fn_1416b25f0 as *const ()), // sub_1416b25f0
    Replacement::draft(0x1416b2600, "kaim_1416b2600", tpl_1::<0x1426aa918> as *const ()), // sub_1416b2600
    Replacement::draft(0x1416b2610, "kaim_1416b2610", fn_1416b2610 as *const ()), // Kaim::RayCastQuery<Kaim::LocalCostAwareTraverseLogic<navigation::NavigationTraverseLogic>>::Kaim::RayCastQuery<Kaim::Loc
    Replacement::draft(0x1416b2680, "kaim_1416b2680", fn_1416b2680 as *const ()), // sub_1416b2680
    Replacement::draft(0x1416b2820, "kaim_1416b2820", fn_1416b2820 as *const ()), // sub_1416b2820
    Replacement::draft(0x1416b2c30, "kaim_1416b2c30", fn_1416b2c30 as *const ()), // sub_1416b2c30
    Replacement::draft(0x1416b2d20, "kaim_1416b2d20", tpl_31744::<0x1416b2dd0> as *const ()), // sub_1416b2d20
    Replacement::draft(0x1416b2dd0, "kaim_1416b2dd0", fn_1416b2dd0 as *const ()), // sub_1416b2dd0
    Replacement::draft(0x1416b2f70, "kaim_1416b2f70", fn_1416b2f70 as *const ()), // sub_1416b2f70
    Replacement::draft(0x1416b3080, "kaim_1416b3080", fn_1416b3080 as *const ()), // sub_1416b3080
    Replacement::draft(0x1416b3160, "kaim_1416b3160", fn_1416b3160 as *const ()), // sub_1416b3160
    Replacement::draft(0x1416b40f0, "kaim_1416b40f0", fn_1416b40f0 as *const ()), // sub_1416b40f0
    Replacement::draft(0x1416b4520, "kaim_1416b4520", fn_1416b4520 as *const ()), // sub_1416b4520
    Replacement::draft(0x1416b47d0, "kaim_1416b47d0", fn_1416b47d0 as *const ()), // sub_1416b47d0
    Replacement::draft(0x1416b4b90, "kaim_1416b4b90", fn_1416b4b90 as *const ()), // sub_1416b4b90
    Replacement::draft(0x1416b4c40, "kaim_1416b4c40", fn_1416b4c40 as *const ()), // sub_1416b4c40
    Replacement::draft(0x1416b4e20, "kaim_1416b4e20", tpl_30667::<0x14> as *const ()), // sub_1416b4e20
    Replacement::draft(0x1416b4e30, "kaim_1416b4e30", fn_1416b4e30 as *const ()), // sub_1416b4e30
    Replacement::draft(0x1416b5540, "kaim_1416b5540", fn_1416b5540 as *const ()), // sub_1416b5540
    Replacement::draft(0x1416b59d0, "kaim_1416b59d0", fn_1416b59d0 as *const ()), // sub_1416b59d0
    Replacement::draft(0x1416b5c80, "kaim_1416b5c80", fn_1416b5c80 as *const ()), // sub_1416b5c80
    Replacement::draft(0x1416b5e60, "kaim_1416b5e60", fn_1416b5e60 as *const ()), // sub_1416b5e60
    Replacement::draft(0x1416b64e0, "kaim_1416b64e0", fn_1416b64e0 as *const ()), // sub_1416b64e0
    Replacement::draft(0x1416b6570, "kaim_1416b6570", fn_1416b6570 as *const ()), // sub_1416b6570
    Replacement::draft(0x1416b6710, "kaim_1416b6710", tpl_22426::<0x1426aaa40, 0x1411fbbe0> as *const ()), // sub_1416b6710
    Replacement::draft(0x1416b6720, "kaim_1416b6720", tpl_22484::<0x1426aaa40, 0x1411fbbe0, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416b6720
    Replacement::draft(0x1416b6790, "kaim_1416b6790", fn_1416b6790 as *const ()), // sub_1416b6790
    Replacement::draft(0x1416b6970, "kaim_1416b6970", fn_1416b6970 as *const ()), // sub_1416b6970
    Replacement::draft(0x1416b6b10, "kaim_1416b6b10", tpl_31756::<0xd0, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x6c, 0x1416b1580, 0x1416b9a10> as *const ()), // sub_1416b6b10
    Replacement::draft(0x1416b6bc0, "kaim_1416b6bc0", fn_1416b6bc0 as *const ()), // sub_1416b6bc0
    Replacement::draft(0x1416b6c40, "kaim_1416b6c40", fn_1416b6c40 as *const ()), // sub_1416b6c40
    Replacement::draft(0x1416b6cd0, "kaim_1416b6cd0", fn_1416b6cd0 as *const ()), // sub_1416b6cd0
    Replacement::draft(0x1416b6de0, "kaim_1416b6de0", fn_1416b6de0 as *const ()), // sub_1416b6de0
    Replacement::draft(0x1416b6f00, "kaim_1416b6f00", fn_1416b6f00 as *const ()), // sub_1416b6f00
    Replacement::draft(0x1416b75b0, "kaim_1416b75b0", fn_1416b75b0 as *const ()), // sub_1416b75b0
    Replacement::draft(0x1416b7860, "kaim_1416b7860", fn_1416b7860 as *const ()), // sub_1416b7860
    Replacement::draft(0x1416b7e40, "kaim_1416b7e40", fn_1416b7e40 as *const ()), // sub_1416b7e40
    Replacement::draft(0x1416b8150, "kaim_1416b8150", tpl_27317::<0xf0, 0x1416bcaf0> as *const ()), // sub_1416b8150
    Replacement::draft(0x1416b8160, "kaim_1416b8160", fn_1416b8160 as *const ()), // sub_1416b8160
    Replacement::draft(0x1416b8200, "kaim_1416b8200", fn_1416b8200 as *const ()), // sub_1416b8200
    Replacement::draft(0x1416b8380, "kaim_1416b8380", fn_1416b8380 as *const ()), // sub_1416b8380
    Replacement::draft(0x1416b8710, "kaim_1416b8710", fn_1416b8710 as *const ()), // sub_1416b8710
    Replacement::draft(0x1416b8a60, "kaim_1416b8a60", fn_1416b8a60 as *const ()), // sub_1416b8a60
    Replacement::draft(0x1416b8fe0, "kaim_1416b8fe0", fn_1416b8fe0 as *const ()), // sub_1416b8fe0
    Replacement::draft(0x1416b9500, "kaim_1416b9500", fn_1416b9500 as *const ()), // sub_1416b9500
    Replacement::draft(0x1416b9830, "kaim_1416b9830", fn_1416b9830 as *const ()), // sub_1416b9830
    Replacement::draft(0x1416b9a10, "kaim_1416b9a10", tpl_31766::<0x1416b1060, 0x1416a41b0, 0x1416b12a0, 0x143428720, 0x1416b1580, 0x1434268d0, 0x1416ae0a0, 0x1416ae7e0, 0x1416ae230, 0x1411fb750, 0x1416ae2d0, 0x143427080, 0x1416ae430, 0x1416ae830, 0x140000000, 0x1416ba2e0, 0x1416b2820, 0x1416a35b0> as *const ()), // sub_1416b9a10
    Replacement::draft(0x1416ba260, "kaim_1416ba260", fn_1416ba260 as *const ()), // sub_1416ba260
    Replacement::draft(0x1416ba2e0, "kaim_1416ba2e0", fn_1416ba2e0 as *const ()), // sub_1416ba2e0
    Replacement::draft(0x1416ba780, "kaim_1416ba780", fn_1416ba780 as *const ()), // sub_1416ba780
    Replacement::draft(0x1416ba830, "kaim_1416ba830", fn_1416ba830 as *const ()), // sub_1416ba830
    Replacement::draft(0x1416ba8e0, "kaim_1416ba8e0", fn_1416ba8e0 as *const ()), // sub_1416ba8e0
    Replacement::draft(0x1416baa10, "kaim_1416baa10", fn_1416baa10 as *const ()), // sub_1416baa10
    Replacement::draft(0x1416baf60, "kaim_1416baf60", fn_1416baf60 as *const ()), // sub_1416baf60
    Replacement::draft(0x1416baf80, "kaim_1416baf80", fn_1416baf80 as *const ()), // sub_1416baf80
    Replacement::draft(0x1416bb120, "kaim_1416bb120", fn_1416bb120 as *const ()), // sub_1416bb120
    Replacement::draft(0x1416bb160, "kaim_1416bb160", fn_1416bb160 as *const ()), // sub_1416bb160
    Replacement::draft(0x1416bb560, "kaim_1416bb560", fn_1416bb560 as *const ()), // sub_1416bb560
    Replacement::draft(0x1416bb5e0, "kaim_1416bb5e0", tpl_31775::<0x8, 0xffffffffffffffff, 0x14169edd0, 0x143932190, 0x60> as *const ()), // sub_1416bb5e0
    Replacement::draft(0x1416bb650, "kaim_1416bb650", tpl_31775::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x143932190, 0x60> as *const ()), // sub_1416bb650
    Replacement::draft(0x1416bb6c0, "kaim_1416bb6c0", tpl_365::<0x1416bb5e0> as *const ()), // sub_1416bb6c0
    Replacement::draft(0x1416bb6d0, "kaim_1416bb6d0", tpl_365::<0x1416bb650> as *const ()), // sub_1416bb6d0
    Replacement::draft(0x1416bb6e0, "kaim_1416bb6e0", fn_1416bb6e0 as *const ()), // sub_1416bb6e0
    Replacement::draft(0x1416bb750, "kaim_1416bb750", fn_1416bb750 as *const ()), // sub_1416bb750
    Replacement::draft(0x1416bbae0, "kaim_1416bbae0", tpl_365::<0x141724ee0> as *const ()), // sub_1416bbae0
    Replacement::draft(0x1416bbaf0, "kaim_1416bbaf0", fn_1416bbaf0 as *const ()), // sub_1416bbaf0
    Replacement::draft(0x1416bbb50, "kaim_1416bbb50", fn_1416bbb50 as *const ()), // sub_1416bbb50
    Replacement::draft(0x1416bbbb0, "unknown_1416bbbb0", tpl_31780::<0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416bbbb0
    Replacement::draft(0x1416bbc10, "unknown_1416bbc10", fn_1416bbc10 as *const ()), // sub_1416bbc10
    Replacement::draft(0x1416bbcb0, "unknown_1416bbcb0", fn_1416bbcb0 as *const ()), // sub_1416bbcb0
    Replacement::draft(0x1416bbd90, "unknown_1416bbd90", fn_1416bbd90 as *const ()), // sub_1416bbd90
    Replacement::draft(0x1416bbfd0, "unknown_1416bbfd0", fn_1416bbfd0 as *const ()), // sub_1416bbfd0
    Replacement::draft(0x1416bc3e0, "unknown_1416bc3e0", fn_1416bc3e0 as *const ()), // sub_1416bc3e0
    Replacement::draft(0x1416bc4c0, "unknown_1416bc4c0", fn_1416bc4c0 as *const ()), // sub_1416bc4c0
    Replacement::draft(0x1416bc540, "unknown_1416bc540", fn_1416bc540 as *const ()), // sub_1416bc540
    Replacement::draft(0x1416bc650, "unknown_1416bc650", fn_1416bc650 as *const ()), // sub_1416bc650
    Replacement::draft(0x1416bcab0, "unknown_1416bcab0", fn_1416bcab0 as *const ()), // sub_1416bcab0
    Replacement::draft(0x1416bcaf0, "unknown_1416bcaf0", tpl_957::<0x10, 0x180> as *const ()), // sub_1416bcaf0
    Replacement::draft(0x1416bcb00, "unknown_1416bcb00", tpl_1::<0x1426aaac8> as *const ()), // sub_1416bcb00
    Replacement::draft(0x1416bcb10, "unknown_1416bcb10", fn_1416bcb10 as *const ()), // sub_1416bcb10
    Replacement::draft(0x1416bcbb0, "unknown_1416bcbb0", fn_1416bcbb0 as *const ()), // sub_1416bcbb0
    Replacement::draft(0x1416bcc80, "unknown_1416bcc80", fn_1416bcc80 as *const ()), // sub_1416bcc80
    Replacement::draft(0x1416bce80, "unknown_1416bce80", fn_1416bce80 as *const ()), // sub_1416bce80
    Replacement::draft(0x1416bcf30, "unknown_1416bcf30", tpl_22437::<0x10, 0x8, 0x1416bce80> as *const ()), // sub_1416bcf30
    Replacement::draft(0x1416bcfb0, "unknown_1416bcfb0", fn_1416bcfb0 as *const ()), // sub_1416bcfb0
    Replacement::draft(0x1416bd020, "unknown_1416bd020", fn_1416bd020 as *const ()), // sub_1416bd020
    Replacement::draft(0x1416bd2c0, "unknown_1416bd2c0", fn_1416bd2c0 as *const ()), // sub_1416bd2c0
    Replacement::draft(0x1416bd550, "unknown_1416bd550", fn_1416bd550 as *const ()), // sub_1416bd550
    Replacement::draft(0x1416bd810, "unknown_1416bd810", fn_1416bd810 as *const ()), // sub_1416bd810
    Replacement::draft(0x1416bd860, "unknown_1416bd860", fn_1416bd860 as *const ()), // sub_1416bd860
    Replacement::draft(0x1416bd8a0, "unknown_1416bd8a0", fn_1416bd8a0 as *const ()), // sub_1416bd8a0
    Replacement::draft(0x1416bd8c0, "unknown_1416bd8c0", fn_1416bd8c0 as *const ()), // sub_1416bd8c0
    Replacement::draft(0x1416bd9b0, "unknown_1416bd9b0", fn_1416bd9b0 as *const ()), // sub_1416bd9b0
    Replacement::draft(0x1416bd9f0, "unknown_1416bd9f0", tpl_957::<0x8, 0x98> as *const ()), // sub_1416bd9f0
    Replacement::draft(0x1416bda00, "unknown_1416bda00", tpl_27317::<0x10, 0x1416c3510> as *const ()), // sub_1416bda00
    Replacement::draft(0x1416bda10, "unknown_1416bda10", tpl_660::<0x10, 0x4c> as *const ()), // sub_1416bda10
    Replacement::draft(0x1416bda20, "unknown_1416bda20", fn_1416bda20 as *const ()), // sub_1416bda20
    Replacement::draft(0x1416bda90, "unknown_1416bda90", fn_1416bda90 as *const ()), // sub_1416bda90
    Replacement::draft(0x1416bdab0, "unknown_1416bdab0", fn_1416bdab0 as *const ()), // sub_1416bdab0
    Replacement::draft(0x1416bdbe0, "unknown_1416bdbe0", fn_1416bdbe0 as *const ()), // sub_1416bdbe0
    Replacement::draft(0x1416bdc60, "unknown_1416bdc60", fn_1416bdc60 as *const ()), // sub_1416bdc60
    Replacement::draft(0x1416bdcc0, "unknown_1416bdcc0", fn_1416bdcc0 as *const ()), // sub_1416bdcc0
    Replacement::draft(0x1416bde00, "unknown_1416bde00", fn_1416bde00 as *const ()), // sub_1416bde00
    Replacement::draft(0x1416bde90, "unknown_1416bde90", fn_1416bde90 as *const ()), // sub_1416bde90
    Replacement::draft(0x1416be040, "unknown_1416be040", fn_1416be040 as *const ()), // sub_1416be040
    Replacement::draft(0x1416be0e0, "unknown_1416be0e0", fn_1416be0e0 as *const ()), // sub_1416be0e0
    Replacement::draft(0x1416be2c0, "unknown_1416be2c0", fn_1416be2c0 as *const ()), // sub_1416be2c0
    Replacement::draft(0x1416be7b0, "unknown_1416be7b0", fn_1416be7b0 as *const ()), // sub_1416be7b0
    Replacement::draft(0x1416be810, "unknown_1416be810", fn_1416be810 as *const ()), // sub_1416be810
    Replacement::draft(0x1416be860, "unknown_1416be860", fn_1416be860 as *const ()), // sub_1416be860
    Replacement::draft(0x1416be8b0, "unknown_1416be8b0", fn_1416be8b0 as *const ()), // sub_1416be8b0
    Replacement::draft(0x1416becc0, "unknown_1416becc0", fn_1416becc0 as *const ()), // sub_1416becc0
    Replacement::draft(0x1416bed20, "unknown_1416bed20", fn_1416bed20 as *const ()), // sub_1416bed20
    Replacement::draft(0x1416beea0, "unknown_1416beea0", fn_1416beea0 as *const ()), // sub_1416beea0
    Replacement::draft(0x1416bf1d0, "unknown_1416bf1d0", fn_1416bf1d0 as *const ()), // sub_1416bf1d0
    Replacement::draft(0x1416bfe60, "unknown_1416bfe60", fn_1416bfe60 as *const ()), // sub_1416bfe60
    Replacement::draft(0x1416c0990, "unknown_1416c0990", fn_1416c0990 as *const ()), // sub_1416c0990
    Replacement::draft(0x1416c0a50, "unknown_1416c0a50", fn_1416c0a50 as *const ()), // sub_1416c0a50
    Replacement::draft(0x1416c0c80, "unknown_1416c0c80", fn_1416c0c80 as *const ()), // sub_1416c0c80
    Replacement::draft(0x1416c0f10, "unknown_1416c0f10", fn_1416c0f10 as *const ()), // sub_1416c0f10
    Replacement::draft(0x1416c0f90, "unknown_1416c0f90", fn_1416c0f90 as *const ()), // sub_1416c0f90
    Replacement::draft(0x1416c1690, "unknown_1416c1690", fn_1416c1690 as *const ()), // sub_1416c1690
    Replacement::draft(0x1416c18c0, "unknown_1416c18c0", fn_1416c18c0 as *const ()), // sub_1416c18c0
    Replacement::draft(0x1416c1a20, "unknown_1416c1a20", fn_1416c1a20 as *const ()), // sub_1416c1a20
    Replacement::draft(0x1416c1af0, "unknown_1416c1af0", fn_1416c1af0 as *const ()), // sub_1416c1af0
    Replacement::draft(0x1416c1b40, "unknown_1416c1b40", fn_1416c1b40 as *const ()), // sub_1416c1b40
    Replacement::draft(0x1416c1cc0, "unknown_1416c1cc0", fn_1416c1cc0 as *const ()), // sub_1416c1cc0
    Replacement::draft(0x1416c2040, "unknown_1416c2040", fn_1416c2040 as *const ()), // sub_1416c2040
    Replacement::draft(0x1416c22b0, "unknown_1416c22b0", fn_1416c22b0 as *const ()), // sub_1416c22b0
    Replacement::draft(0x1416c2430, "unknown_1416c2430", fn_1416c2430 as *const ()), // sub_1416c2430
    Replacement::draft(0x1416c27f0, "unknown_1416c27f0", fn_1416c27f0 as *const ()), // sub_1416c27f0
    Replacement::draft(0x1416c2a60, "unknown_1416c2a60", fn_1416c2a60 as *const ()), // sub_1416c2a60
    Replacement::draft(0x1416c2af0, "unknown_1416c2af0", fn_1416c2af0 as *const ()), // sub_1416c2af0
    Replacement::draft(0x1416c2cd0, "unknown_1416c2cd0", fn_1416c2cd0 as *const ()), // sub_1416c2cd0
    Replacement::draft(0x1416c2f50, "unknown_1416c2f50", fn_1416c2f50 as *const ()), // sub_1416c2f50
    Replacement::draft(0x1416c2f90, "unknown_1416c2f90", fn_1416c2f90 as *const ()), // sub_1416c2f90
    Replacement::draft(0x1416c2fd0, "unknown_1416c2fd0", fn_1416c2fd0 as *const ()), // sub_1416c2fd0
    Replacement::draft(0x1416c3060, "unknown_1416c3060", fn_1416c3060 as *const ()), // sub_1416c3060
    Replacement::draft(0x1416c3110, "physx_1416c3110", fn_1416c3110 as *const ()), // sub_1416c3110
    Replacement::draft(0x1416c31e0, "unknown_1416c31e0", fn_1416c31e0 as *const ()), // sub_1416c31e0
    Replacement::draft(0x1416c3210, "unknown_1416c3210", fn_1416c3210 as *const ()), // sub_1416c3210
    Replacement::draft(0x1416c32c0, "unknown_1416c32c0", fn_1416c32c0 as *const ()), // sub_1416c32c0
    Replacement::draft(0x1416c3370, "kaim_1416c3370", fn_1416c3370 as *const ()), // sub_1416c3370
    Replacement::draft(0x1416c34d0, "kaim_1416c34d0", fn_1416c34d0 as *const ()), // sub_1416c34d0
    Replacement::draft(0x1416c3510, "kaim_1416c3510", fn_1416c3510 as *const ()), // sub_1416c3510
    Replacement::draft(0x1416c35a0, "kaim_1416c35a0", fn_1416c35a0 as *const ()), // sub_1416c35a0
    Replacement::draft(0x1416c35e0, "kaim_1416c35e0", fn_1416c35e0 as *const ()), // sub_1416c35e0
    Replacement::draft(0x1416c36a0, "kaim_1416c36a0", fn_1416c36a0 as *const ()), // sub_1416c36a0
    Replacement::draft(0x1416c3980, "kaim_1416c3980", fn_1416c3980 as *const ()), // sub_1416c3980
    Replacement::draft(0x1416c3c60, "kaim_1416c3c60", fn_1416c3c60 as *const ()), // sub_1416c3c60
    Replacement::draft(0x1416c3cd0, "kaim_1416c3cd0", tpl_31841::<0x48, 0x143932190, 0x60, 0x30> as *const ()), // sub_1416c3cd0
    Replacement::draft(0x1416c3d10, "kaim_1416c3d10", fn_1416c3d10 as *const ()), // sub_1416c3d10
    Replacement::draft(0x1416c3db0, "kaim_1416c3db0", fn_1416c3db0 as *const ()), // sub_1416c3db0
    Replacement::draft(0x1416c3e50, "kaim_1416c3e50", fn_1416c3e50 as *const ()), // sub_1416c3e50
    Replacement::draft(0x1416c3ef0, "kaim_1416c3ef0", fn_1416c3ef0 as *const ()), // sub_1416c3ef0
    Replacement::draft(0x1416c3ff0, "kaim_1416c3ff0", fn_1416c3ff0 as *const ()), // sub_1416c3ff0
    Replacement::draft(0x1416c4330, "kaim_1416c4330", fn_1416c4330 as *const ()), // sub_1416c4330
    Replacement::draft(0x1416c4440, "kaim_1416c4440", fn_1416c4440 as *const ()), // sub_1416c4440
    Replacement::draft(0x1416c4550, "kaim_1416c4550", fn_1416c4550 as *const ()), // sub_1416c4550
    Replacement::draft(0x1416c49a0, "kaim_1416c49a0", fn_1416c49a0 as *const ()), // sub_1416c49a0
    Replacement::draft(0x1416c4df0, "kaim_1416c4df0", fn_1416c4df0 as *const ()), // sub_1416c4df0
    Replacement::draft(0x1416c68d0, "kaim_1416c68d0", fn_1416c68d0 as *const ()), // sub_1416c68d0
    Replacement::draft(0x1416c6960, "kaim_1416c6960", fn_1416c6960 as *const ()), // sub_1416c6960
    Replacement::draft(0x1416c7120, "kaim_1416c7120", fn_1416c7120 as *const ()), // sub_1416c7120
    Replacement::draft(0x1416c72b0, "kaim_1416c72b0", fn_1416c72b0 as *const ()), // sub_1416c72b0
    Replacement::draft(0x1416c81a0, "kaim_1416c81a0", fn_1416c81a0 as *const ()), // sub_1416c81a0
    Replacement::draft(0x1416c8260, "kaim_1416c8260", fn_1416c8260 as *const ()), // sub_1416c8260
    Replacement::draft(0x1416c8310, "kaim_1416c8310", fn_1416c8310 as *const ()), // sub_1416c8310
    Replacement::draft(0x1416c8400, "kaim_1416c8400", fn_1416c8400 as *const ()), // sub_1416c8400
    Replacement::draft(0x1416c85f0, "kaim_1416c85f0", fn_1416c85f0 as *const ()), // sub_1416c85f0
    Replacement::draft(0x1416c8730, "kaim_1416c8730", fn_1416c8730 as *const ()), // sub_1416c8730
    Replacement::draft(0x1416c8880, "kaim_1416c8880", fn_1416c8880 as *const ()), // sub_1416c8880
    Replacement::draft(0x1416c89a0, "kaim_1416c89a0", fn_1416c89a0 as *const ()), // sub_1416c89a0
    Replacement::draft(0x1416c8b50, "kaim_1416c8b50", fn_1416c8b50 as *const ()), // sub_1416c8b50
    Replacement::draft(0x1416c8cf0, "kaim_1416c8cf0", fn_1416c8cf0 as *const ()), // sub_1416c8cf0
    Replacement::draft(0x1416c8f90, "kaim_1416c8f90", fn_1416c8f90 as *const ()), // sub_1416c8f90
    Replacement::draft(0x1416c8fe0, "kaim_1416c8fe0", fn_1416c8fe0 as *const ()), // sub_1416c8fe0
    Replacement::draft(0x1416c9030, "kaim_1416c9030", fn_1416c9030 as *const ()), // sub_1416c9030
    Replacement::draft(0x1416c90f0, "kaim_1416c90f0", fn_1416c90f0 as *const ()), // sub_1416c90f0
    Replacement::draft(0x1416c91b0, "kaim_1416c91b0", fn_1416c91b0 as *const ()), // sub_1416c91b0
    Replacement::draft(0x1416c9270, "kaim_1416c9270", fn_1416c9270 as *const ()), // sub_1416c9270
    Replacement::draft(0x1416c9300, "kaim_1416c9300", fn_1416c9300 as *const ()), // sub_1416c9300
    Replacement::draft(0x1416c95a0, "kaim_1416c95a0", fn_1416c95a0 as *const ()), // sub_1416c95a0
    Replacement::draft(0x1416c95f0, "kaim_1416c95f0", fn_1416c95f0 as *const ()), // sub_1416c95f0
    Replacement::draft(0x1416c96b0, "kaim_1416c96b0", fn_1416c96b0 as *const ()), // sub_1416c96b0
    Replacement::draft(0x1416c9760, "kaim_1416c9760", fn_1416c9760 as *const ()), // sub_1416c9760
    Replacement::draft(0x1416c9810, "kaim_1416c9810", fn_1416c9810 as *const ()), // sub_1416c9810
    Replacement::draft(0x1416c9bc0, "kaim_1416c9bc0", fn_1416c9bc0 as *const ()), // sub_1416c9bc0
    Replacement::draft(0x1416ca1b0, "kaim_1416ca1b0", fn_1416ca1b0 as *const ()), // sub_1416ca1b0
    Replacement::draft(0x1416ca7f0, "kaim_1416ca7f0", fn_1416ca7f0 as *const ()), // sub_1416ca7f0
    Replacement::draft(0x1416ca900, "kaim_1416ca900", fn_1416ca900 as *const ()), // sub_1416ca900
    Replacement::draft(0x1416ca9f0, "kaim_1416ca9f0", fn_1416ca9f0 as *const ()), // sub_1416ca9f0
    Replacement::draft(0x1416caac0, "kaim_1416caac0", fn_1416caac0 as *const ()), // sub_1416caac0
    Replacement::draft(0x1416caaf0, "kaim_1416caaf0", fn_1416caaf0 as *const ()), // sub_1416caaf0
    Replacement::draft(0x1416cab10, "kaim_1416cab10", tpl_22426::<0x1426aab70, 0x141200480> as *const ()), // sub_1416cab10
    Replacement::draft(0x1416cab20, "kaim_1416cab20", tpl_22570::<0x1426aab70, 0x141200480, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416cab20
    Replacement::draft(0x1416cab90, "kaim_1416cab90", fn_1416cab90 as *const ()), // sub_1416cab90
    Replacement::draft(0x1416cacb0, "kaim_1416cacb0", fn_1416cacb0 as *const ()), // sub_1416cacb0
    Replacement::draft(0x1416cad20, "kaim_1416cad20", fn_1416cad20 as *const ()), // sub_1416cad20
    Replacement::draft(0x1416cad90, "kaim_1416cad90", fn_1416cad90 as *const ()), // sub_1416cad90
    Replacement::draft(0x1416cadf0, "kaim_1416cadf0", fn_1416cadf0 as *const ()), // sub_1416cadf0
    Replacement::draft(0x1416caee0, "kaim_1416caee0", fn_1416caee0 as *const ()), // sub_1416caee0
    Replacement::draft(0x1416cb860, "kaim_1416cb860", fn_1416cb860 as *const ()), // sub_1416cb860
    Replacement::draft(0x1416cb970, "kaim_1416cb970", fn_1416cb970 as *const ()), // sub_1416cb970
    Replacement::draft(0x1416cba50, "kaim_1416cba50", fn_1416cba50 as *const ()), // sub_1416cba50
    Replacement::draft(0x1416cbb20, "kaim_1416cbb20", fn_1416cbb20 as *const ()), // sub_1416cbb20
    Replacement::draft(0x1416cbc20, "kaim_1416cbc20", fn_1416cbc20 as *const ()), // sub_1416cbc20
    Replacement::draft(0x1416cc570, "kaim_1416cc570", fn_1416cc570 as *const ()), // sub_1416cc570
    Replacement::draft(0x1416ccbf0, "kaim_1416ccbf0", fn_1416ccbf0 as *const ()), // sub_1416ccbf0
    Replacement::draft(0x1416ccd00, "kaim_1416ccd00", fn_1416ccd00 as *const ()), // sub_1416ccd00
    Replacement::draft(0x1416ccf70, "kaim_1416ccf70", fn_1416ccf70 as *const ()), // sub_1416ccf70
    Replacement::draft(0x1416cd400, "kaim_1416cd400", fn_1416cd400 as *const ()), // sub_1416cd400
    Replacement::draft(0x1416cd740, "kaim_1416cd740", fn_1416cd740 as *const ()), // sub_1416cd740
    Replacement::draft(0x1416cd880, "kaim_1416cd880", fn_1416cd880 as *const ()), // sub_1416cd880
    Replacement::draft(0x1416cd8f0, "kaim_1416cd8f0", fn_1416cd8f0 as *const ()), // sub_1416cd8f0
    Replacement::draft(0x1416cd940, "kaim_1416cd940", fn_1416cd940 as *const ()), // sub_1416cd940
    Replacement::draft(0x1416cd970, "kaim_1416cd970", fn_1416cd970 as *const ()), // sub_1416cd970
    Replacement::draft(0x1416cd9a0, "kaim_1416cd9a0", fn_1416cd9a0 as *const ()), // sub_1416cd9a0
    Replacement::draft(0x1416cd9d0, "kaim_1416cd9d0", fn_1416cd9d0 as *const ()), // sub_1416cd9d0
    Replacement::draft(0x1416cdd30, "kaim_1416cdd30", fn_1416cdd30 as *const ()), // sub_1416cdd30
    Replacement::draft(0x1416cde20, "kaim_1416cde20", fn_1416cde20 as *const ()), // sub_1416cde20
    Replacement::draft(0x1416cdef0, "kaim_1416cdef0", fn_1416cdef0 as *const ()), // sub_1416cdef0
    Replacement::draft(0x1416cdf40, "kaim_1416cdf40", fn_1416cdf40 as *const ()), // sub_1416cdf40
    Replacement::draft(0x1416ce010, "kaim_1416ce010", fn_1416ce010 as *const ()), // sub_1416ce010
    Replacement::draft(0x1416ce0c0, "kaim_1416ce0c0", fn_1416ce0c0 as *const ()), // sub_1416ce0c0
    Replacement::draft(0x1416ce3b0, "kaim_1416ce3b0", fn_1416ce3b0 as *const ()), // sub_1416ce3b0
    Replacement::draft(0x1416ce4a0, "kaim_1416ce4a0", fn_1416ce4a0 as *const ()), // sub_1416ce4a0
    Replacement::draft(0x1416ce560, "kaim_1416ce560", fn_1416ce560 as *const ()), // sub_1416ce560
    Replacement::draft(0x1416ce670, "kaim_1416ce670", fn_1416ce670 as *const ()), // sub_1416ce670
    Replacement::draft(0x1416ce720, "kaim_1416ce720", fn_1416ce720 as *const ()), // sub_1416ce720
    Replacement::draft(0x1416ce830, "kaim_1416ce830", fn_1416ce830 as *const ()), // sub_1416ce830
    Replacement::draft(0x1416cea60, "kaim_1416cea60", fn_1416cea60 as *const ()), // sub_1416cea60
    Replacement::draft(0x1416cebe0, "kaim_1416cebe0", fn_1416cebe0 as *const ()), // sub_1416cebe0
    Replacement::draft(0x1416cee30, "kaim_1416cee30", fn_1416cee30 as *const ()), // sub_1416cee30
    Replacement::draft(0x1416cf070, "kaim_1416cf070", fn_1416cf070 as *const ()), // sub_1416cf070
    Replacement::draft(0x1416cf2e0, "kaim_1416cf2e0", fn_1416cf2e0 as *const ()), // sub_1416cf2e0
    Replacement::draft(0x1416cf3d0, "kaim_1416cf3d0", fn_1416cf3d0 as *const ()), // sub_1416cf3d0
    Replacement::draft(0x1416cf430, "kaim_1416cf430", fn_1416cf430 as *const ()), // sub_1416cf430
    Replacement::draft(0x1416cf480, "kaim_1416cf480", fn_1416cf480 as *const ()), // sub_1416cf480
    Replacement::draft(0x1416cf500, "kaim_1416cf500", fn_1416cf500 as *const ()), // sub_1416cf500
    Replacement::draft(0x1416cf920, "kaim_1416cf920", fn_1416cf920 as *const ()), // sub_1416cf920
    Replacement::draft(0x1416cfd30, "kaim_1416cfd30", fn_1416cfd30 as *const ()), // sub_1416cfd30
    Replacement::draft(0x1416cff30, "kaim_1416cff30", fn_1416cff30 as *const ()), // sub_1416cff30
    Replacement::draft(0x1416d00f0, "kaim_1416d00f0", fn_1416d00f0 as *const ()), // sub_1416d00f0
    Replacement::draft(0x1416d0180, "kaim_1416d0180", fn_1416d0180 as *const ()), // sub_1416d0180
    Replacement::draft(0x1416d0220, "kaim_1416d0220", fn_1416d0220 as *const ()), // sub_1416d0220
    Replacement::draft(0x1416d02c0, "kaim_1416d02c0", fn_1416d02c0 as *const ()), // sub_1416d02c0
    Replacement::draft(0x1416d03e0, "kaim_1416d03e0", fn_1416d03e0 as *const ()), // sub_1416d03e0
    Replacement::draft(0x1416d04b0, "kaim_1416d04b0", fn_1416d04b0 as *const ()), // sub_1416d04b0
    Replacement::draft(0x1416d0590, "kaim_1416d0590", fn_1416d0590 as *const ()), // sub_1416d0590
    Replacement::draft(0x1416d0a10, "kaim_1416d0a10", fn_1416d0a10 as *const ()), // sub_1416d0a10
    Replacement::draft(0x1416d0b00, "kaim_1416d0b00", fn_1416d0b00 as *const ()), // sub_1416d0b00
    Replacement::draft(0x1416d0e60, "kaim_1416d0e60", fn_1416d0e60 as *const ()), // sub_1416d0e60
    Replacement::draft(0x1416d0ef0, "kaim_1416d0ef0", fn_1416d0ef0 as *const ()), // sub_1416d0ef0
    Replacement::draft(0x1416d0fe0, "kaim_1416d0fe0", fn_1416d0fe0 as *const ()), // sub_1416d0fe0
    Replacement::draft(0x1416d1080, "kaim_1416d1080", fn_1416d1080 as *const ()), // sub_1416d1080
    Replacement::draft(0x1416d1800, "kaim_1416d1800", fn_1416d1800 as *const ()), // sub_1416d1800
    Replacement::draft(0x1416d1ab0, "kaim_1416d1ab0", fn_1416d1ab0 as *const ()), // sub_1416d1ab0
    Replacement::draft(0x1416d2500, "kaim_1416d2500", fn_1416d2500 as *const ()), // sub_1416d2500
    Replacement::draft(0x1416d2f60, "kaim_1416d2f60", fn_1416d2f60 as *const ()), // sub_1416d2f60
    Replacement::draft(0x1416d3050, "kaim_1416d3050", fn_1416d3050 as *const ()), // sub_1416d3050
    Replacement::draft(0x1416d3150, "kaim_1416d3150", fn_1416d3150 as *const ()), // Kaim::InsidePosFromOutsidePosQuery<navigation::NavigationTraverseLogic>::Kaim::InsidePosFromOutsidePosQuery<navigation::
    Replacement::draft(0x1416d3240, "kaim_1416d3240", fn_1416d3240 as *const ()), // sub_1416d3240
    Replacement::draft(0x1416d3b30, "kaim_1416d3b30", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aad78> as *const ()), // sub_1416d3b30
    Replacement::draft(0x1416d3b90, "kaim_1416d3b90", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aadb0, 0x8> as *const ()), // sub_1416d3b90
    Replacement::draft(0x1416d3bd0, "kaim_1416d3bd0", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aad78> as *const ()), // sub_1416d3bd0
    Replacement::draft(0x1416d3c30, "kaim_1416d3c30", fn_1416d3c30 as *const ()), // sub_1416d3c30
    Replacement::draft(0x1416d3e60, "kaim_1416d3e60", fn_1416d3e60 as *const ()), // sub_1416d3e60
    Replacement::draft(0x1416d3f70, "kaim_1416d3f70", tpl_22446::<0x1416d42e0> as *const ()), // sub_1416d3f70
    Replacement::draft(0x1416d3f80, "kaim_1416d3f80", fn_1416d3f80 as *const ()), // sub_1416d3f80
    Replacement::draft(0x1416d3f90, "kaim_1416d3f90", tpl_1::<0x1426aad40> as *const ()), // sub_1416d3f90
    Replacement::draft(0x1416d3fa0, "kaim_1416d3fa0", fn_1416d3fa0 as *const ()), // Kaim::InsidePosFromOutsidePosQuery<navigation::NavigationTraverseLogic>::Kaim::InsidePosFromOutsidePosQuery<navigation::
    Replacement::draft(0x1416d41f0, "kaim_1416d41f0", fn_1416d41f0 as *const ()), // sub_1416d41f0
    Replacement::draft(0x1416d42e0, "kaim_1416d42e0", fn_1416d42e0 as *const ()), // sub_1416d42e0
    Replacement::draft(0x1416d43a0, "kaim_1416d43a0", fn_1416d43a0 as *const ()), // sub_1416d43a0
    Replacement::draft(0x1416d4440, "kaim_1416d4440", fn_1416d4440 as *const ()), // sub_1416d4440
    Replacement::draft(0x1416d44d0, "kaim_1416d44d0", fn_1416d44d0 as *const ()), // sub_1416d44d0
    Replacement::draft(0x1416d4570, "kaim_1416d4570", tpl_31930::<0x1426aae20, 0x1416d4750, 0x1416a30e0, 0x8, 0x10, 0x18, 0x20> as *const ()), // Kaim::TriangleFromPosQuery::Kaim::TriangleFromPosQuery::vfunc_4
    Replacement::draft(0x1416d4650, "kaim_1416d4650", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aae30> as *const ()), // sub_1416d4650
    Replacement::draft(0x1416d46b0, "kaim_1416d46b0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aae68, 0x8> as *const ()), // sub_1416d46b0
    Replacement::draft(0x1416d46f0, "kaim_1416d46f0", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aae30> as *const ()), // sub_1416d46f0
    Replacement::draft(0x1416d4750, "kaim_1416d4750", fn_1416d4750 as *const ()), // sub_1416d4750
    Replacement::draft(0x1416d4930, "kaim_1416d4930", fn_1416d4930 as *const ()), // sub_1416d4930
    Replacement::draft(0x1416d4a20, "kaim_1416d4a20", tpl_22446::<0x1416d5830> as *const ()), // sub_1416d4a20
    Replacement::draft(0x1416d4a30, "kaim_1416d4a30", fn_1416d4a30 as *const ()), // sub_1416d4a30
    Replacement::draft(0x1416d4db0, "kaim_1416d4db0", fn_1416d4db0 as *const ()), // sub_1416d4db0
    Replacement::draft(0x1416d5240, "kaim_1416d5240", fn_1416d5240 as *const ()), // sub_1416d5240
    Replacement::draft(0x1416d5250, "kaim_1416d5250", tpl_1::<0x1426aae00> as *const ()), // sub_1416d5250
    Replacement::draft(0x1416d5260, "kaim_1416d5260", fn_1416d5260 as *const ()), // sub_1416d5260
    Replacement::draft(0x1416d5500, "kaim_1416d5500", fn_1416d5500 as *const ()), // Kaim::TriangleFromPosQuery::Kaim::TriangleFromPosQuery::vfunc_5
    Replacement::draft(0x1416d5660, "kaim_1416d5660", fn_1416d5660 as *const ()), // sub_1416d5660
    Replacement::draft(0x1416d56f0, "kaim_1416d56f0", fn_1416d56f0 as *const ()), // sub_1416d56f0
    Replacement::draft(0x1416d57d0, "kaim_1416d57d0", fn_1416d57d0 as *const ()), // sub_1416d57d0
    Replacement::draft(0x1416d5830, "kaim_1416d5830", tpl_31939::<0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x1416d58d0> as *const ()), // sub_1416d5830
    Replacement::draft(0x1416d58d0, "kaim_1416d58d0", fn_1416d58d0 as *const ()), // sub_1416d58d0
    Replacement::draft(0x1416d5970, "kaim_1416d5970", fn_1416d5970 as *const ()), // sub_1416d5970
    Replacement::draft(0x1416d5a70, "kaim_1416d5a70", fn_1416d5a70 as *const ()), // sub_1416d5a70
    Replacement::draft(0x1416d5ac0, "kaim_1416d5ac0", fn_1416d5ac0 as *const ()), // sub_1416d5ac0
    Replacement::draft(0x1416d5c00, "kaim_1416d5c00", fn_1416d5c00 as *const ()), // sub_1416d5c00
    Replacement::draft(0x1416d5c90, "kaim_1416d5c90", fn_1416d5c90 as *const ()), // sub_1416d5c90
    Replacement::draft(0x1416d5e80, "kaim_1416d5e80", fn_1416d5e80 as *const ()), // sub_1416d5e80
    Replacement::draft(0x1416d61c0, "kaim_1416d61c0", fn_1416d61c0 as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_4
    Replacement::draft(0x1416d62b0, "kaim_1416d62b0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aaf08> as *const ()), // sub_1416d62b0
    Replacement::draft(0x1416d6310, "kaim_1416d6310", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aaf50, 0x8> as *const ()), // sub_1416d6310
    Replacement::draft(0x1416d6350, "kaim_1416d6350", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aaf08> as *const ()), // sub_1416d6350
    Replacement::draft(0x1416d63b0, "kaim_1416d63b0", fn_1416d63b0 as *const ()), // sub_1416d63b0
    Replacement::draft(0x1416d64f0, "kaim_1416d64f0", fn_1416d64f0 as *const ()), // sub_1416d64f0
    Replacement::draft(0x1416d6720, "kaim_1416d6720", fn_1416d6720 as *const ()), // sub_1416d6720
    Replacement::draft(0x1416d6a10, "kaim_1416d6a10", fn_1416d6a10 as *const ()), // sub_1416d6a10
    Replacement::draft(0x1416d6c90, "kaim_1416d6c90", fn_1416d6c90 as *const ()), // sub_1416d6c90
    Replacement::draft(0x1416d6cd0, "kaim_1416d6cd0", tpl_22446::<0x1416d75c0> as *const ()), // sub_1416d6cd0
    Replacement::draft(0x1416d6ce0, "kaim_1416d6ce0", fn_1416d6ce0 as *const ()), // sub_1416d6ce0
    Replacement::draft(0x1416d6cf0, "kaim_1416d6cf0", tpl_1::<0x1426aaee8> as *const ()), // sub_1416d6cf0
    Replacement::draft(0x1416d6d00, "kaim_1416d6d00", fn_1416d6d00 as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_10
    Replacement::draft(0x1416d6d80, "kaim_1416d6d80", fn_1416d6d80 as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_8
    Replacement::draft(0x1416d6f00, "kaim_1416d6f00", fn_1416d6f00 as *const ()), // Kaim::AStarQuery<navigation::NavigationTraverseLogic>::Kaim::AStarQuery<navigation::NavigationTraverseLogic>::vfunc_5
    Replacement::draft(0x1416d6fa0, "kaim_1416d6fa0", fn_1416d6fa0 as *const ()), // sub_1416d6fa0
    Replacement::draft(0x1416d7220, "kaim_1416d7220", fn_1416d7220 as *const ()), // sub_1416d7220
    Replacement::draft(0x1416d7500, "kaim_1416d7500", fn_1416d7500 as *const ()), // sub_1416d7500
    Replacement::draft(0x1416d75c0, "kaim_1416d75c0", fn_1416d75c0 as *const ()), // sub_1416d75c0
    Replacement::draft(0x1416d7690, "kaim_1416d7690", fn_1416d7690 as *const ()), // sub_1416d7690
    Replacement::draft(0x1416d76f0, "kaim_1416d76f0", fn_1416d76f0 as *const ()), // sub_1416d76f0
    Replacement::draft(0x1416d77c0, "kaim_1416d77c0", fn_1416d77c0 as *const ()), // sub_1416d77c0
    Replacement::draft(0x1416d78f0, "kaim_1416d78f0", fn_1416d78f0 as *const ()), // sub_1416d78f0
    Replacement::draft(0x1416d83d0, "kaim_1416d83d0", fn_1416d83d0 as *const ()), // sub_1416d83d0
    Replacement::draft(0x1416d8430, "kaim_1416d8430", fn_1416d8430 as *const ()), // sub_1416d8430
    Replacement::draft(0x1416d84f0, "kaim_1416d84f0", fn_1416d84f0 as *const ()), // sub_1416d84f0
    Replacement::draft(0x1416d87b0, "kaim_1416d87b0", fn_1416d87b0 as *const ()), // sub_1416d87b0
    Replacement::draft(0x1416d89a0, "kaim_1416d89a0", fn_1416d89a0 as *const ()), // sub_1416d89a0
    Replacement::draft(0x1416d89b0, "kaim_1416d89b0", fn_1416d89b0 as *const ()), // sub_1416d89b0
    Replacement::draft(0x1416d8a20, "kaim_1416d8a20", fn_1416d8a20 as *const ()), // sub_1416d8a20
    Replacement::draft(0x1416d8a90, "kaim_1416d8a90", fn_1416d8a90 as *const ()), // sub_1416d8a90
    Replacement::draft(0x1416d91a0, "kaim_1416d91a0", fn_1416d91a0 as *const ()), // sub_1416d91a0
    Replacement::draft(0x1416d9580, "kaim_1416d9580", fn_1416d9580 as *const ()), // sub_1416d9580
    Replacement::draft(0x1416d9640, "kaim_1416d9640", fn_1416d9640 as *const ()), // sub_1416d9640
    Replacement::draft(0x1416d9930, "kaim_1416d9930", fn_1416d9930 as *const ()), // sub_1416d9930
    Replacement::draft(0x1416d9ac0, "kaim_1416d9ac0", fn_1416d9ac0 as *const ()), // sub_1416d9ac0
    Replacement::draft(0x1416da050, "kaim_1416da050", fn_1416da050 as *const ()), // sub_1416da050
    Replacement::draft(0x1416da0d0, "kaim_1416da0d0", fn_1416da0d0 as *const ()), // sub_1416da0d0
    Replacement::draft(0x1416da260, "kaim_1416da260", fn_1416da260 as *const ()), // sub_1416da260
    Replacement::draft(0x1416da340, "kaim_1416da340", fn_1416da340 as *const ()), // sub_1416da340
    Replacement::draft(0x1416da3e0, "kaim_1416da3e0", fn_1416da3e0 as *const ()), // sub_1416da3e0
    Replacement::draft(0x1416da5b0, "kaim_1416da5b0", fn_1416da5b0 as *const ()), // sub_1416da5b0
    Replacement::draft(0x1416da740, "kaim_1416da740", fn_1416da740 as *const ()), // sub_1416da740
    Replacement::draft(0x1416da7d0, "kaim_1416da7d0", fn_1416da7d0 as *const ()), // sub_1416da7d0
    Replacement::draft(0x1416daa40, "kaim_1416daa40", fn_1416daa40 as *const ()), // sub_1416daa40
    Replacement::draft(0x1416dab70, "kaim_1416dab70", fn_1416dab70 as *const ()), // sub_1416dab70
    Replacement::draft(0x1416daf20, "kaim_1416daf20", fn_1416daf20 as *const ()), // sub_1416daf20
    Replacement::draft(0x1416db070, "kaim_1416db070", fn_1416db070 as *const ()), // sub_1416db070
    Replacement::draft(0x1416db180, "kaim_1416db180", fn_1416db180 as *const ()), // sub_1416db180
    Replacement::draft(0x1416db260, "kaim_1416db260", fn_1416db260 as *const ()), // sub_1416db260
    Replacement::draft(0x1416db2e0, "kaim_1416db2e0", fn_1416db2e0 as *const ()), // sub_1416db2e0
    Replacement::draft(0x1416db450, "kaim_1416db450", fn_1416db450 as *const ()), // sub_1416db450
    Replacement::draft(0x1416db540, "kaim_1416db540", fn_1416db540 as *const ()), // sub_1416db540
    Replacement::draft(0x1416db6b0, "kaim_1416db6b0", fn_1416db6b0 as *const ()), // Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::vf
    Replacement::draft(0x1416db7a0, "kaim_1416db7a0", fn_1416db7a0 as *const ()), // sub_1416db7a0
    Replacement::draft(0x1416db7d0, "kaim_1416db7d0", fn_1416db7d0 as *const ()), // sub_1416db7d0
    Replacement::draft(0x1416db7f0, "kaim_1416db7f0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab010> as *const ()), // sub_1416db7f0
    Replacement::draft(0x1416db850, "kaim_1416db850", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ab048, 0x8> as *const ()), // sub_1416db850
    Replacement::draft(0x1416db890, "kaim_1416db890", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab010> as *const ()), // sub_1416db890
    Replacement::draft(0x1416db8f0, "kaim_1416db8f0", fn_1416db8f0 as *const ()), // sub_1416db8f0
    Replacement::draft(0x1416dbb60, "kaim_1416dbb60", fn_1416dbb60 as *const ()), // sub_1416dbb60
    Replacement::draft(0x1416dbe00, "kaim_1416dbe00", tpl_22446::<0x1416dc250> as *const ()), // sub_1416dbe00
    Replacement::draft(0x1416dbe10, "kaim_1416dbe10", fn_1416dbe10 as *const ()), // sub_1416dbe10
    Replacement::draft(0x1416dbe20, "kaim_1416dbe20", tpl_1::<0x1426aafe8> as *const ()), // sub_1416dbe20
    Replacement::draft(0x1416dbe30, "kaim_1416dbe30", fn_1416dbe30 as *const ()), // Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCanGoQuery<navigation::NavigationTraverseLogic>::vf
    Replacement::draft(0x1416dc100, "kaim_1416dc100", fn_1416dc100 as *const ()), // sub_1416dc100
    Replacement::draft(0x1416dc250, "kaim_1416dc250", fn_1416dc250 as *const ()), // sub_1416dc250
    Replacement::draft(0x1416dc310, "kaim_1416dc310", fn_1416dc310 as *const ()), // sub_1416dc310
    Replacement::draft(0x1416dc3d0, "kaim_1416dc3d0", fn_1416dc3d0 as *const ()), // sub_1416dc3d0
    Replacement::draft(0x1416dc4b0, "kaim_1416dc4b0", fn_1416dc4b0 as *const ()), // sub_1416dc4b0
    Replacement::draft(0x1416dc620, "kaim_1416dc620", fn_1416dc620 as *const ()), // Kaim::RayCanGoQuery<Kaim::anonymous_namespace::ExtendedPredicate<navigation::NavigationTraverseLogic>>::Kaim::RayCanGoQu
    Replacement::draft(0x1416dc710, "kaim_1416dc710", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab0c0> as *const ()), // sub_1416dc710
    Replacement::draft(0x1416dc770, "kaim_1416dc770", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ab0f8, 0x8> as *const ()), // sub_1416dc770
    Replacement::draft(0x1416dc7b0, "kaim_1416dc7b0", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab0c0> as *const ()), // sub_1416dc7b0
    Replacement::draft(0x1416dc810, "kaim_1416dc810", fn_1416dc810 as *const ()), // sub_1416dc810
    Replacement::draft(0x1416dcae0, "kaim_1416dcae0", fn_1416dcae0 as *const ()), // sub_1416dcae0
    Replacement::draft(0x1416dcda0, "kaim_1416dcda0", tpl_22446::<0x1416dd490> as *const ()), // sub_1416dcda0
    Replacement::draft(0x1416dcdb0, "kaim_1416dcdb0", fn_1416dcdb0 as *const ()), // sub_1416dcdb0
    Replacement::draft(0x1416dcdc0, "kaim_1416dcdc0", tpl_1::<0x1426ab098> as *const ()), // sub_1416dcdc0
    Replacement::draft(0x1416dcdd0, "kaim_1416dcdd0", fn_1416dcdd0 as *const ()), // Kaim::RayCanGoQuery<Kaim::anonymous_namespace::ExtendedPredicate<navigation::NavigationTraverseLogic>>::Kaim::RayCanGoQu
    Replacement::draft(0x1416dd0b0, "kaim_1416dd0b0", fn_1416dd0b0 as *const ()), // sub_1416dd0b0
    Replacement::draft(0x1416dd200, "kaim_1416dd200", fn_1416dd200 as *const ()), // sub_1416dd200
    Replacement::draft(0x1416dd490, "kaim_1416dd490", fn_1416dd490 as *const ()), // sub_1416dd490
    Replacement::draft(0x1416dd560, "kaim_1416dd560", fn_1416dd560 as *const ()), // sub_1416dd560
    Replacement::draft(0x1416dd620, "kaim_1416dd620", fn_1416dd620 as *const ()), // sub_1416dd620
    Replacement::draft(0x1416dd720, "kaim_1416dd720", fn_1416dd720 as *const ()), // sub_1416dd720
    Replacement::draft(0x1416dd8f0, "kaim_1416dd8f0", fn_1416dd8f0 as *const ()), // sub_1416dd8f0
    Replacement::draft(0x1416dd9e0, "kaim_1416dd9e0", fn_1416dd9e0 as *const ()), // sub_1416dd9e0
    Replacement::draft(0x1416ddb00, "kaim_1416ddb00", fn_1416ddb00 as *const ()), // sub_1416ddb00
    Replacement::draft(0x1416ddbc0, "kaim_1416ddbc0", fn_1416ddbc0 as *const ()), // sub_1416ddbc0
    Replacement::draft(0x1416ddca0, "kaim_1416ddca0", fn_1416ddca0 as *const ()), // sub_1416ddca0
    Replacement::draft(0x1416ddde0, "kaim_1416ddde0", fn_1416ddde0 as *const ()), // sub_1416ddde0
    Replacement::draft(0x1416dde40, "kaim_1416dde40", fn_1416dde40 as *const ()), // sub_1416dde40
    Replacement::draft(0x1416dded0, "kaim_1416dded0", fn_1416dded0 as *const ()), // sub_1416dded0
    Replacement::draft(0x1416ddf90, "kaim_1416ddf90", fn_1416ddf90 as *const ()), // sub_1416ddf90
    Replacement::draft(0x1416de150, "kaim_1416de150", fn_1416de150 as *const ()), // sub_1416de150
    Replacement::draft(0x1416de2a0, "kaim_1416de2a0", fn_1416de2a0 as *const ()), // sub_1416de2a0
    Replacement::draft(0x1416de3d0, "kaim_1416de3d0", fn_1416de3d0 as *const ()), // sub_1416de3d0
    Replacement::draft(0x1416de410, "kaim_1416de410", tpl_31603::<0x143932190, 0x60> as *const ()), // sub_1416de410
    Replacement::draft(0x1416de430, "kaim_1416de430", fn_1416de430 as *const ()), // sub_1416de430
    Replacement::draft(0x1416de4a0, "kaim_1416de4a0", fn_1416de4a0 as *const ()), // sub_1416de4a0
    Replacement::draft(0x1416de510, "kaim_1416de510", fn_1416de510 as *const ()), // sub_1416de510
    Replacement::draft(0x1416de740, "kaim_1416de740", fn_1416de740 as *const ()), // sub_1416de740
    Replacement::draft(0x1416de7b0, "kaim_1416de7b0", fn_1416de7b0 as *const ()), // sub_1416de7b0
    Replacement::draft(0x1416de8f0, "kaim_1416de8f0", fn_1416de8f0 as *const ()), // sub_1416de8f0
    Replacement::draft(0x1416deae0, "kaim_1416deae0", fn_1416deae0 as *const ()), // Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::vfun
    Replacement::draft(0x1416debd0, "kaim_1416debd0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab180> as *const ()), // sub_1416debd0
    Replacement::draft(0x1416dec30, "kaim_1416dec30", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ab1b8, 0x8> as *const ()), // sub_1416dec30
    Replacement::draft(0x1416dec70, "kaim_1416dec70", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab180> as *const ()), // sub_1416dec70
    Replacement::draft(0x1416decd0, "kaim_1416decd0", fn_1416decd0 as *const ()), // sub_1416decd0
    Replacement::draft(0x1416deee0, "kaim_1416deee0", fn_1416deee0 as *const ()), // sub_1416deee0
    Replacement::draft(0x1416df2b0, "kaim_1416df2b0", tpl_22446::<0x1416df520> as *const ()), // sub_1416df2b0
    Replacement::draft(0x1416df2c0, "kaim_1416df2c0", fn_1416df2c0 as *const ()), // sub_1416df2c0
    Replacement::draft(0x1416df2d0, "kaim_1416df2d0", tpl_1::<0x1426ab158> as *const ()), // sub_1416df2d0
    Replacement::draft(0x1416df2e0, "kaim_1416df2e0", fn_1416df2e0 as *const ()), // Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::Kaim::DiskCastQuery<navigation::NavigationTraverseLogic>::vfun
    Replacement::draft(0x1416df360, "kaim_1416df360", fn_1416df360 as *const ()), // sub_1416df360
    Replacement::draft(0x1416df520, "kaim_1416df520", fn_1416df520 as *const ()), // sub_1416df520
    Replacement::draft(0x1416df5f0, "kaim_1416df5f0", fn_1416df5f0 as *const ()), // sub_1416df5f0
    Replacement::draft(0x1416df600, "kaim_1416df600", fn_1416df600 as *const ()), // sub_1416df600
    Replacement::draft(0x1416df8a0, "kaim_1416df8a0", fn_1416df8a0 as *const ()), // sub_1416df8a0
    Replacement::draft(0x1416dfbb0, "kaim_1416dfbb0", fn_1416dfbb0 as *const ()), // sub_1416dfbb0
    Replacement::draft(0x1416dfc70, "kaim_1416dfc70", fn_1416dfc70 as *const ()), // sub_1416dfc70
    Replacement::draft(0x1416dfcb0, "kaim_1416dfcb0", fn_1416dfcb0 as *const ()), // sub_1416dfcb0
    Replacement::draft(0x1416dfcf0, "kaim_1416dfcf0", fn_1416dfcf0 as *const ()), // sub_1416dfcf0
    Replacement::draft(0x1416dfd00, "kaim_1416dfd00", fn_1416dfd00 as *const ()), // sub_1416dfd00
    Replacement::draft(0x1416dfdb0, "kaim_1416dfdb0", fn_1416dfdb0 as *const ()), // sub_1416dfdb0
    Replacement::draft(0x1416e0080, "kaim_1416e0080", fn_1416e0080 as *const ()), // sub_1416e0080
    Replacement::draft(0x1416e00e0, "kaim_1416e00e0", fn_1416e00e0 as *const ()), // sub_1416e00e0
    Replacement::draft(0x1416e06a0, "kaim_1416e06a0", fn_1416e06a0 as *const ()), // sub_1416e06a0
    Replacement::draft(0x1416e0b60, "kaim_1416e0b60", fn_1416e0b60 as *const ()), // sub_1416e0b60
    Replacement::draft(0x1416e1530, "kaim_1416e1530", fn_1416e1530 as *const ()), // sub_1416e1530
    Replacement::draft(0x1416e1590, "kaim_1416e1590", fn_1416e1590 as *const ()), // sub_1416e1590
    Replacement::draft(0x1416e1760, "kaim_1416e1760", fn_1416e1760 as *const ()), // sub_1416e1760
    Replacement::draft(0x1416e19f0, "kaim_1416e19f0", fn_1416e19f0 as *const ()), // sub_1416e19f0
    Replacement::draft(0x1416e1c70, "kaim_1416e1c70", fn_1416e1c70 as *const ()), // sub_1416e1c70
    Replacement::draft(0x1416e1ef0, "kaim_1416e1ef0", fn_1416e1ef0 as *const ()), // sub_1416e1ef0
    Replacement::draft(0x1416e2170, "kaim_1416e2170", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab360> as *const ()), // sub_1416e2170
    Replacement::draft(0x1416e21d0, "kaim_1416e21d0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ab398, 0x8> as *const ()), // sub_1416e21d0
    Replacement::draft(0x1416e2210, "unknown_1416e2210", fn_1416e2210 as *const ()), // sub_1416e2210
    Replacement::draft(0x1416e2250, "unknown_1416e2250", fn_1416e2250 as *const ()), // sub_1416e2250
    Replacement::draft(0x1416e2260, "unknown_1416e2260", tpl_1::<0x1426ab228> as *const ()), // sub_1416e2260
    Replacement::draft(0x1416e2270, "unknown_1416e2270", fn_1416e2270 as *const ()), // sub_1416e2270
    Replacement::draft(0x1416e2340, "unknown_1416e2340", fn_1416e2340 as *const ()), // sub_1416e2340
    Replacement::draft(0x1416e24d0, "unknown_1416e24d0", fn_1416e24d0 as *const ()), // sub_1416e24d0
    Replacement::draft(0x1416e2690, "unknown_1416e2690", fn_1416e2690 as *const ()), // sub_1416e2690
    Replacement::draft(0x1416e2890, "unknown_1416e2890", fn_1416e2890 as *const ()), // sub_1416e2890
    Replacement::draft(0x1416e2c60, "unknown_1416e2c60", fn_1416e2c60 as *const ()), // sub_1416e2c60
    Replacement::draft(0x1416e30e0, "unknown_1416e30e0", fn_1416e30e0 as *const ()), // sub_1416e30e0
    Replacement::draft(0x1416e32c0, "unknown_1416e32c0", fn_1416e32c0 as *const ()), // sub_1416e32c0
    Replacement::draft(0x1416e33a0, "unknown_1416e33a0", fn_1416e33a0 as *const ()), // sub_1416e33a0
    Replacement::draft(0x1416e3590, "unknown_1416e3590", fn_1416e3590 as *const ()), // sub_1416e3590
    Replacement::draft(0x1416e3680, "unknown_1416e3680", fn_1416e3680 as *const ()), // sub_1416e3680
    Replacement::draft(0x1416e3900, "unknown_1416e3900", fn_1416e3900 as *const ()), // sub_1416e3900
    Replacement::draft(0x1416e3b30, "physx_1416e3b30", fn_1416e3b30 as *const ()), // sub_1416e3b30
    Replacement::draft(0x1416e3c50, "unknown_1416e3c50", fn_1416e3c50 as *const ()), // sub_1416e3c50
    Replacement::draft(0x1416e3d30, "unknown_1416e3d30", fn_1416e3d30 as *const ()), // sub_1416e3d30
    Replacement::draft(0x1416e3e20, "unknown_1416e3e20", fn_1416e3e20 as *const ()), // sub_1416e3e20
    Replacement::draft(0x1416e3ed0, "unknown_1416e3ed0", fn_1416e3ed0 as *const ()), // sub_1416e3ed0
    Replacement::draft(0x1416e4010, "unknown_1416e4010", fn_1416e4010 as *const ()), // sub_1416e4010
    Replacement::draft(0x1416e4290, "unknown_1416e4290", fn_1416e4290 as *const ()), // sub_1416e4290
    Replacement::draft(0x1416e4590, "unknown_1416e4590", fn_1416e4590 as *const ()), // sub_1416e4590
    Replacement::draft(0x1416e48e0, "unknown_1416e48e0", fn_1416e48e0 as *const ()), // sub_1416e48e0
    Replacement::draft(0x1416e4a00, "unknown_1416e4a00", fn_1416e4a00 as *const ()), // sub_1416e4a00
    Replacement::draft(0x1416e5360, "unknown_1416e5360", fn_1416e5360 as *const ()), // sub_1416e5360
    Replacement::draft(0x1416e5430, "unknown_1416e5430", fn_1416e5430 as *const ()), // sub_1416e5430
    Replacement::draft(0x1416e5530, "unknown_1416e5530", fn_1416e5530 as *const ()), // sub_1416e5530
    Replacement::draft(0x1416e57d0, "unknown_1416e57d0", fn_1416e57d0 as *const ()), // sub_1416e57d0
    Replacement::draft(0x1416e5a60, "kaim_1416e5a60", fn_1416e5a60 as *const ()), // sub_1416e5a60
    Replacement::draft(0x1416e5b40, "kaim_1416e5b40", fn_1416e5b40 as *const ()), // sub_1416e5b40
    Replacement::draft(0x1416e5ca0, "kaim_1416e5ca0", tpl_31930::<0x1426ab410, 0x1416e5e80, 0x1416a30e0, 0x8, 0x10, 0x18, 0x20> as *const ()), // Kaim::NearestBorderHalfEdgeFromPosQuery<navigation::NavigationTraverseLogic>::Kaim::NearestBorderHalfEdgeFromPosQuery<na
    Replacement::draft(0x1416e5d80, "kaim_1416e5d80", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab420> as *const ()), // sub_1416e5d80
    Replacement::draft(0x1416e5de0, "kaim_1416e5de0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ab458, 0x8> as *const ()), // sub_1416e5de0
    Replacement::draft(0x1416e5e20, "kaim_1416e5e20", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ab420> as *const ()), // sub_1416e5e20
    Replacement::draft(0x1416e5e80, "kaim_1416e5e80", fn_1416e5e80 as *const ()), // sub_1416e5e80
    Replacement::draft(0x1416e6090, "kaim_1416e6090", fn_1416e6090 as *const ()), // sub_1416e6090
    Replacement::draft(0x1416e6330, "kaim_1416e6330", tpl_22446::<0x1416e6780> as *const ()), // sub_1416e6330
    Replacement::draft(0x1416e6340, "kaim_1416e6340", fn_1416e6340 as *const ()), // sub_1416e6340
    Replacement::draft(0x1416e6350, "kaim_1416e6350", tpl_1::<0x1426ab3e8> as *const ()), // sub_1416e6350
    Replacement::draft(0x1416e6360, "kaim_1416e6360", fn_1416e6360 as *const ()), // Kaim::NearestBorderHalfEdgeFromPosQuery<navigation::NavigationTraverseLogic>::Kaim::NearestBorderHalfEdgeFromPosQuery<na
    Replacement::draft(0x1416e6630, "kaim_1416e6630", fn_1416e6630 as *const ()), // sub_1416e6630
    Replacement::draft(0x1416e6780, "kaim_1416e6780", tpl_31939::<0x4, 0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x1416e6820> as *const ()), // sub_1416e6780
    Replacement::draft(0x1416e6820, "kaim_1416e6820", fn_1416e6820 as *const ()), // sub_1416e6820
    Replacement::draft(0x1416e6950, "kaim_1416e6950", fn_1416e6950 as *const ()), // sub_1416e6950
    Replacement::draft(0x1416e7400, "kaim_1416e7400", fn_1416e7400 as *const ()), // sub_1416e7400
    Replacement::draft(0x1416e7810, "kaim_1416e7810", fn_1416e7810 as *const ()), // sub_1416e7810
    Replacement::draft(0x1416e7b10, "kaim_1416e7b10", fn_1416e7b10 as *const ()), // sub_1416e7b10
    Replacement::draft(0x1416e8640, "kaim_1416e8640", fn_1416e8640 as *const ()), // sub_1416e8640
    Replacement::draft(0x1416e9130, "kaim_1416e9130", fn_1416e9130 as *const ()), // sub_1416e9130
    Replacement::draft(0x1416e94c0, "kaim_1416e94c0", fn_1416e94c0 as *const ()), // sub_1416e94c0
    Replacement::draft(0x1416e9720, "kaim_1416e9720", fn_1416e9720 as *const ()), // sub_1416e9720
    Replacement::draft(0x1416e9ca0, "kaim_1416e9ca0", fn_1416e9ca0 as *const ()), // sub_1416e9ca0
    Replacement::draft(0x1416e9fc0, "kaim_1416e9fc0", fn_1416e9fc0 as *const ()), // sub_1416e9fc0
    Replacement::draft(0x1416ea3b0, "kaim_1416ea3b0", fn_1416ea3b0 as *const ()), // sub_1416ea3b0
    Replacement::draft(0x1416ea820, "kaim_1416ea820", fn_1416ea820 as *const ()), // sub_1416ea820
    Replacement::draft(0x1416eab20, "kaim_1416eab20", fn_1416eab20 as *const ()), // sub_1416eab20
    Replacement::draft(0x1416eb9a0, "kaim_1416eb9a0", fn_1416eb9a0 as *const ()), // sub_1416eb9a0
    Replacement::draft(0x1416ec820, "kaim_1416ec820", fn_1416ec820 as *const ()), // sub_1416ec820
    Replacement::draft(0x1416ecf80, "kaim_1416ecf80", fn_1416ecf80 as *const ()), // sub_1416ecf80
    Replacement::draft(0x1416ed920, "kaim_1416ed920", fn_1416ed920 as *const ()), // sub_1416ed920
    Replacement::draft(0x1416ee3d0, "kaim_1416ee3d0", fn_1416ee3d0 as *const ()), // sub_1416ee3d0
    Replacement::draft(0x1416ee5a0, "kaim_1416ee5a0", fn_1416ee5a0 as *const ()), // sub_1416ee5a0
    Replacement::draft(0x1416ee7f0, "kaim_1416ee7f0", fn_1416ee7f0 as *const ()), // sub_1416ee7f0
    Replacement::draft(0x1416eea60, "kaim_1416eea60", fn_1416eea60 as *const ()), // sub_1416eea60
    Replacement::draft(0x1416eeb80, "kaim_1416eeb80", fn_1416eeb80 as *const ()), // sub_1416eeb80
    Replacement::draft(0x1416eeca0, "kaim_1416eeca0", fn_1416eeca0 as *const ()), // sub_1416eeca0
    Replacement::draft(0x1416eefe0, "kaim_1416eefe0", fn_1416eefe0 as *const ()), // sub_1416eefe0
    Replacement::draft(0x1416ef200, "kaim_1416ef200", tpl_31701::<0x1416dde40, 0x1416ddb00, 0x1416e3e20, 0x141200150, 0x1416e4010, 0x1416ef960, 0x1416ddde0, 0x140000000> as *const ()), // sub_1416ef200
    Replacement::draft(0x1416ef5b0, "kaim_1416ef5b0", tpl_31701::<0x1416dde40, 0x1416ddb00, 0x1416e3e20, 0x141200150, 0x1416e4010, 0x1416ef960, 0x1416ddde0, 0x140000000> as *const ()), // sub_1416ef5b0
    Replacement::draft(0x1416ef960, "kaim_1416ef960", fn_1416ef960 as *const ()), // sub_1416ef960
    Replacement::draft(0x1416ef9d0, "kaim_1416ef9d0", fn_1416ef9d0 as *const ()), // sub_1416ef9d0
    Replacement::draft(0x1416eff10, "kaim_1416eff10", tpl_32086::<0x1416b1060, 0x1411fd8d0, 0x1411fbda0, 0x1411feed0, 0x1416b0e60, 0x141705050> as *const ()), // sub_1416eff10
    Replacement::draft(0x1416f0180, "kaim_1416f0180", tpl_32086::<0x1416b1060, 0x1411fd8d0, 0x1411fbda0, 0x1411feed0, 0x1416b0e60, 0x141705340> as *const ()), // sub_1416f0180
    Replacement::draft(0x1416f03f0, "kaim_1416f03f0", tpl_32086::<0x1416b1060, 0x1411fd8d0, 0x1411fbda0, 0x1411feed0, 0x1416b0e60, 0x141705630> as *const ()), // sub_1416f03f0
    Replacement::draft(0x1416f0660, "kaim_1416f0660", fn_1416f0660 as *const ()), // sub_1416f0660
    Replacement::draft(0x1416f0830, "kaim_1416f0830", fn_1416f0830 as *const ()), // sub_1416f0830
    Replacement::draft(0x1416f0ae0, "kaim_1416f0ae0", fn_1416f0ae0 as *const ()), // sub_1416f0ae0
    Replacement::draft(0x1416f0d70, "kaim_1416f0d70", fn_1416f0d70 as *const ()), // sub_1416f0d70
    Replacement::draft(0x1416f0ee0, "kaim_1416f0ee0", fn_1416f0ee0 as *const ()), // sub_1416f0ee0
    Replacement::draft(0x1416f1090, "kaim_1416f1090", fn_1416f1090 as *const ()), // sub_1416f1090
    Replacement::draft(0x1416f1290, "kaim_1416f1290", fn_1416f1290 as *const ()), // sub_1416f1290
    Replacement::draft(0x1416f17c0, "kaim_1416f17c0", fn_1416f17c0 as *const ()), // sub_1416f17c0
    Replacement::draft(0x1416f3c50, "kaim_1416f3c50", fn_1416f3c50 as *const ()), // sub_1416f3c50
    Replacement::draft(0x1416f3e70, "kaim_1416f3e70", fn_1416f3e70 as *const ()), // sub_1416f3e70
    Replacement::draft(0x1416f4290, "kaim_1416f4290", fn_1416f4290 as *const ()), // sub_1416f4290
    Replacement::draft(0x1416f4650, "kaim_1416f4650", fn_1416f4650 as *const ()), // sub_1416f4650
    Replacement::draft(0x1416f4920, "kaim_1416f4920", fn_1416f4920 as *const ()), // sub_1416f4920
    Replacement::draft(0x1416f49c0, "kaim_1416f49c0", fn_1416f49c0 as *const ()), // sub_1416f49c0
    Replacement::draft(0x1416f4c20, "kaim_1416f4c20", fn_1416f4c20 as *const ()), // sub_1416f4c20
    Replacement::draft(0x1416f4ce0, "kaim_1416f4ce0", fn_1416f4ce0 as *const ()), // sub_1416f4ce0
    Replacement::draft(0x1416f4e90, "kaim_1416f4e90", tpl_22649::<0x1416bdab0, 0x1416b7e40, 0xf0, 0x1416f3c50, 0x1416baa10, 0x258, 0x1416b8150, 0x1416bdc60> as *const ()), // sub_1416f4e90
    Replacement::draft(0x1416f4f70, "kaim_1416f4f70", fn_1416f4f70 as *const ()), // sub_1416f4f70
    Replacement::draft(0x1416f51c0, "kaim_1416f51c0", fn_1416f51c0 as *const ()), // sub_1416f51c0
    Replacement::draft(0x1416f5400, "kaim_1416f5400", fn_1416f5400 as *const ()), // sub_1416f5400
    Replacement::draft(0x1416f5ac0, "kaim_1416f5ac0", fn_1416f5ac0 as *const ()), // sub_1416f5ac0
    Replacement::draft(0x1416f5b60, "kaim_1416f5b60", tpl_22426::<0x1426ab728, 0x141200d80> as *const ()), // sub_1416f5b60
    Replacement::draft(0x1416f5b70, "kaim_1416f5b70", tpl_22426::<0x1426ab698, 0x1412268d0> as *const ()), // sub_1416f5b70
    Replacement::draft(0x1416f5b80, "kaim_1416f5b80", fn_1416f5b80 as *const ()), // sub_1416f5b80
    Replacement::draft(0x1416f5be0, "kaim_1416f5be0", tpl_32103::<0x10> as *const ()), // sub_1416f5be0
    Replacement::draft(0x1416f5c00, "kaim_1416f5c00", tpl_22426::<0x1426ab5d8, 0x1411fbbe0> as *const ()), // sub_1416f5c00
    Replacement::draft(0x1416f5c10, "kaim_1416f5c10", tpl_365::<0x14169ed90> as *const ()), // sub_1416f5c10
    Replacement::draft(0x1416f5c20, "kaim_1416f5c20", fn_1416f5c20 as *const ()), // sub_1416f5c20
    Replacement::draft(0x1416f5cd0, "kaim_1416f5cd0", tpl_31841::<0x30, 0x143932190, 0x60, 0x18> as *const ()), // sub_1416f5cd0
    Replacement::draft(0x1416f5d10, "kaim_1416f5d10", tpl_22425::<0x8, 0x143932190, 0x60> as *const ()), // sub_1416f5d10
    Replacement::draft(0x1416f5d40, "kaim_1416f5d40", fn_1416f5d40 as *const ()), // sub_1416f5d40
    Replacement::draft(0x1416f5d70, "kaim_1416f5d70", fn_1416f5d70 as *const ()), // sub_1416f5d70
    Replacement::draft(0x1416f6030, "kaim_1416f6030", tpl_22569::<0x1426ab530, 0x141201140, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416f6030
    Replacement::draft(0x1416f60a0, "kaim_1416f60a0", tpl_22645::<0x1426ab728, 0x141200d80, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416f60a0
    Replacement::draft(0x1416f6110, "kaim_1416f6110", tpl_22714::<0x1426ab698, 0x1412268d0, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416f6110
    Replacement::draft(0x1416f6180, "kaim_1416f6180", fn_1416f6180 as *const ()), // sub_1416f6180
    Replacement::draft(0x1416f6230, "kaim_1416f6230", fn_1416f6230 as *const ()), // sub_1416f6230
    Replacement::draft(0x1416f62e0, "kaim_1416f62e0", tpl_22683::<0x1426ab520, 0x14170bbd0, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416f62e0
    Replacement::draft(0x1416f6350, "kaim_1416f6350", tpl_22538::<0x1412009a0, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416f6350
    Replacement::draft(0x1416f63b0, "kaim_1416f63b0", tpl_22484::<0x1426ab5d8, 0x1411fbbe0, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416f63b0
    Replacement::draft(0x1416f6420, "kaim_1416f6420", fn_1416f6420 as *const ()), // sub_1416f6420
    Replacement::draft(0x1416f64b0, "kaim_1416f64b0", tpl_22748::<0x1416f5d70, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_1416f64b0
    Replacement::draft(0x1416f6510, "kaim_1416f6510", fn_1416f6510 as *const ()), // sub_1416f6510
    Replacement::draft(0x1416f65f0, "kaim_1416f65f0", fn_1416f65f0 as *const ()), // sub_1416f65f0
    Replacement::draft(0x1416f66e0, "kaim_1416f66e0", fn_1416f66e0 as *const ()), // sub_1416f66e0
    Replacement::draft(0x1416f6890, "kaim_1416f6890", fn_1416f6890 as *const ()), // sub_1416f6890
    Replacement::draft(0x1416f6a40, "kaim_1416f6a40", fn_1416f6a40 as *const ()), // sub_1416f6a40
    Replacement::draft(0x1416f6bf0, "kaim_1416f6bf0", tpl_22574::<0x1416f7130, 0x80, 0x48, 0x10, 0x1a0> as *const ()), // sub_1416f6bf0
    Replacement::draft(0x1416f6c30, "kaim_1416f6c30", tpl_31756::<0xe0, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x6c, 0x1416b1580, 0x1416ff3c0> as *const ()), // sub_1416f6c30
    Replacement::draft(0x1416f6ce0, "kaim_1416f6ce0", tpl_32111::<0x84, 0x20, 0x68, 0x10, 0xa0, 0x58, 0x48, 0x5c, 0x1416b1580, 0x1416ffc30> as *const ()), // sub_1416f6ce0
    Replacement::draft(0x1416f6d90, "kaim_1416f6d90", tpl_22465::<0x1416fe320, 0x10> as *const ()), // sub_1416f6d90
    Replacement::draft(0x1416f6db0, "kaim_1416f6db0", fn_1416f6db0 as *const ()), // sub_1416f6db0
    Replacement::draft(0x1416f6e60, "kaim_1416f6e60", tpl_22465::<0x1416fee70, 0x10> as *const ()), // sub_1416f6e60
    Replacement::draft(0x1416f6e80, "kaim_1416f6e80", tpl_22465::<0x1416fef40, 0x10> as *const ()), // sub_1416f6e80
    Replacement::draft(0x1416f6ea0, "kaim_1416f6ea0", tpl_31756::<0xc0, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x6c, 0x1416b1580, 0x141700f70> as *const ()), // sub_1416f6ea0
    Replacement::draft(0x1416f6f50, "kaim_1416f6f50", tpl_22465::<0x1416ff1f0, 0x10> as *const ()), // sub_1416f6f50
    Replacement::draft(0x1416f6f70, "kaim_1416f6f70", fn_1416f6f70 as *const ()), // sub_1416f6f70
    Replacement::draft(0x1416f7060, "kaim_1416f7060", tpl_22465::<0x1416ff2f0, 0x10> as *const ()), // sub_1416f7060
    Replacement::draft(0x1416f7080, "kaim_1416f7080", tpl_31756::<0xd0, 0x20, 0x68, 0x10, 0xa0, 0x48, 0x6c, 0x1416b1580, 0x141702b60> as *const ()), // sub_1416f7080
    Replacement::draft(0x1416f7130, "unknown_1416f7130", fn_1416f7130 as *const ()), // sub_1416f7130
    Replacement::draft(0x1416f7fe0, "unknown_1416f7fe0", fn_1416f7fe0 as *const ()), // sub_1416f7fe0
    Replacement::draft(0x1416f8530, "unknown_1416f8530", fn_1416f8530 as *const ()), // sub_1416f8530
    Replacement::draft(0x1416f8540, "unknown_1416f8540", fn_1416f8540 as *const ()), // sub_1416f8540
    Replacement::draft(0x1416f85e0, "unknown_1416f85e0", fn_1416f85e0 as *const ()), // sub_1416f85e0
    Replacement::draft(0x1416f8680, "unknown_1416f8680", tpl_22707::<0x14170f7b0, 0x1416dc3d0, 0x1426ab668, 0x1416dc4b0, 0x1416bcaf0, 0x1416dd0b0, 0x143932190, 0x1412009a0, 0x1416b1580, 0x141702240> as *const ()), // sub_1416f8680
    Replacement::draft(0x1416f8880, "unknown_1416f8880", fn_1416f8880 as *const ()), // sub_1416f8880
    Replacement::draft(0x1416f88c0, "unknown_1416f88c0", fn_1416f88c0 as *const ()), // sub_1416f88c0
    Replacement::draft(0x1416f89f0, "unknown_1416f89f0", fn_1416f89f0 as *const ()), // sub_1416f89f0
    Replacement::draft(0x1416f8c60, "unknown_1416f8c60", fn_1416f8c60 as *const ()), // sub_1416f8c60
    Replacement::draft(0x1416f8f10, "unknown_1416f8f10", fn_1416f8f10 as *const ()), // sub_1416f8f10
    Replacement::draft(0x1416f8f90, "unknown_1416f8f90", fn_1416f8f90 as *const ()), // sub_1416f8f90
    Replacement::draft(0x1416f9150, "unknown_1416f9150", fn_1416f9150 as *const ()), // sub_1416f9150
    Replacement::draft(0x1416f9330, "unknown_1416f9330", fn_1416f9330 as *const ()), // sub_1416f9330
    Replacement::draft(0x1416f9430, "unknown_1416f9430", fn_1416f9430 as *const ()), // sub_1416f9430
    Replacement::draft(0x1416f9710, "unknown_1416f9710", fn_1416f9710 as *const ()), // sub_1416f9710
    Replacement::draft(0x1416f9a00, "unknown_1416f9a00", fn_1416f9a00 as *const ()), // sub_1416f9a00
    Replacement::draft(0x1416f9d10, "unknown_1416f9d10", fn_1416f9d10 as *const ()), // sub_1416f9d10
    Replacement::draft(0x1416f9d70, "unknown_1416f9d70", fn_1416f9d70 as *const ()), // sub_1416f9d70
    Replacement::draft(0x1416f9e60, "unknown_1416f9e60", fn_1416f9e60 as *const ()), // sub_1416f9e60
    Replacement::draft(0x1416f9f60, "unknown_1416f9f60", fn_1416f9f60 as *const ()), // sub_1416f9f60
    Replacement::draft(0x1416fa270, "unknown_1416fa270", fn_1416fa270 as *const ()), // sub_1416fa270
    Replacement::draft(0x1416fa450, "unknown_1416fa450", fn_1416fa450 as *const ()), // sub_1416fa450
    Replacement::draft(0x1416fa6a0, "unknown_1416fa6a0", fn_1416fa6a0 as *const ()), // sub_1416fa6a0
    Replacement::draft(0x1416fa940, "unknown_1416fa940", fn_1416fa940 as *const ()), // sub_1416fa940
    Replacement::draft(0x1416fae20, "unknown_1416fae20", fn_1416fae20 as *const ()), // sub_1416fae20
    Replacement::draft(0x1416fb0f0, "unknown_1416fb0f0", fn_1416fb0f0 as *const ()), // sub_1416fb0f0
    Replacement::draft(0x1416fb470, "unknown_1416fb470", fn_1416fb470 as *const ()), // sub_1416fb470
    Replacement::draft(0x1416fb700, "kaim_1416fb700", fn_1416fb700 as *const ()), // sub_1416fb700
    Replacement::draft(0x1416fbbd0, "kaim_1416fbbd0", fn_1416fbbd0 as *const ()), // sub_1416fbbd0
    Replacement::draft(0x1416fc250, "kaim_1416fc250", fn_1416fc250 as *const ()), // sub_1416fc250
    Replacement::draft(0x1416fc380, "kaim_1416fc380", fn_1416fc380 as *const ()), // sub_1416fc380
    Replacement::draft(0x1416fc480, "kaim_1416fc480", tpl_32140::<0x20, 0x143932190, 0x10, 0x48, 0x8, 0x1433e1ee0, 0x14169edb0, 0x14169eda0> as *const ()), // sub_1416fc480
    Replacement::draft(0x1416fc530, "kaim_1416fc530", fn_1416fc530 as *const ()), // sub_1416fc530
    Replacement::draft(0x1416fc630, "kaim_1416fc630", tpl_32140::<0x28, 0x143932190, 0x10, 0x48, 0x8, 0x1433e1e88, 0x14169edb0, 0x14169eda0> as *const ()), // sub_1416fc630
    Replacement::draft(0x1416fc6e0, "kaim_1416fc6e0", fn_1416fc6e0 as *const ()), // sub_1416fc6e0
    Replacement::draft(0x1416fc920, "kaim_1416fc920", fn_1416fc920 as *const ()), // sub_1416fc920
    Replacement::draft(0x1416fcb10, "kaim_1416fcb10", fn_1416fcb10 as *const ()), // sub_1416fcb10
    Replacement::draft(0x1416fcd00, "kaim_1416fcd00", fn_1416fcd00 as *const ()), // sub_1416fcd00
    Replacement::draft(0x1416fd0e0, "kaim_1416fd0e0", fn_1416fd0e0 as *const ()), // sub_1416fd0e0
    Replacement::draft(0x1416fd2b0, "kaim_1416fd2b0", fn_1416fd2b0 as *const ()), // sub_1416fd2b0
    Replacement::draft(0x1416fd530, "kaim_1416fd530", fn_1416fd530 as *const ()), // sub_1416fd530
    Replacement::draft(0x1416fd770, "kaim_1416fd770", fn_1416fd770 as *const ()), // sub_1416fd770
    Replacement::draft(0x1416fd9a0, "kaim_1416fd9a0", fn_1416fd9a0 as *const ()), // sub_1416fd9a0
    Replacement::draft(0x1416fdcb0, "kaim_1416fdcb0", fn_1416fdcb0 as *const ()), // sub_1416fdcb0
    Replacement::draft(0x1416fde70, "kaim_1416fde70", fn_1416fde70 as *const ()), // sub_1416fde70
    Replacement::draft(0x1416fdfa0, "kaim_1416fdfa0", fn_1416fdfa0 as *const ()), // sub_1416fdfa0
    Replacement::draft(0x1416fe100, "kaim_1416fe100", fn_1416fe100 as *const ()), // sub_1416fe100
    Replacement::draft(0x1416fe320, "kaim_1416fe320", fn_1416fe320 as *const ()), // sub_1416fe320
    Replacement::draft(0x1416fee70, "kaim_1416fee70", tpl_90980::<0xbc, 0x20, 0x68, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x98, 0x7c, 0x141700580> as *const ()), // sub_1416fee70
    Replacement::draft(0x1416fef40, "kaim_1416fef40", fn_1416fef40 as *const ()), // sub_1416fef40
    Replacement::draft(0x1416ff1f0, "kaim_1416ff1f0", fn_1416ff1f0 as *const ()), // sub_1416ff1f0
    Replacement::draft(0x1416ff2f0, "kaim_1416ff2f0", tpl_90980::<0xbc, 0x20, 0x68, 0xa0, 0x48, 0x4c, 0x58, 0x1416b1580, 0x78, 0x98, 0x7c, 0x141702240> as *const ()), // sub_1416ff2f0
    Replacement::draft(0x1416ff3c0, "kaim_1416ff3c0", fn_1416ff3c0 as *const ()), // sub_1416ff3c0
    Replacement::draft(0x1416ffc30, "kaim_1416ffc30", fn_1416ffc30 as *const ()), // sub_1416ffc30
    Replacement::draft(0x141700180, "kaim_141700180", fn_141700180 as *const ()), // sub_141700180
    Replacement::draft(0x141700580, "kaim_141700580", fn_141700580 as *const ()), // sub_141700580
    Replacement::draft(0x141700f70, "kaim_141700f70", fn_141700f70 as *const ()), // sub_141700f70
    Replacement::draft(0x141701250, "kaim_141701250", fn_141701250 as *const ()), // sub_141701250
    Replacement::draft(0x141701920, "kaim_141701920", tpl_230870::<0x1416b1060, 0x1416a41b0, 0x1416b12a0, 0x1434268d0, 0x1416ae0a0, 0x1416ae7e0, 0x1416ae2d0, 0x140000000, 0x1416ef200, 0x1416ae430, 0x1416ae830, 0x1416dd200, 0x1416aed30, 0x1416a35b0> as *const ()), // sub_141701920
    Replacement::draft(0x141702240, "kaim_141702240", tpl_230870::<0x1416b1060, 0x1416a41b0, 0x1416b12a0, 0x1434268d0, 0x1416ae0a0, 0x1416ae7e0, 0x1416ae2d0, 0x140000000, 0x1416ef5b0, 0x1416ae430, 0x1416ae830, 0x1416dd200, 0x1416aed30, 0x1416a35b0> as *const ()), // sub_141702240
    Replacement::draft(0x141702b60, "kaim_141702b60", tpl_31766::<0x1416b1060, 0x1416a41b0, 0x1416b12a0, 0x143428720, 0x1416b1580, 0x1434268d0, 0x1416ae0a0, 0x1416ae7e0, 0x1416ae230, 0x1411fb750, 0x1416ae2d0, 0x143427080, 0x1416ae430, 0x1416ae830, 0x140000000, 0x141703a30, 0x1416b2820, 0x1416a35b0> as *const ()), // sub_141702b60
    Replacement::draft(0x1417033b0, "kaim_1417033b0", fn_1417033b0 as *const ()), // sub_1417033b0
    Replacement::draft(0x141703590, "kaim_141703590", fn_141703590 as *const ()), // sub_141703590
    Replacement::draft(0x1417035d0, "kaim_1417035d0", fn_1417035d0 as *const ()), // sub_1417035d0
    Replacement::draft(0x141703800, "kaim_141703800", fn_141703800 as *const ()), // sub_141703800
    Replacement::draft(0x141703a30, "kaim_141703a30", fn_141703a30 as *const ()), // sub_141703a30
    Replacement::draft(0x141703e60, "kaim_141703e60", tpl_22575::<0x20, 0xa0, 0x1a8, 0x141705920> as *const ()), // sub_141703e60
    Replacement::draft(0x141703e80, "kaim_141703e80", fn_141703e80 as *const ()), // sub_141703e80
    Replacement::draft(0x141704040, "kaim_141704040", tpl_32166::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x1416a01d0, 0x14183acc2> as *const ()), // sub_141704040
    Replacement::draft(0x141704100, "kaim_141704100", fn_141704100 as *const ()), // sub_141704100
    Replacement::draft(0x141704380, "kaim_141704380", fn_141704380 as *const ()), // sub_141704380
    Replacement::draft(0x141704530, "kaim_141704530", tpl_32169::<0x1412172f0, 0x141706d90, 0x1416a7100, 0x14183acbc, 0x143932190, 0x1416f66e0> as *const ()), // sub_141704530
    Replacement::draft(0x1417046f0, "kaim_1417046f0", tpl_32169::<0x1412172f0, 0x141707330, 0x1416a7100, 0x14183acbc, 0x143932190, 0x1416f6890> as *const ()), // sub_1417046f0
    Replacement::draft(0x1417048b0, "kaim_1417048b0", tpl_32169::<0x1412172f0, 0x1417078b0, 0x1416a7100, 0x14183acbc, 0x143932190, 0x1416f6a40> as *const ()), // sub_1417048b0
    Replacement::draft(0x141704a70, "kaim_141704a70", fn_141704a70 as *const ()), // sub_141704a70
    Replacement::draft(0x141704d60, "kaim_141704d60", tpl_32171::<0x141708280, 0x1416a7100, 0x14183acbc, 0x143932190, 0x1416b0e60, 0x1411feed0> as *const ()), // sub_141704d60
    Replacement::draft(0x141705050, "kaim_141705050", tpl_32171::<0x1417086a0, 0x1416a7100, 0x14183acbc, 0x143932190, 0x1416b0e60, 0x1411feed0> as *const ()), // sub_141705050
    Replacement::draft(0x141705340, "kaim_141705340", tpl_32171::<0x141708ad0, 0x1416a7100, 0x14183acbc, 0x143932190, 0x1416b0e60, 0x1411feed0> as *const ()), // sub_141705340
    Replacement::draft(0x141705630, "kaim_141705630", tpl_32171::<0x141708e70, 0x1416a7100, 0x14183acbc, 0x143932190, 0x1416b0e60, 0x1411feed0> as *const ()), // sub_141705630
    Replacement::draft(0x141705920, "kaim_141705920", fn_141705920 as *const ()), // sub_141705920
    Replacement::draft(0x141705a30, "kaim_141705a30", fn_141705a30 as *const ()), // sub_141705a30
    Replacement::draft(0x141706010, "kaim_141706010", fn_141706010 as *const ()), // sub_141706010
    Replacement::draft(0x141706190, "kaim_141706190", tpl_22478::<0x1416fe100, 0x1417033b0, 0x1416b4e30> as *const ()), // sub_141706190
    Replacement::draft(0x1417061f0, "kaim_1417061f0", fn_1417061f0 as *const ()), // sub_1417061f0
    Replacement::draft(0x141706260, "kaim_141706260", fn_141706260 as *const ()), // sub_141706260
    Replacement::draft(0x1417063b0, "kaim_1417063b0", fn_1417063b0 as *const ()), // sub_1417063b0
    Replacement::draft(0x141706540, "kaim_141706540", fn_141706540 as *const ()), // sub_141706540
    Replacement::draft(0x141706820, "kaim_141706820", fn_141706820 as *const ()), // sub_141706820
    Replacement::draft(0x141706a40, "kaim_141706a40", fn_141706a40 as *const ()), // sub_141706a40
    Replacement::draft(0x141706d90, "kaim_141706d90", fn_141706d90 as *const ()), // sub_141706d90
    Replacement::draft(0x141707330, "kaim_141707330", fn_141707330 as *const ()), // sub_141707330
    Replacement::draft(0x1417078b0, "kaim_1417078b0", fn_1417078b0 as *const ()), // sub_1417078b0
    Replacement::draft(0x141707e30, "kaim_141707e30", fn_141707e30 as *const ()), // sub_141707e30
    Replacement::draft(0x141708280, "kaim_141708280", fn_141708280 as *const ()), // sub_141708280
    Replacement::draft(0x1417086a0, "kaim_1417086a0", fn_1417086a0 as *const ()), // sub_1417086a0
    Replacement::draft(0x141708ad0, "kaim_141708ad0", fn_141708ad0 as *const ()), // sub_141708ad0
    Replacement::draft(0x141708e70, "kaim_141708e70", fn_141708e70 as *const ()), // sub_141708e70
    Replacement::draft(0x1417092c0, "kaim_1417092c0", fn_1417092c0 as *const ()), // sub_1417092c0
    Replacement::draft(0x141709b60, "kaim_141709b60", fn_141709b60 as *const ()), // sub_141709b60
    Replacement::draft(0x141709df0, "kaim_141709df0", fn_141709df0 as *const ()), // sub_141709df0
    Replacement::draft(0x141709ef0, "kaim_141709ef0", fn_141709ef0 as *const ()), // sub_141709ef0
    Replacement::draft(0x14170a310, "kaim_14170a310", fn_14170a310 as *const ()), // sub_14170a310
    Replacement::draft(0x14170a680, "kaim_14170a680", fn_14170a680 as *const ()), // sub_14170a680
    Replacement::draft(0x14170a6d0, "kaim_14170a6d0", fn_14170a6d0 as *const ()), // sub_14170a6d0
    Replacement::draft(0x14170a8a0, "kaim_14170a8a0", fn_14170a8a0 as *const ()), // sub_14170a8a0
    Replacement::draft(0x14170a8e0, "kaim_14170a8e0", fn_14170a8e0 as *const ()), // sub_14170a8e0
    Replacement::draft(0x14170a990, "kaim_14170a990", fn_14170a990 as *const ()), // sub_14170a990
    Replacement::draft(0x14170ac30, "kaim_14170ac30", fn_14170ac30 as *const ()), // sub_14170ac30
    Replacement::draft(0x14170adb0, "kaim_14170adb0", fn_14170adb0 as *const ()), // sub_14170adb0
    Replacement::draft(0x14170af00, "kaim_14170af00", fn_14170af00 as *const ()), // sub_14170af00
    Replacement::draft(0x14170b020, "kaim_14170b020", fn_14170b020 as *const ()), // sub_14170b020
    Replacement::draft(0x14170b0d0, "kaim_14170b0d0", fn_14170b0d0 as *const ()), // sub_14170b0d0
    Replacement::draft(0x14170b160, "kaim_14170b160", fn_14170b160 as *const ()), // sub_14170b160
    Replacement::draft(0x14170b220, "kaim_14170b220", fn_14170b220 as *const ()), // sub_14170b220
    Replacement::draft(0x14170b2d0, "kaim_14170b2d0", tpl_1674::<0x1426ab808> as *const ()), // sub_14170b2d0
    Replacement::draft(0x14170b2e0, "kaim_14170b2e0", fn_14170b2e0 as *const ()), // sub_14170b2e0
    Replacement::draft(0x14170b330, "kaim_14170b330", tpl_27062::<0x1433e2298, 0x141834d30> as *const ()), // sub_14170b330
    Replacement::draft(0x14170b360, "unknown_14170b360", tpl_31205::<0x141734b70> as *const ()), // sub_14170b360
    Replacement::draft_throws(0x14170b370, "unknown_14170b370", fn_14170b370 as *const (), Throws::Discard), // sub_14170b370
    Replacement::draft(0x14170b3f0, "unknown_14170b3f0", fn_14170b3f0 as *const ()), // sub_14170b3f0
    Replacement::draft(0x14170b640, "unknown_14170b640", fn_14170b640 as *const ()), // sub_14170b640
    Replacement::draft(0x14170b660, "ak_14170b660", fn_14170b660 as *const ()), // sub_14170b660
    Replacement::draft(0x14170b6e0, "unknown_14170b6e0", fn_14170b6e0 as *const ()), // sub_14170b6e0
    Replacement::draft(0x14170b700, "unknown_14170b700", fn_14170b700 as *const ()), // sub_14170b700
    Replacement::draft(0x14170b740, "unknown_14170b740", fn_14170b740 as *const ()), // sub_14170b740
    Replacement::draft(0x14170b940, "unknown_14170b940", fn_14170b940 as *const ()), // sub_14170b940
    Replacement::draft(0x14170ba80, "unknown_14170ba80", fn_14170ba80 as *const ()), // sub_14170ba80
    Replacement::draft(0x14170bbb0, "unknown_14170bbb0", fn_14170bbb0 as *const ()), // sub_14170bbb0
    Replacement::draft(0x14170bbd0, "kaim_14170bbd0", fn_14170bbd0 as *const ()), // sub_14170bbd0
    Replacement::draft(0x14170bc10, "kaim_14170bc10", fn_14170bc10 as *const ()), // sub_14170bc10
    Replacement::draft(0x14170bca0, "kaim_14170bca0", fn_14170bca0 as *const ()), // sub_14170bca0
    Replacement::draft(0x14170bf30, "kaim_14170bf30", fn_14170bf30 as *const ()), // sub_14170bf30
    Replacement::draft(0x14170c050, "kaim_14170c050", fn_14170c050 as *const ()), // sub_14170c050
    Replacement::draft(0x14170c590, "kaim_14170c590", fn_14170c590 as *const ()), // sub_14170c590
    Replacement::draft(0x14170c600, "kaim_14170c600", fn_14170c600 as *const ()), // sub_14170c600
    Replacement::draft(0x14170d5d0, "kaim_14170d5d0", tpl_22671::<0x30, 0x34, 0x8, 0x1417151a0, 0x14170de90> as *const ()), // sub_14170d5d0
    Replacement::draft(0x14170d610, "kaim_14170d610", fn_14170d610 as *const ()), // sub_14170d610
    Replacement::draft(0x14170d680, "kaim_14170d680", tpl_32214::<0x14169eda0, 0x10, 0x14169edb0> as *const ()), // sub_14170d680
    Replacement::draft(0x14170d6c0, "kaim_14170d6c0", tpl_32214::<0x14169eda0, 0x18, 0x14169edb0> as *const ()), // sub_14170d6c0
    Replacement::draft(0x14170d700, "kaim_14170d700", fn_14170d700 as *const ()), // sub_14170d700
    Replacement::draft(0x14170d9b0, "kaim_14170d9b0", fn_14170d9b0 as *const ()), // sub_14170d9b0
    Replacement::draft(0x14170db10, "kaim_14170db10", fn_14170db10 as *const ()), // sub_14170db10
    Replacement::draft(0x14170db50, "kaim_14170db50", fn_14170db50 as *const ()), // sub_14170db50
    Replacement::draft(0x14170db80, "kaim_14170db80", fn_14170db80 as *const ()), // Kaim::PositionOnPathCheckPointValidator::Kaim::PositionOnPathCheckPointValidator::vfunc_1
    Replacement::draft(0x14170dba0, "kaim_14170dba0", fn_14170dba0 as *const ()), // sub_14170dba0
    Replacement::draft(0x14170dbd0, "kaim_14170dbd0", fn_14170dbd0 as *const ()), // sub_14170dbd0
    Replacement::draft(0x14170dc10, "kaim_14170dc10", fn_14170dc10 as *const ()), // sub_14170dc10
    Replacement::draft(0x14170dc40, "kaim_14170dc40", fn_14170dc40 as *const ()), // sub_14170dc40
    Replacement::draft(0x14170dd90, "kaim_14170dd90", fn_14170dd90 as *const ()), // sub_14170dd90
    Replacement::draft(0x14170de90, "kaim_14170de90", fn_14170de90 as *const ()), // sub_14170de90
    Replacement::draft(0x14170dfa0, "kaim_14170dfa0", fn_14170dfa0 as *const ()), // sub_14170dfa0
    Replacement::draft(0x14170e010, "kaim_14170e010", fn_14170e010 as *const ()), // sub_14170e010
    Replacement::draft(0x14170e050, "kaim_14170e050", fn_14170e050 as *const ()), // sub_14170e050
    Replacement::draft(0x14170e0b0, "kaim_14170e0b0", fn_14170e0b0 as *const ()), // sub_14170e0b0
    Replacement::draft(0x14170e160, "kaim_14170e160", fn_14170e160 as *const ()), // sub_14170e160
    Replacement::draft(0x14170e200, "kaim_14170e200", fn_14170e200 as *const ()), // sub_14170e200
    Replacement::draft(0x14170e250, "kaim_14170e250", fn_14170e250 as *const ()), // sub_14170e250
    Replacement::draft(0x14170e300, "kaim_14170e300", fn_14170e300 as *const ()), // sub_14170e300
    Replacement::draft(0x14170e370, "kaim_14170e370", fn_14170e370 as *const ()), // sub_14170e370
    Replacement::draft(0x14170e490, "kaim_14170e490", fn_14170e490 as *const ()), // sub_14170e490
    Replacement::draft(0x14170e580, "kaim_14170e580", fn_14170e580 as *const ()), // sub_14170e580
    Replacement::draft(0x14170e670, "kaim_14170e670", fn_14170e670 as *const ()), // sub_14170e670
    Replacement::draft(0x14170e830, "kaim_14170e830", tpl_424::<0x14169edb0> as *const ()), // sub_14170e830
    Replacement::draft(0x14170e850, "kaim_14170e850", fn_14170e850 as *const ()), // sub_14170e850
    Replacement::draft(0x14170e9f0, "kaim_14170e9f0", tpl_1742::<0x1416bb650> as *const ()), // sub_14170e9f0
    Replacement::draft(0x14170ea00, "kaim_14170ea00", fn_14170ea00 as *const ()), // sub_14170ea00
    Replacement::draft(0x14170ea80, "kaim_14170ea80", tpl_424::<0x14169edd0> as *const ()), // sub_14170ea80
    Replacement::draft(0x14170eaa0, "kaim_14170eaa0", fn_14170eaa0 as *const ()), // sub_14170eaa0
    Replacement::draft(0x14170ead0, "kaim_14170ead0", fn_14170ead0 as *const ()), // sub_14170ead0
    Replacement::draft(0x14170ecd0, "kaim_14170ecd0", fn_14170ecd0 as *const ()), // sub_14170ecd0
    Replacement::draft(0x14170ed30, "kaim_14170ed30", fn_14170ed30 as *const ()), // sub_14170ed30
    Replacement::draft(0x14170ed80, "kaim_14170ed80", fn_14170ed80 as *const ()), // sub_14170ed80
    Replacement::draft(0x14170ee20, "kaim_14170ee20", fn_14170ee20 as *const ()), // sub_14170ee20
    Replacement::draft(0x14170eec0, "kaim_14170eec0", tpl_22435::<0x14170f150> as *const ()), // sub_14170eec0
    Replacement::draft(0x14170eed0, "kaim_14170eed0", fn_14170eed0 as *const ()), // sub_14170eed0
    Replacement::draft(0x14170efc0, "kaim_14170efc0", fn_14170efc0 as *const ()), // sub_14170efc0
    Replacement::draft(0x14170f0e0, "kaim_14170f0e0", fn_14170f0e0 as *const ()), // sub_14170f0e0
    Replacement::draft(0x14170f150, "kaim_14170f150", fn_14170f150 as *const ()), // sub_14170f150
    Replacement::draft(0x14170f360, "kaim_14170f360", fn_14170f360 as *const ()), // sub_14170f360
    Replacement::draft(0x14170f4f0, "kaim_14170f4f0", fn_14170f4f0 as *const ()), // sub_14170f4f0
    Replacement::draft(0x14170f590, "kaim_14170f590", fn_14170f590 as *const ()), // sub_14170f590
    Replacement::draft(0x14170f7b0, "kaim_14170f7b0", fn_14170f7b0 as *const ()), // sub_14170f7b0
    Replacement::draft(0x14170f800, "kaim_14170f800", fn_14170f800 as *const ()), // sub_14170f800
    Replacement::draft(0x14170f830, "kaim_14170f830", tpl_1::<0x1426aa8c8> as *const ()), // sub_14170f830
    Replacement::draft(0x14170f840, "kaim_14170f840", fn_14170f840 as *const ()), // sub_14170f840
    Replacement::draft(0x14170f960, "kaim_14170f960", fn_14170f960 as *const ()), // sub_14170f960
    Replacement::draft(0x14170fda0, "kaim_14170fda0", fn_14170fda0 as *const ()), // sub_14170fda0
    Replacement::draft(0x1417100e0, "kaim_1417100e0", fn_1417100e0 as *const ()), // sub_1417100e0
    Replacement::draft(0x141710240, "kaim_141710240", fn_141710240 as *const ()), // sub_141710240
    Replacement::draft(0x1417103e0, "kaim_1417103e0", fn_1417103e0 as *const ()), // sub_1417103e0
    Replacement::draft(0x1417104b0, "kaim_1417104b0", fn_1417104b0 as *const ()), // sub_1417104b0
    Replacement::draft(0x141710670, "kaim_141710670", fn_141710670 as *const ()), // sub_141710670
    Replacement::draft(0x141710740, "kaim_141710740", fn_141710740 as *const ()), // sub_141710740
    Replacement::draft(0x141710760, "kaim_141710760", fn_141710760 as *const ()), // sub_141710760
    Replacement::draft(0x141710780, "kaim_141710780", fn_141710780 as *const ()), // sub_141710780
    Replacement::draft(0x1417107c0, "kaim_1417107c0", fn_1417107c0 as *const ()), // sub_1417107c0
    Replacement::draft(0x141710840, "kaim_141710840", fn_141710840 as *const ()), // sub_141710840
    Replacement::draft(0x141710860, "kaim_141710860", fn_141710860 as *const ()), // sub_141710860
    Replacement::draft(0x141710b50, "kaim_141710b50", tpl_32264::<0x141710860, 0x143932190, 0x1416a0960, 0x14169edb0, 0x1416a0ed0, 0x1416a20d0, 0x1416a0c40, 0x14169edd0, 0x1416a1bf0, 0x1416a19e0> as *const ()), // sub_141710b50
    Replacement::draft(0x141710cc0, "kaim_141710cc0", fn_141710cc0 as *const ()), // sub_141710cc0
    Replacement::draft(0x141710d90, "kaim_141710d90", fn_141710d90 as *const ()), // sub_141710d90
    Replacement::draft(0x141710e30, "kaim_141710e30", fn_141710e30 as *const ()), // sub_141710e30
    Replacement::draft(0x141710ed0, "kaim_141710ed0", fn_141710ed0 as *const ()), // sub_141710ed0
    Replacement::draft(0x141710fd0, "kaim_141710fd0", fn_141710fd0 as *const ()), // sub_141710fd0
    Replacement::draft(0x1417110a0, "kaim_1417110a0", fn_1417110a0 as *const ()), // sub_1417110a0
    Replacement::draft(0x141711140, "kaim_141711140", fn_141711140 as *const ()), // sub_141711140
    Replacement::draft(0x141711c30, "kaim_141711c30", fn_141711c30 as *const ()), // sub_141711c30
    Replacement::draft(0x141711d00, "kaim_141711d00", fn_141711d00 as *const ()), // sub_141711d00
    Replacement::draft(0x141711db0, "kaim_141711db0", fn_141711db0 as *const ()), // sub_141711db0
    Replacement::draft(0x141711e60, "kaim_141711e60", fn_141711e60 as *const ()), // sub_141711e60
    Replacement::draft(0x141711ef0, "kaim_141711ef0", fn_141711ef0 as *const ()), // sub_141711ef0
    Replacement::draft(0x141711fa0, "kaim_141711fa0", fn_141711fa0 as *const ()), // sub_141711fa0
    Replacement::draft(0x1417120c0, "kaim_1417120c0", fn_1417120c0 as *const ()), // sub_1417120c0
    Replacement::draft(0x141712170, "kaim_141712170", fn_141712170 as *const ()), // sub_141712170
    Replacement::draft(0x141712460, "kaim_141712460", fn_141712460 as *const ()), // sub_141712460
    Replacement::draft(0x141712530, "kaim_141712530", fn_141712530 as *const ()), // sub_141712530
    Replacement::draft(0x141712570, "kaim_141712570", fn_141712570 as *const ()), // sub_141712570
    Replacement::draft(0x141712600, "kaim_141712600", fn_141712600 as *const ()), // sub_141712600
    Replacement::draft(0x141712690, "kaim_141712690", fn_141712690 as *const ()), // sub_141712690
    Replacement::draft(0x1417126a0, "kaim_1417126a0", tpl_27062::<0x1426aa730, 0x141834d30> as *const ()), // sub_1417126a0
    Replacement::draft(0x1417126d0, "kaim_1417126d0", fn_1417126d0 as *const ()), // sub_1417126d0
    Replacement::draft(0x141712700, "kaim_141712700", fn_141712700 as *const ()), // sub_141712700
    Replacement::draft(0x141712750, "kaim_141712750", fn_141712750 as *const ()), // sub_141712750
    Replacement::draft(0x1417127e0, "kaim_1417127e0", fn_1417127e0 as *const ()), // sub_1417127e0
    Replacement::draft(0x141712890, "kaim_141712890", fn_141712890 as *const ()), // sub_141712890
    Replacement::draft(0x141712a80, "kaim_141712a80", fn_141712a80 as *const ()), // sub_141712a80
    Replacement::draft(0x141712b40, "kaim_141712b40", fn_141712b40 as *const ()), // sub_141712b40
    Replacement::draft(0x141712c00, "kaim_141712c00", fn_141712c00 as *const ()), // sub_141712c00
    Replacement::draft(0x141712cd0, "kaim_141712cd0", fn_141712cd0 as *const ()), // sub_141712cd0
    Replacement::draft(0x141712d40, "kaim_141712d40", fn_141712d40 as *const ()), // sub_141712d40
    Replacement::draft(0x141712d70, "kaim_141712d70", fn_141712d70 as *const ()), // sub_141712d70
    Replacement::draft(0x141712e10, "kaim_141712e10", fn_141712e10 as *const ()), // sub_141712e10
    Replacement::draft(0x141712e70, "kaim_141712e70", fn_141712e70 as *const ()), // sub_141712e70
    Replacement::draft(0x1417132e0, "kaim_1417132e0", fn_1417132e0 as *const ()), // sub_1417132e0
    Replacement::draft(0x1417134e0, "kaim_1417134e0", fn_1417134e0 as *const ()), // sub_1417134e0
    Replacement::draft(0x1417139d0, "kaim_1417139d0", fn_1417139d0 as *const ()), // sub_1417139d0
    Replacement::draft(0x141713a00, "kaim_141713a00", tpl_32297::<0x10> as *const ()), // sub_141713a00
    Replacement::draft(0x141713a10, "kaim_141713a10", fn_141713a10 as *const ()), // sub_141713a10
    Replacement::draft(0x141713b40, "kaim_141713b40", fn_141713b40 as *const ()), // sub_141713b40
    Replacement::draft(0x141713c70, "kaim_141713c70", tpl_22671::<0x30, 0x34, 0x8, 0x141714e10, 0x14170e370> as *const ()), // sub_141713c70
    Replacement::draft(0x141713cb0, "kaim_141713cb0", fn_141713cb0 as *const ()), // sub_141713cb0
    Replacement::draft(0x141713cd0, "kaim_141713cd0", fn_141713cd0 as *const ()), // sub_141713cd0
    Replacement::draft(0x141713d00, "kaim_141713d00", fn_141713d00 as *const ()), // sub_141713d00
    Replacement::draft(0x141713ec0, "kaim_141713ec0", fn_141713ec0 as *const ()), // sub_141713ec0
    Replacement::draft(0x141714150, "kaim_141714150", fn_141714150 as *const ()), // sub_141714150
    Replacement::draft(0x1417141d0, "kaim_1417141d0", fn_1417141d0 as *const ()), // sub_1417141d0
    Replacement::draft(0x141714470, "kaim_141714470", fn_141714470 as *const ()), // sub_141714470
    Replacement::draft(0x141714600, "kaim_141714600", fn_141714600 as *const ()), // sub_141714600
    Replacement::draft(0x141714740, "kaim_141714740", fn_141714740 as *const ()), // sub_141714740
    Replacement::draft(0x141714870, "kaim_141714870", fn_141714870 as *const ()), // sub_141714870
    Replacement::draft(0x1417149c0, "kaim_1417149c0", fn_1417149c0 as *const ()), // sub_1417149c0
    Replacement::draft(0x141714b00, "kaim_141714b00", fn_141714b00 as *const ()), // sub_141714b00
    Replacement::draft(0x141714ba0, "kaim_141714ba0", fn_141714ba0 as *const ()), // sub_141714ba0
    Replacement::draft(0x141714c30, "kaim_141714c30", fn_141714c30 as *const ()), // sub_141714c30
    Replacement::draft(0x141714e10, "kaim_141714e10", fn_141714e10 as *const ()), // sub_141714e10
    Replacement::draft(0x141714ec0, "kaim_141714ec0", fn_141714ec0 as *const ()), // sub_141714ec0
    Replacement::draft(0x1417151a0, "kaim_1417151a0", fn_1417151a0 as *const ()), // sub_1417151a0
    Replacement::draft(0x141715270, "kaim_141715270", fn_141715270 as *const ()), // sub_141715270
    Replacement::draft(0x141715300, "kaim_141715300", fn_141715300 as *const ()), // sub_141715300
    Replacement::draft(0x1417154f0, "kaim_1417154f0", fn_1417154f0 as *const ()), // sub_1417154f0
    Replacement::draft(0x141715570, "kaim_141715570", tpl_22471::<0x1426ab9b0, 0xc0, 0x74, 0xffffffffffffffff, 0x1416a33d0, 0x143932190, 0x60, 0xa0, 0x1433e1728, 0x18, 0x14169edb0, 0x14169ed80> as *const ()), // sub_141715570
    Replacement::draft(0x141715610, "kaim_141715610", tpl_22426::<0x1426ab998, 0x14169ed80> as *const ()), // sub_141715610
    Replacement::draft(0x141715620, "kaim_141715620", fn_141715620 as *const ()), // sub_141715620
    Replacement::draft(0x141715690, "kaim_141715690", fn_141715690 as *const ()), // sub_141715690
    Replacement::draft(0x141715730, "kaim_141715730", tpl_22540::<0x141715570, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_141715730
    Replacement::draft(0x141715790, "kaim_141715790", tpl_22439::<0x1426ab998, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_141715790
    Replacement::draft(0x141715800, "kaim_141715800", tpl_32111::<0xb8, 0x20, 0x68, 0x10, 0xa0, 0x90, 0x80, 0x94, 0x1416b1580, 0x141716390> as *const ()), // sub_141715800
    Replacement::draft(0x1417158c0, "kaim_1417158c0", fn_1417158c0 as *const ()), // sub_1417158c0
    Replacement::draft(0x141715c00, "kaim_141715c00", fn_141715c00 as *const ()), // sub_141715c00
    Replacement::draft(0x141715ec0, "kaim_141715ec0", fn_141715ec0 as *const ()), // sub_141715ec0
    Replacement::draft(0x141716020, "kaim_141716020", fn_141716020 as *const ()), // sub_141716020
    Replacement::draft(0x141716040, "kaim_141716040", fn_141716040 as *const ()), // sub_141716040
    Replacement::draft(0x1417160e0, "kaim_1417160e0", fn_1417160e0 as *const ()), // sub_1417160e0
    Replacement::draft(0x141716320, "kaim_141716320", fn_141716320 as *const ()), // sub_141716320
    Replacement::draft(0x141716390, "kaim_141716390", fn_141716390 as *const ()), // sub_141716390
    Replacement::draft(0x141716660, "kaim_141716660", fn_141716660 as *const ()), // sub_141716660
    Replacement::draft(0x1417167f0, "kaim_1417167f0", fn_1417167f0 as *const ()), // sub_1417167f0
    Replacement::draft(0x141716ac0, "kaim_141716ac0", fn_141716ac0 as *const ()), // sub_141716ac0
    Replacement::draft(0x141716b00, "kaim_141716b00", fn_141716b00 as *const ()), // sub_141716b00
    Replacement::draft(0x141716b10, "kaim_141716b10", fn_141716b10 as *const ()), // sub_141716b10
    Replacement::draft(0x141716b50, "kaim_141716b50", fn_141716b50 as *const ()), // sub_141716b50
    Replacement::draft(0x141716bf0, "kaim_141716bf0", fn_141716bf0 as *const ()), // sub_141716bf0
    Replacement::draft(0x141716e90, "kaim_141716e90", tpl_32332::<0x28, 0x141712d70, 0x30, 0x1417412b0, 0x10, 0x238, 0x240, 0x38, 0x22c, 0x244, 0x250, 0x24c> as *const ()), // sub_141716e90
    Replacement::draft(0x141716f10, "kaim_141716f10", fn_141716f10 as *const ()), // sub_141716f10
    Replacement::draft(0x141716f70, "kaim_141716f70", tpl_32332::<0x28, 0x141712e10, 0x30, 0x1417413e0, 0x10, 0x238, 0x240, 0x38, 0x22c, 0x244, 0x250, 0x24c> as *const ()), // sub_141716f70
    Replacement::draft(0x141716ff0, "kaim_141716ff0", fn_141716ff0 as *const ()), // sub_141716ff0
    Replacement::draft(0x141717080, "kaim_141717080", fn_141717080 as *const ()), // sub_141717080
    Replacement::draft(0x1417171c0, "kaim_1417171c0", fn_1417171c0 as *const ()), // sub_1417171c0
    Replacement::draft(0x141717330, "kaim_141717330", fn_141717330 as *const ()), // sub_141717330
    Replacement::draft(0x141717350, "kaim_141717350", fn_141717350 as *const ()), // sub_141717350
    Replacement::draft(0x141717480, "kaim_141717480", fn_141717480 as *const ()), // sub_141717480
    Replacement::draft(0x1417175d0, "kaim_1417175d0", fn_1417175d0 as *const ()), // sub_1417175d0
    Replacement::draft(0x141717680, "kaim_141717680", fn_141717680 as *const ()), // sub_141717680
    Replacement::draft(0x141717700, "kaim_141717700", fn_141717700 as *const ()), // sub_141717700
    Replacement::draft(0x141717840, "kaim_141717840", fn_141717840 as *const ()), // sub_141717840
    Replacement::draft(0x141717890, "kaim_141717890", fn_141717890 as *const ()), // sub_141717890
    Replacement::draft(0x1417179a0, "kaim_1417179a0", fn_1417179a0 as *const ()), // sub_1417179a0
    Replacement::draft(0x141717a50, "kaim_141717a50", tpl_22437::<0x10, 0x8, 0x1417179a0> as *const ()), // sub_141717a50
    Replacement::draft(0x141717ad0, "kaim_141717ad0", fn_141717ad0 as *const ()), // sub_141717ad0
    Replacement::draft(0x141717ae0, "kaim_141717ae0", fn_141717ae0 as *const ()), // sub_141717ae0
    Replacement::draft(0x141717b90, "kaim_141717b90", fn_141717b90 as *const ()), // sub_141717b90
    Replacement::draft(0x141717c70, "kaim_141717c70", fn_141717c70 as *const ()), // sub_141717c70
    Replacement::draft(0x141717d00, "kaim_141717d00", fn_141717d00 as *const ()), // sub_141717d00
    Replacement::draft(0x141717d70, "kaim_141717d70", tpl_1::<0x1426aa8d0> as *const ()), // sub_141717d70
    Replacement::draft(0x141717d80, "kaim_141717d80", fn_141717d80 as *const ()), // sub_141717d80
    Replacement::draft(0x141717f80, "kaim_141717f80", fn_141717f80 as *const ()), // sub_141717f80
    Replacement::draft(0x1417180c0, "kaim_1417180c0", fn_1417180c0 as *const ()), // sub_1417180c0
    Replacement::draft(0x141718170, "kaim_141718170", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x1417180c0> as *const ()), // sub_141718170
    Replacement::draft(0x141718210, "kaim_141718210", fn_141718210 as *const ()), // sub_141718210
    Replacement::draft(0x1417184e0, "kaim_1417184e0", tpl_32264::<0x141718210, 0x143932190, 0x1416a0960, 0x14169edb0, 0x1416a0ed0, 0x1416a21e0, 0x1416a0c40, 0x14169edd0, 0x1416a1bf0, 0x1416a19e0> as *const ()), // sub_1417184e0
    Replacement::draft(0x141718650, "kaim_141718650", fn_141718650 as *const ()), // sub_141718650
    Replacement::draft(0x1417186d0, "kaim_1417186d0", fn_1417186d0 as *const ()), // sub_1417186d0
    Replacement::draft(0x1417187f0, "kaim_1417187f0", fn_1417187f0 as *const ()), // sub_1417187f0
    Replacement::draft(0x1417188d0, "kaim_1417188d0", fn_1417188d0 as *const ()), // sub_1417188d0
    Replacement::draft(0x141718970, "kaim_141718970", fn_141718970 as *const ()), // sub_141718970
    Replacement::draft(0x141718b30, "kaim_141718b30", fn_141718b30 as *const ()), // sub_141718b30
    Replacement::draft(0x141718dd0, "kaim_141718dd0", fn_141718dd0 as *const ()), // sub_141718dd0
    Replacement::draft(0x141718fe0, "kaim_141718fe0", fn_141718fe0 as *const ()), // sub_141718fe0
    Replacement::draft(0x1417192c0, "kaim_1417192c0", tpl_1::<0x1426aa8e8> as *const ()), // sub_1417192c0
    Replacement::draft(0x1417192d0, "kaim_1417192d0", fn_1417192d0 as *const ()), // sub_1417192d0
    Replacement::draft(0x1417195d0, "kaim_1417195d0", fn_1417195d0 as *const ()), // sub_1417195d0
    Replacement::draft(0x141719630, "kaim_141719630", fn_141719630 as *const ()), // sub_141719630
    Replacement::draft(0x1417197a0, "kaim_1417197a0", fn_1417197a0 as *const ()), // sub_1417197a0
    Replacement::draft(0x141719850, "kaim_141719850", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x1417197a0> as *const ()), // sub_141719850
    Replacement::draft(0x1417198f0, "kaim_1417198f0", fn_1417198f0 as *const ()), // sub_1417198f0
    Replacement::draft(0x141719ae0, "kaim_141719ae0", fn_141719ae0 as *const ()), // sub_141719ae0
    Replacement::draft(0x141719c90, "kaim_141719c90", fn_141719c90 as *const ()), // sub_141719c90
    Replacement::draft(0x141719e10, "kaim_141719e10", fn_141719e10 as *const ()), // sub_141719e10
    Replacement::draft(0x141719ec0, "kaim_141719ec0", fn_141719ec0 as *const ()), // sub_141719ec0
    Replacement::draft(0x141719f40, "kaim_141719f40", fn_141719f40 as *const ()), // sub_141719f40
    Replacement::draft(0x14171a010, "kaim_14171a010", tpl_22439::<0x1426abaa8, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_14171a010
    Replacement::draft(0x14171a080, "kaim_14171a080", fn_14171a080 as *const ()), // sub_14171a080
    Replacement::draft(0x14171a100, "unknown_14171a100", fn_14171a100 as *const ()), // sub_14171a100
    Replacement::draft(0x14171a1d0, "unknown_14171a1d0", fn_14171a1d0 as *const ()), // sub_14171a1d0
    Replacement::draft(0x14171a280, "unknown_14171a280", fn_14171a280 as *const ()), // sub_14171a280
    Replacement::draft(0x14171a350, "unknown_14171a350", fn_14171a350 as *const ()), // sub_14171a350
    Replacement::draft(0x14171a440, "unknown_14171a440", fn_14171a440 as *const ()), // sub_14171a440
    Replacement::draft(0x14171a510, "unknown_14171a510", fn_14171a510 as *const ()), // sub_14171a510
    Replacement::draft(0x14171a5e0, "unknown_14171a5e0", fn_14171a5e0 as *const ()), // sub_14171a5e0
    Replacement::draft(0x14171a690, "unknown_14171a690", fn_14171a690 as *const ()), // sub_14171a690
    Replacement::draft(0x14171a730, "unknown_14171a730", fn_14171a730 as *const ()), // sub_14171a730
    Replacement::draft(0x14171a740, "unknown_14171a740", fn_14171a740 as *const ()), // sub_14171a740
    Replacement::draft(0x14171a7a0, "unknown_14171a7a0", fn_14171a7a0 as *const ()), // sub_14171a7a0
    Replacement::draft(0x14171a9c0, "unknown_14171a9c0", fn_14171a9c0 as *const ()), // sub_14171a9c0
    Replacement::draft(0x14171abb0, "unknown_14171abb0", fn_14171abb0 as *const ()), // sub_14171abb0
    Replacement::draft(0x14171ac10, "unknown_14171ac10", fn_14171ac10 as *const ()), // sub_14171ac10
    Replacement::draft(0x14171acf0, "unknown_14171acf0", fn_14171acf0 as *const ()), // sub_14171acf0
    Replacement::draft(0x14171ad50, "unknown_14171ad50", fn_14171ad50 as *const ()), // sub_14171ad50
    Replacement::draft(0x14171ad60, "unknown_14171ad60", fn_14171ad60 as *const ()), // sub_14171ad60
    Replacement::draft(0x14171adf0, "unknown_14171adf0", fn_14171adf0 as *const ()), // sub_14171adf0
    Replacement::draft(0x14171ae90, "unknown_14171ae90", fn_14171ae90 as *const ()), // sub_14171ae90
    Replacement::draft(0x14171af30, "unknown_14171af30", fn_14171af30 as *const ()), // sub_14171af30
    Replacement::draft(0x14171b000, "unknown_14171b000", fn_14171b000 as *const ()), // sub_14171b000
    Replacement::draft(0x14171b0f0, "unknown_14171b0f0", fn_14171b0f0 as *const ()), // sub_14171b0f0
    Replacement::draft(0x14171b1d0, "unknown_14171b1d0", fn_14171b1d0 as *const ()), // sub_14171b1d0
    Replacement::draft(0x14171b2c0, "unknown_14171b2c0", fn_14171b2c0 as *const ()), // sub_14171b2c0
    Replacement::draft(0x14171b300, "unknown_14171b300", fn_14171b300 as *const ()), // sub_14171b300
    Replacement::draft(0x14171b330, "unknown_14171b330", fn_14171b330 as *const ()), // sub_14171b330
    Replacement::draft(0x14171b3b0, "unknown_14171b3b0", fn_14171b3b0 as *const ()), // sub_14171b3b0
    Replacement::draft(0x14171b3f0, "unknown_14171b3f0", fn_14171b3f0 as *const ()), // sub_14171b3f0
    Replacement::draft(0x14171b580, "unknown_14171b580", fn_14171b580 as *const ()), // sub_14171b580
    Replacement::draft(0x14171b650, "unknown_14171b650", fn_14171b650 as *const ()), // sub_14171b650
    Replacement::draft(0x14171b780, "unknown_14171b780", fn_14171b780 as *const ()), // sub_14171b780
    Replacement::draft(0x14171b860, "unknown_14171b860", fn_14171b860 as *const ()), // sub_14171b860
    Replacement::draft(0x14171b8c0, "unknown_14171b8c0", fn_14171b8c0 as *const ()), // sub_14171b8c0
    Replacement::draft(0x14171b910, "unknown_14171b910", fn_14171b910 as *const ()), // sub_14171b910
    Replacement::draft(0x14171b970, "unknown_14171b970", fn_14171b970 as *const ()), // sub_14171b970
    Replacement::draft(0x14171ba00, "unknown_14171ba00", fn_14171ba00 as *const ()), // sub_14171ba00
    Replacement::draft(0x14171bae0, "unknown_14171bae0", fn_14171bae0 as *const ()), // sub_14171bae0
    Replacement::draft(0x14171bca0, "unknown_14171bca0", fn_14171bca0 as *const ()), // sub_14171bca0
    Replacement::draft(0x14171bd80, "unknown_14171bd80", fn_14171bd80 as *const ()), // sub_14171bd80
    Replacement::draft(0x14171be60, "unknown_14171be60", fn_14171be60 as *const ()), // sub_14171be60
    Replacement::draft(0x14171bfb0, "unknown_14171bfb0", fn_14171bfb0 as *const ()), // sub_14171bfb0
    Replacement::draft(0x14171c100, "unknown_14171c100", fn_14171c100 as *const ()), // sub_14171c100
    Replacement::draft(0x14171c1e0, "unknown_14171c1e0", fn_14171c1e0 as *const ()), // sub_14171c1e0
    Replacement::draft(0x14171c270, "unknown_14171c270", fn_14171c270 as *const ()), // sub_14171c270
    Replacement::draft(0x14171c330, "unknown_14171c330", fn_14171c330 as *const ()), // sub_14171c330
    Replacement::draft(0x14171c420, "unknown_14171c420", fn_14171c420 as *const ()), // sub_14171c420
    Replacement::draft(0x14171c590, "unknown_14171c590", fn_14171c590 as *const ()), // sub_14171c590
    Replacement::draft(0x14171c810, "unknown_14171c810", fn_14171c810 as *const ()), // sub_14171c810
    Replacement::draft(0x14171c860, "unknown_14171c860", tpl_32406::<0x24, 0x8, 0x2c, 0x44, 0x38, 0x14171c1e0> as *const ()), // sub_14171c860
    Replacement::draft(0x14171c890, "unknown_14171c890", tpl_32406::<0x18, 0x8, 0x20, 0x40, 0x30, 0x14171c1e0> as *const ()), // sub_14171c890
    Replacement::draft(0x14171c8c0, "unknown_14171c8c0", fn_14171c8c0 as *const ()), // sub_14171c8c0
    Replacement::draft(0x14171c930, "unknown_14171c930", fn_14171c930 as *const ()), // sub_14171c930
    Replacement::draft(0x14171c970, "unknown_14171c970", fn_14171c970 as *const ()), // sub_14171c970
    Replacement::draft(0x14171c9b0, "unknown_14171c9b0", fn_14171c9b0 as *const ()), // sub_14171c9b0
    Replacement::draft(0x14171ca00, "unknown_14171ca00", fn_14171ca00 as *const ()), // sub_14171ca00
    Replacement::draft(0x14171ca50, "unknown_14171ca50", fn_14171ca50 as *const ()), // sub_14171ca50
    Replacement::draft(0x14171caf0, "unknown_14171caf0", fn_14171caf0 as *const ()), // sub_14171caf0
    Replacement::draft(0x14171cb90, "unknown_14171cb90", fn_14171cb90 as *const ()), // sub_14171cb90
    Replacement::draft(0x14171cd60, "unknown_14171cd60", fn_14171cd60 as *const ()), // sub_14171cd60
    Replacement::draft(0x14171cf10, "unknown_14171cf10", fn_14171cf10 as *const ()), // sub_14171cf10
    Replacement::draft(0x14171cf80, "unknown_14171cf80", fn_14171cf80 as *const ()), // sub_14171cf80
    Replacement::draft(0x14171d060, "unknown_14171d060", fn_14171d060 as *const ()), // sub_14171d060
    Replacement::draft(0x14171d130, "unknown_14171d130", fn_14171d130 as *const ()), // sub_14171d130
    Replacement::draft(0x14171d1e0, "unknown_14171d1e0", fn_14171d1e0 as *const ()), // sub_14171d1e0
    Replacement::draft(0x14171d390, "unknown_14171d390", fn_14171d390 as *const ()), // sub_14171d390
    Replacement::draft(0x14171d400, "unknown_14171d400", fn_14171d400 as *const ()), // sub_14171d400
    Replacement::draft(0x14171d480, "unknown_14171d480", tpl_32423::<0x98, 0x141833c2d, 0xf0, 0x230, 0x141833c49, 0x1418348a3, 0x14171d4cb> as *const ()), // sub_14171d480
    Replacement::draft(0x14171d4d0, "unknown_14171d4d0", fn_14171d4d0 as *const ()), // sub_14171d4d0
    Replacement::draft(0x14171d550, "unknown_14171d550", tpl_32425::<0x98, 0x141833c2d, 0x230, 0xfffffffffffffff8, 0xa0, 0x10, 0x141833c49, 0x1418348a3, 0x14171d5f4> as *const ()), // sub_14171d550
    Replacement::draft(0x14171d600, "unknown_14171d600", tpl_32425::<0x98, 0x141833c2d, 0x238, 0xfffffffffffffff8, 0xb0, 0x10, 0x141833c49, 0x1418348a3, 0x14171d6a4> as *const ()), // sub_14171d600
    Replacement::draft(0x14171d6b0, "unknown_14171d6b0", fn_14171d6b0 as *const ()), // sub_14171d6b0
    Replacement::draft(0x14171d770, "unknown_14171d770", tpl_32423::<0x98, 0x141833c2d, 0xf0, 0x238, 0x141833c49, 0x1418348a3, 0x14171d7bb> as *const ()), // sub_14171d770
    Replacement::draft(0x14171d7c0, "unknown_14171d7c0", fn_14171d7c0 as *const ()), // sub_14171d7c0
    Replacement::draft(0x14171d990, "unknown_14171d990", fn_14171d990 as *const ()), // sub_14171d990
    Replacement::draft(0x14171daa0, "unknown_14171daa0", fn_14171daa0 as *const ()), // sub_14171daa0
    Replacement::draft(0x14171db20, "unknown_14171db20", fn_14171db20 as *const ()), // sub_14171db20
    Replacement::draft(0x14171db80, "unknown_14171db80", fn_14171db80 as *const ()), // sub_14171db80
    Replacement::draft(0x14171dbf0, "unknown_14171dbf0", fn_14171dbf0 as *const ()), // sub_14171dbf0
    Replacement::draft(0x14171dcb0, "navigation_14171dcb0", fn_14171dcb0 as *const ()), // navigation::KaimSysAllocWrapper::navigation::KaimSysAllocWrapper::vfunc_1
    Replacement::draft(0x14171dcf0, "navigation_14171dcf0", fn_14171dcf0 as *const ()), // sub_14171dcf0
    Replacement::draft(0x14171dda0, "navigation_14171dda0", fn_14171dda0 as *const ()), // navigation::KaimSysAllocWrapper::navigation::KaimSysAllocWrapper::vfunc_2
    Replacement::draft(0x14171de30, "kaim_14171de30", fn_14171de30 as *const ()), // sub_14171de30
    Replacement::draft(0x14171dec0, "kaim_14171dec0", fn_14171dec0 as *const ()), // sub_14171dec0
    Replacement::draft(0x14171df00, "kaim_14171df00", fn_14171df00 as *const ()), // sub_14171df00
    Replacement::draft(0x14171e020, "kaim_14171e020", fn_14171e020 as *const ()), // sub_14171e020
    Replacement::draft(0x14171e120, "kaim_14171e120", fn_14171e120 as *const ()), // sub_14171e120
    Replacement::draft(0x14171e1d0, "kaim_14171e1d0", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x14171e120> as *const ()), // sub_14171e1d0
    Replacement::draft(0x14171e270, "kaim_14171e270", fn_14171e270 as *const ()), // sub_14171e270
    Replacement::draft(0x14171e2d0, "kaim_14171e2d0", tpl_424::<0x141746770> as *const ()), // sub_14171e2d0
    Replacement::draft(0x14171e2f0, "kaim_14171e2f0", tpl_27047::<0x14171e680> as *const ()), // sub_14171e2f0
    Replacement::draft(0x14171e310, "kaim_14171e310", fn_14171e310 as *const ()), // sub_14171e310
    Replacement::draft(0x14171e380, "kaim_14171e380", tpl_28319::<0x14171e3c0> as *const ()), // sub_14171e380
    Replacement::draft(0x14171e390, "kaim_14171e390", tpl_893::<0x1426abc20, 0x141834d30> as *const ()), // sub_14171e390
    Replacement::draft(0x14171e3c0, "kaim_14171e3c0", tpl_22508::<0x14171e310, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_14171e3c0
    Replacement::draft(0x14171e420, "kaim_14171e420", fn_14171e420 as *const ()), // sub_14171e420
    Replacement::draft_throws(0x14171e450, "kaim_14171e450", fn_14171e450 as *const (), Throws::Caught), // sub_14171e450
    Replacement::draft(0x14171e560, "kaim_14171e560", fn_14171e560 as *const ()), // sub_14171e560
    Replacement::draft(0x14171e5a0, "kaim_14171e5a0", fn_14171e5a0 as *const ()), // sub_14171e5a0
    Replacement::draft(0x14171e600, "kaim_14171e600", fn_14171e600 as *const ()), // sub_14171e600
    Replacement::draft(0x14171e610, "kaim_14171e610", fn_14171e610 as *const ()), // sub_14171e610
    Replacement::draft(0x14171e660, "kaim_14171e660", fn_14171e660 as *const ()), // sub_14171e660
    Replacement::draft(0x14171e680, "kaim_14171e680", fn_14171e680 as *const ()), // sub_14171e680
    Replacement::draft_throws(0x14171e710, "kaim_14171e710", fn_14171e710 as *const (), Throws::Caught), // sub_14171e710
    Replacement::draft(0x14171e8f0, "kaim_14171e8f0", fn_14171e8f0 as *const ()), // sub_14171e8f0
    Replacement::draft(0x14171ec30, "kaim_14171ec30", fn_14171ec30 as *const ()), // sub_14171ec30
    Replacement::draft(0x14171ee90, "kaim_14171ee90", fn_14171ee90 as *const ()), // sub_14171ee90
    Replacement::draft(0x14171f090, "kaim_14171f090", fn_14171f090 as *const ()), // sub_14171f090
    Replacement::draft(0x14171f1b0, "kaim_14171f1b0", fn_14171f1b0 as *const ()), // sub_14171f1b0
    Replacement::draft(0x14171f1e0, "kaim_14171f1e0", fn_14171f1e0 as *const ()), // sub_14171f1e0
    Replacement::draft(0x14171f250, "kaim_14171f250", tpl_22439::<0x1426abfd8, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_14171f250
    Replacement::draft(0x14171f2c0, "kaim_14171f2c0", tpl_27067::<0x10, 0x60> as *const ()), // sub_14171f2c0
    Replacement::draft(0x14171f2d0, "kaim_14171f2d0", tpl_27067::<0x10, 0x88> as *const ()), // sub_14171f2d0
    Replacement::draft(0x14171f2e0, "kaim_14171f2e0", tpl_27067::<0x10, 0x98> as *const ()), // sub_14171f2e0
    Replacement::draft(0x14171f2f0, "kaim_14171f2f0", fn_14171f2f0 as *const ()), // sub_14171f2f0
    Replacement::draft(0x14171f380, "kaim_14171f380", tpl_27067::<0x10, 0x90> as *const ()), // sub_14171f380
    Replacement::draft(0x14171f390, "kaim_14171f390", tpl_32457 as *const ()), // sub_14171f390
    Replacement::draft(0x14171f3a0, "kaim_14171f3a0", tpl_27067::<0x10, 0x68> as *const ()), // sub_14171f3a0
    Replacement::draft(0x14171f3b0, "kaim_14171f3b0", tpl_27067::<0x10, 0x40> as *const ()), // sub_14171f3b0
    Replacement::draft(0x14171f3c0, "kaim_14171f3c0", fn_14171f3c0 as *const ()), // sub_14171f3c0
    Replacement::draft(0x14171f3e0, "kaim_14171f3e0", fn_14171f3e0 as *const ()), // sub_14171f3e0
    Replacement::draft(0x14171f3f0, "kaim_14171f3f0", tpl_27067::<0x10, 0x8> as *const ()), // sub_14171f3f0
    Replacement::draft(0x14171f400, "kaim_14171f400", tpl_27067::<0x10, 0x30> as *const ()), // sub_14171f400
    Replacement::draft(0x14171f410, "kaim_14171f410", fn_14171f410 as *const ()), // sub_14171f410
    Replacement::draft(0x14171f440, "kaim_14171f440", tpl_27067::<0x10, 0x18> as *const ()), // sub_14171f440
    Replacement::draft(0x14171f450, "kaim_14171f450", tpl_27067::<0x10, 0x38> as *const ()), // sub_14171f450
    Replacement::draft(0x14171f460, "kaim_14171f460", tpl_27067::<0x10, 0x78> as *const ()), // sub_14171f460
    Replacement::draft(0x14171f470, "kaim_14171f470", tpl_27067::<0x10, 0x28> as *const ()), // sub_14171f470
    Replacement::draft(0x14171f480, "kaim_14171f480", tpl_27067::<0x10, 0x50> as *const ()), // sub_14171f480
    Replacement::draft(0x14171f490, "kaim_14171f490", tpl_27067::<0x10, 0x70> as *const ()), // sub_14171f490
    Replacement::draft(0x14171f4a0, "kaim_14171f4a0", tpl_28396::<0x70> as *const ()), // sub_14171f4a0
    Replacement::draft(0x14171f4b0, "kaim_14171f4b0", tpl_27067::<0x10, 0x58> as *const ()), // sub_14171f4b0
    Replacement::draft(0x14171f4c0, "kaim_14171f4c0", tpl_27067::<0x10, 0x20> as *const ()), // sub_14171f4c0
    Replacement::draft(0x14171f4d0, "kaim_14171f4d0", tpl_27067::<0x10, 0x48> as *const ()), // sub_14171f4d0
    Replacement::draft(0x14171f4e0, "kaim_14171f4e0", fn_14171f4e0 as *const ()), // sub_14171f4e0
    Replacement::draft(0x14171f650, "kaim_14171f650", fn_14171f650 as *const ()), // sub_14171f650
    Replacement::draft(0x14171f6f0, "kaim_14171f6f0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac098> as *const ()), // sub_14171f6f0
    Replacement::draft(0x14171f750, "kaim_14171f750", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac0d0, 0x8> as *const ()), // sub_14171f750
    Replacement::draft(0x14171f790, "kaim_14171f790", fn_14171f790 as *const ()), // sub_14171f790
    Replacement::draft(0x14171f7b0, "kaim_14171f7b0", fn_14171f7b0 as *const ()), // sub_14171f7b0
    Replacement::draft(0x14171f7e0, "kaim_14171f7e0", fn_14171f7e0 as *const ()), // sub_14171f7e0
    Replacement::draft(0x14171f7f0, "kaim_14171f7f0", tpl_1::<0x1426ac088> as *const ()), // sub_14171f7f0
    Replacement::draft(0x14171f800, "kaim_14171f800", fn_14171f800 as *const ()), // sub_14171f800
    Replacement::draft(0x14171fa40, "kaim_14171fa40", fn_14171fa40 as *const ()), // sub_14171fa40
    Replacement::draft(0x14171faf0, "kaim_14171faf0", fn_14171faf0 as *const ()), // sub_14171faf0
    Replacement::draft(0x14171fdb0, "kaim_14171fdb0", fn_14171fdb0 as *const ()), // sub_14171fdb0
    Replacement::draft(0x14171fe00, "kaim_14171fe00", fn_14171fe00 as *const ()), // sub_14171fe00
    Replacement::draft(0x14171fe30, "kaim_14171fe30", fn_14171fe30 as *const ()), // sub_14171fe30
    Replacement::draft(0x14171fe70, "kaim_14171fe70", fn_14171fe70 as *const ()), // sub_14171fe70
    Replacement::draft(0x14171ff70, "kaim_14171ff70", fn_14171ff70 as *const ()), // sub_14171ff70
    Replacement::draft(0x141720180, "kaim_141720180", fn_141720180 as *const ()), // sub_141720180
    Replacement::draft(0x141720280, "kaim_141720280", fn_141720280 as *const ()), // sub_141720280
    Replacement::draft(0x141720300, "kaim_141720300", fn_141720300 as *const ()), // sub_141720300
    Replacement::draft(0x141720770, "kaim_141720770", fn_141720770 as *const ()), // sub_141720770
    Replacement::draft(0x1417209f0, "kaim_1417209f0", fn_1417209f0 as *const ()), // sub_1417209f0
    Replacement::draft(0x141720d40, "kaim_141720d40", fn_141720d40 as *const ()), // sub_141720d40
    Replacement::draft(0x141720ed0, "kaim_141720ed0", fn_141720ed0 as *const ()), // sub_141720ed0
    Replacement::draft(0x141720f80, "kaim_141720f80", fn_141720f80 as *const ()), // sub_141720f80
    Replacement::draft(0x141720fe0, "kaim_141720fe0", fn_141720fe0 as *const ()), // sub_141720fe0
    Replacement::draft(0x141721400, "kaim_141721400", fn_141721400 as *const ()), // sub_141721400
    Replacement::draft(0x141721480, "kaim_141721480", fn_141721480 as *const ()), // sub_141721480
    Replacement::draft(0x141721580, "kaim_141721580", fn_141721580 as *const ()), // sub_141721580
    Replacement::draft(0x141721820, "kaim_141721820", fn_141721820 as *const ()), // sub_141721820
    Replacement::draft(0x1417218a0, "kaim_1417218a0", fn_1417218a0 as *const ()), // sub_1417218a0
    Replacement::draft(0x1417218e0, "kaim_1417218e0", fn_1417218e0 as *const ()), // sub_1417218e0
    Replacement::draft(0x141721a60, "kaim_141721a60", fn_141721a60 as *const ()), // sub_141721a60
    Replacement::draft(0x141721e20, "kaim_141721e20", fn_141721e20 as *const ()), // sub_141721e20
    Replacement::draft(0x141721f90, "kaim_141721f90", fn_141721f90 as *const ()), // sub_141721f90
    Replacement::draft(0x141722000, "kaim_141722000", fn_141722000 as *const ()), // sub_141722000
    Replacement::draft(0x1417221f0, "kaim_1417221f0", fn_1417221f0 as *const ()), // sub_1417221f0
    Replacement::draft(0x141722290, "kaim_141722290", fn_141722290 as *const ()), // sub_141722290
    Replacement::draft(0x141722320, "kaim_141722320", fn_141722320 as *const ()), // sub_141722320
    Replacement::draft(0x1417223d0, "kaim_1417223d0", fn_1417223d0 as *const ()), // sub_1417223d0
    Replacement::draft(0x141722480, "kaim_141722480", fn_141722480 as *const ()), // sub_141722480
    Replacement::draft(0x141722520, "kaim_141722520", tpl_32490::<0x10, 0x18, 0x8, 0x141722320> as *const ()), // sub_141722520
    Replacement::draft(0x141722590, "kaim_141722590", fn_141722590 as *const ()), // sub_141722590
    Replacement::draft(0x141722650, "kaim_141722650", fn_141722650 as *const ()), // sub_141722650
    Replacement::draft(0x141722700, "kaim_141722700", fn_141722700 as *const ()), // sub_141722700
    Replacement::draft(0x141722840, "kaim_141722840", fn_141722840 as *const ()), // sub_141722840
    Replacement::draft(0x141722b30, "kaim_141722b30", fn_141722b30 as *const ()), // sub_141722b30
    Replacement::draft(0x141722d30, "kaim_141722d30", fn_141722d30 as *const ()), // sub_141722d30
    Replacement::draft(0x141722df0, "kaim_141722df0", fn_141722df0 as *const ()), // sub_141722df0
    Replacement::draft(0x141722e40, "kaim_141722e40", fn_141722e40 as *const ()), // sub_141722e40
    Replacement::draft(0x1417230b0, "kaim_1417230b0", fn_1417230b0 as *const ()), // sub_1417230b0
    Replacement::draft(0x141723210, "kaim_141723210", fn_141723210 as *const ()), // sub_141723210
    Replacement::draft(0x1417233b0, "kaim_1417233b0", fn_1417233b0 as *const ()), // sub_1417233b0
    Replacement::draft(0x1417234a0, "kaim_1417234a0", fn_1417234a0 as *const ()), // sub_1417234a0
    Replacement::draft(0x141723520, "kaim_141723520", fn_141723520 as *const ()), // sub_141723520
    Replacement::draft(0x1417235d0, "kaim_1417235d0", fn_1417235d0 as *const ()), // sub_1417235d0
    Replacement::draft(0x141723610, "kaim_141723610", fn_141723610 as *const ()), // sub_141723610
    Replacement::draft(0x141723630, "kaim_141723630", fn_141723630 as *const ()), // sub_141723630
    Replacement::draft(0x141723670, "kaim_141723670", fn_141723670 as *const ()), // sub_141723670
    Replacement::draft(0x1417238b0, "kaim_1417238b0", fn_1417238b0 as *const ()), // sub_1417238b0
    Replacement::draft(0x141723ab0, "kaim_141723ab0", fn_141723ab0 as *const ()), // sub_141723ab0
    Replacement::draft(0x141723e50, "kaim_141723e50", fn_141723e50 as *const ()), // sub_141723e50
    Replacement::draft(0x141724200, "kaim_141724200", fn_141724200 as *const ()), // sub_141724200
    Replacement::draft(0x141724270, "kaim_141724270", fn_141724270 as *const ()), // sub_141724270
    Replacement::draft(0x1417242c0, "kaim_1417242c0", fn_1417242c0 as *const ()), // sub_1417242c0
    Replacement::draft(0x141724330, "kaim_141724330", fn_141724330 as *const ()), // sub_141724330
    Replacement::draft(0x141724400, "kaim_141724400", fn_141724400 as *const ()), // sub_141724400
    Replacement::draft(0x141724460, "kaim_141724460", fn_141724460 as *const ()), // sub_141724460
    Replacement::draft(0x141724820, "kaim_141724820", fn_141724820 as *const ()), // sub_141724820
    Replacement::draft(0x141724cb0, "kaim_141724cb0", fn_141724cb0 as *const ()), // sub_141724cb0
    Replacement::draft(0x141724d50, "kaim_141724d50", fn_141724d50 as *const ()), // sub_141724d50
    Replacement::draft(0x141724d70, "kaim_141724d70", fn_141724d70 as *const ()), // sub_141724d70
    Replacement::draft(0x141724ec0, "kaim_141724ec0", fn_141724ec0 as *const ()), // sub_141724ec0
    Replacement::draft(0x141724ee0, "kaim_141724ee0", fn_141724ee0 as *const ()), // sub_141724ee0
    Replacement::draft(0x141724f10, "kaim_141724f10", tpl_32518::<0x8, 0x70, 0xa4, 0x141725440, 0xc, 0x88, 0xa5> as *const ()), // sub_141724f10
    Replacement::draft(0x141724f60, "kaim_141724f60", tpl_32519::<0xa4, 0x8, 0x1, 0xa5, 0xc> as *const ()), // sub_141724f60
    Replacement::draft(0x141724fb0, "kaim_141724fb0", tpl_32520::<0x10, 0x8, 0x18, 0x14, 0x4, 0xc, 0x1c, 0x20, 0xa2, 0x24, 0x40, 0xa0, 0x141725440> as *const ()), // sub_141724fb0
    Replacement::draft(0x141725050, "kaim_141725050", fn_141725050 as *const ()), // sub_141725050
    Replacement::draft(0x141725440, "kaim_141725440", fn_141725440 as *const ()), // sub_141725440
    Replacement::draft(0x1417254c0, "kaim_1417254c0", fn_1417254c0 as *const ()), // sub_1417254c0
    Replacement::draft(0x1417255f0, "kaim_1417255f0", fn_1417255f0 as *const ()), // sub_1417255f0
    Replacement::draft(0x1417256f0, "kaim_1417256f0", tpl_32518::<0x4, 0x58, 0xa3, 0x141725440, 0xc, 0x88, 0xa5> as *const ()), // sub_1417256f0
    Replacement::draft(0x141725740, "kaim_141725740", tpl_32519::<0xa3, 0x4, 0x1, 0xa5, 0xc> as *const ()), // sub_141725740
    Replacement::draft(0x141725790, "kaim_141725790", tpl_32524::<0xa8, 0x8, 0x141725850, 0x14169eda0, 0xfffffffffffffff8> as *const ()), // sub_141725790
    Replacement::draft(0x1417257f0, "kaim_1417257f0", tpl_32524::<0xc0, 0x8, 0x141725850, 0x14169eda0, 0xfffffffffffffff8> as *const ()), // sub_1417257f0
    Replacement::draft(0x141725850, "kaim_141725850", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x1416a2ea0> as *const ()), // sub_141725850
    Replacement::draft(0x1417258f0, "kaim_1417258f0", tpl_32520::<0x28, 0x8, 0x30, 0x2c, 0x4, 0xc, 0x34, 0x38, 0xa2, 0x3c, 0x40, 0xa1, 0x141725440> as *const ()), // sub_1417258f0
    Replacement::draft(0x141725990, "kaim_141725990", fn_141725990 as *const ()), // sub_141725990
    Replacement::draft(0x1417259f0, "kaim_1417259f0", fn_1417259f0 as *const ()), // sub_1417259f0
    Replacement::draft(0x141725b10, "kaim_141725b10", fn_141725b10 as *const ()), // sub_141725b10
    Replacement::draft(0x141725c10, "kaim_141725c10", fn_141725c10 as *const ()), // sub_141725c10
    Replacement::draft(0x141725c90, "kaim_141725c90", fn_141725c90 as *const ()), // sub_141725c90
    Replacement::draft(0x141725e20, "kaim_141725e20", fn_141725e20 as *const ()), // sub_141725e20
    Replacement::draft(0x141725fc0, "kaim_141725fc0", fn_141725fc0 as *const ()), // sub_141725fc0
    Replacement::draft(0x141726130, "kaim_141726130", fn_141726130 as *const ()), // sub_141726130
    Replacement::draft(0x141726140, "kaim_141726140", fn_141726140 as *const ()), // sub_141726140
    Replacement::draft(0x141726240, "kaim_141726240", fn_141726240 as *const ()), // sub_141726240
    Replacement::draft(0x1417262d0, "kaim_1417262d0", fn_1417262d0 as *const ()), // sub_1417262d0
    Replacement::draft(0x141726520, "kaim_141726520", fn_141726520 as *const ()), // sub_141726520
    Replacement::draft(0x141726650, "kaim_141726650", fn_141726650 as *const ()), // sub_141726650
    Replacement::draft(0x141726740, "kaim_141726740", fn_141726740 as *const ()), // sub_141726740
    Replacement::draft(0x141726950, "kaim_141726950", fn_141726950 as *const ()), // sub_141726950
    Replacement::draft(0x1417269e0, "kaim_1417269e0", fn_1417269e0 as *const ()), // sub_1417269e0
    Replacement::draft(0x141726a10, "kaim_141726a10", fn_141726a10 as *const ()), // sub_141726a10
    Replacement::draft(0x141726d00, "kaim_141726d00", fn_141726d00 as *const ()), // sub_141726d00
    Replacement::draft(0x141727000, "kaim_141727000", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac170> as *const ()), // sub_141727000
    Replacement::draft(0x141727060, "kaim_141727060", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac1a8, 0x8> as *const ()), // sub_141727060
    Replacement::draft(0x1417270a0, "kaim_1417270a0", fn_1417270a0 as *const ()), // sub_1417270a0
    Replacement::draft(0x141727120, "kaim_141727120", fn_141727120 as *const ()), // sub_141727120
    Replacement::draft(0x1417271d0, "kaim_1417271d0", tpl_22446::<0x1417287c0> as *const ()), // sub_1417271d0
    Replacement::draft(0x1417271e0, "kaim_1417271e0", fn_1417271e0 as *const ()), // sub_1417271e0
    Replacement::draft(0x1417275e0, "kaim_1417275e0", fn_1417275e0 as *const ()), // sub_1417275e0
    Replacement::draft(0x141727670, "kaim_141727670", fn_141727670 as *const ()), // sub_141727670
    Replacement::draft(0x141727680, "kaim_141727680", tpl_1::<0x1426ac138> as *const ()), // sub_141727680
    Replacement::draft(0x141727690, "kaim_141727690", fn_141727690 as *const ()), // sub_141727690
    Replacement::draft(0x1417277b0, "kaim_1417277b0", fn_1417277b0 as *const ()), // sub_1417277b0
    Replacement::draft(0x141727930, "kaim_141727930", fn_141727930 as *const ()), // sub_141727930
    Replacement::draft(0x141727be0, "kaim_141727be0", fn_141727be0 as *const ()), // sub_141727be0
    Replacement::draft(0x141727dc0, "kaim_141727dc0", fn_141727dc0 as *const ()), // sub_141727dc0
    Replacement::draft(0x141728020, "kaim_141728020", fn_141728020 as *const ()), // sub_141728020
    Replacement::draft(0x1417282a0, "kaim_1417282a0", fn_1417282a0 as *const ()), // sub_1417282a0
    Replacement::draft(0x141728420, "kaim_141728420", fn_141728420 as *const ()), // sub_141728420
    Replacement::draft(0x141728450, "kaim_141728450", fn_141728450 as *const ()), // sub_141728450
    Replacement::draft(0x141728500, "kaim_141728500", fn_141728500 as *const ()), // sub_141728500
    Replacement::draft(0x1417287c0, "kaim_1417287c0", fn_1417287c0 as *const ()), // sub_1417287c0
    Replacement::draft(0x1417288d0, "kaim_1417288d0", fn_1417288d0 as *const ()), // sub_1417288d0
    Replacement::draft(0x1417289e0, "kaim_1417289e0", fn_1417289e0 as *const ()), // sub_1417289e0
    Replacement::draft(0x141728b50, "unknown_141728b50", fn_141728b50 as *const ()), // sub_141728b50
    Replacement::draft(0x141728f60, "unknown_141728f60", fn_141728f60 as *const ()), // sub_141728f60
    Replacement::draft(0x141729170, "unknown_141729170", fn_141729170 as *const ()), // sub_141729170
    Replacement::draft(0x141729440, "unknown_141729440", fn_141729440 as *const ()), // sub_141729440
    Replacement::draft(0x141729720, "unknown_141729720", fn_141729720 as *const ()), // sub_141729720
    Replacement::draft(0x1417299d0, "unknown_1417299d0", fn_1417299d0 as *const ()), // sub_1417299d0
    Replacement::draft(0x141729a60, "unknown_141729a60", fn_141729a60 as *const ()), // sub_141729a60
    Replacement::draft(0x141729ad0, "unknown_141729ad0", fn_141729ad0 as *const ()), // sub_141729ad0
    Replacement::draft(0x141729af0, "unknown_141729af0", fn_141729af0 as *const ()), // sub_141729af0
    Replacement::draft(0x141729c10, "unknown_141729c10", fn_141729c10 as *const ()), // sub_141729c10
    Replacement::draft(0x141729c20, "unknown_141729c20", fn_141729c20 as *const ()), // sub_141729c20
    Replacement::draft(0x141729ea0, "unknown_141729ea0", fn_141729ea0 as *const ()), // sub_141729ea0
    Replacement::draft(0x141729ed0, "unknown_141729ed0", fn_141729ed0 as *const ()), // sub_141729ed0
    Replacement::draft(0x141729f80, "unknown_141729f80", fn_141729f80 as *const ()), // sub_141729f80
    Replacement::draft(0x14172a0b0, "unknown_14172a0b0", fn_14172a0b0 as *const ()), // sub_14172a0b0
    Replacement::draft(0x14172a0d0, "unknown_14172a0d0", fn_14172a0d0 as *const ()), // sub_14172a0d0
    Replacement::draft(0x14172a120, "unknown_14172a120", fn_14172a120 as *const ()), // sub_14172a120
    Replacement::draft(0x14172a240, "unknown_14172a240", fn_14172a240 as *const ()), // sub_14172a240
    Replacement::draft(0x14172a2d0, "kaim_14172a2d0", fn_14172a2d0 as *const ()), // sub_14172a2d0
    Replacement::draft(0x14172a6d0, "unknown_14172a6d0", fn_14172a6d0 as *const ()), // sub_14172a6d0
    Replacement::draft(0x14172a740, "unknown_14172a740", tpl_365::<0x14172a6d0> as *const ()), // sub_14172a740
    Replacement::draft(0x14172a750, "unknown_14172a750", fn_14172a750 as *const ()), // sub_14172a750
    Replacement::draft(0x14172a810, "kaim_14172a810", tpl_32568::<0x1426abaa8, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_14172a810
    Replacement::draft(0x14172a880, "kaim_14172a880", fn_14172a880 as *const ()), // sub_14172a880
    Replacement::draft(0x14172ac60, "kaim_14172ac60", fn_14172ac60 as *const ()), // sub_14172ac60
    Replacement::draft(0x14172ada0, "kaim_14172ada0", fn_14172ada0 as *const ()), // sub_14172ada0
    Replacement::draft(0x14172ae10, "kaim_14172ae10", fn_14172ae10 as *const ()), // sub_14172ae10
    Replacement::draft(0x14172ae30, "kaim_14172ae30", fn_14172ae30 as *const ()), // sub_14172ae30
    Replacement::draft(0x14172b0c0, "kaim_14172b0c0", fn_14172b0c0 as *const ()), // sub_14172b0c0
    Replacement::draft(0x14172b460, "kaim_14172b460", fn_14172b460 as *const ()), // sub_14172b460
    Replacement::draft(0x14172b5c0, "kaim_14172b5c0", fn_14172b5c0 as *const ()), // sub_14172b5c0
    Replacement::draft(0x14172b6e0, "kaim_14172b6e0", fn_14172b6e0 as *const ()), // sub_14172b6e0
    Replacement::draft(0x14172b860, "kaim_14172b860", fn_14172b860 as *const ()), // sub_14172b860
    Replacement::draft(0x14172b8e0, "kaim_14172b8e0", fn_14172b8e0 as *const ()), // sub_14172b8e0
    Replacement::draft(0x14172b980, "kaim_14172b980", fn_14172b980 as *const ()), // sub_14172b980
    Replacement::draft(0x14172ba30, "kaim_14172ba30", fn_14172ba30 as *const ()), // sub_14172ba30
    Replacement::draft(0x14172ba70, "kaim_14172ba70", fn_14172ba70 as *const ()), // sub_14172ba70
    Replacement::draft(0x14172bb20, "kaim_14172bb20", fn_14172bb20 as *const ()), // sub_14172bb20
    Replacement::draft(0x14172bd30, "kaim_14172bd30", fn_14172bd30 as *const ()), // sub_14172bd30
    Replacement::draft(0x14172be00, "kaim_14172be00", fn_14172be00 as *const ()), // sub_14172be00
    Replacement::draft(0x14172be10, "kaim_14172be10", fn_14172be10 as *const ()), // sub_14172be10
    Replacement::draft(0x14172bfc0, "kaim_14172bfc0", fn_14172bfc0 as *const ()), // sub_14172bfc0
    Replacement::draft(0x14172c030, "kaim_14172c030", fn_14172c030 as *const ()), // sub_14172c030
    Replacement::draft(0x14172c5f0, "kaim_14172c5f0", fn_14172c5f0 as *const ()), // sub_14172c5f0
    Replacement::draft(0x14172caa0, "kaim_14172caa0", fn_14172caa0 as *const ()), // sub_14172caa0
    Replacement::draft(0x14172ccb0, "kaim_14172ccb0", fn_14172ccb0 as *const ()), // sub_14172ccb0
    Replacement::draft(0x14172cd60, "kaim_14172cd60", fn_14172cd60 as *const ()), // sub_14172cd60
    Replacement::draft(0x14172ce10, "kaim_14172ce10", fn_14172ce10 as *const ()), // sub_14172ce10
    Replacement::draft(0x14172cec0, "kaim_14172cec0", tpl_22805::<0x8, 0x10, 0x14172ce10, 0xc, 0x18, 0xfffffffffffffff8, 0xfffffffffffffffc> as *const ()), // sub_14172cec0
    Replacement::draft(0x14172cf80, "kaim_14172cf80", tpl_22437::<0x10, 0x8, 0x1416a2ea0> as *const ()), // sub_14172cf80
    Replacement::draft(0x14172d000, "kaim_14172d000", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x14172cd60> as *const ()), // sub_14172d000
    Replacement::draft(0x14172d0a0, "kaim_14172d0a0", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x1416bce80> as *const ()), // sub_14172d0a0
    Replacement::draft(0x14172d140, "kaim_14172d140", fn_14172d140 as *const ()), // sub_14172d140
    Replacement::draft(0x14172d530, "kaim_14172d530", fn_14172d530 as *const ()), // sub_14172d530
    Replacement::draft(0x14172d5a0, "kaim_14172d5a0", fn_14172d5a0 as *const ()), // sub_14172d5a0
    Replacement::draft(0x14172da40, "kaim_14172da40", fn_14172da40 as *const ()), // sub_14172da40
    Replacement::draft(0x14172dae0, "kaim_14172dae0", fn_14172dae0 as *const ()), // sub_14172dae0
    Replacement::draft(0x14172dba0, "kaim_14172dba0", fn_14172dba0 as *const ()), // sub_14172dba0
    Replacement::draft(0x14172dc30, "kaim_14172dc30", fn_14172dc30 as *const ()), // sub_14172dc30
    Replacement::draft(0x14172ddd0, "kaim_14172ddd0", fn_14172ddd0 as *const ()), // sub_14172ddd0
    Replacement::draft(0x14172df80, "kaim_14172df80", fn_14172df80 as *const ()), // sub_14172df80
    Replacement::draft(0x14172e0a0, "kaim_14172e0a0", fn_14172e0a0 as *const ()), // sub_14172e0a0
    Replacement::draft(0x14172e260, "kaim_14172e260", fn_14172e260 as *const ()), // sub_14172e260
    Replacement::draft(0x14172e320, "kaim_14172e320", fn_14172e320 as *const ()), // sub_14172e320
    Replacement::draft(0x14172e440, "kaim_14172e440", fn_14172e440 as *const ()), // sub_14172e440
    Replacement::draft(0x14172e560, "kaim_14172e560", fn_14172e560 as *const ()), // sub_14172e560
    Replacement::draft(0x14172e720, "kaim_14172e720", fn_14172e720 as *const ()), // sub_14172e720
    Replacement::draft(0x14172e8d0, "kaim_14172e8d0", fn_14172e8d0 as *const ()), // sub_14172e8d0
    Replacement::draft(0x14172ea70, "kaim_14172ea70", fn_14172ea70 as *const ()), // sub_14172ea70
    Replacement::draft(0x14172ec20, "kaim_14172ec20", fn_14172ec20 as *const ()), // sub_14172ec20
    Replacement::draft(0x14172ec70, "kaim_14172ec70", fn_14172ec70 as *const ()), // sub_14172ec70
    Replacement::draft(0x14172ed50, "kaim_14172ed50", fn_14172ed50 as *const ()), // sub_14172ed50
    Replacement::draft(0x14172f450, "kaim_14172f450", fn_14172f450 as *const ()), // sub_14172f450
    Replacement::draft(0x14172f940, "kaim_14172f940", fn_14172f940 as *const ()), // sub_14172f940
    Replacement::draft(0x14172fcc0, "kaim_14172fcc0", fn_14172fcc0 as *const ()), // sub_14172fcc0
    Replacement::draft(0x1417302d0, "kaim_1417302d0", fn_1417302d0 as *const ()), // sub_1417302d0
    Replacement::draft(0x141730790, "kaim_141730790", fn_141730790 as *const ()), // sub_141730790
    Replacement::draft(0x141730ca0, "kaim_141730ca0", fn_141730ca0 as *const ()), // sub_141730ca0
    Replacement::draft(0x1417310a0, "kaim_1417310a0", fn_1417310a0 as *const ()), // sub_1417310a0
    Replacement::draft(0x1417310d0, "kaim_1417310d0", fn_1417310d0 as *const ()), // sub_1417310d0
    Replacement::draft(0x141731200, "kaim_141731200", fn_141731200 as *const ()), // sub_141731200
    Replacement::draft(0x141731610, "kaim_141731610", fn_141731610 as *const ()), // sub_141731610
    Replacement::draft(0x141731960, "kaim_141731960", fn_141731960 as *const ()), // sub_141731960
    Replacement::draft(0x1417319f0, "kaim_1417319f0", fn_1417319f0 as *const ()), // sub_1417319f0
    Replacement::draft(0x141731a60, "kaim_141731a60", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac238> as *const ()), // sub_141731a60
    Replacement::draft(0x141731ac0, "kaim_141731ac0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac270, 0x8> as *const ()), // sub_141731ac0
    Replacement::draft(0x141731b00, "kaim_141731b00", fn_141731b00 as *const ()), // sub_141731b00
    Replacement::draft(0x141731c40, "kaim_141731c40", fn_141731c40 as *const ()), // sub_141731c40
    Replacement::draft(0x141731d60, "kaim_141731d60", tpl_22446::<0x1416d76f0> as *const ()), // sub_141731d60
    Replacement::draft(0x141731d70, "kaim_141731d70", fn_141731d70 as *const ()), // sub_141731d70
    Replacement::draft(0x141731d80, "kaim_141731d80", tpl_1::<0x1426ac210> as *const ()), // sub_141731d80
    Replacement::draft(0x141731d90, "kaim_141731d90", fn_141731d90 as *const ()), // sub_141731d90
    Replacement::draft(0x141731e40, "kaim_141731e40", fn_141731e40 as *const ()), // sub_141731e40
    Replacement::draft(0x141731fd0, "kaim_141731fd0", fn_141731fd0 as *const ()), // sub_141731fd0
    Replacement::draft(0x1417321f0, "kaim_1417321f0", fn_1417321f0 as *const ()), // sub_1417321f0
    Replacement::draft(0x1417322b0, "kaim_1417322b0", fn_1417322b0 as *const ()), // sub_1417322b0
    Replacement::draft(0x1417322e0, "kaim_1417322e0", fn_1417322e0 as *const ()), // sub_1417322e0
    Replacement::draft(0x141732370, "kaim_141732370", fn_141732370 as *const ()), // sub_141732370
    Replacement::draft(0x141732470, "kaim_141732470", fn_141732470 as *const ()), // sub_141732470
    Replacement::draft(0x141732490, "kaim_141732490", fn_141732490 as *const ()), // sub_141732490
    Replacement::draft(0x1417324c0, "kaim_1417324c0", fn_1417324c0 as *const ()), // sub_1417324c0
    Replacement::draft(0x141732540, "kaim_141732540", fn_141732540 as *const ()), // sub_141732540
    Replacement::draft(0x1417325c0, "kaim_1417325c0", fn_1417325c0 as *const ()), // sub_1417325c0
    Replacement::draft(0x141732680, "kaim_141732680", fn_141732680 as *const ()), // sub_141732680
    Replacement::draft(0x1417327e0, "kaim_1417327e0", fn_1417327e0 as *const ()), // sub_1417327e0
    Replacement::draft(0x141732840, "kaim_141732840", tpl_32619::<0x141732b80, 0x143932190, 0x60> as *const ()), // sub_141732840
    Replacement::draft(0x141732870, "kaim_141732870", tpl_32619::<0x141732c50, 0x143932190, 0x60> as *const ()), // sub_141732870
    Replacement::draft(0x1417328a0, "kaim_1417328a0", fn_1417328a0 as *const ()), // sub_1417328a0
    Replacement::draft(0x1417328f0, "kaim_1417328f0", fn_1417328f0 as *const ()), // sub_1417328f0
    Replacement::draft(0x141732a20, "kaim_141732a20", fn_141732a20 as *const ()), // sub_141732a20
    Replacement::draft(0x141732aa0, "kaim_141732aa0", fn_141732aa0 as *const ()), // sub_141732aa0
    Replacement::draft(0x141732b40, "kaim_141732b40", tpl_32624::<0x40, 0x38, 0x141733760> as *const ()), // sub_141732b40
    Replacement::draft(0x141732b80, "kaim_141732b80", fn_141732b80 as *const ()), // sub_141732b80
    Replacement::draft(0x141732c50, "kaim_141732c50", fn_141732c50 as *const ()), // sub_141732c50
    Replacement::draft(0x141732d40, "kaim_141732d40", fn_141732d40 as *const ()), // sub_141732d40
    Replacement::draft(0x141732db0, "kaim_141732db0", tpl_32624::<0x40, 0x38, 0x141733970> as *const ()), // sub_141732db0
    Replacement::draft(0x141732df0, "kaim_141732df0", tpl_32624::<0x40, 0x38, 0x141733af0> as *const ()), // sub_141732df0
    Replacement::draft(0x141732e30, "kaim_141732e30", tpl_1::<0x1426ac2e8> as *const ()), // sub_141732e30
    Replacement::draft(0x141732e40, "kaim_141732e40", fn_141732e40 as *const ()), // sub_141732e40
    Replacement::draft(0x141733200, "kaim_141733200", tpl_32624::<0x40, 0x38, 0x141733cb0> as *const ()), // sub_141733200
    Replacement::draft(0x141733240, "kaim_141733240", fn_141733240 as *const ()), // sub_141733240
    Replacement::draft(0x141733330, "kaim_141733330", fn_141733330 as *const ()), // sub_141733330
    Replacement::draft(0x1417333d0, "kaim_1417333d0", fn_1417333d0 as *const ()), // sub_1417333d0
    Replacement::draft(0x141733480, "kaim_141733480", tpl_31540::<0x8, 0xffffffffffffffff, 0x14169edb0, 0x10, 0x1417333d0> as *const ()), // sub_141733480
    Replacement::draft(0x141733520, "kaim_141733520", fn_141733520 as *const ()), // sub_141733520
    Replacement::draft(0x141733570, "kaim_141733570", fn_141733570 as *const ()), // sub_141733570
    Replacement::draft(0x1417335e0, "kaim_1417335e0", fn_1417335e0 as *const ()), // sub_1417335e0
    Replacement::draft(0x141733620, "kaim_141733620", fn_141733620 as *const ()), // sub_141733620
    Replacement::draft(0x141733660, "kaim_141733660", tpl_18960::<0x8, 0x14169edb0> as *const ()), // sub_141733660
    Replacement::draft(0x141733680, "kaim_141733680", fn_141733680 as *const ()), // sub_141733680
    Replacement::draft(0x141733760, "kaim_141733760", fn_141733760 as *const ()), // sub_141733760
    Replacement::draft(0x1417338e0, "kaim_1417338e0", tpl_32636::<0x141733760, 0x58, 0x1416a7a00> as *const ()), // sub_1417338e0
    Replacement::draft(0x141733900, "kaim_141733900", fn_141733900 as *const ()), // sub_141733900
    Replacement::draft(0x141733970, "kaim_141733970", fn_141733970 as *const ()), // sub_141733970
    Replacement::draft(0x141733af0, "kaim_141733af0", fn_141733af0 as *const ()), // sub_141733af0
    Replacement::draft(0x141733c70, "kaim_141733c70", fn_141733c70 as *const ()), // sub_141733c70
    Replacement::draft(0x141733cb0, "kaim_141733cb0", fn_141733cb0 as *const ()), // sub_141733cb0
    Replacement::draft(0x141733e60, "kaim_141733e60", fn_141733e60 as *const ()), // sub_141733e60
    Replacement::draft(0x141733f00, "kaim_141733f00", fn_141733f00 as *const ()), // sub_141733f00
    Replacement::draft(0x141733f60, "kaim_141733f60", fn_141733f60 as *const ()), // sub_141733f60
    Replacement::draft(0x1417340b0, "kaim_1417340b0", fn_1417340b0 as *const ()), // sub_1417340b0
    Replacement::draft(0x141734220, "kaim_141734220", fn_141734220 as *const ()), // sub_141734220
    Replacement::draft(0x141734350, "kaim_141734350", tpl_22426::<0x1426ac338, 0x14169ed80> as *const ()), // sub_141734350
    Replacement::draft(0x141734360, "kaim_141734360", fn_141734360 as *const ()), // sub_141734360
    Replacement::draft(0x141734410, "kaim_141734410", tpl_32568::<0x1426ac338, 0x14169ed80, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_141734410
    Replacement::draft(0x141734480, "unknown_141734480", fn_141734480 as *const ()), // sub_141734480
    Replacement::draft(0x141734580, "unknown_141734580", fn_141734580 as *const ()), // sub_141734580
    Replacement::draft(0x1417345c0, "unknown_1417345c0", fn_1417345c0 as *const ()), // sub_1417345c0
    Replacement::draft(0x1417347f0, "unknown_1417347f0", tpl_365::<0x141734800> as *const ()), // sub_1417347f0
    Replacement::draft(0x141734800, "unknown_141734800", fn_141734800 as *const ()), // sub_141734800
    Replacement::draft(0x141734870, "unknown_141734870", fn_141734870 as *const ()), // sub_141734870
    Replacement::draft(0x141734980, "unknown_141734980", fn_141734980 as *const ()), // sub_141734980
    Replacement::draft(0x141734a90, "unknown_141734a90", fn_141734a90 as *const ()), // sub_141734a90
    Replacement::draft(0x141734b70, "unknown_141734b70", fn_141734b70 as *const ()), // sub_141734b70
    Replacement::draft(0x141734b90, "unknown_141734b90", tpl_906::<0x142646540> as *const ()), // sub_141734b90
    Replacement::draft(0x141734ba0, "unknown_141734ba0", fn_141734ba0 as *const ()), // sub_141734ba0
    Replacement::draft_throws(0x141734bc0, "unknown_141734bc0", fn_141734bc0 as *const (), Throws::Discard), // sub_141734bc0
    Replacement::draft(0x141734c10, "unknown_141734c10", fn_141734c10 as *const ()), // sub_141734c10
    Replacement::draft(0x141734c50, "unknown_141734c50", fn_141734c50 as *const ()), // sub_141734c50
    Replacement::draft(0x141734c80, "unknown_141734c80", fn_141734c80 as *const ()), // sub_141734c80
    Replacement::draft(0x141734cf0, "unknown_141734cf0", tpl_366::<0x143932488> as *const ()), // sub_141734cf0
    Replacement::draft(0x141734d00, "unknown_141734d00", tpl_28926::<0x143932488> as *const ()), // sub_141734d00
    Replacement::draft(0x141734d10, "ak_141734d10", fn_141734d10 as *const ()), // sub_141734d10
    Replacement::draft(0x141734d60, "unknown_141734d60", tpl_365::<0x141734c50> as *const ()), // sub_141734d60
    Replacement::draft(0x141734d70, "unknown_141734d70", tpl_22521::<0x1426ac3d8, 0x120, 0x14169edb0, 0x141734c50> as *const ()), // sub_141734d70
    Replacement::draft(0x141734db0, "unknown_141734db0", fn_141734db0 as *const ()), // sub_141734db0
    Replacement::draft(0x141734e30, "unknown_141734e30", fn_141734e30 as *const ()), // sub_141734e30
    Replacement::draft(0x141734e90, "unknown_141734e90", tpl_913::<0x128> as *const ()), // sub_141734e90
    Replacement::draft(0x141734ea0, "unknown_141734ea0", fn_141734ea0 as *const ()), // sub_141734ea0
    Replacement::draft(0x141734ec0, "unknown_141734ec0", tpl_1738::<0x128> as *const ()), // sub_141734ec0
    Replacement::draft(0x141734ed0, "unknown_141734ed0", fn_141734ed0 as *const ()), // sub_141734ed0
    Replacement::draft(0x141735340, "unknown_141735340", fn_141735340 as *const ()), // sub_141735340
    Replacement::draft(0x141735440, "unknown_141735440", fn_141735440 as *const ()), // sub_141735440
    Replacement::draft(0x1417355a0, "unknown_1417355a0", fn_1417355a0 as *const ()), // sub_1417355a0
    Replacement::draft(0x1417355d0, "unknown_1417355d0", fn_1417355d0 as *const ()), // sub_1417355d0
    Replacement::draft(0x141735660, "unknown_141735660", fn_141735660 as *const ()), // sub_141735660
    Replacement::draft(0x1417356c0, "unknown_1417356c0", fn_1417356c0 as *const ()), // sub_1417356c0
    Replacement::draft(0x141735770, "unknown_141735770", fn_141735770 as *const ()), // sub_141735770
    Replacement::draft(0x141735930, "kaim_141735930", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac540> as *const ()), // sub_141735930
    Replacement::draft(0x141735990, "kaim_141735990", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac578> as *const ()), // sub_141735990
    Replacement::draft(0x1417359f0, "kaim_1417359f0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac508> as *const ()), // sub_1417359f0
    Replacement::draft(0x141735a50, "kaim_141735a50", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac498, 0x8> as *const ()), // sub_141735a50
    Replacement::draft(0x141735a90, "kaim_141735a90", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac4d0, 0x8> as *const ()), // sub_141735a90
    Replacement::draft(0x141735ad0, "kaim_141735ad0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac460, 0x8> as *const ()), // sub_141735ad0
    Replacement::draft(0x141735b10, "kaim_141735b10", fn_141735b10 as *const ()), // sub_141735b10
    Replacement::draft(0x141735b20, "kaim_141735b20", fn_141735b20 as *const ()), // sub_141735b20
    Replacement::draft(0x141735b30, "kaim_141735b30", fn_141735b30 as *const ()), // sub_141735b30
    Replacement::draft(0x141735b40, "kaim_141735b40", tpl_1::<0x1426ac438> as *const ()), // sub_141735b40
    Replacement::draft(0x141735b50, "kaim_141735b50", tpl_1::<0x1426ac450> as *const ()), // sub_141735b50
    Replacement::draft(0x141735b60, "kaim_141735b60", tpl_1::<0x1426ac428> as *const ()), // sub_141735b60
    Replacement::draft(0x141735b70, "kaim_141735b70", fn_141735b70 as *const ()), // sub_141735b70
    Replacement::draft(0x141735cd0, "kaim_141735cd0", fn_141735cd0 as *const ()), // sub_141735cd0
    Replacement::draft(0x141735ec0, "kaim_141735ec0", fn_141735ec0 as *const ()), // sub_141735ec0
    Replacement::draft(0x141736060, "kaim_141736060", fn_141736060 as *const ()), // sub_141736060
    Replacement::draft(0x141736170, "kaim_141736170", fn_141736170 as *const ()), // sub_141736170
    Replacement::draft(0x141736990, "kaim_141736990", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac748> as *const ()), // sub_141736990
    Replacement::draft(0x1417369f0, "kaim_1417369f0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aceb8> as *const ()), // sub_1417369f0
    Replacement::draft(0x141736a50, "kaim_141736a50", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ace80> as *const ()), // sub_141736a50
    Replacement::draft(0x141736ab0, "kaim_141736ab0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ace48> as *const ()), // sub_141736ab0
    Replacement::draft(0x141736b10, "kaim_141736b10", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426accc0> as *const ()), // sub_141736b10
    Replacement::draft(0x141736b70, "kaim_141736b70", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acd30> as *const ()), // sub_141736b70
    Replacement::draft(0x141736bd0, "kaim_141736bd0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acc50> as *const ()), // sub_141736bd0
    Replacement::draft(0x141736c30, "kaim_141736c30", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac6b0> as *const ()), // sub_141736c30
    Replacement::draft(0x141736c90, "kaim_141736c90", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ace10> as *const ()), // sub_141736c90
    Replacement::draft(0x141736cf0, "kaim_141736cf0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acba8> as *const ()), // sub_141736cf0
    Replacement::draft(0x141736d50, "kaim_141736d50", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ac7b8> as *const ()), // sub_141736d50
    Replacement::draft(0x141736db0, "kaim_141736db0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acc88> as *const ()), // sub_141736db0
    Replacement::draft(0x141736e10, "kaim_141736e10", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acc18> as *const ()), // sub_141736e10
    Replacement::draft(0x141736e70, "kaim_141736e70", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acdd8> as *const ()), // sub_141736e70
    Replacement::draft(0x141736ed0, "kaim_141736ed0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acbe0> as *const ()), // sub_141736ed0
    Replacement::draft(0x141736f30, "kaim_141736f30", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426accf8> as *const ()), // sub_141736f30
    Replacement::draft(0x141736f90, "kaim_141736f90", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acd68> as *const ()), // sub_141736f90
    Replacement::draft(0x141736ff0, "kaim_141736ff0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426acda0> as *const ()), // sub_141736ff0
    Replacement::draft(0x141737050, "kaim_141737050", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac828, 0x8> as *const ()), // sub_141737050
    Replacement::draft(0x141737090, "kaim_141737090", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426acb70, 0x8> as *const ()), // sub_141737090
    Replacement::draft(0x1417370d0, "kaim_1417370d0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426acb38, 0x8> as *const ()), // sub_1417370d0
    Replacement::draft(0x141737110, "kaim_141737110", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426acb00, 0x8> as *const ()), // sub_141737110
    Replacement::draft(0x141737150, "kaim_141737150", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac978, 0x8> as *const ()), // sub_141737150
    Replacement::draft(0x141737190, "kaim_141737190", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac9e8, 0x8> as *const ()), // sub_141737190
    Replacement::draft(0x1417371d0, "kaim_1417371d0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac908, 0x8> as *const ()), // sub_1417371d0
    Replacement::draft(0x141737210, "kaim_141737210", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac780, 0x8> as *const ()), // sub_141737210
    Replacement::draft(0x141737250, "kaim_141737250", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426acac8, 0x8> as *const ()), // sub_141737250
    Replacement::draft(0x141737290, "kaim_141737290", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac860, 0x8> as *const ()), // sub_141737290
    Replacement::draft(0x1417372d0, "kaim_1417372d0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac7f0, 0x8> as *const ()), // sub_1417372d0
    Replacement::draft(0x141737310, "kaim_141737310", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac940, 0x8> as *const ()), // sub_141737310
    Replacement::draft(0x141737350, "kaim_141737350", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac8d0, 0x8> as *const ()), // sub_141737350
    Replacement::draft(0x141737390, "kaim_141737390", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aca90, 0x8> as *const ()), // sub_141737390
    Replacement::draft(0x1417373d0, "kaim_1417373d0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac898, 0x8> as *const ()), // sub_1417373d0
    Replacement::draft(0x141737410, "kaim_141737410", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ac9b0, 0x8> as *const ()), // sub_141737410
    Replacement::draft(0x141737450, "kaim_141737450", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aca20, 0x8> as *const ()), // sub_141737450
    Replacement::draft(0x141737490, "kaim_141737490", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aca58, 0x8> as *const ()), // sub_141737490
    Replacement::draft(0x1417374d0, "kaim_1417374d0", tpl_22446::<0x141737730> as *const ()), // sub_1417374d0
    Replacement::draft(0x1417374e0, "kaim_1417374e0", tpl_22446::<0x141737810> as *const ()), // sub_1417374e0
    Replacement::draft(0x1417374f0, "kaim_1417374f0", tpl_22446::<0x141737a30> as *const ()), // sub_1417374f0
    Replacement::draft(0x141737500, "kaim_141737500", tpl_22446::<0x141737b30> as *const ()), // sub_141737500
    Replacement::draft(0x141737510, "kaim_141737510", tpl_22446::<0x141737be0> as *const ()), // sub_141737510
    Replacement::draft(0x141737520, "kaim_141737520", tpl_22446::<0x141737cb0> as *const ()), // sub_141737520
    Replacement::draft(0x141737530, "kaim_141737530", tpl_22446::<0x1417288d0> as *const ()), // sub_141737530
    Replacement::draft(0x141737540, "kaim_141737540", fn_141737540 as *const ()), // sub_141737540
    Replacement::draft(0x141737560, "kaim_141737560", tpl_32674::<0x4, 0x141735ec0> as *const ()), // sub_141737560
    Replacement::draft(0x141737580, "kaim_141737580", tpl_22446::<0x141737dc0> as *const ()), // sub_141737580
    Replacement::draft(0x141737590, "kaim_141737590", tpl_22446::<0x1411fb140> as *const ()), // sub_141737590
    Replacement::draft(0x1417375a0, "kaim_1417375a0", tpl_22446::<0x141737f30> as *const ()), // sub_1417375a0
    Replacement::draft(0x1417375b0, "kaim_1417375b0", fn_1417375b0 as *const ()), // sub_1417375b0
    Replacement::draft(0x1417375e0, "kaim_1417375e0", fn_1417375e0 as *const ()), // sub_1417375e0
    Replacement::draft(0x1417375f0, "kaim_1417375f0", fn_1417375f0 as *const ()), // sub_1417375f0
    Replacement::draft(0x141737600, "kaim_141737600", tpl_1::<0x1426ac730> as *const ()), // sub_141737600
    Replacement::draft(0x141737610, "kaim_141737610", tpl_1::<0x1426ac718> as *const ()), // sub_141737610
    Replacement::draft(0x141737620, "kaim_141737620", tpl_1::<0x1426ac700> as *const ()), // sub_141737620
    Replacement::draft(0x141737630, "kaim_141737630", tpl_1::<0x1426ac6e8> as *const ()), // sub_141737630
    Replacement::draft(0x141737640, "kaim_141737640", tpl_1::<0x1426ac630> as *const ()), // sub_141737640
    Replacement::draft(0x141737650, "kaim_141737650", tpl_1::<0x1426ac640> as *const ()), // sub_141737650
    Replacement::draft(0x141737660, "kaim_141737660", tpl_1::<0x1426ac620> as *const ()), // sub_141737660
    Replacement::draft(0x141737670, "kaim_141737670", tpl_1::<0x1426ac698> as *const ()), // sub_141737670
    Replacement::draft(0x141737680, "kaim_141737680", tpl_1::<0x1426ac5dc> as *const ()), // sub_141737680
    Replacement::draft(0x141737690, "kaim_141737690", tpl_1::<0x1426ac5d0> as *const ()), // sub_141737690
    Replacement::draft(0x1417376a0, "kaim_1417376a0", tpl_1::<0x1426ac610> as *const ()), // sub_1417376a0
    Replacement::draft(0x1417376b0, "kaim_1417376b0", tpl_1::<0x1426ac5c0> as *const ()), // sub_1417376b0
    Replacement::draft(0x1417376c0, "kaim_1417376c0", tpl_1::<0x1426ac600> as *const ()), // sub_1417376c0
    Replacement::draft(0x1417376d0, "kaim_1417376d0", tpl_1::<0x1426ac678> as *const ()), // sub_1417376d0
    Replacement::draft(0x1417376e0, "kaim_1417376e0", tpl_1::<0x1426ac5e8> as *const ()), // sub_1417376e0
    Replacement::draft(0x1417376f0, "kaim_1417376f0", tpl_1::<0x1426ac5b4> as *const ()), // sub_1417376f0
    Replacement::draft(0x141737700, "kaim_141737700", tpl_1::<0x1426ac650> as *const ()), // sub_141737700
    Replacement::draft(0x141737710, "kaim_141737710", tpl_1::<0x1426ac660> as *const ()), // sub_141737710
    Replacement::draft(0x141737720, "kaim_141737720", fn_141737720 as *const ()), // sub_141737720
    Replacement::draft(0x141737730, "kaim_141737730", fn_141737730 as *const ()), // sub_141737730
    Replacement::draft(0x141737810, "kaim_141737810", fn_141737810 as *const ()), // sub_141737810
    Replacement::draft(0x141737a30, "kaim_141737a30", fn_141737a30 as *const ()), // sub_141737a30
    Replacement::draft(0x141737b30, "kaim_141737b30", fn_141737b30 as *const ()), // sub_141737b30
    Replacement::draft(0x141737be0, "kaim_141737be0", fn_141737be0 as *const ()), // sub_141737be0
    Replacement::draft(0x141737cb0, "kaim_141737cb0", fn_141737cb0 as *const ()), // sub_141737cb0
    Replacement::draft(0x141737dc0, "kaim_141737dc0", fn_141737dc0 as *const ()), // sub_141737dc0
    Replacement::draft(0x141737f30, "kaim_141737f30", fn_141737f30 as *const ()), // sub_141737f30
    Replacement::draft(0x141738040, "kaim_141738040", fn_141738040 as *const ()), // sub_141738040
    Replacement::draft(0x141738730, "kaim_141738730", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad2f8> as *const ()), // sub_141738730
    Replacement::draft(0x141738790, "kaim_141738790", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad330> as *const ()), // sub_141738790
    Replacement::draft(0x1417387f0, "kaim_1417387f0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad250> as *const ()), // sub_1417387f0
    Replacement::draft(0x141738850, "kaim_141738850", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad2c0> as *const ()), // sub_141738850
    Replacement::draft(0x1417388b0, "kaim_1417388b0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad1a8> as *const ()), // sub_1417388b0
    Replacement::draft(0x141738910, "kaim_141738910", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad1e0> as *const ()), // sub_141738910
    Replacement::draft(0x141738970, "kaim_141738970", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad218> as *const ()), // sub_141738970
    Replacement::draft(0x1417389d0, "kaim_1417389d0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad288> as *const ()), // sub_1417389d0
    Replacement::draft(0x141738a30, "kaim_141738a30", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad138, 0x8> as *const ()), // sub_141738a30
    Replacement::draft(0x141738a70, "kaim_141738a70", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad170, 0x8> as *const ()), // sub_141738a70
    Replacement::draft(0x141738ab0, "kaim_141738ab0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad090, 0x8> as *const ()), // sub_141738ab0
    Replacement::draft(0x141738af0, "kaim_141738af0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad100, 0x8> as *const ()), // sub_141738af0
    Replacement::draft(0x141738b30, "kaim_141738b30", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426acfe8, 0x8> as *const ()), // sub_141738b30
    Replacement::draft(0x141738b70, "kaim_141738b70", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad020, 0x8> as *const ()), // sub_141738b70
    Replacement::draft(0x141738bb0, "kaim_141738bb0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad058, 0x8> as *const ()), // sub_141738bb0
    Replacement::draft(0x141738bf0, "kaim_141738bf0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad0c8, 0x8> as *const ()), // sub_141738bf0
    Replacement::draft(0x141738c30, "kaim_141738c30", tpl_22446::<0x141738e30> as *const ()), // sub_141738c30
    Replacement::draft(0x141738c40, "kaim_141738c40", tpl_22446::<0x141738f50> as *const ()), // sub_141738c40
    Replacement::draft(0x141738c50, "kaim_141738c50", tpl_22446::<0x141738ff0> as *const ()), // sub_141738c50
    Replacement::draft(0x141738c60, "kaim_141738c60", fn_141738c60 as *const ()), // sub_141738c60
    Replacement::draft(0x141738cb0, "kaim_141738cb0", fn_141738cb0 as *const ()), // sub_141738cb0
    Replacement::draft(0x141738d20, "kaim_141738d20", tpl_22446::<0x1417391a0> as *const ()), // sub_141738d20
    Replacement::draft(0x141738d30, "kaim_141738d30", fn_141738d30 as *const ()), // sub_141738d30
    Replacement::draft(0x141738d40, "kaim_141738d40", fn_141738d40 as *const ()), // sub_141738d40
    Replacement::draft(0x141738d50, "kaim_141738d50", fn_141738d50 as *const ()), // sub_141738d50
    Replacement::draft(0x141738d60, "kaim_141738d60", fn_141738d60 as *const ()), // sub_141738d60
];
