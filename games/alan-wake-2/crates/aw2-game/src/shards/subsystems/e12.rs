#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x141738d70, "kaim_141738d70", fn_141738d70 as *const ()), // sub_141738d70
    Replacement::draft(0x141738d80, "kaim_141738d80", fn_141738d80 as *const ()), // sub_141738d80
    Replacement::draft(0x141738d90, "kaim_141738d90", fn_141738d90 as *const ()), // sub_141738d90
    Replacement::draft(0x141738da0, "kaim_141738da0", fn_141738da0 as *const ()), // sub_141738da0
    Replacement::draft(0x141738db0, "kaim_141738db0", tpl_1::<0x1426acfa0> as *const ()), // sub_141738db0
    Replacement::draft(0x141738dc0, "kaim_141738dc0", tpl_1::<0x1426acfc8> as *const ()), // sub_141738dc0
    Replacement::draft(0x141738dd0, "kaim_141738dd0", tpl_1::<0x1426acf40> as *const ()), // sub_141738dd0
    Replacement::draft(0x141738de0, "kaim_141738de0", tpl_1::<0x1426acf88> as *const ()), // sub_141738de0
    Replacement::draft(0x141738df0, "kaim_141738df0", tpl_1::<0x1426acef8> as *const ()), // sub_141738df0
    Replacement::draft(0x141738e00, "kaim_141738e00", tpl_1::<0x1426acf10> as *const ()), // sub_141738e00
    Replacement::draft(0x141738e10, "kaim_141738e10", tpl_1::<0x1426acf28> as *const ()), // sub_141738e10
    Replacement::draft(0x141738e20, "kaim_141738e20", tpl_1::<0x1426acf58> as *const ()), // sub_141738e20
    Replacement::draft(0x141738e30, "kaim_141738e30", fn_141738e30 as *const ()), // sub_141738e30
    Replacement::draft(0x141738f50, "kaim_141738f50", fn_141738f50 as *const ()), // sub_141738f50
    Replacement::draft(0x141738ff0, "kaim_141738ff0", tpl_31744::<0x1417390a0> as *const ()), // sub_141738ff0
    Replacement::draft(0x1417390a0, "kaim_1417390a0", fn_1417390a0 as *const ()), // sub_1417390a0
    Replacement::draft(0x1417391a0, "kaim_1417391a0", fn_1417391a0 as *const ()), // sub_1417391a0
    Replacement::draft(0x1417392b0, "kaim_1417392b0", fn_1417392b0 as *const ()), // sub_1417392b0
    Replacement::draft(0x141739330, "kaim_141739330", fn_141739330 as *const ()), // sub_141739330
    Replacement::draft(0x141739950, "kaim_141739950", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ada60> as *const ()), // sub_141739950
    Replacement::draft(0x1417399b0, "kaim_1417399b0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426adad0> as *const ()), // sub_1417399b0
    Replacement::draft(0x141739a10, "kaim_141739a10", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ada98> as *const ()), // sub_141739a10
    Replacement::draft(0x141739a70, "kaim_141739a70", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426adbb0> as *const ()), // sub_141739a70
    Replacement::draft(0x141739ad0, "kaim_141739ad0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad9f0> as *const ()), // sub_141739ad0
    Replacement::draft(0x141739b30, "kaim_141739b30", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426adb40> as *const ()), // sub_141739b30
    Replacement::draft(0x141739b90, "kaim_141739b90", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad868> as *const ()), // sub_141739b90
    Replacement::draft(0x141739bf0, "kaim_141739bf0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426adb08> as *const ()), // sub_141739bf0
    Replacement::draft(0x141739c50, "kaim_141739c50", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426adb78> as *const ()), // sub_141739c50
    Replacement::draft(0x141739cb0, "kaim_141739cb0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ada28> as *const ()), // sub_141739cb0
    Replacement::draft(0x141739d10, "kaim_141739d10", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad9b8> as *const ()), // sub_141739d10
    Replacement::draft(0x141739d70, "kaim_141739d70", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad8a0> as *const ()), // sub_141739d70
    Replacement::draft(0x141739dd0, "kaim_141739dd0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad980> as *const ()), // sub_141739dd0
    Replacement::draft(0x141739e30, "kaim_141739e30", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad8d8> as *const ()), // sub_141739e30
    Replacement::draft(0x141739e90, "kaim_141739e90", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad948> as *const ()), // sub_141739e90
    Replacement::draft(0x141739ef0, "kaim_141739ef0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad910> as *const ()), // sub_141739ef0
    Replacement::draft(0x141739f50, "kaim_141739f50", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad6e0, 0x8> as *const ()), // sub_141739f50
    Replacement::draft(0x141739f90, "kaim_141739f90", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad750, 0x8> as *const ()), // sub_141739f90
    Replacement::draft(0x141739fd0, "kaim_141739fd0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad718, 0x8> as *const ()), // sub_141739fd0
    Replacement::draft(0x14173a010, "kaim_14173a010", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad830, 0x8> as *const ()), // sub_14173a010
    Replacement::draft(0x14173a050, "kaim_14173a050", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad670, 0x8> as *const ()), // sub_14173a050
    Replacement::draft(0x14173a090, "kaim_14173a090", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad7c0, 0x8> as *const ()), // sub_14173a090
    Replacement::draft(0x14173a0d0, "kaim_14173a0d0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad4e8, 0x8> as *const ()), // sub_14173a0d0
    Replacement::draft(0x14173a110, "kaim_14173a110", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad788, 0x8> as *const ()), // sub_14173a110
    Replacement::draft(0x14173a150, "kaim_14173a150", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad7f8, 0x8> as *const ()), // sub_14173a150
    Replacement::draft(0x14173a190, "kaim_14173a190", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad6a8, 0x8> as *const ()), // sub_14173a190
    Replacement::draft(0x14173a1d0, "kaim_14173a1d0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad638, 0x8> as *const ()), // sub_14173a1d0
    Replacement::draft(0x14173a210, "kaim_14173a210", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad520, 0x8> as *const ()), // sub_14173a210
    Replacement::draft(0x14173a250, "kaim_14173a250", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad600, 0x8> as *const ()), // sub_14173a250
    Replacement::draft(0x14173a290, "kaim_14173a290", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad558, 0x8> as *const ()), // sub_14173a290
    Replacement::draft(0x14173a2d0, "kaim_14173a2d0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad5c8, 0x8> as *const ()), // sub_14173a2d0
    Replacement::draft(0x14173a310, "kaim_14173a310", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ad590, 0x8> as *const ()), // sub_14173a310
    Replacement::draft(0x14173a350, "kaim_14173a350", tpl_22446::<0x14173a690> as *const ()), // sub_14173a350
    Replacement::draft(0x14173a360, "kaim_14173a360", tpl_32704::<0x4, 0x8> as *const ()), // sub_14173a360
    Replacement::draft(0x14173a380, "kaim_14173a380", tpl_22446::<0x14173a830> as *const ()), // sub_14173a380
    Replacement::draft(0x14173a390, "kaim_14173a390", tpl_22446::<0x14173a910> as *const ()), // sub_14173a390
    Replacement::draft(0x14173a3a0, "kaim_14173a3a0", fn_14173a3a0 as *const ()), // sub_14173a3a0
    Replacement::draft(0x14173a3e0, "kaim_14173a3e0", fn_14173a3e0 as *const ()), // sub_14173a3e0
    Replacement::draft(0x14173a470, "kaim_14173a470", tpl_22446::<0x1416d5c00> as *const ()), // sub_14173a470
    Replacement::draft(0x14173a480, "kaim_14173a480", fn_14173a480 as *const ()), // sub_14173a480
    Replacement::draft(0x14173a490, "kaim_14173a490", fn_14173a490 as *const ()), // sub_14173a490
    Replacement::draft(0x14173a4a0, "kaim_14173a4a0", fn_14173a4a0 as *const ()), // sub_14173a4a0
    Replacement::draft(0x14173a4b0, "kaim_14173a4b0", fn_14173a4b0 as *const ()), // sub_14173a4b0
    Replacement::draft(0x14173a4c0, "kaim_14173a4c0", fn_14173a4c0 as *const ()), // sub_14173a4c0
    Replacement::draft(0x14173a4d0, "kaim_14173a4d0", fn_14173a4d0 as *const ()), // sub_14173a4d0
    Replacement::draft(0x14173a4e0, "kaim_14173a4e0", fn_14173a4e0 as *const ()), // sub_14173a4e0
    Replacement::draft(0x14173a4f0, "kaim_14173a4f0", fn_14173a4f0 as *const ()), // sub_14173a4f0
    Replacement::draft(0x14173a500, "kaim_14173a500", fn_14173a500 as *const ()), // sub_14173a500
    Replacement::draft(0x14173a510, "kaim_14173a510", fn_14173a510 as *const ()), // sub_14173a510
    Replacement::draft(0x14173a520, "kaim_14173a520", fn_14173a520 as *const ()), // sub_14173a520
    Replacement::draft(0x14173a530, "kaim_14173a530", fn_14173a530 as *const ()), // sub_14173a530
    Replacement::draft(0x14173a540, "kaim_14173a540", fn_14173a540 as *const ()), // sub_14173a540
    Replacement::draft(0x14173a550, "kaim_14173a550", fn_14173a550 as *const ()), // sub_14173a550
    Replacement::draft(0x14173a560, "kaim_14173a560", fn_14173a560 as *const ()), // sub_14173a560
    Replacement::draft(0x14173a570, "kaim_14173a570", fn_14173a570 as *const ()), // sub_14173a570
    Replacement::draft(0x14173a580, "kaim_14173a580", fn_14173a580 as *const ()), // sub_14173a580
    Replacement::draft(0x14173a590, "kaim_14173a590", tpl_1::<0x1426ad4a0> as *const ()), // sub_14173a590
    Replacement::draft(0x14173a5a0, "kaim_14173a5a0", tpl_1::<0x1426ad480> as *const ()), // sub_14173a5a0
    Replacement::draft(0x14173a5b0, "kaim_14173a5b0", tpl_1::<0x1426ad4b8> as *const ()), // sub_14173a5b0
    Replacement::draft(0x14173a5c0, "kaim_14173a5c0", tpl_1::<0x1426ad4d8> as *const ()), // sub_14173a5c0
    Replacement::draft(0x14173a5d0, "kaim_14173a5d0", tpl_1::<0x1426ad400> as *const ()), // sub_14173a5d0
    Replacement::draft(0x14173a5e0, "kaim_14173a5e0", tpl_1::<0x1426ad418> as *const ()), // sub_14173a5e0
    Replacement::draft(0x14173a5f0, "kaim_14173a5f0", tpl_1::<0x1426ad388> as *const ()), // sub_14173a5f0
    Replacement::draft(0x14173a600, "kaim_14173a600", tpl_1::<0x1426ad460> as *const ()), // sub_14173a600
    Replacement::draft(0x14173a610, "kaim_14173a610", tpl_1::<0x1426ad448> as *const ()), // sub_14173a610
    Replacement::draft(0x14173a620, "kaim_14173a620", tpl_1::<0x1426ad438> as *const ()), // sub_14173a620
    Replacement::draft(0x14173a630, "kaim_14173a630", tpl_1::<0x1426ad3f0> as *const ()), // sub_14173a630
    Replacement::draft(0x14173a640, "kaim_14173a640", tpl_1::<0x1426ad398> as *const ()), // sub_14173a640
    Replacement::draft(0x14173a650, "kaim_14173a650", tpl_1::<0x1426ad3d8> as *const ()), // sub_14173a650
    Replacement::draft(0x14173a660, "kaim_14173a660", tpl_1::<0x1426ad370> as *const ()), // sub_14173a660
    Replacement::draft(0x14173a670, "kaim_14173a670", tpl_1::<0x1426ad3c0> as *const ()), // sub_14173a670
    Replacement::draft(0x14173a680, "kaim_14173a680", tpl_1::<0x1426ad3a0> as *const ()), // sub_14173a680
    Replacement::draft(0x14173a690, "kaim_14173a690", fn_14173a690 as *const ()), // sub_14173a690
    Replacement::draft(0x14173a830, "kaim_14173a830", fn_14173a830 as *const ()), // sub_14173a830
    Replacement::draft(0x14173a910, "kaim_14173a910", fn_14173a910 as *const ()), // sub_14173a910
    Replacement::draft(0x14173aae0, "kaim_14173aae0", fn_14173aae0 as *const ()), // sub_14173aae0
    Replacement::draft(0x14173ad10, "kaim_14173ad10", fn_14173ad10 as *const ()), // sub_14173ad10
    Replacement::draft(0x14173aef0, "kaim_14173aef0", fn_14173aef0 as *const ()), // sub_14173aef0
    Replacement::draft(0x14173afe0, "kaim_14173afe0", fn_14173afe0 as *const ()), // sub_14173afe0
    Replacement::draft(0x14173bde0, "kaim_14173bde0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aeac8> as *const ()), // sub_14173bde0
    Replacement::draft(0x14173be40, "kaim_14173be40", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae7b8> as *const ()), // sub_14173be40
    Replacement::draft(0x14173bea0, "kaim_14173bea0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae630> as *const ()), // sub_14173bea0
    Replacement::draft(0x14173bf00, "kaim_14173bf00", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae9e8> as *const ()), // sub_14173bf00
    Replacement::draft(0x14173bf60, "kaim_14173bf60", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae9b0> as *const ()), // sub_14173bf60
    Replacement::draft(0x14173bfc0, "kaim_14173bfc0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae668> as *const ()), // sub_14173bfc0
    Replacement::draft(0x14173c020, "kaim_14173c020", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae6a0> as *const ()), // sub_14173c020
    Replacement::draft(0x14173c080, "kaim_14173c080", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aec50> as *const ()), // sub_14173c080
    Replacement::draft(0x14173c0e0, "kaim_14173c0e0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aeb00> as *const ()), // sub_14173c0e0
    Replacement::draft(0x14173c140, "kaim_14173c140", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aec88> as *const ()), // sub_14173c140
    Replacement::draft(0x14173c1a0, "kaim_14173c1a0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aeb70> as *const ()), // sub_14173c1a0
    Replacement::draft(0x14173c200, "kaim_14173c200", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae6d8> as *const ()), // sub_14173c200
    Replacement::draft(0x14173c260, "kaim_14173c260", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aea58> as *const ()), // sub_14173c260
    Replacement::draft(0x14173c2c0, "kaim_14173c2c0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aec18> as *const ()), // sub_14173c2c0
    Replacement::draft(0x14173c320, "kaim_14173c320", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae8d0> as *const ()), // sub_14173c320
    Replacement::draft(0x14173c380, "kaim_14173c380", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae908> as *const ()), // sub_14173c380
    Replacement::draft(0x14173c3e0, "kaim_14173c3e0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae710> as *const ()), // sub_14173c3e0
    Replacement::draft(0x14173c440, "kaim_14173c440", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae940> as *const ()), // sub_14173c440
    Replacement::draft(0x14173c4a0, "kaim_14173c4a0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae860> as *const ()), // sub_14173c4a0
    Replacement::draft(0x14173c500, "kaim_14173c500", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aebe0> as *const ()), // sub_14173c500
    Replacement::draft(0x14173c560, "kaim_14173c560", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae898> as *const ()), // sub_14173c560
    Replacement::draft(0x14173c5c0, "kaim_14173c5c0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae748> as *const ()), // sub_14173c5c0
    Replacement::draft(0x14173c620, "kaim_14173c620", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aea90> as *const ()), // sub_14173c620
    Replacement::draft(0x14173c680, "kaim_14173c680", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aeba8> as *const ()), // sub_14173c680
    Replacement::draft(0x14173c6e0, "kaim_14173c6e0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aecf8> as *const ()), // sub_14173c6e0
    Replacement::draft(0x14173c740, "kaim_14173c740", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae978> as *const ()), // sub_14173c740
    Replacement::draft(0x14173c7a0, "kaim_14173c7a0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae828> as *const ()), // sub_14173c7a0
    Replacement::draft(0x14173c800, "kaim_14173c800", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae7f0> as *const ()), // sub_14173c800
    Replacement::draft(0x14173c860, "kaim_14173c860", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aea20> as *const ()), // sub_14173c860
    Replacement::draft(0x14173c8c0, "kaim_14173c8c0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aecc0> as *const ()), // sub_14173c8c0
    Replacement::draft(0x14173c920, "kaim_14173c920", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aeb38> as *const ()), // sub_14173c920
    Replacement::draft(0x14173c980, "kaim_14173c980", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ae780> as *const ()), // sub_14173c980
    Replacement::draft(0x14173c9e0, "kaim_14173c9e0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae3c8, 0x8> as *const ()), // sub_14173c9e0
    Replacement::draft(0x14173ca20, "kaim_14173ca20", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae0b8, 0x8> as *const ()), // sub_14173ca20
    Replacement::draft(0x14173ca60, "kaim_14173ca60", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426adf30, 0x8> as *const ()), // sub_14173ca60
    Replacement::draft(0x14173caa0, "kaim_14173caa0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae2e8, 0x8> as *const ()), // sub_14173caa0
    Replacement::draft(0x14173cae0, "kaim_14173cae0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae2b0, 0x8> as *const ()), // sub_14173cae0
    Replacement::draft(0x14173cb20, "kaim_14173cb20", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426adf68, 0x8> as *const ()), // sub_14173cb20
    Replacement::draft(0x14173cb60, "kaim_14173cb60", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426adfa0, 0x8> as *const ()), // sub_14173cb60
    Replacement::draft(0x14173cba0, "kaim_14173cba0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae550, 0x8> as *const ()), // sub_14173cba0
    Replacement::draft(0x14173cbe0, "kaim_14173cbe0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae400, 0x8> as *const ()), // sub_14173cbe0
    Replacement::draft(0x14173cc20, "kaim_14173cc20", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae588, 0x8> as *const ()), // sub_14173cc20
    Replacement::draft(0x14173cc60, "kaim_14173cc60", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae470, 0x8> as *const ()), // sub_14173cc60
    Replacement::draft(0x14173cca0, "kaim_14173cca0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426adfd8, 0x8> as *const ()), // sub_14173cca0
    Replacement::draft(0x14173cce0, "kaim_14173cce0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae358, 0x8> as *const ()), // sub_14173cce0
    Replacement::draft(0x14173cd20, "kaim_14173cd20", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae518, 0x8> as *const ()), // sub_14173cd20
    Replacement::draft(0x14173cd60, "kaim_14173cd60", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae1d0, 0x8> as *const ()), // sub_14173cd60
    Replacement::draft(0x14173cda0, "kaim_14173cda0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae208, 0x8> as *const ()), // sub_14173cda0
    Replacement::draft(0x14173cde0, "kaim_14173cde0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae010, 0x8> as *const ()), // sub_14173cde0
    Replacement::draft(0x14173ce20, "kaim_14173ce20", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae240, 0x8> as *const ()), // sub_14173ce20
    Replacement::draft(0x14173ce60, "kaim_14173ce60", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae160, 0x8> as *const ()), // sub_14173ce60
    Replacement::draft(0x14173cea0, "kaim_14173cea0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae4e0, 0x8> as *const ()), // sub_14173cea0
    Replacement::draft(0x14173cee0, "kaim_14173cee0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae198, 0x8> as *const ()), // sub_14173cee0
    Replacement::draft(0x14173cf20, "kaim_14173cf20", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae048, 0x8> as *const ()), // sub_14173cf20
    Replacement::draft(0x14173cf60, "kaim_14173cf60", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae390, 0x8> as *const ()), // sub_14173cf60
    Replacement::draft(0x14173cfa0, "kaim_14173cfa0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae4a8, 0x8> as *const ()), // sub_14173cfa0
    Replacement::draft(0x14173cfe0, "kaim_14173cfe0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae5f8, 0x8> as *const ()), // sub_14173cfe0
    Replacement::draft(0x14173d020, "kaim_14173d020", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae278, 0x8> as *const ()), // sub_14173d020
    Replacement::draft(0x14173d060, "kaim_14173d060", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae128, 0x8> as *const ()), // sub_14173d060
    Replacement::draft(0x14173d0a0, "kaim_14173d0a0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae0f0, 0x8> as *const ()), // sub_14173d0a0
    Replacement::draft(0x14173d0e0, "kaim_14173d0e0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae320, 0x8> as *const ()), // sub_14173d0e0
    Replacement::draft(0x14173d120, "kaim_14173d120", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae5c0, 0x8> as *const ()), // sub_14173d120
    Replacement::draft(0x14173d160, "kaim_14173d160", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae438, 0x8> as *const ()), // sub_14173d160
    Replacement::draft(0x14173d1a0, "kaim_14173d1a0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426ae080, 0x8> as *const ()), // sub_14173d1a0
    Replacement::draft(0x14173d1e0, "kaim_14173d1e0", tpl_22446::<0x14173d990> as *const ()), // sub_14173d1e0
    Replacement::draft(0x14173d1f0, "kaim_14173d1f0", tpl_22446::<0x14173da40> as *const ()), // sub_14173d1f0
    Replacement::draft(0x14173d200, "kaim_14173d200", tpl_22446::<0x14173dc20> as *const ()), // sub_14173d200
    Replacement::draft(0x14173d210, "kaim_14173d210", tpl_22446::<0x14173dd00> as *const ()), // sub_14173d210
    Replacement::draft(0x14173d220, "kaim_14173d220", tpl_22446::<0x14173ddc0> as *const ()), // sub_14173d220
    Replacement::draft(0x14173d230, "kaim_14173d230", tpl_32674::<0x4, 0x14173aef0> as *const ()), // sub_14173d230
    Replacement::draft(0x14173d250, "kaim_14173d250", tpl_22446::<0x14173aae0> as *const ()), // sub_14173d250
    Replacement::draft(0x14173d260, "kaim_14173d260", fn_14173d260 as *const ()), // sub_14173d260
    Replacement::draft(0x14173d2a0, "kaim_14173d2a0", tpl_32674::<0x4, 0x14173ad10> as *const ()), // sub_14173d2a0
    Replacement::draft(0x14173d2c0, "kaim_14173d2c0", tpl_22446::<0x14173de90> as *const ()), // sub_14173d2c0
    Replacement::draft(0x14173d2d0, "kaim_14173d2d0", fn_14173d2d0 as *const ()), // sub_14173d2d0
    Replacement::draft(0x14173d300, "kaim_14173d300", fn_14173d300 as *const ()), // sub_14173d300
    Replacement::draft(0x14173d340, "kaim_14173d340", tpl_22446::<0x14173df30> as *const ()), // sub_14173d340
    Replacement::draft(0x14173d350, "kaim_14173d350", tpl_22446::<0x1416d77c0> as *const ()), // sub_14173d350
    Replacement::draft(0x14173d360, "kaim_14173d360", fn_14173d360 as *const ()), // sub_14173d360
    Replacement::draft(0x14173d3f0, "kaim_14173d3f0", fn_14173d3f0 as *const ()), // sub_14173d3f0
    Replacement::draft(0x14173d430, "kaim_14173d430", fn_14173d430 as *const ()), // sub_14173d430
    Replacement::draft(0x14173d480, "kaim_14173d480", fn_14173d480 as *const ()), // sub_14173d480
    Replacement::draft(0x14173d4d0, "kaim_14173d4d0", tpl_32674::<0x4, 0x1416abd40> as *const ()), // sub_14173d4d0
    Replacement::draft(0x14173d4f0, "kaim_14173d4f0", tpl_32704::<0x8, 0x4> as *const ()), // sub_14173d4f0
    Replacement::draft(0x14173d510, "kaim_14173d510", fn_14173d510 as *const ()), // sub_14173d510
    Replacement::draft(0x14173d560, "kaim_14173d560", tpl_22446::<0x14173e180> as *const ()), // sub_14173d560
    Replacement::draft(0x14173d570, "kaim_14173d570", tpl_22446::<0x14173e280> as *const ()), // sub_14173d570
    Replacement::draft(0x14173d580, "kaim_14173d580", fn_14173d580 as *const ()), // sub_14173d580
    Replacement::draft(0x14173d590, "kaim_14173d590", fn_14173d590 as *const ()), // sub_14173d590
    Replacement::draft(0x14173d5a0, "kaim_14173d5a0", fn_14173d5a0 as *const ()), // sub_14173d5a0
    Replacement::draft(0x14173d5b0, "kaim_14173d5b0", fn_14173d5b0 as *const ()), // sub_14173d5b0
    Replacement::draft(0x14173d5c0, "kaim_14173d5c0", fn_14173d5c0 as *const ()), // sub_14173d5c0
    Replacement::draft(0x14173d5d0, "kaim_14173d5d0", fn_14173d5d0 as *const ()), // sub_14173d5d0
    Replacement::draft(0x14173d5e0, "kaim_14173d5e0", fn_14173d5e0 as *const ()), // sub_14173d5e0
    Replacement::draft(0x14173d5f0, "kaim_14173d5f0", fn_14173d5f0 as *const ()), // sub_14173d5f0
    Replacement::draft(0x14173d600, "kaim_14173d600", fn_14173d600 as *const ()), // sub_14173d600
    Replacement::draft(0x14173d610, "kaim_14173d610", fn_14173d610 as *const ()), // sub_14173d610
    Replacement::draft(0x14173d620, "kaim_14173d620", fn_14173d620 as *const ()), // sub_14173d620
    Replacement::draft(0x14173d630, "kaim_14173d630", fn_14173d630 as *const ()), // sub_14173d630
    Replacement::draft(0x14173d640, "kaim_14173d640", fn_14173d640 as *const ()), // sub_14173d640
    Replacement::draft(0x14173d650, "kaim_14173d650", fn_14173d650 as *const ()), // sub_14173d650
    Replacement::draft(0x14173d660, "kaim_14173d660", fn_14173d660 as *const ()), // sub_14173d660
    Replacement::draft(0x14173d670, "kaim_14173d670", fn_14173d670 as *const ()), // sub_14173d670
    Replacement::draft(0x14173d680, "kaim_14173d680", fn_14173d680 as *const ()), // sub_14173d680
    Replacement::draft(0x14173d690, "kaim_14173d690", fn_14173d690 as *const ()), // sub_14173d690
    Replacement::draft(0x14173d6a0, "kaim_14173d6a0", fn_14173d6a0 as *const ()), // sub_14173d6a0
    Replacement::draft(0x14173d6b0, "kaim_14173d6b0", fn_14173d6b0 as *const ()), // sub_14173d6b0
    Replacement::draft(0x14173d6c0, "kaim_14173d6c0", fn_14173d6c0 as *const ()), // sub_14173d6c0
    Replacement::draft(0x14173d6d0, "kaim_14173d6d0", fn_14173d6d0 as *const ()), // sub_14173d6d0
    Replacement::draft(0x14173d6e0, "kaim_14173d6e0", fn_14173d6e0 as *const ()), // sub_14173d6e0
    Replacement::draft(0x14173d6f0, "kaim_14173d6f0", fn_14173d6f0 as *const ()), // sub_14173d6f0
    Replacement::draft(0x14173d700, "kaim_14173d700", fn_14173d700 as *const ()), // sub_14173d700
    Replacement::draft(0x14173d710, "kaim_14173d710", fn_14173d710 as *const ()), // sub_14173d710
    Replacement::draft(0x14173d720, "kaim_14173d720", fn_14173d720 as *const ()), // sub_14173d720
    Replacement::draft(0x14173d730, "kaim_14173d730", fn_14173d730 as *const ()), // sub_14173d730
    Replacement::draft(0x14173d740, "kaim_14173d740", fn_14173d740 as *const ()), // sub_14173d740
    Replacement::draft(0x14173d750, "kaim_14173d750", fn_14173d750 as *const ()), // sub_14173d750
    Replacement::draft(0x14173d760, "kaim_14173d760", fn_14173d760 as *const ()), // sub_14173d760
    Replacement::draft(0x14173d770, "kaim_14173d770", fn_14173d770 as *const ()), // sub_14173d770
    Replacement::draft(0x14173d780, "kaim_14173d780", tpl_1::<0x1426add78> as *const ()), // sub_14173d780
    Replacement::draft(0x14173d790, "kaim_14173d790", tpl_1::<0x1426addd0> as *const ()), // sub_14173d790
    Replacement::draft(0x14173d7a0, "kaim_14173d7a0", tpl_1::<0x1426adcf8> as *const ()), // sub_14173d7a0
    Replacement::draft(0x14173d7b0, "kaim_14173d7b0", tpl_1::<0x1426adcd0> as *const ()), // sub_14173d7b0
    Replacement::draft(0x14173d7c0, "kaim_14173d7c0", tpl_1::<0x1426adca0> as *const ()), // sub_14173d7c0
    Replacement::draft(0x14173d7d0, "kaim_14173d7d0", tpl_1::<0x1426adc78> as *const ()), // sub_14173d7d0
    Replacement::draft(0x14173d7e0, "kaim_14173d7e0", tpl_1::<0x1426add08> as *const ()), // sub_14173d7e0
    Replacement::draft(0x14173d7f0, "kaim_14173d7f0", tpl_1::<0x1426adef8> as *const ()), // sub_14173d7f0
    Replacement::draft(0x14173d800, "kaim_14173d800", tpl_1::<0x1426ade70> as *const ()), // sub_14173d800
    Replacement::draft(0x14173d810, "kaim_14173d810", tpl_1::<0x1426adf08> as *const ()), // sub_14173d810
    Replacement::draft(0x14173d820, "kaim_14173d820", tpl_1::<0x1426adec0> as *const ()), // sub_14173d820
    Replacement::draft(0x14173d830, "kaim_14173d830", tpl_1::<0x1426add28> as *const ()), // sub_14173d830
    Replacement::draft(0x14173d840, "kaim_14173d840", tpl_1::<0x1426adbf0> as *const ()), // sub_14173d840
    Replacement::draft(0x14173d850, "kaim_14173d850", tpl_1::<0x1426adde0> as *const ()), // sub_14173d850
    Replacement::draft(0x14173d860, "kaim_14173d860", tpl_1::<0x1426add68> as *const ()), // sub_14173d860
    Replacement::draft(0x14173d870, "kaim_14173d870", tpl_1::<0x1426ade38> as *const ()), // sub_14173d870
    Replacement::draft(0x14173d880, "kaim_14173d880", tpl_1::<0x1426add40> as *const ()), // sub_14173d880
    Replacement::draft(0x14173d890, "kaim_14173d890", tpl_1::<0x1426ade58> as *const ()), // sub_14173d890
    Replacement::draft(0x14173d8a0, "kaim_14173d8a0", tpl_1::<0x1426addf8> as *const ()), // sub_14173d8a0
    Replacement::draft(0x14173d8b0, "kaim_14173d8b0", tpl_1::<0x1426ade88> as *const ()), // sub_14173d8b0
    Replacement::draft(0x14173d8c0, "kaim_14173d8c0", tpl_1::<0x1426ade20> as *const ()), // sub_14173d8c0
    Replacement::draft(0x14173d8d0, "kaim_14173d8d0", tpl_1::<0x1426add50> as *const ()), // sub_14173d8d0
    Replacement::draft(0x14173d8e0, "kaim_14173d8e0", tpl_1::<0x1426ade48> as *const ()), // sub_14173d8e0
    Replacement::draft(0x14173d8f0, "kaim_14173d8f0", tpl_1::<0x1426adee0> as *const ()), // sub_14173d8f0
    Replacement::draft(0x14173d900, "kaim_14173d900", tpl_1::<0x1426add90> as *const ()), // sub_14173d900
    Replacement::draft(0x14173d910, "kaim_14173d910", tpl_1::<0x1426adc08> as *const ()), // sub_14173d910
    Replacement::draft(0x14173d920, "kaim_14173d920", tpl_1::<0x1426adc40> as *const ()), // sub_14173d920
    Replacement::draft(0x14173d930, "kaim_14173d930", tpl_1::<0x1426adc58> as *const ()), // sub_14173d930
    Replacement::draft(0x14173d940, "kaim_14173d940", tpl_1::<0x1426adc28> as *const ()), // sub_14173d940
    Replacement::draft(0x14173d950, "kaim_14173d950", tpl_1::<0x1426addb0> as *const ()), // sub_14173d950
    Replacement::draft(0x14173d960, "kaim_14173d960", tpl_1::<0x1426adea8> as *const ()), // sub_14173d960
    Replacement::draft(0x14173d970, "kaim_14173d970", tpl_1::<0x1426ade10> as *const ()), // sub_14173d970
    Replacement::draft(0x14173d980, "kaim_14173d980", fn_14173d980 as *const ()), // sub_14173d980
    Replacement::draft(0x14173d990, "kaim_14173d990", fn_14173d990 as *const ()), // sub_14173d990
    Replacement::draft(0x14173da40, "kaim_14173da40", fn_14173da40 as *const ()), // sub_14173da40
    Replacement::draft(0x14173dc20, "kaim_14173dc20", fn_14173dc20 as *const ()), // sub_14173dc20
    Replacement::draft(0x14173dd00, "kaim_14173dd00", fn_14173dd00 as *const ()), // sub_14173dd00
    Replacement::draft(0x14173ddc0, "kaim_14173ddc0", fn_14173ddc0 as *const ()), // sub_14173ddc0
    Replacement::draft(0x14173de90, "kaim_14173de90", fn_14173de90 as *const ()), // sub_14173de90
    Replacement::draft(0x14173df30, "kaim_14173df30", fn_14173df30 as *const ()), // sub_14173df30
    Replacement::draft(0x14173e030, "kaim_14173e030", fn_14173e030 as *const ()), // sub_14173e030
    Replacement::draft(0x14173e180, "kaim_14173e180", fn_14173e180 as *const ()), // sub_14173e180
    Replacement::draft(0x14173e280, "kaim_14173e280", fn_14173e280 as *const ()), // sub_14173e280
    Replacement::draft(0x14173e3d0, "kaim_14173e3d0", fn_14173e3d0 as *const ()), // sub_14173e3d0
    Replacement::draft(0x14173e530, "kaim_14173e530", fn_14173e530 as *const ()), // sub_14173e530
    Replacement::draft(0x14173e750, "kaim_14173e750", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aeef0> as *const ()), // sub_14173e750
    Replacement::draft(0x14173e7b0, "kaim_14173e7b0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aeda0> as *const ()), // sub_14173e7b0
    Replacement::draft(0x14173e810, "kaim_14173e810", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aef28> as *const ()), // sub_14173e810
    Replacement::draft(0x14173e870, "kaim_14173e870", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aef98> as *const ()), // sub_14173e870
    Replacement::draft(0x14173e8d0, "kaim_14173e8d0", tpl_22444::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426aef60> as *const ()), // sub_14173e8d0
    Replacement::draft(0x14173e930, "kaim_14173e930", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aee10, 0x8> as *const ()), // sub_14173e930
    Replacement::draft(0x14173e970, "kaim_14173e970", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aedd8, 0x8> as *const ()), // sub_14173e970
    Replacement::draft(0x14173e9b0, "kaim_14173e9b0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aee48, 0x8> as *const ()), // sub_14173e9b0
    Replacement::draft(0x14173e9f0, "kaim_14173e9f0", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aeeb8, 0x8> as *const ()), // sub_14173e9f0
    Replacement::draft(0x14173ea30, "kaim_14173ea30", tpl_22445::<0x143932190, 0x10, 0x48, 0x1426aee80, 0x8> as *const ()), // sub_14173ea30
    Replacement::draft(0x14173ea70, "kaim_14173ea70", fn_14173ea70 as *const ()), // sub_14173ea70
    Replacement::draft(0x14173eac0, "kaim_14173eac0", tpl_22446::<0x14173e3d0> as *const ()), // sub_14173eac0
    Replacement::draft(0x14173ead0, "kaim_14173ead0", tpl_22446::<0x14173eb80> as *const ()), // sub_14173ead0
    Replacement::draft(0x14173eae0, "kaim_14173eae0", fn_14173eae0 as *const ()), // sub_14173eae0
    Replacement::draft(0x14173eaf0, "kaim_14173eaf0", fn_14173eaf0 as *const ()), // sub_14173eaf0
    Replacement::draft(0x14173eb00, "kaim_14173eb00", fn_14173eb00 as *const ()), // sub_14173eb00
    Replacement::draft(0x14173eb10, "kaim_14173eb10", fn_14173eb10 as *const ()), // sub_14173eb10
    Replacement::draft(0x14173eb20, "kaim_14173eb20", fn_14173eb20 as *const ()), // sub_14173eb20
    Replacement::draft(0x14173eb30, "kaim_14173eb30", tpl_1::<0x1426aed38> as *const ()), // sub_14173eb30
    Replacement::draft(0x14173eb40, "kaim_14173eb40", tpl_1::<0x1426aed80> as *const ()), // sub_14173eb40
    Replacement::draft(0x14173eb50, "kaim_14173eb50", tpl_1::<0x1426aed48> as *const ()), // sub_14173eb50
    Replacement::draft(0x14173eb60, "kaim_14173eb60", tpl_1::<0x1426aed68> as *const ()), // sub_14173eb60
    Replacement::draft(0x14173eb70, "kaim_14173eb70", tpl_1::<0x1426aed58> as *const ()), // sub_14173eb70
    Replacement::draft(0x14173eb80, "kaim_14173eb80", fn_14173eb80 as *const ()), // sub_14173eb80
    Replacement::draft(0x14173ec40, "kaim_14173ec40", fn_14173ec40 as *const ()), // sub_14173ec40
    Replacement::draft(0x14173ecc0, "kaim_14173ecc0", fn_14173ecc0 as *const ()), // sub_14173ecc0
    Replacement::draft(0x14173ed00, "kaim_14173ed00", fn_14173ed00 as *const ()), // sub_14173ed00
    Replacement::draft(0x14173ef10, "kaim_14173ef10", fn_14173ef10 as *const ()), // sub_14173ef10
    Replacement::draft(0x14173f320, "kaim_14173f320", fn_14173f320 as *const ()), // sub_14173f320
    Replacement::draft(0x14173f520, "kaim_14173f520", fn_14173f520 as *const ()), // sub_14173f520
    Replacement::draft(0x14173f5b0, "kaim_14173f5b0", fn_14173f5b0 as *const ()), // sub_14173f5b0
    Replacement::draft(0x14173f620, "kaim_14173f620", fn_14173f620 as *const ()), // sub_14173f620
    Replacement::draft(0x14173f640, "kaim_14173f640", fn_14173f640 as *const ()), // sub_14173f640
    Replacement::draft(0x14173f6d0, "kaim_14173f6d0", fn_14173f6d0 as *const ()), // sub_14173f6d0
    Replacement::draft(0x14173f7c0, "kaim_14173f7c0", fn_14173f7c0 as *const ()), // sub_14173f7c0
    Replacement::draft(0x14173f910, "kaim_14173f910", fn_14173f910 as *const ()), // sub_14173f910
    Replacement::draft(0x14173fa20, "kaim_14173fa20", tpl_31671::<0x143932190, 0x30, 0x48, 0x1416a3130, 0x1426ad288> as *const ()), // sub_14173fa20
    Replacement::draft(0x14173fa80, "kaim_14173fa80", fn_14173fa80 as *const ()), // sub_14173fa80
    Replacement::draft(0x14173fc90, "kaim_14173fc90", fn_14173fc90 as *const ()), // sub_14173fc90
    Replacement::draft(0x14173fe10, "kaim_14173fe10", fn_14173fe10 as *const ()), // sub_14173fe10
    Replacement::draft(0x14173ff30, "kaim_14173ff30", fn_14173ff30 as *const ()), // sub_14173ff30
    Replacement::draft(0x1417401a0, "kaim_1417401a0", fn_1417401a0 as *const ()), // sub_1417401a0
    Replacement::draft(0x1417402c0, "kaim_1417402c0", fn_1417402c0 as *const ()), // sub_1417402c0
    Replacement::draft(0x141740430, "kaim_141740430", fn_141740430 as *const ()), // sub_141740430
    Replacement::draft(0x141740750, "kaim_141740750", fn_141740750 as *const ()), // sub_141740750
    Replacement::draft(0x1417409b0, "kaim_1417409b0", fn_1417409b0 as *const ()), // sub_1417409b0
    Replacement::draft(0x141740ae0, "kaim_141740ae0", fn_141740ae0 as *const ()), // sub_141740ae0
    Replacement::draft(0x141740bb0, "kaim_141740bb0", fn_141740bb0 as *const ()), // sub_141740bb0
    Replacement::draft(0x141740c80, "kaim_141740c80", fn_141740c80 as *const ()), // sub_141740c80
    Replacement::draft(0x141740d60, "kaim_141740d60", fn_141740d60 as *const ()), // sub_141740d60
    Replacement::draft(0x141740e30, "kaim_141740e30", fn_141740e30 as *const ()), // sub_141740e30
    Replacement::draft(0x141740ef0, "kaim_141740ef0", fn_141740ef0 as *const ()), // sub_141740ef0
    Replacement::draft(0x141740f80, "kaim_141740f80", fn_141740f80 as *const ()), // sub_141740f80
    Replacement::draft(0x1417410b0, "kaim_1417410b0", fn_1417410b0 as *const ()), // sub_1417410b0
    Replacement::draft(0x141741130, "kaim_141741130", fn_141741130 as *const ()), // sub_141741130
    Replacement::draft(0x141741160, "kaim_141741160", fn_141741160 as *const ()), // sub_141741160
    Replacement::draft(0x1417412b0, "kaim_1417412b0", fn_1417412b0 as *const ()), // sub_1417412b0
    Replacement::draft(0x1417412f0, "kaim_1417412f0", fn_1417412f0 as *const ()), // sub_1417412f0
    Replacement::draft(0x1417413a0, "kaim_1417413a0", fn_1417413a0 as *const ()), // sub_1417413a0
    Replacement::draft(0x1417413e0, "kaim_1417413e0", fn_1417413e0 as *const ()), // sub_1417413e0
    Replacement::draft(0x141741400, "kaim_141741400", fn_141741400 as *const ()), // sub_141741400
    Replacement::draft(0x1417415e0, "kaim_1417415e0", fn_1417415e0 as *const ()), // sub_1417415e0
    Replacement::draft(0x141741860, "kaim_141741860", fn_141741860 as *const ()), // sub_141741860
    Replacement::draft(0x1417420c0, "kaim_1417420c0", fn_1417420c0 as *const ()), // sub_1417420c0
    Replacement::draft(0x141742360, "kaim_141742360", fn_141742360 as *const ()), // sub_141742360
    Replacement::draft(0x1417426b0, "kaim_1417426b0", fn_1417426b0 as *const ()), // sub_1417426b0
    Replacement::draft(0x141742c30, "kaim_141742c30", fn_141742c30 as *const ()), // sub_141742c30
    Replacement::draft(0x141742d30, "kaim_141742d30", fn_141742d30 as *const ()), // sub_141742d30
    Replacement::draft(0x141742da0, "kaim_141742da0", tpl_548::<0x28> as *const ()), // sub_141742da0
    Replacement::draft(0x141742db0, "kaim_141742db0", fn_141742db0 as *const ()), // sub_141742db0
    Replacement::draft(0x141743110, "kaim_141743110", fn_141743110 as *const ()), // sub_141743110
    Replacement::draft(0x141743370, "kaim_141743370", fn_141743370 as *const ()), // sub_141743370
    Replacement::draft(0x141743640, "kaim_141743640", fn_141743640 as *const ()), // sub_141743640
    Replacement::draft(0x141743a30, "kaim_141743a30", fn_141743a30 as *const ()), // sub_141743a30
    Replacement::draft(0x141743ae0, "kaim_141743ae0", fn_141743ae0 as *const ()), // sub_141743ae0
    Replacement::draft(0x141743bb0, "kaim_141743bb0", fn_141743bb0 as *const ()), // sub_141743bb0
    Replacement::draft(0x141743c10, "kaim_141743c10", fn_141743c10 as *const ()), // sub_141743c10
    Replacement::draft(0x141743de0, "kaim_141743de0", fn_141743de0 as *const ()), // sub_141743de0
    Replacement::draft(0x141744160, "kaim_141744160", fn_141744160 as *const ()), // sub_141744160
    Replacement::draft(0x1417448d0, "kaim_1417448d0", fn_1417448d0 as *const ()), // sub_1417448d0
    Replacement::draft(0x141744c00, "kaim_141744c00", fn_141744c00 as *const ()), // sub_141744c00
    Replacement::draft(0x141745070, "kaim_141745070", fn_141745070 as *const ()), // sub_141745070
    Replacement::draft(0x141745120, "kaim_141745120", fn_141745120 as *const ()), // sub_141745120
    Replacement::draft(0x141745190, "kaim_141745190", fn_141745190 as *const ()), // sub_141745190
    Replacement::draft(0x141745220, "kaim_141745220", fn_141745220 as *const ()), // sub_141745220
    Replacement::draft(0x1417453b0, "kaim_1417453b0", fn_1417453b0 as *const ()), // sub_1417453b0
    Replacement::draft(0x1417454f0, "kaim_1417454f0", fn_1417454f0 as *const ()), // sub_1417454f0
    Replacement::draft(0x141745550, "kaim_141745550", fn_141745550 as *const ()), // sub_141745550
    Replacement::draft(0x141745600, "kaim_141745600", fn_141745600 as *const ()), // sub_141745600
    Replacement::draft(0x1417456a0, "kaim_1417456a0", fn_1417456a0 as *const ()), // sub_1417456a0
    Replacement::draft(0x1417456f0, "kaim_1417456f0", fn_1417456f0 as *const ()), // sub_1417456f0
    Replacement::draft(0x141745780, "kaim_141745780", fn_141745780 as *const ()), // sub_141745780
    Replacement::draft(0x1417457f0, "kaim_1417457f0", fn_1417457f0 as *const ()), // sub_1417457f0
    Replacement::draft(0x141745810, "kaim_141745810", fn_141745810 as *const ()), // sub_141745810
    Replacement::draft(0x1417458c0, "kaim_1417458c0", fn_1417458c0 as *const ()), // sub_1417458c0
    Replacement::draft(0x141745a30, "kaim_141745a30", fn_141745a30 as *const ()), // sub_141745a30
    Replacement::draft(0x141745cd0, "kaim_141745cd0", fn_141745cd0 as *const ()), // sub_141745cd0
    Replacement::draft(0x141745d70, "kaim_141745d70", fn_141745d70 as *const ()), // sub_141745d70
    Replacement::draft(0x141745e60, "kaim_141745e60", fn_141745e60 as *const ()), // sub_141745e60
    Replacement::draft(0x141745e80, "kaim_141745e80", fn_141745e80 as *const ()), // sub_141745e80
    Replacement::draft(0x141745ec0, "kaim_141745ec0", fn_141745ec0 as *const ()), // sub_141745ec0
    Replacement::draft(0x141745f50, "kaim_141745f50", fn_141745f50 as *const ()), // sub_141745f50
    Replacement::draft(0x141746130, "kaim_141746130", fn_141746130 as *const ()), // sub_141746130
    Replacement::draft(0x1417461c0, "kaim_1417461c0", fn_1417461c0 as *const ()), // sub_1417461c0
    Replacement::draft(0x141746280, "kaim_141746280", fn_141746280 as *const ()), // sub_141746280
    Replacement::draft(0x1417463d0, "kaim_1417463d0", fn_1417463d0 as *const ()), // sub_1417463d0
    Replacement::draft(0x1417464f0, "kaim_1417464f0", fn_1417464f0 as *const ()), // sub_1417464f0
    Replacement::draft(0x141746570, "kaim_141746570", tpl_31780::<0x1417464f0, 0x143932190, 0x60, 0x1401de410> as *const ()), // sub_141746570
    Replacement::draft(0x1417465d0, "kaim_1417465d0", fn_1417465d0 as *const ()), // sub_1417465d0
    Replacement::draft(0x1417466f0, "kaim_1417466f0", fn_1417466f0 as *const ()), // sub_1417466f0
    Replacement::draft(0x141746700, "kaim_141746700", fn_141746700 as *const ()), // sub_141746700
    Replacement::draft(0x141746770, "kaim_141746770", fn_141746770 as *const ()), // sub_141746770
    Replacement::draft(0x1417467d0, "kaim_1417467d0", fn_1417467d0 as *const ()), // sub_1417467d0
    Replacement::draft(0x141746880, "kaim_141746880", fn_141746880 as *const ()), // sub_141746880
    Replacement::draft(0x141746910, "kaim_141746910", fn_141746910 as *const ()), // sub_141746910
    Replacement::draft(0x1417469b0, "kaim_1417469b0", fn_1417469b0 as *const ()), // sub_1417469b0
    Replacement::draft(0x141746a00, "kaim_141746a00", fn_141746a00 as *const ()), // sub_141746a00
    Replacement::draft(0x141746a30, "kaim_141746a30", fn_141746a30 as *const ()), // sub_141746a30
    Replacement::draft(0x141746a80, "kaim_141746a80", fn_141746a80 as *const ()), // sub_141746a80
    Replacement::draft(0x141746b30, "kaim_141746b30", fn_141746b30 as *const ()), // sub_141746b30
    Replacement::draft(0x141746b50, "kaim_141746b50", fn_141746b50 as *const ()), // sub_141746b50
    Replacement::draft(0x141746bc0, "kaim_141746bc0", fn_141746bc0 as *const ()), // sub_141746bc0
    Replacement::draft(0x141746c10, "kaim_141746c10", fn_141746c10 as *const ()), // sub_141746c10
    Replacement::draft(0x141746c60, "kaim_141746c60", fn_141746c60 as *const ()), // sub_141746c60
    Replacement::draft(0x141746d30, "kaim_141746d30", fn_141746d30 as *const ()), // sub_141746d30
    Replacement::draft(0x141746d80, "kaim_141746d80", fn_141746d80 as *const ()), // sub_141746d80
    Replacement::draft(0x141746ed0, "kaim_141746ed0", fn_141746ed0 as *const ()), // sub_141746ed0
    Replacement::draft(0x141746fa0, "kaim_141746fa0", fn_141746fa0 as *const ()), // sub_141746fa0
    Replacement::draft(0x141747010, "kaim_141747010", fn_141747010 as *const ()), // sub_141747010
    Replacement::draft(0x141747050, "kaim_141747050", fn_141747050 as *const ()), // sub_141747050
    Replacement::draft(0x141747160, "kaim_141747160", fn_141747160 as *const ()), // sub_141747160
    Replacement::draft(0x141747170, "kaim_141747170", fn_141747170 as *const ()), // sub_141747170
    Replacement::draft(0x141747270, "kaim_141747270", fn_141747270 as *const ()), // sub_141747270
    Replacement::draft(0x141747320, "kaim_141747320", fn_141747320 as *const ()), // sub_141747320
    Replacement::draft(0x141747330, "kaim_141747330", fn_141747330 as *const ()), // sub_141747330
    Replacement::draft(0x1417473c0, "kaim_1417473c0", fn_1417473c0 as *const ()), // sub_1417473c0
    Replacement::draft(0x141747450, "kaim_141747450", tpl_32881::<0x20, 0x142646898> as *const ()), // sub_141747450
    Replacement::draft(0x141747470, "kaim_141747470", tpl_1691::<0x2c> as *const ()), // sub_141747470
    Replacement::draft(0x141747480, "kaim_141747480", fn_141747480 as *const ()), // sub_141747480
    Replacement::draft(0x141747490, "kaim_141747490", fn_141747490 as *const ()), // sub_141747490
    Replacement::draft(0x141747500, "kaim_141747500", fn_141747500 as *const ()), // sub_141747500
    Replacement::draft(0x141747530, "kaim_141747530", tpl_32884::<0x30> as *const ()), // sub_141747530
    Replacement::draft(0x141747550, "kaim_141747550", tpl_32884::<0x70> as *const ()), // sub_141747550
    Replacement::draft(0x141747570, "kaim_141747570", fn_141747570 as *const ()), // sub_141747570
    Replacement::draft(0x141747600, "kaim_141747600", fn_141747600 as *const ()), // sub_141747600
    Replacement::draft(0x1417476b0, "kaim_1417476b0", fn_1417476b0 as *const ()), // sub_1417476b0
    Replacement::draft(0x141747730, "kaim_141747730", fn_141747730 as *const ()), // sub_141747730
    Replacement::draft(0x1417477a0, "kaim_1417477a0", fn_1417477a0 as *const ()), // sub_1417477a0
    Replacement::draft(0x141747830, "kaim_141747830", fn_141747830 as *const ()), // sub_141747830
    Replacement::draft(0x1417478e0, "kaim_1417478e0", fn_1417478e0 as *const ()), // sub_1417478e0
    Replacement::draft(0x141747aa0, "kaim_141747aa0", fn_141747aa0 as *const ()), // sub_141747aa0
    Replacement::draft(0x141747cb0, "kaim_141747cb0", fn_141747cb0 as *const ()), // sub_141747cb0
    Replacement::draft(0x141747d10, "kaim_141747d10", fn_141747d10 as *const ()), // sub_141747d10
    Replacement::draft(0x141747da0, "kaim_141747da0", fn_141747da0 as *const ()), // sub_141747da0
    Replacement::draft(0x141747f40, "kaim_141747f40", fn_141747f40 as *const ()), // sub_141747f40
    Replacement::draft(0x1417482b0, "kaim_1417482b0", fn_1417482b0 as *const ()), // sub_1417482b0
    Replacement::draft(0x141748590, "kaim_141748590", fn_141748590 as *const ()), // sub_141748590
    Replacement::draft(0x1417486e0, "kaim_1417486e0", fn_1417486e0 as *const ()), // sub_1417486e0
    Replacement::draft(0x141748760, "kaim_141748760", fn_141748760 as *const ()), // sub_141748760
    Replacement::draft(0x1417488f0, "kaim_1417488f0", fn_1417488f0 as *const ()), // sub_1417488f0
    Replacement::draft(0x1417489b0, "kaim_1417489b0", fn_1417489b0 as *const ()), // sub_1417489b0
    Replacement::draft(0x141748a40, "kaim_141748a40", fn_141748a40 as *const ()), // sub_141748a40
    Replacement::draft(0x141748bc0, "kaim_141748bc0", fn_141748bc0 as *const ()), // sub_141748bc0
    Replacement::draft(0x141748c10, "kaim_141748c10", fn_141748c10 as *const ()), // sub_141748c10
    Replacement::draft(0x141748cf0, "kaim_141748cf0", fn_141748cf0 as *const ()), // sub_141748cf0
    Replacement::draft(0x141748df0, "kaim_141748df0", fn_141748df0 as *const ()), // sub_141748df0
    Replacement::draft(0x1417490c0, "kaim_1417490c0", fn_1417490c0 as *const ()), // sub_1417490c0
    Replacement::draft(0x141749170, "kaim_141749170", fn_141749170 as *const ()), // sub_141749170
    Replacement::draft(0x1417492c0, "kaim_1417492c0", fn_1417492c0 as *const ()), // sub_1417492c0
    Replacement::draft(0x1417492f0, "kaim_1417492f0", fn_1417492f0 as *const ()), // sub_1417492f0
    Replacement::draft(0x1417493b0, "kaim_1417493b0", fn_1417493b0 as *const ()), // sub_1417493b0
    Replacement::draft(0x1417493d0, "kaim_1417493d0", fn_1417493d0 as *const ()), // sub_1417493d0
    Replacement::draft(0x141749570, "kaim_141749570", fn_141749570 as *const ()), // sub_141749570
    Replacement::draft(0x1417498b0, "kaim_1417498b0", fn_1417498b0 as *const ()), // sub_1417498b0
    Replacement::draft(0x141749ef0, "kaim_141749ef0", fn_141749ef0 as *const ()), // sub_141749ef0
    Replacement::draft(0x141749f60, "kaim_141749f60", fn_141749f60 as *const ()), // sub_141749f60
    Replacement::draft(0x141749fd0, "kaim_141749fd0", fn_141749fd0 as *const ()), // sub_141749fd0
    Replacement::draft(0x14174a3b0, "kaim_14174a3b0", fn_14174a3b0 as *const ()), // sub_14174a3b0
    Replacement::draft(0x14174a730, "kaim_14174a730", fn_14174a730 as *const ()), // sub_14174a730
    Replacement::draft(0x14174a800, "kaim_14174a800", fn_14174a800 as *const ()), // sub_14174a800
    Replacement::draft(0x14174a8e0, "kaim_14174a8e0", fn_14174a8e0 as *const ()), // sub_14174a8e0
    Replacement::draft(0x14174aa90, "kaim_14174aa90", fn_14174aa90 as *const ()), // sub_14174aa90
    Replacement::draft(0x14174aaf0, "kaim_14174aaf0", fn_14174aaf0 as *const ()), // sub_14174aaf0
    Replacement::draft(0x14174afa0, "kaim_14174afa0", fn_14174afa0 as *const ()), // sub_14174afa0
    Replacement::draft(0x14174b3f0, "kaim_14174b3f0", fn_14174b3f0 as *const ()), // sub_14174b3f0
    Replacement::draft(0x14174b480, "kaim_14174b480", fn_14174b480 as *const ()), // sub_14174b480
    Replacement::draft(0x14174b510, "kaim_14174b510", tpl_365::<0x1416a5d50> as *const ()), // sub_14174b510
    Replacement::draft(0x14174b520, "kaim_14174b520", fn_14174b520 as *const ()), // sub_14174b520
    Replacement::draft(0x14174b730, "kaim_14174b730", fn_14174b730 as *const ()), // sub_14174b730
    Replacement::draft(0x14174b810, "kaim_14174b810", fn_14174b810 as *const ()), // sub_14174b810
    Replacement::draft(0x14174c410, "kaim_14174c410", fn_14174c410 as *const ()), // sub_14174c410
    Replacement::draft(0x14174c450, "kaim_14174c450", fn_14174c450 as *const ()), // sub_14174c450
    Replacement::draft(0x14174c9b0, "kaim_14174c9b0", fn_14174c9b0 as *const ()), // sub_14174c9b0
    Replacement::draft(0x14174cae0, "kaim_14174cae0", fn_14174cae0 as *const ()), // sub_14174cae0
    Replacement::draft(0x14174cc70, "kaim_14174cc70", fn_14174cc70 as *const ()), // sub_14174cc70
    Replacement::draft(0x14174cdf0, "kaim_14174cdf0", fn_14174cdf0 as *const ()), // sub_14174cdf0
    Replacement::draft(0x14174e320, "kaim_14174e320", fn_14174e320 as *const ()), // sub_14174e320
    Replacement::draft(0x14174e390, "kaim_14174e390", fn_14174e390 as *const ()), // sub_14174e390
    Replacement::draft(0x14174ec50, "kaim_14174ec50", fn_14174ec50 as *const ()), // sub_14174ec50
    Replacement::draft(0x14174f070, "kaim_14174f070", fn_14174f070 as *const ()), // sub_14174f070
    Replacement::draft(0x14174f0a0, "kaim_14174f0a0", fn_14174f0a0 as *const ()), // sub_14174f0a0
    Replacement::draft(0x14174f340, "kaim_14174f340", fn_14174f340 as *const ()), // sub_14174f340
    Replacement::draft(0x14174f510, "kaim_14174f510", fn_14174f510 as *const ()), // sub_14174f510
    Replacement::draft(0x14174f750, "kaim_14174f750", fn_14174f750 as *const ()), // sub_14174f750
    Replacement::draft(0x14174f850, "kaim_14174f850", fn_14174f850 as *const ()), // sub_14174f850
    Replacement::draft(0x14174f900, "kaim_14174f900", fn_14174f900 as *const ()), // sub_14174f900
    Replacement::draft(0x14174fbf0, "kaim_14174fbf0", fn_14174fbf0 as *const ()), // sub_14174fbf0
    Replacement::draft(0x14174fc50, "kaim_14174fc50", fn_14174fc50 as *const ()), // sub_14174fc50
    Replacement::draft(0x14174fd00, "kaim_14174fd00", fn_14174fd00 as *const ()), // sub_14174fd00
    Replacement::draft(0x14174fdb0, "kaim_14174fdb0", fn_14174fdb0 as *const ()), // sub_14174fdb0
    Replacement::draft(0x14174fe60, "kaim_14174fe60", fn_14174fe60 as *const ()), // sub_14174fe60
    Replacement::draft(0x14174ff10, "kaim_14174ff10", fn_14174ff10 as *const ()), // sub_14174ff10
    Replacement::draft(0x14174ffc0, "kaim_14174ffc0", fn_14174ffc0 as *const ()), // sub_14174ffc0
    Replacement::draft(0x1417500b0, "kaim_1417500b0", tpl_32166::<0x8, 0xffffffffffffffff, 0x14169edd0, 0x10, 0x1416a2ea0, 0x14183acc2> as *const ()), // sub_1417500b0
    Replacement::draft(0x141750170, "kaim_141750170", fn_141750170 as *const ()), // sub_141750170
    Replacement::draft(0x141750280, "kaim_141750280", fn_141750280 as *const ()), // sub_141750280
    Replacement::draft(0x141750340, "kaim_141750340", fn_141750340 as *const ()), // sub_141750340
    Replacement::draft(0x1417503e0, "kaim_1417503e0", fn_1417503e0 as *const ()), // sub_1417503e0
    Replacement::draft(0x1417504b0, "kaim_1417504b0", fn_1417504b0 as *const ()), // sub_1417504b0
    Replacement::draft(0x1417507a0, "kaim_1417507a0", fn_1417507a0 as *const ()), // sub_1417507a0
    Replacement::draft(0x141750960, "kaim_141750960", fn_141750960 as *const ()), // sub_141750960
    Replacement::draft(0x141750d40, "kaim_141750d40", fn_141750d40 as *const ()), // sub_141750d40
    Replacement::draft(0x141750da0, "kaim_141750da0", fn_141750da0 as *const ()), // sub_141750da0
    Replacement::draft(0x141750e40, "kaim_141750e40", fn_141750e40 as *const ()), // sub_141750e40
    Replacement::draft(0x141750f50, "kaim_141750f50", fn_141750f50 as *const ()), // sub_141750f50
    Replacement::draft(0x141750f70, "kaim_141750f70", fn_141750f70 as *const ()), // sub_141750f70
    Replacement::draft(0x1417515c0, "kaim_1417515c0", fn_1417515c0 as *const ()), // sub_1417515c0
    Replacement::draft(0x141751b70, "kaim_141751b70", fn_141751b70 as *const ()), // sub_141751b70
    Replacement::draft(0x141752100, "kaim_141752100", fn_141752100 as *const ()), // sub_141752100
    Replacement::draft(0x141752110, "kaim_141752110", fn_141752110 as *const ()), // sub_141752110
    Replacement::draft(0x141752130, "kaim_141752130", fn_141752130 as *const ()), // sub_141752130
    Replacement::draft(0x141752650, "kaim_141752650", fn_141752650 as *const ()), // sub_141752650
    Replacement::draft(0x1417526a0, "kaim_1417526a0", fn_1417526a0 as *const ()), // sub_1417526a0
    Replacement::draft(0x141752750, "kaim_141752750", fn_141752750 as *const ()), // sub_141752750
    Replacement::draft(0x1417527f0, "kaim_1417527f0", tpl_32490::<0x10, 0x18, 0x8, 0x14174fd00> as *const ()), // sub_1417527f0
    Replacement::draft(0x141752860, "kaim_141752860", fn_141752860 as *const ()), // sub_141752860
    Replacement::draft(0x141752b40, "kaim_141752b40", fn_141752b40 as *const ()), // sub_141752b40
    Replacement::draft(0x141752b60, "kaim_141752b60", fn_141752b60 as *const ()), // sub_141752b60
    Replacement::draft(0x141752c00, "kaim_141752c00", fn_141752c00 as *const ()), // sub_141752c00
    Replacement::draft(0x141752da0, "kaim_141752da0", fn_141752da0 as *const ()), // sub_141752da0
    Replacement::draft(0x141752df0, "kaim_141752df0", tpl_32166::<0x8, 0xffffffffffffffff, 0x14169edd0, 0x10, 0x1417356c0, 0x14183acc2> as *const ()), // sub_141752df0
    Replacement::draft(0x141752eb0, "kaim_141752eb0", fn_141752eb0 as *const ()), // sub_141752eb0
    Replacement::draft(0x141752f40, "kaim_141752f40", fn_141752f40 as *const ()), // sub_141752f40
    Replacement::draft(0x141752fb0, "kaim_141752fb0", fn_141752fb0 as *const ()), // sub_141752fb0
    Replacement::draft(0x141753050, "kaim_141753050", fn_141753050 as *const ()), // sub_141753050
    Replacement::draft(0x141753170, "kaim_141753170", fn_141753170 as *const ()), // sub_141753170
    Replacement::draft(0x1417532b0, "kaim_1417532b0", fn_1417532b0 as *const ()), // sub_1417532b0
    Replacement::draft(0x141753510, "kaim_141753510", fn_141753510 as *const ()), // sub_141753510
    Replacement::draft(0x1417535e0, "kaim_1417535e0", fn_1417535e0 as *const ()), // sub_1417535e0
    Replacement::draft(0x1417538d0, "kaim_1417538d0", fn_1417538d0 as *const ()), // sub_1417538d0
    Replacement::draft(0x141753b30, "kaim_141753b30", fn_141753b30 as *const ()), // sub_141753b30
    Replacement::draft(0x141753dc0, "kaim_141753dc0", fn_141753dc0 as *const ()), // sub_141753dc0
    Replacement::draft(0x141753e60, "kaim_141753e60", fn_141753e60 as *const ()), // sub_141753e60
    Replacement::draft(0x141753f10, "kaim_141753f10", fn_141753f10 as *const ()), // sub_141753f10
    Replacement::draft(0x141753fb0, "kaim_141753fb0", tpl_365::<0x141753f10> as *const ()), // sub_141753fb0
    Replacement::draft(0x141753fc0, "kaim_141753fc0", fn_141753fc0 as *const ()), // sub_141753fc0
    Replacement::draft(0x1417541c0, "kaim_1417541c0", fn_1417541c0 as *const ()), // sub_1417541c0
    Replacement::draft(0x141754220, "kaim_141754220", fn_141754220 as *const ()), // sub_141754220
    Replacement::draft(0x141754390, "kaim_141754390", fn_141754390 as *const ()), // sub_141754390
    Replacement::draft(0x1417544d0, "unknown_1417544d0", fn_1417544d0 as *const ()), // sub_1417544d0
    Replacement::draft(0x1417547b0, "unknown_1417547b0", fn_1417547b0 as *const ()), // sub_1417547b0
    Replacement::draft(0x141754870, "unknown_141754870", fn_141754870 as *const ()), // sub_141754870
    Replacement::draft(0x141754880, "unknown_141754880", fn_141754880 as *const ()), // sub_141754880
    Replacement::draft(0x141754940, "unknown_141754940", tpl_26861::<0x14176ae10> as *const ()), // sub_141754940
    Replacement::draft(0x141754960, "unknown_141754960", fn_141754960 as *const ()), // sub_141754960
    Replacement::draft(0x141754b60, "unknown_141754b60", fn_141754b60 as *const ()), // sub_141754b60
    Replacement::draft(0x141754cc0, "unknown_141754cc0", fn_141754cc0 as *const ()), // sub_141754cc0
    Replacement::draft(0x141754e60, "unknown_141754e60", fn_141754e60 as *const ()), // sub_141754e60
    Replacement::draft(0x141754f50, "unknown_141754f50", fn_141754f50 as *const ()), // sub_141754f50
    Replacement::draft(0x141754f90, "unknown_141754f90", fn_141754f90 as *const ()), // sub_141754f90
    Replacement::draft(0x141755230, "unknown_141755230", fn_141755230 as *const ()), // sub_141755230
    Replacement::draft(0x141755290, "unknown_141755290", fn_141755290 as *const ()), // sub_141755290
    Replacement::draft(0x141755510, "unknown_141755510", fn_141755510 as *const ()), // sub_141755510
    Replacement::draft(0x1417557a0, "unknown_1417557a0", fn_1417557a0 as *const ()), // sub_1417557a0
    Replacement::draft(0x141755800, "unknown_141755800", fn_141755800 as *const ()), // sub_141755800
    Replacement::draft(0x1417558b0, "unknown_1417558b0", fn_1417558b0 as *const ()), // sub_1417558b0
    Replacement::draft(0x141755970, "unknown_141755970", fn_141755970 as *const ()), // sub_141755970
    Replacement::draft(0x141755a70, "unknown_141755a70", fn_141755a70 as *const ()), // sub_141755a70
    Replacement::draft(0x141755b20, "unknown_141755b20", fn_141755b20 as *const ()), // sub_141755b20
    Replacement::draft(0x141755ba0, "unknown_141755ba0", fn_141755ba0 as *const ()), // sub_141755ba0
    Replacement::draft(0x141755ec0, "unknown_141755ec0", fn_141755ec0 as *const ()), // sub_141755ec0
    Replacement::draft(0x141755f20, "unknown_141755f20", fn_141755f20 as *const ()), // sub_141755f20
    Replacement::draft(0x141755f50, "unknown_141755f50", fn_141755f50 as *const ()), // sub_141755f50
    Replacement::draft(0x1417563c0, "unknown_1417563c0", fn_1417563c0 as *const ()), // sub_1417563c0
    Replacement::draft(0x141756790, "unknown_141756790", fn_141756790 as *const ()), // sub_141756790
    Replacement::draft(0x141756b20, "unknown_141756b20", fn_141756b20 as *const ()), // sub_141756b20
    Replacement::draft(0x141756ca0, "unknown_141756ca0", fn_141756ca0 as *const ()), // sub_141756ca0
    Replacement::draft(0x141757300, "unknown_141757300", fn_141757300 as *const ()), // sub_141757300
    Replacement::draft(0x141757510, "unknown_141757510", fn_141757510 as *const ()), // sub_141757510
    Replacement::draft(0x1417576c0, "unknown_1417576c0", fn_1417576c0 as *const ()), // sub_1417576c0
    Replacement::draft(0x141757770, "unknown_141757770", fn_141757770 as *const ()), // sub_141757770
    Replacement::draft(0x141757880, "unknown_141757880", fn_141757880 as *const ()), // sub_141757880
    Replacement::draft(0x141757cd0, "unknown_141757cd0", fn_141757cd0 as *const ()), // sub_141757cd0
    Replacement::draft(0x141757d60, "unknown_141757d60", fn_141757d60 as *const ()), // sub_141757d60
    Replacement::draft(0x1417580d0, "unknown_1417580d0", tpl_365::<0x1417583e0> as *const ()), // sub_1417580d0
    Replacement::draft(0x1417580e0, "unknown_1417580e0", fn_1417580e0 as *const ()), // sub_1417580e0
    Replacement::draft(0x1417583e0, "unknown_1417583e0", fn_1417583e0 as *const ()), // sub_1417583e0
    Replacement::draft(0x141758470, "unknown_141758470", fn_141758470 as *const ()), // sub_141758470
    Replacement::draft(0x1417584f0, "unknown_1417584f0", fn_1417584f0 as *const ()), // sub_1417584f0
    Replacement::draft(0x1417586a0, "unknown_1417586a0", fn_1417586a0 as *const ()), // sub_1417586a0
    Replacement::draft(0x1417589b0, "unknown_1417589b0", fn_1417589b0 as *const ()), // sub_1417589b0
    Replacement::draft(0x141759e10, "unknown_141759e10", fn_141759e10 as *const ()), // sub_141759e10
    Replacement::draft(0x14175a010, "unknown_14175a010", fn_14175a010 as *const ()), // sub_14175a010
    Replacement::draft(0x14175a340, "unknown_14175a340", fn_14175a340 as *const ()), // sub_14175a340
    Replacement::draft(0x14175a5e0, "unknown_14175a5e0", fn_14175a5e0 as *const ()), // sub_14175a5e0
    Replacement::draft(0x14175a7c0, "unknown_14175a7c0", fn_14175a7c0 as *const ()), // sub_14175a7c0
    Replacement::draft(0x14175a830, "unknown_14175a830", fn_14175a830 as *const ()), // sub_14175a830
    Replacement::draft(0x14175aa30, "unknown_14175aa30", fn_14175aa30 as *const ()), // sub_14175aa30
    Replacement::draft(0x14175ac40, "unknown_14175ac40", fn_14175ac40 as *const ()), // sub_14175ac40
    Replacement::draft(0x14175ae40, "unknown_14175ae40", fn_14175ae40 as *const ()), // sub_14175ae40
    Replacement::draft(0x14175b040, "unknown_14175b040", fn_14175b040 as *const ()), // sub_14175b040
    Replacement::draft(0x14175b210, "unknown_14175b210", fn_14175b210 as *const ()), // sub_14175b210
    Replacement::draft(0x14175b2c0, "unknown_14175b2c0", fn_14175b2c0 as *const ()), // sub_14175b2c0
    Replacement::draft(0x14175b370, "unknown_14175b370", fn_14175b370 as *const ()), // sub_14175b370
    Replacement::draft(0x14175b3d0, "unknown_14175b3d0", fn_14175b3d0 as *const ()), // sub_14175b3d0
    Replacement::draft(0x14175b680, "unknown_14175b680", fn_14175b680 as *const ()), // sub_14175b680
    Replacement::draft(0x14175b7e0, "unknown_14175b7e0", fn_14175b7e0 as *const ()), // sub_14175b7e0
    Replacement::draft(0x14175bb40, "unknown_14175bb40", fn_14175bb40 as *const ()), // sub_14175bb40
    Replacement::draft(0x14175bbe0, "unknown_14175bbe0", fn_14175bbe0 as *const ()), // sub_14175bbe0
    Replacement::draft(0x14175bc60, "unknown_14175bc60", fn_14175bc60 as *const ()), // sub_14175bc60
    Replacement::draft(0x14175bce0, "unknown_14175bce0", fn_14175bce0 as *const ()), // sub_14175bce0
    Replacement::draft(0x14175bd20, "unknown_14175bd20", tpl_31525::<0x1416a5fe0, 0x18, 0x143932190, 0x60> as *const ()), // sub_14175bd20
    Replacement::draft(0x14175bd70, "unknown_14175bd70", fn_14175bd70 as *const ()), // sub_14175bd70
    Replacement::draft(0x14175c240, "unknown_14175c240", fn_14175c240 as *const ()), // sub_14175c240
    Replacement::draft(0x14175c300, "unknown_14175c300", fn_14175c300 as *const ()), // sub_14175c300
    Replacement::draft(0x14175c870, "unknown_14175c870", fn_14175c870 as *const ()), // sub_14175c870
    Replacement::draft(0x14175cb50, "unknown_14175cb50", fn_14175cb50 as *const ()), // sub_14175cb50
    Replacement::draft(0x14175cbe0, "unknown_14175cbe0", fn_14175cbe0 as *const ()), // sub_14175cbe0
    Replacement::draft(0x14175d450, "unknown_14175d450", fn_14175d450 as *const ()), // sub_14175d450
    Replacement::draft(0x14175d670, "unknown_14175d670", fn_14175d670 as *const ()), // sub_14175d670
    Replacement::draft(0x14175d720, "unknown_14175d720", fn_14175d720 as *const ()), // sub_14175d720
    Replacement::draft(0x14175d7c0, "unknown_14175d7c0", fn_14175d7c0 as *const ()), // sub_14175d7c0
    Replacement::draft(0x14175d800, "unknown_14175d800", fn_14175d800 as *const ()), // sub_14175d800
    Replacement::draft(0x14175e080, "unknown_14175e080", fn_14175e080 as *const ()), // sub_14175e080
    Replacement::draft(0x14175e130, "unknown_14175e130", fn_14175e130 as *const ()), // sub_14175e130
    Replacement::draft(0x14175e280, "unknown_14175e280", fn_14175e280 as *const ()), // sub_14175e280
    Replacement::draft(0x14175e320, "unknown_14175e320", fn_14175e320 as *const ()), // sub_14175e320
    Replacement::draft(0x14175e450, "unknown_14175e450", fn_14175e450 as *const ()), // sub_14175e450
    Replacement::draft(0x14175e710, "unknown_14175e710", fn_14175e710 as *const ()), // sub_14175e710
    Replacement::draft(0x14175e7a0, "unknown_14175e7a0", fn_14175e7a0 as *const ()), // sub_14175e7a0
    Replacement::draft(0x14175e860, "unknown_14175e860", fn_14175e860 as *const ()), // sub_14175e860
    Replacement::draft(0x14175e9e0, "unknown_14175e9e0", fn_14175e9e0 as *const ()), // sub_14175e9e0
    Replacement::draft(0x14175ea50, "unknown_14175ea50", fn_14175ea50 as *const ()), // sub_14175ea50
    Replacement::draft(0x14175ead0, "unknown_14175ead0", fn_14175ead0 as *const ()), // sub_14175ead0
    Replacement::draft(0x14175eb30, "unknown_14175eb30", fn_14175eb30 as *const ()), // sub_14175eb30
    Replacement::draft(0x14175eb60, "unknown_14175eb60", fn_14175eb60 as *const ()), // sub_14175eb60
    Replacement::draft(0x14175ec40, "unknown_14175ec40", fn_14175ec40 as *const ()), // sub_14175ec40
    Replacement::draft(0x14175ece0, "unknown_14175ece0", fn_14175ece0 as *const ()), // sub_14175ece0
    Replacement::draft(0x14175ed80, "unknown_14175ed80", fn_14175ed80 as *const ()), // sub_14175ed80
    Replacement::draft(0x14175ee50, "unknown_14175ee50", fn_14175ee50 as *const ()), // sub_14175ee50
    Replacement::draft(0x14175f420, "unknown_14175f420", fn_14175f420 as *const ()), // sub_14175f420
    Replacement::draft(0x14175f660, "unknown_14175f660", fn_14175f660 as *const ()), // sub_14175f660
    Replacement::draft(0x14175f820, "unknown_14175f820", fn_14175f820 as *const ()), // sub_14175f820
    Replacement::draft(0x14175fbb0, "unknown_14175fbb0", fn_14175fbb0 as *const ()), // sub_14175fbb0
    Replacement::draft(0x141760150, "unknown_141760150", fn_141760150 as *const ()), // sub_141760150
    Replacement::draft(0x141760310, "unknown_141760310", fn_141760310 as *const ()), // sub_141760310
    Replacement::draft(0x141760500, "unknown_141760500", fn_141760500 as *const ()), // sub_141760500
    Replacement::draft(0x141760910, "unknown_141760910", fn_141760910 as *const ()), // sub_141760910
    Replacement::draft(0x141760c50, "unknown_141760c50", fn_141760c50 as *const ()), // sub_141760c50
    Replacement::draft(0x141761020, "unknown_141761020", fn_141761020 as *const ()), // sub_141761020
    Replacement::draft(0x141761100, "unknown_141761100", fn_141761100 as *const ()), // sub_141761100
    Replacement::draft(0x141761380, "unknown_141761380", fn_141761380 as *const ()), // sub_141761380
    Replacement::draft(0x141761810, "unknown_141761810", fn_141761810 as *const ()), // sub_141761810
    Replacement::draft(0x1417618c0, "unknown_1417618c0", fn_1417618c0 as *const ()), // sub_1417618c0
    Replacement::draft(0x141761970, "unknown_141761970", fn_141761970 as *const ()), // sub_141761970
    Replacement::draft(0x141761a20, "unknown_141761a20", fn_141761a20 as *const ()), // sub_141761a20
    Replacement::draft(0x141761b60, "unknown_141761b60", fn_141761b60 as *const ()), // sub_141761b60
    Replacement::draft(0x141761ea0, "unknown_141761ea0", fn_141761ea0 as *const ()), // sub_141761ea0
    Replacement::draft(0x1417623a0, "unknown_1417623a0", tpl_31614::<0xb0, 0x143932190, 0x60, 0x98, 0x80, 0x68, 0x50> as *const ()), // sub_1417623a0
    Replacement::draft(0x141762430, "unknown_141762430", fn_141762430 as *const ()), // sub_141762430
    Replacement::draft(0x141762780, "unknown_141762780", fn_141762780 as *const ()), // sub_141762780
    Replacement::draft(0x141762c30, "unknown_141762c30", fn_141762c30 as *const ()), // sub_141762c30
    Replacement::draft(0x1417631c0, "unknown_1417631c0", fn_1417631c0 as *const ()), // sub_1417631c0
    Replacement::draft(0x1417635f0, "unknown_1417635f0", fn_1417635f0 as *const ()), // sub_1417635f0
    Replacement::draft(0x141763970, "unknown_141763970", fn_141763970 as *const ()), // sub_141763970
    Replacement::draft(0x141763cb0, "unknown_141763cb0", fn_141763cb0 as *const ()), // sub_141763cb0
    Replacement::draft(0x1417641c0, "unknown_1417641c0", fn_1417641c0 as *const ()), // sub_1417641c0
    Replacement::draft(0x141764270, "unknown_141764270", fn_141764270 as *const ()), // sub_141764270
    Replacement::draft(0x141764360, "unknown_141764360", fn_141764360 as *const ()), // sub_141764360
    Replacement::draft(0x141764450, "unknown_141764450", fn_141764450 as *const ()), // sub_141764450
    Replacement::draft(0x141764740, "unknown_141764740", fn_141764740 as *const ()), // sub_141764740
    Replacement::draft(0x1417647a0, "unknown_1417647a0", fn_1417647a0 as *const ()), // sub_1417647a0
    Replacement::draft(0x141764870, "unknown_141764870", fn_141764870 as *const ()), // sub_141764870
    Replacement::draft(0x1417649a0, "unknown_1417649a0", fn_1417649a0 as *const ()), // sub_1417649a0
    Replacement::draft(0x141764bb0, "unknown_141764bb0", fn_141764bb0 as *const ()), // sub_141764bb0
    Replacement::draft(0x141764c60, "unknown_141764c60", tpl_22608::<0x8, 0x141767c20, 0x143932190, 0x60> as *const ()), // sub_141764c60
    Replacement::draft(0x141764ca0, "unknown_141764ca0", tpl_365::<0x141766000> as *const ()), // sub_141764ca0
    Replacement::draft(0x141764cb0, "unknown_141764cb0", fn_141764cb0 as *const ()), // sub_141764cb0
    Replacement::draft(0x141764db0, "unknown_141764db0", fn_141764db0 as *const ()), // sub_141764db0
    Replacement::draft(0x141764de0, "unknown_141764de0", fn_141764de0 as *const ()), // sub_141764de0
    Replacement::draft(0x141764e30, "unknown_141764e30", fn_141764e30 as *const ()), // sub_141764e30
    Replacement::draft(0x141764ed0, "unknown_141764ed0", fn_141764ed0 as *const ()), // sub_141764ed0
    Replacement::draft(0x141765010, "unknown_141765010", fn_141765010 as *const ()), // sub_141765010
    Replacement::draft(0x1417652a0, "unknown_1417652a0", fn_1417652a0 as *const ()), // sub_1417652a0
    Replacement::draft(0x141765530, "unknown_141765530", fn_141765530 as *const ()), // sub_141765530
    Replacement::draft(0x141765910, "unknown_141765910", fn_141765910 as *const ()), // sub_141765910
    Replacement::draft(0x141766000, "unknown_141766000", fn_141766000 as *const ()), // sub_141766000
    Replacement::draft(0x141766090, "unknown_141766090", fn_141766090 as *const ()), // sub_141766090
    Replacement::draft(0x1417660c0, "unknown_1417660c0", fn_1417660c0 as *const ()), // sub_1417660c0
    Replacement::draft(0x141766510, "unknown_141766510", fn_141766510 as *const ()), // sub_141766510
    Replacement::draft(0x141766cd0, "unknown_141766cd0", fn_141766cd0 as *const ()), // sub_141766cd0
    Replacement::draft(0x141767670, "unknown_141767670", fn_141767670 as *const ()), // sub_141767670
    Replacement::draft(0x1417678b0, "unknown_1417678b0", fn_1417678b0 as *const ()), // sub_1417678b0
    Replacement::draft(0x141767c20, "unknown_141767c20", fn_141767c20 as *const ()), // sub_141767c20
    Replacement::draft(0x141767c90, "unknown_141767c90", fn_141767c90 as *const ()), // sub_141767c90
    Replacement::draft(0x141767d70, "unknown_141767d70", fn_141767d70 as *const ()), // sub_141767d70
    Replacement::draft(0x141768120, "unknown_141768120", fn_141768120 as *const ()), // sub_141768120
    Replacement::draft(0x1417684d0, "unknown_1417684d0", fn_1417684d0 as *const ()), // sub_1417684d0
    Replacement::draft(0x141768af0, "unknown_141768af0", fn_141768af0 as *const ()), // sub_141768af0
    Replacement::draft(0x1417690c0, "unknown_1417690c0", fn_1417690c0 as *const ()), // sub_1417690c0
    Replacement::draft(0x1417692b0, "unknown_1417692b0", fn_1417692b0 as *const ()), // sub_1417692b0
    Replacement::draft(0x141769780, "unknown_141769780", fn_141769780 as *const ()), // sub_141769780
    Replacement::draft(0x141769810, "unknown_141769810", fn_141769810 as *const ()), // sub_141769810
    Replacement::draft(0x1417699a0, "unknown_1417699a0", fn_1417699a0 as *const ()), // sub_1417699a0
    Replacement::draft(0x141769bb0, "unknown_141769bb0", fn_141769bb0 as *const ()), // sub_141769bb0
    Replacement::draft(0x141769ed0, "unknown_141769ed0", fn_141769ed0 as *const ()), // sub_141769ed0
    Replacement::draft(0x141769fa0, "unknown_141769fa0", fn_141769fa0 as *const ()), // sub_141769fa0
    Replacement::draft(0x14176a0f0, "unknown_14176a0f0", fn_14176a0f0 as *const ()), // sub_14176a0f0
    Replacement::draft(0x14176a250, "unknown_14176a250", fn_14176a250 as *const ()), // sub_14176a250
    Replacement::draft(0x14176a750, "unknown_14176a750", fn_14176a750 as *const ()), // sub_14176a750
    Replacement::draft(0x14176ac30, "unknown_14176ac30", fn_14176ac30 as *const ()), // sub_14176ac30
    Replacement::draft(0x14176ae10, "unknown_14176ae10", fn_14176ae10 as *const ()), // sub_14176ae10
    Replacement::draft(0x14176ae40, "unknown_14176ae40", fn_14176ae40 as *const ()), // sub_14176ae40
    Replacement::draft(0x14176ae50, "unknown_14176ae50", fn_14176ae50 as *const ()), // sub_14176ae50
    Replacement::draft(0x14176af10, "unknown_14176af10", fn_14176af10 as *const ()), // sub_14176af10
    Replacement::draft(0x14176b0b0, "unknown_14176b0b0", tpl_33077::<0x8, 0x14176b2f0> as *const ()), // sub_14176b0b0
    Replacement::draft(0x14176b0e0, "unknown_14176b0e0", fn_14176b0e0 as *const ()), // sub_14176b0e0
    Replacement::draft(0x14176b250, "unknown_14176b250", fn_14176b250 as *const ()), // sub_14176b250
    Replacement::draft(0x14176b2c0, "unknown_14176b2c0", tpl_33077::<0x8, 0x14176b370> as *const ()), // sub_14176b2c0
    Replacement::draft(0x14176b2f0, "unknown_14176b2f0", fn_14176b2f0 as *const ()), // sub_14176b2f0
    Replacement::draft(0x14176b370, "unknown_14176b370", fn_14176b370 as *const ()), // sub_14176b370
    Replacement::draft(0x14176b3e0, "unknown_14176b3e0", fn_14176b3e0 as *const ()), // sub_14176b3e0
    Replacement::draft(0x14176b490, "unknown_14176b490", fn_14176b490 as *const ()), // sub_14176b490
    Replacement::draft(0x14176b8e0, "unknown_14176b8e0", fn_14176b8e0 as *const ()), // sub_14176b8e0
    Replacement::draft(0x14176bdc0, "unknown_14176bdc0", fn_14176bdc0 as *const ()), // sub_14176bdc0
    Replacement::draft(0x14176c230, "unknown_14176c230", fn_14176c230 as *const ()), // sub_14176c230
    Replacement::draft(0x14176c550, "unknown_14176c550", fn_14176c550 as *const ()), // sub_14176c550
    Replacement::draft(0x14176c6c0, "unknown_14176c6c0", fn_14176c6c0 as *const ()), // sub_14176c6c0
    Replacement::draft(0x14176c8a0, "unknown_14176c8a0", fn_14176c8a0 as *const ()), // sub_14176c8a0
    Replacement::draft(0x14176ca80, "unknown_14176ca80", fn_14176ca80 as *const ()), // sub_14176ca80
    Replacement::draft(0x14176cd40, "unknown_14176cd40", fn_14176cd40 as *const ()), // sub_14176cd40
    Replacement::draft(0x14176ceb0, "unknown_14176ceb0", fn_14176ceb0 as *const ()), // sub_14176ceb0
    Replacement::draft(0x14176d6c0, "unknown_14176d6c0", fn_14176d6c0 as *const ()), // sub_14176d6c0
    Replacement::draft(0x14176d860, "unknown_14176d860", fn_14176d860 as *const ()), // sub_14176d860
    Replacement::draft(0x14176e1d0, "unknown_14176e1d0", fn_14176e1d0 as *const ()), // sub_14176e1d0
    Replacement::draft(0x14176e7c0, "unknown_14176e7c0", fn_14176e7c0 as *const ()), // sub_14176e7c0
    Replacement::draft(0x14176eb50, "unknown_14176eb50", fn_14176eb50 as *const ()), // sub_14176eb50
    Replacement::draft(0x14176f010, "unknown_14176f010", fn_14176f010 as *const ()), // sub_14176f010
    Replacement::draft(0x14176f080, "unknown_14176f080", fn_14176f080 as *const ()), // sub_14176f080
    Replacement::draft(0x14176f660, "unknown_14176f660", fn_14176f660 as *const ()), // sub_14176f660
    Replacement::draft(0x14176fa00, "unknown_14176fa00", fn_14176fa00 as *const ()), // sub_14176fa00
    Replacement::draft(0x14176fa70, "unknown_14176fa70", fn_14176fa70 as *const ()), // sub_14176fa70
    Replacement::draft(0x14176faf0, "unknown_14176faf0", fn_14176faf0 as *const ()), // sub_14176faf0
    Replacement::draft(0x14176ff50, "unknown_14176ff50", tpl_22437::<0x10, 0x8, 0x14174fc50> as *const ()), // sub_14176ff50
    Replacement::draft(0x14176ffd0, "unknown_14176ffd0", fn_14176ffd0 as *const ()), // sub_14176ffd0
    Replacement::draft(0x1417700c0, "unknown_1417700c0", fn_1417700c0 as *const ()), // sub_1417700c0
    Replacement::draft(0x141770b60, "unknown_141770b60", fn_141770b60 as *const ()), // sub_141770b60
    Replacement::draft(0x141770bf0, "unknown_141770bf0", fn_141770bf0 as *const ()), // sub_141770bf0
    Replacement::draft(0x141770d60, "unknown_141770d60", fn_141770d60 as *const ()), // sub_141770d60
    Replacement::draft(0x141770fc0, "unknown_141770fc0", fn_141770fc0 as *const ()), // sub_141770fc0
    Replacement::draft(0x141770fe0, "unknown_141770fe0", fn_141770fe0 as *const ()), // sub_141770fe0
    Replacement::draft(0x141771000, "unknown_141771000", fn_141771000 as *const ()), // sub_141771000
    Replacement::draft(0x141771070, "unknown_141771070", fn_141771070 as *const ()), // sub_141771070
    Replacement::draft(0x141771150, "unknown_141771150", fn_141771150 as *const ()), // sub_141771150
    Replacement::draft(0x1417712f0, "unknown_1417712f0", fn_1417712f0 as *const ()), // sub_1417712f0
    Replacement::draft(0x141771610, "unknown_141771610", fn_141771610 as *const ()), // sub_141771610
    Replacement::draft(0x141771780, "unknown_141771780", fn_141771780 as *const ()), // sub_141771780
    Replacement::draft(0x141771ba0, "unknown_141771ba0", fn_141771ba0 as *const ()), // sub_141771ba0
    Replacement::draft(0x141771d20, "unknown_141771d20", fn_141771d20 as *const ()), // sub_141771d20
    Replacement::draft(0x141771ea0, "unknown_141771ea0", fn_141771ea0 as *const ()), // sub_141771ea0
    Replacement::draft(0x141772230, "unknown_141772230", fn_141772230 as *const ()), // sub_141772230
    Replacement::draft(0x141772570, "unknown_141772570", fn_141772570 as *const ()), // sub_141772570
    Replacement::draft(0x141772750, "unknown_141772750", fn_141772750 as *const ()), // sub_141772750
    Replacement::draft(0x141772bc0, "unknown_141772bc0", fn_141772bc0 as *const ()), // sub_141772bc0
    Replacement::draft(0x141772ed0, "unknown_141772ed0", fn_141772ed0 as *const ()), // sub_141772ed0
    Replacement::draft(0x1417730f0, "unknown_1417730f0", fn_1417730f0 as *const ()), // sub_1417730f0
    Replacement::draft(0x1417734a0, "unknown_1417734a0", fn_1417734a0 as *const ()), // sub_1417734a0
    Replacement::draft(0x1417734f0, "unknown_1417734f0", fn_1417734f0 as *const ()), // sub_1417734f0
    Replacement::draft(0x1417735e0, "unknown_1417735e0", fn_1417735e0 as *const ()), // sub_1417735e0
    Replacement::draft(0x141773690, "unknown_141773690", fn_141773690 as *const ()), // sub_141773690
    Replacement::draft(0x141773c40, "unknown_141773c40", fn_141773c40 as *const ()), // sub_141773c40
    Replacement::draft(0x141773da0, "unknown_141773da0", fn_141773da0 as *const ()), // sub_141773da0
    Replacement::draft(0x141773f10, "unknown_141773f10", fn_141773f10 as *const ()), // sub_141773f10
    Replacement::draft(0x1417740d0, "unknown_1417740d0", fn_1417740d0 as *const ()), // sub_1417740d0
    Replacement::draft(0x141774690, "unknown_141774690", fn_141774690 as *const ()), // sub_141774690
    Replacement::draft(0x1417748f0, "unknown_1417748f0", fn_1417748f0 as *const ()), // sub_1417748f0
    Replacement::draft(0x141774a10, "unknown_141774a10", fn_141774a10 as *const ()), // sub_141774a10
    Replacement::draft(0x141774ac0, "unknown_141774ac0", fn_141774ac0 as *const ()), // sub_141774ac0
    Replacement::draft(0x141774b60, "unknown_141774b60", fn_141774b60 as *const ()), // sub_141774b60
    Replacement::draft(0x141774c30, "unknown_141774c30", fn_141774c30 as *const ()), // sub_141774c30
    Replacement::draft(0x141774d80, "unknown_141774d80", fn_141774d80 as *const ()), // sub_141774d80
    Replacement::draft(0x141774d90, "unknown_141774d90", fn_141774d90 as *const ()), // sub_141774d90
    Replacement::draft(0x141774e30, "unknown_141774e30", fn_141774e30 as *const ()), // sub_141774e30
    Replacement::draft(0x141774e70, "ak_141774e70", fn_141774e70 as *const ()), // sub_141774e70
    Replacement::draft(0x141774ed0, "ak_141774ed0", fn_141774ed0 as *const ()), // sub_141774ed0
    Replacement::draft(0x1417752f0, "ak_1417752f0", fn_1417752f0 as *const ()), // sub_1417752f0
    Replacement::draft(0x1417755c0, "ak_1417755c0", tpl_30911::<0x28, 0x1417795f0> as *const ()), // sub_1417755c0
    Replacement::draft(0x1417755e0, "ak_1417755e0", tpl_33124::<0x28, 0x141778b00, 0x10> as *const ()), // sub_1417755e0
    Replacement::draft(0x141775640, "ak_141775640", fn_141775640 as *const ()), // sub_141775640
    Replacement::draft(0x1417756c0, "ak_1417756c0", fn_1417756c0 as *const ()), // sub_1417756c0
    Replacement::draft(0x141775700, "ak_141775700", fn_141775700 as *const ()), // sub_141775700
    Replacement::draft(0x141775780, "ak_141775780", fn_141775780 as *const ()), // sub_141775780
    Replacement::draft(0x1417758c0, "ak_1417758c0", fn_1417758c0 as *const ()), // sub_1417758c0
    Replacement::draft(0x141775960, "ak_141775960", fn_141775960 as *const ()), // sub_141775960
    Replacement::draft(0x141775a00, "ak_141775a00", tpl_22428::<0x143932cf8, 0x10> as *const ()), // sub_141775a00
    Replacement::draft(0x141775a20, "ak_141775a20", tpl_22428::<0x143932cf8, 0x10> as *const ()), // sub_141775a20
    Replacement::draft(0x141775a40, "ak_141775a40", fn_141775a40 as *const ()), // sub_141775a40
    Replacement::draft(0x141775ab0, "ak_141775ab0", fn_141775ab0 as *const ()), // sub_141775ab0
    Replacement::draft(0x141775c30, "ak_141775c30", fn_141775c30 as *const ()), // sub_141775c30
    Replacement::draft(0x141775da0, "ak_141775da0", fn_141775da0 as *const ()), // sub_141775da0
    Replacement::draft(0x141775e10, "ak_141775e10", fn_141775e10 as *const ()), // sub_141775e10
    Replacement::draft(0x141775e80, "ak_141775e80", fn_141775e80 as *const ()), // sub_141775e80
    Replacement::draft(0x141776050, "ak_141776050", fn_141776050 as *const ()), // sub_141776050
    Replacement::draft(0x141776060, "ak_141776060", tpl_1::<0x143932cf8> as *const ()), // sub_141776060
    Replacement::draft(0x141776070, "ak_141776070", tpl_365::<0x1417768e0> as *const ()), // sub_141776070
    Replacement::draft(0x141776080, "ak_141776080", fn_141776080 as *const ()), // sub_141776080
    Replacement::draft(0x1417760e0, "ak_1417760e0", fn_1417760e0 as *const ()), // sub_1417760e0
    Replacement::draft(0x141776140, "ak_141776140", fn_141776140 as *const ()), // sub_141776140
    Replacement::draft(0x1417761a0, "ak_1417761a0", fn_1417761a0 as *const ()), // sub_1417761a0
    Replacement::draft(0x1417763e0, "ak_1417763e0", fn_1417763e0 as *const ()), // sub_1417763e0
    Replacement::draft(0x1417766f0, "ak_1417766f0", fn_1417766f0 as *const ()), // sub_1417766f0
    Replacement::draft(0x141776840, "ak_141776840", fn_141776840 as *const ()), // sub_141776840
    Replacement::draft(0x141776880, "ak_141776880", fn_141776880 as *const ()), // sub_141776880
    Replacement::draft(0x1417768b0, "ak_1417768b0", fn_1417768b0 as *const ()), // sub_1417768b0
    Replacement::draft(0x1417768e0, "ak_1417768e0", fn_1417768e0 as *const ()), // sub_1417768e0
    Replacement::draft(0x141776e00, "ak_141776e00", fn_141776e00 as *const ()), // sub_141776e00
    Replacement::draft(0x141777300, "ak_141777300", fn_141777300 as *const ()), // sub_141777300
    Replacement::draft(0x141777330, "ak_141777330", fn_141777330 as *const ()), // sub_141777330
    Replacement::draft(0x141777390, "ak_141777390", fn_141777390 as *const ()), // sub_141777390
    Replacement::draft(0x141777480, "ak_141777480", fn_141777480 as *const ()), // sub_141777480
    Replacement::draft(0x141777590, "ak_141777590", fn_141777590 as *const ()), // sub_141777590
    Replacement::draft(0x1417775f0, "ak_1417775f0", fn_1417775f0 as *const ()), // sub_1417775f0
    Replacement::draft(0x141777620, "unknown_141777620", tpl_33145::<0x8, 0x10, 0x1417776b0> as *const ()), // sub_141777620
    Replacement::draft(0x1417776b0, "unknown_1417776b0", fn_1417776b0 as *const ()), // sub_1417776b0
    Replacement::draft(0x141777860, "unknown_141777860", fn_141777860 as *const ()), // sub_141777860
    Replacement::draft(0x141777a20, "unknown_141777a20", fn_141777a20 as *const ()), // sub_141777a20
    Replacement::draft(0x141777c60, "unknown_141777c60", fn_141777c60 as *const ()), // sub_141777c60
    Replacement::draft(0x141777cd0, "ak_141777cd0", fn_141777cd0 as *const ()), // sub_141777cd0
    Replacement::draft(0x141778860, "unknown_141778860", tpl_33147::<0x141776060, 0x141776050> as *const ()), // sub_141778860
    Replacement::draft(0x141778880, "unknown_141778880", fn_141778880 as *const ()), // sub_141778880
    Replacement::draft(0x1417788f0, "unknown_1417788f0", tpl_33149::<0x8, 0x14177d5e0, 0x141776060, 0x141776050> as *const ()), // sub_1417788f0
    Replacement::draft(0x141778950, "unknown_141778950", tpl_33149::<0x8, 0x14177b5d0, 0x141776060, 0x141776050> as *const ()), // sub_141778950
    Replacement::draft(0x1417789b0, "unknown_1417789b0", tpl_33149::<0x8, 0x14177dc20, 0x141776060, 0x141776050> as *const ()), // sub_1417789b0
    Replacement::draft(0x141778a10, "unknown_141778a10", tpl_33147::<0x141776060, 0x141776050> as *const ()), // sub_141778a10
    Replacement::draft(0x141778a30, "unknown_141778a30", fn_141778a30 as *const ()), // sub_141778a30
    Replacement::draft(0x141778aa0, "unknown_141778aa0", tpl_33151::<0x8, 0x141778880, 0x141776060, 0x141776050> as *const ()), // sub_141778aa0
    Replacement::draft(0x141778b00, "unknown_141778b00", fn_141778b00 as *const ()), // sub_141778b00
    Replacement::draft(0x141778d10, "ak_141778d10", tpl_27317::<0xd0, 0x14177b840> as *const ()), // sub_141778d10
    Replacement::draft(0x141778d20, "ak_141778d20", tpl_27317::<0xd0, 0x14177b850> as *const ()), // sub_141778d20
    Replacement::draft(0x141778d30, "ak_141778d30", tpl_27317::<0xd0, 0x14177b860> as *const ()), // sub_141778d30
    Replacement::draft(0x141778d40, "ak_141778d40", tpl_27317::<0xd0, 0x14177b870> as *const ()), // sub_141778d40
    Replacement::draft(0x141778d50, "ak_141778d50", tpl_27317::<0xd0, 0x14177b880> as *const ()), // sub_141778d50
    Replacement::draft(0x141778d60, "ak_141778d60", tpl_27317::<0xd0, 0x14177b890> as *const ()), // sub_141778d60
    Replacement::draft(0x141778d70, "ak_141778d70", fn_141778d70 as *const ()), // sub_141778d70
    Replacement::draft(0x141778f70, "unknown_141778f70", fn_141778f70 as *const ()), // sub_141778f70
    Replacement::draft(0x141779400, "unknown_141779400", tpl_33154::<0x130, 0xd0, 0x14177bfa0> as *const ()), // sub_141779400
    Replacement::draft(0x141779430, "unknown_141779430", tpl_33154::<0x130, 0xd0, 0x14177c3d0> as *const ()), // sub_141779430
    Replacement::draft(0x141779460, "unknown_141779460", tpl_33154::<0x130, 0xd0, 0x14177c430> as *const ()), // sub_141779460
    Replacement::draft(0x141779490, "unknown_141779490", tpl_33154::<0x130, 0xd0, 0x14177c450> as *const ()), // sub_141779490
    Replacement::draft(0x1417794c0, "unknown_1417794c0", tpl_33154::<0x130, 0xd0, 0x14177c4b0> as *const ()), // sub_1417794c0
    Replacement::draft(0x1417794f0, "unknown_1417794f0", tpl_33154::<0x130, 0xd0, 0x14177c4d0> as *const ()), // sub_1417794f0
    Replacement::draft(0x141779520, "unknown_141779520", fn_141779520 as *const ()), // sub_141779520
    Replacement::draft(0x1417795f0, "unknown_1417795f0", fn_1417795f0 as *const ()), // sub_1417795f0
    Replacement::draft(0x141779770, "unknown_141779770", tpl_33145::<0x8, 0x10, 0x141779800> as *const ()), // sub_141779770
    Replacement::draft(0x141779800, "unknown_141779800", fn_141779800 as *const ()), // sub_141779800
    Replacement::draft(0x141779990, "unknown_141779990", fn_141779990 as *const ()), // sub_141779990
    Replacement::draft(0x141779ad0, "unknown_141779ad0", fn_141779ad0 as *const ()), // sub_141779ad0
    Replacement::draft(0x141779d00, "unknown_141779d00", tpl_92687::<0x141776060, 0x141775da0, 0x141778880, 0x141776050> as *const ()), // sub_141779d00
    Replacement::draft(0x141779ed0, "unknown_141779ed0", fn_141779ed0 as *const ()), // sub_141779ed0
    Replacement::draft(0x141779f30, "unknown_141779f30", fn_141779f30 as *const ()), // sub_141779f30
    Replacement::draft(0x141779fe0, "unknown_141779fe0", fn_141779fe0 as *const ()), // sub_141779fe0
    Replacement::draft(0x14177b3f0, "unknown_14177b3f0", tpl_170::<0x14177c7d0> as *const ()), // sub_14177b3f0
    Replacement::draft(0x14177b400, "unknown_14177b400", tpl_33149::<0x8, 0x14177e150, 0x141776060, 0x141776050> as *const ()), // sub_14177b400
    Replacement::draft(0x14177b460, "unknown_14177b460", tpl_33149::<0x8, 0x14177e980, 0x141776060, 0x141776050> as *const ()), // sub_14177b460
    Replacement::draft(0x14177b4c0, "unknown_14177b4c0", tpl_33149::<0x8, 0x1401de410, 0x141776060, 0x141776050> as *const ()), // sub_14177b4c0
    Replacement::draft(0x14177b520, "unknown_14177b520", fn_14177b520 as *const ()), // sub_14177b520
    Replacement::draft(0x14177b580, "unknown_14177b580", tpl_33160::<0x8, 0x14177c750, 0x141776060, 0x141776050> as *const ()), // sub_14177b580
    Replacement::draft(0x14177b5b0, "unknown_14177b5b0", tpl_6558::<0x14177c7d0> as *const ()), // sub_14177b5b0
    Replacement::draft(0x14177b5d0, "unknown_14177b5d0", fn_14177b5d0 as *const ()), // sub_14177b5d0
    Replacement::draft(0x14177b7e0, "unknown_14177b7e0", tpl_33151::<0x8, 0x14177c7d0, 0x141776060, 0x141776050> as *const ()), // sub_14177b7e0
    Replacement::draft(0x14177b840, "unknown_14177b840", tpl_586::<0x54> as *const ()), // sub_14177b840
    Replacement::draft(0x14177b850, "unknown_14177b850", tpl_586::<0x68> as *const ()), // sub_14177b850
    Replacement::draft(0x14177b860, "unknown_14177b860", tpl_586::<0x5c> as *const ()), // sub_14177b860
    Replacement::draft(0x14177b870, "unknown_14177b870", tpl_586::<0x60> as *const ()), // sub_14177b870
    Replacement::draft(0x14177b880, "unknown_14177b880", tpl_586::<0x58> as *const ()), // sub_14177b880
    Replacement::draft(0x14177b890, "unknown_14177b890", tpl_586::<0x64> as *const ()), // sub_14177b890
    Replacement::draft(0x14177b8a0, "unknown_14177b8a0", fn_14177b8a0 as *const ()), // sub_14177b8a0
    Replacement::draft(0x14177bfa0, "unknown_14177bfa0", fn_14177bfa0 as *const ()), // sub_14177bfa0
    Replacement::draft(0x14177c090, "unknown_14177c090", fn_14177c090 as *const ()), // sub_14177c090
    Replacement::draft(0x14177c3d0, "unknown_14177c3d0", tpl_33165::<0x54, 0x143426d48, 0x68, 0x1434268d0, 0x1438215d0, 0x14183ad46, 0xd8, 0x14177e320> as *const ()), // sub_14177c3d0
    Replacement::draft(0x14177c430, "unknown_14177c430", tpl_33166::<0x5c, 0xd8, 0x14177e310> as *const ()), // sub_14177c430
    Replacement::draft(0x14177c450, "unknown_14177c450", tpl_33165::<0x54, 0x143426d48, 0x60, 0x1434268d0, 0x1438215d0, 0x14183ad46, 0xd8, 0x14177e340> as *const ()), // sub_14177c450
    Replacement::draft(0x14177c4b0, "unknown_14177c4b0", tpl_33166::<0x58, 0xd8, 0x14177e330> as *const ()), // sub_14177c4b0
    Replacement::draft(0x14177c4d0, "unknown_14177c4d0", tpl_33165::<0x54, 0x143426d48, 0x64, 0x1434268d0, 0x1438215d0, 0x14183ad46, 0xd8, 0x14177e350> as *const ()), // sub_14177c4d0
    Replacement::draft(0x14177c530, "unknown_14177c530", fn_14177c530 as *const ()), // sub_14177c530
    Replacement::draft(0x14177c650, "unknown_14177c650", fn_14177c650 as *const ()), // sub_14177c650
    Replacement::draft(0x14177c750, "unknown_14177c750", fn_14177c750 as *const ()), // sub_14177c750
    Replacement::draft(0x14177c7d0, "unknown_14177c7d0", fn_14177c7d0 as *const ()), // sub_14177c7d0
    Replacement::draft(0x14177c860, "unknown_14177c860", fn_14177c860 as *const ()), // sub_14177c860
    Replacement::draft(0x14177c940, "unknown_14177c940", fn_14177c940 as *const ()), // sub_14177c940
    Replacement::draft(0x14177ca20, "unknown_14177ca20", fn_14177ca20 as *const ()), // sub_14177ca20
    Replacement::draft(0x14177cbc0, "unknown_14177cbc0", fn_14177cbc0 as *const ()), // sub_14177cbc0
    Replacement::draft(0x14177ce30, "unknown_14177ce30", fn_14177ce30 as *const ()), // sub_14177ce30
    Replacement::draft(0x14177d030, "unknown_14177d030", fn_14177d030 as *const ()), // sub_14177d030
    Replacement::draft(0x14177d290, "unknown_14177d290", fn_14177d290 as *const ()), // sub_14177d290
    Replacement::draft(0x14177d460, "unknown_14177d460", fn_14177d460 as *const ()), // sub_14177d460
    Replacement::draft(0x14177d4a0, "unknown_14177d4a0", fn_14177d4a0 as *const ()), // sub_14177d4a0
    Replacement::draft(0x14177d5d0, "unknown_14177d5d0", tpl_365::<0x141778a30> as *const ()), // sub_14177d5d0
    Replacement::draft(0x14177d5e0, "unknown_14177d5e0", fn_14177d5e0 as *const ()), // sub_14177d5e0
    Replacement::draft(0x14177d810, "unknown_14177d810", fn_14177d810 as *const ()), // sub_14177d810
    Replacement::draft(0x14177d9c0, "unknown_14177d9c0", fn_14177d9c0 as *const ()), // sub_14177d9c0
    Replacement::draft(0x14177db90, "unknown_14177db90", tpl_33149::<0x8, 0x141782470, 0x141776060, 0x141776050> as *const ()), // sub_14177db90
    Replacement::draft(0x14177dbf0, "unknown_14177dbf0", tpl_33160::<0x8, 0x14177dde0, 0x141776060, 0x141776050> as *const ()), // sub_14177dbf0
    Replacement::draft(0x14177dc20, "unknown_14177dc20", fn_14177dc20 as *const ()), // sub_14177dc20
    Replacement::draft(0x14177dc70, "unknown_14177dc70", fn_14177dc70 as *const ()), // sub_14177dc70
    Replacement::draft(0x14177dce0, "unknown_14177dce0", fn_14177dce0 as *const ()), // sub_14177dce0
    Replacement::draft(0x14177dda0, "unknown_14177dda0", tpl_33180::<0x40, 0x141784b80> as *const ()), // sub_14177dda0
    Replacement::draft(0x14177dde0, "unknown_14177dde0", tpl_33181::<0x8, 0x141782470, 0x141776060, 0x141776050> as *const ()), // sub_14177dde0
    Replacement::draft(0x14177de60, "unknown_14177de60", tpl_92714::<0x141776060, 0x141775da0, 0x14177dde0, 0x141776050> as *const ()), // sub_14177de60
    Replacement::draft(0x14177e010, "unknown_14177e010", fn_14177e010 as *const ()), // sub_14177e010
    Replacement::draft(0x14177e150, "unknown_14177e150", tpl_33183::<0x58, 0x141776060, 0x141776050, 0x38> as *const ()), // sub_14177e150
    Replacement::draft(0x14177e1a0, "unknown_14177e1a0", fn_14177e1a0 as *const ()), // sub_14177e1a0
    Replacement::draft(0x14177e300, "unknown_14177e300", fn_14177e300 as *const ()), // sub_14177e300
    Replacement::draft(0x14177e310, "unknown_14177e310", tpl_33185::<0x18, 0x14177e550> as *const ()), // sub_14177e310
    Replacement::draft(0x14177e320, "unknown_14177e320", tpl_33185::<0x30, 0x14177e550> as *const ()), // sub_14177e320
    Replacement::draft(0x14177e330, "unknown_14177e330", tpl_33185::<0x10, 0x14177e550> as *const ()), // sub_14177e330
    Replacement::draft(0x14177e340, "unknown_14177e340", tpl_33185::<0x20, 0x14177e550> as *const ()), // sub_14177e340
    Replacement::draft(0x14177e350, "unknown_14177e350", tpl_33185::<0x28, 0x14177e550> as *const ()), // sub_14177e350
    Replacement::draft(0x14177e360, "unknown_14177e360", fn_14177e360 as *const ()), // sub_14177e360
    Replacement::draft(0x14177e3b0, "unknown_14177e3b0", fn_14177e3b0 as *const ()), // sub_14177e3b0
    Replacement::draft(0x14177e550, "unknown_14177e550", fn_14177e550 as *const ()), // sub_14177e550
    Replacement::draft(0x14177e7e0, "unknown_14177e7e0", fn_14177e7e0 as *const ()), // sub_14177e7e0
    Replacement::draft(0x14177e920, "unknown_14177e920", tpl_33149::<0x8, 0x141782210, 0x141776060, 0x141776050> as *const ()), // sub_14177e920
    Replacement::draft(0x14177e980, "unknown_14177e980", fn_14177e980 as *const ()), // sub_14177e980
    Replacement::draft(0x14177ea20, "unknown_14177ea20", fn_14177ea20 as *const ()), // sub_14177ea20
    Replacement::draft(0x14177ea60, "unknown_14177ea60", fn_14177ea60 as *const ()), // sub_14177ea60
    Replacement::draft(0x14177eab0, "unknown_14177eab0", fn_14177eab0 as *const ()), // sub_14177eab0
    Replacement::draft(0x14177eee0, "unknown_14177eee0", fn_14177eee0 as *const ()), // sub_14177eee0
    Replacement::draft(0x14177ef80, "unknown_14177ef80", tpl_33183::<0x30, 0x141776060, 0x141776050, 0x10> as *const ()), // sub_14177ef80
    Replacement::draft(0x14177efd0, "unknown_14177efd0", fn_14177efd0 as *const ()), // sub_14177efd0
    Replacement::draft(0x14177f0e0, "unknown_14177f0e0", fn_14177f0e0 as *const ()), // sub_14177f0e0
    Replacement::draft(0x14177f200, "unknown_14177f200", fn_14177f200 as *const ()), // sub_14177f200
    Replacement::draft(0x14177f3c0, "unknown_14177f3c0", fn_14177f3c0 as *const ()), // sub_14177f3c0
    Replacement::draft(0x14177ff40, "unknown_14177ff40", fn_14177ff40 as *const ()), // sub_14177ff40
    Replacement::draft(0x14177ffa0, "unknown_14177ffa0", fn_14177ffa0 as *const ()), // sub_14177ffa0
    Replacement::draft(0x141781130, "unknown_141781130", fn_141781130 as *const ()), // sub_141781130
    Replacement::draft(0x1417812b0, "luau_1417812b0", fn_1417812b0 as *const ()), // sub_1417812b0
    Replacement::draft(0x141781f20, "unknown_141781f20", fn_141781f20 as *const ()), // sub_141781f20
    Replacement::draft(0x141781fd0, "unknown_141781fd0", tpl_33149::<0x8, 0x141782220, 0x141776060, 0x141776050> as *const ()), // sub_141781fd0
    Replacement::draft(0x141782030, "unknown_141782030", tpl_33160::<0x8, 0x141785380, 0x141776060, 0x141776050> as *const ()), // sub_141782030
    Replacement::draft(0x141782060, "unknown_141782060", fn_141782060 as *const ()), // sub_141782060
    Replacement::draft(0x1417820c0, "unknown_1417820c0", fn_1417820c0 as *const ()), // sub_1417820c0
    Replacement::draft(0x141782210, "unknown_141782210", tpl_1742::<0x141781f20> as *const ()), // sub_141782210
    Replacement::draft(0x141782220, "unknown_141782220", fn_141782220 as *const ()), // sub_141782220
    Replacement::draft(0x141782400, "unknown_141782400", fn_141782400 as *const ()), // sub_141782400
    Replacement::draft(0x141782470, "unknown_141782470", fn_141782470 as *const ()), // sub_141782470
    Replacement::draft(0x141782550, "unknown_141782550", fn_141782550 as *const ()), // sub_141782550
    Replacement::draft(0x141782590, "unknown_141782590", fn_141782590 as *const ()), // sub_141782590
    Replacement::draft(0x141782670, "luau_141782670", fn_141782670 as *const ()), // sub_141782670
    Replacement::draft(0x141782960, "luau_141782960", fn_141782960 as *const ()), // sub_141782960
    Replacement::draft(0x141782d20, "unknown_141782d20", fn_141782d20 as *const ()), // sub_141782d20
    Replacement::draft(0x141783020, "unknown_141783020", fn_141783020 as *const ()), // sub_141783020
    Replacement::draft(0x1417836f0, "unknown_1417836f0", tpl_1742::<0x141783020> as *const ()), // sub_1417836f0
    Replacement::draft(0x141783700, "unknown_141783700", tpl_913::<0x190> as *const ()), // sub_141783700
    Replacement::draft(0x141783710, "unknown_141783710", fn_141783710 as *const ()), // sub_141783710
    Replacement::draft(0x1417838c0, "unknown_1417838c0", fn_1417838c0 as *const ()), // sub_1417838c0
    Replacement::draft(0x1417839e0, "luau_1417839e0", fn_1417839e0 as *const ()), // sub_1417839e0
    Replacement::draft(0x141783af0, "unknown_141783af0", fn_141783af0 as *const ()), // sub_141783af0
    Replacement::draft(0x1417840d0, "unknown_1417840d0", fn_1417840d0 as *const ()), // sub_1417840d0
    Replacement::draft(0x141784290, "unknown_141784290", fn_141784290 as *const ()), // sub_141784290
    Replacement::draft(0x1417843a0, "luau_1417843a0", fn_1417843a0 as *const ()), // sub_1417843a0
    Replacement::draft(0x1417844b0, "unknown_1417844b0", fn_1417844b0 as *const ()), // sub_1417844b0
    Replacement::draft(0x1417845f0, "unknown_1417845f0", fn_1417845f0 as *const ()), // sub_1417845f0
    Replacement::draft(0x141784670, "unknown_141784670", fn_141784670 as *const ()), // sub_141784670
    Replacement::draft(0x141784740, "unknown_141784740", fn_141784740 as *const ()), // sub_141784740
    Replacement::draft(0x141784760, "unknown_141784760", fn_141784760 as *const ()), // sub_141784760
    Replacement::draft(0x141784790, "unknown_141784790", fn_141784790 as *const ()), // sub_141784790
    Replacement::draft(0x141784890, "unknown_141784890", fn_141784890 as *const ()), // sub_141784890
    Replacement::draft(0x1417848b0, "unknown_1417848b0", fn_1417848b0 as *const ()), // sub_1417848b0
    Replacement::draft(0x1417849f0, "unknown_1417849f0", fn_1417849f0 as *const ()), // sub_1417849f0
    Replacement::draft(0x141784a50, "unknown_141784a50", fn_141784a50 as *const ()), // sub_141784a50
    Replacement::draft(0x141784b70, "unknown_141784b70", fn_141784b70 as *const ()), // sub_141784b70
    Replacement::draft(0x141784b80, "unknown_141784b80", fn_141784b80 as *const ()), // sub_141784b80
    Replacement::draft(0x141784cf0, "unknown_141784cf0", fn_141784cf0 as *const ()), // sub_141784cf0
    Replacement::draft(0x141785260, "unknown_141785260", tpl_33217::<0x8, 0x10, 0x141785e20> as *const ()), // sub_141785260
    Replacement::draft(0x1417852f0, "unknown_1417852f0", tpl_33217::<0x8, 0x10, 0x141779990> as *const ()), // sub_1417852f0
    Replacement::draft(0x141785380, "unknown_141785380", tpl_33181::<0x8, 0x141782220, 0x141776060, 0x141776050> as *const ()), // sub_141785380
    Replacement::draft(0x141785400, "unknown_141785400", fn_141785400 as *const ()), // sub_141785400
    Replacement::draft(0x1417855d0, "unknown_1417855d0", fn_1417855d0 as *const ()), // sub_1417855d0
    Replacement::draft(0x141785bf0, "unknown_141785bf0", fn_141785bf0 as *const ()), // sub_141785bf0
    Replacement::draft(0x141785ce0, "unknown_141785ce0", fn_141785ce0 as *const ()), // sub_141785ce0
    Replacement::draft(0x141785e20, "unknown_141785e20", fn_141785e20 as *const ()), // sub_141785e20
    Replacement::draft(0x141785f60, "unknown_141785f60", fn_141785f60 as *const ()), // sub_141785f60
    Replacement::draft(0x141786180, "unknown_141786180", tpl_92714::<0x141776060, 0x141775da0, 0x141785380, 0x141776050> as *const ()), // sub_141786180
    Replacement::draft(0x141786330, "unknown_141786330", fn_141786330 as *const ()), // sub_141786330
    Replacement::draft(0x141786550, "unknown_141786550", fn_141786550 as *const ()), // sub_141786550
    Replacement::draft(0x1417866f0, "unknown_1417866f0", fn_1417866f0 as *const ()), // sub_1417866f0
    Replacement::draft(0x141786790, "unknown_141786790", fn_141786790 as *const ()), // sub_141786790
    Replacement::draft(0x141786810, "unknown_141786810", fn_141786810 as *const ()), // sub_141786810
    Replacement::draft(0x141786eb0, "unknown_141786eb0", tpl_33160::<0x8, 0x141786f90, 0x141776060, 0x141776050> as *const ()), // sub_141786eb0
    Replacement::draft(0x141786ee0, "unknown_141786ee0", fn_141786ee0 as *const ()), // sub_141786ee0
    Replacement::draft(0x141786f90, "unknown_141786f90", tpl_33181::<0x8, 0x141782400, 0x141776060, 0x141776050> as *const ()), // sub_141786f90
    Replacement::draft(0x141787010, "unknown_141787010", tpl_22914::<0x1426711d8, 0x141834d30> as *const ()), // sub_141787010
    Replacement::draft(0x141787040, "ak_141787040", fn_141787040 as *const ()), // sub_141787040
    Replacement::draft(0x1417870c0, "ak_1417870c0", fn_1417870c0 as *const ()), // sub_1417870c0
    Replacement::draft(0x141787690, "ak_141787690", fn_141787690 as *const ()), // sub_141787690
    Replacement::draft(0x1417878b0, "ak_1417878b0", fn_1417878b0 as *const ()), // sub_1417878b0
    Replacement::draft(0x141787b60, "ak_141787b60", fn_141787b60 as *const ()), // sub_141787b60
    Replacement::draft(0x141787b90, "ak_141787b90", fn_141787b90 as *const ()), // sub_141787b90
    Replacement::draft(0x141787c10, "ak_141787c10", fn_141787c10 as *const ()), // sub_141787c10
    Replacement::draft(0x141787c80, "ak_141787c80", tpl_33227::<0x1426af9c0, 0x1b, 0x8, 0x1426af990> as *const ()), // sub_141787c80
    Replacement::draft(0x141787cc0, "ak_141787cc0", fn_141787cc0 as *const ()), // sub_141787cc0
    Replacement::draft(0x141787d10, "ak_141787d10", fn_141787d10 as *const ()), // sub_141787d10
    Replacement::draft(0x141787dd0, "ak_141787dd0", fn_141787dd0 as *const ()), // sub_141787dd0
    Replacement::draft(0x141787e20, "ak_141787e20", fn_141787e20 as *const ()), // sub_141787e20
    Replacement::draft(0x141788170, "ak_141788170", tpl_33149::<0x8, 0x141788930, 0x141776060, 0x141776050> as *const ()), // sub_141788170
    Replacement::draft(0x1417881d0, "ak_1417881d0", fn_1417881d0 as *const ()), // sub_1417881d0
    Replacement::draft(0x141788250, "ak_141788250", tpl_33232::<0x8c> as *const ()), // sub_141788250
    Replacement::draft(0x141788260, "ak_141788260", tpl_6190::<0x141788a60> as *const ()), // sub_141788260
    Replacement::draft(0x141788270, "ak_141788270", fn_141788270 as *const ()), // sub_141788270
    Replacement::draft(0x141788280, "ak_141788280", fn_141788280 as *const ()), // sub_141788280
    Replacement::draft(0x1417882d0, "ak_1417882d0", fn_1417882d0 as *const ()), // sub_1417882d0
    Replacement::draft(0x141788350, "ak_141788350", fn_141788350 as *const ()), // sub_141788350
    Replacement::draft(0x141788360, "ak_141788360", fn_141788360 as *const ()), // sub_141788360
    Replacement::draft(0x141788930, "ak_141788930", fn_141788930 as *const ()), // sub_141788930
    Replacement::draft(0x1417889b0, "ak_1417889b0", fn_1417889b0 as *const ()), // sub_1417889b0
    Replacement::draft(0x141788a60, "ak_141788a60", fn_141788a60 as *const ()), // sub_141788a60
    Replacement::draft(0x141788bf0, "ak_141788bf0", fn_141788bf0 as *const ()), // sub_141788bf0
    Replacement::draft(0x141788ee0, "ak_141788ee0", fn_141788ee0 as *const ()), // sub_141788ee0
    Replacement::draft(0x1417890f0, "ak_1417890f0", fn_1417890f0 as *const ()), // sub_1417890f0
    Replacement::draft(0x1417891f0, "ak_1417891f0", fn_1417891f0 as *const ()), // sub_1417891f0
    Replacement::draft(0x141789210, "ak_141789210", fn_141789210 as *const ()), // sub_141789210
    Replacement::draft(0x141789230, "ak_141789230", fn_141789230 as *const ()), // sub_141789230
    Replacement::draft(0x141789780, "ak_141789780", fn_141789780 as *const ()), // sub_141789780
    Replacement::draft(0x1417897b0, "ak_1417897b0", tpl_28389::<0x141789d10> as *const ()), // sub_1417897b0
    Replacement::draft(0x1417897c0, "ak_1417897c0", fn_1417897c0 as *const ()), // sub_1417897c0
    Replacement::draft(0x141789b50, "ak_141789b50", fn_141789b50 as *const ()), // sub_141789b50
    Replacement::draft(0x141789d10, "ak_141789d10", fn_141789d10 as *const ()), // sub_141789d10
    Replacement::draft(0x141789e40, "ak_141789e40", fn_141789e40 as *const ()), // sub_141789e40
    Replacement::draft(0x141789f40, "ak_141789f40", tpl_23357::<0x1426711d8, 0x141834d30> as *const ()), // sub_141789f40
    Replacement::draft(0x141789f70, "ak_141789f70", fn_141789f70 as *const ()), // sub_141789f70
    Replacement::draft(0x141789fd0, "ak_141789fd0", fn_141789fd0 as *const ()), // sub_141789fd0
    Replacement::draft(0x14178a280, "ak_14178a280", fn_14178a280 as *const ()), // sub_14178a280
    Replacement::draft(0x14178a360, "ak_14178a360", fn_14178a360 as *const ()), // sub_14178a360
    Replacement::draft(0x14178a380, "ak_14178a380", tpl_33124::<0x8, 0x14178aaf0, 0x10> as *const ()), // sub_14178a380
    Replacement::draft(0x14178a3e0, "ak_14178a3e0", fn_14178a3e0 as *const ()), // sub_14178a3e0
    Replacement::draft(0x14178a450, "ak_14178a450", tpl_33227::<0x1426afb40, 0x23, 0x8, 0x1426afb10> as *const ()), // sub_14178a450
    Replacement::draft(0x14178a490, "ak_14178a490", fn_14178a490 as *const ()), // sub_14178a490
    Replacement::draft(0x14178a4f0, "ak_14178a4f0", fn_14178a4f0 as *const ()), // sub_14178a4f0
    Replacement::draft(0x14178a5e0, "ak_14178a5e0", fn_14178a5e0 as *const ()), // sub_14178a5e0
    Replacement::draft(0x14178a650, "ak_14178a650", fn_14178a650 as *const ()), // sub_14178a650
    Replacement::draft(0x14178aa60, "ak_14178aa60", fn_14178aa60 as *const ()), // sub_14178aa60
    Replacement::draft(0x14178aac0, "ak_14178aac0", tpl_33160::<0x8, 0x14178b000, 0x141776060, 0x141776050> as *const ()), // sub_14178aac0
    Replacement::draft(0x14178aaf0, "ak_14178aaf0", fn_14178aaf0 as *const ()), // sub_14178aaf0
    Replacement::draft(0x14178ab60, "ak_14178ab60", fn_14178ab60 as *const ()), // sub_14178ab60
    Replacement::draft(0x14178ac10, "ak_14178ac10", tpl_92818::<0x44, 0x50, 0x48, 0x40, 0x14178b080> as *const ()), // sub_14178ac10
    Replacement::draft(0x14178ac80, "ak_14178ac80", fn_14178ac80 as *const ()), // sub_14178ac80
    Replacement::draft(0x14178ad00, "ak_14178ad00", fn_14178ad00 as *const ()), // sub_14178ad00
    Replacement::draft(0x14178afb0, "ak_14178afb0", fn_14178afb0 as *const ()), // sub_14178afb0
    Replacement::draft(0x14178b000, "ak_14178b000", fn_14178b000 as *const ()), // sub_14178b000
    Replacement::draft(0x14178b080, "ak_14178b080", fn_14178b080 as *const ()), // sub_14178b080
    Replacement::draft(0x14178b110, "ak_14178b110", tpl_92714::<0x141776060, 0x141775da0, 0x14178b000, 0x141776050> as *const ()), // sub_14178b110
    Replacement::draft(0x14178b2c0, "ak_14178b2c0", fn_14178b2c0 as *const ()), // sub_14178b2c0
    Replacement::draft(0x14178b7f0, "ak_14178b7f0", fn_14178b7f0 as *const ()), // sub_14178b7f0
    Replacement::draft(0x14178b820, "ak_14178b820", fn_14178b820 as *const ()), // sub_14178b820
    Replacement::draft(0x14178b900, "ak_14178b900", fn_14178b900 as *const ()), // sub_14178b900
    Replacement::draft(0x14178b980, "ak_14178b980", fn_14178b980 as *const ()), // sub_14178b980
    Replacement::draft(0x14178b9a0, "ak_14178b9a0", fn_14178b9a0 as *const ()), // sub_14178b9a0
    Replacement::draft(0x14178c1d0, "ak_14178c1d0", fn_14178c1d0 as *const ()), // sub_14178c1d0
    Replacement::draft(0x14178c3a0, "ak_14178c3a0", fn_14178c3a0 as *const ()), // sub_14178c3a0
    Replacement::draft(0x14178c460, "ak_14178c460", tpl_33265::<0x8, 0x38, 0x34, 0x1417843a0, 0x48, 0x141784790> as *const ()), // sub_14178c460
    Replacement::draft(0x14178c4b0, "ak_14178c4b0", tpl_33266::<0x8, 0x34, 0x38, 0x1417843a0, 0x48, 0x141784790> as *const ()), // sub_14178c4b0
    Replacement::draft(0x14178c500, "ak_14178c500", tpl_33265::<0x8, 0x30, 0x2c, 0x1417839e0, 0x40, 0x141784790> as *const ()), // sub_14178c500
    Replacement::draft(0x14178c550, "ak_14178c550", tpl_33266::<0x8, 0x2c, 0x30, 0x1417839e0, 0x40, 0x141784790> as *const ()), // sub_14178c550
    Replacement::draft(0x14178c5a0, "ak_14178c5a0", fn_14178c5a0 as *const ()), // sub_14178c5a0
    Replacement::draft(0x14178c7f0, "ak_14178c7f0", fn_14178c7f0 as *const ()), // sub_14178c7f0
    Replacement::draft(0x14178c910, "ak_14178c910", fn_14178c910 as *const ()), // sub_14178c910
    Replacement::draft(0x14178c9f0, "ak_14178c9f0", fn_14178c9f0 as *const ()), // sub_14178c9f0
    Replacement::draft(0x14178ca60, "ak_14178ca60", fn_14178ca60 as *const ()), // sub_14178ca60
    Replacement::draft(0x14178cac0, "ak_14178cac0", fn_14178cac0 as *const ()), // sub_14178cac0
    Replacement::draft(0x14178cbb0, "ak_14178cbb0", fn_14178cbb0 as *const ()), // sub_14178cbb0
    Replacement::draft(0x14178cc90, "ak_14178cc90", fn_14178cc90 as *const ()), // sub_14178cc90
    Replacement::draft(0x14178cd00, "ak_14178cd00", fn_14178cd00 as *const ()), // sub_14178cd00
    Replacement::draft(0x14178cd70, "ak_14178cd70", tpl_33275::<0x1426afca0, 0xb, 0x8, 0x1426afc50, 0x10, 0x18> as *const ()), // sub_14178cd70
    Replacement::draft(0x14178cdc0, "ak_14178cdc0", fn_14178cdc0 as *const ()), // sub_14178cdc0
    Replacement::draft(0x14178cfa0, "ak_14178cfa0", tpl_33277::<0x10, 0x18, 0x3c, 0x30, 0x1417775f0, 0x1426afca0, 0x5c, 0x8, 0x4, 0x14178d3e0, 0x141776060> as *const ()), // sub_14178cfa0
    Replacement::draft(0x14178d080, "ak_14178d080", tpl_30911::<0x8, 0x1401de410> as *const ()), // sub_14178d080
    Replacement::draft(0x14178d0a0, "ak_14178d0a0", tpl_33124::<0x8, 0x14178da20, 0x10> as *const ()), // sub_14178d0a0
    Replacement::draft(0x14178d100, "ak_14178d100", tpl_21733::<0x1426712e0, 0x141834d30> as *const ()), // sub_14178d100
    Replacement::draft(0x14178d130, "ak_14178d130", fn_14178d130 as *const ()), // sub_14178d130
    Replacement::draft(0x14178d1b0, "ak_14178d1b0", tpl_33227::<0x1426afd50, 0x2b, 0x8, 0x1426afd18> as *const ()), // sub_14178d1b0
    Replacement::draft(0x14178d1f0, "ak_14178d1f0", fn_14178d1f0 as *const ()), // sub_14178d1f0
    Replacement::draft(0x14178d250, "ak_14178d250", fn_14178d250 as *const ()), // sub_14178d250
    Replacement::draft(0x14178d360, "ak_14178d360", fn_14178d360 as *const ()), // sub_14178d360
    Replacement::draft(0x14178d3e0, "ak_14178d3e0", fn_14178d3e0 as *const ()), // sub_14178d3e0
    Replacement::draft(0x14178d9c0, "ak_14178d9c0", fn_14178d9c0 as *const ()), // sub_14178d9c0
    Replacement::draft(0x14178da20, "ak_14178da20", fn_14178da20 as *const ()), // sub_14178da20
    Replacement::draft(0x14178dab0, "ak_14178dab0", fn_14178dab0 as *const ()), // sub_14178dab0
    Replacement::draft(0x14178daf0, "ak_14178daf0", tpl_92818::<0x44, 0x4c, 0x48, 0x40, 0x14178df80> as *const ()), // sub_14178daf0
    Replacement::draft(0x14178db60, "ak_14178db60", fn_14178db60 as *const ()), // sub_14178db60
    Replacement::draft(0x14178dbd0, "ak_14178dbd0", fn_14178dbd0 as *const ()), // sub_14178dbd0
    Replacement::draft(0x14178dc40, "ak_14178dc40", tpl_33286::<0x40, 0x1, 0x14178eae0> as *const ()), // sub_14178dc40
    Replacement::draft(0x14178dcb0, "ak_14178dcb0", tpl_33286::<0x40, 0xfffffffffffffffe, 0x14178eae0> as *const ()), // sub_14178dcb0
    Replacement::draft(0x14178dd20, "ak_14178dd20", tpl_33286::<0x40, 0xffffffffffffffff, 0x14178eae0> as *const ()), // sub_14178dd20
    Replacement::draft(0x14178dd90, "ak_14178dd90", fn_14178dd90 as *const ()), // sub_14178dd90
    Replacement::draft(0x14178de60, "ak_14178de60", fn_14178de60 as *const ()), // sub_14178de60
    Replacement::draft(0x14178df80, "ak_14178df80", fn_14178df80 as *const ()), // sub_14178df80
    Replacement::draft(0x14178e050, "ak_14178e050", fn_14178e050 as *const ()), // sub_14178e050
    Replacement::draft(0x14178e280, "ak_14178e280", fn_14178e280 as *const ()), // sub_14178e280
    Replacement::draft(0x14178e9b0, "ak_14178e9b0", tpl_1674::<0x1426afdd0> as *const ()), // sub_14178e9b0
    Replacement::draft(0x14178e9c0, "ak_14178e9c0", fn_14178e9c0 as *const ()), // sub_14178e9c0
    Replacement::draft(0x14178ea20, "ak_14178ea20", fn_14178ea20 as *const ()), // sub_14178ea20
    Replacement::draft(0x14178eae0, "ak_14178eae0", fn_14178eae0 as *const ()), // sub_14178eae0
    Replacement::draft(0x14178ec90, "ak_14178ec90", fn_14178ec90 as *const ()), // sub_14178ec90
    Replacement::draft(0x14178ee40, "ak_14178ee40", fn_14178ee40 as *const ()), // sub_14178ee40
    Replacement::draft(0x14178efb0, "ak_14178efb0", fn_14178efb0 as *const ()), // sub_14178efb0
    Replacement::draft(0x14178f500, "ak_14178f500", fn_14178f500 as *const ()), // sub_14178f500
    Replacement::draft(0x14178f850, "luau_14178f850", fn_14178f850 as *const ()), // sub_14178f850
    Replacement::draft(0x14178fc60, "luau_14178fc60", fn_14178fc60 as *const ()), // sub_14178fc60
    Replacement::draft(0x141790330, "ak_141790330", tpl_33297::<0x1434268d0, 0x1426b1ee0, 0x14178ee40> as *const ()), // sub_141790330
    Replacement::draft(0x141790360, "ak_141790360", tpl_33297::<0x1434268d0, 0x1426b03f0, 0x14178ee40> as *const ()), // sub_141790360
    Replacement::draft(0x141790390, "ak_141790390", tpl_33297::<0x1434268d0, 0x1426b39d0, 0x14178ee40> as *const ()), // sub_141790390
    Replacement::draft(0x1417903c0, "ak_1417903c0", tpl_33297::<0x1434268d0, 0x1426b54c0, 0x14178ee40> as *const ()), // sub_1417903c0
    Replacement::draft(0x1417903f0, "ak_1417903f0", tpl_33297::<0x1434268d0, 0x1426b6fb0, 0x14178ee40> as *const ()), // sub_1417903f0
    Replacement::draft(0x141790420, "ak_141790420", tpl_33297::<0x1434268d0, 0x1426b8aa0, 0x14178ee40> as *const ()), // sub_141790420
    Replacement::draft(0x141790450, "ak_141790450", tpl_33297::<0x1434268d0, 0x1426ba590, 0x14178ee40> as *const ()), // sub_141790450
    Replacement::draft(0x141790480, "ak_141790480", tpl_33297::<0x1434268d0, 0x1426bc080, 0x14178ee40> as *const ()), // sub_141790480
    Replacement::draft(0x1417904b0, "ak_1417904b0", tpl_33297::<0x1434268d0, 0x1426bdb70, 0x14178ee40> as *const ()), // sub_1417904b0
    Replacement::draft(0x1417904e0, "ak_1417904e0", tpl_33297::<0x1434268d0, 0x1426bf660, 0x14178ee40> as *const ()), // sub_1417904e0
    Replacement::draft(0x141790510, "ak_141790510", fn_141790510 as *const ()), // sub_141790510
    Replacement::draft(0x1417906b0, "ak_1417906b0", fn_1417906b0 as *const ()), // sub_1417906b0
    Replacement::draft(0x141790930, "ak_141790930", fn_141790930 as *const ()), // sub_141790930
    Replacement::draft(0x141790b70, "ak_141790b70", fn_141790b70 as *const ()), // sub_141790b70
    Replacement::draft(0x141790d90, "ak_141790d90", fn_141790d90 as *const ()), // sub_141790d90
    Replacement::draft(0x141790e60, "ak_141790e60", fn_141790e60 as *const ()), // sub_141790e60
    Replacement::draft(0x141791120, "ak_141791120", fn_141791120 as *const ()), // sub_141791120
    Replacement::draft(0x141791370, "ak_141791370", fn_141791370 as *const ()), // sub_141791370
    Replacement::draft(0x141791620, "ak_141791620", fn_141791620 as *const ()), // sub_141791620
    Replacement::draft(0x1417917b0, "ak_1417917b0", fn_1417917b0 as *const ()), // sub_1417917b0
    Replacement::draft(0x141791930, "ak_141791930", fn_141791930 as *const ()), // sub_141791930
    Replacement::draft(0x141791e40, "ak_141791e40", fn_141791e40 as *const ()), // sub_141791e40
    Replacement::draft(0x141792050, "ak_141792050", fn_141792050 as *const ()), // sub_141792050
    Replacement::draft(0x1417921a0, "ak_1417921a0", fn_1417921a0 as *const ()), // sub_1417921a0
    Replacement::draft(0x1417922b0, "ak_1417922b0", fn_1417922b0 as *const ()), // sub_1417922b0
    Replacement::draft(0x1417924c0, "ak_1417924c0", fn_1417924c0 as *const ()), // sub_1417924c0
    Replacement::draft(0x141792690, "ak_141792690", fn_141792690 as *const ()), // sub_141792690
    Replacement::draft(0x1417927d0, "ak_1417927d0", fn_1417927d0 as *const ()), // sub_1417927d0
    Replacement::draft(0x141792990, "ak_141792990", fn_141792990 as *const ()), // sub_141792990
    Replacement::draft(0x141792b50, "ak_141792b50", fn_141792b50 as *const ()), // sub_141792b50
    Replacement::draft(0x141792ce0, "ak_141792ce0", fn_141792ce0 as *const ()), // sub_141792ce0
    Replacement::draft(0x141792f10, "ak_141792f10", tpl_33305::<0x14342672c, 0x143426650, 0x14178ec90> as *const ()), // sub_141792f10
    Replacement::draft(0x141792f40, "ak_141792f40", tpl_33305::<0x14342672c, 0x1434268d0, 0x14178ec90> as *const ()), // sub_141792f40
    Replacement::draft(0x141792f70, "ak_141792f70", fn_141792f70 as *const ()), // sub_141792f70
    Replacement::draft(0x141792fa0, "ak_141792fa0", fn_141792fa0 as *const ()), // sub_141792fa0
    Replacement::draft(0x141792fd0, "ak_141792fd0", tpl_33305::<0x1434267cc, 0x1434268d0, 0x14178ec90> as *const ()), // sub_141792fd0
    Replacement::draft(0x141793000, "ak_141793000", fn_141793000 as *const ()), // sub_141793000
    Replacement::draft(0x1417931e0, "ak_1417931e0", fn_1417931e0 as *const ()), // sub_1417931e0
    Replacement::draft(0x1417932f0, "ak_1417932f0", fn_1417932f0 as *const ()), // sub_1417932f0
    Replacement::draft(0x1417933e0, "ak_1417933e0", fn_1417933e0 as *const ()), // sub_1417933e0
    Replacement::draft(0x141793590, "ak_141793590", tpl_33275::<0x1426c2200, 0xb, 0x8, 0x1426c21b0, 0x10, 0x18> as *const ()), // sub_141793590
    Replacement::draft(0x1417935e0, "ak_1417935e0", fn_1417935e0 as *const ()), // sub_1417935e0
    Replacement::draft(0x141793780, "ak_141793780", tpl_33277::<0x10, 0x18, 0x34, 0x30, 0x1417775f0, 0x1426c2200, 0x5c, 0x8, 0x4, 0x141793b50, 0x141776060> as *const ()), // sub_141793780
    Replacement::draft(0x141793860, "ak_141793860", tpl_30911::<0x8, 0x1417944e0> as *const ()), // sub_141793860
    Replacement::draft(0x141793880, "ak_141793880", tpl_33124::<0x8, 0x141794060, 0x10> as *const ()), // sub_141793880
    Replacement::draft(0x1417938e0, "ak_1417938e0", fn_1417938e0 as *const ()), // sub_1417938e0
    Replacement::draft(0x141793950, "ak_141793950", tpl_33227::<0x1426c22b0, 0x23, 0x8, 0x1426c2278> as *const ()), // sub_141793950
    Replacement::draft(0x141793990, "ak_141793990", fn_141793990 as *const ()), // sub_141793990
    Replacement::draft(0x1417939f0, "ak_1417939f0", fn_1417939f0 as *const ()), // sub_1417939f0
    Replacement::draft(0x141793ae0, "ak_141793ae0", fn_141793ae0 as *const ()), // sub_141793ae0
    Replacement::draft(0x141793b50, "ak_141793b50", fn_141793b50 as *const ()), // sub_141793b50
    Replacement::draft(0x141793f90, "ak_141793f90", fn_141793f90 as *const ()), // sub_141793f90
    Replacement::draft(0x141794000, "ak_141794000", tpl_33151::<0x8, 0x141793f90, 0x141776060, 0x141776050> as *const ()), // sub_141794000
    Replacement::draft(0x141794060, "ak_141794060", fn_141794060 as *const ()), // sub_141794060
    Replacement::draft(0x1417940f0, "ak_1417940f0", fn_1417940f0 as *const ()), // sub_1417940f0
    Replacement::draft(0x1417941d0, "ak_1417941d0", fn_1417941d0 as *const ()), // sub_1417941d0
    Replacement::draft(0x1417941f0, "ak_1417941f0", fn_1417941f0 as *const ()), // sub_1417941f0
    Replacement::draft(0x1417944e0, "ak_1417944e0", tpl_33180::<0x40, 0x141794f80> as *const ()), // sub_1417944e0
    Replacement::draft(0x141794520, "ak_141794520", tpl_92687::<0x141776060, 0x141775da0, 0x141793f90, 0x141776050> as *const ()), // sub_141794520
    Replacement::draft(0x1417946f0, "ak_1417946f0", fn_1417946f0 as *const ()), // sub_1417946f0
    Replacement::draft(0x141794920, "ak_141794920", fn_141794920 as *const ()), // sub_141794920
    Replacement::draft(0x141794d60, "ak_141794d60", fn_141794d60 as *const ()), // sub_141794d60
    Replacement::draft(0x141794f80, "ak_141794f80", fn_141794f80 as *const ()), // sub_141794f80
    Replacement::draft(0x141795040, "ak_141795040", fn_141795040 as *const ()), // sub_141795040
    Replacement::draft(0x141795290, "ak_141795290", fn_141795290 as *const ()), // sub_141795290
    Replacement::draft(0x1417952f0, "ak_1417952f0", tpl_33275::<0x1426c23b0, 0xb, 0x8, 0x1426c2360, 0x10, 0x18> as *const ()), // sub_1417952f0
    Replacement::draft(0x141795340, "ak_141795340", fn_141795340 as *const ()), // sub_141795340
    Replacement::draft(0x141795760, "ak_141795760", fn_141795760 as *const ()), // sub_141795760
    Replacement::draft(0x141795920, "ak_141795920", tpl_30911::<0x8, 0x141796f70> as *const ()), // sub_141795920
    Replacement::draft(0x141795940, "ak_141795940", tpl_33124::<0x8, 0x141796610, 0x10> as *const ()), // sub_141795940
    Replacement::draft(0x1417959a0, "ak_1417959a0", fn_1417959a0 as *const ()), // sub_1417959a0
    Replacement::draft(0x141795a50, "ak_141795a50", fn_141795a50 as *const ()), // sub_141795a50
    Replacement::draft(0x141795a90, "ak_141795a90", fn_141795a90 as *const ()), // sub_141795a90
    Replacement::draft(0x141795b90, "ak_141795b90", fn_141795b90 as *const ()), // sub_141795b90
    Replacement::draft(0x141795e80, "ak_141795e80", fn_141795e80 as *const ()), // sub_141795e80
    Replacement::draft(0x141795ff0, "ak_141795ff0", fn_141795ff0 as *const ()), // sub_141795ff0
    Replacement::draft(0x1417960b0, "ak_1417960b0", fn_1417960b0 as *const ()), // sub_1417960b0
    Replacement::draft(0x141796550, "ak_141796550", fn_141796550 as *const ()), // sub_141796550
    Replacement::draft(0x141796590, "ak_141796590", fn_141796590 as *const ()), // sub_141796590
    Replacement::draft(0x141796610, "ak_141796610", fn_141796610 as *const ()), // sub_141796610
    Replacement::draft(0x141796680, "ak_141796680", fn_141796680 as *const ()), // sub_141796680
    Replacement::draft(0x141796740, "ak_141796740", fn_141796740 as *const ()), // sub_141796740
    Replacement::draft(0x1417967b0, "ak_1417967b0", fn_1417967b0 as *const ()), // sub_1417967b0
    Replacement::draft(0x141796830, "ak_141796830", fn_141796830 as *const ()), // sub_141796830
    Replacement::draft(0x1417968b0, "ak_1417968b0", fn_1417968b0 as *const ()), // sub_1417968b0
    Replacement::draft(0x141796930, "ak_141796930", fn_141796930 as *const ()), // sub_141796930
    Replacement::draft(0x1417969b0, "ak_1417969b0", fn_1417969b0 as *const ()), // sub_1417969b0
    Replacement::draft(0x141796a30, "ak_141796a30", fn_141796a30 as *const ()), // sub_141796a30
    Replacement::draft(0x141796ab0, "ak_141796ab0", tpl_33336::<0x60, 0x141798220> as *const ()), // sub_141796ab0
    Replacement::draft(0x141796b20, "ak_141796b20", fn_141796b20 as *const ()), // sub_141796b20
    Replacement::draft(0x141796ba0, "ak_141796ba0", fn_141796ba0 as *const ()), // sub_141796ba0
    Replacement::draft(0x141796c20, "ak_141796c20", tpl_33336::<0x60, 0x141798390> as *const ()), // sub_141796c20
    Replacement::draft(0x141796c90, "ak_141796c90", tpl_33336::<0x60, 0x141797b50> as *const ()), // sub_141796c90
    Replacement::draft(0x141796d00, "ak_141796d00", fn_141796d00 as *const ()), // sub_141796d00
    Replacement::draft(0x141796dc0, "ak_141796dc0", fn_141796dc0 as *const ()), // sub_141796dc0
    Replacement::draft(0x141796f70, "ak_141796f70", tpl_33180::<0x60, 0x1417983c0> as *const ()), // sub_141796f70
    Replacement::draft(0x141796fb0, "ak_141796fb0", fn_141796fb0 as *const ()), // sub_141796fb0
    Replacement::draft(0x141797110, "ak_141797110", fn_141797110 as *const ()), // sub_141797110
    Replacement::draft(0x1417974b0, "ak_1417974b0", fn_1417974b0 as *const ()), // sub_1417974b0
    Replacement::draft(0x141797510, "ak_141797510", tpl_33160::<0x8, 0x141798420, 0x141776060, 0x141776050> as *const ()), // sub_141797510
    Replacement::draft(0x141797540, "ak_141797540", fn_141797540 as *const ()), // sub_141797540
    Replacement::draft(0x141797580, "ak_141797580", fn_141797580 as *const ()), // sub_141797580
    Replacement::draft(0x1417975f0, "ak_1417975f0", tpl_33344::<0x1426c25e0, 0x141798a50, 0x141834d30> as *const ()), // sub_1417975f0
    Replacement::draft(0x141797630, "ak_141797630", fn_141797630 as *const ()), // sub_141797630
    Replacement::draft(0x141797670, "ak_141797670", tpl_33344::<0x1426c2630, 0x141798a50, 0x141834d30> as *const ()), // sub_141797670
    Replacement::draft(0x1417976b0, "ak_1417976b0", fn_1417976b0 as *const ()), // sub_1417976b0
    Replacement::draft(0x1417976f0, "ak_1417976f0", fn_1417976f0 as *const ()), // sub_1417976f0
    Replacement::draft(0x141797730, "ak_141797730", fn_141797730 as *const ()), // sub_141797730
    Replacement::draft(0x141797770, "ak_141797770", fn_141797770 as *const ()), // sub_141797770
    Replacement::draft(0x141797b50, "ak_141797b50", fn_141797b50 as *const ()), // sub_141797b50
    Replacement::draft(0x141798220, "ak_141798220", fn_141798220 as *const ()), // sub_141798220
    Replacement::draft(0x141798250, "ak_141798250", fn_141798250 as *const ()), // sub_141798250
    Replacement::draft(0x141798390, "ak_141798390", fn_141798390 as *const ()), // sub_141798390
    Replacement::draft(0x1417983c0, "ak_1417983c0", fn_1417983c0 as *const ()), // sub_1417983c0
    Replacement::draft(0x141798420, "ak_141798420", fn_141798420 as *const ()), // sub_141798420
    Replacement::draft(0x1417984a0, "ak_1417984a0", fn_1417984a0 as *const ()), // sub_1417984a0
    Replacement::draft(0x141798560, "ak_141798560", tpl_92714::<0x141776060, 0x141775da0, 0x141798420, 0x141776050> as *const ()), // sub_141798560
    Replacement::draft(0x141798710, "ak_141798710", fn_141798710 as *const ()), // sub_141798710
    Replacement::draft(0x141798800, "ak_141798800", fn_141798800 as *const ()), // sub_141798800
    Replacement::draft(0x141798970, "ak_141798970", fn_141798970 as *const ()), // sub_141798970
    Replacement::draft(0x141798a50, "ak_141798a50", tpl_1674::<0x1426c2868> as *const ()), // sub_141798a50
    Replacement::draft(0x141798a60, "ak_141798a60", tpl_21733::<0x1426c2868, 0x141834d30> as *const ()), // sub_141798a60
    Replacement::draft(0x141798a90, "ak_141798a90", fn_141798a90 as *const ()), // sub_141798a90
    Replacement::draft(0x141798b00, "ak_141798b00", fn_141798b00 as *const ()), // sub_141798b00
    Replacement::draft(0x141799120, "ak_141799120", fn_141799120 as *const ()), // sub_141799120
    Replacement::draft(0x141799320, "ak_141799320", fn_141799320 as *const ()), // sub_141799320
    Replacement::draft(0x141799830, "ak_141799830", fn_141799830 as *const ()), // sub_141799830
    Replacement::draft(0x14179a4e0, "ak_14179a4e0", fn_14179a4e0 as *const ()), // sub_14179a4e0
    Replacement::draft(0x14179a990, "ak_14179a990", fn_14179a990 as *const ()), // sub_14179a990
    Replacement::draft(0x14179a9a0, "ak_14179a9a0", fn_14179a9a0 as *const ()), // sub_14179a9a0
    Replacement::draft(0x14179bb50, "ak_14179bb50", fn_14179bb50 as *const ()), // sub_14179bb50
    Replacement::draft(0x14179c560, "ak_14179c560", fn_14179c560 as *const ()), // sub_14179c560
    Replacement::draft(0x14179c620, "ak_14179c620", fn_14179c620 as *const ()), // sub_14179c620
    Replacement::draft(0x14179c720, "ak_14179c720", fn_14179c720 as *const ()), // sub_14179c720
    Replacement::draft(0x14179c850, "ak_14179c850", fn_14179c850 as *const ()), // sub_14179c850
    Replacement::draft(0x14179c900, "ak_14179c900", fn_14179c900 as *const ()), // sub_14179c900
    Replacement::draft(0x14179c9b0, "ak_14179c9b0", fn_14179c9b0 as *const ()), // sub_14179c9b0
    Replacement::draft(0x14179cba0, "ak_14179cba0", fn_14179cba0 as *const ()), // sub_14179cba0
    Replacement::draft(0x14179ccc0, "ak_14179ccc0", fn_14179ccc0 as *const ()), // sub_14179ccc0
    Replacement::draft(0x14179cd40, "ak_14179cd40", tpl_6310::<0x60, 0x58> as *const ()), // sub_14179cd40
    Replacement::draft(0x14179cd50, "ak_14179cd50", fn_14179cd50 as *const ()), // sub_14179cd50
    Replacement::draft(0x14179cde0, "ak_14179cde0", tpl_33366::<0x58, 0x50, 0x48, 0x40, 0x78, 0x70, 0x68, 0x60> as *const ()), // sub_14179cde0
    Replacement::draft(0x14179ce10, "ak_14179ce10", fn_14179ce10 as *const ()), // sub_14179ce10
    Replacement::draft(0x14179ce30, "ak_14179ce30", tpl_33366::<0x80, 0x78, 0x70, 0x68, 0xa0, 0x98, 0x90, 0x88> as *const ()), // sub_14179ce30
    Replacement::draft(0x14179ce70, "ak_14179ce70", tpl_27317::<0x40, 0x141784b70> as *const ()), // sub_14179ce70
    Replacement::draft(0x14179ce80, "ak_14179ce80", fn_14179ce80 as *const ()), // sub_14179ce80
    Replacement::draft(0x14179ce90, "ak_14179ce90", fn_14179ce90 as *const ()), // sub_14179ce90
    Replacement::draft(0x14179cee0, "ak_14179cee0", fn_14179cee0 as *const ()), // sub_14179cee0
    Replacement::draft(0x14179cf80, "ak_14179cf80", tpl_33371::<0xa0, 0x143426780, 0x143426a48, 0x1434268b0, 0x142679bc8, 0x1426c2908, 0x48, 0x1434269f8, 0x40, 0x50> as *const ()), // sub_14179cf80
    Replacement::draft(0x14179d020, "ak_14179d020", fn_14179d020 as *const ()), // sub_14179d020
    Replacement::draft(0x14179d140, "ak_14179d140", tpl_33371::<0x80, 0x143426780, 0x143426a48, 0x1434268b0, 0x142679bc8, 0x1426c2908, 0x48, 0x1434269f8, 0x40, 0x50> as *const ()), // sub_14179d140
    Replacement::draft(0x14179d1e0, "ak_14179d1e0", fn_14179d1e0 as *const ()), // sub_14179d1e0
    Replacement::draft(0x14179d280, "ak_14179d280", fn_14179d280 as *const ()), // sub_14179d280
    Replacement::draft(0x14179d470, "ak_14179d470", fn_14179d470 as *const ()), // sub_14179d470
    Replacement::draft(0x14179d590, "ak_14179d590", fn_14179d590 as *const ()), // sub_14179d590
    Replacement::draft(0x14179d600, "ak_14179d600", fn_14179d600 as *const ()), // sub_14179d600
    Replacement::draft(0x14179e300, "ak_14179e300", fn_14179e300 as *const ()), // sub_14179e300
    Replacement::draft(0x14179e350, "ak_14179e350", fn_14179e350 as *const ()), // sub_14179e350
    Replacement::draft(0x14179e7b0, "ak_14179e7b0", fn_14179e7b0 as *const ()), // sub_14179e7b0
    Replacement::draft(0x14179e8a0, "ak_14179e8a0", tpl_33124::<0x8, 0x14179f980, 0x10> as *const ()), // sub_14179e8a0
    Replacement::draft(0x14179e900, "ak_14179e900", fn_14179e900 as *const ()), // sub_14179e900
    Replacement::draft(0x14179e930, "ak_14179e930", fn_14179e930 as *const ()), // sub_14179e930
    Replacement::draft(0x14179ea00, "ak_14179ea00", fn_14179ea00 as *const ()), // sub_14179ea00
    Replacement::draft(0x14179ea40, "unknown_14179ea40", fn_14179ea40 as *const ()), // sub_14179ea40
    Replacement::draft(0x14179eb30, "unknown_14179eb30", fn_14179eb30 as *const ()), // sub_14179eb30
    Replacement::draft(0x14179eea0, "unknown_14179eea0", fn_14179eea0 as *const ()), // sub_14179eea0
    Replacement::draft(0x14179f040, "ak_14179f040", fn_14179f040 as *const ()), // sub_14179f040
    Replacement::draft(0x14179f8f0, "unknown_14179f8f0", fn_14179f8f0 as *const ()), // sub_14179f8f0
    Replacement::draft(0x14179f950, "unknown_14179f950", tpl_33160::<0x8, 0x1417a0280, 0x141776060, 0x141776050> as *const ()), // sub_14179f950
    Replacement::draft(0x14179f980, "unknown_14179f980", fn_14179f980 as *const ()), // sub_14179f980
    Replacement::draft(0x14179fa20, "unknown_14179fa20", tpl_92818::<0x64, 0x6c, 0x68, 0x60, 0x1417a0300> as *const ()), // sub_14179fa20
    Replacement::draft(0x14179fa90, "unknown_14179fa90", fn_14179fa90 as *const ()), // sub_14179fa90
    Replacement::draft(0x14179fb00, "unknown_14179fb00", fn_14179fb00 as *const ()), // sub_14179fb00
    Replacement::draft(0x14179fb80, "unknown_14179fb80", fn_14179fb80 as *const ()), // sub_14179fb80
    Replacement::draft(0x14179fbf0, "unknown_14179fbf0", fn_14179fbf0 as *const ()), // sub_14179fbf0
    Replacement::draft(0x14179fc70, "unknown_14179fc70", fn_14179fc70 as *const ()), // sub_14179fc70
    Replacement::draft(0x14179fcf0, "unknown_14179fcf0", fn_14179fcf0 as *const ()), // sub_14179fcf0
    Replacement::draft(0x14179fd70, "unknown_14179fd70", fn_14179fd70 as *const ()), // sub_14179fd70
    Replacement::draft(0x14179fdf0, "unknown_14179fdf0", fn_14179fdf0 as *const ()), // sub_14179fdf0
    Replacement::draft(0x14179fee0, "unknown_14179fee0", fn_14179fee0 as *const ()), // sub_14179fee0
    Replacement::draft(0x1417a0280, "unknown_1417a0280", fn_1417a0280 as *const ()), // sub_1417a0280
    Replacement::draft(0x1417a0300, "unknown_1417a0300", fn_1417a0300 as *const ()), // sub_1417a0300
    Replacement::draft(0x1417a0470, "unknown_1417a0470", tpl_92714::<0x141776060, 0x141775da0, 0x1417a0280, 0x141776050> as *const ()), // sub_1417a0470
    Replacement::draft(0x1417a0620, "unknown_1417a0620", fn_1417a0620 as *const ()), // sub_1417a0620
    Replacement::draft(0x1417a1470, "unknown_1417a1470", fn_1417a1470 as *const ()), // sub_1417a1470
    Replacement::draft(0x1417a1670, "unknown_1417a1670", fn_1417a1670 as *const ()), // sub_1417a1670
    Replacement::draft(0x1417a19f0, "unknown_1417a19f0", fn_1417a19f0 as *const ()), // sub_1417a19f0
    Replacement::draft(0x1417a1a00, "unknown_1417a1a00", fn_1417a1a00 as *const ()), // sub_1417a1a00
    Replacement::draft(0x1417a1ae0, "unknown_1417a1ae0", fn_1417a1ae0 as *const ()), // sub_1417a1ae0
    Replacement::draft(0x1417a1c60, "unknown_1417a1c60", fn_1417a1c60 as *const ()), // sub_1417a1c60
    Replacement::draft(0x1417a1d50, "unknown_1417a1d50", fn_1417a1d50 as *const ()), // sub_1417a1d50
    Replacement::draft(0x1417a1dd0, "unknown_1417a1dd0", fn_1417a1dd0 as *const ()), // sub_1417a1dd0
    Replacement::draft(0x1417a1e80, "unknown_1417a1e80", fn_1417a1e80 as *const ()), // sub_1417a1e80
    Replacement::draft(0x1417a1f20, "unknown_1417a1f20", fn_1417a1f20 as *const ()), // sub_1417a1f20
    Replacement::draft(0x1417a1ff0, "unknown_1417a1ff0", fn_1417a1ff0 as *const ()), // sub_1417a1ff0
    Replacement::draft(0x1417a2060, "unknown_1417a2060", fn_1417a2060 as *const ()), // sub_1417a2060
    Replacement::draft(0x1417a21b0, "unknown_1417a21b0", fn_1417a21b0 as *const ()), // sub_1417a21b0
    Replacement::draft(0x1417a2270, "unknown_1417a2270", fn_1417a2270 as *const ()), // sub_1417a2270
    Replacement::draft(0x1417a2320, "unknown_1417a2320", fn_1417a2320 as *const ()), // sub_1417a2320
    Replacement::draft(0x1417a23c0, "unknown_1417a23c0", tpl_33406::<0x143932d08, 0x1417a7a40> as *const ()), // sub_1417a23c0
    Replacement::draft(0x1417a23e0, "unknown_1417a23e0", fn_1417a23e0 as *const ()), // sub_1417a23e0
    Replacement::draft(0x1417a24a0, "unknown_1417a24a0", fn_1417a24a0 as *const ()), // sub_1417a24a0
    Replacement::draft(0x1417a25b0, "unknown_1417a25b0", fn_1417a25b0 as *const ()), // sub_1417a25b0
    Replacement::draft(0x1417a27b0, "unknown_1417a27b0", fn_1417a27b0 as *const ()), // sub_1417a27b0
    Replacement::draft(0x1417a29b0, "unknown_1417a29b0", fn_1417a29b0 as *const ()), // sub_1417a29b0
    Replacement::draft(0x1417a2a60, "unknown_1417a2a60", fn_1417a2a60 as *const ()), // sub_1417a2a60
    Replacement::draft(0x1417a2de0, "unknown_1417a2de0", fn_1417a2de0 as *const ()), // sub_1417a2de0
    Replacement::draft(0x1417a2fc0, "unknown_1417a2fc0", fn_1417a2fc0 as *const ()), // sub_1417a2fc0
    Replacement::draft(0x1417a3120, "unknown_1417a3120", fn_1417a3120 as *const ()), // sub_1417a3120
    Replacement::draft(0x1417a33f0, "unknown_1417a33f0", fn_1417a33f0 as *const ()), // sub_1417a33f0
    Replacement::draft(0x1417a3760, "unknown_1417a3760", fn_1417a3760 as *const ()), // sub_1417a3760
    Replacement::draft(0x1417a3a40, "unknown_1417a3a40", fn_1417a3a40 as *const ()), // sub_1417a3a40
    Replacement::draft(0x1417a3b60, "unknown_1417a3b60", fn_1417a3b60 as *const ()), // sub_1417a3b60
    Replacement::draft(0x1417a3ec0, "unknown_1417a3ec0", fn_1417a3ec0 as *const ()), // sub_1417a3ec0
    Replacement::draft(0x1417a3f80, "unknown_1417a3f80", fn_1417a3f80 as *const ()), // sub_1417a3f80
    Replacement::draft(0x1417a41a0, "unknown_1417a41a0", fn_1417a41a0 as *const ()), // sub_1417a41a0
    Replacement::draft(0x1417a4230, "unknown_1417a4230", fn_1417a4230 as *const ()), // sub_1417a4230
    Replacement::draft(0x1417a4580, "unknown_1417a4580", fn_1417a4580 as *const ()), // sub_1417a4580
    Replacement::draft(0x1417a4590, "std_1417a4590", fn_1417a4590 as *const ()), // sub_1417a4590
    Replacement::draft(0x1417a45b0, "unknown_1417a45b0", fn_1417a45b0 as *const ()), // sub_1417a45b0
    Replacement::draft(0x1417a46b0, "unknown_1417a46b0", fn_1417a46b0 as *const ()), // sub_1417a46b0
    Replacement::draft(0x1417a4790, "unknown_1417a4790", fn_1417a4790 as *const ()), // sub_1417a4790
    Replacement::draft(0x1417a4c70, "unknown_1417a4c70", fn_1417a4c70 as *const ()), // sub_1417a4c70
    Replacement::draft(0x1417a4ce0, "unknown_1417a4ce0", fn_1417a4ce0 as *const ()), // sub_1417a4ce0
    Replacement::draft(0x1417a5000, "unknown_1417a5000", fn_1417a5000 as *const ()), // sub_1417a5000
    Replacement::draft(0x1417a5060, "unknown_1417a5060", fn_1417a5060 as *const ()), // sub_1417a5060
    Replacement::draft(0x1417a5100, "unknown_1417a5100", fn_1417a5100 as *const ()), // sub_1417a5100
    Replacement::draft(0x1417a5220, "unknown_1417a5220", fn_1417a5220 as *const ()), // sub_1417a5220
    Replacement::draft(0x1417a5290, "unknown_1417a5290", fn_1417a5290 as *const ()), // sub_1417a5290
    Replacement::draft(0x1417a5310, "unknown_1417a5310", fn_1417a5310 as *const ()), // sub_1417a5310
    Replacement::draft(0x1417a53a0, "unknown_1417a53a0", fn_1417a53a0 as *const ()), // sub_1417a53a0
    Replacement::draft(0x1417a5490, "unknown_1417a5490", fn_1417a5490 as *const ()), // sub_1417a5490
    Replacement::draft(0x1417a5850, "unknown_1417a5850", fn_1417a5850 as *const ()), // sub_1417a5850
    Replacement::draft(0x1417a5920, "unknown_1417a5920", fn_1417a5920 as *const ()), // sub_1417a5920
    Replacement::draft(0x1417a5a90, "unknown_1417a5a90", fn_1417a5a90 as *const ()), // sub_1417a5a90
    Replacement::draft(0x1417a5b40, "unknown_1417a5b40", fn_1417a5b40 as *const ()), // sub_1417a5b40
    Replacement::draft(0x1417a5b60, "unknown_1417a5b60", fn_1417a5b60 as *const ()), // sub_1417a5b60
    Replacement::draft(0x1417a5dc0, "unknown_1417a5dc0", fn_1417a5dc0 as *const ()), // sub_1417a5dc0
    Replacement::draft(0x1417a5df0, "unknown_1417a5df0", fn_1417a5df0 as *const ()), // sub_1417a5df0
    Replacement::draft(0x1417a5ec0, "unknown_1417a5ec0", fn_1417a5ec0 as *const ()), // sub_1417a5ec0
    Replacement::draft(0x1417a5ef0, "unknown_1417a5ef0", fn_1417a5ef0 as *const ()), // sub_1417a5ef0
    Replacement::draft(0x1417a5f20, "unknown_1417a5f20", fn_1417a5f20 as *const ()), // sub_1417a5f20
    Replacement::draft(0x1417a5f50, "unknown_1417a5f50", fn_1417a5f50 as *const ()), // sub_1417a5f50
    Replacement::draft(0x1417a5f80, "unknown_1417a5f80", fn_1417a5f80 as *const ()), // sub_1417a5f80
    Replacement::draft(0x1417a5fb0, "unknown_1417a5fb0", fn_1417a5fb0 as *const ()), // sub_1417a5fb0
    Replacement::draft(0x1417a5ff0, "unknown_1417a5ff0", fn_1417a5ff0 as *const ()), // sub_1417a5ff0
    Replacement::draft(0x1417a6020, "unknown_1417a6020", fn_1417a6020 as *const ()), // sub_1417a6020
    Replacement::draft(0x1417a60b0, "unknown_1417a60b0", fn_1417a60b0 as *const ()), // sub_1417a60b0
    Replacement::draft(0x1417a6120, "unknown_1417a6120", fn_1417a6120 as *const ()), // sub_1417a6120
    Replacement::draft(0x1417a61e0, "unknown_1417a61e0", fn_1417a61e0 as *const ()), // sub_1417a61e0
    Replacement::draft_throws(0x1417a62a0, "unknown_1417a62a0", fn_1417a62a0 as *const (), Throws::Discard), // sub_1417a62a0
    Replacement::draft_throws(0x1417a63e0, "std_1417a63e0", fn_1417a63e0 as *const (), Throws::Discard), // sub_1417a63e0
    Replacement::draft_throws(0x1417a64e0, "std_1417a64e0", fn_1417a64e0 as *const (), Throws::Discard), // sub_1417a64e0
    Replacement::draft(0x1417a65e0, "std_1417a65e0", fn_1417a65e0 as *const ()), // sub_1417a65e0
    Replacement::draft(0x1417a6620, "std_1417a6620", fn_1417a6620 as *const ()), // sub_1417a6620
    Replacement::draft(0x1417a6650, "std_1417a6650", fn_1417a6650 as *const ()), // sub_1417a6650
    Replacement::draft(0x1417a66a0, "std_1417a66a0", fn_1417a66a0 as *const ()), // sub_1417a66a0
    Replacement::draft(0x1417a66e0, "std_1417a66e0", fn_1417a66e0 as *const ()), // sub_1417a66e0
    Replacement::draft(0x1417a6ce0, "unknown_1417a6ce0", tpl_3548::<0x10, 0x19, 0x8> as *const ()), // sub_1417a6ce0
    Replacement::draft(0x1417a6d40, "unknown_1417a6d40", tpl_3547::<0x10, 0x19, 0x8> as *const ()), // sub_1417a6d40
    Replacement::draft(0x1417a6da0, "unknown_1417a6da0", fn_1417a6da0 as *const ()), // sub_1417a6da0
    Replacement::draft(0x1417a6e30, "unknown_1417a6e30", fn_1417a6e30 as *const ()), // sub_1417a6e30
    Replacement::draft(0x1417a7020, "unknown_1417a7020", fn_1417a7020 as *const ()), // sub_1417a7020
    Replacement::draft(0x1417a7100, "unknown_1417a7100", tpl_28185::<0x143947f61> as *const ()), // sub_1417a7100
    Replacement::draft(0x1417a7110, "unknown_1417a7110", tpl_28185::<0x143947f62> as *const ()), // sub_1417a7110
    Replacement::draft(0x1417a7120, "unknown_1417a7120", tpl_28185::<0x143947f63> as *const ()), // sub_1417a7120
    Replacement::draft(0x1417a7130, "unknown_1417a7130", fn_1417a7130 as *const ()), // sub_1417a7130
    Replacement::draft(0x1417a7380, "unknown_1417a7380", fn_1417a7380 as *const ()), // sub_1417a7380
    Replacement::draft(0x1417a7760, "unknown_1417a7760", fn_1417a7760 as *const ()), // sub_1417a7760
    Replacement::draft(0x1417a7a40, "unknown_1417a7a40", fn_1417a7a40 as *const ()), // sub_1417a7a40
    Replacement::draft(0x1417a7bc0, "unknown_1417a7bc0", fn_1417a7bc0 as *const ()), // sub_1417a7bc0
    Replacement::draft(0x1417a7c00, "unknown_1417a7c00", fn_1417a7c00 as *const ()), // sub_1417a7c00
    Replacement::draft(0x1417a7d80, "unknown_1417a7d80", fn_1417a7d80 as *const ()), // sub_1417a7d80
    Replacement::draft(0x1417a7f60, "unknown_1417a7f60", fn_1417a7f60 as *const ()), // sub_1417a7f60
    Replacement::draft(0x1417a8e70, "unknown_1417a8e70", fn_1417a8e70 as *const ()), // sub_1417a8e70
    Replacement::draft(0x1417a9130, "unknown_1417a9130", fn_1417a9130 as *const ()), // sub_1417a9130
    Replacement::draft(0x1417a94a0, "unknown_1417a94a0", fn_1417a94a0 as *const ()), // sub_1417a94a0
    Replacement::draft(0x1417a95c0, "unknown_1417a95c0", fn_1417a95c0 as *const ()), // sub_1417a95c0
    Replacement::draft(0x1417a97e0, "unknown_1417a97e0", fn_1417a97e0 as *const ()), // sub_1417a97e0
    Replacement::draft(0x1417a9850, "unknown_1417a9850", fn_1417a9850 as *const ()), // sub_1417a9850
    Replacement::draft(0x1417a99c0, "unknown_1417a99c0", fn_1417a99c0 as *const ()), // sub_1417a99c0
    Replacement::draft_throws(0x1417a9a00, "unknown_1417a9a00", fn_1417a9a00 as *const (), Throws::Discard), // sub_1417a9a00
    Replacement::draft(0x1417aa780, "unknown_1417aa780", fn_1417aa780 as *const ()), // sub_1417aa780
    Replacement::draft(0x1417aa850, "unknown_1417aa850", fn_1417aa850 as *const ()), // sub_1417aa850
    Replacement::draft(0x1417aaa60, "unknown_1417aaa60", fn_1417aaa60 as *const ()), // sub_1417aaa60
    Replacement::draft(0x1417aab70, "unknown_1417aab70", fn_1417aab70 as *const ()), // sub_1417aab70
    Replacement::draft_throws(0x1417aacb0, "unknown_1417aacb0", fn_1417aacb0 as *const (), Throws::Discard), // sub_1417aacb0
    Replacement::draft(0x1417acca0, "unknown_1417acca0", fn_1417acca0 as *const ()), // sub_1417acca0
    Replacement::draft(0x1417accd0, "unknown_1417accd0", fn_1417accd0 as *const ()), // sub_1417accd0
    Replacement::draft(0x1417ad170, "unknown_1417ad170", fn_1417ad170 as *const ()), // sub_1417ad170
    Replacement::draft(0x1417ad480, "unknown_1417ad480", fn_1417ad480 as *const ()), // sub_1417ad480
    Replacement::draft(0x1417ad5e0, "unknown_1417ad5e0", tpl_33474::<0x142646558, 0x143979e80> as *const ()), // sub_1417ad5e0
    Replacement::draft(0x1417ad600, "unknown_1417ad600", tpl_33474::<0x142646538, 0x143979e88> as *const ()), // sub_1417ad600
    Replacement::draft(0x1417ad620, "unknown_1417ad620", fn_1417ad620 as *const ()), // sub_1417ad620
    Replacement::draft(0x1417ad630, "unknown_1417ad630", fn_1417ad630 as *const ()), // sub_1417ad630
    Replacement::draft(0x1417ad7d0, "unknown_1417ad7d0", fn_1417ad7d0 as *const ()), // sub_1417ad7d0
    Replacement::draft_throws(0x1417ad890, "std_1417ad890", fn_1417ad890 as *const (), Throws::Discard), // sub_1417ad890
    Replacement::draft(0x1417adda0, "unknown_1417adda0", fn_1417adda0 as *const ()), // sub_1417adda0
    Replacement::draft(0x1417adf60, "unknown_1417adf60", fn_1417adf60 as *const ()), // sub_1417adf60
    Replacement::draft(0x1417ae140, "unknown_1417ae140", fn_1417ae140 as *const ()), // sub_1417ae140
    Replacement::draft(0x1417ae150, "unknown_1417ae150", fn_1417ae150 as *const ()), // sub_1417ae150
    Replacement::draft(0x1417ae650, "unknown_1417ae650", fn_1417ae650 as *const ()), // sub_1417ae650
    Replacement::draft(0x1417ae840, "unknown_1417ae840", tpl_33481::<0x143979e90, 0x143979e98, 0x1426452b0> as *const ()), // sub_1417ae840
    Replacement::draft(0x1417ae870, "unknown_1417ae870", tpl_33482::<0x143979e90, 0x143979e98, 0x1426452a8> as *const ()), // sub_1417ae870
    Replacement::draft(0x1417ae890, "unknown_1417ae890", tpl_33481::<0x143979e90, 0x143979ec0, 0x1426452b0> as *const ()), // sub_1417ae890
    Replacement::draft(0x1417ae8c0, "unknown_1417ae8c0", tpl_33482::<0x143979e90, 0x143979ec0, 0x1426452a8> as *const ()), // sub_1417ae8c0
    Replacement::draft(0x1417ae8e0, "unknown_1417ae8e0", fn_1417ae8e0 as *const ()), // sub_1417ae8e0
    Replacement::draft(0x1417ae920, "unknown_1417ae920", fn_1417ae920 as *const ()), // sub_1417ae920
    Replacement::draft(0x1417ae950, "unknown_1417ae950", fn_1417ae950 as *const ()), // sub_1417ae950
    Replacement::draft(0x1417ae9b0, "unknown_1417ae9b0", fn_1417ae9b0 as *const ()), // sub_1417ae9b0
    Replacement::draft(0x1417aea10, "unknown_1417aea10", fn_1417aea10 as *const ()), // sub_1417aea10
    Replacement::draft(0x1417aeb50, "unknown_1417aeb50", fn_1417aeb50 as *const ()), // sub_1417aeb50
    Replacement::draft(0x1417aeb90, "unknown_1417aeb90", fn_1417aeb90 as *const ()), // sub_1417aeb90
    Replacement::draft(0x1417aeba0, "unknown_1417aeba0", fn_1417aeba0 as *const ()), // sub_1417aeba0
    Replacement::draft(0x1417aebb0, "unknown_1417aebb0", fn_1417aebb0 as *const ()), // sub_1417aebb0
    Replacement::draft(0x1417aebc0, "unknown_1417aebc0", fn_1417aebc0 as *const ()), // sub_1417aebc0
    Replacement::draft(0x1417aef60, "unknown_1417aef60", fn_1417aef60 as *const ()), // sub_1417aef60
    Replacement::draft(0x1417af640, "unknown_1417af640", fn_1417af640 as *const ()), // sub_1417af640
    Replacement::draft(0x1417af9b0, "unknown_1417af9b0", fn_1417af9b0 as *const ()), // sub_1417af9b0
    Replacement::draft(0x1417afbb0, "unknown_1417afbb0", fn_1417afbb0 as *const ()), // sub_1417afbb0
    Replacement::draft(0x1417afc30, "unknown_1417afc30", fn_1417afc30 as *const ()), // sub_1417afc30
    Replacement::draft(0x1417afc70, "unknown_1417afc70", fn_1417afc70 as *const ()), // sub_1417afc70
    Replacement::draft(0x1417afe40, "unknown_1417afe40", fn_1417afe40 as *const ()), // sub_1417afe40
    Replacement::draft(0x1417b0290, "ak_1417b0290", fn_1417b0290 as *const ()), // sub_1417b0290
    Replacement::draft(0x1417b03a0, "ak_1417b03a0", fn_1417b03a0 as *const ()), // sub_1417b03a0
    Replacement::draft(0x1417b0460, "ak_1417b0460", fn_1417b0460 as *const ()), // sub_1417b0460
    Replacement::draft(0x1417b0780, "ak_1417b0780", fn_1417b0780 as *const ()), // sub_1417b0780
    Replacement::draft(0x1417b07b0, "ak_1417b07b0", fn_1417b07b0 as *const ()), // sub_1417b07b0
    Replacement::draft(0x1417b0870, "ak_1417b0870", fn_1417b0870 as *const ()), // sub_1417b0870
    Replacement::draft(0x1417b0940, "ak_1417b0940", fn_1417b0940 as *const ()), // sub_1417b0940
    Replacement::draft(0x1417b0980, "ak_1417b0980", fn_1417b0980 as *const ()), // sub_1417b0980
    Replacement::draft(0x1417b0a60, "ak_1417b0a60", fn_1417b0a60 as *const ()), // sub_1417b0a60
    Replacement::draft(0x1417b38a0, "ak_1417b38a0", fn_1417b38a0 as *const ()), // sub_1417b38a0
    Replacement::draft(0x1417b38c0, "ak_1417b38c0", fn_1417b38c0 as *const ()), // sub_1417b38c0
    Replacement::draft(0x1417b38e0, "ak_1417b38e0", fn_1417b38e0 as *const ()), // sub_1417b38e0
    Replacement::draft(0x1417b3a60, "ak_1417b3a60", fn_1417b3a60 as *const ()), // sub_1417b3a60
    Replacement::draft(0x1417b3b50, "ak_1417b3b50", fn_1417b3b50 as *const ()), // sub_1417b3b50
    Replacement::draft(0x1417b3cd0, "ak_1417b3cd0", fn_1417b3cd0 as *const ()), // sub_1417b3cd0
    Replacement::draft(0x1417b3d60, "ak_1417b3d60", fn_1417b3d60 as *const ()), // sub_1417b3d60
    Replacement::draft(0x1417b3ec0, "ak_1417b3ec0", fn_1417b3ec0 as *const ()), // sub_1417b3ec0
    Replacement::draft(0x1417b4570, "ak_1417b4570", fn_1417b4570 as *const ()), // sub_1417b4570
    Replacement::draft(0x1417b4810, "ak_1417b4810", fn_1417b4810 as *const ()), // sub_1417b4810
    Replacement::draft(0x1417b50c0, "ak_1417b50c0", fn_1417b50c0 as *const ()), // sub_1417b50c0
    Replacement::draft(0x1417b55d0, "ak_1417b55d0", fn_1417b55d0 as *const ()), // sub_1417b55d0
    Replacement::draft(0x1417b56d0, "ak_1417b56d0", fn_1417b56d0 as *const ()), // sub_1417b56d0
    Replacement::draft(0x1417b5db0, "ak_1417b5db0", fn_1417b5db0 as *const ()), // sub_1417b5db0
    Replacement::draft(0x1417b6020, "ak_1417b6020", fn_1417b6020 as *const ()), // sub_1417b6020
    Replacement::draft(0x1417b62d0, "ak_1417b62d0", fn_1417b62d0 as *const ()), // sub_1417b62d0
    Replacement::draft(0x1417b67c0, "ak_1417b67c0", fn_1417b67c0 as *const ()), // sub_1417b67c0
    Replacement::draft(0x1417b6910, "ak_1417b6910", fn_1417b6910 as *const ()), // sub_1417b6910
    Replacement::draft(0x1417b6990, "ak_1417b6990", fn_1417b6990 as *const ()), // sub_1417b6990
    Replacement::draft(0x1417b69c0, "ak_1417b69c0", fn_1417b69c0 as *const ()), // sub_1417b69c0
    Replacement::draft(0x1417b6af0, "ak_1417b6af0", fn_1417b6af0 as *const ()), // sub_1417b6af0
    Replacement::draft(0x1417b6d60, "ak_1417b6d60", fn_1417b6d60 as *const ()), // sub_1417b6d60
    Replacement::draft(0x1417b72e0, "ak_1417b72e0", fn_1417b72e0 as *const ()), // sub_1417b72e0
    Replacement::draft(0x1417b7900, "ak_1417b7900", fn_1417b7900 as *const ()), // sub_1417b7900
    Replacement::draft(0x1417b7930, "ak_1417b7930", fn_1417b7930 as *const ()), // sub_1417b7930
    Replacement::draft(0x1417b81e0, "ak_1417b81e0", fn_1417b81e0 as *const ()), // sub_1417b81e0
    Replacement::draft(0x1417b8220, "ak_1417b8220", fn_1417b8220 as *const ()), // sub_1417b8220
    Replacement::draft(0x1417b8670, "ak_1417b8670", fn_1417b8670 as *const ()), // sub_1417b8670
    Replacement::draft(0x1417b8760, "ak_1417b8760", fn_1417b8760 as *const ()), // sub_1417b8760
    Replacement::draft(0x1417b8890, "ak_1417b8890", fn_1417b8890 as *const ()), // sub_1417b8890
    Replacement::draft(0x1417b8b10, "ak_1417b8b10", fn_1417b8b10 as *const ()), // sub_1417b8b10
    Replacement::draft(0x1417b8bd0, "ak_1417b8bd0", fn_1417b8bd0 as *const ()), // sub_1417b8bd0
    Replacement::draft(0x1417b8fe0, "ak_1417b8fe0", fn_1417b8fe0 as *const ()), // sub_1417b8fe0
    Replacement::draft(0x1417b9400, "ak_1417b9400", fn_1417b9400 as *const ()), // sub_1417b9400
    Replacement::draft(0x1417b94f0, "ak_1417b94f0", fn_1417b94f0 as *const ()), // sub_1417b94f0
    Replacement::draft(0x1417b9600, "ak_1417b9600", fn_1417b9600 as *const ()), // sub_1417b9600
    Replacement::draft(0x1417b9840, "ak_1417b9840", fn_1417b9840 as *const ()), // sub_1417b9840
    Replacement::draft(0x1417b9910, "ak_1417b9910", fn_1417b9910 as *const ()), // sub_1417b9910
    Replacement::draft(0x1417b9a50, "ak_1417b9a50", fn_1417b9a50 as *const ()), // sub_1417b9a50
    Replacement::draft(0x1417b9b80, "ak_1417b9b80", fn_1417b9b80 as *const ()), // sub_1417b9b80
    Replacement::draft(0x1417b9c90, "ak_1417b9c90", fn_1417b9c90 as *const ()), // sub_1417b9c90
    Replacement::draft(0x1417b9e50, "ak_1417b9e50", fn_1417b9e50 as *const ()), // sub_1417b9e50
    Replacement::draft(0x1417ba090, "ak_1417ba090", fn_1417ba090 as *const ()), // sub_1417ba090
    Replacement::draft(0x1417ba1d0, "ak_1417ba1d0", fn_1417ba1d0 as *const ()), // sub_1417ba1d0
    Replacement::draft(0x1417ba240, "ak_1417ba240", fn_1417ba240 as *const ()), // sub_1417ba240
    Replacement::draft(0x1417ba360, "ak_1417ba360", fn_1417ba360 as *const ()), // sub_1417ba360
    Replacement::draft(0x1417ba550, "ak_1417ba550", fn_1417ba550 as *const ()), // sub_1417ba550
    Replacement::draft(0x1417ba800, "ak_1417ba800", fn_1417ba800 as *const ()), // sub_1417ba800
    Replacement::draft(0x1417ba870, "ak_1417ba870", fn_1417ba870 as *const ()), // sub_1417ba870
    Replacement::draft(0x1417baab0, "ak_1417baab0", fn_1417baab0 as *const ()), // sub_1417baab0
    Replacement::draft(0x1417baba0, "ak_1417baba0", fn_1417baba0 as *const ()), // sub_1417baba0
    Replacement::draft(0x1417bad90, "ak_1417bad90", fn_1417bad90 as *const ()), // sub_1417bad90
    Replacement::draft(0x1417bafc0, "ak_1417bafc0", fn_1417bafc0 as *const ()), // sub_1417bafc0
    Replacement::draft(0x1417bb020, "ak_1417bb020", fn_1417bb020 as *const ()), // sub_1417bb020
    Replacement::draft(0x1417bb8b0, "ak_1417bb8b0", fn_1417bb8b0 as *const ()), // sub_1417bb8b0
    Replacement::draft(0x1417bba10, "ak_1417bba10", fn_1417bba10 as *const ()), // sub_1417bba10
    Replacement::draft(0x1417bbe20, "ak_1417bbe20", fn_1417bbe20 as *const ()), // sub_1417bbe20
    Replacement::draft(0x1417bc100, "ak_1417bc100", fn_1417bc100 as *const ()), // sub_1417bc100
    Replacement::draft(0x1417bda50, "ak_1417bda50", fn_1417bda50 as *const ()), // sub_1417bda50
    Replacement::draft(0x1417bdae0, "ak_1417bdae0", fn_1417bdae0 as *const ()), // sub_1417bdae0
    Replacement::draft(0x1417bdd90, "ak_1417bdd90", fn_1417bdd90 as *const ()), // sub_1417bdd90
    Replacement::draft(0x1417bde10, "ak_1417bde10", fn_1417bde10 as *const ()), // sub_1417bde10
    Replacement::draft(0x1417bdf80, "ak_1417bdf80", fn_1417bdf80 as *const ()), // sub_1417bdf80
    Replacement::draft(0x1417be0e0, "ak_1417be0e0", fn_1417be0e0 as *const ()), // sub_1417be0e0
    Replacement::draft(0x1417be590, "ak_1417be590", fn_1417be590 as *const ()), // sub_1417be590
    Replacement::draft(0x1417be790, "ak_1417be790", fn_1417be790 as *const ()), // sub_1417be790
    Replacement::draft(0x1417be990, "ak_1417be990", fn_1417be990 as *const ()), // sub_1417be990
    Replacement::draft(0x1417be9f0, "ak_1417be9f0", fn_1417be9f0 as *const ()), // sub_1417be9f0
    Replacement::draft(0x1417beb10, "ak_1417beb10", fn_1417beb10 as *const ()), // sub_1417beb10
    Replacement::draft(0x1417beb30, "ak_1417beb30", fn_1417beb30 as *const ()), // sub_1417beb30
    Replacement::draft(0x1417becd0, "ak_1417becd0", fn_1417becd0 as *const ()), // sub_1417becd0
    Replacement::draft(0x1417bf1f0, "ak_1417bf1f0", fn_1417bf1f0 as *const ()), // sub_1417bf1f0
    Replacement::draft(0x1417bf210, "ak_1417bf210", fn_1417bf210 as *const ()), // sub_1417bf210
    Replacement::draft(0x1417bf440, "ak_1417bf440", fn_1417bf440 as *const ()), // sub_1417bf440
    Replacement::draft(0x1417bfbc0, "ak_1417bfbc0", fn_1417bfbc0 as *const ()), // sub_1417bfbc0
    Replacement::draft(0x1417bfce0, "ak_1417bfce0", fn_1417bfce0 as *const ()), // sub_1417bfce0
    Replacement::draft(0x1417c0670, "ak_1417c0670", tpl_365::<0x1417c0690> as *const ()), // sub_1417c0670
    Replacement::draft(0x1417c0680, "ak_1417c0680", tpl_365::<0x1417c08a0> as *const ()), // sub_1417c0680
    Replacement::draft(0x1417c08a0, "ak_1417c08a0", fn_1417c08a0 as *const ()), // sub_1417c08a0
    Replacement::draft(0x1417c0c70, "ak_1417c0c70", fn_1417c0c70 as *const ()), // sub_1417c0c70
    Replacement::draft(0x1417c1060, "ak_1417c1060", fn_1417c1060 as *const ()), // sub_1417c1060
    Replacement::draft(0x1417c1150, "ak_1417c1150", fn_1417c1150 as *const ()), // sub_1417c1150
    Replacement::draft(0x1417c1260, "ak_1417c1260", fn_1417c1260 as *const ()), // sub_1417c1260
    Replacement::draft(0x1417c12a0, "ak_1417c12a0", fn_1417c12a0 as *const ()), // sub_1417c12a0
    Replacement::draft(0x1417c1980, "ak_1417c1980", fn_1417c1980 as *const ()), // sub_1417c1980
    Replacement::draft(0x1417c1ce0, "ak_1417c1ce0", fn_1417c1ce0 as *const ()), // sub_1417c1ce0
    Replacement::draft(0x1417c1fa0, "ak_1417c1fa0", fn_1417c1fa0 as *const ()), // sub_1417c1fa0
    Replacement::draft(0x1417c2040, "ak_1417c2040", tpl_27770::<0x1426c3d98> as *const ()), // sub_1417c2040
    Replacement::draft(0x1417c2080, "ak_1417c2080", fn_1417c2080 as *const ()), // sub_1417c2080
    Replacement::draft(0x1417c2170, "ak_1417c2170", fn_1417c2170 as *const ()), // sub_1417c2170
    Replacement::draft(0x1417c2270, "ak_1417c2270", fn_1417c2270 as *const ()), // sub_1417c2270
    Replacement::draft(0x1417c27f0, "ak_1417c27f0", fn_1417c27f0 as *const ()), // sub_1417c27f0
    Replacement::draft(0x1417c29f0, "ak_1417c29f0", fn_1417c29f0 as *const ()), // sub_1417c29f0
    Replacement::draft(0x1417c2a10, "ak_1417c2a10", fn_1417c2a10 as *const ()), // sub_1417c2a10
    Replacement::draft(0x1417c2a20, "ak_1417c2a20", fn_1417c2a20 as *const ()), // sub_1417c2a20
    Replacement::draft(0x1417c2aa0, "ak_1417c2aa0", fn_1417c2aa0 as *const ()), // sub_1417c2aa0
    Replacement::draft(0x1417c2ab0, "ak_1417c2ab0", fn_1417c2ab0 as *const ()), // sub_1417c2ab0
    Replacement::draft(0x1417c2af0, "ak_1417c2af0", fn_1417c2af0 as *const ()), // sub_1417c2af0
    Replacement::draft(0x1417c2b00, "ak_1417c2b00", tpl_26861::<0x1417c60b0> as *const ()), // sub_1417c2b00
    Replacement::draft(0x1417c2b30, "unknown_1417c2b30", fn_1417c2b30 as *const ()), // sub_1417c2b30
    Replacement::draft(0x1417c2b40, "unknown_1417c2b40", fn_1417c2b40 as *const ()), // sub_1417c2b40
    Replacement::draft(0x1417c2ba0, "ak_1417c2ba0", fn_1417c2ba0 as *const ()), // sub_1417c2ba0
    Replacement::draft(0x1417c2ee0, "ak_1417c2ee0", fn_1417c2ee0 as *const ()), // sub_1417c2ee0
    Replacement::draft(0x1417c31a0, "ak_1417c31a0", fn_1417c31a0 as *const ()), // sub_1417c31a0
    Replacement::draft(0x1417c31c0, "ak_1417c31c0", fn_1417c31c0 as *const ()), // sub_1417c31c0
    Replacement::draft(0x1417c31d0, "ak_1417c31d0", fn_1417c31d0 as *const ()), // sub_1417c31d0
    Replacement::draft(0x1417c3220, "ak_1417c3220", fn_1417c3220 as *const ()), // sub_1417c3220
    Replacement::draft(0x1417c3240, "ak_1417c3240", fn_1417c3240 as *const ()), // sub_1417c3240
    Replacement::draft(0x1417c34e0, "ak_1417c34e0", fn_1417c34e0 as *const ()), // sub_1417c34e0
    Replacement::draft(0x1417c3870, "ak_1417c3870", fn_1417c3870 as *const ()), // sub_1417c3870
    Replacement::draft(0x1417c3fd0, "ak_1417c3fd0", fn_1417c3fd0 as *const ()), // sub_1417c3fd0
    Replacement::draft(0x1417c3fe0, "ak_1417c3fe0", fn_1417c3fe0 as *const ()), // sub_1417c3fe0
    Replacement::draft(0x1417c4040, "ak_1417c4040", fn_1417c4040 as *const ()), // sub_1417c4040
    Replacement::draft(0x1417c4170, "ak_1417c4170", fn_1417c4170 as *const ()), // sub_1417c4170
    Replacement::draft(0x1417c42e0, "ak_1417c42e0", fn_1417c42e0 as *const ()), // sub_1417c42e0
    Replacement::draft(0x1417c42f0, "ak_1417c42f0", fn_1417c42f0 as *const ()), // sub_1417c42f0
    Replacement::draft(0x1417c43b0, "ak_1417c43b0", fn_1417c43b0 as *const ()), // sub_1417c43b0
    Replacement::draft(0x1417c43d0, "ak_1417c43d0", fn_1417c43d0 as *const ()), // sub_1417c43d0
    Replacement::draft(0x1417c46d0, "ak_1417c46d0", fn_1417c46d0 as *const ()), // sub_1417c46d0
    Replacement::draft(0x1417c4ad0, "ak_1417c4ad0", fn_1417c4ad0 as *const ()), // sub_1417c4ad0
    Replacement::draft(0x1417c4ae0, "ak_1417c4ae0", fn_1417c4ae0 as *const ()), // sub_1417c4ae0
    Replacement::draft(0x1417c4b90, "ak_1417c4b90", fn_1417c4b90 as *const ()), // sub_1417c4b90
    Replacement::draft(0x1417c4c70, "ak_1417c4c70", fn_1417c4c70 as *const ()), // sub_1417c4c70
    Replacement::draft(0x1417c4ee0, "ak_1417c4ee0", fn_1417c4ee0 as *const ()), // sub_1417c4ee0
    Replacement::draft(0x1417c4ef0, "ak_1417c4ef0", fn_1417c4ef0 as *const ()), // sub_1417c4ef0
    Replacement::draft(0x1417c4f50, "ak_1417c4f50", fn_1417c4f50 as *const ()), // sub_1417c4f50
    Replacement::draft(0x1417c4f70, "ak_1417c4f70", fn_1417c4f70 as *const ()), // sub_1417c4f70
    Replacement::draft(0x1417c5570, "ak_1417c5570", fn_1417c5570 as *const ()), // sub_1417c5570
    Replacement::draft(0x1417c5a30, "ak_1417c5a30", fn_1417c5a30 as *const ()), // sub_1417c5a30
    Replacement::draft(0x1417c5a40, "ak_1417c5a40", fn_1417c5a40 as *const ()), // sub_1417c5a40
    Replacement::draft(0x1417c5a90, "ak_1417c5a90", fn_1417c5a90 as *const ()), // sub_1417c5a90
    Replacement::draft(0x1417c5cf0, "ak_1417c5cf0", fn_1417c5cf0 as *const ()), // sub_1417c5cf0
    Replacement::draft(0x1417c5f30, "ak_1417c5f30", fn_1417c5f30 as *const ()), // sub_1417c5f30
    Replacement::draft(0x1417c60b0, "ak_1417c60b0", fn_1417c60b0 as *const ()), // sub_1417c60b0
    Replacement::draft(0x1417c60d0, "ak_1417c60d0", fn_1417c60d0 as *const ()), // sub_1417c60d0
    Replacement::draft(0x1417c60f0, "ak_1417c60f0", fn_1417c60f0 as *const ()), // sub_1417c60f0
    Replacement::draft(0x1417c6150, "ak_1417c6150", fn_1417c6150 as *const ()), // sub_1417c6150
    Replacement::draft(0x1417c6190, "ak_1417c6190", fn_1417c6190 as *const ()), // sub_1417c6190
    Replacement::draft(0x1417c61b0, "ak_1417c61b0", fn_1417c61b0 as *const ()), // sub_1417c61b0
    Replacement::draft(0x1417c6330, "ak_1417c6330", fn_1417c6330 as *const ()), // sub_1417c6330
    Replacement::draft(0x1417c6360, "ak_1417c6360", fn_1417c6360 as *const ()), // sub_1417c6360
    Replacement::draft(0x1417c63f0, "ak_1417c63f0", fn_1417c63f0 as *const ()), // sub_1417c63f0
    Replacement::draft(0x1417c6540, "ak_1417c6540", fn_1417c6540 as *const ()), // sub_1417c6540
    Replacement::draft(0x1417c65d0, "ak_1417c65d0", fn_1417c65d0 as *const ()), // sub_1417c65d0
    Replacement::draft(0x1417c6660, "ak_1417c6660", tpl_33583::<0x1426c4028, 0x1426c4040, 0x1426755d0, 0x14183adbe, 0x143426a48, 0x1426af858, 0x143426b90, 0x14183ad9a, 0x1426c4030, 0x143428730> as *const ()), // sub_1417c6660
    Replacement::draft(0x1417c6870, "ak_1417c6870", tpl_33583::<0x1426c4028, 0x1426c4040, 0x1426755d0, 0x14183adbe, 0x143426a48, 0x1426af858, 0x143426b90, 0x14183ad9a, 0x1426c4038, 0x143428730> as *const ()), // sub_1417c6870
    Replacement::draft(0x1417c6a80, "ak_1417c6a80", fn_1417c6a80 as *const ()), // sub_1417c6a80
    Replacement::draft(0x1417c6b00, "ak_1417c6b00", fn_1417c6b00 as *const ()), // sub_1417c6b00
    Replacement::draft(0x1417c6b90, "ak_1417c6b90", fn_1417c6b90 as *const ()), // sub_1417c6b90
    Replacement::draft(0x1417c6c60, "ak_1417c6c60", fn_1417c6c60 as *const ()), // sub_1417c6c60
    Replacement::draft(0x1417c6d30, "ak_1417c6d30", fn_1417c6d30 as *const ()), // sub_1417c6d30
    Replacement::draft(0x1417c70e0, "ak_1417c70e0", fn_1417c70e0 as *const ()), // sub_1417c70e0
    Replacement::draft(0x1417c74a0, "ak_1417c74a0", fn_1417c74a0 as *const ()), // sub_1417c74a0
    Replacement::draft(0x1417c74c0, "ak_1417c74c0", fn_1417c74c0 as *const ()), // sub_1417c74c0
    Replacement::draft(0x1417c74e0, "ak_1417c74e0", tpl_365::<0x1417c6360> as *const ()), // sub_1417c74e0
    Replacement::draft(0x1417c74f0, "ak_1417c74f0", fn_1417c74f0 as *const ()), // sub_1417c74f0
    Replacement::draft(0x1417d1790, "ak_1417d1790", fn_1417d1790 as *const ()), // sub_1417d1790
    Replacement::draft(0x1417d1d30, "ak_1417d1d30", fn_1417d1d30 as *const ()), // sub_1417d1d30
    Replacement::draft(0x1417d1d60, "ak_1417d1d60", fn_1417d1d60 as *const ()), // sub_1417d1d60
    Replacement::draft(0x1417d1db0, "ak_1417d1db0", fn_1417d1db0 as *const ()), // sub_1417d1db0
    Replacement::draft(0x1417d1df0, "ak_1417d1df0", fn_1417d1df0 as *const ()), // sub_1417d1df0
    Replacement::draft(0x1417d1fd0, "ak_1417d1fd0", fn_1417d1fd0 as *const ()), // sub_1417d1fd0
    Replacement::draft(0x1417d2320, "ak_1417d2320", fn_1417d2320 as *const ()), // sub_1417d2320
    Replacement::draft(0x1417d2450, "ak_1417d2450", fn_1417d2450 as *const ()), // sub_1417d2450
    Replacement::draft(0x1417d2600, "ak_1417d2600", fn_1417d2600 as *const ()), // sub_1417d2600
    Replacement::draft(0x1417d2680, "ak_1417d2680", tpl_27770::<0x1426c6918> as *const ()), // sub_1417d2680
    Replacement::draft(0x1417d2690, "ak_1417d2690", fn_1417d2690 as *const ()), // sub_1417d2690
    Replacement::draft(0x1417d26f0, "ak_1417d26f0", fn_1417d26f0 as *const ()), // sub_1417d26f0
    Replacement::draft(0x1417d2720, "ak_1417d2720", fn_1417d2720 as *const ()), // sub_1417d2720
    Replacement::draft(0x1417d2840, "ak_1417d2840", fn_1417d2840 as *const ()), // sub_1417d2840
    Replacement::draft(0x1417d28e0, "ak_1417d28e0", fn_1417d28e0 as *const ()), // sub_1417d28e0
    Replacement::draft(0x1417d28f0, "ak_1417d28f0", fn_1417d28f0 as *const ()), // sub_1417d28f0
    Replacement::draft(0x1417d2910, "ak_1417d2910", fn_1417d2910 as *const ()), // sub_1417d2910
    Replacement::draft(0x1417d29d0, "ak_1417d29d0", fn_1417d29d0 as *const ()), // sub_1417d29d0
    Replacement::draft(0x1417d2f90, "ak_1417d2f90", fn_1417d2f90 as *const ()), // sub_1417d2f90
    Replacement::draft(0x1417d2fb0, "ak_1417d2fb0", fn_1417d2fb0 as *const ()), // sub_1417d2fb0
    Replacement::draft(0x1417d33b0, "ak_1417d33b0", fn_1417d33b0 as *const ()), // sub_1417d33b0
    Replacement::draft(0x1417d3420, "ak_1417d3420", fn_1417d3420 as *const ()), // sub_1417d3420
    Replacement::draft(0x1417d3430, "ak_1417d3430", fn_1417d3430 as *const ()), // sub_1417d3430
    Replacement::draft(0x1417d34f0, "ak_1417d34f0", fn_1417d34f0 as *const ()), // sub_1417d34f0
    Replacement::draft(0x1417d3b90, "ak_1417d3b90", fn_1417d3b90 as *const ()), // sub_1417d3b90
    Replacement::draft(0x1417d3be0, "ak_1417d3be0", fn_1417d3be0 as *const ()), // sub_1417d3be0
    Replacement::draft(0x1417d3fa0, "ak_1417d3fa0", fn_1417d3fa0 as *const ()), // sub_1417d3fa0
    Replacement::draft(0x1417d3fb0, "ak_1417d3fb0", fn_1417d3fb0 as *const ()), // sub_1417d3fb0
    Replacement::draft(0x1417d4070, "ak_1417d4070", fn_1417d4070 as *const ()), // sub_1417d4070
    Replacement::draft(0x1417d4bc0, "ak_1417d4bc0", fn_1417d4bc0 as *const ()), // sub_1417d4bc0
    Replacement::draft(0x1417d4cc0, "ak_1417d4cc0", fn_1417d4cc0 as *const ()), // sub_1417d4cc0
    Replacement::draft(0x1417d50a0, "ak_1417d50a0", fn_1417d50a0 as *const ()), // sub_1417d50a0
    Replacement::draft(0x1417d50b0, "ak_1417d50b0", fn_1417d50b0 as *const ()), // sub_1417d50b0
    Replacement::draft(0x1417d5170, "ak_1417d5170", fn_1417d5170 as *const ()), // sub_1417d5170
    Replacement::draft(0x1417d5ec0, "ak_1417d5ec0", fn_1417d5ec0 as *const ()), // sub_1417d5ec0
    Replacement::draft(0x1417d6020, "ak_1417d6020", fn_1417d6020 as *const ()), // sub_1417d6020
    Replacement::draft(0x1417d60a0, "ak_1417d60a0", fn_1417d60a0 as *const ()), // sub_1417d60a0
    Replacement::draft(0x1417d6440, "ak_1417d6440", fn_1417d6440 as *const ()), // sub_1417d6440
    Replacement::draft(0x1417d6450, "ak_1417d6450", fn_1417d6450 as *const ()), // sub_1417d6450
    Replacement::draft(0x1417d6510, "unknown_1417d6510", fn_1417d6510 as *const ()), // sub_1417d6510
    Replacement::draft(0x1417d6f20, "unknown_1417d6f20", fn_1417d6f20 as *const ()), // sub_1417d6f20
    Replacement::draft(0x1417d6f40, "ak_1417d6f40", fn_1417d6f40 as *const ()), // sub_1417d6f40
    Replacement::draft(0x1417d6ff0, "unknown_1417d6ff0", fn_1417d6ff0 as *const ()), // sub_1417d6ff0
    Replacement::draft(0x1417d7070, "unknown_1417d7070", fn_1417d7070 as *const ()), // sub_1417d7070
    Replacement::draft(0x1417d70f0, "unknown_1417d70f0", tpl_27062::<0x1426c6c70, 0x141834d30> as *const ()), // sub_1417d70f0
    Replacement::draft(0x1417d7120, "unknown_1417d7120", fn_1417d7120 as *const ()), // sub_1417d7120
    Replacement::draft(0x1417d7220, "unknown_1417d7220", fn_1417d7220 as *const ()), // sub_1417d7220
    Replacement::draft(0x1417d7240, "unknown_1417d7240", tpl_33625::<0xd8, 0xe8, 0xf8, 0x1417d3be0> as *const ()), // sub_1417d7240
    Replacement::draft(0x1417d7280, "unknown_1417d7280", tpl_33625::<0xd8, 0xe8, 0xf8, 0x1417d60a0> as *const ()), // sub_1417d7280
    Replacement::draft(0x1417d72c0, "unknown_1417d72c0", tpl_33625::<0xd8, 0xe8, 0xf8, 0x1417d4cc0> as *const ()), // sub_1417d72c0
    Replacement::draft(0x1417d7300, "unknown_1417d7300", fn_1417d7300 as *const ()), // sub_1417d7300
    Replacement::draft(0x1417d7370, "ak_1417d7370", fn_1417d7370 as *const ()), // sub_1417d7370
    Replacement::draft(0x1417d7380, "unknown_1417d7380", fn_1417d7380 as *const ()), // sub_1417d7380
    Replacement::draft(0x1417d7540, "unknown_1417d7540", fn_1417d7540 as *const ()), // sub_1417d7540
    Replacement::draft(0x1417d7560, "unknown_1417d7560", fn_1417d7560 as *const ()), // sub_1417d7560
    Replacement::draft(0x1417d7570, "unknown_1417d7570", fn_1417d7570 as *const ()), // sub_1417d7570
    Replacement::draft(0x1417d7592, "crt_1417d7592", tpl_906::<0x1426463e8> as *const ()), // j_#151
    Replacement::draft(0x1417d75a0, "unknown_1417d75a0", fn_1417d75a0 as *const ()), // sub_1417d75a0
    Replacement::draft(0x1417d75c0, "crt_1417d75c0", tpl_906::<0x142645f00> as *const ()), // j_physx::shdfnd::Foundation::j_physx::shdfnd::Foundation::decRefCount
    Replacement::draft(0x1417d75d0, "physx_1417d75d0", fn_1417d75d0 as *const ()), // sub_1417d75d0
    Replacement::draft(0x1417d80b0, "physx_1417d80b0", fn_1417d80b0 as *const ()), // sub_1417d80b0
    Replacement::draft(0x1417d8630, "physx_1417d8630", tpl_93303::<0x1417f1e40, 0x1426c89b0, 0x1417f31f0, 0x1417f1c70> as *const ()), // sub_1417d8630
    Replacement::draft(0x1417d87c0, "physx_1417d87c0", tpl_93303::<0x1417f1d50, 0x1426c89b0, 0x1417f31f0, 0x1417f1c70> as *const ()), // sub_1417d87c0
    Replacement::draft(0x1417d8950, "physx_1417d8950", fn_1417d8950 as *const ()), // sub_1417d8950
    Replacement::draft(0x1417d8ad0, "physx_1417d8ad0", fn_1417d8ad0 as *const ()), // sub_1417d8ad0
    Replacement::draft(0x1417d8c50, "physx_1417d8c50", fn_1417d8c50 as *const ()), // sub_1417d8c50
    Replacement::draft(0x1417d8dd0, "physx_1417d8dd0", fn_1417d8dd0 as *const ()), // sub_1417d8dd0
    Replacement::draft(0x1417d9000, "physx_1417d9000", fn_1417d9000 as *const ()), // sub_1417d9000
    Replacement::draft(0x1417d9190, "physx_1417d9190", fn_1417d9190 as *const ()), // sub_1417d9190
    Replacement::draft(0x1417d9340, "physx_1417d9340", fn_1417d9340 as *const ()), // sub_1417d9340
    Replacement::draft(0x1417d94e0, "physx_1417d94e0", fn_1417d94e0 as *const ()), // sub_1417d94e0
    Replacement::draft(0x1417d95e0, "physx_1417d95e0", fn_1417d95e0 as *const ()), // sub_1417d95e0
    Replacement::draft(0x1417d9870, "physx_1417d9870", fn_1417d9870 as *const ()), // sub_1417d9870
    Replacement::draft(0x1417d9a20, "physx_1417d9a20", fn_1417d9a20 as *const ()), // sub_1417d9a20
    Replacement::draft(0x1417d9b90, "physx_1417d9b90", fn_1417d9b90 as *const ()), // sub_1417d9b90
    Replacement::draft(0x1417d9c70, "physx_1417d9c70", fn_1417d9c70 as *const ()), // sub_1417d9c70
    Replacement::draft(0x1417d9d80, "physx_1417d9d80", fn_1417d9d80 as *const ()), // sub_1417d9d80
    Replacement::draft(0x1417d9e30, "physx_1417d9e30", fn_1417d9e30 as *const ()), // sub_1417d9e30
    Replacement::draft(0x1417d9f10, "physx_1417d9f10", fn_1417d9f10 as *const ()), // sub_1417d9f10
    Replacement::draft(0x1417d9fe0, "physx_1417d9fe0", fn_1417d9fe0 as *const ()), // sub_1417d9fe0
    Replacement::draft(0x1417da120, "physx_1417da120", fn_1417da120 as *const ()), // sub_1417da120
    Replacement::draft(0x1417da320, "physx_1417da320", fn_1417da320 as *const ()), // sub_1417da320
    Replacement::draft(0x1417da5c0, "physx_1417da5c0", fn_1417da5c0 as *const ()), // sub_1417da5c0
    Replacement::draft(0x1417da770, "physx_1417da770", tpl_93324::<0x1417f1d50, 0x1426c89b0, 0x1417f31f0> as *const ()), // sub_1417da770
    Replacement::draft(0x1417daa40, "physx_1417daa40", tpl_93324::<0x1417f1e40, 0x1426c89b0, 0x1417f31f0> as *const ()), // sub_1417daa40
    Replacement::draft(0x1417dad10, "physx_1417dad10", fn_1417dad10 as *const ()), // sub_1417dad10
    Replacement::draft(0x1417dae70, "physx_1417dae70", fn_1417dae70 as *const ()), // sub_1417dae70
    Replacement::draft(0x1417db030, "physx_1417db030", fn_1417db030 as *const ()), // sub_1417db030
    Replacement::draft(0x1417db260, "physx_1417db260", fn_1417db260 as *const ()), // sub_1417db260
    Replacement::draft(0x1417db430, "physx_1417db430", fn_1417db430 as *const ()), // sub_1417db430
    Replacement::draft(0x1417db670, "physx_1417db670", fn_1417db670 as *const ()), // sub_1417db670
    Replacement::draft(0x1417dbfd0, "physx_1417dbfd0", fn_1417dbfd0 as *const ()), // sub_1417dbfd0
    Replacement::draft(0x1417dc170, "physx_1417dc170", fn_1417dc170 as *const ()), // sub_1417dc170
    Replacement::draft(0x1417dc3a0, "physx_1417dc3a0", fn_1417dc3a0 as *const ()), // sub_1417dc3a0
    Replacement::draft(0x1417dc5d0, "physx_1417dc5d0", fn_1417dc5d0 as *const ()), // sub_1417dc5d0
    Replacement::draft(0x1417dc800, "physx_1417dc800", fn_1417dc800 as *const ()), // sub_1417dc800
    Replacement::draft(0x1417dca40, "physx_1417dca40", fn_1417dca40 as *const ()), // sub_1417dca40
    Replacement::draft(0x1417dccf0, "physx_1417dccf0", fn_1417dccf0 as *const ()), // sub_1417dccf0
    Replacement::draft(0x1417dcf50, "physx_1417dcf50", fn_1417dcf50 as *const ()), // sub_1417dcf50
    Replacement::draft(0x1417dcfd0, "physx_1417dcfd0", fn_1417dcfd0 as *const ()), // sub_1417dcfd0
    Replacement::draft(0x1417dd0a0, "physx_1417dd0a0", fn_1417dd0a0 as *const ()), // sub_1417dd0a0
    Replacement::draft(0x1417dd1b0, "physx_1417dd1b0", fn_1417dd1b0 as *const ()), // sub_1417dd1b0
    Replacement::draft(0x1417dd260, "physx_1417dd260", fn_1417dd260 as *const ()), // sub_1417dd260
    Replacement::draft(0x1417dd2e0, "physx_1417dd2e0", fn_1417dd2e0 as *const ()), // sub_1417dd2e0
    Replacement::draft(0x1417dd3c0, "physx_1417dd3c0", fn_1417dd3c0 as *const ()), // sub_1417dd3c0
    Replacement::draft(0x1417dd700, "physx_1417dd700", fn_1417dd700 as *const ()), // sub_1417dd700
    Replacement::draft(0x1417dd790, "physx_1417dd790", fn_1417dd790 as *const ()), // sub_1417dd790
    Replacement::draft(0x1417dda10, "physx_1417dda10", fn_1417dda10 as *const ()), // sub_1417dda10
    Replacement::draft(0x1417ddbf0, "physx_1417ddbf0", fn_1417ddbf0 as *const ()), // sub_1417ddbf0
    Replacement::draft(0x1417dddc0, "physx_1417dddc0", fn_1417dddc0 as *const ()), // sub_1417dddc0
    Replacement::draft(0x1417de010, "physx_1417de010", fn_1417de010 as *const ()), // sub_1417de010
    Replacement::draft(0x1417de610, "physx_1417de610", fn_1417de610 as *const ()), // sub_1417de610
    Replacement::draft(0x1417def90, "physx_1417def90", tpl_33661::<0x1417f1e40, 0x1426c89b0, 0x1417f1c70, 0x143822050, 0x1417f32f0, 0x1426464c8, 0x1438220a0> as *const ()), // sub_1417def90
    Replacement::draft(0x1417df2b0, "physx_1417df2b0", fn_1417df2b0 as *const ()), // sub_1417df2b0
    Replacement::draft(0x1417df760, "physx_1417df760", fn_1417df760 as *const ()), // sub_1417df760
    Replacement::draft(0x1417e05c0, "physx_1417e05c0", fn_1417e05c0 as *const ()), // sub_1417e05c0
    Replacement::draft(0x1417e09c0, "physx_1417e09c0", fn_1417e09c0 as *const ()), // sub_1417e09c0
    Replacement::draft(0x1417e0f30, "physx_1417e0f30", tpl_93354::<0x1417f1e40, 0x1426c89b0, 0x1417dd2e0, 0x1417f1c70, 0x143822350, 0x1417da5c0, 0x1417d8630, 0x143822210, 0x1417f32f0, 0x1417f31f0> as *const ()), // sub_1417e0f30
    Replacement::draft(0x1417e13e0, "physx_1417e13e0", fn_1417e13e0 as *const ()), // sub_1417e13e0
    Replacement::draft(0x1417e1560, "physx_1417e1560", fn_1417e1560 as *const ()), // sub_1417e1560
    Replacement::draft(0x1417e18f0, "physx_1417e18f0", tpl_93354::<0x1417f1d50, 0x1426c89b0, 0x1417dd2e0, 0x1417f1c70, 0x143822350, 0x1417da770, 0x1417d87c0, 0x143822210, 0x1417f32f0, 0x1417f31f0> as *const ()), // sub_1417e18f0
    Replacement::draft(0x1417e1da0, "physx_1417e1da0", fn_1417e1da0 as *const ()), // sub_1417e1da0
    Replacement::draft(0x1417e20f0, "physx_1417e20f0", fn_1417e20f0 as *const ()), // sub_1417e20f0
    Replacement::draft(0x1417e23c0, "physx_1417e23c0", fn_1417e23c0 as *const ()), // sub_1417e23c0
    Replacement::draft(0x1417e2740, "physx_1417e2740", fn_1417e2740 as *const ()), // sub_1417e2740
    Replacement::draft(0x1417e2950, "physx_1417e2950", tpl_33661::<0x1417f1d50, 0x1426c89b0, 0x1417f1c70, 0x143822050, 0x1417f32f0, 0x1426464c8, 0x1438220a0> as *const ()), // sub_1417e2950
    Replacement::draft(0x1417e2c70, "physx_1417e2c70", fn_1417e2c70 as *const ()), // sub_1417e2c70
    Replacement::draft(0x1417e3430, "physx_1417e3430", fn_1417e3430 as *const ()), // sub_1417e3430
    Replacement::draft(0x1417e3620, "physx_1417e3620", tpl_93354::<0x1417f1e40, 0x1426c89b0, 0x1417dd2e0, 0x1417f1c70, 0x143822350, 0x1417daa40, 0x1417d8630, 0x143822210, 0x1417f32f0, 0x1417f31f0> as *const ()), // sub_1417e3620
    Replacement::draft(0x1417e3ad0, "physx_1417e3ad0", fn_1417e3ad0 as *const ()), // sub_1417e3ad0
    Replacement::draft(0x1417e3e00, "physx_1417e3e00", fn_1417e3e00 as *const ()), // sub_1417e3e00
    Replacement::draft(0x1417e42f0, "physx_1417e42f0", fn_1417e42f0 as *const ()), // sub_1417e42f0
    Replacement::draft(0x1417e46b0, "physx_1417e46b0", fn_1417e46b0 as *const ()), // sub_1417e46b0
    Replacement::draft(0x1417e4870, "physx_1417e4870", fn_1417e4870 as *const ()), // sub_1417e4870
    Replacement::draft(0x1417e4eb0, "physx_1417e4eb0", fn_1417e4eb0 as *const ()), // sub_1417e4eb0
    Replacement::draft(0x1417e5650, "physx_1417e5650", fn_1417e5650 as *const ()), // sub_1417e5650
    Replacement::draft(0x1417e58a0, "physx_1417e58a0", fn_1417e58a0 as *const ()), // sub_1417e58a0
    Replacement::draft(0x1417e5d70, "physx_1417e5d70", fn_1417e5d70 as *const ()), // sub_1417e5d70
    Replacement::draft(0x1417e6010, "physx_1417e6010", fn_1417e6010 as *const ()), // sub_1417e6010
    Replacement::draft(0x1417e6360, "physx_1417e6360", fn_1417e6360 as *const ()), // sub_1417e6360
    Replacement::draft(0x1417e6540, "physx_1417e6540", fn_1417e6540 as *const ()), // sub_1417e6540
    Replacement::draft(0x1417e67a0, "physx_1417e67a0", fn_1417e67a0 as *const ()), // sub_1417e67a0
    Replacement::draft(0x1417e68f0, "physx_1417e68f0", fn_1417e68f0 as *const ()), // sub_1417e68f0
    Replacement::draft(0x1417e6f90, "physx_1417e6f90", fn_1417e6f90 as *const ()), // sub_1417e6f90
    Replacement::draft(0x1417e7940, "physx_1417e7940", fn_1417e7940 as *const ()), // sub_1417e7940
    Replacement::draft(0x1417e7cd0, "physx_1417e7cd0", fn_1417e7cd0 as *const ()), // sub_1417e7cd0
    Replacement::draft(0x1417e8340, "physx_1417e8340", fn_1417e8340 as *const ()), // sub_1417e8340
    Replacement::draft(0x1417e9670, "physx_1417e9670", fn_1417e9670 as *const ()), // sub_1417e9670
    Replacement::draft(0x1417e9a10, "physx_1417e9a10", fn_1417e9a10 as *const ()), // sub_1417e9a10
    Replacement::draft(0x1417ea0f0, "physx_1417ea0f0", fn_1417ea0f0 as *const ()), // sub_1417ea0f0
    Replacement::draft(0x1417ea5c0, "physx_1417ea5c0", fn_1417ea5c0 as *const ()), // sub_1417ea5c0
    Replacement::draft(0x1417ea720, "physx_1417ea720", fn_1417ea720 as *const ()), // sub_1417ea720
    Replacement::draft(0x1417eaac0, "physx_1417eaac0", fn_1417eaac0 as *const ()), // sub_1417eaac0
    Replacement::draft(0x1417eac20, "physx_1417eac20", fn_1417eac20 as *const ()), // sub_1417eac20
    Replacement::draft(0x1417eafb0, "physx_1417eafb0", fn_1417eafb0 as *const ()), // sub_1417eafb0
    Replacement::draft(0x1417eb110, "physx_1417eb110", fn_1417eb110 as *const ()), // sub_1417eb110
    Replacement::draft(0x1417eb400, "physx_1417eb400", fn_1417eb400 as *const ()), // sub_1417eb400
    Replacement::draft(0x1417eb800, "physx_1417eb800", fn_1417eb800 as *const ()), // sub_1417eb800
    Replacement::draft(0x1417eb9e0, "physx_1417eb9e0", fn_1417eb9e0 as *const ()), // sub_1417eb9e0
    Replacement::draft(0x1417ec140, "physx_1417ec140", fn_1417ec140 as *const ()), // sub_1417ec140
    Replacement::draft(0x1417ec330, "physx_1417ec330", fn_1417ec330 as *const ()), // sub_1417ec330
    Replacement::draft(0x1417ec670, "physx_1417ec670", fn_1417ec670 as *const ()), // sub_1417ec670
    Replacement::draft_throws(0x1417ecac0, "physx_1417ecac0", fn_1417ecac0 as *const (), Throws::Discard), // sub_1417ecac0
    Replacement::draft(0x1417ed170, "physx_1417ed170", fn_1417ed170 as *const ()), // sub_1417ed170
    Replacement::draft(0x1417ed650, "physx_1417ed650", fn_1417ed650 as *const ()), // sub_1417ed650
    Replacement::draft(0x1417ed880, "physx_1417ed880", fn_1417ed880 as *const ()), // sub_1417ed880
    Replacement::draft(0x1417edab0, "physx_1417edab0", tpl_33698::<0x14181beb0, 0x1417eb110, 0x1417eafb0> as *const ()), // sub_1417edab0
    Replacement::draft(0x1417edb80, "physx_1417edb80", tpl_33698::<0x14181bf80, 0x1417eb110, 0x1417eafb0> as *const ()), // sub_1417edb80
    Replacement::draft(0x1417edc50, "physx_1417edc50", fn_1417edc50 as *const ()), // sub_1417edc50
    Replacement::draft(0x1417edce0, "physx_1417edce0", fn_1417edce0 as *const ()), // sub_1417edce0
    Replacement::draft(0x1417eddb0, "physx_1417eddb0", fn_1417eddb0 as *const ()), // sub_1417eddb0
    Replacement::draft(0x1417ede30, "physx_1417ede30", fn_1417ede30 as *const ()), // sub_1417ede30
    Replacement::draft(0x1417edef0, "physx_1417edef0", fn_1417edef0 as *const ()), // sub_1417edef0
    Replacement::draft(0x1417ee000, "physx_1417ee000", fn_1417ee000 as *const ()), // sub_1417ee000
    Replacement::draft(0x1417ee100, "physx_1417ee100", fn_1417ee100 as *const ()), // sub_1417ee100
    Replacement::draft(0x1417ee1a0, "physx_1417ee1a0", fn_1417ee1a0 as *const ()), // sub_1417ee1a0
    Replacement::draft(0x1417ee230, "physx_1417ee230", fn_1417ee230 as *const ()), // sub_1417ee230
    Replacement::draft(0x1417ee4a0, "physx_1417ee4a0", fn_1417ee4a0 as *const ()), // sub_1417ee4a0
    Replacement::draft(0x1417ee690, "physx_1417ee690", fn_1417ee690 as *const ()), // sub_1417ee690
    Replacement::draft(0x1417ee810, "physx_1417ee810", fn_1417ee810 as *const ()), // sub_1417ee810
    Replacement::draft(0x1417ee860, "physx_1417ee860", tpl_27062::<0x1426c7e00, 0x141834d30> as *const ()), // sub_1417ee860
    Replacement::draft(0x1417ee890, "physx_1417ee890", tpl_893::<0x1426c7e00, 0x141834d30> as *const ()), // sub_1417ee890
    Replacement::draft(0x1417ee8c0, "physx_1417ee8c0", fn_1417ee8c0 as *const ()), // sub_1417ee8c0
    Replacement::draft(0x1417ee980, "physx_1417ee980", fn_1417ee980 as *const ()), // sub_1417ee980
    Replacement::draft(0x1417eea40, "physx_1417eea40", tpl_93412::<0x142645ee8, 0x142645ed8, 0x28, 0x1426c86a0, 0x14397d1d0, 0x143882e08, 0x14183acaa, 0x8> as *const ()), // sub_1417eea40
    Replacement::draft(0x1417eeae0, "physx_1417eeae0", tpl_93412::<0x142645ee8, 0x142645ed8, 0x28, 0x1426c86a0, 0x14397d1d0, 0x143882e68, 0x14183acaa, 0x8> as *const ()), // sub_1417eeae0
    Replacement::draft(0x1417eeb80, "physx_1417eeb80", fn_1417eeb80 as *const ()), // sub_1417eeb80
    Replacement::draft(0x1417eed20, "physx_1417eed20", tpl_31205::<0x141806cf0> as *const ()), // sub_1417eed20
    Replacement::draft(0x1417eed30, "physx_1417eed30", tpl_31205::<0x1418040a0> as *const ()), // sub_1417eed30
    Replacement::draft(0x1417eed40, "physx_1417eed40", tpl_31205::<0x141809e00> as *const ()), // sub_1417eed40
    Replacement::draft(0x1417eed50, "physx_1417eed50", tpl_31205::<0x14180b190> as *const ()), // sub_1417eed50
    Replacement::draft(0x1417eed60, "physx_1417eed60", tpl_31205::<0x14180c790> as *const ()), // sub_1417eed60
    Replacement::draft(0x1417eed70, "physx_1417eed70", tpl_31205::<0x14180e000> as *const ()), // sub_1417eed70
    Replacement::draft(0x1417eed80, "physx_1417eed80", fn_1417eed80 as *const ()), // sub_1417eed80
    Replacement::draft(0x1417eed90, "physx_1417eed90", fn_1417eed90 as *const ()), // sub_1417eed90
    Replacement::draft(0x1417eeda0, "physx_1417eeda0", tpl_33715::<0x198> as *const ()), // sub_1417eeda0
    Replacement::draft(0x1417eedb0, "physx_1417eedb0", tpl_33715::<0x160> as *const ()), // sub_1417eedb0
    Replacement::draft(0x1417eedc0, "physx_1417eedc0", tpl_33715::<0x110> as *const ()), // sub_1417eedc0
    Replacement::draft(0x1417eedd0, "physx_1417eedd0", tpl_33715::<0x148> as *const ()), // sub_1417eedd0
    Replacement::draft(0x1417eede0, "physx_1417eede0", tpl_33715::<0x178> as *const ()), // sub_1417eede0
    Replacement::draft(0x1417eedf0, "physx_1417eedf0", tpl_33715::<0x128> as *const ()), // sub_1417eedf0
    Replacement::draft(0x1417eee00, "physx_1417eee00", tpl_33716::<0x30, 0x28, 0x1426c7d28, 0x8, 0x10, 0x14266801b> as *const ()), // sub_1417eee00
    Replacement::draft(0x1417eeeb0, "physx_1417eeeb0", tpl_33716::<0x30, 0x28, 0x1426c7c80, 0x8, 0x10, 0x14266801b> as *const ()), // sub_1417eeeb0
    Replacement::draft(0x1417eef60, "physx_1417eef60", tpl_33716::<0x30, 0x28, 0x1426c7cf0, 0x8, 0x10, 0x14266801b> as *const ()), // sub_1417eef60
    Replacement::draft(0x1417ef010, "physx_1417ef010", tpl_33716::<0x30, 0x28, 0x1426c7d00, 0x8, 0x10, 0x14266801b> as *const ()), // sub_1417ef010
    Replacement::draft(0x1417ef0c0, "physx_1417ef0c0", fn_1417ef0c0 as *const ()), // sub_1417ef0c0
    Replacement::draft(0x1417ef180, "physx_1417ef180", fn_1417ef180 as *const ()), // sub_1417ef180
    Replacement::draft(0x1417ef2f0, "physx_1417ef2f0", fn_1417ef2f0 as *const ()), // sub_1417ef2f0
    Replacement::draft(0x1417ef310, "physx_1417ef310", fn_1417ef310 as *const ()), // sub_1417ef310
    Replacement::draft(0x1417ef4d0, "physx_1417ef4d0", fn_1417ef4d0 as *const ()), // sub_1417ef4d0
    Replacement::draft(0x1417ef590, "physx_1417ef590", fn_1417ef590 as *const ()), // sub_1417ef590
    Replacement::draft(0x1417ef750, "physx_1417ef750", fn_1417ef750 as *const ()), // sub_1417ef750
    Replacement::draft(0x1417ef910, "physx_1417ef910", fn_1417ef910 as *const ()), // sub_1417ef910
    Replacement::draft(0x1417efa50, "physx_1417efa50", fn_1417efa50 as *const ()), // sub_1417efa50
    Replacement::draft(0x1417efbc0, "physx_1417efbc0", tpl_33726::<0x10, 0x1417dc800> as *const ()), // sub_1417efbc0
    Replacement::draft(0x1417efc10, "physx_1417efc10", fn_1417efc10 as *const ()), // sub_1417efc10
    Replacement::draft(0x1417efdd0, "physx_1417efdd0", fn_1417efdd0 as *const ()), // sub_1417efdd0
    Replacement::draft(0x1417eff90, "physx_1417eff90", fn_1417eff90 as *const ()), // sub_1417eff90
    Replacement::draft(0x1417f0150, "physx_1417f0150", tpl_33726::<0x10, 0x1417dca40> as *const ()), // sub_1417f0150
    Replacement::draft(0x1417f01a0, "physx_1417f01a0", fn_1417f01a0 as *const ()), // sub_1417f01a0
    Replacement::draft(0x1417f0360, "physx_1417f0360", fn_1417f0360 as *const ()), // sub_1417f0360
    Replacement::draft(0x1417f04d0, "physx_1417f04d0", fn_1417f04d0 as *const ()), // sub_1417f04d0
    Replacement::draft(0x1417f0690, "physx_1417f0690", fn_1417f0690 as *const ()), // sub_1417f0690
    Replacement::draft(0x1417f0710, "physx_1417f0710", fn_1417f0710 as *const ()), // sub_1417f0710
    Replacement::draft(0x1417f0720, "physx_1417f0720", fn_1417f0720 as *const ()), // sub_1417f0720
    Replacement::draft(0x1417f0730, "physx_1417f0730", fn_1417f0730 as *const ()), // sub_1417f0730
    Replacement::draft(0x1417f0790, "physx_1417f0790", tpl_1::<0x1426c7d38> as *const ()), // sub_1417f0790
    Replacement::draft(0x1417f07a0, "physx_1417f07a0", tpl_1::<0x1426c7d28> as *const ()), // sub_1417f07a0
    Replacement::draft(0x1417f07b0, "physx_1417f07b0", tpl_1::<0x1426c7ca0> as *const ()), // sub_1417f07b0
    Replacement::draft(0x1417f07c0, "physx_1417f07c0", tpl_1::<0x1426c7cb8> as *const ()), // sub_1417f07c0
    Replacement::draft(0x1417f07d0, "physx_1417f07d0", tpl_1::<0x1426c7d60> as *const ()), // sub_1417f07d0
    Replacement::draft(0x1417f07e0, "physx_1417f07e0", tpl_1::<0x1426c7d50> as *const ()), // sub_1417f07e0
    Replacement::draft(0x1417f07f0, "physx_1417f07f0", tpl_1::<0x1426c7c90> as *const ()), // sub_1417f07f0
    Replacement::draft(0x1417f0800, "physx_1417f0800", tpl_1::<0x1426c7d90> as *const ()), // sub_1417f0800
    Replacement::draft(0x1417f0810, "physx_1417f0810", tpl_1::<0x1426c7da0> as *const ()), // sub_1417f0810
    Replacement::draft(0x1417f0820, "physx_1417f0820", tpl_1::<0x1426c7db0> as *const ()), // sub_1417f0820
    Replacement::draft(0x1417f0830, "physx_1417f0830", tpl_1::<0x1426c7cd0> as *const ()), // sub_1417f0830
    Replacement::draft(0x1417f0840, "physx_1417f0840", tpl_1::<0x1426c7c80> as *const ()), // sub_1417f0840
    Replacement::draft(0x1417f0850, "physx_1417f0850", tpl_1::<0x1426c7d68> as *const ()), // sub_1417f0850
    Replacement::draft(0x1417f0860, "physx_1417f0860", tpl_1::<0x1426c7d78> as *const ()), // sub_1417f0860
    Replacement::draft(0x1417f0870, "physx_1417f0870", tpl_1::<0x1426c7dc0> as *const ()), // sub_1417f0870
    Replacement::draft(0x1417f0880, "physx_1417f0880", tpl_1::<0x1426c7dd8> as *const ()), // sub_1417f0880
    Replacement::draft(0x1417f0890, "physx_1417f0890", tpl_1::<0x1426c7cf0> as *const ()), // sub_1417f0890
    Replacement::draft(0x1417f08a0, "physx_1417f08a0", tpl_1::<0x1426c7d00> as *const ()), // sub_1417f08a0
    Replacement::draft(0x1417f08b0, "physx_1417f08b0", tpl_1::<0x1426c7d48> as *const ()), // sub_1417f08b0
    Replacement::draft(0x1417f08c0, "physx_1417f08c0", tpl_1::<0x1426c7de8> as *const ()), // sub_1417f08c0
    Replacement::draft(0x1417f08d0, "physx_1417f08d0", fn_1417f08d0 as *const ()), // sub_1417f08d0
    Replacement::draft(0x1417f0990, "physx_1417f0990", fn_1417f0990 as *const ()), // sub_1417f0990
    Replacement::draft(0x1417f0a20, "physx_1417f0a20", fn_1417f0a20 as *const ()), // sub_1417f0a20
    Replacement::draft(0x1417f0b20, "physx_1417f0b20", fn_1417f0b20 as *const ()), // sub_1417f0b20
    Replacement::draft(0x1417f0c20, "physx_1417f0c20", fn_1417f0c20 as *const ()), // sub_1417f0c20
    Replacement::draft(0x1417f0d20, "physx_1417f0d20", fn_1417f0d20 as *const ()), // sub_1417f0d20
    Replacement::draft(0x1417f0e10, "physx_1417f0e10", fn_1417f0e10 as *const ()), // sub_1417f0e10
    Replacement::draft(0x1417f0f20, "physx_1417f0f20", fn_1417f0f20 as *const ()), // sub_1417f0f20
    Replacement::draft(0x1417f12d0, "physx_1417f12d0", fn_1417f12d0 as *const ()), // sub_1417f12d0
    Replacement::draft(0x1417f1820, "physx_1417f1820", fn_1417f1820 as *const ()), // sub_1417f1820
    Replacement::draft(0x1417f1830, "physx_1417f1830", fn_1417f1830 as *const ()), // sub_1417f1830
    Replacement::draft_throws(0x1417f18f0, "physx_1417f18f0", fn_1417f18f0 as *const (), Throws::Discard), // sub_1417f18f0
    Replacement::draft(0x1417f1a40, "physx_1417f1a40", fn_1417f1a40 as *const ()), // sub_1417f1a40
    Replacement::draft(0x1417f1b00, "physx_1417f1b00", fn_1417f1b00 as *const ()), // sub_1417f1b00
    Replacement::draft(0x1417f1bc0, "physx_1417f1bc0", fn_1417f1bc0 as *const ()), // sub_1417f1bc0
    Replacement::draft(0x1417f1c70, "physx_1417f1c70", fn_1417f1c70 as *const ()), // sub_1417f1c70
    Replacement::draft(0x1417f1cf0, "physx_1417f1cf0", fn_1417f1cf0 as *const ()), // sub_1417f1cf0
    Replacement::draft(0x1417f1d30, "physx_1417f1d30", fn_1417f1d30 as *const ()), // sub_1417f1d30
    Replacement::draft(0x1417f1d50, "physx_1417f1d50", fn_1417f1d50 as *const ()), // sub_1417f1d50
    Replacement::draft(0x1417f1e40, "physx_1417f1e40", fn_1417f1e40 as *const ()), // sub_1417f1e40
    Replacement::draft(0x1417f1ec0, "physx_1417f1ec0", fn_1417f1ec0 as *const ()), // sub_1417f1ec0
    Replacement::draft(0x1417f1f70, "physx_1417f1f70", fn_1417f1f70 as *const ()), // sub_1417f1f70
    Replacement::draft(0x1417f2350, "physx_1417f2350", fn_1417f2350 as *const ()), // sub_1417f2350
    Replacement::draft(0x1417f2470, "physx_1417f2470", fn_1417f2470 as *const ()), // sub_1417f2470
    Replacement::draft(0x1417f2590, "physx_1417f2590", fn_1417f2590 as *const ()), // sub_1417f2590
    Replacement::draft(0x1417f2690, "physx_1417f2690", fn_1417f2690 as *const ()), // sub_1417f2690
    Replacement::draft(0x1417f2790, "physx_1417f2790", tpl_33759::<0x1417ee8c0, 0x30, 0x38, 0x8, 0x3c, 0x28, 0x142645ee8, 0x10> as *const ()), // sub_1417f2790
    Replacement::draft(0x1417f2850, "physx_1417f2850", fn_1417f2850 as *const ()), // sub_1417f2850
    Replacement::draft(0x1417f2980, "physx_1417f2980", tpl_33761::<0x1426c8630, 0x190> as *const ()), // sub_1417f2980
    Replacement::draft(0x1417f29d0, "physx_1417f29d0", tpl_33761::<0x1426c8630, 0x158> as *const ()), // sub_1417f29d0
    Replacement::draft(0x1417f2a20, "physx_1417f2a20", tpl_33761::<0x1426c8630, 0x108> as *const ()), // sub_1417f2a20
    Replacement::draft(0x1417f2a70, "physx_1417f2a70", tpl_33761::<0x1426c8630, 0x140> as *const ()), // sub_1417f2a70
    Replacement::draft(0x1417f2ac0, "physx_1417f2ac0", tpl_33761::<0x1426c8630, 0x170> as *const ()), // sub_1417f2ac0
    Replacement::draft(0x1417f2b10, "physx_1417f2b10", tpl_33761::<0x1426c8630, 0x120> as *const ()), // sub_1417f2b10
    Replacement::draft(0x1417f2b60, "physx_1417f2b60", tpl_33715::<0x190> as *const ()), // sub_1417f2b60
    Replacement::draft(0x1417f2b70, "physx_1417f2b70", tpl_33715::<0x158> as *const ()), // sub_1417f2b70
    Replacement::draft(0x1417f2b80, "physx_1417f2b80", tpl_33715::<0x108> as *const ()), // sub_1417f2b80
    Replacement::draft(0x1417f2b90, "physx_1417f2b90", tpl_33715::<0x140> as *const ()), // sub_1417f2b90
    Replacement::draft(0x1417f2ba0, "physx_1417f2ba0", tpl_33715::<0x170> as *const ()), // sub_1417f2ba0
    Replacement::draft(0x1417f2bb0, "physx_1417f2bb0", tpl_33715::<0x120> as *const ()), // sub_1417f2bb0
    Replacement::draft(0x1417f2bc0, "physx_1417f2bc0", fn_1417f2bc0 as *const ()), // sub_1417f2bc0
    Replacement::draft(0x1417f2d90, "physx_1417f2d90", tpl_33763::<0x21c, 0x1417f2350, 0x210, 0x218, 0xfffffffffffffff8, 0x8> as *const ()), // sub_1417f2d90
    Replacement::draft(0x1417f2e70, "physx_1417f2e70", tpl_33763::<0x214, 0x1417f2470, 0x208, 0x210, 0xfffffffffffffff8, 0x8> as *const ()), // sub_1417f2e70
    Replacement::draft(0x1417f2f50, "physx_1417f2f50", fn_1417f2f50 as *const ()), // sub_1417f2f50
    Replacement::draft(0x1417f3030, "physx_1417f3030", tpl_33765::<0x1417f2690> as *const ()), // sub_1417f3030
    Replacement::draft(0x1417f3110, "physx_1417f3110", tpl_33765::<0x1417f2790> as *const ()), // sub_1417f3110
    Replacement::draft(0x1417f31f0, "physx_1417f31f0", fn_1417f31f0 as *const ()), // sub_1417f31f0
    Replacement::draft(0x1417f32f0, "physx_1417f32f0", fn_1417f32f0 as *const ()), // sub_1417f32f0
    Replacement::draft(0x1417f33d0, "physx_1417f33d0", fn_1417f33d0 as *const ()), // sub_1417f33d0
    Replacement::draft(0x1417f36e0, "physx_1417f36e0", fn_1417f36e0 as *const ()), // sub_1417f36e0
    Replacement::draft(0x1417f37e0, "physx_1417f37e0", fn_1417f37e0 as *const ()), // sub_1417f37e0
    Replacement::draft(0x1417f3890, "physx_1417f3890", fn_1417f3890 as *const ()), // sub_1417f3890
    Replacement::draft(0x1417f3a40, "physx_1417f3a40", fn_1417f3a40 as *const ()), // sub_1417f3a40
    Replacement::draft(0x1417f3e70, "physx_1417f3e70", fn_1417f3e70 as *const ()), // sub_1417f3e70
    Replacement::draft(0x1417f3fc0, "physx_1417f3fc0", tpl_961::<0x10> as *const ()), // sub_1417f3fc0
    Replacement::draft(0x1417f3fe0, "physx_1417f3fe0", fn_1417f3fe0 as *const ()), // sub_1417f3fe0
    Replacement::draft(0x1417f4080, "physx_1417f4080", fn_1417f4080 as *const ()), // sub_1417f4080
    Replacement::draft(0x1417f40e0, "physx_1417f40e0", fn_1417f40e0 as *const ()), // sub_1417f40e0
    Replacement::draft(0x1417f41b0, "physx_1417f41b0", fn_1417f41b0 as *const ()), // sub_1417f41b0
    Replacement::draft(0x1417f41f0, "physx_1417f41f0", fn_1417f41f0 as *const ()), // sub_1417f41f0
    Replacement::draft(0x1417f4210, "physx_1417f4210", fn_1417f4210 as *const ()), // sub_1417f4210
    Replacement::draft(0x1417f4280, "physx_1417f4280", tpl_4544::<0x3d> as *const ()), // sub_1417f4280
    Replacement::draft(0x1417f4290, "physx_1417f4290", tpl_30363::<0x3d> as *const ()), // sub_1417f4290
    Replacement::draft(0x1417f42a0, "physx_1417f42a0", fn_1417f42a0 as *const ()), // sub_1417f42a0
    Replacement::draft(0x1417f4450, "physx_1417f4450", fn_1417f4450 as *const ()), // sub_1417f4450
    Replacement::draft(0x1417f4590, "physx_1417f4590", fn_1417f4590 as *const ()), // sub_1417f4590
    Replacement::draft(0x1417f46a0, "physx_1417f46a0", fn_1417f46a0 as *const ()), // sub_1417f46a0
    Replacement::draft(0x1417f47c0, "physx_1417f47c0", fn_1417f47c0 as *const ()), // sub_1417f47c0
    Replacement::draft(0x1417f48c0, "physx_1417f48c0", fn_1417f48c0 as *const ()), // sub_1417f48c0
    Replacement::draft(0x1417f4910, "physx_1417f4910", fn_1417f4910 as *const ()), // sub_1417f4910
    Replacement::draft(0x1417f4960, "physx_1417f4960", fn_1417f4960 as *const ()), // sub_1417f4960
    Replacement::draft(0x1417f4a00, "physx_1417f4a00", fn_1417f4a00 as *const ()), // sub_1417f4a00
    Replacement::draft(0x1417f4a20, "physx_1417f4a20", fn_1417f4a20 as *const ()), // sub_1417f4a20
    Replacement::draft(0x1417f4a40, "physx_1417f4a40", fn_1417f4a40 as *const ()), // sub_1417f4a40
    Replacement::draft(0x1417f4b60, "physx_1417f4b60", fn_1417f4b60 as *const ()), // sub_1417f4b60
    Replacement::draft(0x1417f4c30, "physx_1417f4c30", fn_1417f4c30 as *const ()), // sub_1417f4c30
    Replacement::draft(0x1417f4d30, "unknown_1417f4d30", fn_1417f4d30 as *const ()), // sub_1417f4d30
    Replacement::draft(0x1417f4e40, "unknown_1417f4e40", fn_1417f4e40 as *const ()), // sub_1417f4e40
    Replacement::draft(0x1417f4ed0, "unknown_1417f4ed0", fn_1417f4ed0 as *const ()), // sub_1417f4ed0
    Replacement::draft(0x1417f4f80, "unknown_1417f4f80", fn_1417f4f80 as *const ()), // sub_1417f4f80
    Replacement::draft(0x1417f5000, "opentelemetry_1417f5000", tpl_27762::<0x1417f4f80, 0x141834d30> as *const ()), // sub_1417f5000
    Replacement::draft(0x1417f5040, "physx_1417f5040", fn_1417f5040 as *const ()), // sub_1417f5040
    Replacement::draft(0x1417f51a0, "physx_1417f51a0", fn_1417f51a0 as *const ()), // sub_1417f51a0
    Replacement::draft(0x1417f5250, "physx_1417f5250", fn_1417f5250 as *const ()), // sub_1417f5250
    Replacement::draft(0x1417f5340, "physx_1417f5340", fn_1417f5340 as *const ()), // sub_1417f5340
    Replacement::draft(0x1417f53e0, "physx_1417f53e0", fn_1417f53e0 as *const ()), // sub_1417f53e0
    Replacement::draft(0x1417f5530, "physx_1417f5530", tpl_33792::<0x1417f5f90> as *const ()), // sub_1417f5530
    Replacement::draft(0x1417f56d0, "physx_1417f56d0", fn_1417f56d0 as *const ()), // sub_1417f56d0
    Replacement::draft(0x1417f5830, "physx_1417f5830", fn_1417f5830 as *const ()), // sub_1417f5830
    Replacement::draft(0x1417f5900, "physx_1417f5900", tpl_93502::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883040, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417f5900
    Replacement::draft(0x1417f5a30, "physx_1417f5a30", fn_1417f5a30 as *const ()), // sub_1417f5a30
    Replacement::draft(0x1417f5b70, "physx_1417f5b70", fn_1417f5b70 as *const ()), // sub_1417f5b70
    Replacement::draft(0x1417f5d50, "physx_1417f5d50", fn_1417f5d50 as *const ()), // sub_1417f5d50
    Replacement::draft(0x1417f5db0, "physx_1417f5db0", tpl_961::<0x40> as *const ()), // sub_1417f5db0
    Replacement::draft(0x1417f5dd0, "physx_1417f5dd0", fn_1417f5dd0 as *const ()), // sub_1417f5dd0
    Replacement::draft(0x1417f5f90, "physx_1417f5f90", fn_1417f5f90 as *const ()), // sub_1417f5f90
    Replacement::draft(0x1417f6180, "physx_1417f6180", fn_1417f6180 as *const ()), // sub_1417f6180
    Replacement::draft(0x1417f6550, "physx_1417f6550", fn_1417f6550 as *const ()), // sub_1417f6550
    Replacement::draft(0x1417f6f30, "physx_1417f6f30", fn_1417f6f30 as *const ()), // sub_1417f6f30
    Replacement::draft(0x1417f70d0, "physx_1417f70d0", fn_1417f70d0 as *const ()), // sub_1417f70d0
    Replacement::draft(0x1417f71a0, "physx_1417f71a0", tpl_6019::<0x1426c8ec8, 0x141834d30> as *const ()), // sub_1417f71a0
    Replacement::draft(0x1417f71d0, "physx_1417f71d0", tpl_893::<0x1426c8ec8, 0x141834d30> as *const ()), // sub_1417f71d0
    Replacement::draft(0x1417f7200, "physx_1417f7200", fn_1417f7200 as *const ()), // sub_1417f7200
    Replacement::draft(0x1417f7270, "physx_1417f7270", fn_1417f7270 as *const ()), // sub_1417f7270
    Replacement::draft(0x1417f72f0, "physx_1417f72f0", fn_1417f72f0 as *const ()), // sub_1417f72f0
    Replacement::draft(0x1417f7370, "physx_1417f7370", tpl_913::<0xf8> as *const ()), // sub_1417f7370
    Replacement::draft(0x1417f7380, "physx_1417f7380", fn_1417f7380 as *const ()), // sub_1417f7380
    Replacement::draft(0x1417f7390, "physx_1417f7390", fn_1417f7390 as *const ()), // sub_1417f7390
    Replacement::draft(0x1417f74c0, "physx_1417f74c0", fn_1417f74c0 as *const ()), // sub_1417f74c0
    Replacement::draft(0x1417f75d0, "physx_1417f75d0", fn_1417f75d0 as *const ()), // sub_1417f75d0
    Replacement::draft(0x1417f76e0, "physx_1417f76e0", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883118, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417f76e0
    Replacement::draft(0x1417f7800, "physx_1417f7800", fn_1417f7800 as *const ()), // sub_1417f7800
    Replacement::draft(0x1417f7840, "physx_1417f7840", fn_1417f7840 as *const ()), // sub_1417f7840
    Replacement::draft(0x1417f7a80, "physx_1417f7a80", fn_1417f7a80 as *const ()), // sub_1417f7a80
    Replacement::draft(0x1417f7b00, "physx_1417f7b00", fn_1417f7b00 as *const ()), // sub_1417f7b00
    Replacement::draft(0x1417f7b80, "physx_1417f7b80", fn_1417f7b80 as *const ()), // sub_1417f7b80
    Replacement::draft(0x1417f7c00, "physx_1417f7c00", fn_1417f7c00 as *const ()), // sub_1417f7c00
    Replacement::draft(0x1417f7ce0, "physx_1417f7ce0", fn_1417f7ce0 as *const ()), // sub_1417f7ce0
    Replacement::draft(0x1417f7d00, "physx_1417f7d00", fn_1417f7d00 as *const ()), // sub_1417f7d00
    Replacement::draft(0x1417f7d10, "physx_1417f7d10", fn_1417f7d10 as *const ()), // sub_1417f7d10
    Replacement::draft(0x1417f7d80, "physx_1417f7d80", fn_1417f7d80 as *const ()), // sub_1417f7d80
    Replacement::draft(0x1417f7dc0, "physx_1417f7dc0", fn_1417f7dc0 as *const ()), // sub_1417f7dc0
    Replacement::draft(0x1417f85f0, "physx_1417f85f0", fn_1417f85f0 as *const ()), // sub_1417f85f0
    Replacement::draft(0x1417f8640, "physx_1417f8640", tpl_27062::<0x1426c90c8, 0x141834d30> as *const ()), // sub_1417f8640
    Replacement::draft(0x1417f8670, "physx_1417f8670", tpl_33792::<0x1417f8810> as *const ()), // sub_1417f8670
    Replacement::draft(0x1417f8810, "physx_1417f8810", fn_1417f8810 as *const ()), // sub_1417f8810
    Replacement::draft(0x1417f8a00, "physx_1417f8a00", fn_1417f8a00 as *const ()), // sub_1417f8a00
    Replacement::draft(0x1417f8ff0, "physx_1417f8ff0", fn_1417f8ff0 as *const ()), // sub_1417f8ff0
    Replacement::draft(0x1417f9130, "physx_1417f9130", fn_1417f9130 as *const ()), // sub_1417f9130
    Replacement::draft(0x1417f91f0, "physx_1417f91f0", fn_1417f91f0 as *const ()), // sub_1417f91f0
    Replacement::draft(0x1417f94b0, "physx_1417f94b0", fn_1417f94b0 as *const ()), // sub_1417f94b0
    Replacement::draft(0x1417f9530, "physx_1417f9530", fn_1417f9530 as *const ()), // sub_1417f9530
    Replacement::draft(0x1417f95b0, "physx_1417f95b0", tpl_790::<0x1417f9530, 0x141834d30> as *const ()), // sub_1417f95b0
    Replacement::draft(0x1417f95f0, "physx_1417f95f0", fn_1417f95f0 as *const ()), // sub_1417f95f0
    Replacement::draft(0x1417f9720, "physx_1417f9720", fn_1417f9720 as *const ()), // sub_1417f9720
    Replacement::draft(0x1417f9910, "physx_1417f9910", tpl_93540::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x1438831b8, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417f9910
    Replacement::draft(0x1417f9a40, "physx_1417f9a40", tpl_93502::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883170, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417f9a40
    Replacement::draft(0x1417f9b70, "physx_1417f9b70", fn_1417f9b70 as *const ()), // sub_1417f9b70
    Replacement::draft(0x1417f9cd0, "physx_1417f9cd0", fn_1417f9cd0 as *const ()), // sub_1417f9cd0
    Replacement::draft(0x1417f9d30, "physx_1417f9d30", fn_1417f9d30 as *const ()), // sub_1417f9d30
    Replacement::draft(0x1417f9d70, "physx_1417f9d70", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x1438831d0, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417f9d70
    Replacement::draft(0x1417f9e90, "physx_1417f9e90", tpl_33824::<0x1417f9d70> as *const ()), // sub_1417f9e90
    Replacement::draft(0x1417f9f70, "physx_1417f9f70", fn_1417f9f70 as *const ()), // sub_1417f9f70
    Replacement::draft(0x1417fa090, "physx_1417fa090", fn_1417fa090 as *const ()), // sub_1417fa090
    Replacement::draft(0x1417fa260, "physx_1417fa260", fn_1417fa260 as *const ()), // sub_1417fa260
    Replacement::draft(0x1417fa3a0, "physx_1417fa3a0", fn_1417fa3a0 as *const ()), // sub_1417fa3a0
    Replacement::draft(0x1417fa750, "physx_1417fa750", fn_1417fa750 as *const ()), // sub_1417fa750
    Replacement::draft(0x1417faa10, "physx_1417faa10", tpl_93502::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883200, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417faa10
    Replacement::draft(0x1417fab40, "physx_1417fab40", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883288, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417fab40
    Replacement::draft(0x1417fac60, "physx_1417fac60", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883228, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417fac60
    Replacement::draft(0x1417fad80, "physx_1417fad80", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883258, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417fad80
    Replacement::draft(0x1417faea0, "physx_1417faea0", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x1438832b0, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417faea0
    Replacement::draft(0x1417fafc0, "physx_1417fafc0", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883340, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417fafc0
    Replacement::draft(0x1417fb0e0, "physx_1417fb0e0", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x1438832e0, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417fb0e0
    Replacement::draft(0x1417fb200, "physx_1417fb200", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143882e68, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417fb200
    Replacement::draft(0x1417fb320, "physx_1417fb320", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143882e90, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417fb320
    Replacement::draft(0x1417fb440, "physx_1417fb440", tpl_93518::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883310, 0x14183acaa, 0x1426c8650> as *const ()), // sub_1417fb440
    Replacement::draft(0x1417fb560, "physx_1417fb560", tpl_33824::<0x1417fab40> as *const ()), // sub_1417fb560
    Replacement::draft(0x1417fb640, "physx_1417fb640", tpl_33824::<0x1417fac60> as *const ()), // sub_1417fb640
    Replacement::draft(0x1417fb720, "physx_1417fb720", tpl_33824::<0x1417fad80> as *const ()), // sub_1417fb720
    Replacement::draft(0x1417fb800, "physx_1417fb800", tpl_33824::<0x1417faea0> as *const ()), // sub_1417fb800
    Replacement::draft(0x1417fb8e0, "physx_1417fb8e0", tpl_33824::<0x1417fafc0> as *const ()), // sub_1417fb8e0
    Replacement::draft(0x1417fb9c0, "physx_1417fb9c0", tpl_33824::<0x1417fb0e0> as *const ()), // sub_1417fb9c0
    Replacement::draft(0x1417fbaa0, "physx_1417fbaa0", tpl_33824::<0x1417fb200> as *const ()), // sub_1417fbaa0
    Replacement::draft(0x1417fbb80, "physx_1417fbb80", tpl_33824::<0x1417fb320> as *const ()), // sub_1417fbb80
    Replacement::draft(0x1417fbc60, "physx_1417fbc60", tpl_33824::<0x1417fb440> as *const ()), // sub_1417fbc60
    Replacement::draft(0x1417fbd40, "physx_1417fbd40", fn_1417fbd40 as *const ()), // sub_1417fbd40
    Replacement::draft(0x1417fbd70, "physx_1417fbd70", fn_1417fbd70 as *const ()), // sub_1417fbd70
    Replacement::draft(0x1417fbdc0, "physx_1417fbdc0", fn_1417fbdc0 as *const ()), // sub_1417fbdc0
    Replacement::draft(0x1417fbeb0, "physx_1417fbeb0", tpl_29822::<0x1426c9800, 0x8, 0x10, 0x18> as *const ()), // sub_1417fbeb0
    Replacement::draft(0x1417fbee0, "physx_1417fbee0", fn_1417fbee0 as *const ()), // sub_1417fbee0
    Replacement::draft(0x1417fbf30, "physx_1417fbf30", tpl_1691::<0x8> as *const ()), // sub_1417fbf30
    Replacement::draft(0x1417fbf40, "physx_1417fbf40", fn_1417fbf40 as *const ()), // sub_1417fbf40
    Replacement::draft(0x1417fbf60, "physx_1417fbf60", tpl_1691::<0x18> as *const ()), // sub_1417fbf60
    Replacement::draft(0x1417fbf70, "physx_1417fbf70", fn_1417fbf70 as *const ()), // sub_1417fbf70
    Replacement::draft(0x1417fc060, "physx_1417fc060", tpl_33831::<0x1426c9828, 0x8, 0x1426468e8, 0x1433e4548> as *const ()), // sub_1417fc060
    Replacement::draft(0x1417fc0a0, "physx_1417fc0a0", fn_1417fc0a0 as *const ()), // sub_1417fc0a0
    Replacement::draft(0x1417fc0d0, "physx_1417fc0d0", tpl_497::<0x8> as *const ()), // sub_1417fc0d0
    Replacement::draft(0x1417fc0e0, "physx_1417fc0e0", fn_1417fc0e0 as *const ()), // sub_1417fc0e0
    Replacement::draft(0x1417fc1c0, "physx_1417fc1c0", tpl_33831::<0x1426c9840, 0x8, 0x1426468e8, 0x1433e4518> as *const ()), // sub_1417fc1c0
    Replacement::draft(0x1417fc200, "physx_1417fc200", fn_1417fc200 as *const ()), // sub_1417fc200
    Replacement::draft(0x1417fc220, "physx_1417fc220", fn_1417fc220 as *const ()), // sub_1417fc220
    Replacement::draft(0x1417fc240, "physx_1417fc240", fn_1417fc240 as *const ()), // sub_1417fc240
    Replacement::draft(0x1417fc260, "physx_1417fc260", tpl_1691::<0x10> as *const ()), // sub_1417fc260
    Replacement::draft(0x1417fc270, "physx_1417fc270", tpl_497::<0x8> as *const ()), // sub_1417fc270
    Replacement::draft(0x1417fc280, "physx_1417fc280", fn_1417fc280 as *const ()), // sub_1417fc280
    Replacement::draft(0x1417fc2e0, "physx_1417fc2e0", fn_1417fc2e0 as *const ()), // sub_1417fc2e0
    Replacement::draft(0x1417fc340, "physx_1417fc340", tpl_6019::<0x1433e4518, 0x141834d30> as *const ()), // sub_1417fc340
    Replacement::draft(0x1417fc370, "physx_1417fc370", fn_1417fc370 as *const ()), // sub_1417fc370
    Replacement::draft(0x1417fc3d0, "physx_1417fc3d0", tpl_33840::<0x141803bf0> as *const ()), // sub_1417fc3d0
    Replacement::draft(0x1417fc400, "physx_1417fc400", tpl_33841::<0x141803bf0> as *const ()), // sub_1417fc400
    Replacement::draft(0x1417fc440, "physx_1417fc440", tpl_33840::<0x141802d70> as *const ()), // sub_1417fc440
    Replacement::draft(0x1417fc470, "physx_1417fc470", tpl_33841::<0x141802d70> as *const ()), // sub_1417fc470
    Replacement::draft(0x1417fc4b0, "physx_1417fc4b0", fn_1417fc4b0 as *const ()), // sub_1417fc4b0
    Replacement::draft(0x1417fc720, "physx_1417fc720", tpl_28507::<0x1417ffef0> as *const ()), // sub_1417fc720
    Replacement::draft(0x1417fc740, "physx_1417fc740", fn_1417fc740 as *const ()), // sub_1417fc740
    Replacement::draft(0x1417fc910, "physx_1417fc910", fn_1417fc910 as *const ()), // sub_1417fc910
    Replacement::draft(0x1417fcad0, "physx_1417fcad0", fn_1417fcad0 as *const ()), // sub_1417fcad0
    Replacement::draft(0x1417fcd80, "physx_1417fcd80", fn_1417fcd80 as *const ()), // sub_1417fcd80
    Replacement::draft(0x1417fcf70, "physx_1417fcf70", fn_1417fcf70 as *const ()), // sub_1417fcf70
    Replacement::draft(0x1417fd270, "physx_1417fd270", fn_1417fd270 as *const ()), // sub_1417fd270
    Replacement::draft(0x1417fd450, "physx_1417fd450", fn_1417fd450 as *const ()), // sub_1417fd450
    Replacement::draft(0x1417fd8e0, "physx_1417fd8e0", fn_1417fd8e0 as *const ()), // sub_1417fd8e0
    Replacement::draft(0x1417fe140, "physx_1417fe140", fn_1417fe140 as *const ()), // sub_1417fe140
    Replacement::draft(0x1417fe550, "physx_1417fe550", fn_1417fe550 as *const ()), // sub_1417fe550
    Replacement::draft(0x1417fea80, "physx_1417fea80", fn_1417fea80 as *const ()), // sub_1417fea80
    Replacement::draft(0x1417fed20, "physx_1417fed20", fn_1417fed20 as *const ()), // sub_1417fed20
    Replacement::draft(0x1417fed50, "physx_1417fed50", fn_1417fed50 as *const ()), // sub_1417fed50
    Replacement::draft(0x1417fedb0, "physx_1417fedb0", tpl_369 as *const ()), // sub_1417fedb0
    Replacement::draft(0x1417fedc0, "physx_1417fedc0", fn_1417fedc0 as *const ()), // sub_1417fedc0
    Replacement::draft(0x1417ff1f0, "physx_1417ff1f0", fn_1417ff1f0 as *const ()), // sub_1417ff1f0
    Replacement::draft(0x1417ff290, "physx_1417ff290", fn_1417ff290 as *const ()), // sub_1417ff290
    Replacement::draft(0x1417ff2f0, "physx_1417ff2f0", fn_1417ff2f0 as *const ()), // sub_1417ff2f0
    Replacement::draft(0x1417ff3c0, "physx_1417ff3c0", fn_1417ff3c0 as *const ()), // sub_1417ff3c0
    Replacement::draft(0x1417ff4d0, "physx_1417ff4d0", fn_1417ff4d0 as *const ()), // sub_1417ff4d0
    Replacement::draft(0x1417ff560, "physx_1417ff560", fn_1417ff560 as *const ()), // sub_1417ff560
    Replacement::draft(0x1417ff590, "physx_1417ff590", fn_1417ff590 as *const ()), // sub_1417ff590
    Replacement::draft(0x1417ff5c0, "physx_1417ff5c0", fn_1417ff5c0 as *const ()), // sub_1417ff5c0
    Replacement::draft(0x1417ff610, "physx_1417ff610", fn_1417ff610 as *const ()), // sub_1417ff610
    Replacement::draft(0x1417ff660, "physx_1417ff660", fn_1417ff660 as *const ()), // sub_1417ff660
    Replacement::draft(0x1417ff6b0, "physx_1417ff6b0", fn_1417ff6b0 as *const ()), // sub_1417ff6b0
    Replacement::draft(0x1417ff700, "physx_1417ff700", fn_1417ff700 as *const ()), // sub_1417ff700
    Replacement::draft(0x1417ff720, "physx_1417ff720", fn_1417ff720 as *const ()), // sub_1417ff720
    Replacement::draft(0x1417ff740, "physx_1417ff740", fn_1417ff740 as *const ()), // sub_1417ff740
    Replacement::draft(0x1417ff770, "physx_1417ff770", fn_1417ff770 as *const ()), // sub_1417ff770
    Replacement::draft(0x1417ff7a0, "physx_1417ff7a0", fn_1417ff7a0 as *const ()), // sub_1417ff7a0
    Replacement::draft(0x1417ff7d0, "physx_1417ff7d0", fn_1417ff7d0 as *const ()), // sub_1417ff7d0
    Replacement::draft(0x1417ff800, "physx_1417ff800", fn_1417ff800 as *const ()), // sub_1417ff800
    Replacement::draft(0x1417ff850, "physx_1417ff850", fn_1417ff850 as *const ()), // sub_1417ff850
    Replacement::draft(0x1417ff8a0, "physx_1417ff8a0", fn_1417ff8a0 as *const ()), // sub_1417ff8a0
    Replacement::draft(0x1417ffef0, "physx_1417ffef0", fn_1417ffef0 as *const ()), // sub_1417ffef0
    Replacement::draft(0x1418001c0, "physx_1418001c0", fn_1418001c0 as *const ()), // sub_1418001c0
    Replacement::draft(0x141800280, "physx_141800280", fn_141800280 as *const ()), // sub_141800280
    Replacement::draft(0x141800340, "physx_141800340", fn_141800340 as *const ()), // sub_141800340
    Replacement::draft(0x141800400, "physx_141800400", fn_141800400 as *const ()), // sub_141800400
    Replacement::draft(0x141800890, "physx_141800890", fn_141800890 as *const ()), // sub_141800890
    Replacement::draft(0x141801590, "physx_141801590", fn_141801590 as *const ()), // sub_141801590
    Replacement::draft(0x1418017a0, "physx_1418017a0", fn_1418017a0 as *const ()), // sub_1418017a0
    Replacement::draft(0x141801900, "physx_141801900", fn_141801900 as *const ()), // sub_141801900
    Replacement::draft(0x1418019e0, "physx_1418019e0", fn_1418019e0 as *const ()), // sub_1418019e0
    Replacement::draft(0x141801b00, "physx_141801b00", fn_141801b00 as *const ()), // sub_141801b00
    Replacement::draft(0x141801c00, "physx_141801c00", tpl_33759::<0x1418001c0, 0x88, 0x90, 0x8, 0x94, 0x80, 0x142645ee8, 0x10> as *const ()), // sub_141801c00
    Replacement::draft(0x141801ce0, "physx_141801ce0", fn_141801ce0 as *const ()), // sub_141801ce0
    Replacement::draft(0x141801d80, "physx_141801d80", fn_141801d80 as *const ()), // sub_141801d80
    Replacement::draft(0x141801e20, "physx_141801e20", tpl_33763::<0x94, 0x141801c00, 0x88, 0x90, 0xfffffffffffffff8, 0x8> as *const ()), // sub_141801e20
    Replacement::draft(0x141801f00, "physx_141801f00", fn_141801f00 as *const ()), // sub_141801f00
    Replacement::draft(0x1418023c0, "physx_1418023c0", fn_1418023c0 as *const ()), // sub_1418023c0
    Replacement::draft(0x141802d70, "physx_141802d70", fn_141802d70 as *const ()), // PxRigidBodyExt::PxRigidBodyExt::setMassAndUpdateInertia
    Replacement::draft(0x141802fd0, "physx_141802fd0", fn_141802fd0 as *const ()), // sub_141802fd0
    Replacement::draft(0x141803a70, "physx_141803a70", fn_141803a70 as *const ()), // sub_141803a70
    Replacement::draft(0x141803bf0, "physx_141803bf0", fn_141803bf0 as *const ()), // PxRigidBodyExt::PxRigidBodyExt::updateMassAndInertia
    Replacement::draft(0x141803de0, "physx_141803de0", fn_141803de0 as *const ()), // sub_141803de0
    Replacement::draft(0x141803fb0, "physx_141803fb0", fn_141803fb0 as *const ()), // sub_141803fb0
    Replacement::draft(0x141804010, "physx_141804010", tpl_33890::<0x68, 0x8, 0x70, 0x20> as *const ()), // sub_141804010
    Replacement::draft(0x141804060, "physx_141804060", tpl_33891::<0x18, 0x60, 0x143822580, 0x141821ab0> as *const ()), // sub_141804060
    Replacement::draft(0x1418040a0, "physx_1418040a0", tpl_33892::<0xa, 0x1426c9cd8, 0x1426c9e40, 0x18, 0x68, 0x8, 0x70, 0x143822580, 0x20, 0x60, 0x141821ab0> as *const ()), // sub_1418040a0
    Replacement::draft(0x141804150, "physx_141804150", fn_141804150 as *const ()), // sub_141804150
    Replacement::draft(0x141804190, "physx_141804190", tpl_33894::<0x68, 0x50, 0x60, 0x40> as *const ()), // sub_141804190
    Replacement::draft(0x1418041b0, "physx_1418041b0", tpl_28649::<0x68, 0x50> as *const ()), // sub_1418041b0
    Replacement::draft(0x1418041c0, "physx_1418041c0", tpl_33894::<0x68, 0x54, 0x60, 0x40> as *const ()), // sub_1418041c0
    Replacement::draft(0x1418041e0, "physx_1418041e0", tpl_28649::<0x68, 0x54> as *const ()), // sub_1418041e0
    Replacement::draft(0x1418041f0, "physx_1418041f0", tpl_33894::<0x68, 0x58, 0x60, 0x40> as *const ()), // sub_1418041f0
    Replacement::draft(0x141804210, "physx_141804210", tpl_28649::<0x68, 0x58> as *const ()), // sub_141804210
    Replacement::draft(0x141804220, "physx_141804220", tpl_33894::<0x68, 0x5c, 0x60, 0x40> as *const ()), // sub_141804220
    Replacement::draft(0x141804240, "physx_141804240", tpl_28649::<0x68, 0x5c> as *const ()), // sub_141804240
    Replacement::draft(0x141804250, "physx_141804250", fn_141804250 as *const ()), // sub_141804250
    Replacement::draft(0x141804270, "physx_141804270", tpl_28649::<0x68, 0x60> as *const ()), // sub_141804270
    Replacement::draft(0x141804280, "physx_141804280", tpl_33896::<0x68, 0x64> as *const ()), // sub_141804280
    Replacement::draft(0x1418042a0, "physx_1418042a0", tpl_33897::<0x68, 0x64, 0x60, 0x40> as *const ()), // sub_1418042a0
    Replacement::draft(0x1418042c0, "physx_1418042c0", tpl_33898::<0x68, 0x64, 0x60, 0x40> as *const ()), // sub_1418042c0
    Replacement::draft(0x1418042f0, "physx_1418042f0", fn_1418042f0 as *const ()), // sub_1418042f0
    Replacement::draft(0x141804350, "physx_141804350", tpl_1::<0x143822580> as *const ()), // sub_141804350
    Replacement::draft(0x141804360, "physx_141804360", fn_141804360 as *const ()), // sub_141804360
    Replacement::draft(0x141804790, "physx_141804790", tpl_85168 as *const ()), // sub_141804790
    Replacement::draft(0x141804794, "physx_141804794", tpl_28319::<0x1418047a0> as *const ()), // sub_141804794
    Replacement::draft(0x1418047a0, "physx_1418047a0", tpl_93622::<0xa, 0x1426c9cd8, 0x1426c9e40, 0x18, 0x68, 0x142645ee8, 0x10, 0x1433e6be0, 0x1426c9ca8, 0x1401de410> as *const ()), // sub_1418047a0
    Replacement::draft(0x141804850, "physx_141804850", tpl_27062::<0x1426c9ca8, 0x141834d30> as *const ()), // sub_141804850
    Replacement::draft(0x141804880, "physx_141804880", fn_141804880 as *const ()), // sub_141804880
    Replacement::draft(0x1418048a0, "physx_1418048a0", tpl_27067::<0x60, 0x78> as *const ()), // sub_1418048a0
    Replacement::draft(0x1418048b0, "physx_1418048b0", fn_1418048b0 as *const ()), // sub_1418048b0
    Replacement::draft(0x141804b20, "physx_141804b20", tpl_913::<0x50> as *const ()), // sub_141804b20
    Replacement::draft(0x141804b30, "physx_141804b30", tpl_913::<0x60> as *const ()), // sub_141804b30
    Replacement::draft(0x141804b40, "physx_141804b40", tpl_33902::<0x60, 0x50> as *const ()), // sub_141804b40
    Replacement::draft(0x141804b60, "physx_141804b60", fn_141804b60 as *const ()), // sub_141804b60
    Replacement::draft(0x141804b70, "physx_141804b70", tpl_28649::<0x68, 0x4> as *const ()), // sub_141804b70
    Replacement::draft(0x141804b80, "physx_141804b80", tpl_28649::<0x68, 0xc> as *const ()), // sub_141804b80
    Replacement::draft(0x141804b90, "physx_141804b90", tpl_33904::<0x68> as *const ()), // sub_141804b90
    Replacement::draft(0x141804ba0, "physx_141804ba0", tpl_28649::<0x68, 0x8> as *const ()), // sub_141804ba0
    Replacement::draft(0x141804bb0, "physx_141804bb0", fn_141804bb0 as *const ()), // sub_141804bb0
    Replacement::draft(0x141804c00, "physx_141804c00", tpl_366::<0x143822580> as *const ()), // sub_141804c00
    Replacement::draft(0x141804c10, "physx_141804c10", fn_141804c10 as *const ()), // sub_141804c10
    Replacement::draft(0x141804e80, "physx_141804e80", fn_141804e80 as *const ()), // sub_141804e80
    Replacement::draft(0x1418054a0, "physx_1418054a0", fn_1418054a0 as *const ()), // sub_1418054a0
    Replacement::draft(0x1418059c0, "physx_1418059c0", tpl_28648::<0x60, 0x28> as *const ()), // sub_1418059c0
    Replacement::draft(0x1418059e0, "physx_1418059e0", tpl_548::<0xffffffffffffffe8> as *const ()), // sub_1418059e0
    Replacement::draft(0x1418059f0, "physx_1418059f0", fn_1418059f0 as *const ()), // sub_1418059f0
    Replacement::draft(0x141805eb0, "physx_141805eb0", fn_141805eb0 as *const ()), // sub_141805eb0
    Replacement::draft(0x141805ee0, "physx_141805ee0", tpl_33911::<0x1426c7da0, 0x14183ad52, 0x1426c9cd0, 0xffffffffffffffff, 0x1426c9ca0> as *const ()), // sub_141805ee0
    Replacement::draft(0x141805f70, "physx_141805f70", tpl_29360::<0xa> as *const ()), // sub_141805f70
    Replacement::draft(0x141805f80, "physx_141805f80", fn_141805f80 as *const ()), // sub_141805f80
    Replacement::draft(0x1418061c0, "physx_1418061c0", fn_1418061c0 as *const ()), // sub_1418061c0
    Replacement::draft(0x141806210, "physx_141806210", fn_141806210 as *const ()), // sub_141806210
    Replacement::draft(0x141806320, "physx_141806320", tpl_27275::<0x60> as *const ()), // sub_141806320
    Replacement::draft(0x141806330, "physx_141806330", fn_141806330 as *const ()), // sub_141806330
    Replacement::draft(0x1418063a0, "physx_1418063a0", fn_1418063a0 as *const ()), // sub_1418063a0
    Replacement::draft(0x141806740, "physx_141806740", tpl_27067::<0x60, 0x70> as *const ()), // sub_141806740
    Replacement::draft(0x141806750, "physx_141806750", tpl_27067::<0x60, 0x58> as *const ()), // sub_141806750
    Replacement::draft(0x141806760, "physx_141806760", fn_141806760 as *const ()), // sub_141806760
    Replacement::draft(0x141806780, "physx_141806780", tpl_33894::<0x68, 0x4, 0x60, 0x40> as *const ()), // sub_141806780
    Replacement::draft(0x1418067a0, "physx_1418067a0", tpl_33894::<0x68, 0xc, 0x60, 0x40> as *const ()), // sub_1418067a0
    Replacement::draft(0x1418067c0, "physx_1418067c0", fn_1418067c0 as *const ()), // sub_1418067c0
    Replacement::draft(0x1418067e0, "physx_1418067e0", tpl_33894::<0x68, 0x8, 0x60, 0x40> as *const ()), // sub_1418067e0
    Replacement::draft(0x141806800, "physx_141806800", fn_141806800 as *const ()), // sub_141806800
    Replacement::draft(0x141806ad0, "physx_141806ad0", fn_141806ad0 as *const ()), // sub_141806ad0
    Replacement::draft(0x141806c00, "physx_141806c00", fn_141806c00 as *const ()), // sub_141806c00
    Replacement::draft(0x141806c60, "physx_141806c60", tpl_33890::<0x68, 0x8, 0x190, 0x20> as *const ()), // sub_141806c60
    Replacement::draft(0x141806cb0, "physx_141806cb0", tpl_33891::<0x18, 0x60, 0x1438225a0, 0x141821ab0> as *const ()), // sub_141806cb0
    Replacement::draft(0x141806cf0, "physx_141806cf0", fn_141806cf0 as *const ()), // sub_141806cf0
    Replacement::draft(0x141806da0, "physx_141806da0", fn_141806da0 as *const ()), // sub_141806da0
    Replacement::draft(0x141807010, "physx_141807010", fn_141807010 as *const ()), // sub_141807010
    Replacement::draft(0x1418070b0, "physx_1418070b0", fn_1418070b0 as *const ()), // sub_1418070b0
    Replacement::draft(0x141807180, "physx_141807180", fn_141807180 as *const ()), // sub_141807180
    Replacement::draft(0x141807250, "physx_141807250", fn_141807250 as *const ()), // sub_141807250
    Replacement::draft(0x141807270, "physx_141807270", fn_141807270 as *const ()), // sub_141807270
    Replacement::draft(0x1418072a0, "physx_1418072a0", fn_1418072a0 as *const ()), // sub_1418072a0
    Replacement::draft(0x1418072e0, "physx_1418072e0", fn_1418072e0 as *const ()), // sub_1418072e0
    Replacement::draft(0x141807330, "physx_141807330", fn_141807330 as *const ()), // sub_141807330
    Replacement::draft(0x141807350, "physx_141807350", fn_141807350 as *const ()), // sub_141807350
    Replacement::draft(0x141807380, "physx_141807380", tpl_33933::<0x68, 0x80, 0x90, 0x98, 0x10, 0x18> as *const ()), // sub_141807380
    Replacement::draft(0x1418073b0, "physx_1418073b0", tpl_33934::<0x68, 0x80, 0x10, 0x90, 0x18, 0x98, 0x71, 0x60, 0x40> as *const ()), // sub_1418073b0
    Replacement::draft(0x1418073f0, "physx_1418073f0", tpl_33933::<0x68, 0x9c, 0xac, 0xb4, 0x10, 0x18> as *const ()), // sub_1418073f0
    Replacement::draft(0x141807420, "physx_141807420", tpl_33934::<0x68, 0x9c, 0x10, 0xac, 0x18, 0xb4, 0x71, 0x60, 0x40> as *const ()), // sub_141807420
    Replacement::draft(0x141807460, "physx_141807460", fn_141807460 as *const ()), // sub_141807460
    Replacement::draft(0x1418074b0, "physx_1418074b0", fn_1418074b0 as *const ()), // sub_1418074b0
    Replacement::draft(0x141807580, "physx_141807580", fn_141807580 as *const ()), // sub_141807580
    Replacement::draft(0x1418075d0, "physx_1418075d0", fn_1418075d0 as *const ()), // sub_1418075d0
    Replacement::draft(0x141807630, "physx_141807630", tpl_33894::<0x68, 0x184, 0x60, 0x40> as *const ()), // sub_141807630
    Replacement::draft(0x141807650, "physx_141807650", tpl_28649::<0x68, 0x184> as *const ()), // sub_141807650
    Replacement::draft(0x141807670, "physx_141807670", tpl_33894::<0x68, 0x180, 0x60, 0x40> as *const ()), // sub_141807670
    Replacement::draft(0x141807690, "physx_141807690", tpl_28649::<0x68, 0x180> as *const ()), // sub_141807690
    Replacement::draft(0x1418076b0, "physx_1418076b0", fn_1418076b0 as *const ()), // sub_1418076b0
    Replacement::draft(0x141807710, "physx_141807710", fn_141807710 as *const ()), // sub_141807710
    Replacement::draft(0x141807a40, "physx_141807a40", tpl_1::<0x1438225a0> as *const ()), // sub_141807a40
    Replacement::draft(0x141807a50, "physx_141807a50", fn_141807a50 as *const ()), // sub_141807a50
    Replacement::draft(0x141807bb0, "physx_141807bb0", fn_141807bb0 as *const ()), // sub_141807bb0
    Replacement::draft(0x141808720, "physx_141808720", fn_141808720 as *const ()), // sub_141808720
    Replacement::draft(0x14180923c, "physx_14180923c", tpl_28319::<0x141809250> as *const ()), // sub_14180923c
    Replacement::draft(0x141809250, "physx_141809250", fn_141809250 as *const ()), // sub_141809250
    Replacement::draft(0x141809300, "physx_141809300", fn_141809300 as *const ()), // sub_141809300
    Replacement::draft(0x1418097f0, "physx_1418097f0", tpl_366::<0x1438225a0> as *const ()), // sub_1418097f0
    Replacement::draft(0x141809800, "physx_141809800", tpl_33911::<0x1426c7d90, 0x14183ad52, 0x1426c9cd0, 0xffffffffffffffff, 0x1426c9ca0> as *const ()), // sub_141809800
    Replacement::draft(0x141809890, "physx_141809890", fn_141809890 as *const ()), // sub_141809890
    Replacement::draft(0x1418098f0, "physx_1418098f0", fn_1418098f0 as *const ()), // sub_1418098f0
    Replacement::draft(0x141809a50, "physx_141809a50", fn_141809a50 as *const ()), // sub_141809a50
    Replacement::draft(0x141809b80, "physx_141809b80", fn_141809b80 as *const ()), // sub_141809b80
    Replacement::draft(0x141809d10, "physx_141809d10", fn_141809d10 as *const ()), // sub_141809d10
    Replacement::draft(0x141809d70, "physx_141809d70", tpl_33890::<0x68, 0x8, 0x60, 0x20> as *const ()), // sub_141809d70
    Replacement::draft(0x141809dc0, "physx_141809dc0", tpl_33891::<0x18, 0x60, 0x1438225c0, 0x141821ab0> as *const ()), // sub_141809dc0
    Replacement::draft(0x141809e00, "physx_141809e00", fn_141809e00 as *const ()), // sub_141809e00
    Replacement::draft(0x141809eb0, "physx_141809eb0", tpl_28649::<0x68, 0x50> as *const ()), // sub_141809eb0
    Replacement::draft(0x141809ec0, "physx_141809ec0", tpl_33894::<0x68, 0x50, 0x60, 0x40> as *const ()), // sub_141809ec0
    Replacement::draft(0x141809ee0, "physx_141809ee0", tpl_28649::<0x68, 0x54> as *const ()), // sub_141809ee0
    Replacement::draft(0x141809ef0, "physx_141809ef0", tpl_33894::<0x68, 0x54, 0x60, 0x40> as *const ()), // sub_141809ef0
    Replacement::draft(0x141809f10, "physx_141809f10", fn_141809f10 as *const ()), // sub_141809f10
    Replacement::draft(0x141809f70, "physx_141809f70", tpl_1::<0x1438225c0> as *const ()), // sub_141809f70
    Replacement::draft(0x141809f80, "physx_141809f80", fn_141809f80 as *const ()), // sub_141809f80
    Replacement::draft(0x14180a2e0, "physx_14180a2e0", fn_14180a2e0 as *const ()), // sub_14180a2e0
    Replacement::draft(0x14180ad0c, "physx_14180ad0c", tpl_28319::<0x14180ad20> as *const ()), // sub_14180ad0c
    Replacement::draft(0x14180ad20, "physx_14180ad20", tpl_93622::<0xa, 0x1426ca1c0, 0x1426ca2d8, 0x18, 0x68, 0x142645ee8, 0x10, 0x1433e6be0, 0x1426c9ca8, 0x1401de410> as *const ()), // sub_14180ad20
    Replacement::draft(0x14180add0, "physx_14180add0", tpl_366::<0x1438225c0> as *const ()), // sub_14180add0
    Replacement::draft(0x14180ade0, "physx_14180ade0", tpl_33911::<0x1426c7db0, 0x14183ad52, 0x1426c9cd0, 0xffffffffffffffff, 0x1426c9ca0> as *const ()), // sub_14180ade0
    Replacement::draft(0x14180ae70, "physx_14180ae70", fn_14180ae70 as *const ()), // sub_14180ae70
    Replacement::draft(0x14180b0a0, "physx_14180b0a0", tpl_33948::<0x68, 0x18, 0x10, 0x20> as *const ()), // sub_14180b0a0
    Replacement::draft(0x14180b100, "physx_14180b100", tpl_33890::<0x68, 0x8, 0x80, 0x20> as *const ()), // sub_14180b100
    Replacement::draft(0x14180b150, "physx_14180b150", tpl_33891::<0x18, 0x60, 0x1438225e0, 0x141821ab0> as *const ()), // sub_14180b150
    Replacement::draft(0x14180b190, "physx_14180b190", tpl_33892::<0xa, 0x1426ca3f0, 0x1426ca540, 0x18, 0x68, 0x8, 0x80, 0x1438225e0, 0x20, 0x60, 0x141821ab0> as *const ()), // sub_14180b190
    Replacement::draft(0x14180b240, "physx_14180b240", tpl_33894::<0x68, 0x70, 0x60, 0x40> as *const ()), // sub_14180b240
    Replacement::draft(0x14180b260, "physx_14180b260", tpl_28649::<0x68, 0x70> as *const ()), // sub_14180b260
    Replacement::draft(0x14180b270, "physx_14180b270", tpl_33894::<0x68, 0x6c, 0x60, 0x40> as *const ()), // sub_14180b270
    Replacement::draft(0x14180b290, "physx_14180b290", tpl_28649::<0x68, 0x6c> as *const ()), // sub_14180b290
    Replacement::draft(0x14180b2a0, "physx_14180b2a0", tpl_33949::<0x68, 0x50, 0x60, 0x10, 0x18> as *const ()), // sub_14180b2a0
    Replacement::draft(0x14180b2d0, "physx_14180b2d0", tpl_33950::<0x68, 0x50, 0x10, 0x60, 0x18, 0x40> as *const ()), // sub_14180b2d0
    Replacement::draft(0x14180b300, "physx_14180b300", tpl_33896::<0x68, 0x74> as *const ()), // sub_14180b300
    Replacement::draft(0x14180b320, "physx_14180b320", tpl_33897::<0x68, 0x74, 0x60, 0x40> as *const ()), // sub_14180b320
    Replacement::draft(0x14180b340, "physx_14180b340", tpl_33898::<0x68, 0x74, 0x60, 0x40> as *const ()), // sub_14180b340
    Replacement::draft(0x14180b370, "physx_14180b370", tpl_33951::<0x18, 0x1438225e0, 0xf0, 0x60> as *const ()), // sub_14180b370
    Replacement::draft(0x14180b3d0, "physx_14180b3d0", tpl_1::<0x1438225e0> as *const ()), // sub_14180b3d0
    Replacement::draft(0x14180b3e0, "physx_14180b3e0", fn_14180b3e0 as *const ()), // sub_14180b3e0
    Replacement::draft(0x14180b8b0, "physx_14180b8b0", fn_14180b8b0 as *const ()), // sub_14180b8b0
    Replacement::draft(0x14180c2d8, "physx_14180c2d8", tpl_28319::<0x14180c2f0> as *const ()), // sub_14180c2d8
    Replacement::draft(0x14180c2f0, "physx_14180c2f0", tpl_93622::<0xa, 0x1426ca3f0, 0x1426ca540, 0x18, 0x68, 0x142645ee8, 0x10, 0x1433e6be0, 0x1426c9ca8, 0x1401de410> as *const ()), // sub_14180c2f0
    Replacement::draft(0x14180c3a0, "physx_14180c3a0", fn_14180c3a0 as *const ()), // sub_14180c3a0
    Replacement::draft(0x14180c3c0, "physx_14180c3c0", tpl_366::<0x1438225e0> as *const ()), // sub_14180c3c0
    Replacement::draft(0x14180c3d0, "physx_14180c3d0", fn_14180c3d0 as *const ()), // sub_14180c3d0
    Replacement::draft(0x14180c3f0, "physx_14180c3f0", tpl_33911::<0x1426c7dc0, 0x14183ad52, 0x1426c9cd0, 0xffffffffffffffff, 0x1426c9ca0> as *const ()), // sub_14180c3f0
    Replacement::draft(0x14180c480, "physx_14180c480", fn_14180c480 as *const ()), // sub_14180c480
    Replacement::draft(0x14180c6a0, "physx_14180c6a0", tpl_33891::<0x18, 0x60, 0x143822600, 0x141821ab0> as *const ()), // sub_14180c6a0
    Replacement::draft(0x14180c6e0, "physx_14180c6e0", fn_14180c6e0 as *const ()), // sub_14180c6e0
    Replacement::draft(0x14180c740, "physx_14180c740", tpl_33890::<0x68, 0x8, 0x90, 0x20> as *const ()), // sub_14180c740
    Replacement::draft(0x14180c790, "physx_14180c790", tpl_33892::<0xa, 0x1426ca660, 0x1426ca7e0, 0x18, 0x68, 0x8, 0x90, 0x143822600, 0x20, 0x60, 0x141821ab0> as *const ()), // sub_14180c790
    Replacement::draft(0x14180c840, "physx_14180c840", fn_14180c840 as *const ()), // sub_14180c840
    Replacement::draft(0x14180c8f0, "physx_14180c8f0", fn_14180c8f0 as *const ()), // sub_14180c8f0
    Replacement::draft(0x14180c940, "physx_14180c940", tpl_33933::<0x68, 0x5c, 0x6c, 0x74, 0x10, 0x18> as *const ()), // sub_14180c940
    Replacement::draft(0x14180c970, "physx_14180c970", fn_14180c970 as *const ()), // sub_14180c970
    Replacement::draft(0x14180c9a0, "physx_14180c9a0", tpl_28649::<0x68, 0x50> as *const ()), // sub_14180c9a0
    Replacement::draft(0x14180c9b0, "physx_14180c9b0", tpl_33894::<0x68, 0x50, 0x60, 0x40> as *const ()), // sub_14180c9b0
    Replacement::draft(0x14180c9d0, "physx_14180c9d0", tpl_28649::<0x68, 0x54> as *const ()), // sub_14180c9d0
    Replacement::draft(0x14180c9e0, "physx_14180c9e0", tpl_33894::<0x68, 0x54, 0x60, 0x40> as *const ()), // sub_14180c9e0
    Replacement::draft(0x14180ca00, "physx_14180ca00", tpl_28649::<0x68, 0x58> as *const ()), // sub_14180ca00
    Replacement::draft(0x14180ca10, "physx_14180ca10", tpl_33894::<0x68, 0x58, 0x60, 0x40> as *const ()), // sub_14180ca10
    Replacement::draft(0x14180ca30, "physx_14180ca30", tpl_33894::<0x68, 0x88, 0x60, 0x40> as *const ()), // sub_14180ca30
    Replacement::draft(0x14180ca50, "physx_14180ca50", tpl_28649::<0x68, 0x88> as *const ()), // sub_14180ca50
    Replacement::draft(0x14180ca70, "physx_14180ca70", tpl_33894::<0x68, 0x84, 0x60, 0x40> as *const ()), // sub_14180ca70
    Replacement::draft(0x14180ca90, "physx_14180ca90", tpl_28649::<0x68, 0x84> as *const ()), // sub_14180ca90
    Replacement::draft(0x14180cab0, "physx_14180cab0", tpl_33896::<0x68, 0x8c> as *const ()), // sub_14180cab0
    Replacement::draft(0x14180cad0, "physx_14180cad0", tpl_33959::<0x68, 0x8c> as *const ()), // sub_14180cad0
    Replacement::draft(0x14180caf0, "physx_14180caf0", tpl_33898::<0x68, 0x8c, 0x60, 0x40> as *const ()), // sub_14180caf0
    Replacement::draft(0x14180cb30, "physx_14180cb30", fn_14180cb30 as *const ()), // sub_14180cb30
    Replacement::draft(0x14180cbb0, "physx_14180cbb0", fn_14180cbb0 as *const ()), // sub_14180cbb0
    Replacement::draft(0x14180cc10, "physx_14180cc10", tpl_1::<0x143822600> as *const ()), // sub_14180cc10
    Replacement::draft(0x14180cc20, "physx_14180cc20", fn_14180cc20 as *const ()), // sub_14180cc20
    Replacement::draft(0x14180d710, "physx_14180d710", fn_14180d710 as *const ()), // sub_14180d710
    Replacement::draft(0x14180dbb0, "physx_14180dbb0", tpl_28319::<0x14180dbc0> as *const ()), // sub_14180dbb0
    Replacement::draft(0x14180dbc0, "physx_14180dbc0", tpl_93622::<0xa, 0x1426ca660, 0x1426ca7e0, 0x18, 0x68, 0x142645ee8, 0x10, 0x1433e6be0, 0x1426c9ca8, 0x1401de410> as *const ()), // sub_14180dbc0
    Replacement::draft(0x14180dc70, "physx_14180dc70", tpl_366::<0x143822600> as *const ()), // sub_14180dc70
    Replacement::draft(0x14180dc80, "physx_14180dc80", tpl_33911::<0x1426c7dd8, 0x14183ad52, 0x1426c9cd0, 0xffffffffffffffff, 0x1426c9ca0> as *const ()), // sub_14180dc80
    Replacement::draft(0x14180dd10, "physx_14180dd10", fn_14180dd10 as *const ()), // sub_14180dd10
    Replacement::draft(0x14180df10, "physx_14180df10", tpl_33948::<0x68, 0x18, 0x10, 0x20> as *const ()), // sub_14180df10
    Replacement::draft(0x14180df70, "physx_14180df70", tpl_33890::<0x68, 0x8, 0x80, 0x20> as *const ()), // sub_14180df70
    Replacement::draft(0x14180dfc0, "physx_14180dfc0", tpl_33891::<0x18, 0x60, 0x143822620, 0x141821ab0> as *const ()), // sub_14180dfc0
    Replacement::draft(0x14180e000, "physx_14180e000", tpl_33892::<0xa, 0x1426ca8f0, 0x1426caa20, 0x18, 0x68, 0x8, 0x80, 0x143822620, 0x20, 0x60, 0x141821ab0> as *const ()), // sub_14180e000
    Replacement::draft(0x14180e0b0, "physx_14180e0b0", tpl_33894::<0x68, 0x78, 0x60, 0x40> as *const ()), // sub_14180e0b0
    Replacement::draft(0x14180e0d0, "physx_14180e0d0", tpl_28649::<0x68, 0x78> as *const ()), // sub_14180e0d0
    Replacement::draft(0x14180e0e0, "physx_14180e0e0", tpl_33950::<0x68, 0x50, 0x10, 0x60, 0x18, 0x40> as *const ()), // sub_14180e0e0
    Replacement::draft(0x14180e110, "physx_14180e110", tpl_33949::<0x68, 0x50, 0x60, 0x10, 0x18> as *const ()), // sub_14180e110
    Replacement::draft(0x14180e140, "physx_14180e140", tpl_33896::<0x68, 0x7c> as *const ()), // sub_14180e140
    Replacement::draft(0x14180e160, "physx_14180e160", tpl_33959::<0x68, 0x7c> as *const ()), // sub_14180e160
    Replacement::draft(0x14180e180, "physx_14180e180", tpl_33898::<0x68, 0x7c, 0x60, 0x40> as *const ()), // sub_14180e180
    Replacement::draft(0x14180e1b0, "physx_14180e1b0", fn_14180e1b0 as *const ()), // sub_14180e1b0
    Replacement::draft(0x14180e230, "physx_14180e230", tpl_33951::<0x18, 0x143822620, 0xf0, 0x60> as *const ()), // sub_14180e230
    Replacement::draft(0x14180e290, "physx_14180e290", tpl_1::<0x143822620> as *const ()), // sub_14180e290
    Replacement::draft(0x14180e2a0, "physx_14180e2a0", fn_14180e2a0 as *const ()), // sub_14180e2a0
    Replacement::draft(0x14180e820, "physx_14180e820", fn_14180e820 as *const ()), // sub_14180e820
    Replacement::draft(0x14180f168, "physx_14180f168", tpl_28319::<0x14180f180> as *const ()), // sub_14180f168
    Replacement::draft(0x14180f180, "physx_14180f180", tpl_93622::<0xa, 0x1426ca8f0, 0x1426caa20, 0x18, 0x68, 0x142645ee8, 0x10, 0x1433e6be0, 0x1426c9ca8, 0x1401de410> as *const ()), // sub_14180f180
    Replacement::draft(0x14180f230, "physx_14180f230", tpl_366::<0x143822620> as *const ()), // sub_14180f230
    Replacement::draft(0x14180f240, "physx_14180f240", tpl_33911::<0x1426c7de8, 0x14183ad52, 0x1426c9cd0, 0xffffffffffffffff, 0x1426c9ca0> as *const ()), // sub_14180f240
    Replacement::draft(0x14180f2d0, "physx_14180f2d0", fn_14180f2d0 as *const ()), // sub_14180f2d0
    Replacement::draft(0x14180f310, "physx_14180f310", fn_14180f310 as *const ()), // sub_14180f310
    Replacement::draft(0x14180f330, "physx_14180f330", fn_14180f330 as *const ()), // sub_14180f330
    Replacement::draft(0x14180f5a0, "physx_14180f5a0", tpl_233676::<0x1426cafd4, 0x1426c7f00, 0x1417ee690, 0x1426cac18, 0x1426cac20, 0x141817100, 0x141816ca0, 0x141817190, 0x141816d90, 0x1426c7ef0, 0x1417dd700, 0x1426cac30, 0x141834fb8, 0x143426820, 0x1418196b0, 0x1426cabf8, 0x1426cac08, 0x141816e80, 0x1426cac40, 0x141816bb0, 0x141817210, 0x141817330, 0x141834d30> as *const ()), // sub_14180f5a0
    Replacement::draft(0x14180fa30, "physx_14180fa30", fn_14180fa30 as *const ()), // sub_14180fa30
    Replacement::draft(0x14180fdb0, "physx_14180fdb0", tpl_233676::<0x1426cafd4, 0x1426c7f00, 0x1417ee690, 0x1426cac18, 0x1426cac20, 0x141817100, 0x141816ca0, 0x141817190, 0x141816d90, 0x1426c7ef0, 0x1417dd700, 0x1426cac30, 0x141834fb8, 0x143426820, 0x1418196b0, 0x1426cabf8, 0x1426cac08, 0x141816e80, 0x1426cac40, 0x141816bb0, 0x141817210, 0x141817330, 0x141834d30> as *const ()), // sub_14180fdb0
    Replacement::draft(0x141810240, "physx_141810240", fn_141810240 as *const ()), // sub_141810240
    Replacement::draft(0x1418105b0, "physx_1418105b0", fn_1418105b0 as *const ()), // sub_1418105b0
    Replacement::draft(0x141810930, "physx_141810930", fn_141810930 as *const ()), // sub_141810930
    Replacement::draft(0x141810b50, "physx_141810b50", fn_141810b50 as *const ()), // sub_141810b50
    Replacement::draft(0x141810dd0, "physx_141810dd0", fn_141810dd0 as *const ()), // sub_141810dd0
    Replacement::draft(0x141810ff0, "physx_141810ff0", tpl_33973::<0x1434281d0, 0xa8> as *const ()), // sub_141810ff0
    Replacement::draft(0x141811030, "physx_141811030", tpl_33973::<0x1434281d0, 0xb0> as *const ()), // sub_141811030
    Replacement::draft(0x141811070, "physx_141811070", fn_141811070 as *const ()), // sub_141811070
    Replacement::draft(0x141811200, "physx_141811200", fn_141811200 as *const ()), // sub_141811200
    Replacement::draft(0x141811220, "physx_141811220", fn_141811220 as *const ()), // sub_141811220
    Replacement::draft(0x141811590, "physx_141811590", fn_141811590 as *const ()), // sub_141811590
    Replacement::draft(0x1418119d0, "physx_1418119d0", fn_1418119d0 as *const ()), // sub_1418119d0
    Replacement::draft(0x1418120b0, "physx_1418120b0", fn_1418120b0 as *const ()), // sub_1418120b0
    Replacement::draft(0x141812240, "physx_141812240", fn_141812240 as *const ()), // sub_141812240
    Replacement::draft(0x141812fc0, "physx_141812fc0", fn_141812fc0 as *const ()), // sub_141812fc0
    Replacement::draft(0x141813540, "physx_141813540", fn_141813540 as *const ()), // sub_141813540
    Replacement::draft(0x141813560, "physx_141813560", fn_141813560 as *const ()), // sub_141813560
    Replacement::draft(0x141813600, "physx_141813600", tpl_33982::<0x1417ee690, 0x1426c7f00, 0x14183acc2, 0xc, 0x1426c7efc, 0x142645f10> as *const ()), // sub_141813600
    Replacement::draft(0x1418136b0, "physx_1418136b0", tpl_33982::<0x1417ee690, 0x1426c7f00, 0x14183acc2, 0xc, 0x1426c7efc, 0x142645f10> as *const ()), // sub_1418136b0
    Replacement::draft(0x141813760, "physx_141813760", tpl_33982::<0x1417ee690, 0x1426c7f00, 0x14183acc2, 0xc, 0x1426c7efc, 0x142645f10> as *const ()), // sub_141813760
    Replacement::draft(0x141813810, "physx_141813810", fn_141813810 as *const ()), // sub_141813810
    Replacement::draft(0x141813880, "physx_141813880", fn_141813880 as *const ()), // sub_141813880
    Replacement::draft(0x141813910, "physx_141813910", fn_141813910 as *const ()), // sub_141813910
    Replacement::draft(0x141813ad0, "physx_141813ad0", fn_141813ad0 as *const ()), // sub_141813ad0
    Replacement::draft(0x141813c90, "physx_141813c90", fn_141813c90 as *const ()), // sub_141813c90
    Replacement::draft(0x141813df0, "physx_141813df0", fn_141813df0 as *const ()), // sub_141813df0
    Replacement::draft(0x141813fb0, "physx_141813fb0", fn_141813fb0 as *const ()), // sub_141813fb0
    Replacement::draft(0x141814170, "physx_141814170", fn_141814170 as *const ()), // sub_141814170
    Replacement::draft(0x141814330, "physx_141814330", fn_141814330 as *const ()), // sub_141814330
    Replacement::draft(0x141814510, "physx_141814510", fn_141814510 as *const ()), // sub_141814510
    Replacement::draft(0x141814760, "physx_141814760", tpl_93713::<0x14397d130, 0x14183acbc, 0x14266801b, 0x142646980, 0x1417f31f0> as *const ()), // sub_141814760
    Replacement::draft(0x141814980, "physx_141814980", tpl_93713::<0x14397d12c, 0x14183acbc, 0x14266801b, 0x142646980, 0x1417f31f0> as *const ()), // sub_141814980
    Replacement::draft(0x141814ba0, "physx_141814ba0", fn_141814ba0 as *const ()), // sub_141814ba0
    Replacement::draft(0x141814e20, "physx_141814e20", fn_141814e20 as *const ()), // sub_141814e20
    Replacement::draft(0x141815120, "physx_141815120", tpl_93713::<0x14397d128, 0x14183acbc, 0x14266801b, 0x142646980, 0x1417f31f0> as *const ()), // sub_141815120
    Replacement::draft(0x141815340, "physx_141815340", tpl_93713::<0x14397d13c, 0x14183acbc, 0x14266801b, 0x142646980, 0x1417f31f0> as *const ()), // sub_141815340
    Replacement::draft(0x141815560, "physx_141815560", tpl_93713::<0x14397d140, 0x14183acbc, 0x14266801b, 0x142646980, 0x1417f31f0> as *const ()), // sub_141815560
    Replacement::draft(0x141815780, "physx_141815780", fn_141815780 as *const ()), // sub_141815780
    Replacement::draft(0x141815940, "physx_141815940", fn_141815940 as *const ()), // sub_141815940
    Replacement::draft(0x141815b60, "physx_141815b60", tpl_93713::<0x14397d138, 0x14183acbc, 0x14266801b, 0x142646980, 0x1417f31f0> as *const ()), // sub_141815b60
    Replacement::draft(0x141815d80, "physx_141815d80", fn_141815d80 as *const ()), // sub_141815d80
    Replacement::draft(0x141816090, "physx_141816090", fn_141816090 as *const ()), // sub_141816090
    Replacement::draft(0x141816420, "physx_141816420", fn_141816420 as *const ()), // sub_141816420
    Replacement::draft(0x141816850, "physx_141816850", fn_141816850 as *const ()), // sub_141816850
    Replacement::draft(0x141816960, "physx_141816960", fn_141816960 as *const ()), // sub_141816960
    Replacement::draft(0x141816aa0, "physx_141816aa0", fn_141816aa0 as *const ()), // sub_141816aa0
    Replacement::draft(0x141816bb0, "physx_141816bb0", fn_141816bb0 as *const ()), // sub_141816bb0
    Replacement::draft(0x141816ca0, "physx_141816ca0", fn_141816ca0 as *const ()), // sub_141816ca0
    Replacement::draft(0x141816d90, "physx_141816d90", fn_141816d90 as *const ()), // sub_141816d90
    Replacement::draft(0x141816e80, "physx_141816e80", fn_141816e80 as *const ()), // sub_141816e80
    Replacement::draft(0x141816e90, "physx_141816e90", fn_141816e90 as *const ()), // sub_141816e90
    Replacement::draft(0x141816f80, "physx_141816f80", fn_141816f80 as *const ()), // sub_141816f80
    Replacement::draft(0x141817100, "physx_141817100", fn_141817100 as *const ()), // sub_141817100
    Replacement::draft(0x141817190, "physx_141817190", fn_141817190 as *const ()), // sub_141817190
    Replacement::draft(0x141817210, "physx_141817210", fn_141817210 as *const ()), // sub_141817210
    Replacement::draft(0x141817330, "physx_141817330", fn_141817330 as *const ()), // sub_141817330
    Replacement::draft(0x141817460, "physx_141817460", tpl_27239::<0x141817470> as *const ()), // sub_141817460
    Replacement::draft(0x141817470, "physx_141817470", tpl_790::<0x141817210, 0x141834d30> as *const ()), // sub_141817470
    Replacement::draft(0x1418174b0, "physx_1418174b0", fn_1418174b0 as *const ()), // sub_1418174b0
    Replacement::draft(0x141817600, "physx_141817600", fn_141817600 as *const ()), // sub_141817600
    Replacement::draft(0x141817750, "physx_141817750", fn_141817750 as *const ()), // sub_141817750
    Replacement::draft(0x1418178a0, "physx_1418178a0", fn_1418178a0 as *const ()), // sub_1418178a0
    Replacement::draft(0x141817a00, "physx_141817a00", fn_141817a00 as *const ()), // sub_141817a00
    Replacement::draft(0x141818210, "physx_141818210", fn_141818210 as *const ()), // sub_141818210
    Replacement::draft(0x141818a20, "physx_141818a20", fn_141818a20 as *const ()), // sub_141818a20
    Replacement::draft(0x141818b90, "physx_141818b90", fn_141818b90 as *const ()), // sub_141818b90
    Replacement::draft(0x141818c90, "physx_141818c90", fn_141818c90 as *const ()), // sub_141818c90
    Replacement::draft(0x141818e70, "physx_141818e70", tpl_34001::<0x1426c7ef0, 0x1417dd700, 0x1426cacd0, 0x1426cae68, 0x1426cafd4, 0x1426c7f00, 0x1426cae80, 0x141819960, 0x1426cabc0, 0x141816e90, 0x1426cabd0, 0x141813880, 0x1426cabe0, 0x143822a20, 0x1426cabec, 0x141816f80, 0x1426cae98, 0x1438228a0, 0x1426caea8, 0x1417f36e0, 0x143822920, 0x1426caec0, 0x1426caed8, 0x1417ee000, 0x141816960> as *const ()), // sub_141818e70
    Replacement::draft(0x141819200, "physx_141819200", tpl_34001::<0x1426c7ef0, 0x1417dd700, 0x1426cacd0, 0x1426cae68, 0x1426cafd4, 0x1426c7f00, 0x1426cae80, 0x141819960, 0x1426cabc0, 0x141816e90, 0x1426cabd0, 0x141813880, 0x1426cabe0, 0x143822a20, 0x1426cabec, 0x141816f80, 0x1426cae98, 0x1438228a0, 0x1426caea8, 0x1417f36e0, 0x143822920, 0x1426caec0, 0x1426caed8, 0x1417ee000, 0x141816aa0> as *const ()), // sub_141819200
    Replacement::draft(0x141819590, "physx_141819590", fn_141819590 as *const ()), // sub_141819590
    Replacement::draft(0x1418195e0, "physx_1418195e0", fn_1418195e0 as *const ()), // sub_1418195e0
    Replacement::draft(0x1418196b0, "physx_1418196b0", fn_1418196b0 as *const ()), // sub_1418196b0
    Replacement::draft(0x141819850, "physx_141819850", fn_141819850 as *const ()), // sub_141819850
    Replacement::draft(0x141819910, "physx_141819910", fn_141819910 as *const ()), // sub_141819910
    Replacement::draft(0x141819960, "physx_141819960", tpl_365::<0x1417ee690> as *const ()), // sub_141819960
    Replacement::draft(0x141819970, "physx_141819970", fn_141819970 as *const ()), // sub_141819970
    Replacement::draft(0x141819a80, "physx_141819a80", fn_141819a80 as *const ()), // sub_141819a80
    Replacement::draft(0x141819b90, "physx_141819b90", fn_141819b90 as *const ()), // sub_141819b90
    Replacement::draft(0x141819ca0, "physx_141819ca0", fn_141819ca0 as *const ()), // sub_141819ca0
    Replacement::draft(0x141819db0, "physx_141819db0", fn_141819db0 as *const ()), // sub_141819db0
    Replacement::draft(0x141819ec0, "physx_141819ec0", fn_141819ec0 as *const ()), // sub_141819ec0
    Replacement::draft(0x141819fd0, "physx_141819fd0", tpl_93754::<0x1434281d0, 0x1426cac70, 0x1426cafe0, 0x1417dd2e0, 0x1426cafe8, 0x141806ad0, 0x1426c7d90> as *const ()), // sub_141819fd0
    Replacement::draft(0x14181a130, "physx_14181a130", tpl_93754::<0x1434281d0, 0x1426cac70, 0x1426cafe0, 0x1417dd2e0, 0x1426cafe8, 0x141803de0, 0x1426c7da0> as *const ()), // sub_14181a130
    Replacement::draft(0x14181a290, "physx_14181a290", tpl_93754::<0x1434281d0, 0x1426cac70, 0x1426cafe0, 0x1417dd2e0, 0x1426cafe8, 0x141809b80, 0x1426c7db0> as *const ()), // sub_14181a290
    Replacement::draft(0x14181a3f0, "physx_14181a3f0", tpl_93754::<0x1434281d0, 0x1426cac70, 0x1426cafe0, 0x1417dd2e0, 0x1426cafe8, 0x14180ae70, 0x1426c7dc0> as *const ()), // sub_14181a3f0
    Replacement::draft(0x14181a550, "physx_14181a550", tpl_93754::<0x1434281d0, 0x1426cac70, 0x1426cafe0, 0x1417dd2e0, 0x1426cafe8, 0x14180c480, 0x1426c7dd8> as *const ()), // sub_14181a550
    Replacement::draft(0x14181a6b0, "physx_14181a6b0", tpl_93754::<0x1434281d0, 0x1426cac70, 0x1426cafe0, 0x1417dd2e0, 0x1426cafe8, 0x14180dd10, 0x1426c7de8> as *const ()), // sub_14181a6b0
    Replacement::draft(0x14181a810, "physx_14181a810", tpl_34013::<0x141819970> as *const ()), // sub_14181a810
    Replacement::draft(0x14181a830, "physx_14181a830", tpl_34013::<0x141819a80> as *const ()), // sub_14181a830
    Replacement::draft(0x14181a850, "physx_14181a850", tpl_34013::<0x141819b90> as *const ()), // sub_14181a850
    Replacement::draft(0x14181a870, "physx_14181a870", tpl_34013::<0x141819ca0> as *const ()), // sub_14181a870
    Replacement::draft(0x14181a890, "physx_14181a890", tpl_34013::<0x141819db0> as *const ()), // sub_14181a890
    Replacement::draft(0x14181a8b0, "physx_14181a8b0", tpl_34013::<0x141819ec0> as *const ()), // sub_14181a8b0
    Replacement::draft(0x14181a8d0, "physx_14181a8d0", fn_14181a8d0 as *const ()), // sub_14181a8d0
    Replacement::draft(0x14181aa50, "physx_14181aa50", fn_14181aa50 as *const ()), // sub_14181aa50
    Replacement::draft(0x14181ad30, "physx_14181ad30", fn_14181ad30 as *const ()), // sub_14181ad30
    Replacement::draft(0x14181af50, "physx_14181af50", fn_14181af50 as *const ()), // sub_14181af50
    Replacement::draft(0x14181b200, "physx_14181b200", fn_14181b200 as *const ()), // sub_14181b200
    Replacement::draft(0x14181b2d0, "physx_14181b2d0", fn_14181b2d0 as *const ()), // sub_14181b2d0
    Replacement::draft(0x14181b480, "physx_14181b480", fn_14181b480 as *const ()), // sub_14181b480
    Replacement::draft(0x14181b4f0, "physx_14181b4f0", fn_14181b4f0 as *const ()), // sub_14181b4f0
    Replacement::draft(0x14181b5a0, "physx_14181b5a0", fn_14181b5a0 as *const ()), // sub_14181b5a0
    Replacement::draft(0x14181b650, "physx_14181b650", fn_14181b650 as *const ()), // sub_14181b650
    Replacement::draft(0x14181b7b0, "physx_14181b7b0", fn_14181b7b0 as *const ()), // sub_14181b7b0
    Replacement::draft(0x14181b8a0, "physx_14181b8a0", fn_14181b8a0 as *const ()), // sub_14181b8a0
    Replacement::draft(0x14181baa0, "physx_14181baa0", fn_14181baa0 as *const ()), // sub_14181baa0
    Replacement::draft(0x14181bb20, "physx_14181bb20", fn_14181bb20 as *const ()), // sub_14181bb20
    Replacement::draft(0x14181bc00, "physx_14181bc00", fn_14181bc00 as *const ()), // sub_14181bc00
    Replacement::draft(0x14181bc30, "physx_14181bc30", fn_14181bc30 as *const ()), // sub_14181bc30
    Replacement::draft(0x14181bd10, "physx_14181bd10", fn_14181bd10 as *const ()), // sub_14181bd10
    Replacement::draft(0x14181bd40, "physx_14181bd40", fn_14181bd40 as *const ()), // sub_14181bd40
    Replacement::draft(0x14181bda0, "physx_14181bda0", tpl_34032::<0x4, 0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // sub_14181bda0
    Replacement::draft(0x14181bde0, "physx_14181bde0", fn_14181bde0 as *const ()), // sub_14181bde0
    Replacement::draft(0x14181be70, "physx_14181be70", tpl_34032::<0x4, 0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // sub_14181be70
    Replacement::draft(0x14181beb0, "physx_14181beb0", fn_14181beb0 as *const ()), // sub_14181beb0
    Replacement::draft(0x14181bf40, "physx_14181bf40", tpl_34032::<0x4, 0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // sub_14181bf40
    Replacement::draft(0x14181bf80, "physx_14181bf80", fn_14181bf80 as *const ()), // sub_14181bf80
    Replacement::draft(0x14181c010, "physx_14181c010", tpl_34036::<0x4> as *const ()), // sub_14181c010
    Replacement::draft(0x14181c030, "physx_14181c030", fn_14181c030 as *const ()), // sub_14181c030
    Replacement::draft(0x14181c090, "physx_14181c090", fn_14181c090 as *const ()), // sub_14181c090
    Replacement::draft(0x14181c0b0, "physx_14181c0b0", fn_14181c0b0 as *const ()), // sub_14181c0b0
    Replacement::draft(0x14181c160, "physx_14181c160", tpl_5167::<0x28> as *const ()), // sub_14181c160
    Replacement::draft(0x14181c170, "physx_14181c170", tpl_5167::<0x30> as *const ()), // sub_14181c170
    Replacement::draft(0x14181c180, "physx_14181c180", tpl_5167::<0x38> as *const ()), // sub_14181c180
    Replacement::draft(0x14181c190, "physx_14181c190", tpl_34040::<0x40> as *const ()), // sub_14181c190
    Replacement::draft(0x14181c1c0, "physx_14181c1c0", tpl_34041::<0x48> as *const ()), // sub_14181c1c0
    Replacement::draft(0x14181c1f0, "physx_14181c1f0", tpl_34041::<0x50> as *const ()), // sub_14181c1f0
    Replacement::draft(0x14181c220, "physx_14181c220", tpl_34041::<0x58> as *const ()), // sub_14181c220
    Replacement::draft(0x14181c250, "physx_14181c250", tpl_5167::<0x60> as *const ()), // sub_14181c250
    Replacement::draft(0x14181c260, "physx_14181c260", tpl_5167::<0x68> as *const ()), // sub_14181c260
    Replacement::draft(0x14181c270, "physx_14181c270", tpl_34042::<0x70> as *const ()), // sub_14181c270
    Replacement::draft(0x14181c2a0, "physx_14181c2a0", tpl_34041::<0x80> as *const ()), // sub_14181c2a0
    Replacement::draft(0x14181c2d0, "physx_14181c2d0", tpl_5167::<0x88> as *const ()), // sub_14181c2d0
    Replacement::draft(0x14181c2e0, "physx_14181c2e0", tpl_5167::<0x90> as *const ()), // sub_14181c2e0
    Replacement::draft(0x14181c2f0, "physx_14181c2f0", tpl_5167::<0x98> as *const ()), // sub_14181c2f0
    Replacement::draft(0x14181c300, "physx_14181c300", tpl_5167::<0xa0> as *const ()), // sub_14181c300
    Replacement::draft(0x14181c310, "physx_14181c310", tpl_5167::<0xa8> as *const ()), // sub_14181c310
    Replacement::draft(0x14181c320, "physx_14181c320", tpl_5167::<0xb0> as *const ()), // sub_14181c320
    Replacement::draft(0x14181c330, "physx_14181c330", tpl_5167::<0xb8> as *const ()), // sub_14181c330
    Replacement::draft(0x14181c340, "physx_14181c340", tpl_5167::<0xc0> as *const ()), // sub_14181c340
    Replacement::draft(0x14181c350, "physx_14181c350", tpl_5167::<0xc8> as *const ()), // sub_14181c350
    Replacement::draft(0x14181c360, "physx_14181c360", tpl_5167::<0xd0> as *const ()), // sub_14181c360
    Replacement::draft(0x14181c370, "physx_14181c370", tpl_5167::<0xd8> as *const ()), // sub_14181c370
    Replacement::draft(0x14181c380, "physx_14181c380", tpl_5167::<0xe0> as *const ()), // sub_14181c380
    Replacement::draft(0x14181c390, "physx_14181c390", tpl_5167::<0xe8> as *const ()), // sub_14181c390
    Replacement::draft(0x14181c3a0, "physx_14181c3a0", tpl_5167::<0xf0> as *const ()), // sub_14181c3a0
    Replacement::draft(0x14181c3b0, "physx_14181c3b0", tpl_5167::<0xf8> as *const ()), // sub_14181c3b0
    Replacement::draft(0x14181c3c0, "physx_14181c3c0", tpl_5167::<0x100> as *const ()), // sub_14181c3c0
    Replacement::draft(0x14181c3d0, "physx_14181c3d0", tpl_5167::<0x108> as *const ()), // sub_14181c3d0
    Replacement::draft(0x14181c3e0, "physx_14181c3e0", tpl_5167::<0x110> as *const ()), // sub_14181c3e0
    Replacement::draft(0x14181c3f0, "physx_14181c3f0", tpl_34041::<0x118> as *const ()), // sub_14181c3f0
    Replacement::draft(0x14181c420, "physx_14181c420", tpl_5167::<0x120> as *const ()), // sub_14181c420
    Replacement::draft(0x14181c430, "physx_14181c430", tpl_34041::<0x128> as *const ()), // sub_14181c430
    Replacement::draft(0x14181c460, "physx_14181c460", tpl_5167::<0x130> as *const ()), // sub_14181c460
    Replacement::draft(0x14181c470, "physx_14181c470", tpl_34041::<0x138> as *const ()), // sub_14181c470
    Replacement::draft(0x14181c4a0, "physx_14181c4a0", tpl_5167::<0x140> as *const ()), // sub_14181c4a0
    Replacement::draft(0x14181c4b0, "physx_14181c4b0", tpl_34040::<0x148> as *const ()), // sub_14181c4b0
    Replacement::draft(0x14181c4e0, "physx_14181c4e0", tpl_5167::<0x150> as *const ()), // sub_14181c4e0
    Replacement::draft(0x14181c4f0, "physx_14181c4f0", tpl_34041::<0x158> as *const ()), // sub_14181c4f0
    Replacement::draft(0x14181c520, "physx_14181c520", tpl_5167::<0x160> as *const ()), // sub_14181c520
    Replacement::draft(0x14181c530, "physx_14181c530", tpl_5167::<0x168> as *const ()), // sub_14181c530
    Replacement::draft(0x14181c540, "physx_14181c540", tpl_5167::<0x170> as *const ()), // sub_14181c540
    Replacement::draft(0x14181c550, "physx_14181c550", tpl_5167::<0x178> as *const ()), // sub_14181c550
    Replacement::draft(0x14181c560, "physx_14181c560", tpl_5167::<0x180> as *const ()), // sub_14181c560
    Replacement::draft(0x14181c570, "physx_14181c570", tpl_5167::<0x188> as *const ()), // sub_14181c570
    Replacement::draft(0x14181c580, "physx_14181c580", tpl_5167::<0x8> as *const ()), // sub_14181c580
    Replacement::draft(0x14181c590, "physx_14181c590", tpl_5167::<0xe8> as *const ()), // sub_14181c590
    Replacement::draft(0x14181c5a0, "physx_14181c5a0", tpl_5167::<0xf0> as *const ()), // sub_14181c5a0
    Replacement::draft(0x14181c5b0, "physx_14181c5b0", tpl_5167::<0xf8> as *const ()), // sub_14181c5b0
    Replacement::draft(0x14181c5c0, "physx_14181c5c0", tpl_5167::<0x100> as *const ()), // sub_14181c5c0
    Replacement::draft(0x14181c5d0, "physx_14181c5d0", tpl_5167::<0x108> as *const ()), // sub_14181c5d0
    Replacement::draft(0x14181c5e0, "physx_14181c5e0", tpl_5167::<0x110> as *const ()), // sub_14181c5e0
    Replacement::draft(0x14181c5f0, "physx_14181c5f0", tpl_5167::<0x118> as *const ()), // sub_14181c5f0
    Replacement::draft(0x14181c600, "physx_14181c600", tpl_5167::<0x120> as *const ()), // sub_14181c600
    Replacement::draft(0x14181c610, "physx_14181c610", tpl_5167::<0x128> as *const ()), // sub_14181c610
    Replacement::draft(0x14181c620, "physx_14181c620", tpl_5167::<0x130> as *const ()), // sub_14181c620
    Replacement::draft(0x14181c630, "physx_14181c630", tpl_5167::<0x138> as *const ()), // sub_14181c630
    Replacement::draft(0x14181c640, "physx_14181c640", tpl_34042::<0x140> as *const ()), // sub_14181c640
    Replacement::draft(0x14181c670, "physx_14181c670", tpl_34041::<0x150> as *const ()), // sub_14181c670
    Replacement::draft(0x14181c6a0, "physx_14181c6a0", tpl_5167::<0x8> as *const ()), // sub_14181c6a0
    Replacement::draft(0x14181c6b0, "physx_14181c6b0", tpl_5167::<0xe8> as *const ()), // sub_14181c6b0
    Replacement::draft(0x14181c6c0, "physx_14181c6c0", tpl_5167::<0xf0> as *const ()), // sub_14181c6c0
    Replacement::draft(0x14181c6d0, "physx_14181c6d0", tpl_5167::<0xf8> as *const ()), // sub_14181c6d0
    Replacement::draft(0x14181c6e0, "physx_14181c6e0", tpl_5167::<0x100> as *const ()), // sub_14181c6e0
    Replacement::draft(0x14181c6f0, "physx_14181c6f0", tpl_5167::<0x8> as *const ()), // sub_14181c6f0
    Replacement::draft(0x14181c700, "physx_14181c700", tpl_5167::<0xe8> as *const ()), // sub_14181c700
    Replacement::draft(0x14181c710, "physx_14181c710", tpl_5167::<0xf0> as *const ()), // sub_14181c710
    Replacement::draft(0x14181c720, "physx_14181c720", tpl_5167::<0xf8> as *const ()), // sub_14181c720
    Replacement::draft(0x14181c730, "physx_14181c730", tpl_34041::<0x100> as *const ()), // sub_14181c730
    Replacement::draft(0x14181c760, "physx_14181c760", tpl_34042::<0x108> as *const ()), // sub_14181c760
    Replacement::draft(0x14181c790, "physx_14181c790", tpl_34041::<0x118> as *const ()), // sub_14181c790
    Replacement::draft(0x14181c7c0, "physx_14181c7c0", tpl_5167::<0x120> as *const ()), // sub_14181c7c0
    Replacement::draft(0x14181c7d0, "physx_14181c7d0", tpl_5167::<0x128> as *const ()), // sub_14181c7d0
    Replacement::draft(0x14181c7e0, "physx_14181c7e0", tpl_5167::<0x130> as *const ()), // sub_14181c7e0
    Replacement::draft(0x14181c7f0, "physx_14181c7f0", tpl_5167::<0x138> as *const ()), // sub_14181c7f0
    Replacement::draft(0x14181c800, "physx_14181c800", tpl_5167::<0x8> as *const ()), // sub_14181c800
    Replacement::draft(0x14181c810, "physx_14181c810", tpl_5167::<0xe8> as *const ()), // sub_14181c810
    Replacement::draft(0x14181c820, "physx_14181c820", tpl_5167::<0xf0> as *const ()), // sub_14181c820
    Replacement::draft(0x14181c830, "physx_14181c830", tpl_5167::<0xf8> as *const ()), // sub_14181c830
    Replacement::draft(0x14181c840, "physx_14181c840", tpl_34041::<0x100> as *const ()), // sub_14181c840
    Replacement::draft(0x14181c870, "physx_14181c870", tpl_5167::<0x108> as *const ()), // sub_14181c870
    Replacement::draft(0x14181c880, "physx_14181c880", tpl_5167::<0x110> as *const ()), // sub_14181c880
    Replacement::draft(0x14181c890, "physx_14181c890", tpl_5167::<0x118> as *const ()), // sub_14181c890
    Replacement::draft(0x14181c8a0, "physx_14181c8a0", tpl_5167::<0x120> as *const ()), // sub_14181c8a0
    Replacement::draft(0x14181c8b0, "physx_14181c8b0", tpl_5167::<0x128> as *const ()), // sub_14181c8b0
    Replacement::draft(0x14181c8c0, "physx_14181c8c0", tpl_5167::<0x130> as *const ()), // sub_14181c8c0
    Replacement::draft(0x14181c8d0, "physx_14181c8d0", tpl_34042::<0x138> as *const ()), // sub_14181c8d0
    Replacement::draft(0x14181c900, "physx_14181c900", tpl_34041::<0x148> as *const ()), // sub_14181c900
    Replacement::draft(0x14181c930, "physx_14181c930", tpl_5167::<0x150> as *const ()), // sub_14181c930
    Replacement::draft(0x14181c940, "physx_14181c940", tpl_5167::<0x158> as *const ()), // sub_14181c940
    Replacement::draft(0x14181c950, "physx_14181c950", tpl_5167::<0x160> as *const ()), // sub_14181c950
    Replacement::draft(0x14181c960, "physx_14181c960", tpl_5167::<0x168> as *const ()), // sub_14181c960
    Replacement::draft(0x14181c970, "physx_14181c970", tpl_5167::<0x8> as *const ()), // sub_14181c970
    Replacement::draft(0x14181c980, "physx_14181c980", tpl_5167::<0xf0> as *const ()), // sub_14181c980
    Replacement::draft(0x14181c990, "physx_14181c990", tpl_34041::<0xe8> as *const ()), // sub_14181c990
    Replacement::draft(0x14181c9c0, "physx_14181c9c0", tpl_34042::<0xf8> as *const ()), // sub_14181c9c0
    Replacement::draft(0x14181c9f0, "physx_14181c9f0", tpl_34041::<0x108> as *const ()), // sub_14181c9f0
    Replacement::draft(0x14181ca20, "physx_14181ca20", tpl_5167::<0x110> as *const ()), // sub_14181ca20
    Replacement::draft(0x14181ca30, "physx_14181ca30", tpl_5167::<0x118> as *const ()), // sub_14181ca30
    Replacement::draft(0x14181ca40, "physx_14181ca40", tpl_5167::<0x8> as *const ()), // sub_14181ca40
    Replacement::draft(0x14181ca50, "physx_14181ca50", fn_14181ca50 as *const ()), // sub_14181ca50
    Replacement::draft(0x14181ca60, "physx_14181ca60", tpl_586::<0x8> as *const ()), // sub_14181ca60
    Replacement::draft(0x14181ca70, "physx_14181ca70", tpl_586::<0x18> as *const ()), // sub_14181ca70
    Replacement::draft(0x14181ca80, "physx_14181ca80", tpl_586::<0x14> as *const ()), // sub_14181ca80
    Replacement::draft(0x14181ca90, "physx_14181ca90", tpl_586::<0x4> as *const ()), // sub_14181ca90
    Replacement::draft(0x14181caa0, "physx_14181caa0", tpl_586::<0x10> as *const ()), // sub_14181caa0
    Replacement::draft(0x14181cab0, "physx_14181cab0", tpl_586::<0xc> as *const ()), // sub_14181cab0
    Replacement::draft(0x14181cac0, "physx_14181cac0", tpl_34044 as *const ()), // sub_14181cac0
    Replacement::draft(0x14181cad0, "physx_14181cad0", tpl_34045::<0xc> as *const ()), // sub_14181cad0
    Replacement::draft(0x14181cae0, "physx_14181cae0", tpl_30667::<0x18> as *const ()), // sub_14181cae0
    Replacement::draft(0x14181caf0, "physx_14181caf0", tpl_30667::<0x4> as *const ()), // sub_14181caf0
    Replacement::draft(0x14181cb00, "physx_14181cb00", tpl_30667::<0x10> as *const ()), // sub_14181cb00
    Replacement::draft(0x14181cb10, "physx_14181cb10", tpl_30667::<0xc> as *const ()), // sub_14181cb10
    Replacement::draft(0x14181cb20, "physx_14181cb20", fn_14181cb20 as *const ()), // sub_14181cb20
    Replacement::draft(0x14181cb30, "physx_14181cb30", tpl_1738::<0x10> as *const ()), // sub_14181cb30
    Replacement::draft(0x14181cb40, "physx_14181cb40", fn_14181cb40 as *const ()), // sub_14181cb40
    Replacement::draft(0x14181cdb0, "physx_14181cdb0", fn_14181cdb0 as *const ()), // sub_14181cdb0
    Replacement::draft(0x14181ce80, "physx_14181ce80", tpl_1738::<0x28> as *const ()), // sub_14181ce80
    Replacement::draft(0x14181ce90, "physx_14181ce90", fn_14181ce90 as *const ()), // sub_14181ce90
    Replacement::draft(0x14181cfa0, "physx_14181cfa0", fn_14181cfa0 as *const ()), // sub_14181cfa0
    Replacement::draft(0x14181d090, "physx_14181d090", fn_14181d090 as *const ()), // sub_14181d090
    Replacement::draft(0x14181d0a0, "physx_14181d0a0", fn_14181d0a0 as *const ()), // sub_14181d0a0
    Replacement::draft(0x14181d140, "physx_14181d140", fn_14181d140 as *const ()), // sub_14181d140
    Replacement::draft(0x14181d260, "physx_14181d260", tpl_893::<0x1426cb368, 0x141834d30> as *const ()), // sub_14181d260
    Replacement::draft(0x14181d290, "physx_14181d290", fn_14181d290 as *const ()), // sub_14181d290
    Replacement::draft(0x14181d320, "physx_14181d320", fn_14181d320 as *const ()), // sub_14181d320
    Replacement::draft(0x14181d410, "physx_14181d410", fn_14181d410 as *const ()), // sub_14181d410
    Replacement::draft(0x14181d740, "physx_14181d740", fn_14181d740 as *const ()), // sub_14181d740
    Replacement::draft(0x14181da80, "physx_14181da80", fn_14181da80 as *const ()), // sub_14181da80
    Replacement::draft(0x14181dce0, "physx_14181dce0", fn_14181dce0 as *const ()), // sub_14181dce0
    Replacement::draft(0x14181df40, "physx_14181df40", fn_14181df40 as *const ()), // sub_14181df40
    Replacement::draft(0x14181e1a0, "physx_14181e1a0", fn_14181e1a0 as *const ()), // sub_14181e1a0
    Replacement::draft(0x14181e5b0, "physx_14181e5b0", fn_14181e5b0 as *const ()), // sub_14181e5b0
    Replacement::draft(0x14181ea00, "physx_14181ea00", fn_14181ea00 as *const ()), // sub_14181ea00
    Replacement::draft(0x14181f1d0, "physx_14181f1d0", fn_14181f1d0 as *const ()), // sub_14181f1d0
    Replacement::draft(0x14181f800, "physx_14181f800", fn_14181f800 as *const ()), // sub_14181f800
    Replacement::draft(0x14181fa40, "physx_14181fa40", fn_14181fa40 as *const ()), // sub_14181fa40
    Replacement::draft(0x14181fd70, "physx_14181fd70", fn_14181fd70 as *const ()), // sub_14181fd70
    Replacement::draft(0x14181ffc0, "physx_14181ffc0", fn_14181ffc0 as *const ()), // sub_14181ffc0
    Replacement::draft(0x141820100, "physx_141820100", fn_141820100 as *const ()), // sub_141820100
    Replacement::draft(0x141820310, "physx_141820310", fn_141820310 as *const ()), // sub_141820310
    Replacement::draft(0x141820540, "physx_141820540", tpl_93805::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883538, 0x14183acaa, 0x1426c8650> as *const ()), // sub_141820540
    Replacement::draft(0x141820680, "physx_141820680", fn_141820680 as *const ()), // sub_141820680
    Replacement::draft(0x141820690, "physx_141820690", fn_141820690 as *const ()), // sub_141820690
    Replacement::draft(0x1418206c0, "physx_1418206c0", fn_1418206c0 as *const ()), // sub_1418206c0
    Replacement::draft(0x141820700, "physx_141820700", fn_141820700 as *const ()), // sub_141820700
    Replacement::draft(0x141820740, "physx_141820740", fn_141820740 as *const ()), // sub_141820740
    Replacement::draft(0x141820770, "physx_141820770", fn_141820770 as *const ()), // sub_141820770
    Replacement::draft(0x141820820, "physx_141820820", fn_141820820 as *const ()), // sub_141820820
    Replacement::draft(0x1418208c0, "physx_1418208c0", tpl_961::<0x28> as *const ()), // sub_1418208c0
    Replacement::draft(0x1418208e0, "physx_1418208e0", fn_1418208e0 as *const ()), // sub_1418208e0
    Replacement::draft(0x141820970, "physx_141820970", fn_141820970 as *const ()), // sub_141820970
    Replacement::draft(0x1418209d0, "physx_1418209d0", fn_1418209d0 as *const ()), // sub_1418209d0
    Replacement::draft(0x141820bc0, "physx_141820bc0", fn_141820bc0 as *const ()), // sub_141820bc0
    Replacement::draft(0x141820c10, "physx_141820c10", fn_141820c10 as *const ()), // sub_141820c10
    Replacement::draft(0x141820cc0, "physx_141820cc0", fn_141820cc0 as *const ()), // sub_141820cc0
    Replacement::draft(0x141820da0, "physx_141820da0", tpl_28365::<0x15c> as *const ()), // sub_141820da0
    Replacement::draft(0x141820db0, "physx_141820db0", fn_141820db0 as *const ()), // sub_141820db0
    Replacement::draft(0x1418212e0, "physx_1418212e0", fn_1418212e0 as *const ()), // sub_1418212e0
    Replacement::draft(0x141821ab0, "physx_141821ab0", fn_141821ab0 as *const ()), // sub_141821ab0
    Replacement::draft(0x141821b10, "physx_141821b10", fn_141821b10 as *const ()), // sub_141821b10
    Replacement::draft(0x141821d00, "physx_141821d00", fn_141821d00 as *const ()), // sub_141821d00
    Replacement::draft(0x1418249d0, "physx_1418249d0", fn_1418249d0 as *const ()), // sub_1418249d0
    Replacement::draft(0x141824a50, "physx_141824a50", fn_141824a50 as *const ()), // sub_141824a50
    Replacement::draft(0x141824cb0, "physx_141824cb0", fn_141824cb0 as *const ()), // sub_141824cb0
    Replacement::draft(0x141824f00, "physx_141824f00", fn_141824f00 as *const ()), // sub_141824f00
    Replacement::draft(0x141825330, "physx_141825330", fn_141825330 as *const ()), // sub_141825330
    Replacement::draft(0x141825a80, "physx_141825a80", fn_141825a80 as *const ()), // sub_141825a80
    Replacement::draft(0x141825fe0, "physx_141825fe0", fn_141825fe0 as *const ()), // sub_141825fe0
    Replacement::draft(0x141826940, "physx_141826940", fn_141826940 as *const ()), // sub_141826940
    Replacement::draft(0x141827650, "physx_141827650", fn_141827650 as *const ()), // sub_141827650
    Replacement::draft(0x141827ff0, "physx_141827ff0", fn_141827ff0 as *const ()), // sub_141827ff0
    Replacement::draft(0x141828010, "physx_141828010", fn_141828010 as *const ()), // sub_141828010
    Replacement::draft(0x1418280c0, "physx_1418280c0", fn_1418280c0 as *const ()), // sub_1418280c0
    Replacement::draft(0x141828170, "physx_141828170", fn_141828170 as *const ()), // sub_141828170
    Replacement::draft(0x141828670, "physx_141828670", fn_141828670 as *const ()), // sub_141828670
    Replacement::draft(0x141828780, "physx_141828780", fn_141828780 as *const ()), // sub_141828780
    Replacement::draft(0x1418287c0, "physx_1418287c0", fn_1418287c0 as *const ()), // sub_1418287c0
    Replacement::draft(0x141828930, "physx_141828930", fn_141828930 as *const ()), // sub_141828930
    Replacement::draft(0x14182a290, "physx_14182a290", fn_14182a290 as *const ()), // sub_14182a290
    Replacement::draft(0x14182a320, "physx_14182a320", fn_14182a320 as *const ()), // sub_14182a320
    Replacement::draft(0x14182a370, "physx_14182a370", fn_14182a370 as *const ()), // sub_14182a370
    Replacement::draft(0x14182a480, "physx_14182a480", fn_14182a480 as *const ()), // sub_14182a480
    Replacement::draft(0x14182af20, "physx_14182af20", fn_14182af20 as *const ()), // sub_14182af20
    Replacement::draft(0x14182afe0, "physx_14182afe0", fn_14182afe0 as *const ()), // sub_14182afe0
    Replacement::draft(0x14182b060, "physx_14182b060", fn_14182b060 as *const ()), // sub_14182b060
    Replacement::draft(0x14182b1c0, "physx_14182b1c0", fn_14182b1c0 as *const ()), // sub_14182b1c0
    Replacement::draft(0x14182b1e0, "physx_14182b1e0", fn_14182b1e0 as *const ()), // sub_14182b1e0
    Replacement::draft(0x14182b210, "physx_14182b210", fn_14182b210 as *const ()), // sub_14182b210
    Replacement::draft(0x14182b250, "physx_14182b250", fn_14182b250 as *const ()), // sub_14182b250
    Replacement::draft(0x14182b280, "physx_14182b280", fn_14182b280 as *const ()), // sub_14182b280
    Replacement::draft(0x14182b340, "physx_14182b340", fn_14182b340 as *const ()), // sub_14182b340
    Replacement::draft(0x14182b400, "physx_14182b400", tpl_93412::<0x142645ee8, 0x142645ed8, 0x28, 0x1426c86a0, 0x14397d1d0, 0x1438835c0, 0x14183acaa, 0x8> as *const ()), // sub_14182b400
    Replacement::draft(0x14182b4a0, "physx_14182b4a0", fn_14182b4a0 as *const ()), // sub_14182b4a0
    Replacement::draft(0x14182b4b0, "physx_14182b4b0", tpl_34105::<0x10> as *const ()), // sub_14182b4b0
    Replacement::draft(0x14182b4c0, "physx_14182b4c0", tpl_93502::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883648, 0x14183acaa, 0x1426c8650> as *const ()), // sub_14182b4c0
    Replacement::draft(0x14182b5f0, "physx_14182b5f0", fn_14182b5f0 as *const ()), // sub_14182b5f0
    Replacement::draft(0x14182b700, "physx_14182b700", fn_14182b700 as *const ()), // sub_14182b700
    Replacement::draft(0x14182b830, "physx_14182b830", fn_14182b830 as *const ()), // sub_14182b830
    Replacement::draft(0x14182b910, "physx_14182b910", fn_14182b910 as *const ()), // sub_14182b910
    Replacement::draft(0x14182b950, "physx_14182b950", tpl_895::<0x8> as *const ()), // sub_14182b950
    Replacement::draft(0x14182b970, "physx_14182b970", fn_14182b970 as *const ()), // sub_14182b970
    Replacement::draft(0x14182b9b0, "physx_14182b9b0", fn_14182b9b0 as *const ()), // sub_14182b9b0
    Replacement::draft(0x14182b9e0, "physx_14182b9e0", fn_14182b9e0 as *const ()), // sub_14182b9e0
    Replacement::draft(0x14182ba70, "physx_14182ba70", fn_14182ba70 as *const ()), // sub_14182ba70
    Replacement::draft(0x14182bab0, "physx_14182bab0", fn_14182bab0 as *const ()), // sub_14182bab0
    Replacement::draft(0x14182bc50, "physx_14182bc50", tpl_3047::<0x20> as *const ()), // sub_14182bc50
    Replacement::draft(0x14182bc60, "physx_14182bc60", fn_14182bc60 as *const ()), // sub_14182bc60
    Replacement::draft(0x14182be70, "physx_14182be70", fn_14182be70 as *const ()), // sub_14182be70
    Replacement::draft(0x14182c040, "physx_14182c040", fn_14182c040 as *const ()), // sub_14182c040
    Replacement::draft(0x14182c070, "physx_14182c070", fn_14182c070 as *const ()), // sub_14182c070
    Replacement::draft(0x14182c0a0, "physx_14182c0a0", fn_14182c0a0 as *const ()), // sub_14182c0a0
    Replacement::draft(0x14182c0d0, "physx_14182c0d0", fn_14182c0d0 as *const ()), // sub_14182c0d0
    Replacement::draft(0x14182c9f0, "physx_14182c9f0", fn_14182c9f0 as *const ()), // sub_14182c9f0
    Replacement::draft(0x14182cab0, "physx_14182cab0", fn_14182cab0 as *const ()), // sub_14182cab0
    Replacement::draft(0x14182cc20, "physx_14182cc20", fn_14182cc20 as *const ()), // sub_14182cc20
    Replacement::draft(0x14182cca0, "physx_14182cca0", fn_14182cca0 as *const ()), // sub_14182cca0
    Replacement::draft(0x14182d520, "physx_14182d520", fn_14182d520 as *const ()), // sub_14182d520
    Replacement::draft(0x14182d790, "physx_14182d790", tpl_93869::<0x14182f0f0, 0x4, 0x14182e7d0, 0x1426cb9e0, 0x14182b250, 0x14182c0d0> as *const ()), // sub_14182d790
    Replacement::draft(0x14182d850, "physx_14182d850", tpl_93869::<0x14182f0f0, 0x4, 0x14182e7d0, 0x1426cb9f8, 0x14182b250, 0x14182c0d0> as *const ()), // sub_14182d850
    Replacement::draft(0x14182d910, "physx_14182d910", fn_14182d910 as *const ()), // sub_14182d910
    Replacement::draft(0x14182da20, "physx_14182da20", fn_14182da20 as *const ()), // sub_14182da20
    Replacement::draft(0x14182dc90, "physx_14182dc90", fn_14182dc90 as *const ()), // sub_14182dc90
    Replacement::draft(0x14182dcc0, "physx_14182dcc0", fn_14182dcc0 as *const ()), // sub_14182dcc0
    Replacement::draft(0x14182dd40, "physx_14182dd40", tpl_365::<0x14182e7d0> as *const ()), // sub_14182dd40
    Replacement::draft(0x14182dd50, "physx_14182dd50", tpl_365::<0x14182e840> as *const ()), // sub_14182dd50
    Replacement::draft(0x14182dd60, "physx_14182dd60", tpl_93805::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x143883698, 0x14183acaa, 0x1426c8650> as *const ()), // sub_14182dd60
    Replacement::draft(0x14182dea0, "physx_14182dea0", tpl_93540::<0x142645ee8, 0x142645ed8, 0x1426c86a0, 0x14397d1d0, 0x14388f038, 0x14183acaa, 0x1426c8650> as *const ()), // sub_14182dea0
    Replacement::draft(0x14182dfd0, "physx_14182dfd0", fn_14182dfd0 as *const ()), // sub_14182dfd0
    Replacement::draft(0x14182e070, "physx_14182e070", fn_14182e070 as *const ()), // sub_14182e070
    Replacement::draft(0x14182e190, "physx_14182e190", fn_14182e190 as *const ()), // sub_14182e190
    Replacement::draft(0x14182e280, "physx_14182e280", fn_14182e280 as *const ()), // sub_14182e280
    Replacement::draft(0x14182e320, "physx_14182e320", fn_14182e320 as *const ()), // sub_14182e320
    Replacement::draft(0x14182e3a0, "physx_14182e3a0", fn_14182e3a0 as *const ()), // sub_14182e3a0
    Replacement::draft(0x14182e440, "physx_14182e440", fn_14182e440 as *const ()), // sub_14182e440
    Replacement::draft(0x14182e5f0, "physx_14182e5f0", fn_14182e5f0 as *const ()), // sub_14182e5f0
    Replacement::draft(0x14182e6b0, "physx_14182e6b0", tpl_30363::<0x150> as *const ()), // sub_14182e6b0
    Replacement::draft(0x14182e6c0, "physx_14182e6c0", tpl_30363::<0x151> as *const ()), // sub_14182e6c0
    Replacement::draft(0x14182e6d0, "physx_14182e6d0", fn_14182e6d0 as *const ()), // sub_14182e6d0
    Replacement::draft(0x14182e740, "physx_14182e740", tpl_23750::<0x138> as *const ()), // sub_14182e740
    Replacement::draft(0x14182e760, "physx_14182e760", tpl_1691::<0x144> as *const ()), // sub_14182e760
    Replacement::draft(0x14182e770, "physx_14182e770", fn_14182e770 as *const ()), // sub_14182e770
    Replacement::draft(0x14182e7d0, "physx_14182e7d0", fn_14182e7d0 as *const ()), // sub_14182e7d0
    Replacement::draft(0x14182e840, "physx_14182e840", fn_14182e840 as *const ()), // sub_14182e840
    Replacement::draft(0x14182e8e0, "physx_14182e8e0", fn_14182e8e0 as *const ()), // sub_14182e8e0
    Replacement::draft(0x14182e920, "physx_14182e920", fn_14182e920 as *const ()), // sub_14182e920
    Replacement::draft(0x14182e960, "physx_14182e960", fn_14182e960 as *const ()), // sub_14182e960
    Replacement::draft(0x14182e9d0, "physx_14182e9d0", fn_14182e9d0 as *const ()), // sub_14182e9d0
    Replacement::draft(0x14182ea70, "physx_14182ea70", fn_14182ea70 as *const ()), // sub_14182ea70
    Replacement::draft(0x14182eb20, "physx_14182eb20", fn_14182eb20 as *const ()), // sub_14182eb20
    Replacement::draft(0x14182ec10, "physx_14182ec10", fn_14182ec10 as *const ()), // sub_14182ec10
    Replacement::draft(0x14182ecc0, "physx_14182ecc0", fn_14182ecc0 as *const ()), // sub_14182ecc0
    Replacement::draft(0x14182efa0, "physx_14182efa0", tpl_30883::<0x154> as *const ()), // sub_14182efa0
    Replacement::draft(0x14182efb0, "physx_14182efb0", tpl_1691::<0x154> as *const ()), // sub_14182efb0
    Replacement::draft(0x14182efc0, "physx_14182efc0", fn_14182efc0 as *const ()), // sub_14182efc0
    Replacement::draft(0x14182f0f0, "unknown_14182f0f0", fn_14182f0f0 as *const ()), // sub_14182f0f0
    Replacement::draft(0x14182f170, "unknown_14182f170", fn_14182f170 as *const ()), // sub_14182f170
    Replacement::draft(0x14182f1b3, "crt_14182f1b3", tpl_906::<0x142645e50> as *const ()), // j_physx::PxShapeGeometryPropertyHelper::j_physx::PxShapeGeometryPropertyHelper::getGeometry
    Replacement::draft(0x14182f1b9, "crt_14182f1b9", tpl_906::<0x142645e88> as *const ()), // j_physx::PxShapeGeometryPropertyHelper::j_physx::PxShapeGeometryPropertyHelper::getGeometry
    Replacement::draft(0x14182f1bf, "crt_14182f1bf", tpl_906::<0x142645e90> as *const ()), // j_physx::PxShapeGeometryPropertyHelper::j_physx::PxShapeGeometryPropertyHelper::getGeometry
    Replacement::draft(0x14182f1c5, "crt_14182f1c5", tpl_906::<0x142645e98> as *const ()), // j_physx::PxShapeGeometryPropertyHelper::j_physx::PxShapeGeometryPropertyHelper::getGeometry
    Replacement::draft(0x14182f1cb, "crt_14182f1cb", tpl_906::<0x142645ea0> as *const ()), // j_physx::PxShapeGeometryPropertyHelper::j_physx::PxShapeGeometryPropertyHelper::getGeometry
    Replacement::draft(0x14182f1d1, "crt_14182f1d1", tpl_906::<0x142645ea8> as *const ()), // j_physx::PxShapeGeometryPropertyHelper::j_physx::PxShapeGeometryPropertyHelper::getGeometry
    Replacement::draft(0x14182f1d7, "crt_14182f1d7", tpl_906::<0x142645dc0> as *const ()), // j_physx::PxShapeGeometryPropertyHelper::j_physx::PxShapeGeometryPropertyHelper::getGeometry
    Replacement::draft(0x14182f1dd, "crt_14182f1dd", tpl_906::<0x142645db8> as *const ()), // j_physx::PxMaterialGeneratedInfo::j_physx::PxMaterialGeneratedInfo::PxMaterialGeneratedInfo
    Replacement::draft(0x14182f1e3, "crt_14182f1e3", tpl_906::<0x142645d80> as *const ()), // j_physx::PxRigidDynamicGeneratedInfo::j_physx::PxRigidDynamicGeneratedInfo::PxRigidDynamicGeneratedInfo
    Replacement::draft(0x14182f1e9, "crt_14182f1e9", tpl_906::<0x142645da8> as *const ()), // j_physx::PxRigidStaticGeneratedInfo::j_physx::PxRigidStaticGeneratedInfo::PxRigidStaticGeneratedInfo
    Replacement::draft(0x14182f1ef, "crt_14182f1ef", tpl_906::<0x142645e20> as *const ()), // j_physx::PxArticulationLinkGeneratedInfo::j_physx::PxArticulationLinkGeneratedInfo::PxArticulationLinkGeneratedInfo
    Replacement::draft(0x14182f1f5, "crt_14182f1f5", tpl_906::<0x142645eb0> as *const ()), // j_physx::PxArticulationJointGeneratedInfo::j_physx::PxArticulationJointGeneratedInfo::PxArticulationJointGeneratedInfo
    Replacement::draft(0x14182f1fb, "crt_14182f1fb", tpl_906::<0x142645de8> as *const ()), // j_physx::PxArticulationGeneratedInfo::j_physx::PxArticulationGeneratedInfo::PxArticulationGeneratedInfo
    Replacement::draft(0x14182f201, "crt_14182f201", tpl_906::<0x142645df8> as *const ()), // j_physx::PxAggregateGeneratedInfo::j_physx::PxAggregateGeneratedInfo::PxAggregateGeneratedInfo
    Replacement::draft(0x14182f207, "crt_14182f207", tpl_906::<0x142645d70> as *const ()), // j_physx::PxShapeGeneratedInfo::j_physx::PxShapeGeneratedInfo::PxShapeGeneratedInfo
    Replacement::draft(0x14182f20d, "crt_14182f20d", tpl_906::<0x142645d78> as *const ()), // j_physx::PxClothFabricGeneratedInfo::j_physx::PxClothFabricGeneratedInfo::PxClothFabricGeneratedInfo
    Replacement::draft(0x14182f213, "crt_14182f213", tpl_906::<0x142645e80> as *const ()), // j_physx::PxClothGeneratedInfo::j_physx::PxClothGeneratedInfo::PxClothGeneratedInfo
    Replacement::draft(0x14182f219, "crt_14182f219", tpl_906::<0x142645e78> as *const ()), // j_physx::PxParticleFluidGeneratedInfo::j_physx::PxParticleFluidGeneratedInfo::PxParticleFluidGeneratedInfo
    Replacement::draft(0x14182f21f, "crt_14182f21f", tpl_906::<0x142645e70> as *const ()), // j_physx::PxParticleSystemGeneratedInfo::j_physx::PxParticleSystemGeneratedInfo::PxParticleSystemGeneratedInfo
    Replacement::draft(0x14182f225, "crt_14182f225", tpl_906::<0x142645e68> as *const ()), // j_physx::PxBoxGeometryGeneratedInfo::j_physx::PxBoxGeometryGeneratedInfo::PxBoxGeometryGeneratedInfo
    Replacement::draft(0x14182f22b, "crt_14182f22b", tpl_906::<0x142645e60> as *const ()), // j_physx::PxCapsuleGeometryGeneratedInfo::j_physx::PxCapsuleGeometryGeneratedInfo::PxCapsuleGeometryGeneratedInfo
    Replacement::draft(0x14182f231, "crt_14182f231", tpl_906::<0x142645e58> as *const ()), // j_physx::PxMeshScaleGeneratedInfo::j_physx::PxMeshScaleGeneratedInfo::PxMeshScaleGeneratedInfo
    Replacement::draft(0x14182f237, "crt_14182f237", tpl_906::<0x142645e30> as *const ()), // j_physx::PxConvexMeshGeometryGeneratedInfo::j_physx::PxConvexMeshGeometryGeneratedInfo::PxConvexMeshGeometryGeneratedInf
    Replacement::draft(0x14182f23d, "crt_14182f23d", tpl_906::<0x142645e18> as *const ()), // j_physx::PxSphereGeometryGeneratedInfo::j_physx::PxSphereGeometryGeneratedInfo::PxSphereGeometryGeneratedInfo
    Replacement::draft(0x14182f243, "crt_14182f243", tpl_906::<0x142645e10> as *const ()), // j_physx::PxPlaneGeometryGeneratedInfo::j_physx::PxPlaneGeometryGeneratedInfo::PxPlaneGeometryGeneratedInfo
    Replacement::draft(0x14182f249, "crt_14182f249", tpl_906::<0x142645e08> as *const ()), // j_physx::PxTriangleMeshGeometryGeneratedInfo::j_physx::PxTriangleMeshGeometryGeneratedInfo::PxTriangleMeshGeometryGenera
    Replacement::draft(0x14182f24f, "crt_14182f24f", tpl_906::<0x142645e00> as *const ()), // j_physx::PxHeightFieldGeometryGeneratedInfo::j_physx::PxHeightFieldGeometryGeneratedInfo::PxHeightFieldGeometryGenerated
    Replacement::draft(0x14182f255, "crt_14182f255", tpl_906::<0x142645dd8> as *const ()), // j_physx::PxClothStretchConfigGeneratedInfo::j_physx::PxClothStretchConfigGeneratedInfo::PxClothStretchConfigGeneratedInf
    Replacement::draft(0x14182f25b, "crt_14182f25b", tpl_906::<0x142645dd0> as *const ()), // j_physx::PxClothTetherConfigGeneratedInfo::j_physx::PxClothTetherConfigGeneratedInfo::PxClothTetherConfigGeneratedInfo
    Replacement::draft(0x14182f261, "crt_14182f261", tpl_906::<0x142645db0> as *const ()), // j_physx::PxClothMotionConstraintConfigGeneratedInfo::j_physx::PxClothMotionConstraintConfigGeneratedInfo::PxClothMotionC
    Replacement::draft(0x14182f267, "crt_14182f267", tpl_906::<0x142645df0> as *const ()), // j_PxGetPhysicsBinaryMetaData
    Replacement::draft(0x14182f26d, "crt_14182f26d", tpl_906::<0x142645da0> as *const ()), // j_PxRegisterPhysicsSerializers
    Replacement::draft(0x14182f273, "crt_14182f273", tpl_906::<0x142645d98> as *const ()), // j_PxUnregisterPhysicsSerializers
    Replacement::draft(0x14182f279, "crt_14182f279", tpl_906::<0x142645d90> as *const ()), // j_PxAddCollectionToPhysics
    Replacement::draft(0x14182f27f, "crt_14182f27f", tpl_906::<0x142645d88> as *const ()), // j_physx::PxHeightFieldDescGeneratedInfo::j_physx::PxHeightFieldDescGeneratedInfo::PxHeightFieldDescGeneratedInfo
    Replacement::draft(0x14182f290, "unknown_14182f290", fn_14182f290 as *const ()), // sub_14182f290
    Replacement::draft(0x14182f3a0, "unknown_14182f3a0", fn_14182f3a0 as *const ()), // sub_14182f3a0
    Replacement::draft_throws(0x14182f490, "unknown_14182f490", fn_14182f490 as *const (), Throws::Discard), // sub_14182f490
    Replacement::draft(0x14182ffb0, "unknown_14182ffb0", fn_14182ffb0 as *const ()), // sub_14182ffb0
    Replacement::draft_throws(0x141830400, "unknown_141830400", fn_141830400 as *const (), Throws::Discard), // sub_141830400
    Replacement::draft(0x141830580, "unknown_141830580", fn_141830580 as *const ()), // sub_141830580
    Replacement::draft(0x141830b60, "unknown_141830b60", fn_141830b60 as *const ()), // sub_141830b60
    Replacement::draft(0x141830bf0, "unknown_141830bf0", fn_141830bf0 as *const ()), // sub_141830bf0
    Replacement::draft_throws(0x141830e30, "unknown_141830e30", fn_141830e30 as *const (), Throws::Discard), // sub_141830e30
    Replacement::draft_throws(0x141830fa0, "unknown_141830fa0", fn_141830fa0 as *const (), Throws::Discard), // sub_141830fa0
    Replacement::draft(0x1418310f0, "unknown_1418310f0", fn_1418310f0 as *const ()), // sub_1418310f0
    Replacement::draft(0x141831190, "unknown_141831190", fn_141831190 as *const ()), // sub_141831190
    Replacement::draft(0x1418312e0, "unknown_1418312e0", fn_1418312e0 as *const ()), // sub_1418312e0
    Replacement::draft(0x141831320, "unknown_141831320", tpl_23741::<0x1418310f0, 0x141834d30> as *const ()), // sub_141831320
    Replacement::draft(0x141831360, "concurrency_141831360", fn_141831360 as *const ()), // Concurrency::details::UMSFreeThreadProxyFactory::Concurrency::details::UMSFreeThreadProxyFactory::`scalar_deleting_destr
    Replacement::draft_throws(0x1418313b0, "unknown_1418313b0", fn_1418313b0 as *const (), Throws::Discard), // sub_1418313b0
    Replacement::draft(0x141831530, "unknown_141831530", fn_141831530 as *const ()), // sub_141831530
    Replacement::draft_throws(0x1418315c0, "unknown_1418315c0", fn_1418315c0 as *const (), Throws::Discard), // sub_1418315c0
    Replacement::draft_throws(0x141831690, "unknown_141831690", fn_141831690 as *const (), Throws::Discard), // sub_141831690
    Replacement::draft_throws(0x1418319a0, "unknown_1418319a0", fn_1418319a0 as *const (), Throws::Discard), // sub_1418319a0
    Replacement::draft(0x141831e00, "std_141831e00", fn_141831e00 as *const ()), // sub_141831e00
    Replacement::draft(0x141831f60, "std_141831f60", fn_141831f60 as *const ()), // sub_141831f60
    Replacement::draft(0x1418320d0, "std_1418320d0", fn_1418320d0 as *const ()), // sub_1418320d0
    Replacement::draft_throws(0x141832200, "std_141832200", fn_141832200 as *const (), Throws::Discard), // sub_141832200
    Replacement::draft(0x1418325d0, "unknown_1418325d0", tpl_26622::<0x1426711d8, 0x141834d30> as *const ()), // sub_1418325d0
    Replacement::draft(0x141832600, "unknown_141832600", fn_141832600 as *const ()), // sub_141832600
    Replacement::draft(0x141832650, "unknown_141832650", fn_141832650 as *const ()), // sub_141832650
    Replacement::draft(0x141832690, "unknown_141832690", fn_141832690 as *const ()), // sub_141832690
    Replacement::draft(0x1418328f0, "unknown_1418328f0", fn_1418328f0 as *const ()), // sub_1418328f0
    Replacement::draft(0x141832c30, "unknown_141832c30", fn_141832c30 as *const ()), // sub_141832c30
    Replacement::draft(0x141832c50, "unknown_141832c50", fn_141832c50 as *const ()), // sub_141832c50
    Replacement::draft(0x141832c80, "unknown_141832c80", fn_141832c80 as *const ()), // sub_141832c80
    Replacement::draft(0x141832da0, "unknown_141832da0", tpl_28917::<0x1426ccc58, 0x8> as *const ()), // sub_141832da0
    Replacement::draft(0x141832dc0, "unknown_141832dc0", fn_141832dc0 as *const ()), // sub_141832dc0
    Replacement::draft(0x141832e30, "unknown_141832e30", fn_141832e30 as *const ()), // sub_141832e30
    Replacement::draft(0x141832e60, "unknown_141832e60", fn_141832e60 as *const ()), // sub_141832e60
    Replacement::draft(0x141832f70, "unknown_141832f70", fn_141832f70 as *const ()), // sub_141832f70
    Replacement::draft_throws(0x141832fc0, "unknown_141832fc0", fn_141832fc0 as *const (), Throws::Discard), // sub_141832fc0
    Replacement::draft(0x1418331f0, "unknown_1418331f0", fn_1418331f0 as *const ()), // sub_1418331f0
    Replacement::draft(0x1418332a0, "concurrency_1418332a0", fn_1418332a0 as *const ()), // Concurrency::details::UMSFreeThreadProxyFactory::Concurrency::details::UMSFreeThreadProxyFactory::`scalar_deleting_destr
    Replacement::draft(0x1418332f0, "unknown_1418332f0", fn_1418332f0 as *const ()), // sub_1418332f0
    Replacement::draft(0x141833350, "unknown_141833350", fn_141833350 as *const ()), // sub_141833350
    Replacement::draft(0x141833390, "unknown_141833390", fn_141833390 as *const ()), // sub_141833390
    Replacement::draft_throws(0x1418335d0, "unknown_1418335d0", fn_1418335d0 as *const (), Throws::Discard), // sub_1418335d0
    Replacement::draft_throws(0x141833710, "std_141833710", tpl_26811::<0x1430806e0, 0x141833c0f, 0x141833720> as *const (), Throws::Discard), // sub_141833710
    Replacement::draft(0x141833730, "unknown_141833730", tpl_28917::<0x1426ccda8, 0x8> as *const ()), // sub_141833730
    Replacement::draft(0x141833750, "unknown_141833750", fn_141833750 as *const ()), // sub_141833750
    Replacement::draft(0x1418337b0, "unknown_1418337b0", fn_1418337b0 as *const ()), // sub_1418337b0
    Replacement::draft(0x1418337f0, "unknown_1418337f0", fn_1418337f0 as *const ()), // sub_1418337f0
    Replacement::draft(0x141833860, "unknown_141833860", fn_141833860 as *const ()), // sub_141833860
    Replacement::draft(0x141833890, "unknown_141833890", fn_141833890 as *const ()), // sub_141833890
    Replacement::draft(0x1418338c0, "unknown_1418338c0", fn_1418338c0 as *const ()), // sub_1418338c0
    Replacement::draft(0x1418339c0, "unknown_1418339c0", fn_1418339c0 as *const ()), // sub_1418339c0
    Replacement::draft(0x141833ab0, "unknown_141833ab0", fn_141833ab0 as *const ()), // sub_141833ab0
    Replacement::draft(0x141833ad0, "unknown_141833ad0", fn_141833ad0 as *const ()), // sub_141833ad0
    Replacement::draft(0x141833af0, "unknown_141833af0", fn_141833af0 as *const ()), // sub_141833af0
    Replacement::draft(0x141833b30, "unknown_141833b30", tpl_34179::<0x64, 0x68, 0x60> as *const ()), // sub_141833b30
    Replacement::draft(0x141833b40, "unknown_141833b40", tpl_34179::<0x64, 0x6c, 0x60> as *const ()), // sub_141833b40
    Replacement::draft(0x141833b50, "unknown_141833b50", fn_141833b50 as *const ()), // sub_141833b50
    Replacement::draft(0x141833bfd, "crt_141833bfd", tpl_906::<0x142645888> as *const ()), // j__Query_perf_frequency
    Replacement::draft(0x141833c03, "crt_141833c03", tpl_906::<0x142645880> as *const ()), // j__Query_perf_counter
    Replacement::draft_throws(0x141833c09, "crt_141833c09", tpl_34181::<0x142645878> as *const (), Throws::Discard), // j_std::j_std::_Xbad_function_call
    Replacement::draft_throws(0x141833c0f, "crt_141833c0f", tpl_34181::<0x142645870> as *const (), Throws::Discard), // j_std::j_std::_Xlength_error
    Replacement::draft(0x141833c15, "crt_141833c15", tpl_906::<0x142645868> as *const ()), // j__Thrd_yield
    Replacement::draft_throws(0x141833c1b, "crt_141833c1b", tpl_34181::<0x142645840> as *const (), Throws::Discard), // j_std::j_std::_Throw_Cpp_error
    Replacement::draft_throws(0x141833c21, "crt_141833c21", tpl_34181::<0x142645838> as *const (), Throws::Discard), // j_std::j_std::_Xbad_alloc
    Replacement::draft(0x141833c27, "crt_141833c27", tpl_906::<0x142645830> as *const ()), // j__Mtx_destroy_in_situ
    Replacement::draft(0x141833c2d, "crt_141833c2d", tpl_906::<0x142645828> as *const ()), // j__Mtx_lock
    Replacement::draft(0x141833c33, "crt_141833c33", tpl_906::<0x142645820> as *const ()), // j__Mtx_init_in_situ
    Replacement::draft(0x141833c3c, "crt_141833c3c", tpl_906::<0x142645618> as *const ()), // j_Sleep
    Replacement::draft(0x141833c43, "crt_141833c43", tpl_906::<0x142645a50> as *const ()), // j__Xtime_get_ticks
    Replacement::draft(0x141833c49, "crt_141833c49", tpl_906::<0x142645a78> as *const ()), // j__Mtx_unlock
    Replacement::draft(0x141833c4f, "crt_141833c4f", tpl_906::<0x142645a68> as *const ()), // j__Cnd_do_broadcast_at_thread_exit
    Replacement::draft(0x141833c55, "crt_141833c55", tpl_906::<0x1426459f8> as *const ()), // j__Thrd_detach
    Replacement::draft(0x141833c5b, "crt_141833c5b", tpl_906::<0x142645940> as *const ()), // j_std::j_std::_Random_device
    Replacement::draft(0x141833c70, "crt_141833c70", fn_141833c70 as *const ()), // sub_141833c70
    Replacement::draft(0x141833e90, "crt_141833e90", fn_141833e90 as *const ()), // sub_141833e90
    Replacement::draft(0x141833f90, "crt_141833f90", fn_141833f90 as *const ()), // sub_141833f90
    Replacement::draft(0x141834060, "crt_141834060", fn_141834060 as *const ()), // sub_141834060
    Replacement::draft(0x141834130, "crt_141834130", fn_141834130 as *const ()), // sub_141834130
    Replacement::draft(0x1418341e0, "crt_1418341e0", fn_1418341e0 as *const ()), // sub_1418341e0
    Replacement::draft(0x1418342b0, "crt_1418342b0", tpl_365::<0x141833c70> as *const ()), // sub_1418342b0
    Replacement::draft(0x1418342c0, "crt_1418342c0", fn_1418342c0 as *const ()), // sub_1418342c0
    Replacement::draft(0x1418343b4, "crt_1418343b4", tpl_906::<0x1426458c8> as *const ()), // j_std::locale::j_std::locale::_Init
    Replacement::draft_throws(0x1418343bc, "std_1418343bc", fn_1418343bc as *const (), Throws::Discard), // std::std::_Facet_Register
    Replacement::draft(0x1418343fa, "crt_1418343fa", tpl_906::<0x142645918> as *const ()), // j_std::locale::facet::j_std::locale::facet::_Decref
    Replacement::draft(0x141834400, "crt_141834400", tpl_906::<0x142645920> as *const ()), // j_std::locale::facet::j_std::locale::facet::_Incref
    Replacement::draft(0x141834410, "crt_141834410", tpl_906::<0x1426459b0> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::~basic_streamb
    Replacement::draft(0x141834416, "crt_141834416", tpl_906::<0x1426459b8> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::_Lock
    Replacement::draft(0x14183441c, "crt_14183441c", tpl_906::<0x1426459c0> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::_Unlock
    Replacement::draft(0x141834422, "crt_141834422", tpl_906::<0x1426459c8> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::showmanyc
    Replacement::draft(0x141834428, "crt_141834428", tpl_906::<0x1426459d0> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::uflow
    Replacement::draft(0x14183442e, "crt_14183442e", tpl_906::<0x142645a90> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::xsgetn
    Replacement::draft(0x141834434, "crt_141834434", tpl_906::<0x142645a98> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::xsputn
    Replacement::draft(0x14183443a, "crt_14183443a", tpl_906::<0x142645aa0> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::setbuf
    Replacement::draft(0x141834440, "crt_141834440", tpl_906::<0x142645ab0> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::sync
    Replacement::draft(0x141834446, "crt_141834446", tpl_906::<0x142645ab8> as *const ()), // j_std::basic_streambuf<char,std::char_traits<char>>::j_std::basic_streambuf<char,std::char_traits<char>>::imbue
    Replacement::draft(0x14183444c, "crt_14183444c", tpl_906::<0x142645ad8> as *const ()), // j_std::j_std::_Syserror_map
    Replacement::draft(0x141834454, "crt_141834454", fn_141834454 as *const ()), // sub_141834454
    Replacement::draft(0x1418344ec, "crt_1418344ec", tpl_906::<0x1426456f0> as *const ()), // j_LocalFree
    Replacement::draft(0x1418344f3, "crt_1418344f3", tpl_906::<0x142645af0> as *const ()), // j__Thrd_id
    Replacement::draft(0x1418344f9, "crt_1418344f9", tpl_906::<0x142645af8> as *const ()), // j__Thrd_join
    Replacement::draft(0x141834500, "crt_141834500", fn_141834500 as *const ()), // sub_141834500
    Replacement::draft(0x141834828, "global_141834828", fn_141834828 as *const ()), // __std_fs_open_handle
    Replacement::draft(0x14183487f, "crt_14183487f", tpl_906::<0x142645be0> as *const ()), // j_std::j_std::uncaught_exceptions
    Replacement::draft(0x141834885, "crt_141834885", tpl_906::<0x142645c00> as *const ()), // j__Mtx_current_owns
    Replacement::draft(0x14183488b, "crt_14183488b", tpl_906::<0x142645c08> as *const ()), // j__Cnd_init_in_situ
    Replacement::draft(0x141834891, "crt_141834891", tpl_906::<0x142645c10> as *const ()), // j__Cnd_destroy_in_situ
    Replacement::draft(0x141834897, "crt_141834897", tpl_906::<0x142645c18> as *const ()), // j__Cnd_timedwait
    Replacement::draft(0x14183489d, "crt_14183489d", tpl_906::<0x142645c38> as *const ()), // j__Cnd_broadcast
    Replacement::draft(0x1418348a3, "crt_1418348a3", tpl_906::<0x142645c30> as *const ()), // j_std::j_std::_Throw_C_error
    Replacement::draft(0x1418348a9, "crt_1418348a9", tpl_906::<0x142645b80> as *const ()), // j___ExceptionPtrCreate
    Replacement::draft(0x1418348af, "crt_1418348af", tpl_906::<0x142645b78> as *const ()), // j___ExceptionPtrDestroy
    Replacement::draft(0x1418348b5, "crt_1418348b5", tpl_906::<0x142645b70> as *const ()), // j___ExceptionPtrCopy
    Replacement::draft(0x1418348bb, "crt_1418348bb", tpl_906::<0x142645b68> as *const ()), // j___ExceptionPtrAssign
    Replacement::draft(0x1418348c1, "crt_1418348c1", tpl_906::<0x142645b60> as *const ()), // j___ExceptionPtrToBool
    Replacement::draft(0x1418348c7, "crt_1418348c7", tpl_906::<0x142645b58> as *const ()), // j___ExceptionPtrRethrow
    Replacement::draft(0x1418348cd, "crt_1418348cd", tpl_906::<0x142645b50> as *const ()), // j___ExceptionPtrCopyException
    Replacement::draft(0x1418348d3, "crt_1418348d3", tpl_906::<0x142645b48> as *const ()), // j__Cnd_wait
    Replacement::draft(0x1418348d9, "crt_1418348d9", tpl_906::<0x142645b40> as *const ()), // j__Cnd_register_at_thread_exit
    Replacement::draft(0x1418348df, "crt_1418348df", tpl_906::<0x142645b38> as *const ()), // j__Cnd_unregister_at_thread_exit
    Replacement::draft(0x1418348e5, "crt_1418348e5", tpl_906::<0x142645b10> as *const ()), // j__Cnd_signal
    Replacement::draft(0x1418348eb, "crt_1418348eb", tpl_906::<0x142645b08> as *const ()), // j___ExceptionPtrCurrentException
    Replacement::draft(0x1418348f1, "crt_1418348f1", tpl_906::<0x142645ae8> as *const ()), // j_Concurrency::details::j_Concurrency::details::_Schedule_chore
    Replacement::draft(0x1418348f7, "crt_1418348f7", tpl_906::<0x142645aa8> as *const ()), // j_Concurrency::details::j_Concurrency::details::_Release_chore
    Replacement::draft(0x1418348fd, "crt_1418348fd", tpl_906::<0x142645a88> as *const ()), // j_Concurrency::details::j_Concurrency::details::_ReportUnobservedException
    Replacement::draft(0x141834903, "crt_141834903", tpl_906::<0x142645a48> as *const ()), // j_Concurrency::details::platform::j_Concurrency::details::platform::GetCurrentThreadId
    Replacement::draft(0x141834909, "crt_141834909", tpl_906::<0x142645a40> as *const ()), // j_Concurrency::details::_ContextCallback::j_Concurrency::details::_ContextCallback::_CallInContext
    Replacement::draft(0x14183490f, "crt_14183490f", tpl_906::<0x142645a38> as *const ()), // j_Concurrency::details::_ContextCallback::j_Concurrency::details::_ContextCallback::_Reset
    Replacement::draft(0x141834915, "crt_141834915", tpl_906::<0x142645a30> as *const ()), // j_Concurrency::details::_ContextCallback::j_Concurrency::details::_ContextCallback::_IsCurrentOriginSTA
    Replacement::draft(0x14183491b, "crt_14183491b", tpl_906::<0x142645a28> as *const ()), // j_Concurrency::details::_ContextCallback::j_Concurrency::details::_ContextCallback::_Capture
    Replacement::draft(0x141834921, "crt_141834921", tpl_906::<0x142645a20> as *const ()), // j_Concurrency::details::_ExceptionHolder::j_Concurrency::details::_ExceptionHolder::ReportUnhandledError
    Replacement::draft(0x141834927, "crt_141834927", tpl_906::<0x142645a10> as *const ()), // j_Concurrency::task_continuation_context::j_Concurrency::task_continuation_context::task_continuation_context
    Replacement::draft(0x14183492d, "crt_14183492d", tpl_906::<0x142645a08> as *const ()), // j_Concurrency::details::_TaskEventLogger::j_Concurrency::details::_TaskEventLogger::_LogScheduleTask
    Replacement::draft(0x141834933, "crt_141834933", tpl_906::<0x142645a00> as *const ()), // j_Concurrency::details::_TaskEventLogger::j_Concurrency::details::_TaskEventLogger::_LogCancelTask
    Replacement::draft(0x141834939, "crt_141834939", tpl_906::<0x1426459f0> as *const ()), // j_Concurrency::details::_TaskEventLogger::j_Concurrency::details::_TaskEventLogger::_LogTaskCompleted
    Replacement::draft(0x14183493f, "crt_14183493f", tpl_906::<0x1426459e8> as *const ()), // j_Concurrency::details::_TaskEventLogger::j_Concurrency::details::_TaskEventLogger::_LogTaskExecutionCompleted
    Replacement::draft(0x141834945, "crt_141834945", tpl_906::<0x1426459e0> as *const ()), // j_Concurrency::details::_TaskEventLogger::j_Concurrency::details::_TaskEventLogger::_LogWorkItemStarted
    Replacement::draft(0x14183494b, "crt_14183494b", tpl_906::<0x1426459d8> as *const ()), // j_Concurrency::details::_TaskEventLogger::j_Concurrency::details::_TaskEventLogger::_LogWorkItemCompleted
    Replacement::draft(0x141834951, "crt_141834951", tpl_906::<0x142645978> as *const ()), // j__Mtx_trylock
    Replacement::draft(0x141834958, "global_141834958", fn_141834958 as *const ()), // tls_callback_0
    Replacement::draft(0x1418349c0, "global_1418349c0", fn_1418349c0 as *const ()), // __dyn_tls_on_demand_init
    Replacement::draft(0x1418349d0, "crt_1418349d0", fn_1418349d0 as *const ()), // sub_1418349d0
    Replacement::draft(0x141834a0c, "global_141834a0c", fn_141834a0c as *const ()), // _Init_thread_footer
    Replacement::draft(0x141834a78, "crt_141834a78", fn_141834a78 as *const ()), // sub_141834a78
    Replacement::draft(0x141834af0, "global_141834af0", fn_141834af0 as *const ()), // __scrt_acquire_startup_lock
    Replacement::draft(0x141834b2c, "crt_141834b2c", fn_141834b2c as *const ()), // sub_141834b2c
    Replacement::draft(0x141834b68, "global_141834b68", fn_141834b68 as *const ()), // __scrt_initialize_onexit_tables
    Replacement::draft(0x141834bf4, "crt_141834bf4", fn_141834bf4 as *const ()), // sub_141834bf4
    Replacement::draft(0x141834c8c, "global_141834c8c", fn_141834c8c as *const ()), // __scrt_release_startup_lock
    Replacement::draft(0x141834cb0, "global_141834cb0", fn_141834cb0 as *const ()), // __scrt_uninitialize_crt
    Replacement::draft(0x141834cdc, "global_141834cdc", fn_141834cdc as *const ()), // _onexit
    Replacement::draft(0x141834d18, "global_141834d18", fn_141834d18 as *const ()), // atexit
    Replacement::draft(0x141834d30, "global_141834d30", tpl_365::<0x1418350d0> as *const ()), // j_j_free
    Replacement::draft_throws(0x141834d64, "crt_141834d64", fn_141834d64 as *const (), Throws::Discard), // sub_141834d64
    Replacement::draft(0x141834da0, "type_info_141834da0", fn_141834da0 as *const ()), // type_info::type_info::vfunc_1
    Replacement::draft(0x141834e48, "global_141834e48", fn_141834e48 as *const ()), // __tlregdtor
    Replacement::draft(0x141834ee0, "global_141834ee0", tpl_365::<0x141834d30> as *const ()), // j_j_free
    Replacement::draft(0x141834f54, "global_141834f54", fn_141834f54 as *const ()), // __ArrayUnwind
    Replacement::draft(0x141834fb8, "crt_141834fb8", tpl_365::<0x141834d64> as *const ()), // sub_141834fb8
    Replacement::draft(0x1418350a4, "crt_1418350a4", tpl_0::<0x14397d1d0, 0x142645430> as *const ()), // sub_1418350a4
    Replacement::draft(0x1418350b4, "crt_1418350b4", tpl_28225::<0x141835b28> as *const ()), // sub_1418350b4
    Replacement::draft_throws(0x1418350bc, "crt_1418350bc", fn_1418350bc as *const (), Throws::Caught), // sub_1418350bc
    Replacement::draft(0x1418350d0, "global_1418350d0", tpl_365::<0x14183ad1c> as *const ()), // j_j_free
    Replacement::draft(0x1418350d8, "crt_1418350d8", fn_1418350d8 as *const ()), // sub_1418350d8
    Replacement::draft(0x141835190, "crt_141835190", tpl_28737::<0x141835be4> as *const ()), // sub_141835190
    Replacement::draft(0x1418351a0, "crt_1418351a0", fn_1418351a0 as *const ()), // sub_1418351a0
    Replacement::draft_throws(0x1418351bc, "crt_1418351bc", fn_1418351bc as *const (), Throws::Caught), // __scrt_common_main_seh
    Replacement::draft_throws(0x141835330, "global_141835330", fn_141835330 as *const (), Throws::Discard), // entry_point
    Replacement::draft(0x141835344, "crt_141835344", fn_141835344 as *const ()), // crt_handler_141835344
    Replacement::draft(0x141835364, "global_141835364", fn_141835364 as *const ()), // __GSHandlerCheckCommon
    Replacement::draft(0x1418353d0, "crt_1418353d0", fn_1418353d0 as *const ()), // __security_check_cookie
    Replacement::draft(0x1418353f0, "crt_1418353f0", tpl_34214::<0x8, 0x10, 0x18, 0x20, 0x38, 0x4, 0x141835364, 0x14183ac80> as *const ()), // crt_handler_1418353f0
    Replacement::draft(0x141835480, "global_141835480", fn_141835480 as *const ()), // __chkstk
    Replacement::draft(0x1418354d0, "crt_1418354d0", fn_1418354d0 as *const ()), // sub_1418354d0
    Replacement::draft(0x1418354f0, "crt_1418354f0", fn_1418354f0 as *const ()), // sub_1418354f0
    Replacement::draft(0x141835533, "crt_141835533", fn_141835533 as *const ()), // sub_141835533
    Replacement::draft(0x141835540, "crt_141835540", fn_141835540 as *const ()), // sub_141835540
    Replacement::draft(0x1418356ec, "crt_1418356ec", tpl_365::<0x1418350bc> as *const ()), // sub_1418356ec
    Replacement::draft(0x1418356f4, "crt_1418356f4", tpl_34214::<0x8, 0x10, 0x18, 0x20, 0x38, 0x4, 0x141835364, 0x14183ace6> as *const ()), // crt_handler_1418356f4
    Replacement::draft(0x141835780, "crt_141835780", fn_141835780 as *const ()), // sub_141835780
    Replacement::draft(0x1418357a0, "crt_1418357a0", fn_1418357a0 as *const ()), // sub_1418357a0
    Replacement::draft(0x1418357f0, "crt_1418357f0", fn_1418357f0 as *const ()), // sub_1418357f0
    Replacement::draft(0x141835810, "crt_141835810", fn_141835810 as *const ()), // sub_141835810
    Replacement::draft(0x141835830, "crt_141835830", tpl_365::<0x141839160> as *const ()), // sub_141835830
    Replacement::draft(0x141835840, "crt_141835840", tpl_365::<0x141839400> as *const ()), // sub_141835840
    Replacement::draft(0x141835850, "crt_141835850", fn_141835850 as *const ()), // sub_141835850
    Replacement::draft(0x141835870, "crt_141835870", tpl_365::<0x141839ab0> as *const ()), // sub_141835870
    Replacement::draft(0x141835880, "crt_141835880", fn_141835880 as *const ()), // sub_141835880
    Replacement::draft(0x141835888, "global_141835888", fn_141835888 as *const ()), // __scrt_is_ucrt_dll_in_use
    Replacement::draft(0x141835894, "crt_141835894", fn_141835894 as *const ()), // sub_141835894
    Replacement::draft(0x14183589c, "crt_14183589c", fn_14183589c as *const ()), // sub_14183589c
    Replacement::draft(0x1418359e4, "global_1418359e4", fn_1418359e4 as *const ()), // __scrt_get_show_window_mode
    Replacement::draft(0x141835a20, "j_ak_141835a20", tpl_365::<0x1401e7150> as *const ()), // j_AK::MemoryMgr::j_AK::MemoryMgr::StopProfileThreadUsage
    Replacement::draft(0x141835a28, "crt_141835a28", fn_141835a28 as *const ()), // sub_141835a28
    Replacement::draft(0x141835a7c, "crt_141835a7c", tpl_0::<0x141835a8c, 0x142645490> as *const ()), // sub_141835a7c
    Replacement::draft(0x141835a8c, "crt_141835a8c", fn_141835a8c as *const ()), // sub_141835a8c
    Replacement::draft(0x141835ae8, "crt_141835ae8", fn_141835ae8 as *const ()), // sub_141835ae8
    Replacement::draft_throws(0x141835b08, "crt_141835b08", tpl_556::<0x141835ae8, 0x1437fcee0, 0x14183acce, 0x141835b27> as *const (), Throws::Discard), // sub_141835b08
    Replacement::draft(0x141835b28, "global_141835b28", tpl_365::<0x14183ad2e> as *const ()), // j__aligned_free
    Replacement::draft(0x141835b30, "crt_141835b30", fn_141835b30 as *const ()), // __security_init_cookie
    Replacement::draft(0x141835bdc, "crt_141835bdc", fn_141835bdc as *const ()), // sub_141835bdc
    Replacement::draft(0x141835be4, "crt_141835be4", fn_141835be4 as *const ()), // sub_141835be4
    Replacement::draft(0x141835c00, "crt_141835c00", fn_141835c00 as *const ()), // sub_141835c00
    Replacement::draft(0x141835c0c, "crt_141835c0c", tpl_1::<0x1426cd928> as *const ()), // sub_141835c0c
    Replacement::draft(0x141835c14, "crt_141835c14", tpl_1::<0x1426cd940> as *const ()), // sub_141835c14
    Replacement::draft(0x141835c1c, "crt_141835c1c", tpl_34229::<0x1434f0f50, 0x142647390> as *const ()), // sub_141835c1c
    Replacement::draft(0x141835c58, "crt_141835c58", tpl_34229::<0x1434f0f60, 0x142647390> as *const ()), // sub_141835c58
    Replacement::draft(0x141835c94, "global_141835c94", fn_141835c94 as *const ()), // __raise_securityfailure
    Replacement::draft(0x141835cc8, "unknown_141835cc8", fn_141835cc8 as *const ()), // sub_141835cc8
    Replacement::draft(0x141835d9c, "global_141835d9c", fn_141835d9c as *const ()), // capture_previous_context
    Replacement::draft(0x141835e10, "global_141835e10", fn_141835e10 as *const ()), // sub_141835e10
    Replacement::draft(0x141836750, "global_141836750", fn_141836750 as *const ()), // sub_141836750
    Replacement::draft(0x141836ed0, "global_141836ed0", fn_141836ed0 as *const ()), // sub_141836ed0
    Replacement::draft(0x141837550, "global_141837550", fn_141837550 as *const ()), // sub_141837550
    Replacement::draft(0x141837bd0, "global_141837bd0", fn_141837bd0 as *const ()), // sub_141837bd0
    Replacement::draft(0x141838540, "global_141838540", fn_141838540 as *const ()), // sub_141838540
    Replacement::draft(0x141838ce0, "global_141838ce0", fn_141838ce0 as *const ()), // sub_141838ce0
    Replacement::draft(0x141838ef0, "global_141838ef0", fn_141838ef0 as *const ()), // sub_141838ef0
    Replacement::draft(0x141839160, "global_141839160", fn_141839160 as *const ()), // sub_141839160
    Replacement::draft(0x141839400, "global_141839400", fn_141839400 as *const ()), // sub_141839400
    Replacement::draft(0x1418394f0, "global_1418394f0", fn_1418394f0 as *const ()), // sub_1418394f0
    Replacement::draft(0x1418397e0, "global_1418397e0", fn_1418397e0 as *const ()), // sub_1418397e0
    Replacement::draft(0x141839ab0, "global_141839ab0", fn_141839ab0 as *const ()), // sub_141839ab0
    Replacement::draft(0x141839bb0, "global_141839bb0", fn_141839bb0 as *const ()), // sub_141839bb0
    Replacement::draft(0x14183a070, "global_14183a070", fn_14183a070 as *const ()), // sub_14183a070
    Replacement::draft(0x14183a0e0, "global_14183a0e0", fn_14183a0e0 as *const ()), // sub_14183a0e0
    Replacement::draft(0x14183a150, "global_14183a150", fn_14183a150 as *const ()), // __common_scos_cout_rare
    Replacement::draft(0x14183a1c0, "unknown_14183a1c0", fn_14183a1c0 as *const ()), // sub_14183a1c0
    Replacement::draft(0x14183ac80, "crt_14183ac80", tpl_906::<0x142646378> as *const ()), // j___CxxFrameHandler4
    Replacement::draft(0x14183ac86, "crt_14183ac86", tpl_906::<0x142646328> as *const ()), // j___std_terminate
    Replacement::draft(0x14183ac8c, "crt_14183ac8c", tpl_906::<0x142646330> as *const ()), // j___std_exception_destroy
    Replacement::draft(0x14183ac92, "crt_14183ac92", tpl_906::<0x142646338> as *const ()), // j___std_exception_copy
    Replacement::draft(0x14183ac98, "crt_14183ac98", tpl_906::<0x142646340> as *const ()), // j__purecall
    Replacement::draft(0x14183ac9e, "crt_14183ac9e", tpl_906::<0x142646348> as *const ()), // j___std_type_info_compare
    Replacement::draft(0x14183aca4, "crt_14183aca4", tpl_906::<0x142646350> as *const ()), // j___C_specific_handler
    Replacement::draft(0x14183acaa, "crt_14183acaa", tpl_906::<0x142646358> as *const ()), // j___std_type_info_name
    Replacement::draft(0x14183acb0, "crt_14183acb0", tpl_906::<0x142646360> as *const ()), // j_strstr
    Replacement::draft(0x14183acb6, "crt_14183acb6", tpl_906::<0x142646320> as *const ()), // j_strchr
    Replacement::draft(0x14183acbc, "crt_14183acbc", tpl_906::<0x142646300> as *const ()), // j_memcpy
    Replacement::draft(0x14183acc2, "crt_14183acc2", tpl_906::<0x1426462f8> as *const ()), // j_memset
    Replacement::draft(0x14183acc8, "crt_14183acc8", tpl_906::<0x1426462c8> as *const ()), // j_memmove
    Replacement::draft_throws(0x14183acce, "crt_14183acce", tpl_34181::<0x142646308> as *const (), Throws::Discard), // j__CxxThrowException
    Replacement::draft(0x14183acd4, "crt_14183acd4", tpl_906::<0x142646318> as *const ()), // j_memchr
    Replacement::draft(0x14183acda, "crt_14183acda", tpl_906::<0x142646368> as *const ()), // j_memcmp
    Replacement::draft(0x14183ace0, "crt_14183ace0", tpl_906::<0x1426462d0> as *const ()), // j_wcsstr
    Replacement::draft(0x14183ace6, "crt_14183ace6", tpl_906::<0x142646310> as *const ()), // j___CxxFrameHandler3
    Replacement::draft(0x14183acec, "crt_14183acec", tpl_906::<0x1426462e0> as *const ()), // j_wcsrchr
    Replacement::draft(0x14183acf2, "crt_14183acf2", tpl_906::<0x1426462e8> as *const ()), // j___current_exception
    Replacement::draft(0x14183acf8, "crt_14183acf8", tpl_906::<0x1426462f0> as *const ()), // j___current_exception_context
    Replacement::draft(0x14183acfe, "crt_14183acfe", tpl_906::<0x1426467c8> as *const ()), // j_terminate
    Replacement::draft(0x14183ad04, "crt_14183ad04", tpl_906::<0x142646a28> as *const ()), // j_strncmp
    Replacement::draft(0x14183ad0a, "crt_14183ad0a", tpl_906::<0x142646780> as *const ()), // j_exit
    Replacement::draft(0x14183ad10, "crt_14183ad10", tpl_906::<0x1426469e0> as *const ()), // j_isalpha
    Replacement::draft(0x14183ad16, "crt_14183ad16", tpl_906::<0x142646970> as *const ()), // j_isdigit
    Replacement::draft(0x14183ad1c, "crt_14183ad1c", tpl_906::<0x142646558> as *const ()), // j_free
    Replacement::draft(0x14183ad22, "crt_14183ad22", tpl_906::<0x1426468e8> as *const ()), // j_fclose
    Replacement::draft(0x14183ad28, "crt_14183ad28", tpl_906::<0x142646538> as *const ()), // j_malloc
    Replacement::draft(0x14183ad2e, "crt_14183ad2e", tpl_906::<0x142646540> as *const ()), // j__aligned_free
    Replacement::draft(0x14183ad34, "crt_14183ad34", tpl_906::<0x142646a20> as *const ()), // j__wcsnicmp
    Replacement::draft(0x14183ad3a, "crt_14183ad3a", tpl_906::<0x142646a18> as *const ()), // j__wcsicmp
    Replacement::draft(0x14183ad40, "crt_14183ad40", tpl_906::<0x142646a10> as *const ()), // j_strlen
    Replacement::draft(0x14183ad46, "crt_14183ad46", tpl_906::<0x142646658> as *const ()), // j_powf
    Replacement::draft(0x14183ad4c, "crt_14183ad4c", tpl_906::<0x1426469b0> as *const ()), // j_wcscmp
    Replacement::draft(0x14183ad52, "crt_14183ad52", tpl_906::<0x142646948> as *const ()), // j_strcmp
    Replacement::draft(0x14183ad58, "crt_14183ad58", tpl_906::<0x142646630> as *const ()), // j_expf
    Replacement::draft(0x14183ad5e, "crt_14183ad5e", tpl_906::<0x1426465f0> as *const ()), // j_log10
    Replacement::draft(0x14183ad64, "crt_14183ad64", tpl_906::<0x1426465e8> as *const ()), // j_log10f
    Replacement::draft(0x14183ad6a, "crt_14183ad6a", tpl_906::<0x1426465d8> as *const ()), // j_cos
    Replacement::draft(0x14183ad70, "crt_14183ad70", tpl_906::<0x1426466d0> as *const ()), // j_exp
    Replacement::draft(0x14183ad76, "crt_14183ad76", tpl_906::<0x1426465a0> as *const ()), // j_fmodf
    Replacement::draft(0x14183ad7c, "crt_14183ad7c", tpl_906::<0x1426466b8> as *const ()), // j_log
    Replacement::draft(0x14183ad82, "crt_14183ad82", tpl_906::<0x1426466a8> as *const ()), // j_cosf
    Replacement::draft(0x14183ad88, "crt_14183ad88", tpl_906::<0x142646670> as *const ()), // j_sinf
    Replacement::draft(0x14183ad8e, "crt_14183ad8e", tpl_906::<0x1426465b8> as *const ()), // j_tanf
    Replacement::draft(0x14183ad94, "crt_14183ad94", tpl_906::<0x1426465c0> as *const ()), // j_atan2f
    Replacement::draft(0x14183ad9a, "crt_14183ad9a", tpl_906::<0x1426465c8> as *const ()), // j_pow
    Replacement::draft(0x14183ada0, "crt_14183ada0", tpl_906::<0x1426465d0> as *const ()), // j_asinf
    Replacement::draft(0x14183ada6, "crt_14183ada6", tpl_906::<0x142646660> as *const ()), // j_logf
    Replacement::draft(0x14183adac, "crt_14183adac", tpl_906::<0x1426465e0> as *const ()), // j_acosf
    Replacement::draft(0x14183adb2, "crt_14183adb2", tpl_906::<0x1426465f8> as *const ()), // j_atan2
    Replacement::draft(0x14183adb8, "crt_14183adb8", tpl_906::<0x142646600> as *const ()), // j_sin
    Replacement::draft(0x14183adbe, "crt_14183adbe", tpl_906::<0x142646608> as *const ()), // j_tan
    Replacement::draft(0x14183adc4, "crt_14183adc4", tpl_906::<0x142646610> as *const ()), // j_atanf
    Replacement::draft(0x14183adca, "crt_14183adca", tpl_906::<0x142646618> as *const ()), // j_tanhf
    Replacement::draft(0x14183add0, "crt_14183add0", tpl_906::<0x142646620> as *const ()), // j_sqrtf
    Replacement::draft(0x14183add6, "crt_14183add6", tpl_906::<0x1426467b0> as *const ()), // j__configure_narrow_argv
    Replacement::draft(0x14183addc, "crt_14183addc", tpl_906::<0x1426467a8> as *const ()), // j__initialize_narrow_environment
    Replacement::draft(0x14183ade2, "crt_14183ade2", tpl_906::<0x1426467a0> as *const ()), // j__initialize_onexit_table
    Replacement::draft(0x14183ade8, "crt_14183ade8", tpl_906::<0x142646798> as *const ()), // j__register_onexit_function
    Replacement::draft(0x14183adee, "crt_14183adee", tpl_906::<0x142646790> as *const ()), // j__crt_atexit
    Replacement::draft(0x14183adf4, "crt_14183adf4", tpl_906::<0x142646788> as *const ()), // j__cexit
    Replacement::draft(0x14183adfa, "crt_14183adfa", tpl_906::<0x142646570> as *const ()), // j__callnewh
    Replacement::draft(0x14183ae00, "crt_14183ae00", tpl_906::<0x142646778> as *const ()), // j__seh_filter_exe
    Replacement::draft(0x14183ae06, "crt_14183ae06", tpl_906::<0x142646770> as *const ()), // j__set_app_type
    Replacement::draft(0x14183ae0c, "crt_14183ae0c", tpl_906::<0x142646640> as *const ()), // j___setusermatherr
    Replacement::draft(0x14183ae12, "crt_14183ae12", tpl_906::<0x142646768> as *const ()), // j__get_narrow_winmain_command_line
    Replacement::draft(0x14183ae18, "crt_14183ae18", tpl_906::<0x142646760> as *const ()), // j__initterm
    Replacement::draft(0x14183ae1e, "crt_14183ae1e", tpl_906::<0x142646758> as *const ()), // j__initterm_e
    Replacement::draft(0x14183ae24, "crt_14183ae24", tpl_906::<0x142646750> as *const ()), // j__exit
    Replacement::draft(0x14183ae2a, "crt_14183ae2a", tpl_906::<0x142646880> as *const ()), // j__set_fmode
    Replacement::draft(0x14183ae30, "crt_14183ae30", tpl_906::<0x142646730> as *const ()), // j__c_exit
    Replacement::draft(0x14183ae36, "crt_14183ae36", tpl_906::<0x142646740> as *const ()), // j__register_thread_local_exe_atexit_callback
    Replacement::draft(0x14183ae3c, "crt_14183ae3c", tpl_906::<0x142646590> as *const ()), // j__configthreadlocale
    Replacement::draft(0x14183ae42, "crt_14183ae42", tpl_906::<0x142646550> as *const ()), // j__set_new_mode
    Replacement::draft(0x14183ae48, "crt_14183ae48", tpl_906::<0x142646908> as *const ()), // j___p__commode
    Replacement::draft(0x14183ae4e, "crt_14183ae4e", tpl_906::<0x142646f38> as *const ()), // j_google::protobuf::RepeatedField<unsigned __int64>::j_google::protobuf::RepeatedField<unsigned __int64>::Add
    Replacement::draft(0x14183ae54, "crt_14183ae54", tpl_906::<0x142646f40> as *const ()), // j_google::protobuf::RepeatedField<double>::j_google::protobuf::RepeatedField<double>::Add
    Replacement::draft(0x14183ae5a, "crt_14183ae5a", tpl_906::<0x142646f68> as *const ()), // j_google::protobuf::Message::j_google::protobuf::Message::InitializationErrorString
    Replacement::draft(0x14183ae60, "crt_14183ae60", tpl_906::<0x142646f70> as *const ()), // j_google::protobuf::Message::j_google::protobuf::Message::DiscardUnknownFields
    Replacement::draft(0x14183ae66, "crt_14183ae66", tpl_906::<0x142646f78> as *const ()), // j_google::protobuf::Message::j_google::protobuf::Message::SpaceUsedLong
    Replacement::draft(0x14183ae6c, "crt_14183ae6c", tpl_906::<0x142646f80> as *const ()), // j_google::protobuf::Message::j_google::protobuf::Message::GetTypeName
    Replacement::draft(0x14183ae72, "crt_14183ae72", tpl_906::<0x142646f88> as *const ()), // j_google::protobuf::Message::j_google::protobuf::Message::CheckTypeAndMergeFrom
    Replacement::draft(0x14183ae78, "crt_14183ae78", tpl_906::<0x142646f90> as *const ()), // j_google::protobuf::MessageLite::j_google::protobuf::MessageLite::InternalGetTable
    Replacement::draft(0x14183ae7e, "crt_14183ae7e", tpl_906::<0x142647100> as *const ()), // j_google::protobuf::Message::j_google::protobuf::Message::~Message
    Replacement::draft(0x14183ae84, "crt_14183ae84", tpl_906::<0x142647168> as *const ()), // j_google::protobuf::RepeatedField<unsigned __int64>::j_google::protobuf::RepeatedField<unsigned __int64>::RepeatedField
    Replacement::draft(0x14183ae8a, "crt_14183ae8a", tpl_906::<0x142647170> as *const ()), // j_google::protobuf::RepeatedField<unsigned __int64>::j_google::protobuf::RepeatedField<unsigned __int64>::~RepeatedField
    Replacement::draft(0x14183ae90, "crt_14183ae90", tpl_906::<0x142647178> as *const ()), // j_google::protobuf::RepeatedField<unsigned __int64>::j_google::protobuf::RepeatedField<unsigned __int64>::size
    Replacement::draft(0x14183ae96, "crt_14183ae96", tpl_906::<0x142647180> as *const ()), // j_google::protobuf::RepeatedField<unsigned __int64>::j_google::protobuf::RepeatedField<unsigned __int64>::Clear
    Replacement::draft(0x14183ae9c, "crt_14183ae9c", tpl_906::<0x142647188> as *const ()), // j_google::protobuf::RepeatedField<unsigned __int64>::j_google::protobuf::RepeatedField<unsigned __int64>::MergeFrom
    Replacement::draft(0x14183aea2, "crt_14183aea2", tpl_906::<0x142647190> as *const ()), // j_google::protobuf::RepeatedField<unsigned __int64>::j_google::protobuf::RepeatedField<unsigned __int64>::data
    Replacement::draft(0x14183aea8, "crt_14183aea8", tpl_906::<0x142647198> as *const ()), // j_google::protobuf::RepeatedField<double>::j_google::protobuf::RepeatedField<double>::RepeatedField
    Replacement::draft(0x14183aeae, "crt_14183aeae", tpl_906::<0x1426471a0> as *const ()), // j_google::protobuf::RepeatedField<double>::j_google::protobuf::RepeatedField<double>::~RepeatedField
    Replacement::draft(0x14183aeb4, "crt_14183aeb4", tpl_906::<0x1426471a8> as *const ()), // j_google::protobuf::RepeatedField<double>::j_google::protobuf::RepeatedField<double>::size
    Replacement::draft(0x14183aeba, "crt_14183aeba", tpl_906::<0x1426471b0> as *const ()), // j_google::protobuf::RepeatedField<double>::j_google::protobuf::RepeatedField<double>::Clear
    Replacement::draft(0x14183aec0, "crt_14183aec0", tpl_906::<0x1426471b8> as *const ()), // j_google::protobuf::RepeatedField<double>::j_google::protobuf::RepeatedField<double>::MergeFrom
    Replacement::draft(0x14183aec6, "crt_14183aec6", tpl_906::<0x1426471c0> as *const ()), // j_google::protobuf::RepeatedField<double>::j_google::protobuf::RepeatedField<double>::data
    Replacement::draft(0x14183aecc, "unknown_14183aecc", tpl_1674::<0x1426cda30> as *const ()), // sub_14183aecc
    Replacement::draft(0x14183aed8, "opentelemetry_14183aed8", tpl_22063::<0x14183aecc, 0x141834d30> as *const ()), // opentelemetry::v1::sdk::common::internal_log::LogHandler::opentelemetry::v1::sdk::common::internal_log::LogHandler::vfun
    Replacement::draft_throws(0x14183af0c, "opentelemetry_14183af0c", fn_14183af0c as *const (), Throws::Discard), // sub_14183af0c
    Replacement::draft_throws(0x14183afd4, "opentelemetry_14183afd4", fn_14183afd4 as *const (), Throws::Discard), // opentelemetry::v1::sdk::common::internal_log::DefaultLogHandler::opentelemetry::v1::sdk::common::internal_log::DefaultLo
    Replacement::draft(0x14183b194, "opentelemetry_14183b194", fn_14183b194 as *const ()), // sub_14183b194
    Replacement::draft_throws(0x14183b238, "opentelemetry_14183b238", fn_14183b238 as *const (), Throws::Caught), // sub_14183b238
    Replacement::draft_throws(0x14183b4bc, "opentelemetry_14183b4bc", fn_14183b4bc as *const (), Throws::Caught), // sub_14183b4bc
    Replacement::draft_throws(0x14183b638, "opentelemetry_14183b638", fn_14183b638 as *const (), Throws::Discard), // sub_14183b638
    Replacement::draft(0x14183b65c, "opentelemetry_14183b65c", fn_14183b65c as *const ()), // sub_14183b65c
    Replacement::draft_throws(0x14183b82c, "opentelemetry_14183b82c", fn_14183b82c as *const (), Throws::Caught), // sub_14183b82c
    Replacement::draft(0x14183b8a0, "opentelemetry_14183b8a0", fn_14183b8a0 as *const ()), // sub_14183b8a0
    Replacement::draft_throws(0x14183ba74, "opentelemetry_14183ba74", fn_14183ba74 as *const (), Throws::Discard), // sub_14183ba74
    Replacement::draft_throws(0x14183bab4, "opentelemetry_14183bab4", fn_14183bab4 as *const (), Throws::Discard), // sub_14183bab4
    Replacement::draft_throws(0x14183bb24, "unknown_14183bb24", fn_14183bb24 as *const (), Throws::Discard), // sub_14183bb24
    Replacement::draft_throws(0x14183bb84, "unknown_14183bb84", tpl_838::<0x14183bb8c> as *const (), Throws::Discard), // sub_14183bb84
    Replacement::draft_throws(0x14183bb8c, "unknown_14183bb8c", fn_14183bb8c as *const (), Throws::Discard), // sub_14183bb8c
    Replacement::draft(0x14183bc75, "crt_14183bc75", tpl_906::<0x142645248> as *const ()), // j_HidP_GetCaps
    Replacement::draft(0x14183bc7b, "crt_14183bc7b", tpl_906::<0x142645250> as *const ()), // j_HidP_GetValueCaps
    Replacement::draft(0x14183bc81, "crt_14183bc81", tpl_906::<0x142645280> as *const ()), // j_HidD_GetAttributes
    Replacement::draft(0x14183bc87, "crt_14183bc87", tpl_906::<0x142645230> as *const ()), // j_HidD_GetHidGuid
    Replacement::draft(0x14183bc8d, "crt_14183bc8d", tpl_906::<0x142645238> as *const ()), // j_HidD_GetPreparsedData
    Replacement::draft(0x14183bc93, "crt_14183bc93", tpl_906::<0x142645240> as *const ()), // j_HidD_FreePreparsedData
    Replacement::draft(0x14183bc99, "crt_14183bc99", tpl_906::<0x142645258> as *const ()), // j_HidD_GetFeature
    Replacement::draft(0x14183bc9f, "crt_14183bc9f", tpl_906::<0x142645278> as *const ()), // j_HidD_SetFeature
    Replacement::draft(0x14183bca5, "crt_14183bca5", tpl_906::<0x142645270> as *const ()), // j_HidD_GetManufacturerString
    Replacement::draft(0x14183bcab, "crt_14183bcab", tpl_906::<0x142645268> as *const ()), // j_HidD_GetProductString
    Replacement::draft(0x14183bcb1, "crt_14183bcb1", tpl_906::<0x142645260> as *const ()), // j_HidD_GetSerialNumberString
    Replacement::draft_throws(0x14183bcc0, "bundleloadinglogic_14183bcc0", fn_14183bcc0 as *const (), Throws::Discard), // BundleLoadingLogic::BundleLoadingLogic::vfunc_3
    Replacement::draft_throws(0x14183bd00, "bundleloadinglogic_14183bd00", fn_14183bd00 as *const (), Throws::Caught), // BundleLoadingLogic::BundleLoadingLogic::vfunc_4
    Replacement::draft(0x14183be90, "unknown_14183be90", tpl_34252 as *const ()), // sub_14183be90
    Replacement::draft(0x14183bea0, "unknown_14183bea0", fn_14183bea0 as *const ()), // sub_14183bea0
    Replacement::draft(0x14183bee0, "unknown_14183bee0", fn_14183bee0 as *const ()), // sub_14183bee0
    Replacement::draft(0x14183bf40, "unknown_14183bf40", fn_14183bf40 as *const ()), // sub_14183bf40
    Replacement::draft_throws(0x14183c020, "unknown_14183c020", fn_14183c020 as *const (), Throws::Caught), // sub_14183c020
    Replacement::draft(0x14183de30, "unknown_14183de30", fn_14183de30 as *const ()), // sub_14183de30
    Replacement::draft(0x14183de40, "unknown_14183de40", tpl_34256::<0x142442de0> as *const ()), // sub_14183de40
    Replacement::draft(0x14183de60, "unknown_14183de60", tpl_361 as *const ()), // sub_14183de60
    Replacement::draft(0x14183de70, "std_14183de70", tpl_770::<0x1430792c0> as *const ()), // std::_Func_impl_no_alloc<GameClass::GameClass::<lambda_2>,void,const m::SIMDTransform&>::std::_Func_impl_no_alloc<GameCl
    Replacement::draft(0x14183de90, "std_14183de90", tpl_770::<0x1430792c0> as *const ()), // std::_Func_impl_no_alloc<GameClass::GameClass::<lambda_2>,void,const m::SIMDTransform&>::std::_Func_impl_no_alloc<GameCl
    Replacement::draft(0x14183deb0, "std_14183deb0", tpl_34257::<0x14397da68, 0x8, 0x141fbb040, 0x141f67ba0> as *const ()), // std::_Func_impl_no_alloc<GameClass::GameClass::<lambda_2>,void,const m::SIMDTransform&>::std::_Func_impl_no_alloc<GameCl
    Replacement::draft(0x14183df00, "std_14183df00", tpl_1::<0x143883730> as *const ()), // std::_Func_impl_no_alloc<GameClass::GameClass::<lambda_2>,void,const m::SIMDTransform&>::std::_Func_impl_no_alloc<GameCl
    Replacement::draft(0x14183df10, "std_14183df10", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<GameClass::GameClass::<lambda_2>,void,const m::SIMDTransform&>::std::_Func_impl_no_alloc<GameCl
    Replacement::draft(0x14183df20, "std_14183df20", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<GameClass::GameClass::<lambda_2>,void,const m::SIMDTransform&>::std::_Func_impl_no_alloc<GameCl
    Replacement::draft(0x14183df40, "std_14183df40", tpl_550::<0x143079410, 0x8> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::MetadataProvider::buildMap::<lambda_1>,void,?bref6::ResourceID,?bref6::Pat
    Replacement::draft(0x14183df60, "std_14183df60", tpl_550::<0x143079410, 0x8> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::MetadataProvider::buildMap::<lambda_1>,void,?bref6::ResourceID,?bref6::Pat
    Replacement::draft(0x14183e010, "std_14183e010", tpl_1::<0x143883800> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::MetadataProvider::buildMap::<lambda_1>,void,?bref6::ResourceID,?bref6::Pat
    Replacement::draft(0x14183e020, "std_14183e020", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::MetadataProvider::buildMap::<lambda_1>,void,?bref6::ResourceID,?bref6::Pat
    Replacement::draft(0x14183e030, "std_14183e030", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<anonymous_namespace::MetadataProvider::buildMap::<lambda_1>,void,?bref6::ResourceID,?bref6::Pat
    Replacement::draft(0x14183e050, "unknown_14183e050", tpl_34259 as *const ()), // sub_14183e050
    Replacement::draft(0x14183e0a0, "unknown_14183e0a0", fn_14183e0a0 as *const ()), // sub_14183e0a0
    Replacement::draft(0x14183e0f0, "unknown_14183e0f0", fn_14183e0f0 as *const ()), // sub_14183e0f0
    Replacement::draft_throws(0x14183e4f0, "unknown_14183e4f0", fn_14183e4f0 as *const (), Throws::Discard), // sub_14183e4f0
    Replacement::draft_throws(0x14183e5d0, "unknown_14183e5d0", fn_14183e5d0 as *const (), Throws::Caught), // sub_14183e5d0
    Replacement::draft_throws(0x14183e9c0, "unknown_14183e9c0", fn_14183e9c0 as *const (), Throws::Caught), // sub_14183e9c0
    Replacement::draft(0x14183ebc0, "unknown_14183ebc0", tpl_34265::<0x14397d808, 0x18> as *const ()), // sub_14183ebc0
    Replacement::draft_throws(0x14183ebe0, "unknown_14183ebe0", fn_14183ebe0 as *const (), Throws::Discard), // sub_14183ebe0
    Replacement::draft_throws(0x14183f7b0, "unknown_14183f7b0", fn_14183f7b0 as *const (), Throws::Caught), // sub_14183f7b0
    Replacement::draft_throws(0x14183f9b0, "gameclass_14183f9b0", fn_14183f9b0 as *const (), Throws::Discard), // GameClass::GameClass::StreamJob_GameConstruct::GameClass::GameClass::StreamJob_GameConstruct::vfunc_2
    Replacement::draft(0x14183f9d0, "unknown_14183f9d0", tpl_34257::<0x14397da68, 0x8, 0x141fbb040, 0x141f67ba0> as *const ()), // sub_14183f9d0
    Replacement::draft_throws(0x14183fa40, "unknown_14183fa40", fn_14183fa40 as *const (), Throws::Caught), // sub_14183fa40
    Replacement::draft(0x141840ed0, "unknown_141840ed0", tpl_34268::<0x4> as *const ()), // sub_141840ed0
    Replacement::draft(0x141840ee0, "unknown_141840ee0", tpl_34269::<0x143409090, 0x18, 0x14243ef10, 0x142442de0> as *const ()), // sub_141840ee0
    Replacement::draft(0x141840f20, "anonymous_namespace_141840f20", tpl_25168::<0x143409090, 0x18, 0x14243ef10, 0x142442de0, 0x141834d30> as *const ()), // anonymous_namespace::MetadataProvider::anonymous_namespace::MetadataProvider::vfunc_0
    Replacement::draft_throws(0x141840fa0, "anonymous_namespace_141840fa0", fn_141840fa0 as *const (), Throws::Discard), // sub_141840fa0
    Replacement::draft_throws(0x141841040, "anonymous_namespace_141841040", fn_141841040 as *const (), Throws::Caught), // anonymous_namespace::MetadataProvider::anonymous_namespace::MetadataProvider::vfunc_1
    Replacement::draft_throws(0x141841580, "unknown_141841580", fn_141841580 as *const (), Throws::Caught), // sub_141841580
    Replacement::draft(0x141841690, "unknown_141841690", fn_141841690 as *const ()), // sub_141841690
    Replacement::draft(0x1418416f0, "unknown_1418416f0", fn_1418416f0 as *const ()), // sub_1418416f0
    Replacement::draft_throws(0x141841740, "unknown_141841740", fn_141841740 as *const (), Throws::Discard), // sub_141841740
    Replacement::draft_throws(0x141841840, "unknown_141841840", fn_141841840 as *const (), Throws::Discard), // sub_141841840
    Replacement::draft_throws(0x141841930, "unknown_141841930", fn_141841930 as *const (), Throws::Caught), // sub_141841930
    Replacement::draft_throws(0x141841a40, "global_141841a40", fn_141841a40 as *const (), Throws::Discard), // cpu_check_caller
    Replacement::draft_throws(0x141841bd0, "global_141841bd0", fn_141841bd0 as *const (), Throws::Caught), // ssd_seek_penalty_check
    Replacement::draft_throws(0x141841f60, "unknown_141841f60", fn_141841f60 as *const (), Throws::Discard), // sub_141841f60
    Replacement::draft(0x1418420b0, "unknown_1418420b0", fn_1418420b0 as *const ()), // sub_1418420b0
    Replacement::draft(0x1418420e0, "unknown_1418420e0", fn_1418420e0 as *const ()), // sub_1418420e0
    Replacement::draft(0x141842110, "unknown_141842110", fn_141842110 as *const ()), // sub_141842110
    Replacement::draft(0x1418421b0, "gameserver_1418421b0", fn_1418421b0 as *const ()), // GameServer::GameServer::vfunc_32
    Replacement::draft_throws(0x1418421d0, "gameserver_1418421d0", fn_1418421d0 as *const (), Throws::Caught), // GameServer::GameServer::vfunc_30
    Replacement::draft_throws(0x1418422e0, "gameserver_1418422e0", fn_1418422e0 as *const (), Throws::Caught), // GameServer::GameServer::vfunc_29
    Replacement::draft_throws(0x1418423b0, "gameserver_1418423b0", fn_1418423b0 as *const (), Throws::Caught), // GameServer::GameServer::vfunc_28
    Replacement::draft_throws(0x141842600, "gameserver_141842600", fn_141842600 as *const (), Throws::Discard), // GameServer::GameServer::vfunc_26
    Replacement::draft(0x141842620, "gameserver_141842620", tpl_22466::<0x141b21040> as *const ()), // GameServer::GameServer::vfunc_25
    Replacement::draft(0x141842640, "gameserver_141842640", fn_141842640 as *const ()), // GameServer::GameServer::vfunc_24
    Replacement::draft(0x1418427e0, "gameserver_1418427e0", fn_1418427e0 as *const ()), // GameServer::GameServer::vfunc_23
    Replacement::draft(0x141842a30, "gameserver_141842a30", fn_141842a30 as *const ()), // sub_141842a30
    Replacement::draft(0x141842b30, "gameserver_141842b30", fn_141842b30 as *const ()), // sub_141842b30
    Replacement::draft(0x141842c60, "gameserver_141842c60", fn_141842c60 as *const ()), // sub_141842c60
    Replacement::draft_throws(0x141842c80, "gameserver_141842c80", fn_141842c80 as *const (), Throws::Caught), // sub_141842c80
    Replacement::draft(0x141842d70, "gameserver_141842d70", fn_141842d70 as *const ()), // sub_141842d70
    Replacement::draft(0x141842e40, "gameserver_141842e40", fn_141842e40 as *const ()), // sub_141842e40
    Replacement::draft_throws(0x141842e70, "gameserver_141842e70", fn_141842e70 as *const (), Throws::Discard), // GameServer::GameServer::vfunc_33
    Replacement::draft(0x141842ec0, "gameserver_141842ec0", fn_141842ec0 as *const ()), // sub_141842ec0
    Replacement::draft(0x141842ef0, "gameserver_141842ef0", fn_141842ef0 as *const ()), // sub_141842ef0
    Replacement::draft(0x141842f30, "gameserver_141842f30", fn_141842f30 as *const ()), // sub_141842f30
    Replacement::draft(0x141842f80, "gameserver_141842f80", fn_141842f80 as *const ()), // sub_141842f80
    Replacement::draft(0x141842f90, "gameserver_141842f90", fn_141842f90 as *const ()), // GameServer::GameServer::vfunc_21
    Replacement::draft(0x141843000, "gameserver_141843000", fn_141843000 as *const ()), // GameServer::GameServer::vfunc_20
    Replacement::draft(0x141843100, "gameserver_141843100", fn_141843100 as *const ()), // GameServer::GameServer::vfunc_22
    Replacement::draft(0x141843150, "gameserver_141843150", fn_141843150 as *const ()), // GameServer::GameServer::vfunc_11
    Replacement::draft(0x141843420, "gameserver_141843420", fn_141843420 as *const ()), // GameServer::GameServer::vfunc_10
    Replacement::draft_throws(0x1418434a0, "gameserver_1418434a0", fn_1418434a0 as *const (), Throws::Discard), // sub_1418434a0
    Replacement::draft_throws(0x1418437a0, "gameserver_1418437a0", fn_1418437a0 as *const (), Throws::Caught), // GameServer::GameServer::vfunc_9
    Replacement::draft_throws(0x141844310, "gameserver_141844310", fn_141844310 as *const (), Throws::Caught), // GameServer::GameServer::vfunc_6
    Replacement::draft(0x141844c70, "gameserver_141844c70", fn_141844c70 as *const ()), // GameServer::GameServer::vfunc_5
    Replacement::draft(0x141844d10, "gameserver_141844d10", fn_141844d10 as *const ()), // GameServer::GameServer::vfunc_8
    Replacement::draft_throws(0x141844d60, "gameserver_141844d60", fn_141844d60 as *const (), Throws::Caught), // GameServer::GameServer::vfunc_7
    Replacement::draft_throws(0x141844fb0, "gameserver_141844fb0", fn_141844fb0 as *const (), Throws::Caught), // GameServer::GameServer::vfunc_4
    Replacement::draft(0x141845530, "gameserver_141845530", fn_141845530 as *const ()), // sub_141845530
    Replacement::draft(0x141845580, "gameserver_141845580", fn_141845580 as *const ()), // sub_141845580
    Replacement::draft_throws(0x1418455f0, "gameserver_1418455f0", fn_1418455f0 as *const (), Throws::Caught), // GameServer::GameServer::vfunc_1
    Replacement::draft(0x141846310, "gameserver_141846310", fn_141846310 as *const ()), // GameServer::GameServer::vfunc_1
    Replacement::draft_throws(0x141846380, "unknown_141846380", fn_141846380 as *const (), Throws::Discard), // sub_141846380
    Replacement::draft(0x141846590, "unknown_141846590", fn_141846590 as *const ()), // sub_141846590
    Replacement::draft(0x1418467f0, "unknown_1418467f0", fn_1418467f0 as *const ()), // sub_1418467f0
    Replacement::draft(0x141846ad0, "unknown_141846ad0", fn_141846ad0 as *const ()), // sub_141846ad0
    Replacement::draft(0x141846b10, "unknown_141846b10", fn_141846b10 as *const ()), // sub_141846b10
    Replacement::draft(0x141846b40, "unknown_141846b40", fn_141846b40 as *const ()), // sub_141846b40
    Replacement::draft(0x141846b70, "unknown_141846b70", tpl_34312::<0x1401e7f60> as *const ()), // sub_141846b70
    Replacement::draft(0x141846bc0, "unknown_141846bc0", tpl_34313::<0x141834d30> as *const ()), // sub_141846bc0
    Replacement::draft(0x141846be0, "unknown_141846be0", tpl_34313::<0x141834d30> as *const ()), // sub_141846be0
    Replacement::draft(0x141846c00, "unknown_141846c00", fn_141846c00 as *const ()), // sub_141846c00
    Replacement::draft(0x141846c50, "unknown_141846c50", fn_141846c50 as *const ()), // sub_141846c50
    Replacement::draft_throws(0x141846ca0, "std_141846ca0", fn_141846ca0 as *const (), Throws::Caught), // std::_Func_impl_no_alloc<ProgramFlowServerStateInGame::onActionSent::<lambda_1>,void>::std::_Func_impl_no_alloc<ProgramF
    Replacement::draft(0x141846d00, "std_141846d00", tpl_559 as *const ()), // std::_Func_impl_no_alloc<ProgramFlowServerStateInGame::onActionSent::<lambda_1>,void>::std::_Func_impl_no_alloc<ProgramF
    Replacement::draft(0x141846d20, "std_141846d20", tpl_1::<0x143883ca0> as *const ()), // std::_Func_impl_no_alloc<ProgramFlowServerStateInGame::onActionSent::<lambda_1>,void>::std::_Func_impl_no_alloc<ProgramF
    Replacement::draft(0x141846d30, "std_141846d30", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<ProgramFlowServerStateInGame::onActionSent::<lambda_1>,void>::std::_Func_impl_no_alloc<ProgramF
    Replacement::draft(0x141846d40, "std_141846d40", fn_141846d40 as *const ()), // std::_Func_impl_no_alloc<ProgramFlowServerStateInGame::onActionSent::<lambda_1>,void>::std::_Func_impl_no_alloc<ProgramF
    Replacement::draft(0x141846da0, "unknown_141846da0", tpl_34318 as *const ()), // sub_141846da0
    Replacement::draft(0x141846dc0, "unknown_141846dc0", tpl_34319::<0x143c8cecc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141846dc0
    Replacement::draft_throws(0x141846e20, "unknown_141846e20", fn_141846e20 as *const (), Throws::Caught), // sub_141846e20
    Replacement::draft(0x1418470c0, "unknown_1418470c0", tpl_34321::<0x110, 0x114> as *const ()), // sub_1418470c0
    Replacement::draft(0x1418470e0, "unknown_1418470e0", fn_1418470e0 as *const ()), // sub_1418470e0
    Replacement::draft(0x141847110, "unknown_141847110", tpl_34323::<0x108> as *const ()), // sub_141847110
    Replacement::draft(0x141847140, "unknown_141847140", fn_141847140 as *const ()), // sub_141847140
    Replacement::draft_throws(0x141847160, "programflowserverstateingame_141847160", fn_141847160 as *const (), Throws::Discard), // ProgramFlowServerStateInGame::ProgramFlowServerStateInGame::vfunc_4
    Replacement::draft_throws(0x141847260, "programflowserverstateingame_141847260", fn_141847260 as *const (), Throws::Discard), // ProgramFlowServerStateInGame::ProgramFlowServerStateInGame::vfunc_3
    Replacement::draft(0x141847430, "programflowserverstateingame_141847430", fn_141847430 as *const ()), // sub_141847430
    Replacement::draft(0x141847480, "programflowserverstateingame_141847480", fn_141847480 as *const ()), // sub_141847480
    Replacement::draft(0x1418474b0, "programflowserverstateingame_1418474b0", fn_1418474b0 as *const ()), // sub_1418474b0
    Replacement::draft_throws(0x141847670, "programflowserverstateingame_141847670", fn_141847670 as *const (), Throws::Caught), // ProgramFlowServerStateInGame::ProgramFlowServerStateInGame::vfunc_2
    Replacement::draft(0x141848370, "programflowserverstateingame_141848370", fn_141848370 as *const ()), // sub_141848370
    Replacement::draft_throws(0x1418483d0, "programflowserverstateingame_1418483d0", fn_1418483d0 as *const (), Throws::Caught), // ProgramFlowServerStateInGame::ProgramFlowServerStateInGame::vfunc_1
    Replacement::draft(0x1418489e0, "programflowserverstateingame_1418489e0", fn_1418489e0 as *const ()), // sub_1418489e0
    Replacement::draft(0x141848aa0, "programflowserverstateingame_141848aa0", fn_141848aa0 as *const ()), // sub_141848aa0
    Replacement::draft(0x1418491f0, "programflowserverstateingame_1418491f0", fn_1418491f0 as *const ()), // sub_1418491f0
    Replacement::draft(0x141849280, "programflowserverstateingame_141849280", fn_141849280 as *const ()), // sub_141849280
    Replacement::draft(0x141849c00, "unknown_141849c00", fn_141849c00 as *const ()), // sub_141849c00
    Replacement::draft_throws(0x141849db0, "unknown_141849db0", fn_141849db0 as *const (), Throws::Discard), // sub_141849db0
    Replacement::draft_throws(0x141849e90, "unknown_141849e90", fn_141849e90 as *const (), Throws::Caught), // sub_141849e90
    Replacement::draft_throws(0x14184a6e0, "programflowserverstateloadinglevel_14184a6e0", fn_14184a6e0 as *const (), Throws::Discard), // ProgramFlowServerStateLoadingLevel::ProgramFlowServerStateLoadingLevel::vfunc_4
    Replacement::draft_throws(0x14184a7a0, "programflowserverstateloadinglevel_14184a7a0", fn_14184a7a0 as *const (), Throws::Discard), // ProgramFlowServerStateLoadingLevel::ProgramFlowServerStateLoadingLevel::vfunc_3
    Replacement::draft_throws(0x14184a830, "programflowserverstateloadinglevel_14184a830", fn_14184a830 as *const (), Throws::Discard), // ProgramFlowServerStateLoadingLevel::ProgramFlowServerStateLoadingLevel::vfunc_1
    Replacement::draft(0x14184a950, "programflowserverstateloadinglevel_14184a950", fn_14184a950 as *const ()), // sub_14184a950
    Replacement::draft(0x14184a9b0, "programflowserverstateloadinglevel_14184a9b0", fn_14184a9b0 as *const ()), // sub_14184a9b0
    Replacement::draft(0x14184b200, "programflowserverstateloadinglevel_14184b200", fn_14184b200 as *const ()), // ProgramFlowServerStateLoadingLevel::ClassTypeInfoImplementation::ProgramFlowServerStateLoadingLevel::ClassTypeInfoImplem
    Replacement::draft(0x14184b400, "unknown_14184b400", fn_14184b400 as *const ()), // sub_14184b400
    Replacement::draft_throws(0x14184b440, "programflowserverstatemainmenu_14184b440", fn_14184b440 as *const (), Throws::Discard), // ProgramFlowServerStateMainMenu::ProgramFlowServerStateMainMenu::vfunc_4
    Replacement::draft_throws(0x14184b550, "programflowserverstatemainmenu_14184b550", fn_14184b550 as *const (), Throws::Discard), // ProgramFlowServerStateMainMenu::ProgramFlowServerStateMainMenu::vfunc_3
    Replacement::draft_throws(0x14184b6d0, "programflowserverstatemainmenu_14184b6d0", fn_14184b6d0 as *const (), Throws::Discard), // ProgramFlowServerStateMainMenu::ProgramFlowServerStateMainMenu::vfunc_2
    Replacement::draft_throws(0x14184be70, "programflowserverstatemainmenu_14184be70", fn_14184be70 as *const (), Throws::Caught), // ProgramFlowServerStateMainMenu::ProgramFlowServerStateMainMenu::vfunc_1
    Replacement::draft(0x14184c570, "programflowserverstatemainmenu_14184c570", fn_14184c570 as *const ()), // sub_14184c570
    Replacement::draft(0x14184c5c0, "programflowserverstatemainmenu_14184c5c0", fn_14184c5c0 as *const ()), // sub_14184c5c0
    Replacement::draft(0x14184c840, "programflowserverstatemainmenu_14184c840", fn_14184c840 as *const ()), // sub_14184c840
    Replacement::draft(0x14184d290, "unknown_14184d290", fn_14184d290 as *const ()), // sub_14184d290
    Replacement::draft(0x14184d2c0, "unknown_14184d2c0", fn_14184d2c0 as *const ()), // sub_14184d2c0
    Replacement::draft(0x14184d320, "unknown_14184d320", tpl_34323::<0x18> as *const ()), // sub_14184d320
    Replacement::draft_throws(0x14184d360, "programflowserverstatepausegame_14184d360", fn_14184d360 as *const (), Throws::Caught), // ProgramFlowServerStatePauseGame::ProgramFlowServerStatePauseGame::vfunc_4
    Replacement::draft_throws(0x14184d560, "programflowserverstatepausegame_14184d560", fn_14184d560 as *const (), Throws::Discard), // ProgramFlowServerStatePauseGame::ProgramFlowServerStatePauseGame::vfunc_3
    Replacement::draft(0x14184d7a0, "programflowserverstatepausegame_14184d7a0", tpl_34321::<0x28, 0x2c> as *const ()), // sub_14184d7a0
    Replacement::draft_throws(0x14184d7b0, "programflowserverstatepausegame_14184d7b0", fn_14184d7b0 as *const (), Throws::Caught), // ProgramFlowServerStatePauseGame::ProgramFlowServerStatePauseGame::vfunc_2
    Replacement::draft_throws(0x14184def0, "programflowserverstatepausegame_14184def0", fn_14184def0 as *const (), Throws::Caught), // ProgramFlowServerStatePauseGame::ProgramFlowServerStatePauseGame::vfunc_1
    Replacement::draft(0x14184eab0, "programflowserverstatepausegame_14184eab0", fn_14184eab0 as *const ()), // sub_14184eab0
    Replacement::draft(0x14184eb10, "programflowserverstatepausegame_14184eb10", fn_14184eb10 as *const ()), // sub_14184eb10
    Replacement::draft(0x14184eba0, "programflowserverstatepausegame_14184eba0", fn_14184eba0 as *const ()), // sub_14184eba0
    Replacement::draft(0x14184eda0, "programflowserverstatepausegame_14184eda0", fn_14184eda0 as *const ()), // sub_14184eda0
    Replacement::draft(0x14184f570, "programflowserverstatepausegame_14184f570", fn_14184f570 as *const ()), // ProgramFlowServerStatePauseGame::ClassTypeInfoImplementation::ProgramFlowServerStatePauseGame::ClassTypeInfoImplementati
    Replacement::draft(0x14184f820, "unknown_14184f820", tpl_34353::<0x1401e3940, 0x1401e3b70, 0x14184d350, 0x1401e3f60, 0x14184d2c0, 0x14184d320, 0x14184d290> as *const ()), // sub_14184f820
    Replacement::draft(0x14184fa30, "unknown_14184fa30", fn_14184fa30 as *const ()), // sub_14184fa30
    Replacement::draft(0x14184fcb0, "unknown_14184fcb0", fn_14184fcb0 as *const ()), // sub_14184fcb0
    Replacement::draft(0x14184fd80, "unknown_14184fd80", fn_14184fd80 as *const ()), // sub_14184fd80
    Replacement::draft(0x14184fe70, "unknown_14184fe70", fn_14184fe70 as *const ()), // sub_14184fe70
    Replacement::draft_throws(0x14184ffe0, "programflowserverstatephotomode_14184ffe0", fn_14184ffe0 as *const (), Throws::Discard), // ProgramFlowServerStatePhotoMode::ProgramFlowServerStatePhotoMode::vfunc_4
    Replacement::draft_throws(0x141850230, "programflowserverstatephotomode_141850230", fn_141850230 as *const (), Throws::Discard), // ProgramFlowServerStatePhotoMode::ProgramFlowServerStatePhotoMode::vfunc_3
    Replacement::draft_throws(0x1418507b0, "programflowserverstatephotomode_1418507b0", fn_1418507b0 as *const (), Throws::Discard), // ProgramFlowServerStatePhotoMode::ProgramFlowServerStatePhotoMode::vfunc_1
    Replacement::draft(0x1418508c0, "programflowserverstatephotomode_1418508c0", fn_1418508c0 as *const ()), // sub_1418508c0
    Replacement::draft(0x1418508e0, "programflowserverstatephotomode_1418508e0", fn_1418508e0 as *const ()), // sub_1418508e0
    Replacement::draft_throws(0x141850f00, "programflowserverstatephotomode_141850f00", tpl_838::<0x1401f02f0> as *const (), Throws::Discard), // ProgramFlowServerStatePhotoMode::ProgramFlowServerStatePhotoMode::vfunc_8
    Replacement::draft(0x141851670, "programflowserverstatephotomode_141851670", fn_141851670 as *const ()), // ProgramFlowServerStatePhotoMode::ClassTypeInfoImplementation::ProgramFlowServerStatePhotoMode::ClassTypeInfoImplementati
    Replacement::draft(0x1418516c0, "programflowserverstatephotomode_1418516c0", fn_1418516c0 as *const ()), // ProgramFlowServerStatePhotoMode::ClassTypeInfoImplementation::ProgramFlowServerStatePhotoMode::ClassTypeInfoImplementati
    Replacement::draft_throws(0x1418519a0, "unknown_1418519a0", tpl_34362::<0x14397d814, 0x143a340f8, 0x1420e3ef0> as *const (), Throws::Discard), // sub_1418519a0
    Replacement::draft_throws(0x1418519f0, "unknown_1418519f0", tpl_34362::<0x14397d818, 0x143a340f8, 0x1420e3ef0> as *const (), Throws::Discard), // sub_1418519f0
    Replacement::draft_throws(0x141851a40, "programflowserverstatereacquireuser_141851a40", fn_141851a40 as *const (), Throws::Discard), // ProgramFlowServerStateReacquireUser::ProgramFlowServerStateReacquireUser::vfunc_4
    Replacement::draft_throws(0x141851ac0, "programflowserverstatereacquireuser_141851ac0", fn_141851ac0 as *const (), Throws::Caught), // ProgramFlowServerStateReacquireUser::ProgramFlowServerStateReacquireUser::vfunc_3
    Replacement::draft_throws(0x141851c80, "programflowserverstatereacquireuser_141851c80", fn_141851c80 as *const (), Throws::Discard), // sub_141851c80
    Replacement::draft_throws(0x141851cd0, "programflowserverstatereacquireuser_141851cd0", fn_141851cd0 as *const (), Throws::Discard), // sub_141851cd0
    Replacement::draft_throws(0x141851d30, "programflowserverstatereacquireuser_141851d30", fn_141851d30 as *const (), Throws::Discard), // sub_141851d30
    Replacement::draft_throws(0x141851dd0, "programflowserverstatereacquireuser_141851dd0", fn_141851dd0 as *const (), Throws::Caught), // sub_141851dd0
    Replacement::draft_throws(0x141852180, "programflowserverstatereacquireuser_141852180", fn_141852180 as *const (), Throws::Caught), // sub_141852180
    Replacement::draft_throws(0x1418526e0, "programflowserverstatereacquireuser_1418526e0", fn_1418526e0 as *const (), Throws::Discard), // ProgramFlowServerStateReacquireUser::ProgramFlowServerStateReacquireUser::vfunc_1
    Replacement::draft(0x141852700, "programflowserverstatereacquireuser_141852700", fn_141852700 as *const ()), // sub_141852700
    Replacement::draft(0x141852f70, "programflowserverstatereacquireuser_141852f70", fn_141852f70 as *const ()), // ProgramFlowServerStateReacquireUser::ClassTypeInfoImplementation::ProgramFlowServerStateReacquireUser::ClassTypeInfoImpl
    Replacement::draft(0x1418533f0, "unknown_1418533f0", fn_1418533f0 as *const ()), // sub_1418533f0
    Replacement::draft_throws(0x141853400, "unknown_141853400", fn_141853400 as *const (), Throws::Discard), // sub_141853400
    Replacement::draft_throws(0x141853a40, "unknown_141853a40", fn_141853a40 as *const (), Throws::Discard), // sub_141853a40
    Replacement::draft_throws(0x141853a80, "unknown_141853a80", fn_141853a80 as *const (), Throws::Discard), // sub_141853a80
    Replacement::draft(0x141853af0, "programflowserverstatesplashscreen_141853af0", fn_141853af0 as *const ()), // sub_141853af0
    Replacement::draft_throws(0x141853b10, "programflowserverstatesplashscreen_141853b10", fn_141853b10 as *const (), Throws::Caught), // ProgramFlowServerStateSplashScreen::ProgramFlowServerStateSplashScreen::vfunc_3
    Replacement::draft_throws(0x141853ee0, "programflowserverstatesplashscreen_141853ee0", fn_141853ee0 as *const (), Throws::Discard), // ProgramFlowServerStateSplashScreen::ProgramFlowServerStateSplashScreen::vfunc_2
    Replacement::draft(0x1418540b0, "programflowserverstatesplashscreen_1418540b0", fn_1418540b0 as *const ()), // sub_1418540b0
    Replacement::draft(0x1418540e0, "programflowserverstatesplashscreen_1418540e0", fn_1418540e0 as *const ()), // sub_1418540e0
    Replacement::draft_throws(0x141854120, "programflowserverstatesplashscreen_141854120", fn_141854120 as *const (), Throws::Caught), // ProgramFlowServerStateSplashScreen::ProgramFlowServerStateSplashScreen::vfunc_1
    Replacement::draft(0x141854db0, "programflowserverstatesplashscreen_141854db0", fn_141854db0 as *const ()), // sub_141854db0
    Replacement::draft(0x141855b80, "programflowserverstatesplashscreen_141855b80", fn_141855b80 as *const ()), // ProgramFlowServerStateSplashScreen::ClassTypeInfoImplementation::ProgramFlowServerStateSplashScreen::ClassTypeInfoImplem
    Replacement::draft(0x141855d20, "unknown_141855d20", fn_141855d20 as *const ()), // sub_141855d20
    Replacement::draft_throws(0x141855d60, "unknown_141855d60", fn_141855d60 as *const (), Throws::Caught), // sub_141855d60
    Replacement::draft(0x1418560a0, "programflowserverstatestart_1418560a0", tpl_34387::<0x8, 0xd4, 0xb4, 0x143a293d4, 0xa0, 0x1413953d0, 0xa8, 0x108, 0x110, 0x120, 0x130> as *const ()), // ProgramFlowServerStateStart::ProgramFlowServerStateStart::vfunc_3
    Replacement::draft(0x1418561a0, "programflowserverstatestart_1418561a0", tpl_28560::<0x8, 0x10, 0x14307dab8> as *const ()), // sub_1418561a0
    Replacement::draft_throws(0x141856b20, "unknown_141856b20", fn_141856b20 as *const (), Throws::Caught), // sub_141856b20
    Replacement::draft_throws(0x141858620, "unknown_141858620", fn_141858620 as *const (), Throws::Caught), // sub_141858620
    Replacement::draft(0x141858d80, "unknown_141858d80", fn_141858d80 as *const ()), // sub_141858d80
    Replacement::draft(0x141859190, "unknown_141859190", fn_141859190 as *const ()), // sub_141859190
    Replacement::draft(0x141859300, "unknown_141859300", fn_141859300 as *const ()), // sub_141859300
    Replacement::draft(0x141859480, "unknown_141859480", tpl_34393 as *const ()), // sub_141859480
    Replacement::draft(0x141859510, "unknown_141859510", tpl_34394::<0x10> as *const ()), // sub_141859510
    Replacement::draft(0x1418595b0, "unknown_1418595b0", tpl_34395::<0x10, 0x18> as *const ()), // sub_1418595b0
    Replacement::draft(0x141859700, "unknown_141859700", tpl_34396::<0x143a00240, 0x58> as *const ()), // sub_141859700
    Replacement::draft(0x1418597e0, "unknown_1418597e0", fn_1418597e0 as *const ()), // sub_1418597e0
    Replacement::draft_throws(0x141859930, "unknown_141859930", fn_141859930 as *const (), Throws::Discard), // sub_141859930
    Replacement::draft_throws(0x141859b70, "unknown_141859b70", fn_141859b70 as *const (), Throws::Caught), // sub_141859b70
    Replacement::draft_throws(0x141859d30, "unknown_141859d30", fn_141859d30 as *const (), Throws::Caught), // sub_141859d30
    Replacement::draft(0x141859ed0, "unknown_141859ed0", fn_141859ed0 as *const ()), // sub_141859ed0
    Replacement::draft(0x141859f70, "unknown_141859f70", fn_141859f70 as *const ()), // sub_141859f70
    Replacement::draft(0x14185a010, "unknown_14185a010", fn_14185a010 as *const ()), // sub_14185a010
    Replacement::draft_throws(0x14185a110, "unknown_14185a110", fn_14185a110 as *const (), Throws::Discard), // sub_14185a110
    Replacement::draft_throws(0x14185a1c0, "unknown_14185a1c0", fn_14185a1c0 as *const (), Throws::Caught), // sub_14185a1c0
    Replacement::draft_throws(0x14185a630, "unknown_14185a630", fn_14185a630 as *const (), Throws::Discard), // sub_14185a630
    Replacement::draft(0x14185a750, "unknown_14185a750", fn_14185a750 as *const ()), // sub_14185a750
    Replacement::draft(0x14185aac0, "unknown_14185aac0", fn_14185aac0 as *const ()), // sub_14185aac0
    Replacement::draft_throws(0x14185ac40, "unknown_14185ac40", fn_14185ac40 as *const (), Throws::Discard), // sub_14185ac40
    Replacement::draft(0x14185acd0, "unknown_14185acd0", fn_14185acd0 as *const ()), // sub_14185acd0
    Replacement::draft_throws(0x14185ad80, "unknown_14185ad80", fn_14185ad80 as *const (), Throws::Discard), // sub_14185ad80
    Replacement::draft_throws(0x14185aef0, "unknown_14185aef0", fn_14185aef0 as *const (), Throws::Discard), // sub_14185aef0
    Replacement::draft_throws(0x14185b040, "unknown_14185b040", fn_14185b040 as *const (), Throws::Caught), // sub_14185b040
    Replacement::draft_throws(0x14185b400, "unknown_14185b400", fn_14185b400 as *const (), Throws::Discard), // sub_14185b400
    Replacement::draft_throws(0x14185b460, "unknown_14185b460", fn_14185b460 as *const (), Throws::Discard), // sub_14185b460
    Replacement::draft_throws(0x14185b6f0, "unknown_14185b6f0", fn_14185b6f0 as *const (), Throws::Discard), // sub_14185b6f0
    Replacement::draft(0x14185b790, "unknown_14185b790", fn_14185b790 as *const ()), // sub_14185b790
    Replacement::draft_throws(0x14185bae0, "unknown_14185bae0", fn_14185bae0 as *const (), Throws::Caught), // sub_14185bae0
    Replacement::draft(0x14185bf30, "unknown_14185bf30", fn_14185bf30 as *const ()), // sub_14185bf30
    Replacement::draft_throws(0x14185bff0, "unknown_14185bff0", fn_14185bff0 as *const (), Throws::Discard), // sub_14185bff0
    Replacement::draft(0x14185c0e0, "unknown_14185c0e0", fn_14185c0e0 as *const ()), // sub_14185c0e0
    Replacement::draft_throws(0x14185c670, "unknown_14185c670", tpl_34423::<0x6f8, 0x141f78b90> as *const (), Throws::Discard), // sub_14185c670
    Replacement::draft_throws(0x14185c690, "unknown_14185c690", fn_14185c690 as *const (), Throws::Caught), // sub_14185c690
    Replacement::draft_throws(0x14185d370, "unknown_14185d370", fn_14185d370 as *const (), Throws::Caught), // sub_14185d370
    Replacement::draft(0x14185da20, "unknown_14185da20", fn_14185da20 as *const ()), // sub_14185da20
    Replacement::draft_throws(0x14185db40, "unknown_14185db40", fn_14185db40 as *const (), Throws::Caught), // sub_14185db40
    Replacement::draft(0x14185dd10, "unknown_14185dd10", fn_14185dd10 as *const ()), // sub_14185dd10
    Replacement::draft_throws(0x14185de70, "worldloadinglogic_14185de70", fn_14185de70 as *const (), Throws::Caught), // WorldLoadingLogic::onEnterLoadingWorld::InstantiateWorldJob::WorldLoadingLogic::onEnterLoadingWorld::InstantiateWorldJob
    Replacement::draft(0x14185dff0, "worldloadinglogic_14185dff0", fn_14185dff0 as *const ()), // sub_14185dff0
    Replacement::draft(0x14185e030, "worldloadinglogic_14185e030", tpl_34429::<0x2c> as *const ()), // sub_14185e030
    Replacement::draft(0x14185e040, "worldloadinglogic_14185e040", tpl_34429::<0x2c> as *const ()), // sub_14185e040
    Replacement::draft(0x14185e050, "worldloadinglogic_14185e050", tpl_34429::<0x28> as *const ()), // sub_14185e050
    Replacement::draft(0x14185e060, "worldloadinglogic_14185e060", tpl_34396::<0x143a00240, 0x58> as *const ()), // sub_14185e060
    Replacement::draft_throws(0x14185e080, "worldloadinglogic_14185e080", fn_14185e080 as *const (), Throws::Caught), // sub_14185e080
    Replacement::draft(0x14185f8a0, "worldloadinglogic_14185f8a0", fn_14185f8a0 as *const ()), // sub_14185f8a0
    Replacement::draft_throws(0x14185f8d0, "worldloadinglogic_14185f8d0", fn_14185f8d0 as *const (), Throws::Discard), // sub_14185f8d0
    Replacement::draft(0x14185f950, "worldloadinglogic_14185f950", tpl_34396::<0x14397fea8, 0x38> as *const ()), // sub_14185f950
    Replacement::draft(0x14185f970, "worldloadinglogic_14185f970", fn_14185f970 as *const ()), // WorldLoadingLogic::WorldLoadingLogic::vfunc_13
    Replacement::draft_throws(0x14185f9a0, "worldloadinglogic_14185f9a0", tpl_793::<0x142407c80, 0x143c4aa00, 0x128, 0x760, 0x1e0, 0x1e2, 0x141859b70, 0x143a0f4d8, 0x108, 0x142416f10> as *const (), Throws::Discard), // WorldLoadingLogic::WorldLoadingLogic::vfunc_11
    Replacement::draft(0x14185fa30, "worldloadinglogic_14185fa30", fn_14185fa30 as *const ()), // WorldLoadingLogic::WorldLoadingLogic::vfunc_10
    Replacement::draft_throws(0x14185fb10, "worldloadinglogic_14185fb10", fn_14185fb10 as *const (), Throws::Caught), // WorldLoadingLogic::WorldLoadingLogic::vfunc_9
    Replacement::draft(0x14185fdc0, "worldloadinglogic_14185fdc0", fn_14185fdc0 as *const ()), // WorldLoadingLogic::WorldLoadingLogic::vfunc_8
    Replacement::draft(0x14185ff70, "worldloadinglogic_14185ff70", fn_14185ff70 as *const ()), // WorldLoadingLogic::WorldLoadingLogic::vfunc_7
    Replacement::draft(0x141860000, "worldloadinglogic_141860000", fn_141860000 as *const ()), // WorldLoadingLogic::WorldLoadingLogic::vfunc_6
    Replacement::draft(0x1418601b0, "worldloadinglogic_1418601b0", fn_1418601b0 as *const ()), // WorldLoadingLogic::WorldLoadingLogic::vfunc_5
    Replacement::draft(0x141860250, "worldloadinglogic_141860250", fn_141860250 as *const ()), // WorldLoadingLogic::WorldLoadingLogic::vfunc_4
    Replacement::draft_throws(0x141860450, "worldloadinglogic_141860450", fn_141860450 as *const (), Throws::Caught), // WorldLoadingLogic::WorldLoadingLogic::vfunc_2
    Replacement::draft(0x141860630, "unknown_141860630", fn_141860630 as *const ()), // sub_141860630
    Replacement::draft(0x141860750, "unknown_141860750", fn_141860750 as *const ()), // sub_141860750
    Replacement::draft_throws(0x141860860, "unknown_141860860", fn_141860860 as *const (), Throws::Discard), // sub_141860860
    Replacement::draft(0x141861640, "unknown_141861640", tpl_369 as *const ()), // sub_141861640
    Replacement::draft(0x141861650, "std_141861650", tpl_976::<0x141834d30> as *const ()), // std::_Ref_count<anonymous_namespace::GlobalContextStorage>::std::_Ref_count<anonymous_namespace::GlobalContextStorage>::
    Replacement::draft(0x141861680, "std_141861680", tpl_34443 as *const ()), // sub_141861680
    Replacement::draft(0x141861690, "std_141861690", tpl_911::<0x18> as *const ()), // sub_141861690
    Replacement::draft(0x1418616b0, "std_1418616b0", fn_1418616b0 as *const ()), // sub_1418616b0
    Replacement::draft(0x1418616e0, "std_1418616e0", tpl_964::<0x10, 0x18> as *const ()), // std::_Ref_count<anonymous_namespace::GlobalContextStorage>::std::_Ref_count<anonymous_namespace::GlobalContextStorage>::
    Replacement::draft(0x141861700, "std_141861700", tpl_961::<0x10> as *const ()), // std::_Ref_count<anonymous_namespace::GlobalContextStorage>::std::_Ref_count<anonymous_namespace::GlobalContextStorage>::
    Replacement::draft(0x141861720, "unknown_141861720", tpl_895::<0x8> as *const ()), // sub_141861720
    Replacement::draft_throws(0x141861800, "unknown_141861800", fn_141861800 as *const (), Throws::Discard), // sub_141861800
    Replacement::draft_throws(0x1418618a0, "unknown_1418618a0", fn_1418618a0 as *const (), Throws::Discard), // sub_1418618a0
    Replacement::draft_throws(0x141861930, "opentelemetry_141861930", fn_141861930 as *const (), Throws::Caught), // sub_141861930
    Replacement::draft_throws(0x141861a40, "opentelemetry_141861a40", fn_141861a40 as *const (), Throws::Caught), // sub_141861a40
    Replacement::draft(0x141861be0, "opentelemetry_141861be0", tpl_34448::<0x10, 0x18, 0x20> as *const ()), // sub_141861be0
    Replacement::draft_throws(0x141861c00, "opentelemetry_141861c00", tpl_34449::<0x141862900, 0x141862470> as *const (), Throws::Discard), // sub_141861c00
    Replacement::draft_throws(0x141861c30, "opentelemetry_141861c30", fn_141861c30 as *const (), Throws::Discard), // sub_141861c30
    Replacement::draft_throws(0x141861cd0, "opentelemetry_141861cd0", fn_141861cd0 as *const (), Throws::Discard), // sub_141861cd0
    Replacement::draft_throws(0x141861d70, "opentelemetry_141861d70", fn_141861d70 as *const (), Throws::Discard), // sub_141861d70
    Replacement::draft(0x141861f50, "opentelemetry_141861f50", tpl_34448::<0x10, 0x18, 0x20> as *const ()), // sub_141861f50
    Replacement::draft(0x141861f70, "opentelemetry_141861f70", fn_141861f70 as *const ()), // sub_141861f70
    Replacement::draft_throws(0x141862030, "opentelemetry_141862030", fn_141862030 as *const (), Throws::Discard), // sub_141862030
    Replacement::draft_throws(0x1418620f0, "opentelemetry_1418620f0", fn_1418620f0 as *const (), Throws::Caught), // sub_1418620f0
    Replacement::draft_throws(0x1418623d0, "opentelemetry_1418623d0", fn_1418623d0 as *const (), Throws::Discard), // sub_1418623d0
    Replacement::draft_throws(0x141862440, "opentelemetry_141862440", fn_141862440 as *const (), Throws::Discard), // sub_141862440
    Replacement::draft_throws(0x141862470, "opentelemetry_141862470", fn_141862470 as *const (), Throws::Caught), // sub_141862470
    Replacement::draft_throws(0x141862900, "opentelemetry_141862900", fn_141862900 as *const (), Throws::Caught), // sub_141862900
    Replacement::draft(0x141863120, "anonymous_namespace_141863120", fn_141863120 as *const ()), // anonymous_namespace::GlobalContextStorage::anonymous_namespace::GlobalContextStorage::vfunc_3
    Replacement::draft_throws(0x1418631c0, "anonymous_namespace_1418631c0", fn_1418631c0 as *const (), Throws::Discard), // anonymous_namespace::GlobalContextStorage::anonymous_namespace::GlobalContextStorage::vfunc_1
    Replacement::draft_throws(0x141863310, "anonymous_namespace_141863310", fn_141863310 as *const (), Throws::Discard), // anonymous_namespace::GlobalContextStorage::anonymous_namespace::GlobalContextStorage::vfunc_2
    Replacement::draft_throws(0x1418633c0, "anonymous_namespace_1418633c0", fn_1418633c0 as *const (), Throws::Discard), // anonymous_namespace::GlobalContextStorage::anonymous_namespace::GlobalContextStorage::vfunc_0
    Replacement::draft(0x141863480, "unknown_141863480", fn_141863480 as *const ()), // sub_141863480
    Replacement::draft_throws(0x141863500, "unknown_141863500", fn_141863500 as *const (), Throws::Discard), // sub_141863500
    Replacement::draft_throws(0x141863610, "unknown_141863610", fn_141863610 as *const (), Throws::Caught), // sub_141863610
    Replacement::draft_throws(0x141863720, "unknown_141863720", fn_141863720 as *const (), Throws::Discard), // sub_141863720
    Replacement::draft_throws(0x1418637f0, "unknown_1418637f0", fn_1418637f0 as *const (), Throws::Discard), // sub_1418637f0
    Replacement::draft(0x141863900, "unknown_141863900", fn_141863900 as *const ()), // sub_141863900
    Replacement::draft_throws(0x141863940, "unknown_141863940", fn_141863940 as *const (), Throws::Caught), // sub_141863940
    Replacement::draft_throws(0x141863a20, "opentelemetry_141863a20", fn_141863a20 as *const (), Throws::Discard), // sub_141863a20
    Replacement::draft(0x141863a40, "unknown_141863a40", tpl_361 as *const ()), // sub_141863a40
    Replacement::draft(0x141863a50, "unknown_141863a50", tpl_34259 as *const ()), // sub_141863a50
    Replacement::draft(0x141863a70, "unknown_141863a70", fn_141863a70 as *const ()), // sub_141863a70
    Replacement::draft(0x141863af0, "unknown_141863af0", tpl_34469::<0x143080b30, 0x8, 0x1401f3d90, 0x10, 0x18> as *const ()), // sub_141863af0
    Replacement::draft(0x141863b50, "app_141863b50", fn_141863b50 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<HWND__*,app::WindowWin32::WindowWin32::<lambda_1>>::ap
    Replacement::draft(0x141863b90, "app_141863b90", tpl_34471::<0x1401f3d00, 0x141834d30> as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::WindowWin32::~WindowWin32::<lambda_1>>::app:
    Replacement::draft(0x141863bf0, "app_141863bf0", tpl_4771::<0x38, 0x1401f4c10, 0x142442de0> as *const ()), // sub_141863bf0
    Replacement::draft(0x141863c30, "app_141863c30", tpl_34472::<0x38, 0x40, 0x8, 0x10> as *const ()), // sub_141863c30
    Replacement::draft(0x141863c50, "app_141863c50", fn_141863c50 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<HWND__*,app::WindowWin32::WindowWin32::<lambda_1>>::ap
    Replacement::draft(0x141863d40, "app_141863d40", tpl_4771::<0x10, 0x1401f3d00, 0x142442de0> as *const ()), // sub_141863d40
    Replacement::draft(0x141863d80, "app_141863d80", tpl_34474::<0x10, 0x18, 0x8> as *const ()), // sub_141863d80
    Replacement::draft(0x141863da0, "app_141863da0", fn_141863da0 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::WindowWin32::~WindowWin32::<lambda_1>>::app:
    Replacement::draft(0x141863dd0, "app_141863dd0", fn_141863dd0 as *const ()), // sub_141863dd0
    Replacement::draft(0x141863e00, "app_141863e00", fn_141863e00 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_33
    Replacement::draft(0x141863f50, "app_141863f50", fn_141863f50 as *const ()), // sub_141863f50
    Replacement::draft(0x141864040, "app_141864040", fn_141864040 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_32
    Replacement::draft(0x141864170, "app_141864170", fn_141864170 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_31
    Replacement::draft(0x1418641c0, "app_1418641c0", fn_1418641c0 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_30
    Replacement::draft(0x1418641f0, "app_1418641f0", tpl_34482::<0x8, 0x1426461c0> as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_29
    Replacement::draft(0x141864210, "app_141864210", fn_141864210 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_28
    Replacement::draft(0x141864260, "app_141864260", fn_141864260 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_12
    Replacement::draft(0x1418642a0, "app_1418642a0", fn_1418642a0 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_11
    Replacement::draft(0x1418642d0, "app_1418642d0", fn_1418642d0 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_10
    Replacement::draft(0x141864330, "app_141864330", fn_141864330 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_9
    Replacement::draft(0x141864360, "app_141864360", fn_141864360 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_8
    Replacement::draft(0x141864390, "app_141864390", fn_141864390 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_7
    Replacement::draft(0x1418643c0, "app_1418643c0", fn_1418643c0 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_6
    Replacement::draft(0x141864400, "app_141864400", fn_141864400 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_5
    Replacement::draft(0x141864520, "app_141864520", fn_141864520 as *const ()), // sub_141864520
    Replacement::draft(0x141864640, "app_141864640", fn_141864640 as *const ()), // sub_141864640
    Replacement::draft(0x141864660, "app_141864660", fn_141864660 as *const ()), // sub_141864660
    Replacement::draft(0x1418646a0, "app_1418646a0", fn_1418646a0 as *const ()), // sub_1418646a0
    Replacement::draft(0x1418646c0, "app_1418646c0", fn_1418646c0 as *const ()), // sub_1418646c0
    Replacement::draft(0x141864700, "app_141864700", fn_141864700 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_17
    Replacement::draft(0x141864720, "app_141864720", fn_141864720 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_16
    Replacement::draft(0x141864740, "app_141864740", fn_141864740 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_15
    Replacement::draft(0x141864770, "app_141864770", fn_141864770 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_14
    Replacement::draft(0x141864800, "app_141864800", fn_141864800 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_13
    Replacement::draft(0x141864830, "app_141864830", fn_141864830 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_20
    Replacement::draft(0x141864870, "app_141864870", fn_141864870 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_19
    Replacement::draft(0x1418648b0, "app_1418648b0", fn_1418648b0 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_18
    Replacement::draft(0x141864980, "app_141864980", fn_141864980 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_4
    Replacement::draft(0x141864990, "app_141864990", fn_141864990 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_3
    Replacement::draft_throws(0x141864ac0, "app_141864ac0", fn_141864ac0 as *const (), Throws::Discard), // app::WindowWin32::app::WindowWin32::vfunc_2
    Replacement::draft(0x141864bc0, "app_141864bc0", fn_141864bc0 as *const ()), // app::WindowWin32::app::WindowWin32::vfunc_1
    Replacement::draft(0x141864cf0, "app_141864cf0", fn_141864cf0 as *const ()), // sub_141864cf0
    Replacement::draft(0x141864d30, "app_141864d30", fn_141864d30 as *const ()), // sub_141864d30
    Replacement::draft(0x141864e10, "app_141864e10", fn_141864e10 as *const ()), // sub_141864e10
    Replacement::draft(0x141864fb0, "app_141864fb0", fn_141864fb0 as *const ()), // sub_141864fb0
    Replacement::draft(0x141864fe0, "app_141864fe0", fn_141864fe0 as *const ()), // sub_141864fe0
    Replacement::draft_throws(0x1418651c0, "app_1418651c0", fn_1418651c0 as *const (), Throws::Caught), // sub_1418651c0
    Replacement::draft(0x1418654a0, "app_1418654a0", fn_1418654a0 as *const ()), // sub_1418654a0
    Replacement::draft(0x141865580, "app_141865580", fn_141865580 as *const ()), // sub_141865580
    Replacement::draft(0x1418655b0, "app_1418655b0", fn_1418655b0 as *const ()), // sub_1418655b0
    Replacement::draft(0x141865660, "app_141865660", fn_141865660 as *const ()), // sub_141865660
    Replacement::draft_throws(0x1418656b0, "app_1418656b0", fn_1418656b0 as *const (), Throws::Discard), // sub_1418656b0
    Replacement::draft(0x141865750, "app_141865750", tpl_34469::<0x143080bd8, 0x8, 0x1401f3d90, 0x10, 0x18> as *const ()), // sub_141865750
    Replacement::draft(0x1418657c0, "app_1418657c0", fn_1418657c0 as *const ()), // sub_1418657c0
    Replacement::draft(0x141865820, "app_141865820", tpl_34471::<0x1401f3d00, 0x141834d30> as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::SystemEventQueueWindowsDesktop::refreshCurso
    Replacement::draft(0x141865860, "app_141865860", tpl_34471::<0x1401f3d00, 0x141834d30> as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::SystemEventQueueWindowsDesktop::showCursor::
    Replacement::draft(0x1418658c0, "app_1418658c0", tpl_4771::<0x10, 0x1401f3d00, 0x142442de0> as *const ()), // sub_1418658c0
    Replacement::draft(0x141865900, "app_141865900", tpl_34474::<0x10, 0x18, 0x8> as *const ()), // sub_141865900
    Replacement::draft(0x141865920, "app_141865920", fn_141865920 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::SystemEventQueueWindowsDesktop::refreshCurso
    Replacement::draft(0x1418659f0, "app_1418659f0", tpl_4771::<0x10, 0x1401f3d00, 0x142442de0> as *const ()), // sub_1418659f0
    Replacement::draft(0x141865a30, "app_141865a30", tpl_34474::<0x10, 0x18, 0x8> as *const ()), // sub_141865a30
    Replacement::draft(0x141865a50, "app_141865a50", fn_141865a50 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::SystemEventQueueWindowsDesktop::showCursor::
    Replacement::draft(0x141865aa0, "app_141865aa0", tpl_34520::<0x8, 0x142645000> as *const ()), // sub_141865aa0
    Replacement::draft(0x141865ac0, "app_141865ac0", fn_141865ac0 as *const ()), // sub_141865ac0
    Replacement::draft(0x141865b10, "app_141865b10", fn_141865b10 as *const ()), // sub_141865b10
    Replacement::draft(0x141865b60, "app_141865b60", fn_141865b60 as *const ()), // sub_141865b60
    Replacement::draft(0x141865bb0, "app_141865bb0", fn_141865bb0 as *const ()), // sub_141865bb0
    Replacement::draft(0x141865d70, "app_141865d70", fn_141865d70 as *const ()), // sub_141865d70
    Replacement::draft(0x141865e50, "app_141865e50", fn_141865e50 as *const ()), // sub_141865e50
    Replacement::draft(0x141865f50, "app_141865f50", fn_141865f50 as *const ()), // sub_141865f50
    Replacement::draft(0x141866070, "app_141866070", fn_141866070 as *const ()), // sub_141866070
    Replacement::draft(0x141866160, "app_141866160", fn_141866160 as *const ()), // sub_141866160
    Replacement::draft(0x141866230, "app_141866230", fn_141866230 as *const ()), // sub_141866230
    Replacement::draft(0x1418662a0, "app_1418662a0", fn_1418662a0 as *const ()), // sub_1418662a0
    Replacement::draft(0x141866380, "app_141866380", fn_141866380 as *const ()), // sub_141866380
    Replacement::draft(0x141866470, "app_141866470", fn_141866470 as *const ()), // sub_141866470
    Replacement::draft(0x141866550, "app_141866550", fn_141866550 as *const ()), // sub_141866550
    Replacement::draft(0x141866590, "app_141866590", fn_141866590 as *const ()), // sub_141866590
    Replacement::draft(0x1418666e0, "app_1418666e0", fn_1418666e0 as *const ()), // sub_1418666e0
    Replacement::draft(0x141866720, "app_141866720", fn_141866720 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_13
    Replacement::draft(0x141866870, "app_141866870", fn_141866870 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_12
    Replacement::draft(0x1418669f0, "app_1418669f0", fn_1418669f0 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_11
    Replacement::draft(0x141866b80, "app_141866b80", fn_141866b80 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_10
    Replacement::draft(0x141866bb0, "app_141866bb0", fn_141866bb0 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_9
    Replacement::draft(0x141866cb0, "app_141866cb0", fn_141866cb0 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_8
    Replacement::draft(0x141866d40, "app_141866d40", fn_141866d40 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_7
    Replacement::draft(0x141867840, "app_141867840", fn_141867840 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_3
    Replacement::draft(0x141867870, "app_141867870", fn_141867870 as *const ()), // sub_141867870
    Replacement::draft(0x141867880, "app_141867880", fn_141867880 as *const ()), // app::SystemEventQueueWindowsDesktop::app::SystemEventQueueWindowsDesktop::vfunc_2
    Replacement::draft(0x141867970, "app_141867970", fn_141867970 as *const ()), // sub_141867970
    Replacement::draft_throws(0x141867a50, "app_141867a50", fn_141867a50 as *const (), Throws::Discard), // sub_141867a50
    Replacement::draft(0x141867b60, "app_141867b60", fn_141867b60 as *const ()), // sub_141867b60
    Replacement::draft_throws(0x141867bb0, "app_141867bb0", fn_141867bb0 as *const (), Throws::Caught), // sub_141867bb0
    Replacement::draft_throws(0x141868040, "app_141868040", fn_141868040 as *const (), Throws::Discard), // sub_141868040
    Replacement::draft(0x141868150, "app_141868150", fn_141868150 as *const ()), // sub_141868150
    Replacement::draft(0x141868170, "app_141868170", tpl_34520::<0x8, 0x142645000> as *const ()), // sub_141868170
    Replacement::draft(0x1418681d0, "app_1418681d0", fn_1418681d0 as *const ()), // sub_1418681d0
    Replacement::draft(0x1418682d0, "app_1418682d0", fn_1418682d0 as *const ()), // sub_1418682d0
    Replacement::draft(0x1418683c0, "app_1418683c0", fn_1418683c0 as *const ()), // sub_1418683c0
    Replacement::draft(0x141868410, "app_141868410", tpl_34471::<0x1401f3d00, 0x141834d30> as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::SystemEventQueueWindows::stopMessagePump::<l
    Replacement::draft(0x141868460, "app_141868460", tpl_4771::<0x10, 0x1401f3d00, 0x142442de0> as *const ()), // sub_141868460
    Replacement::draft(0x1418684a0, "app_1418684a0", tpl_34474::<0x10, 0x18, 0x8> as *const ()), // sub_1418684a0
    Replacement::draft(0x1418684c0, "app_1418684c0", fn_1418684c0 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::SystemEventQueueWindows::stopMessagePump::<l
    Replacement::draft(0x1418684f0, "app_1418684f0", fn_1418684f0 as *const ()), // sub_1418684f0
    Replacement::draft(0x1418685f0, "app_1418685f0", fn_1418685f0 as *const ()), // sub_1418685f0
    Replacement::draft(0x1418687b0, "app_1418687b0", fn_1418687b0 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::app::SystemEventQueueWindows::MessagePumpThread::vfunc_1
    Replacement::draft_throws(0x1418689f0, "app_1418689f0", fn_1418689f0 as *const (), Throws::Discard), // sub_1418689f0
    Replacement::draft(0x141868a30, "app_141868a30", fn_141868a30 as *const ()), // sub_141868a30
    Replacement::draft(0x141868b40, "app_141868b40", fn_141868b40 as *const ()), // sub_141868b40
    Replacement::draft(0x141868b60, "app_141868b60", fn_141868b60 as *const ()), // sub_141868b60
    Replacement::draft(0x141868bb0, "app_141868bb0", fn_141868bb0 as *const ()), // sub_141868bb0
    Replacement::draft(0x141868c50, "app_141868c50", fn_141868c50 as *const ()), // sub_141868c50
    Replacement::draft(0x141868d30, "app_141868d30", fn_141868d30 as *const ()), // sub_141868d30
    Replacement::draft(0x141868e60, "app_141868e60", fn_141868e60 as *const ()), // sub_141868e60
    Replacement::draft(0x141868eb0, "app_141868eb0", fn_141868eb0 as *const ()), // sub_141868eb0
    Replacement::draft(0x141868f00, "app_141868f00", fn_141868f00 as *const ()), // sub_141868f00
    Replacement::draft(0x1418690c0, "app_1418690c0", fn_1418690c0 as *const ()), // sub_1418690c0
    Replacement::draft(0x141869250, "app_141869250", fn_141869250 as *const ()), // sub_141869250
    Replacement::draft(0x141869400, "app_141869400", fn_141869400 as *const ()), // sub_141869400
    Replacement::draft(0x1418695f0, "app_1418695f0", fn_1418695f0 as *const ()), // sub_1418695f0
    Replacement::draft(0x141869700, "app_141869700", fn_141869700 as *const ()), // sub_141869700
    Replacement::draft_throws(0x141869eb0, "app_141869eb0", fn_141869eb0 as *const (), Throws::Caught), // sub_141869eb0
    Replacement::draft(0x141869fc0, "app_141869fc0", fn_141869fc0 as *const ()), // app::SystemEventQueueWindows::app::SystemEventQueueWindows::vfunc_1
    Replacement::draft_throws(0x14186a0c0, "app_14186a0c0", fn_14186a0c0 as *const (), Throws::Discard), // sub_14186a0c0
    Replacement::draft(0x14186a150, "app_14186a150", fn_14186a150 as *const ()), // sub_14186a150
    Replacement::draft(0x14186a310, "app_14186a310", fn_14186a310 as *const ()), // sub_14186a310
    Replacement::draft(0x14186a450, "app_14186a450", fn_14186a450 as *const ()), // sub_14186a450
    Replacement::draft(0x14186a560, "app_14186a560", fn_14186a560 as *const ()), // sub_14186a560
    Replacement::draft(0x14186a670, "app_14186a670", fn_14186a670 as *const ()), // sub_14186a670
    Replacement::draft(0x14186a7a0, "app_14186a7a0", fn_14186a7a0 as *const ()), // sub_14186a7a0
    Replacement::draft(0x14186a8d0, "app_14186a8d0", fn_14186a8d0 as *const ()), // sub_14186a8d0
    Replacement::draft(0x14186a960, "app_14186a960", fn_14186a960 as *const ()), // sub_14186a960
    Replacement::draft(0x14186aa10, "app_14186aa10", fn_14186aa10 as *const ()), // sub_14186aa10
    Replacement::draft(0x14186aa60, "app_14186aa60", fn_14186aa60 as *const ()), // sub_14186aa60
    Replacement::draft(0x14186aa90, "app_14186aa90", fn_14186aa90 as *const ()), // sub_14186aa90
    Replacement::draft(0x14186aad0, "app_14186aad0", fn_14186aad0 as *const ()), // sub_14186aad0
    Replacement::draft(0x14186ac00, "app_14186ac00", tpl_34584::<0x143080d28, 0x8, 0x14020b850, 0x18, 0x20> as *const ()), // sub_14186ac00
    Replacement::draft(0x14186ac70, "app_14186ac70", tpl_34584::<0x143080d88, 0x8, 0x1401f3d90, 0x18, 0x20> as *const ()), // sub_14186ac70
    Replacement::draft(0x14186acd0, "app_14186acd0", tpl_34585::<0x14020b7e0, 0x141834d30> as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<bool,app::DropTarget::DropTarget::<lambda_1>>::app::Sy
    Replacement::draft(0x14186ad10, "app_14186ad10", tpl_34585::<0x1401f3d00, 0x141834d30> as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::DropTarget::~DropTarget::<lambda_1>>::app::S
    Replacement::draft(0x14186ad70, "app_14186ad70", tpl_4771::<0x18, 0x14020b7e0, 0x142442de0> as *const ()), // sub_14186ad70
    Replacement::draft(0x14186adb0, "app_14186adb0", tpl_34472::<0x18, 0x20, 0x8, 0x4> as *const ()), // sub_14186adb0
    Replacement::draft(0x14186add0, "app_14186add0", fn_14186add0 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<bool,app::DropTarget::DropTarget::<lambda_1>>::app::Sy
    Replacement::draft(0x14186ae20, "app_14186ae20", tpl_4771::<0x18, 0x1401f3d00, 0x142442de0> as *const ()), // sub_14186ae20
    Replacement::draft(0x14186ae60, "app_14186ae60", tpl_34474::<0x18, 0x20, 0x8> as *const ()), // sub_14186ae60
    Replacement::draft(0x14186ae80, "app_14186ae80", fn_14186ae80 as *const ()), // app::SystemEventQueueWindows::MessagePumpThread::UserFunctionImpl<void,app::DropTarget::~DropTarget::<lambda_1>>::app::S
    Replacement::draft(0x14186aec0, "app_14186aec0", fn_14186aec0 as *const ()), // app::DropTarget::app::DropTarget::vfunc_2
    Replacement::draft(0x14186aed0, "app_14186aed0", tpl_30071::<0x8> as *const ()), // app::DropTarget::app::DropTarget::vfunc_1
    Replacement::draft(0x14186aee0, "app_14186aee0", fn_14186aee0 as *const ()), // app::DropTarget::app::DropTarget::vfunc_0
    Replacement::draft(0x14186af40, "app_14186af40", fn_14186af40 as *const ()), // app::DropTarget::app::DropTarget::vfunc_6
    Replacement::draft(0x14186af90, "app_14186af90", fn_14186af90 as *const ()), // app::DropTarget::app::DropTarget::vfunc_5
    Replacement::draft(0x14186afb0, "app_14186afb0", fn_14186afb0 as *const ()), // app::DropTarget::app::DropTarget::vfunc_4
    Replacement::draft_throws(0x14186b010, "app_14186b010", fn_14186b010 as *const (), Throws::Caught), // app::DropTarget::app::DropTarget::vfunc_3
    Replacement::draft(0x14186b4e0, "app_14186b4e0", fn_14186b4e0 as *const ()), // sub_14186b4e0
    Replacement::draft(0x14186b520, "app_14186b520", fn_14186b520 as *const ()), // sub_14186b520
    Replacement::draft(0x14186b620, "app_14186b620", fn_14186b620 as *const ()), // sub_14186b620
    Replacement::draft(0x14186b660, "app_14186b660", fn_14186b660 as *const ()), // sub_14186b660
    Replacement::draft(0x14186b770, "app_14186b770", fn_14186b770 as *const ()), // sub_14186b770
    Replacement::draft_throws(0x14186b860, "app_14186b860", fn_14186b860 as *const (), Throws::Caught), // sub_14186b860
    Replacement::draft(0x14186b8e0, "app_14186b8e0", fn_14186b8e0 as *const ()), // sub_14186b8e0
    Replacement::draft(0x14186b930, "app_14186b930", fn_14186b930 as *const ()), // sub_14186b930
    Replacement::draft(0x14186b980, "app_14186b980", fn_14186b980 as *const ()), // sub_14186b980
    Replacement::draft(0x14186b9f0, "app_14186b9f0", tpl_34602::<0x50, 0x48, 0x8, 0x14183acc8> as *const ()), // sub_14186b9f0
    Replacement::draft(0x14186ba40, "app_14186ba40", tpl_34603::<0x48, 0x8, 0xc, 0x14020bcb0> as *const ()), // sub_14186ba40
    Replacement::draft_throws(0x14186baa0, "app_14186baa0", fn_14186baa0 as *const (), Throws::Discard), // sub_14186baa0
    Replacement::draft(0x14186bae0, "app_14186bae0", fn_14186bae0 as *const ()), // app::EventHandler::app::EventHandler::vfunc_1
    Replacement::draft(0x14186bc70, "unknown_14186bc70", tpl_34605::<0x143084cd0> as *const ()), // sub_14186bc70
    Replacement::draft(0x14186be30, "unknown_14186be30", fn_14186be30 as *const ()), // sub_14186be30
    Replacement::draft(0x14186c260, "unknown_14186c260", fn_14186c260 as *const ()), // sub_14186c260
    Replacement::draft(0x14186c3a0, "unknown_14186c3a0", fn_14186c3a0 as *const ()), // sub_14186c3a0
    Replacement::draft(0x14186c460, "unknown_14186c460", fn_14186c460 as *const ()), // sub_14186c460
    Replacement::draft(0x14186c4b0, "unknown_14186c4b0", tpl_34607::<0x1401de830> as *const ()), // sub_14186c4b0
    Replacement::draft(0x14186c560, "unknown_14186c560", fn_14186c560 as *const ()), // sub_14186c560
    Replacement::draft(0x14186c760, "unknown_14186c760", fn_14186c760 as *const ()), // sub_14186c760
    Replacement::draft(0x14186c890, "unknown_14186c890", fn_14186c890 as *const ()), // sub_14186c890
    Replacement::draft(0x14186c8c0, "unknown_14186c8c0", tpl_34610::<0x20> as *const ()), // sub_14186c8c0
    Replacement::draft(0x14186c960, "unknown_14186c960", tpl_34611::<0x8> as *const ()), // sub_14186c960
    Replacement::draft(0x14186ca80, "unknown_14186ca80", tpl_34612 as *const ()), // sub_14186ca80
    Replacement::draft(0x14186cb10, "unknown_14186cb10", tpl_34611::<0x8> as *const ()), // sub_14186cb10
    Replacement::draft(0x14186cc30, "unknown_14186cc30", tpl_34613 as *const ()), // sub_14186cc30
    Replacement::draft(0x14186cd40, "unknown_14186cd40", fn_14186cd40 as *const ()), // sub_14186cd40
    Replacement::draft(0x14186cee0, "unknown_14186cee0", tpl_34615 as *const ()), // sub_14186cee0
    Replacement::draft(0x14186d600, "unknown_14186d600", fn_14186d600 as *const ()), // sub_14186d600
    Replacement::draft(0x14186d650, "unknown_14186d650", fn_14186d650 as *const ()), // sub_14186d650
    Replacement::draft(0x14186d680, "unknown_14186d680", fn_14186d680 as *const ()), // sub_14186d680
    Replacement::draft(0x14186d700, "unknown_14186d700", fn_14186d700 as *const ()), // sub_14186d700
    Replacement::draft_throws(0x14186d750, "unknown_14186d750", fn_14186d750 as *const (), Throws::Discard), // sub_14186d750
    Replacement::draft(0x14186dc00, "unknown_14186dc00", fn_14186dc00 as *const ()), // sub_14186dc00
    Replacement::draft(0x14186dc80, "unknown_14186dc80", fn_14186dc80 as *const ()), // sub_14186dc80
    Replacement::draft(0x14186dce0, "unknown_14186dce0", fn_14186dce0 as *const ()), // sub_14186dce0
    Replacement::draft(0x14186dd40, "unknown_14186dd40", tpl_34624::<0x143c8d5f4, 0x14020e8f0, 0x143c8d4b4, 0x143c8d46c> as *const ()), // sub_14186dd40
    Replacement::draft(0x14186dfb0, "unknown_14186dfb0", fn_14186dfb0 as *const ()), // sub_14186dfb0
    Replacement::draft(0x14186e0b0, "unknown_14186e0b0", fn_14186e0b0 as *const ()), // sub_14186e0b0
    Replacement::draft(0x14186e2a0, "unknown_14186e2a0", tpl_34627::<0x18, 0x143427080, 0x143843da8, 0x143843c20, 0x143843c60> as *const ()), // sub_14186e2a0
    Replacement::draft(0x14186e2e0, "unknown_14186e2e0", tpl_34627::<0x1c, 0x143427080, 0x143843c40, 0x143843c80, 0x143843d88> as *const ()), // sub_14186e2e0
    Replacement::draft_throws(0x14186e320, "unknown_14186e320", fn_14186e320 as *const (), Throws::Discard), // sub_14186e320
    Replacement::draft(0x14186ea50, "unknown_14186ea50", fn_14186ea50 as *const ()), // sub_14186ea50
    Replacement::draft(0x14186eb00, "unknown_14186eb00", fn_14186eb00 as *const ()), // sub_14186eb00
    Replacement::draft(0x14186eb30, "unknown_14186eb30", fn_14186eb30 as *const ()), // sub_14186eb30
    Replacement::draft(0x14186eb90, "unknown_14186eb90", fn_14186eb90 as *const ()), // sub_14186eb90
    Replacement::draft(0x14186ebe0, "unknown_14186ebe0", tpl_34633::<0x143843e48, 0x143843cd0, 0x143843bd0> as *const ()), // sub_14186ebe0
    Replacement::draft(0x14186ec20, "unknown_14186ec20", tpl_34633::<0x143843e68, 0x143843e28, 0x143843e88> as *const ()), // sub_14186ec20
    Replacement::draft(0x14186ec60, "unknown_14186ec60", tpl_34633::<0x143843cf0, 0x143843e08, 0x143843dc8> as *const ()), // sub_14186ec60
    Replacement::draft(0x14186eca0, "unknown_14186eca0", fn_14186eca0 as *const ()), // sub_14186eca0
    Replacement::draft(0x14186ed10, "bigfish_14186ed10", tpl_34635::<0x143080f10, 0x2, 0x14020f270> as *const ()), // bigfish::world_interaction::SaveData::ClassTypeInfoImplementation::bigfish::world_interaction::SaveData::ClassTypeInfoIm
    Replacement::draft(0x14186ed50, "bigfish_14186ed50", tpl_34635::<0x143080f10, 0x2, 0x14020f270> as *const ()), // bigfish::world_interaction::SaveData::ClassTypeInfoImplementation::bigfish::world_interaction::SaveData::ClassTypeInfoIm
    Replacement::draft(0x14186efb0, "bigfish_14186efb0", fn_14186efb0 as *const ()), // bigfish::world_interaction::SaveData::ClassTypeInfoImplementation::bigfish::world_interaction::SaveData::ClassTypeInfoIm
    Replacement::draft(0x14186eff0, "bigfish_14186eff0", fn_14186eff0 as *const ()), // bigfish::world_interaction::SaveData::ClassTypeInfoImplementation::bigfish::world_interaction::SaveData::ClassTypeInfoIm
    Replacement::draft(0x14186f1b0, "bigfish_14186f1b0", tpl_34638::<0x8> as *const ()), // bigfish::world_interaction::SaveData::ClassTypeInfoImplementation::bigfish::world_interaction::SaveData::ClassTypeInfoIm
    Replacement::draft_throws(0x14186f2f0, "unknown_14186f2f0", fn_14186f2f0 as *const (), Throws::Discard), // sub_14186f2f0
    Replacement::draft(0x14186fbb0, "unknown_14186fbb0", tpl_34394::<0x10> as *const ()), // sub_14186fbb0
    Replacement::draft(0x14186fce0, "unknown_14186fce0", tpl_34393 as *const ()), // sub_14186fce0
    Replacement::draft(0x14186fd70, "unknown_14186fd70", fn_14186fd70 as *const ()), // sub_14186fd70
    Replacement::draft(0x1418700a0, "unknown_1418700a0", fn_1418700a0 as *const ()), // sub_1418700a0
    Replacement::draft(0x141870180, "unknown_141870180", fn_141870180 as *const ()), // sub_141870180
    Replacement::draft(0x141870290, "unknown_141870290", fn_141870290 as *const ()), // sub_141870290
    Replacement::draft(0x141870420, "unknown_141870420", tpl_34394::<0x10> as *const ()), // sub_141870420
    Replacement::draft(0x141870540, "unknown_141870540", tpl_34394::<0x10> as *const ()), // sub_141870540
    Replacement::draft(0x1418706f0, "unknown_1418706f0", fn_1418706f0 as *const ()), // sub_1418706f0
    Replacement::draft(0x141870830, "unknown_141870830", fn_141870830 as *const ()), // sub_141870830
    Replacement::draft(0x141870900, "unknown_141870900", fn_141870900 as *const ()), // sub_141870900
    Replacement::draft(0x1418709f0, "unknown_1418709f0", fn_1418709f0 as *const ()), // sub_1418709f0
    Replacement::draft(0x141870a50, "unknown_141870a50", fn_141870a50 as *const ()), // sub_141870a50
    Replacement::draft(0x141870b80, "unknown_141870b80", tpl_34610::<0x20> as *const ()), // sub_141870b80
    Replacement::draft(0x141870ca0, "unknown_141870ca0", fn_141870ca0 as *const ()), // sub_141870ca0
    Replacement::draft(0x141870d80, "unknown_141870d80", fn_141870d80 as *const ()), // sub_141870d80
    Replacement::draft(0x141870de0, "unknown_141870de0", fn_141870de0 as *const ()), // sub_141870de0
    Replacement::draft_throws(0x1418715f0, "unknown_1418715f0", fn_1418715f0 as *const (), Throws::Discard), // sub_1418715f0
    Replacement::draft(0x141871a30, "unknown_141871a30", fn_141871a30 as *const ()), // sub_141871a30
    Replacement::draft_throws(0x141871c20, "unknown_141871c20", fn_141871c20 as *const (), Throws::Caught), // sub_141871c20
    Replacement::draft_throws(0x1418720d0, "unknown_1418720d0", fn_1418720d0 as *const (), Throws::Caught), // sub_1418720d0
    Replacement::draft(0x141872800, "unknown_141872800", fn_141872800 as *const ()), // sub_141872800
    Replacement::draft_throws(0x141872d90, "unknown_141872d90", fn_141872d90 as *const (), Throws::Caught), // sub_141872d90
    Replacement::draft(0x1418737f0, "unknown_1418737f0", fn_1418737f0 as *const ()), // sub_1418737f0
    Replacement::draft(0x1418739c0, "unknown_1418739c0", fn_1418739c0 as *const ()), // sub_1418739c0
    Replacement::draft(0x141874550, "unknown_141874550", tpl_34654::<0x3, 0x4, 0x8, 0x14240aa00> as *const ()), // sub_141874550
    Replacement::draft(0x1418745a0, "unknown_1418745a0", tpl_34654::<0x3, 0x4, 0x8, 0x14240aa00> as *const ()), // sub_1418745a0
    Replacement::draft(0x1418745f0, "unknown_1418745f0", fn_1418745f0 as *const ()), // sub_1418745f0
    Replacement::draft(0x141874b00, "unknown_141874b00", fn_141874b00 as *const ()), // sub_141874b00
    Replacement::draft(0x141874df0, "unknown_141874df0", fn_141874df0 as *const ()), // sub_141874df0
    Replacement::draft(0x141874ed0, "unknown_141874ed0", fn_141874ed0 as *const ()), // sub_141874ed0
    Replacement::draft_throws(0x141874f70, "unknown_141874f70", fn_141874f70 as *const (), Throws::Discard), // sub_141874f70
    Replacement::draft(0x141875340, "unknown_141875340", fn_141875340 as *const ()), // sub_141875340
    Replacement::draft(0x1418753b0, "unknown_1418753b0", fn_1418753b0 as *const ()), // sub_1418753b0
    Replacement::draft(0x141875400, "unknown_141875400", fn_141875400 as *const ()), // sub_141875400
    Replacement::draft(0x1418754e0, "unknown_1418754e0", fn_1418754e0 as *const ()), // sub_1418754e0
    Replacement::draft_throws(0x1418755b0, "unknown_1418755b0", fn_1418755b0 as *const (), Throws::Discard), // sub_1418755b0
    Replacement::draft(0x141875cd0, "unknown_141875cd0", fn_141875cd0 as *const ()), // sub_141875cd0
    Replacement::draft(0x141875dd0, "unknown_141875dd0", tpl_94422::<0x8, 0xc, 0x10> as *const ()), // sub_141875dd0
    Replacement::draft(0x141875ed0, "unknown_141875ed0", tpl_34607::<0x140236fa0> as *const ()), // sub_141875ed0
    Replacement::draft(0x141875f80, "unknown_141875f80", fn_141875f80 as *const ()), // sub_141875f80
    Replacement::draft(0x141876030, "unknown_141876030", tpl_34667::<0x20> as *const ()), // sub_141876030
    Replacement::draft(0x1418760d0, "unknown_1418760d0", fn_1418760d0 as *const ()), // sub_1418760d0
    Replacement::draft(0x141876190, "unknown_141876190", fn_141876190 as *const ()), // sub_141876190
    Replacement::draft(0x141876280, "unknown_141876280", tpl_34613 as *const ()), // sub_141876280
    Replacement::draft(0x141876310, "unknown_141876310", fn_141876310 as *const ()), // sub_141876310
    Replacement::draft(0x141876400, "unknown_141876400", fn_141876400 as *const ()), // sub_141876400
    Replacement::draft(0x141876470, "unknown_141876470", tpl_34670::<0x40, 0x50> as *const ()), // sub_141876470
    Replacement::draft(0x141876510, "unknown_141876510", tpl_34671::<0x20> as *const ()), // sub_141876510
    Replacement::draft(0x1418765b0, "unknown_1418765b0", tpl_94428::<0x143088f50, 0x8> as *const ()), // sub_1418765b0
    Replacement::draft(0x1418767f0, "unknown_1418767f0", fn_1418767f0 as *const ()), // sub_1418767f0
    Replacement::draft(0x1418768f0, "unknown_1418768f0", fn_1418768f0 as *const ()), // sub_1418768f0
    Replacement::draft(0x141876960, "unknown_141876960", fn_141876960 as *const ()), // sub_141876960
    Replacement::draft(0x141876a20, "unknown_141876a20", fn_141876a20 as *const ()), // sub_141876a20
    Replacement::draft(0x141876c80, "unknown_141876c80", tpl_34676::<0x40, 0x1401f2bc0> as *const ()), // sub_141876c80
    Replacement::draft(0x141876d50, "unknown_141876d50", tpl_34671::<0x20> as *const ()), // sub_141876d50
    Replacement::draft(0x141876df0, "unknown_141876df0", fn_141876df0 as *const ()), // sub_141876df0
    Replacement::draft(0x141877100, "unknown_141877100", tpl_94434::<0x8, 0x10, 0x14, 0x1401e84a0> as *const ()), // sub_141877100
    Replacement::draft(0x1418771c0, "unknown_1418771c0", tpl_34678::<0x8, 0x10, 0x14, 0x1401e84a0> as *const ()), // sub_1418771c0
    Replacement::draft(0x1418771f0, "unknown_1418771f0", tpl_34679::<0x8, 0x18> as *const ()), // sub_1418771f0
    Replacement::draft(0x141877290, "unknown_141877290", fn_141877290 as *const ()), // sub_141877290
    Replacement::draft(0x141877550, "unknown_141877550", fn_141877550 as *const ()), // sub_141877550
    Replacement::draft(0x141877600, "unknown_141877600", fn_141877600 as *const ()), // sub_141877600
    Replacement::draft(0x141877850, "unknown_141877850", fn_141877850 as *const ()), // sub_141877850
    Replacement::draft(0x141877900, "unknown_141877900", fn_141877900 as *const ()), // sub_141877900
    Replacement::draft(0x141877a70, "unknown_141877a70", fn_141877a70 as *const ()), // sub_141877a70
    Replacement::draft(0x141877b20, "unknown_141877b20", tpl_94441::<0x8> as *const ()), // sub_141877b20
    Replacement::draft(0x141877bf0, "unknown_141877bf0", tpl_34684::<0x1402359b0> as *const ()), // sub_141877bf0
    Replacement::draft(0x141877ca0, "unknown_141877ca0", fn_141877ca0 as *const ()), // sub_141877ca0
    Replacement::draft(0x141877e40, "unknown_141877e40", fn_141877e40 as *const ()), // sub_141877e40
    Replacement::draft(0x141877f10, "unknown_141877f10", tpl_34393 as *const ()), // sub_141877f10
    Replacement::draft(0x141877fa0, "unknown_141877fa0", fn_141877fa0 as *const ()), // sub_141877fa0
    Replacement::draft(0x1418780f0, "unknown_1418780f0", fn_1418780f0 as *const ()), // sub_1418780f0
    Replacement::draft(0x1418781a0, "unknown_1418781a0", fn_1418781a0 as *const ()), // sub_1418781a0
    Replacement::draft(0x141878300, "unknown_141878300", fn_141878300 as *const ()), // sub_141878300
    Replacement::draft(0x1418786d0, "unknown_1418786d0", fn_1418786d0 as *const ()), // sub_1418786d0
    Replacement::draft(0x1418787b0, "unknown_1418787b0", tpl_34667::<0x20> as *const ()), // sub_1418787b0
    Replacement::draft(0x141878850, "unknown_141878850", fn_141878850 as *const ()), // sub_141878850
    Replacement::draft(0x141878a60, "unknown_141878a60", fn_141878a60 as *const ()), // sub_141878a60
    Replacement::draft(0x141878b10, "unknown_141878b10", fn_141878b10 as *const ()), // sub_141878b10
    Replacement::draft(0x141878c90, "unknown_141878c90", fn_141878c90 as *const ()), // sub_141878c90
    Replacement::draft(0x141878d70, "unknown_141878d70", tpl_34693::<0x10, 0x18, 0x20> as *const ()), // sub_141878d70
    Replacement::draft(0x141879430, "unknown_141879430", tpl_34694::<0x10> as *const ()), // sub_141879430
    Replacement::draft(0x1418794e0, "unknown_1418794e0", tpl_34695::<0x88, 0x90, 0x10> as *const ()), // sub_1418794e0
    Replacement::draft(0x141879540, "unknown_141879540", tpl_34695::<0x88, 0x90, 0x10> as *const ()), // sub_141879540
    Replacement::draft(0x141879560, "unknown_141879560", tpl_3352::<0x20, 0x18> as *const ()), // sub_141879560
    Replacement::draft_throws(0x1418795f0, "unknown_1418795f0", tpl_1330::<0x8, 0x140232fb0, 0x143844040, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_1418795f0
    Replacement::draft_throws(0x1418796d0, "unknown_1418796d0", tpl_1331::<0x8, 0x140232780, 0x143843fc8, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_1418796d0
    Replacement::draft(0x1418798c0, "unknown_1418798c0", tpl_34696::<0x18, 0x8, 0x10, 0x20, 0x30, 0x28> as *const ()), // sub_1418798c0
    Replacement::draft(0x141879910, "unknown_141879910", tpl_34697::<0x18, 0x20, 0x30, 0x8, 0x28, 0x10> as *const ()), // sub_141879910
    Replacement::draft(0x141879950, "unknown_141879950", fn_141879950 as *const ()), // sub_141879950
    Replacement::draft(0x1418799e0, "unknown_1418799e0", fn_1418799e0 as *const ()), // sub_1418799e0
    Replacement::draft(0x141879a80, "unknown_141879a80", tpl_1267::<0x30, 0x1426467e8, 0x8, 0x10, 0x2, 0x141879aa5> as *const ()), // sub_141879a80
    Replacement::draft(0x14187a7c0, "unknown_14187a7c0", fn_14187a7c0 as *const ()), // sub_14187a7c0
    Replacement::draft(0x14187a870, "unknown_14187a870", fn_14187a870 as *const ()), // sub_14187a870
    Replacement::draft(0x14187a970, "unknown_14187a970", fn_14187a970 as *const ()), // sub_14187a970
    Replacement::draft(0x14187a9d0, "unknown_14187a9d0", tpl_34703::<0x10, 0x8, 0x18, 0x28, 0x30, 0x48, 0x20, 0x10048, 0x14187a870> as *const ()), // sub_14187a9d0
    Replacement::draft(0x14187aa40, "unknown_14187aa40", tpl_34704::<0x10, 0x18, 0x8, 0x48, 0x10048, 0x20, 0x28, 0x30, 0x14187a870> as *const ()), // sub_14187aa40
    Replacement::draft(0x14187ac40, "unknown_14187ac40", fn_14187ac40 as *const ()), // sub_14187ac40
    Replacement::draft(0x14187aca0, "unknown_14187aca0", fn_14187aca0 as *const ()), // sub_14187aca0
    Replacement::draft(0x14187ace0, "unknown_14187ace0", fn_14187ace0 as *const ()), // sub_14187ace0
    Replacement::draft(0x14187ad20, "unknown_14187ad20", fn_14187ad20 as *const ()), // sub_14187ad20
    Replacement::draft(0x14187ada0, "unknown_14187ada0", fn_14187ada0 as *const ()), // sub_14187ada0
    Replacement::draft(0x14187af20, "unknown_14187af20", fn_14187af20 as *const ()), // sub_14187af20
    Replacement::draft_throws(0x14187b270, "unknown_14187b270", fn_14187b270 as *const (), Throws::Discard), // sub_14187b270
    Replacement::draft(0x14187b5f0, "unknown_14187b5f0", fn_14187b5f0 as *const ()), // sub_14187b5f0
    Replacement::draft_throws(0x14187beb0, "unknown_14187beb0", fn_14187beb0 as *const (), Throws::Caught), // sub_14187beb0
    Replacement::draft(0x14187d6e0, "unknown_14187d6e0", fn_14187d6e0 as *const ()), // sub_14187d6e0
    Replacement::draft(0x14187dbd0, "unknown_14187dbd0", fn_14187dbd0 as *const ()), // sub_14187dbd0
    Replacement::draft_throws(0x14187f080, "unknown_14187f080", fn_14187f080 as *const (), Throws::Discard), // sub_14187f080
    Replacement::draft(0x14187fe10, "unknown_14187fe10", fn_14187fe10 as *const ()), // sub_14187fe10
    Replacement::draft(0x141880230, "unknown_141880230", fn_141880230 as *const ()), // sub_141880230
    Replacement::draft(0x141881280, "unknown_141881280", fn_141881280 as *const ()), // sub_141881280
    Replacement::draft(0x141881f00, "unknown_141881f00", tpl_94468::<0x13, 0x143426494> as *const ()), // sub_141881f00
    Replacement::draft(0x141881f40, "unknown_141881f40", tpl_94468::<0x13, 0x143426494> as *const ()), // sub_141881f40
    Replacement::draft_throws(0x141881f80, "unknown_141881f80", fn_141881f80 as *const (), Throws::Discard), // sub_141881f80
    Replacement::draft_throws(0x141882e90, "unknown_141882e90", fn_141882e90 as *const (), Throws::Discard), // sub_141882e90
    Replacement::draft(0x141883650, "unknown_141883650", fn_141883650 as *const ()), // sub_141883650
    Replacement::draft(0x1418837b0, "unknown_1418837b0", fn_1418837b0 as *const ()), // sub_1418837b0
    Replacement::draft(0x1418837f0, "unknown_1418837f0", fn_1418837f0 as *const ()), // sub_1418837f0
    Replacement::draft(0x141883b20, "unknown_141883b20", fn_141883b20 as *const ()), // sub_141883b20
    Replacement::draft(0x141884130, "unknown_141884130", fn_141884130 as *const ()), // sub_141884130
    Replacement::draft(0x141884340, "unknown_141884340", fn_141884340 as *const ()), // sub_141884340
    Replacement::draft(0x1418845c0, "unknown_1418845c0", fn_1418845c0 as *const ()), // sub_1418845c0
    Replacement::draft_throws(0x1418846d0, "unknown_1418846d0", fn_1418846d0 as *const (), Throws::Caught), // sub_1418846d0
    Replacement::draft_throws(0x1418846f0, "unknown_1418846f0", fn_1418846f0 as *const (), Throws::Discard), // sub_1418846f0
    Replacement::draft(0x141884d30, "unknown_141884d30", fn_141884d30 as *const ()), // sub_141884d30
    Replacement::draft(0x141884fc0, "unknown_141884fc0", fn_141884fc0 as *const ()), // sub_141884fc0
    Replacement::draft(0x1418852e0, "unknown_1418852e0", fn_1418852e0 as *const ()), // sub_1418852e0
    Replacement::draft(0x141885430, "unknown_141885430", tpl_34722 as *const ()), // sub_141885430
    Replacement::draft(0x141885670, "unknown_141885670", tpl_34723::<0x4> as *const ()), // sub_141885670
    Replacement::draft(0x141885680, "unknown_141885680", fn_141885680 as *const ()), // sub_141885680
    Replacement::draft(0x1418856b0, "unknown_1418856b0", fn_1418856b0 as *const ()), // sub_1418856b0
    Replacement::draft(0x1418856f0, "unknown_1418856f0", tpl_34723::<0x4> as *const ()), // sub_1418856f0
    Replacement::draft_throws(0x141885710, "unknown_141885710", fn_141885710 as *const (), Throws::Discard), // sub_141885710
    Replacement::draft(0x141885880, "unknown_141885880", fn_141885880 as *const ()), // sub_141885880
    Replacement::draft(0x1418858b0, "unknown_1418858b0", fn_1418858b0 as *const ()), // sub_1418858b0
    Replacement::draft_throws(0x141885900, "unknown_141885900", fn_141885900 as *const (), Throws::Discard), // sub_141885900
    Replacement::draft(0x141885a50, "unknown_141885a50", fn_141885a50 as *const ()), // sub_141885a50
    Replacement::draft(0x141885b10, "unknown_141885b10", fn_141885b10 as *const ()), // sub_141885b10
    Replacement::draft(0x141885b50, "unknown_141885b50", fn_141885b50 as *const ()), // sub_141885b50
    Replacement::draft(0x141885b90, "unknown_141885b90", tpl_34731::<0x28> as *const ()), // sub_141885b90
    Replacement::draft(0x141885bb0, "unknown_141885bb0", tpl_34732::<0x8, 0x30, 0x38, 0x7c, 0x28, 0x40, 0x2c> as *const ()), // sub_141885bb0
    Replacement::draft(0x141885c80, "unknown_141885c80", tpl_34731::<0x24> as *const ()), // sub_141885c80
    Replacement::draft(0x141885ca0, "unknown_141885ca0", tpl_34732::<0x8, 0x30, 0x38, 0x7c, 0x24, 0x40, 0x28> as *const ()), // sub_141885ca0
    Replacement::draft(0x141885d70, "unknown_141885d70", tpl_34731::<0x20> as *const ()), // sub_141885d70
    Replacement::draft(0x141885d90, "unknown_141885d90", tpl_34732::<0x8, 0x30, 0x38, 0x7c, 0x20, 0x40, 0x24> as *const ()), // sub_141885d90
    Replacement::draft(0x141885e60, "unknown_141885e60", tpl_34731::<0x1c> as *const ()), // sub_141885e60
    Replacement::draft(0x141885e80, "unknown_141885e80", tpl_34732::<0x8, 0x30, 0x38, 0x7c, 0x1c, 0x40, 0x20> as *const ()), // sub_141885e80
    Replacement::draft(0x141885f50, "unknown_141885f50", tpl_34731::<0x18> as *const ()), // sub_141885f50
    Replacement::draft(0x141885f70, "unknown_141885f70", tpl_34732::<0x8, 0x30, 0x38, 0x7c, 0x18, 0x40, 0x1c> as *const ()), // sub_141885f70
    Replacement::draft(0x141886040, "unknown_141886040", tpl_34731::<0x14> as *const ()), // sub_141886040
    Replacement::draft(0x141886060, "unknown_141886060", tpl_34732::<0x8, 0x30, 0x38, 0x7c, 0x14, 0x40, 0x18> as *const ()), // sub_141886060
    Replacement::draft(0x141886130, "unknown_141886130", tpl_34731::<0x10> as *const ()), // sub_141886130
    Replacement::draft(0x141886150, "unknown_141886150", tpl_34732::<0x8, 0x30, 0x38, 0x7c, 0x10, 0x40, 0x14> as *const ()), // sub_141886150
    Replacement::draft(0x141886220, "unknown_141886220", tpl_34731::<0xc> as *const ()), // sub_141886220
    Replacement::draft(0x141886240, "unknown_141886240", tpl_34732::<0x8, 0x30, 0x38, 0x7c, 0xc, 0x40, 0x10> as *const ()), // sub_141886240
    Replacement::draft(0x141886310, "unknown_141886310", tpl_34731::<0x8> as *const ()), // sub_141886310
    Replacement::draft(0x141886330, "unknown_141886330", fn_141886330 as *const ()), // sub_141886330
    Replacement::draft(0x141886400, "unknown_141886400", tpl_34731::<0x4> as *const ()), // sub_141886400
    Replacement::draft(0x141886420, "unknown_141886420", fn_141886420 as *const ()), // sub_141886420
    Replacement::draft(0x1418864f0, "unknown_1418864f0", tpl_34735 as *const ()), // sub_1418864f0
    Replacement::draft(0x141886510, "unknown_141886510", fn_141886510 as *const ()), // sub_141886510
    Replacement::draft(0x1418865e0, "unknown_1418865e0", tpl_34731::<0x44> as *const ()), // sub_1418865e0
    Replacement::draft(0x141886600, "unknown_141886600", tpl_34731::<0x40> as *const ()), // sub_141886600
    Replacement::draft(0x141886620, "unknown_141886620", tpl_34731::<0x3c> as *const ()), // sub_141886620
    Replacement::draft(0x141886640, "unknown_141886640", tpl_34731::<0x38> as *const ()), // sub_141886640
    Replacement::draft(0x141886660, "unknown_141886660", tpl_34731::<0x34> as *const ()), // sub_141886660
    Replacement::draft(0x141886680, "unknown_141886680", tpl_34731::<0x30> as *const ()), // sub_141886680
    Replacement::draft(0x1418866a0, "unknown_1418866a0", tpl_34731::<0x2c> as *const ()), // sub_1418866a0
    Replacement::draft(0x1418866c0, "unknown_1418866c0", tpl_34731::<0x28> as *const ()), // sub_1418866c0
    Replacement::draft(0x1418866e0, "unknown_1418866e0", tpl_34731::<0x24> as *const ()), // sub_1418866e0
    Replacement::draft(0x141886700, "unknown_141886700", tpl_34731::<0x20> as *const ()), // sub_141886700
    Replacement::draft(0x141886720, "unknown_141886720", tpl_34731::<0x1c> as *const ()), // sub_141886720
    Replacement::draft(0x141886740, "unknown_141886740", tpl_34731::<0x18> as *const ()), // sub_141886740
    Replacement::draft(0x141886760, "unknown_141886760", tpl_34731::<0x14> as *const ()), // sub_141886760
    Replacement::draft(0x141886780, "unknown_141886780", tpl_34731::<0x10> as *const ()), // sub_141886780
    Replacement::draft(0x1418867a0, "unknown_1418867a0", tpl_34731::<0xc> as *const ()), // sub_1418867a0
    Replacement::draft(0x1418867c0, "unknown_1418867c0", tpl_34731::<0x8> as *const ()), // sub_1418867c0
    Replacement::draft(0x1418867e0, "unknown_1418867e0", tpl_34731::<0x4> as *const ()), // sub_1418867e0
    Replacement::draft(0x141886800, "unknown_141886800", tpl_34735 as *const ()), // sub_141886800
    Replacement::draft(0x141886820, "unknown_141886820", fn_141886820 as *const ()), // sub_141886820
    Replacement::draft(0x141886840, "unknown_141886840", fn_141886840 as *const ()), // sub_141886840
    Replacement::draft(0x1418868d0, "unknown_1418868d0", fn_1418868d0 as *const ()), // sub_1418868d0
    Replacement::draft(0x141886940, "unknown_141886940", fn_141886940 as *const ()), // sub_141886940
    Replacement::draft(0x1418869a0, "unknown_1418869a0", tpl_34741::<0x24, 0x1434268d0> as *const ()), // sub_1418869a0
    Replacement::draft(0x1418869c0, "unknown_1418869c0", fn_1418869c0 as *const ()), // sub_1418869c0
    Replacement::draft(0x141886a40, "unknown_141886a40", tpl_34743::<0x10, 0x3c> as *const ()), // sub_141886a40
    Replacement::draft(0x141886a60, "unknown_141886a60", tpl_34743::<0x10, 0x38> as *const ()), // sub_141886a60
    Replacement::draft(0x141886a80, "unknown_141886a80", tpl_34744::<0x10, 0x34> as *const ()), // sub_141886a80
    Replacement::draft(0x141886aa0, "unknown_141886aa0", tpl_34743::<0x10, 0x30> as *const ()), // sub_141886aa0
    Replacement::draft(0x141886ac0, "unknown_141886ac0", fn_141886ac0 as *const ()), // sub_141886ac0
    Replacement::draft(0x141886ae0, "unknown_141886ae0", tpl_34731::<0x20> as *const ()), // sub_141886ae0
    Replacement::draft(0x141886b00, "unknown_141886b00", tpl_34746::<0x10, 0x18, 0x88, 0x20, 0x24> as *const ()), // sub_141886b00
    Replacement::draft(0x141886b60, "unknown_141886b60", tpl_34731::<0x1c> as *const ()), // sub_141886b60
    Replacement::draft(0x141886b80, "unknown_141886b80", tpl_34731::<0x18> as *const ()), // sub_141886b80
    Replacement::draft(0x141886ba0, "unknown_141886ba0", tpl_34747::<0x8> as *const ()), // sub_141886ba0
    Replacement::draft(0x141886bb0, "unknown_141886bb0", tpl_34748::<0x10, 0x18, 0x88, 0x8, 0xc> as *const ()), // sub_141886bb0
    Replacement::draft(0x141886c10, "unknown_141886c10", tpl_34747::<0x4> as *const ()), // sub_141886c10
    Replacement::draft(0x141886c20, "unknown_141886c20", tpl_34748::<0x10, 0x18, 0x88, 0x4, 0x8> as *const ()), // sub_141886c20
    Replacement::draft(0x141886c90, "unknown_141886c90", fn_141886c90 as *const ()), // sub_141886c90
    Replacement::draft(0x141886cf0, "unknown_141886cf0", tpl_34731::<0x54> as *const ()), // sub_141886cf0
    Replacement::draft(0x141886d10, "unknown_141886d10", tpl_34731::<0x50> as *const ()), // sub_141886d10
    Replacement::draft(0x141886d30, "unknown_141886d30", tpl_34731::<0x4c> as *const ()), // sub_141886d30
    Replacement::draft(0x141886d50, "unknown_141886d50", tpl_34731::<0x48> as *const ()), // sub_141886d50
    Replacement::draft(0x141886d70, "unknown_141886d70", tpl_34747::<0x60> as *const ()), // sub_141886d70
    Replacement::draft(0x141886d80, "unknown_141886d80", tpl_34731::<0x44> as *const ()), // sub_141886d80
    Replacement::draft(0x141886da0, "unknown_141886da0", tpl_34731::<0x40> as *const ()), // sub_141886da0
    Replacement::draft(0x141886dc0, "unknown_141886dc0", tpl_34731::<0x3c> as *const ()), // sub_141886dc0
    Replacement::draft(0x141886de0, "unknown_141886de0", tpl_34731::<0x38> as *const ()), // sub_141886de0
    Replacement::draft(0x141886e00, "unknown_141886e00", tpl_34731::<0x34> as *const ()), // sub_141886e00
    Replacement::draft(0x141886e20, "unknown_141886e20", tpl_34750::<0x30> as *const ()), // sub_141886e20
    Replacement::draft(0x141886e30, "unknown_141886e30", tpl_34747::<0x2c> as *const ()), // sub_141886e30
    Replacement::draft(0x141886e40, "unknown_141886e40", fn_141886e40 as *const ()), // sub_141886e40
    Replacement::draft(0x141886ea0, "unknown_141886ea0", tpl_34731::<0x28> as *const ()), // sub_141886ea0
    Replacement::draft(0x141886ec0, "unknown_141886ec0", tpl_34731::<0x24> as *const ()), // sub_141886ec0
    Replacement::draft(0x141886ee0, "unknown_141886ee0", tpl_34731::<0x20> as *const ()), // sub_141886ee0
    Replacement::draft(0x141886f00, "unknown_141886f00", tpl_34747::<0x1c> as *const ()), // sub_141886f00
    Replacement::draft(0x141886f10, "unknown_141886f10", tpl_34747::<0x18> as *const ()), // sub_141886f10
    Replacement::draft(0x141886f30, "unknown_141886f30", tpl_34731::<0x10> as *const ()), // sub_141886f30
    Replacement::draft(0x141886f50, "unknown_141886f50", tpl_34731::<0xc> as *const ()), // sub_141886f50
    Replacement::draft(0x141886f70, "unknown_141886f70", tpl_34731::<0x8> as *const ()), // sub_141886f70
    Replacement::draft(0x141886f90, "unknown_141886f90", tpl_34731::<0x4> as *const ()), // sub_141886f90
    Replacement::draft(0x141886fb0, "unknown_141886fb0", tpl_34735 as *const ()), // sub_141886fb0
    Replacement::draft(0x141886fd0, "unknown_141886fd0", fn_141886fd0 as *const ()), // sub_141886fd0
    Replacement::draft(0x1418870d0, "unknown_1418870d0", fn_1418870d0 as *const ()), // sub_1418870d0
    Replacement::draft(0x141887190, "unknown_141887190", fn_141887190 as *const ()), // sub_141887190
    Replacement::draft(0x141887260, "unknown_141887260", fn_141887260 as *const ()), // sub_141887260
    Replacement::draft(0x141887500, "unknown_141887500", fn_141887500 as *const ()), // sub_141887500
    Replacement::draft(0x141887560, "unknown_141887560", tpl_34757::<0x20, 0x28, 0x94, 0x14, 0x18> as *const ()), // sub_141887560
    Replacement::draft(0x1418875c0, "unknown_1418875c0", tpl_34758::<0x8, 0xe4, 0x48, 0x4c> as *const ()), // sub_1418875c0
    Replacement::draft(0x141887620, "unknown_141887620", fn_141887620 as *const ()), // sub_141887620
    Replacement::draft(0x141887680, "unknown_141887680", tpl_34757::<0x20, 0x28, 0x94, 0x30, 0x34> as *const ()), // sub_141887680
    Replacement::draft(0x1418876e0, "unknown_1418876e0", fn_1418876e0 as *const ()), // sub_1418876e0
    Replacement::draft(0x141887740, "unknown_141887740", tpl_34758::<0x8, 0xe4, 0x20, 0x24> as *const ()), // sub_141887740
    Replacement::draft(0x1418877a0, "unknown_1418877a0", tpl_34758::<0x8, 0xe4, 0x54, 0x58> as *const ()), // sub_1418877a0
    Replacement::draft(0x141887800, "unknown_141887800", tpl_34758::<0x8, 0xe4, 0x34, 0x38> as *const ()), // sub_141887800
    Replacement::draft(0x141887860, "unknown_141887860", tpl_34758::<0x8, 0xe4, 0x50, 0x54> as *const ()), // sub_141887860
    Replacement::draft(0x1418878c0, "unknown_1418878c0", tpl_34757::<0x20, 0x28, 0x94, 0x3c, 0x40> as *const ()), // sub_1418878c0
    Replacement::draft(0x141887920, "unknown_141887920", tpl_34746::<0x10, 0x18, 0x88, 0x1c, 0x20> as *const ()), // sub_141887920
    Replacement::draft(0x141887980, "unknown_141887980", fn_141887980 as *const ()), // sub_141887980
    Replacement::draft_throws(0x1418879e0, "unknown_1418879e0", fn_1418879e0 as *const (), Throws::Discard), // sub_1418879e0
    Replacement::draft(0x141887a90, "unknown_141887a90", fn_141887a90 as *const ()), // sub_141887a90
    Replacement::draft(0x141887b00, "unknown_141887b00", tpl_34758::<0x8, 0xe4, 0xc, 0x10> as *const ()), // sub_141887b00
    Replacement::draft(0x141887b60, "unknown_141887b60", tpl_34758::<0x8, 0xe4, 0x28, 0x2c> as *const ()), // sub_141887b60
    Replacement::draft(0x141887bc0, "unknown_141887bc0", tpl_34758::<0x8, 0xe4, 0x44, 0x48> as *const ()), // sub_141887bc0
    Replacement::draft(0x141887c20, "unknown_141887c20", tpl_34757::<0x20, 0x28, 0x94, 0x18, 0x1c> as *const ()), // sub_141887c20
    Replacement::draft(0x141887c80, "unknown_141887c80", tpl_34757::<0x20, 0x28, 0x94, 0xc, 0x10> as *const ()), // sub_141887c80
    Replacement::draft(0x141887ce0, "unknown_141887ce0", fn_141887ce0 as *const ()), // sub_141887ce0
    Replacement::draft(0x141887d40, "unknown_141887d40", fn_141887d40 as *const ()), // sub_141887d40
    Replacement::draft(0x141887da0, "unknown_141887da0", tpl_34758::<0x8, 0xe4, 0x24, 0x28> as *const ()), // sub_141887da0
    Replacement::draft(0x141887e00, "unknown_141887e00", tpl_34757::<0x20, 0x28, 0x94, 0x44, 0x48> as *const ()), // sub_141887e00
    Replacement::draft(0x141887e60, "unknown_141887e60", fn_141887e60 as *const ()), // sub_141887e60
    Replacement::draft_throws(0x141887ec0, "unknown_141887ec0", fn_141887ec0 as *const (), Throws::Discard), // sub_141887ec0
    Replacement::draft(0x141887f50, "unknown_141887f50", fn_141887f50 as *const ()), // sub_141887f50
    Replacement::draft(0x141887fb0, "unknown_141887fb0", tpl_34757::<0x20, 0x28, 0x94, 0x10, 0x14> as *const ()), // sub_141887fb0
    Replacement::draft(0x141888010, "unknown_141888010", tpl_34758::<0x8, 0xe4, 0x3c, 0x40> as *const ()), // sub_141888010
    Replacement::draft(0x141888070, "unknown_141888070", tpl_34758::<0x8, 0xe4, 0x4c, 0x50> as *const ()), // sub_141888070
    Replacement::draft(0x1418880d0, "unknown_1418880d0", fn_1418880d0 as *const ()), // sub_1418880d0
    Replacement::draft(0x141888130, "unknown_141888130", tpl_34758::<0x8, 0xe4, 0x10, 0x14> as *const ()), // sub_141888130
    Replacement::draft(0x141888190, "unknown_141888190", tpl_34758::<0x8, 0xe4, 0x40, 0x44> as *const ()), // sub_141888190
    Replacement::draft(0x1418881f0, "unknown_1418881f0", fn_1418881f0 as *const ()), // sub_1418881f0
    Replacement::draft(0x141888250, "unknown_141888250", tpl_34758::<0x8, 0xe4, 0x38, 0x3c> as *const ()), // sub_141888250
    Replacement::draft(0x1418882b0, "unknown_1418882b0", tpl_34757::<0x20, 0x28, 0x94, 0x40, 0x44> as *const ()), // sub_1418882b0
    Replacement::draft(0x141888310, "unknown_141888310", tpl_34757::<0x20, 0x28, 0x94, 0x2c, 0x30> as *const ()), // sub_141888310
    Replacement::draft(0x141888370, "unknown_141888370", tpl_34757::<0x20, 0x28, 0x94, 0x34, 0x38> as *const ()), // sub_141888370
    Replacement::draft(0x1418883d0, "unknown_1418883d0", fn_1418883d0 as *const ()), // sub_1418883d0
    Replacement::draft(0x141888430, "unknown_141888430", tpl_34757::<0x20, 0x28, 0x94, 0x4, 0x8> as *const ()), // sub_141888430
    Replacement::draft(0x141888490, "unknown_141888490", fn_141888490 as *const ()), // sub_141888490
    Replacement::draft(0x1418884f0, "unknown_1418884f0", tpl_34757::<0x20, 0x28, 0x94, 0x38, 0x3c> as *const ()), // sub_1418884f0
    Replacement::draft(0x141888550, "unknown_141888550", tpl_34757::<0x20, 0x28, 0x94, 0x8, 0xc> as *const ()), // sub_141888550
    Replacement::draft(0x1418885b0, "unknown_1418885b0", fn_1418885b0 as *const ()), // sub_1418885b0
    Replacement::draft_throws(0x141888610, "unknown_141888610", fn_141888610 as *const (), Throws::Discard), // sub_141888610
    Replacement::draft_throws(0x141888e20, "unknown_141888e20", fn_141888e20 as *const (), Throws::Discard), // sub_141888e20
    Replacement::draft(0x141888f80, "unknown_141888f80", fn_141888f80 as *const ()), // sub_141888f80
    Replacement::draft(0x1418890e0, "unknown_1418890e0", tpl_94428::<0x143094930, 0x8> as *const ()), // sub_1418890e0
    Replacement::draft(0x141889250, "unknown_141889250", tpl_34775::<0x20, 0x28> as *const ()), // sub_141889250
    Replacement::draft(0x1418892f0, "unknown_1418892f0", fn_1418892f0 as *const ()), // sub_1418892f0
    Replacement::draft(0x1418893a0, "unknown_1418893a0", fn_1418893a0 as *const ()), // sub_1418893a0
    Replacement::draft(0x1418893d0, "unknown_1418893d0", fn_1418893d0 as *const ()), // sub_1418893d0
    Replacement::draft(0x141889530, "unknown_141889530", tpl_34393 as *const ()), // sub_141889530
    Replacement::draft(0x1418895c0, "unknown_1418895c0", tpl_34605::<0x143094798> as *const ()), // sub_1418895c0
    Replacement::draft(0x141889700, "unknown_141889700", tpl_34393 as *const ()), // sub_141889700
    Replacement::draft(0x141889e50, "unknown_141889e50", fn_141889e50 as *const ()), // sub_141889e50
    Replacement::draft_throws(0x14188a2b0, "unknown_14188a2b0", fn_14188a2b0 as *const (), Throws::Caught), // sub_14188a2b0
    Replacement::draft(0x14188a430, "unknown_14188a430", fn_14188a430 as *const ()), // sub_14188a430
    Replacement::draft(0x14188a780, "unknown_14188a780", fn_14188a780 as *const ()), // sub_14188a780
    Replacement::draft_throws(0x14188a800, "unknown_14188a800", fn_14188a800 as *const (), Throws::Discard), // sub_14188a800
    Replacement::draft(0x14188ae10, "unknown_14188ae10", tpl_34671::<0x20> as *const ()), // sub_14188ae10
    Replacement::draft(0x14188aeb0, "unknown_14188aeb0", tpl_34394::<0x10> as *const ()), // sub_14188aeb0
    Replacement::draft(0x14188afe0, "unknown_14188afe0", fn_14188afe0 as *const ()), // sub_14188afe0
    Replacement::draft(0x14188b100, "unknown_14188b100", fn_14188b100 as *const ()), // sub_14188b100
    Replacement::draft(0x14188b220, "unknown_14188b220", tpl_34671::<0x20> as *const ()), // sub_14188b220
    Replacement::draft(0x14188b2c0, "unknown_14188b2c0", fn_14188b2c0 as *const ()), // sub_14188b2c0
    Replacement::draft(0x14188b3b0, "unknown_14188b3b0", fn_14188b3b0 as *const ()), // sub_14188b3b0
    Replacement::draft(0x14188b420, "unknown_14188b420", fn_14188b420 as *const ()), // sub_14188b420
    Replacement::draft(0x14188bb60, "unknown_14188bb60", tpl_1439::<0x8, 0x140243450> as *const ()), // sub_14188bb60
    Replacement::draft(0x14188bba0, "unknown_14188bba0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c8f8d4, 0x158, 0x14020e8f0> as *const ()), // sub_14188bba0
    Replacement::draft(0x14188bca0, "unknown_14188bca0", fn_14188bca0 as *const ()), // sub_14188bca0
    Replacement::draft_throws(0x14188bd40, "unknown_14188bd40", fn_14188bd40 as *const (), Throws::Caught), // sub_14188bd40
    Replacement::draft_throws(0x14188c560, "unknown_14188c560", fn_14188c560 as *const (), Throws::Discard), // sub_14188c560
    Replacement::draft_throws(0x14188c790, "unknown_14188c790", fn_14188c790 as *const (), Throws::Discard), // sub_14188c790
    Replacement::draft_throws(0x14188ca20, "unknown_14188ca20", fn_14188ca20 as *const (), Throws::Caught), // sub_14188ca20
    Replacement::draft_throws(0x14188d0b0, "unknown_14188d0b0", fn_14188d0b0 as *const (), Throws::Caught), // sub_14188d0b0
    Replacement::draft(0x14188d380, "unknown_14188d380", fn_14188d380 as *const ()), // sub_14188d380
    Replacement::draft(0x14188d3a0, "unknown_14188d3a0", fn_14188d3a0 as *const ()), // sub_14188d3a0
    Replacement::draft_throws(0x14188d440, "unknown_14188d440", fn_14188d440 as *const (), Throws::Caught), // sub_14188d440
    Replacement::draft(0x14188d830, "unknown_14188d830", fn_14188d830 as *const ()), // sub_14188d830
    Replacement::draft_throws(0x14188d960, "unknown_14188d960", fn_14188d960 as *const (), Throws::Discard), // sub_14188d960
    Replacement::draft_throws(0x14188dba0, "unknown_14188dba0", fn_14188dba0 as *const (), Throws::Discard), // sub_14188dba0
    Replacement::draft_throws(0x14188de00, "unknown_14188de00", fn_14188de00 as *const (), Throws::Discard), // sub_14188de00
    Replacement::draft_throws(0x14188dfe0, "unknown_14188dfe0", fn_14188dfe0 as *const (), Throws::Discard), // sub_14188dfe0
    Replacement::draft_throws(0x14188e950, "unknown_14188e950", tpl_34795::<0x1426e9790, 0x1410d6ca0, 0x140243650, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x140243930, 0x143823788, 0x1426e9780, 0x140242690, 0x1438234b0, 0x14387ebd0, 0x140243c20, 0x1402116e0> as *const (), Throws::Discard), // sub_14188e950
    Replacement::draft(0x14188edb0, "unknown_14188edb0", fn_14188edb0 as *const ()), // sub_14188edb0
    Replacement::draft(0x14188ee50, "unknown_14188ee50", fn_14188ee50 as *const ()), // sub_14188ee50
    Replacement::draft(0x14188ef30, "unknown_14188ef30", fn_14188ef30 as *const ()), // sub_14188ef30
    Replacement::draft(0x14188f110, "unknown_14188f110", fn_14188f110 as *const ()), // sub_14188f110
    Replacement::draft(0x14188f270, "unknown_14188f270", fn_14188f270 as *const ()), // sub_14188f270
    Replacement::draft(0x14188f390, "unknown_14188f390", tpl_34799::<0x20, 0x1401f2bc0> as *const ()), // sub_14188f390
    Replacement::draft(0x14188f8d0, "unknown_14188f8d0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c8fe90, 0x158, 0x14020e8f0> as *const ()), // sub_14188f8d0
    Replacement::draft(0x14188f9d0, "unknown_14188f9d0", fn_14188f9d0 as *const ()), // sub_14188f9d0
    Replacement::draft_throws(0x14188fa80, "unknown_14188fa80", fn_14188fa80 as *const (), Throws::Discard), // sub_14188fa80
    Replacement::draft_throws(0x14188fcd0, "unknown_14188fcd0", fn_14188fcd0 as *const (), Throws::Discard), // sub_14188fcd0
    Replacement::draft(0x14188ff50, "unknown_14188ff50", fn_14188ff50 as *const ()), // sub_14188ff50
    Replacement::draft(0x141890060, "unknown_141890060", fn_141890060 as *const ()), // sub_141890060
    Replacement::draft_throws(0x141890170, "unknown_141890170", fn_141890170 as *const (), Throws::Discard), // sub_141890170
    Replacement::draft_throws(0x141890ad0, "unknown_141890ad0", fn_141890ad0 as *const (), Throws::Discard), // sub_141890ad0
    Replacement::draft(0x141891100, "unknown_141891100", tpl_34393 as *const ()), // sub_141891100
    Replacement::draft(0x1418913b0, "unknown_1418913b0", fn_1418913b0 as *const ()), // sub_1418913b0
    Replacement::draft_throws(0x1418915a0, "unknown_1418915a0", fn_1418915a0 as *const (), Throws::Discard), // sub_1418915a0
    Replacement::draft(0x141892050, "unknown_141892050", fn_141892050 as *const ()), // sub_141892050
    Replacement::draft(0x1418921a0, "unknown_1418921a0", fn_1418921a0 as *const ()), // sub_1418921a0
    Replacement::draft_throws(0x141892430, "unknown_141892430", fn_141892430 as *const (), Throws::Discard), // sub_141892430
    Replacement::draft(0x141892600, "unknown_141892600", tpl_34394::<0x10> as *const ()), // sub_141892600
    Replacement::draft(0x1418926a0, "unknown_1418926a0", fn_1418926a0 as *const ()), // sub_1418926a0
    Replacement::draft(0x141892840, "unknown_141892840", tpl_34809 as *const ()), // sub_141892840
    Replacement::draft_throws(0x141892a00, "unknown_141892a00", fn_141892a00 as *const (), Throws::Discard), // sub_141892a00
    Replacement::draft(0x141892bc0, "unknown_141892bc0", fn_141892bc0 as *const ()), // sub_141892bc0
    Replacement::draft(0x141893440, "unknown_141893440", fn_141893440 as *const ()), // sub_141893440
    Replacement::draft_throws(0x141893690, "unknown_141893690", fn_141893690 as *const (), Throws::Caught), // sub_141893690
    Replacement::draft_throws(0x1418943c0, "unknown_1418943c0", fn_1418943c0 as *const (), Throws::Discard), // sub_1418943c0
    Replacement::draft_throws(0x141894710, "unknown_141894710", fn_141894710 as *const (), Throws::Discard), // sub_141894710
    Replacement::draft_throws(0x141894740, "unknown_141894740", fn_141894740 as *const (), Throws::Discard), // sub_141894740
    Replacement::draft_throws(0x141894770, "unknown_141894770", fn_141894770 as *const (), Throws::Discard), // sub_141894770
    Replacement::draft(0x141894930, "unknown_141894930", fn_141894930 as *const ()), // sub_141894930
    Replacement::draft(0x141894a20, "unknown_141894a20", fn_141894a20 as *const ()), // sub_141894a20
    Replacement::draft(0x141894a70, "unknown_141894a70", fn_141894a70 as *const ()), // sub_141894a70
    Replacement::draft(0x141894b20, "unknown_141894b20", fn_141894b20 as *const ()), // sub_141894b20
    Replacement::draft(0x141894c00, "unknown_141894c00", fn_141894c00 as *const ()), // sub_141894c00
    Replacement::draft(0x141894c40, "unknown_141894c40", fn_141894c40 as *const ()), // sub_141894c40
    Replacement::draft(0x141894cf0, "unknown_141894cf0", fn_141894cf0 as *const ()), // sub_141894cf0
    Replacement::draft(0x141894dd0, "unknown_141894dd0", fn_141894dd0 as *const ()), // sub_141894dd0
    Replacement::draft(0x141894e10, "unknown_141894e10", tpl_34824::<0x28, 0x14240ab10> as *const ()), // sub_141894e10
    Replacement::draft(0x141894ed0, "unknown_141894ed0", fn_141894ed0 as *const ()), // sub_141894ed0
    Replacement::draft(0x141894fc0, "unknown_141894fc0", fn_141894fc0 as *const ()), // sub_141894fc0
    Replacement::draft(0x141895010, "unknown_141895010", fn_141895010 as *const ()), // sub_141895010
    Replacement::draft_throws(0x141895060, "unknown_141895060", fn_141895060 as *const (), Throws::Discard), // sub_141895060
    Replacement::draft_throws(0x141895090, "unknown_141895090", fn_141895090 as *const (), Throws::Discard), // sub_141895090
    Replacement::draft_throws(0x1418951f0, "unknown_1418951f0", fn_1418951f0 as *const (), Throws::Caught), // sub_1418951f0
    Replacement::draft_throws(0x141895af0, "unknown_141895af0", fn_141895af0 as *const (), Throws::Discard), // sub_141895af0
    Replacement::draft_throws(0x141895bd0, "unknown_141895bd0", fn_141895bd0 as *const (), Throws::Discard), // sub_141895bd0
    Replacement::draft_throws(0x141895ca0, "unknown_141895ca0", fn_141895ca0 as *const (), Throws::Caught), // sub_141895ca0
    Replacement::draft(0x141895da0, "unknown_141895da0", fn_141895da0 as *const ()), // sub_141895da0
    Replacement::draft_throws(0x141895e00, "unknown_141895e00", tpl_34833::<0x143a08418, 0x3, 0x141581080, 0x1424cb9c0> as *const (), Throws::Discard), // sub_141895e00
    Replacement::draft(0x141895e80, "unknown_141895e80", fn_141895e80 as *const ()), // sub_141895e80
    Replacement::draft_throws(0x141896270, "unknown_141896270", fn_141896270 as *const (), Throws::Discard), // sub_141896270
    Replacement::draft(0x141896430, "unknown_141896430", tpl_34835::<0x8, 0x140259450, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141896430
    Replacement::draft(0x1418964b0, "unknown_1418964b0", tpl_34835::<0x8, 0x140259450, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418964b0
    Replacement::draft(0x141896530, "unknown_141896530", fn_141896530 as *const ()), // sub_141896530
    Replacement::draft(0x141896650, "unknown_141896650", tpl_34837::<0x140259450, 0x20, 0x14240aa00> as *const ()), // sub_141896650
    Replacement::draft(0x1418966a0, "unknown_1418966a0", tpl_34837::<0x140259450, 0x20, 0x14240aa00> as *const ()), // sub_1418966a0
    Replacement::draft(0x1418966f0, "unknown_1418966f0", fn_1418966f0 as *const ()), // sub_1418966f0
    Replacement::draft(0x1418967b0, "unknown_1418967b0", fn_1418967b0 as *const ()), // sub_1418967b0
    Replacement::draft(0x141896830, "unknown_141896830", tpl_34840::<0x140258f10, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141896830
    Replacement::draft(0x1418968a0, "unknown_1418968a0", tpl_34840::<0x140258f10, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1418968a0
    Replacement::draft(0x141896910, "unknown_141896910", fn_141896910 as *const ()), // sub_141896910
    Replacement::draft(0x141896a20, "unknown_141896a20", tpl_34842::<0x14384484c, 0x580e0> as *const ()), // sub_141896a20
    Replacement::draft(0x141896a50, "unknown_141896a50", tpl_34842::<0x1438447c4, 0x580e0> as *const ()), // sub_141896a50
    Replacement::draft(0x141896ae0, "unknown_141896ae0", tpl_34843::<0x14384484c, 0x10, 0x50, 0x54, 0x1, 0x48, 0x140215610, 0x113> as *const ()), // sub_141896ae0
    Replacement::draft(0x141896b50, "unknown_141896b50", tpl_34843::<0x1438447c4, 0x10, 0x50, 0x54, 0x1, 0x48, 0x140215610, 0x113> as *const ()), // sub_141896b50
    Replacement::draft_throws(0x141896c40, "unknown_141896c40", tpl_94598::<0x140251800, 0x14309c488, 0x1426ed3c8, 0x1426ed3a0, 0x1426ed400, 0x1426ed3b0, 0x1426ed388, 0x1426ed408, 0x1426ed394, 0x1426ed3f8, 0x1426ed3c0, 0x1426ed380, 0x140254aa0, 0x1424b16a0, 0x140256bc0> as *const (), Throws::Discard), // sub_141896c40
    Replacement::draft_throws(0x141897270, "unknown_141897270", tpl_94598::<0x140251800, 0x14309c488, 0x1426ed3c8, 0x1426ed3a0, 0x1426ed400, 0x1426ed3b0, 0x1426ed388, 0x1426ed408, 0x1426ed394, 0x1426ed3f8, 0x1426ed3c0, 0x1426ed380, 0x140254aa0, 0x1424b16a0, 0x140256bc0> as *const (), Throws::Discard), // sub_141897270
    Replacement::draft_throws(0x1418978a0, "unknown_1418978a0", fn_1418978a0 as *const (), Throws::Caught), // sub_1418978a0
    Replacement::draft_throws(0x141897aa0, "unknown_141897aa0", tpl_34845::<0x18, 0x141896c40, 0x10> as *const (), Throws::Discard), // sub_141897aa0
    Replacement::draft_throws(0x141897ae0, "unknown_141897ae0", tpl_34846::<0x1401f3a20, 0x141896c40> as *const (), Throws::Discard), // sub_141897ae0
    Replacement::draft_throws(0x141897b60, "unknown_141897b60", fn_141897b60 as *const (), Throws::Caught), // sub_141897b60
    Replacement::draft_throws(0x141897d60, "unknown_141897d60", tpl_34845::<0x18, 0x141897270, 0x10> as *const (), Throws::Discard), // sub_141897d60
    Replacement::draft_throws(0x141897da0, "unknown_141897da0", fn_141897da0 as *const (), Throws::Discard), // sub_141897da0
    Replacement::draft(0x141897df0, "unknown_141897df0", fn_141897df0 as *const ()), // sub_141897df0
    Replacement::draft_throws(0x141897e90, "unknown_141897e90", fn_141897e90 as *const (), Throws::Caught), // sub_141897e90
    Replacement::draft_throws(0x141898160, "unknown_141898160", fn_141898160 as *const (), Throws::Caught), // sub_141898160
    Replacement::draft_throws(0x141898730, "unknown_141898730", fn_141898730 as *const (), Throws::Caught), // sub_141898730
    Replacement::draft_throws(0x141899900, "unknown_141899900", fn_141899900 as *const (), Throws::Discard), // sub_141899900
    Replacement::draft_throws(0x141899a30, "unknown_141899a30", fn_141899a30 as *const (), Throws::Discard), // sub_141899a30
    Replacement::draft_throws(0x14189a110, "unknown_14189a110", fn_14189a110 as *const (), Throws::Discard), // sub_14189a110
    Replacement::draft_throws(0x14189a550, "unknown_14189a550", fn_14189a550 as *const (), Throws::Discard), // sub_14189a550
    Replacement::draft(0x14189a640, "unknown_14189a640", fn_14189a640 as *const ()), // sub_14189a640
    Replacement::draft_throws(0x14189a660, "unknown_14189a660", fn_14189a660 as *const (), Throws::Discard), // sub_14189a660
    Replacement::draft_throws(0x14189a6d0, "unknown_14189a6d0", fn_14189a6d0 as *const (), Throws::Discard), // sub_14189a6d0
    Replacement::draft_throws(0x14189a880, "unknown_14189a880", fn_14189a880 as *const (), Throws::Caught), // sub_14189a880
    Replacement::draft_throws(0x14189ab60, "unknown_14189ab60", fn_14189ab60 as *const (), Throws::Caught), // sub_14189ab60
    Replacement::draft_throws(0x14189b140, "unknown_14189b140", fn_14189b140 as *const (), Throws::Caught), // sub_14189b140
    Replacement::draft_throws(0x14189b290, "unknown_14189b290", fn_14189b290 as *const (), Throws::Caught), // sub_14189b290
    Replacement::draft(0x14189b410, "unknown_14189b410", fn_14189b410 as *const ()), // sub_14189b410
    Replacement::draft_throws(0x14189b5a0, "unknown_14189b5a0", fn_14189b5a0 as *const (), Throws::Discard), // sub_14189b5a0
    Replacement::draft(0x14189b770, "unknown_14189b770", fn_14189b770 as *const ()), // sub_14189b770
    Replacement::draft_throws(0x14189b9a0, "unknown_14189b9a0", fn_14189b9a0 as *const (), Throws::Caught), // sub_14189b9a0
    Replacement::draft(0x14189c8b0, "unknown_14189c8b0", fn_14189c8b0 as *const ()), // sub_14189c8b0
    Replacement::draft(0x14189c990, "unknown_14189c990", fn_14189c990 as *const ()), // sub_14189c990
    Replacement::draft_throws(0x14189c9a0, "unknown_14189c9a0", fn_14189c9a0 as *const (), Throws::Discard), // sub_14189c9a0
    Replacement::draft_throws(0x14189cbf0, "unknown_14189cbf0", fn_14189cbf0 as *const (), Throws::Caught), // sub_14189cbf0
    Replacement::draft_throws(0x14189ce70, "unknown_14189ce70", fn_14189ce70 as *const (), Throws::Discard), // sub_14189ce70
    Replacement::draft(0x14189cfd0, "unknown_14189cfd0", fn_14189cfd0 as *const ()), // sub_14189cfd0
    Replacement::draft(0x14189cfe0, "unknown_14189cfe0", fn_14189cfe0 as *const ()), // sub_14189cfe0
    Replacement::draft(0x14189cff0, "unknown_14189cff0", fn_14189cff0 as *const ()), // sub_14189cff0
    Replacement::draft(0x14189d060, "unknown_14189d060", fn_14189d060 as *const ()), // sub_14189d060
    Replacement::draft(0x14189d1a0, "unknown_14189d1a0", fn_14189d1a0 as *const ()), // sub_14189d1a0
    Replacement::draft(0x14189d1d0, "unknown_14189d1d0", tpl_34869::<0x143a11cb3, 0x143a11cb0, 0x143a11cb8, 0x143a11cb4, 0x14020d4a0, 0x143c90338, 0x142489ee0, 0x140251aa0, 0x1426467e8, 0x14189d382> as *const ()), // sub_14189d1d0
    Replacement::draft(0x14189d3c0, "unknown_14189d3c0", fn_14189d3c0 as *const ()), // sub_14189d3c0
    Replacement::draft(0x14189d4b0, "unknown_14189d4b0", tpl_34871::<0x14020d4a0, 0x143a11fcb, 0x143a11fc8, 0x143a11fd0, 0x143a11fcc, 0x1401e8cc0, 0x1401e8c00> as *const ()), // sub_14189d4b0
    Replacement::draft(0x14189d600, "unknown_14189d600", fn_14189d600 as *const ()), // sub_14189d600
    Replacement::draft(0x14189d660, "unknown_14189d660", fn_14189d660 as *const ()), // sub_14189d660
    Replacement::draft_throws(0x14189d690, "unknown_14189d690", fn_14189d690 as *const (), Throws::Caught), // sub_14189d690
    Replacement::draft_throws(0x14189de80, "unknown_14189de80", fn_14189de80 as *const (), Throws::Discard), // sub_14189de80
    Replacement::draft(0x14189ef50, "unknown_14189ef50", fn_14189ef50 as *const ()), // sub_14189ef50
    Replacement::draft(0x14189ef70, "unknown_14189ef70", fn_14189ef70 as *const ()), // sub_14189ef70
    Replacement::draft_throws(0x14189f2a0, "unknown_14189f2a0", fn_14189f2a0 as *const (), Throws::Discard), // sub_14189f2a0
    Replacement::draft_throws(0x14189f350, "unknown_14189f350", fn_14189f350 as *const (), Throws::Discard), // sub_14189f350
    Replacement::draft_throws(0x14189f640, "unknown_14189f640", fn_14189f640 as *const (), Throws::Discard), // sub_14189f640
    Replacement::draft_throws(0x14189f6f0, "unknown_14189f6f0", fn_14189f6f0 as *const (), Throws::Caught), // sub_14189f6f0
    Replacement::draft(0x14189f8d0, "unknown_14189f8d0", fn_14189f8d0 as *const ()), // sub_14189f8d0
    Replacement::draft(0x14189fa90, "unknown_14189fa90", fn_14189fa90 as *const ()), // sub_14189fa90
    Replacement::draft_throws(0x14189fb20, "unknown_14189fb20", fn_14189fb20 as *const (), Throws::Caught), // sub_14189fb20
    Replacement::draft(0x1418a01a0, "unknown_1418a01a0", fn_1418a01a0 as *const ()), // sub_1418a01a0
    Replacement::draft(0x1418a01b0, "unknown_1418a01b0", fn_1418a01b0 as *const ()), // sub_1418a01b0
    Replacement::draft_throws(0x1418a02b0, "unknown_1418a02b0", fn_1418a02b0 as *const (), Throws::Discard), // sub_1418a02b0
    Replacement::draft(0x1418a05f0, "bigfish_1418a05f0", tpl_34885::<0x10> as *const ()), // bigfish::UIPhotoModeModel::bigfish::UIPhotoModeModel::vfunc_1
    Replacement::draft(0x1418a0600, "bigfish_1418a0600", fn_1418a0600 as *const ()), // sub_1418a0600
    Replacement::draft_throws(0x1418a0760, "bigfish_1418a0760", fn_1418a0760 as *const (), Throws::Discard), // sub_1418a0760
    Replacement::draft_throws(0x1418a0a90, "bigfish_1418a0a90", fn_1418a0a90 as *const (), Throws::Discard), // sub_1418a0a90
    Replacement::draft(0x1418a1130, "bigfish_1418a1130", fn_1418a1130 as *const ()), // sub_1418a1130
    Replacement::draft(0x1418a1170, "bigfish_1418a1170", tpl_34835::<0x8, 0x1402611c0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418a1170
    Replacement::draft(0x1418a11f0, "bigfish_1418a11f0", tpl_34835::<0x8, 0x1402611c0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418a11f0
    Replacement::draft(0x1418a1270, "bigfish_1418a1270", tpl_34835::<0x8, 0x140261060, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418a1270
    Replacement::draft(0x1418a12f0, "bigfish_1418a12f0", tpl_34835::<0x8, 0x140261110, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418a12f0
    Replacement::draft(0x1418a1370, "bigfish_1418a1370", tpl_34835::<0x8, 0x1402611c0, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418a1370
    Replacement::draft(0x1418a13f0, "bigfish_1418a13f0", tpl_34837::<0x1402611c0, 0x20, 0x14240aa00> as *const ()), // sub_1418a13f0
    Replacement::draft(0x1418a1440, "bigfish_1418a1440", tpl_34837::<0x1402611c0, 0x20, 0x14240aa00> as *const ()), // sub_1418a1440
    Replacement::draft(0x1418a1490, "bigfish_1418a1490", tpl_34837::<0x140261060, 0x20, 0x14240aa00> as *const ()), // sub_1418a1490
    Replacement::draft(0x1418a14e0, "bigfish_1418a14e0", tpl_34837::<0x140261110, 0x20, 0x14240aa00> as *const ()), // sub_1418a14e0
    Replacement::draft(0x1418a1530, "bigfish_1418a1530", tpl_34837::<0x1402611c0, 0x20, 0x14240aa00> as *const ()), // sub_1418a1530
    Replacement::draft_throws(0x1418a1580, "bigfish_1418a1580", fn_1418a1580 as *const (), Throws::Caught), // sub_1418a1580
    Replacement::draft_throws(0x1418a1780, "bigfish_1418a1780", fn_1418a1780 as *const (), Throws::Caught), // sub_1418a1780
    Replacement::draft_throws(0x1418a1980, "bigfish_1418a1980", fn_1418a1980 as *const (), Throws::Caught), // sub_1418a1980
    Replacement::draft_throws(0x1418a1b80, "bigfish_1418a1b80", fn_1418a1b80 as *const (), Throws::Caught), // sub_1418a1b80
    Replacement::draft_throws(0x1418a1d80, "bigfish_1418a1d80", fn_1418a1d80 as *const (), Throws::Caught), // sub_1418a1d80
    Replacement::draft_throws(0x1418a2070, "bigfish_1418a2070", tpl_94649::<0x14025ccb0, 0x14309c488, 0x1426f02c0, 0x1426f02a4, 0x1426f02d0, 0x1426f0354, 0x1426f0290, 0x140254aa0, 0x1424b16a0, 0x14025f6d0> as *const (), Throws::Discard), // sub_1418a2070
    Replacement::draft_throws(0x1418a23f0, "bigfish_1418a23f0", tpl_94650::<0x14025cb90, 0x14309c488, 0x1426f0300, 0x1426f02f0, 0x1426f02f8, 0x140254aa0, 0x1424b16a0, 0x14025f490> as *const (), Throws::Discard), // sub_1418a23f0
    Replacement::draft_throws(0x1418a2630, "bigfish_1418a2630", tpl_94651::<0x14025ca60, 0x14309c488, 0x1426f0360, 0x1426f0300, 0x1426f02f0, 0x1426f02f8, 0x140254aa0, 0x1424b16a0, 0x14025f260> as *const (), Throws::Discard), // sub_1418a2630
    Replacement::draft_throws(0x1418a2900, "bigfish_1418a2900", tpl_94649::<0x14025ccb0, 0x14309c488, 0x1426f02ac, 0x1426f0260, 0x1426f02b8, 0x1426f029c, 0x1426f0288, 0x140254aa0, 0x1424b16a0, 0x14025f6d0> as *const (), Throws::Discard), // sub_1418a2900
    Replacement::draft_throws(0x1418a2c80, "bigfish_1418a2c80", tpl_94649::<0x14025ccb0, 0x14309c488, 0x1426f02ac, 0x1426f0260, 0x1426f02b8, 0x1426f029c, 0x1426f0288, 0x140254aa0, 0x1424b16a0, 0x14025f6d0> as *const (), Throws::Discard), // sub_1418a2c80
    Replacement::draft_throws(0x1418a3000, "bigfish_1418a3000", tpl_34893::<0x18, 0x1418a2070, 0x10> as *const (), Throws::Discard), // sub_1418a3000
    Replacement::draft_throws(0x1418a3040, "bigfish_1418a3040", tpl_34846::<0x1401f3a20, 0x1418a2070> as *const (), Throws::Discard), // sub_1418a3040
    Replacement::draft_throws(0x1418a30c0, "bigfish_1418a30c0", tpl_34894::<0x18, 0x1418a23f0, 0x10> as *const (), Throws::Discard), // sub_1418a30c0
    Replacement::draft_throws(0x1418a3100, "bigfish_1418a3100", tpl_34846::<0x1401f3a20, 0x1418a23f0> as *const (), Throws::Discard), // sub_1418a3100
    Replacement::draft_throws(0x1418a3180, "bigfish_1418a3180", tpl_34895::<0x18, 0x1418a2630, 0x10> as *const (), Throws::Discard), // sub_1418a3180
    Replacement::draft_throws(0x1418a31c0, "bigfish_1418a31c0", tpl_34846::<0x1401f3a20, 0x1418a2630> as *const (), Throws::Discard), // sub_1418a31c0
    Replacement::draft_throws(0x1418a3240, "bigfish_1418a3240", tpl_34893::<0x18, 0x1418a2900, 0x10> as *const (), Throws::Discard), // sub_1418a3240
    Replacement::draft_throws(0x1418a3280, "bigfish_1418a3280", tpl_34846::<0x1401f3a20, 0x1418a2900> as *const (), Throws::Discard), // sub_1418a3280
    Replacement::draft_throws(0x1418a3300, "bigfish_1418a3300", tpl_34893::<0x18, 0x1418a2c80, 0x10> as *const (), Throws::Discard), // sub_1418a3300
    Replacement::draft_throws(0x1418a3340, "bigfish_1418a3340", tpl_34846::<0x1401f3a20, 0x1418a2c80> as *const (), Throws::Discard), // sub_1418a3340
    Replacement::draft_throws(0x1418a33c0, "bigfish_1418a33c0", fn_1418a33c0 as *const (), Throws::Caught), // sub_1418a33c0
    Replacement::draft_throws(0x1418a3f80, "bigfish_1418a3f80", fn_1418a3f80 as *const (), Throws::Caught), // sub_1418a3f80
    Replacement::draft_throws(0x1418a4210, "bigfish_1418a4210", fn_1418a4210 as *const (), Throws::Caught), // sub_1418a4210
    Replacement::draft_throws(0x1418a45f0, "bigfish_1418a45f0", fn_1418a45f0 as *const (), Throws::Discard), // sub_1418a45f0
    Replacement::draft(0x1418a4840, "bigfish_1418a4840", fn_1418a4840 as *const ()), // sub_1418a4840
    Replacement::draft_throws(0x1418a4a10, "bigfish_1418a4a10", fn_1418a4a10 as *const (), Throws::Caught), // sub_1418a4a10
    Replacement::draft(0x1418a5b40, "bigfish_1418a5b40", fn_1418a5b40 as *const ()), // sub_1418a5b40
    Replacement::draft_throws(0x1418a5d00, "bigfish_1418a5d00", fn_1418a5d00 as *const (), Throws::Caught), // sub_1418a5d00
    Replacement::draft_throws(0x1418a5f80, "bigfish_1418a5f80", fn_1418a5f80 as *const (), Throws::Caught), // sub_1418a5f80
    Replacement::draft_throws(0x1418a7210, "bigfish_1418a7210", fn_1418a7210 as *const (), Throws::Caught), // sub_1418a7210
    Replacement::draft_throws(0x1418a7510, "bigfish_1418a7510", fn_1418a7510 as *const (), Throws::Discard), // sub_1418a7510
    Replacement::draft_throws(0x1418a75f0, "bigfish_1418a75f0", fn_1418a75f0 as *const (), Throws::Caught), // sub_1418a75f0
    Replacement::draft_throws(0x1418a7750, "bigfish_1418a7750", fn_1418a7750 as *const (), Throws::Caught), // sub_1418a7750
    Replacement::draft_throws(0x1418a78b0, "bigfish_1418a78b0", fn_1418a78b0 as *const (), Throws::Discard), // sub_1418a78b0
    Replacement::draft_throws(0x1418a7a30, "bigfish_1418a7a30", fn_1418a7a30 as *const (), Throws::Caught), // sub_1418a7a30
    Replacement::draft(0x1418a7dc0, "bigfish_1418a7dc0", fn_1418a7dc0 as *const ()), // sub_1418a7dc0
    Replacement::draft_throws(0x1418a7de0, "bigfish_1418a7de0", tpl_34833::<0x143a08418, 0x3, 0x141581080, 0x1424cb9c0> as *const (), Throws::Discard), // sub_1418a7de0
    Replacement::draft_throws(0x1418a7e60, "bigfish_1418a7e60", fn_1418a7e60 as *const (), Throws::Discard), // sub_1418a7e60
    Replacement::draft_throws(0x1418a8260, "bigfish_1418a8260", fn_1418a8260 as *const (), Throws::Caught), // sub_1418a8260
    Replacement::draft_throws(0x1418a9360, "bigfish_1418a9360", fn_1418a9360 as *const (), Throws::Caught), // sub_1418a9360
    Replacement::draft(0x1418aa480, "bigfish_1418aa480", fn_1418aa480 as *const ()), // sub_1418aa480
    Replacement::draft_throws(0x1418aa500, "bigfish_1418aa500", fn_1418aa500 as *const (), Throws::Discard), // sub_1418aa500
    Replacement::draft_throws(0x1418aa5a0, "bigfish_1418aa5a0", fn_1418aa5a0 as *const (), Throws::Caught), // sub_1418aa5a0
    Replacement::draft_throws(0x1418aa830, "bigfish_1418aa830", fn_1418aa830 as *const (), Throws::Caught), // sub_1418aa830
    Replacement::draft(0x1418aab80, "bigfish_1418aab80", fn_1418aab80 as *const ()), // sub_1418aab80
    Replacement::draft(0x1418aac70, "cohtml_1418aac70", fn_1418aac70 as *const ()), // sub_1418aac70
    Replacement::draft(0x1418aad20, "cohtml_1418aad20", fn_1418aad20 as *const ()), // sub_1418aad20
    Replacement::draft(0x1418aadd0, "cohtml_1418aadd0", fn_1418aadd0 as *const ()), // sub_1418aadd0
    Replacement::draft_throws(0x1418aae80, "unknown_1418aae80", fn_1418aae80 as *const (), Throws::Discard), // sub_1418aae80
    Replacement::draft(0x1418ab100, "unknown_1418ab100", fn_1418ab100 as *const ()), // sub_1418ab100
    Replacement::draft_throws(0x1418ab1c0, "unknown_1418ab1c0", fn_1418ab1c0 as *const (), Throws::Caught), // sub_1418ab1c0
    Replacement::draft_throws(0x1418ab460, "unknown_1418ab460", fn_1418ab460 as *const (), Throws::Caught), // sub_1418ab460
    Replacement::draft_throws(0x1418ab690, "unknown_1418ab690", fn_1418ab690 as *const (), Throws::Discard), // sub_1418ab690
    Replacement::draft_throws(0x1418abaa0, "unknown_1418abaa0", fn_1418abaa0 as *const (), Throws::Caught), // sub_1418abaa0
    Replacement::draft_throws(0x1418abdb0, "unknown_1418abdb0", fn_1418abdb0 as *const (), Throws::Caught), // sub_1418abdb0
    Replacement::draft(0x1418abe80, "unknown_1418abe80", fn_1418abe80 as *const ()), // sub_1418abe80
    Replacement::draft_throws(0x1418ac190, "unknown_1418ac190", fn_1418ac190 as *const (), Throws::Discard), // sub_1418ac190
    Replacement::draft_throws(0x1418ac3a0, "unknown_1418ac3a0", tpl_34833::<0x143a08418, 0x3, 0x141581080, 0x1424cb9c0> as *const (), Throws::Discard), // sub_1418ac3a0
    Replacement::draft(0x1418ac420, "unknown_1418ac420", tpl_1743::<0x50> as *const ()), // sub_1418ac420
    Replacement::draft(0x1418ac430, "unknown_1418ac430", fn_1418ac430 as *const ()), // sub_1418ac430
    Replacement::draft_throws(0x1418ac490, "unknown_1418ac490", fn_1418ac490 as *const (), Throws::Discard), // sub_1418ac490
    Replacement::draft_throws(0x1418ac870, "unknown_1418ac870", tpl_94685::<0x143a153e8, 0x14309c488, 0x1426f1c98, 0x1426f1c90, 0x1426f1c88, 0x143a153f0, 0x140254aa0, 0x1424b16a0, 0x143a153ec, 0x143a153e0, 0x14025f490> as *const (), Throws::Discard), // sub_1418ac870
    Replacement::draft_throws(0x1418acaa0, "unknown_1418acaa0", tpl_94685::<0x143a15378, 0x14309c488, 0x1426f1c98, 0x1426f1c90, 0x1426f1c88, 0x143a15380, 0x140254aa0, 0x1424b16a0, 0x143a1537c, 0x143a15370, 0x14025f490> as *const (), Throws::Discard), // sub_1418acaa0
    Replacement::draft_throws(0x1418accd0, "unknown_1418accd0", fn_1418accd0 as *const (), Throws::Caught), // sub_1418accd0
    Replacement::draft_throws(0x1418acf00, "unknown_1418acf00", tpl_34924::<0x143a153e8, 0x1418ac870, 0x143a153e0> as *const (), Throws::Discard), // sub_1418acf00
    Replacement::draft_throws(0x1418acf40, "unknown_1418acf40", tpl_34925::<0x143a153d0, 0x1401f3a20, 0x1418ac870> as *const (), Throws::Discard), // sub_1418acf40
    Replacement::draft_throws(0x1418acfb0, "unknown_1418acfb0", fn_1418acfb0 as *const (), Throws::Caught), // sub_1418acfb0
    Replacement::draft_throws(0x1418ad1e0, "unknown_1418ad1e0", tpl_34924::<0x143a15378, 0x1418acaa0, 0x143a15370> as *const (), Throws::Discard), // sub_1418ad1e0
    Replacement::draft_throws(0x1418ad220, "unknown_1418ad220", tpl_34925::<0x143a15360, 0x1401f3a20, 0x1418acaa0> as *const (), Throws::Discard), // sub_1418ad220
    Replacement::draft_throws(0x1418ad290, "bigfish_1418ad290", fn_1418ad290 as *const (), Throws::Discard), // bigfish::UIInterfaceSettings::bigfish::UIInterfaceSettings::vfunc_4
    Replacement::draft_throws(0x1418ad540, "bigfish_1418ad540", fn_1418ad540 as *const (), Throws::Caught), // bigfish::UIInterfaceSettings::bigfish::UIInterfaceSettings::vfunc_2
    Replacement::draft(0x1418ada90, "bigfish_1418ada90", tpl_34885::<0x10> as *const ()), // bigfish::UIInterfaceSettings::bigfish::UIInterfaceSettings::vfunc_1
    Replacement::draft(0x1418adaa0, "bigfish_1418adaa0", fn_1418adaa0 as *const ()), // sub_1418adaa0
    Replacement::draft_throws(0x1418add90, "bigfish_1418add90", fn_1418add90 as *const (), Throws::Caught), // sub_1418add90
    Replacement::draft_throws(0x1418ae170, "bigfish_1418ae170", fn_1418ae170 as *const (), Throws::Discard), // sub_1418ae170
    Replacement::draft_throws(0x1418ae310, "bigfish_1418ae310", fn_1418ae310 as *const (), Throws::Discard), // sub_1418ae310
    Replacement::draft_throws(0x1418ae450, "bigfish_1418ae450", fn_1418ae450 as *const (), Throws::Discard), // sub_1418ae450
    Replacement::draft_throws(0x1418ae700, "bigfish_1418ae700", fn_1418ae700 as *const (), Throws::Discard), // sub_1418ae700
    Replacement::draft_throws(0x1418ae750, "bigfish_1418ae750", fn_1418ae750 as *const (), Throws::Discard), // sub_1418ae750
    Replacement::draft_throws(0x1418ae8b0, "bigfish_1418ae8b0", fn_1418ae8b0 as *const (), Throws::Caught), // sub_1418ae8b0
    Replacement::draft(0x1418aec10, "bigfish_1418aec10", fn_1418aec10 as *const ()), // sub_1418aec10
    Replacement::draft_throws(0x1418aec50, "bigfish_1418aec50", fn_1418aec50 as *const (), Throws::Discard), // sub_1418aec50
    Replacement::draft_throws(0x1418aed10, "bigfish_1418aed10", fn_1418aed10 as *const (), Throws::Caught), // bigfish::UIInputRebinds::bigfish::UIInputRebinds::vfunc_4
    Replacement::draft_throws(0x1418af750, "bigfish_1418af750", fn_1418af750 as *const (), Throws::Discard), // bigfish::UIInputRebinds::bigfish::UIInputRebinds::vfunc_3
    Replacement::draft_throws(0x1418af7c0, "bigfish_1418af7c0", fn_1418af7c0 as *const (), Throws::Discard), // bigfish::UIInputRebinds::bigfish::UIInputRebinds::vfunc_1
    Replacement::draft(0x1418af970, "unknown_1418af970", fn_1418af970 as *const ()), // sub_1418af970
    Replacement::draft(0x1418afb40, "unknown_1418afb40", fn_1418afb40 as *const ()), // sub_1418afb40
    Replacement::draft_throws(0x1418afbc0, "unknown_1418afbc0", fn_1418afbc0 as *const (), Throws::Discard), // sub_1418afbc0
    Replacement::draft(0x1418afe10, "unknown_1418afe10", fn_1418afe10 as *const ()), // sub_1418afe10
    Replacement::draft_throws(0x1418b00d0, "unknown_1418b00d0", fn_1418b00d0 as *const (), Throws::Discard), // sub_1418b00d0
    Replacement::draft(0x1418b0290, "unknown_1418b0290", fn_1418b0290 as *const ()), // sub_1418b0290
    Replacement::draft(0x1418b02c0, "unknown_1418b02c0", fn_1418b02c0 as *const ()), // sub_1418b02c0
    Replacement::draft_throws(0x1418b02f0, "unknown_1418b02f0", fn_1418b02f0 as *const (), Throws::Caught), // sub_1418b02f0
    Replacement::draft(0x1418b06e0, "unknown_1418b06e0", fn_1418b06e0 as *const ()), // sub_1418b06e0
    Replacement::draft(0x1418b0960, "unknown_1418b0960", fn_1418b0960 as *const ()), // sub_1418b0960
    Replacement::draft_throws(0x1418b09a0, "unknown_1418b09a0", fn_1418b09a0 as *const (), Throws::Discard), // sub_1418b09a0
    Replacement::draft(0x1418b0aa0, "unknown_1418b0aa0", tpl_34947::<0x3, 0x8, 0x4, 0x1430a01f8, 0x1426d01e0, 0x1426cdcb0> as *const ()), // sub_1418b0aa0
    Replacement::draft(0x1418b0b80, "unknown_1418b0b80", fn_1418b0b80 as *const ()), // sub_1418b0b80
    Replacement::draft(0x1418b0d90, "unknown_1418b0d90", fn_1418b0d90 as *const ()), // sub_1418b0d90
    Replacement::draft_throws(0x1418b0ed0, "unknown_1418b0ed0", fn_1418b0ed0 as *const (), Throws::Discard), // sub_1418b0ed0
    Replacement::draft_throws(0x1418b1050, "unknown_1418b1050", fn_1418b1050 as *const (), Throws::Discard), // sub_1418b1050
    Replacement::draft(0x1418b1140, "unknown_1418b1140", fn_1418b1140 as *const ()), // sub_1418b1140
    Replacement::draft(0x1418b1270, "unknown_1418b1270", fn_1418b1270 as *const ()), // sub_1418b1270
    Replacement::draft(0x1418b12c0, "unknown_1418b12c0", fn_1418b12c0 as *const ()), // sub_1418b12c0
    Replacement::draft_throws(0x1418b13b0, "unknown_1418b13b0", tpl_94685::<0x143a16a88, 0x14309c488, 0x1426f4190, 0x1426f43c0, 0x1426f43ec, 0x143a16a90, 0x140254aa0, 0x1424b16a0, 0x143a16a8c, 0x143a16a80, 0x14025f490> as *const (), Throws::Discard), // sub_1418b13b0
    Replacement::draft_throws(0x1418b15e0, "unknown_1418b15e0", tpl_94685::<0x143a16af8, 0x14309c488, 0x1426f4190, 0x1426f43c0, 0x1426f43ec, 0x143a16b00, 0x140254aa0, 0x1424b16a0, 0x143a16afc, 0x143a16af0, 0x14025f490> as *const (), Throws::Discard), // sub_1418b15e0
    Replacement::draft_throws(0x1418b1810, "unknown_1418b1810", tpl_94685::<0x143a16c78, 0x14309c488, 0x1426f4190, 0x1426f43c0, 0x1426f43ec, 0x143a16c80, 0x140254aa0, 0x1424b16a0, 0x143a16c7c, 0x143a16c70, 0x14025f490> as *const (), Throws::Discard), // sub_1418b1810
    Replacement::draft_throws(0x1418b1a40, "unknown_1418b1a40", tpl_94685::<0x143a168f8, 0x14309c488, 0x1426f4190, 0x1426f43c0, 0x1426f43ec, 0x143a16900, 0x140254aa0, 0x1424b16a0, 0x143a168fc, 0x143a168f0, 0x14025f490> as *const (), Throws::Discard), // sub_1418b1a40
    Replacement::draft_throws(0x1418b1c70, "unknown_1418b1c70", fn_1418b1c70 as *const (), Throws::Caught), // sub_1418b1c70
    Replacement::draft_throws(0x1418b1ea0, "unknown_1418b1ea0", tpl_34924::<0x143a16a88, 0x1418b13b0, 0x143a16a80> as *const (), Throws::Discard), // sub_1418b1ea0
    Replacement::draft_throws(0x1418b1ee0, "unknown_1418b1ee0", tpl_34925::<0x143a16a70, 0x1401f3a20, 0x1418b13b0> as *const (), Throws::Discard), // sub_1418b1ee0
    Replacement::draft_throws(0x1418b1f50, "unknown_1418b1f50", fn_1418b1f50 as *const (), Throws::Caught), // sub_1418b1f50
    Replacement::draft_throws(0x1418b2180, "unknown_1418b2180", tpl_34924::<0x143a16af8, 0x1418b15e0, 0x143a16af0> as *const (), Throws::Discard), // sub_1418b2180
    Replacement::draft_throws(0x1418b21c0, "unknown_1418b21c0", tpl_34925::<0x143a16ae0, 0x1401f3a20, 0x1418b15e0> as *const (), Throws::Discard), // sub_1418b21c0
    Replacement::draft_throws(0x1418b2230, "unknown_1418b2230", fn_1418b2230 as *const (), Throws::Caught), // sub_1418b2230
    Replacement::draft_throws(0x1418b2460, "unknown_1418b2460", tpl_34924::<0x143a16c78, 0x1418b1810, 0x143a16c70> as *const (), Throws::Discard), // sub_1418b2460
    Replacement::draft_throws(0x1418b24a0, "unknown_1418b24a0", tpl_34925::<0x143a16c60, 0x1401f3a20, 0x1418b1810> as *const (), Throws::Discard), // sub_1418b24a0
    Replacement::draft_throws(0x1418b2510, "unknown_1418b2510", fn_1418b2510 as *const (), Throws::Caught), // sub_1418b2510
    Replacement::draft_throws(0x1418b2740, "unknown_1418b2740", tpl_34924::<0x143a168f8, 0x1418b1a40, 0x143a168f0> as *const (), Throws::Discard), // sub_1418b2740
    Replacement::draft_throws(0x1418b2780, "unknown_1418b2780", tpl_34925::<0x143a168e0, 0x1401f3a20, 0x1418b1a40> as *const (), Throws::Discard), // sub_1418b2780
    Replacement::draft_throws(0x1418b27f0, "unknown_1418b27f0", fn_1418b27f0 as *const (), Throws::Discard), // sub_1418b27f0
    Replacement::draft_throws(0x1418b2970, "unknown_1418b2970", fn_1418b2970 as *const (), Throws::Discard), // sub_1418b2970
    Replacement::draft_throws(0x1418b3aa0, "unknown_1418b3aa0", fn_1418b3aa0 as *const (), Throws::Discard), // sub_1418b3aa0
    Replacement::draft_throws(0x1418b45a0, "unknown_1418b45a0", fn_1418b45a0 as *const (), Throws::Discard), // sub_1418b45a0
    Replacement::draft_throws(0x1418b6340, "unknown_1418b6340", fn_1418b6340 as *const (), Throws::Discard), // sub_1418b6340
    Replacement::draft_throws(0x1418b63b0, "unknown_1418b63b0", fn_1418b63b0 as *const (), Throws::Discard), // sub_1418b63b0
    Replacement::draft_throws(0x1418b6690, "bigfish_1418b6690", fn_1418b6690 as *const (), Throws::Discard), // bigfish::UIGraphicsSettings::bigfish::UIGraphicsSettings::vfunc_4
    Replacement::draft_throws(0x1418b6d90, "bigfish_1418b6d90", fn_1418b6d90 as *const (), Throws::Discard), // bigfish::UIGraphicsSettings::bigfish::UIGraphicsSettings::vfunc_3
    Replacement::draft_throws(0x1418b6dc0, "bigfish_1418b6dc0", fn_1418b6dc0 as *const (), Throws::Discard), // bigfish::UIGraphicsSettings::bigfish::UIGraphicsSettings::vfunc_2
    Replacement::draft_throws(0x1418b6de0, "bigfish_1418b6de0", fn_1418b6de0 as *const (), Throws::Discard), // bigfish::UIGraphicsSettings::bigfish::UIGraphicsSettings::vfunc_1
    Replacement::draft(0x1418b6eb0, "bigfish_1418b6eb0", fn_1418b6eb0 as *const ()), // sub_1418b6eb0
    Replacement::draft(0x1418b8970, "bigfish_1418b8970", fn_1418b8970 as *const ()), // sub_1418b8970
    Replacement::draft(0x1418b89d0, "bigfish_1418b89d0", fn_1418b89d0 as *const ()), // sub_1418b89d0
    Replacement::draft(0x1418b8a00, "bigfish_1418b8a00", tpl_28185::<0x14397d82f> as *const ()), // sub_1418b8a00
    Replacement::draft(0x1418b8a10, "bigfish_1418b8a10", tpl_28185::<0x14397d82e> as *const ()), // sub_1418b8a10
    Replacement::draft(0x1418b8a20, "bigfish_1418b8a20", fn_1418b8a20 as *const ()), // sub_1418b8a20
    Replacement::draft(0x1418b8b10, "bigfish_1418b8b10", fn_1418b8b10 as *const ()), // sub_1418b8b10
    Replacement::draft(0x1418b8d20, "bigfish_1418b8d20", fn_1418b8d20 as *const ()), // sub_1418b8d20
    Replacement::draft_throws(0x1418b8f90, "bigfish_1418b8f90", fn_1418b8f90 as *const (), Throws::Caught), // sub_1418b8f90
    Replacement::draft(0x1418b9140, "bigfish_1418b9140", fn_1418b9140 as *const ()), // sub_1418b9140
    Replacement::draft(0x1418b91e0, "bigfish_1418b91e0", fn_1418b91e0 as *const ()), // sub_1418b91e0
    Replacement::draft(0x1418b97b0, "bigfish_1418b97b0", fn_1418b97b0 as *const ()), // sub_1418b97b0
    Replacement::draft(0x1418b9950, "bigfish_1418b9950", fn_1418b9950 as *const ()), // sub_1418b9950
    Replacement::draft_throws(0x1418b9ab0, "bigfish_1418b9ab0", fn_1418b9ab0 as *const (), Throws::Caught), // bigfish::UIGameplaySettings::bigfish::UIGameplaySettings::vfunc_4
    Replacement::draft(0x1418ba1e0, "bigfish_1418ba1e0", fn_1418ba1e0 as *const ()), // sub_1418ba1e0
    Replacement::draft(0x1418ba440, "bigfish_1418ba440", tpl_34947::<0x3, 0x8, 0x4, 0x1430a01f8, 0x1426d01e0, 0x1426cdcb0> as *const ()), // sub_1418ba440
    Replacement::draft(0x1418ba520, "bigfish_1418ba520", fn_1418ba520 as *const ()), // sub_1418ba520
    Replacement::draft(0x1418ba570, "bigfish_1418ba570", tpl_34970::<0x8, 0x68> as *const ()), // sub_1418ba570
    Replacement::draft(0x1418ba5c0, "bigfish_1418ba5c0", fn_1418ba5c0 as *const ()), // bigfish::UIDemoModel::bigfish::UIDemoModel::vfunc_1
    Replacement::draft(0x1418ba6c0, "cohtml_1418ba6c0", tpl_34971::<0x1430a1258, 0x10, 0x1430a1268, 0x1430a11d8, 0x18, 0x142646d10> as *const ()), // sub_1418ba6c0
    Replacement::draft_throws(0x1418ba730, "bigfish_1418ba730", fn_1418ba730 as *const (), Throws::Discard), // bigfish::UIAudioSettings::bigfish::UIAudioSettings::vfunc_4
    Replacement::draft(0x1418ba760, "unknown_1418ba760", fn_1418ba760 as *const ()), // sub_1418ba760
    Replacement::draft_throws(0x1418ba9e0, "unknown_1418ba9e0", fn_1418ba9e0 as *const (), Throws::Discard), // sub_1418ba9e0
    Replacement::draft(0x1418bb0d0, "unknown_1418bb0d0", fn_1418bb0d0 as *const ()), // sub_1418bb0d0
    Replacement::draft(0x1418bb0f0, "unknown_1418bb0f0", fn_1418bb0f0 as *const ()), // sub_1418bb0f0
    Replacement::draft(0x1418bb340, "unknown_1418bb340", fn_1418bb340 as *const ()), // sub_1418bb340
    Replacement::draft_throws(0x1418bb540, "unknown_1418bb540", fn_1418bb540 as *const (), Throws::Discard), // sub_1418bb540
    Replacement::draft_throws(0x1418bb7e0, "unknown_1418bb7e0", fn_1418bb7e0 as *const (), Throws::Discard), // sub_1418bb7e0
    Replacement::draft(0x1418bb8b0, "unknown_1418bb8b0", fn_1418bb8b0 as *const ()), // sub_1418bb8b0
    Replacement::draft(0x1418bba80, "unknown_1418bba80", fn_1418bba80 as *const ()), // sub_1418bba80
    Replacement::draft(0x1418bbab0, "unknown_1418bbab0", fn_1418bbab0 as *const ()), // sub_1418bbab0
    Replacement::draft(0x1418bbba0, "unknown_1418bbba0", fn_1418bbba0 as *const ()), // sub_1418bbba0
    Replacement::draft_throws(0x1418bbe10, "unknown_1418bbe10", tpl_34978::<0x1426f8b60, 0x1410d6ca0, 0x1402695f0, 0x1438236d8, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_1418bbe10
    Replacement::draft(0x1418bc120, "unknown_1418bc120", tpl_34605::<0x1430a1868> as *const ()), // sub_1418bc120
    Replacement::draft(0x1418bc260, "unknown_1418bc260", tpl_34667::<0x20> as *const ()), // sub_1418bc260
    Replacement::draft_throws(0x1418bcb10, "unknown_1418bcb10", fn_1418bcb10 as *const (), Throws::Discard), // sub_1418bcb10
    Replacement::draft(0x1418bcdf0, "unknown_1418bcdf0", fn_1418bcdf0 as *const ()), // sub_1418bcdf0
    Replacement::draft(0x1418bd010, "unknown_1418bd010", fn_1418bd010 as *const ()), // sub_1418bd010
    Replacement::draft(0x1418bd250, "unknown_1418bd250", tpl_1777::<0x8, 0x14026a770> as *const ()), // sub_1418bd250
    Replacement::draft_throws(0x1418bd290, "unknown_1418bd290", fn_1418bd290 as *const (), Throws::Discard), // sub_1418bd290
    Replacement::draft(0x1418bd3f0, "unknown_1418bd3f0", fn_1418bd3f0 as *const ()), // sub_1418bd3f0
    Replacement::draft(0x1418bd4e0, "unknown_1418bd4e0", fn_1418bd4e0 as *const ()), // sub_1418bd4e0
    Replacement::draft(0x1418bd540, "unknown_1418bd540", fn_1418bd540 as *const ()), // sub_1418bd540
    Replacement::draft(0x1418bd590, "unknown_1418bd590", tpl_34835::<0x8, 0x140261110, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418bd590
    Replacement::draft(0x1418bd610, "unknown_1418bd610", tpl_34837::<0x140261110, 0x20, 0x14240aa00> as *const ()), // sub_1418bd610
    Replacement::draft_throws(0x1418bd660, "unknown_1418bd660", fn_1418bd660 as *const (), Throws::Caught), // sub_1418bd660
    Replacement::draft_throws(0x1418bd890, "unknown_1418bd890", tpl_94650::<0x14025cb90, 0x14309c488, 0x1426fab1c, 0x1426fab28, 0x1426fab24, 0x140254aa0, 0x1424b16a0, 0x14025f490> as *const (), Throws::Discard), // sub_1418bd890
    Replacement::draft_throws(0x1418bdad0, "unknown_1418bdad0", tpl_34894::<0x18, 0x1418bd890, 0x10> as *const (), Throws::Discard), // sub_1418bdad0
    Replacement::draft_throws(0x1418bdb10, "unknown_1418bdb10", tpl_34846::<0x1401f3a20, 0x1418bd890> as *const (), Throws::Discard), // sub_1418bdb10
    Replacement::draft_throws(0x1418bdb90, "unknown_1418bdb90", fn_1418bdb90 as *const (), Throws::Discard), // sub_1418bdb90
    Replacement::draft_throws(0x1418bde60, "unknown_1418bde60", fn_1418bde60 as *const (), Throws::Discard), // sub_1418bde60
    Replacement::draft_throws(0x1418bdfb0, "unknown_1418bdfb0", fn_1418bdfb0 as *const (), Throws::Caught), // sub_1418bdfb0
    Replacement::draft(0x1418be1b0, "unknown_1418be1b0", fn_1418be1b0 as *const ()), // sub_1418be1b0
    Replacement::draft(0x1418be240, "unknown_1418be240", fn_1418be240 as *const ()), // sub_1418be240
    Replacement::draft(0x1418be4f0, "unknown_1418be4f0", fn_1418be4f0 as *const ()), // sub_1418be4f0
    Replacement::draft(0x1418be680, "unknown_1418be680", fn_1418be680 as *const ()), // sub_1418be680
    Replacement::draft_throws(0x1418be980, "unknown_1418be980", fn_1418be980 as *const (), Throws::Caught), // sub_1418be980
    Replacement::draft(0x1418bec40, "unknown_1418bec40", tpl_94770::<0x1430a4238, 0x8, 0xc> as *const ()), // sub_1418bec40
    Replacement::draft(0x1418bee50, "unknown_1418bee50", tpl_34667::<0x20> as *const ()), // sub_1418bee50
    Replacement::draft(0x1418beef0, "unknown_1418beef0", tpl_94771::<0x1402734c0> as *const ()), // sub_1418beef0
    Replacement::draft(0x1418befb0, "unknown_1418befb0", tpl_34684::<0x1401f2bc0> as *const ()), // sub_1418befb0
    Replacement::draft(0x1418bf060, "unknown_1418bf060", fn_1418bf060 as *const ()), // sub_1418bf060
    Replacement::draft(0x1418bf130, "unknown_1418bf130", fn_1418bf130 as *const ()), // sub_1418bf130
    Replacement::draft(0x1418bf180, "unknown_1418bf180", fn_1418bf180 as *const ()), // sub_1418bf180
    Replacement::draft(0x1418bf250, "unknown_1418bf250", fn_1418bf250 as *const ()), // sub_1418bf250
    Replacement::draft(0x1418bf380, "unknown_1418bf380", tpl_34992::<0x1402733f0> as *const ()), // sub_1418bf380
    Replacement::draft(0x1418bf3a0, "unknown_1418bf3a0", tpl_34993 as *const ()), // sub_1418bf3a0
    Replacement::draft(0x1418bf3e0, "unknown_1418bf3e0", tpl_34994::<0x143c914d4, 0x14387a008, 0x14387a000, 0x141834a78, 0x1425dc290, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418bf3e0
    Replacement::draft(0x1418bf450, "unknown_1418bf450", tpl_34995 as *const ()), // sub_1418bf450
    Replacement::draft(0x1418bf470, "std_1418bf470", tpl_550::<0x1430a4390, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const coregame::Phys
    Replacement::draft(0x1418bf490, "std_1418bf490", tpl_550::<0x1430a4390, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const coregame::Phys
    Replacement::draft(0x1418bf4b0, "std_1418bf4b0", tpl_34996::<0x8, 0x40, 0x48, 0x10> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const coregame::Phys
    Replacement::draft(0x1418bf510, "std_1418bf510", tpl_1::<0x143884b70> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const coregame::Phys
    Replacement::draft(0x1418bf520, "std_1418bf520", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const coregame::Phys
    Replacement::draft(0x1418bf530, "std_1418bf530", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_1>,bool,const coregame::Phys
    Replacement::draft(0x1418bf550, "std_1418bf550", tpl_550::<0x1430a4520, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_2>,bool,const coregame::Phys
    Replacement::draft(0x1418bf570, "std_1418bf570", tpl_550::<0x1430a4520, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_2>,bool,const coregame::Phys
    Replacement::draft(0x1418bf590, "std_1418bf590", tpl_34996::<0x8, 0x40, 0x48, 0x10> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_2>,bool,const coregame::Phys
    Replacement::draft(0x1418bf5f0, "std_1418bf5f0", tpl_1::<0x143885180> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_2>,bool,const coregame::Phys
    Replacement::draft(0x1418bf600, "std_1418bf600", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_2>,bool,const coregame::Phys
    Replacement::draft(0x1418bf610, "std_1418bf610", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_2>,bool,const coregame::Phys
    Replacement::draft(0x1418bf630, "std_1418bf630", tpl_20903::<0x1430a4638, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_3>,bool,const coregame::Phys
    Replacement::draft(0x1418bf660, "std_1418bf660", tpl_20903::<0x1430a4638, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_3>,bool,const coregame::Phys
    Replacement::draft(0x1418bf6a0, "std_1418bf6a0", tpl_1::<0x143885790> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_3>,bool,const coregame::Phys
    Replacement::draft(0x1418bf6b0, "std_1418bf6b0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_3>,bool,const coregame::Phys
    Replacement::draft(0x1418bf6c0, "std_1418bf6c0", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_3>,bool,const coregame::Phys
    Replacement::draft(0x1418bf6e0, "std_1418bf6e0", tpl_550::<0x1430a45a0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_4>,bool,const coregame::Phys
    Replacement::draft(0x1418bf700, "std_1418bf700", tpl_550::<0x1430a45a0, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_4>,bool,const coregame::Phys
    Replacement::draft(0x1418bf720, "std_1418bf720", fn_1418bf720 as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_4>,bool,const coregame::Phys
    Replacement::draft(0x1418bf7e0, "std_1418bf7e0", tpl_1::<0x143884560> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_4>,bool,const coregame::Phys
    Replacement::draft(0x1418bf7f0, "std_1418bf7f0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_4>,bool,const coregame::Phys
    Replacement::draft(0x1418bf800, "std_1418bf800", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_4>,bool,const coregame::Phys
    Replacement::draft(0x1418bf820, "std_1418bf820", tpl_20903::<0x1430a4558, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_5>,bool,const coregame::Phys
    Replacement::draft(0x1418bf850, "std_1418bf850", tpl_20903::<0x1430a4558, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_5>,bool,const coregame::Phys
    Replacement::draft(0x1418bf890, "std_1418bf890", tpl_1::<0x143883f50> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_5>,bool,const coregame::Phys
    Replacement::draft(0x1418bf8a0, "std_1418bf8a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_5>,bool,const coregame::Phys
    Replacement::draft(0x1418bf8b0, "std_1418bf8b0", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::targetselector::hasFiringLOSToPositionWithResult::<lambda_5>,bool,const coregame::Phys
    Replacement::draft(0x1418bf8d0, "unknown_1418bf8d0", tpl_34318 as *const ()), // sub_1418bf8d0
    Replacement::draft(0x1418bf8f0, "unknown_1418bf8f0", tpl_34318 as *const ()), // sub_1418bf8f0
    Replacement::draft(0x1418bf910, "unknown_1418bf910", tpl_34318 as *const ()), // sub_1418bf910
    Replacement::draft(0x1418bf930, "unknown_1418bf930", tpl_34318 as *const ()), // sub_1418bf930
    Replacement::draft(0x1418bf950, "unknown_1418bf950", tpl_34318 as *const ()), // sub_1418bf950
    Replacement::draft(0x1418bf970, "unknown_1418bf970", tpl_34318 as *const ()), // sub_1418bf970
    Replacement::draft(0x1418bf990, "unknown_1418bf990", tpl_34318 as *const ()), // sub_1418bf990
    Replacement::draft(0x1418bf9b0, "unknown_1418bf9b0", tpl_34318 as *const ()), // sub_1418bf9b0
    Replacement::draft(0x1418bf9d0, "unknown_1418bf9d0", tpl_34318 as *const ()), // sub_1418bf9d0
    Replacement::draft(0x1418bf9f0, "unknown_1418bf9f0", tpl_34318 as *const ()), // sub_1418bf9f0
    Replacement::draft(0x1418bfa10, "unknown_1418bfa10", tpl_34318 as *const ()), // sub_1418bfa10
    Replacement::draft(0x1418bfa30, "unknown_1418bfa30", tpl_34318 as *const ()), // sub_1418bfa30
    Replacement::draft(0x1418bfa50, "unknown_1418bfa50", tpl_34318 as *const ()), // sub_1418bfa50
    Replacement::draft(0x1418bfa70, "unknown_1418bfa70", tpl_34319::<0x143c914d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfa70
    Replacement::draft(0x1418bfad0, "unknown_1418bfad0", tpl_34319::<0x143c8e654, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfad0
    Replacement::draft(0x1418bfb30, "unknown_1418bfb30", tpl_34319::<0x143c914b8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfb30
    Replacement::draft(0x1418bfb90, "unknown_1418bfb90", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfb90
    Replacement::draft(0x1418bfbf0, "unknown_1418bfbf0", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfbf0
    Replacement::draft(0x1418bfc50, "unknown_1418bfc50", tpl_34319::<0x143c914ac, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfc50
    Replacement::draft(0x1418bfcb0, "unknown_1418bfcb0", tpl_34319::<0x143c8fab8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfcb0
    Replacement::draft(0x1418bfd10, "unknown_1418bfd10", tpl_34319::<0x143c8e70c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfd10
    Replacement::draft(0x1418bfd70, "unknown_1418bfd70", tpl_34319::<0x143c914bc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfd70
    Replacement::draft(0x1418bfdd0, "unknown_1418bfdd0", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfdd0
    Replacement::draft(0x1418bfe30, "unknown_1418bfe30", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfe30
    Replacement::draft(0x1418bfe90, "unknown_1418bfe90", tpl_34319::<0x143c914dc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfe90
    Replacement::draft(0x1418bfef0, "unknown_1418bfef0", tpl_34319::<0x143c914b0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418bfef0
    Replacement::draft(0x1418bff50, "unknown_1418bff50", tpl_361 as *const ()), // sub_1418bff50
    Replacement::draft(0x1418bff60, "unknown_1418bff60", tpl_361 as *const ()), // sub_1418bff60
    Replacement::draft(0x1418bff70, "unknown_1418bff70", fn_1418bff70 as *const ()), // sub_1418bff70
    Replacement::draft(0x1418c11c0, "unknown_1418c11c0", tpl_34998::<0x8, 0x10> as *const ()), // sub_1418c11c0
    Replacement::draft(0x1418c11e0, "unknown_1418c11e0", fn_1418c11e0 as *const ()), // sub_1418c11e0
    Replacement::draft(0x1418c1420, "unknown_1418c1420", tpl_34443 as *const ()), // sub_1418c1420
    Replacement::draft(0x1418c1430, "unknown_1418c1430", fn_1418c1430 as *const ()), // sub_1418c1430
    Replacement::draft(0x1418c14f0, "unknown_1418c14f0", tpl_34998::<0x8, 0x10> as *const ()), // sub_1418c14f0
    Replacement::draft(0x1418c1510, "unknown_1418c1510", fn_1418c1510 as *const ()), // sub_1418c1510
    Replacement::draft(0x1418c16c0, "unknown_1418c16c0", tpl_34443 as *const ()), // sub_1418c16c0
    Replacement::draft(0x1418c16d0, "unknown_1418c16d0", tpl_35001::<0x8, 0x40, 0x48, 0x10> as *const ()), // sub_1418c16d0
    Replacement::draft(0x1418c1740, "unknown_1418c1740", tpl_34443 as *const ()), // sub_1418c1740
    Replacement::draft(0x1418c1750, "unknown_1418c1750", tpl_35001::<0x8, 0x40, 0x48, 0x10> as *const ()), // sub_1418c1750
    Replacement::draft(0x1418c17c0, "unknown_1418c17c0", fn_1418c17c0 as *const ()), // sub_1418c17c0
    Replacement::draft(0x1418c2190, "unknown_1418c2190", fn_1418c2190 as *const ()), // sub_1418c2190
    Replacement::draft(0x1418c23f0, "unknown_1418c23f0", fn_1418c23f0 as *const ()), // sub_1418c23f0
    Replacement::draft(0x1418c2540, "unknown_1418c2540", fn_1418c2540 as *const ()), // sub_1418c2540
    Replacement::draft(0x1418c2580, "unknown_1418c2580", fn_1418c2580 as *const ()), // sub_1418c2580
    Replacement::draft(0x1418c25b0, "unknown_1418c25b0", fn_1418c25b0 as *const ()), // sub_1418c25b0
    Replacement::draft(0x1418c2610, "unknown_1418c2610", tpl_361 as *const ()), // sub_1418c2610
    Replacement::draft(0x1418c2620, "unknown_1418c2620", fn_1418c2620 as *const ()), // sub_1418c2620
    Replacement::draft_throws(0x1418c2720, "unknown_1418c2720", tpl_35009::<0x14387eac8, 0x140272680, 0x1402116e0> as *const (), Throws::Discard), // sub_1418c2720
    Replacement::draft(0x1418c2760, "unknown_1418c2760", tpl_34994::<0x143c91780, 0x14387a058, 0x14387a050, 0x141834a78, 0x1425dce70, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418c2760
    Replacement::draft(0x1418c27d0, "unknown_1418c27d0", fn_1418c27d0 as *const ()), // sub_1418c27d0
    Replacement::draft(0x1418c2890, "unknown_1418c2890", fn_1418c2890 as *const ()), // sub_1418c2890
    Replacement::draft(0x1418c28d0, "unknown_1418c28d0", fn_1418c28d0 as *const ()), // sub_1418c28d0
    Replacement::draft(0x1418c2940, "unknown_1418c2940", tpl_34994::<0x143c91648, 0x14387a068, 0x14387a060, 0x141834a78, 0x1425dce80, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418c2940
    Replacement::draft(0x1418c29b0, "unknown_1418c29b0", tpl_94790 as *const ()), // sub_1418c29b0
    Replacement::draft(0x1418c2a40, "unknown_1418c2a40", tpl_35012 as *const ()), // sub_1418c2a40
    Replacement::draft(0x1418c2a60, "unknown_1418c2a60", tpl_35013::<0x142442de0> as *const ()), // sub_1418c2a60
    Replacement::draft(0x1418c2ad0, "unknown_1418c2ad0", tpl_34994::<0x143c9173c, 0x14387a038, 0x14387a030, 0x141834a78, 0x1425dce90, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418c2ad0
    Replacement::draft(0x1418c2b40, "unknown_1418c2b40", fn_1418c2b40 as *const ()), // sub_1418c2b40
    Replacement::draft(0x1418c2d00, "unknown_1418c2d00", fn_1418c2d00 as *const ()), // sub_1418c2d00
    Replacement::draft(0x1418c2d20, "unknown_1418c2d20", fn_1418c2d20 as *const ()), // sub_1418c2d20
    Replacement::draft(0x1418c2d80, "unknown_1418c2d80", tpl_34994::<0x143c916d0, 0x14387a048, 0x14387a040, 0x141834a78, 0x1425dce60, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418c2d80
    Replacement::draft(0x1418c2df0, "unknown_1418c2df0", tpl_94428::<0x1430a7230, 0x8> as *const ()), // sub_1418c2df0
    Replacement::draft(0x1418c2eb0, "unknown_1418c2eb0", tpl_35016 as *const ()), // sub_1418c2eb0
    Replacement::draft(0x1418c2ef0, "unknown_1418c2ef0", tpl_34318 as *const ()), // sub_1418c2ef0
    Replacement::draft(0x1418c2f10, "unknown_1418c2f10", tpl_34318 as *const ()), // sub_1418c2f10
    Replacement::draft(0x1418c2f30, "unknown_1418c2f30", tpl_34319::<0x143c8cf5c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c2f30
    Replacement::draft(0x1418c2f90, "unknown_1418c2f90", tpl_34319::<0x143c91608, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c2f90
    Replacement::draft(0x1418c2ff0, "std_1418c2ff0", tpl_23342::<0x1430a71f8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::tailcamera::update::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl
    Replacement::draft(0x1418c3010, "std_1418c3010", tpl_23342::<0x1430a71f8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::tailcamera::update::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl
    Replacement::draft(0x1418c3040, "std_1418c3040", tpl_1::<0x143885da0> as *const ()), // std::_Func_impl_no_alloc<bigfish::tailcamera::update::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl
    Replacement::draft(0x1418c3050, "std_1418c3050", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::tailcamera::update::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl
    Replacement::draft(0x1418c3060, "std_1418c3060", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::tailcamera::update::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl
    Replacement::draft(0x1418c3080, "unknown_1418c3080", tpl_34318 as *const ()), // sub_1418c3080
    Replacement::draft(0x1418c30a0, "unknown_1418c30a0", tpl_34318 as *const ()), // sub_1418c30a0
    Replacement::draft(0x1418c30c0, "unknown_1418c30c0", tpl_34318 as *const ()), // sub_1418c30c0
    Replacement::draft(0x1418c30e0, "unknown_1418c30e0", tpl_34318 as *const ()), // sub_1418c30e0
    Replacement::draft(0x1418c3100, "unknown_1418c3100", tpl_34318 as *const ()), // sub_1418c3100
    Replacement::draft(0x1418c3120, "unknown_1418c3120", tpl_34318 as *const ()), // sub_1418c3120
    Replacement::draft(0x1418c3140, "unknown_1418c3140", tpl_34318 as *const ()), // sub_1418c3140
    Replacement::draft(0x1418c3160, "unknown_1418c3160", tpl_34318 as *const ()), // sub_1418c3160
    Replacement::draft(0x1418c3180, "unknown_1418c3180", tpl_34318 as *const ()), // sub_1418c3180
    Replacement::draft(0x1418c31a0, "unknown_1418c31a0", tpl_34319::<0x143c8efb0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c31a0
    Replacement::draft(0x1418c3200, "unknown_1418c3200", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c3200
    Replacement::draft(0x1418c3260, "unknown_1418c3260", tpl_34319::<0x143c917cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c3260
    Replacement::draft(0x1418c32c0, "unknown_1418c32c0", tpl_34319::<0x143c8e840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c32c0
    Replacement::draft(0x1418c3320, "unknown_1418c3320", tpl_34319::<0x143c8e68c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c3320
    Replacement::draft(0x1418c3380, "unknown_1418c3380", tpl_34319::<0x143c8d074, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c3380
    Replacement::draft(0x1418c33e0, "unknown_1418c33e0", tpl_34319::<0x143c90198, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c33e0
    Replacement::draft(0x1418c3440, "unknown_1418c3440", tpl_34319::<0x143c915fc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c3440
    Replacement::draft(0x1418c34a0, "unknown_1418c34a0", tpl_34319::<0x143c8d508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c34a0
    Replacement::draft(0x1418c3500, "unknown_1418c3500", tpl_34318 as *const ()), // sub_1418c3500
    Replacement::draft(0x1418c3520, "unknown_1418c3520", tpl_34319::<0x143c8cefc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418c3520
    Replacement::draft(0x1418c3580, "unknown_1418c3580", fn_1418c3580 as *const ()), // sub_1418c3580
    Replacement::draft_throws(0x1418c37d0, "unknown_1418c37d0", fn_1418c37d0 as *const (), Throws::Discard), // sub_1418c37d0
    Replacement::draft(0x1418c3840, "unknown_1418c3840", tpl_22031::<0x8> as *const ()), // sub_1418c3840
    Replacement::draft(0x1418c3860, "unknown_1418c3860", fn_1418c3860 as *const ()), // sub_1418c3860
    Replacement::draft(0x1418c3b10, "unknown_1418c3b10", tpl_496 as *const ()), // sub_1418c3b10
    Replacement::draft(0x1418c3b20, "unknown_1418c3b20", tpl_496 as *const ()), // sub_1418c3b20
    Replacement::draft(0x1418c3b30, "unknown_1418c3b30", tpl_361 as *const ()), // sub_1418c3b30
    Replacement::draft(0x1418c3b40, "unknown_1418c3b40", tpl_361 as *const ()), // sub_1418c3b40
    Replacement::draft_throws(0x1418c3b50, "unknown_1418c3b50", fn_1418c3b50 as *const (), Throws::Caught), // sub_1418c3b50
    Replacement::draft(0x1418c5210, "unknown_1418c5210", tpl_361 as *const ()), // sub_1418c5210
    Replacement::draft(0x1418c5220, "unknown_1418c5220", fn_1418c5220 as *const ()), // sub_1418c5220
    Replacement::draft(0x1418c5390, "unknown_1418c5390", fn_1418c5390 as *const ()), // sub_1418c5390
    Replacement::draft(0x1418c5b00, "unknown_1418c5b00", fn_1418c5b00 as *const ()), // sub_1418c5b00
    Replacement::draft(0x1418c5cd0, "unknown_1418c5cd0", fn_1418c5cd0 as *const ()), // sub_1418c5cd0
    Replacement::draft(0x1418c5e20, "unknown_1418c5e20", fn_1418c5e20 as *const ()), // sub_1418c5e20
    Replacement::draft(0x1418c6200, "unknown_1418c6200", fn_1418c6200 as *const ()), // sub_1418c6200
    Replacement::draft_throws(0x1418c6510, "unknown_1418c6510", fn_1418c6510 as *const (), Throws::Discard), // sub_1418c6510
    Replacement::draft(0x1418c69f0, "unknown_1418c69f0", tpl_34994::<0x143c919a4, 0x14387a088, 0x14387a080, 0x141834a78, 0x1425dd010, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418c69f0
    Replacement::draft(0x1418c6a60, "unknown_1418c6a60", tpl_34667::<0x20> as *const ()), // sub_1418c6a60
    Replacement::draft(0x1418c6a90, "unknown_1418c6a90", tpl_34994::<0x143c91a34, 0x14387a078, 0x14387a070, 0x141834a78, 0x1425dd000, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418c6a90
    Replacement::draft(0x1418c6b00, "unknown_1418c6b00", tpl_34605::<0x1430a8050> as *const ()), // sub_1418c6b00
    Replacement::draft(0x1418c7000, "unknown_1418c7000", fn_1418c7000 as *const ()), // sub_1418c7000
    Replacement::draft_throws(0x1418c70a0, "unknown_1418c70a0", fn_1418c70a0 as *const (), Throws::Caught), // sub_1418c70a0
    Replacement::draft(0x1418c73a0, "unknown_1418c73a0", fn_1418c73a0 as *const ()), // sub_1418c73a0
    Replacement::draft_throws(0x1418c7780, "unknown_1418c7780", fn_1418c7780 as *const (), Throws::Discard), // sub_1418c7780
    Replacement::draft(0x1418c7be0, "unknown_1418c7be0", tpl_34611::<0x8> as *const ()), // sub_1418c7be0
    Replacement::draft(0x1418c7c70, "unknown_1418c7c70", fn_1418c7c70 as *const ()), // sub_1418c7c70
    Replacement::draft(0x1418c7e70, "unknown_1418c7e70", fn_1418c7e70 as *const ()), // sub_1418c7e70
    Replacement::draft(0x1418c7fe0, "unknown_1418c7fe0", tpl_94810::<0x1430a81a8, 0x8> as *const ()), // sub_1418c7fe0
    Replacement::draft(0x1418c8360, "unknown_1418c8360", tpl_1887::<0x8, 0x140279780, 0x140279980> as *const ()), // sub_1418c8360
    Replacement::draft(0x1418c83d0, "unknown_1418c83d0", fn_1418c83d0 as *const ()), // sub_1418c83d0
    Replacement::draft_throws(0x1418c84a0, "unknown_1418c84a0", fn_1418c84a0 as *const (), Throws::Discard), // sub_1418c84a0
    Replacement::draft(0x1418c86d0, "unknown_1418c86d0", tpl_94770::<0x1430a9e60, 0x8, 0xc> as *const ()), // sub_1418c86d0
    Replacement::draft(0x1418c88e0, "unknown_1418c88e0", fn_1418c88e0 as *const ()), // sub_1418c88e0
    Replacement::draft(0x1418c89f0, "unknown_1418c89f0", fn_1418c89f0 as *const ()), // sub_1418c89f0
    Replacement::draft(0x1418c8df0, "unknown_1418c8df0", tpl_35031::<0x142407c80, 0x143c91cb8, 0x14020e8f0> as *const ()), // sub_1418c8df0
    Replacement::draft(0x1418c8f20, "unknown_1418c8f20", tpl_35032::<0x580f0, 0x142407c80, 0x10, 0x8, 0x14027b240> as *const ()), // sub_1418c8f20
    Replacement::draft_throws(0x1418c8fc0, "unknown_1418c8fc0", fn_1418c8fc0 as *const (), Throws::Discard), // sub_1418c8fc0
    Replacement::draft_throws(0x1418c9260, "unknown_1418c9260", fn_1418c9260 as *const (), Throws::Discard), // sub_1418c9260
    Replacement::draft(0x1418c9730, "bigfish_1418c9730", fn_1418c9730 as *const ()), // bigfish::StartInteractableObjectSearchInstance::bigfish::StartInteractableObjectSearchInstance::vfunc_9
    Replacement::draft(0x1418c97c0, "bigfish_1418c97c0", fn_1418c97c0 as *const ()), // bigfish::StartInteractableObjectSearchInstance::bigfish::StartInteractableObjectSearchInstance::vfunc_4
    Replacement::draft(0x1418c9df0, "bigfish_1418c9df0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bigfish::StartInteractableObjectSearchInstance::ClassTypeInfoImplementation::bigfish::StartInteractableObjectSearchInsta
    Replacement::draft(0x1418ca040, "unknown_1418ca040", tpl_34671::<0x20> as *const ()), // sub_1418ca040
    Replacement::draft(0x1418ca0e0, "unknown_1418ca0e0", tpl_34395::<0x10, 0x18> as *const ()), // sub_1418ca0e0
    Replacement::draft(0x1418ca180, "unknown_1418ca180", tpl_35038::<0x4> as *const ()), // sub_1418ca180
    Replacement::draft(0x1418ca210, "unknown_1418ca210", tpl_94819::<0x1430ab250, 0x8, 0xc, 0x10> as *const ()), // sub_1418ca210
    Replacement::draft(0x1418ca2c0, "unknown_1418ca2c0", tpl_35039::<0x1430ab250, 0x8, 0xc, 0x10> as *const ()), // sub_1418ca2c0
    Replacement::draft(0x1418ca640, "unknown_1418ca640", fn_1418ca640 as *const ()), // sub_1418ca640
    Replacement::draft(0x1418ca910, "unknown_1418ca910", fn_1418ca910 as *const ()), // sub_1418ca910
    Replacement::draft(0x1418caa30, "unknown_1418caa30", tpl_1913::<0x8, 0x14027e6e0, 0x14027e8e0> as *const ()), // sub_1418caa30
    Replacement::draft_throws(0x1418caab0, "unknown_1418caab0", fn_1418caab0 as *const (), Throws::Discard), // sub_1418caab0
    Replacement::draft(0x1418caf70, "unknown_1418caf70", tpl_35043::<0x7c, 0x140282580> as *const ()), // sub_1418caf70
    Replacement::draft(0x1418cafc0, "unknown_1418cafc0", tpl_35044::<0x1402822a0> as *const ()), // sub_1418cafc0
    Replacement::draft(0x1418caff0, "unknown_1418caff0", tpl_35044::<0x140282130> as *const ()), // sub_1418caff0
    Replacement::draft(0x1418cb020, "unknown_1418cb020", fn_1418cb020 as *const ()), // sub_1418cb020
    Replacement::draft(0x1418cb090, "unknown_1418cb090", tpl_35044::<0x140281cc0> as *const ()), // sub_1418cb090
    Replacement::draft(0x1418cb0c0, "unknown_1418cb0c0", tpl_35044::<0x140281b50> as *const ()), // sub_1418cb0c0
    Replacement::draft(0x1418cb0f0, "unknown_1418cb0f0", tpl_35044::<0x1402819e0> as *const ()), // sub_1418cb0f0
    Replacement::draft(0x1418cb180, "unknown_1418cb180", fn_1418cb180 as *const ()), // sub_1418cb180
    Replacement::draft(0x1418cb220, "unknown_1418cb220", tpl_35043::<0x6c, 0x140281240> as *const ()), // sub_1418cb220
    Replacement::draft(0x1418cb240, "unknown_1418cb240", tpl_35043::<0x74, 0x1402810d0> as *const ()), // sub_1418cb240
    Replacement::draft(0x1418cb260, "unknown_1418cb260", tpl_35043::<0x70, 0x140280f60> as *const ()), // sub_1418cb260
    Replacement::draft(0x1418cb280, "unknown_1418cb280", tpl_35043::<0x68, 0x140280df0> as *const ()), // sub_1418cb280
    Replacement::draft(0x1418cb2a0, "unknown_1418cb2a0", tpl_35044::<0x140280b00> as *const ()), // sub_1418cb2a0
    Replacement::draft(0x1418cb2d0, "unknown_1418cb2d0", tpl_35044::<0x140280990> as *const ()), // sub_1418cb2d0
    Replacement::draft(0x1418cb300, "unknown_1418cb300", tpl_35047::<0x64, 0x140280820> as *const ()), // sub_1418cb300
    Replacement::draft(0x1418cb350, "unknown_1418cb350", tpl_35048::<0x58, 0x8, 0x60, 0x1402806a0> as *const ()), // sub_1418cb350
    Replacement::draft(0x1418cb370, "unknown_1418cb370", tpl_35049::<0x20, 0x8, 0x28, 0xc, 0x2c, 0x14, 0x34, 0x18, 0x38, 0x40, 0x24, 0x44, 0x1402804f0> as *const ()), // sub_1418cb370
    Replacement::draft(0x1418cb3c0, "unknown_1418cb3c0", fn_1418cb3c0 as *const ()), // sub_1418cb3c0
    Replacement::draft_throws(0x1418cb430, "unknown_1418cb430", fn_1418cb430 as *const (), Throws::Discard), // sub_1418cb430
    Replacement::draft(0x1418cb6e0, "unknown_1418cb6e0", fn_1418cb6e0 as *const ()), // sub_1418cb6e0
    Replacement::draft_throws(0x1418cb760, "unknown_1418cb760", fn_1418cb760 as *const (), Throws::Discard), // sub_1418cb760
    Replacement::draft(0x1418cba20, "unknown_1418cba20", fn_1418cba20 as *const ()), // sub_1418cba20
    Replacement::draft(0x1418cbb00, "unknown_1418cbb00", fn_1418cbb00 as *const ()), // sub_1418cbb00
    Replacement::draft(0x1418cbb40, "unknown_1418cbb40", tpl_34607::<0x1401f2bc0> as *const ()), // sub_1418cbb40
    Replacement::draft(0x1418cbbf0, "unknown_1418cbbf0", fn_1418cbbf0 as *const ()), // sub_1418cbbf0
    Replacement::draft(0x1418cbe00, "unknown_1418cbe00", tpl_35056::<0x1402844d0> as *const ()), // sub_1418cbe00
    Replacement::draft(0x1418cc160, "unknown_1418cc160", fn_1418cc160 as *const ()), // sub_1418cc160
    Replacement::draft(0x1418cc280, "unknown_1418cc280", fn_1418cc280 as *const ()), // sub_1418cc280
    Replacement::draft_throws(0x1418ccab0, "unknown_1418ccab0", fn_1418ccab0 as *const (), Throws::Discard), // sub_1418ccab0
    Replacement::draft(0x1418ccbf0, "unknown_1418ccbf0", tpl_94835::<0x1430acca0, 0x8, 0x10, 0x14, 0x18> as *const ()), // sub_1418ccbf0
    Replacement::draft(0x1418cccd0, "unknown_1418cccd0", tpl_35059::<0x1430acca0, 0x8, 0x10, 0x14, 0x18> as *const ()), // sub_1418cccd0
    Replacement::draft(0x1418ccd20, "unknown_1418ccd20", tpl_35060 as *const ()), // sub_1418ccd20
    Replacement::draft(0x1418ccd60, "unknown_1418ccd60", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418ccd60
    Replacement::draft(0x1418ccde0, "unknown_1418ccde0", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_1418ccde0
    Replacement::draft_throws(0x1418cce30, "unknown_1418cce30", fn_1418cce30 as *const (), Throws::Caught), // sub_1418cce30
    Replacement::draft(0x1418cd030, "unknown_1418cd030", tpl_35062::<0x14240ab10, 0x10, 0x1401f2bc0> as *const ()), // sub_1418cd030
    Replacement::draft(0x1418cd060, "unknown_1418cd060", tpl_361 as *const ()), // sub_1418cd060
    Replacement::draft(0x1418cd070, "unknown_1418cd070", tpl_34318 as *const ()), // sub_1418cd070
    Replacement::draft(0x1418cd090, "unknown_1418cd090", tpl_35063::<0x143c92484, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_1418cd090
    Replacement::draft(0x1418cd0e0, "unknown_1418cd0e0", tpl_34319::<0x143c8f6e4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418cd0e0
    Replacement::draft(0x1418cd140, "std_1418cd140", tpl_770::<0x1430acdf8> as *const ()), // std::_Func_impl_no_alloc<detail::<lambda_83>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl_no_alloc<detail::<l
    Replacement::draft(0x1418cd160, "std_1418cd160", tpl_770::<0x1430acdf8> as *const ()), // std::_Func_impl_no_alloc<detail::<lambda_83>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl_no_alloc<detail::<l
    Replacement::draft(0x1418cd180, "std_1418cd180", tpl_35064::<0x60, 0x14397fe70, 0xac> as *const ()), // std::_Func_impl_no_alloc<detail::<lambda_83>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl_no_alloc<detail::<l
    Replacement::draft(0x1418cd1b0, "std_1418cd1b0", tpl_1::<0x143886660> as *const ()), // std::_Func_impl_no_alloc<detail::<lambda_83>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl_no_alloc<detail::<l
    Replacement::draft(0x1418cd1c0, "std_1418cd1c0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<detail::<lambda_83>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl_no_alloc<detail::<l
    Replacement::draft(0x1418cd1d0, "std_1418cd1d0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<detail::<lambda_83>,bool,const coregame::PhysicsSweepHit&>::std::_Func_impl_no_alloc<detail::<l
    Replacement::draft(0x1418cd1f0, "unknown_1418cd1f0", tpl_34318 as *const ()), // sub_1418cd1f0
    Replacement::draft(0x1418cd210, "unknown_1418cd210", tpl_34319::<0x143c922c0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418cd210
    Replacement::draft(0x1418cd270, "unknown_1418cd270", tpl_34318 as *const ()), // sub_1418cd270
    Replacement::draft(0x1418cd290, "unknown_1418cd290", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_1418cd290
    Replacement::draft_throws(0x1418cd2f0, "unknown_1418cd2f0", tpl_94836::<0x1402845f0, 0x1427011f0, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_1418cd2f0
    Replacement::draft_throws(0x1418cd420, "unknown_1418cd420", tpl_35065::<0x18, 0x1418cd2f0, 0x10> as *const (), Throws::Discard), // sub_1418cd420
    Replacement::draft_throws(0x1418cd460, "unknown_1418cd460", tpl_34846::<0x1401f3a20, 0x1418cd2f0> as *const (), Throws::Discard), // sub_1418cd460
    Replacement::draft_throws(0x1418cd4e0, "unknown_1418cd4e0", fn_1418cd4e0 as *const (), Throws::Discard), // sub_1418cd4e0
    Replacement::draft(0x1418cd6a0, "unknown_1418cd6a0", tpl_361 as *const ()), // sub_1418cd6a0
    Replacement::draft(0x1418cd6b0, "unknown_1418cd6b0", tpl_361 as *const ()), // sub_1418cd6b0
    Replacement::draft(0x1418cd6c0, "unknown_1418cd6c0", fn_1418cd6c0 as *const ()), // sub_1418cd6c0
    Replacement::draft(0x1418cdd00, "unknown_1418cdd00", fn_1418cdd00 as *const ()), // sub_1418cdd00
    Replacement::draft(0x1418cde20, "unknown_1418cde20", tpl_361 as *const ()), // sub_1418cde20
    Replacement::draft(0x1418cde30, "unknown_1418cde30", tpl_1962::<0x8, 0x1402850f0, 0x1402852f0> as *const ()), // sub_1418cde30
    Replacement::draft(0x1418cdea0, "unknown_1418cdea0", fn_1418cdea0 as *const ()), // sub_1418cdea0
    Replacement::draft(0x1418ce040, "unknown_1418ce040", fn_1418ce040 as *const ()), // sub_1418ce040
    Replacement::draft(0x1418ce070, "unknown_1418ce070", tpl_35064::<0x60, 0x14397fe70, 0xac> as *const ()), // sub_1418ce070
    Replacement::draft_throws(0x1418ce0a0, "unknown_1418ce0a0", fn_1418ce0a0 as *const (), Throws::Caught), // sub_1418ce0a0
    Replacement::draft(0x1418ce250, "bigfish_1418ce250", fn_1418ce250 as *const ()), // bigfish::SetCombatStateDecoratorInstance::bigfish::SetCombatStateDecoratorInstance::vfunc_9
    Replacement::draft(0x1418ce320, "bigfish_1418ce320", fn_1418ce320 as *const ()), // bigfish::SetCombatStateDecoratorInstance::bigfish::SetCombatStateDecoratorInstance::vfunc_4
    Replacement::draft(0x1418ce9a0, "bigfish_1418ce9a0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bigfish::SetCombatStateDecoratorInstance::ClassTypeInfoImplementation::bigfish::SetCombatStateDecoratorInstance::ClassTy
    Replacement::draft(0x1418ceb80, "bigfish_1418ceb80", fn_1418ceb80 as *const ()), // bigfish::SetCanEnterStealthAreasInstance::bigfish::SetCanEnterStealthAreasInstance::vfunc_9
    Replacement::draft(0x1418cebe0, "bigfish_1418cebe0", fn_1418cebe0 as *const ()), // bigfish::SetCanEnterStealthAreasInstance::bigfish::SetCanEnterStealthAreasInstance::vfunc_4
    Replacement::draft_throws(0x1418ced00, "bigfish_1418ced00", tpl_838::<0x140287460> as *const (), Throws::Discard), // bigfish::SetCanEnterStealthAreasInstance::bigfish::SetCanEnterStealthAreasInstance::vfunc_21
    Replacement::draft_throws(0x1418ced10, "bigfish_1418ced10", tpl_838::<0x140287460> as *const (), Throws::Discard), // bigfish::SetCanEnterStealthAreasInstance::bigfish::SetCanEnterStealthAreasInstance::vfunc_22
    Replacement::draft_throws(0x1418ced20, "bigfish_1418ced20", tpl_838::<0x140287460> as *const (), Throws::Discard), // bigfish::SetCanEnterStealthAreasInstance::bigfish::SetCanEnterStealthAreasInstance::vfunc_23
    Replacement::draft(0x1418cf1e0, "bigfish_1418cf1e0", fn_1418cf1e0 as *const ()), // bigfish::SetCanEnterStealthAreasInstance::ClassTypeInfoImplementation::bigfish::SetCanEnterStealthAreasInstance::ClassTy
    Replacement::draft(0x1418cf250, "bigfish_1418cf250", fn_1418cf250 as *const ()), // bigfish::SetCanEnterStealthAreasInstance::ClassTypeInfoImplementation::bigfish::SetCanEnterStealthAreasInstance::ClassTy
    Replacement::draft(0x1418cf780, "unknown_1418cf780", fn_1418cf780 as *const ()), // sub_1418cf780
    Replacement::draft(0x1418cf7e0, "unknown_1418cf7e0", fn_1418cf7e0 as *const ()), // sub_1418cf7e0
    Replacement::draft(0x1418cf8b0, "unknown_1418cf8b0", tpl_34703::<0x10, 0x8, 0x18, 0x28, 0x30, 0x48, 0x20, 0x10048, 0x1418cf6e0> as *const ()), // sub_1418cf8b0
    Replacement::draft(0x1418cf920, "unknown_1418cf920", tpl_34704::<0x10, 0x18, 0x8, 0x48, 0x10048, 0x20, 0x28, 0x30, 0x1418cf6e0> as *const ()), // sub_1418cf920
    Replacement::draft(0x1418cf9e0, "unknown_1418cf9e0", fn_1418cf9e0 as *const ()), // sub_1418cf9e0
    Replacement::draft(0x1418cfa80, "unknown_1418cfa80", fn_1418cfa80 as *const ()), // sub_1418cfa80
    Replacement::draft(0x1418cfb30, "unknown_1418cfb30", fn_1418cfb30 as *const ()), // sub_1418cfb30
    Replacement::draft(0x1418cfcd0, "unknown_1418cfcd0", fn_1418cfcd0 as *const ()), // sub_1418cfcd0
    Replacement::draft(0x1418d0450, "bigfish_1418d0450", tpl_35082::<0x8, 0x10, 0x140287960> as *const ()), // bigfish::ServerPlayerController::ClassTypeInfoImplementation::bigfish::ServerPlayerController::ClassTypeInfoImplementati
    Replacement::draft(0x1418d05f0, "bigfish_1418d05f0", fn_1418d05f0 as *const ()), // bigfish::ServerPlayerController::bigfish::ServerPlayerController::vfunc_3
    Replacement::draft_throws(0x1418d0a70, "bigfish_1418d0a70", fn_1418d0a70 as *const (), Throws::Discard), // bigfish::SendLuaEventDecoratorInstance::bigfish::SendLuaEventDecoratorInstance::vfunc_9
    Replacement::draft(0x1418d10b0, "bigfish_1418d10b0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bigfish::SendLuaEventDecoratorInstance::ClassTypeInfoImplementation::bigfish::SendLuaEventDecoratorInstance::ClassTypeIn
    Replacement::draft_throws(0x1418d1310, "unknown_1418d1310", fn_1418d1310 as *const (), Throws::Discard), // sub_1418d1310
    Replacement::draft_throws(0x1418d1540, "unknown_1418d1540", fn_1418d1540 as *const (), Throws::Discard), // sub_1418d1540
    Replacement::draft_throws(0x1418d18b0, "unknown_1418d18b0", fn_1418d18b0 as *const (), Throws::Discard), // sub_1418d18b0
    Replacement::draft(0x1418d1980, "unknown_1418d1980", fn_1418d1980 as *const ()), // sub_1418d1980
    Replacement::draft(0x1418d1a50, "unknown_1418d1a50", fn_1418d1a50 as *const ()), // sub_1418d1a50
    Replacement::draft(0x1418d1a80, "unknown_1418d1a80", fn_1418d1a80 as *const ()), // sub_1418d1a80
    Replacement::draft_throws(0x1418d1ab0, "unknown_1418d1ab0", fn_1418d1ab0 as *const (), Throws::Discard), // sub_1418d1ab0
    Replacement::draft_throws(0x1418d1b10, "unknown_1418d1b10", fn_1418d1b10 as *const (), Throws::Discard), // sub_1418d1b10
    Replacement::draft(0x1418d20c0, "unknown_1418d20c0", fn_1418d20c0 as *const ()), // sub_1418d20c0
    Replacement::draft(0x1418d21d0, "unknown_1418d21d0", fn_1418d21d0 as *const ()), // sub_1418d21d0
    Replacement::draft(0x1418d2260, "unknown_1418d2260", tpl_94867::<0x8, 0x141f7c390> as *const ()), // sub_1418d2260
    Replacement::draft(0x1418d2390, "unknown_1418d2390", tpl_34393 as *const ()), // sub_1418d2390
    Replacement::draft(0x1418d25c0, "unknown_1418d25c0", tpl_34393 as *const ()), // sub_1418d25c0
    Replacement::draft(0x1418d2650, "unknown_1418d2650", tpl_34393 as *const ()), // sub_1418d2650
    Replacement::draft(0x1418d26e0, "unknown_1418d26e0", fn_1418d26e0 as *const ()), // sub_1418d26e0
    Replacement::draft(0x1418d2800, "unknown_1418d2800", fn_1418d2800 as *const ()), // sub_1418d2800
    Replacement::draft(0x1418d29b0, "unknown_1418d29b0", fn_1418d29b0 as *const ()), // sub_1418d29b0
    Replacement::draft(0x1418d2b70, "unknown_1418d2b70", fn_1418d2b70 as *const ()), // sub_1418d2b70
    Replacement::draft(0x1418d2c20, "unknown_1418d2c20", fn_1418d2c20 as *const ()), // sub_1418d2c20
    Replacement::draft(0x1418d2cd0, "unknown_1418d2cd0", fn_1418d2cd0 as *const ()), // sub_1418d2cd0
    Replacement::draft(0x1418d2d00, "unknown_1418d2d00", fn_1418d2d00 as *const ()), // sub_1418d2d00
    Replacement::draft(0x1418d2dd0, "unknown_1418d2dd0", tpl_94875::<0x8, 0xc> as *const ()), // sub_1418d2dd0
    Replacement::draft(0x1418d2ed0, "unknown_1418d2ed0", tpl_34684::<0x1401de830> as *const ()), // sub_1418d2ed0
    Replacement::draft(0x1418d2f80, "unknown_1418d2f80", fn_1418d2f80 as *const ()), // sub_1418d2f80
    Replacement::draft(0x1418d30c0, "unknown_1418d30c0", tpl_35097 as *const ()), // sub_1418d30c0
    Replacement::draft(0x1418d3280, "unknown_1418d3280", fn_1418d3280 as *const ()), // sub_1418d3280
    Replacement::draft(0x1418d33a0, "unknown_1418d33a0", fn_1418d33a0 as *const ()), // sub_1418d33a0
    Replacement::draft(0x1418d3450, "unknown_1418d3450", tpl_35099::<0x20, 0x40> as *const ()), // sub_1418d3450
    Replacement::draft(0x1418d3500, "unknown_1418d3500", fn_1418d3500 as *const ()), // sub_1418d3500
    Replacement::draft(0x1418d36a0, "unknown_1418d36a0", tpl_34615 as *const ()), // sub_1418d36a0
    Replacement::draft(0x1418d3750, "unknown_1418d3750", tpl_94790 as *const ()), // sub_1418d3750
    Replacement::draft(0x1418d3800, "unknown_1418d3800", fn_1418d3800 as *const ()), // sub_1418d3800
    Replacement::draft(0x1418d3880, "unknown_1418d3880", tpl_35102::<0x14240aa00> as *const ()), // sub_1418d3880
    Replacement::draft(0x1418d3a00, "unknown_1418d3a00", tpl_35102::<0x14240a730> as *const ()), // sub_1418d3a00
    Replacement::draft(0x1418d40a0, "unknown_1418d40a0", tpl_35103::<0x14028b790, 0x78> as *const ()), // sub_1418d40a0
    Replacement::draft(0x1418d66a0, "unknown_1418d66a0", tpl_35123::<0x8> as *const ()), // sub_1418d66a0
    Replacement::draft(0x1418d67e0, "unknown_1418d67e0", fn_1418d67e0 as *const ()), // sub_1418d67e0
    Replacement::draft(0x1418d6960, "unknown_1418d6960", tpl_34429::<0x28> as *const ()), // sub_1418d6960
    Replacement::draft_throws(0x1418d6970, "unknown_1418d6970", fn_1418d6970 as *const (), Throws::Caught), // sub_1418d6970
    Replacement::draft(0x1418d7430, "unknown_1418d7430", fn_1418d7430 as *const ()), // sub_1418d7430
    Replacement::draft_throws(0x1418d74c0, "unknown_1418d74c0", fn_1418d74c0 as *const (), Throws::Caught), // sub_1418d74c0
    Replacement::draft_throws(0x1418d7740, "unknown_1418d7740", fn_1418d7740 as *const (), Throws::Caught), // sub_1418d7740
    Replacement::draft(0x1418d7800, "unknown_1418d7800", tpl_2025::<0x14240ab10, 0x30, 0x8> as *const ()), // sub_1418d7800
    Replacement::draft(0x1418d7830, "unknown_1418d7830", fn_1418d7830 as *const ()), // sub_1418d7830
    Replacement::draft_throws(0x1418d7880, "unknown_1418d7880", fn_1418d7880 as *const (), Throws::Discard), // sub_1418d7880
    Replacement::draft_throws(0x1418d7a40, "unknown_1418d7a40", fn_1418d7a40 as *const (), Throws::Caught), // sub_1418d7a40
    Replacement::draft_throws(0x1418d7c00, "unknown_1418d7c00", fn_1418d7c00 as *const (), Throws::Caught), // sub_1418d7c00
    Replacement::draft(0x1418d8690, "unknown_1418d8690", tpl_35131::<0x14028b790, 0x78> as *const ()), // sub_1418d8690
    Replacement::draft(0x1418d86c0, "unknown_1418d86c0", tpl_35132::<0x4> as *const ()), // sub_1418d86c0
    Replacement::draft_throws(0x1418d86e0, "unknown_1418d86e0", fn_1418d86e0 as *const (), Throws::Caught), // sub_1418d86e0
    Replacement::draft(0x1418d8a40, "unknown_1418d8a40", tpl_35131::<0x14028b790, 0x78> as *const ()), // sub_1418d8a40
    Replacement::draft(0x1418d8a70, "unknown_1418d8a70", tpl_35132::<0x4> as *const ()), // sub_1418d8a70
    Replacement::draft_throws(0x1418d8a90, "unknown_1418d8a90", fn_1418d8a90 as *const (), Throws::Caught), // sub_1418d8a90
    Replacement::draft(0x1418d8f10, "unknown_1418d8f10", tpl_35131::<0x14028b790, 0x78> as *const ()), // sub_1418d8f10
    Replacement::draft(0x1418d8f40, "unknown_1418d8f40", fn_1418d8f40 as *const ()), // sub_1418d8f40
    Replacement::draft_throws(0x1418d8f70, "unknown_1418d8f70", fn_1418d8f70 as *const (), Throws::Caught), // sub_1418d8f70
    Replacement::draft_throws(0x1418d9080, "unknown_1418d9080", fn_1418d9080 as *const (), Throws::Discard), // sub_1418d9080
    Replacement::draft_throws(0x1418d9880, "unknown_1418d9880", fn_1418d9880 as *const (), Throws::Caught), // sub_1418d9880
    Replacement::draft_throws(0x1418d9d80, "unknown_1418d9d80", fn_1418d9d80 as *const (), Throws::Caught), // sub_1418d9d80
    Replacement::draft_throws(0x1418da020, "unknown_1418da020", fn_1418da020 as *const (), Throws::Caught), // sub_1418da020
    Replacement::draft_throws(0x1418da1f0, "unknown_1418da1f0", fn_1418da1f0 as *const (), Throws::Caught), // sub_1418da1f0
    Replacement::draft_throws(0x1418dab50, "unknown_1418dab50", fn_1418dab50 as *const (), Throws::Caught), // sub_1418dab50
    Replacement::draft(0x1418db1c0, "unknown_1418db1c0", fn_1418db1c0 as *const ()), // sub_1418db1c0
    Replacement::draft(0x1418db240, "unknown_1418db240", fn_1418db240 as *const ()), // sub_1418db240
    Replacement::draft(0x1418db560, "unknown_1418db560", fn_1418db560 as *const ()), // sub_1418db560
    Replacement::draft(0x1418db6d0, "unknown_1418db6d0", fn_1418db6d0 as *const ()), // sub_1418db6d0
    Replacement::draft(0x1418dbae0, "unknown_1418dbae0", fn_1418dbae0 as *const ()), // sub_1418dbae0
    Replacement::draft(0x1418dbed0, "unknown_1418dbed0", fn_1418dbed0 as *const ()), // sub_1418dbed0
    Replacement::draft(0x1418dbf00, "unknown_1418dbf00", fn_1418dbf00 as *const ()), // sub_1418dbf00
    Replacement::draft(0x1418dbf40, "unknown_1418dbf40", fn_1418dbf40 as *const ()), // sub_1418dbf40
    Replacement::draft(0x1418dbfb0, "unknown_1418dbfb0", fn_1418dbfb0 as *const ()), // sub_1418dbfb0
    Replacement::draft(0x1418dc020, "unknown_1418dc020", fn_1418dc020 as *const ()), // sub_1418dc020
    Replacement::draft(0x1418dc040, "unknown_1418dc040", fn_1418dc040 as *const ()), // sub_1418dc040
    Replacement::draft_throws(0x1418dc070, "unknown_1418dc070", fn_1418dc070 as *const (), Throws::Discard), // sub_1418dc070
    Replacement::draft_throws(0x1418dc200, "unknown_1418dc200", fn_1418dc200 as *const (), Throws::Discard), // sub_1418dc200
    Replacement::draft(0x1418dc510, "unknown_1418dc510", fn_1418dc510 as *const ()), // sub_1418dc510
    Replacement::draft(0x1418dc5b0, "unknown_1418dc5b0", fn_1418dc5b0 as *const ()), // sub_1418dc5b0
    Replacement::draft(0x1418dc620, "unknown_1418dc620", fn_1418dc620 as *const ()), // sub_1418dc620
    Replacement::draft(0x1418dc670, "unknown_1418dc670", fn_1418dc670 as *const ()), // sub_1418dc670
    Replacement::draft(0x1418dc6b0, "unknown_1418dc6b0", fn_1418dc6b0 as *const ()), // sub_1418dc6b0
    Replacement::draft_throws(0x1418dc7d0, "unknown_1418dc7d0", fn_1418dc7d0 as *const (), Throws::Caught), // sub_1418dc7d0
    Replacement::draft_throws(0x1418dc9e0, "unknown_1418dc9e0", fn_1418dc9e0 as *const (), Throws::Caught), // sub_1418dc9e0
    Replacement::draft_throws(0x1418dcbb0, "unknown_1418dcbb0", fn_1418dcbb0 as *const (), Throws::Caught), // sub_1418dcbb0
    Replacement::draft(0x1418dd190, "unknown_1418dd190", fn_1418dd190 as *const ()), // sub_1418dd190
    Replacement::draft(0x1418dd580, "unknown_1418dd580", fn_1418dd580 as *const ()), // sub_1418dd580
    Replacement::draft(0x1418ddf00, "unknown_1418ddf00", fn_1418ddf00 as *const ()), // sub_1418ddf00
    Replacement::draft_throws(0x1418de750, "unknown_1418de750", fn_1418de750 as *const (), Throws::Discard), // sub_1418de750
    Replacement::draft(0x1418de7c0, "unknown_1418de7c0", fn_1418de7c0 as *const ()), // sub_1418de7c0
    Replacement::draft_throws(0x1418dea20, "unknown_1418dea20", fn_1418dea20 as *const (), Throws::Caught), // sub_1418dea20
    Replacement::draft(0x1418defa0, "unknown_1418defa0", fn_1418defa0 as *const ()), // sub_1418defa0
    Replacement::draft(0x1418deff0, "unknown_1418deff0", fn_1418deff0 as *const ()), // sub_1418deff0
    Replacement::draft(0x1418df5a0, "unknown_1418df5a0", fn_1418df5a0 as *const ()), // sub_1418df5a0
    Replacement::draft(0x1418df9b0, "unknown_1418df9b0", fn_1418df9b0 as *const ()), // sub_1418df9b0
    Replacement::draft_throws(0x1418dfb40, "unknown_1418dfb40", fn_1418dfb40 as *const (), Throws::Caught), // sub_1418dfb40
    Replacement::draft(0x1418e0430, "savegamecontent_1418e0430", tpl_35169::<0x8, 0xc, 0x4, 0x10> as *const ()), // SaveGameContent::Entity<bigfish::world_interaction::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigf
    Replacement::draft(0x1418e0470, "savegamecontent_1418e0470", tpl_35169::<0x8, 0xc, 0x4, 0x10> as *const ()), // SaveGameContent::Entity<bigfish::world_interaction::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigf
    Replacement::draft(0x1418e0640, "savegamecontent_1418e0640", tpl_35170::<0x8, 0x10> as *const ()), // SaveGameContent::Entity<bigfish::world_interaction::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigf
    Replacement::draft(0x1418e0a40, "savegamecontent_1418e0a40", tpl_35171::<0x8, 0xc, 0x4> as *const ()), // SaveGameContent::Entity<bigfish::health::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigfish::health
    Replacement::draft(0x1418e0a80, "savegamecontent_1418e0a80", tpl_35171::<0x8, 0xc, 0x4> as *const ()), // SaveGameContent::Entity<bigfish::health::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigfish::health
    Replacement::draft(0x1418e0c50, "savegamecontent_1418e0c50", tpl_35172::<0x8> as *const ()), // SaveGameContent::Entity<bigfish::health::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigfish::health
    Replacement::draft(0x1418e0c90, "savegamecontent_1418e0c90", fn_1418e0c90 as *const ()), // SaveGameContent::Entity<bigfish::health::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigfish::health
    Replacement::draft(0x1418e1050, "savegamecontent_1418e1050", tpl_35174::<0x8, 0x9, 0x1, 0xa, 0x2, 0xc, 0x4, 0x10, 0x14> as *const ()), // SaveGameContent::Entity<bigfish::door::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigfish::door::Sa
    Replacement::draft(0x1418e10b0, "savegamecontent_1418e10b0", tpl_35174::<0x8, 0x9, 0x1, 0xa, 0x2, 0xc, 0x4, 0x10, 0x14> as *const ()), // SaveGameContent::Entity<bigfish::door::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigfish::door::Sa
    Replacement::draft(0x1418e12a0, "savegamecontent_1418e12a0", fn_1418e12a0 as *const ()), // SaveGameContent::Entity<bigfish::door::SaveData>::ClassTypeInfoImplementation::SaveGameContent::Entity<bigfish::door::Sa
    Replacement::draft(0x1418e1660, "savegamecontent_1418e1660", tpl_35176::<0x8, 0x1402a7930> as *const ()), // SaveGameContent::Entity<SaveGameContent::Player>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent::
    Replacement::draft(0x1418e16b0, "savegamecontent_1418e16b0", tpl_35176::<0x8, 0x1402a7930> as *const ()), // SaveGameContent::Entity<SaveGameContent::Player>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent::
    Replacement::draft(0x1418e18c0, "savegamecontent_1418e18c0", tpl_35177::<0x78, 0x1402c30b0, 0x60, 0x1401f2bc0, 0x50, 0x40, 0x10> as *const ()), // SaveGameContent::Entity<SaveGameContent::Player>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent::
    Replacement::draft(0x1418e1910, "savegamecontent_1418e1910", fn_1418e1910 as *const ()), // SaveGameContent::Entity<SaveGameContent::Player>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent::
    Replacement::draft(0x1418e1960, "savegamecontent_1418e1960", tpl_35179::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x78, 0x1402c30b0, 0x60, 0x1401f2bc0, 0x50, 0x40, 0x10, 0x142442c80> as *const ()), // SaveGameContent::Entity<SaveGameContent::Player>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent::
    Replacement::draft(0x1418e19f0, "savegamecontent_1418e19f0", tpl_35180::<0x78, 0x1402c30b0, 0x60, 0x1401f2bc0, 0x50, 0x40, 0x10, 0x142442de0> as *const ()), // SaveGameContent::Entity<SaveGameContent::Player>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent::
    Replacement::draft(0x1418e1db0, "savegamecontent_1418e1db0", tpl_35181::<0x8, 0x10, 0x1402976e0> as *const ()), // SaveGameContent::Entity<SaveGameContent::Breakable>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e1e00, "savegamecontent_1418e1e00", tpl_35181::<0x8, 0x10, 0x1402976e0> as *const ()), // SaveGameContent::Entity<SaveGameContent::Breakable>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e1fe0, "savegamecontent_1418e1fe0", fn_1418e1fe0 as *const ()), // SaveGameContent::Entity<SaveGameContent::Breakable>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e2020, "savegamecontent_1418e2020", fn_1418e2020 as *const ()), // SaveGameContent::Entity<SaveGameContent::Breakable>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e20a0, "savegamecontent_1418e20a0", tpl_94938::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x14028b170, 0x142442c80> as *const ()), // SaveGameContent::Entity<SaveGameContent::Breakable>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e2100, "savegamecontent_1418e2100", tpl_35184::<0x10, 0x14028b170, 0x142442de0> as *const ()), // SaveGameContent::Entity<SaveGameContent::Breakable>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e2490, "savegamecontent_1418e2490", tpl_35185::<0x8> as *const ()), // SaveGameContent::Entity<SaveGameContent::PlayerCamera>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameCon
    Replacement::draft(0x1418e24d0, "savegamecontent_1418e24d0", tpl_35185::<0x8> as *const ()), // SaveGameContent::Entity<SaveGameContent::PlayerCamera>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameCon
    Replacement::draft(0x1418e26a0, "savegamecontent_1418e26a0", tpl_35170::<0x8, 0x10> as *const ()), // SaveGameContent::Entity<SaveGameContent::PlayerCamera>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameCon
    Replacement::draft(0x1418e26e0, "savegamecontent_1418e26e0", tpl_35186::<0x8, 0x10> as *const ()), // SaveGameContent::Entity<SaveGameContent::PlayerCamera>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameCon
    Replacement::draft(0x1418e2aa0, "savegamecontent_1418e2aa0", tpl_35187::<0x8, 0xc, 0x4, 0xd, 0x5, 0xf, 0x7, 0x10> as *const ()), // SaveGameContent::Entity<SaveGameContent::Timeline>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent
    Replacement::draft(0x1418e2af0, "savegamecontent_1418e2af0", tpl_35187::<0x8, 0xc, 0x4, 0xd, 0x5, 0xf, 0x7, 0x10> as *const ()), // SaveGameContent::Entity<SaveGameContent::Timeline>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent
    Replacement::draft(0x1418e2cd0, "savegamecontent_1418e2cd0", tpl_35170::<0x8, 0x10> as *const ()), // SaveGameContent::Entity<SaveGameContent::Timeline>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent
    Replacement::draft(0x1418e32c0, "savegamecontent_1418e32c0", fn_1418e32c0 as *const ()), // SaveGameContent::Entity<SaveGameContent::EnemyState>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConte
    Replacement::draft(0x1418e3680, "savegamecontent_1418e3680", tpl_35176::<0x8, 0x140293310> as *const ()), // SaveGameContent::Entity<SaveGameContent::Knowledge>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e36d0, "savegamecontent_1418e36d0", tpl_35176::<0x8, 0x140293310> as *const ()), // SaveGameContent::Entity<SaveGameContent::Knowledge>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e3900, "savegamecontent_1418e3900", fn_1418e3900 as *const ()), // SaveGameContent::Entity<SaveGameContent::Knowledge>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e3940, "savegamecontent_1418e3940", tpl_35190::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // SaveGameContent::Entity<SaveGameContent::Knowledge>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e39a0, "savegamecontent_1418e39a0", tpl_35191::<0x8, 0x1401de830, 0x142442de0> as *const ()), // SaveGameContent::Entity<SaveGameContent::Knowledge>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e3d30, "savegamecontent_1418e3d30", tpl_35192::<0x8, 0x10> as *const ()), // SaveGameContent::Entity<SaveGameContent::Transform>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e3d60, "savegamecontent_1418e3d60", tpl_35192::<0x8, 0x10> as *const ()), // SaveGameContent::Entity<SaveGameContent::Transform>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e3f20, "savegamecontent_1418e3f20", fn_1418e3f20 as *const ()), // SaveGameContent::Entity<SaveGameContent::Transform>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConten
    Replacement::draft(0x1418e44d0, "savegamecontent_1418e44d0", tpl_35172::<0x8> as *const ()), // SaveGameContent::Entity<SaveGameContent::Parent>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameContent::
    Replacement::draft(0x1418e4ac0, "savegamecontent_1418e4ac0", fn_1418e4ac0 as *const ()), // SaveGameContent::Entity<SaveGameContent::HideReason>::ClassTypeInfoImplementation::SaveGameContent::Entity<SaveGameConte
    Replacement::draft(0x1418e4e60, "savegamecontent_1418e4e60", tpl_35195::<0x1402cab80> as *const ()), // SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplemen
    Replacement::draft(0x1418e4ea0, "savegamecontent_1418e4ea0", tpl_35195::<0x1402cab80> as *const ()), // SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplemen
    Replacement::draft(0x1418e5070, "savegamecontent_1418e5070", fn_1418e5070 as *const ()), // SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplemen
    Replacement::draft(0x1418e5100, "savegamecontent_1418e5100", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1402c30b0, 0x142442c80> as *const ()), // SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplemen
    Replacement::draft(0x1418e5160, "savegamecontent_1418e5160", tpl_35198::<0x1402c30b0, 0x142442de0> as *const ()), // SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlots::ClassTypeInfoImplemen
    Replacement::draft(0x1418e55d0, "savegamecontent_1418e55d0", tpl_35199::<0x14387a2b0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::PlayerQuickSlotBindings::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlotBindings::ClassTy
    Replacement::draft(0x1418e5630, "savegamecontent_1418e5630", tpl_35199::<0x14387a2b0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::PlayerQuickSlotBindings::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlotBindings::ClassTy
    Replacement::draft(0x1418e56e0, "savegamecontent_1418e56e0", tpl_35200::<0x140265ba0, 0x10> as *const ()), // SaveGameContent::PlayerQuickSlotBindings::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlotBindings::ClassTy
    Replacement::draft(0x1418e5770, "savegamecontent_1418e5770", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // SaveGameContent::PlayerQuickSlotBindings::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlotBindings::ClassTy
    Replacement::draft(0x1418e57d0, "savegamecontent_1418e57d0", tpl_35198::<0x1401de830, 0x142442de0> as *const ()), // SaveGameContent::PlayerQuickSlotBindings::ClassTypeInfoImplementation::SaveGameContent::PlayerQuickSlotBindings::ClassTy
    Replacement::draft(0x1418e5d20, "savegamecontent_1418e5d20", fn_1418e5d20 as *const ()), // SaveGameContent::Episode::ClassTypeInfoImplementation::SaveGameContent::Episode::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1418e6030, "savegamecontent_1418e6030", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x1418e6090, "savegamecontent_1418e6090", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_41
    Replacement::draft(0x1418e6190, "savegamecontent_1418e6190", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x1418e61f0, "savegamecontent_1418e61f0", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_43
    Replacement::draft(0x1418e63d0, "savegamecontent_1418e63d0", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x1418e6430, "savegamecontent_1418e6430", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_49
    Replacement::draft(0x1418e6530, "savegamecontent_1418e6530", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x1418e6590, "savegamecontent_1418e6590", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_51
    Replacement::draft(0x1418e65f0, "savegamecontent_1418e65f0", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1418e6650, "savegamecontent_1418e6650", tpl_35199::<0x14387a288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::World::ClassTypeInfoImplementation::SaveGameContent::World::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1418e6c50, "savegamecontent_1418e6c50", fn_1418e6c50 as *const ()), // SaveGameContent::Global::ClassTypeInfoImplementation::SaveGameContent::Global::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1418e6d10, "savegamecontent_1418e6d10", fn_1418e6d10 as *const ()), // SaveGameContent::Global::ClassTypeInfoImplementation::SaveGameContent::Global::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1418e6d70, "savegamecontent_1418e6d70", fn_1418e6d70 as *const ()), // SaveGameContent::Global::ClassTypeInfoImplementation::SaveGameContent::Global::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1418e6e60, "savegamecontent_1418e6e60", tpl_365::<0x1402a9d30> as *const ()), // SaveGameContent::Global::ClassTypeInfoImplementation::SaveGameContent::Global::ClassTypeInfoImplementation::vfunc_8
    Replacement::draft(0x1418e6f20, "savegamecontent_1418e6f20", tpl_35205::<0x1430af328, 0x4, 0x14020f270> as *const ()), // SaveGameContent::Player::ClassTypeInfoImplementation::SaveGameContent::Player::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x1418e6f60, "savegamecontent_1418e6f60", tpl_35205::<0x1430af328, 0x4, 0x14020f270> as *const ()), // SaveGameContent::Player::ClassTypeInfoImplementation::SaveGameContent::Player::ClassTypeInfoImplementation::vfunc_37
    Replacement::draft(0x1418e73b0, "savegamecontent_1418e73b0", tpl_35177::<0x70, 0x1402c30b0, 0x58, 0x1401f2bc0, 0x48, 0x38, 0x8> as *const ()), // SaveGameContent::Player::ClassTypeInfoImplementation::SaveGameContent::Player::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x1418e7400, "savegamecontent_1418e7400", fn_1418e7400 as *const ()), // SaveGameContent::Player::ClassTypeInfoImplementation::SaveGameContent::Player::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x1418e7450, "savegamecontent_1418e7450", tpl_35179::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x70, 0x1402c30b0, 0x58, 0x1401f2bc0, 0x48, 0x38, 0x8, 0x142442c80> as *const ()), // SaveGameContent::Player::ClassTypeInfoImplementation::SaveGameContent::Player::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1418e74e0, "savegamecontent_1418e74e0", tpl_35180::<0x70, 0x1402c30b0, 0x58, 0x1401f2bc0, 0x48, 0x38, 0x8, 0x142442de0> as *const ()), // SaveGameContent::Player::ClassTypeInfoImplementation::SaveGameContent::Player::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1418e7840, "savegamecontent_1418e7840", tpl_35207::<0x8, 0x14240aa00> as *const ()), // SaveGameContent::StoredCharacterUpgrade::ClassTypeInfoImplementation::SaveGameContent::StoredCharacterUpgrade::ClassType
    Replacement::draft(0x1418e7870, "savegamecontent_1418e7870", tpl_35207::<0x8, 0x14240aa00> as *const ()), // SaveGameContent::StoredCharacterUpgrade::ClassTypeInfoImplementation::SaveGameContent::StoredCharacterUpgrade::ClassType
    Replacement::draft(0x1418e79a0, "savegamecontent_1418e79a0", tpl_35199::<0x14387a250, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::StoredCharacterUpgrade::ClassTypeInfoImplementation::SaveGameContent::StoredCharacterUpgrade::ClassType
    Replacement::draft(0x1418e7a00, "savegamecontent_1418e7a00", tpl_35199::<0x14387a250, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::StoredCharacterUpgrade::ClassTypeInfoImplementation::SaveGameContent::StoredCharacterUpgrade::ClassType
    Replacement::draft(0x1418e7ab0, "savegamecontent_1418e7ab0", fn_1418e7ab0 as *const ()), // SaveGameContent::StoredCharacterUpgrade::ClassTypeInfoImplementation::SaveGameContent::StoredCharacterUpgrade::ClassType
    Replacement::draft(0x1418e7af0, "savegamecontent_1418e7af0", fn_1418e7af0 as *const ()), // SaveGameContent::StoredCharacterUpgrade::ClassTypeInfoImplementation::SaveGameContent::StoredCharacterUpgrade::ClassType
    Replacement::draft(0x1418e7b20, "savegamecontent_1418e7b20", tpl_94951::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x14240ab10, 0x142442c80> as *const ()), // SaveGameContent::StoredCharacterUpgrade::ClassTypeInfoImplementation::SaveGameContent::StoredCharacterUpgrade::ClassType
    Replacement::draft(0x1418e7b80, "savegamecontent_1418e7b80", tpl_35184::<0x4, 0x14240ab10, 0x142442de0> as *const ()), // SaveGameContent::StoredCharacterUpgrade::ClassTypeInfoImplementation::SaveGameContent::StoredCharacterUpgrade::ClassType
    Replacement::draft(0x1418e80a0, "savegamecontent_1418e80a0", fn_1418e80a0 as *const ()), // SaveGameContent::StoredCaseBoardItem::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardItem::ClassTypeInfoIm
    Replacement::draft(0x1418e8230, "savegamecontent_1418e8230", fn_1418e8230 as *const ()), // SaveGameContent::StoredCaseBoardItem::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardItem::ClassTypeInfoIm
    Replacement::draft(0x1418e8760, "savegamecontent_1418e8760", tpl_35212::<0x14387a3e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::StoredCaseBoardCase::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardCase::ClassTypeInfoIm
    Replacement::draft(0x1418e87c0, "savegamecontent_1418e87c0", tpl_35212::<0x14387a3e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::StoredCaseBoardCase::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardCase::ClassTypeInfoIm
    Replacement::draft(0x1418e8870, "savegamecontent_1418e8870", fn_1418e8870 as *const ()), // SaveGameContent::StoredCaseBoardCase::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardCase::ClassTypeInfoIm
    Replacement::draft(0x1418e88d0, "savegamecontent_1418e88d0", tpl_35214::<0x60, 0x140217f00, 0x48, 0x14240ab10> as *const ()), // SaveGameContent::StoredCaseBoardCase::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardCase::ClassTypeInfoIm
    Replacement::draft(0x1418e8900, "savegamecontent_1418e8900", fn_1418e8900 as *const ()), // SaveGameContent::StoredCaseBoardCase::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardCase::ClassTypeInfoIm
    Replacement::draft(0x1418e8930, "savegamecontent_1418e8930", tpl_35216::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x60, 0x140217f00, 0x48, 0x14240ab10, 0x142442c80> as *const ()), // SaveGameContent::StoredCaseBoardCase::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardCase::ClassTypeInfoIm
    Replacement::draft(0x1418e89a0, "savegamecontent_1418e89a0", tpl_35217::<0x60, 0x140217f00, 0x48, 0x14240ab10, 0x142442de0> as *const ()), // SaveGameContent::StoredCaseBoardCase::ClassTypeInfoImplementation::SaveGameContent::StoredCaseBoardCase::ClassTypeInfoIm
    Replacement::draft(0x1418e8ce0, "savegamecontent_1418e8ce0", tpl_35218::<0x8, 0x4, 0xc> as *const ()), // SaveGameContent::InventoryQuickSlot::ClassTypeInfoImplementation::SaveGameContent::InventoryQuickSlot::ClassTypeInfoImpl
    Replacement::draft(0x1418e8d30, "savegamecontent_1418e8d30", tpl_35218::<0x8, 0x4, 0xc> as *const ()), // SaveGameContent::InventoryQuickSlot::ClassTypeInfoImplementation::SaveGameContent::InventoryQuickSlot::ClassTypeInfoImpl
    Replacement::draft(0x1418e8e80, "savegamecontent_1418e8e80", tpl_35199::<0x14387a418, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::InventoryQuickSlot::ClassTypeInfoImplementation::SaveGameContent::InventoryQuickSlot::ClassTypeInfoImpl
    Replacement::draft(0x1418e8ee0, "savegamecontent_1418e8ee0", tpl_35199::<0x14387a418, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::InventoryQuickSlot::ClassTypeInfoImplementation::SaveGameContent::InventoryQuickSlot::ClassTypeInfoImpl
    Replacement::draft(0x1418e8fd0, "savegamecontent_1418e8fd0", tpl_35186::<0x8, 0x10> as *const ()), // SaveGameContent::InventoryQuickSlot::ClassTypeInfoImplementation::SaveGameContent::InventoryQuickSlot::ClassTypeInfoImpl
    Replacement::draft(0x1418e9490, "savegamecontent_1418e9490", tpl_35199::<0x14387a270, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::BraceletCharm::ClassTypeInfoImplementation::SaveGameContent::BraceletCharm::ClassTypeInfoImplementation
    Replacement::draft(0x1418e94f0, "savegamecontent_1418e94f0", tpl_35199::<0x14387a270, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::BraceletCharm::ClassTypeInfoImplementation::SaveGameContent::BraceletCharm::ClassTypeInfoImplementation
    Replacement::draft(0x1418e9aa0, "savegamecontent_1418e9aa0", tpl_35219::<0x14387a228, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::UsedMissionItem::ClassTypeInfoImplementation::SaveGameContent::UsedMissionItem::ClassTypeInfoImplementa
    Replacement::draft(0x1418e9b00, "savegamecontent_1418e9b00", tpl_35219::<0x14387a228, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::UsedMissionItem::ClassTypeInfoImplementation::SaveGameContent::UsedMissionItem::ClassTypeInfoImplementa
    Replacement::draft(0x1418e9bb0, "savegamecontent_1418e9bb0", tpl_34638::<0x8> as *const ()), // SaveGameContent::UsedMissionItem::ClassTypeInfoImplementation::SaveGameContent::UsedMissionItem::ClassTypeInfoImplementa
    Replacement::draft(0x1418e9ce0, "savegamecontent_1418e9ce0", tpl_35220::<0x1430af280, 0x1, 0x14028a830> as *const ()), // SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::vf
    Replacement::draft(0x1418e9d20, "savegamecontent_1418e9d20", tpl_35220::<0x1430af280, 0x1, 0x14028a830> as *const ()), // SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::vf
    Replacement::draft(0x1418e9fa0, "savegamecontent_1418e9fa0", fn_1418e9fa0 as *const ()), // SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::vf
    Replacement::draft(0x1418ea000, "savegamecontent_1418ea000", fn_1418ea000 as *const ()), // SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::vf
    Replacement::draft(0x1418ea150, "savegamecontent_1418ea150", tpl_35199::<0x14387a3d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::vf
    Replacement::draft(0x1418ea1b0, "savegamecontent_1418ea1b0", tpl_35199::<0x14387a3d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::vf
    Replacement::draft(0x1418ea260, "savegamecontent_1418ea260", fn_1418ea260 as *const ()), // SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::SaveGameContent::ShoeboxItem::ClassTypeInfoImplementation::vf
    Replacement::draft(0x1418ea610, "savegamecontent_1418ea610", fn_1418ea610 as *const ()), // SaveGameContent::InventoryItem::ClassTypeInfoImplementation::SaveGameContent::InventoryItem::ClassTypeInfoImplementation
    Replacement::draft(0x1418ea690, "savegamecontent_1418ea690", fn_1418ea690 as *const ()), // SaveGameContent::InventoryItem::ClassTypeInfoImplementation::SaveGameContent::InventoryItem::ClassTypeInfoImplementation
    Replacement::draft(0x1418ea880, "savegamecontent_1418ea880", tpl_35226::<0x10> as *const ()), // SaveGameContent::InventoryItem::ClassTypeInfoImplementation::SaveGameContent::InventoryItem::ClassTypeInfoImplementation
    Replacement::draft(0x1418ea8c0, "savegamecontent_1418ea8c0", fn_1418ea8c0 as *const ()), // SaveGameContent::InventoryItem::ClassTypeInfoImplementation::SaveGameContent::InventoryItem::ClassTypeInfoImplementation
    Replacement::draft_throws(0x1418ea9e0, "savegamecontent_1418ea9e0", tpl_35228::<0x1430aeec4, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // SaveGameContent::Breakable::ClassTypeInfoImplementation::SaveGameContent::Breakable::ClassTypeInfoImplementation::vfunc_
    Replacement::draft_throws(0x1418eaa20, "savegamecontent_1418eaa20", tpl_35228::<0x1430aeec4, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // SaveGameContent::Breakable::ClassTypeInfoImplementation::SaveGameContent::Breakable::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418eadc0, "savegamecontent_1418eadc0", tpl_35199::<0x14387a428, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Breakable::ClassTypeInfoImplementation::SaveGameContent::Breakable::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418eae20, "savegamecontent_1418eae20", tpl_35199::<0x14387a428, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Breakable::ClassTypeInfoImplementation::SaveGameContent::Breakable::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418eaf00, "savegamecontent_1418eaf00", fn_1418eaf00 as *const ()), // SaveGameContent::Breakable::ClassTypeInfoImplementation::SaveGameContent::Breakable::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418eb370, "savegamecontent_1418eb370", tpl_35230::<0x8> as *const ()), // SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::
    Replacement::draft(0x1418eb390, "savegamecontent_1418eb390", tpl_35230::<0x8> as *const ()), // SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::
    Replacement::draft(0x1418eb4b0, "savegamecontent_1418eb4b0", tpl_35199::<0x14387a4f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::
    Replacement::draft(0x1418eb510, "savegamecontent_1418eb510", tpl_35199::<0x14387a4f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::
    Replacement::draft(0x1418eb5c0, "savegamecontent_1418eb5c0", tpl_34638::<0x8> as *const ()), // SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::SaveGameContent::PlayerCamera::ClassTypeInfoImplementation::
    Replacement::draft(0x1418eb9c0, "savegamecontent_1418eb9c0", fn_1418eb9c0 as *const ()), // SaveGameContent::Timeline::ClassTypeInfoImplementation::SaveGameContent::Timeline::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x1418eba20, "savegamecontent_1418eba20", fn_1418eba20 as *const ()), // SaveGameContent::Timeline::ClassTypeInfoImplementation::SaveGameContent::Timeline::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x1418ebb90, "savegamecontent_1418ebb90", tpl_35199::<0x14387a2e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Timeline::ClassTypeInfoImplementation::SaveGameContent::Timeline::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1418ebbf0, "savegamecontent_1418ebbf0", tpl_35199::<0x14387a2e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Timeline::ClassTypeInfoImplementation::SaveGameContent::Timeline::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1418ebca0, "savegamecontent_1418ebca0", tpl_34638::<0x8> as *const ()), // SaveGameContent::Timeline::ClassTypeInfoImplementation::SaveGameContent::Timeline::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1418ec030, "savegamecontent_1418ec030", tpl_35233::<0x8> as *const ()), // SaveGameContent::EnemyState::ClassTypeInfoImplementation::SaveGameContent::EnemyState::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1418ec050, "savegamecontent_1418ec050", tpl_35233::<0x8> as *const ()), // SaveGameContent::EnemyState::ClassTypeInfoImplementation::SaveGameContent::EnemyState::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1418ec170, "savegamecontent_1418ec170", tpl_35199::<0x14387a390, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::EnemyState::ClassTypeInfoImplementation::SaveGameContent::EnemyState::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1418ec1d0, "savegamecontent_1418ec1d0", tpl_35199::<0x14387a390, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::EnemyState::ClassTypeInfoImplementation::SaveGameContent::EnemyState::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1418ec280, "savegamecontent_1418ec280", fn_1418ec280 as *const ()), // SaveGameContent::EnemyState::ClassTypeInfoImplementation::SaveGameContent::EnemyState::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1418ec3a0, "savegamecontent_1418ec3a0", tpl_35220::<0x1430aefe0, 0x1, 0x14028a830> as *const ()), // SaveGameContent::Knowledge::ClassTypeInfoImplementation::SaveGameContent::Knowledge::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ec3e0, "savegamecontent_1418ec3e0", tpl_35220::<0x1430aefe0, 0x1, 0x14028a830> as *const ()), // SaveGameContent::Knowledge::ClassTypeInfoImplementation::SaveGameContent::Knowledge::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ec7c0, "savegamecontent_1418ec7c0", tpl_35235::<0x14387a230, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Knowledge::ClassTypeInfoImplementation::SaveGameContent::Knowledge::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ec820, "savegamecontent_1418ec820", tpl_35235::<0x14387a230, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Knowledge::ClassTypeInfoImplementation::SaveGameContent::Knowledge::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ec910, "savegamecontent_1418ec910", fn_1418ec910 as *const ()), // SaveGameContent::Knowledge::ClassTypeInfoImplementation::SaveGameContent::Knowledge::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ec950, "savegamecontent_1418ec950", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // SaveGameContent::Knowledge::ClassTypeInfoImplementation::SaveGameContent::Knowledge::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ec9b0, "savegamecontent_1418ec9b0", tpl_35198::<0x1401de830, 0x142442de0> as *const ()), // SaveGameContent::Knowledge::ClassTypeInfoImplementation::SaveGameContent::Knowledge::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ecce0, "savegamecontent_1418ecce0", tpl_35237::<0x8> as *const ()), // SaveGameContent::Transform::ClassTypeInfoImplementation::SaveGameContent::Transform::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ecd00, "savegamecontent_1418ecd00", tpl_35237::<0x8> as *const ()), // SaveGameContent::Transform::ClassTypeInfoImplementation::SaveGameContent::Transform::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ece20, "savegamecontent_1418ece20", tpl_35238::<0x14387a410, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Transform::ClassTypeInfoImplementation::SaveGameContent::Transform::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ece80, "savegamecontent_1418ece80", tpl_35238::<0x14387a410, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Transform::ClassTypeInfoImplementation::SaveGameContent::Transform::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ecf30, "savegamecontent_1418ecf30", tpl_35239 as *const ()), // SaveGameContent::Transform::ClassTypeInfoImplementation::SaveGameContent::Transform::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x1418ed460, "savegamecontent_1418ed460", tpl_35199::<0x14387a488, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Parent::ClassTypeInfoImplementation::SaveGameContent::Parent::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1418ed4c0, "savegamecontent_1418ed4c0", tpl_35199::<0x14387a488, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::Parent::ClassTypeInfoImplementation::SaveGameContent::Parent::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1418eda40, "savegamecontent_1418eda40", tpl_35199::<0x14387a388, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::HideReason::ClassTypeInfoImplementation::SaveGameContent::HideReason::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1418edaa0, "savegamecontent_1418edaa0", tpl_35199::<0x14387a388, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // SaveGameContent::HideReason::ClassTypeInfoImplementation::SaveGameContent::HideReason::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1418ee010, "bigfish_1418ee010", tpl_35240::<0x14387a3d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bigfish::save_game::SaveGameHeader::ClassTypeInfoImplementation::bigfish::save_game::SaveGameHeader::ClassTypeInfoImplem
    Replacement::draft(0x1418ee070, "bigfish_1418ee070", tpl_35240::<0x14387a3d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bigfish::save_game::SaveGameHeader::ClassTypeInfoImplementation::bigfish::save_game::SaveGameHeader::ClassTypeInfoImplem
    Replacement::draft(0x1418ee140, "bigfish_1418ee140", fn_1418ee140 as *const ()), // bigfish::save_game::SaveGameHeader::ClassTypeInfoImplementation::bigfish::save_game::SaveGameHeader::ClassTypeInfoImplem
    Replacement::draft(0x1418ee1a0, "bigfish_1418ee1a0", fn_1418ee1a0 as *const ()), // bigfish::save_game::SaveGameHeader::ClassTypeInfoImplementation::bigfish::save_game::SaveGameHeader::ClassTypeInfoImplem
    Replacement::draft(0x1418ee230, "bigfish_1418ee230", fn_1418ee230 as *const ()), // bigfish::save_game::SaveGameHeader::ClassTypeInfoImplementation::bigfish::save_game::SaveGameHeader::ClassTypeInfoImplem
    Replacement::draft_throws(0x1418ee330, "unknown_1418ee330", fn_1418ee330 as *const (), Throws::Caught), // sub_1418ee330
    Replacement::draft_throws(0x1418ee4d0, "unknown_1418ee4d0", fn_1418ee4d0 as *const (), Throws::Caught), // sub_1418ee4d0
    Replacement::draft(0x1418ef2c0, "unknown_1418ef2c0", fn_1418ef2c0 as *const ()), // sub_1418ef2c0
    Replacement::draft(0x1418ef310, "unknown_1418ef310", fn_1418ef310 as *const ()), // sub_1418ef310
    Replacement::draft(0x1418ef600, "unknown_1418ef600", fn_1418ef600 as *const ()), // sub_1418ef600
    Replacement::draft(0x1418ef630, "unknown_1418ef630", fn_1418ef630 as *const ()), // sub_1418ef630
    Replacement::draft(0x1418ef9c0, "unknown_1418ef9c0", fn_1418ef9c0 as *const ()), // sub_1418ef9c0
    Replacement::draft(0x1418ef9e0, "unknown_1418ef9e0", fn_1418ef9e0 as *const ()), // sub_1418ef9e0
    Replacement::draft_throws(0x1418efa70, "unknown_1418efa70", fn_1418efa70 as *const (), Throws::Caught), // sub_1418efa70
    Replacement::draft(0x1418effc0, "unknown_1418effc0", fn_1418effc0 as *const ()), // sub_1418effc0
    Replacement::draft(0x1418f07e0, "unknown_1418f07e0", tpl_34994::<0x143c93884, 0x14387a510, 0x14387a508, 0x141834a78, 0x1425de780, 0x141834d18, 0x141834a0c> as *const ()), // sub_1418f07e0
    Replacement::draft(0x1418f0850, "unknown_1418f0850", tpl_35252 as *const ()), // sub_1418f0850
    Replacement::draft(0x1418f0870, "std_1418f0870", tpl_550::<0x1430b8cb0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rigidbodyfactory::initCoreGamePhysics::<lambda_1>,void,ecs::World<ecs::access::Definition<r::Ty
    Replacement::draft(0x1418f0890, "std_1418f0890", tpl_550::<0x1430b8cb0, 0x8> as *const ()), // std::_Func_impl_no_alloc<rigidbodyfactory::initCoreGamePhysics::<lambda_1>,void,ecs::World<ecs::access::Definition<r::Ty
    Replacement::draft(0x1418f08b0, "std_1418f08b0", fn_1418f08b0 as *const ()), // std::_Func_impl_no_alloc<rigidbodyfactory::initCoreGamePhysics::<lambda_1>,void,ecs::World<ecs::access::Definition<r::Ty
    Replacement::draft(0x1418f09c0, "std_1418f09c0", tpl_1::<0x143886690> as *const ()), // std::_Func_impl_no_alloc<rigidbodyfactory::initCoreGamePhysics::<lambda_1>,void,ecs::World<ecs::access::Definition<r::Ty
    Replacement::draft(0x1418f09d0, "std_1418f09d0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<rigidbodyfactory::initCoreGamePhysics::<lambda_1>,void,ecs::World<ecs::access::Definition<r::Ty
    Replacement::draft(0x1418f09e0, "std_1418f09e0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<rigidbodyfactory::initCoreGamePhysics::<lambda_1>,void,ecs::World<ecs::access::Definition<r::Ty
    Replacement::draft(0x1418f0a00, "unknown_1418f0a00", tpl_2966::<0x8, 0x140349a60> as *const ()), // sub_1418f0a00
    Replacement::draft(0x1418f0a50, "unknown_1418f0a50", tpl_35253 as *const ()), // sub_1418f0a50
    Replacement::draft(0x1418f0a60, "unknown_1418f0a60", fn_1418f0a60 as *const ()), // sub_1418f0a60
    Replacement::draft(0x1418f0b70, "unknown_1418f0b70", fn_1418f0b70 as *const ()), // sub_1418f0b70
    Replacement::draft_throws(0x1418f0d90, "unknown_1418f0d90", fn_1418f0d90 as *const (), Throws::Discard), // sub_1418f0d90
    Replacement::draft_throws(0x1418f0f40, "unknown_1418f0f40", fn_1418f0f40 as *const (), Throws::Discard), // sub_1418f0f40
    Replacement::draft_throws(0x1418f1110, "nlohmann_1418f1110", fn_1418f1110 as *const (), Throws::Discard), // sub_1418f1110
    Replacement::draft(0x1418f1690, "rigidbodyfactory_1418f1690", fn_1418f1690 as *const ()), // RigidBodyFactory::RigidBodyFactory::vfunc_2
    Replacement::draft(0x1418f1760, "rigidbodyfactory_1418f1760", fn_1418f1760 as *const ()), // RigidBodyFactory::RigidBodyFactory::vfunc_6
    Replacement::draft(0x1418f1800, "rigidbodyfactory_1418f1800", fn_1418f1800 as *const ()), // RigidBodyFactory::RigidBodyFactory::vfunc_1
    Replacement::draft(0x1418f1a30, "rigidbodyfactory_1418f1a30", fn_1418f1a30 as *const ()), // RigidBodyFactory::RigidBodyFactory::vfunc_5
    Replacement::draft(0x1418f1a70, "rigidbodyfactory_1418f1a70", fn_1418f1a70 as *const ()), // RigidBodyFactory::RigidBodyFactory::vfunc_4
    Replacement::draft(0x1418f1d50, "unknown_1418f1d50", fn_1418f1d50 as *const ()), // sub_1418f1d50
    Replacement::draft(0x1418f1d70, "unknown_1418f1d70", fn_1418f1d70 as *const ()), // sub_1418f1d70
    Replacement::draft_throws(0x1418f1de0, "unknown_1418f1de0", fn_1418f1de0 as *const (), Throws::Discard), // sub_1418f1de0
    Replacement::draft(0x1418f2420, "unknown_1418f2420", fn_1418f2420 as *const ()), // sub_1418f2420
    Replacement::draft(0x1418f24e0, "unknown_1418f24e0", fn_1418f24e0 as *const ()), // sub_1418f24e0
    Replacement::draft(0x1418f2520, "unknown_1418f2520", tpl_35266::<0x8, 0x1401f2bc0> as *const ()), // sub_1418f2520
    Replacement::draft(0x1418f27e0, "unknown_1418f27e0", tpl_35267::<0x4, 0xc, 0x8> as *const ()), // sub_1418f27e0
    Replacement::draft(0x1418f3940, "unknown_1418f3940", tpl_35273::<0x4, 0xc, 0x8> as *const ()), // sub_1418f3940
    Replacement::draft(0x1418f3980, "unknown_1418f3980", tpl_35274::<0x4, 0xc, 0x8> as *const ()), // sub_1418f3980
    Replacement::draft_throws(0x1418f39c0, "unknown_1418f39c0", fn_1418f39c0 as *const (), Throws::Caught), // sub_1418f39c0
    Replacement::draft_throws(0x1418f3b40, "unknown_1418f3b40", fn_1418f3b40 as *const (), Throws::Discard), // sub_1418f3b40
    Replacement::draft(0x1418f3ba0, "unknown_1418f3ba0", fn_1418f3ba0 as *const ()), // sub_1418f3ba0
    Replacement::draft(0x1418f3c40, "unknown_1418f3c40", fn_1418f3c40 as *const ()), // sub_1418f3c40
    Replacement::draft_throws(0x1418f3c70, "unknown_1418f3c70", fn_1418f3c70 as *const (), Throws::Caught), // sub_1418f3c70
    Replacement::draft_throws(0x1418f40a0, "unknown_1418f40a0", fn_1418f40a0 as *const (), Throws::Caught), // sub_1418f40a0
    Replacement::draft_throws(0x1418f4450, "unknown_1418f4450", fn_1418f4450 as *const (), Throws::Caught), // sub_1418f4450
    Replacement::draft_throws(0x1418f4a30, "unknown_1418f4a30", fn_1418f4a30 as *const (), Throws::Discard), // sub_1418f4a30
    Replacement::draft_throws(0x1418f4b80, "unknown_1418f4b80", fn_1418f4b80 as *const (), Throws::Caught), // sub_1418f4b80
    Replacement::draft(0x1418f5640, "unknown_1418f5640", tpl_35273::<0x4, 0xc, 0x8> as *const ()), // sub_1418f5640
    Replacement::draft(0x1418f5680, "unknown_1418f5680", tpl_35274::<0x4, 0xc, 0x8> as *const ()), // sub_1418f5680
    Replacement::draft_throws(0x1418f56c0, "unknown_1418f56c0", fn_1418f56c0 as *const (), Throws::Caught), // sub_1418f56c0
    Replacement::draft(0x1418f5b10, "unknown_1418f5b10", fn_1418f5b10 as *const ()), // sub_1418f5b10
    Replacement::draft_throws(0x1418f5b80, "unknown_1418f5b80", fn_1418f5b80 as *const (), Throws::Discard), // sub_1418f5b80
    Replacement::draft(0x1418f5f40, "unknown_1418f5f40", tpl_34835::<0x8, 0x140357750, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418f5f40
    Replacement::draft(0x1418f5fc0, "unknown_1418f5fc0", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_1418f5fc0
    Replacement::draft(0x1418f6040, "unknown_1418f6040", tpl_34837::<0x140357750, 0x20, 0x14240aa00> as *const ()), // sub_1418f6040
    Replacement::draft(0x1418f6090, "unknown_1418f6090", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_1418f6090
    Replacement::draft(0x1418f60e0, "unknown_1418f60e0", tpl_34840::<0x1403573c0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1418f60e0
    Replacement::draft(0x1418f6150, "unknown_1418f6150", tpl_34840::<0x1403575b0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_1418f6150
    Replacement::draft_throws(0x1418f6540, "unknown_1418f6540", tpl_94836::<0x1402845f0, 0x14270b48c, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_1418f6540
    Replacement::draft_throws(0x1418f6670, "unknown_1418f6670", fn_1418f6670 as *const (), Throws::Discard), // sub_1418f6670
    Replacement::draft_throws(0x1418f6c10, "unknown_1418f6c10", fn_1418f6c10 as *const (), Throws::Caught), // sub_1418f6c10
    Replacement::draft_throws(0x1418f6e10, "unknown_1418f6e10", tpl_35065::<0x18, 0x1418f6540, 0x10> as *const (), Throws::Discard), // sub_1418f6e10
    Replacement::draft_throws(0x1418f6e50, "unknown_1418f6e50", tpl_34846::<0x1401f3a20, 0x1418f6540> as *const (), Throws::Discard), // sub_1418f6e50
    Replacement::draft_throws(0x1418f6ed0, "unknown_1418f6ed0", fn_1418f6ed0 as *const (), Throws::Caught), // sub_1418f6ed0
    Replacement::draft_throws(0x1418f70d0, "unknown_1418f70d0", fn_1418f70d0 as *const (), Throws::Discard), // sub_1418f70d0
    Replacement::draft_throws(0x1418f7110, "unknown_1418f7110", fn_1418f7110 as *const (), Throws::Discard), // sub_1418f7110
    Replacement::draft(0x1418f71d0, "unknown_1418f71d0", fn_1418f71d0 as *const ()), // sub_1418f71d0
    Replacement::draft_throws(0x1418f7250, "unknown_1418f7250", fn_1418f7250 as *const (), Throws::Caught), // sub_1418f7250
    Replacement::draft_throws(0x1418f79d0, "unknown_1418f79d0", fn_1418f79d0 as *const (), Throws::Discard), // sub_1418f79d0
    Replacement::draft_throws(0x1418f7eb0, "unknown_1418f7eb0", fn_1418f7eb0 as *const (), Throws::Discard), // sub_1418f7eb0
    Replacement::draft(0x1418f7f40, "unknown_1418f7f40", fn_1418f7f40 as *const ()), // sub_1418f7f40
    Replacement::draft_throws(0x1418f7f50, "unknown_1418f7f50", fn_1418f7f50 as *const (), Throws::Discard), // sub_1418f7f50
    Replacement::draft_throws(0x1418f80f0, "unknown_1418f80f0", fn_1418f80f0 as *const (), Throws::Caught), // sub_1418f80f0
    Replacement::draft(0x1418f87a0, "unknown_1418f87a0", fn_1418f87a0 as *const ()), // sub_1418f87a0
    Replacement::draft(0x1418f89d0, "unknown_1418f89d0", fn_1418f89d0 as *const ()), // sub_1418f89d0
    Replacement::draft_throws(0x1418f8b40, "unknown_1418f8b40", fn_1418f8b40 as *const (), Throws::Caught), // sub_1418f8b40
    Replacement::draft(0x1418f9560, "unknown_1418f9560", fn_1418f9560 as *const ()), // sub_1418f9560
    Replacement::draft(0x1418f9620, "unknown_1418f9620", fn_1418f9620 as *const ()), // sub_1418f9620
    Replacement::draft_throws(0x1418f96b0, "unknown_1418f96b0", fn_1418f96b0 as *const (), Throws::Discard), // sub_1418f96b0
    Replacement::draft_throws(0x1418f9880, "unknown_1418f9880", fn_1418f9880 as *const (), Throws::Discard), // sub_1418f9880
    Replacement::draft(0x1418f9c50, "unknown_1418f9c50", fn_1418f9c50 as *const ()), // sub_1418f9c50
    Replacement::draft(0x1418f9e10, "unknown_1418f9e10", fn_1418f9e10 as *const ()), // sub_1418f9e10
    Replacement::draft(0x1418f9e50, "unknown_1418f9e50", fn_1418f9e50 as *const ()), // sub_1418f9e50
    Replacement::draft(0x1418f9fb0, "unknown_1418f9fb0", fn_1418f9fb0 as *const ()), // sub_1418f9fb0
    Replacement::draft(0x1418fa2a0, "unknown_1418fa2a0", fn_1418fa2a0 as *const ()), // sub_1418fa2a0
    Replacement::draft(0x1418fa510, "unknown_1418fa510", fn_1418fa510 as *const ()), // sub_1418fa510
    Replacement::draft(0x1418fa750, "unknown_1418fa750", fn_1418fa750 as *const ()), // sub_1418fa750
    Replacement::draft(0x1418fa770, "unknown_1418fa770", fn_1418fa770 as *const ()), // sub_1418fa770
    Replacement::draft(0x1418fa7e0, "unknown_1418fa7e0", fn_1418fa7e0 as *const ()), // sub_1418fa7e0
    Replacement::draft(0x1418fa9d0, "unknown_1418fa9d0", fn_1418fa9d0 as *const ()), // sub_1418fa9d0
    Replacement::draft(0x1418faa30, "unknown_1418faa30", fn_1418faa30 as *const ()), // sub_1418faa30
    Replacement::draft(0x1418faa70, "unknown_1418faa70", fn_1418faa70 as *const ()), // sub_1418faa70
];
