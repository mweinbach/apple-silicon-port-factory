#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1418fabd0, "unknown_1418fabd0", fn_1418fabd0 as *const ()), // sub_1418fabd0
    Replacement::draft(0x1418face0, "unknown_1418face0", fn_1418face0 as *const ()), // sub_1418face0
    Replacement::draft(0x1418fad40, "unknown_1418fad40", fn_1418fad40 as *const ()), // sub_1418fad40
    Replacement::draft(0x1418fad80, "unknown_1418fad80", fn_1418fad80 as *const ()), // sub_1418fad80
    Replacement::draft(0x1418faeb0, "unknown_1418faeb0", fn_1418faeb0 as *const ()), // sub_1418faeb0
    Replacement::draft_throws(0x1418fafe0, "unknown_1418fafe0", fn_1418fafe0 as *const (), Throws::Caught), // sub_1418fafe0
    Replacement::draft(0x1418fb5d0, "unknown_1418fb5d0", fn_1418fb5d0 as *const ()), // sub_1418fb5d0
    Replacement::draft(0x1418fb6c0, "unknown_1418fb6c0", fn_1418fb6c0 as *const ()), // sub_1418fb6c0
    Replacement::draft(0x1418fb720, "unknown_1418fb720", tpl_35097 as *const ()), // sub_1418fb720
    Replacement::draft_throws(0x1418fbc00, "unknown_1418fbc00", fn_1418fbc00 as *const (), Throws::Caught), // sub_1418fbc00
    Replacement::draft_throws(0x1418fbec0, "unknown_1418fbec0", fn_1418fbec0 as *const (), Throws::Discard), // sub_1418fbec0
    Replacement::draft_throws(0x1418fc090, "unknown_1418fc090", fn_1418fc090 as *const (), Throws::Caught), // sub_1418fc090
    Replacement::draft_throws(0x1418fc2b0, "unknown_1418fc2b0", fn_1418fc2b0 as *const (), Throws::Discard), // sub_1418fc2b0
    Replacement::draft(0x1418fc5f0, "unknown_1418fc5f0", tpl_3037::<0x8, 0x140358b80, 0x140358d80> as *const ()), // sub_1418fc5f0
    Replacement::draft_throws(0x1418fc670, "unknown_1418fc670", fn_1418fc670 as *const (), Throws::Discard), // sub_1418fc670
    Replacement::draft(0x1418fca00, "unknown_1418fca00", tpl_34613 as *const ()), // sub_1418fca00
    Replacement::draft(0x1418fcc30, "unknown_1418fcc30", tpl_34393 as *const ()), // sub_1418fcc30
    Replacement::draft(0x1418fccc0, "unknown_1418fccc0", fn_1418fccc0 as *const ()), // sub_1418fccc0
    Replacement::draft(0x1418fce10, "unknown_1418fce10", tpl_35320::<0x14037e270> as *const ()), // sub_1418fce10
    Replacement::draft(0x1418fcec0, "unknown_1418fcec0", fn_1418fcec0 as *const ()), // sub_1418fcec0
    Replacement::draft(0x1418fcf70, "unknown_1418fcf70", tpl_35321::<0x1430bf490, 0x8, 0xc, 0x10> as *const ()), // sub_1418fcf70
    Replacement::draft(0x1418fcfb0, "unknown_1418fcfb0", tpl_35322 as *const ()), // sub_1418fcfb0
    Replacement::draft(0x1418fcff0, "unknown_1418fcff0", tpl_34994::<0x143c95a20, 0x14387a700, 0x14387a6f8, 0x141834a78, 0x1425df790, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fcff0
    Replacement::draft(0x1418fd060, "unknown_1418fd060", tpl_35252 as *const ()), // sub_1418fd060
    Replacement::draft(0x1418fd080, "unknown_1418fd080", tpl_34994::<0x143c94b80, 0x14387a750, 0x14387a748, 0x141834a78, 0x1425df7a0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd080
    Replacement::draft(0x1418fd0f0, "unknown_1418fd0f0", tpl_35252 as *const ()), // sub_1418fd0f0
    Replacement::draft(0x1418fd110, "unknown_1418fd110", tpl_34994::<0x143c95ecc, 0x14387a760, 0x14387a758, 0x141834a78, 0x1425df7b0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd110
    Replacement::draft(0x1418fd180, "unknown_1418fd180", tpl_34995 as *const ()), // sub_1418fd180
    Replacement::draft(0x1418fd1a0, "unknown_1418fd1a0", tpl_34994::<0x143c94430, 0x14387a600, 0x14387a5f8, 0x141834a78, 0x1425df7c0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd1a0
    Replacement::draft(0x1418fd210, "unknown_1418fd210", tpl_35323 as *const ()), // sub_1418fd210
    Replacement::draft(0x1418fd230, "unknown_1418fd230", tpl_34994::<0x143c94cb8, 0x14387a5a0, 0x14387a598, 0x141834a78, 0x1425df7d0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd230
    Replacement::draft(0x1418fd2a0, "unknown_1418fd2a0", tpl_35323 as *const ()), // sub_1418fd2a0
    Replacement::draft(0x1418fd2c0, "unknown_1418fd2c0", tpl_35323 as *const ()), // sub_1418fd2c0
    Replacement::draft(0x1418fd2e0, "unknown_1418fd2e0", tpl_34994::<0x143c95ec8, 0x14387a690, 0x14387a688, 0x141834a78, 0x1425df6c0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd2e0
    Replacement::draft(0x1418fd350, "unknown_1418fd350", fn_1418fd350 as *const ()), // sub_1418fd350
    Replacement::draft(0x1418fd490, "unknown_1418fd490", tpl_35325::<0x14037dce0> as *const ()), // sub_1418fd490
    Replacement::draft(0x1418fd4b0, "unknown_1418fd4b0", fn_1418fd4b0 as *const ()), // sub_1418fd4b0
    Replacement::draft(0x1418fd4f0, "unknown_1418fd4f0", tpl_34994::<0x143c94c6c, 0x14387a6d0, 0x14387a6c8, 0x141834a78, 0x1425df6d0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd4f0
    Replacement::draft(0x1418fd560, "unknown_1418fd560", tpl_34394::<0x10> as *const ()), // sub_1418fd560
    Replacement::draft(0x1418fd590, "unknown_1418fd590", tpl_34994::<0x143c96380, 0x14387a6b0, 0x14387a6a8, 0x141834a78, 0x1425df6e0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd590
    Replacement::draft(0x1418fd600, "unknown_1418fd600", tpl_34667::<0x20> as *const ()), // sub_1418fd600
    Replacement::draft(0x1418fd630, "unknown_1418fd630", tpl_34994::<0x143c952d0, 0x14387a5c0, 0x14387a5b8, 0x141834a78, 0x1425df6f0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd630
    Replacement::draft(0x1418fd6a0, "unknown_1418fd6a0", tpl_34613 as *const ()), // sub_1418fd6a0
    Replacement::draft(0x1418fd6c0, "unknown_1418fd6c0", tpl_34994::<0x143c956e4, 0x14387a640, 0x14387a638, 0x141834a78, 0x1425df700, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd6c0
    Replacement::draft(0x1418fd730, "unknown_1418fd730", tpl_34995 as *const ()), // sub_1418fd730
    Replacement::draft(0x1418fd750, "unknown_1418fd750", tpl_34994::<0x143c94a18, 0x14387a580, 0x14387a578, 0x141834a78, 0x1425df710, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd750
    Replacement::draft(0x1418fd7c0, "unknown_1418fd7c0", tpl_369 as *const ()), // sub_1418fd7c0
    Replacement::draft(0x1418fd7d0, "unknown_1418fd7d0", tpl_34994::<0x143c954d8, 0x14387a710, 0x14387a708, 0x141834a78, 0x1425df720, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd7d0
    Replacement::draft(0x1418fd840, "unknown_1418fd840", tpl_369 as *const ()), // sub_1418fd840
    Replacement::draft(0x1418fd850, "unknown_1418fd850", tpl_34994::<0x143c944a0, 0x14387a570, 0x14387a568, 0x141834a78, 0x1425df730, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fd850
    Replacement::draft(0x1418fd8c0, "unknown_1418fd8c0", fn_1418fd8c0 as *const ()), // sub_1418fd8c0
    Replacement::draft(0x1418fd9b0, "unknown_1418fd9b0", fn_1418fd9b0 as *const ()), // sub_1418fd9b0
    Replacement::draft(0x1418fda00, "unknown_1418fda00", tpl_35328::<0x1401f2bc0> as *const ()), // sub_1418fda00
    Replacement::draft(0x1418fda40, "unknown_1418fda40", tpl_34994::<0x143c953ac, 0x14387a6f0, 0x14387a6e8, 0x141834a78, 0x1425df740, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fda40
    Replacement::draft(0x1418fdab0, "unknown_1418fdab0", fn_1418fdab0 as *const ()), // sub_1418fdab0
    Replacement::draft(0x1418fdbc0, "unknown_1418fdbc0", fn_1418fdbc0 as *const ()), // sub_1418fdbc0
    Replacement::draft(0x1418fdc30, "unknown_1418fdc30", fn_1418fdc30 as *const ()), // sub_1418fdc30
    Replacement::draft(0x1418fdc70, "unknown_1418fdc70", tpl_34994::<0x143c94e1c, 0x14387a630, 0x14387a628, 0x141834a78, 0x1425df750, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fdc70
    Replacement::draft(0x1418fdce0, "unknown_1418fdce0", tpl_35331 as *const ()), // sub_1418fdce0
    Replacement::draft(0x1418fdd00, "unknown_1418fdd00", tpl_34994::<0x143c94570, 0x14387a5f0, 0x14387a5e8, 0x141834a78, 0x1425df760, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fdd00
    Replacement::draft(0x1418fdd70, "unknown_1418fdd70", tpl_35331 as *const ()), // sub_1418fdd70
    Replacement::draft(0x1418fdd90, "unknown_1418fdd90", tpl_34994::<0x143c94428, 0x14387a620, 0x14387a618, 0x141834a78, 0x1425df770, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fdd90
    Replacement::draft(0x1418fde00, "unknown_1418fde00", tpl_34995 as *const ()), // sub_1418fde00
    Replacement::draft(0x1418fde20, "unknown_1418fde20", tpl_34994::<0x143c958c8, 0x14387a610, 0x14387a608, 0x141834a78, 0x1425df780, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fde20
    Replacement::draft(0x1418fde90, "unknown_1418fde90", tpl_94790 as *const ()), // sub_1418fde90
    Replacement::draft(0x1418fdf20, "unknown_1418fdf20", tpl_35012 as *const ()), // sub_1418fdf20
    Replacement::draft(0x1418fdf40, "unknown_1418fdf40", tpl_35013::<0x142442de0> as *const ()), // sub_1418fdf40
    Replacement::draft(0x1418fdfb0, "unknown_1418fdfb0", tpl_34994::<0x143c95288, 0x14387a590, 0x14387a588, 0x141834a78, 0x1425df650, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fdfb0
    Replacement::draft(0x1418fe020, "unknown_1418fe020", tpl_35099::<0x20, 0x40> as *const ()), // sub_1418fe020
    Replacement::draft(0x1418fe060, "unknown_1418fe060", tpl_34994::<0x143c94e60, 0x14387a5b0, 0x14387a5a8, 0x141834a78, 0x1425df660, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fe060
    Replacement::draft(0x1418fe0d0, "unknown_1418fe0d0", tpl_35332 as *const ()), // sub_1418fe0d0
    Replacement::draft(0x1418fe0e0, "unknown_1418fe0e0", tpl_34994::<0x143c964e4, 0x14387a6e0, 0x14387a6d8, 0x141834a78, 0x1425df670, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fe0e0
    Replacement::draft(0x1418fe150, "unknown_1418fe150", tpl_34667::<0x20> as *const ()), // sub_1418fe150
    Replacement::draft(0x1418fe180, "unknown_1418fe180", tpl_34994::<0x143c95908, 0x14387a730, 0x14387a728, 0x141834a78, 0x1425df680, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fe180
    Replacement::draft(0x1418fe1f0, "unknown_1418fe1f0", fn_1418fe1f0 as *const ()), // sub_1418fe1f0
    Replacement::draft(0x1418fe370, "unknown_1418fe370", fn_1418fe370 as *const ()), // sub_1418fe370
    Replacement::draft(0x1418fe390, "unknown_1418fe390", fn_1418fe390 as *const ()), // sub_1418fe390
    Replacement::draft(0x1418fe3f0, "unknown_1418fe3f0", tpl_34994::<0x143c94ec4, 0x14387a670, 0x14387a668, 0x141834a78, 0x1425df690, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fe3f0
    Replacement::draft(0x1418fe460, "unknown_1418fe460", tpl_35332 as *const ()), // sub_1418fe460
    Replacement::draft(0x1418fe470, "unknown_1418fe470", tpl_34994::<0x143c955d0, 0x14387a660, 0x14387a658, 0x141834a78, 0x1425df6a0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fe470
    Replacement::draft(0x1418fe4e0, "unknown_1418fe4e0", tpl_34671::<0x20> as *const ()), // sub_1418fe4e0
    Replacement::draft(0x1418fe510, "unknown_1418fe510", tpl_34994::<0x143c95d8c, 0x14387a770, 0x14387a768, 0x141834a78, 0x1425df6b0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418fe510
    Replacement::draft(0x1418fe580, "unknown_1418fe580", tpl_35332 as *const ()), // sub_1418fe580
    Replacement::draft(0x1418fe590, "unknown_1418fe590", tpl_34318 as *const ()), // sub_1418fe590
    Replacement::draft(0x1418fe5b0, "unknown_1418fe5b0", tpl_34318 as *const ()), // sub_1418fe5b0
    Replacement::draft(0x1418fe5d0, "unknown_1418fe5d0", tpl_34318 as *const ()), // sub_1418fe5d0
    Replacement::draft(0x1418fe5f0, "unknown_1418fe5f0", tpl_34318 as *const ()), // sub_1418fe5f0
    Replacement::draft(0x1418fe610, "unknown_1418fe610", tpl_34319::<0x143c95490, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe610
    Replacement::draft(0x1418fe670, "unknown_1418fe670", tpl_34319::<0x143c95468, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe670
    Replacement::draft(0x1418fe6d0, "unknown_1418fe6d0", tpl_34319::<0x143c91078, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe6d0
    Replacement::draft(0x1418fe730, "unknown_1418fe730", tpl_34319::<0x143c8dbc8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe730
    Replacement::draft(0x1418fe790, "unknown_1418fe790", tpl_34318 as *const ()), // sub_1418fe790
    Replacement::draft(0x1418fe7b0, "unknown_1418fe7b0", tpl_34319::<0x143c95844, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe7b0
    Replacement::draft(0x1418fe810, "unknown_1418fe810", tpl_34318 as *const ()), // sub_1418fe810
    Replacement::draft(0x1418fe830, "unknown_1418fe830", tpl_34319::<0x143c956a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe830
    Replacement::draft(0x1418fe890, "unknown_1418fe890", tpl_34318 as *const ()), // sub_1418fe890
    Replacement::draft(0x1418fe8b0, "unknown_1418fe8b0", tpl_34319::<0x143c95188, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe8b0
    Replacement::draft(0x1418fe910, "unknown_1418fe910", tpl_34318 as *const ()), // sub_1418fe910
    Replacement::draft(0x1418fe930, "unknown_1418fe930", tpl_34318 as *const ()), // sub_1418fe930
    Replacement::draft(0x1418fe950, "unknown_1418fe950", tpl_34319::<0x143c8fe18, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe950
    Replacement::draft(0x1418fe9b0, "unknown_1418fe9b0", tpl_34319::<0x143c963c0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fe9b0
    Replacement::draft(0x1418fea10, "unknown_1418fea10", tpl_34318 as *const ()), // sub_1418fea10
    Replacement::draft(0x1418fea30, "unknown_1418fea30", tpl_34319::<0x143c954e4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fea30
    Replacement::draft(0x1418fea90, "unknown_1418fea90", tpl_361 as *const ()), // sub_1418fea90
    Replacement::draft(0x1418feaa0, "unknown_1418feaa0", tpl_34318 as *const ()), // sub_1418feaa0
    Replacement::draft(0x1418feac0, "unknown_1418feac0", tpl_35063::<0x143c94b50, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_1418feac0
    Replacement::draft(0x1418feb10, "unknown_1418feb10", tpl_34319::<0x143c94b7c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418feb10
    Replacement::draft(0x1418feb70, "unknown_1418feb70", tpl_34318 as *const ()), // sub_1418feb70
    Replacement::draft(0x1418feb90, "unknown_1418feb90", tpl_34318 as *const ()), // sub_1418feb90
    Replacement::draft(0x1418febb0, "unknown_1418febb0", tpl_34319::<0x143c94b3c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418febb0
    Replacement::draft(0x1418fec10, "unknown_1418fec10", tpl_34319::<0x143c8e710, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fec10
    Replacement::draft(0x1418fec70, "unknown_1418fec70", tpl_34318 as *const ()), // sub_1418fec70
    Replacement::draft(0x1418fec90, "unknown_1418fec90", tpl_34319::<0x143c94b2c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fec90
    Replacement::draft(0x1418fecf0, "unknown_1418fecf0", tpl_34318 as *const ()), // sub_1418fecf0
    Replacement::draft(0x1418fed10, "unknown_1418fed10", tpl_34319::<0x143c953f8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fed10
    Replacement::draft(0x1418fed70, "unknown_1418fed70", tpl_34318 as *const ()), // sub_1418fed70
    Replacement::draft(0x1418fed90, "unknown_1418fed90", tpl_34319::<0x143c94b6c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fed90
    Replacement::draft(0x1418fedf0, "unknown_1418fedf0", tpl_34318 as *const ()), // sub_1418fedf0
    Replacement::draft(0x1418fee10, "unknown_1418fee10", tpl_34318 as *const ()), // sub_1418fee10
    Replacement::draft(0x1418fee30, "unknown_1418fee30", tpl_34319::<0x143c8ee34, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fee30
    Replacement::draft(0x1418fee90, "unknown_1418fee90", tpl_34319::<0x143c8f690, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fee90
    Replacement::draft(0x1418feef0, "unknown_1418feef0", tpl_34318 as *const ()), // sub_1418feef0
    Replacement::draft(0x1418fef10, "unknown_1418fef10", tpl_34319::<0x143c95bf0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fef10
    Replacement::draft(0x1418fef70, "unknown_1418fef70", tpl_34318 as *const ()), // sub_1418fef70
    Replacement::draft(0x1418fef90, "unknown_1418fef90", tpl_34318 as *const ()), // sub_1418fef90
    Replacement::draft(0x1418fefb0, "unknown_1418fefb0", tpl_34319::<0x143c8f6e4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fefb0
    Replacement::draft(0x1418ff010, "unknown_1418ff010", tpl_34319::<0x143c94ef0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff010
    Replacement::draft(0x1418ff070, "unknown_1418ff070", tpl_34318 as *const ()), // sub_1418ff070
    Replacement::draft(0x1418ff090, "unknown_1418ff090", tpl_34318 as *const ()), // sub_1418ff090
    Replacement::draft(0x1418ff0b0, "unknown_1418ff0b0", tpl_34318 as *const ()), // sub_1418ff0b0
    Replacement::draft(0x1418ff0d0, "unknown_1418ff0d0", tpl_34318 as *const ()), // sub_1418ff0d0
    Replacement::draft(0x1418ff0f0, "unknown_1418ff0f0", tpl_34319::<0x143c8d708, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff0f0
    Replacement::draft(0x1418ff150, "unknown_1418ff150", tpl_34319::<0x143c964c0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff150
    Replacement::draft(0x1418ff1b0, "unknown_1418ff1b0", tpl_34319::<0x143c8d68c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff1b0
    Replacement::draft(0x1418ff210, "unknown_1418ff210", tpl_34319::<0x143c8f4cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff210
    Replacement::draft(0x1418ff270, "unknown_1418ff270", tpl_34318 as *const ()), // sub_1418ff270
    Replacement::draft(0x1418ff290, "unknown_1418ff290", tpl_34318 as *const ()), // sub_1418ff290
    Replacement::draft(0x1418ff2b0, "unknown_1418ff2b0", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff2b0
    Replacement::draft(0x1418ff310, "unknown_1418ff310", tpl_34319::<0x143c93358, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff310
    Replacement::draft(0x1418ff370, "std_1418ff370", tpl_35335::<0x1430bf708, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::getProjectileSweepFilter::<lambda_1>,bool,const ?bref6::PhysicsSweepHit&>:
    Replacement::draft(0x1418ff3a0, "std_1418ff3a0", tpl_35335::<0x1430bf708, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::getProjectileSweepFilter::<lambda_1>,bool,const ?bref6::PhysicsSweepHit&>:
    Replacement::draft(0x1418ff3e0, "std_1418ff3e0", tpl_1::<0x143886c20> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::getProjectileSweepFilter::<lambda_1>,bool,const ?bref6::PhysicsSweepHit&>:
    Replacement::draft(0x1418ff3f0, "std_1418ff3f0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::getProjectileSweepFilter::<lambda_1>,bool,const ?bref6::PhysicsSweepHit&>:
    Replacement::draft(0x1418ff400, "std_1418ff400", tpl_35336::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::getProjectileSweepFilter::<lambda_1>,bool,const ?bref6::PhysicsSweepHit&>:
    Replacement::draft(0x1418ff420, "std_1418ff420", tpl_550::<0x1430bf5f8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::on_floor_update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff440, "std_1418ff440", tpl_550::<0x1430bf5f8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::on_floor_update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff460, "std_1418ff460", fn_1418ff460 as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::on_floor_update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff4e0, "std_1418ff4e0", tpl_1::<0x143886eb0> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::on_floor_update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff4f0, "std_1418ff4f0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::on_floor_update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff500, "std_1418ff500", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::on_floor_update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff520, "std_1418ff520", tpl_550::<0x1430bf6c0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::draw_trajectory::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff540, "std_1418ff540", tpl_550::<0x1430bf6c0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::draw_trajectory::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff560, "std_1418ff560", fn_1418ff560 as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::draw_trajectory::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff5d0, "std_1418ff5d0", tpl_1::<0x143886760> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::draw_trajectory::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff5e0, "std_1418ff5e0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::draw_trajectory::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff5f0, "std_1418ff5f0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::projectile::draw_trajectory::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1418ff610, "unknown_1418ff610", tpl_34318 as *const ()), // sub_1418ff610
    Replacement::draft(0x1418ff630, "unknown_1418ff630", tpl_34318 as *const ()), // sub_1418ff630
    Replacement::draft(0x1418ff650, "unknown_1418ff650", tpl_34319::<0x143c8e70c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff650
    Replacement::draft(0x1418ff6b0, "unknown_1418ff6b0", tpl_34319::<0x143c9626c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff6b0
    Replacement::draft(0x1418ff710, "unknown_1418ff710", tpl_34318 as *const ()), // sub_1418ff710
    Replacement::draft(0x1418ff730, "unknown_1418ff730", tpl_34318 as *const ()), // sub_1418ff730
    Replacement::draft(0x1418ff750, "unknown_1418ff750", tpl_34318 as *const ()), // sub_1418ff750
    Replacement::draft(0x1418ff770, "unknown_1418ff770", tpl_34319::<0x143c95048, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff770
    Replacement::draft(0x1418ff7d0, "unknown_1418ff7d0", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff7d0
    Replacement::draft(0x1418ff830, "unknown_1418ff830", tpl_34319::<0x143c94b48, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff830
    Replacement::draft(0x1418ff890, "unknown_1418ff890", tpl_34318 as *const ()), // sub_1418ff890
    Replacement::draft(0x1418ff8b0, "unknown_1418ff8b0", tpl_34318 as *const ()), // sub_1418ff8b0
    Replacement::draft(0x1418ff8d0, "unknown_1418ff8d0", tpl_34318 as *const ()), // sub_1418ff8d0
    Replacement::draft(0x1418ff8f0, "unknown_1418ff8f0", tpl_34318 as *const ()), // sub_1418ff8f0
    Replacement::draft(0x1418ff910, "unknown_1418ff910", tpl_34318 as *const ()), // sub_1418ff910
    Replacement::draft(0x1418ff930, "unknown_1418ff930", tpl_34318 as *const ()), // sub_1418ff930
    Replacement::draft(0x1418ff950, "unknown_1418ff950", tpl_34319::<0x143c914b0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff950
    Replacement::draft(0x1418ff9b0, "unknown_1418ff9b0", tpl_34319::<0x143c8dbd4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ff9b0
    Replacement::draft(0x1418ffa10, "unknown_1418ffa10", tpl_34319::<0x143c8d074, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffa10
    Replacement::draft(0x1418ffa70, "unknown_1418ffa70", tpl_34319::<0x143c933f0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffa70
    Replacement::draft(0x1418ffad0, "unknown_1418ffad0", tpl_34319::<0x143c9584c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffad0
    Replacement::draft(0x1418ffb30, "unknown_1418ffb30", tpl_34319::<0x143c92bcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffb30
    Replacement::draft(0x1418ffb90, "unknown_1418ffb90", tpl_34318 as *const ()), // sub_1418ffb90
    Replacement::draft(0x1418ffbb0, "unknown_1418ffbb0", tpl_34318 as *const ()), // sub_1418ffbb0
    Replacement::draft(0x1418ffbd0, "unknown_1418ffbd0", tpl_34318 as *const ()), // sub_1418ffbd0
    Replacement::draft(0x1418ffbf0, "unknown_1418ffbf0", tpl_34318 as *const ()), // sub_1418ffbf0
    Replacement::draft(0x1418ffc10, "unknown_1418ffc10", tpl_34318 as *const ()), // sub_1418ffc10
    Replacement::draft(0x1418ffc30, "unknown_1418ffc30", tpl_34318 as *const ()), // sub_1418ffc30
    Replacement::draft(0x1418ffc50, "unknown_1418ffc50", tpl_34318 as *const ()), // sub_1418ffc50
    Replacement::draft(0x1418ffc70, "unknown_1418ffc70", tpl_34319::<0x143c95608, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffc70
    Replacement::draft(0x1418ffcd0, "unknown_1418ffcd0", tpl_34319::<0x143c8e654, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffcd0
    Replacement::draft(0x1418ffd30, "unknown_1418ffd30", tpl_34319::<0x143c95c8c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffd30
    Replacement::draft(0x1418ffd90, "unknown_1418ffd90", tpl_34319::<0x143c95748, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffd90
    Replacement::draft(0x1418ffdf0, "unknown_1418ffdf0", tpl_34319::<0x143c954e0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffdf0
    Replacement::draft(0x1418ffe50, "unknown_1418ffe50", tpl_34319::<0x143c914bc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffe50
    Replacement::draft(0x1418ffeb0, "unknown_1418ffeb0", tpl_34319::<0x143c964e0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418ffeb0
    Replacement::draft(0x1418fff10, "unknown_1418fff10", tpl_34318 as *const ()), // sub_1418fff10
    Replacement::draft(0x1418fff30, "unknown_1418fff30", tpl_34318 as *const ()), // sub_1418fff30
    Replacement::draft(0x1418fff50, "unknown_1418fff50", tpl_34318 as *const ()), // sub_1418fff50
    Replacement::draft(0x1418fff70, "unknown_1418fff70", tpl_34318 as *const ()), // sub_1418fff70
    Replacement::draft(0x1418fff90, "unknown_1418fff90", tpl_361 as *const ()), // sub_1418fff90
    Replacement::draft(0x1418fffa0, "unknown_1418fffa0", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418fffa0
    Replacement::draft(0x141900000, "unknown_141900000", tpl_34319::<0x143c8ec28, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141900000
    Replacement::draft(0x141900060, "unknown_141900060", tpl_34319::<0x143c8dbd0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141900060
    Replacement::draft(0x1419000c0, "unknown_1419000c0", tpl_34319::<0x143c8dbc4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419000c0
    Replacement::draft(0x141900120, "unknown_141900120", tpl_35063::<0x143c8d694, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141900120
    Replacement::draft(0x141900170, "unknown_141900170", tpl_34318 as *const ()), // sub_141900170
    Replacement::draft(0x141900190, "unknown_141900190", tpl_34319::<0x143c94e58, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141900190
    Replacement::draft(0x1419001f0, "unknown_1419001f0", tpl_34318 as *const ()), // sub_1419001f0
    Replacement::draft(0x141900210, "unknown_141900210", tpl_34318 as *const ()), // sub_141900210
    Replacement::draft(0x141900230, "unknown_141900230", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141900230
    Replacement::draft(0x141900290, "unknown_141900290", tpl_34319::<0x143c8d508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141900290
    Replacement::draft(0x1419002f0, "unknown_1419002f0", tpl_34318 as *const ()), // sub_1419002f0
    Replacement::draft(0x141900310, "unknown_141900310", tpl_34319::<0x143c8e660, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141900310
    Replacement::draft(0x141900370, "unknown_141900370", tpl_496 as *const ()), // sub_141900370
    Replacement::draft(0x141900380, "unknown_141900380", fn_141900380 as *const ()), // sub_141900380
    Replacement::draft(0x141900580, "unknown_141900580", tpl_361 as *const ()), // sub_141900580
    Replacement::draft(0x141900590, "unknown_141900590", fn_141900590 as *const ()), // sub_141900590
    Replacement::draft(0x141900820, "unknown_141900820", fn_141900820 as *const ()), // sub_141900820
    Replacement::draft(0x141900840, "unknown_141900840", tpl_361 as *const ()), // sub_141900840
    Replacement::draft(0x141900850, "unknown_141900850", fn_141900850 as *const ()), // sub_141900850
    Replacement::draft(0x1419008d0, "unknown_1419008d0", tpl_361 as *const ()), // sub_1419008d0
    Replacement::draft(0x1419008e0, "unknown_1419008e0", fn_1419008e0 as *const ()), // sub_1419008e0
    Replacement::draft(0x141900ac0, "unknown_141900ac0", tpl_361 as *const ()), // sub_141900ac0
    Replacement::draft(0x141900ad0, "unknown_141900ad0", fn_141900ad0 as *const ()), // sub_141900ad0
    Replacement::draft(0x141900b90, "unknown_141900b90", fn_141900b90 as *const ()), // sub_141900b90
    Replacement::draft(0x141901000, "unknown_141901000", tpl_361 as *const ()), // sub_141901000
    Replacement::draft(0x141901010, "unknown_141901010", tpl_361 as *const ()), // sub_141901010
    Replacement::draft_throws(0x141901020, "unknown_141901020", fn_141901020 as *const (), Throws::Discard), // sub_141901020
    Replacement::draft(0x141901390, "unknown_141901390", tpl_361 as *const ()), // sub_141901390
    Replacement::draft(0x1419013a0, "unknown_1419013a0", tpl_496 as *const ()), // sub_1419013a0
    Replacement::draft_throws(0x1419013b0, "unknown_1419013b0", fn_1419013b0 as *const (), Throws::Caught), // sub_1419013b0
    Replacement::draft(0x1419016e0, "unknown_1419016e0", tpl_361 as *const ()), // sub_1419016e0
    Replacement::draft(0x1419016f0, "unknown_1419016f0", tpl_361 as *const ()), // sub_1419016f0
    Replacement::draft(0x141901700, "unknown_141901700", tpl_496 as *const ()), // sub_141901700
    Replacement::draft(0x141901710, "unknown_141901710", tpl_496 as *const ()), // sub_141901710
    Replacement::draft(0x141901720, "unknown_141901720", fn_141901720 as *const ()), // sub_141901720
    Replacement::draft(0x141902180, "unknown_141902180", tpl_361 as *const ()), // sub_141902180
    Replacement::draft(0x141902190, "unknown_141902190", tpl_34443 as *const ()), // sub_141902190
    Replacement::draft(0x1419021a0, "unknown_1419021a0", fn_1419021a0 as *const ()), // sub_1419021a0
    Replacement::draft(0x141902210, "unknown_141902210", tpl_496 as *const ()), // sub_141902210
    Replacement::draft(0x141902220, "unknown_141902220", fn_141902220 as *const ()), // sub_141902220
    Replacement::draft(0x1419026b0, "unknown_1419026b0", tpl_361 as *const ()), // sub_1419026b0
    Replacement::draft(0x1419026c0, "unknown_1419026c0", fn_1419026c0 as *const ()), // sub_1419026c0
    Replacement::draft(0x1419026f0, "unknown_1419026f0", tpl_361 as *const ()), // sub_1419026f0
    Replacement::draft(0x141902700, "unknown_141902700", fn_141902700 as *const ()), // sub_141902700
    Replacement::draft(0x1419027a0, "unknown_1419027a0", tpl_361 as *const ()), // sub_1419027a0
    Replacement::draft(0x1419027b0, "unknown_1419027b0", tpl_34443 as *const ()), // sub_1419027b0
    Replacement::draft(0x1419027c0, "unknown_1419027c0", fn_1419027c0 as *const ()), // sub_1419027c0
    Replacement::draft_throws(0x141902830, "unknown_141902830", fn_141902830 as *const (), Throws::Caught), // sub_141902830
    Replacement::draft(0x141902c90, "unknown_141902c90", tpl_361 as *const ()), // sub_141902c90
    Replacement::draft(0x141902ca0, "unknown_141902ca0", fn_141902ca0 as *const ()), // sub_141902ca0
    Replacement::draft(0x141902e40, "unknown_141902e40", tpl_361 as *const ()), // sub_141902e40
    Replacement::draft(0x141902e50, "unknown_141902e50", tpl_496 as *const ()), // sub_141902e50
    Replacement::draft(0x141902e60, "unknown_141902e60", tpl_361 as *const ()), // sub_141902e60
    Replacement::draft_throws(0x141902e70, "unknown_141902e70", fn_141902e70 as *const (), Throws::Caught), // sub_141902e70
    Replacement::draft(0x1419048c0, "unknown_1419048c0", tpl_361 as *const ()), // sub_1419048c0
    Replacement::draft(0x1419048d0, "unknown_1419048d0", fn_1419048d0 as *const ()), // sub_1419048d0
    Replacement::draft_throws(0x141904a60, "unknown_141904a60", fn_141904a60 as *const (), Throws::Discard), // sub_141904a60
    Replacement::draft(0x141904cb0, "unknown_141904cb0", fn_141904cb0 as *const ()), // sub_141904cb0
    Replacement::draft(0x141904d30, "unknown_141904d30", tpl_496 as *const ()), // sub_141904d30
    Replacement::draft(0x141904d40, "unknown_141904d40", fn_141904d40 as *const ()), // sub_141904d40
    Replacement::draft(0x141904fd0, "unknown_141904fd0", tpl_361 as *const ()), // sub_141904fd0
    Replacement::draft(0x141904fe0, "unknown_141904fe0", tpl_496 as *const ()), // sub_141904fe0
    Replacement::draft(0x141904ff0, "unknown_141904ff0", tpl_496 as *const ()), // sub_141904ff0
    Replacement::draft(0x141905000, "unknown_141905000", fn_141905000 as *const ()), // sub_141905000
    Replacement::draft(0x1419058b0, "unknown_1419058b0", tpl_361 as *const ()), // sub_1419058b0
    Replacement::draft(0x1419058c0, "unknown_1419058c0", tpl_361 as *const ()), // sub_1419058c0
    Replacement::draft(0x1419058d0, "unknown_1419058d0", fn_1419058d0 as *const ()), // sub_1419058d0
    Replacement::draft(0x141905ce0, "unknown_141905ce0", fn_141905ce0 as *const ()), // sub_141905ce0
    Replacement::draft(0x141905eb0, "unknown_141905eb0", tpl_361 as *const ()), // sub_141905eb0
    Replacement::draft(0x141905ec0, "unknown_141905ec0", tpl_361 as *const ()), // sub_141905ec0
    Replacement::draft(0x141905ed0, "unknown_141905ed0", tpl_361 as *const ()), // sub_141905ed0
    Replacement::draft(0x141905ee0, "unknown_141905ee0", tpl_361 as *const ()), // sub_141905ee0
    Replacement::draft_throws(0x141905ef0, "unknown_141905ef0", fn_141905ef0 as *const (), Throws::Caught), // sub_141905ef0
    Replacement::draft(0x1419073c0, "unknown_1419073c0", tpl_361 as *const ()), // sub_1419073c0
    Replacement::draft_throws(0x1419073d0, "unknown_1419073d0", fn_1419073d0 as *const (), Throws::Discard), // sub_1419073d0
    Replacement::draft(0x1419077c0, "unknown_1419077c0", tpl_361 as *const ()), // sub_1419077c0
    Replacement::draft(0x1419077d0, "unknown_1419077d0", fn_1419077d0 as *const ()), // sub_1419077d0
    Replacement::draft(0x141907840, "unknown_141907840", fn_141907840 as *const ()), // sub_141907840
    Replacement::draft(0x141907cd0, "unknown_141907cd0", tpl_361 as *const ()), // sub_141907cd0
    Replacement::draft(0x141907ce0, "unknown_141907ce0", fn_141907ce0 as *const ()), // sub_141907ce0
    Replacement::draft(0x141907dc0, "unknown_141907dc0", tpl_361 as *const ()), // sub_141907dc0
    Replacement::draft(0x141907dd0, "unknown_141907dd0", fn_141907dd0 as *const ()), // sub_141907dd0
    Replacement::draft(0x141907f50, "unknown_141907f50", tpl_361 as *const ()), // sub_141907f50
    Replacement::draft_throws(0x141907f60, "unknown_141907f60", fn_141907f60 as *const (), Throws::Caught), // sub_141907f60
    Replacement::draft(0x1419085b0, "unknown_1419085b0", tpl_361 as *const ()), // sub_1419085b0
    Replacement::draft(0x1419085c0, "unknown_1419085c0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c94b50, 0x158, 0x14020e8f0> as *const ()), // sub_1419085c0
    Replacement::draft(0x1419086b0, "unknown_1419086b0", tpl_496 as *const ()), // sub_1419086b0
    Replacement::draft(0x1419086c0, "unknown_1419086c0", tpl_361 as *const ()), // sub_1419086c0
    Replacement::draft(0x1419086d0, "unknown_1419086d0", fn_1419086d0 as *const ()), // sub_1419086d0
    Replacement::draft(0x1419089b0, "unknown_1419089b0", tpl_361 as *const ()), // sub_1419089b0
    Replacement::draft(0x1419089c0, "unknown_1419089c0", tpl_496 as *const ()), // sub_1419089c0
    Replacement::draft(0x1419089d0, "unknown_1419089d0", tpl_361 as *const ()), // sub_1419089d0
    Replacement::draft(0x1419089e0, "unknown_1419089e0", fn_1419089e0 as *const ()), // sub_1419089e0
    Replacement::draft(0x141908f00, "unknown_141908f00", fn_141908f00 as *const ()), // sub_141908f00
    Replacement::draft(0x1419090f0, "unknown_1419090f0", tpl_496 as *const ()), // sub_1419090f0
    Replacement::draft(0x141909100, "unknown_141909100", fn_141909100 as *const ()), // sub_141909100
    Replacement::draft(0x141909190, "unknown_141909190", fn_141909190 as *const ()), // sub_141909190
    Replacement::draft(0x1419091c0, "unknown_1419091c0", fn_1419091c0 as *const ()), // sub_1419091c0
    Replacement::draft(0x141909330, "unknown_141909330", tpl_496 as *const ()), // sub_141909330
    Replacement::draft(0x141909340, "unknown_141909340", fn_141909340 as *const ()), // sub_141909340
    Replacement::draft(0x141909560, "unknown_141909560", fn_141909560 as *const ()), // sub_141909560
    Replacement::draft(0x1419095f0, "unknown_1419095f0", fn_1419095f0 as *const ()), // sub_1419095f0
    Replacement::draft(0x141909630, "unknown_141909630", fn_141909630 as *const ()), // sub_141909630
    Replacement::draft(0x141909e30, "unknown_141909e30", fn_141909e30 as *const ()), // sub_141909e30
    Replacement::draft(0x141909ed0, "unknown_141909ed0", fn_141909ed0 as *const ()), // sub_141909ed0
    Replacement::draft_throws(0x141909ef0, "unknown_141909ef0", fn_141909ef0 as *const (), Throws::Discard), // sub_141909ef0
    Replacement::draft(0x14190ac10, "unknown_14190ac10", tpl_34994::<0x143c96598, 0x14387a7a0, 0x14387a798, 0x141834a78, 0x1425df940, 0x141834d18, 0x141834a0c> as *const ()), // sub_14190ac10
    Replacement::draft(0x14190ac80, "unknown_14190ac80", tpl_35332 as *const ()), // sub_14190ac80
    Replacement::draft(0x14190ac90, "unknown_14190ac90", tpl_34994::<0x143c965ec, 0x14387a790, 0x14387a788, 0x141834a78, 0x1425df920, 0x141834d18, 0x141834a0c> as *const ()), // sub_14190ac90
    Replacement::draft(0x14190ad00, "unknown_14190ad00", tpl_35331 as *const ()), // sub_14190ad00
    Replacement::draft(0x14190ad20, "unknown_14190ad20", tpl_34994::<0x143c965e8, 0x14387a780, 0x14387a778, 0x141834a78, 0x1425df930, 0x141834d18, 0x141834a0c> as *const ()), // sub_14190ad20
    Replacement::draft(0x14190ad90, "unknown_14190ad90", tpl_35252 as *const ()), // sub_14190ad90
    Replacement::draft(0x14190adb0, "unknown_14190adb0", tpl_34318 as *const ()), // sub_14190adb0
    Replacement::draft(0x14190add0, "unknown_14190add0", tpl_34318 as *const ()), // sub_14190add0
    Replacement::draft(0x14190adf0, "unknown_14190adf0", tpl_34318 as *const ()), // sub_14190adf0
    Replacement::draft(0x14190ae10, "unknown_14190ae10", tpl_34319::<0x143c9659c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14190ae10
    Replacement::draft(0x14190ae70, "unknown_14190ae70", tpl_34319::<0x143c96594, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14190ae70
    Replacement::draft(0x14190aed0, "unknown_14190aed0", tpl_34319::<0x143c8e710, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14190aed0
    Replacement::draft(0x14190af30, "unknown_14190af30", fn_14190af30 as *const ()), // sub_14190af30
    Replacement::draft(0x14190af70, "unknown_14190af70", tpl_34744::<0x30, 0x8> as *const ()), // sub_14190af70
    Replacement::draft(0x14190af90, "unknown_14190af90", tpl_34744::<0x30, 0x4> as *const ()), // sub_14190af90
    Replacement::draft(0x14190afb0, "unknown_14190afb0", tpl_34744::<0x20, 0x34> as *const ()), // sub_14190afb0
    Replacement::draft(0x14190afd0, "unknown_14190afd0", fn_14190afd0 as *const ()), // sub_14190afd0
    Replacement::draft(0x14190b0a0, "unknown_14190b0a0", fn_14190b0a0 as *const ()), // sub_14190b0a0
    Replacement::draft(0x14190b180, "unknown_14190b180", fn_14190b180 as *const ()), // sub_14190b180
    Replacement::draft(0x14190b250, "unknown_14190b250", fn_14190b250 as *const ()), // sub_14190b250
    Replacement::draft(0x14190b310, "unknown_14190b310", tpl_34744::<0x20, 0x14> as *const ()), // sub_14190b310
    Replacement::draft(0x14190b330, "unknown_14190b330", tpl_35375::<0x20, 0x14190ba10, 0x10> as *const ()), // sub_14190b330
    Replacement::draft(0x14190b380, "unknown_14190b380", tpl_35375::<0x20, 0x14190ba10, 0xc> as *const ()), // sub_14190b380
    Replacement::draft(0x14190b3d0, "unknown_14190b3d0", tpl_35376::<0x20, 0x8> as *const ()), // sub_14190b3d0
    Replacement::draft(0x14190b3f0, "unknown_14190b3f0", tpl_35376::<0x20, 0x7> as *const ()), // sub_14190b3f0
    Replacement::draft(0x14190b410, "unknown_14190b410", tpl_35376::<0x20, 0x6> as *const ()), // sub_14190b410
    Replacement::draft(0x14190b430, "unknown_14190b430", tpl_35376::<0x20, 0x4> as *const ()), // sub_14190b430
    Replacement::draft(0x14190b450, "unknown_14190b450", tpl_34744::<0x20, 0x18> as *const ()), // sub_14190b450
    Replacement::draft(0x14190b470, "unknown_14190b470", fn_14190b470 as *const ()), // sub_14190b470
    Replacement::draft(0x14190b520, "unknown_14190b520", tpl_35378::<0x10, 0x30> as *const ()), // sub_14190b520
    Replacement::draft(0x14190b540, "unknown_14190b540", fn_14190b540 as *const ()), // sub_14190b540
    Replacement::draft(0x14190b600, "unknown_14190b600", fn_14190b600 as *const ()), // sub_14190b600
    Replacement::draft(0x14190b6d0, "unknown_14190b6d0", fn_14190b6d0 as *const ()), // sub_14190b6d0
    Replacement::draft(0x14190b7a0, "unknown_14190b7a0", fn_14190b7a0 as *const ()), // sub_14190b7a0
    Replacement::draft(0x14190b7f0, "unknown_14190b7f0", fn_14190b7f0 as *const ()), // sub_14190b7f0
    Replacement::draft(0x14190b840, "unknown_14190b840", tpl_35384::<0x10, 0xc> as *const ()), // sub_14190b840
    Replacement::draft(0x14190b860, "unknown_14190b860", tpl_35384::<0x10, 0x9> as *const ()), // sub_14190b860
    Replacement::draft(0x14190b880, "unknown_14190b880", tpl_35384::<0x10, 0xb> as *const ()), // sub_14190b880
    Replacement::draft(0x14190b8a0, "unknown_14190b8a0", tpl_35384::<0x10, 0x8> as *const ()), // sub_14190b8a0
    Replacement::draft(0x14190b8c0, "unknown_14190b8c0", tpl_35378::<0x10, 0x24> as *const ()), // sub_14190b8c0
    Replacement::draft(0x14190b8e0, "unknown_14190b8e0", fn_14190b8e0 as *const ()), // sub_14190b8e0
    Replacement::draft(0x14190b990, "unknown_14190b990", tpl_35386::<0x41> as *const ()), // sub_14190b990
    Replacement::draft(0x14190b9b0, "unknown_14190b9b0", tpl_35386::<0x40> as *const ()), // sub_14190b9b0
    Replacement::draft(0x14190b9d0, "unknown_14190b9d0", fn_14190b9d0 as *const ()), // sub_14190b9d0
    Replacement::draft(0x14190ba10, "unknown_14190ba10", fn_14190ba10 as *const ()), // sub_14190ba10
    Replacement::draft(0x14190baf0, "unknown_14190baf0", fn_14190baf0 as *const ()), // sub_14190baf0
    Replacement::draft(0x14190bd30, "unknown_14190bd30", tpl_361 as *const ()), // sub_14190bd30
    Replacement::draft(0x14190bd40, "unknown_14190bd40", tpl_35384::<0x10, 0xa> as *const ()), // sub_14190bd40
    Replacement::draft(0x14190bd60, "unknown_14190bd60", tpl_35386::<0x3e> as *const ()), // sub_14190bd60
    Replacement::draft(0x14190bd80, "unknown_14190bd80", tpl_35386::<0x3f> as *const ()), // sub_14190bd80
    Replacement::draft(0x14190bda0, "unknown_14190bda0", tpl_35386::<0x3c> as *const ()), // sub_14190bda0
    Replacement::draft(0x14190bdc0, "unknown_14190bdc0", tpl_35390::<0x4c> as *const ()), // sub_14190bdc0
    Replacement::draft(0x14190bde0, "unknown_14190bde0", tpl_35391::<0x4> as *const ()), // sub_14190bde0
    Replacement::draft(0x14190be00, "unknown_14190be00", tpl_35390::<0x20> as *const ()), // sub_14190be00
    Replacement::draft(0x14190be20, "unknown_14190be20", tpl_35391::<0x8> as *const ()), // sub_14190be20
    Replacement::draft(0x14190be40, "unknown_14190be40", tpl_35391::<0x18> as *const ()), // sub_14190be40
    Replacement::draft(0x14190be60, "unknown_14190be60", tpl_35386::<0x42> as *const ()), // sub_14190be60
    Replacement::draft(0x14190be80, "unknown_14190be80", tpl_35386::<0x3d> as *const ()), // sub_14190be80
    Replacement::draft(0x14190bea0, "unknown_14190bea0", tpl_35390::<0x34> as *const ()), // sub_14190bea0
    Replacement::draft(0x14190bec0, "unknown_14190bec0", tpl_35390::<0x30> as *const ()), // sub_14190bec0
    Replacement::draft(0x14190bee0, "unknown_14190bee0", tpl_35390::<0x24> as *const ()), // sub_14190bee0
    Replacement::draft(0x14190bf00, "unknown_14190bf00", tpl_35391::<0xc> as *const ()), // sub_14190bf00
    Replacement::draft(0x14190bf20, "unknown_14190bf20", tpl_35391::<0x14> as *const ()), // sub_14190bf20
    Replacement::draft(0x14190bf40, "unknown_14190bf40", tpl_35391::<0x10> as *const ()), // sub_14190bf40
    Replacement::draft(0x14190bf60, "unknown_14190bf60", tpl_35390::<0x2c> as *const ()), // sub_14190bf60
    Replacement::draft(0x14190bf80, "unknown_14190bf80", fn_14190bf80 as *const ()), // sub_14190bf80
    Replacement::draft(0x14190bfb0, "unknown_14190bfb0", tpl_35390::<0x1c> as *const ()), // sub_14190bfb0
    Replacement::draft(0x14190bfd0, "unknown_14190bfd0", tpl_35390::<0x28> as *const ()), // sub_14190bfd0
    Replacement::draft(0x14190bff0, "unknown_14190bff0", tpl_35390::<0x38> as *const ()), // sub_14190bff0
    Replacement::draft(0x14190c010, "unknown_14190c010", tpl_35376::<0x20, 0x5> as *const ()), // sub_14190c010
    Replacement::draft(0x14190c030, "unknown_14190c030", tpl_35390::<0x44> as *const ()), // sub_14190c030
    Replacement::draft(0x14190c050, "unknown_14190c050", tpl_35390::<0x48> as *const ()), // sub_14190c050
    Replacement::draft_throws(0x14190c070, "unknown_14190c070", fn_14190c070 as *const (), Throws::Discard), // sub_14190c070
    Replacement::draft(0x14190c510, "unknown_14190c510", tpl_34994::<0x143c96718, 0x14387a7b0, 0x14387a7a8, 0x141834a78, 0x1425df950, 0x141834d18, 0x141834a0c> as *const ()), // sub_14190c510
    Replacement::draft(0x14190c580, "unknown_14190c580", tpl_94428::<0x1430d3420, 0x8> as *const ()), // sub_14190c580
    Replacement::draft(0x14190c8c0, "unknown_14190c8c0", tpl_35394::<0x9, 0x18, 0x10, 0x140380960> as *const ()), // sub_14190c8c0
    Replacement::draft(0x14190c980, "unknown_14190c980", fn_14190c980 as *const ()), // sub_14190c980
    Replacement::draft_throws(0x14190ca50, "unknown_14190ca50", tpl_35396::<0x14387ef70, 0x140381200, 0x1402116e0, 0x14270d8d0, 0x1410d6ca0, 0x140381230, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x140381510, 0x143823788, 0x140380a40> as *const (), Throws::Discard), // sub_14190ca50
    Replacement::draft(0x14190cd50, "unknown_14190cd50", tpl_35044::<0x140383a80> as *const ()), // sub_14190cd50
    Replacement::draft(0x14190cd80, "unknown_14190cd80", tpl_35044::<0x140383790> as *const ()), // sub_14190cd80
    Replacement::draft(0x14190cdb0, "unknown_14190cdb0", tpl_35044::<0x140383620> as *const ()), // sub_14190cdb0
    Replacement::draft(0x14190cde0, "unknown_14190cde0", tpl_35047::<0x6c, 0x1403834b0> as *const ()), // sub_14190cde0
    Replacement::draft(0x14190ce10, "unknown_14190ce10", fn_14190ce10 as *const ()), // sub_14190ce10
    Replacement::draft(0x14190cea0, "unknown_14190cea0", tpl_35047::<0x64, 0x140383060> as *const ()), // sub_14190cea0
    Replacement::draft(0x14190ced0, "unknown_14190ced0", tpl_35048::<0x58, 0x8, 0x60, 0x140382ee0> as *const ()), // sub_14190ced0
    Replacement::draft(0x14190cf00, "unknown_14190cf00", tpl_35049::<0x20, 0x8, 0x28, 0xc, 0x2c, 0x14, 0x34, 0x18, 0x38, 0x40, 0x24, 0x44, 0x140382d30> as *const ()), // sub_14190cf00
    Replacement::draft(0x14190cf50, "unknown_14190cf50", fn_14190cf50 as *const ()), // sub_14190cf50
    Replacement::draft_throws(0x14190cfa0, "unknown_14190cfa0", fn_14190cfa0 as *const (), Throws::Discard), // sub_14190cfa0
    Replacement::draft(0x14190d0d0, "unknown_14190d0d0", fn_14190d0d0 as *const ()), // sub_14190d0d0
    Replacement::draft_throws(0x14190d120, "unknown_14190d120", fn_14190d120 as *const (), Throws::Discard), // sub_14190d120
    Replacement::draft(0x14190d2f0, "unknown_14190d2f0", fn_14190d2f0 as *const ()), // sub_14190d2f0
    Replacement::draft(0x14190d330, "unknown_14190d330", fn_14190d330 as *const ()), // sub_14190d330
    Replacement::draft(0x14190d4d0, "unknown_14190d4d0", tpl_34612 as *const ()), // sub_14190d4d0
    Replacement::draft(0x14190d560, "unknown_14190d560", tpl_34613 as *const ()), // sub_14190d560
    Replacement::draft(0x14190d5f0, "unknown_14190d5f0", tpl_35038::<0x4> as *const ()), // sub_14190d5f0
    Replacement::draft(0x14190d680, "unknown_14190d680", tpl_35403::<0x4> as *const ()), // sub_14190d680
    Replacement::draft(0x14190d710, "unknown_14190d710", fn_14190d710 as *const ()), // sub_14190d710
    Replacement::draft(0x14190d800, "unknown_14190d800", fn_14190d800 as *const ()), // sub_14190d800
    Replacement::draft(0x14190d870, "unknown_14190d870", tpl_95132::<0x4, 0x14240ab10> as *const ()), // sub_14190d870
    Replacement::draft(0x14190d920, "unknown_14190d920", tpl_95133::<0x1430d36a8, 0x1, 0x8, 0x14038baf0> as *const ()), // sub_14190d920
    Replacement::draft(0x14190efe0, "unknown_14190efe0", fn_14190efe0 as *const ()), // sub_14190efe0
    Replacement::draft_throws(0x14190f1c0, "unknown_14190f1c0", fn_14190f1c0 as *const (), Throws::Caught), // sub_14190f1c0
    Replacement::draft_throws(0x141911500, "unknown_141911500", fn_141911500 as *const (), Throws::Caught), // sub_141911500
    Replacement::draft_throws(0x1419120d0, "unknown_1419120d0", fn_1419120d0 as *const (), Throws::Caught), // sub_1419120d0
    Replacement::draft_throws(0x1419124e0, "unknown_1419124e0", fn_1419124e0 as *const (), Throws::Caught), // sub_1419124e0
    Replacement::draft_throws(0x141912f30, "unknown_141912f30", fn_141912f30 as *const (), Throws::Caught), // sub_141912f30
    Replacement::draft(0x1419132c0, "unknown_1419132c0", fn_1419132c0 as *const ()), // sub_1419132c0
    Replacement::draft(0x141913350, "unknown_141913350", tpl_650::<0x8> as *const ()), // sub_141913350
    Replacement::draft(0x141913360, "unknown_141913360", tpl_1269::<0x8, 0x10, 0x14, 0x1401e84a0> as *const ()), // sub_141913360
    Replacement::draft(0x141913390, "unknown_141913390", tpl_35411::<0x18> as *const ()), // sub_141913390
    Replacement::draft(0x1419133b0, "unknown_1419133b0", fn_1419133b0 as *const ()), // sub_1419133b0
    Replacement::draft(0x141913440, "unknown_141913440", tpl_3241::<0x8, 0x140385800, 0x140385a00, 0x14240ab10> as *const ()), // sub_141913440
    Replacement::draft_throws(0x1419134e0, "unknown_1419134e0", fn_1419134e0 as *const (), Throws::Discard), // sub_1419134e0
    Replacement::draft(0x141913b60, "unknown_141913b60", fn_141913b60 as *const ()), // sub_141913b60
    Replacement::draft(0x141913d30, "unknown_141913d30", tpl_35414::<0x30, 0xb0, 0x100> as *const ()), // sub_141913d30
    Replacement::draft(0x141913dd0, "unknown_141913dd0", tpl_34611::<0x8> as *const ()), // sub_141913dd0
    Replacement::draft(0x141913e60, "unknown_141913e60", fn_141913e60 as *const ()), // sub_141913e60
    Replacement::draft(0x141914150, "unknown_141914150", tpl_34611::<0x8> as *const ()), // sub_141914150
    Replacement::draft(0x141914300, "unknown_141914300", fn_141914300 as *const ()), // sub_141914300
    Replacement::draft(0x141914530, "unknown_141914530", tpl_34611::<0x8> as *const ()), // sub_141914530
    Replacement::draft(0x1419146d0, "unknown_1419146d0", tpl_34605::<0x1430d8740> as *const ()), // sub_1419146d0
    Replacement::draft_throws(0x141915530, "unknown_141915530", fn_141915530 as *const (), Throws::Discard), // sub_141915530
    Replacement::draft(0x141915620, "unknown_141915620", tpl_3280::<0x8, 0x14038e670> as *const ()), // sub_141915620
    Replacement::draft(0x141915660, "unknown_141915660", fn_141915660 as *const ()), // sub_141915660
    Replacement::draft(0x141915850, "unknown_141915850", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c970dc, 0x158, 0x14020e8f0> as *const ()), // sub_141915850
    Replacement::draft(0x141915950, "unknown_141915950", fn_141915950 as *const ()), // sub_141915950
    Replacement::draft(0x141915c90, "unknown_141915c90", fn_141915c90 as *const ()), // sub_141915c90
    Replacement::draft(0x141915da0, "unknown_141915da0", fn_141915da0 as *const ()), // sub_141915da0
    Replacement::draft_throws(0x141915fe0, "unknown_141915fe0", fn_141915fe0 as *const (), Throws::Discard), // sub_141915fe0
    Replacement::draft(0x1419162a0, "unknown_1419162a0", fn_1419162a0 as *const ()), // sub_1419162a0
    Replacement::draft(0x141916730, "unknown_141916730", fn_141916730 as *const ()), // sub_141916730
    Replacement::draft(0x141916880, "unknown_141916880", fn_141916880 as *const ()), // sub_141916880
    Replacement::draft(0x141916a90, "unknown_141916a90", fn_141916a90 as *const ()), // sub_141916a90
    Replacement::draft(0x141916b80, "unknown_141916b80", fn_141916b80 as *const ()), // sub_141916b80
    Replacement::draft(0x141916be0, "unknown_141916be0", fn_141916be0 as *const ()), // sub_141916be0
    Replacement::draft(0x141916c40, "unknown_141916c40", fn_141916c40 as *const ()), // sub_141916c40
    Replacement::draft_throws(0x141916d60, "unknown_141916d60", fn_141916d60 as *const (), Throws::Discard), // sub_141916d60
    Replacement::draft(0x141917e60, "unknown_141917e60", fn_141917e60 as *const ()), // sub_141917e60
    Replacement::draft(0x141917e70, "unknown_141917e70", fn_141917e70 as *const ()), // sub_141917e70
    Replacement::draft(0x141917e80, "unknown_141917e80", fn_141917e80 as *const ()), // sub_141917e80
    Replacement::draft(0x141917ed0, "unknown_141917ed0", fn_141917ed0 as *const ()), // sub_141917ed0
    Replacement::draft_throws(0x141917f20, "unknown_141917f20", fn_141917f20 as *const (), Throws::Caught), // sub_141917f20
    Replacement::draft(0x141917fb0, "unknown_141917fb0", fn_141917fb0 as *const ()), // sub_141917fb0
    Replacement::draft(0x141918030, "unknown_141918030", fn_141918030 as *const ()), // sub_141918030
    Replacement::draft(0x141918040, "unknown_141918040", fn_141918040 as *const ()), // sub_141918040
    Replacement::draft(0x141918060, "unknown_141918060", fn_141918060 as *const ()), // sub_141918060
    Replacement::draft(0x1419180b0, "unknown_1419180b0", fn_1419180b0 as *const ()), // sub_1419180b0
    Replacement::draft(0x1419180e0, "unknown_1419180e0", fn_1419180e0 as *const ()), // sub_1419180e0
    Replacement::draft(0x141918160, "unknown_141918160", fn_141918160 as *const ()), // sub_141918160
    Replacement::draft_throws(0x1419181e0, "unknown_1419181e0", fn_1419181e0 as *const (), Throws::Discard), // sub_1419181e0
    Replacement::draft(0x141918360, "unknown_141918360", fn_141918360 as *const ()), // sub_141918360
    Replacement::draft_throws(0x1419183a0, "unknown_1419183a0", fn_1419183a0 as *const (), Throws::Discard), // sub_1419183a0
    Replacement::draft(0x141918780, "unknown_141918780", fn_141918780 as *const ()), // sub_141918780
    Replacement::draft(0x141918870, "unknown_141918870", fn_141918870 as *const ()), // sub_141918870
    Replacement::draft_throws(0x1419188f0, "unknown_1419188f0", fn_1419188f0 as *const (), Throws::Caught), // sub_1419188f0
    Replacement::draft(0x141919070, "unknown_141919070", tpl_34611::<0x8> as *const ()), // sub_141919070
    Replacement::draft(0x141919100, "unknown_141919100", tpl_34611::<0x8> as *const ()), // sub_141919100
    Replacement::draft(0x141919220, "unknown_141919220", tpl_34775::<0x20, 0x28> as *const ()), // sub_141919220
    Replacement::draft(0x1419192c0, "unknown_1419192c0", fn_1419192c0 as *const ()), // sub_1419192c0
    Replacement::draft(0x141919440, "unknown_141919440", tpl_35414::<0x58, 0x30, 0x90> as *const ()), // sub_141919440
    Replacement::draft(0x1419194e0, "unknown_1419194e0", tpl_34613 as *const ()), // sub_1419194e0
    Replacement::draft(0x14191a300, "unknown_14191a300", fn_14191a300 as *const ()), // sub_14191a300
    Replacement::draft(0x14191a430, "unknown_14191a430", fn_14191a430 as *const ()), // sub_14191a430
    Replacement::draft(0x14191a700, "unknown_14191a700", fn_14191a700 as *const ()), // sub_14191a700
    Replacement::draft(0x14191b190, "unknown_14191b190", fn_14191b190 as *const ()), // sub_14191b190
    Replacement::draft_throws(0x14191b1c0, "unknown_14191b1c0", fn_14191b1c0 as *const (), Throws::Discard), // sub_14191b1c0
    Replacement::draft(0x14191b590, "unknown_14191b590", fn_14191b590 as *const ()), // sub_14191b590
    Replacement::draft(0x14191b680, "unknown_14191b680", fn_14191b680 as *const ()), // sub_14191b680
    Replacement::draft(0x14191b760, "unknown_14191b760", tpl_34671::<0x20> as *const ()), // sub_14191b760
    Replacement::draft(0x14191b800, "unknown_14191b800", tpl_34394::<0x10> as *const ()), // sub_14191b800
    Replacement::draft_throws(0x14191bd50, "unknown_14191bd50", fn_14191bd50 as *const (), Throws::Discard), // sub_14191bd50
    Replacement::draft(0x14191be30, "unknown_14191be30", fn_14191be30 as *const ()), // sub_14191be30
    Replacement::draft(0x14191bf40, "unknown_14191bf40", fn_14191bf40 as *const ()), // sub_14191bf40
    Replacement::draft(0x14191bfc0, "unknown_14191bfc0", fn_14191bfc0 as *const ()), // sub_14191bfc0
    Replacement::draft(0x14191c720, "unknown_14191c720", fn_14191c720 as *const ()), // sub_14191c720
    Replacement::draft_throws(0x14191ce50, "unknown_14191ce50", fn_14191ce50 as *const (), Throws::Caught), // sub_14191ce50
    Replacement::draft(0x14191d480, "unknown_14191d480", fn_14191d480 as *const ()), // sub_14191d480
    Replacement::draft_throws(0x14191d750, "unknown_14191d750", fn_14191d750 as *const (), Throws::Discard), // sub_14191d750
    Replacement::draft(0x14191da10, "unknown_14191da10", tpl_34393 as *const ()), // sub_14191da10
    Replacement::draft(0x14191daa0, "unknown_14191daa0", tpl_94428::<0x1430e2a48, 0x8> as *const ()), // sub_14191daa0
    Replacement::draft(0x14191db60, "unknown_14191db60", tpl_35016 as *const ()), // sub_14191db60
    Replacement::draft(0x14191dba0, "unknown_14191dba0", tpl_34318 as *const ()), // sub_14191dba0
    Replacement::draft(0x14191dbc0, "unknown_14191dbc0", tpl_34318 as *const ()), // sub_14191dbc0
    Replacement::draft(0x14191dbe0, "unknown_14191dbe0", tpl_34318 as *const ()), // sub_14191dbe0
    Replacement::draft(0x14191dc00, "unknown_14191dc00", tpl_34318 as *const ()), // sub_14191dc00
    Replacement::draft(0x14191dc20, "unknown_14191dc20", tpl_34318 as *const ()), // sub_14191dc20
    Replacement::draft(0x14191dc40, "unknown_14191dc40", tpl_34318 as *const ()), // sub_14191dc40
    Replacement::draft(0x14191dc60, "unknown_14191dc60", tpl_34318 as *const ()), // sub_14191dc60
    Replacement::draft(0x14191dc80, "unknown_14191dc80", tpl_34318 as *const ()), // sub_14191dc80
    Replacement::draft(0x14191dca0, "unknown_14191dca0", tpl_34319::<0x143c919a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191dca0
    Replacement::draft(0x14191dd00, "unknown_14191dd00", tpl_34319::<0x143c97ec4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191dd00
    Replacement::draft(0x14191dd60, "unknown_14191dd60", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191dd60
    Replacement::draft(0x14191ddc0, "unknown_14191ddc0", tpl_34319::<0x143c8e840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191ddc0
    Replacement::draft(0x14191de20, "unknown_14191de20", tpl_34319::<0x143c8e658, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191de20
    Replacement::draft(0x14191de80, "unknown_14191de80", tpl_34319::<0x143c91f94, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191de80
    Replacement::draft(0x14191dee0, "unknown_14191dee0", tpl_34319::<0x143c8d508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191dee0
    Replacement::draft(0x14191df40, "unknown_14191df40", tpl_34319::<0x143c97f04, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191df40
    Replacement::draft(0x14191dfa0, "std_14191dfa0", tpl_23342::<0x1430e2bb0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::melee::update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x14191dfc0, "std_14191dfc0", tpl_23342::<0x1430e2bb0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::melee::update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x14191dfe0, "std_14191dfe0", fn_14191dfe0 as *const ()), // std::_Func_impl_no_alloc<bigfish::melee::update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x14191e0a0, "std_14191e0a0", tpl_1::<0x1438875b0> as *const ()), // std::_Func_impl_no_alloc<bigfish::melee::update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x14191e0b0, "std_14191e0b0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::melee::update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x14191e0c0, "std_14191e0c0", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::melee::update::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x14191e0e0, "unknown_14191e0e0", tpl_34318 as *const ()), // sub_14191e0e0
    Replacement::draft(0x14191e100, "unknown_14191e100", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191e100
    Replacement::draft(0x14191e160, "unknown_14191e160", tpl_34318 as *const ()), // sub_14191e160
    Replacement::draft(0x14191e180, "unknown_14191e180", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_14191e180
    Replacement::draft(0x14191e1e0, "unknown_14191e1e0", tpl_361 as *const ()), // sub_14191e1e0
    Replacement::draft(0x14191e1f0, "unknown_14191e1f0", tpl_361 as *const ()), // sub_14191e1f0
    Replacement::draft(0x14191e200, "unknown_14191e200", tpl_22031::<0x8> as *const ()), // sub_14191e200
    Replacement::draft(0x14191e220, "unknown_14191e220", fn_14191e220 as *const ()), // sub_14191e220
    Replacement::draft(0x14191e2e0, "unknown_14191e2e0", tpl_496 as *const ()), // sub_14191e2e0
    Replacement::draft(0x14191e2f0, "unknown_14191e2f0", fn_14191e2f0 as *const ()), // sub_14191e2f0
    Replacement::draft(0x14191f5d0, "unknown_14191f5d0", tpl_361 as *const ()), // sub_14191f5d0
    Replacement::draft(0x14191f5e0, "unknown_14191f5e0", tpl_3356::<0x8, 0x14039d480, 0x14039d680> as *const ()), // sub_14191f5e0
    Replacement::draft_throws(0x14191f670, "unknown_14191f670", tpl_35463::<0x142712890, 0x1410d6ca0, 0x14039d220, 0x143823b98, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x14387f0e8, 0x14039dc70, 0x1402116e0> as *const (), Throws::Discard), // sub_14191f670
    Replacement::draft(0x14191f830, "unknown_14191f830", tpl_34994::<0x143c9879c, 0x14387a9d0, 0x14387a9c8, 0x141834a78, 0x1425e0c70, 0x141834d18, 0x141834a0c> as *const ()), // sub_14191f830
    Replacement::draft(0x14191f8a0, "unknown_14191f8a0", tpl_35252 as *const ()), // sub_14191f8a0
    Replacement::draft(0x14191f8c0, "unknown_14191f8c0", tpl_34994::<0x143c98384, 0x14387a9f0, 0x14387a9e8, 0x141834a78, 0x1425e0c60, 0x141834d18, 0x141834a0c> as *const ()), // sub_14191f8c0
    Replacement::draft(0x14191f930, "unknown_14191f930", tpl_35252 as *const ()), // sub_14191f930
    Replacement::draft(0x14191f950, "unknown_14191f950", tpl_34994::<0x143c983c8, 0x14387aa00, 0x14387a9f8, 0x141834a78, 0x1425e0c30, 0x141834d18, 0x141834a0c> as *const ()), // sub_14191f950
    Replacement::draft(0x14191f9c0, "unknown_14191f9c0", tpl_34393 as *const ()), // sub_14191f9c0
    Replacement::draft(0x14191f9e0, "unknown_14191f9e0", tpl_34994::<0x143c9838c, 0x14387a9e0, 0x14387a9d8, 0x141834a78, 0x1425e0c40, 0x141834d18, 0x141834a0c> as *const ()), // sub_14191f9e0
    Replacement::draft(0x14191fa50, "unknown_14191fa50", tpl_34671::<0x20> as *const ()), // sub_14191fa50
    Replacement::draft(0x14191fa80, "unknown_14191fa80", tpl_34994::<0x143c98254, 0x14387aa10, 0x14387aa08, 0x141834a78, 0x1425e0c50, 0x141834d18, 0x141834a0c> as *const ()), // sub_14191fa80
    Replacement::draft(0x14191faf0, "unknown_14191faf0", fn_14191faf0 as *const ()), // sub_14191faf0
    Replacement::draft(0x14191fbd0, "unknown_14191fbd0", fn_14191fbd0 as *const ()), // sub_14191fbd0
    Replacement::draft(0x141921130, "unknown_141921130", fn_141921130 as *const ()), // sub_141921130
    Replacement::draft_throws(0x1419211f0, "unknown_1419211f0", fn_1419211f0 as *const (), Throws::Discard), // sub_1419211f0
    Replacement::draft_throws(0x141921f50, "unknown_141921f50", fn_141921f50 as *const (), Throws::Caught), // sub_141921f50
    Replacement::draft_throws(0x1419239c0, "unknown_1419239c0", fn_1419239c0 as *const (), Throws::Discard), // sub_1419239c0
    Replacement::draft_throws(0x141923bb0, "unknown_141923bb0", fn_141923bb0 as *const (), Throws::Discard), // sub_141923bb0
    Replacement::draft(0x141923c90, "unknown_141923c90", fn_141923c90 as *const ()), // sub_141923c90
    Replacement::draft_throws(0x141923ed0, "unknown_141923ed0", fn_141923ed0 as *const (), Throws::Discard), // sub_141923ed0
    Replacement::draft(0x141924180, "unknown_141924180", fn_141924180 as *const ()), // sub_141924180
    Replacement::draft(0x1419243a0, "unknown_1419243a0", fn_1419243a0 as *const ()), // sub_1419243a0
    Replacement::draft(0x141924440, "unknown_141924440", fn_141924440 as *const ()), // sub_141924440
    Replacement::draft_throws(0x1419245e0, "unknown_1419245e0", fn_1419245e0 as *const (), Throws::Discard), // sub_1419245e0
    Replacement::draft(0x141924c40, "unknown_141924c40", fn_141924c40 as *const ()), // sub_141924c40
    Replacement::draft(0x141924f10, "unknown_141924f10", fn_141924f10 as *const ()), // sub_141924f10
    Replacement::draft(0x141924f50, "unknown_141924f50", fn_141924f50 as *const ()), // sub_141924f50
    Replacement::draft(0x141924f90, "unknown_141924f90", fn_141924f90 as *const ()), // sub_141924f90
    Replacement::draft_throws(0x141924fe0, "unknown_141924fe0", fn_141924fe0 as *const (), Throws::Caught), // sub_141924fe0
    Replacement::draft(0x141925350, "unknown_141925350", fn_141925350 as *const ()), // sub_141925350
    Replacement::draft_throws(0x141925370, "unknown_141925370", fn_141925370 as *const (), Throws::Discard), // sub_141925370
    Replacement::draft_throws(0x141925b60, "unknown_141925b60", fn_141925b60 as *const (), Throws::Discard), // sub_141925b60
    Replacement::draft(0x141925be0, "unknown_141925be0", fn_141925be0 as *const ()), // sub_141925be0
    Replacement::draft_throws(0x141925cb0, "unknown_141925cb0", fn_141925cb0 as *const (), Throws::Discard), // sub_141925cb0
    Replacement::draft(0x141926010, "unknown_141926010", tpl_34611::<0x8> as *const ()), // sub_141926010
    Replacement::draft(0x1419261c0, "unknown_1419261c0", fn_1419261c0 as *const ()), // sub_1419261c0
    Replacement::draft(0x141926260, "unknown_141926260", fn_141926260 as *const ()), // sub_141926260
    Replacement::draft_throws(0x141926750, "unknown_141926750", fn_141926750 as *const (), Throws::Caught), // sub_141926750
    Replacement::draft_throws(0x141926bd0, "unknown_141926bd0", fn_141926bd0 as *const (), Throws::Caught), // sub_141926bd0
    Replacement::draft(0x141926e80, "unknown_141926e80", tpl_3396::<0x8, 0x1403a7ee0, 0x1403a80e0, 0xc> as *const ()), // sub_141926e80
    Replacement::draft_throws(0x141926f10, "unknown_141926f10", fn_141926f10 as *const (), Throws::Discard), // sub_141926f10
    Replacement::draft(0x141927020, "unknown_141927020", tpl_94771::<0x1401ff5b0> as *const ()), // sub_141927020
    Replacement::draft(0x1419270e0, "unknown_1419270e0", tpl_34684::<0x1401f2bc0> as *const ()), // sub_1419270e0
    Replacement::draft(0x141927190, "unknown_141927190", tpl_95219::<0x1430ebb50, 0x8, 0x10> as *const ()), // sub_141927190
    Replacement::draft(0x141927240, "unknown_141927240", tpl_35484::<0x1430ebb50, 0x8, 0x10> as *const ()), // sub_141927240
    Replacement::draft(0x141927320, "unknown_141927320", fn_141927320 as *const ()), // sub_141927320
    Replacement::draft(0x141927400, "unknown_141927400", fn_141927400 as *const ()), // sub_141927400
    Replacement::draft(0x141927520, "unknown_141927520", tpl_34605::<0x1430ebcc8> as *const ()), // sub_141927520
    Replacement::draft(0x141927660, "unknown_141927660", tpl_34395::<0x10, 0x18> as *const ()), // sub_141927660
    Replacement::draft(0x141927cb0, "unknown_141927cb0", fn_141927cb0 as *const ()), // sub_141927cb0
    Replacement::draft(0x141927e60, "unknown_141927e60", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c98e00, 0x158, 0x14020e8f0> as *const ()), // sub_141927e60
    Replacement::draft(0x141927f60, "unknown_141927f60", fn_141927f60 as *const ()), // sub_141927f60
    Replacement::draft(0x141928020, "unknown_141928020", fn_141928020 as *const ()), // sub_141928020
    Replacement::draft(0x1419281b0, "unknown_1419281b0", fn_1419281b0 as *const ()), // sub_1419281b0
    Replacement::draft(0x1419283b0, "unknown_1419283b0", fn_1419283b0 as *const ()), // sub_1419283b0
    Replacement::draft(0x141928750, "unknown_141928750", fn_141928750 as *const ()), // sub_141928750
    Replacement::draft(0x141928780, "unknown_141928780", fn_141928780 as *const ()), // sub_141928780
    Replacement::draft(0x141929500, "unknown_141929500", fn_141929500 as *const ()), // sub_141929500
    Replacement::draft(0x141929780, "unknown_141929780", fn_141929780 as *const ()), // sub_141929780
    Replacement::draft_throws(0x141929f90, "unknown_141929f90", fn_141929f90 as *const (), Throws::Discard), // sub_141929f90
    Replacement::draft(0x14192a780, "unknown_14192a780", fn_14192a780 as *const ()), // sub_14192a780
    Replacement::draft(0x14192b050, "unknown_14192b050", fn_14192b050 as *const ()), // sub_14192b050
    Replacement::draft_throws(0x14192b230, "unknown_14192b230", fn_14192b230 as *const (), Throws::Discard), // sub_14192b230
    Replacement::draft_throws(0x14192b930, "unknown_14192b930", fn_14192b930 as *const (), Throws::Discard), // sub_14192b930
    Replacement::draft(0x14192bdf0, "unknown_14192bdf0", fn_14192bdf0 as *const ()), // sub_14192bdf0
    Replacement::draft(0x14192c000, "unknown_14192c000", fn_14192c000 as *const ()), // sub_14192c000
    Replacement::draft(0x14192c030, "unknown_14192c030", fn_14192c030 as *const ()), // sub_14192c030
    Replacement::draft(0x14192c440, "unknown_14192c440", fn_14192c440 as *const ()), // sub_14192c440
    Replacement::draft_throws(0x14192c7f0, "unknown_14192c7f0", fn_14192c7f0 as *const (), Throws::Caught), // sub_14192c7f0
    Replacement::draft(0x14192cd90, "unknown_14192cd90", fn_14192cd90 as *const ()), // sub_14192cd90
    Replacement::draft(0x14192d1a0, "unknown_14192d1a0", fn_14192d1a0 as *const ()), // sub_14192d1a0
    Replacement::draft(0x14192d1d0, "unknown_14192d1d0", fn_14192d1d0 as *const ()), // sub_14192d1d0
    Replacement::draft(0x14192d280, "unknown_14192d280", fn_14192d280 as *const ()), // sub_14192d280
    Replacement::draft(0x14192d5a0, "unknown_14192d5a0", fn_14192d5a0 as *const ()), // sub_14192d5a0
    Replacement::draft(0x14192d9f0, "unknown_14192d9f0", fn_14192d9f0 as *const ()), // sub_14192d9f0
    Replacement::draft(0x14192da70, "unknown_14192da70", fn_14192da70 as *const ()), // sub_14192da70
    Replacement::draft(0x14192da90, "unknown_14192da90", fn_14192da90 as *const ()), // sub_14192da90
    Replacement::draft_throws(0x14192dac0, "unknown_14192dac0", fn_14192dac0 as *const (), Throws::Discard), // sub_14192dac0
    Replacement::draft(0x14192dde0, "unknown_14192dde0", fn_14192dde0 as *const ()), // sub_14192dde0
    Replacement::draft(0x14192de40, "unknown_14192de40", fn_14192de40 as *const ()), // sub_14192de40
    Replacement::draft(0x14192deb0, "unknown_14192deb0", fn_14192deb0 as *const ()), // sub_14192deb0
    Replacement::draft(0x14192dfa0, "unknown_14192dfa0", tpl_35502::<0x143847270, 0x143426514> as *const ()), // sub_14192dfa0
    Replacement::draft(0x14192dfc0, "unknown_14192dfc0", tpl_35502::<0x143847578, 0x143426514> as *const ()), // sub_14192dfc0
    Replacement::draft(0x14192dfe0, "unknown_14192dfe0", fn_14192dfe0 as *const ()), // sub_14192dfe0
    Replacement::draft_throws(0x14192f000, "unknown_14192f000", fn_14192f000 as *const (), Throws::Discard), // sub_14192f000
    Replacement::draft_throws(0x14192f2d0, "unknown_14192f2d0", fn_14192f2d0 as *const (), Throws::Caught), // sub_14192f2d0
    Replacement::draft(0x14192f630, "unknown_14192f630", tpl_94875::<0x8, 0xc> as *const ()), // sub_14192f630
    Replacement::draft(0x14192f730, "unknown_14192f730", tpl_34684::<0x1401de830> as *const ()), // sub_14192f730
    Replacement::draft(0x14192f890, "unknown_14192f890", tpl_35505::<0x20, 0x40> as *const ()), // sub_14192f890
    Replacement::draft(0x14192f940, "unknown_14192f940", tpl_34671::<0x20> as *const ()), // sub_14192f940
    Replacement::draft(0x14192f9e0, "unknown_14192f9e0", tpl_35505::<0x20, 0x40> as *const ()), // sub_14192f9e0
    Replacement::draft(0x141930410, "unknown_141930410", fn_141930410 as *const ()), // sub_141930410
    Replacement::draft_throws(0x141931680, "unknown_141931680", fn_141931680 as *const (), Throws::Discard), // sub_141931680
    Replacement::draft_throws(0x141931b30, "unknown_141931b30", fn_141931b30 as *const (), Throws::Caught), // sub_141931b30
    Replacement::draft_throws(0x141931f40, "unknown_141931f40", fn_141931f40 as *const (), Throws::Discard), // sub_141931f40
    Replacement::draft_throws(0x141932180, "unknown_141932180", fn_141932180 as *const (), Throws::Caught), // sub_141932180
    Replacement::draft_throws(0x141932980, "unknown_141932980", fn_141932980 as *const (), Throws::Caught), // sub_141932980
    Replacement::draft(0x141933110, "unknown_141933110", tpl_35507::<0x143847790, 0x143847798, 0x8, 0x88> as *const ()), // sub_141933110
    Replacement::draft(0x141933140, "unknown_141933140", tpl_35507::<0x143847910, 0x143847918, 0x8, 0x88> as *const ()), // sub_141933140
    Replacement::draft_throws(0x141933180, "unknown_141933180", fn_141933180 as *const (), Throws::Discard), // sub_141933180
    Replacement::draft_throws(0x1419338b0, "unknown_1419338b0", fn_1419338b0 as *const (), Throws::Discard), // sub_1419338b0
    Replacement::draft_throws(0x141933970, "unknown_141933970", fn_141933970 as *const (), Throws::Discard), // sub_141933970
    Replacement::draft_throws(0x1419339d0, "unknown_1419339d0", fn_1419339d0 as *const (), Throws::Discard), // sub_1419339d0
    Replacement::draft(0x141933d20, "unknown_141933d20", fn_141933d20 as *const ()), // sub_141933d20
    Replacement::draft_throws(0x141934020, "unknown_141934020", fn_141934020 as *const (), Throws::Discard), // sub_141934020
    Replacement::draft(0x141934150, "unknown_141934150", fn_141934150 as *const ()), // sub_141934150
    Replacement::draft(0x1419342c0, "unknown_1419342c0", fn_1419342c0 as *const ()), // sub_1419342c0
    Replacement::draft(0x141934430, "unknown_141934430", fn_141934430 as *const ()), // sub_141934430
    Replacement::draft(0x141934c50, "unknown_141934c50", fn_141934c50 as *const ()), // sub_141934c50
    Replacement::draft_throws(0x141934f50, "unknown_141934f50", fn_141934f50 as *const (), Throws::Discard), // sub_141934f50
    Replacement::draft_throws(0x1419353c0, "unknown_1419353c0", fn_1419353c0 as *const (), Throws::Discard), // sub_1419353c0
    Replacement::draft_throws(0x141935440, "unknown_141935440", fn_141935440 as *const (), Throws::Caught), // sub_141935440
    Replacement::draft_throws(0x141936340, "unknown_141936340", fn_141936340 as *const (), Throws::Caught), // sub_141936340
    Replacement::draft_throws(0x141936910, "unknown_141936910", fn_141936910 as *const (), Throws::Caught), // sub_141936910
    Replacement::draft_throws(0x141937820, "unknown_141937820", fn_141937820 as *const (), Throws::Discard), // sub_141937820
    Replacement::draft(0x141937a70, "unknown_141937a70", fn_141937a70 as *const ()), // sub_141937a70
    Replacement::draft(0x141937b80, "unknown_141937b80", fn_141937b80 as *const ()), // sub_141937b80
    Replacement::draft_throws(0x141937d70, "unknown_141937d70", fn_141937d70 as *const (), Throws::Caught), // sub_141937d70
    Replacement::draft_throws(0x141938280, "unknown_141938280", fn_141938280 as *const (), Throws::Caught), // sub_141938280
    Replacement::draft_throws(0x141938630, "unknown_141938630", fn_141938630 as *const (), Throws::Discard), // sub_141938630
    Replacement::draft_throws(0x1419389c0, "unknown_1419389c0", fn_1419389c0 as *const (), Throws::Discard), // sub_1419389c0
    Replacement::draft_throws(0x141938b50, "unknown_141938b50", fn_141938b50 as *const (), Throws::Discard), // sub_141938b50
    Replacement::draft_throws(0x141938d20, "unknown_141938d20", fn_141938d20 as *const (), Throws::Discard), // sub_141938d20
    Replacement::draft_throws(0x141938e60, "unknown_141938e60", fn_141938e60 as *const (), Throws::Discard), // sub_141938e60
    Replacement::draft(0x141938f10, "unknown_141938f10", fn_141938f10 as *const ()), // sub_141938f10
    Replacement::draft(0x141938f50, "unknown_141938f50", tpl_30459::<0x104> as *const ()), // sub_141938f50
    Replacement::draft_throws(0x141938f70, "unknown_141938f70", fn_141938f70 as *const (), Throws::Discard), // sub_141938f70
    Replacement::draft_throws(0x141939000, "unknown_141939000", fn_141939000 as *const (), Throws::Discard), // sub_141939000
    Replacement::draft_throws(0x141939040, "unknown_141939040", fn_141939040 as *const (), Throws::Discard), // sub_141939040
    Replacement::draft_throws(0x141939070, "unknown_141939070", fn_141939070 as *const (), Throws::Caught), // sub_141939070
    Replacement::draft(0x141939090, "unknown_141939090", tpl_35527::<0x1401e7c10, 0x140348930> as *const ()), // sub_141939090
    Replacement::draft_throws(0x1419391d0, "unknown_1419391d0", fn_1419391d0 as *const (), Throws::Discard), // sub_1419391d0
    Replacement::draft_throws(0x141939220, "unknown_141939220", fn_141939220 as *const (), Throws::Caught), // sub_141939220
    Replacement::draft(0x141939440, "unknown_141939440", fn_141939440 as *const ()), // sub_141939440
    Replacement::draft(0x141939660, "unknown_141939660", tpl_30459::<0x100> as *const ()), // sub_141939660
    Replacement::draft_throws(0x141939680, "unknown_141939680", fn_141939680 as *const (), Throws::Discard), // sub_141939680
    Replacement::draft_throws(0x14193a8e0, "bigfish_14193a8e0", fn_14193a8e0 as *const (), Throws::Caught), // bigfish::PersistentWorld::bigfish::PersistentWorld::vfunc_3
    Replacement::draft(0x14193acd0, "bigfish_14193acd0", fn_14193acd0 as *const ()), // bigfish::PersistentWorld::bigfish::PersistentWorld::vfunc_2
    Replacement::draft(0x14193ad00, "bigfish_14193ad00", fn_14193ad00 as *const ()), // sub_14193ad00
    Replacement::draft_throws(0x14193ad90, "bigfish_14193ad90", fn_14193ad90 as *const (), Throws::Caught), // sub_14193ad90
    Replacement::draft(0x14193af10, "bigfish_14193af10", tpl_94771::<0x1401ff5b0> as *const ()), // sub_14193af10
    Replacement::draft(0x14193afd0, "bigfish_14193afd0", tpl_34684::<0x1401f2bc0> as *const ()), // sub_14193afd0
    Replacement::draft(0x14193b230, "bigfish_14193b230", fn_14193b230 as *const ()), // sub_14193b230
    Replacement::draft(0x14193b2e0, "bigfish_14193b2e0", fn_14193b2e0 as *const ()), // sub_14193b2e0
    Replacement::draft(0x14193b3c0, "bigfish_14193b3c0", fn_14193b3c0 as *const ()), // sub_14193b3c0
    Replacement::draft(0x14193b420, "bigfish_14193b420", tpl_35536::<0x28, 0x1401f2bc0> as *const ()), // sub_14193b420
    Replacement::draft(0x14193b4d0, "bigfish_14193b4d0", tpl_95295::<0x1430f6638, 0x8, 0x10, 0x14> as *const ()), // sub_14193b4d0
    Replacement::draft(0x14193b640, "bigfish_14193b640", tpl_34835::<0x8, 0x1403cbf90, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_14193b640
    Replacement::draft(0x14193b6c0, "bigfish_14193b6c0", tpl_34835::<0x8, 0x1403cbf90, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_14193b6c0
    Replacement::draft(0x14193b740, "bigfish_14193b740", tpl_34837::<0x1403cbf90, 0x20, 0x14240aa00> as *const ()), // sub_14193b740
    Replacement::draft(0x14193b790, "bigfish_14193b790", tpl_34837::<0x1403cbf90, 0x20, 0x14240aa00> as *const ()), // sub_14193b790
    Replacement::draft(0x14193b7e0, "bigfish_14193b7e0", tpl_34840::<0x1403cb780, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_14193b7e0
    Replacement::draft(0x14193b850, "bigfish_14193b850", tpl_34840::<0x1403cb780, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_14193b850
    Replacement::draft_throws(0x14193bcc0, "bigfish_14193bcc0", tpl_95296::<0x1403c8090, 0x14309c488, 0x142718248, 0x142718360, 0x142718338, 0x142718290, 0x142718238, 0x142718200, 0x142718348, 0x142718370, 0x142718280, 0x142718310, 0x1427182a8, 0x142718230, 0x142718328, 0x14271831c, 0x140254aa0, 0x1424b16a0, 0x1403c9960> as *const (), Throws::Discard), // sub_14193bcc0
    Replacement::draft_throws(0x14193c500, "bigfish_14193c500", tpl_95296::<0x1403c8090, 0x14309c488, 0x142718248, 0x142718360, 0x142718338, 0x142718290, 0x142718238, 0x142718200, 0x142718348, 0x142718370, 0x142718280, 0x142718310, 0x1427182a8, 0x142718230, 0x142718328, 0x14271831c, 0x140254aa0, 0x1424b16a0, 0x1403c9960> as *const (), Throws::Discard), // sub_14193c500
    Replacement::draft_throws(0x14193cd40, "bigfish_14193cd40", fn_14193cd40 as *const (), Throws::Caught), // sub_14193cd40
    Replacement::draft_throws(0x14193cf40, "bigfish_14193cf40", tpl_35537::<0x18, 0x14193bcc0, 0x10> as *const (), Throws::Discard), // sub_14193cf40
    Replacement::draft_throws(0x14193cf80, "bigfish_14193cf80", tpl_34846::<0x1401f3a20, 0x14193bcc0> as *const (), Throws::Discard), // sub_14193cf80
    Replacement::draft_throws(0x14193d000, "bigfish_14193d000", fn_14193d000 as *const (), Throws::Caught), // sub_14193d000
    Replacement::draft_throws(0x14193d200, "bigfish_14193d200", tpl_35537::<0x18, 0x14193c500, 0x10> as *const (), Throws::Discard), // sub_14193d200
    Replacement::draft_throws(0x14193d240, "bigfish_14193d240", tpl_34846::<0x1401f3a20, 0x14193c500> as *const (), Throws::Discard), // sub_14193d240
    Replacement::draft_throws(0x14193d320, "bigfish_14193d320", fn_14193d320 as *const (), Throws::Caught), // sub_14193d320
    Replacement::draft_throws(0x14193dc60, "bigfish_14193dc60", fn_14193dc60 as *const (), Throws::Discard), // sub_14193dc60
    Replacement::draft_throws(0x14193dd20, "bigfish_14193dd20", fn_14193dd20 as *const (), Throws::Discard), // sub_14193dd20
    Replacement::draft_throws(0x14193dd70, "bigfish_14193dd70", fn_14193dd70 as *const (), Throws::Caught), // sub_14193dd70
    Replacement::draft(0x14193e180, "bigfish_14193e180", fn_14193e180 as *const ()), // sub_14193e180
    Replacement::draft_throws(0x14193e360, "bigfish_14193e360", fn_14193e360 as *const (), Throws::Discard), // sub_14193e360
    Replacement::draft_throws(0x14193e390, "bigfish_14193e390", fn_14193e390 as *const (), Throws::Discard), // sub_14193e390
    Replacement::draft_throws(0x14193e7f0, "bigfish_14193e7f0", fn_14193e7f0 as *const (), Throws::Caught), // sub_14193e7f0
    Replacement::draft(0x14193ef40, "bigfish_14193ef40", fn_14193ef40 as *const ()), // sub_14193ef40
    Replacement::draft_throws(0x14193efb0, "bigfish_14193efb0", fn_14193efb0 as *const (), Throws::Discard), // sub_14193efb0
    Replacement::draft(0x14193f330, "bigfish_14193f330", fn_14193f330 as *const ()), // sub_14193f330
    Replacement::draft_throws(0x14193f400, "bigfish_14193f400", fn_14193f400 as *const (), Throws::Discard), // sub_14193f400
    Replacement::draft(0x14193f890, "bigfish_14193f890", fn_14193f890 as *const ()), // sub_14193f890
    Replacement::draft(0x14193f8d0, "bigfish_14193f8d0", tpl_35548::<0x143a1c3b3, 0x143a1c3b0, 0x143a1c3b8, 0x143a1c3b4, 0x1401e8cc0, 0x1401e8c00, 0x143c8ceb4, 0x143a1c9e3, 0x143a1c9e0, 0x143a1c9e8, 0x143a1c9e4, 0x141df9270> as *const ()), // sub_14193f8d0
    Replacement::draft(0x14193fa20, "bigfish_14193fa20", tpl_35548::<0x143a1c3b3, 0x143a1c3b0, 0x143a1c3b8, 0x143a1c3b4, 0x1401e8cc0, 0x1401e8c00, 0x143c8ceb4, 0x143a1c38b, 0x143a1c388, 0x143a1c390, 0x143a1c38c, 0x141df9270> as *const ()), // sub_14193fa20
    Replacement::draft(0x14193fb70, "bigfish_14193fb70", tpl_35548::<0x143a1c3b3, 0x143a1c3b0, 0x143a1c3b8, 0x143a1c3b4, 0x1401e8cc0, 0x1401e8c00, 0x143c8ceb4, 0x143a1cafb, 0x143a1caf8, 0x143a1cb00, 0x143a1cafc, 0x141df9270> as *const ()), // sub_14193fb70
    Replacement::draft(0x14193fcc0, "bigfish_14193fcc0", tpl_35548::<0x143a1c3b3, 0x143a1c3b0, 0x143a1c3b8, 0x143a1c3b4, 0x1401e8cc0, 0x1401e8c00, 0x143c8ceb4, 0x143a1c593, 0x143a1c590, 0x143a1c598, 0x143a1c594, 0x141df9270> as *const ()), // sub_14193fcc0
    Replacement::draft_throws(0x14193fe10, "bigfish_14193fe10", fn_14193fe10 as *const (), Throws::Discard), // sub_14193fe10
    Replacement::draft_throws(0x14193fea0, "bigfish_14193fea0", fn_14193fea0 as *const (), Throws::Discard), // sub_14193fea0
    Replacement::draft(0x14193ff40, "bigfish_14193ff40", fn_14193ff40 as *const ()), // sub_14193ff40
    Replacement::draft_throws(0x141940070, "bigfish_141940070", fn_141940070 as *const (), Throws::Discard), // sub_141940070
    Replacement::draft(0x1419401f0, "bigfish_1419401f0", fn_1419401f0 as *const ()), // sub_1419401f0
    Replacement::draft(0x141940280, "bigfish_141940280", fn_141940280 as *const ()), // sub_141940280
    Replacement::draft(0x1419404d0, "bigfish_1419404d0", fn_1419404d0 as *const ()), // sub_1419404d0
    Replacement::draft_throws(0x141940530, "bigfish_141940530", fn_141940530 as *const (), Throws::Caught), // sub_141940530
    Replacement::draft(0x141940ab0, "bigfish_141940ab0", tpl_35557::<0x143a1cbe0, 0x143a1cb90, 0x143a1cb98, 0x14240aa00, 0x1403cc710, 0x8, 0x3, 0x4, 0x14240ab10> as *const ()), // sub_141940ab0
    Replacement::draft(0x141940bc0, "bigfish_141940bc0", tpl_35557::<0x143a1cc40, 0x143a1cbf0, 0x143a1cbf8, 0x14240aa00, 0x1403cc710, 0x8, 0x3, 0x4, 0x14240ab10> as *const ()), // sub_141940bc0
    Replacement::draft(0x141940cd0, "bigfish_141940cd0", tpl_35557::<0x143a1cca0, 0x143a1cc50, 0x143a1cc58, 0x14240aa00, 0x1403cc710, 0x8, 0x3, 0x4, 0x14240ab10> as *const ()), // sub_141940cd0
    Replacement::draft(0x141940de0, "bigfish_141940de0", tpl_35558::<0x18> as *const ()), // sub_141940de0
    Replacement::draft_throws(0x141940e00, "bigfish_141940e00", fn_141940e00 as *const (), Throws::Caught), // sub_141940e00
    Replacement::draft(0x141941740, "bigfish_141941740", tpl_94428::<0x1430f6988, 0x8> as *const ()), // sub_141941740
    Replacement::draft_throws(0x141941880, "bigfish_141941880", tpl_35009::<0x14387ef08, 0x1403ccb40, 0x1402116e0> as *const (), Throws::Discard), // sub_141941880
    Replacement::draft_throws(0x1419418c0, "bigfish_1419418c0", fn_1419418c0 as *const (), Throws::Caught), // sub_1419418c0
    Replacement::draft_throws(0x141941960, "bigfish_141941960", fn_141941960 as *const (), Throws::Discard), // sub_141941960
    Replacement::draft(0x141941ca0, "bigfish_141941ca0", fn_141941ca0 as *const ()), // sub_141941ca0
    Replacement::draft(0x141941d20, "bigfish_141941d20", fn_141941d20 as *const ()), // sub_141941d20
    Replacement::draft(0x141941db0, "bigfish_141941db0", fn_141941db0 as *const ()), // sub_141941db0
    Replacement::draft_throws(0x141941e60, "bigfish_141941e60", fn_141941e60 as *const (), Throws::Discard), // sub_141941e60
    Replacement::draft(0x141942190, "bigfish_141942190", fn_141942190 as *const ()), // sub_141942190
    Replacement::draft(0x1419421c0, "bigfish_1419421c0", fn_1419421c0 as *const ()), // sub_1419421c0
    Replacement::draft(0x1419422b0, "bigfish_1419422b0", fn_1419422b0 as *const ()), // sub_1419422b0
    Replacement::draft(0x141942300, "bigfish_141942300", fn_141942300 as *const ()), // sub_141942300
    Replacement::draft(0x141942380, "bigfish_141942380", fn_141942380 as *const ()), // sub_141942380
    Replacement::draft(0x1419423c0, "bigfish_1419423c0", fn_1419423c0 as *const ()), // sub_1419423c0
    Replacement::draft(0x141942430, "bigfish_141942430", fn_141942430 as *const ()), // sub_141942430
    Replacement::draft(0x141942480, "bigfish_141942480", fn_141942480 as *const ()), // sub_141942480
    Replacement::draft(0x1419424a0, "bigfish_1419424a0", fn_1419424a0 as *const ()), // sub_1419424a0
    Replacement::draft(0x1419424f0, "bigfish_1419424f0", tpl_28104::<0x1419423c0> as *const ()), // sub_1419424f0
    Replacement::draft(0x141942500, "bigfish_141942500", fn_141942500 as *const ()), // sub_141942500
    Replacement::draft(0x141942540, "bigfish_141942540", fn_141942540 as *const ()), // sub_141942540
    Replacement::draft(0x141942600, "bigfish_141942600", fn_141942600 as *const ()), // sub_141942600
    Replacement::draft(0x1419427c0, "bigfish_1419427c0", fn_1419427c0 as *const ()), // sub_1419427c0
    Replacement::draft(0x141942870, "bigfish_141942870", tpl_34667::<0x20> as *const ()), // sub_141942870
    Replacement::draft(0x141942910, "bigfish_141942910", fn_141942910 as *const ()), // sub_141942910
    Replacement::draft(0x141942ad0, "bigfish_141942ad0", fn_141942ad0 as *const ()), // sub_141942ad0
    Replacement::draft(0x141942bb0, "bigfish_141942bb0", fn_141942bb0 as *const ()), // sub_141942bb0
    Replacement::draft(0x141942d60, "bigfish_141942d60", fn_141942d60 as *const ()), // sub_141942d60
    Replacement::draft(0x141942e40, "bigfish_141942e40", tpl_35099::<0x20, 0x40> as *const ()), // sub_141942e40
    Replacement::draft(0x141942f80, "bigfish_141942f80", fn_141942f80 as *const ()), // sub_141942f80
    Replacement::draft(0x141943020, "bigfish_141943020", fn_141943020 as *const ()), // sub_141943020
    Replacement::draft(0x141943100, "bigfish_141943100", fn_141943100 as *const ()), // sub_141943100
    Replacement::draft(0x141943230, "bigfish_141943230", fn_141943230 as *const ()), // sub_141943230
    Replacement::draft(0x1419432c0, "bigfish_1419432c0", tpl_34676::<0x28, 0x1401de830> as *const ()), // sub_1419432c0
    Replacement::draft(0x141943370, "bigfish_141943370", fn_141943370 as *const ()), // sub_141943370
    Replacement::draft(0x141943490, "bigfish_141943490", fn_141943490 as *const ()), // sub_141943490
    Replacement::draft(0x141943530, "bigfish_141943530", fn_141943530 as *const ()), // sub_141943530
    Replacement::draft_throws(0x141943be0, "bigfish_141943be0", fn_141943be0 as *const (), Throws::Caught), // sub_141943be0
    Replacement::draft_throws(0x141944260, "bigfish_141944260", fn_141944260 as *const (), Throws::Caught), // sub_141944260
    Replacement::draft_throws(0x1419443f0, "rapidjson_1419443f0", fn_1419443f0 as *const (), Throws::Discard), // sub_1419443f0
    Replacement::draft(0x141944760, "bigfish_141944760", fn_141944760 as *const ()), // sub_141944760
    Replacement::draft(0x1419448c0, "bigfish_1419448c0", tpl_35031::<0x142407c80, 0x143c9a62c, 0x14020e8f0> as *const ()), // sub_1419448c0
    Replacement::draft_throws(0x1419449f0, "bigfish_1419449f0", fn_1419449f0 as *const (), Throws::Caught), // sub_1419449f0
    Replacement::draft_throws(0x141944b70, "bigfish_141944b70", fn_141944b70 as *const (), Throws::Discard), // sub_141944b70
    Replacement::draft(0x1419450c0, "bigfish_1419450c0", fn_1419450c0 as *const ()), // sub_1419450c0
    Replacement::draft(0x141945280, "bigfish_141945280", fn_141945280 as *const ()), // sub_141945280
    Replacement::draft_throws(0x141945560, "bigfish_141945560", fn_141945560 as *const (), Throws::Discard), // sub_141945560
    Replacement::draft(0x141945670, "bigfish_141945670", fn_141945670 as *const ()), // sub_141945670
    Replacement::draft_throws(0x1419457c0, "bigfish_1419457c0", fn_1419457c0 as *const (), Throws::Caught), // sub_1419457c0
    Replacement::draft_throws(0x141945a80, "bigfish_141945a80", fn_141945a80 as *const (), Throws::Caught), // sub_141945a80
    Replacement::draft_throws(0x141945e90, "bigfish_141945e90", fn_141945e90 as *const (), Throws::Caught), // sub_141945e90
    Replacement::draft_throws(0x141946590, "bigfish_141946590", fn_141946590 as *const (), Throws::Caught), // sub_141946590
    Replacement::draft_throws(0x141946920, "bigfish_141946920", fn_141946920 as *const (), Throws::Discard), // sub_141946920
    Replacement::draft(0x141946960, "bigfish_141946960", fn_141946960 as *const ()), // sub_141946960
    Replacement::draft(0x141946c10, "bigfish_141946c10", fn_141946c10 as *const ()), // sub_141946c10
    Replacement::draft_throws(0x141946e30, "bigfish_141946e30", fn_141946e30 as *const (), Throws::Caught), // sub_141946e30
    Replacement::draft_throws(0x1419471f0, "bigfish_1419471f0", fn_1419471f0 as *const (), Throws::Caught), // sub_1419471f0
    Replacement::draft_throws(0x1419475c0, "bigfish_1419475c0", fn_1419475c0 as *const (), Throws::Discard), // sub_1419475c0
    Replacement::draft_throws(0x1419476d0, "bigfish_1419476d0", fn_1419476d0 as *const (), Throws::Caught), // sub_1419476d0
    Replacement::draft_throws(0x141947af0, "bigfish_141947af0", fn_141947af0 as *const (), Throws::Discard), // sub_141947af0
    Replacement::draft_throws(0x141947b30, "bigfish_141947b30", fn_141947b30 as *const (), Throws::Discard), // sub_141947b30
    Replacement::draft_throws(0x141947b60, "bigfish_141947b60", fn_141947b60 as *const (), Throws::Discard), // sub_141947b60
    Replacement::draft_throws(0x141947d40, "bigfish_141947d40", fn_141947d40 as *const (), Throws::Caught), // sub_141947d40
    Replacement::draft_throws(0x141948200, "bigfish_141948200", fn_141948200 as *const (), Throws::Discard), // sub_141948200
    Replacement::draft_throws(0x141948270, "bigfish_141948270", fn_141948270 as *const (), Throws::Discard), // sub_141948270
    Replacement::draft_throws(0x141948300, "bigfish_141948300", fn_141948300 as *const (), Throws::Discard), // sub_141948300
    Replacement::draft(0x1419488a0, "bigfish_1419488a0", tpl_34835::<0x8, 0x140261060, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419488a0
    Replacement::draft(0x141948990, "bigfish_141948990", fn_141948990 as *const ()), // sub_141948990
    Replacement::draft(0x141948d40, "bigfish_141948d40", fn_141948d40 as *const ()), // sub_141948d40
    Replacement::draft(0x141948df0, "bigfish_141948df0", fn_141948df0 as *const ()), // sub_141948df0
    Replacement::draft(0x141948f10, "bigfish_141948f10", fn_141948f10 as *const ()), // sub_141948f10
    Replacement::draft(0x141948ff0, "bigfish_141948ff0", fn_141948ff0 as *const ()), // sub_141948ff0
    Replacement::draft(0x141949030, "bigfish_141949030", tpl_34824::<0x28, 0x14240ab10> as *const ()), // sub_141949030
    Replacement::draft(0x141949110, "bigfish_141949110", tpl_34837::<0x140261060, 0x20, 0x14240aa00> as *const ()), // sub_141949110
    Replacement::draft(0x141949160, "bigfish_141949160", tpl_34840::<0x1403da6d0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141949160
    Replacement::draft_throws(0x141949430, "bigfish_141949430", tpl_94651::<0x14025ca60, 0x14309c488, 0x14271bfa0, 0x14271bfa8, 0x14271bfb4, 0x14271bfd8, 0x140254aa0, 0x1424b16a0, 0x14025f260> as *const (), Throws::Discard), // sub_141949430
    Replacement::draft_throws(0x141949700, "bigfish_141949700", fn_141949700 as *const (), Throws::Caught), // sub_141949700
    Replacement::draft_throws(0x141949900, "bigfish_141949900", tpl_34895::<0x18, 0x141949430, 0x10> as *const (), Throws::Discard), // sub_141949900
    Replacement::draft_throws(0x141949940, "bigfish_141949940", tpl_34846::<0x1401f3a20, 0x141949430> as *const (), Throws::Discard), // sub_141949940
    Replacement::draft(0x141949a00, "bigfish_141949a00", fn_141949a00 as *const ()), // sub_141949a00
    Replacement::draft(0x141949d20, "bigfish_141949d20", fn_141949d20 as *const ()), // sub_141949d20
    Replacement::draft_throws(0x14194a2a0, "bigfish_14194a2a0", fn_14194a2a0 as *const (), Throws::Caught), // sub_14194a2a0
    Replacement::draft(0x14194aa80, "bigfish_14194aa80", fn_14194aa80 as *const ()), // sub_14194aa80
    Replacement::draft(0x14194aae0, "bigfish_14194aae0", tpl_35616::<0x20, 0x8, 0xc, 0x1, 0x1401deba0> as *const ()), // sub_14194aae0
    Replacement::draft(0x14194ab60, "bigfish_14194ab60", tpl_35616::<0x10, 0x8, 0xc, 0x1, 0x1401deba0> as *const ()), // sub_14194ab60
    Replacement::draft(0x14194abe0, "bigfish_14194abe0", fn_14194abe0 as *const ()), // sub_14194abe0
    Replacement::draft(0x14194ada0, "bigfish_14194ada0", fn_14194ada0 as *const ()), // sub_14194ada0
    Replacement::draft_throws(0x14194ae20, "bigfish_14194ae20", fn_14194ae20 as *const (), Throws::Caught), // sub_14194ae20
    Replacement::draft(0x14194b300, "bigfish_14194b300", fn_14194b300 as *const ()), // sub_14194b300
    Replacement::draft(0x14194b3c0, "bigfish_14194b3c0", fn_14194b3c0 as *const ()), // sub_14194b3c0
    Replacement::draft(0x14194b700, "bigfish_14194b700", fn_14194b700 as *const ()), // sub_14194b700
    Replacement::draft(0x14194b850, "bigfish_14194b850", fn_14194b850 as *const ()), // sub_14194b850
    Replacement::draft_throws(0x14194b940, "bigfish_14194b940", fn_14194b940 as *const (), Throws::Discard), // sub_14194b940
    Replacement::draft(0x14194bb50, "bigfish_14194bb50", tpl_94770::<0x1430fddd0, 0x8, 0xc> as *const ()), // sub_14194bb50
    Replacement::draft(0x14194bcd0, "bigfish_14194bcd0", fn_14194bcd0 as *const ()), // sub_14194bcd0
    Replacement::draft(0x14194bd90, "bigfish_14194bd90", fn_14194bd90 as *const ()), // sub_14194bd90
    Replacement::draft(0x14194be80, "bigfish_14194be80", fn_14194be80 as *const ()), // sub_14194be80
    Replacement::draft(0x14194bf50, "bigfish_14194bf50", fn_14194bf50 as *const ()), // sub_14194bf50
    Replacement::draft(0x14194bf90, "bigfish_14194bf90", fn_14194bf90 as *const ()), // sub_14194bf90
    Replacement::draft(0x14194c450, "bigfish_14194c450", tpl_1439::<0x8, 0x1403dcd00> as *const ()), // sub_14194c450
    Replacement::draft(0x14194c490, "bigfish_14194c490", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c9b4c4, 0x158, 0x14020e8f0> as *const ()), // sub_14194c490
    Replacement::draft(0x14194c590, "bigfish_14194c590", fn_14194c590 as *const ()), // sub_14194c590
    Replacement::draft_throws(0x14194c6b0, "bigfish_14194c6b0", fn_14194c6b0 as *const (), Throws::Discard), // sub_14194c6b0
    Replacement::draft_throws(0x14194c960, "bigfish_14194c960", fn_14194c960 as *const (), Throws::Discard), // sub_14194c960
    Replacement::draft_throws(0x14194cce0, "bigfish_14194cce0", fn_14194cce0 as *const (), Throws::Discard), // sub_14194cce0
    Replacement::draft_throws(0x14194ce60, "bigfish_14194ce60", fn_14194ce60 as *const (), Throws::Caught), // sub_14194ce60
    Replacement::draft_throws(0x14194d1f0, "bigfish_14194d1f0", tpl_34795::<0x14271d620, 0x1410d6ca0, 0x1403dcf00, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1403dd1e0, 0x143823788, 0x14271d610, 0x1403dc7e0, 0x1438234b0, 0x14387f574, 0x1403dd4c0, 0x1402116e0> as *const (), Throws::Discard), // sub_14194d1f0
    Replacement::draft(0x14194d650, "bigfish_14194d650", tpl_94810::<0x1430ff190, 0x8> as *const ()), // sub_14194d650
    Replacement::draft(0x14194db50, "bigfish_14194db50", tpl_1439::<0x8, 0x1403df980> as *const ()), // sub_14194db50
    Replacement::draft_throws(0x14194dba0, "bigfish_14194dba0", fn_14194dba0 as *const (), Throws::Discard), // sub_14194dba0
    Replacement::draft(0x14194e060, "bigfish_14194e060", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c9b7bc, 0x158, 0x14020e8f0> as *const ()), // sub_14194e060
    Replacement::draft(0x14194e160, "bigfish_14194e160", tpl_35630::<0x10, 0x8, 0x1403df6c0> as *const ()), // sub_14194e160
    Replacement::draft_throws(0x14194e1c0, "bigfish_14194e1c0", tpl_34795::<0x14271df20, 0x1410d6ca0, 0x1403dfb80, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1403dfe60, 0x143823788, 0x14271df30, 0x1403df370, 0x1438234b0, 0x14387f56c, 0x1403e0150, 0x1402116e0> as *const (), Throws::Discard), // sub_14194e1c0
    Replacement::draft(0x14194e620, "bigfish_14194e620", fn_14194e620 as *const ()), // sub_14194e620
    Replacement::draft(0x14194e6c0, "bigfish_14194e6c0", tpl_34667::<0x20> as *const ()), // sub_14194e6c0
    Replacement::draft(0x14194ec00, "bigfish_14194ec00", fn_14194ec00 as *const ()), // sub_14194ec00
    Replacement::draft(0x14194eed0, "bigfish_14194eed0", fn_14194eed0 as *const ()), // sub_14194eed0
    Replacement::draft(0x14194ef00, "bigfish_14194ef00", fn_14194ef00 as *const ()), // sub_14194ef00
    Replacement::draft(0x14194f050, "bigfish_14194f050", fn_14194f050 as *const ()), // sub_14194f050
    Replacement::draft_throws(0x14194f760, "bigfish_14194f760", fn_14194f760 as *const (), Throws::Discard), // sub_14194f760
    Replacement::draft(0x14194fa70, "bigfish_14194fa70", fn_14194fa70 as *const ()), // sub_14194fa70
    Replacement::draft(0x14194fb60, "bigfish_14194fb60", fn_14194fb60 as *const ()), // sub_14194fb60
    Replacement::draft(0x14194fe10, "bigfish_14194fe10", fn_14194fe10 as *const ()), // sub_14194fe10
    Replacement::draft(0x141950230, "bigfish_141950230", fn_141950230 as *const ()), // sub_141950230
    Replacement::draft(0x141950280, "bigfish_141950280", fn_141950280 as *const ()), // sub_141950280
    Replacement::draft(0x141950440, "bigfish_141950440", fn_141950440 as *const ()), // sub_141950440
    Replacement::draft(0x1419507f0, "bigfish_1419507f0", fn_1419507f0 as *const ()), // sub_1419507f0
    Replacement::draft(0x141950cb0, "bigfish_141950cb0", fn_141950cb0 as *const ()), // sub_141950cb0
    Replacement::draft(0x141951110, "bigfish_141951110", fn_141951110 as *const ()), // sub_141951110
    Replacement::draft(0x141951230, "bigfish_141951230", fn_141951230 as *const ()), // sub_141951230
    Replacement::draft(0x141951270, "bigfish_141951270", fn_141951270 as *const ()), // sub_141951270
    Replacement::draft(0x1419512f0, "bigfish_1419512f0", fn_1419512f0 as *const ()), // sub_1419512f0
    Replacement::draft(0x141951320, "bigfish_141951320", fn_141951320 as *const ()), // sub_141951320
    Replacement::draft(0x1419513a0, "bigfish_1419513a0", fn_1419513a0 as *const ()), // sub_1419513a0
    Replacement::draft(0x141951610, "bigfish_141951610", fn_141951610 as *const ()), // sub_141951610
    Replacement::draft(0x141951690, "bigfish_141951690", fn_141951690 as *const ()), // sub_141951690
    Replacement::draft(0x1419516f0, "bigfish_1419516f0", fn_1419516f0 as *const ()), // sub_1419516f0
    Replacement::draft_throws(0x141951720, "bigfish_141951720", fn_141951720 as *const (), Throws::Discard), // sub_141951720
    Replacement::draft_throws(0x141951b60, "bigfish_141951b60", fn_141951b60 as *const (), Throws::Discard), // sub_141951b60
    Replacement::draft_throws(0x141952570, "bigfish_141952570", fn_141952570 as *const (), Throws::Discard), // sub_141952570
    Replacement::draft_throws(0x1419526c0, "bigfish_1419526c0", fn_1419526c0 as *const (), Throws::Discard), // sub_1419526c0
    Replacement::draft_throws(0x141952c30, "bigfish_141952c30", fn_141952c30 as *const (), Throws::Discard), // sub_141952c30
    Replacement::draft_throws(0x141952e30, "bigfish_141952e30", fn_141952e30 as *const (), Throws::Discard), // sub_141952e30
    Replacement::draft_throws(0x141953580, "bigfish_141953580", fn_141953580 as *const (), Throws::Discard), // sub_141953580
    Replacement::draft_throws(0x1419535a0, "bigfish_1419535a0", fn_1419535a0 as *const (), Throws::Discard), // bigfish::game_ui_page::MenuUIPage::bigfish::game_ui_page::MenuUIPage::vfunc_25
    Replacement::draft_throws(0x1419536c0, "bigfish_1419536c0", fn_1419536c0 as *const (), Throws::Discard), // bigfish::game_ui_page::MenuUIPage::bigfish::game_ui_page::MenuUIPage::vfunc_26
    Replacement::draft(0x141953910, "bigfish_141953910", fn_141953910 as *const ()), // sub_141953910
    Replacement::draft(0x141953980, "cohtml_141953980", tpl_34971::<0x1431013b8, 0x10, 0x1431013e8, 0x143101150, 0x18, 0x142646d10> as *const ()), // sub_141953980
    Replacement::draft_throws(0x1419539f0, "bigfish_1419539f0", fn_1419539f0 as *const (), Throws::Caught), // sub_1419539f0
    Replacement::draft(0x141953ed0, "bigfish_141953ed0", tpl_34835::<0x8, 0x1403ea7a0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141953ed0
    Replacement::draft(0x141953f50, "bigfish_141953f50", fn_141953f50 as *const ()), // sub_141953f50
    Replacement::draft(0x141953ff0, "bigfish_141953ff0", tpl_35658::<0x8, 0x1419552a0, 0x10, 0x18> as *const ()), // sub_141953ff0
    Replacement::draft(0x141954040, "bigfish_141954040", tpl_1663::<0x8, 0x1403e6f50, 0x1403e8060> as *const ()), // sub_141954040
    Replacement::draft(0x141954170, "bigfish_141954170", fn_141954170 as *const ()), // sub_141954170
    Replacement::draft(0x141954290, "bigfish_141954290", fn_141954290 as *const ()), // sub_141954290
    Replacement::draft(0x1419542f0, "bigfish_1419542f0", tpl_35661::<0x8, 0x1419552a0, 0x10, 0x18, 0x1403ea1e0> as *const ()), // sub_1419542f0
    Replacement::draft(0x1419543c0, "bigfish_1419543c0", fn_1419543c0 as *const ()), // sub_1419543c0
    Replacement::draft(0x141954580, "bigfish_141954580", fn_141954580 as *const ()), // sub_141954580
    Replacement::draft(0x141954640, "bigfish_141954640", fn_141954640 as *const ()), // sub_141954640
    Replacement::draft(0x141954800, "bigfish_141954800", fn_141954800 as *const ()), // sub_141954800
    Replacement::draft(0x141954880, "bigfish_141954880", tpl_64455::<0x8, 0x1403e6f50, 0x10, 0x1403ea1e0> as *const ()), // sub_141954880
    Replacement::draft(0x1419548e0, "bigfish_1419548e0", tpl_64455::<0x8, 0x1403e6f50, 0x10, 0x1403ea1e0> as *const ()), // sub_1419548e0
    Replacement::draft(0x141954940, "bigfish_141954940", tpl_35665::<0x8> as *const ()), // sub_141954940
    Replacement::draft(0x141954980, "bigfish_141954980", tpl_35666::<0x8, 0x1419552a0, 0x10, 0x18> as *const ()), // sub_141954980
    Replacement::draft(0x1419549e0, "bigfish_1419549e0", tpl_3604::<0x8, 0x1419552a0, 0x10, 0x18> as *const ()), // sub_1419549e0
    Replacement::draft_throws(0x141954a40, "bigfish_141954a40", fn_141954a40 as *const (), Throws::Caught), // sub_141954a40
    Replacement::draft_throws(0x141954c40, "bigfish_141954c40", fn_141954c40 as *const (), Throws::Caught), // sub_141954c40
    Replacement::draft(0x141954f50, "bigfish_141954f50", fn_141954f50 as *const ()), // sub_141954f50
    Replacement::draft(0x141954f80, "bigfish_141954f80", fn_141954f80 as *const ()), // sub_141954f80
    Replacement::draft(0x1419550b0, "bigfish_1419550b0", tpl_35671::<0x8, 0x1419552a0> as *const ()), // sub_1419550b0
    Replacement::draft(0x1419550e0, "bigfish_1419550e0", tpl_35672 as *const ()), // sub_1419550e0
    Replacement::draft(0x141955190, "unknown_141955190", tpl_35672 as *const ()), // sub_141955190
    Replacement::draft(0x1419551b0, "unknown_1419551b0", tpl_361 as *const ()), // sub_1419551b0
    Replacement::draft(0x1419551c0, "unknown_1419551c0", tpl_35672 as *const ()), // sub_1419551c0
    Replacement::draft(0x1419551e0, "unknown_1419551e0", tpl_361 as *const ()), // sub_1419551e0
    Replacement::draft(0x1419551f0, "unknown_1419551f0", tpl_422::<0x8, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1419551f0
    Replacement::draft(0x141955290, "unknown_141955290", tpl_365::<0x1419551f0> as *const ()), // sub_141955290
    Replacement::draft(0x1419552a0, "unknown_1419552a0", fn_1419552a0 as *const ()), // sub_1419552a0
    Replacement::draft_throws(0x141955320, "unknown_141955320", fn_141955320 as *const (), Throws::Discard), // sub_141955320
    Replacement::draft(0x141955350, "unknown_141955350", tpl_361 as *const ()), // sub_141955350
    Replacement::draft(0x141955360, "unknown_141955360", tpl_361 as *const ()), // sub_141955360
    Replacement::draft(0x1419555e0, "unknown_1419555e0", tpl_35675::<0x8> as *const ()), // sub_1419555e0
    Replacement::draft(0x141955600, "unknown_141955600", fn_141955600 as *const ()), // sub_141955600
    Replacement::draft(0x141955650, "unknown_141955650", tpl_34318 as *const ()), // sub_141955650
    Replacement::draft(0x141955670, "unknown_141955670", tpl_34319::<0x143c9bd60, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141955670
    Replacement::draft(0x1419556d0, "unknown_1419556d0", tpl_34318 as *const ()), // sub_1419556d0
    Replacement::draft(0x1419556f0, "unknown_1419556f0", tpl_34318 as *const ()), // sub_1419556f0
    Replacement::draft(0x141955710, "unknown_141955710", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141955710
    Replacement::draft(0x141955770, "unknown_141955770", tpl_34319::<0x143c9bebc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141955770
    Replacement::draft(0x1419557d0, "unknown_1419557d0", tpl_35677::<0x10, 0x19> as *const ()), // sub_1419557d0
    Replacement::draft(0x141955800, "unknown_141955800", tpl_35678::<0x19> as *const ()), // sub_141955800
    Replacement::draft(0x141955830, "unknown_141955830", tpl_3548::<0x10, 0x19, 0x8> as *const ()), // sub_141955830
    Replacement::draft(0x141955890, "unknown_141955890", tpl_3547::<0x10, 0x19, 0x8> as *const ()), // sub_141955890
    Replacement::draft(0x1419558f0, "unknown_1419558f0", tpl_63163::<0x141955890, 0x141955830> as *const ()), // sub_1419558f0
    Replacement::draft(0x141955c90, "unknown_141955c90", tpl_369 as *const ()), // sub_141955c90
    Replacement::draft(0x141955ca0, "unknown_141955ca0", fn_141955ca0 as *const ()), // sub_141955ca0
    Replacement::draft(0x141955d10, "unknown_141955d10", tpl_34443 as *const ()), // sub_141955d10
    Replacement::draft(0x141955d20, "unknown_141955d20", tpl_35680 as *const ()), // sub_141955d20
    Replacement::draft(0x141955d40, "unknown_141955d40", fn_141955d40 as *const ()), // sub_141955d40
    Replacement::draft(0x141955df0, "unknown_141955df0", tpl_1606::<0x8, 0x1403e6f50, 0x10> as *const ()), // sub_141955df0
    Replacement::draft(0x141955e30, "unknown_141955e30", tpl_35682::<0x141955ca0> as *const ()), // sub_141955e30
    Replacement::draft(0x141955e50, "unknown_141955e50", tpl_35683 as *const ()), // sub_141955e50
    Replacement::draft(0x141955e60, "unknown_141955e60", tpl_23999 as *const ()), // sub_141955e60
    Replacement::draft(0x141955e70, "unknown_141955e70", fn_141955e70 as *const ()), // sub_141955e70
    Replacement::draft(0x141955fd0, "bigfish_141955fd0", tpl_35685::<0x1419552a0, 0x8, 0x10, 0x18> as *const ()), // sub_141955fd0
    Replacement::draft(0x141956010, "bigfish_141956010", tpl_3546::<0x8, 0x1419552a0, 0x10, 0x18> as *const ()), // sub_141956010
    Replacement::draft(0x141956050, "bigfish_141956050", fn_141956050 as *const ()), // sub_141956050
    Replacement::draft_throws(0x141956230, "bigfish_141956230", tpl_95454::<0x1403e62b0, 0x14309c488, 0x142720d70, 0x142720d78, 0x142720d50, 0x142720d40, 0x142720d48, 0x142720d80, 0x140254aa0, 0x1424b16a0, 0x1403e80f0> as *const (), Throws::Discard), // sub_141956230
    Replacement::draft(0x141956650, "bigfish_141956650", tpl_1149::<0x8, 0x1403e6f50, 0x1403e8060> as *const ()), // sub_141956650
    Replacement::draft(0x141956690, "bigfish_141956690", fn_141956690 as *const ()), // sub_141956690
    Replacement::draft(0x141956780, "bigfish_141956780", tpl_1735::<0x8, 0x19, 0x20, 0x1402174f0, 0x10> as *const ()), // sub_141956780
    Replacement::draft(0x141956820, "bigfish_141956820", tpl_3546::<0x8, 0x1419552a0, 0x10, 0x18> as *const ()), // sub_141956820
    Replacement::draft_throws(0x141956860, "bigfish_141956860", fn_141956860 as *const (), Throws::Caught), // sub_141956860
    Replacement::draft_throws(0x1419569a0, "bigfish_1419569a0", tpl_35688::<0x18, 0x141956230, 0x10> as *const (), Throws::Discard), // sub_1419569a0
    Replacement::draft_throws(0x1419569e0, "bigfish_1419569e0", tpl_34846::<0x1401f3a20, 0x141956230> as *const (), Throws::Discard), // sub_1419569e0
    Replacement::draft(0x141956a80, "bigfish_141956a80", tpl_1149::<0x8, 0x1403e6f50, 0x1403e8060> as *const ()), // sub_141956a80
    Replacement::draft_throws(0x141956ab0, "bigfish_141956ab0", fn_141956ab0 as *const (), Throws::Discard), // sub_141956ab0
    Replacement::draft_throws(0x141956d90, "bigfish_141956d90", fn_141956d90 as *const (), Throws::Caught), // sub_141956d90
    Replacement::draft(0x141957c00, "bigfish_141957c00", fn_141957c00 as *const ()), // sub_141957c00
    Replacement::draft_throws(0x141957d30, "bigfish_141957d30", fn_141957d30 as *const (), Throws::Caught), // sub_141957d30
    Replacement::draft_throws(0x1419580a0, "bigfish_1419580a0", fn_1419580a0 as *const (), Throws::Caught), // sub_1419580a0
    Replacement::draft(0x1419585c0, "bigfish_1419585c0", fn_1419585c0 as *const ()), // sub_1419585c0
    Replacement::draft(0x141958880, "bigfish_141958880", fn_141958880 as *const ()), // sub_141958880
    Replacement::draft_throws(0x1419589a0, "bigfish_1419589a0", fn_1419589a0 as *const (), Throws::Caught), // sub_1419589a0
    Replacement::draft_throws(0x141958db0, "bigfish_141958db0", fn_141958db0 as *const (), Throws::Discard), // sub_141958db0
    Replacement::draft(0x141958f30, "bigfish_141958f30", fn_141958f30 as *const ()), // sub_141958f30
    Replacement::draft(0x141958f40, "bigfish_141958f40", fn_141958f40 as *const ()), // sub_141958f40
    Replacement::draft_throws(0x141958f50, "bigfish_141958f50", fn_141958f50 as *const (), Throws::Discard), // sub_141958f50
    Replacement::draft(0x141959270, "bigfish_141959270", fn_141959270 as *const ()), // sub_141959270
    Replacement::draft(0x1419593a0, "bigfish_1419593a0", tpl_94867::<0xc, 0x14240ab10> as *const ()), // sub_1419593a0
    Replacement::draft(0x141959450, "bigfish_141959450", fn_141959450 as *const ()), // sub_141959450
    Replacement::draft(0x141959610, "bigfish_141959610", tpl_35702 as *const ()), // sub_141959610
    Replacement::draft(0x1419596c0, "bigfish_1419596c0", fn_1419596c0 as *const ()), // sub_1419596c0
    Replacement::draft(0x141959810, "bigfish_141959810", fn_141959810 as *const ()), // sub_141959810
    Replacement::draft_throws(0x141959ee0, "bigfish_141959ee0", fn_141959ee0 as *const (), Throws::Discard), // sub_141959ee0
    Replacement::draft(0x14195a010, "bigfish_14195a010", fn_14195a010 as *const ()), // sub_14195a010
    Replacement::draft_throws(0x14195b1f0, "bigfish_14195b1f0", fn_14195b1f0 as *const (), Throws::Discard), // sub_14195b1f0
    Replacement::draft(0x14195b710, "bigfish_14195b710", fn_14195b710 as *const ()), // sub_14195b710
    Replacement::draft_throws(0x14195bac0, "bigfish_14195bac0", fn_14195bac0 as *const (), Throws::Discard), // sub_14195bac0
    Replacement::draft(0x14195c1d0, "bigfish_14195c1d0", fn_14195c1d0 as *const ()), // sub_14195c1d0
    Replacement::draft(0x14195c7b0, "bigfish_14195c7b0", fn_14195c7b0 as *const ()), // sub_14195c7b0
    Replacement::draft(0x14195ca30, "bigfish_14195ca30", fn_14195ca30 as *const ()), // sub_14195ca30
    Replacement::draft(0x14195cc90, "bigfish_14195cc90", fn_14195cc90 as *const ()), // sub_14195cc90
    Replacement::draft(0x14195ce50, "bigfish_14195ce50", fn_14195ce50 as *const ()), // sub_14195ce50
    Replacement::draft(0x14195d180, "bigfish_14195d180", fn_14195d180 as *const ()), // sub_14195d180
    Replacement::draft(0x14195d1b0, "bigfish_14195d1b0", fn_14195d1b0 as *const ()), // sub_14195d1b0
    Replacement::draft(0x14195d380, "bigfish_14195d380", fn_14195d380 as *const ()), // sub_14195d380
    Replacement::draft(0x14195d4f0, "bigfish_14195d4f0", fn_14195d4f0 as *const ()), // sub_14195d4f0
    Replacement::draft(0x14195d950, "bigfish_14195d950", tpl_95486::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_14195d950
    Replacement::draft(0x14195da30, "bigfish_14195da30", tpl_35718::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_14195da30
    Replacement::draft(0x14195da90, "bigfish_14195da90", tpl_35719::<0x10, 0x1401de830> as *const ()), // sub_14195da90
    Replacement::draft(0x14195dee0, "bigfish_14195dee0", fn_14195dee0 as *const ()), // sub_14195dee0
    Replacement::draft_throws(0x14195e470, "bigfish_14195e470", fn_14195e470 as *const (), Throws::Discard), // sub_14195e470
    Replacement::draft(0x14195ec30, "bigfish_14195ec30", fn_14195ec30 as *const ()), // sub_14195ec30
    Replacement::draft(0x14195ecd0, "bigfish_14195ecd0", fn_14195ecd0 as *const ()), // sub_14195ecd0
    Replacement::draft(0x14195ef30, "bigfish_14195ef30", fn_14195ef30 as *const ()), // sub_14195ef30
    Replacement::draft(0x14195f330, "bigfish_14195f330", fn_14195f330 as *const ()), // sub_14195f330
    Replacement::draft(0x14195f3b0, "bigfish_14195f3b0", fn_14195f3b0 as *const ()), // sub_14195f3b0
    Replacement::draft(0x14195f410, "bigfish_14195f410", fn_14195f410 as *const ()), // sub_14195f410
    Replacement::draft(0x14195f490, "bigfish_14195f490", fn_14195f490 as *const ()), // sub_14195f490
    Replacement::draft(0x14195f510, "bigfish_14195f510", fn_14195f510 as *const ()), // sub_14195f510
    Replacement::draft(0x14195f620, "bigfish_14195f620", fn_14195f620 as *const ()), // sub_14195f620
    Replacement::draft(0x14195f810, "bigfish_14195f810", fn_14195f810 as *const ()), // sub_14195f810
    Replacement::draft_throws(0x1419600d0, "bigfish_1419600d0", fn_1419600d0 as *const (), Throws::Discard), // sub_1419600d0
    Replacement::draft_throws(0x141960a90, "bigfish_141960a90", fn_141960a90 as *const (), Throws::Discard), // sub_141960a90
    Replacement::draft(0x141960da0, "bigfish_141960da0", fn_141960da0 as *const ()), // sub_141960da0
    Replacement::draft_throws(0x141961370, "bigfish_141961370", fn_141961370 as *const (), Throws::Discard), // sub_141961370
    Replacement::draft_throws(0x141961d70, "bigfish_141961d70", fn_141961d70 as *const (), Throws::Discard), // sub_141961d70
    Replacement::draft(0x1419621d0, "unknown_1419621d0", fn_1419621d0 as *const ()), // sub_1419621d0
    Replacement::draft(0x141962240, "unknown_141962240", fn_141962240 as *const ()), // sub_141962240
    Replacement::draft_throws(0x141962620, "unknown_141962620", fn_141962620 as *const (), Throws::Caught), // sub_141962620
    Replacement::draft(0x141962d10, "unknown_141962d10", fn_141962d10 as *const ()), // sub_141962d10
    Replacement::draft(0x141962e80, "unknown_141962e80", tpl_34613 as *const ()), // sub_141962e80
    Replacement::draft(0x1419630a0, "unknown_1419630a0", fn_1419630a0 as *const ()), // sub_1419630a0
    Replacement::draft(0x141963170, "unknown_141963170", fn_141963170 as *const ()), // sub_141963170
    Replacement::draft(0x1419631d0, "unknown_1419631d0", fn_1419631d0 as *const ()), // sub_1419631d0
    Replacement::draft(0x1419635c0, "unknown_1419635c0", fn_1419635c0 as *const ()), // sub_1419635c0
    Replacement::draft(0x141963940, "unknown_141963940", fn_141963940 as *const ()), // sub_141963940
    Replacement::draft(0x1419639f0, "unknown_1419639f0", fn_1419639f0 as *const ()), // sub_1419639f0
    Replacement::draft(0x141963be0, "unknown_141963be0", fn_141963be0 as *const ()), // sub_141963be0
    Replacement::draft(0x141963ec0, "unknown_141963ec0", tpl_35748::<0x14041f8b0> as *const ()), // sub_141963ec0
    Replacement::draft(0x141963ee0, "unknown_141963ee0", fn_141963ee0 as *const ()), // sub_141963ee0
    Replacement::draft(0x141963fc0, "unknown_141963fc0", tpl_95219::<0x14311f060, 0x8, 0x10> as *const ()), // sub_141963fc0
    Replacement::draft(0x141964070, "unknown_141964070", tpl_35484::<0x14311f060, 0x8, 0x10> as *const ()), // sub_141964070
    Replacement::draft(0x141964150, "unknown_141964150", tpl_34393 as *const ()), // sub_141964150
    Replacement::draft(0x1419641e0, "unknown_1419641e0", fn_1419641e0 as *const ()), // sub_1419641e0
    Replacement::draft(0x1419642f0, "unknown_1419642f0", fn_1419642f0 as *const ()), // sub_1419642f0
    Replacement::draft(0x141964360, "unknown_141964360", tpl_34679::<0x18, 0x30> as *const ()), // sub_141964360
    Replacement::draft(0x141964400, "unknown_141964400", tpl_34605::<0x143106928> as *const ()), // sub_141964400
    Replacement::draft(0x141964540, "unknown_141964540", tpl_35751::<0x20, 0x40, 0x60> as *const ()), // sub_141964540
    Replacement::draft(0x1419645f0, "unknown_1419645f0", tpl_34612 as *const ()), // sub_1419645f0
    Replacement::draft(0x141964680, "unknown_141964680", tpl_34393 as *const ()), // sub_141964680
    Replacement::draft(0x141964710, "unknown_141964710", tpl_34395::<0x10, 0x18> as *const ()), // sub_141964710
    Replacement::draft(0x1419647b0, "unknown_1419647b0", tpl_35751::<0x20, 0x40, 0x60> as *const ()), // sub_1419647b0
    Replacement::draft(0x141964860, "unknown_141964860", fn_141964860 as *const ()), // sub_141964860
    Replacement::draft(0x141964880, "unknown_141964880", tpl_34393 as *const ()), // sub_141964880
    Replacement::draft(0x141964990, "unknown_141964990", fn_141964990 as *const ()), // sub_141964990
    Replacement::draft(0x141964b60, "unknown_141964b60", tpl_35751::<0x20, 0x40, 0x60> as *const ()), // sub_141964b60
    Replacement::draft(0x141964d20, "unknown_141964d20", fn_141964d20 as *const ()), // sub_141964d20
    Replacement::draft(0x141964de0, "unknown_141964de0", fn_141964de0 as *const ()), // sub_141964de0
    Replacement::draft(0x141964ee0, "unknown_141964ee0", fn_141964ee0 as *const ()), // sub_141964ee0
    Replacement::draft(0x141965060, "unknown_141965060", fn_141965060 as *const ()), // sub_141965060
    Replacement::draft(0x1419651b0, "unknown_1419651b0", fn_1419651b0 as *const ()), // sub_1419651b0
    Replacement::draft(0x141965260, "unknown_141965260", tpl_35758::<0x8, 0x14041ede0> as *const ()), // sub_141965260
    Replacement::draft(0x141965310, "unknown_141965310", tpl_35759::<0x8, 0x14041ede0> as *const ()), // sub_141965310
    Replacement::draft(0x141965340, "unknown_141965340", tpl_34684::<0x14041ed30> as *const ()), // sub_141965340
    Replacement::draft(0x141965480, "unknown_141965480", fn_141965480 as *const ()), // sub_141965480
    Replacement::draft(0x1419655a0, "unknown_1419655a0", fn_1419655a0 as *const ()), // sub_1419655a0
    Replacement::draft(0x1419657f0, "unknown_1419657f0", tpl_34394::<0x10> as *const ()), // sub_1419657f0
    Replacement::draft(0x141965890, "unknown_141965890", fn_141965890 as *const ()), // sub_141965890
    Replacement::draft(0x141965a80, "unknown_141965a80", fn_141965a80 as *const ()), // sub_141965a80
    Replacement::draft(0x141965b40, "unknown_141965b40", fn_141965b40 as *const ()), // sub_141965b40
    Replacement::draft(0x141965bf0, "unknown_141965bf0", fn_141965bf0 as *const ()), // sub_141965bf0
    Replacement::draft(0x141965d70, "unknown_141965d70", fn_141965d70 as *const ()), // sub_141965d70
    Replacement::draft(0x141965e50, "unknown_141965e50", fn_141965e50 as *const ()), // sub_141965e50
    Replacement::draft(0x141965eb0, "unknown_141965eb0", tpl_35060 as *const ()), // sub_141965eb0
    Replacement::draft(0x141965ef0, "unknown_141965ef0", tpl_34994::<0x143c9f0cc, 0x14387b040, 0x14387b038, 0x141834a78, 0x1425e69c0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141965ef0
    Replacement::draft(0x141965f60, "unknown_141965f60", tpl_35252 as *const ()), // sub_141965f60
    Replacement::draft(0x141965f80, "unknown_141965f80", tpl_34994::<0x143c9ced4, 0x14387aee0, 0x14387aed8, 0x141834a78, 0x1425e69d0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141965f80
    Replacement::draft(0x141965ff0, "unknown_141965ff0", tpl_35332 as *const ()), // sub_141965ff0
    Replacement::draft(0x141966000, "unknown_141966000", tpl_34994::<0x143c9d1f4, 0x14387b0c0, 0x14387b0b8, 0x141834a78, 0x1425e69e0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141966000
    Replacement::draft(0x141966070, "unknown_141966070", tpl_35252 as *const ()), // sub_141966070
    Replacement::draft(0x141966090, "unknown_141966090", tpl_34318 as *const ()), // sub_141966090
    Replacement::draft(0x1419660b0, "unknown_1419660b0", tpl_34319::<0x143c8fa04, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419660b0
    Replacement::draft(0x141966110, "unknown_141966110", tpl_34318 as *const ()), // sub_141966110
    Replacement::draft(0x141966130, "unknown_141966130", tpl_34319::<0x143c8f6e4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966130
    Replacement::draft(0x141966190, "unknown_141966190", tpl_34318 as *const ()), // sub_141966190
    Replacement::draft(0x1419661b0, "unknown_1419661b0", tpl_34319::<0x143c9dba4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419661b0
    Replacement::draft(0x141966210, "unknown_141966210", tpl_34318 as *const ()), // sub_141966210
    Replacement::draft(0x141966230, "unknown_141966230", tpl_34318 as *const ()), // sub_141966230
    Replacement::draft(0x141966250, "unknown_141966250", tpl_34319::<0x143c9f300, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966250
    Replacement::draft(0x1419662b0, "unknown_1419662b0", tpl_34319::<0x143c97e70, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419662b0
    Replacement::draft(0x141966310, "unknown_141966310", tpl_34318 as *const ()), // sub_141966310
    Replacement::draft(0x141966330, "unknown_141966330", tpl_34319::<0x143c9779c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966330
    Replacement::draft(0x141966390, "unknown_141966390", tpl_34318 as *const ()), // sub_141966390
    Replacement::draft(0x1419663b0, "unknown_1419663b0", tpl_34318 as *const ()), // sub_1419663b0
    Replacement::draft(0x1419663d0, "unknown_1419663d0", tpl_34319::<0x143c95048, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419663d0
    Replacement::draft(0x141966430, "unknown_141966430", tpl_34319::<0x143c8ea68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966430
    Replacement::draft(0x141966490, "unknown_141966490", tpl_34318 as *const ()), // sub_141966490
    Replacement::draft(0x1419664b0, "unknown_1419664b0", tpl_34319::<0x143c9e55c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419664b0
    Replacement::draft(0x141966510, "unknown_141966510", tpl_361 as *const ()), // sub_141966510
    Replacement::draft(0x141966520, "unknown_141966520", tpl_34318 as *const ()), // sub_141966520
    Replacement::draft(0x141966540, "unknown_141966540", tpl_34318 as *const ()), // sub_141966540
    Replacement::draft(0x141966560, "unknown_141966560", tpl_35063::<0x143c9e5cc, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141966560
    Replacement::draft(0x1419665b0, "unknown_1419665b0", tpl_34319::<0x143c933f0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419665b0
    Replacement::draft(0x141966610, "unknown_141966610", tpl_34319::<0x143c9d9d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966610
    Replacement::draft(0x141966670, "unknown_141966670", tpl_34318 as *const ()), // sub_141966670
    Replacement::draft(0x141966690, "unknown_141966690", tpl_34319::<0x143c93228, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966690
    Replacement::draft(0x1419666f0, "unknown_1419666f0", tpl_34318 as *const ()), // sub_1419666f0
    Replacement::draft(0x141966710, "unknown_141966710", tpl_361 as *const ()), // sub_141966710
    Replacement::draft(0x141966720, "unknown_141966720", tpl_34319::<0x143c9d614, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966720
    Replacement::draft(0x141966780, "unknown_141966780", tpl_35063::<0x143c9f008, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141966780
    Replacement::draft(0x1419667d0, "unknown_1419667d0", tpl_34318 as *const ()), // sub_1419667d0
    Replacement::draft(0x1419667f0, "unknown_1419667f0", tpl_34318 as *const ()), // sub_1419667f0
    Replacement::draft(0x141966810, "unknown_141966810", tpl_34319::<0x143c9d840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966810
    Replacement::draft(0x141966870, "unknown_141966870", tpl_34319::<0x143c9d9fc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966870
    Replacement::draft(0x1419668d0, "unknown_1419668d0", tpl_34318 as *const ()), // sub_1419668d0
    Replacement::draft(0x1419668f0, "unknown_1419668f0", tpl_34319::<0x143c9d19c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419668f0
    Replacement::draft(0x141966950, "unknown_141966950", tpl_34318 as *const ()), // sub_141966950
    Replacement::draft(0x141966970, "unknown_141966970", tpl_34319::<0x143c92c98, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966970
    Replacement::draft(0x1419669d0, "unknown_1419669d0", tpl_34318 as *const ()), // sub_1419669d0
    Replacement::draft(0x1419669f0, "unknown_1419669f0", tpl_34318 as *const ()), // sub_1419669f0
    Replacement::draft(0x141966a10, "unknown_141966a10", tpl_34318 as *const ()), // sub_141966a10
    Replacement::draft(0x141966a30, "unknown_141966a30", tpl_34319::<0x143c92ee8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966a30
    Replacement::draft(0x141966a90, "unknown_141966a90", tpl_34319::<0x143c937c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966a90
    Replacement::draft(0x141966af0, "unknown_141966af0", tpl_34319::<0x143c92a7c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966af0
    Replacement::draft(0x141966b50, "unknown_141966b50", tpl_34318 as *const ()), // sub_141966b50
    Replacement::draft(0x141966b70, "unknown_141966b70", tpl_34319::<0x143c9d6b8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966b70
    Replacement::draft(0x141966bd0, "unknown_141966bd0", tpl_34318 as *const ()), // sub_141966bd0
    Replacement::draft(0x141966bf0, "unknown_141966bf0", tpl_34318 as *const ()), // sub_141966bf0
    Replacement::draft(0x141966c10, "unknown_141966c10", tpl_34319::<0x143c964c0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966c10
    Replacement::draft(0x141966c70, "unknown_141966c70", tpl_34319::<0x143c9ea00, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966c70
    Replacement::draft(0x141966cd0, "unknown_141966cd0", tpl_34318 as *const ()), // sub_141966cd0
    Replacement::draft(0x141966cf0, "unknown_141966cf0", tpl_34318 as *const ()), // sub_141966cf0
    Replacement::draft(0x141966d10, "unknown_141966d10", tpl_34318 as *const ()), // sub_141966d10
    Replacement::draft(0x141966d30, "unknown_141966d30", tpl_34318 as *const ()), // sub_141966d30
    Replacement::draft(0x141966d50, "unknown_141966d50", tpl_34318 as *const ()), // sub_141966d50
    Replacement::draft(0x141966d70, "unknown_141966d70", tpl_34319::<0x143c977b4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966d70
    Replacement::draft(0x141966dd0, "unknown_141966dd0", tpl_34319::<0x143c90198, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966dd0
    Replacement::draft(0x141966e30, "unknown_141966e30", tpl_34319::<0x143c8e9b0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966e30
    Replacement::draft(0x141966e90, "unknown_141966e90", tpl_34319::<0x143c9deb0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966e90
    Replacement::draft(0x141966ef0, "unknown_141966ef0", tpl_34319::<0x143c970e0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966ef0
    Replacement::draft(0x141966f50, "unknown_141966f50", tpl_34318 as *const ()), // sub_141966f50
    Replacement::draft(0x141966f70, "unknown_141966f70", tpl_34319::<0x143c933b0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966f70
    Replacement::draft(0x141966fd0, "unknown_141966fd0", tpl_34318 as *const ()), // sub_141966fd0
    Replacement::draft(0x141966ff0, "unknown_141966ff0", tpl_34319::<0x143c93310, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141966ff0
    Replacement::draft(0x141967050, "unknown_141967050", tpl_34318 as *const ()), // sub_141967050
    Replacement::draft(0x141967070, "unknown_141967070", tpl_34319::<0x143c8e670, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967070
    Replacement::draft(0x1419670d0, "unknown_1419670d0", tpl_34318 as *const ()), // sub_1419670d0
    Replacement::draft(0x1419670f0, "unknown_1419670f0", tpl_34318 as *const ()), // sub_1419670f0
    Replacement::draft(0x141967110, "unknown_141967110", tpl_361 as *const ()), // sub_141967110
    Replacement::draft(0x141967120, "unknown_141967120", tpl_34319::<0x143c8ec28, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967120
    Replacement::draft(0x141967180, "unknown_141967180", tpl_34319::<0x143c8dbd0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967180
    Replacement::draft(0x1419671e0, "unknown_1419671e0", tpl_35063::<0x143c8d694, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_1419671e0
    Replacement::draft(0x141967230, "unknown_141967230", tpl_34318 as *const ()), // sub_141967230
    Replacement::draft(0x141967250, "unknown_141967250", tpl_34318 as *const ()), // sub_141967250
    Replacement::draft(0x141967270, "unknown_141967270", tpl_34319::<0x143c9a320, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967270
    Replacement::draft(0x1419672d0, "unknown_1419672d0", tpl_34319::<0x143c9a330, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419672d0
    Replacement::draft(0x141967330, "unknown_141967330", tpl_34318 as *const ()), // sub_141967330
    Replacement::draft(0x141967350, "unknown_141967350", tpl_34318 as *const ()), // sub_141967350
    Replacement::draft(0x141967370, "unknown_141967370", tpl_34319::<0x143c8f6dc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967370
    Replacement::draft(0x1419673d0, "unknown_1419673d0", tpl_34319::<0x143c8f7a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419673d0
    Replacement::draft(0x141967430, "unknown_141967430", tpl_34318 as *const ()), // sub_141967430
    Replacement::draft(0x141967450, "unknown_141967450", tpl_34319::<0x143c8ec6c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967450
    Replacement::draft(0x1419674b0, "unknown_1419674b0", tpl_34318 as *const ()), // sub_1419674b0
    Replacement::draft(0x1419674d0, "unknown_1419674d0", tpl_34319::<0x143c8d518, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419674d0
    Replacement::draft(0x141967530, "unknown_141967530", tpl_34318 as *const ()), // sub_141967530
    Replacement::draft(0x141967550, "unknown_141967550", tpl_34318 as *const ()), // sub_141967550
    Replacement::draft(0x141967570, "unknown_141967570", tpl_34318 as *const ()), // sub_141967570
    Replacement::draft(0x141967590, "unknown_141967590", tpl_34319::<0x143c8d5b0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967590
    Replacement::draft(0x1419675f0, "unknown_1419675f0", tpl_34319::<0x143c8d708, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419675f0
    Replacement::draft(0x141967650, "unknown_141967650", tpl_34319::<0x143c8d68c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967650
    Replacement::draft(0x1419676b0, "std_1419676b0", tpl_579::<0x14311e930, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x1419676d0, "std_1419676d0", tpl_579::<0x14311e930, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x1419676f0, "std_1419676f0", fn_1419676f0 as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967700, "std_141967700", tpl_1::<0x143887c20> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967710, "std_141967710", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967720, "std_141967720", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967740, "std_141967740", tpl_770::<0x1431068a0> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_1>,bool,m::SIMDVector,ecs::AliveEntityHandle>::std
    Replacement::draft(0x141967760, "std_141967760", tpl_770::<0x1431068a0> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_1>,bool,m::SIMDVector,ecs::AliveEntityHandle>::std
    Replacement::draft(0x141967790, "std_141967790", tpl_1::<0x143887d30> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_1>,bool,m::SIMDVector,ecs::AliveEntityHandle>::std
    Replacement::draft(0x1419677a0, "std_1419677a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_1>,bool,m::SIMDVector,ecs::AliveEntityHandle>::std
    Replacement::draft(0x1419677b0, "std_1419677b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_1>,bool,m::SIMDVector,ecs::AliveEntityHandle>::std
    Replacement::draft(0x1419677d0, "std_1419677d0", tpl_35765::<0x14311e750, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x1419677f0, "std_1419677f0", tpl_35765::<0x14311e750, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967810, "std_141967810", tpl_586::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967820, "std_141967820", tpl_1::<0x143887d90> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967830, "std_141967830", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967840, "std_141967840", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish_inflict_direct_chunk_damage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bigfish_
    Replacement::draft(0x141967860, "unknown_141967860", tpl_34318 as *const ()), // sub_141967860
    Replacement::draft(0x141967880, "unknown_141967880", tpl_34318 as *const ()), // sub_141967880
    Replacement::draft(0x1419678a0, "unknown_1419678a0", tpl_34319::<0x143c8ec68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419678a0
    Replacement::draft(0x141967900, "unknown_141967900", tpl_34319::<0x143c96b20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967900
    Replacement::draft(0x141967960, "unknown_141967960", tpl_34318 as *const ()), // sub_141967960
    Replacement::draft(0x141967980, "unknown_141967980", tpl_34318 as *const ()), // sub_141967980
    Replacement::draft(0x1419679a0, "unknown_1419679a0", tpl_34318 as *const ()), // sub_1419679a0
    Replacement::draft(0x1419679c0, "unknown_1419679c0", tpl_34318 as *const ()), // sub_1419679c0
    Replacement::draft(0x1419679e0, "unknown_1419679e0", tpl_34318 as *const ()), // sub_1419679e0
    Replacement::draft(0x141967a00, "unknown_141967a00", tpl_34318 as *const ()), // sub_141967a00
    Replacement::draft(0x141967a20, "unknown_141967a20", tpl_34319::<0x143c97ad0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967a20
    Replacement::draft(0x141967a80, "unknown_141967a80", tpl_34319::<0x143c8dbc4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967a80
    Replacement::draft(0x141967ae0, "unknown_141967ae0", tpl_34319::<0x143c8de68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967ae0
    Replacement::draft(0x141967b40, "unknown_141967b40", tpl_34319::<0x143c8e840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967b40
    Replacement::draft(0x141967ba0, "unknown_141967ba0", tpl_34319::<0x143c919a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967ba0
    Replacement::draft(0x141967c00, "unknown_141967c00", tpl_34319::<0x143c97150, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967c00
    Replacement::draft(0x141967c60, "unknown_141967c60", tpl_34318 as *const ()), // sub_141967c60
    Replacement::draft(0x141967c80, "unknown_141967c80", tpl_34319::<0x143c8efb0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967c80
    Replacement::draft(0x141967ce0, "unknown_141967ce0", tpl_34318 as *const ()), // sub_141967ce0
    Replacement::draft(0x141967d00, "unknown_141967d00", tpl_34319::<0x143c8eb90, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967d00
    Replacement::draft(0x141967d60, "unknown_141967d60", tpl_34318 as *const ()), // sub_141967d60
    Replacement::draft(0x141967d80, "unknown_141967d80", tpl_34318 as *const ()), // sub_141967d80
    Replacement::draft(0x141967da0, "unknown_141967da0", tpl_34318 as *const ()), // sub_141967da0
    Replacement::draft(0x141967dc0, "unknown_141967dc0", tpl_34319::<0x143c9ecdc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967dc0
    Replacement::draft(0x141967e20, "unknown_141967e20", tpl_34319::<0x143c9db0c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967e20
    Replacement::draft(0x141967e80, "unknown_141967e80", tpl_34319::<0x143c93b20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967e80
    Replacement::draft(0x141967ee0, "unknown_141967ee0", tpl_34318 as *const ()), // sub_141967ee0
    Replacement::draft(0x141967f00, "unknown_141967f00", tpl_34319::<0x143c8d478, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967f00
    Replacement::draft(0x141967f60, "unknown_141967f60", tpl_34318 as *const ()), // sub_141967f60
    Replacement::draft(0x141967f80, "unknown_141967f80", tpl_34319::<0x143c8cf5c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141967f80
    Replacement::draft(0x141967fe0, "unknown_141967fe0", tpl_34318 as *const ()), // sub_141967fe0
    Replacement::draft(0x141968000, "unknown_141968000", tpl_34319::<0x143c91608, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968000
    Replacement::draft(0x141968060, "unknown_141968060", tpl_34318 as *const ()), // sub_141968060
    Replacement::draft(0x141968080, "unknown_141968080", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968080
    Replacement::draft(0x1419680e0, "unknown_1419680e0", tpl_34318 as *const ()), // sub_1419680e0
    Replacement::draft(0x141968100, "unknown_141968100", tpl_34319::<0x143c95468, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968100
    Replacement::draft(0x141968160, "unknown_141968160", tpl_34318 as *const ()), // sub_141968160
    Replacement::draft(0x141968180, "unknown_141968180", tpl_34319::<0x143c9de20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968180
    Replacement::draft(0x1419681e0, "unknown_1419681e0", tpl_34318 as *const ()), // sub_1419681e0
    Replacement::draft(0x141968200, "unknown_141968200", tpl_34318 as *const ()), // sub_141968200
    Replacement::draft(0x141968220, "unknown_141968220", tpl_34318 as *const ()), // sub_141968220
    Replacement::draft(0x141968240, "unknown_141968240", tpl_34318 as *const ()), // sub_141968240
    Replacement::draft(0x141968260, "unknown_141968260", tpl_34318 as *const ()), // sub_141968260
    Replacement::draft(0x141968280, "unknown_141968280", tpl_34319::<0x143c8f8d4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968280
    Replacement::draft(0x1419682e0, "unknown_1419682e0", tpl_34319::<0x143c9b4c4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419682e0
    Replacement::draft(0x141968340, "unknown_141968340", tpl_34319::<0x143c9eac0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968340
    Replacement::draft(0x1419683a0, "unknown_1419683a0", tpl_34319::<0x143c8fab0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419683a0
    Replacement::draft(0x141968400, "unknown_141968400", tpl_34319::<0x143c9b508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968400
    Replacement::draft(0x141968460, "unknown_141968460", tpl_34318 as *const ()), // sub_141968460
    Replacement::draft(0x141968480, "unknown_141968480", tpl_34319::<0x143c8fabc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968480
    Replacement::draft(0x1419684e0, "unknown_1419684e0", tpl_34318 as *const ()), // sub_1419684e0
    Replacement::draft(0x141968500, "unknown_141968500", tpl_34318 as *const ()), // sub_141968500
    Replacement::draft(0x141968520, "unknown_141968520", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968520
    Replacement::draft(0x141968580, "unknown_141968580", tpl_34319::<0x143c8f0d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968580
    Replacement::draft(0x1419685e0, "unknown_1419685e0", tpl_34318 as *const ()), // sub_1419685e0
    Replacement::draft(0x141968600, "unknown_141968600", tpl_34318 as *const ()), // sub_141968600
    Replacement::draft(0x141968620, "unknown_141968620", tpl_34318 as *const ()), // sub_141968620
    Replacement::draft(0x141968640, "unknown_141968640", tpl_34319::<0x143c9afe8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968640
    Replacement::draft(0x1419686a0, "unknown_1419686a0", tpl_34319::<0x143c9d1d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419686a0
    Replacement::draft(0x141968700, "unknown_141968700", tpl_34319::<0x143c9ed30, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968700
    Replacement::draft(0x141968760, "unknown_141968760", tpl_34318 as *const ()), // sub_141968760
    Replacement::draft(0x141968780, "unknown_141968780", tpl_34319::<0x143c95bf0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968780
    Replacement::draft(0x1419687e0, "unknown_1419687e0", tpl_34318 as *const ()), // sub_1419687e0
    Replacement::draft(0x141968800, "unknown_141968800", tpl_34319::<0x143c8de7c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968800
    Replacement::draft(0x141968860, "unknown_141968860", tpl_34318 as *const ()), // sub_141968860
    Replacement::draft(0x141968880, "unknown_141968880", tpl_34319::<0x143c8eba0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968880
    Replacement::draft(0x1419688e0, "unknown_1419688e0", tpl_34318 as *const ()), // sub_1419688e0
    Replacement::draft(0x141968900, "unknown_141968900", tpl_34319::<0x143c8eb98, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968900
    Replacement::draft(0x141968960, "unknown_141968960", tpl_34318 as *const ()), // sub_141968960
    Replacement::draft(0x141968980, "unknown_141968980", tpl_34319::<0x143c8e664, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968980
    Replacement::draft(0x1419689e0, "unknown_1419689e0", tpl_34318 as *const ()), // sub_1419689e0
    Replacement::draft(0x141968a00, "unknown_141968a00", tpl_34318 as *const ()), // sub_141968a00
    Replacement::draft(0x141968a20, "unknown_141968a20", tpl_34319::<0x143c8ec24, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968a20
    Replacement::draft(0x141968a80, "unknown_141968a80", tpl_34319::<0x143c8dbcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968a80
    Replacement::draft(0x141968ae0, "unknown_141968ae0", tpl_34318 as *const ()), // sub_141968ae0
    Replacement::draft(0x141968b00, "unknown_141968b00", tpl_34319::<0x143c8f690, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968b00
    Replacement::draft(0x141968b60, "unknown_141968b60", tpl_34318 as *const ()), // sub_141968b60
    Replacement::draft(0x141968b80, "unknown_141968b80", tpl_34319::<0x143c956a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968b80
    Replacement::draft(0x141968be0, "unknown_141968be0", tpl_34318 as *const ()), // sub_141968be0
    Replacement::draft(0x141968c00, "unknown_141968c00", tpl_34318 as *const ()), // sub_141968c00
    Replacement::draft(0x141968c20, "unknown_141968c20", tpl_34319::<0x143c9665c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968c20
    Replacement::draft(0x141968c80, "unknown_141968c80", tpl_34319::<0x143c8e710, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968c80
    Replacement::draft(0x141968ce0, "unknown_141968ce0", tpl_34318 as *const ()), // sub_141968ce0
    Replacement::draft(0x141968d00, "unknown_141968d00", tpl_34318 as *const ()), // sub_141968d00
    Replacement::draft(0x141968d20, "unknown_141968d20", tpl_34319::<0x143c98790, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968d20
    Replacement::draft(0x141968d80, "unknown_141968d80", tpl_34319::<0x143c91064, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968d80
    Replacement::draft(0x141968de0, "unknown_141968de0", tpl_34318 as *const ()), // sub_141968de0
    Replacement::draft(0x141968e00, "unknown_141968e00", tpl_34318 as *const ()), // sub_141968e00
    Replacement::draft(0x141968e20, "unknown_141968e20", tpl_34319::<0x143c9efb0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968e20
    Replacement::draft(0x141968e80, "unknown_141968e80", tpl_34319::<0x143c9d1b4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968e80
    Replacement::draft(0x141968ee0, "unknown_141968ee0", tpl_34318 as *const ()), // sub_141968ee0
    Replacement::draft(0x141968f00, "unknown_141968f00", tpl_34318 as *const ()), // sub_141968f00
    Replacement::draft(0x141968f20, "unknown_141968f20", tpl_34318 as *const ()), // sub_141968f20
    Replacement::draft(0x141968f40, "unknown_141968f40", tpl_34318 as *const ()), // sub_141968f40
    Replacement::draft(0x141968f60, "unknown_141968f60", tpl_34319::<0x143c8e70c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968f60
    Replacement::draft(0x141968fc0, "unknown_141968fc0", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141968fc0
    Replacement::draft(0x141969020, "unknown_141969020", tpl_34319::<0x143c8fab8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141969020
    Replacement::draft(0x141969080, "unknown_141969080", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141969080
    Replacement::draft(0x1419690e0, "unknown_1419690e0", tpl_34318 as *const ()), // sub_1419690e0
    Replacement::draft(0x141969100, "unknown_141969100", tpl_34319::<0x143c96de8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141969100
    Replacement::draft(0x141969160, "unknown_141969160", tpl_34318 as *const ()), // sub_141969160
    Replacement::draft(0x141969180, "unknown_141969180", tpl_34319::<0x143c9dc58, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141969180
    Replacement::draft(0x1419691e0, "unknown_1419691e0", tpl_34318 as *const ()), // sub_1419691e0
    Replacement::draft(0x141969200, "unknown_141969200", tpl_34319::<0x143c9eeec, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141969200
    Replacement::draft(0x141969260, "unknown_141969260", tpl_34318 as *const ()), // sub_141969260
    Replacement::draft(0x141969280, "unknown_141969280", tpl_34319::<0x143c96b14, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141969280
    Replacement::draft(0x1419692e0, "unknown_1419692e0", tpl_34318 as *const ()), // sub_1419692e0
    Replacement::draft(0x141969300, "unknown_141969300", tpl_34319::<0x143c9d1ac, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141969300
    Replacement::draft(0x141969360, "unknown_141969360", tpl_34318 as *const ()), // sub_141969360
    Replacement::draft(0x141969380, "unknown_141969380", tpl_34318 as *const ()), // sub_141969380
    Replacement::draft(0x1419693a0, "unknown_1419693a0", tpl_34319::<0x143c90164, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419693a0
    Replacement::draft(0x141969400, "unknown_141969400", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141969400
    Replacement::draft_throws(0x141969460, "unknown_141969460", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140408c30, 0x10, 0x14310d738, 0x142116bc0, 0x1421158a0, 0x1419694ea, 0x1419694fc> as *const (), Throws::Discard), // sub_141969460
    Replacement::draft_throws(0x141969510, "unknown_141969510", tpl_35767::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140408a00, 0x20, 0x14310d790, 0x142116bc0, 0x1421158a0, 0x1419695a0, 0x1419695b2> as *const (), Throws::Discard), // sub_141969510
    Replacement::draft_throws(0x1419695c0, "unknown_1419695c0", tpl_35768::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404085d0, 0x20, 0x14310dc30, 0x142116bc0, 0x1421158a0, 0x14196964d, 0x14196965f> as *const (), Throws::Discard), // sub_1419695c0
    Replacement::draft_throws(0x141969670, "unknown_141969670", tpl_35769::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140407400, 0x143109880, 0x142116bc0, 0x1421158a0, 0x1419696f2, 0x141969704> as *const (), Throws::Discard), // sub_141969670
    Replacement::draft_throws(0x141969710, "unknown_141969710", tpl_35770::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140406b60, 0x20, 0x30, 0x14310a658, 0x142116bc0, 0x1421158a0, 0x1419697a8, 0x1419697ba> as *const (), Throws::Discard), // sub_141969710
    Replacement::draft_throws(0x1419697d0, "unknown_1419697d0", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140406950, 0x10, 0x14310aaf8, 0x142116bc0, 0x1421158a0, 0x14196985a, 0x14196986c> as *const (), Throws::Discard), // sub_1419697d0
    Replacement::draft_throws(0x141969880, "unknown_141969880", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140406740, 0x10, 0x14310ab20, 0x142116bc0, 0x1421158a0, 0x14196990a, 0x14196991c> as *const (), Throws::Discard), // sub_141969880
    Replacement::draft_throws(0x141969930, "unknown_141969930", tpl_35769::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140406270, 0x14310ab40, 0x142116bc0, 0x1421158a0, 0x1419699b2, 0x1419699c4> as *const (), Throws::Discard), // sub_141969930
    Replacement::draft_throws(0x1419699d0, "unknown_1419699d0", tpl_35768::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404060d0, 0x20, 0x14310ab70, 0x142116bc0, 0x1421158a0, 0x141969a5d, 0x141969a6f> as *const (), Throws::Discard), // sub_1419699d0
    Replacement::draft_throws(0x141969a80, "unknown_141969a80", fn_141969a80 as *const (), Throws::Discard), // sub_141969a80
    Replacement::draft_throws(0x141969b20, "unknown_141969b20", fn_141969b20 as *const (), Throws::Discard), // sub_141969b20
    Replacement::draft_throws(0x141969bd0, "unknown_141969bd0", tpl_35768::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140405500, 0x20, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x141969c5d, 0x141969c6f> as *const (), Throws::Discard), // sub_141969bd0
    Replacement::draft_throws(0x141969c80, "unknown_141969c80", tpl_35769::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140405290, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x141969d02, 0x141969d14> as *const (), Throws::Discard), // sub_141969c80
    Replacement::draft_throws(0x141969d20, "unknown_141969d20", fn_141969d20 as *const (), Throws::Discard), // sub_141969d20
    Replacement::draft_throws(0x141969de0, "unknown_141969de0", fn_141969de0 as *const (), Throws::Discard), // sub_141969de0
    Replacement::draft_throws(0x141969eb0, "unknown_141969eb0", fn_141969eb0 as *const (), Throws::Discard), // sub_141969eb0
    Replacement::draft_throws(0x14196a080, "unknown_14196a080", fn_14196a080 as *const (), Throws::Discard), // sub_14196a080
    Replacement::draft_throws(0x14196a160, "unknown_14196a160", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140404a20, 0x10, 0x14310a970, 0x142116bc0, 0x1421158a0, 0x14196a1ea, 0x14196a1fc> as *const (), Throws::Discard), // sub_14196a160
    Replacement::draft_throws(0x14196a210, "unknown_14196a210", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140404810, 0x10, 0x14310a970, 0x142116bc0, 0x1421158a0, 0x14196a29a, 0x14196a2ac> as *const (), Throws::Discard), // sub_14196a210
    Replacement::draft_throws(0x14196a2c0, "unknown_14196a2c0", tpl_35769::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404045a0, 0x14310a970, 0x142116bc0, 0x1421158a0, 0x14196a342, 0x14196a354> as *const (), Throws::Discard), // sub_14196a2c0
    Replacement::draft_throws(0x14196a360, "unknown_14196a360", fn_14196a360 as *const (), Throws::Discard), // sub_14196a360
    Replacement::draft_throws(0x14196a520, "unknown_14196a520", fn_14196a520 as *const (), Throws::Discard), // sub_14196a520
    Replacement::draft_throws(0x14196a5e0, "unknown_14196a5e0", tpl_35768::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140404240, 0x20, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196a66d, 0x14196a67f> as *const (), Throws::Discard), // sub_14196a5e0
    Replacement::draft_throws(0x14196a690, "unknown_14196a690", tpl_35769::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140403fd0, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196a712, 0x14196a724> as *const (), Throws::Discard), // sub_14196a690
    Replacement::draft_throws(0x14196a730, "unknown_14196a730", tpl_35777::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140403e20, 0x20, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196a7bd, 0x14196a7cf> as *const (), Throws::Discard), // sub_14196a730
    Replacement::draft_throws(0x14196a7e0, "unknown_14196a7e0", tpl_35769::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140403bb0, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196a862, 0x14196a874> as *const (), Throws::Discard), // sub_14196a7e0
    Replacement::draft_throws(0x14196a880, "unknown_14196a880", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404039a0, 0x10, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196a90a, 0x14196a91c> as *const (), Throws::Discard), // sub_14196a880
    Replacement::draft_throws(0x14196a930, "unknown_14196a930", fn_14196a930 as *const (), Throws::Discard), // sub_14196a930
    Replacement::draft_throws(0x14196a9e0, "unknown_14196a9e0", tpl_35769::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140403520, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196aa62, 0x14196aa74> as *const (), Throws::Discard), // sub_14196a9e0
    Replacement::draft_throws(0x14196aa80, "unknown_14196aa80", tpl_35770::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404032f0, 0x20, 0x30, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196ab18, 0x14196ab2a> as *const (), Throws::Discard), // sub_14196aa80
    Replacement::draft_throws(0x14196ab40, "unknown_14196ab40", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404030e0, 0x10, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196abca, 0x14196abdc> as *const (), Throws::Discard), // sub_14196ab40
    Replacement::draft_throws(0x14196abf0, "unknown_14196abf0", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140402ed0, 0x10, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196ac7a, 0x14196ac8c> as *const (), Throws::Discard), // sub_14196abf0
    Replacement::draft_throws(0x14196aca0, "unknown_14196aca0", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140402cc0, 0x10, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196ad2a, 0x14196ad3c> as *const (), Throws::Discard), // sub_14196aca0
    Replacement::draft_throws(0x14196ad50, "unknown_14196ad50", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140402ab0, 0x10, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196adda, 0x14196adec> as *const (), Throws::Discard), // sub_14196ad50
    Replacement::draft_throws(0x14196ae00, "unknown_14196ae00", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404028a0, 0x10, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196ae8a, 0x14196ae9c> as *const (), Throws::Discard), // sub_14196ae00
    Replacement::draft_throws(0x14196aeb0, "unknown_14196aeb0", tpl_35768::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140402700, 0x20, 0x14310a9a8, 0x142116bc0, 0x1421158a0, 0x14196af3d, 0x14196af4f> as *const (), Throws::Discard), // sub_14196aeb0
    Replacement::draft_throws(0x14196af60, "unknown_14196af60", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140402460, 0x10, 0x14310a058, 0x142116bc0, 0x1421158a0, 0x14196afea, 0x14196affc> as *const (), Throws::Discard), // sub_14196af60
    Replacement::draft_throws(0x14196b010, "unknown_14196b010", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404021f0, 0x10, 0x14310a0c8, 0x142116bc0, 0x1421158a0, 0x14196b09a, 0x14196b0ac> as *const (), Throws::Discard), // sub_14196b010
    Replacement::draft_throws(0x14196b0c0, "unknown_14196b0c0", fn_14196b0c0 as *const (), Throws::Discard), // sub_14196b0c0
    Replacement::draft_throws(0x14196b290, "unknown_14196b290", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140402030, 0x10, 0x143109e58, 0x142116bc0, 0x1421158a0, 0x14196b31a, 0x14196b32c> as *const (), Throws::Discard), // sub_14196b290
    Replacement::draft_throws(0x14196b340, "unknown_14196b340", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140401e20, 0x10, 0x143109e58, 0x142116bc0, 0x1421158a0, 0x14196b3ca, 0x14196b3dc> as *const (), Throws::Discard), // sub_14196b340
    Replacement::draft_throws(0x14196b3f0, "unknown_14196b3f0", tpl_35768::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140401c80, 0x20, 0x143109e58, 0x142116bc0, 0x1421158a0, 0x14196b47d, 0x14196b48f> as *const (), Throws::Discard), // sub_14196b3f0
    Replacement::draft_throws(0x14196b4a0, "unknown_14196b4a0", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140401a70, 0x10, 0x143109e80, 0x142116bc0, 0x1421158a0, 0x14196b52a, 0x14196b53c> as *const (), Throws::Discard), // sub_14196b4a0
    Replacement::draft_throws(0x14196b550, "unknown_14196b550", tpl_35767::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140401840, 0x20, 0x143109e80, 0x142116bc0, 0x1421158a0, 0x14196b5e0, 0x14196b5f2> as *const (), Throws::Discard), // sub_14196b550
    Replacement::draft_throws(0x14196b600, "unknown_14196b600", fn_14196b600 as *const (), Throws::Discard), // sub_14196b600
    Replacement::draft_throws(0x14196b6c0, "unknown_14196b6c0", tpl_35770::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404013d0, 0x20, 0x30, 0x14310a420, 0x142116bc0, 0x1421158a0, 0x14196b758, 0x14196b76a> as *const (), Throws::Discard), // sub_14196b6c0
    Replacement::draft_throws(0x14196b780, "unknown_14196b780", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404011c0, 0x10, 0x143109d48, 0x142116bc0, 0x1421158a0, 0x14196b80a, 0x14196b81c> as *const (), Throws::Discard), // sub_14196b780
    Replacement::draft_throws(0x14196b830, "unknown_14196b830", fn_14196b830 as *const (), Throws::Discard), // sub_14196b830
    Replacement::draft_throws(0x14196b8f0, "unknown_14196b8f0", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140400df0, 0x10, 0x14310a450, 0x142116bc0, 0x1421158a0, 0x14196b97a, 0x14196b98c> as *const (), Throws::Discard), // sub_14196b8f0
    Replacement::draft_throws(0x14196b9a0, "unknown_14196b9a0", fn_14196b9a0 as *const (), Throws::Discard), // sub_14196b9a0
    Replacement::draft_throws(0x14196bb70, "unknown_14196bb70", fn_14196bb70 as *const (), Throws::Discard), // sub_14196bb70
    Replacement::draft_throws(0x14196bd40, "unknown_14196bd40", tpl_35777::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140400c40, 0x20, 0x14310a450, 0x142116bc0, 0x1421158a0, 0x14196bdcd, 0x14196bddf> as *const (), Throws::Discard), // sub_14196bd40
    Replacement::draft_throws(0x14196bdf0, "unknown_14196bdf0", tpl_35770::<0x142119010, 0x1, 0x142118e10, 0x78, 0x140400a10, 0x20, 0x30, 0x14310a450, 0x142116bc0, 0x1421158a0, 0x14196be88, 0x14196be9a> as *const (), Throws::Discard), // sub_14196bdf0
    Replacement::draft_throws(0x14196beb0, "unknown_14196beb0", tpl_35769::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404007a0, 0x14310a4b8, 0x142116bc0, 0x1421158a0, 0x14196bf32, 0x14196bf44> as *const (), Throws::Discard), // sub_14196beb0
    Replacement::draft_throws(0x14196bf50, "unknown_14196bf50", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1404002b0, 0x10, 0x143109950, 0x142116bc0, 0x1421158a0, 0x14196bfda, 0x14196bfec> as *const (), Throws::Discard), // sub_14196bf50
    Replacement::draft_throws(0x14196c000, "unknown_14196c000", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1403ffec0, 0x10, 0x143109968, 0x142116bc0, 0x1421158a0, 0x14196c08a, 0x14196c09c> as *const (), Throws::Discard), // sub_14196c000
    Replacement::draft_throws(0x14196c0b0, "unknown_14196c0b0", fn_14196c0b0 as *const (), Throws::Discard), // sub_14196c0b0
    Replacement::draft_throws(0x14196c280, "unknown_14196c280", fn_14196c280 as *const (), Throws::Discard), // sub_14196c280
    Replacement::draft_throws(0x14196c450, "unknown_14196c450", fn_14196c450 as *const (), Throws::Discard), // sub_14196c450
    Replacement::draft_throws(0x14196c4c0, "unknown_14196c4c0", fn_14196c4c0 as *const (), Throws::Discard), // sub_14196c4c0
    Replacement::draft_throws(0x14196c6b0, "unknown_14196c6b0", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1403ffb60, 0x10, 0x143108c50, 0x142116bc0, 0x1421158a0, 0x14196c73a, 0x14196c74c> as *const (), Throws::Discard), // sub_14196c6b0
    Replacement::draft_throws(0x14196c760, "unknown_14196c760", tpl_35766::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1403ff950, 0x10, 0x143108a30, 0x142116bc0, 0x1421158a0, 0x14196c7ea, 0x14196c7fc> as *const (), Throws::Discard), // sub_14196c760
    Replacement::draft_throws(0x14196c810, "unknown_14196c810", tpl_35782::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1403ff740, 0x1421158a0, 0x14196c870> as *const (), Throws::Discard), // sub_14196c810
    Replacement::draft_throws(0x14196c880, "unknown_14196c880", tpl_35782::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1403ff520, 0x1421158a0, 0x14196c8e0> as *const (), Throws::Discard), // sub_14196c880
    Replacement::draft_throws(0x14196c8f0, "unknown_14196c8f0", tpl_35782::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1403ff310, 0x1421158a0, 0x14196c950> as *const (), Throws::Discard), // sub_14196c8f0
    Replacement::draft_throws(0x14196c960, "unknown_14196c960", tpl_35782::<0x142119010, 0x1, 0x142118e10, 0x78, 0x1403ff170, 0x1421158a0, 0x14196c9c0> as *const (), Throws::Discard), // sub_14196c960
    Replacement::draft(0x14196c9d0, "unknown_14196c9d0", tpl_28737::<0x141ab4530> as *const ()), // sub_14196c9d0
    Replacement::draft_throws(0x14196c9f0, "unknown_14196c9f0", fn_14196c9f0 as *const (), Throws::Discard), // sub_14196c9f0
    Replacement::draft_throws(0x14196ca80, "unknown_14196ca80", fn_14196ca80 as *const (), Throws::Discard), // sub_14196ca80
    Replacement::draft_throws(0x14196cbd0, "unknown_14196cbd0", fn_14196cbd0 as *const (), Throws::Discard), // sub_14196cbd0
    Replacement::draft_throws(0x14196cc20, "unknown_14196cc20", fn_14196cc20 as *const (), Throws::Discard), // sub_14196cc20
    Replacement::draft_throws(0x14196cd60, "unknown_14196cd60", fn_14196cd60 as *const (), Throws::Discard), // sub_14196cd60
    Replacement::draft_throws(0x14196cd80, "unknown_14196cd80", fn_14196cd80 as *const (), Throws::Discard), // sub_14196cd80
    Replacement::draft(0x14196ce80, "unknown_14196ce80", tpl_35786::<0x8, 0x14397d850, 0x123, 0x10> as *const ()), // sub_14196ce80
    Replacement::draft(0x14196cec0, "unknown_14196cec0", tpl_35786::<0x8, 0x14397d850, 0x122, 0x10> as *const ()), // sub_14196cec0
    Replacement::draft_throws(0x14196cf00, "unknown_14196cf00", fn_14196cf00 as *const (), Throws::Discard), // sub_14196cf00
    Replacement::draft_throws(0x14196cfd0, "unknown_14196cfd0", fn_14196cfd0 as *const (), Throws::Caught), // sub_14196cfd0
    Replacement::draft_throws(0x14196d6d0, "unknown_14196d6d0", fn_14196d6d0 as *const (), Throws::Caught), // sub_14196d6d0
    Replacement::draft_throws(0x14196ddf0, "unknown_14196ddf0", fn_14196ddf0 as *const (), Throws::Caught), // sub_14196ddf0
    Replacement::draft_throws(0x14196e190, "unknown_14196e190", fn_14196e190 as *const (), Throws::Caught), // sub_14196e190
    Replacement::draft_throws(0x14196e570, "unknown_14196e570", fn_14196e570 as *const (), Throws::Discard), // sub_14196e570
    Replacement::draft_throws(0x14196e6f0, "unknown_14196e6f0", fn_14196e6f0 as *const (), Throws::Discard), // sub_14196e6f0
    Replacement::draft_throws(0x14196e820, "unknown_14196e820", fn_14196e820 as *const (), Throws::Discard), // sub_14196e820
    Replacement::draft_throws(0x14196e960, "unknown_14196e960", fn_14196e960 as *const (), Throws::Discard), // sub_14196e960
    Replacement::draft_throws(0x14196eaf0, "unknown_14196eaf0", fn_14196eaf0 as *const (), Throws::Discard), // sub_14196eaf0
    Replacement::draft_throws(0x14196ec30, "unknown_14196ec30", fn_14196ec30 as *const (), Throws::Discard), // sub_14196ec30
    Replacement::draft_throws(0x14196ed30, "unknown_14196ed30", tpl_35794::<0x78, 0x14397d850, 0x141a0bc80> as *const (), Throws::Discard), // sub_14196ed30
    Replacement::draft_throws(0x14196ed60, "unknown_14196ed60", tpl_35794::<0x78, 0x14397d850, 0x141a0c880> as *const (), Throws::Discard), // sub_14196ed60
    Replacement::draft_throws(0x14196ed90, "unknown_14196ed90", fn_14196ed90 as *const (), Throws::Caught), // sub_14196ed90
    Replacement::draft_throws(0x14196f020, "unknown_14196f020", fn_14196f020 as *const (), Throws::Discard), // sub_14196f020
    Replacement::draft(0x14196f0d0, "unknown_14196f0d0", tpl_361 as *const ()), // sub_14196f0d0
    Replacement::draft(0x14196f0e0, "unknown_14196f0e0", fn_14196f0e0 as *const ()), // sub_14196f0e0
    Replacement::draft_throws(0x14196f2f0, "unknown_14196f2f0", fn_14196f2f0 as *const (), Throws::Discard), // sub_14196f2f0
    Replacement::draft_throws(0x14196f420, "unknown_14196f420", fn_14196f420 as *const (), Throws::Discard), // sub_14196f420
    Replacement::draft(0x14196f5c0, "unknown_14196f5c0", tpl_361 as *const ()), // sub_14196f5c0
    Replacement::draft_throws(0x14196f5d0, "unknown_14196f5d0", fn_14196f5d0 as *const (), Throws::Discard), // sub_14196f5d0
    Replacement::draft(0x14196f680, "unknown_14196f680", tpl_559 as *const ()), // sub_14196f680
    Replacement::draft(0x14196f690, "unknown_14196f690", tpl_559 as *const ()), // sub_14196f690
    Replacement::draft(0x14196f6a0, "unknown_14196f6a0", tpl_559 as *const ()), // sub_14196f6a0
    Replacement::draft(0x14196f6b0, "unknown_14196f6b0", tpl_559 as *const ()), // sub_14196f6b0
    Replacement::draft(0x14196f6c0, "unknown_14196f6c0", tpl_559 as *const ()), // sub_14196f6c0
    Replacement::draft(0x14196f6d0, "unknown_14196f6d0", tpl_559 as *const ()), // sub_14196f6d0
    Replacement::draft(0x14196f6e0, "unknown_14196f6e0", tpl_559 as *const ()), // sub_14196f6e0
    Replacement::draft(0x14196f6f0, "unknown_14196f6f0", tpl_559 as *const ()), // sub_14196f6f0
    Replacement::draft_throws(0x14196f700, "unknown_14196f700", fn_14196f700 as *const (), Throws::Discard), // sub_14196f700
    Replacement::draft_throws(0x14196f810, "unknown_14196f810", fn_14196f810 as *const (), Throws::Caught), // sub_14196f810
    Replacement::draft(0x14196fa00, "unknown_14196fa00", tpl_361 as *const ()), // sub_14196fa00
    Replacement::draft_throws(0x14196fa10, "unknown_14196fa10", fn_14196fa10 as *const (), Throws::Caught), // sub_14196fa10
    Replacement::draft(0x14196fd00, "unknown_14196fd00", tpl_361 as *const ()), // sub_14196fd00
    Replacement::draft_throws(0x14196fd10, "unknown_14196fd10", fn_14196fd10 as *const (), Throws::Discard), // sub_14196fd10
    Replacement::draft_throws(0x14196ffc0, "unknown_14196ffc0", fn_14196ffc0 as *const (), Throws::Discard), // sub_14196ffc0
    Replacement::draft(0x141970120, "unknown_141970120", tpl_361 as *const ()), // sub_141970120
    Replacement::draft_throws(0x141970130, "unknown_141970130", fn_141970130 as *const (), Throws::Discard), // sub_141970130
    Replacement::draft_throws(0x141970290, "unknown_141970290", fn_141970290 as *const (), Throws::Caught), // sub_141970290
    Replacement::draft_throws(0x141970530, "unknown_141970530", fn_141970530 as *const (), Throws::Discard), // sub_141970530
    Replacement::draft(0x1419709e0, "unknown_1419709e0", tpl_361 as *const ()), // sub_1419709e0
    Replacement::draft_throws(0x1419709f0, "unknown_1419709f0", fn_1419709f0 as *const (), Throws::Caught), // sub_1419709f0
    Replacement::draft(0x141970f10, "unknown_141970f10", tpl_361 as *const ()), // sub_141970f10
    Replacement::draft(0x141970f20, "unknown_141970f20", fn_141970f20 as *const ()), // sub_141970f20
    Replacement::draft(0x141971070, "unknown_141971070", tpl_361 as *const ()), // sub_141971070
    Replacement::draft(0x141971080, "unknown_141971080", fn_141971080 as *const ()), // sub_141971080
    Replacement::draft(0x1419713b0, "unknown_1419713b0", tpl_361 as *const ()), // sub_1419713b0
    Replacement::draft(0x1419713c0, "unknown_1419713c0", fn_1419713c0 as *const ()), // sub_1419713c0
    Replacement::draft(0x141971620, "unknown_141971620", tpl_361 as *const ()), // sub_141971620
    Replacement::draft(0x141971630, "unknown_141971630", fn_141971630 as *const ()), // sub_141971630
    Replacement::draft(0x141971860, "unknown_141971860", tpl_361 as *const ()), // sub_141971860
    Replacement::draft_throws(0x141971870, "unknown_141971870", fn_141971870 as *const (), Throws::Caught), // sub_141971870
    Replacement::draft(0x141971b60, "unknown_141971b60", tpl_361 as *const ()), // sub_141971b60
    Replacement::draft_throws(0x141971b70, "unknown_141971b70", fn_141971b70 as *const (), Throws::Caught), // sub_141971b70
    Replacement::draft_throws(0x1419725b0, "unknown_1419725b0", fn_1419725b0 as *const (), Throws::Caught), // sub_1419725b0
    Replacement::draft_throws(0x141972f60, "unknown_141972f60", fn_141972f60 as *const (), Throws::Discard), // sub_141972f60
    Replacement::draft_throws(0x141973390, "unknown_141973390", fn_141973390 as *const (), Throws::Discard), // sub_141973390
    Replacement::draft_throws(0x141973550, "unknown_141973550", fn_141973550 as *const (), Throws::Discard), // sub_141973550
    Replacement::draft_throws(0x141973710, "unknown_141973710", fn_141973710 as *const (), Throws::Discard), // sub_141973710
    Replacement::draft_throws(0x141973810, "unknown_141973810", fn_141973810 as *const (), Throws::Discard), // sub_141973810
    Replacement::draft_throws(0x141973c90, "unknown_141973c90", fn_141973c90 as *const (), Throws::Caught), // sub_141973c90
    Replacement::draft_throws(0x1419740f0, "unknown_1419740f0", fn_1419740f0 as *const (), Throws::Discard), // sub_1419740f0
    Replacement::draft_throws(0x1419743a0, "unknown_1419743a0", fn_1419743a0 as *const (), Throws::Discard), // sub_1419743a0
    Replacement::draft_throws(0x141974470, "unknown_141974470", fn_141974470 as *const (), Throws::Discard), // sub_141974470
    Replacement::draft_throws(0x141974540, "unknown_141974540", tpl_35812::<0x14310d064, 0x143427080, 0x1403f4620, 0x143107164, 0x14, 0x14310d1a0, 0x18, 0x14310d198, 0x1c, 0x20> as *const (), Throws::Discard), // sub_141974540
    Replacement::draft_throws(0x1419745f0, "unknown_1419745f0", fn_1419745f0 as *const (), Throws::Discard), // sub_1419745f0
    Replacement::draft_throws(0x1419746a0, "unknown_1419746a0", tpl_35812::<0x14310d064, 0x143427080, 0x1403f4620, 0x143107164, 0x3c, 0x14310d1a0, 0x40, 0x14310d198, 0x44, 0x48> as *const (), Throws::Discard), // sub_1419746a0
    Replacement::draft_throws(0x141974750, "unknown_141974750", tpl_35812::<0x14310d064, 0x143427080, 0x1403f4620, 0x143107164, 0x28, 0x14310d1a0, 0x2c, 0x14310d198, 0x30, 0x34> as *const (), Throws::Discard), // sub_141974750
    Replacement::draft_throws(0x141974800, "unknown_141974800", tpl_35812::<0x14310d064, 0x143427080, 0x1403f4620, 0x143107164, 0x64, 0x14310d1a0, 0x68, 0x14310d198, 0x6c, 0x70> as *const (), Throws::Discard), // sub_141974800
    Replacement::draft_throws(0x1419748b0, "unknown_1419748b0", tpl_35812::<0x14310d064, 0x143427080, 0x1403f4620, 0x143107164, 0x50, 0x14310d1a0, 0x54, 0x14310d198, 0x58, 0x5c> as *const (), Throws::Discard), // sub_1419748b0
    Replacement::draft_throws(0x141974960, "unknown_141974960", fn_141974960 as *const (), Throws::Caught), // sub_141974960
    Replacement::draft(0x141974b80, "unknown_141974b80", tpl_361 as *const ()), // sub_141974b80
    Replacement::draft_throws(0x141974b90, "unknown_141974b90", fn_141974b90 as *const (), Throws::Discard), // sub_141974b90
    Replacement::draft_throws(0x141974d80, "unknown_141974d80", fn_141974d80 as *const (), Throws::Discard), // sub_141974d80
    Replacement::draft_throws(0x141974fc0, "unknown_141974fc0", fn_141974fc0 as *const (), Throws::Caught), // sub_141974fc0
    Replacement::draft_throws(0x141975830, "unknown_141975830", fn_141975830 as *const (), Throws::Caught), // sub_141975830
    Replacement::draft_throws(0x141976000, "unknown_141976000", fn_141976000 as *const (), Throws::Caught), // sub_141976000
    Replacement::draft(0x1419765b0, "unknown_1419765b0", fn_1419765b0 as *const ()), // sub_1419765b0
    Replacement::draft_throws(0x141976620, "unknown_141976620", fn_141976620 as *const (), Throws::Caught), // sub_141976620
    Replacement::draft_throws(0x141977230, "unknown_141977230", fn_141977230 as *const (), Throws::Caught), // sub_141977230
    Replacement::draft_throws(0x141977740, "unknown_141977740", fn_141977740 as *const (), Throws::Caught), // sub_141977740
    Replacement::draft_throws(0x141977b10, "unknown_141977b10", fn_141977b10 as *const (), Throws::Caught), // sub_141977b10
    Replacement::draft(0x141977d30, "unknown_141977d30", tpl_361 as *const ()), // sub_141977d30
    Replacement::draft_throws(0x141977d40, "unknown_141977d40", fn_141977d40 as *const (), Throws::Discard), // sub_141977d40
    Replacement::draft_throws(0x141977f30, "unknown_141977f30", fn_141977f30 as *const (), Throws::Discard), // sub_141977f30
    Replacement::draft_throws(0x141978030, "unknown_141978030", fn_141978030 as *const (), Throws::Caught), // sub_141978030
    Replacement::draft(0x1419784d0, "unknown_1419784d0", tpl_361 as *const ()), // sub_1419784d0
    Replacement::draft_throws(0x1419784e0, "unknown_1419784e0", tpl_35827::<0x143c9d84c, 0x1401e7c10, 0x1403f6760, 0x140242ed0> as *const (), Throws::Discard), // sub_1419784e0
    Replacement::draft_throws(0x1419786d0, "unknown_1419786d0", fn_1419786d0 as *const (), Throws::Discard), // sub_1419786d0
    Replacement::draft_throws(0x141978760, "unknown_141978760", tpl_35827::<0x143c9ed2c, 0x1401e7c10, 0x1403f6860, 0x140242ed0> as *const (), Throws::Discard), // sub_141978760
    Replacement::draft_throws(0x141978950, "unknown_141978950", fn_141978950 as *const (), Throws::Discard), // sub_141978950
    Replacement::draft(0x141978fd0, "unknown_141978fd0", tpl_361 as *const ()), // sub_141978fd0
    Replacement::draft_throws(0x141978fe0, "unknown_141978fe0", tpl_35827::<0x143c9ed2c, 0x1401e7c10, 0x1403f6860, 0x140242ed0> as *const (), Throws::Discard), // sub_141978fe0
    Replacement::draft_throws(0x1419791d0, "unknown_1419791d0", fn_1419791d0 as *const (), Throws::Discard), // sub_1419791d0
    Replacement::draft(0x141979840, "unknown_141979840", tpl_361 as *const ()), // sub_141979840
    Replacement::draft_throws(0x141979850, "unknown_141979850", tpl_35827::<0x143c9da08, 0x1401e7c10, 0x1403f6960, 0x140242ed0> as *const (), Throws::Discard), // sub_141979850
    Replacement::draft_throws(0x141979a40, "unknown_141979a40", fn_141979a40 as *const (), Throws::Discard), // sub_141979a40
    Replacement::draft_throws(0x14197a300, "unknown_14197a300", fn_14197a300 as *const (), Throws::Caught), // sub_14197a300
    Replacement::draft_throws(0x14197a860, "unknown_14197a860", fn_14197a860 as *const (), Throws::Discard), // sub_14197a860
    Replacement::draft(0x14197aa50, "unknown_14197aa50", tpl_361 as *const ()), // sub_14197aa50
    Replacement::draft_throws(0x14197aa60, "unknown_14197aa60", fn_14197aa60 as *const (), Throws::Discard), // sub_14197aa60
    Replacement::draft_throws(0x14197ac50, "unknown_14197ac50", fn_14197ac50 as *const (), Throws::Discard), // sub_14197ac50
    Replacement::draft(0x14197afa0, "unknown_14197afa0", tpl_361 as *const ()), // sub_14197afa0
    Replacement::draft_throws(0x14197afb0, "unknown_14197afb0", tpl_35834::<0x143c9f0bc, 0x1401e7c10, 0x1403f6b60, 0x140242ed0> as *const (), Throws::Discard), // sub_14197afb0
    Replacement::draft_throws(0x14197b1a0, "unknown_14197b1a0", fn_14197b1a0 as *const (), Throws::Discard), // sub_14197b1a0
    Replacement::draft(0x14197b360, "unknown_14197b360", tpl_361 as *const ()), // sub_14197b360
    Replacement::draft_throws(0x14197b370, "unknown_14197b370", tpl_35834::<0x143c9da00, 0x1401e7c10, 0x1403f6c60, 0x140242ed0> as *const (), Throws::Discard), // sub_14197b370
    Replacement::draft_throws(0x14197b560, "unknown_14197b560", fn_14197b560 as *const (), Throws::Discard), // sub_14197b560
    Replacement::draft(0x14197b7d0, "unknown_14197b7d0", tpl_361 as *const ()), // sub_14197b7d0
    Replacement::draft_throws(0x14197b7e0, "unknown_14197b7e0", tpl_35834::<0x143c9efb4, 0x1401e7c10, 0x1403f6d60, 0x140242ed0> as *const (), Throws::Discard), // sub_14197b7e0
    Replacement::draft_throws(0x14197b9d0, "unknown_14197b9d0", fn_14197b9d0 as *const (), Throws::Discard), // sub_14197b9d0
    Replacement::draft_throws(0x14197bd40, "unknown_14197bd40", fn_14197bd40 as *const (), Throws::Discard), // sub_14197bd40
    Replacement::draft(0x14197bdf0, "unknown_14197bdf0", tpl_35839::<0x8, 0x10> as *const ()), // sub_14197bdf0
    Replacement::draft(0x14197be20, "unknown_14197be20", fn_14197be20 as *const ()), // sub_14197be20
    Replacement::draft_throws(0x14197be90, "unknown_14197be90", fn_14197be90 as *const (), Throws::Discard), // sub_14197be90
    Replacement::draft_throws(0x14197c030, "unknown_14197c030", fn_14197c030 as *const (), Throws::Caught), // sub_14197c030
    Replacement::draft_throws(0x14197c240, "unknown_14197c240", fn_14197c240 as *const (), Throws::Discard), // sub_14197c240
    Replacement::draft_throws(0x14197c350, "unknown_14197c350", fn_14197c350 as *const (), Throws::Caught), // sub_14197c350
    Replacement::draft_throws(0x14197c5a0, "unknown_14197c5a0", fn_14197c5a0 as *const (), Throws::Caught), // sub_14197c5a0
    Replacement::draft_throws(0x14197cf40, "unknown_14197cf40", fn_14197cf40 as *const (), Throws::Caught), // sub_14197cf40
    Replacement::draft_throws(0x14197d910, "unknown_14197d910", fn_14197d910 as *const (), Throws::Discard), // sub_14197d910
    Replacement::draft_throws(0x14197d9c0, "unknown_14197d9c0", fn_14197d9c0 as *const (), Throws::Caught), // sub_14197d9c0
    Replacement::draft(0x14197de10, "unknown_14197de10", tpl_35253 as *const ()), // sub_14197de10
    Replacement::draft_throws(0x14197de20, "unknown_14197de20", fn_14197de20 as *const (), Throws::Discard), // sub_14197de20
    Replacement::draft_throws(0x14197df00, "unknown_14197df00", fn_14197df00 as *const (), Throws::Discard), // sub_14197df00
    Replacement::draft(0x14197e720, "unknown_14197e720", tpl_35253 as *const ()), // sub_14197e720
    Replacement::draft_throws(0x14197e730, "unknown_14197e730", fn_14197e730 as *const (), Throws::Discard), // sub_14197e730
    Replacement::draft_throws(0x14197e810, "unknown_14197e810", fn_14197e810 as *const (), Throws::Discard), // sub_14197e810
    Replacement::draft_throws(0x14197efa0, "unknown_14197efa0", fn_14197efa0 as *const (), Throws::Discard), // sub_14197efa0
    Replacement::draft_throws(0x14197f510, "unknown_14197f510", fn_14197f510 as *const (), Throws::Discard), // sub_14197f510
    Replacement::draft_throws(0x14197f960, "unknown_14197f960", fn_14197f960 as *const (), Throws::Discard), // sub_14197f960
    Replacement::draft(0x14197ff10, "unknown_14197ff10", tpl_361 as *const ()), // sub_14197ff10
    Replacement::draft_throws(0x14197ff20, "unknown_14197ff20", fn_14197ff20 as *const (), Throws::Discard), // sub_14197ff20
    Replacement::draft_throws(0x141980140, "unknown_141980140", fn_141980140 as *const (), Throws::Discard), // sub_141980140
    Replacement::draft_throws(0x1419802a0, "unknown_1419802a0", fn_1419802a0 as *const (), Throws::Discard), // sub_1419802a0
    Replacement::draft_throws(0x1419804d0, "unknown_1419804d0", fn_1419804d0 as *const (), Throws::Discard), // sub_1419804d0
    Replacement::draft_throws(0x141980670, "unknown_141980670", fn_141980670 as *const (), Throws::Discard), // sub_141980670
    Replacement::draft(0x141980a00, "unknown_141980a00", fn_141980a00 as *const ()), // sub_141980a00
    Replacement::draft(0x141980a20, "unknown_141980a20", tpl_35860 as *const ()), // sub_141980a20
    Replacement::draft(0x141980a40, "unknown_141980a40", tpl_35861 as *const ()), // sub_141980a40
    Replacement::draft_throws(0x141980a60, "unknown_141980a60", fn_141980a60 as *const (), Throws::Discard), // sub_141980a60
    Replacement::draft(0x141981030, "unknown_141981030", tpl_35862 as *const ()), // sub_141981030
    Replacement::draft(0x141981040, "unknown_141981040", tpl_35860 as *const ()), // sub_141981040
    Replacement::draft(0x141981060, "unknown_141981060", tpl_35861 as *const ()), // sub_141981060
    Replacement::draft_throws(0x141981080, "unknown_141981080", fn_141981080 as *const (), Throws::Caught), // sub_141981080
    Replacement::draft(0x141981560, "unknown_141981560", fn_141981560 as *const ()), // sub_141981560
    Replacement::draft(0x141981660, "unknown_141981660", fn_141981660 as *const ()), // sub_141981660
    Replacement::draft(0x141981760, "unknown_141981760", fn_141981760 as *const ()), // sub_141981760
    Replacement::draft(0x141981860, "unknown_141981860", fn_141981860 as *const ()), // sub_141981860
    Replacement::draft(0x141981960, "unknown_141981960", fn_141981960 as *const ()), // sub_141981960
    Replacement::draft(0x141981a60, "unknown_141981a60", fn_141981a60 as *const ()), // sub_141981a60
    Replacement::draft(0x141981b60, "unknown_141981b60", fn_141981b60 as *const ()), // sub_141981b60
    Replacement::draft(0x141981c60, "unknown_141981c60", fn_141981c60 as *const ()), // sub_141981c60
    Replacement::draft(0x141981d70, "unknown_141981d70", tpl_361 as *const ()), // sub_141981d70
    Replacement::draft_throws(0x141981d80, "unknown_141981d80", fn_141981d80 as *const (), Throws::Discard), // sub_141981d80
    Replacement::draft(0x141982140, "unknown_141982140", tpl_496 as *const ()), // sub_141982140
    Replacement::draft_throws(0x141982150, "unknown_141982150", fn_141982150 as *const (), Throws::Caught), // sub_141982150
    Replacement::draft(0x1419825c0, "unknown_1419825c0", tpl_496 as *const ()), // sub_1419825c0
    Replacement::draft_throws(0x1419825d0, "unknown_1419825d0", fn_1419825d0 as *const (), Throws::Discard), // sub_1419825d0
    Replacement::draft_throws(0x141982ac0, "unknown_141982ac0", tpl_35874::<0x1403fc3c0, 0x8, 0xc, 0x10> as *const (), Throws::Discard), // sub_141982ac0
    Replacement::draft_throws(0x141982b00, "unknown_141982b00", fn_141982b00 as *const (), Throws::Discard), // sub_141982b00
    Replacement::draft_throws(0x141982bf0, "unknown_141982bf0", fn_141982bf0 as *const (), Throws::Discard), // sub_141982bf0
    Replacement::draft_throws(0x141982eb0, "unknown_141982eb0", fn_141982eb0 as *const (), Throws::Discard), // sub_141982eb0
    Replacement::draft_throws(0x1419831c0, "unknown_1419831c0", fn_1419831c0 as *const (), Throws::Discard), // sub_1419831c0
    Replacement::draft_throws(0x1419839c0, "unknown_1419839c0", fn_1419839c0 as *const (), Throws::Discard), // sub_1419839c0
    Replacement::draft(0x141983cf0, "unknown_141983cf0", fn_141983cf0 as *const ()), // sub_141983cf0
    Replacement::draft_throws(0x141983d70, "unknown_141983d70", tpl_35880::<0x78, 0x143c8d118, 0x580e0, 0x10, 0x8, 0x1429868d0, 0x4, 0x1421158a0, 0x143110698, 0x142116b30, 0x141983e19, 0x141983e26> as *const (), Throws::Discard), // sub_141983d70
    Replacement::draft_throws(0x141983e30, "unknown_141983e30", tpl_35880::<0x78, 0x143c8d110, 0x580e0, 0x10, 0x8, 0x1429868d0, 0x425, 0x1421158a0, 0x143110698, 0x142116b30, 0x141983ee2, 0x141983eef> as *const (), Throws::Discard), // sub_141983e30
    Replacement::draft_throws(0x141983f00, "unknown_141983f00", fn_141983f00 as *const (), Throws::Discard), // sub_141983f00
    Replacement::draft_throws(0x141983f60, "unknown_141983f60", fn_141983f60 as *const (), Throws::Discard), // sub_141983f60
    Replacement::draft_throws(0x141984020, "unknown_141984020", fn_141984020 as *const (), Throws::Caught), // sub_141984020
    Replacement::draft(0x1419842e0, "unknown_1419842e0", tpl_361 as *const ()), // sub_1419842e0
    Replacement::draft_throws(0x1419842f0, "unknown_1419842f0", fn_1419842f0 as *const (), Throws::Discard), // sub_1419842f0
    Replacement::draft_throws(0x141984630, "unknown_141984630", fn_141984630 as *const (), Throws::Discard), // sub_141984630
    Replacement::draft_throws(0x141984940, "unknown_141984940", fn_141984940 as *const (), Throws::Caught), // sub_141984940
    Replacement::draft_throws(0x141984c80, "unknown_141984c80", fn_141984c80 as *const (), Throws::Discard), // sub_141984c80
    Replacement::draft(0x141984f00, "unknown_141984f00", fn_141984f00 as *const ()), // sub_141984f00
    Replacement::draft(0x141984f40, "unknown_141984f40", tpl_361 as *const ()), // sub_141984f40
    Replacement::draft_throws(0x141984f50, "unknown_141984f50", fn_141984f50 as *const (), Throws::Discard), // sub_141984f50
    Replacement::draft_throws(0x1419850c0, "unknown_1419850c0", fn_1419850c0 as *const (), Throws::Caught), // sub_1419850c0
    Replacement::draft_throws(0x141985220, "unknown_141985220", fn_141985220 as *const (), Throws::Caught), // sub_141985220
    Replacement::draft_throws(0x1419856a0, "unknown_1419856a0", fn_1419856a0 as *const (), Throws::Caught), // sub_1419856a0
    Replacement::draft_throws(0x1419859d0, "unknown_1419859d0", tpl_35890::<0x78, 0x143c8d110, 0x580e0, 0x141ade6e0, 0x143110698, 0x142116b30, 0x141985a13> as *const (), Throws::Discard), // sub_1419859d0
    Replacement::draft_throws(0x141985a20, "unknown_141985a20", fn_141985a20 as *const (), Throws::Discard), // sub_141985a20
    Replacement::draft_throws(0x141985c80, "unknown_141985c80", fn_141985c80 as *const (), Throws::Caught), // sub_141985c80
    Replacement::draft_throws(0x141985df0, "unknown_141985df0", fn_141985df0 as *const (), Throws::Caught), // sub_141985df0
    Replacement::draft_throws(0x141985f60, "unknown_141985f60", fn_141985f60 as *const (), Throws::Discard), // sub_141985f60
    Replacement::draft_throws(0x141986080, "unknown_141986080", tpl_35893::<0x78, 0x143c8d110, 0x580e0, 0x14198d9f0, 0x8, 0x301, 0x10, 0x143110698, 0x142116b30, 0x1419860e8> as *const (), Throws::Discard), // sub_141986080
    Replacement::draft_throws(0x1419860f0, "unknown_1419860f0", tpl_35893::<0x78, 0x143c8d110, 0x580e0, 0x14198d9f0, 0x8, 0x300, 0x10, 0x143110698, 0x142116b30, 0x141986158> as *const (), Throws::Discard), // sub_1419860f0
    Replacement::draft_throws(0x141986160, "unknown_141986160", fn_141986160 as *const (), Throws::Caught), // sub_141986160
    Replacement::draft_throws(0x141986270, "unknown_141986270", fn_141986270 as *const (), Throws::Discard), // sub_141986270
    Replacement::draft_throws(0x141986310, "unknown_141986310", fn_141986310 as *const (), Throws::Discard), // sub_141986310
    Replacement::draft_throws(0x1419863b0, "unknown_1419863b0", fn_1419863b0 as *const (), Throws::Discard), // sub_1419863b0
    Replacement::draft_throws(0x141986530, "unknown_141986530", tpl_35893::<0x78, 0x143c8d110, 0x580e0, 0x14198d9f0, 0x8, 0x304, 0x10, 0x143110698, 0x142116b30, 0x141986598> as *const (), Throws::Discard), // sub_141986530
    Replacement::draft_throws(0x1419865a0, "unknown_1419865a0", fn_1419865a0 as *const (), Throws::Caught), // sub_1419865a0
    Replacement::draft_throws(0x1419867c0, "unknown_1419867c0", fn_1419867c0 as *const (), Throws::Discard), // sub_1419867c0
    Replacement::draft_throws(0x141986950, "unknown_141986950", fn_141986950 as *const (), Throws::Discard), // sub_141986950
    Replacement::draft_throws(0x141986ad0, "unknown_141986ad0", fn_141986ad0 as *const (), Throws::Caught), // sub_141986ad0
    Replacement::draft_throws(0x141986c70, "unknown_141986c70", tpl_35899::<0x78, 0x143c8d110, 0x580e0, 0x3a0, 0x8, 0x4, 0xc, 0x10, 0x143110698, 0x142116b30, 0x141986cdd> as *const (), Throws::Discard), // sub_141986c70
    Replacement::draft_throws(0x141986cf0, "unknown_141986cf0", tpl_35899::<0x78, 0x143c8d110, 0x580e0, 0x3b0, 0x8, 0x4, 0xc, 0x10, 0x143110698, 0x142116b30, 0x141986d5d> as *const (), Throws::Discard), // sub_141986cf0
    Replacement::draft_throws(0x141986d70, "unknown_141986d70", fn_141986d70 as *const (), Throws::Caught), // sub_141986d70
    Replacement::draft_throws(0x141986fa0, "unknown_141986fa0", fn_141986fa0 as *const (), Throws::Discard), // sub_141986fa0
    Replacement::draft_throws(0x141987030, "unknown_141987030", fn_141987030 as *const (), Throws::Discard), // sub_141987030
    Replacement::draft_throws(0x141987100, "unknown_141987100", tpl_35893::<0x78, 0x143c8d110, 0x580e0, 0x14198d9f0, 0x8, 0x307, 0x10, 0x143110698, 0x142116b30, 0x141987168> as *const (), Throws::Discard), // sub_141987100
    Replacement::draft_throws(0x141987170, "unknown_141987170", tpl_35880::<0x78, 0x143c8d110, 0x580e0, 0x10, 0x8, 0x1429868d0, 0x45a, 0x1421158a0, 0x143110698, 0x142116b30, 0x141987222, 0x14198722f> as *const (), Throws::Discard), // sub_141987170
    Replacement::draft_throws(0x141987240, "unknown_141987240", fn_141987240 as *const (), Throws::Discard), // sub_141987240
    Replacement::draft_throws(0x1419872a0, "unknown_1419872a0", fn_1419872a0 as *const (), Throws::Discard), // sub_1419872a0
    Replacement::draft_throws(0x141987360, "unknown_141987360", fn_141987360 as *const (), Throws::Discard), // sub_141987360
    Replacement::draft_throws(0x141987510, "unknown_141987510", fn_141987510 as *const (), Throws::Discard), // sub_141987510
    Replacement::draft_throws(0x141987590, "unknown_141987590", tpl_35880::<0x78, 0x143c8d110, 0x580e0, 0x10, 0x8, 0x1429868d0, 0x424, 0x1421158a0, 0x143110698, 0x142116b30, 0x141987642, 0x14198764f> as *const (), Throws::Discard), // sub_141987590
    Replacement::draft_throws(0x141987660, "unknown_141987660", fn_141987660 as *const (), Throws::Discard), // sub_141987660
    Replacement::draft_throws(0x141987720, "unknown_141987720", fn_141987720 as *const (), Throws::Discard), // sub_141987720
    Replacement::draft_throws(0x141987f60, "unknown_141987f60", fn_141987f60 as *const (), Throws::Discard), // sub_141987f60
    Replacement::draft_throws(0x14198a0f0, "unknown_14198a0f0", fn_14198a0f0 as *const (), Throws::Discard), // sub_14198a0f0
    Replacement::draft_throws(0x14198a4e0, "unknown_14198a4e0", fn_14198a4e0 as *const (), Throws::Discard), // sub_14198a4e0
    Replacement::draft_throws(0x14198aa40, "unknown_14198aa40", fn_14198aa40 as *const (), Throws::Caught), // sub_14198aa40
    Replacement::draft(0x14198afb0, "unknown_14198afb0", tpl_34443 as *const ()), // sub_14198afb0
    Replacement::draft(0x14198afc0, "unknown_14198afc0", fn_14198afc0 as *const ()), // sub_14198afc0
    Replacement::draft_throws(0x14198afe0, "unknown_14198afe0", fn_14198afe0 as *const (), Throws::Caught), // sub_14198afe0
    Replacement::draft_throws(0x14198c080, "unknown_14198c080", fn_14198c080 as *const (), Throws::Discard), // sub_14198c080
    Replacement::draft_throws(0x14198c4b0, "unknown_14198c4b0", fn_14198c4b0 as *const (), Throws::Discard), // sub_14198c4b0
    Replacement::draft_throws(0x14198c760, "unknown_14198c760", fn_14198c760 as *const (), Throws::Caught), // sub_14198c760
    Replacement::draft_throws(0x14198cb70, "unknown_14198cb70", fn_14198cb70 as *const (), Throws::Discard), // sub_14198cb70
    Replacement::draft_throws(0x14198cc40, "unknown_14198cc40", fn_14198cc40 as *const (), Throws::Caught), // sub_14198cc40
    Replacement::draft_throws(0x14198cdd0, "unknown_14198cdd0", fn_14198cdd0 as *const (), Throws::Caught), // sub_14198cdd0
    Replacement::draft_throws(0x14198cf80, "unknown_14198cf80", fn_14198cf80 as *const (), Throws::Discard), // sub_14198cf80
    Replacement::draft_throws(0x14198d2b0, "unknown_14198d2b0", fn_14198d2b0 as *const (), Throws::Discard), // sub_14198d2b0
    Replacement::draft_throws(0x14198d9f0, "unknown_14198d9f0", fn_14198d9f0 as *const (), Throws::Discard), // sub_14198d9f0
    Replacement::draft_throws(0x14198daf0, "unknown_14198daf0", tpl_3880::<0x78, 0x143c9d1c0, 0x580e0, 0x143110698, 0x142116b30, 0x14198db26> as *const (), Throws::Discard), // sub_14198daf0
    Replacement::draft_throws(0x14198db30, "unknown_14198db30", tpl_3880::<0x78, 0x143c9f26c, 0x580e0, 0x143110698, 0x142116b30, 0x14198db66> as *const (), Throws::Discard), // sub_14198db30
    Replacement::draft_throws(0x14198db70, "unknown_14198db70", tpl_3880::<0x78, 0x143c9f1c4, 0x580e0, 0x143110698, 0x142116b30, 0x14198dba6> as *const (), Throws::Discard), // sub_14198db70
    Replacement::draft_throws(0x14198dbb0, "unknown_14198dbb0", tpl_3880::<0x78, 0x143c8d118, 0x580e0, 0x143110698, 0x142116b30, 0x14198dbe6> as *const (), Throws::Discard), // sub_14198dbb0
    Replacement::draft_throws(0x14198dbf0, "unknown_14198dbf0", tpl_3880::<0x78, 0x143c8d110, 0x580e0, 0x143110698, 0x142116b30, 0x14198dc26> as *const (), Throws::Discard), // sub_14198dbf0
    Replacement::draft(0x14198dc30, "unknown_14198dc30", tpl_361 as *const ()), // sub_14198dc30
    Replacement::draft(0x14198dc40, "unknown_14198dc40", fn_14198dc40 as *const ()), // sub_14198dc40
    Replacement::draft_throws(0x14198de90, "unknown_14198de90", tpl_35913::<0x1403fb770, 0x10, 0x8, 0x1429868d0, 0x9b, 0xffffffffffffffff, 0x1421158a0, 0x14198df38> as *const (), Throws::Discard), // sub_14198de90
    Replacement::draft_throws(0x14198df40, "unknown_14198df40", tpl_35913::<0x1403fb770, 0x10, 0x8, 0x1429868d0, 0x9c, 0xffffffffffffffff, 0x1421158a0, 0x14198dfe8> as *const (), Throws::Discard), // sub_14198df40
    Replacement::draft_throws(0x14198dff0, "unknown_14198dff0", fn_14198dff0 as *const (), Throws::Discard), // sub_14198dff0
    Replacement::draft_throws(0x14198e110, "unknown_14198e110", tpl_35913::<0x1403fb770, 0x10, 0x8, 0x1429868d0, 0xc7, 0xffffffffffffffff, 0x1421158a0, 0x14198e1b8> as *const (), Throws::Discard), // sub_14198e110
    Replacement::draft_throws(0x14198e1c0, "unknown_14198e1c0", fn_14198e1c0 as *const (), Throws::Discard), // sub_14198e1c0
    Replacement::draft_throws(0x14198e2e0, "unknown_14198e2e0", tpl_35915::<0x1403fb770, 0x10, 0x8, 0x1429868d0, 0x4, 0x88, 0x90, 0x8c, 0xc, 0x2, 0x1421158a0, 0x14198e3a1> as *const (), Throws::Discard), // sub_14198e2e0
    Replacement::draft_throws(0x14198e3b0, "unknown_14198e3b0", tpl_35915::<0x1403fb770, 0x10, 0x8, 0x1429868d0, 0x4, 0x70, 0x78, 0x74, 0xc, 0x2, 0x1421158a0, 0x14198e462> as *const (), Throws::Discard), // sub_14198e3b0
    Replacement::draft_throws(0x14198e470, "unknown_14198e470", tpl_35913::<0x1403fb770, 0x10, 0x8, 0x1429868d0, 0x99, 0xffffffffffffffff, 0x1421158a0, 0x14198e518> as *const (), Throws::Discard), // sub_14198e470
    Replacement::draft_throws(0x14198e520, "unknown_14198e520", fn_14198e520 as *const (), Throws::Discard), // sub_14198e520
    Replacement::draft_throws(0x14198e780, "unknown_14198e780", fn_14198e780 as *const (), Throws::Discard), // sub_14198e780
    Replacement::draft_throws(0x14198e970, "unknown_14198e970", fn_14198e970 as *const (), Throws::Discard), // sub_14198e970
    Replacement::draft_throws(0x14198ecf0, "unknown_14198ecf0", tpl_35913::<0x1403fb770, 0x10, 0x8, 0x1429868d0, 0x9a, 0xffffffffffffffff, 0x1421158a0, 0x14198ed98> as *const (), Throws::Discard), // sub_14198ecf0
    Replacement::draft_throws(0x14198eda0, "unknown_14198eda0", tpl_35913::<0x1403fb770, 0x10, 0x8, 0x1429868d0, 0x98, 0xffffffffffffffff, 0x1421158a0, 0x14198ee48> as *const (), Throws::Discard), // sub_14198eda0
    Replacement::draft_throws(0x14198ee50, "unknown_14198ee50", fn_14198ee50 as *const (), Throws::Discard), // sub_14198ee50
    Replacement::draft_throws(0x14198f090, "unknown_14198f090", fn_14198f090 as *const (), Throws::Discard), // sub_14198f090
    Replacement::draft_throws(0x14198f170, "unknown_14198f170", fn_14198f170 as *const (), Throws::Discard), // sub_14198f170
    Replacement::draft_throws(0x14198f2c0, "unknown_14198f2c0", fn_14198f2c0 as *const (), Throws::Discard), // sub_14198f2c0
    Replacement::draft_throws(0x14198f410, "unknown_14198f410", tpl_35880::<0x78, 0x143c8d0d4, 0x580e0, 0x10, 0x8, 0x1429868d0, 0xb5, 0x1421158a0, 0x143110698, 0x142116b30, 0x14198f4c2, 0x14198f4cf> as *const (), Throws::Discard), // sub_14198f410
    Replacement::draft_throws(0x14198f4e0, "unknown_14198f4e0", fn_14198f4e0 as *const (), Throws::Caught), // sub_14198f4e0
    Replacement::draft_throws(0x14198f830, "unknown_14198f830", fn_14198f830 as *const (), Throws::Discard), // sub_14198f830
    Replacement::draft_throws(0x14198f910, "unknown_14198f910", fn_14198f910 as *const (), Throws::Discard), // sub_14198f910
    Replacement::draft_throws(0x14198f960, "unknown_14198f960", tpl_35925::<0x78, 0x143c8d0d4, 0x580e0, 0x1e8, 0x8, 0x10, 0x143110698, 0x142116b30, 0x14198f9f3> as *const (), Throws::Discard), // sub_14198f960
    Replacement::draft_throws(0x14198fa00, "unknown_14198fa00", tpl_35925::<0x78, 0x143c8d0d4, 0x580e0, 0x1e0, 0x8, 0x10, 0x143110698, 0x142116b30, 0x14198fa93> as *const (), Throws::Discard), // sub_14198fa00
    Replacement::draft_throws(0x14198faa0, "unknown_14198faa0", tpl_35925::<0x78, 0x143c8d0d4, 0x580e0, 0x1d8, 0x8, 0x10, 0x143110698, 0x142116b30, 0x14198fb33> as *const (), Throws::Discard), // sub_14198faa0
    Replacement::draft_throws(0x14198fb40, "unknown_14198fb40", tpl_35926::<0x78, 0x143c8d0d4, 0x580e0, 0x8, 0x64, 0x10, 0x143110698, 0x142116b30, 0x14198fb99> as *const (), Throws::Discard), // sub_14198fb40
    Replacement::draft_throws(0x14198fba0, "unknown_14198fba0", fn_14198fba0 as *const (), Throws::Discard), // sub_14198fba0
    Replacement::draft_throws(0x14198fc00, "unknown_14198fc00", tpl_35928::<0x78, 0x143c8d0d4, 0x580e0, 0x70, 0x6c, 0x68, 0x8, 0x4, 0xc, 0x10, 0x143110698, 0x142116b30, 0x14198fc6e> as *const (), Throws::Discard), // sub_14198fc00
    Replacement::draft_throws(0x14198fc80, "unknown_14198fc80", tpl_35928::<0x78, 0x143c8d0d4, 0x580e0, 0x24, 0x20, 0x1c, 0x8, 0x4, 0xc, 0x10, 0x143110698, 0x142116b30, 0x14198fcee> as *const (), Throws::Discard), // sub_14198fc80
    Replacement::draft_throws(0x14198fd00, "unknown_14198fd00", fn_14198fd00 as *const (), Throws::Discard), // sub_14198fd00
    Replacement::draft_throws(0x14198feb0, "unknown_14198feb0", fn_14198feb0 as *const (), Throws::Discard), // sub_14198feb0
    Replacement::draft_throws(0x141990100, "unknown_141990100", fn_141990100 as *const (), Throws::Caught), // sub_141990100
    Replacement::draft_throws(0x141990ce0, "unknown_141990ce0", fn_141990ce0 as *const (), Throws::Discard), // sub_141990ce0
    Replacement::draft_throws(0x141990f10, "unknown_141990f10", fn_141990f10 as *const (), Throws::Discard), // sub_141990f10
    Replacement::draft_throws(0x141991150, "unknown_141991150", fn_141991150 as *const (), Throws::Discard), // sub_141991150
    Replacement::draft_throws(0x141991180, "unknown_141991180", fn_141991180 as *const (), Throws::Discard), // sub_141991180
    Replacement::draft_throws(0x1419914d0, "unknown_1419914d0", fn_1419914d0 as *const (), Throws::Discard), // sub_1419914d0
    Replacement::draft_throws(0x141991730, "unknown_141991730", fn_141991730 as *const (), Throws::Discard), // sub_141991730
    Replacement::draft_throws(0x1419919d0, "unknown_1419919d0", fn_1419919d0 as *const (), Throws::Discard), // sub_1419919d0
    Replacement::draft_throws(0x141991d20, "unknown_141991d20", fn_141991d20 as *const (), Throws::Discard), // sub_141991d20
    Replacement::draft(0x1419920c0, "unknown_1419920c0", fn_1419920c0 as *const ()), // sub_1419920c0
    Replacement::draft_throws(0x141992100, "unknown_141992100", fn_141992100 as *const (), Throws::Discard), // sub_141992100
    Replacement::draft(0x141992380, "unknown_141992380", fn_141992380 as *const ()), // sub_141992380
    Replacement::draft_throws(0x141992450, "unknown_141992450", fn_141992450 as *const (), Throws::Discard), // sub_141992450
    Replacement::draft_throws(0x141992930, "unknown_141992930", fn_141992930 as *const (), Throws::Discard), // sub_141992930
    Replacement::draft_throws(0x1419929a0, "unknown_1419929a0", fn_1419929a0 as *const (), Throws::Discard), // sub_1419929a0
    Replacement::draft_throws(0x141992a60, "unknown_141992a60", fn_141992a60 as *const (), Throws::Discard), // sub_141992a60
    Replacement::draft_throws(0x141992aa0, "unknown_141992aa0", fn_141992aa0 as *const (), Throws::Discard), // sub_141992aa0
    Replacement::draft_throws(0x141992b80, "unknown_141992b80", fn_141992b80 as *const (), Throws::Discard), // sub_141992b80
    Replacement::draft_throws(0x141992c50, "unknown_141992c50", fn_141992c50 as *const (), Throws::Discard), // sub_141992c50
    Replacement::draft_throws(0x141992d90, "unknown_141992d90", fn_141992d90 as *const (), Throws::Discard), // sub_141992d90
    Replacement::draft_throws(0x141992e90, "unknown_141992e90", fn_141992e90 as *const (), Throws::Discard), // sub_141992e90
    Replacement::draft_throws(0x141993400, "unknown_141993400", fn_141993400 as *const (), Throws::Discard), // sub_141993400
    Replacement::draft_throws(0x141993570, "unknown_141993570", fn_141993570 as *const (), Throws::Discard), // sub_141993570
    Replacement::draft_throws(0x1419937e0, "unknown_1419937e0", fn_1419937e0 as *const (), Throws::Discard), // sub_1419937e0
    Replacement::draft_throws(0x141993a20, "unknown_141993a20", fn_141993a20 as *const (), Throws::Caught), // sub_141993a20
    Replacement::draft_throws(0x141993b60, "unknown_141993b60", fn_141993b60 as *const (), Throws::Caught), // sub_141993b60
    Replacement::draft_throws(0x141993dc0, "unknown_141993dc0", fn_141993dc0 as *const (), Throws::Caught), // sub_141993dc0
    Replacement::draft_throws(0x141993f00, "unknown_141993f00", tpl_35948::<0x1403fafe0, 0x10, 0x8, 0x1429868d0, 0xffffffffffffffff, 0x1421158a0, 0x141993f6b> as *const (), Throws::Discard), // sub_141993f00
    Replacement::draft_throws(0x141993fa0, "unknown_141993fa0", fn_141993fa0 as *const (), Throws::Discard), // sub_141993fa0
    Replacement::draft_throws(0x141994190, "unknown_141994190", fn_141994190 as *const (), Throws::Caught), // sub_141994190
    Replacement::draft_throws(0x1419946e0, "unknown_1419946e0", fn_1419946e0 as *const (), Throws::Discard), // sub_1419946e0
    Replacement::draft_throws(0x141994720, "unknown_141994720", fn_141994720 as *const (), Throws::Discard), // sub_141994720
    Replacement::draft_throws(0x141994740, "unknown_141994740", fn_141994740 as *const (), Throws::Discard), // sub_141994740
    Replacement::draft_throws(0x141994780, "unknown_141994780", fn_141994780 as *const (), Throws::Discard), // sub_141994780
    Replacement::draft_throws(0x141994b10, "unknown_141994b10", fn_141994b10 as *const (), Throws::Caught), // sub_141994b10
    Replacement::draft_throws(0x141994f00, "unknown_141994f00", fn_141994f00 as *const (), Throws::Discard), // sub_141994f00
    Replacement::draft_throws(0x141994f70, "unknown_141994f70", fn_141994f70 as *const (), Throws::Caught), // sub_141994f70
    Replacement::draft_throws(0x141995180, "unknown_141995180", fn_141995180 as *const (), Throws::Discard), // sub_141995180
    Replacement::draft_throws(0x141995370, "unknown_141995370", fn_141995370 as *const (), Throws::Discard), // sub_141995370
    Replacement::draft_throws(0x141995490, "unknown_141995490", fn_141995490 as *const (), Throws::Discard), // sub_141995490
    Replacement::draft_throws(0x1419955b0, "unknown_1419955b0", fn_1419955b0 as *const (), Throws::Discard), // sub_1419955b0
    Replacement::draft_throws(0x1419956c0, "unknown_1419956c0", fn_1419956c0 as *const (), Throws::Discard), // sub_1419956c0
    Replacement::draft_throws(0x1419957e0, "unknown_1419957e0", fn_1419957e0 as *const (), Throws::Discard), // sub_1419957e0
    Replacement::draft_throws(0x141995900, "unknown_141995900", fn_141995900 as *const (), Throws::Discard), // sub_141995900
    Replacement::draft_throws(0x141995950, "unknown_141995950", fn_141995950 as *const (), Throws::Discard), // sub_141995950
    Replacement::draft_throws(0x141995d90, "unknown_141995d90", fn_141995d90 as *const (), Throws::Discard), // sub_141995d90
    Replacement::draft_throws(0x141995f70, "unknown_141995f70", fn_141995f70 as *const (), Throws::Discard), // sub_141995f70
    Replacement::draft_throws(0x141996310, "unknown_141996310", fn_141996310 as *const (), Throws::Discard), // sub_141996310
    Replacement::draft_throws(0x141996400, "unknown_141996400", fn_141996400 as *const (), Throws::Discard), // sub_141996400
    Replacement::draft_throws(0x141996570, "unknown_141996570", fn_141996570 as *const (), Throws::Caught), // sub_141996570
    Replacement::draft_throws(0x141996810, "unknown_141996810", fn_141996810 as *const (), Throws::Caught), // sub_141996810
    Replacement::draft_throws(0x141996950, "unknown_141996950", fn_141996950 as *const (), Throws::Discard), // sub_141996950
    Replacement::draft_throws(0x141996a70, "unknown_141996a70", fn_141996a70 as *const (), Throws::Discard), // sub_141996a70
    Replacement::draft_throws(0x141996bb0, "unknown_141996bb0", fn_141996bb0 as *const (), Throws::Discard), // sub_141996bb0
    Replacement::draft_throws(0x141996d30, "unknown_141996d30", fn_141996d30 as *const (), Throws::Discard), // sub_141996d30
    Replacement::draft_throws(0x141996e80, "unknown_141996e80", fn_141996e80 as *const (), Throws::Caught), // sub_141996e80
    Replacement::draft_throws(0x141997210, "unknown_141997210", fn_141997210 as *const (), Throws::Discard), // sub_141997210
    Replacement::draft_throws(0x141997400, "unknown_141997400", fn_141997400 as *const (), Throws::Discard), // sub_141997400
    Replacement::draft_throws(0x1419975c0, "unknown_1419975c0", fn_1419975c0 as *const (), Throws::Caught), // sub_1419975c0
    Replacement::draft_throws(0x1419976f0, "unknown_1419976f0", fn_1419976f0 as *const (), Throws::Discard), // sub_1419976f0
    Replacement::draft_throws(0x141997920, "unknown_141997920", tpl_35972::<0x1429868d0, 0x140403bb0, 0x1420ecd00, 0x141a760c0, 0x1421158a0, 0x14310a9a8, 0x142116bc0, 0x141997a02, 0x141997a14, 0x141997a29> as *const (), Throws::Discard), // sub_141997920
    Replacement::draft_throws(0x141997a30, "unknown_141997a30", fn_141997a30 as *const (), Throws::Discard), // sub_141997a30
    Replacement::draft_throws(0x141997cb0, "unknown_141997cb0", fn_141997cb0 as *const (), Throws::Discard), // sub_141997cb0
    Replacement::draft_throws(0x141997d80, "unknown_141997d80", fn_141997d80 as *const (), Throws::Discard), // sub_141997d80
    Replacement::draft_throws(0x141997e70, "unknown_141997e70", fn_141997e70 as *const (), Throws::Discard), // sub_141997e70
    Replacement::draft_throws(0x1419980c0, "unknown_1419980c0", tpl_35972::<0x1429868d0, 0x140403bb0, 0x1420ecd00, 0x141a76400, 0x1421158a0, 0x14310a9a8, 0x142116bc0, 0x1419981a2, 0x1419981b4, 0x1419981c9> as *const (), Throws::Discard), // sub_1419980c0
    Replacement::draft_throws(0x1419981e0, "unknown_1419981e0", fn_1419981e0 as *const (), Throws::Discard), // sub_1419981e0
    Replacement::draft_throws(0x141998430, "unknown_141998430", tpl_35976::<0x10, 0x1429868d0, 0x8, 0x78, 0x1404039a0, 0x70, 0x143426514, 0x14310a9a8, 0x142116bc0, 0x1, 0x1421158a0, 0x1419984e4, 0x1419984f6> as *const (), Throws::Discard), // sub_141998430
    Replacement::draft_throws(0x141998510, "unknown_141998510", tpl_35976::<0x10, 0x1429868d0, 0x8, 0x78, 0x1404039a0, 0x6c, 0x143426514, 0x14310a9a8, 0x142116bc0, 0x1, 0x1421158a0, 0x1419985c4, 0x1419985d6> as *const (), Throws::Discard), // sub_141998510
    Replacement::draft_throws(0x1419985f0, "unknown_1419985f0", fn_1419985f0 as *const (), Throws::Discard), // sub_1419985f0
    Replacement::draft_throws(0x1419988c0, "unknown_1419988c0", fn_1419988c0 as *const (), Throws::Discard), // sub_1419988c0
    Replacement::draft_throws(0x141998990, "unknown_141998990", fn_141998990 as *const (), Throws::Discard), // sub_141998990
    Replacement::draft_throws(0x141998ac0, "unknown_141998ac0", fn_141998ac0 as *const (), Throws::Discard), // sub_141998ac0
    Replacement::draft_throws(0x141998b40, "unknown_141998b40", fn_141998b40 as *const (), Throws::Discard), // sub_141998b40
    Replacement::draft_throws(0x141998d10, "unknown_141998d10", fn_141998d10 as *const (), Throws::Discard), // sub_141998d10
    Replacement::draft_throws(0x141998df0, "unknown_141998df0", fn_141998df0 as *const (), Throws::Discard), // sub_141998df0
    Replacement::draft_throws(0x141998fb0, "unknown_141998fb0", fn_141998fb0 as *const (), Throws::Discard), // sub_141998fb0
    Replacement::draft_throws(0x1419991c0, "unknown_1419991c0", fn_1419991c0 as *const (), Throws::Discard), // sub_1419991c0
    Replacement::draft_throws(0x1419993d0, "unknown_1419993d0", fn_1419993d0 as *const (), Throws::Discard), // sub_1419993d0
    Replacement::draft_throws(0x141999680, "unknown_141999680", fn_141999680 as *const (), Throws::Discard), // sub_141999680
    Replacement::draft_throws(0x141999890, "unknown_141999890", fn_141999890 as *const (), Throws::Discard), // sub_141999890
    Replacement::draft(0x141999990, "unknown_141999990", fn_141999990 as *const ()), // sub_141999990
    Replacement::draft_throws(0x141999b70, "unknown_141999b70", fn_141999b70 as *const (), Throws::Discard), // sub_141999b70
    Replacement::draft_throws(0x141999e70, "unknown_141999e70", fn_141999e70 as *const (), Throws::Discard), // sub_141999e70
    Replacement::draft_throws(0x14199a250, "unknown_14199a250", fn_14199a250 as *const (), Throws::Discard), // sub_14199a250
    Replacement::draft_throws(0x14199a4e0, "unknown_14199a4e0", fn_14199a4e0 as *const (), Throws::Discard), // sub_14199a4e0
    Replacement::draft_throws(0x14199a630, "unknown_14199a630", fn_14199a630 as *const (), Throws::Caught), // sub_14199a630
    Replacement::draft_throws(0x14199aaf0, "unknown_14199aaf0", fn_14199aaf0 as *const (), Throws::Discard), // sub_14199aaf0
    Replacement::draft_throws(0x14199ab90, "unknown_14199ab90", fn_14199ab90 as *const (), Throws::Discard), // sub_14199ab90
    Replacement::draft_throws(0x14199afd0, "unknown_14199afd0", fn_14199afd0 as *const (), Throws::Discard), // sub_14199afd0
    Replacement::draft_throws(0x14199b050, "unknown_14199b050", fn_14199b050 as *const (), Throws::Discard), // sub_14199b050
    Replacement::draft_throws(0x14199b160, "unknown_14199b160", fn_14199b160 as *const (), Throws::Discard), // sub_14199b160
    Replacement::draft_throws(0x14199b240, "unknown_14199b240", fn_14199b240 as *const (), Throws::Discard), // sub_14199b240
    Replacement::draft_throws(0x14199b670, "unknown_14199b670", fn_14199b670 as *const (), Throws::Discard), // sub_14199b670
    Replacement::draft_throws(0x14199b7c0, "unknown_14199b7c0", fn_14199b7c0 as *const (), Throws::Discard), // sub_14199b7c0
    Replacement::draft_throws(0x14199b900, "unknown_14199b900", fn_14199b900 as *const (), Throws::Caught), // sub_14199b900
    Replacement::draft_throws(0x14199c0d0, "unknown_14199c0d0", fn_14199c0d0 as *const (), Throws::Caught), // sub_14199c0d0
    Replacement::draft_throws(0x14199c9c0, "unknown_14199c9c0", fn_14199c9c0 as *const (), Throws::Caught), // sub_14199c9c0
    Replacement::draft_throws(0x14199d140, "unknown_14199d140", fn_14199d140 as *const (), Throws::Caught), // sub_14199d140
    Replacement::draft_throws(0x14199d9e0, "unknown_14199d9e0", fn_14199d9e0 as *const (), Throws::Caught), // sub_14199d9e0
    Replacement::draft_throws(0x14199e2a0, "unknown_14199e2a0", fn_14199e2a0 as *const (), Throws::Caught), // sub_14199e2a0
    Replacement::draft_throws(0x14199ec90, "unknown_14199ec90", fn_14199ec90 as *const (), Throws::Discard), // sub_14199ec90
    Replacement::draft_throws(0x14199ecb0, "unknown_14199ecb0", fn_14199ecb0 as *const (), Throws::Discard), // sub_14199ecb0
    Replacement::draft_throws(0x14199ece0, "unknown_14199ece0", fn_14199ece0 as *const (), Throws::Discard), // sub_14199ece0
    Replacement::draft_throws(0x14199efa0, "unknown_14199efa0", fn_14199efa0 as *const (), Throws::Discard), // sub_14199efa0
    Replacement::draft_throws(0x14199f0f0, "unknown_14199f0f0", tpl_35874::<0x1403fa6a0, 0x8, 0x4, 0x10> as *const (), Throws::Discard), // sub_14199f0f0
    Replacement::draft_throws(0x14199f130, "unknown_14199f130", fn_14199f130 as *const (), Throws::Discard), // sub_14199f130
    Replacement::draft_throws(0x14199f1e0, "unknown_14199f1e0", fn_14199f1e0 as *const (), Throws::Discard), // sub_14199f1e0
    Replacement::draft(0x14199f380, "unknown_14199f380", fn_14199f380 as *const ()), // sub_14199f380
    Replacement::draft_throws(0x14199f520, "unknown_14199f520", fn_14199f520 as *const (), Throws::Discard), // sub_14199f520
    Replacement::draft_throws(0x14199f5c0, "unknown_14199f5c0", tpl_35998::<0x1403fa380, 0x3> as *const (), Throws::Discard), // sub_14199f5c0
    Replacement::draft_throws(0x14199f5e0, "unknown_14199f5e0", fn_14199f5e0 as *const (), Throws::Discard), // sub_14199f5e0
    Replacement::draft_throws(0x14199f620, "unknown_14199f620", fn_14199f620 as *const (), Throws::Discard), // sub_14199f620
    Replacement::draft_throws(0x14199f650, "unknown_14199f650", tpl_35913::<0x1403fa380, 0x10, 0x8, 0x1429868d0, 0x2, 0xffffffffffffffff, 0x1421158a0, 0x14199f6ef> as *const (), Throws::Discard), // sub_14199f650
    Replacement::draft_throws(0x14199f700, "unknown_14199f700", tpl_36001::<0x143109fa8, 0x1403fa1f0, 0x10, 0x8, 0x1429868d0, 0x2, 0xffffffffffffffff, 0x1421158a0, 0x14199f7a6> as *const (), Throws::Discard), // sub_14199f700
    Replacement::draft_throws(0x14199f7b0, "unknown_14199f7b0", tpl_36001::<0x143109fa8, 0x1403fa1f0, 0x10, 0x8, 0x1429868d0, 0x1, 0xffffffffffffffff, 0x1421158a0, 0x14199f856> as *const (), Throws::Discard), // sub_14199f7b0
    Replacement::draft_throws(0x14199f860, "unknown_14199f860", fn_14199f860 as *const (), Throws::Discard), // sub_14199f860
    Replacement::draft_throws(0x14199f8d0, "unknown_14199f8d0", fn_14199f8d0 as *const (), Throws::Discard), // sub_14199f8d0
    Replacement::draft_throws(0x14199f950, "unknown_14199f950", fn_14199f950 as *const (), Throws::Discard), // sub_14199f950
    Replacement::draft_throws(0x14199faf0, "unknown_14199faf0", fn_14199faf0 as *const (), Throws::Discard), // sub_14199faf0
    Replacement::draft_throws(0x14199fc80, "unknown_14199fc80", fn_14199fc80 as *const (), Throws::Discard), // sub_14199fc80
    Replacement::draft_throws(0x14199fd80, "unknown_14199fd80", fn_14199fd80 as *const (), Throws::Discard), // sub_14199fd80
    Replacement::draft(0x14199fdc0, "unknown_14199fdc0", fn_14199fdc0 as *const ()), // sub_14199fdc0
    Replacement::draft(0x14199fe10, "unknown_14199fe10", fn_14199fe10 as *const ()), // sub_14199fe10
    Replacement::draft_throws(0x14199fe40, "unknown_14199fe40", fn_14199fe40 as *const (), Throws::Discard), // sub_14199fe40
    Replacement::draft_throws(0x14199ff10, "unknown_14199ff10", fn_14199ff10 as *const (), Throws::Discard), // sub_14199ff10
    Replacement::draft_throws(0x1419a00a0, "unknown_1419a00a0", fn_1419a00a0 as *const (), Throws::Caught), // sub_1419a00a0
    Replacement::draft_throws(0x1419a0350, "unknown_1419a0350", fn_1419a0350 as *const (), Throws::Discard), // sub_1419a0350
    Replacement::draft_throws(0x1419a0390, "unknown_1419a0390", fn_1419a0390 as *const (), Throws::Discard), // sub_1419a0390
    Replacement::draft(0x1419a0420, "unknown_1419a0420", tpl_36013::<0x78, 0x143c9dcc8, 0x580e0> as *const ()), // sub_1419a0420
    Replacement::draft_throws(0x1419a0470, "unknown_1419a0470", fn_1419a0470 as *const (), Throws::Discard), // sub_1419a0470
    Replacement::draft_throws(0x1419a0560, "unknown_1419a0560", fn_1419a0560 as *const (), Throws::Discard), // sub_1419a0560
    Replacement::draft_throws(0x1419a05e0, "unknown_1419a05e0", fn_1419a05e0 as *const (), Throws::Discard), // sub_1419a05e0
    Replacement::draft_throws(0x1419a0670, "unknown_1419a0670", fn_1419a0670 as *const (), Throws::Discard), // sub_1419a0670
    Replacement::draft_throws(0x1419a0870, "unknown_1419a0870", fn_1419a0870 as *const (), Throws::Discard), // sub_1419a0870
    Replacement::draft_throws(0x1419a09c0, "unknown_1419a09c0", fn_1419a09c0 as *const (), Throws::Discard), // sub_1419a09c0
    Replacement::draft_throws(0x1419a0bb0, "unknown_1419a0bb0", fn_1419a0bb0 as *const (), Throws::Discard), // sub_1419a0bb0
    Replacement::draft_throws(0x1419a0cd0, "unknown_1419a0cd0", fn_1419a0cd0 as *const (), Throws::Discard), // sub_1419a0cd0
    Replacement::draft_throws(0x1419a0e40, "unknown_1419a0e40", fn_1419a0e40 as *const (), Throws::Discard), // sub_1419a0e40
    Replacement::draft(0x1419a0f30, "unknown_1419a0f30", fn_1419a0f30 as *const ()), // sub_1419a0f30
    Replacement::draft_throws(0x1419a0f60, "unknown_1419a0f60", fn_1419a0f60 as *const (), Throws::Discard), // sub_1419a0f60
    Replacement::draft_throws(0x1419a1260, "unknown_1419a1260", fn_1419a1260 as *const (), Throws::Discard), // sub_1419a1260
    Replacement::draft_throws(0x1419a1550, "unknown_1419a1550", fn_1419a1550 as *const (), Throws::Discard), // sub_1419a1550
    Replacement::draft_throws(0x1419a16a0, "unknown_1419a16a0", fn_1419a16a0 as *const (), Throws::Discard), // sub_1419a16a0
    Replacement::draft_throws(0x1419a18d0, "unknown_1419a18d0", fn_1419a18d0 as *const (), Throws::Discard), // sub_1419a18d0
    Replacement::draft_throws(0x1419a1a20, "unknown_1419a1a20", fn_1419a1a20 as *const (), Throws::Discard), // sub_1419a1a20
    Replacement::draft_throws(0x1419a1c50, "unknown_1419a1c50", fn_1419a1c50 as *const (), Throws::Discard), // sub_1419a1c50
    Replacement::draft_throws(0x1419a1f10, "unknown_1419a1f10", fn_1419a1f10 as *const (), Throws::Caught), // sub_1419a1f10
    Replacement::draft_throws(0x1419a2320, "unknown_1419a2320", fn_1419a2320 as *const (), Throws::Discard), // sub_1419a2320
    Replacement::draft_throws(0x1419a2480, "unknown_1419a2480", fn_1419a2480 as *const (), Throws::Discard), // sub_1419a2480
    Replacement::draft_throws(0x1419a2640, "unknown_1419a2640", fn_1419a2640 as *const (), Throws::Discard), // sub_1419a2640
    Replacement::draft_throws(0x1419a2840, "unknown_1419a2840", fn_1419a2840 as *const (), Throws::Caught), // sub_1419a2840
    Replacement::draft_throws(0x1419a2c00, "unknown_1419a2c00", fn_1419a2c00 as *const (), Throws::Discard), // sub_1419a2c00
    Replacement::draft_throws(0x1419a2db0, "unknown_1419a2db0", fn_1419a2db0 as *const (), Throws::Discard), // sub_1419a2db0
    Replacement::draft_throws(0x1419a2e40, "unknown_1419a2e40", fn_1419a2e40 as *const (), Throws::Discard), // sub_1419a2e40
    Replacement::draft_throws(0x1419a2ef0, "unknown_1419a2ef0", fn_1419a2ef0 as *const (), Throws::Discard), // sub_1419a2ef0
    Replacement::draft_throws(0x1419a2fa0, "unknown_1419a2fa0", fn_1419a2fa0 as *const (), Throws::Discard), // sub_1419a2fa0
    Replacement::draft_throws(0x1419a3110, "unknown_1419a3110", fn_1419a3110 as *const (), Throws::Discard), // sub_1419a3110
    Replacement::draft_throws(0x1419a3200, "unknown_1419a3200", fn_1419a3200 as *const (), Throws::Discard), // sub_1419a3200
    Replacement::draft_throws(0x1419a3250, "unknown_1419a3250", fn_1419a3250 as *const (), Throws::Discard), // sub_1419a3250
    Replacement::draft_throws(0x1419a33c0, "unknown_1419a33c0", fn_1419a33c0 as *const (), Throws::Discard), // sub_1419a33c0
    Replacement::draft_throws(0x1419a35c0, "unknown_1419a35c0", fn_1419a35c0 as *const (), Throws::Discard), // sub_1419a35c0
    Replacement::draft_throws(0x1419a3770, "unknown_1419a3770", fn_1419a3770 as *const (), Throws::Discard), // sub_1419a3770
    Replacement::draft(0x1419a3900, "unknown_1419a3900", fn_1419a3900 as *const ()), // sub_1419a3900
    Replacement::draft_throws(0x1419a3950, "unknown_1419a3950", fn_1419a3950 as *const (), Throws::Discard), // sub_1419a3950
    Replacement::draft_throws(0x1419a3b00, "unknown_1419a3b00", fn_1419a3b00 as *const (), Throws::Discard), // sub_1419a3b00
    Replacement::draft_throws(0x1419a3c00, "unknown_1419a3c00", fn_1419a3c00 as *const (), Throws::Discard), // sub_1419a3c00
    Replacement::draft_throws(0x1419a3cd0, "unknown_1419a3cd0", fn_1419a3cd0 as *const (), Throws::Discard), // sub_1419a3cd0
    Replacement::draft_throws(0x1419a3dd0, "unknown_1419a3dd0", fn_1419a3dd0 as *const (), Throws::Discard), // sub_1419a3dd0
    Replacement::draft_throws(0x1419a3f80, "unknown_1419a3f80", fn_1419a3f80 as *const (), Throws::Caught), // sub_1419a3f80
    Replacement::draft_throws(0x1419a4e80, "unknown_1419a4e80", fn_1419a4e80 as *const (), Throws::Discard), // sub_1419a4e80
    Replacement::draft_throws(0x1419a5ea0, "unknown_1419a5ea0", fn_1419a5ea0 as *const (), Throws::Discard), // sub_1419a5ea0
    Replacement::draft(0x1419a5f80, "unknown_1419a5f80", tpl_34044 as *const ()), // sub_1419a5f80
    Replacement::draft(0x1419a5fa0, "unknown_1419a5fa0", fn_1419a5fa0 as *const ()), // sub_1419a5fa0
    Replacement::draft_throws(0x1419a5fc0, "unknown_1419a5fc0", fn_1419a5fc0 as *const (), Throws::Discard), // sub_1419a5fc0
    Replacement::draft_throws(0x1419a6410, "unknown_1419a6410", fn_1419a6410 as *const (), Throws::Discard), // sub_1419a6410
    Replacement::draft(0x1419a70d0, "unknown_1419a70d0", fn_1419a70d0 as *const ()), // sub_1419a70d0
    Replacement::draft(0x1419a71d0, "unknown_1419a71d0", fn_1419a71d0 as *const ()), // sub_1419a71d0
    Replacement::draft_throws(0x1419a72c0, "unknown_1419a72c0", tpl_95879::<0x1429868d0, 0x1420fcf40, 0x143c9e064, 0x1401e7c10, 0x1403f7510, 0x1426d01e0, 0x1421158a0, 0x1419a759b> as *const (), Throws::Discard), // sub_1419a72c0
    Replacement::draft_throws(0x1419a75c0, "unknown_1419a75c0", tpl_95879::<0x1429868d0, 0x1420fcf40, 0x143c9f40c, 0x1401e7c10, 0x1403f7510, 0x1426d01e0, 0x1421158a0, 0x1419a789b> as *const (), Throws::Discard), // sub_1419a75c0
    Replacement::draft_throws(0x1419a78b0, "unknown_1419a78b0", fn_1419a78b0 as *const (), Throws::Discard), // sub_1419a78b0
    Replacement::draft_throws(0x1419a7900, "unknown_1419a7900", fn_1419a7900 as *const (), Throws::Discard), // sub_1419a7900
    Replacement::draft_throws(0x1419a7970, "unknown_1419a7970", fn_1419a7970 as *const (), Throws::Discard), // sub_1419a7970
    Replacement::draft_throws(0x1419a7c50, "unknown_1419a7c50", fn_1419a7c50 as *const (), Throws::Discard), // sub_1419a7c50
    Replacement::draft_throws(0x1419a7f40, "unknown_1419a7f40", fn_1419a7f40 as *const (), Throws::Discard), // sub_1419a7f40
    Replacement::draft_throws(0x1419a8060, "unknown_1419a8060", fn_1419a8060 as *const (), Throws::Discard), // sub_1419a8060
    Replacement::draft_throws(0x1419a82a0, "unknown_1419a82a0", fn_1419a82a0 as *const (), Throws::Discard), // sub_1419a82a0
    Replacement::draft(0x1419a8370, "unknown_1419a8370", tpl_36051::<0x78, 0x1404db9e0, 0x8, 0x10> as *const ()), // sub_1419a8370
    Replacement::draft(0x1419a8420, "unknown_1419a8420", fn_1419a8420 as *const ()), // sub_1419a8420
    Replacement::draft(0x1419a84d0, "unknown_1419a84d0", tpl_36051::<0x78, 0x1404db9e0, 0x8, 0x10> as *const ()), // sub_1419a84d0
    Replacement::draft_throws(0x1419a8580, "unknown_1419a8580", fn_1419a8580 as *const (), Throws::Caught), // sub_1419a8580
    Replacement::draft_throws(0x1419a8cb0, "unknown_1419a8cb0", fn_1419a8cb0 as *const (), Throws::Caught), // sub_1419a8cb0
    Replacement::draft_throws(0x1419a8d70, "unknown_1419a8d70", fn_1419a8d70 as *const (), Throws::Caught), // sub_1419a8d70
    Replacement::draft_throws(0x1419a8e50, "unknown_1419a8e50", fn_1419a8e50 as *const (), Throws::Caught), // sub_1419a8e50
    Replacement::draft(0x1419a90f0, "unknown_1419a90f0", fn_1419a90f0 as *const ()), // sub_1419a90f0
    Replacement::draft_throws(0x1419a9120, "unknown_1419a9120", fn_1419a9120 as *const (), Throws::Discard), // sub_1419a9120
    Replacement::draft_throws(0x1419a91b0, "unknown_1419a91b0", fn_1419a91b0 as *const (), Throws::Caught), // sub_1419a91b0
    Replacement::draft_throws(0x1419a9360, "unknown_1419a9360", fn_1419a9360 as *const (), Throws::Caught), // sub_1419a9360
    Replacement::draft_throws(0x1419a94a0, "unknown_1419a94a0", fn_1419a94a0 as *const (), Throws::Discard), // sub_1419a94a0
    Replacement::draft_throws(0x1419a96d0, "unknown_1419a96d0", fn_1419a96d0 as *const (), Throws::Caught), // sub_1419a96d0
    Replacement::draft_throws(0x1419a98e0, "unknown_1419a98e0", fn_1419a98e0 as *const (), Throws::Caught), // sub_1419a98e0
    Replacement::draft_throws(0x1419aa130, "unknown_1419aa130", fn_1419aa130 as *const (), Throws::Discard), // sub_1419aa130
    Replacement::draft_throws(0x1419aa1a0, "unknown_1419aa1a0", fn_1419aa1a0 as *const (), Throws::Discard), // sub_1419aa1a0
    Replacement::draft_throws(0x1419aa220, "unknown_1419aa220", fn_1419aa220 as *const (), Throws::Discard), // sub_1419aa220
    Replacement::draft_throws(0x1419aa320, "unknown_1419aa320", fn_1419aa320 as *const (), Throws::Discard), // sub_1419aa320
    Replacement::draft_throws(0x1419aa390, "unknown_1419aa390", fn_1419aa390 as *const (), Throws::Discard), // sub_1419aa390
    Replacement::draft(0x1419aa520, "unknown_1419aa520", tpl_36013::<0x78, 0x143c8d01c, 0x580e0> as *const ()), // sub_1419aa520
    Replacement::draft_throws(0x1419aa560, "unknown_1419aa560", fn_1419aa560 as *const (), Throws::Discard), // sub_1419aa560
    Replacement::draft_throws(0x1419aa5d0, "unknown_1419aa5d0", fn_1419aa5d0 as *const (), Throws::Discard), // sub_1419aa5d0
    Replacement::draft_throws(0x1419aa820, "unknown_1419aa820", fn_1419aa820 as *const (), Throws::Caught), // sub_1419aa820
    Replacement::draft_throws(0x1419aab70, "unknown_1419aab70", fn_1419aab70 as *const (), Throws::Discard), // sub_1419aab70
    Replacement::draft_throws(0x1419aabc0, "unknown_1419aabc0", tpl_35948::<0x1403f89c0, 0x10, 0x8, 0x1429868d0, 0xffffffffffffffff, 0x1421158a0, 0x1419aac2b> as *const (), Throws::Discard), // sub_1419aabc0
    Replacement::draft(0x1419aac40, "unknown_1419aac40", tpl_35839::<0x8, 0x10> as *const ()), // sub_1419aac40
    Replacement::draft_throws(0x1419aac70, "unknown_1419aac70", fn_1419aac70 as *const (), Throws::Discard), // sub_1419aac70
    Replacement::draft_throws(0x1419aae10, "unknown_1419aae10", fn_1419aae10 as *const (), Throws::Discard), // sub_1419aae10
    Replacement::draft_throws(0x1419aaf20, "unknown_1419aaf20", fn_1419aaf20 as *const (), Throws::Discard), // sub_1419aaf20
    Replacement::draft_throws(0x1419ab040, "unknown_1419ab040", fn_1419ab040 as *const (), Throws::Discard), // sub_1419ab040
    Replacement::draft_throws(0x1419ab150, "unknown_1419ab150", fn_1419ab150 as *const (), Throws::Discard), // sub_1419ab150
    Replacement::draft_throws(0x1419ab270, "unknown_1419ab270", fn_1419ab270 as *const (), Throws::Discard), // sub_1419ab270
    Replacement::draft_throws(0x1419ab310, "unknown_1419ab310", fn_1419ab310 as *const (), Throws::Discard), // sub_1419ab310
    Replacement::draft_throws(0x1419ab410, "unknown_1419ab410", fn_1419ab410 as *const (), Throws::Discard), // sub_1419ab410
    Replacement::draft_throws(0x1419ab530, "unknown_1419ab530", fn_1419ab530 as *const (), Throws::Discard), // sub_1419ab530
    Replacement::draft(0x1419ab600, "unknown_1419ab600", fn_1419ab600 as *const ()), // sub_1419ab600
    Replacement::draft_throws(0x1419ab630, "unknown_1419ab630", fn_1419ab630 as *const (), Throws::Discard), // sub_1419ab630
    Replacement::draft(0x1419ab9f0, "unknown_1419ab9f0", fn_1419ab9f0 as *const ()), // sub_1419ab9f0
    Replacement::draft_throws(0x1419aba50, "unknown_1419aba50", fn_1419aba50 as *const (), Throws::Discard), // sub_1419aba50
    Replacement::draft_throws(0x1419abc30, "unknown_1419abc30", fn_1419abc30 as *const (), Throws::Caught), // sub_1419abc30
    Replacement::draft_throws(0x1419abf50, "unknown_1419abf50", fn_1419abf50 as *const (), Throws::Caught), // sub_1419abf50
    Replacement::draft_throws(0x1419ac460, "unknown_1419ac460", fn_1419ac460 as *const (), Throws::Discard), // sub_1419ac460
    Replacement::draft_throws(0x1419ac5c0, "unknown_1419ac5c0", fn_1419ac5c0 as *const (), Throws::Discard), // sub_1419ac5c0
    Replacement::draft_throws(0x1419ac6c0, "unknown_1419ac6c0", fn_1419ac6c0 as *const (), Throws::Caught), // sub_1419ac6c0
    Replacement::draft_throws(0x1419ac7b0, "unknown_1419ac7b0", fn_1419ac7b0 as *const (), Throws::Discard), // sub_1419ac7b0
    Replacement::draft_throws(0x1419ac810, "unknown_1419ac810", fn_1419ac810 as *const (), Throws::Caught), // sub_1419ac810
    Replacement::draft_throws(0x1419ac8d0, "unknown_1419ac8d0", fn_1419ac8d0 as *const (), Throws::Discard), // sub_1419ac8d0
    Replacement::draft_throws(0x1419acb60, "unknown_1419acb60", fn_1419acb60 as *const (), Throws::Caught), // sub_1419acb60
    Replacement::draft_throws(0x1419ad280, "unknown_1419ad280", fn_1419ad280 as *const (), Throws::Discard), // sub_1419ad280
    Replacement::draft_throws(0x1419ad2d0, "unknown_1419ad2d0", tpl_36086::<0x78, 0x8, 0x143c901e8, 0x580e0, 0x10, 0x1429868d0, 0x1421158a0, 0x1419ad36a> as *const (), Throws::Discard), // sub_1419ad2d0
    Replacement::draft_throws(0x1419ad390, "unknown_1419ad390", fn_1419ad390 as *const (), Throws::Caught), // sub_1419ad390
    Replacement::draft_throws(0x1419ada60, "unknown_1419ada60", fn_1419ada60 as *const (), Throws::Discard), // sub_1419ada60
    Replacement::draft_throws(0x1419adc00, "unknown_1419adc00", fn_1419adc00 as *const (), Throws::Discard), // sub_1419adc00
    Replacement::draft_throws(0x1419add50, "unknown_1419add50", fn_1419add50 as *const (), Throws::Discard), // sub_1419add50
    Replacement::draft_throws(0x1419ae590, "unknown_1419ae590", fn_1419ae590 as *const (), Throws::Discard), // sub_1419ae590
    Replacement::draft_throws(0x1419ae670, "unknown_1419ae670", fn_1419ae670 as *const (), Throws::Discard), // sub_1419ae670
    Replacement::draft_throws(0x1419ae7a0, "unknown_1419ae7a0", fn_1419ae7a0 as *const (), Throws::Caught), // sub_1419ae7a0
    Replacement::draft_throws(0x1419af630, "unknown_1419af630", fn_1419af630 as *const (), Throws::Caught), // sub_1419af630
    Replacement::draft_throws(0x1419af8e0, "unknown_1419af8e0", fn_1419af8e0 as *const (), Throws::Caught), // sub_1419af8e0
    Replacement::draft_throws(0x1419afba0, "unknown_1419afba0", fn_1419afba0 as *const (), Throws::Caught), // sub_1419afba0
    Replacement::draft_throws(0x1419b0d60, "unknown_1419b0d60", fn_1419b0d60 as *const (), Throws::Caught), // sub_1419b0d60
    Replacement::draft_throws(0x1419b1960, "unknown_1419b1960", fn_1419b1960 as *const (), Throws::Discard), // sub_1419b1960
    Replacement::draft_throws(0x1419b1a10, "unknown_1419b1a10", fn_1419b1a10 as *const (), Throws::Discard), // sub_1419b1a10
    Replacement::draft_throws(0x1419b1c70, "unknown_1419b1c70", fn_1419b1c70 as *const (), Throws::Discard), // sub_1419b1c70
    Replacement::draft_throws(0x1419b1d50, "unknown_1419b1d50", fn_1419b1d50 as *const (), Throws::Discard), // sub_1419b1d50
    Replacement::draft_throws(0x1419b1e90, "unknown_1419b1e90", fn_1419b1e90 as *const (), Throws::Discard), // sub_1419b1e90
    Replacement::draft_throws(0x1419b1f50, "unknown_1419b1f50", fn_1419b1f50 as *const (), Throws::Discard), // sub_1419b1f50
    Replacement::draft_throws(0x1419b2140, "unknown_1419b2140", fn_1419b2140 as *const (), Throws::Discard), // sub_1419b2140
    Replacement::draft_throws(0x1419b2480, "unknown_1419b2480", tpl_36086::<0x78, 0x8, 0x143c8d17c, 0x580e0, 0x10, 0x1429868d0, 0x1421158a0, 0x1419b251a> as *const (), Throws::Discard), // sub_1419b2480
    Replacement::draft_throws(0x1419b2530, "unknown_1419b2530", fn_1419b2530 as *const (), Throws::Discard), // sub_1419b2530
    Replacement::draft_throws(0x1419b25b0, "unknown_1419b25b0", fn_1419b25b0 as *const (), Throws::Discard), // sub_1419b25b0
    Replacement::draft_throws(0x1419b2640, "unknown_1419b2640", fn_1419b2640 as *const (), Throws::Discard), // sub_1419b2640
    Replacement::draft_throws(0x1419b2730, "unknown_1419b2730", fn_1419b2730 as *const (), Throws::Discard), // sub_1419b2730
    Replacement::draft_throws(0x1419b2830, "unknown_1419b2830", fn_1419b2830 as *const (), Throws::Discard), // sub_1419b2830
    Replacement::draft_throws(0x1419b2920, "unknown_1419b2920", fn_1419b2920 as *const (), Throws::Discard), // sub_1419b2920
    Replacement::draft_throws(0x1419b2a10, "unknown_1419b2a10", fn_1419b2a10 as *const (), Throws::Discard), // sub_1419b2a10
    Replacement::draft_throws(0x1419b2b20, "unknown_1419b2b20", fn_1419b2b20 as *const (), Throws::Discard), // sub_1419b2b20
    Replacement::draft_throws(0x1419b2cc0, "unknown_1419b2cc0", fn_1419b2cc0 as *const (), Throws::Discard), // sub_1419b2cc0
    Replacement::draft_throws(0x1419b2e90, "unknown_1419b2e90", fn_1419b2e90 as *const (), Throws::Discard), // sub_1419b2e90
    Replacement::draft_throws(0x1419b3040, "unknown_1419b3040", fn_1419b3040 as *const (), Throws::Discard), // sub_1419b3040
    Replacement::draft_throws(0x1419b31d0, "unknown_1419b31d0", fn_1419b31d0 as *const (), Throws::Discard), // sub_1419b31d0
    Replacement::draft_throws(0x1419b33f0, "unknown_1419b33f0", fn_1419b33f0 as *const (), Throws::Discard), // sub_1419b33f0
    Replacement::draft_throws(0x1419b3550, "unknown_1419b3550", fn_1419b3550 as *const (), Throws::Discard), // sub_1419b3550
    Replacement::draft_throws(0x1419b36b0, "unknown_1419b36b0", fn_1419b36b0 as *const (), Throws::Discard), // sub_1419b36b0
    Replacement::draft_throws(0x1419b3860, "unknown_1419b3860", fn_1419b3860 as *const (), Throws::Discard), // sub_1419b3860
    Replacement::draft_throws(0x1419b3a50, "unknown_1419b3a50", fn_1419b3a50 as *const (), Throws::Caught), // sub_1419b3a50
    Replacement::draft_throws(0x1419b3b70, "unknown_1419b3b70", fn_1419b3b70 as *const (), Throws::Caught), // sub_1419b3b70
    Replacement::draft_throws(0x1419b3cd0, "unknown_1419b3cd0", fn_1419b3cd0 as *const (), Throws::Discard), // sub_1419b3cd0
    Replacement::draft_throws(0x1419b3e40, "unknown_1419b3e40", fn_1419b3e40 as *const (), Throws::Discard), // sub_1419b3e40
    Replacement::draft_throws(0x1419b3e80, "unknown_1419b3e80", fn_1419b3e80 as *const (), Throws::Discard), // sub_1419b3e80
    Replacement::draft_throws(0x1419b3fc0, "unknown_1419b3fc0", fn_1419b3fc0 as *const (), Throws::Caught), // sub_1419b3fc0
    Replacement::draft_throws(0x1419b4180, "unknown_1419b4180", fn_1419b4180 as *const (), Throws::Discard), // sub_1419b4180
    Replacement::draft_throws(0x1419b42c0, "unknown_1419b42c0", fn_1419b42c0 as *const (), Throws::Discard), // sub_1419b42c0
    Replacement::draft_throws(0x1419b4450, "unknown_1419b4450", fn_1419b4450 as *const (), Throws::Discard), // sub_1419b4450
    Replacement::draft_throws(0x1419b4540, "unknown_1419b4540", tpl_36121::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8f6e4, 0x142489ee0, 0x14040f4d0, 0x1426467e8, 0x1421158a0, 0x143107170, 0x142116b30, 0x1419b4659, 0x1419b466b, 0x1419b467b, 0x1419b4659, 0x1419b466b, 0x1419b467b> as *const (), Throws::Discard), // sub_1419b4540
    Replacement::draft_throws(0x1419b4690, "unknown_1419b4690", tpl_3880::<0x78, 0x143c9f0b8, 0x580e0, 0x143110698, 0x142116b30, 0x1419b46c6> as *const (), Throws::Discard), // sub_1419b4690
    Replacement::draft_throws(0x1419b46d0, "unknown_1419b46d0", tpl_3880::<0x78, 0x143c8d0d4, 0x580e0, 0x143110698, 0x142116b30, 0x1419b4706> as *const (), Throws::Discard), // sub_1419b46d0
    Replacement::draft(0x1419b4710, "unknown_1419b4710", fn_1419b4710 as *const ()), // sub_1419b4710
    Replacement::draft_throws(0x1419b4760, "unknown_1419b4760", fn_1419b4760 as *const (), Throws::Discard), // sub_1419b4760
    Replacement::draft(0x1419b7770, "unknown_1419b7770", tpl_34667::<0x20> as *const ()), // sub_1419b7770
    Replacement::draft(0x1419b7810, "unknown_1419b7810", tpl_34605::<0x14311fae8> as *const ()), // sub_1419b7810
    Replacement::draft(0x1419b78a0, "unknown_1419b78a0", tpl_36123 as *const ()), // sub_1419b78a0
    Replacement::draft(0x1419b78e0, "unknown_1419b78e0", tpl_34318 as *const ()), // sub_1419b78e0
    Replacement::draft(0x1419b7900, "unknown_1419b7900", tpl_34318 as *const ()), // sub_1419b7900
    Replacement::draft(0x1419b7920, "unknown_1419b7920", tpl_34318 as *const ()), // sub_1419b7920
    Replacement::draft(0x1419b7940, "unknown_1419b7940", tpl_34318 as *const ()), // sub_1419b7940
    Replacement::draft(0x1419b7960, "unknown_1419b7960", tpl_34318 as *const ()), // sub_1419b7960
    Replacement::draft(0x1419b7980, "unknown_1419b7980", tpl_34318 as *const ()), // sub_1419b7980
    Replacement::draft(0x1419b79a0, "unknown_1419b79a0", tpl_34319::<0x143c9f438, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b79a0
    Replacement::draft(0x1419b7a00, "unknown_1419b7a00", tpl_34319::<0x143c97a60, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b7a00
    Replacement::draft(0x1419b7a60, "unknown_1419b7a60", tpl_34319::<0x143c917cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b7a60
    Replacement::draft(0x1419b7ac0, "unknown_1419b7ac0", tpl_34319::<0x143c91f1c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b7ac0
    Replacement::draft(0x1419b7b20, "unknown_1419b7b20", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b7b20
    Replacement::draft(0x1419b7b80, "unknown_1419b7b80", tpl_34319::<0x143c8fab8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b7b80
    Replacement::draft(0x1419b7be0, "std_1419b7be0", tpl_770::<0x14311fab0> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::locomotionprobe::update::<lambda_2>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1419b7c00, "std_1419b7c00", tpl_770::<0x14311fab0> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::locomotionprobe::update::<lambda_2>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1419b7c20, "std_1419b7c20", fn_1419b7c20 as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::locomotionprobe::update::<lambda_2>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1419b7ce0, "std_1419b7ce0", tpl_1::<0x143887df0> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::locomotionprobe::update::<lambda_2>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1419b7cf0, "std_1419b7cf0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::locomotionprobe::update::<lambda_2>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1419b7d00, "std_1419b7d00", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::locomotionprobe::update::<lambda_2>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x1419b7d20, "std_1419b7d20", fn_1419b7d20 as *const ()), // sub_1419b7d20
    Replacement::draft(0x1419b7d50, "std_1419b7d50", fn_1419b7d50 as *const ()), // sub_1419b7d50
    Replacement::draft_throws(0x1419b7e10, "std_1419b7e10", fn_1419b7e10 as *const (), Throws::Discard), // sub_1419b7e10
    Replacement::draft(0x1419b8780, "std_1419b8780", tpl_361 as *const ()), // sub_1419b8780
    Replacement::draft(0x1419b8790, "std_1419b8790", fn_1419b8790 as *const ()), // sub_1419b8790
    Replacement::draft_throws(0x1419b89d0, "std_1419b89d0", tpl_35463::<0x1427242a0, 0x1410d6ca0, 0x140421c10, 0x143823898, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x14387e148, 0x140422740, 0x1402116e0> as *const (), Throws::Discard), // sub_1419b89d0
    Replacement::draft(0x1419b8b90, "std_1419b8b90", tpl_34318 as *const ()), // sub_1419b8b90
    Replacement::draft(0x1419b8bb0, "std_1419b8bb0", tpl_34319::<0x143c9d1b8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b8bb0
    Replacement::draft_throws(0x1419b8c10, "std_1419b8c10", fn_1419b8c10 as *const (), Throws::Discard), // sub_1419b8c10
    Replacement::draft(0x1419b8ce0, "std_1419b8ce0", tpl_361 as *const ()), // sub_1419b8ce0
    Replacement::draft_throws(0x1419b8cf0, "std_1419b8cf0", fn_1419b8cf0 as *const (), Throws::Caught), // sub_1419b8cf0
    Replacement::draft(0x1419b8d80, "std_1419b8d80", tpl_361 as *const ()), // sub_1419b8d80
    Replacement::draft(0x1419b8d90, "std_1419b8d90", tpl_361 as *const ()), // sub_1419b8d90
    Replacement::draft_throws(0x1419b8da0, "std_1419b8da0", fn_1419b8da0 as *const (), Throws::Caught), // sub_1419b8da0
    Replacement::draft(0x1419b8e00, "std_1419b8e00", fn_1419b8e00 as *const ()), // sub_1419b8e00
    Replacement::draft_throws(0x1419b8f20, "std_1419b8f20", fn_1419b8f20 as *const (), Throws::Caught), // sub_1419b8f20
    Replacement::draft_throws(0x1419b91e0, "std_1419b91e0", tpl_36133::<0x142724bf0, 0x8, 0x18, 0x1410d6ca0, 0x10, 0x140423d60, 0x140424060> as *const (), Throws::Discard), // sub_1419b91e0
    Replacement::draft(0x1419b9250, "std_1419b9250", tpl_34318 as *const ()), // sub_1419b9250
    Replacement::draft(0x1419b9270, "std_1419b9270", tpl_34319::<0x143c8d478, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b9270
    Replacement::draft(0x1419b92d0, "std_1419b92d0", tpl_34318 as *const ()), // sub_1419b92d0
    Replacement::draft(0x1419b92f0, "std_1419b92f0", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b92f0
    Replacement::draft(0x1419b9350, "std_1419b9350", tpl_361 as *const ()), // sub_1419b9350
    Replacement::draft_throws(0x1419b9360, "std_1419b9360", fn_1419b9360 as *const (), Throws::Discard), // sub_1419b9360
    Replacement::draft(0x1419b97d0, "std_1419b97d0", tpl_361 as *const ()), // sub_1419b97d0
    Replacement::draft(0x1419b97e0, "std_1419b97e0", fn_1419b97e0 as *const ()), // sub_1419b97e0
    Replacement::draft(0x1419b97f0, "std_1419b97f0", tpl_36135 as *const ()), // sub_1419b97f0
    Replacement::draft_throws(0x1419b9810, "std_1419b9810", fn_1419b9810 as *const (), Throws::Discard), // sub_1419b9810
    Replacement::draft(0x1419b9a10, "std_1419b9a10", tpl_34994::<0x143c9fa00, 0x14387b120, 0x14387b118, 0x141834a78, 0x1425e8040, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419b9a10
    Replacement::draft(0x1419b9a80, "std_1419b9a80", tpl_34394::<0x10> as *const ()), // sub_1419b9a80
    Replacement::draft(0x1419b9ab0, "std_1419b9ab0", tpl_34994::<0x143c9f7d0, 0x14387b130, 0x14387b128, 0x141834a78, 0x1425e8050, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419b9ab0
    Replacement::draft(0x1419b9b20, "std_1419b9b20", fn_1419b9b20 as *const ()), // sub_1419b9b20
    Replacement::draft(0x1419b9bf0, "std_1419b9bf0", fn_1419b9bf0 as *const ()), // sub_1419b9bf0
    Replacement::draft(0x1419b9c40, "std_1419b9c40", tpl_36138::<0x140429080> as *const ()), // sub_1419b9c40
    Replacement::draft(0x1419b9c80, "std_1419b9c80", tpl_34318 as *const ()), // sub_1419b9c80
    Replacement::draft(0x1419b9ca0, "std_1419b9ca0", tpl_34318 as *const ()), // sub_1419b9ca0
    Replacement::draft(0x1419b9cc0, "std_1419b9cc0", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b9cc0
    Replacement::draft(0x1419b9d20, "std_1419b9d20", tpl_34319::<0x143c98134, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b9d20
    Replacement::draft(0x1419b9d80, "std_1419b9d80", tpl_550::<0x1431209d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::light_sensitive_decal::decal_activation::system::<lambda_1>,bool,const coregame::Physi
    Replacement::draft(0x1419b9da0, "std_1419b9da0", tpl_550::<0x1431209d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::light_sensitive_decal::decal_activation::system::<lambda_1>,bool,const coregame::Physi
    Replacement::draft(0x1419b9dc0, "std_1419b9dc0", fn_1419b9dc0 as *const ()), // std::_Func_impl_no_alloc<bigfish::light_sensitive_decal::decal_activation::system::<lambda_1>,bool,const coregame::Physi
    Replacement::draft(0x1419b9e30, "std_1419b9e30", tpl_1::<0x143888100> as *const ()), // std::_Func_impl_no_alloc<bigfish::light_sensitive_decal::decal_activation::system::<lambda_1>,bool,const coregame::Physi
    Replacement::draft(0x1419b9e40, "std_1419b9e40", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::light_sensitive_decal::decal_activation::system::<lambda_1>,bool,const coregame::Physi
    Replacement::draft(0x1419b9e50, "std_1419b9e50", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::light_sensitive_decal::decal_activation::system::<lambda_1>,bool,const coregame::Physi
    Replacement::draft(0x1419b9e70, "unknown_1419b9e70", tpl_34318 as *const ()), // sub_1419b9e70
    Replacement::draft(0x1419b9e90, "unknown_1419b9e90", tpl_34318 as *const ()), // sub_1419b9e90
    Replacement::draft(0x1419b9eb0, "unknown_1419b9eb0", tpl_34319::<0x143c9f8e8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b9eb0
    Replacement::draft(0x1419b9f10, "unknown_1419b9f10", tpl_34319::<0x143c9a074, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b9f10
    Replacement::draft(0x1419b9f70, "unknown_1419b9f70", tpl_34318 as *const ()), // sub_1419b9f70
    Replacement::draft(0x1419b9f90, "unknown_1419b9f90", tpl_34319::<0x143c9f930, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419b9f90
    Replacement::draft(0x1419b9ff0, "unknown_1419b9ff0", tpl_34318 as *const ()), // sub_1419b9ff0
    Replacement::draft(0x1419ba010, "unknown_1419ba010", tpl_34318 as *const ()), // sub_1419ba010
    Replacement::draft(0x1419ba030, "unknown_1419ba030", tpl_34318 as *const ()), // sub_1419ba030
    Replacement::draft(0x1419ba050, "unknown_1419ba050", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ba050
    Replacement::draft(0x1419ba0b0, "unknown_1419ba0b0", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ba0b0
    Replacement::draft(0x1419ba110, "unknown_1419ba110", tpl_34319::<0x143c9dba4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ba110
    Replacement::draft(0x1419ba170, "unknown_1419ba170", tpl_4115::<0x8, 0x20, 0x24, 0x2c, 0x140426aa0> as *const ()), // sub_1419ba170
    Replacement::draft(0x1419ba1e0, "unknown_1419ba1e0", tpl_34443 as *const ()), // sub_1419ba1e0
    Replacement::draft(0x1419ba1f0, "unknown_1419ba1f0", fn_1419ba1f0 as *const ()), // sub_1419ba1f0
    Replacement::draft(0x1419ba260, "unknown_1419ba260", tpl_361 as *const ()), // sub_1419ba260
    Replacement::draft(0x1419ba270, "unknown_1419ba270", fn_1419ba270 as *const ()), // sub_1419ba270
    Replacement::draft(0x1419bab00, "unknown_1419bab00", tpl_361 as *const ()), // sub_1419bab00
    Replacement::draft(0x1419bab10, "unknown_1419bab10", tpl_361 as *const ()), // sub_1419bab10
    Replacement::draft_throws(0x1419bab20, "unknown_1419bab20", fn_1419bab20 as *const (), Throws::Caught), // sub_1419bab20
    Replacement::draft(0x1419bb9d0, "unknown_1419bb9d0", tpl_361 as *const ()), // sub_1419bb9d0
    Replacement::draft(0x1419bb9e0, "unknown_1419bb9e0", fn_1419bb9e0 as *const ()), // sub_1419bb9e0
    Replacement::draft_throws(0x1419bbb10, "unknown_1419bbb10", fn_1419bbb10 as *const (), Throws::Discard), // sub_1419bbb10
    Replacement::draft(0x1419bbec0, "unknown_1419bbec0", tpl_34994::<0x143c9fb70, 0x14387b180, 0x14387b178, 0x141834a78, 0x1425e8230, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419bbec0
    Replacement::draft(0x1419bbf30, "unknown_1419bbf30", fn_1419bbf30 as *const ()), // sub_1419bbf30
    Replacement::draft(0x1419bc0e0, "unknown_1419bc0e0", fn_1419bc0e0 as *const ()), // sub_1419bc0e0
    Replacement::draft(0x1419bc100, "unknown_1419bc100", fn_1419bc100 as *const ()), // sub_1419bc100
    Replacement::draft(0x1419bc160, "unknown_1419bc160", tpl_34994::<0x143c9fe5c, 0x14387b160, 0x14387b158, 0x141834a78, 0x1425e8240, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419bc160
    Replacement::draft(0x1419bc1d0, "unknown_1419bc1d0", tpl_34667::<0x20> as *const ()), // sub_1419bc1d0
    Replacement::draft(0x1419bc200, "unknown_1419bc200", tpl_34994::<0x143c9fce8, 0x14387b140, 0x14387b138, 0x141834a78, 0x1425e81f0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419bc200
    Replacement::draft(0x1419bc270, "unknown_1419bc270", tpl_95993::<0x1431223a0, 0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // sub_1419bc270
    Replacement::draft(0x1419bc430, "unknown_1419bc430", tpl_34393 as *const ()), // sub_1419bc430
    Replacement::draft(0x1419bc4c0, "unknown_1419bc4c0", tpl_34612 as *const ()), // sub_1419bc4c0
    Replacement::draft(0x1419bc550, "unknown_1419bc550", tpl_34612 as *const ()), // sub_1419bc550
    Replacement::draft(0x1419bcc30, "unknown_1419bcc30", tpl_34624::<0x143c9fe9c, 0x14020e8f0, 0x143c9fb58, 0x143c9fb54> as *const ()), // sub_1419bcc30
    Replacement::draft(0x1419bcea0, "unknown_1419bcea0", fn_1419bcea0 as *const ()), // sub_1419bcea0
    Replacement::draft(0x1419bcfb0, "unknown_1419bcfb0", fn_1419bcfb0 as *const ()), // sub_1419bcfb0
    Replacement::draft_throws(0x1419bd080, "unknown_1419bd080", fn_1419bd080 as *const (), Throws::Caught), // sub_1419bd080
    Replacement::draft_throws(0x1419bd270, "unknown_1419bd270", fn_1419bd270 as *const (), Throws::Caught), // sub_1419bd270
    Replacement::draft_throws(0x1419bd510, "unknown_1419bd510", fn_1419bd510 as *const (), Throws::Caught), // sub_1419bd510
    Replacement::draft_throws(0x1419bd820, "unknown_1419bd820", fn_1419bd820 as *const (), Throws::Discard), // sub_1419bd820
    Replacement::draft(0x1419bd9d0, "unknown_1419bd9d0", fn_1419bd9d0 as *const ()), // sub_1419bd9d0
    Replacement::draft(0x1419bda40, "unknown_1419bda40", fn_1419bda40 as *const ()), // sub_1419bda40
    Replacement::draft(0x1419bda90, "unknown_1419bda90", fn_1419bda90 as *const ()), // sub_1419bda90
    Replacement::draft(0x1419bdac0, "unknown_1419bdac0", fn_1419bdac0 as *const ()), // sub_1419bdac0
    Replacement::draft_throws(0x1419bde10, "unknown_1419bde10", fn_1419bde10 as *const (), Throws::Discard), // sub_1419bde10
    Replacement::draft(0x1419be750, "unknown_1419be750", tpl_34671::<0x20> as *const ()), // sub_1419be750
    Replacement::draft(0x1419be7f0, "unknown_1419be7f0", tpl_34605::<0x143126360> as *const ()), // sub_1419be7f0
    Replacement::draft(0x1419be880, "unknown_1419be880", tpl_36123 as *const ()), // sub_1419be880
    Replacement::draft(0x1419be8c0, "unknown_1419be8c0", tpl_34318 as *const ()), // sub_1419be8c0
    Replacement::draft(0x1419be8e0, "unknown_1419be8e0", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419be8e0
    Replacement::draft(0x1419be940, "unknown_1419be940", tpl_34318 as *const ()), // sub_1419be940
    Replacement::draft(0x1419be960, "unknown_1419be960", tpl_34318 as *const ()), // sub_1419be960
    Replacement::draft(0x1419be980, "unknown_1419be980", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419be980
    Replacement::draft(0x1419be9e0, "unknown_1419be9e0", tpl_34319::<0x143ca02c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419be9e0
    Replacement::draft(0x1419bea40, "std_1419bea40", tpl_550::<0x143126318, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::lightdamagereceiver::simulated_light::system::<lambda_1>,bool,const coregame::PhysicsS
    Replacement::draft(0x1419bea60, "std_1419bea60", tpl_550::<0x143126318, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::lightdamagereceiver::simulated_light::system::<lambda_1>,bool,const coregame::PhysicsS
    Replacement::draft(0x1419bea80, "std_1419bea80", fn_1419bea80 as *const ()), // std::_Func_impl_no_alloc<bigfish::lightdamagereceiver::simulated_light::system::<lambda_1>,bool,const coregame::PhysicsS
    Replacement::draft(0x1419bebb0, "std_1419bebb0", tpl_1::<0x1438886b0> as *const ()), // std::_Func_impl_no_alloc<bigfish::lightdamagereceiver::simulated_light::system::<lambda_1>,bool,const coregame::PhysicsS
    Replacement::draft(0x1419bebc0, "std_1419bebc0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::lightdamagereceiver::simulated_light::system::<lambda_1>,bool,const coregame::PhysicsS
    Replacement::draft(0x1419bebd0, "std_1419bebd0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::lightdamagereceiver::simulated_light::system::<lambda_1>,bool,const coregame::PhysicsS
    Replacement::draft(0x1419bebf0, "unknown_1419bebf0", tpl_34318 as *const ()), // sub_1419bebf0
    Replacement::draft(0x1419bec10, "unknown_1419bec10", tpl_34319::<0x143c9fde0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419bec10
    Replacement::draft(0x1419bec70, "unknown_1419bec70", tpl_361 as *const ()), // sub_1419bec70
    Replacement::draft(0x1419bec80, "unknown_1419bec80", fn_1419bec80 as *const ()), // sub_1419bec80
    Replacement::draft(0x1419bef10, "unknown_1419bef10", fn_1419bef10 as *const ()), // sub_1419bef10
    Replacement::draft(0x1419bf110, "unknown_1419bf110", tpl_361 as *const ()), // sub_1419bf110
    Replacement::draft(0x1419bf120, "unknown_1419bf120", fn_1419bf120 as *const ()), // sub_1419bf120
    Replacement::draft(0x1419bf210, "unknown_1419bf210", fn_1419bf210 as *const ()), // sub_1419bf210
    Replacement::draft(0x1419bf3c0, "unknown_1419bf3c0", tpl_361 as *const ()), // sub_1419bf3c0
    Replacement::draft(0x1419bf3d0, "unknown_1419bf3d0", tpl_34443 as *const ()), // sub_1419bf3d0
    Replacement::draft(0x1419bf3e0, "unknown_1419bf3e0", fn_1419bf3e0 as *const ()), // sub_1419bf3e0
    Replacement::draft_throws(0x1419bf510, "unknown_1419bf510", fn_1419bf510 as *const (), Throws::Discard), // sub_1419bf510
    Replacement::draft_throws(0x1419bf9d0, "unknown_1419bf9d0", fn_1419bf9d0 as *const (), Throws::Discard), // sub_1419bf9d0
    Replacement::draft(0x1419bfde0, "unknown_1419bfde0", tpl_34994::<0x143ca0554, 0x14387b1d0, 0x14387b1c8, 0x141834a78, 0x1425e83c0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419bfde0
    Replacement::draft(0x1419bfe50, "unknown_1419bfe50", tpl_34605::<0x1431280a0> as *const ()), // sub_1419bfe50
    Replacement::draft(0x1419bff90, "unknown_1419bff90", tpl_34605::<0x143127f48> as *const ()), // sub_1419bff90
    Replacement::draft(0x1419c02c0, "unknown_1419c02c0", tpl_35394::<0x9, 0x18, 0x10, 0x140432200> as *const ()), // sub_1419c02c0
    Replacement::draft(0x1419c0380, "unknown_1419c0380", fn_1419c0380 as *const ()), // sub_1419c0380
    Replacement::draft(0x1419c0430, "unknown_1419c0430", tpl_35394::<0x9, 0x18, 0x10, 0x140432120> as *const ()), // sub_1419c0430
    Replacement::draft(0x1419c04f0, "unknown_1419c04f0", fn_1419c04f0 as *const ()), // sub_1419c04f0
    Replacement::draft_throws(0x1419c05a0, "unknown_1419c05a0", fn_1419c05a0 as *const (), Throws::Discard), // sub_1419c05a0
    Replacement::draft(0x1419c0a60, "unknown_1419c0a60", fn_1419c0a60 as *const ()), // sub_1419c0a60
    Replacement::draft_throws(0x1419c0bd0, "unknown_1419c0bd0", fn_1419c0bd0 as *const (), Throws::Caught), // sub_1419c0bd0
    Replacement::draft_throws(0x1419c1360, "bigfish_1419c1360", fn_1419c1360 as *const (), Throws::Caught), // bigfish::LeadDecorator::bigfish::LeadDecorator::vfunc_12
    Replacement::draft(0x1419c14b0, "bigfish_1419c14b0", fn_1419c14b0 as *const ()), // bigfish::LeadDecorator::bigfish::LeadDecorator::vfunc_5
    Replacement::draft(0x1419c1530, "bigfish_1419c1530", fn_1419c1530 as *const ()), // bigfish::LeadDecorator::bigfish::LeadDecorator::vfunc_4
    Replacement::draft(0x1419c1b10, "bigfish_1419c1b10", fn_1419c1b10 as *const ()), // bigfish::LeadDecorator::ClassTypeInfoImplementation::bigfish::LeadDecorator::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1419c1bc0, "bigfish_1419c1bc0", fn_1419c1bc0 as *const ()), // bigfish::LeadDecorator::ClassTypeInfoImplementation::bigfish::LeadDecorator::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1419c1db0, "unknown_1419c1db0", tpl_35099::<0x20, 0x40> as *const ()), // sub_1419c1db0
    Replacement::draft(0x1419c1e60, "unknown_1419c1e60", tpl_94428::<0x143128510, 0x8> as *const ()), // sub_1419c1e60
    Replacement::draft(0x1419c2660, "unknown_1419c2660", tpl_4161::<0x8, 0x140437b20> as *const ()), // sub_1419c2660
    Replacement::draft(0x1419c26f0, "unknown_1419c26f0", tpl_4160::<0x8, 0x140437650, 0x140437850> as *const ()), // sub_1419c26f0
    Replacement::draft(0x1419c27a0, "unknown_1419c27a0", fn_1419c27a0 as *const ()), // sub_1419c27a0
    Replacement::draft(0x1419c29f0, "unknown_1419c29f0", fn_1419c29f0 as *const ()), // sub_1419c29f0
    Replacement::draft(0x1419c2ec0, "unknown_1419c2ec0", fn_1419c2ec0 as *const ()), // sub_1419c2ec0
    Replacement::draft(0x1419c3580, "unknown_1419c3580", fn_1419c3580 as *const ()), // sub_1419c3580
    Replacement::draft(0x1419c3690, "unknown_1419c3690", fn_1419c3690 as *const ()), // sub_1419c3690
    Replacement::draft(0x1419c37c0, "unknown_1419c37c0", fn_1419c37c0 as *const ()), // sub_1419c37c0
    Replacement::draft(0x1419c3890, "unknown_1419c3890", fn_1419c3890 as *const ()), // sub_1419c3890
    Replacement::draft(0x1419c3da0, "unknown_1419c3da0", fn_1419c3da0 as *const ()), // sub_1419c3da0
    Replacement::draft_throws(0x1419c4310, "unknown_1419c4310", fn_1419c4310 as *const (), Throws::Discard), // sub_1419c4310
    Replacement::draft(0x1419c4a30, "unknown_1419c4a30", fn_1419c4a30 as *const ()), // sub_1419c4a30
    Replacement::draft(0x1419c4a80, "unknown_1419c4a80", tpl_36174::<0x14> as *const ()), // sub_1419c4a80
    Replacement::draft(0x1419c4aa0, "unknown_1419c4aa0", fn_1419c4aa0 as *const ()), // sub_1419c4aa0
    Replacement::draft(0x1419c4ac0, "unknown_1419c4ac0", fn_1419c4ac0 as *const ()), // sub_1419c4ac0
    Replacement::draft(0x1419c4c00, "unknown_1419c4c00", tpl_36174::<0x8> as *const ()), // sub_1419c4c00
    Replacement::draft(0x1419c4c20, "unknown_1419c4c20", tpl_36174::<0x4> as *const ()), // sub_1419c4c20
    Replacement::draft(0x1419c4c40, "unknown_1419c4c40", fn_1419c4c40 as *const ()), // sub_1419c4c40
    Replacement::draft(0x1419c4d00, "unknown_1419c4d00", fn_1419c4d00 as *const ()), // sub_1419c4d00
    Replacement::draft_throws(0x1419c4d40, "unknown_1419c4d40", tpl_36179::<0x143c98648, 0x143ca0ddc, 0x14043b3f0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14043b640, 0x1425371a0, 0x142533200, 0x142729d70, 0x1410d6ca0, 0x14043ae00, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_1419c4d40
    Replacement::draft(0x1419c5070, "unknown_1419c5070", fn_1419c5070 as *const ()), // sub_1419c5070
    Replacement::draft(0x1419c51f0, "unknown_1419c51f0", tpl_34835::<0x8, 0x14045bad0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419c51f0
    Replacement::draft(0x1419c5270, "unknown_1419c5270", tpl_34835::<0x8, 0x14045bb70, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419c5270
    Replacement::draft(0x1419c52f0, "unknown_1419c52f0", tpl_34835::<0x8, 0x1403ea7a0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419c52f0
    Replacement::draft(0x1419c5370, "unknown_1419c5370", tpl_34835::<0x8, 0x140261060, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419c5370
    Replacement::draft(0x1419c53f0, "unknown_1419c53f0", tpl_34835::<0x8, 0x14045bc40, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419c53f0
    Replacement::draft(0x1419c5470, "unknown_1419c5470", tpl_34835::<0x8, 0x140259450, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419c5470
    Replacement::draft(0x1419c54f0, "unknown_1419c54f0", tpl_34837::<0x14045bad0, 0x20, 0x14240aa00> as *const ()), // sub_1419c54f0
    Replacement::draft(0x1419c5540, "unknown_1419c5540", tpl_34837::<0x14045bb70, 0x20, 0x14240aa00> as *const ()), // sub_1419c5540
    Replacement::draft(0x1419c5590, "unknown_1419c5590", tpl_34837::<0x1403ea7a0, 0x20, 0x14240aa00> as *const ()), // sub_1419c5590
    Replacement::draft(0x1419c55e0, "unknown_1419c55e0", tpl_34837::<0x140261060, 0x20, 0x14240aa00> as *const ()), // sub_1419c55e0
    Replacement::draft(0x1419c5630, "unknown_1419c5630", tpl_34837::<0x14045bc40, 0x20, 0x14240aa00> as *const ()), // sub_1419c5630
    Replacement::draft(0x1419c5680, "unknown_1419c5680", tpl_34837::<0x140259450, 0x20, 0x14240aa00> as *const ()), // sub_1419c5680
    Replacement::draft(0x1419c56d0, "unknown_1419c56d0", tpl_34840::<0x14045abd0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1419c56d0
    Replacement::draft(0x1419c5740, "unknown_1419c5740", tpl_34840::<0x14045ad90, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1419c5740
    Replacement::draft(0x1419c57b0, "unknown_1419c57b0", tpl_34840::<0x14045afd0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1419c57b0
    Replacement::draft(0x1419c5820, "unknown_1419c5820", tpl_34840::<0x1403da6d0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1419c5820
    Replacement::draft(0x1419c5890, "unknown_1419c5890", tpl_34840::<0x14045b200, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1419c5890
    Replacement::draft(0x1419c5900, "unknown_1419c5900", tpl_34840::<0x140258f10, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1419c5900
    Replacement::draft_throws(0x1419c63d0, "unknown_1419c63d0", tpl_94598::<0x140251800, 0x14309c488, 0x14272a470, 0x14272a3f0, 0x14272a5c0, 0x14272a440, 0x14272a570, 0x14272a408, 0x14272a650, 0x14272a448, 0x14272a5d0, 0x14272a488, 0x140254aa0, 0x1424b16a0, 0x140256bc0> as *const (), Throws::Discard), // sub_1419c63d0
    Replacement::draft_throws(0x1419c6a00, "unknown_1419c6a00", fn_1419c6a00 as *const (), Throws::Discard), // sub_1419c6a00
    Replacement::draft_throws(0x1419c7590, "unknown_1419c7590", tpl_94651::<0x14025ca60, 0x14309c488, 0x14272a440, 0x14272a448, 0x14272a648, 0x14272a488, 0x140254aa0, 0x1424b16a0, 0x14025f260> as *const (), Throws::Discard), // sub_1419c7590
    Replacement::draft_throws(0x1419c7860, "unknown_1419c7860", tpl_95454::<0x1403e62b0, 0x14309c488, 0x14272a348, 0x14272a368, 0x14272a358, 0x14272a380, 0x14272a374, 0x14272a350, 0x140254aa0, 0x1424b16a0, 0x1403e80f0> as *const (), Throws::Discard), // sub_1419c7860
    Replacement::draft_throws(0x1419c7c50, "unknown_1419c7c50", tpl_96039::<0x14043c3b0, 0x14309c488, 0x14272a680, 0x14272a678, 0x14272a618, 0x14272a624, 0x14272a5f0, 0x14272a66c, 0x14272a674, 0x140254aa0, 0x1424b16a0, 0x140448cc0> as *const (), Throws::Discard), // sub_1419c7c50
    Replacement::draft_throws(0x1419c80e0, "unknown_1419c80e0", tpl_96040::<0x14043c2a0, 0x14309c488, 0x14272a3c8, 0x14272a340, 0x140254aa0, 0x1424b16a0, 0x140448ae0> as *const (), Throws::Discard), // sub_1419c80e0
    Replacement::draft_throws(0x1419c8290, "unknown_1419c8290", fn_1419c8290 as *const (), Throws::Caught), // sub_1419c8290
    Replacement::draft_throws(0x1419c8490, "unknown_1419c8490", tpl_34845::<0x18, 0x1419c63d0, 0x10> as *const (), Throws::Discard), // sub_1419c8490
    Replacement::draft_throws(0x1419c84d0, "unknown_1419c84d0", tpl_34846::<0x1401f3a20, 0x1419c63d0> as *const (), Throws::Discard), // sub_1419c84d0
    Replacement::draft_throws(0x1419c8550, "unknown_1419c8550", fn_1419c8550 as *const (), Throws::Caught), // sub_1419c8550
    Replacement::draft_throws(0x1419c8750, "unknown_1419c8750", fn_1419c8750 as *const (), Throws::Discard), // sub_1419c8750
    Replacement::draft_throws(0x1419c8790, "unknown_1419c8790", tpl_34846::<0x1401f3a20, 0x1419c6a00> as *const (), Throws::Discard), // sub_1419c8790
    Replacement::draft_throws(0x1419c8810, "unknown_1419c8810", fn_1419c8810 as *const (), Throws::Caught), // sub_1419c8810
    Replacement::draft_throws(0x1419c8a10, "unknown_1419c8a10", tpl_34895::<0x18, 0x1419c7590, 0x10> as *const (), Throws::Discard), // sub_1419c8a10
    Replacement::draft_throws(0x1419c8a50, "unknown_1419c8a50", tpl_34846::<0x1401f3a20, 0x1419c7590> as *const (), Throws::Discard), // sub_1419c8a50
    Replacement::draft_throws(0x1419c8ad0, "unknown_1419c8ad0", fn_1419c8ad0 as *const (), Throws::Caught), // sub_1419c8ad0
    Replacement::draft_throws(0x1419c8cd0, "unknown_1419c8cd0", tpl_35688::<0x18, 0x1419c7860, 0x10> as *const (), Throws::Discard), // sub_1419c8cd0
    Replacement::draft_throws(0x1419c8d10, "unknown_1419c8d10", fn_1419c8d10 as *const (), Throws::Discard), // sub_1419c8d10
    Replacement::draft_throws(0x1419c8d60, "unknown_1419c8d60", fn_1419c8d60 as *const (), Throws::Caught), // sub_1419c8d60
    Replacement::draft_throws(0x1419c8f60, "unknown_1419c8f60", tpl_36186::<0x18, 0x1419c7c50, 0x10> as *const (), Throws::Discard), // sub_1419c8f60
    Replacement::draft_throws(0x1419c8fa0, "unknown_1419c8fa0", tpl_34846::<0x1401f3a20, 0x1419c7c50> as *const (), Throws::Discard), // sub_1419c8fa0
    Replacement::draft_throws(0x1419c9020, "unknown_1419c9020", fn_1419c9020 as *const (), Throws::Caught), // sub_1419c9020
    Replacement::draft_throws(0x1419c9220, "unknown_1419c9220", tpl_36188::<0x18, 0x1419c80e0, 0x10> as *const (), Throws::Discard), // sub_1419c9220
    Replacement::draft_throws(0x1419c9260, "unknown_1419c9260", tpl_34846::<0x1401f3a20, 0x1419c80e0> as *const (), Throws::Discard), // sub_1419c9260
    Replacement::draft_throws(0x1419c9400, "unknown_1419c9400", fn_1419c9400 as *const (), Throws::Discard), // sub_1419c9400
    Replacement::draft_throws(0x1419c95c0, "unknown_1419c95c0", fn_1419c95c0 as *const (), Throws::Discard), // sub_1419c95c0
    Replacement::draft(0x1419c9680, "unknown_1419c9680", fn_1419c9680 as *const ()), // sub_1419c9680
    Replacement::draft_throws(0x1419c96e0, "unknown_1419c96e0", fn_1419c96e0 as *const (), Throws::Discard), // sub_1419c96e0
    Replacement::draft_throws(0x1419c97f0, "unknown_1419c97f0", fn_1419c97f0 as *const (), Throws::Discard), // sub_1419c97f0
    Replacement::draft_throws(0x1419c9c00, "unknown_1419c9c00", fn_1419c9c00 as *const (), Throws::Discard), // sub_1419c9c00
    Replacement::draft(0x1419c9eb0, "unknown_1419c9eb0", fn_1419c9eb0 as *const ()), // sub_1419c9eb0
    Replacement::draft(0x1419c9ee0, "unknown_1419c9ee0", tpl_36194::<0x4c> as *const ()), // sub_1419c9ee0
    Replacement::draft_throws(0x1419c9f00, "unknown_1419c9f00", fn_1419c9f00 as *const (), Throws::Caught), // sub_1419c9f00
    Replacement::draft_throws(0x1419ca0c0, "unknown_1419ca0c0", fn_1419ca0c0 as *const (), Throws::Discard), // sub_1419ca0c0
    Replacement::draft_throws(0x1419ca160, "unknown_1419ca160", fn_1419ca160 as *const (), Throws::Discard), // sub_1419ca160
    Replacement::draft_throws(0x1419ca370, "unknown_1419ca370", fn_1419ca370 as *const (), Throws::Caught), // sub_1419ca370
    Replacement::draft_throws(0x1419ca7d0, "unknown_1419ca7d0", fn_1419ca7d0 as *const (), Throws::Caught), // sub_1419ca7d0
    Replacement::draft_throws(0x1419caaa0, "unknown_1419caaa0", fn_1419caaa0 as *const (), Throws::Discard), // sub_1419caaa0
    Replacement::draft_throws(0x1419cabe0, "unknown_1419cabe0", fn_1419cabe0 as *const (), Throws::Discard), // sub_1419cabe0
    Replacement::draft(0x1419cad90, "unknown_1419cad90", tpl_36194::<0x4c> as *const ()), // sub_1419cad90
    Replacement::draft_throws(0x1419cadb0, "unknown_1419cadb0", fn_1419cadb0 as *const (), Throws::Caught), // sub_1419cadb0
    Replacement::draft_throws(0x1419cb040, "unknown_1419cb040", fn_1419cb040 as *const (), Throws::Discard), // sub_1419cb040
    Replacement::draft(0x1419cb360, "unknown_1419cb360", fn_1419cb360 as *const ()), // sub_1419cb360
    Replacement::draft_throws(0x1419cb390, "unknown_1419cb390", fn_1419cb390 as *const (), Throws::Discard), // sub_1419cb390
    Replacement::draft_throws(0x1419cb420, "unknown_1419cb420", fn_1419cb420 as *const (), Throws::Discard), // sub_1419cb420
    Replacement::draft_throws(0x1419cb590, "unknown_1419cb590", fn_1419cb590 as *const (), Throws::Discard), // sub_1419cb590
    Replacement::draft_throws(0x1419cba30, "unknown_1419cba30", fn_1419cba30 as *const (), Throws::Discard), // sub_1419cba30
    Replacement::draft_throws(0x1419cbca0, "unknown_1419cbca0", fn_1419cbca0 as *const (), Throws::Caught), // sub_1419cbca0
    Replacement::draft_throws(0x1419cd140, "unknown_1419cd140", fn_1419cd140 as *const (), Throws::Caught), // sub_1419cd140
    Replacement::draft_throws(0x1419cdba0, "unknown_1419cdba0", fn_1419cdba0 as *const (), Throws::Discard), // sub_1419cdba0
    Replacement::draft_throws(0x1419cde30, "unknown_1419cde30", fn_1419cde30 as *const (), Throws::Discard), // sub_1419cde30
    Replacement::draft_throws(0x1419ce020, "unknown_1419ce020", fn_1419ce020 as *const (), Throws::Discard), // sub_1419ce020
    Replacement::draft(0x1419ce170, "unknown_1419ce170", fn_1419ce170 as *const ()), // sub_1419ce170
    Replacement::draft(0x1419ceb60, "unknown_1419ceb60", fn_1419ceb60 as *const ()), // sub_1419ceb60
    Replacement::draft(0x1419cec30, "unknown_1419cec30", fn_1419cec30 as *const ()), // sub_1419cec30
    Replacement::draft(0x1419cec50, "unknown_1419cec50", fn_1419cec50 as *const ()), // sub_1419cec50
    Replacement::draft(0x1419cecf0, "unknown_1419cecf0", fn_1419cecf0 as *const ()), // sub_1419cecf0
    Replacement::draft_throws(0x1419ceef0, "unknown_1419ceef0", fn_1419ceef0 as *const (), Throws::Caught), // sub_1419ceef0
    Replacement::draft_throws(0x1419cfde0, "unknown_1419cfde0", fn_1419cfde0 as *const (), Throws::Caught), // sub_1419cfde0
    Replacement::draft_throws(0x1419d0b90, "unknown_1419d0b90", fn_1419d0b90 as *const (), Throws::Caught), // sub_1419d0b90
    Replacement::draft_throws(0x1419d1370, "unknown_1419d1370", fn_1419d1370 as *const (), Throws::Caught), // sub_1419d1370
    Replacement::draft_throws(0x1419d1a30, "unknown_1419d1a30", fn_1419d1a30 as *const (), Throws::Caught), // sub_1419d1a30
    Replacement::draft(0x1419d21d0, "unknown_1419d21d0", fn_1419d21d0 as *const ()), // sub_1419d21d0
    Replacement::draft_throws(0x1419d2480, "unknown_1419d2480", fn_1419d2480 as *const (), Throws::Discard), // sub_1419d2480
    Replacement::draft_throws(0x1419d2990, "unknown_1419d2990", fn_1419d2990 as *const (), Throws::Caught), // sub_1419d2990
    Replacement::draft(0x1419d2f80, "unknown_1419d2f80", fn_1419d2f80 as *const ()), // sub_1419d2f80
    Replacement::draft(0x1419d3090, "unknown_1419d3090", fn_1419d3090 as *const ()), // sub_1419d3090
    Replacement::draft_throws(0x1419d3220, "unknown_1419d3220", fn_1419d3220 as *const (), Throws::Discard), // sub_1419d3220
    Replacement::draft_throws(0x1419d3530, "unknown_1419d3530", fn_1419d3530 as *const (), Throws::Discard), // sub_1419d3530
    Replacement::draft_throws(0x1419d36e0, "unknown_1419d36e0", fn_1419d36e0 as *const (), Throws::Discard), // sub_1419d36e0
    Replacement::draft(0x1419d3870, "unknown_1419d3870", fn_1419d3870 as *const ()), // sub_1419d3870
    Replacement::draft_throws(0x1419d3930, "unknown_1419d3930", fn_1419d3930 as *const (), Throws::Discard), // sub_1419d3930
    Replacement::draft_throws(0x1419d3e80, "unknown_1419d3e80", fn_1419d3e80 as *const (), Throws::Caught), // sub_1419d3e80
    Replacement::draft_throws(0x1419d4740, "unknown_1419d4740", fn_1419d4740 as *const (), Throws::Discard), // sub_1419d4740
    Replacement::draft_throws(0x1419d4800, "unknown_1419d4800", fn_1419d4800 as *const (), Throws::Discard), // sub_1419d4800
    Replacement::draft(0x1419d4a90, "unknown_1419d4a90", fn_1419d4a90 as *const ()), // sub_1419d4a90
    Replacement::draft(0x1419d4c60, "unknown_1419d4c60", fn_1419d4c60 as *const ()), // sub_1419d4c60
    Replacement::draft(0x1419d4fc0, "unknown_1419d4fc0", fn_1419d4fc0 as *const ()), // sub_1419d4fc0
    Replacement::draft(0x1419d53a0, "unknown_1419d53a0", fn_1419d53a0 as *const ()), // sub_1419d53a0
    Replacement::draft(0x1419d54f0, "unknown_1419d54f0", fn_1419d54f0 as *const ()), // sub_1419d54f0
    Replacement::draft(0x1419d5560, "unknown_1419d5560", fn_1419d5560 as *const ()), // sub_1419d5560
    Replacement::draft(0x1419d5630, "unknown_1419d5630", fn_1419d5630 as *const ()), // sub_1419d5630
    Replacement::draft(0x1419d5730, "unknown_1419d5730", fn_1419d5730 as *const ()), // sub_1419d5730
    Replacement::draft(0x1419d5c20, "unknown_1419d5c20", fn_1419d5c20 as *const ()), // sub_1419d5c20
    Replacement::draft_throws(0x1419d6060, "unknown_1419d6060", fn_1419d6060 as *const (), Throws::Discard), // sub_1419d6060
    Replacement::draft_throws(0x1419d64b0, "unknown_1419d64b0", fn_1419d64b0 as *const (), Throws::Discard), // sub_1419d64b0
    Replacement::draft_throws(0x1419d65d0, "unknown_1419d65d0", fn_1419d65d0 as *const (), Throws::Discard), // sub_1419d65d0
    Replacement::draft_throws(0x1419d6740, "unknown_1419d6740", fn_1419d6740 as *const (), Throws::Caught), // sub_1419d6740
    Replacement::draft_throws(0x1419d6c80, "unknown_1419d6c80", fn_1419d6c80 as *const (), Throws::Discard), // sub_1419d6c80
    Replacement::draft(0x1419d6f10, "unknown_1419d6f10", fn_1419d6f10 as *const ()), // sub_1419d6f10
    Replacement::draft_throws(0x1419d6f30, "unknown_1419d6f30", fn_1419d6f30 as *const (), Throws::Discard), // sub_1419d6f30
    Replacement::draft_throws(0x1419d7030, "unknown_1419d7030", fn_1419d7030 as *const (), Throws::Caught), // sub_1419d7030
    Replacement::draft(0x1419d7290, "unknown_1419d7290", tpl_36233 as *const ()), // sub_1419d7290
    Replacement::draft_throws(0x1419d72b0, "unknown_1419d72b0", fn_1419d72b0 as *const (), Throws::Discard), // sub_1419d72b0
    Replacement::draft(0x1419d75a0, "unknown_1419d75a0", fn_1419d75a0 as *const ()), // sub_1419d75a0
    Replacement::draft_throws(0x1419d7940, "unknown_1419d7940", fn_1419d7940 as *const (), Throws::Discard), // sub_1419d7940
    Replacement::draft(0x1419d80b0, "unknown_1419d80b0", fn_1419d80b0 as *const ()), // sub_1419d80b0
    Replacement::draft_throws(0x1419d8230, "unknown_1419d8230", fn_1419d8230 as *const (), Throws::Discard), // sub_1419d8230
    Replacement::draft_throws(0x1419d8440, "unknown_1419d8440", fn_1419d8440 as *const (), Throws::Discard), // sub_1419d8440
    Replacement::draft_throws(0x1419d8500, "unknown_1419d8500", fn_1419d8500 as *const (), Throws::Discard), // sub_1419d8500
    Replacement::draft(0x1419d8650, "unknown_1419d8650", fn_1419d8650 as *const ()), // sub_1419d8650
    Replacement::draft(0x1419d8880, "unknown_1419d8880", fn_1419d8880 as *const ()), // sub_1419d8880
    Replacement::draft(0x1419d8a50, "unknown_1419d8a50", fn_1419d8a50 as *const ()), // sub_1419d8a50
    Replacement::draft_throws(0x1419d8a80, "unknown_1419d8a80", fn_1419d8a80 as *const (), Throws::Discard), // sub_1419d8a80
    Replacement::draft(0x1419d8b00, "unknown_1419d8b00", fn_1419d8b00 as *const ()), // sub_1419d8b00
    Replacement::draft(0x1419d8b50, "unknown_1419d8b50", fn_1419d8b50 as *const ()), // sub_1419d8b50
    Replacement::draft(0x1419d8cb0, "unknown_1419d8cb0", fn_1419d8cb0 as *const ()), // sub_1419d8cb0
    Replacement::draft(0x1419d8f40, "unknown_1419d8f40", fn_1419d8f40 as *const ()), // sub_1419d8f40
    Replacement::draft(0x1419d8fe0, "unknown_1419d8fe0", fn_1419d8fe0 as *const ()), // sub_1419d8fe0
    Replacement::draft_throws(0x1419d9130, "unknown_1419d9130", fn_1419d9130 as *const (), Throws::Discard), // sub_1419d9130
    Replacement::draft_throws(0x1419d91f0, "unknown_1419d91f0", fn_1419d91f0 as *const (), Throws::Discard), // sub_1419d91f0
    Replacement::draft(0x1419d9410, "unknown_1419d9410", fn_1419d9410 as *const ()), // sub_1419d9410
    Replacement::draft(0x1419d9580, "unknown_1419d9580", fn_1419d9580 as *const ()), // sub_1419d9580
    Replacement::draft_throws(0x1419d9710, "unknown_1419d9710", fn_1419d9710 as *const (), Throws::Discard), // sub_1419d9710
    Replacement::draft(0x1419d9a40, "unknown_1419d9a40", fn_1419d9a40 as *const ()), // sub_1419d9a40
    Replacement::draft_throws(0x1419d9af0, "unknown_1419d9af0", fn_1419d9af0 as *const (), Throws::Discard), // sub_1419d9af0
    Replacement::draft(0x1419d9b90, "unknown_1419d9b90", fn_1419d9b90 as *const ()), // sub_1419d9b90
    Replacement::draft(0x1419d9bf0, "unknown_1419d9bf0", fn_1419d9bf0 as *const ()), // sub_1419d9bf0
    Replacement::draft(0x1419d9c30, "unknown_1419d9c30", fn_1419d9c30 as *const ()), // sub_1419d9c30
    Replacement::draft(0x1419d9cc0, "unknown_1419d9cc0", fn_1419d9cc0 as *const ()), // sub_1419d9cc0
    Replacement::draft(0x1419d9d00, "unknown_1419d9d00", fn_1419d9d00 as *const ()), // sub_1419d9d00
    Replacement::draft(0x1419d9d30, "unknown_1419d9d30", fn_1419d9d30 as *const ()), // sub_1419d9d30
    Replacement::draft(0x1419d9db0, "unknown_1419d9db0", fn_1419d9db0 as *const ()), // sub_1419d9db0
    Replacement::draft(0x1419d9e50, "unknown_1419d9e50", fn_1419d9e50 as *const ()), // sub_1419d9e50
    Replacement::draft(0x1419d9e80, "unknown_1419d9e80", fn_1419d9e80 as *const ()), // sub_1419d9e80
    Replacement::draft(0x1419d9eb0, "unknown_1419d9eb0", fn_1419d9eb0 as *const ()), // sub_1419d9eb0
    Replacement::draft(0x1419d9ef0, "unknown_1419d9ef0", fn_1419d9ef0 as *const ()), // sub_1419d9ef0
    Replacement::draft(0x1419d9f00, "unknown_1419d9f00", fn_1419d9f00 as *const ()), // sub_1419d9f00
    Replacement::draft(0x1419d9f20, "unknown_1419d9f20", fn_1419d9f20 as *const ()), // sub_1419d9f20
    Replacement::draft(0x1419d9f40, "unknown_1419d9f40", fn_1419d9f40 as *const ()), // sub_1419d9f40
    Replacement::draft(0x1419d9f50, "unknown_1419d9f50", fn_1419d9f50 as *const ()), // sub_1419d9f50
    Replacement::draft(0x1419da010, "unknown_1419da010", fn_1419da010 as *const ()), // sub_1419da010
    Replacement::draft(0x1419da130, "unknown_1419da130", fn_1419da130 as *const ()), // sub_1419da130
    Replacement::draft(0x1419da310, "unknown_1419da310", fn_1419da310 as *const ()), // sub_1419da310
    Replacement::draft(0x1419da4f0, "unknown_1419da4f0", tpl_34869::<0x143a24b63, 0x143a24b60, 0x143a24b68, 0x143a24b64, 0x14020d4a0, 0x143c90338, 0x142489ee0, 0x140251aa0, 0x1426467e8, 0x1419da6a2> as *const ()), // sub_1419da4f0
    Replacement::draft(0x1419da6d0, "unknown_1419da6d0", fn_1419da6d0 as *const ()), // sub_1419da6d0
    Replacement::draft(0x1419da8b0, "unknown_1419da8b0", fn_1419da8b0 as *const ()), // sub_1419da8b0
    Replacement::draft(0x1419da9b0, "unknown_1419da9b0", tpl_34871::<0x14020d4a0, 0x143a244eb, 0x143a244e8, 0x143a244f0, 0x143a244ec, 0x1401e8cc0, 0x1401e8c00> as *const ()), // sub_1419da9b0
    Replacement::draft(0x1419daaf0, "unknown_1419daaf0", fn_1419daaf0 as *const ()), // sub_1419daaf0
    Replacement::draft(0x1419dab40, "unknown_1419dab40", fn_1419dab40 as *const ()), // sub_1419dab40
    Replacement::draft(0x1419dabe0, "unknown_1419dabe0", fn_1419dabe0 as *const ()), // sub_1419dabe0
    Replacement::draft(0x1419dac50, "unknown_1419dac50", fn_1419dac50 as *const ()), // sub_1419dac50
    Replacement::draft(0x1419dac80, "unknown_1419dac80", fn_1419dac80 as *const ()), // sub_1419dac80
    Replacement::draft(0x1419daca0, "unknown_1419daca0", fn_1419daca0 as *const ()), // sub_1419daca0
    Replacement::draft(0x1419dada0, "unknown_1419dada0", fn_1419dada0 as *const ()), // sub_1419dada0
    Replacement::draft(0x1419dadf0, "unknown_1419dadf0", tpl_36272 as *const ()), // sub_1419dadf0
    Replacement::draft_throws(0x1419dae00, "unknown_1419dae00", fn_1419dae00 as *const (), Throws::Caught), // sub_1419dae00
    Replacement::draft(0x1419dcd70, "unknown_1419dcd70", fn_1419dcd70 as *const ()), // sub_1419dcd70
    Replacement::draft(0x1419dcf80, "unknown_1419dcf80", tpl_35056::<0x140463ed0> as *const ()), // sub_1419dcf80
    Replacement::draft(0x1419dcfa0, "unknown_1419dcfa0", tpl_35322 as *const ()), // sub_1419dcfa0
    Replacement::draft(0x1419dcfe0, "unknown_1419dcfe0", tpl_34994::<0x143ca2d3c, 0x14387b290, 0x14387b288, 0x141834a78, 0x1425e92c0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419dcfe0
    Replacement::draft(0x1419dd050, "unknown_1419dd050", fn_1419dd050 as *const ()), // sub_1419dd050
    Replacement::draft(0x1419dd240, "unknown_1419dd240", fn_1419dd240 as *const ()), // sub_1419dd240
    Replacement::draft(0x1419dd260, "unknown_1419dd260", fn_1419dd260 as *const ()), // sub_1419dd260
    Replacement::draft(0x1419dd2c0, "unknown_1419dd2c0", tpl_34318 as *const ()), // sub_1419dd2c0
    Replacement::draft(0x1419dd2e0, "unknown_1419dd2e0", tpl_34319::<0x143c91064, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd2e0
    Replacement::draft(0x1419dd340, "unknown_1419dd340", tpl_36277::<0x8> as *const ()), // sub_1419dd340
    Replacement::draft(0x1419dd360, "unknown_1419dd360", tpl_369 as *const ()), // sub_1419dd360
    Replacement::draft(0x1419dd370, "unknown_1419dd370", tpl_36278::<0x8> as *const ()), // sub_1419dd370
    Replacement::draft(0x1419dd380, "unknown_1419dd380", tpl_34318 as *const ()), // sub_1419dd380
    Replacement::draft(0x1419dd3a0, "unknown_1419dd3a0", tpl_34318 as *const ()), // sub_1419dd3a0
    Replacement::draft(0x1419dd3c0, "unknown_1419dd3c0", tpl_361 as *const ()), // sub_1419dd3c0
    Replacement::draft(0x1419dd3d0, "unknown_1419dd3d0", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd3d0
    Replacement::draft(0x1419dd430, "unknown_1419dd430", tpl_34319::<0x143ca2d94, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd430
    Replacement::draft(0x1419dd490, "unknown_1419dd490", tpl_35063::<0x143c8d694, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_1419dd490
    Replacement::draft(0x1419dd4e0, "unknown_1419dd4e0", tpl_34318 as *const ()), // sub_1419dd4e0
    Replacement::draft(0x1419dd500, "unknown_1419dd500", tpl_34318 as *const ()), // sub_1419dd500
    Replacement::draft(0x1419dd520, "unknown_1419dd520", tpl_34318 as *const ()), // sub_1419dd520
    Replacement::draft(0x1419dd540, "unknown_1419dd540", tpl_34318 as *const ()), // sub_1419dd540
    Replacement::draft(0x1419dd560, "unknown_1419dd560", tpl_34318 as *const ()), // sub_1419dd560
    Replacement::draft(0x1419dd580, "unknown_1419dd580", tpl_34318 as *const ()), // sub_1419dd580
    Replacement::draft(0x1419dd5a0, "unknown_1419dd5a0", tpl_34318 as *const ()), // sub_1419dd5a0
    Replacement::draft(0x1419dd5c0, "unknown_1419dd5c0", tpl_34318 as *const ()), // sub_1419dd5c0
    Replacement::draft(0x1419dd5e0, "unknown_1419dd5e0", tpl_34318 as *const ()), // sub_1419dd5e0
    Replacement::draft(0x1419dd600, "unknown_1419dd600", tpl_34318 as *const ()), // sub_1419dd600
    Replacement::draft(0x1419dd620, "unknown_1419dd620", tpl_34318 as *const ()), // sub_1419dd620
    Replacement::draft(0x1419dd640, "unknown_1419dd640", tpl_34318 as *const ()), // sub_1419dd640
    Replacement::draft(0x1419dd660, "unknown_1419dd660", tpl_34319::<0x143c96b24, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd660
    Replacement::draft(0x1419dd6c0, "unknown_1419dd6c0", tpl_34319::<0x143c8e660, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd6c0
    Replacement::draft(0x1419dd720, "unknown_1419dd720", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd720
    Replacement::draft(0x1419dd780, "unknown_1419dd780", tpl_34319::<0x143c93b20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd780
    Replacement::draft(0x1419dd7e0, "unknown_1419dd7e0", tpl_34319::<0x143c90198, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd7e0
    Replacement::draft(0x1419dd840, "unknown_1419dd840", tpl_34319::<0x143c8d508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd840
    Replacement::draft(0x1419dd8a0, "unknown_1419dd8a0", tpl_34319::<0x143c97e70, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd8a0
    Replacement::draft(0x1419dd900, "unknown_1419dd900", tpl_34319::<0x143c96dec, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd900
    Replacement::draft(0x1419dd960, "unknown_1419dd960", tpl_34319::<0x143c8ec28, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd960
    Replacement::draft(0x1419dd9c0, "unknown_1419dd9c0", tpl_34319::<0x143c97a20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dd9c0
    Replacement::draft(0x1419dda20, "unknown_1419dda20", tpl_34319::<0x143c8dbd0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dda20
    Replacement::draft(0x1419dda80, "unknown_1419dda80", tpl_34319::<0x143c8ec68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419dda80
    Replacement::draft(0x1419ddae0, "unknown_1419ddae0", tpl_34318 as *const ()), // sub_1419ddae0
    Replacement::draft(0x1419ddb00, "unknown_1419ddb00", tpl_34318 as *const ()), // sub_1419ddb00
    Replacement::draft(0x1419ddb20, "unknown_1419ddb20", tpl_34318 as *const ()), // sub_1419ddb20
    Replacement::draft(0x1419ddb40, "unknown_1419ddb40", tpl_34318 as *const ()), // sub_1419ddb40
    Replacement::draft(0x1419ddb60, "unknown_1419ddb60", tpl_34319::<0x143c8dbc4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ddb60
    Replacement::draft(0x1419ddbc0, "unknown_1419ddbc0", tpl_34319::<0x143c8e670, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ddbc0
    Replacement::draft(0x1419ddc20, "unknown_1419ddc20", tpl_34319::<0x143c8e70c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ddc20
    Replacement::draft(0x1419ddc80, "unknown_1419ddc80", tpl_34319::<0x143c8ec6c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ddc80
    Replacement::draft(0x1419ddce0, "unknown_1419ddce0", tpl_36279::<0x8> as *const ()), // sub_1419ddce0
    Replacement::draft_throws(0x1419ddd00, "unknown_1419ddd00", fn_1419ddd00 as *const (), Throws::Discard), // sub_1419ddd00
    Replacement::draft_throws(0x1419ddd30, "unknown_1419ddd30", fn_1419ddd30 as *const (), Throws::Discard), // sub_1419ddd30
    Replacement::draft(0x1419ddd70, "unknown_1419ddd70", tpl_36282 as *const ()), // sub_1419ddd70
    Replacement::draft(0x1419ddd90, "unknown_1419ddd90", tpl_361 as *const ()), // sub_1419ddd90
    Replacement::draft(0x1419ddda0, "unknown_1419ddda0", tpl_36282 as *const ()), // sub_1419ddda0
    Replacement::draft(0x1419dddc0, "unknown_1419dddc0", tpl_361 as *const ()), // sub_1419dddc0
    Replacement::draft_throws(0x1419dddd0, "unknown_1419dddd0", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_1419dddd0
    Replacement::draft(0x1419dddf0, "unknown_1419dddf0", tpl_36282 as *const ()), // sub_1419dddf0
    Replacement::draft(0x1419dde10, "unknown_1419dde10", tpl_3548::<0x10, 0x19, 0x8> as *const ()), // sub_1419dde10
    Replacement::draft(0x1419dde70, "unknown_1419dde70", tpl_3547::<0x10, 0x19, 0x8> as *const ()), // sub_1419dde70
    Replacement::draft(0x1419dded0, "unknown_1419dded0", tpl_361 as *const ()), // sub_1419dded0
    Replacement::draft_throws(0x1419ddee0, "unknown_1419ddee0", tpl_3740::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_1419ddee0
    Replacement::draft_throws(0x1419ddf00, "unknown_1419ddf00", tpl_3740::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_1419ddf00
    Replacement::draft_throws(0x1419ddf20, "unknown_1419ddf20", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_1419ddf20
    Replacement::draft_throws(0x1419ddf40, "unknown_1419ddf40", tpl_36284::<0x8, 0x140200130, 0x1419ddf5e> as *const (), Throws::Discard), // sub_1419ddf40
    Replacement::draft(0x1419ddf70, "unknown_1419ddf70", tpl_361 as *const ()), // sub_1419ddf70
    Replacement::draft(0x1419ddf80, "unknown_1419ddf80", tpl_895::<0x8> as *const ()), // sub_1419ddf80
    Replacement::draft_throws(0x1419de210, "unknown_1419de210", fn_1419de210 as *const (), Throws::Discard), // sub_1419de210
    Replacement::draft(0x1419de230, "unknown_1419de230", fn_1419de230 as *const ()), // sub_1419de230
    Replacement::draft(0x1419de260, "unknown_1419de260", tpl_36287 as *const ()), // sub_1419de260
    Replacement::draft(0x1419de280, "unknown_1419de280", tpl_35683 as *const ()), // sub_1419de280
    Replacement::draft(0x1419de290, "unknown_1419de290", tpl_361 as *const ()), // sub_1419de290
    Replacement::draft(0x1419de2a0, "unknown_1419de2a0", tpl_35675::<0x8> as *const ()), // sub_1419de2a0
    Replacement::draft_throws(0x1419de2c0, "unknown_1419de2c0", tpl_3740::<0x8, 0x14241c190> as *const (), Throws::Discard), // sub_1419de2c0
    Replacement::draft_throws(0x1419de2e0, "unknown_1419de2e0", fn_1419de2e0 as *const (), Throws::Discard), // sub_1419de2e0
    Replacement::draft(0x1419de300, "unknown_1419de300", tpl_361 as *const ()), // sub_1419de300
    Replacement::draft(0x1419de310, "unknown_1419de310", tpl_36289::<0x8> as *const ()), // sub_1419de310
    Replacement::draft(0x1419de330, "unknown_1419de330", tpl_34318 as *const ()), // sub_1419de330
    Replacement::draft(0x1419de350, "unknown_1419de350", tpl_34319::<0x143ca2970, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419de350
    Replacement::draft_throws(0x1419de3b0, "unknown_1419de3b0", fn_1419de3b0 as *const (), Throws::Discard), // sub_1419de3b0
    Replacement::draft(0x1419de3f0, "unknown_1419de3f0", tpl_361 as *const ()), // sub_1419de3f0
    Replacement::draft_throws(0x1419de400, "unknown_1419de400", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_1419de400
    Replacement::draft(0x1419de420, "unknown_1419de420", tpl_3047::<0x8> as *const ()), // sub_1419de420
    Replacement::draft(0x1419de430, "unknown_1419de430", fn_1419de430 as *const ()), // sub_1419de430
    Replacement::draft(0x1419de440, "unknown_1419de440", tpl_361 as *const ()), // sub_1419de440
    Replacement::draft_throws(0x1419de450, "unknown_1419de450", tpl_36292::<0x8, 0x38, 0x14241c0a0, 0x10, 0x18> as *const (), Throws::Discard), // sub_1419de450
    Replacement::draft_throws(0x1419de4b0, "unknown_1419de4b0", tpl_36293::<0xc, 0x8, 0x14241c190> as *const (), Throws::Discard), // sub_1419de4b0
    Replacement::draft_throws(0x1419de560, "unknown_1419de560", tpl_3736::<0x8, 0x19, 0x10, 0x14045fd20, 0x14241c190> as *const (), Throws::Discard), // sub_1419de560
    Replacement::draft(0x1419de5e0, "unknown_1419de5e0", tpl_36294::<0x8, 0x19, 0x1c, 0x4, 0x20, 0x10> as *const ()), // sub_1419de5e0
    Replacement::draft_throws(0x1419de660, "unknown_1419de660", tpl_36292::<0x8, 0x38, 0x14241c0a0, 0x10, 0x18> as *const (), Throws::Discard), // sub_1419de660
    Replacement::draft_throws(0x1419de6c0, "unknown_1419de6c0", tpl_1148::<0x8, 0xc, 0x14241c190> as *const (), Throws::Discard), // sub_1419de6c0
    Replacement::draft_throws(0x1419de700, "unknown_1419de700", fn_1419de700 as *const (), Throws::Discard), // sub_1419de700
    Replacement::draft(0x1419de810, "unknown_1419de810", fn_1419de810 as *const ()), // sub_1419de810
    Replacement::draft(0x1419deaf0, "unknown_1419deaf0", fn_1419deaf0 as *const ()), // sub_1419deaf0
    Replacement::draft_throws(0x1419ded20, "unknown_1419ded20", fn_1419ded20 as *const (), Throws::Caught), // sub_1419ded20
    Replacement::draft_throws(0x1419deef0, "unknown_1419deef0", fn_1419deef0 as *const (), Throws::Caught), // sub_1419deef0
    Replacement::draft(0x1419df1a0, "unknown_1419df1a0", fn_1419df1a0 as *const ()), // sub_1419df1a0
    Replacement::draft(0x1419df5e0, "unknown_1419df5e0", fn_1419df5e0 as *const ()), // sub_1419df5e0
    Replacement::draft(0x1419dfdb0, "unknown_1419dfdb0", fn_1419dfdb0 as *const ()), // sub_1419dfdb0
    Replacement::draft(0x1419dfdf0, "unknown_1419dfdf0", fn_1419dfdf0 as *const ()), // sub_1419dfdf0
    Replacement::draft(0x1419e0210, "unknown_1419e0210", fn_1419e0210 as *const ()), // sub_1419e0210
    Replacement::draft(0x1419e0260, "unknown_1419e0260", fn_1419e0260 as *const ()), // sub_1419e0260
    Replacement::draft(0x1419e0680, "unknown_1419e0680", fn_1419e0680 as *const ()), // sub_1419e0680
    Replacement::draft(0x1419e07a0, "unknown_1419e07a0", fn_1419e07a0 as *const ()), // sub_1419e07a0
    Replacement::draft(0x1419e08e0, "unknown_1419e08e0", fn_1419e08e0 as *const ()), // sub_1419e08e0
    Replacement::draft(0x1419e0d40, "unknown_1419e0d40", fn_1419e0d40 as *const ()), // sub_1419e0d40
    Replacement::draft_throws(0x1419e0d50, "unknown_1419e0d50", fn_1419e0d50 as *const (), Throws::Caught), // sub_1419e0d50
    Replacement::draft_throws(0x1419e10e0, "unknown_1419e10e0", fn_1419e10e0 as *const (), Throws::Caught), // sub_1419e10e0
    Replacement::draft(0x1419e1200, "unknown_1419e1200", fn_1419e1200 as *const ()), // sub_1419e1200
    Replacement::draft(0x1419e1240, "unknown_1419e1240", fn_1419e1240 as *const ()), // sub_1419e1240
    Replacement::draft_throws(0x1419e12c0, "unknown_1419e12c0", fn_1419e12c0 as *const (), Throws::Caught), // sub_1419e12c0
    Replacement::draft(0x1419e1370, "unknown_1419e1370", fn_1419e1370 as *const ()), // sub_1419e1370
    Replacement::draft(0x1419e1500, "unknown_1419e1500", fn_1419e1500 as *const ()), // sub_1419e1500
    Replacement::draft(0x1419e1560, "unknown_1419e1560", fn_1419e1560 as *const ()), // sub_1419e1560
    Replacement::draft(0x1419e1980, "unknown_1419e1980", fn_1419e1980 as *const ()), // sub_1419e1980
    Replacement::draft(0x1419e1a70, "unknown_1419e1a70", fn_1419e1a70 as *const ()), // sub_1419e1a70
    Replacement::draft(0x1419e1ab0, "unknown_1419e1ab0", fn_1419e1ab0 as *const ()), // sub_1419e1ab0
    Replacement::draft(0x1419e1b00, "unknown_1419e1b00", fn_1419e1b00 as *const ()), // sub_1419e1b00
    Replacement::draft(0x1419e1b50, "unknown_1419e1b50", fn_1419e1b50 as *const ()), // sub_1419e1b50
    Replacement::draft(0x1419e1bc0, "unknown_1419e1bc0", fn_1419e1bc0 as *const ()), // sub_1419e1bc0
    Replacement::draft(0x1419e1c30, "unknown_1419e1c30", fn_1419e1c30 as *const ()), // sub_1419e1c30
    Replacement::draft(0x1419e1c50, "unknown_1419e1c50", fn_1419e1c50 as *const ()), // sub_1419e1c50
    Replacement::draft(0x1419e1c80, "unknown_1419e1c80", fn_1419e1c80 as *const ()), // sub_1419e1c80
    Replacement::draft(0x1419e1ce0, "unknown_1419e1ce0", fn_1419e1ce0 as *const ()), // sub_1419e1ce0
    Replacement::draft(0x1419e1d20, "unknown_1419e1d20", fn_1419e1d20 as *const ()), // sub_1419e1d20
    Replacement::draft(0x1419e1d70, "unknown_1419e1d70", fn_1419e1d70 as *const ()), // sub_1419e1d70
    Replacement::draft(0x1419e1dd0, "unknown_1419e1dd0", fn_1419e1dd0 as *const ()), // sub_1419e1dd0
    Replacement::draft(0x1419e1e30, "unknown_1419e1e30", fn_1419e1e30 as *const ()), // sub_1419e1e30
    Replacement::draft(0x1419e1e80, "unknown_1419e1e80", fn_1419e1e80 as *const ()), // sub_1419e1e80
    Replacement::draft(0x1419e1f70, "unknown_1419e1f70", fn_1419e1f70 as *const ()), // sub_1419e1f70
    Replacement::draft(0x1419e2050, "unknown_1419e2050", fn_1419e2050 as *const ()), // sub_1419e2050
    Replacement::draft_throws(0x1419e2200, "unknown_1419e2200", fn_1419e2200 as *const (), Throws::Caught), // sub_1419e2200
    Replacement::draft(0x1419e2350, "unknown_1419e2350", fn_1419e2350 as *const ()), // sub_1419e2350
    Replacement::draft(0x1419e2410, "unknown_1419e2410", fn_1419e2410 as *const ()), // sub_1419e2410
    Replacement::draft(0x1419e2500, "unknown_1419e2500", fn_1419e2500 as *const ()), // sub_1419e2500
    Replacement::draft(0x1419e25a0, "unknown_1419e25a0", fn_1419e25a0 as *const ()), // sub_1419e25a0
    Replacement::draft(0x1419e2640, "unknown_1419e2640", fn_1419e2640 as *const ()), // sub_1419e2640
    Replacement::draft(0x1419e26d0, "unknown_1419e26d0", fn_1419e26d0 as *const ()), // sub_1419e26d0
    Replacement::draft(0x1419e27b0, "unknown_1419e27b0", fn_1419e27b0 as *const ()), // sub_1419e27b0
    Replacement::draft(0x1419e2870, "unknown_1419e2870", fn_1419e2870 as *const ()), // sub_1419e2870
    Replacement::draft(0x1419e29d0, "unknown_1419e29d0", tpl_36339::<0x18, 0xa8, 0xb0, 0x4c> as *const ()), // sub_1419e29d0
    Replacement::draft(0x1419e2a20, "unknown_1419e2a20", fn_1419e2a20 as *const ()), // sub_1419e2a20
    Replacement::draft(0x1419e2ac0, "unknown_1419e2ac0", fn_1419e2ac0 as *const ()), // sub_1419e2ac0
    Replacement::draft(0x1419e2c10, "unknown_1419e2c10", fn_1419e2c10 as *const ()), // sub_1419e2c10
    Replacement::draft_throws(0x1419e2c70, "unknown_1419e2c70", fn_1419e2c70 as *const (), Throws::Discard), // sub_1419e2c70
    Replacement::draft(0x1419e3210, "unknown_1419e3210", fn_1419e3210 as *const ()), // sub_1419e3210
    Replacement::draft(0x1419e3270, "unknown_1419e3270", fn_1419e3270 as *const ()), // sub_1419e3270
    Replacement::draft_throws(0x1419e32d0, "unknown_1419e32d0", fn_1419e32d0 as *const (), Throws::Caught), // sub_1419e32d0
    Replacement::draft(0x1419e3380, "unknown_1419e3380", fn_1419e3380 as *const ()), // sub_1419e3380
    Replacement::draft_throws(0x1419e33d0, "unknown_1419e33d0", fn_1419e33d0 as *const (), Throws::Discard), // sub_1419e33d0
    Replacement::draft(0x1419e3580, "unknown_1419e3580", tpl_36339::<0x18, 0xa8, 0xb0, 0x4c> as *const ()), // sub_1419e3580
    Replacement::draft_throws(0x1419e35d0, "unknown_1419e35d0", fn_1419e35d0 as *const (), Throws::Caught), // sub_1419e35d0
    Replacement::draft(0x1419e3af0, "unknown_1419e3af0", fn_1419e3af0 as *const ()), // sub_1419e3af0
    Replacement::draft(0x1419e3b50, "unknown_1419e3b50", fn_1419e3b50 as *const ()), // sub_1419e3b50
    Replacement::draft(0x1419e3b90, "unknown_1419e3b90", fn_1419e3b90 as *const ()), // sub_1419e3b90
    Replacement::draft(0x1419e3c80, "unknown_1419e3c80", fn_1419e3c80 as *const ()), // sub_1419e3c80
    Replacement::draft_throws(0x1419e3d20, "unknown_1419e3d20", fn_1419e3d20 as *const (), Throws::Discard), // sub_1419e3d20
    Replacement::draft_throws(0x1419e3e90, "unknown_1419e3e90", fn_1419e3e90 as *const (), Throws::Discard), // sub_1419e3e90
    Replacement::draft(0x1419e4020, "unknown_1419e4020", fn_1419e4020 as *const ()), // sub_1419e4020
    Replacement::draft(0x1419e40e0, "unknown_1419e40e0", fn_1419e40e0 as *const ()), // sub_1419e40e0
    Replacement::draft_throws(0x1419e4310, "unknown_1419e4310", fn_1419e4310 as *const (), Throws::Caught), // sub_1419e4310
    Replacement::draft_throws(0x1419e4510, "unknown_1419e4510", fn_1419e4510 as *const (), Throws::Caught), // sub_1419e4510
    Replacement::draft_throws(0x1419e45c0, "unknown_1419e45c0", fn_1419e45c0 as *const (), Throws::Discard), // sub_1419e45c0
    Replacement::draft(0x1419e4aa0, "unknown_1419e4aa0", tpl_34393 as *const ()), // sub_1419e4aa0
    Replacement::draft(0x1419e4be0, "unknown_1419e4be0", tpl_34613 as *const ()), // sub_1419e4be0
    Replacement::draft(0x1419e4c00, "unknown_1419e4c00", tpl_34613 as *const ()), // sub_1419e4c00
    Replacement::draft(0x1419e4d10, "unknown_1419e4d10", tpl_34612 as *const ()), // sub_1419e4d10
    Replacement::draft(0x1419e4da0, "unknown_1419e4da0", tpl_34605::<0x14313e820> as *const ()), // sub_1419e4da0
    Replacement::draft(0x1419e4e30, "unknown_1419e4e30", tpl_36123 as *const ()), // sub_1419e4e30
    Replacement::draft(0x1419e4e70, "unknown_1419e4e70", tpl_34994::<0x143ca38fc, 0x14387b2d0, 0x14387b2c8, 0x141834a78, 0x1425e9810, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419e4e70
    Replacement::draft(0x1419e4ee0, "unknown_1419e4ee0", tpl_94771::<0x1401ff5b0> as *const ()), // sub_1419e4ee0
    Replacement::draft(0x1419e4f80, "unknown_1419e4f80", tpl_36358::<0x1401ff5b0> as *const ()), // sub_1419e4f80
    Replacement::draft(0x1419e4fa0, "unknown_1419e4fa0", tpl_34684::<0x1401f2bc0> as *const ()), // sub_1419e4fa0
    Replacement::draft(0x1419e4fe0, "unknown_1419e4fe0", tpl_34835::<0x8, 0x14045bb70, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419e4fe0
    Replacement::draft(0x1419e5060, "unknown_1419e5060", tpl_36359::<0x8, 0x14045bb70, 0x20, 0x3, 0x4, 0x14240aa00, 0x48, 0x14024d8b0, 0x10, 0x18, 0x14024ed60> as *const ()), // sub_1419e5060
    Replacement::draft(0x1419e5110, "unknown_1419e5110", tpl_34837::<0x14045bb70, 0x20, 0x14240aa00> as *const ()), // sub_1419e5110
    Replacement::draft(0x1419e5160, "unknown_1419e5160", tpl_36360::<0x14045bb70, 0x20, 0x14240aa00, 0x48, 0x14046dc30> as *const ()), // sub_1419e5160
    Replacement::draft(0x1419e51c0, "unknown_1419e51c0", tpl_34840::<0x14045ad90, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1419e51c0
    Replacement::draft(0x1419e5230, "unknown_1419e5230", tpl_36361::<0x14045ad90, 0x20, 0x3, 0x4, 0x8, 0x14240a730, 0x48, 0x50, 0x14024d8b0, 0x10, 0x18> as *const ()), // sub_1419e5230
    Replacement::draft(0x1419e52e0, "unknown_1419e52e0", tpl_34318 as *const ()), // sub_1419e52e0
    Replacement::draft(0x1419e5300, "unknown_1419e5300", tpl_34318 as *const ()), // sub_1419e5300
    Replacement::draft(0x1419e5320, "unknown_1419e5320", tpl_34318 as *const ()), // sub_1419e5320
    Replacement::draft(0x1419e5340, "unknown_1419e5340", tpl_34319::<0x143c8d46c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5340
    Replacement::draft(0x1419e53a0, "unknown_1419e53a0", tpl_34319::<0x143c8d4b4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e53a0
    Replacement::draft(0x1419e5400, "unknown_1419e5400", tpl_34319::<0x143ca36f4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5400
    Replacement::draft(0x1419e5460, "unknown_1419e5460", tpl_34318 as *const ()), // sub_1419e5460
    Replacement::draft(0x1419e5480, "unknown_1419e5480", tpl_34319::<0x143c8dbc4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5480
    Replacement::draft(0x1419e54e0, "unknown_1419e54e0", tpl_34318 as *const ()), // sub_1419e54e0
    Replacement::draft(0x1419e5500, "unknown_1419e5500", tpl_34319::<0x143c93228, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5500
    Replacement::draft(0x1419e5560, "unknown_1419e5560", tpl_34318 as *const ()), // sub_1419e5560
    Replacement::draft(0x1419e5580, "unknown_1419e5580", tpl_34319::<0x143ca38f8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5580
    Replacement::draft(0x1419e55e0, "unknown_1419e55e0", tpl_361 as *const ()), // sub_1419e55e0
    Replacement::draft(0x1419e55f0, "unknown_1419e55f0", tpl_34318 as *const ()), // sub_1419e55f0
    Replacement::draft(0x1419e5610, "unknown_1419e5610", tpl_34318 as *const ()), // sub_1419e5610
    Replacement::draft(0x1419e5630, "unknown_1419e5630", tpl_35063::<0x143c8d528, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_1419e5630
    Replacement::draft(0x1419e5680, "unknown_1419e5680", tpl_34319::<0x143c8d478, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5680
    Replacement::draft(0x1419e56e0, "unknown_1419e56e0", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e56e0
    Replacement::draft(0x1419e5740, "unknown_1419e5740", tpl_34318 as *const ()), // sub_1419e5740
    Replacement::draft(0x1419e5760, "unknown_1419e5760", tpl_34319::<0x143c90198, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5760
    Replacement::draft(0x1419e57c0, "unknown_1419e57c0", tpl_34318 as *const ()), // sub_1419e57c0
    Replacement::draft(0x1419e57e0, "unknown_1419e57e0", tpl_34318 as *const ()), // sub_1419e57e0
    Replacement::draft(0x1419e5800, "unknown_1419e5800", tpl_34318 as *const ()), // sub_1419e5800
    Replacement::draft(0x1419e5820, "unknown_1419e5820", tpl_361 as *const ()), // sub_1419e5820
    Replacement::draft(0x1419e5830, "unknown_1419e5830", tpl_34319::<0x143c97794, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5830
    Replacement::draft(0x1419e5890, "unknown_1419e5890", tpl_34319::<0x143c96de4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5890
    Replacement::draft(0x1419e58f0, "unknown_1419e58f0", tpl_34319::<0x143c8eb90, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e58f0
    Replacement::draft(0x1419e5950, "unknown_1419e5950", tpl_35063::<0x143ca3424, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_1419e5950
    Replacement::draft(0x1419e59a0, "unknown_1419e59a0", tpl_34318 as *const ()), // sub_1419e59a0
    Replacement::draft(0x1419e59c0, "unknown_1419e59c0", tpl_34318 as *const ()), // sub_1419e59c0
    Replacement::draft(0x1419e59e0, "unknown_1419e59e0", tpl_34318 as *const ()), // sub_1419e59e0
    Replacement::draft(0x1419e5a00, "unknown_1419e5a00", tpl_34318 as *const ()), // sub_1419e5a00
    Replacement::draft(0x1419e5a20, "unknown_1419e5a20", tpl_34318 as *const ()), // sub_1419e5a20
    Replacement::draft(0x1419e5a40, "unknown_1419e5a40", tpl_34318 as *const ()), // sub_1419e5a40
    Replacement::draft(0x1419e5a60, "unknown_1419e5a60", tpl_34318 as *const ()), // sub_1419e5a60
    Replacement::draft(0x1419e5a80, "unknown_1419e5a80", tpl_34319::<0x143ca37d4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5a80
    Replacement::draft(0x1419e5ae0, "unknown_1419e5ae0", tpl_34319::<0x143ca34f4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5ae0
    Replacement::draft(0x1419e5b40, "unknown_1419e5b40", tpl_34319::<0x143c964c0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5b40
    Replacement::draft(0x1419e5ba0, "unknown_1419e5ba0", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5ba0
    Replacement::draft(0x1419e5c00, "unknown_1419e5c00", tpl_34319::<0x143c8d518, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5c00
    Replacement::draft(0x1419e5c60, "unknown_1419e5c60", tpl_34319::<0x143ca365c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5c60
    Replacement::draft(0x1419e5cc0, "unknown_1419e5cc0", tpl_34319::<0x143c8d68c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5cc0
    Replacement::draft(0x1419e5d20, "unknown_1419e5d20", tpl_34318 as *const ()), // sub_1419e5d20
    Replacement::draft(0x1419e5d40, "unknown_1419e5d40", tpl_34319::<0x143ca3428, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5d40
    Replacement::draft(0x1419e5da0, "unknown_1419e5da0", tpl_34318 as *const ()), // sub_1419e5da0
    Replacement::draft(0x1419e5dc0, "unknown_1419e5dc0", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5dc0
    Replacement::draft(0x1419e5e20, "unknown_1419e5e20", tpl_34318 as *const ()), // sub_1419e5e20
    Replacement::draft(0x1419e5e40, "unknown_1419e5e40", tpl_34318 as *const ()), // sub_1419e5e40
    Replacement::draft(0x1419e5e60, "unknown_1419e5e60", tpl_34319::<0x143c9d9fc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5e60
    Replacement::draft(0x1419e5ec0, "unknown_1419e5ec0", tpl_34319::<0x143c92ee8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5ec0
    Replacement::draft(0x1419e5f20, "unknown_1419e5f20", tpl_34318 as *const ()), // sub_1419e5f20
    Replacement::draft(0x1419e5f40, "unknown_1419e5f40", tpl_34318 as *const ()), // sub_1419e5f40
    Replacement::draft(0x1419e5f60, "unknown_1419e5f60", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5f60
    Replacement::draft(0x1419e5fc0, "unknown_1419e5fc0", tpl_34319::<0x143c8d708, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419e5fc0
    Replacement::draft_throws(0x1419e6020, "unknown_1419e6020", tpl_34846::<0x1401f3a20, 0x1419e60a0> as *const (), Throws::Discard), // sub_1419e6020
    Replacement::draft_throws(0x1419e60a0, "unknown_1419e60a0", tpl_96039::<0x14043c3b0, 0x14309c488, 0x14272bcd8, 0x14272bc88, 0x14272bcb4, 0x14272bcc0, 0x14272bce0, 0x14272bcac, 0x14272bccc, 0x140254aa0, 0x1424b16a0, 0x140448cc0> as *const (), Throws::Discard), // sub_1419e60a0
    Replacement::draft(0x1419e6530, "unknown_1419e6530", tpl_36362::<0x50, 0x48, 0x19, 0x28, 0x10, 0x8, 0x1> as *const ()), // sub_1419e6530
    Replacement::draft_throws(0x1419e65c0, "unknown_1419e65c0", tpl_36363::<0x48, 0x28, 0x10, 0x19, 0x8, 0x140466950> as *const (), Throws::Discard), // sub_1419e65c0
    Replacement::draft_throws(0x1419e6680, "unknown_1419e6680", fn_1419e6680 as *const (), Throws::Caught), // sub_1419e6680
    Replacement::draft_throws(0x1419e6880, "unknown_1419e6880", tpl_36186::<0x18, 0x1419e60a0, 0x10> as *const (), Throws::Discard), // sub_1419e6880
    Replacement::draft(0x1419e68c0, "unknown_1419e68c0", tpl_96237::<0x1403f7860, 0x1401e66f0> as *const ()), // sub_1419e68c0
    Replacement::draft_throws(0x1419e6a00, "unknown_1419e6a00", tpl_36364::<0x1419e6530, 0x140469cd0> as *const (), Throws::Discard), // sub_1419e6a00
    Replacement::draft_throws(0x1419e6b10, "unknown_1419e6b10", tpl_36365::<0x140466950> as *const (), Throws::Discard), // sub_1419e6b10
    Replacement::draft_throws(0x1419e6c60, "unknown_1419e6c60", tpl_36366::<0x1419e6530, 0x1401f3a20, 0x1419e60a0> as *const (), Throws::Discard), // sub_1419e6c60
    Replacement::draft(0x1419e6ce0, "unknown_1419e6ce0", tpl_36367::<0x140246920, 0x20, 0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_1419e6ce0
    Replacement::draft(0x1419e6d20, "unknown_1419e6d20", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_1419e6d20
    Replacement::draft(0x1419e6d50, "unknown_1419e6d50", tpl_361 as *const ()), // sub_1419e6d50
    Replacement::draft_throws(0x1419e6d60, "unknown_1419e6d60", fn_1419e6d60 as *const (), Throws::Caught), // sub_1419e6d60
    Replacement::draft(0x1419e8180, "unknown_1419e8180", tpl_361 as *const ()), // sub_1419e8180
    Replacement::draft_throws(0x1419e8190, "unknown_1419e8190", fn_1419e8190 as *const (), Throws::Discard), // sub_1419e8190
    Replacement::draft_throws(0x1419e8410, "unknown_1419e8410", fn_1419e8410 as *const (), Throws::Discard), // sub_1419e8410
    Replacement::draft(0x1419e86a0, "unknown_1419e86a0", fn_1419e86a0 as *const ()), // sub_1419e86a0
    Replacement::draft_throws(0x1419e8710, "unknown_1419e8710", fn_1419e8710 as *const (), Throws::Caught), // sub_1419e8710
    Replacement::draft(0x1419e8a70, "unknown_1419e8a70", tpl_36371::<0x10> as *const ()), // sub_1419e8a70
    Replacement::draft(0x1419e8aa0, "unknown_1419e8aa0", tpl_36372 as *const ()), // sub_1419e8aa0
    Replacement::draft(0x1419e8ac0, "unknown_1419e8ac0", fn_1419e8ac0 as *const ()), // sub_1419e8ac0
    Replacement::draft(0x1419e8b90, "unknown_1419e8b90", fn_1419e8b90 as *const ()), // sub_1419e8b90
    Replacement::draft_throws(0x1419e8cf0, "unknown_1419e8cf0", fn_1419e8cf0 as *const (), Throws::Caught), // sub_1419e8cf0
    Replacement::draft(0x1419e8e50, "unknown_1419e8e50", fn_1419e8e50 as *const ()), // sub_1419e8e50
    Replacement::draft(0x1419e8ec0, "unknown_1419e8ec0", tpl_361 as *const ()), // sub_1419e8ec0
    Replacement::draft(0x1419e8ed0, "unknown_1419e8ed0", tpl_361 as *const ()), // sub_1419e8ed0
    Replacement::draft_throws(0x1419e8ee0, "unknown_1419e8ee0", fn_1419e8ee0 as *const (), Throws::Discard), // sub_1419e8ee0
    Replacement::draft(0x1419e9150, "unknown_1419e9150", fn_1419e9150 as *const ()), // sub_1419e9150
    Replacement::draft(0x1419e92e0, "unknown_1419e92e0", tpl_361 as *const ()), // sub_1419e92e0
    Replacement::draft(0x1419e92f0, "unknown_1419e92f0", fn_1419e92f0 as *const ()), // sub_1419e92f0
    Replacement::draft(0x1419e95c0, "unknown_1419e95c0", tpl_361 as *const ()), // sub_1419e95c0
    Replacement::draft(0x1419e95d0, "unknown_1419e95d0", fn_1419e95d0 as *const ()), // sub_1419e95d0
    Replacement::draft(0x1419e9610, "unknown_1419e9610", tpl_361 as *const ()), // sub_1419e9610
    Replacement::draft_throws(0x1419e9620, "unknown_1419e9620", fn_1419e9620 as *const (), Throws::Discard), // sub_1419e9620
    Replacement::draft(0x1419e9d90, "unknown_1419e9d90", tpl_361 as *const ()), // sub_1419e9d90
    Replacement::draft(0x1419e9da0, "unknown_1419e9da0", tpl_361 as *const ()), // sub_1419e9da0
    Replacement::draft(0x1419e9db0, "unknown_1419e9db0", fn_1419e9db0 as *const ()), // sub_1419e9db0
    Replacement::draft(0x1419ea060, "unknown_1419ea060", tpl_361 as *const ()), // sub_1419ea060
    Replacement::draft(0x1419ea070, "unknown_1419ea070", fn_1419ea070 as *const ()), // sub_1419ea070
    Replacement::draft(0x1419ea150, "unknown_1419ea150", tpl_361 as *const ()), // sub_1419ea150
    Replacement::draft(0x1419ea160, "unknown_1419ea160", tpl_361 as *const ()), // sub_1419ea160
    Replacement::draft(0x1419ea170, "unknown_1419ea170", fn_1419ea170 as *const ()), // sub_1419ea170
    Replacement::draft(0x1419ea5d0, "unknown_1419ea5d0", tpl_361 as *const ()), // sub_1419ea5d0
    Replacement::draft(0x1419ea5e0, "unknown_1419ea5e0", fn_1419ea5e0 as *const ()), // sub_1419ea5e0
    Replacement::draft(0x1419ea620, "unknown_1419ea620", tpl_361 as *const ()), // sub_1419ea620
    Replacement::draft(0x1419ea630, "unknown_1419ea630", fn_1419ea630 as *const ()), // sub_1419ea630
    Replacement::draft(0x1419ea6e0, "unknown_1419ea6e0", tpl_36383::<0x8, 0x140466b00, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1419ea6e0
    Replacement::draft(0x1419ea7c0, "unknown_1419ea7c0", tpl_496 as *const ()), // sub_1419ea7c0
    Replacement::draft(0x1419ea7d0, "unknown_1419ea7d0", tpl_496 as *const ()), // sub_1419ea7d0
    Replacement::draft(0x1419ea7e0, "unknown_1419ea7e0", fn_1419ea7e0 as *const ()), // sub_1419ea7e0
    Replacement::draft_throws(0x1419eaa20, "unknown_1419eaa20", fn_1419eaa20 as *const (), Throws::Caught), // sub_1419eaa20
    Replacement::draft(0x1419eb300, "unknown_1419eb300", tpl_361 as *const ()), // sub_1419eb300
    Replacement::draft(0x1419eb310, "unknown_1419eb310", tpl_34318 as *const ()), // sub_1419eb310
    Replacement::draft(0x1419eb330, "unknown_1419eb330", tpl_34318 as *const ()), // sub_1419eb330
    Replacement::draft(0x1419eb350, "unknown_1419eb350", tpl_35063::<0x143ca3424, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_1419eb350
    Replacement::draft(0x1419eb3a0, "unknown_1419eb3a0", tpl_34319::<0x143c917cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419eb3a0
    Replacement::draft(0x1419eb400, "unknown_1419eb400", tpl_34319::<0x143c8d478, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419eb400
    Replacement::draft(0x1419eb460, "unknown_1419eb460", tpl_36385::<0x141834d30> as *const ()), // sub_1419eb460
    Replacement::draft(0x1419eb480, "unknown_1419eb480", tpl_34318 as *const ()), // sub_1419eb480
    Replacement::draft(0x1419eb4a0, "unknown_1419eb4a0", tpl_34319::<0x143ca34f4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419eb4a0
    Replacement::draft(0x1419eb500, "unknown_1419eb500", tpl_36385::<0x141834d30> as *const ()), // sub_1419eb500
    Replacement::draft_throws(0x1419eb520, "std_1419eb520", fn_1419eb520 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<bigfish::detail::filterVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x1419eb570, "std_1419eb570", tpl_559 as *const ()), // std::_Func_impl_no_alloc<bigfish::detail::filterVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x1419eb580, "std_1419eb580", fn_1419eb580 as *const ()), // std::_Func_impl_no_alloc<bigfish::detail::filterVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x1419eb640, "std_1419eb640", tpl_1::<0x1438888b0> as *const ()), // std::_Func_impl_no_alloc<bigfish::detail::filterVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x1419eb650, "std_1419eb650", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::detail::filterVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x1419eb660, "std_1419eb660", fn_1419eb660 as *const ()), // std::_Func_impl_no_alloc<bigfish::detail::filterVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_i
    Replacement::draft(0x1419eb680, "std_1419eb680", tpl_34318 as *const ()), // sub_1419eb680
    Replacement::draft(0x1419eb6a0, "std_1419eb6a0", tpl_34318 as *const ()), // sub_1419eb6a0
    Replacement::draft(0x1419eb6c0, "std_1419eb6c0", tpl_34318 as *const ()), // sub_1419eb6c0
    Replacement::draft(0x1419eb6e0, "std_1419eb6e0", tpl_34318 as *const ()), // sub_1419eb6e0
    Replacement::draft(0x1419eb700, "std_1419eb700", tpl_34319::<0x143c964c0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419eb700
    Replacement::draft(0x1419eb760, "std_1419eb760", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419eb760
    Replacement::draft(0x1419eb7c0, "std_1419eb7c0", tpl_34319::<0x143c8d708, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419eb7c0
    Replacement::draft(0x1419eb820, "std_1419eb820", tpl_34319::<0x143c8d68c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419eb820
    Replacement::draft(0x1419eb880, "std_1419eb880", fn_1419eb880 as *const ()), // sub_1419eb880
    Replacement::draft(0x1419eb8e0, "std_1419eb8e0", tpl_361 as *const ()), // sub_1419eb8e0
    Replacement::draft(0x1419eba60, "std_1419eba60", tpl_36390::<0x18, 0x8, 0x10> as *const ()), // sub_1419eba60
    Replacement::draft(0x1419eba80, "std_1419eba80", fn_1419eba80 as *const ()), // sub_1419eba80
    Replacement::draft(0x1419ebcc0, "std_1419ebcc0", tpl_496 as *const ()), // sub_1419ebcc0
    Replacement::draft_throws(0x1419ebcd0, "std_1419ebcd0", fn_1419ebcd0 as *const (), Throws::Discard), // sub_1419ebcd0
    Replacement::draft(0x1419ec2c0, "std_1419ec2c0", fn_1419ec2c0 as *const ()), // sub_1419ec2c0
    Replacement::draft(0x1419ec380, "std_1419ec380", tpl_361 as *const ()), // sub_1419ec380
    Replacement::draft(0x1419ecc80, "std_1419ecc80", tpl_361 as *const ()), // sub_1419ecc80
    Replacement::draft(0x1419ecc90, "std_1419ecc90", fn_1419ecc90 as *const ()), // sub_1419ecc90
    Replacement::draft(0x1419eccc0, "std_1419eccc0", fn_1419eccc0 as *const ()), // sub_1419eccc0
    Replacement::draft(0x1419ecce0, "std_1419ecce0", tpl_36371::<0x10> as *const ()), // sub_1419ecce0
    Replacement::draft(0x1419ecd10, "std_1419ecd10", tpl_36372 as *const ()), // sub_1419ecd10
    Replacement::draft(0x1419ecd30, "std_1419ecd30", fn_1419ecd30 as *const ()), // sub_1419ecd30
    Replacement::draft(0x1419ece30, "std_1419ece30", fn_1419ece30 as *const ()), // sub_1419ece30
    Replacement::draft(0x1419ece60, "std_1419ece60", tpl_21550::<0x4> as *const ()), // sub_1419ece60
    Replacement::draft(0x1419ece70, "std_1419ece70", fn_1419ece70 as *const ()), // sub_1419ece70
    Replacement::draft(0x1419ecea0, "std_1419ecea0", tpl_21549::<0x4> as *const ()), // sub_1419ecea0
    Replacement::draft(0x1419eceb0, "std_1419eceb0", fn_1419eceb0 as *const ()), // sub_1419eceb0
    Replacement::draft(0x1419ecf00, "std_1419ecf00", fn_1419ecf00 as *const ()), // sub_1419ecf00
    Replacement::draft(0x1419ecf10, "std_1419ecf10", tpl_36401 as *const ()), // sub_1419ecf10
    Replacement::draft(0x1419ecf20, "std_1419ecf20", fn_1419ecf20 as *const ()), // sub_1419ecf20
    Replacement::draft(0x1419ecf30, "std_1419ecf30", fn_1419ecf30 as *const ()), // sub_1419ecf30
    Replacement::draft(0x1419ed110, "std_1419ed110", tpl_30459::<0x4> as *const ()), // sub_1419ed110
    Replacement::draft(0x1419ed120, "std_1419ed120", tpl_30345::<0x4> as *const ()), // sub_1419ed120
    Replacement::draft_throws(0x1419ed130, "std_1419ed130", fn_1419ed130 as *const (), Throws::Discard), // sub_1419ed130
    Replacement::draft(0x1419ed300, "std_1419ed300", tpl_34994::<0x143ca3d98, 0x14387b310, 0x14387b308, 0x141834a78, 0x1425e9c00, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419ed300
    Replacement::draft(0x1419ed370, "std_1419ed370", tpl_35252 as *const ()), // sub_1419ed370
    Replacement::draft(0x1419ed390, "std_1419ed390", tpl_34318 as *const ()), // sub_1419ed390
    Replacement::draft(0x1419ed3b0, "std_1419ed3b0", tpl_34318 as *const ()), // sub_1419ed3b0
    Replacement::draft(0x1419ed3d0, "std_1419ed3d0", tpl_34318 as *const ()), // sub_1419ed3d0
    Replacement::draft(0x1419ed3f0, "std_1419ed3f0", tpl_34318 as *const ()), // sub_1419ed3f0
    Replacement::draft(0x1419ed410, "std_1419ed410", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ed410
    Replacement::draft(0x1419ed470, "std_1419ed470", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ed470
    Replacement::draft(0x1419ed4d0, "std_1419ed4d0", tpl_34319::<0x143ca3cfc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ed4d0
    Replacement::draft(0x1419ed530, "std_1419ed530", tpl_34319::<0x143c8f4d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419ed530
    Replacement::draft_throws(0x1419ed590, "std_1419ed590", fn_1419ed590 as *const (), Throws::Caught), // sub_1419ed590
    Replacement::draft(0x1419ed8d0, "unknown_1419ed8d0", tpl_361 as *const ()), // sub_1419ed8d0
    Replacement::draft(0x1419ed8e0, "unknown_1419ed8e0", fn_1419ed8e0 as *const ()), // sub_1419ed8e0
    Replacement::draft_throws(0x1419ed9f0, "unknown_1419ed9f0", fn_1419ed9f0 as *const (), Throws::Discard), // sub_1419ed9f0
    Replacement::draft(0x1419eda50, "unknown_1419eda50", tpl_34994::<0x143ca3e3c, 0x14387b320, 0x14387b318, 0x141834a78, 0x1425ea320, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419eda50
    Replacement::draft(0x1419edac0, "unknown_1419edac0", tpl_369 as *const ()), // sub_1419edac0
    Replacement::draft(0x1419edad0, "unknown_1419edad0", tpl_34835::<0x8, 0x1403ea7a0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1419edad0
    Replacement::draft(0x1419edb50, "unknown_1419edb50", tpl_34837::<0x1403ea7a0, 0x20, 0x14240aa00> as *const ()), // sub_1419edb50
    Replacement::draft(0x1419edba0, "unknown_1419edba0", tpl_34840::<0x14045afd0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1419edba0
    Replacement::draft(0x1419edc10, "unknown_1419edc10", tpl_34318 as *const ()), // sub_1419edc10
    Replacement::draft(0x1419edc30, "unknown_1419edc30", tpl_34319::<0x143c8dbc4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419edc30
    Replacement::draft(0x1419edc90, "unknown_1419edc90", tpl_34318 as *const ()), // sub_1419edc90
    Replacement::draft(0x1419edcb0, "unknown_1419edcb0", tpl_34318 as *const ()), // sub_1419edcb0
    Replacement::draft(0x1419edcd0, "unknown_1419edcd0", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419edcd0
    Replacement::draft(0x1419edd30, "unknown_1419edd30", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419edd30
    Replacement::draft(0x1419edd90, "unknown_1419edd90", tpl_34318 as *const ()), // sub_1419edd90
    Replacement::draft(0x1419eddb0, "unknown_1419eddb0", tpl_34319::<0x143c90164, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419eddb0
    Replacement::draft_throws(0x1419ede10, "unknown_1419ede10", tpl_95454::<0x1403e62b0, 0x14309c488, 0x14272d798, 0x14272d7f0, 0x14272d7c8, 0x14272d780, 0x14272d7bc, 0x14272d788, 0x140254aa0, 0x1424b16a0, 0x1403e80f0> as *const (), Throws::Discard), // sub_1419ede10
    Replacement::draft_throws(0x1419ee200, "unknown_1419ee200", fn_1419ee200 as *const (), Throws::Caught), // sub_1419ee200
    Replacement::draft_throws(0x1419ee400, "unknown_1419ee400", tpl_35688::<0x18, 0x1419ede10, 0x10> as *const (), Throws::Discard), // sub_1419ee400
    Replacement::draft_throws(0x1419ee440, "unknown_1419ee440", tpl_34846::<0x1401f3a20, 0x1419ede10> as *const (), Throws::Discard), // sub_1419ee440
    Replacement::draft(0x1419ee4c0, "unknown_1419ee4c0", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_1419ee4c0
    Replacement::draft(0x1419ee4f0, "unknown_1419ee4f0", tpl_361 as *const ()), // sub_1419ee4f0
    Replacement::draft(0x1419ee500, "unknown_1419ee500", fn_1419ee500 as *const ()), // sub_1419ee500
    Replacement::draft_throws(0x1419ee5e0, "unknown_1419ee5e0", fn_1419ee5e0 as *const (), Throws::Discard), // sub_1419ee5e0
    Replacement::draft_throws(0x1419ee630, "unknown_1419ee630", fn_1419ee630 as *const (), Throws::Discard), // sub_1419ee630
    Replacement::draft_throws(0x1419ee660, "unknown_1419ee660", fn_1419ee660 as *const (), Throws::Discard), // sub_1419ee660
    Replacement::draft_throws(0x1419ee700, "unknown_1419ee700", fn_1419ee700 as *const (), Throws::Caught), // sub_1419ee700
    Replacement::draft(0x1419eedb0, "unknown_1419eedb0", fn_1419eedb0 as *const ()), // sub_1419eedb0
    Replacement::draft_throws(0x1419eef90, "unknown_1419eef90", fn_1419eef90 as *const (), Throws::Discard), // sub_1419eef90
    Replacement::draft_throws(0x1419ef080, "unknown_1419ef080", fn_1419ef080 as *const (), Throws::Caught), // sub_1419ef080
    Replacement::draft_throws(0x1419ef4c0, "unknown_1419ef4c0", fn_1419ef4c0 as *const (), Throws::Caught), // sub_1419ef4c0
    Replacement::draft_throws(0x1419ef900, "unknown_1419ef900", fn_1419ef900 as *const (), Throws::Caught), // sub_1419ef900
    Replacement::draft_throws(0x1419efef0, "unknown_1419efef0", fn_1419efef0 as *const (), Throws::Caught), // sub_1419efef0
    Replacement::draft(0x1419f07c0, "unknown_1419f07c0", tpl_361 as *const ()), // sub_1419f07c0
    Replacement::draft(0x1419f07d0, "unknown_1419f07d0", tpl_361 as *const ()), // sub_1419f07d0
    Replacement::draft_throws(0x1419f07e0, "unknown_1419f07e0", fn_1419f07e0 as *const (), Throws::Caught), // sub_1419f07e0
    Replacement::draft_throws(0x1419f0e40, "unknown_1419f0e40", fn_1419f0e40 as *const (), Throws::Caught), // sub_1419f0e40
    Replacement::draft_throws(0x1419f1060, "unknown_1419f1060", fn_1419f1060 as *const (), Throws::Discard), // sub_1419f1060
    Replacement::draft(0x1419f1200, "unknown_1419f1200", fn_1419f1200 as *const ()), // sub_1419f1200
    Replacement::draft(0x1419f13a0, "unknown_1419f13a0", fn_1419f13a0 as *const ()), // sub_1419f13a0
    Replacement::draft_throws(0x1419f1620, "unknown_1419f1620", fn_1419f1620 as *const (), Throws::Discard), // sub_1419f1620
    Replacement::draft_throws(0x1419f19e0, "unknown_1419f19e0", fn_1419f19e0 as *const (), Throws::Discard), // sub_1419f19e0
    Replacement::draft_throws(0x1419f1ac0, "unknown_1419f1ac0", fn_1419f1ac0 as *const (), Throws::Discard), // sub_1419f1ac0
    Replacement::draft(0x1419f1ec0, "unknown_1419f1ec0", fn_1419f1ec0 as *const ()), // sub_1419f1ec0
    Replacement::draft(0x1419f2120, "unknown_1419f2120", fn_1419f2120 as *const ()), // sub_1419f2120
    Replacement::draft(0x1419f2220, "unknown_1419f2220", fn_1419f2220 as *const ()), // sub_1419f2220
    Replacement::draft_throws(0x1419f2250, "unknown_1419f2250", fn_1419f2250 as *const (), Throws::Discard), // sub_1419f2250
    Replacement::draft(0x1419f2460, "unknown_1419f2460", fn_1419f2460 as *const ()), // sub_1419f2460
    Replacement::draft(0x1419f2540, "unknown_1419f2540", fn_1419f2540 as *const ()), // sub_1419f2540
    Replacement::draft(0x1419f2550, "unknown_1419f2550", fn_1419f2550 as *const ()), // sub_1419f2550
    Replacement::draft_throws(0x1419f2570, "unknown_1419f2570", fn_1419f2570 as *const (), Throws::Caught), // sub_1419f2570
    Replacement::draft(0x1419f2b20, "unknown_1419f2b20", tpl_34994::<0x143ca43f0, 0x14387b330, 0x14387b328, 0x141834a78, 0x1425ea800, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419f2b20
    Replacement::draft(0x1419f2b90, "unknown_1419f2b90", tpl_95133::<0x143141708, 0x1, 0x8, 0x14047b5e0> as *const ()), // sub_1419f2b90
    Replacement::draft(0x1419f2c80, "unknown_1419f2c80", tpl_35322 as *const ()), // sub_1419f2c80
    Replacement::draft(0x1419f2cc0, "unknown_1419f2cc0", tpl_34994::<0x143ca41f8, 0x14387b340, 0x14387b338, 0x141834a78, 0x1425ea810, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419f2cc0
    Replacement::draft(0x1419f2d30, "unknown_1419f2d30", tpl_34610::<0x20> as *const ()), // sub_1419f2d30
    Replacement::draft(0x1419f2d60, "unknown_1419f2d60", tpl_34318 as *const ()), // sub_1419f2d60
    Replacement::draft(0x1419f2d80, "unknown_1419f2d80", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419f2d80
    Replacement::draft(0x1419f2de0, "unknown_1419f2de0", tpl_34318 as *const ()), // sub_1419f2de0
    Replacement::draft(0x1419f2e00, "unknown_1419f2e00", tpl_34318 as *const ()), // sub_1419f2e00
    Replacement::draft(0x1419f2e20, "unknown_1419f2e20", tpl_34319::<0x143ca4244, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419f2e20
    Replacement::draft(0x1419f2e80, "unknown_1419f2e80", tpl_34319::<0x143ca4318, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419f2e80
    Replacement::draft(0x1419f2ee0, "unknown_1419f2ee0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ca4318, 0x158, 0x14020e8f0> as *const ()), // sub_1419f2ee0
    Replacement::draft(0x1419f2fd0, "unknown_1419f2fd0", tpl_496 as *const ()), // sub_1419f2fd0
    Replacement::draft(0x1419f2fe0, "unknown_1419f2fe0", fn_1419f2fe0 as *const ()), // sub_1419f2fe0
    Replacement::draft(0x1419f3070, "unknown_1419f3070", tpl_496 as *const ()), // sub_1419f3070
    Replacement::draft(0x1419f3080, "unknown_1419f3080", fn_1419f3080 as *const ()), // sub_1419f3080
    Replacement::draft(0x1419f30a0, "unknown_1419f30a0", fn_1419f30a0 as *const ()), // sub_1419f30a0
    Replacement::draft(0x1419f30d0, "unknown_1419f30d0", fn_1419f30d0 as *const ()), // sub_1419f30d0
    Replacement::draft(0x1419f3200, "unknown_1419f3200", tpl_361 as *const ()), // sub_1419f3200
    Replacement::draft_throws(0x1419f3210, "unknown_1419f3210", fn_1419f3210 as *const (), Throws::Discard), // sub_1419f3210
    Replacement::draft_throws(0x1419f3510, "unknown_1419f3510", fn_1419f3510 as *const (), Throws::Discard), // sub_1419f3510
    Replacement::draft(0x1419f3a40, "unknown_1419f3a40", tpl_361 as *const ()), // sub_1419f3a40
    Replacement::draft(0x1419f3a50, "unknown_1419f3a50", tpl_1439::<0x8, 0x140478bd0> as *const ()), // sub_1419f3a50
    Replacement::draft_throws(0x1419f3a90, "unknown_1419f3a90", fn_1419f3a90 as *const (), Throws::Discard), // sub_1419f3a90
    Replacement::draft(0x1419f3f40, "std_1419f3f40", tpl_23342::<0x1431427f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::isGameObjectVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x1419f3f60, "std_1419f3f60", tpl_23342::<0x1431427f0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::isGameObjectVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x1419f3f80, "std_1419f3f80", fn_1419f3f80 as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::isGameObjectVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x1419f4020, "std_1419f4020", tpl_1::<0x143889ee0> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::isGameObjectVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x1419f4030, "std_1419f4030", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::isGameObjectVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x1419f4040, "std_1419f4040", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::isGameObjectVisible::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x1419f4060, "std_1419f4060", tpl_550::<0x143142828, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_1>,bool,m::SIMDVector>::std::_Func_impl_no_allo
    Replacement::draft(0x1419f4080, "std_1419f4080", tpl_550::<0x143142828, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_1>,bool,m::SIMDVector>::std::_Func_impl_no_allo
    Replacement::draft(0x1419f40a0, "std_1419f40a0", fn_1419f40a0 as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_1>,bool,m::SIMDVector>::std::_Func_impl_no_allo
    Replacement::draft(0x1419f40e0, "std_1419f40e0", tpl_1::<0x143888c50> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_1>,bool,m::SIMDVector>::std::_Func_impl_no_allo
    Replacement::draft(0x1419f40f0, "std_1419f40f0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_1>,bool,m::SIMDVector>::std::_Func_impl_no_allo
    Replacement::draft(0x1419f4100, "std_1419f4100", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_1>,bool,m::SIMDVector>::std::_Func_impl_no_allo
    Replacement::draft(0x1419f4120, "std_1419f4120", tpl_550::<0x1431427b8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_2>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f4140, "std_1419f4140", tpl_550::<0x1431427b8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_2>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f4160, "std_1419f4160", fn_1419f4160 as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_2>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f4220, "std_1419f4220", tpl_1::<0x1438898b0> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_2>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f4230, "std_1419f4230", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_2>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f4240, "std_1419f4240", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_2>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f4260, "std_1419f4260", tpl_550::<0x143142860, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_3>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f4280, "std_1419f4280", tpl_550::<0x143142860, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_3>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f42a0, "std_1419f42a0", fn_1419f42a0 as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_3>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f42d0, "std_1419f42d0", tpl_1::<0x143889280> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_3>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f42e0, "std_1419f42e0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_3>,float,const m::Vector3Template<float>&>::std
    Replacement::draft(0x1419f42f0, "std_1419f42f0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::ImpulseWave::applyChunkDamage::<lambda_3>,float,const m::Vector3Template<float>&>::std
    Replacement::draft_throws(0x1419f4310, "unknown_1419f4310", fn_1419f4310 as *const (), Throws::Discard), // sub_1419f4310
    Replacement::draft_throws(0x1419f4820, "unknown_1419f4820", fn_1419f4820 as *const (), Throws::Caught), // sub_1419f4820
    Replacement::draft(0x1419f5dc0, "unknown_1419f5dc0", fn_1419f5dc0 as *const ()), // sub_1419f5dc0
    Replacement::draft(0x1419f5e10, "unknown_1419f5e10", fn_1419f5e10 as *const ()), // sub_1419f5e10
    Replacement::draft(0x1419f5f00, "unknown_1419f5f00", fn_1419f5f00 as *const ()), // sub_1419f5f00
    Replacement::draft(0x1419f6480, "unknown_1419f6480", fn_1419f6480 as *const ()), // sub_1419f6480
    Replacement::draft_throws(0x1419f68a0, "unknown_1419f68a0", fn_1419f68a0 as *const (), Throws::Discard), // sub_1419f68a0
    Replacement::draft(0x1419f6a60, "unknown_1419f6a60", tpl_34443 as *const ()), // sub_1419f6a60
    Replacement::draft(0x1419f6a70, "unknown_1419f6a70", fn_1419f6a70 as *const ()), // sub_1419f6a70
    Replacement::draft(0x1419f6aa0, "unknown_1419f6aa0", tpl_34443 as *const ()), // sub_1419f6aa0
    Replacement::draft(0x1419f6ab0, "unknown_1419f6ab0", fn_1419f6ab0 as *const ()), // sub_1419f6ab0
    Replacement::draft(0x1419f6b70, "unknown_1419f6b70", tpl_34443 as *const ()), // sub_1419f6b70
    Replacement::draft(0x1419f6b80, "unknown_1419f6b80", fn_1419f6b80 as *const ()), // sub_1419f6b80
    Replacement::draft_throws(0x1419f6bb0, "unknown_1419f6bb0", fn_1419f6bb0 as *const (), Throws::Discard), // sub_1419f6bb0
    Replacement::draft_throws(0x1419f6e50, "unknown_1419f6e50", fn_1419f6e50 as *const (), Throws::Discard), // sub_1419f6e50
    Replacement::draft_throws(0x1419f73c0, "unknown_1419f73c0", fn_1419f73c0 as *const (), Throws::Discard), // sub_1419f73c0
    Replacement::draft_throws(0x1419f7410, "unknown_1419f7410", fn_1419f7410 as *const (), Throws::Discard), // sub_1419f7410
    Replacement::draft_throws(0x1419f75b0, "unknown_1419f75b0", fn_1419f75b0 as *const (), Throws::Discard), // sub_1419f75b0
    Replacement::draft_throws(0x1419f7940, "unknown_1419f7940", fn_1419f7940 as *const (), Throws::Discard), // sub_1419f7940
    Replacement::draft_throws(0x1419f7a50, "unknown_1419f7a50", fn_1419f7a50 as *const (), Throws::Discard), // sub_1419f7a50
    Replacement::draft(0x1419f7ac0, "unknown_1419f7ac0", fn_1419f7ac0 as *const ()), // sub_1419f7ac0
    Replacement::draft(0x1419f7d90, "unknown_1419f7d90", fn_1419f7d90 as *const ()), // sub_1419f7d90
    Replacement::draft(0x1419f7e50, "unknown_1419f7e50", fn_1419f7e50 as *const ()), // sub_1419f7e50
    Replacement::draft(0x1419f7ee0, "unknown_1419f7ee0", fn_1419f7ee0 as *const ()), // sub_1419f7ee0
    Replacement::draft(0x1419f7f10, "unknown_1419f7f10", fn_1419f7f10 as *const ()), // sub_1419f7f10
    Replacement::draft(0x1419f7f60, "unknown_1419f7f60", tpl_22031::<0x8> as *const ()), // sub_1419f7f60
    Replacement::draft(0x1419f7f80, "unknown_1419f7f80", fn_1419f7f80 as *const ()), // sub_1419f7f80
    Replacement::draft(0x1419f8020, "unknown_1419f8020", fn_1419f8020 as *const ()), // sub_1419f8020
    Replacement::draft(0x1419f8240, "unknown_1419f8240", fn_1419f8240 as *const ()), // sub_1419f8240
    Replacement::draft(0x1419f8290, "unknown_1419f8290", fn_1419f8290 as *const ()), // sub_1419f8290
    Replacement::draft(0x1419f8380, "unknown_1419f8380", fn_1419f8380 as *const ()), // sub_1419f8380
    Replacement::draft(0x1419f8430, "unknown_1419f8430", fn_1419f8430 as *const ()), // sub_1419f8430
    Replacement::draft(0x1419f84f0, "unknown_1419f84f0", fn_1419f84f0 as *const ()), // sub_1419f84f0
    Replacement::draft(0x1419f85b0, "unknown_1419f85b0", fn_1419f85b0 as *const ()), // sub_1419f85b0
    Replacement::draft(0x1419f8680, "unknown_1419f8680", fn_1419f8680 as *const ()), // sub_1419f8680
    Replacement::draft(0x1419f8770, "unknown_1419f8770", tpl_496 as *const ()), // sub_1419f8770
    Replacement::draft(0x1419f8780, "unknown_1419f8780", fn_1419f8780 as *const ()), // sub_1419f8780
    Replacement::draft(0x1419f8990, "unknown_1419f8990", fn_1419f8990 as *const ()), // sub_1419f8990
    Replacement::draft_throws(0x1419f9050, "unknown_1419f9050", fn_1419f9050 as *const (), Throws::Discard), // sub_1419f9050
    Replacement::draft(0x1419f93c0, "unknown_1419f93c0", tpl_365::<0x14047bad0> as *const ()), // sub_1419f93c0
    Replacement::draft(0x1419f93d0, "unknown_1419f93d0", fn_1419f93d0 as *const ()), // sub_1419f93d0
    Replacement::draft(0x1419f9400, "unknown_1419f9400", fn_1419f9400 as *const ()), // sub_1419f9400
    Replacement::draft(0x1419f9490, "unknown_1419f9490", fn_1419f9490 as *const ()), // sub_1419f9490
    Replacement::draft(0x1419f96d0, "unknown_1419f96d0", fn_1419f96d0 as *const ()), // sub_1419f96d0
    Replacement::draft_throws(0x1419f97d0, "unknown_1419f97d0", fn_1419f97d0 as *const (), Throws::Discard), // sub_1419f97d0
    Replacement::draft(0x1419f9ae0, "unknown_1419f9ae0", fn_1419f9ae0 as *const ()), // sub_1419f9ae0
    Replacement::draft(0x1419f9b70, "unknown_1419f9b70", fn_1419f9b70 as *const ()), // sub_1419f9b70
    Replacement::draft(0x1419f9bc0, "unknown_1419f9bc0", fn_1419f9bc0 as *const ()), // sub_1419f9bc0
    Replacement::draft(0x1419f9fc0, "unknown_1419f9fc0", fn_1419f9fc0 as *const ()), // sub_1419f9fc0
    Replacement::draft(0x1419fa180, "unknown_1419fa180", fn_1419fa180 as *const ()), // sub_1419fa180
    Replacement::draft_throws(0x1419fa1f0, "unknown_1419fa1f0", fn_1419fa1f0 as *const (), Throws::Discard), // sub_1419fa1f0
    Replacement::draft(0x1419fa490, "unknown_1419fa490", tpl_34318 as *const ()), // sub_1419fa490
    Replacement::draft(0x1419fa4b0, "unknown_1419fa4b0", tpl_34318 as *const ()), // sub_1419fa4b0
    Replacement::draft(0x1419fa4d0, "unknown_1419fa4d0", tpl_34318 as *const ()), // sub_1419fa4d0
    Replacement::draft(0x1419fa4f0, "unknown_1419fa4f0", tpl_34318 as *const ()), // sub_1419fa4f0
    Replacement::draft(0x1419fa510, "unknown_1419fa510", tpl_34318 as *const ()), // sub_1419fa510
    Replacement::draft(0x1419fa530, "unknown_1419fa530", tpl_34318 as *const ()), // sub_1419fa530
    Replacement::draft(0x1419fa550, "unknown_1419fa550", tpl_34318 as *const ()), // sub_1419fa550
    Replacement::draft(0x1419fa570, "unknown_1419fa570", tpl_34318 as *const ()), // sub_1419fa570
    Replacement::draft(0x1419fa590, "unknown_1419fa590", tpl_34318 as *const ()), // sub_1419fa590
    Replacement::draft(0x1419fa5b0, "unknown_1419fa5b0", tpl_34319::<0x143c8edbc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa5b0
    Replacement::draft(0x1419fa610, "unknown_1419fa610", tpl_34319::<0x143c9659c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa610
    Replacement::draft(0x1419fa670, "unknown_1419fa670", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa670
    Replacement::draft(0x1419fa6d0, "unknown_1419fa6d0", tpl_34319::<0x143c95048, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa6d0
    Replacement::draft(0x1419fa730, "unknown_1419fa730", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa730
    Replacement::draft(0x1419fa790, "unknown_1419fa790", tpl_34319::<0x143c8ea68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa790
    Replacement::draft(0x1419fa7f0, "unknown_1419fa7f0", tpl_34319::<0x143c95468, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa7f0
    Replacement::draft(0x1419fa850, "unknown_1419fa850", tpl_34319::<0x143c8f4cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa850
    Replacement::draft(0x1419fa8b0, "unknown_1419fa8b0", tpl_34319::<0x143c91070, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fa8b0
    Replacement::draft(0x1419fa910, "unknown_1419fa910", tpl_34318 as *const ()), // sub_1419fa910
    Replacement::draft(0x1419fa930, "unknown_1419fa930", tpl_34318 as *const ()), // sub_1419fa930
    Replacement::draft(0x1419fa950, "unknown_1419fa950", tpl_34318 as *const ()), // sub_1419fa950
    Replacement::draft(0x1419fa970, "unknown_1419fa970", tpl_34318 as *const ()), // sub_1419fa970
    Replacement::draft(0x1419fa990, "unknown_1419fa990", tpl_34318 as *const ()), // sub_1419fa990
    Replacement::draft(0x1419fa9b0, "unknown_1419fa9b0", tpl_34318 as *const ()), // sub_1419fa9b0
    Replacement::draft(0x1419fa9d0, "unknown_1419fa9d0", tpl_34318 as *const ()), // sub_1419fa9d0
    Replacement::draft(0x1419fa9f0, "unknown_1419fa9f0", tpl_34318 as *const ()), // sub_1419fa9f0
    Replacement::draft(0x1419faa10, "unknown_1419faa10", tpl_34318 as *const ()), // sub_1419faa10
    Replacement::draft(0x1419faa30, "unknown_1419faa30", tpl_34319::<0x143c8ec28, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419faa30
    Replacement::draft(0x1419faa90, "unknown_1419faa90", tpl_34319::<0x143c8de68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419faa90
    Replacement::draft(0x1419faaf0, "unknown_1419faaf0", tpl_34319::<0x143c8e9b0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419faaf0
    Replacement::draft(0x1419fab50, "unknown_1419fab50", tpl_34319::<0x143c8dbd0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fab50
    Replacement::draft(0x1419fabb0, "unknown_1419fabb0", tpl_34319::<0x143c8e840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fabb0
    Replacement::draft(0x1419fac10, "unknown_1419fac10", tpl_34319::<0x143c970e0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fac10
    Replacement::draft(0x1419fac70, "unknown_1419fac70", tpl_34319::<0x143c8dbc4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fac70
    Replacement::draft(0x1419facd0, "unknown_1419facd0", tpl_34319::<0x143c8d508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419facd0
    Replacement::draft(0x1419fad30, "unknown_1419fad30", tpl_34319::<0x143c90198, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fad30
    Replacement::draft(0x1419fadd0, "unknown_1419fadd0", fn_1419fadd0 as *const ()), // sub_1419fadd0
    Replacement::draft(0x1419faf60, "unknown_1419faf60", fn_1419faf60 as *const ()), // sub_1419faf60
    Replacement::draft(0x1419fbc80, "unknown_1419fbc80", tpl_361 as *const ()), // sub_1419fbc80
    Replacement::draft(0x1419fbc90, "unknown_1419fbc90", fn_1419fbc90 as *const ()), // sub_1419fbc90
    Replacement::draft(0x1419fbef0, "unknown_1419fbef0", tpl_361 as *const ()), // sub_1419fbef0
    Replacement::draft(0x1419fbf00, "unknown_1419fbf00", fn_1419fbf00 as *const ()), // sub_1419fbf00
    Replacement::draft(0x1419fbf20, "unknown_1419fbf20", fn_1419fbf20 as *const ()), // sub_1419fbf20
    Replacement::draft(0x1419fbf50, "unknown_1419fbf50", tpl_36473::<0x4> as *const ()), // sub_1419fbf50
    Replacement::draft_throws(0x1419fbf70, "unknown_1419fbf70", fn_1419fbf70 as *const (), Throws::Discard), // sub_1419fbf70
    Replacement::draft(0x1419fc480, "unknown_1419fc480", tpl_34994::<0x143ca47c8, 0x14387b350, 0x14387b348, 0x141834a78, 0x1425eb0c0, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419fc480
    Replacement::draft(0x1419fc4f0, "unknown_1419fc4f0", tpl_369 as *const ()), // sub_1419fc4f0
    Replacement::draft(0x1419fc500, "unknown_1419fc500", tpl_34318 as *const ()), // sub_1419fc500
    Replacement::draft(0x1419fc520, "unknown_1419fc520", tpl_34318 as *const ()), // sub_1419fc520
    Replacement::draft(0x1419fc540, "unknown_1419fc540", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fc540
    Replacement::draft(0x1419fc5a0, "unknown_1419fc5a0", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fc5a0
    Replacement::draft(0x1419fc600, "unknown_1419fc600", fn_1419fc600 as *const ()), // sub_1419fc600
    Replacement::draft(0x1419fc750, "unknown_1419fc750", tpl_361 as *const ()), // sub_1419fc750
    Replacement::draft(0x1419fc760, "unknown_1419fc760", fn_1419fc760 as *const ()), // sub_1419fc760
    Replacement::draft(0x1419fc800, "unknown_1419fc800", tpl_361 as *const ()), // sub_1419fc800
    Replacement::draft_throws(0x1419fc810, "unknown_1419fc810", fn_1419fc810 as *const (), Throws::Discard), // sub_1419fc810
    Replacement::draft(0x1419fc880, "unknown_1419fc880", tpl_34994::<0x143ca4920, 0x14387b380, 0x14387b378, 0x141834a78, 0x1425eba30, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419fc880
    Replacement::draft(0x1419fc8f0, "unknown_1419fc8f0", tpl_369 as *const ()), // sub_1419fc8f0
    Replacement::draft(0x1419fc900, "unknown_1419fc900", tpl_34994::<0x143ca5008, 0x14387b370, 0x14387b368, 0x141834a78, 0x1425eba40, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419fc900
    Replacement::draft(0x1419fc970, "unknown_1419fc970", tpl_34611::<0x8> as *const ()), // sub_1419fc970
    Replacement::draft(0x1419fc990, "unknown_1419fc990", tpl_34994::<0x143ca4a20, 0x14387b390, 0x14387b388, 0x141834a78, 0x1425eba50, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419fc990
    Replacement::draft(0x1419fca00, "unknown_1419fca00", tpl_34611::<0x8> as *const ()), // sub_1419fca00
    Replacement::draft(0x1419fca20, "unknown_1419fca20", tpl_34994::<0x143ca49ac, 0x14387b3a8, 0x14387b3a0, 0x141834a78, 0x1425eba60, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419fca20
    Replacement::draft(0x1419fca90, "unknown_1419fca90", tpl_34612 as *const ()), // sub_1419fca90
    Replacement::draft(0x1419fcab0, "unknown_1419fcab0", tpl_34994::<0x143ca4a24, 0x14387b360, 0x14387b358, 0x141834a78, 0x1425eba70, 0x141834d18, 0x141834a0c> as *const ()), // sub_1419fcab0
    Replacement::draft(0x1419fcb20, "unknown_1419fcb20", tpl_35323 as *const ()), // sub_1419fcb20
    Replacement::draft(0x1419fcb40, "unknown_1419fcb40", tpl_34318 as *const ()), // sub_1419fcb40
    Replacement::draft(0x1419fcb60, "unknown_1419fcb60", tpl_34319::<0x143ca5004, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fcb60
    Replacement::draft(0x1419fcbc0, "unknown_1419fcbc0", tpl_34318 as *const ()), // sub_1419fcbc0
    Replacement::draft(0x1419fcbe0, "unknown_1419fcbe0", tpl_34318 as *const ()), // sub_1419fcbe0
    Replacement::draft(0x1419fcc00, "unknown_1419fcc00", tpl_34318 as *const ()), // sub_1419fcc00
    Replacement::draft(0x1419fcc20, "unknown_1419fcc20", tpl_34318 as *const ()), // sub_1419fcc20
    Replacement::draft(0x1419fcc40, "unknown_1419fcc40", tpl_34318 as *const ()), // sub_1419fcc40
    Replacement::draft(0x1419fcc60, "unknown_1419fcc60", tpl_34318 as *const ()), // sub_1419fcc60
    Replacement::draft(0x1419fcc80, "unknown_1419fcc80", tpl_34318 as *const ()), // sub_1419fcc80
    Replacement::draft(0x1419fcca0, "unknown_1419fcca0", tpl_34318 as *const ()), // sub_1419fcca0
    Replacement::draft(0x1419fccc0, "unknown_1419fccc0", tpl_34318 as *const ()), // sub_1419fccc0
    Replacement::draft(0x1419fcce0, "unknown_1419fcce0", tpl_34318 as *const ()), // sub_1419fcce0
    Replacement::draft(0x1419fcd00, "unknown_1419fcd00", tpl_34319::<0x143c97e70, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fcd00
    Replacement::draft(0x1419fcd60, "unknown_1419fcd60", tpl_34319::<0x143c8f4d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fcd60
    Replacement::draft(0x1419fcdc0, "unknown_1419fcdc0", tpl_34319::<0x143c9ecdc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fcdc0
    Replacement::draft(0x1419fce20, "unknown_1419fce20", tpl_34319::<0x143c9779c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fce20
    Replacement::draft(0x1419fce80, "unknown_1419fce80", tpl_34319::<0x143ca50ec, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fce80
    Replacement::draft(0x1419fcee0, "unknown_1419fcee0", tpl_34319::<0x143ca51ac, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fcee0
    Replacement::draft(0x1419fcf40, "unknown_1419fcf40", tpl_34319::<0x143ca4c40, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fcf40
    Replacement::draft(0x1419fcfa0, "unknown_1419fcfa0", tpl_34319::<0x143c8e70c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fcfa0
    Replacement::draft(0x1419fd000, "unknown_1419fd000", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fd000
    Replacement::draft(0x1419fd060, "unknown_1419fd060", tpl_34319::<0x143c9db0c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1419fd060
    Replacement::draft(0x1419fd0c0, "unknown_1419fd0c0", fn_1419fd0c0 as *const ()), // sub_1419fd0c0
    Replacement::draft(0x1419fd270, "unknown_1419fd270", tpl_361 as *const ()), // sub_1419fd270
    Replacement::draft(0x1419fd280, "unknown_1419fd280", fn_1419fd280 as *const ()), // sub_1419fd280
    Replacement::draft(0x1419fd360, "unknown_1419fd360", tpl_361 as *const ()), // sub_1419fd360
    Replacement::draft_throws(0x1419fd370, "unknown_1419fd370", fn_1419fd370 as *const (), Throws::Discard), // sub_1419fd370
    Replacement::draft_throws(0x1419fd420, "unknown_1419fd420", fn_1419fd420 as *const (), Throws::Discard), // sub_1419fd420
    Replacement::draft_throws(0x1419fd7d0, "unknown_1419fd7d0", fn_1419fd7d0 as *const (), Throws::Discard), // sub_1419fd7d0
    Replacement::draft(0x1419fd8f0, "unknown_1419fd8f0", tpl_361 as *const ()), // sub_1419fd8f0
    Replacement::draft(0x1419fd900, "unknown_1419fd900", fn_1419fd900 as *const ()), // sub_1419fd900
    Replacement::draft(0x1419fd9a0, "unknown_1419fd9a0", tpl_361 as *const ()), // sub_1419fd9a0
    Replacement::draft_throws(0x1419fd9b0, "unknown_1419fd9b0", fn_1419fd9b0 as *const (), Throws::Discard), // sub_1419fd9b0
    Replacement::draft(0x1419fda20, "unknown_1419fda20", tpl_361 as *const ()), // sub_1419fda20
    Replacement::draft(0x1419fda30, "unknown_1419fda30", fn_1419fda30 as *const ()), // sub_1419fda30
    Replacement::draft(0x1419fda80, "unknown_1419fda80", fn_1419fda80 as *const ()), // sub_1419fda80
    Replacement::draft_throws(0x1419fdb20, "unknown_1419fdb20", fn_1419fdb20 as *const (), Throws::Discard), // sub_1419fdb20
    Replacement::draft(0x1419fdc80, "unknown_1419fdc80", tpl_361 as *const ()), // sub_1419fdc80
    Replacement::draft(0x1419fdc90, "unknown_1419fdc90", fn_1419fdc90 as *const ()), // sub_1419fdc90
    Replacement::draft(0x1419fdcb0, "unknown_1419fdcb0", fn_1419fdcb0 as *const ()), // sub_1419fdcb0
    Replacement::draft(0x1419fdcf0, "unknown_1419fdcf0", tpl_361 as *const ()), // sub_1419fdcf0
    Replacement::draft_throws(0x1419fdd00, "unknown_1419fdd00", fn_1419fdd00 as *const (), Throws::Discard), // sub_1419fdd00
    Replacement::draft_throws(0x1419fdd70, "unknown_1419fdd70", fn_1419fdd70 as *const (), Throws::Discard), // sub_1419fdd70
    Replacement::draft(0x1419fe000, "unknown_1419fe000", fn_1419fe000 as *const ()), // sub_1419fe000
    Replacement::draft_throws(0x1419fe040, "unknown_1419fe040", fn_1419fe040 as *const (), Throws::Discard), // sub_1419fe040
    Replacement::draft(0x1419fe0d0, "unknown_1419fe0d0", fn_1419fe0d0 as *const ()), // sub_1419fe0d0
    Replacement::draft_throws(0x1419fe100, "unknown_1419fe100", fn_1419fe100 as *const (), Throws::Discard), // sub_1419fe100
    Replacement::draft(0x1419fe2f0, "unknown_1419fe2f0", tpl_361 as *const ()), // sub_1419fe2f0
    Replacement::draft_throws(0x1419fe300, "unknown_1419fe300", fn_1419fe300 as *const (), Throws::Discard), // sub_1419fe300
    Replacement::draft(0x1419fe3a0, "unknown_1419fe3a0", fn_1419fe3a0 as *const ()), // sub_1419fe3a0
    Replacement::draft(0x1419fe5e0, "unknown_1419fe5e0", fn_1419fe5e0 as *const ()), // sub_1419fe5e0
    Replacement::draft(0x1419fe630, "unknown_1419fe630", fn_1419fe630 as *const ()), // sub_1419fe630
    Replacement::draft(0x1419fe670, "unknown_1419fe670", fn_1419fe670 as *const ()), // sub_1419fe670
    Replacement::draft(0x1419fe6b0, "unknown_1419fe6b0", fn_1419fe6b0 as *const ()), // sub_1419fe6b0
    Replacement::draft(0x1419fe770, "unknown_1419fe770", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ca4d5c, 0x158, 0x14020e8f0> as *const ()), // sub_1419fe770
    Replacement::draft(0x1419fe860, "unknown_1419fe860", tpl_496 as *const ()), // sub_1419fe860
    Replacement::draft(0x1419fe870, "unknown_1419fe870", fn_1419fe870 as *const ()), // sub_1419fe870
    Replacement::draft(0x1419fe8f0, "unknown_1419fe8f0", tpl_361 as *const ()), // sub_1419fe8f0
    Replacement::draft(0x1419fe900, "unknown_1419fe900", fn_1419fe900 as *const ()), // sub_1419fe900
    Replacement::draft(0x1419fedb0, "bigfish_1419fedb0", tpl_36497::<0x8, 0x4> as *const ()), // bigfish::health::SaveData::ClassTypeInfoImplementation::bigfish::health::SaveData::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x1419fede0, "bigfish_1419fede0", tpl_36497::<0x8, 0x4> as *const ()), // bigfish::health::SaveData::ClassTypeInfoImplementation::bigfish::health::SaveData::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x1419fef10, "bigfish_1419fef10", tpl_35219::<0x14387b398, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bigfish::health::SaveData::ClassTypeInfoImplementation::bigfish::health::SaveData::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1419fef70, "bigfish_1419fef70", tpl_35219::<0x14387b398, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bigfish::health::SaveData::ClassTypeInfoImplementation::bigfish::health::SaveData::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1419ff030, "bigfish_1419ff030", fn_1419ff030 as *const ()), // bigfish::health::SaveData::ClassTypeInfoImplementation::bigfish::health::SaveData::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1419ff120, "bigfish_1419ff120", fn_1419ff120 as *const ()), // sub_1419ff120
    Replacement::draft(0x1419ff160, "bigfish_1419ff160", fn_1419ff160 as *const ()), // sub_1419ff160
    Replacement::draft(0x1419ff1c0, "bigfish_1419ff1c0", fn_1419ff1c0 as *const ()), // sub_1419ff1c0
    Replacement::draft(0x1419ff200, "bigfish_1419ff200", fn_1419ff200 as *const ()), // sub_1419ff200
    Replacement::draft_throws(0x1419ff230, "bigfish_1419ff230", fn_1419ff230 as *const (), Throws::Discard), // sub_1419ff230
    Replacement::draft(0x1419ffac0, "bigfish_1419ffac0", tpl_34612 as *const ()), // sub_1419ffac0
    Replacement::draft(0x1419ffb50, "bigfish_1419ffb50", tpl_34605::<0x1431488f0> as *const ()), // sub_1419ffb50
    Replacement::draft_throws(0x141a00a20, "bigfish_141a00a20", fn_141a00a20 as *const (), Throws::Discard), // sub_141a00a20
    Replacement::draft_throws(0x141a00ca0, "bigfish_141a00ca0", fn_141a00ca0 as *const (), Throws::Caught), // sub_141a00ca0
    Replacement::draft(0x141a01270, "bigfish_141a01270", fn_141a01270 as *const ()), // sub_141a01270
    Replacement::draft(0x141a015b0, "bigfish_141a015b0", fn_141a015b0 as *const ()), // sub_141a015b0
    Replacement::draft_throws(0x141a015d0, "bigfish_141a015d0", fn_141a015d0 as *const (), Throws::Discard), // sub_141a015d0
    Replacement::draft_throws(0x141a01970, "bigfish_141a01970", fn_141a01970 as *const (), Throws::Discard), // sub_141a01970
    Replacement::draft_throws(0x141a01b20, "bigfish_141a01b20", fn_141a01b20 as *const (), Throws::Discard), // sub_141a01b20
    Replacement::draft_throws(0x141a01c80, "bigfish_141a01c80", fn_141a01c80 as *const (), Throws::Discard), // sub_141a01c80
    Replacement::draft_throws(0x141a01d20, "bigfish_141a01d20", fn_141a01d20 as *const (), Throws::Caught), // sub_141a01d20
    Replacement::draft(0x141a02e10, "bigfish_141a02e10", fn_141a02e10 as *const ()), // sub_141a02e10
    Replacement::draft(0x141a031c0, "bigfish_141a031c0", fn_141a031c0 as *const ()), // sub_141a031c0
    Replacement::draft(0x141a032c0, "bigfish_141a032c0", fn_141a032c0 as *const ()), // sub_141a032c0
    Replacement::draft(0x141a032e0, "bigfish_141a032e0", fn_141a032e0 as *const ()), // sub_141a032e0
    Replacement::draft(0x141a03410, "bigfish_141a03410", fn_141a03410 as *const ()), // sub_141a03410
    Replacement::draft(0x141a03630, "bigfish_141a03630", fn_141a03630 as *const ()), // sub_141a03630
    Replacement::draft(0x141a03740, "bigfish_141a03740", fn_141a03740 as *const ()), // sub_141a03740
    Replacement::draft_throws(0x141a038d0, "bigfish_141a038d0", fn_141a038d0 as *const (), Throws::Discard), // sub_141a038d0
    Replacement::draft(0x141a03a20, "bigfish_141a03a20", tpl_95486::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_141a03a20
    Replacement::draft(0x141a03b00, "bigfish_141a03b00", tpl_35718::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_141a03b00
    Replacement::draft(0x141a03b60, "bigfish_141a03b60", tpl_35719::<0x10, 0x1401de830> as *const ()), // sub_141a03b60
    Replacement::draft(0x141a03c20, "bigfish_141a03c20", fn_141a03c20 as *const ()), // sub_141a03c20
    Replacement::draft(0x141a03d40, "bigfish_141a03d40", fn_141a03d40 as *const ()), // sub_141a03d40
    Replacement::draft(0x141a03e80, "bigfish_141a03e80", fn_141a03e80 as *const ()), // sub_141a03e80
    Replacement::draft(0x141a03f40, "bigfish_141a03f40", fn_141a03f40 as *const ()), // sub_141a03f40
    Replacement::draft(0x141a03f90, "bigfish_141a03f90", tpl_36515::<0x10, 0x1401f2bc0> as *const ()), // sub_141a03f90
    Replacement::draft(0x141a04650, "bigfish_141a04650", fn_141a04650 as *const ()), // sub_141a04650
    Replacement::draft(0x141a047e0, "bigfish_141a047e0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c9eac0, 0x158, 0x14020e8f0> as *const ()), // sub_141a047e0
    Replacement::draft(0x141a048e0, "bigfish_141a048e0", fn_141a048e0 as *const ()), // sub_141a048e0
    Replacement::draft_throws(0x141a04990, "bigfish_141a04990", fn_141a04990 as *const (), Throws::Discard), // sub_141a04990
    Replacement::draft(0x141a056a0, "bigfish_141a056a0", fn_141a056a0 as *const ()), // sub_141a056a0
    Replacement::draft(0x141a05830, "bigfish_141a05830", fn_141a05830 as *const ()), // sub_141a05830
    Replacement::draft_throws(0x141a05860, "bigfish_141a05860", fn_141a05860 as *const (), Throws::Discard), // sub_141a05860
    Replacement::draft(0x141a05930, "bigfish_141a05930", fn_141a05930 as *const ()), // sub_141a05930
    Replacement::draft_throws(0x141a05a40, "bigfish_141a05a40", tpl_34795::<0x14273afa0, 0x1410d6ca0, 0x1404924b0, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x140492790, 0x143823788, 0x14273afb0, 0x140491810, 0x1438234b0, 0x14387ecd0, 0x140492a80, 0x1402116e0> as *const (), Throws::Discard), // sub_141a05a40
    Replacement::draft(0x141a05ea0, "bigfish_141a05ea0", tpl_34605::<0x14314f7f0> as *const ()), // sub_141a05ea0
    Replacement::draft(0x141a05fe0, "bigfish_141a05fe0", tpl_94790 as *const ()), // sub_141a05fe0
    Replacement::draft(0x141a06090, "bigfish_141a06090", tpl_35013::<0x142442de0> as *const ()), // sub_141a06090
    Replacement::draft(0x141a06170, "bigfish_141a06170", tpl_34667::<0x20> as *const ()), // sub_141a06170
    Replacement::draft_throws(0x141a06560, "bigfish_141a06560", fn_141a06560 as *const (), Throws::Discard), // sub_141a06560
    Replacement::draft_throws(0x141a06810, "bigfish_141a06810", fn_141a06810 as *const (), Throws::Discard), // sub_141a06810
    Replacement::draft(0x141a068d0, "bigfish_141a068d0", fn_141a068d0 as *const ()), // sub_141a068d0
    Replacement::draft(0x141a06aa0, "bigfish_141a06aa0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ca5f00, 0x158, 0x14020e8f0> as *const ()), // sub_141a06aa0
    Replacement::draft(0x141a06ba0, "bigfish_141a06ba0", fn_141a06ba0 as *const ()), // sub_141a06ba0
    Replacement::draft(0x141a06d90, "bigfish_141a06d90", fn_141a06d90 as *const ()), // bigfish::component::GameExtendedTraversalLogic::bigfish::component::GameExtendedTraversalLogic::vfunc_2
    Replacement::draft(0x141a06de0, "bigfish_141a06de0", fn_141a06de0 as *const ()), // bigfish::component::GameExtendedTraversalLogic::bigfish::component::GameExtendedTraversalLogic::vfunc_1
    Replacement::draft_throws(0x141a06e40, "unknown_141a06e40", fn_141a06e40 as *const (), Throws::Discard), // sub_141a06e40
    Replacement::draft(0x141a07420, "unknown_141a07420", tpl_36528::<0x1401de2e0> as *const ()), // sub_141a07420
    Replacement::draft(0x141a07460, "unknown_141a07460", tpl_36528::<0x1401de2e0> as *const ()), // sub_141a07460
    Replacement::draft(0x141a07f90, "unknown_141a07f90", tpl_36544::<0x14034e370, 0x14314f9f8, 0x8, 0x1401f3d70, 0x14314f980> as *const ()), // sub_141a07f90
    Replacement::draft(0x141a08000, "unknown_141a08000", tpl_36544::<0x14034e370, 0x14314fb30, 0x8, 0x1401f3d70, 0x14314fb00> as *const ()), // sub_141a08000
    Replacement::draft(0x141a08070, "unknown_141a08070", tpl_36544::<0x14034e370, 0x14314f998, 0x8, 0x1401f3d70, 0x14314fb18> as *const ()), // sub_141a08070
    Replacement::draft(0x141a080e0, "unknown_141a080e0", fn_141a080e0 as *const ()), // sub_141a080e0
    Replacement::draft(0x141a08160, "unknown_141a08160", fn_141a08160 as *const ()), // sub_141a08160
    Replacement::draft(0x141a081c0, "unknown_141a081c0", fn_141a081c0 as *const ()), // sub_141a081c0
    Replacement::draft(0x141a08240, "unknown_141a08240", fn_141a08240 as *const ()), // sub_141a08240
    Replacement::draft(0x141a082a0, "unknown_141a082a0", tpl_4603 as *const ()), // sub_141a082a0
    Replacement::draft(0x141a082c0, "unknown_141a082c0", tpl_36549 as *const ()), // sub_141a082c0
    Replacement::draft(0x141a08310, "std_141a08310", tpl_36550::<0x1401de2e0, 0x1401de2f0, 0x14314f948> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::?bref1::operator()::<lambda_1>,void>::std::_Func_impl_no_al
    Replacement::draft(0x141a08360, "std_141a08360", tpl_36550::<0x1401de2e0, 0x1401de2f0, 0x14314f948> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::?bref1::operator()::<lambda_1>,void>::std::_Func_impl_no_al
    Replacement::draft(0x141a083b0, "std_141a083b0", tpl_36551::<0x141f56ed0, 0x143a2addc, 0x141f56e40> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::?bref1::operator()::<lambda_1>,void>::std::_Func_impl_no_al
    Replacement::draft(0x141a083e0, "std_141a083e0", tpl_36552::<0x143889f70> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::?bref1::operator()::<lambda_1>,void>::std::_Func_impl_no_al
    Replacement::draft(0x141a08400, "std_141a08400", tpl_36553 as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::?bref1::operator()::<lambda_1>,void>::std::_Func_impl_no_al
    Replacement::draft(0x141a08430, "std_141a08430", tpl_36554::<0x1401e3a60> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::?bref1::operator()::<lambda_1>,void>::std::_Func_impl_no_al
    Replacement::draft(0x141a08460, "std_141a08460", tpl_36550::<0x1401de2e0, 0x1401de2f0, 0x14314fa10> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::<lambda_1>::operator()::<lambda_2>,void>::std::_Func_impl_n
    Replacement::draft(0x141a084b0, "std_141a084b0", tpl_36550::<0x1401de2e0, 0x1401de2f0, 0x14314fa10> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::<lambda_1>::operator()::<lambda_2>,void>::std::_Func_impl_n
    Replacement::draft(0x141a08500, "std_141a08500", tpl_36551::<0x141f56ed0, 0x143a2a12c, 0x141f56e40> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::<lambda_1>::operator()::<lambda_2>,void>::std::_Func_impl_n
    Replacement::draft(0x141a08530, "std_141a08530", tpl_36552::<0x143889ff0> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::<lambda_1>::operator()::<lambda_2>,void>::std::_Func_impl_n
    Replacement::draft(0x141a08550, "std_141a08550", tpl_36553 as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::<lambda_1>::operator()::<lambda_2>,void>::std::_Func_impl_n
    Replacement::draft(0x141a08580, "std_141a08580", tpl_36554::<0x1401e3a60> as *const ()), // std::_Func_impl_no_alloc<bigfish::GameProperties::forceSave::<lambda_1>::operator()::<lambda_2>,void>::std::_Func_impl_n
    Replacement::draft(0x141a085b0, "unknown_141a085b0", tpl_4553 as *const ()), // sub_141a085b0
    Replacement::draft(0x141a085d0, "unknown_141a085d0", tpl_4553 as *const ()), // sub_141a085d0
    Replacement::draft(0x141a085f0, "unknown_141a085f0", tpl_4553 as *const ()), // sub_141a085f0
    Replacement::draft(0x141a08610, "unknown_141a08610", tpl_4553 as *const ()), // sub_141a08610
    Replacement::draft(0x141a08630, "unknown_141a08630", tpl_4553 as *const ()), // sub_141a08630
    Replacement::draft(0x141a08650, "unknown_141a08650", tpl_4553 as *const ()), // sub_141a08650
    Replacement::draft(0x141a08670, "unknown_141a08670", tpl_4662::<0x8, 0x142442de0> as *const ()), // sub_141a08670
    Replacement::draft(0x141a086e0, "unknown_141a086e0", tpl_4603 as *const ()), // sub_141a086e0
    Replacement::draft(0x141a08700, "unknown_141a08700", tpl_4603 as *const ()), // sub_141a08700
    Replacement::draft(0x141a08720, "unknown_141a08720", tpl_4603 as *const ()), // sub_141a08720
    Replacement::draft(0x141a08740, "unknown_141a08740", tpl_36549 as *const ()), // sub_141a08740
    Replacement::draft(0x141a08790, "unknown_141a08790", tpl_36555::<0x1401f3ce0> as *const ()), // sub_141a08790
    Replacement::draft(0x141a092a0, "unknown_141a092a0", fn_141a092a0 as *const ()), // sub_141a092a0
    Replacement::draft_throws(0x141a092c0, "unknown_141a092c0", fn_141a092c0 as *const (), Throws::Discard), // sub_141a092c0
    Replacement::draft(0x141a098b0, "unknown_141a098b0", fn_141a098b0 as *const ()), // sub_141a098b0
    Replacement::draft_throws(0x141a09d80, "unknown_141a09d80", fn_141a09d80 as *const (), Throws::Discard), // sub_141a09d80
    Replacement::draft(0x141a09f30, "unknown_141a09f30", fn_141a09f30 as *const ()), // sub_141a09f30
    Replacement::draft(0x141a0a020, "unknown_141a0a020", tpl_4545::<0x14028c540> as *const ()), // sub_141a0a020
    Replacement::draft(0x141a0a050, "unknown_141a0a050", tpl_4545::<0x14028c540> as *const ()), // sub_141a0a050
    Replacement::draft(0x141a0a080, "unknown_141a0a080", fn_141a0a080 as *const ()), // sub_141a0a080
    Replacement::draft(0x141a0a0d0, "unknown_141a0a0d0", fn_141a0a0d0 as *const ()), // sub_141a0a0d0
    Replacement::draft_throws(0x141a0a0f0, "unknown_141a0a0f0", fn_141a0a0f0 as *const (), Throws::Caught), // sub_141a0a0f0
    Replacement::draft(0x141a0a290, "unknown_141a0a290", fn_141a0a290 as *const ()), // sub_141a0a290
    Replacement::draft(0x141a0a2d0, "unknown_141a0a2d0", tpl_36551::<0x141f56ed0, 0x143a2a12c, 0x141f56e40> as *const ()), // sub_141a0a2d0
    Replacement::draft(0x141a0a300, "unknown_141a0a300", tpl_36551::<0x141f56ed0, 0x143a2addc, 0x141f56e40> as *const ()), // sub_141a0a300
    Replacement::draft(0x141a0a330, "unknown_141a0a330", fn_141a0a330 as *const ()), // sub_141a0a330
    Replacement::draft_throws(0x141a0a490, "unknown_141a0a490", fn_141a0a490 as *const (), Throws::Caught), // sub_141a0a490
    Replacement::draft_throws(0x141a0a8e0, "unknown_141a0a8e0", fn_141a0a8e0 as *const (), Throws::Discard), // sub_141a0a8e0
    Replacement::draft(0x141a0ad60, "unknown_141a0ad60", fn_141a0ad60 as *const ()), // sub_141a0ad60
    Replacement::draft_throws(0x141a0ae10, "unknown_141a0ae10", fn_141a0ae10 as *const (), Throws::Caught), // sub_141a0ae10
    Replacement::draft_throws(0x141a0afb0, "unknown_141a0afb0", fn_141a0afb0 as *const (), Throws::Discard), // sub_141a0afb0
    Replacement::draft_throws(0x141a0b0b0, "unknown_141a0b0b0", fn_141a0b0b0 as *const (), Throws::Discard), // sub_141a0b0b0
    Replacement::draft(0x141a0b1a0, "unknown_141a0b1a0", fn_141a0b1a0 as *const ()), // sub_141a0b1a0
    Replacement::draft(0x141a0b200, "unknown_141a0b200", fn_141a0b200 as *const ()), // sub_141a0b200
    Replacement::draft(0x141a0b220, "unknown_141a0b220", fn_141a0b220 as *const ()), // sub_141a0b220
    Replacement::draft_throws(0x141a0b3e0, "unknown_141a0b3e0", fn_141a0b3e0 as *const (), Throws::Discard), // sub_141a0b3e0
    Replacement::draft_throws(0x141a0b5e0, "unknown_141a0b5e0", fn_141a0b5e0 as *const (), Throws::Discard), // sub_141a0b5e0
    Replacement::draft_throws(0x141a0b640, "unknown_141a0b640", fn_141a0b640 as *const (), Throws::Caught), // sub_141a0b640
    Replacement::draft_throws(0x141a0bc80, "unknown_141a0bc80", fn_141a0bc80 as *const (), Throws::Caught), // sub_141a0bc80
    Replacement::draft_throws(0x141a0c880, "unknown_141a0c880", fn_141a0c880 as *const (), Throws::Discard), // sub_141a0c880
    Replacement::draft(0x141a0cdb0, "unknown_141a0cdb0", fn_141a0cdb0 as *const ()), // sub_141a0cdb0
    Replacement::draft_throws(0x141a0cdf0, "unknown_141a0cdf0", tpl_36589::<0x141a0d430, 0x143426e64, 0x1434268d0, 0x143426780, 0x1402378d0, 0x143426978, 0x143426ea4> as *const (), Throws::Discard), // sub_141a0cdf0
    Replacement::draft_throws(0x141a0cec0, "unknown_141a0cec0", tpl_36589::<0x141a0d430, 0x143426e64, 0x1434268d0, 0x1434265d4, 0x1402378d0, 0x143426abc, 0x143426ea4> as *const (), Throws::Discard), // sub_141a0cec0
    Replacement::draft_throws(0x141a0cf90, "unknown_141a0cf90", fn_141a0cf90 as *const (), Throws::Discard), // sub_141a0cf90
    Replacement::draft_throws(0x141a0cfc0, "unknown_141a0cfc0", fn_141a0cfc0 as *const (), Throws::Discard), // sub_141a0cfc0
    Replacement::draft_throws(0x141a0d000, "unknown_141a0d000", fn_141a0d000 as *const (), Throws::Discard), // sub_141a0d000
    Replacement::draft_throws(0x141a0d1e0, "unknown_141a0d1e0", fn_141a0d1e0 as *const (), Throws::Discard), // sub_141a0d1e0
    Replacement::draft_throws(0x141a0d380, "unknown_141a0d380", fn_141a0d380 as *const (), Throws::Discard), // sub_141a0d380
    Replacement::draft_throws(0x141a0d430, "unknown_141a0d430", fn_141a0d430 as *const (), Throws::Discard), // sub_141a0d430
    Replacement::draft_throws(0x141a0d4a0, "unknown_141a0d4a0", fn_141a0d4a0 as *const (), Throws::Discard), // sub_141a0d4a0
    Replacement::draft(0x141a0d600, "unknown_141a0d600", fn_141a0d600 as *const ()), // sub_141a0d600
    Replacement::draft(0x141a0d650, "unknown_141a0d650", fn_141a0d650 as *const ()), // sub_141a0d650
    Replacement::draft_throws(0x141a0d6e0, "unknown_141a0d6e0", fn_141a0d6e0 as *const (), Throws::Discard), // sub_141a0d6e0
    Replacement::draft_throws(0x141a0d7c0, "unknown_141a0d7c0", fn_141a0d7c0 as *const (), Throws::Caught), // sub_141a0d7c0
    Replacement::draft_throws(0x141a0dab0, "unknown_141a0dab0", fn_141a0dab0 as *const (), Throws::Discard), // sub_141a0dab0
    Replacement::draft_throws(0x141a0db20, "unknown_141a0db20", fn_141a0db20 as *const (), Throws::Discard), // sub_141a0db20
    Replacement::draft(0x141a0dbc0, "unknown_141a0dbc0", fn_141a0dbc0 as *const ()), // sub_141a0dbc0
    Replacement::draft_throws(0x141a0dbd0, "unknown_141a0dbd0", fn_141a0dbd0 as *const (), Throws::Discard), // sub_141a0dbd0
    Replacement::draft_throws(0x141a0dc30, "unknown_141a0dc30", fn_141a0dc30 as *const (), Throws::Discard), // sub_141a0dc30
    Replacement::draft_throws(0x141a0dcc0, "unknown_141a0dcc0", fn_141a0dcc0 as *const (), Throws::Discard), // sub_141a0dcc0
    Replacement::draft_throws(0x141a0dec0, "unknown_141a0dec0", fn_141a0dec0 as *const (), Throws::Discard), // sub_141a0dec0
    Replacement::draft(0x141a0def0, "unknown_141a0def0", fn_141a0def0 as *const ()), // sub_141a0def0
    Replacement::draft(0x141a0df20, "unknown_141a0df20", fn_141a0df20 as *const ()), // sub_141a0df20
    Replacement::draft(0x141a0e210, "unknown_141a0e210", fn_141a0e210 as *const ()), // sub_141a0e210
    Replacement::draft(0x141a0e240, "unknown_141a0e240", fn_141a0e240 as *const ()), // sub_141a0e240
    Replacement::draft_throws(0x141a0e280, "unknown_141a0e280", fn_141a0e280 as *const (), Throws::Caught), // sub_141a0e280
    Replacement::draft_throws(0x141a0ee60, "unknown_141a0ee60", fn_141a0ee60 as *const (), Throws::Discard), // sub_141a0ee60
    Replacement::draft_throws(0x141a0efc0, "unknown_141a0efc0", fn_141a0efc0 as *const (), Throws::Caught), // sub_141a0efc0
    Replacement::draft(0x141a0f680, "unknown_141a0f680", fn_141a0f680 as *const ()), // sub_141a0f680
    Replacement::draft(0x141a0f7b0, "unknown_141a0f7b0", tpl_34992::<0x1404a4800> as *const ()), // sub_141a0f7b0
    Replacement::draft(0x141a0f7d0, "unknown_141a0f7d0", tpl_34993 as *const ()), // sub_141a0f7d0
    Replacement::draft(0x141a0f810, "unknown_141a0f810", tpl_34318 as *const ()), // sub_141a0f810
    Replacement::draft(0x141a0f830, "unknown_141a0f830", tpl_34319::<0x143c954e0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0f830
    Replacement::draft(0x141a0f890, "unknown_141a0f890", tpl_34318 as *const ()), // sub_141a0f890
    Replacement::draft(0x141a0f8b0, "unknown_141a0f8b0", tpl_34318 as *const ()), // sub_141a0f8b0
    Replacement::draft(0x141a0f8d0, "unknown_141a0f8d0", tpl_34318 as *const ()), // sub_141a0f8d0
    Replacement::draft(0x141a0f8f0, "unknown_141a0f8f0", tpl_34319::<0x143c933f0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0f8f0
    Replacement::draft(0x141a0f950, "unknown_141a0f950", tpl_34319::<0x143c8ee34, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0f950
    Replacement::draft(0x141a0f9b0, "unknown_141a0f9b0", tpl_34319::<0x143c92bcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0f9b0
    Replacement::draft(0x141a0fa10, "unknown_141a0fa10", tpl_34318 as *const ()), // sub_141a0fa10
    Replacement::draft(0x141a0fa30, "unknown_141a0fa30", tpl_34318 as *const ()), // sub_141a0fa30
    Replacement::draft(0x141a0fa50, "unknown_141a0fa50", tpl_34318 as *const ()), // sub_141a0fa50
    Replacement::draft(0x141a0fa70, "unknown_141a0fa70", tpl_34318 as *const ()), // sub_141a0fa70
    Replacement::draft(0x141a0fa90, "unknown_141a0fa90", tpl_34318 as *const ()), // sub_141a0fa90
    Replacement::draft(0x141a0fab0, "unknown_141a0fab0", tpl_34319::<0x143c8edbc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fab0
    Replacement::draft(0x141a0fb10, "unknown_141a0fb10", tpl_34319::<0x143c8dbcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fb10
    Replacement::draft(0x141a0fb70, "unknown_141a0fb70", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fb70
    Replacement::draft(0x141a0fbd0, "unknown_141a0fbd0", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fbd0
    Replacement::draft(0x141a0fc30, "unknown_141a0fc30", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fc30
    Replacement::draft(0x141a0fc90, "unknown_141a0fc90", tpl_34318 as *const ()), // sub_141a0fc90
    Replacement::draft(0x141a0fcb0, "unknown_141a0fcb0", tpl_34318 as *const ()), // sub_141a0fcb0
    Replacement::draft(0x141a0fcd0, "unknown_141a0fcd0", tpl_34318 as *const ()), // sub_141a0fcd0
    Replacement::draft(0x141a0fcf0, "unknown_141a0fcf0", tpl_34319::<0x143c8e658, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fcf0
    Replacement::draft(0x141a0fd50, "unknown_141a0fd50", tpl_34319::<0x143c8d508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fd50
    Replacement::draft(0x141a0fdb0, "unknown_141a0fdb0", tpl_34319::<0x143c8dbd0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fdb0
    Replacement::draft(0x141a0fe10, "unknown_141a0fe10", tpl_34318 as *const ()), // sub_141a0fe10
    Replacement::draft(0x141a0fe30, "unknown_141a0fe30", tpl_34319::<0x143ca6370, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0fe30
    Replacement::draft(0x141a0fe90, "unknown_141a0fe90", tpl_34318 as *const ()), // sub_141a0fe90
    Replacement::draft(0x141a0feb0, "unknown_141a0feb0", tpl_34319::<0x143c8e660, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a0feb0
    Replacement::draft(0x141a0ff10, "unknown_141a0ff10", tpl_361 as *const ()), // sub_141a0ff10
    Replacement::draft(0x141a0ff20, "unknown_141a0ff20", fn_141a0ff20 as *const ()), // sub_141a0ff20
    Replacement::draft(0x141a100d0, "unknown_141a100d0", tpl_361 as *const ()), // sub_141a100d0
    Replacement::draft_throws(0x141a100e0, "unknown_141a100e0", fn_141a100e0 as *const (), Throws::Caught), // sub_141a100e0
    Replacement::draft(0x141a10cd0, "unknown_141a10cd0", tpl_361 as *const ()), // sub_141a10cd0
    Replacement::draft(0x141a10ce0, "unknown_141a10ce0", fn_141a10ce0 as *const ()), // sub_141a10ce0
    Replacement::draft(0x141a10d50, "unknown_141a10d50", tpl_361 as *const ()), // sub_141a10d50
    Replacement::draft(0x141a10d60, "unknown_141a10d60", fn_141a10d60 as *const ()), // sub_141a10d60
    Replacement::draft(0x141a11620, "unknown_141a11620", tpl_361 as *const ()), // sub_141a11620
    Replacement::draft_throws(0x141a11630, "unknown_141a11630", fn_141a11630 as *const (), Throws::Caught), // sub_141a11630
    Replacement::draft(0x141a11990, "unknown_141a11990", tpl_361 as *const ()), // sub_141a11990
    Replacement::draft(0x141a119e0, "unknown_141a119e0", fn_141a119e0 as *const ()), // sub_141a119e0
    Replacement::draft(0x141a12060, "unknown_141a12060", tpl_361 as *const ()), // sub_141a12060
    Replacement::draft_throws(0x141a12070, "unknown_141a12070", fn_141a12070 as *const (), Throws::Discard), // sub_141a12070
    Replacement::draft(0x141a12590, "unknown_141a12590", fn_141a12590 as *const ()), // sub_141a12590
    Replacement::draft(0x141a12e30, "unknown_141a12e30", fn_141a12e30 as *const ()), // sub_141a12e30
    Replacement::draft(0x141a12e70, "unknown_141a12e70", tpl_34252 as *const ()), // sub_141a12e70
    Replacement::draft(0x141a12e80, "unknown_141a12e80", fn_141a12e80 as *const ()), // sub_141a12e80
    Replacement::draft_throws(0x141a12ee0, "unknown_141a12ee0", fn_141a12ee0 as *const (), Throws::Caught), // sub_141a12ee0
    Replacement::draft_throws(0x141a13580, "unknown_141a13580", fn_141a13580 as *const (), Throws::Caught), // sub_141a13580
    Replacement::draft(0x141a13b90, "unknown_141a13b90", fn_141a13b90 as *const ()), // sub_141a13b90
    Replacement::draft(0x141a13bf0, "unknown_141a13bf0", fn_141a13bf0 as *const ()), // sub_141a13bf0
    Replacement::draft_throws(0x141a13cc0, "unknown_141a13cc0", fn_141a13cc0 as *const (), Throws::Discard), // sub_141a13cc0
    Replacement::draft(0x141a13e10, "unknown_141a13e10", fn_141a13e10 as *const ()), // sub_141a13e10
    Replacement::draft(0x141a141d0, "unknown_141a141d0", fn_141a141d0 as *const ()), // sub_141a141d0
    Replacement::draft(0x141a14200, "unknown_141a14200", fn_141a14200 as *const ()), // sub_141a14200
    Replacement::draft(0x141a14310, "unknown_141a14310", fn_141a14310 as *const ()), // sub_141a14310
    Replacement::draft(0x141a147f0, "unknown_141a147f0", fn_141a147f0 as *const ()), // sub_141a147f0
    Replacement::draft(0x141a15450, "unknown_141a15450", tpl_34970::<0x8, 0x68> as *const ()), // sub_141a15450
    Replacement::draft(0x141a154a0, "unknown_141a154a0", tpl_36628::<0x143152620, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d88c, 0x14240ab10> as *const ()), // sub_141a154a0
    Replacement::draft(0x141a15b10, "unknown_141a15b10", tpl_36629::<0x1431525c8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8c0, 0x14240ab10> as *const ()), // sub_141a15b10
    Replacement::draft(0x141a16180, "unknown_141a16180", tpl_36630::<0x1431525d8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8bc, 0x14240ab10> as *const ()), // sub_141a16180
    Replacement::draft(0x141a167f0, "unknown_141a167f0", tpl_36631::<0x1431525e8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d90c, 0x14240ab10> as *const ()), // sub_141a167f0
    Replacement::draft(0x141a16e60, "unknown_141a16e60", tpl_36632::<0x143152600, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d924, 0x14240ab10> as *const ()), // sub_141a16e60
    Replacement::draft(0x141a174d0, "unknown_141a174d0", tpl_36633::<0x143152548, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d914, 0x14240ab10> as *const ()), // sub_141a174d0
    Replacement::draft(0x141a17b40, "unknown_141a17b40", tpl_36633::<0x143152568, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d91c, 0x14240ab10> as *const ()), // sub_141a17b40
    Replacement::draft(0x141a181b0, "unknown_141a181b0", tpl_36633::<0x143152588, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d918, 0x14240ab10> as *const ()), // sub_141a181b0
    Replacement::draft(0x141a18820, "unknown_141a18820", tpl_36634::<0x1431525a8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d910, 0x14240ab10> as *const ()), // sub_141a18820
    Replacement::draft(0x141a18e90, "unknown_141a18e90", tpl_36635::<0x143152730, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d920, 0x14240ab10> as *const ()), // sub_141a18e90
    Replacement::draft(0x141a19500, "unknown_141a19500", tpl_36631::<0x143152748, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d868, 0x14240ab10> as *const ()), // sub_141a19500
    Replacement::draft(0x141a19b70, "unknown_141a19b70", tpl_36636::<0x143152760, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8d0, 0x14240ab10> as *const ()), // sub_141a19b70
    Replacement::draft(0x141a1a1e0, "unknown_141a1a1e0", tpl_36637::<0x143152778, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d908, 0x14240ab10> as *const ()), // sub_141a1a1e0
    Replacement::draft(0x141a1a850, "unknown_141a1a850", tpl_36632::<0x1431526d0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d99c, 0x14240ab10> as *const ()), // sub_141a1a850
    Replacement::draft(0x141a1aec0, "unknown_141a1aec0", tpl_36636::<0x1431526f0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d990, 0x14240ab10> as *const ()), // sub_141a1aec0
    Replacement::draft(0x141a1b530, "unknown_141a1b530", tpl_36631::<0x143152708, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d98c, 0x14240ab10> as *const ()), // sub_141a1b530
    Replacement::draft(0x141a1bba0, "unknown_141a1bba0", tpl_36638::<0x143152720, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d9a8, 0x14240ab10> as *const ()), // sub_141a1bba0
    Replacement::draft(0x141a1c210, "unknown_141a1c210", tpl_36638::<0x143152688, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d994, 0x14240ab10> as *const ()), // sub_141a1c210
    Replacement::draft(0x141a1c880, "unknown_141a1c880", tpl_36639::<0x143152698, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d998, 0x14240ab10> as *const ()), // sub_141a1c880
    Replacement::draft(0x141a1cef0, "unknown_141a1cef0", tpl_36638::<0x1431526b0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d988, 0x14240ab10> as *const ()), // sub_141a1cef0
    Replacement::draft(0x141a1d560, "unknown_141a1d560", tpl_36640::<0x1431526c0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d9a0, 0x14240ab10> as *const ()), // sub_141a1d560
    Replacement::draft(0x141a1dbd0, "unknown_141a1dbd0", tpl_36641::<0x143152640, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d9a4, 0x14240ab10> as *const ()), // sub_141a1dbd0
    Replacement::draft(0x141a1e240, "unknown_141a1e240", tpl_36638::<0x143152658, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d984, 0x14240ab10> as *const ()), // sub_141a1e240
    Replacement::draft(0x141a1e8b0, "unknown_141a1e8b0", tpl_36638::<0x143152668, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d980, 0x14240ab10> as *const ()), // sub_141a1e8b0
    Replacement::draft(0x141a1ef20, "unknown_141a1ef20", tpl_36642::<0x143152678, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d858, 0x14240ab10> as *const ()), // sub_141a1ef20
    Replacement::draft(0x141a1f590, "unknown_141a1f590", tpl_36628::<0x1431528b0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8f8, 0x14240ab10> as *const ()), // sub_141a1f590
    Replacement::draft(0x141a1fc00, "unknown_141a1fc00", tpl_36631::<0x1431528d0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8f0, 0x14240ab10> as *const ()), // sub_141a1fc00
    Replacement::draft(0x141a20270, "unknown_141a20270", tpl_36638::<0x1431528e8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8f4, 0x14240ab10> as *const ()), // sub_141a20270
    Replacement::draft(0x141a208e0, "unknown_141a208e0", tpl_36641::<0x1431528f8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8fc, 0x14240ab10> as *const ()), // sub_141a208e0
    Replacement::draft(0x141a20f50, "unknown_141a20f50", tpl_36642::<0x143152850, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d900, 0x14240ab10> as *const ()), // sub_141a20f50
    Replacement::draft(0x141a215c0, "unknown_141a215c0", tpl_36642::<0x143152860, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d904, 0x14240ab10> as *const ()), // sub_141a215c0
    Replacement::draft(0x141a21c30, "unknown_141a21c30", tpl_36630::<0x1431393a0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d9ac, 0x14240ab10> as *const ()), // sub_141a21c30
    Replacement::draft(0x141a222a0, "unknown_141a222a0", tpl_36643::<0x143139390, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d9b0, 0x14240ab10> as *const ()), // sub_141a222a0
    Replacement::draft(0x141a22910, "unknown_141a22910", tpl_36644::<0x143152870, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8e0, 0x14240ab10> as *const ()), // sub_141a22910
    Replacement::draft(0x141a22f80, "unknown_141a22f80", tpl_36634::<0x143152890, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8e4, 0x14240ab10> as *const ()), // sub_141a22f80
    Replacement::draft(0x141a235f0, "unknown_141a235f0", tpl_36641::<0x1431527e8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8b4, 0x14240ab10> as *const ()), // sub_141a235f0
    Replacement::draft(0x141a23c60, "unknown_141a23c60", tpl_36644::<0x143152800, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d89c, 0x14240ab10> as *const ()), // sub_141a23c60
    Replacement::draft(0x141a242d0, "unknown_141a242d0", tpl_36645::<0x143152820, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d884, 0x14240ab10> as *const ()), // sub_141a242d0
    Replacement::draft(0x141a24940, "unknown_141a24940", tpl_36637::<0x143152838, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8e8, 0x14240ab10> as *const ()), // sub_141a24940
    Replacement::draft(0x141a24fb0, "unknown_141a24fb0", tpl_36635::<0x143152790, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8d8, 0x14240ab10> as *const ()), // sub_141a24fb0
    Replacement::draft(0x141a25620, "unknown_141a25620", tpl_36637::<0x1431527a8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d890, 0x14240ab10> as *const ()), // sub_141a25620
    Replacement::draft(0x141a25c90, "unknown_141a25c90", tpl_36631::<0x1431527c0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d888, 0x14240ab10> as *const ()), // sub_141a25c90
    Replacement::draft(0x141a26300, "unknown_141a26300", tpl_36635::<0x143106cf8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d928, 0x14240ab10> as *const ()), // sub_141a26300
    Replacement::draft(0x141a26970, "unknown_141a26970", tpl_36646::<0x1431527d8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8c4, 0x14240ab10> as *const ()), // sub_141a26970
    Replacement::draft(0x141a26fe0, "unknown_141a26fe0", fn_141a26fe0 as *const ()), // sub_141a26fe0
    Replacement::draft(0x141a27650, "unknown_141a27650", tpl_36641::<0x143152a10, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8ec, 0x14240ab10> as *const ()), // sub_141a27650
    Replacement::draft(0x141a27cc0, "unknown_141a27cc0", tpl_36648::<0x143152a28, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d878, 0x14240ab10> as *const ()), // sub_141a27cc0
    Replacement::draft(0x141a28330, "unknown_141a28330", tpl_36649::<0x143152a38, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d880, 0x14240ab10> as *const ()), // sub_141a28330
    Replacement::draft(0x141a289a0, "unknown_141a289a0", tpl_36639::<0x1431529b0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8b8, 0x14240ab10> as *const ()), // sub_141a289a0
    Replacement::draft(0x141a29010, "unknown_141a29010", tpl_36628::<0x143152c20, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8a4, 0x14240ab10> as *const ()), // sub_141a29010
    Replacement::draft(0x141a29680, "unknown_141a29680", tpl_36636::<0x1431529c8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d894, 0x14240ab10> as *const ()), // sub_141a29680
    Replacement::draft(0x141a29cf0, "unknown_141a29cf0", tpl_36639::<0x1431529e0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d874, 0x14240ab10> as *const ()), // sub_141a29cf0
    Replacement::draft(0x141a2a360, "unknown_141a2a360", tpl_36637::<0x1431529f8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8b0, 0x14240ab10> as *const ()), // sub_141a2a360
    Replacement::draft(0x141a2a9d0, "unknown_141a2a9d0", tpl_36639::<0x143152950, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d860, 0x14240ab10> as *const ()), // sub_141a2a9d0
    Replacement::draft(0x141a2b040, "unknown_141a2b040", tpl_36639::<0x143152968, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8c8, 0x14240ab10> as *const ()), // sub_141a2b040
    Replacement::draft(0x141a2b6b0, "unknown_141a2b6b0", tpl_36639::<0x143152980, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d898, 0x14240ab10> as *const ()), // sub_141a2b6b0
    Replacement::draft(0x141a2bd20, "unknown_141a2bd20", tpl_36639::<0x143152998, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d87c, 0x14240ab10> as *const ()), // sub_141a2bd20
    Replacement::draft(0x141a2c390, "unknown_141a2c390", tpl_36629::<0x143152910, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d93c, 0x14240ab10> as *const ()), // sub_141a2c390
    Replacement::draft(0x141a2ca00, "unknown_141a2ca00", tpl_36642::<0x143152920, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d944, 0x14240ab10> as *const ()), // sub_141a2ca00
    Replacement::draft(0x141a2d070, "unknown_141a2d070", tpl_36642::<0x143152930, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d960, 0x14240ab10> as *const ()), // sub_141a2d070
    Replacement::draft(0x141a2d6e0, "unknown_141a2d6e0", tpl_36638::<0x143152940, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d97c, 0x14240ab10> as *const ()), // sub_141a2d6e0
    Replacement::draft(0x141a2dd50, "unknown_141a2dd50", tpl_36631::<0x1431524c0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d950, 0x14240ab10> as *const ()), // sub_141a2dd50
    Replacement::draft(0x141a2e3c0, "unknown_141a2e3c0", tpl_36646::<0x1431524d8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d85c, 0x14240ab10> as *const ()), // sub_141a2e3c0
    Replacement::draft(0x141a2ea30, "unknown_141a2ea30", tpl_36650::<0x1431524e4, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8a8, 0x14240ab10> as *const ()), // sub_141a2ea30
    Replacement::draft(0x141a2f0a0, "unknown_141a2f0a0", tpl_36649::<0x1431524f0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8ac, 0x14240ab10> as *const ()), // sub_141a2f0a0
    Replacement::draft(0x141a2f710, "unknown_141a2f710", tpl_36648::<0x143152498, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d968, 0x14240ab10> as *const ()), // sub_141a2f710
    Replacement::draft(0x141a2fd80, "unknown_141a2fd80", tpl_36650::<0x1431524a4, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d938, 0x14240ab10> as *const ()), // sub_141a2fd80
    Replacement::draft(0x141a303f0, "unknown_141a303f0", tpl_36651::<0x1431524ac, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d96c, 0x14240ab10> as *const ()), // sub_141a303f0
    Replacement::draft(0x141a30a60, "unknown_141a30a60", tpl_36648::<0x1431202d8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d958, 0x14240ab10> as *const ()), // sub_141a30a60
    Replacement::draft(0x141a310d0, "unknown_141a310d0", tpl_36631::<0x1430a10b0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d974, 0x14240ab10> as *const ()), // sub_141a310d0
    Replacement::draft(0x141a31740, "unknown_141a31740", tpl_36639::<0x1430a1098, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d870, 0x14240ab10> as *const ()), // sub_141a31740
    Replacement::draft(0x141a31db0, "unknown_141a31db0", tpl_36652::<0x1430a1080, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8d4, 0x14240ab10> as *const ()), // sub_141a31db0
    Replacement::draft(0x141a32420, "unknown_141a32420", tpl_36650::<0x1430a1078, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d934, 0x14240ab10> as *const ()), // sub_141a32420
    Replacement::draft(0x141a32a90, "unknown_141a32a90", tpl_36650::<0x1431524b4, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d940, 0x14240ab10> as *const ()), // sub_141a32a90
    Replacement::draft(0x141a33100, "unknown_141a33100", tpl_36628::<0x143152448, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d948, 0x14240ab10> as *const ()), // sub_141a33100
    Replacement::draft(0x141a33770, "unknown_141a33770", tpl_36639::<0x143152468, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d930, 0x14240ab10> as *const ()), // sub_141a33770
    Replacement::draft(0x141a33de0, "unknown_141a33de0", tpl_36639::<0x143152480, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d92c, 0x14240ab10> as *const ()), // sub_141a33de0
    Replacement::draft(0x141a34450, "unknown_141a34450", tpl_36652::<0x1431523f0, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d970, 0x14240ab10> as *const ()), // sub_141a34450
    Replacement::draft(0x141a34ac0, "unknown_141a34ac0", tpl_36634::<0x143152408, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d95c, 0x14240ab10> as *const ()), // sub_141a34ac0
    Replacement::draft(0x141a35130, "unknown_141a35130", tpl_36640::<0x143152428, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d954, 0x14240ab10> as *const ()), // sub_141a35130
    Replacement::draft(0x141a357a0, "unknown_141a357a0", tpl_36643::<0x143152438, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d94c, 0x14240ab10> as *const ()), // sub_141a357a0
    Replacement::draft(0x141a35e10, "unknown_141a35e10", tpl_36650::<0x143152518, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d86c, 0x14240ab10> as *const ()), // sub_141a35e10
    Replacement::draft(0x141a36480, "unknown_141a36480", tpl_36650::<0x143152520, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8dc, 0x14240ab10> as *const ()), // sub_141a36480
    Replacement::draft(0x141a36af0, "unknown_141a36af0", fn_141a36af0 as *const ()), // sub_141a36af0
    Replacement::draft(0x141a37160, "unknown_141a37160", tpl_36641::<0x143152530, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d864, 0x14240ab10> as *const ()), // sub_141a37160
    Replacement::draft(0x141a377d0, "unknown_141a377d0", tpl_36636::<0x1431524f8, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d8a0, 0x14240ab10> as *const ()), // sub_141a377d0
    Replacement::draft(0x141a37e40, "unknown_141a37e40", tpl_36650::<0x143152510, 0x142409b50, 0x143849db8, 0x1404a78e0, 0x143849dc0, 0x14240a730, 0x14397d964, 0x14240ab10> as *const ()), // sub_141a37e40
    Replacement::draft_throws(0x141a384b0, "unknown_141a384b0", fn_141a384b0 as *const (), Throws::Caught), // sub_141a384b0
    Replacement::draft(0x141a388b0, "unknown_141a388b0", fn_141a388b0 as *const ()), // sub_141a388b0
    Replacement::draft(0x141a388f0, "unknown_141a388f0", fn_141a388f0 as *const ()), // sub_141a388f0
    Replacement::draft_throws(0x141a38920, "unknown_141a38920", fn_141a38920 as *const (), Throws::Caught), // sub_141a38920
    Replacement::draft(0x141a389b0, "unknown_141a389b0", tpl_34252 as *const ()), // sub_141a389b0
    Replacement::draft(0x141a389c0, "unknown_141a389c0", fn_141a389c0 as *const ()), // sub_141a389c0
    Replacement::draft(0x141a38d00, "unknown_141a38d00", fn_141a38d00 as *const ()), // sub_141a38d00
    Replacement::draft(0x141a39030, "unknown_141a39030", fn_141a39030 as *const ()), // sub_141a39030
    Replacement::draft_throws(0x141a39070, "unknown_141a39070", fn_141a39070 as *const (), Throws::Discard), // sub_141a39070
    Replacement::draft(0x141a39270, "unknown_141a39270", fn_141a39270 as *const ()), // sub_141a39270
    Replacement::draft(0x141a39320, "unknown_141a39320", tpl_36663::<0x28, 0x14200ddb0> as *const ()), // sub_141a39320
    Replacement::draft(0x141a39330, "unknown_141a39330", fn_141a39330 as *const ()), // sub_141a39330
    Replacement::draft(0x141a39610, "unknown_141a39610", fn_141a39610 as *const ()), // sub_141a39610
    Replacement::draft(0x141a39850, "unknown_141a39850", tpl_1::<0x143a28aa0> as *const ()), // sub_141a39850
    Replacement::draft(0x141a39860, "unknown_141a39860", tpl_1::<0x143a28b40> as *const ()), // sub_141a39860
    Replacement::draft(0x141a39870, "unknown_141a39870", tpl_36666::<0x1420e8920, 0x5aa0, 0x143a340f8, 0x4fd0, 0x5860, 0x5861> as *const ()), // sub_141a39870
    Replacement::draft_throws(0x141a398d0, "unknown_141a398d0", fn_141a398d0 as *const (), Throws::Caught), // sub_141a398d0
    Replacement::draft_throws(0x141a39a40, "unknown_141a39a40", fn_141a39a40 as *const (), Throws::Discard), // sub_141a39a40
    Replacement::draft_throws(0x141a39ad0, "unknown_141a39ad0", fn_141a39ad0 as *const (), Throws::Caught), // sub_141a39ad0
    Replacement::draft_throws(0x141a39ca0, "unknown_141a39ca0", fn_141a39ca0 as *const (), Throws::Caught), // sub_141a39ca0
    Replacement::draft(0x141a39ff0, "std_141a39ff0", tpl_23342::<0x143153018, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::hasFiringLOSToPosition::<lambda_1>,bool,const coregame::PhysicsSweepHit&>:
    Replacement::draft(0x141a3a010, "std_141a3a010", tpl_23342::<0x143153018, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::hasFiringLOSToPosition::<lambda_1>,bool,const coregame::PhysicsSweepHit&>:
    Replacement::draft(0x141a3a030, "std_141a3a030", fn_141a3a030 as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::hasFiringLOSToPosition::<lambda_1>,bool,const coregame::PhysicsSweepHit&>:
    Replacement::draft(0x141a3a110, "std_141a3a110", tpl_1::<0x14388a8e0> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::hasFiringLOSToPosition::<lambda_1>,bool,const coregame::PhysicsSweepHit&>:
    Replacement::draft(0x141a3a120, "std_141a3a120", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::hasFiringLOSToPosition::<lambda_1>,bool,const coregame::PhysicsSweepHit&>:
    Replacement::draft(0x141a3a130, "std_141a3a130", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::hasFiringLOSToPosition::<lambda_1>,bool,const coregame::PhysicsSweepHit&>:
    Replacement::draft(0x141a3a150, "std_141a3a150", tpl_34318 as *const ()), // sub_141a3a150
    Replacement::draft(0x141a3a170, "std_141a3a170", tpl_34319::<0x143c8e710, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a3a170
    Replacement::draft(0x141a3a1d0, "std_141a3a1d0", tpl_770::<0x143152fe0> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_1>,bool,m::?bref9>::std::_Func_im
    Replacement::draft(0x141a3a1f0, "std_141a3a1f0", tpl_770::<0x143152fe0> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_1>,bool,m::?bref9>::std::_Func_im
    Replacement::draft(0x141a3a220, "std_141a3a220", tpl_1::<0x14388a990> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_1>,bool,m::?bref9>::std::_Func_im
    Replacement::draft(0x141a3a230, "std_141a3a230", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_1>,bool,m::?bref9>::std::_Func_im
    Replacement::draft(0x141a3a240, "std_141a3a240", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_1>,bool,m::?bref9>::std::_Func_im
    Replacement::draft(0x141a3a260, "std_141a3a260", tpl_550::<0x143152eb8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_2>,float,const m::Vector3Template
    Replacement::draft(0x141a3a280, "std_141a3a280", tpl_550::<0x143152eb8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_2>,float,const m::Vector3Template
    Replacement::draft(0x141a3a2a0, "std_141a3a2a0", tpl_33904::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_2>,float,const m::Vector3Template
    Replacement::draft(0x141a3a2b0, "std_141a3a2b0", tpl_1::<0x14388a080> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_2>,float,const m::Vector3Template
    Replacement::draft(0x141a3a2c0, "std_141a3a2c0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_2>,float,const m::Vector3Template
    Replacement::draft(0x141a3a2d0, "std_141a3a2d0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_2>,float,const m::Vector3Template
    Replacement::draft(0x141a3a2f0, "std_141a3a2f0", tpl_550::<0x143152ef0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_3>,float,const m::Vector3Template
    Replacement::draft(0x141a3a310, "std_141a3a310", tpl_550::<0x143152ef0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_3>,float,const m::Vector3Template
    Replacement::draft(0x141a3a330, "std_141a3a330", tpl_33904::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_3>,float,const m::Vector3Template
    Replacement::draft(0x141a3a340, "std_141a3a340", tpl_1::<0x14388a4b0> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_3>,float,const m::Vector3Template
    Replacement::draft(0x141a3a350, "std_141a3a350", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_3>,float,const m::Vector3Template
    Replacement::draft(0x141a3a360, "std_141a3a360", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::gamehelper::handleChunkDamageWithinDistance::<lambda_3>,float,const m::Vector3Template
    Replacement::draft(0x141a3a380, "unknown_141a3a380", tpl_34318 as *const ()), // sub_141a3a380
    Replacement::draft(0x141a3a3a0, "unknown_141a3a3a0", tpl_34318 as *const ()), // sub_141a3a3a0
    Replacement::draft(0x141a3a3c0, "unknown_141a3a3c0", tpl_34319::<0x143c8dbc8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a3a3c0
    Replacement::draft(0x141a3a420, "unknown_141a3a420", tpl_34319::<0x143c8dc20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a3a420
    Replacement::draft(0x141a3a480, "unknown_141a3a480", tpl_361 as *const ()), // sub_141a3a480
    Replacement::draft(0x141a3a490, "unknown_141a3a490", tpl_22031::<0x8> as *const ()), // sub_141a3a490
    Replacement::draft(0x141a3a4b0, "unknown_141a3a4b0", fn_141a3a4b0 as *const ()), // sub_141a3a4b0
    Replacement::draft(0x141a3a560, "unknown_141a3a560", tpl_36673 as *const ()), // sub_141a3a560
    Replacement::draft(0x141a3a570, "unknown_141a3a570", fn_141a3a570 as *const ()), // sub_141a3a570
    Replacement::draft_throws(0x141a3a5d0, "unknown_141a3a5d0", fn_141a3a5d0 as *const (), Throws::Discard), // sub_141a3a5d0
    Replacement::draft(0x141a3aa70, "unknown_141a3aa70", tpl_361 as *const ()), // sub_141a3aa70
    Replacement::draft(0x141a3aa80, "unknown_141a3aa80", fn_141a3aa80 as *const ()), // sub_141a3aa80
    Replacement::draft(0x141a3aac0, "unknown_141a3aac0", fn_141a3aac0 as *const ()), // sub_141a3aac0
    Replacement::draft_throws(0x141a3ad40, "unknown_141a3ad40", fn_141a3ad40 as *const (), Throws::Discard), // sub_141a3ad40
    Replacement::draft(0x141a3aea0, "unknown_141a3aea0", tpl_34443 as *const ()), // sub_141a3aea0
    Replacement::draft(0x141a3aeb0, "unknown_141a3aeb0", tpl_36677 as *const ()), // sub_141a3aeb0
    Replacement::draft(0x141a3aec0, "unknown_141a3aec0", tpl_34443 as *const ()), // sub_141a3aec0
    Replacement::draft(0x141a3aed0, "unknown_141a3aed0", tpl_36677 as *const ()), // sub_141a3aed0
    Replacement::draft(0x141a3aee0, "unknown_141a3aee0", tpl_496 as *const ()), // sub_141a3aee0
    Replacement::draft_throws(0x141a3aef0, "unknown_141a3aef0", fn_141a3aef0 as *const (), Throws::Discard), // sub_141a3aef0
    Replacement::draft(0x141a3b120, "unknown_141a3b120", tpl_22031::<0x8> as *const ()), // sub_141a3b120
    Replacement::draft(0x141a3b140, "unknown_141a3b140", fn_141a3b140 as *const ()), // sub_141a3b140
    Replacement::draft(0x141a3b220, "unknown_141a3b220", fn_141a3b220 as *const ()), // sub_141a3b220
    Replacement::draft_throws(0x141a3b250, "unknown_141a3b250", fn_141a3b250 as *const (), Throws::Discard), // sub_141a3b250
    Replacement::draft_throws(0x141a3b820, "unknown_141a3b820", fn_141a3b820 as *const (), Throws::Discard), // sub_141a3b820
    Replacement::draft(0x141a3c0b0, "unknown_141a3c0b0", fn_141a3c0b0 as *const ()), // sub_141a3c0b0
    Replacement::draft(0x141a3c1b0, "unknown_141a3c1b0", fn_141a3c1b0 as *const ()), // sub_141a3c1b0
    Replacement::draft(0x141a3c810, "unknown_141a3c810", fn_141a3c810 as *const ()), // sub_141a3c810
    Replacement::draft(0x141a3c980, "unknown_141a3c980", fn_141a3c980 as *const ()), // sub_141a3c980
    Replacement::draft(0x141a3c9d0, "unknown_141a3c9d0", fn_141a3c9d0 as *const ()), // sub_141a3c9d0
    Replacement::draft(0x141a3ca30, "unknown_141a3ca30", fn_141a3ca30 as *const ()), // sub_141a3ca30
    Replacement::draft(0x141a3cbb0, "unknown_141a3cbb0", tpl_34667::<0x20> as *const ()), // sub_141a3cbb0
    Replacement::draft(0x141a3cc50, "unknown_141a3cc50", fn_141a3cc50 as *const ()), // sub_141a3cc50
    Replacement::draft(0x141a3cd00, "unknown_141a3cd00", tpl_34611::<0x8> as *const ()), // sub_141a3cd00
    Replacement::draft(0x141a3d7a0, "unknown_141a3d7a0", fn_141a3d7a0 as *const ()), // sub_141a3d7a0
    Replacement::draft_throws(0x141a3d980, "unknown_141a3d980", fn_141a3d980 as *const (), Throws::Discard), // sub_141a3d980
    Replacement::draft(0x141a3e6d0, "unknown_141a3e6d0", fn_141a3e6d0 as *const ()), // sub_141a3e6d0
    Replacement::draft_throws(0x141a3ea70, "unknown_141a3ea70", fn_141a3ea70 as *const (), Throws::Discard), // sub_141a3ea70
    Replacement::draft(0x141a3efa0, "unknown_141a3efa0", fn_141a3efa0 as *const ()), // sub_141a3efa0
    Replacement::draft(0x141a3f590, "unknown_141a3f590", fn_141a3f590 as *const ()), // sub_141a3f590
    Replacement::draft(0x141a3f670, "unknown_141a3f670", fn_141a3f670 as *const ()), // sub_141a3f670
    Replacement::draft(0x141a3f850, "unknown_141a3f850", fn_141a3f850 as *const ()), // sub_141a3f850
    Replacement::draft(0x141a3f8a0, "unknown_141a3f8a0", fn_141a3f8a0 as *const ()), // sub_141a3f8a0
    Replacement::draft_throws(0x141a3f900, "unknown_141a3f900", fn_141a3f900 as *const (), Throws::Caught), // sub_141a3f900
    Replacement::draft_throws(0x141a3faf0, "unknown_141a3faf0", fn_141a3faf0 as *const (), Throws::Discard), // sub_141a3faf0
    Replacement::draft_throws(0x141a3fb50, "unknown_141a3fb50", fn_141a3fb50 as *const (), Throws::Discard), // sub_141a3fb50
    Replacement::draft_throws(0x141a3fdd0, "unknown_141a3fdd0", fn_141a3fdd0 as *const (), Throws::Discard), // sub_141a3fdd0
    Replacement::draft_throws(0x141a41390, "unknown_141a41390", fn_141a41390 as *const (), Throws::Discard), // sub_141a41390
    Replacement::draft_throws(0x141a41fa0, "unknown_141a41fa0", fn_141a41fa0 as *const (), Throws::Discard), // sub_141a41fa0
    Replacement::draft(0x141a429a0, "unknown_141a429a0", fn_141a429a0 as *const ()), // sub_141a429a0
    Replacement::draft(0x141a42ab0, "unknown_141a42ab0", fn_141a42ab0 as *const ()), // sub_141a42ab0
    Replacement::draft(0x141a42dd0, "unknown_141a42dd0", fn_141a42dd0 as *const ()), // sub_141a42dd0
    Replacement::draft(0x141a42fb0, "unknown_141a42fb0", fn_141a42fb0 as *const ()), // sub_141a42fb0
    Replacement::draft(0x141a43040, "unknown_141a43040", fn_141a43040 as *const ()), // sub_141a43040
    Replacement::draft(0x141a431d0, "unknown_141a431d0", fn_141a431d0 as *const ()), // sub_141a431d0
    Replacement::draft(0x141a43db0, "unknown_141a43db0", fn_141a43db0 as *const ()), // sub_141a43db0
    Replacement::draft(0x141a43ee0, "unknown_141a43ee0", fn_141a43ee0 as *const ()), // sub_141a43ee0
    Replacement::draft_throws(0x141a44120, "unknown_141a44120", fn_141a44120 as *const (), Throws::Caught), // sub_141a44120
    Replacement::draft_throws(0x141a44550, "unknown_141a44550", fn_141a44550 as *const (), Throws::Discard), // sub_141a44550
    Replacement::draft(0x141a44c00, "unknown_141a44c00", fn_141a44c00 as *const ()), // sub_141a44c00
    Replacement::draft(0x141a44c60, "unknown_141a44c60", tpl_36666::<0x1420e8920, 0x5aa0, 0x143a340f8, 0x4fd0, 0x5860, 0x5861> as *const ()), // sub_141a44c60
    Replacement::draft_throws(0x141a44cc0, "unknown_141a44cc0", fn_141a44cc0 as *const (), Throws::Caught), // sub_141a44cc0
    Replacement::draft(0x141a45430, "unknown_141a45430", fn_141a45430 as *const ()), // sub_141a45430
    Replacement::draft(0x141a45490, "unknown_141a45490", fn_141a45490 as *const ()), // sub_141a45490
    Replacement::draft(0x141a45520, "unknown_141a45520", fn_141a45520 as *const ()), // sub_141a45520
    Replacement::draft(0x141a45560, "unknown_141a45560", fn_141a45560 as *const ()), // sub_141a45560
    Replacement::draft(0x141a45580, "unknown_141a45580", fn_141a45580 as *const ()), // sub_141a45580
    Replacement::draft(0x141a455a0, "unknown_141a455a0", fn_141a455a0 as *const ()), // sub_141a455a0
    Replacement::draft(0x141a45710, "unknown_141a45710", fn_141a45710 as *const ()), // sub_141a45710
    Replacement::draft(0x141a45740, "unknown_141a45740", fn_141a45740 as *const ()), // sub_141a45740
    Replacement::draft(0x141a459a0, "unknown_141a459a0", fn_141a459a0 as *const ()), // sub_141a459a0
    Replacement::draft(0x141a46870, "unknown_141a46870", fn_141a46870 as *const ()), // sub_141a46870
    Replacement::draft(0x141a46a10, "unknown_141a46a10", fn_141a46a10 as *const ()), // sub_141a46a10
    Replacement::draft(0x141a46a40, "unknown_141a46a40", fn_141a46a40 as *const ()), // sub_141a46a40
    Replacement::draft(0x141a46c50, "unknown_141a46c50", fn_141a46c50 as *const ()), // sub_141a46c50
    Replacement::draft(0x141a47310, "unknown_141a47310", fn_141a47310 as *const ()), // sub_141a47310
    Replacement::draft(0x141a47f60, "unknown_141a47f60", fn_141a47f60 as *const ()), // sub_141a47f60
    Replacement::draft(0x141a47ff0, "unknown_141a47ff0", fn_141a47ff0 as *const ()), // sub_141a47ff0
    Replacement::draft(0x141a484c0, "unknown_141a484c0", fn_141a484c0 as *const ()), // sub_141a484c0
    Replacement::draft(0x141a485c0, "unknown_141a485c0", fn_141a485c0 as *const ()), // sub_141a485c0
    Replacement::draft(0x141a48720, "unknown_141a48720", fn_141a48720 as *const ()), // sub_141a48720
    Replacement::draft(0x141a48820, "unknown_141a48820", fn_141a48820 as *const ()), // sub_141a48820
    Replacement::draft(0x141a488b0, "unknown_141a488b0", fn_141a488b0 as *const ()), // sub_141a488b0
    Replacement::draft(0x141a48910, "unknown_141a48910", fn_141a48910 as *const ()), // sub_141a48910
    Replacement::draft(0x141a48940, "unknown_141a48940", fn_141a48940 as *const ()), // sub_141a48940
    Replacement::draft(0x141a48a00, "unknown_141a48a00", fn_141a48a00 as *const ()), // sub_141a48a00
    Replacement::draft(0x141a48bf0, "unknown_141a48bf0", fn_141a48bf0 as *const ()), // sub_141a48bf0
    Replacement::draft(0x141a48c10, "unknown_141a48c10", fn_141a48c10 as *const ()), // sub_141a48c10
    Replacement::draft_throws(0x141a48c30, "unknown_141a48c30", fn_141a48c30 as *const (), Throws::Discard), // sub_141a48c30
    Replacement::draft(0x141a49880, "unknown_141a49880", fn_141a49880 as *const ()), // sub_141a49880
    Replacement::draft(0x141a49940, "unknown_141a49940", fn_141a49940 as *const ()), // sub_141a49940
    Replacement::draft(0x141a49990, "unknown_141a49990", tpl_35322 as *const ()), // sub_141a49990
    Replacement::draft(0x141a499d0, "unknown_141a499d0", tpl_34994::<0x143ca7324, 0x14387b4e0, 0x14387b4d8, 0x141834a78, 0x1425ed370, 0x141834d18, 0x141834a0c> as *const ()), // sub_141a499d0
    Replacement::draft(0x141a49a40, "unknown_141a49a40", tpl_35323 as *const ()), // sub_141a49a40
    Replacement::draft(0x141a49a60, "unknown_141a49a60", tpl_34994::<0x143ca72c0, 0x14387b4b0, 0x14387b4a8, 0x141834a78, 0x1425ed380, 0x141834d18, 0x141834a0c> as *const ()), // sub_141a49a60
    Replacement::draft(0x141a49ad0, "unknown_141a49ad0", tpl_35252 as *const ()), // sub_141a49ad0
    Replacement::draft(0x141a49af0, "unknown_141a49af0", tpl_34994::<0x143ca746c, 0x14387b4c0, 0x14387b4b8, 0x141834a78, 0x1425ed360, 0x141834d18, 0x141834a0c> as *const ()), // sub_141a49af0
    Replacement::draft(0x141a49b60, "unknown_141a49b60", tpl_35252 as *const ()), // sub_141a49b60
    Replacement::draft(0x141a49b80, "unknown_141a49b80", tpl_34994::<0x143ca72bc, 0x14387b4d0, 0x14387b4c8, 0x141834a78, 0x1425ed350, 0x141834d18, 0x141834a0c> as *const ()), // sub_141a49b80
    Replacement::draft(0x141a49bf0, "unknown_141a49bf0", tpl_94790 as *const ()), // sub_141a49bf0
    Replacement::draft(0x141a49c80, "unknown_141a49c80", tpl_35012 as *const ()), // sub_141a49c80
    Replacement::draft(0x141a49ca0, "unknown_141a49ca0", fn_141a49ca0 as *const ()), // sub_141a49ca0
    Replacement::draft(0x141a49d10, "unknown_141a49d10", tpl_34318 as *const ()), // sub_141a49d10
    Replacement::draft(0x141a49d30, "unknown_141a49d30", tpl_34318 as *const ()), // sub_141a49d30
    Replacement::draft(0x141a49d50, "unknown_141a49d50", tpl_34318 as *const ()), // sub_141a49d50
    Replacement::draft(0x141a49d70, "unknown_141a49d70", tpl_34318 as *const ()), // sub_141a49d70
    Replacement::draft(0x141a49d90, "unknown_141a49d90", tpl_34318 as *const ()), // sub_141a49d90
    Replacement::draft(0x141a49db0, "unknown_141a49db0", tpl_34318 as *const ()), // sub_141a49db0
    Replacement::draft(0x141a49dd0, "unknown_141a49dd0", tpl_34318 as *const ()), // sub_141a49dd0
    Replacement::draft(0x141a49df0, "unknown_141a49df0", tpl_34318 as *const ()), // sub_141a49df0
    Replacement::draft(0x141a49e10, "unknown_141a49e10", tpl_34318 as *const ()), // sub_141a49e10
    Replacement::draft(0x141a49e30, "unknown_141a49e30", tpl_34318 as *const ()), // sub_141a49e30
    Replacement::draft(0x141a49e50, "unknown_141a49e50", tpl_34319::<0x143c8f6e4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a49e50
    Replacement::draft(0x141a49eb0, "unknown_141a49eb0", tpl_34319::<0x143c9f438, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a49eb0
    Replacement::draft(0x141a49f10, "unknown_141a49f10", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a49f10
    Replacement::draft(0x141a49f70, "unknown_141a49f70", tpl_34319::<0x143c97a2c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a49f70
    Replacement::draft(0x141a49fd0, "unknown_141a49fd0", tpl_34319::<0x143c8e68c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a49fd0
    Replacement::draft(0x141a4a030, "unknown_141a4a030", tpl_34319::<0x143ca7420, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a030
    Replacement::draft(0x141a4a090, "unknown_141a4a090", tpl_34319::<0x143c91f1c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a090
    Replacement::draft(0x141a4a0f0, "unknown_141a4a0f0", tpl_34319::<0x143c97a5c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a0f0
    Replacement::draft(0x141a4a150, "unknown_141a4a150", tpl_34319::<0x143ca72cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a150
    Replacement::draft(0x141a4a1b0, "unknown_141a4a1b0", tpl_34319::<0x143c97a60, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a1b0
    Replacement::draft(0x141a4a210, "std_141a4a210", tpl_550::<0x143159c40, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::moveController<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a230, "std_141a4a230", tpl_550::<0x143159c40, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::moveController<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a250, "std_141a4a250", fn_141a4a250 as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::moveController<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a270, "std_141a4a270", tpl_1::<0x14388adc0> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::moveController<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a280, "std_141a4a280", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::moveController<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a290, "std_141a4a290", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::moveController<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a2b0, "std_141a4a2b0", tpl_20903::<0x143159c08, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::applyToPhysics<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a2e0, "std_141a4a2e0", tpl_20903::<0x143159c08, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::applyToPhysics<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a310, "std_141a4a310", fn_141a4a310 as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::applyToPhysics<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a3d0, "std_141a4a3d0", tpl_1::<0x14388b670> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::applyToPhysics<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a3e0, "std_141a4a3e0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::applyToPhysics<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a3f0, "std_141a4a3f0", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::charactercontroller::applyToPhysics<anonymous_namespace::GameCharacterCollisionsConte
    Replacement::draft(0x141a4a410, "unknown_141a4a410", tpl_34318 as *const ()), // sub_141a4a410
    Replacement::draft(0x141a4a430, "unknown_141a4a430", tpl_361 as *const ()), // sub_141a4a430
    Replacement::draft(0x141a4a440, "unknown_141a4a440", tpl_34318 as *const ()), // sub_141a4a440
    Replacement::draft(0x141a4a460, "unknown_141a4a460", tpl_34319::<0x143ca72c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a460
    Replacement::draft(0x141a4a4c0, "unknown_141a4a4c0", tpl_35063::<0x143c92bc8, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141a4a4c0
    Replacement::draft(0x141a4a510, "unknown_141a4a510", tpl_34319::<0x143c91f48, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a510
    Replacement::draft(0x141a4a570, "unknown_141a4a570", tpl_34318 as *const ()), // sub_141a4a570
    Replacement::draft(0x141a4a590, "unknown_141a4a590", tpl_34319::<0x143c914d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a590
    Replacement::draft(0x141a4a5f0, "unknown_141a4a5f0", tpl_34318 as *const ()), // sub_141a4a5f0
    Replacement::draft(0x141a4a610, "unknown_141a4a610", tpl_34318 as *const ()), // sub_141a4a610
    Replacement::draft(0x141a4a630, "unknown_141a4a630", tpl_34318 as *const ()), // sub_141a4a630
    Replacement::draft(0x141a4a650, "unknown_141a4a650", tpl_34318 as *const ()), // sub_141a4a650
    Replacement::draft(0x141a4a670, "unknown_141a4a670", tpl_34318 as *const ()), // sub_141a4a670
    Replacement::draft(0x141a4a690, "unknown_141a4a690", tpl_34318 as *const ()), // sub_141a4a690
    Replacement::draft(0x141a4a6b0, "unknown_141a4a6b0", tpl_34319::<0x143c8d074, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a6b0
    Replacement::draft(0x141a4a710, "unknown_141a4a710", tpl_34319::<0x143ca71a4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a710
    Replacement::draft(0x141a4a770, "unknown_141a4a770", tpl_34319::<0x143c917cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a770
    Replacement::draft(0x141a4a7d0, "unknown_141a4a7d0", tpl_34319::<0x143c8fa04, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a7d0
    Replacement::draft(0x141a4a830, "unknown_141a4a830", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a830
    Replacement::draft(0x141a4a890, "unknown_141a4a890", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a4a890
    Replacement::draft(0x141a4a8f0, "unknown_141a4a8f0", tpl_34998::<0x8, 0x10> as *const ()), // sub_141a4a8f0
    Replacement::draft(0x141a4a910, "unknown_141a4a910", fn_141a4a910 as *const ()), // sub_141a4a910
    Replacement::draft(0x141a4a9c0, "unknown_141a4a9c0", tpl_34443 as *const ()), // sub_141a4a9c0
    Replacement::draft(0x141a4a9d0, "unknown_141a4a9d0", fn_141a4a9d0 as *const ()), // sub_141a4a9d0
    Replacement::draft(0x141a4acb0, "unknown_141a4acb0", tpl_36390::<0x18, 0x8, 0x10> as *const ()), // sub_141a4acb0
    Replacement::draft(0x141a4acd0, "unknown_141a4acd0", tpl_361 as *const ()), // sub_141a4acd0
    Replacement::draft(0x141a4ace0, "unknown_141a4ace0", fn_141a4ace0 as *const ()), // sub_141a4ace0
    Replacement::draft(0x141a4ad30, "unknown_141a4ad30", tpl_361 as *const ()), // sub_141a4ad30
    Replacement::draft(0x141a4ad40, "unknown_141a4ad40", fn_141a4ad40 as *const ()), // sub_141a4ad40
    Replacement::draft(0x141a4af00, "unknown_141a4af00", tpl_361 as *const ()), // sub_141a4af00
    Replacement::draft(0x141a4af10, "unknown_141a4af10", fn_141a4af10 as *const ()), // sub_141a4af10
    Replacement::draft(0x141a4b100, "unknown_141a4b100", tpl_361 as *const ()), // sub_141a4b100
    Replacement::draft_throws(0x141a4b110, "unknown_141a4b110", fn_141a4b110 as *const (), Throws::Discard), // sub_141a4b110
    Replacement::draft(0x141a4b2a0, "unknown_141a4b2a0", fn_141a4b2a0 as *const ()), // sub_141a4b2a0
    Replacement::draft_throws(0x141a4b3b0, "unknown_141a4b3b0", fn_141a4b3b0 as *const (), Throws::Caught), // sub_141a4b3b0
    Replacement::draft(0x141a4b9a0, "unknown_141a4b9a0", fn_141a4b9a0 as *const ()), // sub_141a4b9a0
    Replacement::draft(0x141a4ba90, "unknown_141a4ba90", tpl_34994::<0x143ca75ec, 0x14387b500, 0x14387b4f8, 0x141834a78, 0x1425ed5f0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141a4ba90
    Replacement::draft(0x141a4bb00, "unknown_141a4bb00", fn_141a4bb00 as *const ()), // sub_141a4bb00
    Replacement::draft(0x141a4bbf0, "unknown_141a4bbf0", fn_141a4bbf0 as *const ()), // sub_141a4bbf0
    Replacement::draft(0x141a4bcf0, "unknown_141a4bcf0", fn_141a4bcf0 as *const ()), // sub_141a4bcf0
    Replacement::draft(0x141a4bf30, "unknown_141a4bf30", tpl_94835::<0x14315c170, 0x8, 0x10, 0x14, 0x18> as *const ()), // sub_141a4bf30
    Replacement::draft(0x141a4c010, "unknown_141a4c010", tpl_35059::<0x14315c170, 0x8, 0x10, 0x14, 0x18> as *const ()), // sub_141a4c010
    Replacement::draft(0x141a4c110, "unknown_141a4c110", tpl_94790 as *const ()), // sub_141a4c110
    Replacement::draft(0x141a4c1c0, "unknown_141a4c1c0", tpl_36739::<0x1404bde30> as *const ()), // sub_141a4c1c0
    Replacement::draft(0x141a4c270, "unknown_141a4c270", tpl_36740::<0x14025f9d0, 0x1401e5730> as *const ()), // sub_141a4c270
    Replacement::draft(0x141a4c410, "unknown_141a4c410", tpl_96614::<0x8, 0x10, 0x1401e5380, 0x1401e66f0> as *const ()), // sub_141a4c410
    Replacement::draft(0x141a4c590, "unknown_141a4c590", fn_141a4c590 as *const ()), // sub_141a4c590
    Replacement::draft(0x141a4c7d0, "unknown_141a4c7d0", fn_141a4c7d0 as *const ()), // sub_141a4c7d0
    Replacement::draft(0x141a4c8a0, "unknown_141a4c8a0", tpl_95486::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_141a4c8a0
    Replacement::draft(0x141a4c980, "unknown_141a4c980", tpl_35718::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_141a4c980
    Replacement::draft(0x141a4c9e0, "unknown_141a4c9e0", tpl_35719::<0x10, 0x1401de830> as *const ()), // sub_141a4c9e0
    Replacement::draft(0x141a4caa0, "unknown_141a4caa0", tpl_34671::<0x20> as *const ()), // sub_141a4caa0
    Replacement::draft(0x141a4cb40, "unknown_141a4cb40", tpl_36740::<0x1404bde90, 0x1401e5730> as *const ()), // sub_141a4cb40
    Replacement::draft(0x141a4cce0, "unknown_141a4cce0", tpl_96614::<0x8, 0x10, 0x1401e5380, 0x1401e66f0> as *const ()), // sub_141a4cce0
    Replacement::draft(0x141a4cdd0, "unknown_141a4cdd0", tpl_34394::<0x10> as *const ()), // sub_141a4cdd0
    Replacement::draft(0x141a4ce70, "unknown_141a4ce70", tpl_94790 as *const ()), // sub_141a4ce70
    Replacement::draft(0x141a4cf20, "unknown_141a4cf20", tpl_36742::<0x24, 0x28, 0x14241d6f0> as *const ()), // sub_141a4cf20
    Replacement::draft(0x141a4d000, "unknown_141a4d000", tpl_34611::<0x8> as *const ()), // sub_141a4d000
    Replacement::draft(0x141a4d090, "unknown_141a4d090", fn_141a4d090 as *const ()), // sub_141a4d090
    Replacement::draft(0x141a4d120, "unknown_141a4d120", fn_141a4d120 as *const ()), // sub_141a4d120
    Replacement::draft(0x141a4e1a0, "unknown_141a4e1a0", fn_141a4e1a0 as *const ()), // sub_141a4e1a0
    Replacement::draft(0x141a4e380, "unknown_141a4e380", fn_141a4e380 as *const ()), // sub_141a4e380
    Replacement::draft(0x141a4e450, "unknown_141a4e450", fn_141a4e450 as *const ()), // sub_141a4e450
    Replacement::draft(0x141a4e4b0, "unknown_141a4e4b0", tpl_95132::<0x8, 0x14240ab10> as *const ()), // sub_141a4e4b0
    Replacement::draft(0x141a4ebc0, "unknown_141a4ebc0", fn_141a4ebc0 as *const ()), // sub_141a4ebc0
    Replacement::draft(0x141a4ed60, "unknown_141a4ed60", fn_141a4ed60 as *const ()), // sub_141a4ed60
    Replacement::draft(0x141a4eeb0, "unknown_141a4eeb0", fn_141a4eeb0 as *const ()), // sub_141a4eeb0
    Replacement::draft(0x141a4f2c0, "unknown_141a4f2c0", fn_141a4f2c0 as *const ()), // sub_141a4f2c0
    Replacement::draft(0x141a4f3a0, "unknown_141a4f3a0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c98fc0, 0x158, 0x14020e8f0> as *const ()), // sub_141a4f3a0
    Replacement::draft_throws(0x141a4f4a0, "unknown_141a4f4a0", fn_141a4f4a0 as *const (), Throws::Caught), // sub_141a4f4a0
    Replacement::draft_throws(0x141a4f740, "unknown_141a4f740", fn_141a4f740 as *const (), Throws::Caught), // sub_141a4f740
    Replacement::draft_throws(0x141a4f860, "unknown_141a4f860", fn_141a4f860 as *const (), Throws::Discard), // sub_141a4f860
    Replacement::draft(0x141a4fd10, "unknown_141a4fd10", fn_141a4fd10 as *const ()), // sub_141a4fd10
    Replacement::draft(0x141a4ff10, "unknown_141a4ff10", fn_141a4ff10 as *const ()), // sub_141a4ff10
    Replacement::draft(0x141a50190, "unknown_141a50190", fn_141a50190 as *const ()), // sub_141a50190
    Replacement::draft(0x141a50270, "unknown_141a50270", fn_141a50270 as *const ()), // sub_141a50270
    Replacement::draft(0x141a508c0, "unknown_141a508c0", fn_141a508c0 as *const ()), // sub_141a508c0
    Replacement::draft_throws(0x141a509f0, "unknown_141a509f0", fn_141a509f0 as *const (), Throws::Discard), // sub_141a509f0
    Replacement::draft(0x141a50de0, "unknown_141a50de0", fn_141a50de0 as *const ()), // sub_141a50de0
    Replacement::draft(0x141a50f20, "unknown_141a50f20", fn_141a50f20 as *const ()), // sub_141a50f20
    Replacement::draft(0x141a50fd0, "unknown_141a50fd0", fn_141a50fd0 as *const ()), // sub_141a50fd0
    Replacement::draft(0x141a51150, "unknown_141a51150", fn_141a51150 as *const ()), // sub_141a51150
    Replacement::draft(0x141a51310, "unknown_141a51310", tpl_35031::<0x142407c80, 0x143ca8328, 0x14020e8f0> as *const ()), // sub_141a51310
    Replacement::draft(0x141a51440, "unknown_141a51440", fn_141a51440 as *const ()), // sub_141a51440
    Replacement::draft(0x141a515f0, "unknown_141a515f0", fn_141a515f0 as *const ()), // sub_141a515f0
    Replacement::draft(0x141a516d0, "unknown_141a516d0", fn_141a516d0 as *const ()), // sub_141a516d0
    Replacement::draft_throws(0x141a51a10, "unknown_141a51a10", fn_141a51a10 as *const (), Throws::Discard), // sub_141a51a10
    Replacement::draft(0x141a51e90, "unknown_141a51e90", fn_141a51e90 as *const ()), // sub_141a51e90
    Replacement::draft(0x141a51ec0, "unknown_141a51ec0", fn_141a51ec0 as *const ()), // sub_141a51ec0
    Replacement::draft(0x141a51fb0, "unknown_141a51fb0", fn_141a51fb0 as *const ()), // sub_141a51fb0
    Replacement::draft(0x141a52370, "unknown_141a52370", fn_141a52370 as *const ()), // sub_141a52370
    Replacement::draft(0x141a523d0, "unknown_141a523d0", fn_141a523d0 as *const ()), // sub_141a523d0
    Replacement::draft(0x141a52480, "unknown_141a52480", fn_141a52480 as *const ()), // sub_141a52480
    Replacement::draft(0x141a52610, "unknown_141a52610", tpl_35748::<0x1404ce2f0> as *const ()), // sub_141a52610
    Replacement::draft(0x141a52630, "unknown_141a52630", tpl_36768 as *const ()), // sub_141a52630
    Replacement::draft(0x141a52670, "unknown_141a52670", tpl_34994::<0x143ca88c0, 0x14387b630, 0x14387b628, 0x141834a78, 0x1425ee730, 0x141834d18, 0x141834a0c> as *const ()), // sub_141a52670
    Replacement::draft(0x141a526e0, "unknown_141a526e0", fn_141a526e0 as *const ()), // sub_141a526e0
    Replacement::draft(0x141a527b0, "unknown_141a527b0", fn_141a527b0 as *const ()), // sub_141a527b0
    Replacement::draft(0x141a52800, "unknown_141a52800", fn_141a52800 as *const ()), // sub_141a52800
    Replacement::draft(0x141a52840, "unknown_141a52840", tpl_34994::<0x143ca856c, 0x14387b620, 0x14387b618, 0x141834a78, 0x1425ee740, 0x141834d18, 0x141834a0c> as *const ()), // sub_141a52840
    Replacement::draft(0x141a528b0, "unknown_141a528b0", tpl_34613 as *const ()), // sub_141a528b0
    Replacement::draft(0x141a528d0, "unknown_141a528d0", tpl_34994::<0x143ca8418, 0x14387b640, 0x14387b638, 0x141834a78, 0x1425ee750, 0x141834d18, 0x141834a0c> as *const ()), // sub_141a528d0
    Replacement::draft(0x141a52940, "unknown_141a52940", tpl_34612 as *const ()), // sub_141a52940
    Replacement::draft(0x141a52960, "unknown_141a52960", tpl_34318 as *const ()), // sub_141a52960
    Replacement::draft(0x141a52980, "unknown_141a52980", tpl_34318 as *const ()), // sub_141a52980
    Replacement::draft(0x141a529a0, "unknown_141a529a0", tpl_34318 as *const ()), // sub_141a529a0
    Replacement::draft(0x141a529c0, "unknown_141a529c0", tpl_34319::<0x143c8e840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a529c0
    Replacement::draft(0x141a52a20, "unknown_141a52a20", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52a20
    Replacement::draft(0x141a52a80, "unknown_141a52a80", tpl_34319::<0x143c8e658, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52a80
    Replacement::draft(0x141a52ae0, "unknown_141a52ae0", tpl_34318 as *const ()), // sub_141a52ae0
    Replacement::draft(0x141a52b00, "unknown_141a52b00", tpl_34319::<0x143c9d198, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52b00
    Replacement::draft(0x141a52b60, "std_141a52b60", tpl_20906::<0x1431638e0, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<bigfish::flashlight::targetmodule::find_targets::system::<lambda_1>,bool,const coregame::Physic
    Replacement::draft(0x141a52b90, "std_141a52b90", tpl_20906::<0x1431638e0, 0x8, 0x28> as *const ()), // std::_Func_impl_no_alloc<bigfish::flashlight::targetmodule::find_targets::system::<lambda_1>,bool,const coregame::Physic
    Replacement::draft(0x141a52bd0, "std_141a52bd0", tpl_1::<0x14388bab0> as *const ()), // std::_Func_impl_no_alloc<bigfish::flashlight::targetmodule::find_targets::system::<lambda_1>,bool,const coregame::Physic
    Replacement::draft(0x141a52be0, "std_141a52be0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::flashlight::targetmodule::find_targets::system::<lambda_1>,bool,const coregame::Physic
    Replacement::draft(0x141a52bf0, "std_141a52bf0", tpl_20904::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::flashlight::targetmodule::find_targets::system::<lambda_1>,bool,const coregame::Physic
    Replacement::draft(0x141a52c10, "unknown_141a52c10", tpl_34318 as *const ()), // sub_141a52c10
    Replacement::draft(0x141a52c30, "unknown_141a52c30", tpl_34318 as *const ()), // sub_141a52c30
    Replacement::draft(0x141a52c50, "unknown_141a52c50", tpl_34318 as *const ()), // sub_141a52c50
    Replacement::draft(0x141a52c70, "unknown_141a52c70", tpl_34318 as *const ()), // sub_141a52c70
    Replacement::draft(0x141a52c90, "unknown_141a52c90", tpl_34318 as *const ()), // sub_141a52c90
    Replacement::draft(0x141a52cb0, "unknown_141a52cb0", tpl_34319::<0x143c8e65c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52cb0
    Replacement::draft(0x141a52d10, "unknown_141a52d10", tpl_34319::<0x143ca02c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52d10
    Replacement::draft(0x141a52d70, "unknown_141a52d70", tpl_34319::<0x143c9fde0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52d70
    Replacement::draft(0x141a52dd0, "unknown_141a52dd0", tpl_34319::<0x143c914b8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52dd0
    Replacement::draft(0x141a52e30, "unknown_141a52e30", tpl_34319::<0x143c914bc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52e30
    Replacement::draft(0x141a52e90, "unknown_141a52e90", tpl_34318 as *const ()), // sub_141a52e90
    Replacement::draft(0x141a52eb0, "unknown_141a52eb0", tpl_34318 as *const ()), // sub_141a52eb0
    Replacement::draft(0x141a52ed0, "unknown_141a52ed0", tpl_34318 as *const ()), // sub_141a52ed0
    Replacement::draft(0x141a52ef0, "unknown_141a52ef0", tpl_34318 as *const ()), // sub_141a52ef0
    Replacement::draft(0x141a52f10, "unknown_141a52f10", tpl_34318 as *const ()), // sub_141a52f10
    Replacement::draft(0x141a52f30, "unknown_141a52f30", tpl_34318 as *const ()), // sub_141a52f30
    Replacement::draft(0x141a52f50, "unknown_141a52f50", tpl_34318 as *const ()), // sub_141a52f50
    Replacement::draft(0x141a52f70, "unknown_141a52f70", tpl_34319::<0x143ca88bc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52f70
    Replacement::draft(0x141a52fd0, "unknown_141a52fd0", tpl_34319::<0x143c98134, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a52fd0
    Replacement::draft(0x141a53030, "unknown_141a53030", tpl_34319::<0x143ca87e8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a53030
    Replacement::draft(0x141a53090, "unknown_141a53090", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a53090
    Replacement::draft(0x141a530f0, "unknown_141a530f0", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a530f0
    Replacement::draft(0x141a53150, "unknown_141a53150", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a53150
    Replacement::draft(0x141a531b0, "unknown_141a531b0", tpl_34319::<0x143ca5918, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a531b0
    Replacement::draft(0x141a53210, "unknown_141a53210", tpl_34318 as *const ()), // sub_141a53210
    Replacement::draft(0x141a53230, "unknown_141a53230", tpl_34319::<0x143c8fab0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a53230
    Replacement::draft(0x141a53290, "unknown_141a53290", tpl_34318 as *const ()), // sub_141a53290
    Replacement::draft(0x141a532b0, "unknown_141a532b0", tpl_34319::<0x143ca88c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a532b0
    Replacement::draft(0x141a53310, "unknown_141a53310", tpl_34318 as *const ()), // sub_141a53310
    Replacement::draft(0x141a53330, "unknown_141a53330", tpl_34318 as *const ()), // sub_141a53330
    Replacement::draft(0x141a53350, "unknown_141a53350", tpl_34318 as *const ()), // sub_141a53350
    Replacement::draft(0x141a53370, "unknown_141a53370", tpl_34319::<0x143c917cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a53370
    Replacement::draft(0x141a533d0, "unknown_141a533d0", tpl_34319::<0x143ca8568, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a533d0
    Replacement::draft(0x141a53430, "unknown_141a53430", tpl_34319::<0x143c8ec68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a53430
    Replacement::draft(0x141a53490, "unknown_141a53490", fn_141a53490 as *const ()), // sub_141a53490
    Replacement::draft(0x141a534d0, "unknown_141a534d0", tpl_361 as *const ()), // sub_141a534d0
    Replacement::draft(0x141a534e0, "unknown_141a534e0", tpl_361 as *const ()), // sub_141a534e0
    Replacement::draft(0x141a534f0, "unknown_141a534f0", tpl_361 as *const ()), // sub_141a534f0
    Replacement::draft(0x141a53500, "unknown_141a53500", fn_141a53500 as *const ()), // sub_141a53500
    Replacement::draft(0x141a53930, "unknown_141a53930", tpl_361 as *const ()), // sub_141a53930
    Replacement::draft(0x141a53940, "unknown_141a53940", tpl_361 as *const ()), // sub_141a53940
    Replacement::draft(0x141a53950, "unknown_141a53950", tpl_496 as *const ()), // sub_141a53950
    Replacement::draft(0x141a53960, "unknown_141a53960", tpl_361 as *const ()), // sub_141a53960
    Replacement::draft(0x141a53970, "unknown_141a53970", fn_141a53970 as *const ()), // sub_141a53970
    Replacement::draft(0x141a53e40, "unknown_141a53e40", tpl_361 as *const ()), // sub_141a53e40
    Replacement::draft(0x141a53e50, "unknown_141a53e50", tpl_496 as *const ()), // sub_141a53e50
    Replacement::draft(0x141a53e90, "unknown_141a53e90", fn_141a53e90 as *const ()), // sub_141a53e90
    Replacement::draft_throws(0x141a54020, "unknown_141a54020", fn_141a54020 as *const (), Throws::Caught), // sub_141a54020
    Replacement::draft(0x141a54f00, "unknown_141a54f00", fn_141a54f00 as *const ()), // sub_141a54f00
    Replacement::draft_throws(0x141a55840, "unknown_141a55840", fn_141a55840 as *const (), Throws::Discard), // sub_141a55840
    Replacement::draft(0x141a55d70, "unknown_141a55d70", fn_141a55d70 as *const ()), // sub_141a55d70
    Replacement::draft(0x141a55e30, "unknown_141a55e30", fn_141a55e30 as *const ()), // sub_141a55e30
    Replacement::draft(0x141a55ef0, "unknown_141a55ef0", fn_141a55ef0 as *const ()), // sub_141a55ef0
    Replacement::draft(0x141a55f80, "unknown_141a55f80", fn_141a55f80 as *const ()), // sub_141a55f80
    Replacement::draft(0x141a56000, "unknown_141a56000", fn_141a56000 as *const ()), // sub_141a56000
    Replacement::draft_throws(0x141a560c0, "unknown_141a560c0", tpl_35009::<0x14387ed00, 0x1404ceb80, 0x1402116e0> as *const (), Throws::Discard), // sub_141a560c0
    Replacement::draft(0x141a56170, "unknown_141a56170", tpl_34613 as *const ()), // sub_141a56170
    Replacement::draft(0x141a56200, "unknown_141a56200", tpl_34605::<0x1431667e0> as *const ()), // sub_141a56200
    Replacement::draft(0x141a56340, "unknown_141a56340", tpl_34613 as *const ()), // sub_141a56340
    Replacement::draft(0x141a56e30, "unknown_141a56e30", fn_141a56e30 as *const ()), // sub_141a56e30
    Replacement::draft(0x141a57130, "unknown_141a57130", fn_141a57130 as *const ()), // sub_141a57130
    Replacement::draft(0x141a575e0, "unknown_141a575e0", fn_141a575e0 as *const ()), // sub_141a575e0
    Replacement::draft(0x141a578e0, "unknown_141a578e0", fn_141a578e0 as *const ()), // sub_141a578e0
    Replacement::draft(0x141a57ab0, "unknown_141a57ab0", fn_141a57ab0 as *const ()), // sub_141a57ab0
    Replacement::draft(0x141a57f30, "unknown_141a57f30", fn_141a57f30 as *const ()), // sub_141a57f30
    Replacement::draft(0x141a58120, "unknown_141a58120", fn_141a58120 as *const ()), // sub_141a58120
    Replacement::draft(0x141a58410, "unknown_141a58410", fn_141a58410 as *const ()), // sub_141a58410
    Replacement::draft(0x141a58650, "unknown_141a58650", fn_141a58650 as *const ()), // sub_141a58650
    Replacement::draft(0x141a58c20, "unknown_141a58c20", fn_141a58c20 as *const ()), // sub_141a58c20
    Replacement::draft(0x141a59160, "unknown_141a59160", fn_141a59160 as *const ()), // sub_141a59160
    Replacement::draft(0x141a59370, "unknown_141a59370", fn_141a59370 as *const ()), // sub_141a59370
    Replacement::draft(0x141a593b0, "unknown_141a593b0", fn_141a593b0 as *const ()), // sub_141a593b0
    Replacement::draft(0x141a59530, "unknown_141a59530", fn_141a59530 as *const ()), // sub_141a59530
    Replacement::draft(0x141a59800, "unknown_141a59800", fn_141a59800 as *const ()), // sub_141a59800
    Replacement::draft(0x141a59860, "unknown_141a59860", fn_141a59860 as *const ()), // sub_141a59860
    Replacement::draft(0x141a59ac0, "unknown_141a59ac0", fn_141a59ac0 as *const ()), // sub_141a59ac0
    Replacement::draft(0x141a59c10, "unknown_141a59c10", tpl_4865::<0x8, 0x1404d1900, 0x1404d1ae0> as *const ()), // sub_141a59c10
    Replacement::draft(0x141a59c90, "unknown_141a59c90", fn_141a59c90 as *const ()), // sub_141a59c90
    Replacement::draft_throws(0x141a59db0, "unknown_141a59db0", fn_141a59db0 as *const (), Throws::Discard), // sub_141a59db0
    Replacement::draft(0x141a5a160, "unknown_141a5a160", tpl_34394::<0x10> as *const ()), // sub_141a5a160
    Replacement::draft(0x141a5a200, "unknown_141a5a200", tpl_94428::<0x14316dec8, 0x8> as *const ()), // sub_141a5a200
    Replacement::draft(0x141a5a400, "unknown_141a5a400", fn_141a5a400 as *const ()), // sub_141a5a400
    Replacement::draft(0x141a5a4d0, "unknown_141a5a4d0", fn_141a5a4d0 as *const ()), // sub_141a5a4d0
    Replacement::draft(0x141a5a540, "unknown_141a5a540", tpl_4887::<0x8, 0x1404da530, 0x1404da730> as *const ()), // sub_141a5a540
    Replacement::draft_throws(0x141a5a5c0, "unknown_141a5a5c0", tpl_36792::<0x14387edb8, 0x1404daa70, 0x1402116e0, 0x142745910, 0x18, 0x1410d6ca0, 0x10, 0x1404da250> as *const (), Throws::Discard), // sub_141a5a5c0
    Replacement::draft(0x141a5a6c0, "unknown_141a5a6c0", tpl_34611::<0x8> as *const ()), // sub_141a5a6c0
    Replacement::draft(0x141a5a960, "unknown_141a5a960", tpl_95133::<0x143174108, 0x1, 0x8, 0x1404e9ab0> as *const ()), // sub_141a5a960
    Replacement::draft(0x141a5ab00, "unknown_141a5ab00", tpl_34605::<0x143174260> as *const ()), // sub_141a5ab00
    Replacement::draft(0x141a5acd0, "unknown_141a5acd0", fn_141a5acd0 as *const ()), // sub_141a5acd0
    Replacement::draft(0x141a5ae50, "unknown_141a5ae50", tpl_36794 as *const ()), // sub_141a5ae50
    Replacement::draft_throws(0x141a5bb00, "unknown_141a5bb00", fn_141a5bb00 as *const (), Throws::Discard), // sub_141a5bb00
    Replacement::draft(0x141a5bc80, "unknown_141a5bc80", fn_141a5bc80 as *const ()), // sub_141a5bc80
    Replacement::draft(0x141a5bf70, "unknown_141a5bf70", fn_141a5bf70 as *const ()), // sub_141a5bf70
    Replacement::draft(0x141a5c300, "unknown_141a5c300", fn_141a5c300 as *const ()), // sub_141a5c300
    Replacement::draft(0x141a5c5a0, "unknown_141a5c5a0", fn_141a5c5a0 as *const ()), // sub_141a5c5a0
    Replacement::draft(0x141a5c880, "unknown_141a5c880", fn_141a5c880 as *const ()), // sub_141a5c880
    Replacement::draft(0x141a5cb70, "unknown_141a5cb70", fn_141a5cb70 as *const ()), // sub_141a5cb70
    Replacement::draft(0x141a5cf30, "unknown_141a5cf30", fn_141a5cf30 as *const ()), // sub_141a5cf30
    Replacement::draft(0x141a5cf70, "unknown_141a5cf70", fn_141a5cf70 as *const ()), // sub_141a5cf70
    Replacement::draft(0x141a5d230, "unknown_141a5d230", fn_141a5d230 as *const ()), // sub_141a5d230
    Replacement::draft(0x141a5d520, "unknown_141a5d520", fn_141a5d520 as *const ()), // sub_141a5d520
    Replacement::draft_throws(0x141a5d940, "unknown_141a5d940", fn_141a5d940 as *const (), Throws::Caught), // sub_141a5d940
    Replacement::draft_throws(0x141a5dc20, "unknown_141a5dc20", fn_141a5dc20 as *const (), Throws::Caught), // sub_141a5dc20
    Replacement::draft(0x141a5dfd0, "unknown_141a5dfd0", fn_141a5dfd0 as *const ()), // sub_141a5dfd0
    Replacement::draft(0x141a5e2c0, "unknown_141a5e2c0", fn_141a5e2c0 as *const ()), // sub_141a5e2c0
    Replacement::draft(0x141a5e490, "unknown_141a5e490", tpl_4903::<0x8, 0x1404df010, 0x1404df210> as *const ()), // sub_141a5e490
    Replacement::draft(0x141a5e570, "unknown_141a5e570", fn_141a5e570 as *const ()), // sub_141a5e570
    Replacement::draft(0x141a5e630, "unknown_141a5e630", tpl_4902::<0x8, 0x1404de3b0, 0x1a, 0x1404de5b0> as *const ()), // sub_141a5e630
    Replacement::draft(0x141a5e6d0, "unknown_141a5e6d0", fn_141a5e6d0 as *const ()), // sub_141a5e6d0
    Replacement::draft(0x141a5e6e0, "unknown_141a5e6e0", fn_141a5e6e0 as *const ()), // sub_141a5e6e0
    Replacement::draft(0x141a5e710, "unknown_141a5e710", fn_141a5e710 as *const ()), // sub_141a5e710
    Replacement::draft(0x141a5e730, "unknown_141a5e730", fn_141a5e730 as *const ()), // sub_141a5e730
    Replacement::draft(0x141a5e8f0, "unknown_141a5e8f0", fn_141a5e8f0 as *const ()), // sub_141a5e8f0
    Replacement::draft(0x141a5e960, "unknown_141a5e960", fn_141a5e960 as *const ()), // sub_141a5e960
    Replacement::draft_throws(0x141a5e9d0, "unknown_141a5e9d0", fn_141a5e9d0 as *const (), Throws::Discard), // sub_141a5e9d0
    Replacement::draft(0x141a5f6a0, "unknown_141a5f6a0", fn_141a5f6a0 as *const ()), // sub_141a5f6a0
    Replacement::draft(0x141a5f810, "unknown_141a5f810", fn_141a5f810 as *const ()), // sub_141a5f810
    Replacement::draft(0x141a5f8a0, "unknown_141a5f8a0", fn_141a5f8a0 as *const ()), // sub_141a5f8a0
    Replacement::draft(0x141a5fa90, "unknown_141a5fa90", fn_141a5fa90 as *const ()), // sub_141a5fa90
    Replacement::draft(0x141a5fb50, "unknown_141a5fb50", fn_141a5fb50 as *const ()), // sub_141a5fb50
    Replacement::draft_throws(0x141a5fb90, "unknown_141a5fb90", tpl_36820::<0x14387eb74, 0x8, 0x1404ea840, 0x1402116e0, 0x142746af0, 0x18, 0x1410d6ca0, 0x10, 0x1404e9c10, 0x142746b00, 0x1404e9ef0> as *const (), Throws::Discard), // sub_141a5fb90
    Replacement::draft(0x141a60270, "unknown_141a60270", fn_141a60270 as *const ()), // sub_141a60270
    Replacement::draft(0x141a60630, "unknown_141a60630", tpl_36822::<0x38> as *const ()), // sub_141a60630
    Replacement::draft(0x141a60650, "unknown_141a60650", tpl_36822::<0x3c> as *const ()), // sub_141a60650
    Replacement::draft(0x141a60670, "unknown_141a60670", fn_141a60670 as *const ()), // sub_141a60670
    Replacement::draft(0x141a60a80, "unknown_141a60a80", fn_141a60a80 as *const ()), // sub_141a60a80
    Replacement::draft(0x141a60c90, "unknown_141a60c90", tpl_36822::<0xc> as *const ()), // sub_141a60c90
    Replacement::draft(0x141a60cb0, "unknown_141a60cb0", tpl_36822::<0x40> as *const ()), // sub_141a60cb0
    Replacement::draft(0x141a60cd0, "unknown_141a60cd0", tpl_36822::<0x28> as *const ()), // sub_141a60cd0
    Replacement::draft(0x141a60cf0, "unknown_141a60cf0", tpl_36822::<0x24> as *const ()), // sub_141a60cf0
    Replacement::draft(0x141a60d10, "unknown_141a60d10", tpl_36822::<0x4> as *const ()), // sub_141a60d10
    Replacement::draft(0x141a60d30, "unknown_141a60d30", fn_141a60d30 as *const ()), // sub_141a60d30
    Replacement::draft(0x141a60dc0, "unknown_141a60dc0", fn_141a60dc0 as *const ()), // sub_141a60dc0
    Replacement::draft(0x141a60df0, "unknown_141a60df0", tpl_36822::<0x30> as *const ()), // sub_141a60df0
    Replacement::draft(0x141a60e10, "unknown_141a60e10", tpl_36822::<0x8> as *const ()), // sub_141a60e10
    Replacement::draft(0x141a60e30, "unknown_141a60e30", tpl_36822::<0x10> as *const ()), // sub_141a60e30
    Replacement::draft(0x141a60e50, "unknown_141a60e50", fn_141a60e50 as *const ()), // sub_141a60e50
    Replacement::draft(0x141a61110, "unknown_141a61110", fn_141a61110 as *const ()), // sub_141a61110
    Replacement::draft_throws(0x141a61310, "unknown_141a61310", tpl_36179::<0x143c9da5c, 0x143caae70, 0x1404ed050, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1404ed3f0, 0x1425371a0, 0x142533200, 0x142747cc0, 0x1410d6ca0, 0x1404ec5b0, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_141a61310
    Replacement::draft(0x141a61640, "unknown_141a61640", tpl_94810::<0x143175b40, 0x8> as *const ()), // sub_141a61640
    Replacement::draft(0x141a61b00, "unknown_141a61b00", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143caafec, 0x158, 0x14020e8f0> as *const ()), // sub_141a61b00
    Replacement::draft(0x141a61c00, "unknown_141a61c00", tpl_35630::<0x10, 0x8, 0x1404edc00> as *const ()), // sub_141a61c00
    Replacement::draft_throws(0x141a61c70, "unknown_141a61c70", fn_141a61c70 as *const (), Throws::Caught), // sub_141a61c70
    Replacement::draft_throws(0x141a62810, "unknown_141a62810", tpl_34795::<0x1427486e0, 0x1410d6ca0, 0x1404ee0c0, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1404ee3a0, 0x143823788, 0x1427486f0, 0x1404ed9a0, 0x1438234b0, 0x14387eea0, 0x1404ee690, 0x1402116e0> as *const (), Throws::Discard), // sub_141a62810
    Replacement::draft(0x141a62dc0, "unknown_141a62dc0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cab35c, 0x158, 0x14020e8f0> as *const ()), // sub_141a62dc0
    Replacement::draft(0x141a62ec0, "unknown_141a62ec0", fn_141a62ec0 as *const ()), // sub_141a62ec0
    Replacement::draft_throws(0x141a62f40, "unknown_141a62f40", fn_141a62f40 as *const (), Throws::Discard), // sub_141a62f40
    Replacement::draft(0x141a63350, "unknown_141a63350", fn_141a63350 as *const ()), // sub_141a63350
    Replacement::draft(0x141a634d0, "unknown_141a634d0", tpl_36794 as *const ()), // sub_141a634d0
    Replacement::draft_throws(0x141a63550, "unknown_141a63550", tpl_35009::<0x14387fd0c, 0x1404f1450, 0x1402116e0> as *const (), Throws::Discard), // sub_141a63550
    Replacement::draft(0x141a63600, "unknown_141a63600", fn_141a63600 as *const ()), // sub_141a63600
    Replacement::draft(0x141a636a0, "unknown_141a636a0", fn_141a636a0 as *const ()), // sub_141a636a0
    Replacement::draft(0x141a636d0, "unknown_141a636d0", tpl_35016 as *const ()), // sub_141a636d0
    Replacement::draft(0x141a63710, "unknown_141a63710", tpl_36828::<0x141834d30> as *const ()), // sub_141a63710
    Replacement::draft(0x141a63730, "unknown_141a63730", tpl_34318 as *const ()), // sub_141a63730
    Replacement::draft(0x141a63750, "unknown_141a63750", tpl_34319::<0x143c8e70c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63750
    Replacement::draft(0x141a637b0, "unknown_141a637b0", tpl_34318 as *const ()), // sub_141a637b0
    Replacement::draft(0x141a637d0, "unknown_141a637d0", tpl_34318 as *const ()), // sub_141a637d0
    Replacement::draft(0x141a637f0, "unknown_141a637f0", tpl_34318 as *const ()), // sub_141a637f0
    Replacement::draft(0x141a63810, "unknown_141a63810", tpl_34318 as *const ()), // sub_141a63810
    Replacement::draft(0x141a63830, "unknown_141a63830", tpl_34318 as *const ()), // sub_141a63830
    Replacement::draft(0x141a63850, "unknown_141a63850", tpl_34319::<0x143c8d708, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63850
    Replacement::draft(0x141a638b0, "unknown_141a638b0", tpl_34319::<0x143c914bc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a638b0
    Replacement::draft(0x141a63910, "unknown_141a63910", tpl_34319::<0x143c95608, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63910
    Replacement::draft(0x141a63970, "unknown_141a63970", tpl_34319::<0x143c94b6c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63970
    Replacement::draft(0x141a639d0, "unknown_141a639d0", tpl_34319::<0x143c964e0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a639d0
    Replacement::draft(0x141a63a30, "unknown_141a63a30", tpl_34318 as *const ()), // sub_141a63a30
    Replacement::draft(0x141a63a50, "unknown_141a63a50", tpl_34318 as *const ()), // sub_141a63a50
    Replacement::draft(0x141a63a70, "unknown_141a63a70", tpl_34319::<0x143c95748, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63a70
    Replacement::draft(0x141a63ad0, "unknown_141a63ad0", tpl_34319::<0x143c8e710, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63ad0
    Replacement::draft(0x141a63b30, "unknown_141a63b30", tpl_34318 as *const ()), // sub_141a63b30
    Replacement::draft(0x141a63b50, "unknown_141a63b50", tpl_34319::<0x143c953f8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63b50
    Replacement::draft(0x141a63bb0, "unknown_141a63bb0", tpl_34318 as *const ()), // sub_141a63bb0
    Replacement::draft(0x141a63bd0, "unknown_141a63bd0", tpl_34318 as *const ()), // sub_141a63bd0
    Replacement::draft(0x141a63bf0, "unknown_141a63bf0", tpl_34318 as *const ()), // sub_141a63bf0
    Replacement::draft(0x141a63c10, "unknown_141a63c10", tpl_34318 as *const ()), // sub_141a63c10
    Replacement::draft(0x141a63c30, "unknown_141a63c30", tpl_34318 as *const ()), // sub_141a63c30
    Replacement::draft(0x141a63c50, "unknown_141a63c50", tpl_34318 as *const ()), // sub_141a63c50
    Replacement::draft(0x141a63c70, "unknown_141a63c70", tpl_34319::<0x143c95844, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63c70
    Replacement::draft(0x141a63cd0, "unknown_141a63cd0", tpl_34319::<0x143c956a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63cd0
    Replacement::draft(0x141a63d30, "unknown_141a63d30", tpl_34319::<0x143c91078, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63d30
    Replacement::draft(0x141a63d90, "unknown_141a63d90", tpl_34319::<0x143c8dbc8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63d90
    Replacement::draft(0x141a63df0, "unknown_141a63df0", tpl_34319::<0x143c94b3c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63df0
    Replacement::draft(0x141a63e50, "unknown_141a63e50", tpl_34319::<0x143c8fe18, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a63e50
    Replacement::draft(0x141a63eb0, "unknown_141a63eb0", tpl_36828::<0x141834d30> as *const ()), // sub_141a63eb0
    Replacement::draft_throws(0x141a63ed0, "std_141a63ed0", tpl_36829::<0x141834d64, 0x14317d970, 0x8, 0x28> as *const (), Throws::Discard), // std::_Func_impl_no_alloc<bigfish::explosion::explosion_targets::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&
    Replacement::draft(0x141a63f10, "std_141a63f10", tpl_559 as *const ()), // std::_Func_impl_no_alloc<bigfish::explosion::explosion_targets::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&
    Replacement::draft(0x141a63f20, "std_141a63f20", tpl_36830::<0x8, 0x28, 0x10> as *const ()), // std::_Func_impl_no_alloc<bigfish::explosion::explosion_targets::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&
    Replacement::draft(0x141a63fa0, "std_141a63fa0", tpl_1::<0x14388c280> as *const ()), // std::_Func_impl_no_alloc<bigfish::explosion::explosion_targets::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&
    Replacement::draft(0x141a63fb0, "std_141a63fb0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::explosion::explosion_targets::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&
    Replacement::draft(0x141a63fc0, "std_141a63fc0", tpl_36831::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::explosion::explosion_targets::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&
    Replacement::draft(0x141a63fe0, "unknown_141a63fe0", tpl_361 as *const ()), // sub_141a63fe0
    Replacement::draft(0x141a63ff0, "unknown_141a63ff0", tpl_34318 as *const ()), // sub_141a63ff0
    Replacement::draft(0x141a64010, "unknown_141a64010", tpl_34318 as *const ()), // sub_141a64010
    Replacement::draft(0x141a64030, "unknown_141a64030", tpl_34318 as *const ()), // sub_141a64030
    Replacement::draft(0x141a64050, "unknown_141a64050", tpl_34318 as *const ()), // sub_141a64050
    Replacement::draft(0x141a64070, "unknown_141a64070", tpl_35063::<0x143c92bc8, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141a64070
    Replacement::draft(0x141a640c0, "unknown_141a640c0", tpl_34319::<0x143c9fde0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a640c0
    Replacement::draft(0x141a64120, "unknown_141a64120", tpl_34319::<0x143c8fab8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a64120
    Replacement::draft(0x141a64180, "unknown_141a64180", tpl_34319::<0x143c8f4cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a64180
    Replacement::draft(0x141a641e0, "unknown_141a641e0", tpl_34319::<0x143c8e654, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a641e0
    Replacement::draft(0x141a64240, "unknown_141a64240", tpl_361 as *const ()), // sub_141a64240
    Replacement::draft(0x141a64250, "unknown_141a64250", tpl_34318 as *const ()), // sub_141a64250
    Replacement::draft(0x141a64270, "unknown_141a64270", tpl_34318 as *const ()), // sub_141a64270
    Replacement::draft(0x141a64290, "unknown_141a64290", tpl_34318 as *const ()), // sub_141a64290
    Replacement::draft(0x141a642b0, "unknown_141a642b0", tpl_34318 as *const ()), // sub_141a642b0
    Replacement::draft(0x141a642d0, "unknown_141a642d0", tpl_34318 as *const ()), // sub_141a642d0
    Replacement::draft(0x141a642f0, "unknown_141a642f0", tpl_34318 as *const ()), // sub_141a642f0
    Replacement::draft(0x141a64310, "unknown_141a64310", tpl_34318 as *const ()), // sub_141a64310
    Replacement::draft(0x141a64330, "unknown_141a64330", tpl_35063::<0x143c8f4f0, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141a64330
    Replacement::draft(0x141a64380, "unknown_141a64380", tpl_34319::<0x143cab498, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a64380
    Replacement::draft(0x141a643e0, "unknown_141a643e0", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a643e0
    Replacement::draft(0x141a64440, "unknown_141a64440", tpl_34319::<0x143c8fa04, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a64440
    Replacement::draft(0x141a644a0, "unknown_141a644a0", tpl_34319::<0x143c8dbd4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a644a0
    Replacement::draft(0x141a64500, "unknown_141a64500", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a64500
    Replacement::draft(0x141a64560, "unknown_141a64560", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a64560
    Replacement::draft(0x141a645c0, "unknown_141a645c0", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a645c0
    Replacement::draft(0x141a64620, "unknown_141a64620", tpl_34318 as *const ()), // sub_141a64620
    Replacement::draft(0x141a64640, "unknown_141a64640", tpl_34318 as *const ()), // sub_141a64640
    Replacement::draft(0x141a64660, "unknown_141a64660", tpl_34318 as *const ()), // sub_141a64660
    Replacement::draft(0x141a64680, "unknown_141a64680", tpl_34319::<0x143c96b20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a64680
    Replacement::draft(0x141a646e0, "unknown_141a646e0", tpl_34319::<0x143c96b14, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a646e0
    Replacement::draft(0x141a64740, "unknown_141a64740", tpl_34319::<0x143c8ec68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a64740
    Replacement::draft(0x141a647a0, "unknown_141a647a0", fn_141a647a0 as *const ()), // sub_141a647a0
    Replacement::draft(0x141a64930, "unknown_141a64930", tpl_361 as *const ()), // sub_141a64930
    Replacement::draft(0x141a64940, "unknown_141a64940", fn_141a64940 as *const ()), // sub_141a64940
    Replacement::draft(0x141a64eb0, "unknown_141a64eb0", tpl_361 as *const ()), // sub_141a64eb0
    Replacement::draft(0x141a64ec0, "unknown_141a64ec0", fn_141a64ec0 as *const ()), // sub_141a64ec0
    Replacement::draft(0x141a65fb0, "unknown_141a65fb0", fn_141a65fb0 as *const ()), // sub_141a65fb0
    Replacement::draft(0x141a66040, "unknown_141a66040", tpl_361 as *const ()), // sub_141a66040
    Replacement::draft(0x141a66050, "unknown_141a66050", fn_141a66050 as *const ()), // sub_141a66050
    Replacement::draft(0x141a66100, "unknown_141a66100", tpl_361 as *const ()), // sub_141a66100
    Replacement::draft(0x141a66130, "unknown_141a66130", tpl_36830::<0x8, 0x20, 0x10> as *const ()), // sub_141a66130
    Replacement::draft(0x141a661b0, "unknown_141a661b0", tpl_361 as *const ()), // sub_141a661b0
    Replacement::draft_throws(0x141a661c0, "unknown_141a661c0", fn_141a661c0 as *const (), Throws::Caught), // sub_141a661c0
    Replacement::draft(0x141a66650, "unknown_141a66650", tpl_361 as *const ()), // sub_141a66650
    Replacement::draft(0x141a66660, "unknown_141a66660", fn_141a66660 as *const ()), // sub_141a66660
    Replacement::draft(0x141a66910, "unknown_141a66910", tpl_361 as *const ()), // sub_141a66910
    Replacement::draft(0x141a66920, "unknown_141a66920", fn_141a66920 as *const ()), // sub_141a66920
    Replacement::draft_throws(0x141a66960, "unknown_141a66960", fn_141a66960 as *const (), Throws::Discard), // sub_141a66960
    Replacement::draft(0x141a670e0, "unknown_141a670e0", tpl_361 as *const ()), // sub_141a670e0
    Replacement::draft(0x141a670f0, "unknown_141a670f0", fn_141a670f0 as *const ()), // sub_141a670f0
    Replacement::draft(0x141a67310, "unknown_141a67310", tpl_361 as *const ()), // sub_141a67310
    Replacement::draft(0x141a67320, "unknown_141a67320", fn_141a67320 as *const ()), // sub_141a67320
    Replacement::draft(0x141a67440, "unknown_141a67440", tpl_361 as *const ()), // sub_141a67440
    Replacement::draft(0x141a67450, "unknown_141a67450", fn_141a67450 as *const ()), // sub_141a67450
    Replacement::draft_throws(0x141a67480, "unknown_141a67480", fn_141a67480 as *const (), Throws::Discard), // sub_141a67480
    Replacement::draft(0x141a67dc0, "unknown_141a67dc0", tpl_34318 as *const ()), // sub_141a67dc0
    Replacement::draft(0x141a67de0, "unknown_141a67de0", tpl_34318 as *const ()), // sub_141a67de0
    Replacement::draft(0x141a67e00, "unknown_141a67e00", tpl_34318 as *const ()), // sub_141a67e00
    Replacement::draft(0x141a67e20, "unknown_141a67e20", tpl_34319::<0x143c954e4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a67e20
    Replacement::draft(0x141a67e80, "unknown_141a67e80", tpl_34319::<0x143c8dbd4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a67e80
    Replacement::draft(0x141a67ee0, "unknown_141a67ee0", tpl_34319::<0x143c96d60, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a67ee0
    Replacement::draft(0x141a67f40, "unknown_141a67f40", tpl_34318 as *const ()), // sub_141a67f40
    Replacement::draft(0x141a67f60, "unknown_141a67f60", tpl_34319::<0x143c970e0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a67f60
    Replacement::draft(0x141a67fc0, "unknown_141a67fc0", tpl_34318 as *const ()), // sub_141a67fc0
    Replacement::draft(0x141a67fe0, "unknown_141a67fe0", tpl_34318 as *const ()), // sub_141a67fe0
    Replacement::draft(0x141a68000, "unknown_141a68000", tpl_34318 as *const ()), // sub_141a68000
    Replacement::draft(0x141a68020, "unknown_141a68020", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a68020
    Replacement::draft(0x141a68080, "unknown_141a68080", tpl_34319::<0x143c95048, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a68080
    Replacement::draft(0x141a680e0, "unknown_141a680e0", tpl_34319::<0x143c95468, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a680e0
    Replacement::draft(0x141a68140, "unknown_141a68140", tpl_34318 as *const ()), // sub_141a68140
    Replacement::draft(0x141a68160, "unknown_141a68160", tpl_34318 as *const ()), // sub_141a68160
    Replacement::draft(0x141a68180, "unknown_141a68180", tpl_34319::<0x143c8f4cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a68180
    Replacement::draft(0x141a681e0, "unknown_141a681e0", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a681e0
    Replacement::draft(0x141a68240, "unknown_141a68240", tpl_34318 as *const ()), // sub_141a68240
    Replacement::draft(0x141a68260, "unknown_141a68260", tpl_34318 as *const ()), // sub_141a68260
    Replacement::draft(0x141a68280, "unknown_141a68280", tpl_34319::<0x143c8e654, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a68280
    Replacement::draft(0x141a682e0, "unknown_141a682e0", tpl_34319::<0x143c91064, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a682e0
    Replacement::draft(0x141a68340, "unknown_141a68340", tpl_34318 as *const ()), // sub_141a68340
    Replacement::draft(0x141a68360, "unknown_141a68360", tpl_34319::<0x143c9deb0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a68360
    Replacement::draft(0x141a683c0, "unknown_141a683c0", tpl_34318 as *const ()), // sub_141a683c0
    Replacement::draft(0x141a683e0, "unknown_141a683e0", tpl_34318 as *const ()), // sub_141a683e0
    Replacement::draft(0x141a68400, "unknown_141a68400", tpl_34318 as *const ()), // sub_141a68400
    Replacement::draft(0x141a68420, "unknown_141a68420", tpl_34319::<0x143c96dec, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a68420
    Replacement::draft(0x141a68480, "unknown_141a68480", tpl_34319::<0x143c8dbd0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a68480
    Replacement::draft(0x141a684e0, "unknown_141a684e0", tpl_34319::<0x143c8dbc4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141a684e0
    Replacement::draft_throws(0x141a68540, "unknown_141a68540", fn_141a68540 as *const (), Throws::Caught), // sub_141a68540
    Replacement::draft(0x141a68a30, "unknown_141a68a30", tpl_361 as *const ()), // sub_141a68a30
    Replacement::draft(0x141a68a40, "unknown_141a68a40", fn_141a68a40 as *const ()), // sub_141a68a40
    Replacement::draft(0x141a68bd0, "unknown_141a68bd0", tpl_361 as *const ()), // sub_141a68bd0
    Replacement::draft(0x141a68be0, "unknown_141a68be0", fn_141a68be0 as *const ()), // sub_141a68be0
    Replacement::draft(0x141a68d80, "unknown_141a68d80", tpl_361 as *const ()), // sub_141a68d80
    Replacement::draft(0x141a68d90, "unknown_141a68d90", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c8dbcc, 0x158, 0x14020e8f0> as *const ()), // sub_141a68d90
    Replacement::draft(0x141a68e80, "unknown_141a68e80", tpl_496 as *const ()), // sub_141a68e80
    Replacement::draft(0x141a68e90, "unknown_141a68e90", fn_141a68e90 as *const ()), // sub_141a68e90
    Replacement::draft(0x141a690f0, "unknown_141a690f0", tpl_361 as *const ()), // sub_141a690f0
    Replacement::draft(0x141a69100, "unknown_141a69100", tpl_361 as *const ()), // sub_141a69100
    Replacement::draft(0x141a69110, "unknown_141a69110", fn_141a69110 as *const ()), // sub_141a69110
    Replacement::draft(0x141a69630, "unknown_141a69630", tpl_361 as *const ()), // sub_141a69630
    Replacement::draft(0x141a69640, "unknown_141a69640", fn_141a69640 as *const ()), // sub_141a69640
    Replacement::draft(0x141a69720, "unknown_141a69720", fn_141a69720 as *const ()), // sub_141a69720
    Replacement::draft(0x141a69ac0, "unknown_141a69ac0", fn_141a69ac0 as *const ()), // sub_141a69ac0
    Replacement::draft(0x141a69d20, "unknown_141a69d20", fn_141a69d20 as *const ()), // sub_141a69d20
    Replacement::draft(0x141a69d40, "unknown_141a69d40", fn_141a69d40 as *const ()), // sub_141a69d40
    Replacement::draft(0x141a69e60, "unknown_141a69e60", fn_141a69e60 as *const ()), // sub_141a69e60
    Replacement::draft(0x141a69ee0, "unknown_141a69ee0", fn_141a69ee0 as *const ()), // sub_141a69ee0
    Replacement::draft(0x141a69f70, "unknown_141a69f70", fn_141a69f70 as *const ()), // sub_141a69f70
    Replacement::draft(0x141a6a0a0, "unknown_141a6a0a0", fn_141a6a0a0 as *const ()), // sub_141a6a0a0
    Replacement::draft(0x141a6a120, "unknown_141a6a120", fn_141a6a120 as *const ()), // sub_141a6a120
    Replacement::draft(0x141a6a150, "unknown_141a6a150", fn_141a6a150 as *const ()), // sub_141a6a150
    Replacement::draft(0x141a6a1b0, "unknown_141a6a1b0", fn_141a6a1b0 as *const ()), // sub_141a6a1b0
    Replacement::draft(0x141a6a1d0, "unknown_141a6a1d0", fn_141a6a1d0 as *const ()), // sub_141a6a1d0
    Replacement::draft(0x141a6a280, "unknown_141a6a280", fn_141a6a280 as *const ()), // sub_141a6a280
    Replacement::draft(0x141a6a2f0, "unknown_141a6a2f0", fn_141a6a2f0 as *const ()), // sub_141a6a2f0
    Replacement::draft(0x141a6a340, "unknown_141a6a340", fn_141a6a340 as *const ()), // sub_141a6a340
    Replacement::draft(0x141a6a3c0, "unknown_141a6a3c0", fn_141a6a3c0 as *const ()), // sub_141a6a3c0
    Replacement::draft(0x141a6a430, "unknown_141a6a430", fn_141a6a430 as *const ()), // sub_141a6a430
    Replacement::draft(0x141a6a570, "unknown_141a6a570", tpl_35253 as *const ()), // sub_141a6a570
    Replacement::draft(0x141a6a580, "unknown_141a6a580", fn_141a6a580 as *const ()), // sub_141a6a580
    Replacement::draft_throws(0x141a6a610, "unknown_141a6a610", fn_141a6a610 as *const (), Throws::Discard), // sub_141a6a610
    Replacement::draft(0x141a6ad60, "bigfish_141a6ad60", fn_141a6ad60 as *const ()), // bigfish::EQSStealthAreaTaskInstance::bigfish::EQSStealthAreaTaskInstance::vfunc_4
    Replacement::draft(0x141a6b6f0, "bigfish_141a6b6f0", fn_141a6b6f0 as *const ()), // bigfish::EQSLeashCoreZoneTaskInstance::bigfish::EQSLeashCoreZoneTaskInstance::vfunc_4
    Replacement::draft(0x141a6c150, "bigfish_141a6c150", fn_141a6c150 as *const ()), // bigfish::EQSEnemyAdvantageZoneTaskInstance::bigfish::EQSEnemyAdvantageZoneTaskInstance::vfunc_4
    Replacement::draft(0x141a6cc00, "bigfish_141a6cc00", tpl_34605::<0x143180770> as *const ()), // sub_141a6cc00
    Replacement::draft(0x141a6ce80, "bigfish_141a6ce80", tpl_35394::<0x9, 0x18, 0x10, 0x1404ffdb0> as *const ()), // sub_141a6ce80
    Replacement::draft(0x141a6cf40, "bigfish_141a6cf40", fn_141a6cf40 as *const ()), // sub_141a6cf40
    Replacement::draft_throws(0x141a6cff0, "bigfish_141a6cff0", tpl_35396::<0x14387f508, 0x140500620, 0x1402116e0, 0x14274cef0, 0x1410d6ca0, 0x140500650, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x140500930, 0x143823788, 0x1404ffe90> as *const (), Throws::Discard), // sub_141a6cff0
    Replacement::draft(0x141a6d300, "bigfish_141a6d300", tpl_36864::<0x8, 0x140200d90, 0x140516a00, 0x1, 0xe, 0x141835030> as *const ()), // sub_141a6d300
    Replacement::draft(0x141a6d3e0, "bigfish_141a6d3e0", tpl_36865::<0x8, 0x140200d90, 0x140516a00, 0xe, 0x141835030> as *const ()), // sub_141a6d3e0
    Replacement::draft(0x141a6d440, "bigfish_141a6d440", tpl_36866::<0x8, 0x140200d90, 0xfffffffffffffff2, 0x141834ee8> as *const ()), // sub_141a6d440
    Replacement::draft(0x141a6d720, "bigfish_141a6d720", fn_141a6d720 as *const ()), // sub_141a6d720
    Replacement::draft(0x141a6d740, "bigfish_141a6d740", tpl_34611::<0x8> as *const ()), // sub_141a6d740
    Replacement::draft(0x141a6e5d0, "bigfish_141a6e5d0", fn_141a6e5d0 as *const ()), // sub_141a6e5d0
    Replacement::draft_throws(0x141a6eb60, "bigfish_141a6eb60", fn_141a6eb60 as *const (), Throws::Caught), // sub_141a6eb60
    Replacement::draft(0x141a6f330, "bigfish_141a6f330", fn_141a6f330 as *const ()), // sub_141a6f330
    Replacement::draft(0x141a6f380, "bigfish_141a6f380", fn_141a6f380 as *const ()), // sub_141a6f380
    Replacement::draft(0x141a6f640, "bigfish_141a6f640", fn_141a6f640 as *const ()), // sub_141a6f640
    Replacement::draft(0x141a6f990, "bigfish_141a6f990", fn_141a6f990 as *const ()), // sub_141a6f990
    Replacement::draft(0x141a6fb80, "bigfish_141a6fb80", fn_141a6fb80 as *const ()), // sub_141a6fb80
    Replacement::draft(0x141a6fcc0, "bigfish_141a6fcc0", fn_141a6fcc0 as *const ()), // sub_141a6fcc0
    Replacement::draft(0x141a70760, "bigfish_141a70760", fn_141a70760 as *const ()), // sub_141a70760
    Replacement::draft(0x141a70840, "bigfish_141a70840", fn_141a70840 as *const ()), // sub_141a70840
    Replacement::draft(0x141a70910, "bigfish_141a70910", fn_141a70910 as *const ()), // sub_141a70910
    Replacement::draft(0x141a70d10, "bigfish_141a70d10", fn_141a70d10 as *const ()), // sub_141a70d10
    Replacement::draft(0x141a711b0, "bigfish_141a711b0", fn_141a711b0 as *const ()), // sub_141a711b0
    Replacement::draft(0x141a71590, "bigfish_141a71590", fn_141a71590 as *const ()), // sub_141a71590
    Replacement::draft(0x141a717a0, "bigfish_141a717a0", fn_141a717a0 as *const ()), // sub_141a717a0
    Replacement::draft(0x141a71db0, "bigfish_141a71db0", fn_141a71db0 as *const ()), // sub_141a71db0
    Replacement::draft(0x141a71fc0, "bigfish_141a71fc0", fn_141a71fc0 as *const ()), // sub_141a71fc0
    Replacement::draft(0x141a72100, "bigfish_141a72100", fn_141a72100 as *const ()), // sub_141a72100
    Replacement::draft(0x141a72250, "bigfish_141a72250", fn_141a72250 as *const ()), // sub_141a72250
    Replacement::draft_throws(0x141a72320, "bigfish_141a72320", fn_141a72320 as *const (), Throws::Discard), // sub_141a72320
    Replacement::draft(0x141a72410, "bigfish_141a72410", fn_141a72410 as *const ()), // sub_141a72410
    Replacement::draft(0x141a72790, "bigfish_141a72790", fn_141a72790 as *const ()), // sub_141a72790
    Replacement::draft(0x141a728b0, "bigfish_141a728b0", fn_141a728b0 as *const ()), // sub_141a728b0
    Replacement::draft(0x141a72c80, "bigfish_141a72c80", fn_141a72c80 as *const ()), // sub_141a72c80
    Replacement::draft(0x141a72f20, "bigfish_141a72f20", fn_141a72f20 as *const ()), // sub_141a72f20
    Replacement::draft(0x141a73000, "bigfish_141a73000", fn_141a73000 as *const ()), // sub_141a73000
    Replacement::draft(0x141a73190, "bigfish_141a73190", fn_141a73190 as *const ()), // sub_141a73190
    Replacement::draft(0x141a73330, "bigfish_141a73330", fn_141a73330 as *const ()), // sub_141a73330
    Replacement::draft(0x141a73410, "bigfish_141a73410", fn_141a73410 as *const ()), // sub_141a73410
    Replacement::draft(0x141a737b0, "bigfish_141a737b0", fn_141a737b0 as *const ()), // sub_141a737b0
    Replacement::draft(0x141a73920, "bigfish_141a73920", fn_141a73920 as *const ()), // sub_141a73920
    Replacement::draft(0x141a73b80, "bigfish_141a73b80", fn_141a73b80 as *const ()), // sub_141a73b80
    Replacement::draft(0x141a73cb0, "bigfish_141a73cb0", fn_141a73cb0 as *const ()), // sub_141a73cb0
    Replacement::draft(0x141a73e80, "bigfish_141a73e80", fn_141a73e80 as *const ()), // sub_141a73e80
    Replacement::draft(0x141a73f40, "bigfish_141a73f40", fn_141a73f40 as *const ()), // sub_141a73f40
    Replacement::draft(0x141a74080, "bigfish_141a74080", fn_141a74080 as *const ()), // sub_141a74080
    Replacement::draft(0x141a740d0, "bigfish_141a740d0", fn_141a740d0 as *const ()), // sub_141a740d0
    Replacement::draft(0x141a742d0, "bigfish_141a742d0", fn_141a742d0 as *const ()), // sub_141a742d0
    Replacement::draft(0x141a745d0, "bigfish_141a745d0", fn_141a745d0 as *const ()), // sub_141a745d0
    Replacement::draft(0x141a74800, "bigfish_141a74800", fn_141a74800 as *const ()), // sub_141a74800
    Replacement::draft(0x141a748b0, "bigfish_141a748b0", fn_141a748b0 as *const ()), // sub_141a748b0
    Replacement::draft(0x141a74920, "bigfish_141a74920", fn_141a74920 as *const ()), // sub_141a74920
    Replacement::draft(0x141a74940, "bigfish_141a74940", fn_141a74940 as *const ()), // sub_141a74940
    Replacement::draft(0x141a749b0, "bigfish_141a749b0", tpl_36903::<0x10> as *const ()), // sub_141a749b0
    Replacement::draft(0x141a749d0, "bigfish_141a749d0", fn_141a749d0 as *const ()), // sub_141a749d0
    Replacement::draft(0x141a74ab0, "bigfish_141a74ab0", fn_141a74ab0 as *const ()), // sub_141a74ab0
    Replacement::draft(0x141a74b30, "bigfish_141a74b30", fn_141a74b30 as *const ()), // sub_141a74b30
    Replacement::draft(0x141a74b50, "bigfish_141a74b50", fn_141a74b50 as *const ()), // sub_141a74b50
    Replacement::draft(0x141a74ba0, "bigfish_141a74ba0", fn_141a74ba0 as *const ()), // sub_141a74ba0
    Replacement::draft(0x141a74c20, "bigfish_141a74c20", fn_141a74c20 as *const ()), // sub_141a74c20
    Replacement::draft(0x141a74c40, "bigfish_141a74c40", fn_141a74c40 as *const ()), // sub_141a74c40
    Replacement::draft(0x141a74d10, "bigfish_141a74d10", fn_141a74d10 as *const ()), // sub_141a74d10
    Replacement::draft(0x141a74f70, "bigfish_141a74f70", fn_141a74f70 as *const ()), // sub_141a74f70
    Replacement::draft(0x141a75010, "bigfish_141a75010", fn_141a75010 as *const ()), // sub_141a75010
    Replacement::draft(0x141a75060, "bigfish_141a75060", fn_141a75060 as *const ()), // sub_141a75060
    Replacement::draft(0x141a75170, "bigfish_141a75170", fn_141a75170 as *const ()), // sub_141a75170
    Replacement::draft(0x141a751a0, "bigfish_141a751a0", fn_141a751a0 as *const ()), // sub_141a751a0
    Replacement::draft(0x141a75260, "bigfish_141a75260", fn_141a75260 as *const ()), // sub_141a75260
    Replacement::draft(0x141a75420, "bigfish_141a75420", tpl_36914::<0x1430b0330, 0x2, 0x14020f270> as *const ()), // bigfish::door::SaveData::ClassTypeInfoImplementation::bigfish::door::SaveData::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141a75460, "bigfish_141a75460", tpl_36914::<0x1430b0330, 0x2, 0x14020f270> as *const ()), // bigfish::door::SaveData::ClassTypeInfoImplementation::bigfish::door::SaveData::ClassTypeInfoImplementation::vfunc_37
    Replacement::draft(0x141a756c0, "bigfish_141a756c0", fn_141a756c0 as *const ()), // bigfish::door::SaveData::ClassTypeInfoImplementation::bigfish::door::SaveData::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141a75720, "bigfish_141a75720", fn_141a75720 as *const ()), // bigfish::door::SaveData::ClassTypeInfoImplementation::bigfish::door::SaveData::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141a758a0, "bigfish_141a758a0", tpl_36917::<0x14387b850, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bigfish::door::SaveData::ClassTypeInfoImplementation::bigfish::door::SaveData::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141a75900, "bigfish_141a75900", tpl_36917::<0x14387b850, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bigfish::door::SaveData::ClassTypeInfoImplementation::bigfish::door::SaveData::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141a75ab0, "bigfish_141a75ab0", fn_141a75ab0 as *const ()), // sub_141a75ab0
    Replacement::draft(0x141a75b10, "bigfish_141a75b10", fn_141a75b10 as *const ()), // sub_141a75b10
    Replacement::draft(0x141a75be0, "bigfish_141a75be0", fn_141a75be0 as *const ()), // sub_141a75be0
    Replacement::draft(0x141a75ef0, "bigfish_141a75ef0", fn_141a75ef0 as *const ()), // sub_141a75ef0
    Replacement::draft(0x141a75f30, "bigfish_141a75f30", fn_141a75f30 as *const ()), // sub_141a75f30
    Replacement::draft(0x141a75f50, "bigfish_141a75f50", fn_141a75f50 as *const ()), // sub_141a75f50
    Replacement::draft(0x141a76060, "bigfish_141a76060", fn_141a76060 as *const ()), // sub_141a76060
    Replacement::draft(0x141a760a0, "bigfish_141a760a0", fn_141a760a0 as *const ()), // sub_141a760a0
    Replacement::draft(0x141a760c0, "bigfish_141a760c0", fn_141a760c0 as *const ()), // sub_141a760c0
    Replacement::draft(0x141a761e0, "bigfish_141a761e0", fn_141a761e0 as *const ()), // sub_141a761e0
    Replacement::draft(0x141a76200, "bigfish_141a76200", fn_141a76200 as *const ()), // sub_141a76200
    Replacement::draft(0x141a76220, "bigfish_141a76220", fn_141a76220 as *const ()), // sub_141a76220
    Replacement::draft(0x141a76240, "bigfish_141a76240", fn_141a76240 as *const ()), // sub_141a76240
    Replacement::draft(0x141a763a0, "bigfish_141a763a0", fn_141a763a0 as *const ()), // sub_141a763a0
    Replacement::draft(0x141a763e0, "bigfish_141a763e0", tpl_36932::<0x10, 0x6c, 0x143426514> as *const ()), // sub_141a763e0
    Replacement::draft(0x141a76400, "bigfish_141a76400", fn_141a76400 as *const ()), // sub_141a76400
    Replacement::draft(0x141a764e0, "bigfish_141a764e0", fn_141a764e0 as *const ()), // sub_141a764e0
    Replacement::draft(0x141a765a0, "bigfish_141a765a0", fn_141a765a0 as *const ()), // sub_141a765a0
    Replacement::draft(0x141a765f0, "bigfish_141a765f0", fn_141a765f0 as *const ()), // sub_141a765f0
    Replacement::draft(0x141a766c0, "bigfish_141a766c0", tpl_36935::<0x10, 0x2c> as *const ()), // sub_141a766c0
    Replacement::draft(0x141a766e0, "bigfish_141a766e0", fn_141a766e0 as *const ()), // sub_141a766e0
    Replacement::draft(0x141a76730, "bigfish_141a76730", fn_141a76730 as *const ()), // sub_141a76730
    Replacement::draft(0x141a76890, "bigfish_141a76890", fn_141a76890 as *const ()), // sub_141a76890
    Replacement::draft(0x141a76960, "bigfish_141a76960", fn_141a76960 as *const ()), // sub_141a76960
    Replacement::draft(0x141a769c0, "bigfish_141a769c0", tpl_36935::<0x10, 0x2d> as *const ()), // sub_141a769c0
    Replacement::draft(0x141a769e0, "bigfish_141a769e0", tpl_36932::<0x10, 0x70, 0x143426514> as *const ()), // sub_141a769e0
    Replacement::draft(0x141a76a00, "bigfish_141a76a00", fn_141a76a00 as *const ()), // sub_141a76a00
    Replacement::draft(0x141a76ae0, "bigfish_141a76ae0", fn_141a76ae0 as *const ()), // sub_141a76ae0
    Replacement::draft(0x141a76e00, "bigfish_141a76e00", fn_141a76e00 as *const ()), // sub_141a76e00
    Replacement::draft(0x141a76f00, "bigfish_141a76f00", fn_141a76f00 as *const ()), // sub_141a76f00
    Replacement::draft(0x141a76ff0, "bigfish_141a76ff0", fn_141a76ff0 as *const ()), // sub_141a76ff0
    Replacement::draft(0x141a77180, "bigfish_141a77180", fn_141a77180 as *const ()), // sub_141a77180
    Replacement::draft(0x141a771a0, "bigfish_141a771a0", fn_141a771a0 as *const ()), // sub_141a771a0
    Replacement::draft(0x141a77410, "bigfish_141a77410", fn_141a77410 as *const ()), // sub_141a77410
    Replacement::draft(0x141a77460, "bigfish_141a77460", fn_141a77460 as *const ()), // sub_141a77460
    Replacement::draft(0x141a77570, "bigfish_141a77570", fn_141a77570 as *const ()), // sub_141a77570
    Replacement::draft_throws(0x141a77650, "bigfish_141a77650", fn_141a77650 as *const (), Throws::Discard), // sub_141a77650
    Replacement::draft(0x141a79220, "bigfish_141a79220", fn_141a79220 as *const ()), // sub_141a79220
    Replacement::draft(0x141a792e0, "bigfish_141a792e0", fn_141a792e0 as *const ()), // sub_141a792e0
    Replacement::draft_throws(0x141a79450, "bigfish_141a79450", fn_141a79450 as *const (), Throws::Caught), // sub_141a79450
    Replacement::draft(0x141a7aa60, "bigfish_141a7aa60", fn_141a7aa60 as *const ()), // sub_141a7aa60
    Replacement::draft(0x141a7abd0, "bigfish_141a7abd0", fn_141a7abd0 as *const ()), // sub_141a7abd0
    Replacement::draft(0x141a7aef0, "bigfish_141a7aef0", fn_141a7aef0 as *const ()), // sub_141a7aef0
    Replacement::draft(0x141a7af60, "bigfish_141a7af60", fn_141a7af60 as *const ()), // sub_141a7af60
    Replacement::draft(0x141a7b1b0, "bigfish_141a7b1b0", fn_141a7b1b0 as *const ()), // sub_141a7b1b0
    Replacement::draft(0x141a7b990, "bigfish_141a7b990", fn_141a7b990 as *const ()), // sub_141a7b990
    Replacement::draft(0x141a7bdb0, "bigfish_141a7bdb0", fn_141a7bdb0 as *const ()), // sub_141a7bdb0
    Replacement::draft(0x141a7c200, "bigfish_141a7c200", fn_141a7c200 as *const ()), // sub_141a7c200
    Replacement::draft(0x141a7c280, "bigfish_141a7c280", fn_141a7c280 as *const ()), // sub_141a7c280
    Replacement::draft(0x141a7c320, "bigfish_141a7c320", fn_141a7c320 as *const ()), // sub_141a7c320
    Replacement::draft_throws(0x141a7ca60, "bigfish_141a7ca60", fn_141a7ca60 as *const (), Throws::Discard), // sub_141a7ca60
    Replacement::draft(0x141a7d060, "bigfish_141a7d060", fn_141a7d060 as *const ()), // sub_141a7d060
    Replacement::draft(0x141a7d440, "bigfish_141a7d440", fn_141a7d440 as *const ()), // sub_141a7d440
    Replacement::draft(0x141a7d750, "bigfish_141a7d750", tpl_35758::<0x8, 0x1405231c0> as *const ()), // sub_141a7d750
    Replacement::draft(0x141a7d800, "bigfish_141a7d800", tpl_35759::<0x8, 0x1405231c0> as *const ()), // sub_141a7d800
    Replacement::draft(0x141a7d830, "bigfish_141a7d830", tpl_34684::<0x1401f2bc0> as *const ()), // sub_141a7d830
    Replacement::draft(0x141a7e2f0, "bigfish_141a7e2f0", fn_141a7e2f0 as *const ()), // sub_141a7e2f0
    Replacement::draft(0x141a7e3f0, "bigfish_141a7e3f0", tpl_5094::<0x8, 0x14051fb30, 0x14051fd30> as *const ()), // sub_141a7e3f0
    Replacement::draft(0x141a7e460, "bigfish_141a7e460", fn_141a7e460 as *const ()), // sub_141a7e460
    Replacement::draft(0x141a7e560, "bigfish_141a7e560", fn_141a7e560 as *const ()), // sub_141a7e560
    Replacement::draft(0x141a7e780, "bigfish_141a7e780", fn_141a7e780 as *const ()), // sub_141a7e780
    Replacement::draft(0x141a7ead0, "bigfish_141a7ead0", fn_141a7ead0 as *const ()), // sub_141a7ead0
    Replacement::draft(0x141a7f0b0, "bigfish_141a7f0b0", fn_141a7f0b0 as *const ()), // sub_141a7f0b0
    Replacement::draft(0x141a7f380, "bigfish_141a7f380", fn_141a7f380 as *const ()), // sub_141a7f380
    Replacement::draft(0x141a7f630, "bigfish_141a7f630", fn_141a7f630 as *const ()), // sub_141a7f630
    Replacement::draft(0x141a7f910, "bigfish_141a7f910", fn_141a7f910 as *const ()), // sub_141a7f910
    Replacement::draft(0x141a7ff70, "bigfish_141a7ff70", fn_141a7ff70 as *const ()), // sub_141a7ff70
    Replacement::draft_throws(0x141a801b0, "bigfish_141a801b0", fn_141a801b0 as *const (), Throws::Discard), // sub_141a801b0
    Replacement::draft(0x141a803e0, "bigfish_141a803e0", fn_141a803e0 as *const ()), // sub_141a803e0
    Replacement::draft(0x141a80530, "bigfish_141a80530", fn_141a80530 as *const ()), // sub_141a80530
    Replacement::draft(0x141a805a0, "bigfish_141a805a0", fn_141a805a0 as *const ()), // sub_141a805a0
    Replacement::draft(0x141a80670, "bigfish_141a80670", fn_141a80670 as *const ()), // sub_141a80670
    Replacement::draft(0x141a80840, "bigfish_141a80840", tpl_36968::<0x28, 0x8, 0x18, 0x30, 0x14, 0x142489e40, 0x20, 0x141a809e0> as *const ()), // sub_141a80840
    Replacement::draft(0x141a80910, "bigfish_141a80910", tpl_36968::<0x28, 0x8, 0x18, 0x30, 0x10, 0x142489e40, 0x20, 0x141a809e0> as *const ()), // sub_141a80910
    Replacement::draft(0x141a809e0, "bigfish_141a809e0", fn_141a809e0 as *const ()), // sub_141a809e0
    Replacement::draft(0x141a80c10, "bigfish_141a80c10", fn_141a80c10 as *const ()), // sub_141a80c10
    Replacement::draft(0x141a812a0, "bigfish_141a812a0", fn_141a812a0 as *const ()), // sub_141a812a0
    Replacement::draft(0x141a81860, "bigfish_141a81860", fn_141a81860 as *const ()), // sub_141a81860
    Replacement::draft(0x141a818b0, "bigfish_141a818b0", fn_141a818b0 as *const ()), // sub_141a818b0
    Replacement::draft(0x141a81bc0, "bigfish_141a81bc0", fn_141a81bc0 as *const ()), // sub_141a81bc0
    Replacement::draft(0x141a81cd0, "bigfish_141a81cd0", fn_141a81cd0 as *const ()), // sub_141a81cd0
    Replacement::draft(0x141a81d50, "bigfish_141a81d50", fn_141a81d50 as *const ()), // sub_141a81d50
    Replacement::draft_throws(0x141a81de0, "bigfish_141a81de0", fn_141a81de0 as *const (), Throws::Discard), // sub_141a81de0
    Replacement::draft(0x141a826c0, "bigfish_141a826c0", tpl_1439::<0x8, 0x140526050> as *const ()), // sub_141a826c0
    Replacement::draft(0x141a82700, "bigfish_141a82700", tpl_36972::<0x50> as *const ()), // sub_141a82700
    Replacement::draft(0x141a82710, "bigfish_141a82710", tpl_36973::<0x60, 0x50, 0x78, 0x80, 0x4, 0x14> as *const ()), // sub_141a82710
    Replacement::draft(0x141a82780, "bigfish_141a82780", tpl_36972::<0x64> as *const ()), // sub_141a82780
    Replacement::draft(0x141a82790, "bigfish_141a82790", tpl_36973::<0x74, 0x64, 0x88, 0x90, 0x4, 0x14> as *const ()), // sub_141a82790
    Replacement::draft(0x141a82800, "bigfish_141a82800", fn_141a82800 as *const ()), // sub_141a82800
    Replacement::draft(0x141a82820, "bigfish_141a82820", fn_141a82820 as *const ()), // sub_141a82820
    Replacement::draft(0x141a82840, "bigfish_141a82840", fn_141a82840 as *const ()), // sub_141a82840
    Replacement::draft(0x141a82960, "bigfish_141a82960", tpl_36976 as *const ()), // sub_141a82960
    Replacement::draft_throws(0x141a82970, "bigfish_141a82970", fn_141a82970 as *const (), Throws::Discard), // sub_141a82970
    Replacement::draft(0x141a82dc0, "bigfish_141a82dc0", fn_141a82dc0 as *const ()), // sub_141a82dc0
    Replacement::draft(0x141a82f80, "bigfish_141a82f80", tpl_35097 as *const ()), // sub_141a82f80
    Replacement::draft(0x141a83030, "bigfish_141a83030", fn_141a83030 as *const ()), // sub_141a83030
    Replacement::draft(0x141a831f0, "bigfish_141a831f0", fn_141a831f0 as *const ()), // sub_141a831f0
    Replacement::draft(0x141a832b0, "bigfish_141a832b0", tpl_34611::<0x8> as *const ()), // sub_141a832b0
    Replacement::draft(0x141a83850, "bigfish_141a83850", tpl_36980::<0x4> as *const ()), // sub_141a83850
    Replacement::draft_throws(0x141a83880, "bigfish_141a83880", fn_141a83880 as *const (), Throws::Caught), // sub_141a83880
    Replacement::draft(0x141a845f0, "bigfish_141a845f0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143caef00, 0x158, 0x14020e8f0> as *const ()), // sub_141a845f0
    Replacement::draft_throws(0x141a846f0, "bigfish_141a846f0", fn_141a846f0 as *const (), Throws::Caught), // sub_141a846f0
    Replacement::draft_throws(0x141a849b0, "bigfish_141a849b0", fn_141a849b0 as *const (), Throws::Discard), // sub_141a849b0
    Replacement::draft_throws(0x141a84cb0, "bigfish_141a84cb0", fn_141a84cb0 as *const (), Throws::Discard), // sub_141a84cb0
    Replacement::draft_throws(0x141a84eb0, "bigfish_141a84eb0", fn_141a84eb0 as *const (), Throws::Discard), // sub_141a84eb0
    Replacement::draft(0x141a853b0, "bigfish_141a853b0", fn_141a853b0 as *const ()), // sub_141a853b0
    Replacement::draft(0x141a85690, "bigfish_141a85690", fn_141a85690 as *const ()), // sub_141a85690
    Replacement::draft(0x141a85830, "bigfish_141a85830", fn_141a85830 as *const ()), // sub_141a85830
    Replacement::draft(0x141a85b70, "bigfish_141a85b70", fn_141a85b70 as *const ()), // sub_141a85b70
    Replacement::draft(0x141a85e90, "bigfish_141a85e90", fn_141a85e90 as *const ()), // sub_141a85e90
    Replacement::draft_throws(0x141a85f50, "bigfish_141a85f50", fn_141a85f50 as *const (), Throws::Discard), // sub_141a85f50
    Replacement::draft(0x141a862c0, "bigfish_141a862c0", tpl_34605::<0x143197a78> as *const ()), // sub_141a862c0
    Replacement::draft(0x141a86480, "bigfish_141a86480", tpl_35751::<0x20, 0x40, 0x60> as *const ()), // sub_141a86480
    Replacement::draft_throws(0x141a866f0, "bigfish_141a866f0", fn_141a866f0 as *const (), Throws::Discard), // sub_141a866f0
    Replacement::draft(0x141a86b30, "bigfish_141a86b30", fn_141a86b30 as *const ()), // sub_141a86b30
    Replacement::draft(0x141a86be0, "bigfish_141a86be0", fn_141a86be0 as *const ()), // sub_141a86be0
    Replacement::draft(0x141a86cc0, "bigfish_141a86cc0", fn_141a86cc0 as *const ()), // sub_141a86cc0
    Replacement::draft_throws(0x141a86ce0, "bigfish_141a86ce0", fn_141a86ce0 as *const (), Throws::Discard), // sub_141a86ce0
    Replacement::draft(0x141a86e80, "bigfish_141a86e80", fn_141a86e80 as *const ()), // sub_141a86e80
    Replacement::draft(0x141a87040, "bigfish_141a87040", fn_141a87040 as *const ()), // sub_141a87040
    Replacement::draft(0x141a87290, "bigfish_141a87290", tpl_36992::<0xd0, 0x104> as *const ()), // sub_141a87290
    Replacement::draft(0x141a87330, "bigfish_141a87330", fn_141a87330 as *const ()), // sub_141a87330
    Replacement::draft(0x141a87470, "bigfish_141a87470", fn_141a87470 as *const ()), // sub_141a87470
    Replacement::draft_throws(0x141a875d0, "bigfish_141a875d0", fn_141a875d0 as *const (), Throws::Discard), // sub_141a875d0
    Replacement::draft_throws(0x141a875f0, "bigfish_141a875f0", fn_141a875f0 as *const (), Throws::Caught), // sub_141a875f0
    Replacement::draft_throws(0x141a87950, "bigfish_141a87950", fn_141a87950 as *const (), Throws::Discard), // sub_141a87950
    Replacement::draft_throws(0x141a87a40, "bigfish_141a87a40", fn_141a87a40 as *const (), Throws::Caught), // sub_141a87a40
    Replacement::draft(0x141a87e30, "bigfish_141a87e30", fn_141a87e30 as *const ()), // sub_141a87e30
    Replacement::draft(0x141a88040, "bigfish_141a88040", fn_141a88040 as *const ()), // sub_141a88040
    Replacement::draft_throws(0x141a882f0, "bigfish_141a882f0", fn_141a882f0 as *const (), Throws::Caught), // sub_141a882f0
    Replacement::draft_throws(0x141a88990, "bigfish_141a88990", fn_141a88990 as *const (), Throws::Caught), // sub_141a88990
    Replacement::draft_throws(0x141a88a20, "bigfish_141a88a20", fn_141a88a20 as *const (), Throws::Caught), // sub_141a88a20
    Replacement::draft_throws(0x141a88bc0, "bigfish_141a88bc0", fn_141a88bc0 as *const (), Throws::Caught), // sub_141a88bc0
    Replacement::draft(0x141a88f60, "bigfish_141a88f60", fn_141a88f60 as *const ()), // sub_141a88f60
    Replacement::draft_throws(0x141a89350, "bigfish_141a89350", fn_141a89350 as *const (), Throws::Caught), // sub_141a89350
    Replacement::draft(0x141a89830, "bigfish_141a89830", tpl_37005::<0xe8, 0x14052f984, 0x1401dffd0, 0x38, 0x141b722a0> as *const ()), // sub_141a89830
    Replacement::draft_throws(0x141a89890, "bigfish_141a89890", fn_141a89890 as *const (), Throws::Caught), // sub_141a89890
    Replacement::draft_throws(0x141a899f0, "bigfish_141a899f0", fn_141a899f0 as *const (), Throws::Discard), // bigfish::DecoratorCheckAttackCategoriesInstance::bigfish::DecoratorCheckAttackCategoriesInstance::vfunc_1
    Replacement::draft_throws(0x141a89a60, "bigfish_141a89a60", fn_141a89a60 as *const (), Throws::Discard), // bigfish::DecoratorCheckAttackCategoriesInstance::bigfish::DecoratorCheckAttackCategoriesInstance::vfunc_33
    Replacement::draft(0x141a89b20, "bigfish_141a89b20", fn_141a89b20 as *const ()), // bigfish::DecoratorCheckAttackCategoriesInstance::bigfish::DecoratorCheckAttackCategoriesInstance::vfunc_7
    Replacement::draft(0x141a89b90, "bigfish_141a89b90", fn_141a89b90 as *const ()), // bigfish::DecoratorCheckAttackCategoriesInstance::bigfish::DecoratorCheckAttackCategoriesInstance::vfunc_5
    Replacement::draft_throws(0x141a89c40, "bigfish_141a89c40", fn_141a89c40 as *const (), Throws::Discard), // bigfish::DecoratorCheckAttackCategoriesInstance::bigfish::DecoratorCheckAttackCategoriesInstance::vfunc_4
    Replacement::draft(0x141a8a430, "bigfish_141a8a430", tpl_34605::<0x143199578> as *const ()), // sub_141a8a430
    Replacement::draft(0x141a8a5b0, "bigfish_141a8a5b0", fn_141a8a5b0 as *const ()), // sub_141a8a5b0
    Replacement::draft_throws(0x141a8a5e0, "bigfish_141a8a5e0", fn_141a8a5e0 as *const (), Throws::Discard), // sub_141a8a5e0
    Replacement::draft(0x141a8a8f0, "bigfish_141a8a8f0", tpl_35758::<0x8, 0x140540100> as *const ()), // sub_141a8a8f0
    Replacement::draft(0x141a8a9a0, "bigfish_141a8a9a0", tpl_35759::<0x8, 0x140540100> as *const ()), // sub_141a8a9a0
    Replacement::draft(0x141a8a9d0, "bigfish_141a8a9d0", tpl_34684::<0x1401f2bc0> as *const ()), // sub_141a8a9d0
    Replacement::draft(0x141a8ab00, "bigfish_141a8ab00", fn_141a8ab00 as *const ()), // sub_141a8ab00
    Replacement::draft(0x141a8ad90, "bigfish_141a8ad90", fn_141a8ad90 as *const ()), // sub_141a8ad90
    Replacement::draft(0x141a8ae40, "bigfish_141a8ae40", fn_141a8ae40 as *const ()), // sub_141a8ae40
    Replacement::draft(0x141a8ae70, "bigfish_141a8ae70", tpl_35266::<0x8, 0x1401f2bc0> as *const ()), // sub_141a8ae70
    Replacement::draft(0x141a8afa0, "bigfish_141a8afa0", tpl_34613 as *const ()), // sub_141a8afa0
    Replacement::draft(0x141a8beb0, "bigfish_141a8beb0", fn_141a8beb0 as *const ()), // sub_141a8beb0
    Replacement::draft(0x141a8c040, "bigfish_141a8c040", tpl_37015::<0x1401e7c10, 0x1405305a0> as *const ()), // sub_141a8c040
    Replacement::draft_throws(0x141a8c220, "bigfish_141a8c220", fn_141a8c220 as *const (), Throws::Discard), // sub_141a8c220
    Replacement::draft(0x141a8c620, "bigfish_141a8c620", tpl_37015::<0x1401e7c10, 0x1405305a0> as *const ()), // sub_141a8c620
    Replacement::draft(0x141a8c810, "bigfish_141a8c810", fn_141a8c810 as *const ()), // sub_141a8c810
    Replacement::draft(0x141a8cb00, "bigfish_141a8cb00", fn_141a8cb00 as *const ()), // sub_141a8cb00
    Replacement::draft(0x141a8cd00, "bigfish_141a8cd00", fn_141a8cd00 as *const ()), // sub_141a8cd00
    Replacement::draft_throws(0x141a8cfe0, "bigfish_141a8cfe0", fn_141a8cfe0 as *const (), Throws::Caught), // sub_141a8cfe0
    Replacement::draft_throws(0x141a8db30, "bigfish_141a8db30", fn_141a8db30 as *const (), Throws::Caught), // sub_141a8db30
    Replacement::draft(0x141a8e070, "bigfish_141a8e070", fn_141a8e070 as *const ()), // sub_141a8e070
    Replacement::draft_throws(0x141a8e400, "bigfish_141a8e400", fn_141a8e400 as *const (), Throws::Discard), // sub_141a8e400
    Replacement::draft(0x141a8e860, "bigfish_141a8e860", fn_141a8e860 as *const ()), // sub_141a8e860
    Replacement::draft(0x141a8ed60, "bigfish_141a8ed60", fn_141a8ed60 as *const ()), // sub_141a8ed60
    Replacement::draft_throws(0x141a8f520, "bigfish_141a8f520", fn_141a8f520 as *const (), Throws::Discard), // sub_141a8f520
    Replacement::draft(0x141a8f8f0, "bigfish_141a8f8f0", fn_141a8f8f0 as *const ()), // sub_141a8f8f0
    Replacement::draft(0x141a90100, "bigfish_141a90100", fn_141a90100 as *const ()), // sub_141a90100
    Replacement::draft_throws(0x141a903e0, "bigfish_141a903e0", fn_141a903e0 as *const (), Throws::Discard), // sub_141a903e0
    Replacement::draft(0x141a90d00, "bigfish_141a90d00", fn_141a90d00 as *const ()), // sub_141a90d00
    Replacement::draft(0x141a90ea0, "bigfish_141a90ea0", fn_141a90ea0 as *const ()), // sub_141a90ea0
    Replacement::draft(0x141a90fa0, "bigfish_141a90fa0", fn_141a90fa0 as *const ()), // sub_141a90fa0
    Replacement::draft(0x141a91010, "bigfish_141a91010", fn_141a91010 as *const ()), // sub_141a91010
    Replacement::draft_throws(0x141a91130, "bigfish_141a91130", fn_141a91130 as *const (), Throws::Discard), // sub_141a91130
    Replacement::draft(0x141a919d0, "bigfish_141a919d0", fn_141a919d0 as *const ()), // sub_141a919d0
    Replacement::draft(0x141a91ac0, "bigfish_141a91ac0", fn_141a91ac0 as *const ()), // sub_141a91ac0
    Replacement::draft(0x141a91b10, "bigfish_141a91b10", fn_141a91b10 as *const ()), // sub_141a91b10
    Replacement::draft(0x141a91be0, "bigfish_141a91be0", tpl_94771::<0x1401ff5b0> as *const ()), // sub_141a91be0
    Replacement::draft(0x141a91ca0, "bigfish_141a91ca0", tpl_34684::<0x1401f2bc0> as *const ()), // sub_141a91ca0
    Replacement::draft(0x141a91d50, "bigfish_141a91d50", tpl_96964::<0x1431a1b00, 0x8> as *const ()), // sub_141a91d50
    Replacement::draft(0x141a91f40, "bigfish_141a91f40", fn_141a91f40 as *const ()), // sub_141a91f40
    Replacement::draft(0x141a921e0, "bigfish_141a921e0", fn_141a921e0 as *const ()), // sub_141a921e0
    Replacement::draft_throws(0x141a922f0, "bigfish_141a922f0", tpl_36792::<0x14387ecd4, 0x140540ac0, 0x1402116e0, 0x1427595f0, 0x18, 0x1410d6ca0, 0x10, 0x1405402d0> as *const (), Throws::Discard), // sub_141a922f0
    Replacement::draft(0x141a923f0, "bigfish_141a923f0", tpl_95133::<0x1431a3b50, 0x1, 0x8, 0x140546110> as *const ()), // sub_141a923f0
    Replacement::draft(0x141a92620, "bigfish_141a92620", fn_141a92620 as *const ()), // sub_141a92620
    Replacement::draft(0x141a927b0, "bigfish_141a927b0", fn_141a927b0 as *const ()), // sub_141a927b0
    Replacement::draft(0x141a92810, "bigfish_141a92810", fn_141a92810 as *const ()), // sub_141a92810
    Replacement::draft(0x141a92f90, "bigfish_141a92f90", fn_141a92f90 as *const ()), // sub_141a92f90
    Replacement::draft(0x141a93080, "bigfish_141a93080", fn_141a93080 as *const ()), // sub_141a93080
    Replacement::draft(0x141a93180, "bigfish_141a93180", fn_141a93180 as *const ()), // sub_141a93180
    Replacement::draft(0x141a93330, "bigfish_141a93330", fn_141a93330 as *const ()), // sub_141a93330
    Replacement::draft(0x141a93360, "bigfish_141a93360", fn_141a93360 as *const ()), // sub_141a93360
    Replacement::draft(0x141a933c0, "bigfish_141a933c0", fn_141a933c0 as *const ()), // sub_141a933c0
    Replacement::draft(0x141a93400, "bigfish_141a93400", fn_141a93400 as *const ()), // sub_141a93400
    Replacement::draft(0x141a93670, "bigfish_141a93670", fn_141a93670 as *const ()), // sub_141a93670
    Replacement::draft(0x141a93740, "bigfish_141a93740", fn_141a93740 as *const ()), // sub_141a93740
    Replacement::draft(0x141a93770, "bigfish_141a93770", tpl_19928::<0x8, 0x140543190, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141a93770
    Replacement::draft(0x141a938b0, "bigfish_141a938b0", tpl_37040::<0x143cb0f2c, 0x14020e8f0, 0x143cb0f1c> as *const ()), // sub_141a938b0
    Replacement::draft_throws(0x141a93a90, "bigfish_141a93a90", fn_141a93a90 as *const (), Throws::Discard), // sub_141a93a90
    Replacement::draft(0x141a93f70, "bigfish_141a93f70", fn_141a93f70 as *const ()), // sub_141a93f70
    Replacement::draft_throws(0x141a93fa0, "bigfish_141a93fa0", fn_141a93fa0 as *const (), Throws::Caught), // sub_141a93fa0
    Replacement::draft_throws(0x141a94ec0, "bigfish_141a94ec0", fn_141a94ec0 as *const (), Throws::Discard), // sub_141a94ec0
    Replacement::draft(0x141a95070, "bigfish_141a95070", fn_141a95070 as *const ()), // sub_141a95070
    Replacement::draft(0x141a950a0, "bigfish_141a950a0", fn_141a950a0 as *const ()), // sub_141a950a0
    Replacement::draft(0x141a95240, "bigfish_141a95240", fn_141a95240 as *const ()), // sub_141a95240
    Replacement::draft(0x141a953d0, "bigfish_141a953d0", fn_141a953d0 as *const ()), // sub_141a953d0
    Replacement::draft(0x141a958c0, "bigfish_141a958c0", fn_141a958c0 as *const ()), // sub_141a958c0
    Replacement::draft_throws(0x141a95c30, "bigfish_141a95c30", fn_141a95c30 as *const (), Throws::Discard), // sub_141a95c30
    Replacement::draft(0x141a961b0, "bigfish_141a961b0", tpl_34394::<0x10> as *const ()), // sub_141a961b0
    Replacement::draft(0x141a96250, "bigfish_141a96250", tpl_34394::<0x10> as *const ()), // sub_141a96250
    Replacement::draft(0x141a962f0, "bigfish_141a962f0", tpl_94875::<0x8, 0xc> as *const ()), // sub_141a962f0
    Replacement::draft(0x141a963f0, "bigfish_141a963f0", tpl_34684::<0x1401e59a0> as *const ()), // sub_141a963f0
    Replacement::draft(0x141a96cb0, "bigfish_141a96cb0", fn_141a96cb0 as *const ()), // sub_141a96cb0
    Replacement::draft_throws(0x141a96d80, "bigfish_141a96d80", fn_141a96d80 as *const (), Throws::Caught), // sub_141a96d80
    Replacement::draft(0x141a97430, "bigfish_141a97430", fn_141a97430 as *const ()), // sub_141a97430
    Replacement::draft(0x141a97fc0, "bigfish_141a97fc0", fn_141a97fc0 as *const ()), // sub_141a97fc0
    Replacement::draft(0x141a98310, "bigfish_141a98310", fn_141a98310 as *const ()), // sub_141a98310
    Replacement::draft(0x141a983c0, "bigfish_141a983c0", fn_141a983c0 as *const ()), // sub_141a983c0
    Replacement::draft(0x141a98440, "bigfish_141a98440", fn_141a98440 as *const ()), // sub_141a98440
    Replacement::draft_throws(0x141a985d0, "bigfish_141a985d0", fn_141a985d0 as *const (), Throws::Discard), // sub_141a985d0
    Replacement::draft_throws(0x141a99470, "bigfish_141a99470", fn_141a99470 as *const (), Throws::Discard), // sub_141a99470
    Replacement::draft_throws(0x141a995d0, "bigfish_141a995d0", tpl_36293::<0xc, 0x8, 0x14241c190> as *const (), Throws::Discard), // sub_141a995d0
    Replacement::draft_throws(0x141a99650, "bigfish_141a99650", tpl_1148::<0x8, 0xc, 0x14241c190> as *const (), Throws::Discard), // sub_141a99650
    Replacement::draft_throws(0x141a99690, "bigfish_141a99690", fn_141a99690 as *const (), Throws::Discard), // sub_141a99690
    Replacement::draft_throws(0x141a99780, "bigfish_141a99780", fn_141a99780 as *const (), Throws::Discard), // sub_141a99780
    Replacement::draft_throws(0x141a99820, "bigfish_141a99820", fn_141a99820 as *const (), Throws::Caught), // sub_141a99820
    Replacement::draft_throws(0x141a9a120, "bigfish_141a9a120", fn_141a9a120 as *const (), Throws::Discard), // sub_141a9a120
    Replacement::draft_throws(0x141a9a3d0, "bigfish_141a9a3d0", fn_141a9a3d0 as *const (), Throws::Discard), // sub_141a9a3d0
    Replacement::draft(0x141a9a4e0, "bigfish_141a9a4e0", fn_141a9a4e0 as *const ()), // sub_141a9a4e0
    Replacement::draft_throws(0x141a9a700, "bigfish_141a9a700", fn_141a9a700 as *const (), Throws::Discard), // sub_141a9a700
    Replacement::draft_throws(0x141a9abc0, "bigfish_141a9abc0", fn_141a9abc0 as *const (), Throws::Caught), // sub_141a9abc0
    Replacement::draft_throws(0x141a9b050, "bigfish_141a9b050", fn_141a9b050 as *const (), Throws::Caught), // sub_141a9b050
    Replacement::draft(0x141a9b830, "bigfish_141a9b830", fn_141a9b830 as *const ()), // sub_141a9b830
    Replacement::draft(0x141a9b880, "bigfish_141a9b880", fn_141a9b880 as *const ()), // sub_141a9b880
    Replacement::draft_throws(0x141a9bb40, "bigfish_141a9bb40", fn_141a9bb40 as *const (), Throws::Discard), // sub_141a9bb40
    Replacement::draft(0x141a9c2d0, "bigfish_141a9c2d0", fn_141a9c2d0 as *const ()), // sub_141a9c2d0
    Replacement::draft(0x141a9c3f0, "bigfish_141a9c3f0", fn_141a9c3f0 as *const ()), // sub_141a9c3f0
    Replacement::draft(0x141a9c490, "bigfish_141a9c490", fn_141a9c490 as *const ()), // sub_141a9c490
    Replacement::draft(0x141a9c560, "bigfish_141a9c560", fn_141a9c560 as *const ()), // sub_141a9c560
    Replacement::draft(0x141a9c6f0, "bigfish_141a9c6f0", tpl_35328::<0x140550150> as *const ()), // sub_141a9c6f0
    Replacement::draft_throws(0x141a9cde0, "bigfish_141a9cde0", fn_141a9cde0 as *const (), Throws::Caught), // sub_141a9cde0
    Replacement::draft_throws(0x141a9dba0, "bigfish_141a9dba0", fn_141a9dba0 as *const (), Throws::Caught), // sub_141a9dba0
    Replacement::draft_throws(0x141a9de00, "bigfish_141a9de00", fn_141a9de00 as *const (), Throws::Discard), // sub_141a9de00
    Replacement::draft(0x141a9e270, "bigfish_141a9e270", fn_141a9e270 as *const ()), // sub_141a9e270
    Replacement::draft(0x141a9e3c0, "bigfish_141a9e3c0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cb1778, 0x158, 0x14020e8f0> as *const ()), // sub_141a9e3c0
    Replacement::draft_throws(0x141a9e4c0, "bigfish_141a9e4c0", fn_141a9e4c0 as *const (), Throws::Caught), // sub_141a9e4c0
    Replacement::draft(0x141a9e660, "bigfish_141a9e660", fn_141a9e660 as *const ()), // sub_141a9e660
    Replacement::draft_throws(0x141a9e6c0, "bigfish_141a9e6c0", fn_141a9e6c0 as *const (), Throws::Discard), // sub_141a9e6c0
    Replacement::draft(0x141a9ead0, "bigfish_141a9ead0", tpl_95133::<0x1431ab0e0, 0x1, 0x8, 0x14055b110> as *const ()), // sub_141a9ead0
    Replacement::draft(0x141a9ed70, "bigfish_141a9ed70", tpl_94875::<0x8, 0xc> as *const ()), // sub_141a9ed70
    Replacement::draft(0x141a9ee70, "bigfish_141a9ee70", tpl_34684::<0x140551a50> as *const ()), // sub_141a9ee70
    Replacement::draft_throws(0x141a9f370, "bigfish_141a9f370", fn_141a9f370 as *const (), Throws::Caught), // sub_141a9f370
    Replacement::draft_throws(0x141a9f7b0, "bigfish_141a9f7b0", fn_141a9f7b0 as *const (), Throws::Caught), // sub_141a9f7b0
    Replacement::draft(0x141a9fb30, "bigfish_141a9fb30", fn_141a9fb30 as *const ()), // sub_141a9fb30
    Replacement::draft(0x141a9fd30, "bigfish_141a9fd30", fn_141a9fd30 as *const ()), // sub_141a9fd30
    Replacement::draft(0x141aa0190, "bigfish_141aa0190", fn_141aa0190 as *const ()), // sub_141aa0190
    Replacement::draft(0x141aa0460, "bigfish_141aa0460", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cb218c, 0x158, 0x14020e8f0> as *const ()), // sub_141aa0460
    Replacement::draft_throws(0x141aa0560, "bigfish_141aa0560", fn_141aa0560 as *const (), Throws::Caught), // sub_141aa0560
    Replacement::draft(0x141aa08e0, "bigfish_141aa08e0", tpl_37078::<0x1426cdcb0, 0x1431a8378, 0x1431a8358, 0x140551e90> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa0930, "bigfish_141aa0930", tpl_37078::<0x1426cdcb0, 0x1431a8378, 0x1431a8358, 0x140551e90> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141aa0a20, "bigfish_141aa0a20", tpl_37079::<0x14387baf0, 0x14387bac8, 0x1401e9770, 0x8, 0x140556270, 0x1401e9590> as *const (), Throws::Discard), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141aa0a80, "bigfish_141aa0a80", tpl_37079::<0x14387baf0, 0x14387bac8, 0x1401e9770, 0x8, 0x140556270, 0x1401e9590> as *const (), Throws::Discard), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa0b20, "bigfish_141aa0b20", tpl_37080::<0x14387baf0, 0x14387bac8, 0x1401e9970, 0x8, 0x140555fb0, 0x1401e9860> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa0b80, "bigfish_141aa0b80", tpl_37080::<0x14387baf0, 0x14387bac8, 0x1401e9970, 0x8, 0x140555fb0, 0x1401e9860> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141aa0d20, "bigfish_141aa0d20", tpl_37081::<0x14387baf0, 0x14387bac8, 0x1401e9b60, 0x8, 0x140551c80, 0x1401e9a20> as *const (), Throws::Discard), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141aa0d80, "bigfish_141aa0d80", tpl_37081::<0x14387baf0, 0x14387bac8, 0x1401e9b60, 0x8, 0x140551c80, 0x1401e9a20> as *const (), Throws::Discard), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141aa0e20, "bigfish_141aa0e20", tpl_37081::<0x14387baf0, 0x14387bac8, 0x1401e9fa0, 0x8, 0x140551b50, 0x1401e9e70> as *const (), Throws::Discard), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141aa0e80, "bigfish_141aa0e80", tpl_37081::<0x14387baf0, 0x14387bac8, 0x1401e9fa0, 0x8, 0x140551b50, 0x1401e9e70> as *const (), Throws::Discard), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa0f20, "bigfish_141aa0f20", tpl_35199::<0x14387bac8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa0f80, "bigfish_141aa0f80", tpl_35199::<0x14387bac8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa1010, "bigfish_141aa1010", fn_141aa1010 as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa1070, "bigfish_141aa1070", tpl_37083::<0x18> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa1080, "bigfish_141aa1080", fn_141aa1080 as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa10b0, "bigfish_141aa10b0", tpl_37085::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x18, 0x142442c80> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141aa10f0, "bigfish_141aa10f0", tpl_37086::<0x18, 0x142442de0> as *const ()), // bigfish::component::CrowdNPC::ClassTypeInfoImplementation::bigfish::component::CrowdNPC::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141aa11d0, "unknown_141aa11d0", fn_141aa11d0 as *const (), Throws::Discard), // sub_141aa11d0
    Replacement::draft_throws(0x141aa1a90, "unknown_141aa1a90", fn_141aa1a90 as *const (), Throws::Discard), // sub_141aa1a90
    Replacement::draft_throws(0x141aa2310, "unknown_141aa2310", fn_141aa2310 as *const (), Throws::Discard), // sub_141aa2310
    Replacement::draft(0x141aa23a0, "unknown_141aa23a0", tpl_94810::<0x1431ad160, 0x8> as *const ()), // sub_141aa23a0
    Replacement::draft(0x141aa2510, "unknown_141aa2510", tpl_34613 as *const ()), // sub_141aa2510
    Replacement::draft(0x141aa27b0, "unknown_141aa27b0", tpl_1439::<0x8, 0x14055ccc0> as *const ()), // sub_141aa27b0
    Replacement::draft(0x141aa27f0, "unknown_141aa27f0", fn_141aa27f0 as *const ()), // sub_141aa27f0
    Replacement::draft(0x141aa28c0, "unknown_141aa28c0", fn_141aa28c0 as *const ()), // sub_141aa28c0
    Replacement::draft_throws(0x141aa2cb0, "unknown_141aa2cb0", fn_141aa2cb0 as *const (), Throws::Discard), // sub_141aa2cb0
    Replacement::draft(0x141aa2f10, "unknown_141aa2f10", tpl_34613 as *const ()), // sub_141aa2f10
    Replacement::draft(0x141aa2f30, "unknown_141aa2f30", tpl_34613 as *const ()), // sub_141aa2f30
    Replacement::draft(0x141aa2fc0, "unknown_141aa2fc0", tpl_94771::<0x1401ff5b0> as *const ()), // sub_141aa2fc0
    Replacement::draft(0x141aa3080, "unknown_141aa3080", tpl_34684::<0x1401f2bc0> as *const ()), // sub_141aa3080
    Replacement::draft(0x141aa30c0, "unknown_141aa30c0", tpl_34835::<0x8, 0x140261060, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141aa30c0
    Replacement::draft(0x141aa3140, "unknown_141aa3140", tpl_36359::<0x8, 0x140261060, 0x20, 0x3, 0x4, 0x14240aa00, 0x48, 0x14024d8b0, 0x10, 0x18, 0x14024ed60> as *const ()), // sub_141aa3140
    Replacement::draft(0x141aa31f0, "unknown_141aa31f0", tpl_34837::<0x140261060, 0x20, 0x14240aa00> as *const ()), // sub_141aa31f0
    Replacement::draft(0x141aa3240, "unknown_141aa3240", tpl_36360::<0x140261060, 0x20, 0x14240aa00, 0x48, 0x14046dc30> as *const ()), // sub_141aa3240
    Replacement::draft(0x141aa32a0, "unknown_141aa32a0", tpl_34840::<0x1403da6d0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141aa32a0
    Replacement::draft(0x141aa3310, "unknown_141aa3310", tpl_36361::<0x1403da6d0, 0x20, 0x3, 0x4, 0x8, 0x14240a730, 0x48, 0x50, 0x14024d8b0, 0x10, 0x18> as *const ()), // sub_141aa3310
    Replacement::draft_throws(0x141aa39c0, "unknown_141aa39c0", tpl_34846::<0x1401f3a20, 0x141aa3a40> as *const (), Throws::Discard), // sub_141aa39c0
    Replacement::draft_throws(0x141aa3a40, "unknown_141aa3a40", tpl_94651::<0x14025ca60, 0x14309c488, 0x142762bd0, 0x142762bc0, 0x142762bf0, 0x142762bf8, 0x140254aa0, 0x1424b16a0, 0x14025f260> as *const (), Throws::Discard), // sub_141aa3a40
    Replacement::draft(0x141aa3d10, "unknown_141aa3d10", tpl_36362::<0x50, 0x48, 0x19, 0x28, 0x10, 0x8, 0x1> as *const ()), // sub_141aa3d10
    Replacement::draft_throws(0x141aa3da0, "unknown_141aa3da0", tpl_36363::<0x48, 0x28, 0x10, 0x19, 0x8, 0x140466950> as *const (), Throws::Discard), // sub_141aa3da0
    Replacement::draft_throws(0x141aa3e60, "unknown_141aa3e60", fn_141aa3e60 as *const (), Throws::Caught), // sub_141aa3e60
    Replacement::draft_throws(0x141aa4060, "unknown_141aa4060", tpl_34895::<0x18, 0x141aa3a40, 0x10> as *const (), Throws::Discard), // sub_141aa4060
    Replacement::draft(0x141aa40a0, "unknown_141aa40a0", tpl_96237::<0x1403f7860, 0x1401e66f0> as *const ()), // sub_141aa40a0
    Replacement::draft_throws(0x141aa41e0, "unknown_141aa41e0", tpl_36364::<0x141aa3d10, 0x140469cd0> as *const (), Throws::Discard), // sub_141aa41e0
    Replacement::draft_throws(0x141aa42f0, "unknown_141aa42f0", tpl_36365::<0x140466950> as *const (), Throws::Discard), // sub_141aa42f0
    Replacement::draft_throws(0x141aa4440, "unknown_141aa4440", tpl_36366::<0x141aa3d10, 0x1401f3a20, 0x141aa3a40> as *const (), Throws::Discard), // sub_141aa4440
    Replacement::draft(0x141aa44c0, "unknown_141aa44c0", tpl_36367::<0x140246920, 0x20, 0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141aa44c0
    Replacement::draft_throws(0x141aa4540, "unknown_141aa4540", fn_141aa4540 as *const (), Throws::Caught), // sub_141aa4540
    Replacement::draft_throws(0x141aa4bb0, "unknown_141aa4bb0", fn_141aa4bb0 as *const (), Throws::Caught), // sub_141aa4bb0
    Replacement::draft_throws(0x141aa5c40, "unknown_141aa5c40", fn_141aa5c40 as *const (), Throws::Discard), // sub_141aa5c40
    Replacement::draft(0x141aa5e40, "unknown_141aa5e40", fn_141aa5e40 as *const ()), // sub_141aa5e40
    Replacement::draft(0x141aa5fa0, "unknown_141aa5fa0", fn_141aa5fa0 as *const ()), // sub_141aa5fa0
    Replacement::draft_throws(0x141aa6050, "unknown_141aa6050", fn_141aa6050 as *const (), Throws::Discard), // sub_141aa6050
    Replacement::draft_throws(0x141aa6360, "unknown_141aa6360", fn_141aa6360 as *const (), Throws::Discard), // sub_141aa6360
    Replacement::draft(0x141aa6940, "unknown_141aa6940", fn_141aa6940 as *const ()), // sub_141aa6940
    Replacement::draft(0x141aa6970, "unknown_141aa6970", tpl_36372 as *const ()), // sub_141aa6970
    Replacement::draft(0x141aa6990, "unknown_141aa6990", tpl_36383::<0x8, 0x14055f620, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141aa6990
    Replacement::draft(0x141aa6a70, "unknown_141aa6a70", fn_141aa6a70 as *const ()), // sub_141aa6a70
    Replacement::draft(0x141aa6ac0, "unknown_141aa6ac0", fn_141aa6ac0 as *const ()), // sub_141aa6ac0
    Replacement::draft(0x141aa6b10, "unknown_141aa6b10", fn_141aa6b10 as *const ()), // sub_141aa6b10
    Replacement::draft(0x141aa6bc0, "unknown_141aa6bc0", fn_141aa6bc0 as *const ()), // sub_141aa6bc0
    Replacement::draft(0x141aa6ec0, "unknown_141aa6ec0", tpl_1044::<0x14384c060> as *const ()), // sub_141aa6ec0
    Replacement::draft(0x141aa6ed0, "unknown_141aa6ed0", fn_141aa6ed0 as *const ()), // sub_141aa6ed0
    Replacement::draft(0x141aa6f60, "unknown_141aa6f60", fn_141aa6f60 as *const ()), // sub_141aa6f60
    Replacement::draft(0x141aa6fd0, "unknown_141aa6fd0", tpl_1044::<0x14384bf80> as *const ()), // sub_141aa6fd0
    Replacement::draft(0x141aa6fe0, "unknown_141aa6fe0", fn_141aa6fe0 as *const ()), // sub_141aa6fe0
    Replacement::draft(0x141aa7020, "unknown_141aa7020", fn_141aa7020 as *const ()), // sub_141aa7020
    Replacement::draft(0x141aa7060, "unknown_141aa7060", tpl_36135 as *const ()), // sub_141aa7060
    Replacement::draft(0x141aa7080, "unknown_141aa7080", fn_141aa7080 as *const ()), // sub_141aa7080
    Replacement::draft(0x141aa70c0, "unknown_141aa70c0", fn_141aa70c0 as *const ()), // sub_141aa70c0
    Replacement::draft_throws(0x141aa7130, "unknown_141aa7130", fn_141aa7130 as *const (), Throws::Caught), // sub_141aa7130
    Replacement::draft(0x141aa76f0, "unknown_141aa76f0", tpl_95133::<0x1431aeee0, 0x1, 0x8, 0x1401ff410> as *const ()), // sub_141aa76f0
    Replacement::draft_throws(0x141aa7b60, "unknown_141aa7b60", fn_141aa7b60 as *const (), Throws::Discard), // sub_141aa7b60
    Replacement::draft(0x141aa80a0, "unknown_141aa80a0", tpl_37107::<0x10> as *const ()), // sub_141aa80a0
    Replacement::draft_throws(0x141aa80e0, "unknown_141aa80e0", tpl_36293::<0xc, 0x8, 0x14241c190> as *const (), Throws::Discard), // sub_141aa80e0
    Replacement::draft_throws(0x141aa8160, "unknown_141aa8160", tpl_1148::<0x8, 0xc, 0x14241c190> as *const (), Throws::Discard), // sub_141aa8160
    Replacement::draft(0x141aa8240, "unknown_141aa8240", fn_141aa8240 as *const ()), // sub_141aa8240
    Replacement::draft(0x141aa8370, "unknown_141aa8370", fn_141aa8370 as *const ()), // sub_141aa8370
    Replacement::draft_throws(0x141aa84b0, "unknown_141aa84b0", fn_141aa84b0 as *const (), Throws::Caught), // sub_141aa84b0
    Replacement::draft(0x141aa9450, "unknown_141aa9450", fn_141aa9450 as *const ()), // sub_141aa9450
    Replacement::draft(0x141aa9480, "unknown_141aa9480", fn_141aa9480 as *const ()), // sub_141aa9480
    Replacement::draft(0x141aa94b0, "unknown_141aa94b0", fn_141aa94b0 as *const ()), // sub_141aa94b0
    Replacement::draft(0x141aa95a0, "unknown_141aa95a0", tpl_36401 as *const ()), // sub_141aa95a0
    Replacement::draft_throws(0x141aa95c0, "unknown_141aa95c0", fn_141aa95c0 as *const (), Throws::Discard), // sub_141aa95c0
    Replacement::draft(0x141aa9910, "unknown_141aa9910", tpl_34393 as *const ()), // sub_141aa9910
    Replacement::draft(0x141aaa0b0, "unknown_141aaa0b0", tpl_5401::<0x8, 0x1405661c0, 0x1405663c0> as *const ()), // sub_141aaa0b0
    Replacement::draft(0x141aaa130, "unknown_141aaa130", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cb301c, 0x158, 0x14020e8f0> as *const ()), // sub_141aaa130
    Replacement::draft(0x141aaa230, "unknown_141aaa230", fn_141aaa230 as *const ()), // sub_141aaa230
    Replacement::draft_throws(0x141aaa2d0, "unknown_141aaa2d0", fn_141aaa2d0 as *const (), Throws::Caught), // sub_141aaa2d0
    Replacement::draft_throws(0x141aaaab0, "unknown_141aaaab0", fn_141aaaab0 as *const (), Throws::Discard), // sub_141aaaab0
    Replacement::draft_throws(0x141aaae10, "unknown_141aaae10", fn_141aaae10 as *const (), Throws::Discard), // sub_141aaae10
    Replacement::draft_throws(0x141aaaed0, "unknown_141aaaed0", fn_141aaaed0 as *const (), Throws::Discard), // sub_141aaaed0
    Replacement::draft_throws(0x141aab850, "unknown_141aab850", fn_141aab850 as *const (), Throws::Discard), // sub_141aab850
    Replacement::draft_throws(0x141aaba30, "unknown_141aaba30", fn_141aaba30 as *const (), Throws::Discard), // sub_141aaba30
    Replacement::draft(0x141aabb30, "unknown_141aabb30", fn_141aabb30 as *const ()), // sub_141aabb30
    Replacement::draft_throws(0x141aabc50, "unknown_141aabc50", fn_141aabc50 as *const (), Throws::Discard), // sub_141aabc50
    Replacement::draft(0x141aac3b0, "unknown_141aac3b0", tpl_94770::<0x1431b29c0, 0x8, 0xc> as *const ()), // sub_141aac3b0
    Replacement::draft(0x141aac530, "unknown_141aac530", tpl_34605::<0x1431b2810> as *const ()), // sub_141aac530
    Replacement::draft(0x141aace10, "unknown_141aace10", fn_141aace10 as *const ()), // sub_141aace10
    Replacement::draft(0x141aacea0, "unknown_141aacea0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cb3888, 0x158, 0x14020e8f0> as *const ()), // sub_141aacea0
    Replacement::draft(0x141aacfa0, "unknown_141aacfa0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cb3734, 0x158, 0x14020e8f0> as *const ()), // sub_141aacfa0
    Replacement::draft(0x141aad0a0, "unknown_141aad0a0", fn_141aad0a0 as *const ()), // sub_141aad0a0
    Replacement::draft_throws(0x141aad130, "unknown_141aad130", fn_141aad130 as *const (), Throws::Caught), // sub_141aad130
    Replacement::draft(0x141aad5e0, "unknown_141aad5e0", fn_141aad5e0 as *const ()), // sub_141aad5e0
    Replacement::draft(0x141aad620, "unknown_141aad620", fn_141aad620 as *const ()), // sub_141aad620
    Replacement::draft(0x141aad660, "unknown_141aad660", fn_141aad660 as *const ()), // sub_141aad660
    Replacement::draft(0x141aadc90, "unknown_141aadc90", fn_141aadc90 as *const ()), // sub_141aadc90
    Replacement::draft(0x141aadf90, "unknown_141aadf90", fn_141aadf90 as *const ()), // sub_141aadf90
    Replacement::draft(0x141aae060, "unknown_141aae060", fn_141aae060 as *const ()), // sub_141aae060
    Replacement::draft_throws(0x141aae1b0, "unknown_141aae1b0", fn_141aae1b0 as *const (), Throws::Discard), // sub_141aae1b0
    Replacement::draft(0x141aae850, "unknown_141aae850", tpl_5426::<0x8, 0x14056ef60> as *const ()), // sub_141aae850
    Replacement::draft_throws(0x141aae8a0, "unknown_141aae8a0", fn_141aae8a0 as *const (), Throws::Discard), // sub_141aae8a0
    Replacement::draft(0x141aae940, "unknown_141aae940", fn_141aae940 as *const ()), // sub_141aae940
    Replacement::draft(0x141aaeac0, "unknown_141aaeac0", fn_141aaeac0 as *const ()), // sub_141aaeac0
    Replacement::draft(0x141aaeb70, "unknown_141aaeb70", fn_141aaeb70 as *const ()), // sub_141aaeb70
    Replacement::draft(0x141aaec60, "unknown_141aaec60", tpl_34611::<0x8> as *const ()), // sub_141aaec60
    Replacement::draft(0x141aaf600, "unknown_141aaf600", fn_141aaf600 as *const ()), // sub_141aaf600
    Replacement::draft(0x141aaf6f0, "unknown_141aaf6f0", fn_141aaf6f0 as *const ()), // sub_141aaf6f0
    Replacement::draft(0x141aaf7c0, "unknown_141aaf7c0", fn_141aaf7c0 as *const ()), // sub_141aaf7c0
    Replacement::draft(0x141ab0740, "unknown_141ab0740", fn_141ab0740 as *const ()), // sub_141ab0740
    Replacement::draft_throws(0x141ab0780, "unknown_141ab0780", fn_141ab0780 as *const (), Throws::Discard), // sub_141ab0780
    Replacement::draft(0x141ab0cb0, "unknown_141ab0cb0", tpl_35671::<0x8, 0x141ab1650> as *const ()), // sub_141ab0cb0
    Replacement::draft(0x141ab0dc0, "unknown_141ab0dc0", tpl_422::<0x8, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141ab0dc0
    Replacement::draft_throws(0x141ab0ef0, "unknown_141ab0ef0", fn_141ab0ef0 as *const (), Throws::Discard), // sub_141ab0ef0
    Replacement::draft(0x141ab0f20, "unknown_141ab0f20", tpl_895::<0x8> as *const ()), // sub_141ab0f20
    Replacement::draft(0x141ab11b0, "unknown_141ab11b0", tpl_35675::<0x8> as *const ()), // sub_141ab11b0
    Replacement::draft(0x141ab11d0, "unknown_141ab11d0", tpl_365::<0x141ab0dc0> as *const ()), // sub_141ab11d0
    Replacement::draft(0x141ab11e0, "unknown_141ab11e0", tpl_3548::<0x10, 0x19, 0x8> as *const ()), // sub_141ab11e0
    Replacement::draft(0x141ab1240, "unknown_141ab1240", tpl_3547::<0x10, 0x19, 0x8> as *const ()), // sub_141ab1240
    Replacement::draft(0x141ab12a0, "unknown_141ab12a0", tpl_63163::<0x141ab1240, 0x141ab11e0> as *const ()), // sub_141ab12a0
    Replacement::draft(0x141ab1640, "unknown_141ab1640", tpl_34443 as *const ()), // sub_141ab1640
    Replacement::draft(0x141ab1650, "unknown_141ab1650", tpl_1733::<0x50, 0x10, 0x18, 0xffffffffffffffc8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_141ab1650
    Replacement::draft(0x141ab16d0, "unknown_141ab16d0", fn_141ab16d0 as *const ()), // sub_141ab16d0
    Replacement::draft(0x141ab1750, "unknown_141ab1750", tpl_361 as *const ()), // sub_141ab1750
    Replacement::draft(0x141ab1760, "unknown_141ab1760", tpl_35677::<0x10, 0x19> as *const ()), // sub_141ab1760
    Replacement::draft(0x141ab1790, "unknown_141ab1790", tpl_24915 as *const ()), // sub_141ab1790
    Replacement::draft(0x141ab17a0, "unknown_141ab17a0", tpl_35678::<0x19> as *const ()), // sub_141ab17a0
    Replacement::draft(0x141ab17d0, "unknown_141ab17d0", tpl_361 as *const ()), // sub_141ab17d0
    Replacement::draft(0x141ab17e0, "unknown_141ab17e0", tpl_361 as *const ()), // sub_141ab17e0
    Replacement::draft(0x141ab17f0, "unknown_141ab17f0", tpl_361 as *const ()), // sub_141ab17f0
    Replacement::draft(0x141ab1800, "unknown_141ab1800", tpl_35685::<0x141ab1650, 0x8, 0x10, 0x18> as *const ()), // sub_141ab1800
    Replacement::draft(0x141ab1910, "unknown_141ab1910", tpl_3546::<0x8, 0x141ab1650, 0x10, 0x18> as *const ()), // sub_141ab1910
    Replacement::draft(0x141ab1950, "unknown_141ab1950", tpl_97086::<0x1405720a0, 0x1403e8060, 0x1405720f0> as *const ()), // sub_141ab1950
    Replacement::draft(0x141ab1ab0, "unknown_141ab1ab0", tpl_37137::<0x141ab1870> as *const ()), // sub_141ab1ab0
    Replacement::draft_throws(0x141ab1b20, "unknown_141ab1b20", tpl_37138::<0x142494c40, 0x142494dc0> as *const (), Throws::Discard), // sub_141ab1b20
    Replacement::draft(0x141ab1cb0, "unknown_141ab1cb0", tpl_5649::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060> as *const ()), // sub_141ab1cb0
    Replacement::draft(0x141ab1d10, "unknown_141ab1d10", fn_141ab1d10 as *const ()), // sub_141ab1d10
    Replacement::draft(0x141ab1d30, "unknown_141ab1d30", fn_141ab1d30 as *const ()), // sub_141ab1d30
    Replacement::draft(0x141ab1d50, "unknown_141ab1d50", tpl_3775::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060> as *const ()), // sub_141ab1d50
    Replacement::draft(0x141ab1dc0, "unknown_141ab1dc0", fn_141ab1dc0 as *const ()), // sub_141ab1dc0
    Replacement::draft(0x141ab1e20, "unknown_141ab1e20", tpl_3546::<0x8, 0x141ab1650, 0x10, 0x18> as *const ()), // sub_141ab1e20
    Replacement::draft(0x141ab1e60, "unknown_141ab1e60", tpl_97086::<0x1405720a0, 0x1403e8060, 0x1405720f0> as *const ()), // sub_141ab1e60
    Replacement::draft(0x141ab1fa0, "unknown_141ab1fa0", tpl_35682::<0x141ab1870> as *const ()), // sub_141ab1fa0
    Replacement::draft_throws(0x141ab1ff0, "unknown_141ab1ff0", tpl_37142::<0x8, 0x38, 0x141ab1b20> as *const (), Throws::Discard), // sub_141ab1ff0
    Replacement::draft(0x141ab2020, "unknown_141ab2020", tpl_37143::<0x10, 0x30, 0x20> as *const ()), // sub_141ab2020
    Replacement::draft_throws(0x141ab2040, "unknown_141ab2040", tpl_37144::<0x8, 0x142494c40, 0x142494dc0, 0x141ab1b20> as *const (), Throws::Discard), // sub_141ab2040
    Replacement::draft(0x141ab20b0, "unknown_141ab20b0", fn_141ab20b0 as *const ()), // sub_141ab20b0
    Replacement::draft(0x141ab2280, "unknown_141ab2280", fn_141ab2280 as *const ()), // sub_141ab2280
    Replacement::draft_throws(0x141ab23c0, "unknown_141ab23c0", fn_141ab23c0 as *const (), Throws::Caught), // sub_141ab23c0
    Replacement::draft(0x141ab26b0, "unknown_141ab26b0", fn_141ab26b0 as *const ()), // sub_141ab26b0
    Replacement::draft_throws(0x141ab2810, "unknown_141ab2810", fn_141ab2810 as *const (), Throws::Caught), // sub_141ab2810
    Replacement::draft(0x141ab2bd0, "unknown_141ab2bd0", tpl_5649::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060> as *const ()), // sub_141ab2bd0
    Replacement::draft(0x141ab2c30, "unknown_141ab2c30", fn_141ab2c30 as *const ()), // sub_141ab2c30
    Replacement::draft_throws(0x141ab2dd0, "unknown_141ab2dd0", fn_141ab2dd0 as *const (), Throws::Caught), // sub_141ab2dd0
    Replacement::draft(0x141ab3270, "unknown_141ab3270", fn_141ab3270 as *const ()), // sub_141ab3270
    Replacement::draft(0x141ab33a0, "unknown_141ab33a0", fn_141ab33a0 as *const ()), // sub_141ab33a0
    Replacement::draft(0x141ab3440, "unknown_141ab3440", fn_141ab3440 as *const ()), // sub_141ab3440
    Replacement::draft(0x141ab34e0, "unknown_141ab34e0", fn_141ab34e0 as *const ()), // sub_141ab34e0
    Replacement::draft(0x141ab35b0, "unknown_141ab35b0", fn_141ab35b0 as *const ()), // sub_141ab35b0
    Replacement::draft(0x141ab35f0, "unknown_141ab35f0", fn_141ab35f0 as *const ()), // sub_141ab35f0
    Replacement::draft_throws(0x141ab3630, "unknown_141ab3630", fn_141ab3630 as *const (), Throws::Caught), // sub_141ab3630
    Replacement::draft(0x141ab38d0, "unknown_141ab38d0", fn_141ab38d0 as *const ()), // sub_141ab38d0
    Replacement::draft(0x141ab38f0, "unknown_141ab38f0", tpl_37159::<0x20> as *const ()), // sub_141ab38f0
    Replacement::draft_throws(0x141ab3900, "unknown_141ab3900", fn_141ab3900 as *const (), Throws::Discard), // sub_141ab3900
    Replacement::draft_throws(0x141ab3b10, "unknown_141ab3b10", fn_141ab3b10 as *const (), Throws::Caught), // sub_141ab3b10
    Replacement::draft(0x141ab3e50, "unknown_141ab3e50", fn_141ab3e50 as *const ()), // sub_141ab3e50
    Replacement::draft_throws(0x141ab3f40, "unknown_141ab3f40", fn_141ab3f40 as *const (), Throws::Caught), // sub_141ab3f40
    Replacement::draft(0x141ab41e0, "unknown_141ab41e0", fn_141ab41e0 as *const ()), // sub_141ab41e0
    Replacement::draft(0x141ab4370, "unknown_141ab4370", fn_141ab4370 as *const ()), // sub_141ab4370
    Replacement::draft(0x141ab4530, "unknown_141ab4530", fn_141ab4530 as *const ()), // sub_141ab4530
    Replacement::draft(0x141ab4710, "unknown_141ab4710", fn_141ab4710 as *const ()), // sub_141ab4710
    Replacement::draft(0x141ab4740, "unknown_141ab4740", fn_141ab4740 as *const ()), // sub_141ab4740
    Replacement::draft(0x141ab4890, "unknown_141ab4890", fn_141ab4890 as *const ()), // sub_141ab4890
    Replacement::draft(0x141ab5820, "unknown_141ab5820", fn_141ab5820 as *const ()), // sub_141ab5820
    Replacement::draft(0x141ab5940, "unknown_141ab5940", fn_141ab5940 as *const ()), // sub_141ab5940
    Replacement::draft(0x141ab5e30, "unknown_141ab5e30", fn_141ab5e30 as *const ()), // sub_141ab5e30
    Replacement::draft_throws(0x141ab5eb0, "unknown_141ab5eb0", fn_141ab5eb0 as *const (), Throws::Discard), // sub_141ab5eb0
    Replacement::draft(0x141ab6380, "unknown_141ab6380", tpl_34750::<0x48> as *const ()), // sub_141ab6380
    Replacement::draft(0x141ab63a0, "unknown_141ab63a0", fn_141ab63a0 as *const ()), // sub_141ab63a0
    Replacement::draft(0x141ab63d0, "unknown_141ab63d0", fn_141ab63d0 as *const ()), // sub_141ab63d0
    Replacement::draft(0x141ab6410, "unknown_141ab6410", tpl_34750::<0x68> as *const ()), // sub_141ab6410
    Replacement::draft(0x141ab6420, "unknown_141ab6420", tpl_34731::<0x64> as *const ()), // sub_141ab6420
    Replacement::draft(0x141ab6440, "unknown_141ab6440", tpl_37173::<0x4c> as *const ()), // sub_141ab6440
    Replacement::draft(0x141ab6460, "unknown_141ab6460", tpl_34731::<0x8> as *const ()), // sub_141ab6460
    Replacement::draft(0x141ab6480, "unknown_141ab6480", tpl_34731::<0x4> as *const ()), // sub_141ab6480
    Replacement::draft(0x141ab64a0, "unknown_141ab64a0", tpl_34741::<0x1c, 0x1434268d0> as *const ()), // sub_141ab64a0
    Replacement::draft(0x141ab64c0, "unknown_141ab64c0", tpl_34741::<0x60, 0x1434268d0> as *const ()), // sub_141ab64c0
    Replacement::draft(0x141ab64e0, "unknown_141ab64e0", tpl_34741::<0x5c, 0x1434268d0> as *const ()), // sub_141ab64e0
    Replacement::draft(0x141ab6500, "unknown_141ab6500", tpl_34741::<0x58, 0x1434268d0> as *const ()), // sub_141ab6500
    Replacement::draft(0x141ab6520, "unknown_141ab6520", tpl_34731::<0xc> as *const ()), // sub_141ab6520
    Replacement::draft(0x141ab6540, "unknown_141ab6540", tpl_34741::<0x2c, 0x1434268d0> as *const ()), // sub_141ab6540
    Replacement::draft(0x141ab6560, "unknown_141ab6560", tpl_34741::<0x54, 0x1434268d0> as *const ()), // sub_141ab6560
    Replacement::draft(0x141ab6580, "unknown_141ab6580", tpl_34741::<0x20, 0x1434268d0> as *const ()), // sub_141ab6580
    Replacement::draft(0x141ab65a0, "unknown_141ab65a0", tpl_34731::<0x18> as *const ()), // sub_141ab65a0
    Replacement::draft(0x141ab65c0, "unknown_141ab65c0", tpl_34731::<0x14> as *const ()), // sub_141ab65c0
    Replacement::draft(0x141ab65e0, "unknown_141ab65e0", tpl_34747::<0x44> as *const ()), // sub_141ab65e0
    Replacement::draft(0x141ab65f0, "unknown_141ab65f0", tpl_34747::<0x28> as *const ()), // sub_141ab65f0
    Replacement::draft(0x141ab6600, "unknown_141ab6600", tpl_34747::<0x24> as *const ()), // sub_141ab6600
    Replacement::draft(0x141ab6640, "unknown_141ab6640", fn_141ab6640 as *const ()), // sub_141ab6640
    Replacement::draft(0x141ab6660, "unknown_141ab6660", fn_141ab6660 as *const ()), // sub_141ab6660
    Replacement::draft(0x141ab66e0, "unknown_141ab66e0", fn_141ab66e0 as *const ()), // sub_141ab66e0
    Replacement::draft(0x141ab67b0, "unknown_141ab67b0", fn_141ab67b0 as *const ()), // sub_141ab67b0
    Replacement::draft(0x141ab67f0, "unknown_141ab67f0", tpl_37178::<0x40, 0x1434268d0, 0x60> as *const ()), // sub_141ab67f0
    Replacement::draft(0x141ab6840, "unknown_141ab6840", tpl_37179::<0x40, 0x44> as *const ()), // sub_141ab6840
    Replacement::draft(0x141ab6880, "unknown_141ab6880", tpl_37180::<0x40, 0x68> as *const ()), // sub_141ab6880
    Replacement::draft(0x141ab68c0, "unknown_141ab68c0", tpl_37178::<0x40, 0x1434268d0, 0x58> as *const ()), // sub_141ab68c0
    Replacement::draft(0x141ab6910, "unknown_141ab6910", tpl_37181::<0x40, 0x1434268d0, 0x54> as *const ()), // sub_141ab6910
    Replacement::draft(0x141ab6960, "unknown_141ab6960", tpl_37178::<0x40, 0x1434268d0, 0x5c> as *const ()), // sub_141ab6960
    Replacement::draft(0x141ab69b0, "unknown_141ab69b0", fn_141ab69b0 as *const ()), // sub_141ab69b0
    Replacement::draft(0x141ab69e0, "unknown_141ab69e0", tpl_37183::<0x40, 0xc> as *const ()), // sub_141ab69e0
    Replacement::draft(0x141ab6a20, "unknown_141ab6a20", tpl_37179::<0x40, 0x24> as *const ()), // sub_141ab6a20
    Replacement::draft(0x141ab6a60, "unknown_141ab6a60", tpl_37179::<0x40, 0x10> as *const ()), // sub_141ab6a60
    Replacement::draft(0x141ab6aa0, "unknown_141ab6aa0", fn_141ab6aa0 as *const ()), // sub_141ab6aa0
    Replacement::draft(0x141ab6af0, "unknown_141ab6af0", tpl_37179::<0x40, 0x28> as *const ()), // sub_141ab6af0
    Replacement::draft(0x141ab6b30, "unknown_141ab6b30", tpl_37181::<0x40, 0x1434268d0, 0x1c> as *const ()), // sub_141ab6b30
    Replacement::draft(0x141ab6b80, "unknown_141ab6b80", tpl_37181::<0x40, 0x1434268d0, 0x20> as *const ()), // sub_141ab6b80
    Replacement::draft(0x141ab6bd0, "unknown_141ab6bd0", tpl_37181::<0x40, 0x1434268d0, 0x2c> as *const ()), // sub_141ab6bd0
    Replacement::draft(0x141ab6c20, "unknown_141ab6c20", tpl_37183::<0x40, 0x64> as *const ()), // sub_141ab6c20
    Replacement::draft(0x141ab6c60, "unknown_141ab6c60", tpl_37183::<0x40, 0x14> as *const ()), // sub_141ab6c60
    Replacement::draft(0x141ab6ca0, "unknown_141ab6ca0", tpl_37183::<0x40, 0x18> as *const ()), // sub_141ab6ca0
    Replacement::draft(0x141ab6ce0, "unknown_141ab6ce0", fn_141ab6ce0 as *const ()), // sub_141ab6ce0
    Replacement::draft(0x141ab6d10, "unknown_141ab6d10", tpl_37183::<0x40, 0x4> as *const ()), // sub_141ab6d10
    Replacement::draft(0x141ab6d50, "unknown_141ab6d50", tpl_37183::<0x40, 0x8> as *const ()), // sub_141ab6d50
    Replacement::draft(0x141ab6d90, "unknown_141ab6d90", fn_141ab6d90 as *const ()), // sub_141ab6d90
    Replacement::draft(0x141ab6dd0, "unknown_141ab6dd0", fn_141ab6dd0 as *const ()), // sub_141ab6dd0
    Replacement::draft(0x141ab6fc0, "unknown_141ab6fc0", tpl_37180::<0x40, 0x48> as *const ()), // sub_141ab6fc0
    Replacement::draft_throws(0x141ab7000, "unknown_141ab7000", fn_141ab7000 as *const (), Throws::Discard), // sub_141ab7000
    Replacement::draft(0x141ab7220, "unknown_141ab7220", tpl_34667::<0x20> as *const ()), // sub_141ab7220
    Replacement::draft(0x141ab72c0, "unknown_141ab72c0", fn_141ab72c0 as *const ()), // sub_141ab72c0
    Replacement::draft(0x141ab7360, "unknown_141ab7360", fn_141ab7360 as *const ()), // sub_141ab7360
    Replacement::draft(0x141ab7440, "unknown_141ab7440", fn_141ab7440 as *const ()), // sub_141ab7440
    Replacement::draft(0x141ab74d0, "unknown_141ab74d0", tpl_34611::<0x8> as *const ()), // sub_141ab74d0
    Replacement::draft(0x141ab7880, "unknown_141ab7880", tpl_1439::<0x8, 0x140575850> as *const ()), // sub_141ab7880
    Replacement::draft_throws(0x141ab78c0, "unknown_141ab78c0", fn_141ab78c0 as *const (), Throws::Discard), // sub_141ab78c0
    Replacement::draft(0x141ab7b40, "unknown_141ab7b40", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cb4008, 0x158, 0x14020e8f0> as *const ()), // sub_141ab7b40
    Replacement::draft(0x141ab7c40, "unknown_141ab7c40", fn_141ab7c40 as *const ()), // sub_141ab7c40
    Replacement::draft_throws(0x141ab7cf0, "unknown_141ab7cf0", tpl_37191::<0x14387f50c, 0x140575a50, 0x1402116e0, 0x1427692e0, 0x1410d6ca0, 0x140575a80, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x140575d60, 0x143823788, 0x1427692d0, 0x140575210, 0x1438234b0> as *const (), Throws::Discard), // sub_141ab7cf0
    Replacement::draft(0x141ab82f0, "unknown_141ab82f0", tpl_34694::<0x10> as *const ()), // sub_141ab82f0
    Replacement::draft(0x141ab8310, "unknown_141ab8310", tpl_34694::<0x10> as *const ()), // sub_141ab8310
    Replacement::draft_throws(0x141ab8510, "unknown_141ab8510", tpl_1331::<0x8, 0x140583040, 0x14384c560, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141ab8510
    Replacement::draft(0x141ab85c0, "unknown_141ab85c0", tpl_37192::<0x10, 0x40, 0x8, 0x18, 0x143c90164, 0x1806c, 0x18058, 0x142489ee0, 0xc03> as *const ()), // sub_141ab85c0
    Replacement::draft_throws(0x141ab8730, "unknown_141ab8730", tpl_1330::<0x8, 0x1405825e0, 0x14384bfd8, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_141ab8730
    Replacement::draft_throws(0x141ab8810, "unknown_141ab8810", tpl_1330::<0x8, 0x140582490, 0x14384bd0c, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_141ab8810
    Replacement::draft(0x141ab9860, "unknown_141ab9860", tpl_62975::<0x10, 0x18, 0x14384bfdc, 0x18068, 0x18050, 0x142489ee0, 0x1806c, 0x143c90164, 0x18058, 0xc03, 0x20, 0x38> as *const ()), // sub_141ab9860
    Replacement::draft(0x141ab9940, "unknown_141ab9940", fn_141ab9940 as *const ()), // sub_141ab9940
    Replacement::draft(0x141ab9a90, "unknown_141ab9a90", tpl_34703::<0x10, 0x8, 0x18, 0x28, 0x30, 0x48, 0x20, 0x10048, 0x141ab9860> as *const ()), // sub_141ab9a90
    Replacement::draft(0x141ab9b00, "unknown_141ab9b00", tpl_34704::<0x10, 0x18, 0x8, 0x48, 0x10048, 0x20, 0x28, 0x30, 0x141ab9860> as *const ()), // sub_141ab9b00
    Replacement::draft(0x141ab9be0, "unknown_141ab9be0", tpl_34703::<0x10, 0x8, 0x18, 0x28, 0x30, 0x48, 0x20, 0x10048, 0x141ab9940> as *const ()), // sub_141ab9be0
    Replacement::draft(0x141ab9c50, "unknown_141ab9c50", tpl_34704::<0x10, 0x18, 0x8, 0x48, 0x10048, 0x20, 0x28, 0x30, 0x141ab9940> as *const ()), // sub_141ab9c50
    Replacement::draft(0x141ab9cf0, "unknown_141ab9cf0", fn_141ab9cf0 as *const ()), // sub_141ab9cf0
    Replacement::draft(0x141ab9ee0, "unknown_141ab9ee0", fn_141ab9ee0 as *const ()), // sub_141ab9ee0
    Replacement::draft(0x141aba090, "unknown_141aba090", fn_141aba090 as *const ()), // sub_141aba090
    Replacement::draft(0x141aba140, "unknown_141aba140", fn_141aba140 as *const ()), // sub_141aba140
    Replacement::draft(0x141aba6d0, "unknown_141aba6d0", fn_141aba6d0 as *const ()), // sub_141aba6d0
    Replacement::draft_throws(0x141abb7a0, "unknown_141abb7a0", fn_141abb7a0 as *const (), Throws::Discard), // sub_141abb7a0
    Replacement::draft(0x141abbdc0, "unknown_141abbdc0", fn_141abbdc0 as *const ()), // sub_141abbdc0
    Replacement::draft(0x141abc3a0, "unknown_141abc3a0", fn_141abc3a0 as *const ()), // sub_141abc3a0
    Replacement::draft(0x141abc7c0, "unknown_141abc7c0", tpl_34750::<0x89> as *const ()), // sub_141abc7c0
    Replacement::draft(0x141abc7e0, "unknown_141abc7e0", tpl_37201::<0x14240aa00, 0x14057b610> as *const ()), // sub_141abc7e0
    Replacement::draft(0x141abc850, "unknown_141abc850", tpl_34750::<0x88> as *const ()), // sub_141abc850
    Replacement::draft(0x141abc870, "unknown_141abc870", tpl_34741::<0x80, 0x1434268d0> as *const ()), // sub_141abc870
    Replacement::draft(0x141abc890, "unknown_141abc890", tpl_37202::<0x14240aa00, 0x14057b300> as *const ()), // sub_141abc890
    Replacement::draft(0x141abc900, "unknown_141abc900", tpl_34731::<0x7c> as *const ()), // sub_141abc900
    Replacement::draft(0x141abc920, "unknown_141abc920", tpl_37202::<0x14240aa00, 0x14057b180> as *const ()), // sub_141abc920
    Replacement::draft(0x141abc990, "unknown_141abc990", tpl_37173::<0x78> as *const ()), // sub_141abc990
    Replacement::draft(0x141abc9b0, "unknown_141abc9b0", tpl_37203::<0x14240aa00, 0x1434268d0, 0x14057b000> as *const ()), // sub_141abc9b0
    Replacement::draft(0x141abca20, "unknown_141abca20", tpl_37173::<0x74> as *const ()), // sub_141abca20
    Replacement::draft(0x141abca40, "unknown_141abca40", tpl_37203::<0x14240aa00, 0x1434268d0, 0x14057ae80> as *const ()), // sub_141abca40
    Replacement::draft(0x141abcab0, "unknown_141abcab0", tpl_37173::<0x70> as *const ()), // sub_141abcab0
    Replacement::draft(0x141abcad0, "unknown_141abcad0", tpl_37203::<0x14240aa00, 0x1434268d0, 0x14057ad00> as *const ()), // sub_141abcad0
    Replacement::draft(0x141abcb40, "unknown_141abcb40", tpl_34731::<0x6c> as *const ()), // sub_141abcb40
    Replacement::draft(0x141abcb60, "unknown_141abcb60", tpl_37202::<0x14240aa00, 0x14057ab80> as *const ()), // sub_141abcb60
    Replacement::draft(0x141abcbd0, "unknown_141abcbd0", tpl_34741::<0x68, 0x1434268d0> as *const ()), // sub_141abcbd0
    Replacement::draft(0x141abcbf0, "unknown_141abcbf0", tpl_37202::<0x14240aa00, 0x14057aa00> as *const ()), // sub_141abcbf0
    Replacement::draft(0x141abcc60, "unknown_141abcc60", tpl_34741::<0x64, 0x1434268d0> as *const ()), // sub_141abcc60
    Replacement::draft(0x141abcc80, "unknown_141abcc80", tpl_37202::<0x14240aa00, 0x14057a880> as *const ()), // sub_141abcc80
    Replacement::draft(0x141abccf0, "unknown_141abccf0", tpl_34731::<0x60> as *const ()), // sub_141abccf0
    Replacement::draft(0x141abcd10, "unknown_141abcd10", tpl_37202::<0x14240aa00, 0x14057a700> as *const ()), // sub_141abcd10
    Replacement::draft(0x141abcd80, "unknown_141abcd80", tpl_34731::<0x5c> as *const ()), // sub_141abcd80
    Replacement::draft(0x141abcda0, "unknown_141abcda0", tpl_37202::<0x14240aa00, 0x14057a580> as *const ()), // sub_141abcda0
    Replacement::draft(0x141abce10, "unknown_141abce10", tpl_34731::<0x58> as *const ()), // sub_141abce10
    Replacement::draft(0x141abce30, "unknown_141abce30", tpl_37202::<0x14240aa00, 0x14057a400> as *const ()), // sub_141abce30
    Replacement::draft(0x141abcea0, "unknown_141abcea0", tpl_34741::<0x54, 0x1434268d0> as *const ()), // sub_141abcea0
    Replacement::draft(0x141abcec0, "unknown_141abcec0", tpl_37202::<0x14240aa00, 0x14057a280> as *const ()), // sub_141abcec0
    Replacement::draft(0x141abcf30, "unknown_141abcf30", tpl_34731::<0x50> as *const ()), // sub_141abcf30
    Replacement::draft(0x141abcf50, "unknown_141abcf50", tpl_37202::<0x14240aa00, 0x14057a100> as *const ()), // sub_141abcf50
    Replacement::draft(0x141abcfc0, "unknown_141abcfc0", tpl_34741::<0x4c, 0x1434268d0> as *const ()), // sub_141abcfc0
    Replacement::draft(0x141abcfe0, "unknown_141abcfe0", tpl_37202::<0x14240aa00, 0x140579f80> as *const ()), // sub_141abcfe0
    Replacement::draft(0x141abd050, "unknown_141abd050", tpl_34731::<0x48> as *const ()), // sub_141abd050
    Replacement::draft(0x141abd070, "unknown_141abd070", tpl_37202::<0x14240aa00, 0x140579e00> as *const ()), // sub_141abd070
    Replacement::draft(0x141abd0e0, "unknown_141abd0e0", tpl_34731::<0x44> as *const ()), // sub_141abd0e0
    Replacement::draft(0x141abd100, "unknown_141abd100", tpl_37202::<0x14240aa00, 0x140579c80> as *const ()), // sub_141abd100
    Replacement::draft(0x141abd170, "unknown_141abd170", tpl_34731::<0x40> as *const ()), // sub_141abd170
    Replacement::draft(0x141abd190, "unknown_141abd190", tpl_37202::<0x14240aa00, 0x140579b00> as *const ()), // sub_141abd190
    Replacement::draft(0x141abd200, "unknown_141abd200", tpl_34731::<0x3c> as *const ()), // sub_141abd200
    Replacement::draft(0x141abd220, "unknown_141abd220", tpl_37202::<0x14240aa00, 0x140579980> as *const ()), // sub_141abd220
    Replacement::draft(0x141abd290, "unknown_141abd290", tpl_34741::<0x38, 0x1434268d0> as *const ()), // sub_141abd290
    Replacement::draft(0x141abd2b0, "unknown_141abd2b0", tpl_37202::<0x14240aa00, 0x140579800> as *const ()), // sub_141abd2b0
    Replacement::draft(0x141abd320, "unknown_141abd320", tpl_34741::<0x34, 0x1434268d0> as *const ()), // sub_141abd320
    Replacement::draft(0x141abd340, "unknown_141abd340", tpl_37202::<0x14240aa00, 0x140579680> as *const ()), // sub_141abd340
    Replacement::draft(0x141abd3b0, "unknown_141abd3b0", tpl_34731::<0x30> as *const ()), // sub_141abd3b0
    Replacement::draft(0x141abd3d0, "unknown_141abd3d0", tpl_34731::<0x2c> as *const ()), // sub_141abd3d0
    Replacement::draft(0x141abd3f0, "unknown_141abd3f0", tpl_34731::<0x28> as *const ()), // sub_141abd3f0
    Replacement::draft(0x141abd410, "unknown_141abd410", tpl_37202::<0x14240aa00, 0x140579200> as *const ()), // sub_141abd410
    Replacement::draft(0x141abd480, "unknown_141abd480", tpl_37173::<0x84> as *const ()), // sub_141abd480
    Replacement::draft(0x141abd4b0, "unknown_141abd4b0", fn_141abd4b0 as *const ()), // sub_141abd4b0
    Replacement::draft(0x141abd560, "unknown_141abd560", tpl_34747::<0x24> as *const ()), // sub_141abd560
    Replacement::draft(0x141abd570, "unknown_141abd570", tpl_37201::<0x14240aa00, 0x140578f00> as *const ()), // sub_141abd570
    Replacement::draft(0x141abd5e0, "unknown_141abd5e0", tpl_34747::<0x20> as *const ()), // sub_141abd5e0
    Replacement::draft(0x141abd5f0, "unknown_141abd5f0", tpl_37201::<0x14240aa00, 0x140578d80> as *const ()), // sub_141abd5f0
    Replacement::draft(0x141abd660, "unknown_141abd660", tpl_34747::<0x1c> as *const ()), // sub_141abd660
    Replacement::draft(0x141abd670, "unknown_141abd670", tpl_37201::<0x14240aa00, 0x140578c00> as *const ()), // sub_141abd670
    Replacement::draft(0x141abd6e0, "unknown_141abd6e0", tpl_34747::<0x18> as *const ()), // sub_141abd6e0
    Replacement::draft(0x141abd6f0, "unknown_141abd6f0", tpl_37201::<0x14240aa00, 0x140578a80> as *const ()), // sub_141abd6f0
    Replacement::draft(0x141abd770, "unknown_141abd770", tpl_37201::<0x14240aa00, 0x140578900> as *const ()), // sub_141abd770
    Replacement::draft(0x141abd7f0, "unknown_141abd7f0", tpl_37201::<0x14240aa00, 0x140578780> as *const ()), // sub_141abd7f0
    Replacement::draft(0x141abd860, "unknown_141abd860", tpl_34747::<0xc> as *const ()), // sub_141abd860
    Replacement::draft(0x141abd870, "unknown_141abd870", tpl_37201::<0x14240aa00, 0x140578600> as *const ()), // sub_141abd870
    Replacement::draft(0x141abd8e0, "unknown_141abd8e0", tpl_34747::<0x8> as *const ()), // sub_141abd8e0
    Replacement::draft(0x141abd8f0, "unknown_141abd8f0", tpl_37201::<0x14240aa00, 0x140578480> as *const ()), // sub_141abd8f0
    Replacement::draft(0x141abd960, "unknown_141abd960", tpl_34747::<0x4> as *const ()), // sub_141abd960
    Replacement::draft(0x141abd970, "unknown_141abd970", tpl_37201::<0x14240aa00, 0x140578300> as *const ()), // sub_141abd970
    Replacement::draft(0x141abd9f0, "unknown_141abd9f0", tpl_37205::<0x1426d01e0> as *const ()), // sub_141abd9f0
    Replacement::draft(0x141abdb30, "unknown_141abdb30", fn_141abdb30 as *const ()), // sub_141abdb30
    Replacement::draft(0x141abde20, "unknown_141abde20", fn_141abde20 as *const ()), // sub_141abde20
    Replacement::draft(0x141abdf60, "unknown_141abdf60", fn_141abdf60 as *const ()), // sub_141abdf60
    Replacement::draft(0x141abe0b0, "unknown_141abe0b0", tpl_37203::<0x14240aa00, 0x1434268d0, 0x140579080> as *const ()), // sub_141abe0b0
    Replacement::draft_throws(0x141abe120, "unknown_141abe120", fn_141abe120 as *const (), Throws::Discard), // sub_141abe120
    Replacement::draft(0x141abf780, "unknown_141abf780", tpl_37202::<0x14240aa00, 0x140579380> as *const ()), // sub_141abf780
    Replacement::draft(0x141abf7f0, "unknown_141abf7f0", tpl_37202::<0x14240aa00, 0x140579500> as *const ()), // sub_141abf7f0
    Replacement::draft_throws(0x141abf860, "unknown_141abf860", fn_141abf860 as *const (), Throws::Discard), // sub_141abf860
    Replacement::draft(0x141abf9e0, "unknown_141abf9e0", tpl_37201::<0x14240aa00, 0x140578190> as *const ()), // sub_141abf9e0
    Replacement::draft(0x141abfa50, "unknown_141abfa50", tpl_37201::<0x14240aa00, 0x14057b490> as *const ()), // sub_141abfa50
    Replacement::draft(0x141abfac0, "unknown_141abfac0", tpl_37205::<0x1426d01e0> as *const ()), // sub_141abfac0
    Replacement::draft(0x141abfc00, "unknown_141abfc00", fn_141abfc00 as *const ()), // sub_141abfc00
    Replacement::draft_throws(0x141abfc40, "unknown_141abfc40", fn_141abfc40 as *const (), Throws::Discard), // sub_141abfc40
    Replacement::draft(0x141ac01d0, "unknown_141ac01d0", tpl_35658::<0x8, 0x141ac0df0, 0x10, 0x18> as *const ()), // sub_141ac01d0
    Replacement::draft(0x141ac02d0, "unknown_141ac02d0", tpl_35677::<0x10, 0x19> as *const ()), // sub_141ac02d0
    Replacement::draft(0x141ac0330, "unknown_141ac0330", tpl_35661::<0x8, 0x141ac0df0, 0x10, 0x18, 0x140588570> as *const ()), // sub_141ac0330
    Replacement::draft(0x141ac0400, "unknown_141ac0400", tpl_34393 as *const ()), // sub_141ac0400
    Replacement::draft(0x141ac0490, "unknown_141ac0490", fn_141ac0490 as *const ()), // sub_141ac0490
    Replacement::draft(0x141ac0680, "unknown_141ac0680", fn_141ac0680 as *const ()), // sub_141ac0680
    Replacement::draft(0x141ac0730, "unknown_141ac0730", tpl_34667::<0x20> as *const ()), // sub_141ac0730
    Replacement::draft(0x141ac0780, "unknown_141ac0780", tpl_3775::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060> as *const ()), // sub_141ac0780
    Replacement::draft(0x141ac0800, "unknown_141ac0800", tpl_63557::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060, 0x140588570> as *const ()), // sub_141ac0800
    Replacement::draft(0x141ac0890, "unknown_141ac0890", tpl_63557::<0x8, 0x19, 0x10, 0x1405720a0, 0x1403e8060, 0x140588570> as *const ()), // sub_141ac0890
    Replacement::draft(0x141ac0920, "unknown_141ac0920", tpl_35665::<0x8> as *const ()), // sub_141ac0920
    Replacement::draft(0x141ac0960, "unknown_141ac0960", tpl_35666::<0x8, 0x141ac0df0, 0x10, 0x18> as *const ()), // sub_141ac0960
    Replacement::draft(0x141ac09c0, "unknown_141ac09c0", tpl_3604::<0x8, 0x141ac0df0, 0x10, 0x18> as *const ()), // sub_141ac09c0
    Replacement::draft(0x141ac0b50, "unknown_141ac0b50", tpl_35671::<0x8, 0x141ac0df0> as *const ()), // sub_141ac0b50
    Replacement::draft(0x141ac0b80, "unknown_141ac0b80", tpl_37213 as *const ()), // sub_141ac0b80
    Replacement::draft(0x141ac0c30, "unknown_141ac0c30", tpl_37213 as *const ()), // sub_141ac0c30
    Replacement::draft(0x141ac0c50, "unknown_141ac0c50", tpl_361 as *const ()), // sub_141ac0c50
    Replacement::draft(0x141ac0c60, "unknown_141ac0c60", tpl_37213 as *const ()), // sub_141ac0c60
    Replacement::draft(0x141ac0c80, "unknown_141ac0c80", tpl_3548::<0x10, 0x19, 0x8> as *const ()), // sub_141ac0c80
    Replacement::draft(0x141ac0ce0, "unknown_141ac0ce0", tpl_3547::<0x10, 0x19, 0x8> as *const ()), // sub_141ac0ce0
    Replacement::draft(0x141ac0d40, "unknown_141ac0d40", tpl_422::<0x8, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141ac0d40
    Replacement::draft(0x141ac0de0, "unknown_141ac0de0", tpl_365::<0x141ac0d40> as *const ()), // sub_141ac0de0
    Replacement::draft(0x141ac0df0, "unknown_141ac0df0", tpl_1494::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_141ac0df0
    Replacement::draft_throws(0x141ac0e70, "unknown_141ac0e70", fn_141ac0e70 as *const (), Throws::Discard), // sub_141ac0e70
    Replacement::draft(0x141ac1110, "unknown_141ac1110", tpl_35675::<0x8> as *const ()), // sub_141ac1110
    Replacement::draft(0x141ac1130, "unknown_141ac1130", tpl_365::<0x141ac0d40> as *const ()), // sub_141ac1130
    Replacement::draft(0x141ac1140, "unknown_141ac1140", tpl_34318 as *const ()), // sub_141ac1140
    Replacement::draft(0x141ac1160, "unknown_141ac1160", tpl_34319::<0x143c9d1c4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac1160
    Replacement::draft(0x141ac11c0, "unknown_141ac11c0", tpl_34318 as *const ()), // sub_141ac11c0
    Replacement::draft(0x141ac11e0, "unknown_141ac11e0", tpl_34319::<0x143c98134, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac11e0
    Replacement::draft(0x141ac1240, "unknown_141ac1240", tpl_34318 as *const ()), // sub_141ac1240
    Replacement::draft(0x141ac1260, "unknown_141ac1260", tpl_34319::<0x143c91064, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac1260
    Replacement::draft(0x141ac12c0, "unknown_141ac12c0", tpl_34318 as *const ()), // sub_141ac12c0
    Replacement::draft(0x141ac12e0, "unknown_141ac12e0", tpl_34318 as *const ()), // sub_141ac12e0
    Replacement::draft(0x141ac1300, "unknown_141ac1300", tpl_34318 as *const ()), // sub_141ac1300
    Replacement::draft(0x141ac1320, "unknown_141ac1320", tpl_34318 as *const ()), // sub_141ac1320
    Replacement::draft(0x141ac1340, "unknown_141ac1340", tpl_34318 as *const ()), // sub_141ac1340
    Replacement::draft(0x141ac1360, "unknown_141ac1360", tpl_34318 as *const ()), // sub_141ac1360
    Replacement::draft(0x141ac1380, "unknown_141ac1380", tpl_34318 as *const ()), // sub_141ac1380
    Replacement::draft(0x141ac13a0, "unknown_141ac13a0", tpl_34319::<0x143cb49c4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac13a0
    Replacement::draft(0x141ac1400, "unknown_141ac1400", tpl_34319::<0x143c900a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac1400
    Replacement::draft(0x141ac1460, "unknown_141ac1460", tpl_34319::<0x143cac928, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac1460
    Replacement::draft(0x141ac14c0, "unknown_141ac14c0", tpl_34319::<0x143c8f7a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac14c0
    Replacement::draft(0x141ac1520, "unknown_141ac1520", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac1520
    Replacement::draft(0x141ac1580, "unknown_141ac1580", tpl_34319::<0x143cb4dbc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac1580
    Replacement::draft(0x141ac15e0, "unknown_141ac15e0", tpl_34319::<0x143cb4dcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac15e0
    Replacement::draft(0x141ac1640, "unknown_141ac1640", tpl_35685::<0x141ac0df0, 0x8, 0x10, 0x18> as *const ()), // sub_141ac1640
    Replacement::draft(0x141ac1680, "unknown_141ac1680", tpl_3546::<0x8, 0x141ac0df0, 0x10, 0x18> as *const ()), // sub_141ac1680
    Replacement::draft(0x141ac16f0, "unknown_141ac16f0", tpl_3546::<0x8, 0x141ac0df0, 0x10, 0x18> as *const ()), // sub_141ac16f0
    Replacement::draft_throws(0x141ac1730, "unknown_141ac1730", tpl_25197::<0x8, 0x19, 0x20, 0x10, 0x141ac0df0, 0x28, 0x18, 0x141ac0ea0, 0x140200130, 0x141ac1821> as *const (), Throws::Discard), // sub_141ac1730
    Replacement::draft(0x141ac1850, "unknown_141ac1850", fn_141ac1850 as *const ()), // sub_141ac1850
    Replacement::draft(0x141ac1c20, "unknown_141ac1c20", fn_141ac1c20 as *const ()), // sub_141ac1c20
    Replacement::draft_throws(0x141ac2300, "unknown_141ac2300", fn_141ac2300 as *const (), Throws::Discard), // sub_141ac2300
    Replacement::draft(0x141ac23b0, "unknown_141ac23b0", tpl_35031::<0x142407c80, 0x143cb4dcc, 0x14020e8f0> as *const ()), // sub_141ac23b0
    Replacement::draft_throws(0x141ac24e0, "unknown_141ac24e0", fn_141ac24e0 as *const (), Throws::Discard), // sub_141ac24e0
    Replacement::draft(0x141ac2800, "unknown_141ac2800", tpl_5491::<0x8, 0x1405855a0, 0x1405857a0> as *const ()), // sub_141ac2800
    Replacement::draft(0x141ac2870, "unknown_141ac2870", fn_141ac2870 as *const ()), // sub_141ac2870
    Replacement::draft(0x141ac29c0, "unknown_141ac29c0", fn_141ac29c0 as *const ()), // sub_141ac29c0
    Replacement::draft(0x141ac2cb0, "unknown_141ac2cb0", fn_141ac2cb0 as *const ()), // sub_141ac2cb0
    Replacement::draft(0x141ac30b0, "unknown_141ac30b0", fn_141ac30b0 as *const ()), // sub_141ac30b0
    Replacement::draft(0x141ac30d0, "unknown_141ac30d0", fn_141ac30d0 as *const ()), // sub_141ac30d0
    Replacement::draft_throws(0x141ac3110, "unknown_141ac3110", fn_141ac3110 as *const (), Throws::Discard), // sub_141ac3110
    Replacement::draft_throws(0x141ac3150, "unknown_141ac3150", fn_141ac3150 as *const (), Throws::Discard), // sub_141ac3150
    Replacement::draft_throws(0x141ac3370, "unknown_141ac3370", fn_141ac3370 as *const (), Throws::Discard), // sub_141ac3370
    Replacement::draft(0x141ac3930, "unknown_141ac3930", fn_141ac3930 as *const ()), // sub_141ac3930
    Replacement::draft(0x141ac3a30, "unknown_141ac3a30", fn_141ac3a30 as *const ()), // sub_141ac3a30
    Replacement::draft(0x141ac3aa0, "unknown_141ac3aa0", fn_141ac3aa0 as *const ()), // sub_141ac3aa0
    Replacement::draft(0x141ac3c70, "unknown_141ac3c70", fn_141ac3c70 as *const ()), // sub_141ac3c70
    Replacement::draft(0x141ac3e90, "unknown_141ac3e90", fn_141ac3e90 as *const ()), // sub_141ac3e90
    Replacement::draft(0x141ac3eb0, "unknown_141ac3eb0", tpl_34809 as *const ()), // sub_141ac3eb0
    Replacement::draft(0x141ac3ef0, "unknown_141ac3ef0", tpl_34994::<0x143cb5a30, 0x14387bcb0, 0x14387bca8, 0x141834a78, 0x1425f47d0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac3ef0
    Replacement::draft(0x141ac3f60, "unknown_141ac3f60", tpl_369 as *const ()), // sub_141ac3f60
    Replacement::draft(0x141ac3f70, "unknown_141ac3f70", tpl_34994::<0x143cb5024, 0x14387bd00, 0x14387bcf8, 0x141834a78, 0x1425f47e0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac3f70
    Replacement::draft(0x141ac3fe0, "unknown_141ac3fe0", tpl_369 as *const ()), // sub_141ac3fe0
    Replacement::draft(0x141ac3ff0, "unknown_141ac3ff0", tpl_34994::<0x143cb54f4, 0x14387bc80, 0x14387bc78, 0x141834a78, 0x1425f47f0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac3ff0
    Replacement::draft(0x141ac4060, "unknown_141ac4060", tpl_34995 as *const ()), // sub_141ac4060
    Replacement::draft(0x141ac4080, "unknown_141ac4080", tpl_34995 as *const ()), // sub_141ac4080
    Replacement::draft(0x141ac40a0, "unknown_141ac40a0", tpl_34994::<0x143cb6280, 0x14387bce0, 0x14387bcd8, 0x141834a78, 0x1425f4800, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac40a0
    Replacement::draft(0x141ac4110, "unknown_141ac4110", tpl_34611::<0x8> as *const ()), // sub_141ac4110
    Replacement::draft(0x141ac4130, "unknown_141ac4130", tpl_34994::<0x143cb5edc, 0x14387bd20, 0x14387bd18, 0x141834a78, 0x1425f4810, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac4130
    Replacement::draft(0x141ac41a0, "unknown_141ac41a0", tpl_369 as *const ()), // sub_141ac41a0
    Replacement::draft(0x141ac41b0, "unknown_141ac41b0", tpl_34994::<0x143cb54b4, 0x14387bca0, 0x14387bc98, 0x141834a78, 0x1425f4820, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac41b0
    Replacement::draft(0x141ac4220, "unknown_141ac4220", tpl_369 as *const ()), // sub_141ac4220
    Replacement::draft(0x141ac4230, "unknown_141ac4230", tpl_369 as *const ()), // sub_141ac4230
    Replacement::draft(0x141ac4240, "unknown_141ac4240", tpl_34835::<0x8, 0x14045bad0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ac4240
    Replacement::draft(0x141ac42c0, "unknown_141ac42c0", tpl_34835::<0x8, 0x14045bad0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ac42c0
    Replacement::draft(0x141ac4340, "unknown_141ac4340", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ac4340
    Replacement::draft(0x141ac43c0, "unknown_141ac43c0", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ac43c0
    Replacement::draft(0x141ac4440, "unknown_141ac4440", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ac4440
    Replacement::draft(0x141ac44c0, "unknown_141ac44c0", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ac44c0
    Replacement::draft(0x141ac4540, "unknown_141ac4540", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ac4540
    Replacement::draft(0x141ac45c0, "unknown_141ac45c0", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ac45c0
    Replacement::draft(0x141ac4640, "unknown_141ac4640", tpl_34994::<0x143cb5c80, 0x14387bd30, 0x14387bd28, 0x141834a78, 0x1425f4780, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac4640
    Replacement::draft(0x141ac46b0, "unknown_141ac46b0", tpl_35099::<0x20, 0x40> as *const ()), // sub_141ac46b0
    Replacement::draft(0x141ac46f0, "unknown_141ac46f0", tpl_34994::<0x143cb5838, 0x14387bc60, 0x14387bc58, 0x141834a78, 0x1425f4790, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac46f0
    Replacement::draft(0x141ac4760, "unknown_141ac4760", tpl_34395::<0x10, 0x18> as *const ()), // sub_141ac4760
    Replacement::draft(0x141ac4790, "unknown_141ac4790", tpl_34994::<0x143cb5ed4, 0x14387bc70, 0x14387bc68, 0x141834a78, 0x1425f47a0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ac4790
    Replacement::draft(0x141ac4800, "unknown_141ac4800", tpl_34394::<0x10> as *const ()), // sub_141ac4800
    Replacement::draft(0x141ac4830, "unknown_141ac4830", tpl_34837::<0x14045bad0, 0x20, 0x14240aa00> as *const ()), // sub_141ac4830
    Replacement::draft(0x141ac4880, "unknown_141ac4880", tpl_34837::<0x14045bad0, 0x20, 0x14240aa00> as *const ()), // sub_141ac4880
    Replacement::draft(0x141ac48d0, "unknown_141ac48d0", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_141ac48d0
    Replacement::draft(0x141ac4920, "unknown_141ac4920", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_141ac4920
    Replacement::draft(0x141ac4970, "unknown_141ac4970", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_141ac4970
    Replacement::draft(0x141ac49c0, "unknown_141ac49c0", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_141ac49c0
    Replacement::draft(0x141ac4a10, "unknown_141ac4a10", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_141ac4a10
    Replacement::draft(0x141ac4a60, "unknown_141ac4a60", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_141ac4a60
    Replacement::draft(0x141ac4ab0, "unknown_141ac4ab0", tpl_34840::<0x14045abd0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141ac4ab0
    Replacement::draft(0x141ac4b20, "unknown_141ac4b20", tpl_34840::<0x14045abd0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141ac4b20
    Replacement::draft(0x141ac4b90, "unknown_141ac4b90", tpl_34840::<0x1403575b0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141ac4b90
    Replacement::draft(0x141ac4c00, "unknown_141ac4c00", tpl_34840::<0x1403575b0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141ac4c00
    Replacement::draft(0x141ac4c70, "unknown_141ac4c70", tpl_34840::<0x1403575b0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141ac4c70
    Replacement::draft(0x141ac4ce0, "unknown_141ac4ce0", tpl_34840::<0x1403575b0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141ac4ce0
    Replacement::draft(0x141ac4d50, "unknown_141ac4d50", tpl_34840::<0x1403575b0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141ac4d50
    Replacement::draft(0x141ac4dc0, "unknown_141ac4dc0", tpl_34840::<0x1403575b0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141ac4dc0
    Replacement::draft(0x141ac4e30, "unknown_141ac4e30", tpl_361 as *const ()), // sub_141ac4e30
    Replacement::draft(0x141ac4e40, "unknown_141ac4e40", tpl_35063::<0x143cb6030, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141ac4e40
    Replacement::draft(0x141ac4e90, "unknown_141ac4e90", tpl_34318 as *const ()), // sub_141ac4e90
    Replacement::draft(0x141ac4eb0, "unknown_141ac4eb0", tpl_34319::<0x143c8dbd4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac4eb0
    Replacement::draft(0x141ac4f10, "unknown_141ac4f10", tpl_34318 as *const ()), // sub_141ac4f10
    Replacement::draft(0x141ac4f30, "unknown_141ac4f30", tpl_34319::<0x143c9d618, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac4f30
    Replacement::draft(0x141ac4f90, "unknown_141ac4f90", tpl_34318 as *const ()), // sub_141ac4f90
    Replacement::draft(0x141ac4fb0, "unknown_141ac4fb0", tpl_34318 as *const ()), // sub_141ac4fb0
    Replacement::draft(0x141ac4fd0, "unknown_141ac4fd0", tpl_34318 as *const ()), // sub_141ac4fd0
    Replacement::draft(0x141ac4ff0, "unknown_141ac4ff0", tpl_34319::<0x143c954e4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac4ff0
    Replacement::draft(0x141ac5050, "unknown_141ac5050", tpl_34319::<0x143c8e070, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5050
    Replacement::draft(0x141ac50b0, "unknown_141ac50b0", tpl_34319::<0x143c8de70, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac50b0
    Replacement::draft(0x141ac5110, "unknown_141ac5110", tpl_34318 as *const ()), // sub_141ac5110
    Replacement::draft(0x141ac5130, "unknown_141ac5130", tpl_34318 as *const ()), // sub_141ac5130
    Replacement::draft(0x141ac5150, "unknown_141ac5150", tpl_34319::<0x143cb58bc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5150
    Replacement::draft(0x141ac51b0, "unknown_141ac51b0", tpl_34319::<0x143cb6248, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac51b0
    Replacement::draft(0x141ac5210, "unknown_141ac5210", tpl_34318 as *const ()), // sub_141ac5210
    Replacement::draft(0x141ac5230, "unknown_141ac5230", tpl_34318 as *const ()), // sub_141ac5230
    Replacement::draft(0x141ac5250, "unknown_141ac5250", tpl_34319::<0x143c9d1d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5250
    Replacement::draft(0x141ac52b0, "unknown_141ac52b0", tpl_34319::<0x143c900a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac52b0
    Replacement::draft(0x141ac5310, "unknown_141ac5310", tpl_34318 as *const ()), // sub_141ac5310
    Replacement::draft(0x141ac5330, "unknown_141ac5330", tpl_34319::<0x143ca6cf0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5330
    Replacement::draft(0x141ac5390, "unknown_141ac5390", tpl_34318 as *const ()), // sub_141ac5390
    Replacement::draft(0x141ac53b0, "unknown_141ac53b0", tpl_34319::<0x143cb5940, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac53b0
    Replacement::draft(0x141ac5410, "unknown_141ac5410", tpl_34318 as *const ()), // sub_141ac5410
    Replacement::draft(0x141ac5430, "unknown_141ac5430", tpl_34319::<0x143cb5fe4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5430
    Replacement::draft(0x141ac5490, "unknown_141ac5490", tpl_34318 as *const ()), // sub_141ac5490
    Replacement::draft(0x141ac54b0, "unknown_141ac54b0", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac54b0
    Replacement::draft(0x141ac5510, "unknown_141ac5510", tpl_34318 as *const ()), // sub_141ac5510
    Replacement::draft(0x141ac5530, "unknown_141ac5530", tpl_34319::<0x143cb5f08, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5530
    Replacement::draft(0x141ac5590, "std_141ac5590", tpl_770::<0x1431cc2c0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_1>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac55b0, "std_141ac55b0", tpl_770::<0x1431cc2c0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_1>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac55d0, "std_141ac55d0", tpl_37227::<0x300> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_1>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac55f0, "std_141ac55f0", tpl_1::<0x14388cae0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_1>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5600, "std_141ac5600", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_1>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5610, "std_141ac5610", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_1>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5630, "std_141ac5630", tpl_770::<0x1431cc1e0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_2>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5650, "std_141ac5650", tpl_770::<0x1431cc1e0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_2>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5670, "std_141ac5670", tpl_37227::<0x301> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_2>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5690, "std_141ac5690", tpl_1::<0x14388cd80> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_2>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac56a0, "std_141ac56a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_2>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac56b0, "std_141ac56b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_2>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac56d0, "std_141ac56d0", tpl_770::<0x1431cc2f8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_3>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac56f0, "std_141ac56f0", tpl_770::<0x1431cc2f8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_3>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5710, "std_141ac5710", tpl_37228::<0x3c0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_3>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5730, "std_141ac5730", tpl_1::<0x14388ca70> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_3>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5740, "std_141ac5740", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_3>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5750, "std_141ac5750", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_3>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5770, "std_141ac5770", tpl_770::<0x1431cc3f0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_4>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5790, "std_141ac5790", tpl_770::<0x1431cc3f0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_4>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac57b0, "std_141ac57b0", tpl_37227::<0x303> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_4>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac57d0, "std_141ac57d0", tpl_1::<0x14388cb50> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_4>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac57e0, "std_141ac57e0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_4>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac57f0, "std_141ac57f0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_4>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5810, "std_141ac5810", tpl_770::<0x1431cc340> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_5>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5830, "std_141ac5830", tpl_770::<0x1431cc340> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_5>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5850, "std_141ac5850", tpl_37227::<0x304> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_5>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5870, "std_141ac5870", tpl_1::<0x14388cc30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_5>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5880, "std_141ac5880", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_5>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5890, "std_141ac5890", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_5>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac58b0, "std_141ac58b0", tpl_770::<0x1431cc448> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_6>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac58d0, "std_141ac58d0", tpl_770::<0x1431cc448> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_6>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac58f0, "std_141ac58f0", tpl_37227::<0x305> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_6>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5910, "std_141ac5910", tpl_1::<0x14388cd10> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_6>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5920, "std_141ac5920", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_6>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5930, "std_141ac5930", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_6>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5950, "std_141ac5950", tpl_770::<0x1431cc248> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_7>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5970, "std_141ac5970", tpl_770::<0x1431cc248> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_7>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5990, "std_141ac5990", tpl_37228::<0x57c> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_7>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac59b0, "std_141ac59b0", tpl_1::<0x14388cbc0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_7>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac59c0, "std_141ac59c0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_7>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac59d0, "std_141ac59d0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_7>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac59f0, "std_141ac59f0", tpl_550::<0x1431cc398, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_8>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5a10, "std_141ac5a10", tpl_550::<0x1431cc398, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_8>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5a30, "std_141ac5a30", fn_141ac5a30 as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_8>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5a90, "std_141ac5a90", tpl_1::<0x14388cdf0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_8>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5aa0, "std_141ac5aa0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_8>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5ab0, "std_141ac5ab0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_8>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5ad0, "std_141ac5ad0", tpl_550::<0x1431cc480, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_9>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5af0, "std_141ac5af0", tpl_550::<0x1431cc480, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_9>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5b10, "std_141ac5b10", fn_141ac5b10 as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_9>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5b40, "std_141ac5b40", tpl_1::<0x14388cca0> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_9>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5b50, "std_141ac5b50", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_9>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5b60, "std_141ac5b60", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::caseboard::evaluateCondition::<lambda_9>,bool,const ?bref4::?bref3::Item*>::std::_Func
    Replacement::draft(0x141ac5b80, "unknown_141ac5b80", tpl_34318 as *const ()), // sub_141ac5b80
    Replacement::draft(0x141ac5ba0, "unknown_141ac5ba0", tpl_34319::<0x143c9b0d4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5ba0
    Replacement::draft(0x141ac5c00, "unknown_141ac5c00", tpl_34318 as *const ()), // sub_141ac5c00
    Replacement::draft(0x141ac5c20, "unknown_141ac5c20", tpl_34318 as *const ()), // sub_141ac5c20
    Replacement::draft(0x141ac5c40, "unknown_141ac5c40", tpl_34319::<0x143c8f6dc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5c40
    Replacement::draft(0x141ac5ca0, "unknown_141ac5ca0", tpl_34319::<0x143c8f7a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5ca0
    Replacement::draft(0x141ac5d00, "unknown_141ac5d00", tpl_34318 as *const ()), // sub_141ac5d00
    Replacement::draft(0x141ac5d20, "unknown_141ac5d20", tpl_34318 as *const ()), // sub_141ac5d20
    Replacement::draft(0x141ac5d40, "unknown_141ac5d40", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5d40
    Replacement::draft(0x141ac5da0, "unknown_141ac5da0", tpl_34319::<0x143cb5350, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5da0
    Replacement::draft(0x141ac5e00, "unknown_141ac5e00", tpl_34318 as *const ()), // sub_141ac5e00
    Replacement::draft(0x141ac5e20, "unknown_141ac5e20", tpl_34319::<0x143cb5d00, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5e20
    Replacement::draft(0x141ac5e80, "unknown_141ac5e80", tpl_34318 as *const ()), // sub_141ac5e80
    Replacement::draft(0x141ac5ea0, "unknown_141ac5ea0", tpl_34318 as *const ()), // sub_141ac5ea0
    Replacement::draft(0x141ac5ec0, "unknown_141ac5ec0", tpl_34318 as *const ()), // sub_141ac5ec0
    Replacement::draft(0x141ac5ee0, "unknown_141ac5ee0", tpl_34318 as *const ()), // sub_141ac5ee0
    Replacement::draft(0x141ac5f00, "unknown_141ac5f00", tpl_34318 as *const ()), // sub_141ac5f00
    Replacement::draft(0x141ac5f20, "unknown_141ac5f20", tpl_34318 as *const ()), // sub_141ac5f20
    Replacement::draft(0x141ac5f40, "unknown_141ac5f40", tpl_34318 as *const ()), // sub_141ac5f40
    Replacement::draft(0x141ac5f60, "unknown_141ac5f60", tpl_34319::<0x143cb52a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5f60
    Replacement::draft(0x141ac5fc0, "unknown_141ac5fc0", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac5fc0
    Replacement::draft(0x141ac6020, "unknown_141ac6020", tpl_34319::<0x143c8d074, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac6020
    Replacement::draft(0x141ac6080, "unknown_141ac6080", tpl_34319::<0x143c93358, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac6080
    Replacement::draft(0x141ac60e0, "unknown_141ac60e0", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac60e0
    Replacement::draft(0x141ac6140, "unknown_141ac6140", tpl_34319::<0x143c933f0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac6140
    Replacement::draft(0x141ac61a0, "unknown_141ac61a0", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac61a0
    Replacement::draft(0x141ac6200, "unknown_141ac6200", tpl_34318 as *const ()), // sub_141ac6200
    Replacement::draft(0x141ac6220, "unknown_141ac6220", tpl_34318 as *const ()), // sub_141ac6220
    Replacement::draft(0x141ac6240, "unknown_141ac6240", tpl_34319::<0x143c9bad4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac6240
    Replacement::draft(0x141ac62a0, "unknown_141ac62a0", tpl_34319::<0x143c92bcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac62a0
    Replacement::draft(0x141ac6300, "unknown_141ac6300", tpl_34318 as *const ()), // sub_141ac6300
    Replacement::draft(0x141ac6320, "unknown_141ac6320", tpl_34318 as *const ()), // sub_141ac6320
    Replacement::draft(0x141ac6340, "unknown_141ac6340", tpl_34319::<0x143c9afe8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac6340
    Replacement::draft(0x141ac63a0, "unknown_141ac63a0", tpl_34319::<0x143cb5358, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ac63a0
    Replacement::draft_throws(0x141ac6400, "unknown_141ac6400", tpl_94836::<0x1402845f0, 0x14276b818, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_141ac6400
    Replacement::draft_throws(0x141ac6530, "unknown_141ac6530", tpl_94836::<0x1402845f0, 0x14276b818, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_141ac6530
    Replacement::draft_throws(0x141ac6660, "unknown_141ac6660", tpl_94836::<0x1402845f0, 0x14276b818, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_141ac6660
    Replacement::draft_throws(0x141ac6790, "unknown_141ac6790", tpl_94836::<0x1402845f0, 0x14276b818, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_141ac6790
    Replacement::draft_throws(0x141ac68c0, "unknown_141ac68c0", tpl_94836::<0x1402845f0, 0x14276b818, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_141ac68c0
    Replacement::draft_throws(0x141ac69f0, "unknown_141ac69f0", tpl_94836::<0x1402845f0, 0x14276b818, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_141ac69f0
    Replacement::draft_throws(0x141ac6b20, "unknown_141ac6b20", tpl_96040::<0x14043c2a0, 0x14309c488, 0x14276b8b0, 0x14276b818, 0x140254aa0, 0x1424b16a0, 0x140448ae0> as *const (), Throws::Discard), // sub_141ac6b20
    Replacement::draft_throws(0x141ac6cd0, "unknown_141ac6cd0", tpl_96040::<0x14043c2a0, 0x14309c488, 0x14276b820, 0x14276b818, 0x140254aa0, 0x1424b16a0, 0x140448ae0> as *const (), Throws::Discard), // sub_141ac6cd0
    Replacement::draft_throws(0x141ac6e80, "unknown_141ac6e80", fn_141ac6e80 as *const (), Throws::Caught), // sub_141ac6e80
    Replacement::draft_throws(0x141ac7080, "unknown_141ac7080", tpl_35065::<0x18, 0x141ac6400, 0x10> as *const (), Throws::Discard), // sub_141ac7080
    Replacement::draft_throws(0x141ac70c0, "unknown_141ac70c0", tpl_34846::<0x1401f3a20, 0x141ac6400> as *const (), Throws::Discard), // sub_141ac70c0
    Replacement::draft_throws(0x141ac7140, "unknown_141ac7140", fn_141ac7140 as *const (), Throws::Caught), // sub_141ac7140
    Replacement::draft_throws(0x141ac7340, "unknown_141ac7340", tpl_35065::<0x18, 0x141ac6530, 0x10> as *const (), Throws::Discard), // sub_141ac7340
    Replacement::draft_throws(0x141ac7380, "unknown_141ac7380", tpl_34846::<0x1401f3a20, 0x141ac6530> as *const (), Throws::Discard), // sub_141ac7380
    Replacement::draft_throws(0x141ac7400, "unknown_141ac7400", fn_141ac7400 as *const (), Throws::Caught), // sub_141ac7400
    Replacement::draft_throws(0x141ac7600, "unknown_141ac7600", tpl_35065::<0x18, 0x141ac6660, 0x10> as *const (), Throws::Discard), // sub_141ac7600
    Replacement::draft_throws(0x141ac7640, "unknown_141ac7640", tpl_34846::<0x1401f3a20, 0x141ac6660> as *const (), Throws::Discard), // sub_141ac7640
    Replacement::draft_throws(0x141ac76c0, "unknown_141ac76c0", fn_141ac76c0 as *const (), Throws::Caught), // sub_141ac76c0
    Replacement::draft_throws(0x141ac78c0, "unknown_141ac78c0", tpl_35065::<0x18, 0x141ac6790, 0x10> as *const (), Throws::Discard), // sub_141ac78c0
    Replacement::draft_throws(0x141ac7900, "unknown_141ac7900", tpl_34846::<0x1401f3a20, 0x141ac6790> as *const (), Throws::Discard), // sub_141ac7900
    Replacement::draft_throws(0x141ac7980, "unknown_141ac7980", fn_141ac7980 as *const (), Throws::Caught), // sub_141ac7980
    Replacement::draft_throws(0x141ac7b80, "unknown_141ac7b80", tpl_35065::<0x18, 0x141ac68c0, 0x10> as *const (), Throws::Discard), // sub_141ac7b80
    Replacement::draft_throws(0x141ac7bc0, "unknown_141ac7bc0", tpl_34846::<0x1401f3a20, 0x141ac68c0> as *const (), Throws::Discard), // sub_141ac7bc0
    Replacement::draft_throws(0x141ac7c40, "unknown_141ac7c40", fn_141ac7c40 as *const (), Throws::Caught), // sub_141ac7c40
    Replacement::draft_throws(0x141ac7e40, "unknown_141ac7e40", tpl_35065::<0x18, 0x141ac69f0, 0x10> as *const (), Throws::Discard), // sub_141ac7e40
    Replacement::draft_throws(0x141ac7e80, "unknown_141ac7e80", tpl_34846::<0x1401f3a20, 0x141ac69f0> as *const (), Throws::Discard), // sub_141ac7e80
    Replacement::draft_throws(0x141ac7f00, "unknown_141ac7f00", fn_141ac7f00 as *const (), Throws::Caught), // sub_141ac7f00
    Replacement::draft_throws(0x141ac8100, "unknown_141ac8100", tpl_36188::<0x18, 0x141ac6b20, 0x10> as *const (), Throws::Discard), // sub_141ac8100
    Replacement::draft_throws(0x141ac8140, "unknown_141ac8140", tpl_34846::<0x1401f3a20, 0x141ac6b20> as *const (), Throws::Discard), // sub_141ac8140
    Replacement::draft_throws(0x141ac81c0, "unknown_141ac81c0", fn_141ac81c0 as *const (), Throws::Caught), // sub_141ac81c0
    Replacement::draft_throws(0x141ac83c0, "unknown_141ac83c0", tpl_36188::<0x18, 0x141ac6cd0, 0x10> as *const (), Throws::Discard), // sub_141ac83c0
    Replacement::draft_throws(0x141ac8400, "unknown_141ac8400", tpl_34846::<0x1401f3a20, 0x141ac6cd0> as *const (), Throws::Discard), // sub_141ac8400
    Replacement::draft(0x141ac8480, "unknown_141ac8480", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141ac8480
    Replacement::draft(0x141ac84b0, "unknown_141ac84b0", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141ac84b0
    Replacement::draft(0x141ac84e0, "unknown_141ac84e0", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141ac84e0
    Replacement::draft(0x141ac8510, "unknown_141ac8510", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141ac8510
    Replacement::draft(0x141ac8540, "unknown_141ac8540", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141ac8540
    Replacement::draft(0x141ac8570, "unknown_141ac8570", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141ac8570
    Replacement::draft(0x141ac85a0, "unknown_141ac85a0", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141ac85a0
    Replacement::draft(0x141ac85d0, "unknown_141ac85d0", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_141ac85d0
    Replacement::draft_throws(0x141ac8600, "unknown_141ac8600", fn_141ac8600 as *const (), Throws::Caught), // sub_141ac8600
    Replacement::draft(0x141ac8960, "unknown_141ac8960", tpl_34443 as *const ()), // sub_141ac8960
    Replacement::draft(0x141ac8970, "unknown_141ac8970", tpl_37235::<0xc0> as *const ()), // sub_141ac8970
    Replacement::draft(0x141ac8990, "unknown_141ac8990", tpl_34443 as *const ()), // sub_141ac8990
    Replacement::draft(0x141ac89a0, "unknown_141ac89a0", tpl_37236::<0x8, 0x3, 0x4, 0x1426d01e0> as *const ()), // sub_141ac89a0
    Replacement::draft(0x141ac8a30, "unknown_141ac8a30", tpl_34443 as *const ()), // sub_141ac8a30
    Replacement::draft(0x141ac8a40, "unknown_141ac8a40", tpl_37237::<0xc, 0x7, 0x4, 0x8, 0x1426d01e0> as *const ()), // sub_141ac8a40
    Replacement::draft(0x141ac8ad0, "unknown_141ac8ad0", fn_141ac8ad0 as *const ()), // sub_141ac8ad0
    Replacement::draft(0x141ac8b20, "unknown_141ac8b20", tpl_34443 as *const ()), // sub_141ac8b20
    Replacement::draft(0x141ac8b30, "unknown_141ac8b30", tpl_37236::<0x8, 0x3, 0x4, 0x1426d01e0> as *const ()), // sub_141ac8b30
    Replacement::draft(0x141ac8bc0, "unknown_141ac8bc0", fn_141ac8bc0 as *const ()), // sub_141ac8bc0
    Replacement::draft(0x141ac8cc0, "unknown_141ac8cc0", tpl_34443 as *const ()), // sub_141ac8cc0
    Replacement::draft(0x141ac8cd0, "unknown_141ac8cd0", tpl_37237::<0xc, 0x7, 0x4, 0x8, 0x1426d01e0> as *const ()), // sub_141ac8cd0
    Replacement::draft(0x141ac8d60, "unknown_141ac8d60", tpl_5554::<0x8, 0x140591c90, 0x140591e90, 0x140592160> as *const ()), // sub_141ac8d60
    Replacement::draft(0x141ac8e00, "unknown_141ac8e00", tpl_361 as *const ()), // sub_141ac8e00
    Replacement::draft(0x141ac8e10, "unknown_141ac8e10", fn_141ac8e10 as *const ()), // sub_141ac8e10
    Replacement::draft(0x141ac9230, "unknown_141ac9230", fn_141ac9230 as *const ()), // sub_141ac9230
    Replacement::draft(0x141ac9340, "unknown_141ac9340", tpl_361 as *const ()), // sub_141ac9340
    Replacement::draft(0x141ac9350, "unknown_141ac9350", fn_141ac9350 as *const ()), // sub_141ac9350
    Replacement::draft(0x141ac9400, "unknown_141ac9400", tpl_361 as *const ()), // sub_141ac9400
    Replacement::draft(0x141ac9410, "unknown_141ac9410", fn_141ac9410 as *const ()), // sub_141ac9410
    Replacement::draft(0x141ac9590, "unknown_141ac9590", tpl_361 as *const ()), // sub_141ac9590
    Replacement::draft(0x141ac95a0, "unknown_141ac95a0", fn_141ac95a0 as *const ()), // sub_141ac95a0
    Replacement::draft(0x141ac95f0, "unknown_141ac95f0", fn_141ac95f0 as *const ()), // sub_141ac95f0
    Replacement::draft(0x141ac9810, "unknown_141ac9810", tpl_361 as *const ()), // sub_141ac9810
    Replacement::draft(0x141ac9820, "unknown_141ac9820", fn_141ac9820 as *const ()), // sub_141ac9820
    Replacement::draft(0x141ac9870, "unknown_141ac9870", tpl_361 as *const ()), // sub_141ac9870
    Replacement::draft_throws(0x141ac9880, "unknown_141ac9880", fn_141ac9880 as *const (), Throws::Discard), // sub_141ac9880
    Replacement::draft(0x141aca600, "unknown_141aca600", tpl_361 as *const ()), // sub_141aca600
    Replacement::draft(0x141aca610, "unknown_141aca610", tpl_361 as *const ()), // sub_141aca610
    Replacement::draft(0x141acacb0, "unknown_141acacb0", tpl_361 as *const ()), // sub_141acacb0
    Replacement::draft(0x141acacc0, "unknown_141acacc0", tpl_361 as *const ()), // sub_141acacc0
    Replacement::draft(0x141acacd0, "unknown_141acacd0", tpl_34443 as *const ()), // sub_141acacd0
    Replacement::draft(0x141acace0, "unknown_141acace0", fn_141acace0 as *const ()), // sub_141acace0
    Replacement::draft(0x141acad30, "unknown_141acad30", fn_141acad30 as *const ()), // sub_141acad30
    Replacement::draft(0x141acb3a0, "unknown_141acb3a0", tpl_361 as *const ()), // sub_141acb3a0
    Replacement::draft(0x141acb3b0, "unknown_141acb3b0", tpl_361 as *const ()), // sub_141acb3b0
    Replacement::draft(0x141acb6d0, "unknown_141acb6d0", tpl_361 as *const ()), // sub_141acb6d0
    Replacement::draft(0x141acb6e0, "unknown_141acb6e0", fn_141acb6e0 as *const ()), // sub_141acb6e0
    Replacement::draft(0x141acb720, "unknown_141acb720", tpl_37246::<0x73c> as *const ()), // sub_141acb720
    Replacement::draft(0x141acb740, "unknown_141acb740", tpl_361 as *const ()), // sub_141acb740
    Replacement::draft(0x141acb750, "unknown_141acb750", fn_141acb750 as *const ()), // sub_141acb750
    Replacement::draft(0x141acb950, "unknown_141acb950", tpl_361 as *const ()), // sub_141acb950
    Replacement::draft_throws(0x141acb960, "unknown_141acb960", fn_141acb960 as *const (), Throws::Caught), // sub_141acb960
    Replacement::draft(0x141accdf0, "unknown_141accdf0", fn_141accdf0 as *const ()), // sub_141accdf0
    Replacement::draft(0x141accf80, "unknown_141accf80", tpl_361 as *const ()), // sub_141accf80
    Replacement::draft(0x141accf90, "unknown_141accf90", fn_141accf90 as *const ()), // sub_141accf90
    Replacement::draft(0x141acd1b0, "unknown_141acd1b0", fn_141acd1b0 as *const ()), // sub_141acd1b0
    Replacement::draft(0x141acd2d0, "unknown_141acd2d0", fn_141acd2d0 as *const ()), // sub_141acd2d0
    Replacement::draft(0x141acd440, "unknown_141acd440", fn_141acd440 as *const ()), // sub_141acd440
    Replacement::draft(0x141acd4c0, "unknown_141acd4c0", tpl_361 as *const ()), // sub_141acd4c0
    Replacement::draft(0x141acdfa0, "unknown_141acdfa0", tpl_361 as *const ()), // sub_141acdfa0
    Replacement::draft(0x141acdfb0, "unknown_141acdfb0", fn_141acdfb0 as *const ()), // sub_141acdfb0
    Replacement::draft_throws(0x141ace5b0, "unknown_141ace5b0", fn_141ace5b0 as *const (), Throws::Caught), // sub_141ace5b0
    Replacement::draft_throws(0x141acecb0, "unknown_141acecb0", fn_141acecb0 as *const (), Throws::Discard), // sub_141acecb0
    Replacement::draft(0x141acee30, "unknown_141acee30", fn_141acee30 as *const ()), // sub_141acee30
    Replacement::draft(0x141acf100, "unknown_141acf100", tpl_37254::<0x90, 0x308, 0xa8> as *const ()), // sub_141acf100
    Replacement::draft(0x141acf130, "unknown_141acf130", tpl_37254::<0x90, 0x305, 0xa5> as *const ()), // sub_141acf130
    Replacement::draft(0x141acf160, "unknown_141acf160", fn_141acf160 as *const ()), // sub_141acf160
    Replacement::draft(0x141acf1a0, "unknown_141acf1a0", fn_141acf1a0 as *const ()), // sub_141acf1a0
    Replacement::draft(0x141acf3f0, "unknown_141acf3f0", fn_141acf3f0 as *const ()), // sub_141acf3f0
    Replacement::draft(0x141acf690, "unknown_141acf690", fn_141acf690 as *const ()), // sub_141acf690
    Replacement::draft_throws(0x141acf710, "unknown_141acf710", fn_141acf710 as *const (), Throws::Discard), // sub_141acf710
    Replacement::draft(0x141acf9f0, "unknown_141acf9f0", fn_141acf9f0 as *const ()), // sub_141acf9f0
    Replacement::draft_throws(0x141acfbf0, "unknown_141acfbf0", fn_141acfbf0 as *const (), Throws::Discard), // sub_141acfbf0
    Replacement::draft_throws(0x141ad0010, "unknown_141ad0010", fn_141ad0010 as *const (), Throws::Discard), // sub_141ad0010
    Replacement::draft_throws(0x141ad01f0, "unknown_141ad01f0", fn_141ad01f0 as *const (), Throws::Discard), // sub_141ad01f0
    Replacement::draft(0x141ad04d0, "unknown_141ad04d0", tpl_37246::<0x748> as *const ()), // sub_141ad04d0
    Replacement::draft_throws(0x141ad04f0, "unknown_141ad04f0", fn_141ad04f0 as *const (), Throws::Discard), // sub_141ad04f0
    Replacement::draft(0x141ad0d20, "unknown_141ad0d20", fn_141ad0d20 as *const ()), // sub_141ad0d20
    Replacement::draft(0x141ad1110, "unknown_141ad1110", fn_141ad1110 as *const ()), // sub_141ad1110
    Replacement::draft(0x141ad17a0, "unknown_141ad17a0", tpl_496 as *const ()), // sub_141ad17a0
    Replacement::draft(0x141ad17b0, "unknown_141ad17b0", tpl_361 as *const ()), // sub_141ad17b0
    Replacement::draft_throws(0x141ad17c0, "unknown_141ad17c0", fn_141ad17c0 as *const (), Throws::Caught), // sub_141ad17c0
    Replacement::draft(0x141ad26d0, "unknown_141ad26d0", tpl_496 as *const ()), // sub_141ad26d0
    Replacement::draft(0x141ad26e0, "unknown_141ad26e0", fn_141ad26e0 as *const ()), // sub_141ad26e0
    Replacement::draft(0x141ad2700, "unknown_141ad2700", fn_141ad2700 as *const ()), // sub_141ad2700
    Replacement::draft(0x141ad2920, "unknown_141ad2920", fn_141ad2920 as *const ()), // sub_141ad2920
    Replacement::draft(0x141ad2950, "unknown_141ad2950", fn_141ad2950 as *const ()), // sub_141ad2950
    Replacement::draft_throws(0x141ad2990, "unknown_141ad2990", fn_141ad2990 as *const (), Throws::Discard), // sub_141ad2990
    Replacement::draft_throws(0x141ad2b30, "unknown_141ad2b30", fn_141ad2b30 as *const (), Throws::Caught), // sub_141ad2b30
    Replacement::draft(0x141ad3210, "unknown_141ad3210", fn_141ad3210 as *const ()), // sub_141ad3210
    Replacement::draft_throws(0x141ad3310, "unknown_141ad3310", fn_141ad3310 as *const (), Throws::Discard), // sub_141ad3310
    Replacement::draft(0x141ad3860, "unknown_141ad3860", tpl_361 as *const ()), // sub_141ad3860
    Replacement::draft(0x141ad3870, "unknown_141ad3870", tpl_496 as *const ()), // sub_141ad3870
];
