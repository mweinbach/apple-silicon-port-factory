#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft(0x1424e9800, "ui_1424e9800", fn_1424e9800 as *const ()), // sub_1424e9800
    Replacement::draft(0x1424e98c0, "ui_1424e98c0", fn_1424e98c0 as *const ()), // sub_1424e98c0
    Replacement::draft(0x1424e9950, "ui_1424e9950", fn_1424e9950 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_23
    Replacement::draft_throws(0x1424e9a60, "ui_1424e9a60", fn_1424e9a60 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_22
    Replacement::draft_throws(0x1424e9c20, "ui_1424e9c20", fn_1424e9c20 as *const (), Throws::Caught), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_21
    Replacement::draft_throws(0x1424e9dc0, "ui_1424e9dc0", fn_1424e9dc0 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_20
    Replacement::draft_throws(0x1424e9ea0, "ui_1424e9ea0", fn_1424e9ea0 as *const (), Throws::Caught), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_19
    Replacement::draft_throws(0x1424ea6d0, "ui_1424ea6d0", fn_1424ea6d0 as *const (), Throws::Discard), // sub_1424ea6d0
    Replacement::draft_throws(0x1424eaca0, "ui_1424eaca0", fn_1424eaca0 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_18
    Replacement::draft_throws(0x1424eaee0, "ui_1424eaee0", fn_1424eaee0 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_17
    Replacement::draft(0x1424eb1c0, "ui_1424eb1c0", fn_1424eb1c0 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_16
    Replacement::draft(0x1424eb280, "ui_1424eb280", fn_1424eb280 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_15
    Replacement::draft_throws(0x1424eb2d0, "ui_1424eb2d0", fn_1424eb2d0 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_14
    Replacement::draft_throws(0x1424eb300, "ui_1424eb300", fn_1424eb300 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_13
    Replacement::draft(0x1424eb340, "ui_1424eb340", fn_1424eb340 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_12
    Replacement::draft_throws(0x1424eb470, "ui_1424eb470", fn_1424eb470 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_11
    Replacement::draft_throws(0x1424eb780, "ui_1424eb780", fn_1424eb780 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_10
    Replacement::draft_throws(0x1424eb7b0, "ui_1424eb7b0", fn_1424eb7b0 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_9
    Replacement::draft(0x1424eb7f0, "ui_1424eb7f0", fn_1424eb7f0 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_8
    Replacement::draft_throws(0x1424eb940, "ui_1424eb940", fn_1424eb940 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_7
    Replacement::draft(0x1424ebc80, "ui_1424ebc80", fn_1424ebc80 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_6
    Replacement::draft(0x1424ebd60, "ui_1424ebd60", fn_1424ebd60 as *const ()), // sub_1424ebd60
    Replacement::draft(0x1424ebe30, "ui_1424ebe30", fn_1424ebe30 as *const ()), // sub_1424ebe30
    Replacement::draft(0x1424ebec0, "ui_1424ebec0", fn_1424ebec0 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_5
    Replacement::draft_throws(0x1424ec4e0, "ui_1424ec4e0", fn_1424ec4e0 as *const (), Throws::Discard), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_4
    Replacement::draft(0x1424ec710, "ui_1424ec710", fn_1424ec710 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_3
    Replacement::draft(0x1424ec8c0, "ui_1424ec8c0", fn_1424ec8c0 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_2
    Replacement::draft(0x1424ecb40, "ui_1424ecb40", fn_1424ecb40 as *const ()), // ui::UIRendererBackend::ui::UIRendererBackend::vfunc_1
    Replacement::draft_throws(0x1424ecd80, "ui_1424ecd80", fn_1424ecd80 as *const (), Throws::Discard), // sub_1424ecd80
    Replacement::draft_throws(0x1424ecf70, "ui_1424ecf70", fn_1424ecf70 as *const (), Throws::Discard), // sub_1424ecf70
    Replacement::draft_throws(0x1424ed010, "ui_1424ed010", fn_1424ed010 as *const (), Throws::Discard), // sub_1424ed010
    Replacement::draft_throws(0x1424ed0b0, "ui_1424ed0b0", fn_1424ed0b0 as *const (), Throws::Discard), // sub_1424ed0b0
    Replacement::draft(0x1424ed130, "ui_1424ed130", fn_1424ed130 as *const ()), // sub_1424ed130
    Replacement::draft(0x1424ee240, "ui_1424ee240", fn_1424ee240 as *const ()), // sub_1424ee240
    Replacement::draft(0x1424ee700, "ui_1424ee700", fn_1424ee700 as *const ()), // sub_1424ee700
    Replacement::draft(0x1424ef5f0, "ui_1424ef5f0", fn_1424ef5f0 as *const ()), // sub_1424ef5f0
    Replacement::draft(0x1424ef9e0, "ui_1424ef9e0", fn_1424ef9e0 as *const ()), // sub_1424ef9e0
    Replacement::draft(0x1424efa00, "ui_1424efa00", fn_1424efa00 as *const ()), // sub_1424efa00
    Replacement::draft(0x1424efa30, "ui_1424efa30", fn_1424efa30 as *const ()), // sub_1424efa30
    Replacement::draft(0x1424efb50, "ui_1424efb50", fn_1424efb50 as *const ()), // sub_1424efb50
    Replacement::draft(0x1424efc40, "ui_1424efc40", fn_1424efc40 as *const ()), // sub_1424efc40
    Replacement::draft(0x1424efcb0, "ui_1424efcb0", fn_1424efcb0 as *const ()), // sub_1424efcb0
    Replacement::draft(0x1424efd60, "ui_1424efd60", fn_1424efd60 as *const ()), // sub_1424efd60
    Replacement::draft(0x1424efd90, "ui_1424efd90", fn_1424efd90 as *const ()), // sub_1424efd90
    Replacement::draft(0x1424efe50, "ui_1424efe50", fn_1424efe50 as *const ()), // sub_1424efe50
    Replacement::draft(0x1424efe90, "ui_1424efe90", fn_1424efe90 as *const ()), // sub_1424efe90
    Replacement::draft_throws(0x1424eff30, "ui_1424eff30", fn_1424eff30 as *const (), Throws::Discard), // ui::UIRenderer::ui::UIRenderer::vfunc_3
    Replacement::draft_throws(0x1424f07f0, "ui_1424f07f0", fn_1424f07f0 as *const (), Throws::Caught), // ui::UIRenderer::ui::UIRenderer::vfunc_2
    Replacement::draft(0x1424f0bc0, "ui_1424f0bc0", fn_1424f0bc0 as *const ()), // ui::UIRenderer::ui::UIRenderer::vfunc_0
    Replacement::draft(0x1424f0d00, "ui_1424f0d00", fn_1424f0d00 as *const ()), // sub_1424f0d00
    Replacement::draft(0x1424f0f60, "ui_1424f0f60", fn_1424f0f60 as *const ()), // sub_1424f0f60
    Replacement::draft(0x1424f10f0, "ui_1424f10f0", fn_1424f10f0 as *const ()), // sub_1424f10f0
    Replacement::draft_throws(0x1424f11c0, "ui_1424f11c0", fn_1424f11c0 as *const (), Throws::Caught), // sub_1424f11c0
    Replacement::draft_throws(0x1424f13b0, "ui_1424f13b0", fn_1424f13b0 as *const (), Throws::Caught), // sub_1424f13b0
    Replacement::draft_throws(0x1424f1a30, "ui_1424f1a30", fn_1424f1a30 as *const (), Throws::Discard), // sub_1424f1a30
    Replacement::draft_throws(0x1424f1b20, "ui_1424f1b20", fn_1424f1b20 as *const (), Throws::Caught), // sub_1424f1b20
    Replacement::draft_throws(0x1424f1d60, "ui_1424f1d60", fn_1424f1d60 as *const (), Throws::Discard), // sub_1424f1d60
    Replacement::draft_throws(0x1424f2170, "ui_1424f2170", fn_1424f2170 as *const (), Throws::Caught), // sub_1424f2170
    Replacement::draft_throws(0x1424f24d0, "ui_1424f24d0", fn_1424f24d0 as *const (), Throws::Discard), // sub_1424f24d0
    Replacement::draft_throws(0x1424f2500, "ui_1424f2500", fn_1424f2500 as *const (), Throws::Discard), // sub_1424f2500
    Replacement::draft_throws(0x1424f2610, "ui_1424f2610", fn_1424f2610 as *const (), Throws::Discard), // sub_1424f2610
    Replacement::draft_throws(0x1424f26f0, "ui_1424f26f0", fn_1424f26f0 as *const (), Throws::Caught), // sub_1424f26f0
    Replacement::draft_throws(0x1424f28e0, "ui_1424f28e0", fn_1424f28e0 as *const (), Throws::Discard), // sub_1424f28e0
    Replacement::draft(0x1424f2980, "ui_1424f2980", fn_1424f2980 as *const ()), // sub_1424f2980
    Replacement::draft_throws(0x1424f29e0, "ui_1424f29e0", fn_1424f29e0 as *const (), Throws::Discard), // sub_1424f29e0
    Replacement::draft(0x1424f2b10, "ui_1424f2b10", fn_1424f2b10 as *const ()), // sub_1424f2b10
    Replacement::draft_throws(0x1424f2ba0, "ui_1424f2ba0", fn_1424f2ba0 as *const (), Throws::Caught), // sub_1424f2ba0
    Replacement::draft_throws(0x1424f2e20, "ui_1424f2e20", fn_1424f2e20 as *const (), Throws::Caught), // sub_1424f2e20
    Replacement::draft(0x1424f2fc0, "ui_1424f2fc0", fn_1424f2fc0 as *const ()), // sub_1424f2fc0
    Replacement::draft(0x1424f3020, "ui_1424f3020", fn_1424f3020 as *const ()), // sub_1424f3020
    Replacement::draft(0x1424f3070, "ui_1424f3070", fn_1424f3070 as *const ()), // sub_1424f3070
    Replacement::draft_throws(0x1424f30c0, "ui_1424f30c0", fn_1424f30c0 as *const (), Throws::Discard), // sub_1424f30c0
    Replacement::draft(0x1424f3610, "ui_1424f3610", tpl_38859::<0x1438821d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::TextureAtlasMeta::ClassTypeInfoImplementation::ui::TextureAtlasMeta::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1424f3670, "ui_1424f3670", tpl_38859::<0x1438821d0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::TextureAtlasMeta::ClassTypeInfoImplementation::ui::TextureAtlasMeta::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1424f3720, "ui_1424f3720", fn_1424f3720 as *const ()), // ui::TextureAtlasMeta::ClassTypeInfoImplementation::ui::TextureAtlasMeta::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1424f37a0, "ui_1424f37a0", tpl_39306::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x140e71010, 0x142442c80> as *const ()), // ui::TextureAtlasMeta::ClassTypeInfoImplementation::ui::TextureAtlasMeta::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1424f3800, "ui_1424f3800", tpl_35191::<0x10, 0x140e71010, 0x142442de0> as *const ()), // ui::TextureAtlasMeta::ClassTypeInfoImplementation::ui::TextureAtlasMeta::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1424f3d00, "ui_1424f3d00", fn_1424f3d00 as *const ()), // ui::TextureAtlas::ClassTypeInfoImplementation::ui::TextureAtlas::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1424f3d70, "ui_1424f3d70", tpl_38771::<0x110, 0x141408500, 0x14240ab10> as *const ()), // ui::TextureAtlas::ClassTypeInfoImplementation::ui::TextureAtlas::ClassTypeInfoImplementation::vfunc_13
    Replacement::draft(0x1424f3dd0, "ui_1424f3dd0", tpl_38773::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x110, 0x141408500, 0x14240ab10, 0x142442c80> as *const ()), // ui::TextureAtlas::ClassTypeInfoImplementation::ui::TextureAtlas::ClassTypeInfoImplementation::vfunc_11
    Replacement::draft(0x1424f3e40, "ui_1424f3e40", tpl_38774::<0x110, 0x141408500, 0x14240ab10, 0x142442de0> as *const ()), // ui::TextureAtlas::ClassTypeInfoImplementation::ui::TextureAtlas::ClassTypeInfoImplementation::vfunc_10
    Replacement::draft(0x1424f42c0, "ui_1424f42c0", tpl_38859::<0x1438821c0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::TextureCoordinates::ClassTypeInfoImplementation::ui::TextureCoordinates::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft(0x1424f4320, "ui_1424f4320", tpl_38859::<0x1438821c0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::TextureCoordinates::ClassTypeInfoImplementation::ui::TextureCoordinates::ClassTypeInfoImplementation::vfunc_52
    Replacement::draft(0x1424f43d0, "ui_1424f43d0", tpl_39508::<0x14240aa00, 0x108, 0x10c, 0x110, 0x114> as *const ()), // ui::TextureCoordinates::ClassTypeInfoImplementation::ui::TextureCoordinates::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1424f46d0, "ui_1424f46d0", fn_1424f46d0 as *const ()), // ui::TextTransformationManager::ui::TextTransformationManager::vfunc_1
    Replacement::draft_throws(0x1424f4770, "ui_1424f4770", fn_1424f4770 as *const (), Throws::Caught), // ui::TextTransformationManager::ui::TextTransformationManager::vfunc_2
    Replacement::draft(0x1424f4c40, "unknown_1424f4c40", fn_1424f4c40 as *const ()), // sub_1424f4c40
    Replacement::draft(0x1424f4c60, "unknown_1424f4c60", fn_1424f4c60 as *const ()), // sub_1424f4c60
    Replacement::draft(0x1424f4ea0, "unknown_1424f4ea0", fn_1424f4ea0 as *const ()), // sub_1424f4ea0
    Replacement::draft(0x1424f4fc0, "unknown_1424f4fc0", fn_1424f4fc0 as *const ()), // sub_1424f4fc0
    Replacement::draft(0x1424f5020, "unknown_1424f5020", tpl_29744::<0x8> as *const ()), // sub_1424f5020
    Replacement::draft(0x1424f5060, "unknown_1424f5060", fn_1424f5060 as *const ()), // sub_1424f5060
    Replacement::draft_throws(0x1424f5090, "unknown_1424f5090", tpl_42434::<0x10, 0x8, 0x140200070, 0x1424f50b4> as *const (), Throws::Discard), // sub_1424f5090
    Replacement::draft_throws(0x1424f50d0, "unknown_1424f50d0", fn_1424f50d0 as *const (), Throws::Discard), // sub_1424f50d0
    Replacement::draft_throws(0x1424f5100, "unknown_1424f5100", tpl_42437::<0x10, 0x8, 0x140200070, 0x1424f5124> as *const (), Throws::Discard), // sub_1424f5100
    Replacement::draft(0x1424f5150, "unknown_1424f5150", tpl_40083::<0x10> as *const ()), // sub_1424f5150
    Replacement::draft_throws(0x1424f5160, "unknown_1424f5160", fn_1424f5160 as *const (), Throws::Discard), // sub_1424f5160
    Replacement::draft(0x1424f5300, "unknown_1424f5300", tpl_42439::<0x10> as *const ()), // sub_1424f5300
    Replacement::draft_throws(0x1424f53b0, "unknown_1424f53b0", fn_1424f53b0 as *const (), Throws::Discard), // sub_1424f53b0
    Replacement::draft_throws(0x1424f5500, "unknown_1424f5500", tpl_51872::<0x8, 0x24, 0x10, 0x60, 0x20, 0x18, 0xc, 0x1424f26f0, 0x141410810, 0x30> as *const (), Throws::Discard), // sub_1424f5500
    Replacement::draft_throws(0x1424f55b0, "unknown_1424f55b0", tpl_51873::<0x60, 0x1424f28e0, 0x8, 0x1426455a8, 0x10, 0x142645598, 0x142645590> as *const (), Throws::Discard), // sub_1424f55b0
    Replacement::draft(0x1424f5650, "unknown_1424f5650", tpl_34312::<0x1403c7e70> as *const ()), // sub_1424f5650
    Replacement::draft(0x1424f56e0, "unknown_1424f56e0", fn_1424f56e0 as *const ()), // sub_1424f56e0
    Replacement::draft(0x1424f58b0, "unknown_1424f58b0", fn_1424f58b0 as *const ()), // sub_1424f58b0
    Replacement::draft(0x1424f59f0, "unknown_1424f59f0", fn_1424f59f0 as *const ()), // sub_1424f59f0
    Replacement::draft_throws(0x1424f5af0, "unknown_1424f5af0", fn_1424f5af0 as *const (), Throws::Discard), // sub_1424f5af0
    Replacement::draft(0x1424f5d30, "unknown_1424f5d30", fn_1424f5d30 as *const ()), // sub_1424f5d30
    Replacement::draft_throws(0x1424f5d70, "unknown_1424f5d70", fn_1424f5d70 as *const (), Throws::Discard), // sub_1424f5d70
    Replacement::draft_throws(0x1424f5dc0, "unknown_1424f5dc0", fn_1424f5dc0 as *const (), Throws::Discard), // sub_1424f5dc0
    Replacement::draft(0x1424f5f40, "unknown_1424f5f40", fn_1424f5f40 as *const ()), // sub_1424f5f40
    Replacement::draft(0x1424f6050, "ui_1424f6050", fn_1424f6050 as *const ()), // ui::System::ui::System::vfunc_1
    Replacement::draft_throws(0x1424f6170, "ui_1424f6170", tpl_51883::<0x141410810, 0x28, 0x1d0> as *const (), Throws::Discard), // sub_1424f6170
    Replacement::draft_throws(0x1424f61b0, "ui_1424f61b0", fn_1424f61b0 as *const (), Throws::Discard), // sub_1424f61b0
    Replacement::draft_throws(0x1424f6260, "ui_1424f6260", fn_1424f6260 as *const (), Throws::Discard), // sub_1424f6260
    Replacement::draft_throws(0x1424f6310, "ui_1424f6310", fn_1424f6310 as *const (), Throws::Discard), // sub_1424f6310
    Replacement::draft_throws(0x1424f6400, "ui_1424f6400", fn_1424f6400 as *const (), Throws::Discard), // sub_1424f6400
    Replacement::draft(0x1424f6440, "ui_1424f6440", fn_1424f6440 as *const ()), // sub_1424f6440
    Replacement::draft(0x1424f6460, "ui_1424f6460", tpl_51889::<0x60, 0x30, 0x1426455a8, 0x38, 0x50, 0x10, 0x14, 0x1411d5000, 0xc, 0x8, 0x142645598> as *const ()), // sub_1424f6460
    Replacement::draft_throws(0x1424f66e0, "ui_1424f66e0", fn_1424f66e0 as *const (), Throws::Discard), // sub_1424f66e0
    Replacement::draft(0x1424f6710, "ui_1424f6710", tpl_47379::<0x8, 0xc> as *const ()), // sub_1424f6710
    Replacement::draft(0x1424f67a0, "ui_1424f67a0", fn_1424f67a0 as *const ()), // sub_1424f67a0
    Replacement::draft(0x1424f67e0, "ui_1424f67e0", fn_1424f67e0 as *const ()), // sub_1424f67e0
    Replacement::draft(0x1424f6880, "ui_1424f6880", fn_1424f6880 as *const ()), // sub_1424f6880
    Replacement::draft_throws(0x1424f6980, "ui_1424f6980", fn_1424f6980 as *const (), Throws::Caught), // sub_1424f6980
    Replacement::draft_throws(0x1424f6b30, "ui_1424f6b30", fn_1424f6b30 as *const (), Throws::Discard), // sub_1424f6b30
    Replacement::draft_throws(0x1424f6c30, "ui_1424f6c30", tpl_51872::<0x8, 0x24, 0x10, 0x60, 0x20, 0x18, 0xc, 0x1424f26f0, 0x141410810, 0x30> as *const (), Throws::Discard), // sub_1424f6c30
    Replacement::draft_throws(0x1424f6c90, "ui_1424f6c90", fn_1424f6c90 as *const (), Throws::Caught), // sub_1424f6c90
    Replacement::draft(0x1424f71f0, "ui_1424f71f0", tpl_51889::<0x60, 0x30, 0x1426455a8, 0x38, 0x50, 0x10, 0x14, 0x1411d5000, 0xc, 0x8, 0x142645598> as *const ()), // sub_1424f71f0
    Replacement::draft(0x1424f72b0, "ui_1424f72b0", fn_1424f72b0 as *const ()), // sub_1424f72b0
    Replacement::draft_throws(0x1424f7340, "ui_1424f7340", tpl_51873::<0x60, 0x1424f28e0, 0x8, 0x1426455a8, 0x10, 0x142645598, 0x142645590> as *const (), Throws::Discard), // sub_1424f7340
    Replacement::draft_throws(0x1424f7390, "ui_1424f7390", fn_1424f7390 as *const (), Throws::Caught), // sub_1424f7390
    Replacement::draft(0x1424f77a0, "ui_1424f77a0", fn_1424f77a0 as *const ()), // sub_1424f77a0
    Replacement::draft(0x1424f78d0, "ui_1424f78d0", fn_1424f78d0 as *const ()), // sub_1424f78d0
    Replacement::draft(0x1424f7920, "ui_1424f7920", fn_1424f7920 as *const ()), // sub_1424f7920
    Replacement::draft(0x1424f79e0, "ui_1424f79e0", fn_1424f79e0 as *const ()), // sub_1424f79e0
    Replacement::draft(0x1424f7a20, "ui_1424f7a20", fn_1424f7a20 as *const ()), // sub_1424f7a20
    Replacement::draft(0x1424f7ad0, "ui_1424f7ad0", fn_1424f7ad0 as *const ()), // sub_1424f7ad0
    Replacement::draft(0x1424f7b10, "ui_1424f7b10", fn_1424f7b10 as *const ()), // sub_1424f7b10
    Replacement::draft(0x1424f7b50, "ui_1424f7b50", fn_1424f7b50 as *const ()), // sub_1424f7b50
    Replacement::draft_throws(0x1424f7c60, "ui_1424f7c60", fn_1424f7c60 as *const (), Throws::Discard), // sub_1424f7c60
    Replacement::draft_throws(0x1424f7de0, "ui_1424f7de0", tpl_51906::<0x141410810, 0x28> as *const (), Throws::Discard), // sub_1424f7de0
    Replacement::draft_throws(0x1424f7e10, "ui_1424f7e10", fn_1424f7e10 as *const (), Throws::Discard), // sub_1424f7e10
    Replacement::draft_throws(0x1424f7e40, "ui_1424f7e40", tpl_51906::<0x141410810, 0x18> as *const (), Throws::Discard), // sub_1424f7e40
    Replacement::draft_throws(0x1424f7e70, "ui_1424f7e70", tpl_51906::<0x141410810, 0x10> as *const (), Throws::Discard), // sub_1424f7e70
    Replacement::draft_throws(0x1424f7ea0, "ui_1424f7ea0", fn_1424f7ea0 as *const (), Throws::Discard), // sub_1424f7ea0
    Replacement::draft(0x1424f7ec0, "ui_1424f7ec0", fn_1424f7ec0 as *const ()), // sub_1424f7ec0
    Replacement::draft(0x1424f7fd0, "ui_1424f7fd0", tpl_51910::<0x1424069a0, 0x143426d28> as *const ()), // sub_1424f7fd0
    Replacement::draft(0x1424f7ff0, "ui_1424f7ff0", tpl_51910::<0x1424069a0, 0x143426d28> as *const ()), // sub_1424f7ff0
    Replacement::draft_throws(0x1424f8010, "ui_1424f8010", fn_1424f8010 as *const (), Throws::Discard), // sub_1424f8010
    Replacement::draft(0x1424f8070, "ui_1424f8070", fn_1424f8070 as *const ()), // sub_1424f8070
    Replacement::draft_throws(0x1424f8170, "ui_1424f8170", tpl_51883::<0x141410810, 0x28, 0x250> as *const (), Throws::Discard), // sub_1424f8170
    Replacement::draft_throws(0x1424f81b0, "ui_1424f81b0", fn_1424f81b0 as *const (), Throws::Discard), // sub_1424f81b0
    Replacement::draft_throws(0x1424f81e0, "ui_1424f81e0", fn_1424f81e0 as *const (), Throws::Discard), // sub_1424f81e0
    Replacement::draft(0x1424f86e0, "ui_1424f86e0", fn_1424f86e0 as *const ()), // sub_1424f86e0
    Replacement::draft_throws(0x1424f8700, "ui_1424f8700", fn_1424f8700 as *const (), Throws::Discard), // sub_1424f8700
    Replacement::draft(0x1424f8b90, "ui_1424f8b90", fn_1424f8b90 as *const ()), // sub_1424f8b90
    Replacement::draft(0x1424f8c30, "ui_1424f8c30", tpl_26429::<0x142442de0> as *const ()), // ui::SyncStreamReader::ui::SyncStreamReader::vfunc_3
    Replacement::draft(0x1424f8c60, "ui_1424f8c60", fn_1424f8c60 as *const ()), // ui::SyncStreamReader::ui::SyncStreamReader::vfunc_2
    Replacement::draft(0x1424f8ca0, "ui_1424f8ca0", fn_1424f8ca0 as *const ()), // sub_1424f8ca0
    Replacement::draft(0x1424f8e10, "ui_1424f8e10", fn_1424f8e10 as *const ()), // sub_1424f8e10
    Replacement::draft(0x1424f8ee0, "ui_1424f8ee0", fn_1424f8ee0 as *const ()), // sub_1424f8ee0
    Replacement::draft(0x1424f8f00, "ui_1424f8f00", fn_1424f8f00 as *const ()), // sub_1424f8f00
    Replacement::draft_throws(0x1424f9140, "ui_1424f9140", fn_1424f9140 as *const (), Throws::Caught), // sub_1424f9140
    Replacement::draft(0x1424f9370, "ui_1424f9370", fn_1424f9370 as *const ()), // sub_1424f9370
    Replacement::draft(0x1424f9420, "ui_1424f9420", fn_1424f9420 as *const ()), // sub_1424f9420
    Replacement::draft(0x1424f94e0, "ui_1424f94e0", fn_1424f94e0 as *const ()), // sub_1424f94e0
    Replacement::draft(0x1424f96e0, "ui_1424f96e0", fn_1424f96e0 as *const ()), // sub_1424f96e0
    Replacement::draft(0x1424f9d60, "ui_1424f9d60", tpl_51927::<0x10> as *const ()), // sub_1424f9d60
    Replacement::draft(0x1424f9d80, "ui_1424f9d80", tpl_21787::<0x10> as *const ()), // sub_1424f9d80
    Replacement::draft(0x1424f9d90, "ui_1424f9d90", tpl_51927::<0x40> as *const ()), // sub_1424f9d90
    Replacement::draft(0x1424f9db0, "ui_1424f9db0", tpl_21787::<0x40> as *const ()), // sub_1424f9db0
    Replacement::draft(0x1424f9dc0, "ui_1424f9dc0", fn_1424f9dc0 as *const ()), // sub_1424f9dc0
    Replacement::draft(0x1424f9e00, "ui_1424f9e00", fn_1424f9e00 as *const ()), // sub_1424f9e00
    Replacement::draft_throws(0x1424fa1a0, "ui_1424fa1a0", fn_1424fa1a0 as *const (), Throws::Caught), // sub_1424fa1a0
    Replacement::draft_throws(0x1424fa690, "ui_1424fa690", fn_1424fa690 as *const (), Throws::Caught), // ui::BlobResourceHandler::ui::BlobResourceHandler::vfunc_2
    Replacement::draft_throws(0x1424fa820, "ui_1424fa820", fn_1424fa820 as *const (), Throws::Caught), // ui::BlobResourceHandler::ui::BlobResourceHandler::vfunc_1
    Replacement::draft(0x1424fa8d0, "ui_1424fa8d0", fn_1424fa8d0 as *const ()), // sub_1424fa8d0
    Replacement::draft_throws(0x1424fa990, "ui_1424fa990", fn_1424fa990 as *const (), Throws::Caught), // sub_1424fa990
    Replacement::draft(0x1424faa90, "std_1424faa90", tpl_550::<0x143417790, 0x8> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Pag
    Replacement::draft(0x1424faab0, "std_1424faab0", tpl_550::<0x143417790, 0x8> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Pag
    Replacement::draft(0x1424faae0, "std_1424faae0", tpl_1::<0x14389b790> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Pag
    Replacement::draft(0x1424faaf0, "std_1424faaf0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Pag
    Replacement::draft(0x1424fab00, "std_1424fab00", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::blockingLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Pag
    Replacement::draft(0x1424fab20, "std_1424fab20", tpl_550::<0x143418b10, 0x8> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Page
    Replacement::draft(0x1424fab40, "std_1424fab40", tpl_550::<0x143418b10, 0x8> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Page
    Replacement::draft(0x1424fab70, "std_1424fab70", tpl_1::<0x14389b740> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Page
    Replacement::draft(0x1424fab80, "std_1424fab80", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Page
    Replacement::draft(0x1424fab90, "std_1424fab90", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<ui::PageResource::requestLoad::<lambda_1>,void,r::BaseFile&>::std::_Func_impl_no_alloc<ui::Page
    Replacement::draft(0x1424fabb0, "unknown_1424fabb0", fn_1424fabb0 as *const ()), // sub_1424fabb0
    Replacement::draft(0x1424fae00, "ui_1424fae00", fn_1424fae00 as *const ()), // sub_1424fae00
    Replacement::draft(0x1424faeb0, "ui_1424faeb0", fn_1424faeb0 as *const ()), // sub_1424faeb0
    Replacement::draft(0x1424fb330, "ui_1424fb330", tpl_35199::<0x1438821e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::EventDefinition::ClassTypeInfoImplementation::ui::PageResource::EventDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x1424fb390, "ui_1424fb390", tpl_35199::<0x1438821e8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::EventDefinition::ClassTypeInfoImplementation::ui::PageResource::EventDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x1424fb440, "ui_1424fb440", tpl_38770::<0x14240aa00, 0x28, 0x14053f5f0> as *const ()), // ui::PageResource::EventDefinition::ClassTypeInfoImplementation::ui::PageResource::EventDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x1424fb490, "ui_1424fb490", tpl_38771::<0x28, 0x1401f2bc0, 0x14240ab10> as *const ()), // ui::PageResource::EventDefinition::ClassTypeInfoImplementation::ui::PageResource::EventDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x1424fb4f0, "ui_1424fb4f0", tpl_38773::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x28, 0x1401f2bc0, 0x14240ab10, 0x142442c80> as *const ()), // ui::PageResource::EventDefinition::ClassTypeInfoImplementation::ui::PageResource::EventDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x1424fb560, "ui_1424fb560", tpl_38774::<0x28, 0x1401f2bc0, 0x14240ab10, 0x142442de0> as *const ()), // ui::PageResource::EventDefinition::ClassTypeInfoImplementation::ui::PageResource::EventDefinition::ClassTypeInfoImplemen
    Replacement::draft(0x1424fb9e0, "ui_1424fb9e0", tpl_35219::<0x143882218, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::NavigationContextDefinition::ClassTypeInfoImplementation::ui::PageResource::NavigationContextDefinitio
    Replacement::draft(0x1424fba40, "ui_1424fba40", tpl_35219::<0x143882218, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::NavigationContextDefinition::ClassTypeInfoImplementation::ui::PageResource::NavigationContextDefinitio
    Replacement::draft(0x1424fbaf0, "ui_1424fbaf0", fn_1424fbaf0 as *const ()), // ui::PageResource::NavigationContextDefinition::ClassTypeInfoImplementation::ui::PageResource::NavigationContextDefinitio
    Replacement::draft(0x1424fbb50, "ui_1424fbb50", tpl_38952::<0x28, 0x14240ab10> as *const ()), // ui::PageResource::NavigationContextDefinition::ClassTypeInfoImplementation::ui::PageResource::NavigationContextDefinitio
    Replacement::draft(0x1424fbbc0, "ui_1424fbbc0", tpl_98840::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x28, 0x14240ab10, 0x142442c80> as *const ()), // ui::PageResource::NavigationContextDefinition::ClassTypeInfoImplementation::ui::PageResource::NavigationContextDefinitio
    Replacement::draft(0x1424fbc30, "ui_1424fbc30", tpl_38954::<0x28, 0x14240ab10, 0x142442de0> as *const ()), // ui::PageResource::NavigationContextDefinition::ClassTypeInfoImplementation::ui::PageResource::NavigationContextDefinitio
    Replacement::draft(0x1424fbf70, "ui_1424fbf70", tpl_38845::<0x8, 0x14240aa00, 0x28> as *const ()), // ui::PageResource::StateStackDefinition::ClassTypeInfoImplementation::ui::PageResource::StateStackDefinition::ClassTypeIn
    Replacement::draft(0x1424fbfc0, "ui_1424fbfc0", tpl_38845::<0x8, 0x14240aa00, 0x28> as *const ()), // ui::PageResource::StateStackDefinition::ClassTypeInfoImplementation::ui::PageResource::StateStackDefinition::ClassTypeIn
    Replacement::draft(0x1424fc110, "ui_1424fc110", tpl_35199::<0x1438821f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::StateStackDefinition::ClassTypeInfoImplementation::ui::PageResource::StateStackDefinition::ClassTypeIn
    Replacement::draft(0x1424fc170, "ui_1424fc170", tpl_35199::<0x1438821f0, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::StateStackDefinition::ClassTypeInfoImplementation::ui::PageResource::StateStackDefinition::ClassTypeIn
    Replacement::draft(0x1424fc220, "ui_1424fc220", tpl_38846::<0x14240aa00, 0x28> as *const ()), // ui::PageResource::StateStackDefinition::ClassTypeInfoImplementation::ui::PageResource::StateStackDefinition::ClassTypeIn
    Replacement::draft(0x1424fc800, "ui_1424fc800", tpl_38770::<0x14240aa00, 0x28, 0x14142ab90> as *const ()), // ui::PageResource::TransformArgumentDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentDefinitio
    Replacement::draft(0x1424fc850, "ui_1424fc850", tpl_38771::<0x28, 0x14115d8c0, 0x14240ab10> as *const ()), // ui::PageResource::TransformArgumentDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentDefinitio
    Replacement::draft(0x1424fc8b0, "ui_1424fc8b0", tpl_38773::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x28, 0x14115d8c0, 0x14240ab10, 0x142442c80> as *const ()), // ui::PageResource::TransformArgumentDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentDefinitio
    Replacement::draft(0x1424fc920, "ui_1424fc920", tpl_38774::<0x28, 0x14115d8c0, 0x14240ab10, 0x142442de0> as *const ()), // ui::PageResource::TransformArgumentDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentDefinitio
    Replacement::draft_throws(0x1424fca20, "ui_1424fca20", tpl_51939::<0x143419970, 0x1, 0x14086a110> as *const (), Throws::Discard), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft_throws(0x1424fca60, "ui_1424fca60", tpl_51939::<0x143419970, 0x1, 0x14086a110> as *const (), Throws::Discard), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft(0x1424fccc0, "ui_1424fccc0", fn_1424fccc0 as *const ()), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft(0x1424fcd00, "ui_1424fcd00", fn_1424fcd00 as *const ()), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft(0x1424fce40, "ui_1424fce40", tpl_35199::<0x143882208, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft(0x1424fcea0, "ui_1424fcea0", tpl_35199::<0x143882208, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft(0x1424fcf50, "ui_1424fcf50", fn_1424fcf50 as *const ()), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft(0x1424fcfd0, "ui_1424fcfd0", fn_1424fcfd0 as *const ()), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft(0x1424fd030, "ui_1424fd030", tpl_35184::<0x14, 0x14240ab10, 0x142442de0> as *const ()), // ui::PageResource::TransformArgumentValueDefinition::ClassTypeInfoImplementation::ui::PageResource::TransformArgumentValu
    Replacement::draft_throws(0x1424fd120, "ui_1424fd120", tpl_51943::<0x143419930, 0x1, 0x14086a110> as *const (), Throws::Discard), // ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingObjectListD
    Replacement::draft_throws(0x1424fd160, "ui_1424fd160", tpl_51943::<0x143419930, 0x1, 0x14086a110> as *const (), Throws::Discard), // ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingObjectListD
    Replacement::draft(0x1424fd500, "ui_1424fd500", tpl_35199::<0x1438821f8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingObjectListD
    Replacement::draft(0x1424fd560, "ui_1424fd560", tpl_35199::<0x1438821f8, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingObjectListD
    Replacement::draft(0x1424fd610, "ui_1424fd610", tpl_51944::<0x14240aa00, 0x28, 0x50, 0x78, 0x8c, 0x7c> as *const ()), // ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingObjectListD
    Replacement::draft(0x1424fd690, "ui_1424fd690", tpl_39319::<0x8c, 0x14240ab10, 0x50, 0x28> as *const ()), // ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingObjectListD
    Replacement::draft(0x1424fd750, "ui_1424fd750", tpl_99113::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x8c, 0x14240ab10, 0x50, 0x28, 0x142442c80> as *const ()), // ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingObjectListD
    Replacement::draft(0x1424fd7d0, "ui_1424fd7d0", tpl_39321::<0x8c, 0x14240ab10, 0x50, 0x28, 0x142442de0> as *const ()), // ui::PageResource::FactBindingObjectListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingObjectListD
    Replacement::draft_throws(0x1424fd8e0, "ui_1424fd8e0", tpl_51945::<0x143419930, 0x1, 0x14086a110> as *const (), Throws::Discard), // ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingListDefinition::C
    Replacement::draft_throws(0x1424fd920, "ui_1424fd920", tpl_51945::<0x143419930, 0x1, 0x14086a110> as *const (), Throws::Discard), // ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingListDefinition::C
    Replacement::draft(0x1424fdcc0, "ui_1424fdcc0", tpl_35199::<0x143882270, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingListDefinition::C
    Replacement::draft(0x1424fdd20, "ui_1424fdd20", tpl_35199::<0x143882270, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingListDefinition::C
    Replacement::draft(0x1424fddd0, "ui_1424fddd0", fn_1424fddd0 as *const ()), // ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingListDefinition::C
    Replacement::draft(0x1424fde40, "ui_1424fde40", tpl_38861::<0x64, 0x14240ab10, 0x28> as *const ()), // ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingListDefinition::C
    Replacement::draft(0x1424fded0, "ui_1424fded0", tpl_98785::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x64, 0x14240ab10, 0x28, 0x142442c80> as *const ()), // ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingListDefinition::C
    Replacement::draft(0x1424fdf40, "ui_1424fdf40", tpl_38863::<0x64, 0x14240ab10, 0x28, 0x142442de0> as *const ()), // ui::PageResource::FactBindingListDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingListDefinition::C
    Replacement::draft_throws(0x1424fe040, "ui_1424fe040", tpl_51943::<0x143419930, 0x1, 0x14086a110> as *const (), Throws::Discard), // ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingDefinition::ClassType
    Replacement::draft_throws(0x1424fe080, "ui_1424fe080", tpl_51943::<0x143419930, 0x1, 0x14086a110> as *const (), Throws::Discard), // ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingDefinition::ClassType
    Replacement::draft(0x1424fe420, "ui_1424fe420", tpl_35199::<0x143882278, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingDefinition::ClassType
    Replacement::draft(0x1424fe480, "ui_1424fe480", tpl_35199::<0x143882278, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingDefinition::ClassType
    Replacement::draft(0x1424fe530, "ui_1424fe530", tpl_51944::<0x14240aa00, 0x28, 0x50, 0x78, 0x8c, 0x7c> as *const ()), // ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingDefinition::ClassType
    Replacement::draft(0x1424fe5b0, "ui_1424fe5b0", tpl_39319::<0x8c, 0x14240ab10, 0x50, 0x28> as *const ()), // ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingDefinition::ClassType
    Replacement::draft(0x1424fe670, "ui_1424fe670", tpl_99113::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x8c, 0x14240ab10, 0x50, 0x28, 0x142442c80> as *const ()), // ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingDefinition::ClassType
    Replacement::draft(0x1424fe6f0, "ui_1424fe6f0", tpl_39321::<0x8c, 0x14240ab10, 0x50, 0x28, 0x142442de0> as *const ()), // ui::PageResource::FactBindingDefinition::ClassTypeInfoImplementation::ui::PageResource::FactBindingDefinition::ClassType
    Replacement::draft(0x1424fe7e0, "ui_1424fe7e0", tpl_51947::<0x1434198e0, 0x143263dd0, 0x143275db4, 0x14088ba10> as *const ()), // ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1424fe820, "ui_1424fe820", tpl_51947::<0x1434198e0, 0x143263dd0, 0x143275db4, 0x14088ba10> as *const ()), // ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1424feaa0, "ui_1424feaa0", tpl_51948::<0x8, 0x14240aa00, 0x28, 0x1401e84a0> as *const ()), // ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1424feaf0, "ui_1424feaf0", tpl_51948::<0x8, 0x14240aa00, 0x28, 0x1401e84a0> as *const ()), // ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1424fed30, "ui_1424fed30", tpl_38770::<0x14240aa00, 0x28, 0x1401e84a0> as *const ()), // ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1424fed80, "ui_1424fed80", tpl_39521::<0x30, 0x14240ab10> as *const ()), // ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1424fedd0, "ui_1424fedd0", tpl_99273::<0xfffffffffffffff8, 0xfffffffffffffff0, 0x30, 0x14240ab10, 0x142442c80> as *const ()), // ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1424fee30, "ui_1424fee30", tpl_39523::<0x30, 0x14240ab10, 0x142442de0> as *const ()), // ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::ui::PageMetadata::PageCurve::ClassTypeInfoImplementation::vfun
    Replacement::draft(0x1424fef40, "ui_1424fef40", tpl_51949::<0x143879730, 0x143264170, 0x143419850, 0x1432a72f0, 0x1401de410> as *const ()), // ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementation::ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementati
    Replacement::draft(0x1424fefa0, "ui_1424fefa0", tpl_51949::<0x143879730, 0x143264170, 0x143419850, 0x1432a72f0, 0x1401de410> as *const ()), // ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementation::ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementati
    Replacement::draft(0x1424ff410, "ui_1424ff410", fn_1424ff410 as *const ()), // ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementation::ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementati
    Replacement::draft(0x1424ff490, "ui_1424ff490", fn_1424ff490 as *const ()), // ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementation::ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementati
    Replacement::draft(0x1424ff530, "ui_1424ff530", fn_1424ff530 as *const ()), // ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementation::ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementati
    Replacement::draft(0x1424ff5c0, "ui_1424ff5c0", fn_1424ff5c0 as *const ()), // ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementation::ui::PageMetadata::ResourceGroup::ClassTypeInfoImplementati
    Replacement::draft(0x1424ffcf0, "ui_1424ffcf0", fn_1424ffcf0 as *const ()), // ui::PageMetadata::ClassTypeInfoImplementation::ui::PageMetadata::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x1424fffa0, "ui_1424fffa0", tpl_38887::<0x108, 0x20, 0x8, 0xc, 0x14020bcb0> as *const ()), // ui::PageResource::Header::ClassTypeInfoImplementation::ui::PageResource::Header::ClassTypeInfoImplementation::vfunc_21
    Replacement::draft(0x142500010, "ui_142500010", tpl_38887::<0x108, 0x20, 0x8, 0xc, 0x14020bcb0> as *const ()), // ui::PageResource::Header::ClassTypeInfoImplementation::ui::PageResource::Header::ClassTypeInfoImplementation::vfunc_38
    Replacement::draft(0x142500450, "ui_142500450", fn_142500450 as *const ()), // ui::PageResource::Header::ClassTypeInfoImplementation::ui::PageResource::Header::ClassTypeInfoImplementation::vfunc_14
    Replacement::draft(0x142500d40, "unknown_142500d40", fn_142500d40 as *const ()), // sub_142500d40
    Replacement::draft_throws(0x142500da0, "unknown_142500da0", fn_142500da0 as *const (), Throws::Caught), // sub_142500da0
    Replacement::draft(0x142500ff0, "unknown_142500ff0", tpl_361 as *const ()), // sub_142500ff0
    Replacement::draft(0x142501000, "unknown_142501000", fn_142501000 as *const ()), // sub_142501000
    Replacement::draft(0x142501090, "unknown_142501090", fn_142501090 as *const ()), // sub_142501090
    Replacement::draft(0x1425011a0, "unknown_1425011a0", tpl_34443 as *const ()), // sub_1425011a0
    Replacement::draft(0x1425011b0, "unknown_1425011b0", tpl_37236::<0x8, 0x3, 0x4, 0x1426d01e0> as *const ()), // sub_1425011b0
    Replacement::draft_throws(0x142501240, "unknown_142501240", fn_142501240 as *const (), Throws::Discard), // sub_142501240
    Replacement::draft_throws(0x142501260, "unknown_142501260", fn_142501260 as *const (), Throws::Caught), // sub_142501260
    Replacement::draft_throws(0x142501420, "unknown_142501420", fn_142501420 as *const (), Throws::Discard), // sub_142501420
    Replacement::draft_throws(0x142501470, "unknown_142501470", fn_142501470 as *const (), Throws::Caught), // sub_142501470
    Replacement::draft(0x1425015d0, "cohtml_1425015d0", tpl_34971::<0x143419ac0, 0x10, 0x143251120, 0x143419b40, 0x18, 0x142646d10> as *const ()), // sub_1425015d0
    Replacement::draft(0x142501640, "cohtml_142501640", fn_142501640 as *const ()), // sub_142501640
    Replacement::draft(0x142501d90, "ui_142501d90", tpl_35199::<0x143882290, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::Page::ClassTypeInfoImplementation::ui::Page::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x142501e80, "ui_142501e80", tpl_35199::<0x143882290, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::Page::ClassTypeInfoImplementation::ui::Page::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x142502060, "ui_142502060", tpl_35199::<0x143882290, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::Page::ClassTypeInfoImplementation::ui::Page::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x142502150, "ui_142502150", tpl_35199::<0x143882290, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::Page::ClassTypeInfoImplementation::ui::Page::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x1425021d0, "ui_1425021d0", tpl_35199::<0x143882290, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::Page::ClassTypeInfoImplementation::ui::Page::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft_throws(0x142502410, "ui_142502410", fn_142502410 as *const (), Throws::Discard), // sub_142502410
    Replacement::draft_throws(0x1425024d0, "ui_1425024d0", fn_1425024d0 as *const (), Throws::Caught), // sub_1425024d0
    Replacement::draft(0x142502a20, "ui_142502a20", fn_142502a20 as *const ()), // sub_142502a20
    Replacement::draft_throws(0x142502ae0, "ui_142502ae0", fn_142502ae0 as *const (), Throws::Caught), // sub_142502ae0
    Replacement::draft(0x142502ef0, "ui_142502ef0", fn_142502ef0 as *const ()), // sub_142502ef0
    Replacement::draft(0x142503190, "ui_142503190", tpl_37611::<0x118, 0x128> as *const ()), // sub_142503190
    Replacement::draft(0x1425031b0, "ui_1425031b0", fn_1425031b0 as *const ()), // sub_1425031b0
    Replacement::draft(0x142503200, "ui_142503200", fn_142503200 as *const ()), // sub_142503200
    Replacement::draft(0x1425032b0, "ui_1425032b0", fn_1425032b0 as *const ()), // sub_1425032b0
    Replacement::draft_throws(0x1425033b0, "ui_1425033b0", fn_1425033b0 as *const (), Throws::Discard), // sub_1425033b0
    Replacement::draft_throws(0x1425034e0, "ui_1425034e0", fn_1425034e0 as *const (), Throws::Discard), // sub_1425034e0
    Replacement::draft_throws(0x1425036b0, "ui_1425036b0", fn_1425036b0 as *const (), Throws::Discard), // sub_1425036b0
    Replacement::draft(0x142503810, "ui_142503810", fn_142503810 as *const ()), // sub_142503810
    Replacement::draft(0x142503870, "ui_142503870", fn_142503870 as *const ()), // sub_142503870
    Replacement::draft(0x1425038c0, "ui_1425038c0", fn_1425038c0 as *const ()), // sub_1425038c0
    Replacement::draft(0x142503980, "ui_142503980", tpl_22904::<0x110> as *const ()), // sub_142503980
    Replacement::draft_throws(0x1425039a0, "ui_1425039a0", fn_1425039a0 as *const (), Throws::Discard), // ui::Page::ui::Page::vfunc_4
    Replacement::draft_throws(0x1425039f0, "ui_1425039f0", fn_1425039f0 as *const (), Throws::Discard), // ui::Page::ui::Page::vfunc_15
    Replacement::draft_throws(0x142503af0, "ui_142503af0", fn_142503af0 as *const (), Throws::Discard), // ui::Page::ui::Page::vfunc_14
    Replacement::draft_throws(0x142503c10, "ui_142503c10", fn_142503c10 as *const (), Throws::Discard), // ui::Page::ui::Page::vfunc_13
    Replacement::draft_throws(0x142503c40, "ui_142503c40", fn_142503c40 as *const (), Throws::Discard), // ui::Page::ui::Page::vfunc_16
    Replacement::draft_throws(0x142503d70, "ui_142503d70", fn_142503d70 as *const (), Throws::Discard), // ui::Page::ui::Page::vfunc_7
    Replacement::draft_throws(0x142503dc0, "ui_142503dc0", fn_142503dc0 as *const (), Throws::Caught), // ui::Page::ui::Page::vfunc_23
    Replacement::draft(0x142503f90, "ui_142503f90", fn_142503f90 as *const ()), // ui::Page::ui::Page::vfunc_22
    Replacement::draft_throws(0x142504040, "unknown_142504040", fn_142504040 as *const (), Throws::Discard), // sub_142504040
    Replacement::draft_throws(0x1425041f0, "unknown_1425041f0", fn_1425041f0 as *const (), Throws::Discard), // sub_1425041f0
    Replacement::draft(0x142504250, "renoir_142504250", tpl_25139::<0x142646540> as *const ()), // renoir::StandardBackendAllocator::renoir::StandardBackendAllocator::vfunc_4
    Replacement::draft(0x142504260, "renoir_142504260", fn_142504260 as *const ()), // renoir::StandardBackendAllocator::renoir::StandardBackendAllocator::vfunc_3
    Replacement::draft(0x142504270, "renoir_142504270", tpl_25139::<0x142646558> as *const ()), // renoir::StandardBackendAllocator::renoir::StandardBackendAllocator::vfunc_2
    Replacement::draft(0x142504280, "renoir_142504280", tpl_25139::<0x142646538> as *const ()), // renoir::StandardBackendAllocator::renoir::StandardBackendAllocator::vfunc_1
    Replacement::draft(0x142504290, "renoir_142504290", tpl_1674::<0x143419fc8> as *const ()), // sub_142504290
    Replacement::draft(0x1425042b0, "renoir_1425042b0", fn_1425042b0 as *const ()), // sub_1425042b0
    Replacement::draft_throws(0x142504390, "ui_142504390", fn_142504390 as *const (), Throws::Caught), // ui::MemoryAllocator::ui::MemoryAllocator::vfunc_1
    Replacement::draft(0x142504580, "ui_142504580", fn_142504580 as *const ()), // sub_142504580
    Replacement::draft_throws(0x142504610, "ui_142504610", fn_142504610 as *const (), Throws::Discard), // ui::LocalizationManager::ui::LocalizationManager::vfunc_2
    Replacement::draft(0x142504740, "ui_142504740", fn_142504740 as *const ()), // sub_142504740
    Replacement::draft(0x142504770, "ui_142504770", fn_142504770 as *const ()), // sub_142504770
    Replacement::draft(0x1425048a0, "ui_1425048a0", fn_1425048a0 as *const ()), // ui::InternationalizationManager::ui::InternationalizationManager::vfunc_1
    Replacement::draft(0x142504900, "unknown_142504900", fn_142504900 as *const ()), // sub_142504900
    Replacement::draft(0x1425049a0, "unknown_1425049a0", fn_1425049a0 as *const ()), // sub_1425049a0
    Replacement::draft(0x1425049b0, "unknown_1425049b0", fn_1425049b0 as *const ()), // sub_1425049b0
    Replacement::draft(0x142504bd0, "unknown_142504bd0", fn_142504bd0 as *const ()), // sub_142504bd0
    Replacement::draft(0x142504c30, "std_142504c30", tpl_550::<0x14341a3c8, 0x8> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::
    Replacement::draft(0x142504c50, "std_142504c50", tpl_550::<0x14341a3c8, 0x8> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::
    Replacement::draft(0x142504c70, "std_142504c70", tpl_51995::<0x8, 0xb8, 0x1424968a0, 0x80> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::
    Replacement::draft(0x142504ca0, "std_142504ca0", tpl_1::<0x14389b830> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::
    Replacement::draft(0x142504cb0, "std_142504cb0", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::
    Replacement::draft(0x142504cc0, "std_142504cc0", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::blockingLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::
    Replacement::draft(0x142504ce0, "std_142504ce0", tpl_550::<0x14341a6d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::F
    Replacement::draft(0x142504d00, "std_142504d00", tpl_550::<0x14341a6d8, 0x8> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::F
    Replacement::draft(0x142504d20, "std_142504d20", tpl_51995::<0x8, 0xb8, 0x1424968a0, 0x80> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::F
    Replacement::draft(0x142504d50, "std_142504d50", tpl_1::<0x14389b7e0> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::F
    Replacement::draft(0x142504d60, "std_142504d60", tpl_548::<0x8> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::F
    Replacement::draft(0x142504d70, "std_142504d70", tpl_547::<0x141834d30> as *const ()), // std::_Func_impl_no_alloc<ui::FontResource::requestLoad::<lambda_1>,void,r::FileBuffer&&>::std::_Func_impl_no_alloc<ui::F
    Replacement::draft(0x142504d90, "unknown_142504d90", fn_142504d90 as *const ()), // sub_142504d90
    Replacement::draft(0x142504e00, "ui_142504e00", tpl_51997::<0xb8, 0x1424968a0, 0x80> as *const ()), // sub_142504e00
    Replacement::draft_throws(0x142504e30, "ui_142504e30", tpl_43242::<0x143c4ac24, 0x143a08320, 0x120, 0x14341a6d8, 0x142428500> as *const (), Throws::Discard), // ui::FontResource::ui::FontResource::vfunc_2
    Replacement::draft(0x142504eb0, "ui_142504eb0", tpl_51997::<0xb8, 0x1424968a0, 0x80> as *const ()), // sub_142504eb0
    Replacement::draft(0x1425054c0, "ui_1425054c0", fn_1425054c0 as *const ()), // ui::FontResource::ClassTypeInfoImplementation::ui::FontResource::ClassTypeInfoImplementation::vfunc_12
    Replacement::draft(0x142505b40, "ui_142505b40", tpl_35199::<0x143882298, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::FontMetadata::ClassTypeInfoImplementation::ui::FontMetadata::ClassTypeInfoImplementation::vfunc_24
    Replacement::draft(0x142505c30, "ui_142505c30", tpl_35199::<0x143882298, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::FontMetadata::ClassTypeInfoImplementation::ui::FontMetadata::ClassTypeInfoImplementation::vfunc_26
    Replacement::draft(0x142505e10, "ui_142505e10", tpl_35199::<0x143882298, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::FontMetadata::ClassTypeInfoImplementation::ui::FontMetadata::ClassTypeInfoImplementation::vfunc_32
    Replacement::draft(0x142505f00, "ui_142505f00", tpl_35199::<0x143882298, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::FontMetadata::ClassTypeInfoImplementation::ui::FontMetadata::ClassTypeInfoImplementation::vfunc_34
    Replacement::draft(0x142505f80, "ui_142505f80", tpl_35199::<0x143882298, 0x14243b0b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // ui::FontMetadata::ClassTypeInfoImplementation::ui::FontMetadata::ClassTypeInfoImplementation::vfunc_35
    Replacement::draft_throws(0x142506320, "ui_142506320", fn_142506320 as *const (), Throws::Caught), // sub_142506320
    Replacement::draft_throws(0x1425065f0, "ui_1425065f0", fn_1425065f0 as *const (), Throws::Caught), // ui::FileSystemReader::ui::FileSystemReader::vfunc_3
    Replacement::draft(0x142506700, "ui_142506700", fn_142506700 as *const ()), // ui::FileSystemReader::ui::FileSystemReader::vfunc_2
    Replacement::draft_throws(0x142506770, "ui_142506770", fn_142506770 as *const (), Throws::Discard), // ui::FileSystemReader::ui::FileSystemReader::vfunc_1
    Replacement::draft(0x1425067f0, "unknown_1425067f0", fn_1425067f0 as *const ()), // sub_1425067f0
    Replacement::draft(0x142506820, "unknown_142506820", tpl_28185::<0x14397d8ef> as *const ()), // sub_142506820
    Replacement::draft(0x142506830, "unknown_142506830", fn_142506830 as *const ()), // sub_142506830
    Replacement::draft(0x142507760, "unknown_142507760", fn_142507760 as *const ()), // sub_142507760
    Replacement::draft_throws(0x142507820, "unknown_142507820", fn_142507820 as *const (), Throws::Caught), // sub_142507820
    Replacement::draft_throws(0x14250f830, "std_14250f830", fn_14250f830 as *const (), Throws::Discard), // sub_14250f830
    Replacement::draft(0x14250fb50, "unknown_14250fb50", tpl_52004 as *const ()), // sub_14250fb50
    Replacement::draft(0x14250fb70, "unknown_14250fb70", tpl_52004 as *const ()), // sub_14250fb70
    Replacement::draft(0x14250fb80, "unknown_14250fb80", fn_14250fb80 as *const ()), // sub_14250fb80
    Replacement::draft(0x14250fcc0, "unknown_14250fcc0", fn_14250fcc0 as *const ()), // sub_14250fcc0
    Replacement::draft(0x14250fe60, "unknown_14250fe60", fn_14250fe60 as *const ()), // sub_14250fe60
    Replacement::draft(0x142510230, "unknown_142510230", fn_142510230 as *const ()), // sub_142510230
    Replacement::draft(0x142510490, "unknown_142510490", fn_142510490 as *const ()), // sub_142510490
    Replacement::draft(0x142510930, "unknown_142510930", fn_142510930 as *const ()), // sub_142510930
    Replacement::draft(0x142510fb0, "unknown_142510fb0", fn_142510fb0 as *const ()), // sub_142510fb0
    Replacement::draft(0x1425111e0, "unknown_1425111e0", fn_1425111e0 as *const ()), // sub_1425111e0
    Replacement::draft(0x142511610, "unknown_142511610", fn_142511610 as *const ()), // sub_142511610
    Replacement::draft(0x142511a20, "unknown_142511a20", fn_142511a20 as *const ()), // sub_142511a20
    Replacement::draft(0x142511d00, "unknown_142511d00", fn_142511d00 as *const ()), // sub_142511d00
    Replacement::draft(0x1425120f0, "unknown_1425120f0", fn_1425120f0 as *const ()), // sub_1425120f0
    Replacement::draft(0x1425122d0, "unknown_1425122d0", fn_1425122d0 as *const ()), // sub_1425122d0
    Replacement::draft(0x142512320, "unknown_142512320", fn_142512320 as *const ()), // sub_142512320
    Replacement::draft(0x142512360, "unknown_142512360", fn_142512360 as *const ()), // sub_142512360
    Replacement::draft(0x142512490, "unknown_142512490", fn_142512490 as *const ()), // sub_142512490
    Replacement::draft(0x142512530, "unknown_142512530", fn_142512530 as *const ()), // sub_142512530
    Replacement::draft(0x142512570, "unknown_142512570", fn_142512570 as *const ()), // sub_142512570
    Replacement::draft(0x1425125a0, "unknown_1425125a0", fn_1425125a0 as *const ()), // sub_1425125a0
    Replacement::draft(0x142512660, "unknown_142512660", fn_142512660 as *const ()), // sub_142512660
    Replacement::draft(0x1425126c0, "unknown_1425126c0", fn_1425126c0 as *const ()), // sub_1425126c0
    Replacement::draft(0x142512720, "unknown_142512720", fn_142512720 as *const ()), // sub_142512720
    Replacement::draft(0x142512750, "unknown_142512750", tpl_1738::<0x398> as *const ()), // sub_142512750
    Replacement::draft(0x142512760, "unknown_142512760", fn_142512760 as *const ()), // sub_142512760
    Replacement::draft(0x142512a80, "unknown_142512a80", fn_142512a80 as *const ()), // sub_142512a80
    Replacement::draft(0x142512af0, "unknown_142512af0", fn_142512af0 as *const ()), // sub_142512af0
    Replacement::draft(0x142512b10, "unknown_142512b10", fn_142512b10 as *const ()), // sub_142512b10
    Replacement::draft(0x1425131e0, "unknown_1425131e0", fn_1425131e0 as *const ()), // sub_1425131e0
    Replacement::draft(0x142513200, "unknown_142513200", fn_142513200 as *const ()), // sub_142513200
    Replacement::draft(0x142513240, "unknown_142513240", fn_142513240 as *const ()), // sub_142513240
    Replacement::draft(0x142513360, "unknown_142513360", fn_142513360 as *const ()), // sub_142513360
    Replacement::draft_throws(0x142513450, "unknown_142513450", fn_142513450 as *const (), Throws::Caught), // sub_142513450
    Replacement::draft(0x1425136a0, "unknown_1425136a0", fn_1425136a0 as *const ()), // sub_1425136a0
    Replacement::draft_throws(0x1425136e0, "fmt_1425136e0", fn_1425136e0 as *const (), Throws::Caught), // fmt::v9::format_facet<std::locale>::fmt::v9::format_facet<std::locale>::vfunc_3
    Replacement::draft_throws(0x142513780, "unknown_142513780", fn_142513780 as *const (), Throws::Caught), // sub_142513780
    Replacement::draft(0x142513a30, "unknown_142513a30", tpl_52033::<0x14310d6f4, 0x8> as *const ()), // sub_142513a30
    Replacement::draft(0x142513a50, "unknown_142513a50", tpl_360 as *const ()), // sub_142513a50
    Replacement::draft(0x142513a60, "unknown_142513a60", tpl_52033::<0x14310d6f4, 0x8> as *const ()), // sub_142513a60
    Replacement::draft(0x142513a80, "unknown_142513a80", tpl_360 as *const ()), // sub_142513a80
    Replacement::draft_throws(0x142513a90, "unknown_142513a90", fn_142513a90 as *const (), Throws::Discard), // sub_142513a90
    Replacement::draft(0x142513cb0, "unknown_142513cb0", fn_142513cb0 as *const ()), // sub_142513cb0
    Replacement::draft_throws(0x142513f10, "unknown_142513f10", fn_142513f10 as *const (), Throws::Caught), // sub_142513f10
    Replacement::draft(0x142513f80, "unknown_142513f80", fn_142513f80 as *const ()), // sub_142513f80
    Replacement::draft(0x1425142a0, "global_1425142a0", fn_1425142a0 as *const ()), // ffxFsr2ContextGenerateReactiveMask
    Replacement::draft(0x1425145c0, "global_1425145c0", fn_1425145c0 as *const ()), // ffxFsr2GetJitterOffset
    Replacement::draft(0x142514640, "global_142514640", fn_142514640 as *const ()), // ffxFsr2GetJitterPhaseCount
    Replacement::draft(0x1425146d0, "global_1425146d0", fn_1425146d0 as *const ()), // ffxFsr2ContextDispatch
    Replacement::draft(0x142514720, "global_142514720", fn_142514720 as *const ()), // ffxFsr2ContextDestroy
    Replacement::draft(0x142514850, "global_142514850", fn_142514850 as *const ()), // ffxFsr2ContextCreate
    Replacement::draft(0x1425148e0, "global_1425148e0", fn_1425148e0 as *const ()), // sub_1425148e0
    Replacement::draft(0x142515700, "global_142515700", fn_142515700 as *const ()), // sub_142515700
    Replacement::draft(0x142515990, "global_142515990", fn_142515990 as *const ()), // sub_142515990
    Replacement::draft(0x142515b20, "global_142515b20", fn_142515b20 as *const ()), // sub_142515b20
    Replacement::draft(0x142515c40, "global_142515c40", fn_142515c40 as *const ()), // sub_142515c40
    Replacement::draft(0x142515c70, "global_142515c70", tpl_52045::<0x50> as *const ()), // sub_142515c70
    Replacement::draft(0x142515c90, "global_142515c90", tpl_52045::<0x60> as *const ()), // sub_142515c90
    Replacement::draft(0x142515cb0, "global_142515cb0", fn_142515cb0 as *const ()), // sub_142515cb0
    Replacement::draft(0x142516990, "global_142516990", fn_142516990 as *const ()), // sub_142516990
    Replacement::draft(0x142516c00, "global_142516c00", fn_142516c00 as *const ()), // sub_142516c00
    Replacement::draft(0x142516e10, "global_142516e10", fn_142516e10 as *const ()), // sub_142516e10
    Replacement::draft(0x142517200, "global_142517200", fn_142517200 as *const ()), // sub_142517200
    Replacement::draft(0x142517260, "global_142517260", fn_142517260 as *const ()), // sub_142517260
    Replacement::draft(0x1425172e0, "global_1425172e0", fn_1425172e0 as *const ()), // sub_1425172e0
    Replacement::draft(0x142517390, "global_142517390", fn_142517390 as *const ()), // sub_142517390
    Replacement::draft(0x142517440, "global_142517440", fn_142517440 as *const ()), // sub_142517440
    Replacement::draft(0x1425174e0, "global_1425174e0", fn_1425174e0 as *const ()), // sub_1425174e0
    Replacement::draft(0x142517550, "global_142517550", fn_142517550 as *const ()), // sub_142517550
    Replacement::draft(0x142517590, "global_142517590", fn_142517590 as *const ()), // sub_142517590
    Replacement::draft(0x1425175a0, "global_1425175a0", fn_1425175a0 as *const ()), // sub_1425175a0
    Replacement::draft(0x1425175b0, "global_1425175b0", fn_1425175b0 as *const ()), // sub_1425175b0
    Replacement::draft(0x1425175d0, "global_1425175d0", fn_1425175d0 as *const ()), // ffxFsr2GetRenderResolutionFromQualityMode
    Replacement::draft_throws(0x142517960, "luau_142517960", fn_142517960 as *const (), Throws::Discard), // sub_142517960
    Replacement::draft_throws(0x142517a60, "luau_142517a60", fn_142517a60 as *const (), Throws::Discard), // sub_142517a60
    Replacement::draft_throws(0x142517b70, "luau_142517b70", fn_142517b70 as *const (), Throws::Discard), // sub_142517b70
    Replacement::draft_throws(0x142517c70, "luau_142517c70", fn_142517c70 as *const (), Throws::Discard), // sub_142517c70
    Replacement::draft_throws(0x142517d80, "luau_142517d80", fn_142517d80 as *const (), Throws::Caught), // sub_142517d80
    Replacement::draft_throws(0x142517e90, "luau_142517e90", fn_142517e90 as *const (), Throws::Discard), // sub_142517e90
    Replacement::draft_throws(0x142517ec0, "luau_142517ec0", fn_142517ec0 as *const (), Throws::Caught), // sub_142517ec0
    Replacement::draft_throws(0x142518140, "luau_142518140", fn_142518140 as *const (), Throws::Discard), // sub_142518140
    Replacement::draft_throws(0x142518180, "luau_142518180", fn_142518180 as *const (), Throws::Discard), // sub_142518180
    Replacement::draft_throws(0x142518220, "luau_142518220", fn_142518220 as *const (), Throws::Caught), // sub_142518220
    Replacement::draft_throws(0x142518390, "luau_142518390", fn_142518390 as *const (), Throws::Discard), // sub_142518390
    Replacement::draft_throws(0x1425184b0, "luau_1425184b0", fn_1425184b0 as *const (), Throws::Caught), // sub_1425184b0
    Replacement::draft_throws(0x142518750, "luau_142518750", fn_142518750 as *const (), Throws::Discard), // sub_142518750
    Replacement::draft_throws(0x142518870, "luau_142518870", fn_142518870 as *const (), Throws::Discard), // sub_142518870
    Replacement::draft_throws(0x1425188e0, "luau_1425188e0", fn_1425188e0 as *const (), Throws::Caught), // sub_1425188e0
    Replacement::draft_throws(0x142518a50, "luau_142518a50", fn_142518a50 as *const (), Throws::Discard), // sub_142518a50
    Replacement::draft_throws(0x142518b40, "luau_142518b40", fn_142518b40 as *const (), Throws::Discard), // sub_142518b40
    Replacement::draft_throws(0x142518b50, "luau_142518b50", fn_142518b50 as *const (), Throws::Discard), // sub_142518b50
    Replacement::draft(0x142518c10, "luau_142518c10", fn_142518c10 as *const ()), // sub_142518c10
    Replacement::draft_throws(0x142518c30, "luau_142518c30", fn_142518c30 as *const (), Throws::Discard), // sub_142518c30
    Replacement::draft_throws(0x142518da0, "luau_142518da0", fn_142518da0 as *const (), Throws::Discard), // sub_142518da0
    Replacement::draft_throws(0x142519100, "luau_142519100", fn_142519100 as *const (), Throws::Caught), // sub_142519100
    Replacement::draft_throws(0x142519790, "luau_142519790", fn_142519790 as *const (), Throws::Discard), // sub_142519790
    Replacement::draft_throws(0x142519800, "luau_142519800", fn_142519800 as *const (), Throws::Discard), // sub_142519800
    Replacement::draft_throws(0x142519970, "luau_142519970", fn_142519970 as *const (), Throws::Caught), // sub_142519970
    Replacement::draft_throws(0x142519bf0, "luau_142519bf0", fn_142519bf0 as *const (), Throws::Caught), // sub_142519bf0
    Replacement::draft_throws(0x14251a0c0, "luau_14251a0c0", fn_14251a0c0 as *const (), Throws::Discard), // sub_14251a0c0
    Replacement::draft_throws(0x14251a1a0, "luau_14251a1a0", fn_14251a1a0 as *const (), Throws::Discard), // sub_14251a1a0
    Replacement::draft_throws(0x14251a250, "luau_14251a250", fn_14251a250 as *const (), Throws::Discard), // sub_14251a250
    Replacement::draft_throws(0x14251a2c0, "luau_14251a2c0", fn_14251a2c0 as *const (), Throws::Discard), // sub_14251a2c0
    Replacement::draft_throws(0x14251a480, "luau_14251a480", fn_14251a480 as *const (), Throws::Caught), // sub_14251a480
    Replacement::draft_throws(0x14251a9d0, "luau_14251a9d0", fn_14251a9d0 as *const (), Throws::Discard), // sub_14251a9d0
    Replacement::draft_throws(0x14251a9f0, "luau_14251a9f0", fn_14251a9f0 as *const (), Throws::Caught), // sub_14251a9f0
    Replacement::draft_throws(0x14251ab30, "luau_14251ab30", fn_14251ab30 as *const (), Throws::Caught), // sub_14251ab30
    Replacement::draft_throws(0x14251afb0, "luau_14251afb0", fn_14251afb0 as *const (), Throws::Discard), // sub_14251afb0
    Replacement::draft(0x14251b360, "luau_14251b360", fn_14251b360 as *const ()), // sub_14251b360
    Replacement::draft(0x14251b400, "luau_14251b400", fn_14251b400 as *const ()), // sub_14251b400
    Replacement::draft_throws(0x14251b4d0, "luau_14251b4d0", fn_14251b4d0 as *const (), Throws::Discard), // sub_14251b4d0
    Replacement::draft_throws(0x14251b5e0, "luau_14251b5e0", fn_14251b5e0 as *const (), Throws::Discard), // sub_14251b5e0
    Replacement::draft_throws(0x14251bb80, "luau_14251bb80", fn_14251bb80 as *const (), Throws::Discard), // sub_14251bb80
    Replacement::draft_throws(0x14251bcc0, "luau_14251bcc0", fn_14251bcc0 as *const (), Throws::Discard), // sub_14251bcc0
    Replacement::draft_throws(0x14251bf60, "luau_14251bf60", fn_14251bf60 as *const (), Throws::Discard), // sub_14251bf60
    Replacement::draft_throws(0x14251c440, "luau_14251c440", fn_14251c440 as *const (), Throws::Discard), // sub_14251c440
    Replacement::draft_throws(0x14251c6e0, "luau_14251c6e0", fn_14251c6e0 as *const (), Throws::Discard), // sub_14251c6e0
    Replacement::draft(0x14251c740, "luau_14251c740", fn_14251c740 as *const ()), // sub_14251c740
    Replacement::draft(0x14251c7f0, "luau_14251c7f0", fn_14251c7f0 as *const ()), // sub_14251c7f0
    Replacement::draft(0x14251c930, "luau_14251c930", fn_14251c930 as *const ()), // sub_14251c930
    Replacement::draft_throws(0x14251c980, "luau_14251c980", fn_14251c980 as *const (), Throws::Discard), // sub_14251c980
    Replacement::draft_throws(0x14251cb70, "luau_14251cb70", fn_14251cb70 as *const (), Throws::Discard), // sub_14251cb70
    Replacement::draft_throws(0x14251cd90, "luau_14251cd90", fn_14251cd90 as *const (), Throws::Caught), // sub_14251cd90
    Replacement::draft_throws(0x14251d490, "luau_14251d490", fn_14251d490 as *const (), Throws::Caught), // sub_14251d490
    Replacement::draft_throws(0x14251d590, "luau_14251d590", fn_14251d590 as *const (), Throws::Caught), // sub_14251d590
    Replacement::draft_throws(0x14251d6e0, "luau_14251d6e0", fn_14251d6e0 as *const (), Throws::Discard), // sub_14251d6e0
    Replacement::draft_throws(0x14251dbe0, "luau_14251dbe0", fn_14251dbe0 as *const (), Throws::Discard), // sub_14251dbe0
    Replacement::draft_throws(0x14251dec0, "luau_14251dec0", fn_14251dec0 as *const (), Throws::Caught), // sub_14251dec0
    Replacement::draft_throws(0x14251e2a0, "luau_14251e2a0", fn_14251e2a0 as *const (), Throws::Caught), // sub_14251e2a0
    Replacement::draft_throws(0x14251e8e0, "luau_14251e8e0", fn_14251e8e0 as *const (), Throws::Discard), // sub_14251e8e0
    Replacement::draft_throws(0x14251ea20, "luau_14251ea20", fn_14251ea20 as *const (), Throws::Caught), // sub_14251ea20
    Replacement::draft_throws(0x14251f150, "luau_14251f150", fn_14251f150 as *const (), Throws::Discard), // sub_14251f150
    Replacement::draft_throws(0x14251f230, "luau_14251f230", fn_14251f230 as *const (), Throws::Discard), // sub_14251f230
    Replacement::draft_throws(0x14251f4a0, "luau_14251f4a0", fn_14251f4a0 as *const (), Throws::Discard), // sub_14251f4a0
    Replacement::draft_throws(0x14251f4e0, "luau_14251f4e0", fn_14251f4e0 as *const (), Throws::Discard), // sub_14251f4e0
    Replacement::draft_throws(0x14251f5e0, "luau_14251f5e0", fn_14251f5e0 as *const (), Throws::Discard), // sub_14251f5e0
    Replacement::draft_throws(0x14251f6c0, "luau_14251f6c0", fn_14251f6c0 as *const (), Throws::Discard), // sub_14251f6c0
    Replacement::draft_throws(0x14251f780, "luau_14251f780", fn_14251f780 as *const (), Throws::Caught), // sub_14251f780
    Replacement::draft_throws(0x14251fda0, "luau_14251fda0", fn_14251fda0 as *const (), Throws::Caught), // sub_14251fda0
    Replacement::draft_throws(0x14251ff50, "luau_14251ff50", fn_14251ff50 as *const (), Throws::Discard), // sub_14251ff50
    Replacement::draft_throws(0x1425200a0, "luau_1425200a0", fn_1425200a0 as *const (), Throws::Caught), // sub_1425200a0
    Replacement::draft(0x142520430, "luau_142520430", fn_142520430 as *const ()), // sub_142520430
    Replacement::draft_throws(0x142520460, "luau_142520460", fn_142520460 as *const (), Throws::Caught), // sub_142520460
    Replacement::draft_throws(0x1425210d0, "luau_1425210d0", fn_1425210d0 as *const (), Throws::Caught), // sub_1425210d0
    Replacement::draft_throws(0x1425218a0, "luau_1425218a0", fn_1425218a0 as *const (), Throws::Discard), // sub_1425218a0
    Replacement::draft_throws(0x142521a60, "luau_142521a60", fn_142521a60 as *const (), Throws::Caught), // sub_142521a60
    Replacement::draft_throws(0x142521ce0, "luau_142521ce0", fn_142521ce0 as *const (), Throws::Caught), // sub_142521ce0
    Replacement::draft_throws(0x142522270, "luau_142522270", fn_142522270 as *const (), Throws::Discard), // sub_142522270
    Replacement::draft_throws(0x1425223f0, "luau_1425223f0", fn_1425223f0 as *const (), Throws::Discard), // sub_1425223f0
    Replacement::draft_throws(0x142522760, "luau_142522760", fn_142522760 as *const (), Throws::Caught), // sub_142522760
    Replacement::draft_throws(0x142522e70, "luau_142522e70", fn_142522e70 as *const (), Throws::Discard), // sub_142522e70
    Replacement::draft_throws(0x142522f20, "luau_142522f20", fn_142522f20 as *const (), Throws::Discard), // sub_142522f20
    Replacement::draft_throws(0x142523070, "luau_142523070", fn_142523070 as *const (), Throws::Discard), // sub_142523070
    Replacement::draft_throws(0x142523100, "luau_142523100", fn_142523100 as *const (), Throws::Discard), // sub_142523100
    Replacement::draft_throws(0x142523280, "luau_142523280", fn_142523280 as *const (), Throws::Discard), // sub_142523280
    Replacement::draft_throws(0x142523420, "luau_142523420", fn_142523420 as *const (), Throws::Discard), // sub_142523420
    Replacement::draft_throws(0x1425236d0, "luau_1425236d0", fn_1425236d0 as *const (), Throws::Discard), // sub_1425236d0
    Replacement::draft_throws(0x142524190, "luau_142524190", fn_142524190 as *const (), Throws::Caught), // sub_142524190
    Replacement::draft(0x1425243f0, "luau_1425243f0", fn_1425243f0 as *const ()), // sub_1425243f0
    Replacement::draft_throws(0x142524420, "luau_142524420", fn_142524420 as *const (), Throws::Discard), // sub_142524420
    Replacement::draft_throws(0x142524470, "luau_142524470", fn_142524470 as *const (), Throws::Discard), // sub_142524470
    Replacement::draft(0x1425244e0, "luau_1425244e0", fn_1425244e0 as *const ()), // sub_1425244e0
    Replacement::draft_throws(0x142524510, "luau_142524510", fn_142524510 as *const (), Throws::Caught), // sub_142524510
    Replacement::draft_throws(0x142524b00, "luau_142524b00", fn_142524b00 as *const (), Throws::Caught), // sub_142524b00
    Replacement::draft_throws(0x142525040, "luau_142525040", fn_142525040 as *const (), Throws::Discard), // sub_142525040
    Replacement::draft(0x142525090, "luau_142525090", tpl_52134::<0x48, 0x30> as *const ()), // sub_142525090
    Replacement::draft_throws(0x1425250b0, "luau_1425250b0", fn_1425250b0 as *const (), Throws::Caught), // sub_1425250b0
    Replacement::draft(0x142525110, "luau_142525110", tpl_52134::<0x40, 0x28> as *const ()), // Luau::ParseError::Luau::ParseError::vfunc_1
    Replacement::draft_throws(0x142525130, "luau_142525130", fn_142525130 as *const (), Throws::Caught), // sub_142525130
    Replacement::draft(0x142525180, "luau_142525180", tpl_548::<0x18> as *const ()), // sub_142525180
    Replacement::draft(0x142525190, "luau_142525190", tpl_52137::<0x8> as *const ()), // Luau::AstTypePackGeneric::Luau::AstTypePackGeneric::vfunc_0
    Replacement::draft(0x1425251b0, "luau_1425251b0", tpl_52138::<0x10, 0x20> as *const ()), // Luau::AstTypePackVariadic::Luau::AstTypePackVariadic::vfunc_0
    Replacement::draft(0x1425251f0, "luau_1425251f0", fn_1425251f0 as *const ()), // Luau::AstTypePackExplicit::Luau::AstTypePackExplicit::vfunc_0
    Replacement::draft(0x142525270, "luau_142525270", fn_142525270 as *const ()), // Luau::AstTypeError::Luau::AstTypeError::vfunc_0
    Replacement::draft(0x1425252e0, "luau_1425252e0", tpl_52137::<0x30> as *const ()), // Luau::AstTypeSingletonString::Luau::AstTypeSingletonString::vfunc_0
    Replacement::draft(0x142525300, "luau_142525300", tpl_52137::<0x38> as *const ()), // Luau::AstTypeSingletonBool::Luau::AstTypeSingletonBool::vfunc_0
    Replacement::draft(0x142525320, "luau_142525320", tpl_110454::<0x40, 0x28, 0x20> as *const ()), // Luau::AstTypeIntersection::Luau::AstTypeIntersection::vfunc_0
    Replacement::draft(0x142525390, "luau_142525390", tpl_110454::<0x48, 0x28, 0x20> as *const ()), // Luau::AstTypeUnion::Luau::AstTypeUnion::vfunc_0
    Replacement::draft(0x142525400, "luau_142525400", tpl_52138::<0x50, 0x20> as *const ()), // Luau::AstTypeTypeof::Luau::AstTypeTypeof::vfunc_0
    Replacement::draft(0x142525440, "luau_142525440", tpl_52139::<0x58, 0x40, 0x142526360, 0x68> as *const ()), // Luau::AstTypeFunction::Luau::AstTypeFunction::vfunc_0
    Replacement::draft(0x142525490, "luau_142525490", fn_142525490 as *const ()), // Luau::AstTypeTable::Luau::AstTypeTable::vfunc_0
    Replacement::draft(0x142525530, "luau_142525530", fn_142525530 as *const ()), // Luau::AstTypeReference::Luau::AstTypeReference::vfunc_0
    Replacement::draft(0x1425255b0, "luau_1425255b0", tpl_110457::<0x78> as *const ()), // Luau::AstStatError::Luau::AstStatError::vfunc_0
    Replacement::draft(0x142525650, "luau_142525650", fn_142525650 as *const ()), // Luau::AstStatDeclareClass::Luau::AstStatDeclareClass::vfunc_0
    Replacement::draft(0x1425256c0, "luau_1425256c0", tpl_52139::<0x90, 0x50, 0x142526360, 0x78> as *const ()), // Luau::AstStatDeclareFunction::Luau::AstStatDeclareFunction::vfunc_0
    Replacement::draft(0x142525710, "luau_142525710", tpl_52138::<0x88, 0x30> as *const ()), // Luau::AstStatDeclareGlobal::Luau::AstStatDeclareGlobal::vfunc_0
    Replacement::draft(0x142525750, "luau_142525750", fn_142525750 as *const ()), // Luau::AstStatTypeAlias::Luau::AstStatTypeAlias::vfunc_0
    Replacement::draft(0x142525810, "luau_142525810", tpl_52138::<0xa0, 0x30> as *const ()), // Luau::AstStatLocalFunction::Luau::AstStatLocalFunction::vfunc_0
    Replacement::draft(0x142525850, "luau_142525850", tpl_52140::<0xa8, 0x28, 0x30> as *const ()), // Luau::AstStatFunction::Luau::AstStatFunction::vfunc_0
    Replacement::draft(0x1425258a0, "luau_1425258a0", tpl_52140::<0xb0, 0x30, 0x38> as *const ()), // Luau::AstStatCompoundAssign::Luau::AstStatCompoundAssign::vfunc_0
    Replacement::draft(0x1425258f0, "luau_1425258f0", tpl_110457::<0xb8> as *const ()), // Luau::AstStatAssign::Luau::AstStatAssign::vfunc_0
    Replacement::draft(0x142525990, "luau_142525990", fn_142525990 as *const ()), // Luau::AstStatForIn::Luau::AstStatForIn::vfunc_0
    Replacement::draft(0x142525a40, "luau_142525a40", fn_142525a40 as *const ()), // Luau::AstStatFor::Luau::AstStatFor::vfunc_0
    Replacement::draft(0x142525ac0, "luau_142525ac0", fn_142525ac0 as *const ()), // Luau::AstStatLocal::Luau::AstStatLocal::vfunc_0
    Replacement::draft(0x142525b60, "luau_142525b60", tpl_52138::<0xd8, 0x28> as *const ()), // Luau::AstStatExpr::Luau::AstStatExpr::vfunc_0
    Replacement::draft(0x142525ba0, "luau_142525ba0", tpl_110454::<0xe0, 0x30, 0x28> as *const ()), // Luau::AstStatReturn::Luau::AstStatReturn::vfunc_0
    Replacement::draft(0x142525c10, "luau_142525c10", tpl_52137::<0xe8> as *const ()), // Luau::AstStatContinue::Luau::AstStatContinue::vfunc_0
    Replacement::draft(0x142525c30, "luau_142525c30", tpl_52137::<0xf0> as *const ()), // Luau::AstStatBreak::Luau::AstStatBreak::vfunc_0
    Replacement::draft(0x142525c50, "luau_142525c50", tpl_52140::<0xf8, 0x30, 0x28> as *const ()), // Luau::AstStatRepeat::Luau::AstStatRepeat::vfunc_0
    Replacement::draft(0x142525ca0, "luau_142525ca0", tpl_52140::<0x100, 0x28, 0x30> as *const ()), // Luau::AstStatWhile::Luau::AstStatWhile::vfunc_0
    Replacement::draft(0x142525cf0, "luau_142525cf0", fn_142525cf0 as *const ()), // Luau::AstStatIf::Luau::AstStatIf::vfunc_0
    Replacement::draft(0x142525d50, "luau_142525d50", tpl_110454::<0x110, 0x30, 0x28> as *const ()), // Luau::AstStatBlock::Luau::AstStatBlock::vfunc_0
    Replacement::draft(0x142525dc0, "luau_142525dc0", tpl_110454::<0x120, 0x28, 0x20> as *const ()), // Luau::AstExprError::Luau::AstExprError::vfunc_0
    Replacement::draft(0x142525e30, "luau_142525e30", tpl_110454::<0x128, 0x38, 0x30> as *const ()), // Luau::AstExprInterpString::Luau::AstExprInterpString::vfunc_0
    Replacement::draft(0x142525ea0, "luau_142525ea0", fn_142525ea0 as *const ()), // Luau::AstExprIfElse::Luau::AstExprIfElse::vfunc_0
    Replacement::draft(0x142525f00, "luau_142525f00", tpl_52140::<0x138, 0x20, 0x28> as *const ()), // Luau::AstExprTypeAssertion::Luau::AstExprTypeAssertion::vfunc_0
    Replacement::draft(0x142525f50, "luau_142525f50", tpl_52140::<0x140, 0x28, 0x30> as *const ()), // Luau::AstExprBinary::Luau::AstExprBinary::vfunc_0
    Replacement::draft(0x142525fa0, "luau_142525fa0", tpl_52138::<0x148, 0x28> as *const ()), // Luau::AstExprUnary::Luau::AstExprUnary::vfunc_0
    Replacement::draft(0x142525fe0, "luau_142525fe0", fn_142525fe0 as *const ()), // Luau::AstExprTable::Luau::AstExprTable::vfunc_0
    Replacement::draft(0x142526060, "luau_142526060", fn_142526060 as *const ()), // Luau::AstExprFunction::Luau::AstExprFunction::vfunc_0
    Replacement::draft(0x142526140, "luau_142526140", tpl_52140::<0x160, 0x20, 0x28> as *const ()), // Luau::AstExprIndexExpr::Luau::AstExprIndexExpr::vfunc_0
    Replacement::draft(0x142526190, "luau_142526190", tpl_52138::<0x168, 0x20> as *const ()), // Luau::AstExprIndexName::Luau::AstExprIndexName::vfunc_0
    Replacement::draft(0x1425261d0, "luau_1425261d0", fn_1425261d0 as *const ()), // Luau::AstExprCall::Luau::AstExprCall::vfunc_0
    Replacement::draft(0x142526240, "luau_142526240", tpl_52137::<0x178> as *const ()), // Luau::AstExprVarargs::Luau::AstExprVarargs::vfunc_0
    Replacement::draft(0x142526260, "luau_142526260", tpl_52137::<0x180> as *const ()), // Luau::AstExprGlobal::Luau::AstExprGlobal::vfunc_0
    Replacement::draft(0x142526280, "luau_142526280", tpl_52137::<0x188> as *const ()), // Luau::AstExprLocal::Luau::AstExprLocal::vfunc_0
    Replacement::draft(0x1425262a0, "luau_1425262a0", tpl_52137::<0x190> as *const ()), // Luau::AstExprConstantString::Luau::AstExprConstantString::vfunc_0
    Replacement::draft(0x1425262c0, "luau_1425262c0", tpl_52137::<0x198> as *const ()), // Luau::AstExprConstantNumber::Luau::AstExprConstantNumber::vfunc_0
    Replacement::draft(0x1425262e0, "luau_1425262e0", tpl_52137::<0x1a0> as *const ()), // Luau::AstExprConstantBool::Luau::AstExprConstantBool::vfunc_0
    Replacement::draft(0x142526300, "luau_142526300", tpl_52137::<0x1a8> as *const ()), // Luau::AstExprConstantNil::Luau::AstExprConstantNil::vfunc_0
    Replacement::draft(0x142526320, "luau_142526320", tpl_52138::<0x1b0, 0x20> as *const ()), // Luau::AstExprGroup::Luau::AstExprGroup::vfunc_0
    Replacement::draft(0x142526360, "luau_142526360", fn_142526360 as *const ()), // sub_142526360
    Replacement::draft(0x1425263e0, "luau_1425263e0", tpl_52145::<0x143d01584, 0x143422e38, 0x8, 0x143422780, 0xc, 0x20, 0x28, 0x30> as *const ()), // sub_1425263e0
    Replacement::draft(0x142526420, "luau_142526420", tpl_52146::<0x143d015a4, 0x143422e38, 0x8, 0x143422988, 0xc> as *const ()), // sub_142526420
    Replacement::draft(0x142526450, "luau_142526450", fn_142526450 as *const ()), // sub_142526450
    Replacement::draft(0x142526490, "luau_142526490", fn_142526490 as *const ()), // sub_142526490
    Replacement::draft(0x142526530, "luau_142526530", fn_142526530 as *const ()), // sub_142526530
    Replacement::draft(0x142526620, "luau_142526620", tpl_52150::<0x143d015bc, 0x143422e38, 0x8, 0x1434227d0, 0xc, 0x20, 0x28> as *const ()), // sub_142526620
    Replacement::draft(0x142526660, "luau_142526660", fn_142526660 as *const ()), // sub_142526660
    Replacement::draft(0x1425266d0, "luau_1425266d0", tpl_52152::<0x143d01604, 0x143422e38, 0x8, 0x143422938, 0xc, 0x20> as *const ()), // sub_1425266d0
    Replacement::draft(0x142526710, "luau_142526710", fn_142526710 as *const ()), // sub_142526710
    Replacement::draft(0x142526750, "luau_142526750", tpl_52154::<0x143d01730, 0x143422e38, 0x8, 0x143422870, 0xc, 0x20> as *const ()), // sub_142526750
    Replacement::draft(0x142526790, "luau_142526790", fn_142526790 as *const ()), // sub_142526790
    Replacement::draft(0x1425267f0, "luau_1425267f0", tpl_52152::<0x143d01614, 0x143422e38, 0x8, 0x143422d20, 0xc, 0x20> as *const ()), // sub_1425267f0
    Replacement::draft(0x142526830, "luau_142526830", fn_142526830 as *const ()), // sub_142526830
    Replacement::draft(0x142526890, "luau_142526890", tpl_52154::<0x143d015d0, 0x143422e38, 0x8, 0x143422d48, 0xc, 0x20> as *const ()), // sub_142526890
    Replacement::draft(0x1425268d0, "luau_1425268d0", fn_1425268d0 as *const ()), // sub_1425268d0
    Replacement::draft(0x142526950, "luau_142526950", tpl_52158::<0x143d015b4, 0x143422e38, 0x8, 0x143422a28, 0xc, 0x20> as *const ()), // sub_142526950
    Replacement::draft(0x142526990, "luau_142526990", fn_142526990 as *const ()), // sub_142526990
    Replacement::draft(0x1425269d0, "luau_1425269d0", fn_1425269d0 as *const ()), // sub_1425269d0
    Replacement::draft(0x142526a20, "luau_142526a20", fn_142526a20 as *const ()), // sub_142526a20
    Replacement::draft(0x142526a80, "luau_142526a80", fn_142526a80 as *const ()), // sub_142526a80
    Replacement::draft(0x142526ac0, "luau_142526ac0", tpl_52163::<0x143d0157c, 0x143422e38, 0x8, 0x143422a50, 0xc, 0x20> as *const ()), // sub_142526ac0
    Replacement::draft(0x142526b00, "luau_142526b00", fn_142526b00 as *const ()), // sub_142526b00
    Replacement::draft(0x142526b60, "luau_142526b60", tpl_52145::<0x143d015b0, 0x143422e38, 0x8, 0x143422898, 0xc, 0x20, 0x28, 0x30> as *const ()), // sub_142526b60
    Replacement::draft(0x142526ba0, "luau_142526ba0", tpl_52163::<0x143d01570, 0x143422e38, 0x8, 0x143422b90, 0xc, 0x20> as *const ()), // sub_142526ba0
    Replacement::draft(0x142526be0, "luau_142526be0", tpl_52165::<0x143d01748, 0x8, 0x143422c30, 0xc, 0x20> as *const ()), // sub_142526be0
    Replacement::draft(0x142526c10, "luau_142526c10", fn_142526c10 as *const ()), // sub_142526c10
    Replacement::draft(0x142526c70, "luau_142526c70", tpl_52152::<0x143d01738, 0x143422e38, 0x8, 0x143422758, 0xc, 0x20> as *const ()), // sub_142526c70
    Replacement::draft(0x142526cb0, "luau_142526cb0", fn_142526cb0 as *const ()), // sub_142526cb0
    Replacement::draft(0x142526d00, "luau_142526d00", fn_142526d00 as *const ()), // sub_142526d00
    Replacement::draft(0x142526d50, "luau_142526d50", fn_142526d50 as *const ()), // sub_142526d50
    Replacement::draft(0x142526dd0, "luau_142526dd0", fn_142526dd0 as *const ()), // sub_142526dd0
    Replacement::draft(0x142526e10, "luau_142526e10", fn_142526e10 as *const ()), // sub_142526e10
    Replacement::draft(0x142526ea0, "luau_142526ea0", fn_142526ea0 as *const ()), // sub_142526ea0
    Replacement::draft(0x142526ef0, "luau_142526ef0", fn_142526ef0 as *const ()), // sub_142526ef0
    Replacement::draft(0x142526f40, "luau_142526f40", tpl_52163::<0x143d015ac, 0x143422e38, 0x8, 0x143422cf8, 0xc, 0x20> as *const ()), // sub_142526f40
    Replacement::draft(0x142526f80, "luau_142526f80", tpl_52174::<0x143d01608, 0x143422e38, 0x8, 0x143422a78, 0xc, 0x20, 0x28> as *const ()), // sub_142526f80
    Replacement::draft(0x142526fc0, "luau_142526fc0", tpl_52150::<0x143d015e4, 0x143422e38, 0x8, 0x143422cd0, 0xc, 0x20, 0x28> as *const ()), // sub_142526fc0
    Replacement::draft(0x142527000, "luau_142527000", tpl_52146::<0x143d01568, 0x143422e38, 0x8, 0x143422af0, 0xc> as *const ()), // sub_142527000
    Replacement::draft(0x142527030, "luau_142527030", fn_142527030 as *const ()), // sub_142527030
    Replacement::draft(0x142527080, "luau_142527080", fn_142527080 as *const ()), // sub_142527080
    Replacement::draft(0x1425270c0, "luau_1425270c0", fn_1425270c0 as *const ()), // sub_1425270c0
    Replacement::draft(0x142527100, "luau_142527100", fn_142527100 as *const ()), // sub_142527100
    Replacement::draft(0x142527180, "luau_142527180", tpl_52163::<0x143d01728, 0x143422e38, 0x8, 0x143422b40, 0xc, 0x20> as *const ()), // sub_142527180
    Replacement::draft(0x1425271c0, "luau_1425271c0", fn_1425271c0 as *const ()), // sub_1425271c0
    Replacement::draft(0x142527210, "luau_142527210", fn_142527210 as *const ()), // sub_142527210
    Replacement::draft(0x142527270, "luau_142527270", tpl_52165::<0x143d01588, 0x8, 0x143422ac8, 0xc, 0x20> as *const ()), // sub_142527270
    Replacement::draft(0x1425272a0, "luau_1425272a0", tpl_52174::<0x143d015d8, 0x143422e38, 0x8, 0x143422de8, 0xc, 0x20, 0x28> as *const ()), // sub_1425272a0
    Replacement::draft(0x1425272e0, "luau_1425272e0", fn_1425272e0 as *const ()), // sub_1425272e0
    Replacement::draft(0x142527320, "luau_142527320", fn_142527320 as *const ()), // sub_142527320
    Replacement::draft(0x142527370, "luau_142527370", fn_142527370 as *const ()), // sub_142527370
    Replacement::draft(0x142527400, "luau_142527400", tpl_52158::<0x143d01574, 0x143422e38, 0x8, 0x143422848, 0xc, 0x20> as *const ()), // sub_142527400
    Replacement::draft(0x142527440, "luau_142527440", fn_142527440 as *const ()), // sub_142527440
    Replacement::draft(0x142527490, "luau_142527490", fn_142527490 as *const ()), // sub_142527490
    Replacement::draft_throws(0x1425274d0, "luau_1425274d0", tpl_45421::<0x142527500> as *const (), Throws::Discard), // sub_1425274d0
    Replacement::draft_throws(0x142527500, "luau_142527500", fn_142527500 as *const (), Throws::Discard), // sub_142527500
    Replacement::draft_throws(0x1425275d0, "luau_1425275d0", fn_1425275d0 as *const (), Throws::Caught), // sub_1425275d0
    Replacement::draft_throws(0x142527620, "luau_142527620", fn_142527620 as *const (), Throws::Caught), // sub_142527620
    Replacement::draft(0x142527680, "luau_142527680", fn_142527680 as *const ()), // sub_142527680
    Replacement::draft(0x1425276b0, "luau_1425276b0", tpl_895::<0x8> as *const ()), // sub_1425276b0
    Replacement::draft(0x1425276d0, "luau_1425276d0", tpl_46878::<0x8> as *const ()), // sub_1425276d0
    Replacement::draft(0x1425276f0, "luau_1425276f0", fn_1425276f0 as *const ()), // sub_1425276f0
    Replacement::draft(0x142527710, "luau_142527710", fn_142527710 as *const ()), // sub_142527710
    Replacement::draft(0x142527730, "luau_142527730", fn_142527730 as *const ()), // sub_142527730
    Replacement::draft(0x142527750, "luau_142527750", tpl_1009::<0x4> as *const ()), // sub_142527750
    Replacement::draft(0x142527760, "luau_142527760", fn_142527760 as *const ()), // sub_142527760
    Replacement::draft(0x142527800, "luau_142527800", fn_142527800 as *const ()), // sub_142527800
    Replacement::draft_throws(0x1425279d0, "luau_1425279d0", fn_1425279d0 as *const (), Throws::Discard), // sub_1425279d0
    Replacement::draft_throws(0x1425285c0, "luau_1425285c0", fn_1425285c0 as *const (), Throws::Discard), // sub_1425285c0
    Replacement::draft(0x1425286d0, "luau_1425286d0", fn_1425286d0 as *const ()), // sub_1425286d0
    Replacement::draft_throws(0x142528860, "luau_142528860", fn_142528860 as *const (), Throws::Discard), // sub_142528860
    Replacement::draft_throws(0x142528ae0, "luau_142528ae0", fn_142528ae0 as *const (), Throws::Discard), // sub_142528ae0
    Replacement::draft(0x142528b40, "luau_142528b40", fn_142528b40 as *const ()), // sub_142528b40
    Replacement::draft(0x142528ca0, "luau_142528ca0", fn_142528ca0 as *const ()), // sub_142528ca0
    Replacement::draft(0x142528d70, "luau_142528d70", fn_142528d70 as *const ()), // sub_142528d70
    Replacement::draft(0x142528ee0, "luau_142528ee0", fn_142528ee0 as *const ()), // sub_142528ee0
    Replacement::draft(0x142528f60, "luau_142528f60", fn_142528f60 as *const ()), // sub_142528f60
    Replacement::draft(0x1425290f0, "luau_1425290f0", fn_1425290f0 as *const ()), // sub_1425290f0
    Replacement::draft(0x142529120, "luau_142529120", fn_142529120 as *const ()), // sub_142529120
    Replacement::draft(0x142529140, "luau_142529140", fn_142529140 as *const ()), // sub_142529140
    Replacement::draft(0x1425291e0, "luau_1425291e0", fn_1425291e0 as *const ()), // sub_1425291e0
    Replacement::draft(0x142529240, "luau_142529240", fn_142529240 as *const ()), // sub_142529240
    Replacement::draft_throws(0x1425292a0, "luau_1425292a0", fn_1425292a0 as *const (), Throws::Discard), // sub_1425292a0
    Replacement::draft_throws(0x142529380, "luau_142529380", fn_142529380 as *const (), Throws::Caught), // sub_142529380
    Replacement::draft(0x1425295c0, "luau_1425295c0", fn_1425295c0 as *const ()), // sub_1425295c0
    Replacement::draft(0x142529600, "luau_142529600", fn_142529600 as *const ()), // sub_142529600
    Replacement::draft(0x142529640, "luau_142529640", fn_142529640 as *const ()), // sub_142529640
    Replacement::draft(0x142529670, "luau_142529670", fn_142529670 as *const ()), // sub_142529670
    Replacement::draft(0x1425296a0, "luau_1425296a0", fn_1425296a0 as *const ()), // sub_1425296a0
    Replacement::draft(0x1425296d0, "luau_1425296d0", fn_1425296d0 as *const ()), // sub_1425296d0
    Replacement::draft_throws(0x142529710, "luau_142529710", fn_142529710 as *const (), Throws::Discard), // sub_142529710
    Replacement::draft_throws(0x142529740, "luau_142529740", fn_142529740 as *const (), Throws::Caught), // sub_142529740
    Replacement::draft(0x14252a340, "luau_14252a340", fn_14252a340 as *const ()), // sub_14252a340
    Replacement::draft_throws(0x14252a360, "luau_14252a360", fn_14252a360 as *const (), Throws::Discard), // sub_14252a360
    Replacement::draft_throws(0x14252a400, "luau_14252a400", fn_14252a400 as *const (), Throws::Discard), // sub_14252a400
    Replacement::draft_throws(0x14252a440, "luau_14252a440", fn_14252a440 as *const (), Throws::Discard), // sub_14252a440
    Replacement::draft_throws(0x14252a4d0, "luau_14252a4d0", fn_14252a4d0 as *const (), Throws::Discard), // sub_14252a4d0
    Replacement::draft_throws(0x14252a590, "luau_14252a590", fn_14252a590 as *const (), Throws::Discard), // sub_14252a590
    Replacement::draft_throws(0x14252ab40, "luau_14252ab40", fn_14252ab40 as *const (), Throws::Discard), // sub_14252ab40
    Replacement::draft_throws(0x14252abe0, "luau_14252abe0", fn_14252abe0 as *const (), Throws::Discard), // sub_14252abe0
    Replacement::draft_throws(0x14252acb0, "luau_14252acb0", fn_14252acb0 as *const (), Throws::Discard), // sub_14252acb0
    Replacement::draft(0x14252ad80, "luau_14252ad80", tpl_1743::<0x58> as *const ()), // sub_14252ad80
    Replacement::draft(0x14252ad90, "luau_14252ad90", fn_14252ad90 as *const ()), // sub_14252ad90
    Replacement::draft(0x14252adf0, "luau_14252adf0", fn_14252adf0 as *const ()), // sub_14252adf0
    Replacement::draft(0x14252ae10, "luau_14252ae10", fn_14252ae10 as *const ()), // sub_14252ae10
    Replacement::draft_throws(0x14252ae80, "luau_14252ae80", fn_14252ae80 as *const (), Throws::Caught), // sub_14252ae80
    Replacement::draft_throws(0x14252b3c0, "luau_14252b3c0", fn_14252b3c0 as *const (), Throws::Discard), // sub_14252b3c0
    Replacement::draft_throws(0x14252c480, "luau_14252c480", fn_14252c480 as *const (), Throws::Discard), // sub_14252c480
    Replacement::draft(0x14252c730, "luau_14252c730", fn_14252c730 as *const ()), // sub_14252c730
    Replacement::draft(0x14252c760, "luau_14252c760", fn_14252c760 as *const ()), // sub_14252c760
    Replacement::draft_throws(0x14252c770, "luau_14252c770", fn_14252c770 as *const (), Throws::Caught), // sub_14252c770
    Replacement::draft(0x14252c7e0, "luau_14252c7e0", fn_14252c7e0 as *const ()), // sub_14252c7e0
    Replacement::draft_throws(0x14252c7f0, "luau_14252c7f0", fn_14252c7f0 as *const (), Throws::Caught), // sub_14252c7f0
    Replacement::draft(0x14252cd50, "luau_14252cd50", fn_14252cd50 as *const ()), // sub_14252cd50
    Replacement::draft(0x14252ce60, "luau_14252ce60", fn_14252ce60 as *const ()), // sub_14252ce60
    Replacement::draft(0x14252cea0, "luau_14252cea0", fn_14252cea0 as *const ()), // sub_14252cea0
    Replacement::draft(0x14252cf00, "luau_14252cf00", fn_14252cf00 as *const ()), // sub_14252cf00
    Replacement::draft(0x14252cf20, "luau_14252cf20", fn_14252cf20 as *const ()), // sub_14252cf20
    Replacement::draft(0x14252cf40, "luau_14252cf40", fn_14252cf40 as *const ()), // sub_14252cf40
    Replacement::draft_throws(0x14252cf50, "luau_14252cf50", fn_14252cf50 as *const (), Throws::Caught), // sub_14252cf50
    Replacement::draft_throws(0x14252d390, "luau_14252d390", fn_14252d390 as *const (), Throws::Caught), // sub_14252d390
    Replacement::draft_throws(0x14252d700, "luau_14252d700", fn_14252d700 as *const (), Throws::Discard), // sub_14252d700
    Replacement::draft_throws(0x14252de90, "luau_14252de90", fn_14252de90 as *const (), Throws::Discard), // sub_14252de90
    Replacement::draft_throws(0x14252e0c0, "luau_14252e0c0", fn_14252e0c0 as *const (), Throws::Discard), // sub_14252e0c0
    Replacement::draft(0x14252e1f0, "luau_14252e1f0", tpl_52238::<0x28, 0x20> as *const ()), // sub_14252e1f0
    Replacement::draft_throws(0x14252e210, "luau_14252e210", fn_14252e210 as *const (), Throws::Discard), // sub_14252e210
    Replacement::draft_throws(0x14252e270, "luau_14252e270", fn_14252e270 as *const (), Throws::Discard), // sub_14252e270
    Replacement::draft(0x14252e310, "luau_14252e310", tpl_28365::<0x1a8> as *const ()), // sub_14252e310
    Replacement::draft(0x14252e320, "luau_14252e320", fn_14252e320 as *const ()), // sub_14252e320
    Replacement::draft_throws(0x14252e340, "luau_14252e340", fn_14252e340 as *const (), Throws::Discard), // sub_14252e340
    Replacement::draft(0x14252e420, "luau_14252e420", fn_14252e420 as *const ()), // sub_14252e420
    Replacement::draft_throws(0x14252e450, "luau_14252e450", fn_14252e450 as *const (), Throws::Discard), // sub_14252e450
    Replacement::draft(0x14252e4f0, "luau_14252e4f0", tpl_52238::<0x28, 0x20> as *const ()), // sub_14252e4f0
    Replacement::draft_throws(0x14252e510, "luau_14252e510", fn_14252e510 as *const (), Throws::Discard), // sub_14252e510
    Replacement::draft_throws(0x14252e580, "luau_14252e580", fn_14252e580 as *const (), Throws::Discard), // sub_14252e580
    Replacement::draft_throws(0x14252e600, "luau_14252e600", fn_14252e600 as *const (), Throws::Discard), // sub_14252e600
    Replacement::draft_throws(0x14252e680, "luau_14252e680", fn_14252e680 as *const (), Throws::Discard), // sub_14252e680
    Replacement::draft_throws(0x14252e770, "luau_14252e770", fn_14252e770 as *const (), Throws::Discard), // sub_14252e770
    Replacement::draft_throws(0x14252e7b0, "luau_14252e7b0", fn_14252e7b0 as *const (), Throws::Discard), // sub_14252e7b0
    Replacement::draft_throws(0x14252e950, "luau_14252e950", fn_14252e950 as *const (), Throws::Discard), // sub_14252e950
    Replacement::draft_throws(0x14252e990, "luau_14252e990", fn_14252e990 as *const (), Throws::Discard), // sub_14252e990
    Replacement::draft_throws(0x14252e9f0, "luau_14252e9f0", fn_14252e9f0 as *const (), Throws::Discard), // sub_14252e9f0
    Replacement::draft_throws(0x14252ea40, "luau_14252ea40", fn_14252ea40 as *const (), Throws::Discard), // sub_14252ea40
    Replacement::draft_throws(0x14252ea90, "luau_14252ea90", fn_14252ea90 as *const (), Throws::Discard), // sub_14252ea90
    Replacement::draft_throws(0x14252ead0, "luau_14252ead0", fn_14252ead0 as *const (), Throws::Discard), // sub_14252ead0
    Replacement::draft_throws(0x14252ebc0, "luau_14252ebc0", fn_14252ebc0 as *const (), Throws::Discard), // sub_14252ebc0
    Replacement::draft(0x14252ecb0, "luau_14252ecb0", tpl_28365::<0x1c> as *const ()), // sub_14252ecb0
    Replacement::draft_throws(0x14252ecc0, "luau_14252ecc0", fn_14252ecc0 as *const (), Throws::Discard), // sub_14252ecc0
    Replacement::draft_throws(0x14252efc0, "luau_14252efc0", fn_14252efc0 as *const (), Throws::Caught), // sub_14252efc0
    Replacement::draft(0x14252f200, "luau_14252f200", fn_14252f200 as *const ()), // sub_14252f200
    Replacement::draft(0x14252f240, "luau_14252f240", fn_14252f240 as *const ()), // sub_14252f240
    Replacement::draft(0x14252f290, "luau_14252f290", fn_14252f290 as *const ()), // sub_14252f290
    Replacement::draft(0x14252f2d0, "luau_14252f2d0", fn_14252f2d0 as *const ()), // sub_14252f2d0
    Replacement::draft(0x14252f310, "luau_14252f310", fn_14252f310 as *const ()), // sub_14252f310
    Replacement::draft(0x14252f360, "luau_14252f360", fn_14252f360 as *const ()), // sub_14252f360
    Replacement::draft(0x14252f430, "luau_14252f430", fn_14252f430 as *const ()), // sub_14252f430
    Replacement::draft_throws(0x14252f4c0, "luau_14252f4c0", fn_14252f4c0 as *const (), Throws::Discard), // sub_14252f4c0
    Replacement::draft_throws(0x14252f530, "luau_14252f530", fn_14252f530 as *const (), Throws::Discard), // sub_14252f530
    Replacement::draft_throws(0x14252f560, "luau_14252f560", fn_14252f560 as *const (), Throws::Discard), // sub_14252f560
    Replacement::draft_throws(0x14252f590, "luau_14252f590", fn_14252f590 as *const (), Throws::Discard), // sub_14252f590
    Replacement::draft(0x14252f5c0, "luau_14252f5c0", fn_14252f5c0 as *const ()), // sub_14252f5c0
    Replacement::draft_throws(0x14252f5f0, "luau_14252f5f0", fn_14252f5f0 as *const (), Throws::Caught), // sub_14252f5f0
    Replacement::draft_throws(0x14252f910, "luau_14252f910", fn_14252f910 as *const (), Throws::Caught), // sub_14252f910
    Replacement::draft(0x14252ffc0, "luau_14252ffc0", tpl_52272::<0x8> as *const ()), // sub_14252ffc0
    Replacement::draft(0x14252ffe0, "luau_14252ffe0", tpl_52272::<0x8> as *const ()), // sub_14252ffe0
    Replacement::draft(0x142530000, "luau_142530000", tpl_364::<0x8> as *const ()), // sub_142530000
    Replacement::draft_throws(0x142530030, "luau_142530030", fn_142530030 as *const (), Throws::Caught), // sub_142530030
    Replacement::draft(0x142530090, "luau_142530090", tpl_52134::<0x40, 0x28> as *const ()), // Luau::CompileError::Luau::CompileError::vfunc_1
    Replacement::draft(0x1425300b0, "luau_1425300b0", fn_1425300b0 as *const ()), // sub_1425300b0
    Replacement::draft_throws(0x1425300f0, "luau_1425300f0", fn_1425300f0 as *const (), Throws::Caught), // sub_1425300f0
    Replacement::draft(0x142530140, "luau_142530140", tpl_548::<0x18> as *const ()), // sub_142530140
    Replacement::draft_throws(0x142530150, "luau_142530150", fn_142530150 as *const (), Throws::Caught), // sub_142530150
    Replacement::draft(0x142530460, "luau_142530460", fn_142530460 as *const ()), // sub_142530460
    Replacement::draft(0x1425311c0, "luau_1425311c0", fn_1425311c0 as *const ()), // sub_1425311c0
    Replacement::draft(0x142531360, "luau_142531360", fn_142531360 as *const ()), // sub_142531360
    Replacement::draft(0x1425313a0, "luau_1425313a0", fn_1425313a0 as *const ()), // sub_1425313a0
    Replacement::draft(0x142531430, "luau_142531430", fn_142531430 as *const ()), // sub_142531430
    Replacement::draft(0x142531500, "luau_142531500", fn_142531500 as *const ()), // sub_142531500
    Replacement::draft_throws(0x142531560, "luau_142531560", fn_142531560 as *const (), Throws::Caught), // sub_142531560
    Replacement::draft_throws(0x142531720, "luau_142531720", fn_142531720 as *const (), Throws::Discard), // sub_142531720
    Replacement::draft(0x1425319a0, "luau_1425319a0", fn_1425319a0 as *const ()), // sub_1425319a0
    Replacement::draft(0x1425319d0, "luau_1425319d0", fn_1425319d0 as *const ()), // sub_1425319d0
    Replacement::draft(0x142531a80, "luau_142531a80", fn_142531a80 as *const ()), // sub_142531a80
    Replacement::draft(0x142531b40, "luau_142531b40", fn_142531b40 as *const ()), // sub_142531b40
    Replacement::draft(0x142531e90, "luau_142531e90", fn_142531e90 as *const ()), // sub_142531e90
    Replacement::draft(0x142531f10, "luau_142531f10", fn_142531f10 as *const ()), // sub_142531f10
    Replacement::draft(0x142531fa0, "luau_142531fa0", fn_142531fa0 as *const ()), // sub_142531fa0
    Replacement::draft(0x142532050, "unknown_142532050", fn_142532050 as *const ()), // sub_142532050
    Replacement::draft(0x142532060, "unknown_142532060", fn_142532060 as *const ()), // sub_142532060
    Replacement::draft(0x142532100, "unknown_142532100", fn_142532100 as *const ()), // sub_142532100
    Replacement::draft(0x142532130, "unknown_142532130", fn_142532130 as *const ()), // sub_142532130
    Replacement::draft(0x142532150, "unknown_142532150", fn_142532150 as *const ()), // sub_142532150
    Replacement::draft(0x142532170, "unknown_142532170", tpl_895::<0x8> as *const ()), // sub_142532170
    Replacement::draft(0x142532190, "luau_142532190", fn_142532190 as *const ()), // sub_142532190
    Replacement::draft(0x142532f70, "unknown_142532f70", tpl_7013::<0x142535360> as *const ()), // sub_142532f70
    Replacement::draft(0x142532f80, "unknown_142532f80", tpl_52291::<0xffffffffffffffff, 0x108, 0x4, 0xe, 0x1c, 0x142532ff0> as *const ()), // sub_142532f80
    Replacement::draft(0x142532ff0, "unknown_142532ff0", fn_142532ff0 as *const ()), // sub_142532ff0
    Replacement::draft(0x1425331e0, "unknown_1425331e0", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_1425331e0
    Replacement::draft(0x142533250, "unknown_142533250", fn_142533250 as *const ()), // sub_142533250
    Replacement::draft(0x1425334b0, "unknown_1425334b0", tpl_369 as *const ()), // sub_1425334b0
    Replacement::draft(0x1425334c0, "unknown_1425334c0", tpl_369 as *const ()), // sub_1425334c0
    Replacement::draft(0x1425334d0, "unknown_1425334d0", tpl_369 as *const ()), // sub_1425334d0
    Replacement::draft(0x1425334e0, "unknown_1425334e0", tpl_52299::<0x1c> as *const ()), // sub_1425334e0
    Replacement::draft(0x142533500, "unknown_142533500", tpl_496 as *const ()), // sub_142533500
    Replacement::draft(0x142533510, "unknown_142533510", fn_142533510 as *const ()), // sub_142533510
    Replacement::draft(0x142533530, "unknown_142533530", tpl_52301 as *const ()), // sub_142533530
    Replacement::draft(0x142533540, "unknown_142533540", tpl_110617::<0x143a09580, 0x14183acc2> as *const ()), // sub_142533540
    Replacement::draft(0x142533580, "unknown_142533580", tpl_110617::<0x143a09580, 0x14183acc2> as *const ()), // sub_142533580
    Replacement::draft(0x1425335c0, "unknown_1425335c0", tpl_52302 as *const ()), // sub_1425335c0
    Replacement::draft(0x1425335d0, "unknown_1425335d0", tpl_52303 as *const ()), // sub_1425335d0
    Replacement::draft(0x1425335e0, "unknown_1425335e0", tpl_52302 as *const ()), // sub_1425335e0
    Replacement::draft(0x1425335f0, "unknown_1425335f0", tpl_52303 as *const ()), // sub_1425335f0
    Replacement::draft(0x142533600, "unknown_142533600", tpl_29360::<0xe> as *const ()), // sub_142533600
    Replacement::draft(0x142533610, "unknown_142533610", tpl_1005::<0xe> as *const ()), // sub_142533610
    Replacement::draft(0x142533620, "unknown_142533620", tpl_44345::<0x18> as *const ()), // sub_142533620
    Replacement::draft(0x142533630, "unknown_142533630", fn_142533630 as *const ()), // sub_142533630
    Replacement::draft(0x142533640, "unknown_142533640", tpl_52305 as *const ()), // sub_142533640
    Replacement::draft(0x142533660, "unknown_142533660", tpl_52306 as *const ()), // sub_142533660
    Replacement::draft(0x142533670, "unknown_142533670", tpl_52307 as *const ()), // sub_142533670
    Replacement::draft(0x142533680, "unknown_142533680", tpl_52308 as *const ()), // sub_142533680
    Replacement::draft(0x142533690, "unknown_142533690", tpl_52299::<0x1c> as *const ()), // sub_142533690
    Replacement::draft(0x1425336b0, "unknown_1425336b0", tpl_52299::<0x1c> as *const ()), // sub_1425336b0
    Replacement::draft(0x1425336d0, "unknown_1425336d0", fn_1425336d0 as *const ()), // sub_1425336d0
    Replacement::draft(0x142533740, "unknown_142533740", tpl_52310::<0x4> as *const ()), // sub_142533740
    Replacement::draft(0x142533770, "unknown_142533770", tpl_52311::<0x4> as *const ()), // sub_142533770
    Replacement::draft(0x142533780, "unknown_142533780", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_142533780
    Replacement::draft(0x1425337a0, "unknown_1425337a0", tpl_361 as *const ()), // sub_1425337a0
    Replacement::draft(0x1425337b0, "unknown_1425337b0", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_1425337b0
    Replacement::draft(0x1425337d0, "unknown_1425337d0", tpl_52312::<0x7, 0x8> as *const ()), // sub_1425337d0
    Replacement::draft(0x1425337f0, "unknown_1425337f0", tpl_52313::<0x7> as *const ()), // sub_1425337f0
    Replacement::draft(0x142533800, "unknown_142533800", tpl_473 as *const ()), // sub_142533800
    Replacement::draft(0x142533810, "unknown_142533810", tpl_520 as *const ()), // sub_142533810
    Replacement::draft(0x142533830, "unknown_142533830", tpl_520 as *const ()), // sub_142533830
    Replacement::draft(0x142533850, "unknown_142533850", tpl_52291::<0xffffffffffffffff, 0x108, 0x4, 0xe, 0x1c, 0x142532ff0> as *const ()), // sub_142533850
    Replacement::draft(0x1425338c0, "unknown_1425338c0", fn_1425338c0 as *const ()), // sub_1425338c0
    Replacement::draft(0x142533940, "unknown_142533940", fn_142533940 as *const ()), // sub_142533940
    Replacement::draft(0x142533a80, "unknown_142533a80", fn_142533a80 as *const ()), // sub_142533a80
    Replacement::draft(0x142533dd0, "unknown_142533dd0", fn_142533dd0 as *const ()), // sub_142533dd0
    Replacement::draft(0x142534160, "unknown_142534160", fn_142534160 as *const ()), // sub_142534160
    Replacement::draft(0x142534190, "unknown_142534190", fn_142534190 as *const ()), // sub_142534190
    Replacement::draft(0x1425342d0, "unknown_1425342d0", tpl_369 as *const ()), // sub_1425342d0
    Replacement::draft(0x1425342e0, "unknown_1425342e0", fn_1425342e0 as *const ()), // sub_1425342e0
    Replacement::draft(0x142534300, "unknown_142534300", tpl_28185::<0x143844164> as *const ()), // sub_142534300
    Replacement::draft(0x142534310, "unknown_142534310", fn_142534310 as *const ()), // sub_142534310
    Replacement::draft(0x142534330, "crt_142534330", tpl_906::<0x142645318> as *const ()), // j_FlsSetValue
    Replacement::draft(0x142534340, "unknown_142534340", fn_142534340 as *const ()), // sub_142534340
    Replacement::draft(0x142534500, "unknown_142534500", fn_142534500 as *const ()), // sub_142534500
    Replacement::draft(0x142534560, "unknown_142534560", tpl_366::<0x143867148> as *const ()), // sub_142534560
    Replacement::draft(0x142534570, "unknown_142534570", fn_142534570 as *const ()), // sub_142534570
    Replacement::draft(0x1425345a0, "unknown_1425345a0", fn_1425345a0 as *const ()), // sub_1425345a0
    Replacement::draft(0x142534600, "unknown_142534600", fn_142534600 as *const ()), // sub_142534600
    Replacement::draft(0x142534780, "unknown_142534780", fn_142534780 as *const ()), // sub_142534780
    Replacement::draft(0x142534a50, "unknown_142534a50", fn_142534a50 as *const ()), // sub_142534a50
    Replacement::draft(0x142534ae0, "unknown_142534ae0", fn_142534ae0 as *const ()), // sub_142534ae0
    Replacement::draft(0x142534b50, "unknown_142534b50", fn_142534b50 as *const ()), // sub_142534b50
    Replacement::draft(0x142534c50, "unknown_142534c50", fn_142534c50 as *const ()), // sub_142534c50
    Replacement::draft(0x142534c70, "unknown_142534c70", fn_142534c70 as *const ()), // sub_142534c70
    Replacement::draft(0x142534cf0, "unknown_142534cf0", tpl_52301 as *const ()), // sub_142534cf0
    Replacement::draft(0x142534d00, "unknown_142534d00", tpl_52329 as *const ()), // sub_142534d00
    Replacement::draft(0x142534d20, "unknown_142534d20", tpl_52301 as *const ()), // sub_142534d20
    Replacement::draft(0x142534d30, "unknown_142534d30", tpl_110636::<0x143a09580, 0x14183acc2> as *const ()), // sub_142534d30
    Replacement::draft(0x142534d70, "unknown_142534d70", tpl_110637::<0x143a09580, 0x14183acbc> as *const ()), // sub_142534d70
    Replacement::draft(0x142534db0, "unknown_142534db0", tpl_110636::<0x143a09580, 0x14183acc2> as *const ()), // sub_142534db0
    Replacement::draft(0x142534df0, "unknown_142534df0", tpl_110637::<0x143a09580, 0x14183acbc> as *const ()), // sub_142534df0
    Replacement::draft(0x142534e30, "unknown_142534e30", tpl_52330::<0x143073d80> as *const ()), // sub_142534e30
    Replacement::draft(0x142534e50, "unknown_142534e50", fn_142534e50 as *const ()), // sub_142534e50
    Replacement::draft(0x142534e80, "unknown_142534e80", tpl_473 as *const ()), // sub_142534e80
    Replacement::draft(0x142534e90, "unknown_142534e90", fn_142534e90 as *const ()), // sub_142534e90
    Replacement::draft(0x142534ea0, "unknown_142534ea0", tpl_520 as *const ()), // sub_142534ea0
    Replacement::draft(0x142534ec0, "unknown_142534ec0", tpl_520 as *const ()), // sub_142534ec0
    Replacement::draft(0x142534ee0, "unknown_142534ee0", tpl_52333::<0x143867148> as *const ()), // sub_142534ee0
    Replacement::draft(0x142534f00, "unknown_142534f00", fn_142534f00 as *const ()), // sub_142534f00
    Replacement::draft(0x142534f10, "unknown_142534f10", fn_142534f10 as *const ()), // sub_142534f10
    Replacement::draft(0x142535000, "unknown_142535000", fn_142535000 as *const ()), // sub_142535000
    Replacement::draft(0x1425351d0, "unknown_1425351d0", tpl_52329 as *const ()), // sub_1425351d0
    Replacement::draft(0x1425351f0, "unknown_1425351f0", tpl_110617::<0x143a09580, 0x14183acc2> as *const ()), // sub_1425351f0
    Replacement::draft(0x142535230, "unknown_142535230", tpl_110617::<0x143a09580, 0x14183acc2> as *const ()), // sub_142535230
    Replacement::draft(0x142535270, "unknown_142535270", tpl_47165::<0xe> as *const ()), // sub_142535270
    Replacement::draft(0x142535280, "unknown_142535280", fn_142535280 as *const ()), // sub_142535280
    Replacement::draft(0x1425352b0, "unknown_1425352b0", tpl_52310::<0x4> as *const ()), // sub_1425352b0
    Replacement::draft(0x1425352e0, "unknown_1425352e0", tpl_52311::<0x4> as *const ()), // sub_1425352e0
    Replacement::draft(0x1425352f0, "unknown_1425352f0", tpl_361 as *const ()), // sub_1425352f0
    Replacement::draft(0x142535300, "unknown_142535300", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_142535300
    Replacement::draft(0x142535320, "unknown_142535320", tpl_52312::<0x7, 0x8> as *const ()), // sub_142535320
    Replacement::draft(0x142535340, "unknown_142535340", tpl_52313::<0x7> as *const ()), // sub_142535340
    Replacement::draft(0x142535350, "unknown_142535350", fn_142535350 as *const ()), // sub_142535350
    Replacement::draft(0x142535590, "unknown_142535590", fn_142535590 as *const ()), // sub_142535590
    Replacement::draft(0x142535600, "unknown_142535600", fn_142535600 as *const ()), // sub_142535600
    Replacement::draft(0x142535660, "unknown_142535660", fn_142535660 as *const ()), // sub_142535660
    Replacement::draft(0x142535990, "unknown_142535990", fn_142535990 as *const ()), // sub_142535990
    Replacement::draft(0x142535a00, "unknown_142535a00", fn_142535a00 as *const ()), // sub_142535a00
    Replacement::draft(0x142535b90, "luau_142535b90", fn_142535b90 as *const ()), // sub_142535b90
    Replacement::draft(0x142536150, "unknown_142536150", fn_142536150 as *const ()), // sub_142536150
    Replacement::draft(0x1425362a0, "unknown_1425362a0", fn_1425362a0 as *const ()), // sub_1425362a0
    Replacement::draft(0x142536420, "unknown_142536420", fn_142536420 as *const ()), // sub_142536420
    Replacement::draft(0x1425364c0, "unknown_1425364c0", fn_1425364c0 as *const ()), // sub_1425364c0
    Replacement::draft(0x142536640, "unknown_142536640", fn_142536640 as *const ()), // sub_142536640
    Replacement::draft(0x1425366d0, "unknown_1425366d0", fn_1425366d0 as *const ()), // sub_1425366d0
    Replacement::draft(0x142536750, "unknown_142536750", fn_142536750 as *const ()), // sub_142536750
    Replacement::draft(0x1425367c0, "unknown_1425367c0", fn_1425367c0 as *const ()), // sub_1425367c0
    Replacement::draft(0x1425367d0, "unknown_1425367d0", fn_1425367d0 as *const ()), // sub_1425367d0
    Replacement::draft(0x142536a00, "unknown_142536a00", fn_142536a00 as *const ()), // sub_142536a00
    Replacement::draft(0x142536c20, "unknown_142536c20", fn_142536c20 as *const ()), // sub_142536c20
    Replacement::draft(0x142536c60, "unknown_142536c60", fn_142536c60 as *const ()), // sub_142536c60
    Replacement::draft(0x142536ca0, "unknown_142536ca0", tpl_365::<0x142536cb0> as *const ()), // sub_142536ca0
    Replacement::draft(0x142536d20, "unknown_142536d20", fn_142536d20 as *const ()), // sub_142536d20
    Replacement::draft(0x142536d40, "unknown_142536d40", fn_142536d40 as *const ()), // sub_142536d40
    Replacement::draft(0x142536d60, "unknown_142536d60", fn_142536d60 as *const ()), // sub_142536d60
    Replacement::draft(0x142536d80, "unknown_142536d80", tpl_110617::<0x143a09580, 0x14183acc2> as *const ()), // sub_142536d80
    Replacement::draft(0x142536dc0, "unknown_142536dc0", tpl_110617::<0x143a09580, 0x14183acc2> as *const ()), // sub_142536dc0
    Replacement::draft(0x142536e00, "unknown_142536e00", tpl_52370 as *const ()), // sub_142536e00
    Replacement::draft(0x142536e30, "unknown_142536e30", tpl_52371 as *const ()), // sub_142536e30
    Replacement::draft(0x142536e50, "luau_142536e50", tpl_52372 as *const ()), // sub_142536e50
    Replacement::draft(0x142536ea0, "unknown_142536ea0", tpl_52302 as *const ()), // sub_142536ea0
    Replacement::draft(0x142536eb0, "unknown_142536eb0", tpl_52303 as *const ()), // sub_142536eb0
    Replacement::draft(0x142536ec0, "unknown_142536ec0", tpl_52302 as *const ()), // sub_142536ec0
    Replacement::draft(0x142536ed0, "unknown_142536ed0", tpl_52303 as *const ()), // sub_142536ed0
    Replacement::draft(0x142536ee0, "unknown_142536ee0", fn_142536ee0 as *const ()), // sub_142536ee0
    Replacement::draft(0x142536ef0, "unknown_142536ef0", fn_142536ef0 as *const ()), // sub_142536ef0
    Replacement::draft(0x142536f10, "unknown_142536f10", tpl_1005::<0xe> as *const ()), // sub_142536f10
    Replacement::draft(0x142536f20, "unknown_142536f20", fn_142536f20 as *const ()), // sub_142536f20
    Replacement::draft(0x142536f50, "unknown_142536f50", tpl_497::<0x10> as *const ()), // sub_142536f50
    Replacement::draft(0x142536f60, "unknown_142536f60", tpl_44345::<0x18> as *const ()), // sub_142536f60
    Replacement::draft(0x142536f70, "unknown_142536f70", tpl_52307 as *const ()), // sub_142536f70
    Replacement::draft(0x142536f80, "unknown_142536f80", tpl_52305 as *const ()), // sub_142536f80
    Replacement::draft(0x142536fa0, "unknown_142536fa0", tpl_52306 as *const ()), // sub_142536fa0
    Replacement::draft(0x142536fb0, "unknown_142536fb0", tpl_52307 as *const ()), // sub_142536fb0
    Replacement::draft(0x142536fc0, "unknown_142536fc0", tpl_52308 as *const ()), // sub_142536fc0
    Replacement::draft(0x142536fd0, "unknown_142536fd0", tpl_1738::<0x30> as *const ()), // sub_142536fd0
    Replacement::draft(0x142536fe0, "unknown_142536fe0", tpl_913::<0x30> as *const ()), // sub_142536fe0
    Replacement::draft(0x142536ff0, "unknown_142536ff0", tpl_52376::<0x28> as *const ()), // sub_142536ff0
    Replacement::draft(0x142537000, "unknown_142537000", tpl_52299::<0x1c> as *const ()), // sub_142537000
    Replacement::draft(0x142537020, "unknown_142537020", tpl_52299::<0x1c> as *const ()), // sub_142537020
    Replacement::draft(0x142537040, "unknown_142537040", tpl_52377::<0x1c> as *const ()), // sub_142537040
    Replacement::draft(0x1425370c0, "unknown_1425370c0", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_1425370c0
    Replacement::draft(0x1425370e0, "unknown_1425370e0", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_1425370e0
    Replacement::draft(0x142537100, "unknown_142537100", tpl_52330::<0x143073d80> as *const ()), // sub_142537100
    Replacement::draft(0x142537120, "unknown_142537120", tpl_52313::<0x7> as *const ()), // sub_142537120
    Replacement::draft(0x142537130, "unknown_142537130", tpl_369 as *const ()), // sub_142537130
    Replacement::draft(0x142537140, "unknown_142537140", tpl_42543 as *const ()), // sub_142537140
    Replacement::draft(0x142537150, "unknown_142537150", tpl_473 as *const ()), // sub_142537150
    Replacement::draft(0x142537160, "unknown_142537160", tpl_520 as *const ()), // sub_142537160
    Replacement::draft(0x142537180, "unknown_142537180", tpl_520 as *const ()), // sub_142537180
    Replacement::draft(0x1425373f0, "unknown_1425373f0", fn_1425373f0 as *const ()), // sub_1425373f0
    Replacement::draft(0x142537450, "unknown_142537450", fn_142537450 as *const ()), // sub_142537450
    Replacement::draft(0x142537540, "unknown_142537540", fn_142537540 as *const ()), // sub_142537540
    Replacement::draft(0x1425375a0, "unknown_1425375a0", fn_1425375a0 as *const ()), // sub_1425375a0
    Replacement::draft(0x1425378e0, "unknown_1425378e0", fn_1425378e0 as *const ()), // sub_1425378e0
    Replacement::draft(0x142537920, "unknown_142537920", fn_142537920 as *const ()), // sub_142537920
    Replacement::draft(0x142537950, "unknown_142537950", fn_142537950 as *const ()), // sub_142537950
    Replacement::draft(0x1425379b0, "unknown_1425379b0", fn_1425379b0 as *const ()), // sub_1425379b0
    Replacement::draft(0x142537a00, "unknown_142537a00", fn_142537a00 as *const ()), // sub_142537a00
    Replacement::draft(0x142537a20, "unknown_142537a20", tpl_369 as *const ()), // sub_142537a20
    Replacement::draft(0x142537a30, "unknown_142537a30", fn_142537a30 as *const ()), // sub_142537a30
    Replacement::draft(0x142537ad0, "unknown_142537ad0", fn_142537ad0 as *const ()), // sub_142537ad0
    Replacement::draft(0x142537b60, "unknown_142537b60", fn_142537b60 as *const ()), // sub_142537b60
    Replacement::draft(0x142537d10, "unknown_142537d10", fn_142537d10 as *const ()), // sub_142537d10
    Replacement::draft(0x142537e00, "unknown_142537e00", fn_142537e00 as *const ()), // sub_142537e00
    Replacement::draft(0x142537f10, "unknown_142537f10", tpl_365::<0x142537f30> as *const ()), // sub_142537f10
    Replacement::draft(0x142537f20, "unknown_142537f20", tpl_365::<0x142537f50> as *const ()), // sub_142537f20
    Replacement::draft(0x142537f30, "unknown_142537f30", fn_142537f30 as *const ()), // sub_142537f30
    Replacement::draft(0x142537f50, "unknown_142537f50", fn_142537f50 as *const ()), // sub_142537f50
    Replacement::draft(0x142537f80, "unknown_142537f80", fn_142537f80 as *const ()), // sub_142537f80
    Replacement::draft(0x142537fe0, "unknown_142537fe0", fn_142537fe0 as *const ()), // sub_142537fe0
    Replacement::draft(0x142538010, "unknown_142538010", fn_142538010 as *const ()), // sub_142538010
    Replacement::draft(0x142538040, "unknown_142538040", fn_142538040 as *const ()), // sub_142538040
    Replacement::draft(0x142538090, "unknown_142538090", fn_142538090 as *const ()), // sub_142538090
    Replacement::draft(0x142538150, "unknown_142538150", fn_142538150 as *const ()), // sub_142538150
    Replacement::draft(0x142538260, "unknown_142538260", fn_142538260 as *const ()), // sub_142538260
    Replacement::draft(0x1425382a0, "unknown_1425382a0", tpl_110637::<0x143a09580, 0x14183acbc> as *const ()), // sub_1425382a0
    Replacement::draft(0x1425382e0, "unknown_1425382e0", tpl_473 as *const ()), // sub_1425382e0
    Replacement::draft(0x1425382f0, "unknown_1425382f0", tpl_52402 as *const ()), // sub_1425382f0
    Replacement::draft(0x142538300, "unknown_142538300", fn_142538300 as *const ()), // sub_142538300
    Replacement::draft(0x1425383f0, "unknown_1425383f0", fn_1425383f0 as *const ()), // sub_1425383f0
    Replacement::draft(0x1425384a0, "unknown_1425384a0", fn_1425384a0 as *const ()), // sub_1425384a0
    Replacement::draft(0x142538590, "unknown_142538590", fn_142538590 as *const ()), // sub_142538590
    Replacement::draft(0x1425385e0, "unknown_1425385e0", fn_1425385e0 as *const ()), // sub_1425385e0
    Replacement::draft(0x142538650, "unknown_142538650", fn_142538650 as *const ()), // sub_142538650
    Replacement::draft(0x142538760, "unknown_142538760", fn_142538760 as *const ()), // sub_142538760
    Replacement::draft(0x142538780, "unknown_142538780", fn_142538780 as *const ()), // sub_142538780
    Replacement::draft(0x142538840, "unknown_142538840", fn_142538840 as *const ()), // sub_142538840
    Replacement::draft(0x142538920, "unknown_142538920", fn_142538920 as *const ()), // sub_142538920
    Replacement::draft(0x142538a60, "unknown_142538a60", fn_142538a60 as *const ()), // sub_142538a60
    Replacement::draft(0x142538e90, "unknown_142538e90", fn_142538e90 as *const ()), // sub_142538e90
    Replacement::draft(0x142538f40, "unknown_142538f40", tpl_28345::<0x142538f60> as *const ()), // sub_142538f40
    Replacement::draft(0x142538f50, "unknown_142538f50", fn_142538f50 as *const ()), // sub_142538f50
    Replacement::draft(0x142538f60, "unknown_142538f60", fn_142538f60 as *const ()), // sub_142538f60
    Replacement::draft(0x142539080, "unknown_142539080", fn_142539080 as *const ()), // sub_142539080
    Replacement::draft(0x1425390a0, "unknown_1425390a0", fn_1425390a0 as *const ()), // sub_1425390a0
    Replacement::draft(0x1425391a0, "unknown_1425391a0", fn_1425391a0 as *const ()), // sub_1425391a0
    Replacement::draft(0x142539330, "unknown_142539330", fn_142539330 as *const ()), // sub_142539330
    Replacement::draft(0x142539350, "unknown_142539350", fn_142539350 as *const ()), // sub_142539350
    Replacement::draft(0x142539460, "unknown_142539460", fn_142539460 as *const ()), // sub_142539460
    Replacement::draft(0x142539560, "unknown_142539560", fn_142539560 as *const ()), // sub_142539560
    Replacement::draft(0x1425395e0, "unknown_1425395e0", fn_1425395e0 as *const ()), // sub_1425395e0
    Replacement::draft(0x1425396a0, "unknown_1425396a0", fn_1425396a0 as *const ()), // sub_1425396a0
    Replacement::draft(0x1425397a0, "unknown_1425397a0", fn_1425397a0 as *const ()), // sub_1425397a0
    Replacement::draft(0x1425398e0, "unknown_1425398e0", fn_1425398e0 as *const ()), // sub_1425398e0
    Replacement::draft(0x142539b00, "unknown_142539b00", fn_142539b00 as *const ()), // sub_142539b00
    Replacement::draft(0x142539cc0, "unknown_142539cc0", fn_142539cc0 as *const ()), // sub_142539cc0
    Replacement::draft(0x142539e50, "unknown_142539e50", fn_142539e50 as *const ()), // sub_142539e50
    Replacement::draft(0x142539f60, "unknown_142539f60", fn_142539f60 as *const ()), // sub_142539f60
    Replacement::draft(0x14253a050, "unknown_14253a050", tpl_52419::<0xffffffffffffffff> as *const ()), // sub_14253a050
    Replacement::draft(0x14253a080, "unknown_14253a080", tpl_52420::<0xffffffffffffffff> as *const ()), // sub_14253a080
    Replacement::draft(0x14253a0b0, "unknown_14253a0b0", fn_14253a0b0 as *const ()), // sub_14253a0b0
    Replacement::draft(0x14253a130, "unknown_14253a130", fn_14253a130 as *const ()), // sub_14253a130
    Replacement::draft(0x14253a150, "unknown_14253a150", tpl_28185::<0x143872f4a> as *const ()), // sub_14253a150
    Replacement::draft(0x14253a160, "unknown_14253a160", tpl_28185::<0x143872f48> as *const ()), // sub_14253a160
    Replacement::draft(0x14253a170, "unknown_14253a170", tpl_52329 as *const ()), // sub_14253a170
    Replacement::draft(0x14253a190, "unknown_14253a190", fn_14253a190 as *const ()), // sub_14253a190
    Replacement::draft(0x14253a1b0, "unknown_14253a1b0", fn_14253a1b0 as *const ()), // sub_14253a1b0
    Replacement::draft(0x14253a1f0, "unknown_14253a1f0", tpl_52424::<0x10> as *const ()), // sub_14253a1f0
    Replacement::draft(0x14253a220, "unknown_14253a220", tpl_52425::<0x10, 0x14> as *const ()), // sub_14253a220
    Replacement::draft(0x14253a240, "unknown_14253a240", tpl_52419::<0xffffffffffffffff> as *const ()), // sub_14253a240
    Replacement::draft(0x14253a270, "unknown_14253a270", tpl_52420::<0xffffffffffffffff> as *const ()), // sub_14253a270
    Replacement::draft(0x14253a2a0, "unknown_14253a2a0", tpl_52426::<0xfffffffffffffffd> as *const ()), // sub_14253a2a0
    Replacement::draft(0x14253a2b0, "unknown_14253a2b0", tpl_48139::<0x143d01c60> as *const ()), // sub_14253a2b0
    Replacement::draft(0x14253a2c0, "unknown_14253a2c0", tpl_473 as *const ()), // sub_14253a2c0
    Replacement::draft(0x14253a2d0, "unknown_14253a2d0", tpl_520 as *const ()), // sub_14253a2d0
    Replacement::draft(0x14253a2f0, "unknown_14253a2f0", fn_14253a2f0 as *const ()), // sub_14253a2f0
    Replacement::draft(0x14253a3b0, "std_14253a3b0", tpl_52429::<0x143872f48, 0x143872f4a, 0x142645720, 0x143872f30, 0x143872f40, 0x143425fc8, 0x142645308, 0x143426040, 0x142645758, 0x143bc5fd8, 0x143426058, 0x142645750, 0x143426068, 0x143426078, 0x143bfe128, 0x143425fd8, 0x143425fe8, 0x143426008, 0x143bce008, 0x143426020, 0x143be6098, 0x1434260e0, 0x143bee0c8, 0x143bde068, 0x143872cb4, 0x143872cb0, 0x1425375a0, 0x143872cd4, 0x143872cd0, 0x143872f38, 0x142541180> as *const ()), // sub_14253a3b0
    Replacement::draft(0x14253a550, "unknown_14253a550", fn_14253a550 as *const ()), // sub_14253a550
    Replacement::draft(0x14253a6a0, "unknown_14253a6a0", fn_14253a6a0 as *const ()), // sub_14253a6a0
    Replacement::draft(0x14253a6e0, "unknown_14253a6e0", tpl_366::<0x143872f30> as *const ()), // sub_14253a6e0
    Replacement::draft(0x14253a6f0, "unknown_14253a6f0", fn_14253a6f0 as *const ()), // sub_14253a6f0
    Replacement::draft(0x14253a750, "unknown_14253a750", fn_14253a750 as *const ()), // sub_14253a750
    Replacement::draft(0x14253a7b0, "unknown_14253a7b0", fn_14253a7b0 as *const ()), // sub_14253a7b0
    Replacement::draft(0x14253a7f0, "unknown_14253a7f0", tpl_473 as *const ()), // sub_14253a7f0
    Replacement::draft(0x14253a800, "unknown_14253a800", tpl_520 as *const ()), // sub_14253a800
    Replacement::draft(0x14253a820, "unknown_14253a820", tpl_520 as *const ()), // sub_14253a820
    Replacement::draft(0x14253a840, "unknown_14253a840", fn_14253a840 as *const ()), // sub_14253a840
    Replacement::draft(0x14253aa70, "unknown_14253aa70", fn_14253aa70 as *const ()), // sub_14253aa70
    Replacement::draft(0x14253aae0, "unknown_14253aae0", fn_14253aae0 as *const ()), // sub_14253aae0
    Replacement::draft(0x14253abc0, "unknown_14253abc0", fn_14253abc0 as *const ()), // sub_14253abc0
    Replacement::draft(0x14253ad70, "unknown_14253ad70", fn_14253ad70 as *const ()), // sub_14253ad70
    Replacement::draft(0x14253add0, "unknown_14253add0", fn_14253add0 as *const ()), // sub_14253add0
    Replacement::draft(0x14253b0b0, "unknown_14253b0b0", fn_14253b0b0 as *const ()), // sub_14253b0b0
    Replacement::draft(0x14253b260, "unknown_14253b260", fn_14253b260 as *const ()), // sub_14253b260
    Replacement::draft(0x14253b2a0, "unknown_14253b2a0", fn_14253b2a0 as *const ()), // sub_14253b2a0
    Replacement::draft(0x14253b430, "unknown_14253b430", fn_14253b430 as *const ()), // sub_14253b430
    Replacement::draft(0x14253b4e0, "unknown_14253b4e0", fn_14253b4e0 as *const ()), // sub_14253b4e0
    Replacement::draft(0x14253b500, "unknown_14253b500", fn_14253b500 as *const ()), // sub_14253b500
    Replacement::draft(0x14253b550, "unknown_14253b550", fn_14253b550 as *const ()), // sub_14253b550
    Replacement::draft(0x14253b630, "unknown_14253b630", fn_14253b630 as *const ()), // sub_14253b630
    Replacement::draft(0x14253b670, "unknown_14253b670", fn_14253b670 as *const ()), // sub_14253b670
    Replacement::draft(0x14253b690, "unknown_14253b690", fn_14253b690 as *const ()), // sub_14253b690
    Replacement::draft(0x14253b840, "unknown_14253b840", fn_14253b840 as *const ()), // sub_14253b840
    Replacement::draft(0x14253b9c0, "unknown_14253b9c0", fn_14253b9c0 as *const ()), // sub_14253b9c0
    Replacement::draft(0x14253bc40, "unknown_14253bc40", fn_14253bc40 as *const ()), // sub_14253bc40
    Replacement::draft(0x14253c1b0, "unknown_14253c1b0", fn_14253c1b0 as *const ()), // sub_14253c1b0
    Replacement::draft(0x14253c530, "unknown_14253c530", fn_14253c530 as *const ()), // sub_14253c530
    Replacement::draft(0x14253c750, "unknown_14253c750", tpl_29778::<0x100> as *const ()), // sub_14253c750
    Replacement::draft(0x14253cca0, "unknown_14253cca0", fn_14253cca0 as *const ()), // sub_14253cca0
    Replacement::draft(0x14253d0c0, "unknown_14253d0c0", fn_14253d0c0 as *const ()), // sub_14253d0c0
    Replacement::draft(0x14253d680, "unknown_14253d680", fn_14253d680 as *const ()), // sub_14253d680
    Replacement::draft(0x14253d7e0, "unknown_14253d7e0", tpl_369 as *const ()), // sub_14253d7e0
    Replacement::draft(0x14253d7f0, "unknown_14253d7f0", fn_14253d7f0 as *const ()), // sub_14253d7f0
    Replacement::draft(0x14253d8f0, "unknown_14253d8f0", fn_14253d8f0 as *const ()), // sub_14253d8f0
    Replacement::draft(0x14253d9d0, "unknown_14253d9d0", fn_14253d9d0 as *const ()), // sub_14253d9d0
    Replacement::draft(0x14253da50, "unknown_14253da50", fn_14253da50 as *const ()), // sub_14253da50
    Replacement::draft(0x14253dad0, "unknown_14253dad0", tpl_52465::<0xe8> as *const ()), // sub_14253dad0
    Replacement::draft(0x14253daf0, "unknown_14253daf0", fn_14253daf0 as *const ()), // sub_14253daf0
    Replacement::draft(0x14253db00, "unknown_14253db00", fn_14253db00 as *const ()), // sub_14253db00
    Replacement::draft(0x14253db60, "unknown_14253db60", fn_14253db60 as *const ()), // sub_14253db60
    Replacement::draft(0x14253dbc0, "unknown_14253dbc0", fn_14253dbc0 as *const ()), // sub_14253dbc0
    Replacement::draft(0x14253dc00, "unknown_14253dc00", fn_14253dc00 as *const ()), // sub_14253dc00
    Replacement::draft(0x14253dc40, "unknown_14253dc40", fn_14253dc40 as *const ()), // sub_14253dc40
    Replacement::draft(0x14253dca0, "unknown_14253dca0", fn_14253dca0 as *const ()), // sub_14253dca0
    Replacement::draft(0x14253dd00, "unknown_14253dd00", fn_14253dd00 as *const ()), // sub_14253dd00
    Replacement::draft(0x14253dd40, "unknown_14253dd40", fn_14253dd40 as *const ()), // sub_14253dd40
    Replacement::draft(0x14253dd60, "unknown_14253dd60", fn_14253dd60 as *const ()), // sub_14253dd60
    Replacement::draft(0x14253ddf0, "unknown_14253ddf0", fn_14253ddf0 as *const ()), // sub_14253ddf0
    Replacement::draft(0x14253dfa0, "unknown_14253dfa0", fn_14253dfa0 as *const ()), // sub_14253dfa0
    Replacement::draft(0x14253e080, "unknown_14253e080", fn_14253e080 as *const ()), // sub_14253e080
    Replacement::draft(0x14253e0c0, "unknown_14253e0c0", fn_14253e0c0 as *const ()), // sub_14253e0c0
    Replacement::draft(0x14253e100, "unknown_14253e100", fn_14253e100 as *const ()), // sub_14253e100
    Replacement::draft(0x14253e140, "unknown_14253e140", fn_14253e140 as *const ()), // sub_14253e140
    Replacement::draft(0x14253e170, "unknown_14253e170", fn_14253e170 as *const ()), // sub_14253e170
    Replacement::draft(0x14253e1b0, "unknown_14253e1b0", tpl_110617::<0x143a09580, 0x14183acc2> as *const ()), // sub_14253e1b0
    Replacement::draft(0x14253e1f0, "unknown_14253e1f0", tpl_52370 as *const ()), // sub_14253e1f0
    Replacement::draft(0x14253e220, "unknown_14253e220", tpl_52371 as *const ()), // sub_14253e220
    Replacement::draft(0x14253e240, "unknown_14253e240", fn_14253e240 as *const ()), // sub_14253e240
    Replacement::draft(0x14253e260, "unknown_14253e260", fn_14253e260 as *const ()), // sub_14253e260
    Replacement::draft(0x14253e280, "unknown_14253e280", fn_14253e280 as *const ()), // sub_14253e280
    Replacement::draft(0x14253e2d0, "unknown_14253e2d0", fn_14253e2d0 as *const ()), // sub_14253e2d0
    Replacement::draft(0x14253e300, "unknown_14253e300", fn_14253e300 as *const ()), // sub_14253e300
    Replacement::draft(0x14253e320, "unknown_14253e320", tpl_44345::<0x18> as *const ()), // sub_14253e320
    Replacement::draft(0x14253e330, "unknown_14253e330", tpl_1738::<0x30> as *const ()), // sub_14253e330
    Replacement::draft(0x14253e340, "unknown_14253e340", tpl_52376::<0x28> as *const ()), // sub_14253e340
    Replacement::draft(0x14253e350, "unknown_14253e350", tpl_52465::<0xe0> as *const ()), // sub_14253e350
    Replacement::draft(0x14253e370, "unknown_14253e370", tpl_52299::<0x1c> as *const ()), // sub_14253e370
    Replacement::draft(0x14253e390, "unknown_14253e390", tpl_52377::<0x1c> as *const ()), // sub_14253e390
    Replacement::draft(0x14253e410, "unknown_14253e410", tpl_52311::<0x4> as *const ()), // sub_14253e410
    Replacement::draft(0x14253e420, "unknown_14253e420", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_14253e420
    Replacement::draft(0x14253e440, "unknown_14253e440", tpl_361 as *const ()), // sub_14253e440
    Replacement::draft(0x14253e450, "unknown_14253e450", tpl_361 as *const ()), // sub_14253e450
    Replacement::draft(0x14253e460, "unknown_14253e460", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_14253e460
    Replacement::draft(0x14253e480, "unknown_14253e480", fn_14253e480 as *const ()), // sub_14253e480
    Replacement::draft(0x14253e4a0, "unknown_14253e4a0", fn_14253e4a0 as *const ()), // sub_14253e4a0
    Replacement::draft(0x14253e4c0, "unknown_14253e4c0", fn_14253e4c0 as *const ()), // sub_14253e4c0
    Replacement::draft(0x14253e4e0, "unknown_14253e4e0", tpl_52420::<0xffffffffffffffff> as *const ()), // sub_14253e4e0
    Replacement::draft(0x14253e510, "unknown_14253e510", tpl_369 as *const ()), // sub_14253e510
    Replacement::draft(0x14253e520, "unknown_14253e520", tpl_42543 as *const ()), // sub_14253e520
    Replacement::draft(0x14253e530, "unknown_14253e530", tpl_473 as *const ()), // sub_14253e530
    Replacement::draft(0x14253e540, "unknown_14253e540", tpl_50491::<0x143c7d0c0> as *const ()), // sub_14253e540
    Replacement::draft(0x14253e550, "unknown_14253e550", tpl_520 as *const ()), // sub_14253e550
    Replacement::draft(0x14253e570, "unknown_14253e570", tpl_520 as *const ()), // sub_14253e570
    Replacement::draft(0x14253e590, "unknown_14253e590", fn_14253e590 as *const ()), // sub_14253e590
    Replacement::draft(0x14253e5b0, "unknown_14253e5b0", tpl_52402 as *const ()), // sub_14253e5b0
    Replacement::draft(0x14253e5c0, "unknown_14253e5c0", fn_14253e5c0 as *const ()), // sub_14253e5c0
    Replacement::draft(0x14253e630, "unknown_14253e630", fn_14253e630 as *const ()), // sub_14253e630
    Replacement::draft(0x14253e6d0, "unknown_14253e6d0", fn_14253e6d0 as *const ()), // sub_14253e6d0
    Replacement::draft(0x14253e780, "unknown_14253e780", tpl_52377::<0x1c> as *const ()), // sub_14253e780
    Replacement::draft(0x14253e800, "unknown_14253e800", fn_14253e800 as *const ()), // sub_14253e800
    Replacement::draft(0x14253e890, "luau_14253e890", fn_14253e890 as *const ()), // sub_14253e890
    Replacement::draft(0x14253eaa0, "unknown_14253eaa0", tpl_52494::<0x1425462a0, 0x143a095c0, 0x142540260, 0x14183acc2, 0x14253eb50> as *const ()), // sub_14253eaa0
    Replacement::draft(0x14253eaf0, "unknown_14253eaf0", tpl_52495::<0x143a095c0, 0x142540260, 0x14183acc2> as *const ()), // sub_14253eaf0
    Replacement::draft(0x14253eb30, "unknown_14253eb30", fn_14253eb30 as *const ()), // sub_14253eb30
    Replacement::draft(0x14253eb50, "luau_14253eb50", fn_14253eb50 as *const ()), // sub_14253eb50
    Replacement::draft(0x14253fa60, "unknown_14253fa60", fn_14253fa60 as *const ()), // sub_14253fa60
    Replacement::draft(0x14253faf0, "unknown_14253faf0", fn_14253faf0 as *const ()), // sub_14253faf0
    Replacement::draft(0x14253fba0, "unknown_14253fba0", fn_14253fba0 as *const ()), // sub_14253fba0
    Replacement::draft(0x14253fc20, "unknown_14253fc20", fn_14253fc20 as *const ()), // sub_14253fc20
    Replacement::draft(0x14253fca0, "unknown_14253fca0", fn_14253fca0 as *const ()), // sub_14253fca0
    Replacement::draft(0x14253fd10, "unknown_14253fd10", fn_14253fd10 as *const ()), // sub_14253fd10
    Replacement::draft(0x14253fd80, "unknown_14253fd80", fn_14253fd80 as *const ()), // sub_14253fd80
    Replacement::draft(0x14253fdb0, "unknown_14253fdb0", fn_14253fdb0 as *const ()), // sub_14253fdb0
    Replacement::draft(0x142540030, "unknown_142540030", fn_142540030 as *const ()), // sub_142540030
    Replacement::draft(0x142540080, "unknown_142540080", fn_142540080 as *const ()), // sub_142540080
    Replacement::draft(0x1425400a0, "unknown_1425400a0", fn_1425400a0 as *const ()), // sub_1425400a0
    Replacement::draft(0x142540260, "unknown_142540260", fn_142540260 as *const ()), // sub_142540260
    Replacement::draft(0x142540890, "unknown_142540890", fn_142540890 as *const ()), // sub_142540890
    Replacement::draft(0x1425408b0, "unknown_1425408b0", fn_1425408b0 as *const ()), // sub_1425408b0
    Replacement::draft(0x1425408f0, "unknown_1425408f0", fn_1425408f0 as *const ()), // sub_1425408f0
    Replacement::draft(0x142540980, "unknown_142540980", fn_142540980 as *const ()), // sub_142540980
    Replacement::draft(0x1425409b0, "unknown_1425409b0", fn_1425409b0 as *const ()), // sub_1425409b0
    Replacement::draft(0x1425409d0, "unknown_1425409d0", tpl_52402 as *const ()), // sub_1425409d0
    Replacement::draft(0x1425409e0, "unknown_1425409e0", fn_1425409e0 as *const ()), // sub_1425409e0
    Replacement::draft(0x142540a00, "unknown_142540a00", tpl_52402 as *const ()), // sub_142540a00
    Replacement::draft(0x142540a10, "unknown_142540a10", tpl_473 as *const ()), // sub_142540a10
    Replacement::draft(0x142540a20, "unknown_142540a20", tpl_366::<0x143d01c60> as *const ()), // sub_142540a20
    Replacement::draft(0x142540a30, "unknown_142540a30", tpl_52402 as *const ()), // sub_142540a30
    Replacement::draft(0x142540a40, "unknown_142540a40", tpl_365::<0x1425412f0> as *const ()), // sub_142540a40
    Replacement::draft(0x142540a50, "unknown_142540a50", fn_142540a50 as *const ()), // sub_142540a50
    Replacement::draft(0x142540aa0, "unknown_142540aa0", fn_142540aa0 as *const ()), // sub_142540aa0
    Replacement::draft(0x142540ad0, "unknown_142540ad0", fn_142540ad0 as *const ()), // sub_142540ad0
    Replacement::draft(0x142540b00, "unknown_142540b00", tpl_52495::<0x143a095c0, 0x142540260, 0x14183acc2> as *const ()), // sub_142540b00
    Replacement::draft(0x142540b40, "unknown_142540b40", tpl_365::<0x1425408f0> as *const ()), // sub_142540b40
    Replacement::draft(0x142540b50, "unknown_142540b50", fn_142540b50 as *const ()), // sub_142540b50
    Replacement::draft(0x142540b60, "unknown_142540b60", tpl_52494::<0x1425462a0, 0x143a095c0, 0x142540260, 0x14183acc2, 0x14253eb50> as *const ()), // sub_142540b60
    Replacement::draft(0x142540bb0, "unknown_142540bb0", fn_142540bb0 as *const ()), // sub_142540bb0
    Replacement::draft(0x142540c40, "unknown_142540c40", fn_142540c40 as *const ()), // sub_142540c40
    Replacement::draft(0x142540c70, "unknown_142540c70", fn_142540c70 as *const ()), // sub_142540c70
    Replacement::draft(0x142540cf0, "luau_142540cf0", fn_142540cf0 as *const ()), // sub_142540cf0
    Replacement::draft(0x142540d20, "unknown_142540d20", fn_142540d20 as *const ()), // sub_142540d20
    Replacement::draft(0x142540d90, "unknown_142540d90", fn_142540d90 as *const ()), // sub_142540d90
    Replacement::draft(0x142540f20, "unknown_142540f20", fn_142540f20 as *const ()), // sub_142540f20
    Replacement::draft(0x142541030, "unknown_142541030", fn_142541030 as *const ()), // sub_142541030
    Replacement::draft(0x142541180, "unknown_142541180", fn_142541180 as *const ()), // sub_142541180
    Replacement::draft(0x1425412a0, "unknown_1425412a0", fn_1425412a0 as *const ()), // sub_1425412a0
    Replacement::draft(0x1425412c0, "unknown_1425412c0", tpl_366::<0x143c0e188> as *const ()), // sub_1425412c0
    Replacement::draft(0x1425412d0, "unknown_1425412d0", fn_1425412d0 as *const ()), // sub_1425412d0
    Replacement::draft(0x142541370, "luau_142541370", fn_142541370 as *const ()), // sub_142541370
    Replacement::draft(0x1425414c0, "unknown_1425414c0", fn_1425414c0 as *const ()), // sub_1425414c0
    Replacement::draft(0x142541580, "unknown_142541580", fn_142541580 as *const ()), // sub_142541580
    Replacement::draft(0x142541610, "unknown_142541610", fn_142541610 as *const ()), // sub_142541610
    Replacement::draft(0x1425417c0, "unknown_1425417c0", fn_1425417c0 as *const ()), // sub_1425417c0
    Replacement::draft(0x142541810, "unknown_142541810", fn_142541810 as *const ()), // sub_142541810
    Replacement::draft(0x142541850, "unknown_142541850", fn_142541850 as *const ()), // sub_142541850
    Replacement::draft(0x142541890, "unknown_142541890", fn_142541890 as *const ()), // sub_142541890
    Replacement::draft(0x142541920, "std_142541920", tpl_52429::<0x143872f48, 0x143872f4a, 0x142645720, 0x143872f30, 0x143872f40, 0x143425fc8, 0x142645308, 0x143426040, 0x142645758, 0x143bc5fd8, 0x143426058, 0x142645750, 0x143426068, 0x143426078, 0x143bfe128, 0x143425fd8, 0x143425fe8, 0x143426008, 0x143bce008, 0x143426020, 0x143be6098, 0x1434260e0, 0x143bee0c8, 0x143bde068, 0x143872cb4, 0x143872cb0, 0x1425375a0, 0x143872cd4, 0x143872cd0, 0x143872f38, 0x142541180> as *const ()), // sub_142541920
    Replacement::draft(0x142541ac0, "unknown_142541ac0", fn_142541ac0 as *const ()), // sub_142541ac0
    Replacement::draft(0x142541b00, "unknown_142541b00", fn_142541b00 as *const ()), // sub_142541b00
    Replacement::draft(0x142541c40, "unknown_142541c40", fn_142541c40 as *const ()), // sub_142541c40
    Replacement::draft(0x142541c70, "unknown_142541c70", fn_142541c70 as *const ()), // sub_142541c70
    Replacement::draft(0x142541d70, "unknown_142541d70", fn_142541d70 as *const ()), // sub_142541d70
    Replacement::draft(0x142541da0, "unknown_142541da0", fn_142541da0 as *const ()), // sub_142541da0
    Replacement::draft(0x142541dc0, "unknown_142541dc0", fn_142541dc0 as *const ()), // sub_142541dc0
    Replacement::draft(0x142541dd0, "unknown_142541dd0", fn_142541dd0 as *const ()), // sub_142541dd0
    Replacement::draft(0x1425421a0, "unknown_1425421a0", fn_1425421a0 as *const ()), // sub_1425421a0
    Replacement::draft(0x1425422f0, "unknown_1425422f0", fn_1425422f0 as *const ()), // sub_1425422f0
    Replacement::draft(0x142542620, "unknown_142542620", fn_142542620 as *const ()), // sub_142542620
    Replacement::draft(0x142542670, "unknown_142542670", fn_142542670 as *const ()), // sub_142542670
    Replacement::draft(0x142542690, "unknown_142542690", fn_142542690 as *const ()), // sub_142542690
    Replacement::draft(0x1425427b0, "unknown_1425427b0", fn_1425427b0 as *const ()), // sub_1425427b0
    Replacement::draft(0x1425428d0, "unknown_1425428d0", fn_1425428d0 as *const ()), // sub_1425428d0
    Replacement::draft(0x142542b70, "unknown_142542b70", fn_142542b70 as *const ()), // sub_142542b70
    Replacement::draft(0x142542cd0, "unknown_142542cd0", fn_142542cd0 as *const ()), // sub_142542cd0
    Replacement::draft(0x142542dc0, "unknown_142542dc0", fn_142542dc0 as *const ()), // sub_142542dc0
    Replacement::draft(0x142542e80, "unknown_142542e80", fn_142542e80 as *const ()), // sub_142542e80
    Replacement::draft(0x142542ed0, "unknown_142542ed0", fn_142542ed0 as *const ()), // sub_142542ed0
    Replacement::draft(0x142543010, "unknown_142543010", fn_142543010 as *const ()), // sub_142543010
    Replacement::draft(0x142543210, "unknown_142543210", fn_142543210 as *const ()), // sub_142543210
    Replacement::draft(0x1425432a0, "unknown_1425432a0", fn_1425432a0 as *const ()), // sub_1425432a0
    Replacement::draft(0x142543460, "unknown_142543460", fn_142543460 as *const ()), // sub_142543460
    Replacement::draft(0x1425434c0, "unknown_1425434c0", fn_1425434c0 as *const ()), // sub_1425434c0
    Replacement::draft(0x1425434e0, "unknown_1425434e0", fn_1425434e0 as *const ()), // sub_1425434e0
    Replacement::draft(0x142543530, "unknown_142543530", fn_142543530 as *const ()), // sub_142543530
    Replacement::draft(0x142543660, "unknown_142543660", fn_142543660 as *const ()), // sub_142543660
    Replacement::draft(0x142543780, "unknown_142543780", fn_142543780 as *const ()), // sub_142543780
    Replacement::draft(0x1425437c0, "unknown_1425437c0", fn_1425437c0 as *const ()), // sub_1425437c0
    Replacement::draft(0x142543800, "unknown_142543800", fn_142543800 as *const ()), // sub_142543800
    Replacement::draft(0x142543810, "unknown_142543810", fn_142543810 as *const ()), // sub_142543810
    Replacement::draft(0x142543820, "unknown_142543820", fn_142543820 as *const ()), // sub_142543820
    Replacement::draft(0x142543840, "unknown_142543840", fn_142543840 as *const ()), // sub_142543840
    Replacement::draft(0x142543860, "unknown_142543860", tpl_559 as *const ()), // sub_142543860
    Replacement::draft(0x142543870, "unknown_142543870", fn_142543870 as *const ()), // sub_142543870
    Replacement::draft(0x142543880, "unknown_142543880", fn_142543880 as *const ()), // sub_142543880
    Replacement::draft(0x1425438a0, "unknown_1425438a0", tpl_361 as *const ()), // sub_1425438a0
    Replacement::draft(0x1425438b0, "unknown_1425438b0", tpl_52561 as *const ()), // sub_1425438b0
    Replacement::draft(0x1425438c0, "unknown_1425438c0", tpl_52562 as *const ()), // sub_1425438c0
    Replacement::draft(0x1425438d0, "unknown_1425438d0", tpl_110617::<0x143a09580, 0x14183acc2> as *const ()), // sub_1425438d0
    Replacement::draft(0x142543910, "unknown_142543910", fn_142543910 as *const ()), // sub_142543910
    Replacement::draft(0x142543930, "unknown_142543930", fn_142543930 as *const ()), // sub_142543930
    Replacement::draft(0x142543a00, "unknown_142543a00", tpl_52424::<0x10> as *const ()), // sub_142543a00
    Replacement::draft(0x142543a30, "unknown_142543a30", tpl_52425::<0x10, 0x14> as *const ()), // sub_142543a30
    Replacement::draft(0x142543a50, "unknown_142543a50", fn_142543a50 as *const ()), // sub_142543a50
    Replacement::draft(0x142543a70, "unknown_142543a70", tpl_52420::<0xffffffffffffffff> as *const ()), // sub_142543a70
    Replacement::draft(0x142543aa0, "unknown_142543aa0", tpl_52426::<0xfffffffffffffffd> as *const ()), // sub_142543aa0
    Replacement::draft(0x142543ab0, "unknown_142543ab0", tpl_52402 as *const ()), // sub_142543ab0
    Replacement::draft(0x142543ac0, "unknown_142543ac0", tpl_520 as *const ()), // sub_142543ac0
    Replacement::draft(0x142543ae0, "unknown_142543ae0", tpl_46299 as *const ()), // sub_142543ae0
    Replacement::draft(0x142543af0, "unknown_142543af0", tpl_52402 as *const ()), // sub_142543af0
    Replacement::draft(0x142543b00, "unknown_142543b00", tpl_42543 as *const ()), // sub_142543b00
    Replacement::draft(0x142543b10, "unknown_142543b10", tpl_473 as *const ()), // sub_142543b10
    Replacement::draft(0x142543b20, "unknown_142543b20", tpl_42543 as *const ()), // sub_142543b20
    Replacement::draft(0x142543b30, "unknown_142543b30", tpl_473 as *const ()), // sub_142543b30
    Replacement::draft(0x142543b40, "unknown_142543b40", tpl_520 as *const ()), // sub_142543b40
    Replacement::draft(0x142543b60, "unknown_142543b60", tpl_52333::<0x143c7f580> as *const ()), // sub_142543b60
    Replacement::draft(0x142543b80, "unknown_142543b80", tpl_52402 as *const ()), // sub_142543b80
    Replacement::draft(0x142543b90, "unknown_142543b90", fn_142543b90 as *const ()), // sub_142543b90
    Replacement::draft(0x142543fc0, "unknown_142543fc0", fn_142543fc0 as *const ()), // sub_142543fc0
    Replacement::draft(0x142544280, "unknown_142544280", fn_142544280 as *const ()), // sub_142544280
    Replacement::draft(0x1425442b0, "unknown_1425442b0", fn_1425442b0 as *const ()), // sub_1425442b0
    Replacement::draft(0x1425442e0, "unknown_1425442e0", tpl_3819::<0x142541dd0> as *const ()), // sub_1425442e0
    Replacement::draft(0x142544300, "unknown_142544300", fn_142544300 as *const ()), // sub_142544300
    Replacement::draft(0x142544430, "unknown_142544430", fn_142544430 as *const ()), // sub_142544430
    Replacement::draft(0x142544510, "luau_142544510", fn_142544510 as *const ()), // sub_142544510
    Replacement::draft(0x1425448b0, "luau_1425448b0", fn_1425448b0 as *const ()), // sub_1425448b0
    Replacement::draft(0x142544940, "unknown_142544940", fn_142544940 as *const ()), // sub_142544940
    Replacement::draft(0x1425449e0, "unknown_1425449e0", fn_1425449e0 as *const ()), // sub_1425449e0
    Replacement::draft(0x142544b80, "std_142544b80", fn_142544b80 as *const ()), // sub_142544b80
    Replacement::draft(0x142544bb0, "unknown_142544bb0", fn_142544bb0 as *const ()), // sub_142544bb0
    Replacement::draft(0x142544c00, "unknown_142544c00", fn_142544c00 as *const ()), // sub_142544c00
    Replacement::draft(0x142545060, "unknown_142545060", fn_142545060 as *const ()), // sub_142545060
    Replacement::draft(0x1425450e0, "unknown_1425450e0", fn_1425450e0 as *const ()), // sub_1425450e0
    Replacement::draft(0x1425450f0, "luau_1425450f0", tpl_52372 as *const ()), // sub_1425450f0
    Replacement::draft(0x142545140, "unknown_142545140", tpl_28389::<0x142544510> as *const ()), // sub_142545140
    Replacement::draft(0x142545150, "unknown_142545150", tpl_22435::<0x142544510> as *const ()), // sub_142545150
    Replacement::draft(0x142545160, "unknown_142545160", fn_142545160 as *const ()), // sub_142545160
    Replacement::draft(0x142545180, "unknown_142545180", fn_142545180 as *const ()), // sub_142545180
    Replacement::draft(0x1425451f0, "unknown_1425451f0", fn_1425451f0 as *const ()), // sub_1425451f0
    Replacement::draft(0x1425452f0, "unknown_1425452f0", fn_1425452f0 as *const ()), // sub_1425452f0
    Replacement::draft(0x142545510, "unknown_142545510", fn_142545510 as *const ()), // sub_142545510
    Replacement::draft(0x1425455d0, "unknown_1425455d0", fn_1425455d0 as *const ()), // sub_1425455d0
    Replacement::draft(0x142545730, "unknown_142545730", fn_142545730 as *const ()), // sub_142545730
    Replacement::draft(0x1425457f0, "unknown_1425457f0", fn_1425457f0 as *const ()), // sub_1425457f0
    Replacement::draft(0x142545950, "unknown_142545950", fn_142545950 as *const ()), // sub_142545950
    Replacement::draft(0x142545a20, "unknown_142545a20", fn_142545a20 as *const ()), // sub_142545a20
    Replacement::draft(0x142545ac0, "unknown_142545ac0", fn_142545ac0 as *const ()), // sub_142545ac0
    Replacement::draft(0x142545b40, "unknown_142545b40", fn_142545b40 as *const ()), // sub_142545b40
    Replacement::draft(0x142545b60, "unknown_142545b60", fn_142545b60 as *const ()), // sub_142545b60
    Replacement::draft(0x142545b80, "unknown_142545b80", fn_142545b80 as *const ()), // sub_142545b80
    Replacement::draft(0x142545e40, "unknown_142545e40", fn_142545e40 as *const ()), // sub_142545e40
    Replacement::draft(0x142545ea0, "unknown_142545ea0", fn_142545ea0 as *const ()), // sub_142545ea0
    Replacement::draft(0x142545f20, "unknown_142545f20", fn_142545f20 as *const ()), // sub_142545f20
    Replacement::draft(0x142545fd0, "unknown_142545fd0", tpl_52329 as *const ()), // sub_142545fd0
    Replacement::draft(0x142545ff0, "unknown_142545ff0", tpl_110899::<0x143a09580, 0x14183acbc> as *const ()), // sub_142545ff0
    Replacement::draft(0x142546040, "unknown_142546040", tpl_110899::<0x143a09580, 0x14183acbc> as *const ()), // sub_142546040
    Replacement::draft(0x142546090, "unknown_142546090", tpl_52303 as *const ()), // sub_142546090
    Replacement::draft(0x1425460a0, "unknown_1425460a0", tpl_52303 as *const ()), // sub_1425460a0
    Replacement::draft(0x1425460b0, "unknown_1425460b0", tpl_44345::<0x18> as *const ()), // sub_1425460b0
    Replacement::draft(0x1425460c0, "unknown_1425460c0", tpl_913::<0x30> as *const ()), // sub_1425460c0
    Replacement::draft(0x1425460d0, "unknown_1425460d0", tpl_52299::<0x1c> as *const ()), // sub_1425460d0
    Replacement::draft(0x1425460f0, "unknown_1425460f0", tpl_52299::<0x1c> as *const ()), // sub_1425460f0
    Replacement::draft(0x142546110, "unknown_142546110", tpl_52377::<0x1c> as *const ()), // sub_142546110
    Replacement::draft(0x142546190, "unknown_142546190", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_142546190
    Replacement::draft(0x1425461b0, "unknown_1425461b0", tpl_52294::<0xffffffffffffffff> as *const ()), // sub_1425461b0
    Replacement::draft(0x1425461d0, "unknown_1425461d0", tpl_52330::<0x143073d80> as *const ()), // sub_1425461d0
    Replacement::draft(0x1425461f0, "unknown_1425461f0", fn_1425461f0 as *const ()), // sub_1425461f0
    Replacement::draft(0x142546210, "unknown_142546210", tpl_473 as *const ()), // sub_142546210
    Replacement::draft(0x142546220, "unknown_142546220", fn_142546220 as *const ()), // sub_142546220
    Replacement::draft(0x142546290, "unknown_142546290", tpl_29791::<0x142545b80> as *const ()), // sub_142546290
    Replacement::draft(0x1425462a0, "unknown_1425462a0", fn_1425462a0 as *const ()), // sub_1425462a0
    Replacement::draft(0x142546310, "unknown_142546310", fn_142546310 as *const ()), // sub_142546310
    Replacement::draft(0x142546510, "unknown_142546510", fn_142546510 as *const ()), // sub_142546510
    Replacement::draft(0x1425466b0, "unknown_1425466b0", fn_1425466b0 as *const ()), // sub_1425466b0
    Replacement::draft(0x1425466e0, "unknown_1425466e0", fn_1425466e0 as *const ()), // sub_1425466e0
    Replacement::draft(0x142546780, "unknown_142546780", fn_142546780 as *const ()), // sub_142546780
    Replacement::draft(0x142546890, "unknown_142546890", fn_142546890 as *const ()), // sub_142546890
    Replacement::draft(0x142546a20, "unknown_142546a20", fn_142546a20 as *const ()), // sub_142546a20
    Replacement::draft(0x142546af0, "unknown_142546af0", fn_142546af0 as *const ()), // sub_142546af0
    Replacement::draft(0x142546d00, "unknown_142546d00", fn_142546d00 as *const ()), // sub_142546d00
    Replacement::draft(0x142546e30, "unknown_142546e30", fn_142546e30 as *const ()), // sub_142546e30
    Replacement::draft(0x142546e60, "unknown_142546e60", fn_142546e60 as *const ()), // sub_142546e60
    Replacement::draft(0x142546e70, "unknown_142546e70", tpl_52561 as *const ()), // sub_142546e70
    Replacement::draft(0x142546e80, "unknown_142546e80", tpl_52562 as *const ()), // sub_142546e80
    Replacement::draft(0x142546e90, "unknown_142546e90", fn_142546e90 as *const ()), // sub_142546e90
    Replacement::draft(0x142546eb0, "unknown_142546eb0", tpl_52371 as *const ()), // sub_142546eb0
    Replacement::draft(0x142546ed0, "unknown_142546ed0", fn_142546ed0 as *const ()), // sub_142546ed0
    Replacement::draft(0x142546ee0, "unknown_142546ee0", tpl_20791 as *const ()), // sub_142546ee0
    Replacement::draft(0x142546ef0, "unknown_142546ef0", tpl_473 as *const ()), // sub_142546ef0
    Replacement::draft(0x142546f00, "unknown_142546f00", tpl_520 as *const ()), // sub_142546f00
    Replacement::draft(0x142546f20, "unknown_142546f20", fn_142546f20 as *const ()), // sub_142546f20
    Replacement::draft(0x142546f50, "unknown_142546f50", fn_142546f50 as *const ()), // sub_142546f50
    Replacement::draft(0x142546f80, "unknown_142546f80", fn_142546f80 as *const ()), // sub_142546f80
    Replacement::draft(0x142547120, "unknown_142547120", fn_142547120 as *const ()), // sub_142547120
    Replacement::draft(0x142547190, "unknown_142547190", fn_142547190 as *const ()), // sub_142547190
    Replacement::draft(0x1425471f0, "unknown_1425471f0", fn_1425471f0 as *const ()), // sub_1425471f0
    Replacement::draft(0x142547250, "unknown_142547250", fn_142547250 as *const ()), // sub_142547250
    Replacement::draft(0x1425474d0, "unknown_1425474d0", fn_1425474d0 as *const ()), // sub_1425474d0
    Replacement::draft(0x142547680, "unknown_142547680", tpl_3819::<0x142546890> as *const ()), // sub_142547680
    Replacement::draft(0x14254768e, "crt_14254768e", tpl_906::<0x1426462d8> as *const ()), // j___RTDynamicCast
    Replacement::draft(0x142547694, "crt_142547694", tpl_906::<0x142646680> as *const ()), // j__dclass
    Replacement::draft(0x14254769a, "crt_14254769a", tpl_906::<0x142646688> as *const ()), // j__fdclass
    Replacement::draft(0x1425476a0, "crt_1425476a0", tpl_906::<0x142646690> as *const ()), // j__hypotf
    Replacement::draft(0x1425476a6, "crt_1425476a6", tpl_906::<0x142646698> as *const ()), // j__ldclass
    Replacement::draft(0x1425476ac, "crt_1425476ac", tpl_906::<0x1426466a0> as *const ()), // j_acos
    Replacement::draft(0x1425476b2, "crt_1425476b2", tpl_906::<0x1426466d8> as *const ()), // j_asin
    Replacement::draft(0x1425476b8, "crt_1425476b8", tpl_906::<0x1426466e0> as *const ()), // j_atan
    Replacement::draft(0x1425476be, "crt_1425476be", tpl_906::<0x1426466e8> as *const ()), // j_cosh
    Replacement::draft(0x1425476c4, "crt_1425476c4", tpl_906::<0x1426466f0> as *const ()), // j_fmod
    Replacement::draft(0x1425476ca, "crt_1425476ca", tpl_906::<0x1426466f8> as *const ()), // j_log2
    Replacement::draft(0x1425476d0, "crt_1425476d0", tpl_906::<0x142646700> as *const ()), // j_log2f
    Replacement::draft(0x1425476d6, "crt_1425476d6", tpl_906::<0x142646708> as *const ()), // j_sinh
    Replacement::draft(0x1425476dc, "crt_1425476dc", tpl_906::<0x142646710> as *const ()), // j_sqrt
    Replacement::draft(0x1425476e2, "crt_1425476e2", tpl_906::<0x142646718> as *const ()), // j_tanh
    Replacement::draft(0x142547700, "global_142547700", fn_142547700 as *const ()), // _guard_dispatch_icall
    Replacement::draft(0x142547720, "global_142547720", tpl_906::<0x142647390> as *const ()), // _guard_dispatch_icall
    Replacement::draft(0x142547730, "unknown_142547730", tpl_327::<0x143a09848, 0x14240ab10> as *const ()), // sub_142547730
    Replacement::draft(0x142547740, "unknown_142547740", tpl_327::<0x143a09890, 0x14240ab10> as *const ()), // sub_142547740
    Replacement::draft(0x142547750, "unknown_142547750", tpl_327::<0x1438431d8, 0x1401de420> as *const ()), // sub_142547750
    Replacement::draft(0x142547760, "unknown_142547760", tpl_327::<0x1438431a8, 0x1401de420> as *const ()), // sub_142547760
    Replacement::draft(0x142547770, "unknown_142547770", tpl_327::<0x143843178, 0x1401de420> as *const ()), // sub_142547770
    Replacement::draft(0x142547780, "unknown_142547780", tpl_52607::<0x40, 0x14240ab10> as *const ()), // sub_142547780
    Replacement::draft(0x142547790, "unknown_142547790", tpl_327::<0x143a09b18, 0x14240ab10> as *const ()), // sub_142547790
    Replacement::draft(0x1425477a0, "unknown_1425477a0", tpl_327::<0x143a09b60, 0x14240ab10> as *const ()), // sub_1425477a0
    Replacement::draft(0x1425477b0, "unknown_1425477b0", tpl_327::<0x143843268, 0x1401de420> as *const ()), // sub_1425477b0
    Replacement::draft(0x1425477c0, "unknown_1425477c0", tpl_327::<0x143843238, 0x1401de420> as *const ()), // sub_1425477c0
    Replacement::draft(0x1425477d0, "unknown_1425477d0", tpl_327::<0x143843208, 0x1401de420> as *const ()), // sub_1425477d0
    Replacement::draft(0x1425477e0, "unknown_1425477e0", tpl_327::<0x143a09ce0, 0x14240ab10> as *const ()), // sub_1425477e0
    Replacement::draft(0x1425477f0, "unknown_1425477f0", tpl_327::<0x143a0a3f8, 0x14240ab10> as *const ()), // sub_1425477f0
    Replacement::draft(0x142547800, "unknown_142547800", tpl_327::<0x143a0a0e0, 0x14240ab10> as *const ()), // sub_142547800
    Replacement::draft(0x142547810, "unknown_142547810", tpl_327::<0x143a0a380, 0x14240ab10> as *const ()), // sub_142547810
    Replacement::draft(0x142547820, "unknown_142547820", tpl_327::<0x143a09f10, 0x14240ab10> as *const ()), // sub_142547820
    Replacement::draft(0x142547830, "unknown_142547830", tpl_327::<0x143a09da8, 0x14240ab10> as *const ()), // sub_142547830
    Replacement::draft(0x142547840, "unknown_142547840", tpl_327::<0x143a0a420, 0x14240ab10> as *const ()), // sub_142547840
    Replacement::draft(0x142547850, "unknown_142547850", tpl_327::<0x143a09c90, 0x14240ab10> as *const ()), // sub_142547850
    Replacement::draft(0x142547860, "unknown_142547860", tpl_327::<0x143a0a550, 0x14240ab10> as *const ()), // sub_142547860
    Replacement::draft(0x142547870, "unknown_142547870", tpl_327::<0x143a09e70, 0x14240ab10> as *const ()), // sub_142547870
    Replacement::draft(0x142547880, "unknown_142547880", tpl_327::<0x143a0a470, 0x14240ab10> as *const ()), // sub_142547880
    Replacement::draft(0x142547890, "unknown_142547890", tpl_327::<0x143a0a4d8, 0x14240ab10> as *const ()), // sub_142547890
    Replacement::draft(0x1425478a0, "unknown_1425478a0", tpl_327::<0x143a09cb8, 0x14240ab10> as *const ()), // sub_1425478a0
    Replacement::draft(0x1425478b0, "unknown_1425478b0", tpl_327::<0x143a09dd0, 0x14240ab10> as *const ()), // sub_1425478b0
    Replacement::draft(0x1425478c0, "unknown_1425478c0", tpl_327::<0x143a09f50, 0x14240ab10> as *const ()), // sub_1425478c0
    Replacement::draft(0x1425478d0, "unknown_1425478d0", tpl_327::<0x143a09928, 0x14240ab10> as *const ()), // sub_1425478d0
    Replacement::draft(0x1425478e0, "unknown_1425478e0", tpl_327::<0x143a09ec0, 0x14240ab10> as *const ()), // sub_1425478e0
    Replacement::draft(0x1425478f0, "unknown_1425478f0", tpl_327::<0x143a09d80, 0x14240ab10> as *const ()), // sub_1425478f0
    Replacement::draft(0x142547900, "unknown_142547900", tpl_327::<0x143a0a330, 0x14240ab10> as *const ()), // sub_142547900
    Replacement::draft(0x142547910, "unknown_142547910", tpl_327::<0x143a09950, 0x14240ab10> as *const ()), // sub_142547910
    Replacement::draft(0x142547920, "unknown_142547920", tpl_327::<0x143a0a1a8, 0x14240ab10> as *const ()), // sub_142547920
    Replacement::draft(0x142547930, "unknown_142547930", tpl_327::<0x143a09bf0, 0x14240ab10> as *const ()), // sub_142547930
    Replacement::draft(0x142547940, "unknown_142547940", tpl_327::<0x143a0a248, 0x14240ab10> as *const ()), // sub_142547940
    Replacement::draft(0x142547950, "unknown_142547950", tpl_327::<0x143a0a448, 0x14240ab10> as *const ()), // sub_142547950
    Replacement::draft(0x142547960, "unknown_142547960", tpl_327::<0x143a0a0b8, 0x14240ab10> as *const ()), // sub_142547960
    Replacement::draft(0x142547970, "unknown_142547970", tpl_327::<0x143a0a3d0, 0x14240ab10> as *const ()), // sub_142547970
    Replacement::draft(0x142547980, "unknown_142547980", tpl_327::<0x143a09900, 0x14240ab10> as *const ()), // sub_142547980
    Replacement::draft(0x142547990, "unknown_142547990", tpl_327::<0x143a0a2c0, 0x14240ab10> as *const ()), // sub_142547990
    Replacement::draft(0x1425479a0, "unknown_1425479a0", tpl_327::<0x143a09a90, 0x14240ab10> as *const ()), // sub_1425479a0
    Replacement::draft(0x1425479b0, "unknown_1425479b0", tpl_327::<0x143a0a108, 0x14240ab10> as *const ()), // sub_1425479b0
    Replacement::draft(0x1425479c0, "unknown_1425479c0", tpl_327::<0x143a09bc8, 0x14240ab10> as *const ()), // sub_1425479c0
    Replacement::draft(0x1425479d0, "unknown_1425479d0", tpl_327::<0x143a09a18, 0x14240ab10> as *const ()), // sub_1425479d0
    Replacement::draft(0x1425479e0, "unknown_1425479e0", tpl_327::<0x143a09a68, 0x14240ab10> as *const ()), // sub_1425479e0
    Replacement::draft(0x1425479f0, "unknown_1425479f0", tpl_327::<0x143a0a498, 0x14240ab10> as *const ()), // sub_1425479f0
    Replacement::draft(0x142547a00, "unknown_142547a00", tpl_327::<0x143a0a3a8, 0x14240ab10> as *const ()), // sub_142547a00
    Replacement::draft(0x142547a10, "unknown_142547a10", tpl_327::<0x143a09e20, 0x14240ab10> as *const ()), // sub_142547a10
    Replacement::draft(0x142547a20, "unknown_142547a20", tpl_327::<0x143a09fd0, 0x14240ab10> as *const ()), // sub_142547a20
    Replacement::draft(0x142547a30, "unknown_142547a30", tpl_327::<0x143a0a500, 0x14240ab10> as *const ()), // sub_142547a30
    Replacement::draft(0x142547a40, "unknown_142547a40", tpl_327::<0x143a09a40, 0x14240ab10> as *const ()), // sub_142547a40
    Replacement::draft(0x142547a50, "unknown_142547a50", tpl_327::<0x143a09df8, 0x14240ab10> as *const ()), // sub_142547a50
    Replacement::draft(0x142547a60, "unknown_142547a60", tpl_327::<0x143a0a130, 0x14240ab10> as *const ()), // sub_142547a60
    Replacement::draft(0x142547a70, "unknown_142547a70", tpl_327::<0x143a09d08, 0x14240ab10> as *const ()), // sub_142547a70
    Replacement::draft(0x142547a80, "unknown_142547a80", tpl_327::<0x143a0a270, 0x14240ab10> as *const ()), // sub_142547a80
    Replacement::draft(0x142547a90, "unknown_142547a90", tpl_327::<0x143a0a1d0, 0x14240ab10> as *const ()), // sub_142547a90
    Replacement::draft(0x142547aa0, "unknown_142547aa0", tpl_327::<0x143a09c40, 0x14240ab10> as *const ()), // sub_142547aa0
    Replacement::draft(0x142547ab0, "unknown_142547ab0", tpl_327::<0x143a09ee8, 0x14240ab10> as *const ()), // sub_142547ab0
    Replacement::draft(0x142547ac0, "unknown_142547ac0", tpl_327::<0x143a0a038, 0x14240ab10> as *const ()), // sub_142547ac0
    Replacement::draft(0x142547ad0, "unknown_142547ad0", tpl_327::<0x143a09c18, 0x14240ab10> as *const ()), // sub_142547ad0
    Replacement::draft(0x142547ae0, "unknown_142547ae0", tpl_327::<0x143a09c68, 0x14240ab10> as *const ()), // sub_142547ae0
    Replacement::draft(0x142547af0, "unknown_142547af0", tpl_327::<0x143a09e48, 0x14240ab10> as *const ()), // sub_142547af0
    Replacement::draft(0x142547b00, "unknown_142547b00", tpl_327::<0x143a0a1f8, 0x14240ab10> as *const ()), // sub_142547b00
    Replacement::draft(0x142547b10, "unknown_142547b10", tpl_327::<0x143a0a298, 0x14240ab10> as *const ()), // sub_142547b10
    Replacement::draft(0x142547b20, "unknown_142547b20", tpl_327::<0x143a0a358, 0x14240ab10> as *const ()), // sub_142547b20
    Replacement::draft(0x142547b30, "unknown_142547b30", tpl_327::<0x143a099f0, 0x14240ab10> as *const ()), // sub_142547b30
    Replacement::draft(0x142547b40, "unknown_142547b40", tpl_327::<0x143a09b88, 0x14240ab10> as *const ()), // sub_142547b40
    Replacement::draft(0x142547b50, "unknown_142547b50", tpl_327::<0x143a099c8, 0x14240ab10> as *const ()), // sub_142547b50
    Replacement::draft(0x142547b60, "unknown_142547b60", tpl_327::<0x143a0a220, 0x14240ab10> as *const ()), // sub_142547b60
    Replacement::draft(0x142547b70, "unknown_142547b70", tpl_327::<0x143a0a090, 0x14240ab10> as *const ()), // sub_142547b70
    Replacement::draft(0x142547b80, "unknown_142547b80", tpl_327::<0x143a09d30, 0x14240ab10> as *const ()), // sub_142547b80
    Replacement::draft(0x142547b90, "unknown_142547b90", tpl_327::<0x143a09978, 0x14240ab10> as *const ()), // sub_142547b90
    Replacement::draft(0x142547ba0, "unknown_142547ba0", tpl_327::<0x143a0a180, 0x14240ab10> as *const ()), // sub_142547ba0
    Replacement::draft(0x142547bb0, "unknown_142547bb0", tpl_327::<0x143a0a528, 0x14240ab10> as *const ()), // sub_142547bb0
    Replacement::draft(0x142547bc0, "unknown_142547bc0", tpl_327::<0x143a0a2e8, 0x14240ab10> as *const ()), // sub_142547bc0
    Replacement::draft(0x142547bd0, "unknown_142547bd0", tpl_327::<0x143a0a010, 0x14240ab10> as *const ()), // sub_142547bd0
    Replacement::draft(0x142547be0, "unknown_142547be0", tpl_327::<0x143a099a0, 0x14240ab10> as *const ()), // sub_142547be0
    Replacement::draft(0x142547bf0, "unknown_142547bf0", tpl_52607::<0x30, 0x14240ab10> as *const ()), // sub_142547bf0
    Replacement::draft(0x142547c00, "unknown_142547c00", tpl_327::<0x143a0a578, 0x14240ab10> as *const ()), // sub_142547c00
    Replacement::draft(0x142547c10, "unknown_142547c10", tpl_327::<0x143a0a5c0, 0x14240ab10> as *const ()), // sub_142547c10
    Replacement::draft(0x142547c20, "unknown_142547c20", tpl_327::<0x1438432f8, 0x1401de420> as *const ()), // sub_142547c20
    Replacement::draft(0x142547c30, "unknown_142547c30", tpl_327::<0x1438432c8, 0x1401de420> as *const ()), // sub_142547c30
    Replacement::draft(0x142547c40, "unknown_142547c40", tpl_327::<0x143843298, 0x1401de420> as *const ()), // sub_142547c40
    Replacement::draft(0x142547c50, "unknown_142547c50", tpl_52607::<0x70, 0x1401de3c0> as *const ()), // sub_142547c50
    Replacement::draft(0x142547c60, "unknown_142547c60", fn_142547c60 as *const ()), // sub_142547c60
    Replacement::draft(0x142547c90, "unknown_142547c90", tpl_52609::<0x130, 0x1401de3c0> as *const ()), // sub_142547c90
    Replacement::draft(0x142547ca0, "unknown_142547ca0", tpl_52609::<0x110, 0x1401de3c0> as *const ()), // sub_142547ca0
    Replacement::draft_throws(0x142547cb0, "unknown_142547cb0", fn_142547cb0 as *const (), Throws::Discard), // sub_142547cb0
    Replacement::draft_throws(0x142547cd0, "unknown_142547cd0", fn_142547cd0 as *const (), Throws::Discard), // sub_142547cd0
    Replacement::draft_throws(0x142547cf0, "unknown_142547cf0", fn_142547cf0 as *const (), Throws::Discard), // sub_142547cf0
    Replacement::draft(0x142547d20, "unknown_142547d20", tpl_52607::<0x78, 0x1401de3c0> as *const ()), // sub_142547d20
    Replacement::draft(0x142547d30, "unknown_142547d30", tpl_52607::<0x38, 0x14240ab10> as *const ()), // sub_142547d30
    Replacement::draft_throws(0x142547d40, "unknown_142547d40", fn_142547d40 as *const (), Throws::Discard), // sub_142547d40
    Replacement::draft_throws(0x142547d60, "unknown_142547d60", fn_142547d60 as *const (), Throws::Discard), // sub_142547d60
    Replacement::draft(0x142547d80, "unknown_142547d80", tpl_52609::<0x40, 0x1401e3aa0> as *const ()), // sub_142547d80
    Replacement::draft(0x142547d90, "unknown_142547d90", tpl_52607::<0x30, 0x1401e4630> as *const ()), // sub_142547d90
    Replacement::draft(0x142547da0, "unknown_142547da0", fn_142547da0 as *const ()), // sub_142547da0
    Replacement::draft(0x142547de0, "unknown_142547de0", tpl_52607::<0x80, 0x1401e46d0> as *const ()), // sub_142547de0
    Replacement::draft_throws(0x142547df0, "unknown_142547df0", fn_142547df0 as *const (), Throws::Discard), // sub_142547df0
    Replacement::draft(0x142547e20, "unknown_142547e20", tpl_327::<0x143a0a7e8, 0x14240ab10> as *const ()), // sub_142547e20
    Replacement::draft(0x142547e30, "unknown_142547e30", tpl_327::<0x143a0a830, 0x14240ab10> as *const ()), // sub_142547e30
    Replacement::draft(0x142547e40, "unknown_142547e40", tpl_327::<0x143843428, 0x1401de420> as *const ()), // sub_142547e40
    Replacement::draft(0x142547e50, "unknown_142547e50", tpl_327::<0x1438433b0, 0x1401de420> as *const ()), // sub_142547e50
    Replacement::draft(0x142547e60, "unknown_142547e60", tpl_327::<0x143843360, 0x1401de420> as *const ()), // sub_142547e60
    Replacement::draft(0x142547e70, "unknown_142547e70", tpl_327::<0x143a0a998, 0x14240ab10> as *const ()), // sub_142547e70
    Replacement::draft(0x142547e80, "unknown_142547e80", tpl_327::<0x143a0b010, 0x14240ab10> as *const ()), // sub_142547e80
    Replacement::draft(0x142547e90, "unknown_142547e90", tpl_327::<0x143a0ad40, 0x14240ab10> as *const ()), // sub_142547e90
    Replacement::draft(0x142547ea0, "unknown_142547ea0", tpl_327::<0x143a0af98, 0x14240ab10> as *const ()), // sub_142547ea0
    Replacement::draft(0x142547eb0, "unknown_142547eb0", tpl_327::<0x143a0abb0, 0x14240ab10> as *const ()), // sub_142547eb0
    Replacement::draft(0x142547ec0, "unknown_142547ec0", tpl_327::<0x143a0aa58, 0x14240ab10> as *const ()), // sub_142547ec0
    Replacement::draft(0x142547ed0, "unknown_142547ed0", tpl_327::<0x143a0b038, 0x14240ab10> as *const ()), // sub_142547ed0
    Replacement::draft(0x142547ee0, "unknown_142547ee0", tpl_327::<0x143a0a948, 0x14240ab10> as *const ()), // sub_142547ee0
    Replacement::draft(0x142547ef0, "unknown_142547ef0", tpl_327::<0x143a0b150, 0x14240ab10> as *const ()), // sub_142547ef0
    Replacement::draft(0x142547f00, "unknown_142547f00", tpl_327::<0x143a0ab20, 0x14240ab10> as *const ()), // sub_142547f00
    Replacement::draft(0x142547f10, "unknown_142547f10", tpl_327::<0x143a0b088, 0x14240ab10> as *const ()), // sub_142547f10
    Replacement::draft(0x142547f20, "unknown_142547f20", tpl_327::<0x143a0b0d8, 0x14240ab10> as *const ()), // sub_142547f20
    Replacement::draft(0x142547f30, "unknown_142547f30", tpl_327::<0x143a0a970, 0x14240ab10> as *const ()), // sub_142547f30
    Replacement::draft(0x142547f40, "unknown_142547f40", tpl_327::<0x143a0aa80, 0x14240ab10> as *const ()), // sub_142547f40
    Replacement::draft(0x142547f50, "unknown_142547f50", tpl_327::<0x143a0abf0, 0x14240ab10> as *const ()), // sub_142547f50
    Replacement::draft(0x142547f60, "unknown_142547f60", tpl_327::<0x143a0a658, 0x14240ab10> as *const ()), // sub_142547f60
    Replacement::draft(0x142547f70, "unknown_142547f70", tpl_327::<0x143a0ab60, 0x14240ab10> as *const ()), // sub_142547f70
    Replacement::draft(0x142547f80, "unknown_142547f80", tpl_327::<0x143a0aa30, 0x14240ab10> as *const ()), // sub_142547f80
    Replacement::draft(0x142547f90, "unknown_142547f90", tpl_327::<0x143a0af48, 0x14240ab10> as *const ()), // sub_142547f90
    Replacement::draft(0x142547fa0, "unknown_142547fa0", tpl_327::<0x143a0a680, 0x14240ab10> as *const ()), // sub_142547fa0
    Replacement::draft(0x142547fb0, "unknown_142547fb0", tpl_327::<0x143a0ade0, 0x14240ab10> as *const ()), // sub_142547fb0
    Replacement::draft(0x142547fc0, "unknown_142547fc0", tpl_327::<0x143a0a8a8, 0x14240ab10> as *const ()), // sub_142547fc0
    Replacement::draft(0x142547fd0, "unknown_142547fd0", tpl_327::<0x143a0ae80, 0x14240ab10> as *const ()), // sub_142547fd0
    Replacement::draft(0x142547fe0, "unknown_142547fe0", tpl_327::<0x143a0b060, 0x14240ab10> as *const ()), // sub_142547fe0
    Replacement::draft(0x142547ff0, "unknown_142547ff0", tpl_327::<0x143a0ad18, 0x14240ab10> as *const ()), // sub_142547ff0
    Replacement::draft(0x142548000, "unknown_142548000", tpl_327::<0x143a0afe8, 0x14240ab10> as *const ()), // sub_142548000
    Replacement::draft(0x142548010, "unknown_142548010", tpl_327::<0x143a0a630, 0x14240ab10> as *const ()), // sub_142548010
    Replacement::draft(0x142548020, "unknown_142548020", tpl_327::<0x143a0aef8, 0x14240ab10> as *const ()), // sub_142548020
    Replacement::draft(0x142548030, "unknown_142548030", tpl_327::<0x143a0a7c0, 0x14240ab10> as *const ()), // sub_142548030
    Replacement::draft(0x142548040, "unknown_142548040", tpl_327::<0x143a0ad68, 0x14240ab10> as *const ()), // sub_142548040
    Replacement::draft(0x142548050, "unknown_142548050", tpl_327::<0x143a0a880, 0x14240ab10> as *const ()), // sub_142548050
    Replacement::draft(0x142548060, "unknown_142548060", tpl_327::<0x143a0a748, 0x14240ab10> as *const ()), // sub_142548060
    Replacement::draft(0x142548070, "unknown_142548070", tpl_327::<0x143a0a798, 0x14240ab10> as *const ()), // sub_142548070
    Replacement::draft(0x142548080, "unknown_142548080", tpl_327::<0x143a0b0b0, 0x14240ab10> as *const ()), // sub_142548080
    Replacement::draft(0x142548090, "unknown_142548090", tpl_327::<0x143a0afc0, 0x14240ab10> as *const ()), // sub_142548090
    Replacement::draft(0x1425480a0, "unknown_1425480a0", tpl_327::<0x143a0aad0, 0x14240ab10> as *const ()), // sub_1425480a0
    Replacement::draft(0x1425480b0, "unknown_1425480b0", tpl_327::<0x143a0ac60, 0x14240ab10> as *const ()), // sub_1425480b0
    Replacement::draft(0x1425480c0, "unknown_1425480c0", tpl_327::<0x143a0b100, 0x14240ab10> as *const ()), // sub_1425480c0
    Replacement::draft(0x1425480d0, "unknown_1425480d0", tpl_327::<0x143a0a770, 0x14240ab10> as *const ()), // sub_1425480d0
    Replacement::draft(0x1425480e0, "unknown_1425480e0", tpl_327::<0x143a0aaa8, 0x14240ab10> as *const ()), // sub_1425480e0
    Replacement::draft(0x1425480f0, "unknown_1425480f0", tpl_327::<0x143a0ad90, 0x14240ab10> as *const ()), // sub_1425480f0
    Replacement::draft(0x142548100, "unknown_142548100", tpl_327::<0x143a0a9c0, 0x14240ab10> as *const ()), // sub_142548100
    Replacement::draft(0x142548110, "unknown_142548110", tpl_327::<0x143a0aea8, 0x14240ab10> as *const ()), // sub_142548110
    Replacement::draft(0x142548120, "unknown_142548120", tpl_327::<0x143a0ae08, 0x14240ab10> as *const ()), // sub_142548120
    Replacement::draft(0x142548130, "unknown_142548130", tpl_327::<0x143a0a8f8, 0x14240ab10> as *const ()), // sub_142548130
    Replacement::draft(0x142548140, "unknown_142548140", tpl_327::<0x143a0ab88, 0x14240ab10> as *const ()), // sub_142548140
    Replacement::draft(0x142548150, "unknown_142548150", tpl_327::<0x143a0acb0, 0x14240ab10> as *const ()), // sub_142548150
    Replacement::draft(0x142548160, "unknown_142548160", tpl_327::<0x143a0a8d0, 0x14240ab10> as *const ()), // sub_142548160
    Replacement::draft(0x142548170, "unknown_142548170", tpl_327::<0x143a0a920, 0x14240ab10> as *const ()), // sub_142548170
    Replacement::draft(0x142548180, "unknown_142548180", tpl_327::<0x143a0aaf8, 0x14240ab10> as *const ()), // sub_142548180
    Replacement::draft(0x142548190, "unknown_142548190", tpl_327::<0x143a0ae30, 0x14240ab10> as *const ()), // sub_142548190
    Replacement::draft(0x1425481a0, "unknown_1425481a0", tpl_327::<0x143a0aed0, 0x14240ab10> as *const ()), // sub_1425481a0
    Replacement::draft(0x1425481b0, "unknown_1425481b0", tpl_327::<0x143a0af70, 0x14240ab10> as *const ()), // sub_1425481b0
    Replacement::draft(0x1425481c0, "unknown_1425481c0", tpl_327::<0x143a0a720, 0x14240ab10> as *const ()), // sub_1425481c0
    Replacement::draft(0x1425481d0, "unknown_1425481d0", tpl_327::<0x143a0a858, 0x14240ab10> as *const ()), // sub_1425481d0
    Replacement::draft(0x1425481e0, "unknown_1425481e0", tpl_327::<0x143a0a6f8, 0x14240ab10> as *const ()), // sub_1425481e0
    Replacement::draft(0x1425481f0, "unknown_1425481f0", tpl_327::<0x143a0ae58, 0x14240ab10> as *const ()), // sub_1425481f0
    Replacement::draft(0x142548200, "unknown_142548200", tpl_327::<0x143a0acf0, 0x14240ab10> as *const ()), // sub_142548200
    Replacement::draft(0x142548210, "unknown_142548210", tpl_327::<0x143a0a9e8, 0x14240ab10> as *const ()), // sub_142548210
    Replacement::draft(0x142548220, "unknown_142548220", tpl_327::<0x143a0a6a8, 0x14240ab10> as *const ()), // sub_142548220
    Replacement::draft(0x142548230, "unknown_142548230", tpl_327::<0x143a0adb8, 0x14240ab10> as *const ()), // sub_142548230
    Replacement::draft(0x142548240, "unknown_142548240", tpl_327::<0x143a0b128, 0x14240ab10> as *const ()), // sub_142548240
    Replacement::draft(0x142548250, "unknown_142548250", tpl_327::<0x143a0af20, 0x14240ab10> as *const ()), // sub_142548250
    Replacement::draft(0x142548260, "unknown_142548260", tpl_327::<0x143a0ac88, 0x14240ab10> as *const ()), // sub_142548260
    Replacement::draft(0x142548270, "unknown_142548270", tpl_327::<0x143a0a6d0, 0x14240ab10> as *const ()), // sub_142548270
    Replacement::draft(0x142548280, "unknown_142548280", tpl_52607::<0x60, 0x14240ab10> as *const ()), // sub_142548280
    Replacement::draft_throws(0x142548290, "unknown_142548290", fn_142548290 as *const (), Throws::Discard), // sub_142548290
    Replacement::draft_throws(0x1425482b0, "unknown_1425482b0", tpl_52618::<0x20, 0x1401e68b0> as *const (), Throws::Discard), // sub_1425482b0
    Replacement::draft(0x1425482c0, "unknown_1425482c0", tpl_52607::<0x40, 0x141e6c530> as *const ()), // sub_1425482c0
    Replacement::draft(0x1425482d0, "unknown_1425482d0", tpl_52607::<0x48, 0x14240ab10> as *const ()), // sub_1425482d0
    Replacement::draft(0x1425482e0, "unknown_1425482e0", tpl_52607::<0x170, 0x1401e7140> as *const ()), // sub_1425482e0
    Replacement::draft(0x1425482f0, "unknown_1425482f0", tpl_52609::<0x40, 0x1401de3c0> as *const ()), // sub_1425482f0
    Replacement::draft(0x142548300, "unknown_142548300", tpl_52609::<0x40, 0x14240ab10> as *const ()), // sub_142548300
    Replacement::draft(0x142548310, "unknown_142548310", tpl_52607::<0x40, 0x1401e7140> as *const ()), // sub_142548310
    Replacement::draft(0x142548320, "unknown_142548320", tpl_52607::<0x50, 0x1401e7140> as *const ()), // sub_142548320
    Replacement::draft(0x142548330, "unknown_142548330", tpl_52607::<0x1a0, 0x1401e7140> as *const ()), // sub_142548330
    Replacement::draft(0x142548340, "unknown_142548340", tpl_52619::<0x1a0, 0x1401e7b10, 0x70, 0x1401de3c0> as *const ()), // sub_142548340
    Replacement::draft(0x142548360, "unknown_142548360", tpl_52609::<0x70, 0x14240ab10> as *const ()), // sub_142548360
    Replacement::draft(0x142548370, "unknown_142548370", tpl_52607::<0x40, 0x1401e7d60> as *const ()), // sub_142548370
    Replacement::draft(0x142548380, "unknown_142548380", tpl_52620::<0x40, 0x1401de3c0> as *const ()), // sub_142548380
    Replacement::draft(0x1425483b0, "unknown_1425483b0", tpl_52607::<0x20, 0x14240ab10> as *const ()), // sub_1425483b0
    Replacement::draft(0x1425483c0, "unknown_1425483c0", tpl_52607::<0x30, 0x1401e7d60> as *const ()), // sub_1425483c0
    Replacement::draft(0x1425483d0, "unknown_1425483d0", tpl_52620::<0x30, 0x1401de3c0> as *const ()), // sub_1425483d0
    Replacement::draft(0x142548400, "unknown_142548400", tpl_327::<0x143a0b330, 0x14240ab10> as *const ()), // sub_142548400
    Replacement::draft(0x142548410, "unknown_142548410", tpl_327::<0x143a0b378, 0x14240ab10> as *const ()), // sub_142548410
    Replacement::draft(0x142548420, "unknown_142548420", tpl_327::<0x143843520, 0x1401de420> as *const ()), // sub_142548420
    Replacement::draft(0x142548430, "unknown_142548430", tpl_327::<0x1438434f0, 0x1401de420> as *const ()), // sub_142548430
    Replacement::draft(0x142548440, "unknown_142548440", tpl_327::<0x1438434c0, 0x1401de420> as *const ()), // sub_142548440
    Replacement::draft(0x142548450, "unknown_142548450", tpl_327::<0x143a0b4e0, 0x14240ab10> as *const ()), // sub_142548450
    Replacement::draft(0x142548460, "unknown_142548460", tpl_327::<0x143a0bb28, 0x14240ab10> as *const ()), // sub_142548460
    Replacement::draft(0x142548470, "unknown_142548470", tpl_327::<0x143a0b858, 0x14240ab10> as *const ()), // sub_142548470
    Replacement::draft(0x142548480, "unknown_142548480", tpl_327::<0x143a0bab0, 0x14240ab10> as *const ()), // sub_142548480
    Replacement::draft(0x142548490, "unknown_142548490", tpl_327::<0x143a0b6f8, 0x14240ab10> as *const ()), // sub_142548490
    Replacement::draft(0x1425484a0, "unknown_1425484a0", tpl_327::<0x143a0b580, 0x14240ab10> as *const ()), // sub_1425484a0
    Replacement::draft(0x1425484b0, "unknown_1425484b0", tpl_327::<0x143a0bb50, 0x14240ab10> as *const ()), // sub_1425484b0
    Replacement::draft(0x1425484c0, "unknown_1425484c0", tpl_327::<0x143a0b490, 0x14240ab10> as *const ()), // sub_1425484c0
    Replacement::draft(0x1425484d0, "unknown_1425484d0", tpl_327::<0x143a0bc68, 0x14240ab10> as *const ()), // sub_1425484d0
    Replacement::draft(0x1425484e0, "unknown_1425484e0", tpl_327::<0x143a0b648, 0x14240ab10> as *const ()), // sub_1425484e0
    Replacement::draft(0x1425484f0, "unknown_1425484f0", tpl_327::<0x143a0bba0, 0x14240ab10> as *const ()), // sub_1425484f0
    Replacement::draft(0x142548500, "unknown_142548500", tpl_327::<0x143a0bbf0, 0x14240ab10> as *const ()), // sub_142548500
    Replacement::draft(0x142548510, "unknown_142548510", tpl_327::<0x143a0b4b8, 0x14240ab10> as *const ()), // sub_142548510
    Replacement::draft(0x142548520, "unknown_142548520", tpl_327::<0x143a0b5a8, 0x14240ab10> as *const ()), // sub_142548520
    Replacement::draft(0x142548530, "unknown_142548530", tpl_327::<0x143a0b720, 0x14240ab10> as *const ()), // sub_142548530
    Replacement::draft(0x142548540, "unknown_142548540", tpl_327::<0x143a0b1a0, 0x14240ab10> as *const ()), // sub_142548540
    Replacement::draft(0x142548550, "unknown_142548550", tpl_327::<0x143a0b670, 0x14240ab10> as *const ()), // sub_142548550
    Replacement::draft(0x142548560, "unknown_142548560", tpl_327::<0x143a0b558, 0x14240ab10> as *const ()), // sub_142548560
    Replacement::draft(0x142548570, "unknown_142548570", tpl_327::<0x143a0ba60, 0x14240ab10> as *const ()), // sub_142548570
    Replacement::draft(0x142548580, "unknown_142548580", tpl_327::<0x143a0b1c8, 0x14240ab10> as *const ()), // sub_142548580
    Replacement::draft(0x142548590, "unknown_142548590", tpl_327::<0x143a0b8f8, 0x14240ab10> as *const ()), // sub_142548590
    Replacement::draft(0x1425485a0, "unknown_1425485a0", tpl_327::<0x143a0b3f0, 0x14240ab10> as *const ()), // sub_1425485a0
    Replacement::draft(0x1425485b0, "unknown_1425485b0", tpl_327::<0x143a0b998, 0x14240ab10> as *const ()), // sub_1425485b0
    Replacement::draft(0x1425485c0, "unknown_1425485c0", tpl_327::<0x143a0bb78, 0x14240ab10> as *const ()), // sub_1425485c0
    Replacement::draft(0x1425485d0, "unknown_1425485d0", tpl_327::<0x143a0b830, 0x14240ab10> as *const ()), // sub_1425485d0
    Replacement::draft(0x1425485e0, "unknown_1425485e0", tpl_327::<0x143a0bb00, 0x14240ab10> as *const ()), // sub_1425485e0
    Replacement::draft(0x1425485f0, "unknown_1425485f0", tpl_327::<0x143a0b178, 0x14240ab10> as *const ()), // sub_1425485f0
    Replacement::draft(0x142548600, "unknown_142548600", tpl_327::<0x143a0ba10, 0x14240ab10> as *const ()), // sub_142548600
    Replacement::draft(0x142548610, "unknown_142548610", tpl_327::<0x143a0b308, 0x14240ab10> as *const ()), // sub_142548610
    Replacement::draft(0x142548620, "unknown_142548620", tpl_327::<0x143a0b880, 0x14240ab10> as *const ()), // sub_142548620
    Replacement::draft(0x142548630, "unknown_142548630", tpl_327::<0x143a0b3c8, 0x14240ab10> as *const ()), // sub_142548630
    Replacement::draft(0x142548640, "unknown_142548640", tpl_327::<0x143a0b290, 0x14240ab10> as *const ()), // sub_142548640
    Replacement::draft(0x142548650, "unknown_142548650", tpl_327::<0x143a0b2e0, 0x14240ab10> as *const ()), // sub_142548650
    Replacement::draft(0x142548660, "unknown_142548660", tpl_327::<0x143a0bbc8, 0x14240ab10> as *const ()), // sub_142548660
    Replacement::draft(0x142548670, "unknown_142548670", tpl_327::<0x143a0bad8, 0x14240ab10> as *const ()), // sub_142548670
    Replacement::draft(0x142548680, "unknown_142548680", tpl_327::<0x143a0b5f8, 0x14240ab10> as *const ()), // sub_142548680
    Replacement::draft(0x142548690, "unknown_142548690", tpl_327::<0x143a0b790, 0x14240ab10> as *const ()), // sub_142548690
    Replacement::draft(0x1425486a0, "unknown_1425486a0", tpl_327::<0x143a0bc18, 0x14240ab10> as *const ()), // sub_1425486a0
    Replacement::draft(0x1425486b0, "unknown_1425486b0", tpl_327::<0x143a0b2b8, 0x14240ab10> as *const ()), // sub_1425486b0
    Replacement::draft(0x1425486c0, "unknown_1425486c0", tpl_327::<0x143a0b5d0, 0x14240ab10> as *const ()), // sub_1425486c0
    Replacement::draft(0x1425486d0, "unknown_1425486d0", tpl_327::<0x143a0b8a8, 0x14240ab10> as *const ()), // sub_1425486d0
    Replacement::draft(0x1425486e0, "unknown_1425486e0", tpl_327::<0x143a0b508, 0x14240ab10> as *const ()), // sub_1425486e0
    Replacement::draft(0x1425486f0, "unknown_1425486f0", tpl_327::<0x143a0b9c0, 0x14240ab10> as *const ()), // sub_1425486f0
    Replacement::draft(0x142548700, "unknown_142548700", tpl_327::<0x143a0b920, 0x14240ab10> as *const ()), // sub_142548700
    Replacement::draft(0x142548710, "unknown_142548710", tpl_327::<0x143a0b440, 0x14240ab10> as *const ()), // sub_142548710
    Replacement::draft(0x142548720, "unknown_142548720", tpl_327::<0x143a0b6d0, 0x14240ab10> as *const ()), // sub_142548720
    Replacement::draft(0x142548730, "unknown_142548730", tpl_327::<0x143a0b7e0, 0x14240ab10> as *const ()), // sub_142548730
    Replacement::draft(0x142548740, "unknown_142548740", tpl_327::<0x143a0b418, 0x14240ab10> as *const ()), // sub_142548740
    Replacement::draft(0x142548750, "unknown_142548750", tpl_327::<0x143a0b468, 0x14240ab10> as *const ()), // sub_142548750
    Replacement::draft(0x142548760, "unknown_142548760", tpl_327::<0x143a0b620, 0x14240ab10> as *const ()), // sub_142548760
    Replacement::draft(0x142548770, "unknown_142548770", tpl_327::<0x143a0b948, 0x14240ab10> as *const ()), // sub_142548770
    Replacement::draft(0x142548780, "unknown_142548780", tpl_327::<0x143a0b9e8, 0x14240ab10> as *const ()), // sub_142548780
    Replacement::draft(0x142548790, "unknown_142548790", tpl_327::<0x143a0ba88, 0x14240ab10> as *const ()), // sub_142548790
    Replacement::draft(0x1425487a0, "unknown_1425487a0", tpl_327::<0x143a0b268, 0x14240ab10> as *const ()), // sub_1425487a0
    Replacement::draft(0x1425487b0, "unknown_1425487b0", tpl_327::<0x143a0b3a0, 0x14240ab10> as *const ()), // sub_1425487b0
    Replacement::draft(0x1425487c0, "unknown_1425487c0", tpl_327::<0x143a0b240, 0x14240ab10> as *const ()), // sub_1425487c0
    Replacement::draft(0x1425487d0, "unknown_1425487d0", tpl_327::<0x143a0b970, 0x14240ab10> as *const ()), // sub_1425487d0
    Replacement::draft(0x1425487e0, "unknown_1425487e0", tpl_327::<0x143a0b808, 0x14240ab10> as *const ()), // sub_1425487e0
    Replacement::draft(0x1425487f0, "unknown_1425487f0", tpl_327::<0x143a0b530, 0x14240ab10> as *const ()), // sub_1425487f0
    Replacement::draft(0x142548800, "unknown_142548800", tpl_327::<0x143a0b1f0, 0x14240ab10> as *const ()), // sub_142548800
    Replacement::draft(0x142548810, "unknown_142548810", tpl_327::<0x143a0b8d0, 0x14240ab10> as *const ()), // sub_142548810
    Replacement::draft(0x142548820, "unknown_142548820", tpl_327::<0x143a0bc40, 0x14240ab10> as *const ()), // sub_142548820
    Replacement::draft(0x142548830, "unknown_142548830", tpl_327::<0x143a0ba38, 0x14240ab10> as *const ()), // sub_142548830
    Replacement::draft(0x142548840, "unknown_142548840", tpl_327::<0x143a0b7b8, 0x14240ab10> as *const ()), // sub_142548840
    Replacement::draft(0x142548850, "unknown_142548850", tpl_327::<0x143a0b218, 0x14240ab10> as *const ()), // sub_142548850
    Replacement::draft(0x142548860, "unknown_142548860", fn_142548860 as *const ()), // sub_142548860
    Replacement::draft(0x1425488a0, "unknown_1425488a0", fn_1425488a0 as *const ()), // sub_1425488a0
    Replacement::draft(0x1425488e0, "unknown_1425488e0", tpl_52609::<0x40, 0x1401e7ae0> as *const ()), // sub_1425488e0
    Replacement::draft(0x1425488f0, "unknown_1425488f0", tpl_52609::<0x20, 0x1401e7ae0> as *const ()), // sub_1425488f0
    Replacement::draft(0x142548900, "unknown_142548900", tpl_52609::<0x30, 0x1401e7140> as *const ()), // sub_142548900
    Replacement::draft(0x142548910, "unknown_142548910", fn_142548910 as *const ()), // sub_142548910
    Replacement::draft(0x142548940, "unknown_142548940", tpl_52609::<0x20, 0x1401e7140> as *const ()), // sub_142548940
    Replacement::draft(0x142548950, "unknown_142548950", tpl_52609::<0x30, 0x1401ea2e0> as *const ()), // sub_142548950
    Replacement::draft(0x142548960, "unknown_142548960", tpl_52609::<0x40, 0x1401e7140> as *const ()), // sub_142548960
    Replacement::draft(0x142548970, "unknown_142548970", tpl_52609::<0x50, 0x1401e7140> as *const ()), // sub_142548970
    Replacement::draft(0x142548980, "unknown_142548980", fn_142548980 as *const ()), // sub_142548980
    Replacement::draft(0x1425489c0, "unknown_1425489c0", fn_1425489c0 as *const ()), // sub_1425489c0
    Replacement::draft(0x142548a00, "unknown_142548a00", tpl_52609::<0x30, 0x1401eb110> as *const ()), // sub_142548a00
    Replacement::draft(0x142548a10, "unknown_142548a10", tpl_52609::<0x150, 0x1401e7ae0> as *const ()), // sub_142548a10
    Replacement::draft(0x142548a20, "unknown_142548a20", tpl_52609::<0x30, 0x14240ab10> as *const ()), // sub_142548a20
    Replacement::draft(0x142548a30, "unknown_142548a30", fn_142548a30 as *const ()), // sub_142548a30
    Replacement::draft(0x142548ac0, "unknown_142548ac0", fn_142548ac0 as *const ()), // sub_142548ac0
    Replacement::draft(0x142548b00, "unknown_142548b00", fn_142548b00 as *const ()), // sub_142548b00
    Replacement::draft(0x142548b40, "unknown_142548b40", fn_142548b40 as *const ()), // sub_142548b40
    Replacement::draft(0x142548b80, "unknown_142548b80", fn_142548b80 as *const ()), // sub_142548b80
    Replacement::draft(0x142548bc0, "unknown_142548bc0", fn_142548bc0 as *const ()), // sub_142548bc0
    Replacement::draft(0x142548c00, "unknown_142548c00", fn_142548c00 as *const ()), // sub_142548c00
    Replacement::draft(0x142548c40, "unknown_142548c40", tpl_52609::<0x20, 0x1401ec990> as *const ()), // sub_142548c40
    Replacement::draft(0x142548c50, "unknown_142548c50", tpl_52607::<0x30, 0x1401de3c0> as *const ()), // sub_142548c50
    Replacement::draft(0x142548c60, "unknown_142548c60", fn_142548c60 as *const ()), // sub_142548c60
    Replacement::draft(0x142548cb0, "unknown_142548cb0", tpl_52634::<0x30, 0x1401de3c0> as *const ()), // sub_142548cb0
    Replacement::draft(0x142548cd0, "unknown_142548cd0", tpl_52607::<0x40, 0x1401ed720> as *const ()), // sub_142548cd0
    Replacement::draft(0x142548ce0, "unknown_142548ce0", tpl_52635::<0x40, 0x1401ed5c0> as *const ()), // sub_142548ce0
    Replacement::draft(0x142548d00, "unknown_142548d00", tpl_52607::<0x30, 0x1401ed720> as *const ()), // sub_142548d00
    Replacement::draft(0x142548d10, "unknown_142548d10", tpl_52635::<0x30, 0x1401ed5c0> as *const ()), // sub_142548d10
    Replacement::draft(0x142548d30, "unknown_142548d30", tpl_327::<0x143a0be48, 0x14240ab10> as *const ()), // sub_142548d30
    Replacement::draft(0x142548d40, "unknown_142548d40", tpl_327::<0x143a0be90, 0x14240ab10> as *const ()), // sub_142548d40
    Replacement::draft(0x142548d50, "unknown_142548d50", tpl_327::<0x143843600, 0x1401de420> as *const ()), // sub_142548d50
    Replacement::draft(0x142548d60, "unknown_142548d60", tpl_327::<0x1438435d0, 0x1401de420> as *const ()), // sub_142548d60
    Replacement::draft(0x142548d70, "unknown_142548d70", tpl_327::<0x1438435a0, 0x1401de420> as *const ()), // sub_142548d70
    Replacement::draft(0x142548d80, "unknown_142548d80", tpl_327::<0x143a0bff8, 0x14240ab10> as *const ()), // sub_142548d80
    Replacement::draft(0x142548d90, "unknown_142548d90", tpl_327::<0x143a0c608, 0x14240ab10> as *const ()), // sub_142548d90
    Replacement::draft(0x142548da0, "unknown_142548da0", tpl_327::<0x143a0c338, 0x14240ab10> as *const ()), // sub_142548da0
    Replacement::draft(0x142548db0, "unknown_142548db0", tpl_327::<0x143a0c590, 0x14240ab10> as *const ()), // sub_142548db0
    Replacement::draft(0x142548dc0, "unknown_142548dc0", tpl_327::<0x143a0c1d8, 0x14240ab10> as *const ()), // sub_142548dc0
    Replacement::draft(0x142548dd0, "unknown_142548dd0", tpl_327::<0x143a0c098, 0x14240ab10> as *const ()), // sub_142548dd0
    Replacement::draft(0x142548de0, "unknown_142548de0", tpl_327::<0x143a0c630, 0x14240ab10> as *const ()), // sub_142548de0
    Replacement::draft(0x142548df0, "unknown_142548df0", tpl_327::<0x143a0bfa8, 0x14240ab10> as *const ()), // sub_142548df0
    Replacement::draft(0x142548e00, "unknown_142548e00", tpl_327::<0x143a0c748, 0x14240ab10> as *const ()), // sub_142548e00
    Replacement::draft(0x142548e10, "unknown_142548e10", tpl_327::<0x143a0c160, 0x14240ab10> as *const ()), // sub_142548e10
    Replacement::draft(0x142548e20, "unknown_142548e20", tpl_327::<0x143a0c680, 0x14240ab10> as *const ()), // sub_142548e20
    Replacement::draft(0x142548e30, "unknown_142548e30", tpl_327::<0x143a0c6d0, 0x14240ab10> as *const ()), // sub_142548e30
    Replacement::draft(0x142548e40, "unknown_142548e40", tpl_327::<0x143a0bfd0, 0x14240ab10> as *const ()), // sub_142548e40
    Replacement::draft(0x142548e50, "unknown_142548e50", tpl_327::<0x143a0c0c0, 0x14240ab10> as *const ()), // sub_142548e50
    Replacement::draft(0x142548e60, "unknown_142548e60", tpl_327::<0x143a0c200, 0x14240ab10> as *const ()), // sub_142548e60
    Replacement::draft(0x142548e70, "unknown_142548e70", tpl_327::<0x143a0bcb8, 0x14240ab10> as *const ()), // sub_142548e70
    Replacement::draft(0x142548e80, "unknown_142548e80", tpl_327::<0x143a0c188, 0x14240ab10> as *const ()), // sub_142548e80
    Replacement::draft(0x142548e90, "unknown_142548e90", tpl_327::<0x143a0c070, 0x14240ab10> as *const ()), // sub_142548e90
    Replacement::draft(0x142548ea0, "unknown_142548ea0", tpl_327::<0x143a0c540, 0x14240ab10> as *const ()), // sub_142548ea0
    Replacement::draft(0x142548eb0, "unknown_142548eb0", tpl_327::<0x143a0bce0, 0x14240ab10> as *const ()), // sub_142548eb0
    Replacement::draft(0x142548ec0, "unknown_142548ec0", tpl_327::<0x143a0c3d8, 0x14240ab10> as *const ()), // sub_142548ec0
    Replacement::draft(0x142548ed0, "unknown_142548ed0", tpl_327::<0x143a0bf08, 0x14240ab10> as *const ()), // sub_142548ed0
    Replacement::draft(0x142548ee0, "unknown_142548ee0", tpl_327::<0x143a0c478, 0x14240ab10> as *const ()), // sub_142548ee0
    Replacement::draft(0x142548ef0, "unknown_142548ef0", tpl_327::<0x143a0c658, 0x14240ab10> as *const ()), // sub_142548ef0
    Replacement::draft(0x142548f00, "unknown_142548f00", tpl_327::<0x143a0c310, 0x14240ab10> as *const ()), // sub_142548f00
    Replacement::draft(0x142548f10, "unknown_142548f10", tpl_327::<0x143a0c5e0, 0x14240ab10> as *const ()), // sub_142548f10
    Replacement::draft(0x142548f20, "unknown_142548f20", tpl_327::<0x143a0bc90, 0x14240ab10> as *const ()), // sub_142548f20
    Replacement::draft(0x142548f30, "unknown_142548f30", tpl_327::<0x143a0c4f0, 0x14240ab10> as *const ()), // sub_142548f30
    Replacement::draft(0x142548f40, "unknown_142548f40", tpl_327::<0x143a0be20, 0x14240ab10> as *const ()), // sub_142548f40
    Replacement::draft(0x142548f50, "unknown_142548f50", tpl_327::<0x143a0c360, 0x14240ab10> as *const ()), // sub_142548f50
    Replacement::draft(0x142548f60, "unknown_142548f60", tpl_327::<0x143a0bee0, 0x14240ab10> as *const ()), // sub_142548f60
    Replacement::draft(0x142548f70, "unknown_142548f70", tpl_327::<0x143a0bda8, 0x14240ab10> as *const ()), // sub_142548f70
    Replacement::draft(0x142548f80, "unknown_142548f80", tpl_327::<0x143a0bdf8, 0x14240ab10> as *const ()), // sub_142548f80
    Replacement::draft(0x142548f90, "unknown_142548f90", tpl_327::<0x143a0c6a8, 0x14240ab10> as *const ()), // sub_142548f90
    Replacement::draft(0x142548fa0, "unknown_142548fa0", tpl_327::<0x143a0c5b8, 0x14240ab10> as *const ()), // sub_142548fa0
    Replacement::draft(0x142548fb0, "unknown_142548fb0", tpl_327::<0x143a0c110, 0x14240ab10> as *const ()), // sub_142548fb0
    Replacement::draft(0x142548fc0, "unknown_142548fc0", tpl_327::<0x143a0c270, 0x14240ab10> as *const ()), // sub_142548fc0
    Replacement::draft(0x142548fd0, "unknown_142548fd0", tpl_327::<0x143a0c6f8, 0x14240ab10> as *const ()), // sub_142548fd0
    Replacement::draft(0x142548fe0, "unknown_142548fe0", tpl_327::<0x143a0bdd0, 0x14240ab10> as *const ()), // sub_142548fe0
    Replacement::draft(0x142548ff0, "unknown_142548ff0", tpl_327::<0x143a0c0e8, 0x14240ab10> as *const ()), // sub_142548ff0
    Replacement::draft(0x142549000, "unknown_142549000", tpl_327::<0x143a0c388, 0x14240ab10> as *const ()), // sub_142549000
    Replacement::draft(0x142549010, "unknown_142549010", tpl_327::<0x143a0c020, 0x14240ab10> as *const ()), // sub_142549010
    Replacement::draft(0x142549020, "unknown_142549020", tpl_327::<0x143a0c4a0, 0x14240ab10> as *const ()), // sub_142549020
    Replacement::draft(0x142549030, "unknown_142549030", tpl_327::<0x143a0c400, 0x14240ab10> as *const ()), // sub_142549030
    Replacement::draft(0x142549040, "unknown_142549040", tpl_327::<0x143a0bf58, 0x14240ab10> as *const ()), // sub_142549040
    Replacement::draft(0x142549050, "unknown_142549050", tpl_327::<0x143a0c1b0, 0x14240ab10> as *const ()), // sub_142549050
    Replacement::draft(0x142549060, "unknown_142549060", tpl_327::<0x143a0c2c0, 0x14240ab10> as *const ()), // sub_142549060
    Replacement::draft(0x142549070, "unknown_142549070", tpl_327::<0x143a0bf30, 0x14240ab10> as *const ()), // sub_142549070
    Replacement::draft(0x142549080, "unknown_142549080", tpl_327::<0x143a0bf80, 0x14240ab10> as *const ()), // sub_142549080
    Replacement::draft(0x142549090, "unknown_142549090", tpl_327::<0x143a0c138, 0x14240ab10> as *const ()), // sub_142549090
    Replacement::draft(0x1425490a0, "unknown_1425490a0", tpl_327::<0x143a0c428, 0x14240ab10> as *const ()), // sub_1425490a0
    Replacement::draft(0x1425490b0, "unknown_1425490b0", tpl_327::<0x143a0c4c8, 0x14240ab10> as *const ()), // sub_1425490b0
    Replacement::draft(0x1425490c0, "unknown_1425490c0", tpl_327::<0x143a0c568, 0x14240ab10> as *const ()), // sub_1425490c0
    Replacement::draft(0x1425490d0, "unknown_1425490d0", tpl_327::<0x143a0bd80, 0x14240ab10> as *const ()), // sub_1425490d0
    Replacement::draft(0x1425490e0, "unknown_1425490e0", tpl_327::<0x143a0beb8, 0x14240ab10> as *const ()), // sub_1425490e0
    Replacement::draft(0x1425490f0, "unknown_1425490f0", tpl_327::<0x143a0bd58, 0x14240ab10> as *const ()), // sub_1425490f0
    Replacement::draft(0x142549100, "unknown_142549100", tpl_327::<0x143a0c450, 0x14240ab10> as *const ()), // sub_142549100
    Replacement::draft(0x142549110, "unknown_142549110", tpl_327::<0x143a0c2e8, 0x14240ab10> as *const ()), // sub_142549110
    Replacement::draft(0x142549120, "unknown_142549120", tpl_327::<0x143a0c048, 0x14240ab10> as *const ()), // sub_142549120
    Replacement::draft(0x142549130, "unknown_142549130", tpl_327::<0x143a0bd08, 0x14240ab10> as *const ()), // sub_142549130
    Replacement::draft(0x142549140, "unknown_142549140", tpl_327::<0x143a0c3b0, 0x14240ab10> as *const ()), // sub_142549140
    Replacement::draft(0x142549150, "unknown_142549150", tpl_327::<0x143a0c720, 0x14240ab10> as *const ()), // sub_142549150
    Replacement::draft(0x142549160, "unknown_142549160", tpl_327::<0x143a0c518, 0x14240ab10> as *const ()), // sub_142549160
    Replacement::draft(0x142549170, "unknown_142549170", tpl_327::<0x143a0c298, 0x14240ab10> as *const ()), // sub_142549170
    Replacement::draft(0x142549180, "unknown_142549180", tpl_327::<0x143a0bd30, 0x14240ab10> as *const ()), // sub_142549180
    Replacement::draft_throws(0x142549190, "unknown_142549190", tpl_52618::<0x30, 0x1401e68b0> as *const (), Throws::Discard), // sub_142549190
    Replacement::draft(0x1425491a0, "unknown_1425491a0", tpl_52609::<0x50, 0x1424b5180> as *const ()), // sub_1425491a0
    Replacement::draft(0x1425491b0, "unknown_1425491b0", tpl_52609::<0x190, 0x1401ee260> as *const ()), // sub_1425491b0
    Replacement::draft(0x1425491c0, "unknown_1425491c0", tpl_52609::<0x500, 0x1401ee260> as *const ()), // sub_1425491c0
    Replacement::draft(0x1425491d0, "unknown_1425491d0", tpl_52609::<0x50, 0x1401de3c0> as *const ()), // sub_1425491d0
    Replacement::draft(0x1425491e0, "unknown_1425491e0", tpl_327::<0x143a0c960, 0x14240ab10> as *const ()), // sub_1425491e0
    Replacement::draft(0x1425491f0, "unknown_1425491f0", tpl_327::<0x143a0c9a8, 0x14240ab10> as *const ()), // sub_1425491f0
    Replacement::draft(0x142549200, "unknown_142549200", tpl_327::<0x1438436a0, 0x1401de420> as *const ()), // sub_142549200
    Replacement::draft(0x142549210, "unknown_142549210", tpl_327::<0x143843670, 0x1401de420> as *const ()), // sub_142549210
    Replacement::draft(0x142549220, "unknown_142549220", tpl_327::<0x143843640, 0x1401de420> as *const ()), // sub_142549220
    Replacement::draft(0x142549230, "unknown_142549230", tpl_327::<0x143a0cb10, 0x14240ab10> as *const ()), // sub_142549230
    Replacement::draft(0x142549240, "unknown_142549240", tpl_327::<0x143a0d118, 0x14240ab10> as *const ()), // sub_142549240
    Replacement::draft(0x142549250, "unknown_142549250", tpl_327::<0x143a0ce48, 0x14240ab10> as *const ()), // sub_142549250
    Replacement::draft(0x142549260, "unknown_142549260", tpl_327::<0x143a0d0a0, 0x14240ab10> as *const ()), // sub_142549260
    Replacement::draft(0x142549270, "unknown_142549270", tpl_327::<0x143a0ccf0, 0x14240ab10> as *const ()), // sub_142549270
    Replacement::draft(0x142549280, "unknown_142549280", tpl_327::<0x143a0cbb0, 0x14240ab10> as *const ()), // sub_142549280
    Replacement::draft(0x142549290, "unknown_142549290", tpl_327::<0x143a0d140, 0x14240ab10> as *const ()), // sub_142549290
    Replacement::draft(0x1425492a0, "unknown_1425492a0", tpl_327::<0x143a0cac0, 0x14240ab10> as *const ()), // sub_1425492a0
    Replacement::draft(0x1425492b0, "unknown_1425492b0", tpl_327::<0x143a0d258, 0x14240ab10> as *const ()), // sub_1425492b0
    Replacement::draft(0x1425492c0, "unknown_1425492c0", tpl_327::<0x143a0cc78, 0x14240ab10> as *const ()), // sub_1425492c0
    Replacement::draft(0x1425492d0, "unknown_1425492d0", tpl_327::<0x143a0d190, 0x14240ab10> as *const ()), // sub_1425492d0
    Replacement::draft(0x1425492e0, "unknown_1425492e0", tpl_327::<0x143a0d1e0, 0x14240ab10> as *const ()), // sub_1425492e0
    Replacement::draft(0x1425492f0, "unknown_1425492f0", tpl_327::<0x143a0cae8, 0x14240ab10> as *const ()), // sub_1425492f0
    Replacement::draft(0x142549300, "unknown_142549300", tpl_327::<0x143a0cbd8, 0x14240ab10> as *const ()), // sub_142549300
    Replacement::draft(0x142549310, "unknown_142549310", tpl_327::<0x143a0cd18, 0x14240ab10> as *const ()), // sub_142549310
    Replacement::draft(0x142549320, "unknown_142549320", tpl_327::<0x143a0c7d0, 0x14240ab10> as *const ()), // sub_142549320
    Replacement::draft(0x142549330, "unknown_142549330", tpl_327::<0x143a0cca0, 0x14240ab10> as *const ()), // sub_142549330
    Replacement::draft(0x142549340, "unknown_142549340", tpl_327::<0x143a0cb88, 0x14240ab10> as *const ()), // sub_142549340
    Replacement::draft(0x142549350, "unknown_142549350", tpl_327::<0x143a0d050, 0x14240ab10> as *const ()), // sub_142549350
    Replacement::draft(0x142549360, "unknown_142549360", tpl_327::<0x143a0c7f8, 0x14240ab10> as *const ()), // sub_142549360
    Replacement::draft(0x142549370, "unknown_142549370", tpl_327::<0x143a0cee8, 0x14240ab10> as *const ()), // sub_142549370
    Replacement::draft(0x142549380, "unknown_142549380", tpl_327::<0x143a0ca20, 0x14240ab10> as *const ()), // sub_142549380
    Replacement::draft(0x142549390, "unknown_142549390", tpl_327::<0x143a0cf88, 0x14240ab10> as *const ()), // sub_142549390
    Replacement::draft(0x1425493a0, "unknown_1425493a0", tpl_327::<0x143a0d168, 0x14240ab10> as *const ()), // sub_1425493a0
    Replacement::draft(0x1425493b0, "unknown_1425493b0", tpl_327::<0x143a0ce20, 0x14240ab10> as *const ()), // sub_1425493b0
    Replacement::draft(0x1425493c0, "unknown_1425493c0", tpl_327::<0x143a0d0f0, 0x14240ab10> as *const ()), // sub_1425493c0
    Replacement::draft(0x1425493d0, "unknown_1425493d0", tpl_327::<0x143a0c7a8, 0x14240ab10> as *const ()), // sub_1425493d0
    Replacement::draft(0x1425493e0, "unknown_1425493e0", tpl_327::<0x143a0d000, 0x14240ab10> as *const ()), // sub_1425493e0
    Replacement::draft(0x1425493f0, "unknown_1425493f0", tpl_327::<0x143a0c938, 0x14240ab10> as *const ()), // sub_1425493f0
    Replacement::draft(0x142549400, "unknown_142549400", tpl_327::<0x143a0ce70, 0x14240ab10> as *const ()), // sub_142549400
    Replacement::draft(0x142549410, "unknown_142549410", tpl_327::<0x143a0c9f8, 0x14240ab10> as *const ()), // sub_142549410
    Replacement::draft(0x142549420, "unknown_142549420", tpl_327::<0x143a0c8c0, 0x14240ab10> as *const ()), // sub_142549420
    Replacement::draft(0x142549430, "unknown_142549430", tpl_327::<0x143a0c910, 0x14240ab10> as *const ()), // sub_142549430
    Replacement::draft(0x142549440, "unknown_142549440", tpl_327::<0x143a0d1b8, 0x14240ab10> as *const ()), // sub_142549440
    Replacement::draft(0x142549450, "unknown_142549450", tpl_327::<0x143a0d0c8, 0x14240ab10> as *const ()), // sub_142549450
    Replacement::draft(0x142549460, "unknown_142549460", tpl_327::<0x143a0cc28, 0x14240ab10> as *const ()), // sub_142549460
    Replacement::draft(0x142549470, "unknown_142549470", tpl_327::<0x143a0cd80, 0x14240ab10> as *const ()), // sub_142549470
    Replacement::draft(0x142549480, "unknown_142549480", tpl_327::<0x143a0d208, 0x14240ab10> as *const ()), // sub_142549480
    Replacement::draft(0x142549490, "unknown_142549490", tpl_327::<0x143a0c8e8, 0x14240ab10> as *const ()), // sub_142549490
    Replacement::draft(0x1425494a0, "unknown_1425494a0", tpl_327::<0x143a0cc00, 0x14240ab10> as *const ()), // sub_1425494a0
    Replacement::draft(0x1425494b0, "unknown_1425494b0", tpl_327::<0x143a0ce98, 0x14240ab10> as *const ()), // sub_1425494b0
    Replacement::draft(0x1425494c0, "unknown_1425494c0", tpl_327::<0x143a0cb38, 0x14240ab10> as *const ()), // sub_1425494c0
    Replacement::draft(0x1425494d0, "unknown_1425494d0", tpl_327::<0x143a0cfb0, 0x14240ab10> as *const ()), // sub_1425494d0
    Replacement::draft(0x1425494e0, "unknown_1425494e0", tpl_327::<0x143a0cf10, 0x14240ab10> as *const ()), // sub_1425494e0
    Replacement::draft(0x1425494f0, "unknown_1425494f0", tpl_327::<0x143a0ca70, 0x14240ab10> as *const ()), // sub_1425494f0
    Replacement::draft(0x142549500, "unknown_142549500", tpl_327::<0x143a0ccc8, 0x14240ab10> as *const ()), // sub_142549500
    Replacement::draft(0x142549510, "unknown_142549510", tpl_327::<0x143a0cdd0, 0x14240ab10> as *const ()), // sub_142549510
    Replacement::draft(0x142549520, "unknown_142549520", tpl_327::<0x143a0ca48, 0x14240ab10> as *const ()), // sub_142549520
    Replacement::draft(0x142549530, "unknown_142549530", tpl_327::<0x143a0ca98, 0x14240ab10> as *const ()), // sub_142549530
    Replacement::draft(0x142549540, "unknown_142549540", tpl_327::<0x143a0cc50, 0x14240ab10> as *const ()), // sub_142549540
    Replacement::draft(0x142549550, "unknown_142549550", tpl_327::<0x143a0cf38, 0x14240ab10> as *const ()), // sub_142549550
    Replacement::draft(0x142549560, "unknown_142549560", tpl_327::<0x143a0cfd8, 0x14240ab10> as *const ()), // sub_142549560
    Replacement::draft(0x142549570, "unknown_142549570", tpl_327::<0x143a0d078, 0x14240ab10> as *const ()), // sub_142549570
    Replacement::draft(0x142549580, "unknown_142549580", tpl_327::<0x143a0c898, 0x14240ab10> as *const ()), // sub_142549580
    Replacement::draft(0x142549590, "unknown_142549590", tpl_327::<0x143a0c9d0, 0x14240ab10> as *const ()), // sub_142549590
    Replacement::draft(0x1425495a0, "unknown_1425495a0", tpl_327::<0x143a0c870, 0x14240ab10> as *const ()), // sub_1425495a0
    Replacement::draft(0x1425495b0, "unknown_1425495b0", tpl_327::<0x143a0cf60, 0x14240ab10> as *const ()), // sub_1425495b0
    Replacement::draft(0x1425495c0, "unknown_1425495c0", tpl_327::<0x143a0cdf8, 0x14240ab10> as *const ()), // sub_1425495c0
    Replacement::draft(0x1425495d0, "unknown_1425495d0", tpl_327::<0x143a0cb60, 0x14240ab10> as *const ()), // sub_1425495d0
    Replacement::draft(0x1425495e0, "unknown_1425495e0", tpl_327::<0x143a0c820, 0x14240ab10> as *const ()), // sub_1425495e0
    Replacement::draft(0x1425495f0, "unknown_1425495f0", tpl_327::<0x143a0cec0, 0x14240ab10> as *const ()), // sub_1425495f0
    Replacement::draft(0x142549600, "unknown_142549600", tpl_327::<0x143a0d230, 0x14240ab10> as *const ()), // sub_142549600
    Replacement::draft(0x142549610, "unknown_142549610", tpl_327::<0x143a0d028, 0x14240ab10> as *const ()), // sub_142549610
    Replacement::draft(0x142549620, "unknown_142549620", tpl_327::<0x143a0cda8, 0x14240ab10> as *const ()), // sub_142549620
    Replacement::draft(0x142549630, "unknown_142549630", tpl_327::<0x143a0c848, 0x14240ab10> as *const ()), // sub_142549630
    Replacement::draft_throws(0x142549640, "unknown_142549640", fn_142549640 as *const (), Throws::Discard), // sub_142549640
    Replacement::draft(0x142549660, "unknown_142549660", tpl_52609::<0x20, 0x1401ee770> as *const ()), // sub_142549660
    Replacement::draft_throws(0x142549670, "unknown_142549670", fn_142549670 as *const (), Throws::Discard), // sub_142549670
    Replacement::draft(0x142549690, "unknown_142549690", tpl_327::<0x143a0d4a8, 0x14240ab10> as *const ()), // sub_142549690
    Replacement::draft(0x1425496a0, "unknown_1425496a0", tpl_327::<0x143a0d4f0, 0x14240ab10> as *const ()), // sub_1425496a0
    Replacement::draft(0x1425496b0, "unknown_1425496b0", tpl_327::<0x143843730, 0x1401de420> as *const ()), // sub_1425496b0
    Replacement::draft(0x1425496c0, "unknown_1425496c0", tpl_327::<0x143843700, 0x1401de420> as *const ()), // sub_1425496c0
    Replacement::draft(0x1425496d0, "unknown_1425496d0", tpl_327::<0x1438436d0, 0x1401de420> as *const ()), // sub_1425496d0
    Replacement::draft(0x1425496e0, "unknown_1425496e0", tpl_327::<0x143a0d658, 0x14240ab10> as *const ()), // sub_1425496e0
    Replacement::draft(0x1425496f0, "unknown_1425496f0", tpl_327::<0x143a0dc68, 0x14240ab10> as *const ()), // sub_1425496f0
    Replacement::draft(0x142549700, "unknown_142549700", tpl_327::<0x143a0d998, 0x14240ab10> as *const ()), // sub_142549700
    Replacement::draft(0x142549710, "unknown_142549710", tpl_327::<0x143a0dbf0, 0x14240ab10> as *const ()), // sub_142549710
    Replacement::draft(0x142549720, "unknown_142549720", tpl_327::<0x143a0d838, 0x14240ab10> as *const ()), // sub_142549720
    Replacement::draft(0x142549730, "unknown_142549730", tpl_327::<0x143a0d6f8, 0x14240ab10> as *const ()), // sub_142549730
    Replacement::draft(0x142549740, "unknown_142549740", tpl_327::<0x143a0dc90, 0x14240ab10> as *const ()), // sub_142549740
    Replacement::draft(0x142549750, "unknown_142549750", tpl_327::<0x143a0d608, 0x14240ab10> as *const ()), // sub_142549750
    Replacement::draft(0x142549760, "unknown_142549760", tpl_327::<0x143a0dda8, 0x14240ab10> as *const ()), // sub_142549760
    Replacement::draft(0x142549770, "unknown_142549770", tpl_327::<0x143a0d7c0, 0x14240ab10> as *const ()), // sub_142549770
    Replacement::draft(0x142549780, "unknown_142549780", tpl_327::<0x143a0dce0, 0x14240ab10> as *const ()), // sub_142549780
    Replacement::draft(0x142549790, "unknown_142549790", tpl_327::<0x143a0dd30, 0x14240ab10> as *const ()), // sub_142549790
    Replacement::draft(0x1425497a0, "unknown_1425497a0", tpl_327::<0x143a0d630, 0x14240ab10> as *const ()), // sub_1425497a0
    Replacement::draft(0x1425497b0, "unknown_1425497b0", tpl_327::<0x143a0d720, 0x14240ab10> as *const ()), // sub_1425497b0
    Replacement::draft(0x1425497c0, "unknown_1425497c0", tpl_327::<0x143a0d860, 0x14240ab10> as *const ()), // sub_1425497c0
    Replacement::draft(0x1425497d0, "unknown_1425497d0", tpl_327::<0x143a0d318, 0x14240ab10> as *const ()), // sub_1425497d0
    Replacement::draft(0x1425497e0, "unknown_1425497e0", tpl_327::<0x143a0d7e8, 0x14240ab10> as *const ()), // sub_1425497e0
    Replacement::draft(0x1425497f0, "unknown_1425497f0", tpl_327::<0x143a0d6d0, 0x14240ab10> as *const ()), // sub_1425497f0
    Replacement::draft(0x142549800, "unknown_142549800", tpl_327::<0x143a0dba0, 0x14240ab10> as *const ()), // sub_142549800
    Replacement::draft(0x142549810, "unknown_142549810", tpl_327::<0x143a0d340, 0x14240ab10> as *const ()), // sub_142549810
    Replacement::draft(0x142549820, "unknown_142549820", tpl_327::<0x143a0da38, 0x14240ab10> as *const ()), // sub_142549820
    Replacement::draft(0x142549830, "unknown_142549830", tpl_327::<0x143a0d568, 0x14240ab10> as *const ()), // sub_142549830
    Replacement::draft(0x142549840, "unknown_142549840", tpl_327::<0x143a0dad8, 0x14240ab10> as *const ()), // sub_142549840
    Replacement::draft(0x142549850, "unknown_142549850", tpl_327::<0x143a0dcb8, 0x14240ab10> as *const ()), // sub_142549850
    Replacement::draft(0x142549860, "unknown_142549860", tpl_327::<0x143a0d970, 0x14240ab10> as *const ()), // sub_142549860
    Replacement::draft(0x142549870, "unknown_142549870", tpl_327::<0x143a0dc40, 0x14240ab10> as *const ()), // sub_142549870
    Replacement::draft(0x142549880, "unknown_142549880", tpl_327::<0x143a0d2f0, 0x14240ab10> as *const ()), // sub_142549880
    Replacement::draft(0x142549890, "unknown_142549890", tpl_327::<0x143a0db50, 0x14240ab10> as *const ()), // sub_142549890
    Replacement::draft(0x1425498a0, "unknown_1425498a0", tpl_327::<0x143a0d480, 0x14240ab10> as *const ()), // sub_1425498a0
    Replacement::draft(0x1425498b0, "unknown_1425498b0", tpl_327::<0x143a0d9c0, 0x14240ab10> as *const ()), // sub_1425498b0
    Replacement::draft(0x1425498c0, "unknown_1425498c0", tpl_327::<0x143a0d540, 0x14240ab10> as *const ()), // sub_1425498c0
    Replacement::draft(0x1425498d0, "unknown_1425498d0", tpl_327::<0x143a0d408, 0x14240ab10> as *const ()), // sub_1425498d0
    Replacement::draft(0x1425498e0, "unknown_1425498e0", tpl_327::<0x143a0d458, 0x14240ab10> as *const ()), // sub_1425498e0
    Replacement::draft(0x1425498f0, "unknown_1425498f0", tpl_327::<0x143a0dd08, 0x14240ab10> as *const ()), // sub_1425498f0
    Replacement::draft(0x142549900, "unknown_142549900", tpl_327::<0x143a0dc18, 0x14240ab10> as *const ()), // sub_142549900
    Replacement::draft(0x142549910, "unknown_142549910", tpl_327::<0x143a0d770, 0x14240ab10> as *const ()), // sub_142549910
    Replacement::draft(0x142549920, "unknown_142549920", tpl_327::<0x143a0d8d0, 0x14240ab10> as *const ()), // sub_142549920
    Replacement::draft(0x142549930, "unknown_142549930", tpl_327::<0x143a0dd58, 0x14240ab10> as *const ()), // sub_142549930
    Replacement::draft(0x142549940, "unknown_142549940", tpl_327::<0x143a0d430, 0x14240ab10> as *const ()), // sub_142549940
    Replacement::draft(0x142549950, "unknown_142549950", tpl_327::<0x143a0d748, 0x14240ab10> as *const ()), // sub_142549950
    Replacement::draft(0x142549960, "unknown_142549960", tpl_327::<0x143a0d9e8, 0x14240ab10> as *const ()), // sub_142549960
    Replacement::draft(0x142549970, "unknown_142549970", tpl_327::<0x143a0d680, 0x14240ab10> as *const ()), // sub_142549970
    Replacement::draft(0x142549980, "unknown_142549980", tpl_327::<0x143a0db00, 0x14240ab10> as *const ()), // sub_142549980
    Replacement::draft(0x142549990, "unknown_142549990", tpl_327::<0x143a0da60, 0x14240ab10> as *const ()), // sub_142549990
    Replacement::draft(0x1425499a0, "unknown_1425499a0", tpl_327::<0x143a0d5b8, 0x14240ab10> as *const ()), // sub_1425499a0
    Replacement::draft(0x1425499b0, "unknown_1425499b0", tpl_327::<0x143a0d810, 0x14240ab10> as *const ()), // sub_1425499b0
    Replacement::draft(0x1425499c0, "unknown_1425499c0", tpl_327::<0x143a0d920, 0x14240ab10> as *const ()), // sub_1425499c0
    Replacement::draft(0x1425499d0, "unknown_1425499d0", tpl_327::<0x143a0d590, 0x14240ab10> as *const ()), // sub_1425499d0
    Replacement::draft(0x1425499e0, "unknown_1425499e0", tpl_327::<0x143a0d5e0, 0x14240ab10> as *const ()), // sub_1425499e0
    Replacement::draft(0x1425499f0, "unknown_1425499f0", tpl_327::<0x143a0d798, 0x14240ab10> as *const ()), // sub_1425499f0
    Replacement::draft(0x142549a00, "unknown_142549a00", tpl_327::<0x143a0da88, 0x14240ab10> as *const ()), // sub_142549a00
    Replacement::draft(0x142549a10, "unknown_142549a10", tpl_327::<0x143a0db28, 0x14240ab10> as *const ()), // sub_142549a10
    Replacement::draft(0x142549a20, "unknown_142549a20", tpl_327::<0x143a0dbc8, 0x14240ab10> as *const ()), // sub_142549a20
    Replacement::draft(0x142549a30, "unknown_142549a30", tpl_327::<0x143a0d3e0, 0x14240ab10> as *const ()), // sub_142549a30
    Replacement::draft(0x142549a40, "unknown_142549a40", tpl_327::<0x143a0d518, 0x14240ab10> as *const ()), // sub_142549a40
    Replacement::draft(0x142549a50, "unknown_142549a50", tpl_327::<0x143a0d3b8, 0x14240ab10> as *const ()), // sub_142549a50
    Replacement::draft(0x142549a60, "unknown_142549a60", tpl_327::<0x143a0dab0, 0x14240ab10> as *const ()), // sub_142549a60
    Replacement::draft(0x142549a70, "unknown_142549a70", tpl_327::<0x143a0d948, 0x14240ab10> as *const ()), // sub_142549a70
    Replacement::draft(0x142549a80, "unknown_142549a80", tpl_327::<0x143a0d6a8, 0x14240ab10> as *const ()), // sub_142549a80
    Replacement::draft(0x142549a90, "unknown_142549a90", tpl_327::<0x143a0d368, 0x14240ab10> as *const ()), // sub_142549a90
    Replacement::draft(0x142549aa0, "unknown_142549aa0", tpl_327::<0x143a0da10, 0x14240ab10> as *const ()), // sub_142549aa0
    Replacement::draft(0x142549ab0, "unknown_142549ab0", tpl_327::<0x143a0dd80, 0x14240ab10> as *const ()), // sub_142549ab0
    Replacement::draft(0x142549ac0, "unknown_142549ac0", tpl_327::<0x143a0db78, 0x14240ab10> as *const ()), // sub_142549ac0
    Replacement::draft(0x142549ad0, "unknown_142549ad0", tpl_327::<0x143a0d8f8, 0x14240ab10> as *const ()), // sub_142549ad0
    Replacement::draft(0x142549ae0, "unknown_142549ae0", tpl_327::<0x143a0d390, 0x14240ab10> as *const ()), // sub_142549ae0
    Replacement::draft(0x142549af0, "unknown_142549af0", tpl_327::<0x143a0de48, 0x14240ab10> as *const ()), // sub_142549af0
    Replacement::draft(0x142549b00, "unknown_142549b00", tpl_327::<0x143a0de90, 0x14240ab10> as *const ()), // sub_142549b00
    Replacement::draft(0x142549b10, "unknown_142549b10", tpl_327::<0x1438437c0, 0x1401de420> as *const ()), // sub_142549b10
    Replacement::draft(0x142549b20, "unknown_142549b20", tpl_327::<0x143843790, 0x1401de420> as *const ()), // sub_142549b20
    Replacement::draft(0x142549b30, "unknown_142549b30", tpl_327::<0x143843760, 0x1401de420> as *const ()), // sub_142549b30
    Replacement::draft(0x142549b40, "unknown_142549b40", tpl_327::<0x143a0df30, 0x14240ab10> as *const ()), // sub_142549b40
    Replacement::draft(0x142549b50, "unknown_142549b50", tpl_327::<0x143a0e268, 0x14240ab10> as *const ()), // sub_142549b50
    Replacement::draft(0x142549b60, "unknown_142549b60", tpl_327::<0x143a0e118, 0x14240ab10> as *const ()), // sub_142549b60
    Replacement::draft(0x142549b70, "unknown_142549b70", tpl_327::<0x143a0e218, 0x14240ab10> as *const ()), // sub_142549b70
    Replacement::draft(0x142549b80, "unknown_142549b80", tpl_327::<0x143a0e058, 0x14240ab10> as *const ()), // sub_142549b80
    Replacement::draft(0x142549b90, "unknown_142549b90", tpl_327::<0x143a0df80, 0x14240ab10> as *const ()), // sub_142549b90
    Replacement::draft(0x142549ba0, "unknown_142549ba0", tpl_327::<0x143a0e290, 0x14240ab10> as *const ()), // sub_142549ba0
    Replacement::draft(0x142549bb0, "unknown_142549bb0", tpl_327::<0x143a0dee0, 0x14240ab10> as *const ()), // sub_142549bb0
    Replacement::draft(0x142549bc0, "unknown_142549bc0", tpl_327::<0x143a0e330, 0x14240ab10> as *const ()), // sub_142549bc0
    Replacement::draft(0x142549bd0, "unknown_142549bd0", tpl_327::<0x143a0e008, 0x14240ab10> as *const ()), // sub_142549bd0
    Replacement::draft(0x142549be0, "unknown_142549be0", tpl_327::<0x143a0e2e0, 0x14240ab10> as *const ()), // sub_142549be0
    Replacement::draft(0x142549bf0, "unknown_142549bf0", tpl_327::<0x143a0e308, 0x14240ab10> as *const ()), // sub_142549bf0
    Replacement::draft(0x142549c00, "unknown_142549c00", tpl_327::<0x143a0df08, 0x14240ab10> as *const ()), // sub_142549c00
    Replacement::draft(0x142549c10, "unknown_142549c10", tpl_327::<0x143a0dfa8, 0x14240ab10> as *const ()), // sub_142549c10
    Replacement::draft(0x142549c20, "unknown_142549c20", tpl_327::<0x143a0e080, 0x14240ab10> as *const ()), // sub_142549c20
    Replacement::draft(0x142549c30, "unknown_142549c30", tpl_327::<0x143a0ddf8, 0x14240ab10> as *const ()), // sub_142549c30
    Replacement::draft(0x142549c40, "unknown_142549c40", tpl_327::<0x143a0e030, 0x14240ab10> as *const ()), // sub_142549c40
    Replacement::draft(0x142549c50, "unknown_142549c50", tpl_327::<0x143a0df58, 0x14240ab10> as *const ()), // sub_142549c50
    Replacement::draft(0x142549c60, "unknown_142549c60", tpl_327::<0x143a0e1f0, 0x14240ab10> as *const ()), // sub_142549c60
    Replacement::draft(0x142549c70, "unknown_142549c70", tpl_327::<0x143a0de20, 0x14240ab10> as *const ()), // sub_142549c70
    Replacement::draft(0x142549c80, "unknown_142549c80", tpl_327::<0x143a0e178, 0x14240ab10> as *const ()), // sub_142549c80
    Replacement::draft(0x142549c90, "unknown_142549c90", tpl_327::<0x143a0deb8, 0x14240ab10> as *const ()), // sub_142549c90
    Replacement::draft(0x142549ca0, "unknown_142549ca0", tpl_327::<0x143a0e1a0, 0x14240ab10> as *const ()), // sub_142549ca0
    Replacement::draft(0x142549cb0, "unknown_142549cb0", tpl_327::<0x143a0e2b8, 0x14240ab10> as *const ()), // sub_142549cb0
    Replacement::draft(0x142549cc0, "unknown_142549cc0", tpl_327::<0x143a0e0f0, 0x14240ab10> as *const ()), // sub_142549cc0
    Replacement::draft(0x142549cd0, "unknown_142549cd0", tpl_327::<0x143a0e240, 0x14240ab10> as *const ()), // sub_142549cd0
    Replacement::draft(0x142549ce0, "unknown_142549ce0", tpl_327::<0x143a0ddd0, 0x14240ab10> as *const ()), // sub_142549ce0
    Replacement::draft(0x142549cf0, "unknown_142549cf0", tpl_327::<0x143a0e1c8, 0x14240ab10> as *const ()), // sub_142549cf0
    Replacement::draft(0x142549d00, "unknown_142549d00", tpl_52609::<0x60, 0x14240ab10> as *const ()), // sub_142549d00
    Replacement::draft(0x142549d10, "unknown_142549d10", tpl_52609::<0x20, 0x1401f2670> as *const ()), // sub_142549d10
    Replacement::draft(0x142549d20, "unknown_142549d20", tpl_327::<0x143a0e358, 0x14240ab10> as *const ()), // sub_142549d20
    Replacement::draft(0x142549d30, "unknown_142549d30", tpl_327::<0x143a0e3a0, 0x14240ab10> as *const ()), // sub_142549d30
    Replacement::draft(0x142549d40, "unknown_142549d40", fn_142549d40 as *const ()), // sub_142549d40
    Replacement::draft(0x142549d60, "unknown_142549d60", tpl_327::<0x143843830, 0x1401de420> as *const ()), // sub_142549d60
    Replacement::draft(0x142549d70, "unknown_142549d70", tpl_327::<0x1438437f0, 0x1401de420> as *const ()), // sub_142549d70
    Replacement::draft(0x142549d80, "unknown_142549d80", tpl_52609::<0x28, 0x1401f3d00> as *const ()), // sub_142549d80
    Replacement::draft_throws(0x14254a240, "unknown_14254a240", fn_14254a240 as *const (), Throws::Discard), // sub_14254a240
    Replacement::draft_throws(0x14254a290, "unknown_14254a290", fn_14254a290 as *const (), Throws::Discard), // sub_14254a290
    Replacement::draft_throws(0x14254a4d0, "unknown_14254a4d0", fn_14254a4d0 as *const (), Throws::Discard), // sub_14254a4d0
    Replacement::draft_throws(0x14254aa20, "unknown_14254aa20", fn_14254aa20 as *const (), Throws::Discard), // sub_14254aa20
    Replacement::draft(0x14254abf0, "unknown_14254abf0", fn_14254abf0 as *const ()), // sub_14254abf0
    Replacement::draft(0x14254ad40, "unknown_14254ad40", fn_14254ad40 as *const ()), // sub_14254ad40
    Replacement::draft_throws(0x14254ae00, "unknown_14254ae00", fn_14254ae00 as *const (), Throws::Discard), // sub_14254ae00
    Replacement::draft_throws(0x14254aef0, "unknown_14254aef0", fn_14254aef0 as *const (), Throws::Discard), // sub_14254aef0
    Replacement::draft_throws(0x14254af20, "unknown_14254af20", fn_14254af20 as *const (), Throws::Discard), // sub_14254af20
    Replacement::draft_throws(0x14254af40, "unknown_14254af40", fn_14254af40 as *const (), Throws::Discard), // sub_14254af40
    Replacement::draft(0x14254b000, "unknown_14254b000", tpl_52648::<0x70, 0x1401de3c0, 0x78, 0x14240ab10> as *const ()), // sub_14254b000
    Replacement::draft(0x14254b150, "unknown_14254b150", fn_14254b150 as *const ()), // sub_14254b150
    Replacement::draft_throws(0x14254bc70, "unknown_14254bc70", fn_14254bc70 as *const (), Throws::Discard), // sub_14254bc70
    Replacement::draft(0x14254c530, "unknown_14254c530", fn_14254c530 as *const ()), // sub_14254c530
    Replacement::draft(0x14254c730, "unknown_14254c730", tpl_52652::<0x68, 0x1401ff400, 0x70, 0x14240ab10> as *const ()), // sub_14254c730
    Replacement::draft(0x14254d230, "unknown_14254d230", fn_14254d230 as *const ()), // sub_14254d230
    Replacement::draft(0x14254d7b0, "unknown_14254d7b0", fn_14254d7b0 as *const ()), // sub_14254d7b0
    Replacement::draft(0x14254d890, "unknown_14254d890", fn_14254d890 as *const ()), // sub_14254d890
    Replacement::draft(0x14254d8f0, "unknown_14254d8f0", fn_14254d8f0 as *const ()), // sub_14254d8f0
    Replacement::draft_throws(0x14254d910, "unknown_14254d910", fn_14254d910 as *const (), Throws::Discard), // sub_14254d910
    Replacement::draft_throws(0x14254d940, "unknown_14254d940", fn_14254d940 as *const (), Throws::Discard), // sub_14254d940
    Replacement::draft(0x14254de60, "unknown_14254de60", tpl_52659::<0x400, 0x1401de3c0, 0x30, 0x140265b60> as *const ()), // sub_14254de60
    Replacement::draft(0x14254df20, "unknown_14254df20", fn_14254df20 as *const ()), // sub_14254df20
    Replacement::draft(0x14254e020, "unknown_14254e020", fn_14254e020 as *const ()), // sub_14254e020
    Replacement::draft(0x14254f230, "unknown_14254f230", fn_14254f230 as *const ()), // sub_14254f230
    Replacement::draft(0x14254f6c0, "unknown_14254f6c0", fn_14254f6c0 as *const ()), // sub_14254f6c0
    Replacement::draft_throws(0x14254f770, "unknown_14254f770", fn_14254f770 as *const (), Throws::Discard), // sub_14254f770
    Replacement::draft(0x14254feb0, "unknown_14254feb0", fn_14254feb0 as *const ()), // sub_14254feb0
    Replacement::draft(0x14254ff20, "unknown_14254ff20", fn_14254ff20 as *const ()), // sub_14254ff20
    Replacement::draft(0x1425500d0, "unknown_1425500d0", fn_1425500d0 as *const ()), // sub_1425500d0
    Replacement::draft_throws(0x1425503e0, "unknown_1425503e0", fn_1425503e0 as *const (), Throws::Discard), // sub_1425503e0
    Replacement::draft_throws(0x142550430, "unknown_142550430", fn_142550430 as *const (), Throws::Discard), // sub_142550430
    Replacement::draft(0x142550490, "unknown_142550490", fn_142550490 as *const ()), // sub_142550490
    Replacement::draft(0x142550520, "unknown_142550520", fn_142550520 as *const ()), // sub_142550520
    Replacement::draft(0x1425505d0, "unknown_1425505d0", fn_1425505d0 as *const ()), // sub_1425505d0
    Replacement::draft_throws(0x142550670, "unknown_142550670", fn_142550670 as *const (), Throws::Discard), // sub_142550670
    Replacement::draft(0x1425507a0, "unknown_1425507a0", fn_1425507a0 as *const ()), // sub_1425507a0
    Replacement::draft(0x142550860, "unknown_142550860", fn_142550860 as *const ()), // sub_142550860
    Replacement::draft(0x142550910, "unknown_142550910", fn_142550910 as *const ()), // sub_142550910
    Replacement::draft(0x1425509b0, "unknown_1425509b0", fn_1425509b0 as *const ()), // sub_1425509b0
    Replacement::draft(0x142550a40, "unknown_142550a40", fn_142550a40 as *const ()), // sub_142550a40
    Replacement::draft(0x142550ae0, "unknown_142550ae0", fn_142550ae0 as *const ()), // sub_142550ae0
    Replacement::draft(0x142550d30, "unknown_142550d30", fn_142550d30 as *const ()), // sub_142550d30
    Replacement::draft(0x142550dc0, "unknown_142550dc0", fn_142550dc0 as *const ()), // sub_142550dc0
    Replacement::draft(0x142550e50, "unknown_142550e50", fn_142550e50 as *const ()), // sub_142550e50
    Replacement::draft(0x142550ee0, "unknown_142550ee0", fn_142550ee0 as *const ()), // sub_142550ee0
    Replacement::draft(0x142550f70, "unknown_142550f70", fn_142550f70 as *const ()), // sub_142550f70
    Replacement::draft(0x142550ff0, "unknown_142550ff0", fn_142550ff0 as *const ()), // sub_142550ff0
    Replacement::draft(0x142551070, "unknown_142551070", fn_142551070 as *const ()), // sub_142551070
    Replacement::draft(0x142551870, "unknown_142551870", fn_142551870 as *const ()), // sub_142551870
    Replacement::draft(0x1425519d0, "unknown_1425519d0", fn_1425519d0 as *const ()), // sub_1425519d0
    Replacement::draft(0x142551cd0, "unknown_142551cd0", fn_142551cd0 as *const ()), // sub_142551cd0
    Replacement::draft_throws(0x1425522b0, "unknown_1425522b0", fn_1425522b0 as *const (), Throws::Discard), // sub_1425522b0
    Replacement::draft_throws(0x142552330, "unknown_142552330", fn_142552330 as *const (), Throws::Discard), // sub_142552330
    Replacement::draft_throws(0x142552360, "unknown_142552360", fn_142552360 as *const (), Throws::Discard), // sub_142552360
    Replacement::draft_throws(0x142552390, "unknown_142552390", fn_142552390 as *const (), Throws::Discard), // sub_142552390
    Replacement::draft(0x142552d80, "unknown_142552d80", tpl_52692::<0x30, 0x1403d13f0, 0x38, 0x1403d13c0> as *const ()), // sub_142552d80
    Replacement::draft(0x142552db0, "unknown_142552db0", tpl_52693::<0x78, 0x1401de400, 0x1403d14c0, 0x30> as *const ()), // sub_142552db0
    Replacement::draft(0x142552de0, "unknown_142552de0", tpl_52693::<0x50, 0x1401de400, 0x1403d14c0, 0x58> as *const ()), // sub_142552de0
    Replacement::draft(0x1425532d0, "unknown_1425532d0", fn_1425532d0 as *const ()), // sub_1425532d0
    Replacement::draft_throws(0x1425538a0, "unknown_1425538a0", fn_1425538a0 as *const (), Throws::Discard), // sub_1425538a0
    Replacement::draft(0x1425541b0, "unknown_1425541b0", tpl_52652::<0x70, 0x1401de3c0, 0x60, 0x141955290> as *const ()), // sub_1425541b0
    Replacement::draft(0x1425541e0, "unknown_1425541e0", tpl_52619::<0x70, 0x1401ff400, 0x40, 0x1403e97d0> as *const ()), // sub_1425541e0
    Replacement::draft_throws(0x142554260, "unknown_142554260", fn_142554260 as *const (), Throws::Discard), // sub_142554260
    Replacement::draft(0x142554c80, "unknown_142554c80", tpl_52692::<0x40, 0x1401de3c0, 0x30, 0x1403f54d0> as *const ()), // sub_142554c80
    Replacement::draft(0x142554ce0, "unknown_142554ce0", fn_142554ce0 as *const ()), // sub_142554ce0
    Replacement::draft(0x142554d80, "unknown_142554d80", tpl_52697::<0x30, 0x1401de3c0, 0x1404112d0> as *const ()), // sub_142554d80
    Replacement::draft(0x142554e60, "unknown_142554e60", fn_142554e60 as *const ()), // sub_142554e60
    Replacement::draft(0x142554ee0, "unknown_142554ee0", fn_142554ee0 as *const ()), // sub_142554ee0
    Replacement::draft(0x142554f50, "unknown_142554f50", tpl_52697::<0x40, 0x1401de3c0, 0x1404112d0> as *const ()), // sub_142554f50
    Replacement::draft(0x142555910, "unknown_142555910", fn_142555910 as *const ()), // sub_142555910
    Replacement::draft(0x1425560b0, "unknown_1425560b0", fn_1425560b0 as *const ()), // sub_1425560b0
    Replacement::draft(0x1425566a0, "unknown_1425566a0", fn_1425566a0 as *const ()), // sub_1425566a0
    Replacement::draft(0x1425566c0, "unknown_1425566c0", fn_1425566c0 as *const ()), // sub_1425566c0
    Replacement::draft_throws(0x142556700, "unknown_142556700", fn_142556700 as *const (), Throws::Discard), // sub_142556700
    Replacement::draft(0x1425579a0, "unknown_1425579a0", fn_1425579a0 as *const ()), // sub_1425579a0
    Replacement::draft(0x1425579c0, "unknown_1425579c0", fn_1425579c0 as *const ()), // sub_1425579c0
    Replacement::draft_throws(0x142557a00, "unknown_142557a00", tpl_52706::<0x142442de0, 0x14183acce, 0x140498e8a> as *const (), Throws::Discard), // sub_142557a00
    Replacement::draft_throws(0x142557be0, "unknown_142557be0", tpl_52707::<0x142442de0, 0x14183acce, 0x14049d988> as *const (), Throws::Discard), // sub_142557be0
    Replacement::draft_throws(0x142557c10, "unknown_142557c10", tpl_52707::<0x142442de0, 0x14183acce, 0x14049dad8> as *const (), Throws::Discard), // sub_142557c10
    Replacement::draft_throws(0x142557c40, "unknown_142557c40", tpl_52707::<0x142442de0, 0x14183acce, 0x14049dc38> as *const (), Throws::Discard), // sub_142557c40
    Replacement::draft_throws(0x142557cd0, "unknown_142557cd0", tpl_52707::<0x142442de0, 0x14183acce, 0x14049df8e> as *const (), Throws::Discard), // sub_142557cd0
    Replacement::draft_throws(0x142557d00, "unknown_142557d00", tpl_52707::<0x142442de0, 0x14183acce, 0x14049e057> as *const (), Throws::Discard), // sub_142557d00
    Replacement::draft_throws(0x142557d70, "unknown_142557d70", tpl_52707::<0x142442de0, 0x14183acce, 0x14049e13e> as *const (), Throws::Discard), // sub_142557d70
    Replacement::draft_throws(0x142557e40, "unknown_142557e40", tpl_52706::<0x142442de0, 0x14183acce, 0x14049ec9d> as *const (), Throws::Discard), // sub_142557e40
    Replacement::draft_throws(0x142557e70, "unknown_142557e70", tpl_52706::<0x142442de0, 0x14183acce, 0x14049f137> as *const (), Throws::Discard), // sub_142557e70
    Replacement::draft_throws(0x142557f10, "unknown_142557f10", tpl_52707::<0x142442de0, 0x14183acce, 0x1404a0266> as *const (), Throws::Discard), // sub_142557f10
    Replacement::draft_throws(0x142558050, "unknown_142558050", fn_142558050 as *const (), Throws::Discard), // sub_142558050
    Replacement::draft_throws(0x142558080, "unknown_142558080", fn_142558080 as *const (), Throws::Discard), // sub_142558080
    Replacement::draft(0x1425582a0, "unknown_1425582a0", fn_1425582a0 as *const ()), // sub_1425582a0
    Replacement::draft(0x142558a10, "unknown_142558a10", fn_142558a10 as *const ()), // sub_142558a10
    Replacement::draft(0x142558c40, "unknown_142558c40", fn_142558c40 as *const ()), // sub_142558c40
    Replacement::draft(0x142559300, "unknown_142559300", fn_142559300 as *const ()), // sub_142559300
    Replacement::draft_throws(0x1425594c0, "unknown_1425594c0", fn_1425594c0 as *const (), Throws::Discard), // sub_1425594c0
    Replacement::draft_throws(0x14255a370, "unknown_14255a370", fn_14255a370 as *const (), Throws::Discard), // sub_14255a370
    Replacement::draft(0x14255a4d0, "unknown_14255a4d0", fn_14255a4d0 as *const ()), // sub_14255a4d0
    Replacement::draft_throws(0x14255a5d0, "unknown_14255a5d0", fn_14255a5d0 as *const (), Throws::Discard), // sub_14255a5d0
    Replacement::draft(0x14255abf0, "unknown_14255abf0", fn_14255abf0 as *const ()), // sub_14255abf0
    Replacement::draft_throws(0x14255ae20, "unknown_14255ae20", fn_14255ae20 as *const (), Throws::Discard), // sub_14255ae20
    Replacement::draft_throws(0x14255ae50, "unknown_14255ae50", fn_14255ae50 as *const (), Throws::Discard), // sub_14255ae50
    Replacement::draft_throws(0x14255ae80, "unknown_14255ae80", fn_14255ae80 as *const (), Throws::Discard), // sub_14255ae80
    Replacement::draft_throws(0x14255aeb0, "unknown_14255aeb0", fn_14255aeb0 as *const (), Throws::Discard), // sub_14255aeb0
    Replacement::draft(0x14255b8c0, "unknown_14255b8c0", fn_14255b8c0 as *const ()), // sub_14255b8c0
    Replacement::draft(0x14255bba0, "unknown_14255bba0", fn_14255bba0 as *const ()), // sub_14255bba0
    Replacement::draft_throws(0x14255c1f0, "unknown_14255c1f0", fn_14255c1f0 as *const (), Throws::Discard), // sub_14255c1f0
    Replacement::draft_throws(0x14255c320, "unknown_14255c320", fn_14255c320 as *const (), Throws::Discard), // sub_14255c320
    Replacement::draft_throws(0x14255c3c0, "unknown_14255c3c0", fn_14255c3c0 as *const (), Throws::Discard), // sub_14255c3c0
    Replacement::draft_throws(0x14255c480, "unknown_14255c480", fn_14255c480 as *const (), Throws::Discard), // sub_14255c480
    Replacement::draft(0x14255c5d0, "unknown_14255c5d0", fn_14255c5d0 as *const ()), // sub_14255c5d0
    Replacement::draft(0x14255c760, "unknown_14255c760", fn_14255c760 as *const ()), // sub_14255c760
    Replacement::draft_throws(0x14255cad0, "unknown_14255cad0", fn_14255cad0 as *const (), Throws::Discard), // sub_14255cad0
    Replacement::draft_throws(0x14255cb20, "unknown_14255cb20", fn_14255cb20 as *const (), Throws::Discard), // sub_14255cb20
    Replacement::draft(0x14255cc10, "unknown_14255cc10", tpl_52619::<0x78, 0x1401e1800, 0x40, 0x140654620> as *const ()), // sub_14255cc10
    Replacement::draft(0x14255cd00, "unknown_14255cd00", tpl_52659::<0x68, 0x1402467f0, 0x58, 0x1403b7290> as *const ()), // sub_14255cd00
    Replacement::draft(0x14255cd60, "unknown_14255cd60", fn_14255cd60 as *const ()), // sub_14255cd60
    Replacement::draft(0x14255cf90, "unknown_14255cf90", fn_14255cf90 as *const ()), // sub_14255cf90
    Replacement::draft_throws(0x14255d0f0, "unknown_14255d0f0", fn_14255d0f0 as *const (), Throws::Discard), // sub_14255d0f0
    Replacement::draft(0x14255d240, "unknown_14255d240", fn_14255d240 as *const ()), // sub_14255d240
    Replacement::draft(0x14255d3d0, "unknown_14255d3d0", fn_14255d3d0 as *const ()), // sub_14255d3d0
    Replacement::draft(0x14255d400, "unknown_14255d400", fn_14255d400 as *const ()), // sub_14255d400
    Replacement::draft(0x14255d4b0, "unknown_14255d4b0", tpl_52728::<0x348, 0x1401e7b10, 0x1401e7bc0> as *const ()), // sub_14255d4b0
    Replacement::draft(0x14255d4d0, "unknown_14255d4d0", fn_14255d4d0 as *const ()), // sub_14255d4d0
    Replacement::draft(0x14255d5b0, "unknown_14255d5b0", fn_14255d5b0 as *const ()), // sub_14255d5b0
    Replacement::draft(0x14255f830, "unknown_14255f830", fn_14255f830 as *const ()), // sub_14255f830
    Replacement::draft(0x14255f8b0, "unknown_14255f8b0", fn_14255f8b0 as *const ()), // sub_14255f8b0
    Replacement::draft(0x142560190, "unknown_142560190", fn_142560190 as *const ()), // sub_142560190
    Replacement::draft(0x142560ba0, "unknown_142560ba0", fn_142560ba0 as *const ()), // sub_142560ba0
    Replacement::draft(0x142560c30, "unknown_142560c30", fn_142560c30 as *const ()), // sub_142560c30
    Replacement::draft(0x142560e90, "unknown_142560e90", tpl_52736::<0x40, 0x14052f2f0, 0xf0, 0x14240ab10> as *const ()), // sub_142560e90
    Replacement::draft(0x142560ed0, "unknown_142560ed0", tpl_52736::<0x60, 0x14052f2f0, 0x210, 0x14240ab10> as *const ()), // sub_142560ed0
    Replacement::draft(0x142561440, "unknown_142561440", fn_142561440 as *const ()), // sub_142561440
    Replacement::draft(0x142562f40, "unknown_142562f40", fn_142562f40 as *const ()), // sub_142562f40
    Replacement::draft(0x142563010, "unknown_142563010", fn_142563010 as *const ()), // sub_142563010
    Replacement::draft(0x1425630a0, "unknown_1425630a0", fn_1425630a0 as *const ()), // sub_1425630a0
    Replacement::draft(0x142563150, "unknown_142563150", fn_142563150 as *const ()), // sub_142563150
    Replacement::draft(0x1425631e0, "unknown_1425631e0", fn_1425631e0 as *const ()), // sub_1425631e0
    Replacement::draft(0x142563270, "unknown_142563270", fn_142563270 as *const ()), // sub_142563270
    Replacement::draft(0x1425647b0, "unknown_1425647b0", tpl_52728::<0x50, 0x1401e7b10, 0x1401e7bc0> as *const ()), // sub_1425647b0
    Replacement::draft_throws(0x1425647d0, "unknown_1425647d0", fn_1425647d0 as *const (), Throws::Discard), // sub_1425647d0
    Replacement::draft(0x142564920, "unknown_142564920", fn_142564920 as *const ()), // sub_142564920
    Replacement::draft(0x14257ffe0, "unknown_14257ffe0", fn_14257ffe0 as *const ()), // sub_14257ffe0
    Replacement::draft(0x142580070, "unknown_142580070", fn_142580070 as *const ()), // sub_142580070
    Replacement::draft(0x1425800f0, "unknown_1425800f0", fn_1425800f0 as *const ()), // sub_1425800f0
    Replacement::draft(0x142580200, "unknown_142580200", fn_142580200 as *const ()), // sub_142580200
    Replacement::draft(0x142580300, "unknown_142580300", fn_142580300 as *const ()), // sub_142580300
    Replacement::draft_throws(0x142580430, "unknown_142580430", fn_142580430 as *const (), Throws::Discard), // sub_142580430
    Replacement::draft(0x142580840, "unknown_142580840", fn_142580840 as *const ()), // sub_142580840
    Replacement::draft(0x1425809c0, "unknown_1425809c0", fn_1425809c0 as *const ()), // sub_1425809c0
    Replacement::draft(0x142580e50, "unknown_142580e50", fn_142580e50 as *const ()), // sub_142580e50
    Replacement::draft(0x142580f30, "unknown_142580f30", fn_142580f30 as *const ()), // sub_142580f30
    Replacement::draft(0x142580fb0, "unknown_142580fb0", fn_142580fb0 as *const ()), // sub_142580fb0
    Replacement::draft(0x142581090, "unknown_142581090", fn_142581090 as *const ()), // sub_142581090
    Replacement::draft(0x142581270, "unknown_142581270", fn_142581270 as *const ()), // sub_142581270
    Replacement::draft(0x142581a10, "unknown_142581a10", fn_142581a10 as *const ()), // sub_142581a10
    Replacement::draft_throws(0x142582400, "unknown_142582400", fn_142582400 as *const (), Throws::Discard), // sub_142582400
    Replacement::draft_throws(0x1425824f0, "unknown_1425824f0", fn_1425824f0 as *const (), Throws::Discard), // sub_1425824f0
    Replacement::draft(0x1425826f0, "unknown_1425826f0", fn_1425826f0 as *const ()), // sub_1425826f0
    Replacement::draft(0x142582710, "unknown_142582710", fn_142582710 as *const ()), // sub_142582710
    Replacement::draft(0x1425827a0, "unknown_1425827a0", fn_1425827a0 as *const ()), // sub_1425827a0
    Replacement::draft(0x142582840, "unknown_142582840", fn_142582840 as *const ()), // sub_142582840
    Replacement::draft(0x1425828d0, "unknown_1425828d0", fn_1425828d0 as *const ()), // sub_1425828d0
    Replacement::draft(0x142582ce0, "unknown_142582ce0", fn_142582ce0 as *const ()), // sub_142582ce0
    Replacement::draft_throws(0x142583230, "unknown_142583230", fn_142583230 as *const (), Throws::Discard), // sub_142583230
    Replacement::draft(0x1425835f0, "unknown_1425835f0", fn_1425835f0 as *const ()), // sub_1425835f0
    Replacement::draft_throws(0x1425836a0, "unknown_1425836a0", fn_1425836a0 as *const (), Throws::Discard), // sub_1425836a0
    Replacement::draft(0x142583890, "unknown_142583890", fn_142583890 as *const ()), // sub_142583890
    Replacement::draft_throws(0x1425838c0, "unknown_1425838c0", fn_1425838c0 as *const (), Throws::Discard), // sub_1425838c0
    Replacement::draft_throws(0x142583940, "unknown_142583940", fn_142583940 as *const (), Throws::Discard), // sub_142583940
    Replacement::draft(0x142583a30, "unknown_142583a30", fn_142583a30 as *const ()), // sub_142583a30
    Replacement::draft(0x142583f60, "unknown_142583f60", fn_142583f60 as *const ()), // sub_142583f60
    Replacement::draft(0x142583fc0, "unknown_142583fc0", fn_142583fc0 as *const ()), // sub_142583fc0
    Replacement::draft(0x142584270, "unknown_142584270", fn_142584270 as *const ()), // sub_142584270
    Replacement::draft(0x142584410, "unknown_142584410", fn_142584410 as *const ()), // sub_142584410
    Replacement::draft(0x1425845e0, "unknown_1425845e0", fn_1425845e0 as *const ()), // sub_1425845e0
    Replacement::draft(0x142584850, "unknown_142584850", fn_142584850 as *const ()), // sub_142584850
    Replacement::draft(0x1425848b0, "unknown_1425848b0", fn_1425848b0 as *const ()), // sub_1425848b0
    Replacement::draft(0x142584920, "unknown_142584920", fn_142584920 as *const ()), // sub_142584920
    Replacement::draft(0x142584980, "unknown_142584980", fn_142584980 as *const ()), // sub_142584980
    Replacement::draft(0x1425849f0, "unknown_1425849f0", fn_1425849f0 as *const ()), // sub_1425849f0
    Replacement::draft_throws(0x142584ab0, "unknown_142584ab0", fn_142584ab0 as *const (), Throws::Discard), // sub_142584ab0
    Replacement::draft(0x142584b70, "unknown_142584b70", tpl_52778::<0x38, 0x142646d40> as *const ()), // sub_142584b70
    Replacement::draft(0x142584ba0, "unknown_142584ba0", tpl_52652::<0xb0, 0x1401de3c0, 0x20, 0x14026d670> as *const ()), // sub_142584ba0
    Replacement::draft_throws(0x142584c30, "unknown_142584c30", fn_142584c30 as *const (), Throws::Discard), // sub_142584c30
    Replacement::draft(0x142584c70, "unknown_142584c70", fn_142584c70 as *const ()), // sub_142584c70
    Replacement::draft_throws(0x142584ca0, "unknown_142584ca0", fn_142584ca0 as *const (), Throws::Discard), // sub_142584ca0
    Replacement::draft(0x142584f70, "unknown_142584f70", fn_142584f70 as *const ()), // sub_142584f70
    Replacement::draft(0x142584ff0, "unknown_142584ff0", fn_142584ff0 as *const ()), // sub_142584ff0
    Replacement::draft(0x142585040, "unknown_142585040", fn_142585040 as *const ()), // sub_142585040
    Replacement::draft_throws(0x1425853c0, "unknown_1425853c0", fn_1425853c0 as *const (), Throws::Discard), // sub_1425853c0
    Replacement::draft_throws(0x1425854e0, "unknown_1425854e0", fn_1425854e0 as *const (), Throws::Discard), // sub_1425854e0
    Replacement::draft(0x142585500, "unknown_142585500", fn_142585500 as *const ()), // sub_142585500
    Replacement::draft(0x142585660, "unknown_142585660", fn_142585660 as *const ()), // sub_142585660
    Replacement::draft_throws(0x142585690, "unknown_142585690", fn_142585690 as *const (), Throws::Discard), // sub_142585690
    Replacement::draft_throws(0x142585720, "unknown_142585720", fn_142585720 as *const (), Throws::Discard), // sub_142585720
    Replacement::draft_throws(0x142585750, "unknown_142585750", fn_142585750 as *const (), Throws::Discard), // sub_142585750
    Replacement::draft_throws(0x142585ab0, "unknown_142585ab0", fn_142585ab0 as *const (), Throws::Discard), // sub_142585ab0
    Replacement::draft_throws(0x142585b10, "unknown_142585b10", fn_142585b10 as *const (), Throws::Discard), // sub_142585b10
    Replacement::draft_throws(0x142585b40, "unknown_142585b40", fn_142585b40 as *const (), Throws::Discard), // sub_142585b40
    Replacement::draft_throws(0x142585b70, "unknown_142585b70", fn_142585b70 as *const (), Throws::Discard), // sub_142585b70
    Replacement::draft_throws(0x142585ba0, "unknown_142585ba0", fn_142585ba0 as *const (), Throws::Discard), // sub_142585ba0
    Replacement::draft_throws(0x142585bd0, "unknown_142585bd0", fn_142585bd0 as *const (), Throws::Discard), // sub_142585bd0
    Replacement::draft_throws(0x142585c00, "unknown_142585c00", fn_142585c00 as *const (), Throws::Discard), // sub_142585c00
    Replacement::draft_throws(0x142585cd0, "unknown_142585cd0", fn_142585cd0 as *const (), Throws::Discard), // sub_142585cd0
    Replacement::draft_throws(0x142585d00, "unknown_142585d00", fn_142585d00 as *const (), Throws::Discard), // sub_142585d00
    Replacement::draft_throws(0x142585d50, "unknown_142585d50", fn_142585d50 as *const (), Throws::Discard), // sub_142585d50
    Replacement::draft_throws(0x142585d80, "unknown_142585d80", fn_142585d80 as *const (), Throws::Discard), // sub_142585d80
    Replacement::draft_throws(0x142585e20, "unknown_142585e20", fn_142585e20 as *const (), Throws::Discard), // sub_142585e20
    Replacement::draft_throws(0x142585ea0, "unknown_142585ea0", fn_142585ea0 as *const (), Throws::Discard), // sub_142585ea0
    Replacement::draft_throws(0x142585f20, "unknown_142585f20", fn_142585f20 as *const (), Throws::Discard), // sub_142585f20
    Replacement::draft_throws(0x142585f50, "unknown_142585f50", fn_142585f50 as *const (), Throws::Discard), // sub_142585f50
    Replacement::draft(0x1425862f0, "unknown_1425862f0", tpl_52792::<0x50, 0x14104eb20, 0x60, 0x14104ed20> as *const ()), // sub_1425862f0
    Replacement::draft_throws(0x142586320, "unknown_142586320", fn_142586320 as *const (), Throws::Discard), // sub_142586320
    Replacement::draft(0x1425863b0, "unknown_1425863b0", fn_1425863b0 as *const ()), // sub_1425863b0
    Replacement::draft(0x142586420, "unknown_142586420", tpl_52794::<0x20, 0x1401de3c0, 0x40, 0x141060810> as *const ()), // sub_142586420
    Replacement::draft(0x142586470, "unknown_142586470", fn_142586470 as *const ()), // sub_142586470
    Replacement::draft(0x1425864f0, "unknown_1425864f0", fn_1425864f0 as *const ()), // sub_1425864f0
    Replacement::draft(0x1425865e0, "unknown_1425865e0", fn_1425865e0 as *const ()), // sub_1425865e0
    Replacement::draft(0x1425870d0, "unknown_1425870d0", fn_1425870d0 as *const ()), // sub_1425870d0
    Replacement::draft_throws(0x1425873c0, "unknown_1425873c0", fn_1425873c0 as *const (), Throws::Discard), // sub_1425873c0
    Replacement::draft_throws(0x142587690, "unknown_142587690", fn_142587690 as *const (), Throws::Discard), // sub_142587690
    Replacement::draft_throws(0x1425876c0, "unknown_1425876c0", fn_1425876c0 as *const (), Throws::Discard), // sub_1425876c0
    Replacement::draft_throws(0x1425876f0, "unknown_1425876f0", fn_1425876f0 as *const (), Throws::Discard), // sub_1425876f0
    Replacement::draft_throws(0x1425878f0, "unknown_1425878f0", fn_1425878f0 as *const (), Throws::Discard), // sub_1425878f0
    Replacement::draft(0x142587990, "unknown_142587990", tpl_52800::<0x58, 0x141fae860, 0x20, 0x141142360> as *const ()), // sub_142587990
    Replacement::draft(0x142587bc0, "unknown_142587bc0", fn_142587bc0 as *const ()), // sub_142587bc0
    Replacement::draft(0x142587c70, "unknown_142587c70", tpl_52802::<0x88, 0x1401de400, 0x90, 0x14026d680> as *const ()), // sub_142587c70
    Replacement::draft(0x142587d40, "unknown_142587d40", fn_142587d40 as *const ()), // sub_142587d40
    Replacement::draft(0x142587d70, "unknown_142587d70", fn_142587d70 as *const ()), // sub_142587d70
    Replacement::draft(0x142587dd0, "unknown_142587dd0", tpl_52792::<0x78, 0x14115ba50, 0x68, 0x140267c50> as *const ()), // sub_142587dd0
    Replacement::draft_throws(0x142587fc0, "unknown_142587fc0", fn_142587fc0 as *const (), Throws::Discard), // sub_142587fc0
    Replacement::draft(0x1425880c0, "unknown_1425880c0", tpl_52805::<0x70, 0x1401de3c0> as *const ()), // sub_1425880c0
    Replacement::draft_throws(0x1425881b0, "unknown_1425881b0", fn_1425881b0 as *const (), Throws::Discard), // sub_1425881b0
    Replacement::draft(0x1425881f0, "unknown_1425881f0", tpl_52802::<0xa0, 0x1401de3c0, 0xb0, 0x14028b740> as *const ()), // sub_1425881f0
    Replacement::draft(0x142588210, "unknown_142588210", tpl_52805::<0xb0, 0x1401de3c0> as *const ()), // sub_142588210
    Replacement::draft(0x142588240, "unknown_142588240", fn_142588240 as *const ()), // sub_142588240
    Replacement::draft_throws(0x142588280, "unknown_142588280", fn_142588280 as *const (), Throws::Discard), // sub_142588280
    Replacement::draft(0x1425882d0, "unknown_1425882d0", fn_1425882d0 as *const ()), // sub_1425882d0
    Replacement::draft_throws(0x142588420, "unknown_142588420", fn_142588420 as *const (), Throws::Discard), // sub_142588420
    Replacement::draft_throws(0x142588440, "unknown_142588440", fn_142588440 as *const (), Throws::Discard), // sub_142588440
    Replacement::draft_throws(0x142588640, "unknown_142588640", fn_142588640 as *const (), Throws::Discard), // sub_142588640
    Replacement::draft_throws(0x142588690, "unknown_142588690", fn_142588690 as *const (), Throws::Discard), // sub_142588690
    Replacement::draft_throws(0x1425886e0, "unknown_1425886e0", fn_1425886e0 as *const (), Throws::Discard), // sub_1425886e0
    Replacement::draft_throws(0x142588700, "unknown_142588700", fn_142588700 as *const (), Throws::Discard), // sub_142588700
    Replacement::draft(0x142588740, "unknown_142588740", fn_142588740 as *const ()), // sub_142588740
    Replacement::draft(0x1425887c0, "unknown_1425887c0", fn_1425887c0 as *const ()), // sub_1425887c0
    Replacement::draft(0x1425887f0, "unknown_1425887f0", fn_1425887f0 as *const ()), // sub_1425887f0
    Replacement::draft(0x142588820, "unknown_142588820", fn_142588820 as *const ()), // sub_142588820
    Replacement::draft_throws(0x142588860, "unknown_142588860", fn_142588860 as *const (), Throws::Discard), // sub_142588860
    Replacement::draft(0x142588890, "unknown_142588890", fn_142588890 as *const ()), // sub_142588890
    Replacement::draft_throws(0x1425888e0, "unknown_1425888e0", fn_1425888e0 as *const (), Throws::Discard), // sub_1425888e0
    Replacement::draft_throws(0x142588980, "unknown_142588980", fn_142588980 as *const (), Throws::Discard), // sub_142588980
    Replacement::draft(0x142588a40, "unknown_142588a40", tpl_52792::<0x40, 0x14104eb20, 0x50, 0x14104ed20> as *const ()), // sub_142588a40
    Replacement::draft(0x142588a70, "unknown_142588a70", fn_142588a70 as *const ()), // sub_142588a70
    Replacement::draft(0x142588aa0, "unknown_142588aa0", fn_142588aa0 as *const ()), // sub_142588aa0
    Replacement::draft(0x1425890e0, "unknown_1425890e0", fn_1425890e0 as *const ()), // sub_1425890e0
    Replacement::draft(0x142589120, "unknown_142589120", fn_142589120 as *const ()), // sub_142589120
    Replacement::draft(0x142589180, "unknown_142589180", fn_142589180 as *const ()), // sub_142589180
    Replacement::draft(0x142589240, "unknown_142589240", fn_142589240 as *const ()), // sub_142589240
    Replacement::draft(0x1425896b0, "unknown_1425896b0", fn_1425896b0 as *const ()), // sub_1425896b0
    Replacement::draft(0x142589740, "unknown_142589740", fn_142589740 as *const ()), // sub_142589740
    Replacement::draft(0x1425897f0, "unknown_1425897f0", tpl_52832::<0x30, 0x1411fb920, 0x270> as *const ()), // sub_1425897f0
    Replacement::draft(0x142589920, "unknown_142589920", tpl_52832::<0x70, 0x1411fb920, 0x260> as *const ()), // sub_142589920
    Replacement::draft(0x1425899b0, "unknown_1425899b0", tpl_52833::<0xf0, 0x1411fb920, 0x510, 0x141205a20> as *const ()), // sub_1425899b0
    Replacement::draft(0x142589c60, "unknown_142589c60", tpl_52833::<0x50, 0x1411fb920, 0x120, 0x14211b920> as *const ()), // sub_142589c60
    Replacement::draft(0x142589cc0, "unknown_142589cc0", tpl_52832::<0x60, 0x1411fb920, 0x410> as *const ()), // sub_142589cc0
    Replacement::draft(0x142589e50, "unknown_142589e50", fn_142589e50 as *const ()), // sub_142589e50
    Replacement::draft(0x142589f90, "unknown_142589f90", fn_142589f90 as *const ()), // sub_142589f90
    Replacement::draft(0x142589ff0, "unknown_142589ff0", tpl_52836::<0x38, 0x1411f9e70, 0x40, 0x1412323a0> as *const ()), // sub_142589ff0
    Replacement::draft(0x14258a070, "unknown_14258a070", fn_14258a070 as *const ()), // sub_14258a070
    Replacement::draft(0x14258a0a0, "unknown_14258a0a0", tpl_52800::<0x40, 0x1411fb6c0, 0x48, 0x141234140> as *const ()), // sub_14258a0a0
    Replacement::draft(0x14258a120, "unknown_14258a120", fn_14258a120 as *const ()), // sub_14258a120
    Replacement::draft(0x14258a180, "unknown_14258a180", fn_14258a180 as *const ()), // sub_14258a180
    Replacement::draft(0x14258a1a0, "unknown_14258a1a0", fn_14258a1a0 as *const ()), // sub_14258a1a0
    Replacement::draft(0x14258a1d0, "unknown_14258a1d0", fn_14258a1d0 as *const ()), // sub_14258a1d0
    Replacement::draft(0x14258a270, "unknown_14258a270", fn_14258a270 as *const ()), // sub_14258a270
    Replacement::draft(0x14258a540, "unknown_14258a540", fn_14258a540 as *const ()), // sub_14258a540
    Replacement::draft(0x14258a930, "unknown_14258a930", tpl_52844::<0x40, 0x1401de3c0> as *const ()), // sub_14258a930
    Replacement::draft(0x14258a950, "unknown_14258a950", tpl_52845::<0x40, 0x1401de3c0> as *const ()), // sub_14258a950
    Replacement::draft(0x14258aa00, "unknown_14258aa00", tpl_52844::<0x50, 0x1401de3c0> as *const ()), // sub_14258aa00
    Replacement::draft(0x14258aa20, "unknown_14258aa20", tpl_52845::<0x50, 0x1401de3c0> as *const ()), // sub_14258aa20
    Replacement::draft(0x14258ab20, "_public__virtual_int___cdecl_cmfctoolbar_14258ab20", tpl_52844::<0x60, 0x1401de3c0> as *const ()), // sub_14258ab20
    Replacement::draft(0x14258ab40, "_public__virtual_int___cdecl_cmfctoolbar_14258ab40", tpl_52845::<0x60, 0x1401de3c0> as *const ()), // sub_14258ab40
    Replacement::draft(0x14258abf0, "unknown_14258abf0", tpl_52844::<0x80, 0x1401de3c0> as *const ()), // sub_14258abf0
    Replacement::draft(0x14258ac10, "unknown_14258ac10", tpl_52845::<0x80, 0x1401de3c0> as *const ()), // sub_14258ac10
    Replacement::draft(0x14258acc0, "unknown_14258acc0", tpl_52844::<0x30, 0x1401de3c0> as *const ()), // sub_14258acc0
    Replacement::draft(0x14258ace0, "unknown_14258ace0", tpl_52845::<0x30, 0x1401de3c0> as *const ()), // sub_14258ace0
    Replacement::draft(0x14258aec0, "unknown_14258aec0", fn_14258aec0 as *const ()), // sub_14258aec0
    Replacement::draft(0x14258af20, "unknown_14258af20", fn_14258af20 as *const ()), // sub_14258af20
    Replacement::draft(0x14258b030, "unknown_14258b030", tpl_52848::<0x38, 0x1426458b0> as *const ()), // sub_14258b030
    Replacement::draft(0x14258b060, "unknown_14258b060", fn_14258b060 as *const ()), // sub_14258b060
    Replacement::draft(0x14258b090, "unknown_14258b090", tpl_52848::<0xf0, 0x1426458b0> as *const ()), // sub_14258b090
    Replacement::draft(0x14258b0b0, "unknown_14258b0b0", fn_14258b0b0 as *const ()), // sub_14258b0b0
    Replacement::draft(0x14258b100, "unknown_14258b100", fn_14258b100 as *const ()), // sub_14258b100
    Replacement::draft(0x14258b130, "unknown_14258b130", tpl_52778::<0xf8, 0x142645908> as *const ()), // sub_14258b130
    Replacement::draft(0x14258b160, "unknown_14258b160", fn_14258b160 as *const ()), // sub_14258b160
    Replacement::draft(0x14258b520, "unknown_14258b520", fn_14258b520 as *const ()), // sub_14258b520
    Replacement::draft(0x14258b540, "unknown_14258b540", tpl_52802::<0x140, 0x1401de400, 0x30, 0x1412a7c90> as *const ()), // sub_14258b540
    Replacement::draft(0x14258b580, "unknown_14258b580", fn_14258b580 as *const ()), // sub_14258b580
    Replacement::draft(0x14258c350, "unknown_14258c350", fn_14258c350 as *const ()), // sub_14258c350
    Replacement::draft(0x14258c380, "unknown_14258c380", fn_14258c380 as *const ()), // sub_14258c380
    Replacement::draft(0x14258c570, "unknown_14258c570", fn_14258c570 as *const ()), // sub_14258c570
    Replacement::draft(0x14258c5b0, "unknown_14258c5b0", fn_14258c5b0 as *const ()), // sub_14258c5b0
    Replacement::draft(0x14258c5f0, "unknown_14258c5f0", fn_14258c5f0 as *const ()), // sub_14258c5f0
    Replacement::draft(0x14258c770, "unknown_14258c770", fn_14258c770 as *const ()), // sub_14258c770
    Replacement::draft(0x14258c7a0, "unknown_14258c7a0", fn_14258c7a0 as *const ()), // sub_14258c7a0
    Replacement::draft_throws(0x14258c8b0, "unknown_14258c8b0", fn_14258c8b0 as *const (), Throws::Discard), // sub_14258c8b0
    Replacement::draft(0x14258c940, "unknown_14258c940", fn_14258c940 as *const ()), // sub_14258c940
    Replacement::draft(0x14258c9a0, "unknown_14258c9a0", fn_14258c9a0 as *const ()), // sub_14258c9a0
    Replacement::draft_throws(0x14258cd30, "unknown_14258cd30", fn_14258cd30 as *const (), Throws::Discard), // sub_14258cd30
    Replacement::draft_throws(0x14258cd60, "unknown_14258cd60", fn_14258cd60 as *const (), Throws::Discard), // sub_14258cd60
    Replacement::draft_throws(0x14258ce00, "unknown_14258ce00", fn_14258ce00 as *const (), Throws::Discard), // sub_14258ce00
    Replacement::draft_throws(0x14258ce30, "unknown_14258ce30", fn_14258ce30 as *const (), Throws::Discard), // sub_14258ce30
    Replacement::draft_throws(0x14258ce70, "unknown_14258ce70", fn_14258ce70 as *const (), Throws::Discard), // sub_14258ce70
    Replacement::draft(0x14258cee0, "unknown_14258cee0", fn_14258cee0 as *const ()), // sub_14258cee0
    Replacement::draft(0x14258cf10, "unknown_14258cf10", fn_14258cf10 as *const ()), // sub_14258cf10
    Replacement::draft_throws(0x14258cfc0, "unknown_14258cfc0", fn_14258cfc0 as *const (), Throws::Discard), // sub_14258cfc0
    Replacement::draft(0x14258d040, "unknown_14258d040", fn_14258d040 as *const ()), // sub_14258d040
    Replacement::draft_throws(0x14258d290, "unknown_14258d290", fn_14258d290 as *const (), Throws::Discard), // sub_14258d290
    Replacement::draft(0x14258d2c0, "unknown_14258d2c0", fn_14258d2c0 as *const ()), // sub_14258d2c0
    Replacement::draft(0x14258d3a0, "unknown_14258d3a0", tpl_52648::<0x50, 0x1401de400, 0x30, 0x1401e1800> as *const ()), // sub_14258d3a0
    Replacement::draft_throws(0x14258db40, "unknown_14258db40", fn_14258db40 as *const (), Throws::Discard), // sub_14258db40
    Replacement::draft(0x14258dbb0, "unknown_14258dbb0", fn_14258dbb0 as *const ()), // sub_14258dbb0
    Replacement::draft(0x14258dc60, "unknown_14258dc60", fn_14258dc60 as *const ()), // sub_14258dc60
    Replacement::draft(0x14258dfd0, "unknown_14258dfd0", fn_14258dfd0 as *const ()), // sub_14258dfd0
    Replacement::draft_throws(0x14258e0a0, "unknown_14258e0a0", fn_14258e0a0 as *const (), Throws::Discard), // sub_14258e0a0
    Replacement::draft(0x14258e110, "unknown_14258e110", fn_14258e110 as *const ()), // sub_14258e110
    Replacement::draft_throws(0x14258e130, "unknown_14258e130", fn_14258e130 as *const (), Throws::Discard), // sub_14258e130
    Replacement::draft(0x14258e1f0, "unknown_14258e1f0", tpl_52881::<0x20, 0x1413a43e0, 0x60, 0x1424954f0> as *const ()), // sub_14258e1f0
    Replacement::draft(0x14258e210, "unknown_14258e210", fn_14258e210 as *const ()), // sub_14258e210
    Replacement::draft(0x14258e260, "unknown_14258e260", fn_14258e260 as *const ()), // sub_14258e260
    Replacement::draft_throws(0x14258e6e0, "unknown_14258e6e0", fn_14258e6e0 as *const (), Throws::Discard), // sub_14258e6e0
    Replacement::draft(0x14258e780, "unknown_14258e780", fn_14258e780 as *const ()), // sub_14258e780
    Replacement::draft(0x14258e8d0, "unknown_14258e8d0", fn_14258e8d0 as *const ()), // sub_14258e8d0
    Replacement::draft_throws(0x14258ec40, "unknown_14258ec40", fn_14258ec40 as *const (), Throws::Discard), // sub_14258ec40
    Replacement::draft_throws(0x14258ec80, "unknown_14258ec80", fn_14258ec80 as *const (), Throws::Discard), // sub_14258ec80
    Replacement::draft_throws(0x14258edc0, "unknown_14258edc0", fn_14258edc0 as *const (), Throws::Discard), // sub_14258edc0
    Replacement::draft(0x142590bd0, "std_142590bd0", fn_142590bd0 as *const ()), // sub_142590bd0
    Replacement::draft(0x142590c90, "std_142590c90", fn_142590c90 as *const ()), // sub_142590c90
    Replacement::draft(0x1425919b0, "std_1425919b0", fn_1425919b0 as *const ()), // sub_1425919b0
    Replacement::draft(0x142591bf0, "std_142591bf0", fn_142591bf0 as *const ()), // sub_142591bf0
    Replacement::draft(0x142593590, "unknown_142593590", fn_142593590 as *const ()), // sub_142593590
    Replacement::draft(0x1425935d0, "unknown_1425935d0", fn_1425935d0 as *const ()), // sub_1425935d0
    Replacement::draft(0x142593610, "unknown_142593610", fn_142593610 as *const ()), // sub_142593610
    Replacement::draft(0x1425936b0, "unknown_1425936b0", fn_1425936b0 as *const ()), // sub_1425936b0
    Replacement::draft_throws(0x1425936e0, "unknown_1425936e0", fn_1425936e0 as *const (), Throws::Discard), // sub_1425936e0
    Replacement::draft_throws(0x142593730, "unknown_142593730", fn_142593730 as *const (), Throws::Discard), // sub_142593730
    Replacement::draft_throws(0x142593760, "unknown_142593760", fn_142593760 as *const (), Throws::Discard), // sub_142593760
    Replacement::draft_throws(0x142593790, "unknown_142593790", fn_142593790 as *const (), Throws::Discard), // sub_142593790
    Replacement::draft_throws(0x1425937c0, "unknown_1425937c0", fn_1425937c0 as *const (), Throws::Discard), // sub_1425937c0
    Replacement::draft_throws(0x1425937f0, "unknown_1425937f0", fn_1425937f0 as *const (), Throws::Discard), // sub_1425937f0
    Replacement::draft_throws(0x142593820, "unknown_142593820", fn_142593820 as *const (), Throws::Discard), // sub_142593820
    Replacement::draft_throws(0x1425938a0, "unknown_1425938a0", fn_1425938a0 as *const (), Throws::Discard), // sub_1425938a0
    Replacement::draft(0x142593a70, "unknown_142593a70", tpl_52778::<0x60, 0x142646cf0> as *const ()), // sub_142593a70
    Replacement::draft_throws(0x142593af0, "unknown_142593af0", fn_142593af0 as *const (), Throws::Discard), // sub_142593af0
    Replacement::draft(0x142593b50, "unknown_142593b50", fn_142593b50 as *const ()), // sub_142593b50
    Replacement::draft_throws(0x142593d40, "unknown_142593d40", fn_142593d40 as *const (), Throws::Discard), // sub_142593d40
    Replacement::draft(0x142593e00, "unknown_142593e00", tpl_52778::<0x90, 0x142645908> as *const ()), // sub_142593e00
    Replacement::draft(0x142593e10, "unknown_142593e10", fn_142593e10 as *const ()), // sub_142593e10
    Replacement::draft(0x142593e40, "unknown_142593e40", fn_142593e40 as *const ()), // sub_142593e40
    Replacement::draft(0x142593e60, "unknown_142593e60", fn_142593e60 as *const ()), // sub_142593e60
    Replacement::draft(0x142593e90, "unknown_142593e90", tpl_52778::<0x140, 0x142645908> as *const ()), // sub_142593e90
    Replacement::draft(0x142593fc0, "unknown_142593fc0", fn_142593fc0 as *const ()), // sub_142593fc0
    Replacement::draft_throws(0x142594020, "unknown_142594020", fn_142594020 as *const (), Throws::Discard), // sub_142594020
    Replacement::draft_throws(0x142594060, "unknown_142594060", fn_142594060 as *const (), Throws::Discard), // sub_142594060
    Replacement::draft_throws(0x1425940a0, "unknown_1425940a0", fn_1425940a0 as *const (), Throws::Discard), // sub_1425940a0
    Replacement::draft_throws(0x1425940e0, "unknown_1425940e0", fn_1425940e0 as *const (), Throws::Discard), // sub_1425940e0
    Replacement::draft_throws(0x1425944e5, "unknown_1425944e5", fn_1425944e5 as *const (), Throws::Discard), // sub_1425944e5
    Replacement::draft(0x1425945bf, "unknown_1425945bf", fn_1425945bf as *const ()), // sub_1425945bf
    Replacement::draft_throws(0x142594717, "unknown_142594717", fn_142594717 as *const (), Throws::Discard), // sub_142594717
    Replacement::draft(0x142594753, "unknown_142594753", fn_142594753 as *const ()), // sub_142594753
    Replacement::draft_throws(0x1425949d8, "unknown_1425949d8", fn_1425949d8 as *const (), Throws::Discard), // sub_1425949d8
    Replacement::draft_throws(0x142594a00, "unknown_142594a00", fn_142594a00 as *const (), Throws::Discard), // sub_142594a00
    Replacement::draft(0x142594b38, "unknown_142594b38", fn_142594b38 as *const ()), // sub_142594b38
    Replacement::draft_throws(0x14259524c, "unknown_14259524c", fn_14259524c as *const (), Throws::Caught), // sub_14259524c
    Replacement::draft_throws(0x1425952ee, "unknown_1425952ee", fn_1425952ee as *const (), Throws::Discard), // sub_1425952ee
    Replacement::draft_throws(0x142595462, "global_142595462", fn_142595462 as *const (), Throws::Caught), // sub_142595462
    Replacement::draft_throws(0x1425954bc, "global_1425954bc", fn_1425954bc as *const (), Throws::Caught), // sub_1425954bc
    Replacement::draft(0x14259550b, "global_14259550b", fn_14259550b as *const ()), // sub_14259550b
    Replacement::draft_throws(0x14259554d, "global_14259554d", fn_14259554d as *const (), Throws::Caught), // sub_14259554d
    Replacement::draft(0x142595656, "global_142595656", fn_142595656 as *const ()), // sub_142595656
    Replacement::draft_throws(0x142595676, "global_142595676", fn_142595676 as *const (), Throws::Discard), // sub_142595676
    Replacement::draft_throws(0x1425956bb, "global_1425956bb", fn_1425956bb as *const (), Throws::Discard), // sub_1425956bb
    Replacement::draft_throws(0x142595778, "global_142595778", fn_142595778 as *const (), Throws::Discard), // sub_142595778
    Replacement::draft_throws(0x1425957a8, "global_1425957a8", fn_1425957a8 as *const (), Throws::Discard), // sub_1425957a8
    Replacement::draft_throws(0x142595c7c, "global_142595c7c", fn_142595c7c as *const (), Throws::Discard), // sub_142595c7c
    Replacement::draft_throws(0x142595d34, "global_142595d34", fn_142595d34 as *const (), Throws::Discard), // sub_142595d34
    Replacement::draft_throws(0x142595db0, "global_142595db0", fn_142595db0 as *const (), Throws::Discard), // sub_142595db0
    Replacement::draft(0x142595e2c, "global_142595e2c", fn_142595e2c as *const ()), // sub_142595e2c
    Replacement::draft(0x142595e58, "global_142595e58", fn_142595e58 as *const ()), // sub_142595e58
    Replacement::draft_throws(0x14259617e, "global_14259617e", fn_14259617e as *const (), Throws::Discard), // sub_14259617e
    Replacement::draft(0x1425964b0, "unknown_1425964b0", tpl_52609::<0x20, 0x1414dfaf0> as *const ()), // sub_1425964b0
    Replacement::draft(0x1425964c0, "unknown_1425964c0", tpl_52609::<0xc0, 0x140203010> as *const ()), // sub_1425964c0
    Replacement::draft_throws(0x1425964d0, "unknown_1425964d0", tpl_52618::<0x100, 0x1414e1d70> as *const (), Throws::Discard), // sub_1425964d0
    Replacement::draft_throws(0x1425964e0, "unknown_1425964e0", tpl_52618::<0x38, 0x1414e1600> as *const (), Throws::Discard), // sub_1425964e0
    Replacement::draft(0x1425964f0, "unknown_1425964f0", tpl_52609::<0x88, 0x1413dbb50> as *const ()), // sub_1425964f0
    Replacement::draft(0x142596500, "unknown_142596500", tpl_52609::<0x68, 0x1413db2a0> as *const ()), // sub_142596500
    Replacement::draft(0x142596510, "unknown_142596510", tpl_52609::<0x58, 0x1413db2a0> as *const ()), // sub_142596510
    Replacement::draft(0x142596520, "unknown_142596520", tpl_52609::<0xe8, 0x1413dbb50> as *const ()), // sub_142596520
    Replacement::draft(0x142596530, "unknown_142596530", tpl_52609::<0xd8, 0x1413db2a0> as *const ()), // sub_142596530
    Replacement::draft(0x142596540, "unknown_142596540", tpl_52609::<0x190, 0x1413dbb50> as *const ()), // sub_142596540
    Replacement::draft(0x142596550, "unknown_142596550", tpl_52609::<0xc8, 0x1413db2a0> as *const ()), // sub_142596550
    Replacement::draft(0x142596560, "unknown_142596560", tpl_52609::<0x1a8, 0x1413dbb50> as *const ()), // sub_142596560
    Replacement::draft(0x142596570, "unknown_142596570", tpl_52609::<0x1c0, 0x1413dbb50> as *const ()), // sub_142596570
    Replacement::draft(0x142596580, "unknown_142596580", tpl_52609::<0x130, 0x1413db2a0> as *const ()), // sub_142596580
    Replacement::draft(0x142596590, "unknown_142596590", tpl_52609::<0x1d8, 0x1413dbb50> as *const ()), // sub_142596590
    Replacement::draft(0x1425965a0, "unknown_1425965a0", tpl_52609::<0x1f0, 0x1413dbb50> as *const ()), // sub_1425965a0
    Replacement::draft(0x1425965b0, "unknown_1425965b0", fn_1425965b0 as *const ()), // sub_1425965b0
    Replacement::draft(0x1425965e0, "unknown_1425965e0", tpl_52609::<0x20, 0x1413dbb50> as *const ()), // sub_1425965e0
    Replacement::draft(0x1425965f0, "unknown_1425965f0", tpl_52609::<0x140, 0x1414e1590> as *const ()), // sub_1425965f0
    Replacement::draft(0x142596600, "unknown_142596600", tpl_52609::<0x150, 0x1414e1590> as *const ()), // sub_142596600
    Replacement::draft(0x142596610, "unknown_142596610", tpl_52609::<0x30, 0x1413db2a0> as *const ()), // sub_142596610
    Replacement::draft(0x142596620, "unknown_142596620", fn_142596620 as *const ()), // sub_142596620
    Replacement::draft(0x142596650, "unknown_142596650", tpl_52609::<0x20, 0x1413db2e0> as *const ()), // sub_142596650
    Replacement::draft(0x142596660, "unknown_142596660", fn_142596660 as *const ()), // sub_142596660
    Replacement::draft(0x142596680, "unknown_142596680", tpl_52609::<0x198, 0x1413db2a0> as *const ()), // sub_142596680
    Replacement::draft(0x142596690, "unknown_142596690", tpl_52609::<0x168, 0x1413dbb50> as *const ()), // sub_142596690
    Replacement::draft(0x1425966a0, "unknown_1425966a0", tpl_52609::<0x158, 0x1413db2a0> as *const ()), // sub_1425966a0
    Replacement::draft(0x1425966b0, "unknown_1425966b0", fn_1425966b0 as *const ()), // sub_1425966b0
    Replacement::draft(0x1425966e0, "unknown_1425966e0", tpl_52609::<0x30, 0x1413db890> as *const ()), // sub_1425966e0
    Replacement::draft(0x1425966f0, "unknown_1425966f0", tpl_52609::<0x120, 0x1413dbb50> as *const ()), // sub_1425966f0
    Replacement::draft(0x142596700, "unknown_142596700", tpl_52609::<0x360, 0x1413dbb50> as *const ()), // sub_142596700
    Replacement::draft(0x142596710, "unknown_142596710", tpl_52609::<0x240, 0x1413db2b0> as *const ()), // sub_142596710
    Replacement::draft(0x142596720, "unknown_142596720", tpl_52609::<0xd0, 0x1413dbb50> as *const ()), // sub_142596720
    Replacement::draft(0x142596730, "unknown_142596730", tpl_52609::<0x278, 0x1413db2b0> as *const ()), // sub_142596730
    Replacement::draft(0x142596740, "unknown_142596740", tpl_52609::<0xa0, 0x1413dbb50> as *const ()), // sub_142596740
    Replacement::draft(0x142596750, "unknown_142596750", tpl_52609::<0x2b0, 0x1413db2b0> as *const ()), // sub_142596750
    Replacement::draft(0x142596760, "unknown_142596760", tpl_52609::<0xb8, 0x1413dbb50> as *const ()), // sub_142596760
    Replacement::draft(0x142596770, "unknown_142596770", fn_142596770 as *const ()), // sub_142596770
    Replacement::draft(0x1425967a0, "unknown_1425967a0", tpl_52609::<0xe8, 0x1413db2e0> as *const ()), // sub_1425967a0
    Replacement::draft(0x1425967b0, "unknown_1425967b0", tpl_52609::<0x138, 0x1413dbb50> as *const ()), // sub_1425967b0
    Replacement::draft(0x1425967c0, "unknown_1425967c0", fn_1425967c0 as *const ()), // sub_1425967c0
    Replacement::draft(0x1425967f0, "unknown_1425967f0", tpl_52609::<0xf8, 0x1413db2e0> as *const ()), // sub_1425967f0
    Replacement::draft(0x142596800, "unknown_142596800", tpl_52609::<0x48, 0x1413dbb50> as *const ()), // sub_142596800
    Replacement::draft(0x142596810, "unknown_142596810", tpl_52609::<0x1e0, 0x1413db2a0> as *const ()), // sub_142596810
    Replacement::draft(0x142596820, "unknown_142596820", fn_142596820 as *const ()), // sub_142596820
    Replacement::draft(0x142596850, "unknown_142596850", tpl_52609::<0x3a0, 0x1413db890> as *const ()), // sub_142596850
    Replacement::draft(0x142596860, "unknown_142596860", tpl_52609::<0x378, 0x1413dbb50> as *const ()), // sub_142596860
    Replacement::draft(0x142596870, "unknown_142596870", tpl_52609::<0x1f0, 0x1413db2a0> as *const ()), // sub_142596870
    Replacement::draft(0x142596880, "unknown_142596880", tpl_52609::<0x108, 0x1413dbb50> as *const ()), // sub_142596880
    Replacement::draft(0x142596890, "unknown_142596890", tpl_52609::<0x28, 0x140203010> as *const ()), // sub_142596890
    Replacement::draft(0x1425968a0, "unknown_1425968a0", tpl_52609::<0x200, 0x1414e15b0> as *const ()), // sub_1425968a0
    Replacement::draft(0x1425968b0, "unknown_1425968b0", fn_1425968b0 as *const ()), // sub_1425968b0
    Replacement::draft(0x1425968d0, "unknown_1425968d0", tpl_52609::<0x358, 0x1413dc960> as *const ()), // sub_1425968d0
    Replacement::draft(0x1425968e0, "unknown_1425968e0", tpl_52609::<0x210, 0x1413dd120> as *const ()), // sub_1425968e0
    Replacement::draft(0x1425968f0, "unknown_1425968f0", fn_1425968f0 as *const ()), // sub_1425968f0
    Replacement::draft(0x142596910, "unknown_142596910", tpl_52609::<0x320, 0x140203130> as *const ()), // sub_142596910
    Replacement::draft(0x142596920, "unknown_142596920", tpl_52609::<0x390, 0x1414e15b0> as *const ()), // sub_142596920
    Replacement::draft(0x142596930, "unknown_142596930", tpl_52609::<0x60, 0x1414deaa0> as *const ()), // sub_142596930
    Replacement::draft(0x142596940, "unknown_142596940", tpl_52609::<0x60, 0x1401de410> as *const ()), // sub_142596940
    Replacement::draft(0x142596950, "unknown_142596950", fn_142596950 as *const ()), // sub_142596950
    Replacement::draft(0x142596970, "unknown_142596970", tpl_52609::<0x20, 0x140200d90> as *const ()), // sub_142596970
    Replacement::draft(0x142596980, "unknown_142596980", fn_142596980 as *const ()), // sub_142596980
    Replacement::draft(0x1425969a0, "unknown_1425969a0", tpl_52609::<0x30, 0x141498854> as *const ()), // sub_1425969a0
    Replacement::draft(0x1425969b0, "unknown_1425969b0", tpl_52609::<0xc0, 0x14149d88c> as *const ()), // sub_1425969b0
    Replacement::draft(0x1425969c0, "unknown_1425969c0", tpl_52607::<0xd8, 0x1401e1560> as *const ()), // sub_1425969c0
    Replacement::draft(0x1425969d0, "unknown_1425969d0", tpl_52609::<0x38, 0x1414e1d50> as *const ()), // sub_1425969d0
    Replacement::draft(0x1425969e0, "unknown_1425969e0", tpl_52609::<0x160, 0x14149d88c> as *const ()), // sub_1425969e0
    Replacement::draft(0x1425969f0, "unknown_1425969f0", tpl_52609::<0x150, 0x140203010> as *const ()), // sub_1425969f0
    Replacement::draft(0x142596a00, "unknown_142596a00", tpl_52609::<0x168, 0x140203010> as *const ()), // sub_142596a00
    Replacement::draft(0x142596a10, "unknown_142596a10", tpl_52609::<0xb8, 0x140203010> as *const ()), // sub_142596a10
    Replacement::draft(0x142596a20, "unknown_142596a20", tpl_52609::<0x20, 0x1414dfbd0> as *const ()), // sub_142596a20
    Replacement::draft(0x142596a30, "unknown_142596a30", tpl_52609::<0x20, 0x1414dfbf0> as *const ()), // sub_142596a30
    Replacement::draft(0x142596a40, "unknown_142596a40", tpl_52609::<0x90, 0x141498854> as *const ()), // sub_142596a40
    Replacement::draft(0x142596a50, "unknown_142596a50", tpl_52609::<0x80, 0x141498854> as *const ()), // sub_142596a50
    Replacement::draft(0x142596a60, "unknown_142596a60", tpl_52609::<0x70, 0x141498854> as *const ()), // sub_142596a60
    Replacement::draft_throws(0x142596a70, "unknown_142596a70", tpl_52618::<0xf0, 0x1414e1d70> as *const (), Throws::Discard), // sub_142596a70
    Replacement::draft_throws(0x142596a80, "unknown_142596a80", tpl_52618::<0x50, 0x1414e1600> as *const (), Throws::Discard), // sub_142596a80
    Replacement::draft(0x142596a90, "unknown_142596a90", tpl_52609::<0xb8, 0x1413db2a0> as *const ()), // sub_142596a90
    Replacement::draft(0x142596aa0, "unknown_142596aa0", tpl_52609::<0xa8, 0x1413db2a0> as *const ()), // sub_142596aa0
    Replacement::draft(0x142596ab0, "unknown_142596ab0", fn_142596ab0 as *const ()), // sub_142596ab0
    Replacement::draft(0x142596ae0, "unknown_142596ae0", tpl_52609::<0x200, 0x1413db890> as *const ()), // sub_142596ae0
    Replacement::draft(0x142596af0, "unknown_142596af0", tpl_52609::<0x1c8, 0x1414e1590> as *const ()), // sub_142596af0
    Replacement::draft(0x142596b00, "unknown_142596b00", tpl_52609::<0x180, 0x1414e1590> as *const ()), // sub_142596b00
    Replacement::draft(0x142596b10, "ak_142596b10", fn_142596b10 as *const ()), // sub_142596b10
    Replacement::draft(0x142596b40, "unknown_142596b40", tpl_52607::<0x38, 0x1414dd260> as *const ()), // sub_142596b40
    Replacement::draft(0x142596b50, "unknown_142596b50", tpl_52942::<0x40, 0x141512ea0> as *const ()), // sub_142596b50
    Replacement::draft(0x142596b70, "unknown_142596b70", fn_142596b70 as *const ()), // sub_142596b70
    Replacement::draft(0x142596c40, "unknown_142596c40", tpl_52609::<0x30, 0x14169e2e0> as *const ()), // sub_142596c40
    Replacement::draft(0x142596db0, "unknown_142596db0", fn_142596db0 as *const ()), // sub_142596db0
    Replacement::draft(0x142596e70, "unknown_142596e70", fn_142596e70 as *const ()), // sub_142596e70
    Replacement::draft(0x142596fa0, "unknown_142596fa0", fn_142596fa0 as *const ()), // sub_142596fa0
    Replacement::draft(0x142597040, "unknown_142597040", fn_142597040 as *const ()), // sub_142597040
    Replacement::draft(0x1425970c0, "unknown_1425970c0", fn_1425970c0 as *const ()), // sub_1425970c0
    Replacement::draft(0x142597480, "unknown_142597480", fn_142597480 as *const ()), // sub_142597480
    Replacement::draft(0x1425976d0, "unknown_1425976d0", fn_1425976d0 as *const ()), // sub_1425976d0
    Replacement::draft(0x1425976f0, "unknown_1425976f0", fn_1425976f0 as *const ()), // sub_1425976f0
    Replacement::draft(0x1425978e0, "unknown_1425978e0", tpl_52952::<0x38, 0x1411f9e70, 0x60, 0x1416abf10> as *const ()), // sub_1425978e0
    Replacement::draft(0x142597a10, "unknown_142597a10", tpl_52832::<0x40, 0x1411fb920, 0x1c0> as *const ()), // sub_142597a10
    Replacement::draft(0x142597bb0, "unknown_142597bb0", tpl_52833::<0x80, 0x1411fb920, 0x40, 0x1416cab10> as *const ()), // sub_142597bb0
    Replacement::draft(0x142597be0, "unknown_142597be0", fn_142597be0 as *const ()), // sub_142597be0
    Replacement::draft(0x142597ea0, "unknown_142597ea0", tpl_52881::<0xd0, 0x1411fb920, 0x58, 0x1411fbeb0> as *const ()), // sub_142597ea0
    Replacement::draft(0x1425983d0, "unknown_1425983d0", tpl_52792::<0x30, 0x1411fa150, 0x38, 0x1412267a0> as *const ()), // sub_1425983d0
    Replacement::draft(0x142598570, "unknown_142598570", fn_142598570 as *const ()), // sub_142598570
    Replacement::draft(0x142598810, "unknown_142598810", fn_142598810 as *const ()), // sub_142598810
    Replacement::draft(0x142598850, "unknown_142598850", fn_142598850 as *const ()), // sub_142598850
    Replacement::draft(0x142598a50, "unknown_142598a50", fn_142598a50 as *const ()), // sub_142598a50
    Replacement::draft(0x142598ac0, "unknown_142598ac0", fn_142598ac0 as *const ()), // sub_142598ac0
    Replacement::draft(0x142598af0, "unknown_142598af0", fn_142598af0 as *const ()), // sub_142598af0
    Replacement::draft(0x142598c10, "unknown_142598c10", fn_142598c10 as *const ()), // sub_142598c10
    Replacement::draft(0x142598d10, "unknown_142598d10", tpl_52836::<0x60, 0x1411f9e70, 0x68, 0x1412323a0> as *const ()), // sub_142598d10
    Replacement::draft(0x142598e40, "unknown_142598e40", tpl_52609::<0xb8, 0x14169e2e0> as *const ()), // sub_142598e40
    Replacement::draft(0x142598e50, "unknown_142598e50", tpl_52609::<0xa8, 0x14169e2e0> as *const ()), // sub_142598e50
    Replacement::draft(0x142598e60, "unknown_142598e60", tpl_52609::<0x78, 0x1417457f0> as *const ()), // sub_142598e60
    Replacement::draft(0x142598e70, "unknown_142598e70", tpl_52609::<0x50, 0x1417457f0> as *const ()), // sub_142598e70
    Replacement::draft(0x142598e80, "unknown_142598e80", tpl_52607::<0x68, 0x1411fb050> as *const ()), // sub_142598e80
    Replacement::draft(0x142598e90, "unknown_142598e90", tpl_52607::<0x30, 0x14171f1b0> as *const ()), // sub_142598e90
    Replacement::draft(0x142598ea0, "unknown_142598ea0", tpl_52607::<0x48, 0x1411f9e60> as *const ()), // sub_142598ea0
    Replacement::draft(0x142598eb0, "unknown_142598eb0", tpl_52619::<0x48, 0x1411f9db0, 0x60, 0x141747160> as *const ()), // sub_142598eb0
    Replacement::draft(0x142598ed0, "unknown_142598ed0", tpl_52609::<0x60, 0x141200250> as *const ()), // sub_142598ed0
    Replacement::draft(0x142598ee0, "unknown_142598ee0", tpl_52609::<0x88, 0x1411fc280> as *const ()), // sub_142598ee0
    Replacement::draft(0x142598ef0, "unknown_142598ef0", tpl_52609::<0x78, 0x1411fc280> as *const ()), // sub_142598ef0
    Replacement::draft(0x142598f00, "unknown_142598f00", tpl_52607::<0x30, 0x1411ffeb0> as *const ()), // sub_142598f00
    Replacement::draft(0x142598f10, "unknown_142598f10", fn_142598f10 as *const ()), // sub_142598f10
    Replacement::draft(0x142598f40, "unknown_142598f40", tpl_52609::<0x80, 0x141201d40> as *const ()), // sub_142598f40
    Replacement::draft(0x142598f50, "unknown_142598f50", tpl_52609::<0x48, 0x14174b510> as *const ()), // sub_142598f50
    Replacement::draft(0x142598f60, "unknown_142598f60", tpl_52609::<0x90, 0x1417580e0> as *const ()), // sub_142598f60
    Replacement::draft(0x142598f70, "unknown_142598f70", tpl_52609::<0x108, 0x141201d40> as *const ()), // sub_142598f70
    Replacement::draft(0x142598f80, "unknown_142598f80", tpl_52609::<0x68, 0x1416bb6c0> as *const ()), // sub_142598f80
    Replacement::draft(0x142598f90, "unknown_142598f90", tpl_52607::<0x178, 0x1411f9e90> as *const ()), // sub_142598f90
    Replacement::draft(0x142598fa0, "unknown_142598fa0", tpl_52609::<0x178, 0x1411fa420> as *const ()), // sub_142598fa0
    Replacement::draft(0x142598fb0, "unknown_142598fb0", tpl_52607::<0x100, 0x1416bb5e0> as *const ()), // sub_142598fb0
    Replacement::draft(0x142598fc0, "unknown_142598fc0", tpl_52609::<0x38, 0x141201d40> as *const ()), // sub_142598fc0
    Replacement::draft(0x142598fd0, "unknown_142598fd0", tpl_52609::<0xa8, 0x141201d40> as *const ()), // sub_142598fd0
    Replacement::draft(0x142598fe0, "unknown_142598fe0", tpl_52609::<0x58, 0x1411fb920> as *const ()), // sub_142598fe0
    Replacement::draft(0x142598ff0, "unknown_142598ff0", tpl_52609::<0xb8, 0x1411fb920> as *const ()), // sub_142598ff0
    Replacement::draft(0x142599000, "unknown_142599000", tpl_52609::<0x128, 0x141201d40> as *const ()), // sub_142599000
    Replacement::draft(0x142599010, "unknown_142599010", tpl_52609::<0xf8, 0x141201d40> as *const ()), // sub_142599010
    Replacement::draft(0x142599020, "unknown_142599020", tpl_52609::<0x80, 0x1411f9e70> as *const ()), // sub_142599020
    Replacement::draft(0x142599030, "unknown_142599030", tpl_52609::<0x98, 0x141201d40> as *const ()), // sub_142599030
    Replacement::draft(0x142599040, "unknown_142599040", tpl_52609::<0x30, 0x1411fb920> as *const ()), // sub_142599040
    Replacement::draft(0x142599050, "unknown_142599050", tpl_52609::<0x78, 0x1411fb920> as *const ()), // sub_142599050
    Replacement::draft(0x142599060, "unknown_142599060", tpl_52609::<0x28, 0x141201d40> as *const ()), // sub_142599060
    Replacement::draft(0x142599070, "unknown_142599070", tpl_52609::<0x30, 0x141201d40> as *const ()), // sub_142599070
    Replacement::draft(0x142599080, "unknown_142599080", fn_142599080 as *const ()), // sub_142599080
    Replacement::draft(0x1425990a0, "unknown_1425990a0", tpl_52609::<0xc0, 0x1416a6700> as *const ()), // sub_1425990a0
    Replacement::draft(0x1425990b0, "unknown_1425990b0", tpl_52609::<0x150, 0x1416a6350> as *const ()), // sub_1425990b0
    Replacement::draft(0x1425990c0, "unknown_1425990c0", tpl_52607::<0x2c0, 0x1411f9e90> as *const ()), // sub_1425990c0
    Replacement::draft(0x1425990d0, "unknown_1425990d0", tpl_52609::<0x2c0, 0x1411fa420> as *const ()), // sub_1425990d0
    Replacement::draft(0x1425990e0, "unknown_1425990e0", fn_1425990e0 as *const ()), // sub_1425990e0
    Replacement::draft(0x142599140, "unknown_142599140", tpl_52607::<0x40, 0x1411ffeb0> as *const ()), // sub_142599140
    Replacement::draft(0x142599150, "unknown_142599150", fn_142599150 as *const ()), // sub_142599150
    Replacement::draft(0x142599180, "unknown_142599180", fn_142599180 as *const ()), // sub_142599180
    Replacement::draft(0x1425991a0, "unknown_1425991a0", fn_1425991a0 as *const ()), // sub_1425991a0
    Replacement::draft(0x1425991c0, "unknown_1425991c0", tpl_52607::<0x48, 0x141232380> as *const ()), // sub_1425991c0
    Replacement::draft(0x1425991d0, "unknown_1425991d0", tpl_52836::<0x48, 0x1411f9e70, 0x50, 0x1412323a0> as *const ()), // sub_1425991d0
    Replacement::draft(0x1425991f0, "unknown_1425991f0", fn_1425991f0 as *const ()), // sub_1425991f0
    Replacement::draft(0x142599230, "unknown_142599230", tpl_52968::<0x30, 0x1416bdbe0> as *const ()), // sub_142599230
    Replacement::draft(0x142599250, "unknown_142599250", fn_142599250 as *const ()), // sub_142599250
    Replacement::draft(0x142599270, "unknown_142599270", tpl_52970::<0x30, 0x1416bdbe0> as *const ()), // sub_142599270
    Replacement::draft(0x142599290, "unknown_142599290", fn_142599290 as *const ()), // sub_142599290
    Replacement::draft(0x1425992b0, "unknown_1425992b0", fn_1425992b0 as *const ()), // sub_1425992b0
    Replacement::draft(0x1425992d0, "unknown_1425992d0", fn_1425992d0 as *const ()), // sub_1425992d0
    Replacement::draft(0x1425992f0, "unknown_1425992f0", tpl_52609::<0x98, 0x1417580d0> as *const ()), // sub_1425992f0
    Replacement::draft(0x142599300, "unknown_142599300", tpl_52609::<0x28, 0x1417580d0> as *const ()), // sub_142599300
    Replacement::draft(0x142599310, "unknown_142599310", tpl_52609::<0x60, 0x1417580d0> as *const ()), // sub_142599310
    Replacement::draft(0x142599320, "unknown_142599320", tpl_52609::<0xf0, 0x1411f9db0> as *const ()), // sub_142599320
    Replacement::draft(0x142599330, "unknown_142599330", tpl_52609::<0xf8, 0x1411f9db0> as *const ()), // sub_142599330
    Replacement::draft(0x142599340, "unknown_142599340", tpl_52609::<0x100, 0x1411f9db0> as *const ()), // sub_142599340
    Replacement::draft(0x142599350, "unknown_142599350", tpl_52609::<0x108, 0x1411f9db0> as *const ()), // sub_142599350
    Replacement::draft(0x142599360, "unknown_142599360", tpl_52609::<0x110, 0x1411f9db0> as *const ()), // sub_142599360
    Replacement::draft(0x142599370, "unknown_142599370", tpl_52609::<0x118, 0x1411f9db0> as *const ()), // sub_142599370
    Replacement::draft(0x142599380, "unknown_142599380", tpl_52609::<0x120, 0x1411f9db0> as *const ()), // sub_142599380
    Replacement::draft(0x142599390, "unknown_142599390", tpl_52609::<0x128, 0x1411f9db0> as *const ()), // sub_142599390
    Replacement::draft(0x1425993a0, "unknown_1425993a0", tpl_52609::<0x130, 0x1411f9db0> as *const ()), // sub_1425993a0
    Replacement::draft(0x1425993b0, "unknown_1425993b0", tpl_52609::<0x138, 0x1411f9db0> as *const ()), // sub_1425993b0
    Replacement::draft(0x1425993c0, "unknown_1425993c0", tpl_52609::<0xd8, 0x1411f9db0> as *const ()), // sub_1425993c0
    Replacement::draft(0x1425993d0, "unknown_1425993d0", tpl_52609::<0xd0, 0x1411f9db0> as *const ()), // sub_1425993d0
    Replacement::draft(0x1425993e0, "unknown_1425993e0", tpl_52609::<0xe0, 0x1411f9db0> as *const ()), // sub_1425993e0
    Replacement::draft(0x1425993f0, "unknown_1425993f0", tpl_52609::<0xe8, 0x1411f9db0> as *const ()), // sub_1425993f0
    Replacement::draft(0x142599400, "unknown_142599400", tpl_52609::<0x40, 0x1411f9db0> as *const ()), // sub_142599400
    Replacement::draft(0x142599410, "unknown_142599410", tpl_52609::<0x48, 0x1411f9db0> as *const ()), // sub_142599410
    Replacement::draft(0x142599420, "unknown_142599420", tpl_52607::<0x48, 0x1411fb050> as *const ()), // sub_142599420
    Replacement::draft(0x142599430, "unknown_142599430", tpl_52609::<0x58, 0x14175bd20> as *const ()), // sub_142599430
    Replacement::draft(0x142599440, "unknown_142599440", tpl_52609::<0x58, 0x14175bce0> as *const ()), // sub_142599440
    Replacement::draft(0x142599450, "unknown_142599450", tpl_52607::<0x30, 0x14175bce0> as *const ()), // sub_142599450
    Replacement::draft(0x142599460, "unknown_142599460", tpl_52607::<0x38, 0x1416a5fe0> as *const ()), // sub_142599460
    Replacement::draft(0x142599470, "unknown_142599470", tpl_52609::<0x60, 0x14175bce0> as *const ()), // sub_142599470
    Replacement::draft(0x142599480, "unknown_142599480", tpl_52609::<0x88, 0x14175d7c0> as *const ()), // sub_142599480
    Replacement::draft(0x142599490, "unknown_142599490", tpl_52607::<0x60, 0x1416a60d0> as *const ()), // sub_142599490
    Replacement::draft(0x1425994a0, "unknown_1425994a0", tpl_52607::<0x68, 0x1416de410> as *const ()), // sub_1425994a0
    Replacement::draft(0x1425994b0, "unknown_1425994b0", tpl_52609::<0x28, 0x1416a60d0> as *const ()), // sub_1425994b0
    Replacement::draft(0x1425994c0, "unknown_1425994c0", tpl_52619::<0xb0, 0x1411f9e70, 0x20, 0x14174b510> as *const ()), // sub_1425994c0
    Replacement::draft(0x1425994e0, "unknown_1425994e0", tpl_52609::<0x60, 0x1417623a0> as *const ()), // sub_1425994e0
    Replacement::draft(0x1425994f0, "unknown_1425994f0", tpl_52609::<0x70, 0x1411fb920> as *const ()), // sub_1425994f0
    Replacement::draft(0x142599500, "unknown_142599500", tpl_52609::<0x30, 0x1411fb920> as *const ()), // sub_142599500
    Replacement::draft(0x142599510, "unknown_142599510", tpl_52607::<0x20, 0x1411f9e70> as *const ()), // sub_142599510
    Replacement::draft(0x142599520, "unknown_142599520", fn_142599520 as *const ()), // sub_142599520
    Replacement::draft(0x142599570, "unknown_142599570", tpl_52975::<0x30, 0x141764c60> as *const ()), // sub_142599570
    Replacement::draft(0x142599590, "unknown_142599590", fn_142599590 as *const ()), // sub_142599590
    Replacement::draft(0x1425995b0, "unknown_1425995b0", tpl_52619::<0x38, 0x1411f9db0, 0x30, 0x141764db0> as *const ()), // sub_1425995b0
    Replacement::draft(0x1425995d0, "unknown_1425995d0", tpl_52607::<0xb0, 0x1411fa150> as *const ()), // sub_1425995d0
    Replacement::draft(0x1425995e0, "unknown_1425995e0", tpl_52609::<0x128, 0x1416a60f0> as *const ()), // sub_1425995e0
    Replacement::draft(0x1425995f0, "unknown_1425995f0", tpl_52609::<0x140, 0x1416a68d0> as *const ()), // sub_1425995f0
    Replacement::draft(0x142599600, "unknown_142599600", tpl_52609::<0x78, 0x1416f5d10> as *const ()), // sub_142599600
    Replacement::draft(0x142599610, "unknown_142599610", tpl_52609::<0x58, 0x1416f5d10> as *const ()), // sub_142599610
    Replacement::draft(0x142599620, "unknown_142599620", tpl_52609::<0x20, 0x1416a6930> as *const ()), // sub_142599620
    Replacement::draft(0x142599630, "unknown_142599630", fn_142599630 as *const ()), // sub_142599630
    Replacement::draft(0x142599650, "unknown_142599650", fn_142599650 as *const ()), // sub_142599650
    Replacement::draft(0x142599670, "unknown_142599670", fn_142599670 as *const ()), // sub_142599670
    Replacement::draft(0x142599690, "unknown_142599690", fn_142599690 as *const ()), // sub_142599690
    Replacement::draft(0x1425996b0, "unknown_1425996b0", tpl_52609::<0x50, 0x1416f5d10> as *const ()), // sub_1425996b0
    Replacement::draft(0x1425996c0, "unknown_1425996c0", tpl_52609::<0x20, 0x1416f5d10> as *const ()), // sub_1425996c0
    Replacement::draft(0x1425996d0, "unknown_1425996d0", tpl_52609::<0x70, 0x1416adae0> as *const ()), // sub_1425996d0
    Replacement::draft(0x1425996e0, "unknown_1425996e0", tpl_52609::<0x68, 0x1416adae0> as *const ()), // sub_1425996e0
    Replacement::draft(0x1425996f0, "unknown_1425996f0", tpl_52609::<0x98, 0x1411f9e70> as *const ()), // sub_1425996f0
    Replacement::draft(0x142599700, "unknown_142599700", tpl_52609::<0x30, 0x1416a60d0> as *const ()), // sub_142599700
    Replacement::draft(0x142599710, "unknown_142599710", tpl_52609::<0x60, 0x1411f9e70> as *const ()), // sub_142599710
    Replacement::draft(0x142599720, "unknown_142599720", tpl_52609::<0x60, 0x1411fb920> as *const ()), // sub_142599720
    Replacement::draft(0x142599730, "unknown_142599730", tpl_52607::<0x100, 0x1416a5d50> as *const ()), // sub_142599730
    Replacement::draft(0x142599740, "unknown_142599740", tpl_52609::<0x28, 0x1411fb920> as *const ()), // sub_142599740
    Replacement::draft(0x142599750, "unknown_142599750", tpl_52609::<0x90, 0x1411fb920> as *const ()), // sub_142599750
    Replacement::draft(0x142599760, "unknown_142599760", tpl_52609::<0x88, 0x1411fae70> as *const ()), // sub_142599760
    Replacement::draft(0x142599770, "unknown_142599770", tpl_52609::<0x30, 0x1416f5d10> as *const ()), // sub_142599770
    Replacement::draft(0x142599780, "unknown_142599780", tpl_52609::<0x78, 0x14169eee0> as *const ()), // sub_142599780
    Replacement::draft(0x142599790, "unknown_142599790", tpl_52607::<0x100, 0x1411fb920> as *const ()), // sub_142599790
    Replacement::draft(0x1425997a0, "ak_1425997a0", fn_1425997a0 as *const ()), // sub_1425997a0
    Replacement::draft(0x1425997d0, "ak_1425997d0", fn_1425997d0 as *const ()), // sub_1425997d0
    Replacement::draft(0x142599800, "ak_142599800", tpl_52609::<0x40, 0x141778860> as *const ()), // sub_142599800
    Replacement::draft(0x142599810, "ak_142599810", fn_142599810 as *const ()), // sub_142599810
    Replacement::draft(0x142599860, "ak_142599860", tpl_52942::<0xe0, 0x141778a30> as *const ()), // sub_142599860
    Replacement::draft(0x142599880, "ak_142599880", tpl_52984::<0xe0, 0x141778950> as *const ()), // sub_142599880
    Replacement::draft(0x1425998a0, "ak_1425998a0", fn_1425998a0 as *const ()), // sub_1425998a0
    Replacement::draft(0x1425998c0, "ak_1425998c0", tpl_52968::<0xe0, 0x1417789b0> as *const ()), // sub_1425998c0
    Replacement::draft(0x1425998e0, "ak_1425998e0", tpl_52609::<0x40, 0x141778880> as *const ()), // sub_1425998e0
    Replacement::draft(0x1425998f0, "ak_1425998f0", tpl_52986::<0xe0, 0x141778aa0> as *const ()), // sub_1425998f0
    Replacement::draft(0x142599910, "ak_142599910", tpl_52987::<0xe0, 0x141775a20> as *const ()), // sub_142599910
    Replacement::draft(0x142599930, "ak_142599930", fn_142599930 as *const ()), // sub_142599930
    Replacement::draft(0x142599950, "ak_142599950", fn_142599950 as *const ()), // sub_142599950
    Replacement::draft(0x142599970, "ak_142599970", fn_142599970 as *const ()), // sub_142599970
    Replacement::draft(0x1425999a0, "ak_1425999a0", fn_1425999a0 as *const ()), // sub_1425999a0
    Replacement::draft(0x1425999d0, "ak_1425999d0", tpl_52992::<0x130, 0x14177b4c0> as *const ()), // sub_1425999d0
    Replacement::draft(0x1425999f0, "ak_1425999f0", tpl_52993::<0x130, 0x141778a10> as *const ()), // sub_1425999f0
    Replacement::draft(0x142599a10, "ak_142599a10", tpl_52994::<0x130, 0x14177b400> as *const ()), // sub_142599a10
    Replacement::draft(0x142599a30, "ak_142599a30", tpl_52609::<0x30, 0x14177b7e0> as *const ()), // sub_142599a30
    Replacement::draft(0x142599a40, "ak_142599a40", tpl_52995::<0x130, 0x14177b580> as *const ()), // sub_142599a40
    Replacement::draft(0x142599a60, "ak_142599a60", tpl_52986::<0x130, 0x141778860> as *const ()), // sub_142599a60
    Replacement::draft(0x142599a80, "ak_142599a80", tpl_52996::<0x130, 0x141778a30> as *const ()), // sub_142599a80
    Replacement::draft(0x142599aa0, "ak_142599aa0", tpl_52997::<0x130, 0x141778a10> as *const ()), // sub_142599aa0
    Replacement::draft(0x142599ac0, "ak_142599ac0", tpl_52975::<0x130, 0x141778860> as *const ()), // sub_142599ac0
    Replacement::draft(0x142599ae0, "ak_142599ae0", tpl_52609::<0x30, 0x14177b3f0> as *const ()), // sub_142599ae0
    Replacement::draft(0x142599af0, "ak_142599af0", tpl_52609::<0x30, 0x141778860> as *const ()), // sub_142599af0
    Replacement::draft(0x142599b00, "ak_142599b00", tpl_52607::<0x28, 0x141778860> as *const ()), // sub_142599b00
    Replacement::draft(0x142599b10, "ak_142599b10", tpl_52952::<0x28, 0x141775a20, 0x20, 0x14177b3f0> as *const ()), // sub_142599b10
    Replacement::draft(0x142599b30, "ak_142599b30", tpl_52609::<0x20, 0x14177db90> as *const ()), // sub_142599b30
    Replacement::draft(0x142599b40, "ak_142599b40", tpl_52607::<0x60, 0x14177dbf0> as *const ()), // sub_142599b40
    Replacement::draft(0x142599b50, "ak_142599b50", fn_142599b50 as *const ()), // sub_142599b50
    Replacement::draft(0x142599bc0, "ak_142599bc0", fn_142599bc0 as *const ()), // sub_142599bc0
    Replacement::draft(0x142599c00, "ak_142599c00", fn_142599c00 as *const ()), // sub_142599c00
    Replacement::draft(0x142599c20, "ak_142599c20", tpl_52992::<0xd0, 0x141778860> as *const ()), // sub_142599c20
    Replacement::draft(0x142599c40, "ak_142599c40", tpl_52634::<0xd0, 0x141778860> as *const ()), // sub_142599c40
    Replacement::draft(0x142599c60, "ak_142599c60", tpl_52968::<0xd0, 0x141778860> as *const ()), // sub_142599c60
    Replacement::draft(0x142599c80, "ak_142599c80", tpl_53001::<0xd0, 0x14177d5d0> as *const ()), // sub_142599c80
    Replacement::draft(0x142599ca0, "ak_142599ca0", fn_142599ca0 as *const ()), // sub_142599ca0
    Replacement::draft(0x142599cc0, "ak_142599cc0", fn_142599cc0 as *const ()), // sub_142599cc0
    Replacement::draft(0x142599ce0, "ak_142599ce0", tpl_52970::<0xd0, 0x141778860> as *const ()), // sub_142599ce0
    Replacement::draft(0x142599d00, "ak_142599d00", fn_142599d00 as *const ()), // sub_142599d00
    Replacement::draft(0x142599d20, "ak_142599d20", fn_142599d20 as *const ()), // sub_142599d20
    Replacement::draft(0x142599d40, "ak_142599d40", tpl_52609::<0x28, 0x141778860> as *const ()), // sub_142599d40
    Replacement::draft(0x142599d50, "ak_142599d50", fn_142599d50 as *const ()), // sub_142599d50
    Replacement::draft(0x142599da0, "ak_142599da0", tpl_52609::<0x40, 0x141775a20> as *const ()), // sub_142599da0
    Replacement::draft(0x142599db0, "ak_142599db0", fn_142599db0 as *const ()), // sub_142599db0
    Replacement::draft(0x142599e20, "ak_142599e20", tpl_52984::<0xc0, 0x141778860> as *const ()), // sub_142599e20
    Replacement::draft(0x142599e40, "ak_142599e40", tpl_52609::<0x38, 0x141778860> as *const ()), // sub_142599e40
    Replacement::draft(0x142599e50, "ak_142599e50", tpl_52968::<0xc0, 0x14177d5d0> as *const ()), // sub_142599e50
    Replacement::draft(0x142599e70, "ak_142599e70", tpl_53001::<0xc0, 0x14177d5d0> as *const ()), // sub_142599e70
    Replacement::draft(0x142599e90, "ak_142599e90", tpl_52987::<0xc0, 0x141778860> as *const ()), // sub_142599e90
    Replacement::draft(0x142599eb0, "ak_142599eb0", fn_142599eb0 as *const ()), // sub_142599eb0
    Replacement::draft(0x142599ed0, "ak_142599ed0", tpl_52609::<0x38, 0x141778a30> as *const ()), // sub_142599ed0
    Replacement::draft(0x142599ee0, "ak_142599ee0", fn_142599ee0 as *const ()), // sub_142599ee0
    Replacement::draft(0x142599f00, "ak_142599f00", fn_142599f00 as *const ()), // sub_142599f00
    Replacement::draft(0x142599f20, "ak_142599f20", fn_142599f20 as *const ()), // sub_142599f20
    Replacement::draft(0x142599f50, "ak_142599f50", tpl_52607::<0x90, 0x141788170> as *const ()), // sub_142599f50
    Replacement::draft(0x142599f60, "ak_142599f60", fn_142599f60 as *const ()), // sub_142599f60
    Replacement::draft(0x142599fc0, "ak_142599fc0", tpl_52993::<0x90, 0x1417889b0> as *const ()), // sub_142599fc0
    Replacement::draft(0x142599fe0, "ak_142599fe0", fn_142599fe0 as *const ()), // sub_142599fe0
    Replacement::draft(0x14259a000, "ak_14259a000", fn_14259a000 as *const ()), // sub_14259a000
    Replacement::draft(0x14259a030, "ak_14259a030", tpl_52995::<0x20, 0x141778860> as *const ()), // sub_14259a030
    Replacement::draft(0x14259a050, "ak_14259a050", tpl_52986::<0x20, 0x141778a30> as *const ()), // sub_14259a050
    Replacement::draft(0x14259a070, "ak_14259a070", tpl_52996::<0x20, 0x141782060> as *const ()), // sub_14259a070
    Replacement::draft(0x14259a090, "ak_14259a090", tpl_52997::<0x20, 0x141778a30> as *const ()), // sub_14259a090
    Replacement::draft(0x14259a0b0, "ak_14259a0b0", tpl_52609::<0x20, 0x141778a30> as *const ()), // sub_14259a0b0
    Replacement::draft(0x14259a0c0, "ak_14259a0c0", tpl_53015::<0x1426afaa0, 0x1401de410> as *const ()), // sub_14259a0c0
    Replacement::draft(0x14259a0f0, "ak_14259a0f0", tpl_52609::<0x20, 0x14178aa60> as *const ()), // sub_14259a0f0
    Replacement::draft(0x14259a100, "ak_14259a100", tpl_52607::<0x80, 0x14178aac0> as *const ()), // sub_14259a100
    Replacement::draft(0x14259a110, "ak_14259a110", fn_14259a110 as *const ()), // sub_14259a110
    Replacement::draft(0x14259a180, "ak_14259a180", tpl_52984::<0x80, 0x14178b7f0> as *const ()), // sub_14259a180
    Replacement::draft(0x14259a1a0, "ak_14259a1a0", tpl_52994::<0x80, 0x141778860> as *const ()), // sub_14259a1a0
    Replacement::draft(0x14259a1c0, "ak_14259a1c0", tpl_53015::<0x1426afca0, 0x1401de410> as *const ()), // sub_14259a1c0
    Replacement::draft(0x14259a1f0, "ak_14259a1f0", tpl_52607::<0xc50, 0x14178d9c0> as *const ()), // sub_14259a1f0
    Replacement::draft(0x14259a200, "ak_14259a200", fn_14259a200 as *const ()), // sub_14259a200
    Replacement::draft(0x14259a240, "ak_14259a240", tpl_53015::<0x1426c2200, 0x1401de410> as *const ()), // sub_14259a240
    Replacement::draft(0x14259a270, "ak_14259a270", tpl_52609::<0x20, 0x141793f90> as *const ()), // sub_14259a270
    Replacement::draft(0x14259a280, "ak_14259a280", tpl_52607::<0x80, 0x141794000> as *const ()), // sub_14259a280
    Replacement::draft(0x14259a290, "ak_14259a290", fn_14259a290 as *const ()), // sub_14259a290
    Replacement::draft(0x14259a2c0, "ak_14259a2c0", fn_14259a2c0 as *const ()), // sub_14259a2c0
    Replacement::draft(0x14259a2f0, "ak_14259a2f0", tpl_52609::<0x20, 0x141796550> as *const ()), // sub_14259a2f0
    Replacement::draft(0x14259a300, "ak_14259a300", tpl_52607::<0x80, 0x141796590> as *const ()), // sub_14259a300
    Replacement::draft(0x14259a310, "ak_14259a310", fn_14259a310 as *const ()), // sub_14259a310
    Replacement::draft(0x14259a350, "ak_14259a350", tpl_52794::<0x60, 0x141797510, 0x20, 0x1417974b0> as *const ()), // sub_14259a350
    Replacement::draft(0x14259a370, "ak_14259a370", tpl_52607::<0x148, 0x1417974b0> as *const ()), // sub_14259a370
    Replacement::draft(0x14259a380, "ak_14259a380", tpl_52607::<0x88, 0x1417974b0> as *const ()), // sub_14259a380
    Replacement::draft(0x14259a390, "ak_14259a390", tpl_52607::<0x20, 0x141798a50> as *const ()), // sub_14259a390
    Replacement::draft(0x14259a3a0, "ak_14259a3a0", fn_14259a3a0 as *const ()), // sub_14259a3a0
    Replacement::draft(0x14259a3c0, "ak_14259a3c0", tpl_52607::<0x70, 0x1417974b0> as *const ()), // sub_14259a3c0
    Replacement::draft(0x14259a3d0, "ak_14259a3d0", tpl_52607::<0x88, 0x141798a50> as *const ()), // sub_14259a3d0
    Replacement::draft(0x14259a3e0, "ak_14259a3e0", tpl_52607::<0x30, 0x141798a50> as *const ()), // sub_14259a3e0
    Replacement::draft(0x14259a3f0, "ak_14259a3f0", fn_14259a3f0 as *const ()), // sub_14259a3f0
    Replacement::draft(0x14259a400, "ak_14259a400", fn_14259a400 as *const ()), // sub_14259a400
    Replacement::draft(0x14259a430, "unknown_14259a430", tpl_52609::<0x38, 0x1417974b0> as *const ()), // sub_14259a430
    Replacement::draft(0x14259a440, "unknown_14259a440", tpl_52607::<0x100, 0x141797510> as *const ()), // sub_14259a440
    Replacement::draft(0x14259a450, "unknown_14259a450", fn_14259a450 as *const ()), // sub_14259a450
    Replacement::draft(0x14259a4a0, "unknown_14259a4a0", tpl_52609::<0x68, 0x141778860> as *const ()), // sub_14259a4a0
    Replacement::draft(0x14259a4b0, "unknown_14259a4b0", tpl_52609::<0x38, 0x14179f8f0> as *const ()), // sub_14259a4b0
    Replacement::draft(0x14259a4c0, "unknown_14259a4c0", tpl_52609::<0x68, 0x141775a20> as *const ()), // sub_14259a4c0
    Replacement::draft(0x14259a4d0, "unknown_14259a4d0", tpl_52607::<0x30, 0x141775a20> as *const ()), // sub_14259a4d0
    Replacement::draft(0x14259a4e0, "unknown_14259a4e0", tpl_52609::<0x20, 0x1417a65e0> as *const ()), // sub_14259a4e0
    Replacement::draft(0x14259a4f0, "unknown_14259a4f0", tpl_52609::<0x160, 0x1417a6650> as *const ()), // sub_14259a4f0
    Replacement::draft(0x14259a500, "unknown_14259a500", tpl_52609::<0x80, 0x1417a66a0> as *const ()), // sub_14259a500
    Replacement::draft(0x14259a510, "unknown_14259a510", tpl_52609::<0x70, 0x1417a66a0> as *const ()), // sub_14259a510
    Replacement::draft(0x14259a520, "unknown_14259a520", tpl_52609::<0x60, 0x1417a66a0> as *const ()), // sub_14259a520
    Replacement::draft(0x14259a530, "unknown_14259a530", tpl_52609::<0x1c8, 0x1417a6650> as *const ()), // sub_14259a530
    Replacement::draft(0x14259a540, "unknown_14259a540", tpl_52609::<0x168, 0x1417a6650> as *const ()), // sub_14259a540
    Replacement::draft(0x14259a550, "unknown_14259a550", tpl_52609::<0x268, 0x1417a6650> as *const ()), // sub_14259a550
    Replacement::draft(0x14259a560, "unknown_14259a560", tpl_52609::<0x1a0, 0x1417a6650> as *const ()), // sub_14259a560
    Replacement::draft(0x14259a570, "unknown_14259a570", tpl_52609::<0xb8, 0x1417a6620> as *const ()), // sub_14259a570
    Replacement::draft(0x14259a580, "unknown_14259a580", tpl_52609::<0xb8, 0x1417a65e0> as *const ()), // sub_14259a580
    Replacement::draft(0x14259a590, "unknown_14259a590", tpl_52609::<0xc8, 0x1417a65e0> as *const ()), // sub_14259a590
    Replacement::draft(0x14259a5a0, "unknown_14259a5a0", tpl_52609::<0xd8, 0x1417a65e0> as *const ()), // sub_14259a5a0
    Replacement::draft(0x14259a5b0, "unknown_14259a5b0", tpl_52609::<0x20, 0x1417a66a0> as *const ()), // sub_14259a5b0
    Replacement::draft(0x14259a5c0, "unknown_14259a5c0", tpl_52609::<0x70, 0x1417a6650> as *const ()), // sub_14259a5c0
    Replacement::draft(0x14259a5d0, "unknown_14259a5d0", tpl_52609::<0x48, 0x1417a6620> as *const ()), // sub_14259a5d0
    Replacement::draft(0x14259a5e0, "unknown_14259a5e0", tpl_52609::<0x48, 0x1417a65e0> as *const ()), // sub_14259a5e0
    Replacement::draft(0x14259a5ec, "unknown_14259a5ec", fn_14259a5ec as *const ()), // sub_14259a5ec
    Replacement::draft(0x14259a604, "unknown_14259a604", fn_14259a604 as *const ()), // sub_14259a604
    Replacement::draft(0x14259a630, "unknown_14259a630", fn_14259a630 as *const ()), // sub_14259a630
    Replacement::draft(0x14259a68f, "unknown_14259a68f", fn_14259a68f as *const ()), // sub_14259a68f
    Replacement::draft(0x14259a6bb, "unknown_14259a6bb", fn_14259a6bb as *const ()), // sub_14259a6bb
    Replacement::draft(0x14259a6ea, "unknown_14259a6ea", fn_14259a6ea as *const ()), // sub_14259a6ea
    Replacement::draft(0x14259a708, "unknown_14259a708", fn_14259a708 as *const ()), // sub_14259a708
    Replacement::draft(0x14259a740, "unknown_14259a740", fn_14259a740 as *const ()), // sub_14259a740
    Replacement::draft(0x14259a7a0, "unknown_14259a7a0", fn_14259a7a0 as *const ()), // sub_14259a7a0
    Replacement::draft_throws(0x14259a7d0, "unknown_14259a7d0", fn_14259a7d0 as *const (), Throws::Discard), // sub_14259a7d0
    Replacement::draft_throws(0x14259a977, "unknown_14259a977", fn_14259a977 as *const (), Throws::Discard), // sub_14259a977
    Replacement::draft(0x14259adb0, "unknown_14259adb0", fn_14259adb0 as *const ()), // sub_14259adb0
    Replacement::draft(0x14259b210, "unknown_14259b210", tpl_53035::<0xad38, 0x14248bca0, 0x430, 0x14240ab10, 0x1401de3c0> as *const ()), // sub_14259b210
    Replacement::draft(0x14259b367, "unknown_14259b367", fn_14259b367 as *const ()), // sub_14259b367
    Replacement::draft(0x14259bad6, "unknown_14259bad6", fn_14259bad6 as *const ()), // sub_14259bad6
    Replacement::draft(0x14259c3b0, "unknown_14259c3b0", fn_14259c3b0 as *const ()), // sub_14259c3b0
    Replacement::draft(0x14259cdc0, "unknown_14259cdc0", fn_14259cdc0 as *const ()), // sub_14259cdc0
    Replacement::draft(0x14259da24, "unknown_14259da24", tpl_53038::<0x1401de3c0, 0x141834ee8> as *const ()), // sub_14259da24
    Replacement::draft(0x14259da66, "unknown_14259da66", tpl_53039::<0x1401de3c0, 0x141834ee8> as *const ()), // sub_14259da66
    Replacement::draft(0x14259da90, "unknown_14259da90", fn_14259da90 as *const ()), // sub_14259da90
    Replacement::draft(0x14259dc10, "unknown_14259dc10", fn_14259dc10 as *const ()), // sub_14259dc10
    Replacement::draft(0x14259dd24, "unknown_14259dd24", fn_14259dd24 as *const ()), // sub_14259dd24
    Replacement::draft(0x14259dd69, "unknown_14259dd69", tpl_53039::<0x1401de3c0, 0x141834ee8> as *const ()), // sub_14259dd69
    Replacement::draft(0x14259e070, "unknown_14259e070", fn_14259e070 as *const ()), // sub_14259e070
    Replacement::draft(0x14259e1d8, "unknown_14259e1d8", fn_14259e1d8 as *const ()), // sub_14259e1d8
    Replacement::draft(0x14259e27c, "unknown_14259e27c", tpl_53038::<0x1401de3c0, 0x141834ee8> as *const ()), // sub_14259e27c
    Replacement::draft(0x14259e2be, "unknown_14259e2be", tpl_53039::<0x1401de3c0, 0x141834ee8> as *const ()), // sub_14259e2be
    Replacement::draft(0x14259e400, "unknown_14259e400", fn_14259e400 as *const ()), // sub_14259e400
    Replacement::draft(0x14259e500, "unknown_14259e500", fn_14259e500 as *const ()), // sub_14259e500
    Replacement::draft(0x14259ef50, "unknown_14259ef50", tpl_53046::<0x2c0, 0x14240ab10, 0x190, 0x1401de3c0> as *const ()), // sub_14259ef50
    Replacement::draft(0x14259fe40, "unknown_14259fe40", fn_14259fe40 as *const ()), // sub_14259fe40
    Replacement::draft(0x14259feb0, "unknown_14259feb0", fn_14259feb0 as *const ()), // sub_14259feb0
    Replacement::draft_throws(0x14259ff50, "unknown_14259ff50", fn_14259ff50 as *const (), Throws::Discard), // sub_14259ff50
    Replacement::draft(0x1425a04d0, "unknown_1425a04d0", fn_1425a04d0 as *const ()), // sub_1425a04d0
    Replacement::draft(0x1425a0690, "unknown_1425a0690", fn_1425a0690 as *const ()), // sub_1425a0690
    Replacement::draft(0x1425a0aa0, "unknown_1425a0aa0", fn_1425a0aa0 as *const ()), // sub_1425a0aa0
    Replacement::draft(0x1425a0f60, "crt_1425a0f60", tpl_906::<0x142646d10> as *const ()), // j_cohtml::TypeDescription::j_cohtml::TypeDescription::~TypeDescription
    Replacement::draft(0x1425a11a8, "unknown_1425a11a8", fn_1425a11a8 as *const ()), // sub_1425a11a8
    Replacement::draft(0x1425a124e, "unknown_1425a124e", fn_1425a124e as *const ()), // sub_1425a124e
    Replacement::draft_throws(0x1425a1eb0, "unknown_1425a1eb0", fn_1425a1eb0 as *const (), Throws::Discard), // sub_1425a1eb0
    Replacement::draft(0x1425a26fc, "unknown_1425a26fc", fn_1425a26fc as *const ()), // sub_1425a26fc
    Replacement::draft(0x1425a2938, "unknown_1425a2938", fn_1425a2938 as *const ()), // sub_1425a2938
    Replacement::draft(0x1425a297a, "unknown_1425a297a", fn_1425a297a as *const ()), // sub_1425a297a
    Replacement::draft(0x1425a29f0, "unknown_1425a29f0", fn_1425a29f0 as *const ()), // sub_1425a29f0
    Replacement::draft(0x1425a3060, "unknown_1425a3060", fn_1425a3060 as *const ()), // sub_1425a3060
    Replacement::draft(0x1425a34c9, "unknown_1425a34c9", fn_1425a34c9 as *const ()), // sub_1425a34c9
    Replacement::draft(0x1425a3706, "unknown_1425a3706", fn_1425a3706 as *const ()), // sub_1425a3706
    Replacement::draft(0x1425a37e5, "unknown_1425a37e5", fn_1425a37e5 as *const ()), // sub_1425a37e5
    Replacement::draft(0x1425a38e0, "unknown_1425a38e0", tpl_53062::<0x150, 0x140217a20, 0x170> as *const ()), // sub_1425a38e0
    Replacement::draft(0x1425a3910, "unknown_1425a3910", fn_1425a3910 as *const ()), // sub_1425a3910
    Replacement::draft(0x1425a46d6, "unknown_1425a46d6", fn_1425a46d6 as *const ()), // sub_1425a46d6
    Replacement::draft(0x1425a4730, "unknown_1425a4730", fn_1425a4730 as *const ()), // sub_1425a4730
    Replacement::draft_throws(0x1425a4860, "unknown_1425a4860", fn_1425a4860 as *const (), Throws::Discard), // sub_1425a4860
    Replacement::draft(0x1425a4940, "unknown_1425a4940", fn_1425a4940 as *const ()), // sub_1425a4940
    Replacement::draft_throws(0x1425a4b50, "unknown_1425a4b50", tpl_53067::<0x100, 0x14023c130, 0x1a0, 0x1401e1560, 0x60, 0x140217a50> as *const (), Throws::Discard), // sub_1425a4b50
    Replacement::draft_throws(0x1425a4bb0, "unknown_1425a4bb0", fn_1425a4bb0 as *const (), Throws::Discard), // sub_1425a4bb0
    Replacement::draft_throws(0x1425a4cc0, "unknown_1425a4cc0", tpl_53067::<0x430, 0x14023c130, 0x100, 0x1401e68b0, 0x108, 0x1402503a0> as *const (), Throws::Discard), // sub_1425a4cc0
    Replacement::draft(0x1425a5040, "unknown_1425a5040", fn_1425a5040 as *const ()), // sub_1425a5040
    Replacement::draft(0x1425a53e4, "_protected__void___cdecl_cwinapp_1425a53e4", fn_1425a53e4 as *const ()), // sub_1425a53e4
    Replacement::draft(0x1425a5730, "_protected__void___cdecl_cwinapp_1425a5730", fn_1425a5730 as *const ()), // sub_1425a5730
    Replacement::draft_throws(0x1425a5bb0, "_protected__void___cdecl_cwinapp_1425a5bb0", fn_1425a5bb0 as *const (), Throws::Discard), // sub_1425a5bb0
    Replacement::draft(0x1425a63a0, "_protected__void___cdecl_cwinapp_1425a63a0", fn_1425a63a0 as *const ()), // sub_1425a63a0
    Replacement::draft(0x1425a6798, "_protected__void___cdecl_cwinapp_1425a6798", fn_1425a6798 as *const ()), // sub_1425a6798
    Replacement::draft(0x1425a68a4, "_protected__void___cdecl_cwinapp_1425a68a4", fn_1425a68a4 as *const ()), // sub_1425a68a4
    Replacement::draft(0x1425a69b0, "_protected__void___cdecl_cwinapp_1425a69b0", fn_1425a69b0 as *const ()), // sub_1425a69b0
    Replacement::draft(0x1425a6abc, "_protected__void___cdecl_cwinapp_1425a6abc", fn_1425a6abc as *const ()), // sub_1425a6abc
    Replacement::draft(0x1425a6bc8, "_protected__void___cdecl_cwinapp_1425a6bc8", fn_1425a6bc8 as *const ()), // sub_1425a6bc8
    Replacement::draft(0x1425a6cd4, "_protected__void___cdecl_cwinapp_1425a6cd4", fn_1425a6cd4 as *const ()), // sub_1425a6cd4
    Replacement::draft_throws(0x1425a6f80, "_protected__void___cdecl_cwinapp_1425a6f80", fn_1425a6f80 as *const (), Throws::Discard), // sub_1425a6f80
    Replacement::draft(0x1425a8610, "_protected__void___cdecl_cwinapp_1425a8610", fn_1425a8610 as *const ()), // sub_1425a8610
    Replacement::draft(0x1425a86d0, "_protected__void___cdecl_cwinapp_1425a86d0", fn_1425a86d0 as *const ()), // sub_1425a86d0
    Replacement::draft(0x1425a89c0, "_protected__void___cdecl_cwinapp_1425a89c0", fn_1425a89c0 as *const ()), // sub_1425a89c0
    Replacement::draft(0x1425a8e30, "_protected__void___cdecl_cwinapp_1425a8e30", fn_1425a8e30 as *const ()), // sub_1425a8e30
    Replacement::draft_throws(0x1425a8f60, "_protected__void___cdecl_cwinapp_1425a8f60", fn_1425a8f60 as *const (), Throws::Discard), // sub_1425a8f60
    Replacement::draft(0x1425a9010, "_protected__void___cdecl_cwinapp_1425a9010", fn_1425a9010 as *const ()), // sub_1425a9010
    Replacement::draft_throws(0x1425a95d2, "_protected__void___cdecl_cwinapp_1425a95d2", fn_1425a95d2 as *const (), Throws::Discard), // sub_1425a95d2
    Replacement::draft(0x1425a9800, "_protected__void___cdecl_cwinapp_1425a9800", fn_1425a9800 as *const ()), // sub_1425a9800
    Replacement::draft(0x1425a9db0, "crt_1425a9db0", tpl_906::<0x142646d10> as *const ()), // j_cohtml::TypeDescription::j_cohtml::TypeDescription::~TypeDescription
    Replacement::draft(0x1425aac40, "unknown_1425aac40", fn_1425aac40 as *const ()), // sub_1425aac40
    Replacement::draft(0x1425ab07c, "unknown_1425ab07c", fn_1425ab07c as *const ()), // sub_1425ab07c
    Replacement::draft(0x1425ab0b0, "unknown_1425ab0b0", fn_1425ab0b0 as *const ()), // sub_1425ab0b0
    Replacement::draft(0x1425ab250, "unknown_1425ab250", fn_1425ab250 as *const ()), // sub_1425ab250
    Replacement::draft(0x1425abc08, "unknown_1425abc08", fn_1425abc08 as *const ()), // sub_1425abc08
    Replacement::draft(0x1425abc35, "unknown_1425abc35", fn_1425abc35 as *const ()), // sub_1425abc35
    Replacement::draft(0x1425abc69, "unknown_1425abc69", fn_1425abc69 as *const ()), // sub_1425abc69
    Replacement::draft(0x1425abd60, "unknown_1425abd60", fn_1425abd60 as *const ()), // sub_1425abd60
    Replacement::draft(0x1425ac0f0, "unknown_1425ac0f0", fn_1425ac0f0 as *const ()), // sub_1425ac0f0
    Replacement::draft(0x1425ac118, "unknown_1425ac118", fn_1425ac118 as *const ()), // sub_1425ac118
    Replacement::draft(0x1425ac290, "unknown_1425ac290", fn_1425ac290 as *const ()), // sub_1425ac290
    Replacement::draft(0x1425ac4e0, "unknown_1425ac4e0", tpl_53097::<0x1b0, 0x14240ab10, 0x50, 0x140fe28d0> as *const ()), // sub_1425ac4e0
    Replacement::draft(0x1425acaf0, "unknown_1425acaf0", tpl_53098::<0xc40, 0x14240ab10, 0x1f0, 0x14243a340, 0x30, 0x1401de3c0, 0x1413d1d70, 0x1424976f0, 0x410, 0x14243bc40, 0x170, 0x14249df20> as *const ()), // sub_1425acaf0
    Replacement::draft_throws(0x1425acfb0, "unknown_1425acfb0", fn_1425acfb0 as *const (), Throws::Discard), // sub_1425acfb0
    Replacement::draft(0x1425ad290, "unknown_1425ad290", fn_1425ad290 as *const ()), // sub_1425ad290
    Replacement::draft(0x1425aeca0, "unknown_1425aeca0", fn_1425aeca0 as *const ()), // sub_1425aeca0
    Replacement::draft(0x1425afc30, "unknown_1425afc30", fn_1425afc30 as *const ()), // sub_1425afc30
    Replacement::draft_throws(0x1425b0310, "unknown_1425b0310", fn_1425b0310 as *const (), Throws::Discard), // sub_1425b0310
    Replacement::draft(0x1425b05c0, "unknown_1425b05c0", tpl_53098::<0xc80, 0x14240ab10, 0x210, 0x14243a340, 0xd0, 0x1401de3c0, 0x1413d1d70, 0x1424976f0, 0x430, 0x14243bc40, 0x50, 0x14249df20> as *const ()), // sub_1425b05c0
    Replacement::draft(0x1425b0ec0, "unknown_1425b0ec0", tpl_53062::<0x3c0, 0x14110b480, 0x70> as *const ()), // sub_1425b0ec0
    Replacement::draft(0x1425b11d0, "unknown_1425b11d0", fn_1425b11d0 as *const ()), // sub_1425b11d0
    Replacement::draft(0x1425b1d20, "unknown_1425b1d20", fn_1425b1d20 as *const ()), // sub_1425b1d20
    Replacement::draft(0x1425b1e00, "unknown_1425b1e00", fn_1425b1e00 as *const ()), // sub_1425b1e00
    Replacement::draft(0x1425b1e90, "unknown_1425b1e90", fn_1425b1e90 as *const ()), // sub_1425b1e90
    Replacement::draft_throws(0x1425b2350, "unknown_1425b2350", fn_1425b2350 as *const (), Throws::Discard), // sub_1425b2350
    Replacement::draft(0x1425b24c0, "unknown_1425b24c0", fn_1425b24c0 as *const ()), // sub_1425b24c0
    Replacement::draft(0x1425b25b8, "unknown_1425b25b8", fn_1425b25b8 as *const ()), // sub_1425b25b8
    Replacement::draft(0x1425b26d7, "unknown_1425b26d7", fn_1425b26d7 as *const ()), // sub_1425b26d7
    Replacement::draft(0x1425b2750, "unknown_1425b2750", fn_1425b2750 as *const ()), // sub_1425b2750
    Replacement::draft(0x1425b3660, "unknown_1425b3660", tpl_53035::<0x3a0, 0x140c34930, 0x30, 0x1401edac0, 0x1401ee260> as *const ()), // sub_1425b3660
    Replacement::draft(0x1425b36b0, "unknown_1425b36b0", fn_1425b36b0 as *const ()), // sub_1425b36b0
    Replacement::draft(0x1425b3d00, "unknown_1425b3d00", tpl_53035::<0x3c0, 0x140c34930, 0x50, 0x1401edac0, 0x1401ee260> as *const ()), // sub_1425b3d00
    Replacement::draft_throws(0x1425b3dc0, "unknown_1425b3dc0", fn_1425b3dc0 as *const (), Throws::Discard), // sub_1425b3dc0
    Replacement::draft(0x1425b4150, "unknown_1425b4150", fn_1425b4150 as *const ()), // sub_1425b4150
    Replacement::draft(0x1425b454c, "unknown_1425b454c", fn_1425b454c as *const ()), // sub_1425b454c
    Replacement::draft(0x1425b457a, "unknown_1425b457a", fn_1425b457a as *const ()), // sub_1425b457a
    Replacement::draft(0x1425b45a8, "unknown_1425b45a8", fn_1425b45a8 as *const ()), // sub_1425b45a8
    Replacement::draft_throws(0x1425b49e0, "unknown_1425b49e0", fn_1425b49e0 as *const (), Throws::Discard), // sub_1425b49e0
    Replacement::draft(0x1425b4e37, "unknown_1425b4e37", fn_1425b4e37 as *const ()), // sub_1425b4e37
    Replacement::draft(0x1425b4e87, "unknown_1425b4e87", fn_1425b4e87 as *const ()), // sub_1425b4e87
    Replacement::draft(0x1425b4eb8, "unknown_1425b4eb8", fn_1425b4eb8 as *const ()), // sub_1425b4eb8
    Replacement::draft(0x1425b4ee9, "unknown_1425b4ee9", fn_1425b4ee9 as *const ()), // sub_1425b4ee9
    Replacement::draft(0x1425b4f1a, "unknown_1425b4f1a", fn_1425b4f1a as *const ()), // sub_1425b4f1a
    Replacement::draft(0x1425b4f71, "unknown_1425b4f71", fn_1425b4f71 as *const ()), // sub_1425b4f71
    Replacement::draft(0x1425b5274, "unknown_1425b5274", fn_1425b5274 as *const ()), // sub_1425b5274
    Replacement::draft(0x1425b5614, "unknown_1425b5614", fn_1425b5614 as *const ()), // sub_1425b5614
    Replacement::draft_throws(0x1425b5920, "unknown_1425b5920", fn_1425b5920 as *const (), Throws::Discard), // sub_1425b5920
    Replacement::draft(0x1425b5e62, "unknown_1425b5e62", fn_1425b5e62 as *const ()), // sub_1425b5e62
    Replacement::draft(0x1425b6390, "unknown_1425b6390", fn_1425b6390 as *const ()), // sub_1425b6390
    Replacement::draft_throws(0x1425b63f0, "unknown_1425b63f0", fn_1425b63f0 as *const (), Throws::Caught), // sub_1425b63f0
    Replacement::draft(0x1425b67b0, "unknown_1425b67b0", fn_1425b67b0 as *const ()), // sub_1425b67b0
    Replacement::draft_throws(0x1425b68d0, "unknown_1425b68d0", fn_1425b68d0 as *const (), Throws::Discard), // sub_1425b68d0
    Replacement::draft(0x1425b6dd0, "unknown_1425b6dd0", fn_1425b6dd0 as *const ()), // sub_1425b6dd0
    Replacement::draft(0x1425b7400, "unknown_1425b7400", fn_1425b7400 as *const ()), // sub_1425b7400
    Replacement::draft_throws(0x1425b7428, "unknown_1425b7428", fn_1425b7428 as *const (), Throws::Caught), // sub_1425b7428
    Replacement::draft(0x1425b7463, "unknown_1425b7463", fn_1425b7463 as *const ()), // sub_1425b7463
    Replacement::draft(0x1425b7520, "unknown_1425b7520", fn_1425b7520 as *const ()), // sub_1425b7520
    Replacement::draft(0x1425b75f0, "unknown_1425b75f0", fn_1425b75f0 as *const ()), // sub_1425b75f0
    Replacement::draft_throws(0x1425b7710, "unknown_1425b7710", fn_1425b7710 as *const (), Throws::Discard), // sub_1425b7710
    Replacement::draft(0x1425b7798, "unknown_1425b7798", fn_1425b7798 as *const ()), // sub_1425b7798
    Replacement::draft(0x1425b77ee, "unknown_1425b77ee", fn_1425b77ee as *const ()), // sub_1425b77ee
    Replacement::draft(0x1425b78f0, "unknown_1425b78f0", fn_1425b78f0 as *const ()), // sub_1425b78f0
    Replacement::draft_throws(0x1425b7990, "unknown_1425b7990", fn_1425b7990 as *const (), Throws::Discard), // sub_1425b7990
    Replacement::draft(0x1425b79e0, "unknown_1425b79e0", fn_1425b79e0 as *const ()), // sub_1425b79e0
    Replacement::draft_throws(0x1425b7a60, "unknown_1425b7a60", fn_1425b7a60 as *const (), Throws::Discard), // sub_1425b7a60
    Replacement::draft(0x1425b7bb6, "unknown_1425b7bb6", tpl_53146::<0x80> as *const ()), // sub_1425b7bb6
    Replacement::draft(0x1425b7cd0, "unknown_1425b7cd0", tpl_53035::<0x440, 0x140c34930, 0xd0, 0x1401edac0, 0x1401ee260> as *const ()), // sub_1425b7cd0
    Replacement::draft(0x1425b7cf4, "unknown_1425b7cf4", tpl_53147::<0x80> as *const ()), // sub_1425b7cf4
    Replacement::draft(0x1425b7d24, "unknown_1425b7d24", tpl_53147::<0x80> as *const ()), // sub_1425b7d24
    Replacement::draft(0x1425b7d54, "unknown_1425b7d54", tpl_53147::<0x80> as *const ()), // sub_1425b7d54
    Replacement::draft_throws(0x1425b7f10, "unknown_1425b7f10", fn_1425b7f10 as *const (), Throws::Discard), // sub_1425b7f10
    Replacement::draft_throws(0x1425b811c, "unknown_1425b811c", fn_1425b811c as *const (), Throws::Discard), // sub_1425b811c
    Replacement::draft(0x1425b8576, "unknown_1425b8576", tpl_53146::<0x80> as *const ()), // sub_1425b8576
    Replacement::draft_throws(0x1425b85b0, "unknown_1425b85b0", fn_1425b85b0 as *const (), Throws::Discard), // sub_1425b85b0
    Replacement::draft_throws(0x1425b8730, "unknown_1425b8730", fn_1425b8730 as *const (), Throws::Discard), // sub_1425b8730
    Replacement::draft(0x1425b9123, "unknown_1425b9123", fn_1425b9123 as *const ()), // sub_1425b9123
    Replacement::draft(0x1425b9ba0, "unknown_1425b9ba0", tpl_53097::<0x390, 0x140267c50, 0x20, 0x1412a89f0> as *const ()), // sub_1425b9ba0
    Replacement::draft(0x1425b9f80, "unknown_1425b9f80", tpl_53153::<0x2f0, 0x1401dff10, 0x30, 0x141290c10> as *const ()), // sub_1425b9f80
    Replacement::draft_throws(0x1425ba570, "unknown_1425ba570", fn_1425ba570 as *const (), Throws::Discard), // sub_1425ba570
    Replacement::draft(0x1425babd0, "unknown_1425babd0", tpl_53153::<0xd8, 0x1421a1110, 0x28, 0x1401f9d70> as *const ()), // sub_1425babd0
    Replacement::draft(0x1425bb290, "unknown_1425bb290", tpl_53153::<0xc0, 0x14102bdf0, 0x450, 0x1401dff10> as *const ()), // sub_1425bb290
    Replacement::draft(0x1425bbd60, "unknown_1425bbd60", tpl_53153::<0x24, 0x1412b7cb0, 0x28, 0x1412b7c50> as *const ()), // sub_1425bbd60
    Replacement::draft(0x1425bc120, "unknown_1425bc120", fn_1425bc120 as *const ()), // sub_1425bc120
    Replacement::draft(0x1425bc65c, "unknown_1425bc65c", fn_1425bc65c as *const ()), // sub_1425bc65c
    Replacement::draft(0x1425bc890, "unknown_1425bc890", fn_1425bc890 as *const ()), // sub_1425bc890
    Replacement::draft(0x1425bd1d0, "unknown_1425bd1d0", fn_1425bd1d0 as *const ()), // sub_1425bd1d0
    Replacement::draft(0x1425bd3a2, "unknown_1425bd3a2", fn_1425bd3a2 as *const ()), // sub_1425bd3a2
    Replacement::draft(0x1425bd508, "unknown_1425bd508", fn_1425bd508 as *const ()), // sub_1425bd508
    Replacement::draft_throws(0x1425bd5b1, "unknown_1425bd5b1", fn_1425bd5b1 as *const (), Throws::Discard), // sub_1425bd5b1
    Replacement::draft_throws(0x1425bd610, "unknown_1425bd610", fn_1425bd610 as *const (), Throws::Discard), // sub_1425bd610
    Replacement::draft(0x1425bdd3e, "unknown_1425bdd3e", fn_1425bdd3e as *const ()), // sub_1425bdd3e
    Replacement::draft(0x1425bdd6f, "unknown_1425bdd6f", fn_1425bdd6f as *const ()), // sub_1425bdd6f
    Replacement::draft(0x1425bdda0, "unknown_1425bdda0", fn_1425bdda0 as *const ()), // sub_1425bdda0
    Replacement::draft(0x1425bddd9, "unknown_1425bddd9", fn_1425bddd9 as *const ()), // sub_1425bddd9
    Replacement::draft(0x1425bde33, "unknown_1425bde33", fn_1425bde33 as *const ()), // sub_1425bde33
    Replacement::draft(0x1425bde7a, "unknown_1425bde7a", fn_1425bde7a as *const ()), // sub_1425bde7a
    Replacement::draft(0x1425bdf24, "unknown_1425bdf24", fn_1425bdf24 as *const ()), // sub_1425bdf24
    Replacement::draft(0x1425be009, "unknown_1425be009", fn_1425be009 as *const ()), // sub_1425be009
    Replacement::draft(0x1425be868, "unknown_1425be868", fn_1425be868 as *const ()), // sub_1425be868
    Replacement::draft(0x1425be896, "unknown_1425be896", fn_1425be896 as *const ()), // sub_1425be896
    Replacement::draft(0x1425be8c4, "unknown_1425be8c4", fn_1425be8c4 as *const ()), // sub_1425be8c4
    Replacement::draft(0x1425bea00, "unknown_1425bea00", fn_1425bea00 as *const ()), // sub_1425bea00
    Replacement::draft(0x1425bed28, "unknown_1425bed28", fn_1425bed28 as *const ()), // sub_1425bed28
    Replacement::draft(0x1425bed59, "unknown_1425bed59", fn_1425bed59 as *const ()), // sub_1425bed59
    Replacement::draft(0x1425bed8d, "unknown_1425bed8d", fn_1425bed8d as *const ()), // sub_1425bed8d
    Replacement::draft(0x1425bee04, "unknown_1425bee04", fn_1425bee04 as *const ()), // sub_1425bee04
    Replacement::draft(0x1425beea4, "unknown_1425beea4", fn_1425beea4 as *const ()), // sub_1425beea4
    Replacement::draft_throws(0x1425befc0, "unknown_1425befc0", fn_1425befc0 as *const (), Throws::Discard), // sub_1425befc0
    Replacement::draft(0x1425bf778, "unknown_1425bf778", fn_1425bf778 as *const ()), // sub_1425bf778
    Replacement::draft(0x1425bf7a5, "unknown_1425bf7a5", fn_1425bf7a5 as *const ()), // sub_1425bf7a5
    Replacement::draft(0x1425bf7d2, "unknown_1425bf7d2", fn_1425bf7d2 as *const ()), // sub_1425bf7d2
    Replacement::draft(0x1425c0f7e, "unknown_1425c0f7e", fn_1425c0f7e as *const ()), // sub_1425c0f7e
    Replacement::draft(0x1425c0fb2, "unknown_1425c0fb2", fn_1425c0fb2 as *const ()), // sub_1425c0fb2
    Replacement::draft(0x1425c1290, "unknown_1425c1290", fn_1425c1290 as *const ()), // sub_1425c1290
    Replacement::draft(0x1425c12e0, "unknown_1425c12e0", fn_1425c12e0 as *const ()), // sub_1425c12e0
    Replacement::draft(0x1425c17c0, "unknown_1425c17c0", fn_1425c17c0 as *const ()), // sub_1425c17c0
    Replacement::draft_throws(0x1425c18b0, "unknown_1425c18b0", fn_1425c18b0 as *const (), Throws::Discard), // sub_1425c18b0
    Replacement::draft(0x1425c19c0, "unknown_1425c19c0", fn_1425c19c0 as *const ()), // sub_1425c19c0
    Replacement::draft_throws(0x1425c2410, "unknown_1425c2410", fn_1425c2410 as *const (), Throws::Discard), // sub_1425c2410
    Replacement::draft_throws(0x1425c26d0, "unknown_1425c26d0", fn_1425c26d0 as *const (), Throws::Discard), // sub_1425c26d0
    Replacement::draft_throws(0x1425c2770, "unknown_1425c2770", fn_1425c2770 as *const (), Throws::Discard), // sub_1425c2770
    Replacement::draft_throws(0x1425c29f0, "unknown_1425c29f0", fn_1425c29f0 as *const (), Throws::Discard), // sub_1425c29f0
    Replacement::draft(0x1425c2a30, "unknown_1425c2a30", fn_1425c2a30 as *const ()), // sub_1425c2a30
    Replacement::draft(0x1425c3380, "unknown_1425c3380", fn_1425c3380 as *const ()), // sub_1425c3380
    Replacement::draft(0x1425c4250, "unknown_1425c4250", fn_1425c4250 as *const ()), // sub_1425c4250
    Replacement::draft_throws(0x1425c4460, "unknown_1425c4460", fn_1425c4460 as *const (), Throws::Discard), // sub_1425c4460
    Replacement::draft(0x1425c46c0, "unknown_1425c46c0", fn_1425c46c0 as *const ()), // sub_1425c46c0
    Replacement::draft(0x1425c4833, "unknown_1425c4833", fn_1425c4833 as *const ()), // sub_1425c4833
    Replacement::draft(0x1425c4964, "unknown_1425c4964", fn_1425c4964 as *const ()), // sub_1425c4964
    Replacement::draft(0x1425c4a61, "unknown_1425c4a61", fn_1425c4a61 as *const ()), // sub_1425c4a61
    Replacement::draft_throws(0x1425c4e10, "unknown_1425c4e10", fn_1425c4e10 as *const (), Throws::Discard), // sub_1425c4e10
    Replacement::draft(0x1425c5465, "unknown_1425c5465", fn_1425c5465 as *const ()), // sub_1425c5465
    Replacement::draft(0x1425c5499, "unknown_1425c5499", fn_1425c5499 as *const ()), // sub_1425c5499
    Replacement::draft(0x1425c5598, "unknown_1425c5598", fn_1425c5598 as *const ()), // sub_1425c5598
    Replacement::draft_throws(0x1425c58d0, "unknown_1425c58d0", fn_1425c58d0 as *const (), Throws::Discard), // sub_1425c58d0
    Replacement::draft(0x1425c5df0, "unknown_1425c5df0", fn_1425c5df0 as *const ()), // sub_1425c5df0
    Replacement::draft_throws(0x1425c6470, "unknown_1425c6470", fn_1425c6470 as *const (), Throws::Discard), // sub_1425c6470
    Replacement::draft(0x1425c6a34, "unknown_1425c6a34", fn_1425c6a34 as *const ()), // sub_1425c6a34
    Replacement::draft(0x1425c6c36, "unknown_1425c6c36", fn_1425c6c36 as *const ()), // sub_1425c6c36
    Replacement::draft_throws(0x1425c6c70, "unknown_1425c6c70", fn_1425c6c70 as *const (), Throws::Discard), // sub_1425c6c70
    Replacement::draft_throws(0x1425c7210, "unknown_1425c7210", fn_1425c7210 as *const (), Throws::Discard), // sub_1425c7210
    Replacement::draft(0x1425c724c, "unknown_1425c724c", fn_1425c724c as *const ()), // sub_1425c724c
    Replacement::draft_throws(0x1425c7560, "unknown_1425c7560", fn_1425c7560 as *const (), Throws::Discard), // sub_1425c7560
    Replacement::draft_throws(0x1425c7d90, "unknown_1425c7d90", fn_1425c7d90 as *const (), Throws::Discard), // sub_1425c7d90
    Replacement::draft(0x1425c82f4, "unknown_1425c82f4", fn_1425c82f4 as *const ()), // sub_1425c82f4
    Replacement::draft(0x1425c849c, "unknown_1425c849c", fn_1425c849c as *const ()), // sub_1425c849c
    Replacement::draft(0x1425c8620, "unknown_1425c8620", fn_1425c8620 as *const ()), // sub_1425c8620
    Replacement::draft_throws(0x1425c86e0, "unknown_1425c86e0", fn_1425c86e0 as *const (), Throws::Discard), // sub_1425c86e0
    Replacement::draft(0x1425c8954, "unknown_1425c8954", fn_1425c8954 as *const ()), // sub_1425c8954
    Replacement::draft(0x1425c8d10, "unknown_1425c8d10", fn_1425c8d10 as *const ()), // sub_1425c8d10
    Replacement::draft_throws(0x1425c9c40, "unknown_1425c9c40", fn_1425c9c40 as *const (), Throws::Discard), // sub_1425c9c40
    Replacement::draft(0x1425c9e16, "unknown_1425c9e16", fn_1425c9e16 as *const ()), // sub_1425c9e16
    Replacement::draft(0x1425ca270, "unknown_1425ca270", fn_1425ca270 as *const ()), // sub_1425ca270
    Replacement::draft(0x1425ca2f0, "unknown_1425ca2f0", fn_1425ca2f0 as *const ()), // sub_1425ca2f0
    Replacement::draft(0x1425ca43d, "unknown_1425ca43d", fn_1425ca43d as *const ()), // sub_1425ca43d
    Replacement::draft(0x1425cada0, "unknown_1425cada0", fn_1425cada0 as *const ()), // sub_1425cada0
    Replacement::draft(0x1425cb427, "unknown_1425cb427", fn_1425cb427 as *const ()), // sub_1425cb427
    Replacement::draft(0x1425cbc37, "unknown_1425cbc37", fn_1425cbc37 as *const ()), // sub_1425cbc37
    Replacement::draft(0x1425cbeb0, "unknown_1425cbeb0", fn_1425cbeb0 as *const ()), // sub_1425cbeb0
    Replacement::draft_throws(0x1425cc654, "unknown_1425cc654", fn_1425cc654 as *const (), Throws::Discard), // sub_1425cc654
    Replacement::draft(0x1425cc700, "unknown_1425cc700", fn_1425cc700 as *const ()), // sub_1425cc700
    Replacement::draft_throws(0x1425ccae8, "unknown_1425ccae8", fn_1425ccae8 as *const (), Throws::Discard), // sub_1425ccae8
    Replacement::draft(0x1425cd638, "unknown_1425cd638", fn_1425cd638 as *const ()), // sub_1425cd638
    Replacement::draft(0x1425cdc20, "unknown_1425cdc20", fn_1425cdc20 as *const ()), // sub_1425cdc20
    Replacement::draft(0x1425cdfb0, "unknown_1425cdfb0", fn_1425cdfb0 as *const ()), // sub_1425cdfb0
    Replacement::draft_throws(0x1425ce620, "unknown_1425ce620", fn_1425ce620 as *const (), Throws::Discard), // sub_1425ce620
    Replacement::draft(0x1425cee10, "unknown_1425cee10", tpl_53046::<0x280, 0x14240ab10, 0x40, 0x1401de3c0> as *const ()), // sub_1425cee10
    Replacement::draft(0x1425cf480, "unknown_1425cf480", fn_1425cf480 as *const ()), // sub_1425cf480
    Replacement::draft(0x1425cf6d8, "unknown_1425cf6d8", fn_1425cf6d8 as *const ()), // sub_1425cf6d8
    Replacement::draft(0x1425cf704, "unknown_1425cf704", fn_1425cf704 as *const ()), // sub_1425cf704
    Replacement::draft(0x1425cf73c, "unknown_1425cf73c", fn_1425cf73c as *const ()), // sub_1425cf73c
    Replacement::draft(0x1425cf8df, "unknown_1425cf8df", fn_1425cf8df as *const ()), // sub_1425cf8df
    Replacement::draft(0x1425cf90b, "unknown_1425cf90b", fn_1425cf90b as *const ()), // sub_1425cf90b
    Replacement::draft(0x1425cf943, "unknown_1425cf943", tpl_53239::<0x1413db2b0, 0x141834ee8> as *const ()), // sub_1425cf943
    Replacement::draft(0x1425cfa48, "unknown_1425cfa48", tpl_53239::<0x1413db2b0, 0x141834ee8> as *const ()), // sub_1425cfa48
    Replacement::draft(0x1425cfb2e, "unknown_1425cfb2e", tpl_53240::<0x141834d30> as *const ()), // sub_1425cfb2e
    Replacement::draft(0x1425cfb5a, "unknown_1425cfb5a", tpl_53241::<0x1413dbb50, 0x141834ee8> as *const ()), // sub_1425cfb5a
    Replacement::draft(0x1425cfb9c, "unknown_1425cfb9c", fn_1425cfb9c as *const ()), // sub_1425cfb9c
    Replacement::draft(0x1425cfbe1, "unknown_1425cfbe1", fn_1425cfbe1 as *const ()), // sub_1425cfbe1
    Replacement::draft(0x1425cfc43, "unknown_1425cfc43", tpl_53240::<0x141834d30> as *const ()), // sub_1425cfc43
    Replacement::draft(0x1425cfc6f, "unknown_1425cfc6f", fn_1425cfc6f as *const ()), // sub_1425cfc6f
    Replacement::draft(0x1425cfcb4, "unknown_1425cfcb4", fn_1425cfcb4 as *const ()), // sub_1425cfcb4
    Replacement::draft(0x1425cfced, "unknown_1425cfced", fn_1425cfced as *const ()), // sub_1425cfced
    Replacement::draft(0x1425cfd3e, "unknown_1425cfd3e", fn_1425cfd3e as *const ()), // sub_1425cfd3e
    Replacement::draft(0x1425cfd6a, "unknown_1425cfd6a", tpl_53241::<0x1413dbb50, 0x141834ee8> as *const ()), // sub_1425cfd6a
    Replacement::draft(0x1425cfdac, "unknown_1425cfdac", fn_1425cfdac as *const ()), // sub_1425cfdac
    Replacement::draft(0x1425cfdf1, "unknown_1425cfdf1", fn_1425cfdf1 as *const ()), // sub_1425cfdf1
    Replacement::draft(0x1425cfe70, "unknown_1425cfe70", tpl_53153::<0x820, 0x1413dd8c0, 0x20, 0x14243bc40> as *const ()), // sub_1425cfe70
    Replacement::draft(0x1425d06b0, "unknown_1425d06b0", tpl_53062::<0x170, 0x1424dfa80, 0xd0> as *const ()), // sub_1425d06b0
    Replacement::draft(0x1425d088f, "unknown_1425d088f", fn_1425d088f as *const ()), // sub_1425d088f
    Replacement::draft(0x1425d0b0e, "unknown_1425d0b0e", fn_1425d0b0e as *const ()), // sub_1425d0b0e
    Replacement::draft(0x1425d0b9a, "unknown_1425d0b9a", fn_1425d0b9a as *const ()), // sub_1425d0b9a
    Replacement::draft(0x1425d0da7, "unknown_1425d0da7", fn_1425d0da7 as *const ()), // sub_1425d0da7
    Replacement::draft(0x1425d0e01, "unknown_1425d0e01", fn_1425d0e01 as *const ()), // sub_1425d0e01
    Replacement::draft(0x1425d0e35, "unknown_1425d0e35", fn_1425d0e35 as *const ()), // sub_1425d0e35
    Replacement::draft(0x1425d0f80, "unknown_1425d0f80", fn_1425d0f80 as *const ()), // sub_1425d0f80
    Replacement::draft(0x1425d0fb0, "unknown_1425d0fb0", fn_1425d0fb0 as *const ()), // sub_1425d0fb0
    Replacement::draft(0x1425d14c0, "crt_1425d14c0", tpl_906::<0x142646cf0> as *const ()), // j_cohtml::IAllocator::j_cohtml::IAllocator::~IAllocator
    Replacement::draft(0x1425d16c0, "unknown_1425d16c0", fn_1425d16c0 as *const ()), // sub_1425d16c0
    Replacement::draft_throws(0x1425d1aeb, "unknown_1425d1aeb", fn_1425d1aeb as *const (), Throws::Caught), // sub_1425d1aeb
    Replacement::draft_throws(0x1425d2075, "unknown_1425d2075", fn_1425d2075 as *const (), Throws::Caught), // sub_1425d2075
    Replacement::draft(0x1425d2d80, "unknown_1425d2d80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843390, 0x143a00290> as *const ()), // sub_1425d2d80
    Replacement::draft(0x1425d2e10, "unknown_1425d2e10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438433f8, 0x143a00290> as *const ()), // sub_1425d2e10
    Replacement::draft(0x1425d2ea0, "unknown_1425d2ea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843478, 0x143a00290> as *const ()), // sub_1425d2ea0
    Replacement::draft(0x1425d2f30, "unknown_1425d2f30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438433e0, 0x143a00290> as *const ()), // sub_1425d2f30
    Replacement::draft(0x1425d2fc0, "unknown_1425d2fc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843458, 0x143a00290> as *const ()), // sub_1425d2fc0
    Replacement::draft(0x1425d3050, "unknown_1425d3050", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843348, 0x143a00290> as *const ()), // sub_1425d3050
    Replacement::draft(0x1425d30e0, "unknown_1425d30e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843410, 0x143a00290> as *const ()), // sub_1425d30e0
    Replacement::draft(0x1425d3170, "unknown_1425d3170", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438434a8, 0x143a00290> as *const ()), // sub_1425d3170
    Replacement::draft(0x1425d3200, "unknown_1425d3200", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843490, 0x143a00290> as *const ()), // sub_1425d3200
    Replacement::draft(0x1425d3290, "unknown_1425d3290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843330, 0x143a00290> as *const ()), // sub_1425d3290
    Replacement::draft(0x1425d37f0, "unknown_1425d37f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843580, 0x143a00290> as *const ()), // sub_1425d37f0
    Replacement::draft(0x1425d3880, "unknown_1425d3880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843550, 0x143a00290> as *const ()), // sub_1425d3880
    Replacement::draft(0x1425d3a10, "unknown_1425d3a10", fn_1425d3a10 as *const ()), // sub_1425d3a10
    Replacement::draft(0x1425d52a0, "unknown_1425d52a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843a20, 0x143a00290> as *const ()), // sub_1425d52a0
    Replacement::draft(0x1425d5330, "unknown_1425d5330", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843a80, 0x143a00290> as *const ()), // sub_1425d5330
    Replacement::draft(0x1425d5890, "unknown_1425d5890", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843ae8, 0x143a00290> as *const ()), // sub_1425d5890
    Replacement::draft(0x1425d59b0, "unknown_1425d59b0", tpl_111516::<0x143c8d190, 0x8, 0xc> as *const ()), // sub_1425d59b0
    Replacement::draft(0x1425d5a00, "unknown_1425d5a00", tpl_111516::<0x143c8d1e8, 0x8, 0xc> as *const ()), // sub_1425d5a00
    Replacement::draft(0x1425d5a50, "unknown_1425d5a50", tpl_111516::<0x143c8d1b0, 0x8, 0xc> as *const ()), // sub_1425d5a50
    Replacement::draft(0x1425d5aa0, "unknown_1425d5aa0", fn_1425d5aa0 as *const ()), // sub_1425d5aa0
    Replacement::draft(0x1425d5b00, "unknown_1425d5b00", tpl_111516::<0x143c8d1f8, 0x8, 0xc> as *const ()), // sub_1425d5b00
    Replacement::draft(0x1425d5b50, "unknown_1425d5b50", tpl_111516::<0x143c8d1c8, 0x8, 0xc> as *const ()), // sub_1425d5b50
    Replacement::draft(0x1425d5ba0, "unknown_1425d5ba0", fn_1425d5ba0 as *const ()), // sub_1425d5ba0
    Replacement::draft(0x1425d5ca0, "unknown_1425d5ca0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843d00, 0x143a00290> as *const ()), // sub_1425d5ca0
    Replacement::draft(0x1425d5d30, "unknown_1425d5d30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843d40, 0x143a00290> as *const ()), // sub_1425d5d30
    Replacement::draft(0x1425d5dc0, "unknown_1425d5dc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843dd8, 0x143a00290> as *const ()), // sub_1425d5dc0
    Replacement::draft(0x1425d5e50, "unknown_1425d5e50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843db8, 0x143a00290> as *const ()), // sub_1425d5e50
    Replacement::draft(0x1425d5ee0, "unknown_1425d5ee0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843df8, 0x143a00290> as *const ()), // sub_1425d5ee0
    Replacement::draft(0x1425d5f70, "unknown_1425d5f70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843ce0, 0x143a00290> as *const ()), // sub_1425d5f70
    Replacement::draft(0x1425d6000, "unknown_1425d6000", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843e78, 0x143a00290> as *const ()), // sub_1425d6000
    Replacement::draft(0x1425d6090, "unknown_1425d6090", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843e18, 0x143a00290> as *const ()), // sub_1425d6090
    Replacement::draft(0x1425d6120, "unknown_1425d6120", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843e58, 0x143a00290> as *const ()), // sub_1425d6120
    Replacement::draft(0x1425d61b0, "unknown_1425d61b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843bc0, 0x143a00290> as *const ()), // sub_1425d61b0
    Replacement::draft(0x1425d6240, "unknown_1425d6240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843cc0, 0x143a00290> as *const ()), // sub_1425d6240
    Replacement::draft(0x1425d62d0, "unknown_1425d62d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843e38, 0x143a00290> as *const ()), // sub_1425d62d0
    Replacement::draft(0x1425d6360, "unknown_1425d6360", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843d78, 0x143a00290> as *const ()), // sub_1425d6360
    Replacement::draft(0x1425d63f0, "unknown_1425d63f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843c70, 0x143a00290> as *const ()), // sub_1425d63f0
    Replacement::draft(0x1425d6480, "unknown_1425d6480", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843c30, 0x143a00290> as *const ()), // sub_1425d6480
    Replacement::draft(0x1425d6510, "unknown_1425d6510", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843c50, 0x143a00290> as *const ()), // sub_1425d6510
    Replacement::draft(0x1425d65a0, "unknown_1425d65a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843c10, 0x143a00290> as *const ()), // sub_1425d65a0
    Replacement::draft(0x1425d6630, "unknown_1425d6630", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843d98, 0x143a00290> as *const ()), // sub_1425d6630
    Replacement::draft(0x1425d66c0, "unknown_1425d66c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143843d60, 0x143a00290> as *const ()), // sub_1425d66c0
    Replacement::draft(0x1425d6920, "unknown_1425d6920", tpl_53264::<0x143a0ff20, 0x14021cb50, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_1425d6920
    Replacement::draft(0x1425d6cf0, "unknown_1425d6cf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844130, 0x143a00290> as *const ()), // sub_1425d6cf0
    Replacement::draft(0x1425d6d80, "unknown_1425d6d80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844148, 0x143a00290> as *const ()), // sub_1425d6d80
    Replacement::draft(0x1425d6e10, "unknown_1425d6e10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844118, 0x143a00290> as *const ()), // sub_1425d6e10
    Replacement::draft(0x1425d6ea0, "unknown_1425d6ea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844100, 0x143a00290> as *const ()), // sub_1425d6ea0
    Replacement::draft(0x1425d7310, "unknown_1425d7310", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438442c8, 0x143a00290> as *const ()), // sub_1425d7310
    Replacement::draft(0x1425d7640, "unknown_1425d7640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844310, 0x143a00290> as *const ()), // sub_1425d7640
    Replacement::draft(0x1425d76d0, "unknown_1425d76d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438443d8, 0x143a00290> as *const ()), // sub_1425d76d0
    Replacement::draft(0x1425d7760, "unknown_1425d7760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438443f8, 0x143a00290> as *const ()), // sub_1425d7760
    Replacement::draft(0x1425d77f0, "unknown_1425d77f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844360, 0x143a00290> as *const ()), // sub_1425d77f0
    Replacement::draft(0x1425d7e00, "unknown_1425d7e00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844618, 0x143a00290> as *const ()), // sub_1425d7e00
    Replacement::draft(0x1425d88b0, "unknown_1425d88b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438448a0, 0x143a00290> as *const ()), // sub_1425d88b0
    Replacement::draft(0x1425d8940, "unknown_1425d8940", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844880, 0x143a00290> as *const ()), // sub_1425d8940
    Replacement::draft(0x1425d89d0, "unknown_1425d89d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143844828, 0x143a00290> as *const ()), // sub_1425d89d0
    Replacement::draft(0x1425da840, "unknown_1425da840", tpl_53265::<0x143a16778, 0x8, 0x140267a30, 0x1401e66f0, 0x143a16750, 0x14240ab10> as *const ()), // sub_1425da840
    Replacement::draft(0x1425da880, "unknown_1425da880", tpl_53265::<0x143a16470, 0x8, 0x140267a30, 0x1401e66f0, 0x143a16448, 0x14240ab10> as *const ()), // sub_1425da880
    Replacement::draft(0x1425da8c0, "unknown_1425da8c0", tpl_53265::<0x143a164a8, 0x8, 0x140267a30, 0x1401e66f0, 0x143a16480, 0x14240ab10> as *const ()), // sub_1425da8c0
    Replacement::draft(0x1425db130, "unknown_1425db130", tpl_53265::<0x143a17488, 0x8, 0x140267a30, 0x1401e66f0, 0x143a17460, 0x14240ab10> as *const ()), // sub_1425db130
    Replacement::draft(0x1425db810, "unknown_1425db810", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438456d0, 0x143a00290> as *const ()), // sub_1425db810
    Replacement::draft(0x1425db8a0, "unknown_1425db8a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438457e8, 0x143a00290> as *const ()), // sub_1425db8a0
    Replacement::draft(0x1425db930, "unknown_1425db930", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845820, 0x143a00290> as *const ()), // sub_1425db930
    Replacement::draft(0x1425db9c0, "unknown_1425db9c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438456b0, 0x143a00290> as *const ()), // sub_1425db9c0
    Replacement::draft(0x1425dba50, "unknown_1425dba50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845788, 0x143a00290> as *const ()), // sub_1425dba50
    Replacement::draft(0x1425dbae0, "unknown_1425dbae0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845840, 0x143a00290> as *const ()), // sub_1425dbae0
    Replacement::draft(0x1425dbb70, "unknown_1425dbb70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845748, 0x143a00290> as *const ()), // sub_1425dbb70
    Replacement::draft(0x1425dbc00, "unknown_1425dbc00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845728, 0x143a00290> as *const ()), // sub_1425dbc00
    Replacement::draft(0x1425dbc90, "unknown_1425dbc90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438457c8, 0x143a00290> as *const ()), // sub_1425dbc90
    Replacement::draft(0x1425dbd20, "unknown_1425dbd20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438457a8, 0x143a00290> as *const ()), // sub_1425dbd20
    Replacement::draft(0x1425dbdb0, "unknown_1425dbdb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845708, 0x143a00290> as *const ()), // sub_1425dbdb0
    Replacement::draft(0x1425dbe40, "unknown_1425dbe40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845650, 0x143a00290> as *const ()), // sub_1425dbe40
    Replacement::draft(0x1425dbed0, "unknown_1425dbed0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438456e8, 0x143a00290> as *const ()), // sub_1425dbed0
    Replacement::draft(0x1425dbf60, "unknown_1425dbf60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845800, 0x143a00290> as *const ()), // sub_1425dbf60
    Replacement::draft(0x1425dbff0, "unknown_1425dbff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845670, 0x143a00290> as *const ()), // sub_1425dbff0
    Replacement::draft(0x1425dc080, "unknown_1425dc080", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845630, 0x143a00290> as *const ()), // sub_1425dc080
    Replacement::draft(0x1425dc110, "unknown_1425dc110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845690, 0x143a00290> as *const ()), // sub_1425dc110
    Replacement::draft(0x1425dc1a0, "unknown_1425dc1a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845610, 0x143a00290> as *const ()), // sub_1425dc1a0
    Replacement::draft(0x1425dc370, "unknown_1425dc370", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845a98, 0x143a00290> as *const ()), // sub_1425dc370
    Replacement::draft(0x1425dc400, "unknown_1425dc400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845b40, 0x143a00290> as *const ()), // sub_1425dc400
    Replacement::draft(0x1425dc490, "unknown_1425dc490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845af8, 0x143a00290> as *const ()), // sub_1425dc490
    Replacement::draft(0x1425dc520, "unknown_1425dc520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845cd0, 0x143a00290> as *const ()), // sub_1425dc520
    Replacement::draft(0x1425dc5b0, "unknown_1425dc5b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845a40, 0x143a00290> as *const ()), // sub_1425dc5b0
    Replacement::draft(0x1425dc640, "unknown_1425dc640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438458e0, 0x143a00290> as *const ()), // sub_1425dc640
    Replacement::draft(0x1425dc6d0, "unknown_1425dc6d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845a58, 0x143a00290> as *const ()), // sub_1425dc6d0
    Replacement::draft(0x1425dc760, "unknown_1425dc760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845900, 0x143a00290> as *const ()), // sub_1425dc760
    Replacement::draft(0x1425dc7f0, "unknown_1425dc7f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845cb0, 0x143a00290> as *const ()), // sub_1425dc7f0
    Replacement::draft(0x1425dc880, "unknown_1425dc880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845ae0, 0x143a00290> as *const ()), // sub_1425dc880
    Replacement::draft(0x1425dc910, "unknown_1425dc910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845a00, 0x143a00290> as *const ()), // sub_1425dc910
    Replacement::draft(0x1425dc9a0, "unknown_1425dc9a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845b10, 0x143a00290> as *const ()), // sub_1425dc9a0
    Replacement::draft(0x1425dca30, "unknown_1425dca30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845c98, 0x143a00290> as *const ()), // sub_1425dca30
    Replacement::draft(0x1425dcac0, "unknown_1425dcac0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438458c0, 0x143a00290> as *const ()), // sub_1425dcac0
    Replacement::draft(0x1425dcb50, "unknown_1425dcb50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845a78, 0x143a00290> as *const ()), // sub_1425dcb50
    Replacement::draft(0x1425dcbe0, "unknown_1425dcbe0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845b78, 0x143a00290> as *const ()), // sub_1425dcbe0
    Replacement::draft(0x1425dcc70, "unknown_1425dcc70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845b58, 0x143a00290> as *const ()), // sub_1425dcc70
    Replacement::draft(0x1425dcd00, "unknown_1425dcd00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845b98, 0x143a00290> as *const ()), // sub_1425dcd00
    Replacement::draft(0x1425dcd90, "unknown_1425dcd90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845b28, 0x143a00290> as *const ()), // sub_1425dcd90
    Replacement::draft(0x1425dcf70, "unknown_1425dcf70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845d48, 0x143a00290> as *const ()), // sub_1425dcf70
    Replacement::draft(0x1425dd060, "unknown_1425dd060", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845d90, 0x143a00290> as *const ()), // sub_1425dd060
    Replacement::draft(0x1425dd2e0, "unknown_1425dd2e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845e68, 0x143a00290> as *const ()), // sub_1425dd2e0
    Replacement::draft(0x1425dd3b0, "unknown_1425dd3b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845ee8, 0x143a00290> as *const ()), // sub_1425dd3b0
    Replacement::draft(0x1425dd490, "unknown_1425dd490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845fd0, 0x143a00290> as *const ()), // sub_1425dd490
    Replacement::draft(0x1425dd520, "unknown_1425dd520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845f18, 0x143a00290> as *const ()), // sub_1425dd520
    Replacement::draft(0x1425dd5b0, "unknown_1425dd5b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845ff0, 0x143a00290> as *const ()), // sub_1425dd5b0
    Replacement::draft(0x1425dd640, "unknown_1425dd640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845fb0, 0x143a00290> as *const ()), // sub_1425dd640
    Replacement::draft(0x1425dd6d0, "unknown_1425dd6d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845f38, 0x143a00290> as *const ()), // sub_1425dd6d0
    Replacement::draft(0x1425dd760, "unknown_1425dd760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143845f00, 0x143a00290> as *const ()), // sub_1425dd760
    Replacement::draft(0x1425dd7f0, "unknown_1425dd7f0", tpl_53266::<0x143845fa0, 0x143845f68, 0x20> as *const ()), // sub_1425dd7f0
    Replacement::draft(0x1425de1e0, "unknown_1425de1e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438463f8, 0x143a00290> as *const ()), // sub_1425de1e0
    Replacement::draft(0x1425de270, "unknown_1425de270", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846348, 0x143a00290> as *const ()), // sub_1425de270
    Replacement::draft(0x1425de300, "unknown_1425de300", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846388, 0x143a00290> as *const ()), // sub_1425de300
    Replacement::draft(0x1425de390, "unknown_1425de390", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846310, 0x143a00290> as *const ()), // sub_1425de390
    Replacement::draft(0x1425de420, "unknown_1425de420", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438463a8, 0x143a00290> as *const ()), // sub_1425de420
    Replacement::draft(0x1425de4b0, "unknown_1425de4b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846368, 0x143a00290> as *const ()), // sub_1425de4b0
    Replacement::draft(0x1425de540, "unknown_1425de540", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846328, 0x143a00290> as *const ()), // sub_1425de540
    Replacement::draft(0x1425de5d0, "unknown_1425de5d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438463c0, 0x143a00290> as *const ()), // sub_1425de5d0
    Replacement::draft(0x1425de660, "unknown_1425de660", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438463d8, 0x143a00290> as *const ()), // sub_1425de660
    Replacement::draft(0x1425de6f0, "unknown_1425de6f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846418, 0x143a00290> as *const ()), // sub_1425de6f0
    Replacement::draft(0x1425de850, "unknown_1425de850", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438464d0, 0x143a00290> as *const ()), // sub_1425de850
    Replacement::draft(0x1425dedd0, "unknown_1425dedd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846590, 0x143a00290> as *const ()), // sub_1425dedd0
    Replacement::draft(0x1425df020, "unknown_1425df020", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846768, 0x143a00290> as *const ()), // sub_1425df020
    Replacement::draft(0x1425df0b0, "unknown_1425df0b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438468a8, 0x143a00290> as *const ()), // sub_1425df0b0
    Replacement::draft(0x1425df140, "unknown_1425df140", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846788, 0x143a00290> as *const ()), // sub_1425df140
    Replacement::draft(0x1425df1d0, "unknown_1425df1d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846748, 0x143a00290> as *const ()), // sub_1425df1d0
    Replacement::draft(0x1425df260, "unknown_1425df260", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846870, 0x143a00290> as *const ()), // sub_1425df260
    Replacement::draft(0x1425df2f0, "unknown_1425df2f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438467f0, 0x143a00290> as *const ()), // sub_1425df2f0
    Replacement::draft(0x1425df380, "unknown_1425df380", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846810, 0x143a00290> as *const ()), // sub_1425df380
    Replacement::draft(0x1425df410, "unknown_1425df410", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846730, 0x143a00290> as *const ()), // sub_1425df410
    Replacement::draft(0x1425df4a0, "unknown_1425df4a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846890, 0x143a00290> as *const ()), // sub_1425df4a0
    Replacement::draft(0x1425df530, "unknown_1425df530", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846830, 0x143a00290> as *const ()), // sub_1425df530
    Replacement::draft(0x1425df5c0, "unknown_1425df5c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438467b0, 0x143a00290> as *const ()), // sub_1425df5c0
    Replacement::draft(0x1425dfdd0, "unknown_1425dfdd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846bb8, 0x143a00290> as *const ()), // sub_1425dfdd0
    Replacement::draft(0x1425dfe60, "unknown_1425dfe60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846b98, 0x143a00290> as *const ()), // sub_1425dfe60
    Replacement::draft(0x1425e0130, "unknown_1425e0130", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846de8, 0x143a00290> as *const ()), // sub_1425e0130
    Replacement::draft(0x1425e01c0, "unknown_1425e01c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846db0, 0x143a00290> as *const ()), // sub_1425e01c0
    Replacement::draft(0x1425e0250, "unknown_1425e0250", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846d50, 0x143a00290> as *const ()), // sub_1425e0250
    Replacement::draft(0x1425e02e0, "unknown_1425e02e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846e20, 0x143a00290> as *const ()), // sub_1425e02e0
    Replacement::draft(0x1425e0370, "unknown_1425e0370", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846d90, 0x143a00290> as *const ()), // sub_1425e0370
    Replacement::draft(0x1425e0400, "unknown_1425e0400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846dc8, 0x143a00290> as *const ()), // sub_1425e0400
    Replacement::draft(0x1425e0490, "unknown_1425e0490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846d30, 0x143a00290> as *const ()), // sub_1425e0490
    Replacement::draft(0x1425e0520, "unknown_1425e0520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846d70, 0x143a00290> as *const ()), // sub_1425e0520
    Replacement::draft(0x1425e05b0, "unknown_1425e05b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846d10, 0x143a00290> as *const ()), // sub_1425e05b0
    Replacement::draft(0x1425e0640, "unknown_1425e0640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846e00, 0x143a00290> as *const ()), // sub_1425e0640
    Replacement::draft(0x1425e07b0, "unknown_1425e07b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846fa0, 0x143a00290> as *const ()), // sub_1425e07b0
    Replacement::draft(0x1425e0840, "unknown_1425e0840", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846ed8, 0x143a00290> as *const ()), // sub_1425e0840
    Replacement::draft(0x1425e08d0, "unknown_1425e08d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846ef8, 0x143a00290> as *const ()), // sub_1425e08d0
    Replacement::draft(0x1425e0960, "unknown_1425e0960", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846f60, 0x143a00290> as *const ()), // sub_1425e0960
    Replacement::draft(0x1425e09f0, "unknown_1425e09f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846e40, 0x143a00290> as *const ()), // sub_1425e09f0
    Replacement::draft(0x1425e0a80, "unknown_1425e0a80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846f80, 0x143a00290> as *const ()), // sub_1425e0a80
    Replacement::draft(0x1425e0b10, "unknown_1425e0b10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846e60, 0x143a00290> as *const ()), // sub_1425e0b10
    Replacement::draft(0x1425e0ba0, "unknown_1425e0ba0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143846f18, 0x143a00290> as *const ()), // sub_1425e0ba0
    Replacement::draft(0x1425e0e10, "unknown_1425e0e10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438470e0, 0x143a00290> as *const ()), // sub_1425e0e10
    Replacement::draft(0x1425e0ea0, "unknown_1425e0ea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847140, 0x143a00290> as *const ()), // sub_1425e0ea0
    Replacement::draft(0x1425e0f30, "unknown_1425e0f30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847120, 0x143a00290> as *const ()), // sub_1425e0f30
    Replacement::draft(0x1425e0fc0, "unknown_1425e0fc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847100, 0x143a00290> as *const ()), // sub_1425e0fc0
    Replacement::draft(0x1425e1150, "unknown_1425e1150", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438471c0, 0x143a00290> as *const ()), // sub_1425e1150
    Replacement::draft(0x1425e12f0, "unknown_1425e12f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438473b8, 0x143a00290> as *const ()), // sub_1425e12f0
    Replacement::draft(0x1425e1380, "unknown_1425e1380", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847550, 0x143a00290> as *const ()), // sub_1425e1380
    Replacement::draft(0x1425e1410, "unknown_1425e1410", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847208, 0x143a00290> as *const ()), // sub_1425e1410
    Replacement::draft(0x1425e14a0, "unknown_1425e14a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847518, 0x143a00290> as *const ()), // sub_1425e14a0
    Replacement::draft(0x1425e1530, "unknown_1425e1530", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847588, 0x143a00290> as *const ()), // sub_1425e1530
    Replacement::draft(0x1425e15c0, "unknown_1425e15c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438473d0, 0x143a00290> as *const ()), // sub_1425e15c0
    Replacement::draft(0x1425e1650, "unknown_1425e1650", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847220, 0x143a00290> as *const ()), // sub_1425e1650
    Replacement::draft(0x1425e16e0, "unknown_1425e16e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847328, 0x143a00290> as *const ()), // sub_1425e16e0
    Replacement::draft(0x1425e1770, "unknown_1425e1770", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847348, 0x143a00290> as *const ()), // sub_1425e1770
    Replacement::draft(0x1425e1800, "unknown_1425e1800", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847650, 0x143a00290> as *const ()), // sub_1425e1800
    Replacement::draft(0x1425e1890, "unknown_1425e1890", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847530, 0x143a00290> as *const ()), // sub_1425e1890
    Replacement::draft(0x1425e1920, "unknown_1425e1920", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438474b8, 0x143a00290> as *const ()), // sub_1425e1920
    Replacement::draft(0x1425e19b0, "unknown_1425e19b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847398, 0x143a00290> as *const ()), // sub_1425e19b0
    Replacement::draft(0x1425e1a40, "unknown_1425e1a40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847240, 0x143a00290> as *const ()), // sub_1425e1a40
    Replacement::draft(0x1425e1ad0, "unknown_1425e1ad0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847408, 0x143a00290> as *const ()), // sub_1425e1ad0
    Replacement::draft(0x1425e1b60, "unknown_1425e1b60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847280, 0x143a00290> as *const ()), // sub_1425e1b60
    Replacement::draft(0x1425e1bf0, "unknown_1425e1bf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438475f0, 0x143a00290> as *const ()), // sub_1425e1bf0
    Replacement::draft(0x1425e1c80, "unknown_1425e1c80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438474f8, 0x143a00290> as *const ()), // sub_1425e1c80
    Replacement::draft(0x1425e1d10, "unknown_1425e1d10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438472e8, 0x143a00290> as *const ()), // sub_1425e1d10
    Replacement::draft(0x1425e1da0, "unknown_1425e1da0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847468, 0x143a00290> as *const ()), // sub_1425e1da0
    Replacement::draft(0x1425e1e30, "unknown_1425e1e30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847448, 0x143a00290> as *const ()), // sub_1425e1e30
    Replacement::draft(0x1425e1ec0, "unknown_1425e1ec0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847428, 0x143a00290> as *const ()), // sub_1425e1ec0
    Replacement::draft(0x1425e1f50, "unknown_1425e1f50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847308, 0x143a00290> as *const ()), // sub_1425e1f50
    Replacement::draft(0x1425e1fe0, "unknown_1425e1fe0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438473e8, 0x143a00290> as *const ()), // sub_1425e1fe0
    Replacement::draft(0x1425e2070, "unknown_1425e2070", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847610, 0x143a00290> as *const ()), // sub_1425e2070
    Replacement::draft(0x1425e2100, "unknown_1425e2100", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438472d0, 0x143a00290> as *const ()), // sub_1425e2100
    Replacement::draft(0x1425e2190, "unknown_1425e2190", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438475d8, 0x143a00290> as *const ()), // sub_1425e2190
    Replacement::draft(0x1425e2220, "unknown_1425e2220", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438475b8, 0x143a00290> as *const ()), // sub_1425e2220
    Replacement::draft(0x1425e22b0, "unknown_1425e22b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847630, 0x143a00290> as *const ()), // sub_1425e22b0
    Replacement::draft(0x1425e2340, "unknown_1425e2340", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847260, 0x143a00290> as *const ()), // sub_1425e2340
    Replacement::draft(0x1425e23d0, "unknown_1425e23d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847568, 0x143a00290> as *const ()), // sub_1425e23d0
    Replacement::draft(0x1425e2460, "unknown_1425e2460", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438475a0, 0x143a00290> as *const ()), // sub_1425e2460
    Replacement::draft(0x1425e24f0, "unknown_1425e24f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438474d8, 0x143a00290> as *const ()), // sub_1425e24f0
    Replacement::draft(0x1425e2b60, "unknown_1425e2b60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438477f8, 0x143a00290> as *const ()), // sub_1425e2b60
    Replacement::draft(0x1425e2bf0, "unknown_1425e2bf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438477c0, 0x143a00290> as *const ()), // sub_1425e2bf0
    Replacement::draft(0x1425e2c80, "unknown_1425e2c80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438478a0, 0x143a00290> as *const ()), // sub_1425e2c80
    Replacement::draft(0x1425e3270, "unknown_1425e3270", tpl_53267::<0x1403c7f00, 0xffffffffffffffd3, 0x143847ac0, 0x141834ee8> as *const ()), // sub_1425e3270
    Replacement::draft(0x1425e32a0, "unknown_1425e32a0", tpl_53267::<0x1403c7f00, 0xffffffffffffffd3, 0x143847960, 0x141834ee8> as *const ()), // sub_1425e32a0
    Replacement::draft(0x1425e3990, "unknown_1425e3990", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847cd0, 0x143a00290> as *const ()), // sub_1425e3990
    Replacement::draft(0x1425e3a20, "unknown_1425e3a20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847c30, 0x143a00290> as *const ()), // sub_1425e3a20
    Replacement::draft(0x1425e3ab0, "unknown_1425e3ab0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847c80, 0x143a00290> as *const ()), // sub_1425e3ab0
    Replacement::draft(0x1425e3b40, "unknown_1425e3b40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847cf0, 0x143a00290> as *const ()), // sub_1425e3b40
    Replacement::draft(0x1425e41e0, "unknown_1425e41e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847e10, 0x143a00290> as *const ()), // sub_1425e41e0
    Replacement::draft(0x1425e4270, "unknown_1425e4270", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847e68, 0x143a00290> as *const ()), // sub_1425e4270
    Replacement::draft(0x1425e4300, "unknown_1425e4300", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847db8, 0x143a00290> as *const ()), // sub_1425e4300
    Replacement::draft(0x1425e4390, "unknown_1425e4390", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847e80, 0x143a00290> as *const ()), // sub_1425e4390
    Replacement::draft(0x1425e4420, "unknown_1425e4420", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847e48, 0x143a00290> as *const ()), // sub_1425e4420
    Replacement::draft(0x1425e44b0, "unknown_1425e44b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847dd8, 0x143a00290> as *const ()), // sub_1425e44b0
    Replacement::draft(0x1425e4540, "unknown_1425e4540", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847eb8, 0x143a00290> as *const ()), // sub_1425e4540
    Replacement::draft(0x1425e45d0, "unknown_1425e45d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847ea0, 0x143a00290> as *const ()), // sub_1425e45d0
    Replacement::draft(0x1425e4660, "unknown_1425e4660", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847e28, 0x143a00290> as *const ()), // sub_1425e4660
    Replacement::draft(0x1425e46f0, "unknown_1425e46f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143847df0, 0x143a00290> as *const ()), // sub_1425e46f0
    Replacement::draft(0x1425e5680, "unknown_1425e5680", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848118, 0x143a00290> as *const ()), // sub_1425e5680
    Replacement::draft(0x1425e6c40, "unknown_1425e6c40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438484a8, 0x143a00290> as *const ()), // sub_1425e6c40
    Replacement::draft(0x1425e6cd0, "unknown_1425e6cd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848378, 0x143a00290> as *const ()), // sub_1425e6cd0
    Replacement::draft(0x1425e6d60, "unknown_1425e6d60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848430, 0x143a00290> as *const ()), // sub_1425e6d60
    Replacement::draft(0x1425e6df0, "unknown_1425e6df0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438484c0, 0x143a00290> as *const ()), // sub_1425e6df0
    Replacement::draft(0x1425e6e80, "unknown_1425e6e80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848468, 0x143a00290> as *const ()), // sub_1425e6e80
    Replacement::draft(0x1425e6f10, "unknown_1425e6f10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848488, 0x143a00290> as *const ()), // sub_1425e6f10
    Replacement::draft(0x1425e6fa0, "unknown_1425e6fa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848390, 0x143a00290> as *const ()), // sub_1425e6fa0
    Replacement::draft(0x1425e7030, "unknown_1425e7030", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438483e8, 0x143a00290> as *const ()), // sub_1425e7030
    Replacement::draft(0x1425e70c0, "unknown_1425e70c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848418, 0x143a00290> as *const ()), // sub_1425e70c0
    Replacement::draft(0x1425e7150, "unknown_1425e7150", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438483b0, 0x143a00290> as *const ()), // sub_1425e7150
    Replacement::draft(0x1425e71e0, "unknown_1425e71e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438483c8, 0x143a00290> as *const ()), // sub_1425e71e0
    Replacement::draft(0x1425e7270, "unknown_1425e7270", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848400, 0x143a00290> as *const ()), // sub_1425e7270
    Replacement::draft(0x1425e7300, "unknown_1425e7300", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848448, 0x143a00290> as *const ()), // sub_1425e7300
    Replacement::draft(0x1425e7d20, "unknown_1425e7d20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848648, 0x143a00290> as *const ()), // sub_1425e7d20
    Replacement::draft(0x1425e7e90, "unknown_1425e7e90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438486f8, 0x143a00290> as *const ()), // sub_1425e7e90
    Replacement::draft(0x1425e7f20, "unknown_1425e7f20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848738, 0x143a00290> as *const ()), // sub_1425e7f20
    Replacement::draft(0x1425e7fb0, "unknown_1425e7fb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848718, 0x143a00290> as *const ()), // sub_1425e7fb0
    Replacement::draft(0x1425e8160, "unknown_1425e8160", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848788, 0x143a00290> as *const ()), // sub_1425e8160
    Replacement::draft(0x1425e8310, "unknown_1425e8310", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848890, 0x143a00290> as *const ()), // sub_1425e8310
    Replacement::draft(0x1425e8400, "unknown_1425e8400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438488a8, 0x143a00290> as *const ()), // sub_1425e8400
    Replacement::draft(0x1425e8490, "unknown_1425e8490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438488c0, 0x143a00290> as *const ()), // sub_1425e8490
    Replacement::draft(0x1425e85b0, "unknown_1425e85b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848a50, 0x143a00290> as *const ()), // sub_1425e85b0
    Replacement::draft(0x1425e8640, "unknown_1425e8640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848a90, 0x143a00290> as *const ()), // sub_1425e8640
    Replacement::draft(0x1425e86d0, "unknown_1425e86d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438489d0, 0x143a00290> as *const ()), // sub_1425e86d0
    Replacement::draft(0x1425e8760, "unknown_1425e8760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438489b0, 0x143a00290> as *const ()), // sub_1425e8760
    Replacement::draft(0x1425e87f0, "unknown_1425e87f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438488e0, 0x143a00290> as *const ()), // sub_1425e87f0
    Replacement::draft(0x1425e8880, "unknown_1425e8880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848a08, 0x143a00290> as *const ()), // sub_1425e8880
    Replacement::draft(0x1425e8910, "unknown_1425e8910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848a28, 0x143a00290> as *const ()), // sub_1425e8910
    Replacement::draft(0x1425e89a0, "unknown_1425e89a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438489f0, 0x143a00290> as *const ()), // sub_1425e89a0
    Replacement::draft(0x1425e8a30, "unknown_1425e8a30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848960, 0x143a00290> as *const ()), // sub_1425e8a30
    Replacement::draft(0x1425e9090, "unknown_1425e9090", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848be0, 0x143a00290> as *const ()), // sub_1425e9090
    Replacement::draft(0x1425e9200, "unknown_1425e9200", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848cc0, 0x143a00290> as *const ()), // sub_1425e9200
    Replacement::draft(0x1425e93d0, "unknown_1425e93d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848f80, 0x143a00290> as *const ()), // sub_1425e93d0
    Replacement::draft(0x1425e9460, "unknown_1425e9460", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848d00, 0x143a00290> as *const ()), // sub_1425e9460
    Replacement::draft(0x1425e94f0, "unknown_1425e94f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848f10, 0x143a00290> as *const ()), // sub_1425e94f0
    Replacement::draft(0x1425e9580, "unknown_1425e9580", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848eb0, 0x143a00290> as *const ()), // sub_1425e9580
    Replacement::draft(0x1425e9610, "unknown_1425e9610", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848ce0, 0x143a00290> as *const ()), // sub_1425e9610
    Replacement::draft(0x1425e96a0, "unknown_1425e96a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848e60, 0x143a00290> as *const ()), // sub_1425e96a0
    Replacement::draft(0x1425e9730, "unknown_1425e9730", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848f40, 0x143a00290> as *const ()), // sub_1425e9730
    Replacement::draft(0x1425e9860, "unknown_1425e9860", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848ff0, 0x143a00290> as *const ()), // sub_1425e9860
    Replacement::draft(0x1425e98f0, "unknown_1425e98f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849028, 0x143a00290> as *const ()), // sub_1425e98f0
    Replacement::draft(0x1425e9980, "unknown_1425e9980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848fa0, 0x143a00290> as *const ()), // sub_1425e9980
    Replacement::draft(0x1425e9a10, "unknown_1425e9a10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848fd8, 0x143a00290> as *const ()), // sub_1425e9a10
    Replacement::draft(0x1425e9aa0, "unknown_1425e9aa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849010, 0x143a00290> as *const ()), // sub_1425e9aa0
    Replacement::draft(0x1425e9b30, "unknown_1425e9b30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143848fc0, 0x143a00290> as *const ()), // sub_1425e9b30
    Replacement::draft(0x1425ea0e0, "unknown_1425ea0e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438491a8, 0x143a00290> as *const ()), // sub_1425ea0e0
    Replacement::draft(0x1425ea170, "unknown_1425ea170", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849188, 0x143a00290> as *const ()), // sub_1425ea170
    Replacement::draft(0x1425ea200, "unknown_1425ea200", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849208, 0x143a00290> as *const ()), // sub_1425ea200
    Replacement::draft(0x1425ea290, "unknown_1425ea290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438491c8, 0x143a00290> as *const ()), // sub_1425ea290
    Replacement::draft(0x1425ea8e0, "unknown_1425ea8e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849438, 0x143a00290> as *const ()), // sub_1425ea8e0
    Replacement::draft(0x1425ea970, "unknown_1425ea970", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438493f8, 0x143a00290> as *const ()), // sub_1425ea970
    Replacement::draft(0x1425eaa00, "unknown_1425eaa00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438493e0, 0x143a00290> as *const ()), // sub_1425eaa00
    Replacement::draft(0x1425eaa90, "unknown_1425eaa90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849378, 0x143a00290> as *const ()), // sub_1425eaa90
    Replacement::draft(0x1425eab20, "unknown_1425eab20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438493c0, 0x143a00290> as *const ()), // sub_1425eab20
    Replacement::draft(0x1425eabb0, "unknown_1425eabb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849418, 0x143a00290> as *const ()), // sub_1425eabb0
    Replacement::draft(0x1425eb5b0, "unknown_1425eb5b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849900, 0x143a00290> as *const ()), // sub_1425eb5b0
    Replacement::draft(0x1425eb640, "unknown_1425eb640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438497e0, 0x143a00290> as *const ()), // sub_1425eb640
    Replacement::draft(0x1425eb6d0, "unknown_1425eb6d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849870, 0x143a00290> as *const ()), // sub_1425eb6d0
    Replacement::draft(0x1425eb760, "unknown_1425eb760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849890, 0x143a00290> as *const ()), // sub_1425eb760
    Replacement::draft(0x1425eb7f0, "unknown_1425eb7f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849920, 0x143a00290> as *const ()), // sub_1425eb7f0
    Replacement::draft(0x1425eb880, "unknown_1425eb880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438498b8, 0x143a00290> as *const ()), // sub_1425eb880
    Replacement::draft(0x1425eb910, "unknown_1425eb910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849938, 0x143a00290> as *const ()), // sub_1425eb910
    Replacement::draft(0x1425eb9a0, "unknown_1425eb9a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849840, 0x143a00290> as *const ()), // sub_1425eb9a0
    Replacement::draft(0x1425ec220, "unknown_1425ec220", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849c38, 0x143a00290> as *const ()), // sub_1425ec220
    Replacement::draft(0x1425ec2b0, "unknown_1425ec2b0", fn_1425ec2b0 as *const ()), // sub_1425ec2b0
    Replacement::draft(0x1425ec3a0, "unknown_1425ec3a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849c88, 0x143a00290> as *const ()), // sub_1425ec3a0
    Replacement::draft(0x1425ec430, "unknown_1425ec430", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849ca0, 0x143a00290> as *const ()), // sub_1425ec430
    Replacement::draft(0x1425ec4c0, "unknown_1425ec4c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849d40, 0x143a00290> as *const ()), // sub_1425ec4c0
    Replacement::draft(0x1425ec620, "unknown_1425ec620", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849e18, 0x143a00290> as *const ()), // sub_1425ec620
    Replacement::draft(0x1425ec6d0, "unknown_1425ec6d0", fn_1425ec6d0 as *const ()), // sub_1425ec6d0
    Replacement::draft(0x1425ec980, "unknown_1425ec980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849ea8, 0x143a00290> as *const ()), // sub_1425ec980
    Replacement::draft(0x1425eca10, "unknown_1425eca10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849f78, 0x143a00290> as *const ()), // sub_1425eca10
    Replacement::draft(0x1425ecaa0, "unknown_1425ecaa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849e48, 0x143a00290> as *const ()), // sub_1425ecaa0
    Replacement::draft(0x1425ecb30, "unknown_1425ecb30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849e30, 0x143a00290> as *const ()), // sub_1425ecb30
    Replacement::draft(0x1425ecbc0, "unknown_1425ecbc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849f58, 0x143a00290> as *const ()), // sub_1425ecbc0
    Replacement::draft(0x1425ecc50, "unknown_1425ecc50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849f40, 0x143a00290> as *const ()), // sub_1425ecc50
    Replacement::draft(0x1425ecce0, "unknown_1425ecce0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143849ef8, 0x143a00290> as *const ()), // sub_1425ecce0
    Replacement::draft(0x1425ece00, "unknown_1425ece00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a158, 0x143a00290> as *const ()), // sub_1425ece00
    Replacement::draft(0x1425ece90, "unknown_1425ece90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a1c8, 0x143a00290> as *const ()), // sub_1425ece90
    Replacement::draft(0x1425ecf20, "unknown_1425ecf20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a0e8, 0x143a00290> as *const ()), // sub_1425ecf20
    Replacement::draft(0x1425ecff0, "unknown_1425ecff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a208, 0x143a00290> as *const ()), // sub_1425ecff0
    Replacement::draft(0x1425ed080, "unknown_1425ed080", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a278, 0x143a00290> as *const ()), // sub_1425ed080
    Replacement::draft(0x1425ed110, "unknown_1425ed110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a298, 0x143a00290> as *const ()), // sub_1425ed110
    Replacement::draft(0x1425ed1a0, "unknown_1425ed1a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a258, 0x143a00290> as *const ()), // sub_1425ed1a0
    Replacement::draft(0x1425ed230, "unknown_1425ed230", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a220, 0x143a00290> as *const ()), // sub_1425ed230
    Replacement::draft(0x1425ed2c0, "unknown_1425ed2c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a238, 0x143a00290> as *const ()), // sub_1425ed2c0
    Replacement::draft(0x1425ed8a0, "unknown_1425ed8a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a628, 0x143a00290> as *const ()), // sub_1425ed8a0
    Replacement::draft(0x1425ed930, "unknown_1425ed930", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a680, 0x143a00290> as *const ()), // sub_1425ed930
    Replacement::draft(0x1425ed9c0, "unknown_1425ed9c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a640, 0x143a00290> as *const ()), // sub_1425ed9c0
    Replacement::draft(0x1425eda50, "unknown_1425eda50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a5a8, 0x143a00290> as *const ()), // sub_1425eda50
    Replacement::draft(0x1425edae0, "unknown_1425edae0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a608, 0x143a00290> as *const ()), // sub_1425edae0
    Replacement::draft(0x1425edb70, "unknown_1425edb70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a5e8, 0x143a00290> as *const ()), // sub_1425edb70
    Replacement::draft(0x1425edc00, "unknown_1425edc00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a660, 0x143a00290> as *const ()), // sub_1425edc00
    Replacement::draft(0x1425edc90, "unknown_1425edc90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a5c8, 0x143a00290> as *const ()), // sub_1425edc90
    Replacement::draft(0x1425edd20, "unknown_1425edd20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a588, 0x143a00290> as *const ()), // sub_1425edd20
    Replacement::draft(0x1425ede80, "unknown_1425ede80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a8b0, 0x143a00290> as *const ()), // sub_1425ede80
    Replacement::draft(0x1425edf10, "unknown_1425edf10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a7f0, 0x143a00290> as *const ()), // sub_1425edf10
    Replacement::draft(0x1425edfa0, "unknown_1425edfa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a878, 0x143a00290> as *const ()), // sub_1425edfa0
    Replacement::draft(0x1425ee030, "unknown_1425ee030", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a6f0, 0x143a00290> as *const ()), // sub_1425ee030
    Replacement::draft(0x1425ee0c0, "unknown_1425ee0c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a858, 0x143a00290> as *const ()), // sub_1425ee0c0
    Replacement::draft(0x1425ee150, "unknown_1425ee150", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a7d0, 0x143a00290> as *const ()), // sub_1425ee150
    Replacement::draft(0x1425ee1e0, "unknown_1425ee1e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a6a8, 0x143a00290> as *const ()), // sub_1425ee1e0
    Replacement::draft(0x1425ee270, "unknown_1425ee270", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a790, 0x143a00290> as *const ()), // sub_1425ee270
    Replacement::draft(0x1425ee300, "unknown_1425ee300", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a8c8, 0x143a00290> as *const ()), // sub_1425ee300
    Replacement::draft(0x1425ee390, "unknown_1425ee390", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a890, 0x143a00290> as *const ()), // sub_1425ee390
    Replacement::draft(0x1425ee420, "unknown_1425ee420", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a740, 0x143a00290> as *const ()), // sub_1425ee420
    Replacement::draft(0x1425ee4b0, "unknown_1425ee4b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a6d0, 0x143a00290> as *const ()), // sub_1425ee4b0
    Replacement::draft(0x1425ee540, "unknown_1425ee540", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a838, 0x143a00290> as *const ()), // sub_1425ee540
    Replacement::draft(0x1425ee5d0, "unknown_1425ee5d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384a7b0, 0x143a00290> as *const ()), // sub_1425ee5d0
    Replacement::draft(0x1425ee660, "unknown_1425ee660", fn_1425ee660 as *const ()), // sub_1425ee660
    Replacement::draft(0x1425eeb40, "unknown_1425eeb40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384abe8, 0x143a00290> as *const ()), // sub_1425eeb40
    Replacement::draft(0x1425eebd0, "unknown_1425eebd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384abd0, 0x143a00290> as *const ()), // sub_1425eebd0
    Replacement::draft(0x1425eec60, "unknown_1425eec60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384ac00, 0x143a00290> as *const ()), // sub_1425eec60
    Replacement::draft(0x1425eee40, "unknown_1425eee40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384aca8, 0x143a00290> as *const ()), // sub_1425eee40
    Replacement::draft(0x1425ef1f0, "unknown_1425ef1f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384ae88, 0x143a00290> as *const ()), // sub_1425ef1f0
    Replacement::draft(0x1425ef280, "unknown_1425ef280", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384aee8, 0x143a00290> as *const ()), // sub_1425ef280
    Replacement::draft(0x1425ef310, "unknown_1425ef310", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384aeb8, 0x143a00290> as *const ()), // sub_1425ef310
    Replacement::draft(0x1425ef3a0, "unknown_1425ef3a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384aea0, 0x143a00290> as *const ()), // sub_1425ef3a0
    Replacement::draft(0x1425ef430, "unknown_1425ef430", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384ae70, 0x143a00290> as *const ()), // sub_1425ef430
    Replacement::draft(0x1425ef4c0, "unknown_1425ef4c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384aed0, 0x143a00290> as *const ()), // sub_1425ef4c0
    Replacement::draft(0x1425ef720, "unknown_1425ef720", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b1d0, 0x143a00290> as *const ()), // sub_1425ef720
    Replacement::draft(0x1425ef7b0, "unknown_1425ef7b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b070, 0x143a00290> as *const ()), // sub_1425ef7b0
    Replacement::draft(0x1425ef840, "unknown_1425ef840", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b0b0, 0x143a00290> as *const ()), // sub_1425ef840
    Replacement::draft(0x1425ef8d0, "unknown_1425ef8d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b190, 0x143a00290> as *const ()), // sub_1425ef8d0
    Replacement::draft(0x1425ef960, "unknown_1425ef960", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b130, 0x143a00290> as *const ()), // sub_1425ef960
    Replacement::draft(0x1425ef9f0, "unknown_1425ef9f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b110, 0x143a00290> as *const ()), // sub_1425ef9f0
    Replacement::draft(0x1425efa80, "unknown_1425efa80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b0d0, 0x143a00290> as *const ()), // sub_1425efa80
    Replacement::draft(0x1425efb10, "unknown_1425efb10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b090, 0x143a00290> as *const ()), // sub_1425efb10
    Replacement::draft(0x1425efba0, "unknown_1425efba0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b150, 0x143a00290> as *const ()), // sub_1425efba0
    Replacement::draft(0x1425efc30, "unknown_1425efc30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b1b0, 0x143a00290> as *const ()), // sub_1425efc30
    Replacement::draft(0x1425efcc0, "unknown_1425efcc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b170, 0x143a00290> as *const ()), // sub_1425efcc0
    Replacement::draft(0x1425efd50, "unknown_1425efd50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b0f0, 0x143a00290> as *const ()), // sub_1425efd50
    Replacement::draft(0x1425efde0, "unknown_1425efde0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b050, 0x143a00290> as *const ()), // sub_1425efde0
    Replacement::draft(0x1425efe70, "unknown_1425efe70", fn_1425efe70 as *const ()), // sub_1425efe70
    Replacement::draft(0x1425f0100, "unknown_1425f0100", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b2c0, 0x143a00290> as *const ()), // sub_1425f0100
    Replacement::draft(0x1425f0190, "unknown_1425f0190", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b2e0, 0x143a00290> as *const ()), // sub_1425f0190
    Replacement::draft(0x1425f0220, "unknown_1425f0220", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b300, 0x143a00290> as *const ()), // sub_1425f0220
    Replacement::draft(0x1425f02b0, "unknown_1425f02b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b2a0, 0x143a00290> as *const ()), // sub_1425f02b0
    Replacement::draft(0x1425f0340, "unknown_1425f0340", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b338, 0x143a00290> as *const ()), // sub_1425f0340
    Replacement::draft(0x1425f03d0, "unknown_1425f03d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b280, 0x143a00290> as *const ()), // sub_1425f03d0
    Replacement::draft(0x1425f0460, "unknown_1425f0460", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b358, 0x143a00290> as *const ()), // sub_1425f0460
    Replacement::draft(0x1425f04f0, "unknown_1425f04f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b390, 0x143a00290> as *const ()), // sub_1425f04f0
    Replacement::draft(0x1425f0580, "unknown_1425f0580", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b378, 0x143a00290> as *const ()), // sub_1425f0580
    Replacement::draft(0x1425f0610, "unknown_1425f0610", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b320, 0x143a00290> as *const ()), // sub_1425f0610
    Replacement::draft(0x1425f09f0, "unknown_1425f09f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b6f8, 0x143a00290> as *const ()), // sub_1425f09f0
    Replacement::draft(0x1425f0a80, "unknown_1425f0a80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b678, 0x143a00290> as *const ()), // sub_1425f0a80
    Replacement::draft(0x1425f0b10, "unknown_1425f0b10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b608, 0x143a00290> as *const ()), // sub_1425f0b10
    Replacement::draft(0x1425f0ba0, "unknown_1425f0ba0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b640, 0x143a00290> as *const ()), // sub_1425f0ba0
    Replacement::draft(0x1425f0c30, "unknown_1425f0c30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b6b8, 0x143a00290> as *const ()), // sub_1425f0c30
    Replacement::draft(0x1425f0cc0, "unknown_1425f0cc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b698, 0x143a00290> as *const ()), // sub_1425f0cc0
    Replacement::draft(0x1425f0d50, "unknown_1425f0d50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b710, 0x143a00290> as *const ()), // sub_1425f0d50
    Replacement::draft(0x1425f0de0, "unknown_1425f0de0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b6d8, 0x143a00290> as *const ()), // sub_1425f0de0
    Replacement::draft(0x1425f0e70, "unknown_1425f0e70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b658, 0x143a00290> as *const ()), // sub_1425f0e70
    Replacement::draft(0x1425f0f00, "unknown_1425f0f00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b620, 0x143a00290> as *const ()), // sub_1425f0f00
    Replacement::draft(0x1425f1460, "unknown_1425f1460", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b7e0, 0x143a00290> as *const ()), // sub_1425f1460
    Replacement::draft(0x1425f14f0, "unknown_1425f14f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b790, 0x143a00290> as *const ()), // sub_1425f14f0
    Replacement::draft(0x1425f16f0, "unknown_1425f16f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b860, 0x143a00290> as *const ()), // sub_1425f16f0
    Replacement::draft(0x1425f1780, "unknown_1425f1780", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b8c8, 0x143a00290> as *const ()), // sub_1425f1780
    Replacement::draft(0x1425f1810, "unknown_1425f1810", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b8a8, 0x143a00290> as *const ()), // sub_1425f1810
    Replacement::draft(0x1425f1a40, "unknown_1425f1a40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b9a8, 0x143a00290> as *const ()), // sub_1425f1a40
    Replacement::draft(0x1425f1ad0, "unknown_1425f1ad0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b958, 0x143a00290> as *const ()), // sub_1425f1ad0
    Replacement::draft(0x1425f1b60, "unknown_1425f1b60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b910, 0x143a00290> as *const ()), // sub_1425f1b60
    Replacement::draft(0x1425f1d50, "unknown_1425f1d50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384b9f8, 0x143a00290> as *const ()), // sub_1425f1d50
    Replacement::draft(0x1425f1ed0, "unknown_1425f1ed0", tpl_53272::<0x143a2b640, 0x14022a610, 0x1434077a8, 0x143a2b630, 0x143a001ac, 0x143a00290> as *const ()), // sub_1425f1ed0
    Replacement::draft(0x1425f1f60, "unknown_1425f1f60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384bba8, 0x143a00290> as *const ()), // sub_1425f1f60
    Replacement::draft(0x1425f1ff0, "unknown_1425f1ff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384bbc8, 0x143a00290> as *const ()), // sub_1425f1ff0
    Replacement::draft(0x1425f2080, "unknown_1425f2080", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384bb50, 0x143a00290> as *const ()), // sub_1425f2080
    Replacement::draft(0x1425f2110, "unknown_1425f2110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384bb70, 0x143a00290> as *const ()), // sub_1425f2110
    Replacement::draft(0x1425f2c00, "unknown_1425f2c00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c050, 0x143a00290> as *const ()), // sub_1425f2c00
    Replacement::draft(0x1425f2c90, "unknown_1425f2c90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384bf70, 0x143a00290> as *const ()), // sub_1425f2c90
    Replacement::draft(0x1425f2d20, "unknown_1425f2d20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c070, 0x143a00290> as *const ()), // sub_1425f2d20
    Replacement::draft(0x1425f2db0, "unknown_1425f2db0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384bfe0, 0x143a00290> as *const ()), // sub_1425f2db0
    Replacement::draft(0x1425f2e40, "unknown_1425f2e40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384bfc0, 0x143a00290> as *const ()), // sub_1425f2e40
    Replacement::draft(0x1425f2ed0, "unknown_1425f2ed0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384bf20, 0x143a00290> as *const ()), // sub_1425f2ed0
    Replacement::draft(0x1425f3050, "unknown_1425f3050", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c0f0, 0x143a00290> as *const ()), // sub_1425f3050
    Replacement::draft(0x1425f30e0, "unknown_1425f30e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c1a0, 0x143a00290> as *const ()), // sub_1425f30e0
    Replacement::draft(0x1425f3170, "unknown_1425f3170", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c110, 0x143a00290> as *const ()), // sub_1425f3170
    Replacement::draft(0x1425f3200, "unknown_1425f3200", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c180, 0x143a00290> as *const ()), // sub_1425f3200
    Replacement::draft(0x1425f3290, "unknown_1425f3290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c160, 0x143a00290> as *const ()), // sub_1425f3290
    Replacement::draft(0x1425f3420, "unknown_1425f3420", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c1c0, 0x143a00290> as *const ()), // sub_1425f3420
    Replacement::draft(0x1425f35c0, "unknown_1425f35c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c238, 0x143a00290> as *const ()), // sub_1425f35c0
    Replacement::draft(0x1425f3650, "unknown_1425f3650", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c250, 0x143a00290> as *const ()), // sub_1425f3650
    Replacement::draft(0x1425f36e0, "unknown_1425f36e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c2a0, 0x143a00290> as *const ()), // sub_1425f36e0
    Replacement::draft(0x1425f3770, "unknown_1425f3770", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c2c0, 0x143a00290> as *const ()), // sub_1425f3770
    Replacement::draft(0x1425f3830, "unknown_1425f3830", tpl_53273::<0x143a2c9c0, 0x1405720a0, 0x1425371a0, 0x142533200> as *const ()), // sub_1425f3830
    Replacement::draft(0x1425f3990, "unknown_1425f3990", tpl_53273::<0x143a2c9b0, 0x1405720a0, 0x1425371a0, 0x142533200> as *const ()), // sub_1425f3990
    Replacement::draft(0x1425f3bb0, "unknown_1425f3bb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c3f0, 0x143a00290> as *const ()), // sub_1425f3bb0
    Replacement::draft(0x1425f3e30, "unknown_1425f3e30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c528, 0x143a00290> as *const ()), // sub_1425f3e30
    Replacement::draft(0x1425f3ec0, "unknown_1425f3ec0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c540, 0x143a00290> as *const ()), // sub_1425f3ec0
    Replacement::draft(0x1425f4030, "unknown_1425f4030", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c5f8, 0x143a00290> as *const ()), // sub_1425f4030
    Replacement::draft(0x1425f4660, "unknown_1425f4660", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c760, 0x143a00290> as *const ()), // sub_1425f4660
    Replacement::draft(0x1425f46f0, "unknown_1425f46f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c718, 0x143a00290> as *const ()), // sub_1425f46f0
    Replacement::draft(0x1425f49d0, "unknown_1425f49d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384c8c0, 0x143a00290> as *const ()), // sub_1425f49d0
    Replacement::draft(0x1425f5410, "unknown_1425f5410", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384ca00, 0x143a00290> as *const ()), // sub_1425f5410
    Replacement::draft(0x1425f55c0, "unknown_1425f55c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cb38, 0x143a00290> as *const ()), // sub_1425f55c0
    Replacement::draft(0x1425f5650, "unknown_1425f5650", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cb50, 0x143a00290> as *const ()), // sub_1425f5650
    Replacement::draft(0x1425f56e0, "unknown_1425f56e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cb20, 0x143a00290> as *const ()), // sub_1425f56e0
    Replacement::draft(0x1425f5770, "unknown_1425f5770", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cb00, 0x143a00290> as *const ()), // sub_1425f5770
    Replacement::draft(0x1425f59c0, "unknown_1425f59c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cd68, 0x143a00290> as *const ()), // sub_1425f59c0
    Replacement::draft(0x1425f5a50, "unknown_1425f5a50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cd48, 0x143a00290> as *const ()), // sub_1425f5a50
    Replacement::draft(0x1425f5ae0, "unknown_1425f5ae0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cde0, 0x143a00290> as *const ()), // sub_1425f5ae0
    Replacement::draft(0x1425f5b70, "unknown_1425f5b70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cd80, 0x143a00290> as *const ()), // sub_1425f5b70
    Replacement::draft(0x1425f5c00, "unknown_1425f5c00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cdf8, 0x143a00290> as *const ()), // sub_1425f5c00
    Replacement::draft(0x1425f5c90, "unknown_1425f5c90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cd28, 0x143a00290> as *const ()), // sub_1425f5c90
    Replacement::draft(0x1425f5d20, "unknown_1425f5d20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384ce18, 0x143a00290> as *const ()), // sub_1425f5d20
    Replacement::draft(0x1425f5db0, "unknown_1425f5db0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cdc8, 0x143a00290> as *const ()), // sub_1425f5db0
    Replacement::draft(0x1425f5e40, "unknown_1425f5e40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cd10, 0x143a00290> as *const ()), // sub_1425f5e40
    Replacement::draft(0x1425f5f60, "unknown_1425f5f60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cf50, 0x143a00290> as *const ()), // sub_1425f5f60
    Replacement::draft(0x1425f5ff0, "unknown_1425f5ff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d070, 0x143a00290> as *const ()), // sub_1425f5ff0
    Replacement::draft(0x1425f6080, "unknown_1425f6080", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cf30, 0x143a00290> as *const ()), // sub_1425f6080
    Replacement::draft(0x1425f6110, "unknown_1425f6110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cf10, 0x143a00290> as *const ()), // sub_1425f6110
    Replacement::draft(0x1425f61a0, "unknown_1425f61a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d050, 0x143a00290> as *const ()), // sub_1425f61a0
    Replacement::draft(0x1425f6230, "unknown_1425f6230", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384cfd0, 0x143a00290> as *const ()), // sub_1425f6230
    Replacement::draft(0x1425f62c0, "unknown_1425f62c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d088, 0x143a00290> as *const ()), // sub_1425f62c0
    Replacement::draft(0x1425f6350, "unknown_1425f6350", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384ce30, 0x143a00290> as *const ()), // sub_1425f6350
    Replacement::draft(0x1425f6550, "unknown_1425f6550", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d128, 0x143a00290> as *const ()), // sub_1425f6550
    Replacement::draft(0x1425f65e0, "unknown_1425f65e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d170, 0x143a00290> as *const ()), // sub_1425f65e0
    Replacement::draft(0x1425f6670, "unknown_1425f6670", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d0a8, 0x143a00290> as *const ()), // sub_1425f6670
    Replacement::draft(0x1425f6700, "unknown_1425f6700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d188, 0x143a00290> as *const ()), // sub_1425f6700
    Replacement::draft(0x1425f6790, "unknown_1425f6790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d1a8, 0x143a00290> as *const ()), // sub_1425f6790
    Replacement::draft(0x1425f7120, "unknown_1425f7120", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d538, 0x143a00290> as *const ()), // sub_1425f7120
    Replacement::draft(0x1425f71b0, "unknown_1425f71b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d550, 0x143a00290> as *const ()), // sub_1425f71b0
    Replacement::draft(0x1425f7240, "unknown_1425f7240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d568, 0x143a00290> as *const ()), // sub_1425f7240
    Replacement::draft(0x1425f72d0, "unknown_1425f72d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d588, 0x143a00290> as *const ()), // sub_1425f72d0
    Replacement::draft(0x1425f7460, "unknown_1425f7460", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d5f0, 0x143a00290> as *const ()), // sub_1425f7460
    Replacement::draft(0x1425f74f0, "unknown_1425f74f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d678, 0x143a00290> as *const ()), // sub_1425f74f0
    Replacement::draft(0x1425f75c0, "unknown_1425f75c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d698, 0x143a00290> as *const ()), // sub_1425f75c0
    Replacement::draft(0x1425f7880, "unknown_1425f7880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d818, 0x143a00290> as *const ()), // sub_1425f7880
    Replacement::draft(0x1425f7910, "unknown_1425f7910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d780, 0x143a00290> as *const ()), // sub_1425f7910
    Replacement::draft(0x1425f79a0, "unknown_1425f79a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d7c0, 0x143a00290> as *const ()), // sub_1425f79a0
    Replacement::draft(0x1425f7a30, "unknown_1425f7a30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d7f8, 0x143a00290> as *const ()), // sub_1425f7a30
    Replacement::draft(0x1425f7ac0, "unknown_1425f7ac0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d7a0, 0x143a00290> as *const ()), // sub_1425f7ac0
    Replacement::draft(0x1425f7b50, "unknown_1425f7b50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d760, 0x143a00290> as *const ()), // sub_1425f7b50
    Replacement::draft(0x1425f7be0, "unknown_1425f7be0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d740, 0x143a00290> as *const ()), // sub_1425f7be0
    Replacement::draft(0x1425f7c70, "unknown_1425f7c70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d7e0, 0x143a00290> as *const ()), // sub_1425f7c70
    Replacement::draft(0x1425f7e40, "unknown_1425f7e40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d8c8, 0x143a00290> as *const ()), // sub_1425f7e40
    Replacement::draft(0x1425f7ed0, "unknown_1425f7ed0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d960, 0x143a00290> as *const ()), // sub_1425f7ed0
    Replacement::draft(0x1425f7f60, "unknown_1425f7f60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d8e0, 0x143a00290> as *const ()), // sub_1425f7f60
    Replacement::draft(0x1425f7ff0, "unknown_1425f7ff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d940, 0x143a00290> as *const ()), // sub_1425f7ff0
    Replacement::draft(0x1425f8080, "unknown_1425f8080", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d920, 0x143a00290> as *const ()), // sub_1425f8080
    Replacement::draft(0x1425f8110, "unknown_1425f8110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384d900, 0x143a00290> as *const ()), // sub_1425f8110
    Replacement::draft(0x1425f82d0, "unknown_1425f82d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384da30, 0x143a00290> as *const ()), // sub_1425f82d0
    Replacement::draft(0x1425f8360, "unknown_1425f8360", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384da68, 0x143a00290> as *const ()), // sub_1425f8360
    Replacement::draft(0x1425f83f0, "unknown_1425f83f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384da88, 0x143a00290> as *const ()), // sub_1425f83f0
    Replacement::draft(0x1425f8480, "unknown_1425f8480", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384da10, 0x143a00290> as *const ()), // sub_1425f8480
    Replacement::draft(0x1425f8510, "unknown_1425f8510", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384da48, 0x143a00290> as *const ()), // sub_1425f8510
    Replacement::draft(0x1425f86c0, "unknown_1425f86c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dbf8, 0x143a00290> as *const ()), // sub_1425f86c0
    Replacement::draft(0x1425f8750, "unknown_1425f8750", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dc18, 0x143a00290> as *const ()), // sub_1425f8750
    Replacement::draft(0x1425f87e0, "unknown_1425f87e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dbb8, 0x143a00290> as *const ()), // sub_1425f87e0
    Replacement::draft(0x1425f8870, "unknown_1425f8870", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384daa8, 0x143a00290> as *const ()), // sub_1425f8870
    Replacement::draft(0x1425f8900, "unknown_1425f8900", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384db98, 0x143a00290> as *const ()), // sub_1425f8900
    Replacement::draft(0x1425f8990, "unknown_1425f8990", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384db28, 0x143a00290> as *const ()), // sub_1425f8990
    Replacement::draft(0x1425f8a20, "unknown_1425f8a20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dbd8, 0x143a00290> as *const ()), // sub_1425f8a20
    Replacement::draft(0x1425f8ab0, "unknown_1425f8ab0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384db48, 0x143a00290> as *const ()), // sub_1425f8ab0
    Replacement::draft(0x1425f8b80, "unknown_1425f8b80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dc50, 0x143a00290> as *const ()), // sub_1425f8b80
    Replacement::draft(0x1425f8c10, "unknown_1425f8c10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dc38, 0x143a00290> as *const ()), // sub_1425f8c10
    Replacement::draft(0x1425f8dc0, "unknown_1425f8dc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dd60, 0x143a00290> as *const ()), // sub_1425f8dc0
    Replacement::draft(0x1425f8e50, "unknown_1425f8e50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dcf8, 0x143a00290> as *const ()), // sub_1425f8e50
    Replacement::draft(0x1425f8ee0, "unknown_1425f8ee0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dcd8, 0x143a00290> as *const ()), // sub_1425f8ee0
    Replacement::draft(0x1425f8f70, "unknown_1425f8f70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dd78, 0x143a00290> as *const ()), // sub_1425f8f70
    Replacement::draft(0x1425f9000, "unknown_1425f9000", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dd98, 0x143a00290> as *const ()), // sub_1425f9000
    Replacement::draft(0x1425f9090, "unknown_1425f9090", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dd48, 0x143a00290> as *const ()), // sub_1425f9090
    Replacement::draft(0x1425f91b0, "unknown_1425f91b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384de98, 0x143a00290> as *const ()), // sub_1425f91b0
    Replacement::draft(0x1425f9240, "unknown_1425f9240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384de18, 0x143a00290> as *const ()), // sub_1425f9240
    Replacement::draft(0x1425f92d0, "unknown_1425f92d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384ddb8, 0x143a00290> as *const ()), // sub_1425f92d0
    Replacement::draft(0x1425f9360, "unknown_1425f9360", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dde8, 0x143a00290> as *const ()), // sub_1425f9360
    Replacement::draft(0x1425f93f0, "unknown_1425f93f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384de48, 0x143a00290> as *const ()), // sub_1425f93f0
    Replacement::draft(0x1425f9480, "unknown_1425f9480", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384de78, 0x143a00290> as *const ()), // sub_1425f9480
    Replacement::draft(0x1425f9570, "unknown_1425f9570", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384ded0, 0x143a00290> as *const ()), // sub_1425f9570
    Replacement::draft(0x1425f9600, "unknown_1425f9600", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384df38, 0x143a00290> as *const ()), // sub_1425f9600
    Replacement::draft(0x1425f9690, "unknown_1425f9690", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384deb0, 0x143a00290> as *const ()), // sub_1425f9690
    Replacement::draft(0x1425f9720, "unknown_1425f9720", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384def0, 0x143a00290> as *const ()), // sub_1425f9720
    Replacement::draft(0x1425f97b0, "unknown_1425f97b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384df18, 0x143a00290> as *const ()), // sub_1425f97b0
    Replacement::draft(0x1425f98d0, "unknown_1425f98d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384df58, 0x143a00290> as *const ()), // sub_1425f98d0
    Replacement::draft(0x1425f9960, "unknown_1425f9960", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384df70, 0x143a00290> as *const ()), // sub_1425f9960
    Replacement::draft(0x1425f99f0, "unknown_1425f99f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384df88, 0x143a00290> as *const ()), // sub_1425f99f0
    Replacement::draft(0x1425f9aa0, "unknown_1425f9aa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dfa0, 0x143a00290> as *const ()), // sub_1425f9aa0
    Replacement::draft(0x1425f9b30, "unknown_1425f9b30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384dfb8, 0x143a00290> as *const ()), // sub_1425f9b30
    Replacement::draft(0x1425f9d20, "unknown_1425f9d20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e000, 0x143a00290> as *const ()), // sub_1425f9d20
    Replacement::draft(0x1425f9e30, "unknown_1425f9e30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e048, 0x143a00290> as *const ()), // sub_1425f9e30
    Replacement::draft(0x1425f9ef0, "unknown_1425f9ef0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e060, 0x143a00290> as *const ()), // sub_1425f9ef0
    Replacement::draft(0x1425f9f80, "unknown_1425f9f80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e078, 0x143a00290> as *const ()), // sub_1425f9f80
    Replacement::draft(0x1425fa010, "unknown_1425fa010", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e090, 0x143a00290> as *const ()), // sub_1425fa010
    Replacement::draft(0x1425fa100, "unknown_1425fa100", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e0a8, 0x143a00290> as *const ()), // sub_1425fa100
    Replacement::draft(0x1425fa290, "unknown_1425fa290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e0c8, 0x143a00290> as *const ()), // sub_1425fa290
    Replacement::draft(0x1425fa330, "unknown_1425fa330", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e0e0, 0x143a00290> as *const ()), // sub_1425fa330
    Replacement::draft(0x1425fa3c0, "unknown_1425fa3c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e0f8, 0x143a00290> as *const ()), // sub_1425fa3c0
    Replacement::draft(0x1425fa470, "unknown_1425fa470", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e110, 0x143a00290> as *const ()), // sub_1425fa470
    Replacement::draft(0x1425fa500, "unknown_1425fa500", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e128, 0x143a00290> as *const ()), // sub_1425fa500
    Replacement::draft(0x1425fa5c0, "unknown_1425fa5c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384e140, 0x143a00290> as *const ()), // sub_1425fa5c0
    Replacement::draft(0x1425fa670, "unknown_1425fa670", fn_1425fa670 as *const ()), // sub_1425fa670
    Replacement::draft(0x1425fa690, "unknown_1425fa690", fn_1425fa690 as *const ()), // sub_1425fa690
    Replacement::draft(0x1425fa6b0, "unknown_1425fa6b0", fn_1425fa6b0 as *const ()), // sub_1425fa6b0
    Replacement::draft(0x1425fa6d0, "unknown_1425fa6d0", fn_1425fa6d0 as *const ()), // sub_1425fa6d0
    Replacement::draft(0x1425fa6f0, "unknown_1425fa6f0", fn_1425fa6f0 as *const ()), // sub_1425fa6f0
    Replacement::draft(0x1425fa710, "unknown_1425fa710", fn_1425fa710 as *const ()), // sub_1425fa710
    Replacement::draft(0x1425fa730, "unknown_1425fa730", fn_1425fa730 as *const ()), // sub_1425fa730
    Replacement::draft(0x1425fa750, "unknown_1425fa750", fn_1425fa750 as *const ()), // sub_1425fa750
    Replacement::draft(0x1425fa770, "unknown_1425fa770", fn_1425fa770 as *const ()), // sub_1425fa770
    Replacement::draft(0x1425fa790, "unknown_1425fa790", fn_1425fa790 as *const ()), // sub_1425fa790
    Replacement::draft(0x1425fa7b0, "unknown_1425fa7b0", fn_1425fa7b0 as *const ()), // sub_1425fa7b0
    Replacement::draft(0x1425fa7d0, "unknown_1425fa7d0", fn_1425fa7d0 as *const ()), // sub_1425fa7d0
    Replacement::draft(0x1425fa7f0, "unknown_1425fa7f0", fn_1425fa7f0 as *const ()), // sub_1425fa7f0
    Replacement::draft(0x1425fa810, "unknown_1425fa810", fn_1425fa810 as *const ()), // sub_1425fa810
    Replacement::draft(0x1425fa830, "unknown_1425fa830", fn_1425fa830 as *const ()), // sub_1425fa830
    Replacement::draft(0x1425fa850, "unknown_1425fa850", fn_1425fa850 as *const ()), // sub_1425fa850
    Replacement::draft(0x1425fa870, "unknown_1425fa870", fn_1425fa870 as *const ()), // sub_1425fa870
    Replacement::draft(0x1425fa890, "unknown_1425fa890", fn_1425fa890 as *const ()), // sub_1425fa890
    Replacement::draft(0x1425fa8b0, "unknown_1425fa8b0", fn_1425fa8b0 as *const ()), // sub_1425fa8b0
    Replacement::draft(0x1425fa8d0, "unknown_1425fa8d0", fn_1425fa8d0 as *const ()), // sub_1425fa8d0
    Replacement::draft(0x1425fa8f0, "unknown_1425fa8f0", fn_1425fa8f0 as *const ()), // sub_1425fa8f0
    Replacement::draft(0x1425fa910, "unknown_1425fa910", fn_1425fa910 as *const ()), // sub_1425fa910
    Replacement::draft(0x1425fa930, "unknown_1425fa930", fn_1425fa930 as *const ()), // sub_1425fa930
    Replacement::draft(0x1425fa950, "unknown_1425fa950", fn_1425fa950 as *const ()), // sub_1425fa950
    Replacement::draft(0x1425fa970, "unknown_1425fa970", fn_1425fa970 as *const ()), // sub_1425fa970
    Replacement::draft(0x1425fa990, "unknown_1425fa990", fn_1425fa990 as *const ()), // sub_1425fa990
    Replacement::draft(0x1425fa9b0, "unknown_1425fa9b0", fn_1425fa9b0 as *const ()), // sub_1425fa9b0
    Replacement::draft(0x1425fa9d0, "unknown_1425fa9d0", fn_1425fa9d0 as *const ()), // sub_1425fa9d0
    Replacement::draft(0x1425fa9f0, "unknown_1425fa9f0", fn_1425fa9f0 as *const ()), // sub_1425fa9f0
    Replacement::draft(0x1425faa10, "unknown_1425faa10", fn_1425faa10 as *const ()), // sub_1425faa10
    Replacement::draft(0x1425faa30, "unknown_1425faa30", fn_1425faa30 as *const ()), // sub_1425faa30
    Replacement::draft(0x1425faa50, "unknown_1425faa50", fn_1425faa50 as *const ()), // sub_1425faa50
    Replacement::draft(0x1425faa70, "unknown_1425faa70", fn_1425faa70 as *const ()), // sub_1425faa70
    Replacement::draft(0x1425faa90, "unknown_1425faa90", fn_1425faa90 as *const ()), // sub_1425faa90
    Replacement::draft(0x1425faaf0, "unknown_1425faaf0", fn_1425faaf0 as *const ()), // sub_1425faaf0
    Replacement::draft(0x1425fab10, "unknown_1425fab10", fn_1425fab10 as *const ()), // sub_1425fab10
    Replacement::draft(0x1425fab30, "unknown_1425fab30", fn_1425fab30 as *const ()), // sub_1425fab30
    Replacement::draft(0x1425fab50, "unknown_1425fab50", fn_1425fab50 as *const ()), // sub_1425fab50
    Replacement::draft(0x1425fab70, "unknown_1425fab70", fn_1425fab70 as *const ()), // sub_1425fab70
    Replacement::draft(0x1425fab90, "unknown_1425fab90", fn_1425fab90 as *const ()), // sub_1425fab90
    Replacement::draft(0x1425fabb0, "unknown_1425fabb0", fn_1425fabb0 as *const ()), // sub_1425fabb0
    Replacement::draft(0x1425fabd0, "unknown_1425fabd0", fn_1425fabd0 as *const ()), // sub_1425fabd0
    Replacement::draft(0x1425fabf0, "unknown_1425fabf0", fn_1425fabf0 as *const ()), // sub_1425fabf0
    Replacement::draft(0x1425fac10, "unknown_1425fac10", fn_1425fac10 as *const ()), // sub_1425fac10
    Replacement::draft(0x1425fac30, "unknown_1425fac30", fn_1425fac30 as *const ()), // sub_1425fac30
    Replacement::draft(0x1425fac50, "unknown_1425fac50", fn_1425fac50 as *const ()), // sub_1425fac50
    Replacement::draft(0x1425fac70, "unknown_1425fac70", fn_1425fac70 as *const ()), // sub_1425fac70
    Replacement::draft(0x1425fac90, "unknown_1425fac90", fn_1425fac90 as *const ()), // sub_1425fac90
    Replacement::draft(0x1425facb0, "unknown_1425facb0", fn_1425facb0 as *const ()), // sub_1425facb0
    Replacement::draft(0x1425facd0, "unknown_1425facd0", fn_1425facd0 as *const ()), // sub_1425facd0
    Replacement::draft(0x1425facf0, "unknown_1425facf0", fn_1425facf0 as *const ()), // sub_1425facf0
    Replacement::draft(0x1425fad10, "unknown_1425fad10", fn_1425fad10 as *const ()), // sub_1425fad10
    Replacement::draft(0x1425fad30, "unknown_1425fad30", fn_1425fad30 as *const ()), // sub_1425fad30
    Replacement::draft(0x1425fad50, "unknown_1425fad50", fn_1425fad50 as *const ()), // sub_1425fad50
    Replacement::draft(0x1425fad70, "unknown_1425fad70", fn_1425fad70 as *const ()), // sub_1425fad70
    Replacement::draft(0x1425fad90, "unknown_1425fad90", fn_1425fad90 as *const ()), // sub_1425fad90
    Replacement::draft(0x1425fadb0, "unknown_1425fadb0", fn_1425fadb0 as *const ()), // sub_1425fadb0
    Replacement::draft(0x1425fadd0, "unknown_1425fadd0", fn_1425fadd0 as *const ()), // sub_1425fadd0
    Replacement::draft(0x1425fadf0, "unknown_1425fadf0", fn_1425fadf0 as *const ()), // sub_1425fadf0
    Replacement::draft(0x1425fae10, "unknown_1425fae10", fn_1425fae10 as *const ()), // sub_1425fae10
    Replacement::draft(0x1425fae30, "unknown_1425fae30", fn_1425fae30 as *const ()), // sub_1425fae30
    Replacement::draft(0x1425fae50, "unknown_1425fae50", fn_1425fae50 as *const ()), // sub_1425fae50
    Replacement::draft(0x1425fae70, "unknown_1425fae70", fn_1425fae70 as *const ()), // sub_1425fae70
    Replacement::draft(0x1425fae90, "unknown_1425fae90", fn_1425fae90 as *const ()), // sub_1425fae90
    Replacement::draft(0x1425faeb0, "unknown_1425faeb0", fn_1425faeb0 as *const ()), // sub_1425faeb0
    Replacement::draft(0x1425faed0, "unknown_1425faed0", fn_1425faed0 as *const ()), // sub_1425faed0
    Replacement::draft(0x1425faef0, "unknown_1425faef0", fn_1425faef0 as *const ()), // sub_1425faef0
    Replacement::draft(0x1425faf10, "unknown_1425faf10", fn_1425faf10 as *const ()), // sub_1425faf10
    Replacement::draft(0x1425fc6a0, "unknown_1425fc6a0", fn_1425fc6a0 as *const ()), // sub_1425fc6a0
    Replacement::draft(0x1425fc6c0, "unknown_1425fc6c0", fn_1425fc6c0 as *const ()), // sub_1425fc6c0
    Replacement::draft(0x1425fc6e0, "unknown_1425fc6e0", fn_1425fc6e0 as *const ()), // sub_1425fc6e0
    Replacement::draft(0x1425fc700, "unknown_1425fc700", fn_1425fc700 as *const ()), // sub_1425fc700
    Replacement::draft(0x1425fc720, "unknown_1425fc720", fn_1425fc720 as *const ()), // sub_1425fc720
    Replacement::draft(0x1425fc740, "unknown_1425fc740", fn_1425fc740 as *const ()), // sub_1425fc740
    Replacement::draft(0x1425fc760, "unknown_1425fc760", fn_1425fc760 as *const ()), // sub_1425fc760
    Replacement::draft(0x1425fc780, "unknown_1425fc780", fn_1425fc780 as *const ()), // sub_1425fc780
    Replacement::draft(0x1425fc7a0, "unknown_1425fc7a0", fn_1425fc7a0 as *const ()), // sub_1425fc7a0
    Replacement::draft(0x1425fc7c0, "unknown_1425fc7c0", fn_1425fc7c0 as *const ()), // sub_1425fc7c0
    Replacement::draft(0x1425fc7e0, "unknown_1425fc7e0", fn_1425fc7e0 as *const ()), // sub_1425fc7e0
    Replacement::draft(0x1425fc800, "unknown_1425fc800", fn_1425fc800 as *const ()), // sub_1425fc800
    Replacement::draft(0x1425fc820, "unknown_1425fc820", fn_1425fc820 as *const ()), // sub_1425fc820
    Replacement::draft(0x1425fc840, "unknown_1425fc840", fn_1425fc840 as *const ()), // sub_1425fc840
    Replacement::draft(0x1425fc860, "unknown_1425fc860", fn_1425fc860 as *const ()), // sub_1425fc860
    Replacement::draft(0x1425fc880, "unknown_1425fc880", fn_1425fc880 as *const ()), // sub_1425fc880
    Replacement::draft(0x1425fc8a0, "unknown_1425fc8a0", fn_1425fc8a0 as *const ()), // sub_1425fc8a0
    Replacement::draft(0x1425fc8c0, "unknown_1425fc8c0", fn_1425fc8c0 as *const ()), // sub_1425fc8c0
    Replacement::draft(0x1425fc8e0, "unknown_1425fc8e0", fn_1425fc8e0 as *const ()), // sub_1425fc8e0
    Replacement::draft(0x1425fc900, "unknown_1425fc900", fn_1425fc900 as *const ()), // sub_1425fc900
    Replacement::draft(0x1425fc920, "unknown_1425fc920", fn_1425fc920 as *const ()), // sub_1425fc920
    Replacement::draft(0x1425fc940, "unknown_1425fc940", fn_1425fc940 as *const ()), // sub_1425fc940
    Replacement::draft(0x1425fc960, "unknown_1425fc960", fn_1425fc960 as *const ()), // sub_1425fc960
    Replacement::draft(0x1425fc980, "unknown_1425fc980", fn_1425fc980 as *const ()), // sub_1425fc980
    Replacement::draft(0x1425fc9a0, "unknown_1425fc9a0", fn_1425fc9a0 as *const ()), // sub_1425fc9a0
    Replacement::draft(0x1425fc9c0, "unknown_1425fc9c0", fn_1425fc9c0 as *const ()), // sub_1425fc9c0
    Replacement::draft(0x1425fc9e0, "unknown_1425fc9e0", fn_1425fc9e0 as *const ()), // sub_1425fc9e0
    Replacement::draft(0x1425fca00, "unknown_1425fca00", fn_1425fca00 as *const ()), // sub_1425fca00
    Replacement::draft(0x1425fca20, "unknown_1425fca20", fn_1425fca20 as *const ()), // sub_1425fca20
    Replacement::draft(0x1425fca40, "unknown_1425fca40", fn_1425fca40 as *const ()), // sub_1425fca40
    Replacement::draft(0x1425fca60, "unknown_1425fca60", fn_1425fca60 as *const ()), // sub_1425fca60
    Replacement::draft(0x1425fca80, "unknown_1425fca80", fn_1425fca80 as *const ()), // sub_1425fca80
    Replacement::draft(0x1425fcaa0, "unknown_1425fcaa0", fn_1425fcaa0 as *const ()), // sub_1425fcaa0
    Replacement::draft(0x1425fcac0, "unknown_1425fcac0", fn_1425fcac0 as *const ()), // sub_1425fcac0
    Replacement::draft(0x14260bde0, "unknown_14260bde0", fn_14260bde0 as *const ()), // sub_14260bde0
    Replacement::draft(0x14260c530, "unknown_14260c530", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14384fa10, 0x143a00290> as *const ()), // sub_14260c530
    Replacement::draft(0x14260c600, "unknown_14260c600", tpl_53272::<0x143a44910, 0x14022a610, 0x1434077a8, 0x143a44900, 0x143a001ac, 0x143a00290> as *const ()), // sub_14260c600
    Replacement::draft(0x14260c690, "unknown_14260c690", tpl_53272::<0x143a44938, 0x14022a610, 0x1434077a8, 0x143a44928, 0x143a001ac, 0x143a00290> as *const ()), // sub_14260c690
    Replacement::draft(0x14260c770, "unknown_14260c770", tpl_111516::<0x143c48458, 0x8, 0xc> as *const ()), // sub_14260c770
    Replacement::draft(0x14260c7c0, "unknown_14260c7c0", tpl_111516::<0x143c48468, 0x8, 0xc> as *const ()), // sub_14260c7c0
    Replacement::draft(0x14260c810, "unknown_14260c810", tpl_111516::<0x143c48478, 0x8, 0xc> as *const ()), // sub_14260c810
    Replacement::draft(0x14260cf20, "unknown_14260cf20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850090, 0x143a00290> as *const ()), // sub_14260cf20
    Replacement::draft(0x14260d3a0, "unknown_14260d3a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850420, 0x143a00290> as *const ()), // sub_14260d3a0
    Replacement::draft(0x14260d430, "unknown_14260d430", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850440, 0x143a00290> as *const ()), // sub_14260d430
    Replacement::draft(0x14260d4c0, "unknown_14260d4c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438504a0, 0x143a00290> as *const ()), // sub_14260d4c0
    Replacement::draft(0x14260d5d0, "unknown_14260d5d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438504c0, 0x143a00290> as *const ()), // sub_14260d5d0
    Replacement::draft(0x14260d660, "unknown_14260d660", tpl_53266::<0x143850520, 0x1438504e8, 0x20> as *const ()), // sub_14260d660
    Replacement::draft(0x14260d9e0, "unknown_14260d9e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850708, 0x143a00290> as *const ()), // sub_14260d9e0
    Replacement::draft(0x14260da70, "unknown_14260da70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850728, 0x143a00290> as *const ()), // sub_14260da70
    Replacement::draft(0x14260db00, "unknown_14260db00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850748, 0x143a00290> as *const ()), // sub_14260db00
    Replacement::draft(0x14260db90, "unknown_14260db90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850768, 0x143a00290> as *const ()), // sub_14260db90
    Replacement::draft(0x14260e000, "unknown_14260e000", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850a60, 0x143a00290> as *const ()), // sub_14260e000
    Replacement::draft(0x14260e2a0, "unknown_14260e2a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850b08, 0x143a00290> as *const ()), // sub_14260e2a0
    Replacement::draft(0x14260e330, "unknown_14260e330", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850b20, 0x143a00290> as *const ()), // sub_14260e330
    Replacement::draft(0x14260e3d0, "unknown_14260e3d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850ba8, 0x143a00290> as *const ()), // sub_14260e3d0
    Replacement::draft(0x14260e460, "unknown_14260e460", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850b50, 0x143a00290> as *const ()), // sub_14260e460
    Replacement::draft(0x14260e4f0, "unknown_14260e4f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850b70, 0x143a00290> as *const ()), // sub_14260e4f0
    Replacement::draft(0x14260e580, "unknown_14260e580", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850b90, 0x143a00290> as *const ()), // sub_14260e580
    Replacement::draft(0x14260e610, "unknown_14260e610", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850bc0, 0x143a00290> as *const ()), // sub_14260e610
    Replacement::draft(0x14260e6a0, "unknown_14260e6a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850b38, 0x143a00290> as *const ()), // sub_14260e6a0
    Replacement::draft(0x14260e7c0, "unknown_14260e7c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850bd8, 0x143a00290> as *const ()), // sub_14260e7c0
    Replacement::draft(0x14260e860, "unknown_14260e860", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850bf8, 0x143a00290> as *const ()), // sub_14260e860
    Replacement::draft(0x14260e8f0, "unknown_14260e8f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850c18, 0x143a00290> as *const ()), // sub_14260e8f0
    Replacement::draft(0x14260e980, "unknown_14260e980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850c30, 0x143a00290> as *const ()), // sub_14260e980
    Replacement::draft(0x14260ea10, "unknown_14260ea10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850c48, 0x143a00290> as *const ()), // sub_14260ea10
    Replacement::draft(0x14260eaa0, "unknown_14260eaa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850c60, 0x143a00290> as *const ()), // sub_14260eaa0
    Replacement::draft(0x14260eb30, "unknown_14260eb30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850c78, 0x143a00290> as *const ()), // sub_14260eb30
    Replacement::draft(0x14260ec30, "unknown_14260ec30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850c90, 0x143a00290> as *const ()), // sub_14260ec30
    Replacement::draft(0x14260ecc0, "unknown_14260ecc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850ca8, 0x143a00290> as *const ()), // sub_14260ecc0
    Replacement::draft(0x14260ee40, "unknown_14260ee40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850d50, 0x143a00290> as *const ()), // sub_14260ee40
    Replacement::draft(0x14260eed0, "unknown_14260eed0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850d70, 0x143a00290> as *const ()), // sub_14260eed0
    Replacement::draft(0x14260ef60, "unknown_14260ef60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850d90, 0x143a00290> as *const ()), // sub_14260ef60
    Replacement::draft(0x14260eff0, "unknown_14260eff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850db0, 0x143a00290> as *const ()), // sub_14260eff0
    Replacement::draft(0x14260f240, "unknown_14260f240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850e58, 0x143a00290> as *const ()), // sub_14260f240
    Replacement::draft(0x14260f2d0, "unknown_14260f2d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850e70, 0x143a00290> as *const ()), // sub_14260f2d0
    Replacement::draft(0x14260f360, "unknown_14260f360", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850e88, 0x143a00290> as *const ()), // sub_14260f360
    Replacement::draft(0x14260f3f0, "unknown_14260f3f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850ea0, 0x143a00290> as *const ()), // sub_14260f3f0
    Replacement::draft(0x14260f480, "unknown_14260f480", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850eb8, 0x143a00290> as *const ()), // sub_14260f480
    Replacement::draft(0x14260f510, "unknown_14260f510", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850ed0, 0x143a00290> as *const ()), // sub_14260f510
    Replacement::draft(0x14260f6d0, "unknown_14260f6d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850f00, 0x143a00290> as *const ()), // sub_14260f6d0
    Replacement::draft(0x14260f7a0, "unknown_14260f7a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850f18, 0x143a00290> as *const ()), // sub_14260f7a0
    Replacement::draft(0x14260f830, "unknown_14260f830", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850f30, 0x143a00290> as *const ()), // sub_14260f830
    Replacement::draft(0x14260f900, "unknown_14260f900", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143850f48, 0x143a00290> as *const ()), // sub_14260f900
    Replacement::draft(0x14260fa60, "unknown_14260fa60", tpl_53377::<0x143ccfbe0, 0x140246bb0, 0x1425371a0, 0x142533200> as *const ()), // sub_14260fa60
    Replacement::draft(0x14260fbc0, "unknown_14260fbc0", tpl_53377::<0x143ccfbd0, 0x140246bb0, 0x1425371a0, 0x142533200> as *const ()), // sub_14260fbc0
    Replacement::draft(0x14260fec0, "unknown_14260fec0", tpl_53378::<0x143851bb0, 0x14137e320, 0x143851ba0, 0x1401de830> as *const ()), // sub_14260fec0
    Replacement::draft(0x14260ff70, "unknown_14260ff70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851c18, 0x143a00290> as *const ()), // sub_14260ff70
    Replacement::draft(0x142610000, "unknown_142610000", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851c30, 0x143a00290> as *const ()), // sub_142610000
    Replacement::draft(0x142610140, "unknown_142610140", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851c50, 0x143a00290> as *const ()), // sub_142610140
    Replacement::draft(0x1426101d0, "unknown_1426101d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851c70, 0x143a00290> as *const ()), // sub_1426101d0
    Replacement::draft(0x142610260, "unknown_142610260", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851c90, 0x143a00290> as *const ()), // sub_142610260
    Replacement::draft(0x1426102f0, "unknown_1426102f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851cb0, 0x143a00290> as *const ()), // sub_1426102f0
    Replacement::draft(0x142610380, "unknown_142610380", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851cd0, 0x143a00290> as *const ()), // sub_142610380
    Replacement::draft(0x142610410, "unknown_142610410", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851cf0, 0x143a00290> as *const ()), // sub_142610410
    Replacement::draft(0x1426104a0, "unknown_1426104a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143851d08, 0x143a00290> as *const ()), // sub_1426104a0
    Replacement::draft(0x142610700, "unknown_142610700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853660, 0x143a00290> as *const ()), // sub_142610700
    Replacement::draft(0x142610790, "unknown_142610790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853678, 0x143a00290> as *const ()), // sub_142610790
    Replacement::draft(0x142610860, "unknown_142610860", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438536b0, 0x143a00290> as *const ()), // sub_142610860
    Replacement::draft(0x1426108f0, "unknown_1426108f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438536d0, 0x143a00290> as *const ()), // sub_1426108f0
    Replacement::draft(0x142610980, "unknown_142610980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853690, 0x143a00290> as *const ()), // sub_142610980
    Replacement::draft(0x142610b60, "unknown_142610b60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438537b0, 0x143a00290> as *const ()), // sub_142610b60
    Replacement::draft(0x142610c30, "unknown_142610c30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438537c8, 0x143a00290> as *const ()), // sub_142610c30
    Replacement::draft(0x142610cc0, "unknown_142610cc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438537e8, 0x143a00290> as *const ()), // sub_142610cc0
    Replacement::draft(0x1426112f0, "unknown_1426112f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853898, 0x143a00290> as *const ()), // sub_1426112f0
    Replacement::draft(0x142611380, "unknown_142611380", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438538b8, 0x143a00290> as *const ()), // sub_142611380
    Replacement::draft(0x142611b10, "unknown_142611b10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853a00, 0x143a00290> as *const ()), // sub_142611b10
    Replacement::draft(0x142611e30, "unknown_142611e30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853b58, 0x143a00290> as *const ()), // sub_142611e30
    Replacement::draft(0x142611ec0, "unknown_142611ec0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853b38, 0x143a00290> as *const ()), // sub_142611ec0
    Replacement::draft(0x142612230, "unknown_142612230", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853eb0, 0x143a00290> as *const ()), // sub_142612230
    Replacement::draft(0x1426122c0, "unknown_1426122c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853e00, 0x143a00290> as *const ()), // sub_1426122c0
    Replacement::draft(0x142612350, "unknown_142612350", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853e30, 0x143a00290> as *const ()), // sub_142612350
    Replacement::draft(0x1426123e0, "unknown_1426123e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853de0, 0x143a00290> as *const ()), // sub_1426123e0
    Replacement::draft(0x142612470, "unknown_142612470", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853e18, 0x143a00290> as *const ()), // sub_142612470
    Replacement::draft(0x142612500, "unknown_142612500", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853e70, 0x143a00290> as *const ()), // sub_142612500
    Replacement::draft(0x142612590, "unknown_142612590", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853e90, 0x143a00290> as *const ()), // sub_142612590
    Replacement::draft(0x142612620, "unknown_142612620", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853e50, 0x143a00290> as *const ()), // sub_142612620
    Replacement::draft(0x1426126b0, "unknown_1426126b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853dc0, 0x143a00290> as *const ()), // sub_1426126b0
    Replacement::draft(0x142612740, "unknown_142612740", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853d88, 0x143a00290> as *const ()), // sub_142612740
    Replacement::draft(0x1426127d0, "unknown_1426127d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853cf0, 0x143a00290> as *const ()), // sub_1426127d0
    Replacement::draft(0x142612860, "unknown_142612860", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853da0, 0x143a00290> as *const ()), // sub_142612860
    Replacement::draft(0x1426128f0, "unknown_1426128f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853d58, 0x143a00290> as *const ()), // sub_1426128f0
    Replacement::draft(0x142612980, "unknown_142612980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853d40, 0x143a00290> as *const ()), // sub_142612980
    Replacement::draft(0x142612a10, "unknown_142612a10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853ec8, 0x143a00290> as *const ()), // sub_142612a10
    Replacement::draft(0x142612aa0, "unknown_142612aa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853d70, 0x143a00290> as *const ()), // sub_142612aa0
    Replacement::draft(0x142612b70, "unknown_142612b70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853ee0, 0x143a00290> as *const ()), // sub_142612b70
    Replacement::draft(0x142612c00, "unknown_142612c00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853f18, 0x143a00290> as *const ()), // sub_142612c00
    Replacement::draft(0x142612c90, "unknown_142612c90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853ef8, 0x143a00290> as *const ()), // sub_142612c90
    Replacement::draft(0x142612d20, "unknown_142612d20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853f68, 0x143a00290> as *const ()), // sub_142612d20
    Replacement::draft(0x142612db0, "unknown_142612db0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853f30, 0x143a00290> as *const ()), // sub_142612db0
    Replacement::draft(0x142612e40, "unknown_142612e40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143853f48, 0x143a00290> as *const ()), // sub_142612e40
    Replacement::draft(0x1426131c0, "unknown_1426131c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438542f0, 0x143a00290> as *const ()), // sub_1426131c0
    Replacement::draft(0x142613250, "unknown_142613250", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854178, 0x143a00290> as *const ()), // sub_142613250
    Replacement::draft(0x1426132e0, "unknown_1426132e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854160, 0x143a00290> as *const ()), // sub_1426132e0
    Replacement::draft(0x142613370, "unknown_142613370", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854148, 0x143a00290> as *const ()), // sub_142613370
    Replacement::draft(0x142613400, "unknown_142613400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854130, 0x143a00290> as *const ()), // sub_142613400
    Replacement::draft(0x142613490, "unknown_142613490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143880ee8, 0x143a00290> as *const ()), // sub_142613490
    Replacement::draft(0x142613520, "unknown_142613520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143880ed0, 0x143a00290> as *const ()), // sub_142613520
    Replacement::draft(0x1426135b0, "unknown_1426135b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854340, 0x143a00290> as *const ()), // sub_1426135b0
    Replacement::draft(0x142613640, "unknown_142613640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854308, 0x143a00290> as *const ()), // sub_142613640
    Replacement::draft(0x1426136d0, "unknown_1426136d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854358, 0x143a00290> as *const ()), // sub_1426136d0
    Replacement::draft(0x142613760, "unknown_142613760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854320, 0x143a00290> as *const ()), // sub_142613760
    Replacement::draft(0x1426137f0, "unknown_1426137f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854378, 0x143a00290> as *const ()), // sub_1426137f0
    Replacement::draft(0x142613880, "unknown_142613880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854390, 0x143a00290> as *const ()), // sub_142613880
    Replacement::draft(0x1426139c0, "unknown_1426139c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854450, 0x143a00290> as *const ()), // sub_1426139c0
    Replacement::draft(0x142613a50, "unknown_142613a50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854468, 0x143a00290> as *const ()), // sub_142613a50
    Replacement::draft(0x142613ae0, "unknown_142613ae0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854438, 0x143a00290> as *const ()), // sub_142613ae0
    Replacement::draft(0x142613b70, "unknown_142613b70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854558, 0x143a00290> as *const ()), // sub_142613b70
    Replacement::draft(0x142613c00, "unknown_142613c00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854570, 0x143a00290> as *const ()), // sub_142613c00
    Replacement::draft(0x142613c90, "unknown_142613c90", fn_142613c90 as *const ()), // sub_142613c90
    Replacement::draft(0x142613da0, "unknown_142613da0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854588, 0x143a00290> as *const ()), // sub_142613da0
    Replacement::draft(0x142613e30, "unknown_142613e30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438545c8, 0x143a00290> as *const ()), // sub_142613e30
    Replacement::draft(0x142613ec0, "unknown_142613ec0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438545a8, 0x143a00290> as *const ()), // sub_142613ec0
    Replacement::draft(0x142613f80, "unknown_142613f80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438545e8, 0x143a00290> as *const ()), // sub_142613f80
    Replacement::draft(0x1426140a0, "unknown_1426140a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854680, 0x143a00290> as *const ()), // sub_1426140a0
    Replacement::draft(0x142614130, "unknown_142614130", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854848, 0x143a00290> as *const ()), // sub_142614130
    Replacement::draft(0x1426141c0, "unknown_1426141c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854770, 0x143a00290> as *const ()), // sub_1426141c0
    Replacement::draft(0x142614250, "unknown_142614250", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854660, 0x143a00290> as *const ()), // sub_142614250
    Replacement::draft(0x1426142e0, "unknown_1426142e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854698, 0x143a00290> as *const ()), // sub_1426142e0
    Replacement::draft(0x142614370, "unknown_142614370", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854868, 0x143a00290> as *const ()), // sub_142614370
    Replacement::draft(0x142614400, "unknown_142614400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438547d8, 0x143a00290> as *const ()), // sub_142614400
    Replacement::draft(0x142614490, "unknown_142614490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854790, 0x143a00290> as *const ()), // sub_142614490
    Replacement::draft(0x142614520, "unknown_142614520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854758, 0x143a00290> as *const ()), // sub_142614520
    Replacement::draft(0x1426145b0, "unknown_1426145b0", tpl_53272::<0x143a47ca8, 0x14022a610, 0x1434077a8, 0x143a47c98, 0x143a001ac, 0x143a00290> as *const ()), // sub_1426145b0
    Replacement::draft(0x142614640, "unknown_142614640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854888, 0x143a00290> as *const ()), // sub_142614640
    Replacement::draft(0x1426146d0, "unknown_1426146d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854828, 0x143a00290> as *const ()), // sub_1426146d0
    Replacement::draft(0x142614760, "unknown_142614760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854720, 0x143a00290> as *const ()), // sub_142614760
    Replacement::draft(0x1426147f0, "unknown_1426147f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854708, 0x143a00290> as *const ()), // sub_1426147f0
    Replacement::draft(0x142614880, "unknown_142614880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854738, 0x143a00290> as *const ()), // sub_142614880
    Replacement::draft(0x142614910, "unknown_142614910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438547a8, 0x143a00290> as *const ()), // sub_142614910
    Replacement::draft(0x142614a00, "unknown_142614a00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438548a8, 0x143a00290> as *const ()), // sub_142614a00
    Replacement::draft(0x142614b30, "unknown_142614b30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438548c8, 0x143a00290> as *const ()), // sub_142614b30
    Replacement::draft(0x142614c50, "unknown_142614c50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854990, 0x143a00290> as *const ()), // sub_142614c50
    Replacement::draft(0x142614ce0, "unknown_142614ce0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143854978, 0x143a00290> as *const ()), // sub_142614ce0
    Replacement::draft(0x142614e00, "unknown_142614e00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438555f0, 0x143a00290> as *const ()), // sub_142614e00
    Replacement::draft(0x142614e90, "unknown_142614e90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855610, 0x143a00290> as *const ()), // sub_142614e90
    Replacement::draft(0x142614f20, "unknown_142614f20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855630, 0x143a00290> as *const ()), // sub_142614f20
    Replacement::draft(0x142614ff0, "unknown_142614ff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855648, 0x143a00290> as *const ()), // sub_142614ff0
    Replacement::draft(0x142615080, "unknown_142615080", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855678, 0x143a00290> as *const ()), // sub_142615080
    Replacement::draft(0x142615110, "unknown_142615110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855660, 0x143a00290> as *const ()), // sub_142615110
    Replacement::draft(0x1426151b0, "unknown_1426151b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438556b0, 0x143a00290> as *const ()), // sub_1426151b0
    Replacement::draft(0x142615240, "unknown_142615240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855690, 0x143a00290> as *const ()), // sub_142615240
    Replacement::draft(0x142615340, "unknown_142615340", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438556d0, 0x143a00290> as *const ()), // sub_142615340
    Replacement::draft(0x1426153d0, "unknown_1426153d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855850, 0x143a00290> as *const ()), // sub_1426153d0
    Replacement::draft(0x142615660, "unknown_142615660", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438559b0, 0x143a00290> as *const ()), // sub_142615660
    Replacement::draft(0x142615860, "unknown_142615860", tpl_53378::<0x143a482b8, 0x1401e3450, 0x143a482a8, 0x1401de830> as *const ()), // sub_142615860
    Replacement::draft(0x142615890, "unknown_142615890", tpl_53266::<0x143a48308, 0x143a482d0, 0x20> as *const ()), // sub_142615890
    Replacement::draft(0x142615990, "unknown_142615990", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855b80, 0x143a00290> as *const ()), // sub_142615990
    Replacement::draft(0x142615b20, "unknown_142615b20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855cb0, 0x143a00290> as *const ()), // sub_142615b20
    Replacement::draft(0x142615bb0, "unknown_142615bb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855cf0, 0x143a00290> as *const ()), // sub_142615bb0
    Replacement::draft(0x142615c40, "unknown_142615c40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855c80, 0x143a00290> as *const ()), // sub_142615c40
    Replacement::draft(0x142615cd0, "unknown_142615cd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855cd0, 0x143a00290> as *const ()), // sub_142615cd0
    Replacement::draft(0x142615d60, "unknown_142615d60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855c98, 0x143a00290> as *const ()), // sub_142615d60
    Replacement::draft(0x142615ef0, "unknown_142615ef0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855d80, 0x143a00290> as *const ()), // sub_142615ef0
    Replacement::draft(0x142615f80, "unknown_142615f80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855d68, 0x143a00290> as *const ()), // sub_142615f80
    Replacement::draft(0x142616010, "unknown_142616010", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855e00, 0x143a00290> as *const ()), // sub_142616010
    Replacement::draft(0x1426160a0, "unknown_1426160a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855dc8, 0x143a00290> as *const ()), // sub_1426160a0
    Replacement::draft(0x142616130, "unknown_142616130", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855de8, 0x143a00290> as *const ()), // sub_142616130
    Replacement::draft(0x142616230, "unknown_142616230", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855e20, 0x143a00290> as *const ()), // sub_142616230
    Replacement::draft(0x1426162f0, "unknown_1426162f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855eb0, 0x143a00290> as *const ()), // sub_1426162f0
    Replacement::draft(0x142616380, "unknown_142616380", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855e90, 0x143a00290> as *const ()), // sub_142616380
    Replacement::draft(0x142616410, "unknown_142616410", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855e70, 0x143a00290> as *const ()), // sub_142616410
    Replacement::draft(0x1426164a0, "unknown_1426164a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855e50, 0x143a00290> as *const ()), // sub_1426164a0
    Replacement::draft(0x142616560, "unknown_142616560", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855ed0, 0x143a00290> as *const ()), // sub_142616560
    Replacement::draft(0x1426166c0, "unknown_1426166c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856140, 0x143a00290> as *const ()), // sub_1426166c0
    Replacement::draft(0x142616750, "unknown_142616750", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855fd0, 0x143a00290> as *const ()), // sub_142616750
    Replacement::draft(0x142616800, "unknown_142616800", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143855fe8, 0x143a00290> as *const ()), // sub_142616800
    Replacement::draft(0x142616890, "unknown_142616890", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856008, 0x143a00290> as *const ()), // sub_142616890
    Replacement::draft(0x142616a60, "unknown_142616a60", tpl_53377::<0x143ce20d0, 0x140246bb0, 0x1425371a0, 0x142533200> as *const ()), // sub_142616a60
    Replacement::draft(0x142616bd0, "unknown_142616bd0", tpl_111627::<0x143c48788, 0x1401e5380, 0x143c48778, 0x8, 0x1401e66f0> as *const ()), // sub_142616bd0
    Replacement::draft(0x142616d10, "unknown_142616d10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856360, 0x143a00290> as *const ()), // sub_142616d10
    Replacement::draft(0x142616da0, "unknown_142616da0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856420, 0x143a00290> as *const ()), // sub_142616da0
    Replacement::draft(0x142616e30, "unknown_142616e30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438563b8, 0x143a00290> as *const ()), // sub_142616e30
    Replacement::draft(0x142616ec0, "unknown_142616ec0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856398, 0x143a00290> as *const ()), // sub_142616ec0
    Replacement::draft(0x142616f50, "unknown_142616f50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856380, 0x143a00290> as *const ()), // sub_142616f50
    Replacement::draft(0x142616fe0, "unknown_142616fe0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856408, 0x143a00290> as *const ()), // sub_142616fe0
    Replacement::draft(0x142617070, "unknown_142617070", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438563f0, 0x143a00290> as *const ()), // sub_142617070
    Replacement::draft(0x142617100, "unknown_142617100", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438563d8, 0x143a00290> as *const ()), // sub_142617100
    Replacement::draft(0x1426171e0, "unknown_1426171e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438565c0, 0x143a00290> as *const ()), // sub_1426171e0
    Replacement::draft(0x142617270, "unknown_142617270", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438564c8, 0x143a00290> as *const ()), // sub_142617270
    Replacement::draft(0x142617300, "unknown_142617300", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856540, 0x143a00290> as *const ()), // sub_142617300
    Replacement::draft(0x142617390, "unknown_142617390", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856508, 0x143a00290> as *const ()), // sub_142617390
    Replacement::draft(0x142617420, "unknown_142617420", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856640, 0x143a00290> as *const ()), // sub_142617420
    Replacement::draft(0x1426174b0, "unknown_1426174b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438564e8, 0x143a00290> as *const ()), // sub_1426174b0
    Replacement::draft(0x142617540, "unknown_142617540", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856660, 0x143a00290> as *const ()), // sub_142617540
    Replacement::draft(0x1426175d0, "unknown_1426175d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438564a8, 0x143a00290> as *const ()), // sub_1426175d0
    Replacement::draft(0x142617660, "unknown_142617660", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856610, 0x143a00290> as *const ()), // sub_142617660
    Replacement::draft(0x1426176f0, "unknown_1426176f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438565f0, 0x143a00290> as *const ()), // sub_1426176f0
    Replacement::draft(0x142617780, "unknown_142617780", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438565a8, 0x143a00290> as *const ()), // sub_142617780
    Replacement::draft(0x142617810, "unknown_142617810", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856470, 0x143a00290> as *const ()), // sub_142617810
    Replacement::draft(0x1426178a0, "unknown_1426178a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856458, 0x143a00290> as *const ()), // sub_1426178a0
    Replacement::draft(0x142617930, "unknown_142617930", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856578, 0x143a00290> as *const ()), // sub_142617930
    Replacement::draft(0x1426179c0, "unknown_1426179c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856490, 0x143a00290> as *const ()), // sub_1426179c0
    Replacement::draft(0x142617a50, "unknown_142617a50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856590, 0x143a00290> as *const ()), // sub_142617a50
    Replacement::draft(0x142617ae0, "unknown_142617ae0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856528, 0x143a00290> as *const ()), // sub_142617ae0
    Replacement::draft(0x142617b70, "unknown_142617b70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856628, 0x143a00290> as *const ()), // sub_142617b70
    Replacement::draft(0x142617c00, "unknown_142617c00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856440, 0x143a00290> as *const ()), // sub_142617c00
    Replacement::draft(0x142617c90, "unknown_142617c90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856560, 0x143a00290> as *const ()), // sub_142617c90
    Replacement::draft(0x142617d20, "unknown_142617d20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438565d8, 0x143a00290> as *const ()), // sub_142617d20
    Replacement::draft(0x142617e80, "unknown_142617e80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856698, 0x143a00290> as *const ()), // sub_142617e80
    Replacement::draft(0x142617f10, "unknown_142617f10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143856680, 0x143a00290> as *const ()), // sub_142617f10
    Replacement::draft(0x142618230, "unknown_142618230", tpl_53266::<0x143a48c48, 0x143a48c10, 0x20> as *const ()), // sub_142618230
    Replacement::draft(0x142618270, "unknown_142618270", tpl_53266::<0x143a48c08, 0x143a48bd0, 0x20> as *const ()), // sub_142618270
    Replacement::draft(0x142618330, "unknown_142618330", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857738, 0x143a00290> as *const ()), // sub_142618330
    Replacement::draft(0x1426183c0, "unknown_1426183c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857750, 0x143a00290> as *const ()), // sub_1426183c0
    Replacement::draft(0x142618450, "unknown_142618450", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857608, 0x143a00290> as *const ()), // sub_142618450
    Replacement::draft(0x1426184e0, "unknown_1426184e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857400, 0x143a00290> as *const ()), // sub_1426184e0
    Replacement::draft(0x142618570, "unknown_142618570", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438574e8, 0x143a00290> as *const ()), // sub_142618570
    Replacement::draft(0x142618600, "unknown_142618600", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857700, 0x143a00290> as *const ()), // sub_142618600
    Replacement::draft(0x142618690, "unknown_142618690", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438573c8, 0x143a00290> as *const ()), // sub_142618690
    Replacement::draft(0x142618720, "unknown_142618720", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857480, 0x143a00290> as *const ()), // sub_142618720
    Replacement::draft(0x1426187b0, "unknown_1426187b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857468, 0x143a00290> as *const ()), // sub_1426187b0
    Replacement::draft(0x142618840, "unknown_142618840", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857638, 0x143a00290> as *const ()), // sub_142618840
    Replacement::draft(0x1426188d0, "unknown_1426188d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438575c0, 0x143a00290> as *const ()), // sub_1426188d0
    Replacement::draft(0x142618960, "unknown_142618960", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857620, 0x143a00290> as *const ()), // sub_142618960
    Replacement::draft(0x1426189f0, "unknown_1426189f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857538, 0x143a00290> as *const ()), // sub_1426189f0
    Replacement::draft(0x142618a80, "unknown_142618a80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857380, 0x143a00290> as *const ()), // sub_142618a80
    Replacement::draft(0x142618b10, "unknown_142618b10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857518, 0x143a00290> as *const ()), // sub_142618b10
    Replacement::draft(0x142618ba0, "unknown_142618ba0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438577e0, 0x143a00290> as *const ()), // sub_142618ba0
    Replacement::draft(0x142618c30, "unknown_142618c30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438573b0, 0x143a00290> as *const ()), // sub_142618c30
    Replacement::draft(0x142618cc0, "unknown_142618cc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438575d8, 0x143a00290> as *const ()), // sub_142618cc0
    Replacement::draft(0x142618d50, "unknown_142618d50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857438, 0x143a00290> as *const ()), // sub_142618d50
    Replacement::draft(0x142618de0, "unknown_142618de0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857720, 0x143a00290> as *const ()), // sub_142618de0
    Replacement::draft(0x142618e70, "unknown_142618e70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438575f0, 0x143a00290> as *const ()), // sub_142618e70
    Replacement::draft(0x142618f00, "unknown_142618f00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438574a0, 0x143a00290> as *const ()), // sub_142618f00
    Replacement::draft(0x142618f90, "unknown_142618f90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438573e8, 0x143a00290> as *const ()), // sub_142618f90
    Replacement::draft(0x142619020, "unknown_142619020", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857650, 0x143a00290> as *const ()), // sub_142619020
    Replacement::draft(0x1426190b0, "unknown_1426190b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857398, 0x143a00290> as *const ()), // sub_1426190b0
    Replacement::draft(0x142619140, "unknown_142619140", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857800, 0x143a00290> as *const ()), // sub_142619140
    Replacement::draft(0x1426191d0, "unknown_1426191d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857420, 0x143a00290> as *const ()), // sub_1426191d0
    Replacement::draft(0x142619260, "unknown_142619260", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857500, 0x143a00290> as *const ()), // sub_142619260
    Replacement::draft(0x1426192f0, "unknown_1426192f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857450, 0x143a00290> as *const ()), // sub_1426192f0
    Replacement::draft(0x142619380, "unknown_142619380", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438574d0, 0x143a00290> as *const ()), // sub_142619380
    Replacement::draft(0x142619410, "unknown_142619410", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438574b8, 0x143a00290> as *const ()), // sub_142619410
    Replacement::draft(0x1426194a0, "unknown_1426194a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857670, 0x143a00290> as *const ()), // sub_1426194a0
    Replacement::draft(0x142619610, "unknown_142619610", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857818, 0x143a00290> as *const ()), // sub_142619610
    Replacement::draft(0x1426196a0, "unknown_1426196a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857838, 0x143a00290> as *const ()), // sub_1426196a0
    Replacement::draft(0x142619730, "unknown_142619730", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438578a8, 0x143a00290> as *const ()), // sub_142619730
    Replacement::draft(0x1426197c0, "unknown_1426197c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857870, 0x143a00290> as *const ()), // sub_1426197c0
    Replacement::draft(0x142619850, "unknown_142619850", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438578e0, 0x143a00290> as *const ()), // sub_142619850
    Replacement::draft(0x1426198e0, "unknown_1426198e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857888, 0x143a00290> as *const ()), // sub_1426198e0
    Replacement::draft(0x142619970, "unknown_142619970", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857858, 0x143a00290> as *const ()), // sub_142619970
    Replacement::draft(0x142619a00, "unknown_142619a00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438578c0, 0x143a00290> as *const ()), // sub_142619a00
    Replacement::draft(0x142619ab0, "unknown_142619ab0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857938, 0x143a00290> as *const ()), // sub_142619ab0
    Replacement::draft(0x142619b50, "unknown_142619b50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857998, 0x143a00290> as *const ()), // sub_142619b50
    Replacement::draft(0x142619c00, "unknown_142619c00", fn_142619c00 as *const ()), // sub_142619c00
    Replacement::draft(0x142619c20, "unknown_142619c20", fn_142619c20 as *const ()), // sub_142619c20
    Replacement::draft(0x142619c40, "unknown_142619c40", tpl_53382::<0x143a34f48, 0x142442de0> as *const ()), // sub_142619c40
    Replacement::draft(0x142619c70, "unknown_142619c70", tpl_53382::<0x143a348d8, 0x142442de0> as *const ()), // sub_142619c70
    Replacement::draft(0x142619ca0, "unknown_142619ca0", tpl_53382::<0x143a35528, 0x142442de0> as *const ()), // sub_142619ca0
    Replacement::draft(0x142619cd0, "unknown_142619cd0", tpl_53382::<0x143a349b8, 0x142442de0> as *const ()), // sub_142619cd0
    Replacement::draft(0x142619d00, "unknown_142619d00", tpl_53382::<0x143a34e78, 0x142442de0> as *const ()), // sub_142619d00
    Replacement::draft(0x142619d30, "unknown_142619d30", tpl_53382::<0x143a347b8, 0x142442de0> as *const ()), // sub_142619d30
    Replacement::draft(0x142619d60, "unknown_142619d60", tpl_53382::<0x143a353b8, 0x142442de0> as *const ()), // sub_142619d60
    Replacement::draft(0x142619d90, "unknown_142619d90", tpl_53382::<0x143a35148, 0x142442de0> as *const ()), // sub_142619d90
    Replacement::draft(0x142619dc0, "unknown_142619dc0", tpl_53382::<0x143a34df8, 0x142442de0> as *const ()), // sub_142619dc0
    Replacement::draft(0x142619df0, "unknown_142619df0", tpl_53382::<0x143a34a38, 0x142442de0> as *const ()), // sub_142619df0
    Replacement::draft(0x142619e20, "unknown_142619e20", tpl_53382::<0x143a35328, 0x142442de0> as *const ()), // sub_142619e20
    Replacement::draft(0x142619e50, "unknown_142619e50", tpl_53382::<0x143a34b68, 0x142442de0> as *const ()), // sub_142619e50
    Replacement::draft(0x142619e80, "unknown_142619e80", tpl_53382::<0x143a35898, 0x142442de0> as *const ()), // sub_142619e80
    Replacement::draft(0x142619eb0, "unknown_142619eb0", tpl_53382::<0x143a355f8, 0x142442de0> as *const ()), // sub_142619eb0
    Replacement::draft(0x142619ee0, "unknown_142619ee0", tpl_53382::<0x143a351f8, 0x142442de0> as *const ()), // sub_142619ee0
    Replacement::draft(0x142619f10, "unknown_142619f10", tpl_53382::<0x143a34848, 0x142442de0> as *const ()), // sub_142619f10
    Replacement::draft(0x142619f40, "unknown_142619f40", tpl_53382::<0x143a35448, 0x142442de0> as *const ()), // sub_142619f40
    Replacement::draft(0x142619f70, "unknown_142619f70", tpl_53382::<0x143a34d08, 0x142442de0> as *const ()), // sub_142619f70
    Replacement::draft(0x142619fa0, "unknown_142619fa0", tpl_53382::<0x143a34728, 0x142442de0> as *const ()), // sub_142619fa0
    Replacement::draft(0x142619fd0, "unknown_142619fd0", tpl_53382::<0x143a34fc8, 0x142442de0> as *const ()), // sub_142619fd0
    Replacement::draft(0x14261a000, "unknown_14261a000", tpl_53382::<0x143a34d58, 0x142442de0> as *const ()), // sub_14261a000
    Replacement::draft(0x14261a030, "unknown_14261a030", tpl_53382::<0x143a34a98, 0x142442de0> as *const ()), // sub_14261a030
    Replacement::draft(0x14261a060, "unknown_14261a060", tpl_53382::<0x143a357b8, 0x142442de0> as *const ()), // sub_14261a060
    Replacement::draft(0x14261a090, "unknown_14261a090", tpl_53382::<0x143a35498, 0x142442de0> as *const ()), // sub_14261a090
    Replacement::draft(0x14261a0c0, "unknown_14261a0c0", tpl_53382::<0x143a35268, 0x142442de0> as *const ()), // sub_14261a0c0
    Replacement::draft(0x14261a160, "unknown_14261a160", tpl_53264::<0x143a631d8, 0x14021cb50, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_14261a160
    Replacement::draft(0x14261a3d0, "unknown_14261a3d0", fn_14261a3d0 as *const ()), // sub_14261a3d0
    Replacement::draft_throws(0x14261a460, "unknown_14261a460", fn_14261a460 as *const (), Throws::Caught), // sub_14261a460
    Replacement::draft(0x14261a4a0, "unknown_14261a4a0", fn_14261a4a0 as *const ()), // sub_14261a4a0
    Replacement::draft(0x14261a610, "unknown_14261a610", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857f18, 0x143a00290> as *const ()), // sub_14261a610
    Replacement::draft(0x14261a6a0, "unknown_14261a6a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857f00, 0x143a00290> as *const ()), // sub_14261a6a0
    Replacement::draft(0x14261a750, "unknown_14261a750", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857fc0, 0x143a00290> as *const ()), // sub_14261a750
    Replacement::draft(0x14261a7e0, "unknown_14261a7e0", tpl_53272::<0x143a63700, 0x14022a610, 0x1434077a8, 0x143a636f0, 0x143a001ac, 0x143a00290> as *const ()), // sub_14261a7e0
    Replacement::draft(0x14261a870, "unknown_14261a870", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857fd8, 0x143a00290> as *const ()), // sub_14261a870
    Replacement::draft(0x14261a900, "unknown_14261a900", tpl_53272::<0x143a63728, 0x14022a610, 0x1434077a8, 0x143a63718, 0x143a001ac, 0x143a00290> as *const ()), // sub_14261a900
    Replacement::draft(0x14261a990, "unknown_14261a990", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857ff8, 0x143a00290> as *const ()), // sub_14261a990
    Replacement::draft(0x14261aa20, "unknown_14261aa20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143857fa8, 0x143a00290> as *const ()), // sub_14261aa20
    Replacement::draft(0x14261aab0, "unknown_14261aab0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881630, 0x143a00290> as *const ()), // sub_14261aab0
    Replacement::draft(0x14261ab40, "unknown_14261ab40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858010, 0x143a00290> as *const ()), // sub_14261ab40
    Replacement::draft(0x14261abd0, "unknown_14261abd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858098, 0x143a00290> as *const ()), // sub_14261abd0
    Replacement::draft(0x14261ac60, "unknown_14261ac60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438580f0, 0x143a00290> as *const ()), // sub_14261ac60
    Replacement::draft(0x14261acf0, "unknown_14261acf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858080, 0x143a00290> as *const ()), // sub_14261acf0
    Replacement::draft(0x14261ad80, "unknown_14261ad80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438580b8, 0x143a00290> as *const ()), // sub_14261ad80
    Replacement::draft(0x14261ae10, "unknown_14261ae10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438580d0, 0x143a00290> as *const ()), // sub_14261ae10
    Replacement::draft(0x14261aea0, "unknown_14261aea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858048, 0x143a00290> as *const ()), // sub_14261aea0
    Replacement::draft(0x14261af30, "unknown_14261af30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858028, 0x143a00290> as *const ()), // sub_14261af30
    Replacement::draft(0x14261afc0, "unknown_14261afc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858068, 0x143a00290> as *const ()), // sub_14261afc0
    Replacement::draft(0x14261b050, "unknown_14261b050", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858218, 0x143a00290> as *const ()), // sub_14261b050
    Replacement::draft(0x14261b0e0, "unknown_14261b0e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858298, 0x143a00290> as *const ()), // sub_14261b0e0
    Replacement::draft(0x14261b170, "unknown_14261b170", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858200, 0x143a00290> as *const ()), // sub_14261b170
    Replacement::draft(0x14261b200, "unknown_14261b200", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438581e8, 0x143a00290> as *const ()), // sub_14261b200
    Replacement::draft(0x14261b290, "unknown_14261b290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438582e0, 0x143a00290> as *const ()), // sub_14261b290
    Replacement::draft(0x14261b320, "unknown_14261b320", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438582c8, 0x143a00290> as *const ()), // sub_14261b320
    Replacement::draft(0x14261b3b0, "unknown_14261b3b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858268, 0x143a00290> as *const ()), // sub_14261b3b0
    Replacement::draft(0x14261b440, "unknown_14261b440", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438582b0, 0x143a00290> as *const ()), // sub_14261b440
    Replacement::draft(0x14261b4d0, "unknown_14261b4d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858230, 0x143a00290> as *const ()), // sub_14261b4d0
    Replacement::draft(0x14261b560, "unknown_14261b560", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858280, 0x143a00290> as *const ()), // sub_14261b560
    Replacement::draft(0x14261b5f0, "unknown_14261b5f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858108, 0x143a00290> as *const ()), // sub_14261b5f0
    Replacement::draft(0x14261b680, "unknown_14261b680", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858120, 0x143a00290> as *const ()), // sub_14261b680
    Replacement::draft(0x14261b710, "unknown_14261b710", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858300, 0x143a00290> as *const ()), // sub_14261b710
    Replacement::draft(0x14261b7a0, "unknown_14261b7a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858248, 0x143a00290> as *const ()), // sub_14261b7a0
    Replacement::draft(0x14261b830, "unknown_14261b830", tpl_53272::<0x143a63778, 0x14022a610, 0x1434077a8, 0x143a63768, 0x143a001ac, 0x143a00290> as *const ()), // sub_14261b830
    Replacement::draft(0x14261b8c0, "unknown_14261b8c0", tpl_53272::<0x143a63750, 0x14022a610, 0x1434077a8, 0x143a63740, 0x143a001ac, 0x143a00290> as *const ()), // sub_14261b8c0
    Replacement::draft_throws(0x14261b950, "unknown_14261b950", fn_14261b950 as *const (), Throws::Caught), // sub_14261b950
    Replacement::draft(0x14261ba60, "unknown_14261ba60", tpl_53264::<0x143a877b0, 0x14021cb50, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_14261ba60
    Replacement::draft(0x14261bbc0, "unknown_14261bbc0", tpl_53377::<0x143a877c0, 0x140246bb0, 0x1425371a0, 0x142533200> as *const ()), // sub_14261bbc0
    Replacement::draft(0x14261bd10, "unknown_14261bd10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143858570, 0x143a00290> as *const ()), // sub_14261bd10
    Replacement::draft(0x14261bdd0, "unknown_14261bdd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143861610, 0x143a00290> as *const ()), // sub_14261bdd0
    Replacement::draft(0x14261be60, "unknown_14261be60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143861590, 0x143a00290> as *const ()), // sub_14261be60
    Replacement::draft(0x14261bef0, "unknown_14261bef0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143861628, 0x143a00290> as *const ()), // sub_14261bef0
    Replacement::draft(0x14261bf80, "unknown_14261bf80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143861640, 0x143a00290> as *const ()), // sub_14261bf80
    Replacement::draft(0x14261cd40, "unknown_14261cd40", fn_14261cd40 as *const ()), // sub_14261cd40
    Replacement::draft(0x14261ce00, "unknown_14261ce00", fn_14261ce00 as *const ()), // sub_14261ce00
    Replacement::draft(0x14261ced0, "unknown_14261ced0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863a60, 0x143a00290> as *const ()), // sub_14261ced0
    Replacement::draft(0x14261cf60, "unknown_14261cf60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863b08, 0x143a00290> as *const ()), // sub_14261cf60
    Replacement::draft(0x14261cff0, "unknown_14261cff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863a78, 0x143a00290> as *const ()), // sub_14261cff0
    Replacement::draft(0x14261d080, "unknown_14261d080", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863af0, 0x143a00290> as *const ()), // sub_14261d080
    Replacement::draft(0x14261d110, "unknown_14261d110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863ad0, 0x143a00290> as *const ()), // sub_14261d110
    Replacement::draft(0x14261d1a0, "unknown_14261d1a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863ab0, 0x143a00290> as *const ()), // sub_14261d1a0
    Replacement::draft(0x14261d230, "unknown_14261d230", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863a90, 0x143a00290> as *const ()), // sub_14261d230
    Replacement::draft(0x14261d2d0, "unknown_14261d2d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864160, 0x143a00290> as *const ()), // sub_14261d2d0
    Replacement::draft(0x14261d360, "unknown_14261d360", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863cd0, 0x143a00290> as *const ()), // sub_14261d360
    Replacement::draft(0x14261d3f0, "unknown_14261d3f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863f28, 0x143a00290> as *const ()), // sub_14261d3f0
    Replacement::draft(0x14261d480, "unknown_14261d480", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863b50, 0x143a00290> as *const ()), // sub_14261d480
    Replacement::draft(0x14261d510, "unknown_14261d510", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863dc8, 0x143a00290> as *const ()), // sub_14261d510
    Replacement::draft(0x14261d5a0, "unknown_14261d5a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864198, 0x143a00290> as *const ()), // sub_14261d5a0
    Replacement::draft(0x14261d630, "unknown_14261d630", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863d08, 0x143a00290> as *const ()), // sub_14261d630
    Replacement::draft(0x14261d6c0, "unknown_14261d6c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863e00, 0x143a00290> as *const ()), // sub_14261d6c0
    Replacement::draft(0x14261d750, "unknown_14261d750", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864068, 0x143a00290> as *const ()), // sub_14261d750
    Replacement::draft(0x14261d7e0, "unknown_14261d7e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863e68, 0x143a00290> as *const ()), // sub_14261d7e0
    Replacement::draft(0x14261d870, "unknown_14261d870", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863cb8, 0x143a00290> as *const ()), // sub_14261d870
    Replacement::draft(0x14261d900, "unknown_14261d900", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863cf0, 0x143a00290> as *const ()), // sub_14261d900
    Replacement::draft(0x14261d990, "unknown_14261d990", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863ef8, 0x143a00290> as *const ()), // sub_14261d990
    Replacement::draft(0x14261da20, "unknown_14261da20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863bc0, 0x143a00290> as *const ()), // sub_14261da20
    Replacement::draft(0x14261dab0, "unknown_14261dab0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863f40, 0x143a00290> as *const ()), // sub_14261dab0
    Replacement::draft(0x14261db40, "unknown_14261db40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864088, 0x143a00290> as *const ()), // sub_14261db40
    Replacement::draft(0x14261dbd0, "unknown_14261dbd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863c00, 0x143a00290> as *const ()), // sub_14261dbd0
    Replacement::draft(0x14261dc60, "unknown_14261dc60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863c98, 0x143a00290> as *const ()), // sub_14261dc60
    Replacement::draft(0x14261dcf0, "unknown_14261dcf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863c50, 0x143a00290> as *const ()), // sub_14261dcf0
    Replacement::draft(0x14261dd80, "unknown_14261dd80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438640e0, 0x143a00290> as *const ()), // sub_14261dd80
    Replacement::draft(0x14261de10, "unknown_14261de10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863f90, 0x143a00290> as *const ()), // sub_14261de10
    Replacement::draft(0x14261dea0, "unknown_14261dea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864148, 0x143a00290> as *const ()), // sub_14261dea0
    Replacement::draft(0x14261df30, "unknown_14261df30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863fe0, 0x143a00290> as *const ()), // sub_14261df30
    Replacement::draft(0x14261dfc0, "unknown_14261dfc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863db0, 0x143a00290> as *const ()), // sub_14261dfc0
    Replacement::draft(0x14261e050, "unknown_14261e050", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864118, 0x143a00290> as *const ()), // sub_14261e050
    Replacement::draft(0x14261e0e0, "unknown_14261e0e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864180, 0x143a00290> as *const ()), // sub_14261e0e0
    Replacement::draft(0x14261e170, "unknown_14261e170", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863e50, 0x143a00290> as *const ()), // sub_14261e170
    Replacement::draft(0x14261e200, "unknown_14261e200", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863e38, 0x143a00290> as *const ()), // sub_14261e200
    Replacement::draft(0x14261e290, "unknown_14261e290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864050, 0x143a00290> as *const ()), // sub_14261e290
    Replacement::draft(0x14261e320, "unknown_14261e320", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863f78, 0x143a00290> as *const ()), // sub_14261e320
    Replacement::draft(0x14261e3b0, "unknown_14261e3b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863fc8, 0x143a00290> as *const ()), // sub_14261e3b0
    Replacement::draft(0x14261e440, "unknown_14261e440", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863c30, 0x143a00290> as *const ()), // sub_14261e440
    Replacement::draft(0x14261e4d0, "unknown_14261e4d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863c18, 0x143a00290> as *const ()), // sub_14261e4d0
    Replacement::draft(0x14261e560, "unknown_14261e560", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863b90, 0x143a00290> as *const ()), // sub_14261e560
    Replacement::draft(0x14261e5f0, "unknown_14261e5f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863f10, 0x143a00290> as *const ()), // sub_14261e5f0
    Replacement::draft(0x14261e680, "unknown_14261e680", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863ba8, 0x143a00290> as *const ()), // sub_14261e680
    Replacement::draft(0x14261e710, "unknown_14261e710", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863e20, 0x143a00290> as *const ()), // sub_14261e710
    Replacement::draft(0x14261e7a0, "unknown_14261e7a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863b38, 0x143a00290> as *const ()), // sub_14261e7a0
    Replacement::draft(0x14261e830, "unknown_14261e830", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863d60, 0x143a00290> as *const ()), // sub_14261e830
    Replacement::draft(0x14261e8c0, "unknown_14261e8c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438640f8, 0x143a00290> as *const ()), // sub_14261e8c0
    Replacement::draft(0x14261e950, "unknown_14261e950", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863d78, 0x143a00290> as *const ()), // sub_14261e950
    Replacement::draft(0x14261e9e0, "unknown_14261e9e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863fa8, 0x143a00290> as *const ()), // sub_14261e9e0
    Replacement::draft(0x14261ea70, "unknown_14261ea70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438640c0, 0x143a00290> as *const ()), // sub_14261ea70
    Replacement::draft(0x14261eb00, "unknown_14261eb00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863b70, 0x143a00290> as *const ()), // sub_14261eb00
    Replacement::draft(0x14261eb90, "unknown_14261eb90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438641b8, 0x143a00290> as *const ()), // sub_14261eb90
    Replacement::draft(0x14261ec20, "unknown_14261ec20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864130, 0x143a00290> as *const ()), // sub_14261ec20
    Replacement::draft(0x14261ecb0, "unknown_14261ecb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438640a8, 0x143a00290> as *const ()), // sub_14261ecb0
    Replacement::draft(0x14261ed40, "unknown_14261ed40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863f60, 0x143a00290> as *const ()), // sub_14261ed40
    Replacement::draft(0x14261edd0, "unknown_14261edd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863d48, 0x143a00290> as *const ()), // sub_14261edd0
    Replacement::draft(0x14261ee60, "unknown_14261ee60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863c80, 0x143a00290> as *const ()), // sub_14261ee60
    Replacement::draft(0x14261eef0, "unknown_14261eef0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863d98, 0x143a00290> as *const ()), // sub_14261eef0
    Replacement::draft(0x14261ef80, "unknown_14261ef80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863ea8, 0x143a00290> as *const ()), // sub_14261ef80
    Replacement::draft(0x14261f010, "unknown_14261f010", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863e88, 0x143a00290> as *const ()), // sub_14261f010
    Replacement::draft(0x14261f0a0, "unknown_14261f0a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863d28, 0x143a00290> as *const ()), // sub_14261f0a0
    Replacement::draft(0x14261f130, "unknown_14261f130", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863ff8, 0x143a00290> as *const ()), // sub_14261f130
    Replacement::draft(0x14261f1c0, "unknown_14261f1c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863ec0, 0x143a00290> as *const ()), // sub_14261f1c0
    Replacement::draft(0x14261f250, "unknown_14261f250", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863be0, 0x143a00290> as *const ()), // sub_14261f250
    Replacement::draft(0x14261f2e0, "unknown_14261f2e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864030, 0x143a00290> as *const ()), // sub_14261f2e0
    Replacement::draft(0x14261f370, "unknown_14261f370", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863ee0, 0x143a00290> as *const ()), // sub_14261f370
    Replacement::draft(0x14261f400, "unknown_14261f400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864018, 0x143a00290> as *const ()), // sub_14261f400
    Replacement::draft(0x14261f490, "unknown_14261f490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863c68, 0x143a00290> as *const ()), // sub_14261f490
    Replacement::draft(0x14261f520, "unknown_14261f520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143863de8, 0x143a00290> as *const ()), // sub_14261f520
    Replacement::draft(0x14261f5b0, "unknown_14261f5b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438641d8, 0x143a00290> as *const ()), // sub_14261f5b0
    Replacement::draft(0x14261f640, "unknown_14261f640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864210, 0x143a00290> as *const ()), // sub_14261f640
    Replacement::draft(0x14261f6d0, "unknown_14261f6d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864228, 0x143a00290> as *const ()), // sub_14261f6d0
    Replacement::draft(0x14261f760, "unknown_14261f760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864260, 0x143a00290> as *const ()), // sub_14261f760
    Replacement::draft(0x14261f7f0, "unknown_14261f7f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864240, 0x143a00290> as *const ()), // sub_14261f7f0
    Replacement::draft(0x14261f880, "unknown_14261f880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438641f0, 0x143a00290> as *const ()), // sub_14261f880
    Replacement::draft(0x14261f910, "unknown_14261f910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438643d0, 0x143a00290> as *const ()), // sub_14261f910
    Replacement::draft(0x14261f9a0, "unknown_14261f9a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864368, 0x143a00290> as *const ()), // sub_14261f9a0
    Replacement::draft(0x14261fa30, "unknown_14261fa30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864320, 0x143a00290> as *const ()), // sub_14261fa30
    Replacement::draft(0x14261fac0, "unknown_14261fac0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864280, 0x143a00290> as *const ()), // sub_14261fac0
    Replacement::draft(0x14261fb50, "unknown_14261fb50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864350, 0x143a00290> as *const ()), // sub_14261fb50
    Replacement::draft(0x14261fbe0, "unknown_14261fbe0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864298, 0x143a00290> as *const ()), // sub_14261fbe0
    Replacement::draft(0x14261fc70, "unknown_14261fc70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438642e8, 0x143a00290> as *const ()), // sub_14261fc70
    Replacement::draft(0x14261fd00, "unknown_14261fd00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438642b0, 0x143a00290> as *const ()), // sub_14261fd00
    Replacement::draft(0x14261fd90, "unknown_14261fd90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864338, 0x143a00290> as *const ()), // sub_14261fd90
    Replacement::draft(0x14261fe20, "unknown_14261fe20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438643b8, 0x143a00290> as *const ()), // sub_14261fe20
    Replacement::draft(0x14261feb0, "unknown_14261feb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864380, 0x143a00290> as *const ()), // sub_14261feb0
    Replacement::draft(0x14261ff40, "unknown_14261ff40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438642c8, 0x143a00290> as *const ()), // sub_14261ff40
    Replacement::draft(0x14261ffd0, "unknown_14261ffd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438643a0, 0x143a00290> as *const ()), // sub_14261ffd0
    Replacement::draft(0x142620060, "unknown_142620060", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864300, 0x143a00290> as *const ()), // sub_142620060
    Replacement::draft(0x1426200f0, "unknown_1426200f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438643e8, 0x143a00290> as *const ()), // sub_1426200f0
    Replacement::draft(0x142620180, "unknown_142620180", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864400, 0x143a00290> as *const ()), // sub_142620180
    Replacement::draft(0x142620210, "unknown_142620210", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864418, 0x143a00290> as *const ()), // sub_142620210
    Replacement::draft(0x1426202b0, "unknown_1426202b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864430, 0x143a00290> as *const ()), // sub_1426202b0
    Replacement::draft(0x142620340, "unknown_142620340", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864448, 0x143a00290> as *const ()), // sub_142620340
    Replacement::draft(0x1426203d0, "unknown_1426203d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864490, 0x143a00290> as *const ()), // sub_1426203d0
    Replacement::draft(0x142620460, "unknown_142620460", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864478, 0x143a00290> as *const ()), // sub_142620460
    Replacement::draft(0x1426204f0, "unknown_1426204f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864460, 0x143a00290> as *const ()), // sub_1426204f0
    Replacement::draft(0x142620580, "unknown_142620580", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438644e0, 0x143a00290> as *const ()), // sub_142620580
    Replacement::draft(0x142620610, "unknown_142620610", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438644a8, 0x143a00290> as *const ()), // sub_142620610
    Replacement::draft(0x1426206a0, "unknown_1426206a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864758, 0x143a00290> as *const ()), // sub_1426206a0
    Replacement::draft(0x142620730, "unknown_142620730", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864578, 0x143a00290> as *const ()), // sub_142620730
    Replacement::draft(0x142620880, "unknown_142620880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864688, 0x143a00290> as *const ()), // sub_142620880
    Replacement::draft(0x142620990, "unknown_142620990", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864810, 0x143a00290> as *const ()), // sub_142620990
    Replacement::draft(0x142620a20, "unknown_142620a20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438647f0, 0x143a00290> as *const ()), // sub_142620a20
    Replacement::draft(0x142620ab0, "unknown_142620ab0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438647d0, 0x143a00290> as *const ()), // sub_142620ab0
    Replacement::draft(0x142620b40, "unknown_142620b40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864868, 0x143a00290> as *const ()), // sub_142620b40
    Replacement::draft(0x142620bd0, "unknown_142620bd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864850, 0x143a00290> as *const ()), // sub_142620bd0
    Replacement::draft(0x142620c60, "unknown_142620c60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864830, 0x143a00290> as *const ()), // sub_142620c60
    Replacement::draft(0x142620cf0, "unknown_142620cf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438647b0, 0x143a00290> as *const ()), // sub_142620cf0
    Replacement::draft(0x142620d80, "unknown_142620d80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864880, 0x143a00290> as *const ()), // sub_142620d80
    Replacement::draft(0x142620e10, "unknown_142620e10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438648a0, 0x143a00290> as *const ()), // sub_142620e10
    Replacement::draft(0x142620ea0, "unknown_142620ea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864920, 0x143a00290> as *const ()), // sub_142620ea0
    Replacement::draft(0x142620f30, "unknown_142620f30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438648c0, 0x143a00290> as *const ()), // sub_142620f30
    Replacement::draft(0x142620fc0, "unknown_142620fc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438648d8, 0x143a00290> as *const ()), // sub_142620fc0
    Replacement::draft(0x142621050, "unknown_142621050", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438648f0, 0x143a00290> as *const ()), // sub_142621050
    Replacement::draft(0x1426210e0, "unknown_1426210e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864908, 0x143a00290> as *const ()), // sub_1426210e0
    Replacement::draft(0x142621170, "unknown_142621170", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438649d0, 0x143a00290> as *const ()), // sub_142621170
    Replacement::draft(0x142621200, "unknown_142621200", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864940, 0x143a00290> as *const ()), // sub_142621200
    Replacement::draft(0x142621290, "unknown_142621290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864978, 0x143a00290> as *const ()), // sub_142621290
    Replacement::draft(0x142621320, "unknown_142621320", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864998, 0x143a00290> as *const ()), // sub_142621320
    Replacement::draft(0x1426213b0, "unknown_1426213b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438649b0, 0x143a00290> as *const ()), // sub_1426213b0
    Replacement::draft(0x142621440, "unknown_142621440", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864958, 0x143a00290> as *const ()), // sub_142621440
    Replacement::draft(0x1426214d0, "unknown_1426214d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864aa0, 0x143a00290> as *const ()), // sub_1426214d0
    Replacement::draft(0x142621560, "unknown_142621560", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864a60, 0x143a00290> as *const ()), // sub_142621560
    Replacement::draft(0x1426215f0, "unknown_1426215f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864af0, 0x143a00290> as *const ()), // sub_1426215f0
    Replacement::draft(0x142621680, "unknown_142621680", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864a20, 0x143a00290> as *const ()), // sub_142621680
    Replacement::draft(0x142621710, "unknown_142621710", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864b28, 0x143a00290> as *const ()), // sub_142621710
    Replacement::draft(0x1426217a0, "unknown_1426217a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864ab8, 0x143a00290> as *const ()), // sub_1426217a0
    Replacement::draft(0x142621830, "unknown_142621830", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864a00, 0x143a00290> as *const ()), // sub_142621830
    Replacement::draft(0x1426218c0, "unknown_1426218c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864a40, 0x143a00290> as *const ()), // sub_1426218c0
    Replacement::draft(0x142621950, "unknown_142621950", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864a80, 0x143a00290> as *const ()), // sub_142621950
    Replacement::draft(0x1426219e0, "unknown_1426219e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864ad8, 0x143a00290> as *const ()), // sub_1426219e0
    Replacement::draft(0x142621a70, "unknown_142621a70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864b10, 0x143a00290> as *const ()), // sub_142621a70
    Replacement::draft(0x142621b00, "unknown_142621b00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438649e8, 0x143a00290> as *const ()), // sub_142621b00
    Replacement::draft(0x142621b90, "unknown_142621b90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864cd0, 0x143a00290> as *const ()), // sub_142621b90
    Replacement::draft(0x142621c20, "unknown_142621c20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864da0, 0x143a00290> as *const ()), // sub_142621c20
    Replacement::draft(0x142621cb0, "unknown_142621cb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864d68, 0x143a00290> as *const ()), // sub_142621cb0
    Replacement::draft(0x142621d40, "unknown_142621d40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864c28, 0x143a00290> as *const ()), // sub_142621d40
    Replacement::draft(0x142621dd0, "unknown_142621dd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864b78, 0x143a00290> as *const ()), // sub_142621dd0
    Replacement::draft(0x142621e60, "unknown_142621e60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864e00, 0x143a00290> as *const ()), // sub_142621e60
    Replacement::draft(0x142621ef0, "unknown_142621ef0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864c78, 0x143a00290> as *const ()), // sub_142621ef0
    Replacement::draft(0x142621f80, "unknown_142621f80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864bd0, 0x143a00290> as *const ()), // sub_142621f80
    Replacement::draft(0x142622010, "unknown_142622010", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864c98, 0x143a00290> as *const ()), // sub_142622010
    Replacement::draft(0x1426220a0, "unknown_1426220a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864d10, 0x143a00290> as *const ()), // sub_1426220a0
    Replacement::draft(0x142622130, "unknown_142622130", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864c48, 0x143a00290> as *const ()), // sub_142622130
    Replacement::draft(0x1426221c0, "unknown_1426221c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864b98, 0x143a00290> as *const ()), // sub_1426221c0
    Replacement::draft(0x142622250, "unknown_142622250", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864c08, 0x143a00290> as *const ()), // sub_142622250
    Replacement::draft(0x1426222e0, "unknown_1426222e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864dc0, 0x143a00290> as *const ()), // sub_1426222e0
    Replacement::draft(0x142622370, "unknown_142622370", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864b40, 0x143a00290> as *const ()), // sub_142622370
    Replacement::draft(0x142622400, "unknown_142622400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864d28, 0x143a00290> as *const ()), // sub_142622400
    Replacement::draft(0x142622490, "unknown_142622490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864cf0, 0x143a00290> as *const ()), // sub_142622490
    Replacement::draft(0x142622520, "unknown_142622520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864bb0, 0x143a00290> as *const ()), // sub_142622520
    Replacement::draft(0x1426225b0, "unknown_1426225b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864de0, 0x143a00290> as *const ()), // sub_1426225b0
    Replacement::draft(0x142622640, "unknown_142622640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864be8, 0x143a00290> as *const ()), // sub_142622640
    Replacement::draft(0x1426226d0, "unknown_1426226d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864b60, 0x143a00290> as *const ()), // sub_1426226d0
    Replacement::draft(0x142622760, "unknown_142622760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864cb0, 0x143a00290> as *const ()), // sub_142622760
    Replacement::draft(0x1426227f0, "unknown_1426227f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864d88, 0x143a00290> as *const ()), // sub_1426227f0
    Replacement::draft(0x142622880, "unknown_142622880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864d48, 0x143a00290> as *const ()), // sub_142622880
    Replacement::draft(0x142622910, "unknown_142622910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864c60, 0x143a00290> as *const ()), // sub_142622910
    Replacement::draft(0x1426229a0, "unknown_1426229a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864ea0, 0x143a00290> as *const ()), // sub_1426229a0
    Replacement::draft(0x142622a30, "unknown_142622a30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865160, 0x143a00290> as *const ()), // sub_142622a30
    Replacement::draft(0x142622ac0, "unknown_142622ac0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438650c8, 0x143a00290> as *const ()), // sub_142622ac0
    Replacement::draft(0x142622b50, "unknown_142622b50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864fe8, 0x143a00290> as *const ()), // sub_142622b50
    Replacement::draft(0x142622be0, "unknown_142622be0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864ef8, 0x143a00290> as *const ()), // sub_142622be0
    Replacement::draft(0x142622c70, "unknown_142622c70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864e38, 0x143a00290> as *const ()), // sub_142622c70
    Replacement::draft(0x142622d00, "unknown_142622d00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864fb0, 0x143a00290> as *const ()), // sub_142622d00
    Replacement::draft(0x142622d90, "unknown_142622d90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438651b8, 0x143a00290> as *const ()), // sub_142622d90
    Replacement::draft(0x142622e20, "unknown_142622e20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865070, 0x143a00290> as *const ()), // sub_142622e20
    Replacement::draft(0x142622eb0, "unknown_142622eb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864fc8, 0x143a00290> as *const ()), // sub_142622eb0
    Replacement::draft(0x142622f40, "unknown_142622f40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865108, 0x143a00290> as *const ()), // sub_142622f40
    Replacement::draft(0x142622fd0, "unknown_142622fd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864f18, 0x143a00290> as *const ()), // sub_142622fd0
    Replacement::draft(0x142623060, "unknown_142623060", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865028, 0x143a00290> as *const ()), // sub_142623060
    Replacement::draft(0x1426230f0, "unknown_1426230f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864e50, 0x143a00290> as *const ()), // sub_1426230f0
    Replacement::draft(0x142623180, "unknown_142623180", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438651a0, 0x143a00290> as *const ()), // sub_142623180
    Replacement::draft(0x142623210, "unknown_142623210", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865090, 0x143a00290> as *const ()), // sub_142623210
    Replacement::draft(0x1426232a0, "unknown_1426232a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438651f0, 0x143a00290> as *const ()), // sub_1426232a0
    Replacement::draft(0x142623330, "unknown_142623330", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438651d0, 0x143a00290> as *const ()), // sub_142623330
    Replacement::draft(0x1426233c0, "unknown_1426233c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864eb8, 0x143a00290> as *const ()), // sub_1426233c0
    Replacement::draft(0x142623450, "unknown_142623450", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438650e8, 0x143a00290> as *const ()), // sub_142623450
    Replacement::draft(0x1426234e0, "unknown_1426234e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865180, 0x143a00290> as *const ()), // sub_1426234e0
    Replacement::draft(0x142623570, "unknown_142623570", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864ed8, 0x143a00290> as *const ()), // sub_142623570
    Replacement::draft(0x142623600, "unknown_142623600", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864e20, 0x143a00290> as *const ()), // sub_142623600
    Replacement::draft(0x142623690, "unknown_142623690", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864f78, 0x143a00290> as *const ()), // sub_142623690
    Replacement::draft(0x142623720, "unknown_142623720", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864e68, 0x143a00290> as *const ()), // sub_142623720
    Replacement::draft(0x1426237b0, "unknown_1426237b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865128, 0x143a00290> as *const ()), // sub_1426237b0
    Replacement::draft(0x142623840, "unknown_142623840", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864f90, 0x143a00290> as *const ()), // sub_142623840
    Replacement::draft(0x1426238d0, "unknown_1426238d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864f58, 0x143a00290> as *const ()), // sub_1426238d0
    Replacement::draft(0x142623960, "unknown_142623960", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865040, 0x143a00290> as *const ()), // sub_142623960
    Replacement::draft(0x1426239f0, "unknown_1426239f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865008, 0x143a00290> as *const ()), // sub_1426239f0
    Replacement::draft(0x142623a80, "unknown_142623a80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865058, 0x143a00290> as *const ()), // sub_142623a80
    Replacement::draft(0x142623b10, "unknown_142623b10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864f38, 0x143a00290> as *const ()), // sub_142623b10
    Replacement::draft(0x142623ba0, "unknown_142623ba0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865148, 0x143a00290> as *const ()), // sub_142623ba0
    Replacement::draft(0x142623c30, "unknown_142623c30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438650b0, 0x143a00290> as *const ()), // sub_142623c30
    Replacement::draft(0x142623cc0, "unknown_142623cc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143864e80, 0x143a00290> as *const ()), // sub_142623cc0
    Replacement::draft(0x142623d50, "unknown_142623d50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865228, 0x143a00290> as *const ()), // sub_142623d50
    Replacement::draft(0x142623de0, "unknown_142623de0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865248, 0x143a00290> as *const ()), // sub_142623de0
    Replacement::draft(0x142623e70, "unknown_142623e70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865210, 0x143a00290> as *const ()), // sub_142623e70
    Replacement::draft(0x142624340, "unknown_142624340", tpl_53378::<0x143a89448, 0x1401e3450, 0x143a89438, 0x1401de830> as *const ()), // sub_142624340
    Replacement::draft(0x142624580, "unknown_142624580", tpl_53378::<0x143a898e8, 0x1401e3450, 0x143a898d8, 0x1401de830> as *const ()), // sub_142624580
    Replacement::draft(0x1426245b0, "unknown_1426245b0", tpl_53378::<0x143a89978, 0x1401e3450, 0x143a89968, 0x1401de830> as *const ()), // sub_1426245b0
    Replacement::draft(0x1426245e0, "unknown_1426245e0", tpl_53378::<0x143a899b8, 0x1401e3450, 0x143a899a8, 0x1401de830> as *const ()), // sub_1426245e0
    Replacement::draft(0x142624610, "unknown_142624610", tpl_53378::<0x143a89998, 0x1401e3450, 0x143a89988, 0x1401de830> as *const ()), // sub_142624610
    Replacement::draft(0x142624640, "unknown_142624640", tpl_53378::<0x143a899d8, 0x1401e3450, 0x143a899c8, 0x1401de830> as *const ()), // sub_142624640
    Replacement::draft(0x142624670, "unknown_142624670", tpl_53378::<0x143a898c8, 0x1401e3450, 0x143a898b8, 0x1401de830> as *const ()), // sub_142624670
    Replacement::draft(0x142624c80, "unknown_142624c80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865f80, 0x143a00290> as *const ()), // sub_142624c80
    Replacement::draft(0x142624d10, "unknown_142624d10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865f98, 0x143a00290> as *const ()), // sub_142624d10
    Replacement::draft(0x142624db0, "unknown_142624db0", fn_142624db0 as *const ()), // sub_142624db0
    Replacement::draft(0x142624de0, "unknown_142624de0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865fb0, 0x143a00290> as *const ()), // sub_142624de0
    Replacement::draft(0x142624e70, "unknown_142624e70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865fc8, 0x143a00290> as *const ()), // sub_142624e70
    Replacement::draft(0x142624f00, "unknown_142624f00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143865fe0, 0x143a00290> as *const ()), // sub_142624f00
    Replacement::draft(0x142624fa0, "unknown_142624fa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438660a8, 0x143a00290> as *const ()), // sub_142624fa0
    Replacement::draft(0x142625270, "unknown_142625270", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866840, 0x143a00290> as *const ()), // sub_142625270
    Replacement::draft(0x142625410, "unknown_142625410", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438818e0, 0x143a00290> as *const ()), // sub_142625410
    Replacement::draft(0x1426254a0, "unknown_1426254a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881880, 0x143a00290> as *const ()), // sub_1426254a0
    Replacement::draft(0x142625530, "unknown_142625530", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438818a0, 0x143a00290> as *const ()), // sub_142625530
    Replacement::draft(0x1426255c0, "unknown_1426255c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438818c0, 0x143a00290> as *const ()), // sub_1426255c0
    Replacement::draft(0x142625650, "unknown_142625650", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866c10, 0x143a00290> as *const ()), // sub_142625650
    Replacement::draft(0x142625700, "unknown_142625700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866b10, 0x143a00290> as *const ()), // sub_142625700
    Replacement::draft(0x142625790, "unknown_142625790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438669f0, 0x143a00290> as *const ()), // sub_142625790
    Replacement::draft(0x142625840, "unknown_142625840", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866a30, 0x143a00290> as *const ()), // sub_142625840
    Replacement::draft(0x1426258f0, "unknown_1426258f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866a10, 0x143a00290> as *const ()), // sub_1426258f0
    Replacement::draft(0x142625980, "unknown_142625980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866858, 0x143a00290> as *const ()), // sub_142625980
    Replacement::draft(0x142625a30, "unknown_142625a30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438669b0, 0x143a00290> as *const ()), // sub_142625a30
    Replacement::draft(0x142625ac0, "unknown_142625ac0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866970, 0x143a00290> as *const ()), // sub_142625ac0
    Replacement::draft(0x142625b50, "unknown_142625b50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866890, 0x143a00290> as *const ()), // sub_142625b50
    Replacement::draft(0x142625be0, "unknown_142625be0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438669d0, 0x143a00290> as *const ()), // sub_142625be0
    Replacement::draft(0x142625c70, "unknown_142625c70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866878, 0x143a00290> as *const ()), // sub_142625c70
    Replacement::draft(0x142625d70, "unknown_142625d70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143ceea28, 0x143a00290> as *const ()), // sub_142625d70
    Replacement::draft(0x142625e50, "unknown_142625e50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143ceecd0, 0x143a00290> as *const ()), // sub_142625e50
    Replacement::draft(0x142626060, "unknown_142626060", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881900, 0x143a00290> as *const ()), // sub_142626060
    Replacement::draft(0x1426260f0, "unknown_1426260f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881918, 0x143a00290> as *const ()), // sub_1426260f0
    Replacement::draft(0x1426262f0, "unknown_1426262f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866c88, 0x143a00290> as *const ()), // sub_1426262f0
    Replacement::draft(0x142626380, "unknown_142626380", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866ca0, 0x143a00290> as *const ()), // sub_142626380
    Replacement::draft(0x142626410, "unknown_142626410", tpl_53272::<0x143a8acc8, 0x14022a610, 0x1434077a8, 0x143a8acb8, 0x143a001ac, 0x143a00290> as *const ()), // sub_142626410
    Replacement::draft(0x1426264a0, "unknown_1426264a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866c58, 0x143a00290> as *const ()), // sub_1426264a0
    Replacement::draft(0x142626530, "unknown_142626530", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866cd8, 0x143a00290> as *const ()), // sub_142626530
    Replacement::draft(0x1426265c0, "unknown_1426265c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866cb8, 0x143a00290> as *const ()), // sub_1426265c0
    Replacement::draft(0x1426266c0, "unknown_1426266c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866cf8, 0x143a00290> as *const ()), // sub_1426266c0
    Replacement::draft(0x1426267d0, "unknown_1426267d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866e70, 0x143a00290> as *const ()), // sub_1426267d0
    Replacement::draft(0x142626860, "unknown_142626860", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866e50, 0x143a00290> as *const ()), // sub_142626860
    Replacement::draft(0x1426268f0, "unknown_1426268f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866ff8, 0x143a00290> as *const ()), // sub_1426268f0
    Replacement::draft(0x142626980, "unknown_142626980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866df0, 0x143a00290> as *const ()), // sub_142626980
    Replacement::draft(0x142626a10, "unknown_142626a10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866ec8, 0x143a00290> as *const ()), // sub_142626a10
    Replacement::draft(0x142626aa0, "unknown_142626aa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866e20, 0x143a00290> as *const ()), // sub_142626aa0
    Replacement::draft(0x142626b30, "unknown_142626b30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866d10, 0x143a00290> as *const ()), // sub_142626b30
    Replacement::draft(0x142626bc0, "unknown_142626bc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866e38, 0x143a00290> as *const ()), // sub_142626bc0
    Replacement::draft(0x142626c50, "unknown_142626c50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867018, 0x143a00290> as *const ()), // sub_142626c50
    Replacement::draft(0x142626ce0, "unknown_142626ce0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866fe0, 0x143a00290> as *const ()), // sub_142626ce0
    Replacement::draft(0x142626d70, "unknown_142626d70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866e08, 0x143a00290> as *const ()), // sub_142626d70
    Replacement::draft(0x142626e00, "unknown_142626e00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866eb0, 0x143a00290> as *const ()), // sub_142626e00
    Replacement::draft(0x142626e90, "unknown_142626e90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867030, 0x143a00290> as *const ()), // sub_142626e90
    Replacement::draft(0x142626f20, "unknown_142626f20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866e90, 0x143a00290> as *const ()), // sub_142626f20
    Replacement::draft(0x142626fb0, "unknown_142626fb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143866ee0, 0x143a00290> as *const ()), // sub_142626fb0
    Replacement::draft(0x142627400, "unknown_142627400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438677d0, 0x143a00290> as *const ()), // sub_142627400
    Replacement::draft(0x142627630, "unknown_142627630", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867b30, 0x143a00290> as *const ()), // sub_142627630
    Replacement::draft(0x142627840, "unknown_142627840", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867c10, 0x143a00290> as *const ()), // sub_142627840
    Replacement::draft(0x1426278d0, "unknown_1426278d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867ce8, 0x143a00290> as *const ()), // sub_1426278d0
    Replacement::draft(0x142627960, "unknown_142627960", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867c28, 0x143a00290> as *const ()), // sub_142627960
    Replacement::draft(0x1426279f0, "unknown_1426279f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867d20, 0x143a00290> as *const ()), // sub_1426279f0
    Replacement::draft(0x142627a80, "unknown_142627a80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867d60, 0x143a00290> as *const ()), // sub_142627a80
    Replacement::draft(0x142627b10, "unknown_142627b10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867ca8, 0x143a00290> as *const ()), // sub_142627b10
    Replacement::draft(0x142627ba0, "unknown_142627ba0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867c48, 0x143a00290> as *const ()), // sub_142627ba0
    Replacement::draft(0x142627c30, "unknown_142627c30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867c88, 0x143a00290> as *const ()), // sub_142627c30
    Replacement::draft(0x142627cc0, "unknown_142627cc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867c68, 0x143a00290> as *const ()), // sub_142627cc0
    Replacement::draft(0x142627d50, "unknown_142627d50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867cc8, 0x143a00290> as *const ()), // sub_142627d50
    Replacement::draft(0x142627de0, "unknown_142627de0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867d40, 0x143a00290> as *const ()), // sub_142627de0
    Replacement::draft(0x142627e70, "unknown_142627e70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867d80, 0x143a00290> as *const ()), // sub_142627e70
    Replacement::draft(0x142627f00, "unknown_142627f00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867d08, 0x143a00290> as *const ()), // sub_142627f00
    Replacement::draft(0x142627f90, "unknown_142627f90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867d98, 0x143a00290> as *const ()), // sub_142627f90
    Replacement::draft(0x142628020, "unknown_142628020", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867db0, 0x143a00290> as *const ()), // sub_142628020
    Replacement::draft(0x1426280f0, "unknown_1426280f0", fn_1426280f0 as *const ()), // sub_1426280f0
    Replacement::draft(0x142628130, "unknown_142628130", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867e70, 0x143a00290> as *const ()), // sub_142628130
    Replacement::draft(0x1426281c0, "unknown_1426281c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867e30, 0x143a00290> as *const ()), // sub_1426281c0
    Replacement::draft(0x142628250, "unknown_142628250", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867e10, 0x143a00290> as *const ()), // sub_142628250
    Replacement::draft(0x1426282e0, "unknown_1426282e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867e50, 0x143a00290> as *const ()), // sub_1426282e0
    Replacement::draft(0x142628490, "unknown_142628490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867fd0, 0x143a00290> as *const ()), // sub_142628490
    Replacement::draft(0x142628520, "unknown_142628520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868020, 0x143a00290> as *const ()), // sub_142628520
    Replacement::draft(0x1426285b0, "unknown_1426285b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868008, 0x143a00290> as *const ()), // sub_1426285b0
    Replacement::draft(0x142628640, "unknown_142628640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867fb8, 0x143a00290> as *const ()), // sub_142628640
    Replacement::draft(0x1426286d0, "unknown_1426286d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867f28, 0x143a00290> as *const ()), // sub_1426286d0
    Replacement::draft(0x142628760, "unknown_142628760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867ef8, 0x143a00290> as *const ()), // sub_142628760
    Replacement::draft(0x1426287f0, "unknown_1426287f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867f58, 0x143a00290> as *const ()), // sub_1426287f0
    Replacement::draft(0x142628880, "unknown_142628880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438680d0, 0x143a00290> as *const ()), // sub_142628880
    Replacement::draft(0x142628910, "unknown_142628910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867f70, 0x143a00290> as *const ()), // sub_142628910
    Replacement::draft(0x1426289a0, "unknown_1426289a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438680b8, 0x143a00290> as *const ()), // sub_1426289a0
    Replacement::draft(0x142628a30, "unknown_142628a30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868088, 0x143a00290> as *const ()), // sub_142628a30
    Replacement::draft(0x142628ac0, "unknown_142628ac0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438680a0, 0x143a00290> as *const ()), // sub_142628ac0
    Replacement::draft(0x142628b50, "unknown_142628b50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867f10, 0x143a00290> as *const ()), // sub_142628b50
    Replacement::draft(0x142628be0, "unknown_142628be0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867e88, 0x143a00290> as *const ()), // sub_142628be0
    Replacement::draft(0x142628c70, "unknown_142628c70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867fa0, 0x143a00290> as *const ()), // sub_142628c70
    Replacement::draft(0x142628d00, "unknown_142628d00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867ee0, 0x143a00290> as *const ()), // sub_142628d00
    Replacement::draft(0x142628d90, "unknown_142628d90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867f40, 0x143a00290> as *const ()), // sub_142628d90
    Replacement::draft(0x142628e20, "unknown_142628e20", tpl_53272::<0x143bbce98, 0x14022a610, 0x1434077a8, 0x143bbce88, 0x143a001ac, 0x143a00290> as *const ()), // sub_142628e20
    Replacement::draft(0x142628eb0, "unknown_142628eb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143867f88, 0x143a00290> as *const ()), // sub_142628eb0
    Replacement::draft_throws(0x142628fa0, "unknown_142628fa0", fn_142628fa0 as *const (), Throws::Caught), // sub_142628fa0
    Replacement::draft(0x142629100, "unknown_142629100", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cf0988, 0x143a00290> as *const ()), // sub_142629100
    Replacement::draft(0x1426294e0, "unknown_1426294e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cf0418, 0x143a00290> as *const ()), // sub_1426294e0
    Replacement::draft(0x142629570, "unknown_142629570", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cf0650, 0x143a00290> as *const ()), // sub_142629570
    Replacement::draft(0x1426296e0, "unknown_1426296e0", tpl_53272::<0x143bbce70, 0x14022a610, 0x1434077a8, 0x143bbce60, 0x143a001ac, 0x143a00290> as *const ()), // sub_1426296e0
    Replacement::draft(0x1426297a0, "unknown_1426297a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868190, 0x143a00290> as *const ()), // sub_1426297a0
    Replacement::draft(0x142629830, "unknown_142629830", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868160, 0x143a00290> as *const ()), // sub_142629830
    Replacement::draft(0x1426298c0, "unknown_1426298c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868178, 0x143a00290> as *const ()), // sub_1426298c0
    Replacement::draft(0x142629a10, "unknown_142629a10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868140, 0x143a00290> as *const ()), // sub_142629a10
    Replacement::draft(0x142629aa0, "unknown_142629aa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868250, 0x143a00290> as *const ()), // sub_142629aa0
    Replacement::draft(0x142629b30, "unknown_142629b30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868238, 0x143a00290> as *const ()), // sub_142629b30
    Replacement::draft(0x142629bc0, "unknown_142629bc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868218, 0x143a00290> as *const ()), // sub_142629bc0
    Replacement::draft(0x142629c50, "unknown_142629c50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868200, 0x143a00290> as *const ()), // sub_142629c50
    Replacement::draft(0x142629ce0, "unknown_142629ce0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438681e0, 0x143a00290> as *const ()), // sub_142629ce0
    Replacement::draft(0x142629d70, "unknown_142629d70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438681b0, 0x143a00290> as *const ()), // sub_142629d70
    Replacement::draft(0x142629e00, "unknown_142629e00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438819f0, 0x143a00290> as *const ()), // sub_142629e00
    Replacement::draft(0x142629fb0, "unknown_142629fb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438682c0, 0x143a00290> as *const ()), // sub_142629fb0
    Replacement::draft(0x14262a040, "unknown_14262a040", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438682f8, 0x143a00290> as *const ()), // sub_14262a040
    Replacement::draft(0x14262a0d0, "unknown_14262a0d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868310, 0x143a00290> as *const ()), // sub_14262a0d0
    Replacement::draft(0x14262a160, "unknown_14262a160", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438682d8, 0x143a00290> as *const ()), // sub_14262a160
    Replacement::draft(0x14262a1f0, "unknown_14262a1f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868350, 0x143a00290> as *const ()), // sub_14262a1f0
    Replacement::draft(0x14262a280, "unknown_14262a280", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868330, 0x143a00290> as *const ()), // sub_14262a280
    Replacement::draft(0x14262a310, "unknown_14262a310", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881a08, 0x143a00290> as *const ()), // sub_14262a310
    Replacement::draft(0x14262a400, "unknown_14262a400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868370, 0x143a00290> as *const ()), // sub_14262a400
    Replacement::draft(0x14262a490, "unknown_14262a490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868420, 0x143a00290> as *const ()), // sub_14262a490
    Replacement::draft(0x14262a520, "unknown_14262a520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438683e0, 0x143a00290> as *const ()), // sub_14262a520
    Replacement::draft(0x14262a5b0, "unknown_14262a5b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868400, 0x143a00290> as *const ()), // sub_14262a5b0
    Replacement::draft(0x14262a640, "unknown_14262a640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438683c0, 0x143a00290> as *const ()), // sub_14262a640
    Replacement::draft(0x14262a6d0, "unknown_14262a6d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868390, 0x143a00290> as *const ()), // sub_14262a6d0
    Replacement::draft(0x14262a770, "unknown_14262a770", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868440, 0x143a00290> as *const ()), // sub_14262a770
    Replacement::draft(0x14262a9b0, "unknown_14262a9b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868580, 0x143a00290> as *const ()), // sub_14262a9b0
    Replacement::draft(0x14262ab10, "unknown_14262ab10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868880, 0x143a00290> as *const ()), // sub_14262ab10
    Replacement::draft(0x14262aba0, "unknown_14262aba0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868790, 0x143a00290> as *const ()), // sub_14262aba0
    Replacement::draft(0x14262ac30, "unknown_14262ac30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868660, 0x143a00290> as *const ()), // sub_14262ac30
    Replacement::draft(0x14262acc0, "unknown_14262acc0", tpl_53272::<0x143bbcfa0, 0x14022a610, 0x1434077a8, 0x143bbcf90, 0x143a001ac, 0x143a00290> as *const ()), // sub_14262acc0
    Replacement::draft(0x14262ad50, "unknown_14262ad50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438688b0, 0x143a00290> as *const ()), // sub_14262ad50
    Replacement::draft(0x14262ade0, "unknown_14262ade0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868898, 0x143a00290> as *const ()), // sub_14262ade0
    Replacement::draft(0x14262ae70, "unknown_14262ae70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143868778, 0x143a00290> as *const ()), // sub_14262ae70
    Replacement::draft(0x14262af00, "unknown_14262af00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438687a8, 0x143a00290> as *const ()), // sub_14262af00
    Replacement::draft(0x14262b170, "unknown_14262b170", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438688e0, 0x143a00290> as *const ()), // sub_14262b170
    Replacement::draft(0x14262b200, "unknown_14262b200", tpl_53392::<0x143a33638, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14262b200
    Replacement::draft(0x14262b240, "unknown_14262b240", tpl_53392::<0x143a335e8, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14262b240
    Replacement::draft(0x14262b290, "unknown_14262b290", tpl_53392::<0x143a334b8, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14262b290
    Replacement::draft(0x14262b610, "unknown_14262b610", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869890, 0x143a00290> as *const ()), // sub_14262b610
    Replacement::draft(0x14262b6a0, "unknown_14262b6a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869820, 0x143a00290> as *const ()), // sub_14262b6a0
    Replacement::draft(0x14262b790, "unknown_14262b790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869850, 0x143a00290> as *const ()), // sub_14262b790
    Replacement::draft(0x14262b820, "unknown_14262b820", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869838, 0x143a00290> as *const ()), // sub_14262b820
    Replacement::draft(0x14262b8b0, "unknown_14262b8b0", fn_14262b8b0 as *const ()), // sub_14262b8b0
    Replacement::draft(0x14262b920, "unknown_14262b920", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869c10, 0x143a00290> as *const ()), // sub_14262b920
    Replacement::draft(0x14262b9b0, "unknown_14262b9b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869ce8, 0x143a00290> as *const ()), // sub_14262b9b0
    Replacement::draft(0x14262ba40, "unknown_14262ba40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a1f0, 0x143a00290> as *const ()), // sub_14262ba40
    Replacement::draft(0x14262bad0, "unknown_14262bad0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a608, 0x143a00290> as *const ()), // sub_14262bad0
    Replacement::draft(0x14262bb60, "unknown_14262bb60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a0b0, 0x143a00290> as *const ()), // sub_14262bb60
    Replacement::draft(0x14262bbf0, "unknown_14262bbf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869e70, 0x143a00290> as *const ()), // sub_14262bbf0
    Replacement::draft(0x14262bc80, "unknown_14262bc80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a090, 0x143a00290> as *const ()), // sub_14262bc80
    Replacement::draft(0x14262bd10, "unknown_14262bd10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869c60, 0x143a00290> as *const ()), // sub_14262bd10
    Replacement::draft(0x14262bda0, "unknown_14262bda0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869a30, 0x143a00290> as *const ()), // sub_14262bda0
    Replacement::draft(0x14262be30, "unknown_14262be30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869cc8, 0x143a00290> as *const ()), // sub_14262be30
    Replacement::draft(0x14262bec0, "unknown_14262bec0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869a70, 0x143a00290> as *const ()), // sub_14262bec0
    Replacement::draft(0x14262bf50, "unknown_14262bf50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a448, 0x143a00290> as *const ()), // sub_14262bf50
    Replacement::draft(0x14262bfe0, "unknown_14262bfe0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a538, 0x143a00290> as *const ()), // sub_14262bfe0
    Replacement::draft(0x14262c070, "unknown_14262c070", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a498, 0x143a00290> as *const ()), // sub_14262c070
    Replacement::draft(0x14262c110, "unknown_14262c110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438698a8, 0x143a00290> as *const ()), // sub_14262c110
    Replacement::draft(0x14262c1a0, "unknown_14262c1a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a620, 0x143a00290> as *const ()), // sub_14262c1a0
    Replacement::draft(0x14262c230, "unknown_14262c230", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869ab0, 0x143a00290> as *const ()), // sub_14262c230
    Replacement::draft(0x14262c2c0, "unknown_14262c2c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869d08, 0x143a00290> as *const ()), // sub_14262c2c0
    Replacement::draft(0x14262c350, "unknown_14262c350", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869e38, 0x143a00290> as *const ()), // sub_14262c350
    Replacement::draft(0x14262c3e0, "unknown_14262c3e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869c78, 0x143a00290> as *const ()), // sub_14262c3e0
    Replacement::draft(0x14262c470, "unknown_14262c470", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869ca8, 0x143a00290> as *const ()), // sub_14262c470
    Replacement::draft(0x14262c500, "unknown_14262c500", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a058, 0x143a00290> as *const ()), // sub_14262c500
    Replacement::draft(0x14262c590, "unknown_14262c590", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869c28, 0x143a00290> as *const ()), // sub_14262c590
    Replacement::draft(0x14262c620, "unknown_14262c620", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869fc0, 0x143a00290> as *const ()), // sub_14262c620
    Replacement::draft(0x14262c6b0, "unknown_14262c6b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869a10, 0x143a00290> as *const ()), // sub_14262c6b0
    Replacement::draft(0x14262c740, "unknown_14262c740", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869a50, 0x143a00290> as *const ()), // sub_14262c740
    Replacement::draft(0x14262c7d0, "unknown_14262c7d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a5f0, 0x143a00290> as *const ()), // sub_14262c7d0
    Replacement::draft(0x14262c860, "unknown_14262c860", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a5a0, 0x143a00290> as *const ()), // sub_14262c860
    Replacement::draft(0x14262c8f0, "unknown_14262c8f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a5c0, 0x143a00290> as *const ()), // sub_14262c8f0
    Replacement::draft(0x14262c980, "unknown_14262c980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869908, 0x143a00290> as *const ()), // sub_14262c980
    Replacement::draft(0x14262ca10, "unknown_14262ca10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a260, 0x143a00290> as *const ()), // sub_14262ca10
    Replacement::draft(0x14262caa0, "unknown_14262caa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a570, 0x143a00290> as *const ()), // sub_14262caa0
    Replacement::draft(0x14262cb30, "unknown_14262cb30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a508, 0x143a00290> as *const ()), // sub_14262cb30
    Replacement::draft(0x14262cbc0, "unknown_14262cbc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a1c0, 0x143a00290> as *const ()), // sub_14262cbc0
    Replacement::draft(0x14262cc50, "unknown_14262cc50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a3a8, 0x143a00290> as *const ()), // sub_14262cc50
    Replacement::draft(0x14262ce20, "unknown_14262ce20", fn_14262ce20 as *const ()), // sub_14262ce20
    Replacement::draft_throws(0x14262ce40, "unknown_14262ce40", fn_14262ce40 as *const (), Throws::Caught), // sub_14262ce40
    Replacement::draft_throws(0x14262ce80, "unknown_14262ce80", fn_14262ce80 as *const (), Throws::Caught), // sub_14262ce80
    Replacement::draft_throws(0x14262ceb0, "unknown_14262ceb0", fn_14262ceb0 as *const (), Throws::Caught), // sub_14262ceb0
    Replacement::draft(0x14262d020, "unknown_14262d020", tpl_53272::<0x143bbd118, 0x14022a610, 0x1434077a8, 0x143bbd108, 0x143a001ac, 0x143a00290> as *const ()), // sub_14262d020
    Replacement::draft(0x14262d0b0, "unknown_14262d0b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a460, 0x143a00290> as *const ()), // sub_14262d0b0
    Replacement::draft(0x14262d140, "unknown_14262d140", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438698e8, 0x143a00290> as *const ()), // sub_14262d140
    Replacement::draft(0x14262d1d0, "unknown_14262d1d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869f50, 0x143a00290> as *const ()), // sub_14262d1d0
    Replacement::draft(0x14262d260, "unknown_14262d260", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a210, 0x143a00290> as *const ()), // sub_14262d260
    Replacement::draft(0x14262d350, "unknown_14262d350", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869f68, 0x143a00290> as *const ()), // sub_14262d350
    Replacement::draft(0x14262d670, "unknown_14262d670", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869d28, 0x143a00290> as *const ()), // sub_14262d670
    Replacement::draft(0x14262d700, "unknown_14262d700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869e18, 0x143a00290> as *const ()), // sub_14262d700
    Replacement::draft(0x14262d7f0, "unknown_14262d7f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869fa0, 0x143a00290> as *const ()), // sub_14262d7f0
    Replacement::draft(0x14262d9d0, "unknown_14262d9d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881b50, 0x143a00290> as *const ()), // sub_14262d9d0
    Replacement::draft(0x14262da60, "unknown_14262da60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881b10, 0x143a00290> as *const ()), // sub_14262da60
    Replacement::draft(0x14262e230, "unknown_14262e230", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cf32a8, 0x143a00290> as *const ()), // sub_14262e230
    Replacement::draft(0x14262e590, "unknown_14262e590", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cf43c0, 0x143a00290> as *const ()), // sub_14262e590
    Replacement::draft(0x14262e620, "unknown_14262e620", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cf3f10, 0x143a00290> as *const ()), // sub_14262e620
    Replacement::draft(0x14262e6b0, "unknown_14262e6b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cf3930, 0x143a00290> as *const ()), // sub_14262e6b0
    Replacement::draft(0x14262e870, "unknown_14262e870", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881b68, 0x143a00290> as *const ()), // sub_14262e870
    Replacement::draft(0x14262e9d0, "unknown_14262e9d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881ae0, 0x143a00290> as *const ()), // sub_14262e9d0
    Replacement::draft(0x14262ea60, "unknown_14262ea60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881ab0, 0x143a00290> as *const ()), // sub_14262ea60
    Replacement::draft(0x14262eaf0, "unknown_14262eaf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881ac8, 0x143a00290> as *const ()), // sub_14262eaf0
    Replacement::draft(0x14262eb80, "unknown_14262eb80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881af8, 0x143a00290> as *const ()), // sub_14262eb80
    Replacement::draft(0x14262ec90, "unknown_14262ec90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143869fd8, 0x143a00290> as *const ()), // sub_14262ec90
    Replacement::draft(0x14262ed20, "unknown_14262ed20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a010, 0x143a00290> as *const ()), // sub_14262ed20
    Replacement::draft(0x14262ee30, "unknown_14262ee30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881b30, 0x143a00290> as *const ()), // sub_14262ee30
    Replacement::draft(0x14262eee0, "unknown_14262eee0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a708, 0x143a00290> as *const ()), // sub_14262eee0
    Replacement::draft(0x14262ef70, "unknown_14262ef70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a720, 0x143a00290> as *const ()), // sub_14262ef70
    Replacement::draft(0x14262f000, "unknown_14262f000", tpl_53272::<0x143bbd338, 0x14022a610, 0x1434077a8, 0x143bbd328, 0x143a001ac, 0x143a00290> as *const ()), // sub_14262f000
    Replacement::draft(0x14262f090, "unknown_14262f090", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a690, 0x143a00290> as *const ()), // sub_14262f090
    Replacement::draft(0x14262f120, "unknown_14262f120", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a6b0, 0x143a00290> as *const ()), // sub_14262f120
    Replacement::draft(0x14262f1b0, "unknown_14262f1b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a670, 0x143a00290> as *const ()), // sub_14262f1b0
    Replacement::draft(0x14262f240, "unknown_14262f240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a6e8, 0x143a00290> as *const ()), // sub_14262f240
    Replacement::draft(0x14262f2d0, "unknown_14262f2d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a658, 0x143a00290> as *const ()), // sub_14262f2d0
    Replacement::draft(0x14262f360, "unknown_14262f360", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a6d0, 0x143a00290> as *const ()), // sub_14262f360
    Replacement::draft(0x14262f440, "unknown_14262f440", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a7c0, 0x143a00290> as *const ()), // sub_14262f440
    Replacement::draft(0x14262f4d0, "unknown_14262f4d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a7a8, 0x143a00290> as *const ()), // sub_14262f4d0
    Replacement::draft(0x14262f560, "unknown_14262f560", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a7f8, 0x143a00290> as *const ()), // sub_14262f560
    Replacement::draft(0x14262f5f0, "unknown_14262f5f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a810, 0x143a00290> as *const ()), // sub_14262f5f0
    Replacement::draft(0x14262f680, "unknown_14262f680", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a790, 0x143a00290> as *const ()), // sub_14262f680
    Replacement::draft(0x14262f9f0, "unknown_14262f9f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a998, 0x143a00290> as *const ()), // sub_14262f9f0
    Replacement::draft(0x14262fa80, "unknown_14262fa80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a978, 0x143a00290> as *const ()), // sub_14262fa80
    Replacement::draft(0x14262fb10, "unknown_14262fb10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386a9b0, 0x143a00290> as *const ()), // sub_14262fb10
    Replacement::draft_throws(0x14262fcf0, "unknown_14262fcf0", fn_14262fcf0 as *const (), Throws::Caught), // sub_14262fcf0
    Replacement::draft_throws(0x14262fd30, "unknown_14262fd30", fn_14262fd30 as *const (), Throws::Caught), // sub_14262fd30
    Replacement::draft_throws(0x14262fd70, "unknown_14262fd70", fn_14262fd70 as *const (), Throws::Caught), // sub_14262fd70
    Replacement::draft_throws(0x14262fda0, "unknown_14262fda0", fn_14262fda0 as *const (), Throws::Caught), // sub_14262fda0
    Replacement::draft(0x1426306d0, "unknown_1426306d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386af20, 0x143a00290> as *const ()), // sub_1426306d0
    Replacement::draft(0x142630760, "unknown_142630760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386af40, 0x143a00290> as *const ()), // sub_142630760
    Replacement::draft(0x1426307f0, "unknown_1426307f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386af70, 0x143a00290> as *const ()), // sub_1426307f0
    Replacement::draft(0x142630a20, "unknown_142630a20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b2d0, 0x143a00290> as *const ()), // sub_142630a20
    Replacement::draft(0x142630ab0, "unknown_142630ab0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b628, 0x143a00290> as *const ()), // sub_142630ab0
    Replacement::draft(0x142630b40, "unknown_142630b40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b178, 0x143a00290> as *const ()), // sub_142630b40
    Replacement::draft(0x142630bd0, "unknown_142630bd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b568, 0x143a00290> as *const ()), // sub_142630bd0
    Replacement::draft(0x142630c60, "unknown_142630c60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b290, 0x143a00290> as *const ()), // sub_142630c60
    Replacement::draft(0x142630cf0, "unknown_142630cf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b218, 0x143a00290> as *const ()), // sub_142630cf0
    Replacement::draft(0x142630d80, "unknown_142630d80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b100, 0x143a00290> as *const ()), // sub_142630d80
    Replacement::draft(0x142630e10, "unknown_142630e10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b410, 0x143a00290> as *const ()), // sub_142630e10
    Replacement::draft(0x142630ea0, "unknown_142630ea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b360, 0x143a00290> as *const ()), // sub_142630ea0
    Replacement::draft(0x142630f30, "unknown_142630f30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b6a8, 0x143a00290> as *const ()), // sub_142630f30
    Replacement::draft(0x142630fc0, "unknown_142630fc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b490, 0x143a00290> as *const ()), // sub_142630fc0
    Replacement::draft(0x142631050, "unknown_142631050", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b3d8, 0x143a00290> as *const ()), // sub_142631050
    Replacement::draft(0x1426310e0, "unknown_1426310e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b3f8, 0x143a00290> as *const ()), // sub_1426310e0
    Replacement::draft(0x142631170, "unknown_142631170", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b728, 0x143a00290> as *const ()), // sub_142631170
    Replacement::draft(0x142631200, "unknown_142631200", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b380, 0x143a00290> as *const ()), // sub_142631200
    Replacement::draft(0x142631290, "unknown_142631290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b668, 0x143a00290> as *const ()), // sub_142631290
    Replacement::draft(0x142631320, "unknown_142631320", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b3b8, 0x143a00290> as *const ()), // sub_142631320
    Replacement::draft(0x1426313b0, "unknown_1426313b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b5c8, 0x143a00290> as *const ()), // sub_1426313b0
    Replacement::draft(0x142631440, "unknown_142631440", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b588, 0x143a00290> as *const ()), // sub_142631440
    Replacement::draft(0x1426314d0, "unknown_1426314d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b528, 0x143a00290> as *const ()), // sub_1426314d0
    Replacement::draft(0x142631560, "unknown_142631560", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b7e0, 0x143a00290> as *const ()), // sub_142631560
    Replacement::draft(0x1426315f0, "unknown_1426315f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b1b0, 0x143a00290> as *const ()), // sub_1426315f0
    Replacement::draft(0x142631680, "unknown_142631680", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b708, 0x143a00290> as *const ()), // sub_142631680
    Replacement::draft(0x142631710, "unknown_142631710", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b238, 0x143a00290> as *const ()), // sub_142631710
    Replacement::draft(0x1426317a0, "unknown_1426317a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b0e0, 0x143a00290> as *const ()), // sub_1426317a0
    Replacement::draft(0x142631830, "unknown_142631830", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b2b0, 0x143a00290> as *const ()), // sub_142631830
    Replacement::draft(0x1426318c0, "unknown_1426318c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b270, 0x143a00290> as *const ()), // sub_1426318c0
    Replacement::draft(0x142631950, "unknown_142631950", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b138, 0x143a00290> as *const ()), // sub_142631950
    Replacement::draft(0x1426319e0, "unknown_1426319e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b320, 0x143a00290> as *const ()), // sub_1426319e0
    Replacement::draft(0x142631a70, "unknown_142631a70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b6e8, 0x143a00290> as *const ()), // sub_142631a70
    Replacement::draft(0x142631b00, "unknown_142631b00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b250, 0x143a00290> as *const ()), // sub_142631b00
    Replacement::draft(0x142631b90, "unknown_142631b90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b688, 0x143a00290> as *const ()), // sub_142631b90
    Replacement::draft(0x142631c20, "unknown_142631c20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b5e8, 0x143a00290> as *const ()), // sub_142631c20
    Replacement::draft(0x142631cb0, "unknown_142631cb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b450, 0x143a00290> as *const ()), // sub_142631cb0
    Replacement::draft(0x142631d40, "unknown_142631d40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b740, 0x143a00290> as *const ()), // sub_142631d40
    Replacement::draft(0x142631dd0, "unknown_142631dd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b0c0, 0x143a00290> as *const ()), // sub_142631dd0
    Replacement::draft(0x142631e60, "unknown_142631e60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b780, 0x143a00290> as *const ()), // sub_142631e60
    Replacement::draft(0x142631ef0, "unknown_142631ef0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b7c0, 0x143a00290> as *const ()), // sub_142631ef0
    Replacement::draft(0x142631f80, "unknown_142631f80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b5a8, 0x143a00290> as *const ()), // sub_142631f80
    Replacement::draft(0x142632010, "unknown_142632010", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b470, 0x143a00290> as *const ()), // sub_142632010
    Replacement::draft(0x1426320a0, "unknown_1426320a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b608, 0x143a00290> as *const ()), // sub_1426320a0
    Replacement::draft(0x142632130, "unknown_142632130", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b548, 0x143a00290> as *const ()), // sub_142632130
    Replacement::draft(0x1426321c0, "unknown_1426321c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b1d0, 0x143a00290> as *const ()), // sub_1426321c0
    Replacement::draft(0x142632250, "unknown_142632250", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b430, 0x143a00290> as *const ()), // sub_142632250
    Replacement::draft(0x1426322e0, "unknown_1426322e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b508, 0x143a00290> as *const ()), // sub_1426322e0
    Replacement::draft(0x142632370, "unknown_142632370", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b648, 0x143a00290> as *const ()), // sub_142632370
    Replacement::draft(0x142632400, "unknown_142632400", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b158, 0x143a00290> as *const ()), // sub_142632400
    Replacement::draft(0x142632490, "unknown_142632490", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b4e8, 0x143a00290> as *const ()), // sub_142632490
    Replacement::draft(0x142632520, "unknown_142632520", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b4b0, 0x143a00290> as *const ()), // sub_142632520
    Replacement::draft(0x1426325b0, "unknown_1426325b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b6c8, 0x143a00290> as *const ()), // sub_1426325b0
    Replacement::draft(0x142632640, "unknown_142632640", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b398, 0x143a00290> as *const ()), // sub_142632640
    Replacement::draft(0x1426326d0, "unknown_1426326d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b7a0, 0x143a00290> as *const ()), // sub_1426326d0
    Replacement::draft(0x142632760, "unknown_142632760", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b760, 0x143a00290> as *const ()), // sub_142632760
    Replacement::draft(0x1426327f0, "unknown_1426327f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b120, 0x143a00290> as *const ()), // sub_1426327f0
    Replacement::draft(0x142632880, "unknown_142632880", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b1f0, 0x143a00290> as *const ()), // sub_142632880
    Replacement::draft(0x142632910, "unknown_142632910", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b198, 0x143a00290> as *const ()), // sub_142632910
    Replacement::draft(0x1426329a0, "unknown_1426329a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b308, 0x143a00290> as *const ()), // sub_1426329a0
    Replacement::draft(0x142632a30, "unknown_142632a30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b2f0, 0x143a00290> as *const ()), // sub_142632a30
    Replacement::draft(0x142632ac0, "unknown_142632ac0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b340, 0x143a00290> as *const ()), // sub_142632ac0
    Replacement::draft(0x142632b50, "unknown_142632b50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b4d0, 0x143a00290> as *const ()), // sub_142632b50
    Replacement::draft(0x1426330f0, "unknown_1426330f0", tpl_53272::<0x143bbd3d0, 0x14022a610, 0x1434077a8, 0x143bbd3c0, 0x143a001ac, 0x143a00290> as *const ()), // sub_1426330f0
    Replacement::draft(0x142633180, "unknown_142633180", tpl_53272::<0x143bbd410, 0x14022a610, 0x1434077a8, 0x143bbd400, 0x143a001ac, 0x143a00290> as *const ()), // sub_142633180
    Replacement::draft(0x142633210, "unknown_142633210", tpl_53272::<0x143bbd3a8, 0x14022a610, 0x1434077a8, 0x143bbd398, 0x143a001ac, 0x143a00290> as *const ()), // sub_142633210
    Replacement::draft(0x1426332a0, "unknown_1426332a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386b950, 0x143a00290> as *const ()), // sub_1426332a0
    Replacement::draft(0x142633390, "unknown_142633390", tpl_53395::<0x1418349c0, 0x143857768, 0xb18, 0xb20, 0x48, 0xa80> as *const ()), // sub_142633390
    Replacement::draft(0x1426333f0, "unknown_1426333f0", tpl_53395::<0x1418349c0, 0x14385586c, 0xb18, 0xb20, 0x48, 0xa80> as *const ()), // sub_1426333f0
    Replacement::draft(0x142633450, "unknown_142633450", tpl_53395::<0x1418349c0, 0x14385602c, 0xb18, 0xb20, 0x48, 0xa80> as *const ()), // sub_142633450
    Replacement::draft(0x1426334b0, "unknown_1426334b0", tpl_53395::<0x1418349c0, 0x143856028, 0xb18, 0xb20, 0x48, 0xa80> as *const ()), // sub_1426334b0
    Replacement::draft(0x142633670, "unknown_142633670", tpl_53396::<0x143a369f8, 0x14204db80, 0x142442de0> as *const ()), // sub_142633670
    Replacement::draft(0x1426336a0, "unknown_1426336a0", tpl_53396::<0x143a36b78, 0x14204db80, 0x142442de0> as *const ()), // sub_1426336a0
    Replacement::draft(0x1426336d0, "unknown_1426336d0", tpl_53397::<0x143a36a48, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0> as *const ()), // sub_1426336d0
    Replacement::draft(0x142633710, "unknown_142633710", tpl_53397::<0x143a36958, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0> as *const ()), // sub_142633710
    Replacement::draft(0x142633750, "unknown_142633750", tpl_53397::<0x143a36aa8, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0> as *const ()), // sub_142633750
    Replacement::draft(0x142633790, "unknown_142633790", tpl_53397::<0x143a36b28, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0> as *const ()), // sub_142633790
    Replacement::draft(0x1426337d0, "unknown_1426337d0", tpl_53397::<0x143a368d8, 0x14209d670, 0x58, 0x1411e7fc0, 0x142442de0> as *const ()), // sub_1426337d0
    Replacement::draft(0x142633850, "unknown_142633850", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bbe8, 0x143a00290> as *const ()), // sub_142633850
    Replacement::draft(0x1426338e0, "unknown_1426338e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bba8, 0x143a00290> as *const ()), // sub_1426338e0
    Replacement::draft(0x142633970, "unknown_142633970", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bbc8, 0x143a00290> as *const ()), // sub_142633970
    Replacement::draft(0x142633a00, "unknown_142633a00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bc08, 0x143a00290> as *const ()), // sub_142633a00
    Replacement::draft(0x142633a90, "unknown_142633a90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881b80, 0x143a00290> as *const ()), // sub_142633a90
    Replacement::draft(0x142633bc0, "unknown_142633bc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bc38, 0x143a00290> as *const ()), // sub_142633bc0
    Replacement::draft(0x142633c50, "unknown_142633c50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c058, 0x143a00290> as *const ()), // sub_142633c50
    Replacement::draft(0x142633ce0, "unknown_142633ce0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bcb0, 0x143a00290> as *const ()), // sub_142633ce0
    Replacement::draft(0x142633d70, "unknown_142633d70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bcd0, 0x143a00290> as *const ()), // sub_142633d70
    Replacement::draft(0x142633e00, "unknown_142633e00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bda0, 0x143a00290> as *const ()), // sub_142633e00
    Replacement::draft(0x142633e90, "unknown_142633e90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bdb8, 0x143a00290> as *const ()), // sub_142633e90
    Replacement::draft(0x142633f20, "unknown_142633f20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bf58, 0x143a00290> as *const ()), // sub_142633f20
    Replacement::draft(0x142633fb0, "unknown_142633fb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bed8, 0x143a00290> as *const ()), // sub_142633fb0
    Replacement::draft(0x142634040, "unknown_142634040", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bfa0, 0x143a00290> as *const ()), // sub_142634040
    Replacement::draft(0x1426340d0, "unknown_1426340d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c090, 0x143a00290> as *const ()), // sub_1426340d0
    Replacement::draft(0x142634160, "unknown_142634160", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386beb8, 0x143a00290> as *const ()), // sub_142634160
    Replacement::draft(0x1426341f0, "unknown_1426341f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bc78, 0x143a00290> as *const ()), // sub_1426341f0
    Replacement::draft(0x142634280, "unknown_142634280", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bde8, 0x143a00290> as *const ()), // sub_142634280
    Replacement::draft(0x142634310, "unknown_142634310", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bf18, 0x143a00290> as *const ()), // sub_142634310
    Replacement::draft(0x1426343a0, "unknown_1426343a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bd50, 0x143a00290> as *const ()), // sub_1426343a0
    Replacement::draft(0x142634430, "unknown_142634430", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386be48, 0x143a00290> as *const ()), // sub_142634430
    Replacement::draft(0x1426344c0, "unknown_1426344c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bdd0, 0x143a00290> as *const ()), // sub_1426344c0
    Replacement::draft(0x142634550, "unknown_142634550", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c008, 0x143a00290> as *const ()), // sub_142634550
    Replacement::draft(0x1426345e0, "unknown_1426345e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bce8, 0x143a00290> as *const ()), // sub_1426345e0
    Replacement::draft(0x142634670, "unknown_142634670", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bd18, 0x143a00290> as *const ()), // sub_142634670
    Replacement::draft(0x142634700, "unknown_142634700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bfc0, 0x143a00290> as *const ()), // sub_142634700
    Replacement::draft(0x142634790, "unknown_142634790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386be08, 0x143a00290> as *const ()), // sub_142634790
    Replacement::draft(0x142634820, "unknown_142634820", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c020, 0x143a00290> as *const ()), // sub_142634820
    Replacement::draft(0x1426348b0, "unknown_1426348b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bd00, 0x143a00290> as *const ()), // sub_1426348b0
    Replacement::draft(0x142634940, "unknown_142634940", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bf70, 0x143a00290> as *const ()), // sub_142634940
    Replacement::draft(0x1426349d0, "unknown_1426349d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bea0, 0x143a00290> as *const ()), // sub_1426349d0
    Replacement::draft(0x142634a60, "unknown_142634a60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c0b0, 0x143a00290> as *const ()), // sub_142634a60
    Replacement::draft(0x142634af0, "unknown_142634af0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bc98, 0x143a00290> as *const ()), // sub_142634af0
    Replacement::draft(0x142634b80, "unknown_142634b80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bf38, 0x143a00290> as *const ()), // sub_142634b80
    Replacement::draft(0x142634c10, "unknown_142634c10", tpl_53272::<0x143bbd4c0, 0x14022a610, 0x1434077a8, 0x143bbd4b0, 0x143a001ac, 0x143a00290> as *const ()), // sub_142634c10
    Replacement::draft(0x142634ca0, "unknown_142634ca0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bc58, 0x143a00290> as *const ()), // sub_142634ca0
    Replacement::draft(0x142634d30, "unknown_142634d30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386be80, 0x143a00290> as *const ()), // sub_142634d30
    Replacement::draft(0x142634dc0, "unknown_142634dc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c038, 0x143a00290> as *const ()), // sub_142634dc0
    Replacement::draft(0x142634e50, "unknown_142634e50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bf88, 0x143a00290> as *const ()), // sub_142634e50
    Replacement::draft(0x142634ee0, "unknown_142634ee0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bfd8, 0x143a00290> as *const ()), // sub_142634ee0
    Replacement::draft(0x142634f70, "unknown_142634f70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bd68, 0x143a00290> as *const ()), // sub_142634f70
    Replacement::draft(0x142635000, "unknown_142635000", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386be60, 0x143a00290> as *const ()), // sub_142635000
    Replacement::draft(0x142635090, "unknown_142635090", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bff0, 0x143a00290> as *const ()), // sub_142635090
    Replacement::draft(0x142635120, "unknown_142635120", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bd88, 0x143a00290> as *const ()), // sub_142635120
    Replacement::draft(0x1426351b0, "unknown_1426351b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bd30, 0x143a00290> as *const ()), // sub_1426351b0
    Replacement::draft(0x142635240, "unknown_142635240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c070, 0x143a00290> as *const ()), // sub_142635240
    Replacement::draft(0x1426352d0, "unknown_1426352d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386bef8, 0x143a00290> as *const ()), // sub_1426352d0
    Replacement::draft(0x1426358b0, "unknown_1426358b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c168, 0x143a00290> as *const ()), // sub_1426358b0
    Replacement::draft(0x142635940, "unknown_142635940", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c110, 0x143a00290> as *const ()), // sub_142635940
    Replacement::draft(0x1426359d0, "unknown_1426359d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c148, 0x143a00290> as *const ()), // sub_1426359d0
    Replacement::draft(0x142635a60, "unknown_142635a60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c128, 0x143a00290> as *const ()), // sub_142635a60
    Replacement::draft(0x142635cf0, "unknown_142635cf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c180, 0x143a00290> as *const ()), // sub_142635cf0
    Replacement::draft(0x142635d80, "unknown_142635d80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c198, 0x143a00290> as *const ()), // sub_142635d80
    Replacement::draft(0x142635e10, "unknown_142635e10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c1e0, 0x143a00290> as *const ()), // sub_142635e10
    Replacement::draft(0x142635ea0, "unknown_142635ea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c1b0, 0x143a00290> as *const ()), // sub_142635ea0
    Replacement::draft(0x142635f30, "unknown_142635f30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c1f8, 0x143a00290> as *const ()), // sub_142635f30
    Replacement::draft(0x142635fc0, "unknown_142635fc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c1c8, 0x143a00290> as *const ()), // sub_142635fc0
    Replacement::draft(0x142636060, "unknown_142636060", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c230, 0x143a00290> as *const ()), // sub_142636060
    Replacement::draft(0x142636120, "unknown_142636120", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c338, 0x143a00290> as *const ()), // sub_142636120
    Replacement::draft(0x1426361b0, "unknown_1426361b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c358, 0x143a00290> as *const ()), // sub_1426361b0
    Replacement::draft(0x142636240, "unknown_142636240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c320, 0x143a00290> as *const ()), // sub_142636240
    Replacement::draft(0x1426362d0, "unknown_1426362d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c2e0, 0x143a00290> as *const ()), // sub_1426362d0
    Replacement::draft(0x142636360, "unknown_142636360", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c300, 0x143a00290> as *const ()), // sub_142636360
    Replacement::draft(0x1426363f0, "unknown_1426363f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c370, 0x143a00290> as *const ()), // sub_1426363f0
    Replacement::draft(0x1426364b0, "unknown_1426364b0", tpl_53398::<0x143bbd698, 0x141327e10, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_1426364b0
    Replacement::draft(0x142636670, "unknown_142636670", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c4c8, 0x143a00290> as *const ()), // sub_142636670
    Replacement::draft(0x142636700, "unknown_142636700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c550, 0x143a00290> as *const ()), // sub_142636700
    Replacement::draft(0x142636790, "unknown_142636790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c4b0, 0x143a00290> as *const ()), // sub_142636790
    Replacement::draft(0x142636820, "unknown_142636820", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c468, 0x143a00290> as *const ()), // sub_142636820
    Replacement::draft(0x1426368b0, "unknown_1426368b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c480, 0x143a00290> as *const ()), // sub_1426368b0
    Replacement::draft(0x142636940, "unknown_142636940", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c498, 0x143a00290> as *const ()), // sub_142636940
    Replacement::draft(0x1426369d0, "unknown_1426369d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c588, 0x143a00290> as *const ()), // sub_1426369d0
    Replacement::draft(0x142636a60, "unknown_142636a60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c568, 0x143a00290> as *const ()), // sub_142636a60
    Replacement::draft(0x142636af0, "unknown_142636af0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c400, 0x143a00290> as *const ()), // sub_142636af0
    Replacement::draft(0x142636b80, "unknown_142636b80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c3e0, 0x143a00290> as *const ()), // sub_142636b80
    Replacement::draft(0x142636c10, "unknown_142636c10", tpl_53272::<0x143bbd798, 0x14022a610, 0x1434077a8, 0x143bbd788, 0x143a001ac, 0x143a00290> as *const ()), // sub_142636c10
    Replacement::draft(0x142636ca0, "unknown_142636ca0", tpl_53272::<0x143bbd7e8, 0x14022a610, 0x1434077a8, 0x143bbd7d8, 0x143a001ac, 0x143a00290> as *const ()), // sub_142636ca0
    Replacement::draft(0x142636d30, "unknown_142636d30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c420, 0x143a00290> as *const ()), // sub_142636d30
    Replacement::draft(0x142636dc0, "unknown_142636dc0", tpl_53272::<0x143bbd7c0, 0x14022a610, 0x1434077a8, 0x143bbd7b0, 0x143a001ac, 0x143a00290> as *const ()), // sub_142636dc0
    Replacement::draft(0x142636e70, "unknown_142636e70", tpl_53272::<0x143bbd810, 0x14022a610, 0x1434077a8, 0x143bbd800, 0x143a001ac, 0x143a00290> as *const ()), // sub_142636e70
    Replacement::draft(0x142636f00, "unknown_142636f00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c520, 0x143a00290> as *const ()), // sub_142636f00
    Replacement::draft(0x142636f90, "unknown_142636f90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c538, 0x143a00290> as *const ()), // sub_142636f90
    Replacement::draft(0x142637860, "unknown_142637860", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c5f8, 0x143a00290> as *const ()), // sub_142637860
    Replacement::draft(0x142637990, "unknown_142637990", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c778, 0x143a00290> as *const ()), // sub_142637990
    Replacement::draft(0x142637a20, "unknown_142637a20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c760, 0x143a00290> as *const ()), // sub_142637a20
    Replacement::draft(0x142637ab0, "unknown_142637ab0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c748, 0x143a00290> as *const ()), // sub_142637ab0
    Replacement::draft(0x142637b40, "unknown_142637b40", tpl_53272::<0x143bbd970, 0x14022a610, 0x1434077a8, 0x143bbd960, 0x143a001ac, 0x143a00290> as *const ()), // sub_142637b40
    Replacement::draft(0x142637bd0, "unknown_142637bd0", tpl_53272::<0x143bbd948, 0x14022a610, 0x1434077a8, 0x143bbd938, 0x143a001ac, 0x143a00290> as *const ()), // sub_142637bd0
    Replacement::draft(0x142637c60, "unknown_142637c60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881c78, 0x143a00290> as *const ()), // sub_142637c60
    Replacement::draft(0x142637cf0, "unknown_142637cf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881c58, 0x143a00290> as *const ()), // sub_142637cf0
    Replacement::draft(0x142637e70, "unknown_142637e70", tpl_53398::<0x143bbda10, 0x14021cb50, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_142637e70
    Replacement::draft(0x142638290, "unknown_142638290", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c8c0, 0x143a00290> as *const ()), // sub_142638290
    Replacement::draft(0x142638320, "unknown_142638320", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c848, 0x143a00290> as *const ()), // sub_142638320
    Replacement::draft(0x1426383b0, "unknown_1426383b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c888, 0x143a00290> as *const ()), // sub_1426383b0
    Replacement::draft(0x142638440, "unknown_142638440", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c8d8, 0x143a00290> as *const ()), // sub_142638440
    Replacement::draft(0x1426384d0, "unknown_1426384d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c868, 0x143a00290> as *const ()), // sub_1426384d0
    Replacement::draft(0x142638560, "unknown_142638560", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386c8a8, 0x143a00290> as *const ()), // sub_142638560
    Replacement::draft(0x142638740, "unknown_142638740", tpl_53272::<0x143bbdd88, 0x14022a610, 0x1434077a8, 0x143bbdd78, 0x143a001ac, 0x143a00290> as *const ()), // sub_142638740
    Replacement::draft(0x1426387e0, "unknown_1426387e0", tpl_53396::<0x143a37318, 0x14204db80, 0x142442de0> as *const ()), // sub_1426387e0
    Replacement::draft(0x142638810, "unknown_142638810", tpl_53399::<0x143a37718, 0x8, 0x14209a1e0, 0x142442de0> as *const ()), // sub_142638810
    Replacement::draft(0x142638850, "unknown_142638850", tpl_53399::<0x143a37528, 0x8, 0x14209a1e0, 0x142442de0> as *const ()), // sub_142638850
    Replacement::draft(0x142638890, "unknown_142638890", tpl_53399::<0x143a373a8, 0x8, 0x14209a1e0, 0x142442de0> as *const ()), // sub_142638890
    Replacement::draft(0x1426388d0, "unknown_1426388d0", tpl_53399::<0x143a373f8, 0x8, 0x14209a1e0, 0x142442de0> as *const ()), // sub_1426388d0
    Replacement::draft(0x142638aa0, "unknown_142638aa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881d30, 0x143a00290> as *const ()), // sub_142638aa0
    Replacement::draft(0x142638b30, "unknown_142638b30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881d60, 0x143a00290> as *const ()), // sub_142638b30
    Replacement::draft(0x142638bc0, "unknown_142638bc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881d48, 0x143a00290> as *const ()), // sub_142638bc0
    Replacement::draft(0x142638c50, "unknown_142638c50", tpl_53272::<0x143cfa680, 0x14022a610, 0x1434077a8, 0x143cfa670, 0x143a001ac, 0x143a00290> as *const ()), // sub_142638c50
    Replacement::draft(0x142638e80, "unknown_142638e80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cfa7f8, 0x143a00290> as *const ()), // sub_142638e80
    Replacement::draft(0x142638f10, "unknown_142638f10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cfa120, 0x143a00290> as *const ()), // sub_142638f10
    Replacement::draft(0x142638fa0, "unknown_142638fa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cfb138, 0x143a00290> as *const ()), // sub_142638fa0
    Replacement::draft(0x142639530, "unknown_142639530", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386cb30, 0x143a00290> as *const ()), // sub_142639530
    Replacement::draft(0x1426395c0, "unknown_1426395c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386cb18, 0x143a00290> as *const ()), // sub_1426395c0
    Replacement::draft(0x142639c10, "unknown_142639c10", fn_142639c10 as *const ()), // sub_142639c10
    Replacement::draft_throws(0x142639c30, "unknown_142639c30", fn_142639c30 as *const (), Throws::Caught), // sub_142639c30
    Replacement::draft(0x142639d80, "unknown_142639d80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e1f0, 0x143a00290> as *const ()), // sub_142639d80
    Replacement::draft(0x142639e10, "unknown_142639e10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e1d0, 0x143a00290> as *const ()), // sub_142639e10
    Replacement::draft(0x142639ea0, "unknown_142639ea0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e210, 0x143a00290> as *const ()), // sub_142639ea0
    Replacement::draft(0x142639f30, "unknown_142639f30", tpl_53272::<0x143bbddf8, 0x14022a610, 0x1434077a8, 0x143bbdde8, 0x143a001ac, 0x143a00290> as *const ()), // sub_142639f30
    Replacement::draft(0x14263a190, "unknown_14263a190", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e4a0, 0x143a00290> as *const ()), // sub_14263a190
    Replacement::draft(0x14263a220, "unknown_14263a220", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e4b8, 0x143a00290> as *const ()), // sub_14263a220
    Replacement::draft(0x14263a2b0, "unknown_14263a2b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e488, 0x143a00290> as *const ()), // sub_14263a2b0
    Replacement::draft_throws(0x14263a480, "unknown_14263a480", fn_14263a480 as *const (), Throws::Caught), // sub_14263a480
    Replacement::draft_throws(0x14263a730, "unknown_14263a730", fn_14263a730 as *const (), Throws::Caught), // sub_14263a730
    Replacement::draft_throws(0x14263a760, "unknown_14263a760", fn_14263a760 as *const (), Throws::Caught), // sub_14263a760
    Replacement::draft_throws(0x14263a790, "unknown_14263a790", fn_14263a790 as *const (), Throws::Caught), // sub_14263a790
    Replacement::draft_throws(0x14263a7c0, "unknown_14263a7c0", fn_14263a7c0 as *const (), Throws::Caught), // sub_14263a7c0
    Replacement::draft_throws(0x14263a7f0, "unknown_14263a7f0", fn_14263a7f0 as *const (), Throws::Caught), // sub_14263a7f0
    Replacement::draft_throws(0x14263a820, "unknown_14263a820", fn_14263a820 as *const (), Throws::Caught), // sub_14263a820
    Replacement::draft(0x14263ac40, "unknown_14263ac40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386efb0, 0x143a00290> as *const ()), // sub_14263ac40
    Replacement::draft(0x14263acd0, "unknown_14263acd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386edd0, 0x143a00290> as *const ()), // sub_14263acd0
    Replacement::draft(0x14263ad60, "unknown_14263ad60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e720, 0x143a00290> as *const ()), // sub_14263ad60
    Replacement::draft(0x14263adf0, "unknown_14263adf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386ef80, 0x143a00290> as *const ()), // sub_14263adf0
    Replacement::draft(0x14263ae80, "unknown_14263ae80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e7f0, 0x143a00290> as *const ()), // sub_14263ae80
    Replacement::draft(0x14263af10, "unknown_14263af10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386ef98, 0x143a00290> as *const ()), // sub_14263af10
    Replacement::draft(0x14263afa0, "unknown_14263afa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386f0a0, 0x143a00290> as *const ()), // sub_14263afa0
    Replacement::draft(0x14263b030, "unknown_14263b030", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e8e0, 0x143a00290> as *const ()), // sub_14263b030
    Replacement::draft(0x14263b0c0, "unknown_14263b0c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e588, 0x143a00290> as *const ()), // sub_14263b0c0
    Replacement::draft(0x14263b150, "unknown_14263b150", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386ee08, 0x143a00290> as *const ()), // sub_14263b150
    Replacement::draft(0x14263b1e0, "unknown_14263b1e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e548, 0x143a00290> as *const ()), // sub_14263b1e0
    Replacement::draft(0x14263b270, "unknown_14263b270", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386f080, 0x143a00290> as *const ()), // sub_14263b270
    Replacement::draft(0x14263b300, "unknown_14263b300", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e700, 0x143a00290> as *const ()), // sub_14263b300
    Replacement::draft(0x14263b390, "unknown_14263b390", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e568, 0x143a00290> as *const ()), // sub_14263b390
    Replacement::draft(0x14263b420, "unknown_14263b420", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386ede8, 0x143a00290> as *const ()), // sub_14263b420
    Replacement::draft(0x14263b4b0, "unknown_14263b4b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e808, 0x143a00290> as *const ()), // sub_14263b4b0
    Replacement::draft(0x14263b820, "unknown_14263b820", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386e518, 0x143a00290> as *const ()), // sub_14263b820
    Replacement::draft(0x14263ba00, "unknown_14263ba00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cfdc10, 0x143a00290> as *const ()), // sub_14263ba00
    Replacement::draft(0x14263baf0, "unknown_14263baf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386f808, 0x143a00290> as *const ()), // sub_14263baf0
    Replacement::draft(0x14263bb80, "unknown_14263bb80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386f878, 0x143a00290> as *const ()), // sub_14263bb80
    Replacement::draft(0x14263bc10, "unknown_14263bc10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fa50, 0x143a00290> as *const ()), // sub_14263bc10
    Replacement::draft(0x14263bca0, "unknown_14263bca0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386f840, 0x143a00290> as *const ()), // sub_14263bca0
    Replacement::draft(0x14263bd30, "unknown_14263bd30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fa30, 0x143a00290> as *const ()), // sub_14263bd30
    Replacement::draft(0x14263bdc0, "unknown_14263bdc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386f950, 0x143a00290> as *const ()), // sub_14263bdc0
    Replacement::draft(0x14263be50, "unknown_14263be50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386f820, 0x143a00290> as *const ()), // sub_14263be50
    Replacement::draft(0x14263bee0, "unknown_14263bee0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386f858, 0x143a00290> as *const ()), // sub_14263bee0
    Replacement::draft(0x14263c0b0, "unknown_14263c0b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fab0, 0x143a00290> as *const ()), // sub_14263c0b0
    Replacement::draft(0x14263c140, "unknown_14263c140", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fae0, 0x143a00290> as *const ()), // sub_14263c140
    Replacement::draft(0x14263c1d0, "unknown_14263c1d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fa98, 0x143a00290> as *const ()), // sub_14263c1d0
    Replacement::draft(0x14263c260, "unknown_14263c260", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fa80, 0x143a00290> as *const ()), // sub_14263c260
    Replacement::draft(0x14263c2f0, "unknown_14263c2f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fa68, 0x143a00290> as *const ()), // sub_14263c2f0
    Replacement::draft(0x14263c380, "unknown_14263c380", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fac8, 0x143a00290> as *const ()), // sub_14263c380
    Replacement::draft(0x14263c410, "unknown_14263c410", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881e40, 0x143a00290> as *const ()), // sub_14263c410
    Replacement::draft(0x14263c730, "unknown_14263c730", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881e00, 0x143a00290> as *const ()), // sub_14263c730
    Replacement::draft(0x14263c7c0, "unknown_14263c7c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881e20, 0x143a00290> as *const ()), // sub_14263c7c0
    Replacement::draft(0x14263c850, "unknown_14263c850", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881e78, 0x143a00290> as *const ()), // sub_14263c850
    Replacement::draft(0x14263c8f0, "unknown_14263c8f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881e58, 0x143a00290> as *const ()), // sub_14263c8f0
    Replacement::draft(0x14263c980, "unknown_14263c980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881de0, 0x143a00290> as *const ()), // sub_14263c980
    Replacement::draft(0x14263ca10, "unknown_14263ca10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143881dc8, 0x143a00290> as *const ()), // sub_14263ca10
    Replacement::draft(0x14263ce20, "unknown_14263ce20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fb10, 0x143a00290> as *const ()), // sub_14263ce20
    Replacement::draft(0x14263ceb0, "unknown_14263ceb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386faf8, 0x143a00290> as *const ()), // sub_14263ceb0
    Replacement::draft(0x14263cf40, "unknown_14263cf40", tpl_53272::<0x143bbdf90, 0x14022a610, 0x1434077a8, 0x143bbdf80, 0x143a001ac, 0x143a00290> as *const ()), // sub_14263cf40
    Replacement::draft(0x14263d310, "unknown_14263d310", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870330, 0x143a00290> as *const ()), // sub_14263d310
    Replacement::draft(0x14263d3a0, "unknown_14263d3a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386ff10, 0x143a00290> as *const ()), // sub_14263d3a0
    Replacement::draft(0x14263d430, "unknown_14263d430", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870050, 0x143a00290> as *const ()), // sub_14263d430
    Replacement::draft(0x14263d4c0, "unknown_14263d4c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x14386fff0, 0x143a00290> as *const ()), // sub_14263d4c0
    Replacement::draft(0x14263d550, "unknown_14263d550", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870030, 0x143a00290> as *const ()), // sub_14263d550
    Replacement::draft(0x14263d5e0, "unknown_14263d5e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870878, 0x143a00290> as *const ()), // sub_14263d5e0
    Replacement::draft(0x14263d670, "unknown_14263d670", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870348, 0x143a00290> as *const ()), // sub_14263d670
    Replacement::draft(0x14263d700, "unknown_14263d700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870838, 0x143a00290> as *const ()), // sub_14263d700
    Replacement::draft(0x14263d790, "unknown_14263d790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870010, 0x143a00290> as *const ()), // sub_14263d790
    Replacement::draft(0x14263d820, "unknown_14263d820", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870560, 0x143a00290> as *const ()), // sub_14263d820
    Replacement::draft(0x14263d8b0, "unknown_14263d8b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870150, 0x143a00290> as *const ()), // sub_14263d8b0
    Replacement::draft(0x14263d940, "unknown_14263d940", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870070, 0x143a00290> as *const ()), // sub_14263d940
    Replacement::draft(0x14263d9d0, "unknown_14263d9d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870540, 0x143a00290> as *const ()), // sub_14263d9d0
    Replacement::draft(0x14263da60, "unknown_14263da60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870740, 0x143a00290> as *const ()), // sub_14263da60
    Replacement::draft(0x14263daf0, "unknown_14263daf0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870720, 0x143a00290> as *const ()), // sub_14263daf0
    Replacement::draft(0x14263db80, "unknown_14263db80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870168, 0x143a00290> as *const ()), // sub_14263db80
    Replacement::draft(0x14263dc10, "unknown_14263dc10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870578, 0x143a00290> as *const ()), // sub_14263dc10
    Replacement::draft(0x14263dca0, "unknown_14263dca0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870388, 0x143a00290> as *const ()), // sub_14263dca0
    Replacement::draft(0x14263dd30, "unknown_14263dd30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870310, 0x143a00290> as *const ()), // sub_14263dd30
    Replacement::draft(0x14263ddc0, "unknown_14263ddc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438703a8, 0x143a00290> as *const ()), // sub_14263ddc0
    Replacement::draft(0x14263de50, "unknown_14263de50", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870858, 0x143a00290> as *const ()), // sub_14263de50
    Replacement::draft(0x14263dee0, "unknown_14263dee0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870368, 0x143a00290> as *const ()), // sub_14263dee0
    Replacement::draft(0x14263e010, "unknown_14263e010", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143cfff70, 0x143a00290> as *const ()), // sub_14263e010
    Replacement::draft(0x14263e240, "unknown_14263e240", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870d40, 0x143a00290> as *const ()), // sub_14263e240
    Replacement::draft(0x14263e2d0, "unknown_14263e2d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870da0, 0x143a00290> as *const ()), // sub_14263e2d0
    Replacement::draft(0x14263e360, "unknown_14263e360", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870e80, 0x143a00290> as *const ()), // sub_14263e360
    Replacement::draft(0x14263e3f0, "unknown_14263e3f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870d88, 0x143a00290> as *const ()), // sub_14263e3f0
    Replacement::draft(0x14263e480, "unknown_14263e480", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870d70, 0x143a00290> as *const ()), // sub_14263e480
    Replacement::draft(0x14263e510, "unknown_14263e510", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143870ba0, 0x143a00290> as *const ()), // sub_14263e510
    Replacement::draft(0x14263e8b0, "unknown_14263e8b0", tpl_53403::<0x143bfe110, 0x1403496f0, 0x1401e1800, 0x143bf6108, 0x141834ee8> as *const ()), // sub_14263e8b0
    Replacement::draft(0x14263e8f0, "unknown_14263e8f0", tpl_53403::<0x143bc5fc0, 0x1403496f0, 0x1401e1800, 0x143bbdfb8, 0x141834ee8> as *const ()), // sub_14263e8f0
    Replacement::draft(0x14263e930, "unknown_14263e930", tpl_53403::<0x143be6080, 0x1403496f0, 0x1401e1800, 0x143bde078, 0x141834ee8> as *const ()), // sub_14263e930
    Replacement::draft(0x14263e970, "unknown_14263e970", tpl_53403::<0x143bd6020, 0x1403496f0, 0x1401e1800, 0x143bce018, 0x141834ee8> as *const ()), // sub_14263e970
    Replacement::draft(0x14263e9b0, "unknown_14263e9b0", tpl_53403::<0x143bf60e0, 0x1403496f0, 0x1401e1800, 0x143bee0d8, 0x141834ee8> as *const ()), // sub_14263e9b0
    Replacement::draft(0x14263e9f0, "unknown_14263e9f0", tpl_53403::<0x143c161a0, 0x1403496f0, 0x1401e1800, 0x143c0e198, 0x141834ee8> as *const ()), // sub_14263e9f0
    Replacement::draft(0x14263ea30, "unknown_14263ea30", tpl_53403::<0x143bee0b0, 0x1403496f0, 0x1401e1800, 0x143be60a8, 0x141834ee8> as *const ()), // sub_14263ea30
    Replacement::draft(0x14263ea70, "unknown_14263ea70", tpl_53403::<0x143c0e170, 0x1403496f0, 0x1401e1800, 0x143c06168, 0x141834ee8> as *const ()), // sub_14263ea70
    Replacement::draft(0x14263eab0, "unknown_14263eab0", tpl_53403::<0x143bcdff0, 0x1403496f0, 0x1401e1800, 0x143bc5fe8, 0x141834ee8> as *const ()), // sub_14263eab0
    Replacement::draft(0x14263eaf0, "unknown_14263eaf0", tpl_53403::<0x143bde050, 0x1403496f0, 0x1401e1800, 0x143bd6048, 0x141834ee8> as *const ()), // sub_14263eaf0
    Replacement::draft(0x14263eb30, "unknown_14263eb30", tpl_53403::<0x143c06140, 0x1403496f0, 0x1401e1800, 0x143bfe138, 0x141834ee8> as *const ()), // sub_14263eb30
    Replacement::draft(0x14263eb70, "unknown_14263eb70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871248, 0x143a00290> as *const ()), // sub_14263eb70
    Replacement::draft(0x14263ec00, "unknown_14263ec00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438710e0, 0x143a00290> as *const ()), // sub_14263ec00
    Replacement::draft(0x14263ec90, "unknown_14263ec90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438712e8, 0x143a00290> as *const ()), // sub_14263ec90
    Replacement::draft(0x14263ed20, "unknown_14263ed20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871288, 0x143a00290> as *const ()), // sub_14263ed20
    Replacement::draft(0x14263edb0, "unknown_14263edb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438711c0, 0x143a00290> as *const ()), // sub_14263edb0
    Replacement::draft(0x14263ee40, "unknown_14263ee40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871230, 0x143a00290> as *const ()), // sub_14263ee40
    Replacement::draft(0x14263eed0, "unknown_14263eed0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438712a8, 0x143a00290> as *const ()), // sub_14263eed0
    Replacement::draft(0x14263ef60, "unknown_14263ef60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438711d8, 0x143a00290> as *const ()), // sub_14263ef60
    Replacement::draft(0x14263f010, "unknown_14263f010", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871210, 0x143a00290> as *const ()), // sub_14263f010
    Replacement::draft(0x14263f0a0, "unknown_14263f0a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871338, 0x143a00290> as *const ()), // sub_14263f0a0
    Replacement::draft(0x14263f130, "unknown_14263f130", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871268, 0x143a00290> as *const ()), // sub_14263f130
    Replacement::draft(0x14263f1c0, "unknown_14263f1c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438711f0, 0x143a00290> as *const ()), // sub_14263f1c0
    Replacement::draft(0x14263f250, "unknown_14263f250", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438712c8, 0x143a00290> as *const ()), // sub_14263f250
    Replacement::draft(0x14263f2e0, "unknown_14263f2e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871308, 0x143a00290> as *const ()), // sub_14263f2e0
    Replacement::draft(0x14263f620, "unknown_14263f620", fn_14263f620 as *const ()), // sub_14263f620
    Replacement::draft(0x14263f680, "unknown_14263f680", fn_14263f680 as *const ()), // sub_14263f680
    Replacement::draft(0x14263f6d0, "unknown_14263f6d0", fn_14263f6d0 as *const ()), // sub_14263f6d0
    Replacement::draft(0x14263f7a0, "unknown_14263f7a0", fn_14263f7a0 as *const ()), // sub_14263f7a0
    Replacement::draft(0x14263f7d0, "unknown_14263f7d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871350, 0x143a00290> as *const ()), // sub_14263f7d0
    Replacement::draft(0x14263f860, "unknown_14263f860", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871368, 0x143a00290> as *const ()), // sub_14263f860
    Replacement::draft(0x14263f8f0, "unknown_14263f8f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871388, 0x143a00290> as *const ()), // sub_14263f8f0
    Replacement::draft(0x14263f980, "unknown_14263f980", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438713a8, 0x143a00290> as *const ()), // sub_14263f980
    Replacement::draft(0x14263fa10, "unknown_14263fa10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438713c8, 0x143a00290> as *const ()), // sub_14263fa10
    Replacement::draft(0x14263faa0, "unknown_14263faa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438713e8, 0x143a00290> as *const ()), // sub_14263faa0
    Replacement::draft(0x14263fb30, "unknown_14263fb30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871408, 0x143a00290> as *const ()), // sub_14263fb30
    Replacement::draft(0x14263fbc0, "unknown_14263fbc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871428, 0x143a00290> as *const ()), // sub_14263fbc0
    Replacement::draft(0x14263fd70, "unknown_14263fd70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871590, 0x143a00290> as *const ()), // sub_14263fd70
    Replacement::draft(0x14263fe00, "unknown_14263fe00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871578, 0x143a00290> as *const ()), // sub_14263fe00
    Replacement::draft(0x14263fe90, "unknown_14263fe90", fn_14263fe90 as *const ()), // sub_14263fe90
    Replacement::draft(0x14263feb0, "unknown_14263feb0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438715b0, 0x143a00290> as *const ()), // sub_14263feb0
    Replacement::draft(0x14263ff40, "unknown_14263ff40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871610, 0x143a00290> as *const ()), // sub_14263ff40
    Replacement::draft(0x14263ffd0, "unknown_14263ffd0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871628, 0x143a00290> as *const ()), // sub_14263ffd0
    Replacement::draft(0x142640250, "unknown_142640250", tpl_53398::<0x143c168a8, 0x14021cb50, 0x14240ab10, 0x1425371a0, 0x142533200> as *const ()), // sub_142640250
    Replacement::draft(0x1426403c0, "unknown_1426403c0", tpl_53273::<0x143c168b8, 0x140246bb0, 0x1425371a0, 0x142533200> as *const ()), // sub_1426403c0
    Replacement::draft(0x142640530, "unknown_142640530", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871658, 0x143a00290> as *const ()), // sub_142640530
    Replacement::draft(0x1426405c0, "unknown_1426405c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871688, 0x143a00290> as *const ()), // sub_1426405c0
    Replacement::draft(0x142640650, "unknown_142640650", tpl_53266::<0x143c16908, 0x143c168d0, 0x20> as *const ()), // sub_142640650
    Replacement::draft(0x142640b50, "unknown_142640b50", fn_142640b50 as *const ()), // sub_142640b50
    Replacement::draft(0x142640bd0, "unknown_142640bd0", fn_142640bd0 as *const ()), // sub_142640bd0
    Replacement::draft(0x142640c10, "unknown_142640c10", fn_142640c10 as *const ()), // sub_142640c10
    Replacement::draft(0x142640ca0, "unknown_142640ca0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438717a0, 0x143a00290> as *const ()), // sub_142640ca0
    Replacement::draft(0x142640d30, "unknown_142640d30", tpl_53377::<0x143c47948, 0x140246bb0, 0x1425371a0, 0x142533200> as *const ()), // sub_142640d30
    Replacement::draft(0x142640e80, "unknown_142640e80", fn_142640e80 as *const ()), // sub_142640e80
    Replacement::draft(0x142640ef0, "unknown_142640ef0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438717d0, 0x143a00290> as *const ()), // sub_142640ef0
    Replacement::draft(0x142640f80, "unknown_142640f80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438717b8, 0x143a00290> as *const ()), // sub_142640f80
    Replacement::draft(0x142641010, "unknown_142641010", fn_142641010 as *const ()), // sub_142641010
    Replacement::draft(0x142641080, "ak_142641080", tpl_53414::<0x143c47968, 0x143412ca0, 0x143c47958, 0x143c47960, 0x1413db040, 0x1413dbe20, 0x8, 0x1413dc8d0, 0x141834d30> as *const ()), // sub_142641080
    Replacement::draft(0x142641120, "ak_142641120", tpl_53414::<0x1438820e0, 0x143412ca0, 0x1438820d0, 0x1438820d8, 0x1413db040, 0x1413dbe20, 0x8, 0x1413dc8d0, 0x141834d30> as *const ()), // sub_142641120
    Replacement::draft(0x1426411c0, "ak_1426411c0", fn_1426411c0 as *const ()), // sub_1426411c0
    Replacement::draft(0x1426411e0, "unknown_1426411e0", tpl_369 as *const ()), // sub_1426411e0
    Replacement::draft(0x1426411f0, "unknown_1426411f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438717e8, 0x143a00290> as *const ()), // sub_1426411f0
    Replacement::draft(0x142641280, "unknown_142641280", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871820, 0x143a00290> as *const ()), // sub_142641280
    Replacement::draft(0x142641310, "unknown_142641310", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871808, 0x143a00290> as *const ()), // sub_142641310
    Replacement::draft(0x1426413a0, "unknown_1426413a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438718b8, 0x143a00290> as *const ()), // sub_1426413a0
    Replacement::draft(0x142641430, "unknown_142641430", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871858, 0x143a00290> as *const ()), // sub_142641430
    Replacement::draft(0x1426414c0, "unknown_1426414c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871878, 0x143a00290> as *const ()), // sub_1426414c0
    Replacement::draft(0x142641550, "unknown_142641550", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871898, 0x143a00290> as *const ()), // sub_142641550
    Replacement::draft(0x1426415e0, "unknown_1426415e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871838, 0x143a00290> as *const ()), // sub_1426415e0
    Replacement::draft(0x142641670, "unknown_142641670", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871960, 0x143a00290> as *const ()), // sub_142641670
    Replacement::draft(0x142641700, "unknown_142641700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871990, 0x143a00290> as *const ()), // sub_142641700
    Replacement::draft(0x142641790, "unknown_142641790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871978, 0x143a00290> as *const ()), // sub_142641790
    Replacement::draft(0x142641820, "unknown_142641820", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871928, 0x143a00290> as *const ()), // sub_142641820
    Replacement::draft(0x1426418b0, "unknown_1426418b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438718d0, 0x143a00290> as *const ()), // sub_1426418b0
    Replacement::draft(0x142641940, "unknown_142641940", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871940, 0x143a00290> as *const ()), // sub_142641940
    Replacement::draft(0x1426419d0, "unknown_1426419d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871908, 0x143a00290> as *const ()), // sub_1426419d0
    Replacement::draft(0x142641a60, "unknown_142641a60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438718e8, 0x143a00290> as *const ()), // sub_142641a60
    Replacement::draft(0x142641b70, "unknown_142641b70", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871ac8, 0x143a00290> as *const ()), // sub_142641b70
    Replacement::draft(0x142641c00, "unknown_142641c00", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871b18, 0x143a00290> as *const ()), // sub_142641c00
    Replacement::draft(0x142641c90, "unknown_142641c90", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438719e0, 0x143a00290> as *const ()), // sub_142641c90
    Replacement::draft(0x142641d20, "unknown_142641d20", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871ab0, 0x143a00290> as *const ()), // sub_142641d20
    Replacement::draft(0x142641db0, "unknown_142641db0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871b00, 0x143a00290> as *const ()), // sub_142641db0
    Replacement::draft(0x142641e40, "unknown_142641e40", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871a68, 0x143a00290> as *const ()), // sub_142641e40
    Replacement::draft(0x142641ed0, "unknown_142641ed0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871ae8, 0x143a00290> as *const ()), // sub_142641ed0
    Replacement::draft(0x142641f60, "unknown_142641f60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871a20, 0x143a00290> as *const ()), // sub_142641f60
    Replacement::draft(0x142641ff0, "unknown_142641ff0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438719c8, 0x143a00290> as *const ()), // sub_142641ff0
    Replacement::draft(0x142642080, "unknown_142642080", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871a00, 0x143a00290> as *const ()), // sub_142642080
    Replacement::draft(0x142642110, "unknown_142642110", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871b58, 0x143a00290> as *const ()), // sub_142642110
    Replacement::draft(0x1426421a0, "unknown_1426421a0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871a98, 0x143a00290> as *const ()), // sub_1426421a0
    Replacement::draft(0x142642230, "unknown_142642230", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871a50, 0x143a00290> as *const ()), // sub_142642230
    Replacement::draft(0x1426422c0, "unknown_1426422c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871a38, 0x143a00290> as *const ()), // sub_1426422c0
    Replacement::draft(0x142642350, "unknown_142642350", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871a80, 0x143a00290> as *const ()), // sub_142642350
    Replacement::draft(0x1426423e0, "unknown_1426423e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871b38, 0x143a00290> as *const ()), // sub_1426423e0
    Replacement::draft(0x142642470, "unknown_142642470", fn_142642470 as *const ()), // sub_142642470
    Replacement::draft(0x1426424c0, "unknown_1426424c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871b78, 0x143a00290> as *const ()), // sub_1426424c0
    Replacement::draft(0x142642550, "unknown_142642550", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871b90, 0x143a00290> as *const ()), // sub_142642550
    Replacement::draft(0x1426425e0, "unknown_1426425e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871c10, 0x143a00290> as *const ()), // sub_1426425e0
    Replacement::draft(0x142642670, "unknown_142642670", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871bb0, 0x143a00290> as *const ()), // sub_142642670
    Replacement::draft(0x142642700, "unknown_142642700", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871c28, 0x143a00290> as *const ()), // sub_142642700
    Replacement::draft(0x142642790, "unknown_142642790", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871bf0, 0x143a00290> as *const ()), // sub_142642790
    Replacement::draft(0x142642820, "unknown_142642820", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143871bd0, 0x143a00290> as *const ()), // sub_142642820
    Replacement::draft(0x1426428b0, "unknown_1426428b0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872800, 0x143a00290> as *const ()), // sub_1426428b0
    Replacement::draft(0x142642940, "unknown_142642940", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872820, 0x143a00290> as *const ()), // sub_142642940
    Replacement::draft(0x1426429d0, "unknown_1426429d0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872878, 0x143a00290> as *const ()), // sub_1426429d0
    Replacement::draft(0x142642a60, "unknown_142642a60", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438728e0, 0x143a00290> as *const ()), // sub_142642a60
    Replacement::draft(0x142642af0, "unknown_142642af0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438728b0, 0x143a00290> as *const ()), // sub_142642af0
    Replacement::draft(0x142642b80, "unknown_142642b80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438728c8, 0x143a00290> as *const ()), // sub_142642b80
    Replacement::draft(0x142642c10, "unknown_142642c10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872840, 0x143a00290> as *const ()), // sub_142642c10
    Replacement::draft(0x142642ca0, "unknown_142642ca0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872890, 0x143a00290> as *const ()), // sub_142642ca0
    Replacement::draft(0x142642d30, "unknown_142642d30", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872858, 0x143a00290> as *const ()), // sub_142642d30
    Replacement::draft(0x142642dc0, "unknown_142642dc0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438728f8, 0x143a00290> as *const ()), // sub_142642dc0
    Replacement::draft(0x142642e50, "unknown_142642e50", fn_142642e50 as *const ()), // sub_142642e50
    Replacement::draft(0x142642e80, "unknown_142642e80", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872948, 0x143a00290> as *const ()), // sub_142642e80
    Replacement::draft(0x142642f10, "unknown_142642f10", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872980, 0x143a00290> as *const ()), // sub_142642f10
    Replacement::draft(0x142642fa0, "unknown_142642fa0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438729d0, 0x143a00290> as *const ()), // sub_142642fa0
    Replacement::draft(0x142643030, "unknown_142643030", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438729b8, 0x143a00290> as *const ()), // sub_142643030
    Replacement::draft(0x1426430c0, "unknown_1426430c0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872918, 0x143a00290> as *const ()), // sub_1426430c0
    Replacement::draft(0x142643150, "unknown_142643150", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872930, 0x143a00290> as *const ()), // sub_142643150
    Replacement::draft(0x1426431e0, "unknown_1426431e0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872a28, 0x143a00290> as *const ()), // sub_1426431e0
    Replacement::draft(0x142643270, "unknown_142643270", tpl_111514::<0x143a001ac, 0x1434077a8, 0x1438729e8, 0x143a00290> as *const ()), // sub_142643270
    Replacement::draft(0x142643300, "unknown_142643300", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872998, 0x143a00290> as *const ()), // sub_142643300
    Replacement::draft(0x142643390, "unknown_142643390", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872a08, 0x143a00290> as *const ()), // sub_142643390
    Replacement::draft(0x142643420, "unknown_142643420", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872960, 0x143a00290> as *const ()), // sub_142643420
    Replacement::draft(0x1426434b0, "unknown_1426434b0", tpl_111627::<0x143c4cf58, 0x1401e5380, 0x143c4cf48, 0x8, 0x1401e66f0> as *const ()), // sub_1426434b0
    Replacement::draft(0x142643510, "unknown_142643510", tpl_111627::<0x143c47e48, 0x1401e5380, 0x143c47e38, 0x8, 0x1401e66f0> as *const ()), // sub_142643510
    Replacement::draft(0x142643570, "unknown_142643570", tpl_111627::<0x143c47e88, 0x1401e5380, 0x143c47e78, 0x8, 0x1401e66f0> as *const ()), // sub_142643570
    Replacement::draft(0x142643660, "unknown_142643660", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872a68, 0x143a00290> as *const ()), // sub_142643660
    Replacement::draft(0x1426436f0, "unknown_1426436f0", tpl_111514::<0x143a001ac, 0x1434077a8, 0x143872a48, 0x143a00290> as *const ()), // sub_1426436f0
    Replacement::draft(0x142643780, "unknown_142643780", tpl_327::<0x143872ac0, 0x141405940> as *const ()), // sub_142643780
    Replacement::draft(0x142643790, "unknown_142643790", tpl_327::<0x143d01b08, 0x142201e30> as *const ()), // sub_142643790
    Replacement::draft(0x1426437a0, "unknown_1426437a0", tpl_327::<0x143d01b90, 0x142201e30> as *const ()), // sub_1426437a0
    Replacement::draft(0x1426437b0, "unknown_1426437b0", tpl_327::<0x143d01b58, 0x142201e30> as *const ()), // sub_1426437b0
    Replacement::draft(0x1426437c0, "unknown_1426437c0", tpl_327::<0x143d01ba8, 0x142201e30> as *const ()), // sub_1426437c0
    Replacement::draft(0x1426437d0, "unknown_1426437d0", tpl_327::<0x143d01b70, 0x142201e30> as *const ()), // sub_1426437d0
    Replacement::draft(0x1426437e0, "unknown_1426437e0", tpl_327::<0x143d01b38, 0x142201e30> as *const ()), // sub_1426437e0
    Replacement::draft(0x1426437f0, "unknown_1426437f0", tpl_53273::<0x143c47f68, 0x140246bb0, 0x1425371a0, 0x142533200> as *const ()), // sub_1426437f0
    Replacement::draft(0x142643950, "unknown_142643950", tpl_369 as *const ()), // sub_142643950
    Replacement::draft(0x142643960, "unknown_142643960", tpl_43614::<0x143c47f58, 0x1401f2bc0> as *const ()), // sub_142643960
    Replacement::draft(0x142643980, "renoir_142643980", tpl_53418::<0x143419fc8, 0x143867048> as *const ()), // sub_142643980
    Replacement::draft(0x142643990, "unknown_142643990", tpl_327::<0x143d01bd0, 0x14240ab10> as *const ()), // sub_142643990
    Replacement::draft(0x14264399c, "unknown_14264399c", tpl_53419::<0x143813018, 0x143813000, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x1434278a0, 0x143813010, 0x1426439de> as *const ()), // sub_14264399c
    Replacement::draft(0x142643a00, "unknown_142643a00", tpl_53419::<0x14392bd58, 0x14392bd40, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x1434278a0, 0x14392bd50, 0x142643a42> as *const ()), // sub_142643a00
    Replacement::draft(0x142643a64, "unknown_142643a64", tpl_53419::<0x143813038, 0x143813020, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x1434278a0, 0x143813030, 0x142643aa6> as *const ()), // sub_142643a64
    Replacement::draft(0x142643ac8, "unknown_142643ac8", tpl_53419::<0x14392bd78, 0x14392bd60, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x1434278a0, 0x14392bd70, 0x142643b0a> as *const ()), // sub_142643ac8
    Replacement::draft(0x142643b2c, "unknown_142643b2c", tpl_53419::<0x143813068, 0x143813050, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x1434278a0, 0x143813060, 0x142643b6e> as *const ()), // sub_142643b2c
    Replacement::draft(0x142643b90, "unknown_142643b90", tpl_53419::<0x14392bd98, 0x14392bd80, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x1434278a0, 0x14392bd90, 0x142643bd2> as *const ()), // sub_142643b90
    Replacement::draft(0x142643bf4, "unknown_142643bf4", tpl_53420::<0x14392bda8, 0x8, 0xc> as *const ()), // sub_142643bf4
    Replacement::draft(0x142643c38, "unknown_142643c38", tpl_53420::<0x14392bdc0, 0x8, 0xc> as *const ()), // sub_142643c38
    Replacement::draft(0x142643c7c, "unknown_142643c7c", fn_142643c7c as *const ()), // sub_142643c7c
    Replacement::draft(0x142643c9c, "unknown_142643c9c", tpl_53420::<0x14392be58, 0x8, 0xc> as *const ()), // sub_142643c9c
    Replacement::draft(0x142643ce0, "unknown_142643ce0", tpl_327::<0x14392bde0, 0x1414ab3d0> as *const ()), // sub_142643ce0
    Replacement::draft(0x142643cec, "unknown_142643cec", tpl_53419::<0x14392bf00, 0x14392bee8, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x1434278a0, 0x14392bef8, 0x142643d2e> as *const ()), // sub_142643cec
    Replacement::draft(0x142643d50, "unknown_142643d50", tpl_53419::<0x143813088, 0x143813070, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x1434278a0, 0x143813080, 0x142643d92> as *const ()), // sub_142643d50
    Replacement::draft(0x142643db4, "unknown_142643db4", fn_142643db4 as *const ()), // sub_142643db4
    Replacement::draft(0x142643e18, "unknown_142643e18", tpl_85168 as *const ()), // sub_142643e18
    Replacement::draft(0x142643e1c, "unknown_142643e1c", tpl_85168 as *const ()), // sub_142643e1c
    Replacement::draft(0x142643e20, "unknown_142643e20", tpl_85168 as *const ()), // sub_142643e20
    Replacement::draft(0x142643e24, "unknown_142643e24", tpl_85168 as *const ()), // sub_142643e24
    Replacement::draft(0x142643e28, "unknown_142643e28", tpl_85168 as *const ()), // sub_142643e28
    Replacement::draft(0x142643e2c, "unknown_142643e2c", tpl_85168 as *const ()), // sub_142643e2c
    Replacement::draft(0x142643e30, "unknown_142643e30", tpl_85168 as *const ()), // sub_142643e30
    Replacement::draft(0x142643e34, "unknown_142643e34", tpl_85168 as *const ()), // sub_142643e34
    Replacement::draft(0x142643e38, "unknown_142643e38", tpl_85168 as *const ()), // sub_142643e38
    Replacement::draft(0x142643e3c, "unknown_142643e3c", tpl_85168 as *const ()), // sub_142643e3c
    Replacement::draft(0x142643e40, "unknown_142643e40", tpl_85168 as *const ()), // sub_142643e40
    Replacement::draft(0x142643e44, "unknown_142643e44", tpl_85168 as *const ()), // sub_142643e44
    Replacement::draft(0x142643e48, "unknown_142643e48", tpl_85168 as *const ()), // sub_142643e48
    Replacement::draft(0x142643e4c, "unknown_142643e4c", tpl_85168 as *const ()), // sub_142643e4c
    Replacement::draft(0x142643e50, "unknown_142643e50", tpl_85168 as *const ()), // sub_142643e50
    Replacement::draft(0x142643e54, "unknown_142643e54", tpl_85168 as *const ()), // sub_142643e54
    Replacement::draft(0x142643e58, "unknown_142643e58", tpl_85168 as *const ()), // sub_142643e58
    Replacement::draft(0x142643e5c, "unknown_142643e5c", tpl_85168 as *const ()), // sub_142643e5c
    Replacement::draft(0x142643e60, "unknown_142643e60", tpl_85168 as *const ()), // sub_142643e60
    Replacement::draft(0x142643e64, "unknown_142643e64", tpl_85168 as *const ()), // sub_142643e64
    Replacement::draft(0x142643e68, "unknown_142643e68", tpl_85168 as *const ()), // sub_142643e68
    Replacement::draft(0x142643e6c, "unknown_142643e6c", tpl_85168 as *const ()), // sub_142643e6c
    Replacement::draft(0x142643e70, "unknown_142643e70", tpl_85168 as *const ()), // sub_142643e70
    Replacement::draft(0x142643e74, "unknown_142643e74", tpl_85168 as *const ()), // sub_142643e74
    Replacement::draft(0x142643e78, "unknown_142643e78", tpl_85168 as *const ()), // sub_142643e78
    Replacement::draft(0x142643e7c, "unknown_142643e7c", tpl_85168 as *const ()), // sub_142643e7c
    Replacement::draft(0x142643e80, "unknown_142643e80", tpl_85168 as *const ()), // sub_142643e80
    Replacement::draft(0x142643e84, "unknown_142643e84", tpl_85168 as *const ()), // sub_142643e84
    Replacement::draft(0x142643e88, "unknown_142643e88", tpl_85168 as *const ()), // sub_142643e88
    Replacement::draft(0x142643e8c, "unknown_142643e8c", tpl_85168 as *const ()), // sub_142643e8c
    Replacement::draft(0x142643e90, "unknown_142643e90", tpl_85168 as *const ()), // sub_142643e90
    Replacement::draft(0x142643e94, "unknown_142643e94", tpl_85168 as *const ()), // sub_142643e94
    Replacement::draft(0x142643e98, "unknown_142643e98", tpl_85168 as *const ()), // sub_142643e98
    Replacement::draft(0x142643e9c, "unknown_142643e9c", tpl_85168 as *const ()), // sub_142643e9c
    Replacement::draft(0x142643ea0, "unknown_142643ea0", tpl_85168 as *const ()), // sub_142643ea0
    Replacement::draft(0x142643ea4, "unknown_142643ea4", tpl_85168 as *const ()), // sub_142643ea4
    Replacement::draft(0x142643ea8, "unknown_142643ea8", tpl_85168 as *const ()), // sub_142643ea8
    Replacement::draft(0x142643eac, "unknown_142643eac", tpl_85168 as *const ()), // sub_142643eac
    Replacement::draft(0x142643eb0, "unknown_142643eb0", tpl_369 as *const ()), // sub_142643eb0
    Replacement::draft(0x142643ec0, "unknown_142643ec0", tpl_53423::<0x14392c790, 0x14392c778, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x14392c730, 0x14392c788, 0x140200450, 0x142643f02> as *const ()), // sub_142643ec0
    Replacement::draft(0x142643f40, "unknown_142643f40", tpl_53423::<0x14392c720, 0x14392c708, 0xfffffffffffffff8, 0x1426467f0, 0x141834d30, 0x14392c6c0, 0x14392c718, 0x140200450, 0x142643f82> as *const ()), // sub_142643f40
    Replacement::draft(0x142643fb8, "unknown_142643fb8", fn_142643fb8 as *const ()), // sub_142643fb8
    Replacement::draft(0x142643fd8, "unknown_142643fd8", fn_142643fd8 as *const ()), // sub_142643fd8
    Replacement::draft(0x142644000, "unknown_142644000", tpl_369 as *const ()), // sub_142644000
    Replacement::draft(0x142644010, "unknown_142644010", tpl_369 as *const ()), // sub_142644010
    Replacement::draft(0x142644020, "unknown_142644020", tpl_369 as *const ()), // sub_142644020
    Replacement::draft(0x142644030, "unknown_142644030", tpl_369 as *const ()), // sub_142644030
    Replacement::draft(0x142644040, "unknown_142644040", tpl_369 as *const ()), // sub_142644040
    Replacement::draft(0x142644050, "unknown_142644050", tpl_369 as *const ()), // sub_142644050
    Replacement::draft(0x142644060, "unknown_142644060", tpl_0::<0x143930dc8, 0x1426452a0> as *const ()), // sub_142644060
    Replacement::draft(0x142644070, "unknown_142644070", tpl_369 as *const ()), // sub_142644070
    Replacement::draft(0x142644080, "unknown_142644080", tpl_369 as *const ()), // sub_142644080
    Replacement::draft(0x142644090, "unknown_142644090", tpl_327::<0x143930e58, 0x1415372a0> as *const ()), // sub_142644090
    Replacement::draft(0x1426440a0, "unknown_1426440a0", fn_1426440a0 as *const ()), // sub_1426440a0
    Replacement::draft(0x142644110, "unknown_142644110", tpl_369 as *const ()), // sub_142644110
    Replacement::draft(0x142644120, "unknown_142644120", tpl_369 as *const ()), // sub_142644120
    Replacement::draft(0x142644130, "unknown_142644130", tpl_53418::<0x142688c00, 0x143814f18> as *const ()), // sub_142644130
    Replacement::draft(0x142644140, "unknown_142644140", tpl_369 as *const ()), // sub_142644140
    Replacement::draft(0x142644150, "unknown_142644150", tpl_369 as *const ()), // sub_142644150
    Replacement::draft(0x142644160, "unknown_142644160", tpl_369 as *const ()), // sub_142644160
    Replacement::draft(0x142644170, "unknown_142644170", tpl_369 as *const ()), // sub_142644170
    Replacement::draft(0x142644180, "unknown_142644180", tpl_369 as *const ()), // sub_142644180
    Replacement::draft(0x142644190, "unknown_142644190", tpl_0::<0x143931658, 0x1426452a0> as *const ()), // sub_142644190
    Replacement::draft(0x1426441a0, "unknown_1426441a0", tpl_369 as *const ()), // sub_1426441a0
    Replacement::draft(0x1426441b0, "unknown_1426441b0", tpl_369 as *const ()), // sub_1426441b0
    Replacement::draft(0x1426441c0, "unknown_1426441c0", tpl_369 as *const ()), // sub_1426441c0
    Replacement::draft(0x1426441d0, "unknown_1426441d0", tpl_369 as *const ()), // sub_1426441d0
    Replacement::draft(0x1426441e0, "unknown_1426441e0", tpl_369 as *const ()), // sub_1426441e0
    Replacement::draft(0x1426441f0, "unknown_1426441f0", tpl_369 as *const ()), // sub_1426441f0
    Replacement::draft(0x142644200, "unknown_142644200", fn_142644200 as *const ()), // sub_142644200
    Replacement::draft(0x142644240, "unknown_142644240", tpl_0::<0x1439317e0, 0x1426452a0> as *const ()), // sub_142644240
    Replacement::draft(0x142644250, "unknown_142644250", fn_142644250 as *const ()), // sub_142644250
    Replacement::draft(0x142644280, "unknown_142644280", fn_142644280 as *const ()), // sub_142644280
    Replacement::draft(0x1426442e0, "unknown_1426442e0", tpl_0::<0x143931888, 0x1426452a0> as *const ()), // sub_1426442e0
    Replacement::draft(0x1426442f0, "unknown_1426442f0", tpl_0::<0x1439318e0, 0x1426452a0> as *const ()), // sub_1426442f0
    Replacement::draft(0x142644300, "unknown_142644300", tpl_369 as *const ()), // sub_142644300
    Replacement::draft(0x142644310, "unknown_142644310", tpl_369 as *const ()), // sub_142644310
    Replacement::draft(0x142644320, "unknown_142644320", tpl_369 as *const ()), // sub_142644320
    Replacement::draft(0x142644330, "unknown_142644330", tpl_369 as *const ()), // sub_142644330
    Replacement::draft(0x142644340, "unknown_142644340", tpl_369 as *const ()), // sub_142644340
    Replacement::draft(0x142644350, "unknown_142644350", tpl_0::<0x143931c30, 0x1426452a0> as *const ()), // sub_142644350
    Replacement::draft(0x142644360, "unknown_142644360", tpl_369 as *const ()), // sub_142644360
    Replacement::draft(0x142644370, "unknown_142644370", tpl_369 as *const ()), // sub_142644370
    Replacement::draft(0x142644380, "unknown_142644380", tpl_369 as *const ()), // sub_142644380
    Replacement::draft(0x142644390, "unknown_142644390", tpl_327::<0x143931c20, 0x1416473d0> as *const ()), // sub_142644390
    Replacement::draft(0x1426443a0, "unknown_1426443a0", tpl_369 as *const ()), // sub_1426443a0
    Replacement::draft(0x1426443b0, "unknown_1426443b0", tpl_53418::<0x14268e020, 0x143815af8> as *const ()), // sub_1426443b0
    Replacement::draft(0x1426443c0, "unknown_1426443c0", tpl_369 as *const ()), // sub_1426443c0
    Replacement::draft(0x1426443d0, "unknown_1426443d0", tpl_0::<0x143931cb0, 0x1426452a0> as *const ()), // sub_1426443d0
    Replacement::draft(0x1426443e0, "unknown_1426443e0", tpl_53418::<0x142688ff0, 0x143815b30> as *const ()), // sub_1426443e0
    Replacement::draft(0x1426443f0, "unknown_1426443f0", tpl_53418::<0x142688ff0, 0x143815b20> as *const ()), // sub_1426443f0
    Replacement::draft(0x142644400, "unknown_142644400", tpl_53418::<0x142688ff0, 0x143815b28> as *const ()), // sub_142644400
    Replacement::draft(0x142644410, "unknown_142644410", tpl_53418::<0x142688ff0, 0x143815b10> as *const ()), // sub_142644410
    Replacement::draft(0x142644420, "unknown_142644420", tpl_53418::<0x142688ff0, 0x143815b18> as *const ()), // sub_142644420
    Replacement::draft(0x142644430, "unknown_142644430", tpl_53418::<0x14268b538, 0x143815b50> as *const ()), // sub_142644430
    Replacement::draft(0x142644440, "unknown_142644440", tpl_53418::<0x14268b538, 0x143815b40> as *const ()), // sub_142644440
    Replacement::draft(0x142644450, "unknown_142644450", tpl_53418::<0x142689028, 0x143931d20> as *const ()), // sub_142644450
    Replacement::draft(0x142644460, "unknown_142644460", tpl_369 as *const ()), // sub_142644460
    Replacement::draft(0x142644470, "unknown_142644470", tpl_369 as *const ()), // sub_142644470
    Replacement::draft(0x142644480, "unknown_142644480", tpl_369 as *const ()), // sub_142644480
    Replacement::draft(0x142644490, "unknown_142644490", tpl_0::<0x143931d98, 0x1426452a0> as *const ()), // sub_142644490
    Replacement::draft(0x1426444a0, "unknown_1426444a0", tpl_369 as *const ()), // sub_1426444a0
    Replacement::draft(0x1426444b0, "unknown_1426444b0", tpl_369 as *const ()), // sub_1426444b0
    Replacement::draft(0x1426444c0, "unknown_1426444c0", tpl_369 as *const ()), // sub_1426444c0
    Replacement::draft(0x1426444d0, "unknown_1426444d0", tpl_369 as *const ()), // sub_1426444d0
    Replacement::draft(0x1426444e0, "unknown_1426444e0", tpl_369 as *const ()), // sub_1426444e0
    Replacement::draft(0x1426444f0, "unknown_1426444f0", tpl_0::<0x143932108, 0x1426452a0> as *const ()), // sub_1426444f0
    Replacement::draft(0x142644500, "unknown_142644500", tpl_327::<0x1439321f0, 0x141734d70> as *const ()), // sub_142644500
    Replacement::draft(0x142644510, "unknown_142644510", tpl_53418::<0x1426abc20, 0x143821508> as *const ()), // sub_142644510
    Replacement::draft(0x142644520, "unknown_142644520", tpl_0::<0x143979f10, 0x1426452a0> as *const ()), // sub_142644520
    Replacement::draft(0x142644530, "unknown_142644530", tpl_369 as *const ()), // sub_142644530
    Replacement::draft(0x142644540, "unknown_142644540", tpl_369 as *const ()), // sub_142644540
    Replacement::draft(0x142644550, "unknown_142644550", tpl_369 as *const ()), // sub_142644550
    Replacement::draft(0x142644570, "std_142644570", fn_142644570 as *const ()), // std::_Fac_tidy_reg_t::std::_Fac_tidy_reg_t::~_Fac_tidy_reg_t
    Replacement::draft(0x1426445cc, "unknown_1426445cc", tpl_53420::<0x14397d768, 0x8, 0xc> as *const ()), // sub_1426445cc
];
