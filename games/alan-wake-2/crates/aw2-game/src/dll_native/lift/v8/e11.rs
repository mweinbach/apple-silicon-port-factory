#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use super::super::Lifted;
use super::*;

pub static ENTRIES: &[Lifted] = &[
    Lifted::new(0x7001766850, fn_7001766850 as *const (), 0), // +0x107a850 sub_107a850
    Lifted::new(0x7001766a30, fn_7001766a30 as *const (), 0), // +0x107aa30 sub_107aa30
    Lifted::new(0x7001766fb0, fn_7001766fb0 as *const (), 0), // +0x107afb0 sub_107afb0
    Lifted::new(0x7001767110, fn_7001767110 as *const (), 0), // +0x107b110 sub_107b110
    Lifted::new(0x70017677a0, fn_70017677a0 as *const (), 0), // +0x107b7a0 sub_107b7a0
    Lifted::new(0x7001767970, fn_7001767970 as *const (), 0), // +0x107b970 sub_107b970
    Lifted::new(0x7001767a80, fn_7001767a80 as *const (), 0), // +0x107ba80 sub_107ba80
    Lifted::new(0x7001767bb0, fn_7001767bb0 as *const (), 0), // +0x107bbb0 sub_107bbb0
    Lifted::new(0x7001768040, fn_7001768040 as *const (), 0), // +0x107c040 sub_107c040
    Lifted::new(0x70017681a0, fn_70017681a0 as *const (), 0), // +0x107c1a0 sub_107c1a0
    Lifted::new(0x7001768290, fn_7001768290 as *const (), 0), // +0x107c290 sub_107c290
    Lifted::new(0x70017682c0, fn_70017682c0 as *const (), 0), // +0x107c2c0 sub_107c2c0
    Lifted::new(0x70017682f0, fn_70017682f0 as *const (), 0), // +0x107c2f0 sub_107c2f0
    Lifted::new(0x7001768430, fn_7001768430 as *const (), 0), // +0x107c430 sub_107c430
    Lifted::new(0x70017685c0, fn_70017685c0 as *const (), 0), // +0x107c5c0 sub_107c5c0
    Lifted::new(0x7001768720, fn_7001768720 as *const (), 0), // +0x107c720 sub_107c720
    Lifted::new(0x70017688a0, fn_70017688a0 as *const (), 2), // +0x107c8a0 sub_107c8a0
    Lifted::new(0x70017693f0, fn_70017693f0 as *const (), 0), // +0x107d3f0 sub_107d3f0
    Lifted::new(0x70017696f0, fn_70017696f0 as *const (), 0), // +0x107d6f0 sub_107d6f0
    Lifted::new(0x7001769a20, tpl_243::<0x7001769a30> as *const (), 0), // +0x107da20 sub_107da20
    Lifted::new(0x7001769a30, tpl_1317::<0x10, 0x8, 0x7001d54ce0, 0x7001d55060, 0x70017db8e8, 0x7001d54ef8, 0x7001769a97, 0x700177c0b0, 0x18, 0x7001d54ef0> as *const (), 0), // +0x107da30 sub_107da30
    Lifted::new(0x7001769b20, tpl_1317::<0x10, 0x8, 0x7001d54ce0, 0x7001d55060, 0x70017db8e8, 0x7001d54ef8, 0x7001769b87, 0x700177c0b0, 0x18, 0x7001d54ef0> as *const (), 0), // +0x107db20 sub_107db20
    Lifted::new(0x7001769c10, tpl_1317::<0x10, 0x8, 0x7001d54ce0, 0x7001d55060, 0x70017db8e8, 0x7001d54ef8, 0x7001769c77, 0x700177c0b0, 0x18, 0x7001d54ef0> as *const (), 0), // +0x107dc10 sub_107dc10
    Lifted::new(0x7001769d00, fn_7001769d00 as *const (), 0), // +0x107dd00 sub_107dd00
    Lifted::new(0x7001769da0, fn_7001769da0 as *const (), 0), // +0x107dda0 sub_107dda0
    Lifted::new(0x7001769e00, tpl_2862::<0x10, 0x700078a060, 0x700079b1d0, 0x7000760580> as *const (), 0), // +0x107de00 sub_107de00
    Lifted::new(0x7001769e70, tpl_2862::<0x10, 0x700078a060, 0x700079b1d0, 0x700075c590> as *const (), 0), // +0x107de70 sub_107de70
    Lifted::new(0x7001769ee0, tpl_2852::<0x10, 0x700079b1d0, 0x700078db90> as *const (), 0), // +0x107dee0 sub_107dee0
    Lifted::new(0x7001769f30, fn_7001769f30 as *const (), 0), // +0x107df30 sub_107df30
    Lifted::new(0x7001769f90, fn_7001769f90 as *const (), 0), // +0x107df90 sub_107df90
    Lifted::new(0x700176a000, fn_700176a000 as *const (), 0), // +0x107e000 sub_107e000
    Lifted::new(0x700176a100, tpl_143::<0x10, 0x700079b1d0> as *const (), 0), // +0x107e100 sub_107e100
    Lifted::new(0x700176a110, tpl_143::<0x10, 0x7000799b70> as *const (), 0), // +0x107e110 sub_107e110
    Lifted::new(0x700176a120, fn_700176a120 as *const (), 0), // +0x107e120 sub_107e120
    Lifted::new(0x700176a250, fn_700176a250 as *const (), 0), // +0x107e250 sub_107e250
    Lifted::new(0x700176a2f0, fn_700176a2f0 as *const (), 0), // +0x107e2f0 sub_107e2f0
    Lifted::new(0x700176a400, fn_700176a400 as *const (), 0), // +0x107e400 sub_107e400
    Lifted::new(0x700176a4b0, fn_700176a4b0 as *const (), 0), // +0x107e4b0 sub_107e4b0
    Lifted::new(0x700176a590, fn_700176a590 as *const (), 0), // +0x107e590 sub_107e590
    Lifted::new(0x700176a640, fn_700176a640 as *const (), 0), // +0x107e640 sub_107e640
    Lifted::new(0x700176a710, fn_700176a710 as *const (), 0), // +0x107e710 sub_107e710
    Lifted::new(0x700176a7a0, fn_700176a7a0 as *const (), 0), // +0x107e7a0 sub_107e7a0
    Lifted::new(0x700176a850, fn_700176a850 as *const (), 0), // +0x107e850 sub_107e850
    Lifted::new(0x700176ab50, fn_700176ab50 as *const (), 0), // +0x107eb50 sub_107eb50
    Lifted::new(0x700176add0, fn_700176add0 as *const (), 0), // +0x107edd0 sub_107edd0
    Lifted::new(0x700176b070, fn_700176b070 as *const (), 0), // +0x107f070 sub_107f070
    Lifted::new(0x700176b120, fn_700176b120 as *const (), 0), // +0x107f120 sub_107f120
    Lifted::new(0x700176b540, fn_700176b540 as *const (), 0), // +0x107f540 sub_107f540
    Lifted::new(0x700176b5a0, fn_700176b5a0 as *const (), 0), // +0x107f5a0 sub_107f5a0
    Lifted::new(0x700176b690, fn_700176b690 as *const (), 0), // +0x107f690 sub_107f690
    Lifted::new(0x700176b710, fn_700176b710 as *const (), 0), // +0x107f710 sub_107f710
    Lifted::new(0x700176b7c0, fn_700176b7c0 as *const (), 0), // +0x107f7c0 sub_107f7c0
    Lifted::new(0x700176b890, fn_700176b890 as *const (), 0), // +0x107f890 sub_107f890
    Lifted::new(0x700176b940, fn_700176b940 as *const (), 0), // +0x107f940 sub_107f940
    Lifted::new(0x700176ba00, fn_700176ba00 as *const (), 0), // +0x107fa00 sub_107fa00
    Lifted::new(0x700176ba60, tpl_2863::<0x7001c6abd8, 0x700072a220, 0x7000734700, 0x7001773140, 0x7000782ab0, 0x70007611f0, 0x7001775ad0> as *const (), 0), // +0x107fa60 sub_107fa60
    Lifted::new(0x700176bbf0, tpl_2863::<0x7001c6abd8, 0x700074cf10, 0x7000734700, 0x7001773140, 0x7000782ab0, 0x70007611f0, 0x7001775ad0> as *const (), 0), // +0x107fbf0 sub_107fbf0
    Lifted::new(0x700176bd80, fn_700176bd80 as *const (), 0), // +0x107fd80 sub_107fd80
    Lifted::new(0x700176c030, fn_700176c030 as *const (), 0), // +0x1080030 sub_1080030
    Lifted::new(0x700176c1d0, fn_700176c1d0 as *const (), 0), // +0x10801d0 sub_10801d0
    Lifted::new(0x700176c230, fn_700176c230 as *const (), 0), // +0x1080230 sub_1080230
    Lifted::new(0x700176c2d0, fn_700176c2d0 as *const (), 0), // +0x10802d0 sub_10802d0
    Lifted::new(0x700176c390, fn_700176c390 as *const (), 0), // +0x1080390 sub_1080390
    Lifted::new(0x700176c520, fn_700176c520 as *const (), 2), // +0x1080520 sub_1080520
    Lifted::new(0x700176c960, fn_700176c960 as *const (), 0), // +0x1080960 sub_1080960
    Lifted::new(0x700176cba0, fn_700176cba0 as *const (), 0), // +0x1080ba0 sub_1080ba0
    Lifted::new(0x700176cc80, fn_700176cc80 as *const (), 0), // +0x1080c80 sub_1080c80
    Lifted::new(0x700176cd30, fn_700176cd30 as *const (), 0), // +0x1080d30 sub_1080d30
    Lifted::new(0x700176d190, fn_700176d190 as *const (), 0), // +0x1081190 sub_1081190
    Lifted::new(0x700176d260, fn_700176d260 as *const (), 0), // +0x1081260 sub_1081260
    Lifted::new(0x700176d3f0, tpl_2864::<0x7001c6abd8, 0x70007945d0, 0x70018da2c8, 0x70018da308, 0x70018da348, 0x7001765650, 0x700151de70, 0x7000747e00, 0x7000704e20, 0x7000759750, 0x700074a1e0, 0x7000720c90> as *const (), 2), // +0x10813f0 sub_10813f0
    Lifted::new(0x700176d640, tpl_2864::<0x7001c6abd8, 0x70007945d0, 0x70018da2c8, 0x70018da308, 0x70018da348, 0x7001766fb0, 0x700151de70, 0x7000747e00, 0x7000704e20, 0x7000759750, 0x700074a1e0, 0x7000720c90> as *const (), 2), // +0x1081640 sub_1081640
    Lifted::new(0x700176d890, fn_700176d890 as *const (), 0), // +0x1081890 sub_1081890
    Lifted::new(0x700176d940, fn_700176d940 as *const (), 0), // +0x1081940 sub_1081940
    Lifted::new(0x700176d9b0, fn_700176d9b0 as *const (), 2), // +0x10819b0 sub_10819b0
    Lifted::new(0x700176dc20, fn_700176dc20 as *const (), 0), // +0x1081c20 sub_1081c20
    Lifted::new(0x700176dc50, tpl_2865::<0x7001c6abd8, 0x70007945d0, 0x70018da208, 0x70018da248, 0x70018da288, 0x7001765650, 0x700079b1d0, 0x700151e340, 0x7000747e00, 0x7000704e20, 0x7000759750, 0x700074a1e0, 0x7000720c90> as *const (), 2), // +0x1081c50 sub_1081c50
    Lifted::new(0x700176ded0, tpl_2865::<0x7001c6abd8, 0x70007945d0, 0x70018da208, 0x70018da248, 0x70018da288, 0x7001766fb0, 0x700079b1d0, 0x700151e340, 0x7000747e00, 0x7000704e20, 0x7000759750, 0x700074a1e0, 0x7000720c90> as *const (), 2), // +0x1081ed0 sub_1081ed0
    Lifted::new(0x700176e150, fn_700176e150 as *const (), 0), // +0x1082150 sub_1082150
    Lifted::new(0x700176e1c0, fn_700176e1c0 as *const (), 0), // +0x10821c0 sub_10821c0
    Lifted::new(0x700176e280, fn_700176e280 as *const (), 2), // +0x1082280 sub_1082280
    Lifted::new(0x700176e510, fn_700176e510 as *const (), 0), // +0x1082510 sub_1082510
    Lifted::new(0x700176e540, fn_700176e540 as *const (), 2), // +0x1082540 sub_1082540
    Lifted::new(0x700176f350, fn_700176f350 as *const (), 0), // +0x1083350 sub_1083350
    Lifted::new(0x700176fd10, fn_700176fd10 as *const (), 0), // +0x1083d10 sub_1083d10
    Lifted::new(0x700176fd30, fn_700176fd30 as *const (), 0), // +0x1083d30 sub_1083d30
    Lifted::new(0x700176fec0, fn_700176fec0 as *const (), 2), // +0x1083ec0 sub_1083ec0
    Lifted::new(0x700176ffe0, fn_700176ffe0 as *const (), 2), // +0x1083fe0 sub_1083fe0
    Lifted::new(0x7001770030, fn_7001770030 as *const (), 2), // +0x1084030 sub_1084030
    Lifted::new(0x7001770080, fn_7001770080 as *const (), 0), // +0x1084080 sub_1084080
    Lifted::new(0x7001770130, fn_7001770130 as *const (), 0), // +0x1084130 sub_1084130
    Lifted::new(0x70017701a0, tpl_2866::<0x40, 0x10, 0x7000715730, 0x700074a1e0> as *const (), 0), // +0x10841a0 sub_10841a0
    Lifted::new(0x70017701f0, fn_70017701f0 as *const (), 0), // +0x10841f0 sub_10841f0
    Lifted::new(0x70017704c0, fn_70017704c0 as *const (), 0), // +0x10844c0 sub_10844c0
    Lifted::new(0x70017704f0, fn_70017704f0 as *const (), 0), // +0x10844f0 sub_10844f0
    Lifted::new(0x7001770520, fn_7001770520 as *const (), 0), // +0x1084520 sub_1084520
    Lifted::new(0x7001770560, tpl_2866::<0x40, 0x10, 0x7000762a30, 0x700074a1e0> as *const (), 0), // +0x1084560 sub_1084560
    Lifted::new(0x70017705b0, fn_70017705b0 as *const (), 0), // +0x10845b0 sub_10845b0
    Lifted::new(0x7001770630, fn_7001770630 as *const (), 0), // +0x1084630 sub_1084630
    Lifted::new(0x7001770950, fn_7001770950 as *const (), 0), // +0x1084950 sub_1084950
    Lifted::new(0x7001772b60, fn_7001772b60 as *const (), 0), // +0x1086b60 sub_1086b60
    Lifted::new(0x7001772ec0, fn_7001772ec0 as *const (), 0), // +0x1086ec0 sub_1086ec0
    Lifted::new(0x7001773140, fn_7001773140 as *const (), 2), // +0x1087140 sub_1087140
    Lifted::new(0x7001773290, fn_7001773290 as *const (), 0), // +0x1087290 sub_1087290
    Lifted::new(0x70017734f0, fn_70017734f0 as *const (), 0), // +0x10874f0 sub_10874f0
    Lifted::new(0x7001773770, fn_7001773770 as *const (), 0), // +0x1087770 sub_1087770
    Lifted::new(0x70017738a0, fn_70017738a0 as *const (), 0), // +0x10878a0 sub_10878a0
    Lifted::new(0x7001773970, fn_7001773970 as *const (), 0), // +0x1087970 sub_1087970
    Lifted::new(0x7001773a60, fn_7001773a60 as *const (), 0), // +0x1087a60 sub_1087a60
    Lifted::new(0x7001773b90, fn_7001773b90 as *const (), 0), // +0x1087b90 sub_1087b90
    Lifted::new(0x7001773f70, fn_7001773f70 as *const (), 0), // +0x1087f70 sub_1087f70
    Lifted::new(0x7001774010, fn_7001774010 as *const (), 0), // +0x1088010 sub_1088010
    Lifted::new(0x7001774090, fn_7001774090 as *const (), 2), // +0x1088090 sub_1088090
    Lifted::new(0x70017741b0, fn_70017741b0 as *const (), 2), // +0x10881b0 sub_10881b0
    Lifted::new(0x7001774340, fn_7001774340 as *const (), 0), // +0x1088340 sub_1088340
    Lifted::new(0x7001774390, fn_7001774390 as *const (), 2), // +0x1088390 sub_1088390
    Lifted::new(0x7001774450, fn_7001774450 as *const (), 2), // +0x1088450 sub_1088450
    Lifted::new(0x7001774570, fn_7001774570 as *const (), 0), // +0x1088570 sub_1088570
    Lifted::new(0x70017745d0, fn_70017745d0 as *const (), 0), // +0x10885d0 sub_10885d0
    Lifted::new(0x70017746f0, tpl_2867::<0x10, 0x8, 0x70007824a0, 0x700074a1e0, 0x700176c230> as *const (), 0), // +0x10886f0 sub_10886f0
    Lifted::new(0x7001774790, tpl_2867::<0x10, 0x8, 0x7000796d20, 0x700074a1e0, 0x700176c230> as *const (), 0), // +0x1088790 sub_1088790
    Lifted::new(0x7001774830, fn_7001774830 as *const (), 3), // +0x1088830 sub_1088830
    Lifted::new(0x70017748d0, fn_70017748d0 as *const (), 0), // +0x10888d0 sub_10888d0
    Lifted::new(0x7001774d80, fn_7001774d80 as *const (), 0), // +0x1088d80 sub_1088d80
    Lifted::new(0x7001774e70, fn_7001774e70 as *const (), 2), // +0x1088e70 sub_1088e70
    Lifted::new(0x70017754d0, fn_70017754d0 as *const (), 0), // +0x10894d0 sub_10894d0
    Lifted::new(0x7001775550, fn_7001775550 as *const (), 0), // +0x1089550 sub_1089550
    Lifted::new(0x7001775730, fn_7001775730 as *const (), 0), // +0x1089730 sub_1089730
    Lifted::new(0x70017758f0, fn_70017758f0 as *const (), 0), // +0x10898f0 sub_10898f0
    Lifted::new(0x7001775960, fn_7001775960 as *const (), 0), // +0x1089960 sub_1089960
    Lifted::new(0x7001775a20, fn_7001775a20 as *const (), 0), // +0x1089a20 sub_1089a20
    Lifted::new(0x7001775ad0, tpl_2868::<0x7001768720, 0x10, 0x20, 0x18, 0x8, 0x700071f990, 0x700074a1e0, 0x70006f5350, 0x58, 0x1, 0x7000721b80> as *const (), 0), // +0x1089ad0 sub_1089ad0
    Lifted::new(0x7001775bd0, tpl_2868::<0x7001768720, 0x10, 0x20, 0x18, 0x8, 0x70007024f0, 0x700074a1e0, 0x70006f5350, 0x58, 0x1, 0x7000721b80> as *const (), 0), // +0x1089bd0 sub_1089bd0
    Lifted::new(0x7001775cd0, fn_7001775cd0 as *const (), 0), // +0x1089cd0 sub_1089cd0
    Lifted::new(0x7001776080, fn_7001776080 as *const (), 0), // +0x108a080 sub_108a080
    Lifted::new(0x7001776260, fn_7001776260 as *const (), 0), // +0x108a260 sub_108a260
    Lifted::new(0x7001776320, fn_7001776320 as *const (), 0), // +0x108a320 sub_108a320
    Lifted::new(0x7001777190, fn_7001777190 as *const (), 0), // +0x108b190 sub_108b190
    Lifted::new(0x7001777280, fn_7001777280 as *const (), 1), // +0x108b280 sub_108b280
    Lifted::new(0x70017772b0, fn_70017772b0 as *const (), 0), // +0x108b2b0 sub_108b2b0
    Lifted::new(0x70017777d0, fn_70017777d0 as *const (), 0), // +0x108b7d0 sub_108b7d0
    Lifted::new(0x7001777850, fn_7001777850 as *const (), 0), // +0x108b850 sub_108b850
    Lifted::new(0x7001777870, tpl_980::<0x70018da408, 0x8, 0x18> as *const (), 3), // +0x108b870 sub_108b870
    Lifted::new(0x70017778a0, tpl_918::<0x70018da288, 0x8> as *const (), 3), // +0x108b8a0 sub_108b8a0
    Lifted::new(0x70017778c0, tpl_979::<0x70018da448, 0x8, 0x18> as *const (), 3), // +0x108b8c0 sub_108b8c0
    Lifted::new(0x70017778f0, tpl_918::<0x70018da308, 0x8> as *const (), 3), // +0x108b8f0 sub_108b8f0
    Lifted::new(0x7001777910, tpl_918::<0x70018da2c8, 0x8> as *const (), 3), // +0x108b910 sub_108b910
    Lifted::new(0x7001777930, tpl_980::<0x70018da4c8, 0x8, 0x18> as *const (), 3), // +0x108b930 sub_108b930
    Lifted::new(0x7001777960, tpl_918::<0x70018da208, 0x8> as *const (), 3), // +0x108b960 sub_108b960
    Lifted::new(0x7001777980, tpl_980::<0x70018da488, 0x8, 0x18> as *const (), 3), // +0x108b980 sub_108b980
    Lifted::new(0x70017779b0, tpl_980::<0x70018da3c8, 0x8, 0x18> as *const (), 3), // +0x108b9b0 sub_108b9b0
    Lifted::new(0x70017779e0, tpl_980::<0x70018da388, 0x8, 0x18> as *const (), 3), // +0x108b9e0 sub_108b9e0
    Lifted::new(0x7001777a10, tpl_918::<0x70018da348, 0x8> as *const (), 3), // +0x108ba10 sub_108ba10
    Lifted::new(0x7001777a30, tpl_918::<0x70018da248, 0x8> as *const (), 3), // +0x108ba30 sub_108ba30
    Lifted::new(0x7001777a50, tpl_984::<0x700177c312> as *const (), 0), // +0x108ba50 sub_108ba50
    Lifted::new(0x7001777a70, tpl_919::<0x700177c312> as *const (), 0), // +0x108ba70 sub_108ba70
    Lifted::new(0x7001777a90, tpl_983::<0x700177c312> as *const (), 0), // +0x108ba90 sub_108ba90
    Lifted::new(0x7001777ab0, tpl_919::<0x700177c312> as *const (), 0), // +0x108bab0 sub_108bab0
    Lifted::new(0x7001777ad0, tpl_919::<0x700177c312> as *const (), 0), // +0x108bad0 sub_108bad0
    Lifted::new(0x7001777af0, tpl_984::<0x700177c312> as *const (), 0), // +0x108baf0 sub_108baf0
    Lifted::new(0x7001777b10, tpl_919::<0x700177c312> as *const (), 0), // +0x108bb10 sub_108bb10
    Lifted::new(0x7001777b30, tpl_984::<0x700177c312> as *const (), 0), // +0x108bb30 sub_108bb30
    Lifted::new(0x7001777b50, tpl_984::<0x700177c312> as *const (), 0), // +0x108bb50 sub_108bb50
    Lifted::new(0x7001777b70, tpl_984::<0x700177c312> as *const (), 0), // +0x108bb70 sub_108bb70
    Lifted::new(0x7001777b90, tpl_919::<0x700177c312> as *const (), 0), // +0x108bb90 sub_108bb90
    Lifted::new(0x7001777bb0, tpl_919::<0x700177c312> as *const (), 0), // +0x108bbb0 sub_108bbb0
    Lifted::new(0x7001777bd0, tpl_2869::<0x7000791c20, 0x700074a1e0, 0x70006ef280, 0x7001769a20, 0x700077b690, 0x70006f5350> as *const (), 0), // +0x108bbd0 sub_108bbd0
    Lifted::new(0x7001777d70, tpl_2870::<0x8, 0x10, 0x700079b1d0, 0x700151dd20> as *const (), 0), // +0x108bd70 sub_108bd70
    Lifted::new(0x7001777de0, fn_7001777de0 as *const (), 0), // +0x108bde0 sub_108bde0
    Lifted::new(0x7001777eb0, tpl_2871::<0x10, 0x8, 0x7001775bd0> as *const (), 0), // +0x108beb0 sub_108beb0
    Lifted::new(0x7001777ed0, fn_7001777ed0 as *const (), 0), // +0x108bed0 sub_108bed0
    Lifted::new(0x7001777ef0, tpl_2872::<0x7001750f30, 0x40, 0x80, 0x7001750fd0, 0x18, 0x10, 0x8, 0x7001766fb0, 0x7001751330> as *const (), 0), // +0x108bef0 sub_108bef0
    Lifted::new(0x7001777fa0, fn_7001777fa0 as *const (), 0), // +0x108bfa0 sub_108bfa0
    Lifted::new(0x7001778010, tpl_2872::<0x7001750f30, 0x40, 0x80, 0x7001750fd0, 0x18, 0x10, 0x8, 0x7001765650, 0x7001751330> as *const (), 0), // +0x108c010 sub_108c010
    Lifted::new(0x70017780c0, tpl_2869::<0x7000791c20, 0x700074a1e0, 0x700077b690, 0x7001769a20, 0x70006ef280, 0x70006f5350> as *const (), 0), // +0x108c0c0 sub_108c0c0
    Lifted::new(0x7001778260, tpl_2869::<0x7000791c20, 0x700074a1e0, 0x700077b690, 0x7001769a20, 0x70006ef280, 0x70006f5350> as *const (), 0), // +0x108c260 sub_108c260
    Lifted::new(0x7001778400, tpl_2871::<0x10, 0x8, 0x7001775ad0> as *const (), 0), // +0x108c400 sub_108c400
    Lifted::new(0x7001778420, tpl_2870::<0x8, 0x10, 0x700079b1d0, 0x700151d4f0> as *const (), 0), // +0x108c420 sub_108c420
    Lifted::new(0x7001778490, tpl_52::<0x8> as *const (), 1), // +0x108c490 sub_108c490
    Lifted::new(0x70017784a0, tpl_52::<0x8> as *const (), 1), // +0x108c4a0 sub_108c4a0
    Lifted::new(0x70017784b0, tpl_52::<0x8> as *const (), 1), // +0x108c4b0 sub_108c4b0
    Lifted::new(0x70017784c0, tpl_52::<0x8> as *const (), 1), // +0x108c4c0 sub_108c4c0
    Lifted::new(0x70017784d0, tpl_52::<0x8> as *const (), 1), // +0x108c4d0 sub_108c4d0
    Lifted::new(0x70017784e0, tpl_52::<0x8> as *const (), 1), // +0x108c4e0 sub_108c4e0
    Lifted::new(0x70017784f0, tpl_52::<0x8> as *const (), 1), // +0x108c4f0 sub_108c4f0
    Lifted::new(0x7001778500, tpl_52::<0x8> as *const (), 1), // +0x108c500 sub_108c500
    Lifted::new(0x7001778510, tpl_52::<0x8> as *const (), 1), // +0x108c510 sub_108c510
    Lifted::new(0x7001778520, tpl_52::<0x8> as *const (), 1), // +0x108c520 sub_108c520
    Lifted::new(0x7001778530, tpl_52::<0x8> as *const (), 1), // +0x108c530 sub_108c530
    Lifted::new(0x7001778540, tpl_52::<0x8> as *const (), 1), // +0x108c540 sub_108c540
    Lifted::new(0x7001778550, tpl_980::<0x70018da408, 0x8, 0x18> as *const (), 3), // +0x108c550 sub_108c550
    Lifted::new(0x7001778580, tpl_918::<0x70018da288, 0x8> as *const (), 3), // +0x108c580 sub_108c580
    Lifted::new(0x70017785a0, tpl_979::<0x70018da448, 0x8, 0x18> as *const (), 3), // +0x108c5a0 sub_108c5a0
    Lifted::new(0x70017785d0, tpl_918::<0x70018da308, 0x8> as *const (), 3), // +0x108c5d0 sub_108c5d0
    Lifted::new(0x70017785f0, tpl_918::<0x70018da2c8, 0x8> as *const (), 3), // +0x108c5f0 sub_108c5f0
    Lifted::new(0x7001778610, tpl_980::<0x70018da4c8, 0x8, 0x18> as *const (), 3), // +0x108c610 sub_108c610
    Lifted::new(0x7001778640, tpl_918::<0x70018da208, 0x8> as *const (), 3), // +0x108c640 sub_108c640
    Lifted::new(0x7001778660, tpl_980::<0x70018da488, 0x8, 0x18> as *const (), 3), // +0x108c660 sub_108c660
    Lifted::new(0x7001778690, tpl_980::<0x70018da3c8, 0x8, 0x18> as *const (), 3), // +0x108c690 sub_108c690
    Lifted::new(0x70017786c0, tpl_980::<0x70018da388, 0x8, 0x18> as *const (), 3), // +0x108c6c0 sub_108c6c0
    Lifted::new(0x70017786f0, tpl_918::<0x70018da348, 0x8> as *const (), 3), // +0x108c6f0 sub_108c6f0
    Lifted::new(0x7001778710, tpl_918::<0x70018da248, 0x8> as *const (), 3), // +0x108c710 sub_108c710
    Lifted::new(0x7001778730, tpl_315::<0x7001c6e6e0> as *const (), 1), // +0x108c730 sub_108c730
    Lifted::new(0x7001778740, tpl_315::<0x7001c6e8c0> as *const (), 1), // +0x108c740 sub_108c740
    Lifted::new(0x7001778750, tpl_315::<0x7001c6e690> as *const (), 1), // +0x108c750 sub_108c750
    Lifted::new(0x7001778760, tpl_315::<0x7001c6e820> as *const (), 1), // +0x108c760 sub_108c760
    Lifted::new(0x7001778770, tpl_315::<0x7001c6e870> as *const (), 1), // +0x108c770 sub_108c770
    Lifted::new(0x7001778780, tpl_315::<0x7001c6e5f0> as *const (), 1), // +0x108c780 sub_108c780
    Lifted::new(0x7001778790, tpl_315::<0x7001c6e960> as *const (), 1), // +0x108c790 sub_108c790
    Lifted::new(0x70017787a0, tpl_315::<0x7001c6e640> as *const (), 1), // +0x108c7a0 sub_108c7a0
    Lifted::new(0x70017787b0, tpl_315::<0x7001c6e730> as *const (), 1), // +0x108c7b0 sub_108c7b0
    Lifted::new(0x70017787c0, tpl_315::<0x7001c6e780> as *const (), 1), // +0x108c7c0 sub_108c7c0
    Lifted::new(0x70017787d0, tpl_315::<0x7001c6e7d0> as *const (), 1), // +0x108c7d0 sub_108c7d0
    Lifted::new(0x70017787e0, tpl_315::<0x7001c6e910> as *const (), 1), // +0x108c7e0 sub_108c7e0
    Lifted::new(0x70017787f0, tpl_328::<0x20> as *const (), 1), // +0x108c7f0 sub_108c7f0
    Lifted::new(0x7001778800, fn_7001778800 as *const (), 0), // +0x108c800 sub_108c800
    Lifted::new(0x7001778d60, fn_7001778d60 as *const (), 0), // +0x108cd60 sub_108cd60
    Lifted::new(0x7001778d70, fn_7001778d70 as *const (), 0), // +0x108cd70 sub_108cd70
    Lifted::new(0x7001778d80, fn_7001778d80 as *const (), 0), // +0x108cd80 sub_108cd80
    Lifted::new(0x7001778f4c, fn_7001778f4c as *const (), 0), // +0x108cf4c sub_108cf4c
    Lifted::new(0x7001778f94, fn_7001778f94 as *const (), 0), // +0x108cf94 sub_108cf94
    Lifted::new(0x7001778fac, fn_7001778fac as *const (), 0), // +0x108cfac sub_108cfac
    Lifted::new(0x7001778fe0, fn_7001778fe0 as *const (), 0), // +0x108cfe0 sub_108cfe0
    Lifted::new(0x70017792d8, fn_70017792d8 as *const (), 0), // +0x108d2d8 sub_108d2d8
    Lifted::new(0x700177931c, tpl_2873::<0x8, 0x7001c8e330, 0x10, 0x7001da1020, 0x700177c312> as *const (), 0), // +0x108d31c sub_108d31c
    Lifted::new(0x7001779390, fn_7001779390 as *const (), 0), // +0x108d390 sub_108d390
    Lifted::new(0x7001779400, fn_7001779400 as *const (), 0), // +0x108d400 sub_108d400
    Lifted::new(0x70017798a0, fn_70017798a0 as *const (), 3), // +0x108d8a0 sub_108d8a0
    Lifted::new(0x70017799b0, fn_70017799b0 as *const (), 3), // +0x108d9b0 sub_108d9b0
    Lifted::new(0x7001779ad0, fn_7001779ad0 as *const (), 3), // +0x108dad0 sub_108dad0
    Lifted::new(0x7001779bd0, fn_7001779bd0 as *const (), 3), // +0x108dbd0 sub_108dbd0
    Lifted::new(0x7001779cb8, fn_7001779cb8 as *const (), 0), // +0x108dcb8 sub_108dcb8
    Lifted::new(0x7001779cfc, fn_7001779cfc as *const (), 2), // +0x108dcfc sub_108dcfc
    Lifted::new(0x7001779e20, fn_7001779e20 as *const (), 0), // +0x108de20 sub_108de20
    Lifted::new(0x7001779ea8, fn_7001779ea8 as *const (), 0), // +0x108dea8 sub_108dea8
    Lifted::new(0x7001779ecc, fn_7001779ecc as *const (), 0), // +0x108decc sub_108decc
    Lifted::new(0x7001779f50, fn_7001779f50 as *const (), 0), // +0x108df50 sub_108df50
    Lifted::new(0x7001779fa0, fn_7001779fa0 as *const (), 1), // +0x108dfa0 sub_108dfa0
    Lifted::new(0x700177a020, fn_700177a020 as *const (), 0), // +0x108e020 sub_108e020
    Lifted::new(0x700177a060, fn_700177a060 as *const (), 0), // +0x108e060 sub_108e060
    Lifted::new(0x700177a164, fn_700177a164 as *const (), 0), // +0x108e164 sub_108e164
    Lifted::new(0x700177a250, fn_700177a250 as *const (), 0), // +0x108e250 sub_108e250
    Lifted::new(0x700177a284, fn_700177a284 as *const (), 0), // +0x108e284 sub_108e284
    Lifted::new(0x700177a2c0, fn_700177a2c0 as *const (), 0), // +0x108e2c0 sub_108e2c0
    Lifted::new(0x700177a340, fn_700177a340 as *const (), 0), // +0x108e340 sub_108e340
    Lifted::new(0x700177a3d4, fn_700177a3d4 as *const (), 0), // +0x108e3d4 sub_108e3d4
    Lifted::new(0x700177a43c, fn_700177a43c as *const (), 0), // +0x108e43c sub_108e43c
    Lifted::new(0x700177a4c0, fn_700177a4c0 as *const (), 0), // +0x108e4c0 sub_108e4c0
    Lifted::new(0x700177a540, fn_700177a540 as *const (), 0), // +0x108e540 sub_108e540
    Lifted::new(0x700177a554, fn_700177a554 as *const (), 0), // +0x108e554 sub_108e554
    Lifted::new(0x700177a5a0, fn_700177a5a0 as *const (), 0), // +0x108e5a0 sub_108e5a0
    Lifted::new(0x700177a6b0, fn_700177a6b0 as *const (), 0), // +0x108e6b0 sub_108e6b0
    Lifted::new(0x700177a6c8, fn_700177a6c8 as *const (), 0), // +0x108e6c8 sub_108e6c8
    Lifted::new(0x700177a78c, fn_700177a78c as *const (), 0), // +0x108e78c sub_108e78c
    Lifted::new(0x700177aa00, fn_700177aa00 as *const (), 0), // +0x108ea00 sub_108ea00
    Lifted::new(0x700177aa18, fn_700177aa18 as *const (), 1), // +0x108ea18 sub_108ea18
    Lifted::new(0x700177ac30, fn_700177ac30 as *const (), 0), // +0x108ec30 sub_108ec30
    Lifted::new(0x700177ac94, fn_700177ac94 as *const (), 0), // +0x108ec94 sub_108ec94
    Lifted::new(0x700177adf0, fn_700177adf0 as *const (), 0), // +0x108edf0 sub_108edf0
    Lifted::new(0x700177ae98, fn_700177ae98 as *const (), 0), // +0x108ee98 sub_108ee98
    Lifted::new(0x700177b040, fn_700177b040 as *const (), 0), // +0x108f040 sub_108f040
    Lifted::new(0x700177b08c, tpl_2078::<0x700177c07c, 0x700177c32a> as *const (), 0), // +0x108f08c sub_108f08c
    Lifted::new(0x700177b154, fn_700177b154 as *const (), 0), // +0x108f154 sub_108f154
    Lifted::new(0x700177b19c, fn_700177b19c as *const (), 0), // +0x108f19c sub_108f19c
    Lifted::new(0x700177b1e0, fn_700177b1e0 as *const (), 0), // +0x108f1e0 sub_108f1e0
    Lifted::new(0x700177b1fc, fn_700177b1fc as *const (), 0), // +0x108f1fc sub_108f1fc
    Lifted::new(0x700177b230, fn_700177b230 as *const (), 0), // +0x108f230 sub_108f230
    Lifted::new(0x700177b24c, fn_700177b24c as *const (), 0), // +0x108f24c sub_108f24c
    Lifted::new(0x700177b2c4, fn_700177b2c4 as *const (), 0), // +0x108f2c4 sub_108f2c4
    Lifted::new(0x700177b300, fn_700177b300 as *const (), 0), // +0x108f300 sub_108f300
    Lifted::new(0x700177b31c, fn_700177b31c as *const (), 0), // +0x108f31c sub_108f31c
    Lifted::new(0x700177b378, fn_700177b378 as *const (), 2), // +0x108f378 sub_108f378
    Lifted::new(0x700177b428, fn_700177b428 as *const (), 1), // +0x108f428 sub_108f428
    Lifted::new(0x700177b4e8, fn_700177b4e8 as *const (), 0), // +0x108f4e8 sub_108f4e8
    Lifted::new(0x700177b518, fn_700177b518 as *const (), 0), // +0x108f518 sub_108f518
    Lifted::new(0x700177b54c, fn_700177b54c as *const (), 0), // +0x108f54c sub_108f54c
    Lifted::new(0x700177b5b8, fn_700177b5b8 as *const (), 0), // +0x108f5b8 sub_108f5b8
    Lifted::new(0x700177b5e0, fn_700177b5e0 as *const (), 1), // +0x108f5e0 sub_108f5e0
    Lifted::new(0x700177b640, fn_700177b640 as *const (), 0), // +0x108f640 sub_108f640
    Lifted::new(0x700177b744, fn_700177b744 as *const (), 0), // +0x108f744 sub_108f744
    Lifted::new(0x700177b80c, tpl_2874::<0x70018da770, 0x8, 0x700177c122, 0x70018da7a8> as *const (), 2), // +0x108f80c sub_108f80c
    Lifted::new(0x700177b858, tpl_2875::<0x10, 0x70018da7c0, 0x8, 0x70018da7a8> as *const (), 1), // +0x108f858 sub_108f858
    Lifted::new(0x700177b880, tpl_2874::<0x70018da770, 0x8, 0x700177c122, 0x70018da7e0> as *const (), 2), // +0x108f880 sub_108f880
    Lifted::new(0x700177b8cc, tpl_2875::<0x10, 0x700179fed8, 0x8, 0x70018da7e0> as *const (), 1), // +0x108f8cc sub_108f8cc
    Lifted::new(0x700177b8f4, fn_700177b8f4 as *const (), 2), // +0x108f8f4 sub_108f8f4
    Lifted::new(0x700177b9a0, tpl_2876::<0x70018da770, 0x700177c128, 0x700177c312> as *const (), 0), // +0x108f9a0 sub_108f9a0
    Lifted::new(0x700177ba00, tpl_2876::<0x70018da770, 0x700177c128, 0x700177c312> as *const (), 0), // +0x108fa00 sub_108fa00
    Lifted::new(0x700177ba60, tpl_2876::<0x70018da770, 0x700177c128, 0x700177c312> as *const (), 0), // +0x108fa60 sub_108fa60
    Lifted::new(0x700177bab4, tpl_2877::<0x700177b858, 0x7001a0f7f0, 0x700177c12e, 0x700177bad3> as *const (), 0), // +0x108fab4 sub_108fab4
    Lifted::new(0x700177badc, tpl_2877::<0x700177b8cc, 0x7001a0f878, 0x700177c12e, 0x700177bafb> as *const (), 0), // +0x108fadc sub_108fadc
    Lifted::new(0x700177bb10, fn_700177bb10 as *const (), 1), // +0x108fb10 sub_108fb10
    Lifted::new(0x700177bcd4, fn_700177bcd4 as *const (), 0), // +0x108fcd4 sub_108fcd4
    Lifted::new(0x700177bd28, fn_700177bd28 as *const (), 0), // +0x108fd28 sub_108fd28
    Lifted::new(0x700177bdb0, fn_700177bdb0 as *const (), 0), // +0x108fdb0 sub_108fdb0
    Lifted::new(0x700177be24, fn_700177be24 as *const (), 0), // +0x108fe24 sub_108fe24
    Lifted::new(0x700177beac, fn_700177beac as *const (), 0), // +0x108feac sub_108feac
    Lifted::new(0x700177bf84, fn_700177bf84 as *const (), 0), // +0x108ff84 sub_108ff84
    Lifted::new(0x700177bf98, tpl_5::<0x7001c8eaa0, 0x700177c140> as *const (), 0), // +0x108ff98 sub_108ff98
    Lifted::new(0x700177bfa8, tpl_315::<0x7001c8eab8> as *const (), 1), // +0x108ffa8 sub_108ffa8
    Lifted::new(0x700177bfb4, fn_700177bfb4 as *const (), 0), // +0x108ffb4 sub_108ffb4
    Lifted::new(0x700177bfd8, tpl_315::<0x70018da730> as *const (), 1), // +0x108ffd8 sub_108ffd8
    Lifted::new(0x700177bfe4, tpl_2878::<0x70018e2520, 0x70018e2628, 0x7001da1020> as *const (), 0), // +0x108ffe4 sub_108ffe4
    Lifted::new(0x700177c030, tpl_2878::<0x70018e2740, 0x70018e2848, 0x7001da1020> as *const (), 0), // +0x1090030 sub_1090030
    Lifted::new(0x700177c07c, tpl_53 as *const (), 1), // +0x109007c sub_109007c
    Lifted::new(0x700177c084, fn_700177c084 as *const (), 1), // +0x1090084 sub_1090084
    Lifted::new(0x700177c410, tpl_44 as *const (), 1), // +0x1090410 sub_1090410
    Lifted::new(0x700177c414, tpl_44 as *const (), 1), // +0x1090414 sub_1090414
    Lifted::new(0x700177c418, tpl_44 as *const (), 1), // +0x1090418 sub_1090418
    Lifted::new(0x700177c41c, tpl_44 as *const (), 1), // +0x109041c sub_109041c
    Lifted::new(0x700177c420, tpl_44 as *const (), 1), // +0x1090420 sub_1090420
    Lifted::new(0x700177c424, tpl_196 as *const (), 1), // +0x1090424 sub_1090424
    Lifted::new(0x700179bdc0, fn_700179bdc0 as *const (), 2), // +0x10afdc0 sub_10afdc0
    Lifted::new(0x700179bde0, fn_700179bde0 as *const (), 2), // +0x10afde0 sub_10afde0
    Lifted::new(0x700179cdee, fn_700179cdee as *const (), 1), // +0x10b0dee sub_10b0dee
    Lifted::new(0x700179ce13, fn_700179ce13 as *const (), 0), // +0x10b0e13 sub_10b0e13
    Lifted::new(0x700179ce2f, fn_700179ce2f as *const (), 0), // +0x10b0e2f sub_10b0e2f
    Lifted::new(0x700179ce48, fn_700179ce48 as *const (), 0), // +0x10b0e48 sub_10b0e48
    Lifted::new(0x700179ce67, fn_700179ce67 as *const (), 0), // +0x10b0e67 sub_10b0e67
    Lifted::new(0x700179ceaa, fn_700179ceaa as *const (), 1), // +0x10b0eaa sub_10b0eaa
    Lifted::new(0x700179ded0, tpl_2873::<0x8, 0x7001c8e330, 0x10, 0x7001da1020, 0x700177c312> as *const (), 0), // +0x10b1ed0 sub_10b1ed0
];
