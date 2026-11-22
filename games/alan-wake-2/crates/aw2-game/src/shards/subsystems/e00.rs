#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x140001000, "unknown_140001000", tpl_0::<0x1426c6d60, 0x142645320> as *const ()), // sub_140001000
    Replacement::draft(0x140001010, "unknown_140001010", tpl_1::<0x140001000> as *const ()), // sub_140001010
    Replacement::draft(0x140001020, "csingletoncriticalsection_140001020", fn_140001020 as *const ()), // CSingletonCriticalSection::CSingletonCriticalSection::vfunc_0
    Replacement::draft(0x140001050, "unknown_140001050", fn_140001050 as *const ()), // sub_140001050
    Replacement::draft(0x1400011d0, "curl_1400011d0", fn_1400011d0 as *const ()), // sub_1400011d0
    Replacement::draft(0x1400012a0, "unknown_1400012a0", fn_1400012a0 as *const ()), // sub_1400012a0
    Replacement::draft(0x1400013a0, "unknown_1400013a0", fn_1400013a0 as *const ()), // sub_1400013a0
    Replacement::draft(0x140001430, "unknown_140001430", fn_140001430 as *const ()), // sub_140001430
    Replacement::draft(0x140001500, "unknown_140001500", fn_140001500 as *const ()), // sub_140001500
    Replacement::draft(0x1400015e0, "unknown_1400015e0", fn_1400015e0 as *const ()), // sub_1400015e0
    Replacement::draft(0x1400016d0, "unknown_1400016d0", fn_1400016d0 as *const ()), // sub_1400016d0
    Replacement::draft(0x1400017a0, "unknown_1400017a0", fn_1400017a0 as *const ()), // sub_1400017a0
    Replacement::draft(0x140001870, "unknown_140001870", fn_140001870 as *const ()), // sub_140001870
    Replacement::draft(0x140001940, "unknown_140001940", fn_140001940 as *const ()), // sub_140001940
    Replacement::draft(0x140001a60, "unknown_140001a60", fn_140001a60 as *const ()), // sub_140001a60
    Replacement::draft(0x140001cd0, "unknown_140001cd0", fn_140001cd0 as *const ()), // sub_140001cd0
    Replacement::draft(0x140001dd0, "unknown_140001dd0", fn_140001dd0 as *const ()), // sub_140001dd0
    Replacement::draft(0x140001e30, "unknown_140001e30", fn_140001e30 as *const ()), // sub_140001e30
    Replacement::draft(0x140001e90, "unknown_140001e90", fn_140001e90 as *const ()), // sub_140001e90
    Replacement::draft(0x140001f80, "unknown_140001f80", fn_140001f80 as *const ()), // sub_140001f80
    Replacement::draft(0x140001ff0, "unknown_140001ff0", fn_140001ff0 as *const ()), // sub_140001ff0
    Replacement::draft(0x140002160, "unknown_140002160", fn_140002160 as *const ()), // sub_140002160
    Replacement::draft(0x1400022c0, "std_1400022c0", fn_1400022c0 as *const ()), // sub_1400022c0
    Replacement::draft(0x140002340, "std_140002340", fn_140002340 as *const ()), // sub_140002340
    Replacement::draft(0x1400023d0, "std_1400023d0", fn_1400023d0 as *const ()), // sub_1400023d0
    Replacement::draft(0x140002460, "unknown_140002460", fn_140002460 as *const ()), // sub_140002460
    Replacement::draft(0x140002530, "unknown_140002530", fn_140002530 as *const ()), // sub_140002530
    Replacement::draft(0x140002600, "unknown_140002600", fn_140002600 as *const ()), // sub_140002600
    Replacement::draft(0x140002970, "unknown_140002970", fn_140002970 as *const ()), // sub_140002970
    Replacement::draft(0x140002a80, "unknown_140002a80", fn_140002a80 as *const ()), // sub_140002a80
    Replacement::draft(0x140002bd0, "unknown_140002bd0", fn_140002bd0 as *const ()), // sub_140002bd0
    Replacement::draft(0x140002d90, "unknown_140002d90", fn_140002d90 as *const ()), // sub_140002d90
    Replacement::draft(0x140002e20, "unknown_140002e20", fn_140002e20 as *const ()), // sub_140002e20
    Replacement::draft(0x140002ec0, "unknown_140002ec0", fn_140002ec0 as *const ()), // sub_140002ec0
    Replacement::draft(0x140003000, "unknown_140003000", fn_140003000 as *const ()), // sub_140003000
    Replacement::draft(0x140003230, "unknown_140003230", fn_140003230 as *const ()), // sub_140003230
    Replacement::draft(0x140003420, "unknown_140003420", fn_140003420 as *const ()), // sub_140003420
    Replacement::draft(0x140003550, "unknown_140003550", fn_140003550 as *const ()), // sub_140003550
    Replacement::draft(0x140003740, "unknown_140003740", fn_140003740 as *const ()), // sub_140003740
    Replacement::draft(0x1400039f0, "unknown_1400039f0", fn_1400039f0 as *const ()), // sub_1400039f0
    Replacement::draft(0x140003c00, "unknown_140003c00", fn_140003c00 as *const ()), // sub_140003c00
    Replacement::draft(0x140003e10, "unknown_140003e10", fn_140003e10 as *const ()), // sub_140003e10
    Replacement::draft(0x140003eb0, "unknown_140003eb0", fn_140003eb0 as *const ()), // sub_140003eb0
    Replacement::draft(0x140003fc0, "unknown_140003fc0", fn_140003fc0 as *const ()), // sub_140003fc0
    Replacement::draft(0x140004360, "unknown_140004360", fn_140004360 as *const ()), // sub_140004360
    Replacement::draft(0x140004520, "unknown_140004520", fn_140004520 as *const ()), // sub_140004520
    Replacement::draft(0x140004890, "unknown_140004890", fn_140004890 as *const ()), // sub_140004890
    Replacement::draft(0x140004b90, "unknown_140004b90", fn_140004b90 as *const ()), // sub_140004b90
    Replacement::draft(0x140004dc0, "unknown_140004dc0", fn_140004dc0 as *const ()), // sub_140004dc0
    Replacement::draft(0x140004ed0, "unknown_140004ed0", tpl_23::<0x143a09848, 0x143a0984b, 0x1426cdcb0, 0x142409b50, 0x1425d2230, 0x141834d18> as *const ()), // sub_140004ed0
    Replacement::draft(0x140004f30, "unknown_140004f30", tpl_23::<0x143a09890, 0x143a09893, 0x1426cdcb0, 0x142409b50, 0x1425d2240, 0x141834d18> as *const ()), // sub_140004f30
    Replacement::draft(0x140005120, "unknown_140005120", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438431a8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d2280, 0x141834d18> as *const ()), // sub_140005120
    Replacement::draft(0x140005540, "unknown_140005540", fn_140005540 as *const ()), // sub_140005540
    Replacement::draft(0x140005710, "unknown_140005710", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a0a060, 0x143a0a070> as *const ()), // sub_140005710
    Replacement::draft(0x140005780, "unknown_140005780", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a09f78, 0x143a09f88> as *const ()), // sub_140005780
    Replacement::draft(0x1400057f0, "unknown_1400057f0", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a0a4c0, 0x143a0a4d0> as *const ()), // sub_1400057f0
    Replacement::draft(0x140005860, "unknown_140005860", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a09ff8, 0x143a0a008> as *const ()), // sub_140005860
    Replacement::draft(0x1400058d0, "unknown_1400058d0", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a09bb0, 0x143a09bc0> as *const ()), // sub_1400058d0
    Replacement::draft(0x140005940, "unknown_140005940", tpl_31::<0x1430782fc, 0x143078300, 0x143a09e98, 0x143a09ea8> as *const ()), // sub_140005940
    Replacement::draft(0x1400059b0, "unknown_1400059b0", tpl_32::<0x143078308, 0x143078313, 0x143a09b00, 0x143a09b10> as *const ()), // sub_1400059b0
    Replacement::draft(0x140005a20, "unknown_140005a20", tpl_23::<0x143a09b18, 0x143a09b1b, 0x1426cdcb0, 0x142409b50, 0x1425d22f0, 0x141834d18> as *const ()), // sub_140005a20
    Replacement::draft(0x140005a80, "unknown_140005a80", tpl_23::<0x143a09b60, 0x143a09b63, 0x1426cdcb0, 0x142409b50, 0x1425d2300, 0x141834d18> as *const ()), // sub_140005a80
    Replacement::draft(0x140005c70, "unknown_140005c70", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843238, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d2340, 0x141834d18> as *const ()), // sub_140005c70
    Replacement::draft(0x140006110, "unknown_140006110", tpl_33::<0x143428dc0, 0x143a0a310> as *const ()), // sub_140006110
    Replacement::draft(0x140006130, "unknown_140006130", tpl_33::<0x143428da0, 0x143a0a160> as *const ()), // sub_140006130
    Replacement::draft(0x140006150, "unknown_140006150", tpl_34::<0x143a09ce0, 0x143a09ce3, 0x143078668, 0x142409b50, 0x1425d23b0, 0x141834d18> as *const ()), // sub_140006150
    Replacement::draft(0x1400061b0, "unknown_1400061b0", tpl_23::<0x143a0a3f8, 0x143a0a3fb, 0x1426cdcb0, 0x142409b50, 0x1425d23c0, 0x141834d18> as *const ()), // sub_1400061b0
    Replacement::draft(0x140006210, "unknown_140006210", tpl_35::<0x143a0a0e0, 0x143a0a0e3, 0x143078678, 0x142409b50, 0x1425d23d0, 0x141834d18> as *const ()), // sub_140006210
    Replacement::draft(0x140006270, "unknown_140006270", tpl_36::<0x143a0a380, 0x143a0a383, 0x143078680, 0x142409b50, 0x1425d23e0, 0x141834d18> as *const ()), // sub_140006270
    Replacement::draft(0x1400062d0, "unknown_1400062d0", tpl_37::<0x143a09f10, 0x143a09f13, 0x143078690, 0x142409b50, 0x1425d23f0, 0x141834d18> as *const ()), // sub_1400062d0
    Replacement::draft(0x140006330, "unknown_140006330", tpl_38::<0x143a09da8, 0x143a09dab, 0x1430786a0, 0x142409b50, 0x1425d2400, 0x141834d18> as *const ()), // sub_140006330
    Replacement::draft(0x140006390, "unknown_140006390", tpl_39::<0x143a0a420, 0x143a0a423, 0x1430786b0, 0x142409b50, 0x1425d2410, 0x141834d18> as *const ()), // sub_140006390
    Replacement::draft(0x1400063f0, "unknown_1400063f0", tpl_37::<0x143a09c90, 0x143a09c93, 0x1430786b8, 0x142409b50, 0x1425d2420, 0x141834d18> as *const ()), // sub_1400063f0
    Replacement::draft(0x140006450, "unknown_140006450", tpl_40::<0x143a0a550, 0x143a0a553, 0x1430786c4, 0x142409b50, 0x1425d2430, 0x141834d18> as *const ()), // sub_140006450
    Replacement::draft(0x1400064b0, "unknown_1400064b0", tpl_41::<0x143a09e70, 0x143a09e73, 0x1430786d0, 0x142409b50, 0x1425d2440, 0x141834d18> as *const ()), // sub_1400064b0
    Replacement::draft(0x140006510, "unknown_140006510", tpl_42::<0x143a0a470, 0x143a0a473, 0x1430786e0, 0x142409b50, 0x1425d2450, 0x141834d18> as *const ()), // sub_140006510
    Replacement::draft(0x140006570, "unknown_140006570", tpl_34::<0x143a0a4d8, 0x143a0a4db, 0x1430786f8, 0x142409b50, 0x1425d2460, 0x141834d18> as *const ()), // sub_140006570
    Replacement::draft(0x1400065d0, "unknown_1400065d0", tpl_34::<0x143a09cb8, 0x143a09cbb, 0x143078708, 0x142409b50, 0x1425d2470, 0x141834d18> as *const ()), // sub_1400065d0
    Replacement::draft(0x140006630, "unknown_140006630", tpl_41::<0x143a09dd0, 0x143a09dd3, 0x143078718, 0x142409b50, 0x1425d2480, 0x141834d18> as *const ()), // sub_140006630
    Replacement::draft(0x140006690, "unknown_140006690", tpl_43::<0x143a09f50, 0x143a09f53, 0x143078728, 0x142409b50, 0x1425d2490, 0x141834d18> as *const ()), // sub_140006690
    Replacement::draft(0x1400066f0, "unknown_1400066f0", tpl_44::<0x143a09928, 0x143a0992b, 0x143078748, 0x142409b50, 0x1425d24a0, 0x141834d18> as *const ()), // sub_1400066f0
    Replacement::draft(0x140006750, "unknown_140006750", tpl_39::<0x143a09ec0, 0x143a09ec3, 0x143078760, 0x142409b50, 0x1425d24b0, 0x141834d18> as *const ()), // sub_140006750
    Replacement::draft(0x1400067b0, "unknown_1400067b0", tpl_45::<0x143a09d80, 0x143a09d83, 0x143078768, 0x142409b50, 0x1425d24c0, 0x141834d18> as *const ()), // sub_1400067b0
    Replacement::draft(0x140006810, "unknown_140006810", tpl_41::<0x143a0a330, 0x143a0a333, 0x143078778, 0x142409b50, 0x1425d24d0, 0x141834d18> as *const ()), // sub_140006810
    Replacement::draft(0x140006870, "unknown_140006870", tpl_37::<0x143a09950, 0x143a09953, 0x143078788, 0x142409b50, 0x1425d24e0, 0x141834d18> as *const ()), // sub_140006870
    Replacement::draft(0x1400068d0, "unknown_1400068d0", tpl_46::<0x143a0a1a8, 0x143a0a1ab, 0x143078798, 0x142409b50, 0x1425d24f0, 0x141834d18> as *const ()), // sub_1400068d0
    Replacement::draft(0x140006930, "unknown_140006930", tpl_47::<0x143a09bf0, 0x143a09bf3, 0x1430787b0, 0x142409b50, 0x1425d2500, 0x141834d18> as *const ()), // sub_140006930
    Replacement::draft(0x140006990, "unknown_140006990", tpl_38::<0x143a0a248, 0x143a0a24b, 0x1430787d0, 0x142409b50, 0x1425d2510, 0x141834d18> as *const ()), // sub_140006990
    Replacement::draft(0x1400069f0, "unknown_1400069f0", tpl_48::<0x143a0a448, 0x143a0a44b, 0x1430787e0, 0x142409b50, 0x1425d2520, 0x141834d18> as *const ()), // sub_1400069f0
    Replacement::draft(0x140006a50, "unknown_140006a50", tpl_49::<0x143a0a0b8, 0x143a0a0bb, 0x1430787f8, 0x142409b50, 0x1425d2530, 0x141834d18> as *const ()), // sub_140006a50
    Replacement::draft(0x140006ab0, "unknown_140006ab0", tpl_44::<0x143a0a3d0, 0x143a0a3d3, 0x143078808, 0x142409b50, 0x1425d2540, 0x141834d18> as *const ()), // sub_140006ab0
    Replacement::draft(0x140006b10, "unknown_140006b10", tpl_42::<0x143a09900, 0x143a09903, 0x143078820, 0x142409b50, 0x1425d2550, 0x141834d18> as *const ()), // sub_140006b10
    Replacement::draft(0x140006b70, "unknown_140006b70", tpl_50::<0x143a0a2c0, 0x143a0a2c3, 0x143078838, 0x142409b50, 0x1425d2560, 0x141834d18> as *const ()), // sub_140006b70
    Replacement::draft(0x140006bd0, "unknown_140006bd0", tpl_51::<0x143078928, 0x143078938, 0x143a09ac0, 0x143078944, 0x1430782bc, 0x1430782b4, 0x1401df440, 0x1425d2570, 0x141834d18> as *const ()), // sub_140006bd0
    Replacement::draft(0x140006c60, "unknown_140006c60", tpl_52::<0x143a09a90, 0x143a09a93, 0x143a0a550, 0x143a0a554, 0x143a0a553, 0x143a0a558, 0x142409b50, 0x1425d25a0, 0x141834d18> as *const ()), // sub_140006c60
    Replacement::draft(0x140006d20, "unknown_140006d20", tpl_38::<0x143a0a108, 0x143a0a10b, 0x143078950, 0x142409b50, 0x1425d25b0, 0x141834d18> as *const ()), // sub_140006d20
    Replacement::draft(0x140006d80, "unknown_140006d80", tpl_53::<0x143a09bc8, 0x143a09bcb, 0x143078960, 0x142409b50, 0x1425d25c0, 0x141834d18> as *const ()), // sub_140006d80
    Replacement::draft(0x140006de0, "unknown_140006de0", tpl_45::<0x143a09a18, 0x143a09a1b, 0x143078980, 0x142409b50, 0x1425d25d0, 0x141834d18> as *const ()), // sub_140006de0
    Replacement::draft(0x140006e40, "unknown_140006e40", tpl_35::<0x143a09a68, 0x143a09a6b, 0x14307898c, 0x142409b50, 0x1425d25e0, 0x141834d18> as *const ()), // sub_140006e40
    Replacement::draft(0x140006ea0, "unknown_140006ea0", tpl_42::<0x143a0a498, 0x143a0a49b, 0x1430786e0, 0x142409b50, 0x1425d25f0, 0x141834d18> as *const ()), // sub_140006ea0
    Replacement::draft(0x140006f00, "unknown_140006f00", tpl_38::<0x143a0a3a8, 0x143a0a3ab, 0x143078998, 0x142409b50, 0x1425d2600, 0x141834d18> as *const ()), // sub_140006f00
    Replacement::draft(0x140006f60, "unknown_140006f60", tpl_38::<0x143a09e20, 0x143a09e23, 0x1430789a8, 0x142409b50, 0x1425d2610, 0x141834d18> as *const ()), // sub_140006f60
    Replacement::draft(0x140006fc0, "unknown_140006fc0", tpl_41::<0x143a09fd0, 0x143a09fd3, 0x1430789b8, 0x142409b50, 0x1425d2620, 0x141834d18> as *const ()), // sub_140006fc0
    Replacement::draft(0x140007020, "unknown_140007020", tpl_34::<0x143a0a500, 0x143a0a503, 0x1430789c8, 0x142409b50, 0x1425d2630, 0x141834d18> as *const ()), // sub_140007020
    Replacement::draft(0x140007080, "unknown_140007080", tpl_39::<0x143a09a40, 0x143a09a43, 0x1430789d8, 0x142409b50, 0x1425d2640, 0x141834d18> as *const ()), // sub_140007080
    Replacement::draft(0x1400070e0, "unknown_1400070e0", tpl_41::<0x143a09df8, 0x143a09dfb, 0x1430789e0, 0x142409b50, 0x1425d2650, 0x141834d18> as *const ()), // sub_1400070e0
    Replacement::draft(0x140007140, "unknown_140007140", tpl_34::<0x143a0a130, 0x143a0a133, 0x1430789f0, 0x142409b50, 0x1425d2660, 0x141834d18> as *const ()), // sub_140007140
    Replacement::draft(0x1400071a0, "unknown_1400071a0", tpl_48::<0x143a09d08, 0x143a09d0b, 0x143078a00, 0x142409b50, 0x1425d2670, 0x141834d18> as *const ()), // sub_1400071a0
    Replacement::draft(0x140007200, "unknown_140007200", tpl_46::<0x143a0a270, 0x143a0a273, 0x143078a18, 0x142409b50, 0x1425d2680, 0x141834d18> as *const ()), // sub_140007200
    Replacement::draft(0x140007260, "unknown_140007260", tpl_47::<0x143a0a1d0, 0x143a0a1d3, 0x143078a30, 0x142409b50, 0x1425d2690, 0x141834d18> as *const ()), // sub_140007260
    Replacement::draft(0x1400072c0, "unknown_1400072c0", tpl_37::<0x143a09c40, 0x143a09c43, 0x143078a50, 0x142409b50, 0x1425d26a0, 0x141834d18> as *const ()), // sub_1400072c0
    Replacement::draft(0x140007320, "unknown_140007320", tpl_45::<0x143a09ee8, 0x143a09eeb, 0x143078a60, 0x142409b50, 0x1425d26b0, 0x141834d18> as *const ()), // sub_140007320
    Replacement::draft(0x140007380, "unknown_140007380", tpl_54::<0x143a0a038, 0x143a0a03b, 0x143078a70, 0x142409b50, 0x1425d26c0, 0x141834d18> as *const ()), // sub_140007380
    Replacement::draft(0x1400073e0, "unknown_1400073e0", tpl_40::<0x143a09c18, 0x143a09c1b, 0x143078a88, 0x142409b50, 0x1425d26d0, 0x141834d18> as *const ()), // sub_1400073e0
    Replacement::draft(0x140007440, "unknown_140007440", tpl_50::<0x143a09c68, 0x143a09c6b, 0x143078a90, 0x142409b50, 0x1425d26e0, 0x141834d18> as *const ()), // sub_140007440
    Replacement::draft(0x1400074a0, "unknown_1400074a0", tpl_41::<0x143a09e48, 0x143a09e4b, 0x143078aa8, 0x142409b50, 0x1425d26f0, 0x141834d18> as *const ()), // sub_1400074a0
    Replacement::draft(0x140007500, "unknown_140007500", tpl_55::<0x143a0a1f8, 0x143a0a1fb, 0x143078ab8, 0x142409b50, 0x1425d2700, 0x141834d18> as *const ()), // sub_140007500
    Replacement::draft(0x140007560, "unknown_140007560", tpl_41::<0x143a0a298, 0x143a0a29b, 0x143078ac8, 0x142409b50, 0x1425d2710, 0x141834d18> as *const ()), // sub_140007560
    Replacement::draft(0x1400075c0, "unknown_1400075c0", tpl_49::<0x143a0a358, 0x143a0a35b, 0x143078ad8, 0x142409b50, 0x1425d2720, 0x141834d18> as *const ()), // sub_1400075c0
    Replacement::draft(0x140007620, "unknown_140007620", tpl_40::<0x143a099f0, 0x143a099f3, 0x143078ae4, 0x142409b50, 0x1425d2730, 0x141834d18> as *const ()), // sub_140007620
    Replacement::draft(0x140007680, "unknown_140007680", tpl_44::<0x143a09b88, 0x143a09b8b, 0x143078af0, 0x142409b50, 0x1425d2740, 0x141834d18> as *const ()), // sub_140007680
    Replacement::draft(0x1400076e0, "unknown_1400076e0", tpl_46::<0x143a099c8, 0x143a099cb, 0x143078b08, 0x142409b50, 0x1425d2750, 0x141834d18> as *const ()), // sub_1400076e0
    Replacement::draft(0x140007740, "unknown_140007740", tpl_55::<0x143a0a220, 0x143a0a223, 0x143078b20, 0x142409b50, 0x1425d2760, 0x141834d18> as *const ()), // sub_140007740
    Replacement::draft(0x1400077a0, "unknown_1400077a0", tpl_46::<0x143a0a090, 0x143a0a093, 0x143078b30, 0x142409b50, 0x1425d2770, 0x141834d18> as *const ()), // sub_1400077a0
    Replacement::draft(0x140007800, "unknown_140007800", tpl_50::<0x143a09d30, 0x143a09d33, 0x143078b48, 0x142409b50, 0x1425d2780, 0x141834d18> as *const ()), // sub_140007800
    Replacement::draft(0x140007860, "unknown_140007860", tpl_42::<0x143a09978, 0x143a0997b, 0x143078b60, 0x142409b50, 0x1425d2790, 0x141834d18> as *const ()), // sub_140007860
    Replacement::draft(0x1400078c0, "unknown_1400078c0", tpl_38::<0x143a0a180, 0x143a0a183, 0x143078b78, 0x142409b50, 0x1425d27a0, 0x141834d18> as *const ()), // sub_1400078c0
    Replacement::draft(0x140007920, "unknown_140007920", tpl_34::<0x143a0a528, 0x143a0a52b, 0x143078b88, 0x142409b50, 0x1425d27b0, 0x141834d18> as *const ()), // sub_140007920
    Replacement::draft(0x140007980, "unknown_140007980", tpl_41::<0x143a0a2e8, 0x143a0a2eb, 0x143078b98, 0x142409b50, 0x1425d27c0, 0x141834d18> as *const ()), // sub_140007980
    Replacement::draft(0x1400079e0, "unknown_1400079e0", tpl_36::<0x143a0a010, 0x143a0a013, 0x143078ba8, 0x142409b50, 0x1425d27d0, 0x141834d18> as *const ()), // sub_1400079e0
    Replacement::draft(0x140007a40, "unknown_140007a40", tpl_34::<0x143a099a0, 0x143a099a3, 0x143078bb8, 0x142409b50, 0x1425d27e0, 0x141834d18> as *const ()), // sub_140007a40
    Replacement::draft(0x140007aa0, "unknown_140007aa0", tpl_23::<0x143a0a578, 0x143a0a57b, 0x1426cdcb0, 0x142409b50, 0x1425d27f0, 0x141834d18> as *const ()), // sub_140007aa0
    Replacement::draft(0x140007b00, "unknown_140007b00", tpl_23::<0x143a0a5c0, 0x143a0a5c3, 0x1426cdcb0, 0x142409b50, 0x1425d2800, 0x141834d18> as *const ()), // sub_140007b00
    Replacement::draft(0x140007cf0, "unknown_140007cf0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438432c8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d2840, 0x141834d18> as *const ()), // sub_140007cf0
    Replacement::draft(0x1400083c0, "unknown_1400083c0", tpl_23::<0x143a0a7e8, 0x143a0a7eb, 0x1426cdcb0, 0x142409b50, 0x1425d28b0, 0x141834d18> as *const ()), // sub_1400083c0
    Replacement::draft(0x140008420, "unknown_140008420", tpl_23::<0x143a0a830, 0x143a0a833, 0x1426cdcb0, 0x142409b50, 0x1425d28c0, 0x141834d18> as *const ()), // sub_140008420
    Replacement::draft(0x140008610, "unknown_140008610", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438433b0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d2900, 0x141834d18> as *const ()), // sub_140008610
    Replacement::draft(0x140008a10, "unknown_140008a10", tpl_34::<0x143a0a998, 0x143a0a99b, 0x143078668, 0x142409b50, 0x1425d2970, 0x141834d18> as *const ()), // sub_140008a10
    Replacement::draft(0x140008a70, "unknown_140008a70", tpl_23::<0x143a0b010, 0x143a0b013, 0x1426cdcb0, 0x142409b50, 0x1425d2980, 0x141834d18> as *const ()), // sub_140008a70
    Replacement::draft(0x140008ad0, "unknown_140008ad0", tpl_35::<0x143a0ad40, 0x143a0ad43, 0x143078678, 0x142409b50, 0x1425d2990, 0x141834d18> as *const ()), // sub_140008ad0
    Replacement::draft(0x140008b30, "unknown_140008b30", tpl_36::<0x143a0af98, 0x143a0af9b, 0x143078680, 0x142409b50, 0x1425d29a0, 0x141834d18> as *const ()), // sub_140008b30
    Replacement::draft(0x140008b90, "unknown_140008b90", tpl_37::<0x143a0abb0, 0x143a0abb3, 0x143078690, 0x142409b50, 0x1425d29b0, 0x141834d18> as *const ()), // sub_140008b90
    Replacement::draft(0x140008bf0, "unknown_140008bf0", tpl_38::<0x143a0aa58, 0x143a0aa5b, 0x1430786a0, 0x142409b50, 0x1425d29c0, 0x141834d18> as *const ()), // sub_140008bf0
    Replacement::draft(0x140008c50, "unknown_140008c50", tpl_39::<0x143a0b038, 0x143a0b03b, 0x1430786b0, 0x142409b50, 0x1425d29d0, 0x141834d18> as *const ()), // sub_140008c50
    Replacement::draft(0x140008cb0, "unknown_140008cb0", tpl_37::<0x143a0a948, 0x143a0a94b, 0x1430786b8, 0x142409b50, 0x1425d29e0, 0x141834d18> as *const ()), // sub_140008cb0
    Replacement::draft(0x140008d10, "unknown_140008d10", tpl_40::<0x143a0b150, 0x143a0b153, 0x1430786c4, 0x142409b50, 0x1425d29f0, 0x141834d18> as *const ()), // sub_140008d10
    Replacement::draft(0x140008d70, "unknown_140008d70", tpl_41::<0x143a0ab20, 0x143a0ab23, 0x1430786d0, 0x142409b50, 0x1425d2a00, 0x141834d18> as *const ()), // sub_140008d70
    Replacement::draft(0x140008dd0, "unknown_140008dd0", tpl_42::<0x143a0b088, 0x143a0b08b, 0x1430786e0, 0x142409b50, 0x1425d2a10, 0x141834d18> as *const ()), // sub_140008dd0
    Replacement::draft(0x140008e30, "unknown_140008e30", tpl_34::<0x143a0b0d8, 0x143a0b0db, 0x1430786f8, 0x142409b50, 0x1425d2a20, 0x141834d18> as *const ()), // sub_140008e30
    Replacement::draft(0x140008e90, "unknown_140008e90", tpl_34::<0x143a0a970, 0x143a0a973, 0x143078708, 0x142409b50, 0x1425d2a30, 0x141834d18> as *const ()), // sub_140008e90
    Replacement::draft(0x140008ef0, "unknown_140008ef0", tpl_41::<0x143a0aa80, 0x143a0aa83, 0x143078718, 0x142409b50, 0x1425d2a40, 0x141834d18> as *const ()), // sub_140008ef0
    Replacement::draft(0x140008f50, "unknown_140008f50", tpl_43::<0x143a0abf0, 0x143a0abf3, 0x143078728, 0x142409b50, 0x1425d2a50, 0x141834d18> as *const ()), // sub_140008f50
    Replacement::draft(0x140008fb0, "unknown_140008fb0", tpl_44::<0x143a0a658, 0x143a0a65b, 0x143078748, 0x142409b50, 0x1425d2a60, 0x141834d18> as *const ()), // sub_140008fb0
    Replacement::draft(0x140009010, "unknown_140009010", tpl_39::<0x143a0ab60, 0x143a0ab63, 0x143078760, 0x142409b50, 0x1425d2a70, 0x141834d18> as *const ()), // sub_140009010
    Replacement::draft(0x140009070, "unknown_140009070", tpl_45::<0x143a0aa30, 0x143a0aa33, 0x143078768, 0x142409b50, 0x1425d2a80, 0x141834d18> as *const ()), // sub_140009070
    Replacement::draft(0x1400090d0, "unknown_1400090d0", tpl_41::<0x143a0af48, 0x143a0af4b, 0x143078778, 0x142409b50, 0x1425d2a90, 0x141834d18> as *const ()), // sub_1400090d0
    Replacement::draft(0x140009130, "unknown_140009130", tpl_37::<0x143a0a680, 0x143a0a683, 0x143078788, 0x142409b50, 0x1425d2aa0, 0x141834d18> as *const ()), // sub_140009130
    Replacement::draft(0x140009190, "unknown_140009190", tpl_46::<0x143a0ade0, 0x143a0ade3, 0x143078798, 0x142409b50, 0x1425d2ab0, 0x141834d18> as *const ()), // sub_140009190
    Replacement::draft(0x1400091f0, "unknown_1400091f0", tpl_47::<0x143a0a8a8, 0x143a0a8ab, 0x1430787b0, 0x142409b50, 0x1425d2ac0, 0x141834d18> as *const ()), // sub_1400091f0
    Replacement::draft(0x140009250, "unknown_140009250", tpl_38::<0x143a0ae80, 0x143a0ae83, 0x1430787d0, 0x142409b50, 0x1425d2ad0, 0x141834d18> as *const ()), // sub_140009250
    Replacement::draft(0x1400092b0, "unknown_1400092b0", tpl_48::<0x143a0b060, 0x143a0b063, 0x1430787e0, 0x142409b50, 0x1425d2ae0, 0x141834d18> as *const ()), // sub_1400092b0
    Replacement::draft(0x140009310, "unknown_140009310", tpl_49::<0x143a0ad18, 0x143a0ad1b, 0x1430787f8, 0x142409b50, 0x1425d2af0, 0x141834d18> as *const ()), // sub_140009310
    Replacement::draft(0x140009370, "unknown_140009370", tpl_44::<0x143a0afe8, 0x143a0afeb, 0x143078808, 0x142409b50, 0x1425d2b00, 0x141834d18> as *const ()), // sub_140009370
    Replacement::draft(0x1400093d0, "unknown_1400093d0", tpl_42::<0x143a0a630, 0x143a0a633, 0x143078820, 0x142409b50, 0x1425d2b10, 0x141834d18> as *const ()), // sub_1400093d0
    Replacement::draft(0x140009430, "unknown_140009430", tpl_50::<0x143a0aef8, 0x143a0aefb, 0x143078838, 0x142409b50, 0x1425d2b20, 0x141834d18> as *const ()), // sub_140009430
    Replacement::draft(0x140009490, "unknown_140009490", tpl_52::<0x143a0a7c0, 0x143a0a7c3, 0x143a0b150, 0x143a0b154, 0x143a0b153, 0x143a0b158, 0x142409b50, 0x1425d2b30, 0x141834d18> as *const ()), // sub_140009490
    Replacement::draft(0x140009550, "unknown_140009550", tpl_38::<0x143a0ad68, 0x143a0ad6b, 0x143078950, 0x142409b50, 0x1425d2b40, 0x141834d18> as *const ()), // sub_140009550
    Replacement::draft(0x1400095b0, "unknown_1400095b0", tpl_53::<0x143a0a880, 0x143a0a883, 0x143078960, 0x142409b50, 0x1425d2b50, 0x141834d18> as *const ()), // sub_1400095b0
    Replacement::draft(0x140009610, "unknown_140009610", tpl_45::<0x143a0a748, 0x143a0a74b, 0x143078980, 0x142409b50, 0x1425d2b60, 0x141834d18> as *const ()), // sub_140009610
    Replacement::draft(0x140009670, "unknown_140009670", tpl_35::<0x143a0a798, 0x143a0a79b, 0x14307898c, 0x142409b50, 0x1425d2b70, 0x141834d18> as *const ()), // sub_140009670
    Replacement::draft(0x1400096d0, "unknown_1400096d0", tpl_42::<0x143a0b0b0, 0x143a0b0b3, 0x1430786e0, 0x142409b50, 0x1425d2b80, 0x141834d18> as *const ()), // sub_1400096d0
    Replacement::draft(0x140009730, "unknown_140009730", tpl_38::<0x143a0afc0, 0x143a0afc3, 0x143078998, 0x142409b50, 0x1425d2b90, 0x141834d18> as *const ()), // sub_140009730
    Replacement::draft(0x140009790, "unknown_140009790", tpl_38::<0x143a0aad0, 0x143a0aad3, 0x1430789a8, 0x142409b50, 0x1425d2ba0, 0x141834d18> as *const ()), // sub_140009790
    Replacement::draft(0x1400097f0, "unknown_1400097f0", tpl_41::<0x143a0ac60, 0x143a0ac63, 0x1430789b8, 0x142409b50, 0x1425d2bb0, 0x141834d18> as *const ()), // sub_1400097f0
    Replacement::draft(0x140009850, "unknown_140009850", tpl_34::<0x143a0b100, 0x143a0b103, 0x1430789c8, 0x142409b50, 0x1425d2bc0, 0x141834d18> as *const ()), // sub_140009850
    Replacement::draft(0x1400098b0, "unknown_1400098b0", tpl_39::<0x143a0a770, 0x143a0a773, 0x1430789d8, 0x142409b50, 0x1425d2bd0, 0x141834d18> as *const ()), // sub_1400098b0
    Replacement::draft(0x140009910, "unknown_140009910", tpl_41::<0x143a0aaa8, 0x143a0aaab, 0x1430789e0, 0x142409b50, 0x1425d2be0, 0x141834d18> as *const ()), // sub_140009910
    Replacement::draft(0x140009970, "unknown_140009970", tpl_34::<0x143a0ad90, 0x143a0ad93, 0x1430789f0, 0x142409b50, 0x1425d2bf0, 0x141834d18> as *const ()), // sub_140009970
    Replacement::draft(0x1400099d0, "unknown_1400099d0", tpl_48::<0x143a0a9c0, 0x143a0a9c3, 0x143078a00, 0x142409b50, 0x1425d2c00, 0x141834d18> as *const ()), // sub_1400099d0
    Replacement::draft(0x140009a30, "unknown_140009a30", tpl_46::<0x143a0aea8, 0x143a0aeab, 0x143078a18, 0x142409b50, 0x1425d2c10, 0x141834d18> as *const ()), // sub_140009a30
    Replacement::draft(0x140009a90, "unknown_140009a90", tpl_47::<0x143a0ae08, 0x143a0ae0b, 0x143078a30, 0x142409b50, 0x1425d2c20, 0x141834d18> as *const ()), // sub_140009a90
    Replacement::draft(0x140009af0, "unknown_140009af0", tpl_37::<0x143a0a8f8, 0x143a0a8fb, 0x143078a50, 0x142409b50, 0x1425d2c30, 0x141834d18> as *const ()), // sub_140009af0
    Replacement::draft(0x140009b50, "unknown_140009b50", tpl_45::<0x143a0ab88, 0x143a0ab8b, 0x143078a60, 0x142409b50, 0x1425d2c40, 0x141834d18> as *const ()), // sub_140009b50
    Replacement::draft(0x140009bb0, "unknown_140009bb0", tpl_54::<0x143a0acb0, 0x143a0acb3, 0x143078a70, 0x142409b50, 0x1425d2c50, 0x141834d18> as *const ()), // sub_140009bb0
    Replacement::draft(0x140009c10, "unknown_140009c10", tpl_40::<0x143a0a8d0, 0x143a0a8d3, 0x143078a88, 0x142409b50, 0x1425d2c60, 0x141834d18> as *const ()), // sub_140009c10
    Replacement::draft(0x140009c70, "unknown_140009c70", tpl_50::<0x143a0a920, 0x143a0a923, 0x143078a90, 0x142409b50, 0x1425d2c70, 0x141834d18> as *const ()), // sub_140009c70
    Replacement::draft(0x140009cd0, "unknown_140009cd0", tpl_41::<0x143a0aaf8, 0x143a0aafb, 0x143078aa8, 0x142409b50, 0x1425d2c80, 0x141834d18> as *const ()), // sub_140009cd0
    Replacement::draft(0x140009d30, "unknown_140009d30", tpl_55::<0x143a0ae30, 0x143a0ae33, 0x143078ab8, 0x142409b50, 0x1425d2c90, 0x141834d18> as *const ()), // sub_140009d30
    Replacement::draft(0x140009d90, "unknown_140009d90", tpl_41::<0x143a0aed0, 0x143a0aed3, 0x143078ac8, 0x142409b50, 0x1425d2ca0, 0x141834d18> as *const ()), // sub_140009d90
    Replacement::draft(0x140009df0, "unknown_140009df0", tpl_49::<0x143a0af70, 0x143a0af73, 0x143078ad8, 0x142409b50, 0x1425d2cb0, 0x141834d18> as *const ()), // sub_140009df0
    Replacement::draft(0x140009e50, "unknown_140009e50", tpl_40::<0x143a0a720, 0x143a0a723, 0x143078ae4, 0x142409b50, 0x1425d2cc0, 0x141834d18> as *const ()), // sub_140009e50
    Replacement::draft(0x140009eb0, "unknown_140009eb0", tpl_44::<0x143a0a858, 0x143a0a85b, 0x143078af0, 0x142409b50, 0x1425d2cd0, 0x141834d18> as *const ()), // sub_140009eb0
    Replacement::draft(0x140009f10, "unknown_140009f10", tpl_46::<0x143a0a6f8, 0x143a0a6fb, 0x143078b08, 0x142409b50, 0x1425d2ce0, 0x141834d18> as *const ()), // sub_140009f10
    Replacement::draft(0x140009f70, "unknown_140009f70", tpl_55::<0x143a0ae58, 0x143a0ae5b, 0x143078b20, 0x142409b50, 0x1425d2cf0, 0x141834d18> as *const ()), // sub_140009f70
    Replacement::draft(0x140009fd0, "unknown_140009fd0", tpl_46::<0x143a0acf0, 0x143a0acf3, 0x143078b30, 0x142409b50, 0x1425d2d00, 0x141834d18> as *const ()), // sub_140009fd0
    Replacement::draft(0x14000a030, "unknown_14000a030", tpl_50::<0x143a0a9e8, 0x143a0a9eb, 0x143078b48, 0x142409b50, 0x1425d2d10, 0x141834d18> as *const ()), // sub_14000a030
    Replacement::draft(0x14000a090, "unknown_14000a090", tpl_42::<0x143a0a6a8, 0x143a0a6ab, 0x143078b60, 0x142409b50, 0x1425d2d20, 0x141834d18> as *const ()), // sub_14000a090
    Replacement::draft(0x14000a0f0, "unknown_14000a0f0", tpl_38::<0x143a0adb8, 0x143a0adbb, 0x143078b78, 0x142409b50, 0x1425d2d30, 0x141834d18> as *const ()), // sub_14000a0f0
    Replacement::draft(0x14000a150, "unknown_14000a150", tpl_34::<0x143a0b128, 0x143a0b12b, 0x143078b88, 0x142409b50, 0x1425d2d40, 0x141834d18> as *const ()), // sub_14000a150
    Replacement::draft(0x14000a1b0, "unknown_14000a1b0", tpl_41::<0x143a0af20, 0x143a0af23, 0x143078b98, 0x142409b50, 0x1425d2d50, 0x141834d18> as *const ()), // sub_14000a1b0
    Replacement::draft(0x14000a210, "unknown_14000a210", tpl_36::<0x143a0ac88, 0x143a0ac8b, 0x143078ba8, 0x142409b50, 0x1425d2d60, 0x141834d18> as *const ()), // sub_14000a210
    Replacement::draft(0x14000a270, "unknown_14000a270", tpl_34::<0x143a0a6d0, 0x143a0a6d3, 0x143078bb8, 0x142409b50, 0x1425d2d70, 0x141834d18> as *const ()), // sub_14000a270
    Replacement::draft(0x14000ad70, "unknown_14000ad70", tpl_23::<0x143a0b330, 0x143a0b333, 0x1426cdcb0, 0x142409b50, 0x1425d3320, 0x141834d18> as *const ()), // sub_14000ad70
    Replacement::draft(0x14000add0, "unknown_14000add0", tpl_23::<0x143a0b378, 0x143a0b37b, 0x1426cdcb0, 0x142409b50, 0x1425d3330, 0x141834d18> as *const ()), // sub_14000add0
    Replacement::draft(0x14000afc0, "unknown_14000afc0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438434f0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d3370, 0x141834d18> as *const ()), // sub_14000afc0
    Replacement::draft(0x14000b3c0, "unknown_14000b3c0", tpl_34::<0x143a0b4e0, 0x143a0b4e3, 0x143078668, 0x142409b50, 0x1425d33e0, 0x141834d18> as *const ()), // sub_14000b3c0
    Replacement::draft(0x14000b420, "unknown_14000b420", tpl_23::<0x143a0bb28, 0x143a0bb2b, 0x1426cdcb0, 0x142409b50, 0x1425d33f0, 0x141834d18> as *const ()), // sub_14000b420
    Replacement::draft(0x14000b480, "unknown_14000b480", tpl_35::<0x143a0b858, 0x143a0b85b, 0x143078678, 0x142409b50, 0x1425d3400, 0x141834d18> as *const ()), // sub_14000b480
    Replacement::draft(0x14000b4e0, "unknown_14000b4e0", tpl_36::<0x143a0bab0, 0x143a0bab3, 0x143078680, 0x142409b50, 0x1425d3410, 0x141834d18> as *const ()), // sub_14000b4e0
    Replacement::draft(0x14000b540, "unknown_14000b540", tpl_37::<0x143a0b6f8, 0x143a0b6fb, 0x143078690, 0x142409b50, 0x1425d3420, 0x141834d18> as *const ()), // sub_14000b540
    Replacement::draft(0x14000b5a0, "unknown_14000b5a0", tpl_38::<0x143a0b580, 0x143a0b583, 0x1430786a0, 0x142409b50, 0x1425d3430, 0x141834d18> as *const ()), // sub_14000b5a0
    Replacement::draft(0x14000b600, "unknown_14000b600", tpl_39::<0x143a0bb50, 0x143a0bb53, 0x1430786b0, 0x142409b50, 0x1425d3440, 0x141834d18> as *const ()), // sub_14000b600
    Replacement::draft(0x14000b660, "unknown_14000b660", tpl_37::<0x143a0b490, 0x143a0b493, 0x1430786b8, 0x142409b50, 0x1425d3450, 0x141834d18> as *const ()), // sub_14000b660
    Replacement::draft(0x14000b6c0, "unknown_14000b6c0", tpl_40::<0x143a0bc68, 0x143a0bc6b, 0x1430786c4, 0x142409b50, 0x1425d3460, 0x141834d18> as *const ()), // sub_14000b6c0
    Replacement::draft(0x14000b720, "unknown_14000b720", tpl_41::<0x143a0b648, 0x143a0b64b, 0x1430786d0, 0x142409b50, 0x1425d3470, 0x141834d18> as *const ()), // sub_14000b720
    Replacement::draft(0x14000b780, "unknown_14000b780", tpl_42::<0x143a0bba0, 0x143a0bba3, 0x1430786e0, 0x142409b50, 0x1425d3480, 0x141834d18> as *const ()), // sub_14000b780
    Replacement::draft(0x14000b7e0, "unknown_14000b7e0", tpl_34::<0x143a0bbf0, 0x143a0bbf3, 0x1430786f8, 0x142409b50, 0x1425d3490, 0x141834d18> as *const ()), // sub_14000b7e0
    Replacement::draft(0x14000b840, "unknown_14000b840", tpl_34::<0x143a0b4b8, 0x143a0b4bb, 0x143078708, 0x142409b50, 0x1425d34a0, 0x141834d18> as *const ()), // sub_14000b840
    Replacement::draft(0x14000b8a0, "unknown_14000b8a0", tpl_41::<0x143a0b5a8, 0x143a0b5ab, 0x143078718, 0x142409b50, 0x1425d34b0, 0x141834d18> as *const ()), // sub_14000b8a0
    Replacement::draft(0x14000b900, "unknown_14000b900", tpl_43::<0x143a0b720, 0x143a0b723, 0x143078728, 0x142409b50, 0x1425d34c0, 0x141834d18> as *const ()), // sub_14000b900
    Replacement::draft(0x14000b960, "unknown_14000b960", tpl_44::<0x143a0b1a0, 0x143a0b1a3, 0x143078748, 0x142409b50, 0x1425d34d0, 0x141834d18> as *const ()), // sub_14000b960
    Replacement::draft(0x14000b9c0, "unknown_14000b9c0", tpl_39::<0x143a0b670, 0x143a0b673, 0x143078760, 0x142409b50, 0x1425d34e0, 0x141834d18> as *const ()), // sub_14000b9c0
    Replacement::draft(0x14000ba20, "unknown_14000ba20", tpl_45::<0x143a0b558, 0x143a0b55b, 0x143078768, 0x142409b50, 0x1425d34f0, 0x141834d18> as *const ()), // sub_14000ba20
    Replacement::draft(0x14000ba80, "unknown_14000ba80", tpl_41::<0x143a0ba60, 0x143a0ba63, 0x143078778, 0x142409b50, 0x1425d3500, 0x141834d18> as *const ()), // sub_14000ba80
    Replacement::draft(0x14000bae0, "unknown_14000bae0", tpl_37::<0x143a0b1c8, 0x143a0b1cb, 0x143078788, 0x142409b50, 0x1425d3510, 0x141834d18> as *const ()), // sub_14000bae0
    Replacement::draft(0x14000bb40, "unknown_14000bb40", tpl_46::<0x143a0b8f8, 0x143a0b8fb, 0x143078798, 0x142409b50, 0x1425d3520, 0x141834d18> as *const ()), // sub_14000bb40
    Replacement::draft(0x14000bba0, "unknown_14000bba0", tpl_47::<0x143a0b3f0, 0x143a0b3f3, 0x1430787b0, 0x142409b50, 0x1425d3530, 0x141834d18> as *const ()), // sub_14000bba0
    Replacement::draft(0x14000bc00, "unknown_14000bc00", tpl_38::<0x143a0b998, 0x143a0b99b, 0x1430787d0, 0x142409b50, 0x1425d3540, 0x141834d18> as *const ()), // sub_14000bc00
    Replacement::draft(0x14000bc60, "unknown_14000bc60", tpl_48::<0x143a0bb78, 0x143a0bb7b, 0x1430787e0, 0x142409b50, 0x1425d3550, 0x141834d18> as *const ()), // sub_14000bc60
    Replacement::draft(0x14000bcc0, "unknown_14000bcc0", tpl_49::<0x143a0b830, 0x143a0b833, 0x1430787f8, 0x142409b50, 0x1425d3560, 0x141834d18> as *const ()), // sub_14000bcc0
    Replacement::draft(0x14000bd20, "unknown_14000bd20", tpl_44::<0x143a0bb00, 0x143a0bb03, 0x143078808, 0x142409b50, 0x1425d3570, 0x141834d18> as *const ()), // sub_14000bd20
    Replacement::draft(0x14000bd80, "unknown_14000bd80", tpl_42::<0x143a0b178, 0x143a0b17b, 0x143078820, 0x142409b50, 0x1425d3580, 0x141834d18> as *const ()), // sub_14000bd80
    Replacement::draft(0x14000bde0, "unknown_14000bde0", tpl_50::<0x143a0ba10, 0x143a0ba13, 0x143078838, 0x142409b50, 0x1425d3590, 0x141834d18> as *const ()), // sub_14000bde0
    Replacement::draft(0x14000be40, "unknown_14000be40", tpl_52::<0x143a0b308, 0x143a0b30b, 0x143a0bc68, 0x143a0bc6c, 0x143a0bc6b, 0x143a0bc70, 0x142409b50, 0x1425d35a0, 0x141834d18> as *const ()), // sub_14000be40
    Replacement::draft(0x14000bf00, "unknown_14000bf00", tpl_38::<0x143a0b880, 0x143a0b883, 0x143078950, 0x142409b50, 0x1425d35b0, 0x141834d18> as *const ()), // sub_14000bf00
    Replacement::draft(0x14000bf60, "unknown_14000bf60", tpl_53::<0x143a0b3c8, 0x143a0b3cb, 0x143078960, 0x142409b50, 0x1425d35c0, 0x141834d18> as *const ()), // sub_14000bf60
    Replacement::draft(0x14000bfc0, "unknown_14000bfc0", tpl_45::<0x143a0b290, 0x143a0b293, 0x143078980, 0x142409b50, 0x1425d35d0, 0x141834d18> as *const ()), // sub_14000bfc0
    Replacement::draft(0x14000c020, "unknown_14000c020", tpl_35::<0x143a0b2e0, 0x143a0b2e3, 0x14307898c, 0x142409b50, 0x1425d35e0, 0x141834d18> as *const ()), // sub_14000c020
    Replacement::draft(0x14000c080, "unknown_14000c080", tpl_42::<0x143a0bbc8, 0x143a0bbcb, 0x1430786e0, 0x142409b50, 0x1425d35f0, 0x141834d18> as *const ()), // sub_14000c080
    Replacement::draft(0x14000c0e0, "unknown_14000c0e0", tpl_38::<0x143a0bad8, 0x143a0badb, 0x143078998, 0x142409b50, 0x1425d3600, 0x141834d18> as *const ()), // sub_14000c0e0
    Replacement::draft(0x14000c140, "unknown_14000c140", tpl_38::<0x143a0b5f8, 0x143a0b5fb, 0x1430789a8, 0x142409b50, 0x1425d3610, 0x141834d18> as *const ()), // sub_14000c140
    Replacement::draft(0x14000c1a0, "unknown_14000c1a0", tpl_41::<0x143a0b790, 0x143a0b793, 0x1430789b8, 0x142409b50, 0x1425d3620, 0x141834d18> as *const ()), // sub_14000c1a0
    Replacement::draft(0x14000c200, "unknown_14000c200", tpl_34::<0x143a0bc18, 0x143a0bc1b, 0x1430789c8, 0x142409b50, 0x1425d3630, 0x141834d18> as *const ()), // sub_14000c200
    Replacement::draft(0x14000c260, "unknown_14000c260", tpl_39::<0x143a0b2b8, 0x143a0b2bb, 0x1430789d8, 0x142409b50, 0x1425d3640, 0x141834d18> as *const ()), // sub_14000c260
    Replacement::draft(0x14000c2c0, "unknown_14000c2c0", tpl_41::<0x143a0b5d0, 0x143a0b5d3, 0x1430789e0, 0x142409b50, 0x1425d3650, 0x141834d18> as *const ()), // sub_14000c2c0
    Replacement::draft(0x14000c320, "unknown_14000c320", tpl_34::<0x143a0b8a8, 0x143a0b8ab, 0x1430789f0, 0x142409b50, 0x1425d3660, 0x141834d18> as *const ()), // sub_14000c320
    Replacement::draft(0x14000c380, "unknown_14000c380", tpl_48::<0x143a0b508, 0x143a0b50b, 0x143078a00, 0x142409b50, 0x1425d3670, 0x141834d18> as *const ()), // sub_14000c380
    Replacement::draft(0x14000c3e0, "unknown_14000c3e0", tpl_46::<0x143a0b9c0, 0x143a0b9c3, 0x143078a18, 0x142409b50, 0x1425d3680, 0x141834d18> as *const ()), // sub_14000c3e0
    Replacement::draft(0x14000c440, "unknown_14000c440", tpl_47::<0x143a0b920, 0x143a0b923, 0x143078a30, 0x142409b50, 0x1425d3690, 0x141834d18> as *const ()), // sub_14000c440
    Replacement::draft(0x14000c4a0, "unknown_14000c4a0", tpl_37::<0x143a0b440, 0x143a0b443, 0x143078a50, 0x142409b50, 0x1425d36a0, 0x141834d18> as *const ()), // sub_14000c4a0
    Replacement::draft(0x14000c500, "unknown_14000c500", tpl_45::<0x143a0b6d0, 0x143a0b6d3, 0x143078a60, 0x142409b50, 0x1425d36b0, 0x141834d18> as *const ()), // sub_14000c500
    Replacement::draft(0x14000c560, "unknown_14000c560", tpl_54::<0x143a0b7e0, 0x143a0b7e3, 0x143078a70, 0x142409b50, 0x1425d36c0, 0x141834d18> as *const ()), // sub_14000c560
    Replacement::draft(0x14000c5c0, "unknown_14000c5c0", tpl_40::<0x143a0b418, 0x143a0b41b, 0x143078a88, 0x142409b50, 0x1425d36d0, 0x141834d18> as *const ()), // sub_14000c5c0
    Replacement::draft(0x14000c620, "unknown_14000c620", tpl_50::<0x143a0b468, 0x143a0b46b, 0x143078a90, 0x142409b50, 0x1425d36e0, 0x141834d18> as *const ()), // sub_14000c620
    Replacement::draft(0x14000c680, "unknown_14000c680", tpl_41::<0x143a0b620, 0x143a0b623, 0x143078aa8, 0x142409b50, 0x1425d36f0, 0x141834d18> as *const ()), // sub_14000c680
    Replacement::draft(0x14000c6e0, "unknown_14000c6e0", tpl_55::<0x143a0b948, 0x143a0b94b, 0x143078ab8, 0x142409b50, 0x1425d3700, 0x141834d18> as *const ()), // sub_14000c6e0
    Replacement::draft(0x14000c740, "unknown_14000c740", tpl_41::<0x143a0b9e8, 0x143a0b9eb, 0x143078ac8, 0x142409b50, 0x1425d3710, 0x141834d18> as *const ()), // sub_14000c740
    Replacement::draft(0x14000c7a0, "unknown_14000c7a0", tpl_49::<0x143a0ba88, 0x143a0ba8b, 0x143078ad8, 0x142409b50, 0x1425d3720, 0x141834d18> as *const ()), // sub_14000c7a0
    Replacement::draft(0x14000c800, "unknown_14000c800", tpl_40::<0x143a0b268, 0x143a0b26b, 0x143078ae4, 0x142409b50, 0x1425d3730, 0x141834d18> as *const ()), // sub_14000c800
    Replacement::draft(0x14000c860, "unknown_14000c860", tpl_44::<0x143a0b3a0, 0x143a0b3a3, 0x143078af0, 0x142409b50, 0x1425d3740, 0x141834d18> as *const ()), // sub_14000c860
    Replacement::draft(0x14000c8c0, "unknown_14000c8c0", tpl_46::<0x143a0b240, 0x143a0b243, 0x143078b08, 0x142409b50, 0x1425d3750, 0x141834d18> as *const ()), // sub_14000c8c0
    Replacement::draft(0x14000c920, "unknown_14000c920", tpl_55::<0x143a0b970, 0x143a0b973, 0x143078b20, 0x142409b50, 0x1425d3760, 0x141834d18> as *const ()), // sub_14000c920
    Replacement::draft(0x14000c980, "unknown_14000c980", tpl_46::<0x143a0b808, 0x143a0b80b, 0x143078b30, 0x142409b50, 0x1425d3770, 0x141834d18> as *const ()), // sub_14000c980
    Replacement::draft(0x14000c9e0, "unknown_14000c9e0", tpl_50::<0x143a0b530, 0x143a0b533, 0x143078b48, 0x142409b50, 0x1425d3780, 0x141834d18> as *const ()), // sub_14000c9e0
    Replacement::draft(0x14000ca40, "unknown_14000ca40", tpl_42::<0x143a0b1f0, 0x143a0b1f3, 0x143078b60, 0x142409b50, 0x1425d3790, 0x141834d18> as *const ()), // sub_14000ca40
    Replacement::draft(0x14000caa0, "unknown_14000caa0", tpl_38::<0x143a0b8d0, 0x143a0b8d3, 0x143078b78, 0x142409b50, 0x1425d37a0, 0x141834d18> as *const ()), // sub_14000caa0
    Replacement::draft(0x14000cb00, "unknown_14000cb00", tpl_34::<0x143a0bc40, 0x143a0bc43, 0x143078b88, 0x142409b50, 0x1425d37b0, 0x141834d18> as *const ()), // sub_14000cb00
    Replacement::draft(0x14000cb60, "unknown_14000cb60", tpl_41::<0x143a0ba38, 0x143a0ba3b, 0x143078b98, 0x142409b50, 0x1425d37c0, 0x141834d18> as *const ()), // sub_14000cb60
    Replacement::draft(0x14000cbc0, "unknown_14000cbc0", tpl_36::<0x143a0b7b8, 0x143a0b7bb, 0x143078ba8, 0x142409b50, 0x1425d37d0, 0x141834d18> as *const ()), // sub_14000cbc0
    Replacement::draft(0x14000cc20, "unknown_14000cc20", tpl_34::<0x143a0b218, 0x143a0b21b, 0x143078bb8, 0x142409b50, 0x1425d37e0, 0x141834d18> as *const ()), // sub_14000cc20
    Replacement::draft(0x14000d780, "unknown_14000d780", tpl_23::<0x143a0be48, 0x143a0be4b, 0x1426cdcb0, 0x142409b50, 0x1425d3a80, 0x141834d18> as *const ()), // sub_14000d780
    Replacement::draft(0x14000d7e0, "unknown_14000d7e0", tpl_23::<0x143a0be90, 0x143a0be93, 0x1426cdcb0, 0x142409b50, 0x1425d3a90, 0x141834d18> as *const ()), // sub_14000d7e0
    Replacement::draft(0x14000d9d0, "unknown_14000d9d0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438435d0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d3ad0, 0x141834d18> as *const ()), // sub_14000d9d0
    Replacement::draft(0x14000ddd0, "unknown_14000ddd0", tpl_34::<0x143a0bff8, 0x143a0bffb, 0x143078668, 0x142409b50, 0x1425d3b40, 0x141834d18> as *const ()), // sub_14000ddd0
    Replacement::draft(0x14000de30, "unknown_14000de30", tpl_23::<0x143a0c608, 0x143a0c60b, 0x1426cdcb0, 0x142409b50, 0x1425d3b50, 0x141834d18> as *const ()), // sub_14000de30
    Replacement::draft(0x14000de90, "unknown_14000de90", tpl_35::<0x143a0c338, 0x143a0c33b, 0x143078678, 0x142409b50, 0x1425d3b60, 0x141834d18> as *const ()), // sub_14000de90
    Replacement::draft(0x14000def0, "unknown_14000def0", tpl_36::<0x143a0c590, 0x143a0c593, 0x143078680, 0x142409b50, 0x1425d3b70, 0x141834d18> as *const ()), // sub_14000def0
    Replacement::draft(0x14000df50, "unknown_14000df50", tpl_37::<0x143a0c1d8, 0x143a0c1db, 0x143078690, 0x142409b50, 0x1425d3b80, 0x141834d18> as *const ()), // sub_14000df50
    Replacement::draft(0x14000dfb0, "unknown_14000dfb0", tpl_38::<0x143a0c098, 0x143a0c09b, 0x1430786a0, 0x142409b50, 0x1425d3b90, 0x141834d18> as *const ()), // sub_14000dfb0
    Replacement::draft(0x14000e010, "unknown_14000e010", tpl_39::<0x143a0c630, 0x143a0c633, 0x1430786b0, 0x142409b50, 0x1425d3ba0, 0x141834d18> as *const ()), // sub_14000e010
    Replacement::draft(0x14000e070, "unknown_14000e070", tpl_37::<0x143a0bfa8, 0x143a0bfab, 0x1430786b8, 0x142409b50, 0x1425d3bb0, 0x141834d18> as *const ()), // sub_14000e070
    Replacement::draft(0x14000e0d0, "unknown_14000e0d0", tpl_40::<0x143a0c748, 0x143a0c74b, 0x1430786c4, 0x142409b50, 0x1425d3bc0, 0x141834d18> as *const ()), // sub_14000e0d0
    Replacement::draft(0x14000e130, "unknown_14000e130", tpl_41::<0x143a0c160, 0x143a0c163, 0x1430786d0, 0x142409b50, 0x1425d3bd0, 0x141834d18> as *const ()), // sub_14000e130
    Replacement::draft(0x14000e190, "unknown_14000e190", tpl_42::<0x143a0c680, 0x143a0c683, 0x1430786e0, 0x142409b50, 0x1425d3be0, 0x141834d18> as *const ()), // sub_14000e190
    Replacement::draft(0x14000e1f0, "unknown_14000e1f0", tpl_34::<0x143a0c6d0, 0x143a0c6d3, 0x1430786f8, 0x142409b50, 0x1425d3bf0, 0x141834d18> as *const ()), // sub_14000e1f0
    Replacement::draft(0x14000e250, "unknown_14000e250", tpl_34::<0x143a0bfd0, 0x143a0bfd3, 0x143078708, 0x142409b50, 0x1425d3c00, 0x141834d18> as *const ()), // sub_14000e250
    Replacement::draft(0x14000e2b0, "unknown_14000e2b0", tpl_41::<0x143a0c0c0, 0x143a0c0c3, 0x143078718, 0x142409b50, 0x1425d3c10, 0x141834d18> as *const ()), // sub_14000e2b0
    Replacement::draft(0x14000e310, "unknown_14000e310", tpl_43::<0x143a0c200, 0x143a0c203, 0x143078728, 0x142409b50, 0x1425d3c20, 0x141834d18> as *const ()), // sub_14000e310
    Replacement::draft(0x14000e370, "unknown_14000e370", tpl_44::<0x143a0bcb8, 0x143a0bcbb, 0x143078748, 0x142409b50, 0x1425d3c30, 0x141834d18> as *const ()), // sub_14000e370
    Replacement::draft(0x14000e3d0, "unknown_14000e3d0", tpl_39::<0x143a0c188, 0x143a0c18b, 0x143078760, 0x142409b50, 0x1425d3c40, 0x141834d18> as *const ()), // sub_14000e3d0
    Replacement::draft(0x14000e430, "unknown_14000e430", tpl_45::<0x143a0c070, 0x143a0c073, 0x143078768, 0x142409b50, 0x1425d3c50, 0x141834d18> as *const ()), // sub_14000e430
    Replacement::draft(0x14000e490, "unknown_14000e490", tpl_41::<0x143a0c540, 0x143a0c543, 0x143078778, 0x142409b50, 0x1425d3c60, 0x141834d18> as *const ()), // sub_14000e490
    Replacement::draft(0x14000e4f0, "unknown_14000e4f0", tpl_37::<0x143a0bce0, 0x143a0bce3, 0x143078788, 0x142409b50, 0x1425d3c70, 0x141834d18> as *const ()), // sub_14000e4f0
    Replacement::draft(0x14000e550, "unknown_14000e550", tpl_46::<0x143a0c3d8, 0x143a0c3db, 0x143078798, 0x142409b50, 0x1425d3c80, 0x141834d18> as *const ()), // sub_14000e550
    Replacement::draft(0x14000e5b0, "unknown_14000e5b0", tpl_47::<0x143a0bf08, 0x143a0bf0b, 0x1430787b0, 0x142409b50, 0x1425d3c90, 0x141834d18> as *const ()), // sub_14000e5b0
    Replacement::draft(0x14000e610, "unknown_14000e610", tpl_38::<0x143a0c478, 0x143a0c47b, 0x1430787d0, 0x142409b50, 0x1425d3ca0, 0x141834d18> as *const ()), // sub_14000e610
    Replacement::draft(0x14000e670, "unknown_14000e670", tpl_48::<0x143a0c658, 0x143a0c65b, 0x1430787e0, 0x142409b50, 0x1425d3cb0, 0x141834d18> as *const ()), // sub_14000e670
    Replacement::draft(0x14000e6d0, "unknown_14000e6d0", tpl_49::<0x143a0c310, 0x143a0c313, 0x1430787f8, 0x142409b50, 0x1425d3cc0, 0x141834d18> as *const ()), // sub_14000e6d0
    Replacement::draft(0x14000e730, "unknown_14000e730", tpl_44::<0x143a0c5e0, 0x143a0c5e3, 0x143078808, 0x142409b50, 0x1425d3cd0, 0x141834d18> as *const ()), // sub_14000e730
    Replacement::draft(0x14000e790, "unknown_14000e790", tpl_42::<0x143a0bc90, 0x143a0bc93, 0x143078820, 0x142409b50, 0x1425d3ce0, 0x141834d18> as *const ()), // sub_14000e790
    Replacement::draft(0x14000e7f0, "unknown_14000e7f0", tpl_50::<0x143a0c4f0, 0x143a0c4f3, 0x143078838, 0x142409b50, 0x1425d3cf0, 0x141834d18> as *const ()), // sub_14000e7f0
    Replacement::draft(0x14000e850, "unknown_14000e850", tpl_52::<0x143a0be20, 0x143a0be23, 0x143a0c748, 0x143a0c74c, 0x143a0c74b, 0x143a0c750, 0x142409b50, 0x1425d3d00, 0x141834d18> as *const ()), // sub_14000e850
    Replacement::draft(0x14000e910, "unknown_14000e910", tpl_38::<0x143a0c360, 0x143a0c363, 0x143078950, 0x142409b50, 0x1425d3d10, 0x141834d18> as *const ()), // sub_14000e910
    Replacement::draft(0x14000e970, "unknown_14000e970", tpl_53::<0x143a0bee0, 0x143a0bee3, 0x143078960, 0x142409b50, 0x1425d3d20, 0x141834d18> as *const ()), // sub_14000e970
    Replacement::draft(0x14000e9d0, "unknown_14000e9d0", tpl_45::<0x143a0bda8, 0x143a0bdab, 0x143078980, 0x142409b50, 0x1425d3d30, 0x141834d18> as *const ()), // sub_14000e9d0
    Replacement::draft(0x14000ea30, "unknown_14000ea30", tpl_35::<0x143a0bdf8, 0x143a0bdfb, 0x14307898c, 0x142409b50, 0x1425d3d40, 0x141834d18> as *const ()), // sub_14000ea30
    Replacement::draft(0x14000ea90, "unknown_14000ea90", tpl_42::<0x143a0c6a8, 0x143a0c6ab, 0x1430786e0, 0x142409b50, 0x1425d3d50, 0x141834d18> as *const ()), // sub_14000ea90
    Replacement::draft(0x14000eaf0, "unknown_14000eaf0", tpl_38::<0x143a0c5b8, 0x143a0c5bb, 0x143078998, 0x142409b50, 0x1425d3d60, 0x141834d18> as *const ()), // sub_14000eaf0
    Replacement::draft(0x14000eb50, "unknown_14000eb50", tpl_38::<0x143a0c110, 0x143a0c113, 0x1430789a8, 0x142409b50, 0x1425d3d70, 0x141834d18> as *const ()), // sub_14000eb50
    Replacement::draft(0x14000ebb0, "unknown_14000ebb0", tpl_41::<0x143a0c270, 0x143a0c273, 0x1430789b8, 0x142409b50, 0x1425d3d80, 0x141834d18> as *const ()), // sub_14000ebb0
    Replacement::draft(0x14000ec10, "unknown_14000ec10", tpl_34::<0x143a0c6f8, 0x143a0c6fb, 0x1430789c8, 0x142409b50, 0x1425d3d90, 0x141834d18> as *const ()), // sub_14000ec10
    Replacement::draft(0x14000ec70, "unknown_14000ec70", tpl_39::<0x143a0bdd0, 0x143a0bdd3, 0x1430789d8, 0x142409b50, 0x1425d3da0, 0x141834d18> as *const ()), // sub_14000ec70
    Replacement::draft(0x14000ecd0, "unknown_14000ecd0", tpl_41::<0x143a0c0e8, 0x143a0c0eb, 0x1430789e0, 0x142409b50, 0x1425d3db0, 0x141834d18> as *const ()), // sub_14000ecd0
    Replacement::draft(0x14000ed30, "unknown_14000ed30", tpl_34::<0x143a0c388, 0x143a0c38b, 0x1430789f0, 0x142409b50, 0x1425d3dc0, 0x141834d18> as *const ()), // sub_14000ed30
    Replacement::draft(0x14000ed90, "unknown_14000ed90", tpl_48::<0x143a0c020, 0x143a0c023, 0x143078a00, 0x142409b50, 0x1425d3dd0, 0x141834d18> as *const ()), // sub_14000ed90
    Replacement::draft(0x14000edf0, "unknown_14000edf0", tpl_46::<0x143a0c4a0, 0x143a0c4a3, 0x143078a18, 0x142409b50, 0x1425d3de0, 0x141834d18> as *const ()), // sub_14000edf0
    Replacement::draft(0x14000ee50, "unknown_14000ee50", tpl_47::<0x143a0c400, 0x143a0c403, 0x143078a30, 0x142409b50, 0x1425d3df0, 0x141834d18> as *const ()), // sub_14000ee50
    Replacement::draft(0x14000eeb0, "unknown_14000eeb0", tpl_37::<0x143a0bf58, 0x143a0bf5b, 0x143078a50, 0x142409b50, 0x1425d3e00, 0x141834d18> as *const ()), // sub_14000eeb0
    Replacement::draft(0x14000ef10, "unknown_14000ef10", tpl_45::<0x143a0c1b0, 0x143a0c1b3, 0x143078a60, 0x142409b50, 0x1425d3e10, 0x141834d18> as *const ()), // sub_14000ef10
    Replacement::draft(0x14000ef70, "unknown_14000ef70", tpl_54::<0x143a0c2c0, 0x143a0c2c3, 0x143078a70, 0x142409b50, 0x1425d3e20, 0x141834d18> as *const ()), // sub_14000ef70
    Replacement::draft(0x14000efd0, "unknown_14000efd0", tpl_40::<0x143a0bf30, 0x143a0bf33, 0x143078a88, 0x142409b50, 0x1425d3e30, 0x141834d18> as *const ()), // sub_14000efd0
    Replacement::draft(0x14000f030, "unknown_14000f030", tpl_50::<0x143a0bf80, 0x143a0bf83, 0x143078a90, 0x142409b50, 0x1425d3e40, 0x141834d18> as *const ()), // sub_14000f030
    Replacement::draft(0x14000f090, "unknown_14000f090", tpl_41::<0x143a0c138, 0x143a0c13b, 0x143078aa8, 0x142409b50, 0x1425d3e50, 0x141834d18> as *const ()), // sub_14000f090
    Replacement::draft(0x14000f0f0, "unknown_14000f0f0", tpl_55::<0x143a0c428, 0x143a0c42b, 0x143078ab8, 0x142409b50, 0x1425d3e60, 0x141834d18> as *const ()), // sub_14000f0f0
    Replacement::draft(0x14000f150, "unknown_14000f150", tpl_41::<0x143a0c4c8, 0x143a0c4cb, 0x143078ac8, 0x142409b50, 0x1425d3e70, 0x141834d18> as *const ()), // sub_14000f150
    Replacement::draft(0x14000f1b0, "unknown_14000f1b0", tpl_49::<0x143a0c568, 0x143a0c56b, 0x143078ad8, 0x142409b50, 0x1425d3e80, 0x141834d18> as *const ()), // sub_14000f1b0
    Replacement::draft(0x14000f210, "unknown_14000f210", tpl_40::<0x143a0bd80, 0x143a0bd83, 0x143078ae4, 0x142409b50, 0x1425d3e90, 0x141834d18> as *const ()), // sub_14000f210
    Replacement::draft(0x14000f270, "unknown_14000f270", tpl_44::<0x143a0beb8, 0x143a0bebb, 0x143078af0, 0x142409b50, 0x1425d3ea0, 0x141834d18> as *const ()), // sub_14000f270
    Replacement::draft(0x14000f2d0, "unknown_14000f2d0", tpl_46::<0x143a0bd58, 0x143a0bd5b, 0x143078b08, 0x142409b50, 0x1425d3eb0, 0x141834d18> as *const ()), // sub_14000f2d0
    Replacement::draft(0x14000f330, "unknown_14000f330", tpl_55::<0x143a0c450, 0x143a0c453, 0x143078b20, 0x142409b50, 0x1425d3ec0, 0x141834d18> as *const ()), // sub_14000f330
    Replacement::draft(0x14000f390, "unknown_14000f390", tpl_46::<0x143a0c2e8, 0x143a0c2eb, 0x143078b30, 0x142409b50, 0x1425d3ed0, 0x141834d18> as *const ()), // sub_14000f390
    Replacement::draft(0x14000f3f0, "unknown_14000f3f0", tpl_50::<0x143a0c048, 0x143a0c04b, 0x143078b48, 0x142409b50, 0x1425d3ee0, 0x141834d18> as *const ()), // sub_14000f3f0
    Replacement::draft(0x14000f450, "unknown_14000f450", tpl_42::<0x143a0bd08, 0x143a0bd0b, 0x143078b60, 0x142409b50, 0x1425d3ef0, 0x141834d18> as *const ()), // sub_14000f450
    Replacement::draft(0x14000f4b0, "unknown_14000f4b0", tpl_38::<0x143a0c3b0, 0x143a0c3b3, 0x143078b78, 0x142409b50, 0x1425d3f00, 0x141834d18> as *const ()), // sub_14000f4b0
    Replacement::draft(0x14000f510, "unknown_14000f510", tpl_34::<0x143a0c720, 0x143a0c723, 0x143078b88, 0x142409b50, 0x1425d3f10, 0x141834d18> as *const ()), // sub_14000f510
    Replacement::draft(0x14000f570, "unknown_14000f570", tpl_41::<0x143a0c518, 0x143a0c51b, 0x143078b98, 0x142409b50, 0x1425d3f20, 0x141834d18> as *const ()), // sub_14000f570
    Replacement::draft(0x14000f5d0, "unknown_14000f5d0", tpl_36::<0x143a0c298, 0x143a0c29b, 0x143078ba8, 0x142409b50, 0x1425d3f30, 0x141834d18> as *const ()), // sub_14000f5d0
    Replacement::draft(0x14000f630, "unknown_14000f630", tpl_34::<0x143a0bd30, 0x143a0bd33, 0x143078bb8, 0x142409b50, 0x1425d3f40, 0x141834d18> as *const ()), // sub_14000f630
    Replacement::draft(0x14000f7b0, "unknown_14000f7b0", tpl_23::<0x143a0c960, 0x143a0c963, 0x1426cdcb0, 0x142409b50, 0x1425d3f50, 0x141834d18> as *const ()), // sub_14000f7b0
    Replacement::draft(0x14000f810, "unknown_14000f810", tpl_23::<0x143a0c9a8, 0x143a0c9ab, 0x1426cdcb0, 0x142409b50, 0x1425d3f60, 0x141834d18> as *const ()), // sub_14000f810
    Replacement::draft(0x14000fa00, "unknown_14000fa00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843670, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d3fa0, 0x141834d18> as *const ()), // sub_14000fa00
    Replacement::draft(0x14000fe00, "unknown_14000fe00", tpl_34::<0x143a0cb10, 0x143a0cb13, 0x143078668, 0x142409b50, 0x1425d4010, 0x141834d18> as *const ()), // sub_14000fe00
    Replacement::draft(0x14000fe60, "unknown_14000fe60", tpl_23::<0x143a0d118, 0x143a0d11b, 0x1426cdcb0, 0x142409b50, 0x1425d4020, 0x141834d18> as *const ()), // sub_14000fe60
    Replacement::draft(0x14000fec0, "unknown_14000fec0", tpl_35::<0x143a0ce48, 0x143a0ce4b, 0x143078678, 0x142409b50, 0x1425d4030, 0x141834d18> as *const ()), // sub_14000fec0
    Replacement::draft(0x14000ff20, "unknown_14000ff20", tpl_36::<0x143a0d0a0, 0x143a0d0a3, 0x143078680, 0x142409b50, 0x1425d4040, 0x141834d18> as *const ()), // sub_14000ff20
    Replacement::draft(0x14000ff80, "unknown_14000ff80", tpl_37::<0x143a0ccf0, 0x143a0ccf3, 0x143078690, 0x142409b50, 0x1425d4050, 0x141834d18> as *const ()), // sub_14000ff80
    Replacement::draft(0x14000ffe0, "unknown_14000ffe0", tpl_38::<0x143a0cbb0, 0x143a0cbb3, 0x1430786a0, 0x142409b50, 0x1425d4060, 0x141834d18> as *const ()), // sub_14000ffe0
    Replacement::draft(0x140010040, "unknown_140010040", tpl_39::<0x143a0d140, 0x143a0d143, 0x1430786b0, 0x142409b50, 0x1425d4070, 0x141834d18> as *const ()), // sub_140010040
    Replacement::draft(0x1400100a0, "unknown_1400100a0", tpl_37::<0x143a0cac0, 0x143a0cac3, 0x1430786b8, 0x142409b50, 0x1425d4080, 0x141834d18> as *const ()), // sub_1400100a0
    Replacement::draft(0x140010100, "unknown_140010100", tpl_40::<0x143a0d258, 0x143a0d25b, 0x1430786c4, 0x142409b50, 0x1425d4090, 0x141834d18> as *const ()), // sub_140010100
    Replacement::draft(0x140010160, "unknown_140010160", tpl_41::<0x143a0cc78, 0x143a0cc7b, 0x1430786d0, 0x142409b50, 0x1425d40a0, 0x141834d18> as *const ()), // sub_140010160
    Replacement::draft(0x1400101c0, "unknown_1400101c0", tpl_42::<0x143a0d190, 0x143a0d193, 0x1430786e0, 0x142409b50, 0x1425d40b0, 0x141834d18> as *const ()), // sub_1400101c0
    Replacement::draft(0x140010220, "unknown_140010220", tpl_34::<0x143a0d1e0, 0x143a0d1e3, 0x1430786f8, 0x142409b50, 0x1425d40c0, 0x141834d18> as *const ()), // sub_140010220
    Replacement::draft(0x140010280, "unknown_140010280", tpl_34::<0x143a0cae8, 0x143a0caeb, 0x143078708, 0x142409b50, 0x1425d40d0, 0x141834d18> as *const ()), // sub_140010280
    Replacement::draft(0x1400102e0, "unknown_1400102e0", tpl_41::<0x143a0cbd8, 0x143a0cbdb, 0x143078718, 0x142409b50, 0x1425d40e0, 0x141834d18> as *const ()), // sub_1400102e0
    Replacement::draft(0x140010340, "unknown_140010340", tpl_43::<0x143a0cd18, 0x143a0cd1b, 0x143078728, 0x142409b50, 0x1425d40f0, 0x141834d18> as *const ()), // sub_140010340
    Replacement::draft(0x1400103a0, "unknown_1400103a0", tpl_44::<0x143a0c7d0, 0x143a0c7d3, 0x143078748, 0x142409b50, 0x1425d4100, 0x141834d18> as *const ()), // sub_1400103a0
    Replacement::draft(0x140010400, "unknown_140010400", tpl_39::<0x143a0cca0, 0x143a0cca3, 0x143078760, 0x142409b50, 0x1425d4110, 0x141834d18> as *const ()), // sub_140010400
    Replacement::draft(0x140010460, "unknown_140010460", tpl_45::<0x143a0cb88, 0x143a0cb8b, 0x143078768, 0x142409b50, 0x1425d4120, 0x141834d18> as *const ()), // sub_140010460
    Replacement::draft(0x1400104c0, "unknown_1400104c0", tpl_41::<0x143a0d050, 0x143a0d053, 0x143078778, 0x142409b50, 0x1425d4130, 0x141834d18> as *const ()), // sub_1400104c0
    Replacement::draft(0x140010520, "unknown_140010520", tpl_37::<0x143a0c7f8, 0x143a0c7fb, 0x143078788, 0x142409b50, 0x1425d4140, 0x141834d18> as *const ()), // sub_140010520
    Replacement::draft(0x140010580, "unknown_140010580", tpl_46::<0x143a0cee8, 0x143a0ceeb, 0x143078798, 0x142409b50, 0x1425d4150, 0x141834d18> as *const ()), // sub_140010580
    Replacement::draft(0x1400105e0, "unknown_1400105e0", tpl_47::<0x143a0ca20, 0x143a0ca23, 0x1430787b0, 0x142409b50, 0x1425d4160, 0x141834d18> as *const ()), // sub_1400105e0
    Replacement::draft(0x140010640, "unknown_140010640", tpl_38::<0x143a0cf88, 0x143a0cf8b, 0x1430787d0, 0x142409b50, 0x1425d4170, 0x141834d18> as *const ()), // sub_140010640
    Replacement::draft(0x1400106a0, "unknown_1400106a0", tpl_48::<0x143a0d168, 0x143a0d16b, 0x1430787e0, 0x142409b50, 0x1425d4180, 0x141834d18> as *const ()), // sub_1400106a0
    Replacement::draft(0x140010700, "unknown_140010700", tpl_49::<0x143a0ce20, 0x143a0ce23, 0x1430787f8, 0x142409b50, 0x1425d4190, 0x141834d18> as *const ()), // sub_140010700
    Replacement::draft(0x140010760, "unknown_140010760", tpl_44::<0x143a0d0f0, 0x143a0d0f3, 0x143078808, 0x142409b50, 0x1425d41a0, 0x141834d18> as *const ()), // sub_140010760
    Replacement::draft(0x1400107c0, "unknown_1400107c0", tpl_42::<0x143a0c7a8, 0x143a0c7ab, 0x143078820, 0x142409b50, 0x1425d41b0, 0x141834d18> as *const ()), // sub_1400107c0
    Replacement::draft(0x140010820, "unknown_140010820", tpl_50::<0x143a0d000, 0x143a0d003, 0x143078838, 0x142409b50, 0x1425d41c0, 0x141834d18> as *const ()), // sub_140010820
    Replacement::draft(0x140010880, "unknown_140010880", tpl_52::<0x143a0c938, 0x143a0c93b, 0x143a0d258, 0x143a0d25c, 0x143a0d25b, 0x143a0d260, 0x142409b50, 0x1425d41d0, 0x141834d18> as *const ()), // sub_140010880
    Replacement::draft(0x140010940, "unknown_140010940", tpl_38::<0x143a0ce70, 0x143a0ce73, 0x143078950, 0x142409b50, 0x1425d41e0, 0x141834d18> as *const ()), // sub_140010940
    Replacement::draft(0x1400109a0, "unknown_1400109a0", tpl_53::<0x143a0c9f8, 0x143a0c9fb, 0x143078960, 0x142409b50, 0x1425d41f0, 0x141834d18> as *const ()), // sub_1400109a0
    Replacement::draft(0x140010a00, "unknown_140010a00", tpl_45::<0x143a0c8c0, 0x143a0c8c3, 0x143078980, 0x142409b50, 0x1425d4200, 0x141834d18> as *const ()), // sub_140010a00
    Replacement::draft(0x140010a60, "unknown_140010a60", tpl_35::<0x143a0c910, 0x143a0c913, 0x14307898c, 0x142409b50, 0x1425d4210, 0x141834d18> as *const ()), // sub_140010a60
    Replacement::draft(0x140010ac0, "unknown_140010ac0", tpl_42::<0x143a0d1b8, 0x143a0d1bb, 0x1430786e0, 0x142409b50, 0x1425d4220, 0x141834d18> as *const ()), // sub_140010ac0
    Replacement::draft(0x140010b20, "unknown_140010b20", tpl_38::<0x143a0d0c8, 0x143a0d0cb, 0x143078998, 0x142409b50, 0x1425d4230, 0x141834d18> as *const ()), // sub_140010b20
    Replacement::draft(0x140010b80, "unknown_140010b80", tpl_38::<0x143a0cc28, 0x143a0cc2b, 0x1430789a8, 0x142409b50, 0x1425d4240, 0x141834d18> as *const ()), // sub_140010b80
    Replacement::draft(0x140010be0, "unknown_140010be0", tpl_41::<0x143a0cd80, 0x143a0cd83, 0x1430789b8, 0x142409b50, 0x1425d4250, 0x141834d18> as *const ()), // sub_140010be0
    Replacement::draft(0x140010c40, "unknown_140010c40", tpl_34::<0x143a0d208, 0x143a0d20b, 0x1430789c8, 0x142409b50, 0x1425d4260, 0x141834d18> as *const ()), // sub_140010c40
    Replacement::draft(0x140010ca0, "unknown_140010ca0", tpl_39::<0x143a0c8e8, 0x143a0c8eb, 0x1430789d8, 0x142409b50, 0x1425d4270, 0x141834d18> as *const ()), // sub_140010ca0
    Replacement::draft(0x140010d00, "unknown_140010d00", tpl_41::<0x143a0cc00, 0x143a0cc03, 0x1430789e0, 0x142409b50, 0x1425d4280, 0x141834d18> as *const ()), // sub_140010d00
    Replacement::draft(0x140010d60, "unknown_140010d60", tpl_34::<0x143a0ce98, 0x143a0ce9b, 0x1430789f0, 0x142409b50, 0x1425d4290, 0x141834d18> as *const ()), // sub_140010d60
    Replacement::draft(0x140010dc0, "unknown_140010dc0", tpl_48::<0x143a0cb38, 0x143a0cb3b, 0x143078a00, 0x142409b50, 0x1425d42a0, 0x141834d18> as *const ()), // sub_140010dc0
    Replacement::draft(0x140010e20, "unknown_140010e20", tpl_46::<0x143a0cfb0, 0x143a0cfb3, 0x143078a18, 0x142409b50, 0x1425d42b0, 0x141834d18> as *const ()), // sub_140010e20
    Replacement::draft(0x140010e80, "unknown_140010e80", tpl_47::<0x143a0cf10, 0x143a0cf13, 0x143078a30, 0x142409b50, 0x1425d42c0, 0x141834d18> as *const ()), // sub_140010e80
    Replacement::draft(0x140010ee0, "unknown_140010ee0", tpl_37::<0x143a0ca70, 0x143a0ca73, 0x143078a50, 0x142409b50, 0x1425d42d0, 0x141834d18> as *const ()), // sub_140010ee0
    Replacement::draft(0x140010f40, "unknown_140010f40", tpl_45::<0x143a0ccc8, 0x143a0cccb, 0x143078a60, 0x142409b50, 0x1425d42e0, 0x141834d18> as *const ()), // sub_140010f40
    Replacement::draft(0x140010fa0, "unknown_140010fa0", tpl_54::<0x143a0cdd0, 0x143a0cdd3, 0x143078a70, 0x142409b50, 0x1425d42f0, 0x141834d18> as *const ()), // sub_140010fa0
    Replacement::draft(0x140011000, "unknown_140011000", tpl_40::<0x143a0ca48, 0x143a0ca4b, 0x143078a88, 0x142409b50, 0x1425d4300, 0x141834d18> as *const ()), // sub_140011000
    Replacement::draft(0x140011060, "unknown_140011060", tpl_50::<0x143a0ca98, 0x143a0ca9b, 0x143078a90, 0x142409b50, 0x1425d4310, 0x141834d18> as *const ()), // sub_140011060
    Replacement::draft(0x1400110c0, "unknown_1400110c0", tpl_41::<0x143a0cc50, 0x143a0cc53, 0x143078aa8, 0x142409b50, 0x1425d4320, 0x141834d18> as *const ()), // sub_1400110c0
    Replacement::draft(0x140011120, "unknown_140011120", tpl_55::<0x143a0cf38, 0x143a0cf3b, 0x143078ab8, 0x142409b50, 0x1425d4330, 0x141834d18> as *const ()), // sub_140011120
    Replacement::draft(0x140011180, "unknown_140011180", tpl_41::<0x143a0cfd8, 0x143a0cfdb, 0x143078ac8, 0x142409b50, 0x1425d4340, 0x141834d18> as *const ()), // sub_140011180
    Replacement::draft(0x1400111e0, "unknown_1400111e0", tpl_49::<0x143a0d078, 0x143a0d07b, 0x143078ad8, 0x142409b50, 0x1425d4350, 0x141834d18> as *const ()), // sub_1400111e0
    Replacement::draft(0x140011240, "unknown_140011240", tpl_40::<0x143a0c898, 0x143a0c89b, 0x143078ae4, 0x142409b50, 0x1425d4360, 0x141834d18> as *const ()), // sub_140011240
    Replacement::draft(0x1400112a0, "unknown_1400112a0", tpl_44::<0x143a0c9d0, 0x143a0c9d3, 0x143078af0, 0x142409b50, 0x1425d4370, 0x141834d18> as *const ()), // sub_1400112a0
    Replacement::draft(0x140011300, "unknown_140011300", tpl_46::<0x143a0c870, 0x143a0c873, 0x143078b08, 0x142409b50, 0x1425d4380, 0x141834d18> as *const ()), // sub_140011300
    Replacement::draft(0x140011360, "unknown_140011360", tpl_55::<0x143a0cf60, 0x143a0cf63, 0x143078b20, 0x142409b50, 0x1425d4390, 0x141834d18> as *const ()), // sub_140011360
    Replacement::draft(0x1400113c0, "unknown_1400113c0", tpl_46::<0x143a0cdf8, 0x143a0cdfb, 0x143078b30, 0x142409b50, 0x1425d43a0, 0x141834d18> as *const ()), // sub_1400113c0
    Replacement::draft(0x140011420, "unknown_140011420", tpl_50::<0x143a0cb60, 0x143a0cb63, 0x143078b48, 0x142409b50, 0x1425d43b0, 0x141834d18> as *const ()), // sub_140011420
    Replacement::draft(0x140011480, "unknown_140011480", tpl_42::<0x143a0c820, 0x143a0c823, 0x143078b60, 0x142409b50, 0x1425d43c0, 0x141834d18> as *const ()), // sub_140011480
    Replacement::draft(0x1400114e0, "unknown_1400114e0", tpl_38::<0x143a0cec0, 0x143a0cec3, 0x143078b78, 0x142409b50, 0x1425d43d0, 0x141834d18> as *const ()), // sub_1400114e0
    Replacement::draft(0x140011540, "unknown_140011540", tpl_34::<0x143a0d230, 0x143a0d233, 0x143078b88, 0x142409b50, 0x1425d43e0, 0x141834d18> as *const ()), // sub_140011540
    Replacement::draft(0x1400115a0, "unknown_1400115a0", tpl_41::<0x143a0d028, 0x143a0d02b, 0x143078b98, 0x142409b50, 0x1425d43f0, 0x141834d18> as *const ()), // sub_1400115a0
    Replacement::draft(0x140011600, "unknown_140011600", tpl_36::<0x143a0cda8, 0x143a0cdab, 0x143078ba8, 0x142409b50, 0x1425d4400, 0x141834d18> as *const ()), // sub_140011600
    Replacement::draft(0x140011660, "unknown_140011660", tpl_34::<0x143a0c848, 0x143a0c84b, 0x143078bb8, 0x142409b50, 0x1425d4410, 0x141834d18> as *const ()), // sub_140011660
    Replacement::draft(0x1400119c0, "unknown_1400119c0", tpl_23::<0x143a0d4a8, 0x143a0d4ab, 0x1426cdcb0, 0x142409b50, 0x1425d4420, 0x141834d18> as *const ()), // sub_1400119c0
    Replacement::draft(0x140011a20, "unknown_140011a20", tpl_23::<0x143a0d4f0, 0x143a0d4f3, 0x1426cdcb0, 0x142409b50, 0x1425d4430, 0x141834d18> as *const ()), // sub_140011a20
    Replacement::draft(0x140011c10, "unknown_140011c10", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843700, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d4470, 0x141834d18> as *const ()), // sub_140011c10
    Replacement::draft(0x140012010, "unknown_140012010", tpl_34::<0x143a0d658, 0x143a0d65b, 0x143078668, 0x142409b50, 0x1425d44e0, 0x141834d18> as *const ()), // sub_140012010
    Replacement::draft(0x140012070, "unknown_140012070", tpl_23::<0x143a0dc68, 0x143a0dc6b, 0x1426cdcb0, 0x142409b50, 0x1425d44f0, 0x141834d18> as *const ()), // sub_140012070
    Replacement::draft(0x1400120d0, "unknown_1400120d0", tpl_35::<0x143a0d998, 0x143a0d99b, 0x143078678, 0x142409b50, 0x1425d4500, 0x141834d18> as *const ()), // sub_1400120d0
    Replacement::draft(0x140012130, "unknown_140012130", tpl_36::<0x143a0dbf0, 0x143a0dbf3, 0x143078680, 0x142409b50, 0x1425d4510, 0x141834d18> as *const ()), // sub_140012130
    Replacement::draft(0x140012190, "unknown_140012190", tpl_37::<0x143a0d838, 0x143a0d83b, 0x143078690, 0x142409b50, 0x1425d4520, 0x141834d18> as *const ()), // sub_140012190
    Replacement::draft(0x1400121f0, "unknown_1400121f0", tpl_38::<0x143a0d6f8, 0x143a0d6fb, 0x1430786a0, 0x142409b50, 0x1425d4530, 0x141834d18> as *const ()), // sub_1400121f0
    Replacement::draft(0x140012250, "unknown_140012250", tpl_39::<0x143a0dc90, 0x143a0dc93, 0x1430786b0, 0x142409b50, 0x1425d4540, 0x141834d18> as *const ()), // sub_140012250
    Replacement::draft(0x1400122b0, "unknown_1400122b0", tpl_37::<0x143a0d608, 0x143a0d60b, 0x1430786b8, 0x142409b50, 0x1425d4550, 0x141834d18> as *const ()), // sub_1400122b0
    Replacement::draft(0x140012310, "unknown_140012310", tpl_40::<0x143a0dda8, 0x143a0ddab, 0x1430786c4, 0x142409b50, 0x1425d4560, 0x141834d18> as *const ()), // sub_140012310
    Replacement::draft(0x140012370, "unknown_140012370", tpl_41::<0x143a0d7c0, 0x143a0d7c3, 0x1430786d0, 0x142409b50, 0x1425d4570, 0x141834d18> as *const ()), // sub_140012370
    Replacement::draft(0x1400123d0, "unknown_1400123d0", tpl_42::<0x143a0dce0, 0x143a0dce3, 0x1430786e0, 0x142409b50, 0x1425d4580, 0x141834d18> as *const ()), // sub_1400123d0
    Replacement::draft(0x140012430, "unknown_140012430", tpl_34::<0x143a0dd30, 0x143a0dd33, 0x1430786f8, 0x142409b50, 0x1425d4590, 0x141834d18> as *const ()), // sub_140012430
    Replacement::draft(0x140012490, "unknown_140012490", tpl_34::<0x143a0d630, 0x143a0d633, 0x143078708, 0x142409b50, 0x1425d45a0, 0x141834d18> as *const ()), // sub_140012490
    Replacement::draft(0x1400124f0, "unknown_1400124f0", tpl_41::<0x143a0d720, 0x143a0d723, 0x143078718, 0x142409b50, 0x1425d45b0, 0x141834d18> as *const ()), // sub_1400124f0
    Replacement::draft(0x140012550, "unknown_140012550", tpl_43::<0x143a0d860, 0x143a0d863, 0x143078728, 0x142409b50, 0x1425d45c0, 0x141834d18> as *const ()), // sub_140012550
    Replacement::draft(0x1400125b0, "unknown_1400125b0", tpl_44::<0x143a0d318, 0x143a0d31b, 0x143078748, 0x142409b50, 0x1425d45d0, 0x141834d18> as *const ()), // sub_1400125b0
    Replacement::draft(0x140012610, "unknown_140012610", tpl_39::<0x143a0d7e8, 0x143a0d7eb, 0x143078760, 0x142409b50, 0x1425d45e0, 0x141834d18> as *const ()), // sub_140012610
    Replacement::draft(0x140012670, "unknown_140012670", tpl_45::<0x143a0d6d0, 0x143a0d6d3, 0x143078768, 0x142409b50, 0x1425d45f0, 0x141834d18> as *const ()), // sub_140012670
    Replacement::draft(0x1400126d0, "unknown_1400126d0", tpl_41::<0x143a0dba0, 0x143a0dba3, 0x143078778, 0x142409b50, 0x1425d4600, 0x141834d18> as *const ()), // sub_1400126d0
    Replacement::draft(0x140012730, "unknown_140012730", tpl_37::<0x143a0d340, 0x143a0d343, 0x143078788, 0x142409b50, 0x1425d4610, 0x141834d18> as *const ()), // sub_140012730
    Replacement::draft(0x140012790, "unknown_140012790", tpl_46::<0x143a0da38, 0x143a0da3b, 0x143078798, 0x142409b50, 0x1425d4620, 0x141834d18> as *const ()), // sub_140012790
    Replacement::draft(0x1400127f0, "unknown_1400127f0", tpl_47::<0x143a0d568, 0x143a0d56b, 0x1430787b0, 0x142409b50, 0x1425d4630, 0x141834d18> as *const ()), // sub_1400127f0
    Replacement::draft(0x140012850, "unknown_140012850", tpl_38::<0x143a0dad8, 0x143a0dadb, 0x1430787d0, 0x142409b50, 0x1425d4640, 0x141834d18> as *const ()), // sub_140012850
    Replacement::draft(0x1400128b0, "unknown_1400128b0", tpl_48::<0x143a0dcb8, 0x143a0dcbb, 0x1430787e0, 0x142409b50, 0x1425d4650, 0x141834d18> as *const ()), // sub_1400128b0
    Replacement::draft(0x140012910, "unknown_140012910", tpl_49::<0x143a0d970, 0x143a0d973, 0x1430787f8, 0x142409b50, 0x1425d4660, 0x141834d18> as *const ()), // sub_140012910
    Replacement::draft(0x140012970, "unknown_140012970", tpl_44::<0x143a0dc40, 0x143a0dc43, 0x143078808, 0x142409b50, 0x1425d4670, 0x141834d18> as *const ()), // sub_140012970
    Replacement::draft(0x1400129d0, "unknown_1400129d0", tpl_42::<0x143a0d2f0, 0x143a0d2f3, 0x143078820, 0x142409b50, 0x1425d4680, 0x141834d18> as *const ()), // sub_1400129d0
    Replacement::draft(0x140012a30, "unknown_140012a30", tpl_50::<0x143a0db50, 0x143a0db53, 0x143078838, 0x142409b50, 0x1425d4690, 0x141834d18> as *const ()), // sub_140012a30
    Replacement::draft(0x140012a90, "unknown_140012a90", tpl_52::<0x143a0d480, 0x143a0d483, 0x143a0dda8, 0x143a0ddac, 0x143a0ddab, 0x143a0ddb0, 0x142409b50, 0x1425d46a0, 0x141834d18> as *const ()), // sub_140012a90
    Replacement::draft(0x140012b50, "unknown_140012b50", tpl_38::<0x143a0d9c0, 0x143a0d9c3, 0x143078950, 0x142409b50, 0x1425d46b0, 0x141834d18> as *const ()), // sub_140012b50
    Replacement::draft(0x140012bb0, "unknown_140012bb0", tpl_53::<0x143a0d540, 0x143a0d543, 0x143078960, 0x142409b50, 0x1425d46c0, 0x141834d18> as *const ()), // sub_140012bb0
    Replacement::draft(0x140012c10, "unknown_140012c10", tpl_45::<0x143a0d408, 0x143a0d40b, 0x143078980, 0x142409b50, 0x1425d46d0, 0x141834d18> as *const ()), // sub_140012c10
    Replacement::draft(0x140012c70, "unknown_140012c70", tpl_35::<0x143a0d458, 0x143a0d45b, 0x14307898c, 0x142409b50, 0x1425d46e0, 0x141834d18> as *const ()), // sub_140012c70
    Replacement::draft(0x140012cd0, "unknown_140012cd0", tpl_42::<0x143a0dd08, 0x143a0dd0b, 0x1430786e0, 0x142409b50, 0x1425d46f0, 0x141834d18> as *const ()), // sub_140012cd0
    Replacement::draft(0x140012d30, "unknown_140012d30", tpl_38::<0x143a0dc18, 0x143a0dc1b, 0x143078998, 0x142409b50, 0x1425d4700, 0x141834d18> as *const ()), // sub_140012d30
    Replacement::draft(0x140012d90, "unknown_140012d90", tpl_38::<0x143a0d770, 0x143a0d773, 0x1430789a8, 0x142409b50, 0x1425d4710, 0x141834d18> as *const ()), // sub_140012d90
    Replacement::draft(0x140012df0, "unknown_140012df0", tpl_41::<0x143a0d8d0, 0x143a0d8d3, 0x1430789b8, 0x142409b50, 0x1425d4720, 0x141834d18> as *const ()), // sub_140012df0
    Replacement::draft(0x140012e50, "unknown_140012e50", tpl_34::<0x143a0dd58, 0x143a0dd5b, 0x1430789c8, 0x142409b50, 0x1425d4730, 0x141834d18> as *const ()), // sub_140012e50
    Replacement::draft(0x140012eb0, "unknown_140012eb0", tpl_39::<0x143a0d430, 0x143a0d433, 0x1430789d8, 0x142409b50, 0x1425d4740, 0x141834d18> as *const ()), // sub_140012eb0
    Replacement::draft(0x140012f10, "unknown_140012f10", tpl_41::<0x143a0d748, 0x143a0d74b, 0x1430789e0, 0x142409b50, 0x1425d4750, 0x141834d18> as *const ()), // sub_140012f10
    Replacement::draft(0x140012f70, "unknown_140012f70", tpl_34::<0x143a0d9e8, 0x143a0d9eb, 0x1430789f0, 0x142409b50, 0x1425d4760, 0x141834d18> as *const ()), // sub_140012f70
    Replacement::draft(0x140012fd0, "unknown_140012fd0", tpl_48::<0x143a0d680, 0x143a0d683, 0x143078a00, 0x142409b50, 0x1425d4770, 0x141834d18> as *const ()), // sub_140012fd0
    Replacement::draft(0x140013030, "unknown_140013030", tpl_46::<0x143a0db00, 0x143a0db03, 0x143078a18, 0x142409b50, 0x1425d4780, 0x141834d18> as *const ()), // sub_140013030
    Replacement::draft(0x140013090, "unknown_140013090", tpl_47::<0x143a0da60, 0x143a0da63, 0x143078a30, 0x142409b50, 0x1425d4790, 0x141834d18> as *const ()), // sub_140013090
    Replacement::draft(0x1400130f0, "unknown_1400130f0", tpl_37::<0x143a0d5b8, 0x143a0d5bb, 0x143078a50, 0x142409b50, 0x1425d47a0, 0x141834d18> as *const ()), // sub_1400130f0
    Replacement::draft(0x140013150, "unknown_140013150", tpl_45::<0x143a0d810, 0x143a0d813, 0x143078a60, 0x142409b50, 0x1425d47b0, 0x141834d18> as *const ()), // sub_140013150
    Replacement::draft(0x1400131b0, "unknown_1400131b0", tpl_54::<0x143a0d920, 0x143a0d923, 0x143078a70, 0x142409b50, 0x1425d47c0, 0x141834d18> as *const ()), // sub_1400131b0
    Replacement::draft(0x140013210, "unknown_140013210", tpl_40::<0x143a0d590, 0x143a0d593, 0x143078a88, 0x142409b50, 0x1425d47d0, 0x141834d18> as *const ()), // sub_140013210
    Replacement::draft(0x140013270, "unknown_140013270", tpl_50::<0x143a0d5e0, 0x143a0d5e3, 0x143078a90, 0x142409b50, 0x1425d47e0, 0x141834d18> as *const ()), // sub_140013270
    Replacement::draft(0x1400132d0, "unknown_1400132d0", tpl_41::<0x143a0d798, 0x143a0d79b, 0x143078aa8, 0x142409b50, 0x1425d47f0, 0x141834d18> as *const ()), // sub_1400132d0
    Replacement::draft(0x140013330, "unknown_140013330", tpl_55::<0x143a0da88, 0x143a0da8b, 0x143078ab8, 0x142409b50, 0x1425d4800, 0x141834d18> as *const ()), // sub_140013330
    Replacement::draft(0x140013390, "unknown_140013390", tpl_41::<0x143a0db28, 0x143a0db2b, 0x143078ac8, 0x142409b50, 0x1425d4810, 0x141834d18> as *const ()), // sub_140013390
    Replacement::draft(0x1400133f0, "unknown_1400133f0", tpl_49::<0x143a0dbc8, 0x143a0dbcb, 0x143078ad8, 0x142409b50, 0x1425d4820, 0x141834d18> as *const ()), // sub_1400133f0
    Replacement::draft(0x140013450, "unknown_140013450", tpl_40::<0x143a0d3e0, 0x143a0d3e3, 0x143078ae4, 0x142409b50, 0x1425d4830, 0x141834d18> as *const ()), // sub_140013450
    Replacement::draft(0x1400134b0, "unknown_1400134b0", tpl_44::<0x143a0d518, 0x143a0d51b, 0x143078af0, 0x142409b50, 0x1425d4840, 0x141834d18> as *const ()), // sub_1400134b0
    Replacement::draft(0x140013510, "unknown_140013510", tpl_46::<0x143a0d3b8, 0x143a0d3bb, 0x143078b08, 0x142409b50, 0x1425d4850, 0x141834d18> as *const ()), // sub_140013510
    Replacement::draft(0x140013570, "unknown_140013570", tpl_55::<0x143a0dab0, 0x143a0dab3, 0x143078b20, 0x142409b50, 0x1425d4860, 0x141834d18> as *const ()), // sub_140013570
    Replacement::draft(0x1400135d0, "unknown_1400135d0", tpl_46::<0x143a0d948, 0x143a0d94b, 0x143078b30, 0x142409b50, 0x1425d4870, 0x141834d18> as *const ()), // sub_1400135d0
    Replacement::draft(0x140013630, "unknown_140013630", tpl_50::<0x143a0d6a8, 0x143a0d6ab, 0x143078b48, 0x142409b50, 0x1425d4880, 0x141834d18> as *const ()), // sub_140013630
    Replacement::draft(0x140013690, "unknown_140013690", tpl_42::<0x143a0d368, 0x143a0d36b, 0x143078b60, 0x142409b50, 0x1425d4890, 0x141834d18> as *const ()), // sub_140013690
    Replacement::draft(0x1400136f0, "unknown_1400136f0", tpl_38::<0x143a0da10, 0x143a0da13, 0x143078b78, 0x142409b50, 0x1425d48a0, 0x141834d18> as *const ()), // sub_1400136f0
    Replacement::draft(0x140013750, "unknown_140013750", tpl_34::<0x143a0dd80, 0x143a0dd83, 0x143078b88, 0x142409b50, 0x1425d48b0, 0x141834d18> as *const ()), // sub_140013750
    Replacement::draft(0x1400137b0, "unknown_1400137b0", tpl_41::<0x143a0db78, 0x143a0db7b, 0x143078b98, 0x142409b50, 0x1425d48c0, 0x141834d18> as *const ()), // sub_1400137b0
    Replacement::draft(0x140013810, "unknown_140013810", tpl_36::<0x143a0d8f8, 0x143a0d8fb, 0x143078ba8, 0x142409b50, 0x1425d48d0, 0x141834d18> as *const ()), // sub_140013810
    Replacement::draft(0x140013870, "unknown_140013870", tpl_34::<0x143a0d390, 0x143a0d393, 0x143078bb8, 0x142409b50, 0x1425d48e0, 0x141834d18> as *const ()), // sub_140013870
    Replacement::draft(0x1400139f0, "unknown_1400139f0", tpl_23::<0x143a0de48, 0x143a0de4b, 0x1426cdcb0, 0x142409b50, 0x1425d48f0, 0x141834d18> as *const ()), // sub_1400139f0
    Replacement::draft(0x140013a50, "unknown_140013a50", tpl_23::<0x143a0de90, 0x143a0de93, 0x1426cdcb0, 0x142409b50, 0x1425d4900, 0x141834d18> as *const ()), // sub_140013a50
    Replacement::draft(0x140013c40, "unknown_140013c40", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843790, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d4940, 0x141834d18> as *const ()), // sub_140013c40
    Replacement::draft(0x140014040, "unknown_140014040", tpl_34::<0x143a0df30, 0x143a0df33, 0x143078668, 0x142409b50, 0x1425d49b0, 0x141834d18> as *const ()), // sub_140014040
    Replacement::draft(0x1400140a0, "unknown_1400140a0", tpl_23::<0x143a0e268, 0x143a0e26b, 0x1426cdcb0, 0x142409b50, 0x1425d49c0, 0x141834d18> as *const ()), // sub_1400140a0
    Replacement::draft(0x140014100, "unknown_140014100", tpl_35::<0x143a0e118, 0x143a0e11b, 0x143078678, 0x142409b50, 0x1425d49d0, 0x141834d18> as *const ()), // sub_140014100
    Replacement::draft(0x140014160, "unknown_140014160", tpl_36::<0x143a0e218, 0x143a0e21b, 0x143078680, 0x142409b50, 0x1425d49e0, 0x141834d18> as *const ()), // sub_140014160
    Replacement::draft(0x1400141c0, "unknown_1400141c0", tpl_37::<0x143a0e058, 0x143a0e05b, 0x143078690, 0x142409b50, 0x1425d49f0, 0x141834d18> as *const ()), // sub_1400141c0
    Replacement::draft(0x140014220, "unknown_140014220", tpl_38::<0x143a0df80, 0x143a0df83, 0x1430786a0, 0x142409b50, 0x1425d4a00, 0x141834d18> as *const ()), // sub_140014220
    Replacement::draft(0x140014280, "unknown_140014280", tpl_39::<0x143a0e290, 0x143a0e293, 0x1430786b0, 0x142409b50, 0x1425d4a10, 0x141834d18> as *const ()), // sub_140014280
    Replacement::draft(0x1400142e0, "unknown_1400142e0", tpl_37::<0x143a0dee0, 0x143a0dee3, 0x1430786b8, 0x142409b50, 0x1425d4a20, 0x141834d18> as *const ()), // sub_1400142e0
    Replacement::draft(0x140014340, "unknown_140014340", tpl_40::<0x143a0e330, 0x143a0e333, 0x1430786c4, 0x142409b50, 0x1425d4a30, 0x141834d18> as *const ()), // sub_140014340
    Replacement::draft(0x1400143a0, "unknown_1400143a0", tpl_41::<0x143a0e008, 0x143a0e00b, 0x1430786d0, 0x142409b50, 0x1425d4a40, 0x141834d18> as *const ()), // sub_1400143a0
    Replacement::draft(0x140014400, "unknown_140014400", tpl_42::<0x143a0e2e0, 0x143a0e2e3, 0x1430786e0, 0x142409b50, 0x1425d4a50, 0x141834d18> as *const ()), // sub_140014400
    Replacement::draft(0x140014460, "unknown_140014460", tpl_34::<0x143a0e308, 0x143a0e30b, 0x1430786f8, 0x142409b50, 0x1425d4a60, 0x141834d18> as *const ()), // sub_140014460
    Replacement::draft(0x1400144c0, "unknown_1400144c0", tpl_34::<0x143a0df08, 0x143a0df0b, 0x143078708, 0x142409b50, 0x1425d4a70, 0x141834d18> as *const ()), // sub_1400144c0
    Replacement::draft(0x140014520, "unknown_140014520", tpl_41::<0x143a0dfa8, 0x143a0dfab, 0x143078718, 0x142409b50, 0x1425d4a80, 0x141834d18> as *const ()), // sub_140014520
    Replacement::draft(0x140014580, "unknown_140014580", tpl_43::<0x143a0e080, 0x143a0e083, 0x143078728, 0x142409b50, 0x1425d4a90, 0x141834d18> as *const ()), // sub_140014580
    Replacement::draft(0x1400145e0, "unknown_1400145e0", tpl_44::<0x143a0ddf8, 0x143a0ddfb, 0x143078748, 0x142409b50, 0x1425d4aa0, 0x141834d18> as *const ()), // sub_1400145e0
    Replacement::draft(0x140014640, "unknown_140014640", tpl_39::<0x143a0e030, 0x143a0e033, 0x143078760, 0x142409b50, 0x1425d4ab0, 0x141834d18> as *const ()), // sub_140014640
    Replacement::draft(0x1400146a0, "unknown_1400146a0", tpl_45::<0x143a0df58, 0x143a0df5b, 0x143078768, 0x142409b50, 0x1425d4ac0, 0x141834d18> as *const ()), // sub_1400146a0
    Replacement::draft(0x140014700, "unknown_140014700", tpl_41::<0x143a0e1f0, 0x143a0e1f3, 0x143078778, 0x142409b50, 0x1425d4ad0, 0x141834d18> as *const ()), // sub_140014700
    Replacement::draft(0x140014760, "unknown_140014760", tpl_37::<0x143a0de20, 0x143a0de23, 0x143078788, 0x142409b50, 0x1425d4ae0, 0x141834d18> as *const ()), // sub_140014760
    Replacement::draft(0x1400147c0, "unknown_1400147c0", tpl_46::<0x143a0e178, 0x143a0e17b, 0x143078798, 0x142409b50, 0x1425d4af0, 0x141834d18> as *const ()), // sub_1400147c0
    Replacement::draft(0x140014820, "unknown_140014820", tpl_47::<0x143a0deb8, 0x143a0debb, 0x1430787b0, 0x142409b50, 0x1425d4b00, 0x141834d18> as *const ()), // sub_140014820
    Replacement::draft(0x140014880, "unknown_140014880", tpl_38::<0x143a0e1a0, 0x143a0e1a3, 0x1430787d0, 0x142409b50, 0x1425d4b10, 0x141834d18> as *const ()), // sub_140014880
    Replacement::draft(0x1400148e0, "unknown_1400148e0", tpl_48::<0x143a0e2b8, 0x143a0e2bb, 0x1430787e0, 0x142409b50, 0x1425d4b20, 0x141834d18> as *const ()), // sub_1400148e0
    Replacement::draft(0x140014940, "unknown_140014940", tpl_49::<0x143a0e0f0, 0x143a0e0f3, 0x1430787f8, 0x142409b50, 0x1425d4b30, 0x141834d18> as *const ()), // sub_140014940
    Replacement::draft(0x1400149a0, "unknown_1400149a0", tpl_44::<0x143a0e240, 0x143a0e243, 0x143078808, 0x142409b50, 0x1425d4b40, 0x141834d18> as *const ()), // sub_1400149a0
    Replacement::draft(0x140014a00, "unknown_140014a00", tpl_42::<0x143a0ddd0, 0x143a0ddd3, 0x143078820, 0x142409b50, 0x1425d4b50, 0x141834d18> as *const ()), // sub_140014a00
    Replacement::draft(0x140014a60, "unknown_140014a60", tpl_50::<0x143a0e1c8, 0x143a0e1cb, 0x143078838, 0x142409b50, 0x1425d4b60, 0x141834d18> as *const ()), // sub_140014a60
    Replacement::draft(0x140014ee0, "unknown_140014ee0", tpl_23::<0x143a0e358, 0x143a0e35b, 0x1426cdcb0, 0x142409b50, 0x1425d4b90, 0x141834d18> as *const ()), // sub_140014ee0
    Replacement::draft(0x140014f40, "unknown_140014f40", tpl_23::<0x143a0e3a0, 0x143a0e3a3, 0x1426cdcb0, 0x142409b50, 0x1425d4ba0, 0x141834d18> as *const ()), // sub_140014f40
    Replacement::draft(0x140015130, "unknown_140015130", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843830, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d4be0, 0x141834d18> as *const ()), // sub_140015130
    Replacement::draft(0x1400154f0, "unknown_1400154f0", tpl_23::<0x143a0e5f0, 0x143a0e5f3, 0x1426cdcb0, 0x142409b50, 0x1425d4c20, 0x141834d18> as *const ()), // sub_1400154f0
    Replacement::draft(0x140015550, "unknown_140015550", tpl_23::<0x143a0e638, 0x143a0e63b, 0x1426cdcb0, 0x142409b50, 0x1425d4c30, 0x141834d18> as *const ()), // sub_140015550
    Replacement::draft(0x140015740, "unknown_140015740", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438438d0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d4c70, 0x141834d18> as *const ()), // sub_140015740
    Replacement::draft(0x140015b40, "unknown_140015b40", tpl_34::<0x143a0e7a0, 0x143a0e7a3, 0x143078668, 0x142409b50, 0x1425d4ce0, 0x141834d18> as *const ()), // sub_140015b40
    Replacement::draft(0x140015ba0, "unknown_140015ba0", tpl_23::<0x143a0eda8, 0x143a0edab, 0x1426cdcb0, 0x142409b50, 0x1425d4cf0, 0x141834d18> as *const ()), // sub_140015ba0
    Replacement::draft(0x140015c00, "unknown_140015c00", tpl_35::<0x143a0ead8, 0x143a0eadb, 0x143078678, 0x142409b50, 0x1425d4d00, 0x141834d18> as *const ()), // sub_140015c00
    Replacement::draft(0x140015c60, "unknown_140015c60", tpl_36::<0x143a0ed30, 0x143a0ed33, 0x143078680, 0x142409b50, 0x1425d4d10, 0x141834d18> as *const ()), // sub_140015c60
    Replacement::draft(0x140015cc0, "unknown_140015cc0", tpl_37::<0x143a0e980, 0x143a0e983, 0x143078690, 0x142409b50, 0x1425d4d20, 0x141834d18> as *const ()), // sub_140015cc0
    Replacement::draft(0x140015d20, "unknown_140015d20", tpl_38::<0x143a0e840, 0x143a0e843, 0x1430786a0, 0x142409b50, 0x1425d4d30, 0x141834d18> as *const ()), // sub_140015d20
    Replacement::draft(0x140015d80, "unknown_140015d80", tpl_39::<0x143a0edd0, 0x143a0edd3, 0x1430786b0, 0x142409b50, 0x1425d4d40, 0x141834d18> as *const ()), // sub_140015d80
    Replacement::draft(0x140015de0, "unknown_140015de0", tpl_37::<0x143a0e750, 0x143a0e753, 0x1430786b8, 0x142409b50, 0x1425d4d50, 0x141834d18> as *const ()), // sub_140015de0
    Replacement::draft(0x140015e40, "unknown_140015e40", tpl_40::<0x143a0eee8, 0x143a0eeeb, 0x1430786c4, 0x142409b50, 0x1425d4d60, 0x141834d18> as *const ()), // sub_140015e40
    Replacement::draft(0x140015ea0, "unknown_140015ea0", tpl_41::<0x143a0e908, 0x143a0e90b, 0x1430786d0, 0x142409b50, 0x1425d4d70, 0x141834d18> as *const ()), // sub_140015ea0
    Replacement::draft(0x140015f00, "unknown_140015f00", tpl_42::<0x143a0ee20, 0x143a0ee23, 0x1430786e0, 0x142409b50, 0x1425d4d80, 0x141834d18> as *const ()), // sub_140015f00
    Replacement::draft(0x140015f60, "unknown_140015f60", tpl_34::<0x143a0ee70, 0x143a0ee73, 0x1430786f8, 0x142409b50, 0x1425d4d90, 0x141834d18> as *const ()), // sub_140015f60
    Replacement::draft(0x140015fc0, "unknown_140015fc0", tpl_34::<0x143a0e778, 0x143a0e77b, 0x143078708, 0x142409b50, 0x1425d4da0, 0x141834d18> as *const ()), // sub_140015fc0
    Replacement::draft(0x140016020, "unknown_140016020", tpl_41::<0x143a0e868, 0x143a0e86b, 0x143078718, 0x142409b50, 0x1425d4db0, 0x141834d18> as *const ()), // sub_140016020
    Replacement::draft(0x140016080, "unknown_140016080", tpl_43::<0x143a0e9a8, 0x143a0e9ab, 0x143078728, 0x142409b50, 0x1425d4dc0, 0x141834d18> as *const ()), // sub_140016080
    Replacement::draft(0x1400160e0, "unknown_1400160e0", tpl_44::<0x143a0e428, 0x143a0e42b, 0x143078748, 0x142409b50, 0x1425d4dd0, 0x141834d18> as *const ()), // sub_1400160e0
    Replacement::draft(0x140016140, "unknown_140016140", tpl_39::<0x143a0e930, 0x143a0e933, 0x143078760, 0x142409b50, 0x1425d4de0, 0x141834d18> as *const ()), // sub_140016140
    Replacement::draft(0x1400161a0, "unknown_1400161a0", tpl_45::<0x143a0e818, 0x143a0e81b, 0x143078768, 0x142409b50, 0x1425d4df0, 0x141834d18> as *const ()), // sub_1400161a0
    Replacement::draft(0x140016200, "unknown_140016200", tpl_41::<0x143a0ece0, 0x143a0ece3, 0x143078778, 0x142409b50, 0x1425d4e00, 0x141834d18> as *const ()), // sub_140016200
    Replacement::draft(0x140016260, "unknown_140016260", tpl_37::<0x143a0e450, 0x143a0e453, 0x143078788, 0x142409b50, 0x1425d4e10, 0x141834d18> as *const ()), // sub_140016260
    Replacement::draft(0x1400162c0, "unknown_1400162c0", tpl_46::<0x143a0eb78, 0x143a0eb7b, 0x143078798, 0x142409b50, 0x1425d4e20, 0x141834d18> as *const ()), // sub_1400162c0
    Replacement::draft(0x140016320, "unknown_140016320", tpl_47::<0x143a0e6b0, 0x143a0e6b3, 0x1430787b0, 0x142409b50, 0x1425d4e30, 0x141834d18> as *const ()), // sub_140016320
    Replacement::draft(0x140016380, "unknown_140016380", tpl_38::<0x143a0ec18, 0x143a0ec1b, 0x1430787d0, 0x142409b50, 0x1425d4e40, 0x141834d18> as *const ()), // sub_140016380
    Replacement::draft(0x1400163e0, "unknown_1400163e0", tpl_48::<0x143a0edf8, 0x143a0edfb, 0x1430787e0, 0x142409b50, 0x1425d4e50, 0x141834d18> as *const ()), // sub_1400163e0
    Replacement::draft(0x140016440, "unknown_140016440", tpl_49::<0x143a0eab0, 0x143a0eab3, 0x1430787f8, 0x142409b50, 0x1425d4e60, 0x141834d18> as *const ()), // sub_140016440
    Replacement::draft(0x1400164a0, "unknown_1400164a0", tpl_44::<0x143a0ed80, 0x143a0ed83, 0x143078808, 0x142409b50, 0x1425d4e70, 0x141834d18> as *const ()), // sub_1400164a0
    Replacement::draft(0x140016500, "unknown_140016500", tpl_42::<0x143a0e400, 0x143a0e403, 0x143078820, 0x142409b50, 0x1425d4e80, 0x141834d18> as *const ()), // sub_140016500
    Replacement::draft(0x140016560, "unknown_140016560", tpl_50::<0x143a0ec90, 0x143a0ec93, 0x143078838, 0x142409b50, 0x1425d4e90, 0x141834d18> as *const ()), // sub_140016560
    Replacement::draft(0x1400165c0, "unknown_1400165c0", tpl_52::<0x143a0e590, 0x143a0e593, 0x143a0eee8, 0x143a0eeec, 0x143a0eeeb, 0x143a0eef0, 0x142409b50, 0x1425d4ea0, 0x141834d18> as *const ()), // sub_1400165c0
    Replacement::draft(0x140016680, "unknown_140016680", tpl_38::<0x143a0eb00, 0x143a0eb03, 0x143078950, 0x142409b50, 0x1425d4eb0, 0x141834d18> as *const ()), // sub_140016680
    Replacement::draft(0x1400166e0, "unknown_1400166e0", tpl_53::<0x143a0e688, 0x143a0e68b, 0x143078960, 0x142409b50, 0x1425d4ec0, 0x141834d18> as *const ()), // sub_1400166e0
    Replacement::draft(0x140016740, "unknown_140016740", tpl_45::<0x143a0e518, 0x143a0e51b, 0x143078980, 0x142409b50, 0x1425d4ed0, 0x141834d18> as *const ()), // sub_140016740
    Replacement::draft(0x1400167a0, "unknown_1400167a0", tpl_35::<0x143a0e568, 0x143a0e56b, 0x14307898c, 0x142409b50, 0x1425d4ee0, 0x141834d18> as *const ()), // sub_1400167a0
    Replacement::draft(0x140016800, "unknown_140016800", tpl_42::<0x143a0ee48, 0x143a0ee4b, 0x1430786e0, 0x142409b50, 0x1425d4ef0, 0x141834d18> as *const ()), // sub_140016800
    Replacement::draft(0x140016860, "unknown_140016860", tpl_38::<0x143a0ed58, 0x143a0ed5b, 0x143078998, 0x142409b50, 0x1425d4f00, 0x141834d18> as *const ()), // sub_140016860
    Replacement::draft(0x1400168c0, "unknown_1400168c0", tpl_38::<0x143a0e8b8, 0x143a0e8bb, 0x1430789a8, 0x142409b50, 0x1425d4f10, 0x141834d18> as *const ()), // sub_1400168c0
    Replacement::draft(0x140016920, "unknown_140016920", tpl_41::<0x143a0ea10, 0x143a0ea13, 0x1430789b8, 0x142409b50, 0x1425d4f20, 0x141834d18> as *const ()), // sub_140016920
    Replacement::draft(0x140016980, "unknown_140016980", tpl_34::<0x143a0ee98, 0x143a0ee9b, 0x1430789c8, 0x142409b50, 0x1425d4f30, 0x141834d18> as *const ()), // sub_140016980
    Replacement::draft(0x1400169e0, "unknown_1400169e0", tpl_39::<0x143a0e540, 0x143a0e543, 0x1430789d8, 0x142409b50, 0x1425d4f40, 0x141834d18> as *const ()), // sub_1400169e0
    Replacement::draft(0x140016a40, "unknown_140016a40", tpl_41::<0x143a0e890, 0x143a0e893, 0x1430789e0, 0x142409b50, 0x1425d4f50, 0x141834d18> as *const ()), // sub_140016a40
    Replacement::draft(0x140016aa0, "unknown_140016aa0", tpl_34::<0x143a0eb28, 0x143a0eb2b, 0x1430789f0, 0x142409b50, 0x1425d4f60, 0x141834d18> as *const ()), // sub_140016aa0
    Replacement::draft(0x140016b00, "unknown_140016b00", tpl_48::<0x143a0e7c8, 0x143a0e7cb, 0x143078a00, 0x142409b50, 0x1425d4f70, 0x141834d18> as *const ()), // sub_140016b00
    Replacement::draft(0x140016b60, "unknown_140016b60", tpl_46::<0x143a0ec40, 0x143a0ec43, 0x143078a18, 0x142409b50, 0x1425d4f80, 0x141834d18> as *const ()), // sub_140016b60
    Replacement::draft(0x140016bc0, "unknown_140016bc0", tpl_47::<0x143a0eba0, 0x143a0eba3, 0x143078a30, 0x142409b50, 0x1425d4f90, 0x141834d18> as *const ()), // sub_140016bc0
    Replacement::draft(0x140016c20, "unknown_140016c20", tpl_37::<0x143a0e700, 0x143a0e703, 0x143078a50, 0x142409b50, 0x1425d4fa0, 0x141834d18> as *const ()), // sub_140016c20
    Replacement::draft(0x140016c80, "unknown_140016c80", tpl_45::<0x143a0e958, 0x143a0e95b, 0x143078a60, 0x142409b50, 0x1425d4fb0, 0x141834d18> as *const ()), // sub_140016c80
    Replacement::draft(0x140016ce0, "unknown_140016ce0", tpl_54::<0x143a0ea60, 0x143a0ea63, 0x143078a70, 0x142409b50, 0x1425d4fc0, 0x141834d18> as *const ()), // sub_140016ce0
    Replacement::draft(0x140016d40, "unknown_140016d40", tpl_40::<0x143a0e6d8, 0x143a0e6db, 0x143078a88, 0x142409b50, 0x1425d4fd0, 0x141834d18> as *const ()), // sub_140016d40
    Replacement::draft(0x140016da0, "unknown_140016da0", tpl_50::<0x143a0e728, 0x143a0e72b, 0x143078a90, 0x142409b50, 0x1425d4fe0, 0x141834d18> as *const ()), // sub_140016da0
    Replacement::draft(0x140016e00, "unknown_140016e00", tpl_41::<0x143a0e8e0, 0x143a0e8e3, 0x143078aa8, 0x142409b50, 0x1425d4ff0, 0x141834d18> as *const ()), // sub_140016e00
    Replacement::draft(0x140016e60, "unknown_140016e60", tpl_55::<0x143a0ebc8, 0x143a0ebcb, 0x143078ab8, 0x142409b50, 0x1425d5000, 0x141834d18> as *const ()), // sub_140016e60
    Replacement::draft(0x140016ec0, "unknown_140016ec0", tpl_41::<0x143a0ec68, 0x143a0ec6b, 0x143078ac8, 0x142409b50, 0x1425d5010, 0x141834d18> as *const ()), // sub_140016ec0
    Replacement::draft(0x140016f20, "unknown_140016f20", tpl_49::<0x143a0ed08, 0x143a0ed0b, 0x143078ad8, 0x142409b50, 0x1425d5020, 0x141834d18> as *const ()), // sub_140016f20
    Replacement::draft(0x140016f80, "unknown_140016f80", tpl_40::<0x143a0e4f0, 0x143a0e4f3, 0x143078ae4, 0x142409b50, 0x1425d5030, 0x141834d18> as *const ()), // sub_140016f80
    Replacement::draft(0x140016fe0, "unknown_140016fe0", tpl_44::<0x143a0e660, 0x143a0e663, 0x143078af0, 0x142409b50, 0x1425d5040, 0x141834d18> as *const ()), // sub_140016fe0
    Replacement::draft(0x140017040, "unknown_140017040", tpl_46::<0x143a0e4c8, 0x143a0e4cb, 0x143078b08, 0x142409b50, 0x1425d5050, 0x141834d18> as *const ()), // sub_140017040
    Replacement::draft(0x1400170a0, "unknown_1400170a0", tpl_55::<0x143a0ebf0, 0x143a0ebf3, 0x143078b20, 0x142409b50, 0x1425d5060, 0x141834d18> as *const ()), // sub_1400170a0
    Replacement::draft(0x140017100, "unknown_140017100", tpl_46::<0x143a0ea88, 0x143a0ea8b, 0x143078b30, 0x142409b50, 0x1425d5070, 0x141834d18> as *const ()), // sub_140017100
    Replacement::draft(0x140017160, "unknown_140017160", tpl_50::<0x143a0e7f0, 0x143a0e7f3, 0x143078b48, 0x142409b50, 0x1425d5080, 0x141834d18> as *const ()), // sub_140017160
    Replacement::draft(0x1400171c0, "unknown_1400171c0", tpl_42::<0x143a0e478, 0x143a0e47b, 0x143078b60, 0x142409b50, 0x1425d5090, 0x141834d18> as *const ()), // sub_1400171c0
    Replacement::draft(0x140017220, "unknown_140017220", tpl_38::<0x143a0eb50, 0x143a0eb53, 0x143078b78, 0x142409b50, 0x1425d50a0, 0x141834d18> as *const ()), // sub_140017220
    Replacement::draft(0x140017280, "unknown_140017280", tpl_34::<0x143a0eec0, 0x143a0eec3, 0x143078b88, 0x142409b50, 0x1425d50b0, 0x141834d18> as *const ()), // sub_140017280
    Replacement::draft(0x1400172e0, "unknown_1400172e0", tpl_41::<0x143a0ecb8, 0x143a0ecbb, 0x143078b98, 0x142409b50, 0x1425d50c0, 0x141834d18> as *const ()), // sub_1400172e0
    Replacement::draft(0x140017340, "unknown_140017340", tpl_36::<0x143a0ea38, 0x143a0ea3b, 0x143078ba8, 0x142409b50, 0x1425d50d0, 0x141834d18> as *const ()), // sub_140017340
    Replacement::draft(0x1400173a0, "unknown_1400173a0", tpl_34::<0x143a0e4a0, 0x143a0e4a3, 0x143078bb8, 0x142409b50, 0x1425d50e0, 0x141834d18> as *const ()), // sub_1400173a0
    Replacement::draft(0x140017520, "unknown_140017520", tpl_23::<0x143a0ef10, 0x143a0ef13, 0x1426cdcb0, 0x142409b50, 0x1425d50f0, 0x141834d18> as *const ()), // sub_140017520
    Replacement::draft(0x140017580, "unknown_140017580", tpl_23::<0x143a0ef58, 0x143a0ef5b, 0x1426cdcb0, 0x142409b50, 0x1425d5100, 0x141834d18> as *const ()), // sub_140017580
    Replacement::draft(0x140017680, "unknown_140017680", tpl_23::<0x143a0efb8, 0x143a0efbb, 0x1426cdcb0, 0x142409b50, 0x1425d5120, 0x141834d18> as *const ()), // sub_140017680
    Replacement::draft(0x1400176e0, "unknown_1400176e0", tpl_23::<0x143a0f000, 0x143a0f003, 0x1426cdcb0, 0x142409b50, 0x1425d5130, 0x141834d18> as *const ()), // sub_1400176e0
    Replacement::draft(0x1400178d0, "unknown_1400178d0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843960, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d5170, 0x141834d18> as *const ()), // sub_1400178d0
    Replacement::draft(0x140017cd0, "unknown_140017cd0", tpl_23::<0x143a0f070, 0x143a0f073, 0x1426cdcb0, 0x142409b50, 0x1425d51e0, 0x141834d18> as *const ()), // sub_140017cd0
    Replacement::draft(0x140017d30, "unknown_140017d30", tpl_23::<0x143a0f0b8, 0x143a0f0bb, 0x1426cdcb0, 0x142409b50, 0x1425d51f0, 0x141834d18> as *const ()), // sub_140017d30
    Replacement::draft(0x140017f20, "unknown_140017f20", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438439f0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d5230, 0x141834d18> as *const ()), // sub_140017f20
    Replacement::draft(0x1400183c0, "unknown_1400183c0", tpl_56::<0x143426780, 0x1434265d4, 0x143a001ac, 0x143843a30, 0x143843a48, 0x143a00290, 0x143843a20, 0x1425d52a0, 0x141834d18> as *const ()), // sub_1400183c0
    Replacement::draft(0x140018650, "unknown_140018650", tpl_23::<0x143a0f328, 0x143a0f32b, 0x1426cdcb0, 0x142409b50, 0x1425d53c0, 0x141834d18> as *const ()), // sub_140018650
    Replacement::draft(0x1400186b0, "unknown_1400186b0", tpl_23::<0x143a0f370, 0x143a0f373, 0x1426cdcb0, 0x142409b50, 0x1425d53d0, 0x141834d18> as *const ()), // sub_1400186b0
    Replacement::draft(0x1400188a0, "unknown_1400188a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843b00, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d5410, 0x141834d18> as *const ()), // sub_1400188a0
    Replacement::draft(0x140018ca0, "unknown_140018ca0", tpl_34::<0x143a0f508, 0x143a0f50b, 0x143078668, 0x142409b50, 0x1425d5480, 0x141834d18> as *const ()), // sub_140018ca0
    Replacement::draft(0x140018d00, "unknown_140018d00", tpl_23::<0x143a0fbf8, 0x143a0fbfb, 0x1426cdcb0, 0x142409b50, 0x1425d5490, 0x141834d18> as *const ()), // sub_140018d00
    Replacement::draft(0x140018d60, "unknown_140018d60", tpl_35::<0x143a0f910, 0x143a0f913, 0x143078678, 0x142409b50, 0x1425d54a0, 0x141834d18> as *const ()), // sub_140018d60
    Replacement::draft(0x140018dc0, "unknown_140018dc0", tpl_36::<0x143a0fb80, 0x143a0fb83, 0x143078680, 0x142409b50, 0x1425d54b0, 0x141834d18> as *const ()), // sub_140018dc0
    Replacement::draft(0x140018e20, "unknown_140018e20", tpl_37::<0x143a0f758, 0x143a0f75b, 0x143078690, 0x142409b50, 0x1425d54c0, 0x141834d18> as *const ()), // sub_140018e20
    Replacement::draft(0x140018e80, "unknown_140018e80", tpl_38::<0x143a0f5e0, 0x143a0f5e3, 0x1430786a0, 0x142409b50, 0x1425d54d0, 0x141834d18> as *const ()), // sub_140018e80
    Replacement::draft(0x140018ee0, "unknown_140018ee0", tpl_39::<0x143a0fc28, 0x143a0fc2b, 0x1430786b0, 0x142409b50, 0x1425d54e0, 0x141834d18> as *const ()), // sub_140018ee0
    Replacement::draft(0x140018f40, "unknown_140018f40", tpl_37::<0x143a0f4b0, 0x143a0f4b3, 0x1430786b8, 0x142409b50, 0x1425d54f0, 0x141834d18> as *const ()), // sub_140018f40
    Replacement::draft(0x140018fa0, "unknown_140018fa0", tpl_40::<0x143a0fd50, 0x143a0fd53, 0x1430786c4, 0x142409b50, 0x1425d5500, 0x141834d18> as *const ()), // sub_140018fa0
    Replacement::draft(0x140019000, "unknown_140019000", tpl_41::<0x143a0f6b8, 0x143a0f6bb, 0x1430786d0, 0x142409b50, 0x1425d5510, 0x141834d18> as *const ()), // sub_140019000
    Replacement::draft(0x140019060, "unknown_140019060", tpl_42::<0x143a0fc80, 0x143a0fc83, 0x1430786e0, 0x142409b50, 0x1425d5520, 0x141834d18> as *const ()), // sub_140019060
    Replacement::draft(0x1400190c0, "unknown_1400190c0", tpl_34::<0x143a0fcd8, 0x143a0fcdb, 0x1430786f8, 0x142409b50, 0x1425d5530, 0x141834d18> as *const ()), // sub_1400190c0
    Replacement::draft(0x140019120, "unknown_140019120", tpl_34::<0x143a0f4e0, 0x143a0f4e3, 0x143078708, 0x142409b50, 0x1425d5540, 0x141834d18> as *const ()), // sub_140019120
    Replacement::draft(0x140019180, "unknown_140019180", tpl_41::<0x143a0f608, 0x143a0f60b, 0x143078718, 0x142409b50, 0x1425d5550, 0x141834d18> as *const ()), // sub_140019180
    Replacement::draft(0x1400191e0, "unknown_1400191e0", tpl_43::<0x143a0f798, 0x143a0f79b, 0x143078728, 0x142409b50, 0x1425d5560, 0x141834d18> as *const ()), // sub_1400191e0
    Replacement::draft(0x140019240, "unknown_140019240", tpl_44::<0x143a0f198, 0x143a0f19b, 0x143078748, 0x142409b50, 0x1425d5570, 0x141834d18> as *const ()), // sub_140019240
    Replacement::draft(0x1400192a0, "unknown_1400192a0", tpl_39::<0x143a0f700, 0x143a0f703, 0x143078760, 0x142409b50, 0x1425d5580, 0x141834d18> as *const ()), // sub_1400192a0
    Replacement::draft(0x140019300, "unknown_140019300", tpl_45::<0x143a0f5b8, 0x143a0f5bb, 0x143078768, 0x142409b50, 0x1425d5590, 0x141834d18> as *const ()), // sub_140019300
    Replacement::draft(0x140019360, "unknown_140019360", tpl_41::<0x143a0fb30, 0x143a0fb33, 0x143078778, 0x142409b50, 0x1425d55a0, 0x141834d18> as *const ()), // sub_140019360
    Replacement::draft(0x1400193c0, "unknown_1400193c0", tpl_37::<0x143a0f1c0, 0x143a0f1c3, 0x143078788, 0x142409b50, 0x1425d55b0, 0x141834d18> as *const ()), // sub_1400193c0
    Replacement::draft(0x140019420, "unknown_140019420", tpl_46::<0x143a0f9b0, 0x143a0f9b3, 0x143078798, 0x142409b50, 0x1425d55c0, 0x141834d18> as *const ()), // sub_140019420
    Replacement::draft(0x140019480, "unknown_140019480", tpl_47::<0x143a0f3e8, 0x143a0f3eb, 0x1430787b0, 0x142409b50, 0x1425d55d0, 0x141834d18> as *const ()), // sub_140019480
    Replacement::draft(0x1400194e0, "unknown_1400194e0", tpl_38::<0x143a0fa50, 0x143a0fa53, 0x1430787d0, 0x142409b50, 0x1425d55e0, 0x141834d18> as *const ()), // sub_1400194e0
    Replacement::draft(0x140019540, "unknown_140019540", tpl_48::<0x143a0fc58, 0x143a0fc5b, 0x1430787e0, 0x142409b50, 0x1425d55f0, 0x141834d18> as *const ()), // sub_140019540
    Replacement::draft(0x1400195a0, "unknown_1400195a0", tpl_49::<0x143a0f8e8, 0x143a0f8eb, 0x1430787f8, 0x142409b50, 0x1425d5600, 0x141834d18> as *const ()), // sub_1400195a0
    Replacement::draft(0x140019600, "unknown_140019600", tpl_44::<0x143a0fbd0, 0x143a0fbd3, 0x143078808, 0x142409b50, 0x1425d5610, 0x141834d18> as *const ()), // sub_140019600
    Replacement::draft(0x140019660, "unknown_140019660", tpl_42::<0x143a0f170, 0x143a0f173, 0x143078820, 0x142409b50, 0x1425d5620, 0x141834d18> as *const ()), // sub_140019660
    Replacement::draft(0x1400196c0, "unknown_1400196c0", tpl_50::<0x143a0fad8, 0x143a0fadb, 0x143078838, 0x142409b50, 0x1425d5630, 0x141834d18> as *const ()), // sub_1400196c0
    Replacement::draft(0x140019720, "unknown_140019720", tpl_52::<0x143a0f300, 0x143a0f303, 0x143a0fd50, 0x143a0fd54, 0x143a0fd53, 0x143a0fd58, 0x142409b50, 0x1425d5640, 0x141834d18> as *const ()), // sub_140019720
    Replacement::draft(0x1400197e0, "unknown_1400197e0", tpl_38::<0x143a0f938, 0x143a0f93b, 0x143078950, 0x142409b50, 0x1425d5650, 0x141834d18> as *const ()), // sub_1400197e0
    Replacement::draft(0x140019840, "unknown_140019840", tpl_53::<0x143a0f3c0, 0x143a0f3c3, 0x143078960, 0x142409b50, 0x1425d5660, 0x141834d18> as *const ()), // sub_140019840
    Replacement::draft(0x1400198a0, "unknown_1400198a0", tpl_45::<0x143a0f288, 0x143a0f28b, 0x143078980, 0x142409b50, 0x1425d5670, 0x141834d18> as *const ()), // sub_1400198a0
    Replacement::draft(0x140019900, "unknown_140019900", tpl_35::<0x143a0f2d8, 0x143a0f2db, 0x14307898c, 0x142409b50, 0x1425d5680, 0x141834d18> as *const ()), // sub_140019900
    Replacement::draft(0x140019960, "unknown_140019960", tpl_42::<0x143a0fca8, 0x143a0fcab, 0x1430786e0, 0x142409b50, 0x1425d5690, 0x141834d18> as *const ()), // sub_140019960
    Replacement::draft(0x1400199c0, "unknown_1400199c0", tpl_38::<0x143a0fba8, 0x143a0fbab, 0x143078998, 0x142409b50, 0x1425d56a0, 0x141834d18> as *const ()), // sub_1400199c0
    Replacement::draft(0x140019a20, "unknown_140019a20", tpl_38::<0x143a0f660, 0x143a0f663, 0x1430789a8, 0x142409b50, 0x1425d56b0, 0x141834d18> as *const ()), // sub_140019a20
    Replacement::draft(0x140019a80, "unknown_140019a80", tpl_41::<0x143a0f818, 0x143a0f81b, 0x1430789b8, 0x142409b50, 0x1425d56c0, 0x141834d18> as *const ()), // sub_140019a80
    Replacement::draft(0x140019ae0, "unknown_140019ae0", tpl_34::<0x143a0fd00, 0x143a0fd03, 0x1430789c8, 0x142409b50, 0x1425d56d0, 0x141834d18> as *const ()), // sub_140019ae0
    Replacement::draft(0x140019b40, "unknown_140019b40", tpl_39::<0x143a0f2b0, 0x143a0f2b3, 0x1430789d8, 0x142409b50, 0x1425d56e0, 0x141834d18> as *const ()), // sub_140019b40
    Replacement::draft(0x140019ba0, "unknown_140019ba0", tpl_41::<0x143a0f630, 0x143a0f633, 0x1430789e0, 0x142409b50, 0x1425d56f0, 0x141834d18> as *const ()), // sub_140019ba0
    Replacement::draft(0x140019c00, "unknown_140019c00", tpl_34::<0x143a0f960, 0x143a0f963, 0x1430789f0, 0x142409b50, 0x1425d5700, 0x141834d18> as *const ()), // sub_140019c00
    Replacement::draft(0x140019c60, "unknown_140019c60", tpl_48::<0x143a0f530, 0x143a0f533, 0x143078a00, 0x142409b50, 0x1425d5710, 0x141834d18> as *const ()), // sub_140019c60
    Replacement::draft(0x140019cc0, "unknown_140019cc0", tpl_46::<0x143a0fa80, 0x143a0fa83, 0x143078a18, 0x142409b50, 0x1425d5720, 0x141834d18> as *const ()), // sub_140019cc0
    Replacement::draft(0x140019d20, "unknown_140019d20", tpl_47::<0x143a0f9d8, 0x143a0f9db, 0x143078a30, 0x142409b50, 0x1425d5730, 0x141834d18> as *const ()), // sub_140019d20
    Replacement::draft(0x140019d80, "unknown_140019d80", tpl_37::<0x143a0f458, 0x143a0f45b, 0x143078a50, 0x142409b50, 0x1425d5740, 0x141834d18> as *const ()), // sub_140019d80
    Replacement::draft(0x140019de0, "unknown_140019de0", tpl_45::<0x143a0f728, 0x143a0f72b, 0x143078a60, 0x142409b50, 0x1425d5750, 0x141834d18> as *const ()), // sub_140019de0
    Replacement::draft(0x140019e40, "unknown_140019e40", tpl_54::<0x143a0f870, 0x143a0f873, 0x143078a70, 0x142409b50, 0x1425d5760, 0x141834d18> as *const ()), // sub_140019e40
    Replacement::draft(0x140019ea0, "unknown_140019ea0", tpl_40::<0x143a0f428, 0x143a0f42b, 0x143078a88, 0x142409b50, 0x1425d5770, 0x141834d18> as *const ()), // sub_140019ea0
    Replacement::draft(0x140019f00, "unknown_140019f00", tpl_50::<0x143a0f488, 0x143a0f48b, 0x143078a90, 0x142409b50, 0x1425d5780, 0x141834d18> as *const ()), // sub_140019f00
    Replacement::draft(0x140019f60, "unknown_140019f60", tpl_41::<0x143a0f688, 0x143a0f68b, 0x143078aa8, 0x142409b50, 0x1425d5790, 0x141834d18> as *const ()), // sub_140019f60
    Replacement::draft(0x140019fc0, "unknown_140019fc0", tpl_55::<0x143a0fa00, 0x143a0fa03, 0x143078ab8, 0x142409b50, 0x1425d57a0, 0x141834d18> as *const ()), // sub_140019fc0
    Replacement::draft(0x14001a020, "unknown_14001a020", tpl_41::<0x143a0faa8, 0x143a0faab, 0x143078ac8, 0x142409b50, 0x1425d57b0, 0x141834d18> as *const ()), // sub_14001a020
    Replacement::draft(0x14001a080, "unknown_14001a080", tpl_49::<0x143a0fb58, 0x143a0fb5b, 0x143078ad8, 0x142409b50, 0x1425d57c0, 0x141834d18> as *const ()), // sub_14001a080
    Replacement::draft(0x14001a0e0, "unknown_14001a0e0", tpl_40::<0x143a0f260, 0x143a0f263, 0x143078ae4, 0x142409b50, 0x1425d57d0, 0x141834d18> as *const ()), // sub_14001a0e0
    Replacement::draft(0x14001a140, "unknown_14001a140", tpl_44::<0x143a0f398, 0x143a0f39b, 0x143078af0, 0x142409b50, 0x1425d57e0, 0x141834d18> as *const ()), // sub_14001a140
    Replacement::draft(0x14001a1a0, "unknown_14001a1a0", tpl_46::<0x143a0f238, 0x143a0f23b, 0x143078b08, 0x142409b50, 0x1425d57f0, 0x141834d18> as *const ()), // sub_14001a1a0
    Replacement::draft(0x14001a200, "unknown_14001a200", tpl_55::<0x143a0fa28, 0x143a0fa2b, 0x143078b20, 0x142409b50, 0x1425d5800, 0x141834d18> as *const ()), // sub_14001a200
    Replacement::draft(0x14001a260, "unknown_14001a260", tpl_46::<0x143a0f8b8, 0x143a0f8bb, 0x143078b30, 0x142409b50, 0x1425d5810, 0x141834d18> as *const ()), // sub_14001a260
    Replacement::draft(0x14001a2c0, "unknown_14001a2c0", tpl_50::<0x143a0f560, 0x143a0f563, 0x143078b48, 0x142409b50, 0x1425d5820, 0x141834d18> as *const ()), // sub_14001a2c0
    Replacement::draft(0x14001a320, "unknown_14001a320", tpl_42::<0x143a0f1e8, 0x143a0f1eb, 0x143078b60, 0x142409b50, 0x1425d5830, 0x141834d18> as *const ()), // sub_14001a320
    Replacement::draft(0x14001a380, "unknown_14001a380", tpl_38::<0x143a0f988, 0x143a0f98b, 0x143078b78, 0x142409b50, 0x1425d5840, 0x141834d18> as *const ()), // sub_14001a380
    Replacement::draft(0x14001a3e0, "unknown_14001a3e0", tpl_34::<0x143a0fd28, 0x143a0fd2b, 0x143078b88, 0x142409b50, 0x1425d5850, 0x141834d18> as *const ()), // sub_14001a3e0
    Replacement::draft(0x14001a440, "unknown_14001a440", tpl_41::<0x143a0fb00, 0x143a0fb03, 0x143078b98, 0x142409b50, 0x1425d5860, 0x141834d18> as *const ()), // sub_14001a440
    Replacement::draft(0x14001a4a0, "unknown_14001a4a0", tpl_36::<0x143a0f848, 0x143a0f84b, 0x143078ba8, 0x142409b50, 0x1425d5870, 0x141834d18> as *const ()), // sub_14001a4a0
    Replacement::draft(0x14001a500, "unknown_14001a500", tpl_34::<0x143a0f210, 0x143a0f213, 0x143078bb8, 0x142409b50, 0x1425d5880, 0x141834d18> as *const ()), // sub_14001a500
    Replacement::draft_throws(0x14001d520, "unknown_14001d520", fn_14001d520 as *const (), Throws::Caught), // sub_14001d520
    Replacement::draft(0x14001d5b0, "unknown_14001d5b0", tpl_23::<0x143a0fd78, 0x143a0fd7b, 0x1426cdcb0, 0x142409b50, 0x1425d5be0, 0x141834d18> as *const ()), // sub_14001d5b0
    Replacement::draft(0x14001d610, "unknown_14001d610", tpl_23::<0x143a0fdc0, 0x143a0fdc3, 0x1426cdcb0, 0x142409b50, 0x1425d5bf0, 0x141834d18> as *const ()), // sub_14001d610
    Replacement::draft(0x14001d800, "unknown_14001d800", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843be0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d5c30, 0x141834d18> as *const ()), // sub_14001d800
    Replacement::draft(0x14001ff30, "unknown_14001ff30", tpl_23::<0x143a0fe68, 0x143a0fe6b, 0x1426cdcb0, 0x142409b50, 0x1425d6860, 0x141834d18> as *const ()), // sub_14001ff30
    Replacement::draft(0x14001ff90, "unknown_14001ff90", tpl_23::<0x143a0feb0, 0x143a0feb3, 0x1426cdcb0, 0x142409b50, 0x1425d6870, 0x141834d18> as *const ()), // sub_14001ff90
    Replacement::draft(0x140020180, "unknown_140020180", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843ec8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d68b0, 0x141834d18> as *const ()), // sub_140020180
    Replacement::draft_throws(0x140020580, "unknown_140020580", fn_140020580 as *const (), Throws::Discard), // sub_140020580
    Replacement::draft(0x140022fd0, "unknown_140022fd0", tpl_23::<0x143a0ff30, 0x143a0ff33, 0x1426cdcb0, 0x142409b50, 0x1425d6b90, 0x141834d18> as *const ()), // sub_140022fd0
    Replacement::draft(0x140023030, "unknown_140023030", tpl_23::<0x143a0ff78, 0x143a0ff7b, 0x1426cdcb0, 0x142409b50, 0x1425d6ba0, 0x141834d18> as *const ()), // sub_140023030
    Replacement::draft(0x140023220, "unknown_140023220", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143843fd0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d6be0, 0x141834d18> as *const ()), // sub_140023220
    Replacement::draft(0x140024c10, "unknown_140024c10", tpl_58::<0x14307aafc, 0x14308d8fd, 0x143a00288, 0x143c8ef60, 0x1438440f8, 0x143c8ef68, 0x140231c00, 0x143c8ef90, 0x143c8ef70, 0x143c8ef80> as *const ()), // sub_140024c10
    Replacement::draft(0x140025f60, "unknown_140025f60", fn_140025f60 as *const ()), // sub_140025f60
    Replacement::draft(0x140026770, "unknown_140026770", fn_140026770 as *const ()), // sub_140026770
    Replacement::draft(0x140026f80, "unknown_140026f80", tpl_23::<0x143a10030, 0x143a10033, 0x1426cdcb0, 0x142409b50, 0x1425d7140, 0x141834d18> as *const ()), // sub_140026f80
    Replacement::draft(0x140026fe0, "unknown_140026fe0", tpl_23::<0x143a10078, 0x143a1007b, 0x1426cdcb0, 0x142409b50, 0x1425d7150, 0x141834d18> as *const ()), // sub_140026fe0
    Replacement::draft(0x1400271d0, "unknown_1400271d0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438441d8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d7190, 0x141834d18> as *const ()), // sub_1400271d0
    Replacement::draft(0x140027b70, "unknown_140027b70", tpl_61::<0x143a10300, 0x143a10303, 0x143091fd8, 0x142409b50, 0x1425d7210, 0x141834d18> as *const ()), // sub_140027b70
    Replacement::draft(0x140027bd0, "unknown_140027bd0", tpl_62::<0x143a10303, 0x143a10300, 0x143a10308, 0x143a10304, 0x143a10160> as *const ()), // sub_140027bd0
    Replacement::draft(0x140027c60, "unknown_140027c60", tpl_63::<0x143a102b0, 0x143a102b3, 0x143091ff0, 0x142409b50, 0x1425d7220, 0x141834d18> as *const ()), // sub_140027c60
    Replacement::draft(0x140027cc0, "unknown_140027cc0", tpl_64::<0x143a101f0, 0x143a101f3, 0x143091fa0, 0x142409b50, 0x1425d7230, 0x141834d18> as *const ()), // sub_140027cc0
    Replacement::draft(0x140027d20, "unknown_140027d20", tpl_65::<0x143a10170, 0x143a10173, 0x143091fc0, 0x142409b50, 0x1425d7240, 0x141834d18> as *const ()), // sub_140027d20
    Replacement::draft(0x140027d80, "unknown_140027d80", tpl_62::<0x143a102b3, 0x143a102b0, 0x143a102b8, 0x143a102b4, 0x143a102a0> as *const ()), // sub_140027d80
    Replacement::draft(0x140027e10, "unknown_140027e10", tpl_62::<0x143a101f3, 0x143a101f0, 0x143a101f8, 0x143a101f4, 0x143a101b8> as *const ()), // sub_140027e10
    Replacement::draft(0x140027ea0, "unknown_140027ea0", tpl_62::<0x143a10173, 0x143a10170, 0x143a10178, 0x143a10174, 0x143a101c0> as *const ()), // sub_140027ea0
    Replacement::draft(0x140027f30, "unknown_140027f30", tpl_23::<0x143a100e0, 0x143a100e3, 0x1426cdcb0, 0x142409b50, 0x1425d7250, 0x141834d18> as *const ()), // sub_140027f30
    Replacement::draft(0x140027f90, "unknown_140027f90", tpl_23::<0x143a10128, 0x143a1012b, 0x1426cdcb0, 0x142409b50, 0x1425d7260, 0x141834d18> as *const ()), // sub_140027f90
    Replacement::draft(0x140028180, "unknown_140028180", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844268, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d72a0, 0x141834d18> as *const ()), // sub_140028180
    Replacement::draft(0x140029920, "unknown_140029920", tpl_61::<0x143a104c0, 0x143a104c3, 0x143091fd8, 0x142409b50, 0x1425d7470, 0x141834d18> as *const ()), // sub_140029920
    Replacement::draft(0x140029980, "unknown_140029980", tpl_62::<0x143a104c3, 0x143a104c0, 0x143a104c8, 0x143a104c4, 0x143a10360> as *const ()), // sub_140029980
    Replacement::draft(0x140029a10, "unknown_140029a10", tpl_63::<0x143a10470, 0x143a10473, 0x143091ff0, 0x142409b50, 0x1425d7480, 0x141834d18> as *const ()), // sub_140029a10
    Replacement::draft(0x140029a70, "unknown_140029a70", tpl_64::<0x143a103f0, 0x143a103f3, 0x143091fa0, 0x142409b50, 0x1425d7490, 0x141834d18> as *const ()), // sub_140029a70
    Replacement::draft(0x140029ad0, "unknown_140029ad0", tpl_65::<0x143a10370, 0x143a10373, 0x143091fc0, 0x142409b50, 0x1425d74a0, 0x141834d18> as *const ()), // sub_140029ad0
    Replacement::draft(0x140029b30, "unknown_140029b30", tpl_62::<0x143a10473, 0x143a10470, 0x143a10478, 0x143a10474, 0x143a10460> as *const ()), // sub_140029b30
    Replacement::draft(0x140029bc0, "unknown_140029bc0", tpl_62::<0x143a103f3, 0x143a103f0, 0x143a103f8, 0x143a103f4, 0x143a103b8> as *const ()), // sub_140029bc0
    Replacement::draft(0x140029c50, "unknown_140029c50", tpl_62::<0x143a10373, 0x143a10370, 0x143a10378, 0x143a10374, 0x143a103c0> as *const ()), // sub_140029c50
    Replacement::draft(0x14002ac30, "unknown_14002ac30", tpl_61::<0x143a10730, 0x143a10733, 0x143091fd8, 0x142409b50, 0x1425d7540, 0x141834d18> as *const ()), // sub_14002ac30
    Replacement::draft(0x14002ac90, "unknown_14002ac90", tpl_62::<0x143a10733, 0x143a10730, 0x143a10738, 0x143a10734, 0x143a10590> as *const ()), // sub_14002ac90
    Replacement::draft(0x14002ad20, "unknown_14002ad20", tpl_63::<0x143a106e0, 0x143a106e3, 0x143091ff0, 0x142409b50, 0x1425d7550, 0x141834d18> as *const ()), // sub_14002ad20
    Replacement::draft(0x14002ad80, "unknown_14002ad80", tpl_64::<0x143a10620, 0x143a10623, 0x143091fa0, 0x142409b50, 0x1425d7560, 0x141834d18> as *const ()), // sub_14002ad80
    Replacement::draft(0x14002ade0, "unknown_14002ade0", tpl_65::<0x143a105a0, 0x143a105a3, 0x143091fc0, 0x142409b50, 0x1425d7570, 0x141834d18> as *const ()), // sub_14002ade0
    Replacement::draft(0x14002ae40, "unknown_14002ae40", tpl_62::<0x143a106e3, 0x143a106e0, 0x143a106e8, 0x143a106e4, 0x143a106d0> as *const ()), // sub_14002ae40
    Replacement::draft(0x14002aed0, "unknown_14002aed0", tpl_62::<0x143a10623, 0x143a10620, 0x143a10628, 0x143a10624, 0x143a105e8> as *const ()), // sub_14002aed0
    Replacement::draft(0x14002af60, "unknown_14002af60", tpl_62::<0x143a105a3, 0x143a105a0, 0x143a105a8, 0x143a105a4, 0x143a105f0> as *const ()), // sub_14002af60
    Replacement::draft(0x14002aff0, "unknown_14002aff0", tpl_23::<0x143a10508, 0x143a1050b, 0x1426cdcb0, 0x142409b50, 0x1425d7580, 0x141834d18> as *const ()), // sub_14002aff0
    Replacement::draft(0x14002b050, "unknown_14002b050", tpl_23::<0x143a10550, 0x143a10553, 0x1426cdcb0, 0x142409b50, 0x1425d7590, 0x141834d18> as *const ()), // sub_14002b050
    Replacement::draft(0x14002b240, "unknown_14002b240", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844378, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d75d0, 0x141834d18> as *const ()), // sub_14002b240
    Replacement::draft(0x14002c280, "unknown_14002c280", tpl_23::<0x143a10778, 0x143a1077b, 0x1426cdcb0, 0x142409b50, 0x1425d78c0, 0x141834d18> as *const ()), // sub_14002c280
    Replacement::draft(0x14002c2e0, "unknown_14002c2e0", tpl_23::<0x143a107c0, 0x143a107c3, 0x1426cdcb0, 0x142409b50, 0x1425d78d0, 0x141834d18> as *const ()), // sub_14002c2e0
    Replacement::draft(0x14002c4d0, "unknown_14002c4d0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438444b8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d7910, 0x141834d18> as *const ()), // sub_14002c4d0
    Replacement::draft(0x14002ce00, "unknown_14002ce00", tpl_23::<0x143a109a0, 0x143a109a3, 0x1426cdcb0, 0x142409b50, 0x1425d7960, 0x141834d18> as *const ()), // sub_14002ce00
    Replacement::draft(0x14002ce60, "unknown_14002ce60", tpl_23::<0x143a109e8, 0x143a109eb, 0x1426cdcb0, 0x142409b50, 0x1425d7970, 0x141834d18> as *const ()), // sub_14002ce60
    Replacement::draft(0x14002d050, "unknown_14002d050", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438445e8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d79b0, 0x141834d18> as *const ()), // sub_14002d050
    Replacement::draft(0x14002d3e0, "unknown_14002d3e0", tpl_34::<0x143a10b50, 0x143a10b53, 0x143078668, 0x142409b50, 0x1425d79f0, 0x141834d18> as *const ()), // sub_14002d3e0
    Replacement::draft(0x14002d440, "unknown_14002d440", tpl_23::<0x143a11118, 0x143a1111b, 0x1426cdcb0, 0x142409b50, 0x1425d7a00, 0x141834d18> as *const ()), // sub_14002d440
    Replacement::draft(0x14002d4a0, "unknown_14002d4a0", tpl_35::<0x143a10e48, 0x143a10e4b, 0x143078678, 0x142409b50, 0x1425d7a10, 0x141834d18> as *const ()), // sub_14002d4a0
    Replacement::draft(0x14002d500, "unknown_14002d500", tpl_36::<0x143a110a0, 0x143a110a3, 0x143078680, 0x142409b50, 0x1425d7a20, 0x141834d18> as *const ()), // sub_14002d500
    Replacement::draft(0x14002d560, "unknown_14002d560", tpl_37::<0x143a10d30, 0x143a10d33, 0x143078690, 0x142409b50, 0x1425d7a30, 0x141834d18> as *const ()), // sub_14002d560
    Replacement::draft(0x14002d5c0, "unknown_14002d5c0", tpl_38::<0x143a10bf0, 0x143a10bf3, 0x1430786a0, 0x142409b50, 0x1425d7a40, 0x141834d18> as *const ()), // sub_14002d5c0
    Replacement::draft(0x14002d620, "unknown_14002d620", tpl_39::<0x143a11140, 0x143a11143, 0x1430786b0, 0x142409b50, 0x1425d7a50, 0x141834d18> as *const ()), // sub_14002d620
    Replacement::draft(0x14002d680, "unknown_14002d680", tpl_37::<0x143a10b00, 0x143a10b03, 0x1430786b8, 0x142409b50, 0x1425d7a60, 0x141834d18> as *const ()), // sub_14002d680
    Replacement::draft(0x14002d6e0, "unknown_14002d6e0", tpl_40::<0x143a11258, 0x143a1125b, 0x1430786c4, 0x142409b50, 0x1425d7a70, 0x141834d18> as *const ()), // sub_14002d6e0
    Replacement::draft(0x14002d740, "unknown_14002d740", tpl_41::<0x143a10cb8, 0x143a10cbb, 0x1430786d0, 0x142409b50, 0x1425d7a80, 0x141834d18> as *const ()), // sub_14002d740
    Replacement::draft(0x14002d7a0, "unknown_14002d7a0", tpl_42::<0x143a11190, 0x143a11193, 0x1430786e0, 0x142409b50, 0x1425d7a90, 0x141834d18> as *const ()), // sub_14002d7a0
    Replacement::draft(0x14002d800, "unknown_14002d800", tpl_34::<0x143a111e0, 0x143a111e3, 0x1430786f8, 0x142409b50, 0x1425d7aa0, 0x141834d18> as *const ()), // sub_14002d800
    Replacement::draft(0x14002d860, "unknown_14002d860", tpl_34::<0x143a10b28, 0x143a10b2b, 0x143078708, 0x142409b50, 0x1425d7ab0, 0x141834d18> as *const ()), // sub_14002d860
    Replacement::draft(0x14002d8c0, "unknown_14002d8c0", tpl_41::<0x143a10c18, 0x143a10c1b, 0x143078718, 0x142409b50, 0x1425d7ac0, 0x141834d18> as *const ()), // sub_14002d8c0
    Replacement::draft(0x14002d920, "unknown_14002d920", tpl_43::<0x143a10d58, 0x143a10d5b, 0x143078728, 0x142409b50, 0x1425d7ad0, 0x141834d18> as *const ()), // sub_14002d920
    Replacement::draft(0x14002d980, "unknown_14002d980", tpl_44::<0x143a10810, 0x143a10813, 0x143078748, 0x142409b50, 0x1425d7ae0, 0x141834d18> as *const ()), // sub_14002d980
    Replacement::draft(0x14002d9e0, "unknown_14002d9e0", tpl_39::<0x143a10ce0, 0x143a10ce3, 0x143078760, 0x142409b50, 0x1425d7af0, 0x141834d18> as *const ()), // sub_14002d9e0
    Replacement::draft(0x14002da40, "unknown_14002da40", tpl_45::<0x143a10bc8, 0x143a10bcb, 0x143078768, 0x142409b50, 0x1425d7b00, 0x141834d18> as *const ()), // sub_14002da40
    Replacement::draft(0x14002daa0, "unknown_14002daa0", tpl_41::<0x143a11050, 0x143a11053, 0x143078778, 0x142409b50, 0x1425d7b10, 0x141834d18> as *const ()), // sub_14002daa0
    Replacement::draft(0x14002db00, "unknown_14002db00", tpl_37::<0x143a10838, 0x143a1083b, 0x143078788, 0x142409b50, 0x1425d7b20, 0x141834d18> as *const ()), // sub_14002db00
    Replacement::draft(0x14002db60, "unknown_14002db60", tpl_46::<0x143a10ee8, 0x143a10eeb, 0x143078798, 0x142409b50, 0x1425d7b30, 0x141834d18> as *const ()), // sub_14002db60
    Replacement::draft(0x14002dbc0, "unknown_14002dbc0", tpl_47::<0x143a10a60, 0x143a10a63, 0x1430787b0, 0x142409b50, 0x1425d7b40, 0x141834d18> as *const ()), // sub_14002dbc0
    Replacement::draft(0x14002dc20, "unknown_14002dc20", tpl_38::<0x143a10f88, 0x143a10f8b, 0x1430787d0, 0x142409b50, 0x1425d7b50, 0x141834d18> as *const ()), // sub_14002dc20
    Replacement::draft(0x14002dc80, "unknown_14002dc80", tpl_48::<0x143a11168, 0x143a1116b, 0x1430787e0, 0x142409b50, 0x1425d7b60, 0x141834d18> as *const ()), // sub_14002dc80
    Replacement::draft(0x14002dce0, "unknown_14002dce0", tpl_49::<0x143a10e20, 0x143a10e23, 0x1430787f8, 0x142409b50, 0x1425d7b70, 0x141834d18> as *const ()), // sub_14002dce0
    Replacement::draft(0x14002dd40, "unknown_14002dd40", tpl_44::<0x143a110f0, 0x143a110f3, 0x143078808, 0x142409b50, 0x1425d7b80, 0x141834d18> as *const ()), // sub_14002dd40
    Replacement::draft(0x14002dda0, "unknown_14002dda0", tpl_42::<0x143a107e8, 0x143a107eb, 0x143078820, 0x142409b50, 0x1425d7b90, 0x141834d18> as *const ()), // sub_14002dda0
    Replacement::draft(0x14002de00, "unknown_14002de00", tpl_50::<0x143a11000, 0x143a11003, 0x143078838, 0x142409b50, 0x1425d7ba0, 0x141834d18> as *const ()), // sub_14002de00
    Replacement::draft(0x14002de60, "unknown_14002de60", tpl_52::<0x143a10978, 0x143a1097b, 0x143a11258, 0x143a1125c, 0x143a1125b, 0x143a11260, 0x142409b50, 0x1425d7bb0, 0x141834d18> as *const ()), // sub_14002de60
    Replacement::draft(0x14002df20, "unknown_14002df20", tpl_38::<0x143a10e70, 0x143a10e73, 0x143078950, 0x142409b50, 0x1425d7bc0, 0x141834d18> as *const ()), // sub_14002df20
    Replacement::draft(0x14002df80, "unknown_14002df80", tpl_53::<0x143a10a38, 0x143a10a3b, 0x143078960, 0x142409b50, 0x1425d7bd0, 0x141834d18> as *const ()), // sub_14002df80
    Replacement::draft(0x14002dfe0, "unknown_14002dfe0", tpl_45::<0x143a10900, 0x143a10903, 0x143078980, 0x142409b50, 0x1425d7be0, 0x141834d18> as *const ()), // sub_14002dfe0
    Replacement::draft(0x14002e040, "unknown_14002e040", tpl_35::<0x143a10950, 0x143a10953, 0x14307898c, 0x142409b50, 0x1425d7bf0, 0x141834d18> as *const ()), // sub_14002e040
    Replacement::draft(0x14002e0a0, "unknown_14002e0a0", tpl_42::<0x143a111b8, 0x143a111bb, 0x1430786e0, 0x142409b50, 0x1425d7c00, 0x141834d18> as *const ()), // sub_14002e0a0
    Replacement::draft(0x14002e100, "unknown_14002e100", tpl_38::<0x143a110c8, 0x143a110cb, 0x143078998, 0x142409b50, 0x1425d7c10, 0x141834d18> as *const ()), // sub_14002e100
    Replacement::draft(0x14002e160, "unknown_14002e160", tpl_38::<0x143a10c68, 0x143a10c6b, 0x1430789a8, 0x142409b50, 0x1425d7c20, 0x141834d18> as *const ()), // sub_14002e160
    Replacement::draft(0x14002e1c0, "unknown_14002e1c0", tpl_41::<0x143a10d80, 0x143a10d83, 0x1430789b8, 0x142409b50, 0x1425d7c30, 0x141834d18> as *const ()), // sub_14002e1c0
    Replacement::draft(0x14002e220, "unknown_14002e220", tpl_34::<0x143a11208, 0x143a1120b, 0x1430789c8, 0x142409b50, 0x1425d7c40, 0x141834d18> as *const ()), // sub_14002e220
    Replacement::draft(0x14002e280, "unknown_14002e280", tpl_39::<0x143a10928, 0x143a1092b, 0x1430789d8, 0x142409b50, 0x1425d7c50, 0x141834d18> as *const ()), // sub_14002e280
    Replacement::draft(0x14002e2e0, "unknown_14002e2e0", tpl_41::<0x143a10c40, 0x143a10c43, 0x1430789e0, 0x142409b50, 0x1425d7c60, 0x141834d18> as *const ()), // sub_14002e2e0
    Replacement::draft(0x14002e340, "unknown_14002e340", tpl_34::<0x143a10e98, 0x143a10e9b, 0x1430789f0, 0x142409b50, 0x1425d7c70, 0x141834d18> as *const ()), // sub_14002e340
    Replacement::draft(0x14002e3a0, "unknown_14002e3a0", tpl_48::<0x143a10b78, 0x143a10b7b, 0x143078a00, 0x142409b50, 0x1425d7c80, 0x141834d18> as *const ()), // sub_14002e3a0
    Replacement::draft(0x14002e400, "unknown_14002e400", tpl_46::<0x143a10fb0, 0x143a10fb3, 0x143078a18, 0x142409b50, 0x1425d7c90, 0x141834d18> as *const ()), // sub_14002e400
    Replacement::draft(0x14002e460, "unknown_14002e460", tpl_47::<0x143a10f10, 0x143a10f13, 0x143078a30, 0x142409b50, 0x1425d7ca0, 0x141834d18> as *const ()), // sub_14002e460
    Replacement::draft(0x14002e4c0, "unknown_14002e4c0", tpl_37::<0x143a10ab0, 0x143a10ab3, 0x143078a50, 0x142409b50, 0x1425d7cb0, 0x141834d18> as *const ()), // sub_14002e4c0
    Replacement::draft(0x14002e520, "unknown_14002e520", tpl_45::<0x143a10d08, 0x143a10d0b, 0x143078a60, 0x142409b50, 0x1425d7cc0, 0x141834d18> as *const ()), // sub_14002e520
    Replacement::draft(0x14002e580, "unknown_14002e580", tpl_54::<0x143a10dd0, 0x143a10dd3, 0x143078a70, 0x142409b50, 0x1425d7cd0, 0x141834d18> as *const ()), // sub_14002e580
    Replacement::draft(0x14002e5e0, "unknown_14002e5e0", tpl_40::<0x143a10a88, 0x143a10a8b, 0x143078a88, 0x142409b50, 0x1425d7ce0, 0x141834d18> as *const ()), // sub_14002e5e0
    Replacement::draft(0x14002e640, "unknown_14002e640", tpl_50::<0x143a10ad8, 0x143a10adb, 0x143078a90, 0x142409b50, 0x1425d7cf0, 0x141834d18> as *const ()), // sub_14002e640
    Replacement::draft(0x14002e6a0, "unknown_14002e6a0", tpl_41::<0x143a10c90, 0x143a10c93, 0x143078aa8, 0x142409b50, 0x1425d7d00, 0x141834d18> as *const ()), // sub_14002e6a0
    Replacement::draft(0x14002e700, "unknown_14002e700", tpl_55::<0x143a10f38, 0x143a10f3b, 0x143078ab8, 0x142409b50, 0x1425d7d10, 0x141834d18> as *const ()), // sub_14002e700
    Replacement::draft(0x14002e760, "unknown_14002e760", tpl_41::<0x143a10fd8, 0x143a10fdb, 0x143078ac8, 0x142409b50, 0x1425d7d20, 0x141834d18> as *const ()), // sub_14002e760
    Replacement::draft(0x14002e7c0, "unknown_14002e7c0", tpl_49::<0x143a11078, 0x143a1107b, 0x143078ad8, 0x142409b50, 0x1425d7d30, 0x141834d18> as *const ()), // sub_14002e7c0
    Replacement::draft(0x14002e820, "unknown_14002e820", tpl_40::<0x143a108d8, 0x143a108db, 0x143078ae4, 0x142409b50, 0x1425d7d40, 0x141834d18> as *const ()), // sub_14002e820
    Replacement::draft(0x14002e880, "unknown_14002e880", tpl_44::<0x143a10a10, 0x143a10a13, 0x143078af0, 0x142409b50, 0x1425d7d50, 0x141834d18> as *const ()), // sub_14002e880
    Replacement::draft(0x14002e8e0, "unknown_14002e8e0", tpl_46::<0x143a108b0, 0x143a108b3, 0x143078b08, 0x142409b50, 0x1425d7d60, 0x141834d18> as *const ()), // sub_14002e8e0
    Replacement::draft(0x14002e940, "unknown_14002e940", tpl_55::<0x143a10f60, 0x143a10f63, 0x143078b20, 0x142409b50, 0x1425d7d70, 0x141834d18> as *const ()), // sub_14002e940
    Replacement::draft(0x14002e9a0, "unknown_14002e9a0", tpl_46::<0x143a10df8, 0x143a10dfb, 0x143078b30, 0x142409b50, 0x1425d7d80, 0x141834d18> as *const ()), // sub_14002e9a0
    Replacement::draft(0x14002ea00, "unknown_14002ea00", tpl_50::<0x143a10ba0, 0x143a10ba3, 0x143078b48, 0x142409b50, 0x1425d7d90, 0x141834d18> as *const ()), // sub_14002ea00
    Replacement::draft(0x14002ea60, "unknown_14002ea60", tpl_42::<0x143a10860, 0x143a10863, 0x143078b60, 0x142409b50, 0x1425d7da0, 0x141834d18> as *const ()), // sub_14002ea60
    Replacement::draft(0x14002eac0, "unknown_14002eac0", tpl_38::<0x143a10ec0, 0x143a10ec3, 0x143078b78, 0x142409b50, 0x1425d7db0, 0x141834d18> as *const ()), // sub_14002eac0
    Replacement::draft(0x14002eb20, "unknown_14002eb20", tpl_34::<0x143a11230, 0x143a11233, 0x143078b88, 0x142409b50, 0x1425d7dc0, 0x141834d18> as *const ()), // sub_14002eb20
    Replacement::draft(0x14002eb80, "unknown_14002eb80", tpl_41::<0x143a11028, 0x143a1102b, 0x143078b98, 0x142409b50, 0x1425d7dd0, 0x141834d18> as *const ()), // sub_14002eb80
    Replacement::draft(0x14002ebe0, "unknown_14002ebe0", tpl_36::<0x143a10da8, 0x143a10dab, 0x143078ba8, 0x142409b50, 0x1425d7de0, 0x141834d18> as *const ()), // sub_14002ebe0
    Replacement::draft(0x14002ec40, "unknown_14002ec40", tpl_34::<0x143a10888, 0x143a1088b, 0x143078bb8, 0x142409b50, 0x1425d7df0, 0x141834d18> as *const ()), // sub_14002ec40
    Replacement::draft(0x14002f550, "unknown_14002f550", tpl_23::<0x143a112f8, 0x143a112fb, 0x1426cdcb0, 0x142409b50, 0x1425d7eb0, 0x141834d18> as *const ()), // sub_14002f550
    Replacement::draft(0x14002f5b0, "unknown_14002f5b0", tpl_23::<0x143a11340, 0x143a11343, 0x1426cdcb0, 0x142409b50, 0x1425d7ec0, 0x141834d18> as *const ()), // sub_14002f5b0
    Replacement::draft(0x14002f7a0, "unknown_14002f7a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438446d0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d7f00, 0x141834d18> as *const ()), // sub_14002f7a0
    Replacement::draft(0x14002fb30, "unknown_14002fb30", tpl_34::<0x143a113e0, 0x143a113e3, 0x143078668, 0x142409b50, 0x1425d7f40, 0x141834d18> as *const ()), // sub_14002fb30
    Replacement::draft(0x14002fb90, "unknown_14002fb90", tpl_23::<0x143a11660, 0x143a11663, 0x1426cdcb0, 0x142409b50, 0x1425d7f50, 0x141834d18> as *const ()), // sub_14002fb90
    Replacement::draft(0x14002fbf0, "unknown_14002fbf0", tpl_35::<0x143a11548, 0x143a1154b, 0x143078678, 0x142409b50, 0x1425d7f60, 0x141834d18> as *const ()), // sub_14002fbf0
    Replacement::draft(0x14002fc50, "unknown_14002fc50", tpl_36::<0x143a11610, 0x143a11613, 0x143078680, 0x142409b50, 0x1425d7f70, 0x141834d18> as *const ()), // sub_14002fc50
    Replacement::draft(0x14002fcb0, "unknown_14002fcb0", tpl_37::<0x143a114d0, 0x143a114d3, 0x143078690, 0x142409b50, 0x1425d7f80, 0x141834d18> as *const ()), // sub_14002fcb0
    Replacement::draft(0x14002fd10, "unknown_14002fd10", tpl_38::<0x143a11430, 0x143a11433, 0x1430786a0, 0x142409b50, 0x1425d7f90, 0x141834d18> as *const ()), // sub_14002fd10
    Replacement::draft(0x14002fd70, "unknown_14002fd70", tpl_39::<0x143a11688, 0x143a1168b, 0x1430786b0, 0x142409b50, 0x1425d7fa0, 0x141834d18> as *const ()), // sub_14002fd70
    Replacement::draft(0x14002fdd0, "unknown_14002fdd0", tpl_37::<0x143a11390, 0x143a11393, 0x1430786b8, 0x142409b50, 0x1425d7fb0, 0x141834d18> as *const ()), // sub_14002fdd0
    Replacement::draft(0x14002fe30, "unknown_14002fe30", tpl_40::<0x143a11728, 0x143a1172b, 0x1430786c4, 0x142409b50, 0x1425d7fc0, 0x141834d18> as *const ()), // sub_14002fe30
    Replacement::draft(0x14002fe90, "unknown_14002fe90", tpl_41::<0x143a11480, 0x143a11483, 0x1430786d0, 0x142409b50, 0x1425d7fd0, 0x141834d18> as *const ()), // sub_14002fe90
    Replacement::draft(0x14002fef0, "unknown_14002fef0", tpl_42::<0x143a116d8, 0x143a116db, 0x1430786e0, 0x142409b50, 0x1425d7fe0, 0x141834d18> as *const ()), // sub_14002fef0
    Replacement::draft(0x14002ff50, "unknown_14002ff50", tpl_34::<0x143a11700, 0x143a11703, 0x1430786f8, 0x142409b50, 0x1425d7ff0, 0x141834d18> as *const ()), // sub_14002ff50
    Replacement::draft(0x14002ffb0, "unknown_14002ffb0", tpl_34::<0x143a113b8, 0x143a113bb, 0x143078708, 0x142409b50, 0x1425d8000, 0x141834d18> as *const ()), // sub_14002ffb0
    Replacement::draft(0x140030010, "unknown_140030010", tpl_41::<0x143a11458, 0x143a1145b, 0x143078718, 0x142409b50, 0x1425d8010, 0x141834d18> as *const ()), // sub_140030010
    Replacement::draft(0x140030070, "unknown_140030070", tpl_43::<0x143a114f8, 0x143a114fb, 0x143078728, 0x142409b50, 0x1425d8020, 0x141834d18> as *const ()), // sub_140030070
    Replacement::draft(0x1400300d0, "unknown_1400300d0", tpl_44::<0x143a112a8, 0x143a112ab, 0x143078748, 0x142409b50, 0x1425d8030, 0x141834d18> as *const ()), // sub_1400300d0
    Replacement::draft(0x140030130, "unknown_140030130", tpl_39::<0x143a114a8, 0x143a114ab, 0x143078760, 0x142409b50, 0x1425d8040, 0x141834d18> as *const ()), // sub_140030130
    Replacement::draft(0x140030190, "unknown_140030190", tpl_45::<0x143a11408, 0x143a1140b, 0x143078768, 0x142409b50, 0x1425d8050, 0x141834d18> as *const ()), // sub_140030190
    Replacement::draft(0x1400301f0, "unknown_1400301f0", tpl_41::<0x143a115e8, 0x143a115eb, 0x143078778, 0x142409b50, 0x1425d8060, 0x141834d18> as *const ()), // sub_1400301f0
    Replacement::draft(0x140030250, "unknown_140030250", tpl_37::<0x143a112d0, 0x143a112d3, 0x143078788, 0x142409b50, 0x1425d8070, 0x141834d18> as *const ()), // sub_140030250
    Replacement::draft(0x1400302b0, "unknown_1400302b0", tpl_46::<0x143a11570, 0x143a11573, 0x143078798, 0x142409b50, 0x1425d8080, 0x141834d18> as *const ()), // sub_1400302b0
    Replacement::draft(0x140030310, "unknown_140030310", tpl_47::<0x143a11368, 0x143a1136b, 0x1430787b0, 0x142409b50, 0x1425d8090, 0x141834d18> as *const ()), // sub_140030310
    Replacement::draft(0x140030370, "unknown_140030370", tpl_38::<0x143a11598, 0x143a1159b, 0x1430787d0, 0x142409b50, 0x1425d80a0, 0x141834d18> as *const ()), // sub_140030370
    Replacement::draft(0x1400303d0, "unknown_1400303d0", tpl_48::<0x143a116b0, 0x143a116b3, 0x1430787e0, 0x142409b50, 0x1425d80b0, 0x141834d18> as *const ()), // sub_1400303d0
    Replacement::draft(0x140030430, "unknown_140030430", tpl_49::<0x143a11520, 0x143a11523, 0x1430787f8, 0x142409b50, 0x1425d80c0, 0x141834d18> as *const ()), // sub_140030430
    Replacement::draft(0x140030490, "unknown_140030490", tpl_44::<0x143a11638, 0x143a1163b, 0x143078808, 0x142409b50, 0x1425d80d0, 0x141834d18> as *const ()), // sub_140030490
    Replacement::draft(0x1400304f0, "unknown_1400304f0", tpl_42::<0x143a11280, 0x143a11283, 0x143078820, 0x142409b50, 0x1425d80e0, 0x141834d18> as *const ()), // sub_1400304f0
    Replacement::draft(0x140030550, "unknown_140030550", tpl_50::<0x143a115c0, 0x143a115c3, 0x143078838, 0x142409b50, 0x1425d80f0, 0x141834d18> as *const ()), // sub_140030550
    Replacement::draft(0x140031000, "unknown_140031000", tpl_34::<0x143a118b0, 0x143a118b3, 0x143078668, 0x142409b50, 0x1425d8160, 0x141834d18> as *const ()), // sub_140031000
    Replacement::draft(0x140031060, "unknown_140031060", tpl_23::<0x143a11b70, 0x143a11b73, 0x1426cdcb0, 0x142409b50, 0x1425d8170, 0x141834d18> as *const ()), // sub_140031060
    Replacement::draft(0x1400310c0, "unknown_1400310c0", tpl_35::<0x143a11a58, 0x143a11a5b, 0x143078678, 0x142409b50, 0x1425d8180, 0x141834d18> as *const ()), // sub_1400310c0
    Replacement::draft(0x140031120, "unknown_140031120", tpl_36::<0x143a11b20, 0x143a11b23, 0x143078680, 0x142409b50, 0x1425d8190, 0x141834d18> as *const ()), // sub_140031120
    Replacement::draft(0x140031180, "unknown_140031180", tpl_37::<0x143a119a0, 0x143a119a3, 0x143078690, 0x142409b50, 0x1425d81a0, 0x141834d18> as *const ()), // sub_140031180
    Replacement::draft(0x1400311e0, "unknown_1400311e0", tpl_38::<0x143a11900, 0x143a11903, 0x1430786a0, 0x142409b50, 0x1425d81b0, 0x141834d18> as *const ()), // sub_1400311e0
    Replacement::draft(0x140031240, "unknown_140031240", tpl_39::<0x143a11b98, 0x143a11b9b, 0x1430786b0, 0x142409b50, 0x1425d81c0, 0x141834d18> as *const ()), // sub_140031240
    Replacement::draft(0x1400312a0, "unknown_1400312a0", tpl_37::<0x143a11860, 0x143a11863, 0x1430786b8, 0x142409b50, 0x1425d81d0, 0x141834d18> as *const ()), // sub_1400312a0
    Replacement::draft(0x140031300, "unknown_140031300", tpl_40::<0x143a11c38, 0x143a11c3b, 0x1430786c4, 0x142409b50, 0x1425d81e0, 0x141834d18> as *const ()), // sub_140031300
    Replacement::draft(0x140031360, "unknown_140031360", tpl_41::<0x143a11950, 0x143a11953, 0x1430786d0, 0x142409b50, 0x1425d81f0, 0x141834d18> as *const ()), // sub_140031360
    Replacement::draft(0x1400313c0, "unknown_1400313c0", tpl_42::<0x143a11be8, 0x143a11beb, 0x1430786e0, 0x142409b50, 0x1425d8200, 0x141834d18> as *const ()), // sub_1400313c0
    Replacement::draft(0x140031420, "unknown_140031420", tpl_34::<0x143a11c10, 0x143a11c13, 0x1430786f8, 0x142409b50, 0x1425d8210, 0x141834d18> as *const ()), // sub_140031420
    Replacement::draft(0x140031480, "unknown_140031480", tpl_34::<0x143a11888, 0x143a1188b, 0x143078708, 0x142409b50, 0x1425d8220, 0x141834d18> as *const ()), // sub_140031480
    Replacement::draft(0x1400314e0, "unknown_1400314e0", tpl_41::<0x143a11928, 0x143a1192b, 0x143078718, 0x142409b50, 0x1425d8230, 0x141834d18> as *const ()), // sub_1400314e0
    Replacement::draft(0x140031540, "unknown_140031540", tpl_43::<0x143a119c8, 0x143a119cb, 0x143078728, 0x142409b50, 0x1425d8240, 0x141834d18> as *const ()), // sub_140031540
    Replacement::draft(0x1400315a0, "unknown_1400315a0", tpl_44::<0x143a11778, 0x143a1177b, 0x143078748, 0x142409b50, 0x1425d8250, 0x141834d18> as *const ()), // sub_1400315a0
    Replacement::draft(0x140031600, "unknown_140031600", tpl_39::<0x143a11978, 0x143a1197b, 0x143078760, 0x142409b50, 0x1425d8260, 0x141834d18> as *const ()), // sub_140031600
    Replacement::draft(0x140031660, "unknown_140031660", tpl_45::<0x143a118d8, 0x143a118db, 0x143078768, 0x142409b50, 0x1425d8270, 0x141834d18> as *const ()), // sub_140031660
    Replacement::draft(0x1400316c0, "unknown_1400316c0", tpl_41::<0x143a11af8, 0x143a11afb, 0x143078778, 0x142409b50, 0x1425d8280, 0x141834d18> as *const ()), // sub_1400316c0
    Replacement::draft(0x140031720, "unknown_140031720", tpl_37::<0x143a117a0, 0x143a117a3, 0x143078788, 0x142409b50, 0x1425d8290, 0x141834d18> as *const ()), // sub_140031720
    Replacement::draft(0x140031780, "unknown_140031780", tpl_46::<0x143a11a80, 0x143a11a83, 0x143078798, 0x142409b50, 0x1425d82a0, 0x141834d18> as *const ()), // sub_140031780
    Replacement::draft(0x1400317e0, "unknown_1400317e0", tpl_47::<0x143a11838, 0x143a1183b, 0x1430787b0, 0x142409b50, 0x1425d82b0, 0x141834d18> as *const ()), // sub_1400317e0
    Replacement::draft(0x140031840, "unknown_140031840", tpl_38::<0x143a11aa8, 0x143a11aab, 0x1430787d0, 0x142409b50, 0x1425d82c0, 0x141834d18> as *const ()), // sub_140031840
    Replacement::draft(0x1400318a0, "unknown_1400318a0", tpl_48::<0x143a11bc0, 0x143a11bc3, 0x1430787e0, 0x142409b50, 0x1425d82d0, 0x141834d18> as *const ()), // sub_1400318a0
    Replacement::draft(0x140031900, "unknown_140031900", tpl_49::<0x143a11a30, 0x143a11a33, 0x1430787f8, 0x142409b50, 0x1425d82e0, 0x141834d18> as *const ()), // sub_140031900
    Replacement::draft(0x140031960, "unknown_140031960", tpl_44::<0x143a11b48, 0x143a11b4b, 0x143078808, 0x142409b50, 0x1425d82f0, 0x141834d18> as *const ()), // sub_140031960
    Replacement::draft(0x1400319c0, "unknown_1400319c0", tpl_42::<0x143a11750, 0x143a11753, 0x143078820, 0x142409b50, 0x1425d8300, 0x141834d18> as *const ()), // sub_1400319c0
    Replacement::draft(0x140031a20, "unknown_140031a20", tpl_50::<0x143a11ad0, 0x143a11ad3, 0x143078838, 0x142409b50, 0x1425d8310, 0x141834d18> as *const ()), // sub_140031a20
    Replacement::draft(0x140031a80, "unknown_140031a80", tpl_23::<0x143a117c8, 0x143a117cb, 0x1426cdcb0, 0x142409b50, 0x1425d8320, 0x141834d18> as *const ()), // sub_140031a80
    Replacement::draft(0x140031ae0, "unknown_140031ae0", tpl_23::<0x143a11810, 0x143a11813, 0x1426cdcb0, 0x142409b50, 0x1425d8330, 0x141834d18> as *const ()), // sub_140031ae0
    Replacement::draft(0x140031cd0, "unknown_140031cd0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844760, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d8370, 0x141834d18> as *const ()), // sub_140031cd0
    Replacement::draft(0x140032490, "unknown_140032490", tpl_23::<0x143a11e18, 0x143a11e1b, 0x1426cdcb0, 0x142409b50, 0x1425d83e0, 0x141834d18> as *const ()), // sub_140032490
    Replacement::draft(0x1400324f0, "unknown_1400324f0", tpl_23::<0x143a11e60, 0x143a11e63, 0x1426cdcb0, 0x142409b50, 0x1425d83f0, 0x141834d18> as *const ()), // sub_1400324f0
    Replacement::draft(0x1400326e0, "unknown_1400326e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438447f8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d8430, 0x141834d18> as *const ()), // sub_1400326e0
    Replacement::draft(0x140032ae0, "unknown_140032ae0", tpl_34::<0x143a11fc8, 0x143a11fcb, 0x143078668, 0x142409b50, 0x1425d84a0, 0x141834d18> as *const ()), // sub_140032ae0
    Replacement::draft(0x140032b40, "unknown_140032b40", tpl_23::<0x143a125d8, 0x143a125db, 0x1426cdcb0, 0x142409b50, 0x1425d84b0, 0x141834d18> as *const ()), // sub_140032b40
    Replacement::draft(0x140032ba0, "unknown_140032ba0", tpl_35::<0x143a12308, 0x143a1230b, 0x143078678, 0x142409b50, 0x1425d84c0, 0x141834d18> as *const ()), // sub_140032ba0
    Replacement::draft(0x140032c00, "unknown_140032c00", tpl_36::<0x143a12560, 0x143a12563, 0x143078680, 0x142409b50, 0x1425d84d0, 0x141834d18> as *const ()), // sub_140032c00
    Replacement::draft(0x140032c60, "unknown_140032c60", tpl_37::<0x143a121a8, 0x143a121ab, 0x143078690, 0x142409b50, 0x1425d84e0, 0x141834d18> as *const ()), // sub_140032c60
    Replacement::draft(0x140032cc0, "unknown_140032cc0", tpl_38::<0x143a12068, 0x143a1206b, 0x1430786a0, 0x142409b50, 0x1425d84f0, 0x141834d18> as *const ()), // sub_140032cc0
    Replacement::draft(0x140032d20, "unknown_140032d20", tpl_39::<0x143a12600, 0x143a12603, 0x1430786b0, 0x142409b50, 0x1425d8500, 0x141834d18> as *const ()), // sub_140032d20
    Replacement::draft(0x140032d80, "unknown_140032d80", tpl_37::<0x143a11f78, 0x143a11f7b, 0x1430786b8, 0x142409b50, 0x1425d8510, 0x141834d18> as *const ()), // sub_140032d80
    Replacement::draft(0x140032de0, "unknown_140032de0", tpl_40::<0x143a12718, 0x143a1271b, 0x1430786c4, 0x142409b50, 0x1425d8520, 0x141834d18> as *const ()), // sub_140032de0
    Replacement::draft(0x140032e40, "unknown_140032e40", tpl_41::<0x143a12130, 0x143a12133, 0x1430786d0, 0x142409b50, 0x1425d8530, 0x141834d18> as *const ()), // sub_140032e40
    Replacement::draft(0x140032ea0, "unknown_140032ea0", tpl_42::<0x143a12650, 0x143a12653, 0x1430786e0, 0x142409b50, 0x1425d8540, 0x141834d18> as *const ()), // sub_140032ea0
    Replacement::draft(0x140032f00, "unknown_140032f00", tpl_34::<0x143a126a0, 0x143a126a3, 0x1430786f8, 0x142409b50, 0x1425d8550, 0x141834d18> as *const ()), // sub_140032f00
    Replacement::draft(0x140032f60, "unknown_140032f60", tpl_34::<0x143a11fa0, 0x143a11fa3, 0x143078708, 0x142409b50, 0x1425d8560, 0x141834d18> as *const ()), // sub_140032f60
    Replacement::draft(0x140032fc0, "unknown_140032fc0", tpl_41::<0x143a12090, 0x143a12093, 0x143078718, 0x142409b50, 0x1425d8570, 0x141834d18> as *const ()), // sub_140032fc0
    Replacement::draft(0x140033020, "unknown_140033020", tpl_43::<0x143a121d0, 0x143a121d3, 0x143078728, 0x142409b50, 0x1425d8580, 0x141834d18> as *const ()), // sub_140033020
    Replacement::draft(0x140033080, "unknown_140033080", tpl_44::<0x143a11c88, 0x143a11c8b, 0x143078748, 0x142409b50, 0x1425d8590, 0x141834d18> as *const ()), // sub_140033080
    Replacement::draft(0x1400330e0, "unknown_1400330e0", tpl_39::<0x143a12158, 0x143a1215b, 0x143078760, 0x142409b50, 0x1425d85a0, 0x141834d18> as *const ()), // sub_1400330e0
    Replacement::draft(0x140033140, "unknown_140033140", tpl_45::<0x143a12040, 0x143a12043, 0x143078768, 0x142409b50, 0x1425d85b0, 0x141834d18> as *const ()), // sub_140033140
    Replacement::draft(0x1400331a0, "unknown_1400331a0", tpl_41::<0x143a12510, 0x143a12513, 0x143078778, 0x142409b50, 0x1425d85c0, 0x141834d18> as *const ()), // sub_1400331a0
    Replacement::draft(0x140033200, "unknown_140033200", tpl_37::<0x143a11cb0, 0x143a11cb3, 0x143078788, 0x142409b50, 0x1425d85d0, 0x141834d18> as *const ()), // sub_140033200
    Replacement::draft(0x140033260, "unknown_140033260", tpl_46::<0x143a123a8, 0x143a123ab, 0x143078798, 0x142409b50, 0x1425d85e0, 0x141834d18> as *const ()), // sub_140033260
    Replacement::draft(0x1400332c0, "unknown_1400332c0", tpl_47::<0x143a11ed8, 0x143a11edb, 0x1430787b0, 0x142409b50, 0x1425d85f0, 0x141834d18> as *const ()), // sub_1400332c0
    Replacement::draft(0x140033320, "unknown_140033320", tpl_38::<0x143a12448, 0x143a1244b, 0x1430787d0, 0x142409b50, 0x1425d8600, 0x141834d18> as *const ()), // sub_140033320
    Replacement::draft(0x140033380, "unknown_140033380", tpl_48::<0x143a12628, 0x143a1262b, 0x1430787e0, 0x142409b50, 0x1425d8610, 0x141834d18> as *const ()), // sub_140033380
    Replacement::draft(0x1400333e0, "unknown_1400333e0", tpl_49::<0x143a122e0, 0x143a122e3, 0x1430787f8, 0x142409b50, 0x1425d8620, 0x141834d18> as *const ()), // sub_1400333e0
    Replacement::draft(0x140033440, "unknown_140033440", tpl_44::<0x143a125b0, 0x143a125b3, 0x143078808, 0x142409b50, 0x1425d8630, 0x141834d18> as *const ()), // sub_140033440
    Replacement::draft(0x1400334a0, "unknown_1400334a0", tpl_42::<0x143a11c60, 0x143a11c63, 0x143078820, 0x142409b50, 0x1425d8640, 0x141834d18> as *const ()), // sub_1400334a0
    Replacement::draft(0x140033500, "unknown_140033500", tpl_50::<0x143a124c0, 0x143a124c3, 0x143078838, 0x142409b50, 0x1425d8650, 0x141834d18> as *const ()), // sub_140033500
    Replacement::draft(0x140033560, "unknown_140033560", tpl_52::<0x143a11df0, 0x143a11df3, 0x143a12718, 0x143a1271c, 0x143a1271b, 0x143a12720, 0x142409b50, 0x1425d8660, 0x141834d18> as *const ()), // sub_140033560
    Replacement::draft(0x140033620, "unknown_140033620", tpl_38::<0x143a12330, 0x143a12333, 0x143078950, 0x142409b50, 0x1425d8670, 0x141834d18> as *const ()), // sub_140033620
    Replacement::draft(0x140033680, "unknown_140033680", tpl_53::<0x143a11eb0, 0x143a11eb3, 0x143078960, 0x142409b50, 0x1425d8680, 0x141834d18> as *const ()), // sub_140033680
    Replacement::draft(0x1400336e0, "unknown_1400336e0", tpl_45::<0x143a11d78, 0x143a11d7b, 0x143078980, 0x142409b50, 0x1425d8690, 0x141834d18> as *const ()), // sub_1400336e0
    Replacement::draft(0x140033740, "unknown_140033740", tpl_35::<0x143a11dc8, 0x143a11dcb, 0x14307898c, 0x142409b50, 0x1425d86a0, 0x141834d18> as *const ()), // sub_140033740
    Replacement::draft(0x1400337a0, "unknown_1400337a0", tpl_42::<0x143a12678, 0x143a1267b, 0x1430786e0, 0x142409b50, 0x1425d86b0, 0x141834d18> as *const ()), // sub_1400337a0
    Replacement::draft(0x140033800, "unknown_140033800", tpl_38::<0x143a12588, 0x143a1258b, 0x143078998, 0x142409b50, 0x1425d86c0, 0x141834d18> as *const ()), // sub_140033800
    Replacement::draft(0x140033860, "unknown_140033860", tpl_38::<0x143a120e0, 0x143a120e3, 0x1430789a8, 0x142409b50, 0x1425d86d0, 0x141834d18> as *const ()), // sub_140033860
    Replacement::draft(0x1400338c0, "unknown_1400338c0", tpl_41::<0x143a12240, 0x143a12243, 0x1430789b8, 0x142409b50, 0x1425d86e0, 0x141834d18> as *const ()), // sub_1400338c0
    Replacement::draft(0x140033920, "unknown_140033920", tpl_34::<0x143a126c8, 0x143a126cb, 0x1430789c8, 0x142409b50, 0x1425d86f0, 0x141834d18> as *const ()), // sub_140033920
    Replacement::draft(0x140033980, "unknown_140033980", tpl_39::<0x143a11da0, 0x143a11da3, 0x1430789d8, 0x142409b50, 0x1425d8700, 0x141834d18> as *const ()), // sub_140033980
    Replacement::draft(0x1400339e0, "unknown_1400339e0", tpl_41::<0x143a120b8, 0x143a120bb, 0x1430789e0, 0x142409b50, 0x1425d8710, 0x141834d18> as *const ()), // sub_1400339e0
    Replacement::draft(0x140033a40, "unknown_140033a40", tpl_34::<0x143a12358, 0x143a1235b, 0x1430789f0, 0x142409b50, 0x1425d8720, 0x141834d18> as *const ()), // sub_140033a40
    Replacement::draft(0x140033aa0, "unknown_140033aa0", tpl_48::<0x143a11ff0, 0x143a11ff3, 0x143078a00, 0x142409b50, 0x1425d8730, 0x141834d18> as *const ()), // sub_140033aa0
    Replacement::draft(0x140033b00, "unknown_140033b00", tpl_46::<0x143a12470, 0x143a12473, 0x143078a18, 0x142409b50, 0x1425d8740, 0x141834d18> as *const ()), // sub_140033b00
    Replacement::draft(0x140033b60, "unknown_140033b60", tpl_47::<0x143a123d0, 0x143a123d3, 0x143078a30, 0x142409b50, 0x1425d8750, 0x141834d18> as *const ()), // sub_140033b60
    Replacement::draft(0x140033bc0, "unknown_140033bc0", tpl_37::<0x143a11f28, 0x143a11f2b, 0x143078a50, 0x142409b50, 0x1425d8760, 0x141834d18> as *const ()), // sub_140033bc0
    Replacement::draft(0x140033c20, "unknown_140033c20", tpl_45::<0x143a12180, 0x143a12183, 0x143078a60, 0x142409b50, 0x1425d8770, 0x141834d18> as *const ()), // sub_140033c20
    Replacement::draft(0x140033c80, "unknown_140033c80", tpl_54::<0x143a12290, 0x143a12293, 0x143078a70, 0x142409b50, 0x1425d8780, 0x141834d18> as *const ()), // sub_140033c80
    Replacement::draft(0x140033ce0, "unknown_140033ce0", tpl_40::<0x143a11f00, 0x143a11f03, 0x143078a88, 0x142409b50, 0x1425d8790, 0x141834d18> as *const ()), // sub_140033ce0
    Replacement::draft(0x140033d40, "unknown_140033d40", tpl_50::<0x143a11f50, 0x143a11f53, 0x143078a90, 0x142409b50, 0x1425d87a0, 0x141834d18> as *const ()), // sub_140033d40
    Replacement::draft(0x140033da0, "unknown_140033da0", tpl_41::<0x143a12108, 0x143a1210b, 0x143078aa8, 0x142409b50, 0x1425d87b0, 0x141834d18> as *const ()), // sub_140033da0
    Replacement::draft(0x140033e00, "unknown_140033e00", tpl_55::<0x143a123f8, 0x143a123fb, 0x143078ab8, 0x142409b50, 0x1425d87c0, 0x141834d18> as *const ()), // sub_140033e00
    Replacement::draft(0x140033e60, "unknown_140033e60", tpl_41::<0x143a12498, 0x143a1249b, 0x143078ac8, 0x142409b50, 0x1425d87d0, 0x141834d18> as *const ()), // sub_140033e60
    Replacement::draft(0x140033ec0, "unknown_140033ec0", tpl_49::<0x143a12538, 0x143a1253b, 0x143078ad8, 0x142409b50, 0x1425d87e0, 0x141834d18> as *const ()), // sub_140033ec0
    Replacement::draft(0x140033f20, "unknown_140033f20", tpl_40::<0x143a11d50, 0x143a11d53, 0x143078ae4, 0x142409b50, 0x1425d87f0, 0x141834d18> as *const ()), // sub_140033f20
    Replacement::draft(0x140033f80, "unknown_140033f80", tpl_44::<0x143a11e88, 0x143a11e8b, 0x143078af0, 0x142409b50, 0x1425d8800, 0x141834d18> as *const ()), // sub_140033f80
    Replacement::draft(0x140033fe0, "unknown_140033fe0", tpl_46::<0x143a11d28, 0x143a11d2b, 0x143078b08, 0x142409b50, 0x1425d8810, 0x141834d18> as *const ()), // sub_140033fe0
    Replacement::draft(0x140034040, "unknown_140034040", tpl_55::<0x143a12420, 0x143a12423, 0x143078b20, 0x142409b50, 0x1425d8820, 0x141834d18> as *const ()), // sub_140034040
    Replacement::draft(0x1400340a0, "unknown_1400340a0", tpl_46::<0x143a122b8, 0x143a122bb, 0x143078b30, 0x142409b50, 0x1425d8830, 0x141834d18> as *const ()), // sub_1400340a0
    Replacement::draft(0x140034100, "unknown_140034100", tpl_50::<0x143a12018, 0x143a1201b, 0x143078b48, 0x142409b50, 0x1425d8840, 0x141834d18> as *const ()), // sub_140034100
    Replacement::draft(0x140034160, "unknown_140034160", tpl_42::<0x143a11cd8, 0x143a11cdb, 0x143078b60, 0x142409b50, 0x1425d8850, 0x141834d18> as *const ()), // sub_140034160
    Replacement::draft(0x1400341c0, "unknown_1400341c0", tpl_38::<0x143a12380, 0x143a12383, 0x143078b78, 0x142409b50, 0x1425d8860, 0x141834d18> as *const ()), // sub_1400341c0
    Replacement::draft(0x140034220, "unknown_140034220", tpl_34::<0x143a126f0, 0x143a126f3, 0x143078b88, 0x142409b50, 0x1425d8870, 0x141834d18> as *const ()), // sub_140034220
    Replacement::draft(0x140034280, "unknown_140034280", tpl_41::<0x143a124e8, 0x143a124eb, 0x143078b98, 0x142409b50, 0x1425d8880, 0x141834d18> as *const ()), // sub_140034280
    Replacement::draft(0x1400342e0, "unknown_1400342e0", tpl_36::<0x143a12268, 0x143a1226b, 0x143078ba8, 0x142409b50, 0x1425d8890, 0x141834d18> as *const ()), // sub_1400342e0
    Replacement::draft(0x140034340, "unknown_140034340", tpl_34::<0x143a11d00, 0x143a11d03, 0x143078bb8, 0x142409b50, 0x1425d88a0, 0x141834d18> as *const ()), // sub_140034340
    Replacement::draft(0x140034640, "unknown_140034640", fn_140034640 as *const ()), // sub_140034640
    Replacement::draft(0x1400346d0, "unknown_1400346d0", fn_1400346d0 as *const ()), // sub_1400346d0
    Replacement::draft(0x140034b20, "unknown_140034b20", fn_140034b20 as *const ()), // sub_140034b20
    Replacement::draft(0x140034bb0, "unknown_140034bb0", fn_140034bb0 as *const ()), // sub_140034bb0
    Replacement::draft(0x140034f10, "unknown_140034f10", tpl_34::<0x143a12aa8, 0x143a12aab, 0x143078668, 0x142409b50, 0x1425d8a60, 0x141834d18> as *const ()), // sub_140034f10
    Replacement::draft(0x140034f70, "unknown_140034f70", tpl_23::<0x143a130b8, 0x143a130bb, 0x1426cdcb0, 0x142409b50, 0x1425d8a70, 0x141834d18> as *const ()), // sub_140034f70
    Replacement::draft(0x140034fd0, "unknown_140034fd0", tpl_35::<0x143a12de8, 0x143a12deb, 0x143078678, 0x142409b50, 0x1425d8a80, 0x141834d18> as *const ()), // sub_140034fd0
    Replacement::draft(0x140035030, "unknown_140035030", tpl_36::<0x143a13040, 0x143a13043, 0x143078680, 0x142409b50, 0x1425d8a90, 0x141834d18> as *const ()), // sub_140035030
    Replacement::draft(0x140035090, "unknown_140035090", tpl_37::<0x143a12c88, 0x143a12c8b, 0x143078690, 0x142409b50, 0x1425d8aa0, 0x141834d18> as *const ()), // sub_140035090
    Replacement::draft(0x1400350f0, "unknown_1400350f0", tpl_38::<0x143a12b48, 0x143a12b4b, 0x1430786a0, 0x142409b50, 0x1425d8ab0, 0x141834d18> as *const ()), // sub_1400350f0
    Replacement::draft(0x140035150, "unknown_140035150", tpl_39::<0x143a130e0, 0x143a130e3, 0x1430786b0, 0x142409b50, 0x1425d8ac0, 0x141834d18> as *const ()), // sub_140035150
    Replacement::draft(0x1400351b0, "unknown_1400351b0", tpl_37::<0x143a12a58, 0x143a12a5b, 0x1430786b8, 0x142409b50, 0x1425d8ad0, 0x141834d18> as *const ()), // sub_1400351b0
    Replacement::draft(0x140035210, "unknown_140035210", tpl_40::<0x143a131f8, 0x143a131fb, 0x1430786c4, 0x142409b50, 0x1425d8ae0, 0x141834d18> as *const ()), // sub_140035210
    Replacement::draft(0x140035270, "unknown_140035270", tpl_41::<0x143a12c10, 0x143a12c13, 0x1430786d0, 0x142409b50, 0x1425d8af0, 0x141834d18> as *const ()), // sub_140035270
    Replacement::draft(0x1400352d0, "unknown_1400352d0", tpl_42::<0x143a13130, 0x143a13133, 0x1430786e0, 0x142409b50, 0x1425d8b00, 0x141834d18> as *const ()), // sub_1400352d0
    Replacement::draft(0x140035330, "unknown_140035330", tpl_34::<0x143a13180, 0x143a13183, 0x1430786f8, 0x142409b50, 0x1425d8b10, 0x141834d18> as *const ()), // sub_140035330
    Replacement::draft(0x140035390, "unknown_140035390", tpl_34::<0x143a12a80, 0x143a12a83, 0x143078708, 0x142409b50, 0x1425d8b20, 0x141834d18> as *const ()), // sub_140035390
    Replacement::draft(0x1400353f0, "unknown_1400353f0", tpl_41::<0x143a12b70, 0x143a12b73, 0x143078718, 0x142409b50, 0x1425d8b30, 0x141834d18> as *const ()), // sub_1400353f0
    Replacement::draft(0x140035450, "unknown_140035450", tpl_43::<0x143a12cb0, 0x143a12cb3, 0x143078728, 0x142409b50, 0x1425d8b40, 0x141834d18> as *const ()), // sub_140035450
    Replacement::draft(0x1400354b0, "unknown_1400354b0", tpl_44::<0x143a12768, 0x143a1276b, 0x143078748, 0x142409b50, 0x1425d8b50, 0x141834d18> as *const ()), // sub_1400354b0
    Replacement::draft(0x140035510, "unknown_140035510", tpl_39::<0x143a12c38, 0x143a12c3b, 0x143078760, 0x142409b50, 0x1425d8b60, 0x141834d18> as *const ()), // sub_140035510
    Replacement::draft(0x140035570, "unknown_140035570", tpl_45::<0x143a12b20, 0x143a12b23, 0x143078768, 0x142409b50, 0x1425d8b70, 0x141834d18> as *const ()), // sub_140035570
    Replacement::draft(0x1400355d0, "unknown_1400355d0", tpl_41::<0x143a12ff0, 0x143a12ff3, 0x143078778, 0x142409b50, 0x1425d8b80, 0x141834d18> as *const ()), // sub_1400355d0
    Replacement::draft(0x140035630, "unknown_140035630", tpl_37::<0x143a12790, 0x143a12793, 0x143078788, 0x142409b50, 0x1425d8b90, 0x141834d18> as *const ()), // sub_140035630
    Replacement::draft(0x140035690, "unknown_140035690", tpl_46::<0x143a12e88, 0x143a12e8b, 0x143078798, 0x142409b50, 0x1425d8ba0, 0x141834d18> as *const ()), // sub_140035690
    Replacement::draft(0x1400356f0, "unknown_1400356f0", tpl_47::<0x143a129b8, 0x143a129bb, 0x1430787b0, 0x142409b50, 0x1425d8bb0, 0x141834d18> as *const ()), // sub_1400356f0
    Replacement::draft(0x140035750, "unknown_140035750", tpl_38::<0x143a12f28, 0x143a12f2b, 0x1430787d0, 0x142409b50, 0x1425d8bc0, 0x141834d18> as *const ()), // sub_140035750
    Replacement::draft(0x1400357b0, "unknown_1400357b0", tpl_48::<0x143a13108, 0x143a1310b, 0x1430787e0, 0x142409b50, 0x1425d8bd0, 0x141834d18> as *const ()), // sub_1400357b0
    Replacement::draft(0x140035810, "unknown_140035810", tpl_49::<0x143a12dc0, 0x143a12dc3, 0x1430787f8, 0x142409b50, 0x1425d8be0, 0x141834d18> as *const ()), // sub_140035810
    Replacement::draft(0x140035870, "unknown_140035870", tpl_44::<0x143a13090, 0x143a13093, 0x143078808, 0x142409b50, 0x1425d8bf0, 0x141834d18> as *const ()), // sub_140035870
    Replacement::draft(0x1400358d0, "unknown_1400358d0", tpl_42::<0x143a12740, 0x143a12743, 0x143078820, 0x142409b50, 0x1425d8c00, 0x141834d18> as *const ()), // sub_1400358d0
    Replacement::draft(0x140035930, "unknown_140035930", tpl_50::<0x143a12fa0, 0x143a12fa3, 0x143078838, 0x142409b50, 0x1425d8c10, 0x141834d18> as *const ()), // sub_140035930
    Replacement::draft(0x140035990, "unknown_140035990", tpl_23::<0x143a128f8, 0x143a128fb, 0x1426cdcb0, 0x142409b50, 0x1425d8c20, 0x141834d18> as *const ()), // sub_140035990
    Replacement::draft(0x1400359f0, "unknown_1400359f0", tpl_23::<0x143a12940, 0x143a12943, 0x1426cdcb0, 0x142409b50, 0x1425d8c30, 0x141834d18> as *const ()), // sub_1400359f0
    Replacement::draft(0x140035be0, "unknown_140035be0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844900, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d8c70, 0x141834d18> as *const ()), // sub_140035be0
    Replacement::draft(0x140035fe0, "unknown_140035fe0", tpl_52::<0x143a128d0, 0x143a128d3, 0x143a131f8, 0x143a131fc, 0x143a131fb, 0x143a13200, 0x142409b50, 0x1425d8ce0, 0x141834d18> as *const ()), // sub_140035fe0
    Replacement::draft(0x1400360a0, "unknown_1400360a0", tpl_38::<0x143a12e10, 0x143a12e13, 0x143078950, 0x142409b50, 0x1425d8cf0, 0x141834d18> as *const ()), // sub_1400360a0
    Replacement::draft(0x140036100, "unknown_140036100", tpl_53::<0x143a12990, 0x143a12993, 0x143078960, 0x142409b50, 0x1425d8d00, 0x141834d18> as *const ()), // sub_140036100
    Replacement::draft(0x140036160, "unknown_140036160", tpl_45::<0x143a12858, 0x143a1285b, 0x143078980, 0x142409b50, 0x1425d8d10, 0x141834d18> as *const ()), // sub_140036160
    Replacement::draft(0x1400361c0, "unknown_1400361c0", tpl_35::<0x143a128a8, 0x143a128ab, 0x14307898c, 0x142409b50, 0x1425d8d20, 0x141834d18> as *const ()), // sub_1400361c0
    Replacement::draft(0x140036220, "unknown_140036220", tpl_42::<0x143a13158, 0x143a1315b, 0x1430786e0, 0x142409b50, 0x1425d8d30, 0x141834d18> as *const ()), // sub_140036220
    Replacement::draft(0x140036280, "unknown_140036280", tpl_38::<0x143a13068, 0x143a1306b, 0x143078998, 0x142409b50, 0x1425d8d40, 0x141834d18> as *const ()), // sub_140036280
    Replacement::draft(0x1400362e0, "unknown_1400362e0", tpl_38::<0x143a12bc0, 0x143a12bc3, 0x1430789a8, 0x142409b50, 0x1425d8d50, 0x141834d18> as *const ()), // sub_1400362e0
    Replacement::draft(0x140036340, "unknown_140036340", tpl_41::<0x143a12d20, 0x143a12d23, 0x1430789b8, 0x142409b50, 0x1425d8d60, 0x141834d18> as *const ()), // sub_140036340
    Replacement::draft(0x1400363a0, "unknown_1400363a0", tpl_34::<0x143a131a8, 0x143a131ab, 0x1430789c8, 0x142409b50, 0x1425d8d70, 0x141834d18> as *const ()), // sub_1400363a0
    Replacement::draft(0x140036400, "unknown_140036400", tpl_39::<0x143a12880, 0x143a12883, 0x1430789d8, 0x142409b50, 0x1425d8d80, 0x141834d18> as *const ()), // sub_140036400
    Replacement::draft(0x140036460, "unknown_140036460", tpl_41::<0x143a12b98, 0x143a12b9b, 0x1430789e0, 0x142409b50, 0x1425d8d90, 0x141834d18> as *const ()), // sub_140036460
    Replacement::draft(0x1400364c0, "unknown_1400364c0", tpl_34::<0x143a12e38, 0x143a12e3b, 0x1430789f0, 0x142409b50, 0x1425d8da0, 0x141834d18> as *const ()), // sub_1400364c0
    Replacement::draft(0x140036520, "unknown_140036520", tpl_48::<0x143a12ad0, 0x143a12ad3, 0x143078a00, 0x142409b50, 0x1425d8db0, 0x141834d18> as *const ()), // sub_140036520
    Replacement::draft(0x140036580, "unknown_140036580", tpl_46::<0x143a12f50, 0x143a12f53, 0x143078a18, 0x142409b50, 0x1425d8dc0, 0x141834d18> as *const ()), // sub_140036580
    Replacement::draft(0x1400365e0, "unknown_1400365e0", tpl_47::<0x143a12eb0, 0x143a12eb3, 0x143078a30, 0x142409b50, 0x1425d8dd0, 0x141834d18> as *const ()), // sub_1400365e0
    Replacement::draft(0x140036640, "unknown_140036640", tpl_37::<0x143a12a08, 0x143a12a0b, 0x143078a50, 0x142409b50, 0x1425d8de0, 0x141834d18> as *const ()), // sub_140036640
    Replacement::draft(0x1400366a0, "unknown_1400366a0", tpl_45::<0x143a12c60, 0x143a12c63, 0x143078a60, 0x142409b50, 0x1425d8df0, 0x141834d18> as *const ()), // sub_1400366a0
    Replacement::draft(0x140036700, "unknown_140036700", tpl_54::<0x143a12d70, 0x143a12d73, 0x143078a70, 0x142409b50, 0x1425d8e00, 0x141834d18> as *const ()), // sub_140036700
    Replacement::draft(0x140036760, "unknown_140036760", tpl_40::<0x143a129e0, 0x143a129e3, 0x143078a88, 0x142409b50, 0x1425d8e10, 0x141834d18> as *const ()), // sub_140036760
    Replacement::draft(0x1400367c0, "unknown_1400367c0", tpl_50::<0x143a12a30, 0x143a12a33, 0x143078a90, 0x142409b50, 0x1425d8e20, 0x141834d18> as *const ()), // sub_1400367c0
    Replacement::draft(0x140036820, "unknown_140036820", tpl_41::<0x143a12be8, 0x143a12beb, 0x143078aa8, 0x142409b50, 0x1425d8e30, 0x141834d18> as *const ()), // sub_140036820
    Replacement::draft(0x140036880, "unknown_140036880", tpl_55::<0x143a12ed8, 0x143a12edb, 0x143078ab8, 0x142409b50, 0x1425d8e40, 0x141834d18> as *const ()), // sub_140036880
    Replacement::draft(0x1400368e0, "unknown_1400368e0", tpl_41::<0x143a12f78, 0x143a12f7b, 0x143078ac8, 0x142409b50, 0x1425d8e50, 0x141834d18> as *const ()), // sub_1400368e0
    Replacement::draft(0x140036940, "unknown_140036940", tpl_49::<0x143a13018, 0x143a1301b, 0x143078ad8, 0x142409b50, 0x1425d8e60, 0x141834d18> as *const ()), // sub_140036940
    Replacement::draft(0x1400369a0, "unknown_1400369a0", tpl_40::<0x143a12830, 0x143a12833, 0x143078ae4, 0x142409b50, 0x1425d8e70, 0x141834d18> as *const ()), // sub_1400369a0
    Replacement::draft(0x140036a00, "unknown_140036a00", tpl_44::<0x143a12968, 0x143a1296b, 0x143078af0, 0x142409b50, 0x1425d8e80, 0x141834d18> as *const ()), // sub_140036a00
    Replacement::draft(0x140036a60, "unknown_140036a60", tpl_46::<0x143a12808, 0x143a1280b, 0x143078b08, 0x142409b50, 0x1425d8e90, 0x141834d18> as *const ()), // sub_140036a60
    Replacement::draft(0x140036ac0, "unknown_140036ac0", tpl_55::<0x143a12f00, 0x143a12f03, 0x143078b20, 0x142409b50, 0x1425d8ea0, 0x141834d18> as *const ()), // sub_140036ac0
    Replacement::draft(0x140036b20, "unknown_140036b20", tpl_46::<0x143a12d98, 0x143a12d9b, 0x143078b30, 0x142409b50, 0x1425d8eb0, 0x141834d18> as *const ()), // sub_140036b20
    Replacement::draft(0x140036b80, "unknown_140036b80", tpl_50::<0x143a12af8, 0x143a12afb, 0x143078b48, 0x142409b50, 0x1425d8ec0, 0x141834d18> as *const ()), // sub_140036b80
    Replacement::draft(0x140036be0, "unknown_140036be0", tpl_42::<0x143a127b8, 0x143a127bb, 0x143078b60, 0x142409b50, 0x1425d8ed0, 0x141834d18> as *const ()), // sub_140036be0
    Replacement::draft(0x140036c40, "unknown_140036c40", tpl_38::<0x143a12e60, 0x143a12e63, 0x143078b78, 0x142409b50, 0x1425d8ee0, 0x141834d18> as *const ()), // sub_140036c40
    Replacement::draft(0x140036ca0, "unknown_140036ca0", tpl_34::<0x143a131d0, 0x143a131d3, 0x143078b88, 0x142409b50, 0x1425d8ef0, 0x141834d18> as *const ()), // sub_140036ca0
    Replacement::draft(0x140036d00, "unknown_140036d00", tpl_41::<0x143a12fc8, 0x143a12fcb, 0x143078b98, 0x142409b50, 0x1425d8f00, 0x141834d18> as *const ()), // sub_140036d00
    Replacement::draft(0x140036d60, "unknown_140036d60", tpl_36::<0x143a12d48, 0x143a12d4b, 0x143078ba8, 0x142409b50, 0x1425d8f10, 0x141834d18> as *const ()), // sub_140036d60
    Replacement::draft(0x140036dc0, "unknown_140036dc0", tpl_34::<0x143a127e0, 0x143a127e3, 0x143078bb8, 0x142409b50, 0x1425d8f20, 0x141834d18> as *const ()), // sub_140036dc0
    Replacement::draft(0x140037000, "unknown_140037000", tpl_23::<0x143a13298, 0x143a1329b, 0x1426cdcb0, 0x142409b50, 0x1425d8f30, 0x141834d18> as *const ()), // sub_140037000
    Replacement::draft(0x140037060, "unknown_140037060", tpl_23::<0x143a132e0, 0x143a132e3, 0x1426cdcb0, 0x142409b50, 0x1425d8f40, 0x141834d18> as *const ()), // sub_140037060
    Replacement::draft(0x140037130, "unknown_140037130", tpl_34::<0x143a13380, 0x143a13383, 0x143078668, 0x142409b50, 0x1425d8f60, 0x141834d18> as *const ()), // sub_140037130
    Replacement::draft(0x140037190, "unknown_140037190", tpl_23::<0x143a13600, 0x143a13603, 0x1426cdcb0, 0x142409b50, 0x1425d8f70, 0x141834d18> as *const ()), // sub_140037190
    Replacement::draft(0x1400371f0, "unknown_1400371f0", tpl_35::<0x143a134e8, 0x143a134eb, 0x143078678, 0x142409b50, 0x1425d8f80, 0x141834d18> as *const ()), // sub_1400371f0
    Replacement::draft(0x140037250, "unknown_140037250", tpl_36::<0x143a135b0, 0x143a135b3, 0x143078680, 0x142409b50, 0x1425d8f90, 0x141834d18> as *const ()), // sub_140037250
    Replacement::draft(0x1400372b0, "unknown_1400372b0", tpl_37::<0x143a13470, 0x143a13473, 0x143078690, 0x142409b50, 0x1425d8fa0, 0x141834d18> as *const ()), // sub_1400372b0
    Replacement::draft(0x140037310, "unknown_140037310", tpl_38::<0x143a133d0, 0x143a133d3, 0x1430786a0, 0x142409b50, 0x1425d8fb0, 0x141834d18> as *const ()), // sub_140037310
    Replacement::draft(0x140037370, "unknown_140037370", tpl_39::<0x143a13628, 0x143a1362b, 0x1430786b0, 0x142409b50, 0x1425d8fc0, 0x141834d18> as *const ()), // sub_140037370
    Replacement::draft(0x1400373d0, "unknown_1400373d0", tpl_37::<0x143a13330, 0x143a13333, 0x1430786b8, 0x142409b50, 0x1425d8fd0, 0x141834d18> as *const ()), // sub_1400373d0
    Replacement::draft(0x140037430, "unknown_140037430", tpl_40::<0x143a136c8, 0x143a136cb, 0x1430786c4, 0x142409b50, 0x1425d8fe0, 0x141834d18> as *const ()), // sub_140037430
    Replacement::draft(0x140037490, "unknown_140037490", tpl_41::<0x143a13420, 0x143a13423, 0x1430786d0, 0x142409b50, 0x1425d8ff0, 0x141834d18> as *const ()), // sub_140037490
    Replacement::draft(0x1400374f0, "unknown_1400374f0", tpl_42::<0x143a13678, 0x143a1367b, 0x1430786e0, 0x142409b50, 0x1425d9000, 0x141834d18> as *const ()), // sub_1400374f0
    Replacement::draft(0x140037550, "unknown_140037550", tpl_34::<0x143a136a0, 0x143a136a3, 0x1430786f8, 0x142409b50, 0x1425d9010, 0x141834d18> as *const ()), // sub_140037550
    Replacement::draft(0x1400375b0, "unknown_1400375b0", tpl_34::<0x143a13358, 0x143a1335b, 0x143078708, 0x142409b50, 0x1425d9020, 0x141834d18> as *const ()), // sub_1400375b0
    Replacement::draft(0x140037610, "unknown_140037610", tpl_41::<0x143a133f8, 0x143a133fb, 0x143078718, 0x142409b50, 0x1425d9030, 0x141834d18> as *const ()), // sub_140037610
    Replacement::draft(0x140037670, "unknown_140037670", tpl_43::<0x143a13498, 0x143a1349b, 0x143078728, 0x142409b50, 0x1425d9040, 0x141834d18> as *const ()), // sub_140037670
    Replacement::draft(0x1400376d0, "unknown_1400376d0", tpl_44::<0x143a13248, 0x143a1324b, 0x143078748, 0x142409b50, 0x1425d9050, 0x141834d18> as *const ()), // sub_1400376d0
    Replacement::draft(0x140037730, "unknown_140037730", tpl_39::<0x143a13448, 0x143a1344b, 0x143078760, 0x142409b50, 0x1425d9060, 0x141834d18> as *const ()), // sub_140037730
    Replacement::draft(0x140037790, "unknown_140037790", tpl_45::<0x143a133a8, 0x143a133ab, 0x143078768, 0x142409b50, 0x1425d9070, 0x141834d18> as *const ()), // sub_140037790
    Replacement::draft(0x1400377f0, "unknown_1400377f0", tpl_41::<0x143a13588, 0x143a1358b, 0x143078778, 0x142409b50, 0x1425d9080, 0x141834d18> as *const ()), // sub_1400377f0
    Replacement::draft(0x140037850, "unknown_140037850", tpl_37::<0x143a13270, 0x143a13273, 0x143078788, 0x142409b50, 0x1425d9090, 0x141834d18> as *const ()), // sub_140037850
    Replacement::draft(0x1400378b0, "unknown_1400378b0", tpl_46::<0x143a13510, 0x143a13513, 0x143078798, 0x142409b50, 0x1425d90a0, 0x141834d18> as *const ()), // sub_1400378b0
    Replacement::draft(0x140037910, "unknown_140037910", tpl_47::<0x143a13308, 0x143a1330b, 0x1430787b0, 0x142409b50, 0x1425d90b0, 0x141834d18> as *const ()), // sub_140037910
    Replacement::draft(0x140037970, "unknown_140037970", tpl_38::<0x143a13538, 0x143a1353b, 0x1430787d0, 0x142409b50, 0x1425d90c0, 0x141834d18> as *const ()), // sub_140037970
    Replacement::draft(0x1400379d0, "unknown_1400379d0", tpl_48::<0x143a13650, 0x143a13653, 0x1430787e0, 0x142409b50, 0x1425d90d0, 0x141834d18> as *const ()), // sub_1400379d0
    Replacement::draft(0x140037a30, "unknown_140037a30", tpl_49::<0x143a134c0, 0x143a134c3, 0x1430787f8, 0x142409b50, 0x1425d90e0, 0x141834d18> as *const ()), // sub_140037a30
    Replacement::draft(0x140037a90, "unknown_140037a90", tpl_44::<0x143a135d8, 0x143a135db, 0x143078808, 0x142409b50, 0x1425d90f0, 0x141834d18> as *const ()), // sub_140037a90
    Replacement::draft(0x140037af0, "unknown_140037af0", tpl_42::<0x143a13220, 0x143a13223, 0x143078820, 0x142409b50, 0x1425d9100, 0x141834d18> as *const ()), // sub_140037af0
    Replacement::draft(0x140037b50, "unknown_140037b50", tpl_50::<0x143a13560, 0x143a13563, 0x143078838, 0x142409b50, 0x1425d9110, 0x141834d18> as *const ()), // sub_140037b50
    Replacement::draft(0x140037cd0, "unknown_140037cd0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438449f0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d9140, 0x141834d18> as *const ()), // sub_140037cd0
    Replacement::draft(0x140038330, "unknown_140038330", tpl_23::<0x143a13768, 0x143a1376b, 0x1426cdcb0, 0x142409b50, 0x1425d9180, 0x141834d18> as *const ()), // sub_140038330
    Replacement::draft(0x140038390, "unknown_140038390", tpl_23::<0x143a137b0, 0x143a137b3, 0x1426cdcb0, 0x142409b50, 0x1425d9190, 0x141834d18> as *const ()), // sub_140038390
    Replacement::draft(0x140038580, "unknown_140038580", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844a80, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d91d0, 0x141834d18> as *const ()), // sub_140038580
    Replacement::draft(0x140038910, "unknown_140038910", tpl_34::<0x143a13850, 0x143a13853, 0x143078668, 0x142409b50, 0x1425d9210, 0x141834d18> as *const ()), // sub_140038910
    Replacement::draft(0x140038970, "unknown_140038970", tpl_23::<0x143a13ad0, 0x143a13ad3, 0x1426cdcb0, 0x142409b50, 0x1425d9220, 0x141834d18> as *const ()), // sub_140038970
    Replacement::draft(0x1400389d0, "unknown_1400389d0", tpl_35::<0x143a139b8, 0x143a139bb, 0x143078678, 0x142409b50, 0x1425d9230, 0x141834d18> as *const ()), // sub_1400389d0
    Replacement::draft(0x140038a30, "unknown_140038a30", tpl_36::<0x143a13a80, 0x143a13a83, 0x143078680, 0x142409b50, 0x1425d9240, 0x141834d18> as *const ()), // sub_140038a30
    Replacement::draft(0x140038a90, "unknown_140038a90", tpl_37::<0x143a13940, 0x143a13943, 0x143078690, 0x142409b50, 0x1425d9250, 0x141834d18> as *const ()), // sub_140038a90
    Replacement::draft(0x140038af0, "unknown_140038af0", tpl_38::<0x143a138a0, 0x143a138a3, 0x1430786a0, 0x142409b50, 0x1425d9260, 0x141834d18> as *const ()), // sub_140038af0
    Replacement::draft(0x140038b50, "unknown_140038b50", tpl_39::<0x143a13af8, 0x143a13afb, 0x1430786b0, 0x142409b50, 0x1425d9270, 0x141834d18> as *const ()), // sub_140038b50
    Replacement::draft(0x140038bb0, "unknown_140038bb0", tpl_37::<0x143a13800, 0x143a13803, 0x1430786b8, 0x142409b50, 0x1425d9280, 0x141834d18> as *const ()), // sub_140038bb0
    Replacement::draft(0x140038c10, "unknown_140038c10", tpl_40::<0x143a13b98, 0x143a13b9b, 0x1430786c4, 0x142409b50, 0x1425d9290, 0x141834d18> as *const ()), // sub_140038c10
    Replacement::draft(0x140038c70, "unknown_140038c70", tpl_41::<0x143a138f0, 0x143a138f3, 0x1430786d0, 0x142409b50, 0x1425d92a0, 0x141834d18> as *const ()), // sub_140038c70
    Replacement::draft(0x140038cd0, "unknown_140038cd0", tpl_42::<0x143a13b48, 0x143a13b4b, 0x1430786e0, 0x142409b50, 0x1425d92b0, 0x141834d18> as *const ()), // sub_140038cd0
    Replacement::draft(0x140038d30, "unknown_140038d30", tpl_34::<0x143a13b70, 0x143a13b73, 0x1430786f8, 0x142409b50, 0x1425d92c0, 0x141834d18> as *const ()), // sub_140038d30
    Replacement::draft(0x140038d90, "unknown_140038d90", tpl_34::<0x143a13828, 0x143a1382b, 0x143078708, 0x142409b50, 0x1425d92d0, 0x141834d18> as *const ()), // sub_140038d90
    Replacement::draft(0x140038df0, "unknown_140038df0", tpl_41::<0x143a138c8, 0x143a138cb, 0x143078718, 0x142409b50, 0x1425d92e0, 0x141834d18> as *const ()), // sub_140038df0
    Replacement::draft(0x140038e50, "unknown_140038e50", tpl_43::<0x143a13968, 0x143a1396b, 0x143078728, 0x142409b50, 0x1425d92f0, 0x141834d18> as *const ()), // sub_140038e50
    Replacement::draft(0x140038eb0, "unknown_140038eb0", tpl_44::<0x143a13718, 0x143a1371b, 0x143078748, 0x142409b50, 0x1425d9300, 0x141834d18> as *const ()), // sub_140038eb0
    Replacement::draft(0x140038f10, "unknown_140038f10", tpl_39::<0x143a13918, 0x143a1391b, 0x143078760, 0x142409b50, 0x1425d9310, 0x141834d18> as *const ()), // sub_140038f10
    Replacement::draft(0x140038f70, "unknown_140038f70", tpl_45::<0x143a13878, 0x143a1387b, 0x143078768, 0x142409b50, 0x1425d9320, 0x141834d18> as *const ()), // sub_140038f70
    Replacement::draft(0x140038fd0, "unknown_140038fd0", tpl_41::<0x143a13a58, 0x143a13a5b, 0x143078778, 0x142409b50, 0x1425d9330, 0x141834d18> as *const ()), // sub_140038fd0
    Replacement::draft(0x140039030, "unknown_140039030", tpl_37::<0x143a13740, 0x143a13743, 0x143078788, 0x142409b50, 0x1425d9340, 0x141834d18> as *const ()), // sub_140039030
    Replacement::draft(0x140039090, "unknown_140039090", tpl_46::<0x143a139e0, 0x143a139e3, 0x143078798, 0x142409b50, 0x1425d9350, 0x141834d18> as *const ()), // sub_140039090
    Replacement::draft(0x1400390f0, "unknown_1400390f0", tpl_47::<0x143a137d8, 0x143a137db, 0x1430787b0, 0x142409b50, 0x1425d9360, 0x141834d18> as *const ()), // sub_1400390f0
    Replacement::draft(0x140039150, "unknown_140039150", tpl_38::<0x143a13a08, 0x143a13a0b, 0x1430787d0, 0x142409b50, 0x1425d9370, 0x141834d18> as *const ()), // sub_140039150
    Replacement::draft(0x1400391b0, "unknown_1400391b0", tpl_48::<0x143a13b20, 0x143a13b23, 0x1430787e0, 0x142409b50, 0x1425d9380, 0x141834d18> as *const ()), // sub_1400391b0
    Replacement::draft(0x140039210, "unknown_140039210", tpl_49::<0x143a13990, 0x143a13993, 0x1430787f8, 0x142409b50, 0x1425d9390, 0x141834d18> as *const ()), // sub_140039210
    Replacement::draft(0x140039270, "unknown_140039270", tpl_44::<0x143a13aa8, 0x143a13aab, 0x143078808, 0x142409b50, 0x1425d93a0, 0x141834d18> as *const ()), // sub_140039270
    Replacement::draft(0x1400392d0, "unknown_1400392d0", tpl_42::<0x143a136f0, 0x143a136f3, 0x143078820, 0x142409b50, 0x1425d93b0, 0x141834d18> as *const ()), // sub_1400392d0
    Replacement::draft(0x140039330, "unknown_140039330", tpl_50::<0x143a13a30, 0x143a13a33, 0x143078838, 0x142409b50, 0x1425d93c0, 0x141834d18> as *const ()), // sub_140039330
    Replacement::draft(0x140039390, "unknown_140039390", tpl_23::<0x143a13c38, 0x143a13c3b, 0x1426cdcb0, 0x142409b50, 0x1425d93d0, 0x141834d18> as *const ()), // sub_140039390
    Replacement::draft(0x1400393f0, "unknown_1400393f0", tpl_23::<0x143a13c80, 0x143a13c83, 0x1426cdcb0, 0x142409b50, 0x1425d93e0, 0x141834d18> as *const ()), // sub_1400393f0
    Replacement::draft(0x1400395e0, "unknown_1400395e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844b10, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d9420, 0x141834d18> as *const ()), // sub_1400395e0
    Replacement::draft(0x1400399e0, "unknown_1400399e0", tpl_34::<0x143a13d20, 0x143a13d23, 0x143078668, 0x142409b50, 0x1425d9490, 0x141834d18> as *const ()), // sub_1400399e0
    Replacement::draft(0x140039a40, "unknown_140039a40", tpl_23::<0x143a13fe0, 0x143a13fe3, 0x1426cdcb0, 0x142409b50, 0x1425d94a0, 0x141834d18> as *const ()), // sub_140039a40
    Replacement::draft(0x140039aa0, "unknown_140039aa0", tpl_35::<0x143a13ec8, 0x143a13ecb, 0x143078678, 0x142409b50, 0x1425d94b0, 0x141834d18> as *const ()), // sub_140039aa0
    Replacement::draft(0x140039b00, "unknown_140039b00", tpl_36::<0x143a13f90, 0x143a13f93, 0x143078680, 0x142409b50, 0x1425d94c0, 0x141834d18> as *const ()), // sub_140039b00
    Replacement::draft(0x140039b60, "unknown_140039b60", tpl_37::<0x143a13e10, 0x143a13e13, 0x143078690, 0x142409b50, 0x1425d94d0, 0x141834d18> as *const ()), // sub_140039b60
    Replacement::draft(0x140039bc0, "unknown_140039bc0", tpl_38::<0x143a13d70, 0x143a13d73, 0x1430786a0, 0x142409b50, 0x1425d94e0, 0x141834d18> as *const ()), // sub_140039bc0
    Replacement::draft(0x140039c20, "unknown_140039c20", tpl_39::<0x143a14008, 0x143a1400b, 0x1430786b0, 0x142409b50, 0x1425d94f0, 0x141834d18> as *const ()), // sub_140039c20
    Replacement::draft(0x140039c80, "unknown_140039c80", tpl_37::<0x143a13cd0, 0x143a13cd3, 0x1430786b8, 0x142409b50, 0x1425d9500, 0x141834d18> as *const ()), // sub_140039c80
    Replacement::draft(0x140039ce0, "unknown_140039ce0", tpl_40::<0x143a140a8, 0x143a140ab, 0x1430786c4, 0x142409b50, 0x1425d9510, 0x141834d18> as *const ()), // sub_140039ce0
    Replacement::draft(0x140039d40, "unknown_140039d40", tpl_41::<0x143a13dc0, 0x143a13dc3, 0x1430786d0, 0x142409b50, 0x1425d9520, 0x141834d18> as *const ()), // sub_140039d40
    Replacement::draft(0x140039da0, "unknown_140039da0", tpl_42::<0x143a14058, 0x143a1405b, 0x1430786e0, 0x142409b50, 0x1425d9530, 0x141834d18> as *const ()), // sub_140039da0
    Replacement::draft(0x140039e00, "unknown_140039e00", tpl_34::<0x143a14080, 0x143a14083, 0x1430786f8, 0x142409b50, 0x1425d9540, 0x141834d18> as *const ()), // sub_140039e00
    Replacement::draft(0x140039e60, "unknown_140039e60", tpl_34::<0x143a13cf8, 0x143a13cfb, 0x143078708, 0x142409b50, 0x1425d9550, 0x141834d18> as *const ()), // sub_140039e60
    Replacement::draft(0x140039ec0, "unknown_140039ec0", tpl_41::<0x143a13d98, 0x143a13d9b, 0x143078718, 0x142409b50, 0x1425d9560, 0x141834d18> as *const ()), // sub_140039ec0
    Replacement::draft(0x140039f20, "unknown_140039f20", tpl_43::<0x143a13e38, 0x143a13e3b, 0x143078728, 0x142409b50, 0x1425d9570, 0x141834d18> as *const ()), // sub_140039f20
    Replacement::draft(0x140039f80, "unknown_140039f80", tpl_44::<0x143a13be8, 0x143a13beb, 0x143078748, 0x142409b50, 0x1425d9580, 0x141834d18> as *const ()), // sub_140039f80
    Replacement::draft(0x140039fe0, "unknown_140039fe0", tpl_39::<0x143a13de8, 0x143a13deb, 0x143078760, 0x142409b50, 0x1425d9590, 0x141834d18> as *const ()), // sub_140039fe0
    Replacement::draft(0x14003a040, "unknown_14003a040", tpl_45::<0x143a13d48, 0x143a13d4b, 0x143078768, 0x142409b50, 0x1425d95a0, 0x141834d18> as *const ()), // sub_14003a040
    Replacement::draft(0x14003a0a0, "unknown_14003a0a0", tpl_41::<0x143a13f68, 0x143a13f6b, 0x143078778, 0x142409b50, 0x1425d95b0, 0x141834d18> as *const ()), // sub_14003a0a0
    Replacement::draft(0x14003a100, "unknown_14003a100", tpl_37::<0x143a13c10, 0x143a13c13, 0x143078788, 0x142409b50, 0x1425d95c0, 0x141834d18> as *const ()), // sub_14003a100
    Replacement::draft(0x14003a160, "unknown_14003a160", tpl_46::<0x143a13ef0, 0x143a13ef3, 0x143078798, 0x142409b50, 0x1425d95d0, 0x141834d18> as *const ()), // sub_14003a160
    Replacement::draft(0x14003a1c0, "unknown_14003a1c0", tpl_47::<0x143a13ca8, 0x143a13cab, 0x1430787b0, 0x142409b50, 0x1425d95e0, 0x141834d18> as *const ()), // sub_14003a1c0
    Replacement::draft(0x14003a220, "unknown_14003a220", tpl_38::<0x143a13f18, 0x143a13f1b, 0x1430787d0, 0x142409b50, 0x1425d95f0, 0x141834d18> as *const ()), // sub_14003a220
    Replacement::draft(0x14003a280, "unknown_14003a280", tpl_48::<0x143a14030, 0x143a14033, 0x1430787e0, 0x142409b50, 0x1425d9600, 0x141834d18> as *const ()), // sub_14003a280
    Replacement::draft(0x14003a2e0, "unknown_14003a2e0", tpl_49::<0x143a13ea0, 0x143a13ea3, 0x1430787f8, 0x142409b50, 0x1425d9610, 0x141834d18> as *const ()), // sub_14003a2e0
    Replacement::draft(0x14003a340, "unknown_14003a340", tpl_44::<0x143a13fb8, 0x143a13fbb, 0x143078808, 0x142409b50, 0x1425d9620, 0x141834d18> as *const ()), // sub_14003a340
    Replacement::draft(0x14003a3a0, "unknown_14003a3a0", tpl_42::<0x143a13bc0, 0x143a13bc3, 0x143078820, 0x142409b50, 0x1425d9630, 0x141834d18> as *const ()), // sub_14003a3a0
    Replacement::draft(0x14003a400, "unknown_14003a400", tpl_50::<0x143a13f40, 0x143a13f43, 0x143078838, 0x142409b50, 0x1425d9640, 0x141834d18> as *const ()), // sub_14003a400
    Replacement::draft(0x14003a460, "unknown_14003a460", tpl_34::<0x143a14230, 0x143a14233, 0x143078668, 0x142409b50, 0x1425d9650, 0x141834d18> as *const ()), // sub_14003a460
    Replacement::draft(0x14003a4c0, "unknown_14003a4c0", tpl_23::<0x143a144f0, 0x143a144f3, 0x1426cdcb0, 0x142409b50, 0x1425d9660, 0x141834d18> as *const ()), // sub_14003a4c0
    Replacement::draft(0x14003a520, "unknown_14003a520", tpl_35::<0x143a143d8, 0x143a143db, 0x143078678, 0x142409b50, 0x1425d9670, 0x141834d18> as *const ()), // sub_14003a520
    Replacement::draft(0x14003a580, "unknown_14003a580", tpl_36::<0x143a144a0, 0x143a144a3, 0x143078680, 0x142409b50, 0x1425d9680, 0x141834d18> as *const ()), // sub_14003a580
    Replacement::draft(0x14003a5e0, "unknown_14003a5e0", tpl_37::<0x143a14320, 0x143a14323, 0x143078690, 0x142409b50, 0x1425d9690, 0x141834d18> as *const ()), // sub_14003a5e0
    Replacement::draft(0x14003a640, "unknown_14003a640", tpl_38::<0x143a14280, 0x143a14283, 0x1430786a0, 0x142409b50, 0x1425d96a0, 0x141834d18> as *const ()), // sub_14003a640
    Replacement::draft(0x14003a6a0, "unknown_14003a6a0", tpl_39::<0x143a14518, 0x143a1451b, 0x1430786b0, 0x142409b50, 0x1425d96b0, 0x141834d18> as *const ()), // sub_14003a6a0
    Replacement::draft(0x14003a700, "unknown_14003a700", tpl_37::<0x143a141e0, 0x143a141e3, 0x1430786b8, 0x142409b50, 0x1425d96c0, 0x141834d18> as *const ()), // sub_14003a700
    Replacement::draft(0x14003a760, "unknown_14003a760", tpl_40::<0x143a145b8, 0x143a145bb, 0x1430786c4, 0x142409b50, 0x1425d96d0, 0x141834d18> as *const ()), // sub_14003a760
    Replacement::draft(0x14003a7c0, "unknown_14003a7c0", tpl_41::<0x143a142d0, 0x143a142d3, 0x1430786d0, 0x142409b50, 0x1425d96e0, 0x141834d18> as *const ()), // sub_14003a7c0
    Replacement::draft(0x14003a820, "unknown_14003a820", tpl_42::<0x143a14568, 0x143a1456b, 0x1430786e0, 0x142409b50, 0x1425d96f0, 0x141834d18> as *const ()), // sub_14003a820
    Replacement::draft(0x14003a880, "unknown_14003a880", tpl_34::<0x143a14590, 0x143a14593, 0x1430786f8, 0x142409b50, 0x1425d9700, 0x141834d18> as *const ()), // sub_14003a880
    Replacement::draft(0x14003a8e0, "unknown_14003a8e0", tpl_34::<0x143a14208, 0x143a1420b, 0x143078708, 0x142409b50, 0x1425d9710, 0x141834d18> as *const ()), // sub_14003a8e0
    Replacement::draft(0x14003a940, "unknown_14003a940", tpl_41::<0x143a142a8, 0x143a142ab, 0x143078718, 0x142409b50, 0x1425d9720, 0x141834d18> as *const ()), // sub_14003a940
    Replacement::draft(0x14003a9a0, "unknown_14003a9a0", tpl_43::<0x143a14348, 0x143a1434b, 0x143078728, 0x142409b50, 0x1425d9730, 0x141834d18> as *const ()), // sub_14003a9a0
    Replacement::draft(0x14003aa00, "unknown_14003aa00", tpl_44::<0x143a140f8, 0x143a140fb, 0x143078748, 0x142409b50, 0x1425d9740, 0x141834d18> as *const ()), // sub_14003aa00
    Replacement::draft(0x14003aa60, "unknown_14003aa60", tpl_39::<0x143a142f8, 0x143a142fb, 0x143078760, 0x142409b50, 0x1425d9750, 0x141834d18> as *const ()), // sub_14003aa60
    Replacement::draft(0x14003aac0, "unknown_14003aac0", tpl_45::<0x143a14258, 0x143a1425b, 0x143078768, 0x142409b50, 0x1425d9760, 0x141834d18> as *const ()), // sub_14003aac0
    Replacement::draft(0x14003ab20, "unknown_14003ab20", tpl_41::<0x143a14478, 0x143a1447b, 0x143078778, 0x142409b50, 0x1425d9770, 0x141834d18> as *const ()), // sub_14003ab20
    Replacement::draft(0x14003ab80, "unknown_14003ab80", tpl_37::<0x143a14120, 0x143a14123, 0x143078788, 0x142409b50, 0x1425d9780, 0x141834d18> as *const ()), // sub_14003ab80
    Replacement::draft(0x14003abe0, "unknown_14003abe0", tpl_46::<0x143a14400, 0x143a14403, 0x143078798, 0x142409b50, 0x1425d9790, 0x141834d18> as *const ()), // sub_14003abe0
    Replacement::draft(0x14003ac40, "unknown_14003ac40", tpl_47::<0x143a141b8, 0x143a141bb, 0x1430787b0, 0x142409b50, 0x1425d97a0, 0x141834d18> as *const ()), // sub_14003ac40
    Replacement::draft(0x14003aca0, "unknown_14003aca0", tpl_38::<0x143a14428, 0x143a1442b, 0x1430787d0, 0x142409b50, 0x1425d97b0, 0x141834d18> as *const ()), // sub_14003aca0
    Replacement::draft(0x14003ad00, "unknown_14003ad00", tpl_48::<0x143a14540, 0x143a14543, 0x1430787e0, 0x142409b50, 0x1425d97c0, 0x141834d18> as *const ()), // sub_14003ad00
    Replacement::draft(0x14003ad60, "unknown_14003ad60", tpl_49::<0x143a143b0, 0x143a143b3, 0x1430787f8, 0x142409b50, 0x1425d97d0, 0x141834d18> as *const ()), // sub_14003ad60
    Replacement::draft(0x14003adc0, "unknown_14003adc0", tpl_44::<0x143a144c8, 0x143a144cb, 0x143078808, 0x142409b50, 0x1425d97e0, 0x141834d18> as *const ()), // sub_14003adc0
    Replacement::draft(0x14003ae20, "unknown_14003ae20", tpl_42::<0x143a140d0, 0x143a140d3, 0x143078820, 0x142409b50, 0x1425d97f0, 0x141834d18> as *const ()), // sub_14003ae20
    Replacement::draft(0x14003ae80, "unknown_14003ae80", tpl_50::<0x143a14450, 0x143a14453, 0x143078838, 0x142409b50, 0x1425d9800, 0x141834d18> as *const ()), // sub_14003ae80
    Replacement::draft(0x14003aee0, "unknown_14003aee0", tpl_23::<0x143a14148, 0x143a1414b, 0x1426cdcb0, 0x142409b50, 0x1425d9810, 0x141834d18> as *const ()), // sub_14003aee0
    Replacement::draft(0x14003af40, "unknown_14003af40", tpl_23::<0x143a14190, 0x143a14193, 0x1426cdcb0, 0x142409b50, 0x1425d9820, 0x141834d18> as *const ()), // sub_14003af40
    Replacement::draft(0x14003b130, "unknown_14003b130", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844ba0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d9860, 0x141834d18> as *const ()), // sub_14003b130
    Replacement::draft(0x14003bcb0, "unknown_14003bcb0", tpl_23::<0x143a145e0, 0x143a145e3, 0x1426cdcb0, 0x142409b50, 0x1425d98d0, 0x141834d18> as *const ()), // sub_14003bcb0
    Replacement::draft(0x14003bd10, "unknown_14003bd10", tpl_23::<0x143a14628, 0x143a1462b, 0x1426cdcb0, 0x142409b50, 0x1425d98e0, 0x141834d18> as *const ()), // sub_14003bd10
    Replacement::draft(0x14003bf00, "unknown_14003bf00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844c30, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d9920, 0x141834d18> as *const ()), // sub_14003bf00
    Replacement::draft(0x14003c300, "unknown_14003c300", tpl_23::<0x143a14848, 0x143a1484b, 0x1426cdcb0, 0x142409b50, 0x1425d9990, 0x141834d18> as *const ()), // sub_14003c300
    Replacement::draft(0x14003c360, "unknown_14003c360", tpl_23::<0x143a14890, 0x143a14893, 0x1426cdcb0, 0x142409b50, 0x1425d99a0, 0x141834d18> as *const ()), // sub_14003c360
    Replacement::draft(0x14003c430, "unknown_14003c430", tpl_34::<0x143a149f8, 0x143a149fb, 0x143078668, 0x142409b50, 0x1425d99c0, 0x141834d18> as *const ()), // sub_14003c430
    Replacement::draft(0x14003c490, "unknown_14003c490", tpl_23::<0x143a14fc0, 0x143a14fc3, 0x1426cdcb0, 0x142409b50, 0x1425d99d0, 0x141834d18> as *const ()), // sub_14003c490
    Replacement::draft(0x14003c4f0, "unknown_14003c4f0", tpl_35::<0x143a14cf0, 0x143a14cf3, 0x143078678, 0x142409b50, 0x1425d99e0, 0x141834d18> as *const ()), // sub_14003c4f0
    Replacement::draft(0x14003c550, "unknown_14003c550", tpl_36::<0x143a14f48, 0x143a14f4b, 0x143078680, 0x142409b50, 0x1425d99f0, 0x141834d18> as *const ()), // sub_14003c550
    Replacement::draft(0x14003c5b0, "unknown_14003c5b0", tpl_37::<0x143a14bd8, 0x143a14bdb, 0x143078690, 0x142409b50, 0x1425d9a00, 0x141834d18> as *const ()), // sub_14003c5b0
    Replacement::draft(0x14003c610, "unknown_14003c610", tpl_38::<0x143a14a98, 0x143a14a9b, 0x1430786a0, 0x142409b50, 0x1425d9a10, 0x141834d18> as *const ()), // sub_14003c610
    Replacement::draft(0x14003c670, "unknown_14003c670", tpl_39::<0x143a14fe8, 0x143a14feb, 0x1430786b0, 0x142409b50, 0x1425d9a20, 0x141834d18> as *const ()), // sub_14003c670
    Replacement::draft(0x14003c6d0, "unknown_14003c6d0", tpl_37::<0x143a149a8, 0x143a149ab, 0x1430786b8, 0x142409b50, 0x1425d9a30, 0x141834d18> as *const ()), // sub_14003c6d0
    Replacement::draft(0x14003c730, "unknown_14003c730", tpl_40::<0x143a15100, 0x143a15103, 0x1430786c4, 0x142409b50, 0x1425d9a40, 0x141834d18> as *const ()), // sub_14003c730
    Replacement::draft(0x14003c790, "unknown_14003c790", tpl_41::<0x143a14b60, 0x143a14b63, 0x1430786d0, 0x142409b50, 0x1425d9a50, 0x141834d18> as *const ()), // sub_14003c790
    Replacement::draft(0x14003c7f0, "unknown_14003c7f0", tpl_42::<0x143a15038, 0x143a1503b, 0x1430786e0, 0x142409b50, 0x1425d9a60, 0x141834d18> as *const ()), // sub_14003c7f0
    Replacement::draft(0x14003c850, "unknown_14003c850", tpl_34::<0x143a15088, 0x143a1508b, 0x1430786f8, 0x142409b50, 0x1425d9a70, 0x141834d18> as *const ()), // sub_14003c850
    Replacement::draft(0x14003c8b0, "unknown_14003c8b0", tpl_34::<0x143a149d0, 0x143a149d3, 0x143078708, 0x142409b50, 0x1425d9a80, 0x141834d18> as *const ()), // sub_14003c8b0
    Replacement::draft(0x14003c910, "unknown_14003c910", tpl_41::<0x143a14ac0, 0x143a14ac3, 0x143078718, 0x142409b50, 0x1425d9a90, 0x141834d18> as *const ()), // sub_14003c910
    Replacement::draft(0x14003c970, "unknown_14003c970", tpl_43::<0x143a14c00, 0x143a14c03, 0x143078728, 0x142409b50, 0x1425d9aa0, 0x141834d18> as *const ()), // sub_14003c970
    Replacement::draft(0x14003c9d0, "unknown_14003c9d0", tpl_44::<0x143a146b8, 0x143a146bb, 0x143078748, 0x142409b50, 0x1425d9ab0, 0x141834d18> as *const ()), // sub_14003c9d0
    Replacement::draft(0x14003ca30, "unknown_14003ca30", tpl_39::<0x143a14b88, 0x143a14b8b, 0x143078760, 0x142409b50, 0x1425d9ac0, 0x141834d18> as *const ()), // sub_14003ca30
    Replacement::draft(0x14003ca90, "unknown_14003ca90", tpl_45::<0x143a14a70, 0x143a14a73, 0x143078768, 0x142409b50, 0x1425d9ad0, 0x141834d18> as *const ()), // sub_14003ca90
    Replacement::draft(0x14003caf0, "unknown_14003caf0", tpl_41::<0x143a14ef8, 0x143a14efb, 0x143078778, 0x142409b50, 0x1425d9ae0, 0x141834d18> as *const ()), // sub_14003caf0
    Replacement::draft(0x14003cb50, "unknown_14003cb50", tpl_37::<0x143a146e0, 0x143a146e3, 0x143078788, 0x142409b50, 0x1425d9af0, 0x141834d18> as *const ()), // sub_14003cb50
    Replacement::draft(0x14003cbb0, "unknown_14003cbb0", tpl_46::<0x143a14d90, 0x143a14d93, 0x143078798, 0x142409b50, 0x1425d9b00, 0x141834d18> as *const ()), // sub_14003cbb0
    Replacement::draft(0x14003cc10, "unknown_14003cc10", tpl_47::<0x143a14908, 0x143a1490b, 0x1430787b0, 0x142409b50, 0x1425d9b10, 0x141834d18> as *const ()), // sub_14003cc10
    Replacement::draft(0x14003cc70, "unknown_14003cc70", tpl_38::<0x143a14e30, 0x143a14e33, 0x1430787d0, 0x142409b50, 0x1425d9b20, 0x141834d18> as *const ()), // sub_14003cc70
    Replacement::draft(0x14003ccd0, "unknown_14003ccd0", tpl_48::<0x143a15010, 0x143a15013, 0x1430787e0, 0x142409b50, 0x1425d9b30, 0x141834d18> as *const ()), // sub_14003ccd0
    Replacement::draft(0x14003cd30, "unknown_14003cd30", tpl_49::<0x143a14cc8, 0x143a14ccb, 0x1430787f8, 0x142409b50, 0x1425d9b40, 0x141834d18> as *const ()), // sub_14003cd30
    Replacement::draft(0x14003cd90, "unknown_14003cd90", tpl_44::<0x143a14f98, 0x143a14f9b, 0x143078808, 0x142409b50, 0x1425d9b50, 0x141834d18> as *const ()), // sub_14003cd90
    Replacement::draft(0x14003cdf0, "unknown_14003cdf0", tpl_42::<0x143a14690, 0x143a14693, 0x143078820, 0x142409b50, 0x1425d9b60, 0x141834d18> as *const ()), // sub_14003cdf0
    Replacement::draft(0x14003ce50, "unknown_14003ce50", tpl_50::<0x143a14ea8, 0x143a14eab, 0x143078838, 0x142409b50, 0x1425d9b70, 0x141834d18> as *const ()), // sub_14003ce50
    Replacement::draft(0x14003cfd0, "unknown_14003cfd0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844cc0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d9ba0, 0x141834d18> as *const ()), // sub_14003cfd0
    Replacement::draft(0x14003d360, "unknown_14003d360", tpl_52::<0x143a14820, 0x143a14823, 0x143a15100, 0x143a15104, 0x143a15103, 0x143a15108, 0x142409b50, 0x1425d9be0, 0x141834d18> as *const ()), // sub_14003d360
    Replacement::draft(0x14003d420, "unknown_14003d420", tpl_38::<0x143a14d18, 0x143a14d1b, 0x143078950, 0x142409b50, 0x1425d9bf0, 0x141834d18> as *const ()), // sub_14003d420
    Replacement::draft(0x14003d480, "unknown_14003d480", tpl_53::<0x143a148e0, 0x143a148e3, 0x143078960, 0x142409b50, 0x1425d9c00, 0x141834d18> as *const ()), // sub_14003d480
    Replacement::draft(0x14003d4e0, "unknown_14003d4e0", tpl_45::<0x143a147a8, 0x143a147ab, 0x143078980, 0x142409b50, 0x1425d9c10, 0x141834d18> as *const ()), // sub_14003d4e0
    Replacement::draft(0x14003d540, "unknown_14003d540", tpl_35::<0x143a147f8, 0x143a147fb, 0x14307898c, 0x142409b50, 0x1425d9c20, 0x141834d18> as *const ()), // sub_14003d540
    Replacement::draft(0x14003d5a0, "unknown_14003d5a0", tpl_42::<0x143a15060, 0x143a15063, 0x1430786e0, 0x142409b50, 0x1425d9c30, 0x141834d18> as *const ()), // sub_14003d5a0
    Replacement::draft(0x14003d600, "unknown_14003d600", tpl_38::<0x143a14f70, 0x143a14f73, 0x143078998, 0x142409b50, 0x1425d9c40, 0x141834d18> as *const ()), // sub_14003d600
    Replacement::draft(0x14003d660, "unknown_14003d660", tpl_38::<0x143a14b10, 0x143a14b13, 0x1430789a8, 0x142409b50, 0x1425d9c50, 0x141834d18> as *const ()), // sub_14003d660
    Replacement::draft(0x14003d6c0, "unknown_14003d6c0", tpl_41::<0x143a14c28, 0x143a14c2b, 0x1430789b8, 0x142409b50, 0x1425d9c60, 0x141834d18> as *const ()), // sub_14003d6c0
    Replacement::draft(0x14003d720, "unknown_14003d720", tpl_34::<0x143a150b0, 0x143a150b3, 0x1430789c8, 0x142409b50, 0x1425d9c70, 0x141834d18> as *const ()), // sub_14003d720
    Replacement::draft(0x14003d780, "unknown_14003d780", tpl_39::<0x143a147d0, 0x143a147d3, 0x1430789d8, 0x142409b50, 0x1425d9c80, 0x141834d18> as *const ()), // sub_14003d780
    Replacement::draft(0x14003d7e0, "unknown_14003d7e0", tpl_41::<0x143a14ae8, 0x143a14aeb, 0x1430789e0, 0x142409b50, 0x1425d9c90, 0x141834d18> as *const ()), // sub_14003d7e0
    Replacement::draft(0x14003d840, "unknown_14003d840", tpl_34::<0x143a14d40, 0x143a14d43, 0x1430789f0, 0x142409b50, 0x1425d9ca0, 0x141834d18> as *const ()), // sub_14003d840
    Replacement::draft(0x14003d8a0, "unknown_14003d8a0", tpl_48::<0x143a14a20, 0x143a14a23, 0x143078a00, 0x142409b50, 0x1425d9cb0, 0x141834d18> as *const ()), // sub_14003d8a0
    Replacement::draft(0x14003d900, "unknown_14003d900", tpl_46::<0x143a14e58, 0x143a14e5b, 0x143078a18, 0x142409b50, 0x1425d9cc0, 0x141834d18> as *const ()), // sub_14003d900
    Replacement::draft(0x14003d960, "unknown_14003d960", tpl_47::<0x143a14db8, 0x143a14dbb, 0x143078a30, 0x142409b50, 0x1425d9cd0, 0x141834d18> as *const ()), // sub_14003d960
    Replacement::draft(0x14003d9c0, "unknown_14003d9c0", tpl_37::<0x143a14958, 0x143a1495b, 0x143078a50, 0x142409b50, 0x1425d9ce0, 0x141834d18> as *const ()), // sub_14003d9c0
    Replacement::draft(0x14003da20, "unknown_14003da20", tpl_45::<0x143a14bb0, 0x143a14bb3, 0x143078a60, 0x142409b50, 0x1425d9cf0, 0x141834d18> as *const ()), // sub_14003da20
    Replacement::draft(0x14003da80, "unknown_14003da80", tpl_54::<0x143a14c78, 0x143a14c7b, 0x143078a70, 0x142409b50, 0x1425d9d00, 0x141834d18> as *const ()), // sub_14003da80
    Replacement::draft(0x14003dae0, "unknown_14003dae0", tpl_40::<0x143a14930, 0x143a14933, 0x143078a88, 0x142409b50, 0x1425d9d10, 0x141834d18> as *const ()), // sub_14003dae0
    Replacement::draft(0x14003db40, "unknown_14003db40", tpl_50::<0x143a14980, 0x143a14983, 0x143078a90, 0x142409b50, 0x1425d9d20, 0x141834d18> as *const ()), // sub_14003db40
    Replacement::draft(0x14003dba0, "unknown_14003dba0", tpl_41::<0x143a14b38, 0x143a14b3b, 0x143078aa8, 0x142409b50, 0x1425d9d30, 0x141834d18> as *const ()), // sub_14003dba0
    Replacement::draft(0x14003dc00, "unknown_14003dc00", tpl_55::<0x143a14de0, 0x143a14de3, 0x143078ab8, 0x142409b50, 0x1425d9d40, 0x141834d18> as *const ()), // sub_14003dc00
    Replacement::draft(0x14003dc60, "unknown_14003dc60", tpl_41::<0x143a14e80, 0x143a14e83, 0x143078ac8, 0x142409b50, 0x1425d9d50, 0x141834d18> as *const ()), // sub_14003dc60
    Replacement::draft(0x14003dcc0, "unknown_14003dcc0", tpl_49::<0x143a14f20, 0x143a14f23, 0x143078ad8, 0x142409b50, 0x1425d9d60, 0x141834d18> as *const ()), // sub_14003dcc0
    Replacement::draft(0x14003dd20, "unknown_14003dd20", tpl_40::<0x143a14780, 0x143a14783, 0x143078ae4, 0x142409b50, 0x1425d9d70, 0x141834d18> as *const ()), // sub_14003dd20
    Replacement::draft(0x14003dd80, "unknown_14003dd80", tpl_44::<0x143a148b8, 0x143a148bb, 0x143078af0, 0x142409b50, 0x1425d9d80, 0x141834d18> as *const ()), // sub_14003dd80
    Replacement::draft(0x14003dde0, "unknown_14003dde0", tpl_46::<0x143a14758, 0x143a1475b, 0x143078b08, 0x142409b50, 0x1425d9d90, 0x141834d18> as *const ()), // sub_14003dde0
    Replacement::draft(0x14003de40, "unknown_14003de40", tpl_55::<0x143a14e08, 0x143a14e0b, 0x143078b20, 0x142409b50, 0x1425d9da0, 0x141834d18> as *const ()), // sub_14003de40
    Replacement::draft(0x14003dea0, "unknown_14003dea0", tpl_46::<0x143a14ca0, 0x143a14ca3, 0x143078b30, 0x142409b50, 0x1425d9db0, 0x141834d18> as *const ()), // sub_14003dea0
    Replacement::draft(0x14003df00, "unknown_14003df00", tpl_50::<0x143a14a48, 0x143a14a4b, 0x143078b48, 0x142409b50, 0x1425d9dc0, 0x141834d18> as *const ()), // sub_14003df00
    Replacement::draft(0x14003df60, "unknown_14003df60", tpl_42::<0x143a14708, 0x143a1470b, 0x143078b60, 0x142409b50, 0x1425d9dd0, 0x141834d18> as *const ()), // sub_14003df60
    Replacement::draft(0x14003dfc0, "unknown_14003dfc0", tpl_38::<0x143a14d68, 0x143a14d6b, 0x143078b78, 0x142409b50, 0x1425d9de0, 0x141834d18> as *const ()), // sub_14003dfc0
    Replacement::draft(0x14003e020, "unknown_14003e020", tpl_34::<0x143a150d8, 0x143a150db, 0x143078b88, 0x142409b50, 0x1425d9df0, 0x141834d18> as *const ()), // sub_14003e020
    Replacement::draft(0x14003e080, "unknown_14003e080", tpl_41::<0x143a14ed0, 0x143a14ed3, 0x143078b98, 0x142409b50, 0x1425d9e00, 0x141834d18> as *const ()), // sub_14003e080
    Replacement::draft(0x14003e0e0, "unknown_14003e0e0", tpl_36::<0x143a14c50, 0x143a14c53, 0x143078ba8, 0x142409b50, 0x1425d9e10, 0x141834d18> as *const ()), // sub_14003e0e0
    Replacement::draft(0x14003e140, "unknown_14003e140", tpl_34::<0x143a14730, 0x143a14733, 0x143078bb8, 0x142409b50, 0x1425d9e20, 0x141834d18> as *const ()), // sub_14003e140
    Replacement::draft(0x14003e560, "unknown_14003e560", tpl_23::<0x143a151a0, 0x143a151a3, 0x1426cdcb0, 0x142409b50, 0x1425d9e30, 0x141834d18> as *const ()), // sub_14003e560
    Replacement::draft(0x14003e5c0, "unknown_14003e5c0", tpl_23::<0x143a151e8, 0x143a151eb, 0x1426cdcb0, 0x142409b50, 0x1425d9e40, 0x141834d18> as *const ()), // sub_14003e5c0
    Replacement::draft(0x14003e7b0, "unknown_14003e7b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844d50, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425d9e80, 0x141834d18> as *const ()), // sub_14003e7b0
    Replacement::draft(0x14003eb40, "unknown_14003eb40", tpl_34::<0x143a15298, 0x143a1529b, 0x143078668, 0x142409b50, 0x1425d9ec0, 0x141834d18> as *const ()), // sub_14003eb40
    Replacement::draft(0x14003eba0, "unknown_14003eba0", tpl_23::<0x143a155f0, 0x143a155f3, 0x1426cdcb0, 0x142409b50, 0x1425d9ed0, 0x141834d18> as *const ()), // sub_14003eba0
    Replacement::draft(0x14003ec00, "unknown_14003ec00", tpl_35::<0x143a154d8, 0x143a154db, 0x143078678, 0x142409b50, 0x1425d9ee0, 0x141834d18> as *const ()), // sub_14003ec00
    Replacement::draft(0x14003ec60, "unknown_14003ec60", tpl_36::<0x143a155a0, 0x143a155a3, 0x143078680, 0x142409b50, 0x1425d9ef0, 0x141834d18> as *const ()), // sub_14003ec60
    Replacement::draft(0x14003ecc0, "unknown_14003ecc0", tpl_37::<0x143a15418, 0x143a1541b, 0x143078690, 0x142409b50, 0x1425d9f00, 0x141834d18> as *const ()), // sub_14003ecc0
    Replacement::draft(0x14003ed20, "unknown_14003ed20", tpl_38::<0x143a152e8, 0x143a152eb, 0x1430786a0, 0x142409b50, 0x1425d9f10, 0x141834d18> as *const ()), // sub_14003ed20
    Replacement::draft(0x14003ed80, "unknown_14003ed80", tpl_39::<0x143a15618, 0x143a1561b, 0x1430786b0, 0x142409b50, 0x1425d9f20, 0x141834d18> as *const ()), // sub_14003ed80
    Replacement::draft(0x14003ede0, "unknown_14003ede0", tpl_37::<0x143a15248, 0x143a1524b, 0x1430786b8, 0x142409b50, 0x1425d9f30, 0x141834d18> as *const ()), // sub_14003ede0
    Replacement::draft(0x14003ee40, "unknown_14003ee40", tpl_40::<0x143a156c8, 0x143a156cb, 0x1430786c4, 0x142409b50, 0x1425d9f40, 0x141834d18> as *const ()), // sub_14003ee40
    Replacement::draft(0x14003eea0, "unknown_14003eea0", tpl_41::<0x143a15338, 0x143a1533b, 0x1430786d0, 0x142409b50, 0x1425d9f50, 0x141834d18> as *const ()), // sub_14003eea0
    Replacement::draft(0x14003ef00, "unknown_14003ef00", tpl_42::<0x143a15668, 0x143a1566b, 0x1430786e0, 0x142409b50, 0x1425d9f60, 0x141834d18> as *const ()), // sub_14003ef00
    Replacement::draft(0x14003ef60, "unknown_14003ef60", tpl_34::<0x143a156a0, 0x143a156a3, 0x1430786f8, 0x142409b50, 0x1425d9f70, 0x141834d18> as *const ()), // sub_14003ef60
    Replacement::draft(0x14003efc0, "unknown_14003efc0", tpl_34::<0x143a15270, 0x143a15273, 0x143078708, 0x142409b50, 0x1425d9f80, 0x141834d18> as *const ()), // sub_14003efc0
    Replacement::draft(0x14003f020, "unknown_14003f020", tpl_41::<0x143a15310, 0x143a15313, 0x143078718, 0x142409b50, 0x1425d9f90, 0x141834d18> as *const ()), // sub_14003f020
    Replacement::draft(0x14003f080, "unknown_14003f080", tpl_43::<0x143a15440, 0x143a15443, 0x143078728, 0x142409b50, 0x1425d9fa0, 0x141834d18> as *const ()), // sub_14003f080
    Replacement::draft(0x14003f0e0, "unknown_14003f0e0", tpl_44::<0x143a15150, 0x143a15153, 0x143078748, 0x142409b50, 0x1425d9fb0, 0x141834d18> as *const ()), // sub_14003f0e0
    Replacement::draft(0x14003f140, "unknown_14003f140", tpl_39::<0x143a153a8, 0x143a153ab, 0x143078760, 0x142409b50, 0x1425d9fc0, 0x141834d18> as *const ()), // sub_14003f140
    Replacement::draft(0x14003f1a0, "unknown_14003f1a0", tpl_45::<0x143a152c0, 0x143a152c3, 0x143078768, 0x142409b50, 0x1425d9fd0, 0x141834d18> as *const ()), // sub_14003f1a0
    Replacement::draft(0x14003f200, "unknown_14003f200", tpl_41::<0x143a15578, 0x143a1557b, 0x143078778, 0x142409b50, 0x1425d9fe0, 0x141834d18> as *const ()), // sub_14003f200
    Replacement::draft(0x14003f260, "unknown_14003f260", tpl_37::<0x143a15178, 0x143a1517b, 0x143078788, 0x142409b50, 0x1425d9ff0, 0x141834d18> as *const ()), // sub_14003f260
    Replacement::draft(0x14003f2c0, "unknown_14003f2c0", tpl_46::<0x143a15500, 0x143a15503, 0x143078798, 0x142409b50, 0x1425da000, 0x141834d18> as *const ()), // sub_14003f2c0
    Replacement::draft(0x14003f320, "unknown_14003f320", tpl_47::<0x143a15220, 0x143a15223, 0x1430787b0, 0x142409b50, 0x1425da010, 0x141834d18> as *const ()), // sub_14003f320
    Replacement::draft(0x14003f380, "unknown_14003f380", tpl_38::<0x143a15528, 0x143a1552b, 0x1430787d0, 0x142409b50, 0x1425da020, 0x141834d18> as *const ()), // sub_14003f380
    Replacement::draft(0x14003f3e0, "unknown_14003f3e0", tpl_48::<0x143a15640, 0x143a15643, 0x1430787e0, 0x142409b50, 0x1425da030, 0x141834d18> as *const ()), // sub_14003f3e0
    Replacement::draft(0x14003f440, "unknown_14003f440", tpl_49::<0x143a154b0, 0x143a154b3, 0x1430787f8, 0x142409b50, 0x1425da040, 0x141834d18> as *const ()), // sub_14003f440
    Replacement::draft(0x14003f4a0, "unknown_14003f4a0", tpl_44::<0x143a155c8, 0x143a155cb, 0x143078808, 0x142409b50, 0x1425da050, 0x141834d18> as *const ()), // sub_14003f4a0
    Replacement::draft(0x14003f500, "unknown_14003f500", tpl_42::<0x143a15128, 0x143a1512b, 0x143078820, 0x142409b50, 0x1425da060, 0x141834d18> as *const ()), // sub_14003f500
    Replacement::draft(0x14003f560, "unknown_14003f560", tpl_50::<0x143a15550, 0x143a15553, 0x143078838, 0x142409b50, 0x1425da070, 0x141834d18> as *const ()), // sub_14003f560
    Replacement::draft(0x14003f630, "unknown_14003f630", fn_14003f630 as *const ()), // sub_14003f630
    Replacement::draft(0x14003f6d0, "unknown_14003f6d0", tpl_71::<0x14309fe90, 0x14309fe88, 0x14309fe9b, 0x14309fe93, 0x143a15698> as *const ()), // sub_14003f6d0
    Replacement::draft(0x14003f7b0, "unknown_14003f7b0", tpl_34::<0x143a15a58, 0x143a15a5b, 0x143078668, 0x142409b50, 0x1425da0f0, 0x141834d18> as *const ()), // sub_14003f7b0
    Replacement::draft(0x14003f810, "unknown_14003f810", tpl_23::<0x143a16068, 0x143a1606b, 0x1426cdcb0, 0x142409b50, 0x1425da100, 0x141834d18> as *const ()), // sub_14003f810
    Replacement::draft(0x14003f870, "unknown_14003f870", tpl_35::<0x143a15d98, 0x143a15d9b, 0x143078678, 0x142409b50, 0x1425da110, 0x141834d18> as *const ()), // sub_14003f870
    Replacement::draft(0x14003f8d0, "unknown_14003f8d0", tpl_36::<0x143a15ff0, 0x143a15ff3, 0x143078680, 0x142409b50, 0x1425da120, 0x141834d18> as *const ()), // sub_14003f8d0
    Replacement::draft(0x14003f930, "unknown_14003f930", tpl_37::<0x143a15c38, 0x143a15c3b, 0x143078690, 0x142409b50, 0x1425da130, 0x141834d18> as *const ()), // sub_14003f930
    Replacement::draft(0x14003f990, "unknown_14003f990", tpl_38::<0x143a15af8, 0x143a15afb, 0x1430786a0, 0x142409b50, 0x1425da140, 0x141834d18> as *const ()), // sub_14003f990
    Replacement::draft(0x14003f9f0, "unknown_14003f9f0", tpl_39::<0x143a16090, 0x143a16093, 0x1430786b0, 0x142409b50, 0x1425da150, 0x141834d18> as *const ()), // sub_14003f9f0
    Replacement::draft(0x14003fa50, "unknown_14003fa50", tpl_37::<0x143a15a08, 0x143a15a0b, 0x1430786b8, 0x142409b50, 0x1425da160, 0x141834d18> as *const ()), // sub_14003fa50
    Replacement::draft(0x14003fab0, "unknown_14003fab0", tpl_40::<0x143a161a8, 0x143a161ab, 0x1430786c4, 0x142409b50, 0x1425da170, 0x141834d18> as *const ()), // sub_14003fab0
    Replacement::draft(0x14003fb10, "unknown_14003fb10", tpl_41::<0x143a15bc0, 0x143a15bc3, 0x1430786d0, 0x142409b50, 0x1425da180, 0x141834d18> as *const ()), // sub_14003fb10
    Replacement::draft(0x14003fb70, "unknown_14003fb70", tpl_42::<0x143a160e0, 0x143a160e3, 0x1430786e0, 0x142409b50, 0x1425da190, 0x141834d18> as *const ()), // sub_14003fb70
    Replacement::draft(0x14003fbd0, "unknown_14003fbd0", tpl_34::<0x143a16130, 0x143a16133, 0x1430786f8, 0x142409b50, 0x1425da1a0, 0x141834d18> as *const ()), // sub_14003fbd0
    Replacement::draft(0x14003fc30, "unknown_14003fc30", tpl_34::<0x143a15a30, 0x143a15a33, 0x143078708, 0x142409b50, 0x1425da1b0, 0x141834d18> as *const ()), // sub_14003fc30
    Replacement::draft(0x14003fc90, "unknown_14003fc90", tpl_41::<0x143a15b20, 0x143a15b23, 0x143078718, 0x142409b50, 0x1425da1c0, 0x141834d18> as *const ()), // sub_14003fc90
    Replacement::draft(0x14003fcf0, "unknown_14003fcf0", tpl_43::<0x143a15c60, 0x143a15c63, 0x143078728, 0x142409b50, 0x1425da1d0, 0x141834d18> as *const ()), // sub_14003fcf0
    Replacement::draft(0x14003fd50, "unknown_14003fd50", tpl_44::<0x143a15718, 0x143a1571b, 0x143078748, 0x142409b50, 0x1425da1e0, 0x141834d18> as *const ()), // sub_14003fd50
    Replacement::draft(0x14003fdb0, "unknown_14003fdb0", tpl_39::<0x143a15be8, 0x143a15beb, 0x143078760, 0x142409b50, 0x1425da1f0, 0x141834d18> as *const ()), // sub_14003fdb0
    Replacement::draft(0x14003fe10, "unknown_14003fe10", tpl_45::<0x143a15ad0, 0x143a15ad3, 0x143078768, 0x142409b50, 0x1425da200, 0x141834d18> as *const ()), // sub_14003fe10
    Replacement::draft(0x14003fe70, "unknown_14003fe70", tpl_41::<0x143a15fa0, 0x143a15fa3, 0x143078778, 0x142409b50, 0x1425da210, 0x141834d18> as *const ()), // sub_14003fe70
    Replacement::draft(0x14003fed0, "unknown_14003fed0", tpl_37::<0x143a15740, 0x143a15743, 0x143078788, 0x142409b50, 0x1425da220, 0x141834d18> as *const ()), // sub_14003fed0
    Replacement::draft(0x14003ff30, "unknown_14003ff30", tpl_46::<0x143a15e38, 0x143a15e3b, 0x143078798, 0x142409b50, 0x1425da230, 0x141834d18> as *const ()), // sub_14003ff30
    Replacement::draft(0x14003ff90, "unknown_14003ff90", tpl_47::<0x143a15968, 0x143a1596b, 0x1430787b0, 0x142409b50, 0x1425da240, 0x141834d18> as *const ()), // sub_14003ff90
    Replacement::draft(0x14003fff0, "unknown_14003fff0", tpl_38::<0x143a15ed8, 0x143a15edb, 0x1430787d0, 0x142409b50, 0x1425da250, 0x141834d18> as *const ()), // sub_14003fff0
    Replacement::draft(0x140040050, "unknown_140040050", tpl_48::<0x143a160b8, 0x143a160bb, 0x1430787e0, 0x142409b50, 0x1425da260, 0x141834d18> as *const ()), // sub_140040050
    Replacement::draft(0x1400400b0, "unknown_1400400b0", tpl_49::<0x143a15d70, 0x143a15d73, 0x1430787f8, 0x142409b50, 0x1425da270, 0x141834d18> as *const ()), // sub_1400400b0
    Replacement::draft(0x140040110, "unknown_140040110", tpl_44::<0x143a16040, 0x143a16043, 0x143078808, 0x142409b50, 0x1425da280, 0x141834d18> as *const ()), // sub_140040110
    Replacement::draft(0x140040170, "unknown_140040170", tpl_42::<0x143a156f0, 0x143a156f3, 0x143078820, 0x142409b50, 0x1425da290, 0x141834d18> as *const ()), // sub_140040170
    Replacement::draft(0x1400401d0, "unknown_1400401d0", tpl_50::<0x143a15f50, 0x143a15f53, 0x143078838, 0x142409b50, 0x1425da2a0, 0x141834d18> as *const ()), // sub_1400401d0
    Replacement::draft(0x140040230, "unknown_140040230", tpl_23::<0x143a158a8, 0x143a158ab, 0x1426cdcb0, 0x142409b50, 0x1425da2b0, 0x141834d18> as *const ()), // sub_140040230
    Replacement::draft(0x140040290, "unknown_140040290", tpl_23::<0x143a158f0, 0x143a158f3, 0x1426cdcb0, 0x142409b50, 0x1425da2c0, 0x141834d18> as *const ()), // sub_140040290
    Replacement::draft(0x140040480, "unknown_140040480", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844de0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425da300, 0x141834d18> as *const ()), // sub_140040480
    Replacement::draft(0x140040810, "unknown_140040810", tpl_52::<0x143a15880, 0x143a15883, 0x143a161a8, 0x143a161ac, 0x143a161ab, 0x143a161b0, 0x142409b50, 0x1425da340, 0x141834d18> as *const ()), // sub_140040810
    Replacement::draft(0x1400408d0, "unknown_1400408d0", tpl_38::<0x143a15dc0, 0x143a15dc3, 0x143078950, 0x142409b50, 0x1425da350, 0x141834d18> as *const ()), // sub_1400408d0
    Replacement::draft(0x140040930, "unknown_140040930", tpl_53::<0x143a15940, 0x143a15943, 0x143078960, 0x142409b50, 0x1425da360, 0x141834d18> as *const ()), // sub_140040930
    Replacement::draft(0x140040990, "unknown_140040990", tpl_45::<0x143a15808, 0x143a1580b, 0x143078980, 0x142409b50, 0x1425da370, 0x141834d18> as *const ()), // sub_140040990
    Replacement::draft(0x1400409f0, "unknown_1400409f0", tpl_35::<0x143a15858, 0x143a1585b, 0x14307898c, 0x142409b50, 0x1425da380, 0x141834d18> as *const ()), // sub_1400409f0
    Replacement::draft(0x140040a50, "unknown_140040a50", tpl_42::<0x143a16108, 0x143a1610b, 0x1430786e0, 0x142409b50, 0x1425da390, 0x141834d18> as *const ()), // sub_140040a50
    Replacement::draft(0x140040ab0, "unknown_140040ab0", tpl_38::<0x143a16018, 0x143a1601b, 0x143078998, 0x142409b50, 0x1425da3a0, 0x141834d18> as *const ()), // sub_140040ab0
    Replacement::draft(0x140040b10, "unknown_140040b10", tpl_38::<0x143a15b70, 0x143a15b73, 0x1430789a8, 0x142409b50, 0x1425da3b0, 0x141834d18> as *const ()), // sub_140040b10
    Replacement::draft(0x140040b70, "unknown_140040b70", tpl_41::<0x143a15cd0, 0x143a15cd3, 0x1430789b8, 0x142409b50, 0x1425da3c0, 0x141834d18> as *const ()), // sub_140040b70
    Replacement::draft(0x140040bd0, "unknown_140040bd0", tpl_34::<0x143a16158, 0x143a1615b, 0x1430789c8, 0x142409b50, 0x1425da3d0, 0x141834d18> as *const ()), // sub_140040bd0
    Replacement::draft(0x140040c30, "unknown_140040c30", tpl_39::<0x143a15830, 0x143a15833, 0x1430789d8, 0x142409b50, 0x1425da3e0, 0x141834d18> as *const ()), // sub_140040c30
    Replacement::draft(0x140040c90, "unknown_140040c90", tpl_41::<0x143a15b48, 0x143a15b4b, 0x1430789e0, 0x142409b50, 0x1425da3f0, 0x141834d18> as *const ()), // sub_140040c90
    Replacement::draft(0x140040cf0, "unknown_140040cf0", tpl_34::<0x143a15de8, 0x143a15deb, 0x1430789f0, 0x142409b50, 0x1425da400, 0x141834d18> as *const ()), // sub_140040cf0
    Replacement::draft(0x140040d50, "unknown_140040d50", tpl_48::<0x143a15a80, 0x143a15a83, 0x143078a00, 0x142409b50, 0x1425da410, 0x141834d18> as *const ()), // sub_140040d50
    Replacement::draft(0x140040db0, "unknown_140040db0", tpl_46::<0x143a15f00, 0x143a15f03, 0x143078a18, 0x142409b50, 0x1425da420, 0x141834d18> as *const ()), // sub_140040db0
    Replacement::draft(0x140040e10, "unknown_140040e10", tpl_47::<0x143a15e60, 0x143a15e63, 0x143078a30, 0x142409b50, 0x1425da430, 0x141834d18> as *const ()), // sub_140040e10
    Replacement::draft(0x140040e70, "unknown_140040e70", tpl_37::<0x143a159b8, 0x143a159bb, 0x143078a50, 0x142409b50, 0x1425da440, 0x141834d18> as *const ()), // sub_140040e70
    Replacement::draft(0x140040ed0, "unknown_140040ed0", tpl_45::<0x143a15c10, 0x143a15c13, 0x143078a60, 0x142409b50, 0x1425da450, 0x141834d18> as *const ()), // sub_140040ed0
    Replacement::draft(0x140040f30, "unknown_140040f30", tpl_54::<0x143a15d20, 0x143a15d23, 0x143078a70, 0x142409b50, 0x1425da460, 0x141834d18> as *const ()), // sub_140040f30
    Replacement::draft(0x140040f90, "unknown_140040f90", tpl_40::<0x143a15990, 0x143a15993, 0x143078a88, 0x142409b50, 0x1425da470, 0x141834d18> as *const ()), // sub_140040f90
    Replacement::draft(0x140040ff0, "unknown_140040ff0", tpl_50::<0x143a159e0, 0x143a159e3, 0x143078a90, 0x142409b50, 0x1425da480, 0x141834d18> as *const ()), // sub_140040ff0
    Replacement::draft(0x140041050, "unknown_140041050", tpl_41::<0x143a15b98, 0x143a15b9b, 0x143078aa8, 0x142409b50, 0x1425da490, 0x141834d18> as *const ()), // sub_140041050
    Replacement::draft(0x1400410b0, "unknown_1400410b0", tpl_55::<0x143a15e88, 0x143a15e8b, 0x143078ab8, 0x142409b50, 0x1425da4a0, 0x141834d18> as *const ()), // sub_1400410b0
    Replacement::draft(0x140041110, "unknown_140041110", tpl_41::<0x143a15f28, 0x143a15f2b, 0x143078ac8, 0x142409b50, 0x1425da4b0, 0x141834d18> as *const ()), // sub_140041110
    Replacement::draft(0x140041170, "unknown_140041170", tpl_49::<0x143a15fc8, 0x143a15fcb, 0x143078ad8, 0x142409b50, 0x1425da4c0, 0x141834d18> as *const ()), // sub_140041170
    Replacement::draft(0x1400411d0, "unknown_1400411d0", tpl_40::<0x143a157e0, 0x143a157e3, 0x143078ae4, 0x142409b50, 0x1425da4d0, 0x141834d18> as *const ()), // sub_1400411d0
    Replacement::draft(0x140041230, "unknown_140041230", tpl_44::<0x143a15918, 0x143a1591b, 0x143078af0, 0x142409b50, 0x1425da4e0, 0x141834d18> as *const ()), // sub_140041230
    Replacement::draft(0x140041290, "unknown_140041290", tpl_46::<0x143a157b8, 0x143a157bb, 0x143078b08, 0x142409b50, 0x1425da4f0, 0x141834d18> as *const ()), // sub_140041290
    Replacement::draft(0x1400412f0, "unknown_1400412f0", tpl_55::<0x143a15eb0, 0x143a15eb3, 0x143078b20, 0x142409b50, 0x1425da500, 0x141834d18> as *const ()), // sub_1400412f0
    Replacement::draft(0x140041350, "unknown_140041350", tpl_46::<0x143a15d48, 0x143a15d4b, 0x143078b30, 0x142409b50, 0x1425da510, 0x141834d18> as *const ()), // sub_140041350
    Replacement::draft(0x1400413b0, "unknown_1400413b0", tpl_50::<0x143a15aa8, 0x143a15aab, 0x143078b48, 0x142409b50, 0x1425da520, 0x141834d18> as *const ()), // sub_1400413b0
    Replacement::draft(0x140041410, "unknown_140041410", tpl_42::<0x143a15768, 0x143a1576b, 0x143078b60, 0x142409b50, 0x1425da530, 0x141834d18> as *const ()), // sub_140041410
    Replacement::draft(0x140041470, "unknown_140041470", tpl_38::<0x143a15e10, 0x143a15e13, 0x143078b78, 0x142409b50, 0x1425da540, 0x141834d18> as *const ()), // sub_140041470
    Replacement::draft(0x1400414d0, "unknown_1400414d0", tpl_34::<0x143a16180, 0x143a16183, 0x143078b88, 0x142409b50, 0x1425da550, 0x141834d18> as *const ()), // sub_1400414d0
    Replacement::draft(0x140041530, "unknown_140041530", tpl_41::<0x143a15f78, 0x143a15f7b, 0x143078b98, 0x142409b50, 0x1425da560, 0x141834d18> as *const ()), // sub_140041530
    Replacement::draft(0x140041590, "unknown_140041590", tpl_36::<0x143a15cf8, 0x143a15cfb, 0x143078ba8, 0x142409b50, 0x1425da570, 0x141834d18> as *const ()), // sub_140041590
    Replacement::draft(0x1400415f0, "unknown_1400415f0", tpl_34::<0x143a15790, 0x143a15793, 0x143078bb8, 0x142409b50, 0x1425da580, 0x141834d18> as *const ()), // sub_1400415f0
    Replacement::draft(0x1400416c0, "unknown_1400416c0", tpl_23::<0x143a16248, 0x143a1624b, 0x1426cdcb0, 0x142409b50, 0x1425da5c0, 0x141834d18> as *const ()), // sub_1400416c0
    Replacement::draft(0x140041720, "unknown_140041720", tpl_23::<0x143a16290, 0x143a16293, 0x1426cdcb0, 0x142409b50, 0x1425da5d0, 0x141834d18> as *const ()), // sub_140041720
    Replacement::draft(0x140041910, "unknown_140041910", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143844eb0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425da610, 0x141834d18> as *const ()), // sub_140041910
    Replacement::draft(0x140041d10, "unknown_140041d10", tpl_34::<0x143a16330, 0x143a16333, 0x143078668, 0x142409b50, 0x1425da680, 0x141834d18> as *const ()), // sub_140041d10
    Replacement::draft(0x140041d70, "unknown_140041d70", tpl_23::<0x143a16660, 0x143a16663, 0x1426cdcb0, 0x142409b50, 0x1425da690, 0x141834d18> as *const ()), // sub_140041d70
    Replacement::draft(0x140041dd0, "unknown_140041dd0", tpl_35::<0x143a16548, 0x143a1654b, 0x143078678, 0x142409b50, 0x1425da6a0, 0x141834d18> as *const ()), // sub_140041dd0
    Replacement::draft(0x140041e30, "unknown_140041e30", tpl_36::<0x143a16610, 0x143a16613, 0x143078680, 0x142409b50, 0x1425da6b0, 0x141834d18> as *const ()), // sub_140041e30
    Replacement::draft(0x140041e90, "unknown_140041e90", tpl_37::<0x143a16420, 0x143a16423, 0x143078690, 0x142409b50, 0x1425da6c0, 0x141834d18> as *const ()), // sub_140041e90
    Replacement::draft(0x140041ef0, "unknown_140041ef0", tpl_38::<0x143a16380, 0x143a16383, 0x1430786a0, 0x142409b50, 0x1425da6d0, 0x141834d18> as *const ()), // sub_140041ef0
    Replacement::draft(0x140041f50, "unknown_140041f50", tpl_39::<0x143a16688, 0x143a1668b, 0x1430786b0, 0x142409b50, 0x1425da6e0, 0x141834d18> as *const ()), // sub_140041f50
    Replacement::draft(0x140041fb0, "unknown_140041fb0", tpl_37::<0x143a162e0, 0x143a162e3, 0x1430786b8, 0x142409b50, 0x1425da6f0, 0x141834d18> as *const ()), // sub_140041fb0
    Replacement::draft(0x140042010, "unknown_140042010", tpl_40::<0x143a16728, 0x143a1672b, 0x1430786c4, 0x142409b50, 0x1425da700, 0x141834d18> as *const ()), // sub_140042010
    Replacement::draft(0x140042070, "unknown_140042070", tpl_41::<0x143a163d0, 0x143a163d3, 0x1430786d0, 0x142409b50, 0x1425da710, 0x141834d18> as *const ()), // sub_140042070
    Replacement::draft(0x1400420d0, "unknown_1400420d0", tpl_42::<0x143a166d8, 0x143a166db, 0x1430786e0, 0x142409b50, 0x1425da720, 0x141834d18> as *const ()), // sub_1400420d0
    Replacement::draft(0x140042130, "unknown_140042130", tpl_34::<0x143a16700, 0x143a16703, 0x1430786f8, 0x142409b50, 0x1425da730, 0x141834d18> as *const ()), // sub_140042130
    Replacement::draft(0x140042190, "unknown_140042190", tpl_34::<0x143a16308, 0x143a1630b, 0x143078708, 0x142409b50, 0x1425da740, 0x141834d18> as *const ()), // sub_140042190
    Replacement::draft(0x1400421f0, "unknown_1400421f0", tpl_41::<0x143a163a8, 0x143a163ab, 0x143078718, 0x142409b50, 0x1425da750, 0x141834d18> as *const ()), // sub_1400421f0
    Replacement::draft(0x140042250, "unknown_140042250", tpl_43::<0x143a164b8, 0x143a164bb, 0x143078728, 0x142409b50, 0x1425da760, 0x141834d18> as *const ()), // sub_140042250
    Replacement::draft(0x1400422b0, "unknown_1400422b0", tpl_44::<0x143a161f8, 0x143a161fb, 0x143078748, 0x142409b50, 0x1425da770, 0x141834d18> as *const ()), // sub_1400422b0
    Replacement::draft(0x140042310, "unknown_140042310", tpl_39::<0x143a163f8, 0x143a163fb, 0x143078760, 0x142409b50, 0x1425da780, 0x141834d18> as *const ()), // sub_140042310
    Replacement::draft(0x140042370, "unknown_140042370", tpl_45::<0x143a16358, 0x143a1635b, 0x143078768, 0x142409b50, 0x1425da790, 0x141834d18> as *const ()), // sub_140042370
    Replacement::draft(0x1400423d0, "unknown_1400423d0", tpl_41::<0x143a165e8, 0x143a165eb, 0x143078778, 0x142409b50, 0x1425da7a0, 0x141834d18> as *const ()), // sub_1400423d0
    Replacement::draft(0x140042430, "unknown_140042430", tpl_37::<0x143a16220, 0x143a16223, 0x143078788, 0x142409b50, 0x1425da7b0, 0x141834d18> as *const ()), // sub_140042430
    Replacement::draft(0x140042490, "unknown_140042490", tpl_46::<0x143a16570, 0x143a16573, 0x143078798, 0x142409b50, 0x1425da7c0, 0x141834d18> as *const ()), // sub_140042490
    Replacement::draft(0x1400424f0, "unknown_1400424f0", tpl_47::<0x143a162b8, 0x143a162bb, 0x1430787b0, 0x142409b50, 0x1425da7d0, 0x141834d18> as *const ()), // sub_1400424f0
    Replacement::draft(0x140042550, "unknown_140042550", tpl_38::<0x143a16598, 0x143a1659b, 0x1430787d0, 0x142409b50, 0x1425da7e0, 0x141834d18> as *const ()), // sub_140042550
    Replacement::draft(0x1400425b0, "unknown_1400425b0", tpl_48::<0x143a166b0, 0x143a166b3, 0x1430787e0, 0x142409b50, 0x1425da7f0, 0x141834d18> as *const ()), // sub_1400425b0
    Replacement::draft(0x140042610, "unknown_140042610", tpl_49::<0x143a16520, 0x143a16523, 0x1430787f8, 0x142409b50, 0x1425da800, 0x141834d18> as *const ()), // sub_140042610
    Replacement::draft(0x140042670, "unknown_140042670", tpl_44::<0x143a16638, 0x143a1663b, 0x143078808, 0x142409b50, 0x1425da810, 0x141834d18> as *const ()), // sub_140042670
    Replacement::draft(0x1400426d0, "unknown_1400426d0", tpl_42::<0x143a161d0, 0x143a161d3, 0x143078820, 0x142409b50, 0x1425da820, 0x141834d18> as *const ()), // sub_1400426d0
    Replacement::draft(0x140042730, "unknown_140042730", tpl_50::<0x143a165c0, 0x143a165c3, 0x143078838, 0x142409b50, 0x1425da830, 0x141834d18> as *const ()), // sub_140042730
    Replacement::draft(0x140042790, "unknown_140042790", fn_140042790 as *const ()), // sub_140042790
    Replacement::draft(0x1400429f0, "unknown_1400429f0", fn_1400429f0 as *const ()), // sub_1400429f0
    Replacement::draft(0x140042c50, "unknown_140042c50", tpl_74::<0x1430a00b8, 0x142409b50, 0x143a16480, 0x143a16483, 0x143a16484, 0x143a16488, 0x1401df680, 0x14183acc8, 0x142408f50, 0x14183acbc, 0x143a164a8, 0x143a164b0, 0x140267b90, 0x14240ab10, 0x1425da8c0, 0x141834d18> as *const ()), // sub_140042c50
    Replacement::draft(0x140042eb0, "unknown_140042eb0", tpl_23::<0x143a16788, 0x143a1678b, 0x1426cdcb0, 0x142409b50, 0x1425da900, 0x141834d18> as *const ()), // sub_140042eb0
    Replacement::draft(0x140042f10, "unknown_140042f10", tpl_23::<0x143a167d0, 0x143a167d3, 0x1426cdcb0, 0x142409b50, 0x1425da910, 0x141834d18> as *const ()), // sub_140042f10
    Replacement::draft(0x140042fe0, "unknown_140042fe0", tpl_23::<0x143a16870, 0x143a16873, 0x1426cdcb0, 0x142409b50, 0x1425da930, 0x141834d18> as *const ()), // sub_140042fe0
    Replacement::draft(0x140043040, "unknown_140043040", tpl_23::<0x143a168b8, 0x143a168bb, 0x1426cdcb0, 0x142409b50, 0x1425da940, 0x141834d18> as *const ()), // sub_140043040
    Replacement::draft(0x140043230, "unknown_140043230", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143845040, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425da980, 0x141834d18> as *const ()), // sub_140043230
    Replacement::draft(0x140043630, "unknown_140043630", tpl_34::<0x143a169a0, 0x143a169a3, 0x143078668, 0x142409b50, 0x1425da9f0, 0x141834d18> as *const ()), // sub_140043630
    Replacement::draft(0x140043690, "unknown_140043690", tpl_23::<0x143a16d48, 0x143a16d4b, 0x1426cdcb0, 0x142409b50, 0x1425daa00, 0x141834d18> as *const ()), // sub_140043690
    Replacement::draft(0x1400436f0, "unknown_1400436f0", tpl_35::<0x143a16be8, 0x143a16beb, 0x143078678, 0x142409b50, 0x1425daa10, 0x141834d18> as *const ()), // sub_1400436f0
    Replacement::draft(0x140043750, "unknown_140043750", tpl_36::<0x143a16cf8, 0x143a16cfb, 0x143078680, 0x142409b50, 0x1425daa20, 0x141834d18> as *const ()), // sub_140043750
    Replacement::draft(0x1400437b0, "unknown_1400437b0", tpl_37::<0x143a16b28, 0x143a16b2b, 0x143078690, 0x142409b50, 0x1425daa30, 0x141834d18> as *const ()), // sub_1400437b0
    Replacement::draft(0x140043810, "unknown_140043810", tpl_38::<0x143a169f0, 0x143a169f3, 0x1430786a0, 0x142409b50, 0x1425daa40, 0x141834d18> as *const ()), // sub_140043810
    Replacement::draft(0x140043870, "unknown_140043870", tpl_39::<0x143a16d70, 0x143a16d73, 0x1430786b0, 0x142409b50, 0x1425daa50, 0x141834d18> as *const ()), // sub_140043870
    Replacement::draft(0x1400438d0, "unknown_1400438d0", tpl_37::<0x143a16950, 0x143a16953, 0x1430786b8, 0x142409b50, 0x1425daa60, 0x141834d18> as *const ()), // sub_1400438d0
    Replacement::draft(0x140043930, "unknown_140043930", tpl_40::<0x143a16e10, 0x143a16e13, 0x1430786c4, 0x142409b50, 0x1425daa70, 0x141834d18> as *const ()), // sub_140043930
    Replacement::draft(0x140043990, "unknown_140043990", tpl_41::<0x143a16a40, 0x143a16a43, 0x1430786d0, 0x142409b50, 0x1425daa80, 0x141834d18> as *const ()), // sub_140043990
    Replacement::draft(0x1400439f0, "unknown_1400439f0", tpl_42::<0x143a16dc0, 0x143a16dc3, 0x1430786e0, 0x142409b50, 0x1425daa90, 0x141834d18> as *const ()), // sub_1400439f0
    Replacement::draft(0x140043a50, "unknown_140043a50", tpl_34::<0x143a16de8, 0x143a16deb, 0x1430786f8, 0x142409b50, 0x1425daaa0, 0x141834d18> as *const ()), // sub_140043a50
    Replacement::draft(0x140043ab0, "unknown_140043ab0", tpl_34::<0x143a16978, 0x143a1697b, 0x143078708, 0x142409b50, 0x1425daab0, 0x141834d18> as *const ()), // sub_140043ab0
    Replacement::draft(0x140043b10, "unknown_140043b10", tpl_41::<0x143a16a18, 0x143a16a1b, 0x143078718, 0x142409b50, 0x1425daac0, 0x141834d18> as *const ()), // sub_140043b10
    Replacement::draft(0x140043b70, "unknown_140043b70", tpl_43::<0x143a16b50, 0x143a16b53, 0x143078728, 0x142409b50, 0x1425daad0, 0x141834d18> as *const ()), // sub_140043b70
    Replacement::draft(0x140043bd0, "unknown_140043bd0", tpl_44::<0x143a16820, 0x143a16823, 0x143078748, 0x142409b50, 0x1425daae0, 0x141834d18> as *const ()), // sub_140043bd0
    Replacement::draft(0x140043c30, "unknown_140043c30", tpl_39::<0x143a16ab8, 0x143a16abb, 0x143078760, 0x142409b50, 0x1425daaf0, 0x141834d18> as *const ()), // sub_140043c30
    Replacement::draft(0x140043c90, "unknown_140043c90", tpl_45::<0x143a169c8, 0x143a169cb, 0x143078768, 0x142409b50, 0x1425dab00, 0x141834d18> as *const ()), // sub_140043c90
    Replacement::draft(0x140043cf0, "unknown_140043cf0", tpl_41::<0x143a16cd0, 0x143a16cd3, 0x143078778, 0x142409b50, 0x1425dab10, 0x141834d18> as *const ()), // sub_140043cf0
    Replacement::draft(0x140043d50, "unknown_140043d50", tpl_37::<0x143a16848, 0x143a1684b, 0x143078788, 0x142409b50, 0x1425dab20, 0x141834d18> as *const ()), // sub_140043d50
    Replacement::draft(0x140043db0, "unknown_140043db0", tpl_46::<0x143a16c10, 0x143a16c13, 0x143078798, 0x142409b50, 0x1425dab30, 0x141834d18> as *const ()), // sub_140043db0
    Replacement::draft(0x140043e10, "unknown_140043e10", tpl_47::<0x143a16928, 0x143a1692b, 0x1430787b0, 0x142409b50, 0x1425dab40, 0x141834d18> as *const ()), // sub_140043e10
    Replacement::draft(0x140043e70, "unknown_140043e70", tpl_38::<0x143a16c38, 0x143a16c3b, 0x1430787d0, 0x142409b50, 0x1425dab50, 0x141834d18> as *const ()), // sub_140043e70
    Replacement::draft(0x140043ed0, "unknown_140043ed0", tpl_48::<0x143a16d98, 0x143a16d9b, 0x1430787e0, 0x142409b50, 0x1425dab60, 0x141834d18> as *const ()), // sub_140043ed0
    Replacement::draft(0x140043f30, "unknown_140043f30", tpl_49::<0x143a16bc0, 0x143a16bc3, 0x1430787f8, 0x142409b50, 0x1425dab70, 0x141834d18> as *const ()), // sub_140043f30
    Replacement::draft(0x140043f90, "unknown_140043f90", tpl_44::<0x143a16d20, 0x143a16d23, 0x143078808, 0x142409b50, 0x1425dab80, 0x141834d18> as *const ()), // sub_140043f90
    Replacement::draft(0x140043ff0, "unknown_140043ff0", tpl_42::<0x143a167f8, 0x143a167fb, 0x143078820, 0x142409b50, 0x1425dab90, 0x141834d18> as *const ()), // sub_140043ff0
    Replacement::draft(0x140044050, "unknown_140044050", tpl_50::<0x143a16ca8, 0x143a16cab, 0x143078838, 0x142409b50, 0x1425daba0, 0x141834d18> as *const ()), // sub_140044050
    Replacement::draft(0x140044130, "unknown_140044130", tpl_23::<0x143a16eb0, 0x143a16eb3, 0x1426cdcb0, 0x142409b50, 0x1425dac30, 0x141834d18> as *const ()), // sub_140044130
    Replacement::draft(0x140044190, "unknown_140044190", tpl_23::<0x143a16ef8, 0x143a16efb, 0x1426cdcb0, 0x142409b50, 0x1425dac40, 0x141834d18> as *const ()), // sub_140044190
    Replacement::draft(0x140044380, "unknown_140044380", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438450d0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dac80, 0x141834d18> as *const ()), // sub_140044380
    Replacement::draft(0x140044780, "unknown_140044780", tpl_34::<0x143a16f98, 0x143a16f9b, 0x143078668, 0x142409b50, 0x1425dacf0, 0x141834d18> as *const ()), // sub_140044780
    Replacement::draft(0x1400447e0, "unknown_1400447e0", tpl_23::<0x143a17260, 0x143a17263, 0x1426cdcb0, 0x142409b50, 0x1425dad00, 0x141834d18> as *const ()), // sub_1400447e0
    Replacement::draft(0x140044840, "unknown_140044840", tpl_35::<0x143a17148, 0x143a1714b, 0x143078678, 0x142409b50, 0x1425dad10, 0x141834d18> as *const ()), // sub_140044840
    Replacement::draft(0x1400448a0, "unknown_1400448a0", tpl_36::<0x143a17210, 0x143a17213, 0x143078680, 0x142409b50, 0x1425dad20, 0x141834d18> as *const ()), // sub_1400448a0
    Replacement::draft(0x140044900, "unknown_140044900", tpl_37::<0x143a17088, 0x143a1708b, 0x143078690, 0x142409b50, 0x1425dad30, 0x141834d18> as *const ()), // sub_140044900
    Replacement::draft(0x140044960, "unknown_140044960", tpl_38::<0x143a16fe8, 0x143a16feb, 0x1430786a0, 0x142409b50, 0x1425dad40, 0x141834d18> as *const ()), // sub_140044960
    Replacement::draft(0x1400449c0, "unknown_1400449c0", tpl_39::<0x143a17288, 0x143a1728b, 0x1430786b0, 0x142409b50, 0x1425dad50, 0x141834d18> as *const ()), // sub_1400449c0
    Replacement::draft(0x140044a20, "unknown_140044a20", tpl_37::<0x143a16f48, 0x143a16f4b, 0x1430786b8, 0x142409b50, 0x1425dad60, 0x141834d18> as *const ()), // sub_140044a20
    Replacement::draft(0x140044a80, "unknown_140044a80", tpl_40::<0x143a17328, 0x143a1732b, 0x1430786c4, 0x142409b50, 0x1425dad70, 0x141834d18> as *const ()), // sub_140044a80
    Replacement::draft(0x140044ae0, "unknown_140044ae0", tpl_41::<0x143a17038, 0x143a1703b, 0x1430786d0, 0x142409b50, 0x1425dad80, 0x141834d18> as *const ()), // sub_140044ae0
    Replacement::draft(0x140044b40, "unknown_140044b40", tpl_42::<0x143a172d8, 0x143a172db, 0x1430786e0, 0x142409b50, 0x1425dad90, 0x141834d18> as *const ()), // sub_140044b40
    Replacement::draft(0x140044ba0, "unknown_140044ba0", tpl_34::<0x143a17300, 0x143a17303, 0x1430786f8, 0x142409b50, 0x1425dada0, 0x141834d18> as *const ()), // sub_140044ba0
    Replacement::draft(0x140044c00, "unknown_140044c00", tpl_34::<0x143a16f70, 0x143a16f73, 0x143078708, 0x142409b50, 0x1425dadb0, 0x141834d18> as *const ()), // sub_140044c00
    Replacement::draft(0x140044c60, "unknown_140044c60", tpl_41::<0x143a17010, 0x143a17013, 0x143078718, 0x142409b50, 0x1425dadc0, 0x141834d18> as *const ()), // sub_140044c60
    Replacement::draft(0x140044cc0, "unknown_140044cc0", tpl_43::<0x143a170b0, 0x143a170b3, 0x143078728, 0x142409b50, 0x1425dadd0, 0x141834d18> as *const ()), // sub_140044cc0
    Replacement::draft(0x140044d20, "unknown_140044d20", tpl_44::<0x143a16e60, 0x143a16e63, 0x143078748, 0x142409b50, 0x1425dade0, 0x141834d18> as *const ()), // sub_140044d20
    Replacement::draft(0x140044d80, "unknown_140044d80", tpl_39::<0x143a17060, 0x143a17063, 0x143078760, 0x142409b50, 0x1425dadf0, 0x141834d18> as *const ()), // sub_140044d80
    Replacement::draft(0x140044de0, "unknown_140044de0", tpl_45::<0x143a16fc0, 0x143a16fc3, 0x143078768, 0x142409b50, 0x1425dae00, 0x141834d18> as *const ()), // sub_140044de0
    Replacement::draft(0x140044e40, "unknown_140044e40", tpl_41::<0x143a171e8, 0x143a171eb, 0x143078778, 0x142409b50, 0x1425dae10, 0x141834d18> as *const ()), // sub_140044e40
    Replacement::draft(0x140044ea0, "unknown_140044ea0", tpl_37::<0x143a16e88, 0x143a16e8b, 0x143078788, 0x142409b50, 0x1425dae20, 0x141834d18> as *const ()), // sub_140044ea0
    Replacement::draft(0x140044f00, "unknown_140044f00", tpl_46::<0x143a17170, 0x143a17173, 0x143078798, 0x142409b50, 0x1425dae30, 0x141834d18> as *const ()), // sub_140044f00
    Replacement::draft(0x140044f60, "unknown_140044f60", tpl_47::<0x143a16f20, 0x143a16f23, 0x1430787b0, 0x142409b50, 0x1425dae40, 0x141834d18> as *const ()), // sub_140044f60
    Replacement::draft(0x140044fc0, "unknown_140044fc0", tpl_38::<0x143a17198, 0x143a1719b, 0x1430787d0, 0x142409b50, 0x1425dae50, 0x141834d18> as *const ()), // sub_140044fc0
    Replacement::draft(0x140045020, "unknown_140045020", tpl_48::<0x143a172b0, 0x143a172b3, 0x1430787e0, 0x142409b50, 0x1425dae60, 0x141834d18> as *const ()), // sub_140045020
    Replacement::draft(0x140045080, "unknown_140045080", tpl_49::<0x143a17120, 0x143a17123, 0x1430787f8, 0x142409b50, 0x1425dae70, 0x141834d18> as *const ()), // sub_140045080
    Replacement::draft(0x1400450e0, "unknown_1400450e0", tpl_44::<0x143a17238, 0x143a1723b, 0x143078808, 0x142409b50, 0x1425dae80, 0x141834d18> as *const ()), // sub_1400450e0
    Replacement::draft(0x140045140, "unknown_140045140", tpl_42::<0x143a16e38, 0x143a16e3b, 0x143078820, 0x142409b50, 0x1425dae90, 0x141834d18> as *const ()), // sub_140045140
    Replacement::draft(0x1400451a0, "unknown_1400451a0", tpl_50::<0x143a171c0, 0x143a171c3, 0x143078838, 0x142409b50, 0x1425daea0, 0x141834d18> as *const ()), // sub_1400451a0
    Replacement::draft(0x140045200, "unknown_140045200", tpl_23::<0x143a173c8, 0x143a173cb, 0x1426cdcb0, 0x142409b50, 0x1425daeb0, 0x141834d18> as *const ()), // sub_140045200
    Replacement::draft(0x140045260, "unknown_140045260", tpl_23::<0x143a17410, 0x143a17413, 0x1426cdcb0, 0x142409b50, 0x1425daec0, 0x141834d18> as *const ()), // sub_140045260
    Replacement::draft(0x140045450, "unknown_140045450", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143845160, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425daf00, 0x141834d18> as *const ()), // sub_140045450
    Replacement::draft(0x140045850, "unknown_140045850", tpl_34::<0x143a174e8, 0x143a174eb, 0x143078668, 0x142409b50, 0x1425daf70, 0x141834d18> as *const ()), // sub_140045850
    Replacement::draft(0x1400458b0, "unknown_1400458b0", tpl_23::<0x143a177b0, 0x143a177b3, 0x1426cdcb0, 0x142409b50, 0x1425daf80, 0x141834d18> as *const ()), // sub_1400458b0
    Replacement::draft(0x140045910, "unknown_140045910", tpl_35::<0x143a17698, 0x143a1769b, 0x143078678, 0x142409b50, 0x1425daf90, 0x141834d18> as *const ()), // sub_140045910
    Replacement::draft(0x140045970, "unknown_140045970", tpl_36::<0x143a17760, 0x143a17763, 0x143078680, 0x142409b50, 0x1425dafa0, 0x141834d18> as *const ()), // sub_140045970
    Replacement::draft(0x1400459d0, "unknown_1400459d0", tpl_37::<0x143a175d8, 0x143a175db, 0x143078690, 0x142409b50, 0x1425dafb0, 0x141834d18> as *const ()), // sub_1400459d0
    Replacement::draft(0x140045a30, "unknown_140045a30", tpl_38::<0x143a17538, 0x143a1753b, 0x1430786a0, 0x142409b50, 0x1425dafc0, 0x141834d18> as *const ()), // sub_140045a30
    Replacement::draft(0x140045a90, "unknown_140045a90", tpl_39::<0x143a177d8, 0x143a177db, 0x1430786b0, 0x142409b50, 0x1425dafd0, 0x141834d18> as *const ()), // sub_140045a90
    Replacement::draft(0x140045af0, "unknown_140045af0", tpl_37::<0x143a17498, 0x143a1749b, 0x1430786b8, 0x142409b50, 0x1425dafe0, 0x141834d18> as *const ()), // sub_140045af0
    Replacement::draft(0x140045b50, "unknown_140045b50", tpl_40::<0x143a17878, 0x143a1787b, 0x1430786c4, 0x142409b50, 0x1425daff0, 0x141834d18> as *const ()), // sub_140045b50
    Replacement::draft(0x140045bb0, "unknown_140045bb0", tpl_41::<0x143a17588, 0x143a1758b, 0x1430786d0, 0x142409b50, 0x1425db000, 0x141834d18> as *const ()), // sub_140045bb0
    Replacement::draft(0x140045c10, "unknown_140045c10", tpl_42::<0x143a17828, 0x143a1782b, 0x1430786e0, 0x142409b50, 0x1425db010, 0x141834d18> as *const ()), // sub_140045c10
    Replacement::draft(0x140045c70, "unknown_140045c70", tpl_34::<0x143a17850, 0x143a17853, 0x1430786f8, 0x142409b50, 0x1425db020, 0x141834d18> as *const ()), // sub_140045c70
    Replacement::draft(0x140045cd0, "unknown_140045cd0", tpl_34::<0x143a174c0, 0x143a174c3, 0x143078708, 0x142409b50, 0x1425db030, 0x141834d18> as *const ()), // sub_140045cd0
    Replacement::draft(0x140045d30, "unknown_140045d30", tpl_41::<0x143a17560, 0x143a17563, 0x143078718, 0x142409b50, 0x1425db040, 0x141834d18> as *const ()), // sub_140045d30
    Replacement::draft(0x140045d90, "unknown_140045d90", tpl_43::<0x143a17600, 0x143a17603, 0x143078728, 0x142409b50, 0x1425db050, 0x141834d18> as *const ()), // sub_140045d90
    Replacement::draft(0x140045df0, "unknown_140045df0", tpl_44::<0x143a17378, 0x143a1737b, 0x143078748, 0x142409b50, 0x1425db060, 0x141834d18> as *const ()), // sub_140045df0
    Replacement::draft(0x140045e50, "unknown_140045e50", tpl_39::<0x143a175b0, 0x143a175b3, 0x143078760, 0x142409b50, 0x1425db070, 0x141834d18> as *const ()), // sub_140045e50
    Replacement::draft(0x140045eb0, "unknown_140045eb0", tpl_45::<0x143a17510, 0x143a17513, 0x143078768, 0x142409b50, 0x1425db080, 0x141834d18> as *const ()), // sub_140045eb0
    Replacement::draft(0x140045f10, "unknown_140045f10", tpl_41::<0x143a17738, 0x143a1773b, 0x143078778, 0x142409b50, 0x1425db090, 0x141834d18> as *const ()), // sub_140045f10
    Replacement::draft(0x140045f70, "unknown_140045f70", tpl_37::<0x143a173a0, 0x143a173a3, 0x143078788, 0x142409b50, 0x1425db0a0, 0x141834d18> as *const ()), // sub_140045f70
    Replacement::draft(0x140045fd0, "unknown_140045fd0", tpl_46::<0x143a176c0, 0x143a176c3, 0x143078798, 0x142409b50, 0x1425db0b0, 0x141834d18> as *const ()), // sub_140045fd0
    Replacement::draft(0x140046030, "unknown_140046030", tpl_47::<0x143a17438, 0x143a1743b, 0x1430787b0, 0x142409b50, 0x1425db0c0, 0x141834d18> as *const ()), // sub_140046030
    Replacement::draft(0x140046090, "unknown_140046090", tpl_38::<0x143a176e8, 0x143a176eb, 0x1430787d0, 0x142409b50, 0x1425db0d0, 0x141834d18> as *const ()), // sub_140046090
    Replacement::draft(0x1400460f0, "unknown_1400460f0", tpl_48::<0x143a17800, 0x143a17803, 0x1430787e0, 0x142409b50, 0x1425db0e0, 0x141834d18> as *const ()), // sub_1400460f0
    Replacement::draft(0x140046150, "unknown_140046150", tpl_49::<0x143a17670, 0x143a17673, 0x1430787f8, 0x142409b50, 0x1425db0f0, 0x141834d18> as *const ()), // sub_140046150
    Replacement::draft(0x1400461b0, "unknown_1400461b0", tpl_44::<0x143a17788, 0x143a1778b, 0x143078808, 0x142409b50, 0x1425db100, 0x141834d18> as *const ()), // sub_1400461b0
    Replacement::draft(0x140046210, "unknown_140046210", tpl_42::<0x143a17350, 0x143a17353, 0x143078820, 0x142409b50, 0x1425db110, 0x141834d18> as *const ()), // sub_140046210
    Replacement::draft(0x140046270, "unknown_140046270", tpl_50::<0x143a17710, 0x143a17713, 0x143078838, 0x142409b50, 0x1425db120, 0x141834d18> as *const ()), // sub_140046270
    Replacement::draft(0x1400462d0, "unknown_1400462d0", tpl_74::<0x1430a00b8, 0x142409b50, 0x143a17460, 0x143a17463, 0x143a17464, 0x143a17468, 0x1401df680, 0x14183acc8, 0x142408f50, 0x14183acbc, 0x143a17488, 0x143a17490, 0x140267b90, 0x14240ab10, 0x1425db130, 0x141834d18> as *const ()), // sub_1400462d0
    Replacement::draft(0x140046530, "unknown_140046530", tpl_23::<0x143a17918, 0x143a1791b, 0x1426cdcb0, 0x142409b50, 0x1425db170, 0x141834d18> as *const ()), // sub_140046530
    Replacement::draft(0x140046590, "unknown_140046590", tpl_23::<0x143a17960, 0x143a17963, 0x1426cdcb0, 0x142409b50, 0x1425db180, 0x141834d18> as *const ()), // sub_140046590
    Replacement::draft(0x140046780, "unknown_140046780", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438451f0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425db1c0, 0x141834d18> as *const ()), // sub_140046780
    Replacement::draft(0x140046b10, "unknown_140046b10", tpl_34::<0x143a17a00, 0x143a17a03, 0x143078668, 0x142409b50, 0x1425db200, 0x141834d18> as *const ()), // sub_140046b10
    Replacement::draft(0x140046b70, "unknown_140046b70", tpl_23::<0x143a17cc0, 0x143a17cc3, 0x1426cdcb0, 0x142409b50, 0x1425db210, 0x141834d18> as *const ()), // sub_140046b70
    Replacement::draft(0x140046bd0, "unknown_140046bd0", tpl_35::<0x143a17ba8, 0x143a17bab, 0x143078678, 0x142409b50, 0x1425db220, 0x141834d18> as *const ()), // sub_140046bd0
    Replacement::draft(0x140046c30, "unknown_140046c30", tpl_36::<0x143a17c70, 0x143a17c73, 0x143078680, 0x142409b50, 0x1425db230, 0x141834d18> as *const ()), // sub_140046c30
    Replacement::draft(0x140046c90, "unknown_140046c90", tpl_37::<0x143a17af0, 0x143a17af3, 0x143078690, 0x142409b50, 0x1425db240, 0x141834d18> as *const ()), // sub_140046c90
    Replacement::draft(0x140046cf0, "unknown_140046cf0", tpl_38::<0x143a17a50, 0x143a17a53, 0x1430786a0, 0x142409b50, 0x1425db250, 0x141834d18> as *const ()), // sub_140046cf0
    Replacement::draft(0x140046d50, "unknown_140046d50", tpl_39::<0x143a17ce8, 0x143a17ceb, 0x1430786b0, 0x142409b50, 0x1425db260, 0x141834d18> as *const ()), // sub_140046d50
    Replacement::draft(0x140046db0, "unknown_140046db0", tpl_37::<0x143a179b0, 0x143a179b3, 0x1430786b8, 0x142409b50, 0x1425db270, 0x141834d18> as *const ()), // sub_140046db0
    Replacement::draft(0x140046e10, "unknown_140046e10", tpl_40::<0x143a17d88, 0x143a17d8b, 0x1430786c4, 0x142409b50, 0x1425db280, 0x141834d18> as *const ()), // sub_140046e10
    Replacement::draft(0x140046e70, "unknown_140046e70", tpl_41::<0x143a17aa0, 0x143a17aa3, 0x1430786d0, 0x142409b50, 0x1425db290, 0x141834d18> as *const ()), // sub_140046e70
    Replacement::draft(0x140046ed0, "unknown_140046ed0", tpl_42::<0x143a17d38, 0x143a17d3b, 0x1430786e0, 0x142409b50, 0x1425db2a0, 0x141834d18> as *const ()), // sub_140046ed0
    Replacement::draft(0x140046f30, "unknown_140046f30", tpl_34::<0x143a17d60, 0x143a17d63, 0x1430786f8, 0x142409b50, 0x1425db2b0, 0x141834d18> as *const ()), // sub_140046f30
    Replacement::draft(0x140046f90, "unknown_140046f90", tpl_34::<0x143a179d8, 0x143a179db, 0x143078708, 0x142409b50, 0x1425db2c0, 0x141834d18> as *const ()), // sub_140046f90
    Replacement::draft(0x140046ff0, "unknown_140046ff0", tpl_41::<0x143a17a78, 0x143a17a7b, 0x143078718, 0x142409b50, 0x1425db2d0, 0x141834d18> as *const ()), // sub_140046ff0
    Replacement::draft(0x140047050, "unknown_140047050", tpl_43::<0x143a17b18, 0x143a17b1b, 0x143078728, 0x142409b50, 0x1425db2e0, 0x141834d18> as *const ()), // sub_140047050
    Replacement::draft(0x1400470b0, "unknown_1400470b0", tpl_44::<0x143a178c8, 0x143a178cb, 0x143078748, 0x142409b50, 0x1425db2f0, 0x141834d18> as *const ()), // sub_1400470b0
    Replacement::draft(0x140047110, "unknown_140047110", tpl_39::<0x143a17ac8, 0x143a17acb, 0x143078760, 0x142409b50, 0x1425db300, 0x141834d18> as *const ()), // sub_140047110
    Replacement::draft(0x140047170, "unknown_140047170", tpl_45::<0x143a17a28, 0x143a17a2b, 0x143078768, 0x142409b50, 0x1425db310, 0x141834d18> as *const ()), // sub_140047170
    Replacement::draft(0x1400471d0, "unknown_1400471d0", tpl_41::<0x143a17c48, 0x143a17c4b, 0x143078778, 0x142409b50, 0x1425db320, 0x141834d18> as *const ()), // sub_1400471d0
    Replacement::draft(0x140047230, "unknown_140047230", tpl_37::<0x143a178f0, 0x143a178f3, 0x143078788, 0x142409b50, 0x1425db330, 0x141834d18> as *const ()), // sub_140047230
    Replacement::draft(0x140047290, "unknown_140047290", tpl_46::<0x143a17bd0, 0x143a17bd3, 0x143078798, 0x142409b50, 0x1425db340, 0x141834d18> as *const ()), // sub_140047290
    Replacement::draft(0x1400472f0, "unknown_1400472f0", tpl_47::<0x143a17988, 0x143a1798b, 0x1430787b0, 0x142409b50, 0x1425db350, 0x141834d18> as *const ()), // sub_1400472f0
    Replacement::draft(0x140047350, "unknown_140047350", tpl_38::<0x143a17bf8, 0x143a17bfb, 0x1430787d0, 0x142409b50, 0x1425db360, 0x141834d18> as *const ()), // sub_140047350
    Replacement::draft(0x1400473b0, "unknown_1400473b0", tpl_48::<0x143a17d10, 0x143a17d13, 0x1430787e0, 0x142409b50, 0x1425db370, 0x141834d18> as *const ()), // sub_1400473b0
    Replacement::draft(0x140047410, "unknown_140047410", tpl_49::<0x143a17b80, 0x143a17b83, 0x1430787f8, 0x142409b50, 0x1425db380, 0x141834d18> as *const ()), // sub_140047410
    Replacement::draft(0x140047470, "unknown_140047470", tpl_44::<0x143a17c98, 0x143a17c9b, 0x143078808, 0x142409b50, 0x1425db390, 0x141834d18> as *const ()), // sub_140047470
    Replacement::draft(0x1400474d0, "unknown_1400474d0", tpl_42::<0x143a178a0, 0x143a178a3, 0x143078820, 0x142409b50, 0x1425db3a0, 0x141834d18> as *const ()), // sub_1400474d0
    Replacement::draft(0x140047530, "unknown_140047530", tpl_50::<0x143a17c20, 0x143a17c23, 0x143078838, 0x142409b50, 0x1425db3b0, 0x141834d18> as *const ()), // sub_140047530
    Replacement::draft(0x140047600, "unknown_140047600", tpl_23::<0x143a17db0, 0x143a17db3, 0x1426cdcb0, 0x142409b50, 0x1425db3f0, 0x141834d18> as *const ()), // sub_140047600
    Replacement::draft(0x140047660, "unknown_140047660", tpl_23::<0x143a17df8, 0x143a17dfb, 0x1426cdcb0, 0x142409b50, 0x1425db400, 0x141834d18> as *const ()), // sub_140047660
    Replacement::draft(0x140047850, "unknown_140047850", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438452b0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425db440, 0x141834d18> as *const ()), // sub_140047850
    Replacement::draft(0x140047fe0, "unknown_140047fe0", tpl_23::<0x143a17e20, 0x143a17e23, 0x1426cdcb0, 0x142409b50, 0x1425db490, 0x141834d18> as *const ()), // sub_140047fe0
    Replacement::draft(0x140048040, "unknown_140048040", tpl_23::<0x143a17e68, 0x143a17e6b, 0x1426cdcb0, 0x142409b50, 0x1425db4a0, 0x141834d18> as *const ()), // sub_140048040
    Replacement::draft(0x140048230, "unknown_140048230", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438453d0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425db4e0, 0x141834d18> as *const ()), // sub_140048230
    Replacement::draft(0x1400487a0, "unknown_1400487a0", tpl_23::<0x143a17e90, 0x143a17e93, 0x1426cdcb0, 0x142409b50, 0x1425db520, 0x141834d18> as *const ()), // sub_1400487a0
    Replacement::draft(0x140048800, "unknown_140048800", tpl_23::<0x143a17ed8, 0x143a17edb, 0x1426cdcb0, 0x142409b50, 0x1425db530, 0x141834d18> as *const ()), // sub_140048800
    Replacement::draft(0x1400489f0, "unknown_1400489f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143845490, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425db570, 0x141834d18> as *const ()), // sub_1400489f0
    Replacement::draft(0x140049b50, "unknown_140049b50", tpl_51::<0x143078928, 0x143078938, 0x143a17f40, 0x143078944, 0x1430782bc, 0x1430782b4, 0x1401df440, 0x1425db660, 0x141834d18> as *const ()), // sub_140049b50
    Replacement::draft(0x140049be0, "unknown_140049be0", tpl_23::<0x143a17f80, 0x143a17f83, 0x1426cdcb0, 0x142409b50, 0x1425db690, 0x141834d18> as *const ()), // sub_140049be0
    Replacement::draft(0x140049c40, "unknown_140049c40", tpl_23::<0x143a17fc8, 0x143a17fcb, 0x1426cdcb0, 0x142409b50, 0x1425db6a0, 0x141834d18> as *const ()), // sub_140049c40
    Replacement::draft(0x140049e30, "unknown_140049e30", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143845520, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425db6e0, 0x141834d18> as *const ()), // sub_140049e30
    Replacement::draft(0x14004a580, "unknown_14004a580", tpl_51::<0x143078928, 0x143078938, 0x143a17ff0, 0x143078944, 0x1430782bc, 0x1430782b4, 0x1401df440, 0x1425db720, 0x141834d18> as *const ()), // sub_14004a580
    Replacement::draft(0x14004a610, "unknown_14004a610", tpl_23::<0x143a18030, 0x143a18033, 0x1426cdcb0, 0x142409b50, 0x1425db750, 0x141834d18> as *const ()), // sub_14004a610
    Replacement::draft(0x14004a670, "unknown_14004a670", tpl_23::<0x143a18078, 0x143a1807b, 0x1426cdcb0, 0x142409b50, 0x1425db760, 0x141834d18> as *const ()), // sub_14004a670
    Replacement::draft(0x14004a860, "unknown_14004a860", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438455b0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425db7a0, 0x141834d18> as *const ()), // sub_14004a860
    Replacement::draft(0x14004b990, "unknown_14004b990", tpl_23::<0x143a180e0, 0x143a180e3, 0x1426cdcb0, 0x142409b50, 0x1425dc2b0, 0x141834d18> as *const ()), // sub_14004b990
    Replacement::draft(0x14004b9f0, "unknown_14004b9f0", tpl_23::<0x143a18128, 0x143a1812b, 0x1426cdcb0, 0x142409b50, 0x1425dc2c0, 0x141834d18> as *const ()), // sub_14004b9f0
    Replacement::draft(0x14004bbe0, "unknown_14004bbe0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143845890, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dc300, 0x141834d18> as *const ()), // sub_14004bbe0
    Replacement::draft(0x14004c260, "unknown_14004c260", fn_14004c260 as *const ()), // sub_14004c260
    Replacement::draft_throws(0x14004c510, "unknown_14004c510", fn_14004c510 as *const (), Throws::Caught), // sub_14004c510
    Replacement::draft_throws(0x14004c670, "unknown_14004c670", fn_14004c670 as *const (), Throws::Caught), // sub_14004c670
    Replacement::draft(0x14004cf50, "unknown_14004cf50", tpl_23::<0x143a18190, 0x143a18193, 0x1426cdcb0, 0x142409b50, 0x1425dcea0, 0x141834d18> as *const ()), // sub_14004cf50
    Replacement::draft(0x14004cfb0, "unknown_14004cfb0", tpl_23::<0x143a181d8, 0x143a181db, 0x1426cdcb0, 0x142409b50, 0x1425dceb0, 0x141834d18> as *const ()), // sub_14004cfb0
    Replacement::draft(0x14004d1a0, "unknown_14004d1a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143845d18, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dcef0, 0x141834d18> as *const ()), // sub_14004d1a0
    Replacement::draft(0x14004d530, "unknown_14004d530", tpl_61::<0x143a18310, 0x143a18313, 0x143091fd8, 0x142409b50, 0x1425dcf30, 0x141834d18> as *const ()), // sub_14004d530
    Replacement::draft(0x14004d590, "unknown_14004d590", tpl_62::<0x143a18313, 0x143a18310, 0x143a18318, 0x143a18314, 0x143a18200> as *const ()), // sub_14004d590
    Replacement::draft(0x14004d620, "unknown_14004d620", tpl_63::<0x143a182c0, 0x143a182c3, 0x143091ff0, 0x142409b50, 0x1425dcf40, 0x141834d18> as *const ()), // sub_14004d620
    Replacement::draft(0x14004d680, "unknown_14004d680", tpl_64::<0x143a18270, 0x143a18273, 0x143091fa0, 0x142409b50, 0x1425dcf50, 0x141834d18> as *const ()), // sub_14004d680
    Replacement::draft(0x14004d6e0, "unknown_14004d6e0", tpl_65::<0x143a18210, 0x143a18213, 0x143091fc0, 0x142409b50, 0x1425dcf60, 0x141834d18> as *const ()), // sub_14004d6e0
    Replacement::draft(0x14004d740, "unknown_14004d740", tpl_62::<0x143a182c3, 0x143a182c0, 0x143a182c8, 0x143a182c4, 0x143a182b8> as *const ()), // sub_14004d740
    Replacement::draft(0x14004d7d0, "unknown_14004d7d0", tpl_62::<0x143a18273, 0x143a18270, 0x143a18278, 0x143a18274, 0x143a18258> as *const ()), // sub_14004d7d0
    Replacement::draft(0x14004d860, "unknown_14004d860", tpl_62::<0x143a18213, 0x143a18210, 0x143a18218, 0x143a18214, 0x143a18260> as *const ()), // sub_14004d860
    Replacement::draft(0x14004dcf0, "unknown_14004dcf0", tpl_61::<0x143a18460, 0x143a18463, 0x143091fd8, 0x142409b50, 0x1425dd020, 0x141834d18> as *const ()), // sub_14004dcf0
    Replacement::draft(0x14004dd50, "unknown_14004dd50", tpl_62::<0x143a18463, 0x143a18460, 0x143a18468, 0x143a18464, 0x143a18358> as *const ()), // sub_14004dd50
    Replacement::draft(0x14004dde0, "unknown_14004dde0", tpl_63::<0x143a18410, 0x143a18413, 0x143091ff0, 0x142409b50, 0x1425dd030, 0x141834d18> as *const ()), // sub_14004dde0
    Replacement::draft(0x14004de40, "unknown_14004de40", tpl_64::<0x143a183c0, 0x143a183c3, 0x143091fa0, 0x142409b50, 0x1425dd040, 0x141834d18> as *const ()), // sub_14004de40
    Replacement::draft(0x14004dea0, "unknown_14004dea0", tpl_65::<0x143a18360, 0x143a18363, 0x143091fc0, 0x142409b50, 0x1425dd050, 0x141834d18> as *const ()), // sub_14004dea0
    Replacement::draft(0x14004df00, "unknown_14004df00", tpl_62::<0x143a18413, 0x143a18410, 0x143a18418, 0x143a18414, 0x143a18408> as *const ()), // sub_14004df00
    Replacement::draft(0x14004df90, "unknown_14004df90", tpl_62::<0x143a183c3, 0x143a183c0, 0x143a183c8, 0x143a183c4, 0x143a183a8> as *const ()), // sub_14004df90
    Replacement::draft(0x14004e020, "unknown_14004e020", tpl_62::<0x143a18363, 0x143a18360, 0x143a18368, 0x143a18364, 0x143a183b0> as *const ()), // sub_14004e020
    Replacement::draft(0x14004ed70, "unknown_14004ed70", tpl_23::<0x143a18500, 0x143a18503, 0x1426cdcb0, 0x142409b50, 0x1425dd160, 0x141834d18> as *const ()), // sub_14004ed70
    Replacement::draft(0x14004edd0, "unknown_14004edd0", tpl_23::<0x143a18548, 0x143a1854b, 0x1426cdcb0, 0x142409b50, 0x1425dd170, 0x141834d18> as *const ()), // sub_14004edd0
    Replacement::draft(0x14004efc0, "unknown_14004efc0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143845dd8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dd1b0, 0x141834d18> as *const ()), // sub_14004efc0
    Replacement::draft(0x14004f3f0, "unknown_14004f3f0", tpl_23::<0x143a18638, 0x143a1863b, 0x1426cdcb0, 0x142409b50, 0x1425dd220, 0x141834d18> as *const ()), // sub_14004f3f0
    Replacement::draft(0x14004f450, "unknown_14004f450", tpl_23::<0x143a18680, 0x143a18683, 0x1426cdcb0, 0x142409b50, 0x1425dd230, 0x141834d18> as *const ()), // sub_14004f450
    Replacement::draft(0x14004f640, "unknown_14004f640", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143845e88, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dd270, 0x141834d18> as *const ()), // sub_14004f640
    Replacement::draft(0x140050420, "unknown_140050420", tpl_23::<0x143a186f0, 0x143a186f3, 0x1426cdcb0, 0x142409b50, 0x1425dd460, 0x141834d18> as *const ()), // sub_140050420
    Replacement::draft(0x140050480, "unknown_140050480", tpl_23::<0x143a18738, 0x143a1873b, 0x1426cdcb0, 0x142409b50, 0x1425dd470, 0x141834d18> as *const ()), // sub_140050480
    Replacement::draft(0x140050e10, "unknown_140050e10", tpl_23::<0x143a18760, 0x143a18763, 0x1426cdcb0, 0x142409b50, 0x1425dd850, 0x141834d18> as *const ()), // sub_140050e10
    Replacement::draft(0x140050e70, "unknown_140050e70", tpl_23::<0x143a187a8, 0x143a187ab, 0x1426cdcb0, 0x142409b50, 0x1425dd860, 0x141834d18> as *const ()), // sub_140050e70
    Replacement::draft(0x140051060, "unknown_140051060", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846040, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dd8a0, 0x141834d18> as *const ()), // sub_140051060
    Replacement::draft(0x140051560, "unknown_140051560", tpl_23::<0x143a18928, 0x143a1892b, 0x1426cdcb0, 0x142409b50, 0x1425dd910, 0x141834d18> as *const ()), // sub_140051560
    Replacement::draft(0x1400515c0, "unknown_1400515c0", tpl_23::<0x143a18970, 0x143a18973, 0x1426cdcb0, 0x142409b50, 0x1425dd920, 0x141834d18> as *const ()), // sub_1400515c0
    Replacement::draft(0x1400517b0, "unknown_1400517b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438460d0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dd960, 0x141834d18> as *const ()), // sub_1400517b0
    Replacement::draft(0x140051be0, "unknown_140051be0", fn_140051be0 as *const ()), // sub_140051be0
    Replacement::draft(0x140051c70, "unknown_140051c70", fn_140051c70 as *const ()), // sub_140051c70
    Replacement::draft(0x140051d00, "unknown_140051d00", fn_140051d00 as *const ()), // sub_140051d00
    Replacement::draft(0x140051e30, "unknown_140051e30", tpl_61::<0x143a18ac0, 0x143a18ac3, 0x143091fd8, 0x142409b50, 0x1425dd9e0, 0x141834d18> as *const ()), // sub_140051e30
    Replacement::draft(0x140051e90, "unknown_140051e90", tpl_62::<0x143a18ac3, 0x143a18ac0, 0x143a18ac8, 0x143a18ac4, 0x143a18a30> as *const ()), // sub_140051e90
    Replacement::draft(0x140051f50, "unknown_140051f50", tpl_23::<0x143a18cc0, 0x143a18cc3, 0x1426cdcb0, 0x142409b50, 0x1425dd9f0, 0x141834d18> as *const ()), // sub_140051f50
    Replacement::draft(0x140051fb0, "unknown_140051fb0", tpl_23::<0x143a18d08, 0x143a18d0b, 0x1426cdcb0, 0x142409b50, 0x1425dda00, 0x141834d18> as *const ()), // sub_140051fb0
    Replacement::draft(0x1400521a0, "unknown_1400521a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846160, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dda40, 0x141834d18> as *const ()), // sub_1400521a0
    Replacement::draft(0x140052530, "unknown_140052530", tpl_34::<0x143a18e70, 0x143a18e73, 0x143078668, 0x142409b50, 0x1425dda80, 0x141834d18> as *const ()), // sub_140052530
    Replacement::draft(0x140052590, "unknown_140052590", tpl_23::<0x143a19438, 0x143a1943b, 0x1426cdcb0, 0x142409b50, 0x1425dda90, 0x141834d18> as *const ()), // sub_140052590
    Replacement::draft(0x1400525f0, "unknown_1400525f0", tpl_35::<0x143a19168, 0x143a1916b, 0x143078678, 0x142409b50, 0x1425ddaa0, 0x141834d18> as *const ()), // sub_1400525f0
    Replacement::draft(0x140052650, "unknown_140052650", tpl_36::<0x143a193c0, 0x143a193c3, 0x143078680, 0x142409b50, 0x1425ddab0, 0x141834d18> as *const ()), // sub_140052650
    Replacement::draft(0x1400526b0, "unknown_1400526b0", tpl_37::<0x143a19050, 0x143a19053, 0x143078690, 0x142409b50, 0x1425ddac0, 0x141834d18> as *const ()), // sub_1400526b0
    Replacement::draft(0x140052710, "unknown_140052710", tpl_38::<0x143a18f10, 0x143a18f13, 0x1430786a0, 0x142409b50, 0x1425ddad0, 0x141834d18> as *const ()), // sub_140052710
    Replacement::draft(0x140052770, "unknown_140052770", tpl_39::<0x143a19460, 0x143a19463, 0x1430786b0, 0x142409b50, 0x1425ddae0, 0x141834d18> as *const ()), // sub_140052770
    Replacement::draft(0x1400527d0, "unknown_1400527d0", tpl_37::<0x143a18e20, 0x143a18e23, 0x1430786b8, 0x142409b50, 0x1425ddaf0, 0x141834d18> as *const ()), // sub_1400527d0
    Replacement::draft(0x140052830, "unknown_140052830", tpl_40::<0x143a19578, 0x143a1957b, 0x1430786c4, 0x142409b50, 0x1425ddb00, 0x141834d18> as *const ()), // sub_140052830
    Replacement::draft(0x140052890, "unknown_140052890", tpl_41::<0x143a18fd8, 0x143a18fdb, 0x1430786d0, 0x142409b50, 0x1425ddb10, 0x141834d18> as *const ()), // sub_140052890
    Replacement::draft(0x1400528f0, "unknown_1400528f0", tpl_42::<0x143a194b0, 0x143a194b3, 0x1430786e0, 0x142409b50, 0x1425ddb20, 0x141834d18> as *const ()), // sub_1400528f0
    Replacement::draft(0x140052950, "unknown_140052950", tpl_34::<0x143a19500, 0x143a19503, 0x1430786f8, 0x142409b50, 0x1425ddb30, 0x141834d18> as *const ()), // sub_140052950
    Replacement::draft(0x1400529b0, "unknown_1400529b0", tpl_34::<0x143a18e48, 0x143a18e4b, 0x143078708, 0x142409b50, 0x1425ddb40, 0x141834d18> as *const ()), // sub_1400529b0
    Replacement::draft(0x140052a10, "unknown_140052a10", tpl_41::<0x143a18f38, 0x143a18f3b, 0x143078718, 0x142409b50, 0x1425ddb50, 0x141834d18> as *const ()), // sub_140052a10
    Replacement::draft(0x140052a70, "unknown_140052a70", tpl_43::<0x143a19078, 0x143a1907b, 0x143078728, 0x142409b50, 0x1425ddb60, 0x141834d18> as *const ()), // sub_140052a70
    Replacement::draft(0x140052ad0, "unknown_140052ad0", tpl_44::<0x143a18b30, 0x143a18b33, 0x143078748, 0x142409b50, 0x1425ddb70, 0x141834d18> as *const ()), // sub_140052ad0
    Replacement::draft(0x140052b30, "unknown_140052b30", tpl_39::<0x143a19000, 0x143a19003, 0x143078760, 0x142409b50, 0x1425ddb80, 0x141834d18> as *const ()), // sub_140052b30
    Replacement::draft(0x140052b90, "unknown_140052b90", tpl_45::<0x143a18ee8, 0x143a18eeb, 0x143078768, 0x142409b50, 0x1425ddb90, 0x141834d18> as *const ()), // sub_140052b90
    Replacement::draft(0x140052bf0, "unknown_140052bf0", tpl_41::<0x143a19370, 0x143a19373, 0x143078778, 0x142409b50, 0x1425ddba0, 0x141834d18> as *const ()), // sub_140052bf0
    Replacement::draft(0x140052c50, "unknown_140052c50", tpl_37::<0x143a18b58, 0x143a18b5b, 0x143078788, 0x142409b50, 0x1425ddbb0, 0x141834d18> as *const ()), // sub_140052c50
    Replacement::draft(0x140052cb0, "unknown_140052cb0", tpl_46::<0x143a19208, 0x143a1920b, 0x143078798, 0x142409b50, 0x1425ddbc0, 0x141834d18> as *const ()), // sub_140052cb0
    Replacement::draft(0x140052d10, "unknown_140052d10", tpl_47::<0x143a18d80, 0x143a18d83, 0x1430787b0, 0x142409b50, 0x1425ddbd0, 0x141834d18> as *const ()), // sub_140052d10
    Replacement::draft(0x140052d70, "unknown_140052d70", tpl_38::<0x143a192a8, 0x143a192ab, 0x1430787d0, 0x142409b50, 0x1425ddbe0, 0x141834d18> as *const ()), // sub_140052d70
    Replacement::draft(0x140052dd0, "unknown_140052dd0", tpl_48::<0x143a19488, 0x143a1948b, 0x1430787e0, 0x142409b50, 0x1425ddbf0, 0x141834d18> as *const ()), // sub_140052dd0
    Replacement::draft(0x140052e30, "unknown_140052e30", tpl_49::<0x143a19140, 0x143a19143, 0x1430787f8, 0x142409b50, 0x1425ddc00, 0x141834d18> as *const ()), // sub_140052e30
    Replacement::draft(0x140052e90, "unknown_140052e90", tpl_44::<0x143a19410, 0x143a19413, 0x143078808, 0x142409b50, 0x1425ddc10, 0x141834d18> as *const ()), // sub_140052e90
    Replacement::draft(0x140052ef0, "unknown_140052ef0", tpl_42::<0x143a18b08, 0x143a18b0b, 0x143078820, 0x142409b50, 0x1425ddc20, 0x141834d18> as *const ()), // sub_140052ef0
    Replacement::draft(0x140052f50, "unknown_140052f50", tpl_50::<0x143a19320, 0x143a19323, 0x143078838, 0x142409b50, 0x1425ddc30, 0x141834d18> as *const ()), // sub_140052f50
    Replacement::draft(0x140052fb0, "unknown_140052fb0", tpl_52::<0x143a18c98, 0x143a18c9b, 0x143a19578, 0x143a1957c, 0x143a1957b, 0x143a19580, 0x142409b50, 0x1425ddc40, 0x141834d18> as *const ()), // sub_140052fb0
    Replacement::draft(0x140053070, "unknown_140053070", tpl_38::<0x143a19190, 0x143a19193, 0x143078950, 0x142409b50, 0x1425ddc50, 0x141834d18> as *const ()), // sub_140053070
    Replacement::draft(0x1400530d0, "unknown_1400530d0", tpl_53::<0x143a18d58, 0x143a18d5b, 0x143078960, 0x142409b50, 0x1425ddc60, 0x141834d18> as *const ()), // sub_1400530d0
    Replacement::draft(0x140053130, "unknown_140053130", tpl_45::<0x143a18c20, 0x143a18c23, 0x143078980, 0x142409b50, 0x1425ddc70, 0x141834d18> as *const ()), // sub_140053130
    Replacement::draft(0x140053190, "unknown_140053190", tpl_35::<0x143a18c70, 0x143a18c73, 0x14307898c, 0x142409b50, 0x1425ddc80, 0x141834d18> as *const ()), // sub_140053190
    Replacement::draft(0x1400531f0, "unknown_1400531f0", tpl_42::<0x143a194d8, 0x143a194db, 0x1430786e0, 0x142409b50, 0x1425ddc90, 0x141834d18> as *const ()), // sub_1400531f0
    Replacement::draft(0x140053250, "unknown_140053250", tpl_38::<0x143a193e8, 0x143a193eb, 0x143078998, 0x142409b50, 0x1425ddca0, 0x141834d18> as *const ()), // sub_140053250
    Replacement::draft(0x1400532b0, "unknown_1400532b0", tpl_38::<0x143a18f88, 0x143a18f8b, 0x1430789a8, 0x142409b50, 0x1425ddcb0, 0x141834d18> as *const ()), // sub_1400532b0
    Replacement::draft(0x140053310, "unknown_140053310", tpl_41::<0x143a190a0, 0x143a190a3, 0x1430789b8, 0x142409b50, 0x1425ddcc0, 0x141834d18> as *const ()), // sub_140053310
    Replacement::draft(0x140053370, "unknown_140053370", tpl_34::<0x143a19528, 0x143a1952b, 0x1430789c8, 0x142409b50, 0x1425ddcd0, 0x141834d18> as *const ()), // sub_140053370
    Replacement::draft(0x1400533d0, "unknown_1400533d0", tpl_39::<0x143a18c48, 0x143a18c4b, 0x1430789d8, 0x142409b50, 0x1425ddce0, 0x141834d18> as *const ()), // sub_1400533d0
    Replacement::draft(0x140053430, "unknown_140053430", tpl_41::<0x143a18f60, 0x143a18f63, 0x1430789e0, 0x142409b50, 0x1425ddcf0, 0x141834d18> as *const ()), // sub_140053430
    Replacement::draft(0x140053490, "unknown_140053490", tpl_34::<0x143a191b8, 0x143a191bb, 0x1430789f0, 0x142409b50, 0x1425ddd00, 0x141834d18> as *const ()), // sub_140053490
    Replacement::draft(0x1400534f0, "unknown_1400534f0", tpl_48::<0x143a18e98, 0x143a18e9b, 0x143078a00, 0x142409b50, 0x1425ddd10, 0x141834d18> as *const ()), // sub_1400534f0
    Replacement::draft(0x140053550, "unknown_140053550", tpl_46::<0x143a192d0, 0x143a192d3, 0x143078a18, 0x142409b50, 0x1425ddd20, 0x141834d18> as *const ()), // sub_140053550
    Replacement::draft(0x1400535b0, "unknown_1400535b0", tpl_47::<0x143a19230, 0x143a19233, 0x143078a30, 0x142409b50, 0x1425ddd30, 0x141834d18> as *const ()), // sub_1400535b0
    Replacement::draft(0x140053610, "unknown_140053610", tpl_37::<0x143a18dd0, 0x143a18dd3, 0x143078a50, 0x142409b50, 0x1425ddd40, 0x141834d18> as *const ()), // sub_140053610
    Replacement::draft(0x140053670, "unknown_140053670", tpl_45::<0x143a19028, 0x143a1902b, 0x143078a60, 0x142409b50, 0x1425ddd50, 0x141834d18> as *const ()), // sub_140053670
    Replacement::draft(0x1400536d0, "unknown_1400536d0", tpl_54::<0x143a190f0, 0x143a190f3, 0x143078a70, 0x142409b50, 0x1425ddd60, 0x141834d18> as *const ()), // sub_1400536d0
    Replacement::draft(0x140053730, "unknown_140053730", tpl_40::<0x143a18da8, 0x143a18dab, 0x143078a88, 0x142409b50, 0x1425ddd70, 0x141834d18> as *const ()), // sub_140053730
    Replacement::draft(0x140053790, "unknown_140053790", tpl_50::<0x143a18df8, 0x143a18dfb, 0x143078a90, 0x142409b50, 0x1425ddd80, 0x141834d18> as *const ()), // sub_140053790
    Replacement::draft(0x1400537f0, "unknown_1400537f0", tpl_41::<0x143a18fb0, 0x143a18fb3, 0x143078aa8, 0x142409b50, 0x1425ddd90, 0x141834d18> as *const ()), // sub_1400537f0
    Replacement::draft(0x140053850, "unknown_140053850", tpl_55::<0x143a19258, 0x143a1925b, 0x143078ab8, 0x142409b50, 0x1425ddda0, 0x141834d18> as *const ()), // sub_140053850
    Replacement::draft(0x1400538b0, "unknown_1400538b0", tpl_41::<0x143a192f8, 0x143a192fb, 0x143078ac8, 0x142409b50, 0x1425dddb0, 0x141834d18> as *const ()), // sub_1400538b0
    Replacement::draft(0x140053910, "unknown_140053910", tpl_49::<0x143a19398, 0x143a1939b, 0x143078ad8, 0x142409b50, 0x1425dddc0, 0x141834d18> as *const ()), // sub_140053910
    Replacement::draft(0x140053970, "unknown_140053970", tpl_40::<0x143a18bf8, 0x143a18bfb, 0x143078ae4, 0x142409b50, 0x1425dddd0, 0x141834d18> as *const ()), // sub_140053970
    Replacement::draft(0x1400539d0, "unknown_1400539d0", tpl_44::<0x143a18d30, 0x143a18d33, 0x143078af0, 0x142409b50, 0x1425ddde0, 0x141834d18> as *const ()), // sub_1400539d0
    Replacement::draft(0x140053a30, "unknown_140053a30", tpl_46::<0x143a18bd0, 0x143a18bd3, 0x143078b08, 0x142409b50, 0x1425dddf0, 0x141834d18> as *const ()), // sub_140053a30
    Replacement::draft(0x140053a90, "unknown_140053a90", tpl_55::<0x143a19280, 0x143a19283, 0x143078b20, 0x142409b50, 0x1425dde00, 0x141834d18> as *const ()), // sub_140053a90
    Replacement::draft(0x140053af0, "unknown_140053af0", tpl_46::<0x143a19118, 0x143a1911b, 0x143078b30, 0x142409b50, 0x1425dde10, 0x141834d18> as *const ()), // sub_140053af0
    Replacement::draft(0x140053b50, "unknown_140053b50", tpl_50::<0x143a18ec0, 0x143a18ec3, 0x143078b48, 0x142409b50, 0x1425dde20, 0x141834d18> as *const ()), // sub_140053b50
    Replacement::draft(0x140053bb0, "unknown_140053bb0", tpl_42::<0x143a18b80, 0x143a18b83, 0x143078b60, 0x142409b50, 0x1425dde30, 0x141834d18> as *const ()), // sub_140053bb0
    Replacement::draft(0x140053c10, "unknown_140053c10", tpl_38::<0x143a191e0, 0x143a191e3, 0x143078b78, 0x142409b50, 0x1425dde40, 0x141834d18> as *const ()), // sub_140053c10
    Replacement::draft(0x140053c70, "unknown_140053c70", tpl_34::<0x143a19550, 0x143a19553, 0x143078b88, 0x142409b50, 0x1425dde50, 0x141834d18> as *const ()), // sub_140053c70
    Replacement::draft(0x140053cd0, "unknown_140053cd0", tpl_41::<0x143a19348, 0x143a1934b, 0x143078b98, 0x142409b50, 0x1425dde60, 0x141834d18> as *const ()), // sub_140053cd0
    Replacement::draft(0x140053d30, "unknown_140053d30", tpl_36::<0x143a190c8, 0x143a190cb, 0x143078ba8, 0x142409b50, 0x1425dde70, 0x141834d18> as *const ()), // sub_140053d30
    Replacement::draft(0x140053d90, "unknown_140053d90", tpl_34::<0x143a18ba8, 0x143a18bab, 0x143078bb8, 0x142409b50, 0x1425dde80, 0x141834d18> as *const ()), // sub_140053d90
    Replacement::draft(0x1400540c0, "unknown_1400540c0", tpl_23::<0x143a195a0, 0x143a195a3, 0x1426cdcb0, 0x142409b50, 0x1425ddea0, 0x141834d18> as *const ()), // sub_1400540c0
    Replacement::draft(0x140054120, "unknown_140054120", tpl_23::<0x143a195e8, 0x143a195eb, 0x1426cdcb0, 0x142409b50, 0x1425ddeb0, 0x141834d18> as *const ()), // sub_140054120
    Replacement::draft(0x1400541f0, "unknown_1400541f0", tpl_61::<0x143a197b0, 0x143a197b3, 0x143091fd8, 0x142409b50, 0x1425dded0, 0x141834d18> as *const ()), // sub_1400541f0
    Replacement::draft(0x140054250, "unknown_140054250", tpl_62::<0x143a197b3, 0x143a197b0, 0x143a197b8, 0x143a197b4, 0x143a19610> as *const ()), // sub_140054250
    Replacement::draft(0x1400542e0, "unknown_1400542e0", tpl_63::<0x143a19760, 0x143a19763, 0x143091ff0, 0x142409b50, 0x1425ddee0, 0x141834d18> as *const ()), // sub_1400542e0
    Replacement::draft(0x140054340, "unknown_140054340", tpl_64::<0x143a19680, 0x143a19683, 0x143091fa0, 0x142409b50, 0x1425ddef0, 0x141834d18> as *const ()), // sub_140054340
    Replacement::draft(0x1400543a0, "unknown_1400543a0", tpl_65::<0x143a19620, 0x143a19623, 0x143091fc0, 0x142409b50, 0x1425ddf00, 0x141834d18> as *const ()), // sub_1400543a0
    Replacement::draft(0x140054400, "unknown_140054400", tpl_62::<0x143a19763, 0x143a19760, 0x143a19768, 0x143a19764, 0x143a19758> as *const ()), // sub_140054400
    Replacement::draft(0x140054490, "unknown_140054490", tpl_62::<0x143a19683, 0x143a19680, 0x143a19688, 0x143a19684, 0x143a19668> as *const ()), // sub_140054490
    Replacement::draft(0x140054520, "unknown_140054520", tpl_62::<0x143a19623, 0x143a19620, 0x143a19628, 0x143a19624, 0x143a19670> as *const ()), // sub_140054520
    Replacement::draft(0x1400546d0, "unknown_1400546d0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846220, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ddf30, 0x141834d18> as *const ()), // sub_1400546d0
    Replacement::draft(0x140055160, "unknown_140055160", fn_140055160 as *const ()), // sub_140055160
    Replacement::draft(0x140057fb0, "unknown_140057fb0", tpl_23::<0x143a19cc8, 0x143a19ccb, 0x1426cdcb0, 0x142409b50, 0x1425de0f0, 0x141834d18> as *const ()), // sub_140057fb0
    Replacement::draft(0x140058010, "unknown_140058010", tpl_23::<0x143a19d10, 0x143a19d13, 0x1426cdcb0, 0x142409b50, 0x1425de100, 0x141834d18> as *const ()), // sub_140058010
    Replacement::draft(0x140058200, "unknown_140058200", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438462b0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425de140, 0x141834d18> as *const ()), // sub_140058200
    Replacement::draft(0x140058600, "unknown_140058600", tpl_23::<0x143a19d80, 0x143a19d83, 0x1426cdcb0, 0x142409b50, 0x1425de1b0, 0x141834d18> as *const ()), // sub_140058600
    Replacement::draft(0x140058660, "unknown_140058660", tpl_23::<0x143a19dc8, 0x143a19dcb, 0x1426cdcb0, 0x142409b50, 0x1425de1c0, 0x141834d18> as *const ()), // sub_140058660
    Replacement::draft(0x140058d70, "unknown_140058d70", tpl_23::<0x143a19df0, 0x143a19df3, 0x1426cdcb0, 0x142409b50, 0x1425de790, 0x141834d18> as *const ()), // sub_140058d70
    Replacement::draft(0x140058dd0, "unknown_140058dd0", tpl_23::<0x143a19e38, 0x143a19e3b, 0x1426cdcb0, 0x142409b50, 0x1425de7a0, 0x141834d18> as *const ()), // sub_140058dd0
    Replacement::draft(0x140058fc0, "unknown_140058fc0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846460, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425de7e0, 0x141834d18> as *const ()), // sub_140058fc0
    Replacement::draft(0x140059a90, "unknown_140059a90", tpl_23::<0x143a1a058, 0x143a1a05b, 0x1426cdcb0, 0x142409b50, 0x1425de900, 0x141834d18> as *const ()), // sub_140059a90
    Replacement::draft(0x140059af0, "unknown_140059af0", tpl_23::<0x143a1a0a0, 0x143a1a0a3, 0x1426cdcb0, 0x142409b50, 0x1425de910, 0x141834d18> as *const ()), // sub_140059af0
    Replacement::draft(0x140059ce0, "unknown_140059ce0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846530, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425de950, 0x141834d18> as *const ()), // sub_140059ce0
    Replacement::draft(0x14005a0e0, "unknown_14005a0e0", tpl_34::<0x143a1a208, 0x143a1a20b, 0x143078668, 0x142409b50, 0x1425de9c0, 0x141834d18> as *const ()), // sub_14005a0e0
    Replacement::draft(0x14005a140, "unknown_14005a140", tpl_23::<0x143a1a818, 0x143a1a81b, 0x1426cdcb0, 0x142409b50, 0x1425de9d0, 0x141834d18> as *const ()), // sub_14005a140
    Replacement::draft(0x14005a1a0, "unknown_14005a1a0", tpl_35::<0x143a1a548, 0x143a1a54b, 0x143078678, 0x142409b50, 0x1425de9e0, 0x141834d18> as *const ()), // sub_14005a1a0
    Replacement::draft(0x14005a200, "unknown_14005a200", tpl_36::<0x143a1a7a0, 0x143a1a7a3, 0x143078680, 0x142409b50, 0x1425de9f0, 0x141834d18> as *const ()), // sub_14005a200
    Replacement::draft(0x14005a260, "unknown_14005a260", tpl_37::<0x143a1a3e8, 0x143a1a3eb, 0x143078690, 0x142409b50, 0x1425dea00, 0x141834d18> as *const ()), // sub_14005a260
    Replacement::draft(0x14005a2c0, "unknown_14005a2c0", tpl_38::<0x143a1a2a8, 0x143a1a2ab, 0x1430786a0, 0x142409b50, 0x1425dea10, 0x141834d18> as *const ()), // sub_14005a2c0
    Replacement::draft(0x14005a320, "unknown_14005a320", tpl_39::<0x143a1a840, 0x143a1a843, 0x1430786b0, 0x142409b50, 0x1425dea20, 0x141834d18> as *const ()), // sub_14005a320
    Replacement::draft(0x14005a380, "unknown_14005a380", tpl_37::<0x143a1a1b8, 0x143a1a1bb, 0x1430786b8, 0x142409b50, 0x1425dea30, 0x141834d18> as *const ()), // sub_14005a380
    Replacement::draft(0x14005a3e0, "unknown_14005a3e0", tpl_40::<0x143a1a958, 0x143a1a95b, 0x1430786c4, 0x142409b50, 0x1425dea40, 0x141834d18> as *const ()), // sub_14005a3e0
    Replacement::draft(0x14005a440, "unknown_14005a440", tpl_41::<0x143a1a370, 0x143a1a373, 0x1430786d0, 0x142409b50, 0x1425dea50, 0x141834d18> as *const ()), // sub_14005a440
    Replacement::draft(0x14005a4a0, "unknown_14005a4a0", tpl_42::<0x143a1a890, 0x143a1a893, 0x1430786e0, 0x142409b50, 0x1425dea60, 0x141834d18> as *const ()), // sub_14005a4a0
    Replacement::draft(0x14005a500, "unknown_14005a500", tpl_34::<0x143a1a8e0, 0x143a1a8e3, 0x1430786f8, 0x142409b50, 0x1425dea70, 0x141834d18> as *const ()), // sub_14005a500
    Replacement::draft(0x14005a560, "unknown_14005a560", tpl_34::<0x143a1a1e0, 0x143a1a1e3, 0x143078708, 0x142409b50, 0x1425dea80, 0x141834d18> as *const ()), // sub_14005a560
    Replacement::draft(0x14005a5c0, "unknown_14005a5c0", tpl_41::<0x143a1a2d0, 0x143a1a2d3, 0x143078718, 0x142409b50, 0x1425dea90, 0x141834d18> as *const ()), // sub_14005a5c0
    Replacement::draft(0x14005a620, "unknown_14005a620", tpl_43::<0x143a1a410, 0x143a1a413, 0x143078728, 0x142409b50, 0x1425deaa0, 0x141834d18> as *const ()), // sub_14005a620
    Replacement::draft(0x14005a680, "unknown_14005a680", tpl_44::<0x143a19ec8, 0x143a19ecb, 0x143078748, 0x142409b50, 0x1425deab0, 0x141834d18> as *const ()), // sub_14005a680
    Replacement::draft(0x14005a6e0, "unknown_14005a6e0", tpl_39::<0x143a1a398, 0x143a1a39b, 0x143078760, 0x142409b50, 0x1425deac0, 0x141834d18> as *const ()), // sub_14005a6e0
    Replacement::draft(0x14005a740, "unknown_14005a740", tpl_45::<0x143a1a280, 0x143a1a283, 0x143078768, 0x142409b50, 0x1425dead0, 0x141834d18> as *const ()), // sub_14005a740
    Replacement::draft(0x14005a7a0, "unknown_14005a7a0", tpl_41::<0x143a1a750, 0x143a1a753, 0x143078778, 0x142409b50, 0x1425deae0, 0x141834d18> as *const ()), // sub_14005a7a0
    Replacement::draft(0x14005a800, "unknown_14005a800", tpl_37::<0x143a19ef0, 0x143a19ef3, 0x143078788, 0x142409b50, 0x1425deaf0, 0x141834d18> as *const ()), // sub_14005a800
    Replacement::draft(0x14005a860, "unknown_14005a860", tpl_46::<0x143a1a5e8, 0x143a1a5eb, 0x143078798, 0x142409b50, 0x1425deb00, 0x141834d18> as *const ()), // sub_14005a860
    Replacement::draft(0x14005a8c0, "unknown_14005a8c0", tpl_47::<0x143a1a118, 0x143a1a11b, 0x1430787b0, 0x142409b50, 0x1425deb10, 0x141834d18> as *const ()), // sub_14005a8c0
    Replacement::draft(0x14005a920, "unknown_14005a920", tpl_38::<0x143a1a688, 0x143a1a68b, 0x1430787d0, 0x142409b50, 0x1425deb20, 0x141834d18> as *const ()), // sub_14005a920
    Replacement::draft(0x14005a980, "unknown_14005a980", tpl_48::<0x143a1a868, 0x143a1a86b, 0x1430787e0, 0x142409b50, 0x1425deb30, 0x141834d18> as *const ()), // sub_14005a980
    Replacement::draft(0x14005a9e0, "unknown_14005a9e0", tpl_49::<0x143a1a520, 0x143a1a523, 0x1430787f8, 0x142409b50, 0x1425deb40, 0x141834d18> as *const ()), // sub_14005a9e0
    Replacement::draft(0x14005aa40, "unknown_14005aa40", tpl_44::<0x143a1a7f0, 0x143a1a7f3, 0x143078808, 0x142409b50, 0x1425deb50, 0x141834d18> as *const ()), // sub_14005aa40
    Replacement::draft(0x14005aaa0, "unknown_14005aaa0", tpl_42::<0x143a19ea0, 0x143a19ea3, 0x143078820, 0x142409b50, 0x1425deb60, 0x141834d18> as *const ()), // sub_14005aaa0
    Replacement::draft(0x14005ab00, "unknown_14005ab00", tpl_50::<0x143a1a700, 0x143a1a703, 0x143078838, 0x142409b50, 0x1425deb70, 0x141834d18> as *const ()), // sub_14005ab00
    Replacement::draft(0x14005ab60, "unknown_14005ab60", tpl_52::<0x143a1a030, 0x143a1a033, 0x143a1a958, 0x143a1a95c, 0x143a1a95b, 0x143a1a960, 0x142409b50, 0x1425deb80, 0x141834d18> as *const ()), // sub_14005ab60
    Replacement::draft(0x14005ac20, "unknown_14005ac20", tpl_38::<0x143a1a570, 0x143a1a573, 0x143078950, 0x142409b50, 0x1425deb90, 0x141834d18> as *const ()), // sub_14005ac20
    Replacement::draft(0x14005ac80, "unknown_14005ac80", tpl_53::<0x143a1a0f0, 0x143a1a0f3, 0x143078960, 0x142409b50, 0x1425deba0, 0x141834d18> as *const ()), // sub_14005ac80
    Replacement::draft(0x14005ace0, "unknown_14005ace0", tpl_45::<0x143a19fb8, 0x143a19fbb, 0x143078980, 0x142409b50, 0x1425debb0, 0x141834d18> as *const ()), // sub_14005ace0
    Replacement::draft(0x14005ad40, "unknown_14005ad40", tpl_35::<0x143a1a008, 0x143a1a00b, 0x14307898c, 0x142409b50, 0x1425debc0, 0x141834d18> as *const ()), // sub_14005ad40
    Replacement::draft(0x14005ada0, "unknown_14005ada0", tpl_42::<0x143a1a8b8, 0x143a1a8bb, 0x1430786e0, 0x142409b50, 0x1425debd0, 0x141834d18> as *const ()), // sub_14005ada0
    Replacement::draft(0x14005ae00, "unknown_14005ae00", tpl_38::<0x143a1a7c8, 0x143a1a7cb, 0x143078998, 0x142409b50, 0x1425debe0, 0x141834d18> as *const ()), // sub_14005ae00
    Replacement::draft(0x14005ae60, "unknown_14005ae60", tpl_38::<0x143a1a320, 0x143a1a323, 0x1430789a8, 0x142409b50, 0x1425debf0, 0x141834d18> as *const ()), // sub_14005ae60
    Replacement::draft(0x14005aec0, "unknown_14005aec0", tpl_41::<0x143a1a480, 0x143a1a483, 0x1430789b8, 0x142409b50, 0x1425dec00, 0x141834d18> as *const ()), // sub_14005aec0
    Replacement::draft(0x14005af20, "unknown_14005af20", tpl_34::<0x143a1a908, 0x143a1a90b, 0x1430789c8, 0x142409b50, 0x1425dec10, 0x141834d18> as *const ()), // sub_14005af20
    Replacement::draft(0x14005af80, "unknown_14005af80", tpl_39::<0x143a19fe0, 0x143a19fe3, 0x1430789d8, 0x142409b50, 0x1425dec20, 0x141834d18> as *const ()), // sub_14005af80
    Replacement::draft(0x14005afe0, "unknown_14005afe0", tpl_41::<0x143a1a2f8, 0x143a1a2fb, 0x1430789e0, 0x142409b50, 0x1425dec30, 0x141834d18> as *const ()), // sub_14005afe0
    Replacement::draft(0x14005b040, "unknown_14005b040", tpl_34::<0x143a1a598, 0x143a1a59b, 0x1430789f0, 0x142409b50, 0x1425dec40, 0x141834d18> as *const ()), // sub_14005b040
    Replacement::draft(0x14005b0a0, "unknown_14005b0a0", tpl_48::<0x143a1a230, 0x143a1a233, 0x143078a00, 0x142409b50, 0x1425dec50, 0x141834d18> as *const ()), // sub_14005b0a0
    Replacement::draft(0x14005b100, "unknown_14005b100", tpl_46::<0x143a1a6b0, 0x143a1a6b3, 0x143078a18, 0x142409b50, 0x1425dec60, 0x141834d18> as *const ()), // sub_14005b100
    Replacement::draft(0x14005b160, "unknown_14005b160", tpl_47::<0x143a1a610, 0x143a1a613, 0x143078a30, 0x142409b50, 0x1425dec70, 0x141834d18> as *const ()), // sub_14005b160
    Replacement::draft(0x14005b1c0, "unknown_14005b1c0", tpl_37::<0x143a1a168, 0x143a1a16b, 0x143078a50, 0x142409b50, 0x1425dec80, 0x141834d18> as *const ()), // sub_14005b1c0
    Replacement::draft(0x14005b220, "unknown_14005b220", tpl_45::<0x143a1a3c0, 0x143a1a3c3, 0x143078a60, 0x142409b50, 0x1425dec90, 0x141834d18> as *const ()), // sub_14005b220
    Replacement::draft(0x14005b280, "unknown_14005b280", tpl_54::<0x143a1a4d0, 0x143a1a4d3, 0x143078a70, 0x142409b50, 0x1425deca0, 0x141834d18> as *const ()), // sub_14005b280
    Replacement::draft(0x14005b2e0, "unknown_14005b2e0", tpl_40::<0x143a1a140, 0x143a1a143, 0x143078a88, 0x142409b50, 0x1425decb0, 0x141834d18> as *const ()), // sub_14005b2e0
    Replacement::draft(0x14005b340, "unknown_14005b340", tpl_50::<0x143a1a190, 0x143a1a193, 0x143078a90, 0x142409b50, 0x1425decc0, 0x141834d18> as *const ()), // sub_14005b340
    Replacement::draft(0x14005b3a0, "unknown_14005b3a0", tpl_41::<0x143a1a348, 0x143a1a34b, 0x143078aa8, 0x142409b50, 0x1425decd0, 0x141834d18> as *const ()), // sub_14005b3a0
    Replacement::draft(0x14005b400, "unknown_14005b400", tpl_55::<0x143a1a638, 0x143a1a63b, 0x143078ab8, 0x142409b50, 0x1425dece0, 0x141834d18> as *const ()), // sub_14005b400
    Replacement::draft(0x14005b460, "unknown_14005b460", tpl_41::<0x143a1a6d8, 0x143a1a6db, 0x143078ac8, 0x142409b50, 0x1425decf0, 0x141834d18> as *const ()), // sub_14005b460
    Replacement::draft(0x14005b4c0, "unknown_14005b4c0", tpl_49::<0x143a1a778, 0x143a1a77b, 0x143078ad8, 0x142409b50, 0x1425ded00, 0x141834d18> as *const ()), // sub_14005b4c0
    Replacement::draft(0x14005b520, "unknown_14005b520", tpl_40::<0x143a19f90, 0x143a19f93, 0x143078ae4, 0x142409b50, 0x1425ded10, 0x141834d18> as *const ()), // sub_14005b520
    Replacement::draft(0x14005b580, "unknown_14005b580", tpl_44::<0x143a1a0c8, 0x143a1a0cb, 0x143078af0, 0x142409b50, 0x1425ded20, 0x141834d18> as *const ()), // sub_14005b580
    Replacement::draft(0x14005b5e0, "unknown_14005b5e0", tpl_46::<0x143a19f68, 0x143a19f6b, 0x143078b08, 0x142409b50, 0x1425ded30, 0x141834d18> as *const ()), // sub_14005b5e0
    Replacement::draft(0x14005b640, "unknown_14005b640", tpl_55::<0x143a1a660, 0x143a1a663, 0x143078b20, 0x142409b50, 0x1425ded40, 0x141834d18> as *const ()), // sub_14005b640
    Replacement::draft(0x14005b6a0, "unknown_14005b6a0", tpl_46::<0x143a1a4f8, 0x143a1a4fb, 0x143078b30, 0x142409b50, 0x1425ded50, 0x141834d18> as *const ()), // sub_14005b6a0
    Replacement::draft(0x14005b700, "unknown_14005b700", tpl_50::<0x143a1a258, 0x143a1a25b, 0x143078b48, 0x142409b50, 0x1425ded60, 0x141834d18> as *const ()), // sub_14005b700
    Replacement::draft(0x14005b760, "unknown_14005b760", tpl_42::<0x143a19f18, 0x143a19f1b, 0x143078b60, 0x142409b50, 0x1425ded70, 0x141834d18> as *const ()), // sub_14005b760
    Replacement::draft(0x14005b7c0, "unknown_14005b7c0", tpl_38::<0x143a1a5c0, 0x143a1a5c3, 0x143078b78, 0x142409b50, 0x1425ded80, 0x141834d18> as *const ()), // sub_14005b7c0
    Replacement::draft(0x14005b820, "unknown_14005b820", tpl_34::<0x143a1a930, 0x143a1a933, 0x143078b88, 0x142409b50, 0x1425ded90, 0x141834d18> as *const ()), // sub_14005b820
    Replacement::draft(0x14005b880, "unknown_14005b880", tpl_41::<0x143a1a728, 0x143a1a72b, 0x143078b98, 0x142409b50, 0x1425deda0, 0x141834d18> as *const ()), // sub_14005b880
    Replacement::draft(0x14005b8e0, "unknown_14005b8e0", tpl_36::<0x143a1a4a8, 0x143a1a4ab, 0x143078ba8, 0x142409b50, 0x1425dedb0, 0x141834d18> as *const ()), // sub_14005b8e0
    Replacement::draft(0x14005b940, "unknown_14005b940", tpl_34::<0x143a19f40, 0x143a19f43, 0x143078bb8, 0x142409b50, 0x1425dedc0, 0x141834d18> as *const ()), // sub_14005b940
    Replacement::draft(0x14005c610, "unknown_14005c610", tpl_23::<0x143a1a980, 0x143a1a983, 0x1426cdcb0, 0x142409b50, 0x1425dee60, 0x141834d18> as *const ()), // sub_14005c610
    Replacement::draft(0x14005c670, "unknown_14005c670", tpl_23::<0x143a1a9c8, 0x143a1a9cb, 0x1426cdcb0, 0x142409b50, 0x1425dee70, 0x141834d18> as *const ()), // sub_14005c670
    Replacement::draft(0x14005c860, "unknown_14005c860", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438465e0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425deeb0, 0x141834d18> as *const ()), // sub_14005c860
    Replacement::draft(0x14005d400, "unknown_14005d400", tpl_23::<0x143a1aa40, 0x143a1aa43, 0x1426cdcb0, 0x142409b50, 0x1425def50, 0x141834d18> as *const ()), // sub_14005d400
    Replacement::draft(0x14005d460, "unknown_14005d460", tpl_23::<0x143a1aa88, 0x143a1aa8b, 0x1426cdcb0, 0x142409b50, 0x1425def60, 0x141834d18> as *const ()), // sub_14005d460
    Replacement::draft(0x14005d650, "unknown_14005d650", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438466b0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425defa0, 0x141834d18> as *const ()), // sub_14005d650
    Replacement::draft(0x14005dd70, "unknown_14005dd70", fn_14005dd70 as *const ()), // sub_14005dd70
    Replacement::draft(0x14005de10, "unknown_14005de10", fn_14005de10 as *const ()), // sub_14005de10
    Replacement::draft(0x140063600, "unknown_140063600", tpl_23::<0x143a1ab50, 0x143a1ab53, 0x1426cdcb0, 0x142409b50, 0x1425df860, 0x141834d18> as *const ()), // sub_140063600
    Replacement::draft(0x140063660, "unknown_140063660", tpl_23::<0x143a1ab98, 0x143a1ab9b, 0x1426cdcb0, 0x142409b50, 0x1425df870, 0x141834d18> as *const ()), // sub_140063660
    Replacement::draft(0x140063850, "unknown_140063850", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438468f8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425df8b0, 0x141834d18> as *const ()), // sub_140063850
    Replacement::draft(0x1400646a0, "unknown_1400646a0", tpl_23::<0x143a1ac00, 0x143a1ac03, 0x1426cdcb0, 0x142409b50, 0x1425df970, 0x141834d18> as *const ()), // sub_1400646a0
    Replacement::draft(0x140064700, "unknown_140064700", tpl_23::<0x143a1ac48, 0x143a1ac4b, 0x1426cdcb0, 0x142409b50, 0x1425df980, 0x141834d18> as *const ()), // sub_140064700
    Replacement::draft(0x1400648f0, "unknown_1400648f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846988, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425df9c0, 0x141834d18> as *const ()), // sub_1400648f0
    Replacement::draft(0x140065dd0, "unknown_140065dd0", tpl_23::<0x143a1acb0, 0x143a1acb3, 0x1426cdcb0, 0x142409b50, 0x1425dfaa0, 0x141834d18> as *const ()), // sub_140065dd0
    Replacement::draft(0x140065e30, "unknown_140065e30", tpl_23::<0x143a1acf8, 0x143a1acfb, 0x1426cdcb0, 0x142409b50, 0x1425dfab0, 0x141834d18> as *const ()), // sub_140065e30
    Replacement::draft(0x140066020, "unknown_140066020", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846a18, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dfaf0, 0x141834d18> as *const ()), // sub_140066020
    Replacement::draft(0x140066420, "unknown_140066420", tpl_84::<0x1430d8ac0, 0x1430d8a84, 0x143a1ad20, 0x1430d8a88, 0x1430d8aa0, 0x1430d8ab0, 0x1430d8bd8, 0x1430d8bf0, 0x1430d8c00, 0x1430d8c18, 0x14038be40, 0x1425dfb60, 0x141834d18> as *const ()), // sub_140066420
    Replacement::draft(0x1400678b0, "unknown_1400678b0", tpl_23::<0x143a1ada0, 0x143a1ada3, 0x1426cdcb0, 0x142409b50, 0x1425dfc50, 0x141834d18> as *const ()), // sub_1400678b0
    Replacement::draft(0x140067910, "unknown_140067910", tpl_23::<0x143a1ade8, 0x143a1adeb, 0x1426cdcb0, 0x142409b50, 0x1425dfc60, 0x141834d18> as *const ()), // sub_140067910
    Replacement::draft(0x140067b00, "unknown_140067b00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846aa8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dfca0, 0x141834d18> as *const ()), // sub_140067b00
    Replacement::draft(0x140067f00, "unknown_140067f00", tpl_23::<0x143a1ae50, 0x143a1ae53, 0x1426cdcb0, 0x142409b50, 0x1425dfd10, 0x141834d18> as *const ()), // sub_140067f00
    Replacement::draft(0x140067f60, "unknown_140067f60", tpl_23::<0x143a1ae98, 0x143a1ae9b, 0x1426cdcb0, 0x142409b50, 0x1425dfd20, 0x141834d18> as *const ()), // sub_140067f60
    Replacement::draft(0x140068150, "unknown_140068150", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846b38, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dfd60, 0x141834d18> as *const ()), // sub_140068150
    Replacement::draft(0x140068f30, "unknown_140068f30", tpl_23::<0x143a1af00, 0x143a1af03, 0x1426cdcb0, 0x142409b50, 0x1425dff50, 0x141834d18> as *const ()), // sub_140068f30
    Replacement::draft(0x140068f90, "unknown_140068f90", tpl_23::<0x143a1af48, 0x143a1af4b, 0x1426cdcb0, 0x142409b50, 0x1425dff60, 0x141834d18> as *const ()), // sub_140068f90
    Replacement::draft(0x140069180, "unknown_140069180", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846c08, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425dffa0, 0x141834d18> as *const ()), // sub_140069180
    Replacement::draft(0x14006a010, "unknown_14006a010", tpl_23::<0x143a1afb0, 0x143a1afb3, 0x1426cdcb0, 0x142409b50, 0x1425e0070, 0x141834d18> as *const ()), // sub_14006a010
    Replacement::draft(0x14006a070, "unknown_14006a070", tpl_23::<0x143a1aff8, 0x143a1affb, 0x1426cdcb0, 0x142409b50, 0x1425e0080, 0x141834d18> as *const ()), // sub_14006a070
    Replacement::draft(0x14006a260, "unknown_14006a260", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846cb0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e00c0, 0x141834d18> as *const ()), // sub_14006a260
    Replacement::draft(0x14006abb0, "unknown_14006abb0", tpl_23::<0x143a1b060, 0x143a1b063, 0x1426cdcb0, 0x142409b50, 0x1425e06f0, 0x141834d18> as *const ()), // sub_14006abb0
    Replacement::draft(0x14006ac10, "unknown_14006ac10", tpl_23::<0x143a1b0a8, 0x143a1b0ab, 0x1426cdcb0, 0x142409b50, 0x1425e0700, 0x141834d18> as *const ()), // sub_14006ac10
    Replacement::draft(0x14006ae00, "unknown_14006ae00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846ea8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e0740, 0x141834d18> as *const ()), // sub_14006ae00
    Replacement::draft(0x14006c7b0, "unknown_14006c7b0", tpl_23::<0x143a1b110, 0x143a1b113, 0x1426cdcb0, 0x142409b50, 0x1425e0c80, 0x141834d18> as *const ()), // sub_14006c7b0
    Replacement::draft(0x14006c810, "unknown_14006c810", tpl_23::<0x143a1b158, 0x143a1b15b, 0x1426cdcb0, 0x142409b50, 0x1425e0c90, 0x141834d18> as *const ()), // sub_14006c810
    Replacement::draft(0x14006ca00, "unknown_14006ca00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143846ff0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e0cd0, 0x141834d18> as *const ()), // sub_14006ca00
    Replacement::draft(0x14006ce00, "unknown_14006ce00", fn_14006ce00 as *const ()), // sub_14006ce00
    Replacement::draft(0x14006cea0, "unknown_14006cea0", fn_14006cea0 as *const ()), // sub_14006cea0
    Replacement::draft(0x14006cf40, "unknown_14006cf40", fn_14006cf40 as *const ()), // sub_14006cf40
    Replacement::draft(0x14006d2b0, "unknown_14006d2b0", tpl_23::<0x143a1b1f0, 0x143a1b1f3, 0x1426cdcb0, 0x142409b50, 0x1425e0d50, 0x141834d18> as *const ()), // sub_14006d2b0
    Replacement::draft(0x14006d310, "unknown_14006d310", tpl_23::<0x143a1b238, 0x143a1b23b, 0x1426cdcb0, 0x142409b50, 0x1425e0d60, 0x141834d18> as *const ()), // sub_14006d310
    Replacement::draft(0x14006d500, "unknown_14006d500", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847080, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e0da0, 0x141834d18> as *const ()), // sub_14006d500
    Replacement::draft(0x14006dfa0, "unknown_14006dfa0", tpl_23::<0x143a1b2a0, 0x143a1b2a3, 0x1426cdcb0, 0x142409b50, 0x1425e1090, 0x141834d18> as *const ()), // sub_14006dfa0
    Replacement::draft(0x14006e000, "unknown_14006e000", tpl_23::<0x143a1b2e8, 0x143a1b2eb, 0x1426cdcb0, 0x142409b50, 0x1425e10a0, 0x141834d18> as *const ()), // sub_14006e000
    Replacement::draft(0x14006e1f0, "unknown_14006e1f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847190, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e10e0, 0x141834d18> as *const ()), // sub_14006e1f0
    Replacement::draft(0x14006f170, "unknown_14006f170", tpl_23::<0x143a1b350, 0x143a1b353, 0x1426cdcb0, 0x142409b50, 0x1425e1230, 0x141834d18> as *const ()), // sub_14006f170
    Replacement::draft(0x14006f1d0, "unknown_14006f1d0", tpl_23::<0x143a1b398, 0x143a1b39b, 0x1426cdcb0, 0x142409b50, 0x1425e1240, 0x141834d18> as *const ()), // sub_14006f1d0
    Replacement::draft(0x14006f3c0, "unknown_14006f3c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847368, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e1280, 0x141834d18> as *const ()), // sub_14006f3c0
    Replacement::draft(0x1400705a0, "unknown_1400705a0", tpl_23::<0x143a1b400, 0x143a1b403, 0x1426cdcb0, 0x142409b50, 0x1425e25a0, 0x141834d18> as *const ()), // sub_1400705a0
    Replacement::draft(0x140070600, "unknown_140070600", tpl_23::<0x143a1b448, 0x143a1b44b, 0x1426cdcb0, 0x142409b50, 0x1425e25b0, 0x141834d18> as *const ()), // sub_140070600
    Replacement::draft(0x1400706d0, "unknown_1400706d0", tpl_23::<0x143a1b470, 0x143a1b473, 0x1426cdcb0, 0x142409b50, 0x1425e25d0, 0x141834d18> as *const ()), // sub_1400706d0
    Replacement::draft(0x140070730, "unknown_140070730", tpl_23::<0x143a1b4b8, 0x143a1b4bb, 0x1426cdcb0, 0x142409b50, 0x1425e25e0, 0x141834d18> as *const ()), // sub_140070730
    Replacement::draft(0x140070920, "unknown_140070920", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438476a0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e2620, 0x141834d18> as *const ()), // sub_140070920
    Replacement::draft(0x140070d20, "unknown_140070d20", tpl_23::<0x143a1b6d8, 0x143a1b6db, 0x1426cdcb0, 0x142409b50, 0x1425e2690, 0x141834d18> as *const ()), // sub_140070d20
    Replacement::draft(0x140070d80, "unknown_140070d80", tpl_23::<0x143a1b720, 0x143a1b723, 0x1426cdcb0, 0x142409b50, 0x1425e26a0, 0x141834d18> as *const ()), // sub_140070d80
    Replacement::draft(0x140070f70, "unknown_140070f70", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847730, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e26e0, 0x141834d18> as *const ()), // sub_140070f70
    Replacement::draft(0x140071300, "unknown_140071300", tpl_34::<0x143a1b888, 0x143a1b88b, 0x143078668, 0x142409b50, 0x1425e2720, 0x141834d18> as *const ()), // sub_140071300
    Replacement::draft(0x140071360, "unknown_140071360", tpl_23::<0x143a1bee0, 0x143a1bee3, 0x1426cdcb0, 0x142409b50, 0x1425e2730, 0x141834d18> as *const ()), // sub_140071360
    Replacement::draft(0x1400713c0, "unknown_1400713c0", tpl_35::<0x143a1bbc8, 0x143a1bbcb, 0x143078678, 0x142409b50, 0x1425e2740, 0x141834d18> as *const ()), // sub_1400713c0
    Replacement::draft(0x140071420, "unknown_140071420", tpl_36::<0x143a1be68, 0x143a1be6b, 0x143078680, 0x142409b50, 0x1425e2750, 0x141834d18> as *const ()), // sub_140071420
    Replacement::draft(0x140071480, "unknown_140071480", tpl_37::<0x143a1ba68, 0x143a1ba6b, 0x143078690, 0x142409b50, 0x1425e2760, 0x141834d18> as *const ()), // sub_140071480
    Replacement::draft(0x1400714e0, "unknown_1400714e0", tpl_38::<0x143a1b928, 0x143a1b92b, 0x1430786a0, 0x142409b50, 0x1425e2770, 0x141834d18> as *const ()), // sub_1400714e0
    Replacement::draft(0x140071540, "unknown_140071540", tpl_39::<0x143a1bf08, 0x143a1bf0b, 0x1430786b0, 0x142409b50, 0x1425e2780, 0x141834d18> as *const ()), // sub_140071540
    Replacement::draft(0x1400715a0, "unknown_1400715a0", tpl_37::<0x143a1b838, 0x143a1b83b, 0x1430786b8, 0x142409b50, 0x1425e2790, 0x141834d18> as *const ()), // sub_1400715a0
    Replacement::draft(0x140071600, "unknown_140071600", tpl_40::<0x143a1c020, 0x143a1c023, 0x1430786c4, 0x142409b50, 0x1425e27a0, 0x141834d18> as *const ()), // sub_140071600
    Replacement::draft(0x140071660, "unknown_140071660", tpl_41::<0x143a1b9f0, 0x143a1b9f3, 0x1430786d0, 0x142409b50, 0x1425e27b0, 0x141834d18> as *const ()), // sub_140071660
    Replacement::draft(0x1400716c0, "unknown_1400716c0", tpl_42::<0x143a1bf58, 0x143a1bf5b, 0x1430786e0, 0x142409b50, 0x1425e27c0, 0x141834d18> as *const ()), // sub_1400716c0
    Replacement::draft(0x140071720, "unknown_140071720", tpl_34::<0x143a1bfa8, 0x143a1bfab, 0x1430786f8, 0x142409b50, 0x1425e27d0, 0x141834d18> as *const ()), // sub_140071720
    Replacement::draft(0x140071780, "unknown_140071780", tpl_34::<0x143a1b860, 0x143a1b863, 0x143078708, 0x142409b50, 0x1425e27e0, 0x141834d18> as *const ()), // sub_140071780
    Replacement::draft(0x1400717e0, "unknown_1400717e0", tpl_41::<0x143a1b950, 0x143a1b953, 0x143078718, 0x142409b50, 0x1425e27f0, 0x141834d18> as *const ()), // sub_1400717e0
    Replacement::draft(0x140071840, "unknown_140071840", tpl_43::<0x143a1ba90, 0x143a1ba93, 0x143078728, 0x142409b50, 0x1425e2800, 0x141834d18> as *const ()), // sub_140071840
    Replacement::draft(0x1400718a0, "unknown_1400718a0", tpl_44::<0x143a1b548, 0x143a1b54b, 0x143078748, 0x142409b50, 0x1425e2810, 0x141834d18> as *const ()), // sub_1400718a0
    Replacement::draft(0x140071900, "unknown_140071900", tpl_39::<0x143a1ba18, 0x143a1ba1b, 0x143078760, 0x142409b50, 0x1425e2820, 0x141834d18> as *const ()), // sub_140071900
    Replacement::draft(0x140071960, "unknown_140071960", tpl_45::<0x143a1b900, 0x143a1b903, 0x143078768, 0x142409b50, 0x1425e2830, 0x141834d18> as *const ()), // sub_140071960
    Replacement::draft(0x1400719c0, "unknown_1400719c0", tpl_41::<0x143a1be18, 0x143a1be1b, 0x143078778, 0x142409b50, 0x1425e2840, 0x141834d18> as *const ()), // sub_1400719c0
    Replacement::draft(0x140071a20, "unknown_140071a20", tpl_37::<0x143a1b570, 0x143a1b573, 0x143078788, 0x142409b50, 0x1425e2850, 0x141834d18> as *const ()), // sub_140071a20
    Replacement::draft(0x140071a80, "unknown_140071a80", tpl_46::<0x143a1bc68, 0x143a1bc6b, 0x143078798, 0x142409b50, 0x1425e2860, 0x141834d18> as *const ()), // sub_140071a80
    Replacement::draft(0x140071ae0, "unknown_140071ae0", tpl_47::<0x143a1b798, 0x143a1b79b, 0x1430787b0, 0x142409b50, 0x1425e2870, 0x141834d18> as *const ()), // sub_140071ae0
    Replacement::draft(0x140071b40, "unknown_140071b40", tpl_38::<0x143a1bd08, 0x143a1bd0b, 0x1430787d0, 0x142409b50, 0x1425e2880, 0x141834d18> as *const ()), // sub_140071b40
    Replacement::draft(0x140071ba0, "unknown_140071ba0", tpl_48::<0x143a1bf30, 0x143a1bf33, 0x1430787e0, 0x142409b50, 0x1425e2890, 0x141834d18> as *const ()), // sub_140071ba0
    Replacement::draft(0x140071c00, "unknown_140071c00", tpl_49::<0x143a1bba0, 0x143a1bba3, 0x1430787f8, 0x142409b50, 0x1425e28a0, 0x141834d18> as *const ()), // sub_140071c00
    Replacement::draft(0x140071c60, "unknown_140071c60", tpl_44::<0x143a1beb8, 0x143a1bebb, 0x143078808, 0x142409b50, 0x1425e28b0, 0x141834d18> as *const ()), // sub_140071c60
    Replacement::draft(0x140071cc0, "unknown_140071cc0", tpl_42::<0x143a1b520, 0x143a1b523, 0x143078820, 0x142409b50, 0x1425e28c0, 0x141834d18> as *const ()), // sub_140071cc0
    Replacement::draft(0x140071d20, "unknown_140071d20", tpl_50::<0x143a1bd80, 0x143a1bd83, 0x143078838, 0x142409b50, 0x1425e28d0, 0x141834d18> as *const ()), // sub_140071d20
    Replacement::draft(0x140071d80, "unknown_140071d80", tpl_52::<0x143a1b6b0, 0x143a1b6b3, 0x143a1c020, 0x143a1c024, 0x143a1c023, 0x143a1c028, 0x142409b50, 0x1425e28e0, 0x141834d18> as *const ()), // sub_140071d80
    Replacement::draft(0x140071e40, "unknown_140071e40", tpl_38::<0x143a1bbf0, 0x143a1bbf3, 0x143078950, 0x142409b50, 0x1425e28f0, 0x141834d18> as *const ()), // sub_140071e40
    Replacement::draft(0x140071ea0, "unknown_140071ea0", tpl_53::<0x143a1b770, 0x143a1b773, 0x143078960, 0x142409b50, 0x1425e2900, 0x141834d18> as *const ()), // sub_140071ea0
    Replacement::draft(0x140071f00, "unknown_140071f00", tpl_45::<0x143a1b638, 0x143a1b63b, 0x143078980, 0x142409b50, 0x1425e2910, 0x141834d18> as *const ()), // sub_140071f00
    Replacement::draft(0x140071f60, "unknown_140071f60", tpl_35::<0x143a1b688, 0x143a1b68b, 0x14307898c, 0x142409b50, 0x1425e2920, 0x141834d18> as *const ()), // sub_140071f60
    Replacement::draft(0x140071fc0, "unknown_140071fc0", tpl_42::<0x143a1bf80, 0x143a1bf83, 0x1430786e0, 0x142409b50, 0x1425e2930, 0x141834d18> as *const ()), // sub_140071fc0
    Replacement::draft(0x140072020, "unknown_140072020", tpl_38::<0x143a1be90, 0x143a1be93, 0x143078998, 0x142409b50, 0x1425e2940, 0x141834d18> as *const ()), // sub_140072020
    Replacement::draft(0x140072080, "unknown_140072080", tpl_38::<0x143a1b9a0, 0x143a1b9a3, 0x1430789a8, 0x142409b50, 0x1425e2950, 0x141834d18> as *const ()), // sub_140072080
    Replacement::draft(0x1400720e0, "unknown_1400720e0", tpl_41::<0x143a1bb00, 0x143a1bb03, 0x1430789b8, 0x142409b50, 0x1425e2960, 0x141834d18> as *const ()), // sub_1400720e0
    Replacement::draft(0x140072140, "unknown_140072140", tpl_34::<0x143a1bfd0, 0x143a1bfd3, 0x1430789c8, 0x142409b50, 0x1425e2970, 0x141834d18> as *const ()), // sub_140072140
    Replacement::draft(0x1400721a0, "unknown_1400721a0", tpl_39::<0x143a1b660, 0x143a1b663, 0x1430789d8, 0x142409b50, 0x1425e2980, 0x141834d18> as *const ()), // sub_1400721a0
    Replacement::draft(0x140072200, "unknown_140072200", tpl_41::<0x143a1b978, 0x143a1b97b, 0x1430789e0, 0x142409b50, 0x1425e2990, 0x141834d18> as *const ()), // sub_140072200
    Replacement::draft(0x140072260, "unknown_140072260", tpl_34::<0x143a1bc18, 0x143a1bc1b, 0x1430789f0, 0x142409b50, 0x1425e29a0, 0x141834d18> as *const ()), // sub_140072260
    Replacement::draft(0x1400722c0, "unknown_1400722c0", tpl_48::<0x143a1b8b0, 0x143a1b8b3, 0x143078a00, 0x142409b50, 0x1425e29b0, 0x141834d18> as *const ()), // sub_1400722c0
    Replacement::draft(0x140072320, "unknown_140072320", tpl_46::<0x143a1bd30, 0x143a1bd33, 0x143078a18, 0x142409b50, 0x1425e29c0, 0x141834d18> as *const ()), // sub_140072320
    Replacement::draft(0x140072380, "unknown_140072380", tpl_47::<0x143a1bc90, 0x143a1bc93, 0x143078a30, 0x142409b50, 0x1425e29d0, 0x141834d18> as *const ()), // sub_140072380
    Replacement::draft(0x1400723e0, "unknown_1400723e0", tpl_37::<0x143a1b7e8, 0x143a1b7eb, 0x143078a50, 0x142409b50, 0x1425e29e0, 0x141834d18> as *const ()), // sub_1400723e0
    Replacement::draft(0x140072440, "unknown_140072440", tpl_45::<0x143a1ba40, 0x143a1ba43, 0x143078a60, 0x142409b50, 0x1425e29f0, 0x141834d18> as *const ()), // sub_140072440
    Replacement::draft(0x1400724a0, "unknown_1400724a0", tpl_54::<0x143a1bb50, 0x143a1bb53, 0x143078a70, 0x142409b50, 0x1425e2a00, 0x141834d18> as *const ()), // sub_1400724a0
    Replacement::draft(0x140072500, "unknown_140072500", tpl_40::<0x143a1b7c0, 0x143a1b7c3, 0x143078a88, 0x142409b50, 0x1425e2a10, 0x141834d18> as *const ()), // sub_140072500
    Replacement::draft(0x140072560, "unknown_140072560", tpl_50::<0x143a1b810, 0x143a1b813, 0x143078a90, 0x142409b50, 0x1425e2a20, 0x141834d18> as *const ()), // sub_140072560
    Replacement::draft(0x1400725c0, "unknown_1400725c0", tpl_41::<0x143a1b9c8, 0x143a1b9cb, 0x143078aa8, 0x142409b50, 0x1425e2a30, 0x141834d18> as *const ()), // sub_1400725c0
    Replacement::draft(0x140072620, "unknown_140072620", tpl_55::<0x143a1bcb8, 0x143a1bcbb, 0x143078ab8, 0x142409b50, 0x1425e2a40, 0x141834d18> as *const ()), // sub_140072620
    Replacement::draft(0x140072680, "unknown_140072680", tpl_41::<0x143a1bd58, 0x143a1bd5b, 0x143078ac8, 0x142409b50, 0x1425e2a50, 0x141834d18> as *const ()), // sub_140072680
    Replacement::draft(0x1400726e0, "unknown_1400726e0", tpl_49::<0x143a1be40, 0x143a1be43, 0x143078ad8, 0x142409b50, 0x1425e2a60, 0x141834d18> as *const ()), // sub_1400726e0
    Replacement::draft(0x140072740, "unknown_140072740", tpl_40::<0x143a1b610, 0x143a1b613, 0x143078ae4, 0x142409b50, 0x1425e2a70, 0x141834d18> as *const ()), // sub_140072740
    Replacement::draft(0x1400727a0, "unknown_1400727a0", tpl_44::<0x143a1b748, 0x143a1b74b, 0x143078af0, 0x142409b50, 0x1425e2a80, 0x141834d18> as *const ()), // sub_1400727a0
    Replacement::draft(0x140072800, "unknown_140072800", tpl_46::<0x143a1b5e8, 0x143a1b5eb, 0x143078b08, 0x142409b50, 0x1425e2a90, 0x141834d18> as *const ()), // sub_140072800
    Replacement::draft(0x140072860, "unknown_140072860", tpl_55::<0x143a1bce0, 0x143a1bce3, 0x143078b20, 0x142409b50, 0x1425e2aa0, 0x141834d18> as *const ()), // sub_140072860
    Replacement::draft(0x1400728c0, "unknown_1400728c0", tpl_46::<0x143a1bb78, 0x143a1bb7b, 0x143078b30, 0x142409b50, 0x1425e2ab0, 0x141834d18> as *const ()), // sub_1400728c0
    Replacement::draft(0x140072920, "unknown_140072920", tpl_50::<0x143a1b8d8, 0x143a1b8db, 0x143078b48, 0x142409b50, 0x1425e2ac0, 0x141834d18> as *const ()), // sub_140072920
    Replacement::draft(0x140072980, "unknown_140072980", tpl_42::<0x143a1b598, 0x143a1b59b, 0x143078b60, 0x142409b50, 0x1425e2ad0, 0x141834d18> as *const ()), // sub_140072980
    Replacement::draft(0x1400729e0, "unknown_1400729e0", tpl_38::<0x143a1bc40, 0x143a1bc43, 0x143078b78, 0x142409b50, 0x1425e2ae0, 0x141834d18> as *const ()), // sub_1400729e0
    Replacement::draft(0x140072a40, "unknown_140072a40", tpl_34::<0x143a1bff8, 0x143a1bffb, 0x143078b88, 0x142409b50, 0x1425e2af0, 0x141834d18> as *const ()), // sub_140072a40
    Replacement::draft(0x140072aa0, "unknown_140072aa0", tpl_41::<0x143a1bda8, 0x143a1bdab, 0x143078b98, 0x142409b50, 0x1425e2b00, 0x141834d18> as *const ()), // sub_140072aa0
    Replacement::draft(0x140072b00, "unknown_140072b00", tpl_36::<0x143a1bb28, 0x143a1bb2b, 0x143078ba8, 0x142409b50, 0x1425e2b10, 0x141834d18> as *const ()), // sub_140072b00
    Replacement::draft(0x140072b60, "unknown_140072b60", tpl_34::<0x143a1b5c0, 0x143a1b5c3, 0x143078bb8, 0x142409b50, 0x1425e2b20, 0x141834d18> as *const ()), // sub_140072b60
    Replacement::draft(0x140072cf0, "unknown_140072cf0", fn_140072cf0 as *const ()), // sub_140072cf0
    Replacement::draft(0x140074c40, "unknown_140074c40", tpl_34::<0x143a1c3b0, 0x143a1c3b3, 0x143078668, 0x142409b50, 0x1425e2da0, 0x141834d18> as *const ()), // sub_140074c40
    Replacement::draft(0x140074ca0, "unknown_140074ca0", tpl_23::<0x143a1c9b8, 0x143a1c9bb, 0x1426cdcb0, 0x142409b50, 0x1425e2db0, 0x141834d18> as *const ()), // sub_140074ca0
    Replacement::draft(0x140074d00, "unknown_140074d00", tpl_35::<0x143a1c6e8, 0x143a1c6eb, 0x143078678, 0x142409b50, 0x1425e2dc0, 0x141834d18> as *const ()), // sub_140074d00
    Replacement::draft(0x140074d60, "unknown_140074d60", tpl_36::<0x143a1c940, 0x143a1c943, 0x143078680, 0x142409b50, 0x1425e2dd0, 0x141834d18> as *const ()), // sub_140074d60
    Replacement::draft(0x140074dc0, "unknown_140074dc0", tpl_37::<0x143a1c590, 0x143a1c593, 0x143078690, 0x142409b50, 0x1425e2de0, 0x141834d18> as *const ()), // sub_140074dc0
    Replacement::draft(0x140074e20, "unknown_140074e20", tpl_38::<0x143a1c450, 0x143a1c453, 0x1430786a0, 0x142409b50, 0x1425e2df0, 0x141834d18> as *const ()), // sub_140074e20
    Replacement::draft(0x140074e80, "unknown_140074e80", tpl_39::<0x143a1c9e0, 0x143a1c9e3, 0x1430786b0, 0x142409b50, 0x1425e2e00, 0x141834d18> as *const ()), // sub_140074e80
    Replacement::draft(0x140074ee0, "unknown_140074ee0", tpl_37::<0x143a1c360, 0x143a1c363, 0x1430786b8, 0x142409b50, 0x1425e2e10, 0x141834d18> as *const ()), // sub_140074ee0
    Replacement::draft(0x140074f40, "unknown_140074f40", tpl_40::<0x143a1caf8, 0x143a1cafb, 0x1430786c4, 0x142409b50, 0x1425e2e20, 0x141834d18> as *const ()), // sub_140074f40
    Replacement::draft(0x140074fa0, "unknown_140074fa0", tpl_41::<0x143a1c518, 0x143a1c51b, 0x1430786d0, 0x142409b50, 0x1425e2e30, 0x141834d18> as *const ()), // sub_140074fa0
    Replacement::draft(0x140075000, "unknown_140075000", tpl_42::<0x143a1ca30, 0x143a1ca33, 0x1430786e0, 0x142409b50, 0x1425e2e40, 0x141834d18> as *const ()), // sub_140075000
    Replacement::draft(0x140075060, "unknown_140075060", tpl_34::<0x143a1ca80, 0x143a1ca83, 0x1430786f8, 0x142409b50, 0x1425e2e50, 0x141834d18> as *const ()), // sub_140075060
    Replacement::draft(0x1400750c0, "unknown_1400750c0", tpl_34::<0x143a1c388, 0x143a1c38b, 0x143078708, 0x142409b50, 0x1425e2e60, 0x141834d18> as *const ()), // sub_1400750c0
    Replacement::draft(0x140075120, "unknown_140075120", tpl_41::<0x143a1c478, 0x143a1c47b, 0x143078718, 0x142409b50, 0x1425e2e70, 0x141834d18> as *const ()), // sub_140075120
    Replacement::draft(0x140075180, "unknown_140075180", tpl_43::<0x143a1c5b8, 0x143a1c5bb, 0x143078728, 0x142409b50, 0x1425e2e80, 0x141834d18> as *const ()), // sub_140075180
    Replacement::draft(0x1400751e0, "unknown_1400751e0", tpl_44::<0x143a1c070, 0x143a1c073, 0x143078748, 0x142409b50, 0x1425e2e90, 0x141834d18> as *const ()), // sub_1400751e0
    Replacement::draft(0x140075240, "unknown_140075240", tpl_39::<0x143a1c540, 0x143a1c543, 0x143078760, 0x142409b50, 0x1425e2ea0, 0x141834d18> as *const ()), // sub_140075240
    Replacement::draft(0x1400752a0, "unknown_1400752a0", tpl_45::<0x143a1c428, 0x143a1c42b, 0x143078768, 0x142409b50, 0x1425e2eb0, 0x141834d18> as *const ()), // sub_1400752a0
    Replacement::draft(0x140075300, "unknown_140075300", tpl_41::<0x143a1c8f0, 0x143a1c8f3, 0x143078778, 0x142409b50, 0x1425e2ec0, 0x141834d18> as *const ()), // sub_140075300
    Replacement::draft(0x140075360, "unknown_140075360", tpl_37::<0x143a1c098, 0x143a1c09b, 0x143078788, 0x142409b50, 0x1425e2ed0, 0x141834d18> as *const ()), // sub_140075360
    Replacement::draft(0x1400753c0, "unknown_1400753c0", tpl_46::<0x143a1c788, 0x143a1c78b, 0x143078798, 0x142409b50, 0x1425e2ee0, 0x141834d18> as *const ()), // sub_1400753c0
    Replacement::draft(0x140075420, "unknown_140075420", tpl_47::<0x143a1c2c0, 0x143a1c2c3, 0x1430787b0, 0x142409b50, 0x1425e2ef0, 0x141834d18> as *const ()), // sub_140075420
    Replacement::draft(0x140075480, "unknown_140075480", tpl_38::<0x143a1c828, 0x143a1c82b, 0x1430787d0, 0x142409b50, 0x1425e2f00, 0x141834d18> as *const ()), // sub_140075480
    Replacement::draft(0x1400754e0, "unknown_1400754e0", tpl_48::<0x143a1ca08, 0x143a1ca0b, 0x1430787e0, 0x142409b50, 0x1425e2f10, 0x141834d18> as *const ()), // sub_1400754e0
    Replacement::draft(0x140075540, "unknown_140075540", tpl_49::<0x143a1c6c0, 0x143a1c6c3, 0x1430787f8, 0x142409b50, 0x1425e2f20, 0x141834d18> as *const ()), // sub_140075540
    Replacement::draft(0x1400755a0, "unknown_1400755a0", tpl_44::<0x143a1c990, 0x143a1c993, 0x143078808, 0x142409b50, 0x1425e2f30, 0x141834d18> as *const ()), // sub_1400755a0
    Replacement::draft(0x140075600, "unknown_140075600", tpl_42::<0x143a1c048, 0x143a1c04b, 0x143078820, 0x142409b50, 0x1425e2f40, 0x141834d18> as *const ()), // sub_140075600
    Replacement::draft(0x140075660, "unknown_140075660", tpl_50::<0x143a1c8a0, 0x143a1c8a3, 0x143078838, 0x142409b50, 0x1425e2f50, 0x141834d18> as *const ()), // sub_140075660
    Replacement::draft(0x1400756c0, "unknown_1400756c0", tpl_23::<0x143a1c200, 0x143a1c203, 0x1426cdcb0, 0x142409b50, 0x1425e2f60, 0x141834d18> as *const ()), // sub_1400756c0
    Replacement::draft(0x140075720, "unknown_140075720", tpl_23::<0x143a1c248, 0x143a1c24b, 0x1426cdcb0, 0x142409b50, 0x1425e2f70, 0x141834d18> as *const ()), // sub_140075720
    Replacement::draft(0x140075910, "unknown_140075910", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847a30, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e2fb0, 0x141834d18> as *const ()), // sub_140075910
    Replacement::draft(0x140075ca0, "unknown_140075ca0", tpl_52::<0x143a1c1d8, 0x143a1c1db, 0x143a1caf8, 0x143a1cafc, 0x143a1cafb, 0x143a1cb00, 0x142409b50, 0x1425e2ff0, 0x141834d18> as *const ()), // sub_140075ca0
    Replacement::draft(0x140075d60, "unknown_140075d60", tpl_38::<0x143a1c710, 0x143a1c713, 0x143078950, 0x142409b50, 0x1425e3000, 0x141834d18> as *const ()), // sub_140075d60
    Replacement::draft(0x140075dc0, "unknown_140075dc0", tpl_53::<0x143a1c298, 0x143a1c29b, 0x143078960, 0x142409b50, 0x1425e3010, 0x141834d18> as *const ()), // sub_140075dc0
    Replacement::draft(0x140075e20, "unknown_140075e20", tpl_45::<0x143a1c160, 0x143a1c163, 0x143078980, 0x142409b50, 0x1425e3020, 0x141834d18> as *const ()), // sub_140075e20
    Replacement::draft(0x140075e80, "unknown_140075e80", tpl_35::<0x143a1c1b0, 0x143a1c1b3, 0x14307898c, 0x142409b50, 0x1425e3030, 0x141834d18> as *const ()), // sub_140075e80
    Replacement::draft(0x140075ee0, "unknown_140075ee0", tpl_42::<0x143a1ca58, 0x143a1ca5b, 0x1430786e0, 0x142409b50, 0x1425e3040, 0x141834d18> as *const ()), // sub_140075ee0
    Replacement::draft(0x140075f40, "unknown_140075f40", tpl_38::<0x143a1c968, 0x143a1c96b, 0x143078998, 0x142409b50, 0x1425e3050, 0x141834d18> as *const ()), // sub_140075f40
    Replacement::draft(0x140075fa0, "unknown_140075fa0", tpl_38::<0x143a1c4c8, 0x143a1c4cb, 0x1430789a8, 0x142409b50, 0x1425e3060, 0x141834d18> as *const ()), // sub_140075fa0
    Replacement::draft(0x140076000, "unknown_140076000", tpl_41::<0x143a1c620, 0x143a1c623, 0x1430789b8, 0x142409b50, 0x1425e3070, 0x141834d18> as *const ()), // sub_140076000
    Replacement::draft(0x140076060, "unknown_140076060", tpl_34::<0x143a1caa8, 0x143a1caab, 0x1430789c8, 0x142409b50, 0x1425e3080, 0x141834d18> as *const ()), // sub_140076060
    Replacement::draft(0x1400760c0, "unknown_1400760c0", tpl_39::<0x143a1c188, 0x143a1c18b, 0x1430789d8, 0x142409b50, 0x1425e3090, 0x141834d18> as *const ()), // sub_1400760c0
    Replacement::draft(0x140076120, "unknown_140076120", tpl_41::<0x143a1c4a0, 0x143a1c4a3, 0x1430789e0, 0x142409b50, 0x1425e30a0, 0x141834d18> as *const ()), // sub_140076120
    Replacement::draft(0x140076180, "unknown_140076180", tpl_34::<0x143a1c738, 0x143a1c73b, 0x1430789f0, 0x142409b50, 0x1425e30b0, 0x141834d18> as *const ()), // sub_140076180
    Replacement::draft(0x1400761e0, "unknown_1400761e0", tpl_48::<0x143a1c3d8, 0x143a1c3db, 0x143078a00, 0x142409b50, 0x1425e30c0, 0x141834d18> as *const ()), // sub_1400761e0
    Replacement::draft(0x140076240, "unknown_140076240", tpl_46::<0x143a1c850, 0x143a1c853, 0x143078a18, 0x142409b50, 0x1425e30d0, 0x141834d18> as *const ()), // sub_140076240
    Replacement::draft(0x1400762a0, "unknown_1400762a0", tpl_47::<0x143a1c7b0, 0x143a1c7b3, 0x143078a30, 0x142409b50, 0x1425e30e0, 0x141834d18> as *const ()), // sub_1400762a0
    Replacement::draft(0x140076300, "unknown_140076300", tpl_37::<0x143a1c310, 0x143a1c313, 0x143078a50, 0x142409b50, 0x1425e30f0, 0x141834d18> as *const ()), // sub_140076300
    Replacement::draft(0x140076360, "unknown_140076360", tpl_45::<0x143a1c568, 0x143a1c56b, 0x143078a60, 0x142409b50, 0x1425e3100, 0x141834d18> as *const ()), // sub_140076360
    Replacement::draft(0x1400763c0, "unknown_1400763c0", tpl_54::<0x143a1c670, 0x143a1c673, 0x143078a70, 0x142409b50, 0x1425e3110, 0x141834d18> as *const ()), // sub_1400763c0
    Replacement::draft(0x140076420, "unknown_140076420", tpl_40::<0x143a1c2e8, 0x143a1c2eb, 0x143078a88, 0x142409b50, 0x1425e3120, 0x141834d18> as *const ()), // sub_140076420
    Replacement::draft(0x140076480, "unknown_140076480", tpl_50::<0x143a1c338, 0x143a1c33b, 0x143078a90, 0x142409b50, 0x1425e3130, 0x141834d18> as *const ()), // sub_140076480
    Replacement::draft(0x1400764e0, "unknown_1400764e0", tpl_41::<0x143a1c4f0, 0x143a1c4f3, 0x143078aa8, 0x142409b50, 0x1425e3140, 0x141834d18> as *const ()), // sub_1400764e0
    Replacement::draft(0x140076540, "unknown_140076540", tpl_55::<0x143a1c7d8, 0x143a1c7db, 0x143078ab8, 0x142409b50, 0x1425e3150, 0x141834d18> as *const ()), // sub_140076540
    Replacement::draft(0x1400765a0, "unknown_1400765a0", tpl_41::<0x143a1c878, 0x143a1c87b, 0x143078ac8, 0x142409b50, 0x1425e3160, 0x141834d18> as *const ()), // sub_1400765a0
    Replacement::draft(0x140076600, "unknown_140076600", tpl_49::<0x143a1c918, 0x143a1c91b, 0x143078ad8, 0x142409b50, 0x1425e3170, 0x141834d18> as *const ()), // sub_140076600
    Replacement::draft(0x140076660, "unknown_140076660", tpl_40::<0x143a1c138, 0x143a1c13b, 0x143078ae4, 0x142409b50, 0x1425e3180, 0x141834d18> as *const ()), // sub_140076660
    Replacement::draft(0x1400766c0, "unknown_1400766c0", tpl_44::<0x143a1c270, 0x143a1c273, 0x143078af0, 0x142409b50, 0x1425e3190, 0x141834d18> as *const ()), // sub_1400766c0
    Replacement::draft(0x140076720, "unknown_140076720", tpl_46::<0x143a1c110, 0x143a1c113, 0x143078b08, 0x142409b50, 0x1425e31a0, 0x141834d18> as *const ()), // sub_140076720
    Replacement::draft(0x140076780, "unknown_140076780", tpl_55::<0x143a1c800, 0x143a1c803, 0x143078b20, 0x142409b50, 0x1425e31b0, 0x141834d18> as *const ()), // sub_140076780
    Replacement::draft(0x1400767e0, "unknown_1400767e0", tpl_46::<0x143a1c698, 0x143a1c69b, 0x143078b30, 0x142409b50, 0x1425e31c0, 0x141834d18> as *const ()), // sub_1400767e0
    Replacement::draft(0x140076840, "unknown_140076840", tpl_50::<0x143a1c400, 0x143a1c403, 0x143078b48, 0x142409b50, 0x1425e31d0, 0x141834d18> as *const ()), // sub_140076840
    Replacement::draft(0x1400768a0, "unknown_1400768a0", tpl_42::<0x143a1c0c0, 0x143a1c0c3, 0x143078b60, 0x142409b50, 0x1425e31e0, 0x141834d18> as *const ()), // sub_1400768a0
    Replacement::draft(0x140076900, "unknown_140076900", tpl_38::<0x143a1c760, 0x143a1c763, 0x143078b78, 0x142409b50, 0x1425e31f0, 0x141834d18> as *const ()), // sub_140076900
    Replacement::draft(0x140076960, "unknown_140076960", tpl_34::<0x143a1cad0, 0x143a1cad3, 0x143078b88, 0x142409b50, 0x1425e3200, 0x141834d18> as *const ()), // sub_140076960
    Replacement::draft(0x1400769c0, "unknown_1400769c0", tpl_41::<0x143a1c8c8, 0x143a1c8cb, 0x143078b98, 0x142409b50, 0x1425e3210, 0x141834d18> as *const ()), // sub_1400769c0
    Replacement::draft(0x140076a20, "unknown_140076a20", tpl_36::<0x143a1c648, 0x143a1c64b, 0x143078ba8, 0x142409b50, 0x1425e3220, 0x141834d18> as *const ()), // sub_140076a20
    Replacement::draft(0x140076a80, "unknown_140076a80", tpl_34::<0x143a1c0e8, 0x143a1c0eb, 0x143078bb8, 0x142409b50, 0x1425e3230, 0x141834d18> as *const ()), // sub_140076a80
    Replacement::draft(0x140076b50, "unknown_140076b50", fn_140076b50 as *const ()), // sub_140076b50
    Replacement::draft(0x140076e20, "unknown_140076e20", fn_140076e20 as *const ()), // sub_140076e20
    Replacement::draft(0x140077b80, "unknown_140077b80", tpl_23::<0x143a1cb20, 0x143a1cb23, 0x1426cdcb0, 0x142409b50, 0x1425e3340, 0x141834d18> as *const ()), // sub_140077b80
    Replacement::draft(0x140077be0, "unknown_140077be0", tpl_23::<0x143a1cb68, 0x143a1cb6b, 0x1426cdcb0, 0x142409b50, 0x1425e3350, 0x141834d18> as *const ()), // sub_140077be0
    Replacement::draft(0x140077dd0, "unknown_140077dd0", tpl_61::<0x143a1ccb0, 0x143a1ccb3, 0x143091fd8, 0x142409b50, 0x1425e33e0, 0x141834d18> as *const ()), // sub_140077dd0
    Replacement::draft(0x140077e30, "unknown_140077e30", tpl_62::<0x143a1ccb3, 0x143a1ccb0, 0x143a1ccb8, 0x143a1ccb4, 0x143a1cca8> as *const ()), // sub_140077e30
    Replacement::draft(0x140078170, "unknown_140078170", tpl_61::<0x143a1cd80, 0x143a1cd83, 0x143091fd8, 0x142409b50, 0x1425e3450, 0x141834d18> as *const ()), // sub_140078170
    Replacement::draft(0x1400781d0, "unknown_1400781d0", tpl_62::<0x143a1cd83, 0x143a1cd80, 0x143a1cd88, 0x143a1cd84, 0x143a1cd30> as *const ()), // sub_1400781d0
    Replacement::draft(0x140079070, "unknown_140079070", tpl_23::<0x143a1cf80, 0x143a1cf83, 0x1426cdcb0, 0x142409b50, 0x1425e34f0, 0x141834d18> as *const ()), // sub_140079070
    Replacement::draft(0x1400790d0, "unknown_1400790d0", tpl_23::<0x143a1cfc8, 0x143a1cfcb, 0x1426cdcb0, 0x142409b50, 0x1425e3500, 0x141834d18> as *const ()), // sub_1400790d0
    Replacement::draft(0x1400792c0, "unknown_1400792c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847c50, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e3540, 0x141834d18> as *const ()), // sub_1400792c0
    Replacement::draft(0x140079650, "unknown_140079650", tpl_34::<0x143a1d130, 0x143a1d133, 0x143078668, 0x142409b50, 0x1425e3580, 0x141834d18> as *const ()), // sub_140079650
    Replacement::draft(0x1400796b0, "unknown_1400796b0", tpl_23::<0x143a1d6f8, 0x143a1d6fb, 0x1426cdcb0, 0x142409b50, 0x1425e3590, 0x141834d18> as *const ()), // sub_1400796b0
    Replacement::draft(0x140079710, "unknown_140079710", tpl_35::<0x143a1d428, 0x143a1d42b, 0x143078678, 0x142409b50, 0x1425e35a0, 0x141834d18> as *const ()), // sub_140079710
    Replacement::draft(0x140079770, "unknown_140079770", tpl_36::<0x143a1d680, 0x143a1d683, 0x143078680, 0x142409b50, 0x1425e35b0, 0x141834d18> as *const ()), // sub_140079770
    Replacement::draft(0x1400797d0, "unknown_1400797d0", tpl_37::<0x143a1d310, 0x143a1d313, 0x143078690, 0x142409b50, 0x1425e35c0, 0x141834d18> as *const ()), // sub_1400797d0
    Replacement::draft(0x140079830, "unknown_140079830", tpl_38::<0x143a1d1d0, 0x143a1d1d3, 0x1430786a0, 0x142409b50, 0x1425e35d0, 0x141834d18> as *const ()), // sub_140079830
    Replacement::draft(0x140079890, "unknown_140079890", tpl_39::<0x143a1d720, 0x143a1d723, 0x1430786b0, 0x142409b50, 0x1425e35e0, 0x141834d18> as *const ()), // sub_140079890
    Replacement::draft(0x1400798f0, "unknown_1400798f0", tpl_37::<0x143a1d0e0, 0x143a1d0e3, 0x1430786b8, 0x142409b50, 0x1425e35f0, 0x141834d18> as *const ()), // sub_1400798f0
    Replacement::draft(0x140079950, "unknown_140079950", tpl_40::<0x143a1d838, 0x143a1d83b, 0x1430786c4, 0x142409b50, 0x1425e3600, 0x141834d18> as *const ()), // sub_140079950
    Replacement::draft(0x1400799b0, "unknown_1400799b0", tpl_41::<0x143a1d298, 0x143a1d29b, 0x1430786d0, 0x142409b50, 0x1425e3610, 0x141834d18> as *const ()), // sub_1400799b0
    Replacement::draft(0x140079a10, "unknown_140079a10", tpl_42::<0x143a1d770, 0x143a1d773, 0x1430786e0, 0x142409b50, 0x1425e3620, 0x141834d18> as *const ()), // sub_140079a10
    Replacement::draft(0x140079a70, "unknown_140079a70", tpl_34::<0x143a1d7c0, 0x143a1d7c3, 0x1430786f8, 0x142409b50, 0x1425e3630, 0x141834d18> as *const ()), // sub_140079a70
    Replacement::draft(0x140079ad0, "unknown_140079ad0", tpl_34::<0x143a1d108, 0x143a1d10b, 0x143078708, 0x142409b50, 0x1425e3640, 0x141834d18> as *const ()), // sub_140079ad0
    Replacement::draft(0x140079b30, "unknown_140079b30", tpl_41::<0x143a1d1f8, 0x143a1d1fb, 0x143078718, 0x142409b50, 0x1425e3650, 0x141834d18> as *const ()), // sub_140079b30
    Replacement::draft(0x140079b90, "unknown_140079b90", tpl_43::<0x143a1d338, 0x143a1d33b, 0x143078728, 0x142409b50, 0x1425e3660, 0x141834d18> as *const ()), // sub_140079b90
    Replacement::draft(0x140079bf0, "unknown_140079bf0", tpl_44::<0x143a1cdf0, 0x143a1cdf3, 0x143078748, 0x142409b50, 0x1425e3670, 0x141834d18> as *const ()), // sub_140079bf0
    Replacement::draft(0x140079c50, "unknown_140079c50", tpl_39::<0x143a1d2c0, 0x143a1d2c3, 0x143078760, 0x142409b50, 0x1425e3680, 0x141834d18> as *const ()), // sub_140079c50
    Replacement::draft(0x140079cb0, "unknown_140079cb0", tpl_45::<0x143a1d1a8, 0x143a1d1ab, 0x143078768, 0x142409b50, 0x1425e3690, 0x141834d18> as *const ()), // sub_140079cb0
    Replacement::draft(0x140079d10, "unknown_140079d10", tpl_41::<0x143a1d630, 0x143a1d633, 0x143078778, 0x142409b50, 0x1425e36a0, 0x141834d18> as *const ()), // sub_140079d10
    Replacement::draft(0x140079d70, "unknown_140079d70", tpl_37::<0x143a1ce18, 0x143a1ce1b, 0x143078788, 0x142409b50, 0x1425e36b0, 0x141834d18> as *const ()), // sub_140079d70
    Replacement::draft(0x140079dd0, "unknown_140079dd0", tpl_46::<0x143a1d4c8, 0x143a1d4cb, 0x143078798, 0x142409b50, 0x1425e36c0, 0x141834d18> as *const ()), // sub_140079dd0
    Replacement::draft(0x140079e30, "unknown_140079e30", tpl_47::<0x143a1d040, 0x143a1d043, 0x1430787b0, 0x142409b50, 0x1425e36d0, 0x141834d18> as *const ()), // sub_140079e30
    Replacement::draft(0x140079e90, "unknown_140079e90", tpl_38::<0x143a1d568, 0x143a1d56b, 0x1430787d0, 0x142409b50, 0x1425e36e0, 0x141834d18> as *const ()), // sub_140079e90
    Replacement::draft(0x140079ef0, "unknown_140079ef0", tpl_48::<0x143a1d748, 0x143a1d74b, 0x1430787e0, 0x142409b50, 0x1425e36f0, 0x141834d18> as *const ()), // sub_140079ef0
    Replacement::draft(0x140079f50, "unknown_140079f50", tpl_49::<0x143a1d400, 0x143a1d403, 0x1430787f8, 0x142409b50, 0x1425e3700, 0x141834d18> as *const ()), // sub_140079f50
    Replacement::draft(0x140079fb0, "unknown_140079fb0", tpl_44::<0x143a1d6d0, 0x143a1d6d3, 0x143078808, 0x142409b50, 0x1425e3710, 0x141834d18> as *const ()), // sub_140079fb0
    Replacement::draft(0x14007a010, "unknown_14007a010", tpl_42::<0x143a1cdc8, 0x143a1cdcb, 0x143078820, 0x142409b50, 0x1425e3720, 0x141834d18> as *const ()), // sub_14007a010
    Replacement::draft(0x14007a070, "unknown_14007a070", tpl_50::<0x143a1d5e0, 0x143a1d5e3, 0x143078838, 0x142409b50, 0x1425e3730, 0x141834d18> as *const ()), // sub_14007a070
    Replacement::draft(0x14007a0d0, "unknown_14007a0d0", tpl_52::<0x143a1cf58, 0x143a1cf5b, 0x143a1d838, 0x143a1d83c, 0x143a1d83b, 0x143a1d840, 0x142409b50, 0x1425e3740, 0x141834d18> as *const ()), // sub_14007a0d0
    Replacement::draft(0x14007a190, "unknown_14007a190", tpl_38::<0x143a1d450, 0x143a1d453, 0x143078950, 0x142409b50, 0x1425e3750, 0x141834d18> as *const ()), // sub_14007a190
    Replacement::draft(0x14007a1f0, "unknown_14007a1f0", tpl_53::<0x143a1d018, 0x143a1d01b, 0x143078960, 0x142409b50, 0x1425e3760, 0x141834d18> as *const ()), // sub_14007a1f0
    Replacement::draft(0x14007a250, "unknown_14007a250", tpl_45::<0x143a1cee0, 0x143a1cee3, 0x143078980, 0x142409b50, 0x1425e3770, 0x141834d18> as *const ()), // sub_14007a250
    Replacement::draft(0x14007a2b0, "unknown_14007a2b0", tpl_35::<0x143a1cf30, 0x143a1cf33, 0x14307898c, 0x142409b50, 0x1425e3780, 0x141834d18> as *const ()), // sub_14007a2b0
    Replacement::draft(0x14007a310, "unknown_14007a310", tpl_42::<0x143a1d798, 0x143a1d79b, 0x1430786e0, 0x142409b50, 0x1425e3790, 0x141834d18> as *const ()), // sub_14007a310
    Replacement::draft(0x14007a370, "unknown_14007a370", tpl_38::<0x143a1d6a8, 0x143a1d6ab, 0x143078998, 0x142409b50, 0x1425e37a0, 0x141834d18> as *const ()), // sub_14007a370
    Replacement::draft(0x14007a3d0, "unknown_14007a3d0", tpl_38::<0x143a1d248, 0x143a1d24b, 0x1430789a8, 0x142409b50, 0x1425e37b0, 0x141834d18> as *const ()), // sub_14007a3d0
    Replacement::draft(0x14007a430, "unknown_14007a430", tpl_41::<0x143a1d360, 0x143a1d363, 0x1430789b8, 0x142409b50, 0x1425e37c0, 0x141834d18> as *const ()), // sub_14007a430
    Replacement::draft(0x14007a490, "unknown_14007a490", tpl_34::<0x143a1d7e8, 0x143a1d7eb, 0x1430789c8, 0x142409b50, 0x1425e37d0, 0x141834d18> as *const ()), // sub_14007a490
    Replacement::draft(0x14007a4f0, "unknown_14007a4f0", tpl_39::<0x143a1cf08, 0x143a1cf0b, 0x1430789d8, 0x142409b50, 0x1425e37e0, 0x141834d18> as *const ()), // sub_14007a4f0
    Replacement::draft(0x14007a550, "unknown_14007a550", tpl_41::<0x143a1d220, 0x143a1d223, 0x1430789e0, 0x142409b50, 0x1425e37f0, 0x141834d18> as *const ()), // sub_14007a550
    Replacement::draft(0x14007a5b0, "unknown_14007a5b0", tpl_34::<0x143a1d478, 0x143a1d47b, 0x1430789f0, 0x142409b50, 0x1425e3800, 0x141834d18> as *const ()), // sub_14007a5b0
    Replacement::draft(0x14007a610, "unknown_14007a610", tpl_48::<0x143a1d158, 0x143a1d15b, 0x143078a00, 0x142409b50, 0x1425e3810, 0x141834d18> as *const ()), // sub_14007a610
    Replacement::draft(0x14007a670, "unknown_14007a670", tpl_46::<0x143a1d590, 0x143a1d593, 0x143078a18, 0x142409b50, 0x1425e3820, 0x141834d18> as *const ()), // sub_14007a670
    Replacement::draft(0x14007a6d0, "unknown_14007a6d0", tpl_47::<0x143a1d4f0, 0x143a1d4f3, 0x143078a30, 0x142409b50, 0x1425e3830, 0x141834d18> as *const ()), // sub_14007a6d0
    Replacement::draft(0x14007a730, "unknown_14007a730", tpl_37::<0x143a1d090, 0x143a1d093, 0x143078a50, 0x142409b50, 0x1425e3840, 0x141834d18> as *const ()), // sub_14007a730
    Replacement::draft(0x14007a790, "unknown_14007a790", tpl_45::<0x143a1d2e8, 0x143a1d2eb, 0x143078a60, 0x142409b50, 0x1425e3850, 0x141834d18> as *const ()), // sub_14007a790
    Replacement::draft(0x14007a7f0, "unknown_14007a7f0", tpl_54::<0x143a1d3b0, 0x143a1d3b3, 0x143078a70, 0x142409b50, 0x1425e3860, 0x141834d18> as *const ()), // sub_14007a7f0
    Replacement::draft(0x14007a850, "unknown_14007a850", tpl_40::<0x143a1d068, 0x143a1d06b, 0x143078a88, 0x142409b50, 0x1425e3870, 0x141834d18> as *const ()), // sub_14007a850
    Replacement::draft(0x14007a8b0, "unknown_14007a8b0", tpl_50::<0x143a1d0b8, 0x143a1d0bb, 0x143078a90, 0x142409b50, 0x1425e3880, 0x141834d18> as *const ()), // sub_14007a8b0
    Replacement::draft(0x14007a910, "unknown_14007a910", tpl_41::<0x143a1d270, 0x143a1d273, 0x143078aa8, 0x142409b50, 0x1425e3890, 0x141834d18> as *const ()), // sub_14007a910
    Replacement::draft(0x14007a970, "unknown_14007a970", tpl_55::<0x143a1d518, 0x143a1d51b, 0x143078ab8, 0x142409b50, 0x1425e38a0, 0x141834d18> as *const ()), // sub_14007a970
    Replacement::draft(0x14007a9d0, "unknown_14007a9d0", tpl_41::<0x143a1d5b8, 0x143a1d5bb, 0x143078ac8, 0x142409b50, 0x1425e38b0, 0x141834d18> as *const ()), // sub_14007a9d0
    Replacement::draft(0x14007aa30, "unknown_14007aa30", tpl_49::<0x143a1d658, 0x143a1d65b, 0x143078ad8, 0x142409b50, 0x1425e38c0, 0x141834d18> as *const ()), // sub_14007aa30
    Replacement::draft(0x14007aa90, "unknown_14007aa90", tpl_40::<0x143a1ceb8, 0x143a1cebb, 0x143078ae4, 0x142409b50, 0x1425e38d0, 0x141834d18> as *const ()), // sub_14007aa90
    Replacement::draft(0x14007aaf0, "unknown_14007aaf0", tpl_44::<0x143a1cff0, 0x143a1cff3, 0x143078af0, 0x142409b50, 0x1425e38e0, 0x141834d18> as *const ()), // sub_14007aaf0
    Replacement::draft(0x14007ab50, "unknown_14007ab50", tpl_46::<0x143a1ce90, 0x143a1ce93, 0x143078b08, 0x142409b50, 0x1425e38f0, 0x141834d18> as *const ()), // sub_14007ab50
    Replacement::draft(0x14007abb0, "unknown_14007abb0", tpl_55::<0x143a1d540, 0x143a1d543, 0x143078b20, 0x142409b50, 0x1425e3900, 0x141834d18> as *const ()), // sub_14007abb0
    Replacement::draft(0x14007ac10, "unknown_14007ac10", tpl_46::<0x143a1d3d8, 0x143a1d3db, 0x143078b30, 0x142409b50, 0x1425e3910, 0x141834d18> as *const ()), // sub_14007ac10
    Replacement::draft(0x14007ac70, "unknown_14007ac70", tpl_50::<0x143a1d180, 0x143a1d183, 0x143078b48, 0x142409b50, 0x1425e3920, 0x141834d18> as *const ()), // sub_14007ac70
    Replacement::draft(0x14007acd0, "unknown_14007acd0", tpl_42::<0x143a1ce40, 0x143a1ce43, 0x143078b60, 0x142409b50, 0x1425e3930, 0x141834d18> as *const ()), // sub_14007acd0
    Replacement::draft(0x14007ad30, "unknown_14007ad30", tpl_38::<0x143a1d4a0, 0x143a1d4a3, 0x143078b78, 0x142409b50, 0x1425e3940, 0x141834d18> as *const ()), // sub_14007ad30
    Replacement::draft(0x14007ad90, "unknown_14007ad90", tpl_34::<0x143a1d810, 0x143a1d813, 0x143078b88, 0x142409b50, 0x1425e3950, 0x141834d18> as *const ()), // sub_14007ad90
    Replacement::draft(0x14007adf0, "unknown_14007adf0", tpl_41::<0x143a1d608, 0x143a1d60b, 0x143078b98, 0x142409b50, 0x1425e3960, 0x141834d18> as *const ()), // sub_14007adf0
    Replacement::draft(0x14007ae50, "unknown_14007ae50", tpl_36::<0x143a1d388, 0x143a1d38b, 0x143078ba8, 0x142409b50, 0x1425e3970, 0x141834d18> as *const ()), // sub_14007ae50
    Replacement::draft(0x14007aeb0, "unknown_14007aeb0", tpl_34::<0x143a1ce68, 0x143a1ce6b, 0x143078bb8, 0x142409b50, 0x1425e3980, 0x141834d18> as *const ()), // sub_14007aeb0
    Replacement::draft(0x14007be20, "unknown_14007be20", tpl_34::<0x143a1dbc8, 0x143a1dbcb, 0x143078668, 0x142409b50, 0x1425e3c10, 0x141834d18> as *const ()), // sub_14007be20
    Replacement::draft(0x14007be80, "unknown_14007be80", tpl_23::<0x143a1e190, 0x143a1e193, 0x1426cdcb0, 0x142409b50, 0x1425e3c20, 0x141834d18> as *const ()), // sub_14007be80
    Replacement::draft(0x14007bee0, "unknown_14007bee0", tpl_35::<0x143a1dec0, 0x143a1dec3, 0x143078678, 0x142409b50, 0x1425e3c30, 0x141834d18> as *const ()), // sub_14007bee0
    Replacement::draft(0x14007bf40, "unknown_14007bf40", tpl_36::<0x143a1e118, 0x143a1e11b, 0x143078680, 0x142409b50, 0x1425e3c40, 0x141834d18> as *const ()), // sub_14007bf40
    Replacement::draft(0x14007bfa0, "unknown_14007bfa0", tpl_37::<0x143a1dda8, 0x143a1ddab, 0x143078690, 0x142409b50, 0x1425e3c50, 0x141834d18> as *const ()), // sub_14007bfa0
    Replacement::draft(0x14007c000, "unknown_14007c000", tpl_38::<0x143a1dc68, 0x143a1dc6b, 0x1430786a0, 0x142409b50, 0x1425e3c60, 0x141834d18> as *const ()), // sub_14007c000
    Replacement::draft(0x14007c060, "unknown_14007c060", tpl_39::<0x143a1e1b8, 0x143a1e1bb, 0x1430786b0, 0x142409b50, 0x1425e3c70, 0x141834d18> as *const ()), // sub_14007c060
    Replacement::draft(0x14007c0c0, "unknown_14007c0c0", tpl_37::<0x143a1db78, 0x143a1db7b, 0x1430786b8, 0x142409b50, 0x1425e3c80, 0x141834d18> as *const ()), // sub_14007c0c0
    Replacement::draft(0x14007c120, "unknown_14007c120", tpl_40::<0x143a1e2d0, 0x143a1e2d3, 0x1430786c4, 0x142409b50, 0x1425e3c90, 0x141834d18> as *const ()), // sub_14007c120
    Replacement::draft(0x14007c180, "unknown_14007c180", tpl_41::<0x143a1dd30, 0x143a1dd33, 0x1430786d0, 0x142409b50, 0x1425e3ca0, 0x141834d18> as *const ()), // sub_14007c180
    Replacement::draft(0x14007c1e0, "unknown_14007c1e0", tpl_42::<0x143a1e208, 0x143a1e20b, 0x1430786e0, 0x142409b50, 0x1425e3cb0, 0x141834d18> as *const ()), // sub_14007c1e0
    Replacement::draft(0x14007c240, "unknown_14007c240", tpl_34::<0x143a1e258, 0x143a1e25b, 0x1430786f8, 0x142409b50, 0x1425e3cc0, 0x141834d18> as *const ()), // sub_14007c240
    Replacement::draft(0x14007c2a0, "unknown_14007c2a0", tpl_34::<0x143a1dba0, 0x143a1dba3, 0x143078708, 0x142409b50, 0x1425e3cd0, 0x141834d18> as *const ()), // sub_14007c2a0
    Replacement::draft(0x14007c300, "unknown_14007c300", tpl_41::<0x143a1dc90, 0x143a1dc93, 0x143078718, 0x142409b50, 0x1425e3ce0, 0x141834d18> as *const ()), // sub_14007c300
    Replacement::draft(0x14007c360, "unknown_14007c360", tpl_43::<0x143a1ddd0, 0x143a1ddd3, 0x143078728, 0x142409b50, 0x1425e3cf0, 0x141834d18> as *const ()), // sub_14007c360
    Replacement::draft(0x14007c3c0, "unknown_14007c3c0", tpl_44::<0x143a1d888, 0x143a1d88b, 0x143078748, 0x142409b50, 0x1425e3d00, 0x141834d18> as *const ()), // sub_14007c3c0
    Replacement::draft(0x14007c420, "unknown_14007c420", tpl_39::<0x143a1dd58, 0x143a1dd5b, 0x143078760, 0x142409b50, 0x1425e3d10, 0x141834d18> as *const ()), // sub_14007c420
    Replacement::draft(0x14007c480, "unknown_14007c480", tpl_45::<0x143a1dc40, 0x143a1dc43, 0x143078768, 0x142409b50, 0x1425e3d20, 0x141834d18> as *const ()), // sub_14007c480
    Replacement::draft(0x14007c4e0, "unknown_14007c4e0", tpl_41::<0x143a1e0c8, 0x143a1e0cb, 0x143078778, 0x142409b50, 0x1425e3d30, 0x141834d18> as *const ()), // sub_14007c4e0
    Replacement::draft(0x14007c540, "unknown_14007c540", tpl_37::<0x143a1d8b0, 0x143a1d8b3, 0x143078788, 0x142409b50, 0x1425e3d40, 0x141834d18> as *const ()), // sub_14007c540
    Replacement::draft(0x14007c5a0, "unknown_14007c5a0", tpl_46::<0x143a1df60, 0x143a1df63, 0x143078798, 0x142409b50, 0x1425e3d50, 0x141834d18> as *const ()), // sub_14007c5a0
    Replacement::draft(0x14007c600, "unknown_14007c600", tpl_47::<0x143a1dad8, 0x143a1dadb, 0x1430787b0, 0x142409b50, 0x1425e3d60, 0x141834d18> as *const ()), // sub_14007c600
    Replacement::draft(0x14007c660, "unknown_14007c660", tpl_38::<0x143a1e000, 0x143a1e003, 0x1430787d0, 0x142409b50, 0x1425e3d70, 0x141834d18> as *const ()), // sub_14007c660
    Replacement::draft(0x14007c6c0, "unknown_14007c6c0", tpl_48::<0x143a1e1e0, 0x143a1e1e3, 0x1430787e0, 0x142409b50, 0x1425e3d80, 0x141834d18> as *const ()), // sub_14007c6c0
    Replacement::draft(0x14007c720, "unknown_14007c720", tpl_49::<0x143a1de98, 0x143a1de9b, 0x1430787f8, 0x142409b50, 0x1425e3d90, 0x141834d18> as *const ()), // sub_14007c720
    Replacement::draft(0x14007c780, "unknown_14007c780", tpl_44::<0x143a1e168, 0x143a1e16b, 0x143078808, 0x142409b50, 0x1425e3da0, 0x141834d18> as *const ()), // sub_14007c780
    Replacement::draft(0x14007c7e0, "unknown_14007c7e0", tpl_42::<0x143a1d860, 0x143a1d863, 0x143078820, 0x142409b50, 0x1425e3db0, 0x141834d18> as *const ()), // sub_14007c7e0
    Replacement::draft(0x14007c840, "unknown_14007c840", tpl_50::<0x143a1e078, 0x143a1e07b, 0x143078838, 0x142409b50, 0x1425e3dc0, 0x141834d18> as *const ()), // sub_14007c840
    Replacement::draft(0x14007c8a0, "unknown_14007c8a0", tpl_23::<0x143a1da18, 0x143a1da1b, 0x1426cdcb0, 0x142409b50, 0x1425e3dd0, 0x141834d18> as *const ()), // sub_14007c8a0
    Replacement::draft(0x14007c900, "unknown_14007c900", tpl_23::<0x143a1da60, 0x143a1da63, 0x1426cdcb0, 0x142409b50, 0x1425e3de0, 0x141834d18> as *const ()), // sub_14007c900
    Replacement::draft(0x14007caf0, "unknown_14007caf0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847d40, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e3e20, 0x141834d18> as *const ()), // sub_14007caf0
    Replacement::draft(0x14007ce80, "unknown_14007ce80", tpl_52::<0x143a1d9f0, 0x143a1d9f3, 0x143a1e2d0, 0x143a1e2d4, 0x143a1e2d3, 0x143a1e2d8, 0x142409b50, 0x1425e3e60, 0x141834d18> as *const ()), // sub_14007ce80
    Replacement::draft(0x14007cf40, "unknown_14007cf40", tpl_38::<0x143a1dee8, 0x143a1deeb, 0x143078950, 0x142409b50, 0x1425e3e70, 0x141834d18> as *const ()), // sub_14007cf40
    Replacement::draft(0x14007cfa0, "unknown_14007cfa0", tpl_53::<0x143a1dab0, 0x143a1dab3, 0x143078960, 0x142409b50, 0x1425e3e80, 0x141834d18> as *const ()), // sub_14007cfa0
    Replacement::draft(0x14007d000, "unknown_14007d000", tpl_45::<0x143a1d978, 0x143a1d97b, 0x143078980, 0x142409b50, 0x1425e3e90, 0x141834d18> as *const ()), // sub_14007d000
    Replacement::draft(0x14007d060, "unknown_14007d060", tpl_35::<0x143a1d9c8, 0x143a1d9cb, 0x14307898c, 0x142409b50, 0x1425e3ea0, 0x141834d18> as *const ()), // sub_14007d060
    Replacement::draft(0x14007d0c0, "unknown_14007d0c0", tpl_42::<0x143a1e230, 0x143a1e233, 0x1430786e0, 0x142409b50, 0x1425e3eb0, 0x141834d18> as *const ()), // sub_14007d0c0
    Replacement::draft(0x14007d120, "unknown_14007d120", tpl_38::<0x143a1e140, 0x143a1e143, 0x143078998, 0x142409b50, 0x1425e3ec0, 0x141834d18> as *const ()), // sub_14007d120
    Replacement::draft(0x14007d180, "unknown_14007d180", tpl_38::<0x143a1dce0, 0x143a1dce3, 0x1430789a8, 0x142409b50, 0x1425e3ed0, 0x141834d18> as *const ()), // sub_14007d180
    Replacement::draft(0x14007d1e0, "unknown_14007d1e0", tpl_41::<0x143a1ddf8, 0x143a1ddfb, 0x1430789b8, 0x142409b50, 0x1425e3ee0, 0x141834d18> as *const ()), // sub_14007d1e0
    Replacement::draft(0x14007d240, "unknown_14007d240", tpl_34::<0x143a1e280, 0x143a1e283, 0x1430789c8, 0x142409b50, 0x1425e3ef0, 0x141834d18> as *const ()), // sub_14007d240
    Replacement::draft(0x14007d2a0, "unknown_14007d2a0", tpl_39::<0x143a1d9a0, 0x143a1d9a3, 0x1430789d8, 0x142409b50, 0x1425e3f00, 0x141834d18> as *const ()), // sub_14007d2a0
    Replacement::draft(0x14007d300, "unknown_14007d300", tpl_41::<0x143a1dcb8, 0x143a1dcbb, 0x1430789e0, 0x142409b50, 0x1425e3f10, 0x141834d18> as *const ()), // sub_14007d300
    Replacement::draft(0x14007d360, "unknown_14007d360", tpl_34::<0x143a1df10, 0x143a1df13, 0x1430789f0, 0x142409b50, 0x1425e3f20, 0x141834d18> as *const ()), // sub_14007d360
    Replacement::draft(0x14007d3c0, "unknown_14007d3c0", tpl_48::<0x143a1dbf0, 0x143a1dbf3, 0x143078a00, 0x142409b50, 0x1425e3f30, 0x141834d18> as *const ()), // sub_14007d3c0
    Replacement::draft(0x14007d420, "unknown_14007d420", tpl_46::<0x143a1e028, 0x143a1e02b, 0x143078a18, 0x142409b50, 0x1425e3f40, 0x141834d18> as *const ()), // sub_14007d420
    Replacement::draft(0x14007d480, "unknown_14007d480", tpl_47::<0x143a1df88, 0x143a1df8b, 0x143078a30, 0x142409b50, 0x1425e3f50, 0x141834d18> as *const ()), // sub_14007d480
    Replacement::draft(0x14007d4e0, "unknown_14007d4e0", tpl_37::<0x143a1db28, 0x143a1db2b, 0x143078a50, 0x142409b50, 0x1425e3f60, 0x141834d18> as *const ()), // sub_14007d4e0
    Replacement::draft(0x14007d540, "unknown_14007d540", tpl_45::<0x143a1dd80, 0x143a1dd83, 0x143078a60, 0x142409b50, 0x1425e3f70, 0x141834d18> as *const ()), // sub_14007d540
    Replacement::draft(0x14007d5a0, "unknown_14007d5a0", tpl_54::<0x143a1de48, 0x143a1de4b, 0x143078a70, 0x142409b50, 0x1425e3f80, 0x141834d18> as *const ()), // sub_14007d5a0
    Replacement::draft(0x14007d600, "unknown_14007d600", tpl_40::<0x143a1db00, 0x143a1db03, 0x143078a88, 0x142409b50, 0x1425e3f90, 0x141834d18> as *const ()), // sub_14007d600
    Replacement::draft(0x14007d660, "unknown_14007d660", tpl_50::<0x143a1db50, 0x143a1db53, 0x143078a90, 0x142409b50, 0x1425e3fa0, 0x141834d18> as *const ()), // sub_14007d660
    Replacement::draft(0x14007d6c0, "unknown_14007d6c0", tpl_41::<0x143a1dd08, 0x143a1dd0b, 0x143078aa8, 0x142409b50, 0x1425e3fb0, 0x141834d18> as *const ()), // sub_14007d6c0
    Replacement::draft(0x14007d720, "unknown_14007d720", tpl_55::<0x143a1dfb0, 0x143a1dfb3, 0x143078ab8, 0x142409b50, 0x1425e3fc0, 0x141834d18> as *const ()), // sub_14007d720
    Replacement::draft(0x14007d780, "unknown_14007d780", tpl_41::<0x143a1e050, 0x143a1e053, 0x143078ac8, 0x142409b50, 0x1425e3fd0, 0x141834d18> as *const ()), // sub_14007d780
    Replacement::draft(0x14007d7e0, "unknown_14007d7e0", tpl_49::<0x143a1e0f0, 0x143a1e0f3, 0x143078ad8, 0x142409b50, 0x1425e3fe0, 0x141834d18> as *const ()), // sub_14007d7e0
    Replacement::draft(0x14007d840, "unknown_14007d840", tpl_40::<0x143a1d950, 0x143a1d953, 0x143078ae4, 0x142409b50, 0x1425e3ff0, 0x141834d18> as *const ()), // sub_14007d840
    Replacement::draft(0x14007d8a0, "unknown_14007d8a0", tpl_44::<0x143a1da88, 0x143a1da8b, 0x143078af0, 0x142409b50, 0x1425e4000, 0x141834d18> as *const ()), // sub_14007d8a0
    Replacement::draft(0x14007d900, "unknown_14007d900", tpl_46::<0x143a1d928, 0x143a1d92b, 0x143078b08, 0x142409b50, 0x1425e4010, 0x141834d18> as *const ()), // sub_14007d900
    Replacement::draft(0x14007d960, "unknown_14007d960", tpl_55::<0x143a1dfd8, 0x143a1dfdb, 0x143078b20, 0x142409b50, 0x1425e4020, 0x141834d18> as *const ()), // sub_14007d960
    Replacement::draft(0x14007d9c0, "unknown_14007d9c0", tpl_46::<0x143a1de70, 0x143a1de73, 0x143078b30, 0x142409b50, 0x1425e4030, 0x141834d18> as *const ()), // sub_14007d9c0
    Replacement::draft(0x14007da20, "unknown_14007da20", tpl_50::<0x143a1dc18, 0x143a1dc1b, 0x143078b48, 0x142409b50, 0x1425e4040, 0x141834d18> as *const ()), // sub_14007da20
    Replacement::draft(0x14007da80, "unknown_14007da80", tpl_42::<0x143a1d8d8, 0x143a1d8db, 0x143078b60, 0x142409b50, 0x1425e4050, 0x141834d18> as *const ()), // sub_14007da80
    Replacement::draft(0x14007dae0, "unknown_14007dae0", tpl_38::<0x143a1df38, 0x143a1df3b, 0x143078b78, 0x142409b50, 0x1425e4060, 0x141834d18> as *const ()), // sub_14007dae0
    Replacement::draft(0x14007db40, "unknown_14007db40", tpl_34::<0x143a1e2a8, 0x143a1e2ab, 0x143078b88, 0x142409b50, 0x1425e4070, 0x141834d18> as *const ()), // sub_14007db40
    Replacement::draft(0x14007dba0, "unknown_14007dba0", tpl_41::<0x143a1e0a0, 0x143a1e0a3, 0x143078b98, 0x142409b50, 0x1425e4080, 0x141834d18> as *const ()), // sub_14007dba0
    Replacement::draft(0x14007dc00, "unknown_14007dc00", tpl_36::<0x143a1de20, 0x143a1de23, 0x143078ba8, 0x142409b50, 0x1425e4090, 0x141834d18> as *const ()), // sub_14007dc00
    Replacement::draft(0x14007dc60, "unknown_14007dc60", tpl_34::<0x143a1d900, 0x143a1d903, 0x143078bb8, 0x142409b50, 0x1425e40a0, 0x141834d18> as *const ()), // sub_14007dc60
    Replacement::draft(0x14007e930, "unknown_14007e930", tpl_61::<0x143a1e400, 0x143a1e403, 0x143091fd8, 0x142409b50, 0x1425e4110, 0x141834d18> as *const ()), // sub_14007e930
    Replacement::draft(0x14007e990, "unknown_14007e990", tpl_62::<0x143a1e403, 0x143a1e400, 0x143a1e408, 0x143a1e404, 0x143a1e2f8> as *const ()), // sub_14007e990
    Replacement::draft(0x14007ea20, "unknown_14007ea20", tpl_63::<0x143a1e3b0, 0x143a1e3b3, 0x143091ff0, 0x142409b50, 0x1425e4120, 0x141834d18> as *const ()), // sub_14007ea20
    Replacement::draft(0x14007ea80, "unknown_14007ea80", tpl_64::<0x143a1e360, 0x143a1e363, 0x143091fa0, 0x142409b50, 0x1425e4130, 0x141834d18> as *const ()), // sub_14007ea80
    Replacement::draft(0x14007eae0, "unknown_14007eae0", tpl_65::<0x143a1e300, 0x143a1e303, 0x143091fc0, 0x142409b50, 0x1425e4140, 0x141834d18> as *const ()), // sub_14007eae0
    Replacement::draft(0x14007eb40, "unknown_14007eb40", tpl_62::<0x143a1e3b3, 0x143a1e3b0, 0x143a1e3b8, 0x143a1e3b4, 0x143a1e3a8> as *const ()), // sub_14007eb40
    Replacement::draft(0x14007ebd0, "unknown_14007ebd0", tpl_62::<0x143a1e363, 0x143a1e360, 0x143a1e368, 0x143a1e364, 0x143a1e348> as *const ()), // sub_14007ebd0
    Replacement::draft(0x14007ec60, "unknown_14007ec60", tpl_62::<0x143a1e303, 0x143a1e300, 0x143a1e308, 0x143a1e304, 0x143a1e350> as *const ()), // sub_14007ec60
    Replacement::draft(0x14007f380, "unknown_14007f380", tpl_61::<0x143a1e5c0, 0x143a1e5c3, 0x143091fd8, 0x142409b50, 0x1425e4180, 0x141834d18> as *const ()), // sub_14007f380
    Replacement::draft(0x14007f3e0, "unknown_14007f3e0", tpl_62::<0x143a1e5c3, 0x143a1e5c0, 0x143a1e5c8, 0x143a1e5c4, 0x143a1e460> as *const ()), // sub_14007f3e0
    Replacement::draft(0x14007f470, "unknown_14007f470", tpl_63::<0x143a1e570, 0x143a1e573, 0x143091ff0, 0x142409b50, 0x1425e4190, 0x141834d18> as *const ()), // sub_14007f470
    Replacement::draft(0x14007f4d0, "unknown_14007f4d0", tpl_64::<0x143a1e4f0, 0x143a1e4f3, 0x143091fa0, 0x142409b50, 0x1425e41a0, 0x141834d18> as *const ()), // sub_14007f4d0
    Replacement::draft(0x14007f530, "unknown_14007f530", tpl_65::<0x143a1e470, 0x143a1e473, 0x143091fc0, 0x142409b50, 0x1425e41b0, 0x141834d18> as *const ()), // sub_14007f530
    Replacement::draft(0x14007f590, "unknown_14007f590", tpl_62::<0x143a1e573, 0x143a1e570, 0x143a1e578, 0x143a1e574, 0x143a1e560> as *const ()), // sub_14007f590
    Replacement::draft(0x14007f620, "unknown_14007f620", tpl_62::<0x143a1e4f3, 0x143a1e4f0, 0x143a1e4f8, 0x143a1e4f4, 0x143a1e4b8> as *const ()), // sub_14007f620
    Replacement::draft(0x14007f6b0, "unknown_14007f6b0", tpl_62::<0x143a1e473, 0x143a1e470, 0x143a1e478, 0x143a1e474, 0x143a1e4c0> as *const ()), // sub_14007f6b0
    Replacement::draft(0x1400804b0, "unknown_1400804b0", tpl_23::<0x143a1e608, 0x143a1e60b, 0x1426cdcb0, 0x142409b50, 0x1425e47a0, 0x141834d18> as *const ()), // sub_1400804b0
    Replacement::draft(0x140080510, "unknown_140080510", tpl_23::<0x143a1e650, 0x143a1e653, 0x1426cdcb0, 0x142409b50, 0x1425e47b0, 0x141834d18> as *const ()), // sub_140080510
    Replacement::draft(0x140080700, "unknown_140080700", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847f08, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e47f0, 0x141834d18> as *const ()), // sub_140080700
    Replacement::draft(0x140080ce0, "unknown_140080ce0", tpl_23::<0x143a1e878, 0x143a1e87b, 0x1426cdcb0, 0x142409b50, 0x1425e4870, 0x141834d18> as *const ()), // sub_140080ce0
    Replacement::draft(0x140080d40, "unknown_140080d40", tpl_23::<0x143a1e8c0, 0x143a1e8c3, 0x1426cdcb0, 0x142409b50, 0x1425e4880, 0x141834d18> as *const ()), // sub_140080d40
    Replacement::draft(0x140080f30, "unknown_140080f30", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143847f98, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e48c0, 0x141834d18> as *const ()), // sub_140080f30
    Replacement::draft(0x1400812c0, "unknown_1400812c0", tpl_34::<0x143a1ea28, 0x143a1ea2b, 0x143078668, 0x142409b50, 0x1425e4900, 0x141834d18> as *const ()), // sub_1400812c0
    Replacement::draft(0x140081320, "unknown_140081320", tpl_23::<0x143a1eff0, 0x143a1eff3, 0x1426cdcb0, 0x142409b50, 0x1425e4910, 0x141834d18> as *const ()), // sub_140081320
    Replacement::draft(0x140081380, "unknown_140081380", tpl_35::<0x143a1ed20, 0x143a1ed23, 0x143078678, 0x142409b50, 0x1425e4920, 0x141834d18> as *const ()), // sub_140081380
    Replacement::draft(0x1400813e0, "unknown_1400813e0", tpl_36::<0x143a1ef78, 0x143a1ef7b, 0x143078680, 0x142409b50, 0x1425e4930, 0x141834d18> as *const ()), // sub_1400813e0
    Replacement::draft(0x140081440, "unknown_140081440", tpl_37::<0x143a1ec08, 0x143a1ec0b, 0x143078690, 0x142409b50, 0x1425e4940, 0x141834d18> as *const ()), // sub_140081440
    Replacement::draft(0x1400814a0, "unknown_1400814a0", tpl_38::<0x143a1eac8, 0x143a1eacb, 0x1430786a0, 0x142409b50, 0x1425e4950, 0x141834d18> as *const ()), // sub_1400814a0
    Replacement::draft(0x140081500, "unknown_140081500", tpl_39::<0x143a1f018, 0x143a1f01b, 0x1430786b0, 0x142409b50, 0x1425e4960, 0x141834d18> as *const ()), // sub_140081500
    Replacement::draft(0x140081560, "unknown_140081560", tpl_37::<0x143a1e9d8, 0x143a1e9db, 0x1430786b8, 0x142409b50, 0x1425e4970, 0x141834d18> as *const ()), // sub_140081560
    Replacement::draft(0x1400815c0, "unknown_1400815c0", tpl_40::<0x143a1f130, 0x143a1f133, 0x1430786c4, 0x142409b50, 0x1425e4980, 0x141834d18> as *const ()), // sub_1400815c0
    Replacement::draft(0x140081620, "unknown_140081620", tpl_41::<0x143a1eb90, 0x143a1eb93, 0x1430786d0, 0x142409b50, 0x1425e4990, 0x141834d18> as *const ()), // sub_140081620
    Replacement::draft(0x140081680, "unknown_140081680", tpl_42::<0x143a1f068, 0x143a1f06b, 0x1430786e0, 0x142409b50, 0x1425e49a0, 0x141834d18> as *const ()), // sub_140081680
    Replacement::draft(0x1400816e0, "unknown_1400816e0", tpl_34::<0x143a1f0b8, 0x143a1f0bb, 0x1430786f8, 0x142409b50, 0x1425e49b0, 0x141834d18> as *const ()), // sub_1400816e0
    Replacement::draft(0x140081740, "unknown_140081740", tpl_34::<0x143a1ea00, 0x143a1ea03, 0x143078708, 0x142409b50, 0x1425e49c0, 0x141834d18> as *const ()), // sub_140081740
    Replacement::draft(0x1400817a0, "unknown_1400817a0", tpl_41::<0x143a1eaf0, 0x143a1eaf3, 0x143078718, 0x142409b50, 0x1425e49d0, 0x141834d18> as *const ()), // sub_1400817a0
    Replacement::draft(0x140081800, "unknown_140081800", tpl_43::<0x143a1ec30, 0x143a1ec33, 0x143078728, 0x142409b50, 0x1425e49e0, 0x141834d18> as *const ()), // sub_140081800
    Replacement::draft(0x140081860, "unknown_140081860", tpl_44::<0x143a1e6e8, 0x143a1e6eb, 0x143078748, 0x142409b50, 0x1425e49f0, 0x141834d18> as *const ()), // sub_140081860
    Replacement::draft(0x1400818c0, "unknown_1400818c0", tpl_39::<0x143a1ebb8, 0x143a1ebbb, 0x143078760, 0x142409b50, 0x1425e4a00, 0x141834d18> as *const ()), // sub_1400818c0
    Replacement::draft(0x140081920, "unknown_140081920", tpl_45::<0x143a1eaa0, 0x143a1eaa3, 0x143078768, 0x142409b50, 0x1425e4a10, 0x141834d18> as *const ()), // sub_140081920
    Replacement::draft(0x140081980, "unknown_140081980", tpl_41::<0x143a1ef28, 0x143a1ef2b, 0x143078778, 0x142409b50, 0x1425e4a20, 0x141834d18> as *const ()), // sub_140081980
    Replacement::draft(0x1400819e0, "unknown_1400819e0", tpl_37::<0x143a1e710, 0x143a1e713, 0x143078788, 0x142409b50, 0x1425e4a30, 0x141834d18> as *const ()), // sub_1400819e0
    Replacement::draft(0x140081a40, "unknown_140081a40", tpl_46::<0x143a1edc0, 0x143a1edc3, 0x143078798, 0x142409b50, 0x1425e4a40, 0x141834d18> as *const ()), // sub_140081a40
    Replacement::draft(0x140081aa0, "unknown_140081aa0", tpl_47::<0x143a1e938, 0x143a1e93b, 0x1430787b0, 0x142409b50, 0x1425e4a50, 0x141834d18> as *const ()), // sub_140081aa0
    Replacement::draft(0x140081b00, "unknown_140081b00", tpl_38::<0x143a1ee60, 0x143a1ee63, 0x1430787d0, 0x142409b50, 0x1425e4a60, 0x141834d18> as *const ()), // sub_140081b00
    Replacement::draft(0x140081b60, "unknown_140081b60", tpl_48::<0x143a1f040, 0x143a1f043, 0x1430787e0, 0x142409b50, 0x1425e4a70, 0x141834d18> as *const ()), // sub_140081b60
    Replacement::draft(0x140081bc0, "unknown_140081bc0", tpl_49::<0x143a1ecf8, 0x143a1ecfb, 0x1430787f8, 0x142409b50, 0x1425e4a80, 0x141834d18> as *const ()), // sub_140081bc0
    Replacement::draft(0x140081c20, "unknown_140081c20", tpl_44::<0x143a1efc8, 0x143a1efcb, 0x143078808, 0x142409b50, 0x1425e4a90, 0x141834d18> as *const ()), // sub_140081c20
    Replacement::draft(0x140081c80, "unknown_140081c80", tpl_42::<0x143a1e6c0, 0x143a1e6c3, 0x143078820, 0x142409b50, 0x1425e4aa0, 0x141834d18> as *const ()), // sub_140081c80
    Replacement::draft(0x140081ce0, "unknown_140081ce0", tpl_50::<0x143a1eed8, 0x143a1eedb, 0x143078838, 0x142409b50, 0x1425e4ab0, 0x141834d18> as *const ()), // sub_140081ce0
    Replacement::draft(0x140081d40, "unknown_140081d40", tpl_52::<0x143a1e850, 0x143a1e853, 0x143a1f130, 0x143a1f134, 0x143a1f133, 0x143a1f138, 0x142409b50, 0x1425e4ac0, 0x141834d18> as *const ()), // sub_140081d40
    Replacement::draft(0x140081e00, "unknown_140081e00", tpl_38::<0x143a1ed48, 0x143a1ed4b, 0x143078950, 0x142409b50, 0x1425e4ad0, 0x141834d18> as *const ()), // sub_140081e00
    Replacement::draft(0x140081e60, "unknown_140081e60", tpl_53::<0x143a1e910, 0x143a1e913, 0x143078960, 0x142409b50, 0x1425e4ae0, 0x141834d18> as *const ()), // sub_140081e60
    Replacement::draft(0x140081ec0, "unknown_140081ec0", tpl_45::<0x143a1e7d8, 0x143a1e7db, 0x143078980, 0x142409b50, 0x1425e4af0, 0x141834d18> as *const ()), // sub_140081ec0
    Replacement::draft(0x140081f20, "unknown_140081f20", tpl_35::<0x143a1e828, 0x143a1e82b, 0x14307898c, 0x142409b50, 0x1425e4b00, 0x141834d18> as *const ()), // sub_140081f20
    Replacement::draft(0x140081f80, "unknown_140081f80", tpl_42::<0x143a1f090, 0x143a1f093, 0x1430786e0, 0x142409b50, 0x1425e4b10, 0x141834d18> as *const ()), // sub_140081f80
    Replacement::draft(0x140081fe0, "unknown_140081fe0", tpl_38::<0x143a1efa0, 0x143a1efa3, 0x143078998, 0x142409b50, 0x1425e4b20, 0x141834d18> as *const ()), // sub_140081fe0
    Replacement::draft(0x140082040, "unknown_140082040", tpl_38::<0x143a1eb40, 0x143a1eb43, 0x1430789a8, 0x142409b50, 0x1425e4b30, 0x141834d18> as *const ()), // sub_140082040
    Replacement::draft(0x1400820a0, "unknown_1400820a0", tpl_41::<0x143a1ec58, 0x143a1ec5b, 0x1430789b8, 0x142409b50, 0x1425e4b40, 0x141834d18> as *const ()), // sub_1400820a0
    Replacement::draft(0x140082100, "unknown_140082100", tpl_34::<0x143a1f0e0, 0x143a1f0e3, 0x1430789c8, 0x142409b50, 0x1425e4b50, 0x141834d18> as *const ()), // sub_140082100
    Replacement::draft(0x140082160, "unknown_140082160", tpl_39::<0x143a1e800, 0x143a1e803, 0x1430789d8, 0x142409b50, 0x1425e4b60, 0x141834d18> as *const ()), // sub_140082160
    Replacement::draft(0x1400821c0, "unknown_1400821c0", tpl_41::<0x143a1eb18, 0x143a1eb1b, 0x1430789e0, 0x142409b50, 0x1425e4b70, 0x141834d18> as *const ()), // sub_1400821c0
    Replacement::draft(0x140082220, "unknown_140082220", tpl_34::<0x143a1ed70, 0x143a1ed73, 0x1430789f0, 0x142409b50, 0x1425e4b80, 0x141834d18> as *const ()), // sub_140082220
    Replacement::draft(0x140082280, "unknown_140082280", tpl_48::<0x143a1ea50, 0x143a1ea53, 0x143078a00, 0x142409b50, 0x1425e4b90, 0x141834d18> as *const ()), // sub_140082280
    Replacement::draft(0x1400822e0, "unknown_1400822e0", tpl_46::<0x143a1ee88, 0x143a1ee8b, 0x143078a18, 0x142409b50, 0x1425e4ba0, 0x141834d18> as *const ()), // sub_1400822e0
    Replacement::draft(0x140082340, "unknown_140082340", tpl_47::<0x143a1ede8, 0x143a1edeb, 0x143078a30, 0x142409b50, 0x1425e4bb0, 0x141834d18> as *const ()), // sub_140082340
    Replacement::draft(0x1400823a0, "unknown_1400823a0", tpl_37::<0x143a1e988, 0x143a1e98b, 0x143078a50, 0x142409b50, 0x1425e4bc0, 0x141834d18> as *const ()), // sub_1400823a0
    Replacement::draft(0x140082400, "unknown_140082400", tpl_45::<0x143a1ebe0, 0x143a1ebe3, 0x143078a60, 0x142409b50, 0x1425e4bd0, 0x141834d18> as *const ()), // sub_140082400
    Replacement::draft(0x140082460, "unknown_140082460", tpl_54::<0x143a1eca8, 0x143a1ecab, 0x143078a70, 0x142409b50, 0x1425e4be0, 0x141834d18> as *const ()), // sub_140082460
    Replacement::draft(0x1400824c0, "unknown_1400824c0", tpl_40::<0x143a1e960, 0x143a1e963, 0x143078a88, 0x142409b50, 0x1425e4bf0, 0x141834d18> as *const ()), // sub_1400824c0
    Replacement::draft(0x140082520, "unknown_140082520", tpl_50::<0x143a1e9b0, 0x143a1e9b3, 0x143078a90, 0x142409b50, 0x1425e4c00, 0x141834d18> as *const ()), // sub_140082520
    Replacement::draft(0x140082580, "unknown_140082580", tpl_41::<0x143a1eb68, 0x143a1eb6b, 0x143078aa8, 0x142409b50, 0x1425e4c10, 0x141834d18> as *const ()), // sub_140082580
    Replacement::draft(0x1400825e0, "unknown_1400825e0", tpl_55::<0x143a1ee10, 0x143a1ee13, 0x143078ab8, 0x142409b50, 0x1425e4c20, 0x141834d18> as *const ()), // sub_1400825e0
    Replacement::draft(0x140082640, "unknown_140082640", tpl_41::<0x143a1eeb0, 0x143a1eeb3, 0x143078ac8, 0x142409b50, 0x1425e4c30, 0x141834d18> as *const ()), // sub_140082640
    Replacement::draft(0x1400826a0, "unknown_1400826a0", tpl_49::<0x143a1ef50, 0x143a1ef53, 0x143078ad8, 0x142409b50, 0x1425e4c40, 0x141834d18> as *const ()), // sub_1400826a0
    Replacement::draft(0x140082700, "unknown_140082700", tpl_40::<0x143a1e7b0, 0x143a1e7b3, 0x143078ae4, 0x142409b50, 0x1425e4c50, 0x141834d18> as *const ()), // sub_140082700
    Replacement::draft(0x140082760, "unknown_140082760", tpl_44::<0x143a1e8e8, 0x143a1e8eb, 0x143078af0, 0x142409b50, 0x1425e4c60, 0x141834d18> as *const ()), // sub_140082760
    Replacement::draft(0x1400827c0, "unknown_1400827c0", tpl_46::<0x143a1e788, 0x143a1e78b, 0x143078b08, 0x142409b50, 0x1425e4c70, 0x141834d18> as *const ()), // sub_1400827c0
    Replacement::draft(0x140082820, "unknown_140082820", tpl_55::<0x143a1ee38, 0x143a1ee3b, 0x143078b20, 0x142409b50, 0x1425e4c80, 0x141834d18> as *const ()), // sub_140082820
    Replacement::draft(0x140082880, "unknown_140082880", tpl_46::<0x143a1ecd0, 0x143a1ecd3, 0x143078b30, 0x142409b50, 0x1425e4c90, 0x141834d18> as *const ()), // sub_140082880
    Replacement::draft(0x1400828e0, "unknown_1400828e0", tpl_50::<0x143a1ea78, 0x143a1ea7b, 0x143078b48, 0x142409b50, 0x1425e4ca0, 0x141834d18> as *const ()), // sub_1400828e0
    Replacement::draft(0x140082940, "unknown_140082940", tpl_42::<0x143a1e738, 0x143a1e73b, 0x143078b60, 0x142409b50, 0x1425e4cb0, 0x141834d18> as *const ()), // sub_140082940
    Replacement::draft(0x1400829a0, "unknown_1400829a0", tpl_38::<0x143a1ed98, 0x143a1ed9b, 0x143078b78, 0x142409b50, 0x1425e4cc0, 0x141834d18> as *const ()), // sub_1400829a0
    Replacement::draft(0x140082a00, "unknown_140082a00", tpl_34::<0x143a1f108, 0x143a1f10b, 0x143078b88, 0x142409b50, 0x1425e4cd0, 0x141834d18> as *const ()), // sub_140082a00
    Replacement::draft(0x140082a60, "unknown_140082a60", tpl_41::<0x143a1ef00, 0x143a1ef03, 0x143078b98, 0x142409b50, 0x1425e4ce0, 0x141834d18> as *const ()), // sub_140082a60
    Replacement::draft(0x140082ac0, "unknown_140082ac0", tpl_36::<0x143a1ec80, 0x143a1ec83, 0x143078ba8, 0x142409b50, 0x1425e4cf0, 0x141834d18> as *const ()), // sub_140082ac0
    Replacement::draft(0x140082b20, "unknown_140082b20", tpl_34::<0x143a1e760, 0x143a1e763, 0x143078bb8, 0x142409b50, 0x1425e4d00, 0x141834d18> as *const ()), // sub_140082b20
    Replacement::draft(0x140082e50, "unknown_140082e50", tpl_23::<0x143a1f310, 0x143a1f313, 0x1426cdcb0, 0x142409b50, 0x1425e4d10, 0x141834d18> as *const ()), // sub_140082e50
    Replacement::draft(0x140082eb0, "unknown_140082eb0", tpl_23::<0x143a1f358, 0x143a1f35b, 0x1426cdcb0, 0x142409b50, 0x1425e4d20, 0x141834d18> as *const ()), // sub_140082eb0
    Replacement::draft(0x1400830a0, "unknown_1400830a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848028, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e4d60, 0x141834d18> as *const ()), // sub_1400830a0
    Replacement::draft(0x140083430, "unknown_140083430", tpl_34::<0x143a1f4c0, 0x143a1f4c3, 0x143078668, 0x142409b50, 0x1425e4da0, 0x141834d18> as *const ()), // sub_140083430
    Replacement::draft(0x140083490, "unknown_140083490", tpl_23::<0x143a1fac8, 0x143a1facb, 0x1426cdcb0, 0x142409b50, 0x1425e4db0, 0x141834d18> as *const ()), // sub_140083490
    Replacement::draft(0x1400834f0, "unknown_1400834f0", tpl_35::<0x143a1f7f8, 0x143a1f7fb, 0x143078678, 0x142409b50, 0x1425e4dc0, 0x141834d18> as *const ()), // sub_1400834f0
    Replacement::draft(0x140083550, "unknown_140083550", tpl_36::<0x143a1fa50, 0x143a1fa53, 0x143078680, 0x142409b50, 0x1425e4dd0, 0x141834d18> as *const ()), // sub_140083550
    Replacement::draft(0x1400835b0, "unknown_1400835b0", tpl_37::<0x143a1f6a0, 0x143a1f6a3, 0x143078690, 0x142409b50, 0x1425e4de0, 0x141834d18> as *const ()), // sub_1400835b0
    Replacement::draft(0x140083610, "unknown_140083610", tpl_38::<0x143a1f560, 0x143a1f563, 0x1430786a0, 0x142409b50, 0x1425e4df0, 0x141834d18> as *const ()), // sub_140083610
    Replacement::draft(0x140083670, "unknown_140083670", tpl_39::<0x143a1faf0, 0x143a1faf3, 0x1430786b0, 0x142409b50, 0x1425e4e00, 0x141834d18> as *const ()), // sub_140083670
    Replacement::draft(0x1400836d0, "unknown_1400836d0", tpl_37::<0x143a1f470, 0x143a1f473, 0x1430786b8, 0x142409b50, 0x1425e4e10, 0x141834d18> as *const ()), // sub_1400836d0
    Replacement::draft(0x140083730, "unknown_140083730", tpl_40::<0x143a1fc08, 0x143a1fc0b, 0x1430786c4, 0x142409b50, 0x1425e4e20, 0x141834d18> as *const ()), // sub_140083730
    Replacement::draft(0x140083790, "unknown_140083790", tpl_41::<0x143a1f628, 0x143a1f62b, 0x1430786d0, 0x142409b50, 0x1425e4e30, 0x141834d18> as *const ()), // sub_140083790
    Replacement::draft(0x1400837f0, "unknown_1400837f0", tpl_42::<0x143a1fb40, 0x143a1fb43, 0x1430786e0, 0x142409b50, 0x1425e4e40, 0x141834d18> as *const ()), // sub_1400837f0
    Replacement::draft(0x140083850, "unknown_140083850", tpl_34::<0x143a1fb90, 0x143a1fb93, 0x1430786f8, 0x142409b50, 0x1425e4e50, 0x141834d18> as *const ()), // sub_140083850
    Replacement::draft(0x1400838b0, "unknown_1400838b0", tpl_34::<0x143a1f498, 0x143a1f49b, 0x143078708, 0x142409b50, 0x1425e4e60, 0x141834d18> as *const ()), // sub_1400838b0
    Replacement::draft(0x140083910, "unknown_140083910", tpl_41::<0x143a1f588, 0x143a1f58b, 0x143078718, 0x142409b50, 0x1425e4e70, 0x141834d18> as *const ()), // sub_140083910
    Replacement::draft(0x140083970, "unknown_140083970", tpl_43::<0x143a1f6c8, 0x143a1f6cb, 0x143078728, 0x142409b50, 0x1425e4e80, 0x141834d18> as *const ()), // sub_140083970
    Replacement::draft(0x1400839d0, "unknown_1400839d0", tpl_44::<0x143a1f180, 0x143a1f183, 0x143078748, 0x142409b50, 0x1425e4e90, 0x141834d18> as *const ()), // sub_1400839d0
    Replacement::draft(0x140083a30, "unknown_140083a30", tpl_39::<0x143a1f650, 0x143a1f653, 0x143078760, 0x142409b50, 0x1425e4ea0, 0x141834d18> as *const ()), // sub_140083a30
    Replacement::draft(0x140083a90, "unknown_140083a90", tpl_45::<0x143a1f538, 0x143a1f53b, 0x143078768, 0x142409b50, 0x1425e4eb0, 0x141834d18> as *const ()), // sub_140083a90
    Replacement::draft(0x140083af0, "unknown_140083af0", tpl_41::<0x143a1fa00, 0x143a1fa03, 0x143078778, 0x142409b50, 0x1425e4ec0, 0x141834d18> as *const ()), // sub_140083af0
    Replacement::draft(0x140083b50, "unknown_140083b50", tpl_37::<0x143a1f1a8, 0x143a1f1ab, 0x143078788, 0x142409b50, 0x1425e4ed0, 0x141834d18> as *const ()), // sub_140083b50
    Replacement::draft(0x140083bb0, "unknown_140083bb0", tpl_46::<0x143a1f898, 0x143a1f89b, 0x143078798, 0x142409b50, 0x1425e4ee0, 0x141834d18> as *const ()), // sub_140083bb0
    Replacement::draft(0x140083c10, "unknown_140083c10", tpl_47::<0x143a1f3d0, 0x143a1f3d3, 0x1430787b0, 0x142409b50, 0x1425e4ef0, 0x141834d18> as *const ()), // sub_140083c10
    Replacement::draft(0x140083c70, "unknown_140083c70", tpl_38::<0x143a1f938, 0x143a1f93b, 0x1430787d0, 0x142409b50, 0x1425e4f00, 0x141834d18> as *const ()), // sub_140083c70
    Replacement::draft(0x140083cd0, "unknown_140083cd0", tpl_48::<0x143a1fb18, 0x143a1fb1b, 0x1430787e0, 0x142409b50, 0x1425e4f10, 0x141834d18> as *const ()), // sub_140083cd0
    Replacement::draft(0x140083d30, "unknown_140083d30", tpl_49::<0x143a1f7d0, 0x143a1f7d3, 0x1430787f8, 0x142409b50, 0x1425e4f20, 0x141834d18> as *const ()), // sub_140083d30
    Replacement::draft(0x140083d90, "unknown_140083d90", tpl_44::<0x143a1faa0, 0x143a1faa3, 0x143078808, 0x142409b50, 0x1425e4f30, 0x141834d18> as *const ()), // sub_140083d90
    Replacement::draft(0x140083df0, "unknown_140083df0", tpl_42::<0x143a1f158, 0x143a1f15b, 0x143078820, 0x142409b50, 0x1425e4f40, 0x141834d18> as *const ()), // sub_140083df0
    Replacement::draft(0x140083e50, "unknown_140083e50", tpl_50::<0x143a1f9b0, 0x143a1f9b3, 0x143078838, 0x142409b50, 0x1425e4f50, 0x141834d18> as *const ()), // sub_140083e50
    Replacement::draft(0x140083eb0, "unknown_140083eb0", tpl_52::<0x143a1f2e8, 0x143a1f2eb, 0x143a1fc08, 0x143a1fc0c, 0x143a1fc0b, 0x143a1fc10, 0x142409b50, 0x1425e4f60, 0x141834d18> as *const ()), // sub_140083eb0
    Replacement::draft(0x140083f70, "unknown_140083f70", tpl_38::<0x143a1f820, 0x143a1f823, 0x143078950, 0x142409b50, 0x1425e4f70, 0x141834d18> as *const ()), // sub_140083f70
    Replacement::draft(0x140083fd0, "unknown_140083fd0", tpl_53::<0x143a1f3a8, 0x143a1f3ab, 0x143078960, 0x142409b50, 0x1425e4f80, 0x141834d18> as *const ()), // sub_140083fd0
    Replacement::draft(0x140084030, "unknown_140084030", tpl_45::<0x143a1f270, 0x143a1f273, 0x143078980, 0x142409b50, 0x1425e4f90, 0x141834d18> as *const ()), // sub_140084030
    Replacement::draft(0x140084090, "unknown_140084090", tpl_35::<0x143a1f2c0, 0x143a1f2c3, 0x14307898c, 0x142409b50, 0x1425e4fa0, 0x141834d18> as *const ()), // sub_140084090
    Replacement::draft(0x1400840f0, "unknown_1400840f0", tpl_42::<0x143a1fb68, 0x143a1fb6b, 0x1430786e0, 0x142409b50, 0x1425e4fb0, 0x141834d18> as *const ()), // sub_1400840f0
    Replacement::draft(0x140084150, "unknown_140084150", tpl_38::<0x143a1fa78, 0x143a1fa7b, 0x143078998, 0x142409b50, 0x1425e4fc0, 0x141834d18> as *const ()), // sub_140084150
    Replacement::draft(0x1400841b0, "unknown_1400841b0", tpl_38::<0x143a1f5d8, 0x143a1f5db, 0x1430789a8, 0x142409b50, 0x1425e4fd0, 0x141834d18> as *const ()), // sub_1400841b0
    Replacement::draft(0x140084210, "unknown_140084210", tpl_41::<0x143a1f730, 0x143a1f733, 0x1430789b8, 0x142409b50, 0x1425e4fe0, 0x141834d18> as *const ()), // sub_140084210
    Replacement::draft(0x140084270, "unknown_140084270", tpl_34::<0x143a1fbb8, 0x143a1fbbb, 0x1430789c8, 0x142409b50, 0x1425e4ff0, 0x141834d18> as *const ()), // sub_140084270
    Replacement::draft(0x1400842d0, "unknown_1400842d0", tpl_39::<0x143a1f298, 0x143a1f29b, 0x1430789d8, 0x142409b50, 0x1425e5000, 0x141834d18> as *const ()), // sub_1400842d0
    Replacement::draft(0x140084330, "unknown_140084330", tpl_41::<0x143a1f5b0, 0x143a1f5b3, 0x1430789e0, 0x142409b50, 0x1425e5010, 0x141834d18> as *const ()), // sub_140084330
    Replacement::draft(0x140084390, "unknown_140084390", tpl_34::<0x143a1f848, 0x143a1f84b, 0x1430789f0, 0x142409b50, 0x1425e5020, 0x141834d18> as *const ()), // sub_140084390
    Replacement::draft(0x1400843f0, "unknown_1400843f0", tpl_48::<0x143a1f4e8, 0x143a1f4eb, 0x143078a00, 0x142409b50, 0x1425e5030, 0x141834d18> as *const ()), // sub_1400843f0
    Replacement::draft(0x140084450, "unknown_140084450", tpl_46::<0x143a1f960, 0x143a1f963, 0x143078a18, 0x142409b50, 0x1425e5040, 0x141834d18> as *const ()), // sub_140084450
    Replacement::draft(0x1400844b0, "unknown_1400844b0", tpl_47::<0x143a1f8c0, 0x143a1f8c3, 0x143078a30, 0x142409b50, 0x1425e5050, 0x141834d18> as *const ()), // sub_1400844b0
    Replacement::draft(0x140084510, "unknown_140084510", tpl_37::<0x143a1f420, 0x143a1f423, 0x143078a50, 0x142409b50, 0x1425e5060, 0x141834d18> as *const ()), // sub_140084510
    Replacement::draft(0x140084570, "unknown_140084570", tpl_45::<0x143a1f678, 0x143a1f67b, 0x143078a60, 0x142409b50, 0x1425e5070, 0x141834d18> as *const ()), // sub_140084570
    Replacement::draft(0x1400845d0, "unknown_1400845d0", tpl_54::<0x143a1f780, 0x143a1f783, 0x143078a70, 0x142409b50, 0x1425e5080, 0x141834d18> as *const ()), // sub_1400845d0
    Replacement::draft(0x140084630, "unknown_140084630", tpl_40::<0x143a1f3f8, 0x143a1f3fb, 0x143078a88, 0x142409b50, 0x1425e5090, 0x141834d18> as *const ()), // sub_140084630
    Replacement::draft(0x140084690, "unknown_140084690", tpl_50::<0x143a1f448, 0x143a1f44b, 0x143078a90, 0x142409b50, 0x1425e50a0, 0x141834d18> as *const ()), // sub_140084690
    Replacement::draft(0x1400846f0, "unknown_1400846f0", tpl_41::<0x143a1f600, 0x143a1f603, 0x143078aa8, 0x142409b50, 0x1425e50b0, 0x141834d18> as *const ()), // sub_1400846f0
    Replacement::draft(0x140084750, "unknown_140084750", tpl_55::<0x143a1f8e8, 0x143a1f8eb, 0x143078ab8, 0x142409b50, 0x1425e50c0, 0x141834d18> as *const ()), // sub_140084750
    Replacement::draft(0x1400847b0, "unknown_1400847b0", tpl_41::<0x143a1f988, 0x143a1f98b, 0x143078ac8, 0x142409b50, 0x1425e50d0, 0x141834d18> as *const ()), // sub_1400847b0
    Replacement::draft(0x140084810, "unknown_140084810", tpl_49::<0x143a1fa28, 0x143a1fa2b, 0x143078ad8, 0x142409b50, 0x1425e50e0, 0x141834d18> as *const ()), // sub_140084810
    Replacement::draft(0x140084870, "unknown_140084870", tpl_40::<0x143a1f248, 0x143a1f24b, 0x143078ae4, 0x142409b50, 0x1425e50f0, 0x141834d18> as *const ()), // sub_140084870
    Replacement::draft(0x1400848d0, "unknown_1400848d0", tpl_44::<0x143a1f380, 0x143a1f383, 0x143078af0, 0x142409b50, 0x1425e5100, 0x141834d18> as *const ()), // sub_1400848d0
    Replacement::draft(0x140084930, "unknown_140084930", tpl_46::<0x143a1f220, 0x143a1f223, 0x143078b08, 0x142409b50, 0x1425e5110, 0x141834d18> as *const ()), // sub_140084930
    Replacement::draft(0x140084990, "unknown_140084990", tpl_55::<0x143a1f910, 0x143a1f913, 0x143078b20, 0x142409b50, 0x1425e5120, 0x141834d18> as *const ()), // sub_140084990
    Replacement::draft(0x1400849f0, "unknown_1400849f0", tpl_46::<0x143a1f7a8, 0x143a1f7ab, 0x143078b30, 0x142409b50, 0x1425e5130, 0x141834d18> as *const ()), // sub_1400849f0
    Replacement::draft(0x140084a50, "unknown_140084a50", tpl_50::<0x143a1f510, 0x143a1f513, 0x143078b48, 0x142409b50, 0x1425e5140, 0x141834d18> as *const ()), // sub_140084a50
    Replacement::draft(0x140084ab0, "unknown_140084ab0", tpl_42::<0x143a1f1d0, 0x143a1f1d3, 0x143078b60, 0x142409b50, 0x1425e5150, 0x141834d18> as *const ()), // sub_140084ab0
    Replacement::draft(0x140084b10, "unknown_140084b10", tpl_38::<0x143a1f870, 0x143a1f873, 0x143078b78, 0x142409b50, 0x1425e5160, 0x141834d18> as *const ()), // sub_140084b10
    Replacement::draft(0x140084b70, "unknown_140084b70", tpl_34::<0x143a1fbe0, 0x143a1fbe3, 0x143078b88, 0x142409b50, 0x1425e5170, 0x141834d18> as *const ()), // sub_140084b70
    Replacement::draft(0x140084bd0, "unknown_140084bd0", tpl_41::<0x143a1f9d8, 0x143a1f9db, 0x143078b98, 0x142409b50, 0x1425e5180, 0x141834d18> as *const ()), // sub_140084bd0
    Replacement::draft(0x140084c30, "unknown_140084c30", tpl_36::<0x143a1f758, 0x143a1f75b, 0x143078ba8, 0x142409b50, 0x1425e5190, 0x141834d18> as *const ()), // sub_140084c30
    Replacement::draft(0x140084c90, "unknown_140084c90", tpl_34::<0x143a1f1f8, 0x143a1f1fb, 0x143078bb8, 0x142409b50, 0x1425e51a0, 0x141834d18> as *const ()), // sub_140084c90
    Replacement::draft(0x140084d60, "unknown_140084d60", tpl_23::<0x143a1fde8, 0x143a1fdeb, 0x1426cdcb0, 0x142409b50, 0x1425e51e0, 0x141834d18> as *const ()), // sub_140084d60
    Replacement::draft(0x140084dc0, "unknown_140084dc0", tpl_23::<0x143a1fe30, 0x143a1fe33, 0x1426cdcb0, 0x142409b50, 0x1425e51f0, 0x141834d18> as *const ()), // sub_140084dc0
    Replacement::draft(0x140084fb0, "unknown_140084fb0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438480b8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e5230, 0x141834d18> as *const ()), // sub_140084fb0
    Replacement::draft(0x140085340, "unknown_140085340", tpl_34::<0x143a1ff98, 0x143a1ff9b, 0x143078668, 0x142409b50, 0x1425e5270, 0x141834d18> as *const ()), // sub_140085340
    Replacement::draft(0x1400853a0, "unknown_1400853a0", tpl_23::<0x143a20560, 0x143a20563, 0x1426cdcb0, 0x142409b50, 0x1425e5280, 0x141834d18> as *const ()), // sub_1400853a0
    Replacement::draft(0x140085400, "unknown_140085400", tpl_35::<0x143a20290, 0x143a20293, 0x143078678, 0x142409b50, 0x1425e5290, 0x141834d18> as *const ()), // sub_140085400
    Replacement::draft(0x140085460, "unknown_140085460", tpl_36::<0x143a204e8, 0x143a204eb, 0x143078680, 0x142409b50, 0x1425e52a0, 0x141834d18> as *const ()), // sub_140085460
    Replacement::draft(0x1400854c0, "unknown_1400854c0", tpl_37::<0x143a20178, 0x143a2017b, 0x143078690, 0x142409b50, 0x1425e52b0, 0x141834d18> as *const ()), // sub_1400854c0
    Replacement::draft(0x140085520, "unknown_140085520", tpl_38::<0x143a20038, 0x143a2003b, 0x1430786a0, 0x142409b50, 0x1425e52c0, 0x141834d18> as *const ()), // sub_140085520
    Replacement::draft(0x140085580, "unknown_140085580", tpl_39::<0x143a20588, 0x143a2058b, 0x1430786b0, 0x142409b50, 0x1425e52d0, 0x141834d18> as *const ()), // sub_140085580
    Replacement::draft(0x1400855e0, "unknown_1400855e0", tpl_37::<0x143a1ff48, 0x143a1ff4b, 0x1430786b8, 0x142409b50, 0x1425e52e0, 0x141834d18> as *const ()), // sub_1400855e0
    Replacement::draft(0x140085640, "unknown_140085640", tpl_40::<0x143a206a0, 0x143a206a3, 0x1430786c4, 0x142409b50, 0x1425e52f0, 0x141834d18> as *const ()), // sub_140085640
    Replacement::draft(0x1400856a0, "unknown_1400856a0", tpl_41::<0x143a20100, 0x143a20103, 0x1430786d0, 0x142409b50, 0x1425e5300, 0x141834d18> as *const ()), // sub_1400856a0
    Replacement::draft(0x140085700, "unknown_140085700", tpl_42::<0x143a205d8, 0x143a205db, 0x1430786e0, 0x142409b50, 0x1425e5310, 0x141834d18> as *const ()), // sub_140085700
    Replacement::draft(0x140085760, "unknown_140085760", tpl_34::<0x143a20628, 0x143a2062b, 0x1430786f8, 0x142409b50, 0x1425e5320, 0x141834d18> as *const ()), // sub_140085760
    Replacement::draft(0x1400857c0, "unknown_1400857c0", tpl_34::<0x143a1ff70, 0x143a1ff73, 0x143078708, 0x142409b50, 0x1425e5330, 0x141834d18> as *const ()), // sub_1400857c0
    Replacement::draft(0x140085820, "unknown_140085820", tpl_41::<0x143a20060, 0x143a20063, 0x143078718, 0x142409b50, 0x1425e5340, 0x141834d18> as *const ()), // sub_140085820
    Replacement::draft(0x140085880, "unknown_140085880", tpl_43::<0x143a201a0, 0x143a201a3, 0x143078728, 0x142409b50, 0x1425e5350, 0x141834d18> as *const ()), // sub_140085880
    Replacement::draft(0x1400858e0, "unknown_1400858e0", tpl_44::<0x143a1fc58, 0x143a1fc5b, 0x143078748, 0x142409b50, 0x1425e5360, 0x141834d18> as *const ()), // sub_1400858e0
    Replacement::draft(0x140085940, "unknown_140085940", tpl_39::<0x143a20128, 0x143a2012b, 0x143078760, 0x142409b50, 0x1425e5370, 0x141834d18> as *const ()), // sub_140085940
    Replacement::draft(0x1400859a0, "unknown_1400859a0", tpl_45::<0x143a20010, 0x143a20013, 0x143078768, 0x142409b50, 0x1425e5380, 0x141834d18> as *const ()), // sub_1400859a0
    Replacement::draft(0x140085a00, "unknown_140085a00", tpl_41::<0x143a20498, 0x143a2049b, 0x143078778, 0x142409b50, 0x1425e5390, 0x141834d18> as *const ()), // sub_140085a00
    Replacement::draft(0x140085a60, "unknown_140085a60", tpl_37::<0x143a1fc80, 0x143a1fc83, 0x143078788, 0x142409b50, 0x1425e53a0, 0x141834d18> as *const ()), // sub_140085a60
    Replacement::draft(0x140085ac0, "unknown_140085ac0", tpl_46::<0x143a20330, 0x143a20333, 0x143078798, 0x142409b50, 0x1425e53b0, 0x141834d18> as *const ()), // sub_140085ac0
    Replacement::draft(0x140085b20, "unknown_140085b20", tpl_47::<0x143a1fea8, 0x143a1feab, 0x1430787b0, 0x142409b50, 0x1425e53c0, 0x141834d18> as *const ()), // sub_140085b20
    Replacement::draft(0x140085b80, "unknown_140085b80", tpl_38::<0x143a203d0, 0x143a203d3, 0x1430787d0, 0x142409b50, 0x1425e53d0, 0x141834d18> as *const ()), // sub_140085b80
    Replacement::draft(0x140085be0, "unknown_140085be0", tpl_48::<0x143a205b0, 0x143a205b3, 0x1430787e0, 0x142409b50, 0x1425e53e0, 0x141834d18> as *const ()), // sub_140085be0
    Replacement::draft(0x140085c40, "unknown_140085c40", tpl_49::<0x143a20268, 0x143a2026b, 0x1430787f8, 0x142409b50, 0x1425e53f0, 0x141834d18> as *const ()), // sub_140085c40
    Replacement::draft(0x140085ca0, "unknown_140085ca0", tpl_44::<0x143a20538, 0x143a2053b, 0x143078808, 0x142409b50, 0x1425e5400, 0x141834d18> as *const ()), // sub_140085ca0
    Replacement::draft(0x140085d00, "unknown_140085d00", tpl_42::<0x143a1fc30, 0x143a1fc33, 0x143078820, 0x142409b50, 0x1425e5410, 0x141834d18> as *const ()), // sub_140085d00
    Replacement::draft(0x140085d60, "unknown_140085d60", tpl_50::<0x143a20448, 0x143a2044b, 0x143078838, 0x142409b50, 0x1425e5420, 0x141834d18> as *const ()), // sub_140085d60
    Replacement::draft(0x140085dc0, "unknown_140085dc0", tpl_52::<0x143a1fdc0, 0x143a1fdc3, 0x143a206a0, 0x143a206a4, 0x143a206a3, 0x143a206a8, 0x142409b50, 0x1425e5430, 0x141834d18> as *const ()), // sub_140085dc0
    Replacement::draft(0x140085e80, "unknown_140085e80", tpl_38::<0x143a202b8, 0x143a202bb, 0x143078950, 0x142409b50, 0x1425e5440, 0x141834d18> as *const ()), // sub_140085e80
    Replacement::draft(0x140085ee0, "unknown_140085ee0", tpl_53::<0x143a1fe80, 0x143a1fe83, 0x143078960, 0x142409b50, 0x1425e5450, 0x141834d18> as *const ()), // sub_140085ee0
    Replacement::draft(0x140085f40, "unknown_140085f40", tpl_45::<0x143a1fd48, 0x143a1fd4b, 0x143078980, 0x142409b50, 0x1425e5460, 0x141834d18> as *const ()), // sub_140085f40
    Replacement::draft(0x140085fa0, "unknown_140085fa0", tpl_35::<0x143a1fd98, 0x143a1fd9b, 0x14307898c, 0x142409b50, 0x1425e5470, 0x141834d18> as *const ()), // sub_140085fa0
    Replacement::draft(0x140086000, "unknown_140086000", tpl_42::<0x143a20600, 0x143a20603, 0x1430786e0, 0x142409b50, 0x1425e5480, 0x141834d18> as *const ()), // sub_140086000
    Replacement::draft(0x140086060, "unknown_140086060", tpl_38::<0x143a20510, 0x143a20513, 0x143078998, 0x142409b50, 0x1425e5490, 0x141834d18> as *const ()), // sub_140086060
    Replacement::draft(0x1400860c0, "unknown_1400860c0", tpl_38::<0x143a200b0, 0x143a200b3, 0x1430789a8, 0x142409b50, 0x1425e54a0, 0x141834d18> as *const ()), // sub_1400860c0
    Replacement::draft(0x140086120, "unknown_140086120", tpl_41::<0x143a201c8, 0x143a201cb, 0x1430789b8, 0x142409b50, 0x1425e54b0, 0x141834d18> as *const ()), // sub_140086120
    Replacement::draft(0x140086180, "unknown_140086180", tpl_34::<0x143a20650, 0x143a20653, 0x1430789c8, 0x142409b50, 0x1425e54c0, 0x141834d18> as *const ()), // sub_140086180
    Replacement::draft(0x1400861e0, "unknown_1400861e0", tpl_39::<0x143a1fd70, 0x143a1fd73, 0x1430789d8, 0x142409b50, 0x1425e54d0, 0x141834d18> as *const ()), // sub_1400861e0
    Replacement::draft(0x140086240, "unknown_140086240", tpl_41::<0x143a20088, 0x143a2008b, 0x1430789e0, 0x142409b50, 0x1425e54e0, 0x141834d18> as *const ()), // sub_140086240
    Replacement::draft(0x1400862a0, "unknown_1400862a0", tpl_34::<0x143a202e0, 0x143a202e3, 0x1430789f0, 0x142409b50, 0x1425e54f0, 0x141834d18> as *const ()), // sub_1400862a0
    Replacement::draft(0x140086300, "unknown_140086300", tpl_48::<0x143a1ffc0, 0x143a1ffc3, 0x143078a00, 0x142409b50, 0x1425e5500, 0x141834d18> as *const ()), // sub_140086300
    Replacement::draft(0x140086360, "unknown_140086360", tpl_46::<0x143a203f8, 0x143a203fb, 0x143078a18, 0x142409b50, 0x1425e5510, 0x141834d18> as *const ()), // sub_140086360
    Replacement::draft(0x1400863c0, "unknown_1400863c0", tpl_47::<0x143a20358, 0x143a2035b, 0x143078a30, 0x142409b50, 0x1425e5520, 0x141834d18> as *const ()), // sub_1400863c0
    Replacement::draft(0x140086420, "unknown_140086420", tpl_37::<0x143a1fef8, 0x143a1fefb, 0x143078a50, 0x142409b50, 0x1425e5530, 0x141834d18> as *const ()), // sub_140086420
    Replacement::draft(0x140086480, "unknown_140086480", tpl_45::<0x143a20150, 0x143a20153, 0x143078a60, 0x142409b50, 0x1425e5540, 0x141834d18> as *const ()), // sub_140086480
    Replacement::draft(0x1400864e0, "unknown_1400864e0", tpl_54::<0x143a20218, 0x143a2021b, 0x143078a70, 0x142409b50, 0x1425e5550, 0x141834d18> as *const ()), // sub_1400864e0
    Replacement::draft(0x140086540, "unknown_140086540", tpl_40::<0x143a1fed0, 0x143a1fed3, 0x143078a88, 0x142409b50, 0x1425e5560, 0x141834d18> as *const ()), // sub_140086540
    Replacement::draft(0x1400865a0, "unknown_1400865a0", tpl_50::<0x143a1ff20, 0x143a1ff23, 0x143078a90, 0x142409b50, 0x1425e5570, 0x141834d18> as *const ()), // sub_1400865a0
    Replacement::draft(0x140086600, "unknown_140086600", tpl_41::<0x143a200d8, 0x143a200db, 0x143078aa8, 0x142409b50, 0x1425e5580, 0x141834d18> as *const ()), // sub_140086600
    Replacement::draft(0x140086660, "unknown_140086660", tpl_55::<0x143a20380, 0x143a20383, 0x143078ab8, 0x142409b50, 0x1425e5590, 0x141834d18> as *const ()), // sub_140086660
    Replacement::draft(0x1400866c0, "unknown_1400866c0", tpl_41::<0x143a20420, 0x143a20423, 0x143078ac8, 0x142409b50, 0x1425e55a0, 0x141834d18> as *const ()), // sub_1400866c0
    Replacement::draft(0x140086720, "unknown_140086720", tpl_49::<0x143a204c0, 0x143a204c3, 0x143078ad8, 0x142409b50, 0x1425e55b0, 0x141834d18> as *const ()), // sub_140086720
    Replacement::draft(0x140086780, "unknown_140086780", tpl_40::<0x143a1fd20, 0x143a1fd23, 0x143078ae4, 0x142409b50, 0x1425e55c0, 0x141834d18> as *const ()), // sub_140086780
    Replacement::draft(0x1400867e0, "unknown_1400867e0", tpl_44::<0x143a1fe58, 0x143a1fe5b, 0x143078af0, 0x142409b50, 0x1425e55d0, 0x141834d18> as *const ()), // sub_1400867e0
    Replacement::draft(0x140086840, "unknown_140086840", tpl_46::<0x143a1fcf8, 0x143a1fcfb, 0x143078b08, 0x142409b50, 0x1425e55e0, 0x141834d18> as *const ()), // sub_140086840
    Replacement::draft(0x1400868a0, "unknown_1400868a0", tpl_55::<0x143a203a8, 0x143a203ab, 0x143078b20, 0x142409b50, 0x1425e55f0, 0x141834d18> as *const ()), // sub_1400868a0
    Replacement::draft(0x140086900, "unknown_140086900", tpl_46::<0x143a20240, 0x143a20243, 0x143078b30, 0x142409b50, 0x1425e5600, 0x141834d18> as *const ()), // sub_140086900
    Replacement::draft(0x140086960, "unknown_140086960", tpl_50::<0x143a1ffe8, 0x143a1ffeb, 0x143078b48, 0x142409b50, 0x1425e5610, 0x141834d18> as *const ()), // sub_140086960
    Replacement::draft(0x1400869c0, "unknown_1400869c0", tpl_42::<0x143a1fca8, 0x143a1fcab, 0x143078b60, 0x142409b50, 0x1425e5620, 0x141834d18> as *const ()), // sub_1400869c0
    Replacement::draft(0x140086a20, "unknown_140086a20", tpl_38::<0x143a20308, 0x143a2030b, 0x143078b78, 0x142409b50, 0x1425e5630, 0x141834d18> as *const ()), // sub_140086a20
    Replacement::draft(0x140086a80, "unknown_140086a80", tpl_34::<0x143a20678, 0x143a2067b, 0x143078b88, 0x142409b50, 0x1425e5640, 0x141834d18> as *const ()), // sub_140086a80
    Replacement::draft(0x140086ae0, "unknown_140086ae0", tpl_41::<0x143a20470, 0x143a20473, 0x143078b98, 0x142409b50, 0x1425e5650, 0x141834d18> as *const ()), // sub_140086ae0
    Replacement::draft(0x140086b40, "unknown_140086b40", tpl_36::<0x143a201f0, 0x143a201f3, 0x143078ba8, 0x142409b50, 0x1425e5660, 0x141834d18> as *const ()), // sub_140086b40
    Replacement::draft(0x140086ba0, "unknown_140086ba0", tpl_34::<0x143a1fcd0, 0x143a1fcd3, 0x143078bb8, 0x142409b50, 0x1425e5670, 0x141834d18> as *const ()), // sub_140086ba0
    Replacement::draft(0x1400874b0, "unknown_1400874b0", tpl_23::<0x143a206c8, 0x143a206cb, 0x1426cdcb0, 0x142409b50, 0x1425e5750, 0x141834d18> as *const ()), // sub_1400874b0
    Replacement::draft(0x140087510, "unknown_140087510", tpl_23::<0x143a20710, 0x143a20713, 0x1426cdcb0, 0x142409b50, 0x1425e5760, 0x141834d18> as *const ()), // sub_140087510
    Replacement::draft(0x140087700, "unknown_140087700", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848168, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e57a0, 0x141834d18> as *const ()), // sub_140087700
    Replacement::draft(0x140088c60, "unknown_140088c60", tpl_23::<0x143a20738, 0x143a2073b, 0x1426cdcb0, 0x142409b50, 0x1425e5840, 0x141834d18> as *const ()), // sub_140088c60
    Replacement::draft(0x140088cc0, "unknown_140088cc0", tpl_23::<0x143a20780, 0x143a20783, 0x1426cdcb0, 0x142409b50, 0x1425e5850, 0x141834d18> as *const ()), // sub_140088cc0
    Replacement::draft(0x140088eb0, "unknown_140088eb0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438481f8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e5890, 0x141834d18> as *const ()), // sub_140088eb0
    Replacement::draft(0x140089b20, "unknown_140089b20", tpl_23::<0x143a209a8, 0x143a209ab, 0x1426cdcb0, 0x142409b50, 0x1425e5920, 0x141834d18> as *const ()), // sub_140089b20
    Replacement::draft(0x140089b80, "unknown_140089b80", tpl_23::<0x143a209f0, 0x143a209f3, 0x1426cdcb0, 0x142409b50, 0x1425e5930, 0x141834d18> as *const ()), // sub_140089b80
    Replacement::draft(0x140089d70, "unknown_140089d70", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848288, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e5970, 0x141834d18> as *const ()), // sub_140089d70
    Replacement::draft(0x14008a100, "unknown_14008a100", tpl_34::<0x143a20b58, 0x143a20b5b, 0x143078668, 0x142409b50, 0x1425e59b0, 0x141834d18> as *const ()), // sub_14008a100
    Replacement::draft(0x14008a160, "unknown_14008a160", tpl_23::<0x143a21168, 0x143a2116b, 0x1426cdcb0, 0x142409b50, 0x1425e59c0, 0x141834d18> as *const ()), // sub_14008a160
    Replacement::draft(0x14008a1c0, "unknown_14008a1c0", tpl_35::<0x143a20e98, 0x143a20e9b, 0x143078678, 0x142409b50, 0x1425e59d0, 0x141834d18> as *const ()), // sub_14008a1c0
    Replacement::draft(0x14008a220, "unknown_14008a220", tpl_36::<0x143a210f0, 0x143a210f3, 0x143078680, 0x142409b50, 0x1425e59e0, 0x141834d18> as *const ()), // sub_14008a220
    Replacement::draft(0x14008a280, "unknown_14008a280", tpl_37::<0x143a20d38, 0x143a20d3b, 0x143078690, 0x142409b50, 0x1425e59f0, 0x141834d18> as *const ()), // sub_14008a280
    Replacement::draft(0x14008a2e0, "unknown_14008a2e0", tpl_38::<0x143a20bf8, 0x143a20bfb, 0x1430786a0, 0x142409b50, 0x1425e5a00, 0x141834d18> as *const ()), // sub_14008a2e0
    Replacement::draft(0x14008a340, "unknown_14008a340", tpl_39::<0x143a21190, 0x143a21193, 0x1430786b0, 0x142409b50, 0x1425e5a10, 0x141834d18> as *const ()), // sub_14008a340
    Replacement::draft(0x14008a3a0, "unknown_14008a3a0", tpl_37::<0x143a20b08, 0x143a20b0b, 0x1430786b8, 0x142409b50, 0x1425e5a20, 0x141834d18> as *const ()), // sub_14008a3a0
    Replacement::draft(0x14008a400, "unknown_14008a400", tpl_40::<0x143a212a8, 0x143a212ab, 0x1430786c4, 0x142409b50, 0x1425e5a30, 0x141834d18> as *const ()), // sub_14008a400
    Replacement::draft(0x14008a460, "unknown_14008a460", tpl_41::<0x143a20cc0, 0x143a20cc3, 0x1430786d0, 0x142409b50, 0x1425e5a40, 0x141834d18> as *const ()), // sub_14008a460
    Replacement::draft(0x14008a4c0, "unknown_14008a4c0", tpl_42::<0x143a211e0, 0x143a211e3, 0x1430786e0, 0x142409b50, 0x1425e5a50, 0x141834d18> as *const ()), // sub_14008a4c0
    Replacement::draft(0x14008a520, "unknown_14008a520", tpl_34::<0x143a21230, 0x143a21233, 0x1430786f8, 0x142409b50, 0x1425e5a60, 0x141834d18> as *const ()), // sub_14008a520
    Replacement::draft(0x14008a580, "unknown_14008a580", tpl_34::<0x143a20b30, 0x143a20b33, 0x143078708, 0x142409b50, 0x1425e5a70, 0x141834d18> as *const ()), // sub_14008a580
    Replacement::draft(0x14008a5e0, "unknown_14008a5e0", tpl_41::<0x143a20c20, 0x143a20c23, 0x143078718, 0x142409b50, 0x1425e5a80, 0x141834d18> as *const ()), // sub_14008a5e0
    Replacement::draft(0x14008a640, "unknown_14008a640", tpl_43::<0x143a20d60, 0x143a20d63, 0x143078728, 0x142409b50, 0x1425e5a90, 0x141834d18> as *const ()), // sub_14008a640
    Replacement::draft(0x14008a6a0, "unknown_14008a6a0", tpl_44::<0x143a20818, 0x143a2081b, 0x143078748, 0x142409b50, 0x1425e5aa0, 0x141834d18> as *const ()), // sub_14008a6a0
    Replacement::draft(0x14008a700, "unknown_14008a700", tpl_39::<0x143a20ce8, 0x143a20ceb, 0x143078760, 0x142409b50, 0x1425e5ab0, 0x141834d18> as *const ()), // sub_14008a700
    Replacement::draft(0x14008a760, "unknown_14008a760", tpl_45::<0x143a20bd0, 0x143a20bd3, 0x143078768, 0x142409b50, 0x1425e5ac0, 0x141834d18> as *const ()), // sub_14008a760
    Replacement::draft(0x14008a7c0, "unknown_14008a7c0", tpl_41::<0x143a210a0, 0x143a210a3, 0x143078778, 0x142409b50, 0x1425e5ad0, 0x141834d18> as *const ()), // sub_14008a7c0
    Replacement::draft(0x14008a820, "unknown_14008a820", tpl_37::<0x143a20840, 0x143a20843, 0x143078788, 0x142409b50, 0x1425e5ae0, 0x141834d18> as *const ()), // sub_14008a820
    Replacement::draft(0x14008a880, "unknown_14008a880", tpl_46::<0x143a20f38, 0x143a20f3b, 0x143078798, 0x142409b50, 0x1425e5af0, 0x141834d18> as *const ()), // sub_14008a880
    Replacement::draft(0x14008a8e0, "unknown_14008a8e0", tpl_47::<0x143a20a68, 0x143a20a6b, 0x1430787b0, 0x142409b50, 0x1425e5b00, 0x141834d18> as *const ()), // sub_14008a8e0
    Replacement::draft(0x14008a940, "unknown_14008a940", tpl_38::<0x143a20fd8, 0x143a20fdb, 0x1430787d0, 0x142409b50, 0x1425e5b10, 0x141834d18> as *const ()), // sub_14008a940
    Replacement::draft(0x14008a9a0, "unknown_14008a9a0", tpl_48::<0x143a211b8, 0x143a211bb, 0x1430787e0, 0x142409b50, 0x1425e5b20, 0x141834d18> as *const ()), // sub_14008a9a0
    Replacement::draft(0x14008aa00, "unknown_14008aa00", tpl_49::<0x143a20e70, 0x143a20e73, 0x1430787f8, 0x142409b50, 0x1425e5b30, 0x141834d18> as *const ()), // sub_14008aa00
    Replacement::draft(0x14008aa60, "unknown_14008aa60", tpl_44::<0x143a21140, 0x143a21143, 0x143078808, 0x142409b50, 0x1425e5b40, 0x141834d18> as *const ()), // sub_14008aa60
    Replacement::draft(0x14008aac0, "unknown_14008aac0", tpl_42::<0x143a207f0, 0x143a207f3, 0x143078820, 0x142409b50, 0x1425e5b50, 0x141834d18> as *const ()), // sub_14008aac0
    Replacement::draft(0x14008ab20, "unknown_14008ab20", tpl_50::<0x143a21050, 0x143a21053, 0x143078838, 0x142409b50, 0x1425e5b60, 0x141834d18> as *const ()), // sub_14008ab20
    Replacement::draft(0x14008ab80, "unknown_14008ab80", tpl_52::<0x143a20980, 0x143a20983, 0x143a212a8, 0x143a212ac, 0x143a212ab, 0x143a212b0, 0x142409b50, 0x1425e5b70, 0x141834d18> as *const ()), // sub_14008ab80
    Replacement::draft(0x14008ac40, "unknown_14008ac40", tpl_38::<0x143a20ec0, 0x143a20ec3, 0x143078950, 0x142409b50, 0x1425e5b80, 0x141834d18> as *const ()), // sub_14008ac40
    Replacement::draft(0x14008aca0, "unknown_14008aca0", tpl_53::<0x143a20a40, 0x143a20a43, 0x143078960, 0x142409b50, 0x1425e5b90, 0x141834d18> as *const ()), // sub_14008aca0
    Replacement::draft(0x14008ad00, "unknown_14008ad00", tpl_45::<0x143a20908, 0x143a2090b, 0x143078980, 0x142409b50, 0x1425e5ba0, 0x141834d18> as *const ()), // sub_14008ad00
    Replacement::draft(0x14008ad60, "unknown_14008ad60", tpl_35::<0x143a20958, 0x143a2095b, 0x14307898c, 0x142409b50, 0x1425e5bb0, 0x141834d18> as *const ()), // sub_14008ad60
    Replacement::draft(0x14008adc0, "unknown_14008adc0", tpl_42::<0x143a21208, 0x143a2120b, 0x1430786e0, 0x142409b50, 0x1425e5bc0, 0x141834d18> as *const ()), // sub_14008adc0
    Replacement::draft(0x14008ae20, "unknown_14008ae20", tpl_38::<0x143a21118, 0x143a2111b, 0x143078998, 0x142409b50, 0x1425e5bd0, 0x141834d18> as *const ()), // sub_14008ae20
    Replacement::draft(0x14008ae80, "unknown_14008ae80", tpl_38::<0x143a20c70, 0x143a20c73, 0x1430789a8, 0x142409b50, 0x1425e5be0, 0x141834d18> as *const ()), // sub_14008ae80
    Replacement::draft(0x14008aee0, "unknown_14008aee0", tpl_41::<0x143a20dd0, 0x143a20dd3, 0x1430789b8, 0x142409b50, 0x1425e5bf0, 0x141834d18> as *const ()), // sub_14008aee0
    Replacement::draft(0x14008af40, "unknown_14008af40", tpl_34::<0x143a21258, 0x143a2125b, 0x1430789c8, 0x142409b50, 0x1425e5c00, 0x141834d18> as *const ()), // sub_14008af40
    Replacement::draft(0x14008afa0, "unknown_14008afa0", tpl_39::<0x143a20930, 0x143a20933, 0x1430789d8, 0x142409b50, 0x1425e5c10, 0x141834d18> as *const ()), // sub_14008afa0
    Replacement::draft(0x14008b000, "unknown_14008b000", tpl_41::<0x143a20c48, 0x143a20c4b, 0x1430789e0, 0x142409b50, 0x1425e5c20, 0x141834d18> as *const ()), // sub_14008b000
    Replacement::draft(0x14008b060, "unknown_14008b060", tpl_34::<0x143a20ee8, 0x143a20eeb, 0x1430789f0, 0x142409b50, 0x1425e5c30, 0x141834d18> as *const ()), // sub_14008b060
    Replacement::draft(0x14008b0c0, "unknown_14008b0c0", tpl_48::<0x143a20b80, 0x143a20b83, 0x143078a00, 0x142409b50, 0x1425e5c40, 0x141834d18> as *const ()), // sub_14008b0c0
    Replacement::draft(0x14008b120, "unknown_14008b120", tpl_46::<0x143a21000, 0x143a21003, 0x143078a18, 0x142409b50, 0x1425e5c50, 0x141834d18> as *const ()), // sub_14008b120
    Replacement::draft(0x14008b180, "unknown_14008b180", tpl_47::<0x143a20f60, 0x143a20f63, 0x143078a30, 0x142409b50, 0x1425e5c60, 0x141834d18> as *const ()), // sub_14008b180
    Replacement::draft(0x14008b1e0, "unknown_14008b1e0", tpl_37::<0x143a20ab8, 0x143a20abb, 0x143078a50, 0x142409b50, 0x1425e5c70, 0x141834d18> as *const ()), // sub_14008b1e0
    Replacement::draft(0x14008b240, "unknown_14008b240", tpl_45::<0x143a20d10, 0x143a20d13, 0x143078a60, 0x142409b50, 0x1425e5c80, 0x141834d18> as *const ()), // sub_14008b240
    Replacement::draft(0x14008b2a0, "unknown_14008b2a0", tpl_54::<0x143a20e20, 0x143a20e23, 0x143078a70, 0x142409b50, 0x1425e5c90, 0x141834d18> as *const ()), // sub_14008b2a0
    Replacement::draft(0x14008b300, "unknown_14008b300", tpl_40::<0x143a20a90, 0x143a20a93, 0x143078a88, 0x142409b50, 0x1425e5ca0, 0x141834d18> as *const ()), // sub_14008b300
    Replacement::draft(0x14008b360, "unknown_14008b360", tpl_50::<0x143a20ae0, 0x143a20ae3, 0x143078a90, 0x142409b50, 0x1425e5cb0, 0x141834d18> as *const ()), // sub_14008b360
    Replacement::draft(0x14008b3c0, "unknown_14008b3c0", tpl_41::<0x143a20c98, 0x143a20c9b, 0x143078aa8, 0x142409b50, 0x1425e5cc0, 0x141834d18> as *const ()), // sub_14008b3c0
    Replacement::draft(0x14008b420, "unknown_14008b420", tpl_55::<0x143a20f88, 0x143a20f8b, 0x143078ab8, 0x142409b50, 0x1425e5cd0, 0x141834d18> as *const ()), // sub_14008b420
    Replacement::draft(0x14008b480, "unknown_14008b480", tpl_41::<0x143a21028, 0x143a2102b, 0x143078ac8, 0x142409b50, 0x1425e5ce0, 0x141834d18> as *const ()), // sub_14008b480
    Replacement::draft(0x14008b4e0, "unknown_14008b4e0", tpl_49::<0x143a210c8, 0x143a210cb, 0x143078ad8, 0x142409b50, 0x1425e5cf0, 0x141834d18> as *const ()), // sub_14008b4e0
    Replacement::draft(0x14008b540, "unknown_14008b540", tpl_40::<0x143a208e0, 0x143a208e3, 0x143078ae4, 0x142409b50, 0x1425e5d00, 0x141834d18> as *const ()), // sub_14008b540
    Replacement::draft(0x14008b5a0, "unknown_14008b5a0", tpl_44::<0x143a20a18, 0x143a20a1b, 0x143078af0, 0x142409b50, 0x1425e5d10, 0x141834d18> as *const ()), // sub_14008b5a0
    Replacement::draft(0x14008b600, "unknown_14008b600", tpl_46::<0x143a208b8, 0x143a208bb, 0x143078b08, 0x142409b50, 0x1425e5d20, 0x141834d18> as *const ()), // sub_14008b600
    Replacement::draft(0x14008b660, "unknown_14008b660", tpl_55::<0x143a20fb0, 0x143a20fb3, 0x143078b20, 0x142409b50, 0x1425e5d30, 0x141834d18> as *const ()), // sub_14008b660
    Replacement::draft(0x14008b6c0, "unknown_14008b6c0", tpl_46::<0x143a20e48, 0x143a20e4b, 0x143078b30, 0x142409b50, 0x1425e5d40, 0x141834d18> as *const ()), // sub_14008b6c0
    Replacement::draft(0x14008b720, "unknown_14008b720", tpl_50::<0x143a20ba8, 0x143a20bab, 0x143078b48, 0x142409b50, 0x1425e5d50, 0x141834d18> as *const ()), // sub_14008b720
    Replacement::draft(0x14008b780, "unknown_14008b780", tpl_42::<0x143a20868, 0x143a2086b, 0x143078b60, 0x142409b50, 0x1425e5d60, 0x141834d18> as *const ()), // sub_14008b780
    Replacement::draft(0x14008b7e0, "unknown_14008b7e0", tpl_38::<0x143a20f10, 0x143a20f13, 0x143078b78, 0x142409b50, 0x1425e5d70, 0x141834d18> as *const ()), // sub_14008b7e0
    Replacement::draft(0x14008b840, "unknown_14008b840", tpl_34::<0x143a21280, 0x143a21283, 0x143078b88, 0x142409b50, 0x1425e5d80, 0x141834d18> as *const ()), // sub_14008b840
    Replacement::draft(0x14008b8a0, "unknown_14008b8a0", tpl_41::<0x143a21078, 0x143a2107b, 0x143078b98, 0x142409b50, 0x1425e5d90, 0x141834d18> as *const ()), // sub_14008b8a0
    Replacement::draft(0x14008b900, "unknown_14008b900", tpl_36::<0x143a20df8, 0x143a20dfb, 0x143078ba8, 0x142409b50, 0x1425e5da0, 0x141834d18> as *const ()), // sub_14008b900
    Replacement::draft(0x14008b960, "unknown_14008b960", tpl_34::<0x143a20890, 0x143a20893, 0x143078bb8, 0x142409b50, 0x1425e5db0, 0x141834d18> as *const ()), // sub_14008b960
    Replacement::draft(0x14008ba30, "unknown_14008ba30", tpl_61::<0x143a224b0, 0x143a224b3, 0x143091fd8, 0x142409b50, 0x1425e5df0, 0x141834d18> as *const ()), // sub_14008ba30
    Replacement::draft(0x14008ba90, "unknown_14008ba90", tpl_62::<0x143a224b3, 0x143a224b0, 0x143a224b8, 0x143a224b4, 0x143a21890> as *const ()), // sub_14008ba90
    Replacement::draft(0x14008bb20, "unknown_14008bb20", tpl_63::<0x143a22440, 0x143a22443, 0x143091ff0, 0x142409b50, 0x1425e5e00, 0x141834d18> as *const ()), // sub_14008bb20
    Replacement::draft(0x14008bb80, "unknown_14008bb80", tpl_64::<0x143a21b30, 0x143a21b33, 0x143091fa0, 0x142409b50, 0x1425e5e10, 0x141834d18> as *const ()), // sub_14008bb80
    Replacement::draft(0x14008bbe0, "unknown_14008bbe0", tpl_65::<0x143a218a0, 0x143a218a3, 0x143091fc0, 0x142409b50, 0x1425e5e20, 0x141834d18> as *const ()), // sub_14008bbe0
    Replacement::draft(0x14008bc40, "unknown_14008bc40", tpl_62::<0x143a22443, 0x143a22440, 0x143a22448, 0x143a22444, 0x143a22288> as *const ()), // sub_14008bc40
    Replacement::draft(0x14008bcd0, "unknown_14008bcd0", tpl_62::<0x143a21b33, 0x143a21b30, 0x143a21b38, 0x143a21b34, 0x143a218e8> as *const ()), // sub_14008bcd0
    Replacement::draft(0x14008bd60, "unknown_14008bd60", tpl_62::<0x143a218a3, 0x143a218a0, 0x143a218a8, 0x143a218a4, 0x143a218f0> as *const ()), // sub_14008bd60
    Replacement::draft(0x14008bdf0, "unknown_14008bdf0", tpl_23::<0x143a21550, 0x143a21553, 0x1426cdcb0, 0x142409b50, 0x1425e5e30, 0x141834d18> as *const ()), // sub_14008bdf0
    Replacement::draft(0x14008be50, "unknown_14008be50", tpl_23::<0x143a21598, 0x143a2159b, 0x1426cdcb0, 0x142409b50, 0x1425e5e40, 0x141834d18> as *const ()), // sub_14008be50
    Replacement::draft(0x14008c040, "unknown_14008c040", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848318, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e5e80, 0x141834d18> as *const ()), // sub_14008c040
    Replacement::draft(0x14008c4e0, "unknown_14008c4e0", tpl_33::<0x143428dc0, 0x143a221b0> as *const ()), // sub_14008c4e0
    Replacement::draft(0x14008c500, "unknown_14008c500", tpl_33::<0x143428da0, 0x143a21f30> as *const ()), // sub_14008c500
    Replacement::draft(0x14008c520, "unknown_14008c520", tpl_51::<0x143078928, 0x143078938, 0x143a21510, 0x143078944, 0x1430782bc, 0x1430782b4, 0x1401df440, 0x1425e5ef0, 0x141834d18> as *const ()), // sub_14008c520
    Replacement::draft(0x14008c5b0, "unknown_14008c5b0", tpl_34::<0x143a217c8, 0x143a217cb, 0x143078668, 0x142409b50, 0x1425e5f20, 0x141834d18> as *const ()), // sub_14008c5b0
    Replacement::draft(0x14008c610, "unknown_14008c610", tpl_23::<0x143a22580, 0x143a22583, 0x1426cdcb0, 0x142409b50, 0x1425e5f30, 0x141834d18> as *const ()), // sub_14008c610
    Replacement::draft(0x14008c670, "unknown_14008c670", tpl_35::<0x143a21eb0, 0x143a21eb3, 0x143078678, 0x142409b50, 0x1425e5f40, 0x141834d18> as *const ()), // sub_14008c670
    Replacement::draft(0x14008c6d0, "unknown_14008c6d0", tpl_36::<0x143a22260, 0x143a22263, 0x143078680, 0x142409b50, 0x1425e5f50, 0x141834d18> as *const ()), // sub_14008c6d0
    Replacement::draft(0x14008c730, "unknown_14008c730", tpl_37::<0x143a21be8, 0x143a21beb, 0x143078690, 0x142409b50, 0x1425e5f60, 0x141834d18> as *const ()), // sub_14008c730
    Replacement::draft(0x14008c790, "unknown_14008c790", tpl_38::<0x143a21978, 0x143a2197b, 0x1430786a0, 0x142409b50, 0x1425e5f70, 0x141834d18> as *const ()), // sub_14008c790
    Replacement::draft(0x14008c7f0, "unknown_14008c7f0", tpl_39::<0x143a226b0, 0x143a226b3, 0x1430786b0, 0x142409b50, 0x1425e5f80, 0x141834d18> as *const ()), // sub_14008c7f0
    Replacement::draft(0x14008c850, "unknown_14008c850", tpl_37::<0x143a21778, 0x143a2177b, 0x1430786b8, 0x142409b50, 0x1425e5f90, 0x141834d18> as *const ()), // sub_14008c850
    Replacement::draft(0x14008c8b0, "unknown_14008c8b0", tpl_40::<0x143a22850, 0x143a22853, 0x1430786c4, 0x142409b50, 0x1425e5fa0, 0x141834d18> as *const ()), // sub_14008c8b0
    Replacement::draft(0x14008c910, "unknown_14008c910", tpl_41::<0x143a21a40, 0x143a21a43, 0x1430786d0, 0x142409b50, 0x1425e5fb0, 0x141834d18> as *const ()), // sub_14008c910
    Replacement::draft(0x14008c970, "unknown_14008c970", tpl_42::<0x143a22740, 0x143a22743, 0x1430786e0, 0x142409b50, 0x1425e5fc0, 0x141834d18> as *const ()), // sub_14008c970
    Replacement::draft(0x14008c9d0, "unknown_14008c9d0", tpl_34::<0x143a22790, 0x143a22793, 0x1430786f8, 0x142409b50, 0x1425e5fd0, 0x141834d18> as *const ()), // sub_14008c9d0
    Replacement::draft(0x14008ca30, "unknown_14008ca30", tpl_34::<0x143a217a0, 0x143a217a3, 0x143078708, 0x142409b50, 0x1425e5fe0, 0x141834d18> as *const ()), // sub_14008ca30
    Replacement::draft(0x14008ca90, "unknown_14008ca90", tpl_41::<0x143a219a0, 0x143a219a3, 0x143078718, 0x142409b50, 0x1425e5ff0, 0x141834d18> as *const ()), // sub_14008ca90
    Replacement::draft(0x14008caf0, "unknown_14008caf0", tpl_43::<0x143a21c60, 0x143a21c63, 0x143078728, 0x142409b50, 0x1425e6000, 0x141834d18> as *const ()), // sub_14008caf0
    Replacement::draft(0x14008cb50, "unknown_14008cb50", tpl_44::<0x143a21378, 0x143a2137b, 0x143078748, 0x142409b50, 0x1425e6010, 0x141834d18> as *const ()), // sub_14008cb50
    Replacement::draft(0x14008cbb0, "unknown_14008cbb0", tpl_39::<0x143a21ac0, 0x143a21ac3, 0x143078760, 0x142409b50, 0x1425e6020, 0x141834d18> as *const ()), // sub_14008cbb0
    Replacement::draft(0x14008cc10, "unknown_14008cc10", tpl_45::<0x143a21950, 0x143a21953, 0x143078768, 0x142409b50, 0x1425e6030, 0x141834d18> as *const ()), // sub_14008cc10
    Replacement::draft(0x14008cc70, "unknown_14008cc70", tpl_41::<0x143a22210, 0x143a22213, 0x143078778, 0x142409b50, 0x1425e6040, 0x141834d18> as *const ()), // sub_14008cc70
    Replacement::draft(0x14008ccd0, "unknown_14008ccd0", tpl_37::<0x143a213a0, 0x143a213a3, 0x143078788, 0x142409b50, 0x1425e6050, 0x141834d18> as *const ()), // sub_14008ccd0
    Replacement::draft(0x14008cd30, "unknown_14008cd30", tpl_46::<0x143a21fc0, 0x143a21fc3, 0x143078798, 0x142409b50, 0x1425e6060, 0x141834d18> as *const ()), // sub_14008cd30
    Replacement::draft(0x14008cd90, "unknown_14008cd90", tpl_47::<0x143a21610, 0x143a21613, 0x1430787b0, 0x142409b50, 0x1425e6070, 0x141834d18> as *const ()), // sub_14008cd90
    Replacement::draft(0x14008cdf0, "unknown_14008cdf0", tpl_38::<0x143a22060, 0x143a22063, 0x1430787d0, 0x142409b50, 0x1425e6080, 0x141834d18> as *const ()), // sub_14008cdf0
    Replacement::draft(0x14008ce50, "unknown_14008ce50", tpl_48::<0x143a226d8, 0x143a226db, 0x1430787e0, 0x142409b50, 0x1425e6090, 0x141834d18> as *const ()), // sub_14008ce50
    Replacement::draft(0x14008ceb0, "unknown_14008ceb0", tpl_49::<0x143a21e88, 0x143a21e8b, 0x1430787f8, 0x142409b50, 0x1425e60a0, 0x141834d18> as *const ()), // sub_14008ceb0
    Replacement::draft(0x14008cf10, "unknown_14008cf10", tpl_44::<0x143a22488, 0x143a2248b, 0x143078808, 0x142409b50, 0x1425e60b0, 0x141834d18> as *const ()), // sub_14008cf10
    Replacement::draft(0x14008cf70, "unknown_14008cf70", tpl_42::<0x143a21350, 0x143a21353, 0x143078820, 0x142409b50, 0x1425e60c0, 0x141834d18> as *const ()), // sub_14008cf70
    Replacement::draft(0x14008cfd0, "unknown_14008cfd0", tpl_50::<0x143a22160, 0x143a22163, 0x143078838, 0x142409b50, 0x1425e60d0, 0x141834d18> as *const ()), // sub_14008cfd0
    Replacement::draft(0x14008d030, "unknown_14008d030", tpl_52::<0x143a214e0, 0x143a214e3, 0x143a22850, 0x143a22854, 0x143a22853, 0x143a22858, 0x142409b50, 0x1425e60e0, 0x141834d18> as *const ()), // sub_14008d030
    Replacement::draft(0x14008d0f0, "unknown_14008d0f0", tpl_38::<0x143a21ed8, 0x143a21edb, 0x143078950, 0x142409b50, 0x1425e60f0, 0x141834d18> as *const ()), // sub_14008d0f0
    Replacement::draft(0x14008d150, "unknown_14008d150", tpl_53::<0x143a215e8, 0x143a215eb, 0x143078960, 0x142409b50, 0x1425e6100, 0x141834d18> as *const ()), // sub_14008d150
    Replacement::draft(0x14008d1b0, "unknown_14008d1b0", tpl_45::<0x143a21468, 0x143a2146b, 0x143078980, 0x142409b50, 0x1425e6110, 0x141834d18> as *const ()), // sub_14008d1b0
    Replacement::draft(0x14008d210, "unknown_14008d210", tpl_35::<0x143a214b8, 0x143a214bb, 0x14307898c, 0x142409b50, 0x1425e6120, 0x141834d18> as *const ()), // sub_14008d210
    Replacement::draft(0x14008d270, "unknown_14008d270", tpl_42::<0x143a22768, 0x143a2276b, 0x1430786e0, 0x142409b50, 0x1425e6130, 0x141834d18> as *const ()), // sub_14008d270
    Replacement::draft(0x14008d2d0, "unknown_14008d2d0", tpl_38::<0x143a22390, 0x143a22393, 0x143078998, 0x142409b50, 0x1425e6140, 0x141834d18> as *const ()), // sub_14008d2d0
    Replacement::draft(0x14008d330, "unknown_14008d330", tpl_38::<0x143a219f0, 0x143a219f3, 0x1430789a8, 0x142409b50, 0x1425e6150, 0x141834d18> as *const ()), // sub_14008d330
    Replacement::draft(0x14008d390, "unknown_14008d390", tpl_41::<0x143a21d50, 0x143a21d53, 0x1430789b8, 0x142409b50, 0x1425e6160, 0x141834d18> as *const ()), // sub_14008d390
    Replacement::draft(0x14008d3f0, "unknown_14008d3f0", tpl_34::<0x143a227b8, 0x143a227bb, 0x1430789c8, 0x142409b50, 0x1425e6170, 0x141834d18> as *const ()), // sub_14008d3f0
    Replacement::draft(0x14008d450, "unknown_14008d450", tpl_39::<0x143a21490, 0x143a21493, 0x1430789d8, 0x142409b50, 0x1425e6180, 0x141834d18> as *const ()), // sub_14008d450
    Replacement::draft(0x14008d4b0, "unknown_14008d4b0", tpl_41::<0x143a219c8, 0x143a219cb, 0x1430789e0, 0x142409b50, 0x1425e6190, 0x141834d18> as *const ()), // sub_14008d4b0
    Replacement::draft(0x14008d510, "unknown_14008d510", tpl_34::<0x143a21f00, 0x143a21f03, 0x1430789f0, 0x142409b50, 0x1425e61a0, 0x141834d18> as *const ()), // sub_14008d510
    Replacement::draft(0x14008d570, "unknown_14008d570", tpl_48::<0x143a217f0, 0x143a217f3, 0x143078a00, 0x142409b50, 0x1425e61b0, 0x141834d18> as *const ()), // sub_14008d570
    Replacement::draft(0x14008d5d0, "unknown_14008d5d0", tpl_46::<0x143a220d0, 0x143a220d3, 0x143078a18, 0x142409b50, 0x1425e61c0, 0x141834d18> as *const ()), // sub_14008d5d0
    Replacement::draft(0x14008d630, "unknown_14008d630", tpl_47::<0x143a21fe8, 0x143a21feb, 0x143078a30, 0x142409b50, 0x1425e61d0, 0x141834d18> as *const ()), // sub_14008d630
    Replacement::draft(0x14008d690, "unknown_14008d690", tpl_37::<0x143a216a8, 0x143a216ab, 0x143078a50, 0x142409b50, 0x1425e61e0, 0x141834d18> as *const ()), // sub_14008d690
    Replacement::draft(0x14008d6f0, "unknown_14008d6f0", tpl_45::<0x143a21bc0, 0x143a21bc3, 0x143078a60, 0x142409b50, 0x1425e61f0, 0x141834d18> as *const ()), // sub_14008d6f0
    Replacement::draft(0x14008d750, "unknown_14008d750", tpl_54::<0x143a21da0, 0x143a21da3, 0x143078a70, 0x142409b50, 0x1425e6200, 0x141834d18> as *const ()), // sub_14008d750
    Replacement::draft(0x14008d7b0, "unknown_14008d7b0", tpl_40::<0x143a21680, 0x143a21683, 0x143078a88, 0x142409b50, 0x1425e6210, 0x141834d18> as *const ()), // sub_14008d7b0
    Replacement::draft(0x14008d810, "unknown_14008d810", tpl_50::<0x143a21750, 0x143a21753, 0x143078a90, 0x142409b50, 0x1425e6220, 0x141834d18> as *const ()), // sub_14008d810
    Replacement::draft(0x14008d870, "unknown_14008d870", tpl_41::<0x143a21a18, 0x143a21a1b, 0x143078aa8, 0x142409b50, 0x1425e6230, 0x141834d18> as *const ()), // sub_14008d870
    Replacement::draft(0x14008d8d0, "unknown_14008d8d0", tpl_55::<0x143a22010, 0x143a22013, 0x143078ab8, 0x142409b50, 0x1425e6240, 0x141834d18> as *const ()), // sub_14008d8d0
    Replacement::draft(0x14008d930, "unknown_14008d930", tpl_41::<0x143a220f8, 0x143a220fb, 0x143078ac8, 0x142409b50, 0x1425e6250, 0x141834d18> as *const ()), // sub_14008d930
    Replacement::draft(0x14008d990, "unknown_14008d990", tpl_49::<0x143a22238, 0x143a2223b, 0x143078ad8, 0x142409b50, 0x1425e6260, 0x141834d18> as *const ()), // sub_14008d990
    Replacement::draft(0x14008d9f0, "unknown_14008d9f0", tpl_40::<0x143a21440, 0x143a21443, 0x143078ae4, 0x142409b50, 0x1425e6270, 0x141834d18> as *const ()), // sub_14008d9f0
    Replacement::draft(0x14008da50, "unknown_14008da50", tpl_44::<0x143a215c0, 0x143a215c3, 0x143078af0, 0x142409b50, 0x1425e6280, 0x141834d18> as *const ()), // sub_14008da50
    Replacement::draft(0x14008dab0, "unknown_14008dab0", tpl_46::<0x143a21418, 0x143a2141b, 0x143078b08, 0x142409b50, 0x1425e6290, 0x141834d18> as *const ()), // sub_14008dab0
    Replacement::draft(0x14008db10, "unknown_14008db10", tpl_55::<0x143a22038, 0x143a2203b, 0x143078b20, 0x142409b50, 0x1425e62a0, 0x141834d18> as *const ()), // sub_14008db10
    Replacement::draft(0x14008db70, "unknown_14008db70", tpl_46::<0x143a21e60, 0x143a21e63, 0x143078b30, 0x142409b50, 0x1425e62b0, 0x141834d18> as *const ()), // sub_14008db70
    Replacement::draft(0x14008dbd0, "unknown_14008dbd0", tpl_50::<0x143a21818, 0x143a2181b, 0x143078b48, 0x142409b50, 0x1425e62c0, 0x141834d18> as *const ()), // sub_14008dbd0
    Replacement::draft(0x14008dc30, "unknown_14008dc30", tpl_42::<0x143a213c8, 0x143a213cb, 0x143078b60, 0x142409b50, 0x1425e62d0, 0x141834d18> as *const ()), // sub_14008dc30
    Replacement::draft(0x14008dc90, "unknown_14008dc90", tpl_38::<0x143a21f50, 0x143a21f53, 0x143078b78, 0x142409b50, 0x1425e62e0, 0x141834d18> as *const ()), // sub_14008dc90
    Replacement::draft(0x14008dcf0, "unknown_14008dcf0", tpl_34::<0x143a227e0, 0x143a227e3, 0x143078b88, 0x142409b50, 0x1425e62f0, 0x141834d18> as *const ()), // sub_14008dcf0
    Replacement::draft(0x14008dd50, "unknown_14008dd50", tpl_41::<0x143a22188, 0x143a2218b, 0x143078b98, 0x142409b50, 0x1425e6300, 0x141834d18> as *const ()), // sub_14008dd50
    Replacement::draft(0x14008ddb0, "unknown_14008ddb0", tpl_36::<0x143a21d78, 0x143a21d7b, 0x143078ba8, 0x142409b50, 0x1425e6310, 0x141834d18> as *const ()), // sub_14008ddb0
    Replacement::draft(0x14008de10, "unknown_14008de10", tpl_34::<0x143a213f0, 0x143a213f3, 0x143078bb8, 0x142409b50, 0x1425e6320, 0x141834d18> as *const ()), // sub_14008de10
    Replacement::draft(0x14008de70, "unknown_14008de70", fn_14008de70 as *const ()), // sub_14008de70
    Replacement::draft(0x14008ea30, "unknown_14008ea30", fn_14008ea30 as *const ()), // sub_14008ea30
    Replacement::draft(0x14008fc80, "unknown_14008fc80", fn_14008fc80 as *const ()), // sub_14008fc80
    Replacement::draft(0x14008fcf0, "unknown_14008fcf0", fn_14008fcf0 as *const ()), // sub_14008fcf0
    Replacement::draft(0x14008fe40, "unknown_14008fe40", tpl_94::<0x143109a38, 0x143109a28, 0x143a216d0, 0x143109a10, 0x1431099f8, 0x1403f8780, 0x1425e6420, 0x141834d18> as *const ()), // sub_14008fe40
    Replacement::draft(0x14008fec0, "unknown_14008fec0", fn_14008fec0 as *const ()), // sub_14008fec0
    Replacement::draft(0x140090390, "unknown_140090390", tpl_96::<0x14309f914, 0x14309f90c, 0x143a22670, 0x143109b3c, 0x143109bf8, 0x143109ad4, 0x143109be8, 0x1403f88a0, 0x1425e6480, 0x141834d18> as *const ()), // sub_140090390
    Replacement::draft(0x140090430, "unknown_140090430", fn_140090430 as *const ()), // sub_140090430
    Replacement::draft(0x140090980, "unknown_140090980", fn_140090980 as *const ()), // sub_140090980
    Replacement::draft(0x140090a00, "unknown_140090a00", fn_140090a00 as *const ()), // sub_140090a00
    Replacement::draft(0x140090af0, "unknown_140090af0", tpl_100::<0x143109924, 0x14310a190, 0x143a22700, 0x14310a180, 0x143109bc8, 0x1403f8780, 0x1425e65a0, 0x141834d18> as *const ()), // sub_140090af0
    Replacement::draft(0x140090b70, "unknown_140090b70", tpl_100::<0x14309f824, 0x14310a3b4, 0x143a222d0, 0x143107164, 0x14310a3ac, 0x1403f8780, 0x1425e65d0, 0x141834d18> as *const ()), // sub_140090b70
    Replacement::draft(0x140090bf0, "unknown_140090bf0", tpl_101::<0x14310a300, 0x143a22810, 0x1403f9250, 0x1425e6600, 0x141834d18> as *const ()), // sub_140090bf0
    Replacement::draft(0x140090c30, "unknown_140090c30", tpl_102::<0x143109c0c, 0x143109c00, 0x143a22540, 0x1403f9a70, 0x1425e6630, 0x141834d18> as *const ()), // sub_140090c30
    Replacement::draft(0x140090c80, "unknown_140090c80", fn_140090c80 as *const ()), // sub_140090c80
    Replacement::draft(0x140090d20, "unknown_140090d20", tpl_104::<0x14310a0b8, 0x14310a0a8, 0x143a21910, 0x1403f9a70, 0x1425e6690, 0x141834d18> as *const ()), // sub_140090d20
    Replacement::draft(0x140090d70, "unknown_140090d70", tpl_101::<0x14310a0a0, 0x143a22500, 0x1403f9250, 0x1425e66c0, 0x141834d18> as *const ()), // sub_140090d70
    Replacement::draft(0x140090db0, "unknown_140090db0", tpl_104::<0x14310a098, 0x14310a090, 0x143a212d0, 0x1403f9a70, 0x1425e66f0, 0x141834d18> as *const ()), // sub_140090db0
    Replacement::draft(0x140090e00, "unknown_140090e00", fn_140090e00 as *const ()), // sub_140090e00
    Replacement::draft(0x140090e70, "unknown_140090e70", tpl_102::<0x14310a8b8, 0x14310a8b0, 0x143a21310, 0x1403f9a70, 0x1425e6750, 0x141834d18> as *const ()), // sub_140090e70
    Replacement::draft(0x140090ec0, "unknown_140090ec0", tpl_51::<0x14310a8a8, 0x14310a9e8, 0x143a21dd0, 0x14310a9d8, 0x14310a9c8, 0x14310a9c0, 0x1401df440, 0x1425e6780, 0x141834d18> as *const ()), // sub_140090ec0
    Replacement::draft(0x140090f50, "unknown_140090f50", tpl_102::<0x14310a684, 0x14310a7d8, 0x143a223c0, 0x1403f9a70, 0x1425e67b0, 0x141834d18> as *const ()), // sub_140090f50
    Replacement::draft(0x140090fa0, "unknown_140090fa0", fn_140090fa0 as *const ()), // sub_140090fa0
    Replacement::draft(0x140091010, "unknown_140091010", tpl_107::<0x14310a7b8, 0x14310a7b4, 0x143a22310, 0x14310a7a8, 0x1401de5b0, 0x1425e6810, 0x141834d18> as *const ()), // sub_140091010
    Replacement::draft(0x140091080, "unknown_140091080", tpl_84::<0x14310b164, 0x143109ba4, 0x143a22400, 0x143109b98, 0x14310b158, 0x143109720, 0x14310a190, 0x14310ac90, 0x143109bc8, 0x14310ac88, 0x14038be40, 0x1425e6840, 0x141834d18> as *const ()), // sub_140091080
    Replacement::draft(0x140091160, "unknown_140091160", fn_140091160 as *const ()), // sub_140091160
    Replacement::draft(0x140091240, "unknown_140091240", fn_140091240 as *const ()), // sub_140091240
    Replacement::draft(0x1400916e0, "unknown_1400916e0", fn_1400916e0 as *const ()), // sub_1400916e0
    Replacement::draft(0x140091780, "unknown_140091780", tpl_96::<0x14310c2c8, 0x14310c404, 0x143a21af0, 0x14310c3f8, 0x14310c3d8, 0x14310c3b8, 0x14310c3a8, 0x1403f88a0, 0x1425e6900, 0x141834d18> as *const ()), // sub_140091780
    Replacement::draft(0x140091820, "unknown_140091820", tpl_51::<0x14310d064, 0x143107164, 0x143a22290, 0x14310d1a0, 0x14310d198, 0x14310a9c0, 0x1401df440, 0x1425e6930, 0x141834d18> as *const ()), // sub_140091820
    Replacement::draft(0x14009c0f0, "unknown_14009c0f0", tpl_34::<0x143a22be0, 0x143a22be3, 0x143078668, 0x142409b50, 0x1425e73b0, 0x141834d18> as *const ()), // sub_14009c0f0
    Replacement::draft(0x14009c150, "unknown_14009c150", tpl_23::<0x143a231a8, 0x143a231ab, 0x1426cdcb0, 0x142409b50, 0x1425e73c0, 0x141834d18> as *const ()), // sub_14009c150
    Replacement::draft(0x14009c1b0, "unknown_14009c1b0", tpl_35::<0x143a22ed8, 0x143a22edb, 0x143078678, 0x142409b50, 0x1425e73d0, 0x141834d18> as *const ()), // sub_14009c1b0
    Replacement::draft(0x14009c210, "unknown_14009c210", tpl_36::<0x143a23130, 0x143a23133, 0x143078680, 0x142409b50, 0x1425e73e0, 0x141834d18> as *const ()), // sub_14009c210
    Replacement::draft(0x14009c270, "unknown_14009c270", tpl_37::<0x143a22dc0, 0x143a22dc3, 0x143078690, 0x142409b50, 0x1425e73f0, 0x141834d18> as *const ()), // sub_14009c270
    Replacement::draft(0x14009c2d0, "unknown_14009c2d0", tpl_38::<0x143a22c80, 0x143a22c83, 0x1430786a0, 0x142409b50, 0x1425e7400, 0x141834d18> as *const ()), // sub_14009c2d0
    Replacement::draft(0x14009c330, "unknown_14009c330", tpl_39::<0x143a231d0, 0x143a231d3, 0x1430786b0, 0x142409b50, 0x1425e7410, 0x141834d18> as *const ()), // sub_14009c330
    Replacement::draft(0x14009c390, "unknown_14009c390", tpl_37::<0x143a22b90, 0x143a22b93, 0x1430786b8, 0x142409b50, 0x1425e7420, 0x141834d18> as *const ()), // sub_14009c390
    Replacement::draft(0x14009c3f0, "unknown_14009c3f0", tpl_40::<0x143a232e8, 0x143a232eb, 0x1430786c4, 0x142409b50, 0x1425e7430, 0x141834d18> as *const ()), // sub_14009c3f0
    Replacement::draft(0x14009c450, "unknown_14009c450", tpl_41::<0x143a22d48, 0x143a22d4b, 0x1430786d0, 0x142409b50, 0x1425e7440, 0x141834d18> as *const ()), // sub_14009c450
    Replacement::draft(0x14009c4b0, "unknown_14009c4b0", tpl_42::<0x143a23220, 0x143a23223, 0x1430786e0, 0x142409b50, 0x1425e7450, 0x141834d18> as *const ()), // sub_14009c4b0
    Replacement::draft(0x14009c510, "unknown_14009c510", tpl_34::<0x143a23270, 0x143a23273, 0x1430786f8, 0x142409b50, 0x1425e7460, 0x141834d18> as *const ()), // sub_14009c510
    Replacement::draft(0x14009c570, "unknown_14009c570", tpl_34::<0x143a22bb8, 0x143a22bbb, 0x143078708, 0x142409b50, 0x1425e7470, 0x141834d18> as *const ()), // sub_14009c570
    Replacement::draft(0x14009c5d0, "unknown_14009c5d0", tpl_41::<0x143a22ca8, 0x143a22cab, 0x143078718, 0x142409b50, 0x1425e7480, 0x141834d18> as *const ()), // sub_14009c5d0
    Replacement::draft(0x14009c630, "unknown_14009c630", tpl_43::<0x143a22de8, 0x143a22deb, 0x143078728, 0x142409b50, 0x1425e7490, 0x141834d18> as *const ()), // sub_14009c630
    Replacement::draft(0x14009c690, "unknown_14009c690", tpl_44::<0x143a228a0, 0x143a228a3, 0x143078748, 0x142409b50, 0x1425e74a0, 0x141834d18> as *const ()), // sub_14009c690
    Replacement::draft(0x14009c6f0, "unknown_14009c6f0", tpl_39::<0x143a22d70, 0x143a22d73, 0x143078760, 0x142409b50, 0x1425e74b0, 0x141834d18> as *const ()), // sub_14009c6f0
    Replacement::draft(0x14009c750, "unknown_14009c750", tpl_45::<0x143a22c58, 0x143a22c5b, 0x143078768, 0x142409b50, 0x1425e74c0, 0x141834d18> as *const ()), // sub_14009c750
    Replacement::draft(0x14009c7b0, "unknown_14009c7b0", tpl_41::<0x143a230e0, 0x143a230e3, 0x143078778, 0x142409b50, 0x1425e74d0, 0x141834d18> as *const ()), // sub_14009c7b0
    Replacement::draft(0x14009c810, "unknown_14009c810", tpl_37::<0x143a228c8, 0x143a228cb, 0x143078788, 0x142409b50, 0x1425e74e0, 0x141834d18> as *const ()), // sub_14009c810
    Replacement::draft(0x14009c870, "unknown_14009c870", tpl_46::<0x143a22f78, 0x143a22f7b, 0x143078798, 0x142409b50, 0x1425e74f0, 0x141834d18> as *const ()), // sub_14009c870
    Replacement::draft(0x14009c8d0, "unknown_14009c8d0", tpl_47::<0x143a22af0, 0x143a22af3, 0x1430787b0, 0x142409b50, 0x1425e7500, 0x141834d18> as *const ()), // sub_14009c8d0
    Replacement::draft(0x14009c930, "unknown_14009c930", tpl_38::<0x143a23018, 0x143a2301b, 0x1430787d0, 0x142409b50, 0x1425e7510, 0x141834d18> as *const ()), // sub_14009c930
    Replacement::draft(0x14009c990, "unknown_14009c990", tpl_48::<0x143a231f8, 0x143a231fb, 0x1430787e0, 0x142409b50, 0x1425e7520, 0x141834d18> as *const ()), // sub_14009c990
    Replacement::draft(0x14009c9f0, "unknown_14009c9f0", tpl_49::<0x143a22eb0, 0x143a22eb3, 0x1430787f8, 0x142409b50, 0x1425e7530, 0x141834d18> as *const ()), // sub_14009c9f0
    Replacement::draft(0x14009ca50, "unknown_14009ca50", tpl_44::<0x143a23180, 0x143a23183, 0x143078808, 0x142409b50, 0x1425e7540, 0x141834d18> as *const ()), // sub_14009ca50
    Replacement::draft(0x14009cab0, "unknown_14009cab0", tpl_42::<0x143a22878, 0x143a2287b, 0x143078820, 0x142409b50, 0x1425e7550, 0x141834d18> as *const ()), // sub_14009cab0
    Replacement::draft(0x14009cb10, "unknown_14009cb10", tpl_50::<0x143a23090, 0x143a23093, 0x143078838, 0x142409b50, 0x1425e7560, 0x141834d18> as *const ()), // sub_14009cb10
    Replacement::draft(0x14009cb70, "unknown_14009cb70", tpl_23::<0x143a22a30, 0x143a22a33, 0x1426cdcb0, 0x142409b50, 0x1425e7570, 0x141834d18> as *const ()), // sub_14009cb70
    Replacement::draft(0x14009cbd0, "unknown_14009cbd0", tpl_23::<0x143a22a78, 0x143a22a7b, 0x1426cdcb0, 0x142409b50, 0x1425e7580, 0x141834d18> as *const ()), // sub_14009cbd0
    Replacement::draft(0x14009cdc0, "unknown_14009cdc0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848528, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e75c0, 0x141834d18> as *const ()), // sub_14009cdc0
    Replacement::draft(0x14009d150, "unknown_14009d150", tpl_52::<0x143a22a08, 0x143a22a0b, 0x143a232e8, 0x143a232ec, 0x143a232eb, 0x143a232f0, 0x142409b50, 0x1425e7600, 0x141834d18> as *const ()), // sub_14009d150
    Replacement::draft(0x14009d210, "unknown_14009d210", tpl_38::<0x143a22f00, 0x143a22f03, 0x143078950, 0x142409b50, 0x1425e7610, 0x141834d18> as *const ()), // sub_14009d210
    Replacement::draft(0x14009d270, "unknown_14009d270", tpl_53::<0x143a22ac8, 0x143a22acb, 0x143078960, 0x142409b50, 0x1425e7620, 0x141834d18> as *const ()), // sub_14009d270
    Replacement::draft(0x14009d2d0, "unknown_14009d2d0", tpl_45::<0x143a22990, 0x143a22993, 0x143078980, 0x142409b50, 0x1425e7630, 0x141834d18> as *const ()), // sub_14009d2d0
    Replacement::draft(0x14009d330, "unknown_14009d330", tpl_35::<0x143a229e0, 0x143a229e3, 0x14307898c, 0x142409b50, 0x1425e7640, 0x141834d18> as *const ()), // sub_14009d330
    Replacement::draft(0x14009d390, "unknown_14009d390", tpl_42::<0x143a23248, 0x143a2324b, 0x1430786e0, 0x142409b50, 0x1425e7650, 0x141834d18> as *const ()), // sub_14009d390
    Replacement::draft(0x14009d3f0, "unknown_14009d3f0", tpl_38::<0x143a23158, 0x143a2315b, 0x143078998, 0x142409b50, 0x1425e7660, 0x141834d18> as *const ()), // sub_14009d3f0
    Replacement::draft(0x14009d450, "unknown_14009d450", tpl_38::<0x143a22cf8, 0x143a22cfb, 0x1430789a8, 0x142409b50, 0x1425e7670, 0x141834d18> as *const ()), // sub_14009d450
    Replacement::draft(0x14009d4b0, "unknown_14009d4b0", tpl_41::<0x143a22e10, 0x143a22e13, 0x1430789b8, 0x142409b50, 0x1425e7680, 0x141834d18> as *const ()), // sub_14009d4b0
    Replacement::draft(0x14009d510, "unknown_14009d510", tpl_34::<0x143a23298, 0x143a2329b, 0x1430789c8, 0x142409b50, 0x1425e7690, 0x141834d18> as *const ()), // sub_14009d510
    Replacement::draft(0x14009d570, "unknown_14009d570", tpl_39::<0x143a229b8, 0x143a229bb, 0x1430789d8, 0x142409b50, 0x1425e76a0, 0x141834d18> as *const ()), // sub_14009d570
    Replacement::draft(0x14009d5d0, "unknown_14009d5d0", tpl_41::<0x143a22cd0, 0x143a22cd3, 0x1430789e0, 0x142409b50, 0x1425e76b0, 0x141834d18> as *const ()), // sub_14009d5d0
    Replacement::draft(0x14009d630, "unknown_14009d630", tpl_34::<0x143a22f28, 0x143a22f2b, 0x1430789f0, 0x142409b50, 0x1425e76c0, 0x141834d18> as *const ()), // sub_14009d630
    Replacement::draft(0x14009d690, "unknown_14009d690", tpl_48::<0x143a22c08, 0x143a22c0b, 0x143078a00, 0x142409b50, 0x1425e76d0, 0x141834d18> as *const ()), // sub_14009d690
    Replacement::draft(0x14009d6f0, "unknown_14009d6f0", tpl_46::<0x143a23040, 0x143a23043, 0x143078a18, 0x142409b50, 0x1425e76e0, 0x141834d18> as *const ()), // sub_14009d6f0
    Replacement::draft(0x14009d750, "unknown_14009d750", tpl_47::<0x143a22fa0, 0x143a22fa3, 0x143078a30, 0x142409b50, 0x1425e76f0, 0x141834d18> as *const ()), // sub_14009d750
    Replacement::draft(0x14009d7b0, "unknown_14009d7b0", tpl_37::<0x143a22b40, 0x143a22b43, 0x143078a50, 0x142409b50, 0x1425e7700, 0x141834d18> as *const ()), // sub_14009d7b0
    Replacement::draft(0x14009d810, "unknown_14009d810", tpl_45::<0x143a22d98, 0x143a22d9b, 0x143078a60, 0x142409b50, 0x1425e7710, 0x141834d18> as *const ()), // sub_14009d810
    Replacement::draft(0x14009d870, "unknown_14009d870", tpl_54::<0x143a22e60, 0x143a22e63, 0x143078a70, 0x142409b50, 0x1425e7720, 0x141834d18> as *const ()), // sub_14009d870
    Replacement::draft(0x14009d8d0, "unknown_14009d8d0", tpl_40::<0x143a22b18, 0x143a22b1b, 0x143078a88, 0x142409b50, 0x1425e7730, 0x141834d18> as *const ()), // sub_14009d8d0
    Replacement::draft(0x14009d930, "unknown_14009d930", tpl_50::<0x143a22b68, 0x143a22b6b, 0x143078a90, 0x142409b50, 0x1425e7740, 0x141834d18> as *const ()), // sub_14009d930
    Replacement::draft(0x14009d990, "unknown_14009d990", tpl_41::<0x143a22d20, 0x143a22d23, 0x143078aa8, 0x142409b50, 0x1425e7750, 0x141834d18> as *const ()), // sub_14009d990
    Replacement::draft(0x14009d9f0, "unknown_14009d9f0", tpl_55::<0x143a22fc8, 0x143a22fcb, 0x143078ab8, 0x142409b50, 0x1425e7760, 0x141834d18> as *const ()), // sub_14009d9f0
    Replacement::draft(0x14009da50, "unknown_14009da50", tpl_41::<0x143a23068, 0x143a2306b, 0x143078ac8, 0x142409b50, 0x1425e7770, 0x141834d18> as *const ()), // sub_14009da50
    Replacement::draft(0x14009dab0, "unknown_14009dab0", tpl_49::<0x143a23108, 0x143a2310b, 0x143078ad8, 0x142409b50, 0x1425e7780, 0x141834d18> as *const ()), // sub_14009dab0
    Replacement::draft(0x14009db10, "unknown_14009db10", tpl_40::<0x143a22968, 0x143a2296b, 0x143078ae4, 0x142409b50, 0x1425e7790, 0x141834d18> as *const ()), // sub_14009db10
    Replacement::draft(0x14009db70, "unknown_14009db70", tpl_44::<0x143a22aa0, 0x143a22aa3, 0x143078af0, 0x142409b50, 0x1425e77a0, 0x141834d18> as *const ()), // sub_14009db70
    Replacement::draft(0x14009dbd0, "unknown_14009dbd0", tpl_46::<0x143a22940, 0x143a22943, 0x143078b08, 0x142409b50, 0x1425e77b0, 0x141834d18> as *const ()), // sub_14009dbd0
    Replacement::draft(0x14009dc30, "unknown_14009dc30", tpl_55::<0x143a22ff0, 0x143a22ff3, 0x143078b20, 0x142409b50, 0x1425e77c0, 0x141834d18> as *const ()), // sub_14009dc30
    Replacement::draft(0x14009dc90, "unknown_14009dc90", tpl_46::<0x143a22e88, 0x143a22e8b, 0x143078b30, 0x142409b50, 0x1425e77d0, 0x141834d18> as *const ()), // sub_14009dc90
    Replacement::draft(0x14009dcf0, "unknown_14009dcf0", tpl_50::<0x143a22c30, 0x143a22c33, 0x143078b48, 0x142409b50, 0x1425e77e0, 0x141834d18> as *const ()), // sub_14009dcf0
    Replacement::draft(0x14009dd50, "unknown_14009dd50", tpl_42::<0x143a228f0, 0x143a228f3, 0x143078b60, 0x142409b50, 0x1425e77f0, 0x141834d18> as *const ()), // sub_14009dd50
    Replacement::draft(0x14009ddb0, "unknown_14009ddb0", tpl_38::<0x143a22f50, 0x143a22f53, 0x143078b78, 0x142409b50, 0x1425e7800, 0x141834d18> as *const ()), // sub_14009ddb0
    Replacement::draft(0x14009de10, "unknown_14009de10", tpl_34::<0x143a232c0, 0x143a232c3, 0x143078b88, 0x142409b50, 0x1425e7810, 0x141834d18> as *const ()), // sub_14009de10
    Replacement::draft(0x14009de70, "unknown_14009de70", tpl_41::<0x143a230b8, 0x143a230bb, 0x143078b98, 0x142409b50, 0x1425e7820, 0x141834d18> as *const ()), // sub_14009de70
    Replacement::draft(0x14009ded0, "unknown_14009ded0", tpl_36::<0x143a22e38, 0x143a22e3b, 0x143078ba8, 0x142409b50, 0x1425e7830, 0x141834d18> as *const ()), // sub_14009ded0
    Replacement::draft(0x14009df30, "unknown_14009df30", tpl_34::<0x143a22918, 0x143a2291b, 0x143078bb8, 0x142409b50, 0x1425e7840, 0x141834d18> as *const ()), // sub_14009df30
    Replacement::draft(0x14009e170, "unknown_14009e170", tpl_23::<0x143a234c8, 0x143a234cb, 0x1426cdcb0, 0x142409b50, 0x1425e7850, 0x141834d18> as *const ()), // sub_14009e170
    Replacement::draft(0x14009e1d0, "unknown_14009e1d0", tpl_23::<0x143a23510, 0x143a23513, 0x1426cdcb0, 0x142409b50, 0x1425e7860, 0x141834d18> as *const ()), // sub_14009e1d0
    Replacement::draft(0x14009e3c0, "unknown_14009e3c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438485e8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e78a0, 0x141834d18> as *const ()), // sub_14009e3c0
    Replacement::draft(0x14009e7c0, "unknown_14009e7c0", tpl_34::<0x143a23678, 0x143a2367b, 0x143078668, 0x142409b50, 0x1425e7910, 0x141834d18> as *const ()), // sub_14009e7c0
    Replacement::draft(0x14009e820, "unknown_14009e820", tpl_23::<0x143a23c88, 0x143a23c8b, 0x1426cdcb0, 0x142409b50, 0x1425e7920, 0x141834d18> as *const ()), // sub_14009e820
    Replacement::draft(0x14009e880, "unknown_14009e880", tpl_35::<0x143a239b8, 0x143a239bb, 0x143078678, 0x142409b50, 0x1425e7930, 0x141834d18> as *const ()), // sub_14009e880
    Replacement::draft(0x14009e8e0, "unknown_14009e8e0", tpl_36::<0x143a23c10, 0x143a23c13, 0x143078680, 0x142409b50, 0x1425e7940, 0x141834d18> as *const ()), // sub_14009e8e0
    Replacement::draft(0x14009e940, "unknown_14009e940", tpl_37::<0x143a23858, 0x143a2385b, 0x143078690, 0x142409b50, 0x1425e7950, 0x141834d18> as *const ()), // sub_14009e940
    Replacement::draft(0x14009e9a0, "unknown_14009e9a0", tpl_38::<0x143a23718, 0x143a2371b, 0x1430786a0, 0x142409b50, 0x1425e7960, 0x141834d18> as *const ()), // sub_14009e9a0
    Replacement::draft(0x14009ea00, "unknown_14009ea00", tpl_39::<0x143a23cb0, 0x143a23cb3, 0x1430786b0, 0x142409b50, 0x1425e7970, 0x141834d18> as *const ()), // sub_14009ea00
    Replacement::draft(0x14009ea60, "unknown_14009ea60", tpl_37::<0x143a23628, 0x143a2362b, 0x1430786b8, 0x142409b50, 0x1425e7980, 0x141834d18> as *const ()), // sub_14009ea60
    Replacement::draft(0x14009eac0, "unknown_14009eac0", tpl_40::<0x143a23dd8, 0x143a23ddb, 0x1430786c4, 0x142409b50, 0x1425e7990, 0x141834d18> as *const ()), // sub_14009eac0
    Replacement::draft(0x14009eb20, "unknown_14009eb20", tpl_41::<0x143a237e0, 0x143a237e3, 0x1430786d0, 0x142409b50, 0x1425e79a0, 0x141834d18> as *const ()), // sub_14009eb20
    Replacement::draft(0x14009eb80, "unknown_14009eb80", tpl_42::<0x143a23d00, 0x143a23d03, 0x1430786e0, 0x142409b50, 0x1425e79b0, 0x141834d18> as *const ()), // sub_14009eb80
    Replacement::draft(0x14009ebe0, "unknown_14009ebe0", tpl_34::<0x143a23d50, 0x143a23d53, 0x1430786f8, 0x142409b50, 0x1425e79c0, 0x141834d18> as *const ()), // sub_14009ebe0
    Replacement::draft(0x14009ec40, "unknown_14009ec40", tpl_34::<0x143a23650, 0x143a23653, 0x143078708, 0x142409b50, 0x1425e79d0, 0x141834d18> as *const ()), // sub_14009ec40
    Replacement::draft(0x14009eca0, "unknown_14009eca0", tpl_41::<0x143a23740, 0x143a23743, 0x143078718, 0x142409b50, 0x1425e79e0, 0x141834d18> as *const ()), // sub_14009eca0
    Replacement::draft(0x14009ed00, "unknown_14009ed00", tpl_43::<0x143a23880, 0x143a23883, 0x143078728, 0x142409b50, 0x1425e79f0, 0x141834d18> as *const ()), // sub_14009ed00
    Replacement::draft(0x14009ed60, "unknown_14009ed60", tpl_44::<0x143a23338, 0x143a2333b, 0x143078748, 0x142409b50, 0x1425e7a00, 0x141834d18> as *const ()), // sub_14009ed60
    Replacement::draft(0x14009edc0, "unknown_14009edc0", tpl_39::<0x143a23808, 0x143a2380b, 0x143078760, 0x142409b50, 0x1425e7a10, 0x141834d18> as *const ()), // sub_14009edc0
    Replacement::draft(0x14009ee20, "unknown_14009ee20", tpl_45::<0x143a236f0, 0x143a236f3, 0x143078768, 0x142409b50, 0x1425e7a20, 0x141834d18> as *const ()), // sub_14009ee20
    Replacement::draft(0x14009ee80, "unknown_14009ee80", tpl_41::<0x143a23bc0, 0x143a23bc3, 0x143078778, 0x142409b50, 0x1425e7a30, 0x141834d18> as *const ()), // sub_14009ee80
    Replacement::draft(0x14009eee0, "unknown_14009eee0", tpl_37::<0x143a23360, 0x143a23363, 0x143078788, 0x142409b50, 0x1425e7a40, 0x141834d18> as *const ()), // sub_14009eee0
    Replacement::draft(0x14009ef40, "unknown_14009ef40", tpl_46::<0x143a23a58, 0x143a23a5b, 0x143078798, 0x142409b50, 0x1425e7a50, 0x141834d18> as *const ()), // sub_14009ef40
    Replacement::draft(0x14009efa0, "unknown_14009efa0", tpl_47::<0x143a23588, 0x143a2358b, 0x1430787b0, 0x142409b50, 0x1425e7a60, 0x141834d18> as *const ()), // sub_14009efa0
    Replacement::draft(0x14009f000, "unknown_14009f000", tpl_38::<0x143a23af8, 0x143a23afb, 0x1430787d0, 0x142409b50, 0x1425e7a70, 0x141834d18> as *const ()), // sub_14009f000
    Replacement::draft(0x14009f060, "unknown_14009f060", tpl_48::<0x143a23cd8, 0x143a23cdb, 0x1430787e0, 0x142409b50, 0x1425e7a80, 0x141834d18> as *const ()), // sub_14009f060
    Replacement::draft(0x14009f0c0, "unknown_14009f0c0", tpl_49::<0x143a23990, 0x143a23993, 0x1430787f8, 0x142409b50, 0x1425e7a90, 0x141834d18> as *const ()), // sub_14009f0c0
    Replacement::draft(0x14009f120, "unknown_14009f120", tpl_44::<0x143a23c60, 0x143a23c63, 0x143078808, 0x142409b50, 0x1425e7aa0, 0x141834d18> as *const ()), // sub_14009f120
    Replacement::draft(0x14009f180, "unknown_14009f180", tpl_42::<0x143a23310, 0x143a23313, 0x143078820, 0x142409b50, 0x1425e7ab0, 0x141834d18> as *const ()), // sub_14009f180
    Replacement::draft(0x14009f1e0, "unknown_14009f1e0", tpl_50::<0x143a23b70, 0x143a23b73, 0x143078838, 0x142409b50, 0x1425e7ac0, 0x141834d18> as *const ()), // sub_14009f1e0
    Replacement::draft(0x14009f240, "unknown_14009f240", tpl_52::<0x143a234a0, 0x143a234a3, 0x143a23dd8, 0x143a23ddc, 0x143a23ddb, 0x143a23de0, 0x142409b50, 0x1425e7ad0, 0x141834d18> as *const ()), // sub_14009f240
    Replacement::draft(0x14009f300, "unknown_14009f300", tpl_38::<0x143a239e0, 0x143a239e3, 0x143078950, 0x142409b50, 0x1425e7ae0, 0x141834d18> as *const ()), // sub_14009f300
    Replacement::draft(0x14009f360, "unknown_14009f360", tpl_53::<0x143a23560, 0x143a23563, 0x143078960, 0x142409b50, 0x1425e7af0, 0x141834d18> as *const ()), // sub_14009f360
    Replacement::draft(0x14009f3c0, "unknown_14009f3c0", tpl_45::<0x143a23428, 0x143a2342b, 0x143078980, 0x142409b50, 0x1425e7b00, 0x141834d18> as *const ()), // sub_14009f3c0
    Replacement::draft(0x14009f420, "unknown_14009f420", tpl_35::<0x143a23478, 0x143a2347b, 0x14307898c, 0x142409b50, 0x1425e7b10, 0x141834d18> as *const ()), // sub_14009f420
    Replacement::draft(0x14009f480, "unknown_14009f480", tpl_42::<0x143a23d28, 0x143a23d2b, 0x1430786e0, 0x142409b50, 0x1425e7b20, 0x141834d18> as *const ()), // sub_14009f480
    Replacement::draft(0x14009f4e0, "unknown_14009f4e0", tpl_38::<0x143a23c38, 0x143a23c3b, 0x143078998, 0x142409b50, 0x1425e7b30, 0x141834d18> as *const ()), // sub_14009f4e0
    Replacement::draft(0x14009f540, "unknown_14009f540", tpl_38::<0x143a23790, 0x143a23793, 0x1430789a8, 0x142409b50, 0x1425e7b40, 0x141834d18> as *const ()), // sub_14009f540
    Replacement::draft(0x14009f5a0, "unknown_14009f5a0", tpl_41::<0x143a238f0, 0x143a238f3, 0x1430789b8, 0x142409b50, 0x1425e7b50, 0x141834d18> as *const ()), // sub_14009f5a0
    Replacement::draft(0x14009f600, "unknown_14009f600", tpl_34::<0x143a23d88, 0x143a23d8b, 0x1430789c8, 0x142409b50, 0x1425e7b60, 0x141834d18> as *const ()), // sub_14009f600
    Replacement::draft(0x14009f660, "unknown_14009f660", tpl_39::<0x143a23450, 0x143a23453, 0x1430789d8, 0x142409b50, 0x1425e7b70, 0x141834d18> as *const ()), // sub_14009f660
    Replacement::draft(0x14009f6c0, "unknown_14009f6c0", tpl_41::<0x143a23768, 0x143a2376b, 0x1430789e0, 0x142409b50, 0x1425e7b80, 0x141834d18> as *const ()), // sub_14009f6c0
    Replacement::draft(0x14009f720, "unknown_14009f720", tpl_34::<0x143a23a08, 0x143a23a0b, 0x1430789f0, 0x142409b50, 0x1425e7b90, 0x141834d18> as *const ()), // sub_14009f720
    Replacement::draft(0x14009f780, "unknown_14009f780", tpl_48::<0x143a236a0, 0x143a236a3, 0x143078a00, 0x142409b50, 0x1425e7ba0, 0x141834d18> as *const ()), // sub_14009f780
    Replacement::draft(0x14009f7e0, "unknown_14009f7e0", tpl_46::<0x143a23b20, 0x143a23b23, 0x143078a18, 0x142409b50, 0x1425e7bb0, 0x141834d18> as *const ()), // sub_14009f7e0
    Replacement::draft(0x14009f840, "unknown_14009f840", tpl_47::<0x143a23a80, 0x143a23a83, 0x143078a30, 0x142409b50, 0x1425e7bc0, 0x141834d18> as *const ()), // sub_14009f840
    Replacement::draft(0x14009f8a0, "unknown_14009f8a0", tpl_37::<0x143a235d8, 0x143a235db, 0x143078a50, 0x142409b50, 0x1425e7bd0, 0x141834d18> as *const ()), // sub_14009f8a0
    Replacement::draft(0x14009f900, "unknown_14009f900", tpl_45::<0x143a23830, 0x143a23833, 0x143078a60, 0x142409b50, 0x1425e7be0, 0x141834d18> as *const ()), // sub_14009f900
    Replacement::draft(0x14009f960, "unknown_14009f960", tpl_54::<0x143a23940, 0x143a23943, 0x143078a70, 0x142409b50, 0x1425e7bf0, 0x141834d18> as *const ()), // sub_14009f960
    Replacement::draft(0x14009f9c0, "unknown_14009f9c0", tpl_40::<0x143a235b0, 0x143a235b3, 0x143078a88, 0x142409b50, 0x1425e7c00, 0x141834d18> as *const ()), // sub_14009f9c0
    Replacement::draft(0x14009fa20, "unknown_14009fa20", tpl_50::<0x143a23600, 0x143a23603, 0x143078a90, 0x142409b50, 0x1425e7c10, 0x141834d18> as *const ()), // sub_14009fa20
    Replacement::draft(0x14009fa80, "unknown_14009fa80", tpl_41::<0x143a237b8, 0x143a237bb, 0x143078aa8, 0x142409b50, 0x1425e7c20, 0x141834d18> as *const ()), // sub_14009fa80
    Replacement::draft(0x14009fae0, "unknown_14009fae0", tpl_55::<0x143a23aa8, 0x143a23aab, 0x143078ab8, 0x142409b50, 0x1425e7c30, 0x141834d18> as *const ()), // sub_14009fae0
    Replacement::draft(0x14009fb40, "unknown_14009fb40", tpl_41::<0x143a23b48, 0x143a23b4b, 0x143078ac8, 0x142409b50, 0x1425e7c40, 0x141834d18> as *const ()), // sub_14009fb40
    Replacement::draft(0x14009fba0, "unknown_14009fba0", tpl_49::<0x143a23be8, 0x143a23beb, 0x143078ad8, 0x142409b50, 0x1425e7c50, 0x141834d18> as *const ()), // sub_14009fba0
    Replacement::draft(0x14009fc00, "unknown_14009fc00", tpl_40::<0x143a23400, 0x143a23403, 0x143078ae4, 0x142409b50, 0x1425e7c60, 0x141834d18> as *const ()), // sub_14009fc00
    Replacement::draft(0x14009fc60, "unknown_14009fc60", tpl_44::<0x143a23538, 0x143a2353b, 0x143078af0, 0x142409b50, 0x1425e7c70, 0x141834d18> as *const ()), // sub_14009fc60
    Replacement::draft(0x14009fcc0, "unknown_14009fcc0", tpl_46::<0x143a233d8, 0x143a233db, 0x143078b08, 0x142409b50, 0x1425e7c80, 0x141834d18> as *const ()), // sub_14009fcc0
    Replacement::draft(0x14009fd20, "unknown_14009fd20", tpl_55::<0x143a23ad0, 0x143a23ad3, 0x143078b20, 0x142409b50, 0x1425e7c90, 0x141834d18> as *const ()), // sub_14009fd20
    Replacement::draft(0x14009fd80, "unknown_14009fd80", tpl_46::<0x143a23968, 0x143a2396b, 0x143078b30, 0x142409b50, 0x1425e7ca0, 0x141834d18> as *const ()), // sub_14009fd80
    Replacement::draft(0x14009fde0, "unknown_14009fde0", tpl_50::<0x143a236c8, 0x143a236cb, 0x143078b48, 0x142409b50, 0x1425e7cb0, 0x141834d18> as *const ()), // sub_14009fde0
    Replacement::draft(0x14009fe40, "unknown_14009fe40", tpl_42::<0x143a23388, 0x143a2338b, 0x143078b60, 0x142409b50, 0x1425e7cc0, 0x141834d18> as *const ()), // sub_14009fe40
    Replacement::draft(0x14009fea0, "unknown_14009fea0", tpl_38::<0x143a23a30, 0x143a23a33, 0x143078b78, 0x142409b50, 0x1425e7cd0, 0x141834d18> as *const ()), // sub_14009fea0
    Replacement::draft(0x14009ff00, "unknown_14009ff00", tpl_34::<0x143a23db0, 0x143a23db3, 0x143078b88, 0x142409b50, 0x1425e7ce0, 0x141834d18> as *const ()), // sub_14009ff00
    Replacement::draft(0x14009ff60, "unknown_14009ff60", tpl_41::<0x143a23b98, 0x143a23b9b, 0x143078b98, 0x142409b50, 0x1425e7cf0, 0x141834d18> as *const ()), // sub_14009ff60
    Replacement::draft(0x14009ffc0, "unknown_14009ffc0", tpl_36::<0x143a23918, 0x143a2391b, 0x143078ba8, 0x142409b50, 0x1425e7d00, 0x141834d18> as *const ()), // sub_14009ffc0
    Replacement::draft(0x1400a0020, "unknown_1400a0020", tpl_34::<0x143a233b0, 0x143a233b3, 0x143078bb8, 0x142409b50, 0x1425e7d10, 0x141834d18> as *const ()), // sub_1400a0020
    Replacement::draft(0x1400a00c0, "unknown_1400a00c0", fn_1400a00c0 as *const ()), // sub_1400a00c0
    Replacement::draft(0x1400a0430, "unknown_1400a0430", tpl_23::<0x143a23e00, 0x143a23e03, 0x1426cdcb0, 0x142409b50, 0x1425e7dd0, 0x141834d18> as *const ()), // sub_1400a0430
    Replacement::draft(0x1400a0490, "unknown_1400a0490", tpl_23::<0x143a23e48, 0x143a23e4b, 0x1426cdcb0, 0x142409b50, 0x1425e7de0, 0x141834d18> as *const ()), // sub_1400a0490
    Replacement::draft(0x1400a0680, "unknown_1400a0680", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848698, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e7e20, 0x141834d18> as *const ()), // sub_1400a0680
    Replacement::draft(0x1400a12c0, "unknown_1400a12c0", tpl_23::<0x143a23eb0, 0x143a23eb3, 0x1426cdcb0, 0x142409b50, 0x1425e8060, 0x141834d18> as *const ()), // sub_1400a12c0
    Replacement::draft(0x1400a1320, "unknown_1400a1320", tpl_23::<0x143a23ef8, 0x143a23efb, 0x1426cdcb0, 0x142409b50, 0x1425e8070, 0x141834d18> as *const ()), // sub_1400a1320
    Replacement::draft(0x1400a1510, "unknown_1400a1510", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438487a0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e80b0, 0x141834d18> as *const ()), // sub_1400a1510
    Replacement::draft(0x1400a1910, "unknown_1400a1910", tpl_61::<0x143a24080, 0x143a24083, 0x143091fd8, 0x142409b50, 0x1425e8120, 0x141834d18> as *const ()), // sub_1400a1910
    Replacement::draft(0x1400a1970, "unknown_1400a1970", tpl_62::<0x143a24083, 0x143a24080, 0x143a24088, 0x143a24084, 0x143a23f20> as *const ()), // sub_1400a1970
    Replacement::draft(0x1400a1a00, "unknown_1400a1a00", tpl_63::<0x143a24030, 0x143a24033, 0x143091ff0, 0x142409b50, 0x1425e8130, 0x141834d18> as *const ()), // sub_1400a1a00
    Replacement::draft(0x1400a1a60, "unknown_1400a1a60", tpl_64::<0x143a23f90, 0x143a23f93, 0x143091fa0, 0x142409b50, 0x1425e8140, 0x141834d18> as *const ()), // sub_1400a1a60
    Replacement::draft(0x1400a1ac0, "unknown_1400a1ac0", tpl_65::<0x143a23f30, 0x143a23f33, 0x143091fc0, 0x142409b50, 0x1425e8150, 0x141834d18> as *const ()), // sub_1400a1ac0
    Replacement::draft(0x1400a1b20, "unknown_1400a1b20", tpl_62::<0x143a24033, 0x143a24030, 0x143a24038, 0x143a24034, 0x143a24020> as *const ()), // sub_1400a1b20
    Replacement::draft(0x1400a1bb0, "unknown_1400a1bb0", tpl_62::<0x143a23f93, 0x143a23f90, 0x143a23f98, 0x143a23f94, 0x143a23f78> as *const ()), // sub_1400a1bb0
    Replacement::draft(0x1400a1c40, "unknown_1400a1c40", tpl_62::<0x143a23f33, 0x143a23f30, 0x143a23f38, 0x143a23f34, 0x143a23f80> as *const ()), // sub_1400a1c40
    Replacement::draft(0x1400a2a30, "unknown_1400a2a30", tpl_23::<0x143a240c8, 0x143a240cb, 0x1426cdcb0, 0x142409b50, 0x1425e8250, 0x141834d18> as *const ()), // sub_1400a2a30
    Replacement::draft(0x1400a2a90, "unknown_1400a2a90", tpl_23::<0x143a24110, 0x143a24113, 0x1426cdcb0, 0x142409b50, 0x1425e8260, 0x141834d18> as *const ()), // sub_1400a2a90
    Replacement::draft(0x1400a2c80, "unknown_1400a2c80", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848830, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e82a0, 0x141834d18> as *const ()), // sub_1400a2c80
    Replacement::draft(0x1400a4110, "unknown_1400a4110", tpl_23::<0x143a24210, 0x143a24213, 0x1426cdcb0, 0x142409b50, 0x1425e8520, 0x141834d18> as *const ()), // sub_1400a4110
    Replacement::draft(0x1400a4170, "unknown_1400a4170", tpl_23::<0x143a24258, 0x143a2425b, 0x1426cdcb0, 0x142409b50, 0x1425e8530, 0x141834d18> as *const ()), // sub_1400a4170
    Replacement::draft(0x1400a4360, "unknown_1400a4360", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848930, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e8570, 0x141834d18> as *const ()), // sub_1400a4360
    Replacement::draft(0x1400a46f0, "unknown_1400a46f0", tpl_112::<0x1434267cc, 0x143427090, 0x143426978, 0x143426650, 0x143a001ac, 0x143848a6c, 0x143848a78, 0x143848a68, 0x143848a74, 0x143848a80, 0x143848a8c, 0x143848a60, 0x143848a84, 0x143a00290, 0x143848a50, 0x1425e85b0, 0x141834d18> as *const ()), // sub_1400a46f0
    Replacement::draft(0x1400a47a0, "unknown_1400a47a0", tpl_112::<0x143426904, 0x143427090, 0x143426978, 0x143426650, 0x143a001ac, 0x143848aac, 0x143848ab8, 0x143848aa8, 0x143848ab4, 0x143848ac0, 0x143848acc, 0x143848aa0, 0x143848ac4, 0x143a00290, 0x143848a90, 0x1425e8640, 0x141834d18> as *const ()), // sub_1400a47a0
    Replacement::draft(0x1400a5280, "unknown_1400a5280", tpl_23::<0x143a24280, 0x143a24283, 0x1426cdcb0, 0x142409b50, 0x1425e8ae0, 0x141834d18> as *const ()), // sub_1400a5280
    Replacement::draft(0x1400a52e0, "unknown_1400a52e0", tpl_23::<0x143a242c8, 0x143a242cb, 0x1426cdcb0, 0x142409b50, 0x1425e8af0, 0x141834d18> as *const ()), // sub_1400a52e0
    Replacement::draft(0x1400a54d0, "unknown_1400a54d0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848b00, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e8b30, 0x141834d18> as *const ()), // sub_1400a54d0
    Replacement::draft(0x1400a5be0, "unknown_1400a5be0", tpl_23::<0x143a24510, 0x143a24513, 0x1426cdcb0, 0x142409b50, 0x1425e8bc0, 0x141834d18> as *const ()), // sub_1400a5be0
    Replacement::draft(0x1400a5c40, "unknown_1400a5c40", tpl_23::<0x143a24558, 0x143a2455b, 0x1426cdcb0, 0x142409b50, 0x1425e8bd0, 0x141834d18> as *const ()), // sub_1400a5c40
    Replacement::draft(0x1400a5e30, "unknown_1400a5e30", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848bb0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e8c10, 0x141834d18> as *const ()), // sub_1400a5e30
    Replacement::draft(0x1400a6230, "unknown_1400a6230", tpl_34::<0x143a246c0, 0x143a246c3, 0x143078668, 0x142409b50, 0x1425e8c80, 0x141834d18> as *const ()), // sub_1400a6230
    Replacement::draft(0x1400a6290, "unknown_1400a6290", tpl_23::<0x143a24cd0, 0x143a24cd3, 0x1426cdcb0, 0x142409b50, 0x1425e8c90, 0x141834d18> as *const ()), // sub_1400a6290
    Replacement::draft(0x1400a62f0, "unknown_1400a62f0", tpl_35::<0x143a249f8, 0x143a249fb, 0x143078678, 0x142409b50, 0x1425e8ca0, 0x141834d18> as *const ()), // sub_1400a62f0
    Replacement::draft(0x1400a6350, "unknown_1400a6350", tpl_36::<0x143a24c50, 0x143a24c53, 0x143078680, 0x142409b50, 0x1425e8cb0, 0x141834d18> as *const ()), // sub_1400a6350
    Replacement::draft(0x1400a63b0, "unknown_1400a63b0", tpl_37::<0x143a248a0, 0x143a248a3, 0x143078690, 0x142409b50, 0x1425e8cc0, 0x141834d18> as *const ()), // sub_1400a63b0
    Replacement::draft(0x1400a6410, "unknown_1400a6410", tpl_38::<0x143a24760, 0x143a24763, 0x1430786a0, 0x142409b50, 0x1425e8cd0, 0x141834d18> as *const ()), // sub_1400a6410
    Replacement::draft(0x1400a6470, "unknown_1400a6470", tpl_39::<0x143a24cf8, 0x143a24cfb, 0x1430786b0, 0x142409b50, 0x1425e8ce0, 0x141834d18> as *const ()), // sub_1400a6470
    Replacement::draft(0x1400a64d0, "unknown_1400a64d0", tpl_37::<0x143a24670, 0x143a24673, 0x1430786b8, 0x142409b50, 0x1425e8cf0, 0x141834d18> as *const ()), // sub_1400a64d0
    Replacement::draft(0x1400a6530, "unknown_1400a6530", tpl_40::<0x143a24e10, 0x143a24e13, 0x1430786c4, 0x142409b50, 0x1425e8d00, 0x141834d18> as *const ()), // sub_1400a6530
    Replacement::draft(0x1400a6590, "unknown_1400a6590", tpl_41::<0x143a24828, 0x143a2482b, 0x1430786d0, 0x142409b50, 0x1425e8d10, 0x141834d18> as *const ()), // sub_1400a6590
    Replacement::draft(0x1400a65f0, "unknown_1400a65f0", tpl_42::<0x143a24d48, 0x143a24d4b, 0x1430786e0, 0x142409b50, 0x1425e8d20, 0x141834d18> as *const ()), // sub_1400a65f0
    Replacement::draft(0x1400a6650, "unknown_1400a6650", tpl_34::<0x143a24d98, 0x143a24d9b, 0x1430786f8, 0x142409b50, 0x1425e8d30, 0x141834d18> as *const ()), // sub_1400a6650
    Replacement::draft(0x1400a66b0, "unknown_1400a66b0", tpl_34::<0x143a24698, 0x143a2469b, 0x143078708, 0x142409b50, 0x1425e8d40, 0x141834d18> as *const ()), // sub_1400a66b0
    Replacement::draft(0x1400a6710, "unknown_1400a6710", tpl_41::<0x143a24788, 0x143a2478b, 0x143078718, 0x142409b50, 0x1425e8d50, 0x141834d18> as *const ()), // sub_1400a6710
    Replacement::draft(0x1400a6770, "unknown_1400a6770", tpl_43::<0x143a248c8, 0x143a248cb, 0x143078728, 0x142409b50, 0x1425e8d60, 0x141834d18> as *const ()), // sub_1400a6770
    Replacement::draft(0x1400a67d0, "unknown_1400a67d0", tpl_44::<0x143a24358, 0x143a2435b, 0x143078748, 0x142409b50, 0x1425e8d70, 0x141834d18> as *const ()), // sub_1400a67d0
    Replacement::draft(0x1400a6830, "unknown_1400a6830", tpl_39::<0x143a24850, 0x143a24853, 0x143078760, 0x142409b50, 0x1425e8d80, 0x141834d18> as *const ()), // sub_1400a6830
    Replacement::draft(0x1400a6890, "unknown_1400a6890", tpl_45::<0x143a24738, 0x143a2473b, 0x143078768, 0x142409b50, 0x1425e8d90, 0x141834d18> as *const ()), // sub_1400a6890
    Replacement::draft(0x1400a68f0, "unknown_1400a68f0", tpl_41::<0x143a24c00, 0x143a24c03, 0x143078778, 0x142409b50, 0x1425e8da0, 0x141834d18> as *const ()), // sub_1400a68f0
    Replacement::draft(0x1400a6950, "unknown_1400a6950", tpl_37::<0x143a24380, 0x143a24383, 0x143078788, 0x142409b50, 0x1425e8db0, 0x141834d18> as *const ()), // sub_1400a6950
    Replacement::draft(0x1400a69b0, "unknown_1400a69b0", tpl_46::<0x143a24a98, 0x143a24a9b, 0x143078798, 0x142409b50, 0x1425e8dc0, 0x141834d18> as *const ()), // sub_1400a69b0
    Replacement::draft(0x1400a6a10, "unknown_1400a6a10", tpl_47::<0x143a245d0, 0x143a245d3, 0x1430787b0, 0x142409b50, 0x1425e8dd0, 0x141834d18> as *const ()), // sub_1400a6a10
    Replacement::draft(0x1400a6a70, "unknown_1400a6a70", tpl_38::<0x143a24b38, 0x143a24b3b, 0x1430787d0, 0x142409b50, 0x1425e8de0, 0x141834d18> as *const ()), // sub_1400a6a70
    Replacement::draft(0x1400a6ad0, "unknown_1400a6ad0", tpl_48::<0x143a24d20, 0x143a24d23, 0x1430787e0, 0x142409b50, 0x1425e8df0, 0x141834d18> as *const ()), // sub_1400a6ad0
    Replacement::draft(0x1400a6b30, "unknown_1400a6b30", tpl_49::<0x143a249d0, 0x143a249d3, 0x1430787f8, 0x142409b50, 0x1425e8e00, 0x141834d18> as *const ()), // sub_1400a6b30
    Replacement::draft(0x1400a6b90, "unknown_1400a6b90", tpl_44::<0x143a24ca8, 0x143a24cab, 0x143078808, 0x142409b50, 0x1425e8e10, 0x141834d18> as *const ()), // sub_1400a6b90
    Replacement::draft(0x1400a6bf0, "unknown_1400a6bf0", tpl_42::<0x143a24330, 0x143a24333, 0x143078820, 0x142409b50, 0x1425e8e20, 0x141834d18> as *const ()), // sub_1400a6bf0
    Replacement::draft(0x1400a6c50, "unknown_1400a6c50", tpl_50::<0x143a24bb0, 0x143a24bb3, 0x143078838, 0x142409b50, 0x1425e8e30, 0x141834d18> as *const ()), // sub_1400a6c50
    Replacement::draft(0x1400a6cb0, "unknown_1400a6cb0", tpl_52::<0x143a244e8, 0x143a244eb, 0x143a24e10, 0x143a24e14, 0x143a24e13, 0x143a24e18, 0x142409b50, 0x1425e8e40, 0x141834d18> as *const ()), // sub_1400a6cb0
    Replacement::draft(0x1400a6d70, "unknown_1400a6d70", tpl_38::<0x143a24a20, 0x143a24a23, 0x143078950, 0x142409b50, 0x1425e8e50, 0x141834d18> as *const ()), // sub_1400a6d70
    Replacement::draft(0x1400a6dd0, "unknown_1400a6dd0", tpl_53::<0x143a245a8, 0x143a245ab, 0x143078960, 0x142409b50, 0x1425e8e60, 0x141834d18> as *const ()), // sub_1400a6dd0
    Replacement::draft(0x1400a6e30, "unknown_1400a6e30", tpl_45::<0x143a24470, 0x143a24473, 0x143078980, 0x142409b50, 0x1425e8e70, 0x141834d18> as *const ()), // sub_1400a6e30
    Replacement::draft(0x1400a6e90, "unknown_1400a6e90", tpl_35::<0x143a244c0, 0x143a244c3, 0x14307898c, 0x142409b50, 0x1425e8e80, 0x141834d18> as *const ()), // sub_1400a6e90
    Replacement::draft(0x1400a6ef0, "unknown_1400a6ef0", tpl_42::<0x143a24d70, 0x143a24d73, 0x1430786e0, 0x142409b50, 0x1425e8e90, 0x141834d18> as *const ()), // sub_1400a6ef0
    Replacement::draft(0x1400a6f50, "unknown_1400a6f50", tpl_38::<0x143a24c80, 0x143a24c83, 0x143078998, 0x142409b50, 0x1425e8ea0, 0x141834d18> as *const ()), // sub_1400a6f50
    Replacement::draft(0x1400a6fb0, "unknown_1400a6fb0", tpl_38::<0x143a247d8, 0x143a247db, 0x1430789a8, 0x142409b50, 0x1425e8eb0, 0x141834d18> as *const ()), // sub_1400a6fb0
    Replacement::draft(0x1400a7010, "unknown_1400a7010", tpl_41::<0x143a24930, 0x143a24933, 0x1430789b8, 0x142409b50, 0x1425e8ec0, 0x141834d18> as *const ()), // sub_1400a7010
    Replacement::draft(0x1400a7070, "unknown_1400a7070", tpl_34::<0x143a24dc0, 0x143a24dc3, 0x1430789c8, 0x142409b50, 0x1425e8ed0, 0x141834d18> as *const ()), // sub_1400a7070
    Replacement::draft(0x1400a70d0, "unknown_1400a70d0", tpl_39::<0x143a24498, 0x143a2449b, 0x1430789d8, 0x142409b50, 0x1425e8ee0, 0x141834d18> as *const ()), // sub_1400a70d0
    Replacement::draft(0x1400a7130, "unknown_1400a7130", tpl_41::<0x143a247b0, 0x143a247b3, 0x1430789e0, 0x142409b50, 0x1425e8ef0, 0x141834d18> as *const ()), // sub_1400a7130
    Replacement::draft(0x1400a7190, "unknown_1400a7190", tpl_34::<0x143a24a48, 0x143a24a4b, 0x1430789f0, 0x142409b50, 0x1425e8f00, 0x141834d18> as *const ()), // sub_1400a7190
    Replacement::draft(0x1400a71f0, "unknown_1400a71f0", tpl_48::<0x143a246e8, 0x143a246eb, 0x143078a00, 0x142409b50, 0x1425e8f10, 0x141834d18> as *const ()), // sub_1400a71f0
    Replacement::draft(0x1400a7250, "unknown_1400a7250", tpl_46::<0x143a24b60, 0x143a24b63, 0x143078a18, 0x142409b50, 0x1425e8f20, 0x141834d18> as *const ()), // sub_1400a7250
    Replacement::draft(0x1400a72b0, "unknown_1400a72b0", tpl_47::<0x143a24ac0, 0x143a24ac3, 0x143078a30, 0x142409b50, 0x1425e8f30, 0x141834d18> as *const ()), // sub_1400a72b0
    Replacement::draft(0x1400a7310, "unknown_1400a7310", tpl_37::<0x143a24620, 0x143a24623, 0x143078a50, 0x142409b50, 0x1425e8f40, 0x141834d18> as *const ()), // sub_1400a7310
    Replacement::draft(0x1400a7370, "unknown_1400a7370", tpl_45::<0x143a24878, 0x143a2487b, 0x143078a60, 0x142409b50, 0x1425e8f50, 0x141834d18> as *const ()), // sub_1400a7370
    Replacement::draft(0x1400a73d0, "unknown_1400a73d0", tpl_54::<0x143a24980, 0x143a24983, 0x143078a70, 0x142409b50, 0x1425e8f60, 0x141834d18> as *const ()), // sub_1400a73d0
    Replacement::draft(0x1400a7430, "unknown_1400a7430", tpl_40::<0x143a245f8, 0x143a245fb, 0x143078a88, 0x142409b50, 0x1425e8f70, 0x141834d18> as *const ()), // sub_1400a7430
    Replacement::draft(0x1400a7490, "unknown_1400a7490", tpl_50::<0x143a24648, 0x143a2464b, 0x143078a90, 0x142409b50, 0x1425e8f80, 0x141834d18> as *const ()), // sub_1400a7490
    Replacement::draft(0x1400a74f0, "unknown_1400a74f0", tpl_41::<0x143a24800, 0x143a24803, 0x143078aa8, 0x142409b50, 0x1425e8f90, 0x141834d18> as *const ()), // sub_1400a74f0
    Replacement::draft(0x1400a7550, "unknown_1400a7550", tpl_55::<0x143a24ae8, 0x143a24aeb, 0x143078ab8, 0x142409b50, 0x1425e8fa0, 0x141834d18> as *const ()), // sub_1400a7550
    Replacement::draft(0x1400a75b0, "unknown_1400a75b0", tpl_41::<0x143a24b88, 0x143a24b8b, 0x143078ac8, 0x142409b50, 0x1425e8fb0, 0x141834d18> as *const ()), // sub_1400a75b0
    Replacement::draft(0x1400a7610, "unknown_1400a7610", tpl_49::<0x143a24c28, 0x143a24c2b, 0x143078ad8, 0x142409b50, 0x1425e8fc0, 0x141834d18> as *const ()), // sub_1400a7610
    Replacement::draft(0x1400a7670, "unknown_1400a7670", tpl_40::<0x143a24448, 0x143a2444b, 0x143078ae4, 0x142409b50, 0x1425e8fd0, 0x141834d18> as *const ()), // sub_1400a7670
    Replacement::draft(0x1400a76d0, "unknown_1400a76d0", tpl_44::<0x143a24580, 0x143a24583, 0x143078af0, 0x142409b50, 0x1425e8fe0, 0x141834d18> as *const ()), // sub_1400a76d0
    Replacement::draft(0x1400a7730, "unknown_1400a7730", tpl_46::<0x143a24420, 0x143a24423, 0x143078b08, 0x142409b50, 0x1425e8ff0, 0x141834d18> as *const ()), // sub_1400a7730
    Replacement::draft(0x1400a7790, "unknown_1400a7790", tpl_55::<0x143a24b10, 0x143a24b13, 0x143078b20, 0x142409b50, 0x1425e9000, 0x141834d18> as *const ()), // sub_1400a7790
    Replacement::draft(0x1400a77f0, "unknown_1400a77f0", tpl_46::<0x143a249a8, 0x143a249ab, 0x143078b30, 0x142409b50, 0x1425e9010, 0x141834d18> as *const ()), // sub_1400a77f0
    Replacement::draft(0x1400a7850, "unknown_1400a7850", tpl_50::<0x143a24710, 0x143a24713, 0x143078b48, 0x142409b50, 0x1425e9020, 0x141834d18> as *const ()), // sub_1400a7850
    Replacement::draft(0x1400a78b0, "unknown_1400a78b0", tpl_42::<0x143a243d0, 0x143a243d3, 0x143078b60, 0x142409b50, 0x1425e9030, 0x141834d18> as *const ()), // sub_1400a78b0
    Replacement::draft(0x1400a7910, "unknown_1400a7910", tpl_38::<0x143a24a70, 0x143a24a73, 0x143078b78, 0x142409b50, 0x1425e9040, 0x141834d18> as *const ()), // sub_1400a7910
    Replacement::draft(0x1400a7970, "unknown_1400a7970", tpl_34::<0x143a24de8, 0x143a24deb, 0x143078b88, 0x142409b50, 0x1425e9050, 0x141834d18> as *const ()), // sub_1400a7970
    Replacement::draft(0x1400a79d0, "unknown_1400a79d0", tpl_41::<0x143a24bd8, 0x143a24bdb, 0x143078b98, 0x142409b50, 0x1425e9060, 0x141834d18> as *const ()), // sub_1400a79d0
    Replacement::draft(0x1400a7a30, "unknown_1400a7a30", tpl_36::<0x143a24958, 0x143a2495b, 0x143078ba8, 0x142409b50, 0x1425e9070, 0x141834d18> as *const ()), // sub_1400a7a30
    Replacement::draft(0x1400a7a90, "unknown_1400a7a90", tpl_34::<0x143a243f8, 0x143a243fb, 0x143078bb8, 0x142409b50, 0x1425e9080, 0x141834d18> as *const ()), // sub_1400a7a90
    Replacement::draft(0x1400a7b40, "unknown_1400a7b40", fn_1400a7b40 as *const ()), // sub_1400a7b40
    Replacement::draft(0x1400ab200, "unknown_1400ab200", tpl_23::<0x143a24e38, 0x143a24e3b, 0x1426cdcb0, 0x142409b50, 0x1425e9140, 0x141834d18> as *const ()), // sub_1400ab200
    Replacement::draft(0x1400ab260, "unknown_1400ab260", tpl_23::<0x143a24e80, 0x143a24e83, 0x1426cdcb0, 0x142409b50, 0x1425e9150, 0x141834d18> as *const ()), // sub_1400ab260
    Replacement::draft(0x1400ab450, "unknown_1400ab450", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848c60, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e9190, 0x141834d18> as *const ()), // sub_1400ab450
    Replacement::draft(0x1400ab890, "unknown_1400ab890", tpl_51::<0x143136510, 0x143136500, 0x143a24ef0, 0x143136470, 0x143136460, 0x143136450, 0x1401df440, 0x1425e9290, 0x141834d18> as *const ()), // sub_1400ab890
    Replacement::draft(0x1400ac460, "unknown_1400ac460", tpl_23::<0x143a24f30, 0x143a24f33, 0x1426cdcb0, 0x142409b50, 0x1425e9310, 0x141834d18> as *const ()), // sub_1400ac460
    Replacement::draft(0x1400ac4c0, "unknown_1400ac4c0", tpl_23::<0x143a24f78, 0x143a24f7b, 0x1426cdcb0, 0x142409b50, 0x1425e9320, 0x141834d18> as *const ()), // sub_1400ac4c0
    Replacement::draft(0x1400ac6b0, "unknown_1400ac6b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143848db8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e9360, 0x141834d18> as *const ()), // sub_1400ac6b0
    Replacement::draft(0x1400acbf0, "unknown_1400acbf0", fn_1400acbf0 as *const ()), // sub_1400acbf0
    Replacement::draft(0x1400acc90, "unknown_1400acc90", fn_1400acc90 as *const ()), // sub_1400acc90
    Replacement::draft(0x1400ae860, "unknown_1400ae860", tpl_23::<0x143a24fe0, 0x143a24fe3, 0x1426cdcb0, 0x142409b50, 0x1425e9830, 0x141834d18> as *const ()), // sub_1400ae860
    Replacement::draft(0x1400ae8c0, "unknown_1400ae8c0", tpl_23::<0x143a25028, 0x143a2502b, 0x1426cdcb0, 0x142409b50, 0x1425e9840, 0x141834d18> as *const ()), // sub_1400ae8c0
    Replacement::draft(0x1400aede0, "unknown_1400aede0", tpl_61::<0x143a25160, 0x143a25163, 0x143091fd8, 0x142409b50, 0x1425e9bc0, 0x141834d18> as *const ()), // sub_1400aede0
    Replacement::draft(0x1400aee40, "unknown_1400aee40", tpl_62::<0x143a25163, 0x143a25160, 0x143a25168, 0x143a25164, 0x143a25050> as *const ()), // sub_1400aee40
    Replacement::draft(0x1400aeed0, "unknown_1400aeed0", tpl_63::<0x143a25110, 0x143a25113, 0x143091ff0, 0x142409b50, 0x1425e9bd0, 0x141834d18> as *const ()), // sub_1400aeed0
    Replacement::draft(0x1400aef30, "unknown_1400aef30", tpl_64::<0x143a250c0, 0x143a250c3, 0x143091fa0, 0x142409b50, 0x1425e9be0, 0x141834d18> as *const ()), // sub_1400aef30
    Replacement::draft(0x1400aef90, "unknown_1400aef90", tpl_65::<0x143a25060, 0x143a25063, 0x143091fc0, 0x142409b50, 0x1425e9bf0, 0x141834d18> as *const ()), // sub_1400aef90
    Replacement::draft(0x1400aeff0, "unknown_1400aeff0", tpl_62::<0x143a25113, 0x143a25110, 0x143a25118, 0x143a25114, 0x143a25108> as *const ()), // sub_1400aeff0
    Replacement::draft(0x1400af080, "unknown_1400af080", tpl_62::<0x143a250c3, 0x143a250c0, 0x143a250c8, 0x143a250c4, 0x143a250a8> as *const ()), // sub_1400af080
    Replacement::draft(0x1400af110, "unknown_1400af110", tpl_62::<0x143a25063, 0x143a25060, 0x143a25068, 0x143a25064, 0x143a250b0> as *const ()), // sub_1400af110
    Replacement::draft(0x1400af470, "unknown_1400af470", tpl_23::<0x143a25400, 0x143a25403, 0x1426cdcb0, 0x142409b50, 0x1425e9c10, 0x141834d18> as *const ()), // sub_1400af470
    Replacement::draft(0x1400af4d0, "unknown_1400af4d0", tpl_23::<0x143a25448, 0x143a2544b, 0x1426cdcb0, 0x142409b50, 0x1425e9c20, 0x141834d18> as *const ()), // sub_1400af4d0
    Replacement::draft(0x1400af6c0, "unknown_1400af6c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438490a8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425e9c60, 0x141834d18> as *const ()), // sub_1400af6c0
    Replacement::draft(0x1400afac0, "unknown_1400afac0", tpl_34::<0x143a255b0, 0x143a255b3, 0x143078668, 0x142409b50, 0x1425e9cd0, 0x141834d18> as *const ()), // sub_1400afac0
    Replacement::draft(0x1400afb20, "unknown_1400afb20", tpl_23::<0x143a25cc0, 0x143a25cc3, 0x1426cdcb0, 0x142409b50, 0x1425e9ce0, 0x141834d18> as *const ()), // sub_1400afb20
    Replacement::draft(0x1400afb80, "unknown_1400afb80", tpl_35::<0x143a25998, 0x143a2599b, 0x143078678, 0x142409b50, 0x1425e9cf0, 0x141834d18> as *const ()), // sub_1400afb80
    Replacement::draft(0x1400afbe0, "unknown_1400afbe0", tpl_36::<0x143a25c48, 0x143a25c4b, 0x143078680, 0x142409b50, 0x1425e9d00, 0x141834d18> as *const ()), // sub_1400afbe0
    Replacement::draft(0x1400afc40, "unknown_1400afc40", tpl_37::<0x143a257e8, 0x143a257eb, 0x143078690, 0x142409b50, 0x1425e9d10, 0x141834d18> as *const ()), // sub_1400afc40
    Replacement::draft(0x1400afca0, "unknown_1400afca0", tpl_38::<0x143a25650, 0x143a25653, 0x1430786a0, 0x142409b50, 0x1425e9d20, 0x141834d18> as *const ()), // sub_1400afca0
    Replacement::draft(0x1400afd00, "unknown_1400afd00", tpl_39::<0x143a25ce8, 0x143a25ceb, 0x1430786b0, 0x142409b50, 0x1425e9d30, 0x141834d18> as *const ()), // sub_1400afd00
    Replacement::draft(0x1400afd60, "unknown_1400afd60", tpl_37::<0x143a25560, 0x143a25563, 0x1430786b8, 0x142409b50, 0x1425e9d40, 0x141834d18> as *const ()), // sub_1400afd60
    Replacement::draft(0x1400afdc0, "unknown_1400afdc0", tpl_40::<0x143a25eb0, 0x143a25eb3, 0x1430786c4, 0x142409b50, 0x1425e9d50, 0x141834d18> as *const ()), // sub_1400afdc0
    Replacement::draft(0x1400afe20, "unknown_1400afe20", tpl_41::<0x143a25718, 0x143a2571b, 0x1430786d0, 0x142409b50, 0x1425e9d60, 0x141834d18> as *const ()), // sub_1400afe20
    Replacement::draft(0x1400afe80, "unknown_1400afe80", tpl_42::<0x143a25d38, 0x143a25d3b, 0x1430786e0, 0x142409b50, 0x1425e9d70, 0x141834d18> as *const ()), // sub_1400afe80
    Replacement::draft(0x1400afee0, "unknown_1400afee0", tpl_34::<0x143a25de0, 0x143a25de3, 0x1430786f8, 0x142409b50, 0x1425e9d80, 0x141834d18> as *const ()), // sub_1400afee0
    Replacement::draft(0x1400aff40, "unknown_1400aff40", tpl_34::<0x143a25588, 0x143a2558b, 0x143078708, 0x142409b50, 0x1425e9d90, 0x141834d18> as *const ()), // sub_1400aff40
    Replacement::draft(0x1400affa0, "unknown_1400affa0", tpl_41::<0x143a25678, 0x143a2567b, 0x143078718, 0x142409b50, 0x1425e9da0, 0x141834d18> as *const ()), // sub_1400affa0
    Replacement::draft(0x1400b0000, "unknown_1400b0000", tpl_43::<0x143a25810, 0x143a25813, 0x143078728, 0x142409b50, 0x1425e9db0, 0x141834d18> as *const ()), // sub_1400b0000
    Replacement::draft(0x1400b0060, "unknown_1400b0060", tpl_44::<0x143a251d0, 0x143a251d3, 0x143078748, 0x142409b50, 0x1425e9dc0, 0x141834d18> as *const ()), // sub_1400b0060
    Replacement::draft(0x1400b00c0, "unknown_1400b00c0", tpl_39::<0x143a25740, 0x143a25743, 0x143078760, 0x142409b50, 0x1425e9dd0, 0x141834d18> as *const ()), // sub_1400b00c0
    Replacement::draft(0x1400b0120, "unknown_1400b0120", tpl_45::<0x143a25628, 0x143a2562b, 0x143078768, 0x142409b50, 0x1425e9de0, 0x141834d18> as *const ()), // sub_1400b0120
    Replacement::draft(0x1400b0180, "unknown_1400b0180", tpl_41::<0x143a25bf8, 0x143a25bfb, 0x143078778, 0x142409b50, 0x1425e9df0, 0x141834d18> as *const ()), // sub_1400b0180
    Replacement::draft(0x1400b01e0, "unknown_1400b01e0", tpl_37::<0x143a251f8, 0x143a251fb, 0x143078788, 0x142409b50, 0x1425e9e00, 0x141834d18> as *const ()), // sub_1400b01e0
    Replacement::draft(0x1400b0240, "unknown_1400b0240", tpl_46::<0x143a25a38, 0x143a25a3b, 0x143078798, 0x142409b50, 0x1425e9e10, 0x141834d18> as *const ()), // sub_1400b0240
    Replacement::draft(0x1400b02a0, "unknown_1400b02a0", tpl_47::<0x143a254c0, 0x143a254c3, 0x1430787b0, 0x142409b50, 0x1425e9e20, 0x141834d18> as *const ()), // sub_1400b02a0
    Replacement::draft(0x1400b0300, "unknown_1400b0300", tpl_38::<0x143a25b30, 0x143a25b33, 0x1430787d0, 0x142409b50, 0x1425e9e30, 0x141834d18> as *const ()), // sub_1400b0300
    Replacement::draft(0x1400b0360, "unknown_1400b0360", tpl_48::<0x143a25d10, 0x143a25d13, 0x1430787e0, 0x142409b50, 0x1425e9e40, 0x141834d18> as *const ()), // sub_1400b0360
    Replacement::draft(0x1400b03c0, "unknown_1400b03c0", tpl_49::<0x143a25970, 0x143a25973, 0x1430787f8, 0x142409b50, 0x1425e9e50, 0x141834d18> as *const ()), // sub_1400b03c0
    Replacement::draft(0x1400b0420, "unknown_1400b0420", tpl_44::<0x143a25c98, 0x143a25c9b, 0x143078808, 0x142409b50, 0x1425e9e60, 0x141834d18> as *const ()), // sub_1400b0420
    Replacement::draft(0x1400b0480, "unknown_1400b0480", tpl_42::<0x143a251a8, 0x143a251ab, 0x143078820, 0x142409b50, 0x1425e9e70, 0x141834d18> as *const ()), // sub_1400b0480
    Replacement::draft(0x1400b04e0, "unknown_1400b04e0", tpl_50::<0x143a25ba8, 0x143a25bab, 0x143078838, 0x142409b50, 0x1425e9e80, 0x141834d18> as *const ()), // sub_1400b04e0
    Replacement::draft(0x1400b0540, "unknown_1400b0540", tpl_52::<0x143a253d8, 0x143a253db, 0x143a25eb0, 0x143a25eb4, 0x143a25eb3, 0x143a25eb8, 0x142409b50, 0x1425e9e90, 0x141834d18> as *const ()), // sub_1400b0540
    Replacement::draft(0x1400b0600, "unknown_1400b0600", tpl_38::<0x143a259c0, 0x143a259c3, 0x143078950, 0x142409b50, 0x1425e9ea0, 0x141834d18> as *const ()), // sub_1400b0600
    Replacement::draft(0x1400b0660, "unknown_1400b0660", tpl_53::<0x143a25498, 0x143a2549b, 0x143078960, 0x142409b50, 0x1425e9eb0, 0x141834d18> as *const ()), // sub_1400b0660
    Replacement::draft(0x1400b06c0, "unknown_1400b06c0", tpl_45::<0x143a25360, 0x143a25363, 0x143078980, 0x142409b50, 0x1425e9ec0, 0x141834d18> as *const ()), // sub_1400b06c0
    Replacement::draft(0x1400b0720, "unknown_1400b0720", tpl_35::<0x143a253b0, 0x143a253b3, 0x14307898c, 0x142409b50, 0x1425e9ed0, 0x141834d18> as *const ()), // sub_1400b0720
    Replacement::draft(0x1400b0780, "unknown_1400b0780", tpl_42::<0x143a25d60, 0x143a25d63, 0x1430786e0, 0x142409b50, 0x1425e9ee0, 0x141834d18> as *const ()), // sub_1400b0780
    Replacement::draft(0x1400b07e0, "unknown_1400b07e0", tpl_38::<0x143a25c70, 0x143a25c73, 0x143078998, 0x142409b50, 0x1425e9ef0, 0x141834d18> as *const ()), // sub_1400b07e0
    Replacement::draft(0x1400b0840, "unknown_1400b0840", tpl_38::<0x143a256c8, 0x143a256cb, 0x1430789a8, 0x142409b50, 0x1425e9f00, 0x141834d18> as *const ()), // sub_1400b0840
    Replacement::draft(0x1400b08a0, "unknown_1400b08a0", tpl_41::<0x143a258d0, 0x143a258d3, 0x1430789b8, 0x142409b50, 0x1425e9f10, 0x141834d18> as *const ()), // sub_1400b08a0
    Replacement::draft(0x1400b0900, "unknown_1400b0900", tpl_34::<0x143a25e08, 0x143a25e0b, 0x1430789c8, 0x142409b50, 0x1425e9f20, 0x141834d18> as *const ()), // sub_1400b0900
    Replacement::draft(0x1400b0960, "unknown_1400b0960", tpl_39::<0x143a25388, 0x143a2538b, 0x1430789d8, 0x142409b50, 0x1425e9f30, 0x141834d18> as *const ()), // sub_1400b0960
    Replacement::draft(0x1400b09c0, "unknown_1400b09c0", tpl_41::<0x143a256a0, 0x143a256a3, 0x1430789e0, 0x142409b50, 0x1425e9f40, 0x141834d18> as *const ()), // sub_1400b09c0
    Replacement::draft(0x1400b0a20, "unknown_1400b0a20", tpl_34::<0x143a259e8, 0x143a259eb, 0x1430789f0, 0x142409b50, 0x1425e9f50, 0x141834d18> as *const ()), // sub_1400b0a20
    Replacement::draft(0x1400b0a80, "unknown_1400b0a80", tpl_48::<0x143a255d8, 0x143a255db, 0x143078a00, 0x142409b50, 0x1425e9f60, 0x141834d18> as *const ()), // sub_1400b0a80
    Replacement::draft(0x1400b0ae0, "unknown_1400b0ae0", tpl_46::<0x143a25b58, 0x143a25b5b, 0x143078a18, 0x142409b50, 0x1425e9f70, 0x141834d18> as *const ()), // sub_1400b0ae0
    Replacement::draft(0x1400b0b40, "unknown_1400b0b40", tpl_47::<0x143a25a60, 0x143a25a63, 0x143078a30, 0x142409b50, 0x1425e9f80, 0x141834d18> as *const ()), // sub_1400b0b40
    Replacement::draft(0x1400b0ba0, "unknown_1400b0ba0", tpl_37::<0x143a25510, 0x143a25513, 0x143078a50, 0x142409b50, 0x1425e9f90, 0x141834d18> as *const ()), // sub_1400b0ba0
    Replacement::draft(0x1400b0c00, "unknown_1400b0c00", tpl_45::<0x143a257c0, 0x143a257c3, 0x143078a60, 0x142409b50, 0x1425e9fa0, 0x141834d18> as *const ()), // sub_1400b0c00
    Replacement::draft(0x1400b0c60, "unknown_1400b0c60", tpl_54::<0x143a25920, 0x143a25923, 0x143078a70, 0x142409b50, 0x1425e9fb0, 0x141834d18> as *const ()), // sub_1400b0c60
    Replacement::draft(0x1400b0cc0, "unknown_1400b0cc0", tpl_40::<0x143a254e8, 0x143a254eb, 0x143078a88, 0x142409b50, 0x1425e9fc0, 0x141834d18> as *const ()), // sub_1400b0cc0
    Replacement::draft(0x1400b0d20, "unknown_1400b0d20", tpl_50::<0x143a25538, 0x143a2553b, 0x143078a90, 0x142409b50, 0x1425e9fd0, 0x141834d18> as *const ()), // sub_1400b0d20
    Replacement::draft(0x1400b0d80, "unknown_1400b0d80", tpl_41::<0x143a256f0, 0x143a256f3, 0x143078aa8, 0x142409b50, 0x1425e9fe0, 0x141834d18> as *const ()), // sub_1400b0d80
    Replacement::draft(0x1400b0de0, "unknown_1400b0de0", tpl_55::<0x143a25a88, 0x143a25a8b, 0x143078ab8, 0x142409b50, 0x1425e9ff0, 0x141834d18> as *const ()), // sub_1400b0de0
    Replacement::draft(0x1400b0e40, "unknown_1400b0e40", tpl_41::<0x143a25b80, 0x143a25b83, 0x143078ac8, 0x142409b50, 0x1425ea000, 0x141834d18> as *const ()), // sub_1400b0e40
    Replacement::draft(0x1400b0ea0, "unknown_1400b0ea0", tpl_49::<0x143a25c20, 0x143a25c23, 0x143078ad8, 0x142409b50, 0x1425ea010, 0x141834d18> as *const ()), // sub_1400b0ea0
    Replacement::draft(0x1400b0f00, "unknown_1400b0f00", tpl_40::<0x143a25298, 0x143a2529b, 0x143078ae4, 0x142409b50, 0x1425ea020, 0x141834d18> as *const ()), // sub_1400b0f00
    Replacement::draft(0x1400b0f60, "unknown_1400b0f60", tpl_44::<0x143a25470, 0x143a25473, 0x143078af0, 0x142409b50, 0x1425ea030, 0x141834d18> as *const ()), // sub_1400b0f60
    Replacement::draft(0x1400b0fc0, "unknown_1400b0fc0", tpl_46::<0x143a25270, 0x143a25273, 0x143078b08, 0x142409b50, 0x1425ea040, 0x141834d18> as *const ()), // sub_1400b0fc0
    Replacement::draft(0x1400b1020, "unknown_1400b1020", tpl_55::<0x143a25ab0, 0x143a25ab3, 0x143078b20, 0x142409b50, 0x1425ea050, 0x141834d18> as *const ()), // sub_1400b1020
    Replacement::draft(0x1400b1080, "unknown_1400b1080", tpl_46::<0x143a25948, 0x143a2594b, 0x143078b30, 0x142409b50, 0x1425ea060, 0x141834d18> as *const ()), // sub_1400b1080
    Replacement::draft(0x1400b10e0, "unknown_1400b10e0", tpl_50::<0x143a25600, 0x143a25603, 0x143078b48, 0x142409b50, 0x1425ea070, 0x141834d18> as *const ()), // sub_1400b10e0
    Replacement::draft(0x1400b1140, "unknown_1400b1140", tpl_42::<0x143a25220, 0x143a25223, 0x143078b60, 0x142409b50, 0x1425ea080, 0x141834d18> as *const ()), // sub_1400b1140
    Replacement::draft(0x1400b11a0, "unknown_1400b11a0", tpl_38::<0x143a25a10, 0x143a25a13, 0x143078b78, 0x142409b50, 0x1425ea090, 0x141834d18> as *const ()), // sub_1400b11a0
    Replacement::draft(0x1400b1200, "unknown_1400b1200", tpl_34::<0x143a25e30, 0x143a25e33, 0x143078b88, 0x142409b50, 0x1425ea0a0, 0x141834d18> as *const ()), // sub_1400b1200
    Replacement::draft(0x1400b1260, "unknown_1400b1260", tpl_41::<0x143a25bd0, 0x143a25bd3, 0x143078b98, 0x142409b50, 0x1425ea0b0, 0x141834d18> as *const ()), // sub_1400b1260
    Replacement::draft(0x1400b12c0, "unknown_1400b12c0", tpl_36::<0x143a258f8, 0x143a258fb, 0x143078ba8, 0x142409b50, 0x1425ea0c0, 0x141834d18> as *const ()), // sub_1400b12c0
    Replacement::draft(0x1400b1320, "unknown_1400b1320", tpl_34::<0x143a25248, 0x143a2524b, 0x143078bb8, 0x142409b50, 0x1425ea0d0, 0x141834d18> as *const ()), // sub_1400b1320
    Replacement::draft_throws(0x1400b1480, "unknown_1400b1480", tpl_116::<0x14313ff18, 0x140472c60, 0x143a25e60, 0x20, 0x143a25e80, 0x40, 0x143a25ea0> as *const (), Throws::Discard), // sub_1400b1480
    Replacement::draft_throws(0x1400b14d0, "unknown_1400b14d0", tpl_116::<0x14313ff38, 0x140472c60, 0x143a25880, 0x20, 0x143a258a0, 0x40, 0x143a258c0> as *const (), Throws::Discard), // sub_1400b14d0
    Replacement::draft_throws(0x1400b1520, "unknown_1400b1520", tpl_116::<0x14313ff58, 0x140472c60, 0x143a25310, 0x20, 0x143a25330, 0x40, 0x143a25350> as *const (), Throws::Discard), // sub_1400b1520
    Replacement::draft_throws(0x1400b1570, "unknown_1400b1570", tpl_116::<0x14313fe88, 0x140472c60, 0x143a25770, 0x20, 0x143a25790, 0x40, 0x143a257b0> as *const (), Throws::Discard), // sub_1400b1570
    Replacement::draft_throws(0x1400b15c0, "unknown_1400b15c0", tpl_116::<0x14313fea8, 0x140472c60, 0x143a25ae0, 0x20, 0x143a25b00, 0x40, 0x143a25b20> as *const (), Throws::Discard), // sub_1400b15c0
    Replacement::draft_throws(0x1400b1610, "unknown_1400b1610", tpl_116::<0x14313fed0, 0x140472c60, 0x143a25d90, 0x20, 0x143a25db0, 0x40, 0x143a25dd0> as *const (), Throws::Discard), // sub_1400b1610
    Replacement::draft_throws(0x1400b1660, "unknown_1400b1660", tpl_116::<0x14313fef0, 0x140472c60, 0x143a252c0, 0x20, 0x143a252e0, 0x40, 0x143a25300> as *const (), Throws::Discard), // sub_1400b1660
    Replacement::draft(0x1400b1e30, "unknown_1400b1e30", tpl_23::<0x143a26090, 0x143a26093, 0x1426cdcb0, 0x142409b50, 0x1425ea330, 0x141834d18> as *const ()), // sub_1400b1e30
    Replacement::draft(0x1400b1e90, "unknown_1400b1e90", tpl_23::<0x143a260d8, 0x143a260db, 0x1426cdcb0, 0x142409b50, 0x1425ea340, 0x141834d18> as *const ()), // sub_1400b1e90
    Replacement::draft(0x1400b2080, "unknown_1400b2080", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849288, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ea380, 0x141834d18> as *const ()), // sub_1400b2080
    Replacement::draft(0x1400b2480, "unknown_1400b2480", tpl_34::<0x143a26240, 0x143a26243, 0x143078668, 0x142409b50, 0x1425ea3f0, 0x141834d18> as *const ()), // sub_1400b2480
    Replacement::draft(0x1400b24e0, "unknown_1400b24e0", tpl_23::<0x143a26848, 0x143a2684b, 0x1426cdcb0, 0x142409b50, 0x1425ea400, 0x141834d18> as *const ()), // sub_1400b24e0
    Replacement::draft(0x1400b2540, "unknown_1400b2540", tpl_35::<0x143a26578, 0x143a2657b, 0x143078678, 0x142409b50, 0x1425ea410, 0x141834d18> as *const ()), // sub_1400b2540
    Replacement::draft(0x1400b25a0, "unknown_1400b25a0", tpl_36::<0x143a267d0, 0x143a267d3, 0x143078680, 0x142409b50, 0x1425ea420, 0x141834d18> as *const ()), // sub_1400b25a0
    Replacement::draft(0x1400b2600, "unknown_1400b2600", tpl_37::<0x143a26420, 0x143a26423, 0x143078690, 0x142409b50, 0x1425ea430, 0x141834d18> as *const ()), // sub_1400b2600
    Replacement::draft(0x1400b2660, "unknown_1400b2660", tpl_38::<0x143a262e0, 0x143a262e3, 0x1430786a0, 0x142409b50, 0x1425ea440, 0x141834d18> as *const ()), // sub_1400b2660
    Replacement::draft(0x1400b26c0, "unknown_1400b26c0", tpl_39::<0x143a26870, 0x143a26873, 0x1430786b0, 0x142409b50, 0x1425ea450, 0x141834d18> as *const ()), // sub_1400b26c0
    Replacement::draft(0x1400b2720, "unknown_1400b2720", tpl_37::<0x143a261f0, 0x143a261f3, 0x1430786b8, 0x142409b50, 0x1425ea460, 0x141834d18> as *const ()), // sub_1400b2720
    Replacement::draft(0x1400b2780, "unknown_1400b2780", tpl_40::<0x143a26988, 0x143a2698b, 0x1430786c4, 0x142409b50, 0x1425ea470, 0x141834d18> as *const ()), // sub_1400b2780
    Replacement::draft(0x1400b27e0, "unknown_1400b27e0", tpl_41::<0x143a263a8, 0x143a263ab, 0x1430786d0, 0x142409b50, 0x1425ea480, 0x141834d18> as *const ()), // sub_1400b27e0
    Replacement::draft(0x1400b2840, "unknown_1400b2840", tpl_42::<0x143a268c0, 0x143a268c3, 0x1430786e0, 0x142409b50, 0x1425ea490, 0x141834d18> as *const ()), // sub_1400b2840
    Replacement::draft(0x1400b28a0, "unknown_1400b28a0", tpl_34::<0x143a26910, 0x143a26913, 0x1430786f8, 0x142409b50, 0x1425ea4a0, 0x141834d18> as *const ()), // sub_1400b28a0
    Replacement::draft(0x1400b2900, "unknown_1400b2900", tpl_34::<0x143a26218, 0x143a2621b, 0x143078708, 0x142409b50, 0x1425ea4b0, 0x141834d18> as *const ()), // sub_1400b2900
    Replacement::draft(0x1400b2960, "unknown_1400b2960", tpl_41::<0x143a26308, 0x143a2630b, 0x143078718, 0x142409b50, 0x1425ea4c0, 0x141834d18> as *const ()), // sub_1400b2960
    Replacement::draft(0x1400b29c0, "unknown_1400b29c0", tpl_43::<0x143a26448, 0x143a2644b, 0x143078728, 0x142409b50, 0x1425ea4d0, 0x141834d18> as *const ()), // sub_1400b29c0
    Replacement::draft(0x1400b2a20, "unknown_1400b2a20", tpl_44::<0x143a25f00, 0x143a25f03, 0x143078748, 0x142409b50, 0x1425ea4e0, 0x141834d18> as *const ()), // sub_1400b2a20
    Replacement::draft(0x1400b2a80, "unknown_1400b2a80", tpl_39::<0x143a263d0, 0x143a263d3, 0x143078760, 0x142409b50, 0x1425ea4f0, 0x141834d18> as *const ()), // sub_1400b2a80
    Replacement::draft(0x1400b2ae0, "unknown_1400b2ae0", tpl_45::<0x143a262b8, 0x143a262bb, 0x143078768, 0x142409b50, 0x1425ea500, 0x141834d18> as *const ()), // sub_1400b2ae0
    Replacement::draft(0x1400b2b40, "unknown_1400b2b40", tpl_41::<0x143a26780, 0x143a26783, 0x143078778, 0x142409b50, 0x1425ea510, 0x141834d18> as *const ()), // sub_1400b2b40
    Replacement::draft(0x1400b2ba0, "unknown_1400b2ba0", tpl_37::<0x143a25f28, 0x143a25f2b, 0x143078788, 0x142409b50, 0x1425ea520, 0x141834d18> as *const ()), // sub_1400b2ba0
    Replacement::draft(0x1400b2c00, "unknown_1400b2c00", tpl_46::<0x143a26618, 0x143a2661b, 0x143078798, 0x142409b50, 0x1425ea530, 0x141834d18> as *const ()), // sub_1400b2c00
    Replacement::draft(0x1400b2c60, "unknown_1400b2c60", tpl_47::<0x143a26150, 0x143a26153, 0x1430787b0, 0x142409b50, 0x1425ea540, 0x141834d18> as *const ()), // sub_1400b2c60
    Replacement::draft(0x1400b2cc0, "unknown_1400b2cc0", tpl_38::<0x143a266b8, 0x143a266bb, 0x1430787d0, 0x142409b50, 0x1425ea550, 0x141834d18> as *const ()), // sub_1400b2cc0
    Replacement::draft(0x1400b2d20, "unknown_1400b2d20", tpl_48::<0x143a26898, 0x143a2689b, 0x1430787e0, 0x142409b50, 0x1425ea560, 0x141834d18> as *const ()), // sub_1400b2d20
    Replacement::draft(0x1400b2d80, "unknown_1400b2d80", tpl_49::<0x143a26550, 0x143a26553, 0x1430787f8, 0x142409b50, 0x1425ea570, 0x141834d18> as *const ()), // sub_1400b2d80
    Replacement::draft(0x1400b2de0, "unknown_1400b2de0", tpl_44::<0x143a26820, 0x143a26823, 0x143078808, 0x142409b50, 0x1425ea580, 0x141834d18> as *const ()), // sub_1400b2de0
    Replacement::draft(0x1400b2e40, "unknown_1400b2e40", tpl_42::<0x143a25ed8, 0x143a25edb, 0x143078820, 0x142409b50, 0x1425ea590, 0x141834d18> as *const ()), // sub_1400b2e40
    Replacement::draft(0x1400b2ea0, "unknown_1400b2ea0", tpl_50::<0x143a26730, 0x143a26733, 0x143078838, 0x142409b50, 0x1425ea5a0, 0x141834d18> as *const ()), // sub_1400b2ea0
    Replacement::draft(0x1400b2f00, "unknown_1400b2f00", tpl_52::<0x143a26068, 0x143a2606b, 0x143a26988, 0x143a2698c, 0x143a2698b, 0x143a26990, 0x142409b50, 0x1425ea5b0, 0x141834d18> as *const ()), // sub_1400b2f00
    Replacement::draft(0x1400b2fc0, "unknown_1400b2fc0", tpl_38::<0x143a265a0, 0x143a265a3, 0x143078950, 0x142409b50, 0x1425ea5c0, 0x141834d18> as *const ()), // sub_1400b2fc0
    Replacement::draft(0x1400b3020, "unknown_1400b3020", tpl_53::<0x143a26128, 0x143a2612b, 0x143078960, 0x142409b50, 0x1425ea5d0, 0x141834d18> as *const ()), // sub_1400b3020
    Replacement::draft(0x1400b3080, "unknown_1400b3080", tpl_45::<0x143a25ff0, 0x143a25ff3, 0x143078980, 0x142409b50, 0x1425ea5e0, 0x141834d18> as *const ()), // sub_1400b3080
    Replacement::draft(0x1400b30e0, "unknown_1400b30e0", tpl_35::<0x143a26040, 0x143a26043, 0x14307898c, 0x142409b50, 0x1425ea5f0, 0x141834d18> as *const ()), // sub_1400b30e0
    Replacement::draft(0x1400b3140, "unknown_1400b3140", tpl_42::<0x143a268e8, 0x143a268eb, 0x1430786e0, 0x142409b50, 0x1425ea600, 0x141834d18> as *const ()), // sub_1400b3140
    Replacement::draft(0x1400b31a0, "unknown_1400b31a0", tpl_38::<0x143a267f8, 0x143a267fb, 0x143078998, 0x142409b50, 0x1425ea610, 0x141834d18> as *const ()), // sub_1400b31a0
    Replacement::draft(0x1400b3200, "unknown_1400b3200", tpl_38::<0x143a26358, 0x143a2635b, 0x1430789a8, 0x142409b50, 0x1425ea620, 0x141834d18> as *const ()), // sub_1400b3200
    Replacement::draft(0x1400b3260, "unknown_1400b3260", tpl_41::<0x143a264b0, 0x143a264b3, 0x1430789b8, 0x142409b50, 0x1425ea630, 0x141834d18> as *const ()), // sub_1400b3260
    Replacement::draft(0x1400b32c0, "unknown_1400b32c0", tpl_34::<0x143a26938, 0x143a2693b, 0x1430789c8, 0x142409b50, 0x1425ea640, 0x141834d18> as *const ()), // sub_1400b32c0
    Replacement::draft(0x1400b3320, "unknown_1400b3320", tpl_39::<0x143a26018, 0x143a2601b, 0x1430789d8, 0x142409b50, 0x1425ea650, 0x141834d18> as *const ()), // sub_1400b3320
    Replacement::draft(0x1400b3380, "unknown_1400b3380", tpl_41::<0x143a26330, 0x143a26333, 0x1430789e0, 0x142409b50, 0x1425ea660, 0x141834d18> as *const ()), // sub_1400b3380
    Replacement::draft(0x1400b33e0, "unknown_1400b33e0", tpl_34::<0x143a265c8, 0x143a265cb, 0x1430789f0, 0x142409b50, 0x1425ea670, 0x141834d18> as *const ()), // sub_1400b33e0
    Replacement::draft(0x1400b3440, "unknown_1400b3440", tpl_48::<0x143a26268, 0x143a2626b, 0x143078a00, 0x142409b50, 0x1425ea680, 0x141834d18> as *const ()), // sub_1400b3440
    Replacement::draft(0x1400b34a0, "unknown_1400b34a0", tpl_46::<0x143a266e0, 0x143a266e3, 0x143078a18, 0x142409b50, 0x1425ea690, 0x141834d18> as *const ()), // sub_1400b34a0
    Replacement::draft(0x1400b3500, "unknown_1400b3500", tpl_47::<0x143a26640, 0x143a26643, 0x143078a30, 0x142409b50, 0x1425ea6a0, 0x141834d18> as *const ()), // sub_1400b3500
    Replacement::draft(0x1400b3560, "unknown_1400b3560", tpl_37::<0x143a261a0, 0x143a261a3, 0x143078a50, 0x142409b50, 0x1425ea6b0, 0x141834d18> as *const ()), // sub_1400b3560
    Replacement::draft(0x1400b35c0, "unknown_1400b35c0", tpl_45::<0x143a263f8, 0x143a263fb, 0x143078a60, 0x142409b50, 0x1425ea6c0, 0x141834d18> as *const ()), // sub_1400b35c0
    Replacement::draft(0x1400b3620, "unknown_1400b3620", tpl_54::<0x143a26500, 0x143a26503, 0x143078a70, 0x142409b50, 0x1425ea6d0, 0x141834d18> as *const ()), // sub_1400b3620
    Replacement::draft(0x1400b3680, "unknown_1400b3680", tpl_40::<0x143a26178, 0x143a2617b, 0x143078a88, 0x142409b50, 0x1425ea6e0, 0x141834d18> as *const ()), // sub_1400b3680
    Replacement::draft(0x1400b36e0, "unknown_1400b36e0", tpl_50::<0x143a261c8, 0x143a261cb, 0x143078a90, 0x142409b50, 0x1425ea6f0, 0x141834d18> as *const ()), // sub_1400b36e0
    Replacement::draft(0x1400b3740, "unknown_1400b3740", tpl_41::<0x143a26380, 0x143a26383, 0x143078aa8, 0x142409b50, 0x1425ea700, 0x141834d18> as *const ()), // sub_1400b3740
    Replacement::draft(0x1400b37a0, "unknown_1400b37a0", tpl_55::<0x143a26668, 0x143a2666b, 0x143078ab8, 0x142409b50, 0x1425ea710, 0x141834d18> as *const ()), // sub_1400b37a0
    Replacement::draft(0x1400b3800, "unknown_1400b3800", tpl_41::<0x143a26708, 0x143a2670b, 0x143078ac8, 0x142409b50, 0x1425ea720, 0x141834d18> as *const ()), // sub_1400b3800
    Replacement::draft(0x1400b3860, "unknown_1400b3860", tpl_49::<0x143a267a8, 0x143a267ab, 0x143078ad8, 0x142409b50, 0x1425ea730, 0x141834d18> as *const ()), // sub_1400b3860
    Replacement::draft(0x1400b38c0, "unknown_1400b38c0", tpl_40::<0x143a25fc8, 0x143a25fcb, 0x143078ae4, 0x142409b50, 0x1425ea740, 0x141834d18> as *const ()), // sub_1400b38c0
    Replacement::draft(0x1400b3920, "unknown_1400b3920", tpl_44::<0x143a26100, 0x143a26103, 0x143078af0, 0x142409b50, 0x1425ea750, 0x141834d18> as *const ()), // sub_1400b3920
    Replacement::draft(0x1400b3980, "unknown_1400b3980", tpl_46::<0x143a25fa0, 0x143a25fa3, 0x143078b08, 0x142409b50, 0x1425ea760, 0x141834d18> as *const ()), // sub_1400b3980
    Replacement::draft(0x1400b39e0, "unknown_1400b39e0", tpl_55::<0x143a26690, 0x143a26693, 0x143078b20, 0x142409b50, 0x1425ea770, 0x141834d18> as *const ()), // sub_1400b39e0
    Replacement::draft(0x1400b3a40, "unknown_1400b3a40", tpl_46::<0x143a26528, 0x143a2652b, 0x143078b30, 0x142409b50, 0x1425ea780, 0x141834d18> as *const ()), // sub_1400b3a40
    Replacement::draft(0x1400b3aa0, "unknown_1400b3aa0", tpl_50::<0x143a26290, 0x143a26293, 0x143078b48, 0x142409b50, 0x1425ea790, 0x141834d18> as *const ()), // sub_1400b3aa0
    Replacement::draft(0x1400b3b00, "unknown_1400b3b00", tpl_42::<0x143a25f50, 0x143a25f53, 0x143078b60, 0x142409b50, 0x1425ea7a0, 0x141834d18> as *const ()), // sub_1400b3b00
    Replacement::draft(0x1400b3b60, "unknown_1400b3b60", tpl_38::<0x143a265f0, 0x143a265f3, 0x143078b78, 0x142409b50, 0x1425ea7b0, 0x141834d18> as *const ()), // sub_1400b3b60
    Replacement::draft(0x1400b3bc0, "unknown_1400b3bc0", tpl_34::<0x143a26960, 0x143a26963, 0x143078b88, 0x142409b50, 0x1425ea7c0, 0x141834d18> as *const ()), // sub_1400b3bc0
    Replacement::draft(0x1400b3c20, "unknown_1400b3c20", tpl_41::<0x143a26758, 0x143a2675b, 0x143078b98, 0x142409b50, 0x1425ea7d0, 0x141834d18> as *const ()), // sub_1400b3c20
    Replacement::draft(0x1400b3c80, "unknown_1400b3c80", tpl_36::<0x143a264d8, 0x143a264db, 0x143078ba8, 0x142409b50, 0x1425ea7e0, 0x141834d18> as *const ()), // sub_1400b3c80
    Replacement::draft(0x1400b3ce0, "unknown_1400b3ce0", tpl_34::<0x143a25f78, 0x143a25f7b, 0x143078bb8, 0x142409b50, 0x1425ea7f0, 0x141834d18> as *const ()), // sub_1400b3ce0
    Replacement::draft(0x1400b42e0, "unknown_1400b42e0", tpl_23::<0x143a269b0, 0x143a269b3, 0x1426cdcb0, 0x142409b50, 0x1425ea820, 0x141834d18> as *const ()), // sub_1400b42e0
    Replacement::draft(0x1400b4340, "unknown_1400b4340", tpl_23::<0x143a269f8, 0x143a269fb, 0x1426cdcb0, 0x142409b50, 0x1425ea830, 0x141834d18> as *const ()), // sub_1400b4340
    Replacement::draft(0x1400b4530, "unknown_1400b4530", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849348, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ea870, 0x141834d18> as *const ()), // sub_1400b4530
    Replacement::draft(0x1400b4ab0, "unknown_1400b4ab0", tpl_23::<0x143a26a60, 0x143a26a63, 0x1426cdcb0, 0x142409b50, 0x1425eac40, 0x141834d18> as *const ()), // sub_1400b4ab0
    Replacement::draft(0x1400b4b10, "unknown_1400b4b10", tpl_23::<0x143a26aa8, 0x143a26aab, 0x1426cdcb0, 0x142409b50, 0x1425eac50, 0x141834d18> as *const ()), // sub_1400b4b10
    Replacement::draft(0x1400b4d00, "unknown_1400b4d00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849480, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eac90, 0x141834d18> as *const ()), // sub_1400b4d00
    Replacement::draft(0x1400b51f0, "unknown_1400b51f0", tpl_23::<0x143a26b10, 0x143a26b13, 0x1426cdcb0, 0x142409b50, 0x1425ead00, 0x141834d18> as *const ()), // sub_1400b51f0
    Replacement::draft(0x1400b5250, "unknown_1400b5250", tpl_23::<0x143a26b58, 0x143a26b5b, 0x1426cdcb0, 0x142409b50, 0x1425ead10, 0x141834d18> as *const ()), // sub_1400b5250
    Replacement::draft(0x1400b5440, "unknown_1400b5440", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849510, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ead50, 0x141834d18> as *const ()), // sub_1400b5440
    Replacement::draft(0x1400b5840, "unknown_1400b5840", tpl_23::<0x143a26bc0, 0x143a26bc3, 0x1426cdcb0, 0x142409b50, 0x1425eadc0, 0x141834d18> as *const ()), // sub_1400b5840
    Replacement::draft(0x1400b58a0, "unknown_1400b58a0", tpl_23::<0x143a26c08, 0x143a26c0b, 0x1426cdcb0, 0x142409b50, 0x1425eadd0, 0x141834d18> as *const ()), // sub_1400b58a0
    Replacement::draft(0x1400b5a90, "unknown_1400b5a90", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438495a0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eae10, 0x141834d18> as *const ()), // sub_1400b5a90
    Replacement::draft(0x1400b5e90, "unknown_1400b5e90", tpl_23::<0x143a26c70, 0x143a26c73, 0x1426cdcb0, 0x142409b50, 0x1425eae80, 0x141834d18> as *const ()), // sub_1400b5e90
    Replacement::draft(0x1400b5ef0, "unknown_1400b5ef0", tpl_23::<0x143a26cb8, 0x143a26cbb, 0x1426cdcb0, 0x142409b50, 0x1425eae90, 0x141834d18> as *const ()), // sub_1400b5ef0
    Replacement::draft(0x1400b5fc0, "unknown_1400b5fc0", tpl_23::<0x143a26ce0, 0x143a26ce3, 0x1426cdcb0, 0x142409b50, 0x1425eaeb0, 0x141834d18> as *const ()), // sub_1400b5fc0
    Replacement::draft(0x1400b6020, "unknown_1400b6020", tpl_23::<0x143a26d28, 0x143a26d2b, 0x1426cdcb0, 0x142409b50, 0x1425eaec0, 0x141834d18> as *const ()), // sub_1400b6020
    Replacement::draft(0x1400b6210, "unknown_1400b6210", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849630, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eaf00, 0x141834d18> as *const ()), // sub_1400b6210
    Replacement::draft(0x1400b66b0, "unknown_1400b66b0", tpl_23::<0x143a26de0, 0x143a26de3, 0x1426cdcb0, 0x142409b50, 0x1425eaf70, 0x141834d18> as *const ()), // sub_1400b66b0
    Replacement::draft(0x1400b6710, "unknown_1400b6710", tpl_23::<0x143a26e28, 0x143a26e2b, 0x1426cdcb0, 0x142409b50, 0x1425eaf80, 0x141834d18> as *const ()), // sub_1400b6710
    Replacement::draft(0x1400b6900, "unknown_1400b6900", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438496c0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eafc0, 0x141834d18> as *const ()), // sub_1400b6900
    Replacement::draft(0x1400b6c90, "unknown_1400b6c90", tpl_23::<0x143a26e50, 0x143a26e53, 0x1426cdcb0, 0x142409b50, 0x1425eb000, 0x141834d18> as *const ()), // sub_1400b6c90
    Replacement::draft(0x1400b6cf0, "unknown_1400b6cf0", tpl_23::<0x143a26e98, 0x143a26e9b, 0x1426cdcb0, 0x142409b50, 0x1425eb010, 0x141834d18> as *const ()), // sub_1400b6cf0
    Replacement::draft(0x1400b6ee0, "unknown_1400b6ee0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849750, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eb050, 0x141834d18> as *const ()), // sub_1400b6ee0
    Replacement::draft(0x1400b7790, "unknown_1400b7790", tpl_23::<0x143a27110, 0x143a27113, 0x1426cdcb0, 0x142409b50, 0x1425eb0d0, 0x141834d18> as *const ()), // sub_1400b7790
    Replacement::draft(0x1400b77f0, "unknown_1400b77f0", tpl_23::<0x143a27158, 0x143a2715b, 0x1426cdcb0, 0x142409b50, 0x1425eb0e0, 0x141834d18> as *const ()), // sub_1400b77f0
    Replacement::draft(0x1400b79e0, "unknown_1400b79e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849800, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eb120, 0x141834d18> as *const ()), // sub_1400b79e0
    Replacement::draft(0x1400b7de0, "unknown_1400b7de0", tpl_34::<0x143a272c0, 0x143a272c3, 0x143078668, 0x142409b50, 0x1425eb190, 0x141834d18> as *const ()), // sub_1400b7de0
    Replacement::draft(0x1400b7e40, "unknown_1400b7e40", tpl_23::<0x143a27928, 0x143a2792b, 0x1426cdcb0, 0x142409b50, 0x1425eb1a0, 0x141834d18> as *const ()), // sub_1400b7e40
    Replacement::draft(0x1400b7ea0, "unknown_1400b7ea0", tpl_35::<0x143a27608, 0x143a2760b, 0x143078678, 0x142409b50, 0x1425eb1b0, 0x141834d18> as *const ()), // sub_1400b7ea0
    Replacement::draft(0x1400b7f00, "unknown_1400b7f00", tpl_36::<0x143a27860, 0x143a27863, 0x143078680, 0x142409b50, 0x1425eb1c0, 0x141834d18> as *const ()), // sub_1400b7f00
    Replacement::draft(0x1400b7f60, "unknown_1400b7f60", tpl_37::<0x143a274a8, 0x143a274ab, 0x143078690, 0x142409b50, 0x1425eb1d0, 0x141834d18> as *const ()), // sub_1400b7f60
    Replacement::draft(0x1400b7fc0, "unknown_1400b7fc0", tpl_38::<0x143a27368, 0x143a2736b, 0x1430786a0, 0x142409b50, 0x1425eb1e0, 0x141834d18> as *const ()), // sub_1400b7fc0
    Replacement::draft(0x1400b8020, "unknown_1400b8020", tpl_39::<0x143a27950, 0x143a27953, 0x1430786b0, 0x142409b50, 0x1425eb1f0, 0x141834d18> as *const ()), // sub_1400b8020
    Replacement::draft(0x1400b8080, "unknown_1400b8080", tpl_37::<0x143a27270, 0x143a27273, 0x1430786b8, 0x142409b50, 0x1425eb200, 0x141834d18> as *const ()), // sub_1400b8080
    Replacement::draft(0x1400b80e0, "unknown_1400b80e0", tpl_40::<0x143a27a68, 0x143a27a6b, 0x1430786c4, 0x142409b50, 0x1425eb210, 0x141834d18> as *const ()), // sub_1400b80e0
    Replacement::draft(0x1400b8140, "unknown_1400b8140", tpl_41::<0x143a27430, 0x143a27433, 0x1430786d0, 0x142409b50, 0x1425eb220, 0x141834d18> as *const ()), // sub_1400b8140
    Replacement::draft(0x1400b81a0, "unknown_1400b81a0", tpl_42::<0x143a279a0, 0x143a279a3, 0x1430786e0, 0x142409b50, 0x1425eb230, 0x141834d18> as *const ()), // sub_1400b81a0
    Replacement::draft(0x1400b8200, "unknown_1400b8200", tpl_34::<0x143a279f0, 0x143a279f3, 0x1430786f8, 0x142409b50, 0x1425eb240, 0x141834d18> as *const ()), // sub_1400b8200
    Replacement::draft(0x1400b8260, "unknown_1400b8260", tpl_34::<0x143a27298, 0x143a2729b, 0x143078708, 0x142409b50, 0x1425eb250, 0x141834d18> as *const ()), // sub_1400b8260
    Replacement::draft(0x1400b82c0, "unknown_1400b82c0", tpl_41::<0x143a27390, 0x143a27393, 0x143078718, 0x142409b50, 0x1425eb260, 0x141834d18> as *const ()), // sub_1400b82c0
    Replacement::draft(0x1400b8320, "unknown_1400b8320", tpl_43::<0x143a274d0, 0x143a274d3, 0x143078728, 0x142409b50, 0x1425eb270, 0x141834d18> as *const ()), // sub_1400b8320
    Replacement::draft(0x1400b8380, "unknown_1400b8380", tpl_44::<0x143a26f28, 0x143a26f2b, 0x143078748, 0x142409b50, 0x1425eb280, 0x141834d18> as *const ()), // sub_1400b8380
    Replacement::draft(0x1400b83e0, "unknown_1400b83e0", tpl_39::<0x143a27458, 0x143a2745b, 0x143078760, 0x142409b50, 0x1425eb290, 0x141834d18> as *const ()), // sub_1400b83e0
    Replacement::draft(0x1400b8440, "unknown_1400b8440", tpl_45::<0x143a27340, 0x143a27343, 0x143078768, 0x142409b50, 0x1425eb2a0, 0x141834d18> as *const ()), // sub_1400b8440
    Replacement::draft(0x1400b84a0, "unknown_1400b84a0", tpl_41::<0x143a27810, 0x143a27813, 0x143078778, 0x142409b50, 0x1425eb2b0, 0x141834d18> as *const ()), // sub_1400b84a0
    Replacement::draft(0x1400b8500, "unknown_1400b8500", tpl_37::<0x143a26f50, 0x143a26f53, 0x143078788, 0x142409b50, 0x1425eb2c0, 0x141834d18> as *const ()), // sub_1400b8500
    Replacement::draft(0x1400b8560, "unknown_1400b8560", tpl_46::<0x143a276a8, 0x143a276ab, 0x143078798, 0x142409b50, 0x1425eb2d0, 0x141834d18> as *const ()), // sub_1400b8560
    Replacement::draft(0x1400b85c0, "unknown_1400b85c0", tpl_47::<0x143a271d0, 0x143a271d3, 0x1430787b0, 0x142409b50, 0x1425eb2e0, 0x141834d18> as *const ()), // sub_1400b85c0
    Replacement::draft(0x1400b8620, "unknown_1400b8620", tpl_38::<0x143a27748, 0x143a2774b, 0x1430787d0, 0x142409b50, 0x1425eb2f0, 0x141834d18> as *const ()), // sub_1400b8620
    Replacement::draft(0x1400b8680, "unknown_1400b8680", tpl_48::<0x143a27978, 0x143a2797b, 0x1430787e0, 0x142409b50, 0x1425eb300, 0x141834d18> as *const ()), // sub_1400b8680
    Replacement::draft(0x1400b86e0, "unknown_1400b86e0", tpl_49::<0x143a275e0, 0x143a275e3, 0x1430787f8, 0x142409b50, 0x1425eb310, 0x141834d18> as *const ()), // sub_1400b86e0
    Replacement::draft(0x1400b8740, "unknown_1400b8740", tpl_44::<0x143a278b0, 0x143a278b3, 0x143078808, 0x142409b50, 0x1425eb320, 0x141834d18> as *const ()), // sub_1400b8740
    Replacement::draft(0x1400b87a0, "unknown_1400b87a0", tpl_42::<0x143a26f00, 0x143a26f03, 0x143078820, 0x142409b50, 0x1425eb330, 0x141834d18> as *const ()), // sub_1400b87a0
    Replacement::draft(0x1400b8800, "unknown_1400b8800", tpl_50::<0x143a277c0, 0x143a277c3, 0x143078838, 0x142409b50, 0x1425eb340, 0x141834d18> as *const ()), // sub_1400b8800
    Replacement::draft(0x1400b8860, "unknown_1400b8860", tpl_52::<0x143a270e8, 0x143a270eb, 0x143a27a68, 0x143a27a6c, 0x143a27a6b, 0x143a27a70, 0x142409b50, 0x1425eb350, 0x141834d18> as *const ()), // sub_1400b8860
    Replacement::draft(0x1400b8920, "unknown_1400b8920", tpl_38::<0x143a27630, 0x143a27633, 0x143078950, 0x142409b50, 0x1425eb360, 0x141834d18> as *const ()), // sub_1400b8920
    Replacement::draft(0x1400b8980, "unknown_1400b8980", tpl_53::<0x143a271a8, 0x143a271ab, 0x143078960, 0x142409b50, 0x1425eb370, 0x141834d18> as *const ()), // sub_1400b8980
    Replacement::draft(0x1400b89e0, "unknown_1400b89e0", tpl_45::<0x143a27070, 0x143a27073, 0x143078980, 0x142409b50, 0x1425eb380, 0x141834d18> as *const ()), // sub_1400b89e0
    Replacement::draft(0x1400b8a40, "unknown_1400b8a40", tpl_35::<0x143a270c0, 0x143a270c3, 0x14307898c, 0x142409b50, 0x1425eb390, 0x141834d18> as *const ()), // sub_1400b8a40
    Replacement::draft(0x1400b8aa0, "unknown_1400b8aa0", tpl_42::<0x143a279c8, 0x143a279cb, 0x1430786e0, 0x142409b50, 0x1425eb3a0, 0x141834d18> as *const ()), // sub_1400b8aa0
    Replacement::draft(0x1400b8b00, "unknown_1400b8b00", tpl_38::<0x143a27888, 0x143a2788b, 0x143078998, 0x142409b50, 0x1425eb3b0, 0x141834d18> as *const ()), // sub_1400b8b00
    Replacement::draft(0x1400b8b60, "unknown_1400b8b60", tpl_38::<0x143a273e0, 0x143a273e3, 0x1430789a8, 0x142409b50, 0x1425eb3c0, 0x141834d18> as *const ()), // sub_1400b8b60
    Replacement::draft(0x1400b8bc0, "unknown_1400b8bc0", tpl_41::<0x143a27540, 0x143a27543, 0x1430789b8, 0x142409b50, 0x1425eb3d0, 0x141834d18> as *const ()), // sub_1400b8bc0
    Replacement::draft(0x1400b8c20, "unknown_1400b8c20", tpl_34::<0x143a27a18, 0x143a27a1b, 0x1430789c8, 0x142409b50, 0x1425eb3e0, 0x141834d18> as *const ()), // sub_1400b8c20
    Replacement::draft(0x1400b8c80, "unknown_1400b8c80", tpl_39::<0x143a27098, 0x143a2709b, 0x1430789d8, 0x142409b50, 0x1425eb3f0, 0x141834d18> as *const ()), // sub_1400b8c80
    Replacement::draft(0x1400b8ce0, "unknown_1400b8ce0", tpl_41::<0x143a273b8, 0x143a273bb, 0x1430789e0, 0x142409b50, 0x1425eb400, 0x141834d18> as *const ()), // sub_1400b8ce0
    Replacement::draft(0x1400b8d40, "unknown_1400b8d40", tpl_34::<0x143a27658, 0x143a2765b, 0x1430789f0, 0x142409b50, 0x1425eb410, 0x141834d18> as *const ()), // sub_1400b8d40
    Replacement::draft(0x1400b8da0, "unknown_1400b8da0", tpl_48::<0x143a272e8, 0x143a272eb, 0x143078a00, 0x142409b50, 0x1425eb420, 0x141834d18> as *const ()), // sub_1400b8da0
    Replacement::draft(0x1400b8e00, "unknown_1400b8e00", tpl_46::<0x143a27770, 0x143a27773, 0x143078a18, 0x142409b50, 0x1425eb430, 0x141834d18> as *const ()), // sub_1400b8e00
    Replacement::draft(0x1400b8e60, "unknown_1400b8e60", tpl_47::<0x143a276d0, 0x143a276d3, 0x143078a30, 0x142409b50, 0x1425eb440, 0x141834d18> as *const ()), // sub_1400b8e60
    Replacement::draft(0x1400b8ec0, "unknown_1400b8ec0", tpl_37::<0x143a27220, 0x143a27223, 0x143078a50, 0x142409b50, 0x1425eb450, 0x141834d18> as *const ()), // sub_1400b8ec0
    Replacement::draft(0x1400b8f20, "unknown_1400b8f20", tpl_45::<0x143a27480, 0x143a27483, 0x143078a60, 0x142409b50, 0x1425eb460, 0x141834d18> as *const ()), // sub_1400b8f20
    Replacement::draft(0x1400b8f80, "unknown_1400b8f80", tpl_54::<0x143a27590, 0x143a27593, 0x143078a70, 0x142409b50, 0x1425eb470, 0x141834d18> as *const ()), // sub_1400b8f80
    Replacement::draft(0x1400b8fe0, "unknown_1400b8fe0", tpl_40::<0x143a271f8, 0x143a271fb, 0x143078a88, 0x142409b50, 0x1425eb480, 0x141834d18> as *const ()), // sub_1400b8fe0
    Replacement::draft(0x1400b9040, "unknown_1400b9040", tpl_50::<0x143a27248, 0x143a2724b, 0x143078a90, 0x142409b50, 0x1425eb490, 0x141834d18> as *const ()), // sub_1400b9040
    Replacement::draft(0x1400b90a0, "unknown_1400b90a0", tpl_41::<0x143a27408, 0x143a2740b, 0x143078aa8, 0x142409b50, 0x1425eb4a0, 0x141834d18> as *const ()), // sub_1400b90a0
    Replacement::draft(0x1400b9100, "unknown_1400b9100", tpl_55::<0x143a276f8, 0x143a276fb, 0x143078ab8, 0x142409b50, 0x1425eb4b0, 0x141834d18> as *const ()), // sub_1400b9100
    Replacement::draft(0x1400b9160, "unknown_1400b9160", tpl_41::<0x143a27798, 0x143a2779b, 0x143078ac8, 0x142409b50, 0x1425eb4c0, 0x141834d18> as *const ()), // sub_1400b9160
    Replacement::draft(0x1400b91c0, "unknown_1400b91c0", tpl_49::<0x143a27838, 0x143a2783b, 0x143078ad8, 0x142409b50, 0x1425eb4d0, 0x141834d18> as *const ()), // sub_1400b91c0
    Replacement::draft(0x1400b9220, "unknown_1400b9220", tpl_40::<0x143a26ff0, 0x143a26ff3, 0x143078ae4, 0x142409b50, 0x1425eb4e0, 0x141834d18> as *const ()), // sub_1400b9220
    Replacement::draft(0x1400b9280, "unknown_1400b9280", tpl_44::<0x143a27180, 0x143a27183, 0x143078af0, 0x142409b50, 0x1425eb4f0, 0x141834d18> as *const ()), // sub_1400b9280
    Replacement::draft(0x1400b92e0, "unknown_1400b92e0", tpl_46::<0x143a26fc8, 0x143a26fcb, 0x143078b08, 0x142409b50, 0x1425eb500, 0x141834d18> as *const ()), // sub_1400b92e0
    Replacement::draft(0x1400b9340, "unknown_1400b9340", tpl_55::<0x143a27720, 0x143a27723, 0x143078b20, 0x142409b50, 0x1425eb510, 0x141834d18> as *const ()), // sub_1400b9340
    Replacement::draft(0x1400b93a0, "unknown_1400b93a0", tpl_46::<0x143a275b8, 0x143a275bb, 0x143078b30, 0x142409b50, 0x1425eb520, 0x141834d18> as *const ()), // sub_1400b93a0
    Replacement::draft(0x1400b9400, "unknown_1400b9400", tpl_50::<0x143a27310, 0x143a27313, 0x143078b48, 0x142409b50, 0x1425eb530, 0x141834d18> as *const ()), // sub_1400b9400
    Replacement::draft(0x1400b9460, "unknown_1400b9460", tpl_42::<0x143a26f78, 0x143a26f7b, 0x143078b60, 0x142409b50, 0x1425eb540, 0x141834d18> as *const ()), // sub_1400b9460
    Replacement::draft(0x1400b94c0, "unknown_1400b94c0", tpl_38::<0x143a27680, 0x143a27683, 0x143078b78, 0x142409b50, 0x1425eb550, 0x141834d18> as *const ()), // sub_1400b94c0
    Replacement::draft(0x1400b9520, "unknown_1400b9520", tpl_34::<0x143a27a40, 0x143a27a43, 0x143078b88, 0x142409b50, 0x1425eb560, 0x141834d18> as *const ()), // sub_1400b9520
    Replacement::draft(0x1400b9580, "unknown_1400b9580", tpl_41::<0x143a277e8, 0x143a277eb, 0x143078b98, 0x142409b50, 0x1425eb570, 0x141834d18> as *const ()), // sub_1400b9580
    Replacement::draft(0x1400b95e0, "unknown_1400b95e0", tpl_36::<0x143a27568, 0x143a2756b, 0x143078ba8, 0x142409b50, 0x1425eb580, 0x141834d18> as *const ()), // sub_1400b95e0
    Replacement::draft(0x1400b9640, "unknown_1400b9640", tpl_34::<0x143a26fa0, 0x143a26fa3, 0x143078bb8, 0x142409b50, 0x1425eb590, 0x141834d18> as *const ()), // sub_1400b9640
    Replacement::draft(0x1400b96a0, "unknown_1400b96a0", tpl_61::<0x143a278e0, 0x143a278e3, 0x143091fd8, 0x142409b50, 0x1425eb5a0, 0x141834d18> as *const ()), // sub_1400b96a0
    Replacement::draft(0x1400b9700, "unknown_1400b9700", tpl_62::<0x143a278e3, 0x143a278e0, 0x143a278e8, 0x143a278e4, 0x143a27338> as *const ()), // sub_1400b9700
    Replacement::draft(0x1400b99c0, "unknown_1400b99c0", fn_1400b99c0 as *const ()), // sub_1400b99c0
    Replacement::draft(0x1400b9a50, "unknown_1400b9a50", fn_1400b9a50 as *const ()), // sub_1400b9a50
    Replacement::draft(0x1400ba9f0, "unknown_1400ba9f0", tpl_23::<0x143a27c48, 0x143a27c4b, 0x1426cdcb0, 0x142409b50, 0x1425eba80, 0x141834d18> as *const ()), // sub_1400ba9f0
    Replacement::draft(0x1400baa50, "unknown_1400baa50", tpl_23::<0x143a27c90, 0x143a27c93, 0x1426cdcb0, 0x142409b50, 0x1425eba90, 0x141834d18> as *const ()), // sub_1400baa50
    Replacement::draft(0x1400bac40, "unknown_1400bac40", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438499d8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ebad0, 0x141834d18> as *const ()), // sub_1400bac40
    Replacement::draft(0x1400bb040, "unknown_1400bb040", tpl_34::<0x143a27df8, 0x143a27dfb, 0x143078668, 0x142409b50, 0x1425ebb40, 0x141834d18> as *const ()), // sub_1400bb040
    Replacement::draft(0x1400bb0a0, "unknown_1400bb0a0", tpl_23::<0x143a28408, 0x143a2840b, 0x1426cdcb0, 0x142409b50, 0x1425ebb50, 0x141834d18> as *const ()), // sub_1400bb0a0
    Replacement::draft(0x1400bb100, "unknown_1400bb100", tpl_35::<0x143a28138, 0x143a2813b, 0x143078678, 0x142409b50, 0x1425ebb60, 0x141834d18> as *const ()), // sub_1400bb100
    Replacement::draft(0x1400bb160, "unknown_1400bb160", tpl_36::<0x143a28390, 0x143a28393, 0x143078680, 0x142409b50, 0x1425ebb70, 0x141834d18> as *const ()), // sub_1400bb160
    Replacement::draft(0x1400bb1c0, "unknown_1400bb1c0", tpl_37::<0x143a27fd8, 0x143a27fdb, 0x143078690, 0x142409b50, 0x1425ebb80, 0x141834d18> as *const ()), // sub_1400bb1c0
    Replacement::draft(0x1400bb220, "unknown_1400bb220", tpl_38::<0x143a27e98, 0x143a27e9b, 0x1430786a0, 0x142409b50, 0x1425ebb90, 0x141834d18> as *const ()), // sub_1400bb220
    Replacement::draft(0x1400bb280, "unknown_1400bb280", tpl_39::<0x143a28430, 0x143a28433, 0x1430786b0, 0x142409b50, 0x1425ebba0, 0x141834d18> as *const ()), // sub_1400bb280
    Replacement::draft(0x1400bb2e0, "unknown_1400bb2e0", tpl_37::<0x143a27da8, 0x143a27dab, 0x1430786b8, 0x142409b50, 0x1425ebbb0, 0x141834d18> as *const ()), // sub_1400bb2e0
    Replacement::draft(0x1400bb340, "unknown_1400bb340", tpl_40::<0x143a28548, 0x143a2854b, 0x1430786c4, 0x142409b50, 0x1425ebbc0, 0x141834d18> as *const ()), // sub_1400bb340
    Replacement::draft(0x1400bb3a0, "unknown_1400bb3a0", tpl_41::<0x143a27f60, 0x143a27f63, 0x1430786d0, 0x142409b50, 0x1425ebbd0, 0x141834d18> as *const ()), // sub_1400bb3a0
    Replacement::draft(0x1400bb400, "unknown_1400bb400", tpl_42::<0x143a28480, 0x143a28483, 0x1430786e0, 0x142409b50, 0x1425ebbe0, 0x141834d18> as *const ()), // sub_1400bb400
    Replacement::draft(0x1400bb460, "unknown_1400bb460", tpl_34::<0x143a284d0, 0x143a284d3, 0x1430786f8, 0x142409b50, 0x1425ebbf0, 0x141834d18> as *const ()), // sub_1400bb460
    Replacement::draft(0x1400bb4c0, "unknown_1400bb4c0", tpl_34::<0x143a27dd0, 0x143a27dd3, 0x143078708, 0x142409b50, 0x1425ebc00, 0x141834d18> as *const ()), // sub_1400bb4c0
    Replacement::draft(0x1400bb520, "unknown_1400bb520", tpl_41::<0x143a27ec0, 0x143a27ec3, 0x143078718, 0x142409b50, 0x1425ebc10, 0x141834d18> as *const ()), // sub_1400bb520
    Replacement::draft(0x1400bb580, "unknown_1400bb580", tpl_43::<0x143a28000, 0x143a28003, 0x143078728, 0x142409b50, 0x1425ebc20, 0x141834d18> as *const ()), // sub_1400bb580
    Replacement::draft(0x1400bb5e0, "unknown_1400bb5e0", tpl_44::<0x143a27ab8, 0x143a27abb, 0x143078748, 0x142409b50, 0x1425ebc30, 0x141834d18> as *const ()), // sub_1400bb5e0
    Replacement::draft(0x1400bb640, "unknown_1400bb640", tpl_39::<0x143a27f88, 0x143a27f8b, 0x143078760, 0x142409b50, 0x1425ebc40, 0x141834d18> as *const ()), // sub_1400bb640
    Replacement::draft(0x1400bb6a0, "unknown_1400bb6a0", tpl_45::<0x143a27e70, 0x143a27e73, 0x143078768, 0x142409b50, 0x1425ebc50, 0x141834d18> as *const ()), // sub_1400bb6a0
    Replacement::draft(0x1400bb700, "unknown_1400bb700", tpl_41::<0x143a28340, 0x143a28343, 0x143078778, 0x142409b50, 0x1425ebc60, 0x141834d18> as *const ()), // sub_1400bb700
    Replacement::draft(0x1400bb760, "unknown_1400bb760", tpl_37::<0x143a27ae0, 0x143a27ae3, 0x143078788, 0x142409b50, 0x1425ebc70, 0x141834d18> as *const ()), // sub_1400bb760
    Replacement::draft(0x1400bb7c0, "unknown_1400bb7c0", tpl_46::<0x143a281d8, 0x143a281db, 0x143078798, 0x142409b50, 0x1425ebc80, 0x141834d18> as *const ()), // sub_1400bb7c0
    Replacement::draft(0x1400bb820, "unknown_1400bb820", tpl_47::<0x143a27d08, 0x143a27d0b, 0x1430787b0, 0x142409b50, 0x1425ebc90, 0x141834d18> as *const ()), // sub_1400bb820
    Replacement::draft(0x1400bb880, "unknown_1400bb880", tpl_38::<0x143a28278, 0x143a2827b, 0x1430787d0, 0x142409b50, 0x1425ebca0, 0x141834d18> as *const ()), // sub_1400bb880
    Replacement::draft(0x1400bb8e0, "unknown_1400bb8e0", tpl_48::<0x143a28458, 0x143a2845b, 0x1430787e0, 0x142409b50, 0x1425ebcb0, 0x141834d18> as *const ()), // sub_1400bb8e0
    Replacement::draft(0x1400bb940, "unknown_1400bb940", tpl_49::<0x143a28110, 0x143a28113, 0x1430787f8, 0x142409b50, 0x1425ebcc0, 0x141834d18> as *const ()), // sub_1400bb940
    Replacement::draft(0x1400bb9a0, "unknown_1400bb9a0", tpl_44::<0x143a283e0, 0x143a283e3, 0x143078808, 0x142409b50, 0x1425ebcd0, 0x141834d18> as *const ()), // sub_1400bb9a0
    Replacement::draft(0x1400bba00, "unknown_1400bba00", tpl_42::<0x143a27a90, 0x143a27a93, 0x143078820, 0x142409b50, 0x1425ebce0, 0x141834d18> as *const ()), // sub_1400bba00
    Replacement::draft(0x1400bba60, "unknown_1400bba60", tpl_50::<0x143a282f0, 0x143a282f3, 0x143078838, 0x142409b50, 0x1425ebcf0, 0x141834d18> as *const ()), // sub_1400bba60
    Replacement::draft(0x1400bbac0, "unknown_1400bbac0", tpl_52::<0x143a27c20, 0x143a27c23, 0x143a28548, 0x143a2854c, 0x143a2854b, 0x143a28550, 0x142409b50, 0x1425ebd00, 0x141834d18> as *const ()), // sub_1400bbac0
    Replacement::draft(0x1400bbb80, "unknown_1400bbb80", tpl_38::<0x143a28160, 0x143a28163, 0x143078950, 0x142409b50, 0x1425ebd10, 0x141834d18> as *const ()), // sub_1400bbb80
    Replacement::draft(0x1400bbbe0, "unknown_1400bbbe0", tpl_53::<0x143a27ce0, 0x143a27ce3, 0x143078960, 0x142409b50, 0x1425ebd20, 0x141834d18> as *const ()), // sub_1400bbbe0
    Replacement::draft(0x1400bbc40, "unknown_1400bbc40", tpl_45::<0x143a27ba8, 0x143a27bab, 0x143078980, 0x142409b50, 0x1425ebd30, 0x141834d18> as *const ()), // sub_1400bbc40
    Replacement::draft(0x1400bbca0, "unknown_1400bbca0", tpl_35::<0x143a27bf8, 0x143a27bfb, 0x14307898c, 0x142409b50, 0x1425ebd40, 0x141834d18> as *const ()), // sub_1400bbca0
    Replacement::draft(0x1400bbd00, "unknown_1400bbd00", tpl_42::<0x143a284a8, 0x143a284ab, 0x1430786e0, 0x142409b50, 0x1425ebd50, 0x141834d18> as *const ()), // sub_1400bbd00
    Replacement::draft(0x1400bbd60, "unknown_1400bbd60", tpl_38::<0x143a283b8, 0x143a283bb, 0x143078998, 0x142409b50, 0x1425ebd60, 0x141834d18> as *const ()), // sub_1400bbd60
    Replacement::draft(0x1400bbdc0, "unknown_1400bbdc0", tpl_38::<0x143a27f10, 0x143a27f13, 0x1430789a8, 0x142409b50, 0x1425ebd70, 0x141834d18> as *const ()), // sub_1400bbdc0
    Replacement::draft(0x1400bbe20, "unknown_1400bbe20", tpl_41::<0x143a28070, 0x143a28073, 0x1430789b8, 0x142409b50, 0x1425ebd80, 0x141834d18> as *const ()), // sub_1400bbe20
    Replacement::draft(0x1400bbe80, "unknown_1400bbe80", tpl_34::<0x143a284f8, 0x143a284fb, 0x1430789c8, 0x142409b50, 0x1425ebd90, 0x141834d18> as *const ()), // sub_1400bbe80
    Replacement::draft(0x1400bbee0, "unknown_1400bbee0", tpl_39::<0x143a27bd0, 0x143a27bd3, 0x1430789d8, 0x142409b50, 0x1425ebda0, 0x141834d18> as *const ()), // sub_1400bbee0
    Replacement::draft(0x1400bbf40, "unknown_1400bbf40", tpl_41::<0x143a27ee8, 0x143a27eeb, 0x1430789e0, 0x142409b50, 0x1425ebdb0, 0x141834d18> as *const ()), // sub_1400bbf40
    Replacement::draft(0x1400bbfa0, "unknown_1400bbfa0", tpl_34::<0x143a28188, 0x143a2818b, 0x1430789f0, 0x142409b50, 0x1425ebdc0, 0x141834d18> as *const ()), // sub_1400bbfa0
    Replacement::draft(0x1400bc000, "unknown_1400bc000", tpl_48::<0x143a27e20, 0x143a27e23, 0x143078a00, 0x142409b50, 0x1425ebdd0, 0x141834d18> as *const ()), // sub_1400bc000
    Replacement::draft(0x1400bc060, "unknown_1400bc060", tpl_46::<0x143a282a0, 0x143a282a3, 0x143078a18, 0x142409b50, 0x1425ebde0, 0x141834d18> as *const ()), // sub_1400bc060
    Replacement::draft(0x1400bc0c0, "unknown_1400bc0c0", tpl_47::<0x143a28200, 0x143a28203, 0x143078a30, 0x142409b50, 0x1425ebdf0, 0x141834d18> as *const ()), // sub_1400bc0c0
    Replacement::draft(0x1400bc120, "unknown_1400bc120", tpl_37::<0x143a27d58, 0x143a27d5b, 0x143078a50, 0x142409b50, 0x1425ebe00, 0x141834d18> as *const ()), // sub_1400bc120
    Replacement::draft(0x1400bc180, "unknown_1400bc180", tpl_45::<0x143a27fb0, 0x143a27fb3, 0x143078a60, 0x142409b50, 0x1425ebe10, 0x141834d18> as *const ()), // sub_1400bc180
    Replacement::draft(0x1400bc1e0, "unknown_1400bc1e0", tpl_54::<0x143a280c0, 0x143a280c3, 0x143078a70, 0x142409b50, 0x1425ebe20, 0x141834d18> as *const ()), // sub_1400bc1e0
    Replacement::draft(0x1400bc240, "unknown_1400bc240", tpl_40::<0x143a27d30, 0x143a27d33, 0x143078a88, 0x142409b50, 0x1425ebe30, 0x141834d18> as *const ()), // sub_1400bc240
    Replacement::draft(0x1400bc2a0, "unknown_1400bc2a0", tpl_50::<0x143a27d80, 0x143a27d83, 0x143078a90, 0x142409b50, 0x1425ebe40, 0x141834d18> as *const ()), // sub_1400bc2a0
    Replacement::draft(0x1400bc300, "unknown_1400bc300", tpl_41::<0x143a27f38, 0x143a27f3b, 0x143078aa8, 0x142409b50, 0x1425ebe50, 0x141834d18> as *const ()), // sub_1400bc300
    Replacement::draft(0x1400bc360, "unknown_1400bc360", tpl_55::<0x143a28228, 0x143a2822b, 0x143078ab8, 0x142409b50, 0x1425ebe60, 0x141834d18> as *const ()), // sub_1400bc360
    Replacement::draft(0x1400bc3c0, "unknown_1400bc3c0", tpl_41::<0x143a282c8, 0x143a282cb, 0x143078ac8, 0x142409b50, 0x1425ebe70, 0x141834d18> as *const ()), // sub_1400bc3c0
    Replacement::draft(0x1400bc420, "unknown_1400bc420", tpl_49::<0x143a28368, 0x143a2836b, 0x143078ad8, 0x142409b50, 0x1425ebe80, 0x141834d18> as *const ()), // sub_1400bc420
    Replacement::draft(0x1400bc480, "unknown_1400bc480", tpl_40::<0x143a27b80, 0x143a27b83, 0x143078ae4, 0x142409b50, 0x1425ebe90, 0x141834d18> as *const ()), // sub_1400bc480
    Replacement::draft(0x1400bc4e0, "unknown_1400bc4e0", tpl_44::<0x143a27cb8, 0x143a27cbb, 0x143078af0, 0x142409b50, 0x1425ebea0, 0x141834d18> as *const ()), // sub_1400bc4e0
    Replacement::draft(0x1400bc540, "unknown_1400bc540", tpl_46::<0x143a27b58, 0x143a27b5b, 0x143078b08, 0x142409b50, 0x1425ebeb0, 0x141834d18> as *const ()), // sub_1400bc540
    Replacement::draft(0x1400bc5a0, "unknown_1400bc5a0", tpl_55::<0x143a28250, 0x143a28253, 0x143078b20, 0x142409b50, 0x1425ebec0, 0x141834d18> as *const ()), // sub_1400bc5a0
    Replacement::draft(0x1400bc600, "unknown_1400bc600", tpl_46::<0x143a280e8, 0x143a280eb, 0x143078b30, 0x142409b50, 0x1425ebed0, 0x141834d18> as *const ()), // sub_1400bc600
    Replacement::draft(0x1400bc660, "unknown_1400bc660", tpl_50::<0x143a27e48, 0x143a27e4b, 0x143078b48, 0x142409b50, 0x1425ebee0, 0x141834d18> as *const ()), // sub_1400bc660
    Replacement::draft(0x1400bc6c0, "unknown_1400bc6c0", tpl_42::<0x143a27b08, 0x143a27b0b, 0x143078b60, 0x142409b50, 0x1425ebef0, 0x141834d18> as *const ()), // sub_1400bc6c0
    Replacement::draft(0x1400bc720, "unknown_1400bc720", tpl_38::<0x143a281b0, 0x143a281b3, 0x143078b78, 0x142409b50, 0x1425ebf00, 0x141834d18> as *const ()), // sub_1400bc720
    Replacement::draft(0x1400bc780, "unknown_1400bc780", tpl_34::<0x143a28520, 0x143a28523, 0x143078b88, 0x142409b50, 0x1425ebf10, 0x141834d18> as *const ()), // sub_1400bc780
    Replacement::draft(0x1400bc7e0, "unknown_1400bc7e0", tpl_41::<0x143a28318, 0x143a2831b, 0x143078b98, 0x142409b50, 0x1425ebf20, 0x141834d18> as *const ()), // sub_1400bc7e0
    Replacement::draft(0x1400bc840, "unknown_1400bc840", tpl_36::<0x143a28098, 0x143a2809b, 0x143078ba8, 0x142409b50, 0x1425ebf30, 0x141834d18> as *const ()), // sub_1400bc840
    Replacement::draft(0x1400bc8a0, "unknown_1400bc8a0", tpl_34::<0x143a27b30, 0x143a27b33, 0x143078bb8, 0x142409b50, 0x1425ebf40, 0x141834d18> as *const ()), // sub_1400bc8a0
    Replacement::draft(0x1400bd240, "unknown_1400bd240", tpl_61::<0x143a28580, 0x143a28583, 0x143091fd8, 0x142409b50, 0x1425ebff0, 0x141834d18> as *const ()), // sub_1400bd240
    Replacement::draft(0x1400bd2a0, "unknown_1400bd2a0", tpl_62::<0x143a28583, 0x143a28580, 0x143a28588, 0x143a28584, 0x143a28570> as *const ()), // sub_1400bd2a0
    Replacement::draft(0x1400bdba0, "unknown_1400bdba0", tpl_61::<0x143a287f0, 0x143a287f3, 0x143091fd8, 0x142409b50, 0x1425ec030, 0x141834d18> as *const ()), // sub_1400bdba0
    Replacement::draft(0x1400bdc00, "unknown_1400bdc00", tpl_62::<0x143a287f3, 0x143a287f0, 0x143a287f8, 0x143a287f4, 0x143a28650> as *const ()), // sub_1400bdc00
    Replacement::draft(0x1400bdc90, "unknown_1400bdc90", tpl_63::<0x143a287a0, 0x143a287a3, 0x143091ff0, 0x142409b50, 0x1425ec040, 0x141834d18> as *const ()), // sub_1400bdc90
    Replacement::draft(0x1400bdcf0, "unknown_1400bdcf0", tpl_64::<0x143a286e0, 0x143a286e3, 0x143091fa0, 0x142409b50, 0x1425ec050, 0x141834d18> as *const ()), // sub_1400bdcf0
    Replacement::draft(0x1400bdd50, "unknown_1400bdd50", tpl_65::<0x143a28660, 0x143a28663, 0x143091fc0, 0x142409b50, 0x1425ec060, 0x141834d18> as *const ()), // sub_1400bdd50
    Replacement::draft(0x1400bddb0, "unknown_1400bddb0", tpl_62::<0x143a287a3, 0x143a287a0, 0x143a287a8, 0x143a287a4, 0x143a28790> as *const ()), // sub_1400bddb0
    Replacement::draft(0x1400bde40, "unknown_1400bde40", tpl_62::<0x143a286e3, 0x143a286e0, 0x143a286e8, 0x143a286e4, 0x143a286a8> as *const ()), // sub_1400bde40
    Replacement::draft(0x1400bded0, "unknown_1400bded0", tpl_62::<0x143a28663, 0x143a28660, 0x143a28668, 0x143a28664, 0x143a286b0> as *const ()), // sub_1400bded0
    Replacement::draft(0x1400bdf60, "unknown_1400bdf60", tpl_23::<0x143a285c8, 0x143a285cb, 0x1426cdcb0, 0x142409b50, 0x1425ec070, 0x141834d18> as *const ()), // sub_1400bdf60
    Replacement::draft(0x1400bdfc0, "unknown_1400bdfc0", tpl_23::<0x143a28610, 0x143a28613, 0x1426cdcb0, 0x142409b50, 0x1425ec080, 0x141834d18> as *const ()), // sub_1400bdfc0
    Replacement::draft(0x1400be1b0, "unknown_1400be1b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849b48, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ec0c0, 0x141834d18> as *const ()), // sub_1400be1b0
    Replacement::draft(0x1400beec0, "unknown_1400beec0", tpl_23::<0x143a28880, 0x143a28883, 0x1426cdcb0, 0x142409b50, 0x1425ec160, 0x141834d18> as *const ()), // sub_1400beec0
    Replacement::draft(0x1400bef20, "unknown_1400bef20", tpl_23::<0x143a288c8, 0x143a288cb, 0x1426cdcb0, 0x142409b50, 0x1425ec170, 0x141834d18> as *const ()), // sub_1400bef20
    Replacement::draft(0x1400bf110, "unknown_1400bf110", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849bd8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ec1b0, 0x141834d18> as *const ()), // sub_1400bf110
    Replacement::draft(0x1400bf550, "unknown_1400bf550", fn_1400bf550 as *const ()), // sub_1400bf550
    Replacement::draft(0x1400bf5a0, "unknown_1400bf5a0", tpl_23::<0x143a28930, 0x143a28933, 0x1426cdcb0, 0x142409b50, 0x1425ec2d0, 0x141834d18> as *const ()), // sub_1400bf5a0
    Replacement::draft(0x1400bf600, "unknown_1400bf600", tpl_23::<0x143a28978, 0x143a2897b, 0x1426cdcb0, 0x142409b50, 0x1425ec2e0, 0x141834d18> as *const ()), // sub_1400bf600
    Replacement::draft(0x1400bf7f0, "unknown_1400bf7f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849cc0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ec320, 0x141834d18> as *const ()), // sub_1400bf7f0
    Replacement::draft(0x1400c0560, "unknown_1400c0560", tpl_23::<0x143a289e0, 0x143a289e3, 0x1426cdcb0, 0x142409b50, 0x1425ec560, 0x141834d18> as *const ()), // sub_1400c0560
    Replacement::draft(0x1400c05c0, "unknown_1400c05c0", tpl_23::<0x143a28a28, 0x143a28a2b, 0x1426cdcb0, 0x142409b50, 0x1425ec570, 0x141834d18> as *const ()), // sub_1400c05c0
    Replacement::draft(0x1400c07b0, "unknown_1400c07b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849d88, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ec5b0, 0x141834d18> as *const ()), // sub_1400c07b0
    Replacement::draft(0x1400c0bf0, "unknown_1400c0bf0", tpl_120::<0x143a28b60, 0x143a28b63, 0x143079988, 0x142409b50, 0x143a28b88, 0x143a28b90, 0x143a28ba0, 0x143a28bb0, 0x1425ec6b0, 0x141834d18> as *const ()), // sub_1400c0bf0
    Replacement::draft(0x1400c0c80, "unknown_1400c0c80", tpl_120::<0x143a28ac0, 0x143a28ac3, 0x143079988, 0x142409b50, 0x143a28ae8, 0x143a28af0, 0x143a28b00, 0x143a28b10, 0x1425ec6c0, 0x141834d18> as *const ()), // sub_1400c0c80
    Replacement::draft(0x1400c0d10, "unknown_1400c0d10", tpl_121::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143a28bb8, 0x1425ec6d0, 0x141834d18> as *const ()), // sub_1400c0d10
    Replacement::draft(0x1400c1350, "unknown_1400c1350", tpl_23::<0x143a28bc8, 0x143a28bcb, 0x1426cdcb0, 0x142409b50, 0x1425ec8c0, 0x141834d18> as *const ()), // sub_1400c1350
    Replacement::draft(0x1400c13b0, "unknown_1400c13b0", tpl_23::<0x143a28c10, 0x143a28c13, 0x1426cdcb0, 0x142409b50, 0x1425ec8d0, 0x141834d18> as *const ()), // sub_1400c13b0
    Replacement::draft(0x1400c15a0, "unknown_1400c15a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143849ec8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ec910, 0x141834d18> as *const ()), // sub_1400c15a0
    Replacement::draft(0x1400c1a20, "unknown_1400c1a20", tpl_56::<0x1434268b0, 0x1434268fc, 0x143a001ac, 0x143849e58, 0x143849e70, 0x143a00290, 0x143849e48, 0x1425ecaa0, 0x141834d18> as *const ()), // sub_1400c1a20
    Replacement::draft(0x1400c1d70, "unknown_1400c1d70", tpl_23::<0x143a28c80, 0x143a28c83, 0x1426cdcb0, 0x142409b50, 0x1425ecd70, 0x141834d18> as *const ()), // sub_1400c1d70
    Replacement::draft(0x1400c1dd0, "unknown_1400c1dd0", tpl_23::<0x143a28cc8, 0x143a28ccb, 0x1426cdcb0, 0x142409b50, 0x1425ecd80, 0x141834d18> as *const ()), // sub_1400c1dd0
    Replacement::draft(0x1400c1fc0, "unknown_1400c1fc0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a078, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ecdc0, 0x141834d18> as *const ()), // sub_1400c1fc0
    Replacement::draft(0x1400c4390, "unknown_1400c4390", tpl_23::<0x143a28cf0, 0x143a28cf3, 0x1426cdcb0, 0x142409b50, 0x1425ed3a0, 0x141834d18> as *const ()), // sub_1400c4390
    Replacement::draft(0x1400c43f0, "unknown_1400c43f0", tpl_23::<0x143a28d38, 0x143a28d3b, 0x1426cdcb0, 0x142409b50, 0x1425ed3b0, 0x141834d18> as *const ()), // sub_1400c43f0
    Replacement::draft(0x1400c45e0, "unknown_1400c45e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a2e8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ed3f0, 0x141834d18> as *const ()), // sub_1400c45e0
    Replacement::draft(0x1400c4a90, "unknown_1400c4a90", tpl_23::<0x143a28db0, 0x143a28db3, 0x1426cdcb0, 0x142409b50, 0x1425ed480, 0x141834d18> as *const ()), // sub_1400c4a90
    Replacement::draft(0x1400c4af0, "unknown_1400c4af0", tpl_23::<0x143a28df8, 0x143a28dfb, 0x1426cdcb0, 0x142409b50, 0x1425ed490, 0x141834d18> as *const ()), // sub_1400c4af0
    Replacement::draft(0x1400c4ce0, "unknown_1400c4ce0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a378, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ed4d0, 0x141834d18> as *const ()), // sub_1400c4ce0
    Replacement::draft(0x1400c63a0, "unknown_1400c63a0", tpl_23::<0x143a28eb0, 0x143a28eb3, 0x1426cdcb0, 0x142409b50, 0x1425ed620, 0x141834d18> as *const ()), // sub_1400c63a0
    Replacement::draft(0x1400c6400, "unknown_1400c6400", tpl_23::<0x143a28ef8, 0x143a28efb, 0x1426cdcb0, 0x142409b50, 0x1425ed630, 0x141834d18> as *const ()), // sub_1400c6400
    Replacement::draft(0x1400c65f0, "unknown_1400c65f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a408, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ed670, 0x141834d18> as *const ()), // sub_1400c65f0
    Replacement::draft(0x1400c7620, "unknown_1400c7620", tpl_23::<0x143a28f60, 0x143a28f63, 0x1426cdcb0, 0x142409b50, 0x1425ed700, 0x141834d18> as *const ()), // sub_1400c7620
    Replacement::draft(0x1400c7680, "unknown_1400c7680", tpl_23::<0x143a28fa8, 0x143a28fab, 0x1426cdcb0, 0x142409b50, 0x1425ed710, 0x141834d18> as *const ()), // sub_1400c7680
    Replacement::draft(0x1400c7870, "unknown_1400c7870", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a498, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ed750, 0x141834d18> as *const ()), // sub_1400c7870
    Replacement::draft(0x1400c7f40, "unknown_1400c7f40", tpl_23::<0x143a29010, 0x143a29013, 0x1426cdcb0, 0x142409b50, 0x1425ed7c0, 0x141834d18> as *const ()), // sub_1400c7f40
    Replacement::draft(0x1400c7fa0, "unknown_1400c7fa0", tpl_23::<0x143a29058, 0x143a2905b, 0x1426cdcb0, 0x142409b50, 0x1425ed7d0, 0x141834d18> as *const ()), // sub_1400c7fa0
    Replacement::draft(0x1400c8190, "unknown_1400c8190", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a528, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ed810, 0x141834d18> as *const ()), // sub_1400c8190
    Replacement::draft(0x1400c8d70, "unknown_1400c8d70", tpl_23::<0x143a290c0, 0x143a290c3, 0x1426cdcb0, 0x142409b50, 0x1425eddc0, 0x141834d18> as *const ()), // sub_1400c8d70
    Replacement::draft(0x1400c8dd0, "unknown_1400c8dd0", tpl_23::<0x143a29108, 0x143a2910b, 0x1426cdcb0, 0x142409b50, 0x1425eddd0, 0x141834d18> as *const ()), // sub_1400c8dd0
    Replacement::draft(0x1400c8fc0, "unknown_1400c8fc0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a760, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ede10, 0x141834d18> as *const ()), // sub_1400c8fc0
    Replacement::draft(0x1400c9740, "unknown_1400c9740", fn_1400c9740 as *const ()), // sub_1400c9740
    Replacement::draft(0x1400ca480, "unknown_1400ca480", tpl_23::<0x143a29178, 0x143a2917b, 0x1426cdcb0, 0x142409b50, 0x1425ee760, 0x141834d18> as *const ()), // sub_1400ca480
    Replacement::draft(0x1400ca4e0, "unknown_1400ca4e0", tpl_23::<0x143a291c0, 0x143a291c3, 0x1426cdcb0, 0x142409b50, 0x1425ee770, 0x141834d18> as *const ()), // sub_1400ca4e0
    Replacement::draft(0x1400ca6d0, "unknown_1400ca6d0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a918, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ee7b0, 0x141834d18> as *const ()), // sub_1400ca6d0
    Replacement::draft(0x1400cacb0, "unknown_1400cacb0", tpl_23::<0x143a29230, 0x143a29233, 0x1426cdcb0, 0x142409b50, 0x1425ee820, 0x141834d18> as *const ()), // sub_1400cacb0
    Replacement::draft(0x1400cad10, "unknown_1400cad10", tpl_23::<0x143a29278, 0x143a2927b, 0x1426cdcb0, 0x142409b50, 0x1425ee830, 0x141834d18> as *const ()), // sub_1400cad10
    Replacement::draft(0x1400caf00, "unknown_1400caf00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384a9c8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ee870, 0x141834d18> as *const ()), // sub_1400caf00
    Replacement::draft(0x1400ccfb0, "unknown_1400ccfb0", tpl_23::<0x143a292e0, 0x143a292e3, 0x1426cdcb0, 0x142409b50, 0x1425ee9a0, 0x141834d18> as *const ()), // sub_1400ccfb0
    Replacement::draft(0x1400cd010, "unknown_1400cd010", tpl_23::<0x143a29328, 0x143a2932b, 0x1426cdcb0, 0x142409b50, 0x1425ee9b0, 0x141834d18> as *const ()), // sub_1400cd010
    Replacement::draft(0x1400cd200, "unknown_1400cd200", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384aad0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ee9f0, 0x141834d18> as *const ()), // sub_1400cd200
    Replacement::draft(0x1400cd8d0, "unknown_1400cd8d0", tpl_23::<0x143a29390, 0x143a29393, 0x1426cdcb0, 0x142409b50, 0x1425eea80, 0x141834d18> as *const ()), // sub_1400cd8d0
    Replacement::draft(0x1400cd930, "unknown_1400cd930", tpl_23::<0x143a293d8, 0x143a293db, 0x1426cdcb0, 0x142409b50, 0x1425eea90, 0x141834d18> as *const ()), // sub_1400cd930
    Replacement::draft(0x1400cdb20, "unknown_1400cdb20", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384ab60, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eead0, 0x141834d18> as *const ()), // sub_1400cdb20
    Replacement::draft(0x1400d01a0, "unknown_1400d01a0", tpl_23::<0x143a29440, 0x143a29443, 0x1426cdcb0, 0x142409b50, 0x1425eed80, 0x141834d18> as *const ()), // sub_1400d01a0
    Replacement::draft(0x1400d0200, "unknown_1400d0200", tpl_23::<0x143a29488, 0x143a2948b, 0x1426cdcb0, 0x142409b50, 0x1425eed90, 0x141834d18> as *const ()), // sub_1400d0200
    Replacement::draft(0x1400d03f0, "unknown_1400d03f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384ac48, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eedd0, 0x141834d18> as *const ()), // sub_1400d03f0
    Replacement::draft(0x1400d0b00, "unknown_1400d0b00", tpl_23::<0x143a294f0, 0x143a294f3, 0x1426cdcb0, 0x142409b50, 0x1425eeed0, 0x141834d18> as *const ()), // sub_1400d0b00
    Replacement::draft(0x1400d0b60, "unknown_1400d0b60", tpl_23::<0x143a29538, 0x143a2953b, 0x1426cdcb0, 0x142409b50, 0x1425eeee0, 0x141834d18> as *const ()), // sub_1400d0b60
    Replacement::draft(0x1400d0d50, "unknown_1400d0d50", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384acf0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425eef20, 0x141834d18> as *const ()), // sub_1400d0d50
    Replacement::draft(0x1400d1150, "unknown_1400d1150", fn_1400d1150 as *const ()), // sub_1400d1150
    Replacement::draft(0x1400d15c0, "unknown_1400d15c0", tpl_102::<0x143175084, 0x143174ff8, 0x143a295a0, 0x1403f9a70, 0x1425eefc0, 0x141834d18> as *const ()), // sub_1400d15c0
    Replacement::draft(0x1400d1610, "unknown_1400d1610", tpl_23::<0x143a29620, 0x143a29623, 0x1426cdcb0, 0x142409b50, 0x1425eeff0, 0x141834d18> as *const ()), // sub_1400d1610
    Replacement::draft(0x1400d1670, "unknown_1400d1670", tpl_23::<0x143a29668, 0x143a2966b, 0x1426cdcb0, 0x142409b50, 0x1425ef000, 0x141834d18> as *const ()), // sub_1400d1670
    Replacement::draft(0x1400d1860, "unknown_1400d1860", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384ad80, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ef040, 0x141834d18> as *const ()), // sub_1400d1860
    Replacement::draft(0x1400d1f30, "unknown_1400d1f30", tpl_61::<0x143a297e0, 0x143a297e3, 0x143091fd8, 0x142409b50, 0x1425ef0b0, 0x141834d18> as *const ()), // sub_1400d1f30
    Replacement::draft(0x1400d1f90, "unknown_1400d1f90", tpl_62::<0x143a297e3, 0x143a297e0, 0x143a297e8, 0x143a297e4, 0x143a296d0> as *const ()), // sub_1400d1f90
    Replacement::draft(0x1400d2020, "unknown_1400d2020", tpl_63::<0x143a29790, 0x143a29793, 0x143091ff0, 0x142409b50, 0x1425ef0c0, 0x141834d18> as *const ()), // sub_1400d2020
    Replacement::draft(0x1400d2080, "unknown_1400d2080", tpl_64::<0x143a29740, 0x143a29743, 0x143091fa0, 0x142409b50, 0x1425ef0d0, 0x141834d18> as *const ()), // sub_1400d2080
    Replacement::draft(0x1400d20e0, "unknown_1400d20e0", tpl_65::<0x143a296e0, 0x143a296e3, 0x143091fc0, 0x142409b50, 0x1425ef0e0, 0x141834d18> as *const ()), // sub_1400d20e0
    Replacement::draft(0x1400d2140, "unknown_1400d2140", tpl_62::<0x143a29793, 0x143a29790, 0x143a29798, 0x143a29794, 0x143a29788> as *const ()), // sub_1400d2140
    Replacement::draft(0x1400d21d0, "unknown_1400d21d0", tpl_62::<0x143a29743, 0x143a29740, 0x143a29748, 0x143a29744, 0x143a29728> as *const ()), // sub_1400d21d0
    Replacement::draft(0x1400d2260, "unknown_1400d2260", tpl_62::<0x143a296e3, 0x143a296e0, 0x143a296e8, 0x143a296e4, 0x143a29730> as *const ()), // sub_1400d2260
    Replacement::draft(0x1400d2c50, "unknown_1400d2c50", tpl_23::<0x143a29828, 0x143a2982b, 0x1426cdcb0, 0x142409b50, 0x1425ef130, 0x141834d18> as *const ()), // sub_1400d2c50
    Replacement::draft(0x1400d2cb0, "unknown_1400d2cb0", tpl_23::<0x143a29870, 0x143a29873, 0x1426cdcb0, 0x142409b50, 0x1425ef140, 0x141834d18> as *const ()), // sub_1400d2cb0
    Replacement::draft(0x1400d2ea0, "unknown_1400d2ea0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384ae10, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ef180, 0x141834d18> as *const ()), // sub_1400d2ea0
    Replacement::draft(0x1400d34c0, "unknown_1400d34c0", fn_1400d34c0 as *const ()), // sub_1400d34c0
    Replacement::draft(0x1400d3690, "unknown_1400d3690", fn_1400d3690 as *const ()), // sub_1400d3690
    Replacement::draft(0x1400d4510, "unknown_1400d4510", tpl_23::<0x143a29950, 0x143a29953, 0x1426cdcb0, 0x142409b50, 0x1425ef580, 0x141834d18> as *const ()), // sub_1400d4510
    Replacement::draft(0x1400d4570, "unknown_1400d4570", tpl_23::<0x143a29998, 0x143a2999b, 0x1426cdcb0, 0x142409b50, 0x1425ef590, 0x141834d18> as *const ()), // sub_1400d4570
    Replacement::draft(0x1400d4760, "unknown_1400d4760", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384af50, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ef5d0, 0x141834d18> as *const ()), // sub_1400d4760
    Replacement::draft(0x1400d5a00, "unknown_1400d5a00", tpl_23::<0x143a29be8, 0x143a29beb, 0x1426cdcb0, 0x142409b50, 0x1425ef660, 0x141834d18> as *const ()), // sub_1400d5a00
    Replacement::draft(0x1400d5a60, "unknown_1400d5a60", tpl_23::<0x143a29c30, 0x143a29c33, 0x1426cdcb0, 0x142409b50, 0x1425ef670, 0x141834d18> as *const ()), // sub_1400d5a60
    Replacement::draft(0x1400d5c50, "unknown_1400d5c50", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384aff0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425ef6b0, 0x141834d18> as *const ()), // sub_1400d5c50
    Replacement::draft(0x1400d6050, "unknown_1400d6050", tpl_33::<0x143428dc0, 0x143a29d10> as *const ()), // sub_1400d6050
    Replacement::draft(0x1400d6070, "unknown_1400d6070", tpl_33::<0x143428da0, 0x143a29cf0> as *const ()), // sub_1400d6070
    Replacement::draft_throws(0x1400d64a0, "unknown_1400d64a0", fn_1400d64a0 as *const (), Throws::Discard), // sub_1400d64a0
    Replacement::draft(0x1400d91f0, "unknown_1400d91f0", tpl_23::<0x143a29d40, 0x143a29d43, 0x1426cdcb0, 0x142409b50, 0x1425f0040, 0x141834d18> as *const ()), // sub_1400d91f0
    Replacement::draft(0x1400d9250, "unknown_1400d9250", tpl_23::<0x143a29d88, 0x143a29d8b, 0x1426cdcb0, 0x142409b50, 0x1425f0050, 0x141834d18> as *const ()), // sub_1400d9250
    Replacement::draft(0x1400d9440, "unknown_1400d9440", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384b220, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f0090, 0x141834d18> as *const ()), // sub_1400d9440
    Replacement::draft(0x1400d9840, "unknown_1400d9840", tpl_33::<0x143428dc0, 0x143a29e10> as *const ()), // sub_1400d9840
    Replacement::draft(0x1400d9860, "unknown_1400d9860", tpl_33::<0x143428da0, 0x143a29df0> as *const ()), // sub_1400d9860
    Replacement::draft(0x1400da640, "unknown_1400da640", tpl_61::<0x143a2a000, 0x143a2a003, 0x143091fd8, 0x142409b50, 0x1425f06b0, 0x141834d18> as *const ()), // sub_1400da640
    Replacement::draft(0x1400da6a0, "unknown_1400da6a0", tpl_62::<0x143a2a003, 0x143a2a000, 0x143a2a008, 0x143a2a004, 0x143a29ea0> as *const ()), // sub_1400da6a0
    Replacement::draft(0x1400da730, "unknown_1400da730", tpl_63::<0x143a29fb0, 0x143a29fb3, 0x143091ff0, 0x142409b50, 0x1425f06c0, 0x141834d18> as *const ()), // sub_1400da730
    Replacement::draft(0x1400da790, "unknown_1400da790", tpl_64::<0x143a29f10, 0x143a29f13, 0x143091fa0, 0x142409b50, 0x1425f06d0, 0x141834d18> as *const ()), // sub_1400da790
    Replacement::draft(0x1400da7f0, "unknown_1400da7f0", tpl_65::<0x143a29eb0, 0x143a29eb3, 0x143091fc0, 0x142409b50, 0x1425f06e0, 0x141834d18> as *const ()), // sub_1400da7f0
    Replacement::draft(0x1400da850, "unknown_1400da850", tpl_62::<0x143a29fb3, 0x143a29fb0, 0x143a29fb8, 0x143a29fb4, 0x143a29fa0> as *const ()), // sub_1400da850
    Replacement::draft(0x1400da8e0, "unknown_1400da8e0", tpl_62::<0x143a29f13, 0x143a29f10, 0x143a29f18, 0x143a29f14, 0x143a29ef8> as *const ()), // sub_1400da8e0
    Replacement::draft(0x1400da970, "unknown_1400da970", tpl_62::<0x143a29eb3, 0x143a29eb0, 0x143a29eb8, 0x143a29eb4, 0x143a29f00> as *const ()), // sub_1400da970
    Replacement::draft(0x1400daa00, "unknown_1400daa00", tpl_23::<0x143a29e30, 0x143a29e33, 0x1426cdcb0, 0x142409b50, 0x1425f06f0, 0x141834d18> as *const ()), // sub_1400daa00
    Replacement::draft(0x1400daa60, "unknown_1400daa60", tpl_23::<0x143a29e78, 0x143a29e7b, 0x1426cdcb0, 0x142409b50, 0x1425f0700, 0x141834d18> as *const ()), // sub_1400daa60
    Replacement::draft(0x1400dac50, "unknown_1400dac50", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384b460, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f0740, 0x141834d18> as *const ()), // sub_1400dac50
    Replacement::draft(0x1400dc210, "unknown_1400dc210", tpl_23::<0x143a2a048, 0x143a2a04b, 0x1426cdcb0, 0x142409b50, 0x1425f08c0, 0x141834d18> as *const ()), // sub_1400dc210
    Replacement::draft(0x1400dc270, "unknown_1400dc270", tpl_23::<0x143a2a090, 0x143a2a093, 0x1426cdcb0, 0x142409b50, 0x1425f08d0, 0x141834d18> as *const ()), // sub_1400dc270
    Replacement::draft(0x1400dc340, "unknown_1400dc340", tpl_23::<0x143a2a0b8, 0x143a2a0bb, 0x1426cdcb0, 0x142409b50, 0x1425f08f0, 0x141834d18> as *const ()), // sub_1400dc340
    Replacement::draft(0x1400dc3a0, "unknown_1400dc3a0", tpl_23::<0x143a2a100, 0x143a2a103, 0x1426cdcb0, 0x142409b50, 0x1425f0900, 0x141834d18> as *const ()), // sub_1400dc3a0
    Replacement::draft(0x1400dc590, "unknown_1400dc590", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384b5a8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f0940, 0x141834d18> as *const ()), // sub_1400dc590
    Replacement::draft(0x1400dde80, "unknown_1400dde80", tpl_23::<0x143a2a378, 0x143a2a37b, 0x1426cdcb0, 0x142409b50, 0x1425f0fc0, 0x141834d18> as *const ()), // sub_1400dde80
    Replacement::draft(0x1400ddee0, "unknown_1400ddee0", tpl_23::<0x143a2a3c0, 0x143a2a3c3, 0x1426cdcb0, 0x142409b50, 0x1425f0fd0, 0x141834d18> as *const ()), // sub_1400ddee0
    Replacement::draft(0x1400de0d0, "unknown_1400de0d0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384b760, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f1010, 0x141834d18> as *const ()), // sub_1400de0d0
    Replacement::draft(0x1400de460, "unknown_1400de460", tpl_34::<0x143a2a528, 0x143a2a52b, 0x143078668, 0x142409b50, 0x1425f1050, 0x141834d18> as *const ()), // sub_1400de460
    Replacement::draft(0x1400de4c0, "unknown_1400de4c0", tpl_23::<0x143a2aaf0, 0x143a2aaf3, 0x1426cdcb0, 0x142409b50, 0x1425f1060, 0x141834d18> as *const ()), // sub_1400de4c0
    Replacement::draft(0x1400de520, "unknown_1400de520", tpl_35::<0x143a2a820, 0x143a2a823, 0x143078678, 0x142409b50, 0x1425f1070, 0x141834d18> as *const ()), // sub_1400de520
    Replacement::draft(0x1400de580, "unknown_1400de580", tpl_36::<0x143a2aa78, 0x143a2aa7b, 0x143078680, 0x142409b50, 0x1425f1080, 0x141834d18> as *const ()), // sub_1400de580
    Replacement::draft(0x1400de5e0, "unknown_1400de5e0", tpl_37::<0x143a2a708, 0x143a2a70b, 0x143078690, 0x142409b50, 0x1425f1090, 0x141834d18> as *const ()), // sub_1400de5e0
    Replacement::draft(0x1400de640, "unknown_1400de640", tpl_38::<0x143a2a5c8, 0x143a2a5cb, 0x1430786a0, 0x142409b50, 0x1425f10a0, 0x141834d18> as *const ()), // sub_1400de640
    Replacement::draft(0x1400de6a0, "unknown_1400de6a0", tpl_39::<0x143a2ab18, 0x143a2ab1b, 0x1430786b0, 0x142409b50, 0x1425f10b0, 0x141834d18> as *const ()), // sub_1400de6a0
    Replacement::draft(0x1400de700, "unknown_1400de700", tpl_37::<0x143a2a4d8, 0x143a2a4db, 0x1430786b8, 0x142409b50, 0x1425f10c0, 0x141834d18> as *const ()), // sub_1400de700
    Replacement::draft(0x1400de760, "unknown_1400de760", tpl_40::<0x143a2ac58, 0x143a2ac5b, 0x1430786c4, 0x142409b50, 0x1425f10d0, 0x141834d18> as *const ()), // sub_1400de760
    Replacement::draft(0x1400de7c0, "unknown_1400de7c0", tpl_41::<0x143a2a690, 0x143a2a693, 0x1430786d0, 0x142409b50, 0x1425f10e0, 0x141834d18> as *const ()), // sub_1400de7c0
    Replacement::draft(0x1400de820, "unknown_1400de820", tpl_42::<0x143a2ab68, 0x143a2ab6b, 0x1430786e0, 0x142409b50, 0x1425f10f0, 0x141834d18> as *const ()), // sub_1400de820
    Replacement::draft(0x1400de880, "unknown_1400de880", tpl_34::<0x143a2abb8, 0x143a2abbb, 0x1430786f8, 0x142409b50, 0x1425f1100, 0x141834d18> as *const ()), // sub_1400de880
    Replacement::draft(0x1400de8e0, "unknown_1400de8e0", tpl_34::<0x143a2a500, 0x143a2a503, 0x143078708, 0x142409b50, 0x1425f1110, 0x141834d18> as *const ()), // sub_1400de8e0
    Replacement::draft(0x1400de940, "unknown_1400de940", tpl_41::<0x143a2a5f0, 0x143a2a5f3, 0x143078718, 0x142409b50, 0x1425f1120, 0x141834d18> as *const ()), // sub_1400de940
    Replacement::draft(0x1400de9a0, "unknown_1400de9a0", tpl_43::<0x143a2a730, 0x143a2a733, 0x143078728, 0x142409b50, 0x1425f1130, 0x141834d18> as *const ()), // sub_1400de9a0
    Replacement::draft(0x1400dea00, "unknown_1400dea00", tpl_44::<0x143a2a1e8, 0x143a2a1eb, 0x143078748, 0x142409b50, 0x1425f1140, 0x141834d18> as *const ()), // sub_1400dea00
    Replacement::draft(0x1400dea60, "unknown_1400dea60", tpl_39::<0x143a2a6b8, 0x143a2a6bb, 0x143078760, 0x142409b50, 0x1425f1150, 0x141834d18> as *const ()), // sub_1400dea60
    Replacement::draft(0x1400deac0, "unknown_1400deac0", tpl_45::<0x143a2a5a0, 0x143a2a5a3, 0x143078768, 0x142409b50, 0x1425f1160, 0x141834d18> as *const ()), // sub_1400deac0
    Replacement::draft(0x1400deb20, "unknown_1400deb20", tpl_41::<0x143a2aa28, 0x143a2aa2b, 0x143078778, 0x142409b50, 0x1425f1170, 0x141834d18> as *const ()), // sub_1400deb20
    Replacement::draft(0x1400deb80, "unknown_1400deb80", tpl_37::<0x143a2a210, 0x143a2a213, 0x143078788, 0x142409b50, 0x1425f1180, 0x141834d18> as *const ()), // sub_1400deb80
    Replacement::draft(0x1400debe0, "unknown_1400debe0", tpl_46::<0x143a2a8c0, 0x143a2a8c3, 0x143078798, 0x142409b50, 0x1425f1190, 0x141834d18> as *const ()), // sub_1400debe0
    Replacement::draft(0x1400dec40, "unknown_1400dec40", tpl_47::<0x143a2a438, 0x143a2a43b, 0x1430787b0, 0x142409b50, 0x1425f11a0, 0x141834d18> as *const ()), // sub_1400dec40
    Replacement::draft(0x1400deca0, "unknown_1400deca0", tpl_38::<0x143a2a960, 0x143a2a963, 0x1430787d0, 0x142409b50, 0x1425f11b0, 0x141834d18> as *const ()), // sub_1400deca0
    Replacement::draft(0x1400ded00, "unknown_1400ded00", tpl_48::<0x143a2ab40, 0x143a2ab43, 0x1430787e0, 0x142409b50, 0x1425f11c0, 0x141834d18> as *const ()), // sub_1400ded00
    Replacement::draft(0x1400ded60, "unknown_1400ded60", tpl_49::<0x143a2a7f8, 0x143a2a7fb, 0x1430787f8, 0x142409b50, 0x1425f11d0, 0x141834d18> as *const ()), // sub_1400ded60
    Replacement::draft(0x1400dedc0, "unknown_1400dedc0", tpl_44::<0x143a2aac8, 0x143a2aacb, 0x143078808, 0x142409b50, 0x1425f11e0, 0x141834d18> as *const ()), // sub_1400dedc0
    Replacement::draft(0x1400dee20, "unknown_1400dee20", tpl_42::<0x143a2a1c0, 0x143a2a1c3, 0x143078820, 0x142409b50, 0x1425f11f0, 0x141834d18> as *const ()), // sub_1400dee20
    Replacement::draft(0x1400dee80, "unknown_1400dee80", tpl_50::<0x143a2a9d8, 0x143a2a9db, 0x143078838, 0x142409b50, 0x1425f1200, 0x141834d18> as *const ()), // sub_1400dee80
    Replacement::draft(0x1400deee0, "unknown_1400deee0", tpl_52::<0x143a2a350, 0x143a2a353, 0x143a2ac58, 0x143a2ac5c, 0x143a2ac5b, 0x143a2ac60, 0x142409b50, 0x1425f1210, 0x141834d18> as *const ()), // sub_1400deee0
    Replacement::draft(0x1400defa0, "unknown_1400defa0", tpl_38::<0x143a2a848, 0x143a2a84b, 0x143078950, 0x142409b50, 0x1425f1220, 0x141834d18> as *const ()), // sub_1400defa0
    Replacement::draft(0x1400df000, "unknown_1400df000", tpl_53::<0x143a2a410, 0x143a2a413, 0x143078960, 0x142409b50, 0x1425f1230, 0x141834d18> as *const ()), // sub_1400df000
    Replacement::draft(0x1400df060, "unknown_1400df060", tpl_45::<0x143a2a2d8, 0x143a2a2db, 0x143078980, 0x142409b50, 0x1425f1240, 0x141834d18> as *const ()), // sub_1400df060
    Replacement::draft(0x1400df0c0, "unknown_1400df0c0", tpl_35::<0x143a2a328, 0x143a2a32b, 0x14307898c, 0x142409b50, 0x1425f1250, 0x141834d18> as *const ()), // sub_1400df0c0
    Replacement::draft(0x1400df120, "unknown_1400df120", tpl_42::<0x143a2ab90, 0x143a2ab93, 0x1430786e0, 0x142409b50, 0x1425f1260, 0x141834d18> as *const ()), // sub_1400df120
    Replacement::draft(0x1400df180, "unknown_1400df180", tpl_38::<0x143a2aaa0, 0x143a2aaa3, 0x143078998, 0x142409b50, 0x1425f1270, 0x141834d18> as *const ()), // sub_1400df180
    Replacement::draft(0x1400df1e0, "unknown_1400df1e0", tpl_38::<0x143a2a640, 0x143a2a643, 0x1430789a8, 0x142409b50, 0x1425f1280, 0x141834d18> as *const ()), // sub_1400df1e0
    Replacement::draft(0x1400df240, "unknown_1400df240", tpl_41::<0x143a2a758, 0x143a2a75b, 0x1430789b8, 0x142409b50, 0x1425f1290, 0x141834d18> as *const ()), // sub_1400df240
    Replacement::draft(0x1400df2a0, "unknown_1400df2a0", tpl_34::<0x143a2abe0, 0x143a2abe3, 0x1430789c8, 0x142409b50, 0x1425f12a0, 0x141834d18> as *const ()), // sub_1400df2a0
    Replacement::draft(0x1400df300, "unknown_1400df300", tpl_39::<0x143a2a300, 0x143a2a303, 0x1430789d8, 0x142409b50, 0x1425f12b0, 0x141834d18> as *const ()), // sub_1400df300
    Replacement::draft(0x1400df360, "unknown_1400df360", tpl_41::<0x143a2a618, 0x143a2a61b, 0x1430789e0, 0x142409b50, 0x1425f12c0, 0x141834d18> as *const ()), // sub_1400df360
    Replacement::draft(0x1400df3c0, "unknown_1400df3c0", tpl_34::<0x143a2a870, 0x143a2a873, 0x1430789f0, 0x142409b50, 0x1425f12d0, 0x141834d18> as *const ()), // sub_1400df3c0
    Replacement::draft(0x1400df420, "unknown_1400df420", tpl_48::<0x143a2a550, 0x143a2a553, 0x143078a00, 0x142409b50, 0x1425f12e0, 0x141834d18> as *const ()), // sub_1400df420
    Replacement::draft(0x1400df480, "unknown_1400df480", tpl_46::<0x143a2a988, 0x143a2a98b, 0x143078a18, 0x142409b50, 0x1425f12f0, 0x141834d18> as *const ()), // sub_1400df480
    Replacement::draft(0x1400df4e0, "unknown_1400df4e0", tpl_47::<0x143a2a8e8, 0x143a2a8eb, 0x143078a30, 0x142409b50, 0x1425f1300, 0x141834d18> as *const ()), // sub_1400df4e0
    Replacement::draft(0x1400df540, "unknown_1400df540", tpl_37::<0x143a2a488, 0x143a2a48b, 0x143078a50, 0x142409b50, 0x1425f1310, 0x141834d18> as *const ()), // sub_1400df540
    Replacement::draft(0x1400df5a0, "unknown_1400df5a0", tpl_45::<0x143a2a6e0, 0x143a2a6e3, 0x143078a60, 0x142409b50, 0x1425f1320, 0x141834d18> as *const ()), // sub_1400df5a0
    Replacement::draft(0x1400df600, "unknown_1400df600", tpl_54::<0x143a2a7a8, 0x143a2a7ab, 0x143078a70, 0x142409b50, 0x1425f1330, 0x141834d18> as *const ()), // sub_1400df600
    Replacement::draft(0x1400df660, "unknown_1400df660", tpl_40::<0x143a2a460, 0x143a2a463, 0x143078a88, 0x142409b50, 0x1425f1340, 0x141834d18> as *const ()), // sub_1400df660
    Replacement::draft(0x1400df6c0, "unknown_1400df6c0", tpl_50::<0x143a2a4b0, 0x143a2a4b3, 0x143078a90, 0x142409b50, 0x1425f1350, 0x141834d18> as *const ()), // sub_1400df6c0
    Replacement::draft(0x1400df720, "unknown_1400df720", tpl_41::<0x143a2a668, 0x143a2a66b, 0x143078aa8, 0x142409b50, 0x1425f1360, 0x141834d18> as *const ()), // sub_1400df720
    Replacement::draft(0x1400df780, "unknown_1400df780", tpl_55::<0x143a2a910, 0x143a2a913, 0x143078ab8, 0x142409b50, 0x1425f1370, 0x141834d18> as *const ()), // sub_1400df780
    Replacement::draft(0x1400df7e0, "unknown_1400df7e0", tpl_41::<0x143a2a9b0, 0x143a2a9b3, 0x143078ac8, 0x142409b50, 0x1425f1380, 0x141834d18> as *const ()), // sub_1400df7e0
    Replacement::draft(0x1400df840, "unknown_1400df840", tpl_49::<0x143a2aa50, 0x143a2aa53, 0x143078ad8, 0x142409b50, 0x1425f1390, 0x141834d18> as *const ()), // sub_1400df840
    Replacement::draft(0x1400df8a0, "unknown_1400df8a0", tpl_40::<0x143a2a2b0, 0x143a2a2b3, 0x143078ae4, 0x142409b50, 0x1425f13a0, 0x141834d18> as *const ()), // sub_1400df8a0
    Replacement::draft(0x1400df900, "unknown_1400df900", tpl_44::<0x143a2a3e8, 0x143a2a3eb, 0x143078af0, 0x142409b50, 0x1425f13b0, 0x141834d18> as *const ()), // sub_1400df900
    Replacement::draft(0x1400df960, "unknown_1400df960", tpl_46::<0x143a2a288, 0x143a2a28b, 0x143078b08, 0x142409b50, 0x1425f13c0, 0x141834d18> as *const ()), // sub_1400df960
    Replacement::draft(0x1400df9c0, "unknown_1400df9c0", tpl_55::<0x143a2a938, 0x143a2a93b, 0x143078b20, 0x142409b50, 0x1425f13d0, 0x141834d18> as *const ()), // sub_1400df9c0
    Replacement::draft(0x1400dfa20, "unknown_1400dfa20", tpl_46::<0x143a2a7d0, 0x143a2a7d3, 0x143078b30, 0x142409b50, 0x1425f13e0, 0x141834d18> as *const ()), // sub_1400dfa20
    Replacement::draft(0x1400dfa80, "unknown_1400dfa80", tpl_50::<0x143a2a578, 0x143a2a57b, 0x143078b48, 0x142409b50, 0x1425f13f0, 0x141834d18> as *const ()), // sub_1400dfa80
    Replacement::draft(0x1400dfae0, "unknown_1400dfae0", tpl_42::<0x143a2a238, 0x143a2a23b, 0x143078b60, 0x142409b50, 0x1425f1400, 0x141834d18> as *const ()), // sub_1400dfae0
    Replacement::draft(0x1400dfb40, "unknown_1400dfb40", tpl_38::<0x143a2a898, 0x143a2a89b, 0x143078b78, 0x142409b50, 0x1425f1410, 0x141834d18> as *const ()), // sub_1400dfb40
    Replacement::draft(0x1400dfba0, "unknown_1400dfba0", tpl_34::<0x143a2ac30, 0x143a2ac33, 0x143078b88, 0x142409b50, 0x1425f1420, 0x141834d18> as *const ()), // sub_1400dfba0
    Replacement::draft(0x1400dfc00, "unknown_1400dfc00", tpl_41::<0x143a2aa00, 0x143a2aa03, 0x143078b98, 0x142409b50, 0x1425f1430, 0x141834d18> as *const ()), // sub_1400dfc00
    Replacement::draft(0x1400dfc60, "unknown_1400dfc60", tpl_36::<0x143a2a780, 0x143a2a783, 0x143078ba8, 0x142409b50, 0x1425f1440, 0x141834d18> as *const ()), // sub_1400dfc60
    Replacement::draft(0x1400dfcc0, "unknown_1400dfcc0", tpl_34::<0x143a2a260, 0x143a2a263, 0x143078bb8, 0x142409b50, 0x1425f1450, 0x141834d18> as *const ()), // sub_1400dfcc0
    Replacement::draft(0x1400dfda0, "unknown_1400dfda0", tpl_44::<0x143a2ac08, 0x143a2ac0b, 0x143078af0, 0x142409b50, 0x1425f1580, 0x141834d18> as *const ()), // sub_1400dfda0
    Replacement::draft(0x1400e0530, "unknown_1400e0530", tpl_61::<0x143a2ae30, 0x143a2ae33, 0x143091fd8, 0x142409b50, 0x1425f15a0, 0x141834d18> as *const ()), // sub_1400e0530
    Replacement::draft(0x1400e0590, "unknown_1400e0590", tpl_62::<0x143a2ae33, 0x143a2ae30, 0x143a2ae38, 0x143a2ae34, 0x143a2ac90> as *const ()), // sub_1400e0590
    Replacement::draft(0x1400e0620, "unknown_1400e0620", tpl_63::<0x143a2ade0, 0x143a2ade3, 0x143091ff0, 0x142409b50, 0x1425f15b0, 0x141834d18> as *const ()), // sub_1400e0620
    Replacement::draft(0x1400e0680, "unknown_1400e0680", tpl_64::<0x143a2ad60, 0x143a2ad63, 0x143091fa0, 0x142409b50, 0x1425f15c0, 0x141834d18> as *const ()), // sub_1400e0680
    Replacement::draft(0x1400e06e0, "unknown_1400e06e0", tpl_65::<0x143a2aca0, 0x143a2aca3, 0x143091fc0, 0x142409b50, 0x1425f15d0, 0x141834d18> as *const ()), // sub_1400e06e0
    Replacement::draft(0x1400e0740, "unknown_1400e0740", tpl_62::<0x143a2ade3, 0x143a2ade0, 0x143a2ade8, 0x143a2ade4, 0x143a2add0> as *const ()), // sub_1400e0740
    Replacement::draft(0x1400e07d0, "unknown_1400e07d0", tpl_62::<0x143a2ad63, 0x143a2ad60, 0x143a2ad68, 0x143a2ad64, 0x143a2ace8> as *const ()), // sub_1400e07d0
    Replacement::draft(0x1400e0860, "unknown_1400e0860", tpl_62::<0x143a2aca3, 0x143a2aca0, 0x143a2aca8, 0x143a2aca4, 0x143a2acf0> as *const ()), // sub_1400e0860
    Replacement::draft(0x1400e0b00, "unknown_1400e0b00", tpl_61::<0x143a2b090, 0x143a2b093, 0x143091fd8, 0x142409b50, 0x1425f15f0, 0x141834d18> as *const ()), // sub_1400e0b00
    Replacement::draft(0x1400e0b60, "unknown_1400e0b60", tpl_62::<0x143a2b093, 0x143a2b090, 0x143a2b098, 0x143a2b094, 0x143a2af30> as *const ()), // sub_1400e0b60
    Replacement::draft(0x1400e0bf0, "unknown_1400e0bf0", tpl_63::<0x143a2b040, 0x143a2b043, 0x143091ff0, 0x142409b50, 0x1425f1600, 0x141834d18> as *const ()), // sub_1400e0bf0
    Replacement::draft(0x1400e0c50, "unknown_1400e0c50", tpl_64::<0x143a2afa0, 0x143a2afa3, 0x143091fa0, 0x142409b50, 0x1425f1610, 0x141834d18> as *const ()), // sub_1400e0c50
    Replacement::draft(0x1400e0cb0, "unknown_1400e0cb0", tpl_65::<0x143a2af40, 0x143a2af43, 0x143091fc0, 0x142409b50, 0x1425f1620, 0x141834d18> as *const ()), // sub_1400e0cb0
    Replacement::draft(0x1400e0d10, "unknown_1400e0d10", tpl_62::<0x143a2b043, 0x143a2b040, 0x143a2b048, 0x143a2b044, 0x143a2b030> as *const ()), // sub_1400e0d10
    Replacement::draft(0x1400e0da0, "unknown_1400e0da0", tpl_62::<0x143a2afa3, 0x143a2afa0, 0x143a2afa8, 0x143a2afa4, 0x143a2af88> as *const ()), // sub_1400e0da0
    Replacement::draft(0x1400e0e30, "unknown_1400e0e30", tpl_62::<0x143a2af43, 0x143a2af40, 0x143a2af48, 0x143a2af44, 0x143a2af90> as *const ()), // sub_1400e0e30
    Replacement::draft(0x1400e0ec0, "unknown_1400e0ec0", tpl_23::<0x143a2ae78, 0x143a2ae7b, 0x1426cdcb0, 0x142409b50, 0x1425f1630, 0x141834d18> as *const ()), // sub_1400e0ec0
    Replacement::draft(0x1400e0f20, "unknown_1400e0f20", tpl_23::<0x143a2aec0, 0x143a2aec3, 0x1426cdcb0, 0x142409b50, 0x1425f1640, 0x141834d18> as *const ()), // sub_1400e0f20
    Replacement::draft(0x1400e1110, "unknown_1400e1110", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384b830, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f1680, 0x141834d18> as *const ()), // sub_1400e1110
    Replacement::draft(0x1400e15d0, "unknown_1400e15d0", tpl_100::<0x1431998f0, 0x14309f90c, 0x143a2aef0, 0x1431998e8, 0x14309f914, 0x1403f8780, 0x1425f18a0, 0x141834d18> as *const ()), // sub_1400e15d0
    Replacement::draft(0x1400e4170, "unknown_1400e4170", tpl_61::<0x143a2b2a0, 0x143a2b2a3, 0x143091fd8, 0x142409b50, 0x1425f1970, 0x141834d18> as *const ()), // sub_1400e4170
    Replacement::draft(0x1400e41d0, "unknown_1400e41d0", tpl_62::<0x143a2b2a3, 0x143a2b2a0, 0x143a2b2a8, 0x143a2b2a4, 0x143a2b190> as *const ()), // sub_1400e41d0
    Replacement::draft(0x1400e4260, "unknown_1400e4260", tpl_63::<0x143a2b250, 0x143a2b253, 0x143091ff0, 0x142409b50, 0x1425f1980, 0x141834d18> as *const ()), // sub_1400e4260
    Replacement::draft(0x1400e42c0, "unknown_1400e42c0", tpl_64::<0x143a2b200, 0x143a2b203, 0x143091fa0, 0x142409b50, 0x1425f1990, 0x141834d18> as *const ()), // sub_1400e42c0
    Replacement::draft(0x1400e4320, "unknown_1400e4320", tpl_65::<0x143a2b1a0, 0x143a2b1a3, 0x143091fc0, 0x142409b50, 0x1425f19a0, 0x141834d18> as *const ()), // sub_1400e4320
    Replacement::draft(0x1400e4380, "unknown_1400e4380", tpl_62::<0x143a2b253, 0x143a2b250, 0x143a2b258, 0x143a2b254, 0x143a2b248> as *const ()), // sub_1400e4380
    Replacement::draft(0x1400e4410, "unknown_1400e4410", tpl_62::<0x143a2b203, 0x143a2b200, 0x143a2b208, 0x143a2b204, 0x143a2b1e8> as *const ()), // sub_1400e4410
    Replacement::draft(0x1400e44a0, "unknown_1400e44a0", tpl_62::<0x143a2b1a3, 0x143a2b1a0, 0x143a2b1a8, 0x143a2b1a4, 0x143a2b1f0> as *const ()), // sub_1400e44a0
    Replacement::draft(0x1400e4530, "unknown_1400e4530", tpl_23::<0x143a2b0d8, 0x143a2b0db, 0x1426cdcb0, 0x142409b50, 0x1425f19b0, 0x141834d18> as *const ()), // sub_1400e4530
    Replacement::draft(0x1400e4590, "unknown_1400e4590", tpl_23::<0x143a2b120, 0x143a2b123, 0x1426cdcb0, 0x142409b50, 0x1425f19c0, 0x141834d18> as *const ()), // sub_1400e4590
    Replacement::draft(0x1400e4780, "unknown_1400e4780", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384b928, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f1a00, 0x141834d18> as *const ()), // sub_1400e4780
    Replacement::draft(0x1400e4bd0, "unknown_1400e4bd0", fn_1400e4bd0 as *const ()), // sub_1400e4bd0
    Replacement::draft(0x1400e5760, "unknown_1400e5760", tpl_61::<0x143a2b4b0, 0x143a2b4b3, 0x143091fd8, 0x142409b50, 0x1425f1c50, 0x141834d18> as *const ()), // sub_1400e5760
    Replacement::draft(0x1400e57c0, "unknown_1400e57c0", tpl_62::<0x143a2b4b3, 0x143a2b4b0, 0x143a2b4b8, 0x143a2b4b4, 0x143a2b358> as *const ()), // sub_1400e57c0
    Replacement::draft(0x1400e5850, "unknown_1400e5850", tpl_63::<0x143a2b460, 0x143a2b463, 0x143091ff0, 0x142409b50, 0x1425f1c60, 0x141834d18> as *const ()), // sub_1400e5850
    Replacement::draft(0x1400e58b0, "unknown_1400e58b0", tpl_64::<0x143a2b3c0, 0x143a2b3c3, 0x143091fa0, 0x142409b50, 0x1425f1c70, 0x141834d18> as *const ()), // sub_1400e58b0
    Replacement::draft(0x1400e5910, "unknown_1400e5910", tpl_65::<0x143a2b360, 0x143a2b363, 0x143091fc0, 0x142409b50, 0x1425f1c80, 0x141834d18> as *const ()), // sub_1400e5910
    Replacement::draft(0x1400e5970, "unknown_1400e5970", tpl_62::<0x143a2b463, 0x143a2b460, 0x143a2b468, 0x143a2b464, 0x143a2b450> as *const ()), // sub_1400e5970
    Replacement::draft(0x1400e5a00, "unknown_1400e5a00", tpl_62::<0x143a2b3c3, 0x143a2b3c0, 0x143a2b3c8, 0x143a2b3c4, 0x143a2b3a8> as *const ()), // sub_1400e5a00
    Replacement::draft(0x1400e5a90, "unknown_1400e5a90", tpl_62::<0x143a2b363, 0x143a2b360, 0x143a2b368, 0x143a2b364, 0x143a2b3b0> as *const ()), // sub_1400e5a90
    Replacement::draft(0x1400e5b20, "unknown_1400e5b20", tpl_23::<0x143a2b2e8, 0x143a2b2eb, 0x1426cdcb0, 0x142409b50, 0x1425f1c90, 0x141834d18> as *const ()), // sub_1400e5b20
    Replacement::draft(0x1400e5b80, "unknown_1400e5b80", tpl_23::<0x143a2b330, 0x143a2b333, 0x1426cdcb0, 0x142409b50, 0x1425f1ca0, 0x141834d18> as *const ()), // sub_1400e5b80
    Replacement::draft(0x1400e5d70, "unknown_1400e5d70", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384ba10, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f1ce0, 0x141834d18> as *const ()), // sub_1400e5d70
    Replacement::draft(0x1400e6930, "unknown_1400e6930", tpl_23::<0x143a2b4f8, 0x143a2b4fb, 0x1426cdcb0, 0x142409b50, 0x1425f1e10, 0x141834d18> as *const ()), // sub_1400e6930
    Replacement::draft(0x1400e6990, "unknown_1400e6990", tpl_23::<0x143a2b540, 0x143a2b543, 0x1426cdcb0, 0x142409b50, 0x1425f1e20, 0x141834d18> as *const ()), // sub_1400e6990
    Replacement::draft(0x1400e6b80, "unknown_1400e6b80", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384baa0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f1e60, 0x141834d18> as *const ()), // sub_1400e6b80
    Replacement::draft(0x1400e6f80, "unknown_1400e6f80", tpl_127::<0x14384bb30, 0x143a2b630, 0x140548a40, 0x1425f1ed0, 0x141834d18> as *const ()), // sub_1400e6f80
    Replacement::draft(0x1400e7170, "unknown_1400e7170", fn_1400e7170 as *const ()), // sub_1400e7170
    Replacement::draft(0x1400e7620, "unknown_1400e7620", fn_1400e7620 as *const ()), // sub_1400e7620
    Replacement::draft(0x1400e7eb0, "unknown_1400e7eb0", tpl_61::<0x143a2b6b0, 0x143a2b6b3, 0x143091fd8, 0x142409b50, 0x1425f2230, 0x141834d18> as *const ()), // sub_1400e7eb0
    Replacement::draft(0x1400e7f10, "unknown_1400e7f10", tpl_62::<0x143a2b6b3, 0x143a2b6b0, 0x143a2b6b8, 0x143a2b6b4, 0x143a2b4a8> as *const ()), // sub_1400e7f10
    Replacement::draft(0x1400e87c0, "unknown_1400e87c0", tpl_61::<0x143a2b790, 0x143a2b793, 0x143091fd8, 0x142409b50, 0x1425f2260, 0x141834d18> as *const ()), // sub_1400e87c0
    Replacement::draft(0x1400e8820, "unknown_1400e8820", tpl_62::<0x143a2b793, 0x143a2b790, 0x143a2b798, 0x143a2b794, 0x143a2b568> as *const ()), // sub_1400e8820
    Replacement::draft(0x1400e96f0, "unknown_1400e96f0", tpl_23::<0x143a2b7d8, 0x143a2b7db, 0x1426cdcb0, 0x142409b50, 0x1425f25f0, 0x141834d18> as *const ()), // sub_1400e96f0
    Replacement::draft(0x1400e9750, "unknown_1400e9750", tpl_23::<0x143a2b820, 0x143a2b823, 0x1426cdcb0, 0x142409b50, 0x1425f2600, 0x141834d18> as *const ()), // sub_1400e9750
    Replacement::draft(0x1400e9940, "unknown_1400e9940", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384bc28, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f2640, 0x141834d18> as *const ()), // sub_1400e9940
    Replacement::draft(0x1400e9f20, "unknown_1400e9f20", tpl_23::<0x143a2b890, 0x143a2b893, 0x1426cdcb0, 0x142409b50, 0x1425f26b0, 0x141834d18> as *const ()), // sub_1400e9f20
    Replacement::draft(0x1400e9f80, "unknown_1400e9f80", tpl_23::<0x143a2b8d8, 0x143a2b8db, 0x1426cdcb0, 0x142409b50, 0x1425f26c0, 0x141834d18> as *const ()), // sub_1400e9f80
    Replacement::draft_throws(0x1400ea050, "unknown_1400ea050", fn_1400ea050 as *const (), Throws::Caught), // sub_1400ea050
    Replacement::draft_throws(0x1400ea160, "unknown_1400ea160", fn_1400ea160 as *const (), Throws::Caught), // sub_1400ea160
    Replacement::draft_throws(0x1400ea270, "unknown_1400ea270", fn_1400ea270 as *const (), Throws::Caught), // sub_1400ea270
    Replacement::draft(0x1400ea830, "unknown_1400ea830", tpl_23::<0x143a2bab8, 0x143a2babb, 0x1426cdcb0, 0x142409b50, 0x1425f2760, 0x141834d18> as *const ()), // sub_1400ea830
    Replacement::draft(0x1400ea890, "unknown_1400ea890", tpl_23::<0x143a2bb00, 0x143a2bb03, 0x1426cdcb0, 0x142409b50, 0x1425f2770, 0x141834d18> as *const ()), // sub_1400ea890
    Replacement::draft(0x1400eaa80, "unknown_1400eaa80", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384bf90, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f27b0, 0x141834d18> as *const ()), // sub_1400eaa80
    Replacement::draft(0x1400eae10, "unknown_1400eae10", tpl_34::<0x143a2bc68, 0x143a2bc6b, 0x143078668, 0x142409b50, 0x1425f27f0, 0x141834d18> as *const ()), // sub_1400eae10
    Replacement::draft(0x1400eae70, "unknown_1400eae70", tpl_23::<0x143a2c230, 0x143a2c233, 0x1426cdcb0, 0x142409b50, 0x1425f2800, 0x141834d18> as *const ()), // sub_1400eae70
    Replacement::draft(0x1400eaed0, "unknown_1400eaed0", tpl_35::<0x143a2bf60, 0x143a2bf63, 0x143078678, 0x142409b50, 0x1425f2810, 0x141834d18> as *const ()), // sub_1400eaed0
    Replacement::draft(0x1400eaf30, "unknown_1400eaf30", tpl_36::<0x143a2c1b8, 0x143a2c1bb, 0x143078680, 0x142409b50, 0x1425f2820, 0x141834d18> as *const ()), // sub_1400eaf30
    Replacement::draft(0x1400eaf90, "unknown_1400eaf90", tpl_37::<0x143a2be48, 0x143a2be4b, 0x143078690, 0x142409b50, 0x1425f2830, 0x141834d18> as *const ()), // sub_1400eaf90
    Replacement::draft(0x1400eaff0, "unknown_1400eaff0", tpl_38::<0x143a2bd08, 0x143a2bd0b, 0x1430786a0, 0x142409b50, 0x1425f2840, 0x141834d18> as *const ()), // sub_1400eaff0
    Replacement::draft(0x1400eb050, "unknown_1400eb050", tpl_39::<0x143a2c258, 0x143a2c25b, 0x1430786b0, 0x142409b50, 0x1425f2850, 0x141834d18> as *const ()), // sub_1400eb050
    Replacement::draft(0x1400eb0b0, "unknown_1400eb0b0", tpl_37::<0x143a2bc18, 0x143a2bc1b, 0x1430786b8, 0x142409b50, 0x1425f2860, 0x141834d18> as *const ()), // sub_1400eb0b0
    Replacement::draft(0x1400eb110, "unknown_1400eb110", tpl_40::<0x143a2c370, 0x143a2c373, 0x1430786c4, 0x142409b50, 0x1425f2870, 0x141834d18> as *const ()), // sub_1400eb110
    Replacement::draft(0x1400eb170, "unknown_1400eb170", tpl_41::<0x143a2bdd0, 0x143a2bdd3, 0x1430786d0, 0x142409b50, 0x1425f2880, 0x141834d18> as *const ()), // sub_1400eb170
    Replacement::draft(0x1400eb1d0, "unknown_1400eb1d0", tpl_42::<0x143a2c2a8, 0x143a2c2ab, 0x1430786e0, 0x142409b50, 0x1425f2890, 0x141834d18> as *const ()), // sub_1400eb1d0
    Replacement::draft(0x1400eb230, "unknown_1400eb230", tpl_34::<0x143a2c2f8, 0x143a2c2fb, 0x1430786f8, 0x142409b50, 0x1425f28a0, 0x141834d18> as *const ()), // sub_1400eb230
    Replacement::draft(0x1400eb290, "unknown_1400eb290", tpl_34::<0x143a2bc40, 0x143a2bc43, 0x143078708, 0x142409b50, 0x1425f28b0, 0x141834d18> as *const ()), // sub_1400eb290
    Replacement::draft(0x1400eb2f0, "unknown_1400eb2f0", tpl_41::<0x143a2bd30, 0x143a2bd33, 0x143078718, 0x142409b50, 0x1425f28c0, 0x141834d18> as *const ()), // sub_1400eb2f0
    Replacement::draft(0x1400eb350, "unknown_1400eb350", tpl_43::<0x143a2be70, 0x143a2be73, 0x143078728, 0x142409b50, 0x1425f28d0, 0x141834d18> as *const ()), // sub_1400eb350
    Replacement::draft(0x1400eb3b0, "unknown_1400eb3b0", tpl_44::<0x143a2b928, 0x143a2b92b, 0x143078748, 0x142409b50, 0x1425f28e0, 0x141834d18> as *const ()), // sub_1400eb3b0
    Replacement::draft(0x1400eb410, "unknown_1400eb410", tpl_39::<0x143a2bdf8, 0x143a2bdfb, 0x143078760, 0x142409b50, 0x1425f28f0, 0x141834d18> as *const ()), // sub_1400eb410
    Replacement::draft(0x1400eb470, "unknown_1400eb470", tpl_45::<0x143a2bce0, 0x143a2bce3, 0x143078768, 0x142409b50, 0x1425f2900, 0x141834d18> as *const ()), // sub_1400eb470
    Replacement::draft(0x1400eb4d0, "unknown_1400eb4d0", tpl_41::<0x143a2c168, 0x143a2c16b, 0x143078778, 0x142409b50, 0x1425f2910, 0x141834d18> as *const ()), // sub_1400eb4d0
    Replacement::draft(0x1400eb530, "unknown_1400eb530", tpl_37::<0x143a2b950, 0x143a2b953, 0x143078788, 0x142409b50, 0x1425f2920, 0x141834d18> as *const ()), // sub_1400eb530
    Replacement::draft(0x1400eb590, "unknown_1400eb590", tpl_46::<0x143a2c000, 0x143a2c003, 0x143078798, 0x142409b50, 0x1425f2930, 0x141834d18> as *const ()), // sub_1400eb590
    Replacement::draft(0x1400eb5f0, "unknown_1400eb5f0", tpl_47::<0x143a2bb78, 0x143a2bb7b, 0x1430787b0, 0x142409b50, 0x1425f2940, 0x141834d18> as *const ()), // sub_1400eb5f0
    Replacement::draft(0x1400eb650, "unknown_1400eb650", tpl_38::<0x143a2c0a0, 0x143a2c0a3, 0x1430787d0, 0x142409b50, 0x1425f2950, 0x141834d18> as *const ()), // sub_1400eb650
    Replacement::draft(0x1400eb6b0, "unknown_1400eb6b0", tpl_48::<0x143a2c280, 0x143a2c283, 0x1430787e0, 0x142409b50, 0x1425f2960, 0x141834d18> as *const ()), // sub_1400eb6b0
    Replacement::draft(0x1400eb710, "unknown_1400eb710", tpl_49::<0x143a2bf38, 0x143a2bf3b, 0x1430787f8, 0x142409b50, 0x1425f2970, 0x141834d18> as *const ()), // sub_1400eb710
    Replacement::draft(0x1400eb770, "unknown_1400eb770", tpl_44::<0x143a2c208, 0x143a2c20b, 0x143078808, 0x142409b50, 0x1425f2980, 0x141834d18> as *const ()), // sub_1400eb770
    Replacement::draft(0x1400eb7d0, "unknown_1400eb7d0", tpl_42::<0x143a2b900, 0x143a2b903, 0x143078820, 0x142409b50, 0x1425f2990, 0x141834d18> as *const ()), // sub_1400eb7d0
    Replacement::draft(0x1400eb830, "unknown_1400eb830", tpl_50::<0x143a2c118, 0x143a2c11b, 0x143078838, 0x142409b50, 0x1425f29a0, 0x141834d18> as *const ()), // sub_1400eb830
    Replacement::draft(0x1400eb890, "unknown_1400eb890", tpl_52::<0x143a2ba90, 0x143a2ba93, 0x143a2c370, 0x143a2c374, 0x143a2c373, 0x143a2c378, 0x142409b50, 0x1425f29b0, 0x141834d18> as *const ()), // sub_1400eb890
    Replacement::draft(0x1400eb950, "unknown_1400eb950", tpl_38::<0x143a2bf88, 0x143a2bf8b, 0x143078950, 0x142409b50, 0x1425f29c0, 0x141834d18> as *const ()), // sub_1400eb950
    Replacement::draft(0x1400eb9b0, "unknown_1400eb9b0", tpl_53::<0x143a2bb50, 0x143a2bb53, 0x143078960, 0x142409b50, 0x1425f29d0, 0x141834d18> as *const ()), // sub_1400eb9b0
    Replacement::draft(0x1400eba10, "unknown_1400eba10", tpl_45::<0x143a2ba18, 0x143a2ba1b, 0x143078980, 0x142409b50, 0x1425f29e0, 0x141834d18> as *const ()), // sub_1400eba10
    Replacement::draft(0x1400eba70, "unknown_1400eba70", tpl_35::<0x143a2ba68, 0x143a2ba6b, 0x14307898c, 0x142409b50, 0x1425f29f0, 0x141834d18> as *const ()), // sub_1400eba70
    Replacement::draft(0x1400ebad0, "unknown_1400ebad0", tpl_42::<0x143a2c2d0, 0x143a2c2d3, 0x1430786e0, 0x142409b50, 0x1425f2a00, 0x141834d18> as *const ()), // sub_1400ebad0
    Replacement::draft(0x1400ebb30, "unknown_1400ebb30", tpl_38::<0x143a2c1e0, 0x143a2c1e3, 0x143078998, 0x142409b50, 0x1425f2a10, 0x141834d18> as *const ()), // sub_1400ebb30
    Replacement::draft(0x1400ebb90, "unknown_1400ebb90", tpl_38::<0x143a2bd80, 0x143a2bd83, 0x1430789a8, 0x142409b50, 0x1425f2a20, 0x141834d18> as *const ()), // sub_1400ebb90
    Replacement::draft(0x1400ebbf0, "unknown_1400ebbf0", tpl_41::<0x143a2be98, 0x143a2be9b, 0x1430789b8, 0x142409b50, 0x1425f2a30, 0x141834d18> as *const ()), // sub_1400ebbf0
    Replacement::draft(0x1400ebc50, "unknown_1400ebc50", tpl_34::<0x143a2c320, 0x143a2c323, 0x1430789c8, 0x142409b50, 0x1425f2a40, 0x141834d18> as *const ()), // sub_1400ebc50
    Replacement::draft(0x1400ebcb0, "unknown_1400ebcb0", tpl_39::<0x143a2ba40, 0x143a2ba43, 0x1430789d8, 0x142409b50, 0x1425f2a50, 0x141834d18> as *const ()), // sub_1400ebcb0
    Replacement::draft(0x1400ebd10, "unknown_1400ebd10", tpl_41::<0x143a2bd58, 0x143a2bd5b, 0x1430789e0, 0x142409b50, 0x1425f2a60, 0x141834d18> as *const ()), // sub_1400ebd10
    Replacement::draft(0x1400ebd70, "unknown_1400ebd70", tpl_34::<0x143a2bfb0, 0x143a2bfb3, 0x1430789f0, 0x142409b50, 0x1425f2a70, 0x141834d18> as *const ()), // sub_1400ebd70
    Replacement::draft(0x1400ebdd0, "unknown_1400ebdd0", tpl_48::<0x143a2bc90, 0x143a2bc93, 0x143078a00, 0x142409b50, 0x1425f2a80, 0x141834d18> as *const ()), // sub_1400ebdd0
    Replacement::draft(0x1400ebe30, "unknown_1400ebe30", tpl_46::<0x143a2c0c8, 0x143a2c0cb, 0x143078a18, 0x142409b50, 0x1425f2a90, 0x141834d18> as *const ()), // sub_1400ebe30
    Replacement::draft(0x1400ebe90, "unknown_1400ebe90", tpl_47::<0x143a2c028, 0x143a2c02b, 0x143078a30, 0x142409b50, 0x1425f2aa0, 0x141834d18> as *const ()), // sub_1400ebe90
    Replacement::draft(0x1400ebef0, "unknown_1400ebef0", tpl_37::<0x143a2bbc8, 0x143a2bbcb, 0x143078a50, 0x142409b50, 0x1425f2ab0, 0x141834d18> as *const ()), // sub_1400ebef0
    Replacement::draft(0x1400ebf50, "unknown_1400ebf50", tpl_45::<0x143a2be20, 0x143a2be23, 0x143078a60, 0x142409b50, 0x1425f2ac0, 0x141834d18> as *const ()), // sub_1400ebf50
    Replacement::draft(0x1400ebfb0, "unknown_1400ebfb0", tpl_54::<0x143a2bee8, 0x143a2beeb, 0x143078a70, 0x142409b50, 0x1425f2ad0, 0x141834d18> as *const ()), // sub_1400ebfb0
    Replacement::draft(0x1400ec010, "unknown_1400ec010", tpl_40::<0x143a2bba0, 0x143a2bba3, 0x143078a88, 0x142409b50, 0x1425f2ae0, 0x141834d18> as *const ()), // sub_1400ec010
    Replacement::draft(0x1400ec070, "unknown_1400ec070", tpl_50::<0x143a2bbf0, 0x143a2bbf3, 0x143078a90, 0x142409b50, 0x1425f2af0, 0x141834d18> as *const ()), // sub_1400ec070
    Replacement::draft(0x1400ec0d0, "unknown_1400ec0d0", tpl_41::<0x143a2bda8, 0x143a2bdab, 0x143078aa8, 0x142409b50, 0x1425f2b00, 0x141834d18> as *const ()), // sub_1400ec0d0
    Replacement::draft(0x1400ec130, "unknown_1400ec130", tpl_55::<0x143a2c050, 0x143a2c053, 0x143078ab8, 0x142409b50, 0x1425f2b10, 0x141834d18> as *const ()), // sub_1400ec130
    Replacement::draft(0x1400ec190, "unknown_1400ec190", tpl_41::<0x143a2c0f0, 0x143a2c0f3, 0x143078ac8, 0x142409b50, 0x1425f2b20, 0x141834d18> as *const ()), // sub_1400ec190
    Replacement::draft(0x1400ec1f0, "unknown_1400ec1f0", tpl_49::<0x143a2c190, 0x143a2c193, 0x143078ad8, 0x142409b50, 0x1425f2b30, 0x141834d18> as *const ()), // sub_1400ec1f0
    Replacement::draft(0x1400ec250, "unknown_1400ec250", tpl_40::<0x143a2b9f0, 0x143a2b9f3, 0x143078ae4, 0x142409b50, 0x1425f2b40, 0x141834d18> as *const ()), // sub_1400ec250
    Replacement::draft(0x1400ec2b0, "unknown_1400ec2b0", tpl_44::<0x143a2bb28, 0x143a2bb2b, 0x143078af0, 0x142409b50, 0x1425f2b50, 0x141834d18> as *const ()), // sub_1400ec2b0
    Replacement::draft(0x1400ec310, "unknown_1400ec310", tpl_46::<0x143a2b9c8, 0x143a2b9cb, 0x143078b08, 0x142409b50, 0x1425f2b60, 0x141834d18> as *const ()), // sub_1400ec310
    Replacement::draft(0x1400ec370, "unknown_1400ec370", tpl_55::<0x143a2c078, 0x143a2c07b, 0x143078b20, 0x142409b50, 0x1425f2b70, 0x141834d18> as *const ()), // sub_1400ec370
    Replacement::draft(0x1400ec3d0, "unknown_1400ec3d0", tpl_46::<0x143a2bf10, 0x143a2bf13, 0x143078b30, 0x142409b50, 0x1425f2b80, 0x141834d18> as *const ()), // sub_1400ec3d0
    Replacement::draft(0x1400ec430, "unknown_1400ec430", tpl_50::<0x143a2bcb8, 0x143a2bcbb, 0x143078b48, 0x142409b50, 0x1425f2b90, 0x141834d18> as *const ()), // sub_1400ec430
    Replacement::draft(0x1400ec490, "unknown_1400ec490", tpl_42::<0x143a2b978, 0x143a2b97b, 0x143078b60, 0x142409b50, 0x1425f2ba0, 0x141834d18> as *const ()), // sub_1400ec490
    Replacement::draft(0x1400ec4f0, "unknown_1400ec4f0", tpl_38::<0x143a2bfd8, 0x143a2bfdb, 0x143078b78, 0x142409b50, 0x1425f2bb0, 0x141834d18> as *const ()), // sub_1400ec4f0
    Replacement::draft(0x1400ec550, "unknown_1400ec550", tpl_34::<0x143a2c348, 0x143a2c34b, 0x143078b88, 0x142409b50, 0x1425f2bc0, 0x141834d18> as *const ()), // sub_1400ec550
    Replacement::draft(0x1400ec5b0, "unknown_1400ec5b0", tpl_41::<0x143a2c140, 0x143a2c143, 0x143078b98, 0x142409b50, 0x1425f2bd0, 0x141834d18> as *const ()), // sub_1400ec5b0
    Replacement::draft(0x1400ec610, "unknown_1400ec610", tpl_36::<0x143a2bec0, 0x143a2bec3, 0x143078ba8, 0x142409b50, 0x1425f2be0, 0x141834d18> as *const ()), // sub_1400ec610
    Replacement::draft(0x1400ec670, "unknown_1400ec670", tpl_34::<0x143a2b9a0, 0x143a2b9a3, 0x143078bb8, 0x142409b50, 0x1425f2bf0, 0x141834d18> as *const ()), // sub_1400ec670
    Replacement::draft(0x1400ec790, "unknown_1400ec790", fn_1400ec790 as *const ()), // sub_1400ec790
    Replacement::draft(0x1400ed4f0, "unknown_1400ed4f0", tpl_23::<0x143a2c398, 0x143a2c39b, 0x1426cdcb0, 0x142409b50, 0x1425f2f80, 0x141834d18> as *const ()), // sub_1400ed4f0
    Replacement::draft(0x1400ed550, "unknown_1400ed550", tpl_23::<0x143a2c3e0, 0x143a2c3e3, 0x1426cdcb0, 0x142409b50, 0x1425f2f90, 0x141834d18> as *const ()), // sub_1400ed550
    Replacement::draft(0x1400ed740, "unknown_1400ed740", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c0c0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f2fd0, 0x141834d18> as *const ()), // sub_1400ed740
    Replacement::draft(0x1400edad0, "unknown_1400edad0", tpl_61::<0x143a2c500, 0x143a2c503, 0x143091fd8, 0x142409b50, 0x1425f3010, 0x141834d18> as *const ()), // sub_1400edad0
    Replacement::draft(0x1400edb30, "unknown_1400edb30", tpl_62::<0x143a2c503, 0x143a2c500, 0x143a2c508, 0x143a2c504, 0x143a2b658> as *const ()), // sub_1400edb30
    Replacement::draft(0x1400edbc0, "unknown_1400edbc0", tpl_63::<0x143a2c4b0, 0x143a2c4b3, 0x143091ff0, 0x142409b50, 0x1425f3020, 0x141834d18> as *const ()), // sub_1400edbc0
    Replacement::draft(0x1400edc20, "unknown_1400edc20", tpl_64::<0x143a2c460, 0x143a2c463, 0x143091fa0, 0x142409b50, 0x1425f3030, 0x141834d18> as *const ()), // sub_1400edc20
    Replacement::draft(0x1400edc80, "unknown_1400edc80", tpl_65::<0x143a2c410, 0x143a2c413, 0x143091fc0, 0x142409b50, 0x1425f3040, 0x141834d18> as *const ()), // sub_1400edc80
    Replacement::draft(0x1400edce0, "unknown_1400edce0", tpl_62::<0x143a2c4b3, 0x143a2c4b0, 0x143a2c4b8, 0x143a2c4b4, 0x143a2b848> as *const ()), // sub_1400edce0
    Replacement::draft(0x1400edd70, "unknown_1400edd70", tpl_62::<0x143a2c463, 0x143a2c460, 0x143a2c468, 0x143a2c464, 0x143a2b6f8> as *const ()), // sub_1400edd70
    Replacement::draft(0x1400ede00, "unknown_1400ede00", tpl_62::<0x143a2c413, 0x143a2c410, 0x143a2c418, 0x143a2c414, 0x143a2b758> as *const ()), // sub_1400ede00
    Replacement::draft(0x1400ee480, "unknown_1400ee480", tpl_61::<0x143a2c700, 0x143a2c703, 0x143091fd8, 0x142409b50, 0x1425f3330, 0x141834d18> as *const ()), // sub_1400ee480
    Replacement::draft(0x1400ee4e0, "unknown_1400ee4e0", tpl_62::<0x143a2c703, 0x143a2c700, 0x143a2c708, 0x143a2c704, 0x143a2c408> as *const ()), // sub_1400ee4e0
    Replacement::draft(0x1400ee570, "unknown_1400ee570", tpl_63::<0x143a2c6b0, 0x143a2c6b3, 0x143091ff0, 0x142409b50, 0x1425f3340, 0x141834d18> as *const ()), // sub_1400ee570
    Replacement::draft(0x1400ee5d0, "unknown_1400ee5d0", tpl_64::<0x143a2c640, 0x143a2c643, 0x143091fa0, 0x142409b50, 0x1425f3350, 0x141834d18> as *const ()), // sub_1400ee5d0
    Replacement::draft(0x1400ee630, "unknown_1400ee630", tpl_65::<0x143a2c5d0, 0x143a2c5d3, 0x143091fc0, 0x142409b50, 0x1425f3360, 0x141834d18> as *const ()), // sub_1400ee630
    Replacement::draft(0x1400ee690, "unknown_1400ee690", tpl_62::<0x143a2c6b3, 0x143a2c6b0, 0x143a2c6b8, 0x143a2c6b4, 0x143a2c4f8> as *const ()), // sub_1400ee690
    Replacement::draft(0x1400ee720, "unknown_1400ee720", tpl_62::<0x143a2c643, 0x143a2c640, 0x143a2c648, 0x143a2c644, 0x143a2c458> as *const ()), // sub_1400ee720
    Replacement::draft(0x1400ee7b0, "unknown_1400ee7b0", tpl_62::<0x143a2c5d3, 0x143a2c5d0, 0x143a2c5d8, 0x143a2c5d4, 0x143a2c4a8> as *const ()), // sub_1400ee7b0
    Replacement::draft(0x1400ee8e0, "unknown_1400ee8e0", tpl_23::<0x143a2c548, 0x143a2c54b, 0x1426cdcb0, 0x142409b50, 0x1425f3370, 0x141834d18> as *const ()), // sub_1400ee8e0
    Replacement::draft(0x1400ee940, "unknown_1400ee940", tpl_23::<0x143a2c590, 0x143a2c593, 0x1426cdcb0, 0x142409b50, 0x1425f3380, 0x141834d18> as *const ()), // sub_1400ee940
    Replacement::draft(0x1400ef0a0, "unknown_1400ef0a0", tpl_61::<0x143a2c8b0, 0x143a2c8b3, 0x143091fd8, 0x142409b50, 0x1425f33b0, 0x141834d18> as *const ()), // sub_1400ef0a0
    Replacement::draft(0x1400ef100, "unknown_1400ef100", tpl_62::<0x143a2c8b3, 0x143a2c8b0, 0x143a2c8b8, 0x143a2c8b4, 0x143a2c5b8> as *const ()), // sub_1400ef100
    Replacement::draft(0x1400ef190, "unknown_1400ef190", tpl_63::<0x143a2c860, 0x143a2c863, 0x143091ff0, 0x142409b50, 0x1425f33c0, 0x141834d18> as *const ()), // sub_1400ef190
    Replacement::draft(0x1400ef1f0, "unknown_1400ef1f0", tpl_64::<0x143a2c810, 0x143a2c813, 0x143091fa0, 0x142409b50, 0x1425f33d0, 0x141834d18> as *const ()), // sub_1400ef1f0
    Replacement::draft(0x1400ef250, "unknown_1400ef250", tpl_65::<0x143a2c7c0, 0x143a2c7c3, 0x143091fc0, 0x142409b50, 0x1425f33e0, 0x141834d18> as *const ()), // sub_1400ef250
    Replacement::draft(0x1400ef2b0, "unknown_1400ef2b0", tpl_62::<0x143a2c863, 0x143a2c860, 0x143a2c868, 0x143a2c864, 0x143a2c6f8> as *const ()), // sub_1400ef2b0
    Replacement::draft(0x1400ef340, "unknown_1400ef340", tpl_62::<0x143a2c813, 0x143a2c810, 0x143a2c818, 0x143a2c814, 0x143a2c618> as *const ()), // sub_1400ef340
    Replacement::draft(0x1400ef3d0, "unknown_1400ef3d0", tpl_62::<0x143a2c7c3, 0x143a2c7c0, 0x143a2c7c8, 0x143a2c7c4, 0x143a2c688> as *const ()), // sub_1400ef3d0
    Replacement::draft(0x1400ef460, "unknown_1400ef460", tpl_23::<0x143a2c748, 0x143a2c74b, 0x1426cdcb0, 0x142409b50, 0x1425f33f0, 0x141834d18> as *const ()), // sub_1400ef460
    Replacement::draft(0x1400ef4c0, "unknown_1400ef4c0", tpl_23::<0x143a2c790, 0x143a2c793, 0x1426cdcb0, 0x142409b50, 0x1425f3400, 0x141834d18> as *const ()), // sub_1400ef4c0
    Replacement::draft(0x1400f03e0, "unknown_1400f03e0", tpl_23::<0x143a2c8f8, 0x143a2c8fb, 0x1426cdcb0, 0x142409b50, 0x1425f3500, 0x141834d18> as *const ()), // sub_1400f03e0
    Replacement::draft(0x1400f0440, "unknown_1400f0440", tpl_23::<0x143a2c940, 0x143a2c943, 0x1426cdcb0, 0x142409b50, 0x1425f3510, 0x141834d18> as *const ()), // sub_1400f0440
    Replacement::draft(0x1400f0630, "unknown_1400f0630", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c208, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f3550, 0x141834d18> as *const ()), // sub_1400f0630
    Replacement::draft(0x1400f10e0, "unknown_1400f10e0", tpl_134::<0x50, 0x10, 0x18, 0xffffffffffffffc8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143a2c9c0, 0x1425f3830, 0x141834d18> as *const ()), // sub_1400f10e0
    Replacement::draft(0x1400f1170, "unknown_1400f1170", tpl_134::<0x50, 0x10, 0x18, 0xffffffffffffffc8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143a2c9b0, 0x1425f3990, 0x141834d18> as *const ()), // sub_1400f1170
    Replacement::draft(0x1400f1200, "unknown_1400f1200", tpl_23::<0x143a2c9d0, 0x143a2c9d3, 0x1426cdcb0, 0x142409b50, 0x1425f3af0, 0x141834d18> as *const ()), // sub_1400f1200
    Replacement::draft(0x1400f1260, "unknown_1400f1260", tpl_23::<0x143a2ca18, 0x143a2ca1b, 0x1426cdcb0, 0x142409b50, 0x1425f3b00, 0x141834d18> as *const ()), // sub_1400f1260
    Replacement::draft(0x1400f1450, "unknown_1400f1450", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c390, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f3b40, 0x141834d18> as *const ()), // sub_1400f1450
    Replacement::draft(0x1400f1c50, "unknown_1400f1c50", tpl_23::<0x143a2ca80, 0x143a2ca83, 0x1426cdcb0, 0x142409b50, 0x1425f3c40, 0x141834d18> as *const ()), // sub_1400f1c50
    Replacement::draft(0x1400f1cb0, "unknown_1400f1cb0", tpl_23::<0x143a2cac8, 0x143a2cacb, 0x1426cdcb0, 0x142409b50, 0x1425f3c50, 0x141834d18> as *const ()), // sub_1400f1cb0
    Replacement::draft(0x1400f1ea0, "unknown_1400f1ea0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c438, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f3c90, 0x141834d18> as *const ()), // sub_1400f1ea0
    Replacement::draft(0x1400f2390, "unknown_1400f2390", tpl_61::<0x143a2cc70, 0x143a2cc73, 0x143091fd8, 0x142409b50, 0x1425f3d00, 0x141834d18> as *const ()), // sub_1400f2390
    Replacement::draft(0x1400f23f0, "unknown_1400f23f0", tpl_62::<0x143a2cc73, 0x143a2cc70, 0x143a2cc78, 0x143a2cc74, 0x143a2c7b8> as *const ()), // sub_1400f23f0
    Replacement::draft(0x1400f2480, "unknown_1400f2480", tpl_63::<0x143a2cc20, 0x143a2cc23, 0x143091ff0, 0x142409b50, 0x1425f3d10, 0x141834d18> as *const ()), // sub_1400f2480
    Replacement::draft(0x1400f24e0, "unknown_1400f24e0", tpl_64::<0x143a2cbb0, 0x143a2cbb3, 0x143091fa0, 0x142409b50, 0x1425f3d20, 0x141834d18> as *const ()), // sub_1400f24e0
    Replacement::draft(0x1400f2540, "unknown_1400f2540", tpl_65::<0x143a2cb40, 0x143a2cb43, 0x143091fc0, 0x142409b50, 0x1425f3d30, 0x141834d18> as *const ()), // sub_1400f2540
    Replacement::draft(0x1400f25a0, "unknown_1400f25a0", tpl_62::<0x143a2cc23, 0x143a2cc20, 0x143a2cc28, 0x143a2cc24, 0x143a2c8a8> as *const ()), // sub_1400f25a0
    Replacement::draft(0x1400f2630, "unknown_1400f2630", tpl_62::<0x143a2cbb3, 0x143a2cbb0, 0x143a2cbb8, 0x143a2cbb4, 0x143a2c808> as *const ()), // sub_1400f2630
    Replacement::draft(0x1400f26c0, "unknown_1400f26c0", tpl_62::<0x143a2cb43, 0x143a2cb40, 0x143a2cb48, 0x143a2cb44, 0x143a2c858> as *const ()), // sub_1400f26c0
    Replacement::draft(0x1400f2d90, "unknown_1400f2d90", tpl_23::<0x143a2ccb8, 0x143a2ccbb, 0x1426cdcb0, 0x142409b50, 0x1425f3d70, 0x141834d18> as *const ()), // sub_1400f2d90
    Replacement::draft(0x1400f2df0, "unknown_1400f2df0", tpl_23::<0x143a2cd00, 0x143a2cd03, 0x1426cdcb0, 0x142409b50, 0x1425f3d80, 0x141834d18> as *const ()), // sub_1400f2df0
    Replacement::draft(0x1400f2fe0, "unknown_1400f2fe0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c4c8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f3dc0, 0x141834d18> as *const ()), // sub_1400f2fe0
    Replacement::draft(0x1400f3460, "unknown_1400f3460", tpl_135::<0x14307aafc, 0x1431ba1cd, 0x143a00288, 0x143cb41e8, 0x14384bfdc, 0x143cb41f0, 0x140581e30, 0x143cb4218, 0x143cb41f8, 0x143cb4208> as *const ()), // sub_1400f3460
    Replacement::draft(0x1400f36d0, "unknown_1400f36d0", fn_1400f36d0 as *const ()), // sub_1400f36d0
    Replacement::draft(0x1400f3760, "unknown_1400f3760", fn_1400f3760 as *const ()), // sub_1400f3760
    Replacement::draft(0x1400f3ac0, "unknown_1400f3ac0", fn_1400f3ac0 as *const ()), // sub_1400f3ac0
    Replacement::draft(0x1400f3b50, "unknown_1400f3b50", tpl_23::<0x143a2cd70, 0x143a2cd73, 0x1426cdcb0, 0x142409b50, 0x1425f3f70, 0x141834d18> as *const ()), // sub_1400f3b50
    Replacement::draft(0x1400f3bb0, "unknown_1400f3bb0", tpl_23::<0x143a2cdb8, 0x143a2cdbb, 0x1426cdcb0, 0x142409b50, 0x1425f3f80, 0x141834d18> as *const ()), // sub_1400f3bb0
    Replacement::draft(0x1400f3da0, "unknown_1400f3da0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c598, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f3fc0, 0x141834d18> as *const ()), // sub_1400f3da0
    Replacement::draft(0x1400f41a0, "unknown_1400f41a0", fn_1400f41a0 as *const ()), // sub_1400f41a0
    Replacement::draft(0x1400f4e30, "unknown_1400f4e30", tpl_23::<0x143a2ce20, 0x143a2ce23, 0x1426cdcb0, 0x142409b50, 0x1425f4100, 0x141834d18> as *const ()), // sub_1400f4e30
    Replacement::draft(0x1400f4e90, "unknown_1400f4e90", tpl_23::<0x143a2ce68, 0x143a2ce6b, 0x1426cdcb0, 0x142409b50, 0x1425f4110, 0x141834d18> as *const ()), // sub_1400f4e90
    Replacement::draft(0x1400f5080, "unknown_1400f5080", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c658, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f4150, 0x141834d18> as *const ()), // sub_1400f5080
    Replacement::draft(0x1400f5410, "unknown_1400f5410", tpl_23::<0x143a2d048, 0x143a2d04b, 0x1426cdcb0, 0x142409b50, 0x1425f4190, 0x141834d18> as *const ()), // sub_1400f5410
    Replacement::draft(0x1400f5470, "unknown_1400f5470", tpl_23::<0x143a2d090, 0x143a2d093, 0x1426cdcb0, 0x142409b50, 0x1425f41a0, 0x141834d18> as *const ()), // sub_1400f5470
    Replacement::draft(0x1400f5660, "unknown_1400f5660", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c6e8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f41e0, 0x141834d18> as *const ()), // sub_1400f5660
    Replacement::draft(0x1400f5a60, "unknown_1400f5a60", tpl_34::<0x143a2d1f8, 0x143a2d1fb, 0x143078668, 0x142409b50, 0x1425f4250, 0x141834d18> as *const ()), // sub_1400f5a60
    Replacement::draft(0x1400f5ac0, "unknown_1400f5ac0", tpl_23::<0x143a2d808, 0x143a2d80b, 0x1426cdcb0, 0x142409b50, 0x1425f4260, 0x141834d18> as *const ()), // sub_1400f5ac0
    Replacement::draft(0x1400f5b20, "unknown_1400f5b20", tpl_35::<0x143a2d538, 0x143a2d53b, 0x143078678, 0x142409b50, 0x1425f4270, 0x141834d18> as *const ()), // sub_1400f5b20
    Replacement::draft(0x1400f5b80, "unknown_1400f5b80", tpl_36::<0x143a2d790, 0x143a2d793, 0x143078680, 0x142409b50, 0x1425f4280, 0x141834d18> as *const ()), // sub_1400f5b80
    Replacement::draft(0x1400f5be0, "unknown_1400f5be0", tpl_37::<0x143a2d3d8, 0x143a2d3db, 0x143078690, 0x142409b50, 0x1425f4290, 0x141834d18> as *const ()), // sub_1400f5be0
    Replacement::draft(0x1400f5c40, "unknown_1400f5c40", tpl_38::<0x143a2d298, 0x143a2d29b, 0x1430786a0, 0x142409b50, 0x1425f42a0, 0x141834d18> as *const ()), // sub_1400f5c40
    Replacement::draft(0x1400f5ca0, "unknown_1400f5ca0", tpl_39::<0x143a2d830, 0x143a2d833, 0x1430786b0, 0x142409b50, 0x1425f42b0, 0x141834d18> as *const ()), // sub_1400f5ca0
    Replacement::draft(0x1400f5d00, "unknown_1400f5d00", tpl_37::<0x143a2d1a8, 0x143a2d1ab, 0x1430786b8, 0x142409b50, 0x1425f42c0, 0x141834d18> as *const ()), // sub_1400f5d00
    Replacement::draft(0x1400f5d60, "unknown_1400f5d60", tpl_40::<0x143a2d948, 0x143a2d94b, 0x1430786c4, 0x142409b50, 0x1425f42d0, 0x141834d18> as *const ()), // sub_1400f5d60
    Replacement::draft(0x1400f5dc0, "unknown_1400f5dc0", tpl_41::<0x143a2d360, 0x143a2d363, 0x1430786d0, 0x142409b50, 0x1425f42e0, 0x141834d18> as *const ()), // sub_1400f5dc0
    Replacement::draft(0x1400f5e20, "unknown_1400f5e20", tpl_42::<0x143a2d880, 0x143a2d883, 0x1430786e0, 0x142409b50, 0x1425f42f0, 0x141834d18> as *const ()), // sub_1400f5e20
    Replacement::draft(0x1400f5e80, "unknown_1400f5e80", tpl_34::<0x143a2d8d0, 0x143a2d8d3, 0x1430786f8, 0x142409b50, 0x1425f4300, 0x141834d18> as *const ()), // sub_1400f5e80
    Replacement::draft(0x1400f5ee0, "unknown_1400f5ee0", tpl_34::<0x143a2d1d0, 0x143a2d1d3, 0x143078708, 0x142409b50, 0x1425f4310, 0x141834d18> as *const ()), // sub_1400f5ee0
    Replacement::draft(0x1400f5f40, "unknown_1400f5f40", tpl_41::<0x143a2d2c0, 0x143a2d2c3, 0x143078718, 0x142409b50, 0x1425f4320, 0x141834d18> as *const ()), // sub_1400f5f40
    Replacement::draft(0x1400f5fa0, "unknown_1400f5fa0", tpl_43::<0x143a2d400, 0x143a2d403, 0x143078728, 0x142409b50, 0x1425f4330, 0x141834d18> as *const ()), // sub_1400f5fa0
    Replacement::draft(0x1400f6000, "unknown_1400f6000", tpl_44::<0x143a2ceb8, 0x143a2cebb, 0x143078748, 0x142409b50, 0x1425f4340, 0x141834d18> as *const ()), // sub_1400f6000
    Replacement::draft(0x1400f6060, "unknown_1400f6060", tpl_39::<0x143a2d388, 0x143a2d38b, 0x143078760, 0x142409b50, 0x1425f4350, 0x141834d18> as *const ()), // sub_1400f6060
    Replacement::draft(0x1400f60c0, "unknown_1400f60c0", tpl_45::<0x143a2d270, 0x143a2d273, 0x143078768, 0x142409b50, 0x1425f4360, 0x141834d18> as *const ()), // sub_1400f60c0
    Replacement::draft(0x1400f6120, "unknown_1400f6120", tpl_41::<0x143a2d740, 0x143a2d743, 0x143078778, 0x142409b50, 0x1425f4370, 0x141834d18> as *const ()), // sub_1400f6120
    Replacement::draft(0x1400f6180, "unknown_1400f6180", tpl_37::<0x143a2cee0, 0x143a2cee3, 0x143078788, 0x142409b50, 0x1425f4380, 0x141834d18> as *const ()), // sub_1400f6180
    Replacement::draft(0x1400f61e0, "unknown_1400f61e0", tpl_46::<0x143a2d5d8, 0x143a2d5db, 0x143078798, 0x142409b50, 0x1425f4390, 0x141834d18> as *const ()), // sub_1400f61e0
    Replacement::draft(0x1400f6240, "unknown_1400f6240", tpl_47::<0x143a2d108, 0x143a2d10b, 0x1430787b0, 0x142409b50, 0x1425f43a0, 0x141834d18> as *const ()), // sub_1400f6240
    Replacement::draft(0x1400f62a0, "unknown_1400f62a0", tpl_38::<0x143a2d678, 0x143a2d67b, 0x1430787d0, 0x142409b50, 0x1425f43b0, 0x141834d18> as *const ()), // sub_1400f62a0
    Replacement::draft(0x1400f6300, "unknown_1400f6300", tpl_48::<0x143a2d858, 0x143a2d85b, 0x1430787e0, 0x142409b50, 0x1425f43c0, 0x141834d18> as *const ()), // sub_1400f6300
    Replacement::draft(0x1400f6360, "unknown_1400f6360", tpl_49::<0x143a2d510, 0x143a2d513, 0x1430787f8, 0x142409b50, 0x1425f43d0, 0x141834d18> as *const ()), // sub_1400f6360
    Replacement::draft(0x1400f63c0, "unknown_1400f63c0", tpl_44::<0x143a2d7e0, 0x143a2d7e3, 0x143078808, 0x142409b50, 0x1425f43e0, 0x141834d18> as *const ()), // sub_1400f63c0
    Replacement::draft(0x1400f6420, "unknown_1400f6420", tpl_42::<0x143a2ce90, 0x143a2ce93, 0x143078820, 0x142409b50, 0x1425f43f0, 0x141834d18> as *const ()), // sub_1400f6420
    Replacement::draft(0x1400f6480, "unknown_1400f6480", tpl_50::<0x143a2d6f0, 0x143a2d6f3, 0x143078838, 0x142409b50, 0x1425f4400, 0x141834d18> as *const ()), // sub_1400f6480
    Replacement::draft(0x1400f64e0, "unknown_1400f64e0", tpl_52::<0x143a2d020, 0x143a2d023, 0x143a2d948, 0x143a2d94c, 0x143a2d94b, 0x143a2d950, 0x142409b50, 0x1425f4410, 0x141834d18> as *const ()), // sub_1400f64e0
    Replacement::draft(0x1400f65a0, "unknown_1400f65a0", tpl_38::<0x143a2d560, 0x143a2d563, 0x143078950, 0x142409b50, 0x1425f4420, 0x141834d18> as *const ()), // sub_1400f65a0
    Replacement::draft(0x1400f6600, "unknown_1400f6600", tpl_53::<0x143a2d0e0, 0x143a2d0e3, 0x143078960, 0x142409b50, 0x1425f4430, 0x141834d18> as *const ()), // sub_1400f6600
    Replacement::draft(0x1400f6660, "unknown_1400f6660", tpl_45::<0x143a2cfa8, 0x143a2cfab, 0x143078980, 0x142409b50, 0x1425f4440, 0x141834d18> as *const ()), // sub_1400f6660
    Replacement::draft(0x1400f66c0, "unknown_1400f66c0", tpl_35::<0x143a2cff8, 0x143a2cffb, 0x14307898c, 0x142409b50, 0x1425f4450, 0x141834d18> as *const ()), // sub_1400f66c0
    Replacement::draft(0x1400f6720, "unknown_1400f6720", tpl_42::<0x143a2d8a8, 0x143a2d8ab, 0x1430786e0, 0x142409b50, 0x1425f4460, 0x141834d18> as *const ()), // sub_1400f6720
    Replacement::draft(0x1400f6780, "unknown_1400f6780", tpl_38::<0x143a2d7b8, 0x143a2d7bb, 0x143078998, 0x142409b50, 0x1425f4470, 0x141834d18> as *const ()), // sub_1400f6780
    Replacement::draft(0x1400f67e0, "unknown_1400f67e0", tpl_38::<0x143a2d310, 0x143a2d313, 0x1430789a8, 0x142409b50, 0x1425f4480, 0x141834d18> as *const ()), // sub_1400f67e0
    Replacement::draft(0x1400f6840, "unknown_1400f6840", tpl_41::<0x143a2d470, 0x143a2d473, 0x1430789b8, 0x142409b50, 0x1425f4490, 0x141834d18> as *const ()), // sub_1400f6840
    Replacement::draft(0x1400f68a0, "unknown_1400f68a0", tpl_34::<0x143a2d8f8, 0x143a2d8fb, 0x1430789c8, 0x142409b50, 0x1425f44a0, 0x141834d18> as *const ()), // sub_1400f68a0
    Replacement::draft(0x1400f6900, "unknown_1400f6900", tpl_39::<0x143a2cfd0, 0x143a2cfd3, 0x1430789d8, 0x142409b50, 0x1425f44b0, 0x141834d18> as *const ()), // sub_1400f6900
    Replacement::draft(0x1400f6960, "unknown_1400f6960", tpl_41::<0x143a2d2e8, 0x143a2d2eb, 0x1430789e0, 0x142409b50, 0x1425f44c0, 0x141834d18> as *const ()), // sub_1400f6960
    Replacement::draft(0x1400f69c0, "unknown_1400f69c0", tpl_34::<0x143a2d588, 0x143a2d58b, 0x1430789f0, 0x142409b50, 0x1425f44d0, 0x141834d18> as *const ()), // sub_1400f69c0
    Replacement::draft(0x1400f6a20, "unknown_1400f6a20", tpl_48::<0x143a2d220, 0x143a2d223, 0x143078a00, 0x142409b50, 0x1425f44e0, 0x141834d18> as *const ()), // sub_1400f6a20
    Replacement::draft(0x1400f6a80, "unknown_1400f6a80", tpl_46::<0x143a2d6a0, 0x143a2d6a3, 0x143078a18, 0x142409b50, 0x1425f44f0, 0x141834d18> as *const ()), // sub_1400f6a80
    Replacement::draft(0x1400f6ae0, "unknown_1400f6ae0", tpl_47::<0x143a2d600, 0x143a2d603, 0x143078a30, 0x142409b50, 0x1425f4500, 0x141834d18> as *const ()), // sub_1400f6ae0
    Replacement::draft(0x1400f6b40, "unknown_1400f6b40", tpl_37::<0x143a2d158, 0x143a2d15b, 0x143078a50, 0x142409b50, 0x1425f4510, 0x141834d18> as *const ()), // sub_1400f6b40
    Replacement::draft(0x1400f6ba0, "unknown_1400f6ba0", tpl_45::<0x143a2d3b0, 0x143a2d3b3, 0x143078a60, 0x142409b50, 0x1425f4520, 0x141834d18> as *const ()), // sub_1400f6ba0
    Replacement::draft(0x1400f6c00, "unknown_1400f6c00", tpl_54::<0x143a2d4c0, 0x143a2d4c3, 0x143078a70, 0x142409b50, 0x1425f4530, 0x141834d18> as *const ()), // sub_1400f6c00
    Replacement::draft(0x1400f6c60, "unknown_1400f6c60", tpl_40::<0x143a2d130, 0x143a2d133, 0x143078a88, 0x142409b50, 0x1425f4540, 0x141834d18> as *const ()), // sub_1400f6c60
    Replacement::draft(0x1400f6cc0, "unknown_1400f6cc0", tpl_50::<0x143a2d180, 0x143a2d183, 0x143078a90, 0x142409b50, 0x1425f4550, 0x141834d18> as *const ()), // sub_1400f6cc0
    Replacement::draft(0x1400f6d20, "unknown_1400f6d20", tpl_41::<0x143a2d338, 0x143a2d33b, 0x143078aa8, 0x142409b50, 0x1425f4560, 0x141834d18> as *const ()), // sub_1400f6d20
    Replacement::draft(0x1400f6d80, "unknown_1400f6d80", tpl_55::<0x143a2d628, 0x143a2d62b, 0x143078ab8, 0x142409b50, 0x1425f4570, 0x141834d18> as *const ()), // sub_1400f6d80
    Replacement::draft(0x1400f6de0, "unknown_1400f6de0", tpl_41::<0x143a2d6c8, 0x143a2d6cb, 0x143078ac8, 0x142409b50, 0x1425f4580, 0x141834d18> as *const ()), // sub_1400f6de0
    Replacement::draft(0x1400f6e40, "unknown_1400f6e40", tpl_49::<0x143a2d768, 0x143a2d76b, 0x143078ad8, 0x142409b50, 0x1425f4590, 0x141834d18> as *const ()), // sub_1400f6e40
    Replacement::draft(0x1400f6ea0, "unknown_1400f6ea0", tpl_40::<0x143a2cf80, 0x143a2cf83, 0x143078ae4, 0x142409b50, 0x1425f45a0, 0x141834d18> as *const ()), // sub_1400f6ea0
    Replacement::draft(0x1400f6f00, "unknown_1400f6f00", tpl_44::<0x143a2d0b8, 0x143a2d0bb, 0x143078af0, 0x142409b50, 0x1425f45b0, 0x141834d18> as *const ()), // sub_1400f6f00
    Replacement::draft(0x1400f6f60, "unknown_1400f6f60", tpl_46::<0x143a2cf58, 0x143a2cf5b, 0x143078b08, 0x142409b50, 0x1425f45c0, 0x141834d18> as *const ()), // sub_1400f6f60
    Replacement::draft(0x1400f6fc0, "unknown_1400f6fc0", tpl_55::<0x143a2d650, 0x143a2d653, 0x143078b20, 0x142409b50, 0x1425f45d0, 0x141834d18> as *const ()), // sub_1400f6fc0
    Replacement::draft(0x1400f7020, "unknown_1400f7020", tpl_46::<0x143a2d4e8, 0x143a2d4eb, 0x143078b30, 0x142409b50, 0x1425f45e0, 0x141834d18> as *const ()), // sub_1400f7020
    Replacement::draft(0x1400f7080, "unknown_1400f7080", tpl_50::<0x143a2d248, 0x143a2d24b, 0x143078b48, 0x142409b50, 0x1425f45f0, 0x141834d18> as *const ()), // sub_1400f7080
    Replacement::draft(0x1400f70e0, "unknown_1400f70e0", tpl_42::<0x143a2cf08, 0x143a2cf0b, 0x143078b60, 0x142409b50, 0x1425f4600, 0x141834d18> as *const ()), // sub_1400f70e0
    Replacement::draft(0x1400f7140, "unknown_1400f7140", tpl_38::<0x143a2d5b0, 0x143a2d5b3, 0x143078b78, 0x142409b50, 0x1425f4610, 0x141834d18> as *const ()), // sub_1400f7140
    Replacement::draft(0x1400f71a0, "unknown_1400f71a0", tpl_34::<0x143a2d920, 0x143a2d923, 0x143078b88, 0x142409b50, 0x1425f4620, 0x141834d18> as *const ()), // sub_1400f71a0
    Replacement::draft(0x1400f7200, "unknown_1400f7200", tpl_41::<0x143a2d718, 0x143a2d71b, 0x143078b98, 0x142409b50, 0x1425f4630, 0x141834d18> as *const ()), // sub_1400f7200
    Replacement::draft(0x1400f7260, "unknown_1400f7260", tpl_36::<0x143a2d498, 0x143a2d49b, 0x143078ba8, 0x142409b50, 0x1425f4640, 0x141834d18> as *const ()), // sub_1400f7260
    Replacement::draft(0x1400f72c0, "unknown_1400f72c0", tpl_34::<0x143a2cf30, 0x143a2cf33, 0x143078bb8, 0x142409b50, 0x1425f4650, 0x141834d18> as *const ()), // sub_1400f72c0
    Replacement::draft(0x1400f9dd0, "unknown_1400f9dd0", tpl_23::<0x143a2d970, 0x143a2d973, 0x1426cdcb0, 0x142409b50, 0x1425f4880, 0x141834d18> as *const ()), // sub_1400f9dd0
    Replacement::draft(0x1400f9e30, "unknown_1400f9e30", tpl_23::<0x143a2d9b8, 0x143a2d9bb, 0x1426cdcb0, 0x142409b50, 0x1425f4890, 0x141834d18> as *const ()), // sub_1400f9e30
    Replacement::draft(0x1400fa020, "unknown_1400fa020", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c7b0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f48d0, 0x141834d18> as *const ()), // sub_1400fa020
    Replacement::draft(0x1400fbfd0, "unknown_1400fbfd0", tpl_23::<0x143a2d9e0, 0x143a2d9e3, 0x1426cdcb0, 0x142409b50, 0x1425f4940, 0x141834d18> as *const ()), // sub_1400fbfd0
    Replacement::draft(0x1400fc030, "unknown_1400fc030", tpl_23::<0x143a2da28, 0x143a2da2b, 0x1426cdcb0, 0x142409b50, 0x1425f4950, 0x141834d18> as *const ()), // sub_1400fc030
    Replacement::draft(0x1400fc220, "unknown_1400fc220", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c850, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f4990, 0x141834d18> as *const ()), // sub_1400fc220
    Replacement::draft(0x1400fc6e0, "unknown_1400fc6e0", tpl_23::<0x143a2dc08, 0x143a2dc0b, 0x1426cdcb0, 0x142409b50, 0x1425f4a60, 0x141834d18> as *const ()), // sub_1400fc6e0
    Replacement::draft(0x1400fc740, "unknown_1400fc740", tpl_23::<0x143a2dc50, 0x143a2dc53, 0x1426cdcb0, 0x142409b50, 0x1425f4a70, 0x141834d18> as *const ()), // sub_1400fc740
    Replacement::draft(0x1400fc930, "unknown_1400fc930", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c940, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f4ab0, 0x141834d18> as *const ()), // sub_1400fc930
    Replacement::draft(0x1400fcd30, "unknown_1400fcd30", tpl_34::<0x143a2ddb8, 0x143a2ddbb, 0x143078668, 0x142409b50, 0x1425f4b20, 0x141834d18> as *const ()), // sub_1400fcd30
    Replacement::draft(0x1400fcd90, "unknown_1400fcd90", tpl_23::<0x143a2e3c8, 0x143a2e3cb, 0x1426cdcb0, 0x142409b50, 0x1425f4b30, 0x141834d18> as *const ()), // sub_1400fcd90
    Replacement::draft(0x1400fcdf0, "unknown_1400fcdf0", tpl_35::<0x143a2e0f8, 0x143a2e0fb, 0x143078678, 0x142409b50, 0x1425f4b40, 0x141834d18> as *const ()), // sub_1400fcdf0
    Replacement::draft(0x1400fce50, "unknown_1400fce50", tpl_36::<0x143a2e350, 0x143a2e353, 0x143078680, 0x142409b50, 0x1425f4b50, 0x141834d18> as *const ()), // sub_1400fce50
    Replacement::draft(0x1400fceb0, "unknown_1400fceb0", tpl_37::<0x143a2df98, 0x143a2df9b, 0x143078690, 0x142409b50, 0x1425f4b60, 0x141834d18> as *const ()), // sub_1400fceb0
    Replacement::draft(0x1400fcf10, "unknown_1400fcf10", tpl_38::<0x143a2de58, 0x143a2de5b, 0x1430786a0, 0x142409b50, 0x1425f4b70, 0x141834d18> as *const ()), // sub_1400fcf10
    Replacement::draft(0x1400fcf70, "unknown_1400fcf70", tpl_39::<0x143a2e3f0, 0x143a2e3f3, 0x1430786b0, 0x142409b50, 0x1425f4b80, 0x141834d18> as *const ()), // sub_1400fcf70
    Replacement::draft(0x1400fcfd0, "unknown_1400fcfd0", tpl_37::<0x143a2dd68, 0x143a2dd6b, 0x1430786b8, 0x142409b50, 0x1425f4b90, 0x141834d18> as *const ()), // sub_1400fcfd0
    Replacement::draft(0x1400fd030, "unknown_1400fd030", tpl_40::<0x143a2e508, 0x143a2e50b, 0x1430786c4, 0x142409b50, 0x1425f4ba0, 0x141834d18> as *const ()), // sub_1400fd030
    Replacement::draft(0x1400fd090, "unknown_1400fd090", tpl_41::<0x143a2df20, 0x143a2df23, 0x1430786d0, 0x142409b50, 0x1425f4bb0, 0x141834d18> as *const ()), // sub_1400fd090
    Replacement::draft(0x1400fd0f0, "unknown_1400fd0f0", tpl_42::<0x143a2e440, 0x143a2e443, 0x1430786e0, 0x142409b50, 0x1425f4bc0, 0x141834d18> as *const ()), // sub_1400fd0f0
    Replacement::draft(0x1400fd150, "unknown_1400fd150", tpl_34::<0x143a2e490, 0x143a2e493, 0x1430786f8, 0x142409b50, 0x1425f4bd0, 0x141834d18> as *const ()), // sub_1400fd150
    Replacement::draft(0x1400fd1b0, "unknown_1400fd1b0", tpl_34::<0x143a2dd90, 0x143a2dd93, 0x143078708, 0x142409b50, 0x1425f4be0, 0x141834d18> as *const ()), // sub_1400fd1b0
    Replacement::draft(0x1400fd210, "unknown_1400fd210", tpl_41::<0x143a2de80, 0x143a2de83, 0x143078718, 0x142409b50, 0x1425f4bf0, 0x141834d18> as *const ()), // sub_1400fd210
    Replacement::draft(0x1400fd270, "unknown_1400fd270", tpl_43::<0x143a2dfc0, 0x143a2dfc3, 0x143078728, 0x142409b50, 0x1425f4c00, 0x141834d18> as *const ()), // sub_1400fd270
    Replacement::draft(0x1400fd2d0, "unknown_1400fd2d0", tpl_44::<0x143a2da78, 0x143a2da7b, 0x143078748, 0x142409b50, 0x1425f4c10, 0x141834d18> as *const ()), // sub_1400fd2d0
    Replacement::draft(0x1400fd330, "unknown_1400fd330", tpl_39::<0x143a2df48, 0x143a2df4b, 0x143078760, 0x142409b50, 0x1425f4c20, 0x141834d18> as *const ()), // sub_1400fd330
    Replacement::draft(0x1400fd390, "unknown_1400fd390", tpl_45::<0x143a2de30, 0x143a2de33, 0x143078768, 0x142409b50, 0x1425f4c30, 0x141834d18> as *const ()), // sub_1400fd390
    Replacement::draft(0x1400fd3f0, "unknown_1400fd3f0", tpl_41::<0x143a2e300, 0x143a2e303, 0x143078778, 0x142409b50, 0x1425f4c40, 0x141834d18> as *const ()), // sub_1400fd3f0
    Replacement::draft(0x1400fd450, "unknown_1400fd450", tpl_37::<0x143a2daa0, 0x143a2daa3, 0x143078788, 0x142409b50, 0x1425f4c50, 0x141834d18> as *const ()), // sub_1400fd450
    Replacement::draft(0x1400fd4b0, "unknown_1400fd4b0", tpl_46::<0x143a2e198, 0x143a2e19b, 0x143078798, 0x142409b50, 0x1425f4c60, 0x141834d18> as *const ()), // sub_1400fd4b0
    Replacement::draft(0x1400fd510, "unknown_1400fd510", tpl_47::<0x143a2dcc8, 0x143a2dccb, 0x1430787b0, 0x142409b50, 0x1425f4c70, 0x141834d18> as *const ()), // sub_1400fd510
    Replacement::draft(0x1400fd570, "unknown_1400fd570", tpl_38::<0x143a2e238, 0x143a2e23b, 0x1430787d0, 0x142409b50, 0x1425f4c80, 0x141834d18> as *const ()), // sub_1400fd570
    Replacement::draft(0x1400fd5d0, "unknown_1400fd5d0", tpl_48::<0x143a2e418, 0x143a2e41b, 0x1430787e0, 0x142409b50, 0x1425f4c90, 0x141834d18> as *const ()), // sub_1400fd5d0
    Replacement::draft(0x1400fd630, "unknown_1400fd630", tpl_49::<0x143a2e0d0, 0x143a2e0d3, 0x1430787f8, 0x142409b50, 0x1425f4ca0, 0x141834d18> as *const ()), // sub_1400fd630
    Replacement::draft(0x1400fd690, "unknown_1400fd690", tpl_44::<0x143a2e3a0, 0x143a2e3a3, 0x143078808, 0x142409b50, 0x1425f4cb0, 0x141834d18> as *const ()), // sub_1400fd690
    Replacement::draft(0x1400fd6f0, "unknown_1400fd6f0", tpl_42::<0x143a2da50, 0x143a2da53, 0x143078820, 0x142409b50, 0x1425f4cc0, 0x141834d18> as *const ()), // sub_1400fd6f0
    Replacement::draft(0x1400fd750, "unknown_1400fd750", tpl_50::<0x143a2e2b0, 0x143a2e2b3, 0x143078838, 0x142409b50, 0x1425f4cd0, 0x141834d18> as *const ()), // sub_1400fd750
    Replacement::draft(0x1400fd7b0, "unknown_1400fd7b0", tpl_52::<0x143a2dbe0, 0x143a2dbe3, 0x143a2e508, 0x143a2e50c, 0x143a2e50b, 0x143a2e510, 0x142409b50, 0x1425f4ce0, 0x141834d18> as *const ()), // sub_1400fd7b0
    Replacement::draft(0x1400fd870, "unknown_1400fd870", tpl_38::<0x143a2e120, 0x143a2e123, 0x143078950, 0x142409b50, 0x1425f4cf0, 0x141834d18> as *const ()), // sub_1400fd870
    Replacement::draft(0x1400fd8d0, "unknown_1400fd8d0", tpl_53::<0x143a2dca0, 0x143a2dca3, 0x143078960, 0x142409b50, 0x1425f4d00, 0x141834d18> as *const ()), // sub_1400fd8d0
    Replacement::draft(0x1400fd930, "unknown_1400fd930", tpl_45::<0x143a2db68, 0x143a2db6b, 0x143078980, 0x142409b50, 0x1425f4d10, 0x141834d18> as *const ()), // sub_1400fd930
    Replacement::draft(0x1400fd990, "unknown_1400fd990", tpl_35::<0x143a2dbb8, 0x143a2dbbb, 0x14307898c, 0x142409b50, 0x1425f4d20, 0x141834d18> as *const ()), // sub_1400fd990
    Replacement::draft(0x1400fd9f0, "unknown_1400fd9f0", tpl_42::<0x143a2e468, 0x143a2e46b, 0x1430786e0, 0x142409b50, 0x1425f4d30, 0x141834d18> as *const ()), // sub_1400fd9f0
    Replacement::draft(0x1400fda50, "unknown_1400fda50", tpl_38::<0x143a2e378, 0x143a2e37b, 0x143078998, 0x142409b50, 0x1425f4d40, 0x141834d18> as *const ()), // sub_1400fda50
    Replacement::draft(0x1400fdab0, "unknown_1400fdab0", tpl_38::<0x143a2ded0, 0x143a2ded3, 0x1430789a8, 0x142409b50, 0x1425f4d50, 0x141834d18> as *const ()), // sub_1400fdab0
    Replacement::draft(0x1400fdb10, "unknown_1400fdb10", tpl_41::<0x143a2e030, 0x143a2e033, 0x1430789b8, 0x142409b50, 0x1425f4d60, 0x141834d18> as *const ()), // sub_1400fdb10
    Replacement::draft(0x1400fdb70, "unknown_1400fdb70", tpl_34::<0x143a2e4b8, 0x143a2e4bb, 0x1430789c8, 0x142409b50, 0x1425f4d70, 0x141834d18> as *const ()), // sub_1400fdb70
    Replacement::draft(0x1400fdbd0, "unknown_1400fdbd0", tpl_39::<0x143a2db90, 0x143a2db93, 0x1430789d8, 0x142409b50, 0x1425f4d80, 0x141834d18> as *const ()), // sub_1400fdbd0
    Replacement::draft(0x1400fdc30, "unknown_1400fdc30", tpl_41::<0x143a2dea8, 0x143a2deab, 0x1430789e0, 0x142409b50, 0x1425f4d90, 0x141834d18> as *const ()), // sub_1400fdc30
    Replacement::draft(0x1400fdc90, "unknown_1400fdc90", tpl_34::<0x143a2e148, 0x143a2e14b, 0x1430789f0, 0x142409b50, 0x1425f4da0, 0x141834d18> as *const ()), // sub_1400fdc90
    Replacement::draft(0x1400fdcf0, "unknown_1400fdcf0", tpl_48::<0x143a2dde0, 0x143a2dde3, 0x143078a00, 0x142409b50, 0x1425f4db0, 0x141834d18> as *const ()), // sub_1400fdcf0
    Replacement::draft(0x1400fdd50, "unknown_1400fdd50", tpl_46::<0x143a2e260, 0x143a2e263, 0x143078a18, 0x142409b50, 0x1425f4dc0, 0x141834d18> as *const ()), // sub_1400fdd50
    Replacement::draft(0x1400fddb0, "unknown_1400fddb0", tpl_47::<0x143a2e1c0, 0x143a2e1c3, 0x143078a30, 0x142409b50, 0x1425f4dd0, 0x141834d18> as *const ()), // sub_1400fddb0
    Replacement::draft(0x1400fde10, "unknown_1400fde10", tpl_37::<0x143a2dd18, 0x143a2dd1b, 0x143078a50, 0x142409b50, 0x1425f4de0, 0x141834d18> as *const ()), // sub_1400fde10
    Replacement::draft(0x1400fde70, "unknown_1400fde70", tpl_45::<0x143a2df70, 0x143a2df73, 0x143078a60, 0x142409b50, 0x1425f4df0, 0x141834d18> as *const ()), // sub_1400fde70
    Replacement::draft(0x1400fded0, "unknown_1400fded0", tpl_54::<0x143a2e080, 0x143a2e083, 0x143078a70, 0x142409b50, 0x1425f4e00, 0x141834d18> as *const ()), // sub_1400fded0
    Replacement::draft(0x1400fdf30, "unknown_1400fdf30", tpl_40::<0x143a2dcf0, 0x143a2dcf3, 0x143078a88, 0x142409b50, 0x1425f4e10, 0x141834d18> as *const ()), // sub_1400fdf30
    Replacement::draft(0x1400fdf90, "unknown_1400fdf90", tpl_50::<0x143a2dd40, 0x143a2dd43, 0x143078a90, 0x142409b50, 0x1425f4e20, 0x141834d18> as *const ()), // sub_1400fdf90
    Replacement::draft(0x1400fdff0, "unknown_1400fdff0", tpl_41::<0x143a2def8, 0x143a2defb, 0x143078aa8, 0x142409b50, 0x1425f4e30, 0x141834d18> as *const ()), // sub_1400fdff0
    Replacement::draft(0x1400fe050, "unknown_1400fe050", tpl_55::<0x143a2e1e8, 0x143a2e1eb, 0x143078ab8, 0x142409b50, 0x1425f4e40, 0x141834d18> as *const ()), // sub_1400fe050
    Replacement::draft(0x1400fe0b0, "unknown_1400fe0b0", tpl_41::<0x143a2e288, 0x143a2e28b, 0x143078ac8, 0x142409b50, 0x1425f4e50, 0x141834d18> as *const ()), // sub_1400fe0b0
    Replacement::draft(0x1400fe110, "unknown_1400fe110", tpl_49::<0x143a2e328, 0x143a2e32b, 0x143078ad8, 0x142409b50, 0x1425f4e60, 0x141834d18> as *const ()), // sub_1400fe110
    Replacement::draft(0x1400fe170, "unknown_1400fe170", tpl_40::<0x143a2db40, 0x143a2db43, 0x143078ae4, 0x142409b50, 0x1425f4e70, 0x141834d18> as *const ()), // sub_1400fe170
    Replacement::draft(0x1400fe1d0, "unknown_1400fe1d0", tpl_44::<0x143a2dc78, 0x143a2dc7b, 0x143078af0, 0x142409b50, 0x1425f4e80, 0x141834d18> as *const ()), // sub_1400fe1d0
    Replacement::draft(0x1400fe230, "unknown_1400fe230", tpl_46::<0x143a2db18, 0x143a2db1b, 0x143078b08, 0x142409b50, 0x1425f4e90, 0x141834d18> as *const ()), // sub_1400fe230
    Replacement::draft(0x1400fe290, "unknown_1400fe290", tpl_55::<0x143a2e210, 0x143a2e213, 0x143078b20, 0x142409b50, 0x1425f4ea0, 0x141834d18> as *const ()), // sub_1400fe290
    Replacement::draft(0x1400fe2f0, "unknown_1400fe2f0", tpl_46::<0x143a2e0a8, 0x143a2e0ab, 0x143078b30, 0x142409b50, 0x1425f4eb0, 0x141834d18> as *const ()), // sub_1400fe2f0
    Replacement::draft(0x1400fe350, "unknown_1400fe350", tpl_50::<0x143a2de08, 0x143a2de0b, 0x143078b48, 0x142409b50, 0x1425f4ec0, 0x141834d18> as *const ()), // sub_1400fe350
    Replacement::draft(0x1400fe3b0, "unknown_1400fe3b0", tpl_42::<0x143a2dac8, 0x143a2dacb, 0x143078b60, 0x142409b50, 0x1425f4ed0, 0x141834d18> as *const ()), // sub_1400fe3b0
    Replacement::draft(0x1400fe410, "unknown_1400fe410", tpl_38::<0x143a2e170, 0x143a2e173, 0x143078b78, 0x142409b50, 0x1425f4ee0, 0x141834d18> as *const ()), // sub_1400fe410
    Replacement::draft(0x1400fe470, "unknown_1400fe470", tpl_34::<0x143a2e4e0, 0x143a2e4e3, 0x143078b88, 0x142409b50, 0x1425f4ef0, 0x141834d18> as *const ()), // sub_1400fe470
    Replacement::draft(0x1400fe4d0, "unknown_1400fe4d0", tpl_41::<0x143a2e2d8, 0x143a2e2db, 0x143078b98, 0x142409b50, 0x1425f4f00, 0x141834d18> as *const ()), // sub_1400fe4d0
    Replacement::draft(0x1400fe530, "unknown_1400fe530", tpl_36::<0x143a2e058, 0x143a2e05b, 0x143078ba8, 0x142409b50, 0x1425f4f10, 0x141834d18> as *const ()), // sub_1400fe530
    Replacement::draft(0x1400fe590, "unknown_1400fe590", tpl_34::<0x143a2daf0, 0x143a2daf3, 0x143078bb8, 0x142409b50, 0x1425f4f20, 0x141834d18> as *const ()), // sub_1400fe590
    Replacement::draft(0x1400fec80, "unknown_1400fec80", tpl_23::<0x143a2e6e8, 0x143a2e6eb, 0x1426cdcb0, 0x142409b50, 0x1425f4f40, 0x141834d18> as *const ()), // sub_1400fec80
    Replacement::draft(0x1400fece0, "unknown_1400fece0", tpl_23::<0x143a2e730, 0x143a2e733, 0x1426cdcb0, 0x142409b50, 0x1425f4f50, 0x141834d18> as *const ()), // sub_1400fece0
    Replacement::draft(0x1400feed0, "unknown_1400feed0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384c9d0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f4f90, 0x141834d18> as *const ()), // sub_1400feed0
    Replacement::draft(0x1400ff2d0, "unknown_1400ff2d0", tpl_34::<0x143a2e898, 0x143a2e89b, 0x143078668, 0x142409b50, 0x1425f5000, 0x141834d18> as *const ()), // sub_1400ff2d0
    Replacement::draft(0x1400ff330, "unknown_1400ff330", tpl_23::<0x143a2eea8, 0x143a2eeab, 0x1426cdcb0, 0x142409b50, 0x1425f5010, 0x141834d18> as *const ()), // sub_1400ff330
    Replacement::draft(0x1400ff390, "unknown_1400ff390", tpl_35::<0x143a2ebd8, 0x143a2ebdb, 0x143078678, 0x142409b50, 0x1425f5020, 0x141834d18> as *const ()), // sub_1400ff390
    Replacement::draft(0x1400ff3f0, "unknown_1400ff3f0", tpl_36::<0x143a2ee30, 0x143a2ee33, 0x143078680, 0x142409b50, 0x1425f5030, 0x141834d18> as *const ()), // sub_1400ff3f0
    Replacement::draft(0x1400ff450, "unknown_1400ff450", tpl_37::<0x143a2ea78, 0x143a2ea7b, 0x143078690, 0x142409b50, 0x1425f5040, 0x141834d18> as *const ()), // sub_1400ff450
    Replacement::draft(0x1400ff4b0, "unknown_1400ff4b0", tpl_38::<0x143a2e938, 0x143a2e93b, 0x1430786a0, 0x142409b50, 0x1425f5050, 0x141834d18> as *const ()), // sub_1400ff4b0
    Replacement::draft(0x1400ff510, "unknown_1400ff510", tpl_39::<0x143a2eed0, 0x143a2eed3, 0x1430786b0, 0x142409b50, 0x1425f5060, 0x141834d18> as *const ()), // sub_1400ff510
    Replacement::draft(0x1400ff570, "unknown_1400ff570", tpl_37::<0x143a2e848, 0x143a2e84b, 0x1430786b8, 0x142409b50, 0x1425f5070, 0x141834d18> as *const ()), // sub_1400ff570
    Replacement::draft(0x1400ff5d0, "unknown_1400ff5d0", tpl_40::<0x143a2efe8, 0x143a2efeb, 0x1430786c4, 0x142409b50, 0x1425f5080, 0x141834d18> as *const ()), // sub_1400ff5d0
    Replacement::draft(0x1400ff630, "unknown_1400ff630", tpl_41::<0x143a2ea00, 0x143a2ea03, 0x1430786d0, 0x142409b50, 0x1425f5090, 0x141834d18> as *const ()), // sub_1400ff630
    Replacement::draft(0x1400ff690, "unknown_1400ff690", tpl_42::<0x143a2ef20, 0x143a2ef23, 0x1430786e0, 0x142409b50, 0x1425f50a0, 0x141834d18> as *const ()), // sub_1400ff690
    Replacement::draft(0x1400ff6f0, "unknown_1400ff6f0", tpl_34::<0x143a2ef70, 0x143a2ef73, 0x1430786f8, 0x142409b50, 0x1425f50b0, 0x141834d18> as *const ()), // sub_1400ff6f0
    Replacement::draft(0x1400ff750, "unknown_1400ff750", tpl_34::<0x143a2e870, 0x143a2e873, 0x143078708, 0x142409b50, 0x1425f50c0, 0x141834d18> as *const ()), // sub_1400ff750
    Replacement::draft(0x1400ff7b0, "unknown_1400ff7b0", tpl_41::<0x143a2e960, 0x143a2e963, 0x143078718, 0x142409b50, 0x1425f50d0, 0x141834d18> as *const ()), // sub_1400ff7b0
    Replacement::draft(0x1400ff810, "unknown_1400ff810", tpl_43::<0x143a2eaa0, 0x143a2eaa3, 0x143078728, 0x142409b50, 0x1425f50e0, 0x141834d18> as *const ()), // sub_1400ff810
    Replacement::draft(0x1400ff870, "unknown_1400ff870", tpl_44::<0x143a2e558, 0x143a2e55b, 0x143078748, 0x142409b50, 0x1425f50f0, 0x141834d18> as *const ()), // sub_1400ff870
    Replacement::draft(0x1400ff8d0, "unknown_1400ff8d0", tpl_39::<0x143a2ea28, 0x143a2ea2b, 0x143078760, 0x142409b50, 0x1425f5100, 0x141834d18> as *const ()), // sub_1400ff8d0
    Replacement::draft(0x1400ff930, "unknown_1400ff930", tpl_45::<0x143a2e910, 0x143a2e913, 0x143078768, 0x142409b50, 0x1425f5110, 0x141834d18> as *const ()), // sub_1400ff930
    Replacement::draft(0x1400ff990, "unknown_1400ff990", tpl_41::<0x143a2ede0, 0x143a2ede3, 0x143078778, 0x142409b50, 0x1425f5120, 0x141834d18> as *const ()), // sub_1400ff990
    Replacement::draft(0x1400ff9f0, "unknown_1400ff9f0", tpl_37::<0x143a2e580, 0x143a2e583, 0x143078788, 0x142409b50, 0x1425f5130, 0x141834d18> as *const ()), // sub_1400ff9f0
    Replacement::draft(0x1400ffa50, "unknown_1400ffa50", tpl_46::<0x143a2ec78, 0x143a2ec7b, 0x143078798, 0x142409b50, 0x1425f5140, 0x141834d18> as *const ()), // sub_1400ffa50
    Replacement::draft(0x1400ffab0, "unknown_1400ffab0", tpl_47::<0x143a2e7a8, 0x143a2e7ab, 0x1430787b0, 0x142409b50, 0x1425f5150, 0x141834d18> as *const ()), // sub_1400ffab0
    Replacement::draft(0x1400ffb10, "unknown_1400ffb10", tpl_38::<0x143a2ed18, 0x143a2ed1b, 0x1430787d0, 0x142409b50, 0x1425f5160, 0x141834d18> as *const ()), // sub_1400ffb10
    Replacement::draft(0x1400ffb70, "unknown_1400ffb70", tpl_48::<0x143a2eef8, 0x143a2eefb, 0x1430787e0, 0x142409b50, 0x1425f5170, 0x141834d18> as *const ()), // sub_1400ffb70
    Replacement::draft(0x1400ffbd0, "unknown_1400ffbd0", tpl_49::<0x143a2ebb0, 0x143a2ebb3, 0x1430787f8, 0x142409b50, 0x1425f5180, 0x141834d18> as *const ()), // sub_1400ffbd0
    Replacement::draft(0x1400ffc30, "unknown_1400ffc30", tpl_44::<0x143a2ee80, 0x143a2ee83, 0x143078808, 0x142409b50, 0x1425f5190, 0x141834d18> as *const ()), // sub_1400ffc30
    Replacement::draft(0x1400ffc90, "unknown_1400ffc90", tpl_42::<0x143a2e530, 0x143a2e533, 0x143078820, 0x142409b50, 0x1425f51a0, 0x141834d18> as *const ()), // sub_1400ffc90
    Replacement::draft(0x1400ffcf0, "unknown_1400ffcf0", tpl_50::<0x143a2ed90, 0x143a2ed93, 0x143078838, 0x142409b50, 0x1425f51b0, 0x141834d18> as *const ()), // sub_1400ffcf0
    Replacement::draft(0x1400ffd50, "unknown_1400ffd50", tpl_52::<0x143a2e6c0, 0x143a2e6c3, 0x143a2efe8, 0x143a2efec, 0x143a2efeb, 0x143a2eff0, 0x142409b50, 0x1425f51c0, 0x141834d18> as *const ()), // sub_1400ffd50
    Replacement::draft(0x1400ffe10, "unknown_1400ffe10", tpl_38::<0x143a2ec00, 0x143a2ec03, 0x143078950, 0x142409b50, 0x1425f51d0, 0x141834d18> as *const ()), // sub_1400ffe10
    Replacement::draft(0x1400ffe70, "unknown_1400ffe70", tpl_53::<0x143a2e780, 0x143a2e783, 0x143078960, 0x142409b50, 0x1425f51e0, 0x141834d18> as *const ()), // sub_1400ffe70
    Replacement::draft(0x1400ffed0, "unknown_1400ffed0", tpl_45::<0x143a2e648, 0x143a2e64b, 0x143078980, 0x142409b50, 0x1425f51f0, 0x141834d18> as *const ()), // sub_1400ffed0
    Replacement::draft(0x1400fff30, "unknown_1400fff30", tpl_35::<0x143a2e698, 0x143a2e69b, 0x14307898c, 0x142409b50, 0x1425f5200, 0x141834d18> as *const ()), // sub_1400fff30
    Replacement::draft(0x1400fff90, "unknown_1400fff90", tpl_42::<0x143a2ef48, 0x143a2ef4b, 0x1430786e0, 0x142409b50, 0x1425f5210, 0x141834d18> as *const ()), // sub_1400fff90
    Replacement::draft(0x1400ffff0, "unknown_1400ffff0", tpl_38::<0x143a2ee58, 0x143a2ee5b, 0x143078998, 0x142409b50, 0x1425f5220, 0x141834d18> as *const ()), // sub_1400ffff0
    Replacement::draft(0x140100050, "unknown_140100050", tpl_38::<0x143a2e9b0, 0x143a2e9b3, 0x1430789a8, 0x142409b50, 0x1425f5230, 0x141834d18> as *const ()), // sub_140100050
    Replacement::draft(0x1401000b0, "unknown_1401000b0", tpl_41::<0x143a2eb10, 0x143a2eb13, 0x1430789b8, 0x142409b50, 0x1425f5240, 0x141834d18> as *const ()), // sub_1401000b0
    Replacement::draft(0x140100110, "unknown_140100110", tpl_34::<0x143a2ef98, 0x143a2ef9b, 0x1430789c8, 0x142409b50, 0x1425f5250, 0x141834d18> as *const ()), // sub_140100110
    Replacement::draft(0x140100170, "unknown_140100170", tpl_39::<0x143a2e670, 0x143a2e673, 0x1430789d8, 0x142409b50, 0x1425f5260, 0x141834d18> as *const ()), // sub_140100170
    Replacement::draft(0x1401001d0, "unknown_1401001d0", tpl_41::<0x143a2e988, 0x143a2e98b, 0x1430789e0, 0x142409b50, 0x1425f5270, 0x141834d18> as *const ()), // sub_1401001d0
    Replacement::draft(0x140100230, "unknown_140100230", tpl_34::<0x143a2ec28, 0x143a2ec2b, 0x1430789f0, 0x142409b50, 0x1425f5280, 0x141834d18> as *const ()), // sub_140100230
    Replacement::draft(0x140100290, "unknown_140100290", tpl_48::<0x143a2e8c0, 0x143a2e8c3, 0x143078a00, 0x142409b50, 0x1425f5290, 0x141834d18> as *const ()), // sub_140100290
    Replacement::draft(0x1401002f0, "unknown_1401002f0", tpl_46::<0x143a2ed40, 0x143a2ed43, 0x143078a18, 0x142409b50, 0x1425f52a0, 0x141834d18> as *const ()), // sub_1401002f0
    Replacement::draft(0x140100350, "unknown_140100350", tpl_47::<0x143a2eca0, 0x143a2eca3, 0x143078a30, 0x142409b50, 0x1425f52b0, 0x141834d18> as *const ()), // sub_140100350
    Replacement::draft(0x1401003b0, "unknown_1401003b0", tpl_37::<0x143a2e7f8, 0x143a2e7fb, 0x143078a50, 0x142409b50, 0x1425f52c0, 0x141834d18> as *const ()), // sub_1401003b0
    Replacement::draft(0x140100410, "unknown_140100410", tpl_45::<0x143a2ea50, 0x143a2ea53, 0x143078a60, 0x142409b50, 0x1425f52d0, 0x141834d18> as *const ()), // sub_140100410
    Replacement::draft(0x140100470, "unknown_140100470", tpl_54::<0x143a2eb60, 0x143a2eb63, 0x143078a70, 0x142409b50, 0x1425f52e0, 0x141834d18> as *const ()), // sub_140100470
    Replacement::draft(0x1401004d0, "unknown_1401004d0", tpl_40::<0x143a2e7d0, 0x143a2e7d3, 0x143078a88, 0x142409b50, 0x1425f52f0, 0x141834d18> as *const ()), // sub_1401004d0
    Replacement::draft(0x140100530, "unknown_140100530", tpl_50::<0x143a2e820, 0x143a2e823, 0x143078a90, 0x142409b50, 0x1425f5300, 0x141834d18> as *const ()), // sub_140100530
    Replacement::draft(0x140100590, "unknown_140100590", tpl_41::<0x143a2e9d8, 0x143a2e9db, 0x143078aa8, 0x142409b50, 0x1425f5310, 0x141834d18> as *const ()), // sub_140100590
    Replacement::draft(0x1401005f0, "unknown_1401005f0", tpl_55::<0x143a2ecc8, 0x143a2eccb, 0x143078ab8, 0x142409b50, 0x1425f5320, 0x141834d18> as *const ()), // sub_1401005f0
    Replacement::draft(0x140100650, "unknown_140100650", tpl_41::<0x143a2ed68, 0x143a2ed6b, 0x143078ac8, 0x142409b50, 0x1425f5330, 0x141834d18> as *const ()), // sub_140100650
    Replacement::draft(0x1401006b0, "unknown_1401006b0", tpl_49::<0x143a2ee08, 0x143a2ee0b, 0x143078ad8, 0x142409b50, 0x1425f5340, 0x141834d18> as *const ()), // sub_1401006b0
    Replacement::draft(0x140100710, "unknown_140100710", tpl_40::<0x143a2e620, 0x143a2e623, 0x143078ae4, 0x142409b50, 0x1425f5350, 0x141834d18> as *const ()), // sub_140100710
    Replacement::draft(0x140100770, "unknown_140100770", tpl_44::<0x143a2e758, 0x143a2e75b, 0x143078af0, 0x142409b50, 0x1425f5360, 0x141834d18> as *const ()), // sub_140100770
    Replacement::draft(0x1401007d0, "unknown_1401007d0", tpl_46::<0x143a2e5f8, 0x143a2e5fb, 0x143078b08, 0x142409b50, 0x1425f5370, 0x141834d18> as *const ()), // sub_1401007d0
    Replacement::draft(0x140100830, "unknown_140100830", tpl_55::<0x143a2ecf0, 0x143a2ecf3, 0x143078b20, 0x142409b50, 0x1425f5380, 0x141834d18> as *const ()), // sub_140100830
    Replacement::draft(0x140100890, "unknown_140100890", tpl_46::<0x143a2eb88, 0x143a2eb8b, 0x143078b30, 0x142409b50, 0x1425f5390, 0x141834d18> as *const ()), // sub_140100890
    Replacement::draft(0x1401008f0, "unknown_1401008f0", tpl_50::<0x143a2e8e8, 0x143a2e8eb, 0x143078b48, 0x142409b50, 0x1425f53a0, 0x141834d18> as *const ()), // sub_1401008f0
    Replacement::draft(0x140100950, "unknown_140100950", tpl_42::<0x143a2e5a8, 0x143a2e5ab, 0x143078b60, 0x142409b50, 0x1425f53b0, 0x141834d18> as *const ()), // sub_140100950
    Replacement::draft(0x1401009b0, "unknown_1401009b0", tpl_38::<0x143a2ec50, 0x143a2ec53, 0x143078b78, 0x142409b50, 0x1425f53c0, 0x141834d18> as *const ()), // sub_1401009b0
    Replacement::draft(0x140100a10, "unknown_140100a10", tpl_34::<0x143a2efc0, 0x143a2efc3, 0x143078b88, 0x142409b50, 0x1425f53d0, 0x141834d18> as *const ()), // sub_140100a10
    Replacement::draft(0x140100a70, "unknown_140100a70", tpl_41::<0x143a2edb8, 0x143a2edbb, 0x143078b98, 0x142409b50, 0x1425f53e0, 0x141834d18> as *const ()), // sub_140100a70
    Replacement::draft(0x140100ad0, "unknown_140100ad0", tpl_36::<0x143a2eb38, 0x143a2eb3b, 0x143078ba8, 0x142409b50, 0x1425f53f0, 0x141834d18> as *const ()), // sub_140100ad0
    Replacement::draft(0x140100b30, "unknown_140100b30", tpl_34::<0x143a2e5d0, 0x143a2e5d3, 0x143078bb8, 0x142409b50, 0x1425f5400, 0x141834d18> as *const ()), // sub_140100b30
    Replacement::draft(0x140100b90, "unknown_140100b90", fn_140100b90 as *const ()), // sub_140100b90
    Replacement::draft(0x140101f00, "unknown_140101f00", tpl_23::<0x143a2f010, 0x143a2f013, 0x1426cdcb0, 0x142409b50, 0x1425f54e0, 0x141834d18> as *const ()), // sub_140101f00
    Replacement::draft(0x140101f60, "unknown_140101f60", tpl_23::<0x143a2f058, 0x143a2f05b, 0x1426cdcb0, 0x142409b50, 0x1425f54f0, 0x141834d18> as *const ()), // sub_140101f60
    Replacement::draft(0x140102150, "unknown_140102150", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384caa0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f5530, 0x141834d18> as *const ()), // sub_140102150
    Replacement::draft_throws(0x140102550, "unknown_140102550", fn_140102550 as *const (), Throws::Caught), // sub_140102550
    Replacement::draft(0x140102760, "unknown_140102760", tpl_23::<0x143a2f0c0, 0x143a2f0c3, 0x1426cdcb0, 0x142409b50, 0x1425f5800, 0x141834d18> as *const ()), // sub_140102760
    Replacement::draft(0x1401027c0, "unknown_1401027c0", tpl_23::<0x143a2f108, 0x143a2f10b, 0x1426cdcb0, 0x142409b50, 0x1425f5810, 0x141834d18> as *const ()), // sub_1401027c0
    Replacement::draft(0x1401029b0, "unknown_1401029b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384cc50, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f5850, 0x141834d18> as *const ()), // sub_1401029b0
    Replacement::draft(0x140102db0, "unknown_140102db0", tpl_142::<0x143a2f150, 0x143a2f153, 0x143079988, 0x142409b50, 0x143a2f178, 0x143a2f180, 0x143a2f190, 0x143a2f1a0, 0x1425f58c0, 0x141834d18> as *const ()), // sub_140102db0
    Replacement::draft(0x140103980, "unknown_140103980", tpl_23::<0x143a2f1f0, 0x143a2f1f3, 0x1426cdcb0, 0x142409b50, 0x1425f5900, 0x141834d18> as *const ()), // sub_140103980
    Replacement::draft(0x1401039e0, "unknown_1401039e0", tpl_23::<0x143a2f238, 0x143a2f23b, 0x1426cdcb0, 0x142409b50, 0x1425f5910, 0x141834d18> as *const ()), // sub_1401039e0
    Replacement::draft(0x140103bd0, "unknown_140103bd0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384cce0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f5950, 0x141834d18> as *const ()), // sub_140103bd0
    Replacement::draft(0x140105b80, "unknown_140105b80", tpl_143::<0x143a001ac, 0x143a00290, 0x14384ceb0, 0x14384ced0, 0x14384cef0, 0x1425f63e0, 0x141834d18> as *const ()), // sub_140105b80
    Replacement::draft(0x140105c00, "unknown_140105c00", tpl_143::<0x143a001ac, 0x143a00290, 0x14384cff0, 0x14384d010, 0x14384d030, 0x1425f6400, 0x141834d18> as *const ()), // sub_140105c00
    Replacement::draft(0x140105c80, "unknown_140105c80", tpl_143::<0x143a001ac, 0x143a00290, 0x14384ce50, 0x14384ce70, 0x14384ce90, 0x1425f6420, 0x141834d18> as *const ()), // sub_140105c80
    Replacement::draft(0x140105d00, "unknown_140105d00", tpl_143::<0x143a001ac, 0x143a00290, 0x14384cf70, 0x14384cf90, 0x14384cfb0, 0x1425f6440, 0x141834d18> as *const ()), // sub_140105d00
    Replacement::draft(0x140106320, "unknown_140106320", tpl_23::<0x143a2f2a0, 0x143a2f2a3, 0x1426cdcb0, 0x142409b50, 0x1425f6490, 0x141834d18> as *const ()), // sub_140106320
    Replacement::draft(0x140106380, "unknown_140106380", tpl_23::<0x143a2f2e8, 0x143a2f2eb, 0x1426cdcb0, 0x142409b50, 0x1425f64a0, 0x141834d18> as *const ()), // sub_140106380
    Replacement::draft(0x140106570, "unknown_140106570", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d0f8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f64e0, 0x141834d18> as *const ()), // sub_140106570
    Replacement::draft(0x140107230, "unknown_140107230", tpl_23::<0x143a2f350, 0x143a2f353, 0x1426cdcb0, 0x142409b50, 0x1425f6830, 0x141834d18> as *const ()), // sub_140107230
    Replacement::draft(0x140107290, "unknown_140107290", tpl_23::<0x143a2f398, 0x143a2f39b, 0x1426cdcb0, 0x142409b50, 0x1425f6840, 0x141834d18> as *const ()), // sub_140107290
    Replacement::draft(0x140107480, "unknown_140107480", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d1f8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f6880, 0x141834d18> as *const ()), // sub_140107480
    Replacement::draft(0x140107880, "unknown_140107880", tpl_45::<0x143a2f3c0, 0x143a2f3c3, 0x1431e6108, 0x142409b50, 0x1425f68f0, 0x141834d18> as *const ()), // sub_140107880
    Replacement::draft(0x1401078e0, "unknown_1401078e0", fn_1401078e0 as *const ()), // sub_1401078e0
    Replacement::draft(0x140108470, "unknown_140108470", tpl_61::<0x143a2f640, 0x143a2f643, 0x143091fd8, 0x142409b50, 0x1425f6920, 0x141834d18> as *const ()), // sub_140108470
    Replacement::draft(0x1401084d0, "unknown_1401084d0", tpl_62::<0x143a2f643, 0x143a2f640, 0x143a2f648, 0x143a2f644, 0x143a2c968> as *const ()), // sub_1401084d0
    Replacement::draft(0x140108560, "unknown_140108560", tpl_63::<0x143a2f5f0, 0x143a2f5f3, 0x143091ff0, 0x142409b50, 0x1425f6930, 0x141834d18> as *const ()), // sub_140108560
    Replacement::draft(0x1401085c0, "unknown_1401085c0", tpl_64::<0x143a2f580, 0x143a2f583, 0x143091fa0, 0x142409b50, 0x1425f6940, 0x141834d18> as *const ()), // sub_1401085c0
    Replacement::draft(0x140108620, "unknown_140108620", tpl_65::<0x143a2f510, 0x143a2f513, 0x143091fc0, 0x142409b50, 0x1425f6950, 0x141834d18> as *const ()), // sub_140108620
    Replacement::draft(0x140108680, "unknown_140108680", tpl_62::<0x143a2f5f3, 0x143a2f5f0, 0x143a2f5f8, 0x143a2f5f4, 0x143a2cc68> as *const ()), // sub_140108680
    Replacement::draft(0x140108710, "unknown_140108710", tpl_62::<0x143a2f583, 0x143a2f580, 0x143a2f588, 0x143a2f584, 0x143a2cb88> as *const ()), // sub_140108710
    Replacement::draft(0x1401087a0, "unknown_1401087a0", tpl_62::<0x143a2f513, 0x143a2f510, 0x143a2f518, 0x143a2f514, 0x143a2cbf8> as *const ()), // sub_1401087a0
    Replacement::draft_throws(0x1401088d0, "unknown_1401088d0", fn_1401088d0 as *const (), Throws::Caught), // sub_1401088d0
    Replacement::draft(0x1401091b0, "unknown_1401091b0", tpl_61::<0x143a2f990, 0x143a2f993, 0x143091fd8, 0x142409b50, 0x1425f6990, 0x141834d18> as *const ()), // sub_1401091b0
    Replacement::draft(0x140109210, "unknown_140109210", tpl_62::<0x143a2f993, 0x143a2f990, 0x143a2f998, 0x143a2f994, 0x143a2cd28> as *const ()), // sub_140109210
    Replacement::draft(0x1401092a0, "unknown_1401092a0", tpl_63::<0x143a2f940, 0x143a2f943, 0x143091ff0, 0x142409b50, 0x1425f69a0, 0x141834d18> as *const ()), // sub_1401092a0
    Replacement::draft(0x140109300, "unknown_140109300", tpl_64::<0x143a2f890, 0x143a2f893, 0x143091fa0, 0x142409b50, 0x1425f69b0, 0x141834d18> as *const ()), // sub_140109300
    Replacement::draft(0x140109360, "unknown_140109360", tpl_65::<0x143a2f820, 0x143a2f823, 0x143091fc0, 0x142409b50, 0x1425f69c0, 0x141834d18> as *const ()), // sub_140109360
    Replacement::draft(0x1401093c0, "unknown_1401093c0", tpl_62::<0x143a2f943, 0x143a2f940, 0x143a2f948, 0x143a2f944, 0x143a2eac8> as *const ()), // sub_1401093c0
    Replacement::draft(0x140109450, "unknown_140109450", tpl_62::<0x143a2f893, 0x143a2f890, 0x143a2f898, 0x143a2f894, 0x143a2d428> as *const ()), // sub_140109450
    Replacement::draft(0x1401094e0, "unknown_1401094e0", tpl_62::<0x143a2f823, 0x143a2f820, 0x143a2f828, 0x143a2f824, 0x143a2dfe8> as *const ()), // sub_1401094e0
    Replacement::draft(0x140109570, "unknown_140109570", tpl_23::<0x143a2f7a0, 0x143a2f7a3, 0x1426cdcb0, 0x142409b50, 0x1425f69d0, 0x141834d18> as *const ()), // sub_140109570
    Replacement::draft(0x1401095d0, "unknown_1401095d0", tpl_23::<0x143a2f7e8, 0x143a2f7eb, 0x1426cdcb0, 0x142409b50, 0x1425f69e0, 0x141834d18> as *const ()), // sub_1401095d0
    Replacement::draft(0x1401097c0, "unknown_1401097c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d288, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f6a20, 0x141834d18> as *const ()), // sub_1401097c0
    Replacement::draft(0x140109c90, "unknown_140109c90", tpl_61::<0x143a2fb20, 0x143a2fb23, 0x143091fd8, 0x142409b50, 0x1425f6a90, 0x141834d18> as *const ()), // sub_140109c90
    Replacement::draft(0x140109cf0, "unknown_140109cf0", tpl_62::<0x143a2fb23, 0x143a2fb20, 0x143a2fb28, 0x143a2fb24, 0x143a2f1a8> as *const ()), // sub_140109cf0
    Replacement::draft(0x140109d80, "unknown_140109d80", tpl_63::<0x143a2fad0, 0x143a2fad3, 0x143091ff0, 0x142409b50, 0x1425f6aa0, 0x141834d18> as *const ()), // sub_140109d80
    Replacement::draft(0x140109de0, "unknown_140109de0", tpl_64::<0x143a2fa60, 0x143a2fa63, 0x143091fa0, 0x142409b50, 0x1425f6ab0, 0x141834d18> as *const ()), // sub_140109de0
    Replacement::draft(0x140109e40, "unknown_140109e40", tpl_65::<0x143a2f9f0, 0x143a2f9f3, 0x143091fc0, 0x142409b50, 0x1425f6ac0, 0x141834d18> as *const ()), // sub_140109e40
    Replacement::draft(0x140109ea0, "unknown_140109ea0", tpl_62::<0x143a2fad3, 0x143a2fad0, 0x143a2fad8, 0x143a2fad4, 0x143a2f558> as *const ()), // sub_140109ea0
    Replacement::draft(0x140109f30, "unknown_140109f30", tpl_62::<0x143a2fa63, 0x143a2fa60, 0x143a2fa68, 0x143a2fa64, 0x143a2f3e8> as *const ()), // sub_140109f30
    Replacement::draft(0x140109fc0, "unknown_140109fc0", tpl_62::<0x143a2f9f3, 0x143a2f9f0, 0x143a2f9f8, 0x143a2f9f4, 0x143a2f4f8> as *const ()), // sub_140109fc0
    Replacement::draft(0x14010a120, "unknown_14010a120", tpl_23::<0x143a2fba0, 0x143a2fba3, 0x1426cdcb0, 0x142409b50, 0x1425f6ad0, 0x141834d18> as *const ()), // sub_14010a120
    Replacement::draft(0x14010a180, "unknown_14010a180", tpl_23::<0x143a2fbe8, 0x143a2fbeb, 0x1426cdcb0, 0x142409b50, 0x1425f6ae0, 0x141834d18> as *const ()), // sub_14010a180
    Replacement::draft(0x14010a370, "unknown_14010a370", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d318, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f6b20, 0x141834d18> as *const ()), // sub_14010a370
    Replacement::draft(0x14010a840, "unknown_14010a840", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a2fde0, 0x143a2fdf0> as *const ()), // sub_14010a840
    Replacement::draft(0x14010a8b0, "unknown_14010a8b0", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a2fdb0, 0x143a2fdc0> as *const ()), // sub_14010a8b0
    Replacement::draft(0x14010a920, "unknown_14010a920", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a2fe10, 0x143a2fe20> as *const ()), // sub_14010a920
    Replacement::draft(0x14010a990, "unknown_14010a990", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a2fdc8, 0x143a2fdd8> as *const ()), // sub_14010a990
    Replacement::draft(0x14010aa00, "unknown_14010aa00", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a2fcf8, 0x143a2fd08> as *const ()), // sub_14010aa00
    Replacement::draft(0x14010aa70, "unknown_14010aa70", tpl_31::<0x1430782fc, 0x143078300, 0x143a2fd30, 0x143a2fd40> as *const ()), // sub_14010aa70
    Replacement::draft(0x14010aae0, "unknown_14010aae0", tpl_32::<0x143078308, 0x143078313, 0x143a2fce0, 0x143a2fcf0> as *const ()), // sub_14010aae0
    Replacement::draft(0x14010ab80, "unknown_14010ab80", tpl_23::<0x143a2ffe0, 0x143a2ffe3, 0x1426cdcb0, 0x142409b50, 0x1425f6b90, 0x141834d18> as *const ()), // sub_14010ab80
    Replacement::draft(0x14010abe0, "unknown_14010abe0", tpl_23::<0x143a30028, 0x143a3002b, 0x1426cdcb0, 0x142409b50, 0x1425f6ba0, 0x141834d18> as *const ()), // sub_14010abe0
    Replacement::draft(0x14010add0, "unknown_14010add0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d3b8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f6be0, 0x141834d18> as *const ()), // sub_14010add0
    Replacement::draft(0x14010b1d0, "unknown_14010b1d0", tpl_34::<0x143a30190, 0x143a30193, 0x143078668, 0x142409b50, 0x1425f6c50, 0x141834d18> as *const ()), // sub_14010b1d0
    Replacement::draft(0x14010b230, "unknown_14010b230", tpl_23::<0x143a30798, 0x143a3079b, 0x1426cdcb0, 0x142409b50, 0x1425f6c60, 0x141834d18> as *const ()), // sub_14010b230
    Replacement::draft(0x14010b290, "unknown_14010b290", tpl_35::<0x143a304c8, 0x143a304cb, 0x143078678, 0x142409b50, 0x1425f6c70, 0x141834d18> as *const ()), // sub_14010b290
    Replacement::draft(0x14010b2f0, "unknown_14010b2f0", tpl_36::<0x143a30720, 0x143a30723, 0x143078680, 0x142409b50, 0x1425f6c80, 0x141834d18> as *const ()), // sub_14010b2f0
    Replacement::draft(0x14010b350, "unknown_14010b350", tpl_37::<0x143a30370, 0x143a30373, 0x143078690, 0x142409b50, 0x1425f6c90, 0x141834d18> as *const ()), // sub_14010b350
    Replacement::draft(0x14010b3b0, "unknown_14010b3b0", tpl_38::<0x143a30230, 0x143a30233, 0x1430786a0, 0x142409b50, 0x1425f6ca0, 0x141834d18> as *const ()), // sub_14010b3b0
    Replacement::draft(0x14010b410, "unknown_14010b410", tpl_39::<0x143a307c0, 0x143a307c3, 0x1430786b0, 0x142409b50, 0x1425f6cb0, 0x141834d18> as *const ()), // sub_14010b410
    Replacement::draft(0x14010b470, "unknown_14010b470", tpl_37::<0x143a30140, 0x143a30143, 0x1430786b8, 0x142409b50, 0x1425f6cc0, 0x141834d18> as *const ()), // sub_14010b470
    Replacement::draft(0x14010b4d0, "unknown_14010b4d0", tpl_40::<0x143a308d8, 0x143a308db, 0x1430786c4, 0x142409b50, 0x1425f6cd0, 0x141834d18> as *const ()), // sub_14010b4d0
    Replacement::draft(0x14010b530, "unknown_14010b530", tpl_41::<0x143a302f8, 0x143a302fb, 0x1430786d0, 0x142409b50, 0x1425f6ce0, 0x141834d18> as *const ()), // sub_14010b530
    Replacement::draft(0x14010b590, "unknown_14010b590", tpl_42::<0x143a30810, 0x143a30813, 0x1430786e0, 0x142409b50, 0x1425f6cf0, 0x141834d18> as *const ()), // sub_14010b590
    Replacement::draft(0x14010b5f0, "unknown_14010b5f0", tpl_34::<0x143a30860, 0x143a30863, 0x1430786f8, 0x142409b50, 0x1425f6d00, 0x141834d18> as *const ()), // sub_14010b5f0
    Replacement::draft(0x14010b650, "unknown_14010b650", tpl_34::<0x143a30168, 0x143a3016b, 0x143078708, 0x142409b50, 0x1425f6d10, 0x141834d18> as *const ()), // sub_14010b650
    Replacement::draft(0x14010b6b0, "unknown_14010b6b0", tpl_41::<0x143a30258, 0x143a3025b, 0x143078718, 0x142409b50, 0x1425f6d20, 0x141834d18> as *const ()), // sub_14010b6b0
    Replacement::draft(0x14010b710, "unknown_14010b710", tpl_43::<0x143a30398, 0x143a3039b, 0x143078728, 0x142409b50, 0x1425f6d30, 0x141834d18> as *const ()), // sub_14010b710
    Replacement::draft(0x14010b770, "unknown_14010b770", tpl_44::<0x143a2fe50, 0x143a2fe53, 0x143078748, 0x142409b50, 0x1425f6d40, 0x141834d18> as *const ()), // sub_14010b770
    Replacement::draft(0x14010b7d0, "unknown_14010b7d0", tpl_39::<0x143a30320, 0x143a30323, 0x143078760, 0x142409b50, 0x1425f6d50, 0x141834d18> as *const ()), // sub_14010b7d0
    Replacement::draft(0x14010b830, "unknown_14010b830", tpl_45::<0x143a30208, 0x143a3020b, 0x143078768, 0x142409b50, 0x1425f6d60, 0x141834d18> as *const ()), // sub_14010b830
    Replacement::draft(0x14010b890, "unknown_14010b890", tpl_41::<0x143a306d0, 0x143a306d3, 0x143078778, 0x142409b50, 0x1425f6d70, 0x141834d18> as *const ()), // sub_14010b890
    Replacement::draft(0x14010b8f0, "unknown_14010b8f0", tpl_37::<0x143a2fe78, 0x143a2fe7b, 0x143078788, 0x142409b50, 0x1425f6d80, 0x141834d18> as *const ()), // sub_14010b8f0
    Replacement::draft(0x14010b950, "unknown_14010b950", tpl_46::<0x143a30568, 0x143a3056b, 0x143078798, 0x142409b50, 0x1425f6d90, 0x141834d18> as *const ()), // sub_14010b950
    Replacement::draft(0x14010b9b0, "unknown_14010b9b0", tpl_47::<0x143a300a0, 0x143a300a3, 0x1430787b0, 0x142409b50, 0x1425f6da0, 0x141834d18> as *const ()), // sub_14010b9b0
    Replacement::draft(0x14010ba10, "unknown_14010ba10", tpl_38::<0x143a30608, 0x143a3060b, 0x1430787d0, 0x142409b50, 0x1425f6db0, 0x141834d18> as *const ()), // sub_14010ba10
    Replacement::draft(0x14010ba70, "unknown_14010ba70", tpl_48::<0x143a307e8, 0x143a307eb, 0x1430787e0, 0x142409b50, 0x1425f6dc0, 0x141834d18> as *const ()), // sub_14010ba70
    Replacement::draft(0x14010bad0, "unknown_14010bad0", tpl_49::<0x143a304a0, 0x143a304a3, 0x1430787f8, 0x142409b50, 0x1425f6dd0, 0x141834d18> as *const ()), // sub_14010bad0
    Replacement::draft(0x14010bb30, "unknown_14010bb30", tpl_44::<0x143a30770, 0x143a30773, 0x143078808, 0x142409b50, 0x1425f6de0, 0x141834d18> as *const ()), // sub_14010bb30
    Replacement::draft(0x14010bb90, "unknown_14010bb90", tpl_42::<0x143a2fe28, 0x143a2fe2b, 0x143078820, 0x142409b50, 0x1425f6df0, 0x141834d18> as *const ()), // sub_14010bb90
    Replacement::draft(0x14010bbf0, "unknown_14010bbf0", tpl_50::<0x143a30680, 0x143a30683, 0x143078838, 0x142409b50, 0x1425f6e00, 0x141834d18> as *const ()), // sub_14010bbf0
    Replacement::draft(0x14010bc50, "unknown_14010bc50", tpl_52::<0x143a2ffb8, 0x143a2ffbb, 0x143a308d8, 0x143a308dc, 0x143a308db, 0x143a308e0, 0x142409b50, 0x1425f6e10, 0x141834d18> as *const ()), // sub_14010bc50
    Replacement::draft(0x14010bd10, "unknown_14010bd10", tpl_38::<0x143a304f0, 0x143a304f3, 0x143078950, 0x142409b50, 0x1425f6e20, 0x141834d18> as *const ()), // sub_14010bd10
    Replacement::draft(0x14010bd70, "unknown_14010bd70", tpl_53::<0x143a30078, 0x143a3007b, 0x143078960, 0x142409b50, 0x1425f6e30, 0x141834d18> as *const ()), // sub_14010bd70
    Replacement::draft(0x14010bdd0, "unknown_14010bdd0", tpl_45::<0x143a2ff40, 0x143a2ff43, 0x143078980, 0x142409b50, 0x1425f6e40, 0x141834d18> as *const ()), // sub_14010bdd0
    Replacement::draft(0x14010be30, "unknown_14010be30", tpl_35::<0x143a2ff90, 0x143a2ff93, 0x14307898c, 0x142409b50, 0x1425f6e50, 0x141834d18> as *const ()), // sub_14010be30
    Replacement::draft(0x14010be90, "unknown_14010be90", tpl_42::<0x143a30838, 0x143a3083b, 0x1430786e0, 0x142409b50, 0x1425f6e60, 0x141834d18> as *const ()), // sub_14010be90
    Replacement::draft(0x14010bef0, "unknown_14010bef0", tpl_38::<0x143a30748, 0x143a3074b, 0x143078998, 0x142409b50, 0x1425f6e70, 0x141834d18> as *const ()), // sub_14010bef0
    Replacement::draft(0x14010bf50, "unknown_14010bf50", tpl_38::<0x143a302a8, 0x143a302ab, 0x1430789a8, 0x142409b50, 0x1425f6e80, 0x141834d18> as *const ()), // sub_14010bf50
    Replacement::draft(0x14010bfb0, "unknown_14010bfb0", tpl_41::<0x143a30400, 0x143a30403, 0x1430789b8, 0x142409b50, 0x1425f6e90, 0x141834d18> as *const ()), // sub_14010bfb0
    Replacement::draft(0x14010c010, "unknown_14010c010", tpl_34::<0x143a30888, 0x143a3088b, 0x1430789c8, 0x142409b50, 0x1425f6ea0, 0x141834d18> as *const ()), // sub_14010c010
    Replacement::draft(0x14010c070, "unknown_14010c070", tpl_39::<0x143a2ff68, 0x143a2ff6b, 0x1430789d8, 0x142409b50, 0x1425f6eb0, 0x141834d18> as *const ()), // sub_14010c070
    Replacement::draft(0x14010c0d0, "unknown_14010c0d0", tpl_41::<0x143a30280, 0x143a30283, 0x1430789e0, 0x142409b50, 0x1425f6ec0, 0x141834d18> as *const ()), // sub_14010c0d0
    Replacement::draft(0x14010c130, "unknown_14010c130", tpl_34::<0x143a30518, 0x143a3051b, 0x1430789f0, 0x142409b50, 0x1425f6ed0, 0x141834d18> as *const ()), // sub_14010c130
    Replacement::draft(0x14010c190, "unknown_14010c190", tpl_48::<0x143a301b8, 0x143a301bb, 0x143078a00, 0x142409b50, 0x1425f6ee0, 0x141834d18> as *const ()), // sub_14010c190
    Replacement::draft(0x14010c1f0, "unknown_14010c1f0", tpl_46::<0x143a30630, 0x143a30633, 0x143078a18, 0x142409b50, 0x1425f6ef0, 0x141834d18> as *const ()), // sub_14010c1f0
    Replacement::draft(0x14010c250, "unknown_14010c250", tpl_47::<0x143a30590, 0x143a30593, 0x143078a30, 0x142409b50, 0x1425f6f00, 0x141834d18> as *const ()), // sub_14010c250
    Replacement::draft(0x14010c2b0, "unknown_14010c2b0", tpl_37::<0x143a300f0, 0x143a300f3, 0x143078a50, 0x142409b50, 0x1425f6f10, 0x141834d18> as *const ()), // sub_14010c2b0
    Replacement::draft(0x14010c310, "unknown_14010c310", tpl_45::<0x143a30348, 0x143a3034b, 0x143078a60, 0x142409b50, 0x1425f6f20, 0x141834d18> as *const ()), // sub_14010c310
    Replacement::draft(0x14010c370, "unknown_14010c370", tpl_54::<0x143a30450, 0x143a30453, 0x143078a70, 0x142409b50, 0x1425f6f30, 0x141834d18> as *const ()), // sub_14010c370
    Replacement::draft(0x14010c3d0, "unknown_14010c3d0", tpl_40::<0x143a300c8, 0x143a300cb, 0x143078a88, 0x142409b50, 0x1425f6f40, 0x141834d18> as *const ()), // sub_14010c3d0
    Replacement::draft(0x14010c430, "unknown_14010c430", tpl_50::<0x143a30118, 0x143a3011b, 0x143078a90, 0x142409b50, 0x1425f6f50, 0x141834d18> as *const ()), // sub_14010c430
    Replacement::draft(0x14010c490, "unknown_14010c490", tpl_41::<0x143a302d0, 0x143a302d3, 0x143078aa8, 0x142409b50, 0x1425f6f60, 0x141834d18> as *const ()), // sub_14010c490
    Replacement::draft(0x14010c4f0, "unknown_14010c4f0", tpl_55::<0x143a305b8, 0x143a305bb, 0x143078ab8, 0x142409b50, 0x1425f6f70, 0x141834d18> as *const ()), // sub_14010c4f0
    Replacement::draft(0x14010c550, "unknown_14010c550", tpl_41::<0x143a30658, 0x143a3065b, 0x143078ac8, 0x142409b50, 0x1425f6f80, 0x141834d18> as *const ()), // sub_14010c550
    Replacement::draft(0x14010c5b0, "unknown_14010c5b0", tpl_49::<0x143a306f8, 0x143a306fb, 0x143078ad8, 0x142409b50, 0x1425f6f90, 0x141834d18> as *const ()), // sub_14010c5b0
    Replacement::draft(0x14010c610, "unknown_14010c610", tpl_40::<0x143a2ff18, 0x143a2ff1b, 0x143078ae4, 0x142409b50, 0x1425f6fa0, 0x141834d18> as *const ()), // sub_14010c610
    Replacement::draft(0x14010c670, "unknown_14010c670", tpl_44::<0x143a30050, 0x143a30053, 0x143078af0, 0x142409b50, 0x1425f6fb0, 0x141834d18> as *const ()), // sub_14010c670
    Replacement::draft(0x14010c6d0, "unknown_14010c6d0", tpl_46::<0x143a2fef0, 0x143a2fef3, 0x143078b08, 0x142409b50, 0x1425f6fc0, 0x141834d18> as *const ()), // sub_14010c6d0
    Replacement::draft(0x14010c730, "unknown_14010c730", tpl_55::<0x143a305e0, 0x143a305e3, 0x143078b20, 0x142409b50, 0x1425f6fd0, 0x141834d18> as *const ()), // sub_14010c730
    Replacement::draft(0x14010c790, "unknown_14010c790", tpl_46::<0x143a30478, 0x143a3047b, 0x143078b30, 0x142409b50, 0x1425f6fe0, 0x141834d18> as *const ()), // sub_14010c790
    Replacement::draft(0x14010c7f0, "unknown_14010c7f0", tpl_50::<0x143a301e0, 0x143a301e3, 0x143078b48, 0x142409b50, 0x1425f6ff0, 0x141834d18> as *const ()), // sub_14010c7f0
    Replacement::draft(0x14010c850, "unknown_14010c850", tpl_42::<0x143a2fea0, 0x143a2fea3, 0x143078b60, 0x142409b50, 0x1425f7000, 0x141834d18> as *const ()), // sub_14010c850
    Replacement::draft(0x14010c8b0, "unknown_14010c8b0", tpl_38::<0x143a30540, 0x143a30543, 0x143078b78, 0x142409b50, 0x1425f7010, 0x141834d18> as *const ()), // sub_14010c8b0
    Replacement::draft(0x14010c910, "unknown_14010c910", tpl_34::<0x143a308b0, 0x143a308b3, 0x143078b88, 0x142409b50, 0x1425f7020, 0x141834d18> as *const ()), // sub_14010c910
    Replacement::draft(0x14010c970, "unknown_14010c970", tpl_41::<0x143a306a8, 0x143a306ab, 0x143078b98, 0x142409b50, 0x1425f7030, 0x141834d18> as *const ()), // sub_14010c970
    Replacement::draft(0x14010c9d0, "unknown_14010c9d0", tpl_36::<0x143a30428, 0x143a3042b, 0x143078ba8, 0x142409b50, 0x1425f7040, 0x141834d18> as *const ()), // sub_14010c9d0
    Replacement::draft(0x14010ca30, "unknown_14010ca30", tpl_34::<0x143a2fec8, 0x143a2fecb, 0x143078bb8, 0x142409b50, 0x1425f7050, 0x141834d18> as *const ()), // sub_14010ca30
    Replacement::draft(0x14010d030, "unknown_14010d030", tpl_23::<0x143a30900, 0x143a30903, 0x1426cdcb0, 0x142409b50, 0x1425f7060, 0x141834d18> as *const ()), // sub_14010d030
    Replacement::draft(0x14010d090, "unknown_14010d090", tpl_23::<0x143a30948, 0x143a3094b, 0x1426cdcb0, 0x142409b50, 0x1425f7070, 0x141834d18> as *const ()), // sub_14010d090
    Replacement::draft(0x14010d280, "unknown_14010d280", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d4d8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f70b0, 0x141834d18> as *const ()), // sub_14010d280
    Replacement::draft(0x14010e210, "unknown_14010e210", tpl_23::<0x143a309b0, 0x143a309b3, 0x1426cdcb0, 0x142409b50, 0x1425f73a0, 0x141834d18> as *const ()), // sub_14010e210
    Replacement::draft(0x14010e270, "unknown_14010e270", tpl_23::<0x143a309f8, 0x143a309fb, 0x1426cdcb0, 0x142409b50, 0x1425f73b0, 0x141834d18> as *const ()), // sub_14010e270
    Replacement::draft(0x14010e460, "unknown_14010e460", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d618, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f73f0, 0x141834d18> as *const ()), // sub_14010e460
    Replacement::draft(0x14010ee30, "unknown_14010ee30", tpl_61::<0x143a30ba0, 0x143a30ba3, 0x143091fd8, 0x142409b50, 0x1425f7580, 0x141834d18> as *const ()), // sub_14010ee30
    Replacement::draft(0x14010ee90, "unknown_14010ee90", tpl_62::<0x143a30ba3, 0x143a30ba0, 0x143a30ba8, 0x143a30ba4, 0x143a2f5c8> as *const ()), // sub_14010ee90
    Replacement::draft(0x14010ef20, "unknown_14010ef20", tpl_63::<0x143a30b50, 0x143a30b53, 0x143091ff0, 0x142409b50, 0x1425f7590, 0x141834d18> as *const ()), // sub_14010ef20
    Replacement::draft(0x14010ef80, "unknown_14010ef80", tpl_64::<0x143a30ae0, 0x143a30ae3, 0x143091fa0, 0x142409b50, 0x1425f75a0, 0x141834d18> as *const ()), // sub_14010ef80
    Replacement::draft(0x14010efe0, "unknown_14010efe0", tpl_65::<0x143a30a70, 0x143a30a73, 0x143091fc0, 0x142409b50, 0x1425f75b0, 0x141834d18> as *const ()), // sub_14010efe0
    Replacement::draft(0x14010f040, "unknown_14010f040", tpl_62::<0x143a30b53, 0x143a30b50, 0x143a30b58, 0x143a30b54, 0x143a2f868> as *const ()), // sub_14010f040
    Replacement::draft(0x14010f0d0, "unknown_14010f0d0", tpl_62::<0x143a30ae3, 0x143a30ae0, 0x143a30ae8, 0x143a30ae4, 0x143a2f638> as *const ()), // sub_14010f0d0
    Replacement::draft(0x14010f160, "unknown_14010f160", tpl_62::<0x143a30a73, 0x143a30a70, 0x143a30a78, 0x143a30a74, 0x143a2f688> as *const ()), // sub_14010f160
    Replacement::draft(0x14010f970, "unknown_14010f970", tpl_61::<0x143a30c70, 0x143a30c73, 0x143091fd8, 0x142409b50, 0x1425f7680, 0x141834d18> as *const ()), // sub_14010f970
    Replacement::draft(0x14010f9d0, "unknown_14010f9d0", tpl_62::<0x143a30c73, 0x143a30c70, 0x143a30c78, 0x143a30c74, 0x143a2f8d8> as *const ()), // sub_14010f9d0
    Replacement::draft(0x140110000, "unknown_140110000", tpl_61::<0x143a30e70, 0x143a30e73, 0x143091fd8, 0x142409b50, 0x1425f76b0, 0x141834d18> as *const ()), // sub_140110000
    Replacement::draft(0x140110060, "unknown_140110060", tpl_62::<0x143a30e73, 0x143a30e70, 0x143a30e78, 0x143a30e74, 0x143a2f988> as *const ()), // sub_140110060
    Replacement::draft(0x1401100f0, "unknown_1401100f0", tpl_63::<0x143a30e20, 0x143a30e23, 0x143091ff0, 0x142409b50, 0x1425f76c0, 0x141834d18> as *const ()), // sub_1401100f0
    Replacement::draft(0x140110150, "unknown_140110150", tpl_64::<0x143a30db0, 0x143a30db3, 0x143091fa0, 0x142409b50, 0x1425f76d0, 0x141834d18> as *const ()), // sub_140110150
    Replacement::draft(0x1401101b0, "unknown_1401101b0", tpl_65::<0x143a30d40, 0x143a30d43, 0x143091fc0, 0x142409b50, 0x1425f76e0, 0x141834d18> as *const ()), // sub_1401101b0
    Replacement::draft(0x140110210, "unknown_140110210", tpl_62::<0x143a30e23, 0x143a30e20, 0x143a30e28, 0x143a30e24, 0x143a2faa8> as *const ()), // sub_140110210
    Replacement::draft(0x1401102a0, "unknown_1401102a0", tpl_62::<0x143a30db3, 0x143a30db0, 0x143a30db8, 0x143a30db4, 0x143a2f9d8> as *const ()), // sub_1401102a0
    Replacement::draft(0x140110330, "unknown_140110330", tpl_62::<0x143a30d43, 0x143a30d40, 0x143a30d48, 0x143a30d44, 0x143a2fa38> as *const ()), // sub_140110330
    Replacement::draft(0x140110460, "unknown_140110460", tpl_23::<0x143a30cb8, 0x143a30cbb, 0x1426cdcb0, 0x142409b50, 0x1425f76f0, 0x141834d18> as *const ()), // sub_140110460
    Replacement::draft(0x1401104c0, "unknown_1401104c0", tpl_23::<0x143a30d00, 0x143a30d03, 0x1426cdcb0, 0x142409b50, 0x1425f7700, 0x141834d18> as *const ()), // sub_1401104c0
    Replacement::draft(0x140111210, "unknown_140111210", tpl_23::<0x143a30f60, 0x143a30f63, 0x1426cdcb0, 0x142409b50, 0x1425f7760, 0x141834d18> as *const ()), // sub_140111210
    Replacement::draft(0x140111270, "unknown_140111270", tpl_23::<0x143a30fa8, 0x143a30fab, 0x1426cdcb0, 0x142409b50, 0x1425f7770, 0x141834d18> as *const ()), // sub_140111270
    Replacement::draft(0x140111340, "unknown_140111340", tpl_23::<0x143a30fd0, 0x143a30fd3, 0x1426cdcb0, 0x142409b50, 0x1425f7790, 0x141834d18> as *const ()), // sub_140111340
    Replacement::draft(0x1401113a0, "unknown_1401113a0", tpl_23::<0x143a31018, 0x143a3101b, 0x1426cdcb0, 0x142409b50, 0x1425f77a0, 0x141834d18> as *const ()), // sub_1401113a0
    Replacement::draft(0x140111470, "unknown_140111470", tpl_23::<0x143a31040, 0x143a31043, 0x1426cdcb0, 0x142409b50, 0x1425f77c0, 0x141834d18> as *const ()), // sub_140111470
    Replacement::draft(0x1401114d0, "unknown_1401114d0", tpl_23::<0x143a31088, 0x143a3108b, 0x1426cdcb0, 0x142409b50, 0x1425f77d0, 0x141834d18> as *const ()), // sub_1401114d0
    Replacement::draft(0x1401116c0, "unknown_1401116c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d6e0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f7810, 0x141834d18> as *const ()), // sub_1401116c0
    Replacement::draft(0x140111ac0, "unknown_140111ac0", tpl_33::<0x143428dc0, 0x143a31120> as *const ()), // sub_140111ac0
    Replacement::draft(0x140111ae0, "unknown_140111ae0", tpl_33::<0x143428da0, 0x143a310f0> as *const ()), // sub_140111ae0
    Replacement::draft(0x140111d00, "unknown_140111d00", fn_140111d00 as *const ()), // sub_140111d00
    Replacement::draft(0x1401124f0, "unknown_1401124f0", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a311b8, 0x143a311c8> as *const ()), // sub_1401124f0
    Replacement::draft(0x140112560, "unknown_140112560", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a31188, 0x143a31198> as *const ()), // sub_140112560
    Replacement::draft(0x1401125d0, "unknown_1401125d0", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a311d0, 0x143a311e0> as *const ()), // sub_1401125d0
    Replacement::draft(0x140112640, "unknown_140112640", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a311a0, 0x143a311b0> as *const ()), // sub_140112640
    Replacement::draft(0x1401126b0, "unknown_1401126b0", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a31158, 0x143a31168> as *const ()), // sub_1401126b0
    Replacement::draft(0x140112720, "unknown_140112720", tpl_31::<0x1430782fc, 0x143078300, 0x143a31170, 0x143a31180> as *const ()), // sub_140112720
    Replacement::draft(0x140112790, "unknown_140112790", tpl_32::<0x143078308, 0x143078313, 0x143a31140, 0x143a31150> as *const ()), // sub_140112790
    Replacement::draft(0x1401129e0, "unknown_1401129e0", tpl_23::<0x143a311e8, 0x143a311eb, 0x1426cdcb0, 0x142409b50, 0x1425f7d50, 0x141834d18> as *const ()), // sub_1401129e0
    Replacement::draft(0x140112a40, "unknown_140112a40", tpl_23::<0x143a31230, 0x143a31233, 0x1426cdcb0, 0x142409b50, 0x1425f7d60, 0x141834d18> as *const ()), // sub_140112a40
    Replacement::draft(0x140112c30, "unknown_140112c30", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d868, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f7da0, 0x141834d18> as *const ()), // sub_140112c30
    Replacement::draft(0x140113240, "unknown_140113240", tpl_61::<0x143a312b0, 0x143a312b3, 0x143091fd8, 0x142409b50, 0x1425f7df0, 0x141834d18> as *const ()), // sub_140113240
    Replacement::draft(0x1401132a0, "unknown_1401132a0", tpl_62::<0x143a312b3, 0x143a312b0, 0x143a312b8, 0x143a312b4, 0x143a2fb18> as *const ()), // sub_1401132a0
    Replacement::draft(0x140113330, "unknown_140113330", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a31408, 0x143a31418> as *const ()), // sub_140113330
    Replacement::draft(0x1401133a0, "unknown_1401133a0", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a313d8, 0x143a313e8> as *const ()), // sub_1401133a0
    Replacement::draft(0x140113410, "unknown_140113410", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a314b8, 0x143a314c8> as *const ()), // sub_140113410
    Replacement::draft(0x140113480, "unknown_140113480", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a313f0, 0x143a31400> as *const ()), // sub_140113480
    Replacement::draft(0x1401134f0, "unknown_1401134f0", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a31310, 0x143a31320> as *const ()), // sub_1401134f0
    Replacement::draft(0x140113560, "unknown_140113560", tpl_31::<0x1430782fc, 0x143078300, 0x143a31378, 0x143a31388> as *const ()), // sub_140113560
    Replacement::draft(0x1401135d0, "unknown_1401135d0", tpl_32::<0x143078308, 0x143078313, 0x143a312f8, 0x143a31308> as *const ()), // sub_1401135d0
    Replacement::draft(0x140113640, "unknown_140113640", tpl_61::<0x143a31470, 0x143a31473, 0x143091fd8, 0x142409b50, 0x1425f7e00, 0x141834d18> as *const ()), // sub_140113640
    Replacement::draft(0x1401136a0, "unknown_1401136a0", tpl_62::<0x143a31473, 0x143a31470, 0x143a31478, 0x143a31474, 0x143a2fc48> as *const ()), // sub_1401136a0
    Replacement::draft(0x140113730, "unknown_140113730", tpl_63::<0x143a31420, 0x143a31423, 0x143091ff0, 0x142409b50, 0x1425f7e10, 0x141834d18> as *const ()), // sub_140113730
    Replacement::draft(0x140113790, "unknown_140113790", tpl_64::<0x143a31390, 0x143a31393, 0x143091fa0, 0x142409b50, 0x1425f7e20, 0x141834d18> as *const ()), // sub_140113790
    Replacement::draft(0x1401137f0, "unknown_1401137f0", tpl_65::<0x143a31330, 0x143a31333, 0x143091fc0, 0x142409b50, 0x1425f7e30, 0x141834d18> as *const ()), // sub_1401137f0
    Replacement::draft(0x140113850, "unknown_140113850", tpl_62::<0x143a31423, 0x143a31420, 0x143a31428, 0x143a31424, 0x143a2fdf8> as *const ()), // sub_140113850
    Replacement::draft(0x1401138e0, "unknown_1401138e0", tpl_62::<0x143a31393, 0x143a31390, 0x143a31398, 0x143a31394, 0x143a2fd48> as *const ()), // sub_1401138e0
    Replacement::draft(0x140113970, "unknown_140113970", tpl_62::<0x143a31333, 0x143a31330, 0x143a31338, 0x143a31334, 0x143a2fd98> as *const ()), // sub_140113970
    Replacement::draft(0x1401148a0, "unknown_1401148a0", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a316e8, 0x143a316f8> as *const ()), // sub_1401148a0
    Replacement::draft(0x140114910, "unknown_140114910", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a31670, 0x143a31680> as *const ()), // sub_140114910
    Replacement::draft(0x140114980, "unknown_140114980", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a317a8, 0x143a317b8> as *const ()), // sub_140114980
    Replacement::draft(0x1401149f0, "unknown_1401149f0", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a316d0, 0x143a316e0> as *const ()), // sub_1401149f0
    Replacement::draft(0x140114a60, "unknown_140114a60", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a31558, 0x143a31568> as *const ()), // sub_140114a60
    Replacement::draft(0x140114ad0, "unknown_140114ad0", tpl_31::<0x1430782fc, 0x143078300, 0x143a315e0, 0x143a315f0> as *const ()), // sub_140114ad0
    Replacement::draft(0x140114b40, "unknown_140114b40", tpl_32::<0x143078308, 0x143078313, 0x143a314d0, 0x143a314e0> as *const ()), // sub_140114b40
    Replacement::draft(0x140114bb0, "unknown_140114bb0", tpl_61::<0x143a31760, 0x143a31763, 0x143091fd8, 0x142409b50, 0x1425f81d0, 0x141834d18> as *const ()), // sub_140114bb0
    Replacement::draft(0x140114c10, "unknown_140114c10", tpl_62::<0x143a31763, 0x143a31760, 0x143a31768, 0x143a31764, 0x143a30ab8> as *const ()), // sub_140114c10
    Replacement::draft(0x140114ca0, "unknown_140114ca0", tpl_63::<0x143a31710, 0x143a31713, 0x143091ff0, 0x142409b50, 0x1425f81e0, 0x141834d18> as *const ()), // sub_140114ca0
    Replacement::draft(0x140114d00, "unknown_140114d00", tpl_64::<0x143a31610, 0x143a31613, 0x143091fa0, 0x142409b50, 0x1425f81f0, 0x141834d18> as *const ()), // sub_140114d00
    Replacement::draft(0x140114d60, "unknown_140114d60", tpl_65::<0x143a31580, 0x143a31583, 0x143091fc0, 0x142409b50, 0x1425f8200, 0x141834d18> as *const ()), // sub_140114d60
    Replacement::draft(0x140114dc0, "unknown_140114dc0", tpl_62::<0x143a31713, 0x143a31710, 0x143a31718, 0x143a31714, 0x143a30c18> as *const ()), // sub_140114dc0
    Replacement::draft(0x140114e50, "unknown_140114e50", tpl_62::<0x143a31613, 0x143a31610, 0x143a31618, 0x143a31614, 0x143a30b28> as *const ()), // sub_140114e50
    Replacement::draft(0x140114ee0, "unknown_140114ee0", tpl_62::<0x143a31583, 0x143a31580, 0x143a31588, 0x143a31584, 0x143a30b98> as *const ()), // sub_140114ee0
    Replacement::draft(0x140115010, "unknown_140115010", tpl_23::<0x143a314e8, 0x143a314eb, 0x1426cdcb0, 0x142409b50, 0x1425f8210, 0x141834d18> as *const ()), // sub_140115010
    Replacement::draft(0x140115070, "unknown_140115070", tpl_23::<0x143a31530, 0x143a31533, 0x1426cdcb0, 0x142409b50, 0x1425f8220, 0x141834d18> as *const ()), // sub_140115070
    Replacement::draft(0x140115260, "unknown_140115260", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384d9b0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f8260, 0x141834d18> as *const ()), // sub_140115260
    Replacement::draft(0x140115f20, "unknown_140115f20", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a319d8, 0x143a319e8> as *const ()), // sub_140115f20
    Replacement::draft(0x140115f90, "unknown_140115f90", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a31960, 0x143a31970> as *const ()), // sub_140115f90
    Replacement::draft(0x140116000, "unknown_140116000", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a31a98, 0x143a31aa8> as *const ()), // sub_140116000
    Replacement::draft(0x140116070, "unknown_140116070", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a319c0, 0x143a319d0> as *const ()), // sub_140116070
    Replacement::draft(0x1401160e0, "unknown_1401160e0", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a31848, 0x143a31858> as *const ()), // sub_1401160e0
    Replacement::draft(0x140116150, "unknown_140116150", tpl_31::<0x1430782fc, 0x143078300, 0x143a318d0, 0x143a318e0> as *const ()), // sub_140116150
    Replacement::draft(0x1401161c0, "unknown_1401161c0", tpl_32::<0x143078308, 0x143078313, 0x143a317c0, 0x143a317d0> as *const ()), // sub_1401161c0
    Replacement::draft(0x1401162d0, "unknown_1401162d0", tpl_61::<0x143a31a50, 0x143a31a53, 0x143091fd8, 0x142409b50, 0x1425f85c0, 0x141834d18> as *const ()), // sub_1401162d0
    Replacement::draft(0x140116330, "unknown_140116330", tpl_62::<0x143a31a53, 0x143a31a50, 0x143a31a58, 0x143a31a54, 0x143a30d28> as *const ()), // sub_140116330
    Replacement::draft(0x1401163c0, "unknown_1401163c0", tpl_63::<0x143a31a00, 0x143a31a03, 0x143091ff0, 0x142409b50, 0x1425f85d0, 0x141834d18> as *const ()), // sub_1401163c0
    Replacement::draft(0x140116420, "unknown_140116420", tpl_64::<0x143a31900, 0x143a31903, 0x143091fa0, 0x142409b50, 0x1425f85e0, 0x141834d18> as *const ()), // sub_140116420
    Replacement::draft(0x140116480, "unknown_140116480", tpl_65::<0x143a31870, 0x143a31873, 0x143091fc0, 0x142409b50, 0x1425f85f0, 0x141834d18> as *const ()), // sub_140116480
    Replacement::draft(0x1401164e0, "unknown_1401164e0", tpl_62::<0x143a31a03, 0x143a31a00, 0x143a31a08, 0x143a31a04, 0x143a30e68> as *const ()), // sub_1401164e0
    Replacement::draft(0x140116570, "unknown_140116570", tpl_62::<0x143a31903, 0x143a31900, 0x143a31908, 0x143a31904, 0x143a30d88> as *const ()), // sub_140116570
    Replacement::draft(0x140116600, "unknown_140116600", tpl_62::<0x143a31873, 0x143a31870, 0x143a31878, 0x143a31874, 0x143a30df8> as *const ()), // sub_140116600
    Replacement::draft(0x140116690, "unknown_140116690", tpl_23::<0x143a317d8, 0x143a317db, 0x1426cdcb0, 0x142409b50, 0x1425f8600, 0x141834d18> as *const ()), // sub_140116690
    Replacement::draft(0x1401166f0, "unknown_1401166f0", tpl_23::<0x143a31820, 0x143a31823, 0x1426cdcb0, 0x142409b50, 0x1425f8610, 0x141834d18> as *const ()), // sub_1401166f0
    Replacement::draft(0x1401168e0, "unknown_1401168e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384daf8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f8650, 0x141834d18> as *const ()), // sub_1401168e0
    Replacement::draft(0x140117930, "unknown_140117930", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a31bb8, 0x143a31bc8> as *const ()), // sub_140117930
    Replacement::draft(0x1401179a0, "unknown_1401179a0", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a31b88, 0x143a31b98> as *const ()), // sub_1401179a0
    Replacement::draft(0x140117a10, "unknown_140117a10", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a31c68, 0x143a31c78> as *const ()), // sub_140117a10
    Replacement::draft(0x140117a80, "unknown_140117a80", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a31ba0, 0x143a31bb0> as *const ()), // sub_140117a80
    Replacement::draft(0x140117af0, "unknown_140117af0", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a31ac8, 0x143a31ad8> as *const ()), // sub_140117af0
    Replacement::draft(0x140117b60, "unknown_140117b60", tpl_31::<0x1430782fc, 0x143078300, 0x143a31b28, 0x143a31b38> as *const ()), // sub_140117b60
    Replacement::draft(0x140117bd0, "unknown_140117bd0", tpl_32::<0x143078308, 0x143078313, 0x143a31ab0, 0x143a31ac0> as *const ()), // sub_140117bd0
    Replacement::draft(0x140117c40, "unknown_140117c40", tpl_61::<0x143a31c20, 0x143a31c23, 0x143091fd8, 0x142409b50, 0x1425f8b40, 0x141834d18> as *const ()), // sub_140117c40
    Replacement::draft(0x140117ca0, "unknown_140117ca0", tpl_62::<0x143a31c23, 0x143a31c20, 0x143a31c28, 0x143a31c24, 0x143a30eb8> as *const ()), // sub_140117ca0
    Replacement::draft(0x140117d30, "unknown_140117d30", tpl_63::<0x143a31bd0, 0x143a31bd3, 0x143091ff0, 0x142409b50, 0x1425f8b50, 0x141834d18> as *const ()), // sub_140117d30
    Replacement::draft(0x140117d90, "unknown_140117d90", tpl_64::<0x143a31b40, 0x143a31b43, 0x143091fa0, 0x142409b50, 0x1425f8b60, 0x141834d18> as *const ()), // sub_140117d90
    Replacement::draft(0x140117df0, "unknown_140117df0", tpl_65::<0x143a31ae0, 0x143a31ae3, 0x143091fc0, 0x142409b50, 0x1425f8b70, 0x141834d18> as *const ()), // sub_140117df0
    Replacement::draft(0x140117e50, "unknown_140117e50", tpl_62::<0x143a31bd3, 0x143a31bd0, 0x143a31bd8, 0x143a31bd4, 0x143a31468> as *const ()), // sub_140117e50
    Replacement::draft(0x140117ee0, "unknown_140117ee0", tpl_62::<0x143a31b43, 0x143a31b40, 0x143a31b48, 0x143a31b44, 0x143a31258> as *const ()), // sub_140117ee0
    Replacement::draft(0x140117f70, "unknown_140117f70", tpl_62::<0x143a31ae3, 0x143a31ae0, 0x143a31ae8, 0x143a31ae4, 0x143a31328> as *const ()), // sub_140117f70
    Replacement::draft(0x140118080, "unknown_140118080", fn_140118080 as *const ()), // sub_140118080
    Replacement::draft(0x140118200, "unknown_140118200", fn_140118200 as *const ()), // sub_140118200
    Replacement::draft(0x140118290, "unknown_140118290", fn_140118290 as *const ()), // sub_140118290
    Replacement::draft(0x140118320, "unknown_140118320", fn_140118320 as *const ()), // sub_140118320
    Replacement::draft(0x1401183b0, "unknown_1401183b0", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a31e98, 0x143a31ea8> as *const ()), // sub_1401183b0
    Replacement::draft(0x140118420, "unknown_140118420", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a31e20, 0x143a31e30> as *const ()), // sub_140118420
    Replacement::draft(0x140118490, "unknown_140118490", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a31f58, 0x143a31f68> as *const ()), // sub_140118490
    Replacement::draft(0x140118500, "unknown_140118500", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a31e80, 0x143a31e90> as *const ()), // sub_140118500
    Replacement::draft(0x140118570, "unknown_140118570", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a31d08, 0x143a31d18> as *const ()), // sub_140118570
    Replacement::draft(0x1401185e0, "unknown_1401185e0", tpl_31::<0x1430782fc, 0x143078300, 0x143a31d90, 0x143a31da0> as *const ()), // sub_1401185e0
    Replacement::draft(0x140118650, "unknown_140118650", tpl_32::<0x143078308, 0x143078313, 0x143a31c80, 0x143a31c90> as *const ()), // sub_140118650
    Replacement::draft(0x1401186c0, "unknown_1401186c0", tpl_61::<0x143a31f10, 0x143a31f13, 0x143091fd8, 0x142409b50, 0x1425f8cc0, 0x141834d18> as *const ()), // sub_1401186c0
    Replacement::draft(0x140118720, "unknown_140118720", tpl_62::<0x143a31f13, 0x143a31f10, 0x143a31f18, 0x143a31f14, 0x143a315c8> as *const ()), // sub_140118720
    Replacement::draft(0x1401187b0, "unknown_1401187b0", tpl_63::<0x143a31ec0, 0x143a31ec3, 0x143091ff0, 0x142409b50, 0x1425f8cd0, 0x141834d18> as *const ()), // sub_1401187b0
    Replacement::draft(0x140118810, "unknown_140118810", tpl_64::<0x143a31dc0, 0x143a31dc3, 0x143091fa0, 0x142409b50, 0x1425f8ce0, 0x141834d18> as *const ()), // sub_140118810
    Replacement::draft(0x140118870, "unknown_140118870", tpl_65::<0x143a31d30, 0x143a31d33, 0x143091fc0, 0x142409b50, 0x1425f8cf0, 0x141834d18> as *const ()), // sub_140118870
    Replacement::draft(0x1401188d0, "unknown_1401188d0", tpl_62::<0x143a31ec3, 0x143a31ec0, 0x143a31ec8, 0x143a31ec4, 0x143a31688> as *const ()), // sub_1401188d0
    Replacement::draft(0x140118960, "unknown_140118960", tpl_62::<0x143a31dc3, 0x143a31dc0, 0x143a31dc8, 0x143a31dc4, 0x143a315f8> as *const ()), // sub_140118960
    Replacement::draft(0x1401189f0, "unknown_1401189f0", tpl_62::<0x143a31d33, 0x143a31d30, 0x143a31d38, 0x143a31d34, 0x143a31658> as *const ()), // sub_1401189f0
    Replacement::draft(0x140118a80, "unknown_140118a80", tpl_23::<0x143a31c98, 0x143a31c9b, 0x1426cdcb0, 0x142409b50, 0x1425f8d00, 0x141834d18> as *const ()), // sub_140118a80
    Replacement::draft(0x140118ae0, "unknown_140118ae0", tpl_23::<0x143a31ce0, 0x143a31ce3, 0x1426cdcb0, 0x142409b50, 0x1425f8d10, 0x141834d18> as *const ()), // sub_140118ae0
    Replacement::draft(0x140118cd0, "unknown_140118cd0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384dca8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1425f8d50, 0x141834d18> as *const ()), // sub_140118cd0
    Replacement::draft(0x14011aa60, "unknown_14011aa60", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a32078, 0x143a32088> as *const ()), // sub_14011aa60
    Replacement::draft(0x14011aad0, "unknown_14011aad0", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a32048, 0x143a32058> as *const ()), // sub_14011aad0
    Replacement::draft(0x14011ab40, "unknown_14011ab40", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a32128, 0x143a32138> as *const ()), // sub_14011ab40
    Replacement::draft(0x14011abb0, "unknown_14011abb0", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a32060, 0x143a32070> as *const ()), // sub_14011abb0
    Replacement::draft(0x14011ac20, "unknown_14011ac20", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a31f88, 0x143a31f98> as *const ()), // sub_14011ac20
    Replacement::draft(0x14011ac90, "unknown_14011ac90", tpl_31::<0x1430782fc, 0x143078300, 0x143a31fe8, 0x143a31ff8> as *const ()), // sub_14011ac90
    Replacement::draft(0x14011ad00, "unknown_14011ad00", tpl_32::<0x143078308, 0x143078313, 0x143a31f70, 0x143a31f80> as *const ()), // sub_14011ad00
    Replacement::draft(0x14011ad70, "unknown_14011ad70", tpl_61::<0x143a320e0, 0x143a320e3, 0x143091fd8, 0x142409b50, 0x1425f9170, 0x141834d18> as *const ()), // sub_14011ad70
    Replacement::draft(0x14011add0, "unknown_14011add0", tpl_62::<0x143a320e3, 0x143a320e0, 0x143a320e8, 0x143a320e4, 0x143a31758> as *const ()), // sub_14011add0
    Replacement::draft(0x14011ae60, "unknown_14011ae60", tpl_63::<0x143a32090, 0x143a32093, 0x143091ff0, 0x142409b50, 0x1425f9180, 0x141834d18> as *const ()), // sub_14011ae60
    Replacement::draft(0x14011aec0, "unknown_14011aec0", tpl_64::<0x143a32000, 0x143a32003, 0x143091fa0, 0x142409b50, 0x1425f9190, 0x141834d18> as *const ()), // sub_14011aec0
    Replacement::draft(0x14011af20, "unknown_14011af20", tpl_65::<0x143a31fa0, 0x143a31fa3, 0x143091fc0, 0x142409b50, 0x1425f91a0, 0x141834d18> as *const ()), // sub_14011af20
    Replacement::draft(0x14011af80, "unknown_14011af80", tpl_62::<0x143a32093, 0x143a32090, 0x143a32098, 0x143a32094, 0x143a31948> as *const ()), // sub_14011af80
    Replacement::draft(0x14011b010, "unknown_14011b010", tpl_62::<0x143a32003, 0x143a32000, 0x143a32008, 0x143a32004, 0x143a318b8> as *const ()), // sub_14011b010
    Replacement::draft(0x14011b0a0, "unknown_14011b0a0", tpl_62::<0x143a31fa3, 0x143a31fa0, 0x143a31fa8, 0x143a31fa4, 0x143a318e8> as *const ()), // sub_14011b0a0
    Replacement::draft(0x14011b170, "unknown_14011b170", tpl_150::<0x143a001ac, 0x14384de28, 0x14384de40, 0x143a00290, 0x14384de18, 0x1425f9240, 0x141834d18> as *const ()), // sub_14011b170
    Replacement::draft(0x14011b1d0, "unknown_14011b1d0", tpl_150::<0x143a001ac, 0x14384ddc8, 0x14384dde0, 0x143a00290, 0x14384ddb8, 0x1425f92d0, 0x141834d18> as *const ()), // sub_14011b1d0
    Replacement::draft(0x14011b230, "unknown_14011b230", tpl_150::<0x143a001ac, 0x14384ddf8, 0x14384de10, 0x143a00290, 0x14384dde8, 0x1425f9360, 0x141834d18> as *const ()), // sub_14011b230
    Replacement::draft(0x14011b290, "unknown_14011b290", tpl_150::<0x143a001ac, 0x14384de58, 0x14384de70, 0x143a00290, 0x14384de48, 0x1425f93f0, 0x141834d18> as *const ()), // sub_14011b290
    Replacement::draft(0x14011bc90, "unknown_14011bc90", tpl_26::<0x1430782c4, 0x1430782c9, 0x143a322a0, 0x143a322b0> as *const ()), // sub_14011bc90
    Replacement::draft(0x14011bd00, "unknown_14011bd00", tpl_27::<0x1430782cc, 0x1430782d2, 0x143a32270, 0x143a32280> as *const ()), // sub_14011bd00
    Replacement::draft(0x14011bd70, "unknown_14011bd70", tpl_28::<0x1430782d8, 0x1430782e4, 0x143a32368, 0x143a32378> as *const ()), // sub_14011bd70
    Replacement::draft(0x14011bde0, "unknown_14011bde0", tpl_29::<0x1430782e8, 0x1430782ef, 0x143a32288, 0x143a32298> as *const ()), // sub_14011bde0
    Replacement::draft(0x14011be50, "unknown_14011be50", tpl_30::<0x1430782f0, 0x1430782f9, 0x143a32158, 0x143a32168> as *const ()), // sub_14011be50
    Replacement::draft(0x14011bec0, "unknown_14011bec0", tpl_31::<0x1430782fc, 0x143078300, 0x143a321e0, 0x143a321f0> as *const ()), // sub_14011bec0
    Replacement::draft(0x14011bf30, "unknown_14011bf30", tpl_32::<0x143078308, 0x143078313, 0x143a32140, 0x143a32150> as *const ()), // sub_14011bf30
    Replacement::draft(0x14011bfa0, "unknown_14011bfa0", tpl_61::<0x143a32320, 0x143a32323, 0x143091fd8, 0x142409b50, 0x1425f9530, 0x141834d18> as *const ()), // sub_14011bfa0
    Replacement::draft(0x14011c000, "unknown_14011c000", tpl_62::<0x143a32323, 0x143a32320, 0x143a32328, 0x143a32324, 0x143a31978> as *const ()), // sub_14011c000
    Replacement::draft(0x14011c090, "unknown_14011c090", tpl_63::<0x143a322d0, 0x143a322d3, 0x143091ff0, 0x142409b50, 0x1425f9540, 0x141834d18> as *const ()), // sub_14011c090
    Replacement::draft(0x14011c0f0, "unknown_14011c0f0", tpl_64::<0x143a32210, 0x143a32213, 0x143091fa0, 0x142409b50, 0x1425f9550, 0x141834d18> as *const ()), // sub_14011c0f0
    Replacement::draft(0x14011c150, "unknown_14011c150", tpl_65::<0x143a32180, 0x143a32183, 0x143091fc0, 0x142409b50, 0x1425f9560, 0x141834d18> as *const ()), // sub_14011c150
    Replacement::draft(0x14011c1b0, "unknown_14011c1b0", tpl_62::<0x143a322d3, 0x143a322d0, 0x143a322d8, 0x143a322d4, 0x143a31d78> as *const ()), // sub_14011c1b0
    Replacement::draft(0x14011c240, "unknown_14011c240", tpl_62::<0x143a32213, 0x143a32210, 0x143a32218, 0x143a32214, 0x143a31a48> as *const ()), // sub_14011c240
    Replacement::draft(0x14011c2d0, "unknown_14011c2d0", tpl_62::<0x143a32183, 0x143a32180, 0x143a32188, 0x143a32184, 0x143a31c18> as *const ()), // sub_14011c2d0
    Replacement::draft(0x14011c540, "unknown_14011c540", tpl_58::<0x14307aafc, 0x14320c63d, 0x143a00288, 0x143cbe300, 0x14384dee8, 0x143cbe308, 0x1406231a0, 0x143cbe330, 0x143cbe310, 0x143cbe320> as *const ()), // sub_14011c540
    Replacement::draft(0x14011c6c0, "unknown_14011c6c0", fn_14011c6c0 as *const ()), // sub_14011c6c0
    Replacement::draft(0x14011c750, "unknown_14011c750", fn_14011c750 as *const ()), // sub_14011c750
    Replacement::draft(0x14011c7e0, "unknown_14011c7e0", fn_14011c7e0 as *const ()), // sub_14011c7e0
    Replacement::draft(0x14011c910, "unknown_14011c910", tpl_61::<0x143a323d0, 0x143a323d3, 0x143091fd8, 0x142409b50, 0x1425f9860, 0x141834d18> as *const ()), // sub_14011c910
    Replacement::draft(0x14011c970, "unknown_14011c970", tpl_62::<0x143a323d3, 0x143a323d0, 0x143a323d8, 0x143a323d4, 0x143a31da8> as *const ()), // sub_14011c970
    Replacement::draft(0x14011d4f0, "unknown_14011d4f0", tpl_61::<0x143a32470, 0x143a32473, 0x143091fd8, 0x142409b50, 0x1425f98a0, 0x141834d18> as *const ()), // sub_14011d4f0
    Replacement::draft(0x14011d550, "unknown_14011d550", tpl_62::<0x143a32473, 0x143a32470, 0x143a32478, 0x143a32474, 0x143a31e08> as *const ()), // sub_14011d550
    Replacement::draft(0x14011ed00, "unknown_14011ed00", tpl_154::<0x143a32d90, 0x143a32d93, 0x143214688, 0x142409b50, 0x1425f9a80, 0x141834d18> as *const ()), // sub_14011ed00
    Replacement::draft(0x1401227d0, "unknown_1401227d0", tpl_61::<0x143a330f0, 0x143a330f3, 0x143091fd8, 0x142409b50, 0x1425f9ce0, 0x141834d18> as *const ()), // sub_1401227d0
    Replacement::draft(0x140122830, "unknown_140122830", tpl_62::<0x143a330f3, 0x143a330f0, 0x143a330f8, 0x143a330f4, 0x143a31e38> as *const ()), // sub_140122830
    Replacement::draft(0x140122f00, "unknown_140122f00", tpl_61::<0x143a33190, 0x143a33193, 0x143091fd8, 0x142409b50, 0x1425f9d10, 0x141834d18> as *const ()), // sub_140122f00
    Replacement::draft(0x140122f60, "unknown_140122f60", tpl_62::<0x143a33193, 0x143a33190, 0x143a33198, 0x143a33194, 0x143a31f08> as *const ()), // sub_140122f60
    Replacement::draft(0x1401232e0, "unknown_1401232e0", tpl_61::<0x143a333e0, 0x143a333e3, 0x143091fd8, 0x142409b50, 0x1425f9db0, 0x141834d18> as *const ()), // sub_1401232e0
    Replacement::draft(0x140123340, "unknown_140123340", tpl_62::<0x143a333e3, 0x143a333e0, 0x143a333e8, 0x143a333e4, 0x143a320d8> as *const ()), // sub_140123340
    Replacement::draft(0x140123440, "unknown_140123440", tpl_102::<0x14310a7d8, 0x1432228a0, 0x143a33330, 0x1403f9a70, 0x1425f9df0, 0x141834d18> as *const ()), // sub_140123440
    Replacement::draft(0x140123770, "unknown_140123770", tpl_61::<0x143a335f0, 0x143a335f3, 0x143091fd8, 0x142409b50, 0x1425f9e20, 0x141834d18> as *const ()), // sub_140123770
    Replacement::draft(0x1401237d0, "unknown_1401237d0", tpl_62::<0x143a335f3, 0x143a335f0, 0x143a335f8, 0x143a335f4, 0x143a321c8> as *const ()), // sub_1401237d0
    Replacement::draft(0x140124b10, "unknown_140124b10", tpl_61::<0x143a33cd0, 0x143a33cd3, 0x143091fd8, 0x142409b50, 0x1425f9ee0, 0x141834d18> as *const ()), // sub_140124b10
    Replacement::draft(0x140124b70, "unknown_140124b70", tpl_62::<0x143a33cd3, 0x143a33cd0, 0x143a33cd8, 0x143a33cd4, 0x143a321f8> as *const ()), // sub_140124b70
    Replacement::draft(0x140126610, "unknown_140126610", tpl_23::<0x143a34bc0, 0x143a34bc3, 0x1426cdcb0, 0x142409b50, 0x1425fa0a0, 0x141834d18> as *const ()), // sub_140126610
    Replacement::draft(0x140126670, "unknown_140126670", tpl_23::<0x143a34c08, 0x143a34c0b, 0x1426cdcb0, 0x142409b50, 0x1425fa0b0, 0x141834d18> as *const ()), // sub_140126670
    Replacement::draft(0x140126cd0, "unknown_140126cd0", tpl_155::<0x143a34ec0, 0x143a34ec3, 0x14322c9f8, 0x142409b50, 0x1425fa0d0, 0x141834d18> as *const ()), // sub_140126cd0
    Replacement::draft(0x140126ed0, "unknown_140126ed0", tpl_156::<0x143a35030, 0x143a35033, 0x14322d008, 0x142409b50, 0x1425fa0e0, 0x141834d18> as *const ()), // sub_140126ed0
    Replacement::draft(0x140127340, "unknown_140127340", tpl_156::<0x143a353c0, 0x143a353c3, 0x14322df30, 0x142409b50, 0x1425fa0f0, 0x141834d18> as *const ()), // sub_140127340
    Replacement::draft(0x140127c70, "unknown_140127c70", tpl_61::<0x143a359b0, 0x143a359b3, 0x143091fd8, 0x142409b50, 0x1425fa190, 0x141834d18> as *const ()), // sub_140127c70
    Replacement::draft(0x140127cd0, "unknown_140127cd0", tpl_62::<0x143a359b3, 0x143a359b0, 0x143a359b8, 0x143a359b4, 0x143a32258> as *const ()), // sub_140127cd0
    Replacement::draft(0x140128580, "unknown_140128580", tpl_61::<0x143a35a50, 0x143a35a53, 0x143091fd8, 0x142409b50, 0x1425fa1c0, 0x141834d18> as *const ()), // sub_140128580
    Replacement::draft(0x1401285e0, "unknown_1401285e0", tpl_62::<0x143a35a53, 0x143a35a50, 0x143a35a58, 0x143a35a54, 0x143a322b8> as *const ()), // sub_1401285e0
    Replacement::draft(0x140128a60, "unknown_140128a60", tpl_61::<0x143a35cc0, 0x143a35cc3, 0x143091fd8, 0x142409b50, 0x1425fa1d0, 0x141834d18> as *const ()), // sub_140128a60
    Replacement::draft(0x140128ac0, "unknown_140128ac0", tpl_62::<0x143a35cc3, 0x143a35cc0, 0x143a35cc8, 0x143a35cc4, 0x143a32318> as *const ()), // sub_140128ac0
    Replacement::draft(0x140129e90, "unknown_140129e90", tpl_61::<0x143a35e30, 0x143a35e33, 0x143091fd8, 0x142409b50, 0x1425fa260, 0x141834d18> as *const ()), // sub_140129e90
    Replacement::draft(0x140129ef0, "unknown_140129ef0", tpl_62::<0x143a35e33, 0x143a35e30, 0x143a35e38, 0x143a35e34, 0x143a32418> as *const ()), // sub_140129ef0
    Replacement::draft(0x14012b1e0, "unknown_14012b1e0", tpl_61::<0x143a36890, 0x143a36893, 0x143091fd8, 0x142409b50, 0x1425fa280, 0x141834d18> as *const ()), // sub_14012b1e0
    Replacement::draft(0x14012b240, "unknown_14012b240", tpl_62::<0x143a36893, 0x143a36890, 0x143a36898, 0x143a36894, 0x143a324b8> as *const ()), // sub_14012b240
    Replacement::draft(0x14012b4b0, "unknown_14012b4b0", tpl_61::<0x143a36a00, 0x143a36a03, 0x143091fd8, 0x142409b50, 0x1425fa320, 0x141834d18> as *const ()), // sub_14012b4b0
    Replacement::draft(0x14012b510, "unknown_14012b510", tpl_62::<0x143a36a03, 0x143a36a00, 0x143a36a08, 0x143a36a04, 0x143a32548> as *const ()), // sub_14012b510
    Replacement::draft(0x14012b6f0, "unknown_14012b6f0", tpl_61::<0x143a36ae0, 0x143a36ae3, 0x143091fd8, 0x142409b50, 0x1425fa450, 0x141834d18> as *const ()), // sub_14012b6f0
    Replacement::draft(0x14012b750, "unknown_14012b750", tpl_62::<0x143a36ae3, 0x143a36ae0, 0x143a36ae8, 0x143a36ae4, 0x143a325d8> as *const ()), // sub_14012b750
    Replacement::draft(0x14012c6a0, "unknown_14012c6a0", tpl_157::<0x143a37290, 0x143a37293, 0x14323fc78, 0x142409b50, 0x1425fa460, 0x141834d18> as *const ()), // sub_14012c6a0
    Replacement::draft(0x14012d310, "unknown_14012d310", tpl_23::<0x143a37790, 0x143a37793, 0x1426cdcb0, 0x142409b50, 0x1425fa590, 0x141834d18> as *const ()), // sub_14012d310
    Replacement::draft(0x14012d370, "unknown_14012d370", tpl_23::<0x143a377d8, 0x143a377db, 0x1426cdcb0, 0x142409b50, 0x1425fa5a0, 0x141834d18> as *const ()), // sub_14012d370
    Replacement::draft(0x14012d610, "unknown_14012d610", tpl_154::<0x143a37890, 0x143a37893, 0x143242428, 0x142409b50, 0x1425fa650, 0x141834d18> as *const ()), // sub_14012d610
    Replacement::draft(0x14012d740, "unknown_14012d740", tpl_61::<0x143a37970, 0x143a37973, 0x143091fd8, 0x142409b50, 0x1425fa660, 0x141834d18> as *const ()), // sub_14012d740
    Replacement::draft(0x14012d7a0, "unknown_14012d7a0", tpl_62::<0x143a37973, 0x143a37970, 0x143a37978, 0x143a37974, 0x143a32668> as *const ()), // sub_14012d7a0
    Replacement::draft(0x14012e1a0, "unknown_14012e1a0", tpl_158::<0x143a387ec, 0x143a383f0, 0x1424aff00, 0x14324f850, 0x143a387b8, 0x143a38428, 0x143250028, 0x143a38430> as *const ()), // sub_14012e1a0
    Replacement::draft(0x14012e760, "unknown_14012e760", tpl_154::<0x143a391d0, 0x143a391d3, 0x143242428, 0x142409b50, 0x1425fc610, 0x141834d18> as *const ()), // sub_14012e760
    Replacement::draft(0x14012e7c0, "unknown_14012e7c0", tpl_157::<0x143a39260, 0x143a39263, 0x14323fc78, 0x142409b50, 0x1425fc620, 0x141834d18> as *const ()), // sub_14012e7c0
    Replacement::draft(0x14012e820, "unknown_14012e820", tpl_156::<0x143a394d0, 0x143a394d3, 0x14322df30, 0x142409b50, 0x1425fc630, 0x141834d18> as *const ()), // sub_14012e820
    Replacement::draft(0x14012e880, "unknown_14012e880", tpl_156::<0x143a392f0, 0x143a392f3, 0x14322d008, 0x142409b50, 0x1425fc640, 0x141834d18> as *const ()), // sub_14012e880
    Replacement::draft(0x14012e8e0, "unknown_14012e8e0", tpl_155::<0x143a39380, 0x143a39383, 0x14322c9f8, 0x142409b50, 0x1425fc650, 0x141834d18> as *const ()), // sub_14012e8e0
    Replacement::draft(0x14012e940, "unknown_14012e940", tpl_154::<0x143a39410, 0x143a39413, 0x143214688, 0x142409b50, 0x1425fc660, 0x141834d18> as *const ()), // sub_14012e940
    Replacement::draft(0x14012f840, "unknown_14012f840", fn_14012f840 as *const ()), // sub_14012f840
    Replacement::draft(0x140134180, "unknown_140134180", tpl_158::<0x143a3bfec, 0x143a3bdb0, 0x1424aff00, 0x14324f850, 0x143a3bfb8, 0x143a3bde8, 0x14329f3e8, 0x143a3bdf0> as *const ()), // sub_140134180
    Replacement::draft(0x1401341d0, "unknown_1401341d0", tpl_158::<0x143a3be2c, 0x143a3d090, 0x1424aff00, 0x14324f850, 0x143a3bdf8, 0x143a3d0c8, 0x14329f418, 0x143a3d0d0> as *const ()), // sub_1401341d0
    Replacement::draft(0x140134eb0, "unknown_140134eb0", fn_140134eb0 as *const ()), // sub_140134eb0
    Replacement::draft(0x1401387a0, "unknown_1401387a0", tpl_158::<0x143a42254, 0x143a42290, 0x1424aff00, 0x14324f850, 0x143a42220, 0x143a422c8, 0x1432ebdb8, 0x143a422d0> as *const ()), // sub_1401387a0
    Replacement::draft(0x140138820, "unknown_140138820", tpl_158::<0x143a421ac, 0x143a422e0, 0x1424aff00, 0x14324f850, 0x143a42178, 0x143a42318, 0x1432ebdd8, 0x143a42320> as *const ()), // sub_140138820
    Replacement::draft(0x14013ad90, "unknown_14013ad90", tpl_23::<0x143a446d0, 0x143a446d3, 0x1426cdcb0, 0x142409b50, 0x14260c3f0, 0x141834d18> as *const ()), // sub_14013ad90
    Replacement::draft(0x14013adf0, "unknown_14013adf0", tpl_23::<0x143a446f8, 0x143a446fb, 0x1426cdcb0, 0x142409b50, 0x14260c400, 0x141834d18> as *const ()), // sub_14013adf0
    Replacement::draft(0x14013aec0, "unknown_14013aec0", tpl_23::<0x143a44740, 0x143a44743, 0x1426cdcb0, 0x142409b50, 0x14260c420, 0x141834d18> as *const ()), // sub_14013aec0
    Replacement::draft(0x14013af20, "unknown_14013af20", tpl_23::<0x143a44788, 0x143a4478b, 0x1426cdcb0, 0x142409b50, 0x14260c430, 0x141834d18> as *const ()), // sub_14013af20
    Replacement::draft(0x14013aff0, "unknown_14013aff0", tpl_23::<0x143a447b0, 0x143a447b3, 0x1426cdcb0, 0x142409b50, 0x14260c450, 0x141834d18> as *const ()), // sub_14013aff0
    Replacement::draft(0x14013b050, "unknown_14013b050", tpl_23::<0x143a447f8, 0x143a447fb, 0x1426cdcb0, 0x142409b50, 0x14260c460, 0x141834d18> as *const ()), // sub_14013b050
    Replacement::draft(0x14013b8a0, "unknown_14013b8a0", tpl_23::<0x143a44820, 0x143a44823, 0x1426cdcb0, 0x142409b50, 0x14260c4a0, 0x141834d18> as *const ()), // sub_14013b8a0
    Replacement::draft(0x14013b900, "unknown_14013b900", tpl_23::<0x143a44848, 0x143a4484b, 0x1426cdcb0, 0x142409b50, 0x14260c4b0, 0x141834d18> as *const ()), // sub_14013b900
    Replacement::draft(0x14013baf0, "unknown_14013baf0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384edf8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260c4f0, 0x141834d18> as *const ()), // sub_14013baf0
    Replacement::draft(0x14013c460, "unknown_14013c460", tpl_23::<0x143a44890, 0x143a44893, 0x1426cdcb0, 0x142409b50, 0x14260c5d0, 0x141834d18> as *const ()), // sub_14013c460
    Replacement::draft(0x14013c4c0, "unknown_14013c4c0", tpl_23::<0x143a448d8, 0x143a448db, 0x1426cdcb0, 0x142409b50, 0x14260c5e0, 0x141834d18> as *const ()), // sub_14013c4c0
    Replacement::draft(0x14013c590, "unknown_14013c590", tpl_127::<0x1438235a0, 0x143a44900, 0x140548a40, 0x14260c600, 0x141834d18> as *const ()), // sub_14013c590
    Replacement::draft(0x14013c5c0, "unknown_14013c5c0", tpl_161::<0x1438235c0, 0x143a44928, 0x140548a40, 0x14260c690, 0x141834d18> as *const ()), // sub_14013c5c0
    Replacement::draft(0x14013cdb0, "unknown_14013cdb0", tpl_23::<0x143a44950, 0x143a44953, 0x1426cdcb0, 0x142409b50, 0x14260c860, 0x141834d18> as *const ()), // sub_14013cdb0
    Replacement::draft(0x14013ce10, "unknown_14013ce10", tpl_23::<0x143a44978, 0x143a4497b, 0x1426cdcb0, 0x142409b50, 0x14260c870, 0x141834d18> as *const ()), // sub_14013ce10
    Replacement::draft(0x14013d390, "unknown_14013d390", tpl_23::<0x143a449c0, 0x143a449c3, 0x1426cdcb0, 0x142409b50, 0x14260c890, 0x141834d18> as *const ()), // sub_14013d390
    Replacement::draft(0x14013d3f0, "unknown_14013d3f0", tpl_23::<0x143a449e8, 0x143a449eb, 0x1426cdcb0, 0x142409b50, 0x14260c8a0, 0x141834d18> as *const ()), // sub_14013d3f0
    Replacement::draft(0x14013d5e0, "unknown_14013d5e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384fa70, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260c8e0, 0x141834d18> as *const ()), // sub_14013d5e0
    Replacement::draft(0x14013d970, "unknown_14013d970", tpl_23::<0x143a44a30, 0x143a44a33, 0x1426cdcb0, 0x142409b50, 0x14260c920, 0x141834d18> as *const ()), // sub_14013d970
    Replacement::draft(0x14013d9d0, "unknown_14013d9d0", tpl_23::<0x143a44a78, 0x143a44a7b, 0x1426cdcb0, 0x142409b50, 0x14260c930, 0x141834d18> as *const ()), // sub_14013d9d0
    Replacement::draft(0x14013dbc0, "unknown_14013dbc0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384fb20, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260c970, 0x141834d18> as *const ()), // sub_14013dbc0
    Replacement::draft(0x14013df50, "unknown_14013df50", tpl_23::<0x143a44aa0, 0x143a44aa3, 0x1426cdcb0, 0x142409b50, 0x14260c9b0, 0x141834d18> as *const ()), // sub_14013df50
    Replacement::draft(0x14013dfb0, "unknown_14013dfb0", tpl_23::<0x143a44ac8, 0x143a44acb, 0x1426cdcb0, 0x142409b50, 0x14260c9c0, 0x141834d18> as *const ()), // sub_14013dfb0
    Replacement::draft(0x14013e1a0, "unknown_14013e1a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384fbb0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260ca00, 0x141834d18> as *const ()), // sub_14013e1a0
    Replacement::draft(0x14013e530, "unknown_14013e530", tpl_23::<0x143a44b10, 0x143a44b13, 0x1426cdcb0, 0x142409b50, 0x14260ca40, 0x141834d18> as *const ()), // sub_14013e530
    Replacement::draft(0x14013e590, "unknown_14013e590", tpl_23::<0x143a44b38, 0x143a44b3b, 0x1426cdcb0, 0x142409b50, 0x14260ca50, 0x141834d18> as *const ()), // sub_14013e590
    Replacement::draft(0x14013e780, "unknown_14013e780", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384fc40, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260ca90, 0x141834d18> as *const ()), // sub_14013e780
    Replacement::draft(0x14013eb10, "unknown_14013eb10", tpl_71::<0x14314fc48, 0x14314fc40, 0x14314fc53, 0x14314fc4b, 0x143a44b88> as *const ()), // sub_14013eb10
    Replacement::draft(0x14013ebb0, "unknown_14013ebb0", tpl_23::<0x143a44b90, 0x143a44b93, 0x1426cdcb0, 0x142409b50, 0x14260cad0, 0x141834d18> as *const ()), // sub_14013ebb0
    Replacement::draft(0x14013ec10, "unknown_14013ec10", tpl_23::<0x143a44bb8, 0x143a44bbb, 0x1426cdcb0, 0x142409b50, 0x14260cae0, 0x141834d18> as *const ()), // sub_14013ec10
    Replacement::draft(0x14013ee00, "unknown_14013ee00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384fcd0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260cb20, 0x141834d18> as *const ()), // sub_14013ee00
    Replacement::draft(0x14013f190, "unknown_14013f190", tpl_23::<0x143a44c00, 0x143a44c03, 0x1426cdcb0, 0x142409b50, 0x14260cb60, 0x141834d18> as *const ()), // sub_14013f190
    Replacement::draft(0x14013f1f0, "unknown_14013f1f0", tpl_23::<0x143a44c48, 0x143a44c4b, 0x1426cdcb0, 0x142409b50, 0x14260cb70, 0x141834d18> as *const ()), // sub_14013f1f0
    Replacement::draft(0x14013f3e0, "unknown_14013f3e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384fd60, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260cbb0, 0x141834d18> as *const ()), // sub_14013f3e0
    Replacement::draft(0x14013fa40, "unknown_14013fa40", fn_14013fa40 as *const ()), // sub_14013fa40
    Replacement::draft(0x14013ff80, "unknown_14013ff80", fn_14013ff80 as *const ()), // sub_14013ff80
    Replacement::draft(0x140140010, "unknown_140140010", tpl_23::<0x143a44c70, 0x143a44c73, 0x1426cdcb0, 0x142409b50, 0x14260cc20, 0x141834d18> as *const ()), // sub_140140010
    Replacement::draft(0x140140070, "unknown_140140070", tpl_23::<0x143a44c98, 0x143a44c9b, 0x1426cdcb0, 0x142409b50, 0x14260cc30, 0x141834d18> as *const ()), // sub_140140070
    Replacement::draft(0x140140260, "unknown_140140260", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384fdf0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260cc70, 0x141834d18> as *const ()), // sub_140140260
    Replacement::draft(0x1401407d0, "unknown_1401407d0", tpl_23::<0x143a44ce0, 0x143a44ce3, 0x1426cdcb0, 0x142409b50, 0x14260ccb0, 0x141834d18> as *const ()), // sub_1401407d0
    Replacement::draft(0x140140830, "unknown_140140830", tpl_23::<0x143a44d08, 0x143a44d0b, 0x1426cdcb0, 0x142409b50, 0x14260ccc0, 0x141834d18> as *const ()), // sub_140140830
    Replacement::draft(0x140140a20, "unknown_140140a20", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384fe80, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260cd00, 0x141834d18> as *const ()), // sub_140140a20
    Replacement::draft(0x140140db0, "unknown_140140db0", tpl_23::<0x143a44d50, 0x143a44d53, 0x1426cdcb0, 0x142409b50, 0x14260cd40, 0x141834d18> as *const ()), // sub_140140db0
    Replacement::draft(0x140140e10, "unknown_140140e10", tpl_23::<0x143a44d78, 0x143a44d7b, 0x1426cdcb0, 0x142409b50, 0x14260cd50, 0x141834d18> as *const ()), // sub_140140e10
    Replacement::draft(0x140141000, "unknown_140141000", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384ff10, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260cd90, 0x141834d18> as *const ()), // sub_140141000
    Replacement::draft(0x140141930, "unknown_140141930", tpl_23::<0x143a44dc0, 0x143a44dc3, 0x1426cdcb0, 0x142409b50, 0x14260ce00, 0x141834d18> as *const ()), // sub_140141930
    Replacement::draft(0x140141990, "unknown_140141990", tpl_23::<0x143a44e08, 0x143a44e0b, 0x1426cdcb0, 0x142409b50, 0x14260ce10, 0x141834d18> as *const ()), // sub_140141990
    Replacement::draft(0x140141b80, "unknown_140141b80", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x14384ffa0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260ce50, 0x141834d18> as *const ()), // sub_140141b80
    Replacement::draft(0x140141f10, "unknown_140141f10", tpl_23::<0x143a44e30, 0x143a44e33, 0x1426cdcb0, 0x142409b50, 0x14260ce90, 0x141834d18> as *const ()), // sub_140141f10
    Replacement::draft(0x140141f70, "unknown_140141f70", tpl_23::<0x143a44e78, 0x143a44e7b, 0x1426cdcb0, 0x142409b50, 0x14260cea0, 0x141834d18> as *const ()), // sub_140141f70
    Replacement::draft(0x140142160, "unknown_140142160", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850030, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260cee0, 0x141834d18> as *const ()), // sub_140142160
    Replacement::draft(0x140142620, "unknown_140142620", tpl_23::<0x143a44ea0, 0x143a44ea3, 0x1426cdcb0, 0x142409b50, 0x14260cfb0, 0x141834d18> as *const ()), // sub_140142620
    Replacement::draft(0x140142680, "unknown_140142680", tpl_23::<0x143a44ee8, 0x143a44eeb, 0x1426cdcb0, 0x142409b50, 0x14260cfc0, 0x141834d18> as *const ()), // sub_140142680
    Replacement::draft(0x140142870, "unknown_140142870", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438500e0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260d000, 0x141834d18> as *const ()), // sub_140142870
    Replacement::draft(0x140142cf0, "unknown_140142cf0", tpl_164::<0x14307aafc, 0x143313d8d, 0x143a00288, 0x143cc62d0, 0x14384e7dc, 0x143cc62d8, 0x140f5edd0, 0x143cc6300, 0x143cc62e0, 0x143cc62f0> as *const ()), // sub_140142cf0
    Replacement::draft(0x140142e70, "unknown_140142e70", fn_140142e70 as *const ()), // sub_140142e70
    Replacement::draft(0x140142f00, "unknown_140142f00", tpl_23::<0x143a44f10, 0x143a44f13, 0x1426cdcb0, 0x142409b50, 0x14260d050, 0x141834d18> as *const ()), // sub_140142f00
    Replacement::draft(0x140142f60, "unknown_140142f60", tpl_23::<0x143a44f38, 0x143a44f3b, 0x1426cdcb0, 0x142409b50, 0x14260d060, 0x141834d18> as *const ()), // sub_140142f60
    Replacement::draft(0x140143150, "unknown_140143150", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850170, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260d0a0, 0x141834d18> as *const ()), // sub_140143150
    Replacement::draft(0x1401434e0, "unknown_1401434e0", tpl_23::<0x143a44f80, 0x143a44f83, 0x1426cdcb0, 0x142409b50, 0x14260d0e0, 0x141834d18> as *const ()), // sub_1401434e0
    Replacement::draft(0x140143540, "unknown_140143540", tpl_23::<0x143a44fa8, 0x143a44fab, 0x1426cdcb0, 0x142409b50, 0x14260d0f0, 0x141834d18> as *const ()), // sub_140143540
    Replacement::draft(0x140143730, "unknown_140143730", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850200, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260d130, 0x141834d18> as *const ()), // sub_140143730
    Replacement::draft(0x140144060, "unknown_140144060", tpl_23::<0x143a44ff0, 0x143a44ff3, 0x1426cdcb0, 0x142409b50, 0x14260d180, 0x141834d18> as *const ()), // sub_140144060
    Replacement::draft(0x1401440c0, "unknown_1401440c0", tpl_23::<0x143a45018, 0x143a4501b, 0x1426cdcb0, 0x142409b50, 0x14260d190, 0x141834d18> as *const ()), // sub_1401440c0
    Replacement::draft(0x140144190, "unknown_140144190", tpl_23::<0x143a45060, 0x143a45063, 0x1426cdcb0, 0x142409b50, 0x14260d1b0, 0x141834d18> as *const ()), // sub_140144190
    Replacement::draft(0x1401441f0, "unknown_1401441f0", tpl_23::<0x143a450a8, 0x143a450ab, 0x1426cdcb0, 0x142409b50, 0x14260d1c0, 0x141834d18> as *const ()), // sub_1401441f0
    Replacement::draft(0x1401443e0, "unknown_1401443e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850290, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260d200, 0x141834d18> as *const ()), // sub_1401443e0
    Replacement::draft(0x140144770, "unknown_140144770", tpl_23::<0x143a450d0, 0x143a450d3, 0x1426cdcb0, 0x142409b50, 0x14260d250, 0x141834d18> as *const ()), // sub_140144770
    Replacement::draft(0x1401447d0, "unknown_1401447d0", tpl_23::<0x143a450f8, 0x143a450fb, 0x1426cdcb0, 0x142409b50, 0x14260d260, 0x141834d18> as *const ()), // sub_1401447d0
    Replacement::draft(0x1401449c0, "unknown_1401449c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850320, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260d2a0, 0x141834d18> as *const ()), // sub_1401449c0
    Replacement::draft(0x140144d50, "unknown_140144d50", tpl_23::<0x143a45140, 0x143a45143, 0x1426cdcb0, 0x142409b50, 0x14260d2e0, 0x141834d18> as *const ()), // sub_140144d50
    Replacement::draft(0x140144db0, "unknown_140144db0", tpl_23::<0x143a45168, 0x143a4516b, 0x1426cdcb0, 0x142409b50, 0x14260d2f0, 0x141834d18> as *const ()), // sub_140144db0
    Replacement::draft(0x140144fa0, "unknown_140144fa0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438503b0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260d330, 0x141834d18> as *const ()), // sub_140144fa0
    Replacement::draft(0x140145600, "unknown_140145600", tpl_23::<0x143a451b0, 0x143a451b3, 0x1426cdcb0, 0x142409b50, 0x14260d370, 0x141834d18> as *const ()), // sub_140145600
    Replacement::draft(0x140145660, "unknown_140145660", tpl_23::<0x143a451f8, 0x143a451fb, 0x1426cdcb0, 0x142409b50, 0x14260d380, 0x141834d18> as *const ()), // sub_140145660
    Replacement::draft(0x140145f70, "unknown_140145f70", tpl_23::<0x143a45220, 0x143a45223, 0x1426cdcb0, 0x142409b50, 0x14260d5a0, 0x141834d18> as *const ()), // sub_140145f70
    Replacement::draft(0x140145fd0, "unknown_140145fd0", tpl_23::<0x143a45268, 0x143a4526b, 0x1426cdcb0, 0x142409b50, 0x14260d5b0, 0x141834d18> as *const ()), // sub_140145fd0
    Replacement::draft(0x140149b50, "unknown_140149b50", tpl_23::<0x143a452b0, 0x143a452b3, 0x1426cdcb0, 0x142409b50, 0x14260d7f0, 0x141834d18> as *const ()), // sub_140149b50
    Replacement::draft(0x140149bb0, "unknown_140149bb0", tpl_23::<0x143a452d8, 0x143a452db, 0x1426cdcb0, 0x142409b50, 0x14260d800, 0x141834d18> as *const ()), // sub_140149bb0
    Replacement::draft(0x14014a2f0, "unknown_14014a2f0", tpl_166::<0x1426456f8, 0x143c48480, 0x142406920, 0x143c48488> as *const ()), // sub_14014a2f0
    Replacement::draft(0x14014ad90, "unknown_14014ad90", tpl_23::<0x143a45320, 0x143a45323, 0x1426cdcb0, 0x142409b50, 0x14260d8c0, 0x141834d18> as *const ()), // sub_14014ad90
    Replacement::draft(0x14014adf0, "unknown_14014adf0", tpl_23::<0x143a45368, 0x143a4536b, 0x1426cdcb0, 0x142409b50, 0x14260d8d0, 0x141834d18> as *const ()), // sub_14014adf0
    Replacement::draft(0x14014afe0, "unknown_14014afe0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850618, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260d910, 0x141834d18> as *const ()), // sub_14014afe0
    Replacement::draft(0x14014b550, "unknown_14014b550", tpl_23::<0x143a45390, 0x143a45393, 0x1426cdcb0, 0x142409b50, 0x14260d950, 0x141834d18> as *const ()), // sub_14014b550
    Replacement::draft(0x14014b5b0, "unknown_14014b5b0", tpl_23::<0x143a453b8, 0x143a453bb, 0x1426cdcb0, 0x142409b50, 0x14260d960, 0x141834d18> as *const ()), // sub_14014b5b0
    Replacement::draft(0x14014b7a0, "unknown_14014b7a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438506a8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260d9a0, 0x141834d18> as *const ()), // sub_14014b7a0
    Replacement::draft(0x14014d420, "unknown_14014d420", tpl_23::<0x143a45400, 0x143a45403, 0x1426cdcb0, 0x142409b50, 0x14260dca0, 0x141834d18> as *const ()), // sub_14014d420
    Replacement::draft(0x14014d480, "unknown_14014d480", tpl_23::<0x143a45448, 0x143a4544b, 0x1426cdcb0, 0x142409b50, 0x14260dcb0, 0x141834d18> as *const ()), // sub_14014d480
    Replacement::draft(0x14014d670, "unknown_14014d670", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438507f0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260dcf0, 0x141834d18> as *const ()), // sub_14014d670
    Replacement::draft(0x14014dbe0, "unknown_14014dbe0", tpl_23::<0x143a45470, 0x143a45473, 0x1426cdcb0, 0x142409b50, 0x14260dd30, 0x141834d18> as *const ()), // sub_14014dbe0
    Replacement::draft(0x14014dc40, "unknown_14014dc40", tpl_23::<0x143a454b8, 0x143a454bb, 0x1426cdcb0, 0x142409b50, 0x14260dd40, 0x141834d18> as *const ()), // sub_14014dc40
    Replacement::draft(0x14014de30, "unknown_14014de30", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850890, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260dd80, 0x141834d18> as *const ()), // sub_14014de30
    Replacement::draft(0x14014e490, "unknown_14014e490", tpl_23::<0x143a454e0, 0x143a454e3, 0x1426cdcb0, 0x142409b50, 0x14260ddc0, 0x141834d18> as *const ()), // sub_14014e490
    Replacement::draft(0x14014e4f0, "unknown_14014e4f0", tpl_23::<0x143a45508, 0x143a4550b, 0x1426cdcb0, 0x142409b50, 0x14260ddd0, 0x141834d18> as *const ()), // sub_14014e4f0
    Replacement::draft(0x14014e6e0, "unknown_14014e6e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850920, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260de10, 0x141834d18> as *const ()), // sub_14014e6e0
    Replacement::draft(0x14014ef20, "unknown_14014ef20", tpl_23::<0x143a45550, 0x143a45553, 0x1426cdcb0, 0x142409b50, 0x14260de50, 0x141834d18> as *const ()), // sub_14014ef20
    Replacement::draft(0x14014ef80, "unknown_14014ef80", tpl_23::<0x143a45578, 0x143a4557b, 0x1426cdcb0, 0x142409b50, 0x14260de60, 0x141834d18> as *const ()), // sub_14014ef80
    Replacement::draft(0x140150a60, "unknown_140150a60", tpl_23::<0x143a455c0, 0x143a455c3, 0x1426cdcb0, 0x142409b50, 0x14260df40, 0x141834d18> as *const ()), // sub_140150a60
    Replacement::draft(0x140150ac0, "unknown_140150ac0", tpl_23::<0x143a45608, 0x143a4560b, 0x1426cdcb0, 0x142409b50, 0x14260df50, 0x141834d18> as *const ()), // sub_140150ac0
    Replacement::draft(0x140150cb0, "unknown_140150cb0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850a00, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260df90, 0x141834d18> as *const ()), // sub_140150cb0
    Replacement::draft(0x140151080, "unknown_140151080", fn_140151080 as *const ()), // sub_140151080
    Replacement::draft(0x140151200, "unknown_140151200", fn_140151200 as *const ()), // sub_140151200
    Replacement::draft(0x1401513c0, "unknown_1401513c0", tpl_23::<0x143a45678, 0x143a4567b, 0x1426cdcb0, 0x142409b50, 0x14260e090, 0x141834d18> as *const ()), // sub_1401513c0
    Replacement::draft(0x140151420, "unknown_140151420", tpl_23::<0x143a456a0, 0x143a456a3, 0x1426cdcb0, 0x142409b50, 0x14260e0a0, 0x141834d18> as *const ()), // sub_140151420
    Replacement::draft(0x1401514f0, "unknown_1401514f0", tpl_23::<0x143a456e8, 0x143a456eb, 0x1426cdcb0, 0x142409b50, 0x14260e0c0, 0x141834d18> as *const ()), // sub_1401514f0
    Replacement::draft(0x140151550, "unknown_140151550", tpl_23::<0x143a45710, 0x143a45713, 0x1426cdcb0, 0x142409b50, 0x14260e0d0, 0x141834d18> as *const ()), // sub_140151550
    Replacement::draft(0x140151620, "unknown_140151620", tpl_23::<0x143a45758, 0x143a4575b, 0x1426cdcb0, 0x142409b50, 0x14260e0f0, 0x141834d18> as *const ()), // sub_140151620
    Replacement::draft(0x140151680, "unknown_140151680", tpl_23::<0x143a457a0, 0x143a457a3, 0x1426cdcb0, 0x142409b50, 0x14260e100, 0x141834d18> as *const ()), // sub_140151680
    Replacement::draft(0x140151870, "unknown_140151870", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850aa8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260e140, 0x141834d18> as *const ()), // sub_140151870
    Replacement::draft(0x140151c00, "unknown_140151c00", tpl_23::<0x143a457c8, 0x143a457cb, 0x1426cdcb0, 0x142409b50, 0x14260e180, 0x141834d18> as *const ()), // sub_140151c00
    Replacement::draft(0x140151c60, "unknown_140151c60", tpl_23::<0x143a457f0, 0x143a457f3, 0x1426cdcb0, 0x142409b50, 0x14260e190, 0x141834d18> as *const ()), // sub_140151c60
    Replacement::draft(0x140152f00, "unknown_140152f00", tpl_23::<0x143a45838, 0x143a4583b, 0x1426cdcb0, 0x142409b50, 0x14260e270, 0x141834d18> as *const ()), // sub_140152f00
    Replacement::draft(0x140152f60, "unknown_140152f60", tpl_23::<0x143a45860, 0x143a45863, 0x1426cdcb0, 0x142409b50, 0x14260e280, 0x141834d18> as *const ()), // sub_140152f60
    Replacement::draft(0x140155b80, "unknown_140155b80", tpl_23::<0x143a458e0, 0x143a458e3, 0x1426cdcb0, 0x142409b50, 0x14260ebd0, 0x141834d18> as *const ()), // sub_140155b80
    Replacement::draft(0x140155be0, "unknown_140155be0", tpl_23::<0x143a45908, 0x143a4590b, 0x1426cdcb0, 0x142409b50, 0x14260ebe0, 0x141834d18> as *const ()), // sub_140155be0
    Replacement::draft(0x1401570e0, "unknown_1401570e0", tpl_23::<0x143a45950, 0x143a45953, 0x1426cdcb0, 0x142409b50, 0x14260edb0, 0x141834d18> as *const ()), // sub_1401570e0
    Replacement::draft(0x140157140, "unknown_140157140", tpl_23::<0x143a45978, 0x143a4597b, 0x1426cdcb0, 0x142409b50, 0x14260edc0, 0x141834d18> as *const ()), // sub_140157140
    Replacement::draft(0x140157330, "unknown_140157330", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850cf0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260ee00, 0x141834d18> as *const ()), // sub_140157330
    Replacement::draft(0x140158990, "unknown_140158990", tpl_23::<0x143a459c0, 0x143a459c3, 0x1426cdcb0, 0x142409b50, 0x14260f100, 0x141834d18> as *const ()), // sub_140158990
    Replacement::draft(0x1401589f0, "unknown_1401589f0", tpl_23::<0x143a459e8, 0x143a459eb, 0x1426cdcb0, 0x142409b50, 0x14260f110, 0x141834d18> as *const ()), // sub_1401589f0
    Replacement::draft(0x140159060, "unknown_140159060", tpl_23::<0x143a45a30, 0x143a45a33, 0x1426cdcb0, 0x142409b50, 0x14260f150, 0x141834d18> as *const ()), // sub_140159060
    Replacement::draft(0x1401590c0, "unknown_1401590c0", tpl_23::<0x143a45a58, 0x143a45a5b, 0x1426cdcb0, 0x142409b50, 0x14260f160, 0x141834d18> as *const ()), // sub_1401590c0
    Replacement::draft(0x1401592b0, "unknown_1401592b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143850df8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260f1a0, 0x141834d18> as *const ()), // sub_1401592b0
    Replacement::draft(0x14015ac60, "unknown_14015ac60", tpl_23::<0x143a45aa0, 0x143a45aa3, 0x1426cdcb0, 0x142409b50, 0x14260f660, 0x141834d18> as *const ()), // sub_14015ac60
    Replacement::draft(0x14015acc0, "unknown_14015acc0", tpl_23::<0x143a45ae8, 0x143a45aeb, 0x1426cdcb0, 0x142409b50, 0x14260f670, 0x141834d18> as *const ()), // sub_14015acc0
    Replacement::draft(0x14015add0, "unknown_14015add0", tpl_23::<0x143a45b10, 0x143a45b13, 0x1426cdcb0, 0x142409b50, 0x14260f6a0, 0x141834d18> as *const ()), // sub_14015add0
    Replacement::draft(0x14015ae30, "unknown_14015ae30", tpl_23::<0x143a45b38, 0x143a45b3b, 0x1426cdcb0, 0x142409b50, 0x14260f6b0, 0x141834d18> as *const ()), // sub_14015ae30
    Replacement::draft(0x14015b990, "unknown_14015b990", tpl_23::<0x143a45b80, 0x143a45b83, 0x1426cdcb0, 0x142409b50, 0x14260f770, 0x141834d18> as *const ()), // sub_14015b990
    Replacement::draft(0x14015b9f0, "unknown_14015b9f0", tpl_23::<0x143a45ba8, 0x143a45bab, 0x1426cdcb0, 0x142409b50, 0x14260f780, 0x141834d18> as *const ()), // sub_14015b9f0
    Replacement::draft(0x14015d7a0, "unknown_14015d7a0", tpl_121::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143ccfbd0, 0x14260fbc0, 0x141834d18> as *const ()), // sub_14015d7a0
    Replacement::draft(0x14015d840, "unknown_14015d840", tpl_121::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143ccfbe0, 0x14260fa60, 0x141834d18> as *const ()), // sub_14015d840
    Replacement::draft(0x14015d9d0, "unknown_14015d9d0", tpl_23::<0x143a45bf0, 0x143a45bf3, 0x1426cdcb0, 0x142409b50, 0x14260fd10, 0x141834d18> as *const ()), // sub_14015d9d0
    Replacement::draft(0x14015da30, "unknown_14015da30", tpl_23::<0x143a45c18, 0x143a45c1b, 0x1426cdcb0, 0x142409b50, 0x14260fd20, 0x141834d18> as *const ()), // sub_14015da30
    Replacement::draft(0x14015db30, "unknown_14015db30", tpl_23::<0x143a45c98, 0x143a45c9b, 0x1426cdcb0, 0x142409b50, 0x14260fd40, 0x141834d18> as *const ()), // sub_14015db30
    Replacement::draft(0x14015db90, "unknown_14015db90", tpl_23::<0x143a45cc0, 0x143a45cc3, 0x1426cdcb0, 0x142409b50, 0x14260fd50, 0x141834d18> as *const ()), // sub_14015db90
    Replacement::draft(0x14015dc90, "unknown_14015dc90", tpl_23::<0x143a45d40, 0x143a45d43, 0x1426cdcb0, 0x142409b50, 0x14260fd70, 0x141834d18> as *const ()), // sub_14015dc90
    Replacement::draft(0x14015dcf0, "unknown_14015dcf0", tpl_23::<0x143a45d68, 0x143a45d6b, 0x1426cdcb0, 0x142409b50, 0x14260fd80, 0x141834d18> as *const ()), // sub_14015dcf0
    Replacement::draft(0x14015ddf0, "unknown_14015ddf0", tpl_23::<0x143a45de8, 0x143a45deb, 0x1426cdcb0, 0x142409b50, 0x14260fda0, 0x141834d18> as *const ()), // sub_14015ddf0
    Replacement::draft(0x14015de50, "unknown_14015de50", tpl_23::<0x143a45e10, 0x143a45e13, 0x1426cdcb0, 0x142409b50, 0x14260fdb0, 0x141834d18> as *const ()), // sub_14015de50
    Replacement::draft(0x14015df50, "unknown_14015df50", tpl_23::<0x143a45e90, 0x143a45e93, 0x1426cdcb0, 0x142409b50, 0x14260fdd0, 0x141834d18> as *const ()), // sub_14015df50
    Replacement::draft(0x14015dfb0, "unknown_14015dfb0", tpl_23::<0x143a45eb8, 0x143a45ebb, 0x1426cdcb0, 0x142409b50, 0x14260fde0, 0x141834d18> as *const ()), // sub_14015dfb0
    Replacement::draft(0x14015e1a0, "unknown_14015e1a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143851b40, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14260fe20, 0x141834d18> as *const ()), // sub_14015e1a0
    Replacement::draft(0x14015e560, "unknown_14015e560", tpl_23::<0x143a45f38, 0x143a45f3b, 0x1426cdcb0, 0x142409b50, 0x14260fe60, 0x141834d18> as *const ()), // sub_14015e560
    Replacement::draft(0x14015e5c0, "unknown_14015e5c0", tpl_23::<0x143a45f60, 0x143a45f63, 0x1426cdcb0, 0x142409b50, 0x14260fe70, 0x141834d18> as *const ()), // sub_14015e5c0
    Replacement::draft(0x14015e6f0, "unknown_14015e6f0", tpl_23::<0x143a46018, 0x143a4601b, 0x1426cdcb0, 0x142409b50, 0x14260fe90, 0x141834d18> as *const ()), // sub_14015e6f0
    Replacement::draft(0x14015e750, "unknown_14015e750", tpl_23::<0x143a46060, 0x143a46063, 0x1426cdcb0, 0x142409b50, 0x14260fea0, 0x141834d18> as *const ()), // sub_14015e750
    Replacement::draft(0x14015f1a0, "unknown_14015f1a0", tpl_23::<0x143a46088, 0x143a4608b, 0x1426cdcb0, 0x142409b50, 0x14260ff00, 0x141834d18> as *const ()), // sub_14015f1a0
    Replacement::draft(0x14015f200, "unknown_14015f200", tpl_23::<0x143a460d0, 0x143a460d3, 0x1426cdcb0, 0x142409b50, 0x14260ff10, 0x141834d18> as *const ()), // sub_14015f200
    Replacement::draft(0x14015f2d0, "unknown_14015f2d0", tpl_23::<0x143a460f8, 0x143a460fb, 0x1426cdcb0, 0x142409b50, 0x14260ff30, 0x141834d18> as *const ()), // sub_14015f2d0
    Replacement::draft(0x14015f330, "unknown_14015f330", tpl_23::<0x143a46120, 0x143a46123, 0x1426cdcb0, 0x142409b50, 0x14260ff40, 0x141834d18> as *const ()), // sub_14015f330
    Replacement::draft(0x14015fde0, "unknown_14015fde0", tpl_23::<0x143a46168, 0x143a4616b, 0x1426cdcb0, 0x142409b50, 0x1426100a0, 0x141834d18> as *const ()), // sub_14015fde0
    Replacement::draft(0x14015fe40, "unknown_14015fe40", tpl_23::<0x143a46190, 0x143a46193, 0x1426cdcb0, 0x142409b50, 0x1426100b0, 0x141834d18> as *const ()), // sub_14015fe40
    Replacement::draft(0x14015ff10, "unknown_14015ff10", tpl_23::<0x143a461d8, 0x143a461db, 0x1426cdcb0, 0x142409b50, 0x1426100d0, 0x141834d18> as *const ()), // sub_14015ff10
    Replacement::draft(0x14015ff70, "unknown_14015ff70", tpl_23::<0x143a46200, 0x143a46203, 0x1426cdcb0, 0x142409b50, 0x1426100e0, 0x141834d18> as *const ()), // sub_14015ff70
    Replacement::draft(0x1401607c0, "unknown_1401607c0", tpl_23::<0x143a46248, 0x143a4624b, 0x1426cdcb0, 0x142409b50, 0x142610110, 0x141834d18> as *const ()), // sub_1401607c0
    Replacement::draft(0x140160820, "unknown_140160820", tpl_23::<0x143a46270, 0x143a46273, 0x1426cdcb0, 0x142409b50, 0x142610120, 0x141834d18> as *const ()), // sub_140160820
    Replacement::draft_throws(0x1401626e0, "unknown_1401626e0", tpl_169::<0x1438528e0, 0x143852904, 0x143852907, 0x143852998, 0x14101ff20, 0x1438529b0, 0x1438528f0, 0x143426fc8, 0x1438528f4, 0x1438528e8, 0x1438528f8, 0x1426105c0, 0x141834d18> as *const (), Throws::Discard), // sub_1401626e0
    Replacement::draft_throws(0x1401627e0, "unknown_1401627e0", tpl_169::<0x143853570, 0x143853594, 0x143853597, 0x143853628, 0x14101ff20, 0x143853640, 0x143853580, 0x143426fc8, 0x143853584, 0x143853578, 0x143853588, 0x1426105f0, 0x141834d18> as *const (), Throws::Discard), // sub_1401627e0
    Replacement::draft(0x1401628d0, "unknown_1401628d0", tpl_23::<0x143a462b8, 0x143a462bb, 0x1426cdcb0, 0x142409b50, 0x142610610, 0x141834d18> as *const ()), // sub_1401628d0
    Replacement::draft(0x140162930, "unknown_140162930", tpl_23::<0x143a462e0, 0x143a462e3, 0x1426cdcb0, 0x142409b50, 0x142610620, 0x141834d18> as *const ()), // sub_140162930
    Replacement::draft(0x140162a00, "unknown_140162a00", tpl_23::<0x143a46328, 0x143a4632b, 0x1426cdcb0, 0x142409b50, 0x142610640, 0x141834d18> as *const ()), // sub_140162a00
    Replacement::draft(0x140162a60, "unknown_140162a60", tpl_23::<0x143a46350, 0x143a46353, 0x1426cdcb0, 0x142409b50, 0x142610650, 0x141834d18> as *const ()), // sub_140162a60
    Replacement::draft(0x1401632b0, "unknown_1401632b0", tpl_23::<0x143a46398, 0x143a4639b, 0x1426cdcb0, 0x142409b50, 0x1426106a0, 0x141834d18> as *const ()), // sub_1401632b0
    Replacement::draft(0x140163310, "unknown_140163310", tpl_23::<0x143a463e0, 0x143a463e3, 0x1426cdcb0, 0x142409b50, 0x1426106b0, 0x141834d18> as *const ()), // sub_140163310
    Replacement::draft(0x1401633e0, "unknown_1401633e0", tpl_23::<0x143a46408, 0x143a4640b, 0x1426cdcb0, 0x142409b50, 0x1426106d0, 0x141834d18> as *const ()), // sub_1401633e0
    Replacement::draft(0x140163440, "unknown_140163440", tpl_23::<0x143a46450, 0x143a46453, 0x1426cdcb0, 0x142409b50, 0x1426106e0, 0x141834d18> as *const ()), // sub_140163440
    Replacement::draft(0x140163ef0, "unknown_140163ef0", tpl_23::<0x143a46478, 0x143a4647b, 0x1426cdcb0, 0x142409b50, 0x142610830, 0x141834d18> as *const ()), // sub_140163ef0
    Replacement::draft(0x140163f50, "unknown_140163f50", tpl_23::<0x143a464c0, 0x143a464c3, 0x1426cdcb0, 0x142409b50, 0x142610840, 0x141834d18> as *const ()), // sub_140163f50
    Replacement::draft(0x140166570, "unknown_140166570", tpl_23::<0x143a464e8, 0x143a464eb, 0x1426cdcb0, 0x142409b50, 0x142610ab0, 0x141834d18> as *const ()), // sub_140166570
    Replacement::draft(0x1401665d0, "unknown_1401665d0", tpl_23::<0x143a46530, 0x143a46533, 0x1426cdcb0, 0x142409b50, 0x142610ac0, 0x141834d18> as *const ()), // sub_1401665d0
    Replacement::draft(0x1401667c0, "unknown_1401667c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853720, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142610b00, 0x141834d18> as *const ()), // sub_1401667c0
    Replacement::draft(0x140166c60, "unknown_140166c60", tpl_170::<0x141eb63a0> as *const ()), // sub_140166c60
    Replacement::draft(0x140169f50, "unknown_140169f50", tpl_51::<0x14336a660, 0x14336a670, 0x143a46570, 0x143311f7c, 0x14336a680, 0x14336a698, 0x1401df440, 0x142610e20, 0x141834d18> as *const ()), // sub_140169f50
    Replacement::draft(0x140169fe0, "unknown_140169fe0", tpl_23::<0x143a465b0, 0x143a465b3, 0x1426cdcb0, 0x142409b50, 0x142610e50, 0x141834d18> as *const ()), // sub_140169fe0
    Replacement::draft(0x14016a040, "unknown_14016a040", tpl_23::<0x143a465d8, 0x143a465db, 0x1426cdcb0, 0x142409b50, 0x142610e60, 0x141834d18> as *const ()), // sub_14016a040
    Replacement::draft(0x14016a230, "unknown_14016a230", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853838, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142610ea0, 0x141834d18> as *const ()), // sub_14016a230
    Replacement::draft(0x14016a5c0, "unknown_14016a5c0", fn_14016a5c0 as *const ()), // sub_14016a5c0
    Replacement::draft(0x14016a6a0, "unknown_14016a6a0", tpl_51::<0x143101424, 0x14336a894, 0x143a466a0, 0x14336ab30, 0x14336ab20, 0x1430787d0, 0x1401df440, 0x142610f40, 0x141834d18> as *const ()), // sub_14016a6a0
    Replacement::draft(0x14016a730, "unknown_14016a730", tpl_101::<0x14336ab68, 0x143a466e0, 0x1403f9250, 0x142610f70, 0x141834d18> as *const ()), // sub_14016a730
    Replacement::draft(0x14016a770, "unknown_14016a770", tpl_101::<0x14310a7d0, 0x143a46720, 0x1403f9250, 0x142610fa0, 0x141834d18> as *const ()), // sub_14016a770
    Replacement::draft(0x14016a7b0, "unknown_14016a7b0", tpl_101::<0x143109a28, 0x143a46760, 0x1403f9250, 0x142610fd0, 0x141834d18> as *const ()), // sub_14016a7b0
    Replacement::draft(0x14016a7f0, "unknown_14016a7f0", fn_14016a7f0 as *const ()), // sub_14016a7f0
    Replacement::draft(0x14016a830, "unknown_14016a830", tpl_104::<0x14336aa78, 0x14336aa90, 0x143a467e0, 0x1403f9a70, 0x142611030, 0x141834d18> as *const ()), // sub_14016a830
    Replacement::draft(0x14016a880, "unknown_14016a880", tpl_102::<0x14336ac38, 0x14336ac20, 0x143a46820, 0x1403f9a70, 0x142611060, 0x141834d18> as *const ()), // sub_14016a880
    Replacement::draft(0x14016a8d0, "unknown_14016a8d0", tpl_101::<0x14336acb0, 0x143a46860, 0x1403f9250, 0x142611090, 0x141834d18> as *const ()), // sub_14016a8d0
    Replacement::draft(0x14016a910, "unknown_14016a910", fn_14016a910 as *const ()), // sub_14016a910
    Replacement::draft(0x14016a990, "unknown_14016a990", tpl_107::<0x14336ac18, 0x14336ac10, 0x143a468e0, 0x14336adb8, 0x1401de5b0, 0x1426110f0, 0x141834d18> as *const ()), // sub_14016a990
    Replacement::draft(0x14016aa00, "unknown_14016aa00", fn_14016aa00 as *const ()), // sub_14016aa00
    Replacement::draft(0x14016ae40, "unknown_14016ae40", tpl_100::<0x14336af58, 0x14336af50, 0x143a46960, 0x14336af6c, 0x14336af64, 0x1403f8780, 0x142611150, 0x141834d18> as *const ()), // sub_14016ae40
    Replacement::draft(0x14016aec0, "unknown_14016aec0", tpl_84::<0x143109ad4, 0x143109b3c, 0x143a469a0, 0x143109bf8, 0x14309f914, 0x14309f90c, 0x143109b30, 0x143109b10, 0x143109b68, 0x143109b48, 0x14038be40, 0x142611180, 0x141834d18> as *const ()), // sub_14016aec0
    Replacement::draft(0x14016afa0, "unknown_14016afa0", tpl_102::<0x1426cf918, 0x14336af90, 0x143a469e0, 0x1403f9a70, 0x1426111b0, 0x141834d18> as *const ()), // sub_14016afa0
    Replacement::draft(0x14016aff0, "unknown_14016aff0", tpl_101::<0x14336afa8, 0x143a46a20, 0x1403f9250, 0x1426111e0, 0x141834d18> as *const ()), // sub_14016aff0
    Replacement::draft(0x14016bec0, "unknown_14016bec0", tpl_102::<0x14336ca10, 0x14336ca08, 0x143a46ae0, 0x1403f9a70, 0x142611410, 0x141834d18> as *const ()), // sub_14016bec0
    Replacement::draft(0x14016bf10, "unknown_14016bf10", tpl_102::<0x14336ca30, 0x14336ca20, 0x143a46b20, 0x1403f9a70, 0x142611440, 0x141834d18> as *const ()), // sub_14016bf10
    Replacement::draft(0x14016bf60, "unknown_14016bf60", tpl_101::<0x14336cb08, 0x143a46aa0, 0x1403f9250, 0x142611470, 0x141834d18> as *const ()), // sub_14016bf60
    Replacement::draft(0x14016bfa0, "unknown_14016bfa0", fn_14016bfa0 as *const ()), // sub_14016bfa0
    Replacement::draft(0x14016cc50, "unknown_14016cc50", tpl_23::<0x143a46b60, 0x143a46b63, 0x1426cdcb0, 0x142409b50, 0x1426114e0, 0x141834d18> as *const ()), // sub_14016cc50
    Replacement::draft(0x14016ccb0, "unknown_14016ccb0", tpl_23::<0x143a46ba8, 0x143a46bab, 0x1426cdcb0, 0x142409b50, 0x1426114f0, 0x141834d18> as *const ()), // sub_14016ccb0
    Replacement::draft(0x14016cea0, "unknown_14016cea0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853908, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142611530, 0x141834d18> as *const ()), // sub_14016cea0
    Replacement::draft(0x14016d230, "unknown_14016d230", tpl_142::<0x143a46bf0, 0x143a46bf3, 0x143079988, 0x142409b50, 0x143a46c18, 0x143a46c20, 0x143a46c30, 0x143a46c40, 0x142611570, 0x141834d18> as *const ()), // sub_14016d230
    Replacement::draft(0x14016d2c0, "unknown_14016d2c0", fn_14016d2c0 as *const ()), // sub_14016d2c0
    Replacement::draft(0x14016d7d0, "unknown_14016d7d0", fn_14016d7d0 as *const ()), // sub_14016d7d0
    Replacement::draft(0x14016d850, "unknown_14016d850", fn_14016d850 as *const ()), // sub_14016d850
    Replacement::draft(0x14016d8c0, "unknown_14016d8c0", fn_14016d8c0 as *const ()), // sub_14016d8c0
    Replacement::draft(0x14016df80, "unknown_14016df80", fn_14016df80 as *const ()), // sub_14016df80
    Replacement::draft(0x14016e440, "unknown_14016e440", tpl_23::<0x143a46dd0, 0x143a46dd3, 0x1426cdcb0, 0x142409b50, 0x1426116a0, 0x141834d18> as *const ()), // sub_14016e440
    Replacement::draft(0x14016e4a0, "unknown_14016e4a0", tpl_23::<0x143a46e18, 0x143a46e1b, 0x1426cdcb0, 0x142409b50, 0x1426116b0, 0x141834d18> as *const ()), // sub_14016e4a0
    Replacement::draft(0x14016e570, "unknown_14016e570", tpl_23::<0x143a46e40, 0x143a46e43, 0x1426cdcb0, 0x142409b50, 0x1426116d0, 0x141834d18> as *const ()), // sub_14016e570
    Replacement::draft(0x14016e5d0, "unknown_14016e5d0", tpl_23::<0x143a46e88, 0x143a46e8b, 0x1426cdcb0, 0x142409b50, 0x1426116e0, 0x141834d18> as *const ()), // sub_14016e5d0
    Replacement::draft(0x14016e7c0, "unknown_14016e7c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853998, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142611720, 0x141834d18> as *const ()), // sub_14016e7c0
    Replacement::draft(0x14016eb50, "unknown_14016eb50", tpl_101::<0x143109a38, 0x143a46f70, 0x1403f9250, 0x142611760, 0x141834d18> as *const ()), // sub_14016eb50
    Replacement::draft(0x14016eb90, "unknown_14016eb90", tpl_101::<0x14336fa80, 0x143a470b0, 0x1403f9250, 0x142611790, 0x141834d18> as *const ()), // sub_14016eb90
    Replacement::draft(0x14016ebd0, "unknown_14016ebd0", fn_14016ebd0 as *const ()), // sub_14016ebd0
    Replacement::draft(0x14016ec30, "unknown_14016ec30", fn_14016ec30 as *const ()), // sub_14016ec30
    Replacement::draft(0x14016f4f0, "unknown_14016f4f0", fn_14016f4f0 as *const ()), // sub_14016f4f0
    Replacement::draft(0x14016fa50, "unknown_14016fa50", fn_14016fa50 as *const ()), // sub_14016fa50
    Replacement::draft(0x14016fa90, "unknown_14016fa90", tpl_101::<0x1430ae7d0, 0x143a46ff0, 0x1403f9250, 0x142611880, 0x141834d18> as *const ()), // sub_14016fa90
    Replacement::draft(0x14016fb40, "unknown_14016fb40", tpl_100::<0x14310a7c8, 0x1433711b4, 0x143a46eb0, 0x143371210, 0x143371208, 0x1403f8780, 0x1426118e0, 0x141834d18> as *const ()), // sub_14016fb40
    Replacement::draft(0x14016fbc0, "unknown_14016fbc0", fn_14016fbc0 as *const ()), // sub_14016fbc0
    Replacement::draft(0x14016fc50, "unknown_14016fc50", tpl_94::<0x14337149c, 0x14336adb8, 0x143a47070, 0x143371490, 0x1433714f8, 0x1403f8780, 0x142611940, 0x141834d18> as *const ()), // sub_14016fc50
    Replacement::draft(0x14016fcd0, "unknown_14016fcd0", tpl_94::<0x1433714e8, 0x143371490, 0x143a470f0, 0x1433714d8, 0x1433714c8, 0x1403f8780, 0x142611970, 0x141834d18> as *const ()), // sub_14016fcd0
    Replacement::draft(0x140170b60, "unknown_140170b60", tpl_94::<0x14336adb8, 0x1433734c8, 0x143a471b0, 0x1433734b8, 0x1433734a8, 0x1403f8780, 0x142611a00, 0x141834d18> as *const ()), // sub_140170b60
    Replacement::draft(0x140170be0, "unknown_140170be0", tpl_101::<0x143373490, 0x143a471f0, 0x1403f9250, 0x142611a30, 0x141834d18> as *const ()), // sub_140170be0
    Replacement::draft(0x1401722a0, "unknown_1401722a0", tpl_23::<0x143a47230, 0x143a47233, 0x1426cdcb0, 0x142409b50, 0x142611ae0, 0x141834d18> as *const ()), // sub_1401722a0
    Replacement::draft(0x140172300, "unknown_140172300", tpl_23::<0x143a47278, 0x143a4727b, 0x1426cdcb0, 0x142409b50, 0x142611af0, 0x141834d18> as *const ()), // sub_140172300
    Replacement::draft(0x140175110, "unknown_140175110", tpl_23::<0x143a472a0, 0x143a472a3, 0x1426cdcb0, 0x142409b50, 0x142611c90, 0x141834d18> as *const ()), // sub_140175110
    Replacement::draft(0x140175170, "unknown_140175170", tpl_23::<0x143a472e8, 0x143a472eb, 0x1426cdcb0, 0x142409b50, 0x142611ca0, 0x141834d18> as *const ()), // sub_140175170
    Replacement::draft(0x140175240, "unknown_140175240", tpl_23::<0x143a47310, 0x143a47313, 0x1426cdcb0, 0x142409b50, 0x142611cc0, 0x141834d18> as *const ()), // sub_140175240
    Replacement::draft(0x1401752a0, "unknown_1401752a0", tpl_23::<0x143a47358, 0x143a4735b, 0x1426cdcb0, 0x142409b50, 0x142611cd0, 0x141834d18> as *const ()), // sub_1401752a0
    Replacement::draft(0x140175370, "unknown_140175370", fn_140175370 as *const ()), // sub_140175370
    Replacement::draft(0x140175a30, "unknown_140175a30", tpl_23::<0x143a473c8, 0x143a473cb, 0x1426cdcb0, 0x142409b50, 0x142611d10, 0x141834d18> as *const ()), // sub_140175a30
    Replacement::draft(0x140175a90, "unknown_140175a90", tpl_23::<0x143a47410, 0x143a47413, 0x1426cdcb0, 0x142409b50, 0x142611d20, 0x141834d18> as *const ()), // sub_140175a90
    Replacement::draft(0x140175c80, "unknown_140175c80", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853a48, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142611d60, 0x141834d18> as *const ()), // sub_140175c80
    Replacement::draft(0x140176010, "unknown_140176010", tpl_23::<0x143a47438, 0x143a4743b, 0x1426cdcb0, 0x142409b50, 0x142611da0, 0x141834d18> as *const ()), // sub_140176010
    Replacement::draft(0x140176070, "unknown_140176070", tpl_23::<0x143a47480, 0x143a47483, 0x1426cdcb0, 0x142409b50, 0x142611db0, 0x141834d18> as *const ()), // sub_140176070
    Replacement::draft(0x140176260, "unknown_140176260", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853ad8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142611df0, 0x141834d18> as *const ()), // sub_140176260
    Replacement::draft(0x140176850, "unknown_140176850", tpl_23::<0x143a474a8, 0x143a474ab, 0x1426cdcb0, 0x142409b50, 0x142611f50, 0x141834d18> as *const ()), // sub_140176850
    Replacement::draft(0x1401768b0, "unknown_1401768b0", tpl_23::<0x143a474f0, 0x143a474f3, 0x1426cdcb0, 0x142409b50, 0x142611f60, 0x141834d18> as *const ()), // sub_1401768b0
    Replacement::draft(0x140176980, "unknown_140176980", tpl_23::<0x143a47518, 0x143a4751b, 0x1426cdcb0, 0x142409b50, 0x142611f80, 0x141834d18> as *const ()), // sub_140176980
    Replacement::draft(0x1401769e0, "unknown_1401769e0", tpl_23::<0x143a47560, 0x143a47563, 0x1426cdcb0, 0x142409b50, 0x142611f90, 0x141834d18> as *const ()), // sub_1401769e0
    Replacement::draft(0x140176bd0, "unknown_140176bd0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853ba0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142611fd0, 0x141834d18> as *const ()), // sub_140176bd0
    Replacement::draft(0x140176f60, "unknown_140176f60", tpl_23::<0x143a47588, 0x143a4758b, 0x1426cdcb0, 0x142409b50, 0x142612010, 0x141834d18> as *const ()), // sub_140176f60
    Replacement::draft(0x140176fc0, "unknown_140176fc0", tpl_23::<0x143a475d0, 0x143a475d3, 0x1426cdcb0, 0x142409b50, 0x142612020, 0x141834d18> as *const ()), // sub_140176fc0
    Replacement::draft(0x1401771b0, "unknown_1401771b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853c30, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142612060, 0x141834d18> as *const ()), // sub_1401771b0
    Replacement::draft(0x140177540, "unknown_140177540", tpl_23::<0x143a475f8, 0x143a475fb, 0x1426cdcb0, 0x142409b50, 0x1426120a0, 0x141834d18> as *const ()), // sub_140177540
    Replacement::draft(0x1401775a0, "unknown_1401775a0", tpl_23::<0x143a47640, 0x143a47643, 0x1426cdcb0, 0x142409b50, 0x1426120b0, 0x141834d18> as *const ()), // sub_1401775a0
    Replacement::draft(0x140177670, "unknown_140177670", tpl_23::<0x143a47668, 0x143a4766b, 0x1426cdcb0, 0x142409b50, 0x1426120d0, 0x141834d18> as *const ()), // sub_140177670
    Replacement::draft(0x1401776d0, "unknown_1401776d0", tpl_23::<0x143a476b0, 0x143a476b3, 0x1426cdcb0, 0x142409b50, 0x1426120e0, 0x141834d18> as *const ()), // sub_1401776d0
    Replacement::draft(0x1401777a0, "unknown_1401777a0", tpl_23::<0x143a476d8, 0x143a476db, 0x1426cdcb0, 0x142409b50, 0x142612100, 0x141834d18> as *const ()), // sub_1401777a0
    Replacement::draft(0x140177800, "unknown_140177800", tpl_23::<0x143a47720, 0x143a47723, 0x1426cdcb0, 0x142409b50, 0x142612110, 0x141834d18> as *const ()), // sub_140177800
    Replacement::draft(0x1401778d0, "unknown_1401778d0", tpl_23::<0x143a47748, 0x143a4774b, 0x1426cdcb0, 0x142409b50, 0x142612130, 0x141834d18> as *const ()), // sub_1401778d0
    Replacement::draft(0x140177930, "unknown_140177930", tpl_23::<0x143a47790, 0x143a47793, 0x1426cdcb0, 0x142409b50, 0x142612140, 0x141834d18> as *const ()), // sub_140177930
    Replacement::draft(0x140178360, "unknown_140178360", tpl_23::<0x143a477b8, 0x143a477bb, 0x1426cdcb0, 0x142409b50, 0x1426121a0, 0x141834d18> as *const ()), // sub_140178360
    Replacement::draft(0x1401783c0, "unknown_1401783c0", tpl_23::<0x143a47800, 0x143a47803, 0x1426cdcb0, 0x142409b50, 0x1426121b0, 0x141834d18> as *const ()), // sub_1401783c0
    Replacement::draft(0x1401785b0, "unknown_1401785b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853cc0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1426121f0, 0x141834d18> as *const ()), // sub_1401785b0
    Replacement::draft(0x1401792e0, "unknown_1401792e0", tpl_23::<0x143a47828, 0x143a4782b, 0x1426cdcb0, 0x142409b50, 0x142612b40, 0x141834d18> as *const ()), // sub_1401792e0
    Replacement::draft(0x140179340, "unknown_140179340", tpl_23::<0x143a47870, 0x143a47873, 0x1426cdcb0, 0x142409b50, 0x142612b50, 0x141834d18> as *const ()), // sub_140179340
    Replacement::draft(0x14017b660, "unknown_14017b660", tpl_23::<0x143a47898, 0x143a4789b, 0x1426cdcb0, 0x142409b50, 0x142612f70, 0x141834d18> as *const ()), // sub_14017b660
    Replacement::draft(0x14017b6c0, "unknown_14017b6c0", tpl_23::<0x143a478e0, 0x143a478e3, 0x1426cdcb0, 0x142409b50, 0x142612f80, 0x141834d18> as *const ()), // sub_14017b6c0
    Replacement::draft(0x14017b8b0, "unknown_14017b8b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143853fb0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142612fc0, 0x141834d18> as *const ()), // sub_14017b8b0
    Replacement::draft(0x14017bd30, "unknown_14017bd30", tpl_23::<0x143a47908, 0x143a4790b, 0x1426cdcb0, 0x142409b50, 0x142613000, 0x141834d18> as *const ()), // sub_14017bd30
    Replacement::draft(0x14017bd90, "unknown_14017bd90", tpl_23::<0x143a47950, 0x143a47953, 0x1426cdcb0, 0x142409b50, 0x142613010, 0x141834d18> as *const ()), // sub_14017bd90
    Replacement::draft(0x14017c130, "unknown_14017c130", tpl_23::<0x143a47978, 0x143a4797b, 0x1426cdcb0, 0x142409b50, 0x142613030, 0x141834d18> as *const ()), // sub_14017c130
    Replacement::draft(0x14017c190, "unknown_14017c190", tpl_23::<0x143a479c0, 0x143a479c3, 0x1426cdcb0, 0x142409b50, 0x142613040, 0x141834d18> as *const ()), // sub_14017c190
    Replacement::draft(0x14017d430, "unknown_14017d430", fn_14017d430 as *const ()), // sub_14017d430
    Replacement::draft(0x14017d450, "unknown_14017d450", tpl_23::<0x143a479e8, 0x143a479eb, 0x1426cdcb0, 0x142409b50, 0x1426130a0, 0x141834d18> as *const ()), // sub_14017d450
    Replacement::draft(0x14017d4b0, "unknown_14017d4b0", tpl_23::<0x143a47a30, 0x143a47a33, 0x1426cdcb0, 0x142409b50, 0x1426130b0, 0x141834d18> as *const ()), // sub_14017d4b0
    Replacement::draft(0x14017d6a0, "unknown_14017d6a0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143854040, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1426130f0, 0x141834d18> as *const ()), // sub_14017d6a0
    Replacement::draft(0x14017da30, "unknown_14017da30", tpl_23::<0x143a47a58, 0x143a47a5b, 0x1426cdcb0, 0x142409b50, 0x142613130, 0x141834d18> as *const ()), // sub_14017da30
    Replacement::draft(0x14017da90, "unknown_14017da90", tpl_23::<0x143a47aa0, 0x143a47aa3, 0x1426cdcb0, 0x142409b50, 0x142613140, 0x141834d18> as *const ()), // sub_14017da90
    Replacement::draft(0x14017dc80, "unknown_14017dc80", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438540d0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142613180, 0x141834d18> as *const ()), // sub_14017dc80
    Replacement::draft(0x14017e2d0, "unknown_14017e2d0", tpl_23::<0x143a47ac8, 0x143a47acb, 0x1426cdcb0, 0x142409b50, 0x142613910, 0x141834d18> as *const ()), // sub_14017e2d0
    Replacement::draft(0x14017e330, "unknown_14017e330", tpl_23::<0x143a47b10, 0x143a47b13, 0x1426cdcb0, 0x142409b50, 0x142613920, 0x141834d18> as *const ()), // sub_14017e330
    Replacement::draft(0x14017e520, "unknown_14017e520", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438543d8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142613960, 0x141834d18> as *const ()), // sub_14017e520
    Replacement::draft_throws(0x14017e8b0, "unknown_14017e8b0", fn_14017e8b0 as *const (), Throws::Caught), // sub_14017e8b0
    Replacement::draft(0x14017ec40, "unknown_14017ec40", tpl_23::<0x143a47b48, 0x143a47b4b, 0x1426cdcb0, 0x142409b50, 0x142613d70, 0x141834d18> as *const ()), // sub_14017ec40
    Replacement::draft(0x14017eca0, "unknown_14017eca0", tpl_23::<0x143a47b90, 0x143a47b93, 0x1426cdcb0, 0x142409b50, 0x142613d80, 0x141834d18> as *const ()), // sub_14017eca0
    Replacement::draft(0x14017ee30, "unknown_14017ee30", tpl_23::<0x143a47bb8, 0x143a47bbb, 0x1426cdcb0, 0x142409b50, 0x142613f50, 0x141834d18> as *const ()), // sub_14017ee30
    Replacement::draft(0x14017ee90, "unknown_14017ee90", tpl_23::<0x143a47c00, 0x143a47c03, 0x1426cdcb0, 0x142409b50, 0x142613f60, 0x141834d18> as *const ()), // sub_14017ee90
    Replacement::draft(0x14017efa0, "unknown_14017efa0", tpl_23::<0x143a47c28, 0x143a47c2b, 0x1426cdcb0, 0x142409b50, 0x142614010, 0x141834d18> as *const ()), // sub_14017efa0
    Replacement::draft(0x14017f000, "unknown_14017f000", tpl_23::<0x143a47c70, 0x143a47c73, 0x1426cdcb0, 0x142409b50, 0x142614020, 0x141834d18> as *const ()), // sub_14017f000
    Replacement::draft(0x14017f1f0, "unknown_14017f1f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143854630, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142614060, 0x141834d18> as *const ()), // sub_14017f1f0
    Replacement::draft(0x14017f7c0, "unknown_14017f7c0", tpl_127::<0x1438546c0, 0x143a47c98, 0x140548a40, 0x1426145b0, 0x141834d18> as *const ()), // sub_14017f7c0
    Replacement::draft(0x14017f930, "unknown_14017f930", tpl_56::<0x143426abc, 0x143426d48, 0x143a001ac, 0x1438547b8, 0x1438547d0, 0x143a00290, 0x1438547a8, 0x142614910, 0x141834d18> as *const ()), // sub_14017f930
    Replacement::draft(0x14017f9a0, "unknown_14017f9a0", tpl_135::<0x14307aafc, 0x143391e8d, 0x143a00288, 0x143cdc050, 0x14384e98c, 0x143cdc058, 0x1410db310, 0x143cdc080, 0x143cdc060, 0x143cdc070> as *const ()), // sub_14017f9a0
    Replacement::draft(0x14017fa30, "unknown_14017fa30", fn_14017fa30 as *const ()), // sub_14017fa30
    Replacement::draft(0x14017fac0, "unknown_14017fac0", fn_14017fac0 as *const ()), // sub_14017fac0
    Replacement::draft(0x14017fc40, "unknown_14017fc40", fn_14017fc40 as *const ()), // sub_14017fc40
    Replacement::draft(0x14017fcd0, "unknown_14017fcd0", fn_14017fcd0 as *const ()), // sub_14017fcd0
    Replacement::draft(0x14017fe50, "unknown_14017fe50", tpl_23::<0x143a47cc0, 0x143a47cc3, 0x1426cdcb0, 0x142409b50, 0x1426149d0, 0x141834d18> as *const ()), // sub_14017fe50
    Replacement::draft(0x14017feb0, "unknown_14017feb0", tpl_23::<0x143a47d08, 0x143a47d0b, 0x1426cdcb0, 0x142409b50, 0x1426149e0, 0x141834d18> as *const ()), // sub_14017feb0
    Replacement::draft(0x140181460, "unknown_140181460", tpl_23::<0x143a47d30, 0x143a47d33, 0x1426cdcb0, 0x142409b50, 0x142614ae0, 0x141834d18> as *const ()), // sub_140181460
    Replacement::draft(0x1401814c0, "unknown_1401814c0", tpl_23::<0x143a47d78, 0x143a47d7b, 0x1426cdcb0, 0x142409b50, 0x142614af0, 0x141834d18> as *const ()), // sub_1401814c0
    Replacement::draft(0x140182200, "unknown_140182200", tpl_23::<0x143a47da0, 0x143a47da3, 0x1426cdcb0, 0x142409b50, 0x142614bc0, 0x141834d18> as *const ()), // sub_140182200
    Replacement::draft(0x140182260, "unknown_140182260", tpl_23::<0x143a47de8, 0x143a47deb, 0x1426cdcb0, 0x142409b50, 0x142614bd0, 0x141834d18> as *const ()), // sub_140182260
    Replacement::draft(0x140182450, "unknown_140182450", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143854918, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142614c10, 0x141834d18> as *const ()), // sub_140182450
    Replacement::draft(0x140182950, "unknown_140182950", tpl_23::<0x143a47e10, 0x143a47e13, 0x1426cdcb0, 0x142409b50, 0x142614d70, 0x141834d18> as *const ()), // sub_140182950
    Replacement::draft(0x1401829b0, "unknown_1401829b0", tpl_23::<0x143a47e58, 0x143a47e5b, 0x1426cdcb0, 0x142409b50, 0x142614d80, 0x141834d18> as *const ()), // sub_1401829b0
    Replacement::draft(0x140182ba0, "unknown_140182ba0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143855590, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142614dc0, 0x141834d18> as *const ()), // sub_140182ba0
    Replacement::draft(0x140182ff0, "unknown_140182ff0", tpl_193::<0x14307aafc, 0x14339a49d, 0x143a00288, 0x143cdd5c8, 0x1438542ec, 0x143cdd5d0, 0x1410f4e70, 0x143cdd5f8, 0x143cdd5d8, 0x143cdd5e8> as *const ()), // sub_140182ff0
    Replacement::draft(0x140183080, "unknown_140183080", tpl_193::<0x14307aafc, 0x14339986d, 0x143a00288, 0x143cdd450, 0x14384ed6c, 0x143cdd458, 0x1410f4f60, 0x143cdd480, 0x143cdd460, 0x143cdd470> as *const ()), // sub_140183080
    Replacement::draft(0x140183200, "unknown_140183200", tpl_164::<0x14307aafc, 0x143399c4d, 0x143a00288, 0x143cdd4c8, 0x14384ed68, 0x143cdd4d0, 0x1410f5140, 0x143cdd4f8, 0x143cdd4d8, 0x143cdd4e8> as *const ()), // sub_140183200
    Replacement::draft(0x140183290, "unknown_140183290", fn_140183290 as *const ()), // sub_140183290
    Replacement::draft(0x140183320, "unknown_140183320", fn_140183320 as *const ()), // sub_140183320
    Replacement::draft(0x140183590, "unknown_140183590", fn_140183590 as *const ()), // sub_140183590
    Replacement::draft(0x140183640, "unknown_140183640", fn_140183640 as *const ()), // sub_140183640
    Replacement::draft(0x140184c10, "unknown_140184c10", tpl_23::<0x143a47eb8, 0x143a47ebb, 0x1426cdcb0, 0x142409b50, 0x1426152e0, 0x141834d18> as *const ()), // sub_140184c10
    Replacement::draft(0x140184c70, "unknown_140184c70", tpl_23::<0x143a47f00, 0x143a47f03, 0x1426cdcb0, 0x142409b50, 0x1426152f0, 0x141834d18> as *const ()), // sub_140184c70
    Replacement::draft(0x140184d40, "unknown_140184d40", tpl_23::<0x143a47f28, 0x143a47f2b, 0x1426cdcb0, 0x142409b50, 0x142615310, 0x141834d18> as *const ()), // sub_140184d40
    Replacement::draft(0x140184da0, "unknown_140184da0", tpl_23::<0x143a47f70, 0x143a47f73, 0x1426cdcb0, 0x142409b50, 0x142615320, 0x141834d18> as *const ()), // sub_140184da0
    Replacement::draft_throws(0x140184ef0, "unknown_140184ef0", fn_140184ef0 as *const (), Throws::Caught), // sub_140184ef0
    Replacement::draft_throws(0x140184ff0, "unknown_140184ff0", fn_140184ff0 as *const (), Throws::Caught), // sub_140184ff0
    Replacement::draft_throws(0x140185100, "unknown_140185100", fn_140185100 as *const (), Throws::Caught), // sub_140185100
    Replacement::draft(0x140186790, "unknown_140186790", tpl_23::<0x143a47f98, 0x143a47f9b, 0x1426cdcb0, 0x142409b50, 0x142615530, 0x141834d18> as *const ()), // sub_140186790
    Replacement::draft(0x1401867f0, "unknown_1401867f0", tpl_23::<0x143a47fe0, 0x143a47fe3, 0x1426cdcb0, 0x142409b50, 0x142615540, 0x141834d18> as *const ()), // sub_1401867f0
    Replacement::draft(0x1401868c0, "unknown_1401868c0", tpl_23::<0x143a48008, 0x143a4800b, 0x1426cdcb0, 0x142409b50, 0x142615560, 0x141834d18> as *const ()), // sub_1401868c0
    Replacement::draft(0x140186920, "unknown_140186920", tpl_23::<0x143a48050, 0x143a48053, 0x1426cdcb0, 0x142409b50, 0x142615570, 0x141834d18> as *const ()), // sub_140186920
    Replacement::draft(0x140186b10, "unknown_140186b10", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143855950, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1426155b0, 0x141834d18> as *const ()), // sub_140186b10
    Replacement::draft(0x140186ea0, "unknown_140186ea0", tpl_23::<0x143a48078, 0x143a4807b, 0x1426cdcb0, 0x142409b50, 0x1426155f0, 0x141834d18> as *const ()), // sub_140186ea0
    Replacement::draft(0x140186f00, "unknown_140186f00", tpl_23::<0x143a480c0, 0x143a480c3, 0x1426cdcb0, 0x142409b50, 0x142615600, 0x141834d18> as *const ()), // sub_140186f00
    Replacement::draft(0x140186fd0, "unknown_140186fd0", tpl_23::<0x143a480e8, 0x143a480eb, 0x1426cdcb0, 0x142409b50, 0x142615620, 0x141834d18> as *const ()), // sub_140186fd0
    Replacement::draft(0x140187030, "unknown_140187030", tpl_23::<0x143a48130, 0x143a48133, 0x1426cdcb0, 0x142409b50, 0x142615630, 0x141834d18> as *const ()), // sub_140187030
    Replacement::draft(0x140187320, "unknown_140187320", tpl_23::<0x143a48158, 0x143a4815b, 0x1426cdcb0, 0x142409b50, 0x1426156f0, 0x141834d18> as *const ()), // sub_140187320
    Replacement::draft(0x140187380, "unknown_140187380", tpl_23::<0x143a481a0, 0x143a481a3, 0x1426cdcb0, 0x142409b50, 0x142615700, 0x141834d18> as *const ()), // sub_140187380
    Replacement::draft(0x140187570, "unknown_140187570", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438559f8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142615740, 0x141834d18> as *const ()), // sub_140187570
    Replacement::draft(0x140187900, "unknown_140187900", tpl_23::<0x143a481c8, 0x143a481cb, 0x1426cdcb0, 0x142409b50, 0x142615780, 0x141834d18> as *const ()), // sub_140187900
    Replacement::draft(0x140187960, "unknown_140187960", tpl_23::<0x143a48210, 0x143a48213, 0x1426cdcb0, 0x142409b50, 0x142615790, 0x141834d18> as *const ()), // sub_140187960
    Replacement::draft(0x140187b50, "unknown_140187b50", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143855a88, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x1426157d0, 0x141834d18> as *const ()), // sub_140187b50
    Replacement::draft(0x140187ee0, "unknown_140187ee0", tpl_23::<0x143a48238, 0x143a4823b, 0x1426cdcb0, 0x142409b50, 0x142615810, 0x141834d18> as *const ()), // sub_140187ee0
    Replacement::draft(0x140187f40, "unknown_140187f40", tpl_23::<0x143a48280, 0x143a48283, 0x1426cdcb0, 0x142409b50, 0x142615820, 0x141834d18> as *const ()), // sub_140187f40
    Replacement::draft(0x140188880, "unknown_140188880", tpl_23::<0x143a48310, 0x143a48313, 0x1426cdcb0, 0x142409b50, 0x1426158d0, 0x141834d18> as *const ()), // sub_140188880
    Replacement::draft(0x1401888e0, "unknown_1401888e0", tpl_23::<0x143a48358, 0x143a4835b, 0x1426cdcb0, 0x142409b50, 0x1426158e0, 0x141834d18> as *const ()), // sub_1401888e0
    Replacement::draft(0x140188ad0, "unknown_140188ad0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143855b20, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142615920, 0x141834d18> as *const ()), // sub_140188ad0
    Replacement::draft(0x140188e60, "unknown_140188e60", tpl_23::<0x143a48380, 0x143a48383, 0x1426cdcb0, 0x142409b50, 0x142615960, 0x141834d18> as *const ()), // sub_140188e60
    Replacement::draft(0x140188ec0, "unknown_140188ec0", tpl_23::<0x143a483c8, 0x143a483cb, 0x1426cdcb0, 0x142409b50, 0x142615970, 0x141834d18> as *const ()), // sub_140188ec0
    Replacement::draft(0x1401890c0, "unknown_1401890c0", tpl_23::<0x143a483f0, 0x143a483f3, 0x1426cdcb0, 0x142409b50, 0x142615a20, 0x141834d18> as *const ()), // sub_1401890c0
    Replacement::draft(0x140189120, "unknown_140189120", tpl_23::<0x143a48438, 0x143a4843b, 0x1426cdcb0, 0x142409b50, 0x142615a30, 0x141834d18> as *const ()), // sub_140189120
    Replacement::draft(0x140189310, "unknown_140189310", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143855bc8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142615a70, 0x141834d18> as *const ()), // sub_140189310
    Replacement::draft(0x14018ba60, "unknown_14018ba60", tpl_23::<0x143a48460, 0x143a48463, 0x1426cdcb0, 0x142409b50, 0x142615e60, 0x141834d18> as *const ()), // sub_14018ba60
    Replacement::draft(0x14018bac0, "unknown_14018bac0", tpl_23::<0x143a484a8, 0x143a484ab, 0x1426cdcb0, 0x142409b50, 0x142615e70, 0x141834d18> as *const ()), // sub_14018bac0
    Replacement::draft(0x14018bcb0, "unknown_14018bcb0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143855d38, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142615eb0, 0x141834d18> as *const ()), // sub_14018bcb0
    Replacement::draft(0x14018e5b0, "unknown_14018e5b0", tpl_23::<0x143a484d0, 0x143a484d3, 0x1426cdcb0, 0x142409b50, 0x142616530, 0x141834d18> as *const ()), // sub_14018e5b0
    Replacement::draft(0x14018e610, "unknown_14018e610", tpl_23::<0x143a48518, 0x143a4851b, 0x1426cdcb0, 0x142409b50, 0x142616540, 0x141834d18> as *const ()), // sub_14018e610
    Replacement::draft(0x14018e810, "unknown_14018e810", tpl_23::<0x143a48540, 0x143a48543, 0x1426cdcb0, 0x142409b50, 0x1426165f0, 0x141834d18> as *const ()), // sub_14018e810
    Replacement::draft(0x14018e870, "unknown_14018e870", tpl_23::<0x143a48588, 0x143a4858b, 0x1426cdcb0, 0x142409b50, 0x142616600, 0x141834d18> as *const ()), // sub_14018e870
    Replacement::draft(0x14018ea60, "unknown_14018ea60", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438560e0, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142616640, 0x141834d18> as *const ()), // sub_14018ea60
    Replacement::draft_throws(0x14018ee70, "unknown_14018ee70", fn_14018ee70 as *const (), Throws::Caught), // sub_14018ee70
    Replacement::draft_throws(0x14018f000, "unknown_14018f000", fn_14018f000 as *const (), Throws::Caught), // sub_14018f000
    Replacement::draft_throws(0x14018f190, "unknown_14018f190", fn_14018f190 as *const (), Throws::Caught), // sub_14018f190
    Replacement::draft(0x14018f2a0, "unknown_14018f2a0", fn_14018f2a0 as *const ()), // sub_14018f2a0
    Replacement::draft(0x14018f5c0, "unknown_14018f5c0", tpl_23::<0x143a488a0, 0x143a488a3, 0x1426cdcb0, 0x142409b50, 0x142616950, 0x141834d18> as *const ()), // sub_14018f5c0
    Replacement::draft(0x14018f620, "unknown_14018f620", tpl_23::<0x143a488e8, 0x143a488eb, 0x1426cdcb0, 0x142409b50, 0x142616960, 0x141834d18> as *const ()), // sub_14018f620
    Replacement::draft(0x140190500, "unknown_140190500", tpl_23::<0x143a48910, 0x143a48913, 0x1426cdcb0, 0x142409b50, 0x1426169c0, 0x141834d18> as *const ()), // sub_140190500
    Replacement::draft(0x140190560, "unknown_140190560", tpl_23::<0x143a48958, 0x143a4895b, 0x1426cdcb0, 0x142409b50, 0x1426169d0, 0x141834d18> as *const ()), // sub_140190560
    Replacement::draft(0x140190750, "unknown_140190750", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143856270, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142616a10, 0x141834d18> as *const ()), // sub_140190750
    Replacement::draft(0x140190cc0, "unknown_140190cc0", tpl_121::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143ce20d0, 0x142616a60, 0x141834d18> as *const ()), // sub_140190cc0
    Replacement::draft_throws(0x140190d60, "unknown_140190d60", fn_140190d60 as *const (), Throws::Discard), // sub_140190d60
    Replacement::draft_throws(0x140190f10, "unknown_140190f10", fn_140190f10 as *const (), Throws::Discard), // sub_140190f10
    Replacement::draft(0x140191bf0, "unknown_140191bf0", tpl_23::<0x143a489c0, 0x143a489c3, 0x1426cdcb0, 0x142409b50, 0x142616c70, 0x141834d18> as *const ()), // sub_140191bf0
    Replacement::draft(0x140191c50, "unknown_140191c50", tpl_23::<0x143a48a08, 0x143a48a0b, 0x1426cdcb0, 0x142409b50, 0x142616c80, 0x141834d18> as *const ()), // sub_140191c50
    Replacement::draft(0x140191e40, "unknown_140191e40", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143856300, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142616cc0, 0x141834d18> as *const ()), // sub_140191e40
    Replacement::draft(0x140193930, "unknown_140193930", tpl_206::<0x1401fc200, 0x8, 0x143c48998, 0x143c489a8, 0x143c489b0, 0x143c489c0, 0x143c489c8, 0x1434268d0, 0x143c48990, 0x1401e5730, 0x142617db0, 0x141834d18> as *const ()), // sub_140193930
    Replacement::draft(0x1401939b0, "unknown_1401939b0", tpl_206::<0x1401fc200, 0x8, 0x143c48888, 0x143c48898, 0x143c488a0, 0x143c488b0, 0x143c488b8, 0x1434268d0, 0x143c48880, 0x1401e5730, 0x142617dd0, 0x141834d18> as *const ()), // sub_1401939b0
    Replacement::draft(0x140193a30, "unknown_140193a30", fn_140193a30 as *const ()), // sub_140193a30
    Replacement::draft(0x140193ad0, "unknown_140193ad0", fn_140193ad0 as *const ()), // sub_140193ad0
    Replacement::draft(0x1401948a0, "unknown_1401948a0", tpl_23::<0x143a48a68, 0x143a48a6b, 0x1426cdcb0, 0x142409b50, 0x142617e50, 0x141834d18> as *const ()), // sub_1401948a0
    Replacement::draft(0x140194900, "unknown_140194900", tpl_23::<0x143a48ab0, 0x143a48ab3, 0x1426cdcb0, 0x142409b50, 0x142617e60, 0x141834d18> as *const ()), // sub_140194900
    Replacement::draft(0x140198650, "unknown_140198650", tpl_23::<0x143a48ad8, 0x143a48adb, 0x1426cdcb0, 0x142409b50, 0x1426180f0, 0x141834d18> as *const ()), // sub_140198650
    Replacement::draft(0x1401986b0, "unknown_1401986b0", tpl_23::<0x143a48b20, 0x143a48b23, 0x1426cdcb0, 0x142409b50, 0x142618100, 0x141834d18> as *const ()), // sub_1401986b0
    Replacement::draft(0x140198870, "unknown_140198870", tpl_23::<0x143a48b48, 0x143a48b4b, 0x1426cdcb0, 0x142409b50, 0x142618120, 0x141834d18> as *const ()), // sub_140198870
    Replacement::draft(0x1401988d0, "unknown_1401988d0", tpl_23::<0x143a48b90, 0x143a48b93, 0x1426cdcb0, 0x142409b50, 0x142618130, 0x141834d18> as *const ()), // sub_1401988d0
    Replacement::draft(0x140198ac0, "unknown_140198ac0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143857290, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x142618170, 0x141834d18> as *const ()), // sub_140198ac0
    Replacement::draft(0x140199160, "unknown_140199160", fn_140199160 as *const ()), // sub_140199160
    Replacement::draft(0x140199370, "unknown_140199370", fn_140199370 as *const ()), // sub_140199370
    Replacement::draft(0x14019b320, "unknown_14019b320", fn_14019b320 as *const ()), // sub_14019b320
    Replacement::draft(0x14019b390, "unknown_14019b390", fn_14019b390 as *const ()), // sub_14019b390
    Replacement::draft(0x14019b3d0, "unknown_14019b3d0", fn_14019b3d0 as *const ()), // sub_14019b3d0
    Replacement::draft(0x14019b600, "unknown_14019b600", tpl_214::<0x14021b730, 0x14261a160, 0x8, 0x10, 0x18, 0x143a631d8, 0x141834d18> as *const ()), // sub_14019b600
    Replacement::draft(0x14019b730, "unknown_14019b730", fn_14019b730 as *const ()), // sub_14019b730
    Replacement::draft(0x14019bb90, "unknown_14019bb90", fn_14019bb90 as *const ()), // sub_14019bb90
    Replacement::draft(0x14019bc20, "unknown_14019bc20", fn_14019bc20 as *const ()), // sub_14019bc20
    Replacement::draft(0x14019bcd0, "unknown_14019bcd0", tpl_127::<0x143857f80, 0x143a636f0, 0x140548a40, 0x14261a7e0, 0x141834d18> as *const ()), // sub_14019bcd0
    Replacement::draft(0x14019bd40, "unknown_14019bd40", tpl_127::<0x143857f60, 0x143a63718, 0x140548a40, 0x14261a900, 0x141834d18> as *const ()), // sub_14019bd40
    Replacement::draft(0x14019c3b0, "unknown_14019c3b0", tpl_127::<0x143858180, 0x143a63768, 0x140548a40, 0x14261b830, 0x141834d18> as *const ()), // sub_14019c3b0
    Replacement::draft(0x14019c3e0, "unknown_14019c3e0", tpl_127::<0x143858140, 0x143a63740, 0x140548a40, 0x14261b8c0, 0x141834d18> as *const ()), // sub_14019c3e0
    Replacement::draft(0x14019c460, "unknown_14019c460", fn_14019c460 as *const ()), // sub_14019c460
    Replacement::draft(0x14019c490, "unknown_14019c490", tpl_214::<0x14021b730, 0x14261ba60, 0x8, 0x10, 0x18, 0x143a877b0, 0x141834d18> as *const ()), // sub_14019c490
    Replacement::draft(0x14019c4d0, "unknown_14019c4d0", tpl_219::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143a877c0, 0x14261bbc0, 0x141834d18> as *const ()), // sub_14019c4d0
    Replacement::draft(0x14019c5a0, "unknown_14019c5a0", fn_14019c5a0 as *const ()), // sub_14019c5a0
    Replacement::draft_throws(0x14019c5e0, "unknown_14019c5e0", fn_14019c5e0 as *const (), Throws::Discard), // sub_14019c5e0
    Replacement::draft(0x14019c7b0, "unknown_14019c7b0", tpl_23::<0x143a877d0, 0x143a877d3, 0x1426cdcb0, 0x142409b50, 0x14261c010, 0x141834d18> as *const ()), // sub_14019c7b0
    Replacement::draft(0x14019c810, "unknown_14019c810", tpl_23::<0x143a87818, 0x143a8781b, 0x1426cdcb0, 0x142409b50, 0x14261c020, 0x141834d18> as *const ()), // sub_14019c810
    Replacement::draft(0x14019ca00, "unknown_14019ca00", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861688, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c060, 0x141834d18> as *const ()), // sub_14019ca00
    Replacement::draft(0x14019cd90, "unknown_14019cd90", tpl_23::<0x143a87840, 0x143a87843, 0x1426cdcb0, 0x142409b50, 0x14261c0a0, 0x141834d18> as *const ()), // sub_14019cd90
    Replacement::draft(0x14019cdf0, "unknown_14019cdf0", tpl_23::<0x143a87888, 0x143a8788b, 0x1426cdcb0, 0x142409b50, 0x14261c0b0, 0x141834d18> as *const ()), // sub_14019cdf0
    Replacement::draft(0x14019cfe0, "unknown_14019cfe0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861718, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c0f0, 0x141834d18> as *const ()), // sub_14019cfe0
    Replacement::draft(0x14019d370, "unknown_14019d370", tpl_23::<0x143a878b0, 0x143a878b3, 0x1426cdcb0, 0x142409b50, 0x14261c130, 0x141834d18> as *const ()), // sub_14019d370
    Replacement::draft(0x14019d3d0, "unknown_14019d3d0", tpl_23::<0x143a878f8, 0x143a878fb, 0x1426cdcb0, 0x142409b50, 0x14261c140, 0x141834d18> as *const ()), // sub_14019d3d0
    Replacement::draft(0x14019d5c0, "unknown_14019d5c0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438617a8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c180, 0x141834d18> as *const ()), // sub_14019d5c0
    Replacement::draft(0x14019d950, "unknown_14019d950", tpl_23::<0x143a87920, 0x143a87923, 0x1426cdcb0, 0x142409b50, 0x14261c1c0, 0x141834d18> as *const ()), // sub_14019d950
    Replacement::draft(0x14019d9b0, "unknown_14019d9b0", tpl_23::<0x143a87968, 0x143a8796b, 0x1426cdcb0, 0x142409b50, 0x14261c1d0, 0x141834d18> as *const ()), // sub_14019d9b0
    Replacement::draft(0x14019dba0, "unknown_14019dba0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861838, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c210, 0x141834d18> as *const ()), // sub_14019dba0
    Replacement::draft(0x14019df30, "unknown_14019df30", tpl_23::<0x143a87990, 0x143a87993, 0x1426cdcb0, 0x142409b50, 0x14261c250, 0x141834d18> as *const ()), // sub_14019df30
    Replacement::draft(0x14019df90, "unknown_14019df90", tpl_23::<0x143a879d8, 0x143a879db, 0x1426cdcb0, 0x142409b50, 0x14261c260, 0x141834d18> as *const ()), // sub_14019df90
    Replacement::draft(0x14019e180, "unknown_14019e180", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438618c8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c2a0, 0x141834d18> as *const ()), // sub_14019e180
    Replacement::draft(0x14019e510, "unknown_14019e510", tpl_23::<0x143a87a00, 0x143a87a03, 0x1426cdcb0, 0x142409b50, 0x14261c2e0, 0x141834d18> as *const ()), // sub_14019e510
    Replacement::draft(0x14019e570, "unknown_14019e570", tpl_23::<0x143a87a48, 0x143a87a4b, 0x1426cdcb0, 0x142409b50, 0x14261c2f0, 0x141834d18> as *const ()), // sub_14019e570
    Replacement::draft(0x14019e760, "unknown_14019e760", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861958, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c330, 0x141834d18> as *const ()), // sub_14019e760
    Replacement::draft(0x14019eaf0, "unknown_14019eaf0", tpl_23::<0x143a87a70, 0x143a87a73, 0x1426cdcb0, 0x142409b50, 0x14261c370, 0x141834d18> as *const ()), // sub_14019eaf0
    Replacement::draft(0x14019eb50, "unknown_14019eb50", tpl_23::<0x143a87ab8, 0x143a87abb, 0x1426cdcb0, 0x142409b50, 0x14261c380, 0x141834d18> as *const ()), // sub_14019eb50
    Replacement::draft(0x14019ed40, "unknown_14019ed40", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438619e8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c3c0, 0x141834d18> as *const ()), // sub_14019ed40
    Replacement::draft(0x14019f0d0, "unknown_14019f0d0", tpl_23::<0x143a87ae0, 0x143a87ae3, 0x1426cdcb0, 0x142409b50, 0x14261c400, 0x141834d18> as *const ()), // sub_14019f0d0
    Replacement::draft(0x14019f130, "unknown_14019f130", tpl_23::<0x143a87b28, 0x143a87b2b, 0x1426cdcb0, 0x142409b50, 0x14261c410, 0x141834d18> as *const ()), // sub_14019f130
    Replacement::draft(0x14019f320, "unknown_14019f320", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861a78, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c450, 0x141834d18> as *const ()), // sub_14019f320
    Replacement::draft(0x14019f6b0, "unknown_14019f6b0", tpl_23::<0x143a87b50, 0x143a87b53, 0x1426cdcb0, 0x142409b50, 0x14261c490, 0x141834d18> as *const ()), // sub_14019f6b0
    Replacement::draft(0x14019f710, "unknown_14019f710", tpl_23::<0x143a87b98, 0x143a87b9b, 0x1426cdcb0, 0x142409b50, 0x14261c4a0, 0x141834d18> as *const ()), // sub_14019f710
    Replacement::draft(0x14019f900, "unknown_14019f900", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861b08, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c4e0, 0x141834d18> as *const ()), // sub_14019f900
    Replacement::draft(0x14019fc90, "unknown_14019fc90", tpl_23::<0x143a87bc0, 0x143a87bc3, 0x1426cdcb0, 0x142409b50, 0x14261c520, 0x141834d18> as *const ()), // sub_14019fc90
    Replacement::draft(0x14019fcf0, "unknown_14019fcf0", tpl_23::<0x143a87c08, 0x143a87c0b, 0x1426cdcb0, 0x142409b50, 0x14261c530, 0x141834d18> as *const ()), // sub_14019fcf0
    Replacement::draft(0x14019fee0, "unknown_14019fee0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861b98, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c570, 0x141834d18> as *const ()), // sub_14019fee0
    Replacement::draft(0x1401a0270, "unknown_1401a0270", tpl_23::<0x143a87c30, 0x143a87c33, 0x1426cdcb0, 0x142409b50, 0x14261c5b0, 0x141834d18> as *const ()), // sub_1401a0270
    Replacement::draft(0x1401a02d0, "unknown_1401a02d0", tpl_23::<0x143a87c78, 0x143a87c7b, 0x1426cdcb0, 0x142409b50, 0x14261c5c0, 0x141834d18> as *const ()), // sub_1401a02d0
    Replacement::draft(0x1401a03a0, "unknown_1401a03a0", tpl_23::<0x143a87ca0, 0x143a87ca3, 0x1426cdcb0, 0x142409b50, 0x14261c5e0, 0x141834d18> as *const ()), // sub_1401a03a0
    Replacement::draft(0x1401a0400, "unknown_1401a0400", tpl_23::<0x143a87ce8, 0x143a87ceb, 0x1426cdcb0, 0x142409b50, 0x14261c5f0, 0x141834d18> as *const ()), // sub_1401a0400
    Replacement::draft(0x1401a05f0, "unknown_1401a05f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861c28, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c630, 0x141834d18> as *const ()), // sub_1401a05f0
    Replacement::draft(0x1401a0980, "unknown_1401a0980", tpl_23::<0x143a87d10, 0x143a87d13, 0x1426cdcb0, 0x142409b50, 0x14261c670, 0x141834d18> as *const ()), // sub_1401a0980
    Replacement::draft(0x1401a09e0, "unknown_1401a09e0", tpl_23::<0x143a87d58, 0x143a87d5b, 0x1426cdcb0, 0x142409b50, 0x14261c680, 0x141834d18> as *const ()), // sub_1401a09e0
    Replacement::draft(0x1401a0bd0, "unknown_1401a0bd0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861cb8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c6c0, 0x141834d18> as *const ()), // sub_1401a0bd0
    Replacement::draft(0x1401a0f60, "unknown_1401a0f60", tpl_23::<0x143a87d80, 0x143a87d83, 0x1426cdcb0, 0x142409b50, 0x14261c700, 0x141834d18> as *const ()), // sub_1401a0f60
    Replacement::draft(0x1401a0fc0, "unknown_1401a0fc0", tpl_23::<0x143a87dc8, 0x143a87dcb, 0x1426cdcb0, 0x142409b50, 0x14261c710, 0x141834d18> as *const ()), // sub_1401a0fc0
    Replacement::draft(0x1401a11b0, "unknown_1401a11b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861d48, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c750, 0x141834d18> as *const ()), // sub_1401a11b0
    Replacement::draft(0x1401a1540, "unknown_1401a1540", tpl_23::<0x143a87df0, 0x143a87df3, 0x1426cdcb0, 0x142409b50, 0x14261c790, 0x141834d18> as *const ()), // sub_1401a1540
    Replacement::draft(0x1401a15a0, "unknown_1401a15a0", tpl_23::<0x143a87e38, 0x143a87e3b, 0x1426cdcb0, 0x142409b50, 0x14261c7a0, 0x141834d18> as *const ()), // sub_1401a15a0
    Replacement::draft(0x1401a1790, "unknown_1401a1790", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861dd8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c7e0, 0x141834d18> as *const ()), // sub_1401a1790
    Replacement::draft(0x1401a1b20, "unknown_1401a1b20", tpl_23::<0x143a87e60, 0x143a87e63, 0x1426cdcb0, 0x142409b50, 0x14261c820, 0x141834d18> as *const ()), // sub_1401a1b20
    Replacement::draft(0x1401a1b80, "unknown_1401a1b80", tpl_23::<0x143a87ea8, 0x143a87eab, 0x1426cdcb0, 0x142409b50, 0x14261c830, 0x141834d18> as *const ()), // sub_1401a1b80
    Replacement::draft(0x1401a1d70, "unknown_1401a1d70", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861e68, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c870, 0x141834d18> as *const ()), // sub_1401a1d70
    Replacement::draft(0x1401a2100, "unknown_1401a2100", tpl_23::<0x143a87ed0, 0x143a87ed3, 0x1426cdcb0, 0x142409b50, 0x14261c8b0, 0x141834d18> as *const ()), // sub_1401a2100
    Replacement::draft(0x1401a2160, "unknown_1401a2160", tpl_23::<0x143a87f18, 0x143a87f1b, 0x1426cdcb0, 0x142409b50, 0x14261c8c0, 0x141834d18> as *const ()), // sub_1401a2160
    Replacement::draft(0x1401a2350, "unknown_1401a2350", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861ef8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c900, 0x141834d18> as *const ()), // sub_1401a2350
    Replacement::draft(0x1401a26e0, "unknown_1401a26e0", tpl_23::<0x143a87f40, 0x143a87f43, 0x1426cdcb0, 0x142409b50, 0x14261c940, 0x141834d18> as *const ()), // sub_1401a26e0
    Replacement::draft(0x1401a2740, "unknown_1401a2740", tpl_23::<0x143a87f88, 0x143a87f8b, 0x1426cdcb0, 0x142409b50, 0x14261c950, 0x141834d18> as *const ()), // sub_1401a2740
    Replacement::draft(0x1401a2930, "unknown_1401a2930", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143861f88, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261c990, 0x141834d18> as *const ()), // sub_1401a2930
    Replacement::draft(0x1401a2cc0, "unknown_1401a2cc0", tpl_23::<0x143a87fb0, 0x143a87fb3, 0x1426cdcb0, 0x142409b50, 0x14261c9d0, 0x141834d18> as *const ()), // sub_1401a2cc0
    Replacement::draft(0x1401a2d20, "unknown_1401a2d20", tpl_23::<0x143a87ff8, 0x143a87ffb, 0x1426cdcb0, 0x142409b50, 0x14261c9e0, 0x141834d18> as *const ()), // sub_1401a2d20
    Replacement::draft(0x1401a2f10, "unknown_1401a2f10", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438633b8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261ca20, 0x141834d18> as *const ()), // sub_1401a2f10
    Replacement::draft(0x1401a32a0, "unknown_1401a32a0", tpl_23::<0x143a88020, 0x143a88023, 0x1426cdcb0, 0x142409b50, 0x14261ca60, 0x141834d18> as *const ()), // sub_1401a32a0
    Replacement::draft(0x1401a3300, "unknown_1401a3300", tpl_23::<0x143a88068, 0x143a8806b, 0x1426cdcb0, 0x142409b50, 0x14261ca70, 0x141834d18> as *const ()), // sub_1401a3300
    Replacement::draft(0x1401a34f0, "unknown_1401a34f0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143863448, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261cab0, 0x141834d18> as *const ()), // sub_1401a34f0
    Replacement::draft(0x1401a3880, "unknown_1401a3880", tpl_23::<0x143a88090, 0x143a88093, 0x1426cdcb0, 0x142409b50, 0x14261caf0, 0x141834d18> as *const ()), // sub_1401a3880
    Replacement::draft(0x1401a38e0, "unknown_1401a38e0", tpl_23::<0x143a880d8, 0x143a880db, 0x1426cdcb0, 0x142409b50, 0x14261cb00, 0x141834d18> as *const ()), // sub_1401a38e0
    Replacement::draft(0x1401a3ad0, "unknown_1401a3ad0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438634d8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261cb40, 0x141834d18> as *const ()), // sub_1401a3ad0
    Replacement::draft(0x1401a3e60, "unknown_1401a3e60", tpl_23::<0x143a88100, 0x143a88103, 0x1426cdcb0, 0x142409b50, 0x14261cb80, 0x141834d18> as *const ()), // sub_1401a3e60
    Replacement::draft(0x1401a3ec0, "unknown_1401a3ec0", tpl_23::<0x143a88148, 0x143a8814b, 0x1426cdcb0, 0x142409b50, 0x14261cb90, 0x141834d18> as *const ()), // sub_1401a3ec0
    Replacement::draft(0x1401a40b0, "unknown_1401a40b0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143863568, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261cbd0, 0x141834d18> as *const ()), // sub_1401a40b0
    Replacement::draft(0x1401a4440, "unknown_1401a4440", tpl_23::<0x143a88170, 0x143a88173, 0x1426cdcb0, 0x142409b50, 0x14261cc10, 0x141834d18> as *const ()), // sub_1401a4440
    Replacement::draft(0x1401a44a0, "unknown_1401a44a0", tpl_23::<0x143a881b8, 0x143a881bb, 0x1426cdcb0, 0x142409b50, 0x14261cc20, 0x141834d18> as *const ()), // sub_1401a44a0
    Replacement::draft(0x1401a4690, "unknown_1401a4690", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x1438635f8, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261cc60, 0x141834d18> as *const ()), // sub_1401a4690
    Replacement::draft(0x1401a4a20, "unknown_1401a4a20", tpl_23::<0x143a881e0, 0x143a881e3, 0x1426cdcb0, 0x142409b50, 0x14261cca0, 0x141834d18> as *const ()), // sub_1401a4a20
    Replacement::draft(0x1401a4a80, "unknown_1401a4a80", tpl_23::<0x143a88228, 0x143a8822b, 0x1426cdcb0, 0x142409b50, 0x14261ccb0, 0x141834d18> as *const ()), // sub_1401a4a80
    Replacement::draft(0x1401a4c70, "unknown_1401a4c70", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143863688, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261ccf0, 0x141834d18> as *const ()), // sub_1401a4c70
    Replacement::draft(0x1401a5000, "unknown_1401a5000", tpl_120::<0x143a88270, 0x143a88273, 0x143079988, 0x142409b50, 0x143a88298, 0x143a882a0, 0x143a882b0, 0x143a882c0, 0x14261cd30, 0x141834d18> as *const ()), // sub_1401a5000
    Replacement::draft(0x1401a5090, "unknown_1401a5090", tpl_23::<0x143a882c8, 0x143a882cb, 0x1426cdcb0, 0x142409b50, 0x14261cd70, 0x141834d18> as *const ()), // sub_1401a5090
    Replacement::draft(0x1401a50f0, "unknown_1401a50f0", tpl_23::<0x143a88310, 0x143a88313, 0x1426cdcb0, 0x142409b50, 0x14261cd80, 0x141834d18> as *const ()), // sub_1401a50f0
    Replacement::draft(0x1401a52e0, "unknown_1401a52e0", tpl_24::<0x1426cf5f8, 0x1426cf608, 0x1426cf618, 0x1426cf628, 0x1426cf638, 0x1426cf648, 0x1426cf658, 0x1426cf668, 0x1426cf678, 0x1426cf690, 0x1426cf6a8, 0x1426cf6c0, 0x143863718, 0x1401de990, 0x1424b16a0, 0x1401dede0, 0x14261cdc0, 0x141834d18> as *const ()), // sub_1401a52e0
    Replacement::draft(0x1401a5670, "unknown_1401a5670", fn_1401a5670 as *const ()), // sub_1401a5670
    Replacement::draft(0x1401a5790, "unknown_1401a5790", tpl_223::<0x143c8c648, 0x143863a18, 0x143863a08> as *const ()), // sub_1401a5790
    Replacement::draft(0x1401a7590, "unknown_1401a7590", fn_1401a7590 as *const ()), // sub_1401a7590
    Replacement::draft(0x1401a92e0, "unknown_1401a92e0", tpl_23::<0x143a88a10, 0x143a88a13, 0x1426cdcb0, 0x142409b50, 0x142623f00, 0x141834d18> as *const ()), // sub_1401a92e0
    Replacement::draft(0x1401a9340, "unknown_1401a9340", tpl_23::<0x143a88a58, 0x143a88a5b, 0x1426cdcb0, 0x142409b50, 0x142623f10, 0x141834d18> as *const ()), // sub_1401a9340
    Replacement::draft(0x1401a9410, "unknown_1401a9410", tpl_23::<0x143a88a80, 0x143a88a83, 0x1426cdcb0, 0x142409b50, 0x142623f30, 0x141834d18> as *const ()), // sub_1401a9410
    Replacement::draft(0x1401a9470, "unknown_1401a9470", tpl_23::<0x143a88ac8, 0x143a88acb, 0x1426cdcb0, 0x142409b50, 0x142623f40, 0x141834d18> as *const ()), // sub_1401a9470
    Replacement::draft(0x1401a9540, "unknown_1401a9540", tpl_23::<0x143a88af0, 0x143a88af3, 0x1426cdcb0, 0x142409b50, 0x142623f60, 0x141834d18> as *const ()), // sub_1401a9540
    Replacement::draft(0x1401a95a0, "unknown_1401a95a0", tpl_23::<0x143a88b38, 0x143a88b3b, 0x1426cdcb0, 0x142409b50, 0x142623f70, 0x141834d18> as *const ()), // sub_1401a95a0
    Replacement::draft(0x1401a9670, "unknown_1401a9670", tpl_23::<0x143a88b60, 0x143a88b63, 0x1426cdcb0, 0x142409b50, 0x142623f90, 0x141834d18> as *const ()), // sub_1401a9670
    Replacement::draft(0x1401a96d0, "unknown_1401a96d0", tpl_23::<0x143a88ba8, 0x143a88bab, 0x1426cdcb0, 0x142409b50, 0x142623fa0, 0x141834d18> as *const ()), // sub_1401a96d0
    Replacement::draft(0x1401a97a0, "unknown_1401a97a0", tpl_23::<0x143a88bd0, 0x143a88bd3, 0x1426cdcb0, 0x142409b50, 0x142623fc0, 0x141834d18> as *const ()), // sub_1401a97a0
    Replacement::draft(0x1401a9800, "unknown_1401a9800", tpl_23::<0x143a88c18, 0x143a88c1b, 0x1426cdcb0, 0x142409b50, 0x142623fd0, 0x141834d18> as *const ()), // sub_1401a9800
    Replacement::draft(0x1401a98d0, "unknown_1401a98d0", tpl_23::<0x143a88c40, 0x143a88c43, 0x1426cdcb0, 0x142409b50, 0x142623ff0, 0x141834d18> as *const ()), // sub_1401a98d0
    Replacement::draft(0x1401a9930, "unknown_1401a9930", tpl_23::<0x143a88c88, 0x143a88c8b, 0x1426cdcb0, 0x142409b50, 0x142624000, 0x141834d18> as *const ()), // sub_1401a9930
    Replacement::draft(0x1401a9a00, "unknown_1401a9a00", tpl_23::<0x143a88cb0, 0x143a88cb3, 0x1426cdcb0, 0x142409b50, 0x142624020, 0x141834d18> as *const ()), // sub_1401a9a00
    Replacement::draft(0x1401a9a60, "unknown_1401a9a60", tpl_23::<0x143a88cf8, 0x143a88cfb, 0x1426cdcb0, 0x142409b50, 0x142624030, 0x141834d18> as *const ()), // sub_1401a9a60
    Replacement::draft(0x1401a9b30, "unknown_1401a9b30", tpl_23::<0x143a88d20, 0x143a88d23, 0x1426cdcb0, 0x142409b50, 0x142624050, 0x141834d18> as *const ()), // sub_1401a9b30
    Replacement::draft(0x1401a9b90, "unknown_1401a9b90", tpl_23::<0x143a88d68, 0x143a88d6b, 0x1426cdcb0, 0x142409b50, 0x142624060, 0x141834d18> as *const ()), // sub_1401a9b90
    Replacement::draft(0x1401a9c60, "unknown_1401a9c60", tpl_23::<0x143a88d90, 0x143a88d93, 0x1426cdcb0, 0x142409b50, 0x142624080, 0x141834d18> as *const ()), // sub_1401a9c60
    Replacement::draft(0x1401a9cc0, "unknown_1401a9cc0", tpl_23::<0x143a88dd8, 0x143a88ddb, 0x1426cdcb0, 0x142409b50, 0x142624090, 0x141834d18> as *const ()), // sub_1401a9cc0
    Replacement::draft(0x1401a9d90, "unknown_1401a9d90", tpl_23::<0x143a88e00, 0x143a88e03, 0x1426cdcb0, 0x142409b50, 0x1426240b0, 0x141834d18> as *const ()), // sub_1401a9d90
    Replacement::draft(0x1401a9df0, "unknown_1401a9df0", tpl_23::<0x143a88e48, 0x143a88e4b, 0x1426cdcb0, 0x142409b50, 0x1426240c0, 0x141834d18> as *const ()), // sub_1401a9df0
    Replacement::draft(0x1401a9ec0, "unknown_1401a9ec0", tpl_23::<0x143a88e70, 0x143a88e73, 0x1426cdcb0, 0x142409b50, 0x1426240e0, 0x141834d18> as *const ()), // sub_1401a9ec0
    Replacement::draft(0x1401a9f20, "unknown_1401a9f20", tpl_23::<0x143a88eb8, 0x143a88ebb, 0x1426cdcb0, 0x142409b50, 0x1426240f0, 0x141834d18> as *const ()), // sub_1401a9f20
    Replacement::draft(0x1401a9ff0, "unknown_1401a9ff0", fn_1401a9ff0 as *const ()), // sub_1401a9ff0
    Replacement::draft(0x1401aa050, "unknown_1401aa050", tpl_23::<0x143a88f68, 0x143a88f6b, 0x1426cdcb0, 0x142409b50, 0x142624120, 0x141834d18> as *const ()), // sub_1401aa050
    Replacement::draft(0x1401aa0b0, "unknown_1401aa0b0", tpl_23::<0x143a88fb0, 0x143a88fb3, 0x1426cdcb0, 0x142409b50, 0x142624130, 0x141834d18> as *const ()), // sub_1401aa0b0
    Replacement::draft(0x1401aa180, "unknown_1401aa180", tpl_23::<0x143a88fd8, 0x143a88fdb, 0x1426cdcb0, 0x142409b50, 0x142624150, 0x141834d18> as *const ()), // sub_1401aa180
    Replacement::draft(0x1401aa1e0, "unknown_1401aa1e0", tpl_23::<0x143a89020, 0x143a89023, 0x1426cdcb0, 0x142409b50, 0x142624160, 0x141834d18> as *const ()), // sub_1401aa1e0
    Replacement::draft(0x1401aa2b0, "unknown_1401aa2b0", tpl_23::<0x143a89048, 0x143a8904b, 0x1426cdcb0, 0x142409b50, 0x142624180, 0x141834d18> as *const ()), // sub_1401aa2b0
    Replacement::draft(0x1401aa310, "unknown_1401aa310", tpl_23::<0x143a89090, 0x143a89093, 0x1426cdcb0, 0x142409b50, 0x142624190, 0x141834d18> as *const ()), // sub_1401aa310
    Replacement::draft(0x1401aa3e0, "unknown_1401aa3e0", tpl_23::<0x143a890b8, 0x143a890bb, 0x1426cdcb0, 0x142409b50, 0x1426241b0, 0x141834d18> as *const ()), // sub_1401aa3e0
    Replacement::draft(0x1401aa440, "unknown_1401aa440", tpl_23::<0x143a89100, 0x143a89103, 0x1426cdcb0, 0x142409b50, 0x1426241c0, 0x141834d18> as *const ()), // sub_1401aa440
    Replacement::draft(0x1401aa510, "unknown_1401aa510", tpl_23::<0x143a89128, 0x143a8912b, 0x1426cdcb0, 0x142409b50, 0x1426241e0, 0x141834d18> as *const ()), // sub_1401aa510
    Replacement::draft(0x1401aa570, "unknown_1401aa570", tpl_23::<0x143a89170, 0x143a89173, 0x1426cdcb0, 0x142409b50, 0x1426241f0, 0x141834d18> as *const ()), // sub_1401aa570
    Replacement::draft(0x1401aa640, "unknown_1401aa640", tpl_23::<0x143a89198, 0x143a8919b, 0x1426cdcb0, 0x142409b50, 0x142624210, 0x141834d18> as *const ()), // sub_1401aa640
    Replacement::draft(0x1401aa6a0, "unknown_1401aa6a0", tpl_23::<0x143a891e0, 0x143a891e3, 0x1426cdcb0, 0x142409b50, 0x142624220, 0x141834d18> as *const ()), // sub_1401aa6a0
    Replacement::draft(0x1401aa770, "unknown_1401aa770", tpl_23::<0x143a89208, 0x143a8920b, 0x1426cdcb0, 0x142409b50, 0x142624240, 0x141834d18> as *const ()), // sub_1401aa770
    Replacement::draft(0x1401aa7d0, "unknown_1401aa7d0", tpl_23::<0x143a89250, 0x143a89253, 0x1426cdcb0, 0x142409b50, 0x142624250, 0x141834d18> as *const ()), // sub_1401aa7d0
    Replacement::draft(0x1401aa8a0, "unknown_1401aa8a0", tpl_23::<0x143a89278, 0x143a8927b, 0x1426cdcb0, 0x142409b50, 0x142624270, 0x141834d18> as *const ()), // sub_1401aa8a0
    Replacement::draft(0x1401aa900, "unknown_1401aa900", tpl_23::<0x143a892c0, 0x143a892c3, 0x1426cdcb0, 0x142409b50, 0x142624280, 0x141834d18> as *const ()), // sub_1401aa900
    Replacement::draft(0x1401aa9d0, "unknown_1401aa9d0", tpl_23::<0x143a892e8, 0x143a892eb, 0x1426cdcb0, 0x142409b50, 0x1426242b0, 0x141834d18> as *const ()), // sub_1401aa9d0
    Replacement::draft(0x1401aaa30, "unknown_1401aaa30", tpl_23::<0x143a89330, 0x143a89333, 0x1426cdcb0, 0x142409b50, 0x1426242c0, 0x141834d18> as *const ()), // sub_1401aaa30
    Replacement::draft(0x1401aab00, "unknown_1401aab00", tpl_23::<0x143a89358, 0x143a8935b, 0x1426cdcb0, 0x142409b50, 0x1426242e0, 0x141834d18> as *const ()), // sub_1401aab00
    Replacement::draft(0x1401aab60, "unknown_1401aab60", tpl_23::<0x143a893a0, 0x143a893a3, 0x1426cdcb0, 0x142409b50, 0x1426242f0, 0x141834d18> as *const ()), // sub_1401aab60
    Replacement::draft(0x1401aac30, "unknown_1401aac30", tpl_23::<0x143a893c8, 0x143a893cb, 0x1426cdcb0, 0x142409b50, 0x142624310, 0x141834d18> as *const ()), // sub_1401aac30
    Replacement::draft(0x1401aac90, "unknown_1401aac90", tpl_23::<0x143a89410, 0x143a89413, 0x1426cdcb0, 0x142409b50, 0x142624320, 0x141834d18> as *const ()), // sub_1401aac90
    Replacement::draft(0x1401aad70, "unknown_1401aad70", tpl_23::<0x143a89458, 0x143a8945b, 0x1426cdcb0, 0x142409b50, 0x142624370, 0x141834d18> as *const ()), // sub_1401aad70
    Replacement::draft(0x1401aadd0, "unknown_1401aadd0", tpl_23::<0x143a894a0, 0x143a894a3, 0x1426cdcb0, 0x142409b50, 0x142624380, 0x141834d18> as *const ()), // sub_1401aadd0
    Replacement::draft(0x1401aaea0, "unknown_1401aaea0", tpl_23::<0x143a894c8, 0x143a894cb, 0x1426cdcb0, 0x142409b50, 0x1426243a0, 0x141834d18> as *const ()), // sub_1401aaea0
    Replacement::draft(0x1401aaf00, "unknown_1401aaf00", tpl_23::<0x143a89510, 0x143a89513, 0x1426cdcb0, 0x142409b50, 0x1426243b0, 0x141834d18> as *const ()), // sub_1401aaf00
    Replacement::draft(0x1401aafd0, "unknown_1401aafd0", tpl_23::<0x143a89538, 0x143a8953b, 0x1426cdcb0, 0x142409b50, 0x1426243d0, 0x141834d18> as *const ()), // sub_1401aafd0
    Replacement::draft(0x1401ab030, "unknown_1401ab030", tpl_23::<0x143a89580, 0x143a89583, 0x1426cdcb0, 0x142409b50, 0x1426243e0, 0x141834d18> as *const ()), // sub_1401ab030
    Replacement::draft(0x1401ab100, "unknown_1401ab100", tpl_23::<0x143a895a8, 0x143a895ab, 0x1426cdcb0, 0x142409b50, 0x142624400, 0x141834d18> as *const ()), // sub_1401ab100
    Replacement::draft(0x1401ab160, "unknown_1401ab160", tpl_23::<0x143a895f0, 0x143a895f3, 0x1426cdcb0, 0x142409b50, 0x142624410, 0x141834d18> as *const ()), // sub_1401ab160
    Replacement::draft(0x1401ab230, "unknown_1401ab230", tpl_23::<0x143a89618, 0x143a8961b, 0x1426cdcb0, 0x142409b50, 0x142624430, 0x141834d18> as *const ()), // sub_1401ab230
    Replacement::draft(0x1401ab290, "unknown_1401ab290", tpl_23::<0x143a89660, 0x143a89663, 0x1426cdcb0, 0x142409b50, 0x142624440, 0x141834d18> as *const ()), // sub_1401ab290
    Replacement::draft(0x1401ab360, "unknown_1401ab360", tpl_23::<0x143a89688, 0x143a8968b, 0x1426cdcb0, 0x142409b50, 0x142624460, 0x141834d18> as *const ()), // sub_1401ab360
    Replacement::draft(0x1401ab3c0, "unknown_1401ab3c0", tpl_23::<0x143a896d0, 0x143a896d3, 0x1426cdcb0, 0x142409b50, 0x142624470, 0x141834d18> as *const ()), // sub_1401ab3c0
    Replacement::draft(0x1401ab490, "unknown_1401ab490", tpl_23::<0x143a896f8, 0x143a896fb, 0x1426cdcb0, 0x142409b50, 0x142624490, 0x141834d18> as *const ()), // sub_1401ab490
    Replacement::draft(0x1401ab4f0, "unknown_1401ab4f0", tpl_23::<0x143a89740, 0x143a89743, 0x1426cdcb0, 0x142409b50, 0x1426244a0, 0x141834d18> as *const ()), // sub_1401ab4f0
    Replacement::draft(0x1401ab5c0, "unknown_1401ab5c0", tpl_23::<0x143a89768, 0x143a8976b, 0x1426cdcb0, 0x142409b50, 0x1426244c0, 0x141834d18> as *const ()), // sub_1401ab5c0
    Replacement::draft(0x1401ab620, "unknown_1401ab620", tpl_23::<0x143a897b0, 0x143a897b3, 0x1426cdcb0, 0x142409b50, 0x1426244d0, 0x141834d18> as *const ()), // sub_1401ab620
    Replacement::draft(0x1401ab6f0, "unknown_1401ab6f0", tpl_23::<0x143a897d8, 0x143a897db, 0x1426cdcb0, 0x142409b50, 0x1426244f0, 0x141834d18> as *const ()), // sub_1401ab6f0
    Replacement::draft(0x1401ab750, "unknown_1401ab750", tpl_23::<0x143a89820, 0x143a89823, 0x1426cdcb0, 0x142409b50, 0x142624500, 0x141834d18> as *const ()), // sub_1401ab750
    Replacement::draft(0x1401ab820, "unknown_1401ab820", tpl_23::<0x143a89848, 0x143a8984b, 0x1426cdcb0, 0x142409b50, 0x142624520, 0x141834d18> as *const ()), // sub_1401ab820
    Replacement::draft(0x1401ab880, "unknown_1401ab880", tpl_23::<0x143a89890, 0x143a89893, 0x1426cdcb0, 0x142409b50, 0x142624530, 0x141834d18> as *const ()), // sub_1401ab880
    Replacement::draft(0x1401ab950, "unknown_1401ab950", tpl_23::<0x143a898f8, 0x143a898fb, 0x1426cdcb0, 0x142409b50, 0x142624550, 0x141834d18> as *const ()), // sub_1401ab950
    Replacement::draft(0x1401ab9b0, "unknown_1401ab9b0", tpl_23::<0x143a89940, 0x143a89943, 0x1426cdcb0, 0x142409b50, 0x142624560, 0x141834d18> as *const ()), // sub_1401ab9b0
    Replacement::draft(0x1401abae0, "unknown_1401abae0", tpl_23::<0x143a899e8, 0x143a899eb, 0x1426cdcb0, 0x142409b50, 0x1426246a0, 0x141834d18> as *const ()), // sub_1401abae0
    Replacement::draft(0x1401abb40, "unknown_1401abb40", tpl_23::<0x143a89a30, 0x143a89a33, 0x1426cdcb0, 0x142409b50, 0x1426246b0, 0x141834d18> as *const ()), // sub_1401abb40
    Replacement::draft(0x1401abc10, "unknown_1401abc10", tpl_23::<0x143a89a58, 0x143a89a5b, 0x1426cdcb0, 0x142409b50, 0x1426246d0, 0x141834d18> as *const ()), // sub_1401abc10
    Replacement::draft(0x1401abc70, "unknown_1401abc70", tpl_23::<0x143a89aa0, 0x143a89aa3, 0x1426cdcb0, 0x142409b50, 0x1426246e0, 0x141834d18> as *const ()), // sub_1401abc70
    Replacement::draft(0x1401abd40, "unknown_1401abd40", tpl_23::<0x143a89ac8, 0x143a89acb, 0x1426cdcb0, 0x142409b50, 0x142624700, 0x141834d18> as *const ()), // sub_1401abd40
    Replacement::draft(0x1401abda0, "unknown_1401abda0", tpl_23::<0x143a89b10, 0x143a89b13, 0x1426cdcb0, 0x142409b50, 0x142624710, 0x141834d18> as *const ()), // sub_1401abda0
    Replacement::draft(0x1401abe70, "unknown_1401abe70", tpl_23::<0x143a89b38, 0x143a89b3b, 0x1426cdcb0, 0x142409b50, 0x142624730, 0x141834d18> as *const ()), // sub_1401abe70
    Replacement::draft(0x1401abed0, "unknown_1401abed0", tpl_23::<0x143a89b80, 0x143a89b83, 0x1426cdcb0, 0x142409b50, 0x142624740, 0x141834d18> as *const ()), // sub_1401abed0
    Replacement::draft(0x1401abfa0, "unknown_1401abfa0", tpl_23::<0x143a89ba8, 0x143a89bab, 0x1426cdcb0, 0x142409b50, 0x142624760, 0x141834d18> as *const ()), // sub_1401abfa0
    Replacement::draft(0x1401ac000, "unknown_1401ac000", tpl_23::<0x143a89bf0, 0x143a89bf3, 0x1426cdcb0, 0x142409b50, 0x142624770, 0x141834d18> as *const ()), // sub_1401ac000
    Replacement::draft(0x1401ac0d0, "unknown_1401ac0d0", tpl_23::<0x143a89c18, 0x143a89c1b, 0x1426cdcb0, 0x142409b50, 0x142624790, 0x141834d18> as *const ()), // sub_1401ac0d0
    Replacement::draft(0x1401ac130, "unknown_1401ac130", tpl_23::<0x143a89c60, 0x143a89c63, 0x1426cdcb0, 0x142409b50, 0x1426247a0, 0x141834d18> as *const ()), // sub_1401ac130
    Replacement::draft(0x1401ac200, "unknown_1401ac200", tpl_23::<0x143a89c88, 0x143a89c8b, 0x1426cdcb0, 0x142409b50, 0x1426247c0, 0x141834d18> as *const ()), // sub_1401ac200
    Replacement::draft(0x1401ac260, "unknown_1401ac260", tpl_23::<0x143a89cd0, 0x143a89cd3, 0x1426cdcb0, 0x142409b50, 0x1426247d0, 0x141834d18> as *const ()), // sub_1401ac260
    Replacement::draft(0x1401ac330, "unknown_1401ac330", tpl_23::<0x143a89cf8, 0x143a89cfb, 0x1426cdcb0, 0x142409b50, 0x1426247f0, 0x141834d18> as *const ()), // sub_1401ac330
    Replacement::draft(0x1401ac390, "unknown_1401ac390", tpl_23::<0x143a89d40, 0x143a89d43, 0x1426cdcb0, 0x142409b50, 0x142624800, 0x141834d18> as *const ()), // sub_1401ac390
    Replacement::draft(0x1401ac460, "unknown_1401ac460", tpl_23::<0x143a89d68, 0x143a89d6b, 0x1426cdcb0, 0x142409b50, 0x142624820, 0x141834d18> as *const ()), // sub_1401ac460
    Replacement::draft(0x1401ac4c0, "unknown_1401ac4c0", tpl_23::<0x143a89db0, 0x143a89db3, 0x1426cdcb0, 0x142409b50, 0x142624830, 0x141834d18> as *const ()), // sub_1401ac4c0
    Replacement::draft(0x1401ac590, "unknown_1401ac590", tpl_23::<0x143a89dd8, 0x143a89ddb, 0x1426cdcb0, 0x142409b50, 0x142624850, 0x141834d18> as *const ()), // sub_1401ac590
    Replacement::draft(0x1401ac5f0, "unknown_1401ac5f0", tpl_23::<0x143a89e20, 0x143a89e23, 0x1426cdcb0, 0x142409b50, 0x142624860, 0x141834d18> as *const ()), // sub_1401ac5f0
    Replacement::draft(0x1401ac6c0, "unknown_1401ac6c0", tpl_23::<0x143a89e48, 0x143a89e4b, 0x1426cdcb0, 0x142409b50, 0x142624880, 0x141834d18> as *const ()), // sub_1401ac6c0
    Replacement::draft(0x1401ac720, "unknown_1401ac720", tpl_23::<0x143a89e90, 0x143a89e93, 0x1426cdcb0, 0x142409b50, 0x142624890, 0x141834d18> as *const ()), // sub_1401ac720
    Replacement::draft(0x1401ac7f0, "unknown_1401ac7f0", tpl_23::<0x143a89eb8, 0x143a89ebb, 0x1426cdcb0, 0x142409b50, 0x1426248b0, 0x141834d18> as *const ()), // sub_1401ac7f0
    Replacement::draft(0x1401ac850, "unknown_1401ac850", tpl_23::<0x143a89f00, 0x143a89f03, 0x1426cdcb0, 0x142409b50, 0x1426248c0, 0x141834d18> as *const ()), // sub_1401ac850
    Replacement::draft(0x1401ac920, "unknown_1401ac920", tpl_23::<0x143a89f28, 0x143a89f2b, 0x1426cdcb0, 0x142409b50, 0x1426248e0, 0x141834d18> as *const ()), // sub_1401ac920
    Replacement::draft(0x1401ac980, "unknown_1401ac980", tpl_23::<0x143a89f70, 0x143a89f73, 0x1426cdcb0, 0x142409b50, 0x1426248f0, 0x141834d18> as *const ()), // sub_1401ac980
    Replacement::draft(0x1401aca50, "unknown_1401aca50", tpl_23::<0x143a89f98, 0x143a89f9b, 0x1426cdcb0, 0x142409b50, 0x142624910, 0x141834d18> as *const ()), // sub_1401aca50
    Replacement::draft(0x1401acab0, "unknown_1401acab0", tpl_23::<0x143a89fe0, 0x143a89fe3, 0x1426cdcb0, 0x142409b50, 0x142624920, 0x141834d18> as *const ()), // sub_1401acab0
    Replacement::draft(0x1401acb80, "unknown_1401acb80", fn_1401acb80 as *const ()), // sub_1401acb80
    Replacement::draft(0x1401acbc0, "unknown_1401acbc0", tpl_23::<0x143a8a050, 0x143a8a053, 0x1426cdcb0, 0x142409b50, 0x142624950, 0x141834d18> as *const ()), // sub_1401acbc0
    Replacement::draft(0x1401acc20, "unknown_1401acc20", tpl_23::<0x143a8a098, 0x143a8a09b, 0x1426cdcb0, 0x142409b50, 0x142624960, 0x141834d18> as *const ()), // sub_1401acc20
    Replacement::draft(0x1401accf0, "unknown_1401accf0", tpl_23::<0x143a8a0c0, 0x143a8a0c3, 0x1426cdcb0, 0x142409b50, 0x142624980, 0x141834d18> as *const ()), // sub_1401accf0
    Replacement::draft(0x1401acd50, "unknown_1401acd50", tpl_23::<0x143a8a108, 0x143a8a10b, 0x1426cdcb0, 0x142409b50, 0x142624990, 0x141834d18> as *const ()), // sub_1401acd50
    Replacement::draft(0x1401ace20, "unknown_1401ace20", tpl_23::<0x143a8a130, 0x143a8a133, 0x1426cdcb0, 0x142409b50, 0x1426249b0, 0x141834d18> as *const ()), // sub_1401ace20
    Replacement::draft(0x1401ace80, "unknown_1401ace80", tpl_23::<0x143a8a178, 0x143a8a17b, 0x1426cdcb0, 0x142409b50, 0x1426249c0, 0x141834d18> as *const ()), // sub_1401ace80
    Replacement::draft(0x1401acf50, "unknown_1401acf50", tpl_23::<0x143a8a1a0, 0x143a8a1a3, 0x1426cdcb0, 0x142409b50, 0x1426249e0, 0x141834d18> as *const ()), // sub_1401acf50
    Replacement::draft(0x1401acfb0, "unknown_1401acfb0", tpl_23::<0x143a8a1e8, 0x143a8a1eb, 0x1426cdcb0, 0x142409b50, 0x1426249f0, 0x141834d18> as *const ()), // sub_1401acfb0
    Replacement::draft(0x1401ad080, "unknown_1401ad080", tpl_23::<0x143a8a210, 0x143a8a213, 0x1426cdcb0, 0x142409b50, 0x142624a10, 0x141834d18> as *const ()), // sub_1401ad080
    Replacement::draft(0x1401ad0e0, "unknown_1401ad0e0", tpl_23::<0x143a8a258, 0x143a8a25b, 0x1426cdcb0, 0x142409b50, 0x142624a20, 0x141834d18> as *const ()), // sub_1401ad0e0
    Replacement::draft(0x1401ad1b0, "unknown_1401ad1b0", tpl_23::<0x143a8a280, 0x143a8a283, 0x1426cdcb0, 0x142409b50, 0x142624a40, 0x141834d18> as *const ()), // sub_1401ad1b0
    Replacement::draft(0x1401ad210, "unknown_1401ad210", tpl_23::<0x143a8a2c8, 0x143a8a2cb, 0x1426cdcb0, 0x142409b50, 0x142624a50, 0x141834d18> as *const ()), // sub_1401ad210
    Replacement::draft(0x1401ad2e0, "unknown_1401ad2e0", tpl_23::<0x143a8a2f0, 0x143a8a2f3, 0x1426cdcb0, 0x142409b50, 0x142624a70, 0x141834d18> as *const ()), // sub_1401ad2e0
    Replacement::draft(0x1401ad340, "unknown_1401ad340", tpl_23::<0x143a8a338, 0x143a8a33b, 0x1426cdcb0, 0x142409b50, 0x142624a80, 0x141834d18> as *const ()), // sub_1401ad340
    Replacement::draft(0x1401ad410, "unknown_1401ad410", tpl_23::<0x143a8a360, 0x143a8a363, 0x1426cdcb0, 0x142409b50, 0x142624aa0, 0x141834d18> as *const ()), // sub_1401ad410
    Replacement::draft(0x1401ad470, "unknown_1401ad470", tpl_23::<0x143a8a3a8, 0x143a8a3ab, 0x1426cdcb0, 0x142409b50, 0x142624ab0, 0x141834d18> as *const ()), // sub_1401ad470
    Replacement::draft(0x1401ad540, "unknown_1401ad540", tpl_23::<0x143a8a3d0, 0x143a8a3d3, 0x1426cdcb0, 0x142409b50, 0x142624ad0, 0x141834d18> as *const ()), // sub_1401ad540
    Replacement::draft(0x1401ad5a0, "unknown_1401ad5a0", tpl_23::<0x143a8a418, 0x143a8a41b, 0x1426cdcb0, 0x142409b50, 0x142624ae0, 0x141834d18> as *const ()), // sub_1401ad5a0
    Replacement::draft(0x1401ad670, "unknown_1401ad670", tpl_23::<0x143a8a440, 0x143a8a443, 0x1426cdcb0, 0x142409b50, 0x142624b00, 0x141834d18> as *const ()), // sub_1401ad670
    Replacement::draft(0x1401ad6d0, "unknown_1401ad6d0", tpl_23::<0x143a8a488, 0x143a8a48b, 0x1426cdcb0, 0x142409b50, 0x142624b10, 0x141834d18> as *const ()), // sub_1401ad6d0
    Replacement::draft(0x1401ad7a0, "unknown_1401ad7a0", tpl_23::<0x143a8a4b0, 0x143a8a4b3, 0x1426cdcb0, 0x142409b50, 0x142624b30, 0x141834d18> as *const ()), // sub_1401ad7a0
    Replacement::draft(0x1401ad800, "unknown_1401ad800", tpl_23::<0x143a8a4f8, 0x143a8a4fb, 0x1426cdcb0, 0x142409b50, 0x142624b40, 0x141834d18> as *const ()), // sub_1401ad800
    Replacement::draft(0x1401ad8d0, "unknown_1401ad8d0", tpl_23::<0x143a8a520, 0x143a8a523, 0x1426cdcb0, 0x142409b50, 0x142624b60, 0x141834d18> as *const ()), // sub_1401ad8d0
    Replacement::draft(0x1401ad930, "unknown_1401ad930", tpl_23::<0x143a8a568, 0x143a8a56b, 0x1426cdcb0, 0x142409b50, 0x142624b70, 0x141834d18> as *const ()), // sub_1401ad930
    Replacement::draft(0x1401ada00, "unknown_1401ada00", fn_1401ada00 as *const ()), // sub_1401ada00
    Replacement::draft(0x1401ada50, "unknown_1401ada50", tpl_23::<0x143a8a590, 0x143a8a593, 0x1426cdcb0, 0x142409b50, 0x142624b90, 0x141834d18> as *const ()), // sub_1401ada50
    Replacement::draft(0x1401adab0, "unknown_1401adab0", tpl_23::<0x143a8a5d8, 0x143a8a5db, 0x1426cdcb0, 0x142409b50, 0x142624ba0, 0x141834d18> as *const ()), // sub_1401adab0
    Replacement::draft(0x1401adb80, "unknown_1401adb80", tpl_23::<0x143a8a600, 0x143a8a603, 0x1426cdcb0, 0x142409b50, 0x142624bc0, 0x141834d18> as *const ()), // sub_1401adb80
    Replacement::draft(0x1401adbe0, "unknown_1401adbe0", tpl_23::<0x143a8a648, 0x143a8a64b, 0x1426cdcb0, 0x142409b50, 0x142624bd0, 0x141834d18> as *const ()), // sub_1401adbe0
    Replacement::draft(0x1401adcb0, "unknown_1401adcb0", tpl_23::<0x143a8a670, 0x143a8a673, 0x1426cdcb0, 0x142409b50, 0x142624bf0, 0x141834d18> as *const ()), // sub_1401adcb0
    Replacement::draft(0x1401add10, "unknown_1401add10", tpl_23::<0x143a8a6b8, 0x143a8a6bb, 0x1426cdcb0, 0x142409b50, 0x142624c00, 0x141834d18> as *const ()), // sub_1401add10
    Replacement::draft(0x1401adde0, "unknown_1401adde0", tpl_23::<0x143a8a6e0, 0x143a8a6e3, 0x1426cdcb0, 0x142409b50, 0x142624c20, 0x141834d18> as *const ()), // sub_1401adde0
    Replacement::draft(0x1401ade40, "unknown_1401ade40", tpl_23::<0x143a8a728, 0x143a8a72b, 0x1426cdcb0, 0x142409b50, 0x142624c30, 0x141834d18> as *const ()), // sub_1401ade40
    Replacement::draft(0x1401adf10, "unknown_1401adf10", tpl_23::<0x143a8a7c0, 0x143a8a7c3, 0x1426cdcb0, 0x142409b50, 0x142624c50, 0x141834d18> as *const ()), // sub_1401adf10
    Replacement::draft(0x1401adf70, "unknown_1401adf70", tpl_23::<0x143a8a808, 0x143a8a80b, 0x1426cdcb0, 0x142409b50, 0x142624c60, 0x141834d18> as *const ()), // sub_1401adf70
    Replacement::draft_throws(0x1401ae5a0, "unknown_1401ae5a0", fn_1401ae5a0 as *const (), Throws::Caught), // sub_1401ae5a0
    Replacement::draft_throws(0x1401ae6d0, "unknown_1401ae6d0", fn_1401ae6d0 as *const (), Throws::Caught), // sub_1401ae6d0
    Replacement::draft_throws(0x1401ae800, "unknown_1401ae800", fn_1401ae800 as *const (), Throws::Caught), // sub_1401ae800
    Replacement::draft_throws(0x1401ae930, "unknown_1401ae930", fn_1401ae930 as *const (), Throws::Caught), // sub_1401ae930
    Replacement::draft_throws(0x1401aea50, "unknown_1401aea50", fn_1401aea50 as *const (), Throws::Caught), // sub_1401aea50
    Replacement::draft_throws(0x1401aeb70, "unknown_1401aeb70", fn_1401aeb70 as *const (), Throws::Caught), // sub_1401aeb70
    Replacement::draft_throws(0x1401aeca0, "unknown_1401aeca0", fn_1401aeca0 as *const (), Throws::Caught), // sub_1401aeca0
    Replacement::draft_throws(0x1401aedd0, "unknown_1401aedd0", fn_1401aedd0 as *const (), Throws::Caught), // sub_1401aedd0
    Replacement::draft_throws(0x1401aef00, "unknown_1401aef00", fn_1401aef00 as *const (), Throws::Caught), // sub_1401aef00
    Replacement::draft_throws(0x1401af030, "unknown_1401af030", fn_1401af030 as *const (), Throws::Caught), // sub_1401af030
    Replacement::draft_throws(0x1401af2e0, "unknown_1401af2e0", fn_1401af2e0 as *const (), Throws::Caught), // sub_1401af2e0
    Replacement::draft_throws(0x1401af440, "unknown_1401af440", fn_1401af440 as *const (), Throws::Caught), // sub_1401af440
    Replacement::draft_throws(0x1401af5f0, "unknown_1401af5f0", fn_1401af5f0 as *const (), Throws::Caught), // sub_1401af5f0
    Replacement::draft(0x1401af960, "unknown_1401af960", tpl_127::<0x143866c70, 0x143a8acb8, 0x140548a40, 0x142626410, 0x141834d18> as *const ()), // sub_1401af960
    Replacement::draft_throws(0x1401afe50, "unknown_1401afe50", fn_1401afe50 as *const (), Throws::Caught), // sub_1401afe50
    Replacement::draft_throws(0x1401aff80, "unknown_1401aff80", fn_1401aff80 as *const (), Throws::Caught), // sub_1401aff80
    Replacement::draft_throws(0x1401b0180, "unknown_1401b0180", fn_1401b0180 as *const (), Throws::Caught), // sub_1401b0180
    Replacement::draft_throws(0x1401b02b0, "unknown_1401b02b0", fn_1401b02b0 as *const (), Throws::Caught), // sub_1401b02b0
    Replacement::draft_throws(0x1401b03e0, "unknown_1401b03e0", fn_1401b03e0 as *const (), Throws::Caught), // sub_1401b03e0
    Replacement::draft_throws(0x1401b0510, "unknown_1401b0510", fn_1401b0510 as *const (), Throws::Caught), // sub_1401b0510
    Replacement::draft_throws(0x1401b06d0, "unknown_1401b06d0", fn_1401b06d0 as *const (), Throws::Caught), // sub_1401b06d0
    Replacement::draft_throws(0x1401b0890, "unknown_1401b0890", fn_1401b0890 as *const (), Throws::Caught), // sub_1401b0890
    Replacement::draft_throws(0x1401b09c0, "unknown_1401b09c0", fn_1401b09c0 as *const (), Throws::Caught), // sub_1401b09c0
    Replacement::draft_throws(0x1401b0af0, "unknown_1401b0af0", fn_1401b0af0 as *const (), Throws::Caught), // sub_1401b0af0
    Replacement::draft_throws(0x1401b0c10, "unknown_1401b0c10", fn_1401b0c10 as *const (), Throws::Caught), // sub_1401b0c10
    Replacement::draft_throws(0x1401b0d30, "unknown_1401b0d30", fn_1401b0d30 as *const (), Throws::Caught), // sub_1401b0d30
    Replacement::draft_throws(0x1401b0ec0, "unknown_1401b0ec0", fn_1401b0ec0 as *const (), Throws::Caught), // sub_1401b0ec0
    Replacement::draft_throws(0x1401b1050, "unknown_1401b1050", fn_1401b1050 as *const (), Throws::Caught), // sub_1401b1050
    Replacement::draft_throws(0x1401b1180, "unknown_1401b1180", fn_1401b1180 as *const (), Throws::Caught), // sub_1401b1180
    Replacement::draft_throws(0x1401b12f0, "unknown_1401b12f0", fn_1401b12f0 as *const (), Throws::Caught), // sub_1401b12f0
    Replacement::draft(0x1401b17e0, "unknown_1401b17e0", fn_1401b17e0 as *const ()), // sub_1401b17e0
    Replacement::draft(0x1401b18b0, "unknown_1401b18b0", fn_1401b18b0 as *const ()), // sub_1401b18b0
    Replacement::draft(0x1401b1a10, "unknown_1401b1a10", tpl_256::<0x143867ec0, 0x143bbce10, 0x140548a40, 0x142628450, 0x141834d18> as *const ()), // sub_1401b1a10
    Replacement::draft(0x1401b1ec0, "unknown_1401b1ec0", tpl_127::<0x143867ea0, 0x143bbce88, 0x140548a40, 0x142628e20, 0x141834d18> as *const ()), // sub_1401b1ec0
    Replacement::draft(0x1401b1fb0, "unknown_1401b1fb0", tpl_127::<0x143868120, 0x143bbce38, 0x140548a40, 0x142628f60, 0x141834d18> as *const ()), // sub_1401b1fb0
    Replacement::draft(0x1401b2030, "unknown_1401b2030", tpl_127::<0x143867fe8, 0x143bbce60, 0x140548a40, 0x1426296e0, 0x141834d18> as *const ()), // sub_1401b2030
    Replacement::draft_throws(0x1401b2820, "unknown_1401b2820", fn_1401b2820 as *const (), Throws::Caught), // sub_1401b2820
    Replacement::draft(0x1401b2a10, "unknown_1401b2a10", tpl_161::<0x143841610, 0x143bbcfb8, 0x140548a40, 0x14262aae0, 0x141834d18> as *const ()), // sub_1401b2a10
    Replacement::draft(0x1401b2b00, "unknown_1401b2b00", tpl_127::<0x1438415d0, 0x143bbcf90, 0x140548a40, 0x14262acc0, 0x141834d18> as *const ()), // sub_1401b2b00
    Replacement::draft_throws(0x1401b2c30, "unknown_1401b2c30", fn_1401b2c30 as *const (), Throws::Caught), // sub_1401b2c30
    Replacement::draft_throws(0x1401b2d50, "unknown_1401b2d50", fn_1401b2d50 as *const (), Throws::Caught), // sub_1401b2d50
    Replacement::draft_throws(0x1401b2ee0, "unknown_1401b2ee0", fn_1401b2ee0 as *const (), Throws::Caught), // sub_1401b2ee0
    Replacement::draft(0x1401b3080, "unknown_1401b3080", fn_1401b3080 as *const ()), // sub_1401b3080
    Replacement::draft_throws(0x1401b30d0, "unknown_1401b30d0", fn_1401b30d0 as *const (), Throws::Caught), // sub_1401b30d0
    Replacement::draft_throws(0x1401b31f0, "unknown_1401b31f0", fn_1401b31f0 as *const (), Throws::Caught), // sub_1401b31f0
    Replacement::draft_throws(0x1401b3310, "unknown_1401b3310", fn_1401b3310 as *const (), Throws::Caught), // sub_1401b3310
    Replacement::draft_throws(0x1401b3430, "unknown_1401b3430", fn_1401b3430 as *const (), Throws::Caught), // sub_1401b3430
    Replacement::draft_throws(0x1401b3550, "unknown_1401b3550", fn_1401b3550 as *const (), Throws::Caught), // sub_1401b3550
    Replacement::draft_throws(0x1401b3670, "unknown_1401b3670", fn_1401b3670 as *const (), Throws::Caught), // sub_1401b3670
    Replacement::draft_throws(0x1401b3790, "unknown_1401b3790", fn_1401b3790 as *const (), Throws::Caught), // sub_1401b3790
    Replacement::draft_throws(0x1401b38b0, "unknown_1401b38b0", fn_1401b38b0 as *const (), Throws::Caught), // sub_1401b38b0
    Replacement::draft_throws(0x1401b39d0, "unknown_1401b39d0", fn_1401b39d0 as *const (), Throws::Caught), // sub_1401b39d0
    Replacement::draft_throws(0x1401b3af0, "unknown_1401b3af0", fn_1401b3af0 as *const (), Throws::Caught), // sub_1401b3af0
    Replacement::draft_throws(0x1401b3c10, "unknown_1401b3c10", fn_1401b3c10 as *const (), Throws::Caught), // sub_1401b3c10
    Replacement::draft_throws(0x1401b3d30, "unknown_1401b3d30", fn_1401b3d30 as *const (), Throws::Caught), // sub_1401b3d30
    Replacement::draft_throws(0x1401b3e50, "unknown_1401b3e50", fn_1401b3e50 as *const (), Throws::Caught), // sub_1401b3e50
    Replacement::draft_throws(0x1401b3f70, "unknown_1401b3f70", fn_1401b3f70 as *const (), Throws::Caught), // sub_1401b3f70
    Replacement::draft_throws(0x1401b4090, "unknown_1401b4090", fn_1401b4090 as *const (), Throws::Caught), // sub_1401b4090
    Replacement::draft_throws(0x1401b41b0, "unknown_1401b41b0", fn_1401b41b0 as *const (), Throws::Caught), // sub_1401b41b0
    Replacement::draft_throws(0x1401b42d0, "unknown_1401b42d0", fn_1401b42d0 as *const (), Throws::Caught), // sub_1401b42d0
    Replacement::draft_throws(0x1401b43f0, "unknown_1401b43f0", fn_1401b43f0 as *const (), Throws::Caught), // sub_1401b43f0
    Replacement::draft_throws(0x1401b4510, "unknown_1401b4510", fn_1401b4510 as *const (), Throws::Caught), // sub_1401b4510
    Replacement::draft_throws(0x1401b4630, "unknown_1401b4630", fn_1401b4630 as *const (), Throws::Caught), // sub_1401b4630
    Replacement::draft(0x1401b48d0, "unknown_1401b48d0", fn_1401b48d0 as *const ()), // sub_1401b48d0
    Replacement::draft(0x1401b52d0, "unknown_1401b52d0", tpl_127::<0x143869a90, 0x143bbd2d8, 0x140548a40, 0x14262cce0, 0x141834d18> as *const ()), // sub_1401b52d0
    Replacement::draft(0x1401b5300, "unknown_1401b5300", tpl_256::<0x14386a370, 0x143bbd300, 0x140548a40, 0x14262cd10, 0x141834d18> as *const ()), // sub_1401b5300
    Replacement::draft_throws(0x1401b5330, "unknown_1401b5330", fn_1401b5330 as *const (), Throws::Caught), // sub_1401b5330
    Replacement::draft_throws(0x1401b5460, "unknown_1401b5460", fn_1401b5460 as *const (), Throws::Caught), // sub_1401b5460
    Replacement::draft_throws(0x1401b5590, "unknown_1401b5590", fn_1401b5590 as *const (), Throws::Caught), // sub_1401b5590
    Replacement::draft_throws(0x1401b56c0, "unknown_1401b56c0", fn_1401b56c0 as *const (), Throws::Caught), // sub_1401b56c0
    Replacement::draft_throws(0x1401b57f0, "unknown_1401b57f0", fn_1401b57f0 as *const (), Throws::Caught), // sub_1401b57f0
    Replacement::draft_throws(0x1401b5910, "unknown_1401b5910", fn_1401b5910 as *const (), Throws::Caught), // sub_1401b5910
    Replacement::draft(0x1401b5ff0, "unknown_1401b5ff0", tpl_256::<0x1438698c0, 0x143bbd108, 0x140548a40, 0x14262d020, 0x141834d18> as *const ()), // sub_1401b5ff0
    Replacement::draft(0x1401b6420, "unknown_1401b6420", tpl_127::<0x14386a638, 0x143bbd328, 0x140548a40, 0x14262f000, 0x141834d18> as *const ()), // sub_1401b6420
    Replacement::draft(0x1401b65d0, "unknown_1401b65d0", tpl_166::<0x1426456f8, 0x143c4a520, 0x142406920, 0x143c4a528> as *const ()), // sub_1401b65d0
    Replacement::draft(0x1401b71a0, "unknown_1401b71a0", tpl_127::<0x143841648, 0x143bbd370, 0x140548a40, 0x14262fdd0, 0x141834d18> as *const ()), // sub_1401b71a0
    Replacement::draft(0x1401b8bb0, "unknown_1401b8bb0", tpl_161::<0x14386b930, 0x143bbd3c0, 0x140548a40, 0x1426330f0, 0x141834d18> as *const ()), // sub_1401b8bb0
    Replacement::draft(0x1401b8be0, "unknown_1401b8be0", tpl_127::<0x14386b930, 0x143bbd400, 0x140548a40, 0x142633180, 0x141834d18> as *const ()), // sub_1401b8be0
    Replacement::draft(0x1401b8c10, "unknown_1401b8c10", tpl_127::<0x14386b930, 0x143bbd398, 0x140548a40, 0x142633210, 0x141834d18> as *const ()), // sub_1401b8c10
    Replacement::draft(0x1401b8cc0, "unknown_1401b8cc0", fn_1401b8cc0 as *const ()), // sub_1401b8cc0
    Replacement::draft(0x1401b8ce0, "unknown_1401b8ce0", fn_1401b8ce0 as *const ()), // sub_1401b8ce0
    Replacement::draft(0x1401b8d00, "unknown_1401b8d00", tpl_278::<0x14386bb00, 0x14386bac0, 0x1424b16a0, 0x14397db80, 0x143a2afec, 0x1, 0x143a2afe8, 0x14386bae0, 0x143a49130, 0x143857768, 0x20, 0x14386bb20, 0x142633390, 0x141834d18> as *const ()), // sub_1401b8d00
    Replacement::draft(0x1401b8de0, "unknown_1401b8de0", tpl_278::<0x14386b9b0, 0x14386b970, 0x1424b16a0, 0x14397db80, 0x143a2afec, 0x1, 0x143a2afe8, 0x14386b990, 0x143a49130, 0x14385586c, 0x20, 0x14386b9d0, 0x1426333f0, 0x141834d18> as *const ()), // sub_1401b8de0
    Replacement::draft(0x1401b8ec0, "unknown_1401b8ec0", tpl_278::<0x14386ba20, 0x14386b9e0, 0x1424b16a0, 0x14397db80, 0x143a2afec, 0x1, 0x143a2afe8, 0x14386ba00, 0x143a49130, 0x14385602c, 0x20, 0x14386ba40, 0x142633450, 0x141834d18> as *const ()), // sub_1401b8ec0
    Replacement::draft(0x1401b8fa0, "unknown_1401b8fa0", tpl_278::<0x14386ba90, 0x14386ba50, 0x1424b16a0, 0x14397db80, 0x143a2afec, 0x1, 0x143a2afe8, 0x14386ba70, 0x143a49130, 0x143856028, 0x20, 0x14386bab0, 0x1426334b0, 0x141834d18> as *const ()), // sub_1401b8fa0
    Replacement::draft(0x1401b9330, "unknown_1401b9330", tpl_127::<0x143841668, 0x143bbd460, 0x140548a40, 0x142633b50, 0x141834d18> as *const ()), // sub_1401b9330
    Replacement::draft(0x1401b9360, "unknown_1401b9360", tpl_161::<0x143841688, 0x143bbd488, 0x140548a40, 0x142633b80, 0x141834d18> as *const ()), // sub_1401b9360
    Replacement::draft(0x1401b9b10, "unknown_1401b9b10", tpl_161::<0x14386be20, 0x143bbd4b0, 0x140548a40, 0x142634c10, 0x141834d18> as *const ()), // sub_1401b9b10
    Replacement::draft(0x1401ba600, "unknown_1401ba600", fn_1401ba600 as *const ()), // sub_1401ba600
    Replacement::draft(0x1401ba9e0, "unknown_1401ba9e0", tpl_127::<0x14386c448, 0x143bbd788, 0x140548a40, 0x142636c10, 0x141834d18> as *const ()), // sub_1401ba9e0
    Replacement::draft(0x1401baa10, "unknown_1401baa10", tpl_127::<0x14386c448, 0x143bbd7d8, 0x140548a40, 0x142636ca0, 0x141834d18> as *const ()), // sub_1401baa10
    Replacement::draft(0x1401baa80, "unknown_1401baa80", tpl_127::<0x14386c448, 0x143bbd7b0, 0x140548a40, 0x142636dc0, 0x141834d18> as *const ()), // sub_1401baa80
    Replacement::draft(0x1401baac0, "unknown_1401baac0", tpl_127::<0x14386c438, 0x143bbd800, 0x140548a40, 0x142636e70, 0x141834d18> as *const ()), // sub_1401baac0
    Replacement::draft(0x1401bae60, "unknown_1401bae60", tpl_127::<0x14386c690, 0x143bbd960, 0x140548a40, 0x142637b40, 0x141834d18> as *const ()), // sub_1401bae60
    Replacement::draft(0x1401bae90, "unknown_1401bae90", tpl_127::<0x14386c728, 0x143bbd938, 0x140548a40, 0x142637bd0, 0x141834d18> as *const ()), // sub_1401bae90
    Replacement::draft(0x1401baee0, "unknown_1401baee0", tpl_280::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143bbda10, 0x142637e70, 0x141834d18> as *const ()), // sub_1401baee0
    Replacement::draft(0x1401baf80, "unknown_1401baf80", fn_1401baf80 as *const ()), // sub_1401baf80
    Replacement::draft(0x1401bb090, "unknown_1401bb090", fn_1401bb090 as *const ()), // sub_1401bb090
    Replacement::draft(0x1401bb160, "unknown_1401bb160", fn_1401bb160 as *const ()), // sub_1401bb160
    Replacement::draft(0x1401bb540, "unknown_1401bb540", tpl_284::<0x143a001ac, 0x143a00290, 0x14386c810, 0x142638230, 0x141834d18> as *const ()), // sub_1401bb540
    Replacement::draft(0x1401bb940, "unknown_1401bb940", tpl_161::<0x1438416e8, 0x143bbdda0, 0x140548a40, 0x142638650, 0x141834d18> as *const ()), // sub_1401bb940
    Replacement::draft(0x1401bbc00, "unknown_1401bbc00", tpl_127::<0x14386c8f8, 0x143bbdd78, 0x140548a40, 0x142638740, 0x141834d18> as *const ()), // sub_1401bbc00
    Replacement::draft(0x1401bbf60, "unknown_1401bbf60", fn_1401bbf60 as *const ()), // sub_1401bbf60
    Replacement::draft_throws(0x1401bd9e0, "unknown_1401bd9e0", fn_1401bd9e0 as *const (), Throws::Caught), // sub_1401bd9e0
    Replacement::draft_throws(0x1401bdb00, "unknown_1401bdb00", fn_1401bdb00 as *const (), Throws::Caught), // sub_1401bdb00
    Replacement::draft_throws(0x1401bdc20, "unknown_1401bdc20", fn_1401bdc20 as *const (), Throws::Caught), // sub_1401bdc20
    Replacement::draft_throws(0x1401bdd50, "unknown_1401bdd50", fn_1401bdd50 as *const (), Throws::Caught), // sub_1401bdd50
    Replacement::draft_throws(0x1401bde80, "unknown_1401bde80", fn_1401bde80 as *const (), Throws::Caught), // sub_1401bde80
    Replacement::draft_throws(0x1401bdfa0, "unknown_1401bdfa0", fn_1401bdfa0 as *const (), Throws::Caught), // sub_1401bdfa0
    Replacement::draft_throws(0x1401be0c0, "unknown_1401be0c0", fn_1401be0c0 as *const (), Throws::Caught), // sub_1401be0c0
    Replacement::draft_throws(0x1401be1e0, "unknown_1401be1e0", fn_1401be1e0 as *const (), Throws::Caught), // sub_1401be1e0
    Replacement::draft(0x1401be430, "unknown_1401be430", tpl_127::<0x14386e470, 0x143bbdde8, 0x140548a40, 0x142639f30, 0x141834d18> as *const ()), // sub_1401be430
    Replacement::draft_throws(0x1401beb00, "unknown_1401beb00", fn_1401beb00 as *const (), Throws::Caught), // sub_1401beb00
    Replacement::draft_throws(0x1401bec10, "unknown_1401bec10", fn_1401bec10 as *const (), Throws::Caught), // sub_1401bec10
    Replacement::draft_throws(0x1401bed10, "unknown_1401bed10", fn_1401bed10 as *const (), Throws::Caught), // sub_1401bed10
    Replacement::draft_throws(0x1401bee20, "unknown_1401bee20", fn_1401bee20 as *const (), Throws::Caught), // sub_1401bee20
    Replacement::draft_throws(0x1401bef20, "unknown_1401bef20", fn_1401bef20 as *const (), Throws::Caught), // sub_1401bef20
    Replacement::draft_throws(0x1401bf020, "unknown_1401bf020", fn_1401bf020 as *const (), Throws::Caught), // sub_1401bf020
    Replacement::draft_throws(0x1401bf120, "unknown_1401bf120", fn_1401bf120 as *const (), Throws::Caught), // sub_1401bf120
    Replacement::draft_throws(0x1401bf220, "unknown_1401bf220", fn_1401bf220 as *const (), Throws::Caught), // sub_1401bf220
    Replacement::draft_throws(0x1401bf320, "unknown_1401bf320", fn_1401bf320 as *const (), Throws::Caught), // sub_1401bf320
    Replacement::draft_throws(0x1401bf420, "unknown_1401bf420", fn_1401bf420 as *const (), Throws::Caught), // sub_1401bf420
    Replacement::draft_throws(0x1401bf520, "unknown_1401bf520", fn_1401bf520 as *const (), Throws::Caught), // sub_1401bf520
    Replacement::draft_throws(0x1401bf620, "unknown_1401bf620", fn_1401bf620 as *const (), Throws::Caught), // sub_1401bf620
    Replacement::draft_throws(0x1401bf720, "unknown_1401bf720", fn_1401bf720 as *const (), Throws::Caught), // sub_1401bf720
    Replacement::draft_throws(0x1401bf820, "unknown_1401bf820", fn_1401bf820 as *const (), Throws::Caught), // sub_1401bf820
    Replacement::draft_throws(0x1401bf920, "unknown_1401bf920", fn_1401bf920 as *const (), Throws::Caught), // sub_1401bf920
    Replacement::draft_throws(0x1401bfa20, "unknown_1401bfa20", fn_1401bfa20 as *const (), Throws::Caught), // sub_1401bfa20
    Replacement::draft_throws(0x1401bfb20, "unknown_1401bfb20", fn_1401bfb20 as *const (), Throws::Caught), // sub_1401bfb20
    Replacement::draft_throws(0x1401bfc20, "unknown_1401bfc20", fn_1401bfc20 as *const (), Throws::Caught), // sub_1401bfc20
    Replacement::draft_throws(0x1401bfd20, "unknown_1401bfd20", fn_1401bfd20 as *const (), Throws::Caught), // sub_1401bfd20
    Replacement::draft_throws(0x1401bfe20, "unknown_1401bfe20", fn_1401bfe20 as *const (), Throws::Caught), // sub_1401bfe20
    Replacement::draft_throws(0x1401bff20, "unknown_1401bff20", fn_1401bff20 as *const (), Throws::Caught), // sub_1401bff20
    Replacement::draft_throws(0x1401c0020, "unknown_1401c0020", fn_1401c0020 as *const (), Throws::Caught), // sub_1401c0020
    Replacement::draft_throws(0x1401c0120, "unknown_1401c0120", fn_1401c0120 as *const (), Throws::Caught), // sub_1401c0120
    Replacement::draft_throws(0x1401c0260, "unknown_1401c0260", fn_1401c0260 as *const (), Throws::Caught), // sub_1401c0260
    Replacement::draft_throws(0x1401c0420, "unknown_1401c0420", fn_1401c0420 as *const (), Throws::Caught), // sub_1401c0420
    Replacement::draft_throws(0x1401c0540, "unknown_1401c0540", fn_1401c0540 as *const (), Throws::Caught), // sub_1401c0540
    Replacement::draft(0x1401c0a70, "unknown_1401c0a70", tpl_127::<0x14386fb28, 0x143bbdf80, 0x140548a40, 0x14263cf40, 0x141834d18> as *const ()), // sub_1401c0a70
    Replacement::draft_throws(0x1401c0aa0, "unknown_1401c0aa0", fn_1401c0aa0 as *const (), Throws::Caught), // sub_1401c0aa0
    Replacement::draft_throws(0x1401c0bd0, "unknown_1401c0bd0", fn_1401c0bd0 as *const (), Throws::Caught), // sub_1401c0bd0
    Replacement::draft_throws(0x1401c0d00, "unknown_1401c0d00", fn_1401c0d00 as *const (), Throws::Caught), // sub_1401c0d00
    Replacement::draft_throws(0x1401c0e30, "unknown_1401c0e30", fn_1401c0e30 as *const (), Throws::Caught), // sub_1401c0e30
    Replacement::draft_throws(0x1401c0f60, "unknown_1401c0f60", fn_1401c0f60 as *const (), Throws::Caught), // sub_1401c0f60
    Replacement::draft_throws(0x1401c1090, "unknown_1401c1090", fn_1401c1090 as *const (), Throws::Caught), // sub_1401c1090
    Replacement::draft_throws(0x1401c11c0, "unknown_1401c11c0", fn_1401c11c0 as *const (), Throws::Caught), // sub_1401c11c0
    Replacement::draft_throws(0x1401c12e0, "unknown_1401c12e0", fn_1401c12e0 as *const (), Throws::Caught), // sub_1401c12e0
    Replacement::draft_throws(0x1401c1400, "unknown_1401c1400", fn_1401c1400 as *const (), Throws::Caught), // sub_1401c1400
    Replacement::draft_throws(0x1401c1530, "unknown_1401c1530", fn_1401c1530 as *const (), Throws::Caught), // sub_1401c1530
    Replacement::draft_throws(0x1401c1650, "unknown_1401c1650", fn_1401c1650 as *const (), Throws::Caught), // sub_1401c1650
    Replacement::draft_throws(0x1401c1780, "unknown_1401c1780", fn_1401c1780 as *const (), Throws::Caught), // sub_1401c1780
    Replacement::draft_throws(0x1401c18d0, "unknown_1401c18d0", fn_1401c18d0 as *const (), Throws::Caught), // sub_1401c18d0
    Replacement::draft_throws(0x1401c1a00, "unknown_1401c1a00", fn_1401c1a00 as *const (), Throws::Caught), // sub_1401c1a00
    Replacement::draft(0x1401c20b0, "unknown_1401c20b0", tpl_56::<0x143426760, 0x143426564, 0x143a001ac, 0x143870d50, 0x143870d68, 0x143a00290, 0x143870d40, 0x14263e240, 0x141834d18> as *const ()), // sub_1401c20b0
    Replacement::draft_throws(0x1401c2260, "unknown_1401c2260", fn_1401c2260 as *const (), Throws::Caught), // sub_1401c2260
    Replacement::draft_throws(0x1401c2380, "unknown_1401c2380", fn_1401c2380 as *const (), Throws::Caught), // sub_1401c2380
    Replacement::draft_throws(0x1401c24b0, "unknown_1401c24b0", fn_1401c24b0 as *const (), Throws::Caught), // sub_1401c24b0
    Replacement::draft_throws(0x1401c25e0, "unknown_1401c25e0", fn_1401c25e0 as *const (), Throws::Caught), // sub_1401c25e0
    Replacement::draft_throws(0x1401c2710, "unknown_1401c2710", fn_1401c2710 as *const (), Throws::Caught), // sub_1401c2710
    Replacement::draft_throws(0x1401c2840, "unknown_1401c2840", fn_1401c2840 as *const (), Throws::Caught), // sub_1401c2840
    Replacement::draft_throws(0x1401c2970, "unknown_1401c2970", fn_1401c2970 as *const (), Throws::Caught), // sub_1401c2970
    Replacement::draft_throws(0x1401c2aa0, "unknown_1401c2aa0", fn_1401c2aa0 as *const (), Throws::Caught), // sub_1401c2aa0
    Replacement::draft_throws(0x1401c2bd0, "unknown_1401c2bd0", fn_1401c2bd0 as *const (), Throws::Caught), // sub_1401c2bd0
    Replacement::draft_throws(0x1401c2d00, "unknown_1401c2d00", fn_1401c2d00 as *const (), Throws::Caught), // sub_1401c2d00
    Replacement::draft(0x1401c2e30, "unknown_1401c2e30", tpl_317::<0x1401e1800, 0x140200580, 0x143bf6108, 0x141834fc0, 0x14263e8b0, 0x143bfe108, 0x143bfe110, 0x143bfe118, 0x143bfe120, 0x141834d18> as *const ()), // sub_1401c2e30
    Replacement::draft(0x1401c2e90, "unknown_1401c2e90", tpl_317::<0x1401e1800, 0x140200580, 0x143bbdfb8, 0x141834fc0, 0x14263e8f0, 0x143bc5fb8, 0x143bc5fc0, 0x143bc5fc8, 0x143bc5fd0, 0x141834d18> as *const ()), // sub_1401c2e90
    Replacement::draft(0x1401c2ef0, "unknown_1401c2ef0", tpl_317::<0x1401e1800, 0x140200580, 0x143bde078, 0x141834fc0, 0x14263e930, 0x143be6078, 0x143be6080, 0x143be6088, 0x143be6090, 0x141834d18> as *const ()), // sub_1401c2ef0
    Replacement::draft(0x1401c2f50, "unknown_1401c2f50", tpl_317::<0x1401e1800, 0x140200580, 0x143bce018, 0x141834fc0, 0x14263e970, 0x143bd6018, 0x143bd6020, 0x143bd6028, 0x143bd6030, 0x141834d18> as *const ()), // sub_1401c2f50
    Replacement::draft(0x1401c2fb0, "unknown_1401c2fb0", tpl_317::<0x1401e1800, 0x140200580, 0x143bee0d8, 0x141834fc0, 0x14263e9b0, 0x143bf60d8, 0x143bf60e0, 0x143bf60e8, 0x143bf60f0, 0x141834d18> as *const ()), // sub_1401c2fb0
    Replacement::draft(0x1401c3010, "unknown_1401c3010", tpl_317::<0x1401e1800, 0x140200580, 0x143c0e198, 0x141834fc0, 0x14263e9f0, 0x143c16198, 0x143c161a0, 0x143c161a8, 0x143c161b0, 0x141834d18> as *const ()), // sub_1401c3010
    Replacement::draft(0x1401c3070, "unknown_1401c3070", tpl_317::<0x1401e1800, 0x140200580, 0x143be60a8, 0x141834fc0, 0x14263ea30, 0x143bee0a8, 0x143bee0b0, 0x143bee0b8, 0x143bee0c0, 0x141834d18> as *const ()), // sub_1401c3070
    Replacement::draft(0x1401c30d0, "unknown_1401c30d0", tpl_317::<0x1401e1800, 0x140200580, 0x143c06168, 0x141834fc0, 0x14263ea70, 0x143c0e168, 0x143c0e170, 0x143c0e178, 0x143c0e180, 0x141834d18> as *const ()), // sub_1401c30d0
    Replacement::draft(0x1401c3130, "unknown_1401c3130", tpl_317::<0x1401e1800, 0x140200580, 0x143bc5fe8, 0x141834fc0, 0x14263eab0, 0x143bcdfe8, 0x143bcdff0, 0x143bcdff8, 0x143bce000, 0x141834d18> as *const ()), // sub_1401c3130
    Replacement::draft(0x1401c3190, "unknown_1401c3190", tpl_317::<0x1401e1800, 0x140200580, 0x143bd6048, 0x141834fc0, 0x14263eaf0, 0x143bde048, 0x143bde050, 0x143bde058, 0x143bde060, 0x141834d18> as *const ()), // sub_1401c3190
    Replacement::draft(0x1401c31f0, "unknown_1401c31f0", tpl_317::<0x1401e1800, 0x140200580, 0x143bfe138, 0x141834fc0, 0x14263eb30, 0x143c06138, 0x143c06140, 0x143c06148, 0x143c06150, 0x141834d18> as *const ()), // sub_1401c31f0
    Replacement::draft_throws(0x1401c3450, "unknown_1401c3450", fn_1401c3450 as *const (), Throws::Caught), // sub_1401c3450
    Replacement::draft_throws(0x1401c3980, "unknown_1401c3980", fn_1401c3980 as *const (), Throws::Caught), // sub_1401c3980
    Replacement::draft(0x1401c3cc0, "unknown_1401c3cc0", tpl_158::<0x143c164e4, 0x143c164f0, 0x1424aff00, 0x14324f850, 0x143c164b0, 0x143c16528, 0x143409108, 0x143c16530> as *const ()), // sub_1401c3cc0
    Replacement::draft(0x1401c3d70, "unknown_1401c3d70", tpl_158::<0x143c165a4, 0x143c165b0, 0x1424aff00, 0x14324f850, 0x143c16570, 0x143c165e8, 0x143409e08, 0x143c165f0> as *const ()), // sub_1401c3d70
    Replacement::draft(0x1401c3f10, "unknown_1401c3f10", tpl_158::<0x143c1681c, 0x143c16730, 0x1424aff00, 0x14324f850, 0x143c167e8, 0x143c16768, 0x14340a568, 0x143c16770> as *const ()), // sub_1401c3f10
    Replacement::draft(0x1401c3f60, "unknown_1401c3f60", tpl_158::<0x143c167e4, 0x143c166e0, 0x1424aff00, 0x14324f850, 0x143c167b0, 0x143c16718, 0x14340a448, 0x143c16720> as *const ()), // sub_1401c3f60
    Replacement::draft(0x1401c3fc0, "unknown_1401c3fc0", fn_1401c3fc0 as *const ()), // sub_1401c3fc0
    Replacement::draft(0x1401c4700, "unknown_1401c4700", fn_1401c4700 as *const ()), // sub_1401c4700
    Replacement::draft(0x1401c4880, "unknown_1401c4880", tpl_280::<0xb0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143c168a8, 0x142640250, 0x141834d18> as *const ()), // sub_1401c4880
    Replacement::draft(0x1401c4920, "unknown_1401c4920", tpl_219::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143c168b8, 0x1426403c0, 0x141834d18> as *const ()), // sub_1401c4920
    Replacement::draft(0x1401c4ab0, "unknown_1401c4ab0", fn_1401c4ab0 as *const ()), // sub_1401c4ab0
    Replacement::draft(0x1401c4c40, "unknown_1401c4c40", fn_1401c4c40 as *const ()), // sub_1401c4c40
    Replacement::draft(0x1401c4ca0, "unknown_1401c4ca0", fn_1401c4ca0 as *const ()), // sub_1401c4ca0
    Replacement::draft(0x1401cb370, "unknown_1401cb370", fn_1401cb370 as *const ()), // sub_1401cb370
    Replacement::draft(0x1401cb3d0, "unknown_1401cb3d0", fn_1401cb3d0 as *const ()), // sub_1401cb3d0
    Replacement::draft(0x1401cb400, "unknown_1401cb400", fn_1401cb400 as *const ()), // sub_1401cb400
    Replacement::draft(0x1401cb580, "unknown_1401cb580", tpl_327::<0x142640c60, 0x141834d18> as *const ()), // sub_1401cb580
    Replacement::draft(0x1401cb590, "unknown_1401cb590", tpl_327::<0x142640c70, 0x141834d18> as *const ()), // sub_1401cb590
    Replacement::draft(0x1401cb5a0, "unknown_1401cb5a0", tpl_327::<0x142640c90, 0x141834d18> as *const ()), // sub_1401cb5a0
    Replacement::draft(0x1401cb5b0, "unknown_1401cb5b0", tpl_284::<0x143a001ac, 0x143a00290, 0x1438717a0, 0x142640ca0, 0x141834d18> as *const ()), // sub_1401cb5b0
    Replacement::draft(0x1401cb5f0, "unknown_1401cb5f0", fn_1401cb5f0 as *const ()), // sub_1401cb5f0
    Replacement::draft(0x1401cb680, "unknown_1401cb680", fn_1401cb680 as *const ()), // sub_1401cb680
    Replacement::draft(0x1401cb740, "unknown_1401cb740", tpl_284::<0x143a001ac, 0x143a00290, 0x1438717d0, 0x142640ef0, 0x141834d18> as *const ()), // sub_1401cb740
    Replacement::draft(0x1401cb780, "unknown_1401cb780", tpl_284::<0x143a001ac, 0x143a00290, 0x1438717b8, 0x142640f80, 0x141834d18> as *const ()), // sub_1401cb780
    Replacement::draft_throws(0x1401cb7c0, "ak_1401cb7c0", fn_1401cb7c0 as *const (), Throws::Caught), // sub_1401cb7c0
    Replacement::draft(0x1401cc170, "unknown_1401cc170", fn_1401cc170 as *const ()), // sub_1401cc170
    Replacement::draft(0x1401cc760, "unknown_1401cc760", tpl_206::<0x1401fc200, 0x8, 0x143c4cf08, 0x143c4cf18, 0x143c4cf20, 0x143c4cf30, 0x143c4cf38, 0x1434268d0, 0x143c4cf00, 0x1401e5730, 0x142641b50, 0x141834d18> as *const ()), // sub_1401cc760
    Replacement::draft(0x1401ccfb0, "unknown_1401ccfb0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872800, 0x1426428b0, 0x141834d18> as *const ()), // sub_1401ccfb0
    Replacement::draft(0x1401ccff0, "unknown_1401ccff0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872820, 0x142642940, 0x141834d18> as *const ()), // sub_1401ccff0
    Replacement::draft(0x1401cd030, "unknown_1401cd030", tpl_284::<0x143a001ac, 0x143a00290, 0x143872878, 0x1426429d0, 0x141834d18> as *const ()), // sub_1401cd030
    Replacement::draft(0x1401cd070, "unknown_1401cd070", tpl_284::<0x143a001ac, 0x143a00290, 0x1438728e0, 0x142642a60, 0x141834d18> as *const ()), // sub_1401cd070
    Replacement::draft(0x1401cd0b0, "unknown_1401cd0b0", tpl_284::<0x143a001ac, 0x143a00290, 0x1438728b0, 0x142642af0, 0x141834d18> as *const ()), // sub_1401cd0b0
    Replacement::draft(0x1401cd0f0, "unknown_1401cd0f0", tpl_284::<0x143a001ac, 0x143a00290, 0x1438728c8, 0x142642b80, 0x141834d18> as *const ()), // sub_1401cd0f0
    Replacement::draft(0x1401cd130, "unknown_1401cd130", tpl_284::<0x143a001ac, 0x143a00290, 0x143872840, 0x142642c10, 0x141834d18> as *const ()), // sub_1401cd130
    Replacement::draft(0x1401cd170, "unknown_1401cd170", tpl_284::<0x143a001ac, 0x143a00290, 0x143872890, 0x142642ca0, 0x141834d18> as *const ()), // sub_1401cd170
    Replacement::draft(0x1401cd1b0, "unknown_1401cd1b0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872858, 0x142642d30, 0x141834d18> as *const ()), // sub_1401cd1b0
    Replacement::draft(0x1401cd1f0, "unknown_1401cd1f0", tpl_284::<0x143a001ac, 0x143a00290, 0x1438728f8, 0x142642dc0, 0x141834d18> as *const ()), // sub_1401cd1f0
    Replacement::draft(0x1401cd230, "unknown_1401cd230", fn_1401cd230 as *const ()), // sub_1401cd230
    Replacement::draft(0x1401cd270, "unknown_1401cd270", tpl_284::<0x143a001ac, 0x143a00290, 0x143872948, 0x142642e80, 0x141834d18> as *const ()), // sub_1401cd270
    Replacement::draft(0x1401cd2b0, "unknown_1401cd2b0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872980, 0x142642f10, 0x141834d18> as *const ()), // sub_1401cd2b0
    Replacement::draft(0x1401cd2f0, "unknown_1401cd2f0", tpl_284::<0x143a001ac, 0x143a00290, 0x1438729d0, 0x142642fa0, 0x141834d18> as *const ()), // sub_1401cd2f0
    Replacement::draft(0x1401cd330, "unknown_1401cd330", tpl_284::<0x143a001ac, 0x143a00290, 0x1438729b8, 0x142643030, 0x141834d18> as *const ()), // sub_1401cd330
    Replacement::draft(0x1401cd370, "unknown_1401cd370", tpl_284::<0x143a001ac, 0x143a00290, 0x143872918, 0x1426430c0, 0x141834d18> as *const ()), // sub_1401cd370
    Replacement::draft(0x1401cd3b0, "unknown_1401cd3b0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872930, 0x142643150, 0x141834d18> as *const ()), // sub_1401cd3b0
    Replacement::draft(0x1401cd3f0, "unknown_1401cd3f0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872a28, 0x1426431e0, 0x141834d18> as *const ()), // sub_1401cd3f0
    Replacement::draft(0x1401cd430, "unknown_1401cd430", tpl_284::<0x143a001ac, 0x143a00290, 0x1438729e8, 0x142643270, 0x141834d18> as *const ()), // sub_1401cd430
    Replacement::draft(0x1401cd470, "unknown_1401cd470", tpl_284::<0x143a001ac, 0x143a00290, 0x143872998, 0x142643300, 0x141834d18> as *const ()), // sub_1401cd470
    Replacement::draft(0x1401cd4b0, "unknown_1401cd4b0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872a08, 0x142643390, 0x141834d18> as *const ()), // sub_1401cd4b0
    Replacement::draft(0x1401cd4f0, "unknown_1401cd4f0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872960, 0x142643420, 0x141834d18> as *const ()), // sub_1401cd4f0
    Replacement::draft_throws(0x1401cd530, "unknown_1401cd530", fn_1401cd530 as *const (), Throws::Discard), // sub_1401cd530
    Replacement::draft_throws(0x1401cd970, "std_1401cd970", fn_1401cd970 as *const (), Throws::Discard), // sub_1401cd970
    Replacement::draft_throws(0x1401d4700, "std_1401d4700", fn_1401d4700 as *const (), Throws::Discard), // sub_1401d4700
    Replacement::draft(0x1401db340, "unknown_1401db340", tpl_284::<0x143a001ac, 0x143a00290, 0x143872a88, 0x142643620, 0x141834d18> as *const ()), // sub_1401db340
    Replacement::draft(0x1401db380, "unknown_1401db380", tpl_284::<0x143a001ac, 0x143a00290, 0x143872af0, 0x142643630, 0x141834d18> as *const ()), // sub_1401db380
    Replacement::draft(0x1401db3c0, "unknown_1401db3c0", tpl_284::<0x143a001ac, 0x143a00290, 0x143872b08, 0x142643640, 0x141834d18> as *const ()), // sub_1401db3c0
    Replacement::draft(0x1401db400, "unknown_1401db400", tpl_284::<0x143a001ac, 0x143a00290, 0x143872aa0, 0x142643650, 0x141834d18> as *const ()), // sub_1401db400
    Replacement::draft(0x1401db440, "unknown_1401db440", tpl_284::<0x143a001ac, 0x143a00290, 0x143872a68, 0x142643660, 0x141834d18> as *const ()), // sub_1401db440
    Replacement::draft(0x1401db480, "unknown_1401db480", tpl_284::<0x143a001ac, 0x143a00290, 0x143872a48, 0x1426436f0, 0x141834d18> as *const ()), // sub_1401db480
    Replacement::draft(0x1401db4c0, "unknown_1401db4c0", tpl_327::<0x142643780, 0x141834d18> as *const ()), // sub_1401db4c0
    Replacement::draft(0x1401db560, "unknown_1401db560", tpl_219::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143c47f68, 0x1426437f0, 0x141834d18> as *const ()), // sub_1401db560
    Replacement::draft(0x1401db920, "unknown_1401db920", tpl_223::<0x143c8ce60, 0x143872b68, 0x143872b58> as *const ()), // sub_1401db920
    Replacement::draft(0x1401db940, "unknown_1401db940", tpl_223::<0x143c8ce60, 0x143872b38, 0x143872b28> as *const ()), // sub_1401db940
    Replacement::draft(0x1401db960, "unknown_1401db960", tpl_223::<0x143c8c648, 0x143872b50, 0x143872b40> as *const ()), // sub_1401db960
    Replacement::draft(0x1401db980, "unknown_1401db980", tpl_223::<0x143c8ce60, 0x143872be0, 0x143872bd0> as *const ()), // sub_1401db980
    Replacement::draft(0x1401db9a0, "unknown_1401db9a0", tpl_223::<0x143c8ce60, 0x143872b80, 0x143872b70> as *const ()), // sub_1401db9a0
    Replacement::draft(0x1401db9c0, "unknown_1401db9c0", tpl_223::<0x143c8ce60, 0x143872b98, 0x143872b88> as *const ()), // sub_1401db9c0
    Replacement::draft(0x1401db9e0, "unknown_1401db9e0", tpl_223::<0x143c8ce60, 0x143872bc8, 0x143872bb8> as *const ()), // sub_1401db9e0
    Replacement::draft(0x1401dba00, "unknown_1401dba00", tpl_223::<0x143c8ce60, 0x143872bb0, 0x143872ba0> as *const ()), // sub_1401dba00
    Replacement::draft_throws(0x1401dba20, "opentelemetry_1401dba20", tpl_334::<0x142669648, 0x14392bd40, 0x1402050d0, 0x142643a00, 0x141834d18> as *const (), Throws::Discard), // sub_1401dba20
    Replacement::draft(0x1401dba50, "opentelemetry_1401dba50", tpl_327::<0x14264399c, 0x141834d18> as *const ()), // sub_1401dba50
    Replacement::draft_throws(0x1401dba5c, "opentelemetry_1401dba5c", tpl_334::<0x142669648, 0x14392bd60, 0x1402050d0, 0x142643ac8, 0x141834d18> as *const (), Throws::Discard), // sub_1401dba5c
    Replacement::draft(0x1401dba8c, "opentelemetry_1401dba8c", tpl_327::<0x142643a64, 0x141834d18> as *const ()), // sub_1401dba8c
    Replacement::draft_throws(0x1401dba98, "opentelemetry_1401dba98", tpl_334::<0x142669648, 0x14392bd80, 0x1402050d0, 0x142643b90, 0x141834d18> as *const (), Throws::Discard), // sub_1401dba98
    Replacement::draft(0x1401dbac8, "opentelemetry_1401dbac8", tpl_327::<0x142643b2c, 0x141834d18> as *const ()), // sub_1401dbac8
    Replacement::draft_throws(0x1401dbad4, "opentelemetry_1401dbad4", fn_1401dbad4 as *const (), Throws::Discard), // sub_1401dbad4
    Replacement::draft(0x1401dbb04, "opentelemetry_1401dbb04", tpl_327::<0x142643d50, 0x141834d18> as *const ()), // sub_1401dbb04
    Replacement::draft(0x1401dbb10, "opentelemetry_1401dbb10", tpl_336::<0x14392c000, 0x1426470f0, 0x14266c648, 0x14392c010, 0x14392c018, 0x14392c01c, 0x14392c020, 0x142646f60, 0x14392c028, 0x142646fc0, 0x14392c030, 0x14392c038, 0x1426470a8, 0x142643e18, 0x141834d18> as *const ()), // sub_1401dbb10
    Replacement::draft(0x1401dbb90, "opentelemetry_1401dbb90", tpl_336::<0x14392bfc0, 0x1426470f0, 0x14266c6e8, 0x14392bfd0, 0x14392bfd8, 0x14392bfdc, 0x14392bfe0, 0x142646f60, 0x14392bfe8, 0x142646fc0, 0x14392bff0, 0x14392bff8, 0x1426470a8, 0x142643e1c, 0x141834d18> as *const ()), // sub_1401dbb90
    Replacement::draft(0x1401dbc10, "opentelemetry_1401dbc10", fn_1401dbc10 as *const ()), // sub_1401dbc10
    Replacement::draft(0x1401dbc94, "opentelemetry_1401dbc94", tpl_338::<0x14392c040, 0x1414c3094, 0x142643e24, 0x141834d18> as *const ()), // sub_1401dbc94
    Replacement::draft(0x1401dbcb8, "opentelemetry_1401dbcb8", tpl_338::<0x14392c0d0, 0x1414c2eb8, 0x142643e28, 0x141834d18> as *const ()), // sub_1401dbcb8
    Replacement::draft(0x1401dbcdc, "opentelemetry_1401dbcdc", fn_1401dbcdc as *const ()), // sub_1401dbcdc
    Replacement::draft(0x1401dbd3c, "opentelemetry_1401dbd3c", tpl_340::<0x1438130b0, 0x14392bf98, 0x1426470e8> as *const ()), // sub_1401dbd3c
    Replacement::draft(0x1401dbd54, "opentelemetry_1401dbd54", tpl_327::<0x142643e30, 0x141834d18> as *const ()), // sub_1401dbd54
    Replacement::draft(0x1401dbd60, "opentelemetry_1401dbd60", tpl_341::<0x14392c1b8, 0x1426470f0, 0x14266cf08, 0x142646f60, 0x14392c1c8, 0x142646fc0, 0x14392c1d0, 0x14392c1d8, 0x1426470a8, 0x142643e34, 0x141834d18> as *const ()), // sub_1401dbd60
    Replacement::draft(0x1401dbdc0, "opentelemetry_1401dbdc0", fn_1401dbdc0 as *const ()), // sub_1401dbdc0
    Replacement::draft(0x1401dbdd0, "opentelemetry_1401dbdd0", fn_1401dbdd0 as *const ()), // sub_1401dbdd0
    Replacement::draft(0x1401dbdd8, "opentelemetry_1401dbdd8", tpl_327::<0x142643e38, 0x141834d18> as *const ()), // sub_1401dbdd8
    Replacement::draft(0x1401dbde4, "opentelemetry_1401dbde4", tpl_327::<0x142643e3c, 0x141834d18> as *const ()), // sub_1401dbde4
    Replacement::draft(0x1401dbdf0, "opentelemetry_1401dbdf0", tpl_341::<0x14392c228, 0x1426470f0, 0x14266d548, 0x142646f60, 0x14392c238, 0x142646fc0, 0x14392c240, 0x14392c248, 0x1426470a8, 0x142643e40, 0x141834d18> as *const ()), // sub_1401dbdf0
    Replacement::draft(0x1401dbe50, "opentelemetry_1401dbe50", tpl_340::<0x1438131f0, 0x14392c220, 0x1426470e8> as *const ()), // sub_1401dbe50
    Replacement::draft(0x1401dbe68, "opentelemetry_1401dbe68", tpl_327::<0x142643e44, 0x141834d18> as *const ()), // sub_1401dbe68
    Replacement::draft(0x1401dbe74, "opentelemetry_1401dbe74", tpl_327::<0x142643e48, 0x141834d18> as *const ()), // sub_1401dbe74
    Replacement::draft(0x1401dbe80, "opentelemetry_1401dbe80", fn_1401dbe80 as *const ()), // sub_1401dbe80
    Replacement::draft(0x1401dbf10, "opentelemetry_1401dbf10", tpl_341::<0x14392c2b0, 0x1426470f0, 0x14266dcd0, 0x142646f60, 0x14392c2c0, 0x142646fc0, 0x14392c2c8, 0x14392c2d0, 0x1426470a8, 0x142643e5c, 0x141834d18> as *const ()), // sub_1401dbf10
    Replacement::draft(0x1401dbf70, "opentelemetry_1401dbf70", tpl_340::<0x1438132a0, 0x14392c2a8, 0x1426470e8> as *const ()), // sub_1401dbf70
    Replacement::draft(0x1401dbf88, "opentelemetry_1401dbf88", tpl_327::<0x142643e4c, 0x141834d18> as *const ()), // sub_1401dbf88
    Replacement::draft(0x1401dbf94, "opentelemetry_1401dbf94", tpl_327::<0x142643e50, 0x141834d18> as *const ()), // sub_1401dbf94
    Replacement::draft(0x1401dbfa0, "opentelemetry_1401dbfa0", tpl_327::<0x142643e58, 0x141834d18> as *const ()), // sub_1401dbfa0
    Replacement::draft(0x1401dbfac, "opentelemetry_1401dbfac", tpl_340::<0x143813380, 0x14392c338, 0x1426470e8> as *const ()), // sub_1401dbfac
    Replacement::draft(0x1401dbfc4, "opentelemetry_1401dbfc4", tpl_327::<0x142643e60, 0x141834d18> as *const ()), // sub_1401dbfc4
    Replacement::draft(0x1401dbfd0, "opentelemetry_1401dbfd0", tpl_338::<0x14392c3e0, 0x1414cba2c, 0x142643e64, 0x141834d18> as *const ()), // sub_1401dbfd0
    Replacement::draft(0x1401dbff4, "opentelemetry_1401dbff4", tpl_336::<0x14392c450, 0x1426470f0, 0x14266eac8, 0x14392c460, 0x14392c468, 0x14392c46c, 0x14392c470, 0x142646f60, 0x14392c478, 0x142646fc0, 0x14392c480, 0x14392c488, 0x1426470a8, 0x142643e6c, 0x141834d18> as *const ()), // sub_1401dbff4
    Replacement::draft(0x1401dc074, "opentelemetry_1401dc074", tpl_336::<0x14392c3a0, 0x1426470f0, 0x14266eb68, 0x14392c3b0, 0x14392c3b8, 0x14392c3bc, 0x14392c3c0, 0x142646f60, 0x14392c3c8, 0x142646fc0, 0x14392c3d0, 0x14392c3d8, 0x1426470a8, 0x142643e70, 0x141834d18> as *const ()), // sub_1401dc074
    Replacement::draft(0x1401dc0f4, "opentelemetry_1401dc0f4", tpl_340::<0x143813410, 0x14392c398, 0x1426470e8> as *const ()), // sub_1401dc0f4
    Replacement::draft(0x1401dc10c, "opentelemetry_1401dc10c", tpl_327::<0x142643e68, 0x141834d18> as *const ()), // sub_1401dc10c
    Replacement::draft(0x1401dc118, "opentelemetry_1401dc118", tpl_338::<0x14392c5b0, 0x1414d4628, 0x142643e74, 0x141834d18> as *const ()), // sub_1401dc118
    Replacement::draft(0x1401dc13c, "opentelemetry_1401dc13c", fn_1401dc13c as *const ()), // sub_1401dc13c
    Replacement::draft(0x1401dc1c4, "opentelemetry_1401dc1c4", tpl_336::<0x14392c680, 0x1426470f0, 0x1426708a8, 0x14392c690, 0x14392c698, 0x14392c69c, 0x14392c6a0, 0x142646f60, 0x14392c6a8, 0x142646fc0, 0x14392c6b0, 0x14392c6b8, 0x1426470a8, 0x142643e9c, 0x141834d18> as *const ()), // sub_1401dc1c4
    Replacement::draft(0x1401dc244, "opentelemetry_1401dc244", tpl_336::<0x14392c640, 0x1426470f0, 0x142670948, 0x14392c650, 0x14392c658, 0x14392c65c, 0x14392c660, 0x142646f60, 0x14392c668, 0x142646fc0, 0x14392c670, 0x14392c678, 0x1426470a8, 0x142643ea0, 0x141834d18> as *const ()), // sub_1401dc244
    Replacement::draft(0x1401dc2c4, "unknown_1401dc2c4", tpl_340::<0x1438134a0, 0x14392c5a8, 0x1426470e8> as *const ()), // sub_1401dc2c4
    Replacement::draft(0x1401dc2dc, "unknown_1401dc2dc", tpl_327::<0x142643e78, 0x141834d18> as *const ()), // sub_1401dc2dc
    Replacement::draft(0x1401dc2e8, "unknown_1401dc2e8", tpl_327::<0x142643e7c, 0x141834d18> as *const ()), // sub_1401dc2e8
    Replacement::draft(0x1401dc2f4, "unknown_1401dc2f4", tpl_327::<0x142643e80, 0x141834d18> as *const ()), // sub_1401dc2f4
    Replacement::draft(0x1401dc300, "unknown_1401dc300", tpl_327::<0x142643e84, 0x141834d18> as *const ()), // sub_1401dc300
    Replacement::draft(0x1401dc30c, "unknown_1401dc30c", tpl_327::<0x142643e88, 0x141834d18> as *const ()), // sub_1401dc30c
    Replacement::draft(0x1401dc318, "unknown_1401dc318", tpl_327::<0x142643e8c, 0x141834d18> as *const ()), // sub_1401dc318
    Replacement::draft(0x1401dc324, "unknown_1401dc324", tpl_327::<0x142643e94, 0x141834d18> as *const ()), // sub_1401dc324
    Replacement::draft(0x1401dc330, "unknown_1401dc330", tpl_327::<0x142643e98, 0x141834d18> as *const ()), // sub_1401dc330
    Replacement::draft(0x1401dc33c, "unknown_1401dc33c", tpl_327::<0x142643ea4, 0x141834d18> as *const ()), // sub_1401dc33c
    Replacement::draft(0x1401dc348, "unknown_1401dc348", tpl_327::<0x142643ea8, 0x141834d18> as *const ()), // sub_1401dc348
    Replacement::draft(0x1401dc354, "unknown_1401dc354", tpl_327::<0x142643eac, 0x141834d18> as *const ()), // sub_1401dc354
    Replacement::draft(0x1401dc360, "unknown_1401dc360", tpl_327::<0x142643eb0, 0x141834d18> as *const ()), // sub_1401dc360
    Replacement::draft(0x1401dc370, "unknown_1401dc370", tpl_345::<0x143931550, 0x1438138e0> as *const ()), // sub_1401dc370
    Replacement::draft(0x1401dc390, "unknown_1401dc390", tpl_345::<0x143931550, 0x143814140> as *const ()), // sub_1401dc390
    Replacement::draft(0x1401dc3b0, "unknown_1401dc3b0", tpl_345::<0x143931550, 0x1438141b0> as *const ()), // sub_1401dc3b0
    Replacement::draft(0x1401dc3d0, "unknown_1401dc3d0", tpl_345::<0x143931550, 0x143814220> as *const ()), // sub_1401dc3d0
    Replacement::draft(0x1401dc3f0, "unknown_1401dc3f0", tpl_345::<0x143931550, 0x143814290> as *const ()), // sub_1401dc3f0
    Replacement::draft(0x1401dc410, "unknown_1401dc410", tpl_345::<0x143931550, 0x143814300> as *const ()), // sub_1401dc410
    Replacement::draft(0x1401dc430, "unknown_1401dc430", tpl_345::<0x143931550, 0x143814370> as *const ()), // sub_1401dc430
    Replacement::draft(0x1401dc450, "unknown_1401dc450", tpl_345::<0x143931550, 0x1438143e0> as *const ()), // sub_1401dc450
    Replacement::draft(0x1401dc470, "unknown_1401dc470", tpl_345::<0x143931550, 0x143814450> as *const ()), // sub_1401dc470
    Replacement::draft(0x1401dc490, "unknown_1401dc490", tpl_345::<0x143931550, 0x1438144c0> as *const ()), // sub_1401dc490
    Replacement::draft(0x1401dc4b0, "unknown_1401dc4b0", tpl_327::<0x142644010, 0x141834d18> as *const ()), // sub_1401dc4b0
];
