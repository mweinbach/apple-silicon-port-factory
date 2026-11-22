#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x141ad3880, "unknown_141ad3880", fn_141ad3880 as *const ()), // sub_141ad3880
    Replacement::draft(0x141ad3960, "unknown_141ad3960", fn_141ad3960 as *const ()), // sub_141ad3960
    Replacement::draft(0x141ad3b20, "unknown_141ad3b20", fn_141ad3b20 as *const ()), // sub_141ad3b20
    Replacement::draft(0x141ad4390, "unknown_141ad4390", fn_141ad4390 as *const ()), // sub_141ad4390
    Replacement::draft(0x141ad45d0, "unknown_141ad45d0", fn_141ad45d0 as *const ()), // sub_141ad45d0
    Replacement::draft(0x141ad4780, "unknown_141ad4780", fn_141ad4780 as *const ()), // sub_141ad4780
    Replacement::draft(0x141ad4800, "unknown_141ad4800", tpl_361 as *const ()), // sub_141ad4800
    Replacement::draft(0x141ad4810, "unknown_141ad4810", fn_141ad4810 as *const ()), // sub_141ad4810
    Replacement::draft(0x141ad48a0, "unknown_141ad48a0", tpl_369 as *const ()), // sub_141ad48a0
    Replacement::draft(0x141ad48b0, "unknown_141ad48b0", tpl_361 as *const ()), // sub_141ad48b0
    Replacement::draft(0x141ad48c0, "unknown_141ad48c0", fn_141ad48c0 as *const ()), // sub_141ad48c0
    Replacement::draft(0x141ad4ce0, "unknown_141ad4ce0", tpl_361 as *const ()), // sub_141ad4ce0
    Replacement::draft_throws(0x141ad4cf0, "unknown_141ad4cf0", fn_141ad4cf0 as *const (), Throws::Discard), // sub_141ad4cf0
    Replacement::draft(0x141ad4ea0, "unknown_141ad4ea0", fn_141ad4ea0 as *const ()), // sub_141ad4ea0
    Replacement::draft_throws(0x141ad5070, "unknown_141ad5070", fn_141ad5070 as *const (), Throws::Discard), // sub_141ad5070
    Replacement::draft(0x141ad53a0, "unknown_141ad53a0", tpl_361 as *const ()), // sub_141ad53a0
    Replacement::draft_throws(0x141ad53b0, "unknown_141ad53b0", fn_141ad53b0 as *const (), Throws::Caught), // sub_141ad53b0
    Replacement::draft(0x141ad5f10, "unknown_141ad5f10", tpl_361 as *const ()), // sub_141ad5f10
    Replacement::draft_throws(0x141ad5f20, "unknown_141ad5f20", fn_141ad5f20 as *const (), Throws::Discard), // sub_141ad5f20
    Replacement::draft(0x141ad66c0, "unknown_141ad66c0", fn_141ad66c0 as *const ()), // sub_141ad66c0
    Replacement::draft(0x141ad6840, "unknown_141ad6840", fn_141ad6840 as *const ()), // sub_141ad6840
    Replacement::draft(0x141ad6850, "unknown_141ad6850", fn_141ad6850 as *const ()), // sub_141ad6850
    Replacement::draft(0x141ad6980, "unknown_141ad6980", fn_141ad6980 as *const ()), // sub_141ad6980
    Replacement::draft_throws(0x141ad6ae0, "unknown_141ad6ae0", fn_141ad6ae0 as *const (), Throws::Caught), // sub_141ad6ae0
    Replacement::draft(0x141ad7730, "unknown_141ad7730", fn_141ad7730 as *const ()), // sub_141ad7730
    Replacement::draft(0x141ad7750, "unknown_141ad7750", fn_141ad7750 as *const ()), // sub_141ad7750
    Replacement::draft(0x141ad7770, "unknown_141ad7770", fn_141ad7770 as *const ()), // sub_141ad7770
    Replacement::draft(0x141ad7790, "unknown_141ad7790", fn_141ad7790 as *const ()), // sub_141ad7790
    Replacement::draft(0x141ad7820, "unknown_141ad7820", fn_141ad7820 as *const ()), // sub_141ad7820
    Replacement::draft(0x141ad7870, "unknown_141ad7870", tpl_31123::<0x458> as *const ()), // sub_141ad7870
    Replacement::draft(0x141ad7880, "unknown_141ad7880", tpl_361 as *const ()), // sub_141ad7880
    Replacement::draft(0x141ad7890, "unknown_141ad7890", tpl_361 as *const ()), // sub_141ad7890
    Replacement::draft(0x141ad78a0, "unknown_141ad78a0", fn_141ad78a0 as *const ()), // sub_141ad78a0
    Replacement::draft(0x141ad7cd0, "unknown_141ad7cd0", tpl_35253 as *const ()), // sub_141ad7cd0
    Replacement::draft(0x141ad7ce0, "unknown_141ad7ce0", tpl_37287::<0x50> as *const ()), // sub_141ad7ce0
    Replacement::draft(0x141ad7d00, "unknown_141ad7d00", fn_141ad7d00 as *const ()), // sub_141ad7d00
    Replacement::draft(0x141ad7e10, "unknown_141ad7e10", tpl_361 as *const ()), // sub_141ad7e10
    Replacement::draft(0x141ad7e20, "unknown_141ad7e20", fn_141ad7e20 as *const ()), // sub_141ad7e20
    Replacement::draft(0x141ad7fe0, "unknown_141ad7fe0", tpl_4544::<0x60> as *const ()), // sub_141ad7fe0
    Replacement::draft(0x141ad7ff0, "unknown_141ad7ff0", fn_141ad7ff0 as *const ()), // sub_141ad7ff0
    Replacement::draft(0x141ad8030, "unknown_141ad8030", tpl_37291::<0x4> as *const ()), // sub_141ad8030
    Replacement::draft(0x141ad8080, "unknown_141ad8080", fn_141ad8080 as *const ()), // sub_141ad8080
    Replacement::draft(0x141ad8140, "unknown_141ad8140", fn_141ad8140 as *const ()), // sub_141ad8140
    Replacement::draft(0x141ad81a0, "unknown_141ad81a0", fn_141ad81a0 as *const ()), // sub_141ad81a0
    Replacement::draft(0x141ad8360, "unknown_141ad8360", fn_141ad8360 as *const ()), // sub_141ad8360
    Replacement::draft(0x141ad8390, "unknown_141ad8390", tpl_37297::<0x141e234c0, 0x141e22a40> as *const ()), // sub_141ad8390
    Replacement::draft(0x141ad84c0, "unknown_141ad84c0", tpl_37297::<0x141e23170, 0x141e22e20> as *const ()), // sub_141ad84c0
    Replacement::draft(0x141ad85f0, "unknown_141ad85f0", fn_141ad85f0 as *const ()), // sub_141ad85f0
    Replacement::draft(0x141ad8760, "unknown_141ad8760", fn_141ad8760 as *const ()), // sub_141ad8760
    Replacement::draft(0x141ad8830, "unknown_141ad8830", fn_141ad8830 as *const ()), // sub_141ad8830
    Replacement::draft(0x141ad8890, "unknown_141ad8890", fn_141ad8890 as *const ()), // sub_141ad8890
    Replacement::draft(0x141ad88e0, "unknown_141ad88e0", fn_141ad88e0 as *const ()), // sub_141ad88e0
    Replacement::draft(0x141ad8a90, "unknown_141ad8a90", fn_141ad8a90 as *const ()), // sub_141ad8a90
    Replacement::draft_throws(0x141ad8ae0, "unknown_141ad8ae0", fn_141ad8ae0 as *const (), Throws::Discard), // sub_141ad8ae0
    Replacement::draft_throws(0x141ad8bc0, "unknown_141ad8bc0", fn_141ad8bc0 as *const (), Throws::Discard), // sub_141ad8bc0
    Replacement::draft_throws(0x141ad91e0, "unknown_141ad91e0", fn_141ad91e0 as *const (), Throws::Discard), // sub_141ad91e0
    Replacement::draft_throws(0x141ad9370, "unknown_141ad9370", fn_141ad9370 as *const (), Throws::Discard), // sub_141ad9370
    Replacement::draft(0x141ad9470, "unknown_141ad9470", fn_141ad9470 as *const ()), // sub_141ad9470
    Replacement::draft_throws(0x141ad9740, "unknown_141ad9740", fn_141ad9740 as *const (), Throws::Discard), // sub_141ad9740
    Replacement::draft_throws(0x141ad98e0, "unknown_141ad98e0", fn_141ad98e0 as *const (), Throws::Discard), // sub_141ad98e0
    Replacement::draft_throws(0x141ad9ae0, "unknown_141ad9ae0", fn_141ad9ae0 as *const (), Throws::Discard), // sub_141ad9ae0
    Replacement::draft_throws(0x141ad9c40, "unknown_141ad9c40", fn_141ad9c40 as *const (), Throws::Discard), // sub_141ad9c40
    Replacement::draft(0x141ad9d00, "unknown_141ad9d00", fn_141ad9d00 as *const ()), // sub_141ad9d00
    Replacement::draft(0x141ad9db0, "unknown_141ad9db0", fn_141ad9db0 as *const ()), // sub_141ad9db0
    Replacement::draft(0x141ad9e00, "unknown_141ad9e00", fn_141ad9e00 as *const ()), // sub_141ad9e00
    Replacement::draft(0x141ad9fd0, "unknown_141ad9fd0", fn_141ad9fd0 as *const ()), // sub_141ad9fd0
    Replacement::draft(0x141ada190, "unknown_141ada190", fn_141ada190 as *const ()), // sub_141ada190
    Replacement::draft(0x141ada2b0, "unknown_141ada2b0", fn_141ada2b0 as *const ()), // sub_141ada2b0
    Replacement::draft(0x141ada3d0, "unknown_141ada3d0", fn_141ada3d0 as *const ()), // sub_141ada3d0
    Replacement::draft(0x141ada420, "unknown_141ada420", fn_141ada420 as *const ()), // sub_141ada420
    Replacement::draft_throws(0x141ada550, "unknown_141ada550", fn_141ada550 as *const (), Throws::Discard), // sub_141ada550
    Replacement::draft(0x141ada630, "unknown_141ada630", fn_141ada630 as *const ()), // sub_141ada630
    Replacement::draft(0x141ada870, "unknown_141ada870", tpl_37318::<0x306, 0xc8, 0x14058f540> as *const ()), // sub_141ada870
    Replacement::draft(0x141ada8d0, "unknown_141ada8d0", tpl_37319::<0x738> as *const ()), // sub_141ada8d0
    Replacement::draft(0x141ada8f0, "unknown_141ada8f0", fn_141ada8f0 as *const ()), // sub_141ada8f0
    Replacement::draft(0x141ada960, "unknown_141ada960", fn_141ada960 as *const ()), // sub_141ada960
    Replacement::draft(0x141ada980, "unknown_141ada980", fn_141ada980 as *const ()), // sub_141ada980
    Replacement::draft_throws(0x141ada9a0, "unknown_141ada9a0", fn_141ada9a0 as *const (), Throws::Discard), // sub_141ada9a0
    Replacement::draft_throws(0x141adac50, "unknown_141adac50", fn_141adac50 as *const (), Throws::Discard), // sub_141adac50
    Replacement::draft(0x141adb600, "unknown_141adb600", fn_141adb600 as *const ()), // sub_141adb600
    Replacement::draft(0x141adb650, "unknown_141adb650", tpl_34443 as *const ()), // sub_141adb650
    Replacement::draft(0x141adb660, "unknown_141adb660", fn_141adb660 as *const ()), // sub_141adb660
    Replacement::draft(0x141adb690, "unknown_141adb690", tpl_34443 as *const ()), // sub_141adb690
    Replacement::draft(0x141adb6a0, "unknown_141adb6a0", fn_141adb6a0 as *const ()), // sub_141adb6a0
    Replacement::draft(0x141adb6f0, "unknown_141adb6f0", tpl_37327::<0x57c> as *const ()), // sub_141adb6f0
    Replacement::draft(0x141adb710, "unknown_141adb710", tpl_37328::<0x305> as *const ()), // sub_141adb710
    Replacement::draft(0x141adb720, "unknown_141adb720", tpl_37328::<0x304> as *const ()), // sub_141adb720
    Replacement::draft(0x141adb730, "unknown_141adb730", tpl_37328::<0x303> as *const ()), // sub_141adb730
    Replacement::draft(0x141adb740, "unknown_141adb740", tpl_37327::<0x3c0> as *const ()), // sub_141adb740
    Replacement::draft(0x141adb760, "unknown_141adb760", tpl_37328::<0x301> as *const ()), // sub_141adb760
    Replacement::draft(0x141adb770, "unknown_141adb770", tpl_37328::<0x300> as *const ()), // sub_141adb770
    Replacement::draft_throws(0x141adb780, "unknown_141adb780", fn_141adb780 as *const (), Throws::Discard), // sub_141adb780
    Replacement::draft_throws(0x141adb930, "unknown_141adb930", fn_141adb930 as *const (), Throws::Discard), // sub_141adb930
    Replacement::draft_throws(0x141adba00, "unknown_141adba00", fn_141adba00 as *const (), Throws::Discard), // sub_141adba00
    Replacement::draft(0x141adbac0, "unknown_141adbac0", fn_141adbac0 as *const ()), // sub_141adbac0
    Replacement::draft(0x141adbb20, "unknown_141adbb20", fn_141adbb20 as *const ()), // sub_141adbb20
    Replacement::draft(0x141adbb50, "unknown_141adbb50", fn_141adbb50 as *const ()), // sub_141adbb50
    Replacement::draft(0x141adbc80, "unknown_141adbc80", fn_141adbc80 as *const ()), // sub_141adbc80
    Replacement::draft(0x141adbd00, "unknown_141adbd00", fn_141adbd00 as *const ()), // sub_141adbd00
    Replacement::draft_throws(0x141adbd50, "unknown_141adbd50", fn_141adbd50 as *const (), Throws::Discard), // sub_141adbd50
    Replacement::draft(0x141adc5a0, "unknown_141adc5a0", fn_141adc5a0 as *const ()), // sub_141adc5a0
    Replacement::draft_throws(0x141adc6d0, "unknown_141adc6d0", fn_141adc6d0 as *const (), Throws::Discard), // sub_141adc6d0
    Replacement::draft(0x141adc8b0, "unknown_141adc8b0", fn_141adc8b0 as *const ()), // sub_141adc8b0
    Replacement::draft(0x141adcb20, "unknown_141adcb20", fn_141adcb20 as *const ()), // sub_141adcb20
    Replacement::draft(0x141adcc40, "unknown_141adcc40", fn_141adcc40 as *const ()), // sub_141adcc40
    Replacement::draft(0x141adcc50, "unknown_141adcc50", fn_141adcc50 as *const ()), // sub_141adcc50
    Replacement::draft(0x141adcfc0, "unknown_141adcfc0", fn_141adcfc0 as *const ()), // sub_141adcfc0
    Replacement::draft(0x141adcfe0, "unknown_141adcfe0", fn_141adcfe0 as *const ()), // sub_141adcfe0
    Replacement::draft(0x141add220, "unknown_141add220", fn_141add220 as *const ()), // sub_141add220
    Replacement::draft(0x141add230, "unknown_141add230", fn_141add230 as *const ()), // sub_141add230
    Replacement::draft(0x141add270, "unknown_141add270", fn_141add270 as *const ()), // sub_141add270
    Replacement::draft(0x141add450, "unknown_141add450", fn_141add450 as *const ()), // sub_141add450
    Replacement::draft_throws(0x141add5d0, "unknown_141add5d0", fn_141add5d0 as *const (), Throws::Discard), // sub_141add5d0
    Replacement::draft(0x141add620, "unknown_141add620", fn_141add620 as *const ()), // sub_141add620
    Replacement::draft(0x141add6b0, "unknown_141add6b0", fn_141add6b0 as *const ()), // sub_141add6b0
    Replacement::draft(0x141adda20, "unknown_141adda20", fn_141adda20 as *const ()), // sub_141adda20
    Replacement::draft(0x141addc50, "unknown_141addc50", fn_141addc50 as *const ()), // sub_141addc50
    Replacement::draft_throws(0x141addc70, "unknown_141addc70", fn_141addc70 as *const (), Throws::Discard), // sub_141addc70
    Replacement::draft(0x141ade140, "unknown_141ade140", tpl_37318::<0x30a, 0xc8, 0x14058f540> as *const ()), // sub_141ade140
    Replacement::draft(0x141ade300, "unknown_141ade300", fn_141ade300 as *const ()), // sub_141ade300
    Replacement::draft(0x141ade4f0, "unknown_141ade4f0", fn_141ade4f0 as *const ()), // sub_141ade4f0
    Replacement::draft(0x141ade5b0, "unknown_141ade5b0", fn_141ade5b0 as *const ()), // sub_141ade5b0
    Replacement::draft(0x141ade6e0, "unknown_141ade6e0", fn_141ade6e0 as *const ()), // sub_141ade6e0
    Replacement::draft(0x141ade810, "unknown_141ade810", fn_141ade810 as *const ()), // sub_141ade810
    Replacement::draft_throws(0x141adebd0, "unknown_141adebd0", fn_141adebd0 as *const (), Throws::Caught), // sub_141adebd0
    Replacement::draft(0x141adfa60, "unknown_141adfa60", tpl_34994::<0x143cb6468, 0x14387bd80, 0x14387bd78, 0x141834a78, 0x1425f4910, 0x141834d18, 0x141834a0c> as *const ()), // sub_141adfa60
    Replacement::draft(0x141adfad0, "unknown_141adfad0", tpl_369 as *const ()), // sub_141adfad0
    Replacement::draft(0x141adfae0, "unknown_141adfae0", tpl_34994::<0x143cb68c8, 0x14387bd70, 0x14387bd68, 0x141834a78, 0x1425f4920, 0x141834d18, 0x141834a0c> as *const ()), // sub_141adfae0
    Replacement::draft(0x141adfb50, "unknown_141adfb50", tpl_35332 as *const ()), // sub_141adfb50
    Replacement::draft(0x141adfb60, "unknown_141adfb60", tpl_34994::<0x143cb67c8, 0x14387bd60, 0x14387bd58, 0x141834a78, 0x1425f4930, 0x141834d18, 0x141834a0c> as *const ()), // sub_141adfb60
    Replacement::draft(0x141adfbd0, "unknown_141adfbd0", tpl_369 as *const ()), // sub_141adfbd0
    Replacement::draft(0x141adfbe0, "unknown_141adfbe0", tpl_34318 as *const ()), // sub_141adfbe0
    Replacement::draft(0x141adfc00, "unknown_141adfc00", tpl_34319::<0x143cb6248, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141adfc00
    Replacement::draft(0x141adfc60, "unknown_141adfc60", tpl_34318 as *const ()), // sub_141adfc60
    Replacement::draft(0x141adfc80, "unknown_141adfc80", tpl_34319::<0x143cb5940, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141adfc80
    Replacement::draft(0x141adfce0, "unknown_141adfce0", tpl_34318 as *const ()), // sub_141adfce0
    Replacement::draft(0x141adfd00, "unknown_141adfd00", tpl_34318 as *const ()), // sub_141adfd00
    Replacement::draft(0x141adfd20, "unknown_141adfd20", tpl_34319::<0x143c8dbd4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141adfd20
    Replacement::draft(0x141adfd80, "unknown_141adfd80", tpl_34319::<0x143cb69f4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141adfd80
    Replacement::draft(0x141adfde0, "unknown_141adfde0", tpl_34318 as *const ()), // sub_141adfde0
    Replacement::draft(0x141adfe00, "unknown_141adfe00", tpl_34318 as *const ()), // sub_141adfe00
    Replacement::draft(0x141adfe20, "unknown_141adfe20", tpl_34318 as *const ()), // sub_141adfe20
    Replacement::draft(0x141adfe40, "unknown_141adfe40", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141adfe40
    Replacement::draft(0x141adfea0, "unknown_141adfea0", tpl_34319::<0x143cb5d00, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141adfea0
    Replacement::draft(0x141adff00, "unknown_141adff00", tpl_34319::<0x143cb56c0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141adff00
    Replacement::draft(0x141adff60, "unknown_141adff60", tpl_34318 as *const ()), // sub_141adff60
    Replacement::draft(0x141adff80, "unknown_141adff80", tpl_34318 as *const ()), // sub_141adff80
    Replacement::draft(0x141adffa0, "unknown_141adffa0", tpl_34318 as *const ()), // sub_141adffa0
    Replacement::draft(0x141adffc0, "unknown_141adffc0", tpl_34319::<0x143c933f0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141adffc0
    Replacement::draft(0x141ae0020, "unknown_141ae0020", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae0020
    Replacement::draft(0x141ae0080, "unknown_141ae0080", tpl_34319::<0x143c92bcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae0080
    Replacement::draft(0x141ae00e0, "unknown_141ae00e0", tpl_34318 as *const ()), // sub_141ae00e0
    Replacement::draft(0x141ae0100, "unknown_141ae0100", tpl_34318 as *const ()), // sub_141ae0100
    Replacement::draft(0x141ae0120, "unknown_141ae0120", tpl_34318 as *const ()), // sub_141ae0120
    Replacement::draft(0x141ae0140, "unknown_141ae0140", tpl_34318 as *const ()), // sub_141ae0140
    Replacement::draft(0x141ae0160, "unknown_141ae0160", tpl_34318 as *const ()), // sub_141ae0160
    Replacement::draft(0x141ae0180, "unknown_141ae0180", tpl_34318 as *const ()), // sub_141ae0180
    Replacement::draft(0x141ae01a0, "unknown_141ae01a0", tpl_34318 as *const ()), // sub_141ae01a0
    Replacement::draft(0x141ae01c0, "unknown_141ae01c0", tpl_34318 as *const ()), // sub_141ae01c0
    Replacement::draft(0x141ae01e0, "unknown_141ae01e0", tpl_34318 as *const ()), // sub_141ae01e0
    Replacement::draft(0x141ae0200, "unknown_141ae0200", tpl_34319::<0x143c9d1d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae0200
    Replacement::draft(0x141ae0260, "unknown_141ae0260", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae0260
    Replacement::draft(0x141ae02c0, "unknown_141ae02c0", tpl_34319::<0x143cb5358, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae02c0
    Replacement::draft(0x141ae0320, "unknown_141ae0320", tpl_34319::<0x143c9afe8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae0320
    Replacement::draft(0x141ae0380, "unknown_141ae0380", tpl_34319::<0x143c8e070, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae0380
    Replacement::draft(0x141ae03e0, "unknown_141ae03e0", tpl_34319::<0x143c8de70, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae03e0
    Replacement::draft(0x141ae0440, "unknown_141ae0440", tpl_34319::<0x143c8f6dc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae0440
    Replacement::draft(0x141ae04a0, "unknown_141ae04a0", tpl_34319::<0x143c900a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae04a0
    Replacement::draft(0x141ae0500, "unknown_141ae0500", tpl_34319::<0x143c8f7a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141ae0500
    Replacement::draft(0x141ae0560, "unknown_141ae0560", fn_141ae0560 as *const ()), // sub_141ae0560
    Replacement::draft(0x141ae0960, "unknown_141ae0960", tpl_361 as *const ()), // sub_141ae0960
    Replacement::draft(0x141ae0970, "unknown_141ae0970", tpl_361 as *const ()), // sub_141ae0970
    Replacement::draft(0x141ae0980, "unknown_141ae0980", fn_141ae0980 as *const ()), // sub_141ae0980
    Replacement::draft(0x141ae0c50, "unknown_141ae0c50", tpl_361 as *const ()), // sub_141ae0c50
    Replacement::draft(0x141ae0c60, "unknown_141ae0c60", tpl_361 as *const ()), // sub_141ae0c60
    Replacement::draft(0x141ae0c70, "unknown_141ae0c70", fn_141ae0c70 as *const ()), // sub_141ae0c70
    Replacement::draft(0x141ae1250, "unknown_141ae1250", tpl_361 as *const ()), // sub_141ae1250
    Replacement::draft(0x141ae1260, "unknown_141ae1260", tpl_361 as *const ()), // sub_141ae1260
    Replacement::draft(0x141ae1270, "unknown_141ae1270", fn_141ae1270 as *const ()), // sub_141ae1270
    Replacement::draft(0x141ae23f0, "unknown_141ae23f0", fn_141ae23f0 as *const ()), // sub_141ae23f0
    Replacement::draft(0x141ae2490, "unknown_141ae2490", tpl_361 as *const ()), // sub_141ae2490
    Replacement::draft_throws(0x141ae24a0, "unknown_141ae24a0", fn_141ae24a0 as *const (), Throws::Discard), // sub_141ae24a0
    Replacement::draft(0x141ae2560, "unknown_141ae2560", tpl_36673 as *const ()), // sub_141ae2560
    Replacement::draft(0x141ae2570, "unknown_141ae2570", fn_141ae2570 as *const ()), // sub_141ae2570
    Replacement::draft(0x141ae2590, "unknown_141ae2590", tpl_35132::<0x4> as *const ()), // sub_141ae2590
    Replacement::draft(0x141ae25b0, "unknown_141ae25b0", tpl_37356::<0x4> as *const ()), // sub_141ae25b0
    Replacement::draft_throws(0x141ae25d0, "unknown_141ae25d0", fn_141ae25d0 as *const (), Throws::Caught), // sub_141ae25d0
    Replacement::draft(0x141ae3830, "unknown_141ae3830", tpl_35132::<0x4> as *const ()), // sub_141ae3830
    Replacement::draft(0x141ae3850, "unknown_141ae3850", tpl_37356::<0x4> as *const ()), // sub_141ae3850
    Replacement::draft_throws(0x141ae3870, "unknown_141ae3870", fn_141ae3870 as *const (), Throws::Caught), // sub_141ae3870
    Replacement::draft(0x141ae43a0, "unknown_141ae43a0", tpl_361 as *const ()), // sub_141ae43a0
    Replacement::draft(0x141ae43b0, "unknown_141ae43b0", fn_141ae43b0 as *const ()), // sub_141ae43b0
    Replacement::draft_throws(0x141ae4410, "unknown_141ae4410", fn_141ae4410 as *const (), Throws::Discard), // sub_141ae4410
    Replacement::draft(0x141ae4740, "unknown_141ae4740", tpl_34835::<0x8, 0x140261110, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141ae4740
    Replacement::draft(0x141ae47c0, "unknown_141ae47c0", tpl_34837::<0x140261110, 0x20, 0x14240aa00> as *const ()), // sub_141ae47c0
    Replacement::draft_throws(0x141ae4810, "unknown_141ae4810", fn_141ae4810 as *const (), Throws::Caught), // sub_141ae4810
    Replacement::draft(0x141ae4a10, "unknown_141ae4a10", fn_141ae4a10 as *const ()), // sub_141ae4a10
    Replacement::draft_throws(0x141ae4a40, "unknown_141ae4a40", tpl_94650::<0x14025cb90, 0x14309c488, 0x14276cb80, 0x14276cb9c, 0x14276cb90, 0x140254aa0, 0x1424b16a0, 0x14025f490> as *const (), Throws::Discard), // sub_141ae4a40
    Replacement::draft_throws(0x141ae4c80, "unknown_141ae4c80", tpl_34894::<0x18, 0x141ae4a40, 0x10> as *const (), Throws::Discard), // sub_141ae4c80
    Replacement::draft_throws(0x141ae4cc0, "unknown_141ae4cc0", tpl_34846::<0x1401f3a20, 0x141ae4a40> as *const (), Throws::Discard), // sub_141ae4cc0
    Replacement::draft_throws(0x141ae4d40, "unknown_141ae4d40", fn_141ae4d40 as *const (), Throws::Caught), // sub_141ae4d40
    Replacement::draft(0x141ae5a10, "unknown_141ae5a10", tpl_37291::<0x4> as *const ()), // sub_141ae5a10
    Replacement::draft(0x141ae5a60, "unknown_141ae5a60", tpl_37360::<0x1431cf6d0, 0x1431cf720, 0x1431cf700, 0x1431cf758> as *const ()), // sub_141ae5a60
    Replacement::draft(0x141ae5aa0, "unknown_141ae5aa0", tpl_37360::<0x1431cf740, 0x1431cf698, 0x1431cf680, 0x1431cf6b0> as *const ()), // sub_141ae5aa0
    Replacement::draft(0x141ae5ae0, "unknown_141ae5ae0", fn_141ae5ae0 as *const ()), // sub_141ae5ae0
    Replacement::draft(0x141ae5b20, "unknown_141ae5b20", fn_141ae5b20 as *const ()), // sub_141ae5b20
    Replacement::draft(0x141ae5b80, "unknown_141ae5b80", fn_141ae5b80 as *const ()), // sub_141ae5b80
    Replacement::draft_throws(0x141ae5ba0, "unknown_141ae5ba0", fn_141ae5ba0 as *const (), Throws::Caught), // sub_141ae5ba0
    Replacement::draft(0x141ae5de0, "unknown_141ae5de0", tpl_34994::<0x143cb704c, 0x14387bd90, 0x14387bd88, 0x141834a78, 0x1425f4f30, 0x141834d18, 0x141834a0c> as *const ()), // sub_141ae5de0
    Replacement::draft(0x141ae5e50, "unknown_141ae5e50", tpl_95133::<0x1431d1720, 0x1, 0x8, 0x1405b4a20> as *const ()), // sub_141ae5e50
    Replacement::draft(0x141ae6780, "unknown_141ae6780", fn_141ae6780 as *const ()), // sub_141ae6780
    Replacement::draft_throws(0x141ae67a0, "unknown_141ae67a0", fn_141ae67a0 as *const (), Throws::Discard), // sub_141ae67a0
    Replacement::draft_throws(0x141ae6b60, "unknown_141ae6b60", fn_141ae6b60 as *const (), Throws::Caught), // sub_141ae6b60
    Replacement::draft(0x141ae80b0, "unknown_141ae80b0", fn_141ae80b0 as *const ()), // sub_141ae80b0
    Replacement::draft(0x141ae82c0, "unknown_141ae82c0", fn_141ae82c0 as *const ()), // sub_141ae82c0
    Replacement::draft(0x141ae8550, "unknown_141ae8550", fn_141ae8550 as *const ()), // sub_141ae8550
    Replacement::draft(0x141ae8670, "unknown_141ae8670", fn_141ae8670 as *const ()), // sub_141ae8670
    Replacement::draft(0x141ae8890, "unknown_141ae8890", fn_141ae8890 as *const ()), // sub_141ae8890
    Replacement::draft_throws(0x141ae9000, "unknown_141ae9000", fn_141ae9000 as *const (), Throws::Discard), // sub_141ae9000
    Replacement::draft(0x141ae94f0, "unknown_141ae94f0", tpl_37370 as *const ()), // sub_141ae94f0
    Replacement::draft(0x141ae9500, "unknown_141ae9500", tpl_37370 as *const ()), // sub_141ae9500
    Replacement::draft_throws(0x141ae9510, "unknown_141ae9510", fn_141ae9510 as *const (), Throws::Discard), // sub_141ae9510
    Replacement::draft(0x141aea270, "unknown_141aea270", fn_141aea270 as *const ()), // sub_141aea270
    Replacement::draft(0x141aea2f0, "unknown_141aea2f0", fn_141aea2f0 as *const ()), // sub_141aea2f0
    Replacement::draft(0x141aeaa10, "unknown_141aeaa10", fn_141aeaa10 as *const ()), // sub_141aeaa10
    Replacement::draft(0x141aeaab0, "unknown_141aeaab0", fn_141aeaab0 as *const ()), // sub_141aeaab0
    Replacement::draft(0x141aeab60, "unknown_141aeab60", fn_141aeab60 as *const ()), // sub_141aeab60
    Replacement::draft(0x141aeab90, "unknown_141aeab90", fn_141aeab90 as *const ()), // sub_141aeab90
    Replacement::draft(0x141aeade0, "unknown_141aeade0", fn_141aeade0 as *const ()), // sub_141aeade0
    Replacement::draft(0x141aeb430, "unknown_141aeb430", fn_141aeb430 as *const ()), // sub_141aeb430
    Replacement::draft(0x141aeb4e0, "unknown_141aeb4e0", fn_141aeb4e0 as *const ()), // sub_141aeb4e0
    Replacement::draft(0x141aebad0, "unknown_141aebad0", fn_141aebad0 as *const ()), // sub_141aebad0
    Replacement::draft(0x141aebb30, "unknown_141aebb30", fn_141aebb30 as *const ()), // sub_141aebb30
    Replacement::draft(0x141aebb60, "unknown_141aebb60", fn_141aebb60 as *const ()), // sub_141aebb60
    Replacement::draft_throws(0x141aebbf0, "unknown_141aebbf0", fn_141aebbf0 as *const (), Throws::Discard), // sub_141aebbf0
    Replacement::draft(0x141aebcc0, "unknown_141aebcc0", fn_141aebcc0 as *const ()), // sub_141aebcc0
    Replacement::draft_throws(0x141aebd50, "unknown_141aebd50", fn_141aebd50 as *const (), Throws::Discard), // sub_141aebd50
    Replacement::draft_throws(0x141aebf60, "unknown_141aebf60", fn_141aebf60 as *const (), Throws::Discard), // sub_141aebf60
    Replacement::draft(0x141aec190, "unknown_141aec190", fn_141aec190 as *const ()), // sub_141aec190
    Replacement::draft(0x141aec2d0, "unknown_141aec2d0", fn_141aec2d0 as *const ()), // sub_141aec2d0
    Replacement::draft_throws(0x141aec300, "unknown_141aec300", fn_141aec300 as *const (), Throws::Discard), // sub_141aec300
    Replacement::draft(0x141aec330, "unknown_141aec330", fn_141aec330 as *const ()), // sub_141aec330
    Replacement::draft(0x141aec3a0, "unknown_141aec3a0", fn_141aec3a0 as *const ()), // sub_141aec3a0
    Replacement::draft(0x141aec3f0, "unknown_141aec3f0", tpl_37236::<0x8, 0x3, 0x4, 0x1426d01e0> as *const ()), // sub_141aec3f0
    Replacement::draft(0x141aec480, "unknown_141aec480", fn_141aec480 as *const ()), // sub_141aec480
    Replacement::draft(0x141aec5b0, "unknown_141aec5b0", tpl_37236::<0x8, 0x3, 0x4, 0x1426d01e0> as *const ()), // sub_141aec5b0
    Replacement::draft_throws(0x141aec640, "unknown_141aec640", fn_141aec640 as *const (), Throws::Discard), // sub_141aec640
    Replacement::draft_throws(0x141aec730, "unknown_141aec730", fn_141aec730 as *const (), Throws::Discard), // sub_141aec730
    Replacement::draft(0x141aecb20, "unknown_141aecb20", fn_141aecb20 as *const ()), // sub_141aecb20
    Replacement::draft(0x141aecca0, "unknown_141aecca0", fn_141aecca0 as *const ()), // sub_141aecca0
    Replacement::draft(0x141aecec0, "unknown_141aecec0", fn_141aecec0 as *const ()), // sub_141aecec0
    Replacement::draft(0x141aecf90, "unknown_141aecf90", fn_141aecf90 as *const ()), // sub_141aecf90
    Replacement::draft_throws(0x141aed0b0, "unknown_141aed0b0", fn_141aed0b0 as *const (), Throws::Discard), // sub_141aed0b0
    Replacement::draft_throws(0x141aed180, "unknown_141aed180", fn_141aed180 as *const (), Throws::Discard), // sub_141aed180
    Replacement::draft(0x141aed820, "unknown_141aed820", fn_141aed820 as *const ()), // sub_141aed820
    Replacement::draft(0x141aed900, "unknown_141aed900", fn_141aed900 as *const ()), // sub_141aed900
    Replacement::draft(0x141aed9f0, "unknown_141aed9f0", fn_141aed9f0 as *const ()), // sub_141aed9f0
    Replacement::draft(0x141aedb00, "unknown_141aedb00", fn_141aedb00 as *const ()), // sub_141aedb00
    Replacement::draft(0x141aedbb0, "unknown_141aedbb0", fn_141aedbb0 as *const ()), // sub_141aedbb0
    Replacement::draft(0x141aedd80, "unknown_141aedd80", fn_141aedd80 as *const ()), // sub_141aedd80
    Replacement::draft(0x141aedef0, "unknown_141aedef0", fn_141aedef0 as *const ()), // sub_141aedef0
    Replacement::draft(0x141aee840, "unknown_141aee840", fn_141aee840 as *const ()), // sub_141aee840
    Replacement::draft_throws(0x141aee960, "unknown_141aee960", fn_141aee960 as *const (), Throws::Caught), // sub_141aee960
    Replacement::draft(0x141aef7a0, "unknown_141aef7a0", fn_141aef7a0 as *const ()), // sub_141aef7a0
    Replacement::draft(0x141aef830, "unknown_141aef830", tpl_37397::<0x18, 0x10, 0x58, 0x60, 0x143cb7a1c, 0x158, 0x8, 0x14020e8f0> as *const ()), // sub_141aef830
    Replacement::draft(0x141aef930, "unknown_141aef930", fn_141aef930 as *const ()), // sub_141aef930
    Replacement::draft(0x141aeff40, "unknown_141aeff40", tpl_37399::<0x60> as *const ()), // sub_141aeff40
    Replacement::draft_throws(0x141aeff60, "unknown_141aeff60", fn_141aeff60 as *const (), Throws::Caught), // sub_141aeff60
    Replacement::draft(0x141af0ab0, "unknown_141af0ab0", fn_141af0ab0 as *const ()), // sub_141af0ab0
    Replacement::draft_throws(0x141af0af0, "unknown_141af0af0", fn_141af0af0 as *const (), Throws::Discard), // sub_141af0af0
    Replacement::draft(0x141af0df0, "unknown_141af0df0", fn_141af0df0 as *const ()), // sub_141af0df0
    Replacement::draft(0x141af0ea0, "unknown_141af0ea0", tpl_37397::<0x18, 0x10, 0x58, 0x60, 0x143cb760c, 0x158, 0x8, 0x14020e8f0> as *const ()), // sub_141af0ea0
    Replacement::draft(0x141af0fa0, "unknown_141af0fa0", fn_141af0fa0 as *const ()), // sub_141af0fa0
    Replacement::draft_throws(0x141af10e0, "unknown_141af10e0", fn_141af10e0 as *const (), Throws::Discard), // sub_141af10e0
    Replacement::draft(0x141af1570, "unknown_141af1570", fn_141af1570 as *const ()), // sub_141af1570
    Replacement::draft(0x141af1a80, "unknown_141af1a80", fn_141af1a80 as *const ()), // sub_141af1a80
    Replacement::draft_throws(0x141af1ba0, "unknown_141af1ba0", fn_141af1ba0 as *const (), Throws::Caught), // sub_141af1ba0
    Replacement::draft(0x141af21e0, "unknown_141af21e0", fn_141af21e0 as *const ()), // sub_141af21e0
    Replacement::draft_throws(0x141af23e0, "unknown_141af23e0", fn_141af23e0 as *const (), Throws::Discard), // sub_141af23e0
    Replacement::draft_throws(0x141af2540, "unknown_141af2540", fn_141af2540 as *const (), Throws::Discard), // sub_141af2540
    Replacement::draft(0x141af25e0, "unknown_141af25e0", fn_141af25e0 as *const ()), // sub_141af25e0
    Replacement::draft(0x141af2660, "unknown_141af2660", fn_141af2660 as *const ()), // sub_141af2660
    Replacement::draft_throws(0x141af26b0, "unknown_141af26b0", fn_141af26b0 as *const (), Throws::Discard), // sub_141af26b0
    Replacement::draft(0x141af2920, "unknown_141af2920", fn_141af2920 as *const ()), // sub_141af2920
    Replacement::draft(0x141af2ff0, "unknown_141af2ff0", fn_141af2ff0 as *const ()), // sub_141af2ff0
    Replacement::draft(0x141af3a70, "unknown_141af3a70", fn_141af3a70 as *const ()), // sub_141af3a70
    Replacement::draft_throws(0x141af3b70, "unknown_141af3b70", fn_141af3b70 as *const (), Throws::Discard), // sub_141af3b70
    Replacement::draft(0x141af3c40, "unknown_141af3c40", fn_141af3c40 as *const ()), // sub_141af3c40
    Replacement::draft(0x141af52b0, "unknown_141af52b0", fn_141af52b0 as *const ()), // sub_141af52b0
    Replacement::draft(0x141af5450, "unknown_141af5450", fn_141af5450 as *const ()), // sub_141af5450
    Replacement::draft(0x141af5560, "unknown_141af5560", fn_141af5560 as *const ()), // sub_141af5560
    Replacement::draft(0x141af5d10, "unknown_141af5d10", tpl_1962::<0x8, 0x1405c0b00, 0x1405c0d00> as *const ()), // sub_141af5d10
    Replacement::draft_throws(0x141af5d90, "unknown_141af5d90", fn_141af5d90 as *const (), Throws::Discard), // sub_141af5d90
    Replacement::draft(0x141af8540, "unknown_141af8540", fn_141af8540 as *const ()), // sub_141af8540
    Replacement::draft(0x141af8800, "unknown_141af8800", fn_141af8800 as *const ()), // sub_141af8800
    Replacement::draft(0x141af8e20, "unknown_141af8e20", fn_141af8e20 as *const ()), // sub_141af8e20
    Replacement::draft_throws(0x141af90c0, "unknown_141af90c0", fn_141af90c0 as *const (), Throws::Caught), // sub_141af90c0
    Replacement::draft(0x141af9320, "unknown_141af9320", fn_141af9320 as *const ()), // sub_141af9320
    Replacement::draft(0x141af95a0, "unknown_141af95a0", fn_141af95a0 as *const ()), // sub_141af95a0
    Replacement::draft(0x141af9770, "unknown_141af9770", fn_141af9770 as *const ()), // sub_141af9770
    Replacement::draft(0x141af9860, "unknown_141af9860", fn_141af9860 as *const ()), // sub_141af9860
    Replacement::draft(0x141af98c0, "unknown_141af98c0", fn_141af98c0 as *const ()), // sub_141af98c0
    Replacement::draft_throws(0x141af9920, "unknown_141af9920", fn_141af9920 as *const (), Throws::Discard), // sub_141af9920
    Replacement::draft(0x141af9a30, "unknown_141af9a30", fn_141af9a30 as *const ()), // sub_141af9a30
    Replacement::draft(0x141af9a90, "unknown_141af9a90", fn_141af9a90 as *const ()), // sub_141af9a90
    Replacement::draft(0x141af9af0, "unknown_141af9af0", fn_141af9af0 as *const ()), // sub_141af9af0
    Replacement::draft(0x141af9b50, "unknown_141af9b50", fn_141af9b50 as *const ()), // sub_141af9b50
    Replacement::draft(0x141af9bf0, "unknown_141af9bf0", tpl_37425::<0x143cb5bc0, 0x20, 0x8, 0x1405bfa40, 0x28, 0x30, 0x38, 0x10d> as *const ()), // sub_141af9bf0
    Replacement::draft(0x141af9c90, "unknown_141af9c90", tpl_37426::<0x143cb5bc0, 0x8, 0x20, 0x1405bfa40, 0x28, 0x30, 0x38, 0x128> as *const ()), // sub_141af9c90
    Replacement::draft(0x141af9d20, "unknown_141af9d20", fn_141af9d20 as *const ()), // sub_141af9d20
    Replacement::draft(0x141af9db0, "unknown_141af9db0", tpl_37425::<0x143cb5bc0, 0x20, 0x8, 0x1405bfa40, 0x28, 0x30, 0x38, 0x10e> as *const ()), // sub_141af9db0
    Replacement::draft(0x141af9e50, "unknown_141af9e50", fn_141af9e50 as *const ()), // sub_141af9e50
    Replacement::draft(0x141af9fd0, "unknown_141af9fd0", tpl_37426::<0x143cb5bc0, 0x8, 0x20, 0x1405bfa40, 0x28, 0x30, 0x38, 0x10d> as *const ()), // sub_141af9fd0
    Replacement::draft(0x141afa060, "unknown_141afa060", fn_141afa060 as *const ()), // sub_141afa060
    Replacement::draft(0x141afa150, "unknown_141afa150", fn_141afa150 as *const ()), // sub_141afa150
    Replacement::draft(0x141afa240, "unknown_141afa240", fn_141afa240 as *const ()), // sub_141afa240
    Replacement::draft(0x141afa320, "unknown_141afa320", fn_141afa320 as *const ()), // sub_141afa320
    Replacement::draft(0x141afa3f0, "unknown_141afa3f0", fn_141afa3f0 as *const ()), // sub_141afa3f0
    Replacement::draft(0x141afa4a0, "unknown_141afa4a0", tpl_37425::<0x143cb5bc0, 0x20, 0x8, 0x1405bfa40, 0x28, 0x30, 0x38, 0x128> as *const ()), // sub_141afa4a0
    Replacement::draft(0x141afa540, "unknown_141afa540", fn_141afa540 as *const ()), // sub_141afa540
    Replacement::draft(0x141afa5a0, "unknown_141afa5a0", tpl_37426::<0x143cb5bc0, 0x8, 0x20, 0x1405bfa40, 0x28, 0x30, 0x38, 0x10e> as *const ()), // sub_141afa5a0
    Replacement::draft(0x141afa630, "unknown_141afa630", fn_141afa630 as *const ()), // sub_141afa630
    Replacement::draft(0x141afa670, "unknown_141afa670", fn_141afa670 as *const ()), // sub_141afa670
    Replacement::draft(0x141afa770, "unknown_141afa770", fn_141afa770 as *const ()), // sub_141afa770
    Replacement::draft(0x141afa7c0, "unknown_141afa7c0", fn_141afa7c0 as *const ()), // sub_141afa7c0
    Replacement::draft(0x141afa9c0, "unknown_141afa9c0", fn_141afa9c0 as *const ()), // sub_141afa9c0
    Replacement::draft_throws(0x141afaa20, "unknown_141afaa20", fn_141afaa20 as *const (), Throws::Discard), // sub_141afaa20
    Replacement::draft(0x141afafe0, "unknown_141afafe0", fn_141afafe0 as *const ()), // sub_141afafe0
    Replacement::draft(0x141afb100, "unknown_141afb100", fn_141afb100 as *const ()), // sub_141afb100
    Replacement::draft(0x141afb1f0, "unknown_141afb1f0", fn_141afb1f0 as *const ()), // sub_141afb1f0
    Replacement::draft(0x141afb400, "unknown_141afb400", fn_141afb400 as *const ()), // sub_141afb400
    Replacement::draft(0x141afb530, "unknown_141afb530", fn_141afb530 as *const ()), // sub_141afb530
    Replacement::draft(0x141afb630, "unknown_141afb630", fn_141afb630 as *const ()), // sub_141afb630
    Replacement::draft(0x141afb760, "unknown_141afb760", fn_141afb760 as *const ()), // sub_141afb760
    Replacement::draft(0x141afb780, "unknown_141afb780", tpl_37447 as *const ()), // sub_141afb780
    Replacement::draft(0x141afb7c0, "unknown_141afb7c0", tpl_34994::<0x143cb8784, 0x14387be90, 0x14387be88, 0x141834a78, 0x1425f5f20, 0x141834d18, 0x141834a0c> as *const ()), // sub_141afb7c0
    Replacement::draft(0x141afb830, "unknown_141afb830", tpl_369 as *const ()), // sub_141afb830
    Replacement::draft(0x141afb840, "unknown_141afb840", tpl_34994::<0x143cb8d28, 0x14387be60, 0x14387be58, 0x141834a78, 0x1425f5ed0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141afb840
    Replacement::draft(0x141afb8b0, "unknown_141afb8b0", tpl_35323 as *const ()), // sub_141afb8b0
    Replacement::draft(0x141afb8d0, "unknown_141afb8d0", tpl_34994::<0x143cb8710, 0x14387be70, 0x14387be68, 0x141834a78, 0x1425f5ee0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141afb8d0
    Replacement::draft(0x141afb940, "unknown_141afb940", tpl_34611::<0x8> as *const ()), // sub_141afb940
    Replacement::draft(0x141afb960, "unknown_141afb960", tpl_34994::<0x143cb83a8, 0x14387be40, 0x14387be38, 0x141834a78, 0x1425f5ef0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141afb960
    Replacement::draft(0x141afb9d0, "unknown_141afb9d0", tpl_34612 as *const ()), // sub_141afb9d0
    Replacement::draft(0x141afb9f0, "unknown_141afb9f0", tpl_34318 as *const ()), // sub_141afb9f0
    Replacement::draft(0x141afba10, "unknown_141afba10", tpl_34319::<0x143cb8c4c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afba10
    Replacement::draft(0x141afba70, "unknown_141afba70", tpl_34318 as *const ()), // sub_141afba70
    Replacement::draft(0x141afba90, "unknown_141afba90", tpl_34319::<0x143c8de68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afba90
    Replacement::draft(0x141afbaf0, "unknown_141afbaf0", tpl_34318 as *const ()), // sub_141afbaf0
    Replacement::draft(0x141afbb10, "unknown_141afbb10", tpl_34319::<0x143cb88a0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afbb10
    Replacement::draft(0x141afbb70, "unknown_141afbb70", tpl_34318 as *const ()), // sub_141afbb70
    Replacement::draft(0x141afbb90, "unknown_141afbb90", tpl_34319::<0x143cb82c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afbb90
    Replacement::draft(0x141afbbf0, "std_141afbbf0", tpl_550::<0x1431d8bb8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::cameramixer::updateBoneCamera::<lambda_1>,bool,const ?bref7::PhysicsSweepHit&>::std::_
    Replacement::draft(0x141afbc10, "std_141afbc10", tpl_550::<0x1431d8bb8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::cameramixer::updateBoneCamera::<lambda_1>,bool,const ?bref7::PhysicsSweepHit&>::std::_
    Replacement::draft(0x141afbc30, "std_141afbc30", fn_141afbc30 as *const ()), // std::_Func_impl_no_alloc<bigfish::cameramixer::updateBoneCamera::<lambda_1>,bool,const ?bref7::PhysicsSweepHit&>::std::_
    Replacement::draft(0x141afbca0, "std_141afbca0", tpl_1::<0x14388ce60> as *const ()), // std::_Func_impl_no_alloc<bigfish::cameramixer::updateBoneCamera::<lambda_1>,bool,const ?bref7::PhysicsSweepHit&>::std::_
    Replacement::draft(0x141afbcb0, "std_141afbcb0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::cameramixer::updateBoneCamera::<lambda_1>,bool,const ?bref7::PhysicsSweepHit&>::std::_
    Replacement::draft(0x141afbcc0, "std_141afbcc0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::cameramixer::updateBoneCamera::<lambda_1>,bool,const ?bref7::PhysicsSweepHit&>::std::_
    Replacement::draft(0x141afbce0, "unknown_141afbce0", tpl_34318 as *const ()), // sub_141afbce0
    Replacement::draft(0x141afbd00, "unknown_141afbd00", tpl_34318 as *const ()), // sub_141afbd00
    Replacement::draft(0x141afbd20, "unknown_141afbd20", tpl_34318 as *const ()), // sub_141afbd20
    Replacement::draft(0x141afbd40, "unknown_141afbd40", tpl_34318 as *const ()), // sub_141afbd40
    Replacement::draft(0x141afbd60, "unknown_141afbd60", tpl_34318 as *const ()), // sub_141afbd60
    Replacement::draft(0x141afbd80, "unknown_141afbd80", tpl_34318 as *const ()), // sub_141afbd80
    Replacement::draft(0x141afbda0, "unknown_141afbda0", tpl_34318 as *const ()), // sub_141afbda0
    Replacement::draft(0x141afbdc0, "unknown_141afbdc0", tpl_34318 as *const ()), // sub_141afbdc0
    Replacement::draft(0x141afbde0, "unknown_141afbde0", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afbde0
    Replacement::draft(0x141afbe40, "unknown_141afbe40", tpl_34319::<0x143c93358, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afbe40
    Replacement::draft(0x141afbea0, "unknown_141afbea0", tpl_34319::<0x143c933f0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afbea0
    Replacement::draft(0x141afbf00, "unknown_141afbf00", tpl_34319::<0x143cb82cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afbf00
    Replacement::draft(0x141afbf60, "unknown_141afbf60", tpl_34319::<0x143c997a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afbf60
    Replacement::draft(0x141afbfc0, "unknown_141afbfc0", tpl_34319::<0x143cb87f8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afbfc0
    Replacement::draft(0x141afc020, "unknown_141afc020", tpl_34319::<0x143c8cefc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc020
    Replacement::draft(0x141afc080, "unknown_141afc080", tpl_34319::<0x143c92bcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc080
    Replacement::draft(0x141afc0e0, "unknown_141afc0e0", tpl_34318 as *const ()), // sub_141afc0e0
    Replacement::draft(0x141afc100, "unknown_141afc100", tpl_34319::<0x143c8cf5c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc100
    Replacement::draft(0x141afc160, "unknown_141afc160", tpl_34318 as *const ()), // sub_141afc160
    Replacement::draft(0x141afc180, "unknown_141afc180", tpl_34319::<0x143c91608, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc180
    Replacement::draft(0x141afc1e0, "unknown_141afc1e0", tpl_34318 as *const ()), // sub_141afc1e0
    Replacement::draft(0x141afc200, "unknown_141afc200", tpl_34318 as *const ()), // sub_141afc200
    Replacement::draft(0x141afc220, "unknown_141afc220", tpl_34318 as *const ()), // sub_141afc220
    Replacement::draft(0x141afc240, "unknown_141afc240", tpl_34318 as *const ()), // sub_141afc240
    Replacement::draft(0x141afc260, "unknown_141afc260", tpl_34318 as *const ()), // sub_141afc260
    Replacement::draft(0x141afc280, "unknown_141afc280", tpl_34318 as *const ()), // sub_141afc280
    Replacement::draft(0x141afc2a0, "unknown_141afc2a0", tpl_34318 as *const ()), // sub_141afc2a0
    Replacement::draft(0x141afc2c0, "unknown_141afc2c0", tpl_34318 as *const ()), // sub_141afc2c0
    Replacement::draft(0x141afc2e0, "unknown_141afc2e0", tpl_34318 as *const ()), // sub_141afc2e0
    Replacement::draft(0x141afc300, "unknown_141afc300", tpl_34319::<0x143c8e658, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc300
    Replacement::draft(0x141afc360, "unknown_141afc360", tpl_34319::<0x143c96dec, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc360
    Replacement::draft(0x141afc3c0, "unknown_141afc3c0", tpl_34319::<0x143c8fab8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc3c0
    Replacement::draft(0x141afc420, "unknown_141afc420", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc420
    Replacement::draft(0x141afc480, "unknown_141afc480", tpl_34319::<0x143c8e68c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc480
    Replacement::draft(0x141afc4e0, "unknown_141afc4e0", tpl_34319::<0x143c8d074, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc4e0
    Replacement::draft(0x141afc540, "unknown_141afc540", tpl_34319::<0x143c8dbd0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc540
    Replacement::draft(0x141afc5a0, "unknown_141afc5a0", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc5a0
    Replacement::draft(0x141afc600, "unknown_141afc600", tpl_34319::<0x143c914c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc600
    Replacement::draft(0x141afc660, "unknown_141afc660", tpl_34318 as *const ()), // sub_141afc660
    Replacement::draft(0x141afc680, "unknown_141afc680", tpl_34318 as *const ()), // sub_141afc680
    Replacement::draft(0x141afc6a0, "unknown_141afc6a0", tpl_34318 as *const ()), // sub_141afc6a0
    Replacement::draft(0x141afc6c0, "unknown_141afc6c0", tpl_34318 as *const ()), // sub_141afc6c0
    Replacement::draft(0x141afc6e0, "unknown_141afc6e0", tpl_34318 as *const ()), // sub_141afc6e0
    Replacement::draft(0x141afc700, "unknown_141afc700", tpl_34318 as *const ()), // sub_141afc700
    Replacement::draft(0x141afc720, "unknown_141afc720", tpl_34318 as *const ()), // sub_141afc720
    Replacement::draft(0x141afc740, "unknown_141afc740", tpl_34318 as *const ()), // sub_141afc740
    Replacement::draft(0x141afc760, "unknown_141afc760", tpl_34318 as *const ()), // sub_141afc760
    Replacement::draft(0x141afc780, "unknown_141afc780", tpl_34318 as *const ()), // sub_141afc780
    Replacement::draft(0x141afc7a0, "unknown_141afc7a0", tpl_34318 as *const ()), // sub_141afc7a0
    Replacement::draft(0x141afc7c0, "unknown_141afc7c0", tpl_34318 as *const ()), // sub_141afc7c0
    Replacement::draft(0x141afc7e0, "unknown_141afc7e0", tpl_34318 as *const ()), // sub_141afc7e0
    Replacement::draft(0x141afc800, "unknown_141afc800", tpl_34319::<0x143c8efb0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc800
    Replacement::draft(0x141afc860, "unknown_141afc860", tpl_34319::<0x143c8ec6c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc860
    Replacement::draft(0x141afc8c0, "unknown_141afc8c0", tpl_34319::<0x143c96de8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc8c0
    Replacement::draft(0x141afc920, "unknown_141afc920", tpl_34319::<0x143c97794, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc920
    Replacement::draft(0x141afc980, "unknown_141afc980", tpl_34319::<0x143c8e840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc980
    Replacement::draft(0x141afc9e0, "unknown_141afc9e0", tpl_34319::<0x143c932b4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afc9e0
    Replacement::draft(0x141afca40, "unknown_141afca40", tpl_34319::<0x143cb8420, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afca40
    Replacement::draft(0x141afcaa0, "unknown_141afcaa0", tpl_34319::<0x143c91168, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afcaa0
    Replacement::draft(0x141afcb00, "unknown_141afcb00", tpl_34319::<0x143c8d508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afcb00
    Replacement::draft(0x141afcb60, "unknown_141afcb60", tpl_34319::<0x143c90198, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afcb60
    Replacement::draft(0x141afcbc0, "unknown_141afcbc0", tpl_34319::<0x143c96de4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afcbc0
    Replacement::draft(0x141afcc20, "unknown_141afcc20", tpl_34319::<0x143c91f48, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afcc20
    Replacement::draft(0x141afcc80, "unknown_141afcc80", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141afcc80
    Replacement::draft(0x141afcce0, "unknown_141afcce0", fn_141afcce0 as *const ()), // sub_141afcce0
    Replacement::draft(0x141afcdc0, "unknown_141afcdc0", tpl_361 as *const ()), // sub_141afcdc0
    Replacement::draft(0x141afcdd0, "unknown_141afcdd0", fn_141afcdd0 as *const ()), // sub_141afcdd0
    Replacement::draft(0x141afd3e0, "unknown_141afd3e0", fn_141afd3e0 as *const ()), // sub_141afd3e0
    Replacement::draft_throws(0x141afd580, "unknown_141afd580", fn_141afd580 as *const (), Throws::Discard), // sub_141afd580
    Replacement::draft(0x141afd910, "unknown_141afd910", fn_141afd910 as *const ()), // sub_141afd910
    Replacement::draft(0x141afdc70, "unknown_141afdc70", fn_141afdc70 as *const ()), // sub_141afdc70
    Replacement::draft_throws(0x141afe040, "unknown_141afe040", fn_141afe040 as *const (), Throws::Discard), // sub_141afe040
    Replacement::draft_throws(0x141afe310, "unknown_141afe310", fn_141afe310 as *const (), Throws::Discard), // sub_141afe310
    Replacement::draft_throws(0x141aff2b0, "unknown_141aff2b0", fn_141aff2b0 as *const (), Throws::Discard), // sub_141aff2b0
    Replacement::draft_throws(0x141aff370, "unknown_141aff370", fn_141aff370 as *const (), Throws::Discard), // sub_141aff370
    Replacement::draft(0x141affee0, "unknown_141affee0", fn_141affee0 as *const ()), // sub_141affee0
    Replacement::draft(0x141b00520, "unknown_141b00520", fn_141b00520 as *const ()), // sub_141b00520
    Replacement::draft(0x141b00f50, "unknown_141b00f50", fn_141b00f50 as *const ()), // sub_141b00f50
    Replacement::draft(0x141b00fe0, "unknown_141b00fe0", fn_141b00fe0 as *const ()), // sub_141b00fe0
    Replacement::draft(0x141b01160, "unknown_141b01160", fn_141b01160 as *const ()), // sub_141b01160
    Replacement::draft(0x141b012d0, "unknown_141b012d0", fn_141b012d0 as *const ()), // sub_141b012d0
    Replacement::draft(0x141b01750, "unknown_141b01750", fn_141b01750 as *const ()), // sub_141b01750
    Replacement::draft(0x141b017f0, "unknown_141b017f0", fn_141b017f0 as *const ()), // sub_141b017f0
    Replacement::draft(0x141b01d80, "unknown_141b01d80", fn_141b01d80 as *const ()), // sub_141b01d80
    Replacement::draft_throws(0x141b01e00, "unknown_141b01e00", fn_141b01e00 as *const (), Throws::Discard), // sub_141b01e00
    Replacement::draft(0x141b023e0, "unknown_141b023e0", tpl_37460::<0x143426d6c, 0x143426e04, 0x143426dd4, 0x1434268d0, 0x140275010, 0x143428760, 0x140275570, 0x140275440> as *const ()), // sub_141b023e0
    Replacement::draft(0x141b02570, "unknown_141b02570", fn_141b02570 as *const ()), // sub_141b02570
    Replacement::draft(0x141b025b0, "unknown_141b025b0", fn_141b025b0 as *const ()), // sub_141b025b0
    Replacement::draft(0x141b02740, "unknown_141b02740", fn_141b02740 as *const ()), // sub_141b02740
    Replacement::draft(0x141b02850, "unknown_141b02850", fn_141b02850 as *const ()), // sub_141b02850
    Replacement::draft(0x141b028b0, "unknown_141b028b0", fn_141b028b0 as *const ()), // sub_141b028b0
    Replacement::draft(0x141b028e0, "unknown_141b028e0", tpl_36473::<0x4> as *const ()), // sub_141b028e0
    Replacement::draft(0x141b02920, "unknown_141b02920", fn_141b02920 as *const ()), // sub_141b02920
    Replacement::draft(0x141b02970, "unknown_141b02970", fn_141b02970 as *const ()), // sub_141b02970
    Replacement::draft(0x141b029c0, "unknown_141b029c0", fn_141b029c0 as *const ()), // sub_141b029c0
    Replacement::draft(0x141b029e0, "unknown_141b029e0", fn_141b029e0 as *const ()), // sub_141b029e0
    Replacement::draft(0x141b02b80, "unknown_141b02b80", fn_141b02b80 as *const ()), // sub_141b02b80
    Replacement::draft(0x141b02ba0, "unknown_141b02ba0", fn_141b02ba0 as *const ()), // sub_141b02ba0
    Replacement::draft(0x141b02c40, "unknown_141b02c40", fn_141b02c40 as *const ()), // sub_141b02c40
    Replacement::draft(0x141b02ce0, "unknown_141b02ce0", fn_141b02ce0 as *const ()), // sub_141b02ce0
    Replacement::draft(0x141b02d10, "unknown_141b02d10", fn_141b02d10 as *const ()), // sub_141b02d10
    Replacement::draft(0x141b02e30, "unknown_141b02e30", fn_141b02e30 as *const ()), // sub_141b02e30
    Replacement::draft(0x141b02eb0, "unknown_141b02eb0", fn_141b02eb0 as *const ()), // sub_141b02eb0
    Replacement::draft(0x141b03010, "unknown_141b03010", fn_141b03010 as *const ()), // sub_141b03010
    Replacement::draft(0x141b03150, "unknown_141b03150", fn_141b03150 as *const ()), // sub_141b03150
    Replacement::draft(0x141b03330, "unknown_141b03330", fn_141b03330 as *const ()), // sub_141b03330
    Replacement::draft(0x141b03470, "unknown_141b03470", fn_141b03470 as *const ()), // sub_141b03470
    Replacement::draft(0x141b03530, "unknown_141b03530", fn_141b03530 as *const ()), // sub_141b03530
    Replacement::draft(0x141b036b0, "unknown_141b036b0", fn_141b036b0 as *const ()), // sub_141b036b0
    Replacement::draft(0x141b038c0, "unknown_141b038c0", fn_141b038c0 as *const ()), // sub_141b038c0
    Replacement::draft(0x141b03980, "unknown_141b03980", fn_141b03980 as *const ()), // sub_141b03980
    Replacement::draft(0x141b03a20, "unknown_141b03a20", fn_141b03a20 as *const ()), // sub_141b03a20
    Replacement::draft(0x141b03ac0, "unknown_141b03ac0", fn_141b03ac0 as *const ()), // sub_141b03ac0
    Replacement::draft(0x141b03b00, "unknown_141b03b00", fn_141b03b00 as *const ()), // sub_141b03b00
    Replacement::draft(0x141b03bf0, "unknown_141b03bf0", fn_141b03bf0 as *const ()), // sub_141b03bf0
    Replacement::draft_throws(0x141b03c20, "unknown_141b03c20", fn_141b03c20 as *const (), Throws::Discard), // sub_141b03c20
    Replacement::draft(0x141b04a00, "unknown_141b04a00", fn_141b04a00 as *const ()), // sub_141b04a00
    Replacement::draft_throws(0x141b04ae0, "unknown_141b04ae0", fn_141b04ae0 as *const (), Throws::Discard), // sub_141b04ae0
    Replacement::draft(0x141b05270, "unknown_141b05270", tpl_37480::<0x20> as *const ()), // sub_141b05270
    Replacement::draft(0x141b05310, "unknown_141b05310", fn_141b05310 as *const ()), // sub_141b05310
    Replacement::draft(0x141b05420, "unknown_141b05420", fn_141b05420 as *const ()), // sub_141b05420
    Replacement::draft(0x141b05490, "unknown_141b05490", tpl_35328::<0x1401f2bc0> as *const ()), // sub_141b05490
    Replacement::draft(0x141b05540, "unknown_141b05540", fn_141b05540 as *const ()), // sub_141b05540
    Replacement::draft(0x141b05640, "unknown_141b05640", fn_141b05640 as *const ()), // sub_141b05640
    Replacement::draft(0x141b056a0, "unknown_141b056a0", tpl_35060 as *const ()), // sub_141b056a0
    Replacement::draft(0x141b056e0, "unknown_141b056e0", tpl_34318 as *const ()), // sub_141b056e0
    Replacement::draft(0x141b05700, "unknown_141b05700", tpl_34318 as *const ()), // sub_141b05700
    Replacement::draft(0x141b05720, "unknown_141b05720", tpl_34319::<0x143c8e840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b05720
    Replacement::draft(0x141b05780, "unknown_141b05780", tpl_34319::<0x143cb8ec8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b05780
    Replacement::draft(0x141b057e0, "unknown_141b057e0", tpl_34318 as *const ()), // sub_141b057e0
    Replacement::draft(0x141b05800, "unknown_141b05800", tpl_34319::<0x143c98afc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b05800
    Replacement::draft(0x141b05860, "unknown_141b05860", tpl_34318 as *const ()), // sub_141b05860
    Replacement::draft(0x141b05880, "unknown_141b05880", tpl_34318 as *const ()), // sub_141b05880
    Replacement::draft(0x141b058a0, "unknown_141b058a0", tpl_34318 as *const ()), // sub_141b058a0
    Replacement::draft(0x141b058c0, "unknown_141b058c0", tpl_34319::<0x143cb8420, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b058c0
    Replacement::draft(0x141b05920, "unknown_141b05920", tpl_34319::<0x143cb8fc8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b05920
    Replacement::draft(0x141b05980, "unknown_141b05980", tpl_34319::<0x143cb8eb4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b05980
    Replacement::draft(0x141b059e0, "unknown_141b059e0", fn_141b059e0 as *const ()), // sub_141b059e0
    Replacement::draft(0x141b05bc0, "unknown_141b05bc0", fn_141b05bc0 as *const ()), // sub_141b05bc0
    Replacement::draft(0x141b05c70, "unknown_141b05c70", tpl_361 as *const ()), // sub_141b05c70
    Replacement::draft(0x141b05c80, "unknown_141b05c80", fn_141b05c80 as *const ()), // sub_141b05c80
    Replacement::draft(0x141b05e30, "unknown_141b05e30", tpl_361 as *const ()), // sub_141b05e30
    Replacement::draft(0x141b05e40, "unknown_141b05e40", fn_141b05e40 as *const ()), // sub_141b05e40
    Replacement::draft(0x141b06220, "unknown_141b06220", tpl_361 as *const ()), // sub_141b06220
    Replacement::draft_throws(0x141b06230, "unknown_141b06230", fn_141b06230 as *const (), Throws::Discard), // sub_141b06230
    Replacement::draft(0x141b06450, "unknown_141b06450", tpl_34994::<0x143cb90ac, 0x14387bed0, 0x14387bec8, 0x141834a78, 0x1425f6820, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b06450
    Replacement::draft(0x141b064c0, "unknown_141b064c0", fn_141b064c0 as *const ()), // sub_141b064c0
    Replacement::draft(0x141b06560, "unknown_141b06560", fn_141b06560 as *const ()), // sub_141b06560
    Replacement::draft(0x141b06590, "unknown_141b06590", fn_141b06590 as *const ()), // sub_141b06590
    Replacement::draft(0x141b065f0, "unknown_141b065f0", tpl_34318 as *const ()), // sub_141b065f0
    Replacement::draft(0x141b06610, "unknown_141b06610", tpl_34318 as *const ()), // sub_141b06610
    Replacement::draft(0x141b06630, "unknown_141b06630", tpl_34318 as *const ()), // sub_141b06630
    Replacement::draft(0x141b06650, "unknown_141b06650", tpl_34318 as *const ()), // sub_141b06650
    Replacement::draft(0x141b06670, "unknown_141b06670", tpl_34319::<0x143c8de7c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06670
    Replacement::draft(0x141b066d0, "unknown_141b066d0", tpl_34319::<0x143c8f4cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b066d0
    Replacement::draft(0x141b06730, "unknown_141b06730", tpl_34319::<0x143c8e66c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06730
    Replacement::draft(0x141b06790, "unknown_141b06790", tpl_34319::<0x143c8f3b8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06790
    Replacement::draft(0x141b067f0, "unknown_141b067f0", tpl_34318 as *const ()), // sub_141b067f0
    Replacement::draft(0x141b06810, "unknown_141b06810", tpl_34318 as *const ()), // sub_141b06810
    Replacement::draft(0x141b06830, "unknown_141b06830", tpl_34318 as *const ()), // sub_141b06830
    Replacement::draft(0x141b06850, "unknown_141b06850", tpl_34318 as *const ()), // sub_141b06850
    Replacement::draft(0x141b06870, "unknown_141b06870", tpl_34318 as *const ()), // sub_141b06870
    Replacement::draft(0x141b06890, "unknown_141b06890", tpl_34318 as *const ()), // sub_141b06890
    Replacement::draft(0x141b068b0, "unknown_141b068b0", tpl_34318 as *const ()), // sub_141b068b0
    Replacement::draft(0x141b068d0, "unknown_141b068d0", tpl_34318 as *const ()), // sub_141b068d0
    Replacement::draft(0x141b068f0, "unknown_141b068f0", tpl_34319::<0x143caf9ac, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b068f0
    Replacement::draft(0x141b06950, "unknown_141b06950", tpl_34319::<0x143c8edbc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06950
    Replacement::draft(0x141b069b0, "unknown_141b069b0", tpl_34319::<0x143c8dbcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b069b0
    Replacement::draft(0x141b06a10, "unknown_141b06a10", tpl_34319::<0x143c8eb98, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06a10
    Replacement::draft(0x141b06a70, "unknown_141b06a70", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06a70
    Replacement::draft(0x141b06ad0, "unknown_141b06ad0", tpl_34319::<0x143c8fa04, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06ad0
    Replacement::draft(0x141b06b30, "unknown_141b06b30", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06b30
    Replacement::draft(0x141b06b90, "unknown_141b06b90", tpl_34319::<0x143c8ea68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06b90
    Replacement::draft(0x141b06bf0, "unknown_141b06bf0", tpl_34318 as *const ()), // sub_141b06bf0
    Replacement::draft(0x141b06c10, "unknown_141b06c10", tpl_34319::<0x143c8e81c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b06c10
    Replacement::draft(0x141b06c70, "std_141b06c70", tpl_23342::<0x1431e3c68, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::bullet_ray_cast::casting::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x141b06c90, "std_141b06c90", tpl_23342::<0x1431e3c68, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::bullet_ray_cast::casting::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x141b06cb0, "std_141b06cb0", fn_141b06cb0 as *const ()), // std::_Func_impl_no_alloc<bigfish::bullet_ray_cast::casting::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x141b06d20, "std_141b06d20", tpl_1::<0x14388d040> as *const ()), // std::_Func_impl_no_alloc<bigfish::bullet_ray_cast::casting::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x141b06d30, "std_141b06d30", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::bullet_ray_cast::casting::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x141b06d40, "std_141b06d40", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::bullet_ray_cast::casting::system::<lambda_1>,bool,const coregame::PhysicsSweepHit&>::s
    Replacement::draft(0x141b06d60, "unknown_141b06d60", fn_141b06d60 as *const ()), // sub_141b06d60
    Replacement::draft(0x141b06e50, "unknown_141b06e50", tpl_361 as *const ()), // sub_141b06e50
    Replacement::draft(0x141b06e60, "unknown_141b06e60", fn_141b06e60 as *const ()), // sub_141b06e60
    Replacement::draft(0x141b077e0, "unknown_141b077e0", tpl_34443 as *const ()), // sub_141b077e0
    Replacement::draft(0x141b077f0, "unknown_141b077f0", fn_141b077f0 as *const ()), // sub_141b077f0
    Replacement::draft(0x141b078c0, "unknown_141b078c0", tpl_22031::<0x8> as *const ()), // sub_141b078c0
    Replacement::draft(0x141b078e0, "unknown_141b078e0", fn_141b078e0 as *const ()), // sub_141b078e0
    Replacement::draft(0x141b07950, "unknown_141b07950", tpl_496 as *const ()), // sub_141b07950
    Replacement::draft(0x141b07960, "unknown_141b07960", tpl_496 as *const ()), // sub_141b07960
    Replacement::draft(0x141b07970, "unknown_141b07970", fn_141b07970 as *const ()), // sub_141b07970
    Replacement::draft(0x141b08b80, "unknown_141b08b80", tpl_361 as *const ()), // sub_141b08b80
    Replacement::draft(0x141b08b90, "unknown_141b08b90", fn_141b08b90 as *const ()), // sub_141b08b90
    Replacement::draft(0x141b08d80, "unknown_141b08d80", tpl_496 as *const ()), // sub_141b08d80
    Replacement::draft(0x141b08d90, "unknown_141b08d90", tpl_496 as *const ()), // sub_141b08d90
    Replacement::draft(0x141b08da0, "unknown_141b08da0", tpl_496 as *const ()), // sub_141b08da0
    Replacement::draft(0x141b08db0, "unknown_141b08db0", tpl_496 as *const ()), // sub_141b08db0
    Replacement::draft(0x141b08dc0, "unknown_141b08dc0", tpl_496 as *const ()), // sub_141b08dc0
    Replacement::draft(0x141b08dd0, "unknown_141b08dd0", fn_141b08dd0 as *const ()), // sub_141b08dd0
    Replacement::draft(0x141b09d10, "unknown_141b09d10", tpl_361 as *const ()), // sub_141b09d10
    Replacement::draft(0x141b09d20, "unknown_141b09d20", fn_141b09d20 as *const ()), // sub_141b09d20
    Replacement::draft(0x141b09e30, "unknown_141b09e30", fn_141b09e30 as *const ()), // sub_141b09e30
    Replacement::draft_throws(0x141b0a140, "unknown_141b0a140", fn_141b0a140 as *const (), Throws::Discard), // sub_141b0a140
    Replacement::draft_throws(0x141b0a680, "unknown_141b0a680", tpl_35009::<0x14387de74, 0x1405d8a70, 0x1402116e0> as *const (), Throws::Discard), // sub_141b0a680
    Replacement::draft(0x141b0a6c0, "unknown_141b0a6c0", tpl_34994::<0x143cb95c0, 0x14387bef0, 0x14387bee8, 0x141834a78, 0x1425f6900, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b0a6c0
    Replacement::draft(0x141b0a730, "unknown_141b0a730", tpl_35332 as *const ()), // sub_141b0a730
    Replacement::draft(0x141b0a740, "unknown_141b0a740", tpl_34994::<0x143cb95b4, 0x14387bee0, 0x14387bed8, 0x141834a78, 0x1425f6910, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b0a740
    Replacement::draft(0x141b0a7b0, "unknown_141b0a7b0", fn_141b0a7b0 as *const ()), // sub_141b0a7b0
    Replacement::draft(0x141b0a870, "unknown_141b0a870", fn_141b0a870 as *const ()), // sub_141b0a870
    Replacement::draft(0x141b0a8c0, "unknown_141b0a8c0", tpl_36138::<0x14240ab10> as *const ()), // sub_141b0a8c0
    Replacement::draft(0x141b0a900, "unknown_141b0a900", tpl_34318 as *const ()), // sub_141b0a900
    Replacement::draft(0x141b0a920, "unknown_141b0a920", tpl_34318 as *const ()), // sub_141b0a920
    Replacement::draft(0x141b0a940, "unknown_141b0a940", tpl_34318 as *const ()), // sub_141b0a940
    Replacement::draft(0x141b0a960, "unknown_141b0a960", tpl_34318 as *const ()), // sub_141b0a960
    Replacement::draft(0x141b0a980, "unknown_141b0a980", tpl_34318 as *const ()), // sub_141b0a980
    Replacement::draft(0x141b0a9a0, "unknown_141b0a9a0", tpl_34318 as *const ()), // sub_141b0a9a0
    Replacement::draft(0x141b0a9c0, "unknown_141b0a9c0", tpl_34318 as *const ()), // sub_141b0a9c0
    Replacement::draft(0x141b0a9e0, "unknown_141b0a9e0", tpl_34318 as *const ()), // sub_141b0a9e0
    Replacement::draft(0x141b0aa00, "unknown_141b0aa00", tpl_34318 as *const ()), // sub_141b0aa00
    Replacement::draft(0x141b0aa20, "unknown_141b0aa20", tpl_34318 as *const ()), // sub_141b0aa20
    Replacement::draft(0x141b0aa40, "unknown_141b0aa40", tpl_34318 as *const ()), // sub_141b0aa40
    Replacement::draft(0x141b0aa60, "unknown_141b0aa60", tpl_34318 as *const ()), // sub_141b0aa60
    Replacement::draft(0x141b0aa80, "unknown_141b0aa80", tpl_34318 as *const ()), // sub_141b0aa80
    Replacement::draft(0x141b0aaa0, "unknown_141b0aaa0", tpl_34318 as *const ()), // sub_141b0aaa0
    Replacement::draft(0x141b0aac0, "unknown_141b0aac0", tpl_34318 as *const ()), // sub_141b0aac0
    Replacement::draft(0x141b0aae0, "unknown_141b0aae0", tpl_34319::<0x143c97794, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0aae0
    Replacement::draft(0x141b0ab40, "unknown_141b0ab40", tpl_34319::<0x143c91f1c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0ab40
    Replacement::draft(0x141b0aba0, "unknown_141b0aba0", tpl_34319::<0x143c8d87c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0aba0
    Replacement::draft(0x141b0ac00, "unknown_141b0ac00", tpl_34319::<0x143c8e70c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0ac00
    Replacement::draft(0x141b0ac60, "unknown_141b0ac60", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0ac60
    Replacement::draft(0x141b0acc0, "unknown_141b0acc0", tpl_34319::<0x143c8e658, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0acc0
    Replacement::draft(0x141b0ad20, "unknown_141b0ad20", tpl_34319::<0x143c91f48, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0ad20
    Replacement::draft(0x141b0ad80, "unknown_141b0ad80", tpl_34319::<0x143c97a20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0ad80
    Replacement::draft(0x141b0ade0, "unknown_141b0ade0", tpl_34319::<0x143c8dbc4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0ade0
    Replacement::draft(0x141b0ae40, "unknown_141b0ae40", tpl_34319::<0x143c8d508, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0ae40
    Replacement::draft(0x141b0aea0, "unknown_141b0aea0", tpl_34319::<0x143c8d474, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0aea0
    Replacement::draft(0x141b0af00, "unknown_141b0af00", tpl_34319::<0x143c8eb90, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0af00
    Replacement::draft(0x141b0af60, "unknown_141b0af60", tpl_34319::<0x143c91168, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0af60
    Replacement::draft(0x141b0afc0, "unknown_141b0afc0", tpl_34319::<0x143c8dbd0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0afc0
    Replacement::draft(0x141b0b020, "unknown_141b0b020", tpl_34319::<0x143c96de8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0b020
    Replacement::draft(0x141b0b080, "unknown_141b0b080", tpl_34318 as *const ()), // sub_141b0b080
    Replacement::draft(0x141b0b0a0, "unknown_141b0b0a0", tpl_361 as *const ()), // sub_141b0b0a0
    Replacement::draft(0x141b0b0b0, "unknown_141b0b0b0", tpl_34319::<0x143c8fabc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0b0b0
    Replacement::draft(0x141b0b110, "unknown_141b0b110", tpl_35063::<0x143c9d17c, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141b0b110
    Replacement::draft(0x141b0b160, "unknown_141b0b160", tpl_34318 as *const ()), // sub_141b0b160
    Replacement::draft(0x141b0b180, "unknown_141b0b180", tpl_34318 as *const ()), // sub_141b0b180
    Replacement::draft(0x141b0b1a0, "unknown_141b0b1a0", tpl_34318 as *const ()), // sub_141b0b1a0
    Replacement::draft(0x141b0b1c0, "unknown_141b0b1c0", tpl_34318 as *const ()), // sub_141b0b1c0
    Replacement::draft(0x141b0b1e0, "unknown_141b0b1e0", tpl_34319::<0x143ca02c8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0b1e0
    Replacement::draft(0x141b0b240, "unknown_141b0b240", tpl_34319::<0x143cb95b0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0b240
    Replacement::draft(0x141b0b2a0, "unknown_141b0b2a0", tpl_34319::<0x143cb96dc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0b2a0
    Replacement::draft(0x141b0b300, "unknown_141b0b300", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0b300
    Replacement::draft(0x141b0b360, "unknown_141b0b360", fn_141b0b360 as *const ()), // sub_141b0b360
    Replacement::draft(0x141b0b470, "unknown_141b0b470", tpl_496 as *const ()), // sub_141b0b470
    Replacement::draft_throws(0x141b0b480, "unknown_141b0b480", fn_141b0b480 as *const (), Throws::Caught), // sub_141b0b480
    Replacement::draft(0x141b0b680, "unknown_141b0b680", tpl_361 as *const ()), // sub_141b0b680
    Replacement::draft(0x141b0b690, "unknown_141b0b690", tpl_361 as *const ()), // sub_141b0b690
    Replacement::draft_throws(0x141b0b6a0, "unknown_141b0b6a0", fn_141b0b6a0 as *const (), Throws::Discard), // sub_141b0b6a0
    Replacement::draft(0x141b0c5a0, "unknown_141b0c5a0", tpl_361 as *const ()), // sub_141b0c5a0
    Replacement::draft_throws(0x141b0c5b0, "unknown_141b0c5b0", fn_141b0c5b0 as *const (), Throws::Discard), // sub_141b0c5b0
    Replacement::draft(0x141b0c970, "unknown_141b0c970", tpl_34318 as *const ()), // sub_141b0c970
    Replacement::draft(0x141b0c990, "unknown_141b0c990", tpl_34318 as *const ()), // sub_141b0c990
    Replacement::draft(0x141b0c9b0, "unknown_141b0c9b0", tpl_34319::<0x143c8de68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0c9b0
    Replacement::draft(0x141b0ca10, "unknown_141b0ca10", tpl_34319::<0x143c8ff58, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0ca10
    Replacement::draft(0x141b0ca70, "unknown_141b0ca70", fn_141b0ca70 as *const ()), // sub_141b0ca70
    Replacement::draft(0x141b0cbb0, "unknown_141b0cbb0", tpl_361 as *const ()), // sub_141b0cbb0
    Replacement::draft_throws(0x141b0cbc0, "unknown_141b0cbc0", tpl_37501::<0x142773280, 0x1410d6ca0, 0x1405dbd50, 0x143823500, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_141b0cbc0
    Replacement::draft(0x141b0cd50, "unknown_141b0cd50", tpl_34318 as *const ()), // sub_141b0cd50
    Replacement::draft(0x141b0cd70, "unknown_141b0cd70", tpl_34318 as *const ()), // sub_141b0cd70
    Replacement::draft(0x141b0cd90, "unknown_141b0cd90", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0cd90
    Replacement::draft(0x141b0cdf0, "unknown_141b0cdf0", tpl_34319::<0x143c90060, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0cdf0
    Replacement::draft(0x141b0ce50, "unknown_141b0ce50", tpl_34318 as *const ()), // sub_141b0ce50
    Replacement::draft(0x141b0ce70, "unknown_141b0ce70", tpl_34318 as *const ()), // sub_141b0ce70
    Replacement::draft(0x141b0ce90, "unknown_141b0ce90", tpl_34318 as *const ()), // sub_141b0ce90
    Replacement::draft(0x141b0ceb0, "unknown_141b0ceb0", tpl_34318 as *const ()), // sub_141b0ceb0
    Replacement::draft(0x141b0ced0, "unknown_141b0ced0", tpl_34318 as *const ()), // sub_141b0ced0
    Replacement::draft(0x141b0cef0, "unknown_141b0cef0", tpl_34318 as *const ()), // sub_141b0cef0
    Replacement::draft(0x141b0cf10, "unknown_141b0cf10", tpl_34318 as *const ()), // sub_141b0cf10
    Replacement::draft(0x141b0cf30, "unknown_141b0cf30", tpl_34319::<0x143c8faa0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0cf30
    Replacement::draft(0x141b0cf90, "unknown_141b0cf90", tpl_34319::<0x143ca760c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0cf90
    Replacement::draft(0x141b0cff0, "unknown_141b0cff0", tpl_34319::<0x143ca7874, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0cff0
    Replacement::draft(0x141b0d050, "unknown_141b0d050", tpl_34319::<0x143c973f4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0d050
    Replacement::draft(0x141b0d0b0, "unknown_141b0d0b0", tpl_34319::<0x143c9db0c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0d0b0
    Replacement::draft(0x141b0d110, "unknown_141b0d110", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0d110
    Replacement::draft(0x141b0d170, "unknown_141b0d170", tpl_34319::<0x143c8ff58, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0d170
    Replacement::draft(0x141b0d1d0, "unknown_141b0d1d0", tpl_34318 as *const ()), // sub_141b0d1d0
    Replacement::draft(0x141b0d1f0, "unknown_141b0d1f0", tpl_34319::<0x143c8e70c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0d1f0
    Replacement::draft(0x141b0d250, "unknown_141b0d250", tpl_34318 as *const ()), // sub_141b0d250
    Replacement::draft(0x141b0d270, "unknown_141b0d270", tpl_34318 as *const ()), // sub_141b0d270
    Replacement::draft(0x141b0d290, "unknown_141b0d290", tpl_34319::<0x143c8f4d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0d290
    Replacement::draft(0x141b0d2f0, "unknown_141b0d2f0", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b0d2f0
    Replacement::draft(0x141b0d350, "unknown_141b0d350", tpl_1439::<0x8, 0x1405dd590> as *const ()), // sub_141b0d350
    Replacement::draft_throws(0x141b0d390, "unknown_141b0d390", fn_141b0d390 as *const (), Throws::Discard), // sub_141b0d390
    Replacement::draft(0x141b0d4f0, "unknown_141b0d4f0", tpl_361 as *const ()), // sub_141b0d4f0
    Replacement::draft_throws(0x141b0d500, "unknown_141b0d500", fn_141b0d500 as *const (), Throws::Caught), // sub_141b0d500
    Replacement::draft(0x141b0d5f0, "unknown_141b0d5f0", fn_141b0d5f0 as *const ()), // sub_141b0d5f0
    Replacement::draft_throws(0x141b0dac0, "unknown_141b0dac0", fn_141b0dac0 as *const (), Throws::Discard), // sub_141b0dac0
    Replacement::draft(0x141b0dec0, "unknown_141b0dec0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c8faa0, 0x158, 0x14020e8f0> as *const ()), // sub_141b0dec0
    Replacement::draft(0x141b0dfc0, "unknown_141b0dfc0", fn_141b0dfc0 as *const ()), // sub_141b0dfc0
    Replacement::draft_throws(0x141b0e040, "unknown_141b0e040", fn_141b0e040 as *const (), Throws::Discard), // sub_141b0e040
    Replacement::draft(0x141b0e880, "bigfish_141b0e880", tpl_37507::<0x1405e02b0, 0x142442de0> as *const ()), // bigfish::AbilityChargeAttackProgramParams::bigfish::AbilityChargeAttackProgramParams::vfunc_2
    Replacement::draft(0x141b0e8e0, "bigfish_141b0e8e0", tpl_5853::<0x8, 0x1405e0420, 0x143428240, 0x1434268d0, 0x1405e06f0> as *const ()), // sub_141b0e8e0
    Replacement::draft_throws(0x141b0e980, "bigfish_141b0e980", tpl_35009::<0x14387cbcc, 0x1405e0a30, 0x1402116e0> as *const (), Throws::Discard), // sub_141b0e980
    Replacement::draft(0x141b0e9c0, "bigfish_141b0e9c0", fn_141b0e9c0 as *const ()), // bigfish::BehaviorTreeUseMeleeNodeInstance::bigfish::BehaviorTreeUseMeleeNodeInstance::vfunc_37
    Replacement::draft(0x141b0e9f0, "bigfish_141b0e9f0", fn_141b0e9f0 as *const ()), // sub_141b0e9f0
    Replacement::draft(0x141b0f1e0, "bigfish_141b0f1e0", fn_141b0f1e0 as *const ()), // bigfish::BehaviorTreeUseMeleeNodeInstance::ClassTypeInfoImplementation::bigfish::BehaviorTreeUseMeleeNodeInstance::Class
    Replacement::draft(0x141b0f290, "bigfish_141b0f290", fn_141b0f290 as *const ()), // bigfish::BehaviorTreeUseMeleeNodeInstance::ClassTypeInfoImplementation::bigfish::BehaviorTreeUseMeleeNodeInstance::Class
    Replacement::draft_throws(0x141b0f420, "bigfish_141b0f420", fn_141b0f420 as *const (), Throws::Caught), // sub_141b0f420
    Replacement::draft_throws(0x141b0f720, "bigfish_141b0f720", fn_141b0f720 as *const (), Throws::Discard), // sub_141b0f720
    Replacement::draft_throws(0x141b0f900, "bigfish_141b0f900", fn_141b0f900 as *const (), Throws::Caught), // bigfish::BehaviorTreeRushThroughNodeInstance::bigfish::BehaviorTreeRushThroughNodeInstance::vfunc_6
    Replacement::draft_throws(0x141b0feb0, "bigfish_141b0feb0", fn_141b0feb0 as *const (), Throws::Discard), // bigfish::BehaviorTreeRushThroughNodeInstance::bigfish::BehaviorTreeRushThroughNodeInstance::vfunc_11
    Replacement::draft_throws(0x141b0ffa0, "bigfish_141b0ffa0", fn_141b0ffa0 as *const (), Throws::Discard), // bigfish::BehaviorTreeRushThroughNodeInstance::bigfish::BehaviorTreeRushThroughNodeInstance::vfunc_10
    Replacement::draft(0x141b105a0, "bigfish_141b105a0", fn_141b105a0 as *const ()), // bigfish::BehaviorTreeRushThroughNodeInstance::ClassTypeInfoImplementation::bigfish::BehaviorTreeRushThroughNodeInstance:
    Replacement::draft(0x141b10680, "bigfish_141b10680", fn_141b10680 as *const ()), // bigfish::BehaviorTreeRushThroughNodeInstance::ClassTypeInfoImplementation::bigfish::BehaviorTreeRushThroughNodeInstance:
    Replacement::draft_throws(0x141b10830, "bigfish_141b10830", fn_141b10830 as *const (), Throws::Caught), // sub_141b10830
    Replacement::draft_throws(0x141b10b40, "bigfish_141b10b40", fn_141b10b40 as *const (), Throws::Discard), // sub_141b10b40
    Replacement::draft_throws(0x141b10d20, "bigfish_141b10d20", fn_141b10d20 as *const (), Throws::Caught), // bigfish::BehaviorTreeMoveToCanDestroyNodeInstance::bigfish::BehaviorTreeMoveToCanDestroyNodeInstance::vfunc_6
    Replacement::draft_throws(0x141b11140, "bigfish_141b11140", fn_141b11140 as *const (), Throws::Discard), // bigfish::BehaviorTreeMoveToCanDestroyNodeInstance::bigfish::BehaviorTreeMoveToCanDestroyNodeInstance::vfunc_11
    Replacement::draft_throws(0x141b11250, "bigfish_141b11250", fn_141b11250 as *const (), Throws::Discard), // bigfish::BehaviorTreeMoveToCanDestroyNodeInstance::bigfish::BehaviorTreeMoveToCanDestroyNodeInstance::vfunc_10
    Replacement::draft(0x141b11850, "bigfish_141b11850", fn_141b11850 as *const ()), // bigfish::BehaviorTreeMoveToCanDestroyNodeInstance::ClassTypeInfoImplementation::bigfish::BehaviorTreeMoveToCanDestroyNod
    Replacement::draft(0x141b11940, "bigfish_141b11940", fn_141b11940 as *const ()), // bigfish::BehaviorTreeMoveToCanDestroyNodeInstance::ClassTypeInfoImplementation::bigfish::BehaviorTreeMoveToCanDestroyNod
    Replacement::draft(0x141b11b00, "bigfish_141b11b00", fn_141b11b00 as *const ()), // bigfish::BehaviorTreeFollowScheduleInstance::bigfish::BehaviorTreeFollowScheduleInstance::vfunc_6
    Replacement::draft(0x141b11b20, "bigfish_141b11b20", fn_141b11b20 as *const ()), // bigfish::BehaviorTreeFollowScheduleInstance::bigfish::BehaviorTreeFollowScheduleInstance::vfunc_11
    Replacement::draft(0x141b11ba0, "bigfish_141b11ba0", fn_141b11ba0 as *const ()), // bigfish::BehaviorTreeFollowScheduleInstance::bigfish::BehaviorTreeFollowScheduleInstance::vfunc_10
    Replacement::draft(0x141b11c20, "bigfish_141b11c20", fn_141b11c20 as *const ()), // sub_141b11c20
    Replacement::draft(0x141b121d0, "bigfish_141b121d0", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bigfish::BehaviorTreeFollowScheduleInstance::ClassTypeInfoImplementation::bigfish::BehaviorTreeFollowScheduleInstance::C
    Replacement::draft(0x141b123f0, "bigfish_141b123f0", fn_141b123f0 as *const ()), // bigfish::BehaviorTreeChargeAttackNode::bigfish::BehaviorTreeChargeAttackNode::vfunc_6
    Replacement::draft(0x141b12450, "bigfish_141b12450", fn_141b12450 as *const ()), // bigfish::BehaviorTreeChargeAttackNode::bigfish::BehaviorTreeChargeAttackNode::vfunc_11
    Replacement::draft(0x141b124e0, "bigfish_141b124e0", fn_141b124e0 as *const ()), // bigfish::BehaviorTreeChargeAttackNode::bigfish::BehaviorTreeChargeAttackNode::vfunc_10
    Replacement::draft(0x141b12b40, "bigfish_141b12b40", fn_141b12b40 as *const ()), // bigfish::BehaviorTreeChargeAttackNode::ClassTypeInfoImplementation::bigfish::BehaviorTreeChargeAttackNode::ClassTypeInfo
    Replacement::draft(0x141b12bf0, "bigfish_141b12bf0", fn_141b12bf0 as *const ()), // bigfish::BehaviorTreeChargeAttackNode::ClassTypeInfoImplementation::bigfish::BehaviorTreeChargeAttackNode::ClassTypeInfo
    Replacement::draft(0x141b12d80, "bigfish_141b12d80", tpl_34835::<0x8, 0x140286960, 0x20, 0x3, 0x4, 0x14240aa00> as *const ()), // sub_141b12d80
    Replacement::draft(0x141b12e00, "bigfish_141b12e00", tpl_34837::<0x140286960, 0x20, 0x14240aa00> as *const ()), // sub_141b12e00
    Replacement::draft(0x141b12e50, "bigfish_141b12e50", tpl_34840::<0x1403575b0, 0x20, 0x3, 0x4, 0x8, 0x14240a730> as *const ()), // sub_141b12e50
    Replacement::draft_throws(0x141b134c0, "bigfish_141b134c0", tpl_94836::<0x1402845f0, 0x1427783d8, 0x14309c488, 0x140254aa0, 0x1424b16a0, 0x140268f50> as *const (), Throws::Discard), // sub_141b134c0
    Replacement::draft_throws(0x141b135f0, "bigfish_141b135f0", fn_141b135f0 as *const (), Throws::Caught), // sub_141b135f0
    Replacement::draft_throws(0x141b137f0, "bigfish_141b137f0", fn_141b137f0 as *const (), Throws::Discard), // sub_141b137f0
    Replacement::draft_throws(0x141b139d0, "bigfish_141b139d0", fn_141b139d0 as *const (), Throws::Discard), // sub_141b139d0
    Replacement::draft_throws(0x141b13a50, "bigfish_141b13a50", fn_141b13a50 as *const (), Throws::Discard), // sub_141b13a50
    Replacement::draft_throws(0x141b13f80, "bigfish_141b13f80", fn_141b13f80 as *const (), Throws::Discard), // sub_141b13f80
    Replacement::draft_throws(0x141b14100, "bigfish_141b14100", fn_141b14100 as *const (), Throws::Discard), // sub_141b14100
    Replacement::draft_throws(0x141b144a0, "bigfish_141b144a0", fn_141b144a0 as *const (), Throws::Discard), // sub_141b144a0
    Replacement::draft_throws(0x141b14560, "bigfish_141b14560", fn_141b14560 as *const (), Throws::Caught), // sub_141b14560
    Replacement::draft_throws(0x141b14830, "bigfish_141b14830", fn_141b14830 as *const (), Throws::Discard), // sub_141b14830
    Replacement::draft_throws(0x141b14b20, "bigfish_141b14b20", fn_141b14b20 as *const (), Throws::Discard), // sub_141b14b20
    Replacement::draft_throws(0x141b14cc0, "bigfish_141b14cc0", fn_141b14cc0 as *const (), Throws::Discard), // sub_141b14cc0
    Replacement::draft_throws(0x141b14d20, "bigfish_141b14d20", fn_141b14d20 as *const (), Throws::Discard), // sub_141b14d20
    Replacement::draft_throws(0x141b14d80, "bigfish_141b14d80", fn_141b14d80 as *const (), Throws::Discard), // sub_141b14d80
    Replacement::draft_throws(0x141b14eb0, "bigfish_141b14eb0", fn_141b14eb0 as *const (), Throws::Discard), // sub_141b14eb0
    Replacement::draft_throws(0x141b14f60, "bigfish_141b14f60", fn_141b14f60 as *const (), Throws::Discard), // sub_141b14f60
    Replacement::draft_throws(0x141b14ff0, "bigfish_141b14ff0", fn_141b14ff0 as *const (), Throws::Caught), // sub_141b14ff0
    Replacement::draft(0x141b151e0, "bigfish_141b151e0", fn_141b151e0 as *const ()), // sub_141b151e0
    Replacement::draft(0x141b153a0, "bigfish_141b153a0", fn_141b153a0 as *const ()), // sub_141b153a0
    Replacement::draft(0x141b154a0, "bigfish_141b154a0", fn_141b154a0 as *const ()), // sub_141b154a0
    Replacement::draft_throws(0x141b15800, "bigfish_141b15800", fn_141b15800 as *const (), Throws::Discard), // sub_141b15800
    Replacement::draft(0x141b15b10, "bigfish_141b15b10", fn_141b15b10 as *const ()), // sub_141b15b10
    Replacement::draft(0x141b15c20, "bigfish_141b15c20", fn_141b15c20 as *const ()), // sub_141b15c20
    Replacement::draft(0x141b16100, "bigfish_141b16100", fn_141b16100 as *const ()), // sub_141b16100
    Replacement::draft(0x141b166f0, "bigfish_141b166f0", fn_141b166f0 as *const ()), // sub_141b166f0
    Replacement::draft(0x141b16a20, "bigfish_141b16a20", fn_141b16a20 as *const ()), // sub_141b16a20
    Replacement::draft(0x141b16a90, "bigfish_141b16a90", fn_141b16a90 as *const ()), // sub_141b16a90
    Replacement::draft(0x141b16ac0, "bigfish_141b16ac0", fn_141b16ac0 as *const ()), // sub_141b16ac0
    Replacement::draft(0x141b16af0, "bigfish_141b16af0", fn_141b16af0 as *const ()), // sub_141b16af0
    Replacement::draft(0x141b16c30, "bigfish_141b16c30", tpl_95486::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_141b16c30
    Replacement::draft(0x141b16d10, "bigfish_141b16d10", tpl_35718::<0x8, 0xc, 0x10, 0x18, 0x1c> as *const ()), // sub_141b16d10
    Replacement::draft(0x141b16d70, "bigfish_141b16d70", tpl_35719::<0x10, 0x1401de830> as *const ()), // sub_141b16d70
    Replacement::draft(0x141b16fc0, "bigfish_141b16fc0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c9d61c, 0x158, 0x14020e8f0> as *const ()), // sub_141b16fc0
    Replacement::draft(0x141b170c0, "bigfish_141b170c0", fn_141b170c0 as *const ()), // sub_141b170c0
    Replacement::draft(0x141b17140, "bigfish_141b17140", fn_141b17140 as *const ()), // sub_141b17140
    Replacement::draft_throws(0x141b17540, "bigfish_141b17540", fn_141b17540 as *const (), Throws::Discard), // sub_141b17540
    Replacement::draft(0x141b17900, "bigfish_141b17900", fn_141b17900 as *const ()), // sub_141b17900
    Replacement::draft(0x141b17a70, "bigfish_141b17a70", fn_141b17a70 as *const ()), // sub_141b17a70
    Replacement::draft(0x141b17b20, "bigfish_141b17b20", fn_141b17b20 as *const ()), // sub_141b17b20
    Replacement::draft(0x141b17c90, "bigfish_141b17c90", fn_141b17c90 as *const ()), // sub_141b17c90
    Replacement::draft_throws(0x141b17d20, "bigfish_141b17d20", fn_141b17d20 as *const (), Throws::Discard), // sub_141b17d20
    Replacement::draft_throws(0x141b186a0, "bigfish_141b186a0", fn_141b186a0 as *const (), Throws::Caught), // sub_141b186a0
    Replacement::draft(0x141b18a40, "bigfish_141b18a40", fn_141b18a40 as *const ()), // sub_141b18a40
    Replacement::draft_throws(0x141b18ee0, "bigfish_141b18ee0", fn_141b18ee0 as *const (), Throws::Discard), // sub_141b18ee0
    Replacement::draft(0x141b19280, "bigfish_141b19280", fn_141b19280 as *const ()), // sub_141b19280
    Replacement::draft(0x141b19660, "bigfish_141b19660", fn_141b19660 as *const ()), // sub_141b19660
    Replacement::draft(0x141b19a50, "luau_141b19a50", fn_141b19a50 as *const ()), // sub_141b19a50
    Replacement::draft(0x141b19d60, "bigfish_141b19d60", fn_141b19d60 as *const ()), // sub_141b19d60
    Replacement::draft_throws(0x141b19df0, "bigfish_141b19df0", fn_141b19df0 as *const (), Throws::Caught), // sub_141b19df0
    Replacement::draft(0x141b1a4f0, "bigfish_141b1a4f0", fn_141b1a4f0 as *const ()), // sub_141b1a4f0
    Replacement::draft(0x141b1a700, "unknown_141b1a700", tpl_35056::<0x1405f0b90> as *const ()), // sub_141b1a700
    Replacement::draft(0x141b1a720, "unknown_141b1a720", tpl_35322 as *const ()), // sub_141b1a720
    Replacement::draft(0x141b1a760, "unknown_141b1a760", tpl_34318 as *const ()), // sub_141b1a760
    Replacement::draft(0x141b1a780, "unknown_141b1a780", tpl_34318 as *const ()), // sub_141b1a780
    Replacement::draft(0x141b1a7a0, "unknown_141b1a7a0", tpl_34318 as *const ()), // sub_141b1a7a0
    Replacement::draft(0x141b1a7c0, "unknown_141b1a7c0", tpl_34318 as *const ()), // sub_141b1a7c0
    Replacement::draft(0x141b1a7e0, "unknown_141b1a7e0", tpl_34319::<0x143cba740, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b1a7e0
    Replacement::draft(0x141b1a840, "unknown_141b1a840", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b1a840
    Replacement::draft(0x141b1a8a0, "unknown_141b1a8a0", tpl_34319::<0x143c8f4d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b1a8a0
    Replacement::draft(0x141b1a900, "unknown_141b1a900", tpl_34319::<0x143ca78cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b1a900
    Replacement::draft(0x141b1a960, "unknown_141b1a960", tpl_34318 as *const ()), // sub_141b1a960
    Replacement::draft(0x141b1a980, "unknown_141b1a980", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b1a980
    Replacement::draft(0x141b1a9e0, "unknown_141b1a9e0", tpl_496 as *const ()), // sub_141b1a9e0
    Replacement::draft(0x141b1a9f0, "unknown_141b1a9f0", tpl_496 as *const ()), // sub_141b1a9f0
    Replacement::draft(0x141b1aa00, "unknown_141b1aa00", tpl_496 as *const ()), // sub_141b1aa00
    Replacement::draft(0x141b1aa10, "unknown_141b1aa10", tpl_496 as *const ()), // sub_141b1aa10
    Replacement::draft(0x141b1aa20, "unknown_141b1aa20", tpl_496 as *const ()), // sub_141b1aa20
    Replacement::draft(0x141b1aa30, "unknown_141b1aa30", tpl_361 as *const ()), // sub_141b1aa30
    Replacement::draft_throws(0x141b1aa40, "unknown_141b1aa40", fn_141b1aa40 as *const (), Throws::Caught), // sub_141b1aa40
    Replacement::draft(0x141b1b300, "unknown_141b1b300", tpl_1439::<0x8, 0x1405ef830> as *const ()), // sub_141b1b300
    Replacement::draft_throws(0x141b1b340, "unknown_141b1b340", fn_141b1b340 as *const (), Throws::Discard), // sub_141b1b340
    Replacement::draft_throws(0x141b1b4f0, "unknown_141b1b4f0", fn_141b1b4f0 as *const (), Throws::Discard), // sub_141b1b4f0
    Replacement::draft(0x141b1ba40, "unknown_141b1ba40", tpl_361 as *const ()), // sub_141b1ba40
    Replacement::draft(0x141b1ba50, "unknown_141b1ba50", tpl_34318 as *const ()), // sub_141b1ba50
    Replacement::draft(0x141b1ba70, "unknown_141b1ba70", tpl_35063::<0x143c8f4d4, 0x18068, 0x18050, 0x142489ee0> as *const ()), // sub_141b1ba70
    Replacement::draft(0x141b1bac0, "unknown_141b1bac0", tpl_34319::<0x143ca78cc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b1bac0
    Replacement::draft(0x141b1bb20, "unknown_141b1bb20", tpl_496 as *const ()), // sub_141b1bb20
    Replacement::draft_throws(0x141b1bb30, "std_141b1bb30", fn_141b1bb30 as *const (), Throws::Caught), // sub_141b1bb30
    Replacement::draft(0x141b1c720, "unknown_141b1c720", tpl_361 as *const ()), // sub_141b1c720
    Replacement::draft_throws(0x141b1c730, "unknown_141b1c730", fn_141b1c730 as *const (), Throws::Caught), // sub_141b1c730
    Replacement::draft_throws(0x141b1c980, "unknown_141b1c980", fn_141b1c980 as *const (), Throws::Caught), // sub_141b1c980
    Replacement::draft(0x141b1ca80, "unknown_141b1ca80", fn_141b1ca80 as *const ()), // sub_141b1ca80
    Replacement::draft(0x141b1caf0, "unknown_141b1caf0", fn_141b1caf0 as *const ()), // sub_141b1caf0
    Replacement::draft_throws(0x141b1cb80, "unknown_141b1cb80", fn_141b1cb80 as *const (), Throws::Discard), // sub_141b1cb80
    Replacement::draft(0x141b1ce00, "unknown_141b1ce00", fn_141b1ce00 as *const ()), // sub_141b1ce00
    Replacement::draft_throws(0x141b1ce30, "unknown_141b1ce30", fn_141b1ce30 as *const (), Throws::Discard), // sub_141b1ce30
    Replacement::draft(0x141b1d110, "unknown_141b1d110", fn_141b1d110 as *const ()), // sub_141b1d110
    Replacement::draft_throws(0x141b1d140, "unknown_141b1d140", fn_141b1d140 as *const (), Throws::Discard), // sub_141b1d140
    Replacement::draft(0x141b1d160, "unknown_141b1d160", fn_141b1d160 as *const ()), // sub_141b1d160
    Replacement::draft(0x141b1d190, "unknown_141b1d190", fn_141b1d190 as *const ()), // sub_141b1d190
    Replacement::draft_throws(0x141b1d2f0, "unknown_141b1d2f0", fn_141b1d2f0 as *const (), Throws::Discard), // sub_141b1d2f0
    Replacement::draft(0x141b1da90, "unknown_141b1da90", tpl_34994::<0x143cbab60, 0x14387bf98, 0x14387bf90, 0x141834a78, 0x1425f7690, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b1da90
    Replacement::draft(0x141b1db00, "bigfish_141b1db00", tpl_34605::<0x1431f1d70> as *const ()), // sub_141b1db00
    Replacement::draft_throws(0x141b1e020, "bigfish_141b1e020", fn_141b1e020 as *const (), Throws::Discard), // sub_141b1e020
    Replacement::draft_throws(0x141b1e1b0, "bigfish_141b1e1b0", fn_141b1e1b0 as *const (), Throws::Discard), // sub_141b1e1b0
    Replacement::draft(0x141b1e3e0, "bigfish_141b1e3e0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cbaa18, 0x158, 0x14020e8f0> as *const ()), // sub_141b1e3e0
    Replacement::draft(0x141b1e4e0, "bigfish_141b1e4e0", tpl_35630::<0x10, 0x8, 0x1405f4220> as *const ()), // sub_141b1e4e0
    Replacement::draft_throws(0x141b1e540, "bigfish_141b1e540", fn_141b1e540 as *const (), Throws::Discard), // sub_141b1e540
    Replacement::draft_throws(0x141b1e5d0, "bigfish_141b1e5d0", fn_141b1e5d0 as *const (), Throws::Discard), // sub_141b1e5d0
    Replacement::draft_throws(0x141b1e600, "bigfish_141b1e600", tpl_37191::<0x14387f4d8, 0x1405f46c0, 0x1402116e0, 0x14277e210, 0x1410d6ca0, 0x1405f46f0, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1405f49d0, 0x143823788, 0x14277e220, 0x1405f3ed0, 0x143823a58> as *const (), Throws::Discard), // sub_141b1e600
    Replacement::draft(0x141b1ea50, "bigfish_141b1ea50", fn_141b1ea50 as *const ()), // sub_141b1ea50
    Replacement::draft(0x141b1eb90, "bigfish_141b1eb90", fn_141b1eb90 as *const ()), // sub_141b1eb90
    Replacement::draft(0x141b1ed00, "bigfish_141b1ed00", fn_141b1ed00 as *const ()), // sub_141b1ed00
    Replacement::draft(0x141b1f180, "bigfish_141b1f180", fn_141b1f180 as *const ()), // sub_141b1f180
    Replacement::draft(0x141b1f3c0, "bigfish_141b1f3c0", fn_141b1f3c0 as *const ()), // sub_141b1f3c0
    Replacement::draft(0x141b1f460, "bigfish_141b1f460", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cbade0, 0x158, 0x14020e8f0> as *const ()), // sub_141b1f460
    Replacement::draft(0x141b1f560, "bigfish_141b1f560", fn_141b1f560 as *const ()), // sub_141b1f560
    Replacement::draft(0x141b1f6e0, "bigfish_141b1f6e0", fn_141b1f6e0 as *const ()), // sub_141b1f6e0
    Replacement::draft_throws(0x141b1f800, "bigfish_141b1f800", fn_141b1f800 as *const (), Throws::Discard), // sub_141b1f800
    Replacement::draft(0x141b1fe50, "bigfish_141b1fe50", tpl_94428::<0x1431f4068, 0x8> as *const ()), // sub_141b1fe50
    Replacement::draft_throws(0x141b20090, "bigfish_141b20090", fn_141b20090 as *const (), Throws::Discard), // sub_141b20090
    Replacement::draft_throws(0x141b201f0, "bigfish_141b201f0", fn_141b201f0 as *const (), Throws::Discard), // sub_141b201f0
    Replacement::draft_throws(0x141b20300, "bigfish_141b20300", fn_141b20300 as *const (), Throws::Discard), // sub_141b20300
    Replacement::draft_throws(0x141b203c0, "bigfish_141b203c0", fn_141b203c0 as *const (), Throws::Discard), // sub_141b203c0
    Replacement::draft_throws(0x141b20470, "bigfish_141b20470", fn_141b20470 as *const (), Throws::Discard), // sub_141b20470
    Replacement::draft(0x141b20a30, "bigfish_141b20a30", fn_141b20a30 as *const ()), // sub_141b20a30
    Replacement::draft_throws(0x141b20e10, "bigfish_141b20e10", tpl_37596::<0x142780e00, 0x1410d6ca0, 0x1405fb210, 0x143823b98, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_141b20e10
    Replacement::draft(0x141b20fe0, "bigfish_141b20fe0", fn_141b20fe0 as *const ()), // sub_141b20fe0
    Replacement::draft_throws(0x141b21000, "bigfish_141b21000", tpl_35009::<0x14387e980, 0x1405fbd00, 0x1402116e0> as *const (), Throws::Discard), // sub_141b21000
    Replacement::draft(0x141b21040, "bigfish_141b21040", fn_141b21040 as *const ()), // sub_141b21040
    Replacement::draft(0x141b25ab0, "bigfish_141b25ab0", tpl_96964::<0x1431f5318, 0x8> as *const ()), // sub_141b25ab0
    Replacement::draft_throws(0x141b26d20, "bigfish_141b26d20", fn_141b26d20 as *const (), Throws::Discard), // sub_141b26d20
    Replacement::draft(0x141b270c0, "bigfish_141b270c0", fn_141b270c0 as *const ()), // sub_141b270c0
    Replacement::draft(0x141b27120, "bigfish_141b27120", fn_141b27120 as *const ()), // sub_141b27120
    Replacement::draft(0x141b281a0, "bigfish_141b281a0", fn_141b281a0 as *const ()), // sub_141b281a0
    Replacement::draft_throws(0x141b282e0, "bigfish_141b282e0", fn_141b282e0 as *const (), Throws::Discard), // sub_141b282e0
    Replacement::draft(0x141b28500, "rapidjson_141b28500", fn_141b28500 as *const ()), // sub_141b28500
    Replacement::draft_throws(0x141b28620, "bigfish_141b28620", fn_141b28620 as *const (), Throws::Discard), // sub_141b28620
    Replacement::draft(0x141b28ad0, "bigfish_141b28ad0", fn_141b28ad0 as *const ()), // sub_141b28ad0
    Replacement::draft(0x141b28d20, "bigfish_141b28d20", tpl_96964::<0x1431f8350, 0x8> as *const ()), // sub_141b28d20
    Replacement::draft_throws(0x141b29190, "bigfish_141b29190", fn_141b29190 as *const (), Throws::Discard), // sub_141b29190
    Replacement::draft(0x141b29320, "bigfish_141b29320", tpl_5985::<0x8, 0x140600880, 0x140600a80> as *const ()), // sub_141b29320
    Replacement::draft(0x141b293a0, "bigfish_141b293a0", fn_141b293a0 as *const ()), // sub_141b293a0
    Replacement::draft_throws(0x141b293b0, "bigfish_141b293b0", tpl_36792::<0x14387f030, 0x140600d60, 0x1402116e0, 0x1427838c0, 0x18, 0x1410d6ca0, 0x10, 0x140600670> as *const (), Throws::Discard), // sub_141b293b0
    Replacement::draft_throws(0x141b29440, "bigfish_141b29440", tpl_37607::<0xc0, 0xa8, 0x78, 0x141bd6940, 0x3b, 0x84, 0x8, 0x85, 0x39, 0x18, 0x142783f00, 0x60, 0x140241da0> as *const (), Throws::Discard), // sub_141b29440
    Replacement::draft_throws(0x141b294e0, "bigfish_141b294e0", tpl_37608::<0xc0, 0xa8, 0x78, 0x141bd6940, 0x84, 0x5, 0x4, 0x39, 0x85, 0x18, 0x142783f00, 0x60, 0x140241da0> as *const (), Throws::Discard), // sub_141b294e0
    Replacement::draft(0x141b295b0, "bigfish_141b295b0", fn_141b295b0 as *const ()), // bigfish::AbilityChargeAttackProgram::bigfish::AbilityChargeAttackProgram::vfunc_16
    Replacement::draft(0x141b29630, "bigfish_141b29630", tpl_37610::<0xa0, 0xa8> as *const ()), // bigfish::AbilityChargeAttackProgram::bigfish::AbilityChargeAttackProgram::vfunc_15
    Replacement::draft(0x141b296a0, "bigfish_141b296a0", tpl_37611::<0x3a, 0x1e> as *const ()), // bigfish::AbilityChargeAttackProgram::bigfish::AbilityChargeAttackProgram::vfunc_7
    Replacement::draft_throws(0x141b296b0, "bigfish_141b296b0", tpl_37612::<0xa0, 0xa8, 0x141b29440> as *const (), Throws::Discard), // bigfish::AbilityChargeAttackProgram::bigfish::AbilityChargeAttackProgram::vfunc_6
    Replacement::draft_throws(0x141b29720, "bigfish_141b29720", tpl_37612::<0xa0, 0xa8, 0x141b29440> as *const (), Throws::Discard), // bigfish::AbilityChargeAttackProgram::bigfish::AbilityChargeAttackProgram::vfunc_5
    Replacement::draft_throws(0x141b29790, "bigfish_141b29790", tpl_37613::<0xa0, 0xa8, 0x141b29810> as *const (), Throws::Discard), // bigfish::AbilityChargeAttackProgram::bigfish::AbilityChargeAttackProgram::vfunc_1
    Replacement::draft_throws(0x141b29810, "unknown_141b29810", fn_141b29810 as *const (), Throws::Discard), // sub_141b29810
    Replacement::draft(0x141b29970, "unknown_141b29970", tpl_37615::<0x141ba4aa0, 0x38, 0x1431f84c8, 0x3c, 0x1430f6b98, 0x44, 0x48, 0x50, 0x58, 0x8, 0x60, 0xc, 0x1431eaeb8, 0x1431f8560> as *const ()), // sub_141b29970
    Replacement::draft(0x141b29b30, "unknown_141b29b30", tpl_94819::<0x1431fb5a0, 0x8, 0xc, 0x10> as *const ()), // sub_141b29b30
    Replacement::draft(0x141b29be0, "unknown_141b29be0", tpl_35039::<0x1431fb5a0, 0x8, 0xc, 0x10> as *const ()), // sub_141b29be0
    Replacement::draft(0x141b29cd0, "unknown_141b29cd0", fn_141b29cd0 as *const ()), // sub_141b29cd0
    Replacement::draft(0x141b29dc0, "unknown_141b29dc0", fn_141b29dc0 as *const ()), // sub_141b29dc0
    Replacement::draft(0x141b29e70, "unknown_141b29e70", tpl_19401::<0x8, 0x141b29ef0> as *const ()), // sub_141b29e70
    Replacement::draft(0x141b29eb0, "unknown_141b29eb0", tpl_19401::<0x8, 0x141b29f70> as *const ()), // sub_141b29eb0
    Replacement::draft(0x141b29ef0, "unknown_141b29ef0", tpl_2432::<0x8, 0x141b2a1b0, 0xc, 0x14183acc8> as *const ()), // sub_141b29ef0
    Replacement::draft(0x141b29f70, "unknown_141b29f70", tpl_2424::<0x8, 0x141b2a2a0, 0xc, 0x14183acc8> as *const ()), // sub_141b29f70
    Replacement::draft(0x141b29ff0, "unknown_141b29ff0", tpl_37617::<0x141b2ab20, 0x8, 0x14183acc8, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141b29ff0
    Replacement::draft(0x141b2a0d0, "unknown_141b2a0d0", fn_141b2a0d0 as *const ()), // sub_141b2a0d0
    Replacement::draft(0x141b2a1b0, "unknown_141b2a1b0", tpl_61669::<0xc, 0x141b2ab20, 0x8, 0x14183acc8, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141b2a1b0
    Replacement::draft(0x141b2a2a0, "unknown_141b2a2a0", fn_141b2a2a0 as *const ()), // sub_141b2a2a0
    Replacement::draft(0x141b2a3a0, "unknown_141b2a3a0", tpl_2880::<0x141b2b300, 0xc, 0x8, 0x141b2a1b0, 0x14183acc8> as *const ()), // sub_141b2a3a0
    Replacement::draft(0x141b2a460, "unknown_141b2a460", fn_141b2a460 as *const ()), // sub_141b2a460
    Replacement::draft(0x141b2a520, "unknown_141b2a520", tpl_21339::<0x141b2ab20, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_141b2a520
    Replacement::draft(0x141b2a6c0, "unknown_141b2a6c0", tpl_2937::<0x141b2ac40, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_141b2a6c0
    Replacement::draft(0x141b2afc0, "unknown_141b2afc0", fn_141b2afc0 as *const ()), // sub_141b2afc0
    Replacement::draft(0x141b2afe0, "unknown_141b2afe0", fn_141b2afe0 as *const ()), // sub_141b2afe0
    Replacement::draft(0x141b2b180, "unknown_141b2b180", tpl_37622::<0x10, 0x18> as *const ()), // sub_141b2b180
    Replacement::draft(0x141b2b1a0, "unknown_141b2b1a0", tpl_37622::<0x20, 0x28> as *const ()), // sub_141b2b1a0
    Replacement::draft(0x141b2b3b0, "unknown_141b2b3b0", fn_141b2b3b0 as *const ()), // sub_141b2b3b0
    Replacement::draft(0x141b2b500, "unknown_141b2b500", fn_141b2b500 as *const ()), // sub_141b2b500
    Replacement::draft(0x141b2b650, "unknown_141b2b650", fn_141b2b650 as *const ()), // sub_141b2b650
    Replacement::draft(0x141b2b730, "unknown_141b2b730", fn_141b2b730 as *const ()), // sub_141b2b730
    Replacement::draft(0x141b2b810, "unknown_141b2b810", tpl_37625::<0x20> as *const ()), // sub_141b2b810
    Replacement::draft(0x141b2b860, "unknown_141b2b860", tpl_37625::<0x30> as *const ()), // sub_141b2b860
    Replacement::draft(0x141b2b890, "unknown_141b2b890", tpl_35031::<0x142407c80, 0x143c977a4, 0x14020e8f0> as *const ()), // sub_141b2b890
    Replacement::draft(0x141b2b9c0, "unknown_141b2b9c0", tpl_35031::<0x142407c80, 0x143cbbc0c, 0x14020e8f0> as *const ()), // sub_141b2b9c0
    Replacement::draft(0x141b2baf0, "unknown_141b2baf0", fn_141b2baf0 as *const ()), // sub_141b2baf0
    Replacement::draft(0x141b2bb80, "unknown_141b2bb80", fn_141b2bb80 as *const ()), // sub_141b2bb80
    Replacement::draft(0x141b2bc40, "unknown_141b2bc40", tpl_5994::<0x8, 0x140602c80, 0x10, 0x14384d970, 0x14384d96c, 0x14, 0x14384d8f0, 0x14384d8ec, 0x18, 0x14384d950, 0x14384d94c> as *const ()), // sub_141b2bc40
    Replacement::draft_throws(0x141b2bcd0, "unknown_141b2bcd0", fn_141b2bcd0 as *const (), Throws::Discard), // sub_141b2bcd0
    Replacement::draft(0x141b2c0d0, "unknown_141b2c0d0", fn_141b2c0d0 as *const ()), // sub_141b2c0d0
    Replacement::draft_throws(0x141b2c1a0, "unknown_141b2c1a0", fn_141b2c1a0 as *const (), Throws::Discard), // sub_141b2c1a0
    Replacement::draft(0x141b2c340, "unknown_141b2c340", fn_141b2c340 as *const ()), // sub_141b2c340
    Replacement::draft(0x141b2cb50, "unknown_141b2cb50", fn_141b2cb50 as *const ()), // sub_141b2cb50
    Replacement::draft(0x141b2d010, "unknown_141b2d010", fn_141b2d010 as *const ()), // sub_141b2d010
    Replacement::draft(0x141b2d2f0, "unknown_141b2d2f0", fn_141b2d2f0 as *const ()), // sub_141b2d2f0
    Replacement::draft(0x141b2d3b0, "unknown_141b2d3b0", fn_141b2d3b0 as *const ()), // sub_141b2d3b0
    Replacement::draft_throws(0x141b2d3d0, "unknown_141b2d3d0", fn_141b2d3d0 as *const (), Throws::Discard), // sub_141b2d3d0
    Replacement::draft_throws(0x141b2d720, "unknown_141b2d720", fn_141b2d720 as *const (), Throws::Discard), // sub_141b2d720
    Replacement::draft_throws(0x141b2dbe0, "unknown_141b2dbe0", fn_141b2dbe0 as *const (), Throws::Discard), // sub_141b2dbe0
    Replacement::draft_throws(0x141b2e090, "unknown_141b2e090", fn_141b2e090 as *const (), Throws::Discard), // sub_141b2e090
    Replacement::draft_throws(0x141b2e150, "unknown_141b2e150", fn_141b2e150 as *const (), Throws::Discard), // sub_141b2e150
    Replacement::draft_throws(0x141b2e1c0, "unknown_141b2e1c0", fn_141b2e1c0 as *const (), Throws::Discard), // sub_141b2e1c0
    Replacement::draft(0x141b2e9d0, "unknown_141b2e9d0", tpl_34611::<0x8> as *const ()), // sub_141b2e9d0
    Replacement::draft(0x141b2ea60, "unknown_141b2ea60", fn_141b2ea60 as *const ()), // sub_141b2ea60
    Replacement::draft(0x141b2ec60, "unknown_141b2ec60", fn_141b2ec60 as *const ()), // sub_141b2ec60
    Replacement::draft(0x141b2f420, "unknown_141b2f420", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cbc044, 0x158, 0x14020e8f0> as *const ()), // sub_141b2f420
    Replacement::draft_throws(0x141b2f520, "unknown_141b2f520", fn_141b2f520 as *const (), Throws::Caught), // sub_141b2f520
    Replacement::draft(0x141b2f6b0, "unknown_141b2f6b0", fn_141b2f6b0 as *const ()), // sub_141b2f6b0
    Replacement::draft(0x141b2f860, "unknown_141b2f860", tpl_361 as *const ()), // sub_141b2f860
    Replacement::draft_throws(0x141b2f870, "unknown_141b2f870", fn_141b2f870 as *const (), Throws::Caught), // sub_141b2f870
    Replacement::draft(0x141b2fe20, "unknown_141b2fe20", tpl_361 as *const ()), // sub_141b2fe20
    Replacement::draft(0x141b2fe30, "unknown_141b2fe30", tpl_361 as *const ()), // sub_141b2fe30
    Replacement::draft_throws(0x141b2fe40, "unknown_141b2fe40", fn_141b2fe40 as *const (), Throws::Discard), // sub_141b2fe40
    Replacement::draft_throws(0x141b308d0, "unknown_141b308d0", fn_141b308d0 as *const (), Throws::Discard), // sub_141b308d0
    Replacement::draft_throws(0x141b30da0, "unknown_141b30da0", fn_141b30da0 as *const (), Throws::Discard), // sub_141b30da0
    Replacement::draft_throws(0x141b30f30, "unknown_141b30f30", fn_141b30f30 as *const (), Throws::Discard), // sub_141b30f30
    Replacement::draft_throws(0x141b31010, "unknown_141b31010", fn_141b31010 as *const (), Throws::Discard), // sub_141b31010
    Replacement::draft(0x141b31080, "unknown_141b31080", tpl_361 as *const ()), // sub_141b31080
    Replacement::draft(0x141b31090, "unknown_141b31090", fn_141b31090 as *const ()), // sub_141b31090
    Replacement::draft(0x141b312a0, "unknown_141b312a0", fn_141b312a0 as *const ()), // sub_141b312a0
    Replacement::draft_throws(0x141b31420, "unknown_141b31420", fn_141b31420 as *const (), Throws::Discard), // sub_141b31420
    Replacement::draft(0x141b31910, "unknown_141b31910", tpl_34318 as *const ()), // sub_141b31910
    Replacement::draft(0x141b31930, "unknown_141b31930", tpl_34319::<0x143c8fa04, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31930
    Replacement::draft(0x141b31990, "unknown_141b31990", tpl_34318 as *const ()), // sub_141b31990
    Replacement::draft(0x141b319b0, "unknown_141b319b0", tpl_34318 as *const ()), // sub_141b319b0
    Replacement::draft(0x141b319d0, "unknown_141b319d0", tpl_34318 as *const ()), // sub_141b319d0
    Replacement::draft(0x141b319f0, "unknown_141b319f0", tpl_34318 as *const ()), // sub_141b319f0
    Replacement::draft(0x141b31a10, "unknown_141b31a10", tpl_34318 as *const ()), // sub_141b31a10
    Replacement::draft(0x141b31a30, "unknown_141b31a30", tpl_34318 as *const ()), // sub_141b31a30
    Replacement::draft(0x141b31a50, "unknown_141b31a50", tpl_34318 as *const ()), // sub_141b31a50
    Replacement::draft(0x141b31a70, "unknown_141b31a70", tpl_34318 as *const ()), // sub_141b31a70
    Replacement::draft(0x141b31a90, "unknown_141b31a90", tpl_34319::<0x143c8de68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31a90
    Replacement::draft(0x141b31af0, "unknown_141b31af0", tpl_34319::<0x143c919a8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31af0
    Replacement::draft(0x141b31b50, "unknown_141b31b50", tpl_34319::<0x143c8f4d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31b50
    Replacement::draft(0x141b31bb0, "unknown_141b31bb0", tpl_34319::<0x143ca7874, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31bb0
    Replacement::draft(0x141b31c10, "unknown_141b31c10", tpl_34319::<0x143c8ff58, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31c10
    Replacement::draft(0x141b31c70, "unknown_141b31c70", tpl_34319::<0x143c8fe90, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31c70
    Replacement::draft(0x141b31cd0, "unknown_141b31cd0", tpl_34319::<0x143c8fab8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31cd0
    Replacement::draft(0x141b31d30, "unknown_141b31d30", tpl_34319::<0x143c8f0d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31d30
    Replacement::draft(0x141b31d90, "unknown_141b31d90", tpl_34318 as *const ()), // sub_141b31d90
    Replacement::draft(0x141b31db0, "unknown_141b31db0", tpl_34318 as *const ()), // sub_141b31db0
    Replacement::draft(0x141b31dd0, "unknown_141b31dd0", tpl_34318 as *const ()), // sub_141b31dd0
    Replacement::draft(0x141b31df0, "unknown_141b31df0", tpl_34318 as *const ()), // sub_141b31df0
    Replacement::draft(0x141b31e10, "unknown_141b31e10", tpl_34319::<0x143c8ec20, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31e10
    Replacement::draft(0x141b31e70, "unknown_141b31e70", tpl_34319::<0x143c8e840, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31e70
    Replacement::draft(0x141b31ed0, "unknown_141b31ed0", tpl_34319::<0x143c9dc74, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31ed0
    Replacement::draft(0x141b31f30, "unknown_141b31f30", tpl_34319::<0x143c9d8d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31f30
    Replacement::draft(0x141b31f90, "unknown_141b31f90", tpl_34318 as *const ()), // sub_141b31f90
    Replacement::draft(0x141b31fb0, "unknown_141b31fb0", tpl_34318 as *const ()), // sub_141b31fb0
    Replacement::draft(0x141b31fd0, "unknown_141b31fd0", tpl_34319::<0x143ca7dbc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b31fd0
    Replacement::draft(0x141b32030, "unknown_141b32030", tpl_34319::<0x143c98fc0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b32030
    Replacement::draft(0x141b32090, "std_141b32090", tpl_23342::<0x1432007e8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b320b0, "std_141b320b0", tpl_23342::<0x1432007e8, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b320d0, "std_141b320d0", fn_141b320d0 as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b320f0, "std_141b320f0", tpl_1::<0x14388e4e0> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32100, "std_141b32100", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32110, "std_141b32110", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_2>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32130, "std_141b32130", tpl_23342::<0x143200858, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_1>,bool,m::?bref9,ecs::AliveEntityHandle>::s
    Replacement::draft(0x141b32150, "std_141b32150", tpl_23342::<0x143200858, 0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_1>,bool,m::?bref9,ecs::AliveEntityHandle>::s
    Replacement::draft(0x141b32170, "std_141b32170", fn_141b32170 as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_1>,bool,m::?bref9,ecs::AliveEntityHandle>::s
    Replacement::draft(0x141b321a0, "std_141b321a0", tpl_1::<0x14388df20> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_1>,bool,m::?bref9,ecs::AliveEntityHandle>::s
    Replacement::draft(0x141b321b0, "std_141b321b0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_1>,bool,m::?bref9,ecs::AliveEntityHandle>::s
    Replacement::draft(0x141b321c0, "std_141b321c0", tpl_671::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_1>,bool,m::?bref9,ecs::AliveEntityHandle>::s
    Replacement::draft(0x141b321e0, "std_141b321e0", tpl_770::<0x1431fdbe8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32200, "std_141b32200", tpl_770::<0x1431fdbe8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32220, "std_141b32220", tpl_37646 as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32230, "std_141b32230", tpl_1::<0x14388de40> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32240, "std_141b32240", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32250, "std_141b32250", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::applyChunkDamage::<lambda_3>,float,float>::std::_Func_impl_no_alloc<bi
    Replacement::draft(0x141b32270, "std_141b32270", tpl_37647::<0x141834d30> as *const ()), // sub_141b32270
    Replacement::draft(0x141b32290, "std_141b32290", tpl_37647::<0x141834d30> as *const ()), // sub_141b32290
    Replacement::draft_throws(0x141b322b0, "std_141b322b0", fn_141b322b0 as *const (), Throws::Discard), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::handleHitBox::<lambda_1>,void,const coregame::PhysicsSweepHit&,const m
    Replacement::draft(0x141b32310, "std_141b32310", tpl_559 as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::handleHitBox::<lambda_1>,void,const coregame::PhysicsSweepHit&,const m
    Replacement::draft(0x141b32330, "std_141b32330", tpl_1::<0x14388d6c0> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::handleHitBox::<lambda_1>,void,const coregame::PhysicsSweepHit&,const m
    Replacement::draft(0x141b32340, "std_141b32340", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::handleHitBox::<lambda_1>,void,const coregame::PhysicsSweepHit&,const m
    Replacement::draft(0x141b32350, "std_141b32350", fn_141b32350 as *const ()), // std::_Func_impl_no_alloc<bigfish::abilityaimelee::handleHitBox::<lambda_1>,void,const coregame::PhysicsSweepHit&,const m
    Replacement::draft(0x141b32370, "unknown_141b32370", tpl_34318 as *const ()), // sub_141b32370
    Replacement::draft(0x141b32390, "unknown_141b32390", tpl_34318 as *const ()), // sub_141b32390
    Replacement::draft(0x141b323b0, "unknown_141b323b0", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b323b0
    Replacement::draft(0x141b32410, "unknown_141b32410", tpl_34319::<0x143c9d1c4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b32410
    Replacement::draft(0x141b32470, "unknown_141b32470", tpl_34318 as *const ()), // sub_141b32470
    Replacement::draft(0x141b32490, "unknown_141b32490", tpl_34318 as *const ()), // sub_141b32490
    Replacement::draft(0x141b324b0, "unknown_141b324b0", tpl_34319::<0x143c9b928, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b324b0
    Replacement::draft(0x141b32510, "unknown_141b32510", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b32510
    Replacement::draft(0x141b32570, "unknown_141b32570", tpl_1439::<0x8, 0x14060c180> as *const ()), // sub_141b32570
    Replacement::draft(0x141b325b0, "unknown_141b325b0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143c9d8d0, 0x158, 0x14020e8f0> as *const ()), // sub_141b325b0
    Replacement::draft(0x141b326a0, "unknown_141b326a0", tpl_496 as *const ()), // sub_141b326a0
    Replacement::draft_throws(0x141b326b0, "unknown_141b326b0", fn_141b326b0 as *const (), Throws::Discard), // sub_141b326b0
    Replacement::draft(0x141b32e70, "unknown_141b32e70", tpl_361 as *const ()), // sub_141b32e70
    Replacement::draft_throws(0x141b32e80, "unknown_141b32e80", fn_141b32e80 as *const (), Throws::Discard), // sub_141b32e80
    Replacement::draft(0x141b330d0, "unknown_141b330d0", fn_141b330d0 as *const ()), // sub_141b330d0
    Replacement::draft(0x141b335b0, "unknown_141b335b0", tpl_361 as *const ()), // sub_141b335b0
    Replacement::draft(0x141b335c0, "unknown_141b335c0", tpl_496 as *const ()), // sub_141b335c0
    Replacement::draft(0x141b335d0, "unknown_141b335d0", tpl_361 as *const ()), // sub_141b335d0
    Replacement::draft(0x141b335e0, "unknown_141b335e0", tpl_361 as *const ()), // sub_141b335e0
    Replacement::draft(0x141b335f0, "unknown_141b335f0", fn_141b335f0 as *const ()), // sub_141b335f0
    Replacement::draft_throws(0x141b34240, "unknown_141b34240", fn_141b34240 as *const (), Throws::Discard), // sub_141b34240
    Replacement::draft_throws(0x141b34a50, "unknown_141b34a50", fn_141b34a50 as *const (), Throws::Discard), // sub_141b34a50
    Replacement::draft(0x141b34cc0, "unknown_141b34cc0", fn_141b34cc0 as *const ()), // sub_141b34cc0
    Replacement::draft_throws(0x141b354f0, "kaim_141b354f0", fn_141b354f0 as *const (), Throws::Caught), // sub_141b354f0
    Replacement::draft(0x141b35e20, "unknown_141b35e20", fn_141b35e20 as *const ()), // sub_141b35e20
    Replacement::draft(0x141b35e60, "unknown_141b35e60", tpl_361 as *const ()), // sub_141b35e60
    Replacement::draft_throws(0x141b35e70, "unknown_141b35e70", fn_141b35e70 as *const (), Throws::Discard), // sub_141b35e70
    Replacement::draft(0x141b35fa0, "unknown_141b35fa0", fn_141b35fa0 as *const ()), // sub_141b35fa0
    Replacement::draft_throws(0x141b36000, "unknown_141b36000", fn_141b36000 as *const (), Throws::Discard), // sub_141b36000
    Replacement::draft(0x141b36990, "unknown_141b36990", tpl_361 as *const ()), // sub_141b36990
    Replacement::draft_throws(0x141b369a0, "unknown_141b369a0", fn_141b369a0 as *const (), Throws::Caught), // sub_141b369a0
    Replacement::draft(0x141b36ba0, "unknown_141b36ba0", tpl_34998::<0x8, 0x10> as *const ()), // sub_141b36ba0
    Replacement::draft_throws(0x141b36bc0, "unknown_141b36bc0", fn_141b36bc0 as *const (), Throws::Discard), // sub_141b36bc0
    Replacement::draft_throws(0x141b36c90, "unknown_141b36c90", fn_141b36c90 as *const (), Throws::Discard), // sub_141b36c90
    Replacement::draft_throws(0x141b36fb0, "unknown_141b36fb0", fn_141b36fb0 as *const (), Throws::Discard), // sub_141b36fb0
    Replacement::draft_throws(0x141b37010, "unknown_141b37010", fn_141b37010 as *const (), Throws::Discard), // sub_141b37010
    Replacement::draft_throws(0x141b371e0, "unknown_141b371e0", fn_141b371e0 as *const (), Throws::Discard), // sub_141b371e0
    Replacement::draft_throws(0x141b372b0, "unknown_141b372b0", fn_141b372b0 as *const (), Throws::Discard), // sub_141b372b0
    Replacement::draft(0x141b37330, "unknown_141b37330", fn_141b37330 as *const ()), // sub_141b37330
    Replacement::draft_throws(0x141b37370, "unknown_141b37370", fn_141b37370 as *const (), Throws::Discard), // sub_141b37370
    Replacement::draft_throws(0x141b37460, "unknown_141b37460", fn_141b37460 as *const (), Throws::Discard), // sub_141b37460
    Replacement::draft_throws(0x141b377f0, "unknown_141b377f0", fn_141b377f0 as *const (), Throws::Discard), // sub_141b377f0
    Replacement::draft_throws(0x141b379a0, "unknown_141b379a0", fn_141b379a0 as *const (), Throws::Discard), // sub_141b379a0
    Replacement::draft(0x141b37af0, "unknown_141b37af0", fn_141b37af0 as *const ()), // sub_141b37af0
    Replacement::draft(0x141b37bd0, "unknown_141b37bd0", tpl_361 as *const ()), // sub_141b37bd0
    Replacement::draft_throws(0x141b37be0, "unknown_141b37be0", fn_141b37be0 as *const (), Throws::Discard), // sub_141b37be0
    Replacement::draft(0x141b38070, "unknown_141b38070", fn_141b38070 as *const ()), // sub_141b38070
    Replacement::draft(0x141b38080, "unknown_141b38080", tpl_22031::<0x8> as *const ()), // sub_141b38080
    Replacement::draft(0x141b380a0, "unknown_141b380a0", fn_141b380a0 as *const ()), // sub_141b380a0
    Replacement::draft(0x141b380c0, "unknown_141b380c0", tpl_22031::<0x8> as *const ()), // sub_141b380c0
    Replacement::draft(0x141b380e0, "unknown_141b380e0", fn_141b380e0 as *const ()), // sub_141b380e0
    Replacement::draft(0x141b38110, "unknown_141b38110", fn_141b38110 as *const ()), // sub_141b38110
    Replacement::draft(0x141b38360, "unknown_141b38360", fn_141b38360 as *const ()), // sub_141b38360
    Replacement::draft(0x141b383e0, "unknown_141b383e0", fn_141b383e0 as *const ()), // sub_141b383e0
    Replacement::draft(0x141b38400, "unknown_141b38400", tpl_37673::<0x30> as *const ()), // sub_141b38400
    Replacement::draft(0x141b38420, "unknown_141b38420", tpl_37673::<0x20> as *const ()), // sub_141b38420
    Replacement::draft(0x141b38440, "unknown_141b38440", fn_141b38440 as *const ()), // sub_141b38440
    Replacement::draft_throws(0x141b38490, "unknown_141b38490", fn_141b38490 as *const (), Throws::Discard), // sub_141b38490
    Replacement::draft(0x141b44670, "unknown_141b44670", tpl_95993::<0x14320cbb8, 0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // sub_141b44670
    Replacement::draft(0x141b44850, "unknown_141b44850", fn_141b44850 as *const ()), // sub_141b44850
    Replacement::draft(0x141b448e0, "unknown_141b448e0", tpl_34671::<0x20> as *const ()), // sub_141b448e0
    Replacement::draft(0x141b44960, "unknown_141b44960", tpl_37677::<0x20, 0x40, 0x50, 0x58> as *const ()), // sub_141b44960
    Replacement::draft_throws(0x141b449a0, "unknown_141b449a0", tpl_1331::<0x8, 0x140623910, 0x14384deec, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141b449a0
    Replacement::draft(0x141b44a50, "unknown_141b44a50", tpl_37192::<0x10, 0x40, 0x8, 0x18, 0x143cbe1fc, 0x1806c, 0x18058, 0x142489ee0, 0xc03> as *const ()), // sub_141b44a50
    Replacement::draft_throws(0x141b44b20, "unknown_141b44b20", tpl_1331::<0x8, 0x140623820, 0x14384dc74, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141b44b20
    Replacement::draft(0x141b44bd0, "unknown_141b44bd0", tpl_37678::<0x10, 0x8, 0x18> as *const ()), // sub_141b44bd0
    Replacement::draft(0x141b44bf0, "unknown_141b44bf0", tpl_37679::<0x18, 0x8, 0x10> as *const ()), // sub_141b44bf0
    Replacement::draft(0x141b45130, "unknown_141b45130", tpl_37680::<0x18, 0x8, 0x10> as *const ()), // sub_141b45130
    Replacement::draft(0x141b45190, "unknown_141b45190", tpl_37681::<0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x8, 0x10> as *const ()), // sub_141b45190
    Replacement::draft(0x141b451f0, "unknown_141b451f0", tpl_37718::<0x8, 0x10> as *const ()), // sub_141b451f0
    Replacement::draft(0x141b45210, "unknown_141b45210", tpl_37718::<0x8, 0x10> as *const ()), // sub_141b45210
    Replacement::draft_throws(0x141b45230, "unknown_141b45230", tpl_1331::<0x8, 0x1406235e0, 0x14384df10, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141b45230
    Replacement::draft(0x141b45400, "unknown_141b45400", tpl_37682::<0x18, 0x8, 0x10, 0x20, 0x40, 0x38, 0x60, 0x58> as *const ()), // sub_141b45400
    Replacement::draft(0x141b45440, "unknown_141b45440", tpl_37719 as *const ()), // sub_141b45440
    Replacement::draft(0x141b45450, "unknown_141b45450", tpl_37683::<0x20, 0x18, 0x40, 0x60, 0x38, 0x8, 0x58, 0x10> as *const ()), // sub_141b45450
    Replacement::draft(0x141b45490, "unknown_141b45490", tpl_37718::<0x8, 0x10> as *const ()), // sub_141b45490
    Replacement::draft(0x141b454b0, "unknown_141b454b0", tpl_37718::<0x8, 0x10> as *const ()), // sub_141b454b0
    Replacement::draft(0x141b45510, "unknown_141b45510", tpl_35031::<0x142407c80, 0x14384dee8, 0x14020e8f0> as *const ()), // sub_141b45510
    Replacement::draft(0x141b45630, "unknown_141b45630", tpl_496 as *const ()), // sub_141b45630
    Replacement::draft(0x141b45640, "unknown_141b45640", tpl_496 as *const ()), // sub_141b45640
    Replacement::draft(0x141b45650, "unknown_141b45650", tpl_496 as *const ()), // sub_141b45650
    Replacement::draft(0x141b45660, "unknown_141b45660", tpl_369 as *const ()), // sub_141b45660
    Replacement::draft(0x141b45670, "unknown_141b45670", fn_141b45670 as *const ()), // sub_141b45670
    Replacement::draft(0x141b45760, "unknown_141b45760", tpl_496 as *const ()), // sub_141b45760
    Replacement::draft(0x141b45770, "unknown_141b45770", tpl_361 as *const ()), // sub_141b45770
    Replacement::draft(0x141b45780, "unknown_141b45780", tpl_361 as *const ()), // sub_141b45780
    Replacement::draft_throws(0x141b45790, "unknown_141b45790", fn_141b45790 as *const (), Throws::Discard), // sub_141b45790
    Replacement::draft_throws(0x141b45a30, "unknown_141b45a30", fn_141b45a30 as *const (), Throws::Discard), // sub_141b45a30
    Replacement::draft_throws(0x141b45c10, "unknown_141b45c10", fn_141b45c10 as *const (), Throws::Discard), // sub_141b45c10
    Replacement::draft(0x141b45cb0, "unknown_141b45cb0", fn_141b45cb0 as *const ()), // sub_141b45cb0
    Replacement::draft(0x141b46030, "unknown_141b46030", fn_141b46030 as *const ()), // sub_141b46030
    Replacement::draft(0x141b465a0, "unknown_141b465a0", fn_141b465a0 as *const ()), // sub_141b465a0
    Replacement::draft(0x141b46720, "kaim_141b46720", fn_141b46720 as *const ()), // sub_141b46720
    Replacement::draft(0x141b46d20, "unknown_141b46d20", tpl_361 as *const ()), // sub_141b46d20
    Replacement::draft(0x141b46d30, "unknown_141b46d30", tpl_361 as *const ()), // sub_141b46d30
    Replacement::draft(0x141b46d40, "unknown_141b46d40", tpl_361 as *const ()), // sub_141b46d40
    Replacement::draft(0x141b46d50, "unknown_141b46d50", tpl_361 as *const ()), // sub_141b46d50
    Replacement::draft(0x141b46d60, "unknown_141b46d60", tpl_361 as *const ()), // sub_141b46d60
    Replacement::draft(0x141b46d70, "unknown_141b46d70", tpl_361 as *const ()), // sub_141b46d70
    Replacement::draft(0x141b46d80, "unknown_141b46d80", tpl_361 as *const ()), // sub_141b46d80
    Replacement::draft(0x141b46d90, "unknown_141b46d90", tpl_361 as *const ()), // sub_141b46d90
    Replacement::draft(0x141b46da0, "unknown_141b46da0", tpl_361 as *const ()), // sub_141b46da0
    Replacement::draft(0x141b46db0, "unknown_141b46db0", tpl_361 as *const ()), // sub_141b46db0
    Replacement::draft(0x141b46dc0, "unknown_141b46dc0", tpl_361 as *const ()), // sub_141b46dc0
    Replacement::draft(0x141b46dd0, "unknown_141b46dd0", tpl_361 as *const ()), // sub_141b46dd0
    Replacement::draft(0x141b46de0, "unknown_141b46de0", tpl_361 as *const ()), // sub_141b46de0
    Replacement::draft_throws(0x141b46df0, "unknown_141b46df0", fn_141b46df0 as *const (), Throws::Discard), // sub_141b46df0
    Replacement::draft(0x141b471d0, "unknown_141b471d0", tpl_34994::<0x143cbe8b8, 0x14387c148, 0x14387c140, 0x141834a78, 0x1425f9890, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b471d0
    Replacement::draft(0x141b47240, "unknown_141b47240", tpl_369 as *const ()), // sub_141b47240
    Replacement::draft(0x141b47250, "unknown_141b47250", tpl_34994::<0x143cbe7f4, 0x14387c138, 0x14387c130, 0x141834a78, 0x1425f9870, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b47250
    Replacement::draft(0x141b472c0, "unknown_141b472c0", fn_141b472c0 as *const ()), // sub_141b472c0
    Replacement::draft(0x141b47370, "unknown_141b47370", fn_141b47370 as *const ()), // sub_141b47370
    Replacement::draft(0x141b473a0, "unknown_141b473a0", tpl_35536::<0x8, 0x1403b8050> as *const ()), // sub_141b473a0
    Replacement::draft(0x141b473e0, "unknown_141b473e0", tpl_34994::<0x143cbe7f0, 0x14387c128, 0x14387c120, 0x141834a78, 0x1425f9880, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b473e0
    Replacement::draft(0x141b47450, "unknown_141b47450", fn_141b47450 as *const ()), // sub_141b47450
    Replacement::draft(0x141b47630, "unknown_141b47630", fn_141b47630 as *const ()), // sub_141b47630
    Replacement::draft(0x141b47650, "unknown_141b47650", tpl_35097 as *const ()), // sub_141b47650
    Replacement::draft(0x141b47690, "unknown_141b47690", tpl_36277::<0x8> as *const ()), // sub_141b47690
    Replacement::draft(0x141b476b0, "unknown_141b476b0", tpl_369 as *const ()), // sub_141b476b0
    Replacement::draft(0x141b476c0, "unknown_141b476c0", tpl_36278::<0x8> as *const ()), // sub_141b476c0
    Replacement::draft_throws(0x141b476d0, "unknown_141b476d0", tpl_37728::<0x14241c0a0> as *const (), Throws::Discard), // sub_141b476d0
    Replacement::draft(0x141b476f0, "unknown_141b476f0", tpl_35253 as *const ()), // sub_141b476f0
    Replacement::draft(0x141b47700, "unknown_141b47700", tpl_37729 as *const ()), // sub_141b47700
    Replacement::draft(0x141b47720, "unknown_141b47720", tpl_37730::<0xc> as *const ()), // sub_141b47720
    Replacement::draft(0x141b47740, "unknown_141b47740", tpl_37731 as *const ()), // sub_141b47740
    Replacement::draft(0x141b47750, "unknown_141b47750", tpl_361 as *const ()), // sub_141b47750
    Replacement::draft(0x141b47760, "unknown_141b47760", tpl_496 as *const ()), // sub_141b47760
    Replacement::draft(0x141b47770, "unknown_141b47770", tpl_361 as *const ()), // sub_141b47770
    Replacement::draft(0x141b47780, "unknown_141b47780", tpl_548::<0x8> as *const ()), // sub_141b47780
    Replacement::draft(0x141b47790, "unknown_141b47790", tpl_548::<0xc> as *const ()), // sub_141b47790
    Replacement::draft_throws(0x141b477a0, "unknown_141b477a0", tpl_36283::<0x14241c190> as *const (), Throws::Discard), // sub_141b477a0
    Replacement::draft(0x141b477c0, "unknown_141b477c0", tpl_895::<0x8> as *const ()), // sub_141b477c0
    Replacement::draft(0x141b477e0, "unknown_141b477e0", tpl_361 as *const ()), // sub_141b477e0
    Replacement::draft(0x141b477f0, "unknown_141b477f0", tpl_361 as *const ()), // sub_141b477f0
    Replacement::draft(0x141b47800, "unknown_141b47800", tpl_895::<0x8> as *const ()), // sub_141b47800
    Replacement::draft(0x141b47820, "unknown_141b47820", tpl_361 as *const ()), // sub_141b47820
    Replacement::draft(0x141b47830, "unknown_141b47830", tpl_473 as *const ()), // sub_141b47830
    Replacement::draft(0x141b47840, "unknown_141b47840", tpl_37730::<0x8> as *const ()), // sub_141b47840
    Replacement::draft(0x141b47860, "unknown_141b47860", tpl_34443 as *const ()), // sub_141b47860
    Replacement::draft(0x141b47870, "unknown_141b47870", tpl_37732 as *const ()), // sub_141b47870
    Replacement::draft(0x141b47890, "unknown_141b47890", tpl_34318 as *const ()), // sub_141b47890
    Replacement::draft(0x141b478b0, "unknown_141b478b0", tpl_34318 as *const ()), // sub_141b478b0
    Replacement::draft(0x141b478d0, "unknown_141b478d0", tpl_34318 as *const ()), // sub_141b478d0
    Replacement::draft(0x141b478f0, "unknown_141b478f0", tpl_34319::<0x143ca7604, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b478f0
    Replacement::draft(0x141b47950, "unknown_141b47950", tpl_34319::<0x143cbe480, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b47950
    Replacement::draft(0x141b479b0, "unknown_141b479b0", tpl_34319::<0x143c9a624, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b479b0
    Replacement::draft(0x141b47a10, "unknown_141b47a10", tpl_34318 as *const ()), // sub_141b47a10
    Replacement::draft(0x141b47a30, "unknown_141b47a30", tpl_34318 as *const ()), // sub_141b47a30
    Replacement::draft(0x141b47a50, "unknown_141b47a50", tpl_34318 as *const ()), // sub_141b47a50
    Replacement::draft(0x141b47a70, "unknown_141b47a70", tpl_34318 as *const ()), // sub_141b47a70
    Replacement::draft(0x141b47a90, "unknown_141b47a90", tpl_34318 as *const ()), // sub_141b47a90
    Replacement::draft(0x141b47ab0, "unknown_141b47ab0", tpl_34318 as *const ()), // sub_141b47ab0
    Replacement::draft(0x141b47ad0, "unknown_141b47ad0", tpl_34318 as *const ()), // sub_141b47ad0
    Replacement::draft(0x141b47af0, "unknown_141b47af0", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b47af0
    Replacement::draft(0x141b47b50, "unknown_141b47b50", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b47b50
    Replacement::draft(0x141b47bb0, "unknown_141b47bb0", tpl_34319::<0x143c8fa04, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b47bb0
    Replacement::draft(0x141b47c10, "unknown_141b47c10", tpl_34319::<0x143c8de68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b47c10
    Replacement::draft(0x141b47c70, "unknown_141b47c70", tpl_34319::<0x143c9a660, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b47c70
    Replacement::draft(0x141b47cd0, "unknown_141b47cd0", tpl_34319::<0x143c9a86c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b47cd0
    Replacement::draft(0x141b47d30, "unknown_141b47d30", tpl_34319::<0x143c8f4d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b47d30
    Replacement::draft(0x141b47d90, "unknown_141b47d90", tpl_35253 as *const ()), // sub_141b47d90
    Replacement::draft(0x141b47da0, "unknown_141b47da0", tpl_473 as *const ()), // sub_141b47da0
    Replacement::draft(0x141b47db0, "unknown_141b47db0", tpl_37733 as *const ()), // sub_141b47db0
    Replacement::draft(0x141b47dd0, "unknown_141b47dd0", tpl_23999 as *const ()), // sub_141b47dd0
    Replacement::draft(0x141b47de0, "unknown_141b47de0", tpl_37040::<0x143c9a660, 0x14020e8f0, 0x143c9a86c> as *const ()), // sub_141b47de0
    Replacement::draft(0x141b47f90, "unknown_141b47f90", tpl_496 as *const ()), // sub_141b47f90
    Replacement::draft(0x141b47fa0, "unknown_141b47fa0", fn_141b47fa0 as *const ()), // sub_141b47fa0
    Replacement::draft(0x141b482d0, "unknown_141b482d0", tpl_496 as *const ()), // sub_141b482d0
    Replacement::draft(0x141b482e0, "unknown_141b482e0", fn_141b482e0 as *const ()), // sub_141b482e0
    Replacement::draft(0x141b48390, "unknown_141b48390", tpl_496 as *const ()), // sub_141b48390
    Replacement::draft(0x141b483a0, "unknown_141b483a0", tpl_496 as *const ()), // sub_141b483a0
    Replacement::draft(0x141b483b0, "unknown_141b483b0", tpl_361 as *const ()), // sub_141b483b0
    Replacement::draft(0x141b483c0, "unknown_141b483c0", tpl_496 as *const ()), // sub_141b483c0
    Replacement::draft(0x141b483d0, "unknown_141b483d0", tpl_496 as *const ()), // sub_141b483d0
    Replacement::draft(0x141b483e0, "unknown_141b483e0", tpl_496 as *const ()), // sub_141b483e0
    Replacement::draft(0x141b483f0, "unknown_141b483f0", tpl_361 as *const ()), // sub_141b483f0
    Replacement::draft_throws(0x141b48400, "unknown_141b48400", fn_141b48400 as *const (), Throws::Discard), // sub_141b48400
    Replacement::draft(0x141b48fa0, "unknown_141b48fa0", fn_141b48fa0 as *const ()), // sub_141b48fa0
    Replacement::draft(0x141b49020, "unknown_141b49020", fn_141b49020 as *const ()), // sub_141b49020
    Replacement::draft(0x141b49190, "unknown_141b49190", fn_141b49190 as *const ()), // sub_141b49190
    Replacement::draft(0x141b491c0, "unknown_141b491c0", tpl_34443 as *const ()), // sub_141b491c0
    Replacement::draft(0x141b491d0, "unknown_141b491d0", fn_141b491d0 as *const ()), // sub_141b491d0
    Replacement::draft_throws(0x141b49240, "unknown_141b49240", fn_141b49240 as *const (), Throws::Discard), // sub_141b49240
    Replacement::draft(0x141b49430, "unknown_141b49430", tpl_496 as *const ()), // sub_141b49430
    Replacement::draft(0x141b49440, "unknown_141b49440", tpl_496 as *const ()), // sub_141b49440
    Replacement::draft(0x141b49450, "unknown_141b49450", tpl_496 as *const ()), // sub_141b49450
    Replacement::draft(0x141b49460, "unknown_141b49460", tpl_895::<0x8> as *const ()), // sub_141b49460
    Replacement::draft_throws(0x141b49480, "unknown_141b49480", fn_141b49480 as *const (), Throws::Caught), // sub_141b49480
    Replacement::draft_throws(0x141b49fe0, "unknown_141b49fe0", fn_141b49fe0 as *const (), Throws::Discard), // sub_141b49fe0
    Replacement::draft(0x141b4a280, "unknown_141b4a280", tpl_496 as *const ()), // sub_141b4a280
    Replacement::draft(0x141b4a290, "unknown_141b4a290", tpl_496 as *const ()), // sub_141b4a290
    Replacement::draft(0x141b4a2a0, "unknown_141b4a2a0", tpl_496 as *const ()), // sub_141b4a2a0
    Replacement::draft_throws(0x141b4a2b0, "std_141b4a2b0", fn_141b4a2b0 as *const (), Throws::Discard), // sub_141b4a2b0
    Replacement::draft(0x141b4a730, "unknown_141b4a730", fn_141b4a730 as *const ()), // sub_141b4a730
    Replacement::draft(0x141b4a770, "unknown_141b4a770", fn_141b4a770 as *const ()), // sub_141b4a770
    Replacement::draft(0x141b4a7c0, "unknown_141b4a7c0", fn_141b4a7c0 as *const ()), // sub_141b4a7c0
    Replacement::draft_throws(0x141b4a820, "unknown_141b4a820", fn_141b4a820 as *const (), Throws::Discard), // sub_141b4a820
    Replacement::draft(0x141b4ad90, "unknown_141b4ad90", tpl_34994::<0x143cbedc4, 0x14387c168, 0x14387c160, 0x141834a78, 0x1425f98c0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b4ad90
    Replacement::draft(0x141b4ae00, "unknown_141b4ae00", tpl_35252 as *const ()), // sub_141b4ae00
    Replacement::draft(0x141b4ae20, "unknown_141b4ae20", tpl_34994::<0x143cbedc0, 0x14387c158, 0x14387c150, 0x141834a78, 0x1425f98b0, 0x141834d18, 0x141834a0c> as *const ()), // sub_141b4ae20
    Replacement::draft(0x141b4ae90, "unknown_141b4ae90", tpl_35252 as *const ()), // sub_141b4ae90
    Replacement::draft(0x141b4aeb0, "unknown_141b4aeb0", tpl_34318 as *const ()), // sub_141b4aeb0
    Replacement::draft(0x141b4aed0, "unknown_141b4aed0", tpl_34318 as *const ()), // sub_141b4aed0
    Replacement::draft(0x141b4aef0, "unknown_141b4aef0", tpl_34319::<0x143cbeb80, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4aef0
    Replacement::draft(0x141b4af50, "unknown_141b4af50", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4af50
    Replacement::draft(0x141b4afb0, "unknown_141b4afb0", tpl_34318 as *const ()), // sub_141b4afb0
    Replacement::draft(0x141b4afd0, "unknown_141b4afd0", tpl_34318 as *const ()), // sub_141b4afd0
    Replacement::draft(0x141b4aff0, "unknown_141b4aff0", tpl_34319::<0x143cb1824, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4aff0
    Replacement::draft(0x141b4b050, "unknown_141b4b050", tpl_34319::<0x143cbe47c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4b050
    Replacement::draft(0x141b4b0b0, "unknown_141b4b0b0", tpl_34318 as *const ()), // sub_141b4b0b0
    Replacement::draft(0x141b4b0d0, "unknown_141b4b0d0", tpl_34318 as *const ()), // sub_141b4b0d0
    Replacement::draft(0x141b4b0f0, "unknown_141b4b0f0", tpl_34318 as *const ()), // sub_141b4b0f0
    Replacement::draft(0x141b4b110, "unknown_141b4b110", tpl_34318 as *const ()), // sub_141b4b110
    Replacement::draft(0x141b4b130, "unknown_141b4b130", tpl_34319::<0x143c8de68, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4b130
    Replacement::draft(0x141b4b190, "unknown_141b4b190", tpl_34319::<0x143ca7604, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4b190
    Replacement::draft(0x141b4b1f0, "unknown_141b4b1f0", tpl_34319::<0x143cbe480, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4b1f0
    Replacement::draft(0x141b4b250, "unknown_141b4b250", tpl_34319::<0x143c9a624, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4b250
    Replacement::draft(0x141b4b2b0, "unknown_141b4b2b0", tpl_34318 as *const ()), // sub_141b4b2b0
    Replacement::draft(0x141b4b2d0, "unknown_141b4b2d0", tpl_34318 as *const ()), // sub_141b4b2d0
    Replacement::draft(0x141b4b2f0, "unknown_141b4b2f0", tpl_34318 as *const ()), // sub_141b4b2f0
    Replacement::draft(0x141b4b310, "unknown_141b4b310", tpl_34319::<0x143c8d160, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4b310
    Replacement::draft(0x141b4b370, "unknown_141b4b370", tpl_34319::<0x143cbe8e4, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4b370
    Replacement::draft(0x141b4b3d0, "unknown_141b4b3d0", tpl_34319::<0x143c8f4d0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b4b3d0
    Replacement::draft_throws(0x141b4b4b0, "unknown_141b4b4b0", fn_141b4b4b0 as *const (), Throws::Discard), // sub_141b4b4b0
    Replacement::draft(0x141b4b910, "unknown_141b4b910", tpl_361 as *const ()), // sub_141b4b910
    Replacement::draft_throws(0x141b4b920, "unknown_141b4b920", fn_141b4b920 as *const (), Throws::Discard), // sub_141b4b920
    Replacement::draft(0x141b4bee0, "unknown_141b4bee0", tpl_361 as *const ()), // sub_141b4bee0
    Replacement::draft(0x141b4bef0, "unknown_141b4bef0", tpl_496 as *const ()), // sub_141b4bef0
    Replacement::draft_throws(0x141b4bf00, "unknown_141b4bf00", fn_141b4bf00 as *const (), Throws::Discard), // sub_141b4bf00
    Replacement::draft(0x141b4c7b0, "unknown_141b4c7b0", tpl_496 as *const ()), // sub_141b4c7b0
    Replacement::draft(0x141b4c7c0, "unknown_141b4c7c0", fn_141b4c7c0 as *const ()), // sub_141b4c7c0
    Replacement::draft_throws(0x141b4ca40, "unknown_141b4ca40", fn_141b4ca40 as *const (), Throws::Discard), // sub_141b4ca40
    Replacement::draft(0x141b4ce90, "unknown_141b4ce90", tpl_361 as *const ()), // sub_141b4ce90
    Replacement::draft_throws(0x141b4cea0, "unknown_141b4cea0", fn_141b4cea0 as *const (), Throws::Discard), // sub_141b4cea0
    Replacement::draft(0x141b4d2b0, "unknown_141b4d2b0", tpl_361 as *const ()), // sub_141b4d2b0
    Replacement::draft(0x141b4d2c0, "unknown_141b4d2c0", tpl_361 as *const ()), // sub_141b4d2c0
    Replacement::draft(0x141b4d2d0, "unknown_141b4d2d0", fn_141b4d2d0 as *const ()), // sub_141b4d2d0
    Replacement::draft(0x141b4d650, "unknown_141b4d650", tpl_361 as *const ()), // sub_141b4d650
    Replacement::draft(0x141b4d660, "unknown_141b4d660", fn_141b4d660 as *const ()), // sub_141b4d660
    Replacement::draft_throws(0x141b4d680, "unknown_141b4d680", fn_141b4d680 as *const (), Throws::Discard), // sub_141b4d680
    Replacement::draft(0x141b4ddb0, "unknown_141b4ddb0", fn_141b4ddb0 as *const ()), // sub_141b4ddb0
    Replacement::draft(0x141b4de00, "unknown_141b4de00", fn_141b4de00 as *const ()), // sub_141b4de00
    Replacement::draft(0x141b4ded0, "unknown_141b4ded0", fn_141b4ded0 as *const ()), // sub_141b4ded0
    Replacement::draft_throws(0x141b4e170, "bonsai_141b4e170", fn_141b4e170 as *const (), Throws::Caught), // bonsai::WarningShotDecoratorInstance::bonsai::WarningShotDecoratorInstance::vfunc_12
    Replacement::draft_throws(0x141b4e860, "bonsai_141b4e860", fn_141b4e860 as *const (), Throws::Discard), // bonsai::WarningShotDecoratorInstance::bonsai::WarningShotDecoratorInstance::vfunc_4
    Replacement::draft(0x141b4e910, "bonsai_141b4e910", fn_141b4e910 as *const ()), // sub_141b4e910
    Replacement::draft(0x141b4eee0, "bonsai_141b4eee0", fn_141b4eee0 as *const ()), // bonsai::WarningShotDecoratorInstance::ClassTypeInfoImplementation::bonsai::WarningShotDecoratorInstance::ClassTypeInfoIm
    Replacement::draft_throws(0x141b4f0e0, "bonsai_141b4f0e0", tpl_37758::<0x8, 0xc8, 0x40, 0x10, 0x141bd6940, 0x4, 0x5, 0x18> as *const (), Throws::Discard), // bonsai::UseAbilityServiceInstance::bonsai::UseAbilityServiceInstance::vfunc_6
    Replacement::draft_throws(0x141b4f130, "bonsai_141b4f130", tpl_37758::<0x8, 0xc8, 0x40, 0x10, 0x141bd6940, 0x4, 0x5, 0x18> as *const (), Throws::Discard), // sub_141b4f130
    Replacement::draft_throws(0x141b4f180, "bonsai_141b4f180", fn_141b4f180 as *const (), Throws::Discard), // bonsai::UseAbilityServiceInstance::bonsai::UseAbilityServiceInstance::vfunc_2
    Replacement::draft_throws(0x141b4f910, "bonsai_141b4f910", fn_141b4f910 as *const (), Throws::Discard), // bonsai::UnsetPropertyDecoratorInstance::bonsai::UnsetPropertyDecoratorInstance::vfunc_9
    Replacement::draft_throws(0x141b4f960, "bonsai_141b4f960", fn_141b4f960 as *const (), Throws::Discard), // sub_141b4f960
    Replacement::draft_throws(0x141b4f990, "bonsai_141b4f990", fn_141b4f990 as *const (), Throws::Discard), // bonsai::UnsetPropertyDecoratorInstance::bonsai::UnsetPropertyDecoratorInstance::vfunc_4
    Replacement::draft(0x141b4ff80, "bonsai_141b4ff80", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::UnsetPropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::UnsetPropertyDecoratorInstance::ClassTypeIn
    Replacement::draft(0x141b50160, "bonsai_141b50160", fn_141b50160 as *const ()), // bonsai::TimeSincePropertySetConditionDecoratorInstance::bonsai::TimeSincePropertySetConditionDecoratorInstance::vfunc_12
    Replacement::draft(0x141b501b0, "bonsai_141b501b0", fn_141b501b0 as *const ()), // bonsai::TimeSincePropertySetConditionDecoratorInstance::bonsai::TimeSincePropertySetConditionDecoratorInstance::vfunc_4
    Replacement::draft_throws(0x141b50230, "bonsai_141b50230", fn_141b50230 as *const (), Throws::Discard), // bonsai::TimeSincePropertySetConditionDecoratorInstance::bonsai::TimeSincePropertySetConditionDecoratorInstance::vfunc_1
    Replacement::draft(0x141b508c0, "bonsai_141b508c0", tpl_37765::<0x8, 0x10, 0x18, 0x20, 0x30, 0x38> as *const ()), // bonsai::TimeSincePropertySetConditionDecoratorInstance::ClassTypeInfoImplementation::bonsai::TimeSincePropertySetConditi
    Replacement::draft(0x141b50aa0, "bonsai_141b50aa0", tpl_37766::<0xc8, 0x70, 0x60, 0x141bac2d0> as *const ()), // bonsai::SwitchTraversalNodeModeInstance::bonsai::SwitchTraversalNodeModeInstance::vfunc_6
    Replacement::draft(0x141b50af0, "bonsai_141b50af0", tpl_37767::<0x141ba3170, 0x1c, 0x10, 0xc8, 0x70, 0x141bac780> as *const ()), // bonsai::SwitchTraversalNodeModeInstance::bonsai::SwitchTraversalNodeModeInstance::vfunc_11
    Replacement::draft(0x141b50b80, "bonsai_141b50b80", tpl_37768::<0x141ba31e0, 0x50, 0x58, 0x1c, 0x8> as *const ()), // bonsai::SwitchTraversalNodeModeInstance::bonsai::SwitchTraversalNodeModeInstance::vfunc_10
    Replacement::draft(0x141b51150, "bonsai_141b51150", tpl_37769::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60> as *const ()), // bonsai::SwitchTraversalNodeModeInstance::ClassTypeInfoImplementation::bonsai::SwitchTraversalNodeModeInstance::ClassType
    Replacement::draft(0x141b511f0, "bonsai_141b511f0", tpl_37770::<0x143212090, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::SwitchTraversalNodeModeInstance::ClassTypeInfoImplementation::bonsai::SwitchTraversalNodeModeInstance::ClassType
    Replacement::draft(0x141b51380, "bonsai_141b51380", tpl_37771::<0x68, 0x70> as *const ()), // bonsai::StunnedProgram::bonsai::StunnedProgram::vfunc_15
    Replacement::draft(0x141b513e0, "bonsai_141b513e0", fn_141b513e0 as *const ()), // bonsai::StunnedProgram::bonsai::StunnedProgram::vfunc_6
    Replacement::draft(0x141b51450, "bonsai_141b51450", tpl_37773::<0x68, 0x70> as *const ()), // bonsai::StunnedProgram::bonsai::StunnedProgram::vfunc_5
    Replacement::draft(0x141b514a0, "bonsai_141b514a0", tpl_37774::<0x68, 0x70, 0x141b51590> as *const ()), // bonsai::StunnedProgram::bonsai::StunnedProgram::vfunc_1
    Replacement::draft(0x141b51510, "bonsai_141b51510", fn_141b51510 as *const ()), // sub_141b51510
    Replacement::draft(0x141b51590, "bonsai_141b51590", fn_141b51590 as *const ()), // sub_141b51590
    Replacement::draft(0x141b516a0, "bonsai_141b516a0", tpl_37776::<0x1431f8560, 0x60, 0x1430f6b98, 0x38, 0x1432316d0> as *const ()), // sub_141b516a0
    Replacement::draft(0x141b516d0, "bonsai_141b516d0", fn_141b516d0 as *const ()), // sub_141b516d0
    Replacement::draft(0x141b51780, "bonsai_141b51780", fn_141b51780 as *const ()), // sub_141b51780
    Replacement::draft(0x141b517f0, "bonsai_141b517f0", fn_141b517f0 as *const ()), // sub_141b517f0
    Replacement::draft(0x141b518c0, "bonsai_141b518c0", fn_141b518c0 as *const ()), // sub_141b518c0
    Replacement::draft(0x141b51a30, "bonsai_141b51a30", fn_141b51a30 as *const ()), // sub_141b51a30
    Replacement::draft_throws(0x141b51be0, "bonsai_141b51be0", fn_141b51be0 as *const (), Throws::Discard), // bonsai::StuckDetectionServiceInstance::bonsai::StuckDetectionServiceInstance::vfunc_6
    Replacement::draft(0x141b51f30, "bonsai_141b51f30", fn_141b51f30 as *const ()), // bonsai::StuckDetectionServiceInstance::bonsai::StuckDetectionServiceInstance::vfunc_1
    Replacement::draft(0x141b51fe0, "bonsai_141b51fe0", fn_141b51fe0 as *const ()), // bonsai::StuckDetectionServiceInstance::bonsai::StuckDetectionServiceInstance::vfunc_3
    Replacement::draft(0x141b52560, "bonsai_141b52560", fn_141b52560 as *const ()), // bonsai::StuckDetectionServiceInstance::ClassTypeInfoImplementation::bonsai::StuckDetectionServiceInstance::ClassTypeInfo
    Replacement::draft(0x141b52670, "bonsai_141b52670", fn_141b52670 as *const ()), // bonsai::StuckDetectionServiceInstance::ClassTypeInfoImplementation::bonsai::StuckDetectionServiceInstance::ClassTypeInfo
    Replacement::draft(0x141b52810, "unknown_141b52810", fn_141b52810 as *const ()), // sub_141b52810
    Replacement::draft(0x141b52860, "unknown_141b52860", fn_141b52860 as *const ()), // sub_141b52860
    Replacement::draft(0x141b52a50, "unknown_141b52a50", tpl_37792::<0x38, 0x30, 0x18, 0x28, 0x10, 0x14062ee90> as *const ()), // sub_141b52a50
    Replacement::draft(0x141b52ae0, "unknown_141b52ae0", tpl_37793::<0x18, 0x10, 0x30, 0x28, 0x14062ee90> as *const ()), // sub_141b52ae0
    Replacement::draft(0x141b52b30, "std_141b52b30", tpl_770::<0x143212750> as *const ()), // std::_Func_impl_no_alloc<bonsai::sphericalpoints::getScaledPoints::?bref1::operator()::<lambda_1>,bool,const coregame::P
    Replacement::draft(0x141b52b50, "std_141b52b50", tpl_770::<0x143212750> as *const ()), // std::_Func_impl_no_alloc<bonsai::sphericalpoints::getScaledPoints::?bref1::operator()::<lambda_1>,bool,const coregame::P
    Replacement::draft(0x141b52b70, "std_141b52b70", tpl_37794::<0x20> as *const ()), // std::_Func_impl_no_alloc<bonsai::sphericalpoints::getScaledPoints::?bref1::operator()::<lambda_1>,bool,const coregame::P
    Replacement::draft(0x141b52b90, "std_141b52b90", tpl_1::<0x14388e5c0> as *const ()), // std::_Func_impl_no_alloc<bonsai::sphericalpoints::getScaledPoints::?bref1::operator()::<lambda_1>,bool,const coregame::P
    Replacement::draft(0x141b52ba0, "std_141b52ba0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bonsai::sphericalpoints::getScaledPoints::?bref1::operator()::<lambda_1>,bool,const coregame::P
    Replacement::draft(0x141b52bb0, "std_141b52bb0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bonsai::sphericalpoints::getScaledPoints::?bref1::operator()::<lambda_1>,bool,const coregame::P
    Replacement::draft(0x141b52bd0, "unknown_141b52bd0", fn_141b52bd0 as *const ()), // sub_141b52bd0
    Replacement::draft(0x141b52bf0, "unknown_141b52bf0", tpl_34443 as *const ()), // sub_141b52bf0
    Replacement::draft(0x141b52c00, "unknown_141b52c00", tpl_37794::<0x20> as *const ()), // sub_141b52c00
    Replacement::draft(0x141b52c20, "unknown_141b52c20", fn_141b52c20 as *const ()), // sub_141b52c20
    Replacement::draft(0x141b52d30, "unknown_141b52d30", fn_141b52d30 as *const ()), // sub_141b52d30
    Replacement::draft_throws(0x141b53240, "unknown_141b53240", fn_141b53240 as *const (), Throws::Discard), // sub_141b53240
    Replacement::draft_throws(0x141b53570, "bonsai_141b53570", fn_141b53570 as *const (), Throws::Caught), // bonsai::SpeedOverrideServiceInstance::bonsai::SpeedOverrideServiceInstance::vfunc_6
    Replacement::draft(0x141b53e00, "bonsai_141b53e00", tpl_37798::<0x1434268d0> as *const ()), // sub_141b53e00
    Replacement::draft(0x141b53e60, "bonsai_141b53e60", fn_141b53e60 as *const ()), // sub_141b53e60
    Replacement::draft(0x141b53f00, "bonsai_141b53f00", fn_141b53f00 as *const ()), // sub_141b53f00
    Replacement::draft(0x141b53fc0, "bonsai_141b53fc0", fn_141b53fc0 as *const ()), // bonsai::SpeedMatchingDecoratorInstance::bonsai::SpeedMatchingDecoratorInstance::vfunc_33
    Replacement::draft_throws(0x141b54070, "bonsai_141b54070", fn_141b54070 as *const (), Throws::Discard), // sub_141b54070
    Replacement::draft_throws(0x141b54420, "bonsai_141b54420", fn_141b54420 as *const (), Throws::Caught), // bonsai::SpeedMatchingDecoratorInstance::bonsai::SpeedMatchingDecoratorInstance::vfunc_12
    Replacement::draft(0x141b54570, "bonsai_141b54570", fn_141b54570 as *const ()), // bonsai::SpeedMatchingDecoratorInstance::bonsai::SpeedMatchingDecoratorInstance::vfunc_4
    Replacement::draft(0x141b54610, "bonsai_141b54610", fn_141b54610 as *const ()), // sub_141b54610
    Replacement::draft(0x141b54bb0, "bonsai_141b54bb0", fn_141b54bb0 as *const ()), // bonsai::SpeedMatchingDecoratorInstance::ClassTypeInfoImplementation::bonsai::SpeedMatchingDecoratorInstance::ClassTypeIn
    Replacement::draft(0x141b54c50, "bonsai_141b54c50", fn_141b54c50 as *const ()), // bonsai::SpeedMatchingDecoratorInstance::ClassTypeInfoImplementation::bonsai::SpeedMatchingDecoratorInstance::ClassTypeIn
    Replacement::draft(0x141b54dd0, "bonsai_141b54dd0", fn_141b54dd0 as *const ()), // sub_141b54dd0
    Replacement::draft(0x141b54e00, "bonsai_141b54e00", tpl_37771::<0x78, 0x80> as *const ()), // bonsai::ShootProgram::bonsai::ShootProgram::vfunc_15
    Replacement::draft(0x141b54e60, "bonsai_141b54e60", fn_141b54e60 as *const ()), // bonsai::ShootProgram::bonsai::ShootProgram::vfunc_3
    Replacement::draft(0x141b54e80, "bonsai_141b54e80", fn_141b54e80 as *const ()), // bonsai::ShootProgram::bonsai::ShootProgram::vfunc_6
    Replacement::draft(0x141b54f00, "bonsai_141b54f00", tpl_37773::<0x78, 0x80> as *const ()), // bonsai::ShootProgram::bonsai::ShootProgram::vfunc_5
    Replacement::draft(0x141b54f50, "bonsai_141b54f50", tpl_37774::<0x78, 0x80, 0x141b54fd0> as *const ()), // bonsai::ShootProgram::bonsai::ShootProgram::vfunc_1
    Replacement::draft(0x141b54fd0, "bonsai_141b54fd0", fn_141b54fd0 as *const ()), // sub_141b54fd0
    Replacement::draft(0x141b551d0, "bonsai_141b551d0", tpl_37776::<0x1431f8560, 0x70, 0x1430f6b98, 0x38, 0x1432316d0> as *const ()), // sub_141b551d0
    Replacement::draft(0x141b55200, "bonsai_141b55200", fn_141b55200 as *const ()), // sub_141b55200
    Replacement::draft_throws(0x141b55290, "bonsai_141b55290", fn_141b55290 as *const (), Throws::Discard), // sub_141b55290
    Replacement::draft_throws(0x141b552e0, "bonsai_141b552e0", fn_141b552e0 as *const (), Throws::Caught), // sub_141b552e0
    Replacement::draft_throws(0x141b553f0, "bonsai_141b553f0", fn_141b553f0 as *const (), Throws::Caught), // sub_141b553f0
    Replacement::draft(0x141b554f0, "bonsai_141b554f0", fn_141b554f0 as *const ()), // sub_141b554f0
    Replacement::draft_throws(0x141b55560, "bonsai_141b55560", tpl_37815::<0x18, 0x2c, 0x141b555a0> as *const (), Throws::Discard), // bonsai::SetPositionReservationDecoratorInstance::bonsai::SetPositionReservationDecoratorInstance::vfunc_9
    Replacement::draft_throws(0x141b555a0, "bonsai_141b555a0", fn_141b555a0 as *const (), Throws::Discard), // sub_141b555a0
    Replacement::draft_throws(0x141b55690, "bonsai_141b55690", fn_141b55690 as *const (), Throws::Discard), // bonsai::SetPositionReservationDecoratorInstance::bonsai::SetPositionReservationDecoratorInstance::vfunc_4
    Replacement::draft(0x141b55c70, "bonsai_141b55c70", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::SetPositionReservationDecoratorInstance::ClassTypeInfoImplementation::bonsai::SetPositionReservationDecoratorIns
    Replacement::draft_throws(0x141b55e50, "bonsai_141b55e50", fn_141b55e50 as *const (), Throws::Discard), // bonsai::SetBoolKnowledgePropertyDecoratorInstance::bonsai::SetBoolKnowledgePropertyDecoratorInstance::vfunc_33
    Replacement::draft_throws(0x141b55e90, "bonsai_141b55e90", fn_141b55e90 as *const (), Throws::Discard), // bonsai::SetRandomFloatKnowledgePropertyDecoratorInstance::bonsai::SetRandomFloatKnowledgePropertyDecoratorInstance::vfun
    Replacement::draft_throws(0x141b55f30, "bonsai_141b55f30", fn_141b55f30 as *const (), Throws::Discard), // bonsai::SetFloatKnowledgePropertyDecoratorInstance::bonsai::SetFloatKnowledgePropertyDecoratorInstance::vfunc_33
    Replacement::draft_throws(0x141b55f80, "bonsai_141b55f80", fn_141b55f80 as *const (), Throws::Discard), // bonsai::SetRandomIntKnowledgePropertyDecoratorInstance::bonsai::SetRandomIntKnowledgePropertyDecoratorInstance::vfunc_33
    Replacement::draft_throws(0x141b56000, "bonsai_141b56000", fn_141b56000 as *const (), Throws::Discard), // bonsai::SetIntKnowledgePropertyDecoratorInstance::bonsai::SetIntKnowledgePropertyDecoratorInstance::vfunc_33
    Replacement::draft(0x141b56040, "bonsai_141b56040", fn_141b56040 as *const ()), // bonsai::SetKnowledgePropertyDecoratorInstance::bonsai::SetKnowledgePropertyDecoratorInstance::vfunc_9
    Replacement::draft(0x141b56080, "bonsai_141b56080", fn_141b56080 as *const ()), // bonsai::SetKnowledgePropertyDecoratorInstance::bonsai::SetKnowledgePropertyDecoratorInstance::vfunc_4
    Replacement::draft(0x141b568a0, "bonsai_141b568a0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::SetBoolKnowledgePropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::SetBoolKnowledgePropertyDecorato
    Replacement::draft(0x141b57220, "bonsai_141b57220", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::SetRandomFloatKnowledgePropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::SetRandomFloatKnowledgePr
    Replacement::draft(0x141b57ba0, "bonsai_141b57ba0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::SetFloatKnowledgePropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::SetFloatKnowledgePropertyDecora
    Replacement::draft(0x141b58520, "bonsai_141b58520", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::SetRandomIntKnowledgePropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::SetRandomIntKnowledgeProper
    Replacement::draft(0x141b58ea0, "bonsai_141b58ea0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::SetIntKnowledgePropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::SetIntKnowledgePropertyDecoratorI
    Replacement::draft(0x141b595e0, "bonsai_141b595e0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::SetKnowledgePropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::SetKnowledgePropertyDecoratorInstanc
    Replacement::draft(0x141b597c0, "bonsai_141b597c0", tpl_37825::<0x10, 0x20> as *const ()), // bonsai::SayDecoratorInstance::bonsai::SayDecoratorInstance::vfunc_4
    Replacement::draft(0x141b59d80, "bonsai_141b59d80", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::SayDecoratorInstance::ClassTypeInfoImplementation::bonsai::SayDecoratorInstance::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141b59f60, "bonsai_141b59f60", fn_141b59f60 as *const (), Throws::Discard), // bonsai::ReleaseTicketDecoratorInstance::bonsai::ReleaseTicketDecoratorInstance::vfunc_9
    Replacement::draft_throws(0x141b59ff0, "bonsai_141b59ff0", fn_141b59ff0 as *const (), Throws::Discard), // bonsai::ReleaseTicketDecoratorInstance::bonsai::ReleaseTicketDecoratorInstance::vfunc_8
    Replacement::draft(0x141b5a5f0, "bonsai_141b5a5f0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::ReleaseTicketDecoratorInstance::ClassTypeInfoImplementation::bonsai::ReleaseTicketDecoratorInstance::ClassTypeIn
    Replacement::draft(0x141b5a850, "bonsai_141b5a850", fn_141b5a850 as *const ()), // sub_141b5a850
    Replacement::draft(0x141b5a920, "bonsai_141b5a920", fn_141b5a920 as *const ()), // sub_141b5a920
    Replacement::draft(0x141b5a970, "bonsai_141b5a970", fn_141b5a970 as *const ()), // sub_141b5a970
    Replacement::draft(0x141b5ad10, "bonsai_141b5ad10", fn_141b5ad10 as *const ()), // sub_141b5ad10
    Replacement::draft(0x141b5afe0, "bonsai_141b5afe0", fn_141b5afe0 as *const ()), // sub_141b5afe0
    Replacement::draft(0x141b5b080, "bonsai_141b5b080", tpl_35031::<0x142407c80, 0x143cbedf4, 0x14020e8f0> as *const ()), // sub_141b5b080
    Replacement::draft(0x141b5b1b0, "bonsai_141b5b1b0", fn_141b5b1b0 as *const ()), // sub_141b5b1b0
    Replacement::draft_throws(0x141b5b3f0, "bonsai_141b5b3f0", fn_141b5b3f0 as *const (), Throws::Discard), // sub_141b5b3f0
    Replacement::draft_throws(0x141b5b7d0, "bonsai_141b5b7d0", fn_141b5b7d0 as *const (), Throws::Discard), // sub_141b5b7d0
    Replacement::draft_throws(0x141b5bc40, "bonsai_141b5bc40", fn_141b5bc40 as *const (), Throws::Discard), // sub_141b5bc40
    Replacement::draft(0x141b5be20, "bonsai_141b5be20", fn_141b5be20 as *const ()), // sub_141b5be20
    Replacement::draft_throws(0x141b5bef0, "bonsai_141b5bef0", fn_141b5bef0 as *const (), Throws::Discard), // sub_141b5bef0
    Replacement::draft(0x141b5c030, "bonsai_141b5c030", fn_141b5c030 as *const ()), // sub_141b5c030
    Replacement::draft(0x141b5c1b0, "bonsai_141b5c1b0", fn_141b5c1b0 as *const ()), // sub_141b5c1b0
    Replacement::draft(0x141b5c200, "bonsai_141b5c200", fn_141b5c200 as *const ()), // sub_141b5c200
    Replacement::draft(0x141b5c250, "bonsai_141b5c250", fn_141b5c250 as *const ()), // sub_141b5c250
    Replacement::draft_throws(0x141b5c310, "bonsai_141b5c310", fn_141b5c310 as *const (), Throws::Discard), // sub_141b5c310
    Replacement::draft(0x141b5c6f0, "bonsai_141b5c6f0", fn_141b5c6f0 as *const ()), // sub_141b5c6f0
    Replacement::draft(0x141b5c760, "bonsai_141b5c760", fn_141b5c760 as *const ()), // sub_141b5c760
    Replacement::draft_throws(0x141b5c810, "bonsai_141b5c810", fn_141b5c810 as *const (), Throws::Caught), // sub_141b5c810
    Replacement::draft(0x141b5cc70, "bonsai_141b5cc70", fn_141b5cc70 as *const ()), // sub_141b5cc70
    Replacement::draft_throws(0x141b5ccd0, "bonsai_141b5ccd0", fn_141b5ccd0 as *const (), Throws::Discard), // sub_141b5ccd0
    Replacement::draft(0x141b5d0c0, "bonsai_141b5d0c0", tpl_34605::<0x143216b80> as *const ()), // sub_141b5d0c0
    Replacement::draft(0x141b5d200, "bonsai_141b5d200", fn_141b5d200 as *const ()), // sub_141b5d200
    Replacement::draft(0x141b5d2a0, "bonsai_141b5d2a0", fn_141b5d2a0 as *const ()), // sub_141b5d2a0
    Replacement::draft(0x141b5d2d0, "bonsai_141b5d2d0", fn_141b5d2d0 as *const ()), // sub_141b5d2d0
    Replacement::draft(0x141b5d560, "bonsai_141b5d560", fn_141b5d560 as *const ()), // sub_141b5d560
    Replacement::draft(0x141b5d5f0, "bonsai_141b5d5f0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143cbf0d0, 0x158, 0x14020e8f0> as *const ()), // sub_141b5d5f0
    Replacement::draft(0x141b5d6f0, "bonsai_141b5d6f0", fn_141b5d6f0 as *const ()), // sub_141b5d6f0
    Replacement::draft(0x141b5d8a0, "bonsai_141b5d8a0", fn_141b5d8a0 as *const ()), // sub_141b5d8a0
    Replacement::draft_throws(0x141b5d9a0, "bonsai_141b5d9a0", fn_141b5d9a0 as *const (), Throws::Discard), // sub_141b5d9a0
    Replacement::draft_throws(0x141b5ddb0, "bonsai_141b5ddb0", fn_141b5ddb0 as *const (), Throws::Discard), // bonsai::PathFollowingServiceInstance::bonsai::PathFollowingServiceInstance::vfunc_6
    Replacement::draft_throws(0x141b5e6f0, "bonsai_141b5e6f0", fn_141b5e6f0 as *const (), Throws::Caught), // bonsai::OverrideSpeedDecorator::bonsai::OverrideSpeedDecorator::vfunc_12
    Replacement::draft(0x141b5eda0, "bonsai_141b5eda0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::OverrideSpeedDecorator::ClassTypeInfoImplementation::bonsai::OverrideSpeedDecorator::ClassTypeInfoImplementation
    Replacement::draft(0x141b5ef80, "bonsai_141b5ef80", fn_141b5ef80 as *const ()), // bonsai::OnStateCompletedDecoratorInstance::bonsai::OnStateCompletedDecoratorInstance::vfunc_1
    Replacement::draft(0x141b5efc0, "bonsai_141b5efc0", fn_141b5efc0 as *const ()), // bonsai::OnStateCompletedDecoratorInstance::bonsai::OnStateCompletedDecoratorInstance::vfunc_11
    Replacement::draft(0x141b5f550, "bonsai_141b5f550", tpl_37765::<0x8, 0x10, 0x18, 0x20, 0x30, 0x38> as *const ()), // bonsai::OnStateCompletedDecoratorInstance::ClassTypeInfoImplementation::bonsai::OnStateCompletedDecoratorInstance::Class
    Replacement::draft(0x141b5f5c0, "bonsai_141b5f5c0", fn_141b5f5c0 as *const ()), // bonsai::OnStateCompletedDecoratorInstance::ClassTypeInfoImplementation::bonsai::OnStateCompletedDecoratorInstance::Class
    Replacement::draft(0x141b5fab0, "bonsai_141b5fab0", tpl_37852::<0x143ca760c, 0x14020e8f0, 0x143c8fabc> as *const ()), // sub_141b5fab0
    Replacement::draft(0x141b5fca0, "bonsai_141b5fca0", fn_141b5fca0 as *const ()), // sub_141b5fca0
    Replacement::draft_throws(0x141b5fe50, "bonsai_141b5fe50", fn_141b5fe50 as *const (), Throws::Discard), // sub_141b5fe50
    Replacement::draft_throws(0x141b60680, "bonsai_141b60680", fn_141b60680 as *const (), Throws::Discard), // sub_141b60680
    Replacement::draft_throws(0x141b60770, "bonsai_141b60770", fn_141b60770 as *const (), Throws::Discard), // sub_141b60770
    Replacement::draft_throws(0x141b60890, "bonsai_141b60890", fn_141b60890 as *const (), Throws::Caught), // sub_141b60890
    Replacement::draft(0x141b60b50, "bonsai_141b60b50", fn_141b60b50 as *const ()), // sub_141b60b50
    Replacement::draft(0x141b60c40, "bonsai_141b60c40", fn_141b60c40 as *const ()), // sub_141b60c40
    Replacement::draft_throws(0x141b60df0, "bonsai_141b60df0", fn_141b60df0 as *const (), Throws::Caught), // sub_141b60df0
    Replacement::draft_throws(0x141b61450, "bonsai_141b61450", fn_141b61450 as *const (), Throws::Discard), // sub_141b61450
    Replacement::draft_throws(0x141b61a10, "bonsai_141b61a10", fn_141b61a10 as *const (), Throws::Discard), // sub_141b61a10
    Replacement::draft(0x141b623c0, "bonsai_141b623c0", fn_141b623c0 as *const ()), // sub_141b623c0
    Replacement::draft(0x141b62400, "bonsai_141b62400", fn_141b62400 as *const ()), // sub_141b62400
    Replacement::draft(0x141b62420, "bonsai_141b62420", fn_141b62420 as *const ()), // sub_141b62420
    Replacement::draft_throws(0x141b624e0, "bonsai_141b624e0", fn_141b624e0 as *const (), Throws::Discard), // sub_141b624e0
    Replacement::draft(0x141b62810, "bonsai_141b62810", fn_141b62810 as *const ()), // sub_141b62810
    Replacement::draft(0x141b62960, "bonsai_141b62960", tpl_37864::<0x140641a40> as *const ()), // sub_141b62960
    Replacement::draft(0x141b62980, "bonsai_141b62980", fn_141b62980 as *const ()), // sub_141b62980
    Replacement::draft(0x141b62a70, "bonsai_141b62a70", fn_141b62a70 as *const ()), // sub_141b62a70
    Replacement::draft(0x141b62b10, "bonsai_141b62b10", fn_141b62b10 as *const ()), // sub_141b62b10
    Replacement::draft(0x141b62b40, "bonsai_141b62b40", fn_141b62b40 as *const ()), // sub_141b62b40
    Replacement::draft(0x141b62c10, "bonsai_141b62c10", tpl_34605::<0x14321abf0> as *const ()), // sub_141b62c10
    Replacement::draft(0x141b62d50, "bonsai_141b62d50", tpl_97860::<0x14321a930, 0x8, 0x18> as *const ()), // sub_141b62d50
    Replacement::draft(0x141b62e00, "bonsai_141b62e00", tpl_37868::<0x14321a930, 0x8, 0x18> as *const ()), // sub_141b62e00
    Replacement::draft(0x141b62ee0, "bonsai_141b62ee0", fn_141b62ee0 as *const ()), // sub_141b62ee0
    Replacement::draft(0x141b63080, "bonsai_141b63080", tpl_37689 as *const ()), // sub_141b63080
    Replacement::draft(0x141b633d0, "bonsai_141b633d0", fn_141b633d0 as *const ()), // sub_141b633d0
    Replacement::draft(0x141b634b0, "bonsai_141b634b0", fn_141b634b0 as *const ()), // sub_141b634b0
    Replacement::draft_throws(0x141b63530, "bonsai_141b63530", fn_141b63530 as *const (), Throws::Discard), // sub_141b63530
    Replacement::draft(0x141b63650, "bonsai_141b63650", fn_141b63650 as *const ()), // sub_141b63650
    Replacement::draft_throws(0x141b63910, "bonsai_141b63910", fn_141b63910 as *const (), Throws::Caught), // sub_141b63910
    Replacement::draft(0x141b63ea0, "bonsai_141b63ea0", fn_141b63ea0 as *const ()), // sub_141b63ea0
    Replacement::draft_throws(0x141b63f30, "bonsai_141b63f30", fn_141b63f30 as *const (), Throws::Discard), // sub_141b63f30
    Replacement::draft_throws(0x141b63ff0, "bonsai_141b63ff0", fn_141b63ff0 as *const (), Throws::Caught), // sub_141b63ff0
    Replacement::draft_throws(0x141b64210, "bonsai_141b64210", fn_141b64210 as *const (), Throws::Discard), // sub_141b64210
    Replacement::draft(0x141b64a30, "bonsai_141b64a30", fn_141b64a30 as *const ()), // sub_141b64a30
    Replacement::draft(0x141b64aa0, "bonsai_141b64aa0", tpl_37040::<0x143c91ad4, 0x14020e8f0, 0x143c91b20> as *const ()), // sub_141b64aa0
    Replacement::draft(0x141b64c60, "bonsai_141b64c60", fn_141b64c60 as *const ()), // sub_141b64c60
    Replacement::draft(0x141b64ec0, "bonsai_141b64ec0", fn_141b64ec0 as *const ()), // sub_141b64ec0
    Replacement::draft(0x141b64f50, "bonsai_141b64f50", fn_141b64f50 as *const ()), // sub_141b64f50
    Replacement::draft(0x141b64f90, "bonsai_141b64f90", fn_141b64f90 as *const ()), // sub_141b64f90
    Replacement::draft(0x141b650a0, "bonsai_141b650a0", fn_141b650a0 as *const ()), // sub_141b650a0
    Replacement::draft(0x141b65120, "bonsai_141b65120", fn_141b65120 as *const ()), // sub_141b65120
    Replacement::draft(0x141b654b0, "bonsai_141b654b0", fn_141b654b0 as *const ()), // sub_141b654b0
    Replacement::draft_throws(0x141b65890, "bonsai_141b65890", fn_141b65890 as *const (), Throws::Discard), // sub_141b65890
    Replacement::draft(0x141b66010, "bonsai_141b66010", tpl_34605::<0x14321d330> as *const ()), // sub_141b66010
    Replacement::draft(0x141b66470, "bonsai_141b66470", fn_141b66470 as *const ()), // sub_141b66470
    Replacement::draft(0x141b66510, "bonsai_141b66510", fn_141b66510 as *const ()), // sub_141b66510
    Replacement::draft(0x141b66580, "bonsai_141b66580", fn_141b66580 as *const ()), // sub_141b66580
    Replacement::draft_throws(0x141b666a0, "bonsai_141b666a0", fn_141b666a0 as *const (), Throws::Discard), // sub_141b666a0
    Replacement::draft(0x141b66cb0, "bonsai_141b66cb0", tpl_34394::<0x10> as *const ()), // sub_141b66cb0
    Replacement::draft(0x141b66d50, "bonsai_141b66d50", tpl_34605::<0x143220278> as *const ()), // sub_141b66d50
    Replacement::draft(0x141b66e90, "bonsai_141b66e90", fn_141b66e90 as *const ()), // sub_141b66e90
    Replacement::draft(0x141b66f60, "bonsai_141b66f60", fn_141b66f60 as *const ()), // sub_141b66f60
    Replacement::draft(0x141b66fb0, "bonsai_141b66fb0", tpl_34607::<0x1401de830> as *const ()), // sub_141b66fb0
    Replacement::draft(0x141b67060, "bonsai_141b67060", tpl_35099::<0x20, 0x40> as *const ()), // sub_141b67060
    Replacement::draft(0x141b674b0, "bonsai_141b674b0", fn_141b674b0 as *const ()), // sub_141b674b0
    Replacement::draft(0x141b67870, "bonsai_141b67870", fn_141b67870 as *const ()), // sub_141b67870
    Replacement::draft(0x141b67a80, "bonsai_141b67a80", fn_141b67a80 as *const ()), // sub_141b67a80
    Replacement::draft(0x141b67b10, "bonsai_141b67b10", fn_141b67b10 as *const ()), // sub_141b67b10
    Replacement::draft(0x141b67c50, "bonsai_141b67c50", fn_141b67c50 as *const ()), // sub_141b67c50
    Replacement::draft(0x141b68170, "bonsai_141b68170", fn_141b68170 as *const ()), // sub_141b68170
    Replacement::draft(0x141b68450, "bonsai_141b68450", fn_141b68450 as *const ()), // sub_141b68450
    Replacement::draft(0x141b68af0, "bonsai_141b68af0", fn_141b68af0 as *const ()), // sub_141b68af0
    Replacement::draft(0x141b68d50, "bonsai_141b68d50", fn_141b68d50 as *const ()), // sub_141b68d50
    Replacement::draft(0x141b68f60, "bonsai_141b68f60", fn_141b68f60 as *const ()), // sub_141b68f60
    Replacement::draft(0x141b68fa0, "bonsai_141b68fa0", fn_141b68fa0 as *const ()), // sub_141b68fa0
    Replacement::draft(0x141b68fc0, "bonsai_141b68fc0", tpl_36992::<0xd0, 0x104> as *const ()), // sub_141b68fc0
    Replacement::draft_throws(0x141b68ff0, "bonsai_141b68ff0", fn_141b68ff0 as *const (), Throws::Discard), // sub_141b68ff0
    Replacement::draft(0x141b69580, "bonsai_141b69580", fn_141b69580 as *const ()), // sub_141b69580
    Replacement::draft(0x141b698f0, "bonsai_141b698f0", fn_141b698f0 as *const ()), // sub_141b698f0
    Replacement::draft(0x141b69be0, "bonsai_141b69be0", fn_141b69be0 as *const ()), // sub_141b69be0
    Replacement::draft(0x141b69d90, "bonsai_141b69d90", tpl_35702 as *const ()), // sub_141b69d90
    Replacement::draft(0x141b69e40, "bonsai_141b69e40", fn_141b69e40 as *const ()), // sub_141b69e40
    Replacement::draft(0x141b6a040, "bonsai_141b6a040", fn_141b6a040 as *const ()), // sub_141b6a040
    Replacement::draft(0x141b6a3e0, "bonsai_141b6a3e0", fn_141b6a3e0 as *const ()), // sub_141b6a3e0
    Replacement::draft(0x141b6a4a0, "bonsai_141b6a4a0", fn_141b6a4a0 as *const ()), // sub_141b6a4a0
    Replacement::draft_throws(0x141b6a600, "bonsai_141b6a600", fn_141b6a600 as *const (), Throws::Discard), // sub_141b6a600
    Replacement::draft(0x141b6a9f0, "bonsai_141b6a9f0", fn_141b6a9f0 as *const ()), // sub_141b6a9f0
    Replacement::draft_throws(0x141b6acd0, "bonsai_141b6acd0", fn_141b6acd0 as *const (), Throws::Discard), // sub_141b6acd0
    Replacement::draft(0x141b6aff0, "bonsai_141b6aff0", fn_141b6aff0 as *const ()), // sub_141b6aff0
    Replacement::draft(0x141b6b280, "bonsai_141b6b280", fn_141b6b280 as *const ()), // sub_141b6b280
    Replacement::draft(0x141b6b540, "bonsai_141b6b540", fn_141b6b540 as *const ()), // sub_141b6b540
    Replacement::draft(0x141b6b5a0, "bonsai_141b6b5a0", fn_141b6b5a0 as *const ()), // sub_141b6b5a0
    Replacement::draft_throws(0x141b6b620, "bonsai_141b6b620", fn_141b6b620 as *const (), Throws::Discard), // sub_141b6b620
    Replacement::draft(0x141b6bc00, "bonsai_141b6bc00", fn_141b6bc00 as *const ()), // sub_141b6bc00
    Replacement::draft_throws(0x141b6bc40, "bonsai_141b6bc40", fn_141b6bc40 as *const (), Throws::Caught), // sub_141b6bc40
    Replacement::draft_throws(0x141b6be50, "bonsai_141b6be50", fn_141b6be50 as *const (), Throws::Discard), // sub_141b6be50
    Replacement::draft_throws(0x141b6bee0, "bonsai_141b6bee0", fn_141b6bee0 as *const (), Throws::Caught), // sub_141b6bee0
    Replacement::draft(0x141b6c100, "bonsai_141b6c100", fn_141b6c100 as *const ()), // sub_141b6c100
    Replacement::draft(0x141b6c230, "bonsai_141b6c230", fn_141b6c230 as *const ()), // sub_141b6c230
    Replacement::draft_throws(0x141b6c320, "bonsai_141b6c320", fn_141b6c320 as *const (), Throws::Discard), // sub_141b6c320
    Replacement::draft(0x141b6c3d0, "bonsai_141b6c3d0", fn_141b6c3d0 as *const ()), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_17
    Replacement::draft(0x141b6c400, "bonsai_141b6c400", fn_141b6c400 as *const ()), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_16
    Replacement::draft(0x141b6c480, "bonsai_141b6c480", tpl_37610::<0xc0, 0xc8> as *const ()), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_15
    Replacement::draft(0x141b6c4f0, "bonsai_141b6c4f0", fn_141b6c4f0 as *const ()), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_14
    Replacement::draft_throws(0x141b6c530, "bonsai_141b6c530", fn_141b6c530 as *const (), Throws::Discard), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_13
    Replacement::draft(0x141b6c560, "bonsai_141b6c560", fn_141b6c560 as *const ()), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_7
    Replacement::draft_throws(0x141b6c5a0, "bonsai_141b6c5a0", fn_141b6c5a0 as *const (), Throws::Discard), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_6
    Replacement::draft(0x141b6c620, "bonsai_141b6c620", fn_141b6c620 as *const ()), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_4
    Replacement::draft(0x141b6c6b0, "bonsai_141b6c6b0", fn_141b6c6b0 as *const ()), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_3
    Replacement::draft_throws(0x141b6c7c0, "bonsai_141b6c7c0", tpl_37613::<0xc0, 0xc8, 0x141b6c840> as *const (), Throws::Discard), // bonsai::MoveProgram::bonsai::MoveProgram::vfunc_1
    Replacement::draft_throws(0x141b6c840, "bonsai_141b6c840", fn_141b6c840 as *const (), Throws::Discard), // sub_141b6c840
    Replacement::draft(0x141b6cde0, "bonsai_141b6cde0", fn_141b6cde0 as *const ()), // sub_141b6cde0
    Replacement::draft(0x141b6ce50, "bonsai_141b6ce50", fn_141b6ce50 as *const ()), // sub_141b6ce50
    Replacement::draft_throws(0x141b6cf70, "bonsai_141b6cf70", fn_141b6cf70 as *const (), Throws::Discard), // sub_141b6cf70
    Replacement::draft_throws(0x141b6cfd0, "bonsai_141b6cfd0", fn_141b6cfd0 as *const (), Throws::Discard), // sub_141b6cfd0
    Replacement::draft_throws(0x141b6d030, "bonsai_141b6d030", fn_141b6d030 as *const (), Throws::Discard), // sub_141b6d030
    Replacement::draft_throws(0x141b6d190, "bonsai_141b6d190", tpl_37815::<0x18, 0x20, 0x141b6d030> as *const (), Throws::Discard), // bonsai::ModifyIntPropertyDecoratorInstance::bonsai::ModifyIntPropertyDecoratorInstance::vfunc_9
    Replacement::draft_throws(0x141b6d1d0, "bonsai_141b6d1d0", tpl_37920::<0x10, 0x28, 0x20, 0x141b6d030> as *const (), Throws::Discard), // bonsai::ModifyIntPropertyDecoratorInstance::bonsai::ModifyIntPropertyDecoratorInstance::vfunc_4
    Replacement::draft(0x141b6d7b0, "bonsai_141b6d7b0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::ModifyIntPropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::ModifyIntPropertyDecoratorInstance::Cla
    Replacement::draft_throws(0x141b6d990, "bonsai_141b6d990", fn_141b6d990 as *const (), Throws::Discard), // sub_141b6d990
    Replacement::draft_throws(0x141b6da30, "bonsai_141b6da30", fn_141b6da30 as *const (), Throws::Caught), // sub_141b6da30
    Replacement::draft_throws(0x141b6dc40, "bonsai_141b6dc40", fn_141b6dc40 as *const (), Throws::Discard), // sub_141b6dc40
    Replacement::draft_throws(0x141b6ded0, "bonsai_141b6ded0", fn_141b6ded0 as *const (), Throws::Discard), // sub_141b6ded0
    Replacement::draft_throws(0x141b6e110, "bonsai_141b6e110", fn_141b6e110 as *const (), Throws::Discard), // sub_141b6e110
    Replacement::draft_throws(0x141b6e250, "bonsai_141b6e250", fn_141b6e250 as *const (), Throws::Discard), // sub_141b6e250
    Replacement::draft(0x141b6e300, "bonsai_141b6e300", fn_141b6e300 as *const ()), // sub_141b6e300
    Replacement::draft_throws(0x141b6e510, "bonsai_141b6e510", fn_141b6e510 as *const (), Throws::Discard), // sub_141b6e510
    Replacement::draft_throws(0x141b6e540, "bonsai_141b6e540", fn_141b6e540 as *const (), Throws::Discard), // sub_141b6e540
    Replacement::draft_throws(0x141b6e960, "bonsai_141b6e960", fn_141b6e960 as *const (), Throws::Discard), // sub_141b6e960
    Replacement::draft(0x141b6e9b0, "bonsai_141b6e9b0", fn_141b6e9b0 as *const ()), // sub_141b6e9b0
    Replacement::draft(0x141b6ea20, "bonsai_141b6ea20", fn_141b6ea20 as *const ()), // sub_141b6ea20
    Replacement::draft_throws(0x141b6ea90, "bonsai_141b6ea90", fn_141b6ea90 as *const (), Throws::Discard), // sub_141b6ea90
    Replacement::draft_throws(0x141b6ead0, "bonsai_141b6ead0", fn_141b6ead0 as *const (), Throws::Discard), // sub_141b6ead0
    Replacement::draft_throws(0x141b6eb30, "bonsai_141b6eb30", fn_141b6eb30 as *const (), Throws::Discard), // sub_141b6eb30
    Replacement::draft_throws(0x141b6eb70, "bonsai_141b6eb70", fn_141b6eb70 as *const (), Throws::Discard), // sub_141b6eb70
    Replacement::draft_throws(0x141b6ebf0, "bonsai_141b6ebf0", fn_141b6ebf0 as *const (), Throws::Discard), // sub_141b6ebf0
    Replacement::draft(0x141b6ed60, "bonsai_141b6ed60", tpl_37935::<0x8, 0x10> as *const ()), // sub_141b6ed60
    Replacement::draft_throws(0x141b6ed80, "bonsai_141b6ed80", fn_141b6ed80 as *const (), Throws::Discard), // sub_141b6ed80
    Replacement::draft_throws(0x141b6eef0, "bonsai_141b6eef0", fn_141b6eef0 as *const (), Throws::Discard), // sub_141b6eef0
    Replacement::draft_throws(0x141b6f0c0, "bonsai_141b6f0c0", fn_141b6f0c0 as *const (), Throws::Caught), // sub_141b6f0c0
    Replacement::draft_throws(0x141b6f270, "bonsai_141b6f270", fn_141b6f270 as *const (), Throws::Caught), // sub_141b6f270
    Replacement::draft_throws(0x141b6f490, "bonsai_141b6f490", fn_141b6f490 as *const (), Throws::Caught), // sub_141b6f490
    Replacement::draft_throws(0x141b6f6f0, "bonsai_141b6f6f0", fn_141b6f6f0 as *const (), Throws::Caught), // sub_141b6f6f0
    Replacement::draft_throws(0x141b6f940, "bonsai_141b6f940", fn_141b6f940 as *const (), Throws::Caught), // sub_141b6f940
    Replacement::draft_throws(0x141b6fb90, "bonsai_141b6fb90", fn_141b6fb90 as *const (), Throws::Caught), // sub_141b6fb90
    Replacement::draft_throws(0x141b6fd60, "bonsai_141b6fd60", fn_141b6fd60 as *const (), Throws::Discard), // sub_141b6fd60
    Replacement::draft_throws(0x141b700f0, "bonsai_141b700f0", fn_141b700f0 as *const (), Throws::Discard), // sub_141b700f0
    Replacement::draft_throws(0x141b70160, "bonsai_141b70160", fn_141b70160 as *const (), Throws::Discard), // sub_141b70160
    Replacement::draft_throws(0x141b703b0, "bonsai_141b703b0", fn_141b703b0 as *const (), Throws::Discard), // sub_141b703b0
    Replacement::draft_throws(0x141b70550, "bonsai_141b70550", fn_141b70550 as *const (), Throws::Caught), // sub_141b70550
    Replacement::draft_throws(0x141b70680, "bonsai_141b70680", fn_141b70680 as *const (), Throws::Caught), // sub_141b70680
    Replacement::draft_throws(0x141b70850, "bonsai_141b70850", fn_141b70850 as *const (), Throws::Discard), // sub_141b70850
    Replacement::draft_throws(0x141b70940, "bonsai_141b70940", fn_141b70940 as *const (), Throws::Discard), // sub_141b70940
    Replacement::draft_throws(0x141b70a70, "bonsai_141b70a70", fn_141b70a70 as *const (), Throws::Discard), // sub_141b70a70
    Replacement::draft_throws(0x141b70c20, "bonsai_141b70c20", fn_141b70c20 as *const (), Throws::Discard), // sub_141b70c20
    Replacement::draft_throws(0x141b70e80, "bonsai_141b70e80", tpl_37952::<0x1429868d0, 0x1401fa730, 0x1403ff100, 0x1421158a0, 0x141b70f93, 0x141b70f93> as *const (), Throws::Discard), // sub_141b70e80
    Replacement::draft_throws(0x141b70fc0, "bonsai_141b70fc0", tpl_37953::<0x142119010, 0x142118e10, 0x1401fa730, 0x1404056a0, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1426cf238, 0x141ef4820, 0x141b710b4, 0x141b710c6, 0x141b710db, 0x141b710b4, 0x141b710c6, 0x141b710db> as *const (), Throws::Discard), // sub_141b70fc0
    Replacement::draft_throws(0x141b71150, "bonsai_141b71150", tpl_37953::<0x142119010, 0x142118e10, 0x1401fa730, 0x1403ff100, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x143222640, 0x141ef4820, 0x141b71244, 0x141b71256, 0x141b7126b, 0x141b71244, 0x141b71256, 0x141b7126b> as *const (), Throws::Discard), // sub_141b71150
    Replacement::draft_throws(0x141b71280, "bonsai_141b71280", fn_141b71280 as *const (), Throws::Discard), // sub_141b71280
    Replacement::draft(0x141b71610, "bonsai_141b71610", fn_141b71610 as *const ()), // bonsai::LogicBlockDecoratorInstance::bonsai::LogicBlockDecoratorInstance::vfunc_1
    Replacement::draft(0x141b71c40, "bonsai_141b71c40", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::LogicBlockDecoratorInstance::ClassTypeInfoImplementation::bonsai::LogicBlockDecoratorInstance::ClassTypeInfoImpl
    Replacement::draft(0x141b71e20, "bonsai_141b71e20", tpl_37771::<0x68, 0x70> as *const ()), // bonsai::LevitateProgram::bonsai::LevitateProgram::vfunc_15
    Replacement::draft(0x141b71e80, "bonsai_141b71e80", fn_141b71e80 as *const ()), // sub_141b71e80
    Replacement::draft(0x141b72050, "bonsai_141b72050", tpl_37773::<0x68, 0x70> as *const ()), // bonsai::LevitateProgram::bonsai::LevitateProgram::vfunc_6
    Replacement::draft(0x141b720a0, "bonsai_141b720a0", tpl_37773::<0x68, 0x70> as *const ()), // bonsai::LevitateProgram::bonsai::LevitateProgram::vfunc_5
    Replacement::draft(0x141b720f0, "bonsai_141b720f0", tpl_37774::<0x68, 0x70, 0x141b71e80> as *const ()), // bonsai::LevitateProgram::bonsai::LevitateProgram::vfunc_1
    Replacement::draft(0x141b72160, "bonsai_141b72160", tpl_37776::<0x1431f8560, 0x60, 0x1430f6b98, 0x38, 0x1432316d0> as *const ()), // sub_141b72160
    Replacement::draft(0x141b72190, "bonsai_141b72190", fn_141b72190 as *const ()), // sub_141b72190
    Replacement::draft_throws(0x141b72220, "bonsai_141b72220", fn_141b72220 as *const (), Throws::Discard), // sub_141b72220
    Replacement::draft(0x141b722a0, "bonsai_141b722a0", fn_141b722a0 as *const ()), // sub_141b722a0
    Replacement::draft_throws(0x141b723d0, "bonsai_141b723d0", fn_141b723d0 as *const (), Throws::Discard), // sub_141b723d0
    Replacement::draft_throws(0x141b72430, "bonsai_141b72430", fn_141b72430 as *const (), Throws::Discard), // sub_141b72430
    Replacement::draft_throws(0x141b72600, "bonsai_141b72600", fn_141b72600 as *const (), Throws::Discard), // sub_141b72600
    Replacement::draft(0x141b72670, "bonsai_141b72670", fn_141b72670 as *const ()), // sub_141b72670
    Replacement::draft(0x141b726a0, "bonsai_141b726a0", fn_141b726a0 as *const ()), // sub_141b726a0
    Replacement::draft(0x141b72700, "bonsai_141b72700", fn_141b72700 as *const ()), // sub_141b72700
    Replacement::draft(0x141b72780, "bonsai_141b72780", fn_141b72780 as *const ()), // sub_141b72780
    Replacement::draft_throws(0x141b727f0, "bonsai_141b727f0", fn_141b727f0 as *const (), Throws::Discard), // sub_141b727f0
    Replacement::draft(0x141b72890, "bonsai_141b72890", tpl_37005::<0xe8, 0x14052f984, 0x1401dffd0, 0x38, 0x141b722a0> as *const ()), // sub_141b72890
    Replacement::draft_throws(0x141b728f0, "bonsai_141b728f0", fn_141b728f0 as *const (), Throws::Discard), // sub_141b728f0
    Replacement::draft_throws(0x141b72990, "bonsai_141b72990", fn_141b72990 as *const (), Throws::Discard), // bonsai::KnowledgeConditionDecoratorInstance::bonsai::KnowledgeConditionDecoratorInstance::vfunc_33
    Replacement::draft_throws(0x141b72aa0, "bonsai_141b72aa0", fn_141b72aa0 as *const (), Throws::Discard), // bonsai::KnowledgeConditionDecoratorInstance::bonsai::KnowledgeConditionDecoratorInstance::vfunc_1
    Replacement::draft(0x141b72b40, "bonsai_141b72b40", fn_141b72b40 as *const ()), // bonsai::KnowledgeConditionDecoratorInstance::bonsai::KnowledgeConditionDecoratorInstance::vfunc_5
    Replacement::draft_throws(0x141b72c00, "bonsai_141b72c00", fn_141b72c00 as *const (), Throws::Discard), // bonsai::KnowledgeConditionDecoratorInstance::bonsai::KnowledgeConditionDecoratorInstance::vfunc_4
    Replacement::draft(0x141b72ca0, "bonsai_141b72ca0", fn_141b72ca0 as *const ()), // bonsai::KnowledgeConditionDecoratorInstance::bonsai::KnowledgeConditionDecoratorInstance::vfunc_7
    Replacement::draft(0x141b734b0, "bonsai_141b734b0", fn_141b734b0 as *const ()), // sub_141b734b0
    Replacement::draft_throws(0x141b736f0, "bonsai_141b736f0", fn_141b736f0 as *const (), Throws::Caught), // sub_141b736f0
    Replacement::draft_throws(0x141b73a70, "bonsai_141b73a70", fn_141b73a70 as *const (), Throws::Discard), // sub_141b73a70
    Replacement::draft_throws(0x141b73cb0, "bonsai_141b73cb0", fn_141b73cb0 as *const (), Throws::Discard), // sub_141b73cb0
    Replacement::draft_throws(0x141b73f80, "bonsai_141b73f80", tpl_37972::<0x14387cb68, 0x140658530, 0x1402116e0, 0x14278c2d0, 0x1410d6ca0, 0x140658120, 0x1438234a0, 0x14020e980, 0x14020afa0, 0x1420a6b70> as *const (), Throws::Discard), // sub_141b73f80
    Replacement::draft(0x141b74140, "bonsai_141b74140", fn_141b74140 as *const ()), // bonsai::IsPropertySetConditionDecoratorInstance::bonsai::IsPropertySetConditionDecoratorInstance::vfunc_33
    Replacement::draft_throws(0x141b74190, "bonsai_141b74190", fn_141b74190 as *const (), Throws::Discard), // bonsai::IsPropertySetConditionDecoratorInstance::bonsai::IsPropertySetConditionDecoratorInstance::vfunc_1
    Replacement::draft(0x141b74200, "bonsai_141b74200", fn_141b74200 as *const ()), // bonsai::IsPropertySetConditionDecoratorInstance::bonsai::IsPropertySetConditionDecoratorInstance::vfunc_5
    Replacement::draft_throws(0x141b742a0, "bonsai_141b742a0", fn_141b742a0 as *const (), Throws::Discard), // bonsai::IsPropertySetConditionDecoratorInstance::bonsai::IsPropertySetConditionDecoratorInstance::vfunc_4
    Replacement::draft(0x141b74380, "bonsai_141b74380", fn_141b74380 as *const ()), // bonsai::IsPropertySetConditionDecoratorInstance::bonsai::IsPropertySetConditionDecoratorInstance::vfunc_7
    Replacement::draft(0x141b74ae0, "bonsai_141b74ae0", tpl_37771::<0x60, 0x68> as *const ()), // bonsai::IdleBodyProgram::bonsai::IdleBodyProgram::vfunc_15
    Replacement::draft(0x141b74b40, "bonsai_141b74b40", fn_141b74b40 as *const ()), // sub_141b74b40
    Replacement::draft(0x141b74b50, "bonsai_141b74b50", tpl_37773::<0x60, 0x68> as *const ()), // bonsai::IdleBodyProgram::bonsai::IdleBodyProgram::vfunc_6
    Replacement::draft(0x141b74ba0, "bonsai_141b74ba0", tpl_37773::<0x60, 0x68> as *const ()), // bonsai::IdleBodyProgram::bonsai::IdleBodyProgram::vfunc_5
    Replacement::draft(0x141b74bf0, "bonsai_141b74bf0", tpl_37774::<0x60, 0x68, 0x141b74b40> as *const ()), // bonsai::IdleBodyProgram::bonsai::IdleBodyProgram::vfunc_1
    Replacement::draft(0x141b74c60, "bonsai_141b74c60", tpl_37776::<0x1431f8560, 0x58, 0x1430f6b98, 0x38, 0x1432316d0> as *const ()), // sub_141b74c60
    Replacement::draft(0x141b74c90, "bonsai_141b74c90", fn_141b74c90 as *const ()), // sub_141b74c90
    Replacement::draft(0x141b74e00, "bonsai_141b74e00", fn_141b74e00 as *const ()), // sub_141b74e00
    Replacement::draft(0x141b75070, "bonsai_141b75070", fn_141b75070 as *const ()), // sub_141b75070
    Replacement::draft_throws(0x141b75220, "bonsai_141b75220", fn_141b75220 as *const (), Throws::Discard), // sub_141b75220
    Replacement::draft(0x141b754c0, "bonsai_141b754c0", fn_141b754c0 as *const ()), // sub_141b754c0
    Replacement::draft(0x141b75510, "bonsai_141b75510", fn_141b75510 as *const ()), // sub_141b75510
    Replacement::draft(0x141b755f0, "bonsai_141b755f0", fn_141b755f0 as *const ()), // sub_141b755f0
    Replacement::draft(0x141b756a0, "bonsai_141b756a0", tpl_35321::<0x143224e78, 0x8, 0xc, 0x10> as *const ()), // sub_141b756a0
    Replacement::draft(0x141b75790, "bonsai_141b75790", tpl_34671::<0x20> as *const ()), // sub_141b75790
    Replacement::draft(0x141b759c0, "bonsai_141b759c0", fn_141b759c0 as *const ()), // sub_141b759c0
    Replacement::draft(0x141b75a80, "bonsai_141b75a80", fn_141b75a80 as *const ()), // sub_141b75a80
    Replacement::draft(0x141b75ce0, "bonsai_141b75ce0", fn_141b75ce0 as *const ()), // sub_141b75ce0
    Replacement::draft(0x141b75dc0, "bonsai_141b75dc0", fn_141b75dc0 as *const ()), // sub_141b75dc0
    Replacement::draft(0x141b75fc0, "bonsai_141b75fc0", fn_141b75fc0 as *const ()), // sub_141b75fc0
    Replacement::draft(0x141b75ff0, "bonsai_141b75ff0", fn_141b75ff0 as *const ()), // sub_141b75ff0
    Replacement::draft(0x141b76020, "bonsai_141b76020", fn_141b76020 as *const ()), // sub_141b76020
    Replacement::draft(0x141b760d0, "bonsai_141b760d0", fn_141b760d0 as *const ()), // sub_141b760d0
    Replacement::draft(0x141b761d0, "bonsai_141b761d0", fn_141b761d0 as *const ()), // sub_141b761d0
    Replacement::draft_throws(0x141b76510, "bonsai_141b76510", fn_141b76510 as *const (), Throws::Discard), // sub_141b76510
    Replacement::draft(0x141b76720, "bonsai_141b76720", fn_141b76720 as *const ()), // bonsai::HazardServiceInstance::bonsai::HazardServiceInstance::vfunc_6
    Replacement::draft(0x141b76740, "bonsai_141b76740", fn_141b76740 as *const ()), // bonsai::HazardServiceInstance::bonsai::HazardServiceInstance::vfunc_1
    Replacement::draft(0x141b76cc0, "bonsai_141b76cc0", fn_141b76cc0 as *const ()), // bonsai::HazardServiceInstance::ClassTypeInfoImplementation::bonsai::HazardServiceInstance::ClassTypeInfoImplementation::
    Replacement::draft(0x141b76d30, "bonsai_141b76d30", fn_141b76d30 as *const ()), // bonsai::HazardServiceInstance::ClassTypeInfoImplementation::bonsai::HazardServiceInstance::ClassTypeInfoImplementation::
    Replacement::draft(0x141b76ea0, "bonsai_141b76ea0", fn_141b76ea0 as *const ()), // bonsai::GuideSpeedDecoratorInstance::bonsai::GuideSpeedDecoratorInstance::vfunc_33
    Replacement::draft_throws(0x141b76fd0, "bonsai_141b76fd0", fn_141b76fd0 as *const (), Throws::Discard), // bonsai::GuideSpeedDecoratorInstance::bonsai::GuideSpeedDecoratorInstance::vfunc_1
    Replacement::draft(0x141b770c0, "bonsai_141b770c0", fn_141b770c0 as *const ()), // bonsai::GuideSpeedDecoratorInstance::bonsai::GuideSpeedDecoratorInstance::vfunc_4
    Replacement::draft(0x141b778c0, "bonsai_141b778c0", fn_141b778c0 as *const ()), // bonsai::GuideSpeedDecoratorInstance::ClassTypeInfoImplementation::bonsai::GuideSpeedDecoratorInstance::ClassTypeInfoImpl
    Replacement::draft(0x141b77970, "bonsai_141b77970", tpl_37999::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x143225478> as *const ()), // bonsai::GuideSpeedDecoratorInstance::ClassTypeInfoImplementation::bonsai::GuideSpeedDecoratorInstance::ClassTypeInfoImpl
    Replacement::draft(0x141b77af0, "bonsai_141b77af0", fn_141b77af0 as *const ()), // sub_141b77af0
    Replacement::draft(0x141b77b20, "bonsai_141b77b20", fn_141b77b20 as *const ()), // bonsai::FollowSpeedDecoratorInstance::bonsai::FollowSpeedDecoratorInstance::vfunc_33
    Replacement::draft(0x141b77c40, "bonsai_141b77c40", fn_141b77c40 as *const ()), // bonsai::FollowSpeedDecoratorInstance::bonsai::FollowSpeedDecoratorInstance::vfunc_4
    Replacement::draft(0x141b78430, "bonsai_141b78430", fn_141b78430 as *const ()), // bonsai::FollowSpeedDecoratorInstance::ClassTypeInfoImplementation::bonsai::FollowSpeedDecoratorInstance::ClassTypeInfoIm
    Replacement::draft(0x141b784f0, "bonsai_141b784f0", tpl_37999::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x1432255b8> as *const ()), // bonsai::FollowSpeedDecoratorInstance::ClassTypeInfoImplementation::bonsai::FollowSpeedDecoratorInstance::ClassTypeInfoIm
    Replacement::draft_throws(0x141b78670, "bonsai_141b78670", fn_141b78670 as *const (), Throws::Discard), // bonsai::FloatDifferenceConditionDecoratorInstance::bonsai::FloatDifferenceConditionDecoratorInstance::vfunc_34
    Replacement::draft_throws(0x141b787c0, "bonsai_141b787c0", tpl_38004::<0x141b72aa0, 0x110, 0x18, 0x8> as *const (), Throws::Discard), // bonsai::FloatDifferenceConditionDecoratorInstance::bonsai::FloatDifferenceConditionDecoratorInstance::vfunc_1
    Replacement::draft(0x141b78820, "bonsai_141b78820", fn_141b78820 as *const ()), // sub_141b78820
    Replacement::draft(0x141b791b0, "bonsai_141b791b0", tpl_38006::<0x8, 0x10, 0x50, 0x58> as *const ()), // sub_141b791b0
    Replacement::draft(0x141b79230, "bonsai_141b79230", fn_141b79230 as *const ()), // bonsai::FiniteStateMachineStateNodeInstance::bonsai::FiniteStateMachineStateNodeInstance::vfunc_15
    Replacement::draft(0x141b792d0, "bonsai_141b792d0", tpl_38007::<0x10, 0x18> as *const ()), // bonsai::FiniteStateMachineStateNodeInstance::bonsai::FiniteStateMachineStateNodeInstance::vfunc_3
    Replacement::draft(0x141b79300, "bonsai_141b79300", fn_141b79300 as *const ()), // bonsai::FiniteStateMachineStateNodeInstance::bonsai::FiniteStateMachineStateNodeInstance::vfunc_10
    Replacement::draft(0x141b79b30, "bonsai_141b79b30", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::FiniteStateMachineStateNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineStateNodeInstance::C
    Replacement::draft(0x141b79bc0, "bonsai_141b79bc0", tpl_38008::<0x143225fe8, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::FiniteStateMachineStateNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineStateNodeInstance::C
    Replacement::draft_throws(0x141b79d50, "bonsai_141b79d50", fn_141b79d50 as *const (), Throws::Discard), // sub_141b79d50
    Replacement::draft_throws(0x141b79dd0, "bonsai_141b79dd0", fn_141b79dd0 as *const (), Throws::Discard), // sub_141b79dd0
    Replacement::draft_throws(0x141b79fc0, "bonsai_141b79fc0", fn_141b79fc0 as *const (), Throws::Discard), // sub_141b79fc0
    Replacement::draft_throws(0x141b7a0e0, "bonsai_141b7a0e0", fn_141b7a0e0 as *const (), Throws::Discard), // sub_141b7a0e0
    Replacement::draft(0x141b7a2d0, "bonsai_141b7a2d0", fn_141b7a2d0 as *const ()), // bonsai::FiniteStateMachineRootNodeInstance::bonsai::FiniteStateMachineRootNodeInstance::vfunc_6
    Replacement::draft(0x141b7a2f0, "bonsai_141b7a2f0", fn_141b7a2f0 as *const ()), // bonsai::FiniteStateMachineRootNodeInstance::bonsai::FiniteStateMachineRootNodeInstance::vfunc_9
    Replacement::draft_throws(0x141b7a3b0, "bonsai_141b7a3b0", fn_141b7a3b0 as *const (), Throws::Discard), // bonsai::FiniteStateMachineRootNodeInstance::bonsai::FiniteStateMachineRootNodeInstance::vfunc_15
    Replacement::draft_throws(0x141b7a4a0, "bonsai_141b7a4a0", fn_141b7a4a0 as *const (), Throws::Discard), // bonsai::FiniteStateMachineRootNodeInstance::bonsai::FiniteStateMachineRootNodeInstance::vfunc_11
    Replacement::draft_throws(0x141b7a580, "bonsai_141b7a580", fn_141b7a580 as *const (), Throws::Discard), // bonsai::FiniteStateMachineRootNodeInstance::bonsai::FiniteStateMachineRootNodeInstance::vfunc_10
    Replacement::draft_throws(0x141b7a6b0, "bonsai_141b7a6b0", fn_141b7a6b0 as *const (), Throws::Discard), // bonsai::FiniteStateMachineRootNodeInstance::bonsai::FiniteStateMachineRootNodeInstance::vfunc_4
    Replacement::draft(0x141b7a7f0, "bonsai_141b7a7f0", fn_141b7a7f0 as *const ()), // bonsai::FiniteStateMachineRootNodeInstance::bonsai::FiniteStateMachineRootNodeInstance::vfunc_3
    Replacement::draft(0x141b7afd0, "bonsai_141b7afd0", fn_141b7afd0 as *const ()), // bonsai::FiniteStateMachineRootNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineRootNodeInstance::Cla
    Replacement::draft(0x141b7b090, "bonsai_141b7b090", tpl_37770::<0x143226308, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::FiniteStateMachineRootNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineRootNodeInstance::Cla
    Replacement::draft(0x141b7b650, "bonsai_141b7b650", tpl_38020::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48> as *const ()), // bonsai::FiniteStateMachineNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineNodeInstance::ClassTypeIn
    Replacement::draft(0x141b7b6e0, "bonsai_141b7b6e0", tpl_38021::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x143225eb8> as *const ()), // bonsai::FiniteStateMachineNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineNodeInstance::ClassTypeIn
    Replacement::draft(0x141b7b9a0, "bonsai_141b7b9a0", tpl_38022::<0x28, 0x48> as *const ()), // bonsai::FiniteStateMachineExitNodeInstance::bonsai::FiniteStateMachineExitNodeInstance::vfunc_10
    Replacement::draft(0x141b7c1a0, "bonsai_141b7c1a0", tpl_38020::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48> as *const ()), // bonsai::FiniteStateMachineExitNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineExitNodeInstance::Cla
    Replacement::draft(0x141b7c230, "bonsai_141b7c230", tpl_38021::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x143226650> as *const ()), // bonsai::FiniteStateMachineExitNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineExitNodeInstance::Cla
    Replacement::draft_throws(0x141b7c3b0, "bonsai_141b7c3b0", fn_141b7c3b0 as *const (), Throws::Discard), // bonsai::FiniteStateMachineConnectionNodeInstance::bonsai::FiniteStateMachineConnectionNodeInstance::vfunc_2
    Replacement::draft(0x141b7c500, "bonsai_141b7c500", tpl_38022::<0x28, 0x48> as *const ()), // bonsai::FiniteStateMachineConnectionNodeInstance::bonsai::FiniteStateMachineConnectionNodeInstance::vfunc_10
    Replacement::draft(0x141b7cd00, "bonsai_141b7cd00", fn_141b7cd00 as *const ()), // bonsai::FiniteStateMachineConnectionNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineConnectionNodeI
    Replacement::draft(0x141b7cd90, "bonsai_141b7cd90", fn_141b7cd90 as *const ()), // bonsai::FiniteStateMachineConnectionNodeInstance::ClassTypeInfoImplementation::bonsai::FiniteStateMachineConnectionNodeI
    Replacement::draft(0x141b7cf10, "bonsai_141b7cf10", tpl_38026::<0x48, 0x20> as *const ()), // sub_141b7cf10
    Replacement::draft(0x141b7cf30, "bonsai_141b7cf30", tpl_38027::<0x28, 0x50> as *const ()), // sub_141b7cf30
    Replacement::draft(0x141b7cf90, "bonsai_141b7cf90", tpl_38028::<0x28, 0x58> as *const ()), // sub_141b7cf90
    Replacement::draft_throws(0x141b7cff0, "bonsai_141b7cff0", fn_141b7cff0 as *const (), Throws::Discard), // sub_141b7cff0
    Replacement::draft_throws(0x141b7d0d0, "bonsai_141b7d0d0", fn_141b7d0d0 as *const (), Throws::Discard), // sub_141b7d0d0
    Replacement::draft_throws(0x141b7d150, "bonsai_141b7d150", fn_141b7d150 as *const (), Throws::Discard), // sub_141b7d150
    Replacement::draft_throws(0x141b7d2a0, "bonsai_141b7d2a0", fn_141b7d2a0 as *const (), Throws::Discard), // sub_141b7d2a0
    Replacement::draft_throws(0x141b7d460, "bonsai_141b7d460", fn_141b7d460 as *const (), Throws::Discard), // sub_141b7d460
    Replacement::draft_throws(0x141b7d540, "bonsai_141b7d540", fn_141b7d540 as *const (), Throws::Caught), // sub_141b7d540
    Replacement::draft_throws(0x141b7d7c0, "bonsai_141b7d7c0", fn_141b7d7c0 as *const (), Throws::Discard), // sub_141b7d7c0
    Replacement::draft_throws(0x141b7db20, "bonsai_141b7db20", fn_141b7db20 as *const (), Throws::Discard), // sub_141b7db20
    Replacement::draft_throws(0x141b7dce0, "bonsai_141b7dce0", fn_141b7dce0 as *const (), Throws::Discard), // sub_141b7dce0
    Replacement::draft(0x141b7df10, "bonsai_141b7df10", fn_141b7df10 as *const ()), // bonsai::FindWorkspotServiceInstance::bonsai::FindWorkspotServiceInstance::vfunc_6
    Replacement::draft(0x141b7e6d0, "bonsai_141b7e6d0", fn_141b7e6d0 as *const ()), // sub_141b7e6d0
    Replacement::draft(0x141b7e840, "bonsai_141b7e840", tpl_35199::<0x14387c500, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSTaskInstance::ClassTypeInfoImplementation::bonsai::EQSTaskInstance::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141b7e930, "bonsai_141b7e930", tpl_35199::<0x14387c500, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSTaskInstance::ClassTypeInfoImplementation::bonsai::EQSTaskInstance::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141b7eb10, "bonsai_141b7eb10", tpl_35199::<0x14387c500, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSTaskInstance::ClassTypeInfoImplementation::bonsai::EQSTaskInstance::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141b7ec00, "bonsai_141b7ec00", tpl_35199::<0x14387c500, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSTaskInstance::ClassTypeInfoImplementation::bonsai::EQSTaskInstance::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141b7ec80, "bonsai_141b7ec80", tpl_35199::<0x14387c500, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSTaskInstance::ClassTypeInfoImplementation::bonsai::EQSTaskInstance::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141b7f290, "bonsai_141b7f290", fn_141b7f290 as *const ()), // sub_141b7f290
    Replacement::draft(0x141b7f2a0, "bonsai_141b7f2a0", fn_141b7f2a0 as *const ()), // sub_141b7f2a0
    Replacement::draft(0x141b7f2c0, "bonsai_141b7f2c0", fn_141b7f2c0 as *const ()), // bonsai::EQSTaskInstance::bonsai::EQSTaskInstance::vfunc_3
    Replacement::draft(0x141b7f2f0, "bonsai_141b7f2f0", fn_141b7f2f0 as *const ()), // bonsai::EQSTaskInstance::bonsai::EQSTaskInstance::vfunc_2
    Replacement::draft_throws(0x141b7f320, "bonsai_141b7f320", tpl_38040::<0x30, 0x8, 0x28, 0x140436250> as *const (), Throws::Discard), // sub_141b7f320
    Replacement::draft_throws(0x141b7f350, "bonsai_141b7f350", fn_141b7f350 as *const (), Throws::Discard), // bonsai::EQSSphereGeneratorNodeInstance::bonsai::EQSSphereGeneratorNodeInstance::vfunc_3
    Replacement::draft(0x141b7fde0, "bonsai_141b7fde0", fn_141b7fde0 as *const ()), // bonsai::EQSScoreNodeInstance::bonsai::EQSScoreNodeInstance::vfunc_3
    Replacement::draft(0x141b7fe20, "bonsai_141b7fe20", tpl_38042::<0x1432283f0, 0x8, 0x20, 0x142442de0, 0x10> as *const ()), // sub_141b7fe20
    Replacement::draft(0x141b805c0, "bonsai_141b805c0", fn_141b805c0 as *const ()), // bonsai::EQSRootNodeInstance::bonsai::EQSRootNodeInstance::vfunc_3
    Replacement::draft_throws(0x141b80d00, "bonsai_141b80d00", fn_141b80d00 as *const (), Throws::Caught), // bonsai::EQSRaycastToTargetOnNavmeshTaskInstance::bonsai::EQSRaycastToTargetOnNavmeshTaskInstance::vfunc_4
    Replacement::draft_throws(0x141b81680, "bonsai_141b81680", fn_141b81680 as *const (), Throws::Caught), // sub_141b81680
    Replacement::draft_throws(0x141b816f0, "bonsai_141b816f0", fn_141b816f0 as *const (), Throws::Caught), // bonsai::EQSRandomEuclideanDistanceTaskInstance::bonsai::EQSRandomEuclideanDistanceTaskInstance::vfunc_4
    Replacement::draft(0x141b81fe0, "bonsai_141b81fe0", tpl_38047::<0x8, 0x143c8d164, 0x10, 0x580e0, 0x141b91990> as *const ()), // sub_141b81fe0
    Replacement::draft_throws(0x141b82020, "bonsai_141b82020", tpl_38048::<0x8, 0x143c8d164, 0x10, 0x580e0, 0x141b91ac0> as *const (), Throws::Discard), // sub_141b82020
    Replacement::draft_throws(0x141b82060, "bonsai_141b82060", fn_141b82060 as *const (), Throws::Discard), // bonsai::EQSQueryServiceInstance::bonsai::EQSQueryServiceInstance::vfunc_6
    Replacement::draft_throws(0x141b82110, "bonsai_141b82110", fn_141b82110 as *const (), Throws::Caught), // sub_141b82110
    Replacement::draft(0x141b822d0, "bonsai_141b822d0", fn_141b822d0 as *const ()), // bonsai::EQSQueryServiceInstance::bonsai::EQSQueryServiceInstance::vfunc_2
    Replacement::draft_throws(0x141b82300, "bonsai_141b82300", fn_141b82300 as *const (), Throws::Discard), // bonsai::EQSQueryServiceInstance::bonsai::EQSQueryServiceInstance::vfunc_1
    Replacement::draft(0x141b82320, "bonsai_141b82320", tpl_38047::<0x8, 0x143c8d164, 0x10, 0x580e0, 0x141b91990> as *const ()), // bonsai::EQSQueryServiceInstance::bonsai::EQSQueryServiceInstance::vfunc_4
    Replacement::draft_throws(0x141b82360, "bonsai_141b82360", tpl_38048::<0x8, 0x143c8d164, 0x10, 0x580e0, 0x141b91ac0> as *const (), Throws::Discard), // bonsai::EQSQueryServiceInstance::bonsai::EQSQueryServiceInstance::vfunc_3
    Replacement::draft(0x141b82ae0, "bonsai_141b82ae0", fn_141b82ae0 as *const ()), // sub_141b82ae0
    Replacement::draft(0x141b82b60, "bonsai_141b82b60", fn_141b82b60 as *const ()), // sub_141b82b60
    Replacement::draft_throws(0x141b82bc0, "bonsai_141b82bc0", fn_141b82bc0 as *const (), Throws::Discard), // sub_141b82bc0
    Replacement::draft_throws(0x141b82dc0, "bonsai_141b82dc0", fn_141b82dc0 as *const (), Throws::Discard), // sub_141b82dc0
    Replacement::draft(0x141b82ec0, "bonsai_141b82ec0", fn_141b82ec0 as *const ()), // sub_141b82ec0
    Replacement::draft(0x141b82ee0, "bonsai_141b82ee0", fn_141b82ee0 as *const ()), // sub_141b82ee0
    Replacement::draft(0x141b83010, "bonsai_141b83010", fn_141b83010 as *const ()), // sub_141b83010
    Replacement::draft(0x141b83050, "bonsai_141b83050", fn_141b83050 as *const ()), // sub_141b83050
    Replacement::draft(0x141b831a0, "bonsai_141b831a0", fn_141b831a0 as *const ()), // sub_141b831a0
    Replacement::draft_throws(0x141b832a0, "bonsai_141b832a0", fn_141b832a0 as *const (), Throws::Discard), // sub_141b832a0
    Replacement::draft(0x141b83650, "bonsai_141b83650", fn_141b83650 as *const ()), // sub_141b83650
    Replacement::draft(0x141b83680, "bonsai_141b83680", fn_141b83680 as *const ()), // sub_141b83680
    Replacement::draft(0x141b836c0, "bonsai_141b836c0", fn_141b836c0 as *const ()), // sub_141b836c0
    Replacement::draft_throws(0x141b83730, "bonsai_141b83730", fn_141b83730 as *const (), Throws::Discard), // sub_141b83730
    Replacement::draft(0x141b83850, "bonsai_141b83850", fn_141b83850 as *const ()), // sub_141b83850
    Replacement::draft(0x141b839c0, "bonsai_141b839c0", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141b83a20, "bonsai_141b83a20", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_41
    Replacement::draft(0x141b83b20, "bonsai_141b83b20", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141b83b80, "bonsai_141b83b80", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_43
    Replacement::draft(0x141b83d60, "bonsai_141b83d60", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141b83dc0, "bonsai_141b83dc0", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_49
    Replacement::draft(0x141b83ec0, "bonsai_141b83ec0", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141b83f20, "bonsai_141b83f20", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_51
    Replacement::draft(0x141b83f80, "bonsai_141b83f80", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141b83fe0, "bonsai_141b83fe0", tpl_35199::<0x14387c568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141b84090, "bonsai_141b84090", fn_141b84090 as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141b84140, "bonsai_141b84140", fn_141b84140 as *const ()), // bonsai::EQSQueryInstance::ClassTypeInfoImplementation::bonsai::EQSQueryInstance::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141b842e0, "unknown_141b842e0", fn_141b842e0 as *const ()), // sub_141b842e0
    Replacement::draft(0x141b84320, "unknown_141b84320", fn_141b84320 as *const ()), // sub_141b84320
    Replacement::draft(0x141b845a0, "unknown_141b845a0", tpl_37792::<0x40, 0x38, 0x18, 0x28, 0x10, 0x140666d20> as *const ()), // sub_141b845a0
    Replacement::draft(0x141b84640, "unknown_141b84640", tpl_37793::<0x18, 0x10, 0x38, 0x28, 0x140666d20> as *const ()), // sub_141b84640
    Replacement::draft(0x141b84690, "unknown_141b84690", fn_141b84690 as *const ()), // sub_141b84690
    Replacement::draft(0x141b84710, "unknown_141b84710", fn_141b84710 as *const ()), // sub_141b84710
    Replacement::draft(0x141b84c30, "bonsai_141b84c30", tpl_35199::<0x14387c578, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSNodeInstance::ClassTypeInfoImplementation::bonsai::EQSNodeInstance::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141b84d20, "bonsai_141b84d20", tpl_35199::<0x14387c578, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSNodeInstance::ClassTypeInfoImplementation::bonsai::EQSNodeInstance::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141b84f00, "bonsai_141b84f00", tpl_35199::<0x14387c578, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSNodeInstance::ClassTypeInfoImplementation::bonsai::EQSNodeInstance::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141b84ff0, "bonsai_141b84ff0", tpl_35199::<0x14387c578, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSNodeInstance::ClassTypeInfoImplementation::bonsai::EQSNodeInstance::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141b85070, "bonsai_141b85070", tpl_35199::<0x14387c578, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::EQSNodeInstance::ClassTypeInfoImplementation::bonsai::EQSNodeInstance::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141b85180, "bonsai_141b85180", fn_141b85180 as *const ()), // bonsai::EQSNodeInstance::ClassTypeInfoImplementation::bonsai::EQSNodeInstance::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141b852f0, "bonsai_141b852f0", fn_141b852f0 as *const ()), // sub_141b852f0
    Replacement::draft(0x141b855c0, "bonsai_141b855c0", fn_141b855c0 as *const ()), // sub_141b855c0
    Replacement::draft(0x141b85650, "bonsai_141b85650", fn_141b85650 as *const ()), // sub_141b85650
    Replacement::draft(0x141b856d0, "bonsai_141b856d0", fn_141b856d0 as *const ()), // sub_141b856d0
    Replacement::draft(0x141b85700, "bonsai_141b85700", tpl_38042::<0x1432283f0, 0x8, 0x20, 0x142442de0, 0x10> as *const ()), // sub_141b85700
    Replacement::draft_throws(0x141b85760, "bonsai_141b85760", fn_141b85760 as *const (), Throws::Discard), // bonsai::EQSNodeInstance::bonsai::EQSNodeInstance::vfunc_2
    Replacement::draft(0x141b85b00, "bonsai_141b85b00", fn_141b85b00 as *const ()), // sub_141b85b00
    Replacement::draft(0x141b85b80, "bonsai_141b85b80", fn_141b85b80 as *const ()), // sub_141b85b80
    Replacement::draft_throws(0x141b85c60, "bonsai_141b85c60", fn_141b85c60 as *const (), Throws::Caught), // sub_141b85c60
    Replacement::draft_throws(0x141b86740, "bonsai_141b86740", fn_141b86740 as *const (), Throws::Caught), // sub_141b86740
    Replacement::draft_throws(0x141b86810, "bonsai_141b86810", tpl_838::<0x140667350> as *const (), Throws::Discard), // bonsai::EQSLineOfSightTaskInstance::bonsai::EQSLineOfSightTaskInstance::vfunc_10
    Replacement::draft(0x141b86ee0, "bonsai_141b86ee0", fn_141b86ee0 as *const ()), // bonsai::EQSKeepDistanceFromReservedPositionsTaskInstance::bonsai::EQSKeepDistanceFromReservedPositionsTaskInstance::vfun
    Replacement::draft_throws(0x141b87760, "bonsai_141b87760", fn_141b87760 as *const (), Throws::Discard), // sub_141b87760
    Replacement::draft_throws(0x141b87790, "bonsai_141b87790", fn_141b87790 as *const (), Throws::Discard), // bonsai::EQSInvalidateCoverTaskInstance::bonsai::EQSInvalidateCoverTaskInstance::vfunc_4
    Replacement::draft(0x141b880f0, "bonsai_141b880f0", fn_141b880f0 as *const ()), // sub_141b880f0
    Replacement::draft(0x141b881e0, "bonsai_141b881e0", fn_141b881e0 as *const ()), // sub_141b881e0
    Replacement::draft(0x141b88250, "bonsai_141b88250", fn_141b88250 as *const ()), // sub_141b88250
    Replacement::draft_throws(0x141b882d0, "bonsai_141b882d0", fn_141b882d0 as *const (), Throws::Caught), // sub_141b882d0
    Replacement::draft_throws(0x141b88340, "bonsai_141b88340", fn_141b88340 as *const (), Throws::Caught), // bonsai::EQSHeightTaskInstance::bonsai::EQSHeightTaskInstance::vfunc_4
    Replacement::draft(0x141b88c60, "bonsai_141b88c60", fn_141b88c60 as *const ()), // bonsai::EQSHazardFieldTaskInstance::bonsai::EQSHazardFieldTaskInstance::vfunc_4
    Replacement::draft(0x141b89b90, "bonsai_141b89b90", tpl_38042::<0x1432283f0, 0x8, 0x20, 0x142442de0, 0x10> as *const ()), // sub_141b89b90
    Replacement::draft(0x141b89c00, "bonsai_141b89c00", fn_141b89c00 as *const ()), // bonsai::EQSGeneratorBaseNodeInstance::bonsai::EQSGeneratorBaseNodeInstance::vfunc_25
    Replacement::draft_throws(0x141b89c60, "bonsai_141b89c60", fn_141b89c60 as *const (), Throws::Caught), // sub_141b89c60
    Replacement::draft_throws(0x141b89cd0, "bonsai_141b89cd0", fn_141b89cd0 as *const (), Throws::Caught), // bonsai::EQSEuclideanDistanceTaskInstance::bonsai::EQSEuclideanDistanceTaskInstance::vfunc_4
    Replacement::draft_throws(0x141b8a780, "bonsai_141b8a780", fn_141b8a780 as *const (), Throws::Caught), // sub_141b8a780
    Replacement::draft_throws(0x141b8a7f0, "bonsai_141b8a7f0", fn_141b8a7f0 as *const (), Throws::Caught), // bonsai::EQSElevationTaskInstance::bonsai::EQSElevationTaskInstance::vfunc_4
    Replacement::draft_throws(0x141b8b0f0, "bonsai_141b8b0f0", fn_141b8b0f0 as *const (), Throws::Discard), // sub_141b8b0f0
    Replacement::draft_throws(0x141b8b130, "bonsai_141b8b130", fn_141b8b130 as *const (), Throws::Discard), // sub_141b8b130
    Replacement::draft_throws(0x141b8b170, "bonsai_141b8b170", tpl_38040::<0x30, 0x8, 0x28, 0x140436250> as *const (), Throws::Discard), // sub_141b8b170
    Replacement::draft_throws(0x141b8b1a0, "bonsai_141b8b1a0", fn_141b8b1a0 as *const (), Throws::Caught), // sub_141b8b1a0
    Replacement::draft_throws(0x141b8b3b0, "bonsai_141b8b3b0", fn_141b8b3b0 as *const (), Throws::Caught), // sub_141b8b3b0
    Replacement::draft_throws(0x141b8b760, "bonsai_141b8b760", fn_141b8b760 as *const (), Throws::Discard), // bonsai::EQSDynamicGridGeneratorNodeInstance::bonsai::EQSDynamicGridGeneratorNodeInstance::vfunc_3
    Replacement::draft_throws(0x141b8b930, "bonsai_141b8b930", tpl_838::<0x1406696d0> as *const (), Throws::Discard), // bonsai::EQSDynamicGridGeneratorNodeInstance::bonsai::EQSDynamicGridGeneratorNodeInstance::vfunc_7
    Replacement::draft_throws(0x141b8c270, "bonsai_141b8c270", fn_141b8c270 as *const (), Throws::Discard), // sub_141b8c270
    Replacement::draft_throws(0x141b8c2a0, "bonsai_141b8c2a0", fn_141b8c2a0 as *const (), Throws::Discard), // sub_141b8c2a0
    Replacement::draft_throws(0x141b8c2d0, "bonsai_141b8c2d0", fn_141b8c2d0 as *const (), Throws::Discard), // bonsai::EQSDotCompareTaskInstance::bonsai::EQSDotCompareTaskInstance::vfunc_4
    Replacement::draft_throws(0x141b8cb70, "bonsai_141b8cb70", fn_141b8cb70 as *const (), Throws::Discard), // sub_141b8cb70
    Replacement::draft(0x141b8cba0, "bonsai_141b8cba0", fn_141b8cba0 as *const ()), // sub_141b8cba0
    Replacement::draft(0x141b8ce50, "bonsai_141b8ce50", fn_141b8ce50 as *const ()), // sub_141b8ce50
    Replacement::draft_throws(0x141b8d1a0, "bonsai_141b8d1a0", fn_141b8d1a0 as *const (), Throws::Discard), // bonsai::EQSDiscGeneratorNodeInstance::bonsai::EQSDiscGeneratorNodeInstance::vfunc_3
    Replacement::draft(0x141b8d270, "bonsai_141b8d270", tpl_38042::<0x1432283f0, 0x8, 0x20, 0x142442de0, 0x10> as *const ()), // sub_141b8d270
    Replacement::draft_throws(0x141b8dc50, "bonsai_141b8dc50", fn_141b8dc50 as *const (), Throws::Discard), // sub_141b8dc50
    Replacement::draft_throws(0x141b8dc80, "bonsai_141b8dc80", fn_141b8dc80 as *const (), Throws::Discard), // bonsai::EQSCoverScoringTaskInstance::bonsai::EQSCoverScoringTaskInstance::vfunc_4
    Replacement::draft_throws(0x141b8e620, "bonsai_141b8e620", fn_141b8e620 as *const (), Throws::Discard), // sub_141b8e620
    Replacement::draft_throws(0x141b8e650, "bonsai_141b8e650", fn_141b8e650 as *const (), Throws::Discard), // sub_141b8e650
    Replacement::draft(0x141b8e680, "bonsai_141b8e680", fn_141b8e680 as *const ()), // sub_141b8e680
    Replacement::draft(0x141b8e750, "bonsai_141b8e750", tpl_38103::<0x18, 0x20, 0x28, 0x30, 0x38, 0x8, 0x10> as *const ()), // sub_141b8e750
    Replacement::draft(0x141b8e7a0, "bonsai_141b8e7a0", fn_141b8e7a0 as *const ()), // sub_141b8e7a0
    Replacement::draft_throws(0x141b8ecb0, "bonsai_141b8ecb0", fn_141b8ecb0 as *const (), Throws::Discard), // bonsai::EQSCoverPositionGeneratorInstance::bonsai::EQSCoverPositionGeneratorInstance::vfunc_3
    Replacement::draft_throws(0x141b8f790, "bonsai_141b8f790", fn_141b8f790 as *const (), Throws::Discard), // sub_141b8f790
    Replacement::draft_throws(0x141b8f7e0, "bonsai_141b8f7e0", fn_141b8f7e0 as *const (), Throws::Discard), // sub_141b8f7e0
    Replacement::draft_throws(0x141b8f8d0, "bonsai_141b8f8d0", fn_141b8f8d0 as *const (), Throws::Discard), // bonsai::EQSAngleCompareTaskInstance::bonsai::EQSAngleCompareTaskInstance::vfunc_4
    Replacement::draft_throws(0x141b8fd50, "bonsai_141b8fd50", tpl_838::<0x14066b040> as *const (), Throws::Discard), // bonsai::EQSAngleCompareTaskInstance::bonsai::EQSAngleCompareTaskInstance::vfunc_19
    Replacement::draft_throws(0x141b90390, "bonsai_141b90390", fn_141b90390 as *const (), Throws::Discard), // bonsai::EQSAllyInLoSTaskInstance::bonsai::EQSAllyInLoSTaskInstance::vfunc_4
    Replacement::draft(0x141b910c0, "unknown_141b910c0", tpl_34318 as *const ()), // sub_141b910c0
    Replacement::draft(0x141b910e0, "unknown_141b910e0", tpl_34319::<0x143ca761c, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141b910e0
    Replacement::draft(0x141b91140, "unknown_141b91140", tpl_361 as *const ()), // sub_141b91140
    Replacement::draft(0x141b91150, "unknown_141b91150", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ca761c, 0x158, 0x14020e8f0> as *const ()), // sub_141b91150
    Replacement::draft(0x141b91240, "unknown_141b91240", tpl_496 as *const ()), // sub_141b91240
    Replacement::draft(0x141b91250, "unknown_141b91250", fn_141b91250 as *const ()), // sub_141b91250
    Replacement::draft(0x141b914f0, "unknown_141b914f0", tpl_496 as *const ()), // sub_141b914f0
    Replacement::draft(0x141b91500, "unknown_141b91500", tpl_361 as *const ()), // sub_141b91500
    Replacement::draft_throws(0x141b91510, "std_141b91510", fn_141b91510 as *const (), Throws::Discard), // sub_141b91510
    Replacement::draft(0x141b91990, "unknown_141b91990", fn_141b91990 as *const ()), // sub_141b91990
    Replacement::draft_throws(0x141b91ac0, "bonsai_141b91ac0", fn_141b91ac0 as *const (), Throws::Discard), // sub_141b91ac0
    Replacement::draft_throws(0x141b91d80, "bonsai_141b91d80", fn_141b91d80 as *const (), Throws::Discard), // sub_141b91d80
    Replacement::draft_throws(0x141b91e70, "bonsai_141b91e70", fn_141b91e70 as *const (), Throws::Discard), // sub_141b91e70
    Replacement::draft_throws(0x141b924e0, "bonsai_141b924e0", fn_141b924e0 as *const (), Throws::Discard), // sub_141b924e0
    Replacement::draft_throws(0x141b92500, "bonsai_141b92500", fn_141b92500 as *const (), Throws::Caught), // sub_141b92500
    Replacement::draft(0x141b92590, "bonsai_141b92590", fn_141b92590 as *const ()), // sub_141b92590
    Replacement::draft_throws(0x141b927b0, "bonsai_141b927b0", fn_141b927b0 as *const (), Throws::Discard), // bonsai::DistanceConditionDecoratorInstance::bonsai::DistanceConditionDecoratorInstance::vfunc_34
    Replacement::draft_throws(0x141b928d0, "bonsai_141b928d0", tpl_38004::<0x141b72aa0, 0x110, 0x18, 0x8> as *const (), Throws::Discard), // bonsai::DistanceConditionDecoratorInstance::bonsai::DistanceConditionDecoratorInstance::vfunc_1
    Replacement::draft(0x141b937f0, "bonsai_141b937f0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::DisabledDecoratorInstance::ClassTypeInfoImplementation::bonsai::DisabledDecoratorInstance::ClassTypeInfoImplemen
    Replacement::draft(0x141b939d0, "bonsai_141b939d0", fn_141b939d0 as *const ()), // bonsai::DisableAvoidanceServiceInstance::bonsai::DisableAvoidanceServiceInstance::vfunc_2
    Replacement::draft(0x141b93a00, "bonsai_141b93a00", fn_141b93a00 as *const ()), // bonsai::DisableAvoidanceServiceInstance::bonsai::DisableAvoidanceServiceInstance::vfunc_1
    Replacement::draft(0x141b94160, "bonsai_141b94160", fn_141b94160 as *const ()), // sub_141b94160
    Replacement::draft(0x141b94190, "bonsai_141b94190", fn_141b94190 as *const ()), // sub_141b94190
    Replacement::draft(0x141b941b0, "bonsai_141b941b0", tpl_38114::<0xb0, 0xc8, 0x18, 0x20, 0x30, 0x8, 0x10, 0x28, 0x1a> as *const ()), // sub_141b941b0
    Replacement::draft(0x141b94240, "bonsai_141b94240", tpl_38115::<0x141b941b0, 0x18, 0x8> as *const ()), // bonsai::DecoratorTicketConditionInstance::bonsai::DecoratorTicketConditionInstance::vfunc_1
    Replacement::draft_throws(0x141b94270, "bonsai_141b94270", fn_141b94270 as *const (), Throws::Discard), // bonsai::DecoratorTicketConditionInstance::bonsai::DecoratorTicketConditionInstance::vfunc_12
    Replacement::draft_throws(0x141b94340, "bonsai_141b94340", fn_141b94340 as *const (), Throws::Discard), // bonsai::DecoratorTicketConditionInstance::bonsai::DecoratorTicketConditionInstance::vfunc_9
    Replacement::draft_throws(0x141b943b0, "bonsai_141b943b0", fn_141b943b0 as *const (), Throws::Discard), // bonsai::DecoratorTicketConditionInstance::bonsai::DecoratorTicketConditionInstance::vfunc_5
    Replacement::draft_throws(0x141b94460, "bonsai_141b94460", fn_141b94460 as *const (), Throws::Discard), // bonsai::DecoratorTicketConditionInstance::bonsai::DecoratorTicketConditionInstance::vfunc_4
    Replacement::draft(0x141b94a80, "bonsai_141b94a80", fn_141b94a80 as *const ()), // bonsai::DecoratorTicketConditionInstance::ClassTypeInfoImplementation::bonsai::DecoratorTicketConditionInstance::ClassTy
    Replacement::draft(0x141b94b00, "bonsai_141b94b00", fn_141b94b00 as *const ()), // bonsai::DecoratorTicketConditionInstance::ClassTypeInfoImplementation::bonsai::DecoratorTicketConditionInstance::ClassTy
    Replacement::draft_throws(0x141b94c80, "bonsai_141b94c80", fn_141b94c80 as *const (), Throws::Discard), // bonsai::DecoratorRestrictedConditionInstance::bonsai::DecoratorRestrictedConditionInstance::vfunc_34
    Replacement::draft(0x141b94cc0, "bonsai_141b94cc0", tpl_38123::<0x110, 0x18, 0x8> as *const ()), // bonsai::DecoratorRestrictedConditionInstance::bonsai::DecoratorRestrictedConditionInstance::vfunc_1
    Replacement::draft(0x141b95650, "bonsai_141b95650", fn_141b95650 as *const ()), // sub_141b95650
    Replacement::draft(0x141b956d0, "bonsai_141b956d0", tpl_38115::<0x141b95650, 0x18, 0x8> as *const ()), // bonsai::DecoratorLostTicketConditionInstance::bonsai::DecoratorLostTicketConditionInstance::vfunc_1
    Replacement::draft(0x141b95700, "bonsai_141b95700", fn_141b95700 as *const ()), // bonsai::DecoratorLostTicketConditionInstance::bonsai::DecoratorLostTicketConditionInstance::vfunc_12
    Replacement::draft(0x141b95ca0, "bonsai_141b95ca0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::DecoratorLostTicketConditionInstance::ClassTypeInfoImplementation::bonsai::DecoratorLostTicketConditionInstance:
    Replacement::draft_throws(0x141b95e80, "bonsai_141b95e80", fn_141b95e80 as *const (), Throws::Discard), // bonsai::DecoratorIntConditionInstance::bonsai::DecoratorIntConditionInstance::vfunc_34
    Replacement::draft_throws(0x141b95fd0, "bonsai_141b95fd0", tpl_38004::<0x141b72aa0, 0x110, 0x18, 0x8> as *const (), Throws::Discard), // bonsai::DecoratorIntConditionInstance::bonsai::DecoratorIntConditionInstance::vfunc_1
    Replacement::draft_throws(0x141b96980, "bonsai_141b96980", fn_141b96980 as *const (), Throws::Discard), // bonsai::DecoratorFloatConditionInstance::bonsai::DecoratorFloatConditionInstance::vfunc_34
    Replacement::draft_throws(0x141b96aa0, "bonsai_141b96aa0", tpl_38004::<0x141b72aa0, 0x110, 0x18, 0x8> as *const (), Throws::Discard), // bonsai::DecoratorFloatConditionInstance::bonsai::DecoratorFloatConditionInstance::vfunc_1
    Replacement::draft(0x141b97460, "bonsai_141b97460", tpl_38123::<0x110, 0x18, 0x8> as *const ()), // bonsai::DecoratorCanChargeConditionInstance::bonsai::DecoratorCanChargeConditionInstance::vfunc_1
    Replacement::draft_throws(0x141b97df0, "bonsai_141b97df0", fn_141b97df0 as *const (), Throws::Discard), // bonsai::DecoratorBoolConditionInstance::bonsai::DecoratorBoolConditionInstance::vfunc_34
    Replacement::draft_throws(0x141b97e90, "bonsai_141b97e90", tpl_38004::<0x141b72aa0, 0x110, 0x18, 0x8> as *const (), Throws::Discard), // bonsai::DecoratorBoolConditionInstance::bonsai::DecoratorBoolConditionInstance::vfunc_1
    Replacement::draft_throws(0x141b98840, "bonsai_141b98840", fn_141b98840 as *const (), Throws::Discard), // bonsai::DecoratorBlockTicketStealingInstance::bonsai::DecoratorBlockTicketStealingInstance::vfunc_5
    Replacement::draft_throws(0x141b988e0, "bonsai_141b988e0", fn_141b988e0 as *const (), Throws::Discard), // bonsai::DecoratorBlockTicketStealingInstance::bonsai::DecoratorBlockTicketStealingInstance::vfunc_4
    Replacement::draft(0x141b98ee0, "bonsai_141b98ee0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::DecoratorBlockTicketStealingInstance::ClassTypeInfoImplementation::bonsai::DecoratorBlockTicketStealingInstance:
    Replacement::draft(0x141b99630, "bonsai_141b99630", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::DecoratorAlwaysSucceedInstance::ClassTypeInfoImplementation::bonsai::DecoratorAlwaysSucceedInstance::ClassTypeIn
    Replacement::draft(0x141b99810, "bonsai_141b99810", fn_141b99810 as *const ()), // bonsai::DamageTrackingDecoratorInstance::bonsai::DamageTrackingDecoratorInstance::vfunc_3
    Replacement::draft(0x141b99830, "bonsai_141b99830", fn_141b99830 as *const ()), // bonsai::DamageTrackingDecoratorInstance::bonsai::DamageTrackingDecoratorInstance::vfunc_12
    Replacement::draft(0x141b99880, "bonsai_141b99880", fn_141b99880 as *const ()), // bonsai::DamageTrackingDecoratorInstance::bonsai::DamageTrackingDecoratorInstance::vfunc_34
    Replacement::draft_throws(0x141b999c0, "bonsai_141b999c0", tpl_38004::<0x141b72aa0, 0x110, 0x18, 0x8> as *const (), Throws::Discard), // bonsai::DamageTrackingDecoratorInstance::bonsai::DamageTrackingDecoratorInstance::vfunc_1
    Replacement::draft(0x141b9a380, "bonsai_141b9a380", fn_141b9a380 as *const ()), // sub_141b9a380
    Replacement::draft(0x141b9a460, "bonsai_141b9a460", fn_141b9a460 as *const ()), // sub_141b9a460
    Replacement::draft_throws(0x141b9a490, "bonsai_141b9a490", fn_141b9a490 as *const (), Throws::Discard), // bonsai::CoverServiceInstance::bonsai::CoverServiceInstance::vfunc_6
    Replacement::draft(0x141b9a7c0, "bonsai_141b9a7c0", fn_141b9a7c0 as *const ()), // bonsai::CoverServiceInstance::bonsai::CoverServiceInstance::vfunc_1
    Replacement::draft(0x141b9ad60, "bonsai_141b9ad60", fn_141b9ad60 as *const ()), // bonsai::CoverServiceInstance::ClassTypeInfoImplementation::bonsai::CoverServiceInstance::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141b9af40, "bonsai_141b9af40", tpl_38136::<0xb8, 0xb0, 0xac> as *const ()), // sub_141b9af40
    Replacement::draft(0x141b9afb0, "bonsai_141b9afb0", tpl_37771::<0x98, 0xa0> as *const ()), // bonsai::CoverProgram::bonsai::CoverProgram::vfunc_16
    Replacement::draft(0x141b9b010, "bonsai_141b9b010", fn_141b9b010 as *const ()), // bonsai::CoverProgram::bonsai::CoverProgram::vfunc_15
    Replacement::draft(0x141b9b070, "bonsai_141b9b070", fn_141b9b070 as *const ()), // bonsai::CoverProgram::bonsai::CoverProgram::vfunc_6
    Replacement::draft(0x141b9b130, "bonsai_141b9b130", tpl_37773::<0x98, 0xa0> as *const ()), // bonsai::CoverProgram::bonsai::CoverProgram::vfunc_5
    Replacement::draft(0x141b9b190, "bonsai_141b9b190", fn_141b9b190 as *const ()), // bonsai::CoverProgram::bonsai::CoverProgram::vfunc_3
    Replacement::draft(0x141b9b1b0, "bonsai_141b9b1b0", fn_141b9b1b0 as *const ()), // bonsai::CoverProgram::bonsai::CoverProgram::vfunc_1
    Replacement::draft(0x141b9b4d0, "bonsai_141b9b4d0", fn_141b9b4d0 as *const ()), // sub_141b9b4d0
    Replacement::draft(0x141b9b680, "bonsai_141b9b680", fn_141b9b680 as *const ()), // sub_141b9b680
    Replacement::draft(0x141b9b780, "bonsai_141b9b780", fn_141b9b780 as *const ()), // sub_141b9b780
    Replacement::draft(0x141b9b8d0, "bonsai_141b9b8d0", fn_141b9b8d0 as *const ()), // sub_141b9b8d0
    Replacement::draft(0x141b9ba50, "bonsai_141b9ba50", fn_141b9ba50 as *const ()), // sub_141b9ba50
    Replacement::draft(0x141b9bc00, "bonsai_141b9bc00", fn_141b9bc00 as *const ()), // sub_141b9bc00
    Replacement::draft(0x141b9bcf0, "bonsai_141b9bcf0", fn_141b9bcf0 as *const ()), // sub_141b9bcf0
    Replacement::draft(0x141b9c030, "bonsai_141b9c030", fn_141b9c030 as *const ()), // sub_141b9c030
    Replacement::draft(0x141b9c3b0, "bonsai_141b9c3b0", fn_141b9c3b0 as *const ()), // sub_141b9c3b0
    Replacement::draft(0x141b9c580, "bonsai_141b9c580", tpl_37771::<0x98, 0xa0> as *const ()), // bonsai::CoverEnterProgram::bonsai::CoverEnterProgram::vfunc_16
    Replacement::draft(0x141b9c5e0, "bonsai_141b9c5e0", tpl_37771::<0x98, 0xa0> as *const ()), // bonsai::CoverEnterProgram::bonsai::CoverEnterProgram::vfunc_15
    Replacement::draft(0x141b9c640, "bonsai_141b9c640", fn_141b9c640 as *const ()), // sub_141b9c640
    Replacement::draft(0x141b9c880, "bonsai_141b9c880", fn_141b9c880 as *const ()), // bonsai::CoverEnterProgram::bonsai::CoverEnterProgram::vfunc_6
    Replacement::draft(0x141b9c930, "bonsai_141b9c930", tpl_37773::<0x98, 0xa0> as *const ()), // bonsai::CoverEnterProgram::bonsai::CoverEnterProgram::vfunc_5
    Replacement::draft(0x141b9c990, "bonsai_141b9c990", tpl_37774::<0x98, 0xa0, 0x141b9c640> as *const ()), // bonsai::CoverEnterProgram::bonsai::CoverEnterProgram::vfunc_1
    Replacement::draft(0x141b9ca40, "bonsai_141b9ca40", fn_141b9ca40 as *const ()), // sub_141b9ca40
    Replacement::draft_throws(0x141b9cae0, "bonsai_141b9cae0", fn_141b9cae0 as *const (), Throws::Discard), // bonsai::CountdownServiceInstance::bonsai::CountdownServiceInstance::vfunc_6
    Replacement::draft(0x141b9d6d0, "bonsai_141b9d6d0", tpl_35199::<0x14387c750, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::Countdown::ClassTypeInfoImplementation::bonsai::Countdown::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141b9d730, "bonsai_141b9d730", tpl_35199::<0x14387c750, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::Countdown::ClassTypeInfoImplementation::bonsai::Countdown::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft_throws(0x141b9d900, "bonsai_141b9d900", tpl_37815::<0x18, 0x20, 0x141b9d940> as *const (), Throws::Discard), // bonsai::CopyPropertyDecoratorInstance::bonsai::CopyPropertyDecoratorInstance::vfunc_9
    Replacement::draft_throws(0x141b9d940, "bonsai_141b9d940", fn_141b9d940 as *const (), Throws::Discard), // sub_141b9d940
    Replacement::draft_throws(0x141b9d9b0, "bonsai_141b9d9b0", tpl_37920::<0x10, 0x28, 0x20, 0x141b9d940> as *const (), Throws::Discard), // bonsai::CopyPropertyDecoratorInstance::bonsai::CopyPropertyDecoratorInstance::vfunc_4
    Replacement::draft(0x141b9df90, "bonsai_141b9df90", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::CopyPropertyDecoratorInstance::ClassTypeInfoImplementation::bonsai::CopyPropertyDecoratorInstance::ClassTypeInfo
    Replacement::draft_throws(0x141b9e190, "bonsai_141b9e190", fn_141b9e190 as *const (), Throws::Discard), // sub_141b9e190
    Replacement::draft_throws(0x141b9e210, "bonsai_141b9e210", fn_141b9e210 as *const (), Throws::Discard), // sub_141b9e210
    Replacement::draft(0x141b9e290, "bonsai_141b9e290", fn_141b9e290 as *const ()), // sub_141b9e290
    Replacement::draft(0x141b9e2c0, "bonsai_141b9e2c0", fn_141b9e2c0 as *const ()), // sub_141b9e2c0
    Replacement::draft(0x141b9e2e0, "bonsai_141b9e2e0", fn_141b9e2e0 as *const ()), // sub_141b9e2e0
    Replacement::draft(0x141b9e2f0, "bonsai_141b9e2f0", fn_141b9e2f0 as *const ()), // sub_141b9e2f0
    Replacement::draft(0x141b9e390, "bonsai_141b9e390", fn_141b9e390 as *const ()), // bonsai::CallScriptDecorator::bonsai::CallScriptDecorator::vfunc_9
    Replacement::draft(0x141b9e3d0, "bonsai_141b9e3d0", fn_141b9e3d0 as *const ()), // bonsai::CallScriptDecorator::bonsai::CallScriptDecorator::vfunc_4
    Replacement::draft(0x141b9e9b0, "bonsai_141b9e9b0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::CallScriptDecorator::ClassTypeInfoImplementation::bonsai::CallScriptDecorator::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141b9eb90, "bonsai_141b9eb90", tpl_37815::<0x18, 0x20, 0x141b9ebd0> as *const (), Throws::Discard), // bonsai::CalculateDirectionDecoratorInstance::bonsai::CalculateDirectionDecoratorInstance::vfunc_9
    Replacement::draft_throws(0x141b9ebd0, "bonsai_141b9ebd0", fn_141b9ebd0 as *const (), Throws::Discard), // sub_141b9ebd0
    Replacement::draft_throws(0x141b9ec90, "bonsai_141b9ec90", tpl_37920::<0x10, 0x28, 0x20, 0x141b9ebd0> as *const (), Throws::Discard), // bonsai::CalculateDirectionDecoratorInstance::bonsai::CalculateDirectionDecoratorInstance::vfunc_4
    Replacement::draft(0x141b9f270, "bonsai_141b9f270", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::CalculateDirectionDecoratorInstance::ClassTypeInfoImplementation::bonsai::CalculateDirectionDecoratorInstance::C
    Replacement::draft_throws(0x141b9f490, "bonsai_141b9f490", tpl_35009::<0x14387cb20, 0x1406757f0, 0x1402116e0> as *const (), Throws::Discard), // sub_141b9f490
    Replacement::draft(0x141b9f4d0, "bonsai_141b9f4d0", fn_141b9f4d0 as *const ()), // sub_141b9f4d0
    Replacement::draft(0x141b9f6c0, "bonsai_141b9f6c0", fn_141b9f6c0 as *const ()), // sub_141b9f6c0
    Replacement::draft(0x141b9f780, "bonsai_141b9f780", fn_141b9f780 as *const ()), // sub_141b9f780
    Replacement::draft(0x141b9f7b0, "bonsai_141b9f7b0", fn_141b9f7b0 as *const ()), // sub_141b9f7b0
    Replacement::draft(0x141b9f820, "bonsai_141b9f820", tpl_38157::<0x14322fa50, 0x18, 0x28, 0x30, 0x84, 0x80, 0x8c, 0x1401de830, 0x10> as *const ()), // sub_141b9f820
    Replacement::draft(0x141b9f880, "bonsai_141b9f880", fn_141b9f880 as *const ()), // sub_141b9f880
    Replacement::draft(0x141b9f910, "bonsai_141b9f910", tpl_38159::<0x143428120, 0x143428460, 0x140275010> as *const ()), // sub_141b9f910
    Replacement::draft(0x141b9fa70, "bonsai_141b9fa70", fn_141b9fa70 as *const ()), // sub_141b9fa70
    Replacement::draft(0x141b9fcd0, "bonsai_141b9fcd0", fn_141b9fcd0 as *const ()), // sub_141b9fcd0
    Replacement::draft(0x141b9fd00, "bonsai_141b9fd00", fn_141b9fd00 as *const ()), // sub_141b9fd00
    Replacement::draft(0x141b9fd50, "bonsai_141b9fd50", fn_141b9fd50 as *const ()), // sub_141b9fd50
    Replacement::draft(0x141b9fdf0, "bonsai_141b9fdf0", tpl_34605::<0x143231058> as *const ()), // sub_141b9fdf0
    Replacement::draft(0x141b9ff30, "bonsai_141b9ff30", fn_141b9ff30 as *const ()), // sub_141b9ff30
    Replacement::draft(0x141ba0060, "bonsai_141ba0060", fn_141ba0060 as *const ()), // sub_141ba0060
    Replacement::draft(0x141ba00f0, "bonsai_141ba00f0", fn_141ba00f0 as *const ()), // sub_141ba00f0
    Replacement::draft(0x141ba03e0, "bonsai_141ba03e0", fn_141ba03e0 as *const ()), // sub_141ba03e0
    Replacement::draft(0x141ba0560, "bonsai_141ba0560", fn_141ba0560 as *const ()), // sub_141ba0560
    Replacement::draft(0x141ba0930, "bonsai_141ba0930", fn_141ba0930 as *const ()), // sub_141ba0930
    Replacement::draft(0x141ba0ba0, "bonsai_141ba0ba0", fn_141ba0ba0 as *const ()), // sub_141ba0ba0
    Replacement::draft(0x141ba0c30, "bonsai_141ba0c30", tpl_38168::<0x10, 0x8, 0x58, 0x60, 0x143c9fca8, 0x158, 0x14020e8f0> as *const ()), // sub_141ba0c30
    Replacement::draft(0x141ba0d30, "bonsai_141ba0d30", fn_141ba0d30 as *const ()), // sub_141ba0d30
    Replacement::draft(0x141ba0ea0, "bonsai_141ba0ea0", fn_141ba0ea0 as *const ()), // sub_141ba0ea0
    Replacement::draft(0x141ba1160, "bonsai_141ba1160", fn_141ba1160 as *const ()), // sub_141ba1160
    Replacement::draft(0x141ba11a0, "bonsai_141ba11a0", fn_141ba11a0 as *const ()), // sub_141ba11a0
    Replacement::draft_throws(0x141ba1360, "bonsai_141ba1360", fn_141ba1360 as *const (), Throws::Discard), // sub_141ba1360
    Replacement::draft_throws(0x141ba1460, "bonsai_141ba1460", fn_141ba1460 as *const (), Throws::Discard), // sub_141ba1460
    Replacement::draft(0x141ba1570, "bonsai_141ba1570", fn_141ba1570 as *const ()), // sub_141ba1570
    Replacement::draft(0x141ba1600, "bonsai_141ba1600", fn_141ba1600 as *const ()), // sub_141ba1600
    Replacement::draft(0x141ba1720, "bonsai_141ba1720", fn_141ba1720 as *const ()), // sub_141ba1720
    Replacement::draft_throws(0x141ba17b0, "bonsai_141ba17b0", fn_141ba17b0 as *const (), Throws::Discard), // sub_141ba17b0
    Replacement::draft_throws(0x141ba1820, "bonsai_141ba1820", fn_141ba1820 as *const (), Throws::Discard), // sub_141ba1820
    Replacement::draft(0x141ba1e90, "bonsai_141ba1e90", fn_141ba1e90 as *const ()), // sub_141ba1e90
    Replacement::draft(0x141ba1f80, "bonsai_141ba1f80", fn_141ba1f80 as *const ()), // sub_141ba1f80
    Replacement::draft(0x141ba2080, "bonsai_141ba2080", fn_141ba2080 as *const ()), // sub_141ba2080
    Replacement::draft_throws(0x141ba2140, "bonsai_141ba2140", fn_141ba2140 as *const (), Throws::Discard), // sub_141ba2140
    Replacement::draft(0x141ba21f0, "bonsai_141ba21f0", fn_141ba21f0 as *const ()), // sub_141ba21f0
    Replacement::draft_throws(0x141ba2260, "bonsai_141ba2260", fn_141ba2260 as *const (), Throws::Discard), // sub_141ba2260
    Replacement::draft(0x141ba22c0, "bonsai_141ba22c0", tpl_968::<0x18> as *const ()), // sub_141ba22c0
    Replacement::draft(0x141ba22d0, "bonsai_141ba22d0", fn_141ba22d0 as *const ()), // sub_141ba22d0
    Replacement::draft(0x141ba2480, "bonsai_141ba2480", tpl_35199::<0x14387c7a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementation::bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementati
    Replacement::draft(0x141ba2570, "bonsai_141ba2570", tpl_35199::<0x14387c7a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementation::bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementati
    Replacement::draft(0x141ba2750, "bonsai_141ba2750", tpl_35199::<0x14387c7a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementation::bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementati
    Replacement::draft(0x141ba2840, "bonsai_141ba2840", tpl_35199::<0x14387c7a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementation::bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementati
    Replacement::draft(0x141ba28c0, "bonsai_141ba28c0", tpl_35199::<0x14387c7a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementation::bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementati
    Replacement::draft(0x141ba2970, "bonsai_141ba2970", tpl_38020::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48> as *const ()), // bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementation::bonsai::BonsaiGraphNodeInstance::ClassTypeInfoImplementati
    Replacement::draft(0x141ba2b40, "bonsai_141ba2b40", fn_141ba2b40 as *const ()), // sub_141ba2b40
    Replacement::draft(0x141ba2be0, "bonsai_141ba2be0", tpl_38185::<0x18, 0x10> as *const ()), // sub_141ba2be0
    Replacement::draft(0x141ba2c00, "bonsai_141ba2c00", tpl_38186::<0x18> as *const ()), // sub_141ba2c00
    Replacement::draft(0x141ba2c10, "bonsai_141ba2c10", tpl_38185::<0x30, 0x28> as *const ()), // sub_141ba2c10
    Replacement::draft(0x141ba2c50, "bonsai_141ba2c50", fn_141ba2c50 as *const ()), // sub_141ba2c50
    Replacement::draft(0x141ba2cf0, "bonsai_141ba2cf0", tpl_38186::<0x30> as *const ()), // sub_141ba2cf0
    Replacement::draft(0x141ba2d00, "bonsai_141ba2d00", tpl_38188::<0x28> as *const ()), // sub_141ba2d00
    Replacement::draft(0x141ba30f0, "bonsai_141ba30f0", tpl_38189::<0x28, 0x48> as *const ()), // bonsai::FiniteStateMachineNodeInstance::bonsai::FiniteStateMachineNodeInstance::vfunc_12
    Replacement::draft(0x141ba3170, "bonsai_141ba3170", fn_141ba3170 as *const ()), // bonsai::FiniteStateMachineNodeInstance::bonsai::FiniteStateMachineNodeInstance::vfunc_11
    Replacement::draft(0x141ba31e0, "bonsai_141ba31e0", fn_141ba31e0 as *const ()), // bonsai::FiniteStateMachineNodeInstance::bonsai::FiniteStateMachineNodeInstance::vfunc_10
    Replacement::draft(0x141ba3270, "bonsai_141ba3270", tpl_38191::<0x28, 0x60> as *const ()), // bonsai::BehaviorTreeNodeInstance::bonsai::BehaviorTreeNodeInstance::vfunc_9
    Replacement::draft_throws(0x141ba32f0, "bonsai_141ba32f0", fn_141ba32f0 as *const (), Throws::Discard), // bonsai::BehaviorTreeNodeInstance::bonsai::BehaviorTreeNodeInstance::vfunc_2
    Replacement::draft(0x141ba3390, "bonsai_141ba3390", fn_141ba3390 as *const ()), // bonsai::BehaviorTreeNodeInstance::bonsai::BehaviorTreeNodeInstance::vfunc_1
    Replacement::draft_throws(0x141ba33f0, "bonsai_141ba33f0", fn_141ba33f0 as *const (), Throws::Discard), // sub_141ba33f0
    Replacement::draft(0x141ba3520, "bonsai_141ba3520", fn_141ba3520 as *const ()), // sub_141ba3520
    Replacement::draft(0x141ba44c0, "bonsai_141ba44c0", tpl_37507::<0x14067b260, 0x142442de0> as *const ()), // bonsai::AnimatedMoveToProgramParams::bonsai::AnimatedMoveToProgramParams::vfunc_2
    Replacement::draft(0x141ba4530, "bonsai_141ba4530", tpl_37507::<0x14067afc0, 0x142442de0> as *const ()), // bonsai::AnimatedMovementProgramParams::bonsai::AnimatedMovementProgramParams::vfunc_2
    Replacement::draft(0x141ba45a0, "bonsai_141ba45a0", tpl_37507::<0x14067aa90, 0x142442de0> as *const ()), // bonsai::AimProgramParams::bonsai::AimProgramParams::vfunc_2
    Replacement::draft(0x141ba4610, "bonsai_141ba4610", tpl_37507::<0x14067a830, 0x142442de0> as *const ()), // bonsai::AirMoveProgramParams::bonsai::AirMoveProgramParams::vfunc_2
    Replacement::draft(0x141ba4680, "bonsai_141ba4680", tpl_37507::<0x14067b120, 0x142442de0> as *const ()), // bonsai::CoverEnterProgramParams::bonsai::CoverEnterProgramParams::vfunc_2
    Replacement::draft(0x141ba46f0, "bonsai_141ba46f0", tpl_37507::<0x14067ad00, 0x142442de0> as *const ()), // bonsai::CoverProgramParams::bonsai::CoverProgramParams::vfunc_2
    Replacement::draft(0x141ba4760, "bonsai_141ba4760", tpl_37507::<0x14067a970, 0x142442de0> as *const ()), // bonsai::LevitateProgramParams::bonsai::LevitateProgramParams::vfunc_2
    Replacement::draft(0x141ba47d0, "bonsai_141ba47d0", tpl_37507::<0x14067abd0, 0x142442de0> as *const ()), // bonsai::ShootProgramParams::bonsai::ShootProgramParams::vfunc_2
    Replacement::draft(0x141ba4840, "bonsai_141ba4840", tpl_37507::<0x14067ae80, 0x142442de0> as *const ()), // bonsai::StunnedProgramParams::bonsai::StunnedProgramParams::vfunc_2
    Replacement::draft(0x141ba48b0, "bonsai_141ba48b0", tpl_37507::<0x14067b4e0, 0x142442de0> as *const ()), // bonsai::AbilityProgramParams::bonsai::AbilityProgramParams::vfunc_2
    Replacement::draft(0x141ba4920, "bonsai_141ba4920", tpl_37507::<0x14067a670, 0x142442de0> as *const ()), // bonsai::MoveProgramParams::bonsai::MoveProgramParams::vfunc_2
    Replacement::draft(0x141ba4990, "bonsai_141ba4990", tpl_37507::<0x14067b3c0, 0x142442de0> as *const ()), // bonsai::IdleBodyProgramParams::bonsai::IdleBodyProgramParams::vfunc_2
    Replacement::draft(0x141ba49f0, "bonsai_141ba49f0", fn_141ba49f0 as *const ()), // sub_141ba49f0
    Replacement::draft(0x141ba4a20, "bonsai_141ba4a20", fn_141ba4a20 as *const ()), // sub_141ba4a20
    Replacement::draft(0x141ba4a60, "bonsai_141ba4a60", fn_141ba4a60 as *const ()), // bonsai::BodyProgram::bonsai::BodyProgram::vfunc_4
    Replacement::draft(0x141ba4aa0, "bonsai_141ba4aa0", fn_141ba4aa0 as *const ()), // sub_141ba4aa0
    Replacement::draft(0x141ba4ba0, "bonsai_141ba4ba0", fn_141ba4ba0 as *const ()), // sub_141ba4ba0
    Replacement::draft(0x141ba5050, "bonsai_141ba5050", tpl_34393 as *const ()), // sub_141ba5050
    Replacement::draft(0x141ba5170, "bonsai_141ba5170", fn_141ba5170 as *const ()), // sub_141ba5170
    Replacement::draft(0x141ba52b0, "bonsai_141ba52b0", tpl_34670::<0x1e0, 0x210> as *const ()), // sub_141ba52b0
    Replacement::draft_throws(0x141ba6360, "bonsai_141ba6360", fn_141ba6360 as *const (), Throws::Discard), // sub_141ba6360
    Replacement::draft(0x141ba6610, "bonsai_141ba6610", fn_141ba6610 as *const ()), // sub_141ba6610
    Replacement::draft_throws(0x141ba67c0, "bonsai_141ba67c0", fn_141ba67c0 as *const (), Throws::Discard), // sub_141ba67c0
    Replacement::draft_throws(0x141ba6b00, "bonsai_141ba6b00", fn_141ba6b00 as *const (), Throws::Caught), // sub_141ba6b00
    Replacement::draft(0x141ba7a20, "bonsai_141ba7a20", fn_141ba7a20 as *const ()), // sub_141ba7a20
    Replacement::draft(0x141ba7ec0, "bonsai_141ba7ec0", fn_141ba7ec0 as *const ()), // sub_141ba7ec0
    Replacement::draft(0x141ba8040, "bonsai_141ba8040", fn_141ba8040 as *const ()), // sub_141ba8040
    Replacement::draft(0x141ba80c0, "bonsai_141ba80c0", fn_141ba80c0 as *const ()), // sub_141ba80c0
    Replacement::draft_throws(0x141ba8170, "bonsai_141ba8170", fn_141ba8170 as *const (), Throws::Discard), // sub_141ba8170
    Replacement::draft(0x141ba84b0, "bonsai_141ba84b0", fn_141ba84b0 as *const ()), // sub_141ba84b0
    Replacement::draft(0x141ba8520, "bonsai_141ba8520", fn_141ba8520 as *const ()), // sub_141ba8520
    Replacement::draft_throws(0x141ba8df0, "bonsai_141ba8df0", fn_141ba8df0 as *const (), Throws::Caught), // sub_141ba8df0
    Replacement::draft(0x141ba97d0, "bonsai_141ba97d0", fn_141ba97d0 as *const ()), // sub_141ba97d0
    Replacement::draft(0x141ba9890, "bonsai_141ba9890", fn_141ba9890 as *const ()), // sub_141ba9890
    Replacement::draft(0x141ba98d0, "bonsai_141ba98d0", tpl_30667::<0x44> as *const ()), // sub_141ba98d0
    Replacement::draft(0x141ba9900, "bonsai_141ba9900", fn_141ba9900 as *const ()), // sub_141ba9900
    Replacement::draft(0x141ba9920, "bonsai_141ba9920", fn_141ba9920 as *const ()), // sub_141ba9920
    Replacement::draft(0x141ba9b20, "bonsai_141ba9b20", fn_141ba9b20 as *const ()), // sub_141ba9b20
    Replacement::draft_throws(0x141ba9bb0, "bonsai_141ba9bb0", fn_141ba9bb0 as *const (), Throws::Discard), // sub_141ba9bb0
    Replacement::draft(0x141ba9ce0, "bonsai_141ba9ce0", fn_141ba9ce0 as *const ()), // sub_141ba9ce0
    Replacement::draft(0x141baa470, "bonsai_141baa470", fn_141baa470 as *const ()), // sub_141baa470
    Replacement::draft(0x141baa540, "bonsai_141baa540", fn_141baa540 as *const ()), // sub_141baa540
    Replacement::draft(0x141baa640, "bonsai_141baa640", fn_141baa640 as *const ()), // sub_141baa640
    Replacement::draft(0x141baa780, "bonsai_141baa780", fn_141baa780 as *const ()), // sub_141baa780
    Replacement::draft(0x141baa7a0, "bonsai_141baa7a0", fn_141baa7a0 as *const ()), // sub_141baa7a0
    Replacement::draft(0x141baadd0, "bonsai_141baadd0", fn_141baadd0 as *const ()), // sub_141baadd0
    Replacement::draft(0x141bab000, "bonsai_141bab000", fn_141bab000 as *const ()), // sub_141bab000
    Replacement::draft(0x141bab090, "bonsai_141bab090", fn_141bab090 as *const ()), // sub_141bab090
    Replacement::draft(0x141bab0c0, "bonsai_141bab0c0", fn_141bab0c0 as *const ()), // sub_141bab0c0
    Replacement::draft(0x141bab150, "bonsai_141bab150", fn_141bab150 as *const ()), // sub_141bab150
    Replacement::draft(0x141bab1e0, "bonsai_141bab1e0", fn_141bab1e0 as *const ()), // sub_141bab1e0
    Replacement::draft(0x141bab310, "bonsai_141bab310", tpl_38218::<0xc0, 0x58, 0x18, 0x10, 0x70> as *const ()), // sub_141bab310
    Replacement::draft(0x141bab380, "bonsai_141bab380", tpl_38218::<0xc0, 0x58, 0x18, 0x10, 0x68> as *const ()), // sub_141bab380
    Replacement::draft(0x141bab3f0, "bonsai_141bab3f0", fn_141bab3f0 as *const ()), // sub_141bab3f0
    Replacement::draft(0x141bab480, "bonsai_141bab480", fn_141bab480 as *const ()), // sub_141bab480
    Replacement::draft(0x141bab580, "bonsai_141bab580", fn_141bab580 as *const ()), // sub_141bab580
    Replacement::draft(0x141bab5c0, "bonsai_141bab5c0", fn_141bab5c0 as *const ()), // sub_141bab5c0
    Replacement::draft(0x141bab610, "bonsai_141bab610", fn_141bab610 as *const ()), // sub_141bab610
    Replacement::draft(0x141bab7d0, "bonsai_141bab7d0", fn_141bab7d0 as *const ()), // sub_141bab7d0
    Replacement::draft(0x141bab850, "bonsai_141bab850", fn_141bab850 as *const ()), // sub_141bab850
    Replacement::draft(0x141bab880, "bonsai_141bab880", fn_141bab880 as *const ()), // sub_141bab880
    Replacement::draft(0x141bab9f0, "bonsai_141bab9f0", fn_141bab9f0 as *const ()), // sub_141bab9f0
    Replacement::draft(0x141baba30, "bonsai_141baba30", fn_141baba30 as *const ()), // sub_141baba30
    Replacement::draft(0x141babae0, "bonsai_141babae0", fn_141babae0 as *const ()), // sub_141babae0
    Replacement::draft(0x141babb00, "bonsai_141babb00", fn_141babb00 as *const ()), // sub_141babb00
    Replacement::draft(0x141babb40, "bonsai_141babb40", fn_141babb40 as *const ()), // sub_141babb40
    Replacement::draft(0x141babdc0, "bonsai_141babdc0", fn_141babdc0 as *const ()), // sub_141babdc0
    Replacement::draft(0x141babe30, "bonsai_141babe30", fn_141babe30 as *const ()), // sub_141babe30
    Replacement::draft(0x141babe90, "bonsai_141babe90", fn_141babe90 as *const ()), // sub_141babe90
    Replacement::draft(0x141babf80, "bonsai_141babf80", fn_141babf80 as *const ()), // sub_141babf80
    Replacement::draft(0x141babfa0, "bonsai_141babfa0", fn_141babfa0 as *const ()), // sub_141babfa0
    Replacement::draft(0x141babfb0, "bonsai_141babfb0", fn_141babfb0 as *const ()), // sub_141babfb0
    Replacement::draft(0x141bac090, "bonsai_141bac090", fn_141bac090 as *const ()), // sub_141bac090
    Replacement::draft(0x141bac0d0, "bonsai_141bac0d0", fn_141bac0d0 as *const ()), // sub_141bac0d0
    Replacement::draft(0x141bac130, "bonsai_141bac130", fn_141bac130 as *const ()), // sub_141bac130
    Replacement::draft(0x141bac160, "bonsai_141bac160", fn_141bac160 as *const ()), // sub_141bac160
    Replacement::draft(0x141bac1c0, "bonsai_141bac1c0", fn_141bac1c0 as *const ()), // sub_141bac1c0
    Replacement::draft(0x141bac240, "bonsai_141bac240", fn_141bac240 as *const ()), // sub_141bac240
    Replacement::draft(0x141bac2d0, "bonsai_141bac2d0", fn_141bac2d0 as *const ()), // sub_141bac2d0
    Replacement::draft(0x141bac380, "bonsai_141bac380", fn_141bac380 as *const ()), // sub_141bac380
    Replacement::draft(0x141bac780, "bonsai_141bac780", fn_141bac780 as *const ()), // sub_141bac780
    Replacement::draft_throws(0x141bac8e0, "bonsai_141bac8e0", fn_141bac8e0 as *const (), Throws::Discard), // sub_141bac8e0
    Replacement::draft(0x141bad1f0, "bonsai_141bad1f0", fn_141bad1f0 as *const ()), // bonsai::BehaviorTreeWaitNodeInstance::bonsai::BehaviorTreeWaitNodeInstance::vfunc_6
    Replacement::draft(0x141bad250, "bonsai_141bad250", tpl_38248::<0x141ba3170, 0x50, 0x58, 0x1c, 0x10> as *const ()), // bonsai::BehaviorTreeWaitNodeInstance::bonsai::BehaviorTreeWaitNodeInstance::vfunc_11
    Replacement::draft(0x141bad2c0, "bonsai_141bad2c0", fn_141bad2c0 as *const ()), // bonsai::BehaviorTreeWaitNodeInstance::bonsai::BehaviorTreeWaitNodeInstance::vfunc_10
    Replacement::draft(0x141bad960, "bonsai_141bad960", tpl_38249::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60, 0x68> as *const ()), // bonsai::BehaviorTreeWaitNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeWaitNodeInstance::ClassTypeInfoIm
    Replacement::draft(0x141bada00, "bonsai_141bada00", tpl_38250::<0x143237348, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeWaitNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeWaitNodeInstance::ClassTypeInfoIm
    Replacement::draft(0x141badb90, "bonsai_141badb90", fn_141badb90 as *const ()), // sub_141badb90
    Replacement::draft(0x141badc10, "bonsai_141badc10", fn_141badc10 as *const ()), // sub_141badc10
    Replacement::draft_throws(0x141badd80, "bonsai_141badd80", fn_141badd80 as *const (), Throws::Discard), // bonsai::BehaviorTreeUseWorkspotNodeInstance::bonsai::BehaviorTreeUseWorkspotNodeInstance::vfunc_6
    Replacement::draft(0x141bae090, "bonsai_141bae090", fn_141bae090 as *const ()), // bonsai::BehaviorTreeUseWorkspotNodeInstance::bonsai::BehaviorTreeUseWorkspotNodeInstance::vfunc_11
    Replacement::draft_throws(0x141bae190, "bonsai_141bae190", fn_141bae190 as *const (), Throws::Discard), // bonsai::BehaviorTreeUseWorkspotNodeInstance::bonsai::BehaviorTreeUseWorkspotNodeInstance::vfunc_10
    Replacement::draft(0x141bae900, "bonsai_141bae900", fn_141bae900 as *const ()), // bonsai::BehaviorTreeUseWorkspotNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeUseWorkspotNodeInstance::C
    Replacement::draft(0x141bae9d0, "bonsai_141bae9d0", fn_141bae9d0 as *const ()), // bonsai::BehaviorTreeUseWorkspotNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeUseWorkspotNodeInstance::C
    Replacement::draft(0x141baefb0, "bonsai_141baefb0", tpl_37769::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60> as *const ()), // bonsai::BehaviorTreeUseAbilityNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeUseAbilityNodeInstance::Cla
    Replacement::draft(0x141baf050, "bonsai_141baf050", tpl_37770::<0x1431ea018, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeUseAbilityNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeUseAbilityNodeInstance::Cla
    Replacement::draft(0x141baf310, "bonsai_141baf310", tpl_37767::<0x141ba3170, 0x1c, 0x10, 0xc8, 0x70, 0x141bac780> as *const ()), // bonsai::BehaviorTreeUseAbilityNodeInstance::bonsai::BehaviorTreeUseAbilityNodeInstance::vfunc_11
    Replacement::draft(0x141baf3a0, "bonsai_141baf3a0", fn_141baf3a0 as *const ()), // bonsai::BehaviorTreeUseAbilityNodeInstance::bonsai::BehaviorTreeUseAbilityNodeInstance::vfunc_10
    Replacement::draft(0x141baf4a0, "bonsai_141baf4a0", fn_141baf4a0 as *const ()), // bonsai::BehaviorTreeUseAbilityNodeInstance::bonsai::BehaviorTreeUseAbilityNodeInstance::vfunc_6
    Replacement::draft(0x141baf500, "bonsai_141baf500", tpl_37768::<0x141ba31e0, 0x50, 0x58, 0x1c, 0x8> as *const ()), // bonsai::BehaviorTreeTestNodeInstance::bonsai::BehaviorTreeTestNodeInstance::vfunc_10
    Replacement::draft(0x141bafad0, "bonsai_141bafad0", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::BehaviorTreeTestNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeTestNodeInstance::ClassTypeInfoIm
    Replacement::draft_throws(0x141bafcf0, "bonsai_141bafcf0", fn_141bafcf0 as *const (), Throws::Discard), // sub_141bafcf0
    Replacement::draft_throws(0x141baffd0, "bonsai_141baffd0", fn_141baffd0 as *const (), Throws::Discard), // sub_141baffd0
    Replacement::draft(0x141bb0000, "bonsai_141bb0000", fn_141bb0000 as *const ()), // bonsai::BehaviorTreeTeleportNodeInstance::bonsai::BehaviorTreeTeleportNodeInstance::vfunc_6
    Replacement::draft_throws(0x141bb0050, "bonsai_141bb0050", fn_141bb0050 as *const (), Throws::Discard), // bonsai::BehaviorTreeTeleportNodeInstance::bonsai::BehaviorTreeTeleportNodeInstance::vfunc_10
    Replacement::draft(0x141bb06f0, "bonsai_141bb06f0", fn_141bb06f0 as *const ()), // bonsai::BehaviorTreeTeleportNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeTeleportNodeInstance::ClassTy
    Replacement::draft(0x141bb0790, "bonsai_141bb0790", fn_141bb0790 as *const ()), // bonsai::BehaviorTreeTeleportNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeTeleportNodeInstance::ClassTy
    Replacement::draft(0x141bb0e50, "bonsai_141bb0e50", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::BehaviorTreeTaskNodeBaseInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeTaskNodeBaseInstance::ClassTy
    Replacement::draft(0x141bb12e0, "bonsai_141bb12e0", tpl_37766::<0xc8, 0x70, 0x60, 0x141bac2d0> as *const ()), // bonsai::BehaviorTreeTakeCoverNodeInstance::bonsai::BehaviorTreeTakeCoverNodeInstance::vfunc_6
    Replacement::draft(0x141bb1330, "bonsai_141bb1330", tpl_37767::<0x141ba3170, 0x1c, 0x10, 0xc8, 0x70, 0x141bac780> as *const ()), // bonsai::BehaviorTreeTakeCoverNodeInstance::bonsai::BehaviorTreeTakeCoverNodeInstance::vfunc_11
    Replacement::draft_throws(0x141bb13c0, "bonsai_141bb13c0", fn_141bb13c0 as *const (), Throws::Caught), // bonsai::BehaviorTreeTakeCoverNodeInstance::bonsai::BehaviorTreeTakeCoverNodeInstance::vfunc_10
    Replacement::draft(0x141bb1b00, "bonsai_141bb1b00", fn_141bb1b00 as *const ()), // bonsai::BehaviorTreeTakeCoverNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeTakeCoverNodeInstance::Class
    Replacement::draft(0x141bb1bb0, "bonsai_141bb1bb0", fn_141bb1bb0 as *const ()), // bonsai::BehaviorTreeTakeCoverNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeTakeCoverNodeInstance::Class
    Replacement::draft_throws(0x141bb2070, "bonsai_141bb2070", fn_141bb2070 as *const (), Throws::Discard), // sub_141bb2070
    Replacement::draft_throws(0x141bb21f0, "bonsai_141bb21f0", fn_141bb21f0 as *const (), Throws::Discard), // sub_141bb21f0
    Replacement::draft_throws(0x141bb2680, "bonsai_141bb2680", fn_141bb2680 as *const (), Throws::Caught), // sub_141bb2680
    Replacement::draft_throws(0x141bb2cf0, "bonsai_141bb2cf0", fn_141bb2cf0 as *const (), Throws::Discard), // sub_141bb2cf0
    Replacement::draft_throws(0x141bb33d0, "bonsai_141bb33d0", tpl_35009::<0x14387cb90, 0x14068a620, 0x1402116e0> as *const (), Throws::Discard), // sub_141bb33d0
    Replacement::draft(0x141bb3410, "bonsai_141bb3410", tpl_37766::<0xc8, 0x70, 0x60, 0x141bac2d0> as *const ()), // bonsai::BehaviorTreeStunnedNodeInstance::bonsai::BehaviorTreeStunnedNodeInstance::vfunc_6
    Replacement::draft(0x141bb3460, "bonsai_141bb3460", tpl_37767::<0x141ba3170, 0x1c, 0x10, 0xc8, 0x70, 0x141bac780> as *const ()), // bonsai::BehaviorTreeStunnedNodeInstance::bonsai::BehaviorTreeStunnedNodeInstance::vfunc_11
    Replacement::draft(0x141bb34f0, "bonsai_141bb34f0", fn_141bb34f0 as *const ()), // bonsai::BehaviorTreeStunnedNodeInstance::bonsai::BehaviorTreeStunnedNodeInstance::vfunc_10
    Replacement::draft(0x141bb3b20, "bonsai_141bb3b20", tpl_37769::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60> as *const ()), // bonsai::BehaviorTreeStunnedNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeStunnedNodeInstance::ClassType
    Replacement::draft(0x141bb3bc0, "bonsai_141bb3bc0", tpl_37770::<0x143239cc8, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeStunnedNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeStunnedNodeInstance::ClassType
    Replacement::draft_throws(0x141bb3d50, "bonsai_141bb3d50", fn_141bb3d50 as *const (), Throws::Discard), // bonsai::BehaviorTreeShootFromCoverNodeInstance::bonsai::BehaviorTreeShootFromCoverNodeInstance::vfunc_40
    Replacement::draft_throws(0x141bb3dc0, "bonsai_141bb3dc0", fn_141bb3dc0 as *const (), Throws::Discard), // bonsai::BehaviorTreeShootFromCoverNodeInstance::bonsai::BehaviorTreeShootFromCoverNodeInstance::vfunc_39
    Replacement::draft_throws(0x141bb3e30, "bonsai_141bb3e30", fn_141bb3e30 as *const (), Throws::Discard), // bonsai::BehaviorTreeShootFromCoverNodeInstance::bonsai::BehaviorTreeShootFromCoverNodeInstance::vfunc_38
    Replacement::draft(0x141bb3eb0, "bonsai_141bb3eb0", tpl_38275::<0x48, 0x3c, 0x38, 0x141b554f0, 0x7c> as *const ()), // bonsai::BehaviorTreeShootFromCoverNodeInstance::bonsai::BehaviorTreeShootFromCoverNodeInstance::vfunc_37
    Replacement::draft(0x141bb3ee0, "bonsai_141bb3ee0", fn_141bb3ee0 as *const ()), // sub_141bb3ee0
    Replacement::draft_throws(0x141bb3f40, "bonsai_141bb3f40", fn_141bb3f40 as *const (), Throws::Caught), // bonsai::BehaviorTreeShootFromCoverNodeInstance::bonsai::BehaviorTreeShootFromCoverNodeInstance::vfunc_6
    Replacement::draft(0x141bb4240, "bonsai_141bb4240", fn_141bb4240 as *const ()), // bonsai::BehaviorTreeShootFromCoverNodeInstance::bonsai::BehaviorTreeShootFromCoverNodeInstance::vfunc_11
    Replacement::draft_throws(0x141bb4310, "bonsai_141bb4310", fn_141bb4310 as *const (), Throws::Caught), // bonsai::BehaviorTreeShootFromCoverNodeInstance::bonsai::BehaviorTreeShootFromCoverNodeInstance::vfunc_10
    Replacement::draft(0x141bb4ea0, "bonsai_141bb4ea0", fn_141bb4ea0 as *const ()), // bonsai::BehaviorTreeShootFromCoverNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeShootFromCoverNodeInsta
    Replacement::draft(0x141bb4fb0, "bonsai_141bb4fb0", fn_141bb4fb0 as *const ()), // bonsai::BehaviorTreeShootFromCoverNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeShootFromCoverNodeInsta
    Replacement::draft_throws(0x141bb5170, "bonsai_141bb5170", fn_141bb5170 as *const (), Throws::Discard), // bonsai::BehaviorTreeShootAtNodeInstance::bonsai::BehaviorTreeShootAtNodeInstance::vfunc_40
    Replacement::draft_throws(0x141bb51e0, "bonsai_141bb51e0", fn_141bb51e0 as *const (), Throws::Discard), // bonsai::BehaviorTreeShootAtNodeInstance::bonsai::BehaviorTreeShootAtNodeInstance::vfunc_39
    Replacement::draft_throws(0x141bb5250, "bonsai_141bb5250", fn_141bb5250 as *const (), Throws::Discard), // bonsai::BehaviorTreeShootAtNodeInstance::bonsai::BehaviorTreeShootAtNodeInstance::vfunc_38
    Replacement::draft(0x141bb52e0, "bonsai_141bb52e0", tpl_38275::<0x48, 0x12c, 0x128, 0x141b554f0, 0x6c> as *const ()), // bonsai::BehaviorTreeShootAtNodeInstance::bonsai::BehaviorTreeShootAtNodeInstance::vfunc_37
    Replacement::draft(0x141bb5320, "bonsai_141bb5320", fn_141bb5320 as *const ()), // bonsai::BehaviorTreeShootAtNodeInstance::bonsai::BehaviorTreeShootAtNodeInstance::vfunc_6
    Replacement::draft(0x141bb5350, "bonsai_141bb5350", fn_141bb5350 as *const ()), // bonsai::BehaviorTreeShootAtNodeInstance::bonsai::BehaviorTreeShootAtNodeInstance::vfunc_11
    Replacement::draft(0x141bb53d0, "bonsai_141bb53d0", fn_141bb53d0 as *const ()), // bonsai::BehaviorTreeShootAtNodeInstance::bonsai::BehaviorTreeShootAtNodeInstance::vfunc_10
    Replacement::draft(0x141bb59b0, "bonsai_141bb59b0", fn_141bb59b0 as *const ()), // bonsai::BehaviorTreeShootAtNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeShootAtNodeInstance::ClassType
    Replacement::draft(0x141bb5a60, "bonsai_141bb5a60", fn_141bb5a60 as *const ()), // bonsai::BehaviorTreeShootAtNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeShootAtNodeInstance::ClassType
    Replacement::draft(0x141bb5bf0, "bonsai_141bb5bf0", fn_141bb5bf0 as *const ()), // sub_141bb5bf0
    Replacement::draft(0x141bb60f0, "bonsai_141bb60f0", tpl_35199::<0x14387c8e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImpl
    Replacement::draft(0x141bb61e0, "bonsai_141bb61e0", tpl_35199::<0x14387c8e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImpl
    Replacement::draft(0x141bb63c0, "bonsai_141bb63c0", tpl_35199::<0x14387c8e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImpl
    Replacement::draft(0x141bb64b0, "bonsai_141bb64b0", tpl_35199::<0x14387c8e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImpl
    Replacement::draft(0x141bb6530, "bonsai_141bb6530", tpl_35199::<0x14387c8e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeServiceInstance::ClassTypeInfoImpl
    Replacement::draft(0x141bb6800, "bonsai_141bb6800", fn_141bb6800 as *const ()), // sub_141bb6800
    Replacement::draft(0x141bb6830, "bonsai_141bb6830", fn_141bb6830 as *const ()), // sub_141bb6830
    Replacement::draft(0x141bb6850, "bonsai_141bb6850", fn_141bb6850 as *const ()), // bonsai::BehaviorTreeSequenceNodeInstance::bonsai::BehaviorTreeSequenceNodeInstance::vfunc_15
    Replacement::draft(0x141bb68b0, "bonsai_141bb68b0", tpl_38294::<0x141ba3170, 0x50, 0x58, 0x1c, 0x10, 0x60> as *const ()), // bonsai::BehaviorTreeSequenceNodeInstance::bonsai::BehaviorTreeSequenceNodeInstance::vfunc_11
    Replacement::draft(0x141bb6920, "bonsai_141bb6920", tpl_38295::<0x141ba31e0, 0x50, 0x58, 0x1c, 0x8, 0x60> as *const ()), // bonsai::BehaviorTreeSequenceNodeInstance::bonsai::BehaviorTreeSequenceNodeInstance::vfunc_10
    Replacement::draft(0x141bb6990, "bonsai_141bb6990", tpl_38296::<0x18, 0x10> as *const ()), // bonsai::BehaviorTreeSequenceNodeInstance::bonsai::BehaviorTreeSequenceNodeInstance::vfunc_3
    Replacement::draft(0x141bb6f30, "bonsai_141bb6f30", tpl_38297::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60> as *const ()), // bonsai::BehaviorTreeSequenceNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeSequenceNodeInstance::ClassTy
    Replacement::draft(0x141bb6fd0, "bonsai_141bb6fd0", tpl_38298::<0x14323a798, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeSequenceNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeSequenceNodeInstance::ClassTy
    Replacement::draft_throws(0x141bb7160, "bonsai_141bb7160", fn_141bb7160 as *const (), Throws::Discard), // sub_141bb7160
    Replacement::draft_throws(0x141bb71e0, "bonsai_141bb71e0", fn_141bb71e0 as *const (), Throws::Discard), // sub_141bb71e0
    Replacement::draft_throws(0x141bb7380, "bonsai_141bb7380", fn_141bb7380 as *const (), Throws::Discard), // sub_141bb7380
    Replacement::draft_throws(0x141bb7490, "bonsai_141bb7490", fn_141bb7490 as *const (), Throws::Discard), // sub_141bb7490
    Replacement::draft_throws(0x141bb7540, "bonsai_141bb7540", fn_141bb7540 as *const (), Throws::Discard), // bonsai::BehaviorTreeSelectorNodeInstance::bonsai::BehaviorTreeSelectorNodeInstance::vfunc_4
    Replacement::draft(0x141bb77b0, "bonsai_141bb77b0", fn_141bb77b0 as *const ()), // bonsai::BehaviorTreeSelectorNodeInstance::bonsai::BehaviorTreeSelectorNodeInstance::vfunc_9
    Replacement::draft_throws(0x141bb7870, "bonsai_141bb7870", fn_141bb7870 as *const (), Throws::Discard), // bonsai::BehaviorTreeSelectorNodeInstance::bonsai::BehaviorTreeSelectorNodeInstance::vfunc_15
    Replacement::draft_throws(0x141bb7950, "bonsai_141bb7950", fn_141bb7950 as *const (), Throws::Discard), // bonsai::BehaviorTreeSelectorNodeInstance::bonsai::BehaviorTreeSelectorNodeInstance::vfunc_11
    Replacement::draft_throws(0x141bb7a80, "bonsai_141bb7a80", fn_141bb7a80 as *const (), Throws::Discard), // bonsai::BehaviorTreeSelectorNodeInstance::bonsai::BehaviorTreeSelectorNodeInstance::vfunc_10
    Replacement::draft(0x141bb7c40, "bonsai_141bb7c40", fn_141bb7c40 as *const ()), // bonsai::BehaviorTreeSelectorNodeInstance::bonsai::BehaviorTreeSelectorNodeInstance::vfunc_3
    Replacement::draft(0x141bb8240, "bonsai_141bb8240", fn_141bb8240 as *const ()), // bonsai::BehaviorTreeSelectorNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeSelectorNodeInstance::ClassTy
    Replacement::draft(0x141bb8300, "bonsai_141bb8300", fn_141bb8300 as *const ()), // bonsai::BehaviorTreeSelectorNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeSelectorNodeInstance::ClassTy
    Replacement::draft(0x141bb8490, "bonsai_141bb8490", fn_141bb8490 as *const ()), // bonsai::BehaviorTreeSayNodeInstance::bonsai::BehaviorTreeSayNodeInstance::vfunc_6
    Replacement::draft(0x141bb84e0, "bonsai_141bb84e0", fn_141bb84e0 as *const ()), // bonsai::BehaviorTreeSayNodeInstance::bonsai::BehaviorTreeSayNodeInstance::vfunc_10
    Replacement::draft(0x141bb8ac0, "bonsai_141bb8ac0", tpl_38249::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60, 0x68> as *const ()), // bonsai::BehaviorTreeSayNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeSayNodeInstance::ClassTypeInfoImpl
    Replacement::draft(0x141bb8b60, "bonsai_141bb8b60", tpl_38250::<0x14323af00, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeSayNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeSayNodeInstance::ClassTypeInfoImpl
    Replacement::draft(0x141bb8cf0, "bonsai_141bb8cf0", fn_141bb8cf0 as *const ()), // bonsai::BehaviorTreeRootNodeInstance::bonsai::BehaviorTreeRootNodeInstance::vfunc_15
    Replacement::draft(0x141bb8d20, "bonsai_141bb8d20", tpl_38313::<0x10, 0x18> as *const ()), // bonsai::BehaviorTreeRootNodeInstance::bonsai::BehaviorTreeRootNodeInstance::vfunc_3
    Replacement::draft(0x141bb9550, "bonsai_141bb9550", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::BehaviorTreeRootNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeRootNodeInstance::ClassTypeInfoIm
    Replacement::draft(0x141bb9770, "bonsai_141bb9770", tpl_38007::<0x10, 0x18> as *const ()), // bonsai::BehaviorTreeReferenceNodeInstance::bonsai::BehaviorTreeReferenceNodeInstance::vfunc_3
    Replacement::draft(0x141bb9f40, "bonsai_141bb9f40", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::BehaviorTreeReferenceNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeReferenceNodeInstance::Class
    Replacement::draft(0x141bba160, "bonsai_141bba160", fn_141bba160 as *const ()), // sub_141bba160
    Replacement::draft_throws(0x141bba240, "bonsai_141bba240", fn_141bba240 as *const (), Throws::Discard), // sub_141bba240
    Replacement::draft(0x141bba490, "bonsai_141bba490", fn_141bba490 as *const ()), // bonsai::BehaviorTreeRandomSelectorNodeInstance::bonsai::BehaviorTreeRandomSelectorNodeInstance::vfunc_15
    Replacement::draft_throws(0x141bba4a0, "bonsai_141bba4a0", fn_141bba4a0 as *const (), Throws::Discard), // bonsai::BehaviorTreeRandomSelectorNodeInstance::bonsai::BehaviorTreeRandomSelectorNodeInstance::vfunc_3
    Replacement::draft(0x141bba520, "bonsai_141bba520", fn_141bba520 as *const ()), // sub_141bba520
    Replacement::draft(0x141bbaae0, "bonsai_141bbaae0", fn_141bbaae0 as *const ()), // bonsai::BehaviorTreeRandomSelectorNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeRandomSelectorNodeInsta
    Replacement::draft(0x141bbab80, "bonsai_141bbab80", fn_141bbab80 as *const ()), // bonsai::BehaviorTreeRandomSelectorNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeRandomSelectorNodeInsta
    Replacement::draft(0x141bbad20, "bonsai_141bbad20", fn_141bbad20 as *const ()), // bonsai::BehaviorTreeParallelNodeInstance::bonsai::BehaviorTreeParallelNodeInstance::vfunc_15
    Replacement::draft(0x141bbada0, "bonsai_141bbada0", fn_141bbada0 as *const ()), // bonsai::BehaviorTreeParallelNodeInstance::bonsai::BehaviorTreeParallelNodeInstance::vfunc_3
    Replacement::draft(0x141bbb420, "bonsai_141bbb420", tpl_38297::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60> as *const ()), // bonsai::BehaviorTreeParallelNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeParallelNodeInstance::ClassTy
    Replacement::draft(0x141bbb4c0, "bonsai_141bbb4c0", tpl_38298::<0x14323bb78, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeParallelNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeParallelNodeInstance::ClassTy
    Replacement::draft(0x141bbb660, "bonsai_141bbb660", tpl_38186::<0x50> as *const ()), // sub_141bbb660
    Replacement::draft(0x141bbb670, "bonsai_141bbb670", tpl_38006::<0x8, 0x10, 0x50, 0x58> as *const ()), // sub_141bbb670
    Replacement::draft(0x141bbbb20, "bonsai_141bbbb20", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::BehaviorTreeNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeNodeInstance::ClassTypeInfoImplementa
    Replacement::draft(0x141bbbbb0, "bonsai_141bbbbb0", tpl_38008::<0x1431ea288, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeNodeInstance::ClassTypeInfoImplementa
    Replacement::draft(0x141bbbe70, "bonsai_141bbbe70", fn_141bbbe70 as *const ()), // bonsai::BehaviorTreeNodeInstance::bonsai::BehaviorTreeNodeInstance::vfunc_12
    Replacement::draft(0x141bbbf20, "bonsai_141bbbf20", tpl_38248::<0x141ba3170, 0x50, 0x58, 0x1c, 0x10> as *const ()), // bonsai::BehaviorTreeNodeInstance::bonsai::BehaviorTreeNodeInstance::vfunc_11
    Replacement::draft(0x141bbbf90, "bonsai_141bbbf90", tpl_37768::<0x141ba31e0, 0x50, 0x58, 0x1c, 0x8> as *const ()), // bonsai::BehaviorTreeNodeInstance::bonsai::BehaviorTreeNodeInstance::vfunc_10
    Replacement::draft(0x141bbc000, "bonsai_141bbc000", fn_141bbc000 as *const ()), // bonsai::BehaviorTreeNodeInstance::bonsai::BehaviorTreeNodeInstance::vfunc_8
    Replacement::draft(0x141bbc0a0, "bonsai_141bbc0a0", tpl_38313::<0x10, 0x18> as *const ()), // bonsai::BehaviorTreeNodeInstance::bonsai::BehaviorTreeNodeInstance::vfunc_3
    Replacement::draft_throws(0x141bbc120, "bonsai_141bbc120", fn_141bbc120 as *const (), Throws::Caught), // bonsai::BehaviorTreeMoveToNodeInstance::bonsai::BehaviorTreeMoveToNodeInstance::vfunc_6
    Replacement::draft(0x141bbc4c0, "bonsai_141bbc4c0", fn_141bbc4c0 as *const ()), // bonsai::BehaviorTreeMoveToNodeInstance::bonsai::BehaviorTreeMoveToNodeInstance::vfunc_11
    Replacement::draft_throws(0x141bbc580, "bonsai_141bbc580", fn_141bbc580 as *const (), Throws::Discard), // bonsai::BehaviorTreeMoveToNodeInstance::bonsai::BehaviorTreeMoveToNodeInstance::vfunc_10
    Replacement::draft(0x141bbcd10, "bonsai_141bbcd10", fn_141bbcd10 as *const ()), // bonsai::BehaviorTreeMoveToNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeMoveToNodeInstance::ClassTypeIn
    Replacement::draft(0x141bbce10, "bonsai_141bbce10", fn_141bbce10 as *const ()), // bonsai::BehaviorTreeMoveToNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeMoveToNodeInstance::ClassTypeIn
    Replacement::draft(0x141bbcfd0, "unknown_141bbcfd0", fn_141bbcfd0 as *const ()), // sub_141bbcfd0
    Replacement::draft(0x141bbd0e0, "unknown_141bbd0e0", tpl_37798::<0x1434268d0> as *const ()), // sub_141bbd0e0
    Replacement::draft(0x141bbd130, "unknown_141bbd130", fn_141bbd130 as *const ()), // sub_141bbd130
    Replacement::draft(0x141bbd220, "unknown_141bbd220", fn_141bbd220 as *const ()), // sub_141bbd220
    Replacement::draft(0x141bbd250, "unknown_141bbd250", fn_141bbd250 as *const ()), // sub_141bbd250
    Replacement::draft(0x141bbd390, "unknown_141bbd390", fn_141bbd390 as *const ()), // sub_141bbd390
    Replacement::draft(0x141bbd460, "unknown_141bbd460", fn_141bbd460 as *const ()), // sub_141bbd460
    Replacement::draft(0x141bbd480, "unknown_141bbd480", fn_141bbd480 as *const ()), // sub_141bbd480
    Replacement::draft_throws(0x141bbd5d0, "kaim_141bbd5d0", fn_141bbd5d0 as *const (), Throws::Discard), // sub_141bbd5d0
    Replacement::draft(0x141bbd870, "kaim_141bbd870", fn_141bbd870 as *const ()), // sub_141bbd870
    Replacement::draft(0x141bbdd80, "kaim_141bbdd80", fn_141bbdd80 as *const ()), // sub_141bbdd80
    Replacement::draft_throws(0x141bbde40, "kaim_141bbde40", fn_141bbde40 as *const (), Throws::Caught), // sub_141bbde40
    Replacement::draft_throws(0x141bbe720, "unknown_141bbe720", fn_141bbe720 as *const (), Throws::Caught), // sub_141bbe720
    Replacement::draft_throws(0x141bbefa0, "bonsai_141bbefa0", fn_141bbefa0 as *const (), Throws::Discard), // sub_141bbefa0
    Replacement::draft_throws(0x141bbf120, "bonsai_141bbf120", fn_141bbf120 as *const (), Throws::Discard), // sub_141bbf120
    Replacement::draft_throws(0x141bbf150, "bonsai_141bbf150", fn_141bbf150 as *const (), Throws::Discard), // sub_141bbf150
    Replacement::draft_throws(0x141bbf5a0, "bonsai_141bbf5a0", fn_141bbf5a0 as *const (), Throws::Caught), // sub_141bbf5a0
    Replacement::draft_throws(0x141bbf870, "bonsai_141bbf870", fn_141bbf870 as *const (), Throws::Caught), // bonsai::BehaviorTreeMoveToFollowInstance::bonsai::BehaviorTreeMoveToFollowInstance::vfunc_6
    Replacement::draft(0x141bbfc30, "bonsai_141bbfc30", fn_141bbfc30 as *const ()), // bonsai::BehaviorTreeMoveToFollowInstance::bonsai::BehaviorTreeMoveToFollowInstance::vfunc_11
    Replacement::draft_throws(0x141bbfd10, "bonsai_141bbfd10", fn_141bbfd10 as *const (), Throws::Discard), // bonsai::BehaviorTreeMoveToFollowInstance::bonsai::BehaviorTreeMoveToFollowInstance::vfunc_10
    Replacement::draft(0x141bc0320, "bonsai_141bc0320", fn_141bc0320 as *const ()), // bonsai::BehaviorTreeMoveToFollowInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeMoveToFollowInstance::ClassTy
    Replacement::draft(0x141bc05e0, "bonsai_141bc05e0", fn_141bc05e0 as *const ()), // bonsai::BehaviorTreeMoveToFollowInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeMoveToFollowInstance::ClassTy
    Replacement::draft(0x141bc07b0, "bonsai_141bc07b0", fn_141bc07b0 as *const ()), // sub_141bc07b0
    Replacement::draft_throws(0x141bc0830, "bonsai_141bc0830", fn_141bc0830 as *const (), Throws::Caught), // sub_141bc0830
    Replacement::draft_throws(0x141bc09e0, "bonsai_141bc09e0", fn_141bc09e0 as *const (), Throws::Caught), // sub_141bc09e0
    Replacement::draft(0x141bc0b70, "bonsai_141bc0b70", fn_141bc0b70 as *const ()), // sub_141bc0b70
    Replacement::draft_throws(0x141bc1060, "bonsai_141bc1060", fn_141bc1060 as *const (), Throws::Caught), // bonsai::BehaviorTreeMoveToCoverNodeInstance::bonsai::BehaviorTreeMoveToCoverNodeInstance::vfunc_6
    Replacement::draft_throws(0x141bc1460, "bonsai_141bc1460", fn_141bc1460 as *const (), Throws::Discard), // bonsai::BehaviorTreeMoveToCoverNodeInstance::bonsai::BehaviorTreeMoveToCoverNodeInstance::vfunc_11
    Replacement::draft_throws(0x141bc1580, "bonsai_141bc1580", fn_141bc1580 as *const (), Throws::Discard), // bonsai::BehaviorTreeMoveToCoverNodeInstance::bonsai::BehaviorTreeMoveToCoverNodeInstance::vfunc_10
    Replacement::draft(0x141bc1b50, "bonsai_141bc1b50", fn_141bc1b50 as *const ()), // bonsai::BehaviorTreeMoveToCoverNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeMoveToCoverNodeInstance::C
    Replacement::draft(0x141bc1c40, "bonsai_141bc1c40", fn_141bc1c40 as *const ()), // bonsai::BehaviorTreeMoveToCoverNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeMoveToCoverNodeInstance::C
    Replacement::draft_throws(0x141bc1e00, "bonsai_141bc1e00", fn_141bc1e00 as *const (), Throws::Discard), // sub_141bc1e00
    Replacement::draft_throws(0x141bc1f20, "bonsai_141bc1f20", fn_141bc1f20 as *const (), Throws::Discard), // sub_141bc1f20
    Replacement::draft_throws(0x141bc20a0, "bonsai_141bc20a0", fn_141bc20a0 as *const (), Throws::Discard), // sub_141bc20a0
    Replacement::draft_throws(0x141bc2200, "bonsai_141bc2200", fn_141bc2200 as *const (), Throws::Discard), // sub_141bc2200
    Replacement::draft_throws(0x141bc24a0, "bonsai_141bc24a0", fn_141bc24a0 as *const (), Throws::Discard), // sub_141bc24a0
    Replacement::draft_throws(0x141bc2670, "bonsai_141bc2670", fn_141bc2670 as *const (), Throws::Discard), // bonsai::BehaviorTreeMoveOnRouteNodeInstance::bonsai::BehaviorTreeMoveOnRouteNodeInstance::vfunc_6
    Replacement::draft(0x141bc27a0, "bonsai_141bc27a0", fn_141bc27a0 as *const ()), // bonsai::BehaviorTreeMoveOnRouteNodeInstance::bonsai::BehaviorTreeMoveOnRouteNodeInstance::vfunc_11
    Replacement::draft_throws(0x141bc28e0, "bonsai_141bc28e0", fn_141bc28e0 as *const (), Throws::Caught), // sub_141bc28e0
    Replacement::draft_throws(0x141bc3120, "bonsai_141bc3120", fn_141bc3120 as *const (), Throws::Caught), // sub_141bc3120
    Replacement::draft_throws(0x141bc34e0, "bonsai_141bc34e0", fn_141bc34e0 as *const (), Throws::Discard), // bonsai::BehaviorTreeMoveOnRouteNodeInstance::bonsai::BehaviorTreeMoveOnRouteNodeInstance::vfunc_10
    Replacement::draft(0x141bc3ba0, "bonsai_141bc3ba0", fn_141bc3ba0 as *const ()), // bonsai::BehaviorTreeMoveOnRouteNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeMoveOnRouteNodeInstance::C
    Replacement::draft(0x141bc3e70, "bonsai_141bc3e70", fn_141bc3e70 as *const ()), // bonsai::BehaviorTreeLoopNodeInstance::bonsai::BehaviorTreeLoopNodeInstance::vfunc_15
    Replacement::draft(0x141bc3ee0, "bonsai_141bc3ee0", tpl_38294::<0x141ba3170, 0x50, 0x58, 0x1c, 0x10, 0x60> as *const ()), // bonsai::BehaviorTreeLoopNodeInstance::bonsai::BehaviorTreeLoopNodeInstance::vfunc_11
    Replacement::draft(0x141bc3f50, "bonsai_141bc3f50", tpl_38295::<0x141ba31e0, 0x50, 0x58, 0x1c, 0x8, 0x60> as *const ()), // bonsai::BehaviorTreeLoopNodeInstance::bonsai::BehaviorTreeLoopNodeInstance::vfunc_10
    Replacement::draft(0x141bc3fc0, "bonsai_141bc3fc0", tpl_38296::<0x18, 0x10> as *const ()), // bonsai::BehaviorTreeLoopNodeInstance::bonsai::BehaviorTreeLoopNodeInstance::vfunc_3
    Replacement::draft(0x141bc4560, "bonsai_141bc4560", tpl_38297::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60> as *const ()), // bonsai::BehaviorTreeLoopNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeLoopNodeInstance::ClassTypeInfoIm
    Replacement::draft(0x141bc4600, "bonsai_141bc4600", tpl_38298::<0x14323cee0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeLoopNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeLoopNodeInstance::ClassTypeInfoIm
    Replacement::draft(0x141bc4790, "bonsai_141bc4790", tpl_37766::<0xc8, 0x70, 0x60, 0x141bac2d0> as *const ()), // bonsai::BehaviorTreeLevitateNodeInstance::bonsai::BehaviorTreeLevitateNodeInstance::vfunc_6
    Replacement::draft(0x141bc47e0, "bonsai_141bc47e0", tpl_37767::<0x141ba3170, 0x1c, 0x10, 0xc8, 0x70, 0x141bac780> as *const ()), // bonsai::BehaviorTreeLevitateNodeInstance::bonsai::BehaviorTreeLevitateNodeInstance::vfunc_11
    Replacement::draft(0x141bc4870, "bonsai_141bc4870", fn_141bc4870 as *const ()), // bonsai::BehaviorTreeLevitateNodeInstance::bonsai::BehaviorTreeLevitateNodeInstance::vfunc_10
    Replacement::draft(0x141bc4ea0, "bonsai_141bc4ea0", tpl_37769::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60> as *const ()), // bonsai::BehaviorTreeLevitateNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeLevitateNodeInstance::ClassTy
    Replacement::draft(0x141bc4f40, "bonsai_141bc4f40", tpl_37770::<0x14323d040, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeLevitateNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeLevitateNodeInstance::ClassTy
    Replacement::draft_throws(0x141bc50d0, "bonsai_141bc50d0", fn_141bc50d0 as *const (), Throws::Discard), // sub_141bc50d0
    Replacement::draft_throws(0x141bc5190, "bonsai_141bc5190", fn_141bc5190 as *const (), Throws::Caught), // bonsai::BehaviorTreeFlyToNodeInstance::bonsai::BehaviorTreeFlyToNodeInstance::vfunc_6
    Replacement::draft(0x141bc5350, "bonsai_141bc5350", fn_141bc5350 as *const ()), // bonsai::BehaviorTreeFlyToNodeInstance::bonsai::BehaviorTreeFlyToNodeInstance::vfunc_11
    Replacement::draft_throws(0x141bc5410, "bonsai_141bc5410", fn_141bc5410 as *const (), Throws::Caught), // bonsai::BehaviorTreeFlyToNodeInstance::bonsai::BehaviorTreeFlyToNodeInstance::vfunc_10
    Replacement::draft(0x141bc5b50, "bonsai_141bc5b50", fn_141bc5b50 as *const ()), // bonsai::BehaviorTreeFlyToNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeFlyToNodeInstance::ClassTypeInfo
    Replacement::draft(0x141bc5c00, "bonsai_141bc5c00", fn_141bc5c00 as *const ()), // bonsai::BehaviorTreeFlyToNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeFlyToNodeInstance::ClassTypeInfo
    Replacement::draft(0x141bc5da0, "bonsai_141bc5da0", fn_141bc5da0 as *const ()), // bonsai::BehaviorTreeEquipNextWeaponNodeInstance::bonsai::BehaviorTreeEquipNextWeaponNodeInstance::vfunc_6
    Replacement::draft(0x141bc5e00, "bonsai_141bc5e00", fn_141bc5e00 as *const ()), // bonsai::BehaviorTreeEquipNextWeaponNodeInstance::bonsai::BehaviorTreeEquipNextWeaponNodeInstance::vfunc_10
    Replacement::draft(0x141bc63f0, "bonsai_141bc63f0", fn_141bc63f0 as *const ()), // bonsai::BehaviorTreeEquipNextWeaponNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeEquipNextWeaponNodeIns
    Replacement::draft(0x141bc64a0, "bonsai_141bc64a0", fn_141bc64a0 as *const ()), // bonsai::BehaviorTreeEquipNextWeaponNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeEquipNextWeaponNodeIns
    Replacement::draft(0x141bc6630, "bonsai_141bc6630", tpl_38047::<0x48, 0x143c8d164, 0x38, 0x580e0, 0x141b91990> as *const ()), // sub_141bc6630
    Replacement::draft_throws(0x141bc6670, "bonsai_141bc6670", tpl_38048::<0x48, 0x143c8d164, 0x38, 0x580e0, 0x141b91ac0> as *const (), Throws::Discard), // sub_141bc6670
    Replacement::draft_throws(0x141bc66b0, "bonsai_141bc66b0", fn_141bc66b0 as *const (), Throws::Discard), // sub_141bc66b0
    Replacement::draft_throws(0x141bc6770, "bonsai_141bc6770", fn_141bc6770 as *const (), Throws::Discard), // bonsai::BehaviorTreeEQSQueryNodeInstance::bonsai::BehaviorTreeEQSQueryNodeInstance::vfunc_6
    Replacement::draft(0x141bc6930, "bonsai_141bc6930", fn_141bc6930 as *const ()), // bonsai::BehaviorTreeEQSQueryNodeInstance::bonsai::BehaviorTreeEQSQueryNodeInstance::vfunc_11
    Replacement::draft(0x141bc69c0, "bonsai_141bc69c0", fn_141bc69c0 as *const ()), // bonsai::BehaviorTreeEQSQueryNodeInstance::bonsai::BehaviorTreeEQSQueryNodeInstance::vfunc_10
    Replacement::draft(0x141bc6af0, "bonsai_141bc6af0", tpl_38047::<0x48, 0x143c8d164, 0x38, 0x580e0, 0x141b91990> as *const ()), // bonsai::BehaviorTreeEQSQueryNodeInstance::bonsai::BehaviorTreeEQSQueryNodeInstance::vfunc_14
    Replacement::draft_throws(0x141bc6b30, "bonsai_141bc6b30", tpl_38048::<0x48, 0x143c8d164, 0x38, 0x580e0, 0x141b91ac0> as *const (), Throws::Discard), // bonsai::BehaviorTreeEQSQueryNodeInstance::bonsai::BehaviorTreeEQSQueryNodeInstance::vfunc_13
    Replacement::draft(0x141bc70d0, "bonsai_141bc70d0", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::BehaviorTreeEQSQueryNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeEQSQueryNodeInstance::ClassTy
    Replacement::draft(0x141bc72f0, "bonsai_141bc72f0", fn_141bc72f0 as *const ()), // sub_141bc72f0
    Replacement::draft(0x141bc7360, "bonsai_141bc7360", fn_141bc7360 as *const ()), // bonsai::BehaviorTreeDetachChildTreeNodeInstance::bonsai::BehaviorTreeDetachChildTreeNodeInstance::vfunc_6
    Replacement::draft(0x141bc7970, "bonsai_141bc7970", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::BehaviorTreeDetachChildTreeNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeDetachChildTreeNodeIns
    Replacement::draft(0x141bc7cd0, "bonsai_141bc7cd0", tpl_35199::<0x14387c9e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfo
    Replacement::draft(0x141bc7dc0, "bonsai_141bc7dc0", tpl_35199::<0x14387c9e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfo
    Replacement::draft(0x141bc7fa0, "bonsai_141bc7fa0", tpl_35199::<0x14387c9e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfo
    Replacement::draft(0x141bc8090, "bonsai_141bc8090", tpl_35199::<0x14387c9e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfo
    Replacement::draft(0x141bc8110, "bonsai_141bc8110", tpl_35199::<0x14387c9e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfo
    Replacement::draft(0x141bc81c0, "bonsai_141bc81c0", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfo
    Replacement::draft(0x141bc8230, "bonsai_141bc8230", fn_141bc8230 as *const ()), // bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeDecoratorInstance::ClassTypeInfo
    Replacement::draft(0x141bc83a0, "bonsai_141bc83a0", fn_141bc83a0 as *const ()), // sub_141bc83a0
    Replacement::draft(0x141bc8440, "bonsai_141bc8440", tpl_38186::<0x28> as *const ()), // sub_141bc8440
    Replacement::draft(0x141bc8450, "bonsai_141bc8450", tpl_38185::<0x28, 0x20> as *const ()), // sub_141bc8450
    Replacement::draft(0x141bc8830, "bonsai_141bc8830", tpl_38191::<0x20, 0x60> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_12
    Replacement::draft(0x141bc88b0, "bonsai_141bc88b0", tpl_38028::<0x20, 0x58> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_11
    Replacement::draft(0x141bc8910, "bonsai_141bc8910", tpl_38027::<0x20, 0x50> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_10
    Replacement::draft(0x141bc8970, "bonsai_141bc8970", tpl_38189::<0x20, 0x48> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_9
    Replacement::draft(0x141bc89f0, "bonsai_141bc89f0", fn_141bc89f0 as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_8
    Replacement::draft(0x141bc8a50, "bonsai_141bc8a50", fn_141bc8a50 as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_7
    Replacement::draft(0x141bc8ad0, "bonsai_141bc8ad0", fn_141bc8ad0 as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_6
    Replacement::draft(0x141bc8b50, "bonsai_141bc8b50", fn_141bc8b50 as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_5
    Replacement::draft(0x141bc8bb0, "bonsai_141bc8bb0", tpl_37825::<0x10, 0x20> as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_4
    Replacement::draft(0x141bc8c10, "bonsai_141bc8c10", fn_141bc8c10 as *const ()), // bonsai::BehaviorTreeDecoratorInstance::bonsai::BehaviorTreeDecoratorInstance::vfunc_2
    Replacement::draft(0x141bc91a0, "bonsai_141bc91a0", tpl_37528::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50> as *const ()), // bonsai::BehaviorTreeContainerNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeContainerNodeInstance::Class
    Replacement::draft_throws(0x141bc94e0, "bonsai_141bc94e0", tpl_838::<0x14068ee00> as *const (), Throws::Discard), // bonsai::BehaviorTreeContainerNodeInstance::bonsai::BehaviorTreeContainerNodeInstance::vfunc_27
    Replacement::draft_throws(0x141bc94f0, "bonsai_141bc94f0", tpl_838::<0x14068ee00> as *const (), Throws::Discard), // bonsai::BehaviorTreeContainerNodeInstance::bonsai::BehaviorTreeContainerNodeInstance::vfunc_26
    Replacement::draft_throws(0x141bc9500, "bonsai_141bc9500", tpl_838::<0x14068ee00> as *const (), Throws::Discard), // bonsai::BehaviorTreeContainerNodeInstance::bonsai::BehaviorTreeContainerNodeInstance::vfunc_25
    Replacement::draft(0x141bc9630, "bonsai_141bc9630", fn_141bc9630 as *const ()), // bonsai::BehaviorTreeChildTreeNodeInstance::bonsai::BehaviorTreeChildTreeNodeInstance::vfunc_3
    Replacement::draft(0x141bc9bc0, "bonsai_141bc9bc0", fn_141bc9bc0 as *const ()), // bonsai::BehaviorTreeChildTreeNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeChildTreeNodeInstance::Class
    Replacement::draft(0x141bc9c60, "bonsai_141bc9c60", fn_141bc9c60 as *const ()), // bonsai::BehaviorTreeChildTreeNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeChildTreeNodeInstance::Class
    Replacement::draft(0x141bc9df0, "bonsai_141bc9df0", tpl_4843::<0x60, 0x41> as *const ()), // sub_141bc9df0
    Replacement::draft(0x141bca100, "bonsai_141bca100", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ca7888, 0x158, 0x14020e8f0> as *const ()), // sub_141bca100
    Replacement::draft(0x141bca200, "bonsai_141bca200", fn_141bca200 as *const ()), // sub_141bca200
    Replacement::draft_throws(0x141bca3a0, "bonsai_141bca3a0", fn_141bca3a0 as *const (), Throws::Discard), // sub_141bca3a0
    Replacement::draft(0x141bca9a0, "bonsai_141bca9a0", fn_141bca9a0 as *const ()), // sub_141bca9a0
    Replacement::draft(0x141bcaa60, "bonsai_141bcaa60", fn_141bcaa60 as *const ()), // sub_141bcaa60
    Replacement::draft(0x141bcab00, "bonsai_141bcab00", fn_141bcab00 as *const ()), // sub_141bcab00
    Replacement::draft_throws(0x141bcac90, "bonsai_141bcac90", fn_141bcac90 as *const (), Throws::Discard), // sub_141bcac90
    Replacement::draft(0x141bcad40, "bonsai_141bcad40", fn_141bcad40 as *const ()), // sub_141bcad40
    Replacement::draft(0x141bcad50, "bonsai_141bcad50", fn_141bcad50 as *const ()), // sub_141bcad50
    Replacement::draft(0x141bcadc0, "bonsai_141bcadc0", fn_141bcadc0 as *const ()), // sub_141bcadc0
    Replacement::draft_throws(0x141bcae30, "bonsai_141bcae30", fn_141bcae30 as *const (), Throws::Discard), // sub_141bcae30
    Replacement::draft_throws(0x141bcb3a0, "bonsai_141bcb3a0", fn_141bcb3a0 as *const (), Throws::Caught), // bonsai::BehaviorTreeAnimatedMoveToNodeInstance::bonsai::BehaviorTreeAnimatedMoveToNodeInstance::vfunc_6
    Replacement::draft_throws(0x141bcb510, "bonsai_141bcb510", fn_141bcb510 as *const (), Throws::Caught), // bonsai::BehaviorTreeAnimatedMoveToNodeInstance::bonsai::BehaviorTreeAnimatedMoveToNodeInstance::vfunc_10
    Replacement::draft(0x141bcbbd0, "bonsai_141bcbbd0", tpl_37769::<0x8, 0x10, 0x20, 0x28, 0x38, 0x40, 0x41, 0x48, 0x50, 0x60> as *const ()), // bonsai::BehaviorTreeAnimatedMoveToNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeAnimatedMoveToNodeInsta
    Replacement::draft(0x141bcbc70, "bonsai_141bcbc70", tpl_37770::<0x14323f6a0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // bonsai::BehaviorTreeAnimatedMoveToNodeInstance::ClassTypeInfoImplementation::bonsai::BehaviorTreeAnimatedMoveToNodeInsta
    Replacement::draft_throws(0x141bcbe00, "bonsai_141bcbe00", fn_141bcbe00 as *const (), Throws::Discard), // sub_141bcbe00
    Replacement::draft_throws(0x141bcbed0, "bonsai_141bcbed0", fn_141bcbed0 as *const (), Throws::Discard), // sub_141bcbed0
    Replacement::draft(0x141bcbf60, "bonsai_141bcbf60", fn_141bcbf60 as *const ()), // sub_141bcbf60
    Replacement::draft(0x141bcc000, "bonsai_141bcc000", fn_141bcc000 as *const ()), // sub_141bcc000
    Replacement::draft(0x141bcc0a0, "bonsai_141bcc0a0", fn_141bcc0a0 as *const ()), // sub_141bcc0a0
    Replacement::draft(0x141bcc1b0, "bonsai_141bcc1b0", fn_141bcc1b0 as *const ()), // sub_141bcc1b0
    Replacement::draft(0x141bcc250, "bonsai_141bcc250", fn_141bcc250 as *const ()), // sub_141bcc250
    Replacement::draft_throws(0x141bcc280, "bonsai_141bcc280", fn_141bcc280 as *const (), Throws::Caught), // sub_141bcc280
    Replacement::draft_throws(0x141bcc390, "bonsai_141bcc390", fn_141bcc390 as *const (), Throws::Discard), // sub_141bcc390
    Replacement::draft(0x141bcc700, "bonsai_141bcc700", fn_141bcc700 as *const ()), // sub_141bcc700
    Replacement::draft_throws(0x141bcc770, "bonsai_141bcc770", fn_141bcc770 as *const (), Throws::Discard), // sub_141bcc770
    Replacement::draft(0x141bcc870, "bonsai_141bcc870", fn_141bcc870 as *const ()), // sub_141bcc870
    Replacement::draft(0x141bcc950, "bonsai_141bcc950", fn_141bcc950 as *const ()), // sub_141bcc950
    Replacement::draft_throws(0x141bcc990, "bonsai_141bcc990", fn_141bcc990 as *const (), Throws::Discard), // sub_141bcc990
    Replacement::draft_throws(0x141bcc9d0, "bonsai_141bcc9d0", fn_141bcc9d0 as *const (), Throws::Discard), // sub_141bcc9d0
    Replacement::draft_throws(0x141bcca10, "bonsai_141bcca10", fn_141bcca10 as *const (), Throws::Discard), // sub_141bcca10
    Replacement::draft_throws(0x141bccbf0, "bonsai_141bccbf0", fn_141bccbf0 as *const (), Throws::Discard), // sub_141bccbf0
    Replacement::draft(0x141bccc70, "bonsai_141bccc70", fn_141bccc70 as *const ()), // sub_141bccc70
    Replacement::draft_throws(0x141bccd80, "bonsai_141bccd80", fn_141bccd80 as *const (), Throws::Discard), // sub_141bccd80
    Replacement::draft_throws(0x141bcd1d0, "bonsai_141bcd1d0", fn_141bcd1d0 as *const (), Throws::Discard), // sub_141bcd1d0
    Replacement::draft_throws(0x141bcd380, "bonsai_141bcd380", fn_141bcd380 as *const (), Throws::Discard), // sub_141bcd380
    Replacement::draft(0x141bcd530, "bonsai_141bcd530", fn_141bcd530 as *const ()), // sub_141bcd530
    Replacement::draft_throws(0x141bcd5d0, "bonsai_141bcd5d0", fn_141bcd5d0 as *const (), Throws::Discard), // sub_141bcd5d0
    Replacement::draft_throws(0x141bcdab0, "bonsai_141bcdab0", fn_141bcdab0 as *const (), Throws::Caught), // sub_141bcdab0
    Replacement::draft_throws(0x141bcdc10, "bonsai_141bcdc10", fn_141bcdc10 as *const (), Throws::Discard), // bonsai::AvoidanceTypeService::bonsai::AvoidanceTypeService::vfunc_2
    Replacement::draft_throws(0x141bcdcd0, "bonsai_141bcdcd0", fn_141bcdcd0 as *const (), Throws::Discard), // bonsai::AvoidanceTypeService::bonsai::AvoidanceTypeService::vfunc_6
    Replacement::draft(0x141bce430, "bonsai_141bce430", fn_141bce430 as *const ()), // bonsai::AvoidanceConfigScopeDecoratorInstance::bonsai::AvoidanceConfigScopeDecoratorInstance::vfunc_5
    Replacement::draft(0x141bce520, "bonsai_141bce520", fn_141bce520 as *const ()), // bonsai::AvoidanceConfigScopeDecoratorInstance::bonsai::AvoidanceConfigScopeDecoratorInstance::vfunc_4
    Replacement::draft(0x141bceb50, "bonsai_141bceb50", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::AvoidanceConfigScopeDecoratorInstance::ClassTypeInfoImplementation::bonsai::AvoidanceConfigScopeDecoratorInstanc
    Replacement::draft(0x141bced30, "bonsai_141bced30", fn_141bced30 as *const ()), // bonsai::AnimationModeServiceInstance::bonsai::AnimationModeServiceInstance::vfunc_6
    Replacement::draft(0x141bced60, "bonsai_141bced60", fn_141bced60 as *const ()), // bonsai::AnimationModeServiceInstance::bonsai::AnimationModeServiceInstance::vfunc_2
    Replacement::draft(0x141bced80, "bonsai_141bced80", fn_141bced80 as *const ()), // bonsai::AnimationModeServiceInstance::bonsai::AnimationModeServiceInstance::vfunc_1
    Replacement::draft(0x141bcf320, "bonsai_141bcf320", fn_141bcf320 as *const ()), // bonsai::AnimationModeServiceInstance::ClassTypeInfoImplementation::bonsai::AnimationModeServiceInstance::ClassTypeInfoIm
    Replacement::draft(0x141bcf380, "bonsai_141bcf380", fn_141bcf380 as *const ()), // bonsai::AnimationModeServiceInstance::ClassTypeInfoImplementation::bonsai::AnimationModeServiceInstance::ClassTypeInfoIm
    Replacement::draft(0x141bcf4f0, "bonsai_141bcf4f0", fn_141bcf4f0 as *const ()), // sub_141bcf4f0
    Replacement::draft(0x141bcf560, "bonsai_141bcf560", fn_141bcf560 as *const ()), // bonsai::AnimationControlParamDecorator::bonsai::AnimationControlParamDecorator::vfunc_5
    Replacement::draft(0x141bcf620, "bonsai_141bcf620", fn_141bcf620 as *const ()), // bonsai::AnimationControlParamDecorator::bonsai::AnimationControlParamDecorator::vfunc_4
    Replacement::draft(0x141bcfc40, "bonsai_141bcfc40", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::AnimationControlParamDecorator::ClassTypeInfoImplementation::bonsai::AnimationControlParamDecorator::ClassTypeIn
    Replacement::draft(0x141bcfe20, "bonsai_141bcfe20", tpl_38136::<0xb4, 0xb0, 0xac> as *const ()), // sub_141bcfe20
    Replacement::draft(0x141bcfe90, "bonsai_141bcfe90", fn_141bcfe90 as *const ()), // sub_141bcfe90
    Replacement::draft(0x141bcfed0, "bonsai_141bcfed0", fn_141bcfed0 as *const ()), // bonsai::AnimatedMoveToProgram::bonsai::AnimatedMoveToProgram::vfunc_15
    Replacement::draft(0x141bd0150, "bonsai_141bd0150", fn_141bd0150 as *const ()), // bonsai::AnimatedMoveToProgram::bonsai::AnimatedMoveToProgram::vfunc_3
    Replacement::draft(0x141bd01b0, "bonsai_141bd01b0", tpl_38429::<0xc0, 0x88, 0x24, 0x80, 0x20, 0x98, 0xa0> as *const ()), // bonsai::AnimatedMoveToProgram::bonsai::AnimatedMoveToProgram::vfunc_6
    Replacement::draft(0x141bd0240, "bonsai_141bd0240", tpl_38429::<0xc0, 0x88, 0x24, 0x80, 0x20, 0x98, 0xa0> as *const ()), // bonsai::AnimatedMoveToProgram::bonsai::AnimatedMoveToProgram::vfunc_5
    Replacement::draft(0x141bd02d0, "bonsai_141bd02d0", tpl_38430::<0xb4, 0x98, 0xa0, 0x141bd0360> as *const ()), // bonsai::AnimatedMoveToProgram::bonsai::AnimatedMoveToProgram::vfunc_1
    Replacement::draft(0x141bd0360, "bonsai_141bd0360", fn_141bd0360 as *const ()), // sub_141bd0360
    Replacement::draft(0x141bd06e0, "bonsai_141bd06e0", tpl_38431::<0x141ba4aa0, 0x143240540, 0x1430f6b98, 0x40, 0x8, 0x48, 0xc, 0x4c, 0x10, 0x50, 0x14, 0x54, 0x14323f990, 0x1431f8560> as *const ()), // sub_141bd06e0
    Replacement::draft(0x141bd07a0, "bonsai_141bd07a0", tpl_38136::<0xb4, 0xb0, 0xac> as *const ()), // sub_141bd07a0
    Replacement::draft(0x141bd0820, "bonsai_141bd0820", tpl_37771::<0x98, 0xa0> as *const ()), // bonsai::AnimatedMovementProgram::bonsai::AnimatedMovementProgram::vfunc_15
    Replacement::draft(0x141bd0880, "bonsai_141bd0880", tpl_37773::<0x98, 0xa0> as *const ()), // bonsai::AnimatedMovementProgram::bonsai::AnimatedMovementProgram::vfunc_6
    Replacement::draft(0x141bd08e0, "bonsai_141bd08e0", tpl_37773::<0x98, 0xa0> as *const ()), // bonsai::AnimatedMovementProgram::bonsai::AnimatedMovementProgram::vfunc_5
    Replacement::draft(0x141bd0940, "bonsai_141bd0940", tpl_38430::<0xb4, 0x98, 0xa0, 0x141bd09d0> as *const ()), // bonsai::AnimatedMovementProgram::bonsai::AnimatedMovementProgram::vfunc_1
    Replacement::draft(0x141bd09d0, "bonsai_141bd09d0", fn_141bd09d0 as *const ()), // sub_141bd09d0
    Replacement::draft(0x141bd0aa0, "bonsai_141bd0aa0", tpl_38431::<0x141ba4aa0, 0x1432405f0, 0x1430f6b98, 0x40, 0x8, 0x48, 0xc, 0x4c, 0x10, 0x50, 0x14, 0x54, 0x143231900, 0x1431f8560> as *const ()), // sub_141bd0aa0
    Replacement::draft_throws(0x141bd0b60, "bonsai_141bd0b60", fn_141bd0b60 as *const (), Throws::Discard), // bonsai::AngleCompareConditionDecoratorInstance::bonsai::AngleCompareConditionDecoratorInstance::vfunc_34
    Replacement::draft_throws(0x141bd0cf0, "bonsai_141bd0cf0", tpl_38004::<0x141b72aa0, 0x110, 0x18, 0x8> as *const (), Throws::Discard), // bonsai::AngleCompareConditionDecoratorInstance::bonsai::AngleCompareConditionDecoratorInstance::vfunc_1
    Replacement::draft(0x141bd16a0, "unknown_141bd16a0", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ca78cc, 0x158, 0x14020e8f0> as *const ()), // sub_141bd16a0
    Replacement::draft(0x141bd1790, "unknown_141bd1790", tpl_496 as *const ()), // sub_141bd1790
    Replacement::draft(0x141bd17a0, "unknown_141bd17a0", fn_141bd17a0 as *const ()), // sub_141bd17a0
    Replacement::draft(0x141bd19a0, "unknown_141bd19a0", tpl_496 as *const ()), // sub_141bd19a0
    Replacement::draft_throws(0x141bd19b0, "unknown_141bd19b0", tpl_38434::<0x1427c8a40, 0x1410d6ca0, 0x14069ade0, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x14069b0c0, 0x143823788> as *const (), Throws::Discard), // sub_141bd19b0
    Replacement::draft(0x141bd1c30, "unknown_141bd1c30", tpl_34318 as *const ()), // sub_141bd1c30
    Replacement::draft(0x141bd1c50, "unknown_141bd1c50", tpl_34318 as *const ()), // sub_141bd1c50
    Replacement::draft(0x141bd1c70, "unknown_141bd1c70", tpl_34318 as *const ()), // sub_141bd1c70
    Replacement::draft(0x141bd1c90, "unknown_141bd1c90", tpl_34318 as *const ()), // sub_141bd1c90
    Replacement::draft(0x141bd1cb0, "unknown_141bd1cb0", tpl_34318 as *const ()), // sub_141bd1cb0
    Replacement::draft(0x141bd1cd0, "unknown_141bd1cd0", tpl_34319::<0x143c8f6d8, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd1cd0
    Replacement::draft(0x141bd1d30, "unknown_141bd1d30", tpl_34319::<0x143c93358, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd1d30
    Replacement::draft(0x141bd1d90, "unknown_141bd1d90", tpl_34319::<0x143c933f0, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd1d90
    Replacement::draft(0x141bd1df0, "unknown_141bd1df0", tpl_34319::<0x143c92bcc, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd1df0
    Replacement::draft(0x141bd1e50, "unknown_141bd1e50", tpl_34319::<0x143ca7870, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd1e50
    Replacement::draft(0x141bd1eb0, "unknown_141bd1eb0", tpl_34318 as *const ()), // sub_141bd1eb0
    Replacement::draft(0x141bd1ed0, "unknown_141bd1ed0", tpl_34318 as *const ()), // sub_141bd1ed0
    Replacement::draft(0x141bd1ef0, "unknown_141bd1ef0", tpl_34318 as *const ()), // sub_141bd1ef0
    Replacement::draft(0x141bd1f10, "unknown_141bd1f10", tpl_34318 as *const ()), // sub_141bd1f10
    Replacement::draft(0x141bd1f30, "unknown_141bd1f30", tpl_34319::<0x143c8d120, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd1f30
    Replacement::draft(0x141bd1f90, "unknown_141bd1f90", tpl_34319::<0x143c8d074, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd1f90
    Replacement::draft(0x141bd1ff0, "unknown_141bd1ff0", tpl_34319::<0x143ca7610, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd1ff0
    Replacement::draft(0x141bd2050, "unknown_141bd2050", tpl_34319::<0x143c8ff58, 0x1806c, 0x142489ee0, 0xc03> as *const ()), // sub_141bd2050
    Replacement::draft(0x141bd20b0, "unknown_141bd20b0", tpl_6549::<0x143426abc, 0x8, 0x14069c360> as *const ()), // sub_141bd20b0
    Replacement::draft(0x141bd2110, "unknown_141bd2110", tpl_496 as *const ()), // sub_141bd2110
    Replacement::draft(0x141bd2120, "unknown_141bd2120", tpl_496 as *const ()), // sub_141bd2120
    Replacement::draft(0x141bd2130, "unknown_141bd2130", tpl_496 as *const ()), // sub_141bd2130
    Replacement::draft(0x141bd2140, "unknown_141bd2140", tpl_361 as *const ()), // sub_141bd2140
    Replacement::draft_throws(0x141bd2150, "unknown_141bd2150", fn_141bd2150 as *const (), Throws::Discard), // sub_141bd2150
    Replacement::draft_throws(0x141bd2490, "unknown_141bd2490", fn_141bd2490 as *const (), Throws::Discard), // sub_141bd2490
    Replacement::draft(0x141bd2760, "unknown_141bd2760", tpl_361 as *const ()), // sub_141bd2760
    Replacement::draft(0x141bd2770, "unknown_141bd2770", fn_141bd2770 as *const ()), // sub_141bd2770
    Replacement::draft(0x141bd2790, "unknown_141bd2790", tpl_38436 as *const ()), // sub_141bd2790
    Replacement::draft(0x141bd27a0, "unknown_141bd27a0", fn_141bd27a0 as *const ()), // sub_141bd27a0
    Replacement::draft_throws(0x141bd27e0, "unknown_141bd27e0", fn_141bd27e0 as *const (), Throws::Discard), // sub_141bd27e0
    Replacement::draft(0x141bd2ac0, "std_141bd2ac0", tpl_20903::<0x143241f30, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<bonsai::AirNavigationBot::checkIfCapsuleFits::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x141bd2af0, "std_141bd2af0", tpl_20903::<0x143241f30, 0x8, 0x18> as *const ()), // std::_Func_impl_no_alloc<bonsai::AirNavigationBot::checkIfCapsuleFits::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x141bd2b30, "std_141bd2b30", tpl_1::<0x14388e710> as *const ()), // std::_Func_impl_no_alloc<bonsai::AirNavigationBot::checkIfCapsuleFits::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x141bd2b40, "std_141bd2b40", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<bonsai::AirNavigationBot::checkIfCapsuleFits::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x141bd2b50, "std_141bd2b50", tpl_20901::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<bonsai::AirNavigationBot::checkIfCapsuleFits::<lambda_1>,bool,const coregame::PhysicsSweepHit&>
    Replacement::draft(0x141bd2b90, "unknown_141bd2b90", fn_141bd2b90 as *const ()), // sub_141bd2b90
    Replacement::draft(0x141bd2bb0, "unknown_141bd2bb0", tpl_34998::<0x8, 0x10> as *const ()), // sub_141bd2bb0
    Replacement::draft(0x141bd2bd0, "unknown_141bd2bd0", fn_141bd2bd0 as *const ()), // sub_141bd2bd0
    Replacement::draft(0x141bd2d00, "unknown_141bd2d00", fn_141bd2d00 as *const ()), // sub_141bd2d00
    Replacement::draft(0x141bd2f50, "unknown_141bd2f50", fn_141bd2f50 as *const ()), // sub_141bd2f50
    Replacement::draft(0x141bd2fd0, "unknown_141bd2fd0", fn_141bd2fd0 as *const ()), // sub_141bd2fd0
    Replacement::draft(0x141bd3000, "unknown_141bd3000", fn_141bd3000 as *const ()), // sub_141bd3000
    Replacement::draft(0x141bd3050, "unknown_141bd3050", fn_141bd3050 as *const ()), // sub_141bd3050
    Replacement::draft(0x141bd3140, "unknown_141bd3140", fn_141bd3140 as *const ()), // sub_141bd3140
    Replacement::draft(0x141bd32c0, "unknown_141bd32c0", fn_141bd32c0 as *const ()), // sub_141bd32c0
    Replacement::draft(0x141bd34e0, "unknown_141bd34e0", fn_141bd34e0 as *const ()), // sub_141bd34e0
    Replacement::draft(0x141bd3520, "unknown_141bd3520", tpl_38157::<0x14322fa50, 0x18, 0x28, 0x30, 0x84, 0x80, 0x8c, 0x1401de830, 0x10> as *const ()), // sub_141bd3520
    Replacement::draft(0x141bd3580, "bonsai_141bd3580", tpl_37771::<0x88, 0x90> as *const ()), // bonsai::AirMoveProgram::bonsai::AirMoveProgram::vfunc_15
    Replacement::draft(0x141bd35e0, "bonsai_141bd35e0", fn_141bd35e0 as *const ()), // sub_141bd35e0
    Replacement::draft(0x141bd3770, "bonsai_141bd3770", tpl_37773::<0x88, 0x90> as *const ()), // bonsai::AirMoveProgram::bonsai::AirMoveProgram::vfunc_6
    Replacement::draft(0x141bd37d0, "bonsai_141bd37d0", tpl_37773::<0x88, 0x90> as *const ()), // bonsai::AirMoveProgram::bonsai::AirMoveProgram::vfunc_5
    Replacement::draft(0x141bd3830, "bonsai_141bd3830", tpl_37774::<0x88, 0x90, 0x141bd35e0> as *const ()), // bonsai::AirMoveProgram::bonsai::AirMoveProgram::vfunc_1
    Replacement::draft(0x141bd38b0, "bonsai_141bd38b0", tpl_37776::<0x1431f8560, 0x80, 0x1430f6b98, 0x40, 0x1432316d0> as *const ()), // sub_141bd38b0
    Replacement::draft(0x141bd38e0, "bonsai_141bd38e0", fn_141bd38e0 as *const ()), // sub_141bd38e0
    Replacement::draft_throws(0x141bd3980, "bonsai_141bd3980", fn_141bd3980 as *const (), Throws::Caught), // bonsai::AimServiceInstance::bonsai::AimServiceInstance::vfunc_6
    Replacement::draft(0x141bd3b10, "bonsai_141bd3b10", fn_141bd3b10 as *const ()), // bonsai::AimServiceInstance::bonsai::AimServiceInstance::vfunc_2
    Replacement::draft_throws(0x141bd3b40, "bonsai_141bd3b40", fn_141bd3b40 as *const (), Throws::Caught), // bonsai::AimServiceInstance::bonsai::AimServiceInstance::vfunc_1
    Replacement::draft(0x141bd3c40, "bonsai_141bd3c40", fn_141bd3c40 as *const ()), // sub_141bd3c40
    Replacement::draft(0x141bd41d0, "bonsai_141bd41d0", fn_141bd41d0 as *const ()), // bonsai::AimServiceInstance::ClassTypeInfoImplementation::bonsai::AimServiceInstance::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141bd4230, "bonsai_141bd4230", fn_141bd4230 as *const ()), // bonsai::AimServiceInstance::ClassTypeInfoImplementation::bonsai::AimServiceInstance::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141bd43a0, "bonsai_141bd43a0", fn_141bd43a0 as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_17
    Replacement::draft(0x141bd43d0, "bonsai_141bd43d0", tpl_37771::<0x98, 0xa0> as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_15
    Replacement::draft(0x141bd4430, "bonsai_141bd4430", fn_141bd4430 as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_14
    Replacement::draft(0x141bd4460, "bonsai_141bd4460", fn_141bd4460 as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_13
    Replacement::draft(0x141bd4490, "bonsai_141bd4490", tpl_38459::<0x98, 0xa0, 0xc0, 0x48, 0x84> as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_6
    Replacement::draft(0x141bd4510, "bonsai_141bd4510", tpl_38459::<0x98, 0xa0, 0xc0, 0x48, 0x84> as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_5
    Replacement::draft(0x141bd4590, "bonsai_141bd4590", fn_141bd4590 as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_4
    Replacement::draft(0x141bd45f0, "bonsai_141bd45f0", fn_141bd45f0 as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_3
    Replacement::draft(0x141bd4610, "bonsai_141bd4610", fn_141bd4610 as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_2
    Replacement::draft(0x141bd4680, "bonsai_141bd4680", fn_141bd4680 as *const ()), // bonsai::AimProgram::bonsai::AimProgram::vfunc_1
    Replacement::draft(0x141bd4720, "bonsai_141bd4720", fn_141bd4720 as *const ()), // sub_141bd4720
    Replacement::draft(0x141bd47b0, "bonsai_141bd47b0", fn_141bd47b0 as *const ()), // sub_141bd47b0
    Replacement::draft(0x141bd47f0, "bonsai_141bd47f0", fn_141bd47f0 as *const ()), // sub_141bd47f0
    Replacement::draft(0x141bd48a0, "kaim_141bd48a0", fn_141bd48a0 as *const ()), // sub_141bd48a0
    Replacement::draft(0x141bd4a60, "unknown_141bd4a60", fn_141bd4a60 as *const ()), // sub_141bd4a60
    Replacement::draft_throws(0x141bd4b40, "bonsai_141bd4b40", fn_141bd4b40 as *const (), Throws::Discard), // sub_141bd4b40
    Replacement::draft(0x141bd4dd0, "std_141bd4dd0", fn_141bd4dd0 as *const ()), // sub_141bd4dd0
    Replacement::draft(0x141bd4f20, "unknown_141bd4f20", fn_141bd4f20 as *const ()), // sub_141bd4f20
    Replacement::draft(0x141bd5250, "unknown_141bd5250", tpl_38114::<0xb0, 0xc8, 0x18, 0x20, 0x30, 0x8, 0x10, 0x28, 0x1a> as *const ()), // sub_141bd5250
    Replacement::draft(0x141bd52e0, "bonsai_141bd52e0", fn_141bd52e0 as *const ()), // bonsai::AcquireTicketDecoratorInstance::bonsai::AcquireTicketDecoratorInstance::vfunc_1
    Replacement::draft_throws(0x141bd5380, "bonsai_141bd5380", fn_141bd5380 as *const (), Throws::Discard), // bonsai::AcquireTicketDecoratorInstance::bonsai::AcquireTicketDecoratorInstance::vfunc_9
    Replacement::draft_throws(0x141bd53e0, "bonsai_141bd53e0", fn_141bd53e0 as *const (), Throws::Discard), // bonsai::AcquireTicketDecoratorInstance::bonsai::AcquireTicketDecoratorInstance::vfunc_5
    Replacement::draft_throws(0x141bd5480, "bonsai_141bd5480", fn_141bd5480 as *const (), Throws::Discard), // bonsai::AcquireTicketDecoratorInstance::bonsai::AcquireTicketDecoratorInstance::vfunc_4
    Replacement::draft(0x141bd5a80, "bonsai_141bd5a80", tpl_35037::<0x8, 0x10, 0x18, 0x20, 0x30> as *const ()), // bonsai::AcquireTicketDecoratorInstance::ClassTypeInfoImplementation::bonsai::AcquireTicketDecoratorInstance::ClassTypeIn
    Replacement::draft_throws(0x141bd5c60, "bonsai_141bd5c60", tpl_37607::<0xc0, 0xa8, 0x78, 0x141bd6940, 0x3b, 0x84, 0x8, 0x85, 0x39, 0x18, 0x1427cd5b0, 0x60, 0x140241da0> as *const (), Throws::Discard), // sub_141bd5c60
    Replacement::draft_throws(0x141bd5d00, "bonsai_141bd5d00", tpl_37608::<0xc0, 0xa8, 0x78, 0x141bd6940, 0x84, 0x5, 0x4, 0x39, 0x85, 0x18, 0x1427cd5b0, 0x60, 0x140241da0> as *const (), Throws::Discard), // sub_141bd5d00
    Replacement::draft(0x141bd5dd0, "bonsai_141bd5dd0", fn_141bd5dd0 as *const ()), // bonsai::AbilityProgram::bonsai::AbilityProgram::vfunc_16
    Replacement::draft(0x141bd5e40, "bonsai_141bd5e40", tpl_37610::<0xa0, 0xa8> as *const ()), // bonsai::AbilityProgram::bonsai::AbilityProgram::vfunc_15
    Replacement::draft_throws(0x141bd5eb0, "bonsai_141bd5eb0", fn_141bd5eb0 as *const (), Throws::Discard), // bonsai::AbilityProgram::bonsai::AbilityProgram::vfunc_7
    Replacement::draft_throws(0x141bd5f30, "bonsai_141bd5f30", tpl_37612::<0xa0, 0xa8, 0x141bd5c60> as *const (), Throws::Discard), // bonsai::AbilityProgram::bonsai::AbilityProgram::vfunc_6
    Replacement::draft_throws(0x141bd5fa0, "bonsai_141bd5fa0", tpl_37612::<0xa0, 0xa8, 0x141bd5c60> as *const (), Throws::Discard), // bonsai::AbilityProgram::bonsai::AbilityProgram::vfunc_5
    Replacement::draft_throws(0x141bd6010, "bonsai_141bd6010", tpl_37613::<0xa0, 0xa8, 0x141bd6090> as *const (), Throws::Discard), // bonsai::AbilityProgram::bonsai::AbilityProgram::vfunc_1
    Replacement::draft_throws(0x141bd6090, "unknown_141bd6090", fn_141bd6090 as *const (), Throws::Discard), // sub_141bd6090
    Replacement::draft(0x141bd6260, "unknown_141bd6260", tpl_37615::<0x141ba4aa0, 0x38, 0x143242728, 0x3c, 0x1430f6b98, 0x44, 0x48, 0x50, 0x58, 0x8, 0x60, 0xc, 0x1431ea988, 0x1431f8560> as *const ()), // sub_141bd6260
    Replacement::draft(0x141bd6330, "unknown_141bd6330", tpl_5006::<0x10, 0x8, 0x58, 0x60, 0x143ca7874, 0x158, 0x14020e8f0> as *const ()), // sub_141bd6330
    Replacement::draft(0x141bd6420, "unknown_141bd6420", tpl_496 as *const ()), // sub_141bd6420
    Replacement::draft_throws(0x141bd6430, "unknown_141bd6430", fn_141bd6430 as *const (), Throws::Caught), // sub_141bd6430
    Replacement::draft(0x141bd6670, "unknown_141bd6670", tpl_496 as *const ()), // sub_141bd6670
    Replacement::draft_throws(0x141bd6680, "unknown_141bd6680", fn_141bd6680 as *const (), Throws::Discard), // sub_141bd6680
    Replacement::draft(0x141bd68d0, "unknown_141bd68d0", fn_141bd68d0 as *const ()), // sub_141bd68d0
    Replacement::draft_throws(0x141bd6920, "unknown_141bd6920", fn_141bd6920 as *const (), Throws::Discard), // sub_141bd6920
    Replacement::draft_throws(0x141bd6940, "std_141bd6940", fn_141bd6940 as *const (), Throws::Discard), // sub_141bd6940
    Replacement::draft_throws(0x141bd69d0, "unknown_141bd69d0", fn_141bd69d0 as *const (), Throws::Discard), // sub_141bd69d0
    Replacement::draft_throws(0x141bd6a60, "unknown_141bd6a60", tpl_38434::<0x1427ce0a0, 0x1410d6ca0, 0x1406a0310, 0x143823778, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1406a0660, 0x143823788> as *const (), Throws::Discard), // sub_141bd6a60
    Replacement::draft(0x141bd6ce0, "unknown_141bd6ce0", tpl_38484::<0x142442de0> as *const ()), // sub_141bd6ce0
    Replacement::draft(0x141bd71d0, "content_141bd71d0", tpl_38485::<0x142442de0> as *const ()), // content::FiniteStateMachineData::ClassTypeInfoImplementation::content::FiniteStateMachineData::ClassTypeInfoImplementati
    Replacement::draft(0x141bd7240, "content_141bd7240", tpl_38486::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x142442de0, 0x142442c80> as *const ()), // content::FiniteStateMachineData::ClassTypeInfoImplementation::content::FiniteStateMachineData::ClassTypeInfoImplementati
    Replacement::draft(0x141bd72d0, "content_141bd72d0", tpl_38487::<0x142442de0> as *const ()), // content::FiniteStateMachineData::ClassTypeInfoImplementation::content::FiniteStateMachineData::ClassTypeInfoImplementati
    Replacement::draft(0x141bd73c0, "content_141bd73c0", tpl_38484::<0x142442de0> as *const ()), // sub_141bd73c0
    Replacement::draft(0x141bd78b0, "content_141bd78b0", tpl_38485::<0x142442de0> as *const ()), // content::EQSQueryData::ClassTypeInfoImplementation::content::EQSQueryData::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141bd7920, "content_141bd7920", tpl_38486::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x142442de0, 0x142442c80> as *const ()), // content::EQSQueryData::ClassTypeInfoImplementation::content::EQSQueryData::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141bd79b0, "content_141bd79b0", tpl_38487::<0x142442de0> as *const ()), // content::EQSQueryData::ClassTypeInfoImplementation::content::EQSQueryData::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141bd8670, "contentbonsai_141bd8670", fn_141bd8670 as *const ()), // contentbonsai::NavigationAvoidanceConfiguration::ClassTypeInfoImplementation::contentbonsai::NavigationAvoidanceConfigur
    Replacement::draft(0x141bd86d0, "contentbonsai_141bd86d0", fn_141bd86d0 as *const ()), // contentbonsai::NavigationAvoidanceConfiguration::ClassTypeInfoImplementation::contentbonsai::NavigationAvoidanceConfigur
    Replacement::draft(0x141bd8860, "contentbonsai_141bd8860", fn_141bd8860 as *const ()), // contentbonsai::NavigationLinkComponent::contentbonsai::NavigationLinkComponent::vfunc_24
    Replacement::draft(0x141bd8a20, "contentbonsai_141bd8a20", fn_141bd8a20 as *const ()), // contentbonsai::NavigationLinkComponent::ClassTypeInfoImplementation::contentbonsai::NavigationLinkComponent::ClassTypeIn
    Replacement::draft(0x141bd8e70, "contentbonsai_141bd8e70", tpl_38492::<0x8, 0x10, 0x1404b9990, 0x30, 0x50, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x88> as *const ()), // contentbonsai::NavigationLinkComponent::ClassTypeInfoImplementation::contentbonsai::NavigationLinkComponent::ClassTypeIn
    Replacement::draft(0x141bd8f40, "contentbonsai_141bd8f40", tpl_30707::<0x1406d6720> as *const ()), // contentbonsai::NavigationLinkComponent::ClassTypeInfoImplementation::contentbonsai::NavigationLinkComponent::ClassTypeIn
    Replacement::draft(0x141bd9320, "contentbonsai_141bd9320", tpl_38493::<0x8, 0x18> as *const ()), // contentbonsai::NavmeshVolumeBoxComponent::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeBoxComponent::ClassTy
    Replacement::draft(0x141bd94e0, "contentbonsai_141bd94e0", tpl_38494::<0x8, 0x18> as *const ()), // contentbonsai::NavmeshVolumeBoxComponent::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeBoxComponent::ClassTy
    Replacement::draft(0x141bd9530, "contentbonsai_141bd9530", fn_141bd9530 as *const ()), // contentbonsai::NavmeshVolumeBoxComponent::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeBoxComponent::ClassTy
    Replacement::draft(0x141bd9c20, "contentbonsai_141bd9c20", fn_141bd9c20 as *const ()), // contentbonsai::NavmeshVolumeComponent::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeComponent::ClassTypeInfo
    Replacement::draft(0x141bd9ca0, "contentbonsai_141bd9ca0", fn_141bd9ca0 as *const ()), // contentbonsai::NavmeshVolumeComponent::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeComponent::ClassTypeInfo
    Replacement::draft(0x141bda090, "contentbonsai_141bda090", tpl_38498::<0x8, 0x10, 0x18, 0x20, 0x24, 0x2c> as *const ()), // contentbonsai::NavmeshVolumeBox::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeBox::ClassTypeInfoImplementati
    Replacement::draft(0x141bda0f0, "contentbonsai_141bda0f0", tpl_38499::<0x10, 0x8, 0x18, 0x24> as *const ()), // contentbonsai::NavmeshVolumeBox::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeBox::ClassTypeInfoImplementati
    Replacement::draft(0x141bda2e0, "contentbonsai_141bda2e0", tpl_38500::<0x20> as *const ()), // contentbonsai::NavmeshVolumeBox::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeBox::ClassTypeInfoImplementati
    Replacement::draft(0x141bda320, "contentbonsai_141bda320", fn_141bda320 as *const ()), // contentbonsai::NavmeshVolumeBox::ClassTypeInfoImplementation::contentbonsai::NavmeshVolumeBox::ClassTypeInfoImplementati
    Replacement::draft(0x141bda450, "content_141bda450", fn_141bda450 as *const ()), // content::WorkspotComponent::ClassTypeInfoImplementation::content::WorkspotComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141bda4b0, "content_141bda4b0", fn_141bda4b0 as *const ()), // content::WorkspotComponent::ClassTypeInfoImplementation::content::WorkspotComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141bda8f0, "content_141bda8f0", fn_141bda8f0 as *const ()), // content::WorkspotComponent::ClassTypeInfoImplementation::content::WorkspotComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141bda980, "content_141bda980", fn_141bda980 as *const ()), // content::WorkspotComponent::ClassTypeInfoImplementation::content::WorkspotComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141bdaf50, "content_141bdaf50", fn_141bdaf50 as *const ()), // content::WorkspotAreaComponent::ClassTypeInfoImplementation::content::WorkspotAreaComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141bdb040, "content_141bdb040", fn_141bdb040 as *const ()), // content::WorkspotAreaComponent::ClassTypeInfoImplementation::content::WorkspotAreaComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141bdb290, "content_141bdb290", fn_141bdb290 as *const ()), // content::VisualSensorComponent::content::VisualSensorComponent::vfunc_33
    Replacement::draft(0x141bdb5b0, "content_141bdb5b0", fn_141bdb5b0 as *const ()), // content::VisualSensorComponent::ClassTypeInfoImplementation::content::VisualSensorComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141bdb7a0, "content_141bdb7a0", tpl_38510::<0x8, 0x9, 0xc> as *const ()), // content::VisualSensorComponent::ClassTypeInfoImplementation::content::VisualSensorComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141bdb7f0, "content_141bdb7f0", fn_141bdb7f0 as *const ()), // content::VisualSensorComponent::ClassTypeInfoImplementation::content::VisualSensorComponent::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141bdd2d0, "content_141bdd2d0", fn_141bdd2d0 as *const (), Throws::Discard), // content::OverrideSpeedDecorator::ClassTypeInfoImplementation::content::OverrideSpeedDecorator::ClassTypeInfoImplementati
    Replacement::draft(0x141bdd360, "content_141bdd360", tpl_38513::<0x143246b58, 0x8, 0x10, 0x18, 0x78, 0x20, 0x23, 0x24, 0x28, 0x70, 0x80, 0x88> as *const ()), // content::OverrideSpeedDecorator::ClassTypeInfoImplementation::content::OverrideSpeedDecorator::ClassTypeInfoImplementati
    Replacement::draft(0x141bdd970, "content_141bdd970", fn_141bdd970 as *const ()), // content::NavmeshVolumeComponent::ClassTypeInfoImplementation::content::NavmeshVolumeComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141bdda30, "content_141bdda30", fn_141bdda30 as *const ()), // content::NavmeshVolumeComponent::ClassTypeInfoImplementation::content::NavmeshVolumeComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141bde6e0, "content_141bde6e0", fn_141bde6e0 as *const ()), // content::NavMeshModifierZoneComponent::ClassTypeInfoImplementation::content::NavMeshModifierZoneComponent::ClassTypeInfo
    Replacement::draft(0x141bde770, "content_141bde770", fn_141bde770 as *const ()), // content::NavMeshModifierZoneComponent::ClassTypeInfoImplementation::content::NavMeshModifierZoneComponent::ClassTypeInfo
    Replacement::draft(0x141bdffc0, "content_141bdffc0", tpl_38518::<0x14384e1c0, 0x143243c70, 0x1432448d8, 0x143244890, 0x1401de410> as *const ()), // content::NavigationLinkComponent::ClassTypeInfoImplementation::content::NavigationLinkComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141be0080, "content_141be0080", tpl_38518::<0x14384e1c0, 0x143243c70, 0x1432448d8, 0x143244890, 0x1401de410> as *const ()), // content::NavigationLinkComponent::ClassTypeInfoImplementation::content::NavigationLinkComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141be04b0, "content_141be04b0", tpl_38492::<0x8, 0x10, 0x1404b9990, 0x30, 0x50, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x88> as *const ()), // content::NavigationLinkComponent::ClassTypeInfoImplementation::content::NavigationLinkComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141be0580, "content_141be0580", fn_141be0580 as *const ()), // content::NavigationLinkComponent::ClassTypeInfoImplementation::content::NavigationLinkComponent::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141be0b60, "content_141be0b60", fn_141be0b60 as *const (), Throws::Discard), // content::NavigationAvoidanceConfigurationComponent::ClassTypeInfoImplementation::content::NavigationAvoidanceConfigurati
    Replacement::draft(0x141be0be0, "content_141be0be0", fn_141be0be0 as *const ()), // content::NavigationAvoidanceConfigurationComponent::ClassTypeInfoImplementation::content::NavigationAvoidanceConfigurati
    Replacement::draft(0x141be16f0, "content_141be16f0", tpl_38522::<0x8, 0xc, 0x10> as *const ()), // content::HazardComponent::ClassTypeInfoImplementation::content::HazardComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141be18c0, "content_141be18c0", tpl_38523::<0x8, 0xc, 0x10> as *const ()), // content::HazardComponent::ClassTypeInfoImplementation::content::HazardComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141be1d50, "content_141be1d50", fn_141be1d50 as *const ()), // content::BonsaiWaypointComponent::ClassTypeInfoImplementation::content::BonsaiWaypointComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141be1f60, "content_141be1f60", tpl_38525::<0x14315c430, 0x10, 0x18, 0x20, 0x28> as *const ()), // content::BonsaiWaypointComponent::ClassTypeInfoImplementation::content::BonsaiWaypointComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141be20d0, "content_141be20d0", tpl_38526::<0x143244cb8, 0x28, 0x143873c40, 0x143244cd8, 0x143244ca8, 0x1401de410> as *const ()), // content::BonsaiSmartObjectComponent::content::BonsaiSmartObjectComponent::vfunc_22
    Replacement::draft(0x141be2130, "content_141be2130", tpl_38526::<0x143244cb8, 0x28, 0x143873c68, 0x143244cd8, 0x143244ca8, 0x1401de410> as *const ()), // content::BonsaiSmartObjectComponent::content::BonsaiSmartObjectComponent::vfunc_23
    Replacement::draft(0x141be2190, "content_141be2190", tpl_38526::<0x143244cb8, 0x28, 0x143873c18, 0x143244cd8, 0x143244ca8, 0x1401de410> as *const ()), // content::BonsaiSmartObjectComponent::content::BonsaiSmartObjectComponent::vfunc_24
    Replacement::draft(0x141be2270, "content_141be2270", fn_141be2270 as *const ()), // content::BonsaiSmartObjectComponent::content::BonsaiSmartObjectComponent::vfunc_33
    Replacement::draft(0x141be2330, "content_141be2330", tpl_38528::<0x143873c40, 0x143244cb8, 0x143244cd8, 0x143244ca8, 0x1401de410> as *const ()), // content::BonsaiSmartObjectComponent::ClassTypeInfoImplementation::content::BonsaiSmartObjectComponent::ClassTypeInfoImpl
    Replacement::draft(0x141be23b0, "content_141be23b0", tpl_38528::<0x143873c68, 0x143244cb8, 0x143244cd8, 0x143244ca8, 0x1401de410> as *const ()), // content::BonsaiSmartObjectComponent::ClassTypeInfoImplementation::content::BonsaiSmartObjectComponent::ClassTypeInfoImpl
    Replacement::draft(0x141be2430, "content_141be2430", tpl_38528::<0x143873c18, 0x143244cb8, 0x143244cd8, 0x143244ca8, 0x1401de410> as *const ()), // content::BonsaiSmartObjectComponent::ClassTypeInfoImplementation::content::BonsaiSmartObjectComponent::ClassTypeInfoImpl
    Replacement::draft(0x141be26b0, "content_141be26b0", fn_141be26b0 as *const ()), // content::BonsaiSmartObjectComponent::ClassTypeInfoImplementation::content::BonsaiSmartObjectComponent::ClassTypeInfoImpl
    Replacement::draft(0x141be28d0, "content_141be28d0", tpl_38530::<0x8, 0x10, 0x18, 0x20, 0x28> as *const ()), // content::BonsaiSmartObjectComponent::ClassTypeInfoImplementation::content::BonsaiSmartObjectComponent::ClassTypeInfoImpl
    Replacement::draft(0x141be2930, "content_141be2930", fn_141be2930 as *const ()), // content::BonsaiSmartObjectComponent::ClassTypeInfoImplementation::content::BonsaiSmartObjectComponent::ClassTypeInfoImpl
    Replacement::draft(0x141be30b0, "content_141be30b0", tpl_38532::<0x14384e178, 0x143244d08, 0x1426cdcb0, 0x143244d40, 0x1401de410> as *const ()), // content::BonsaiRouteComponent::ClassTypeInfoImplementation::content::BonsaiRouteComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be3130, "content_141be3130", tpl_38532::<0x14384e178, 0x143244d08, 0x1426cdcb0, 0x143244d40, 0x1401de410> as *const ()), // content::BonsaiRouteComponent::ClassTypeInfoImplementation::content::BonsaiRouteComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be33b0, "content_141be33b0", fn_141be33b0 as *const ()), // content::BonsaiRouteComponent::ClassTypeInfoImplementation::content::BonsaiRouteComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be3590, "content_141be3590", tpl_38534::<0x8, 0x1406d70a0, 0x18> as *const ()), // content::BonsaiRouteComponent::ClassTypeInfoImplementation::content::BonsaiRouteComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be3600, "content_141be3600", tpl_38535::<0x14315c170, 0x8, 0x10, 0x18> as *const ()), // content::BonsaiRouteComponent::ClassTypeInfoImplementation::content::BonsaiRouteComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be3a00, "content_141be3a00", tpl_38536::<0x8, 0xc> as *const ()), // content::BodySensorComponent::ClassTypeInfoImplementation::content::BodySensorComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141be3bc0, "content_141be3bc0", tpl_38537::<0x8, 0xc> as *const ()), // content::BodySensorComponent::ClassTypeInfoImplementation::content::BodySensorComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141be3d70, "content_141be3d70", tpl_38538::<0x14384e1c0, 0x143243c70, 0x143243cc8, 0x143243ce0, 0x1401de410, 0x30, 0x14384e1d8, 0x1432435d8, 0x143243608, 0x143243640> as *const ()), // content::BodyControlComponent::ClassTypeInfoImplementation::content::BodyControlComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be3e70, "content_141be3e70", tpl_38538::<0x14384e1c0, 0x143243c70, 0x143243cc8, 0x143243ce0, 0x1401de410, 0x30, 0x14384e1d8, 0x1432435d8, 0x143243608, 0x143243640> as *const ()), // content::BodyControlComponent::ClassTypeInfoImplementation::content::BodyControlComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be42e0, "content_141be42e0", fn_141be42e0 as *const ()), // content::BodyControlComponent::ClassTypeInfoImplementation::content::BodyControlComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be4380, "content_141be4380", fn_141be4380 as *const ()), // content::BodyControlComponent::ClassTypeInfoImplementation::content::BodyControlComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141be4390, "content_141be4390", fn_141be4390 as *const ()), // sub_141be4390
    Replacement::draft(0x141be44f0, "content_141be44f0", tpl_37078::<0x143243d38, 0x143243d20, 0x143243cf8, 0x1406d7950> as *const ()), // content::BehaviorTreeComponent::ClassTypeInfoImplementation::content::BehaviorTreeComponent::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141be4c80, "content_141be4c80", tpl_838::<0x1406b4fe0> as *const (), Throws::Discard), // content::BonsaiScriptInterfaceComponent::content::BonsaiScriptInterfaceComponent::vfunc_36
    Replacement::draft(0x141be4e40, "content_141be4e40", tpl_38542::<0x8, 0xc, 0x10> as *const ()), // content::HazardComponent::content::HazardComponent::vfunc_33
    Replacement::draft(0x141be4f00, "content_141be4f00", tpl_38526::<0x143243c70, 0x8, 0x14384e1c0, 0x1432448d8, 0x143244890, 0x1401de410> as *const ()), // content::NavigationLinkComponent::content::NavigationLinkComponent::vfunc_22
    Replacement::draft(0x141be4f70, "content_141be4f70", tpl_38526::<0x143243c70, 0x8, 0x14384e1c0, 0x1432448d8, 0x143244890, 0x1401de410> as *const ()), // content::NavigationLinkComponent::content::NavigationLinkComponent::vfunc_24
    Replacement::draft(0x141be50d0, "content_141be50d0", tpl_38543::<0x143243c70, 0x34, 0x14384e1c0, 0x143243cc8, 0x143243ce0, 0x1401de410, 0x30, 0x14384e1d8, 0x1432435d8, 0x143243608, 0x143243640> as *const ()), // content::BodyControlComponent::content::BodyControlComponent::vfunc_22
    Replacement::draft(0x141be5180, "content_141be5180", tpl_38543::<0x143243c70, 0x34, 0x14384e1c0, 0x143243cc8, 0x143243ce0, 0x1401de410, 0x30, 0x14384e1d8, 0x1432435d8, 0x143243608, 0x143243640> as *const ()), // content::BodyControlComponent::content::BodyControlComponent::vfunc_24
    Replacement::draft(0x141be5900, "contentbonsai_141be5900", tpl_38544::<0x8, 0x18> as *const ()), // contentbonsai::NavmeshVolumeBoxComponent::contentbonsai::NavmeshVolumeBoxComponent::vfunc_33
    Replacement::draft(0x141be60f0, "content_141be60f0", tpl_38545::<0x8, 0xc> as *const ()), // content::QuadrupedBodyObstacleComponent::content::QuadrupedBodyObstacleComponent::vfunc_33
    Replacement::draft(0x141be6300, "content_141be6300", tpl_38546::<0x143243d20, 0x8, 0x143243d38, 0x143243cf8, 0x1406d7950> as *const ()), // content::BehaviorTreeComponent::content::BehaviorTreeComponent::vfunc_22
    Replacement::draft(0x141be6460, "content_141be6460", fn_141be6460 as *const ()), // content::WorkspotComponent::content::WorkspotComponent::vfunc_23
    Replacement::draft(0x141be64a0, "content_141be64a0", tpl_38548::<0x10, 0x1432440a8, 0x143244128, 0x1406db220> as *const ()), // content::WorkspotComponent::content::WorkspotComponent::vfunc_22
    Replacement::draft(0x141be6550, "content_141be6550", tpl_38549::<0x8, 0x10, 0x1404b9990> as *const ()), // content::BonsaiWaypointComponent::content::BonsaiWaypointComponent::vfunc_33
    Replacement::draft(0x141be66f0, "content_141be66f0", fn_141be66f0 as *const ()), // content::BonsaiRouteComponent::content::BonsaiRouteComponent::vfunc_33
    Replacement::draft(0x141be67d0, "content_141be67d0", tpl_38551::<0x143244d08, 0x18, 0x14384e178, 0x1426cdcb0, 0x143244d40, 0x1401de410> as *const ()), // content::BonsaiRouteComponent::content::BonsaiRouteComponent::vfunc_24
    Replacement::draft(0x141be6830, "content_141be6830", tpl_38551::<0x143244d08, 0x18, 0x14384e178, 0x1426cdcb0, 0x143244d40, 0x1401de410> as *const ()), // content::BonsaiRouteComponent::content::BonsaiRouteComponent::vfunc_23
    Replacement::draft(0x141be6a40, "content_141be6a40", fn_141be6a40 as *const ()), // content::BodySensorComponent::content::BodySensorComponent::vfunc_33
    Replacement::draft_throws(0x141be70b0, "content_141be70b0", fn_141be70b0 as *const (), Throws::Discard), // content::FiniteStateMachineStateNode::ClassTypeInfoImplementation::content::FiniteStateMachineStateNode::ClassTypeInfoIm
    Replacement::draft(0x141be7160, "content_141be7160", fn_141be7160 as *const ()), // content::FiniteStateMachineStateNode::ClassTypeInfoImplementation::content::FiniteStateMachineStateNode::ClassTypeInfoIm
    Replacement::draft_throws(0x141be7870, "content_141be7870", fn_141be7870 as *const (), Throws::Discard), // content::FiniteStateMachineRootNode::ClassTypeInfoImplementation::content::FiniteStateMachineRootNode::ClassTypeInfoImpl
    Replacement::draft(0x141be7910, "content_141be7910", fn_141be7910 as *const ()), // content::FiniteStateMachineRootNode::ClassTypeInfoImplementation::content::FiniteStateMachineRootNode::ClassTypeInfoImpl
    Replacement::draft(0x141be7a80, "content_141be7a80", tpl_38557::<0x8, 0x10, 0x18> as *const ()), // content::FiniteStateMachineNode::content::FiniteStateMachineNode::vfunc_1
    Replacement::draft(0x141be7af0, "content_141be7af0", tpl_38557::<0x8, 0x10, 0x28> as *const ()), // content::FiniteStateMachineNode::content::FiniteStateMachineNode::vfunc_3
    Replacement::draft_throws(0x141be7bd0, "content_141be7bd0", fn_141be7bd0 as *const (), Throws::Discard), // content::FiniteStateMachineNode::content::FiniteStateMachineNode::vfunc_12
    Replacement::draft(0x141be7c70, "content_141be7c70", tpl_38559::<0x8, 0x10, 0x18> as *const ()), // content::FiniteStateMachineNode::ClassTypeInfoImplementation::content::FiniteStateMachineNode::ClassTypeInfoImplementati
    Replacement::draft(0x141be7d20, "content_141be7d20", tpl_38559::<0x8, 0x10, 0x28> as *const ()), // content::FiniteStateMachineNode::ClassTypeInfoImplementation::content::FiniteStateMachineNode::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141be7fa0, "content_141be7fa0", fn_141be7fa0 as *const (), Throws::Discard), // content::FiniteStateMachineNode::ClassTypeInfoImplementation::content::FiniteStateMachineNode::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141be81a0, "content_141be81a0", tpl_38561::<0x8, 0x1406f2520> as *const (), Throws::Discard), // content::FiniteStateMachineNode::ClassTypeInfoImplementation::content::FiniteStateMachineNode::ClassTypeInfoImplementati
    Replacement::draft(0x141be8360, "content_141be8360", tpl_38557::<0x8, 0x10, 0x18> as *const ()), // content::FiniteStateMachineExitNode::content::FiniteStateMachineExitNode::vfunc_1
    Replacement::draft(0x141be83d0, "content_141be83d0", tpl_38557::<0x8, 0x10, 0x28> as *const ()), // content::FiniteStateMachineExitNode::content::FiniteStateMachineExitNode::vfunc_3
    Replacement::draft_throws(0x141be8550, "content_141be8550", fn_141be8550 as *const (), Throws::Discard), // content::FiniteStateMachineExitNode::content::FiniteStateMachineExitNode::vfunc_12
    Replacement::draft(0x141be8680, "content_141be8680", tpl_38559::<0x8, 0x10, 0x18> as *const ()), // content::FiniteStateMachineExitNode::ClassTypeInfoImplementation::content::FiniteStateMachineExitNode::ClassTypeInfoImpl
    Replacement::draft(0x141be8730, "content_141be8730", tpl_38559::<0x8, 0x10, 0x28> as *const ()), // content::FiniteStateMachineExitNode::ClassTypeInfoImplementation::content::FiniteStateMachineExitNode::ClassTypeInfoImpl
    Replacement::draft_throws(0x141be8a30, "content_141be8a30", fn_141be8a30 as *const (), Throws::Discard), // content::FiniteStateMachineExitNode::ClassTypeInfoImplementation::content::FiniteStateMachineExitNode::ClassTypeInfoImpl
    Replacement::draft_throws(0x141be8ca0, "content_141be8ca0", tpl_38561::<0x8, 0x1406f2520> as *const (), Throws::Discard), // content::FiniteStateMachineExitNode::ClassTypeInfoImplementation::content::FiniteStateMachineExitNode::ClassTypeInfoImpl
    Replacement::draft(0x141be8e60, "content_141be8e60", tpl_38557::<0x8, 0x10, 0x18> as *const ()), // content::FiniteStateMachineConnectionNode::content::FiniteStateMachineConnectionNode::vfunc_1
    Replacement::draft(0x141be8ed0, "content_141be8ed0", tpl_38557::<0x8, 0x10, 0x28> as *const ()), // content::FiniteStateMachineConnectionNode::content::FiniteStateMachineConnectionNode::vfunc_3
    Replacement::draft(0x141be9010, "content_141be9010", tpl_38559::<0x8, 0x10, 0x18> as *const ()), // content::FiniteStateMachineConnectionNode::ClassTypeInfoImplementation::content::FiniteStateMachineConnectionNode::Class
    Replacement::draft(0x141be90c0, "content_141be90c0", tpl_38559::<0x8, 0x10, 0x28> as *const ()), // content::FiniteStateMachineConnectionNode::ClassTypeInfoImplementation::content::FiniteStateMachineConnectionNode::Class
    Replacement::draft_throws(0x141be9510, "content_141be9510", fn_141be9510 as *const (), Throws::Discard), // content::FiniteStateMachineConnectionNode::ClassTypeInfoImplementation::content::FiniteStateMachineConnectionNode::Class
    Replacement::draft(0x141be95a0, "content_141be95a0", fn_141be95a0 as *const ()), // content::FiniteStateMachineConnectionNode::ClassTypeInfoImplementation::content::FiniteStateMachineConnectionNode::Class
    Replacement::draft(0x141be9d00, "content_141be9d00", tpl_38566::<0x14324b850, 0xc> as *const ()), // content::EQSTask::ClassTypeInfoImplementation::content::EQSTask::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft_throws(0x141bea470, "content_141bea470", fn_141bea470 as *const (), Throws::Discard), // content::EQSSphereGeneratorNode::ClassTypeInfoImplementation::content::EQSSphereGeneratorNode::ClassTypeInfoImplementati
    Replacement::draft(0x141bea520, "content_141bea520", fn_141bea520 as *const ()), // content::EQSSphereGeneratorNode::ClassTypeInfoImplementation::content::EQSSphereGeneratorNode::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141beaee0, "content_141beaee0", tpl_38569::<0x8, 0x140763e00, 0x18, 0x1407640a0> as *const (), Throws::Discard), // content::EQSScoreNode::ClassTypeInfoImplementation::content::EQSScoreNode::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141beb330, "content_141beb330", tpl_365::<0x1407573b0> as *const ()), // content::EQSRootNode::content::EQSRootNode::vfunc_16
    Replacement::draft_throws(0x141beb8e0, "content_141beb8e0", tpl_38569::<0x8, 0x140763e00, 0x18, 0x1407640a0> as *const (), Throws::Discard), // content::EQSRootNode::ClassTypeInfoImplementation::content::EQSRootNode::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141bec080, "content_141bec080", fn_141bec080 as *const ()), // content::EQSRaycastToTargetOnNavmeshTask::ClassTypeInfoImplementation::content::EQSRaycastToTargetOnNavmeshTask::ClassTy
    Replacement::draft(0x141bec120, "content_141bec120", tpl_38571::<0x14324b610, 0x78, 0xc, 0x20, 0x23, 0x24, 0x28, 0x70, 0x80, 0x88, 0x90, 0x68> as *const ()), // content::EQSRaycastToTargetOnNavmeshTask::ClassTypeInfoImplementation::content::EQSRaycastToTargetOnNavmeshTask::ClassTy
    Replacement::draft(0x141bec860, "content_141bec860", fn_141bec860 as *const ()), // content::EQSRandomEuclideanDistanceTask::ClassTypeInfoImplementation::content::EQSRandomEuclideanDistanceTask::ClassType
    Replacement::draft(0x141bec930, "content_141bec930", fn_141bec930 as *const ()), // content::EQSRandomEuclideanDistanceTask::ClassTypeInfoImplementation::content::EQSRandomEuclideanDistanceTask::ClassType
    Replacement::draft(0x141becad0, "content_141becad0", fn_141becad0 as *const ()), // content::EQSQueryService::content::EQSQueryService::vfunc_3
    Replacement::draft(0x141becb50, "content_141becb50", fn_141becb50 as *const ()), // content::EQSQueryService::content::EQSQueryService::vfunc_5
    Replacement::draft(0x141becc80, "content_141becc80", fn_141becc80 as *const ()), // content::EQSQueryService::ClassTypeInfoImplementation::content::EQSQueryService::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141becd50, "content_141becd50", fn_141becd50 as *const ()), // content::EQSQueryService::ClassTypeInfoImplementation::content::EQSQueryService::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141bed190, "content_141bed190", fn_141bed190 as *const ()), // content::EQSQueryService::ClassTypeInfoImplementation::content::EQSQueryService::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft_throws(0x141bed950, "content_141bed950", tpl_38569::<0x8, 0x140763e00, 0x18, 0x1407640a0> as *const (), Throws::Discard), // content::EQSNode::ClassTypeInfoImplementation::content::EQSNode::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141bee0f0, "content_141bee0f0", fn_141bee0f0 as *const ()), // content::EQSLineOfSightTask::ClassTypeInfoImplementation::content::EQSLineOfSightTask::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141bee1a0, "content_141bee1a0", fn_141bee1a0 as *const ()), // content::EQSLineOfSightTask::ClassTypeInfoImplementation::content::EQSLineOfSightTask::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141bee900, "content_141bee900", fn_141bee900 as *const ()), // content::EQSInvalidateCoverTask::ClassTypeInfoImplementation::content::EQSInvalidateCoverTask::ClassTypeInfoImplementati
    Replacement::draft(0x141bee990, "content_141bee990", tpl_38571::<0x14324e948, 0x78, 0xc, 0x20, 0x23, 0x24, 0x28, 0x70, 0x80, 0x88, 0x90, 0x68> as *const ()), // content::EQSInvalidateCoverTask::ClassTypeInfoImplementation::content::EQSInvalidateCoverTask::ClassTypeInfoImplementati
    Replacement::draft(0x141beeb30, "content_141beeb30", tpl_38582::<0x14075a4d0, 0x14384e2a0, 0x1426cdcb0, 0x1432505e8, 0x1401de410> as *const ()), // content::EQSHeightTask::content::EQSHeightTask::vfunc_3
    Replacement::draft(0x141beeba0, "content_141beeba0", tpl_38582::<0x14075a380, 0x14384e2a0, 0x1426cdcb0, 0x1432505e8, 0x1401de410> as *const ()), // content::EQSHeightTask::content::EQSHeightTask::vfunc_5
    Replacement::draft(0x141beece0, "content_141beece0", tpl_38583::<0x14075a4d0, 0x14384e2a0, 0x1426cdcb0, 0x1432505e8, 0x1401de410> as *const ()), // content::EQSHeightTask::ClassTypeInfoImplementation::content::EQSHeightTask::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141beedb0, "content_141beedb0", tpl_38583::<0x14075a380, 0x14384e2a0, 0x1426cdcb0, 0x1432505e8, 0x1401de410> as *const ()), // content::EQSHeightTask::ClassTypeInfoImplementation::content::EQSHeightTask::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141bef210, "content_141bef210", fn_141bef210 as *const ()), // content::EQSHeightTask::ClassTypeInfoImplementation::content::EQSHeightTask::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141bef2d0, "content_141bef2d0", fn_141bef2d0 as *const ()), // content::EQSHeightTask::ClassTypeInfoImplementation::content::EQSHeightTask::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141bef490, "content_141bef490", tpl_38586::<0x14075a400, 0x1434270b0, 0x1426cdcb0, 0x18, 0x143250378, 0x14020f270> as *const ()), // content::EQSHazardFieldTask::content::EQSHazardFieldTask::vfunc_4
    Replacement::draft(0x141bef620, "content_141bef620", tpl_38587::<0x14075a400, 0x1434270b0, 0x1426cdcb0, 0x18, 0x143250378, 0x14020f270> as *const ()), // content::EQSHazardFieldTask::ClassTypeInfoImplementation::content::EQSHazardFieldTask::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141befac0, "content_141befac0", fn_141befac0 as *const ()), // content::EQSHazardFieldTask::ClassTypeInfoImplementation::content::EQSHazardFieldTask::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141befb20, "content_141befb20", fn_141befb20 as *const ()), // content::EQSHazardFieldTask::ClassTypeInfoImplementation::content::EQSHazardFieldTask::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141bf0290, "content_141bf0290", fn_141bf0290 as *const (), Throws::Discard), // content::EQSGridPropagationGeneratorNode::ClassTypeInfoImplementation::content::EQSGridPropagationGeneratorNode::ClassTy
    Replacement::draft(0x141bf0360, "content_141bf0360", fn_141bf0360 as *const ()), // content::EQSGridPropagationGeneratorNode::ClassTypeInfoImplementation::content::EQSGridPropagationGeneratorNode::ClassTy
    Replacement::draft_throws(0x141bf0d30, "content_141bf0d30", tpl_38569::<0x8, 0x140763e00, 0x18, 0x1407640a0> as *const (), Throws::Discard), // content::EQSGeneratorBaseNode::ClassTypeInfoImplementation::content::EQSGeneratorBaseNode::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf14d0, "content_141bf14d0", fn_141bf14d0 as *const ()), // content::EQSGeneralPurposeTaskBase::ClassTypeInfoImplementation::content::EQSGeneralPurposeTaskBase::ClassTypeInfoImplem
    Replacement::draft(0x141bf1550, "content_141bf1550", tpl_38592::<0x14324f288, 0xc, 0x20> as *const ()), // content::EQSGeneralPurposeTaskBase::ClassTypeInfoImplementation::content::EQSGeneralPurposeTaskBase::ClassTypeInfoImplem
    Replacement::draft(0x141bf1c50, "content_141bf1c50", tpl_38593::<0x8, 0x18, 0xc, 0x10, 0x14240aa00, 0x60, 0x140636a20, 0x90, 0x94, 0x98, 0x9c> as *const ()), // content::EQSEuclideanPreferredDistanceTask::ClassTypeInfoImplementation::content::EQSEuclideanPreferredDistanceTask::Cla
    Replacement::draft(0x141bf1d20, "content_141bf1d20", tpl_38594::<0x70, 0xc, 0x18, 0x1b, 0x1c, 0x20, 0x68, 0x78, 0x80, 0x88, 0x60, 0x14324d168> as *const ()), // content::EQSEuclideanPreferredDistanceTask::ClassTypeInfoImplementation::content::EQSEuclideanPreferredDistanceTask::Cla
    Replacement::draft(0x141bf2460, "content_141bf2460", tpl_38593::<0x8, 0x18, 0xc, 0x10, 0x14240aa00, 0x60, 0x140636a20, 0x90, 0x94, 0x98, 0x9c> as *const ()), // content::EQSEuclideanLinearDistanceTask::ClassTypeInfoImplementation::content::EQSEuclideanLinearDistanceTask::ClassType
    Replacement::draft(0x141bf2530, "content_141bf2530", tpl_38594::<0x70, 0xc, 0x18, 0x1b, 0x1c, 0x20, 0x68, 0x78, 0x80, 0x88, 0x60, 0x14324e428> as *const ()), // content::EQSEuclideanLinearDistanceTask::ClassTypeInfoImplementation::content::EQSEuclideanLinearDistanceTask::ClassType
    Replacement::draft(0x141bf2c70, "content_141bf2c70", fn_141bf2c70 as *const ()), // content::EQSEuclideanDistanceTask::ClassTypeInfoImplementation::content::EQSEuclideanDistanceTask::ClassTypeInfoImplemen
    Replacement::draft(0x141bf2cf0, "content_141bf2cf0", fn_141bf2cf0 as *const ()), // content::EQSEuclideanDistanceTask::ClassTypeInfoImplementation::content::EQSEuclideanDistanceTask::ClassTypeInfoImplemen
    Replacement::draft(0x141bf3430, "content_141bf3430", fn_141bf3430 as *const ()), // content::EQSAllyInLoSTask::ClassTypeInfoImplementation::content::EQSAllyInLoSTask::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141bf34d0, "content_141bf34d0", tpl_38571::<0x14324b910, 0x78, 0xc, 0x20, 0x23, 0x24, 0x28, 0x70, 0x80, 0x88, 0x90, 0x68> as *const ()), // content::EQSAllyInLoSTask::ClassTypeInfoImplementation::content::EQSAllyInLoSTask::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141bf3c10, "content_141bf3c10", fn_141bf3c10 as *const ()), // content::EQSElevationTask::ClassTypeInfoImplementation::content::EQSElevationTask::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141bf3cb0, "content_141bf3cb0", tpl_38571::<0x14324e2a0, 0x78, 0xc, 0x20, 0x23, 0x24, 0x28, 0x70, 0x80, 0x88, 0x90, 0x68> as *const ()), // content::EQSElevationTask::ClassTypeInfoImplementation::content::EQSElevationTask::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft_throws(0x141bf43f0, "content_141bf43f0", fn_141bf43f0 as *const (), Throws::Discard), // content::EQSDynamicGridGeneratorNode::ClassTypeInfoImplementation::content::EQSDynamicGridGeneratorNode::ClassTypeInfoIm
    Replacement::draft(0x141bf4c20, "content_141bf4c20", fn_141bf4c20 as *const ()), // content::EQSDotCompareTask::ClassTypeInfoImplementation::content::EQSDotCompareTask::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141bf4e80, "content_141bf4e80", tpl_38586::<0x14074a0c0, 0x143426d28, 0x1426cdcb0, 0x38, 0x143250658, 0x14020f270> as *const ()), // content::EQSDistanceFieldTask::content::EQSDistanceFieldTask::vfunc_4
    Replacement::draft(0x141bf4f70, "content_141bf4f70", fn_141bf4f70 as *const ()), // content::EQSDistanceFieldTask::content::EQSDistanceFieldTask::vfunc_14
    Replacement::draft(0x141bf5060, "content_141bf5060", tpl_38587::<0x14074a0c0, 0x143426d28, 0x1426cdcb0, 0x38, 0x143250658, 0x14020f270> as *const ()), // content::EQSDistanceFieldTask::ClassTypeInfoImplementation::content::EQSDistanceFieldTask::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf5330, "content_141bf5330", fn_141bf5330 as *const ()), // content::EQSDistanceFieldTask::ClassTypeInfoImplementation::content::EQSDistanceFieldTask::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf5540, "content_141bf5540", fn_141bf5540 as *const ()), // content::EQSDistanceFieldTask::ClassTypeInfoImplementation::content::EQSDistanceFieldTask::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf55d0, "content_141bf55d0", fn_141bf55d0 as *const ()), // content::EQSDistanceFieldTask::ClassTypeInfoImplementation::content::EQSDistanceFieldTask::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf5ce0, "content_141bf5ce0", fn_141bf5ce0 as *const ()), // content::EQSDistanceFieldGenerationTask::ClassTypeInfoImplementation::content::EQSDistanceFieldGenerationTask::ClassType
    Replacement::draft(0x141bf5de0, "content_141bf5de0", fn_141bf5de0 as *const ()), // content::EQSDistanceFieldGenerationTask::ClassTypeInfoImplementation::content::EQSDistanceFieldGenerationTask::ClassType
    Replacement::draft(0x141bf60d0, "content_141bf60d0", tpl_38607::<0x140753c00> as *const ()), // content::EQSDiscGeneratorNode::ClassTypeInfoImplementation::content::EQSDiscGeneratorNode::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf6150, "content_141bf6150", tpl_38607::<0x140753b60> as *const ()), // content::EQSDiscGeneratorNode::ClassTypeInfoImplementation::content::EQSDiscGeneratorNode::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141bf6560, "content_141bf6560", fn_141bf6560 as *const (), Throws::Discard), // content::EQSDiscGeneratorNode::ClassTypeInfoImplementation::content::EQSDiscGeneratorNode::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf6640, "content_141bf6640", fn_141bf6640 as *const ()), // content::EQSDiscGeneratorNode::ClassTypeInfoImplementation::content::EQSDiscGeneratorNode::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf6800, "content_141bf6800", tpl_38610::<0x14075a4d0, 0x14384e350, 0x1426cdcb0, 0x14324fd00, 0x1401de410> as *const ()), // content::EQSCoverScoringTask::content::EQSCoverScoringTask::vfunc_3
    Replacement::draft(0x141bf6870, "content_141bf6870", tpl_38610::<0x14075a380, 0x14384e350, 0x1426cdcb0, 0x14324fd00, 0x1401de410> as *const ()), // content::EQSCoverScoringTask::content::EQSCoverScoringTask::vfunc_5
    Replacement::draft(0x141bf69b0, "content_141bf69b0", tpl_38611::<0x14075a4d0, 0x14384e350, 0x1426cdcb0, 0x14324fd00, 0x1401de410> as *const ()), // content::EQSCoverScoringTask::ClassTypeInfoImplementation::content::EQSCoverScoringTask::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141bf6a80, "content_141bf6a80", tpl_38611::<0x14075a380, 0x14384e350, 0x1426cdcb0, 0x14324fd00, 0x1401de410> as *const ()), // content::EQSCoverScoringTask::ClassTypeInfoImplementation::content::EQSCoverScoringTask::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141bf6ee0, "content_141bf6ee0", fn_141bf6ee0 as *const ()), // content::EQSCoverScoringTask::ClassTypeInfoImplementation::content::EQSCoverScoringTask::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141bf6f80, "content_141bf6f80", fn_141bf6f80 as *const ()), // content::EQSCoverScoringTask::ClassTypeInfoImplementation::content::EQSCoverScoringTask::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141bf76d0, "content_141bf76d0", fn_141bf76d0 as *const (), Throws::Discard), // content::EQSCoverPositionGenerator::ClassTypeInfoImplementation::content::EQSCoverPositionGenerator::ClassTypeInfoImplem
    Replacement::draft(0x141bf7920, "content_141bf7920", tpl_38615::<0x14075a4d0, 0x14384e2c8, 0x1426cdcb0, 0x14324fc38, 0x1401de410> as *const ()), // content::EQSAngleCompareTask::content::EQSAngleCompareTask::vfunc_3
    Replacement::draft(0x141bf7990, "content_141bf7990", tpl_38615::<0x14075a380, 0x14384e2c8, 0x1426cdcb0, 0x14324fc38, 0x1401de410> as *const ()), // content::EQSAngleCompareTask::content::EQSAngleCompareTask::vfunc_5
    Replacement::draft(0x141bf7ad0, "content_141bf7ad0", tpl_38616::<0x14075a4d0, 0x14384e2c8, 0x1426cdcb0, 0x14324fc38, 0x1401de410> as *const ()), // content::EQSAngleCompareTask::ClassTypeInfoImplementation::content::EQSAngleCompareTask::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141bf7ba0, "content_141bf7ba0", tpl_38616::<0x14075a380, 0x14384e2c8, 0x1426cdcb0, 0x14324fc38, 0x1401de410> as *const ()), // content::EQSAngleCompareTask::ClassTypeInfoImplementation::content::EQSAngleCompareTask::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141bf8000, "content_141bf8000", fn_141bf8000 as *const ()), // content::EQSAngleCompareTask::ClassTypeInfoImplementation::content::EQSAngleCompareTask::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141bf8290, "content_141bf8290", tpl_38618::<0x14075a4d0, 0x14384e2f8, 0x1426cdcb0, 0x14324fc20, 0x1401de410> as *const ()), // content::EQSKeepDistanceFromReservedPositionsTask::content::EQSKeepDistanceFromReservedPositionsTask::vfunc_3
    Replacement::draft(0x141bf8300, "content_141bf8300", tpl_38618::<0x14075a380, 0x14384e2f8, 0x1426cdcb0, 0x14324fc20, 0x1401de410> as *const ()), // content::EQSKeepDistanceFromReservedPositionsTask::content::EQSKeepDistanceFromReservedPositionsTask::vfunc_5
    Replacement::draft(0x141bf8440, "content_141bf8440", tpl_38619::<0x14075a4d0, 0x14384e2f8, 0x1426cdcb0, 0x14324fc20, 0x1401de410> as *const ()), // content::EQSKeepDistanceFromReservedPositionsTask::ClassTypeInfoImplementation::content::EQSKeepDistanceFromReservedPosi
    Replacement::draft(0x141bf8510, "content_141bf8510", tpl_38619::<0x14075a380, 0x14384e2f8, 0x1426cdcb0, 0x14324fc20, 0x1401de410> as *const ()), // content::EQSKeepDistanceFromReservedPositionsTask::ClassTypeInfoImplementation::content::EQSKeepDistanceFromReservedPosi
    Replacement::draft(0x141bf8970, "content_141bf8970", tpl_38620::<0x8, 0xc, 0x10, 0x14> as *const ()), // content::EQSKeepDistanceFromReservedPositionsTask::ClassTypeInfoImplementation::content::EQSKeepDistanceFromReservedPosi
    Replacement::draft(0x141bf89d0, "content_141bf89d0", fn_141bf89d0 as *const ()), // content::EQSKeepDistanceFromReservedPositionsTask::ClassTypeInfoImplementation::content::EQSKeepDistanceFromReservedPosi
    Replacement::draft_throws(0x141bf8b60, "content_141bf8b60", tpl_838::<0x14079af30> as *const (), Throws::Discard), // content::WarningShotDecorator::content::WarningShotDecorator::vfunc_6
    Replacement::draft_throws(0x141bf90d0, "content_141bf90d0", fn_141bf90d0 as *const (), Throws::Discard), // content::WarningShotDecorator::ClassTypeInfoImplementation::content::WarningShotDecorator::ClassTypeInfoImplementation::
    Replacement::draft(0x141bf9380, "content_141bf9380", tpl_38623::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::UnsetPropertyDecorator::content::UnsetPropertyDecorator::vfunc_4
    Replacement::draft(0x141bf9510, "content_141bf9510", tpl_38624::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::UnsetPropertyDecorator::ClassTypeInfoImplementation::content::UnsetPropertyDecorator::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141bf99b0, "content_141bf99b0", fn_141bf99b0 as *const (), Throws::Discard), // content::UnsetPropertyDecorator::ClassTypeInfoImplementation::content::UnsetPropertyDecorator::ClassTypeInfoImplementati
    Replacement::draft(0x141bf9a50, "content_141bf9a50", tpl_38626::<0x143256990, 0x8, 0x10, 0x18, 0x80, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88> as *const ()), // content::UnsetPropertyDecorator::ClassTypeInfoImplementation::content::UnsetPropertyDecorator::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141bfa1a0, "content_141bfa1a0", fn_141bfa1a0 as *const (), Throws::Discard), // content::TimeSincePropertySetConditionDecorator::ClassTypeInfoImplementation::content::TimeSincePropertySetConditionDeco
    Replacement::draft(0x141bfab20, "content_141bfab20", tpl_38623::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SetRandomIntKnowledgePropertyDecorator::content::SetRandomIntKnowledgePropertyDecorator::vfunc_4
    Replacement::draft(0x141bfacb0, "content_141bfacb0", tpl_38624::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SetRandomIntKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetRandomIntKnowledgePropertyDeco
    Replacement::draft_throws(0x141bfb150, "content_141bfb150", tpl_38628::<0x14079bb90, 0xa0, 0xa4> as *const (), Throws::Discard), // content::SetRandomIntKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetRandomIntKnowledgePropertyDeco
    Replacement::draft(0x141bfb1d0, "content_141bfb1d0", tpl_38629::<0x80, 0x8, 0x10, 0x18, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88, 0x90, 0x1432573d0> as *const ()), // content::SetRandomIntKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetRandomIntKnowledgePropertyDeco
    Replacement::draft_throws(0x141bfb920, "content_141bfb920", tpl_38628::<0x14079bb90, 0xa0, 0xa4> as *const (), Throws::Discard), // content::SetRandomFloatKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetRandomFloatKnowledgeProperty
    Replacement::draft(0x141bfb9a0, "content_141bfb9a0", tpl_38629::<0x80, 0x8, 0x10, 0x18, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88, 0x90, 0x143256050> as *const ()), // content::SetRandomFloatKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetRandomFloatKnowledgeProperty
    Replacement::draft_throws(0x141bfc0f0, "content_141bfc0f0", fn_141bfc0f0 as *const (), Throws::Discard), // content::SetPositionReservationDecorator::ClassTypeInfoImplementation::content::SetPositionReservationDecorator::ClassTy
    Replacement::draft(0x141bfc1d0, "content_141bfc1d0", fn_141bfc1d0 as *const ()), // content::SetPositionReservationDecorator::ClassTypeInfoImplementation::content::SetPositionReservationDecorator::ClassTy
    Replacement::draft(0x141bfc3b0, "content_141bfc3b0", tpl_38623::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SetKnowledgePropertyDecorator::content::SetKnowledgePropertyDecorator::vfunc_4
    Replacement::draft(0x141bfc540, "content_141bfc540", tpl_38624::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SetKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetKnowledgePropertyDecorator::ClassTypeIn
    Replacement::draft(0x141bfca20, "content_141bfca20", tpl_38626::<0x143256f90, 0x8, 0x10, 0x18, 0x80, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88> as *const ()), // content::SetKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetKnowledgePropertyDecorator::ClassTypeIn
    Replacement::draft_throws(0x141bfd170, "content_141bfd170", fn_141bfd170 as *const (), Throws::Discard), // content::SetKnowledgeBulletinDecorator::ClassTypeInfoImplementation::content::SetKnowledgeBulletinDecorator::ClassTypeIn
    Replacement::draft(0x141bfd380, "content_141bfd380", tpl_38623::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SetIntKnowledgePropertyDecorator::content::SetIntKnowledgePropertyDecorator::vfunc_4
    Replacement::draft_throws(0x141bfd470, "content_141bfd470", tpl_38633::<0x8, 0x140790320, 0xa0> as *const (), Throws::Discard), // content::SetIntKnowledgePropertyDecorator::content::SetIntKnowledgePropertyDecorator::vfunc_14
    Replacement::draft(0x141bfd560, "content_141bfd560", tpl_38624::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SetIntKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetIntKnowledgePropertyDecorator::Class
    Replacement::draft_throws(0x141bfd830, "content_141bfd830", tpl_38634::<0x8, 0x140790320, 0xa0> as *const (), Throws::Discard), // content::SetIntKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetIntKnowledgePropertyDecorator::Class
    Replacement::draft_throws(0x141bfda40, "content_141bfda40", tpl_38635::<0x14079bb90, 0xa0> as *const (), Throws::Discard), // content::SetIntKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetIntKnowledgePropertyDecorator::Class
    Replacement::draft(0x141bfdab0, "content_141bfdab0", tpl_38629::<0x80, 0x8, 0x10, 0x18, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88, 0x90, 0x143257d90> as *const ()), // content::SetIntKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetIntKnowledgePropertyDecorator::Class
    Replacement::draft_throws(0x141bfdd10, "content_141bfdd10", tpl_38633::<0x8, 0x140790320, 0xa0> as *const (), Throws::Discard), // content::SetFloatKnowledgePropertyDecorator::content::SetFloatKnowledgePropertyDecorator::vfunc_14
    Replacement::draft_throws(0x141bfe080, "content_141bfe080", tpl_38634::<0x8, 0x140790320, 0xa0> as *const (), Throws::Discard), // content::SetFloatKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetFloatKnowledgePropertyDecorator::C
    Replacement::draft_throws(0x141bfe290, "content_141bfe290", tpl_38635::<0x14079bb90, 0xa0> as *const (), Throws::Discard), // content::SetFloatKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetFloatKnowledgePropertyDecorator::C
    Replacement::draft(0x141bfe300, "content_141bfe300", tpl_38629::<0x80, 0x8, 0x10, 0x18, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88, 0x90, 0x1432529d0> as *const ()), // content::SetFloatKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetFloatKnowledgePropertyDecorator::C
    Replacement::draft(0x141bfe4c0, "content_141bfe4c0", tpl_38623::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SetBoolKnowledgePropertyDecorator::content::SetBoolKnowledgePropertyDecorator::vfunc_4
    Replacement::draft_throws(0x141bfe5b0, "content_141bfe5b0", tpl_38636::<0x8, 0x140790320, 0xa0> as *const (), Throws::Discard), // content::SetBoolKnowledgePropertyDecorator::content::SetBoolKnowledgePropertyDecorator::vfunc_14
    Replacement::draft(0x141bfe6a0, "content_141bfe6a0", tpl_38624::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SetBoolKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetBoolKnowledgePropertyDecorator::Cla
    Replacement::draft_throws(0x141bfe970, "content_141bfe970", tpl_38637::<0x8, 0x140790320, 0xa0> as *const (), Throws::Discard), // content::SetBoolKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetBoolKnowledgePropertyDecorator::Cla
    Replacement::draft_throws(0x141bfeb80, "content_141bfeb80", tpl_38638::<0x14079bb90, 0xa0> as *const (), Throws::Discard), // content::SetBoolKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetBoolKnowledgePropertyDecorator::Cla
    Replacement::draft(0x141bfebf0, "content_141bfebf0", tpl_38629::<0x80, 0x8, 0x10, 0x18, 0x20, 0x28, 0x2b, 0x2c, 0x30, 0x78, 0x88, 0x90, 0x143254d90> as *const ()), // content::SetBoolKnowledgePropertyDecorator::ClassTypeInfoImplementation::content::SetBoolKnowledgePropertyDecorator::Cla
    Replacement::draft(0x141bfedb0, "content_141bfedb0", tpl_38639::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SayDecorator::content::SayDecorator::vfunc_4
    Replacement::draft(0x141bfef40, "content_141bfef40", tpl_38640::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::SayDecorator::ClassTypeInfoImplementation::content::SayDecorator::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft_throws(0x141bff3e0, "content_141bff3e0", tpl_38641::<0x8, 0x10, 0x1406f2520, 0x20, 0x24, 0x14240aa00> as *const (), Throws::Discard), // content::SayDecorator::ClassTypeInfoImplementation::content::SayDecorator::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141bff470, "content_141bff470", fn_141bff470 as *const ()), // content::SayDecorator::ClassTypeInfoImplementation::content::SayDecorator::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141bff600, "content_141bff600", fn_141bff600 as *const ()), // content::ReleaseTicketDecorator::content::ReleaseTicketDecorator::vfunc_4
    Replacement::draft_throws(0x141bff720, "content_141bff720", tpl_838::<0x14078a4d0> as *const (), Throws::Discard), // content::ReleaseTicketDecorator::content::ReleaseTicketDecorator::vfunc_17
    Replacement::draft(0x141bff790, "content_141bff790", fn_141bff790 as *const ()), // content::ReleaseTicketDecorator::ClassTypeInfoImplementation::content::ReleaseTicketDecorator::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141bffc30, "content_141bffc30", fn_141bffc30 as *const (), Throws::Discard), // content::ReleaseTicketDecorator::ClassTypeInfoImplementation::content::ReleaseTicketDecorator::ClassTypeInfoImplementati
    Replacement::draft(0x141bffcc0, "content_141bffcc0", fn_141bffcc0 as *const ()), // content::ReleaseTicketDecorator::ClassTypeInfoImplementation::content::ReleaseTicketDecorator::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c003d0, "content_141c003d0", fn_141c003d0 as *const (), Throws::Discard), // content::OnStateCompletedDecorator::ClassTypeInfoImplementation::content::OnStateCompletedDecorator::ClassTypeInfoImplem
    Replacement::draft(0x141c00460, "content_141c00460", fn_141c00460 as *const ()), // content::OnStateCompletedDecorator::ClassTypeInfoImplementation::content::OnStateCompletedDecorator::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c00b70, "content_141c00b70", fn_141c00b70 as *const (), Throws::Discard), // content::ModifyIntPropertyDecorator::ClassTypeInfoImplementation::content::ModifyIntPropertyDecorator::ClassTypeInfoImpl
    Replacement::draft(0x141c00de0, "content_141c00de0", tpl_38639::<0x1406ec9b0, 0x14384e490, 0x1426cdcb0, 0x1432514f8, 0x1401de410> as *const ()), // content::LogicBlockDecorator::content::LogicBlockDecorator::vfunc_4
    Replacement::draft(0x141c00f70, "content_141c00f70", tpl_38640::<0x1406ec9b0, 0x14384e490, 0x1426cdcb0, 0x1432514f8, 0x1401de410> as *const ()), // content::LogicBlockDecorator::ClassTypeInfoImplementation::content::LogicBlockDecorator::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141c01410, "content_141c01410", tpl_38650::<0x8, 0x10, 0x1406f2520, 0x20> as *const (), Throws::Discard), // content::LogicBlockDecorator::ClassTypeInfoImplementation::content::LogicBlockDecorator::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c01610, "content_141c01610", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::KnowledgeConditionDecorator::content::KnowledgeConditionDecorator::vfunc_3
    Replacement::draft(0x141c01670, "content_141c01670", tpl_38652::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::KnowledgeConditionDecorator::content::KnowledgeConditionDecorator::vfunc_4
    Replacement::draft(0x141c016d0, "content_141c016d0", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::KnowledgeConditionDecorator::content::KnowledgeConditionDecorator::vfunc_5
    Replacement::draft(0x141c01810, "content_141c01810", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::KnowledgeConditionDecorator::ClassTypeInfoImplementation::content::KnowledgeConditionDecorator::ClassTypeInfoIm
    Replacement::draft(0x141c018a0, "content_141c018a0", tpl_38654::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::KnowledgeConditionDecorator::ClassTypeInfoImplementation::content::KnowledgeConditionDecorator::ClassTypeInfoIm
    Replacement::draft(0x141c01920, "content_141c01920", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::KnowledgeConditionDecorator::ClassTypeInfoImplementation::content::KnowledgeConditionDecorator::ClassTypeInfoIm
    Replacement::draft_throws(0x141c024b0, "content_141c024b0", fn_141c024b0 as *const (), Throws::Discard), // content::IsPropertySetConditionDecorator::ClassTypeInfoImplementation::content::IsPropertySetConditionDecorator::ClassTy
    Replacement::draft(0x141c02570, "content_141c02570", tpl_38513::<0x143258290, 0x8, 0x10, 0x18, 0x78, 0x20, 0x23, 0x24, 0x28, 0x70, 0x80, 0x88> as *const ()), // content::IsPropertySetConditionDecorator::ClassTypeInfoImplementation::content::IsPropertySetConditionDecorator::ClassTy
    Replacement::draft_throws(0x141c02cc0, "content_141c02cc0", fn_141c02cc0 as *const (), Throws::Discard), // content::GuideSpeedDecorator::ClassTypeInfoImplementation::content::GuideSpeedDecorator::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141c03460, "content_141c03460", tpl_38628::<0x14079b8c0, 0x128, 0x12c> as *const (), Throws::Discard), // content::FollowSpeedDecorator::ClassTypeInfoImplementation::content::FollowSpeedDecorator::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141c03bf0, "content_141c03bf0", tpl_38628::<0x14079b390, 0x118, 0x11c> as *const (), Throws::Discard), // content::FloatDifferenceConditionDecorator::ClassTypeInfoImplementation::content::FloatDifferenceConditionDecorator::Cla
    Replacement::draft(0x141c03c70, "content_141c03c70", fn_141c03c70 as *const ()), // content::FloatDifferenceConditionDecorator::ClassTypeInfoImplementation::content::FloatDifferenceConditionDecorator::Cla
    Replacement::draft(0x141c03df0, "content_141c03df0", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DistanceConditionDecorator::content::DistanceConditionDecorator::vfunc_3
    Replacement::draft(0x141c03e60, "content_141c03e60", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DistanceConditionDecorator::content::DistanceConditionDecorator::vfunc_5
    Replacement::draft_throws(0x141c03f40, "content_141c03f40", tpl_38633::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DistanceConditionDecorator::content::DistanceConditionDecorator::vfunc_14
    Replacement::draft(0x141c03ff0, "content_141c03ff0", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DistanceConditionDecorator::ClassTypeInfoImplementation::content::DistanceConditionDecorator::ClassTypeInfoImpl
    Replacement::draft(0x141c040c0, "content_141c040c0", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DistanceConditionDecorator::ClassTypeInfoImplementation::content::DistanceConditionDecorator::ClassTypeInfoImpl
    Replacement::draft(0x141c04320, "content_141c04320", fn_141c04320 as *const ()), // sub_141c04320
    Replacement::draft_throws(0x141c04350, "content_141c04350", tpl_38634::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DistanceConditionDecorator::ClassTypeInfoImplementation::content::DistanceConditionDecorator::ClassTypeInfoImpl
    Replacement::draft_throws(0x141c04560, "content_141c04560", tpl_38635::<0x14079b390, 0x118> as *const (), Throws::Discard), // content::DistanceConditionDecorator::ClassTypeInfoImplementation::content::DistanceConditionDecorator::ClassTypeInfoImpl
    Replacement::draft_throws(0x141c04f20, "content_141c04f20", tpl_38659::<0x8, 0x10, 0x1406f2520> as *const (), Throws::Discard), // content::DisabledDecorator::ClassTypeInfoImplementation::content::DisabledDecorator::ClassTypeInfoImplementation::vfunc_
    Replacement::draft_throws(0x141c056a0, "content_141c056a0", fn_141c056a0 as *const (), Throws::Discard), // content::DecoratorTicketCondition::ClassTypeInfoImplementation::content::DecoratorTicketCondition::ClassTypeInfoImplemen
    Replacement::draft(0x141c05740, "content_141c05740", fn_141c05740 as *const ()), // content::DecoratorTicketCondition::ClassTypeInfoImplementation::content::DecoratorTicketCondition::ClassTypeInfoImplemen
    Replacement::draft(0x141c058f0, "content_141c058f0", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorRestrictedCondition::content::DecoratorRestrictedCondition::vfunc_3
    Replacement::draft(0x141c05950, "content_141c05950", tpl_38652::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorRestrictedCondition::content::DecoratorRestrictedCondition::vfunc_4
    Replacement::draft(0x141c059b0, "content_141c059b0", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorRestrictedCondition::content::DecoratorRestrictedCondition::vfunc_5
    Replacement::draft(0x141c05c50, "content_141c05c50", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorRestrictedCondition::ClassTypeInfoImplementation::content::DecoratorRestrictedCondition::ClassTypeInfo
    Replacement::draft(0x141c05ce0, "content_141c05ce0", tpl_38654::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorRestrictedCondition::ClassTypeInfoImplementation::content::DecoratorRestrictedCondition::ClassTypeInfo
    Replacement::draft(0x141c05d60, "content_141c05d60", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorRestrictedCondition::ClassTypeInfoImplementation::content::DecoratorRestrictedCondition::ClassTypeInfo
    Replacement::draft_throws(0x141c06a20, "content_141c06a20", tpl_38662::<0x8, 0x10, 0x1406f2520, 0x20> as *const (), Throws::Discard), // content::DecoratorLostTicketCondition::ClassTypeInfoImplementation::content::DecoratorLostTicketCondition::ClassTypeInfo
    Replacement::draft(0x141c06c20, "content_141c06c20", tpl_38663::<0x1406eca20, 0x14384e390, 0x20, 0x143251388, 0x1, 0x1401de410> as *const ()), // content::DecoratorKnowledgeBulletinCondition::content::DecoratorKnowledgeBulletinCondition::vfunc_3
    Replacement::draft(0x141c06c80, "content_141c06c80", fn_141c06c80 as *const ()), // content::DecoratorKnowledgeBulletinCondition::content::DecoratorKnowledgeBulletinCondition::vfunc_4
    Replacement::draft(0x141c06ce0, "content_141c06ce0", tpl_38663::<0x1406ec8f0, 0x14384e390, 0x20, 0x143251388, 0x1, 0x1401de410> as *const ()), // content::DecoratorKnowledgeBulletinCondition::content::DecoratorKnowledgeBulletinCondition::vfunc_5
    Replacement::draft(0x141c06e20, "content_141c06e20", tpl_38665::<0x1406eca20, 0x14384e390, 0x20, 0x143251388, 0x1, 0x1401de410> as *const ()), // content::DecoratorKnowledgeBulletinCondition::ClassTypeInfoImplementation::content::DecoratorKnowledgeBulletinCondition:
    Replacement::draft(0x141c06ea0, "content_141c06ea0", fn_141c06ea0 as *const ()), // content::DecoratorKnowledgeBulletinCondition::ClassTypeInfoImplementation::content::DecoratorKnowledgeBulletinCondition:
    Replacement::draft(0x141c06f20, "content_141c06f20", tpl_38665::<0x1406ec8f0, 0x14384e390, 0x20, 0x143251388, 0x1, 0x1401de410> as *const ()), // content::DecoratorKnowledgeBulletinCondition::ClassTypeInfoImplementation::content::DecoratorKnowledgeBulletinCondition:
    Replacement::draft_throws(0x141c07370, "content_141c07370", fn_141c07370 as *const (), Throws::Discard), // content::DecoratorKnowledgeBulletinCondition::ClassTypeInfoImplementation::content::DecoratorKnowledgeBulletinCondition:
    Replacement::draft(0x141c07570, "content_141c07570", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorIntCondition::content::DecoratorIntCondition::vfunc_3
    Replacement::draft(0x141c075d0, "content_141c075d0", tpl_38652::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorIntCondition::content::DecoratorIntCondition::vfunc_4
    Replacement::draft(0x141c07630, "content_141c07630", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorIntCondition::content::DecoratorIntCondition::vfunc_5
    Replacement::draft_throws(0x141c07710, "content_141c07710", tpl_38633::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DecoratorIntCondition::content::DecoratorIntCondition::vfunc_14
    Replacement::draft(0x141c077c0, "content_141c077c0", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorIntCondition::ClassTypeInfoImplementation::content::DecoratorIntCondition::ClassTypeInfoImplementation
    Replacement::draft(0x141c07850, "content_141c07850", tpl_38654::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorIntCondition::ClassTypeInfoImplementation::content::DecoratorIntCondition::ClassTypeInfoImplementation
    Replacement::draft(0x141c078d0, "content_141c078d0", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorIntCondition::ClassTypeInfoImplementation::content::DecoratorIntCondition::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141c07b60, "content_141c07b60", tpl_38634::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DecoratorIntCondition::ClassTypeInfoImplementation::content::DecoratorIntCondition::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141c07d70, "content_141c07d70", tpl_38635::<0x14079b390, 0x118> as *const (), Throws::Discard), // content::DecoratorIntCondition::ClassTypeInfoImplementation::content::DecoratorIntCondition::ClassTypeInfoImplementation
    Replacement::draft(0x141c07f50, "content_141c07f50", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorFloatCondition::content::DecoratorFloatCondition::vfunc_3
    Replacement::draft(0x141c07fc0, "content_141c07fc0", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorFloatCondition::content::DecoratorFloatCondition::vfunc_5
    Replacement::draft_throws(0x141c080a0, "content_141c080a0", tpl_38633::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DecoratorFloatCondition::content::DecoratorFloatCondition::vfunc_14
    Replacement::draft_throws(0x141c08100, "content_141c08100", tpl_838::<0x14077c030> as *const (), Throws::Discard), // content::DecoratorFloatCondition::content::DecoratorFloatCondition::vfunc_15
    Replacement::draft_throws(0x141c08120, "content_141c08120", tpl_838::<0x14077bf20> as *const (), Throws::Discard), // content::DecoratorFloatCondition::content::DecoratorFloatCondition::vfunc_17
    Replacement::draft(0x141c08150, "content_141c08150", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorFloatCondition::ClassTypeInfoImplementation::content::DecoratorFloatCondition::ClassTypeInfoImplementa
    Replacement::draft(0x141c08220, "content_141c08220", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorFloatCondition::ClassTypeInfoImplementation::content::DecoratorFloatCondition::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141c084b0, "content_141c084b0", tpl_38634::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DecoratorFloatCondition::ClassTypeInfoImplementation::content::DecoratorFloatCondition::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141c086c0, "content_141c086c0", tpl_38635::<0x14079b390, 0x118> as *const (), Throws::Discard), // content::DecoratorFloatCondition::ClassTypeInfoImplementation::content::DecoratorFloatCondition::ClassTypeInfoImplementa
    Replacement::draft(0x141c088a0, "content_141c088a0", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorCanThrowCondition::content::DecoratorCanThrowCondition::vfunc_3
    Replacement::draft(0x141c08910, "content_141c08910", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorCanThrowCondition::content::DecoratorCanThrowCondition::vfunc_5
    Replacement::draft(0x141c08a50, "content_141c08a50", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorCanThrowCondition::ClassTypeInfoImplementation::content::DecoratorCanThrowCondition::ClassTypeInfoImpl
    Replacement::draft(0x141c08b20, "content_141c08b20", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorCanThrowCondition::ClassTypeInfoImplementation::content::DecoratorCanThrowCondition::ClassTypeInfoImpl
    Replacement::draft_throws(0x141c08f80, "content_141c08f80", fn_141c08f80 as *const (), Throws::Discard), // content::DecoratorCanThrowCondition::ClassTypeInfoImplementation::content::DecoratorCanThrowCondition::ClassTypeInfoImpl
    Replacement::draft(0x141c091c0, "content_141c091c0", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorCanChargeCondition::content::DecoratorCanChargeCondition::vfunc_3
    Replacement::draft(0x141c09230, "content_141c09230", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorCanChargeCondition::content::DecoratorCanChargeCondition::vfunc_5
    Replacement::draft_throws(0x141c09310, "content_141c09310", tpl_38633::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DecoratorCanChargeCondition::content::DecoratorCanChargeCondition::vfunc_14
    Replacement::draft(0x141c093c0, "content_141c093c0", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorCanChargeCondition::ClassTypeInfoImplementation::content::DecoratorCanChargeCondition::ClassTypeInfoIm
    Replacement::draft(0x141c09490, "content_141c09490", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorCanChargeCondition::ClassTypeInfoImplementation::content::DecoratorCanChargeCondition::ClassTypeInfoIm
    Replacement::draft_throws(0x141c09720, "content_141c09720", tpl_38634::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DecoratorCanChargeCondition::ClassTypeInfoImplementation::content::DecoratorCanChargeCondition::ClassTypeInfoIm
    Replacement::draft_throws(0x141c09930, "content_141c09930", tpl_38635::<0x14079b390, 0x118> as *const (), Throws::Discard), // content::DecoratorCanChargeCondition::ClassTypeInfoImplementation::content::DecoratorCanChargeCondition::ClassTypeInfoIm
    Replacement::draft(0x141c09b10, "content_141c09b10", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorBoolCondition::content::DecoratorBoolCondition::vfunc_3
    Replacement::draft(0x141c09b70, "content_141c09b70", tpl_38652::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorBoolCondition::content::DecoratorBoolCondition::vfunc_4
    Replacement::draft(0x141c09bd0, "content_141c09bd0", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorBoolCondition::content::DecoratorBoolCondition::vfunc_5
    Replacement::draft_throws(0x141c09cb0, "content_141c09cb0", tpl_38636::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DecoratorBoolCondition::content::DecoratorBoolCondition::vfunc_14
    Replacement::draft(0x141c09d60, "content_141c09d60", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorBoolCondition::ClassTypeInfoImplementation::content::DecoratorBoolCondition::ClassTypeInfoImplementati
    Replacement::draft(0x141c09df0, "content_141c09df0", tpl_38654::<0x1406ec9b0, 0x14384e4a8, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorBoolCondition::ClassTypeInfoImplementation::content::DecoratorBoolCondition::ClassTypeInfoImplementati
    Replacement::draft(0x141c09e70, "content_141c09e70", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DecoratorBoolCondition::ClassTypeInfoImplementation::content::DecoratorBoolCondition::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c0a100, "content_141c0a100", tpl_38637::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::DecoratorBoolCondition::ClassTypeInfoImplementation::content::DecoratorBoolCondition::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c0a310, "content_141c0a310", tpl_38638::<0x14079b390, 0x118> as *const (), Throws::Discard), // content::DecoratorBoolCondition::ClassTypeInfoImplementation::content::DecoratorBoolCondition::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c0a580, "content_141c0a580", tpl_838::<0x140778de0> as *const (), Throws::Discard), // content::DecoratorBlockTicketStealing::content::DecoratorBlockTicketStealing::vfunc_12
    Replacement::draft_throws(0x141c0aa90, "content_141c0aa90", tpl_38662::<0x8, 0x10, 0x1406f2520, 0x20> as *const (), Throws::Discard), // content::DecoratorBlockTicketStealing::ClassTypeInfoImplementation::content::DecoratorBlockTicketStealing::ClassTypeInfo
    Replacement::draft_throws(0x141c0b470, "content_141c0b470", tpl_38659::<0x8, 0x10, 0x1406f2520> as *const (), Throws::Discard), // content::DecoratorAlwaysSucceed::ClassTypeInfoImplementation::content::DecoratorAlwaysSucceed::ClassTypeInfoImplementati
    Replacement::draft(0x141c0b650, "content_141c0b650", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DamageTrackingDecorator::content::DamageTrackingDecorator::vfunc_3
    Replacement::draft(0x141c0b6c0, "content_141c0b6c0", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DamageTrackingDecorator::content::DamageTrackingDecorator::vfunc_5
    Replacement::draft(0x141c0b800, "content_141c0b800", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DamageTrackingDecorator::ClassTypeInfoImplementation::content::DamageTrackingDecorator::ClassTypeInfoImplementa
    Replacement::draft(0x141c0b8d0, "content_141c0b8d0", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::DamageTrackingDecorator::ClassTypeInfoImplementation::content::DamageTrackingDecorator::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141c0bd30, "content_141c0bd30", fn_141c0bd30 as *const (), Throws::Discard), // content::DamageTrackingDecorator::ClassTypeInfoImplementation::content::DamageTrackingDecorator::ClassTypeInfoImplementa
    Replacement::draft(0x141c0bf40, "content_141c0bf40", tpl_38623::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::CopyPropertyDecorator::content::CopyPropertyDecorator::vfunc_4
    Replacement::draft(0x141c0c0d0, "content_141c0c0d0", tpl_38624::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::CopyPropertyDecorator::ClassTypeInfoImplementation::content::CopyPropertyDecorator::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141c0c570, "content_141c0c570", fn_141c0c570 as *const (), Throws::Discard), // content::CopyPropertyDecorator::ClassTypeInfoImplementation::content::CopyPropertyDecorator::ClassTypeInfoImplementation
    Replacement::draft(0x141c0c7a0, "content_141c0c7a0", tpl_38639::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::CallScriptDecorator::content::CallScriptDecorator::vfunc_4
    Replacement::draft(0x141c0c930, "content_141c0c930", tpl_38640::<0x1406ec9b0, 0x14384e410, 0x1426cdcb0, 0x143251798, 0x1401de410> as *const ()), // content::CallScriptDecorator::ClassTypeInfoImplementation::content::CallScriptDecorator::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141c0cdd0, "content_141c0cdd0", tpl_38641::<0x8, 0x10, 0x1406f2520, 0x20, 0x24, 0x14240aa00> as *const (), Throws::Discard), // content::CallScriptDecorator::ClassTypeInfoImplementation::content::CallScriptDecorator::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c0ce60, "content_141c0ce60", fn_141c0ce60 as *const ()), // content::CallScriptDecorator::ClassTypeInfoImplementation::content::CallScriptDecorator::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c0cff0, "content_141c0cff0", fn_141c0cff0 as *const ()), // content::CalculateDirectionDecorator::content::CalculateDirectionDecorator::vfunc_4
    Replacement::draft(0x141c0d180, "content_141c0d180", fn_141c0d180 as *const ()), // content::CalculateDirectionDecorator::ClassTypeInfoImplementation::content::CalculateDirectionDecorator::ClassTypeInfoIm
    Replacement::draft_throws(0x141c0d620, "content_141c0d620", fn_141c0d620 as *const (), Throws::Discard), // content::CalculateDirectionDecorator::ClassTypeInfoImplementation::content::CalculateDirectionDecorator::ClassTypeInfoIm
    Replacement::draft_throws(0x141c0de10, "content_141c0de10", tpl_38662::<0x8, 0x10, 0x1406f2520, 0x20> as *const (), Throws::Discard), // content::AvoidanceConfigScopeDecorator::ClassTypeInfoImplementation::content::AvoidanceConfigScopeDecorator::ClassTypeIn
    Replacement::draft_throws(0x141c0e5b0, "content_141c0e5b0", tpl_38650::<0x8, 0x10, 0x1406f2520, 0x20> as *const (), Throws::Discard), // content::AnimationControlParamDecorator::ClassTypeInfoImplementation::content::AnimationControlParamDecorator::ClassType
    Replacement::draft(0x141c0e7b0, "content_141c0e7b0", tpl_38651::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::AngleCompareConditionDecorator::content::AngleCompareConditionDecorator::vfunc_3
    Replacement::draft(0x141c0e820, "content_141c0e820", tpl_38651::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::AngleCompareConditionDecorator::content::AngleCompareConditionDecorator::vfunc_5
    Replacement::draft_throws(0x141c0e900, "content_141c0e900", tpl_38633::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::AngleCompareConditionDecorator::content::AngleCompareConditionDecorator::vfunc_14
    Replacement::draft(0x141c0e9b0, "content_141c0e9b0", tpl_38653::<0x1406eca20, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::AngleCompareConditionDecorator::ClassTypeInfoImplementation::content::AngleCompareConditionDecorator::ClassType
    Replacement::draft(0x141c0ea80, "content_141c0ea80", tpl_38653::<0x1406ec8f0, 0x14384e4a8, 0x20, 0x143251510, 0x3, 0x1401de410> as *const ()), // content::AngleCompareConditionDecorator::ClassTypeInfoImplementation::content::AngleCompareConditionDecorator::ClassType
    Replacement::draft_throws(0x141c0ed10, "content_141c0ed10", tpl_38634::<0x8, 0x140786200, 0x118> as *const (), Throws::Discard), // content::AngleCompareConditionDecorator::ClassTypeInfoImplementation::content::AngleCompareConditionDecorator::ClassType
    Replacement::draft_throws(0x141c0ef20, "content_141c0ef20", tpl_38635::<0x14079b390, 0x118> as *const (), Throws::Discard), // content::AngleCompareConditionDecorator::ClassTypeInfoImplementation::content::AngleCompareConditionDecorator::ClassType
    Replacement::draft_throws(0x141c0f6a0, "content_141c0f6a0", tpl_38662::<0x8, 0x10, 0x1406f2520, 0x20> as *const (), Throws::Discard), // content::AcquireTicketDecorator::ClassTypeInfoImplementation::content::AcquireTicketDecorator::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c0fe40, "content_141c0fe40", fn_141c0fe40 as *const (), Throws::Discard), // content::BehaviorTreeWaitNode::ClassTypeInfoImplementation::content::BehaviorTreeWaitNode::ClassTypeInfoImplementation::
    Replacement::draft(0x141c10020, "content_141c10020", tpl_38663::<0x1407b54e0, 0x14384e4e0, 0x38, 0x14325e4f0, 0x1, 0x1401de410> as *const ()), // content::BehaviorTreeUseWorkspotNode::content::BehaviorTreeUseWorkspotNode::vfunc_1
    Replacement::draft(0x141c10090, "content_141c10090", tpl_38663::<0x1407b5410, 0x14384e4e0, 0x38, 0x14325e4f0, 0x1, 0x1401de410> as *const ()), // content::BehaviorTreeUseWorkspotNode::content::BehaviorTreeUseWorkspotNode::vfunc_3
    Replacement::draft(0x141c101d0, "content_141c101d0", tpl_38665::<0x1407b54e0, 0x14384e4e0, 0x38, 0x14325e4f0, 0x1, 0x1401de410> as *const ()), // content::BehaviorTreeUseWorkspotNode::ClassTypeInfoImplementation::content::BehaviorTreeUseWorkspotNode::ClassTypeInfoIm
    Replacement::draft(0x141c10290, "content_141c10290", tpl_38665::<0x1407b5410, 0x14384e4e0, 0x38, 0x14325e4f0, 0x1, 0x1401de410> as *const ()), // content::BehaviorTreeUseWorkspotNode::ClassTypeInfoImplementation::content::BehaviorTreeUseWorkspotNode::ClassTypeInfoIm
    Replacement::draft_throws(0x141c106e0, "content_141c106e0", fn_141c106e0 as *const (), Throws::Discard), // content::BehaviorTreeUseWorkspotNode::ClassTypeInfoImplementation::content::BehaviorTreeUseWorkspotNode::ClassTypeInfoIm
    Replacement::draft(0x141c10750, "content_141c10750", tpl_38677::<0x143258510, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const ()), // content::BehaviorTreeUseWorkspotNode::ClassTypeInfoImplementation::content::BehaviorTreeUseWorkspotNode::ClassTypeInfoIm
    Replacement::draft_throws(0x141c10e70, "content_141c10e70", fn_141c10e70 as *const (), Throws::Discard), // content::BehaviorTreeUseAbilityNode::ClassTypeInfoImplementation::content::BehaviorTreeUseAbilityNode::ClassTypeInfoImpl
    Replacement::draft(0x141c10ef0, "content_141c10ef0", fn_141c10ef0 as *const ()), // content::BehaviorTreeUseAbilityNode::ClassTypeInfoImplementation::content::BehaviorTreeUseAbilityNode::ClassTypeInfoImpl
    Replacement::draft_throws(0x141c11600, "content_141c11600", tpl_38680::<0x1407cabd0, 0x38, 0x14240aa00, 0x80, 0x140636a20> as *const (), Throws::Discard), // content::BehaviorTreeThrowGrenadeNode::ClassTypeInfoImplementation::content::BehaviorTreeThrowGrenadeNode::ClassTypeInfo
    Replacement::draft(0x141c11680, "content_141c11680", fn_141c11680 as *const ()), // content::BehaviorTreeThrowGrenadeNode::ClassTypeInfoImplementation::content::BehaviorTreeThrowGrenadeNode::ClassTypeInfo
    Replacement::draft_throws(0x141c11df0, "content_141c11df0", fn_141c11df0 as *const (), Throws::Discard), // content::BehaviorTreeThrowGrenadeFromCoverNode::ClassTypeInfoImplementation::content::BehaviorTreeThrowGrenadeFromCoverN
    Replacement::draft(0x141c12030, "content_141c12030", tpl_38683::<0x14325e3b8, 0x38, 0x1, 0x14020f270> as *const ()), // content::BehaviorTreeTestNode::content::BehaviorTreeTestNode::vfunc_2
    Replacement::draft_throws(0x141c12080, "content_141c12080", tpl_838::<0x1407c2a50> as *const (), Throws::Discard), // content::BehaviorTreeTestNode::content::BehaviorTreeTestNode::vfunc_4
    Replacement::draft(0x141c121a0, "content_141c121a0", tpl_38684::<0x14325e3b8, 0x1, 0x14020f270> as *const ()), // content::BehaviorTreeTestNode::ClassTypeInfoImplementation::content::BehaviorTreeTestNode::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141c12610, "content_141c12610", fn_141c12610 as *const (), Throws::Discard), // content::BehaviorTreeTestNode::ClassTypeInfoImplementation::content::BehaviorTreeTestNode::ClassTypeInfoImplementation::
    Replacement::draft(0x141c126a0, "content_141c126a0", fn_141c126a0 as *const ()), // content::BehaviorTreeTestNode::ClassTypeInfoImplementation::content::BehaviorTreeTestNode::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141c12e40, "content_141c12e40", fn_141c12e40 as *const (), Throws::Discard), // content::BehaviorTreeTeleportNode::ClassTypeInfoImplementation::content::BehaviorTreeTeleportNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c14050, "content_141c14050", fn_141c14050 as *const (), Throws::Discard), // content::BehaviorTreeTakeCoverNode::ClassTypeInfoImplementation::content::BehaviorTreeTakeCoverNode::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c14310, "content_141c14310", fn_141c14310 as *const (), Throws::Discard), // content::BehaviorTreeStunnedNode::content::BehaviorTreeStunnedNode::vfunc_12
    Replacement::draft_throws(0x141c14680, "content_141c14680", fn_141c14680 as *const (), Throws::Discard), // content::BehaviorTreeStunnedNode::ClassTypeInfoImplementation::content::BehaviorTreeStunnedNode::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141c14890, "content_141c14890", tpl_38628::<0x1407cabd0, 0x38, 0x3c> as *const (), Throws::Discard), // content::BehaviorTreeStunnedNode::ClassTypeInfoImplementation::content::BehaviorTreeStunnedNode::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141c15010, "content_141c15010", fn_141c15010 as *const (), Throws::Discard), // content::BehaviorTreeShootFromCoverNode::ClassTypeInfoImplementation::content::BehaviorTreeShootFromCoverNode::ClassType
    Replacement::draft_throws(0x141c15830, "content_141c15830", fn_141c15830 as *const (), Throws::Discard), // content::BehaviorTreeShootAtNode::ClassTypeInfoImplementation::content::BehaviorTreeShootAtNode::ClassTypeInfoImplementa
    Replacement::draft(0x141c15a70, "content_141c15a70", tpl_38693 as *const ()), // content::BehaviorTreeService::ClassTypeInfoImplementation::content::BehaviorTreeService::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c15b10, "content_141c15b10", tpl_38694 as *const ()), // content::BehaviorTreeService::ClassTypeInfoImplementation::content::BehaviorTreeService::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c15f20, "content_141c15f20", tpl_38695::<0x8> as *const ()), // content::BehaviorTreeService::ClassTypeInfoImplementation::content::BehaviorTreeService::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141c168f0, "content_141c168f0", tpl_38636::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeSelectorNode::content::BehaviorTreeSelectorNode::vfunc_12
    Replacement::draft_throws(0x141c16c40, "content_141c16c40", tpl_38637::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeSelectorNode::ClassTypeInfoImplementation::content::BehaviorTreeSelectorNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c16e40, "content_141c16e40", tpl_38638::<0x1407cabd0, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeSelectorNode::ClassTypeInfoImplementation::content::BehaviorTreeSelectorNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c17620, "content_141c17620", fn_141c17620 as *const (), Throws::Discard), // content::BehaviorTreeSayNode::ClassTypeInfoImplementation::content::BehaviorTreeSayNode::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c176a0, "content_141c176a0", fn_141c176a0 as *const ()), // content::BehaviorTreeSayNode::ClassTypeInfoImplementation::content::BehaviorTreeSayNode::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c178d0, "content_141c178d0", tpl_365::<0x1407b7f80> as *const ()), // content::BehaviorTreeRootNode::content::BehaviorTreeRootNode::vfunc_6
    Replacement::draft_throws(0x141c18290, "content_141c18290", fn_141c18290 as *const (), Throws::Discard), // content::BehaviorTreeReferenceNode::content::BehaviorTreeReferenceNode::vfunc_12
    Replacement::draft_throws(0x141c185f0, "content_141c185f0", fn_141c185f0 as *const (), Throws::Discard), // content::BehaviorTreeReferenceNode::ClassTypeInfoImplementation::content::BehaviorTreeReferenceNode::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c187f0, "content_141c187f0", fn_141c187f0 as *const (), Throws::Discard), // content::BehaviorTreeReferenceNode::ClassTypeInfoImplementation::content::BehaviorTreeReferenceNode::ClassTypeInfoImplem
    Replacement::draft(0x141c18860, "content_141c18860", tpl_38701::<0x143258da0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40> as *const ()), // content::BehaviorTreeReferenceNode::ClassTypeInfoImplementation::content::BehaviorTreeReferenceNode::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c18f70, "content_141c18f70", fn_141c18f70 as *const (), Throws::Discard), // content::BehaviorTreeRandomSelectorNode::ClassTypeInfoImplementation::content::BehaviorTreeRandomSelectorNode::ClassType
    Replacement::draft(0x141c18fe0, "content_141c18fe0", tpl_38701::<0x14325d690, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x40, 0x48> as *const ()), // content::BehaviorTreeRandomSelectorNode::ClassTypeInfoImplementation::content::BehaviorTreeRandomSelectorNode::ClassType
    Replacement::draft_throws(0x141c19210, "content_141c19210", fn_141c19210 as *const (), Throws::Discard), // content::BehaviorTreeParallelNode::content::BehaviorTreeParallelNode::vfunc_12
    Replacement::draft_throws(0x141c19570, "content_141c19570", fn_141c19570 as *const (), Throws::Discard), // content::BehaviorTreeParallelNode::ClassTypeInfoImplementation::content::BehaviorTreeParallelNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c19790, "content_141c19790", fn_141c19790 as *const (), Throws::Discard), // content::BehaviorTreeParallelNode::ClassTypeInfoImplementation::content::BehaviorTreeParallelNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c1a650, "content_141c1a650", fn_141c1a650 as *const (), Throws::Discard), // content::BehaviorTreeMoveOnRouteNode::ClassTypeInfoImplementation::content::BehaviorTreeMoveOnRouteNode::ClassTypeInfoIm
    Replacement::draft(0x141c1a710, "content_141c1a710", fn_141c1a710 as *const ()), // content::BehaviorTreeMoveOnRouteNode::ClassTypeInfoImplementation::content::BehaviorTreeMoveOnRouteNode::ClassTypeInfoIm
    Replacement::draft_throws(0x141c1ae80, "content_141c1ae80", fn_141c1ae80 as *const (), Throws::Discard), // content::BehaviorTreeMoveToNode::ClassTypeInfoImplementation::content::BehaviorTreeMoveToNode::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c1b650, "content_141c1b650", fn_141c1b650 as *const (), Throws::Discard), // content::BehaviorTreeMoveToFollow::ClassTypeInfoImplementation::content::BehaviorTreeMoveToFollow::ClassTypeInfoImplemen
    Replacement::draft(0x141c1b910, "content_141c1b910", fn_141c1b910 as *const ()), // content::BehaviorTreeMoveToCoverNode::content::BehaviorTreeMoveToCoverNode::vfunc_2
    Replacement::draft(0x141c1bad0, "content_141c1bad0", fn_141c1bad0 as *const ()), // content::BehaviorTreeMoveToCoverNode::ClassTypeInfoImplementation::content::BehaviorTreeMoveToCoverNode::ClassTypeInfoIm
    Replacement::draft_throws(0x141c1bf90, "content_141c1bf90", fn_141c1bf90 as *const (), Throws::Discard), // content::BehaviorTreeMoveToCoverNode::ClassTypeInfoImplementation::content::BehaviorTreeMoveToCoverNode::ClassTypeInfoIm
    Replacement::draft_throws(0x141c1c2b0, "content_141c1c2b0", tpl_38633::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeLoopNode::content::BehaviorTreeLoopNode::vfunc_12
    Replacement::draft_throws(0x141c1c600, "content_141c1c600", tpl_38634::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeLoopNode::ClassTypeInfoImplementation::content::BehaviorTreeLoopNode::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141c1c800, "content_141c1c800", tpl_38635::<0x1407cabd0, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeLoopNode::ClassTypeInfoImplementation::content::BehaviorTreeLoopNode::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141c1cac0, "content_141c1cac0", tpl_38633::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeLevitateNode::content::BehaviorTreeLevitateNode::vfunc_12
    Replacement::draft_throws(0x141c1ce50, "content_141c1ce50", tpl_38634::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeLevitateNode::ClassTypeInfoImplementation::content::BehaviorTreeLevitateNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c1d050, "content_141c1d050", tpl_38635::<0x1407cabd0, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeLevitateNode::ClassTypeInfoImplementation::content::BehaviorTreeLevitateNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c1d7c0, "content_141c1d7c0", fn_141c1d7c0 as *const (), Throws::Discard), // content::BehaviorTreeFlyToNode::ClassTypeInfoImplementation::content::BehaviorTreeFlyToNode::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141c1daa0, "content_141c1daa0", fn_141c1daa0 as *const (), Throws::Discard), // content::BehaviorTreeEquipNextWeaponNode::content::BehaviorTreeEquipNextWeaponNode::vfunc_12
    Replacement::draft_throws(0x141c1de10, "content_141c1de10", fn_141c1de10 as *const (), Throws::Discard), // content::BehaviorTreeEquipNextWeaponNode::ClassTypeInfoImplementation::content::BehaviorTreeEquipNextWeaponNode::ClassTy
    Replacement::draft_throws(0x141c1e020, "content_141c1e020", tpl_38628::<0x1407cabd0, 0x38, 0x3c> as *const (), Throws::Discard), // content::BehaviorTreeEquipNextWeaponNode::ClassTypeInfoImplementation::content::BehaviorTreeEquipNextWeaponNode::ClassTy
    Replacement::draft(0x141c1e090, "content_141c1e090", tpl_38677::<0x14325c220, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const ()), // content::BehaviorTreeEquipNextWeaponNode::ClassTypeInfoImplementation::content::BehaviorTreeEquipNextWeaponNode::ClassTy
    Replacement::draft_throws(0x141c1e7b0, "content_141c1e7b0", fn_141c1e7b0 as *const (), Throws::Discard), // content::BehaviorTreeEQSQueryNode::ClassTypeInfoImplementation::content::BehaviorTreeEQSQueryNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c1eaf0, "content_141c1eaf0", tpl_38717::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeDetachChildTreeNode::content::BehaviorTreeDetachChildTreeNode::vfunc_12
    Replacement::draft_throws(0x141c1ee40, "content_141c1ee40", tpl_38718::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeDetachChildTreeNode::ClassTypeInfoImplementation::content::BehaviorTreeDetachChildTreeNode::ClassTy
    Replacement::draft_throws(0x141c1f040, "content_141c1f040", tpl_38719::<0x1407cabd0, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeDetachChildTreeNode::ClassTypeInfoImplementation::content::BehaviorTreeDetachChildTreeNode::ClassTy
    Replacement::draft(0x141c1f0b0, "content_141c1f0b0", tpl_38720::<0x14325a7f0, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const ()), // content::BehaviorTreeDetachChildTreeNode::ClassTypeInfoImplementation::content::BehaviorTreeDetachChildTreeNode::ClassTy
    Replacement::draft_throws(0x141c1f6a0, "content_141c1f6a0", tpl_38659::<0x8, 0x10, 0x1406f2520> as *const (), Throws::Discard), // content::BehaviorTreeDecorator::ClassTypeInfoImplementation::content::BehaviorTreeDecorator::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141c202c0, "content_141c202c0", tpl_838::<0x1407a34b0> as *const (), Throws::Discard), // content::BehaviorTreeChildTreeNode::content::BehaviorTreeChildTreeNode::vfunc_10
    Replacement::draft_throws(0x141c202d0, "content_141c202d0", tpl_838::<0x1407a32b0> as *const (), Throws::Discard), // content::BehaviorTreeChildTreeNode::content::BehaviorTreeChildTreeNode::vfunc_11
    Replacement::draft_throws(0x141c202e0, "content_141c202e0", tpl_38717::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeChildTreeNode::content::BehaviorTreeChildTreeNode::vfunc_12
    Replacement::draft_throws(0x141c20630, "content_141c20630", tpl_38718::<0x8, 0x1407b4640, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeChildTreeNode::ClassTypeInfoImplementation::content::BehaviorTreeChildTreeNode::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c20820, "content_141c20820", tpl_38719::<0x1407cabd0, 0x38> as *const (), Throws::Discard), // content::BehaviorTreeChildTreeNode::ClassTypeInfoImplementation::content::BehaviorTreeChildTreeNode::ClassTypeInfoImplem
    Replacement::draft(0x141c20890, "content_141c20890", tpl_38720::<0x143259908, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38> as *const ()), // content::BehaviorTreeChildTreeNode::ClassTypeInfoImplementation::content::BehaviorTreeChildTreeNode::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c21020, "content_141c21020", fn_141c21020 as *const (), Throws::Discard), // content::BehaviorTreeAnimatedMoveToNode::ClassTypeInfoImplementation::content::BehaviorTreeAnimatedMoveToNode::ClassType
    Replacement::draft(0x141c21350, "content_141c21350", tpl_38694 as *const ()), // content::BehaviorTreeService::content::BehaviorTreeService::vfunc_5
    Replacement::draft(0x141c213a0, "content_141c213a0", tpl_38693 as *const ()), // content::BehaviorTreeService::content::BehaviorTreeService::vfunc_3
    Replacement::draft(0x141c21ad0, "content_141c21ad0", fn_141c21ad0 as *const ()), // content::UseAbilityService::ClassTypeInfoImplementation::content::UseAbilityService::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c21b30, "content_141c21b30", tpl_38723::<0x143260ca0, 0x10, 0x18> as *const ()), // content::UseAbilityService::ClassTypeInfoImplementation::content::UseAbilityService::ClassTypeInfoImplementation::vfunc_
    Replacement::draft_throws(0x141c22220, "content_141c22220", fn_141c22220 as *const (), Throws::Discard), // content::SwitchTraversalNodeMode::ClassTypeInfoImplementation::content::SwitchTraversalNodeMode::ClassTypeInfoImplementa
    Replacement::draft(0x141c222a0, "content_141c222a0", fn_141c222a0 as *const ()), // content::SwitchTraversalNodeMode::ClassTypeInfoImplementation::content::SwitchTraversalNodeMode::ClassTypeInfoImplementa
    Replacement::draft(0x141c22470, "content_141c22470", tpl_38693 as *const ()), // content::StuckDetectionService::content::StuckDetectionService::vfunc_3
    Replacement::draft(0x141c224c0, "content_141c224c0", tpl_38694 as *const ()), // content::StuckDetectionService::content::StuckDetectionService::vfunc_5
    Replacement::draft(0x141c225e0, "content_141c225e0", tpl_38693 as *const ()), // content::StuckDetectionService::ClassTypeInfoImplementation::content::StuckDetectionService::ClassTypeInfoImplementation
    Replacement::draft(0x141c22680, "content_141c22680", tpl_38694 as *const ()), // content::StuckDetectionService::ClassTypeInfoImplementation::content::StuckDetectionService::ClassTypeInfoImplementation
    Replacement::draft(0x141c22ab0, "content_141c22ab0", fn_141c22ab0 as *const ()), // content::StuckDetectionService::ClassTypeInfoImplementation::content::StuckDetectionService::ClassTypeInfoImplementation
    Replacement::draft(0x141c232f0, "content_141c232f0", fn_141c232f0 as *const ()), // content::SpeedOverrideService::ClassTypeInfoImplementation::content::SpeedOverrideService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c23540, "content_141c23540", tpl_38693 as *const ()), // content::SmartObjectService::content::SmartObjectService::vfunc_3
    Replacement::draft(0x141c23580, "content_141c23580", fn_141c23580 as *const ()), // content::SmartObjectService::content::SmartObjectService::vfunc_4
    Replacement::draft(0x141c235e0, "content_141c235e0", tpl_38694 as *const ()), // content::SmartObjectService::content::SmartObjectService::vfunc_5
    Replacement::draft(0x141c23700, "content_141c23700", tpl_38693 as *const ()), // content::SmartObjectService::ClassTypeInfoImplementation::content::SmartObjectService::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c23760, "content_141c23760", fn_141c23760 as *const ()), // content::SmartObjectService::ClassTypeInfoImplementation::content::SmartObjectService::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c237f0, "content_141c237f0", tpl_38694 as *const ()), // content::SmartObjectService::ClassTypeInfoImplementation::content::SmartObjectService::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c23c20, "content_141c23c20", fn_141c23c20 as *const ()), // content::SmartObjectService::ClassTypeInfoImplementation::content::SmartObjectService::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c23ca0, "content_141c23ca0", tpl_38731::<0x143262460, 0x70, 0x18, 0x1b, 0x1c, 0x20, 0x68, 0x78, 0x80, 0x88, 0x60, 0x6c> as *const ()), // content::SmartObjectService::ClassTypeInfoImplementation::content::SmartObjectService::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c240d0, "content_141c240d0", tpl_38732::<0x8, 0x10, 0x14> as *const ()), // content::RangedRandomInt::ClassTypeInfoImplementation::content::RangedRandomInt::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c24290, "content_141c24290", tpl_38733::<0x8, 0x10, 0x14> as *const ()), // content::RangedRandomInt::ClassTypeInfoImplementation::content::RangedRandomInt::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c246e0, "content_141c246e0", tpl_38732::<0x8, 0x10, 0x14> as *const ()), // content::RangedRandomFloat::ClassTypeInfoImplementation::content::RangedRandomFloat::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c248a0, "content_141c248a0", tpl_38733::<0x8, 0x10, 0x14> as *const ()), // content::RangedRandomFloat::ClassTypeInfoImplementation::content::RangedRandomFloat::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c24e20, "content_141c24e20", tpl_35199::<0x14387d358, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::RangedRandomBase::ClassTypeInfoImplementation::content::RangedRandomBase::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141c25070, "content_141c25070", tpl_38693 as *const ()), // content::PathFollowingService::content::PathFollowingService::vfunc_3
    Replacement::draft(0x141c250b0, "content_141c250b0", tpl_38586::<0x140763d90, 0x143426c78, 0x1426cdcb0, 0x10, 0x14325ee10, 0x14020f270> as *const ()), // content::PathFollowingService::content::PathFollowingService::vfunc_4
    Replacement::draft(0x141c25110, "content_141c25110", tpl_38694 as *const ()), // content::PathFollowingService::content::PathFollowingService::vfunc_5
    Replacement::draft(0x141c25230, "content_141c25230", tpl_38693 as *const ()), // content::PathFollowingService::ClassTypeInfoImplementation::content::PathFollowingService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c25290, "content_141c25290", tpl_38587::<0x140763d90, 0x143426c78, 0x1426cdcb0, 0x10, 0x14325ee10, 0x14020f270> as *const ()), // content::PathFollowingService::ClassTypeInfoImplementation::content::PathFollowingService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c25320, "content_141c25320", tpl_38694 as *const ()), // content::PathFollowingService::ClassTypeInfoImplementation::content::PathFollowingService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c25750, "content_141c25750", fn_141c25750 as *const ()), // content::PathFollowingService::ClassTypeInfoImplementation::content::PathFollowingService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c25990, "content_141c25990", tpl_38693 as *const ()), // content::HazardService::content::HazardService::vfunc_3
    Replacement::draft(0x141c259e0, "content_141c259e0", tpl_38694 as *const ()), // content::HazardService::content::HazardService::vfunc_5
    Replacement::draft(0x141c25b00, "content_141c25b00", tpl_38693 as *const ()), // content::HazardService::ClassTypeInfoImplementation::content::HazardService::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141c25ba0, "content_141c25ba0", tpl_38694 as *const ()), // content::HazardService::ClassTypeInfoImplementation::content::HazardService::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c25fd0, "content_141c25fd0", fn_141c25fd0 as *const ()), // content::HazardService::ClassTypeInfoImplementation::content::HazardService::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c26070, "content_141c26070", fn_141c26070 as *const ()), // content::HazardService::ClassTypeInfoImplementation::content::HazardService::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c267c0, "content_141c267c0", fn_141c267c0 as *const ()), // content::FindWorkspotService::ClassTypeInfoImplementation::content::FindWorkspotService::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c26840, "content_141c26840", tpl_38738::<0x143261520, 0x10, 0x18, 0x24> as *const ()), // content::FindWorkspotService::ClassTypeInfoImplementation::content::FindWorkspotService::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c269b0, "content_141c269b0", tpl_38693 as *const ()), // content::DisableAvoidanceService::content::DisableAvoidanceService::vfunc_3
    Replacement::draft(0x141c26a00, "content_141c26a00", tpl_38694 as *const ()), // content::DisableAvoidanceService::content::DisableAvoidanceService::vfunc_5
    Replacement::draft(0x141c26b60, "content_141c26b60", fn_141c26b60 as *const ()), // content::DisableAvoidanceService::content::DisableAvoidanceService::vfunc_14
    Replacement::draft(0x141c26cb0, "content_141c26cb0", tpl_38693 as *const ()), // content::DisableAvoidanceService::ClassTypeInfoImplementation::content::DisableAvoidanceService::ClassTypeInfoImplementa
    Replacement::draft(0x141c26d50, "content_141c26d50", tpl_38694 as *const ()), // content::DisableAvoidanceService::ClassTypeInfoImplementation::content::DisableAvoidanceService::ClassTypeInfoImplementa
    Replacement::draft(0x141c27030, "content_141c27030", fn_141c27030 as *const ()), // content::DisableAvoidanceService::ClassTypeInfoImplementation::content::DisableAvoidanceService::ClassTypeInfoImplementa
    Replacement::draft(0x141c272c0, "content_141c272c0", tpl_38695::<0x8> as *const ()), // content::DisableAvoidanceService::ClassTypeInfoImplementation::content::DisableAvoidanceService::ClassTypeInfoImplementa
    Replacement::draft(0x141c27470, "content_141c27470", tpl_38693 as *const ()), // content::CoverService::content::CoverService::vfunc_3
    Replacement::draft(0x141c274c0, "content_141c274c0", tpl_38694 as *const ()), // content::CoverService::content::CoverService::vfunc_5
    Replacement::draft(0x141c275e0, "content_141c275e0", tpl_38693 as *const ()), // content::CoverService::ClassTypeInfoImplementation::content::CoverService::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141c27680, "content_141c27680", tpl_38694 as *const ()), // content::CoverService::ClassTypeInfoImplementation::content::CoverService::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c27ab0, "content_141c27ab0", fn_141c27ab0 as *const ()), // content::CoverService::ClassTypeInfoImplementation::content::CoverService::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c27d50, "content_141c27d50", tpl_38693 as *const ()), // content::CountdownService::content::CountdownService::vfunc_3
    Replacement::draft(0x141c27da0, "content_141c27da0", tpl_38694 as *const ()), // content::CountdownService::content::CountdownService::vfunc_5
    Replacement::draft(0x141c27ec0, "content_141c27ec0", tpl_38693 as *const ()), // content::CountdownService::ClassTypeInfoImplementation::content::CountdownService::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141c27f60, "content_141c27f60", tpl_38694 as *const ()), // content::CountdownService::ClassTypeInfoImplementation::content::CountdownService::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c28390, "content_141c28390", fn_141c28390 as *const ()), // content::CountdownService::ClassTypeInfoImplementation::content::CountdownService::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c28560, "content_141c28560", tpl_38693 as *const ()), // content::AvoidanceTypeService::content::AvoidanceTypeService::vfunc_3
    Replacement::draft(0x141c285b0, "content_141c285b0", tpl_38694 as *const ()), // content::AvoidanceTypeService::content::AvoidanceTypeService::vfunc_5
    Replacement::draft(0x141c286d0, "content_141c286d0", tpl_38693 as *const ()), // content::AvoidanceTypeService::ClassTypeInfoImplementation::content::AvoidanceTypeService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c28770, "content_141c28770", tpl_38694 as *const ()), // content::AvoidanceTypeService::ClassTypeInfoImplementation::content::AvoidanceTypeService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c28ba0, "content_141c28ba0", fn_141c28ba0 as *const ()), // content::AvoidanceTypeService::ClassTypeInfoImplementation::content::AvoidanceTypeService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c29340, "content_141c29340", fn_141c29340 as *const ()), // content::AnimationModeService::ClassTypeInfoImplementation::content::AnimationModeService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c29390, "content_141c29390", tpl_38566::<0x1432601a0, 0x10> as *const ()), // content::AnimationModeService::ClassTypeInfoImplementation::content::AnimationModeService::ClassTypeInfoImplementation::
    Replacement::draft(0x141c294f0, "content_141c294f0", tpl_38693 as *const ()), // content::AimService::content::AimService::vfunc_3
    Replacement::draft(0x141c29540, "content_141c29540", tpl_38694 as *const ()), // content::AimService::content::AimService::vfunc_5
    Replacement::draft(0x141c29660, "content_141c29660", tpl_38693 as *const ()), // content::AimService::ClassTypeInfoImplementation::content::AimService::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141c29700, "content_141c29700", tpl_38694 as *const ()), // content::AimService::ClassTypeInfoImplementation::content::AimService::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c29b30, "content_141c29b30", fn_141c29b30 as *const ()), // content::AimService::ClassTypeInfoImplementation::content::AimService::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c29bb0, "content_141c29bb0", tpl_38731::<0x143261e60, 0x68, 0x10, 0x13, 0x14, 0x18, 0x60, 0x70, 0x78, 0x80, 0x58, 0x64> as *const ()), // content::AimService::ClassTypeInfoImplementation::content::AimService::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c29fd0, "content_141c29fd0", tpl_38746::<0x8, 0x1, 0x4> as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceContactSolverConfig::ClassTypeInfoImplemen
    Replacement::draft(0x141c2a020, "content_141c2a020", tpl_38747::<0x8, 0x1, 0x4> as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceContactSolverConfig::ClassTypeInfoImplemen
    Replacement::draft(0x141c2a250, "content_141c2a250", tpl_34638::<0x8> as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceContactSolverConfig::ClassTypeInfoImplemen
    Replacement::draft(0x141c2a290, "content_141c2a290", fn_141c2a290 as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceContactSolverConfig::ClassTypeInfoImplemen
    Replacement::draft(0x141c2a610, "content_141c2a610", tpl_38746::<0x8, 0x1, 0x4> as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceCapConfig::ClassTypeInfoImplementation::co
    Replacement::draft(0x141c2a660, "content_141c2a660", tpl_38747::<0x8, 0x1, 0x4> as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceCapConfig::ClassTypeInfoImplementation::co
    Replacement::draft(0x141c2a890, "content_141c2a890", tpl_34638::<0x8> as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceCapConfig::ClassTypeInfoImplementation::co
    Replacement::draft(0x141c2a8d0, "content_141c2a8d0", fn_141c2a8d0 as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceCapConfig::ClassTypeInfoImplementation::co
    Replacement::draft(0x141c2ac50, "content_141c2ac50", fn_141c2ac50 as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceUtilityFunctionConfig::ClassTypeInfoImplem
    Replacement::draft(0x141c2acc0, "content_141c2acc0", tpl_38751::<0x8, 0x4, 0xc, 0x10> as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceUtilityFunctionConfig::ClassTypeInfoImplem
    Replacement::draft(0x141c2af20, "content_141c2af20", fn_141c2af20 as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::AvoidanceUtilityFunctionConfig::ClassTypeInfoImplem
    Replacement::draft(0x141c2b460, "content_141c2b460", fn_141c2b460 as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::ClassTypeInfoImplementation::content::NavigationAvo
    Replacement::draft(0x141c2b4b0, "content_141c2b4b0", fn_141c2b4b0 as *const ()), // content::NavigationAvoidanceConfigurationComponent::AvoidanceConfig::ClassTypeInfoImplementation::content::NavigationAvo
    Replacement::draft(0x141c2b8e0, "content_141c2b8e0", tpl_38755::<0x8, 0xc, 0x14, 0x18, 0x20, 0x24, 0x2c, 0x30, 0x10> as *const ()), // content::NavmeshVolumeComponent::ChildData::ClassTypeInfoImplementation::content::NavmeshVolumeComponent::ChildData::Cla
    Replacement::draft(0x141c2b950, "content_141c2b950", tpl_38755::<0x8, 0xc, 0x14, 0x18, 0x20, 0x24, 0x2c, 0x30, 0x10> as *const ()), // content::NavmeshVolumeComponent::ChildData::ClassTypeInfoImplementation::content::NavmeshVolumeComponent::ChildData::Cla
    Replacement::draft(0x141c2bb30, "content_141c2bb30", fn_141c2bb30 as *const ()), // content::NavmeshVolumeComponent::ChildData::ClassTypeInfoImplementation::content::NavmeshVolumeComponent::ChildData::Cla
    Replacement::draft(0x141c2bbc0, "content_141c2bbc0", fn_141c2bbc0 as *const ()), // content::NavmeshVolumeComponent::ChildData::ClassTypeInfoImplementation::content::NavmeshVolumeComponent::ChildData::Cla
    Replacement::draft(0x141c2bd80, "content_141c2bd80", tpl_35205::<0x143245078, 0x1, 0x14020f270> as *const ()), // content::RouteWaypointData::ClassTypeInfoImplementation::content::RouteWaypointData::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c2bdc0, "content_141c2bdc0", tpl_35205::<0x143245078, 0x1, 0x14020f270> as *const ()), // content::RouteWaypointData::ClassTypeInfoImplementation::content::RouteWaypointData::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c2c020, "content_141c2c020", fn_141c2c020 as *const ()), // content::RouteWaypointData::ClassTypeInfoImplementation::content::RouteWaypointData::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c2c070, "content_141c2c070", fn_141c2c070 as *const ()), // content::RouteWaypointData::ClassTypeInfoImplementation::content::RouteWaypointData::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c2c1d0, "content_141c2c1d0", tpl_35199::<0x14387d340, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::RouteWaypointData::ClassTypeInfoImplementation::content::RouteWaypointData::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c2c230, "content_141c2c230", tpl_35199::<0x14387d340, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::RouteWaypointData::ClassTypeInfoImplementation::content::RouteWaypointData::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c2c2c0, "content_141c2c2c0", fn_141c2c2c0 as *const ()), // content::RouteWaypointData::ClassTypeInfoImplementation::content::RouteWaypointData::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c2c310, "content_141c2c310", fn_141c2c310 as *const ()), // content::RouteWaypointData::ClassTypeInfoImplementation::content::RouteWaypointData::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c2c410, "content_141c2c410", tpl_15058 as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141c2c450, "content_141c2c450", tpl_15058 as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_36
    Replacement::draft(0x141c2c4d0, "content_141c2c4d0", tpl_15057 as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c2c510, "content_141c2c510", tpl_15057 as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_38
    Replacement::draft(0x141c2c920, "content_141c2c920", fn_141c2c920 as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c2c980, "content_141c2c980", tpl_38763::<0x28, 0x8> as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141c2c9a0, "content_141c2c9a0", fn_141c2c9a0 as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c2c9d0, "content_141c2c9d0", tpl_38765::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x28, 0x8, 0x142442c80> as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141c2ca20, "content_141c2ca20", tpl_38766::<0x28, 0x8, 0x142442de0> as *const ()), // content::NoiseData::ClassTypeInfoImplementation::content::NoiseData::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141c2cd40, "content_141c2cd40", tpl_38767::<0x8, 0x1407f6680, 0x1407f6e80> as *const ()), // content::FirePattern::ClassTypeInfoImplementation::content::FirePattern::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c2cd90, "content_141c2cd90", tpl_38767::<0x8, 0x1407f6680, 0x1407f6e80> as *const ()), // content::FirePattern::ClassTypeInfoImplementation::content::FirePattern::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141c2cfd0, "content_141c2cfd0", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // content::FirePattern::ClassTypeInfoImplementation::content::FirePattern::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141c2d030, "content_141c2d030", tpl_35198::<0x1401de830, 0x142442de0> as *const ()), // content::FirePattern::ClassTypeInfoImplementation::content::FirePattern::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141c2d380, "content_141c2d380", tpl_38768::<0x8, 0x4, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20> as *const ()), // content::FirePatternPart::ClassTypeInfoImplementation::content::FirePatternPart::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c2d410, "content_141c2d410", tpl_38768::<0x8, 0x4, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20> as *const ()), // content::FirePatternPart::ClassTypeInfoImplementation::content::FirePatternPart::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141c2d5a0, "content_141c2d5a0", tpl_35212::<0x14387d288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FirePatternPart::ClassTypeInfoImplementation::content::FirePatternPart::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141c2d600, "content_141c2d600", tpl_35212::<0x14387d288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FirePatternPart::ClassTypeInfoImplementation::content::FirePatternPart::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141c2d6b0, "content_141c2d6b0", fn_141c2d6b0 as *const ()), // content::FirePatternPart::ClassTypeInfoImplementation::content::FirePatternPart::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c2db80, "content_141c2db80", tpl_35199::<0x14387d230, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BonsaiPropertyPath::ClassTypeInfoImplementation::content::BonsaiPropertyPath::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c2dbe0, "content_141c2dbe0", tpl_35199::<0x14387d230, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BonsaiPropertyPath::ClassTypeInfoImplementation::content::BonsaiPropertyPath::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c2dc70, "content_141c2dc70", tpl_38770::<0x14240aa00, 0x48, 0x140636a20> as *const ()), // content::BonsaiPropertyPath::ClassTypeInfoImplementation::content::BonsaiPropertyPath::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c2dcc0, "content_141c2dcc0", tpl_38771::<0x48, 0x1403b8050, 0x14240ab10> as *const ()), // content::BonsaiPropertyPath::ClassTypeInfoImplementation::content::BonsaiPropertyPath::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c2dcf0, "content_141c2dcf0", fn_141c2dcf0 as *const ()), // content::BonsaiPropertyPath::ClassTypeInfoImplementation::content::BonsaiPropertyPath::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c2dd40, "content_141c2dd40", tpl_38773::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x48, 0x1403b8050, 0x14240ab10, 0x142442c80> as *const ()), // content::BonsaiPropertyPath::ClassTypeInfoImplementation::content::BonsaiPropertyPath::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c2ddb0, "content_141c2ddb0", tpl_38774::<0x48, 0x1403b8050, 0x14240ab10, 0x142442de0> as *const ()), // content::BonsaiPropertyPath::ClassTypeInfoImplementation::content::BonsaiPropertyPath::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c2e110, "content_141c2e110", tpl_38775::<0x8, 0xc, 0x14, 0x1c, 0x24, 0x2c, 0x2d> as *const ()), // content::AITicketSetting::ClassTypeInfoImplementation::content::AITicketSetting::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c2e1a0, "content_141c2e1a0", tpl_38775::<0x8, 0xc, 0x14, 0x1c, 0x24, 0x2c, 0x2d> as *const ()), // content::AITicketSetting::ClassTypeInfoImplementation::content::AITicketSetting::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141c2e330, "content_141c2e330", tpl_38776::<0x14387d318, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AITicketSetting::ClassTypeInfoImplementation::content::AITicketSetting::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141c2e390, "content_141c2e390", tpl_38776::<0x14387d318, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AITicketSetting::ClassTypeInfoImplementation::content::AITicketSetting::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141c2e420, "content_141c2e420", tpl_38500::<0x20> as *const ()), // content::AITicketSetting::ClassTypeInfoImplementation::content::AITicketSetting::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c2e460, "content_141c2e460", fn_141c2e460 as *const ()), // content::AITicketSetting::ClassTypeInfoImplementation::content::AITicketSetting::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c2e5f0, "content_141c2e5f0", tpl_38778::<0x8, 0x10, 0x14> as *const ()), // content::RangedRandomInt::content::RangedRandomInt::vfunc_12
    Replacement::draft(0x141c2e750, "content_141c2e750", tpl_38778::<0x8, 0x10, 0x14> as *const ()), // content::RangedRandomFloat::content::RangedRandomFloat::vfunc_12
    Replacement::draft(0x141c2ead0, "content_141c2ead0", tpl_38484::<0x142442de0> as *const ()), // sub_141c2ead0
    Replacement::draft(0x141c2efc0, "content_141c2efc0", tpl_38485::<0x142442de0> as *const ()), // content::BehaviorTreeData::ClassTypeInfoImplementation::content::BehaviorTreeData::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141c2f030, "content_141c2f030", tpl_38486::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x142442de0, 0x142442c80> as *const ()), // content::BehaviorTreeData::ClassTypeInfoImplementation::content::BehaviorTreeData::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141c2f0c0, "content_141c2f0c0", tpl_38487::<0x142442de0> as *const ()), // content::BehaviorTreeData::ClassTypeInfoImplementation::content::BehaviorTreeData::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141c2f1b0, "std_141c2f1b0", tpl_550::<0x143263460, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c2f1d0, "std_141c2f1d0", tpl_550::<0x143263460, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c2f200, "std_141c2f200", tpl_1::<0x14388e990> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c2f210, "std_141c2f210", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c2f220, "std_141c2f220", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c2f240, "std_141c2f240", tpl_550::<0x1432635b0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c2f260, "std_141c2f260", tpl_550::<0x1432635b0, 0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c2f290, "std_141c2f290", tpl_1::<0x14388e930> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c2f2a0, "std_141c2f2a0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c2f2b0, "std_141c2f2b0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<coregame::TimelineResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft_throws(0x141c2f300, "coregame_141c2f300", fn_141c2f300 as *const (), Throws::Caught), // sub_141c2f300
    Replacement::draft(0x141c2f550, "coregame_141c2f550", fn_141c2f550 as *const ()), // sub_141c2f550
    Replacement::draft(0x141c2f5b0, "coregame_141c2f5b0", fn_141c2f5b0 as *const ()), // sub_141c2f5b0
    Replacement::draft(0x141c30090, "coregame_141c30090", tpl_38782::<0x1432633c0, 0xb8, 0x1408033e0> as *const ()), // coregame::TimelineResource::coregame::TimelineResource::vfunc_14
    Replacement::draft(0x141c300e0, "coregame_141c300e0", tpl_38782::<0x1432633c0, 0xb8, 0x140803390> as *const ()), // coregame::TimelineResource::coregame::TimelineResource::vfunc_16
    Replacement::draft_throws(0x141c301a0, "coregame_141c301a0", fn_141c301a0 as *const (), Throws::Discard), // coregame::TimelineResource::coregame::TimelineResource::vfunc_25
    Replacement::draft(0x141c30230, "coregame_141c30230", tpl_38784::<0x1432633c0, 0x1408033e0> as *const ()), // coregame::TimelineResource::ClassTypeInfoImplementation::coregame::TimelineResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c30270, "coregame_141c30270", tpl_38784::<0x1432633c0, 0x1408033e0> as *const ()), // coregame::TimelineResource::ClassTypeInfoImplementation::coregame::TimelineResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c302d0, "coregame_141c302d0", tpl_38784::<0x1432633c0, 0x140803390> as *const ()), // coregame::TimelineResource::ClassTypeInfoImplementation::coregame::TimelineResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c30310, "coregame_141c30310", tpl_38784::<0x1432633c0, 0x140803390> as *const ()), // coregame::TimelineResource::ClassTypeInfoImplementation::coregame::TimelineResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c30780, "coregame_141c30780", fn_141c30780 as *const ()), // coregame::TimelineResource::ClassTypeInfoImplementation::coregame::TimelineResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c307f0, "coregame_141c307f0", fn_141c307f0 as *const ()), // coregame::TimelineResource::ClassTypeInfoImplementation::coregame::TimelineResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c308a0, "coregame_141c308a0", fn_141c308a0 as *const ()), // coregame::TimelineResource::ClassTypeInfoImplementation::coregame::TimelineResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c309b0, "content_141c309b0", tpl_12760::<0x3, 0x4, 0x8, 0x1425fcae0, 0x141834e48> as *const ()), // content::TimelineSlotBinding::ClassTypeInfoImplementation::content::TimelineSlotBinding::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c30a10, "content_141c30a10", tpl_12760::<0x3, 0x4, 0x8, 0x1425fcae0, 0x141834e48> as *const ()), // content::TimelineSlotBinding::ClassTypeInfoImplementation::content::TimelineSlotBinding::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c30a70, "content_141c30a70", tpl_38788::<0x3, 0x4, 0x8, 0x1425fcb00, 0x141834e48> as *const ()), // content::TimelineSlotBinding::ClassTypeInfoImplementation::content::TimelineSlotBinding::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c30ad0, "content_141c30ad0", tpl_38788::<0x3, 0x4, 0x8, 0x1425fcb00, 0x141834e48> as *const ()), // content::TimelineSlotBinding::ClassTypeInfoImplementation::content::TimelineSlotBinding::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c30b30, "content_141c30b30", tpl_12759::<0x3, 0x4, 0x8, 0x1425fcb20, 0x141834e48> as *const ()), // content::TimelineSlotBinding::ClassTypeInfoImplementation::content::TimelineSlotBinding::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c30b90, "content_141c30b90", tpl_12759::<0x3, 0x4, 0x8, 0x1425fcb20, 0x141834e48> as *const ()), // content::TimelineSlotBinding::ClassTypeInfoImplementation::content::TimelineSlotBinding::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c30fc0, "content_141c30fc0", fn_141c30fc0 as *const ()), // content::TimelineSlotBinding::ClassTypeInfoImplementation::content::TimelineSlotBinding::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c31000, "content_141c31000", fn_141c31000 as *const ()), // content::TimelineSlotBinding::ClassTypeInfoImplementation::content::TimelineSlotBinding::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c31110, "content_141c31110", tpl_38683::<0x1432645a0, 0x60, 0x1, 0x14082b2b0> as *const ()), // content::VideoTimelineEvent::content::VideoTimelineEvent::vfunc_1
    Replacement::draft(0x141c31250, "content_141c31250", tpl_38791::<0x1432645a0, 0x1, 0x14082b2b0> as *const ()), // content::VideoTimelineEvent::ClassTypeInfoImplementation::content::VideoTimelineEvent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c316f0, "content_141c316f0", fn_141c316f0 as *const ()), // content::VideoTimelineEvent::ClassTypeInfoImplementation::content::VideoTimelineEvent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c317c0, "content_141c317c0", tpl_38793::<0x8, 0x143265db8, 0xb, 0xc, 0x10, 0x38, 0x3b, 0x3c, 0x40, 0x48, 0x60, 0x68> as *const ()), // content::VideoTimelineEvent::ClassTypeInfoImplementation::content::VideoTimelineEvent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c31ee0, "content_141c31ee0", fn_141c31ee0 as *const ()), // content::TransformTimelineEvent::ClassTypeInfoImplementation::content::TransformTimelineEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141c32000, "content_141c32000", fn_141c32000 as *const ()), // content::TransformTimelineEvent::ClassTypeInfoImplementation::content::TransformTimelineEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141c32750, "content_141c32750", fn_141c32750 as *const ()), // content::TimelineMessageStartRecording::ClassTypeInfoImplementation::content::TimelineMessageStartRecording::ClassTypeIn
    Replacement::draft(0x141c32800, "content_141c32800", fn_141c32800 as *const ()), // content::TimelineMessageStartRecording::ClassTypeInfoImplementation::content::TimelineMessageStartRecording::ClassTypeIn
    Replacement::draft(0x141c329c0, "content_141c329c0", tpl_38782::<0x143263b10, 0x80, 0x14020f270> as *const ()), // content::TimelineMessageJumpToFrame::content::TimelineMessageJumpToFrame::vfunc_2
    Replacement::draft(0x141c32a90, "content_141c32a90", fn_141c32a90 as *const ()), // content::TimelineMessageJumpToFrame::content::TimelineMessageJumpToFrame::vfunc_12
    Replacement::draft(0x141c32b80, "content_141c32b80", fn_141c32b80 as *const ()), // content::TimelineMessageJumpToFrame::ClassTypeInfoImplementation::content::TimelineMessageJumpToFrame::ClassTypeInfoImpl
    Replacement::draft(0x141c32e10, "content_141c32e10", fn_141c32e10 as *const ()), // content::TimelineMessageJumpToFrame::ClassTypeInfoImplementation::content::TimelineMessageJumpToFrame::ClassTypeInfoImpl
    Replacement::draft(0x141c33020, "content_141c33020", fn_141c33020 as *const ()), // content::TimelineMessageJumpToFrame::ClassTypeInfoImplementation::content::TimelineMessageJumpToFrame::ClassTypeInfoImpl
    Replacement::draft(0x141c330a0, "content_141c330a0", fn_141c330a0 as *const ()), // content::TimelineMessageJumpToFrame::ClassTypeInfoImplementation::content::TimelineMessageJumpToFrame::ClassTypeInfoImpl
    Replacement::draft(0x141c332d0, "content_141c332d0", tpl_38803::<0x8, 0x14240aa00, 0x38> as *const ()), // content::TimelineMessageBase::content::TimelineMessageBase::vfunc_12
    Replacement::draft(0x141c33640, "content_141c33640", tpl_38804::<0x8, 0x14240aa00, 0x38> as *const ()), // content::TimelineMessageBase::ClassTypeInfoImplementation::content::TimelineMessageBase::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c33870, "content_141c33870", tpl_38805::<0x8, 0x14240aa00, 0x38> as *const ()), // content::TimelineMessageBase::ClassTypeInfoImplementation::content::TimelineMessageBase::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c338e0, "content_141c338e0", fn_141c338e0 as *const ()), // content::TimelineMessageBase::ClassTypeInfoImplementation::content::TimelineMessageBase::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c33fd0, "content_141c33fd0", tpl_38807::<0x8, 0x14240aa00, 0x38, 0x48, 0x4c, 0x50, 0x54, 0x58> as *const ()), // content::TimelineEvent::ClassTypeInfoImplementation::content::TimelineEvent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c34060, "content_141c34060", fn_141c34060 as *const ()), // content::TimelineEvent::ClassTypeInfoImplementation::content::TimelineEvent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c34760, "content_141c34760", fn_141c34760 as *const ()), // content::TimelineTimelineEvent::ClassTypeInfoImplementation::content::TimelineTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c34810, "content_141c34810", fn_141c34810 as *const ()), // content::TimelineTimelineEvent::ClassTypeInfoImplementation::content::TimelineTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c34b10, "content_141c34b10", tpl_38811::<0x8, 0x38, 0x14240aa00, 0x48, 0x4c, 0x54, 0x50, 0x58> as *const ()), // content::SkipMarkerTimelineEvent::content::SkipMarkerTimelineEvent::vfunc_12
    Replacement::draft(0x141c34f70, "content_141c34f70", tpl_38812::<0x8, 0x38, 0x14240aa00, 0x48, 0x4c, 0x54, 0x50, 0x58> as *const ()), // content::SkipMarkerTimelineEvent::ClassTypeInfoImplementation::content::SkipMarkerTimelineEvent::ClassTypeInfoImplementa
    Replacement::draft(0x141c35200, "content_141c35200", tpl_38807::<0x8, 0x14240aa00, 0x38, 0x48, 0x4c, 0x50, 0x54, 0x58> as *const ()), // content::SkipMarkerTimelineEvent::ClassTypeInfoImplementation::content::SkipMarkerTimelineEvent::ClassTypeInfoImplementa
    Replacement::draft(0x141c35290, "content_141c35290", tpl_38813::<0x8, 0xc, 0x10, 0xb, 0x38, 0x3c, 0x40, 0x3b, 0x48, 0x143267b40> as *const ()), // content::SkipMarkerTimelineEvent::ClassTypeInfoImplementation::content::SkipMarkerTimelineEvent::ClassTypeInfoImplementa
    Replacement::draft(0x141c35990, "content_141c35990", fn_141c35990 as *const ()), // content::ScriptClipTimelineEvent::ClassTypeInfoImplementation::content::ScriptClipTimelineEvent::ClassTypeInfoImplementa
    Replacement::draft(0x141c35a30, "content_141c35a30", tpl_38815::<0x8, 0x143266f18, 0xb, 0xc, 0x10, 0x38, 0x3b, 0x3c, 0x40, 0x48, 0x60, 0x68> as *const ()), // content::ScriptClipTimelineEvent::ClassTypeInfoImplementation::content::ScriptClipTimelineEvent::ClassTypeInfoImplementa
    Replacement::draft(0x141c36130, "content_141c36130", fn_141c36130 as *const ()), // content::ScriptTimelineEvent::ClassTypeInfoImplementation::content::ScriptTimelineEvent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c361e0, "content_141c361e0", fn_141c361e0 as *const ()), // content::ScriptTimelineEvent::ClassTypeInfoImplementation::content::ScriptTimelineEvent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c363a0, "content_141c363a0", tpl_38683::<0x143263c30, 0x60, 0x1, 0x14082a150> as *const ()), // content::ParticleCommandTimelineEvent::content::ParticleCommandTimelineEvent::vfunc_1
    Replacement::draft(0x141c364e0, "content_141c364e0", tpl_38791::<0x143263c30, 0x1, 0x14082a150> as *const ()), // content::ParticleCommandTimelineEvent::ClassTypeInfoImplementation::content::ParticleCommandTimelineEvent::ClassTypeInfo
    Replacement::draft(0x141c36980, "content_141c36980", fn_141c36980 as *const ()), // content::ParticleCommandTimelineEvent::ClassTypeInfoImplementation::content::ParticleCommandTimelineEvent::ClassTypeInfo
    Replacement::draft(0x141c36a20, "content_141c36a20", tpl_38815::<0x8, 0x143266a38, 0xb, 0xc, 0x10, 0x38, 0x3b, 0x3c, 0x40, 0x48, 0x60, 0x68> as *const ()), // content::ParticleCommandTimelineEvent::ClassTypeInfoImplementation::content::ParticleCommandTimelineEvent::ClassTypeInfo
    Replacement::draft(0x141c36bb0, "content_141c36bb0", tpl_38782::<0x143263c18, 0x60, 0x14082a150> as *const ()), // content::ParticleTimelineEvent::content::ParticleTimelineEvent::vfunc_1
    Replacement::draft(0x141c36cf0, "content_141c36cf0", tpl_38819::<0x143263c18, 0x14082a150> as *const ()), // content::ParticleTimelineEvent::ClassTypeInfoImplementation::content::ParticleTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c37190, "content_141c37190", fn_141c37190 as *const ()), // content::ParticleTimelineEvent::ClassTypeInfoImplementation::content::ParticleTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c37230, "content_141c37230", tpl_38821::<0x8, 0x143266398, 0xb, 0xc, 0x10, 0x38, 0x3b, 0x3c, 0x40, 0x48, 0x60> as *const ()), // content::ParticleTimelineEvent::ClassTypeInfoImplementation::content::ParticleTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c373c0, "content_141c373c0", tpl_38683::<0x143263be8, 0xf8, 0x1, 0x140829e00> as *const ()), // content::ParameterCurveTimelineEvent::content::ParameterCurveTimelineEvent::vfunc_1
    Replacement::draft(0x141c37500, "content_141c37500", tpl_38822::<0x143263be8, 0x1, 0x140829e00> as *const ()), // content::ParameterCurveTimelineEvent::ClassTypeInfoImplementation::content::ParameterCurveTimelineEvent::ClassTypeInfoIm
    Replacement::draft(0x141c379b0, "content_141c379b0", fn_141c379b0 as *const ()), // content::ParameterCurveTimelineEvent::ClassTypeInfoImplementation::content::ParameterCurveTimelineEvent::ClassTypeInfoIm
    Replacement::draft(0x141c38160, "content_141c38160", fn_141c38160 as *const ()), // content::HideTimelineEvent::ClassTypeInfoImplementation::content::HideTimelineEvent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c38200, "content_141c38200", tpl_38813::<0x8, 0xc, 0x10, 0xb, 0x38, 0x3c, 0x40, 0x3b, 0x48, 0x143264818> as *const ()), // content::HideTimelineEvent::ClassTypeInfoImplementation::content::HideTimelineEvent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c38390, "content_141c38390", tpl_38825::<0x143263b78, 0x98, 0x143873d20, 0x1426cdcb0, 0x1401de410> as *const ()), // content::GfxGraphVariableTimelineEvent::content::GfxGraphVariableTimelineEvent::vfunc_1
    Replacement::draft(0x141c383f0, "content_141c383f0", tpl_38825::<0x143263b78, 0x98, 0x143873cd8, 0x1426cdcb0, 0x1401de410> as *const ()), // content::GfxGraphVariableTimelineEvent::content::GfxGraphVariableTimelineEvent::vfunc_2
    Replacement::draft(0x141c38450, "content_141c38450", tpl_38825::<0x143263b78, 0x98, 0x143873d40, 0x1426cdcb0, 0x1401de410> as *const ()), // content::GfxGraphVariableTimelineEvent::content::GfxGraphVariableTimelineEvent::vfunc_3
    Replacement::draft(0x141c38590, "content_141c38590", tpl_38826::<0x143873d20, 0x143263b78, 0x1426cdcb0, 0x1401de410> as *const ()), // content::GfxGraphVariableTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphVariableTimelineEvent::ClassTypeIn
    Replacement::draft(0x141c38610, "content_141c38610", tpl_38826::<0x143873cd8, 0x143263b78, 0x1426cdcb0, 0x1401de410> as *const ()), // content::GfxGraphVariableTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphVariableTimelineEvent::ClassTypeIn
    Replacement::draft(0x141c38690, "content_141c38690", tpl_38826::<0x143873d40, 0x143263b78, 0x1426cdcb0, 0x1401de410> as *const ()), // content::GfxGraphVariableTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphVariableTimelineEvent::ClassTypeIn
    Replacement::draft(0x141c38ae0, "content_141c38ae0", fn_141c38ae0 as *const ()), // content::GfxGraphVariableTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphVariableTimelineEvent::ClassTypeIn
    Replacement::draft(0x141c38b90, "content_141c38b90", fn_141c38b90 as *const ()), // content::GfxGraphVariableTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphVariableTimelineEvent::ClassTypeIn
    Replacement::draft_throws(0x141c38d80, "content_141c38d80", tpl_38829::<0x14387d4a0, 0x14387d3b0, 0x1401e9770, 0x14081e120, 0x1401e9590> as *const (), Throws::Discard), // content::GfxGraphTimelineEvent::content::GfxGraphTimelineEvent::vfunc_5
    Replacement::draft(0x141c38df0, "content_141c38df0", tpl_38830::<0x14387d4a0, 0x14387d3b0, 0x1401e9970, 0x14081dc70, 0x1401e9860> as *const ()), // content::GfxGraphTimelineEvent::content::GfxGraphTimelineEvent::vfunc_7
    Replacement::draft(0x141c38e90, "content_141c38e90", tpl_38811::<0x8, 0x38, 0x14240aa00, 0x48, 0x4c, 0x54, 0x50, 0x58> as *const ()), // content::GfxGraphTimelineEvent::content::GfxGraphTimelineEvent::vfunc_12
    Replacement::draft_throws(0x141c38ef0, "content_141c38ef0", tpl_38829::<0x14387d4a0, 0x14387d3b0, 0x1401e9b60, 0x14081c990, 0x1401e9a20> as *const (), Throws::Discard), // content::GfxGraphTimelineEvent::content::GfxGraphTimelineEvent::vfunc_13
    Replacement::draft_throws(0x141c38f60, "content_141c38f60", tpl_38829::<0x14387d4a0, 0x14387d3b0, 0x1401e9fa0, 0x14081c440, 0x1401e9e70> as *const (), Throws::Discard), // content::GfxGraphTimelineEvent::content::GfxGraphTimelineEvent::vfunc_15
    Replacement::draft_throws(0x141c390b0, "content_141c390b0", tpl_38831::<0x14387d4a0, 0x14387d3b0, 0x1401e9770, 0x14081e120, 0x1401e9590> as *const (), Throws::Discard), // content::GfxGraphTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c39170, "content_141c39170", tpl_38832::<0x14387d4a0, 0x14387d3b0, 0x1401e9970, 0x14081dc70, 0x1401e9860> as *const ()), // content::GfxGraphTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c392f0, "content_141c392f0", tpl_38812::<0x8, 0x38, 0x14240aa00, 0x48, 0x4c, 0x54, 0x50, 0x58> as *const ()), // content::GfxGraphTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141c39370, "content_141c39370", tpl_38831::<0x14387d4a0, 0x14387d3b0, 0x1401e9b60, 0x14081c990, 0x1401e9a20> as *const (), Throws::Discard), // content::GfxGraphTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft_throws(0x141c39430, "content_141c39430", tpl_38831::<0x14387d4a0, 0x14387d3b0, 0x1401e9fa0, 0x14081c440, 0x1401e9e70> as *const (), Throws::Discard), // content::GfxGraphTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c39580, "content_141c39580", tpl_38807::<0x8, 0x14240aa00, 0x38, 0x48, 0x4c, 0x50, 0x54, 0x58> as *const ()), // content::GfxGraphTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c39610, "content_141c39610", tpl_38813::<0x8, 0xc, 0x10, 0xb, 0x38, 0x3c, 0x40, 0x3b, 0x48, 0x1432648c8> as *const ()), // content::GfxGraphTimelineEvent::ClassTypeInfoImplementation::content::GfxGraphTimelineEvent::ClassTypeInfoImplementation
    Replacement::draft(0x141c39d10, "content_141c39d10", fn_141c39d10 as *const ()), // content::EarlyExitTimelineEvent::ClassTypeInfoImplementation::content::EarlyExitTimelineEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141c39da0, "content_141c39da0", fn_141c39da0 as *const ()), // content::EarlyExitTimelineEvent::ClassTypeInfoImplementation::content::EarlyExitTimelineEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141c39f30, "content_141c39f30", tpl_38683::<0x143263d40, 0x60, 0x1, 0x140828b50> as *const ()), // content::CinematicLightTimelineEvent::content::CinematicLightTimelineEvent::vfunc_1
    Replacement::draft(0x141c39f80, "content_141c39f80", tpl_38683::<0x143263d40, 0x60, 0x1, 0x140828b40> as *const ()), // content::CinematicLightTimelineEvent::content::CinematicLightTimelineEvent::vfunc_3
    Replacement::draft(0x141c3a0a0, "content_141c3a0a0", tpl_38791::<0x143263d40, 0x1, 0x140828b50> as *const ()), // content::CinematicLightTimelineEvent::ClassTypeInfoImplementation::content::CinematicLightTimelineEvent::ClassTypeInfoIm
    Replacement::draft(0x141c3a140, "content_141c3a140", tpl_38791::<0x143263d40, 0x1, 0x140828b40> as *const ()), // content::CinematicLightTimelineEvent::ClassTypeInfoImplementation::content::CinematicLightTimelineEvent::ClassTypeInfoIm
    Replacement::draft(0x141c3a570, "content_141c3a570", fn_141c3a570 as *const ()), // content::CinematicLightTimelineEvent::ClassTypeInfoImplementation::content::CinematicLightTimelineEvent::ClassTypeInfoIm
    Replacement::draft(0x141c3a790, "content_141c3a790", tpl_38782::<0x143263cf0, 0x60, 0x140827dd0> as *const ()), // content::CameraTimelineEvent::content::CameraTimelineEvent::vfunc_1
    Replacement::draft(0x141c3a8d0, "content_141c3a8d0", tpl_38819::<0x143263cf0, 0x140827dd0> as *const ()), // content::CameraTimelineEvent::ClassTypeInfoImplementation::content::CameraTimelineEvent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c3ad80, "content_141c3ad80", fn_141c3ad80 as *const ()), // content::CameraTimelineEvent::ClassTypeInfoImplementation::content::CameraTimelineEvent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c3ae40, "content_141c3ae40", tpl_38793::<0x8, 0x1432651d8, 0xb, 0xc, 0x10, 0x38, 0x3b, 0x3c, 0x40, 0x48, 0x60, 0x68> as *const ()), // content::CameraTimelineEvent::ClassTypeInfoImplementation::content::CameraTimelineEvent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c3b000, "content_141c3b000", tpl_38557::<0x40, 0x48, 0x18> as *const ()), // content::TimelineTrack::content::TimelineTrack::vfunc_3
    Replacement::draft(0x141c3b1b0, "content_141c3b1b0", tpl_38559::<0x40, 0x48, 0x18> as *const ()), // content::TimelineTrack::ClassTypeInfoImplementation::content::TimelineTrack::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c3b640, "content_141c3b640", fn_141c3b640 as *const ()), // content::TimelineTrack::ClassTypeInfoImplementation::content::TimelineTrack::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c3b7c0, "content_141c3b7c0", tpl_38559::<0x30, 0x38, 0x8> as *const ()), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141c3b820, "content_141c3b820", tpl_38559::<0x30, 0x38, 0x8> as *const ()), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_36
    Replacement::draft(0x141c3b8a0, "content_141c3b8a0", tpl_38559::<0x30, 0x38, 0x18> as *const ()), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c3b900, "content_141c3b900", tpl_38559::<0x30, 0x38, 0x18> as *const ()), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_38
    Replacement::draft_throws(0x141c3bd10, "content_141c3bd10", fn_141c3bd10 as *const (), Throws::Discard), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c3bd80, "content_141c3bd80", tpl_38839::<0x48, 0x1401de830, 0x30, 0x1401ffd50, 0x14240ab10> as *const ()), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141c3bdb0, "content_141c3bdb0", fn_141c3bdb0 as *const ()), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c3bde0, "content_141c3bde0", tpl_38841::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x48, 0x1401de830, 0x30, 0x1401ffd50, 0x14240ab10, 0x142442c80> as *const ()), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141c3be50, "content_141c3be50", tpl_38842::<0x48, 0x1401de830, 0x30, 0x1401ffd50, 0x14240ab10, 0x142442de0> as *const ()), // content::Timeline::ClassTypeInfoImplementation::content::Timeline::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141c3c430, "contentcore_141c3c430", tpl_35199::<0x14387d4a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // contentcore::ScheduleBase::ClassTypeInfoImplementation::contentcore::ScheduleBase::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141c3c520, "contentcore_141c3c520", tpl_35199::<0x14387d4a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // contentcore::ScheduleBase::ClassTypeInfoImplementation::contentcore::ScheduleBase::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141c3c700, "contentcore_141c3c700", tpl_35199::<0x14387d4a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // contentcore::ScheduleBase::ClassTypeInfoImplementation::contentcore::ScheduleBase::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141c3c7f0, "contentcore_141c3c7f0", tpl_35199::<0x14387d4a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // contentcore::ScheduleBase::ClassTypeInfoImplementation::contentcore::ScheduleBase::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141c3c870, "contentcore_141c3c870", tpl_35199::<0x14387d4a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // contentcore::ScheduleBase::ClassTypeInfoImplementation::contentcore::ScheduleBase::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft_throws(0x141c3cb80, "content_141c3cb80", tpl_38843::<0x14387d58c, 0x14387d528, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::DebugMenuGameControlReply::ClassTypeInfoImplementation::content::DebugMenuGameControlReply::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c3cbe0, "content_141c3cbe0", tpl_38843::<0x14387d58c, 0x14387d528, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::DebugMenuGameControlReply::ClassTypeInfoImplementation::content::DebugMenuGameControlReply::ClassTypeInfoImplem
    Replacement::draft(0x141c3cc80, "content_141c3cc80", tpl_38844::<0x14387d58c, 0x14387d528, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::DebugMenuGameControlReply::ClassTypeInfoImplementation::content::DebugMenuGameControlReply::ClassTypeInfoImplem
    Replacement::draft(0x141c3cce0, "content_141c3cce0", tpl_38844::<0x14387d58c, 0x14387d528, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::DebugMenuGameControlReply::ClassTypeInfoImplementation::content::DebugMenuGameControlReply::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c3d2a0, "content_141c3d2a0", tpl_38843::<0x14387d4f8, 0x14387d4f0, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::DebugMenuSetGameControl::ClassTypeInfoImplementation::content::DebugMenuSetGameControl::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141c3d300, "content_141c3d300", tpl_38843::<0x14387d4f8, 0x14387d4f0, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::DebugMenuSetGameControl::ClassTypeInfoImplementation::content::DebugMenuSetGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c3d3a0, "content_141c3d3a0", tpl_38844::<0x14387d4f8, 0x14387d4f0, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::DebugMenuSetGameControl::ClassTypeInfoImplementation::content::DebugMenuSetGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c3d400, "content_141c3d400", tpl_38844::<0x14387d4f8, 0x14387d4f0, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::DebugMenuSetGameControl::ClassTypeInfoImplementation::content::DebugMenuSetGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c3db80, "content_141c3db80", tpl_38845::<0x8, 0x14240aa00, 0x88> as *const ()), // content::DebugMenuGetGameControl::ClassTypeInfoImplementation::content::DebugMenuGetGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c3dbd0, "content_141c3dbd0", tpl_38845::<0x8, 0x14240aa00, 0x88> as *const ()), // content::DebugMenuGetGameControl::ClassTypeInfoImplementation::content::DebugMenuGetGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c3de10, "content_141c3de10", tpl_38846::<0x14240aa00, 0x88> as *const ()), // content::DebugMenuGetGameControl::ClassTypeInfoImplementation::content::DebugMenuGetGameControl::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141c3e070, "content_141c3e070", tpl_38843::<0x14387d570, 0x14387d560, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::DebugMenuExecGameControl::ClassTypeInfoImplementation::content::DebugMenuExecGameControl::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c3e0d0, "content_141c3e0d0", tpl_38843::<0x14387d570, 0x14387d560, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::DebugMenuExecGameControl::ClassTypeInfoImplementation::content::DebugMenuExecGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c3e170, "content_141c3e170", tpl_38844::<0x14387d570, 0x14387d560, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::DebugMenuExecGameControl::ClassTypeInfoImplementation::content::DebugMenuExecGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c3e1d0, "content_141c3e1d0", tpl_38844::<0x14387d570, 0x14387d560, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::DebugMenuExecGameControl::ClassTypeInfoImplementation::content::DebugMenuExecGameControl::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141c3e790, "content_141c3e790", tpl_38843::<0x14387d550, 0x14387d580, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::TweakableGameControlReply::ClassTypeInfoImplementation::content::TweakableGameControlReply::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c3e7f0, "content_141c3e7f0", tpl_38843::<0x14387d550, 0x14387d580, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::TweakableGameControlReply::ClassTypeInfoImplementation::content::TweakableGameControlReply::ClassTypeInfoImplem
    Replacement::draft(0x141c3e890, "content_141c3e890", tpl_38844::<0x14387d550, 0x14387d580, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::TweakableGameControlReply::ClassTypeInfoImplementation::content::TweakableGameControlReply::ClassTypeInfoImplem
    Replacement::draft(0x141c3e8f0, "content_141c3e8f0", tpl_38844::<0x14387d550, 0x14387d580, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::TweakableGameControlReply::ClassTypeInfoImplementation::content::TweakableGameControlReply::ClassTypeInfoImplem
    Replacement::draft(0x141c3f2c0, "content_141c3f2c0", tpl_38847::<0x4, 0x14240aa00> as *const ()), // content::TweakableGameControl::ClassTypeInfoImplementation::content::TweakableGameControl::ClassTypeInfoImplementation::
    Replacement::draft(0x141c3f300, "content_141c3f300", tpl_38848::<0x4, 0x7, 0x8, 0xc> as *const ()), // content::TweakableGameControl::ClassTypeInfoImplementation::content::TweakableGameControl::ClassTypeInfoImplementation::
    Replacement::draft(0x141c3f330, "content_141c3f330", tpl_94951::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x14240ab10, 0x142442c80> as *const ()), // content::TweakableGameControl::ClassTypeInfoImplementation::content::TweakableGameControl::ClassTypeInfoImplementation::
    Replacement::draft(0x141c3f390, "content_141c3f390", tpl_35184::<0x4, 0x14240ab10, 0x142442de0> as *const ()), // content::TweakableGameControl::ClassTypeInfoImplementation::content::TweakableGameControl::ClassTypeInfoImplementation::
    Replacement::draft(0x141c3f910, "content_141c3f910", tpl_38847::<0x4, 0x14240aa00> as *const ()), // content::CommandGameControlReply::ClassTypeInfoImplementation::content::CommandGameControlReply::ClassTypeInfoImplementa
    Replacement::draft(0x141c3f950, "content_141c3f950", tpl_38848::<0x4, 0x7, 0x8, 0xc> as *const ()), // content::CommandGameControlReply::ClassTypeInfoImplementation::content::CommandGameControlReply::ClassTypeInfoImplementa
    Replacement::draft(0x141c3f980, "content_141c3f980", tpl_94951::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x14240ab10, 0x142442c80> as *const ()), // content::CommandGameControlReply::ClassTypeInfoImplementation::content::CommandGameControlReply::ClassTypeInfoImplementa
    Replacement::draft(0x141c3f9e0, "content_141c3f9e0", tpl_35184::<0x4, 0x14240ab10, 0x142442de0> as *const ()), // content::CommandGameControlReply::ClassTypeInfoImplementation::content::CommandGameControlReply::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141c3fb50, "content_141c3fb50", tpl_38843::<0x14387d50c, 0x14387d4e8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::CommandGameControl::ClassTypeInfoImplementation::content::CommandGameControl::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141c3fbb0, "content_141c3fbb0", tpl_38843::<0x14387d50c, 0x14387d4e8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::CommandGameControl::ClassTypeInfoImplementation::content::CommandGameControl::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c3fc50, "content_141c3fc50", tpl_38844::<0x14387d50c, 0x14387d4e8, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::CommandGameControl::ClassTypeInfoImplementation::content::CommandGameControl::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c3fcb0, "content_141c3fcb0", tpl_38844::<0x14387d50c, 0x14387d4e8, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::CommandGameControl::ClassTypeInfoImplementation::content::CommandGameControl::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141c40a80, "content_141c40a80", tpl_38843::<0x14387d558, 0x14387d598, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::LayerReloadingGameControl::ClassTypeInfoImplementation::content::LayerReloadingGameControl::ClassTypeInfoImplem
    Replacement::draft_throws(0x141c40ae0, "content_141c40ae0", tpl_38843::<0x14387d558, 0x14387d598, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::LayerReloadingGameControl::ClassTypeInfoImplementation::content::LayerReloadingGameControl::ClassTypeInfoImplem
    Replacement::draft(0x141c40b80, "content_141c40b80", tpl_38844::<0x14387d558, 0x14387d598, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::LayerReloadingGameControl::ClassTypeInfoImplementation::content::LayerReloadingGameControl::ClassTypeInfoImplem
    Replacement::draft(0x141c40be0, "content_141c40be0", tpl_38844::<0x14387d558, 0x14387d598, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::LayerReloadingGameControl::ClassTypeInfoImplementation::content::LayerReloadingGameControl::ClassTypeInfoImplem
    Replacement::draft(0x141c41910, "content_141c41910", fn_141c41910 as *const ()), // content::WorldLoadingGameControl::ClassTypeInfoImplementation::content::WorldLoadingGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c41970, "content_141c41970", fn_141c41970 as *const ()), // content::WorldLoadingGameControl::ClassTypeInfoImplementation::content::WorldLoadingGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c41bc0, "content_141c41bc0", fn_141c41bc0 as *const ()), // content::WorldLoadingGameControl::ClassTypeInfoImplementation::content::WorldLoadingGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c41c20, "content_141c41c20", tpl_38852::<0x8c, 0x14240ab10, 0x4> as *const ()), // content::WorldLoadingGameControl::ClassTypeInfoImplementation::content::WorldLoadingGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c41c50, "content_141c41c50", fn_141c41c50 as *const ()), // content::WorldLoadingGameControl::ClassTypeInfoImplementation::content::WorldLoadingGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c41ca0, "content_141c41ca0", tpl_98782::<0xfffffffffffffff8, 0x8c, 0xfffffffffffffff0, 0x14240ab10, 0x4, 0x142442c80> as *const ()), // content::WorldLoadingGameControl::ClassTypeInfoImplementation::content::WorldLoadingGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c41d10, "content_141c41d10", tpl_38854::<0x8c, 0x14240ab10, 0x4, 0x142442de0> as *const ()), // content::WorldLoadingGameControl::ClassTypeInfoImplementation::content::WorldLoadingGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c41f50, "content_141c41f50", tpl_38855::<0x14387d590, 0x14387d4b8, 0x1401e9970, 0x338, 0x140715760, 0x1401e9860> as *const ()), // content::GotoPositionGameControl::ClassTypeInfoImplementation::content::GotoPositionGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c41fb0, "content_141c41fb0", tpl_38855::<0x14387d590, 0x14387d4b8, 0x1401e9970, 0x338, 0x140715760, 0x1401e9860> as *const ()), // content::GotoPositionGameControl::ClassTypeInfoImplementation::content::GotoPositionGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c42110, "content_141c42110", tpl_38856::<0x8, 0xc, 0x14, 0x18, 0x20, 0x24, 0x2c> as *const ()), // content::GotoPositionGameControl::ClassTypeInfoImplementation::content::GotoPositionGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c42160, "content_141c42160", tpl_38856::<0x8, 0xc, 0x14, 0x18, 0x20, 0x24, 0x2c> as *const ()), // content::GotoPositionGameControl::ClassTypeInfoImplementation::content::GotoPositionGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c423a0, "content_141c423a0", tpl_38857::<0x8, 0xc, 0x14, 0x18, 0x20, 0x24, 0x2c> as *const ()), // content::GotoPositionGameControl::ClassTypeInfoImplementation::content::GotoPositionGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c42410, "content_141c42410", tpl_38858::<0x143427660, 0x8, 0xc, 0x24, 0x18, 0x2c, 0x14, 0x20> as *const ()), // content::GotoPositionGameControl::ClassTypeInfoImplementation::content::GotoPositionGameControl::ClassTypeInfoImplementa
    Replacement::draft(0x141c42900, "content_141c42900", tpl_38859::<0x14387d568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GotoJumpPointGameControl::ClassTypeInfoImplementation::content::GotoJumpPointGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c42960, "content_141c42960", tpl_38859::<0x14387d568, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GotoJumpPointGameControl::ClassTypeInfoImplementation::content::GotoJumpPointGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c42a10, "content_141c42a10", fn_141c42a10 as *const ()), // content::GotoJumpPointGameControl::ClassTypeInfoImplementation::content::GotoJumpPointGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c42a90, "content_141c42a90", tpl_38861::<0x90, 0x14240ab10, 0x48> as *const ()), // content::GotoJumpPointGameControl::ClassTypeInfoImplementation::content::GotoJumpPointGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c42ad0, "content_141c42ad0", fn_141c42ad0 as *const ()), // content::GotoJumpPointGameControl::ClassTypeInfoImplementation::content::GotoJumpPointGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c42b30, "content_141c42b30", tpl_98785::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x90, 0x14240ab10, 0x48, 0x142442c80> as *const ()), // content::GotoJumpPointGameControl::ClassTypeInfoImplementation::content::GotoJumpPointGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c42bb0, "content_141c42bb0", tpl_38863::<0x90, 0x14240ab10, 0x48, 0x142442de0> as *const ()), // content::GotoJumpPointGameControl::ClassTypeInfoImplementation::content::GotoJumpPointGameControl::ClassTypeInfoImplemen
    Replacement::draft(0x141c430e0, "content_141c430e0", fn_141c430e0 as *const ()), // content::HideMeshGameControl::ClassTypeInfoImplementation::content::HideMeshGameControl::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c43130, "content_141c43130", fn_141c43130 as *const ()), // content::HideMeshGameControl::ClassTypeInfoImplementation::content::HideMeshGameControl::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c43150, "content_141c43150", tpl_35190::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // content::HideMeshGameControl::ClassTypeInfoImplementation::content::HideMeshGameControl::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c431b0, "content_141c431b0", tpl_35191::<0x8, 0x1401de830, 0x142442de0> as *const ()), // content::HideMeshGameControl::ClassTypeInfoImplementation::content::HideMeshGameControl::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c43970, "content_141c43970", tpl_38844::<0x14387d4fc, 0x14387d500, 0x1401e9970, 0x338, 0x88, 0x1401e9860> as *const ()), // content::PingGameControl::ClassTypeInfoImplementation::content::PingGameControl::ClassTypeInfoImplementation::vfunc_25
    Replacement::draft(0x141c439d0, "content_141c439d0", tpl_38844::<0x14387d4fc, 0x14387d500, 0x1401e9970, 0x338, 0x88, 0x1401e9860> as *const ()), // content::PingGameControl::ClassTypeInfoImplementation::content::PingGameControl::ClassTypeInfoImplementation::vfunc_42
    Replacement::draft_throws(0x141c43f20, "content_141c43f20", tpl_38843::<0x14387d588, 0x14387d4c0, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::GameControlReply::ClassTypeInfoImplementation::content::GameControlReply::ClassTypeInfoImplementation::vfunc_23
    Replacement::draft_throws(0x141c43f80, "content_141c43f80", tpl_38843::<0x14387d588, 0x14387d4c0, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::GameControlReply::ClassTypeInfoImplementation::content::GameControlReply::ClassTypeInfoImplementation::vfunc_40
    Replacement::draft(0x141c44020, "content_141c44020", tpl_38844::<0x14387d588, 0x14387d4c0, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::GameControlReply::ClassTypeInfoImplementation::content::GameControlReply::ClassTypeInfoImplementation::vfunc_25
    Replacement::draft(0x141c44080, "content_141c44080", tpl_38844::<0x14387d588, 0x14387d4c0, 0x1401e9970, 0x338, 0xc0, 0x1401e9860> as *const ()), // content::GameControlReply::ClassTypeInfoImplementation::content::GameControlReply::ClassTypeInfoImplementation::vfunc_42
    Replacement::draft(0x141c445a0, "std_141c445a0", tpl_550::<0x14326a478, 0x8> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c445c0, "std_141c445c0", tpl_550::<0x14326a478, 0x8> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c445f0, "std_141c445f0", tpl_1::<0x14388ea50> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c44600, "std_141c44600", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c44610, "std_141c44610", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_allo
    Replacement::draft(0x141c44630, "std_141c44630", tpl_550::<0x14326a788, 0x8> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c44650, "std_141c44650", tpl_550::<0x14326a788, 0x8> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c44680, "std_141c44680", tpl_1::<0x14388e9f0> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c44690, "std_141c44690", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c446a0, "std_141c446a0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<content::LuaScriptResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_all
    Replacement::draft(0x141c446c0, "content_141c446c0", fn_141c446c0 as *const ()), // content::LuaScriptResource::content::LuaScriptResource::vfunc_13
    Replacement::draft(0x141c44730, "content_141c44730", fn_141c44730 as *const ()), // sub_141c44730
    Replacement::draft(0x141c448b0, "content_141c448b0", tpl_38868::<0x14326a4b0, 0x14240ab10, 0x142428ce0> as *const ()), // sub_141c448b0
    Replacement::draft(0x141c448f0, "content_141c448f0", fn_141c448f0 as *const ()), // sub_141c448f0
    Replacement::draft(0x141c44a00, "content_141c44a00", tpl_38870::<0xb8, 0x8, 0x14240aa00> as *const ()), // content::LuaScriptResource::content::LuaScriptResource::vfunc_25
    Replacement::draft(0x141c44d00, "content_141c44d00", fn_141c44d00 as *const ()), // content::LuaScriptResource::ClassTypeInfoImplementation::content::LuaScriptResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c44f10, "content_141c44f10", fn_141c44f10 as *const ()), // content::LuaScriptResource::ClassTypeInfoImplementation::content::LuaScriptResource::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c45590, "content_141c45590", tpl_35199::<0x14387d5a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::LuaScriptMetadata::ClassTypeInfoImplementation::content::LuaScriptMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c45680, "content_141c45680", tpl_35199::<0x14387d5a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::LuaScriptMetadata::ClassTypeInfoImplementation::content::LuaScriptMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c45860, "content_141c45860", tpl_35199::<0x14387d5a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::LuaScriptMetadata::ClassTypeInfoImplementation::content::LuaScriptMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c45950, "content_141c45950", tpl_35199::<0x14387d5a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::LuaScriptMetadata::ClassTypeInfoImplementation::content::LuaScriptMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c459d0, "content_141c459d0", tpl_35199::<0x14387d5a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::LuaScriptMetadata::ClassTypeInfoImplementation::content::LuaScriptMetadata::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c461b0, "content_141c461b0", fn_141c461b0 as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::InplaceString<64>,r::Allocator<r::InplaceString<64>,0>>>::ClassTypeInfoImp
    Replacement::draft(0x141c46930, "content_141c46930", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x140868f20> as *const ()), // content::LuaScriptProperty<r::DynamicArray<content::ScriptReference,r::Allocator<content::ScriptReference,0>>>::ClassTyp
    Replacement::draft(0x141c470a0, "content_141c470a0", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x1403daa30> as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::GlobalID,r::Allocator<r::GlobalID,0>>>::ClassTypeInfoImplementation::conte
    Replacement::draft(0x141c472b0, "content_141c472b0", tpl_38875::<0x50, 0x140853830> as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::ResourceID,r::Allocator<r::ResourceID,0>>>::content::LuaScriptProperty<r::
    Replacement::draft(0x141c47410, "content_141c47410", fn_141c47410 as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::ResourceID,r::Allocator<r::ResourceID,0>>>::ClassTypeInfoImplementation::c
    Replacement::draft(0x141c47820, "content_141c47820", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x1403d79a0> as *const ()), // content::LuaScriptProperty<r::DynamicArray<r::ResourceID,r::Allocator<r::ResourceID,0>>>::ClassTypeInfoImplementation::c
    Replacement::draft(0x141c47f80, "content_141c47f80", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x140655c70> as *const ()), // content::LuaScriptProperty<r::DynamicArray<m::Vector4Template<float>,r::Allocator<m::Vector4Template<float>,0>>>::ClassT
    Replacement::draft(0x141c486e0, "content_141c486e0", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x140868e20> as *const ()), // content::LuaScriptProperty<r::DynamicArray<m::Vector3Template<float>,r::Allocator<m::Vector3Template<float>,0>>>::ClassT
    Replacement::draft(0x141c48e40, "content_141c48e40", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x1403d79a0> as *const ()), // content::LuaScriptProperty<r::DynamicArray<m::Vector2Template<float>,r::Allocator<m::Vector2Template<float>,0>>>::ClassT
    Replacement::draft(0x141c495a0, "content_141c495a0", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x140265ba0> as *const ()), // content::LuaScriptProperty<r::DynamicArray<float,r::Allocator<float,0>>>::ClassTypeInfoImplementation::content::LuaScrip
    Replacement::draft(0x141c49d00, "content_141c49d00", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x140265ba0> as *const ()), // content::LuaScriptProperty<r::DynamicArray<unsigned int,r::Allocator<unsigned int,0>>>::ClassTypeInfoImplementation::con
    Replacement::draft(0x141c4a460, "content_141c4a460", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x140265ba0> as *const ()), // content::LuaScriptProperty<r::DynamicArray<int,r::Allocator<int,0>>>::ClassTypeInfoImplementation::content::LuaScriptPro
    Replacement::draft(0x141c4a700, "content_141c4a700", fn_141c4a700 as *const ()), // content::LuaScriptProperty<r::DynamicArray<bool,r::Allocator<bool,0>>>::content::LuaScriptProperty<r::DynamicArray<bool,
    Replacement::draft(0x141c4aa40, "content_141c4aa40", fn_141c4aa40 as *const ()), // content::LuaScriptProperty<r::DynamicArray<bool,r::Allocator<bool,0>>>::ClassTypeInfoImplementation::content::LuaScriptP
    Replacement::draft(0x141c4ac50, "content_141c4ac50", tpl_38874::<0x8, 0x14240aa00, 0x50, 0x1402bf220> as *const ()), // content::LuaScriptProperty<r::DynamicArray<bool,r::Allocator<bool,0>>>::ClassTypeInfoImplementation::content::LuaScriptP
    Replacement::draft(0x141c4aef0, "content_141c4aef0", fn_141c4aef0 as *const ()), // content::LuaScriptProperty<r::InplaceString<64>>::content::LuaScriptProperty<r::InplaceString<64>>::vfunc_13
    Replacement::draft(0x141c4b220, "content_141c4b220", fn_141c4b220 as *const ()), // content::LuaScriptProperty<r::InplaceString<64>>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::InplaceStri
    Replacement::draft(0x141c4b420, "content_141c4b420", tpl_38805::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<r::InplaceString<64>>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::InplaceStri
    Replacement::draft(0x141c4b490, "content_141c4b490", fn_141c4b490 as *const ()), // content::LuaScriptProperty<r::InplaceString<64>>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::InplaceStri
    Replacement::draft(0x141c4b610, "content_141c4b610", tpl_38683::<0x143251120, 0x50, 0x1, 0x14086ad00> as *const ()), // content::LuaScriptProperty<content::ScriptReference>::content::LuaScriptProperty<content::ScriptReference>::vfunc_2
    Replacement::draft(0x141c4b6f0, "content_141c4b6f0", tpl_38882::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<content::ScriptReference>::content::LuaScriptProperty<content::ScriptReference>::vfunc_13
    Replacement::draft(0x141c4b790, "content_141c4b790", tpl_38883::<0x143251120, 0x1, 0x14086ad00> as *const ()), // content::LuaScriptProperty<content::ScriptReference>::ClassTypeInfoImplementation::content::LuaScriptProperty<content::S
    Replacement::draft(0x141c4ba50, "content_141c4ba50", tpl_38884::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<content::ScriptReference>::ClassTypeInfoImplementation::content::LuaScriptProperty<content::S
    Replacement::draft(0x141c4bc50, "content_141c4bc50", tpl_38885::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<content::ScriptReference>::ClassTypeInfoImplementation::content::LuaScriptProperty<content::S
    Replacement::draft(0x141c4be30, "content_141c4be30", tpl_38683::<0x143251120, 0x50, 0x1, 0x14028a830> as *const ()), // content::LuaScriptProperty<r::GlobalID>::content::LuaScriptProperty<r::GlobalID>::vfunc_2
    Replacement::draft(0x141c4bf10, "content_141c4bf10", tpl_38882::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<r::GlobalID>::content::LuaScriptProperty<r::GlobalID>::vfunc_13
    Replacement::draft(0x141c4bfb0, "content_141c4bfb0", tpl_38883::<0x143251120, 0x1, 0x14028a830> as *const ()), // content::LuaScriptProperty<r::GlobalID>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::GlobalID>::ClassType
    Replacement::draft(0x141c4c270, "content_141c4c270", tpl_38884::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<r::GlobalID>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::GlobalID>::ClassType
    Replacement::draft(0x141c4c470, "content_141c4c470", tpl_38885::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<r::GlobalID>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::GlobalID>::ClassType
    Replacement::draft(0x141c4c670, "content_141c4c670", fn_141c4c670 as *const ()), // content::LuaScriptProperty<r::ResourceID>::content::LuaScriptProperty<r::ResourceID>::vfunc_4
    Replacement::draft(0x141c4c750, "content_141c4c750", tpl_38882::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<r::ResourceID>::content::LuaScriptProperty<r::ResourceID>::vfunc_13
    Replacement::draft(0x141c4c850, "content_141c4c850", tpl_38887::<0x50, 0x20, 0x8, 0xc, 0x14020bcb0> as *const ()), // content::LuaScriptProperty<r::ResourceID>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::ResourceID>::Class
    Replacement::draft(0x141c4cad0, "content_141c4cad0", tpl_38884::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<r::ResourceID>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::ResourceID>::Class
    Replacement::draft(0x141c4ccd0, "content_141c4ccd0", tpl_38885::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<r::ResourceID>::ClassTypeInfoImplementation::content::LuaScriptProperty<r::ResourceID>::Class
    Replacement::draft_throws(0x141c4cec0, "content_141c4cec0", tpl_38888::<0x143251120, 0x50, 0x1, 0x14086a110> as *const (), Throws::Discard), // content::LuaScriptProperty<m::Vector4Template<float>>::content::LuaScriptProperty<m::Vector4Template<float>>::vfunc_3
    Replacement::draft(0x141c4cf90, "content_141c4cf90", fn_141c4cf90 as *const ()), // content::LuaScriptProperty<m::Vector4Template<float>>::content::LuaScriptProperty<m::Vector4Template<float>>::vfunc_13
    Replacement::draft_throws(0x141c4d060, "content_141c4d060", tpl_38890::<0x143251120, 0x1, 0x14086a110> as *const (), Throws::Discard), // content::LuaScriptProperty<m::Vector4Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft(0x141c4d2f0, "content_141c4d2f0", fn_141c4d2f0 as *const ()), // content::LuaScriptProperty<m::Vector4Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft(0x141c4d4f0, "content_141c4d4f0", fn_141c4d4f0 as *const ()), // content::LuaScriptProperty<m::Vector4Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft_throws(0x141c4d6f0, "content_141c4d6f0", tpl_38888::<0x143251120, 0x50, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // content::LuaScriptProperty<m::Vector3Template<float>>::content::LuaScriptProperty<m::Vector3Template<float>>::vfunc_3
    Replacement::draft(0x141c4d7c0, "content_141c4d7c0", fn_141c4d7c0 as *const ()), // content::LuaScriptProperty<m::Vector3Template<float>>::content::LuaScriptProperty<m::Vector3Template<float>>::vfunc_13
    Replacement::draft_throws(0x141c4d890, "content_141c4d890", tpl_38890::<0x143251120, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // content::LuaScriptProperty<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft(0x141c4db20, "content_141c4db20", fn_141c4db20 as *const ()), // content::LuaScriptProperty<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft(0x141c4dd20, "content_141c4dd20", tpl_38895::<0x8, 0x14240aa00, 0x50, 0x58> as *const ()), // content::LuaScriptProperty<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft(0x141c4dd90, "content_141c4dd90", fn_141c4dd90 as *const ()), // content::LuaScriptProperty<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft_throws(0x141c4df20, "content_141c4df20", tpl_38888::<0x143251120, 0x50, 0x1, 0x1402c61c0> as *const (), Throws::Discard), // content::LuaScriptProperty<m::Vector2Template<float>>::content::LuaScriptProperty<m::Vector2Template<float>>::vfunc_3
    Replacement::draft(0x141c4dff0, "content_141c4dff0", tpl_38882::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<m::Vector2Template<float>>::content::LuaScriptProperty<m::Vector2Template<float>>::vfunc_13
    Replacement::draft_throws(0x141c4e0c0, "content_141c4e0c0", tpl_38890::<0x143251120, 0x1, 0x1402c61c0> as *const (), Throws::Discard), // content::LuaScriptProperty<m::Vector2Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft(0x141c4e350, "content_141c4e350", tpl_38884::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<m::Vector2Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft(0x141c4e550, "content_141c4e550", tpl_38885::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<m::Vector2Template<float>>::ClassTypeInfoImplementation::content::LuaScriptProperty<m::Vector
    Replacement::draft(0x141c4e740, "content_141c4e740", tpl_38683::<0x143251120, 0x50, 0x1, 0x14020f270> as *const ()), // content::LuaScriptProperty<float>::content::LuaScriptProperty<float>::vfunc_3
    Replacement::draft(0x141c4e810, "content_141c4e810", fn_141c4e810 as *const ()), // content::LuaScriptProperty<float>::content::LuaScriptProperty<float>::vfunc_13
    Replacement::draft(0x141c4e8d0, "content_141c4e8d0", tpl_38883::<0x143251120, 0x1, 0x14020f270> as *const ()), // content::LuaScriptProperty<float>::ClassTypeInfoImplementation::content::LuaScriptProperty<float>::ClassTypeInfoImplemen
    Replacement::draft(0x141c4eb60, "content_141c4eb60", fn_141c4eb60 as *const ()), // content::LuaScriptProperty<float>::ClassTypeInfoImplementation::content::LuaScriptProperty<float>::ClassTypeInfoImplemen
    Replacement::draft(0x141c4ed50, "content_141c4ed50", tpl_38534::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<float>::ClassTypeInfoImplementation::content::LuaScriptProperty<float>::ClassTypeInfoImplemen
    Replacement::draft(0x141c4efe0, "content_141c4efe0", tpl_38899::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<unsigned int>::content::LuaScriptProperty<unsigned int>::vfunc_13
    Replacement::draft(0x141c4f300, "content_141c4f300", tpl_38900::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<unsigned int>::ClassTypeInfoImplementation::content::LuaScriptProperty<unsigned int>::ClassTy
    Replacement::draft(0x141c4f4f0, "content_141c4f4f0", tpl_38534::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<unsigned int>::ClassTypeInfoImplementation::content::LuaScriptProperty<unsigned int>::ClassTy
    Replacement::draft(0x141c4f780, "content_141c4f780", tpl_38899::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<int>::content::LuaScriptProperty<int>::vfunc_13
    Replacement::draft(0x141c4faa0, "content_141c4faa0", tpl_38900::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<int>::ClassTypeInfoImplementation::content::LuaScriptProperty<int>::ClassTypeInfoImplementati
    Replacement::draft(0x141c4fc90, "content_141c4fc90", tpl_38534::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<int>::ClassTypeInfoImplementation::content::LuaScriptProperty<int>::ClassTypeInfoImplementati
    Replacement::draft(0x141c4ff20, "content_141c4ff20", fn_141c4ff20 as *const ()), // content::LuaScriptProperty<bool>::content::LuaScriptProperty<bool>::vfunc_13
    Replacement::draft(0x141c50250, "content_141c50250", fn_141c50250 as *const ()), // content::LuaScriptProperty<bool>::ClassTypeInfoImplementation::content::LuaScriptProperty<bool>::ClassTypeInfoImplementa
    Replacement::draft(0x141c50450, "content_141c50450", tpl_38903::<0x8, 0x14240aa00, 0x50> as *const ()), // content::LuaScriptProperty<bool>::ClassTypeInfoImplementation::content::LuaScriptProperty<bool>::ClassTypeInfoImplementa
    Replacement::draft(0x141c50b70, "content_141c50b70", tpl_35199::<0x14387d5c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::LuaScriptPropertyBase::ClassTypeInfoImplementation::content::LuaScriptPropertyBase::ClassTypeInfoImplementation
    Replacement::draft(0x141c511d0, "content_141c511d0", tpl_35199::<0x14387d6e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ScriptReference::ClassTypeInfoImplementation::content::ScriptReference::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141c51230, "content_141c51230", tpl_35199::<0x14387d6e0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ScriptReference::ClassTypeInfoImplementation::content::ScriptReference::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141c51410, "content_141c51410", fn_141c51410 as *const ()), // sub_141c51410
    Replacement::draft(0x141c51430, "content_141c51430", fn_141c51430 as *const ()), // sub_141c51430
    Replacement::draft(0x141c51560, "content_141c51560", fn_141c51560 as *const ()), // content::LuaScriptComponent::content::LuaScriptComponent::vfunc_18
    Replacement::draft(0x141c519e0, "content_141c519e0", fn_141c519e0 as *const ()), // content::LuaEventHandler::ClassTypeInfoImplementation::content::LuaEventHandler::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c51a20, "content_141c51a20", fn_141c51a20 as *const ()), // content::LuaEventHandler::ClassTypeInfoImplementation::content::LuaEventHandler::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141c51b60, "content_141c51b60", tpl_35199::<0x14387d708, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::LuaEventHandler::ClassTypeInfoImplementation::content::LuaEventHandler::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141c51bc0, "content_141c51bc0", tpl_35199::<0x14387d708, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::LuaEventHandler::ClassTypeInfoImplementation::content::LuaEventHandler::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141c51c70, "content_141c51c70", fn_141c51c70 as *const ()), // content::LuaEventHandler::ClassTypeInfoImplementation::content::LuaEventHandler::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c51cb0, "content_141c51cb0", fn_141c51cb0 as *const ()), // content::LuaEventHandler::ClassTypeInfoImplementation::content::LuaEventHandler::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c51ce0, "content_141c51ce0", tpl_98818::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x14240ab10, 0x142442c80> as *const ()), // content::LuaEventHandler::ClassTypeInfoImplementation::content::LuaEventHandler::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141c51d40, "content_141c51d40", tpl_35184::<0xc, 0x14240ab10, 0x142442de0> as *const ()), // content::LuaEventHandler::ClassTypeInfoImplementation::content::LuaEventHandler::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141c51e60, "content_141c51e60", tpl_365::<0x14088a870> as *const ()), // content::GfxGraphValue<content::gfxgraph::EntityTag>::content::GfxGraphValue<content::gfxgraph::EntityTag>::vfunc_6
    Replacement::draft(0x141c52550, "content_141c52550", tpl_38683::<0x143251120, 0x8, 0x1, 0x140890960> as *const ()), // content::GfxGraphValue<content::gfxgraph::ContentShaderRef>::content::GfxGraphValue<content::gfxgraph::ContentShaderRef>
    Replacement::draft(0x141c52610, "content_141c52610", tpl_38911::<0x8> as *const ()), // content::GfxGraphValue<content::gfxgraph::ContentShaderRef>::content::GfxGraphValue<content::gfxgraph::ContentShaderRef>
    Replacement::draft(0x141c526f0, "content_141c526f0", tpl_34635::<0x143251120, 0x1, 0x140890960> as *const ()), // content::GfxGraphValue<content::gfxgraph::ContentShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValue<content
    Replacement::draft(0x141c52950, "content_141c52950", tpl_38912::<0x8, 0x10> as *const ()), // content::GfxGraphValue<content::gfxgraph::ContentShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValue<content
    Replacement::draft_throws(0x141c52d00, "content_141c52d00", tpl_838::<0x140889240> as *const (), Throws::Discard), // content::GfxGraphValue<content::gfxgraph::EngineShaderRef>::content::GfxGraphValue<content::gfxgraph::EngineShaderRef>::
    Replacement::draft(0x141c52d80, "content_141c52d80", tpl_38911::<0x8> as *const ()), // content::GfxGraphValue<content::gfxgraph::EngineShaderRef>::content::GfxGraphValue<content::gfxgraph::EngineShaderRef>::
    Replacement::draft(0x141c53090, "content_141c53090", tpl_38912::<0x8, 0x10> as *const ()), // content::GfxGraphValue<content::gfxgraph::EngineShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValue<content:
    Replacement::draft(0x141c53410, "content_141c53410", tpl_38683::<0x143251120, 0x8, 0x1, 0x14088eb50> as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::content::GfxGraphValue<content::gfxgraph::Temporar
    Replacement::draft(0x141c53460, "content_141c53460", tpl_38683::<0x143251120, 0x8, 0x1, 0x14088eb40> as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::content::GfxGraphValue<content::gfxgraph::Temporar
    Replacement::draft(0x141c53520, "content_141c53520", fn_141c53520 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::content::GfxGraphValue<content::gfxgraph::Temporar
    Replacement::draft(0x141c535a0, "content_141c535a0", tpl_34635::<0x143251120, 0x1, 0x14088eb50> as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValu
    Replacement::draft(0x141c53630, "content_141c53630", tpl_34635::<0x143251120, 0x1, 0x14088eb40> as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValu
    Replacement::draft(0x141c53890, "content_141c53890", fn_141c53890 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValu
    Replacement::draft(0x141c53a70, "content_141c53a70", tpl_35082::<0x8, 0x10, 0x14240aa00> as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValu
    Replacement::draft(0x141c53ac0, "content_141c53ac0", fn_141c53ac0 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryContentShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValu
    Replacement::draft(0x141c53ce0, "content_141c53ce0", fn_141c53ce0 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryEngineShaderRef>::content::GfxGraphValue<content::gfxgraph::Temporary
    Replacement::draft(0x141c54000, "content_141c54000", fn_141c54000 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryEngineShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValue
    Replacement::draft(0x141c541f0, "content_141c541f0", fn_141c541f0 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryEngineShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValue
    Replacement::draft(0x141c54260, "content_141c54260", tpl_38919::<0x14326f538, 0x8, 0xc, 0x10, 0xb, 0x50, 0x54, 0x58, 0x53> as *const ()), // content::GfxGraphValue<content::gfxgraph::TemporaryEngineShaderRef>::ClassTypeInfoImplementation::content::GfxGraphValue
    Replacement::draft(0x141c54490, "content_141c54490", fn_141c54490 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemplatePinInfo>::content::GfxGraphValue<content::gfxgraph::TemplatePinInfo>::
    Replacement::draft(0x141c547a0, "content_141c547a0", fn_141c547a0 as *const ()), // content::GfxGraphValue<content::gfxgraph::TemplatePinInfo>::ClassTypeInfoImplementation::content::GfxGraphValue<content:
    Replacement::draft(0x141c54bc0, "content_141c54bc0", fn_141c54bc0 as *const ()), // content::GfxGraphValue<content::gfxgraph::GatherToken>::content::GfxGraphValue<content::gfxgraph::GatherToken>::vfunc_12
    Replacement::draft(0x141c54ed0, "content_141c54ed0", fn_141c54ed0 as *const ()), // content::GfxGraphValue<content::gfxgraph::GatherToken>::ClassTypeInfoImplementation::content::GfxGraphValue<content::gfx
    Replacement::draft(0x141c560b0, "content_141c560b0", tpl_38683::<0x143251120, 0x8, 0x1, 0x14028a830> as *const ()), // content::GfxGraphValue<r::GlobalID>::content::GfxGraphValue<r::GlobalID>::vfunc_1
    Replacement::draft(0x141c56200, "content_141c56200", tpl_34635::<0x143251120, 0x1, 0x14028a830> as *const ()), // content::GfxGraphValue<r::GlobalID>::ClassTypeInfoImplementation::content::GfxGraphValue<r::GlobalID>::ClassTypeInfoImpl
    Replacement::draft(0x141c56850, "content_141c56850", tpl_38924::<0x8, 0x20, 0xc, 0x14020bcb0> as *const ()), // content::GfxGraphValue<r::ResourceID>::content::GfxGraphValue<r::ResourceID>::vfunc_3
    Replacement::draft(0x141c56a00, "content_141c56a00", tpl_38925::<0x8, 0x20, 0xc, 0x14020bcb0> as *const ()), // content::GfxGraphValue<r::ResourceID>::ClassTypeInfoImplementation::content::GfxGraphValue<r::ResourceID>::ClassTypeInfo
    Replacement::draft(0x141c56ff0, "content_141c56ff0", tpl_38683::<0x143251120, 0x8, 0x1, 0x14088ba10> as *const ()), // content::GfxGraphValue<r::GlobalIDRef<content::Curve>>::content::GfxGraphValue<r::GlobalIDRef<content::Curve>>::vfunc_1
    Replacement::draft(0x141c57140, "content_141c57140", tpl_34635::<0x143251120, 0x1, 0x14088ba10> as *const ()), // content::GfxGraphValue<r::GlobalIDRef<content::Curve>>::ClassTypeInfoImplementation::content::GfxGraphValue<r::GlobalIDR
    Replacement::draft(0x141c57790, "content_141c57790", tpl_38683::<0x143251120, 0x8, 0x1, 0x14088ba00> as *const ()), // content::GfxGraphValue<r::ResourceRef<10>>::content::GfxGraphValue<r::ResourceRef<10>>::vfunc_1
    Replacement::draft(0x141c577e0, "content_141c577e0", tpl_38926::<0x8, 0x140728030> as *const ()), // content::GfxGraphValue<r::ResourceRef<10>>::content::GfxGraphValue<r::ResourceRef<10>>::vfunc_3
    Replacement::draft(0x141c578f0, "content_141c578f0", tpl_34635::<0x143251120, 0x1, 0x14088ba00> as *const ()), // content::GfxGraphValue<r::ResourceRef<10>>::ClassTypeInfoImplementation::content::GfxGraphValue<r::ResourceRef<10>>::Cla
    Replacement::draft(0x141c57980, "content_141c57980", fn_141c57980 as *const ()), // content::GfxGraphValue<r::ResourceRef<10>>::ClassTypeInfoImplementation::content::GfxGraphValue<r::ResourceRef<10>>::Cla
    Replacement::draft(0x141c57f30, "content_141c57f30", tpl_38683::<0x143251120, 0x8, 0x1, 0x14088b6b0> as *const ()), // content::GfxGraphValue<r::ResourceRef<53>>::content::GfxGraphValue<r::ResourceRef<53>>::vfunc_1
    Replacement::draft(0x141c58080, "content_141c58080", tpl_34635::<0x143251120, 0x1, 0x14088b6b0> as *const ()), // content::GfxGraphValue<r::ResourceRef<53>>::ClassTypeInfoImplementation::content::GfxGraphValue<r::ResourceRef<53>>::Cla
    Replacement::draft(0x141c586c0, "content_141c586c0", tpl_38683::<0x143251120, 0x8, 0x1, 0x14088b360> as *const ()), // content::GfxGraphValue<r::ResourceRef<7>>::content::GfxGraphValue<r::ResourceRef<7>>::vfunc_1
    Replacement::draft(0x141c58810, "content_141c58810", tpl_34635::<0x143251120, 0x1, 0x14088b360> as *const ()), // content::GfxGraphValue<r::ResourceRef<7>>::ClassTypeInfoImplementation::content::GfxGraphValue<r::ResourceRef<7>>::Class
    Replacement::draft(0x141c58e50, "content_141c58e50", tpl_38683::<0x143251120, 0x8, 0x1, 0x14082b2b0> as *const ()), // content::GfxGraphValue<r::ResourceRef<60>>::content::GfxGraphValue<r::ResourceRef<60>>::vfunc_1
    Replacement::draft(0x141c58fa0, "content_141c58fa0", tpl_34635::<0x143251120, 0x1, 0x14082b2b0> as *const ()), // content::GfxGraphValue<r::ResourceRef<60>>::ClassTypeInfoImplementation::content::GfxGraphValue<r::ResourceRef<60>>::Cla
    Replacement::draft_throws(0x141c59d30, "content_141c59d30", tpl_38888::<0x143251120, 0x8, 0x1, 0x14086a110> as *const (), Throws::Discard), // content::GfxGraphValue<m::Vector4Template<float>>::content::GfxGraphValue<m::Vector4Template<float>>::vfunc_2
    Replacement::draft_throws(0x141c59ea0, "content_141c59ea0", tpl_38928::<0x143251120, 0x1, 0x14086a110> as *const (), Throws::Discard), // content::GfxGraphValue<m::Vector4Template<float>>::ClassTypeInfoImplementation::content::GfxGraphValue<m::Vector4Templat
    Replacement::draft_throws(0x141c5a4c0, "content_141c5a4c0", tpl_38888::<0x143251120, 0x8, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // content::GfxGraphValue<m::Vector3Template<float>>::content::GfxGraphValue<m::Vector3Template<float>>::vfunc_2
    Replacement::draft(0x141c5a590, "content_141c5a590", tpl_38929::<0x8, 0x10> as *const ()), // content::GfxGraphValue<m::Vector3Template<float>>::content::GfxGraphValue<m::Vector3Template<float>>::vfunc_12
    Replacement::draft_throws(0x141c5a640, "content_141c5a640", tpl_38928::<0x143251120, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // content::GfxGraphValue<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::GfxGraphValue<m::Vector3Templat
    Replacement::draft(0x141c5a8d0, "content_141c5a8d0", tpl_38930::<0x8, 0x10> as *const ()), // content::GfxGraphValue<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::GfxGraphValue<m::Vector3Templat
    Replacement::draft(0x141c5aaa0, "content_141c5aaa0", tpl_38931::<0x8, 0x10> as *const ()), // content::GfxGraphValue<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::GfxGraphValue<m::Vector3Templat
    Replacement::draft_throws(0x141c5ac60, "content_141c5ac60", tpl_38888::<0x143251120, 0x8, 0x1, 0x1402c61c0> as *const (), Throws::Discard), // content::GfxGraphValue<m::Vector2Template<float>>::content::GfxGraphValue<m::Vector2Template<float>>::vfunc_2
    Replacement::draft_throws(0x141c5add0, "content_141c5add0", tpl_38928::<0x143251120, 0x1, 0x1402c61c0> as *const (), Throws::Discard), // content::GfxGraphValue<m::Vector2Template<float>>::ClassTypeInfoImplementation::content::GfxGraphValue<m::Vector2Templat
    Replacement::draft(0x141c5b3e0, "content_141c5b3e0", tpl_38683::<0x143251120, 0x8, 0x1, 0x14020f270> as *const ()), // content::GfxGraphValue<float>::content::GfxGraphValue<float>::vfunc_2
    Replacement::draft(0x141c5b550, "content_141c5b550", tpl_34635::<0x143251120, 0x1, 0x14020f270> as *const ()), // content::GfxGraphValue<float>::ClassTypeInfoImplementation::content::GfxGraphValue<float>::ClassTypeInfoImplementation::
    Replacement::draft(0x141c5b9b0, "content_141c5b9b0", tpl_38695::<0x8> as *const ()), // content::GfxGraphValue<float>::ClassTypeInfoImplementation::content::GfxGraphValue<float>::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141c5bb90, "content_141c5bb90", tpl_838::<0x14087b740> as *const (), Throws::Discard), // content::GfxGraphValue<m::Vector4Template<int>>::content::GfxGraphValue<m::Vector4Template<int>>::vfunc_4
    Replacement::draft(0x141c5c340, "content_141c5c340", tpl_38929::<0x8, 0x10> as *const ()), // content::GfxGraphValue<m::Vector3Template<int>>::content::GfxGraphValue<m::Vector3Template<int>>::vfunc_12
    Replacement::draft(0x141c5c650, "content_141c5c650", tpl_38930::<0x8, 0x10> as *const ()), // content::GfxGraphValue<m::Vector3Template<int>>::ClassTypeInfoImplementation::content::GfxGraphValue<m::Vector3Template<
    Replacement::draft(0x141c5c820, "content_141c5c820", tpl_38931::<0x8, 0x10> as *const ()), // content::GfxGraphValue<m::Vector3Template<int>>::ClassTypeInfoImplementation::content::GfxGraphValue<m::Vector3Template<
    Replacement::draft(0x141c5d870, "content_141c5d870", tpl_365::<0x140877e80> as *const ()), // content::GfxGraphValue<bool>::content::GfxGraphValue<bool>::vfunc_7
    Replacement::draft(0x141c5e410, "content_141c5e410", tpl_35199::<0x14387d870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphValueBase::ClassTypeInfoImplementation::content::GfxGraphValueBase::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c5e500, "content_141c5e500", tpl_35199::<0x14387d870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphValueBase::ClassTypeInfoImplementation::content::GfxGraphValueBase::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c5e6e0, "content_141c5e6e0", tpl_35199::<0x14387d870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphValueBase::ClassTypeInfoImplementation::content::GfxGraphValueBase::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c5e7d0, "content_141c5e7d0", tpl_35199::<0x14387d870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphValueBase::ClassTypeInfoImplementation::content::GfxGraphValueBase::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c5e850, "content_141c5e850", tpl_35199::<0x14387d870, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphValueBase::ClassTypeInfoImplementation::content::GfxGraphValueBase::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c5eef0, "content_141c5eef0", fn_141c5eef0 as *const ()), // content::gfxgraph::GraphDebugInfo::ClassTypeInfoImplementation::content::gfxgraph::GraphDebugInfo::ClassTypeInfoImplemen
    Replacement::draft(0x141c5ef50, "content_141c5ef50", fn_141c5ef50 as *const ()), // content::gfxgraph::GraphDebugInfo::ClassTypeInfoImplementation::content::gfxgraph::GraphDebugInfo::ClassTypeInfoImplemen
    Replacement::draft(0x141c5efb0, "content_141c5efb0", fn_141c5efb0 as *const ()), // content::gfxgraph::GraphDebugInfo::ClassTypeInfoImplementation::content::gfxgraph::GraphDebugInfo::ClassTypeInfoImplemen
    Replacement::draft(0x141c5f050, "content_141c5f050", fn_141c5f050 as *const ()), // content::gfxgraph::GraphDebugInfo::ClassTypeInfoImplementation::content::gfxgraph::GraphDebugInfo::ClassTypeInfoImplemen
    Replacement::draft(0x141c5f5f0, "content_141c5f5f0", tpl_35190::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // content::gfxgraph::NodeDebugInfo::ClassTypeInfoImplementation::content::gfxgraph::NodeDebugInfo::ClassTypeInfoImplementa
    Replacement::draft(0x141c5f650, "content_141c5f650", tpl_35191::<0x8, 0x1401de830, 0x142442de0> as *const ()), // content::gfxgraph::NodeDebugInfo::ClassTypeInfoImplementation::content::gfxgraph::NodeDebugInfo::ClassTypeInfoImplementa
    Replacement::draft(0x141c5fac0, "content_141c5fac0", tpl_35199::<0x14387d8b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::NodeInstanceDebugInfo::ClassTypeInfoImplementation::content::gfxgraph::NodeInstanceDebugInfo::ClassTy
    Replacement::draft(0x141c5fb20, "content_141c5fb20", tpl_35199::<0x14387d8b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::NodeInstanceDebugInfo::ClassTypeInfoImplementation::content::gfxgraph::NodeInstanceDebugInfo::ClassTy
    Replacement::draft(0x141c60310, "content_141c60310", tpl_38936::<0x140853830, 0x10> as *const ()), // content::gfxgraph::GraphExecutionOrder::ClassTypeInfoImplementation::content::gfxgraph::GraphExecutionOrder::ClassTypeIn
    Replacement::draft(0x141c60350, "content_141c60350", tpl_38936::<0x140853830, 0x10> as *const ()), // content::gfxgraph::GraphExecutionOrder::ClassTypeInfoImplementation::content::gfxgraph::GraphExecutionOrder::ClassTypeIn
    Replacement::draft(0x141c60760, "content_141c60760", tpl_38937::<0x1403d79a0, 0x10> as *const ()), // content::gfxgraph::GraphExecutionOrder::ClassTypeInfoImplementation::content::gfxgraph::GraphExecutionOrder::ClassTypeIn
    Replacement::draft(0x141c607b0, "content_141c607b0", tpl_38938::<0x10, 0x1401de830> as *const ()), // content::gfxgraph::GraphExecutionOrder::ClassTypeInfoImplementation::content::gfxgraph::GraphExecutionOrder::ClassTypeIn
    Replacement::draft(0x141c60800, "content_141c60800", tpl_38939::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x10, 0x1401de830, 0x142442c80> as *const ()), // content::gfxgraph::GraphExecutionOrder::ClassTypeInfoImplementation::content::gfxgraph::GraphExecutionOrder::ClassTypeIn
    Replacement::draft(0x141c60870, "content_141c60870", tpl_38940::<0x10, 0x1401de830, 0x142442de0> as *const ()), // content::gfxgraph::GraphExecutionOrder::ClassTypeInfoImplementation::content::gfxgraph::GraphExecutionOrder::ClassTypeIn
    Replacement::draft(0x141c60cf0, "content_141c60cf0", tpl_35199::<0x14387d930, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::GraphResources::ClassTypeInfoImplementation::content::gfxgraph::GraphResources::ClassTypeInfoImplemen
    Replacement::draft(0x141c60d50, "content_141c60d50", tpl_35199::<0x14387d930, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::GraphResources::ClassTypeInfoImplementation::content::gfxgraph::GraphResources::ClassTypeInfoImplemen
    Replacement::draft(0x141c60e20, "content_141c60e20", tpl_38941::<0x50, 0x1401de830, 0x40, 0x30, 0x20, 0x10> as *const ()), // content::gfxgraph::GraphResources::ClassTypeInfoImplementation::content::gfxgraph::GraphResources::ClassTypeInfoImplemen
    Replacement::draft(0x141c60e70, "content_141c60e70", tpl_38942::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // content::gfxgraph::GraphResources::ClassTypeInfoImplementation::content::gfxgraph::GraphResources::ClassTypeInfoImplemen
    Replacement::draft(0x141c60eb0, "content_141c60eb0", tpl_38943::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x50, 0x1401de830, 0x40, 0x30, 0x20, 0x10, 0x142442c80> as *const ()), // content::gfxgraph::GraphResources::ClassTypeInfoImplementation::content::gfxgraph::GraphResources::ClassTypeInfoImplemen
    Replacement::draft(0x141c60f50, "content_141c60f50", tpl_38944::<0x50, 0x1401de830, 0x40, 0x30, 0x20, 0x10, 0x142442de0> as *const ()), // content::gfxgraph::GraphResources::ClassTypeInfoImplementation::content::gfxgraph::GraphResources::ClassTypeInfoImplemen
    Replacement::draft(0x141c612b0, "content_141c612b0", fn_141c612b0 as *const ()), // content::gfxgraph::VariableDefinition::ClassTypeInfoImplementation::content::gfxgraph::VariableDefinition::ClassTypeInfo
    Replacement::draft(0x141c61300, "content_141c61300", fn_141c61300 as *const ()), // content::gfxgraph::VariableDefinition::ClassTypeInfoImplementation::content::gfxgraph::VariableDefinition::ClassTypeInfo
    Replacement::draft(0x141c61460, "content_141c61460", tpl_35199::<0x14387d960, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::VariableDefinition::ClassTypeInfoImplementation::content::gfxgraph::VariableDefinition::ClassTypeInfo
    Replacement::draft(0x141c614c0, "content_141c614c0", tpl_35199::<0x14387d960, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::VariableDefinition::ClassTypeInfoImplementation::content::gfxgraph::VariableDefinition::ClassTypeInfo
    Replacement::draft(0x141c61570, "content_141c61570", tpl_35226::<0x10> as *const ()), // content::gfxgraph::VariableDefinition::ClassTypeInfoImplementation::content::gfxgraph::VariableDefinition::ClassTypeInfo
    Replacement::draft(0x141c615b0, "content_141c615b0", fn_141c615b0 as *const ()), // content::gfxgraph::VariableDefinition::ClassTypeInfoImplementation::content::gfxgraph::VariableDefinition::ClassTypeInfo
    Replacement::draft(0x141c61a50, "content_141c61a50", tpl_35199::<0x14387d8f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::EntityTag::ClassTypeInfoImplementation::content::gfxgraph::EntityTag::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c61ab0, "content_141c61ab0", tpl_35199::<0x14387d8f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::EntityTag::ClassTypeInfoImplementation::content::gfxgraph::EntityTag::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c61ca0, "content_141c61ca0", tpl_38948::<0x20, 0x8, 0xc, 0x14020bcb0> as *const ()), // content::gfxgraph::ContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::ContentShaderRef::ClassTypeInfoImpl
    Replacement::draft(0x141c61d00, "content_141c61d00", tpl_38948::<0x20, 0x8, 0xc, 0x14020bcb0> as *const ()), // content::gfxgraph::ContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::ContentShaderRef::ClassTypeInfoImpl
    Replacement::draft(0x141c62620, "content_141c62620", tpl_35199::<0x14387d970, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::EngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::EngineShaderRef::ClassTypeInfoImplem
    Replacement::draft(0x141c62680, "content_141c62680", tpl_35199::<0x14387d970, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::EngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::EngineShaderRef::ClassTypeInfoImplem
    Replacement::draft(0x141c62b10, "content_141c62b10", tpl_38949::<0x8, 0x14240aa00> as *const ()), // content::gfxgraph::TemporaryContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryContentShaderRef:
    Replacement::draft(0x141c62b40, "content_141c62b40", tpl_38949::<0x8, 0x14240aa00> as *const ()), // content::gfxgraph::TemporaryContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryContentShaderRef:
    Replacement::draft(0x141c62c70, "content_141c62c70", tpl_35199::<0x14387d8d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::TemporaryContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryContentShaderRef:
    Replacement::draft(0x141c62cd0, "content_141c62cd0", tpl_35199::<0x14387d8d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::TemporaryContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryContentShaderRef:
    Replacement::draft(0x141c62da0, "content_141c62da0", fn_141c62da0 as *const ()), // content::gfxgraph::TemporaryContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryContentShaderRef:
    Replacement::draft(0x141c62dd0, "content_141c62dd0", tpl_98838::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x14240ab10, 0x142442c80> as *const ()), // content::gfxgraph::TemporaryContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryContentShaderRef:
    Replacement::draft(0x141c62e30, "content_141c62e30", tpl_35184::<0x8, 0x14240ab10, 0x142442de0> as *const ()), // content::gfxgraph::TemporaryContentShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryContentShaderRef:
    Replacement::draft(0x141c63160, "content_141c63160", tpl_38951::<0x8, 0x14240aa00, 0x48> as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c631b0, "content_141c631b0", tpl_38951::<0x8, 0x14240aa00, 0x48> as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c63300, "content_141c63300", tpl_35199::<0x14387d948, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c63360, "content_141c63360", tpl_35199::<0x14387d948, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c633f0, "content_141c633f0", tpl_38937::<0x14240aa00, 0x48> as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c63440, "content_141c63440", tpl_38952::<0x48, 0x14240ab10> as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c63470, "content_141c63470", fn_141c63470 as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c634b0, "content_141c634b0", tpl_98840::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x48, 0x14240ab10, 0x142442c80> as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c63520, "content_141c63520", tpl_38954::<0x48, 0x14240ab10, 0x142442de0> as *const ()), // content::gfxgraph::TemporaryEngineShaderRef::ClassTypeInfoImplementation::content::gfxgraph::TemporaryEngineShaderRef::C
    Replacement::draft(0x141c63860, "content_141c63860", fn_141c63860 as *const ()), // content::gfxgraph::NodeDefinition::ClassTypeInfoImplementation::content::gfxgraph::NodeDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x141c638a0, "content_141c638a0", fn_141c638a0 as *const ()), // content::gfxgraph::NodeDefinition::ClassTypeInfoImplementation::content::gfxgraph::NodeDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x141c639e0, "content_141c639e0", tpl_35199::<0x14387d950, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::NodeDefinition::ClassTypeInfoImplementation::content::gfxgraph::NodeDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x141c63a40, "content_141c63a40", tpl_35199::<0x14387d950, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::NodeDefinition::ClassTypeInfoImplementation::content::gfxgraph::NodeDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x141c63c10, "content_141c63c10", tpl_38957::<0x8> as *const ()), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft(0x141c63c30, "content_141c63c30", tpl_38957::<0x8> as *const ()), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft(0x141c63c70, "content_141c63c70", tpl_38958::<0x8, 0x18> as *const ()), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft(0x141c63c90, "content_141c63c90", tpl_38958::<0x8, 0x18> as *const ()), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c63eb0, "content_141c63eb0", tpl_38959::<0x8, 0x1408b1160> as *const (), Throws::Discard), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c63ed0, "content_141c63ed0", tpl_38959::<0x8, 0x1408b1160> as *const (), Throws::Discard), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft(0x141c640b0, "content_141c640b0", fn_141c640b0 as *const ()), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft(0x141c64120, "content_141c64120", fn_141c64120 as *const ()), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft(0x141c641b0, "content_141c641b0", fn_141c641b0 as *const ()), // content::gfxgraph::DefaultValue::ClassTypeInfoImplementation::content::gfxgraph::DefaultValue::ClassTypeInfoImplementati
    Replacement::draft(0x141c64660, "content_141c64660", tpl_35199::<0x14387d8a0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::TemplatePinInfo::ClassTypeInfoImplementation::content::gfxgraph::TemplatePinInfo::ClassTypeInfoImplem
    Replacement::draft(0x141c646c0, "content_141c646c0", tpl_35199::<0x14387d8a0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::TemplatePinInfo::ClassTypeInfoImplementation::content::gfxgraph::TemplatePinInfo::ClassTypeInfoImplem
    Replacement::draft(0x141c64ad0, "content_141c64ad0", tpl_38963::<0x8, 0xc> as *const ()), // content::gfxgraph::GatherToken::ClassTypeInfoImplementation::content::gfxgraph::GatherToken::ClassTypeInfoImplementation
    Replacement::draft(0x141c64b10, "content_141c64b10", tpl_38964::<0x8, 0xc> as *const ()), // content::gfxgraph::GatherToken::ClassTypeInfoImplementation::content::gfxgraph::GatherToken::ClassTypeInfoImplementation
    Replacement::draft(0x141c64c50, "content_141c64c50", tpl_35199::<0x14387d898, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::GatherToken::ClassTypeInfoImplementation::content::gfxgraph::GatherToken::ClassTypeInfoImplementation
    Replacement::draft(0x141c64cb0, "content_141c64cb0", tpl_35199::<0x14387d898, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::GatherToken::ClassTypeInfoImplementation::content::gfxgraph::GatherToken::ClassTypeInfoImplementation
    Replacement::draft(0x141c652f0, "content_141c652f0", tpl_27414::<0x8, 0x10> as *const ()), // content::gfxgraph::UpdateFlow::ClassTypeInfoImplementation::content::gfxgraph::UpdateFlow::ClassTypeInfoImplementation::
    Replacement::draft(0x141c65310, "content_141c65310", tpl_35190::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // content::gfxgraph::UpdateFlow::ClassTypeInfoImplementation::content::gfxgraph::UpdateFlow::ClassTypeInfoImplementation::
    Replacement::draft(0x141c65370, "content_141c65370", tpl_35191::<0x8, 0x1401de830, 0x142442de0> as *const ()), // content::gfxgraph::UpdateFlow::ClassTypeInfoImplementation::content::gfxgraph::UpdateFlow::ClassTypeInfoImplementation::
    Replacement::draft(0x141c656a0, "content_141c656a0", tpl_38965::<0x8, 0x2> as *const ()), // content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::
    Replacement::draft(0x141c656d0, "content_141c656d0", tpl_38965::<0x8, 0x2> as *const ()), // content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::
    Replacement::draft(0x141c65800, "content_141c65800", tpl_35199::<0x14387d8a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::
    Replacement::draft(0x141c65860, "content_141c65860", tpl_35199::<0x14387d8a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::
    Replacement::draft(0x141c65940, "content_141c65940", fn_141c65940 as *const ()), // content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::content::gfxgraph::NodeHandle::ClassTypeInfoImplementation::
    Replacement::draft(0x141c65df0, "content_141c65df0", tpl_35199::<0x14387d968, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::NamedValue::ClassTypeInfoImplementation::content::gfxgraph::NamedValue::ClassTypeInfoImplementation::
    Replacement::draft(0x141c65e50, "content_141c65e50", tpl_35199::<0x14387d968, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::gfxgraph::NamedValue::ClassTypeInfoImplementation::content::gfxgraph::NamedValue::ClassTypeInfoImplementation::
    Replacement::draft(0x141c66110, "content_141c66110", fn_141c66110 as *const ()), // sub_141c66110
    Replacement::draft(0x141c662f0, "content_141c662f0", fn_141c662f0 as *const ()), // sub_141c662f0
    Replacement::draft(0x141c663b0, "content_141c663b0", fn_141c663b0 as *const ()), // sub_141c663b0
    Replacement::draft_throws(0x141c66cc0, "content_141c66cc0", fn_141c66cc0 as *const (), Throws::Discard), // content::GfxGraph::ClassTypeInfoImplementation::content::GfxGraph::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c66f70, "content_141c66f70", fn_141c66f70 as *const ()), // content::GfxGraphConnection::content::GfxGraphConnection::vfunc_12
    Replacement::draft(0x141c67280, "content_141c67280", fn_141c67280 as *const ()), // content::GfxGraphConnection::ClassTypeInfoImplementation::content::GfxGraphConnection::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c67460, "content_141c67460", fn_141c67460 as *const ()), // content::GfxGraphConnection::ClassTypeInfoImplementation::content::GfxGraphConnection::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c67620, "content_141c67620", fn_141c67620 as *const ()), // content::GfxGraphNode::content::GfxGraphNode::vfunc_1
    Replacement::draft(0x141c67770, "content_141c67770", fn_141c67770 as *const ()), // content::GfxGraphNode::ClassTypeInfoImplementation::content::GfxGraphNode::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft_throws(0x141c67c20, "content_141c67c20", fn_141c67c20 as *const (), Throws::Discard), // content::GfxGraphNode::ClassTypeInfoImplementation::content::GfxGraphNode::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c67cf0, "content_141c67cf0", fn_141c67cf0 as *const ()), // content::GfxGraphNode::ClassTypeInfoImplementation::content::GfxGraphNode::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c67e10, "content_141c67e10", tpl_365::<0x1408e06d0> as *const ()), // content::GfxGraphNode::ClassTypeInfoImplementation::content::GfxGraphNode::ClassTypeInfoImplementation::vfunc_8
    Replacement::draft(0x141c67ea0, "content_141c67ea0", tpl_38958::<0x238, 0x8> as *const ()), // content::GfxGraphPin::ClassTypeInfoImplementation::content::GfxGraphPin::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141c67ed0, "content_141c67ed0", tpl_38958::<0x238, 0x8> as *const ()), // content::GfxGraphPin::ClassTypeInfoImplementation::content::GfxGraphPin::ClassTypeInfoImplementation::vfunc_36
    Replacement::draft(0x141c67f20, "content_141c67f20", tpl_38958::<0x238, 0x18> as *const ()), // content::GfxGraphPin::ClassTypeInfoImplementation::content::GfxGraphPin::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c67f50, "content_141c67f50", tpl_38958::<0x238, 0x18> as *const ()), // content::GfxGraphPin::ClassTypeInfoImplementation::content::GfxGraphPin::ClassTypeInfoImplementation::vfunc_38
    Replacement::draft(0x141c68380, "content_141c68380", fn_141c68380 as *const ()), // content::GfxGraphPin::ClassTypeInfoImplementation::content::GfxGraphPin::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c68420, "content_141c68420", fn_141c68420 as *const ()), // content::GfxGraphPin::ClassTypeInfoImplementation::content::GfxGraphPin::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141c68600, "content_141c68600", tpl_38979::<0x143873d80, 0x143275de8, 0x2, 0x1401de410> as *const ()), // content::GfxGraphVariable::ClassTypeInfoImplementation::content::GfxGraphVariable::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141c68650, "content_141c68650", tpl_38979::<0x143873d80, 0x143275de8, 0x2, 0x1401de410> as *const ()), // content::GfxGraphVariable::ClassTypeInfoImplementation::content::GfxGraphVariable::ClassTypeInfoImplementation::vfunc_37
    Replacement::draft(0x141c68ae0, "content_141c68ae0", fn_141c68ae0 as *const ()), // content::GfxGraphVariable::ClassTypeInfoImplementation::content::GfxGraphVariable::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c68b40, "content_141c68b40", fn_141c68b40 as *const ()), // content::GfxGraphVariable::ClassTypeInfoImplementation::content::GfxGraphVariable::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141c693c0, "content_141c693c0", fn_141c693c0 as *const ()), // sub_141c693c0
    Replacement::draft(0x141c696c0, "content_141c696c0", fn_141c696c0 as *const ()), // sub_141c696c0
    Replacement::draft(0x141c69980, "content_141c69980", fn_141c69980 as *const ()), // sub_141c69980
    Replacement::draft(0x141c699a0, "content_141c699a0", fn_141c699a0 as *const ()), // sub_141c699a0
    Replacement::draft(0x141c69be0, "content_141c69be0", fn_141c69be0 as *const ()), // sub_141c69be0
    Replacement::draft(0x141c69cb0, "content_141c69cb0", fn_141c69cb0 as *const ()), // sub_141c69cb0
    Replacement::draft(0x141c69cd0, "content_141c69cd0", fn_141c69cd0 as *const ()), // sub_141c69cd0
    Replacement::draft(0x141c69ec0, "content_141c69ec0", fn_141c69ec0 as *const ()), // sub_141c69ec0
    Replacement::draft(0x141c69ef0, "content_141c69ef0", tpl_38989::<0x18, 0x38, 0x40, 0x48, 0x1434268d0, 0x20, 0x50> as *const ()), // sub_141c69ef0
    Replacement::draft(0x141c69f70, "content_141c69f70", tpl_38989::<0x14, 0x34, 0x3c, 0x44, 0x1434268d0, 0x1c, 0x4c> as *const ()), // sub_141c69f70
    Replacement::draft(0x141c69ff0, "content_141c69ff0", fn_141c69ff0 as *const ()), // sub_141c69ff0
    Replacement::draft(0x141c6a000, "content_141c6a000", fn_141c6a000 as *const ()), // sub_141c6a000
    Replacement::draft(0x141c6a090, "content_141c6a090", fn_141c6a090 as *const ()), // content::HierarchyLOD::content::HierarchyLOD::vfunc_1
    Replacement::draft(0x141c6a1d0, "content_141c6a1d0", fn_141c6a1d0 as *const ()), // content::HierarchyLOD::ClassTypeInfoImplementation::content::HierarchyLOD::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141c6a680, "content_141c6a680", fn_141c6a680 as *const ()), // content::HierarchyLOD::ClassTypeInfoImplementation::content::HierarchyLOD::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c6a720, "content_141c6a720", tpl_38996::<0x14093bce0, 0x14327da78, 0xc8, 0xd0> as *const ()), // content::HierarchyLOD::ClassTypeInfoImplementation::content::HierarchyLOD::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft_throws(0x141c6a8b0, "content_141c6a8b0", tpl_38997::<0x143269e18, 0x40, 0x1407f52b0> as *const (), Throws::Discard), // content::GenericEntity::content::GenericEntity::vfunc_2
    Replacement::draft_throws(0x141c6aa20, "content_141c6aa20", tpl_38998::<0x143269e18, 0x1407f52b0> as *const (), Throws::Discard), // content::GenericEntity::ClassTypeInfoImplementation::content::GenericEntity::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft_throws(0x141c6b020, "content_141c6b020", tpl_38997::<0x143269e18, 0x40, 0x1407f52b0> as *const (), Throws::Discard), // content::HierarchyGroup::content::HierarchyGroup::vfunc_2
    Replacement::draft(0x141c6b0f0, "content_141c6b0f0", fn_141c6b0f0 as *const ()), // content::HierarchyGroup::content::HierarchyGroup::vfunc_12
    Replacement::draft_throws(0x141c6b1e0, "content_141c6b1e0", tpl_38998::<0x143269e18, 0x1407f52b0> as *const (), Throws::Discard), // content::HierarchyGroup::ClassTypeInfoImplementation::content::HierarchyGroup::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141c6b470, "content_141c6b470", fn_141c6b470 as *const ()), // content::HierarchyGroup::ClassTypeInfoImplementation::content::HierarchyGroup::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c6b680, "content_141c6b680", tpl_38846::<0x140941970, 0xc0> as *const ()), // content::HierarchyGroup::ClassTypeInfoImplementation::content::HierarchyGroup::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c6b880, "content_141c6b880", fn_141c6b880 as *const ()), // content::GIVolumeEntity::content::GIVolumeEntity::vfunc_3
    Replacement::draft(0x141c6ba40, "content_141c6ba40", fn_141c6ba40 as *const ()), // content::GIVolumeEntity::ClassTypeInfoImplementation::content::GIVolumeEntity::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141c6be90, "content_141c6be90", fn_141c6be90 as *const ()), // content::GIVolumeEntity::ClassTypeInfoImplementation::content::GIVolumeEntity::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c6bf40, "content_141c6bf40", tpl_38996::<0x14093bce0, 0x143278778, 0xc0, 0xc8> as *const ()), // content::GIVolumeEntity::ClassTypeInfoImplementation::content::GIVolumeEntity::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c6c320, "content_141c6c320", tpl_38951::<0x8, 0x14240aa00, 0x30> as *const ()), // content::EntityReference::ClassTypeInfoImplementation::content::EntityReference::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c6c370, "content_141c6c370", tpl_38951::<0x8, 0x14240aa00, 0x30> as *const ()), // content::EntityReference::ClassTypeInfoImplementation::content::EntityReference::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141c6c4c0, "content_141c6c4c0", tpl_35199::<0x14387dab8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::EntityReference::ClassTypeInfoImplementation::content::EntityReference::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141c6c520, "content_141c6c520", tpl_35199::<0x14387dab8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::EntityReference::ClassTypeInfoImplementation::content::EntityReference::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141c6c5d0, "content_141c6c5d0", tpl_38937::<0x14240aa00, 0x30> as *const ()), // content::EntityReference::ClassTypeInfoImplementation::content::EntityReference::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c6c620, "content_141c6c620", tpl_38952::<0x30, 0x14240ab10> as *const ()), // content::EntityReference::ClassTypeInfoImplementation::content::EntityReference::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141c6c690, "content_141c6c690", tpl_98840::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x30, 0x14240ab10, 0x142442c80> as *const ()), // content::EntityReference::ClassTypeInfoImplementation::content::EntityReference::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141c6c700, "content_141c6c700", tpl_38954::<0x30, 0x14240ab10, 0x142442de0> as *const ()), // content::EntityReference::ClassTypeInfoImplementation::content::EntityReference::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141c6ca40, "content_141c6ca40", tpl_39004::<0x8, 0x14240aa00, 0x48> as *const ()), // content::BlueprintChild::ClassTypeInfoImplementation::content::BlueprintChild::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c6ca90, "content_141c6ca90", tpl_39004::<0x8, 0x14240aa00, 0x48> as *const ()), // content::BlueprintChild::ClassTypeInfoImplementation::content::BlueprintChild::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141c6cbe0, "content_141c6cbe0", tpl_35199::<0x14387dbf8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BlueprintChild::ClassTypeInfoImplementation::content::BlueprintChild::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141c6cc40, "content_141c6cc40", tpl_35199::<0x14387dbf8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BlueprintChild::ClassTypeInfoImplementation::content::BlueprintChild::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141c6ccf0, "content_141c6ccf0", tpl_39005::<0x14240aa00, 0x48> as *const ()), // content::BlueprintChild::ClassTypeInfoImplementation::content::BlueprintChild::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c825f0, "content_141c825f0", tpl_39006::<0x8, 0x4, 0xc, 0x10, 0x14, 0x18> as *const ()), // content::CharacterLookAtComponent::Category::ClassTypeInfoImplementation::content::CharacterLookAtComponent::Category::C
    Replacement::draft(0x141c82660, "content_141c82660", tpl_39006::<0x8, 0x4, 0xc, 0x10, 0x14, 0x18> as *const ()), // content::CharacterLookAtComponent::Category::ClassTypeInfoImplementation::content::CharacterLookAtComponent::Category::C
    Replacement::draft(0x141c82860, "content_141c82860", tpl_39007::<0x10, 0x18> as *const ()), // content::CharacterLookAtComponent::Category::ClassTypeInfoImplementation::content::CharacterLookAtComponent::Category::C
    Replacement::draft(0x141c84f00, "contentcore_141c84f00", tpl_39008::<0x8, 0xc> as *const ()), // contentcore::SplineControlPoint::ClassTypeInfoImplementation::contentcore::SplineControlPoint::ClassTypeInfoImplementati
    Replacement::draft(0x141c84f30, "contentcore_141c84f30", tpl_39008::<0x8, 0xc> as *const ()), // contentcore::SplineControlPoint::ClassTypeInfoImplementation::contentcore::SplineControlPoint::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141c85290, "contentcore_141c85290", tpl_39009::<0x1432a60d8, 0x8, 0x1432a6cc0, 0x1432442c0, 0x1402c4e80> as *const (), Throws::Discard), // contentcore::SDFVolumeComponentEditor::contentcore::SDFVolumeComponentEditor::vfunc_23
    Replacement::draft(0x141c85360, "contentcore_141c85360", tpl_38929::<0x8, 0x10> as *const ()), // contentcore::SDFVolumeComponentEditor::contentcore::SDFVolumeComponentEditor::vfunc_33
    Replacement::draft_throws(0x141c85410, "contentcore_141c85410", tpl_39010::<0x1432a6cc0, 0x1432a60d8, 0x1432442c0, 0x1402c4e80> as *const (), Throws::Discard), // contentcore::SDFVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::SDFVolumeComponentEditor::ClassTypeInfo
    Replacement::draft(0x141c856a0, "contentcore_141c856a0", tpl_38930::<0x8, 0x10> as *const ()), // contentcore::SDFVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::SDFVolumeComponentEditor::ClassTypeInfo
    Replacement::draft(0x141c85870, "contentcore_141c85870", tpl_38931::<0x8, 0x10> as *const ()), // contentcore::SDFVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::SDFVolumeComponentEditor::ClassTypeInfo
    Replacement::draft(0x141c858c0, "contentcore_141c858c0", tpl_39011::<0x143294070, 0x8, 0xc, 0x10> as *const ()), // contentcore::SDFVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::SDFVolumeComponentEditor::ClassTypeInfo
    Replacement::draft(0x141c85a30, "contentcore_141c85a30", tpl_38782::<0x1432a6b08, 0x8, 0x1409ff500> as *const ()), // contentcore::SDFVolumeComponent::contentcore::SDFVolumeComponent::vfunc_22
    Replacement::draft(0x141c85b80, "contentcore_141c85b80", fn_141c85b80 as *const ()), // contentcore::SDFVolumeComponent::ClassTypeInfoImplementation::contentcore::SDFVolumeComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141c86060, "contentcore_141c86060", tpl_39013::<0x14329acb8, 0x8> as *const ()), // contentcore::SDFVolumeComponent::ClassTypeInfoImplementation::contentcore::SDFVolumeComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141c861c0, "contentcore_141c861c0", tpl_39014::<0x1432a6d80, 0x14, 0x14384e6e8, 0x1432a6a78, 0x143276a48, 0x1401de410> as *const ()), // contentcore::GIResolutionVolumeComponentEditor::contentcore::GIResolutionVolumeComponentEditor::vfunc_22
    Replacement::draft(0x141c86230, "contentcore_141c86230", tpl_39014::<0x1432a6d80, 0x14, 0x14384e6e8, 0x1432a6a78, 0x143276a48, 0x1401de410> as *const ()), // contentcore::GIResolutionVolumeComponentEditor::contentcore::GIResolutionVolumeComponentEditor::vfunc_24
    Replacement::draft(0x141c86310, "contentcore_141c86310", fn_141c86310 as *const ()), // contentcore::GIResolutionVolumeComponentEditor::contentcore::GIResolutionVolumeComponentEditor::vfunc_33
    Replacement::draft(0x141c863b0, "contentcore_141c863b0", tpl_39016::<0x14384e6e8, 0x1432a6d80, 0x1432a6a78, 0x143276a48, 0x1401de410> as *const ()), // contentcore::GIResolutionVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIResolutionVolumeComponentEdi
    Replacement::draft(0x141c86470, "contentcore_141c86470", tpl_39016::<0x14384e6e8, 0x1432a6d80, 0x1432a6a78, 0x143276a48, 0x1401de410> as *const ()), // contentcore::GIResolutionVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIResolutionVolumeComponentEdi
    Replacement::draft(0x141c866f0, "contentcore_141c866f0", fn_141c866f0 as *const ()), // contentcore::GIResolutionVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIResolutionVolumeComponentEdi
    Replacement::draft(0x141c868e0, "contentcore_141c868e0", fn_141c868e0 as *const ()), // contentcore::GIResolutionVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIResolutionVolumeComponentEdi
    Replacement::draft(0x141c86940, "contentcore_141c86940", fn_141c86940 as *const ()), // contentcore::GIResolutionVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIResolutionVolumeComponentEdi
    Replacement::draft(0x141c87400, "contentcore_141c87400", tpl_39014::<0x1432a6d80, 0x14, 0x14384e6e8, 0x1432a6ce0, 0x143276a48, 0x1401de410> as *const ()), // contentcore::GIVolumeComponentEditor::contentcore::GIVolumeComponentEditor::vfunc_22
    Replacement::draft(0x141c87470, "contentcore_141c87470", tpl_39014::<0x1432a6d80, 0x14, 0x14384e6e8, 0x1432a6ce0, 0x143276a48, 0x1401de410> as *const ()), // contentcore::GIVolumeComponentEditor::contentcore::GIVolumeComponentEditor::vfunc_24
    Replacement::draft_throws(0x141c87520, "contentcore_141c87520", tpl_838::<0x1409f0e70> as *const (), Throws::Discard), // contentcore::GIVolumeComponentEditor::contentcore::GIVolumeComponentEditor::vfunc_30
    Replacement::draft(0x141c87550, "contentcore_141c87550", fn_141c87550 as *const ()), // contentcore::GIVolumeComponentEditor::contentcore::GIVolumeComponentEditor::vfunc_33
    Replacement::draft(0x141c87630, "contentcore_141c87630", tpl_39016::<0x14384e6e8, 0x1432a6d80, 0x1432a6ce0, 0x143276a48, 0x1401de410> as *const ()), // contentcore::GIVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIVolumeComponentEditor::ClassTypeInfoIm
    Replacement::draft(0x141c876f0, "contentcore_141c876f0", tpl_39016::<0x14384e6e8, 0x1432a6d80, 0x1432a6ce0, 0x143276a48, 0x1401de410> as *const ()), // contentcore::GIVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIVolumeComponentEditor::ClassTypeInfoIm
    Replacement::draft(0x141c87970, "contentcore_141c87970", fn_141c87970 as *const ()), // contentcore::GIVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIVolumeComponentEditor::ClassTypeInfoIm
    Replacement::draft(0x141c87ba0, "contentcore_141c87ba0", fn_141c87ba0 as *const ()), // contentcore::GIVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIVolumeComponentEditor::ClassTypeInfoIm
    Replacement::draft(0x141c87c20, "contentcore_141c87c20", fn_141c87c20 as *const ()), // contentcore::GIVolumeComponentEditor::ClassTypeInfoImplementation::contentcore::GIVolumeComponentEditor::ClassTypeInfoIm
    Replacement::draft(0x141c87e80, "contentcore_141c87e80", fn_141c87e80 as *const ()), // contentcore::GIVolumeComponent::contentcore::GIVolumeComponent::vfunc_33
    Replacement::draft(0x141c88200, "contentcore_141c88200", fn_141c88200 as *const ()), // contentcore::GIVolumeComponent::ClassTypeInfoImplementation::contentcore::GIVolumeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c88410, "contentcore_141c88410", fn_141c88410 as *const ()), // contentcore::GIVolumeComponent::ClassTypeInfoImplementation::contentcore::GIVolumeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c88480, "contentcore_141c88480", fn_141c88480 as *const ()), // contentcore::GIVolumeComponent::ClassTypeInfoImplementation::contentcore::GIVolumeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c88ba0, "contentcore_141c88ba0", fn_141c88ba0 as *const ()), // contentcore::NWEDMeshComponent::ClassTypeInfoImplementation::contentcore::NWEDMeshComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c88cd0, "contentcore_141c88cd0", fn_141c88cd0 as *const ()), // contentcore::NWEDMeshComponent::ClassTypeInfoImplementation::contentcore::NWEDMeshComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c88f30, "contentcore_141c88f30", tpl_39030::<0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28> as *const ()), // contentcore::ThumbnailRendererConfigComponent::contentcore::ThumbnailRendererConfigComponent::vfunc_33
    Replacement::draft(0x141c892d0, "contentcore_141c892d0", tpl_39031::<0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28> as *const ()), // contentcore::ThumbnailRendererConfigComponent::ClassTypeInfoImplementation::contentcore::ThumbnailRendererConfigComponen
    Replacement::draft(0x141c89530, "contentcore_141c89530", tpl_39032::<0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28> as *const ()), // contentcore::ThumbnailRendererConfigComponent::ClassTypeInfoImplementation::contentcore::ThumbnailRendererConfigComponen
    Replacement::draft(0x141c895b0, "contentcore_141c895b0", fn_141c895b0 as *const ()), // contentcore::ThumbnailRendererConfigComponent::ClassTypeInfoImplementation::contentcore::ThumbnailRendererConfigComponen
    Replacement::draft(0x141c897e0, "contentcore_141c897e0", fn_141c897e0 as *const ()), // contentcore::AcousticsListenerComponent::contentcore::AcousticsListenerComponent::vfunc_33
    Replacement::draft(0x141c89b30, "contentcore_141c89b30", fn_141c89b30 as *const ()), // contentcore::AcousticsListenerComponent::ClassTypeInfoImplementation::contentcore::AcousticsListenerComponent::ClassType
    Replacement::draft(0x141c89d40, "contentcore_141c89d40", fn_141c89d40 as *const ()), // contentcore::AcousticsListenerComponent::ClassTypeInfoImplementation::contentcore::AcousticsListenerComponent::ClassType
    Replacement::draft(0x141c89da0, "contentcore_141c89da0", fn_141c89da0 as *const ()), // contentcore::AcousticsListenerComponent::ClassTypeInfoImplementation::contentcore::AcousticsListenerComponent::ClassType
    Replacement::draft(0x141c89f20, "contentcore_141c89f20", tpl_39038::<0x1432a5c98, 0x8, 0x14384e7c0, 0x1432a5cc8, 0x1401de410> as *const ()), // contentcore::UIPageComponent::contentcore::UIPageComponent::vfunc_23
    Replacement::draft(0x141c8a0a0, "contentcore_141c8a0a0", tpl_39039::<0x14384e7c0, 0x1432a5c98, 0x1432a5cc8, 0x1401de410> as *const ()), // contentcore::UIPageComponent::ClassTypeInfoImplementation::contentcore::UIPageComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c8a520, "contentcore_141c8a520", fn_141c8a520 as *const ()), // contentcore::UIPageComponent::ClassTypeInfoImplementation::contentcore::UIPageComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c8a5b0, "contentcore_141c8a5b0", fn_141c8a5b0 as *const ()), // contentcore::UIPageComponent::ClassTypeInfoImplementation::contentcore::UIPageComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141c8a750, "contentcore_141c8a750", tpl_39042::<0x1432a5e18, 0x8, 0x1432a5e90, 0x1432a5f20, 0x14020f270> as *const ()), // contentcore::StreamingUnitComponent::contentcore::StreamingUnitComponent::vfunc_23
    Replacement::draft_throws(0x141c8a7f0, "contentcore_141c8a7f0", tpl_838::<0x1409e4d90> as *const (), Throws::Discard), // contentcore::StreamingUnitComponent::contentcore::StreamingUnitComponent::vfunc_30
    Replacement::draft(0x141c8a8c0, "contentcore_141c8a8c0", tpl_39043::<0x1432a5e90, 0x1432a5e18, 0x1432a5f20, 0x14020f270> as *const ()), // contentcore::StreamingUnitComponent::ClassTypeInfoImplementation::contentcore::StreamingUnitComponent::ClassTypeInfoImpl
    Replacement::draft(0x141c8ad20, "contentcore_141c8ad20", fn_141c8ad20 as *const ()), // contentcore::StreamingUnitComponent::ClassTypeInfoImplementation::contentcore::StreamingUnitComponent::ClassTypeInfoImpl
    Replacement::draft(0x141c8ad90, "contentcore_141c8ad90", fn_141c8ad90 as *const ()), // contentcore::StreamingUnitComponent::ClassTypeInfoImplementation::contentcore::StreamingUnitComponent::ClassTypeInfoImpl
    Replacement::draft(0x141c8b8a0, "contentcore_141c8b8a0", tpl_38498::<0x8, 0x10, 0x18, 0x20, 0x24, 0x2c> as *const ()), // contentcore::OBBVolumeBox::ClassTypeInfoImplementation::contentcore::OBBVolumeBox::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c8b900, "contentcore_141c8b900", tpl_38499::<0x10, 0x8, 0x18, 0x24> as *const ()), // contentcore::OBBVolumeBox::ClassTypeInfoImplementation::contentcore::OBBVolumeBox::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141c8baf0, "contentcore_141c8baf0", tpl_38500::<0x20> as *const ()), // contentcore::OBBVolumeBox::ClassTypeInfoImplementation::contentcore::OBBVolumeBox::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c8bb30, "contentcore_141c8bb30", fn_141c8bb30 as *const ()), // contentcore::OBBVolumeBox::ClassTypeInfoImplementation::contentcore::OBBVolumeBox::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c8cd70, "contentcore_141c8cd70", tpl_39047::<0x8, 0x14240aa00, 0x20> as *const ()), // contentcore::IconComponentDebug::contentcore::IconComponentDebug::vfunc_33
    Replacement::draft(0x141c8d0a0, "contentcore_141c8d0a0", tpl_39048::<0x8, 0x14240aa00, 0x20> as *const ()), // contentcore::IconComponentDebug::ClassTypeInfoImplementation::contentcore::IconComponentDebug::ClassTypeInfoImplementati
    Replacement::draft(0x141c8d2a0, "contentcore_141c8d2a0", tpl_38534::<0x8, 0x14240aa00, 0x20> as *const ()), // contentcore::IconComponentDebug::ClassTypeInfoImplementation::contentcore::IconComponentDebug::ClassTypeInfoImplementati
    Replacement::draft(0x141c8ddb0, "contentcore_141c8ddb0", tpl_39049::<0x8, 0x143277b20, 0x1432a6080, 0x14020f270> as *const ()), // contentcore::HierarchyLODComponent::contentcore::HierarchyLODComponent::vfunc_23
    Replacement::draft(0x141c8df20, "contentcore_141c8df20", tpl_39050::<0x143277b20, 0x1432a6080, 0x14020f270> as *const ()), // contentcore::HierarchyLODComponent::ClassTypeInfoImplementation::contentcore::HierarchyLODComponent::ClassTypeInfoImplem
    Replacement::draft(0x141c8e380, "contentcore_141c8e380", fn_141c8e380 as *const ()), // contentcore::HierarchyLODComponent::ClassTypeInfoImplementation::contentcore::HierarchyLODComponent::ClassTypeInfoImplem
    Replacement::draft(0x141c8e440, "contentcore_141c8e440", fn_141c8e440 as *const ()), // contentcore::HierarchyLODComponent::ClassTypeInfoImplementation::contentcore::HierarchyLODComponent::ClassTypeInfoImplem
    Replacement::draft(0x141c8ebb0, "contentcore_141c8ebb0", fn_141c8ebb0 as *const ()), // contentcore::JumpPointComponentDebug::ClassTypeInfoImplementation::contentcore::JumpPointComponentDebug::ClassTypeInfoIm
    Replacement::draft_throws(0x141c8edb0, "contentcore_141c8edb0", tpl_838::<0x1409ddd30> as *const (), Throws::Discard), // contentcore::JumpPointComponent::contentcore::JumpPointComponent::vfunc_31
    Replacement::draft(0x141c8edd0, "contentcore_141c8edd0", fn_141c8edd0 as *const ()), // contentcore::JumpPointComponent::contentcore::JumpPointComponent::vfunc_33
    Replacement::draft(0x141c8f0e0, "contentcore_141c8f0e0", fn_141c8f0e0 as *const ()), // contentcore::JumpPointComponent::ClassTypeInfoImplementation::contentcore::JumpPointComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141c8f2c0, "contentcore_141c8f2c0", fn_141c8f2c0 as *const ()), // contentcore::JumpPointComponent::ClassTypeInfoImplementation::contentcore::JumpPointComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141c8f310, "contentcore_141c8f310", fn_141c8f310 as *const ()), // contentcore::JumpPointComponent::ClassTypeInfoImplementation::contentcore::JumpPointComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141c8fa20, "contentcore_141c8fa20", fn_141c8fa20 as *const ()), // contentcore::TimelineComponent::ClassTypeInfoImplementation::contentcore::TimelineComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c8fac0, "contentcore_141c8fac0", fn_141c8fac0 as *const ()), // contentcore::TimelineComponent::ClassTypeInfoImplementation::contentcore::TimelineComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c901b0, "contentcore_141c901b0", tpl_39060::<0x8, 0x140a359f0> as *const ()), // contentcore::TagComponent::ClassTypeInfoImplementation::contentcore::TagComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c90370, "contentcore_141c90370", tpl_39038::<0x1432a6200, 0x8, 0x14384e7a8, 0x1432a61f0, 0x1401de410> as *const ()), // contentcore::LocalTimeScaleComponent::contentcore::LocalTimeScaleComponent::vfunc_22
    Replacement::draft(0x141c903c0, "contentcore_141c903c0", tpl_39038::<0x1432a6200, 0x8, 0x14384e7a8, 0x1432a61f0, 0x1401de410> as *const ()), // contentcore::LocalTimeScaleComponent::contentcore::LocalTimeScaleComponent::vfunc_23
    Replacement::draft(0x141c90410, "contentcore_141c90410", tpl_39038::<0x1432a6200, 0x8, 0x14384e7a8, 0x1432a61f0, 0x1401de410> as *const ()), // contentcore::LocalTimeScaleComponent::contentcore::LocalTimeScaleComponent::vfunc_24
    Replacement::draft(0x141c90550, "contentcore_141c90550", tpl_39039::<0x14384e7a8, 0x1432a6200, 0x1432a61f0, 0x1401de410> as *const ()), // contentcore::LocalTimeScaleComponent::ClassTypeInfoImplementation::contentcore::LocalTimeScaleComponent::ClassTypeInfoIm
    Replacement::draft(0x141c905c0, "contentcore_141c905c0", tpl_39039::<0x14384e7a8, 0x1432a6200, 0x1432a61f0, 0x1401de410> as *const ()), // contentcore::LocalTimeScaleComponent::ClassTypeInfoImplementation::contentcore::LocalTimeScaleComponent::ClassTypeInfoIm
    Replacement::draft(0x141c90630, "contentcore_141c90630", tpl_39039::<0x14384e7a8, 0x1432a6200, 0x1432a61f0, 0x1401de410> as *const ()), // contentcore::LocalTimeScaleComponent::ClassTypeInfoImplementation::contentcore::LocalTimeScaleComponent::ClassTypeInfoIm
    Replacement::draft_throws(0x141c90c20, "contentcore_141c90c20", tpl_39061::<0x20, 0x1432a61a0, 0x14310b378, 0x1402c4e80> as *const (), Throws::Discard), // contentcore::PointOfInterestBoneComponentEditor::contentcore::PointOfInterestBoneComponentEditor::vfunc_23
    Replacement::draft(0x141c90cf0, "contentcore_141c90cf0", fn_141c90cf0 as *const ()), // contentcore::PointOfInterestBoneComponentEditor::contentcore::PointOfInterestBoneComponentEditor::vfunc_33
    Replacement::draft_throws(0x141c90dc0, "contentcore_141c90dc0", fn_141c90dc0 as *const (), Throws::Discard), // contentcore::PointOfInterestBoneComponentEditor::ClassTypeInfoImplementation::contentcore::PointOfInterestBoneComponentE
    Replacement::draft(0x141c91050, "contentcore_141c91050", fn_141c91050 as *const ()), // contentcore::PointOfInterestBoneComponentEditor::ClassTypeInfoImplementation::contentcore::PointOfInterestBoneComponentE
    Replacement::draft(0x141c91250, "contentcore_141c91250", tpl_38895::<0x8, 0x14240aa00, 0x20, 0x28> as *const ()), // contentcore::PointOfInterestBoneComponentEditor::ClassTypeInfoImplementation::contentcore::PointOfInterestBoneComponentE
    Replacement::draft(0x141c912c0, "contentcore_141c912c0", fn_141c912c0 as *const ()), // contentcore::PointOfInterestBoneComponentEditor::ClassTypeInfoImplementation::contentcore::PointOfInterestBoneComponentE
    Replacement::draft(0x141c91d80, "contentcore_141c91d80", tpl_39049::<0x8, 0x1432a6290, 0x1432a6298, 0x140a00c00> as *const ()), // contentcore::NWEDDecalComponent::contentcore::NWEDDecalComponent::vfunc_22
    Replacement::draft_throws(0x141c91dc0, "contentcore_141c91dc0", tpl_838::<0x1409d96d0> as *const (), Throws::Discard), // contentcore::NWEDDecalComponent::contentcore::NWEDDecalComponent::vfunc_23
    Replacement::draft(0x141c91ec0, "contentcore_141c91ec0", tpl_39050::<0x1432a6290, 0x1432a6298, 0x140a00c00> as *const ()), // contentcore::NWEDDecalComponent::ClassTypeInfoImplementation::contentcore::NWEDDecalComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141c92370, "contentcore_141c92370", fn_141c92370 as *const ()), // contentcore::NWEDDecalComponent::ClassTypeInfoImplementation::contentcore::NWEDDecalComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141c92400, "contentcore_141c92400", fn_141c92400 as *const ()), // contentcore::NWEDDecalComponent::ClassTypeInfoImplementation::contentcore::NWEDDecalComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141c92cd0, "contentcore_141c92cd0", tpl_39049::<0x8, 0x1432a6278, 0x1432a6210, 0x1409fde50> as *const ()), // contentcore::ScriptPrototypeComponent::contentcore::ScriptPrototypeComponent::vfunc_22
    Replacement::draft(0x141c92e20, "contentcore_141c92e20", tpl_39050::<0x1432a6278, 0x1432a6210, 0x1409fde50> as *const ()), // contentcore::ScriptPrototypeComponent::ClassTypeInfoImplementation::contentcore::ScriptPrototypeComponent::ClassTypeInfo
    Replacement::draft(0x141c93300, "contentcore_141c93300", tpl_39013::<0x14328d190, 0x8> as *const ()), // contentcore::ScriptPrototypeComponent::ClassTypeInfoImplementation::contentcore::ScriptPrototypeComponent::ClassTypeInfo
    Replacement::draft_throws(0x141c93470, "contentcore_141c93470", tpl_39061::<0x8, 0x1432442c0, 0x1432a6268, 0x1402c4e80> as *const (), Throws::Discard), // contentcore::BoxShapeComponent::contentcore::BoxShapeComponent::vfunc_23
    Replacement::draft(0x141c93540, "contentcore_141c93540", tpl_38929::<0x8, 0x10> as *const ()), // contentcore::BoxShapeComponent::contentcore::BoxShapeComponent::vfunc_33
    Replacement::draft_throws(0x141c935f0, "contentcore_141c935f0", fn_141c935f0 as *const (), Throws::Discard), // contentcore::BoxShapeComponent::ClassTypeInfoImplementation::contentcore::BoxShapeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c93880, "contentcore_141c93880", tpl_38930::<0x8, 0x10> as *const ()), // contentcore::BoxShapeComponent::ClassTypeInfoImplementation::contentcore::BoxShapeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c93a50, "contentcore_141c93a50", tpl_38931::<0x8, 0x10> as *const ()), // contentcore::BoxShapeComponent::ClassTypeInfoImplementation::contentcore::BoxShapeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c93aa0, "contentcore_141c93aa0", tpl_39011::<0x14328a820, 0x8, 0xc, 0x10> as *const ()), // contentcore::BoxShapeComponent::ClassTypeInfoImplementation::contentcore::BoxShapeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c93c60, "contentcore_141c93c60", tpl_365::<0x1409d5170> as *const ()), // contentcore::ContactSoundMaterialDatabaseComponent::contentcore::ContactSoundMaterialDatabaseComponent::vfunc_27
    Replacement::draft(0x141c941a0, "contentcore_141c941a0", fn_141c941a0 as *const ()), // contentcore::ContactSoundMaterialDatabaseComponent::ClassTypeInfoImplementation::contentcore::ContactSoundMaterialDataba
    Replacement::draft(0x141c94820, "contentcore_141c94820", fn_141c94820 as *const ()), // contentcore::ContactSoundMaterialConfig::ClassTypeInfoImplementation::contentcore::ContactSoundMaterialConfig::ClassType
    Replacement::draft(0x141c94860, "contentcore_141c94860", fn_141c94860 as *const ()), // contentcore::ContactSoundMaterialConfig::ClassTypeInfoImplementation::contentcore::ContactSoundMaterialConfig::ClassType
    Replacement::draft(0x141c948f0, "contentcore_141c948f0", fn_141c948f0 as *const ()), // contentcore::ContactSoundMaterialConfig::ClassTypeInfoImplementation::contentcore::ContactSoundMaterialConfig::ClassType
    Replacement::draft(0x141c94970, "contentcore_141c94970", fn_141c94970 as *const ()), // contentcore::ContactSoundMaterialConfig::ClassTypeInfoImplementation::contentcore::ContactSoundMaterialConfig::ClassType
    Replacement::draft(0x141c94a60, "contentcore_141c94a60", tpl_39074::<0x14384e748, 0x1432a4e70, 0x1432a4e90, 0x1432a4e60, 0x1401de410, 0x14, 0x14384e738, 0x1432a4c28, 0x1432a4ed8> as *const ()), // contentcore::ContactSoundSyncParameters::ClassTypeInfoImplementation::contentcore::ContactSoundSyncParameters::ClassType
    Replacement::draft(0x141c94af0, "contentcore_141c94af0", tpl_39074::<0x14384e748, 0x1432a4e70, 0x1432a4e90, 0x1432a4e60, 0x1401de410, 0x14, 0x14384e738, 0x1432a4c28, 0x1432a4ed8> as *const ()), // contentcore::ContactSoundSyncParameters::ClassTypeInfoImplementation::contentcore::ContactSoundSyncParameters::ClassType
    Replacement::draft(0x141c94bc0, "contentcore_141c94bc0", tpl_39074::<0x14384e748, 0x1432a4e70, 0x1432a4e90, 0x1432a4e60, 0x1401de410, 0x14, 0x14384e738, 0x1432a4c28, 0x1432a4ed8> as *const ()), // contentcore::ContactSoundSyncParameters::ClassTypeInfoImplementation::contentcore::ContactSoundSyncParameters::ClassType
    Replacement::draft(0x141c94c50, "contentcore_141c94c50", tpl_39074::<0x14384e748, 0x1432a4e70, 0x1432a4e90, 0x1432a4e60, 0x1401de410, 0x14, 0x14384e738, 0x1432a4c28, 0x1432a4ed8> as *const ()), // contentcore::ContactSoundSyncParameters::ClassTypeInfoImplementation::contentcore::ContactSoundSyncParameters::ClassType
    Replacement::draft(0x141c95110, "contentcore_141c95110", fn_141c95110 as *const ()), // contentcore::ContactSoundSyncParameters::ClassTypeInfoImplementation::contentcore::ContactSoundSyncParameters::ClassType
    Replacement::draft(0x141c951a0, "contentcore_141c951a0", fn_141c951a0 as *const ()), // contentcore::ContactSoundSyncParameters::ClassTypeInfoImplementation::contentcore::ContactSoundSyncParameters::ClassType
    Replacement::draft(0x141c95200, "contentcore_141c95200", tpl_35184::<0x18, 0x14240ab10, 0x142442de0> as *const ()), // contentcore::ContactSoundSyncParameters::ClassTypeInfoImplementation::contentcore::ContactSoundSyncParameters::ClassType
    Replacement::draft(0x141c95570, "contentcore_141c95570", fn_141c95570 as *const ()), // contentcore::ContactSoundTriggerEvents::ClassTypeInfoImplementation::contentcore::ContactSoundTriggerEvents::ClassTypeIn
    Replacement::draft(0x141c955e0, "contentcore_141c955e0", fn_141c955e0 as *const ()), // contentcore::ContactSoundTriggerEvents::ClassTypeInfoImplementation::contentcore::ContactSoundTriggerEvents::ClassTypeIn
    Replacement::draft(0x141c95860, "contentcore_141c95860", tpl_38500::<0x20> as *const ()), // contentcore::ContactSoundTriggerEvents::ClassTypeInfoImplementation::contentcore::ContactSoundTriggerEvents::ClassTypeIn
    Replacement::draft(0x141c958a0, "contentcore_141c958a0", fn_141c958a0 as *const ()), // contentcore::ContactSoundTriggerEvents::ClassTypeInfoImplementation::contentcore::ContactSoundTriggerEvents::ClassTypeIn
    Replacement::draft(0x141c95f40, "contentcore_141c95f40", tpl_39079::<0x8, 0x14240aa00, 0x20, 0x28, 0x1409f8d30> as *const ()), // contentcore::ContactSoundOverrideComponent::ClassTypeInfoImplementation::contentcore::ContactSoundOverrideComponent::Cla
    Replacement::draft(0x141c95fb0, "contentcore_141c95fb0", fn_141c95fb0 as *const ()), // contentcore::ContactSoundOverrideComponent::ClassTypeInfoImplementation::contentcore::ContactSoundOverrideComponent::Cla
    Replacement::draft(0x141c96170, "contentcore_141c96170", tpl_39081::<0x14384e710, 0x143106ac0, 0x1432a51d8, 0x1401de410> as *const ()), // contentcore::ContactSoundEventOverride::ClassTypeInfoImplementation::contentcore::ContactSoundEventOverride::ClassTypeIn
    Replacement::draft(0x141c961c0, "contentcore_141c961c0", tpl_39081::<0x14384e710, 0x143106ac0, 0x1432a51d8, 0x1401de410> as *const ()), // contentcore::ContactSoundEventOverride::ClassTypeInfoImplementation::contentcore::ContactSoundEventOverride::ClassTypeIn
    Replacement::draft(0x141c96450, "contentcore_141c96450", tpl_39082::<0x8, 0x10, 0x14> as *const ()), // contentcore::ContactSoundEventOverride::ClassTypeInfoImplementation::contentcore::ContactSoundEventOverride::ClassTypeIn
    Replacement::draft(0x141c964a0, "contentcore_141c964a0", tpl_39082::<0x8, 0x10, 0x14> as *const ()), // contentcore::ContactSoundEventOverride::ClassTypeInfoImplementation::contentcore::ContactSoundEventOverride::ClassTypeIn
    Replacement::draft(0x141c966e0, "contentcore_141c966e0", tpl_35226::<0x10> as *const ()), // contentcore::ContactSoundEventOverride::ClassTypeInfoImplementation::contentcore::ContactSoundEventOverride::ClassTypeIn
    Replacement::draft(0x141c96720, "contentcore_141c96720", fn_141c96720 as *const ()), // contentcore::ContactSoundEventOverride::ClassTypeInfoImplementation::contentcore::ContactSoundEventOverride::ClassTypeIn
    Replacement::draft(0x141c96f30, "contentcore_141c96f30", tpl_38803::<0x8, 0x14240aa00, 0x20> as *const ()), // contentcore::DialogueVoiceProfileComponent::contentcore::DialogueVoiceProfileComponent::vfunc_33
    Replacement::draft(0x141c97260, "contentcore_141c97260", tpl_38804::<0x8, 0x14240aa00, 0x20> as *const ()), // contentcore::DialogueVoiceProfileComponent::ClassTypeInfoImplementation::contentcore::DialogueVoiceProfileComponent::Cla
    Replacement::draft(0x141c97460, "contentcore_141c97460", tpl_38805::<0x8, 0x14240aa00, 0x20> as *const ()), // contentcore::DialogueVoiceProfileComponent::ClassTypeInfoImplementation::contentcore::DialogueVoiceProfileComponent::Cla
    Replacement::draft(0x141c974d0, "contentcore_141c974d0", fn_141c974d0 as *const ()), // contentcore::DialogueVoiceProfileComponent::ClassTypeInfoImplementation::contentcore::DialogueVoiceProfileComponent::Cla
    Replacement::draft(0x141c97e40, "content_141c97e40", fn_141c97e40 as *const ()), // content::TriggerComponent::content::TriggerComponent::vfunc_33
    Replacement::draft(0x141c981b0, "content_141c981b0", fn_141c981b0 as *const ()), // content::TriggerComponent::ClassTypeInfoImplementation::content::TriggerComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c983f0, "content_141c983f0", fn_141c983f0 as *const ()), // content::TriggerComponent::ClassTypeInfoImplementation::content::TriggerComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c985c0, "content_141c985c0", tpl_38683::<0x1430af010, 0x30, 0x2, 0x1406db220> as *const ()), // content::TransformComponent::content::TransformComponent::vfunc_22
    Replacement::draft(0x141c986a0, "content_141c986a0", fn_141c986a0 as *const ()), // content::TransformComponent::content::TransformComponent::vfunc_33
    Replacement::draft(0x141c98730, "content_141c98730", tpl_35205::<0x1430af010, 0x2, 0x1406db220> as *const ()), // content::TransformComponent::ClassTypeInfoImplementation::content::TransformComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c989f0, "content_141c989f0", fn_141c989f0 as *const ()), // content::TransformComponent::ClassTypeInfoImplementation::content::TransformComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c98bd0, "content_141c98bd0", fn_141c98bd0 as *const ()), // content::TransformComponent::ClassTypeInfoImplementation::content::TransformComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c98c30, "content_141c98c30", fn_141c98c30 as *const ()), // content::TransformComponent::ClassTypeInfoImplementation::content::TransformComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c99210, "content_141c99210", fn_141c99210 as *const ()), // content::TimelineComponent::ClassTypeInfoImplementation::content::TimelineComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c992b0, "content_141c992b0", fn_141c992b0 as *const ()), // content::TimelineComponent::ClassTypeInfoImplementation::content::TimelineComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141c99e00, "content_141c99e00", fn_141c99e00 as *const ()), // content::TagComponent::content::TagComponent::vfunc_33
    Replacement::draft_throws(0x141c99e90, "content_141c99e90", tpl_838::<0x1409c4890> as *const (), Throws::Discard), // content::TagComponent::content::TagComponent::vfunc_36
    Replacement::draft(0x141c9a150, "content_141c9a150", fn_141c9a150 as *const ()), // content::TagComponent::ClassTypeInfoImplementation::content::TagComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141c9b1b0, "content_141c9b1b0", fn_141c9b1b0 as *const ()), // content::StreamingStateComponent::ClassTypeInfoImplementation::content::StreamingStateComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141c9bfb0, "content_141c9bfb0", fn_141c9bfb0 as *const ()), // content::SpotLightComponent::ClassTypeInfoImplementation::content::SpotLightComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141c9c760, "content_141c9c760", tpl_38695::<0x8> as *const ()), // content::SphereShapeComponent::ClassTypeInfoImplementation::content::SphereShapeComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141c9c9c0, "content_141c9c9c0", tpl_38803::<0x8, 0x1401e84a0, 0x20> as *const ()), // content::SharedEntityCollectionRefComponent::content::SharedEntityCollectionRefComponent::vfunc_33
    Replacement::draft(0x141c9cd00, "content_141c9cd00", tpl_38804::<0x8, 0x1401e84a0, 0x20> as *const ()), // content::SharedEntityCollectionRefComponent::ClassTypeInfoImplementation::content::SharedEntityCollectionRefComponent::C
    Replacement::draft(0x141c9cf00, "content_141c9cf00", tpl_38805::<0x8, 0x1401e84a0, 0x20> as *const ()), // content::SharedEntityCollectionRefComponent::ClassTypeInfoImplementation::content::SharedEntityCollectionRefComponent::C
    Replacement::draft(0x141c9cf70, "content_141c9cf70", tpl_39098::<0x14328e578, 0x8, 0x10, 0x18, 0x1c, 0x20, 0x28, 0x30, 0x34> as *const ()), // content::SharedEntityCollectionRefComponent::ClassTypeInfoImplementation::content::SharedEntityCollectionRefComponent::C
    Replacement::draft(0x141c9d670, "content_141c9d670", tpl_39060::<0x8, 0x140a34d40> as *const ()), // content::SharedEntityCollectionComponent::ClassTypeInfoImplementation::content::SharedEntityCollectionComponent::ClassTy
    Replacement::draft(0x141c9d830, "content_141c9d830", tpl_39099::<0x1432a29e0, 0x4c, 0x143874370, 0x1432a2a28, 0x1401de410, 0x50, 0x1438742a8, 0x1432a4698, 0x1432a46e0> as *const ()), // content::SoundSpawnerComponent::content::SoundSpawnerComponent::vfunc_22
    Replacement::draft(0x141c9d8d0, "content_141c9d8d0", tpl_39099::<0x1432a29e0, 0x4c, 0x143874410, 0x1432a2a28, 0x1401de410, 0x50, 0x143873fb0, 0x1432a4698, 0x1432a46e0> as *const ()), // content::SoundSpawnerComponent::content::SoundSpawnerComponent::vfunc_24
    Replacement::draft(0x141c9da40, "content_141c9da40", tpl_39100::<0x143874370, 0x1432a29e0, 0x1432a2a28, 0x1401de410, 0x50, 0x1438742a8, 0x1432a4698, 0x1432a46e0> as *const ()), // content::SoundSpawnerComponent::ClassTypeInfoImplementation::content::SoundSpawnerComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c9db30, "content_141c9db30", tpl_39100::<0x143874410, 0x1432a29e0, 0x1432a2a28, 0x1401de410, 0x50, 0x143873fb0, 0x1432a4698, 0x1432a46e0> as *const ()), // content::SoundSpawnerComponent::ClassTypeInfoImplementation::content::SoundSpawnerComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c9dfb0, "content_141c9dfb0", fn_141c9dfb0 as *const ()), // content::SoundSpawnerComponent::ClassTypeInfoImplementation::content::SoundSpawnerComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c9e0a0, "content_141c9e0a0", tpl_39102::<0x14329bee0, 0x4c, 0x54, 0x5c, 0x64> as *const ()), // content::SoundSpawnerComponent::ClassTypeInfoImplementation::content::SoundSpawnerComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141c9e7b0, "content_141c9e7b0", fn_141c9e7b0 as *const ()), // content::SoundComponent::ClassTypeInfoImplementation::content::SoundComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141c9e880, "content_141c9e880", fn_141c9e880 as *const ()), // content::SoundComponent::ClassTypeInfoImplementation::content::SoundComponent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141c9f170, "content_141c9f170", tpl_38924::<0x8, 0x20, 0xc, 0x14020bcb0> as *const ()), // content::SkeletonComponent::content::SkeletonComponent::vfunc_24
    Replacement::draft(0x141c9f320, "content_141c9f320", tpl_38925::<0x8, 0x20, 0xc, 0x14020bcb0> as *const ()), // content::SkeletonComponent::ClassTypeInfoImplementation::content::SkeletonComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141ca0250, "content_141ca0250", tpl_39105::<0x1432a2da0, 0x8, 0x1432a2dc8, 0x1432a2d88, 0x1409ff500> as *const ()), // content::SDFVolumeComponent::content::SDFVolumeComponent::vfunc_22
    Replacement::draft(0x141ca0390, "content_141ca0390", fn_141ca0390 as *const ()), // content::SDFVolumeComponent::content::SDFVolumeComponent::vfunc_33
    Replacement::draft(0x141ca0420, "content_141ca0420", tpl_39107::<0x1432a2dc8, 0x1432a2da0, 0x1432a2d88, 0x1409ff500> as *const ()), // content::SDFVolumeComponent::ClassTypeInfoImplementation::content::SDFVolumeComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ca0750, "content_141ca0750", fn_141ca0750 as *const ()), // content::SDFVolumeComponent::ClassTypeInfoImplementation::content::SDFVolumeComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ca0940, "content_141ca0940", fn_141ca0940 as *const ()), // content::SDFVolumeComponent::ClassTypeInfoImplementation::content::SDFVolumeComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ca1080, "content_141ca1080", fn_141ca1080 as *const ()), // content::ScatterPointsComponent::ClassTypeInfoImplementation::content::ScatterPointsComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141ca1190, "content_141ca1190", fn_141ca1190 as *const ()), // content::ScatterPointsComponent::ClassTypeInfoImplementation::content::ScatterPointsComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141ca18e0, "content_141ca18e0", fn_141ca18e0 as *const ()), // content::ScatterParamsComponent::ClassTypeInfoImplementation::content::ScatterParamsComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141ca19a0, "content_141ca19a0", tpl_39113::<0x14328ca08, 0x10, 0x24, 0x2c, 0x34, 0x50, 0x58> as *const ()), // content::ScatterParamsComponent::ClassTypeInfoImplementation::content::ScatterParamsComponent::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141ca2450, "content_141ca2450", tpl_38888::<0x1432a3528, 0x8, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // content::RenderBoxComponent::content::RenderBoxComponent::vfunc_23
    Replacement::draft(0x141ca2520, "content_141ca2520", tpl_38929::<0x8, 0x10> as *const ()), // content::RenderBoxComponent::content::RenderBoxComponent::vfunc_33
    Replacement::draft_throws(0x141ca25d0, "content_141ca25d0", tpl_38928::<0x1432a3528, 0x1, 0x1402c4e80> as *const (), Throws::Discard), // content::RenderBoxComponent::ClassTypeInfoImplementation::content::RenderBoxComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ca2860, "content_141ca2860", tpl_38930::<0x8, 0x10> as *const ()), // content::RenderBoxComponent::ClassTypeInfoImplementation::content::RenderBoxComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ca2a30, "content_141ca2a30", tpl_38931::<0x8, 0x10> as *const ()), // content::RenderBoxComponent::ClassTypeInfoImplementation::content::RenderBoxComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ca2c90, "content_141ca2c90", fn_141ca2c90 as *const ()), // content::RecoilComponent::content::RecoilComponent::vfunc_33
    Replacement::draft(0x141ca3000, "content_141ca3000", fn_141ca3000 as *const ()), // content::RecoilComponent::ClassTypeInfoImplementation::content::RecoilComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141ca3220, "content_141ca3220", fn_141ca3220 as *const ()), // content::RecoilComponent::ClassTypeInfoImplementation::content::RecoilComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ca33f0, "content_141ca33f0", tpl_38546::<0x1432a3690, 0x8, 0x1432a35f8, 0x1432a3628, 0x1409fe6c0> as *const ()), // content::RBFComponent::content::RBFComponent::vfunc_22
    Replacement::draft(0x141ca3550, "content_141ca3550", tpl_37078::<0x1432a35f8, 0x1432a3690, 0x1432a3628, 0x1409fe6c0> as *const ()), // content::RBFComponent::ClassTypeInfoImplementation::content::RBFComponent::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141ca4170, "content_141ca4170", tpl_39117::<0x143290408, 0x8, 0xb, 0xc, 0x10> as *const ()), // content::RagdollRecoveryComponent::ClassTypeInfoImplementation::content::RagdollRecoveryComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ca49f0, "content_141ca49f0", fn_141ca49f0 as *const ()), // content::RagdollControllerComponent::content::RagdollControllerComponent::vfunc_23
    Replacement::draft(0x141ca4ad0, "content_141ca4ad0", fn_141ca4ad0 as *const ()), // content::RagdollControllerComponent::content::RagdollControllerComponent::vfunc_33
    Replacement::draft(0x141ca4bc0, "content_141ca4bc0", fn_141ca4bc0 as *const ()), // content::RagdollControllerComponent::ClassTypeInfoImplementation::content::RagdollControllerComponent::ClassTypeInfoImpl
    Replacement::draft(0x141ca4e70, "content_141ca4e70", fn_141ca4e70 as *const ()), // content::RagdollControllerComponent::ClassTypeInfoImplementation::content::RagdollControllerComponent::ClassTypeInfoImpl
    Replacement::draft(0x141ca5080, "content_141ca5080", fn_141ca5080 as *const ()), // content::RagdollControllerComponent::ClassTypeInfoImplementation::content::RagdollControllerComponent::ClassTypeInfoImpl
    Replacement::draft(0x141ca50e0, "content_141ca50e0", fn_141ca50e0 as *const ()), // content::RagdollControllerComponent::ClassTypeInfoImplementation::content::RagdollControllerComponent::ClassTypeInfoImpl
    Replacement::draft(0x141ca5300, "content_141ca5300", fn_141ca5300 as *const ()), // content::PointOfInterestComponent::content::PointOfInterestComponent::vfunc_33
    Replacement::draft(0x141ca5660, "content_141ca5660", fn_141ca5660 as *const ()), // content::PointOfInterestComponent::ClassTypeInfoImplementation::content::PointOfInterestComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ca5880, "content_141ca5880", fn_141ca5880 as *const ()), // content::PointOfInterestComponent::ClassTypeInfoImplementation::content::PointOfInterestComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ca58f0, "content_141ca58f0", tpl_38738::<0x14329a9a0, 0x8, 0x10, 0x18> as *const ()), // content::PointOfInterestComponent::ClassTypeInfoImplementation::content::PointOfInterestComponent::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141ca5ae0, "content_141ca5ae0", tpl_838::<0x14099ec20> as *const (), Throws::Discard), // content::PointLightComponent::content::PointLightComponent::vfunc_30
    Replacement::draft(0x141ca6000, "content_141ca6000", fn_141ca6000 as *const ()), // content::PointLightComponent::ClassTypeInfoImplementation::content::PointLightComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141ca6100, "content_141ca6100", fn_141ca6100 as *const ()), // content::PointLightComponent::ClassTypeInfoImplementation::content::PointLightComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141ca6520, "content_141ca6520", tpl_38804::<0x8, 0x1401e84a0, 0x20> as *const ()), // content::PlayerCameraComponent::ClassTypeInfoImplementation::content::PlayerCameraComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141ca6700, "content_141ca6700", tpl_38805::<0x8, 0x1401e84a0, 0x20> as *const ()), // content::PlayerCameraComponent::ClassTypeInfoImplementation::content::PlayerCameraComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141ca6770, "content_141ca6770", tpl_39098::<0x1431d8a50, 0x8, 0x10, 0x18, 0x1c, 0x20, 0x28, 0x30, 0x34> as *const ()), // content::PlayerCameraComponent::ClassTypeInfoImplementation::content::PlayerCameraComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141ca69a0, "content_141ca69a0", tpl_39129::<0x10, 0x8, 0x14, 0x18> as *const ()), // content::PlaneShapeComponent::content::PlaneShapeComponent::vfunc_33
    Replacement::draft(0x141ca6cd0, "content_141ca6cd0", tpl_39130::<0x8, 0x10, 0x14, 0x18> as *const ()), // content::PlaneShapeComponent::ClassTypeInfoImplementation::content::PlaneShapeComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141ca6ec0, "content_141ca6ec0", fn_141ca6ec0 as *const ()), // content::PlaneShapeComponent::ClassTypeInfoImplementation::content::PlaneShapeComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141ca7120, "content_141ca7120", fn_141ca7120 as *const ()), // content::PerspectiveViewComponent::content::PerspectiveViewComponent::vfunc_33
    Replacement::draft(0x141ca7470, "content_141ca7470", fn_141ca7470 as *const ()), // content::PerspectiveViewComponent::ClassTypeInfoImplementation::content::PerspectiveViewComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ca7670, "content_141ca7670", fn_141ca7670 as *const ()), // content::PerspectiveViewComponent::ClassTypeInfoImplementation::content::PerspectiveViewComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ca76d0, "content_141ca76d0", tpl_39135::<0x1432839b0, 0x14> as *const ()), // content::PerspectiveViewComponent::ClassTypeInfoImplementation::content::PerspectiveViewComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ca7830, "content_141ca7830", tpl_39136::<0x8, 0x1432a20e8, 0x1432a2108, 0x14082a150> as *const ()), // content::ParticleSystemComponent::content::ParticleSystemComponent::vfunc_22
    Replacement::draft(0x141ca7910, "content_141ca7910", fn_141ca7910 as *const ()), // content::ParticleSystemComponent::content::ParticleSystemComponent::vfunc_33
    Replacement::draft(0x141ca7980, "content_141ca7980", tpl_365::<0x140998c70> as *const ()), // content::ParticleSystemComponent::content::ParticleSystemComponent::vfunc_38
    Replacement::draft(0x141ca7990, "content_141ca7990", tpl_39138::<0x1432a20e8, 0x1432a2108, 0x14082a150> as *const ()), // content::ParticleSystemComponent::ClassTypeInfoImplementation::content::ParticleSystemComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141ca7c60, "content_141ca7c60", fn_141ca7c60 as *const ()), // content::ParticleSystemComponent::ClassTypeInfoImplementation::content::ParticleSystemComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141ca7e40, "content_141ca7e40", tpl_39140::<0x8, 0x10> as *const ()), // content::ParticleSystemComponent::ClassTypeInfoImplementation::content::ParticleSystemComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141ca80a0, "content_141ca80a0", tpl_39141::<0x8, 0xc, 0x10> as *const ()), // content::OrthographicViewComponent::content::OrthographicViewComponent::vfunc_33
    Replacement::draft(0x141ca83d0, "content_141ca83d0", tpl_39142::<0x8, 0xc, 0x10> as *const ()), // content::OrthographicViewComponent::ClassTypeInfoImplementation::content::OrthographicViewComponent::ClassTypeInfoImplem
    Replacement::draft(0x141ca85c0, "content_141ca85c0", tpl_39143::<0x8, 0xc, 0x10> as *const ()), // content::OrthographicViewComponent::ClassTypeInfoImplementation::content::OrthographicViewComponent::ClassTypeInfoImplem
    Replacement::draft(0x141ca8610, "content_141ca8610", tpl_39135::<0x14328c058, 0x10> as *const ()), // content::OrthographicViewComponent::ClassTypeInfoImplementation::content::OrthographicViewComponent::ClassTypeInfoImplem
    Replacement::draft(0x141caa690, "content_141caa690", fn_141caa690 as *const ()), // content::MeshComponent::ClassTypeInfoImplementation::content::MeshComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141caa720, "content_141caa720", tpl_39145::<0x14329a848, 0x8, 0x14, 0x20, 0x28> as *const ()), // content::MeshComponent::ClassTypeInfoImplementation::content::MeshComponent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141caa940, "content_141caa940", fn_141caa940 as *const ()), // content::LuaScriptNameComponent::content::LuaScriptNameComponent::vfunc_33
    Replacement::draft(0x141caac70, "content_141caac70", fn_141caac70 as *const ()), // content::LuaScriptNameComponent::ClassTypeInfoImplementation::content::LuaScriptNameComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141cab0e0, "content_141cab0e0", tpl_39148::<0x8, 0x14028e490, 0x14183acbc> as *const ()), // content::LuaSystemsGlueExportedComponent::content::LuaSystemsGlueExportedComponent::vfunc_33
    Replacement::draft(0x141cab410, "content_141cab410", tpl_39149::<0x8, 0x14028e490, 0x14183acbc> as *const ()), // content::LuaSystemsGlueExportedComponent::ClassTypeInfoImplementation::content::LuaSystemsGlueExportedComponent::ClassTy
    Replacement::draft(0x141cab870, "content_141cab870", tpl_39148::<0x8, 0x14028e490, 0x14183acbc> as *const ()), // content::LuaScriptExportedComponent::content::LuaScriptExportedComponent::vfunc_33
    Replacement::draft(0x141cabba0, "content_141cabba0", tpl_39149::<0x8, 0x14028e490, 0x14183acbc> as *const ()), // content::LuaScriptExportedComponent::ClassTypeInfoImplementation::content::LuaScriptExportedComponent::ClassTypeInfoImpl
    Replacement::draft_throws(0x141cac4e0, "content_141cac4e0", fn_141cac4e0 as *const (), Throws::Discard), // content::LuaScriptComponent::ClassTypeInfoImplementation::content::LuaScriptComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cac5e0, "content_141cac5e0", fn_141cac5e0 as *const ()), // content::LuaScriptComponent::ClassTypeInfoImplementation::content::LuaScriptComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cad0d0, "content_141cad0d0", tpl_38546::<0x1432a04b0, 0x8, 0x1432a04d8, 0x1432a0490, 0x1409fa050> as *const ()), // content::LogicAnimationComponent::content::LogicAnimationComponent::vfunc_22
    Replacement::draft(0x141cad1c0, "content_141cad1c0", fn_141cad1c0 as *const ()), // content::LogicAnimationComponent::content::LogicAnimationComponent::vfunc_33
    Replacement::draft(0x141cad280, "content_141cad280", tpl_37078::<0x1432a04d8, 0x1432a04b0, 0x1432a0490, 0x1409fa050> as *const ()), // content::LogicAnimationComponent::ClassTypeInfoImplementation::content::LogicAnimationComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cad570, "content_141cad570", fn_141cad570 as *const ()), // content::LogicAnimationComponent::ClassTypeInfoImplementation::content::LogicAnimationComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cad780, "content_141cad780", fn_141cad780 as *const ()), // content::LogicAnimationComponent::ClassTypeInfoImplementation::content::LogicAnimationComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141caf260, "content_141caf260", tpl_238937 as *const ()), // sub_141caf260
    Replacement::draft(0x141caf620, "content_141caf620", fn_141caf620 as *const ()), // content::IlluminationVolumeComponent::ClassTypeInfoImplementation::content::IlluminationVolumeComponent::ClassTypeInfoIm
    Replacement::draft(0x141caf6d0, "content_141caf6d0", fn_141caf6d0 as *const ()), // content::IlluminationVolumeComponent::ClassTypeInfoImplementation::content::IlluminationVolumeComponent::ClassTypeInfoIm
    Replacement::draft(0x141cafe30, "content_141cafe30", fn_141cafe30 as *const ()), // content::HierarchyLODComponent::ClassTypeInfoImplementation::content::HierarchyLODComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141cb0640, "content_141cb0640", fn_141cb0640 as *const ()), // content::HierarchyGroupComponent::ClassTypeInfoImplementation::content::HierarchyGroupComponent::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141cb0890, "content_141cb0890", tpl_838::<0x140986d50> as *const (), Throws::Discard), // content::HeightfieldExportVolumeComponent::content::HeightfieldExportVolumeComponent::vfunc_32
    Replacement::draft(0x141cb08a0, "content_141cb08a0", fn_141cb08a0 as *const ()), // content::HeightfieldExportVolumeComponent::content::HeightfieldExportVolumeComponent::vfunc_33
    Replacement::draft(0x141cb0c30, "content_141cb0c30", fn_141cb0c30 as *const ()), // content::HeightfieldExportVolumeComponent::ClassTypeInfoImplementation::content::HeightfieldExportVolumeComponent::Class
    Replacement::draft(0x141cb0e40, "content_141cb0e40", fn_141cb0e40 as *const ()), // content::HeightfieldExportVolumeComponent::ClassTypeInfoImplementation::content::HeightfieldExportVolumeComponent::Class
    Replacement::draft_throws(0x141cb1080, "content_141cb1080", tpl_838::<0x140985270> as *const (), Throws::Discard), // content::HeightfieldLayerComponent::content::HeightfieldLayerComponent::vfunc_30
    Replacement::draft(0x141cb15a0, "content_141cb15a0", fn_141cb15a0 as *const ()), // content::HeightfieldLayerComponent::ClassTypeInfoImplementation::content::HeightfieldLayerComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cb1650, "content_141cb1650", fn_141cb1650 as *const ()), // content::HeightfieldLayerComponent::ClassTypeInfoImplementation::content::HeightfieldLayerComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cb1db0, "content_141cb1db0", fn_141cb1db0 as *const ()), // content::HeightfieldComponent::ClassTypeInfoImplementation::content::HeightfieldComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cb2df0, "content_141cb2df0", fn_141cb2df0 as *const ()), // content::GfxGraphSpawnerComponent::ClassTypeInfoImplementation::content::GfxGraphSpawnerComponent::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141cb3550, "content_141cb3550", fn_141cb3550 as *const (), Throws::Discard), // content::GfxGraphComponent::ClassTypeInfoImplementation::content::GfxGraphComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cb35d0, "content_141cb35d0", tpl_38738::<0x143285990, 0x8, 0x18, 0x20> as *const ()), // content::GfxGraphComponent::ClassTypeInfoImplementation::content::GfxGraphComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft_throws(0x141cb49f0, "content_141cb49f0", tpl_838::<0x14097fb10> as *const (), Throws::Discard), // content::ForceStreamingByPointModifierComponent::content::ForceStreamingByPointModifierComponent::vfunc_25
    Replacement::draft(0x141cb53b0, "content_141cb53b0", tpl_38544::<0x8, 0x18> as *const ()), // content::FogBlockerComponent::content::FogBlockerComponent::vfunc_33
    Replacement::draft(0x141cb56c0, "content_141cb56c0", tpl_38493::<0x8, 0x18> as *const ()), // content::FogBlockerComponent::ClassTypeInfoImplementation::content::FogBlockerComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cb58a0, "content_141cb58a0", tpl_38494::<0x8, 0x18> as *const ()), // content::FogBlockerComponent::ClassTypeInfoImplementation::content::FogBlockerComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cb58f0, "content_141cb58f0", fn_141cb58f0 as *const ()), // content::FogBlockerComponent::ClassTypeInfoImplementation::content::FogBlockerComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cb5a80, "content_141cb5a80", tpl_38526::<0x1432a16e8, 0x10, 0x1438743a8, 0x1432a1710, 0x1432a16d0, 0x1401de410> as *const ()), // content::FogVolumeComponent::content::FogVolumeComponent::vfunc_22
    Replacement::draft(0x141cb5af0, "content_141cb5af0", tpl_38526::<0x1432a16e8, 0x10, 0x1438743e0, 0x1432a1710, 0x1432a16d0, 0x1401de410> as *const ()), // content::FogVolumeComponent::content::FogVolumeComponent::vfunc_24
    Replacement::draft(0x141cb5c30, "content_141cb5c30", tpl_39168::<0x1438743a8, 0x1432a16e8, 0x1432a1710, 0x1432a16d0, 0x1401de410> as *const ()), // content::FogVolumeComponent::ClassTypeInfoImplementation::content::FogVolumeComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cb5cf0, "content_141cb5cf0", tpl_39168::<0x1438743e0, 0x1432a16e8, 0x1432a1710, 0x1432a16d0, 0x1401de410> as *const ()), // content::FogVolumeComponent::ClassTypeInfoImplementation::content::FogVolumeComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cb6140, "content_141cb6140", fn_141cb6140 as *const ()), // content::FogVolumeComponent::ClassTypeInfoImplementation::content::FogVolumeComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cb61d0, "content_141cb61d0", fn_141cb61d0 as *const ()), // content::FogVolumeComponent::ClassTypeInfoImplementation::content::FogVolumeComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141cb63a0, "content_141cb63a0", tpl_838::<0x14097c9a0> as *const (), Throws::Discard), // content::FlickerComponent::content::FlickerComponent::vfunc_25
    Replacement::draft(0x141cb6420, "content_141cb6420", tpl_39171::<0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // content::FlickerComponent::content::FlickerComponent::vfunc_33
    Replacement::draft(0x141cb6770, "content_141cb6770", tpl_39172::<0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // content::FlickerComponent::ClassTypeInfoImplementation::content::FlickerComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141cb6b30, "content_141cb6b30", tpl_39173::<0x8, 0x14329f718, 0x14329f730, 0x1409fc9b0> as *const ()), // content::FlareComponent::content::FlareComponent::vfunc_22
    Replacement::draft(0x141cb6c10, "content_141cb6c10", fn_141cb6c10 as *const ()), // content::FlareComponent::content::FlareComponent::vfunc_33
    Replacement::draft(0x141cb6ce0, "content_141cb6ce0", fn_141cb6ce0 as *const ()), // content::FlareComponent::ClassTypeInfoImplementation::content::FlareComponent::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141cb6fc0, "content_141cb6fc0", fn_141cb6fc0 as *const ()), // content::FlareComponent::ClassTypeInfoImplementation::content::FlareComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141cb71e0, "content_141cb71e0", fn_141cb71e0 as *const ()), // content::FlareComponent::ClassTypeInfoImplementation::content::FlareComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cb73d0, "content_141cb73d0", tpl_39014::<0x14329f6a0, 0x8, 0x14384e7e0, 0x14329f6c0, 0x14329f690, 0x1401de410> as *const ()), // content::FactionComponent::content::FactionComponent::vfunc_22
    Replacement::draft(0x141cb7430, "content_141cb7430", tpl_39014::<0x14329f6a0, 0x8, 0x14384e7e0, 0x14329f6c0, 0x14329f690, 0x1401de410> as *const ()), // content::FactionComponent::content::FactionComponent::vfunc_23
    Replacement::draft(0x141cb7490, "content_141cb7490", tpl_39014::<0x14329f6a0, 0x8, 0x14384e7e0, 0x14329f6c0, 0x14329f690, 0x1401de410> as *const ()), // content::FactionComponent::content::FactionComponent::vfunc_24
    Replacement::draft(0x141cb7510, "content_141cb7510", tpl_365::<0x140979970> as *const ()), // content::FactionComponent::content::FactionComponent::vfunc_27
    Replacement::draft(0x141cb75e0, "content_141cb75e0", tpl_39178::<0x14384e7e0, 0x14329f6a0, 0x14329f6c0, 0x14329f690, 0x1401de410> as *const ()), // content::FactionComponent::ClassTypeInfoImplementation::content::FactionComponent::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141cb7660, "content_141cb7660", tpl_39178::<0x14384e7e0, 0x14329f6a0, 0x14329f6c0, 0x14329f690, 0x1401de410> as *const ()), // content::FactionComponent::ClassTypeInfoImplementation::content::FactionComponent::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141cb76e0, "content_141cb76e0", tpl_39178::<0x14384e7e0, 0x14329f6a0, 0x14329f6c0, 0x14329f690, 0x1401de410> as *const ()), // content::FactionComponent::ClassTypeInfoImplementation::content::FactionComponent::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft_throws(0x141cb86a0, "content_141cb86a0", tpl_838::<0x140978580> as *const (), Throws::Discard), // content::EntitySpawnerComponent::content::EntitySpawnerComponent::vfunc_31
    Replacement::draft(0x141cbb100, "content_141cbb100", fn_141cbb100 as *const ()), // content::DirectionalLightComponent::content::DirectionalLightComponent::vfunc_33
    Replacement::draft(0x141cbb4b0, "content_141cbb4b0", fn_141cbb4b0 as *const ()), // content::DirectionalLightComponent::ClassTypeInfoImplementation::content::DirectionalLightComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cbb6f0, "content_141cbb6f0", fn_141cbb6f0 as *const ()), // content::DirectionalLightComponent::ClassTypeInfoImplementation::content::DirectionalLightComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cbb770, "content_141cbb770", fn_141cbb770 as *const ()), // content::DirectionalLightComponent::ClassTypeInfoImplementation::content::DirectionalLightComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cbbd40, "content_141cbbd40", fn_141cbbd40 as *const ()), // content::DialogueControllerComponent::ClassTypeInfoImplementation::content::DialogueControllerComponent::ClassTypeInfoIm
    Replacement::draft(0x141cbbde0, "content_141cbbde0", fn_141cbbde0 as *const ()), // content::DialogueControllerComponent::ClassTypeInfoImplementation::content::DialogueControllerComponent::ClassTypeInfoIm
    Replacement::draft(0x141cbdb40, "content_141cbdb40", tpl_38683::<0x14329fa10, 0x10, 0x1, 0x1409fbf20> as *const ()), // content::DecalComponent::content::DecalComponent::vfunc_22
    Replacement::draft(0x141cbdb90, "content_141cbdb90", tpl_38683::<0x14329fa10, 0x10, 0x1, 0x1409fbec0> as *const ()), // content::DecalComponent::content::DecalComponent::vfunc_24
    Replacement::draft(0x141cbdc50, "content_141cbdc50", fn_141cbdc50 as *const ()), // content::DecalComponent::content::DecalComponent::vfunc_33
    Replacement::draft(0x141cbdce0, "content_141cbdce0", tpl_35220::<0x14329fa10, 0x1, 0x1409fbf20> as *const ()), // content::DecalComponent::ClassTypeInfoImplementation::content::DecalComponent::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141cbdd70, "content_141cbdd70", tpl_35220::<0x14329fa10, 0x1, 0x1409fbec0> as *const ()), // content::DecalComponent::ClassTypeInfoImplementation::content::DecalComponent::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141cbdfd0, "content_141cbdfd0", fn_141cbdfd0 as *const ()), // content::DecalComponent::ClassTypeInfoImplementation::content::DecalComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141cbe1c0, "content_141cbe1c0", fn_141cbe1c0 as *const ()), // content::DecalComponent::ClassTypeInfoImplementation::content::DecalComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cbe210, "content_141cbe210", fn_141cbe210 as *const ()), // content::DecalComponent::ClassTypeInfoImplementation::content::DecalComponent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141cbe900, "content_141cbe900", fn_141cbe900 as *const ()), // content::DebugTextComponent::ClassTypeInfoImplementation::content::DebugTextComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cbe9d0, "content_141cbe9d0", fn_141cbe9d0 as *const ()), // content::DebugTextComponent::ClassTypeInfoImplementation::content::DebugTextComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cc0b10, "content_141cc0b10", tpl_39117::<0x14328f7a8, 0x8, 0xb, 0xc, 0x10> as *const ()), // content::ClothParamsFromBonesComponent::ClassTypeInfoImplementation::content::ClothParamsFromBonesComponent::ClassTypeIn
    Replacement::draft(0x141cc2330, "content_141cc2330", fn_141cc2330 as *const ()), // content::CharacterLookAtComponent::ClassTypeInfoImplementation::content::CharacterLookAtComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141cc23c0, "content_141cc23c0", fn_141cc23c0 as *const ()), // content::CharacterLookAtComponent::ClassTypeInfoImplementation::content::CharacterLookAtComponent::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141cc2620, "content_141cc2620", tpl_838::<0x140969030> as *const (), Throws::Discard), // content::CharacterFootstepComponent::content::CharacterFootstepComponent::vfunc_36
    Replacement::draft(0x141cc2ad0, "content_141cc2ad0", fn_141cc2ad0 as *const ()), // content::CharacterFootstepComponent::ClassTypeInfoImplementation::content::CharacterFootstepComponent::ClassTypeInfoImpl
    Replacement::draft(0x141cc2f60, "content_141cc2f60", fn_141cc2f60 as *const ()), // content::CharacterControllerComponent::ClassTypeInfoImplementation::content::CharacterControllerComponent::ClassTypeInfo
    Replacement::draft(0x141cc3140, "content_141cc3140", fn_141cc3140 as *const ()), // content::CharacterControllerComponent::ClassTypeInfoImplementation::content::CharacterControllerComponent::ClassTypeInfo
    Replacement::draft(0x141cc3ee0, "content_141cc3ee0", fn_141cc3ee0 as *const ()), // content::CameraComponent::ClassTypeInfoImplementation::content::CameraComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141cc40c0, "content_141cc40c0", fn_141cc40c0 as *const ()), // content::CameraComponent::ClassTypeInfoImplementation::content::CameraComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cc4520, "content_141cc4520", tpl_38930::<0x8, 0x10> as *const ()), // content::BoxShapeComponent::ClassTypeInfoImplementation::content::BoxShapeComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cc46d0, "content_141cc46d0", tpl_38931::<0x8, 0x10> as *const ()), // content::BoxShapeComponent::ClassTypeInfoImplementation::content::BoxShapeComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft_throws(0x141cc4890, "content_141cc4890", tpl_39009::<0x14329e980, 0x8, 0x14329e900, 0x14329e914, 0x1402c4e80> as *const (), Throws::Discard), // content::BoxComponent::content::BoxComponent::vfunc_23
    Replacement::draft(0x141cc4960, "content_141cc4960", tpl_38929::<0x8, 0x10> as *const ()), // content::BoxComponent::content::BoxComponent::vfunc_33
    Replacement::draft_throws(0x141cc4a10, "content_141cc4a10", tpl_39010::<0x14329e900, 0x14329e980, 0x14329e914, 0x1402c4e80> as *const (), Throws::Discard), // content::BoxComponent::ClassTypeInfoImplementation::content::BoxComponent::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141cc4ca0, "content_141cc4ca0", tpl_38930::<0x8, 0x10> as *const ()), // content::BoxComponent::ClassTypeInfoImplementation::content::BoxComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141cc4e70, "content_141cc4e70", tpl_38931::<0x8, 0x10> as *const ()), // content::BoxComponent::ClassTypeInfoImplementation::content::BoxComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cc5020, "content_141cc5020", fn_141cc5020 as *const ()), // content::BoneFoleyComponent::content::BoneFoleyComponent::vfunc_22
    Replacement::draft(0x141cc51a0, "content_141cc51a0", fn_141cc51a0 as *const ()), // content::BoneFoleyComponent::ClassTypeInfoImplementation::content::BoneFoleyComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cc5680, "content_141cc5680", fn_141cc5680 as *const ()), // content::BoneFoleyComponent::ClassTypeInfoImplementation::content::BoneFoleyComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cc5770, "content_141cc5770", fn_141cc5770 as *const ()), // content::BoneFoleyComponent::ClassTypeInfoImplementation::content::BoneFoleyComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141cc59a0, "content_141cc59a0", tpl_39202::<0x8, 0x14240aa00, 0x30, 0x58> as *const ()), // content::BoneAttachedWindAreaComponent::content::BoneAttachedWindAreaComponent::vfunc_33
    Replacement::draft(0x141cc5ce0, "content_141cc5ce0", tpl_39203::<0x8, 0x14240aa00, 0x30, 0x58> as *const ()), // content::BoneAttachedWindAreaComponent::ClassTypeInfoImplementation::content::BoneAttachedWindAreaComponent::ClassTypeIn
    Replacement::draft(0x141cc5ef0, "content_141cc5ef0", tpl_39204::<0x8, 0x14240aa00, 0x30, 0x58> as *const ()), // content::BoneAttachedWindAreaComponent::ClassTypeInfoImplementation::content::BoneAttachedWindAreaComponent::ClassTypeIn
    Replacement::draft(0x141cc5f70, "content_141cc5f70", fn_141cc5f70 as *const ()), // content::BoneAttachedWindAreaComponent::ClassTypeInfoImplementation::content::BoneAttachedWindAreaComponent::ClassTypeIn
    Replacement::draft(0x141cc6680, "content_141cc6680", fn_141cc6680 as *const ()), // content::AudioSwitchesComponent::ClassTypeInfoImplementation::content::AudioSwitchesComponent::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141cc71a0, "content_141cc71a0", fn_141cc71a0 as *const (), Throws::Discard), // content::AudioOffsetComponent::content::AudioOffsetComponent::vfunc_23
    Replacement::draft(0x141cc7270, "content_141cc7270", tpl_38929::<0x8, 0x10> as *const ()), // content::AudioOffsetComponent::content::AudioOffsetComponent::vfunc_33
    Replacement::draft_throws(0x141cc7320, "content_141cc7320", fn_141cc7320 as *const (), Throws::Discard), // content::AudioOffsetComponent::ClassTypeInfoImplementation::content::AudioOffsetComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cc75b0, "content_141cc75b0", tpl_38930::<0x8, 0x10> as *const ()), // content::AudioOffsetComponent::ClassTypeInfoImplementation::content::AudioOffsetComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cc7780, "content_141cc7780", tpl_38931::<0x8, 0x10> as *const ()), // content::AudioOffsetComponent::ClassTypeInfoImplementation::content::AudioOffsetComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cc8280, "content_141cc8280", tpl_39209::<0x1434269c8, 0x14329ecf8, 0xc, 0x14329ec48, 0x14329ecd8, 0x14020f270> as *const ()), // content::AudioAcousticsComponent::content::AudioAcousticsComponent::vfunc_23
    Replacement::draft(0x141cc8370, "content_141cc8370", fn_141cc8370 as *const ()), // content::AudioAcousticsComponent::content::AudioAcousticsComponent::vfunc_33
    Replacement::draft(0x141cc8430, "content_141cc8430", fn_141cc8430 as *const ()), // content::AudioAcousticsComponent::ClassTypeInfoImplementation::content::AudioAcousticsComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cc86e0, "content_141cc86e0", fn_141cc86e0 as *const ()), // content::AudioAcousticsComponent::ClassTypeInfoImplementation::content::AudioAcousticsComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cc88d0, "content_141cc88d0", tpl_38510::<0x8, 0x9, 0xc> as *const ()), // content::AudioAcousticsComponent::ClassTypeInfoImplementation::content::AudioAcousticsComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cca0a0, "content_141cca0a0", tpl_39140::<0x8, 0x10> as *const ()), // content::AreaVertexComponent::ClassTypeInfoImplementation::content::AreaVertexComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cca0f0, "content_141cca0f0", fn_141cca0f0 as *const ()), // content::AreaVertexComponent::ClassTypeInfoImplementation::content::AreaVertexComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cca860, "content_141cca860", tpl_38534::<0x8, 0x1403d79a0, 0x18> as *const ()), // content::AreaShapeComponent::ClassTypeInfoImplementation::content::AreaShapeComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ccaa30, "content_141ccaa30", tpl_39105::<0x14329ee08, 0x8, 0x14329ee28, 0x14329edf8, 0x1409fa280> as *const ()), // content::AnimationSystemComponent::content::AnimationSystemComponent::vfunc_22
    Replacement::draft(0x141ccab70, "content_141ccab70", tpl_365::<0x1409588a0> as *const ()), // content::AnimationSystemComponent::content::AnimationSystemComponent::vfunc_37
    Replacement::draft(0x141ccab90, "content_141ccab90", tpl_39107::<0x14329ee28, 0x14329ee08, 0x14329edf8, 0x1409fa280> as *const ()), // content::AnimationSystemComponent::ClassTypeInfoImplementation::content::AnimationSystemComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ccb1e0, "content_141ccb1e0", tpl_38683::<0x14329eda8, 0x18, 0x9, 0x14028a830> as *const ()), // content::AnimationSourceComponent::content::AnimationSourceComponent::vfunc_22
    Replacement::draft_throws(0x141ccb2b0, "content_141ccb2b0", tpl_838::<0x140957b80> as *const (), Throws::Discard), // content::AnimationSourceComponent::content::AnimationSourceComponent::vfunc_32
    Replacement::draft(0x141ccb2c0, "content_141ccb2c0", fn_141ccb2c0 as *const ()), // content::AnimationSourceComponent::content::AnimationSourceComponent::vfunc_33
    Replacement::draft(0x141ccb380, "content_141ccb380", fn_141ccb380 as *const ()), // content::AnimationSourceComponent::ClassTypeInfoImplementation::content::AnimationSourceComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ccb650, "content_141ccb650", fn_141ccb650 as *const ()), // content::AnimationSourceComponent::ClassTypeInfoImplementation::content::AnimationSourceComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ccb860, "content_141ccb860", fn_141ccb860 as *const ()), // content::AnimationSourceComponent::ClassTypeInfoImplementation::content::AnimationSourceComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ccb8d0, "content_141ccb8d0", tpl_38738::<0x14328dc30, 0x8, 0x10, 0x18> as *const ()), // content::AnimationSourceComponent::ClassTypeInfoImplementation::content::AnimationSourceComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141ccbaf0, "content_141ccbaf0", fn_141ccbaf0 as *const ()), // content::AnimationSlaveComponent::content::AnimationSlaveComponent::vfunc_33
    Replacement::draft(0x141ccbe20, "content_141ccbe20", fn_141ccbe20 as *const ()), // content::AnimationSlaveComponent::ClassTypeInfoImplementation::content::AnimationSlaveComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141ccc020, "content_141ccc020", fn_141ccc020 as *const ()), // content::AnimationSlaveComponent::ClassTypeInfoImplementation::content::AnimationSlaveComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cccb20, "content_141cccb20", tpl_39042::<0x14329ef20, 0x8, 0x14329ef38, 0x14329ef08, 0x1409fa050> as *const ()), // content::AnimationGraphComponent::content::AnimationGraphComponent::vfunc_22
    Replacement::draft(0x141cccc70, "content_141cccc70", tpl_39043::<0x14329ef38, 0x14329ef20, 0x14329ef08, 0x1409fa050> as *const ()), // content::AnimationGraphComponent::ClassTypeInfoImplementation::content::AnimationGraphComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141ccd9b0, "content_141ccd9b0", tpl_38546::<0x14329ef28, 0x8, 0x14329f168, 0x14329f188, 0x1409f9d00> as *const ()), // content::ActionSystemComponent::content::ActionSystemComponent::vfunc_22
    Replacement::draft(0x141ccdb10, "content_141ccdb10", tpl_37078::<0x14329f168, 0x14329ef28, 0x14329f188, 0x1409f9d00> as *const ()), // content::ActionSystemComponent::ClassTypeInfoImplementation::content::ActionSystemComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141cce6d0, "content_141cce6d0", tpl_38903::<0x8, 0x1409f9a40, 0x18> as *const ()), // content::AcousticsProbeGroupComponent::ClassTypeInfoImplementation::content::AcousticsProbeGroupComponent::ClassTypeInfo
    Replacement::draft(0x141cce8b0, "content_141cce8b0", fn_141cce8b0 as *const ()), // content::AcousticsProbeComponent::content::AcousticsProbeComponent::vfunc_23
    Replacement::draft(0x141ccea40, "content_141ccea40", fn_141ccea40 as *const ()), // content::AcousticsProbeComponent::ClassTypeInfoImplementation::content::AcousticsProbeComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cceec0, "content_141cceec0", tpl_38695::<0x8> as *const ()), // content::AcousticsProbeComponent::ClassTypeInfoImplementation::content::AcousticsProbeComponent::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141ccfa40, "content_141ccfa40", tpl_838::<0x1409bd6e0> as *const (), Throws::Discard), // content::SpotLightComponent::content::SpotLightComponent::vfunc_31
    Replacement::draft(0x141ccfb50, "content_141ccfb50", tpl_38803::<0x8, 0x1401e84a0, 0x20> as *const ()), // content::PlayerCameraComponent::content::PlayerCameraComponent::vfunc_33
    Replacement::draft(0x141ccfdf0, "content_141ccfdf0", fn_141ccfdf0 as *const ()), // content::CharacterControllerComponent::content::CharacterControllerComponent::vfunc_33
    Replacement::draft(0x141cd01e0, "content_141cd01e0", tpl_38929::<0x8, 0x10> as *const ()), // content::BoxShapeComponent::content::BoxShapeComponent::vfunc_33
    Replacement::draft_throws(0x141cd0230, "content_141cd0230", tpl_838::<0x1409650a0> as *const (), Throws::Discard), // content::BoxShapeComponent::content::BoxShapeComponent::vfunc_30
    Replacement::draft(0x141cd0330, "content_141cd0330", fn_141cd0330 as *const ()), // content::CameraComponent::content::CameraComponent::vfunc_33
    Replacement::draft(0x141cd09c0, "content_141cd09c0", fn_141cd09c0 as *const ()), // content::MovementModels::ClassTypeInfoImplementation::content::MovementModels::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cd0a30, "content_141cd0a30", fn_141cd0a30 as *const ()), // content::MovementModels::ClassTypeInfoImplementation::content::MovementModels::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141cd0aa0, "content_141cd0aa0", fn_141cd0aa0 as *const ()), // content::MovementModels::ClassTypeInfoImplementation::content::MovementModels::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141cd0b20, "content_141cd0b20", tpl_39228::<0x20, 0x1401fb9f0, 0x10, 0x1401de830, 0x142442de0> as *const ()), // content::MovementModels::ClassTypeInfoImplementation::content::MovementModels::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141cd0fc0, "content_141cd0fc0", tpl_38776::<0x14387e5b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::MovementModel::ClassTypeInfoImplementation::content::MovementModel::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141cd1020, "content_141cd1020", tpl_38776::<0x14387e5b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::MovementModel::ClassTypeInfoImplementation::content::MovementModel::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141cd10d0, "content_141cd10d0", tpl_39229::<0x20, 0x28> as *const ()), // content::MovementModel::ClassTypeInfoImplementation::content::MovementModel::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cd1120, "content_141cd1120", tpl_39230::<0x28> as *const ()), // content::MovementModel::ClassTypeInfoImplementation::content::MovementModel::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141cd1220, "content_141cd1220", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelAnimatedMovement::content::MovementModelAnimatedMovement::vfunc_1
    Replacement::draft(0x141cd1270, "content_141cd1270", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelAnimatedMovement::content::MovementModelAnimatedMovement::vfunc_2
    Replacement::draft(0x141cd12c0, "content_141cd12c0", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelAnimatedMovement::content::MovementModelAnimatedMovement::vfunc_3
    Replacement::draft(0x141cd13f0, "content_141cd13f0", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelAnimatedMovement::ClassTypeInfoImplementation::content::MovementModelAnimatedMovement::ClassTypeIn
    Replacement::draft(0x141cd1460, "content_141cd1460", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelAnimatedMovement::ClassTypeInfoImplementation::content::MovementModelAnimatedMovement::ClassTypeIn
    Replacement::draft(0x141cd14d0, "content_141cd14d0", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelAnimatedMovement::ClassTypeInfoImplementation::content::MovementModelAnimatedMovement::ClassTypeIn
    Replacement::draft(0x141cd1710, "content_141cd1710", tpl_239153 as *const ()), // sub_141cd1710
    Replacement::draft(0x141cd1910, "content_141cd1910", fn_141cd1910 as *const ()), // content::MovementModelAnimatedMovement::ClassTypeInfoImplementation::content::MovementModelAnimatedMovement::ClassTypeIn
    Replacement::draft(0x141cd19a0, "content_141cd19a0", fn_141cd19a0 as *const ()), // content::MovementModelAnimatedMovement::ClassTypeInfoImplementation::content::MovementModelAnimatedMovement::ClassTypeIn
    Replacement::draft(0x141cd1b20, "content_141cd1b20", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelTargetedMagnet::content::MovementModelTargetedMagnet::vfunc_1
    Replacement::draft(0x141cd1b70, "content_141cd1b70", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelTargetedMagnet::content::MovementModelTargetedMagnet::vfunc_2
    Replacement::draft(0x141cd1bc0, "content_141cd1bc0", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelTargetedMagnet::content::MovementModelTargetedMagnet::vfunc_3
    Replacement::draft(0x141cd1cf0, "content_141cd1cf0", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelTargetedMagnet::ClassTypeInfoImplementation::content::MovementModelTargetedMagnet::ClassTypeInfoIm
    Replacement::draft(0x141cd1d60, "content_141cd1d60", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelTargetedMagnet::ClassTypeInfoImplementation::content::MovementModelTargetedMagnet::ClassTypeInfoIm
    Replacement::draft(0x141cd1dd0, "content_141cd1dd0", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelTargetedMagnet::ClassTypeInfoImplementation::content::MovementModelTargetedMagnet::ClassTypeInfoIm
    Replacement::draft(0x141cd2210, "content_141cd2210", fn_141cd2210 as *const ()), // content::MovementModelTargetedMagnet::ClassTypeInfoImplementation::content::MovementModelTargetedMagnet::ClassTypeInfoIm
    Replacement::draft(0x141cd22a0, "content_141cd22a0", tpl_39236::<0x8, 0x10, 0x20, 0x28, 0x30, 0x1432a7df8> as *const ()), // content::MovementModelTargetedMagnet::ClassTypeInfoImplementation::content::MovementModelTargetedMagnet::ClassTypeInfoIm
    Replacement::draft(0x141cd2410, "content_141cd2410", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelSpeedLimiter::content::MovementModelSpeedLimiter::vfunc_1
    Replacement::draft(0x141cd2470, "content_141cd2470", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelSpeedLimiter::content::MovementModelSpeedLimiter::vfunc_3
    Replacement::draft(0x141cd25a0, "content_141cd25a0", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelSpeedLimiter::ClassTypeInfoImplementation::content::MovementModelSpeedLimiter::ClassTypeInfoImplem
    Replacement::draft(0x141cd2650, "content_141cd2650", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelSpeedLimiter::ClassTypeInfoImplementation::content::MovementModelSpeedLimiter::ClassTypeInfoImplem
    Replacement::draft(0x141cd2a90, "content_141cd2a90", fn_141cd2a90 as *const ()), // content::MovementModelSpeedLimiter::ClassTypeInfoImplementation::content::MovementModelSpeedLimiter::ClassTypeInfoImplem
    Replacement::draft(0x141cd2b20, "content_141cd2b20", tpl_39236::<0x8, 0x10, 0x20, 0x28, 0x30, 0x1432a7eb0> as *const ()), // content::MovementModelSpeedLimiter::ClassTypeInfoImplementation::content::MovementModelSpeedLimiter::ClassTypeInfoImplem
    Replacement::draft(0x141cd2c90, "content_141cd2c90", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelImpulseMagnet::content::MovementModelImpulseMagnet::vfunc_1
    Replacement::draft(0x141cd2cf0, "content_141cd2cf0", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelImpulseMagnet::content::MovementModelImpulseMagnet::vfunc_3
    Replacement::draft(0x141cd2e20, "content_141cd2e20", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelImpulseMagnet::ClassTypeInfoImplementation::content::MovementModelImpulseMagnet::ClassTypeInfoImpl
    Replacement::draft(0x141cd2ed0, "content_141cd2ed0", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelImpulseMagnet::ClassTypeInfoImplementation::content::MovementModelImpulseMagnet::ClassTypeInfoImpl
    Replacement::draft(0x141cd3310, "content_141cd3310", fn_141cd3310 as *const ()), // content::MovementModelImpulseMagnet::ClassTypeInfoImplementation::content::MovementModelImpulseMagnet::ClassTypeInfoImpl
    Replacement::draft(0x141cd33b0, "content_141cd33b0", fn_141cd33b0 as *const ()), // content::MovementModelImpulseMagnet::ClassTypeInfoImplementation::content::MovementModelImpulseMagnet::ClassTypeInfoImpl
    Replacement::draft(0x141cd3520, "content_141cd3520", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelStandard::content::MovementModelStandard::vfunc_1
    Replacement::draft(0x141cd3580, "content_141cd3580", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelStandard::content::MovementModelStandard::vfunc_3
    Replacement::draft(0x141cd36b0, "content_141cd36b0", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelStandard::ClassTypeInfoImplementation::content::MovementModelStandard::ClassTypeInfoImplementation
    Replacement::draft(0x141cd3760, "content_141cd3760", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelStandard::ClassTypeInfoImplementation::content::MovementModelStandard::ClassTypeInfoImplementation
    Replacement::draft(0x141cd3ba0, "content_141cd3ba0", fn_141cd3ba0 as *const ()), // content::MovementModelStandard::ClassTypeInfoImplementation::content::MovementModelStandard::ClassTypeInfoImplementation
    Replacement::draft(0x141cd3c40, "content_141cd3c40", fn_141cd3c40 as *const ()), // content::MovementModelStandard::ClassTypeInfoImplementation::content::MovementModelStandard::ClassTypeInfoImplementation
    Replacement::draft(0x141cd3df0, "content_141cd3df0", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelForwardBias::content::MovementModelForwardBias::vfunc_1
    Replacement::draft(0x141cd3e50, "content_141cd3e50", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelForwardBias::content::MovementModelForwardBias::vfunc_3
    Replacement::draft(0x141cd3f80, "content_141cd3f80", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelForwardBias::ClassTypeInfoImplementation::content::MovementModelForwardBias::ClassTypeInfoImplemen
    Replacement::draft(0x141cd4030, "content_141cd4030", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelForwardBias::ClassTypeInfoImplementation::content::MovementModelForwardBias::ClassTypeInfoImplemen
    Replacement::draft(0x141cd4470, "content_141cd4470", fn_141cd4470 as *const ()), // content::MovementModelForwardBias::ClassTypeInfoImplementation::content::MovementModelForwardBias::ClassTypeInfoImplemen
    Replacement::draft(0x141cd4520, "content_141cd4520", fn_141cd4520 as *const ()), // content::MovementModelForwardBias::ClassTypeInfoImplementation::content::MovementModelForwardBias::ClassTypeInfoImplemen
    Replacement::draft(0x141cd46d0, "content_141cd46d0", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelFalling::content::MovementModelFalling::vfunc_1
    Replacement::draft(0x141cd4730, "content_141cd4730", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelFalling::content::MovementModelFalling::vfunc_3
    Replacement::draft(0x141cd4860, "content_141cd4860", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelFalling::ClassTypeInfoImplementation::content::MovementModelFalling::ClassTypeInfoImplementation::
    Replacement::draft(0x141cd4910, "content_141cd4910", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelFalling::ClassTypeInfoImplementation::content::MovementModelFalling::ClassTypeInfoImplementation::
    Replacement::draft(0x141cd4d50, "content_141cd4d50", fn_141cd4d50 as *const ()), // content::MovementModelFalling::ClassTypeInfoImplementation::content::MovementModelFalling::ClassTypeInfoImplementation::
    Replacement::draft(0x141cd4e00, "content_141cd4e00", fn_141cd4e00 as *const ()), // content::MovementModelFalling::ClassTypeInfoImplementation::content::MovementModelFalling::ClassTypeInfoImplementation::
    Replacement::draft(0x141cd4f70, "content_141cd4f70", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelPropertiesBase::content::MovementModelPropertiesBase::vfunc_1
    Replacement::draft(0x141cd4fc0, "content_141cd4fc0", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelPropertiesBase::content::MovementModelPropertiesBase::vfunc_2
    Replacement::draft(0x141cd5010, "content_141cd5010", tpl_39231::<0x1432a8c88, 0x30, 0x14384e890, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelPropertiesBase::content::MovementModelPropertiesBase::vfunc_3
    Replacement::draft(0x141cd5140, "content_141cd5140", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelPropertiesBase::ClassTypeInfoImplementation::content::MovementModelPropertiesBase::ClassTypeInfoIm
    Replacement::draft(0x141cd51b0, "content_141cd51b0", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelPropertiesBase::ClassTypeInfoImplementation::content::MovementModelPropertiesBase::ClassTypeInfoIm
    Replacement::draft(0x141cd5220, "content_141cd5220", tpl_39232::<0x14384e890, 0x1432a8c88, 0x1432a8c70, 0x1401de410> as *const ()), // content::MovementModelPropertiesBase::ClassTypeInfoImplementation::content::MovementModelPropertiesBase::ClassTypeInfoIm
    Replacement::draft(0x141cd5660, "content_141cd5660", fn_141cd5660 as *const ()), // content::MovementModelPropertiesBase::ClassTypeInfoImplementation::content::MovementModelPropertiesBase::ClassTypeInfoIm
    Replacement::draft(0x141cd5820, "content_141cd5820", tpl_38546::<0x143251120, 0x8, 0x1432a90cc, 0x1432a90d8, 0x14088ba10> as *const ()), // content::GfxGraphParameterValue<r::GlobalIDRef<content::Curve>>::content::GfxGraphParameterValue<r::GlobalIDRef<content:
    Replacement::draft(0x141cd5980, "content_141cd5980", tpl_37078::<0x1432a90cc, 0x143251120, 0x1432a90d8, 0x14088ba10> as *const ()), // content::GfxGraphParameterValue<r::GlobalIDRef<content::Curve>>::ClassTypeInfoImplementation::content::GfxGraphParameter
    Replacement::draft(0x141cd5fe0, "content_141cd5fe0", tpl_38546::<0x143251120, 0x8, 0x1432a90cc, 0x1432a9108, 0x14088b360> as *const ()), // content::GfxGraphParameterValue<r::ResourceRef<7>>::content::GfxGraphParameterValue<r::ResourceRef<7>>::vfunc_1
    Replacement::draft(0x141cd6140, "content_141cd6140", tpl_37078::<0x1432a90cc, 0x143251120, 0x1432a9108, 0x14088b360> as *const ()), // content::GfxGraphParameterValue<r::ResourceRef<7>>::ClassTypeInfoImplementation::content::GfxGraphParameterValue<r::Reso
    Replacement::draft(0x141cd6790, "content_141cd6790", tpl_38546::<0x143251120, 0x8, 0x1432a90cc, 0x1432a9118, 0x14082b2b0> as *const ()), // content::GfxGraphParameterValue<r::ResourceRef<60>>::content::GfxGraphParameterValue<r::ResourceRef<60>>::vfunc_1
    Replacement::draft(0x141cd68f0, "content_141cd68f0", tpl_37078::<0x1432a90cc, 0x143251120, 0x1432a9118, 0x14082b2b0> as *const ()), // content::GfxGraphParameterValue<r::ResourceRef<60>>::ClassTypeInfoImplementation::content::GfxGraphParameterValue<r::Res
    Replacement::draft(0x141cd7500, "content_141cd7500", fn_141cd7500 as *const ()), // content::GfxGraphParameterValue<g::Color>::ClassTypeInfoImplementation::content::GfxGraphParameterValue<g::Color>::Class
    Replacement::draft_throws(0x141cd7680, "content_141cd7680", tpl_39248::<0x143251120, 0x8, 0x1432a90cc, 0x1432a9380, 0x14086a110> as *const (), Throws::Discard), // content::GfxGraphParameterValue<m::Vector4Template<float>>::content::GfxGraphParameterValue<m::Vector4Template<float>>::
    Replacement::draft_throws(0x141cd7800, "content_141cd7800", tpl_39249::<0x1432a90cc, 0x143251120, 0x1432a9380, 0x14086a110> as *const (), Throws::Discard), // content::GfxGraphParameterValue<m::Vector4Template<float>>::ClassTypeInfoImplementation::content::GfxGraphParameterValue
    Replacement::draft_throws(0x141cd7e30, "content_141cd7e30", tpl_39248::<0x143251120, 0x8, 0x1432a90cc, 0x1432a9390, 0x1402c4e80> as *const (), Throws::Discard), // content::GfxGraphParameterValue<m::Vector3Template<float>>::content::GfxGraphParameterValue<m::Vector3Template<float>>::
    Replacement::draft(0x141cd7f10, "content_141cd7f10", tpl_38929::<0x8, 0x10> as *const ()), // content::GfxGraphParameterValue<m::Vector3Template<float>>::content::GfxGraphParameterValue<m::Vector3Template<float>>::
    Replacement::draft_throws(0x141cd7fc0, "content_141cd7fc0", tpl_39249::<0x1432a90cc, 0x143251120, 0x1432a9390, 0x1402c4e80> as *const (), Throws::Discard), // content::GfxGraphParameterValue<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::GfxGraphParameterValue
    Replacement::draft(0x141cd8260, "content_141cd8260", tpl_38930::<0x8, 0x10> as *const ()), // content::GfxGraphParameterValue<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::GfxGraphParameterValue
    Replacement::draft(0x141cd8430, "content_141cd8430", tpl_38931::<0x8, 0x10> as *const ()), // content::GfxGraphParameterValue<m::Vector3Template<float>>::ClassTypeInfoImplementation::content::GfxGraphParameterValue
    Replacement::draft_throws(0x141cd85f0, "content_141cd85f0", tpl_39248::<0x143251120, 0x8, 0x1432a90cc, 0x1432a93a0, 0x1402c61c0> as *const (), Throws::Discard), // content::GfxGraphParameterValue<m::Vector2Template<float>>::content::GfxGraphParameterValue<m::Vector2Template<float>>::
    Replacement::draft_throws(0x141cd8770, "content_141cd8770", tpl_39249::<0x1432a90cc, 0x143251120, 0x1432a93a0, 0x1402c61c0> as *const (), Throws::Discard), // content::GfxGraphParameterValue<m::Vector2Template<float>>::ClassTypeInfoImplementation::content::GfxGraphParameterValue
    Replacement::draft(0x141cd8d90, "content_141cd8d90", tpl_38546::<0x143251120, 0x8, 0x1432a90cc, 0x1432a93b0, 0x14020f270> as *const ()), // content::GfxGraphParameterValue<float>::content::GfxGraphParameterValue<float>::vfunc_2
    Replacement::draft_throws(0x141cd8df0, "content_141cd8df0", tpl_838::<0x140aa7900> as *const (), Throws::Discard), // content::GfxGraphParameterValue<float>::content::GfxGraphParameterValue<float>::vfunc_4
    Replacement::draft(0x141cd8f10, "content_141cd8f10", tpl_37078::<0x1432a90cc, 0x143251120, 0x1432a93b0, 0x14020f270> as *const ()), // content::GfxGraphParameterValue<float>::ClassTypeInfoImplementation::content::GfxGraphParameterValue<float>::ClassTypeIn
    Replacement::draft(0x141cd9380, "content_141cd9380", tpl_38695::<0x8> as *const ()), // content::GfxGraphParameterValue<float>::ClassTypeInfoImplementation::content::GfxGraphParameterValue<float>::ClassTypeIn
    Replacement::draft(0x141cda130, "content_141cda130", tpl_35199::<0x14387e648, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphParameterValueBase::ClassTypeInfoImplementation::content::GfxGraphParameterValueBase::ClassTypeInfoImpl
    Replacement::draft(0x141cda220, "content_141cda220", tpl_35199::<0x14387e648, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphParameterValueBase::ClassTypeInfoImplementation::content::GfxGraphParameterValueBase::ClassTypeInfoImpl
    Replacement::draft(0x141cda400, "content_141cda400", tpl_35199::<0x14387e648, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphParameterValueBase::ClassTypeInfoImplementation::content::GfxGraphParameterValueBase::ClassTypeInfoImpl
    Replacement::draft(0x141cda4f0, "content_141cda4f0", tpl_35199::<0x14387e648, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphParameterValueBase::ClassTypeInfoImplementation::content::GfxGraphParameterValueBase::ClassTypeInfoImpl
    Replacement::draft(0x141cda570, "content_141cda570", tpl_35199::<0x14387e648, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GfxGraphParameterValueBase::ClassTypeInfoImplementation::content::GfxGraphParameterValueBase::ClassTypeInfoImpl
    Replacement::draft(0x141cda820, "content_141cda820", tpl_39250::<0x143874430, 0x143264170, 0x1432a7320, 0x1432a72f0, 0x1401de410> as *const ()), // content::GfxGraphParameter::ClassTypeInfoImplementation::content::GfxGraphParameter::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cda880, "content_141cda880", tpl_39250::<0x143874430, 0x143264170, 0x1432a7320, 0x1432a72f0, 0x1401de410> as *const ()), // content::GfxGraphParameter::ClassTypeInfoImplementation::content::GfxGraphParameter::ClassTypeInfoImplementation::vfunc_
    Replacement::draft_throws(0x141cdacd0, "content_141cdacd0", fn_141cdacd0 as *const (), Throws::Discard), // content::GfxGraphParameter::ClassTypeInfoImplementation::content::GfxGraphParameter::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdad30, "content_141cdad30", fn_141cdad30 as *const ()), // content::GfxGraphParameter::ClassTypeInfoImplementation::content::GfxGraphParameter::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdad80, "content_141cdad80", fn_141cdad80 as *const ()), // content::GfxGraphParameter::ClassTypeInfoImplementation::content::GfxGraphParameter::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdadb0, "content_141cdadb0", fn_141cdadb0 as *const ()), // content::GfxGraphParameter::ClassTypeInfoImplementation::content::GfxGraphParameter::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdae50, "content_141cdae50", fn_141cdae50 as *const ()), // content::GfxGraphParameter::ClassTypeInfoImplementation::content::GfxGraphParameter::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdaf90, "content_141cdaf90", tpl_38875::<0x8, 0x140853830> as *const ()), // content::ActionSystemActorCollectorComponent::content::ActionSystemActorCollectorComponent::vfunc_24
    Replacement::draft(0x141cdb760, "content_141cdb760", fn_141cdb760 as *const ()), // content::ActionSystemActorResource::content::ActionSystemActorResource::vfunc_25
    Replacement::draft(0x141cdbc80, "content_141cdbc80", fn_141cdbc80 as *const ()), // content::ActionSystemActorResource::ClassTypeInfoImplementation::content::ActionSystemActorResource::ClassTypeInfoImplem
    Replacement::draft(0x141cdbce0, "content_141cdbce0", fn_141cdbce0 as *const ()), // content::ActionSystemActorResource::ClassTypeInfoImplementation::content::ActionSystemActorResource::ClassTypeInfoImplem
    Replacement::draft(0x141cdbd70, "content_141cdbd70", fn_141cdbd70 as *const ()), // content::ActionSystemActorResource::ClassTypeInfoImplementation::content::ActionSystemActorResource::ClassTypeInfoImplem
    Replacement::draft(0x141cdc350, "content_141cdc350", tpl_35199::<0x14387e6d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActorMetadata::ClassTypeInfoImplementation::content::ActionSystemActorMetadata::ClassTypeInfoImplem
    Replacement::draft(0x141cdc440, "content_141cdc440", tpl_35199::<0x14387e6d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActorMetadata::ClassTypeInfoImplementation::content::ActionSystemActorMetadata::ClassTypeInfoImplem
    Replacement::draft(0x141cdc620, "content_141cdc620", tpl_35199::<0x14387e6d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActorMetadata::ClassTypeInfoImplementation::content::ActionSystemActorMetadata::ClassTypeInfoImplem
    Replacement::draft(0x141cdc710, "content_141cdc710", tpl_35199::<0x14387e6d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActorMetadata::ClassTypeInfoImplementation::content::ActionSystemActorMetadata::ClassTypeInfoImplem
    Replacement::draft(0x141cdc790, "content_141cdc790", tpl_35199::<0x14387e6d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActorMetadata::ClassTypeInfoImplementation::content::ActionSystemActorMetadata::ClassTypeInfoImplem
    Replacement::draft(0x141cdcf80, "content_141cdcf80", fn_141cdcf80 as *const ()), // content::ActionSystemActionTableResource::ClassTypeInfoImplementation::content::ActionSystemActionTableResource::ClassTy
    Replacement::draft(0x141cdd060, "content_141cdd060", tpl_39261::<0x140aade80, 0x142442de0> as *const ()), // content::ActionSystemActionTableResource::ClassTypeInfoImplementation::content::ActionSystemActionTableResource::ClassTy
    Replacement::draft(0x141cdd610, "content_141cdd610", tpl_35199::<0x14387e710, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActionTableMetadata::ClassTypeInfoImplementation::content::ActionSystemActionTableMetadata::ClassTy
    Replacement::draft(0x141cdd700, "content_141cdd700", tpl_35199::<0x14387e710, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActionTableMetadata::ClassTypeInfoImplementation::content::ActionSystemActionTableMetadata::ClassTy
    Replacement::draft(0x141cdd8e0, "content_141cdd8e0", tpl_35199::<0x14387e710, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActionTableMetadata::ClassTypeInfoImplementation::content::ActionSystemActionTableMetadata::ClassTy
    Replacement::draft(0x141cdd9d0, "content_141cdd9d0", tpl_35199::<0x14387e710, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActionTableMetadata::ClassTypeInfoImplementation::content::ActionSystemActionTableMetadata::ClassTy
    Replacement::draft(0x141cdda50, "content_141cdda50", tpl_35199::<0x14387e710, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActionTableMetadata::ClassTypeInfoImplementation::content::ActionSystemActionTableMetadata::ClassTy
    Replacement::draft(0x141cddcc0, "content_141cddcc0", tpl_39262::<0x8> as *const ()), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft(0x141cddce0, "content_141cddce0", tpl_39262::<0x8> as *const ()), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft(0x141cddd20, "content_141cddd20", tpl_39262::<0x18> as *const ()), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft(0x141cddd40, "content_141cddd40", tpl_39262::<0x18> as *const ()), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft_throws(0x141cddfa0, "content_141cddfa0", tpl_39263::<0x14387e6fc, 0x14387e6f0, 0x1401e9b60, 0x140ab9890, 0x1401e9a20> as *const (), Throws::Discard), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft_throws(0x141cde000, "content_141cde000", tpl_39263::<0x14387e6fc, 0x14387e6f0, 0x1401e9b60, 0x140ab9890, 0x1401e9a20> as *const (), Throws::Discard), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft_throws(0x141cde0a0, "content_141cde0a0", tpl_39263::<0x14387e6fc, 0x14387e6f0, 0x1401e9fa0, 0x140ab9370, 0x1401e9e70> as *const (), Throws::Discard), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft_throws(0x141cde100, "content_141cde100", tpl_39263::<0x14387e6fc, 0x14387e6f0, 0x1401e9fa0, 0x140ab9370, 0x1401e9e70> as *const (), Throws::Discard), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft(0x141cde250, "content_141cde250", tpl_38485::<0x142442de0> as *const ()), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft(0x141cde2c0, "content_141cde2c0", tpl_38486::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x142442de0, 0x142442c80> as *const ()), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft(0x141cde350, "content_141cde350", tpl_38487::<0x142442de0> as *const ()), // content::ActionSystemMovementModel::ClassTypeInfoImplementation::content::ActionSystemMovementModel::ClassTypeInfoImplem
    Replacement::draft(0x141cde460, "content_141cde460", tpl_35205::<0x1432aae28, 0x1, 0x14020f270> as *const ()), // content::ActionSystemActorEvent::ClassTypeInfoImplementation::content::ActionSystemActorEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141cde4a0, "content_141cde4a0", tpl_35205::<0x1432aae28, 0x1, 0x14020f270> as *const ()), // content::ActionSystemActorEvent::ClassTypeInfoImplementation::content::ActionSystemActorEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141cde700, "content_141cde700", tpl_39004::<0x8, 0x14240aa00, 0x30> as *const ()), // content::ActionSystemActorEvent::ClassTypeInfoImplementation::content::ActionSystemActorEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141cde750, "content_141cde750", tpl_39004::<0x8, 0x14240aa00, 0x30> as *const ()), // content::ActionSystemActorEvent::ClassTypeInfoImplementation::content::ActionSystemActorEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141cde8a0, "content_141cde8a0", tpl_35199::<0x14387e6d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActorEvent::ClassTypeInfoImplementation::content::ActionSystemActorEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141cde900, "content_141cde900", tpl_35199::<0x14387e6d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionSystemActorEvent::ClassTypeInfoImplementation::content::ActionSystemActorEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141cde9b0, "content_141cde9b0", tpl_39005::<0x14240aa00, 0x30> as *const ()), // content::ActionSystemActorEvent::ClassTypeInfoImplementation::content::ActionSystemActorEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141cdea00, "content_141cdea00", fn_141cdea00 as *const ()), // content::ActionSystemActorEvent::ClassTypeInfoImplementation::content::ActionSystemActorEvent::ClassTypeInfoImplementati
    Replacement::draft(0x141cdef90, "content_141cdef90", fn_141cdef90 as *const ()), // content::ActionSystemActor::ClassTypeInfoImplementation::content::ActionSystemActor::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdf000, "content_141cdf000", tpl_39266::<0x60, 0x140ab7fe0, 0x30, 0x14240ab10> as *const ()), // content::ActionSystemActor::ClassTypeInfoImplementation::content::ActionSystemActor::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdf030, "content_141cdf030", fn_141cdf030 as *const ()), // content::ActionSystemActor::ClassTypeInfoImplementation::content::ActionSystemActor::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdf070, "content_141cdf070", tpl_39268::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x60, 0x140ab7fe0, 0x30, 0x14240ab10, 0x142442c80> as *const ()), // content::ActionSystemActor::ClassTypeInfoImplementation::content::ActionSystemActor::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cdf0e0, "content_141cdf0e0", tpl_39228::<0x60, 0x140ab7fe0, 0x30, 0x14240ab10, 0x142442de0> as *const ()), // content::ActionSystemActor::ClassTypeInfoImplementation::content::ActionSystemActor::ClassTypeInfoImplementation::vfunc_
    Replacement::draft_throws(0x141cdf5f0, "content_141cdf5f0", fn_141cdf5f0 as *const (), Throws::Discard), // content::ActionSystemActionTable::ClassTypeInfoImplementation::content::ActionSystemActionTable::ClassTypeInfoImplementa
    Replacement::draft(0x141cdf730, "content_141cdf730", fn_141cdf730 as *const ()), // content::ActionSystemActionTable::ClassTypeInfoImplementation::content::ActionSystemActionTable::ClassTypeInfoImplementa
    Replacement::draft(0x141cdf790, "content_141cdf790", fn_141cdf790 as *const ()), // content::ActionSystemActionTable::ClassTypeInfoImplementation::content::ActionSystemActionTable::ClassTypeInfoImplementa
    Replacement::draft(0x141cdf830, "content_141cdf830", fn_141cdf830 as *const ()), // content::ActionSystemActionTable::ClassTypeInfoImplementation::content::ActionSystemActionTable::ClassTypeInfoImplementa
    Replacement::draft(0x141cdf930, "content_141cdf930", tpl_35220::<0x1432aae10, 0x1, 0x140ab8e70> as *const ()), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft(0x141cdf970, "content_141cdf970", tpl_35220::<0x1432aae10, 0x1, 0x140ab8e70> as *const ()), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft(0x141cdf9d0, "content_141cdf9d0", tpl_35220::<0x1432aae10, 0x1, 0x140ab8e50> as *const ()), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft(0x141cdfa10, "content_141cdfa10", tpl_35220::<0x1432aae10, 0x1, 0x140ab8e50> as *const ()), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft_throws(0x141cdfc50, "content_141cdfc50", tpl_39273::<0x1408b1160, 0x8, 0x140abf240> as *const (), Throws::Discard), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft_throws(0x141cdfc90, "content_141cdfc90", tpl_39273::<0x1408b1160, 0x8, 0x140abf240> as *const (), Throws::Discard), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft(0x141cdfe80, "content_141cdfe80", fn_141cdfe80 as *const ()), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft(0x141cdfed0, "content_141cdfed0", tpl_39275::<0x8, 0x1401fc200, 0x10, 0x18> as *const ()), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft(0x141cdff20, "content_141cdff20", fn_141cdff20 as *const ()), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft(0x141cdffc0, "content_141cdffc0", fn_141cdffc0 as *const ()), // content::ActionSystemActionExported::ClassTypeInfoImplementation::content::ActionSystemActionExported::ClassTypeInfoImpl
    Replacement::draft(0x141ce00c0, "content_141ce00c0", tpl_35220::<0x1432aae10, 0x1, 0x140ab8e70> as *const ()), // content::ActionSystemAction::ClassTypeInfoImplementation::content::ActionSystemAction::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce0100, "content_141ce0100", tpl_35220::<0x1432aae10, 0x1, 0x140ab8e70> as *const ()), // content::ActionSystemAction::ClassTypeInfoImplementation::content::ActionSystemAction::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce0160, "content_141ce0160", tpl_35220::<0x1432aae10, 0x1, 0x140ab8e50> as *const ()), // content::ActionSystemAction::ClassTypeInfoImplementation::content::ActionSystemAction::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce01a0, "content_141ce01a0", tpl_35220::<0x1432aae10, 0x1, 0x140ab8e50> as *const ()), // content::ActionSystemAction::ClassTypeInfoImplementation::content::ActionSystemAction::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141ce03e0, "content_141ce03e0", tpl_39273::<0x1408b1160, 0x8, 0x140abc6c0> as *const (), Throws::Discard), // content::ActionSystemAction::ClassTypeInfoImplementation::content::ActionSystemAction::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141ce0420, "content_141ce0420", tpl_39273::<0x1408b1160, 0x8, 0x140abc6c0> as *const (), Throws::Discard), // content::ActionSystemAction::ClassTypeInfoImplementation::content::ActionSystemAction::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce0620, "content_141ce0620", tpl_39275::<0x8, 0x140abc3e0, 0x10, 0x18> as *const ()), // content::ActionSystemAction::ClassTypeInfoImplementation::content::ActionSystemAction::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce0670, "content_141ce0670", fn_141ce0670 as *const ()), // content::ActionSystemAction::ClassTypeInfoImplementation::content::ActionSystemAction::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce07f0, "content_141ce07f0", tpl_39279::<0x14384e8d8, 0x1432aae20, 0x1432aade0, 0x1401de410> as *const ()), // content::ActionRule::ClassTypeInfoImplementation::content::ActionRule::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141ce0840, "content_141ce0840", tpl_39279::<0x14384e8d8, 0x1432aae20, 0x1432aade0, 0x1401de410> as *const ()), // content::ActionRule::ClassTypeInfoImplementation::content::ActionRule::ClassTypeInfoImplementation::vfunc_36
    Replacement::draft(0x141ce0890, "content_141ce0890", tpl_39279::<0x14384e8d8, 0x1432aae20, 0x1432aade0, 0x1401de410> as *const ()), // content::ActionRule::ClassTypeInfoImplementation::content::ActionRule::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141ce08e0, "content_141ce08e0", tpl_39279::<0x14384e8d8, 0x1432aae20, 0x1432aade0, 0x1401de410> as *const ()), // content::ActionRule::ClassTypeInfoImplementation::content::ActionRule::ClassTypeInfoImplementation::vfunc_37
    Replacement::draft(0x141ce0930, "content_141ce0930", tpl_39279::<0x14384e8d8, 0x1432aae20, 0x1432aade0, 0x1401de410> as *const ()), // content::ActionRule::ClassTypeInfoImplementation::content::ActionRule::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141ce0980, "content_141ce0980", tpl_39279::<0x14384e8d8, 0x1432aae20, 0x1432aade0, 0x1401de410> as *const ()), // content::ActionRule::ClassTypeInfoImplementation::content::ActionRule::ClassTypeInfoImplementation::vfunc_38
    Replacement::draft(0x141ce0d10, "content_141ce0d10", tpl_35199::<0x14387e698, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionRule::ClassTypeInfoImplementation::content::ActionRule::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce0d70, "content_141ce0d70", tpl_35199::<0x14387e698, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ActionRule::ClassTypeInfoImplementation::content::ActionRule::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141ce11a0, "content_141ce11a0", tpl_38951::<0x8, 0x14240aa00, 0x30> as *const ()), // content::PhysicsMaterialGuidName::ClassTypeInfoImplementation::content::PhysicsMaterialGuidName::ClassTypeInfoImplementa
    Replacement::draft(0x141ce11f0, "content_141ce11f0", tpl_38951::<0x8, 0x14240aa00, 0x30> as *const ()), // content::PhysicsMaterialGuidName::ClassTypeInfoImplementation::content::PhysicsMaterialGuidName::ClassTypeInfoImplementa
    Replacement::draft(0x141ce1340, "content_141ce1340", tpl_35199::<0x14387e828, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::PhysicsMaterialGuidName::ClassTypeInfoImplementation::content::PhysicsMaterialGuidName::ClassTypeInfoImplementa
    Replacement::draft(0x141ce13a0, "content_141ce13a0", tpl_35199::<0x14387e828, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::PhysicsMaterialGuidName::ClassTypeInfoImplementation::content::PhysicsMaterialGuidName::ClassTypeInfoImplementa
    Replacement::draft(0x141ce1450, "content_141ce1450", tpl_38937::<0x14240aa00, 0x30> as *const ()), // content::PhysicsMaterialGuidName::ClassTypeInfoImplementation::content::PhysicsMaterialGuidName::ClassTypeInfoImplementa
    Replacement::draft(0x141ce14a0, "content_141ce14a0", tpl_38952::<0x30, 0x14240ab10> as *const ()), // content::PhysicsMaterialGuidName::ClassTypeInfoImplementation::content::PhysicsMaterialGuidName::ClassTypeInfoImplementa
    Replacement::draft(0x141ce1510, "content_141ce1510", tpl_98840::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x30, 0x14240ab10, 0x142442c80> as *const ()), // content::PhysicsMaterialGuidName::ClassTypeInfoImplementation::content::PhysicsMaterialGuidName::ClassTypeInfoImplementa
    Replacement::draft(0x141ce1580, "content_141ce1580", tpl_38954::<0x30, 0x14240ab10, 0x142442de0> as *const ()), // content::PhysicsMaterialGuidName::ClassTypeInfoImplementation::content::PhysicsMaterialGuidName::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141ce1700, "content_141ce1700", tpl_39280::<0x14387e8e4, 0x14387e8d8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::ArchetypeReference::ClassTypeInfoImplementation::content::ArchetypeReference::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141ce1760, "content_141ce1760", tpl_39280::<0x14387e8e4, 0x14387e8d8, 0x1401e9770, 0x338, 0x1424b4cb0, 0x1401e9590> as *const (), Throws::Discard), // content::ArchetypeReference::ClassTypeInfoImplementation::content::ArchetypeReference::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce1a80, "content_141ce1a80", tpl_35199::<0x14387e8d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ArchetypeReference::ClassTypeInfoImplementation::content::ArchetypeReference::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce1ae0, "content_141ce1ae0", tpl_35199::<0x14387e8d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ArchetypeReference::ClassTypeInfoImplementation::content::ArchetypeReference::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce1f80, "content_141ce1f80", tpl_39281::<0x8, 0x10> as *const ()), // content::StreamingStateNode::OBBVolume::ClassTypeInfoImplementation::content::StreamingStateNode::OBBVolume::ClassTypeIn
    Replacement::draft(0x141ce1fb0, "content_141ce1fb0", tpl_39281::<0x8, 0x10> as *const ()), // content::StreamingStateNode::OBBVolume::ClassTypeInfoImplementation::content::StreamingStateNode::OBBVolume::ClassTypeIn
    Replacement::draft(0x141ce20e0, "content_141ce20e0", tpl_35199::<0x14387e878, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::StreamingStateNode::OBBVolume::ClassTypeInfoImplementation::content::StreamingStateNode::OBBVolume::ClassTypeIn
    Replacement::draft(0x141ce2140, "content_141ce2140", tpl_35199::<0x14387e878, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::StreamingStateNode::OBBVolume::ClassTypeInfoImplementation::content::StreamingStateNode::OBBVolume::ClassTypeIn
    Replacement::draft(0x141ce21f0, "content_141ce21f0", tpl_38500::<0x20> as *const ()), // content::StreamingStateNode::OBBVolume::ClassTypeInfoImplementation::content::StreamingStateNode::OBBVolume::ClassTypeIn
    Replacement::draft(0x141ce2770, "content_141ce2770", fn_141ce2770 as *const ()), // content::StreamingStateNode::ClassTypeInfoImplementation::content::StreamingStateNode::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce27e0, "content_141ce27e0", fn_141ce27e0 as *const ()), // content::StreamingStateNode::ClassTypeInfoImplementation::content::StreamingStateNode::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce2820, "content_141ce2820", fn_141ce2820 as *const ()), // content::StreamingStateNode::ClassTypeInfoImplementation::content::StreamingStateNode::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce2850, "content_141ce2850", tpl_38841::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x20, 0x14240ab10, 0x10, 0x140afc400, 0x1401de830, 0x142442c80> as *const ()), // content::StreamingStateNode::ClassTypeInfoImplementation::content::StreamingStateNode::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce28d0, "content_141ce28d0", tpl_38842::<0x20, 0x14240ab10, 0x10, 0x140afc400, 0x1401de830, 0x142442de0> as *const ()), // content::StreamingStateNode::ClassTypeInfoImplementation::content::StreamingStateNode::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141ce2c70, "content_141ce2c70", tpl_38845::<0x8, 0x1401e84a0, 0x18> as *const ()), // content::StreamingUnit::ClassTypeInfoImplementation::content::StreamingUnit::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141ce2cc0, "content_141ce2cc0", tpl_38845::<0x8, 0x1401e84a0, 0x18> as *const ()), // content::StreamingUnit::ClassTypeInfoImplementation::content::StreamingUnit::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141ce2e10, "content_141ce2e10", tpl_35199::<0x14387e748, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::StreamingUnit::ClassTypeInfoImplementation::content::StreamingUnit::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce2e70, "content_141ce2e70", tpl_35199::<0x14387e748, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::StreamingUnit::ClassTypeInfoImplementation::content::StreamingUnit::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141ce2f20, "content_141ce2f20", tpl_38846::<0x1401e84a0, 0x18> as *const ()), // content::StreamingUnit::ClassTypeInfoImplementation::content::StreamingUnit::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ce2f60, "content_141ce2f60", tpl_37083::<0x8> as *const ()), // content::StreamingUnit::ClassTypeInfoImplementation::content::StreamingUnit::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141ce2f90, "content_141ce2f90", tpl_37085::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x8, 0x142442c80> as *const ()), // content::StreamingUnit::ClassTypeInfoImplementation::content::StreamingUnit::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141ce2fd0, "content_141ce2fd0", tpl_37086::<0x8, 0x142442de0> as *const ()), // content::StreamingUnit::ClassTypeInfoImplementation::content::StreamingUnit::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141ce3450, "content_141ce3450", tpl_39285::<0x14387e8d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::StreamingState::ClassTypeInfoImplementation::content::StreamingState::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce34b0, "content_141ce34b0", tpl_39285::<0x14387e8d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::StreamingState::ClassTypeInfoImplementation::content::StreamingState::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141ce3560, "content_141ce3560", fn_141ce3560 as *const ()), // content::StreamingState::ClassTypeInfoImplementation::content::StreamingState::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ce35d0, "content_141ce35d0", tpl_38938::<0x10, 0x1401de830> as *const ()), // content::StreamingState::ClassTypeInfoImplementation::content::StreamingState::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141ce3620, "content_141ce3620", tpl_38939::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x10, 0x1401de830, 0x142442c80> as *const ()), // content::StreamingState::ClassTypeInfoImplementation::content::StreamingState::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141ce3690, "content_141ce3690", tpl_38940::<0x10, 0x1401de830, 0x142442de0> as *const ()), // content::StreamingState::ClassTypeInfoImplementation::content::StreamingState::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141ce3770, "content_141ce3770", tpl_39287::<0x1432b0f58, 0xe, 0x1406db220> as *const ()), // content::StaticObject::ClassTypeInfoImplementation::content::StaticObject::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141ce37b0, "content_141ce37b0", tpl_39287::<0x1432b0f58, 0xe, 0x1406db220> as *const ()), // content::StaticObject::ClassTypeInfoImplementation::content::StaticObject::ClassTypeInfoImplementation::vfunc_36
    Replacement::draft(0x141ce3c40, "content_141ce3c40", fn_141ce3c40 as *const ()), // content::StaticObject::ClassTypeInfoImplementation::content::StaticObject::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ce3d30, "content_141ce3d30", fn_141ce3d30 as *const ()), // content::StaticObject::ClassTypeInfoImplementation::content::StaticObject::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141ce4230, "content_141ce4230", tpl_38964::<0x8, 0xc> as *const ()), // content::ScatterParamsComponent::MeshVariation::ClassTypeInfoImplementation::content::ScatterParamsComponent::MeshVariat
    Replacement::draft(0x141ce4270, "content_141ce4270", tpl_38963::<0x8, 0xc> as *const ()), // content::ScatterParamsComponent::MeshVariation::ClassTypeInfoImplementation::content::ScatterParamsComponent::MeshVariat
    Replacement::draft(0x141ce43b0, "content_141ce43b0", tpl_38859::<0x14387e7a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ScatterParamsComponent::MeshVariation::ClassTypeInfoImplementation::content::ScatterParamsComponent::MeshVariat
    Replacement::draft(0x141ce4410, "content_141ce4410", tpl_38859::<0x14387e7a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ScatterParamsComponent::MeshVariation::ClassTypeInfoImplementation::content::ScatterParamsComponent::MeshVariat
    Replacement::draft(0x141ce4f10, "content_141ce4f10", tpl_35212::<0x14387e8f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::MaterialGfxGraph::ClassTypeInfoImplementation::content::MaterialGfxGraph::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce4f70, "content_141ce4f70", tpl_35212::<0x14387e8f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::MaterialGfxGraph::ClassTypeInfoImplementation::content::MaterialGfxGraph::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141ce5000, "content_141ce5000", fn_141ce5000 as *const ()), // content::MaterialGfxGraph::ClassTypeInfoImplementation::content::MaterialGfxGraph::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ce5070, "content_141ce5070", fn_141ce5070 as *const ()), // content::MaterialGfxGraph::ClassTypeInfoImplementation::content::MaterialGfxGraph::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141ce50a0, "content_141ce50a0", tpl_94938::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x14240ab10, 0x142442c80> as *const ()), // content::MaterialGfxGraph::ClassTypeInfoImplementation::content::MaterialGfxGraph::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141ce5100, "content_141ce5100", tpl_35184::<0x10, 0x14240ab10, 0x142442de0> as *const ()), // content::MaterialGfxGraph::ClassTypeInfoImplementation::content::MaterialGfxGraph::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141ce5660, "content_141ce5660", fn_141ce5660 as *const ()), // content::Heightfield::ClassTypeInfoImplementation::content::Heightfield::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ce56f0, "content_141ce56f0", tpl_39293::<0x48, 0x140a02a70> as *const ()), // content::Heightfield::ClassTypeInfoImplementation::content::Heightfield::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141ce5710, "content_141ce5710", fn_141ce5710 as *const ()), // content::Heightfield::ClassTypeInfoImplementation::content::Heightfield::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141ce5750, "content_141ce5750", fn_141ce5750 as *const ()), // content::Heightfield::ClassTypeInfoImplementation::content::Heightfield::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141ce57b0, "content_141ce57b0", tpl_35191::<0x48, 0x140a02a70, 0x142442de0> as *const ()), // content::Heightfield::ClassTypeInfoImplementation::content::Heightfield::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141ce5b20, "content_141ce5b20", fn_141ce5b20 as *const ()), // content::HeightfieldStamp::ClassTypeInfoImplementation::content::HeightfieldStamp::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141ce5b80, "content_141ce5b80", fn_141ce5b80 as *const ()), // content::HeightfieldStamp::ClassTypeInfoImplementation::content::HeightfieldStamp::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141ce5cd0, "content_141ce5cd0", tpl_35219::<0x14387e7b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::HeightfieldStamp::ClassTypeInfoImplementation::content::HeightfieldStamp::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce5d30, "content_141ce5d30", tpl_35219::<0x14387e7b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::HeightfieldStamp::ClassTypeInfoImplementation::content::HeightfieldStamp::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141ce5de0, "content_141ce5de0", tpl_35239 as *const ()), // content::HeightfieldStamp::ClassTypeInfoImplementation::content::HeightfieldStamp::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ce6400, "content_141ce6400", tpl_35199::<0x14387e778, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphPin::ClassTypeInfoImplementation::content::GraphPin::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141ce64f0, "content_141ce64f0", tpl_35199::<0x14387e778, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphPin::ClassTypeInfoImplementation::content::GraphPin::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141ce66d0, "content_141ce66d0", tpl_35199::<0x14387e778, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphPin::ClassTypeInfoImplementation::content::GraphPin::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141ce67c0, "content_141ce67c0", tpl_35199::<0x14387e778, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphPin::ClassTypeInfoImplementation::content::GraphPin::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141ce6840, "content_141ce6840", tpl_35199::<0x14387e778, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphPin::ClassTypeInfoImplementation::content::GraphPin::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce6bf0, "content_141ce6bf0", tpl_35199::<0x14387e8c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphConnection::ClassTypeInfoImplementation::content::GraphConnection::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141ce6ce0, "content_141ce6ce0", tpl_35199::<0x14387e8c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphConnection::ClassTypeInfoImplementation::content::GraphConnection::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141ce6ec0, "content_141ce6ec0", tpl_35199::<0x14387e8c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphConnection::ClassTypeInfoImplementation::content::GraphConnection::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141ce6fb0, "content_141ce6fb0", tpl_35199::<0x14387e8c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphConnection::ClassTypeInfoImplementation::content::GraphConnection::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141ce7030, "content_141ce7030", tpl_35199::<0x14387e8c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphConnection::ClassTypeInfoImplementation::content::GraphConnection::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce73c0, "content_141ce73c0", tpl_35199::<0x14387e740, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphNode::ClassTypeInfoImplementation::content::GraphNode::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141ce74b0, "content_141ce74b0", tpl_35199::<0x14387e740, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphNode::ClassTypeInfoImplementation::content::GraphNode::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141ce7690, "content_141ce7690", tpl_35199::<0x14387e740, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphNode::ClassTypeInfoImplementation::content::GraphNode::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141ce7780, "content_141ce7780", tpl_35199::<0x14387e740, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphNode::ClassTypeInfoImplementation::content::GraphNode::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141ce7800, "content_141ce7800", tpl_35199::<0x14387e740, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GraphNode::ClassTypeInfoImplementation::content::GraphNode::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce7b90, "content_141ce7b90", tpl_35199::<0x14387e768, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::Graph::ClassTypeInfoImplementation::content::Graph::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141ce7c80, "content_141ce7c80", tpl_35199::<0x14387e768, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::Graph::ClassTypeInfoImplementation::content::Graph::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141ce7e60, "content_141ce7e60", tpl_35199::<0x14387e768, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::Graph::ClassTypeInfoImplementation::content::Graph::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141ce7f50, "content_141ce7f50", tpl_35199::<0x14387e768, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::Graph::ClassTypeInfoImplementation::content::Graph::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141ce7fd0, "content_141ce7fd0", tpl_35199::<0x14387e768, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::Graph::ClassTypeInfoImplementation::content::Graph::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft_throws(0x141ce8690, "content_141ce8690", fn_141ce8690 as *const (), Throws::Discard), // content::EntityArchetype::ClassTypeInfoImplementation::content::EntityArchetype::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ce8730, "content_141ce8730", fn_141ce8730 as *const ()), // content::EntityArchetype::ClassTypeInfoImplementation::content::EntityArchetype::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141ce8790, "content_141ce8790", fn_141ce8790 as *const ()), // content::EntityArchetype::ClassTypeInfoImplementation::content::EntityArchetype::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141ce8d90, "content_141ce8d90", fn_141ce8d90 as *const ()), // content::EntityBatch::ClassTypeInfoImplementation::content::EntityBatch::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141ce8e10, "content_141ce8e10", fn_141ce8e10 as *const ()), // content::EntityBatch::ClassTypeInfoImplementation::content::EntityBatch::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141ce8e80, "content_141ce8e80", fn_141ce8e80 as *const ()), // content::EntityBatch::ClassTypeInfoImplementation::content::EntityBatch::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141ce8f00, "content_141ce8f00", fn_141ce8f00 as *const ()), // content::EntityBatch::ClassTypeInfoImplementation::content::EntityBatch::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141ce94c0, "content_141ce94c0", tpl_35199::<0x14387e810, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FontMetadata::ClassTypeInfoImplementation::content::FontMetadata::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x141ce95b0, "content_141ce95b0", tpl_35199::<0x14387e810, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FontMetadata::ClassTypeInfoImplementation::content::FontMetadata::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x141ce9790, "content_141ce9790", tpl_35199::<0x14387e810, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FontMetadata::ClassTypeInfoImplementation::content::FontMetadata::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x141ce9880, "content_141ce9880", tpl_35199::<0x14387e810, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FontMetadata::ClassTypeInfoImplementation::content::FontMetadata::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x141ce9900, "content_141ce9900", tpl_35199::<0x14387e810, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FontMetadata::ClassTypeInfoImplementation::content::FontMetadata::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141ce9fe0, "content_141ce9fe0", fn_141ce9fe0 as *const ()), // content::FlareSystem::ClassTypeInfoImplementation::content::FlareSystem::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cea070, "content_141cea070", tpl_39306::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x140a5c0a0, 0x142442c80> as *const ()), // content::FlareSystem::ClassTypeInfoImplementation::content::FlareSystem::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141cea0d0, "content_141cea0d0", tpl_35191::<0x10, 0x140a5c0a0, 0x142442de0> as *const ()), // content::FlareSystem::ClassTypeInfoImplementation::content::FlareSystem::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141cea1a0, "content_141cea1a0", tpl_39307::<0x1432b1390, 0x1, 0x14082b2b0> as *const ()), // content::FlareItem::ClassTypeInfoImplementation::content::FlareItem::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141cea1e0, "content_141cea1e0", tpl_39307::<0x1432b1390, 0x1, 0x14082b2b0> as *const ()), // content::FlareItem::ClassTypeInfoImplementation::content::FlareItem::ClassTypeInfoImplementation::vfunc_36
    Replacement::draft(0x141cea6a0, "content_141cea6a0", fn_141cea6a0 as *const ()), // content::FlareItem::ClassTypeInfoImplementation::content::FlareItem::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141ceaa80, "content_141ceaa80", fn_141ceaa80 as *const ()), // content::BundleHeader::Location::ClassTypeInfoImplementation::content::BundleHeader::Location::ClassTypeInfoImplementati
    Replacement::draft(0x141ceaac0, "content_141ceaac0", fn_141ceaac0 as *const ()), // content::BundleHeader::Location::ClassTypeInfoImplementation::content::BundleHeader::Location::ClassTypeInfoImplementati
    Replacement::draft(0x141ceac00, "content_141ceac00", tpl_35199::<0x14387e7d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BundleHeader::Location::ClassTypeInfoImplementation::content::BundleHeader::Location::ClassTypeInfoImplementati
    Replacement::draft(0x141ceac60, "content_141ceac60", tpl_35199::<0x14387e7d8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BundleHeader::Location::ClassTypeInfoImplementation::content::BundleHeader::Location::ClassTypeInfoImplementati
    Replacement::draft(0x141cead10, "content_141cead10", tpl_35226::<0x10> as *const ()), // content::BundleHeader::Location::ClassTypeInfoImplementation::content::BundleHeader::Location::ClassTypeInfoImplementati
    Replacement::draft(0x141ceaeb0, "content_141ceaeb0", tpl_239153 as *const ()), // sub_141ceaeb0
    Replacement::draft(0x141ceaed0, "content_141ceaed0", tpl_239153 as *const ()), // sub_141ceaed0
    Replacement::draft(0x141ceb2d0, "content_141ceb2d0", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // content::BundleHeader::ClassTypeInfoImplementation::content::BundleHeader::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141ceb330, "content_141ceb330", tpl_35198::<0x1401de830, 0x142442de0> as *const ()), // content::BundleHeader::ClassTypeInfoImplementation::content::BundleHeader::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141ceb680, "content_141ceb680", tpl_39311::<0x8, 0x10, 0x18, 0x1c> as *const ()), // content::CurvePoint::ClassTypeInfoImplementation::content::CurvePoint::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141ceb6e0, "content_141ceb6e0", tpl_39311::<0x8, 0x10, 0x18, 0x1c> as *const ()), // content::CurvePoint::ClassTypeInfoImplementation::content::CurvePoint::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141ceb8b0, "content_141ceb8b0", tpl_35239 as *const ()), // content::CurvePoint::ClassTypeInfoImplementation::content::CurvePoint::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cebe20, "content_141cebe20", fn_141cebe20 as *const ()), // content::Curve::ClassTypeInfoImplementation::content::Curve::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141cebe90, "content_141cebe90", fn_141cebe90 as *const ()), // content::Curve::ClassTypeInfoImplementation::content::Curve::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141cebf00, "content_141cebf00", fn_141cebf00 as *const ()), // content::Curve::ClassTypeInfoImplementation::content::Curve::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141cebfe0, "content_141cebfe0", tpl_39315::<0x1432a74f0, 0x1432a6658, 0x1432a7520, 0x1409fb590, 0x14240ab10> as *const ()), // content::CharacterFootstepBone::ClassTypeInfoImplementation::content::CharacterFootstepBone::ClassTypeInfoImplementation
    Replacement::draft(0x141cec040, "content_141cec040", tpl_39315::<0x1432a74f0, 0x1432a6658, 0x1432a7520, 0x1409fb590, 0x14240ab10> as *const ()), // content::CharacterFootstepBone::ClassTypeInfoImplementation::content::CharacterFootstepBone::ClassTypeInfoImplementation
    Replacement::draft(0x141cec4f0, "content_141cec4f0", fn_141cec4f0 as *const ()), // content::CharacterFootstepBone::ClassTypeInfoImplementation::content::CharacterFootstepBone::ClassTypeInfoImplementation
    Replacement::draft(0x141cec550, "content_141cec550", tpl_38952::<0x28, 0x14240ab10> as *const ()), // content::CharacterFootstepBone::ClassTypeInfoImplementation::content::CharacterFootstepBone::ClassTypeInfoImplementation
    Replacement::draft(0x141cec580, "content_141cec580", fn_141cec580 as *const ()), // content::CharacterFootstepBone::ClassTypeInfoImplementation::content::CharacterFootstepBone::ClassTypeInfoImplementation
    Replacement::draft(0x141cec5c0, "content_141cec5c0", tpl_98840::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x28, 0x14240ab10, 0x142442c80> as *const ()), // content::CharacterFootstepBone::ClassTypeInfoImplementation::content::CharacterFootstepBone::ClassTypeInfoImplementation
    Replacement::draft(0x141cec630, "content_141cec630", tpl_38954::<0x28, 0x14240ab10, 0x142442de0> as *const ()), // content::CharacterFootstepBone::ClassTypeInfoImplementation::content::CharacterFootstepBone::ClassTypeInfoImplementation
    Replacement::draft(0x141cecad0, "content_141cecad0", tpl_39318::<0x14387e758, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BoneFoleyDistance::ClassTypeInfoImplementation::content::BoneFoleyDistance::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cecb30, "content_141cecb30", tpl_39318::<0x14387e758, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BoneFoleyDistance::ClassTypeInfoImplementation::content::BoneFoleyDistance::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cecbe0, "content_141cecbe0", tpl_39319::<0xa8, 0x14240ab10, 0x70, 0x48> as *const ()), // content::BoneFoleyDistance::ClassTypeInfoImplementation::content::BoneFoleyDistance::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cecc20, "content_141cecc20", fn_141cecc20 as *const ()), // content::BoneFoleyDistance::ClassTypeInfoImplementation::content::BoneFoleyDistance::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cecca0, "content_141cecca0", tpl_99113::<0xfffffffffffffff8, 0xfffffffffffffff0, 0xa8, 0x14240ab10, 0x70, 0x48, 0x142442c80> as *const ()), // content::BoneFoleyDistance::ClassTypeInfoImplementation::content::BoneFoleyDistance::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cecd20, "content_141cecd20", tpl_39321::<0xa8, 0x14240ab10, 0x70, 0x48, 0x142442de0> as *const ()), // content::BoneFoleyDistance::ClassTypeInfoImplementation::content::BoneFoleyDistance::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141ced1d0, "content_141ced1d0", tpl_39318::<0x14387e800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BoneFoleyRelativeVelocity::ClassTypeInfoImplementation::content::BoneFoleyRelativeVelocity::ClassTypeInfoImplem
    Replacement::draft(0x141ced230, "content_141ced230", tpl_39318::<0x14387e800, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BoneFoleyRelativeVelocity::ClassTypeInfoImplementation::content::BoneFoleyRelativeVelocity::ClassTypeInfoImplem
    Replacement::draft(0x141ced2e0, "content_141ced2e0", tpl_39319::<0xa0, 0x14240ab10, 0x70, 0x48> as *const ()), // content::BoneFoleyRelativeVelocity::ClassTypeInfoImplementation::content::BoneFoleyRelativeVelocity::ClassTypeInfoImplem
    Replacement::draft(0x141ced320, "content_141ced320", fn_141ced320 as *const ()), // content::BoneFoleyRelativeVelocity::ClassTypeInfoImplementation::content::BoneFoleyRelativeVelocity::ClassTypeInfoImplem
    Replacement::draft(0x141ced390, "content_141ced390", tpl_99113::<0xfffffffffffffff8, 0xfffffffffffffff0, 0xa0, 0x14240ab10, 0x70, 0x48, 0x142442c80> as *const ()), // content::BoneFoleyRelativeVelocity::ClassTypeInfoImplementation::content::BoneFoleyRelativeVelocity::ClassTypeInfoImplem
    Replacement::draft(0x141ced410, "content_141ced410", tpl_39321::<0xa0, 0x14240ab10, 0x70, 0x48, 0x142442de0> as *const ()), // content::BoneFoleyRelativeVelocity::ClassTypeInfoImplementation::content::BoneFoleyRelativeVelocity::ClassTypeInfoImplem
    Replacement::draft(0x141ced8c0, "content_141ced8c0", tpl_39318::<0x14387e788, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BoneFoleyVelocity::ClassTypeInfoImplementation::content::BoneFoleyVelocity::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141ced920, "content_141ced920", tpl_39318::<0x14387e788, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BoneFoleyVelocity::ClassTypeInfoImplementation::content::BoneFoleyVelocity::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141ced9b0, "content_141ced9b0", fn_141ced9b0 as *const ()), // content::BoneFoleyVelocity::ClassTypeInfoImplementation::content::BoneFoleyVelocity::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141ceda10, "content_141ceda10", tpl_38861::<0x78, 0x14240ab10, 0x48> as *const ()), // content::BoneFoleyVelocity::ClassTypeInfoImplementation::content::BoneFoleyVelocity::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141ceda40, "content_141ceda40", fn_141ceda40 as *const ()), // content::BoneFoleyVelocity::ClassTypeInfoImplementation::content::BoneFoleyVelocity::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141ceda90, "content_141ceda90", tpl_98785::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x78, 0x14240ab10, 0x48, 0x142442c80> as *const ()), // content::BoneFoleyVelocity::ClassTypeInfoImplementation::content::BoneFoleyVelocity::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cedb00, "content_141cedb00", tpl_38863::<0x78, 0x14240ab10, 0x48, 0x142442de0> as *const ()), // content::BoneFoleyVelocity::ClassTypeInfoImplementation::content::BoneFoleyVelocity::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cede40, "content_141cede40", tpl_38951::<0x8, 0x14240aa00, 0x28> as *const ()), // content::AudioSwitch::ClassTypeInfoImplementation::content::AudioSwitch::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141cede90, "content_141cede90", tpl_38951::<0x8, 0x14240aa00, 0x28> as *const ()), // content::AudioSwitch::ClassTypeInfoImplementation::content::AudioSwitch::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141cee0b0, "content_141cee0b0", tpl_38937::<0x14240aa00, 0x28> as *const ()), // content::AudioSwitch::ClassTypeInfoImplementation::content::AudioSwitch::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cee100, "content_141cee100", tpl_38952::<0x28, 0x14240ab10> as *const ()), // content::AudioSwitch::ClassTypeInfoImplementation::content::AudioSwitch::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141cee170, "content_141cee170", tpl_98840::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x28, 0x14240ab10, 0x142442c80> as *const ()), // content::AudioSwitch::ClassTypeInfoImplementation::content::AudioSwitch::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141cee1e0, "content_141cee1e0", tpl_38954::<0x28, 0x14240ab10, 0x142442de0> as *const ()), // content::AudioSwitch::ClassTypeInfoImplementation::content::AudioSwitch::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141cee710, "content_141cee710", fn_141cee710 as *const ()), // content::AttachmentResources::ClassTypeInfoImplementation::content::AttachmentResources::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cee790, "content_141cee790", tpl_38941::<0x50, 0x1401de830, 0x40, 0x30, 0x20, 0x10> as *const ()), // content::AttachmentResources::ClassTypeInfoImplementation::content::AttachmentResources::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cee7e0, "content_141cee7e0", tpl_38942::<0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58> as *const ()), // content::AttachmentResources::ClassTypeInfoImplementation::content::AttachmentResources::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cee820, "content_141cee820", tpl_38943::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x50, 0x1401de830, 0x40, 0x30, 0x20, 0x10, 0x142442c80> as *const ()), // content::AttachmentResources::ClassTypeInfoImplementation::content::AttachmentResources::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cee8c0, "content_141cee8c0", tpl_38944::<0x50, 0x1401de830, 0x40, 0x30, 0x20, 0x10, 0x142442de0> as *const ()), // content::AttachmentResources::ClassTypeInfoImplementation::content::AttachmentResources::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141ceefa0, "content_141ceefa0", fn_141ceefa0 as *const ()), // content::AnimationSourceDebugMetadata::ClassTypeInfoImplementation::content::AnimationSourceDebugMetadata::ClassTypeInfo
    Replacement::draft(0x141cef1a0, "content_141cef1a0", tpl_38875::<0x10, 0x140853830> as *const ()), // content::AnimationSourceMetadata::content::AnimationSourceMetadata::vfunc_3
    Replacement::draft(0x141cef300, "content_141cef300", fn_141cef300 as *const ()), // content::AnimationSourceMetadata::ClassTypeInfoImplementation::content::AnimationSourceMetadata::ClassTypeInfoImplementa
    Replacement::draft(0x141cef710, "content_141cef710", tpl_39328::<0x8, 0x10, 0x1403d79a0, 0x20, 0x140265ba0> as *const ()), // content::AnimationSourceMetadata::ClassTypeInfoImplementation::content::AnimationSourceMetadata::ClassTypeInfoImplementa
    Replacement::draft(0x141cef900, "content_141cef900", tpl_39329::<0x1432b0a48, 0x14329ebe8, 0x1432b0ac0, 0x140947eb0> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141cef940, "content_141cef940", tpl_39329::<0x1432b0a48, 0x14329ebe8, 0x1432b0ac0, 0x140947eb0> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_36
    Replacement::draft(0x141cefbc0, "content_141cefbc0", tpl_39004::<0x8, 0x1401e84a0, 0x18> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141cefc10, "content_141cefc10", tpl_39004::<0x8, 0x1401e84a0, 0x18> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141cefd60, "content_141cefd60", tpl_35199::<0x14387e858, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141cefdc0, "content_141cefdc0", tpl_35199::<0x14387e858, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141cefe70, "content_141cefe70", tpl_39005::<0x1401e84a0, 0x18> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cefeb0, "content_141cefeb0", tpl_37083::<0x8> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141cefee0, "content_141cefee0", tpl_37085::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x8, 0x142442c80> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141ceff20, "content_141ceff20", tpl_37086::<0x8, 0x142442de0> as *const ()), // content::SpawnAmounts::ClassTypeInfoImplementation::content::SpawnAmounts::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft_throws(0x141cf0c60, "content_141cf0c60", fn_141cf0c60 as *const (), Throws::Caught), // sub_141cf0c60
    Replacement::draft_throws(0x141cf0cd0, "content_141cf0cd0", fn_141cf0cd0 as *const (), Throws::Caught), // sub_141cf0cd0
    Replacement::draft(0x141cf0d40, "content_141cf0d40", fn_141cf0d40 as *const ()), // sub_141cf0d40
    Replacement::draft(0x141cf0ee0, "content_141cf0ee0", fn_141cf0ee0 as *const ()), // sub_141cf0ee0
    Replacement::draft(0x141cf16a0, "content_141cf16a0", tpl_39173::<0x10, 0x1432d64a8, 0x1432bd058, 0x140c05f70> as *const ()), // content::NLWeaponComponent::content::NLWeaponComponent::vfunc_22
    Replacement::draft(0x141cf16f0, "content_141cf16f0", tpl_39173::<0x10, 0x1432d64a8, 0x1432bd058, 0x140c05f40> as *const ()), // content::NLWeaponComponent::content::NLWeaponComponent::vfunc_24
    Replacement::draft_throws(0x141cf17a0, "content_141cf17a0", tpl_838::<0x140bf9480> as *const (), Throws::Discard), // content::NLWeaponComponent::content::NLWeaponComponent::vfunc_32
    Replacement::draft(0x141cf1810, "content_141cf1810", tpl_39331::<0x1432d64a8, 0x1432bd058, 0x140c05f70> as *const ()), // content::NLWeaponComponent::ClassTypeInfoImplementation::content::NLWeaponComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cf18b0, "content_141cf18b0", tpl_39331::<0x1432d64a8, 0x1432bd058, 0x140c05f40> as *const ()), // content::NLWeaponComponent::ClassTypeInfoImplementation::content::NLWeaponComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cf1ce0, "content_141cf1ce0", fn_141cf1ce0 as *const ()), // content::NLWeaponComponent::ClassTypeInfoImplementation::content::NLWeaponComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cf1d70, "content_141cf1d70", tpl_39333::<0x1432c7118, 0x10, 0x18, 0x20, 0x24, 0x2c, 0x30, 0x38, 0x40, 0x44> as *const ()), // content::NLWeaponComponent::ClassTypeInfoImplementation::content::NLWeaponComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cf2470, "content_141cf2470", fn_141cf2470 as *const ()), // content::NLPlayerLocomotionComponent::ClassTypeInfoImplementation::content::NLPlayerLocomotionComponent::ClassTypeInfoIm
    Replacement::draft_throws(0x141cf2e10, "content_141cf2e10", tpl_38829::<0x14387ec28, 0x14387eb60, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // content::NLPlayerInputComponent::content::NLPlayerInputComponent::vfunc_26
    Replacement::draft(0x141cf2e80, "content_141cf2e80", tpl_38830::<0x14387ec28, 0x14387eb60, 0x1401e9970, 0x1406e1480, 0x1401e9860> as *const ()), // content::NLPlayerInputComponent::content::NLPlayerInputComponent::vfunc_28
    Replacement::draft_throws(0x141cf2f30, "content_141cf2f30", tpl_38829::<0x14387ec28, 0x14387eb60, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // content::NLPlayerInputComponent::content::NLPlayerInputComponent::vfunc_34
    Replacement::draft_throws(0x141cf2fa0, "content_141cf2fa0", tpl_38829::<0x14387ec28, 0x14387eb60, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // content::NLPlayerInputComponent::content::NLPlayerInputComponent::vfunc_36
    Replacement::draft_throws(0x141cf30f0, "content_141cf30f0", tpl_38831::<0x14387ec28, 0x14387eb60, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // content::NLPlayerInputComponent::ClassTypeInfoImplementation::content::NLPlayerInputComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141cf31b0, "content_141cf31b0", tpl_38832::<0x14387ec28, 0x14387eb60, 0x1401e9970, 0x1406e1480, 0x1401e9860> as *const ()), // content::NLPlayerInputComponent::ClassTypeInfoImplementation::content::NLPlayerInputComponent::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141cf3360, "content_141cf3360", tpl_38831::<0x14387ec28, 0x14387eb60, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // content::NLPlayerInputComponent::ClassTypeInfoImplementation::content::NLPlayerInputComponent::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141cf3420, "content_141cf3420", tpl_38831::<0x14387ec28, 0x14387eb60, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // content::NLPlayerInputComponent::ClassTypeInfoImplementation::content::NLPlayerInputComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141cf3710, "content_141cf3710", tpl_38526::<0x1432d4710, 0x44, 0x143875430, 0x1432d4838, 0x1432d46e8, 0x1401de410> as *const ()), // content::NLPlayerCameraComponent::content::NLPlayerCameraComponent::vfunc_22
    Replacement::draft(0x141cf3780, "content_141cf3780", tpl_38526::<0x1432d4710, 0x44, 0x1438754b8, 0x1432d4838, 0x1432d46e8, 0x1401de410> as *const ()), // content::NLPlayerCameraComponent::content::NLPlayerCameraComponent::vfunc_24
    Replacement::draft(0x141cf38c0, "content_141cf38c0", tpl_39335::<0x143875430, 0x1432d4710, 0x1432d4838, 0x1432d46e8, 0x1401de410> as *const ()), // content::NLPlayerCameraComponent::ClassTypeInfoImplementation::content::NLPlayerCameraComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cf3980, "content_141cf3980", tpl_39335::<0x1438754b8, 0x1432d4710, 0x1432d4838, 0x1432d46e8, 0x1401de410> as *const ()), // content::NLPlayerCameraComponent::ClassTypeInfoImplementation::content::NLPlayerCameraComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cf3dd0, "content_141cf3dd0", fn_141cf3dd0 as *const ()), // content::NLPlayerCameraComponent::ClassTypeInfoImplementation::content::NLPlayerCameraComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cf3e80, "content_141cf3e80", tpl_38566::<0x1432c2230, 0x44> as *const ()), // content::NLPlayerCameraComponent::ClassTypeInfoImplementation::content::NLPlayerCameraComponent::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141cf4020, "content_141cf4020", tpl_38829::<0x14387f080, 0x14387f0f8, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // content::NLPlayerAimComponent::content::NLPlayerAimComponent::vfunc_26
    Replacement::draft(0x141cf4090, "content_141cf4090", tpl_38830::<0x14387f080, 0x14387f0f8, 0x1401e9970, 0x1406e1480, 0x1401e9860> as *const ()), // content::NLPlayerAimComponent::content::NLPlayerAimComponent::vfunc_28
    Replacement::draft_throws(0x141cf4140, "content_141cf4140", tpl_38829::<0x14387f080, 0x14387f0f8, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // content::NLPlayerAimComponent::content::NLPlayerAimComponent::vfunc_34
    Replacement::draft_throws(0x141cf41b0, "content_141cf41b0", tpl_38829::<0x14387f080, 0x14387f0f8, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // content::NLPlayerAimComponent::content::NLPlayerAimComponent::vfunc_36
    Replacement::draft_throws(0x141cf4300, "content_141cf4300", tpl_38831::<0x14387f080, 0x14387f0f8, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // content::NLPlayerAimComponent::ClassTypeInfoImplementation::content::NLPlayerAimComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cf43c0, "content_141cf43c0", tpl_38832::<0x14387f080, 0x14387f0f8, 0x1401e9970, 0x1406e1480, 0x1401e9860> as *const ()), // content::NLPlayerAimComponent::ClassTypeInfoImplementation::content::NLPlayerAimComponent::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141cf4570, "content_141cf4570", tpl_38831::<0x14387f080, 0x14387f0f8, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // content::NLPlayerAimComponent::ClassTypeInfoImplementation::content::NLPlayerAimComponent::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141cf4630, "content_141cf4630", tpl_38831::<0x14387f080, 0x14387f0f8, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // content::NLPlayerAimComponent::ClassTypeInfoImplementation::content::NLPlayerAimComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cf49d0, "content_141cf49d0", fn_141cf49d0 as *const ()), // content::NLHealthComponent::content::NLHealthComponent::vfunc_33
    Replacement::draft(0x141cf4cf0, "content_141cf4cf0", fn_141cf4cf0 as *const ()), // content::NLHealthComponent::ClassTypeInfoImplementation::content::NLHealthComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cf4ee0, "content_141cf4ee0", tpl_39339::<0x8, 0xc, 0xd> as *const ()), // content::NLHealthComponent::ClassTypeInfoImplementation::content::NLHealthComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cf56b0, "content_141cf56b0", tpl_38695::<0x8> as *const ()), // content::NLDeathHidingComponent::ClassTypeInfoImplementation::content::NLDeathHidingComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141cf5df0, "content_141cf5df0", tpl_39340::<0x8, 0x140244c00, 0x18, 0x140244b20, 0x28, 0x29, 0x2c> as *const ()), // content::NLAIVisionSensorComponent::ClassTypeInfoImplementation::content::NLAIVisionSensorComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cf6430, "content_141cf6430", fn_141cf6430 as *const ()), // content::NLAILocomotionComponent::ClassTypeInfoImplementation::content::NLAILocomotionComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cf64f0, "content_141cf64f0", fn_141cf64f0 as *const ()), // content::NLAILocomotionComponent::ClassTypeInfoImplementation::content::NLAILocomotionComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cf6c90, "content_141cf6c90", tpl_38695::<0x8> as *const ()), // content::NLAbilityLaunchComponent::ClassTypeInfoImplementation::content::NLAbilityLaunchComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141cf7bf0, "content_141cf7bf0", fn_141cf7bf0 as *const ()), // content::WorldInteractionComponent::ClassTypeInfoImplementation::content::WorldInteractionComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cf7cc0, "content_141cf7cc0", fn_141cf7cc0 as *const ()), // content::WorldInteractionComponent::ClassTypeInfoImplementation::content::WorldInteractionComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cf8290, "content_141cf8290", fn_141cf8290 as *const ()), // content::WeaponUpgradesComponent::ClassTypeInfoImplementation::content::WeaponUpgradesComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cf8310, "content_141cf8310", tpl_39113::<0x143085238, 0x8, 0x10, 0x20, 0x28, 0x30, 0x38> as *const ()), // content::WeaponUpgradesComponent::ClassTypeInfoImplementation::content::WeaponUpgradesComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141cf8720, "content_141cf8720", fn_141cf8720 as *const ()), // content::WeaponInputLogicComponent::ClassTypeInfoImplementation::content::WeaponInputLogicComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cf88f0, "content_141cf88f0", tpl_38620::<0x8, 0xc, 0x10, 0x14> as *const ()), // content::WeaponInputLogicComponent::ClassTypeInfoImplementation::content::WeaponInputLogicComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cf8950, "content_141cf8950", tpl_38723::<0x143088d68, 0xc, 0x14> as *const ()), // content::WeaponInputLogicComponent::ClassTypeInfoImplementation::content::WeaponInputLogicComponent::ClassTypeInfoImplem
    Replacement::draft(0x141cf9790, "content_141cf9790", tpl_39347::<0x8, 0x140bfa790, 0x18, 0x140bfa5e0, 0x28> as *const ()), // content::UtilityObjectSpawnerSingletonComponent::ClassTypeInfoImplementation::content::UtilityObjectSpawnerSingletonComp
    Replacement::draft(0x141cf9c60, "content_141cf9c60", tpl_39348::<0x8, 0xc, 0x10> as *const ()), // content::TidalWaveTargetComponent::ClassTypeInfoImplementation::content::TidalWaveTargetComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141cfbe30, "content_141cfbe30", fn_141cfbe30 as *const ()), // content::SourcePointComponent::ClassTypeInfoImplementation::content::SourcePointComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cfbf10, "content_141cfbf10", fn_141cfbf10 as *const ()), // content::SourcePointComponent::ClassTypeInfoImplementation::content::SourcePointComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cfc140, "content_141cfc140", fn_141cfc140 as *const ()), // content::SlowdownComponent::content::SlowdownComponent::vfunc_33
    Replacement::draft(0x141cfc4a0, "content_141cfc4a0", fn_141cfc4a0 as *const ()), // content::SlowdownComponent::ClassTypeInfoImplementation::content::SlowdownComponent::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141cfce60, "content_141cfce60", tpl_39353::<0x14384ea70, 0x1432bdf58, 0x1432bdf88, 0x1432bdf68, 0x1401de410> as *const ()), // content::SignageComponent::ClassTypeInfoImplementation::content::SignageComponent::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141cfcee0, "content_141cfcee0", tpl_39353::<0x14384ea70, 0x1432bdf58, 0x1432bdf88, 0x1432bdf68, 0x1401de410> as *const ()), // content::SignageComponent::ClassTypeInfoImplementation::content::SignageComponent::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141cfcf60, "content_141cfcf60", tpl_39353::<0x14384ea70, 0x1432bdf58, 0x1432bdf88, 0x1432bdf68, 0x1401de410> as *const ()), // content::SignageComponent::ClassTypeInfoImplementation::content::SignageComponent::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141cfd390, "content_141cfd390", fn_141cfd390 as *const ()), // content::SignageComponent::ClassTypeInfoImplementation::content::SignageComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141cfd420, "content_141cfd420", fn_141cfd420 as *const ()), // content::SignageComponent::ClassTypeInfoImplementation::content::SignageComponent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141cfde10, "content_141cfde10", fn_141cfde10 as *const ()), // content::QuickEscapeComponent::ClassTypeInfoImplementation::content::QuickEscapeComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cfdfd0, "content_141cfdfd0", fn_141cfdfd0 as *const ()), // content::QuickEscapeComponent::ClassTypeInfoImplementation::content::QuickEscapeComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cfe020, "content_141cfe020", fn_141cfe020 as *const ()), // content::QuickEscapeComponent::ClassTypeInfoImplementation::content::QuickEscapeComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cfe100, "content_141cfe100", tpl_365::<0x140bd1270> as *const ()), // content::QuickEscapeComponent::ClassTypeInfoImplementation::content::QuickEscapeComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141cff000, "content_141cff000", fn_141cff000 as *const ()), // content::ProjectileComponent::ClassTypeInfoImplementation::content::ProjectileComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cff4a0, "content_141cff4a0", fn_141cff4a0 as *const ()), // content::ProjectileComponent::ClassTypeInfoImplementation::content::ProjectileComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141cff750, "content_141cff750", tpl_38518::<0x14384e930, 0x1432be4d0, 0x1432be3e0, 0x1432be4e8, 0x1401de410> as *const ()), // content::ProhibitFactionZoneComponent::ClassTypeInfoImplementation::content::ProhibitFactionZoneComponent::ClassTypeInfo
    Replacement::draft(0x141cff7d0, "content_141cff7d0", tpl_38518::<0x14384e930, 0x1432be4d0, 0x1432be3e0, 0x1432be4e8, 0x1401de410> as *const ()), // content::ProhibitFactionZoneComponent::ClassTypeInfoImplementation::content::ProhibitFactionZoneComponent::ClassTypeInfo
    Replacement::draft(0x141cff850, "content_141cff850", tpl_38518::<0x14384e930, 0x1432be4d0, 0x1432be3e0, 0x1432be4e8, 0x1401de410> as *const ()), // content::ProhibitFactionZoneComponent::ClassTypeInfoImplementation::content::ProhibitFactionZoneComponent::ClassTypeInfo
    Replacement::draft(0x141cffcc0, "content_141cffcc0", fn_141cffcc0 as *const ()), // content::ProhibitFactionZoneComponent::ClassTypeInfoImplementation::content::ProhibitFactionZoneComponent::ClassTypeInfo
    Replacement::draft(0x141d006c0, "content_141d006c0", tpl_39362::<0x8, 0x14240aa00, 0x90, 0x94> as *const ()), // content::PlayerStatusComponent::ClassTypeInfoImplementation::content::PlayerStatusComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d008c0, "content_141d008c0", tpl_39363::<0x8, 0x14240aa00, 0x90, 0x94> as *const ()), // content::PlayerStatusComponent::ClassTypeInfoImplementation::content::PlayerStatusComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d01980, "content_141d01980", fn_141d01980 as *const ()), // content::PlayerPeekingComponent::ClassTypeInfoImplementation::content::PlayerPeekingComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d01a30, "content_141d01a30", tpl_39098::<0x1432c7fb8, 0x30, 0x38, 0x40, 0x44, 0x48, 0x50, 0x58, 0x5c> as *const ()), // content::PlayerPeekingComponent::ClassTypeInfoImplementation::content::PlayerPeekingComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d02020, "content_141d02020", fn_141d02020 as *const ()), // content::PlayerMovementSystemComponent::ClassTypeInfoImplementation::content::PlayerMovementSystemComponent::ClassTypeIn
    Replacement::draft(0x141d02100, "content_141d02100", fn_141d02100 as *const ()), // content::PlayerMovementSystemComponent::ClassTypeInfoImplementation::content::PlayerMovementSystemComponent::ClassTypeIn
    Replacement::draft(0x141d02560, "content_141d02560", tpl_38536::<0x8, 0xc> as *const ()), // content::PlayerDeathComponent::ClassTypeInfoImplementation::content::PlayerDeathComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d02720, "content_141d02720", tpl_38537::<0x8, 0xc> as *const ()), // content::PlayerDeathComponent::ClassTypeInfoImplementation::content::PlayerDeathComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d02970, "content_141d02970", tpl_365::<0x140bb97f0> as *const ()), // content::PlayerCoverComponent::content::PlayerCoverComponent::vfunc_27
    Replacement::draft(0x141d029e0, "content_141d029e0", tpl_365::<0x140bb97f0> as *const ()), // content::PlayerCoverComponent::content::PlayerCoverComponent::vfunc_29
    Replacement::draft(0x141d02a90, "content_141d02a90", tpl_365::<0x140bb97f0> as *const ()), // content::PlayerCoverComponent::content::PlayerCoverComponent::vfunc_35
    Replacement::draft(0x141d02b00, "content_141d02b00", tpl_365::<0x140bb97f0> as *const ()), // content::PlayerCoverComponent::content::PlayerCoverComponent::vfunc_37
    Replacement::draft(0x141d02b10, "content_141d02b10", tpl_365::<0x140bb97f0> as *const ()), // content::PlayerCoverComponent::content::PlayerCoverComponent::vfunc_38
    Replacement::draft(0x141d03660, "content_141d03660", fn_141d03660 as *const ()), // content::PerfectDodgeTargetComponent::ClassTypeInfoImplementation::content::PerfectDodgeTargetComponent::ClassTypeInfoIm
    Replacement::draft(0x141d036d0, "content_141d036d0", tpl_39145::<0x143195660, 0x10, 0x18, 0x20, 0x28> as *const ()), // content::PerfectDodgeTargetComponent::ClassTypeInfoImplementation::content::PerfectDodgeTargetComponent::ClassTypeInfoIm
    Replacement::draft(0x141d03ca0, "content_141d03ca0", fn_141d03ca0 as *const ()), // content::OutfitComponent::ClassTypeInfoImplementation::content::OutfitComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d04450, "content_141d04450", fn_141d04450 as *const ()), // content::NPCScheduleComponent::ClassTypeInfoImplementation::content::NPCScheduleComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d04780, "content_141d04780", fn_141d04780 as *const ()), // content::NPCScheduleComponent::ClassTypeInfoImplementation::content::NPCScheduleComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d04940, "content_141d04940", fn_141d04940 as *const ()), // content::NPCScheduleComponent::ClassTypeInfoImplementation::content::NPCScheduleComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d04990, "content_141d04990", fn_141d04990 as *const ()), // content::NPCScheduleComponent::ClassTypeInfoImplementation::content::NPCScheduleComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d04af0, "content_141d04af0", tpl_39138::<0x1432bd068, 0x1432bd058, 0x140c05f70> as *const ()), // content::NoiseComponent::ClassTypeInfoImplementation::content::NoiseComponent::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141d04b80, "content_141d04b80", tpl_39138::<0x1432bd068, 0x1432bd058, 0x140c05f40> as *const ()), // content::NoiseComponent::ClassTypeInfoImplementation::content::NoiseComponent::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141d04f90, "content_141d04f90", fn_141d04f90 as *const ()), // content::NoiseComponent::ClassTypeInfoImplementation::content::NoiseComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d05010, "content_141d05010", tpl_39333::<0x1430fc700, 0x8, 0x10, 0x18, 0x1c, 0x24, 0x28, 0x30, 0x38, 0x3c> as *const ()), // content::NoiseComponent::ClassTypeInfoImplementation::content::NoiseComponent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141d05710, "content_141d05710", fn_141d05710 as *const ()), // content::MovementSystemComponent::ClassTypeInfoImplementation::content::MovementSystemComponent::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141d05940, "content_141d05940", tpl_838::<0x140ba7ed0> as *const (), Throws::Discard), // content::MapVisualizationComponent::content::MapVisualizationComponent::vfunc_26
    Replacement::draft(0x141d05e60, "content_141d05e60", tpl_38874::<0x8, 0x14240aa00, 0x30, 0x14028cf90> as *const ()), // content::MapVisualizationComponent::ClassTypeInfoImplementation::content::MapVisualizationComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d05ed0, "content_141d05ed0", tpl_39375::<0x1432bfce8, 0x8, 0xc, 0x10, 0xb, 0x30, 0x38> as *const ()), // content::MapVisualizationComponent::ClassTypeInfoImplementation::content::MapVisualizationComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d062e0, "content_141d062e0", fn_141d062e0 as *const ()), // content::MapBoundsPolygonDefinition::ClassTypeInfoImplementation::content::MapBoundsPolygonDefinition::ClassTypeInfoImpl
    Replacement::draft(0x141d064d0, "content_141d064d0", fn_141d064d0 as *const ()), // content::MapBoundsPolygonDefinition::ClassTypeInfoImplementation::content::MapBoundsPolygonDefinition::ClassTypeInfoImpl
    Replacement::draft(0x141d06550, "content_141d06550", fn_141d06550 as *const ()), // content::MapBoundsPolygonDefinition::ClassTypeInfoImplementation::content::MapBoundsPolygonDefinition::ClassTypeInfoImpl
    Replacement::draft(0x141d06960, "content_141d06960", fn_141d06960 as *const ()), // content::MapRoomDefinition::ClassTypeInfoImplementation::content::MapRoomDefinition::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d06b50, "content_141d06b50", fn_141d06b50 as *const ()), // content::MapRoomDefinition::ClassTypeInfoImplementation::content::MapRoomDefinition::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d06bd0, "content_141d06bd0", fn_141d06bd0 as *const ()), // content::MapRoomDefinition::ClassTypeInfoImplementation::content::MapRoomDefinition::ClassTypeInfoImplementation::vfunc_
    Replacement::draft_throws(0x141d06d90, "content_141d06d90", fn_141d06d90 as *const (), Throws::Discard), // content::MapOverlayDefinition::ClassTypeInfoImplementation::content::MapOverlayDefinition::ClassTypeInfoImplementation::
    Replacement::draft(0x141d07030, "content_141d07030", fn_141d07030 as *const ()), // content::MapOverlayDefinition::ClassTypeInfoImplementation::content::MapOverlayDefinition::ClassTypeInfoImplementation::
    Replacement::draft(0x141d07220, "content_141d07220", fn_141d07220 as *const ()), // content::MapOverlayDefinition::ClassTypeInfoImplementation::content::MapOverlayDefinition::ClassTypeInfoImplementation::
    Replacement::draft(0x141d072a0, "content_141d072a0", fn_141d072a0 as *const ()), // content::MapOverlayDefinition::ClassTypeInfoImplementation::content::MapOverlayDefinition::ClassTypeInfoImplementation::
    Replacement::draft(0x141d07870, "content_141d07870", fn_141d07870 as *const ()), // content::MapMarkerComponent::ClassTypeInfoImplementation::content::MapMarkerComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d07910, "content_141d07910", fn_141d07910 as *const ()), // content::MapMarkerComponent::ClassTypeInfoImplementation::content::MapMarkerComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d07b70, "content_141d07b70", fn_141d07b70 as *const ()), // content::MapAreaComponent::content::MapAreaComponent::vfunc_33
    Replacement::draft(0x141d07eb0, "content_141d07eb0", fn_141d07eb0 as *const ()), // content::MapAreaComponent::ClassTypeInfoImplementation::content::MapAreaComponent::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141d080a0, "content_141d080a0", tpl_38895::<0x8, 0x14240aa00, 0x30, 0x38> as *const ()), // content::MapAreaComponent::ClassTypeInfoImplementation::content::MapAreaComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d08110, "content_141d08110", fn_141d08110 as *const ()), // content::MapAreaComponent::ClassTypeInfoImplementation::content::MapAreaComponent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141d08960, "content_141d08960", tpl_39105::<0x1432bb9e8, 0x10, 0x1426cdcb0, 0x1432bb9f8, 0x14082b2b0> as *const ()), // content::LightSensitiveDecalComponent::content::LightSensitiveDecalComponent::vfunc_22
    Replacement::draft(0x141d08a50, "content_141d08a50", fn_141d08a50 as *const ()), // content::LightSensitiveDecalComponent::content::LightSensitiveDecalComponent::vfunc_33
    Replacement::draft(0x141d08b20, "content_141d08b20", fn_141d08b20 as *const ()), // content::LightSensitiveDecalComponent::ClassTypeInfoImplementation::content::LightSensitiveDecalComponent::ClassTypeInfo
    Replacement::draft(0x141d08e10, "content_141d08e10", fn_141d08e10 as *const ()), // content::LightSensitiveDecalComponent::ClassTypeInfoImplementation::content::LightSensitiveDecalComponent::ClassTypeInfo
    Replacement::draft(0x141d09030, "content_141d09030", fn_141d09030 as *const ()), // content::LightSensitiveDecalComponent::ClassTypeInfoImplementation::content::LightSensitiveDecalComponent::ClassTypeInfo
    Replacement::draft(0x141d090a0, "content_141d090a0", fn_141d090a0 as *const ()), // content::LightSensitiveDecalComponent::ClassTypeInfoImplementation::content::LightSensitiveDecalComponent::ClassTypeInfo
    Replacement::draft(0x141d094b0, "content_141d094b0", fn_141d094b0 as *const ()), // content::LightReactionComponent::ClassTypeInfoImplementation::content::LightReactionComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d09700, "content_141d09700", tpl_38566::<0x1431223a0, 0x18> as *const ()), // content::LightReactionComponent::ClassTypeInfoImplementation::content::LightReactionComponent::ClassTypeInfoImplementati
    Replacement::draft_throws(0x141d0b8b0, "content_141d0b8b0", tpl_838::<0x140b98ec0> as *const (), Throws::Discard), // content::InventoryInputLogicComponent::content::InventoryInputLogicComponent::vfunc_25
    Replacement::draft(0x141d0c600, "content_141d0c600", tpl_39079::<0x8, 0x14240aa00, 0x30, 0x38, 0x14028cf90> as *const ()), // content::InventoryComponent::ClassTypeInfoImplementation::content::InventoryComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d0c670, "content_141d0c670", tpl_39375::<0x143136210, 0x8, 0xc, 0x10, 0xb, 0x38, 0x40> as *const ()), // content::InventoryComponent::ClassTypeInfoImplementation::content::InventoryComponent::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d0c7f0, "content_141d0c7f0", tpl_39397::<0x1432bc000, 0x8, 0x143875580, 0x1432bbf20, 0x1432bc018, 0x1401de410> as *const ()), // content::InteractModuleSelectorComponent::content::InteractModuleSelectorComponent::vfunc_22
    Replacement::draft(0x141d0c850, "content_141d0c850", tpl_39397::<0x1432bc000, 0x8, 0x143875458, 0x1432bbf20, 0x1432bc018, 0x1401de410> as *const ()), // content::InteractModuleSelectorComponent::content::InteractModuleSelectorComponent::vfunc_23
    Replacement::draft(0x141d0c8b0, "content_141d0c8b0", tpl_39397::<0x1432bc000, 0x8, 0x1438771e8, 0x1432bbf20, 0x1432bc018, 0x1401de410> as *const ()), // content::InteractModuleSelectorComponent::content::InteractModuleSelectorComponent::vfunc_24
    Replacement::draft(0x141d0ca00, "content_141d0ca00", tpl_39398::<0x143875580, 0x1432bc000, 0x1432bbf20, 0x1432bc018, 0x1401de410> as *const ()), // content::InteractModuleSelectorComponent::ClassTypeInfoImplementation::content::InteractModuleSelectorComponent::ClassTy
    Replacement::draft(0x141d0ca80, "content_141d0ca80", tpl_39398::<0x143875458, 0x1432bc000, 0x1432bbf20, 0x1432bc018, 0x1401de410> as *const ()), // content::InteractModuleSelectorComponent::ClassTypeInfoImplementation::content::InteractModuleSelectorComponent::ClassTy
    Replacement::draft(0x141d0cb00, "content_141d0cb00", tpl_39398::<0x1438771e8, 0x1432bc000, 0x1432bbf20, 0x1432bc018, 0x1401de410> as *const ()), // content::InteractModuleSelectorComponent::ClassTypeInfoImplementation::content::InteractModuleSelectorComponent::ClassTy
    Replacement::draft(0x141d0f430, "content_141d0f430", tpl_39399::<0x143876230, 0x1432bc178, 0x1432bc500, 0x1432bc188, 0x1401de410> as *const ()), // content::HolsterableComponent::ClassTypeInfoImplementation::content::HolsterableComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d0f4b0, "content_141d0f4b0", tpl_39399::<0x143876230, 0x1432bc178, 0x1432bc500, 0x1432bc188, 0x1401de410> as *const ()), // content::HolsterableComponent::ClassTypeInfoImplementation::content::HolsterableComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d0f530, "content_141d0f530", tpl_39399::<0x143876230, 0x1432bc178, 0x1432bc500, 0x1432bc188, 0x1401de410> as *const ()), // content::HolsterableComponent::ClassTypeInfoImplementation::content::HolsterableComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d0f7b0, "content_141d0f7b0", tpl_38522::<0x8, 0xc, 0x10> as *const ()), // content::HolsterableComponent::ClassTypeInfoImplementation::content::HolsterableComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d0f980, "content_141d0f980", tpl_38523::<0x8, 0xc, 0x10> as *const ()), // content::HolsterableComponent::ClassTypeInfoImplementation::content::HolsterableComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d0f9d0, "content_141d0f9d0", tpl_39135::<0x1430bf490, 0xc> as *const ()), // content::HolsterableComponent::ClassTypeInfoImplementation::content::HolsterableComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d10550, "content_141d10550", fn_141d10550 as *const ()), // content::HapticSensorComponent::ClassTypeInfoImplementation::content::HapticSensorComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d10de0, "content_141d10de0", tpl_239153 as *const ()), // sub_141d10de0
    Replacement::draft(0x141d11180, "content_141d11180", fn_141d11180 as *const ()), // content::GameWeaponKickbackComponent::ClassTypeInfoImplementation::content::GameWeaponKickbackComponent::ClassTypeInfoIm
    Replacement::draft(0x141d11800, "content_141d11800", fn_141d11800 as *const ()), // content::GameWeaponDispersionComponent::ClassTypeInfoImplementation::content::GameWeaponDispersionComponent::ClassTypeIn
    Replacement::draft(0x141d11ab0, "content_141d11ab0", fn_141d11ab0 as *const ()), // content::GameWeaponComponent::ClassTypeInfoImplementation::content::GameWeaponComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141d11f60, "content_141d11f60", fn_141d11f60 as *const ()), // content::GameWeaponComponent::ClassTypeInfoImplementation::content::GameWeaponComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141d12040, "content_141d12040", fn_141d12040 as *const ()), // content::GameWeaponComponent::ClassTypeInfoImplementation::content::GameWeaponComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141d122c0, "content_141d122c0", tpl_39202::<0x8, 0x14240aa00, 0x90, 0x118> as *const ()), // content::GameWeaponAudioComponent::content::GameWeaponAudioComponent::vfunc_33
    Replacement::draft(0x141d12600, "content_141d12600", tpl_39203::<0x8, 0x14240aa00, 0x90, 0x118> as *const ()), // content::GameWeaponAudioComponent::ClassTypeInfoImplementation::content::GameWeaponAudioComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141d12810, "content_141d12810", tpl_39204::<0x8, 0x14240aa00, 0x90, 0x118> as *const ()), // content::GameWeaponAudioComponent::ClassTypeInfoImplementation::content::GameWeaponAudioComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141d128a0, "content_141d128a0", tpl_39406::<0x1432c4480, 0x8, 0xc, 0x10, 0xb, 0x90, 0x94, 0x98, 0x93, 0x118, 0x11c, 0x120> as *const ()), // content::GameWeaponAudioComponent::ClassTypeInfoImplementation::content::GameWeaponAudioComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141d12ec0, "content_141d12ec0", tpl_39340::<0x8, 0x140244c00, 0x18, 0x140244b20, 0x28, 0x29, 0x2c> as *const ()), // content::GameVisualSensorComponent::ClassTypeInfoImplementation::content::GameVisualSensorComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d12f40, "content_141d12f40", fn_141d12f40 as *const ()), // content::GameVisualSensorComponent::ClassTypeInfoImplementation::content::GameVisualSensorComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d140d0, "content_141d140d0", fn_141d140d0 as *const ()), // content::GameRayCastDamageActionComponent::ClassTypeInfoImplementation::content::GameRayCastDamageActionComponent::Class
    Replacement::draft(0x141d143a0, "content_141d143a0", fn_141d143a0 as *const ()), // content::GameRayCastDamageActionComponent::ClassTypeInfoImplementation::content::GameRayCastDamageActionComponent::Class
    Replacement::draft(0x141d14590, "content_141d14590", tpl_39347::<0x8, 0x1401e84a0, 0x20, 0x14240aa00, 0xa8> as *const ()), // content::GameRayCastDamageActionComponent::ClassTypeInfoImplementation::content::GameRayCastDamageActionComponent::Class
    Replacement::draft(0x141d14610, "content_141d14610", fn_141d14610 as *const ()), // content::GameRayCastDamageActionComponent::ClassTypeInfoImplementation::content::GameRayCastDamageActionComponent::Class
    Replacement::draft(0x141d14790, "content_141d14790", fn_141d14790 as *const ()), // content::GameProjectileDamageActionComponent::ClassTypeInfoImplementation::content::GameProjectileDamageActionComponent:
    Replacement::draft(0x141d14c20, "content_141d14c20", fn_141d14c20 as *const ()), // content::GameProjectileDamageActionComponent::ClassTypeInfoImplementation::content::GameProjectileDamageActionComponent:
    Replacement::draft(0x141d14cb0, "content_141d14cb0", fn_141d14cb0 as *const ()), // content::GameProjectileDamageActionComponent::ClassTypeInfoImplementation::content::GameProjectileDamageActionComponent:
    Replacement::draft(0x141d15270, "content_141d15270", fn_141d15270 as *const ()), // content::GamePlayerCameraMovementComponent::ClassTypeInfoImplementation::content::GamePlayerCameraMovementComponent::Cla
    Replacement::draft(0x141d15e30, "content_141d15e30", tpl_365::<0x140b76eb0> as *const ()), // content::GameInventoryComponent::content::GameInventoryComponent::vfunc_28
    Replacement::draft(0x141d15e80, "content_141d15e80", fn_141d15e80 as *const ()), // content::GameInventoryComponent::content::GameInventoryComponent::vfunc_33
    Replacement::draft(0x141d161a0, "content_141d161a0", fn_141d161a0 as *const ()), // content::GameInventoryComponent::ClassTypeInfoImplementation::content::GameInventoryComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d16390, "content_141d16390", tpl_38494::<0x8, 0x18> as *const ()), // content::GameInventoryComponent::ClassTypeInfoImplementation::content::GameInventoryComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d163e0, "content_141d163e0", tpl_39135::<0x1432ce580, 0x14> as *const ()), // content::GameInventoryComponent::ClassTypeInfoImplementation::content::GameInventoryComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d16990, "content_141d16990", fn_141d16990 as *const ()), // content::GameHealthComponent::ClassTypeInfoImplementation::content::GameHealthComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141d16d70, "content_141d16d70", tpl_239153 as *const ()), // sub_141d16d70
    Replacement::draft(0x141d17120, "content_141d17120", fn_141d17120 as *const ()), // content::GameDamageReceiverComponent::ClassTypeInfoImplementation::content::GameDamageReceiverComponent::ClassTypeInfoIm
    Replacement::draft(0x141d171a0, "content_141d171a0", tpl_39419::<0x1432c65a8, 0x28, 0x30> as *const ()), // content::GameDamageReceiverComponent::ClassTypeInfoImplementation::content::GameDamageReceiverComponent::ClassTypeInfoIm
    Replacement::draft(0x141d17590, "content_141d17590", fn_141d17590 as *const ()), // content::GameCursorInteractableComponent::ClassTypeInfoImplementation::content::GameCursorInteractableComponent::ClassTy
    Replacement::draft(0x141d17760, "content_141d17760", tpl_39421::<0x8, 0x9, 0xa> as *const ()), // content::GameCursorInteractableComponent::ClassTypeInfoImplementation::content::GameCursorInteractableComponent::ClassTy
    Replacement::draft(0x141d18360, "content_141d18360", tpl_38874::<0x8, 0x1404c1630, 0x18, 0x14240aa00> as *const ()), // content::GameArmoryComponent::ClassTypeInfoImplementation::content::GameArmoryComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141d183d0, "content_141d183d0", fn_141d183d0 as *const ()), // content::GameArmoryComponent::ClassTypeInfoImplementation::content::GameArmoryComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141d19dc0, "content_141d19dc0", fn_141d19dc0 as *const ()), // content::GameAnimationInputDamageComponent::ClassTypeInfoImplementation::content::GameAnimationInputDamageComponent::Cla
    Replacement::draft(0x141d19f80, "content_141d19f80", tpl_38931::<0x8, 0x10> as *const ()), // content::GameAnimationInputDamageComponent::ClassTypeInfoImplementation::content::GameAnimationInputDamageComponent::Cla
    Replacement::draft(0x141d19fd0, "content_141d19fd0", tpl_38566::<0x1430ebb50, 0x10> as *const ()), // content::GameAnimationInputDamageComponent::ClassTypeInfoImplementation::content::GameAnimationInputDamageComponent::Cla
    Replacement::draft(0x141d1bcf0, "content_141d1bcf0", tpl_39424::<0x1438745e0, 0x143264170, 0x1432b9920, 0x1432b9910, 0x1401de410> as *const ()), // content::GameAimTargetComponent::ClassTypeInfoImplementation::content::GameAimTargetComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d1bdb0, "content_141d1bdb0", tpl_39424::<0x1438762d8, 0x143264170, 0x1432b9920, 0x1432b9910, 0x1401de410> as *const ()), // content::GameAimTargetComponent::ClassTypeInfoImplementation::content::GameAimTargetComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d1c1e0, "content_141d1c1e0", fn_141d1c1e0 as *const ()), // content::GameAimTargetComponent::ClassTypeInfoImplementation::content::GameAimTargetComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d1c290, "content_141d1c290", fn_141d1c290 as *const ()), // content::GameAimTargetComponent::ClassTypeInfoImplementation::content::GameAimTargetComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d1c850, "content_141d1c850", fn_141d1c850 as *const ()), // content::FloatingTextComponent::ClassTypeInfoImplementation::content::FloatingTextComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d1c900, "content_141d1c900", fn_141d1c900 as *const ()), // content::FloatingTextComponent::ClassTypeInfoImplementation::content::FloatingTextComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d1d010, "content_141d1d010", fn_141d1d010 as *const ()), // content::FlashlightTriggersComponent::ClassTypeInfoImplementation::content::FlashlightTriggersComponent::ClassTypeInfoIm
    Replacement::draft(0x141d1d0b0, "content_141d1d0b0", fn_141d1d0b0 as *const ()), // content::FlashlightTriggersComponent::ClassTypeInfoImplementation::content::FlashlightTriggersComponent::ClassTypeInfoIm
    Replacement::draft(0x141d1e910, "content_141d1e910", fn_141d1e910 as *const ()), // content::FlashlightLightComponent::ClassTypeInfoImplementation::content::FlashlightLightComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141d1e9a0, "content_141d1e9a0", fn_141d1e9a0 as *const ()), // content::FlashlightLightComponent::ClassTypeInfoImplementation::content::FlashlightLightComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141d1f580, "content_141d1f580", fn_141d1f580 as *const ()), // content::FlashlightDamageComponent::ClassTypeInfoImplementation::content::FlashlightDamageComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d1f640, "content_141d1f640", fn_141d1f640 as *const ()), // content::FlashlightDamageComponent::ClassTypeInfoImplementation::content::FlashlightDamageComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d1fa80, "content_141d1fa80", fn_141d1fa80 as *const ()), // content::FlashlightControllerComponent::ClassTypeInfoImplementation::content::FlashlightControllerComponent::ClassTypeIn
    Replacement::draft(0x141d1fc90, "content_141d1fc90", fn_141d1fc90 as *const ()), // content::FlashlightControllerComponent::ClassTypeInfoImplementation::content::FlashlightControllerComponent::ClassTypeIn
    Replacement::draft(0x141d1fd10, "content_141d1fd10", tpl_38566::<0x14311eb70, 0x1c> as *const ()), // content::FlashlightControllerComponent::ClassTypeInfoImplementation::content::FlashlightControllerComponent::ClassTypeIn
    Replacement::draft(0x141d1ff20, "content_141d1ff20", fn_141d1ff20 as *const ()), // content::FlashlightBatteryComponent::content::FlashlightBatteryComponent::vfunc_33
    Replacement::draft(0x141d202a0, "content_141d202a0", fn_141d202a0 as *const ()), // content::FlashlightBatteryComponent::ClassTypeInfoImplementation::content::FlashlightBatteryComponent::ClassTypeInfoImpl
    Replacement::draft(0x141d204e0, "content_141d204e0", fn_141d204e0 as *const ()), // content::FlashlightBatteryComponent::ClassTypeInfoImplementation::content::FlashlightBatteryComponent::ClassTypeInfoImpl
    Replacement::draft(0x141d20c90, "content_141d20c90", tpl_39440::<0x143876380, 0x1432b8b80, 0x1432b8bb0, 0x1432b8b90, 0x1401de410> as *const ()), // content::FadeoutComponent::ClassTypeInfoImplementation::content::FadeoutComponent::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141d20d10, "content_141d20d10", tpl_39440::<0x143877220, 0x1432b8b80, 0x1432b8bb0, 0x1432b8b90, 0x1401de410> as *const ()), // content::FadeoutComponent::ClassTypeInfoImplementation::content::FadeoutComponent::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141d20d90, "content_141d20d90", tpl_39440::<0x143874658, 0x1432b8b80, 0x1432b8bb0, 0x1432b8b90, 0x1401de410> as *const ()), // content::FadeoutComponent::ClassTypeInfoImplementation::content::FadeoutComponent::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft_throws(0x141d21430, "content_141d21430", tpl_38831::<0x14387ef90, 0x14387eda0, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // content::EquippableComponent::ClassTypeInfoImplementation::content::EquippableComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141d214f0, "content_141d214f0", tpl_38832::<0x14387ef90, 0x14387eda0, 0x1401e9970, 0x1406e1480, 0x1401e9860> as *const ()), // content::EquippableComponent::ClassTypeInfoImplementation::content::EquippableComponent::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141d216a0, "content_141d216a0", tpl_38831::<0x14387ef90, 0x14387eda0, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // content::EquippableComponent::ClassTypeInfoImplementation::content::EquippableComponent::ClassTypeInfoImplementation::vf
    Replacement::draft_throws(0x141d21760, "content_141d21760", tpl_38831::<0x14387ef90, 0x14387eda0, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // content::EquippableComponent::ClassTypeInfoImplementation::content::EquippableComponent::ClassTypeInfoImplementation::vf
    Replacement::draft(0x141d21a30, "content_141d21a30", tpl_38518::<0x1438755b0, 0x1432b8b50, 0x1432b8af0, 0x1432b8b70, 0x1401de410> as *const ()), // content::EquippableAnimationIDComponent::ClassTypeInfoImplementation::content::EquippableAnimationIDComponent::ClassType
    Replacement::draft(0x141d21ab0, "content_141d21ab0", tpl_38518::<0x1438763a0, 0x1432b8b50, 0x1432b8af0, 0x1432b8b70, 0x1401de410> as *const ()), // content::EquippableAnimationIDComponent::ClassTypeInfoImplementation::content::EquippableAnimationIDComponent::ClassType
    Replacement::draft(0x141d21b30, "content_141d21b30", tpl_38518::<0x143874730, 0x1432b8b50, 0x1432b8af0, 0x1432b8b70, 0x1401de410> as *const ()), // content::EquippableAnimationIDComponent::ClassTypeInfoImplementation::content::EquippableAnimationIDComponent::ClassType
    Replacement::draft(0x141d22710, "content_141d22710", tpl_39441::<0x143876318, 0x143264170, 0x1426cdcb0, 0x1432b6030, 0x1401de410, 0x3c, 0x143874480, 0x1432b7778, 0x1432b7648, 0x1432b7788> as *const ()), // content::DoorComponent::ClassTypeInfoImplementation::content::DoorComponent::ClassTypeInfoImplementation::vfunc_19
    Replacement::draft(0x141d22810, "content_141d22810", tpl_39441::<0x1438753f8, 0x143264170, 0x1426cdcb0, 0x1432b6030, 0x1401de410, 0x3c, 0x143875488, 0x1432b7778, 0x1432b7648, 0x1432b7788> as *const ()), // content::DoorComponent::ClassTypeInfoImplementation::content::DoorComponent::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x141d22c80, "content_141d22c80", fn_141d22c80 as *const ()), // content::DoorComponent::ClassTypeInfoImplementation::content::DoorComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d22d50, "content_141d22d50", fn_141d22d50 as *const ()), // content::DoorComponent::ClassTypeInfoImplementation::content::DoorComponent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141d24930, "content_141d24930", fn_141d24930 as *const ()), // content::DarknessShieldComponent::ClassTypeInfoImplementation::content::DarknessShieldComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d249c0, "content_141d249c0", fn_141d249c0 as *const ()), // content::DarknessShieldComponent::ClassTypeInfoImplementation::content::DarknessShieldComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d24fa0, "content_141d24fa0", tpl_38695::<0x8> as *const ()), // content::DarknessBurnerComponent::ClassTypeInfoImplementation::content::DarknessBurnerComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d255a0, "content_141d255a0", fn_141d255a0 as *const ()), // content::DamageReactionComponent::ClassTypeInfoImplementation::content::DamageReactionComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d25620, "content_141d25620", fn_141d25620 as *const ()), // content::DamageReactionComponent::ClassTypeInfoImplementation::content::DamageReactionComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d25da0, "content_141d25da0", tpl_39448::<0x1432b4ea0, 0x1, 0x1406db220> as *const ()), // content::CameraPanningVolumeComponent::ClassTypeInfoImplementation::content::CameraPanningVolumeComponent::ClassTypeInfo
    Replacement::draft(0x141d26220, "content_141d26220", fn_141d26220 as *const ()), // content::CameraPanningVolumeComponent::ClassTypeInfoImplementation::content::CameraPanningVolumeComponent::ClassTypeInfo
    Replacement::draft(0x141d262f0, "content_141d262f0", fn_141d262f0 as *const ()), // content::CameraPanningVolumeComponent::ClassTypeInfoImplementation::content::CameraPanningVolumeComponent::ClassTypeInfo
    Replacement::draft(0x141d27d60, "content_141d27d60", fn_141d27d60 as *const ()), // content::ClimbableLedgeComponent::ClassTypeInfoImplementation::content::ClimbableLedgeComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d28010, "content_141d28010", fn_141d28010 as *const ()), // content::ClimbableLedgeComponent::ClassTypeInfoImplementation::content::ClimbableLedgeComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d281e0, "content_141d281e0", tpl_39339::<0x8, 0xc, 0xd> as *const ()), // content::ClimbableLedgeComponent::ClassTypeInfoImplementation::content::ClimbableLedgeComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d287e0, "content_141d287e0", fn_141d287e0 as *const ()), // content::CaseBoardCabinetDrawerComponent::ClassTypeInfoImplementation::content::CaseBoardCabinetDrawerComponent::ClassTy
    Replacement::draft(0x141d28860, "content_141d28860", fn_141d28860 as *const ()), // content::CaseBoardCabinetDrawerComponent::ClassTypeInfoImplementation::content::CaseBoardCabinetDrawerComponent::ClassTy
    Replacement::draft(0x141d289e0, "content_141d289e0", tpl_39138::<0x1432b6d98, 0x1432b4ea0, 0x1406db220> as *const ()), // content::CaseBoardCabinetComponent::ClassTypeInfoImplementation::content::CaseBoardCabinetComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d28ea0, "content_141d28ea0", tpl_38525::<0x1431d6388, 0x8, 0x10, 0x18, 0x20> as *const ()), // content::CaseBoardCabinetComponent::ClassTypeInfoImplementation::content::CaseBoardCabinetComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d29ba0, "content_141d29ba0", tpl_39363::<0x8, 0x14028cf90, 0x18, 0x1c> as *const ()), // content::CameraAttachComponent::ClassTypeInfoImplementation::content::CameraAttachComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d2a7f0, "content_141d2a7f0", fn_141d2a7f0 as *const ()), // content::BoardComponent::ClassTypeInfoImplementation::content::BoardComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d2aa60, "content_141d2aa60", tpl_39456::<0x143875470, 0x1432b5250, 0x1432b51b8, 0x1432b5190, 0x1401de410> as *const ()), // content::AvatarSwapperComponent::ClassTypeInfoImplementation::content::AvatarSwapperComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d2aae0, "content_141d2aae0", tpl_39456::<0x143875598, 0x1432b5250, 0x1432b51b8, 0x1432b5190, 0x1401de410> as *const ()), // content::AvatarSwapperComponent::ClassTypeInfoImplementation::content::AvatarSwapperComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d2ab60, "content_141d2ab60", tpl_39456::<0x1438772d0, 0x1432b5250, 0x1432b51b8, 0x1432b5190, 0x1401de410> as *const ()), // content::AvatarSwapperComponent::ClassTypeInfoImplementation::content::AvatarSwapperComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d2af90, "content_141d2af90", fn_141d2af90 as *const ()), // content::AvatarSwapperComponent::ClassTypeInfoImplementation::content::AvatarSwapperComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d2b030, "content_141d2b030", fn_141d2b030 as *const ()), // content::AvatarSwapperComponent::ClassTypeInfoImplementation::content::AvatarSwapperComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d2b740, "content_141d2b740", tpl_38874::<0x8, 0x140c09090, 0x18, 0x14240aa00> as *const ()), // content::AvatarManagerSingletonComponent::ClassTypeInfoImplementation::content::AvatarManagerSingletonComponent::ClassTy
    Replacement::draft(0x141d2b7b0, "content_141d2b7b0", tpl_39459::<0x1432c48f0, 0x8, 0x10, 0x18, 0x1c, 0x20, 0x1b> as *const ()), // content::AvatarManagerSingletonComponent::ClassTypeInfoImplementation::content::AvatarManagerSingletonComponent::ClassTy
    Replacement::draft(0x141d2bd80, "content_141d2bd80", fn_141d2bd80 as *const ()), // content::AutoAimComponent::ClassTypeInfoImplementation::content::AutoAimComponent::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d2be20, "content_141d2be20", fn_141d2be20 as *const ()), // content::AutoAimComponent::ClassTypeInfoImplementation::content::AutoAimComponent::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141d2c940, "content_141d2c940", fn_141d2c940 as *const ()), // content::AITicketScoringComponent::ClassTypeInfoImplementation::content::AITicketScoringComponent::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141d2cd90, "content_141d2cd90", tpl_838::<0x140b395c0> as *const (), Throws::Discard), // content::AIStatusComponent::content::AIStatusComponent::vfunc_34
    Replacement::draft(0x141d2d3d0, "content_141d2d3d0", tpl_38518::<0x14384ea10, 0x1432b5510, 0x1426cdcb0, 0x1432b5528, 0x1401de410> as *const ()), // content::AIInteractableObjectComponent::ClassTypeInfoImplementation::content::AIInteractableObjectComponent::ClassTypeIn
    Replacement::draft(0x141d2d450, "content_141d2d450", tpl_38518::<0x14384ea10, 0x1432b5510, 0x1426cdcb0, 0x1432b5528, 0x1401de410> as *const ()), // content::AIInteractableObjectComponent::ClassTypeInfoImplementation::content::AIInteractableObjectComponent::ClassTypeIn
    Replacement::draft(0x141d2d4d0, "content_141d2d4d0", tpl_38518::<0x14384ea10, 0x1432b5510, 0x1426cdcb0, 0x1432b5528, 0x1401de410> as *const ()), // content::AIInteractableObjectComponent::ClassTypeInfoImplementation::content::AIInteractableObjectComponent::ClassTypeIn
    Replacement::draft(0x141d2e0b0, "content_141d2e0b0", tpl_39463::<0x143877078, 0x1432b5630, 0x1432b59e8, 0x1432b59c0, 0x1401de410> as *const ()), // content::AdhesionTargetComponent::ClassTypeInfoImplementation::content::AdhesionTargetComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d2e170, "content_141d2e170", tpl_39463::<0x1438773b0, 0x1432b5630, 0x1432b59e8, 0x1432b59c0, 0x1401de410> as *const ()), // content::AdhesionTargetComponent::ClassTypeInfoImplementation::content::AdhesionTargetComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d2e5a0, "content_141d2e5a0", fn_141d2e5a0 as *const ()), // content::AdhesionTargetComponent::ClassTypeInfoImplementation::content::AdhesionTargetComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d2e610, "content_141d2e610", fn_141d2e610 as *const ()), // content::AdhesionTargetComponent::ClassTypeInfoImplementation::content::AdhesionTargetComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d2ea20, "content_141d2ea20", tpl_39348::<0x8, 0xc, 0x10> as *const ()), // content::AbilitySprintComponent::ClassTypeInfoImplementation::content::AbilitySprintComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d2f320, "content_141d2f320", fn_141d2f320 as *const ()), // content::AbilityPropertiesComponent::ClassTypeInfoImplementation::content::AbilityPropertiesComponent::ClassTypeInfoImpl
    Replacement::draft(0x141d2f3a0, "content_141d2f3a0", tpl_39459::<0x1432c7c88, 0x10, 0x18, 0x20, 0x24, 0x28, 0x23> as *const ()), // content::AbilityPropertiesComponent::ClassTypeInfoImplementation::content::AbilityPropertiesComponent::ClassTypeInfoImpl
    Replacement::draft(0x141d2ff90, "content_141d2ff90", tpl_38695::<0x8> as *const ()), // content::AbilityInputLogicComponent::ClassTypeInfoImplementation::content::AbilityInputLogicComponent::ClassTypeInfoImpl
    Replacement::draft(0x141d30140, "content_141d30140", fn_141d30140 as *const ()), // content::AbilityDodgeComponent::ClassTypeInfoImplementation::content::AbilityDodgeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d305c0, "content_141d305c0", fn_141d305c0 as *const ()), // content::AbilityDodgeComponent::ClassTypeInfoImplementation::content::AbilityDodgeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d30690, "content_141d30690", fn_141d30690 as *const ()), // content::AbilityDodgeComponent::ClassTypeInfoImplementation::content::AbilityDodgeComponent::ClassTypeInfoImplementation
    Replacement::draft(0x141d30830, "content_141d30830", fn_141d30830 as *const ()), // content::AbilityCrouchComponent::ClassTypeInfoImplementation::content::AbilityCrouchComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d30c90, "content_141d30c90", tpl_38695::<0x8> as *const ()), // content::AbilityCrouchComponent::ClassTypeInfoImplementation::content::AbilityCrouchComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d31550, "content_141d31550", tpl_39471::<0x143877338, 0x1432b41e0, 0x1432b4218, 0x1432b41f8, 0x1401de410> as *const ()), // content::AbilityAITidalWaveComponent::ClassTypeInfoImplementation::content::AbilityAITidalWaveComponent::ClassTypeInfoIm
    Replacement::draft(0x141d31610, "content_141d31610", tpl_39471::<0x1438771d0, 0x1432b41e0, 0x1432b4218, 0x1432b41f8, 0x1401de410> as *const ()), // content::AbilityAITidalWaveComponent::ClassTypeInfoImplementation::content::AbilityAITidalWaveComponent::ClassTypeInfoIm
    Replacement::draft(0x141d31890, "content_141d31890", tpl_39031::<0x8, 0xc, 0x10, 0x14, 0x20, 0x24, 0x18, 0x1c, 0x28> as *const ()), // content::AbilityAITidalWaveComponent::ClassTypeInfoImplementation::content::AbilityAITidalWaveComponent::ClassTypeInfoIm
    Replacement::draft(0x141d31ad0, "content_141d31ad0", tpl_39032::<0x8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x24, 0x28> as *const ()), // content::AbilityAITidalWaveComponent::ClassTypeInfoImplementation::content::AbilityAITidalWaveComponent::ClassTypeInfoIm
    Replacement::draft(0x141d31b50, "content_141d31b50", tpl_38723::<0x1431fb438, 0x8, 0x28> as *const ()), // content::AbilityAITidalWaveComponent::ClassTypeInfoImplementation::content::AbilityAITidalWaveComponent::ClassTypeInfoIm
    Replacement::draft(0x141d32100, "content_141d32100", fn_141d32100 as *const ()), // content::AbilityAIThrowWeaponComponent::ClassTypeInfoImplementation::content::AbilityAIThrowWeaponComponent::ClassTypeIn
    Replacement::draft(0x141d321d0, "content_141d321d0", fn_141d321d0 as *const ()), // content::AbilityAIThrowWeaponComponent::ClassTypeInfoImplementation::content::AbilityAIThrowWeaponComponent::ClassTypeIn
    Replacement::draft(0x141d32850, "content_141d32850", fn_141d32850 as *const ()), // content::AbilityAIMeleeComponent::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d328e0, "content_141d328e0", fn_141d328e0 as *const ()), // content::AbilityAIMeleeComponent::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::ClassTypeInfoImplementa
    Replacement::draft(0x141d32d40, "content_141d32d40", fn_141d32d40 as *const ()), // content::AbilityAILeapComponent::ClassTypeInfoImplementation::content::AbilityAILeapComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d32f30, "content_141d32f30", fn_141d32f30 as *const ()), // content::AbilityAILeapComponent::ClassTypeInfoImplementation::content::AbilityAILeapComponent::ClassTypeInfoImplementati
    Replacement::draft(0x141d33550, "content_141d33550", fn_141d33550 as *const ()), // content::AbilityAIGrappleComponent::ClassTypeInfoImplementation::content::AbilityAIGrappleComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d33670, "content_141d33670", fn_141d33670 as *const ()), // content::AbilityAIGrappleComponent::ClassTypeInfoImplementation::content::AbilityAIGrappleComponent::ClassTypeInfoImplem
    Replacement::draft(0x141d33ac0, "content_141d33ac0", fn_141d33ac0 as *const ()), // content::AbilityAIGrabAndThrowComponent::ClassTypeInfoImplementation::content::AbilityAIGrabAndThrowComponent::ClassType
    Replacement::draft(0x141d33cc0, "content_141d33cc0", tpl_38530::<0x8, 0x10, 0x18, 0x20, 0x28> as *const ()), // content::AbilityAIGrabAndThrowComponent::ClassTypeInfoImplementation::content::AbilityAIGrabAndThrowComponent::ClassType
    Replacement::draft(0x141d34130, "content_141d34130", tpl_39172::<0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // content::AbilityAIGlitchComponent::ClassTypeInfoImplementation::content::AbilityAIGlitchComponent::ClassTypeInfoImplemen
    Replacement::draft(0x141d34540, "content_141d34540", tpl_39171::<0x8, 0xc, 0x10, 0x14, 0x18> as *const ()), // content::AbilityAIGlitchComponent::content::AbilityAIGlitchComponent::vfunc_33
    Replacement::draft(0x141d346c0, "content_141d346c0", fn_141d346c0 as *const ()), // content::AbilityAIGrabAndThrowComponent::content::AbilityAIGrabAndThrowComponent::vfunc_33
    Replacement::draft_throws(0x141d34730, "content_141d34730", tpl_838::<0x140b1b8f0> as *const (), Throws::Discard), // content::AbilityAIGrabAndThrowComponent::content::AbilityAIGrabAndThrowComponent::vfunc_32
    Replacement::draft(0x141d34ac0, "content_141d34ac0", fn_141d34ac0 as *const ()), // content::AbilityAILeapComponent::content::AbilityAILeapComponent::vfunc_33
    Replacement::draft(0x141d34dd0, "content_141d34dd0", tpl_39483::<0x8, 0xc, 0x10> as *const ()), // content::TidalWaveTargetComponent::content::TidalWaveTargetComponent::vfunc_33
    Replacement::draft(0x141d34f80, "content_141d34f80", tpl_39030::<0x8, 0xc, 0x10, 0x14, 0x20, 0x24, 0x18, 0x1c, 0x28> as *const ()), // content::AbilityAITidalWaveComponent::content::AbilityAITidalWaveComponent::vfunc_33
    Replacement::draft_throws(0x141d35040, "content_141d35040", tpl_838::<0x140b29580> as *const (), Throws::Discard), // content::AbilityAITidalWaveComponent::content::AbilityAITidalWaveComponent::vfunc_31
    Replacement::draft(0x141d350b0, "content_141d350b0", tpl_39484::<0x1432b41e0, 0x28, 0x1438771d0, 0x1432b4218, 0x1432b41f8, 0x1401de410> as *const ()), // content::AbilityAITidalWaveComponent::content::AbilityAITidalWaveComponent::vfunc_24
    Replacement::draft(0x141d35120, "content_141d35120", tpl_39484::<0x1432b41e0, 0x28, 0x143877338, 0x1432b4218, 0x1432b41f8, 0x1401de410> as *const ()), // content::AbilityAITidalWaveComponent::content::AbilityAITidalWaveComponent::vfunc_22
    Replacement::draft(0x141d35250, "content_141d35250", tpl_365::<0x140b2c510> as *const ()), // content::AbilityCrouchComponent::content::AbilityCrouchComponent::vfunc_28
    Replacement::draft(0x141d352a0, "content_141d352a0", fn_141d352a0 as *const ()), // content::AbilityCrouchComponent::content::AbilityCrouchComponent::vfunc_23
    Replacement::draft(0x141d355b0, "content_141d355b0", tpl_38526::<0x1432b5510, 0x8, 0x14384ea10, 0x1426cdcb0, 0x1432b5528, 0x1401de410> as *const ()), // content::AIInteractableObjectComponent::content::AIInteractableObjectComponent::vfunc_24
    Replacement::draft(0x141d35610, "content_141d35610", tpl_38526::<0x1432b5510, 0x8, 0x14384ea10, 0x1426cdcb0, 0x1432b5528, 0x1401de410> as *const ()), // content::AIInteractableObjectComponent::content::AIInteractableObjectComponent::vfunc_23
    Replacement::draft(0x141d35670, "content_141d35670", tpl_38526::<0x1432b5510, 0x8, 0x14384ea10, 0x1426cdcb0, 0x1432b5528, 0x1401de410> as *const ()), // content::AIInteractableObjectComponent::content::AIInteractableObjectComponent::vfunc_22
    Replacement::draft(0x141d35870, "content_141d35870", fn_141d35870 as *const ()), // content::AITicketScoringComponent::content::AITicketScoringComponent::vfunc_33
    Replacement::draft(0x141d35cf0, "content_141d35cf0", tpl_38683::<0x1432b4ea0, 0x58, 0x1, 0x1406db220> as *const ()), // content::CameraPanningVolumeComponent::content::CameraPanningVolumeComponent::vfunc_22
    Replacement::draft(0x141d35e60, "content_141d35e60", tpl_39136::<0x8, 0x1432b6d98, 0x1432b4ea0, 0x1406db220> as *const ()), // content::CaseBoardCabinetComponent::content::CaseBoardCabinetComponent::vfunc_22
    Replacement::draft_throws(0x141d36290, "content_141d36290", tpl_838::<0x140b49770> as *const (), Throws::Discard), // content::ClimbableLedgeComponent::content::ClimbableLedgeComponent::vfunc_34
    Replacement::draft(0x141d362a0, "content_141d362a0", fn_141d362a0 as *const ()), // content::ClimbableLedgeComponent::content::ClimbableLedgeComponent::vfunc_33
    Replacement::draft(0x141d36370, "content_141d36370", tpl_39209::<0x1434269f8, 0x1432b6ad8, 0x8, 0x1432b6b00, 0x1432b6ae8, 0x14020f270> as *const ()), // content::ClimbableLedgeComponent::content::ClimbableLedgeComponent::vfunc_23
    Replacement::draft(0x141d368f0, "content_141d368f0", tpl_365::<0x140b4bf00> as *const ()), // content::CorpseHidingComponent::content::CorpseHidingComponent::vfunc_38
    Replacement::draft(0x141d373b0, "content_141d373b0", fn_141d373b0 as *const ()), // content::AbilityDodgeComponent::content::AbilityDodgeComponent::vfunc_22
    Replacement::draft_throws(0x141d37f10, "content_141d37f10", tpl_838::<0x140b6f680> as *const (), Throws::Discard), // content::GameArmoryComponent::content::GameArmoryComponent::vfunc_36
    Replacement::draft(0x141d38120, "content_141d38120", fn_141d38120 as *const ()), // content::GameProjectileDamageActionComponent::content::GameProjectileDamageActionComponent::vfunc_22
    Replacement::draft(0x141d39400, "content_141d39400", fn_141d39400 as *const ()), // content::LightReactionComponent::content::LightReactionComponent::vfunc_33
    Replacement::draft(0x141d39990, "content_141d39990", fn_141d39990 as *const ()), // content::GameCursorInteractableComponent::content::GameCursorInteractableComponent::vfunc_33
    Replacement::draft_throws(0x141d39b80, "content_141d39b80", tpl_838::<0x140b75f80> as *const (), Throws::Discard), // content::GameHealthComponent::content::GameHealthComponent::vfunc_25
    Replacement::draft(0x141d39c30, "content_141d39c30", fn_141d39c30 as *const ()), // content::FlashlightControllerComponent::content::FlashlightControllerComponent::vfunc_33
    Replacement::draft(0x141d3a110, "content_141d3a110", tpl_39136::<0x8, 0x1432bd068, 0x1432bd058, 0x140c05f40> as *const ()), // content::NoiseComponent::content::NoiseComponent::vfunc_24
    Replacement::draft(0x141d3a160, "content_141d3a160", tpl_39136::<0x8, 0x1432bd068, 0x1432bd058, 0x140c05f70> as *const ()), // content::NoiseComponent::content::NoiseComponent::vfunc_22
    Replacement::draft(0x141d3a210, "content_141d3a210", fn_141d3a210 as *const ()), // content::NPCScheduleComponent::content::NPCScheduleComponent::vfunc_33
    Replacement::draft(0x141d3a330, "content_141d3a330", tpl_38546::<0x1432bd118, 0x10, 0x1432bd148, 0x1432bd130, 0x140551e90> as *const ()), // content::NPCScheduleComponent::content::NPCScheduleComponent::vfunc_22
    Replacement::draft(0x141d3a660, "content_141d3a660", fn_141d3a660 as *const ()), // content::MapOverlayDefinition::content::MapOverlayDefinition::vfunc_33
    Replacement::draft_throws(0x141d3a750, "content_141d3a750", tpl_39248::<0x1432bb57c, 0x98, 0x1432bb858, 0x1432bb588, 0x1402c61c0> as *const (), Throws::Discard), // content::MapOverlayDefinition::content::MapOverlayDefinition::vfunc_23
    Replacement::draft(0x141d3a820, "content_141d3a820", fn_141d3a820 as *const ()), // content::MapRoomDefinition::content::MapRoomDefinition::vfunc_33
    Replacement::draft(0x141d3a9a0, "content_141d3a9a0", fn_141d3a9a0 as *const ()), // content::MapBoundsPolygonDefinition::content::MapBoundsPolygonDefinition::vfunc_33
    Replacement::draft(0x141d3abb0, "content_141d3abb0", tpl_39397::<0x1432b5630, 0xc, 0x1438773b0, 0x1432b59e8, 0x1432b59c0, 0x1401de410> as *const ()), // content::AdhesionTargetComponent::content::AdhesionTargetComponent::vfunc_24
    Replacement::draft(0x141d3ac20, "content_141d3ac20", tpl_39397::<0x1432b5630, 0xc, 0x143877078, 0x1432b59e8, 0x1432b59c0, 0x1401de410> as *const ()), // content::AdhesionTargetComponent::content::AdhesionTargetComponent::vfunc_22
    Replacement::draft(0x141d3af50, "content_141d3af50", fn_141d3af50 as *const ()), // content::GameAnimationInputDamageComponent::content::GameAnimationInputDamageComponent::vfunc_33
    Replacement::draft(0x141d3b1d0, "content_141d3b1d0", fn_141d3b1d0 as *const ()), // content::PlayerDeathComponent::content::PlayerDeathComponent::vfunc_33
    Replacement::draft(0x141d3bca0, "content_141d3bca0", tpl_38526::<0x1432be4d0, 0x8, 0x14384e930, 0x1432be3e0, 0x1432be4e8, 0x1401de410> as *const ()), // content::ProhibitFactionZoneComponent::content::ProhibitFactionZoneComponent::vfunc_24
    Replacement::draft(0x141d3bd00, "content_141d3bd00", tpl_38526::<0x1432be4d0, 0x8, 0x14384e930, 0x1432be3e0, 0x1432be4e8, 0x1401de410> as *const ()), // content::ProhibitFactionZoneComponent::content::ProhibitFactionZoneComponent::vfunc_23
    Replacement::draft(0x141d3bd60, "content_141d3bd60", tpl_38526::<0x1432be4d0, 0x8, 0x14384e930, 0x1432be3e0, 0x1432be4e8, 0x1401de410> as *const ()), // content::ProhibitFactionZoneComponent::content::ProhibitFactionZoneComponent::vfunc_22
    Replacement::draft(0x141d3be30, "content_141d3be30", tpl_38542::<0x8, 0xc, 0x10> as *const ()), // content::HolsterableComponent::content::HolsterableComponent::vfunc_33
    Replacement::draft(0x141d3bef0, "content_141d3bef0", tpl_38526::<0x1432bc178, 0xc, 0x143876230, 0x1432bc500, 0x1432bc188, 0x1401de410> as *const ()), // content::HolsterableComponent::content::HolsterableComponent::vfunc_24
    Replacement::draft(0x141d3bf50, "content_141d3bf50", tpl_38526::<0x1432bc178, 0xc, 0x143876230, 0x1432bc500, 0x1432bc188, 0x1401de410> as *const ()), // content::HolsterableComponent::content::HolsterableComponent::vfunc_23
    Replacement::draft(0x141d3bfb0, "content_141d3bfb0", tpl_38526::<0x1432bc178, 0xc, 0x143876230, 0x1432bc500, 0x1432bc188, 0x1401de410> as *const ()), // content::HolsterableComponent::content::HolsterableComponent::vfunc_22
    Replacement::draft(0x141d3c080, "content_141d3c080", tpl_38549::<0x8, 0xc, 0x14240aa00> as *const ()), // content::QuickEscapeComponent::content::QuickEscapeComponent::vfunc_33
    Replacement::draft(0x141d3c1e0, "content_141d3c1e0", fn_141d3c1e0 as *const ()), // content::PlayerStatusComponent::content::PlayerStatusComponent::vfunc_33
    Replacement::draft(0x141d3c400, "content_141d3c400", tpl_39500::<0x143264170, 0xc, 0x1438753f8, 0x1426cdcb0, 0x1432b6030, 0x1401de410, 0x3c, 0x143875488, 0x1432b7778, 0x1432b7648, 0x1432b7788> as *const ()), // content::DoorComponent::content::DoorComponent::vfunc_24
    Replacement::draft(0x141d3c4b0, "content_141d3c4b0", tpl_39500::<0x143264170, 0xc, 0x143876318, 0x1426cdcb0, 0x1432b6030, 0x1401de410, 0x3c, 0x143874480, 0x1432b7778, 0x1432b7648, 0x1432b7788> as *const ()), // content::DoorComponent::content::DoorComponent::vfunc_22
    Replacement::draft(0x141d3c650, "content_141d3c650", tpl_39484::<0x1432bdf58, 0x18, 0x14384ea70, 0x1432bdf88, 0x1432bdf68, 0x1401de410> as *const ()), // content::SignageComponent::content::SignageComponent::vfunc_24
    Replacement::draft(0x141d3c6b0, "content_141d3c6b0", tpl_39484::<0x1432bdf58, 0x18, 0x14384ea70, 0x1432bdf88, 0x1432bdf68, 0x1401de410> as *const ()), // content::SignageComponent::content::SignageComponent::vfunc_23
    Replacement::draft(0x141d3c710, "content_141d3c710", tpl_39484::<0x1432bdf58, 0x18, 0x14384ea70, 0x1432bdf88, 0x1432bdf68, 0x1401de410> as *const ()), // content::SignageComponent::content::SignageComponent::vfunc_22
    Replacement::draft(0x141d3c910, "content_141d3c910", tpl_39483::<0x8, 0xc, 0x10> as *const ()), // content::AbilitySprintComponent::content::AbilitySprintComponent::vfunc_33
    Replacement::draft(0x141d3d010, "content_141d3d010", tpl_39501::<0x1432b8b80, 0x8, 0x143874658, 0x1432b8bb0, 0x1432b8b90, 0x1401de410> as *const ()), // content::FadeoutComponent::content::FadeoutComponent::vfunc_24
    Replacement::draft(0x141d3d070, "content_141d3d070", tpl_39501::<0x1432b8b80, 0x8, 0x143877220, 0x1432b8bb0, 0x1432b8b90, 0x1401de410> as *const ()), // content::FadeoutComponent::content::FadeoutComponent::vfunc_23
    Replacement::draft(0x141d3d0d0, "content_141d3d0d0", tpl_39501::<0x1432b8b80, 0x8, 0x143876380, 0x1432b8bb0, 0x1432b8b90, 0x1401de410> as *const ()), // content::FadeoutComponent::content::FadeoutComponent::vfunc_22
    Replacement::draft(0x141d3d380, "content_141d3d380", tpl_39502::<0x143264170, 0x38, 0x1438762d8, 0x1432b9920, 0x1432b9910, 0x1401de410> as *const ()), // content::GameAimTargetComponent::content::GameAimTargetComponent::vfunc_24
    Replacement::draft(0x141d3d3f0, "content_141d3d3f0", tpl_39502::<0x143264170, 0x38, 0x1438745e0, 0x1432b9920, 0x1432b9910, 0x1401de410> as *const ()), // content::GameAimTargetComponent::content::GameAimTargetComponent::vfunc_22
    Replacement::draft_throws(0x141d3d490, "content_141d3d490", tpl_38829::<0x14387ef90, 0x14387eda0, 0x1401e9fa0, 0x1406e02b0, 0x1401e9e70> as *const (), Throws::Discard), // content::EquippableComponent::content::EquippableComponent::vfunc_36
    Replacement::draft_throws(0x141d3d500, "content_141d3d500", tpl_38829::<0x14387ef90, 0x14387eda0, 0x1401e9b60, 0x1406e03a0, 0x1401e9a20> as *const (), Throws::Discard), // content::EquippableComponent::content::EquippableComponent::vfunc_34
    Replacement::draft(0x141d3d5b0, "content_141d3d5b0", tpl_38830::<0x14387ef90, 0x14387eda0, 0x1401e9970, 0x1406e1480, 0x1401e9860> as *const ()), // content::EquippableComponent::content::EquippableComponent::vfunc_28
    Replacement::draft_throws(0x141d3d620, "content_141d3d620", tpl_38829::<0x14387ef90, 0x14387eda0, 0x1401e9770, 0x1406e1570, 0x1401e9590> as *const (), Throws::Discard), // content::EquippableComponent::content::EquippableComponent::vfunc_26
    Replacement::draft(0x141d3d7c0, "content_141d3d7c0", tpl_38551::<0x1432b5250, 0x11c, 0x1438772d0, 0x1432b51b8, 0x1432b5190, 0x1401de410> as *const ()), // content::AvatarSwapperComponent::content::AvatarSwapperComponent::vfunc_24
    Replacement::draft(0x141d3d820, "content_141d3d820", tpl_38551::<0x1432b5250, 0x11c, 0x143875598, 0x1432b51b8, 0x1432b5190, 0x1401de410> as *const ()), // content::AvatarSwapperComponent::content::AvatarSwapperComponent::vfunc_23
    Replacement::draft(0x141d3d880, "content_141d3d880", tpl_38551::<0x1432b5250, 0x11c, 0x143875470, 0x1432b51b8, 0x1432b5190, 0x1401de410> as *const ()), // content::AvatarSwapperComponent::content::AvatarSwapperComponent::vfunc_22
    Replacement::draft(0x141d3dd90, "content_141d3dd90", tpl_38526::<0x1432b8b50, 0x8, 0x143874730, 0x1432b8af0, 0x1432b8b70, 0x1401de410> as *const ()), // content::EquippableAnimationIDComponent::content::EquippableAnimationIDComponent::vfunc_24
    Replacement::draft(0x141d3ddf0, "content_141d3ddf0", tpl_38526::<0x1432b8b50, 0x8, 0x1438763a0, 0x1432b8af0, 0x1432b8b70, 0x1401de410> as *const ()), // content::EquippableAnimationIDComponent::content::EquippableAnimationIDComponent::vfunc_23
    Replacement::draft(0x141d3de50, "content_141d3de50", tpl_38526::<0x1432b8b50, 0x8, 0x1438755b0, 0x1432b8af0, 0x1432b8b70, 0x1401de410> as *const ()), // content::EquippableAnimationIDComponent::content::EquippableAnimationIDComponent::vfunc_22
    Replacement::draft(0x141d3df20, "content_141d3df20", fn_141d3df20 as *const ()), // content::GameRayCastDamageActionComponent::content::GameRayCastDamageActionComponent::vfunc_33
    Replacement::draft(0x141d3e020, "content_141d3e020", tpl_38548::<0x8, 0x1432bb528, 0x1432bb518, 0x140947eb0> as *const ()), // content::GameRayCastDamageActionComponent::content::GameRayCastDamageActionComponent::vfunc_22
    Replacement::draft(0x141d3e170, "content_141d3e170", fn_141d3e170 as *const ()), // content::GameWeaponComponent::content::GameWeaponComponent::vfunc_23
    Replacement::draft(0x141d3e330, "content_141d3e330", fn_141d3e330 as *const ()), // content::ProjectileComponent::content::ProjectileComponent::vfunc_23
    Replacement::draft_throws(0x141d3e4c0, "content_141d3e4c0", tpl_838::<0x140b87f20> as *const (), Throws::Discard), // content::GameWeaponDispersionComponent::content::GameWeaponDispersionComponent::vfunc_25
    Replacement::draft(0x141d3e6a0, "content_141d3e6a0", fn_141d3e6a0 as *const ()), // content::WeaponInputLogicComponent::content::WeaponInputLogicComponent::vfunc_33
    Replacement::draft(0x141d3f200, "content_141d3f200", fn_141d3f200 as *const ()), // content::Outfit::ClassTypeInfoImplementation::content::Outfit::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d3f2b0, "content_141d3f2b0", tpl_39306::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401de830, 0x142442c80> as *const ()), // content::Outfit::ClassTypeInfoImplementation::content::Outfit::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141d3f310, "content_141d3f310", tpl_35191::<0x10, 0x1401de830, 0x142442de0> as *const ()), // content::Outfit::ClassTypeInfoImplementation::content::Outfit::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141d3f810, "content_141d3f810", tpl_39508::<0x14240aa00, 0x48, 0x4c, 0x50, 0x54> as *const ()), // content::AimBone::ClassTypeInfoImplementation::content::AimBone::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d3fd80, "content_141d3fd80", tpl_35199::<0x14387f2f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradePermutationSetup::ClassTypeInfoImplementation::content::WeaponUpgradesCom
    Replacement::draft(0x141d3fde0, "content_141d3fde0", tpl_35199::<0x14387f2f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradePermutationSetup::ClassTypeInfoImplementation::content::WeaponUpgradesCom
    Replacement::draft(0x141d3fe70, "content_141d3fe70", tpl_38770::<0x140225870, 0x10, 0x140225740> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradePermutationSetup::ClassTypeInfoImplementation::content::WeaponUpgradesCom
    Replacement::draft(0x141d3fec0, "content_141d3fec0", tpl_39509::<0x10, 0x1401f2bc0, 0x1402183c0> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradePermutationSetup::ClassTypeInfoImplementation::content::WeaponUpgradesCom
    Replacement::draft(0x141d3ff10, "content_141d3ff10", tpl_39510::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x10, 0x1401f2bc0, 0x1402183c0, 0x142442c80> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradePermutationSetup::ClassTypeInfoImplementation::content::WeaponUpgradesCom
    Replacement::draft(0x141d3ff80, "content_141d3ff80", tpl_38774::<0x10, 0x1401f2bc0, 0x1402183c0, 0x142442de0> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradePermutationSetup::ClassTypeInfoImplementation::content::WeaponUpgradesCom
    Replacement::draft(0x141d402c0, "content_141d402c0", tpl_39511::<0x8, 0xc, 0x14240aa00> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeMaterialSetSwap::ClassTypeInfoImplementation::content::WeaponUpgradesComp
    Replacement::draft(0x141d40310, "content_141d40310", tpl_39511::<0x8, 0xc, 0x14240aa00> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeMaterialSetSwap::ClassTypeInfoImplementation::content::WeaponUpgradesComp
    Replacement::draft(0x141d40460, "content_141d40460", tpl_35199::<0x14387f378, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeMaterialSetSwap::ClassTypeInfoImplementation::content::WeaponUpgradesComp
    Replacement::draft(0x141d404c0, "content_141d404c0", tpl_35199::<0x14387f378, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeMaterialSetSwap::ClassTypeInfoImplementation::content::WeaponUpgradesComp
    Replacement::draft(0x141d40550, "content_141d40550", fn_141d40550 as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeMaterialSetSwap::ClassTypeInfoImplementation::content::WeaponUpgradesComp
    Replacement::draft(0x141d405a0, "content_141d405a0", fn_141d405a0 as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeMaterialSetSwap::ClassTypeInfoImplementation::content::WeaponUpgradesComp
    Replacement::draft(0x141d405d0, "content_141d405d0", tpl_94938::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x14240ab10, 0x142442c80> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeMaterialSetSwap::ClassTypeInfoImplementation::content::WeaponUpgradesComp
    Replacement::draft(0x141d40630, "content_141d40630", tpl_35184::<0x10, 0x14240ab10, 0x142442de0> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeMaterialSetSwap::ClassTypeInfoImplementation::content::WeaponUpgradesComp
    Replacement::draft(0x141d40b10, "content_141d40b10", fn_141d40b10 as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeInfo::ClassTypeInfoImplementation::content::WeaponUpgradesComponent::Weap
    Replacement::draft(0x141d40b70, "content_141d40b70", tpl_39515::<0x18, 0x14240ab10, 0x8> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeInfo::ClassTypeInfoImplementation::content::WeaponUpgradesComponent::Weap
    Replacement::draft(0x141d40ba0, "content_141d40ba0", tpl_39516::<0x8, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x1b> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeInfo::ClassTypeInfoImplementation::content::WeaponUpgradesComponent::Weap
    Replacement::draft(0x141d40bd0, "content_141d40bd0", tpl_99271::<0xfffffffffffffff8, 0x18, 0xfffffffffffffff0, 0x14240ab10, 0x8, 0x142442c80> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeInfo::ClassTypeInfoImplementation::content::WeaponUpgradesComponent::Weap
    Replacement::draft(0x141d40c40, "content_141d40c40", tpl_39517::<0x18, 0x14240ab10, 0x8, 0x142442de0> as *const ()), // content::WeaponUpgradesComponent::WeaponUpgradeInfo::ClassTypeInfoImplementation::content::WeaponUpgradesComponent::Weap
    Replacement::draft(0x141d40d10, "content_141d40d10", tpl_39518::<0x1432d79e8, 0x1432d7938, 0x14088ba10> as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d40d50, "content_141d40d50", tpl_39518::<0x1432d79e8, 0x1432d7938, 0x14088ba10> as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d40fd0, "content_141d40fd0", tpl_39519::<0x8, 0x1401e84a0, 0x14240aa00, 0x60> as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d41030, "content_141d41030", tpl_39519::<0x8, 0x1401e84a0, 0x14240aa00, 0x60> as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d41200, "content_141d41200", tpl_39520::<0x14240aa00, 0x48, 0x1401e84a0, 0x60> as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d41250, "content_141d41250", tpl_39521::<0x50, 0x14240ab10> as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d41270, "content_141d41270", fn_141d41270 as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d412a0, "content_141d412a0", tpl_99273::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x50, 0x14240ab10, 0x142442c80> as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d41300, "content_141d41300", tpl_39523::<0x50, 0x14240ab10, 0x142442de0> as *const ()), // content::GameArmoryComponent::WeaponInfo::ClassTypeInfoImplementation::content::GameArmoryComponent::WeaponInfo::ClassTy
    Replacement::draft(0x141d41880, "content_141d41880", tpl_98838::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x140260f30, 0x142442c80> as *const ()), // content::ScatterShotProfile::ClassTypeInfoImplementation::content::ScatterShotProfile::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d418e0, "content_141d418e0", tpl_35184::<0x8, 0x140260f30, 0x142442de0> as *const ()), // content::ScatterShotProfile::ClassTypeInfoImplementation::content::ScatterShotProfile::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d41d90, "content_141d41d90", tpl_35199::<0x14387f370, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ScatterShotProfilePelletInfo::ClassTypeInfoImplementation::content::ScatterShotProfilePelletInfo::ClassTypeInfo
    Replacement::draft(0x141d41df0, "content_141d41df0", tpl_35199::<0x14387f370, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ScatterShotProfilePelletInfo::ClassTypeInfoImplementation::content::ScatterShotProfilePelletInfo::ClassTypeInfo
    Replacement::draft(0x141d41ea0, "content_141d41ea0", tpl_39524 as *const ()), // content::ScatterShotProfilePelletInfo::ClassTypeInfoImplementation::content::ScatterShotProfilePelletInfo::ClassTypeInfo
    Replacement::draft(0x141d427d0, "content_141d427d0", tpl_39525::<0x8, 0x4> as *const ()), // content::ResourcesManagerComponent::InitialAmountInfo::ClassTypeInfoImplementation::content::ResourcesManagerComponent::
    Replacement::draft(0x141d42810, "content_141d42810", tpl_39525::<0x8, 0x4> as *const ()), // content::ResourcesManagerComponent::InitialAmountInfo::ClassTypeInfoImplementation::content::ResourcesManagerComponent::
    Replacement::draft(0x141d429c0, "content_141d429c0", tpl_34638::<0x8> as *const ()), // content::ResourcesManagerComponent::InitialAmountInfo::ClassTypeInfoImplementation::content::ResourcesManagerComponent::
    Replacement::draft(0x141d42dc0, "content_141d42dc0", tpl_39004::<0x8, 0x1401e84a0, 0x18> as *const ()), // content::ProjectilePoolData::ClassTypeInfoImplementation::content::ProjectilePoolData::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d42e10, "content_141d42e10", tpl_39004::<0x8, 0x1401e84a0, 0x18> as *const ()), // content::ProjectilePoolData::ClassTypeInfoImplementation::content::ProjectilePoolData::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d42f60, "content_141d42f60", tpl_35199::<0x14387f2a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ProjectilePoolData::ClassTypeInfoImplementation::content::ProjectilePoolData::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d42fc0, "content_141d42fc0", tpl_35199::<0x14387f2a8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ProjectilePoolData::ClassTypeInfoImplementation::content::ProjectilePoolData::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d43050, "content_141d43050", tpl_39005::<0x1401e84a0, 0x18> as *const ()), // content::ProjectilePoolData::ClassTypeInfoImplementation::content::ProjectilePoolData::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d43090, "content_141d43090", tpl_37083::<0x8> as *const ()), // content::ProjectilePoolData::ClassTypeInfoImplementation::content::ProjectilePoolData::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d430c0, "content_141d430c0", tpl_37085::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x8, 0x142442c80> as *const ()), // content::ProjectilePoolData::ClassTypeInfoImplementation::content::ProjectilePoolData::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d43100, "content_141d43100", tpl_37086::<0x8, 0x142442de0> as *const ()), // content::ProjectilePoolData::ClassTypeInfoImplementation::content::ProjectilePoolData::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d434c0, "content_141d434c0", tpl_39362::<0x8, 0x1401e84a0, 0x20, 0x24> as *const ()), // content::ProjectileGameSpawnPool::ClassTypeInfoImplementation::content::ProjectileGameSpawnPool::ClassTypeInfoImplementa
    Replacement::draft(0x141d43520, "content_141d43520", tpl_39526::<0x8, 0x1401e84a0, 0x20, 0x24> as *const ()), // content::ProjectileGameSpawnPool::ClassTypeInfoImplementation::content::ProjectileGameSpawnPool::ClassTypeInfoImplementa
    Replacement::draft(0x141d43670, "content_141d43670", tpl_35199::<0x14387f268, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ProjectileGameSpawnPool::ClassTypeInfoImplementation::content::ProjectileGameSpawnPool::ClassTypeInfoImplementa
    Replacement::draft(0x141d436d0, "content_141d436d0", tpl_35199::<0x14387f268, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ProjectileGameSpawnPool::ClassTypeInfoImplementation::content::ProjectileGameSpawnPool::ClassTypeInfoImplementa
    Replacement::draft(0x141d43760, "content_141d43760", tpl_39363::<0x8, 0x1401e84a0, 0x20, 0x24> as *const ()), // content::ProjectileGameSpawnPool::ClassTypeInfoImplementation::content::ProjectileGameSpawnPool::ClassTypeInfoImplementa
    Replacement::draft(0x141d43d60, "content_141d43d60", fn_141d43d60 as *const ()), // content::ProjectileBalancedData::ClassTypeInfoImplementation::content::ProjectileBalancedData::ClassTypeInfoImplementati
    Replacement::draft(0x141d43eb0, "content_141d43eb0", tpl_39528::<0x1432b0a48, 0x14329ebe8, 0x1432b0ac0, 0x14088ba10> as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d43f00, "content_141d43f00", tpl_39528::<0x1432b0a48, 0x14329ebe8, 0x1432b0ac0, 0x14088ba10> as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d442f0, "content_141d442f0", tpl_35199::<0x14387f1a0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d44350, "content_141d44350", tpl_35199::<0x14387f1a0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d443e0, "content_141d443e0", fn_141d443e0 as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d44430, "content_141d44430", tpl_37083::<0x20> as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d44440, "content_141d44440", fn_141d44440 as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d44470, "content_141d44470", tpl_37085::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x20, 0x142442c80> as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d444b0, "content_141d444b0", tpl_37086::<0x20, 0x142442de0> as *const ()), // content::PlayerThrowerComponent::ProjectileInfo::ClassTypeInfoImplementation::content::PlayerThrowerComponent::Projectil
    Replacement::draft(0x141d44570, "content_141d44570", tpl_39138::<0x1432d9680, 0x1432d93c8, 0x140c05f70> as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d445b0, "content_141d445b0", tpl_39138::<0x1432d9680, 0x1432d93c8, 0x140c05f70> as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d44670, "content_141d44670", tpl_39138::<0x1432d9680, 0x1432d93c8, 0x140c05f40> as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d446b0, "content_141d446b0", tpl_39138::<0x1432d9680, 0x1432d93c8, 0x140c05f40> as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d44aa0, "content_141d44aa0", fn_141d44aa0 as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d44b00, "content_141d44b00", tpl_38763::<0x30, 0x10> as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d44b20, "content_141d44b20", fn_141d44b20 as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d44b50, "content_141d44b50", tpl_38765::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x30, 0x10, 0x142442c80> as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d44ba0, "content_141d44ba0", tpl_38766::<0x30, 0x10, 0x142442de0> as *const ()), // content::PlayerMovementSystemComponent::MovementNoiseData::ClassTypeInfoImplementation::content::PlayerMovementSystemCom
    Replacement::draft(0x141d44ee0, "content_141d44ee0", tpl_39533::<0x8, 0x4> as *const ()), // content::PerfectDodgeTargetComponent::AttackInfo::ClassTypeInfoImplementation::content::PerfectDodgeTargetComponent::Att
    Replacement::draft(0x141d45060, "content_141d45060", tpl_35199::<0x14387f168, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::PerfectDodgeTargetComponent::AttackInfo::ClassTypeInfoImplementation::content::PerfectDodgeTargetComponent::Att
    Replacement::draft(0x141d450c0, "content_141d450c0", tpl_35199::<0x14387f168, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::PerfectDodgeTargetComponent::AttackInfo::ClassTypeInfoImplementation::content::PerfectDodgeTargetComponent::Att
    Replacement::draft(0x141d45150, "content_141d45150", tpl_34638::<0x8> as *const ()), // content::PerfectDodgeTargetComponent::AttackInfo::ClassTypeInfoImplementation::content::PerfectDodgeTargetComponent::Att
    Replacement::draft(0x141d45500, "content_141d45500", tpl_39534::<0x8, 0xc, 0x4, 0x18> as *const ()), // content::ObjectVolumePropertyGroup::ClassTypeInfoImplementation::content::ObjectVolumePropertyGroup::ClassTypeInfoImplem
    Replacement::draft(0x141d45540, "content_141d45540", tpl_39534::<0x8, 0xc, 0x4, 0x18> as *const ()), // content::ObjectVolumePropertyGroup::ClassTypeInfoImplementation::content::ObjectVolumePropertyGroup::ClassTypeInfoImplem
    Replacement::draft(0x141d456f0, "content_141d456f0", tpl_35239 as *const ()), // content::ObjectVolumePropertyGroup::ClassTypeInfoImplementation::content::ObjectVolumePropertyGroup::ClassTypeInfoImplem
    Replacement::draft(0x141d45730, "content_141d45730", fn_141d45730 as *const ()), // content::ObjectVolumePropertyGroup::ClassTypeInfoImplementation::content::ObjectVolumePropertyGroup::ClassTypeInfoImplem
    Replacement::draft(0x141d45850, "content_141d45850", tpl_39536::<0x143426cf0, 0x1432e26d0, 0x143250778, 0x1432e2704, 0x14020f270> as *const ()), // content::MultilaunchProjectileInfo::ClassTypeInfoImplementation::content::MultilaunchProjectileInfo::ClassTypeInfoImplem
    Replacement::draft(0x141d458b0, "content_141d458b0", tpl_39536::<0x143426cf0, 0x1432e26d0, 0x143250778, 0x1432e2704, 0x14020f270> as *const ()), // content::MultilaunchProjectileInfo::ClassTypeInfoImplementation::content::MultilaunchProjectileInfo::ClassTypeInfoImplem
    Replacement::draft(0x141d45d00, "content_141d45d00", tpl_39537 as *const ()), // content::MultilaunchProjectileInfo::ClassTypeInfoImplementation::content::MultilaunchProjectileInfo::ClassTypeInfoImplem
    Replacement::draft(0x141d462d0, "content_141d462d0", tpl_98838::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x140260f30, 0x142442c80> as *const ()), // content::MultilaunchProfile::ClassTypeInfoImplementation::content::MultilaunchProfile::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d46330, "content_141d46330", tpl_35184::<0x8, 0x140260f30, 0x142442de0> as *const ()), // content::MultilaunchProfile::ClassTypeInfoImplementation::content::MultilaunchProfile::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d46660, "content_141d46660", tpl_39004::<0x8, 0x14240aa00, 0x48> as *const ()), // content::ItemCategory::ClassTypeInfoImplementation::content::ItemCategory::ClassTypeInfoImplementation::vfunc_30
    Replacement::draft(0x141d466b0, "content_141d466b0", tpl_39004::<0x8, 0x14240aa00, 0x48> as *const ()), // content::ItemCategory::ClassTypeInfoImplementation::content::ItemCategory::ClassTypeInfoImplementation::vfunc_47
    Replacement::draft(0x141d46800, "content_141d46800", tpl_35199::<0x14387f458, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ItemCategory::ClassTypeInfoImplementation::content::ItemCategory::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x141d46860, "content_141d46860", tpl_35199::<0x14387f458, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::ItemCategory::ClassTypeInfoImplementation::content::ItemCategory::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x141d468f0, "content_141d468f0", tpl_39005::<0x14240aa00, 0x48> as *const ()), // content::ItemCategory::ClassTypeInfoImplementation::content::ItemCategory::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d47040, "content_141d47040", tpl_39538::<0x1430f4400, 0x9, 0x14020f270> as *const ()), // content::Item::ClassTypeInfoImplementation::content::Item::ClassTypeInfoImplementation::vfunc_20
    Replacement::draft(0x141d47080, "content_141d47080", tpl_39538::<0x1430f4400, 0x9, 0x14020f270> as *const ()), // content::Item::ClassTypeInfoImplementation::content::Item::ClassTypeInfoImplementation::vfunc_37
    Replacement::draft(0x141d47490, "content_141d47490", fn_141d47490 as *const ()), // content::Item::ClassTypeInfoImplementation::content::Item::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d475a0, "content_141d475a0", fn_141d475a0 as *const ()), // content::Item::ClassTypeInfoImplementation::content::Item::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x141d475d0, "content_141d475d0", fn_141d475d0 as *const ()), // content::Item::ClassTypeInfoImplementation::content::Item::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x141d47650, "content_141d47650", fn_141d47650 as *const ()), // content::Item::ClassTypeInfoImplementation::content::Item::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x141d476d0, "content_141d476d0", fn_141d476d0 as *const ()), // content::Item::ClassTypeInfoImplementation::content::Item::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x141d47b80, "content_141d47b80", tpl_35219::<0x14387f320, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::InputMinigameComponent::InputMinigameEntry::ClassTypeInfoImplementation::content::InputMinigameComponent::Input
    Replacement::draft(0x141d47be0, "content_141d47be0", tpl_35219::<0x14387f320, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::InputMinigameComponent::InputMinigameEntry::ClassTypeInfoImplementation::content::InputMinigameComponent::Input
    Replacement::draft(0x141d47c70, "content_141d47c70", fn_141d47c70 as *const ()), // content::InputMinigameComponent::InputMinigameEntry::ClassTypeInfoImplementation::content::InputMinigameComponent::Input
    Replacement::draft(0x141d47d10, "content_141d47d10", tpl_38952::<0x88, 0x14240ab10> as *const ()), // content::InputMinigameComponent::InputMinigameEntry::ClassTypeInfoImplementation::content::InputMinigameComponent::Input
    Replacement::draft(0x141d47d40, "content_141d47d40", fn_141d47d40 as *const ()), // content::InputMinigameComponent::InputMinigameEntry::ClassTypeInfoImplementation::content::InputMinigameComponent::Input
    Replacement::draft(0x141d47d90, "content_141d47d90", tpl_98840::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x88, 0x14240ab10, 0x142442c80> as *const ()), // content::InputMinigameComponent::InputMinigameEntry::ClassTypeInfoImplementation::content::InputMinigameComponent::Input
    Replacement::draft(0x141d47e00, "content_141d47e00", tpl_38954::<0x88, 0x14240ab10, 0x142442de0> as *const ()), // content::InputMinigameComponent::InputMinigameEntry::ClassTypeInfoImplementation::content::InputMinigameComponent::Input
    Replacement::draft(0x141d47ee0, "content_141d47ee0", tpl_39545::<0x143876230, 0x1432bc178, 0x1432d78e8, 0x1432d7910, 0x1401de410> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d47f40, "content_141d47f40", tpl_39545::<0x143876230, 0x1432bc178, 0x1432d78e8, 0x1432d7910, 0x1401de410> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d47fa0, "content_141d47fa0", tpl_39545::<0x143876230, 0x1432bc178, 0x1432d78e8, 0x1432d7910, 0x1401de410> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d48000, "content_141d48000", tpl_39545::<0x143876230, 0x1432bc178, 0x1432d78e8, 0x1432d7910, 0x1401de410> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d48060, "content_141d48060", tpl_39545::<0x143876230, 0x1432bc178, 0x1432d78e8, 0x1432d7910, 0x1401de410> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d480c0, "content_141d480c0", tpl_39545::<0x143876230, 0x1432bc178, 0x1432d78e8, 0x1432d7910, 0x1401de410> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d48320, "content_141d48320", fn_141d48320 as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d48360, "content_141d48360", fn_141d48360 as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d484a0, "content_141d484a0", tpl_39318::<0x14387f258, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d48500, "content_141d48500", tpl_39318::<0x14387f258, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d48590, "content_141d48590", tpl_34638::<0x8> as *const ()), // content::HolsteringComponent::HolsterSlot::ClassTypeInfoImplementation::content::HolsteringComponent::HolsterSlot::Class
    Replacement::draft(0x141d48aa0, "content_141d48aa0", tpl_35199::<0x14387f2b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::HealingSystemComponent::HealingCategory::ClassTypeInfoImplementation::content::HealingSystemComponent::HealingC
    Replacement::draft(0x141d48b00, "content_141d48b00", tpl_35199::<0x14387f2b8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::HealingSystemComponent::HealingCategory::ClassTypeInfoImplementation::content::HealingSystemComponent::HealingC
    Replacement::draft(0x141d48bb0, "content_141d48bb0", tpl_39524 as *const ()), // content::HealingSystemComponent::HealingCategory::ClassTypeInfoImplementation::content::HealingSystemComponent::HealingC
    Replacement::draft(0x141d49100, "content_141d49100", tpl_39548::<0x20, 0x30> as *const ()), // content::GameWeaponKickbackBalancedData::ClassTypeInfoImplementation::content::GameWeaponKickbackBalancedData::ClassType
    Replacement::draft(0x141d49670, "content_141d49670", tpl_39229::<0x20, 0x28> as *const ()), // content::GameWeaponDispersionBalancedData::ClassTypeInfoImplementation::content::GameWeaponDispersionBalancedData::Class
    Replacement::draft(0x141d49a30, "content_141d49a30", tpl_39549::<0x8, 0x4, 0xc, 0x10, 0x14, 0x18> as *const ()), // content::GameVisualSensorComponent::VisionSection::ClassTypeInfoImplementation::content::GameVisualSensorComponent::Visi
    Replacement::draft(0x141d49ac0, "content_141d49ac0", tpl_39006::<0x8, 0x4, 0xc, 0x10, 0x14, 0x18> as *const ()), // content::GameVisualSensorComponent::VisionSection::ClassTypeInfoImplementation::content::GameVisualSensorComponent::Visi
    Replacement::draft(0x141d49c30, "content_141d49c30", tpl_35199::<0x14387f150, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GameVisualSensorComponent::VisionSection::ClassTypeInfoImplementation::content::GameVisualSensorComponent::Visi
    Replacement::draft(0x141d49c90, "content_141d49c90", tpl_35199::<0x14387f150, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GameVisualSensorComponent::VisionSection::ClassTypeInfoImplementation::content::GameVisualSensorComponent::Visi
    Replacement::draft(0x141d49d20, "content_141d49d20", tpl_39007::<0x10, 0x18> as *const ()), // content::GameVisualSensorComponent::VisionSection::ClassTypeInfoImplementation::content::GameVisualSensorComponent::Visi
    Replacement::draft(0x141d49e60, "content_141d49e60", tpl_39545::<0x14384e908, 0x1432d7820, 0x1432d77d0, 0x1432d7810, 0x1401de410> as *const ()), // content::GameVisualSensorComponent::VisionData::ClassTypeInfoImplementation::content::GameVisualSensorComponent::VisionD
    Replacement::draft(0x141d49ec0, "content_141d49ec0", tpl_39545::<0x14384e908, 0x1432d7820, 0x1432d77d0, 0x1432d7810, 0x1401de410> as *const ()), // content::GameVisualSensorComponent::VisionData::ClassTypeInfoImplementation::content::GameVisualSensorComponent::VisionD
    Replacement::draft(0x141d49f60, "content_141d49f60", tpl_39545::<0x14384e908, 0x1432d7820, 0x1432d77d0, 0x1432d7810, 0x1401de410> as *const ()), // content::GameVisualSensorComponent::VisionData::ClassTypeInfoImplementation::content::GameVisualSensorComponent::VisionD
    Replacement::draft(0x141d49fc0, "content_141d49fc0", tpl_39545::<0x14384e908, 0x1432d7820, 0x1432d77d0, 0x1432d7810, 0x1401de410> as *const ()), // content::GameVisualSensorComponent::VisionData::ClassTypeInfoImplementation::content::GameVisualSensorComponent::VisionD
    Replacement::draft(0x141d4a3d0, "content_141d4a3d0", tpl_39550::<0x8, 0x140241850, 0x18, 0x1c> as *const ()), // content::GameVisualSensorComponent::VisionData::ClassTypeInfoImplementation::content::GameVisualSensorComponent::VisionD
    Replacement::draft(0x141d4a460, "content_141d4a460", tpl_35190::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401f2bc0, 0x142442c80> as *const ()), // content::GameVisualSensorComponent::VisionData::ClassTypeInfoImplementation::content::GameVisualSensorComponent::VisionD
    Replacement::draft(0x141d4a4c0, "content_141d4a4c0", tpl_35191::<0x8, 0x1401f2bc0, 0x142442de0> as *const ()), // content::GameVisualSensorComponent::VisionData::ClassTypeInfoImplementation::content::GameVisualSensorComponent::VisionD
    Replacement::draft(0x141d4aaa0, "content_141d4aaa0", tpl_39551::<0x1432d5c40, 0x1432d5c60, 0x14088ba10> as *const ()), // content::GamePlayerCameraMovementComponent::QuickTurn::ClassTypeInfoImplementation::content::GamePlayerCameraMovementCom
    Replacement::draft(0x141d4aae0, "content_141d4aae0", tpl_39551::<0x1432d5c40, 0x1432d5c60, 0x14088ba10> as *const ()), // content::GamePlayerCameraMovementComponent::QuickTurn::ClassTypeInfoImplementation::content::GamePlayerCameraMovementCom
    Replacement::draft(0x141d4af30, "content_141d4af30", tpl_37083::<0x8> as *const ()), // content::GamePlayerCameraMovementComponent::QuickTurn::ClassTypeInfoImplementation::content::GamePlayerCameraMovementCom
    Replacement::draft(0x141d4af60, "content_141d4af60", tpl_37085::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x8, 0x142442c80> as *const ()), // content::GamePlayerCameraMovementComponent::QuickTurn::ClassTypeInfoImplementation::content::GamePlayerCameraMovementCom
    Replacement::draft(0x141d4afa0, "content_141d4afa0", tpl_37086::<0x8, 0x142442de0> as *const ()), // content::GamePlayerCameraMovementComponent::QuickTurn::ClassTypeInfoImplementation::content::GamePlayerCameraMovementCom
    Replacement::draft(0x141d4b490, "content_141d4b490", fn_141d4b490 as *const ()), // content::GamePlayerCameraMovementComponent::FireMode::ClassTypeInfoImplementation::content::GamePlayerCameraMovementComp
    Replacement::draft(0x141d4b4f0, "content_141d4b4f0", fn_141d4b4f0 as *const ()), // content::GamePlayerCameraMovementComponent::FireMode::ClassTypeInfoImplementation::content::GamePlayerCameraMovementComp
    Replacement::draft(0x141d4b570, "content_141d4b570", fn_141d4b570 as *const ()), // content::GamePlayerCameraMovementComponent::FireMode::ClassTypeInfoImplementation::content::GamePlayerCameraMovementComp
    Replacement::draft(0x141d4b8f0, "content_141d4b8f0", tpl_38951::<0x8, 0x1401e84a0, 0x18> as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4b940, "content_141d4b940", tpl_38951::<0x8, 0x1401e84a0, 0x18> as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4ba90, "content_141d4ba90", tpl_35199::<0x14387f228, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4baf0, "content_141d4baf0", tpl_35199::<0x14387f228, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4bb80, "content_141d4bb80", tpl_38937::<0x1401e84a0, 0x18> as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4bbd0, "content_141d4bbd0", tpl_38763::<0x20, 0x8> as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4bbf0, "content_141d4bbf0", fn_141d4bbf0 as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4bc20, "content_141d4bc20", tpl_38765::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x20, 0x8, 0x142442c80> as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4bc70, "content_141d4bc70", tpl_38766::<0x20, 0x8, 0x142442de0> as *const ()), // content::GamePlayerCameraMovementComponent::CrouchCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMovement
    Replacement::draft(0x141d4c0d0, "content_141d4c0d0", tpl_35199::<0x14387f338, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionMoveCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c130, "content_141d4c130", tpl_35199::<0x14387f338, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionMoveCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c1e0, "content_141d4c1e0", fn_141d4c1e0 as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionMoveCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c210, "content_141d4c210", fn_141d4c210 as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionMoveCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c280, "content_141d4c280", fn_141d4c280 as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionMoveCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c2e0, "content_141d4c2e0", fn_141d4c2e0 as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionMoveCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c620, "content_141d4c620", tpl_39560::<0x8, 0x1401e84a0, 0x18, 0x30> as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c680, "content_141d4c680", tpl_39560::<0x8, 0x1401e84a0, 0x18, 0x30> as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c7e0, "content_141d4c7e0", tpl_35199::<0x14387f2c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c840, "content_141d4c840", tpl_35199::<0x14387f2c8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c8d0, "content_141d4c8d0", tpl_39561::<0x1401e84a0, 0x18, 0x30> as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c930, "content_141d4c930", fn_141d4c930 as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c950, "content_141d4c950", fn_141d4c950 as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c990, "content_141d4c990", fn_141d4c990 as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4c9e0, "content_141d4c9e0", fn_141d4c9e0 as *const ()), // content::GamePlayerCameraMovementComponent::AdhesionLookCurves::ClassTypeInfoImplementation::content::GamePlayerCameraMo
    Replacement::draft(0x141d4cec0, "content_141d4cec0", tpl_39561::<0x140bf9ea0, 0xd8, 0x1b0> as *const ()), // content::GamePlayerCameraMovementComponent::Adhesion::ClassTypeInfoImplementation::content::GamePlayerCameraMovementComp
    Replacement::draft(0x141d4cf30, "content_141d4cf30", fn_141d4cf30 as *const ()), // content::GamePlayerCameraMovementComponent::Adhesion::ClassTypeInfoImplementation::content::GamePlayerCameraMovementComp
    Replacement::draft(0x141d4d320, "content_141d4d320", fn_141d4d320 as *const ()), // content::GameInventoryComponent::InspectInfo::ClassTypeInfoImplementation::content::GameInventoryComponent::InspectInfo:
    Replacement::draft(0x141d4d390, "content_141d4d390", fn_141d4d390 as *const ()), // content::GameInventoryComponent::InspectInfo::ClassTypeInfoImplementation::content::GameInventoryComponent::InspectInfo:
    Replacement::draft(0x141d4d560, "content_141d4d560", tpl_35226::<0x10> as *const ()), // content::GameInventoryComponent::InspectInfo::ClassTypeInfoImplementation::content::GameInventoryComponent::InspectInfo:
    Replacement::draft(0x141d4d5a0, "content_141d4d5a0", fn_141d4d5a0 as *const ()), // content::GameInventoryComponent::InspectInfo::ClassTypeInfoImplementation::content::GameInventoryComponent::InspectInfo:
    Replacement::draft(0x141d4da90, "content_141d4da90", tpl_35199::<0x14387f420, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GameAnimationInputDamageBalancedData::ClassTypeInfoImplementation::content::GameAnimationInputDamageBalancedDat
    Replacement::draft(0x141d4daf0, "content_141d4daf0", tpl_35199::<0x14387f420, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::GameAnimationInputDamageBalancedData::ClassTypeInfoImplementation::content::GameAnimationInputDamageBalancedDat
    Replacement::draft(0x141d4db80, "content_141d4db80", tpl_34638::<0x8> as *const ()), // content::GameAnimationInputDamageBalancedData::ClassTypeInfoImplementation::content::GameAnimationInputDamageBalancedDat
    Replacement::draft(0x141d4dbc0, "content_141d4dbc0", tpl_39230::<0x8> as *const ()), // content::GameAnimationInputDamageBalancedData::ClassTypeInfoImplementation::content::GameAnimationInputDamageBalancedDat
    Replacement::draft(0x141d4e080, "content_141d4e080", tpl_35199::<0x14387f460, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FlashlightTriggersComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightTriggersCompon
    Replacement::draft(0x141d4e0e0, "content_141d4e0e0", tpl_35199::<0x14387f460, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FlashlightTriggersComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightTriggersCompon
    Replacement::draft(0x141d4e170, "content_141d4e170", tpl_39537 as *const ()), // content::FlashlightTriggersComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightTriggersCompon
    Replacement::draft(0x141d4e7c0, "content_141d4e7c0", tpl_39570::<0x1432d7b18, 0x1432d7cd0, 0x1432d7b38, 0x14088ba10> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4e800, "content_141d4e800", tpl_39570::<0x1432d7b18, 0x1432d7cd0, 0x1432d7b38, 0x14088ba10> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4ea80, "content_141d4ea80", tpl_39571::<0x8, 0x1401e84a0, 0x18, 0x14240aa00, 0xa0> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4eae0, "content_141d4eae0", tpl_39571::<0x8, 0x1401e84a0, 0x18, 0x14240aa00, 0xa0> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4ec40, "content_141d4ec40", tpl_38859::<0x14387f230, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4eca0, "content_141d4eca0", tpl_38859::<0x14387f230, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4ed30, "content_141d4ed30", tpl_39520::<0x1401e84a0, 0x18, 0x14240aa00, 0xa0> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4ed90, "content_141d4ed90", tpl_39515::<0x18, 0x14240ab10, 0x8> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4edc0, "content_141d4edc0", tpl_39516::<0x8, 0x10, 0x14, 0x18, 0x1c, 0x20, 0x1b> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4edf0, "content_141d4edf0", tpl_99271::<0xfffffffffffffff8, 0x18, 0xfffffffffffffff0, 0x14240ab10, 0x8, 0x142442c80> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4ee60, "content_141d4ee60", tpl_39517::<0x18, 0x14240ab10, 0x8, 0x142442de0> as *const ()), // content::FlashlightInventoryComponent::ArchetypeInfo::ClassTypeInfoImplementation::content::FlashlightInventoryComponent
    Replacement::draft(0x141d4f380, "content_141d4f380", tpl_39524 as *const ()), // content::FlashlightBatteryComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightBatteryComponen
    Replacement::draft(0x141d4f4a0, "content_141d4f4a0", tpl_39572::<0x1432d5f60, 0x1432d5ea0, 0x14088ba10> as *const ()), // content::FlashlightDamageComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightDamageComponent:
    Replacement::draft(0x141d4f4e0, "content_141d4f4e0", tpl_39572::<0x1432d5f60, 0x1432d5ea0, 0x14088ba10> as *const ()), // content::FlashlightDamageComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightDamageComponent:
    Replacement::draft(0x141d4f930, "content_141d4f930", fn_141d4f930 as *const ()), // content::FlashlightDamageComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightDamageComponent:
    Replacement::draft(0x141d4f970, "content_141d4f970", tpl_37083::<0x18> as *const ()), // content::FlashlightDamageComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightDamageComponent:
    Replacement::draft(0x141d4f980, "content_141d4f980", fn_141d4f980 as *const ()), // content::FlashlightDamageComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightDamageComponent:
    Replacement::draft(0x141d4f9a0, "content_141d4f9a0", tpl_37085::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x18, 0x142442c80> as *const ()), // content::FlashlightDamageComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightDamageComponent:
    Replacement::draft(0x141d4f9e0, "content_141d4f9e0", tpl_37086::<0x18, 0x142442de0> as *const ()), // content::FlashlightDamageComponent::FlashlightModeInfo::ClassTypeInfoImplementation::content::FlashlightDamageComponent:
    Replacement::draft(0x141d4faa0, "content_141d4faa0", tpl_39575::<0x14384e990, 0x1430e9528, 0x1432da918, 0x1432da970, 0x1401de410> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::DamageSpecificCooldown::ClassTypeInfoImplementation::co
    Replacement::draft(0x141d4fb00, "content_141d4fb00", tpl_39575::<0x14384e990, 0x1430e9528, 0x1432da918, 0x1432da970, 0x1401de410> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::DamageSpecificCooldown::ClassTypeInfoImplementation::co
    Replacement::draft(0x141d4fba0, "content_141d4fba0", tpl_39575::<0x14384e990, 0x1430e9528, 0x1432da918, 0x1432da970, 0x1401de410> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::DamageSpecificCooldown::ClassTypeInfoImplementation::co
    Replacement::draft(0x141d4fc00, "content_141d4fc00", tpl_39575::<0x14384e990, 0x1430e9528, 0x1432da918, 0x1432da970, 0x1401de410> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::DamageSpecificCooldown::ClassTypeInfoImplementation::co
    Replacement::draft(0x141d4fe60, "content_141d4fe60", tpl_39576::<0x8, 0x4> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::DamageSpecificCooldown::ClassTypeInfoImplementation::co
    Replacement::draft(0x141d4fe90, "content_141d4fe90", tpl_39576::<0x8, 0x4> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::DamageSpecificCooldown::ClassTypeInfoImplementation::co
    Replacement::draft(0x141d4ffc0, "content_141d4ffc0", tpl_38859::<0x14387f1e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::DamageSpecificCooldown::ClassTypeInfoImplementation::co
    Replacement::draft(0x141d50020, "content_141d50020", tpl_38859::<0x14387f1e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::DamageSpecificCooldown::ClassTypeInfoImplementation::co
    Replacement::draft(0x141d50650, "content_141d50650", fn_141d50650 as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::ClassTypeInfoImplementation::content::EnemyDamageReacti
    Replacement::draft(0x141d506a0, "content_141d506a0", tpl_39293::<0x18, 0x1401f2bc0> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::ClassTypeInfoImplementation::content::EnemyDamageReacti
    Replacement::draft(0x141d506c0, "content_141d506c0", fn_141d506c0 as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::ClassTypeInfoImplementation::content::EnemyDamageReacti
    Replacement::draft(0x141d506e0, "content_141d506e0", fn_141d506e0 as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::ClassTypeInfoImplementation::content::EnemyDamageReacti
    Replacement::draft(0x141d50740, "content_141d50740", tpl_35191::<0x18, 0x1401f2bc0, 0x142442de0> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionLevel::ClassTypeInfoImplementation::content::EnemyDamageReacti
    Replacement::draft(0x141d50c80, "content_141d50c80", tpl_39580::<0x140541650, 0x10, 0x14, 0x18> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionBodyPart::ClassTypeInfoImplementation::content::EnemyDamageRea
    Replacement::draft(0x141d50cf0, "content_141d50cf0", tpl_35186::<0x8, 0x18> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionBodyPart::ClassTypeInfoImplementation::content::EnemyDamageRea
    Replacement::draft(0x141d50d10, "content_141d50d10", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1405415a0, 0x142442c80> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionBodyPart::ClassTypeInfoImplementation::content::EnemyDamageRea
    Replacement::draft(0x141d50d70, "content_141d50d70", tpl_35198::<0x1405415a0, 0x142442de0> as *const ()), // content::EnemyDamageReactionComponent::EnemyDamageReactionBodyPart::ClassTypeInfoImplementation::content::EnemyDamageRea
    Replacement::draft(0x141d51140, "content_141d51140", tpl_39362::<0x8, 0x1401e84a0, 0x20, 0x24> as *const ()), // content::EffectsGameSpawnPool::ClassTypeInfoImplementation::content::EffectsGameSpawnPool::ClassTypeInfoImplementation::
    Replacement::draft(0x141d511a0, "content_141d511a0", tpl_39526::<0x8, 0x1401e84a0, 0x20, 0x24> as *const ()), // content::EffectsGameSpawnPool::ClassTypeInfoImplementation::content::EffectsGameSpawnPool::ClassTypeInfoImplementation::
    Replacement::draft(0x141d512f0, "content_141d512f0", tpl_35199::<0x14387f360, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::EffectsGameSpawnPool::ClassTypeInfoImplementation::content::EffectsGameSpawnPool::ClassTypeInfoImplementation::
    Replacement::draft(0x141d51350, "content_141d51350", tpl_35199::<0x14387f360, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::EffectsGameSpawnPool::ClassTypeInfoImplementation::content::EffectsGameSpawnPool::ClassTypeInfoImplementation::
    Replacement::draft(0x141d513e0, "content_141d513e0", tpl_39363::<0x8, 0x1401e84a0, 0x20, 0x24> as *const ()), // content::EffectsGameSpawnPool::ClassTypeInfoImplementation::content::EffectsGameSpawnPool::ClassTypeInfoImplementation::
    Replacement::draft(0x141d515b0, "content_141d515b0", tpl_39545::<0x14384ea30, 0x1432d81e0, 0x1432d9838, 0x1432d9878, 0x1401de410> as *const ()), // content::DarknessShieldComponent::DarknessShield::DarknessShieldBodyPart::ClassTypeInfoImplementation::content::Darkness
    Replacement::draft(0x141d51610, "content_141d51610", tpl_39545::<0x14384ea30, 0x1432d81e0, 0x1432d9838, 0x1432d9878, 0x1401de410> as *const ()), // content::DarknessShieldComponent::DarknessShield::DarknessShieldBodyPart::ClassTypeInfoImplementation::content::Darkness
    Replacement::draft(0x141d51670, "content_141d51670", tpl_39545::<0x14384eb40, 0x1432d81e0, 0x1432d9838, 0x1432d9878, 0x1401de410> as *const ()), // content::DarknessShieldComponent::DarknessShield::DarknessShieldBodyPart::ClassTypeInfoImplementation::content::Darkness
    Replacement::draft(0x141d516d0, "content_141d516d0", tpl_39545::<0x14384eb40, 0x1432d81e0, 0x1432d9838, 0x1432d9878, 0x1401de410> as *const ()), // content::DarknessShieldComponent::DarknessShield::DarknessShieldBodyPart::ClassTypeInfoImplementation::content::Darkness
    Replacement::draft(0x141d51730, "content_141d51730", tpl_39545::<0x14384ea30, 0x1432d81e0, 0x1432d9838, 0x1432d9878, 0x1401de410> as *const ()), // content::DarknessShieldComponent::DarknessShield::DarknessShieldBodyPart::ClassTypeInfoImplementation::content::Darkness
    Replacement::draft(0x141d51790, "content_141d51790", tpl_39545::<0x14384ea30, 0x1432d81e0, 0x1432d9838, 0x1432d9878, 0x1401de410> as *const ()), // content::DarknessShieldComponent::DarknessShield::DarknessShieldBodyPart::ClassTypeInfoImplementation::content::Darkness
    Replacement::draft(0x141d51b30, "content_141d51b30", tpl_38859::<0x14387f3b0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::DarknessShieldComponent::DarknessShield::DarknessShieldBodyPart::ClassTypeInfoImplementation::content::Darkness
    Replacement::draft(0x141d51b90, "content_141d51b90", tpl_38859::<0x14387f3b0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::DarknessShieldComponent::DarknessShield::DarknessShieldBodyPart::ClassTypeInfoImplementation::content::Darkness
    Replacement::draft(0x141d521b0, "content_141d521b0", fn_141d521b0 as *const ()), // content::DarknessShieldComponent::DarknessShield::ClassTypeInfoImplementation::content::DarknessShieldComponent::Darknes
    Replacement::draft(0x141d52250, "content_141d52250", tpl_38938::<0x18, 0x1401f2bc0> as *const ()), // content::DarknessShieldComponent::DarknessShield::ClassTypeInfoImplementation::content::DarknessShieldComponent::Darknes
    Replacement::draft(0x141d52280, "content_141d52280", fn_141d52280 as *const ()), // content::DarknessShieldComponent::DarknessShield::ClassTypeInfoImplementation::content::DarknessShieldComponent::Darknes
    Replacement::draft(0x141d522b0, "content_141d522b0", tpl_38939::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x18, 0x1401f2bc0, 0x142442c80> as *const ()), // content::DarknessShieldComponent::DarknessShield::ClassTypeInfoImplementation::content::DarknessShieldComponent::Darknes
    Replacement::draft(0x141d52320, "content_141d52320", tpl_38940::<0x18, 0x1401f2bc0, 0x142442de0> as *const ()), // content::DarknessShieldComponent::DarknessShield::ClassTypeInfoImplementation::content::DarknessShieldComponent::Darknes
    Replacement::draft(0x141d52400, "content_141d52400", tpl_39545::<0x14384ea30, 0x1432d81e0, 0x1432d8280, 0x1432d82c0, 0x1401de410> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d52460, "content_141d52460", tpl_39545::<0x14384ea30, 0x1432d81e0, 0x1432d8280, 0x1432d82c0, 0x1401de410> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d52500, "content_141d52500", tpl_39545::<0x14384ea30, 0x1432d81e0, 0x1432d8280, 0x1432d82c0, 0x1401de410> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d52560, "content_141d52560", tpl_39545::<0x14384ea30, 0x1432d81e0, 0x1432d8280, 0x1432d82c0, 0x1401de410> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d527c0, "content_141d527c0", tpl_39583::<0x8, 0x4, 0x10, 0x1c, 0x14, 0x20, 0x24, 0x28> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d52850, "content_141d52850", tpl_39583::<0x8, 0x4, 0x10, 0x1c, 0x14, 0x20, 0x24, 0x28> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d529e0, "content_141d529e0", tpl_39584::<0x14387f440, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d52a40, "content_141d52a40", tpl_39584::<0x14387f440, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d52ad0, "content_141d52ad0", tpl_39229::<0x20, 0x28> as *const ()), // content::DarknessShieldComponent::SourcePoint::ClassTypeInfoImplementation::content::DarknessShieldComponent::SourcePoin
    Replacement::draft(0x141d52c40, "content_141d52c40", tpl_39585::<0x1432d62a8, 0x1432d62f0, 0x1432d62c8, 0x14020f270> as *const ()), // content::DamageReactionComponent::DamageReactionData::ClassTypeInfoImplementation::content::DamageReactionComponent::Dam
    Replacement::draft(0x141d52c90, "content_141d52c90", tpl_39585::<0x1432d62a8, 0x1432d62f0, 0x1432d62c8, 0x14020f270> as *const ()), // content::DamageReactionComponent::DamageReactionData::ClassTypeInfoImplementation::content::DamageReactionComponent::Dam
    Replacement::draft(0x141d52f00, "content_141d52f00", tpl_39586::<0x8, 0x4, 0xc> as *const ()), // content::DamageReactionComponent::DamageReactionData::ClassTypeInfoImplementation::content::DamageReactionComponent::Dam
    Replacement::draft(0x141d52f50, "content_141d52f50", tpl_39586::<0x8, 0x4, 0xc> as *const ()), // content::DamageReactionComponent::DamageReactionData::ClassTypeInfoImplementation::content::DamageReactionComponent::Dam
    Replacement::draft(0x141d530a0, "content_141d530a0", tpl_35219::<0x14387f368, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::DamageReactionComponent::DamageReactionData::ClassTypeInfoImplementation::content::DamageReactionComponent::Dam
    Replacement::draft(0x141d53100, "content_141d53100", tpl_35219::<0x14387f368, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::DamageReactionComponent::DamageReactionData::ClassTypeInfoImplementation::content::DamageReactionComponent::Dam
    Replacement::draft(0x141d53670, "content_141d53670", tpl_35199::<0x14387f278, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::CameraPanningVolumeComponent::ZoomStep::ClassTypeInfoImplementation::content::CameraPanningVolumeComponent::Zoo
    Replacement::draft(0x141d536d0, "content_141d536d0", tpl_35199::<0x14387f278, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::CameraPanningVolumeComponent::ZoomStep::ClassTypeInfoImplementation::content::CameraPanningVolumeComponent::Zoo
    Replacement::draft(0x141d53760, "content_141d53760", tpl_39537 as *const ()), // content::CameraPanningVolumeComponent::ZoomStep::ClassTypeInfoImplementation::content::CameraPanningVolumeComponent::Zoo
    Replacement::draft(0x141d53880, "content_141d53880", tpl_39587::<0x14384ea30, 0x1432d7a08, 0x1432d7a90, 0x1432d7ac8, 0x1401de410> as *const ()), // content::BodyPartDamageReceiver::ClassTypeInfoImplementation::content::BodyPartDamageReceiver::ClassTypeInfoImplementati
    Replacement::draft(0x141d538e0, "content_141d538e0", tpl_39587::<0x14384ea30, 0x1432d7a08, 0x1432d7a90, 0x1432d7ac8, 0x1401de410> as *const ()), // content::BodyPartDamageReceiver::ClassTypeInfoImplementation::content::BodyPartDamageReceiver::ClassTypeInfoImplementati
    Replacement::draft(0x141d53980, "content_141d53980", tpl_39587::<0x14384ea30, 0x1432d7a08, 0x1432d7a90, 0x1432d7ac8, 0x1401de410> as *const ()), // content::BodyPartDamageReceiver::ClassTypeInfoImplementation::content::BodyPartDamageReceiver::ClassTypeInfoImplementati
    Replacement::draft(0x141d539e0, "content_141d539e0", tpl_39587::<0x14384ea30, 0x1432d7a08, 0x1432d7a90, 0x1432d7ac8, 0x1401de410> as *const ()), // content::BodyPartDamageReceiver::ClassTypeInfoImplementation::content::BodyPartDamageReceiver::ClassTypeInfoImplementati
    Replacement::draft(0x141d53c40, "content_141d53c40", tpl_38751::<0x8, 0x4, 0xc, 0x10> as *const ()), // content::BodyPartDamageReceiver::ClassTypeInfoImplementation::content::BodyPartDamageReceiver::ClassTypeInfoImplementati
    Replacement::draft(0x141d53c90, "content_141d53c90", tpl_38751::<0x8, 0x4, 0xc, 0x10> as *const ()), // content::BodyPartDamageReceiver::ClassTypeInfoImplementation::content::BodyPartDamageReceiver::ClassTypeInfoImplementati
    Replacement::draft(0x141d53de0, "content_141d53de0", tpl_38859::<0x14387f348, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BodyPartDamageReceiver::ClassTypeInfoImplementation::content::BodyPartDamageReceiver::ClassTypeInfoImplementati
    Replacement::draft(0x141d53e40, "content_141d53e40", tpl_38859::<0x14387f348, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BodyPartDamageReceiver::ClassTypeInfoImplementation::content::BodyPartDamageReceiver::ClassTypeInfoImplementati
    Replacement::draft(0x141d54400, "content_141d54400", tpl_35219::<0x14387f188, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BoardHandComponent::HandLayout::ClassTypeInfoImplementation::content::BoardHandComponent::HandLayout::ClassType
    Replacement::draft(0x141d54460, "content_141d54460", tpl_35219::<0x14387f188, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::BoardHandComponent::HandLayout::ClassTypeInfoImplementation::content::BoardHandComponent::HandLayout::ClassType
    Replacement::draft(0x141d544f0, "content_141d544f0", fn_141d544f0 as *const ()), // content::BoardHandComponent::HandLayout::ClassTypeInfoImplementation::content::BoardHandComponent::HandLayout::ClassType
    Replacement::draft(0x141d54580, "content_141d54580", tpl_38763::<0x30, 0x18> as *const ()), // content::BoardHandComponent::HandLayout::ClassTypeInfoImplementation::content::BoardHandComponent::HandLayout::ClassType
    Replacement::draft(0x141d545a0, "content_141d545a0", fn_141d545a0 as *const ()), // content::BoardHandComponent::HandLayout::ClassTypeInfoImplementation::content::BoardHandComponent::HandLayout::ClassType
    Replacement::draft(0x141d545e0, "content_141d545e0", tpl_38765::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x30, 0x18, 0x142442c80> as *const ()), // content::BoardHandComponent::HandLayout::ClassTypeInfoImplementation::content::BoardHandComponent::HandLayout::ClassType
    Replacement::draft(0x141d54630, "content_141d54630", tpl_38766::<0x30, 0x18, 0x142442de0> as *const ()), // content::BoardHandComponent::HandLayout::ClassTypeInfoImplementation::content::BoardHandComponent::HandLayout::ClassType
    Replacement::draft(0x141d54730, "content_141d54730", tpl_39590::<0x1438773d0, 0x1432d6d10, 0x1432d6ce0, 0x1432d6cf8, 0x1401de410, 0x143877430, 0x1432d6b70, 0x1432d6bf0, 0x3, 0x1432d6b58> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d547c0, "content_141d547c0", tpl_39590::<0x1438773d0, 0x1432d6d10, 0x1432d6ce0, 0x1432d6cf8, 0x1401de410, 0x143877430, 0x1432d6b70, 0x1432d6bf0, 0x3, 0x1432d6b58> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d54850, "content_141d54850", tpl_39590::<0x143877270, 0x1432d6d10, 0x1432d6ce0, 0x1432d6cf8, 0x1401de410, 0x143876330, 0x1432d6b70, 0x1432d6bf0, 0x3, 0x1432d6b58> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d548e0, "content_141d548e0", tpl_39590::<0x143877270, 0x1432d6d10, 0x1432d6ce0, 0x1432d6cf8, 0x1401de410, 0x143876330, 0x1432d6b70, 0x1432d6bf0, 0x3, 0x1432d6b58> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d54b70, "content_141d54b70", tpl_39591::<0x8, 0x1401e84a0, 0x20> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d54bd0, "content_141d54bd0", tpl_39591::<0x8, 0x1401e84a0, 0x20> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d54da0, "content_141d54da0", fn_141d54da0 as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d54df0, "content_141d54df0", tpl_37083::<0x10> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d54e00, "content_141d54e00", fn_141d54e00 as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d54e20, "content_141d54e20", tpl_37085::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x10, 0x142442c80> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d54e60, "content_141d54e60", tpl_37086::<0x10, 0x142442de0> as *const ()), // content::BoardComponent::BoardItemInfo::ClassTypeInfoImplementation::content::BoardComponent::BoardItemInfo::ClassTypeIn
    Replacement::draft(0x141d552c0, "content_141d552c0", tpl_35199::<0x14387f3c0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AvatarManagerSingletonComponent::AvatarLoadInfo::ClassTypeInfoImplementation::content::AvatarManagerSingletonCo
    Replacement::draft(0x141d55320, "content_141d55320", tpl_35199::<0x14387f3c0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AvatarManagerSingletonComponent::AvatarLoadInfo::ClassTypeInfoImplementation::content::AvatarManagerSingletonCo
    Replacement::draft(0x141d553b0, "content_141d553b0", fn_141d553b0 as *const ()), // content::AvatarManagerSingletonComponent::AvatarLoadInfo::ClassTypeInfoImplementation::content::AvatarManagerSingletonCo
    Replacement::draft(0x141d55430, "content_141d55430", tpl_39319::<0x198, 0x14240ab10, 0x110, 0x88> as *const ()), // content::AvatarManagerSingletonComponent::AvatarLoadInfo::ClassTypeInfoImplementation::content::AvatarManagerSingletonCo
    Replacement::draft(0x141d55470, "content_141d55470", fn_141d55470 as *const ()), // content::AvatarManagerSingletonComponent::AvatarLoadInfo::ClassTypeInfoImplementation::content::AvatarManagerSingletonCo
    Replacement::draft(0x141d55500, "content_141d55500", tpl_99113::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x198, 0x14240ab10, 0x110, 0x88, 0x142442c80> as *const ()), // content::AvatarManagerSingletonComponent::AvatarLoadInfo::ClassTypeInfoImplementation::content::AvatarManagerSingletonCo
    Replacement::draft(0x141d55580, "content_141d55580", tpl_39321::<0x198, 0x14240ab10, 0x110, 0x88, 0x142442de0> as *const ()), // content::AvatarManagerSingletonComponent::AvatarLoadInfo::ClassTypeInfoImplementation::content::AvatarManagerSingletonCo
    Replacement::draft(0x141d55a50, "content_141d55a50", tpl_35199::<0x14387f488, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AbilityDodgeBalancedData::ClassTypeInfoImplementation::content::AbilityDodgeBalancedData::ClassTypeInfoImplemen
    Replacement::draft(0x141d55ab0, "content_141d55ab0", tpl_35199::<0x14387f488, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AbilityDodgeBalancedData::ClassTypeInfoImplementation::content::AbilityDodgeBalancedData::ClassTypeInfoImplemen
    Replacement::draft(0x141d55c60, "content_141d55c60", tpl_39329::<0x1432d69c8, 0x1432d6910, 0x1432d68f8, 0x140551e90> as *const ()), // content::CrowdScheduleEntry::ClassTypeInfoImplementation::content::CrowdScheduleEntry::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d55ca0, "content_141d55ca0", tpl_39329::<0x1432d69c8, 0x1432d6910, 0x1432d68f8, 0x140551e90> as *const ()), // content::CrowdScheduleEntry::ClassTypeInfoImplementation::content::CrowdScheduleEntry::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d560f0, "content_141d560f0", fn_141d560f0 as *const ()), // content::CrowdScheduleEntry::ClassTypeInfoImplementation::content::CrowdScheduleEntry::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d56150, "content_141d56150", tpl_39597::<0x28, 0x10, 0x140559d70> as *const ()), // content::CrowdScheduleEntry::ClassTypeInfoImplementation::content::CrowdScheduleEntry::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d56170, "content_141d56170", fn_141d56170 as *const ()), // content::CrowdScheduleEntry::ClassTypeInfoImplementation::content::CrowdScheduleEntry::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d561a0, "content_141d561a0", fn_141d561a0 as *const ()), // content::CrowdScheduleEntry::ClassTypeInfoImplementation::content::CrowdScheduleEntry::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d56200, "content_141d56200", tpl_39600::<0x28, 0x10, 0x140559d70, 0x142442de0> as *const ()), // content::CrowdScheduleEntry::ClassTypeInfoImplementation::content::CrowdScheduleEntry::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d565c0, "content_141d565c0", tpl_39601::<0x8, 0x1401e84a0> as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d565f0, "content_141d565f0", tpl_39601::<0x8, 0x1401e84a0> as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d56720, "content_141d56720", tpl_35219::<0x14387f418, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d56780, "content_141d56780", tpl_35219::<0x14387f418, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d56810, "content_141d56810", fn_141d56810 as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d56860, "content_141d56860", tpl_38763::<0x28, 0x10> as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d56880, "content_141d56880", fn_141d56880 as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d568b0, "content_141d568b0", tpl_38765::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x28, 0x10, 0x142442c80> as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d56900, "content_141d56900", tpl_38766::<0x28, 0x10, 0x142442de0> as *const ()), // content::CrowdNPCArchetype::ClassTypeInfoImplementation::content::CrowdNPCArchetype::ClassTypeInfoImplementation::vfunc_
    Replacement::draft(0x141d56e50, "content_141d56e50", fn_141d56e50 as *const ()), // content::AITicketScoring::ClassTypeInfoImplementation::content::AITicketScoring::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d57340, "content_141d57340", tpl_35219::<0x14387f138, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::AIMeleeComboPhase::ClassTypeInfoImplementation::content::AbilityAIMe
    Replacement::draft(0x141d573a0, "content_141d573a0", tpl_35219::<0x14387f138, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::AIMeleeComboPhase::ClassTypeInfoImplementation::content::AbilityAIMe
    Replacement::draft(0x141d57430, "content_141d57430", fn_141d57430 as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::AIMeleeComboPhase::ClassTypeInfoImplementation::content::AbilityAIMe
    Replacement::draft(0x141d574c0, "content_141d574c0", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401f2bc0, 0x142442c80> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::AIMeleeComboPhase::ClassTypeInfoImplementation::content::AbilityAIMe
    Replacement::draft(0x141d57520, "content_141d57520", tpl_35198::<0x1401f2bc0, 0x142442de0> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::AIMeleeComboPhase::ClassTypeInfoImplementation::content::AbilityAIMe
    Replacement::draft(0x141d57a00, "content_141d57a00", fn_141d57a00 as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMel
    Replacement::draft(0x141d57a80, "content_141d57a80", tpl_39607::<0x10, 0x18, 0x20> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMel
    Replacement::draft(0x141d57aa0, "content_141d57aa0", tpl_39306::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x1404213b0, 0x142442c80> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMel
    Replacement::draft(0x141d57b00, "content_141d57b00", tpl_35191::<0x10, 0x1404213b0, 0x142442de0> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeComboInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMel
    Replacement::draft(0x141d57f90, "content_141d57f90", tpl_35219::<0x14387f288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::AIMeleeAttackInfoHitBox::ClassTypeInfoImplementation::content::Abil
    Replacement::draft(0x141d57ff0, "content_141d57ff0", tpl_35219::<0x14387f288, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::AIMeleeAttackInfoHitBox::ClassTypeInfoImplementation::content::Abil
    Replacement::draft(0x141d58080, "content_141d58080", fn_141d58080 as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::AIMeleeAttackInfoHitBox::ClassTypeInfoImplementation::content::Abil
    Replacement::draft(0x141d58100, "content_141d58100", tpl_38952::<0x88, 0x14240ab10> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::AIMeleeAttackInfoHitBox::ClassTypeInfoImplementation::content::Abil
    Replacement::draft(0x141d58130, "content_141d58130", fn_141d58130 as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::AIMeleeAttackInfoHitBox::ClassTypeInfoImplementation::content::Abil
    Replacement::draft(0x141d58180, "content_141d58180", tpl_98840::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x88, 0x14240ab10, 0x142442c80> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::AIMeleeAttackInfoHitBox::ClassTypeInfoImplementation::content::Abil
    Replacement::draft(0x141d581f0, "content_141d581f0", tpl_38954::<0x88, 0x14240ab10, 0x142442de0> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::AIMeleeAttackInfoHitBox::ClassTypeInfoImplementation::content::Abil
    Replacement::draft(0x141d582d0, "content_141d582d0", tpl_39610::<0x14384e990, 0x1432d5a30, 0x1432d5d00, 0x1432d5d10, 0x1401de410> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMe
    Replacement::draft(0x141d58330, "content_141d58330", tpl_39610::<0x14384e990, 0x1432d5a30, 0x1432d5d00, 0x1432d5d10, 0x1401de410> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMe
    Replacement::draft(0x141d583d0, "content_141d583d0", tpl_39610::<0x14384e990, 0x1432d5a30, 0x1432d5d00, 0x1432d5d10, 0x1401de410> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMe
    Replacement::draft(0x141d58430, "content_141d58430", tpl_39610::<0x14384e990, 0x1432d5a30, 0x1432d5d00, 0x1432d5d10, 0x1401de410> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMe
    Replacement::draft(0x141d58840, "content_141d58840", fn_141d58840 as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMe
    Replacement::draft(0x141d58900, "content_141d58900", fn_141d58900 as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMe
    Replacement::draft(0x141d58930, "content_141d58930", tpl_35197::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x140421460, 0x142442c80> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMe
    Replacement::draft(0x141d58990, "content_141d58990", tpl_35198::<0x140421460, 0x142442de0> as *const ()), // content::AbilityAIMeleeComponent::AIMeleeAttackInfo::ClassTypeInfoImplementation::content::AbilityAIMeleeComponent::AIMe
    Replacement::draft_throws(0x141d59740, "content_141d59740", fn_141d59740 as *const (), Throws::Discard), // content::StartInteractableObjectSearchDecorator::ClassTypeInfoImplementation::content::StartInteractableObjectSearchDeco
    Replacement::draft(0x141d597d0, "content_141d597d0", fn_141d597d0 as *const ()), // content::StartInteractableObjectSearchDecorator::ClassTypeInfoImplementation::content::StartInteractableObjectSearchDeco
    Replacement::draft_throws(0x141d59ee0, "content_141d59ee0", fn_141d59ee0 as *const (), Throws::Discard), // content::SetCombatStateDecorator::ClassTypeInfoImplementation::content::SetCombatStateDecorator::ClassTypeInfoImplementa
    Replacement::draft(0x141d5a0f0, "content_141d5a0f0", fn_141d5a0f0 as *const ()), // content::SetCanEnterStealthAreas::content::SetCanEnterStealthAreas::vfunc_4
    Replacement::draft_throws(0x141d5a1d0, "content_141d5a1d0", tpl_838::<0x140d269b0> as *const (), Throws::Discard), // content::SetCanEnterStealthAreas::content::SetCanEnterStealthAreas::vfunc_13
    Replacement::draft(0x141d5a280, "content_141d5a280", fn_141d5a280 as *const ()), // content::SetCanEnterStealthAreas::ClassTypeInfoImplementation::content::SetCanEnterStealthAreas::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141d5a720, "content_141d5a720", fn_141d5a720 as *const (), Throws::Discard), // content::SetCanEnterStealthAreas::ClassTypeInfoImplementation::content::SetCanEnterStealthAreas::ClassTypeInfoImplementa
    Replacement::draft(0x141d5a7b0, "content_141d5a7b0", fn_141d5a7b0 as *const ()), // content::SetCanEnterStealthAreas::ClassTypeInfoImplementation::content::SetCanEnterStealthAreas::ClassTypeInfoImplementa
    Replacement::draft_throws(0x141d5b100, "content_141d5b100", tpl_38659::<0x8, 0x10, 0x1406f2520> as *const (), Throws::Discard), // content::SendLuaEventDecorator::ClassTypeInfoImplementation::content::SendLuaEventDecorator::ClassTypeInfoImplementation
    Replacement::draft(0x141d5b580, "content_141d5b580", fn_141d5b580 as *const ()), // content::NoiseSensorComponent::ClassTypeInfoImplementation::content::NoiseSensorComponent::ClassTypeInfoImplementation::
    Replacement::draft(0x141d5b750, "content_141d5b750", fn_141d5b750 as *const ()), // content::NoiseSensorComponent::ClassTypeInfoImplementation::content::NoiseSensorComponent::ClassTypeInfoImplementation::
    Replacement::draft_throws(0x141d5c490, "content_141d5c490", fn_141d5c490 as *const (), Throws::Discard), // content::LeadDecorator::ClassTypeInfoImplementation::content::LeadDecorator::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x141d5ccd0, "content_141d5ccd0", tpl_39623::<0x8, 0xc, 0x10, 0x14, 0x15, 0x18, 0x1c> as *const ()), // content::EQSStealthAreaTask::ClassTypeInfoImplementation::content::EQSStealthAreaTask::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d5cd40, "content_141d5cd40", tpl_38592::<0x1432e4918, 0xc, 0x18> as *const ()), // content::EQSStealthAreaTask::ClassTypeInfoImplementation::content::EQSStealthAreaTask::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d5d440, "content_141d5d440", fn_141d5d440 as *const ()), // content::EQSLeashCoreZoneTask::ClassTypeInfoImplementation::content::EQSLeashCoreZoneTask::ClassTypeInfoImplementation::
    Replacement::draft(0x141d5d4b0, "content_141d5d4b0", fn_141d5d4b0 as *const ()), // content::EQSLeashCoreZoneTask::ClassTypeInfoImplementation::content::EQSLeashCoreZoneTask::ClassTypeInfoImplementation::
    Replacement::draft(0x141d5dbc0, "content_141d5dbc0", tpl_39623::<0x8, 0xc, 0x10, 0x14, 0x15, 0x18, 0x1c> as *const ()), // content::EQSEnemyAdvantageZoneTask::ClassTypeInfoImplementation::content::EQSEnemyAdvantageZoneTask::ClassTypeInfoImplem
    Replacement::draft(0x141d5dc30, "content_141d5dc30", tpl_38592::<0x1432e61b8, 0xc, 0x18> as *const ()), // content::EQSEnemyAdvantageZoneTask::ClassTypeInfoImplementation::content::EQSEnemyAdvantageZoneTask::ClassTypeInfoImplem
    Replacement::draft_throws(0x141d5f0d0, "content_141d5f0d0", tpl_38650::<0x8, 0x10, 0x1406f2520, 0x20> as *const (), Throws::Discard), // content::DecoratorCheckAttackCategories::ClassTypeInfoImplementation::content::DecoratorCheckAttackCategories::ClassType
    Replacement::draft(0x141d5f300, "content_141d5f300", fn_141d5f300 as *const ()), // content::ChargeAttackSensor::ClassTypeInfoImplementation::content::ChargeAttackSensor::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d5f5a0, "content_141d5f5a0", fn_141d5f5a0 as *const ()), // content::ChargeAttackSensor::ClassTypeInfoImplementation::content::ChargeAttackSensor::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d5f760, "content_141d5f760", tpl_39628::<0x8, 0xc> as *const ()), // content::ChargeAttackSensor::ClassTypeInfoImplementation::content::ChargeAttackSensor::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x141d5f7b0, "content_141d5f7b0", fn_141d5f7b0 as *const ()), // content::ChargeAttackSensor::ClassTypeInfoImplementation::content::ChargeAttackSensor::ClassTypeInfoImplementation::vfun
    Replacement::draft_throws(0x141d5feb0, "content_141d5feb0", tpl_39630::<0x1407cabd0, 0x38, 0x40, 0x44, 0x45, 0x48, 0x50, 0x58> as *const (), Throws::Discard), // content::BehaviorTreeUseMeleeNode::ClassTypeInfoImplementation::content::BehaviorTreeUseMeleeNode::ClassTypeInfoImplemen
    Replacement::draft(0x141d5ff40, "content_141d5ff40", tpl_39631::<0x1432e4d58, 0x8, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x50, 0x58> as *const ()), // content::BehaviorTreeUseMeleeNode::ClassTypeInfoImplementation::content::BehaviorTreeUseMeleeNode::ClassTypeInfoImplemen
    Replacement::draft_throws(0x141d60660, "content_141d60660", fn_141d60660 as *const (), Throws::Discard), // content::BehaviorTreeRushThroughNode::ClassTypeInfoImplementation::content::BehaviorTreeRushThroughNode::ClassTypeInfoIm
    Replacement::draft(0x141d60750, "content_141d60750", fn_141d60750 as *const ()), // content::BehaviorTreeRushThroughNode::ClassTypeInfoImplementation::content::BehaviorTreeRushThroughNode::ClassTypeInfoIm
    Replacement::draft_throws(0x141d60ec0, "content_141d60ec0", fn_141d60ec0 as *const (), Throws::Discard), // content::BehaviorTreeMoveToCanDestroyNode::ClassTypeInfoImplementation::content::BehaviorTreeMoveToCanDestroyNode::Class
    Replacement::draft(0x141d60f90, "content_141d60f90", fn_141d60f90 as *const ()), // content::BehaviorTreeMoveToCanDestroyNode::ClassTypeInfoImplementation::content::BehaviorTreeMoveToCanDestroyNode::Class
];
