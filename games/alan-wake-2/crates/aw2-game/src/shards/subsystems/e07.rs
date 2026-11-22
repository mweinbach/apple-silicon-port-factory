#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]
use crate::registry::{Replacement, Throws};
use super::*;

pub static ENTRIES: &[Replacement] = &[
    Replacement::draft_throws(0x140e53d20, "content_140e53d20", tpl_71382::<0x14387fb78, 0x14387fbf0, 0x1401e9fa0, 0x1426cdcb0, 0x328, 0x338, 0x20, 0x369, 0x142409b50, 0x108, 0x1401e9e70> as *const (), Throws::Discard), // sub_140e53d20
    Replacement::draft(0x140e53e20, "content_140e53e20", tpl_12961::<0x14387fbf0, 0x14243b0b0, 0x338, 0x20, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e53e20
    Replacement::draft_throws(0x140e53ea0, "content_140e53ea0", tpl_71384::<0x14387fb78, 0x14387fbf0, 0x1401e9b60, 0x1426cdcb0, 0x328, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0x108, 0x1401e9a20> as *const (), Throws::Discard), // sub_140e53ea0
    Replacement::draft_throws(0x140e53fa0, "content_140e53fa0", fn_140e53fa0 as *const (), Throws::Caught), // sub_140e53fa0
    Replacement::draft_throws(0x140e54060, "content_140e54060", fn_140e54060 as *const (), Throws::Caught), // sub_140e54060
    Replacement::draft_throws(0x140e54120, "content_140e54120", fn_140e54120 as *const (), Throws::Caught), // sub_140e54120
    Replacement::draft(0x140e541e0, "content_140e541e0", tpl_17887::<0x14387fbf0, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e541e0
    Replacement::draft(0x140e542b0, "content_140e542b0", tpl_12962::<0x14387fb78, 0x14387fbf0, 0x1401e9970, 0x338, 0xc0, 0x108, 0x1401e9860> as *const ()), // sub_140e542b0
    Replacement::draft(0x140e54330, "content_140e54330", tpl_12961::<0x14387fbf0, 0x14243b0b0, 0x338, 0xc0, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e54330
    Replacement::draft_throws(0x140e543b0, "content_140e543b0", tpl_12963::<0x14387fb78, 0x14387fbf0, 0x1401e9770, 0x338, 0x1424b4cb0, 0x108, 0x1401e9590> as *const (), Throws::Discard), // sub_140e543b0
    Replacement::draft_throws(0x140e54420, "content_140e54420", tpl_17888::<0x14387fbf0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x14329fb68, 0x1402c4730, 0x108, 0x1432f9d10> as *const (), Throws::Discard), // sub_140e54420
    Replacement::draft(0x140e54500, "content_140e54500", fn_140e54500 as *const ()), // sub_140e54500
    Replacement::draft(0x140e54590, "content_140e54590", fn_140e54590 as *const ()), // sub_140e54590
    Replacement::draft(0x140e54640, "content_140e54640", fn_140e54640 as *const ()), // sub_140e54640
    Replacement::draft_throws(0x140e54920, "content_140e54920", fn_140e54920 as *const (), Throws::Discard), // sub_140e54920
    Replacement::draft(0x140e54b90, "content_140e54b90", fn_140e54b90 as *const ()), // sub_140e54b90
    Replacement::draft_throws(0x140e54cb0, "content_140e54cb0", fn_140e54cb0 as *const (), Throws::Discard), // sub_140e54cb0
    Replacement::draft(0x140e54f60, "content_140e54f60", fn_140e54f60 as *const ()), // sub_140e54f60
    Replacement::draft_throws(0x140e55040, "content_140e55040", fn_140e55040 as *const (), Throws::Caught), // sub_140e55040
    Replacement::draft_throws(0x140e55430, "content_140e55430", fn_140e55430 as *const (), Throws::Caught), // sub_140e55430
    Replacement::draft_throws(0x140e557d0, "content_140e557d0", fn_140e557d0 as *const (), Throws::Caught), // sub_140e557d0
    Replacement::draft(0x140e55b60, "content_140e55b60", fn_140e55b60 as *const ()), // sub_140e55b60
    Replacement::draft(0x140e55d00, "content_140e55d00", fn_140e55d00 as *const ()), // sub_140e55d00
    Replacement::draft(0x140e55e30, "content_140e55e30", fn_140e55e30 as *const ()), // sub_140e55e30
    Replacement::draft_throws(0x140e55f70, "content_140e55f70", fn_140e55f70 as *const (), Throws::Discard), // sub_140e55f70
    Replacement::draft_throws(0x140e560c0, "content_140e560c0", fn_140e560c0 as *const (), Throws::Discard), // sub_140e560c0
    Replacement::draft(0x140e56350, "content_140e56350", fn_140e56350 as *const ()), // sub_140e56350
    Replacement::draft(0x140e56400, "content_140e56400", fn_140e56400 as *const ()), // sub_140e56400
    Replacement::draft(0x140e56490, "content_140e56490", fn_140e56490 as *const ()), // sub_140e56490
    Replacement::draft(0x140e56540, "content_140e56540", fn_140e56540 as *const ()), // sub_140e56540
    Replacement::draft(0x140e565d0, "content_140e565d0", fn_140e565d0 as *const ()), // sub_140e565d0
    Replacement::draft_throws(0x140e56770, "content_140e56770", fn_140e56770 as *const (), Throws::Discard), // sub_140e56770
    Replacement::draft(0x140e568c0, "content_140e568c0", fn_140e568c0 as *const ()), // sub_140e568c0
    Replacement::draft_throws(0x140e569a0, "content_140e569a0", fn_140e569a0 as *const (), Throws::Discard), // sub_140e569a0
    Replacement::draft_throws(0x140e56b10, "content_140e56b10", fn_140e56b10 as *const (), Throws::Caught), // sub_140e56b10
    Replacement::draft_throws(0x140e56d90, "content_140e56d90", fn_140e56d90 as *const (), Throws::Caught), // sub_140e56d90
    Replacement::draft_throws(0x140e56fd0, "content_140e56fd0", fn_140e56fd0 as *const (), Throws::Caught), // sub_140e56fd0
    Replacement::draft(0x140e57210, "content_140e57210", fn_140e57210 as *const ()), // sub_140e57210
    Replacement::draft(0x140e57310, "content_140e57310", fn_140e57310 as *const ()), // sub_140e57310
    Replacement::draft(0x140e573f0, "content_140e573f0", fn_140e573f0 as *const ()), // sub_140e573f0
    Replacement::draft_throws(0x140e574e0, "content_140e574e0", fn_140e574e0 as *const (), Throws::Discard), // sub_140e574e0
    Replacement::draft_throws(0x140e575d0, "content_140e575d0", fn_140e575d0 as *const (), Throws::Discard), // sub_140e575d0
    Replacement::draft(0x140e57710, "content_140e57710", tpl_17854::<0x14387fbd8, 0x14243b0b0, 0x1426d4580> as *const ()), // sub_140e57710
    Replacement::draft_throws(0x140e57950, "content_140e57950", fn_140e57950 as *const (), Throws::Discard), // sub_140e57950
    Replacement::draft(0x140e57af0, "content_140e57af0", fn_140e57af0 as *const ()), // sub_140e57af0
    Replacement::draft_throws(0x140e57c00, "content_140e57c00", fn_140e57c00 as *const (), Throws::Caught), // sub_140e57c00
    Replacement::draft(0x140e57de0, "content_140e57de0", tpl_17858::<0x8, 0x1401e84a0, 0x140943c70> as *const ()), // sub_140e57de0
    Replacement::draft_throws(0x140e57e70, "content_140e57e70", fn_140e57e70 as *const (), Throws::Caught), // sub_140e57e70
    Replacement::draft_throws(0x140e581b0, "content_140e581b0", fn_140e581b0 as *const (), Throws::Caught), // sub_140e581b0
    Replacement::draft_throws(0x140e584b0, "content_140e584b0", fn_140e584b0 as *const (), Throws::Caught), // sub_140e584b0
    Replacement::draft(0x140e585a0, "content_140e585a0", fn_140e585a0 as *const ()), // sub_140e585a0
    Replacement::draft(0x140e58760, "content_140e58760", fn_140e58760 as *const ()), // sub_140e58760
    Replacement::draft(0x140e58840, "content_140e58840", fn_140e58840 as *const ()), // sub_140e58840
    Replacement::draft_throws(0x140e58920, "content_140e58920", tpl_17863::<0x14387fba0, 0x14387fbd8, 0x1401e9770, 0x338, 0x120, 0x8, 0x28, 0x20, 0x18, 0x1401e9590> as *const (), Throws::Discard), // sub_140e58920
    Replacement::draft_throws(0x140e58a00, "content_140e58a00", fn_140e58a00 as *const (), Throws::Discard), // sub_140e58a00
    Replacement::draft(0x140e58af0, "content_140e58af0", tpl_17865::<0x1432f9d30, 0x8, 0x140947eb0, 0x28, 0x20, 0x1432f9d40> as *const ()), // sub_140e58af0
    Replacement::draft(0x140e58b80, "content_140e58b80", fn_140e58b80 as *const ()), // sub_140e58b80
    Replacement::draft(0x140e58c70, "content_140e58c70", fn_140e58c70 as *const ()), // sub_140e58c70
    Replacement::draft(0x140e58d10, "content_140e58d10", fn_140e58d10 as *const ()), // sub_140e58d10
    Replacement::draft_throws(0x140e58d90, "content_140e58d90", fn_140e58d90 as *const (), Throws::Caught), // sub_140e58d90
    Replacement::draft(0x140e59030, "content_140e59030", fn_140e59030 as *const ()), // sub_140e59030
    Replacement::draft_throws(0x140e59110, "content_140e59110", fn_140e59110 as *const (), Throws::Caught), // sub_140e59110
    Replacement::draft_throws(0x140e593b0, "content_140e593b0", fn_140e593b0 as *const (), Throws::Caught), // sub_140e593b0
    Replacement::draft_throws(0x140e59650, "content_140e59650", fn_140e59650 as *const (), Throws::Caught), // sub_140e59650
    Replacement::draft_throws(0x140e598f0, "content_140e598f0", fn_140e598f0 as *const (), Throws::Caught), // sub_140e598f0
    Replacement::draft_throws(0x140e59ac0, "content_140e59ac0", fn_140e59ac0 as *const (), Throws::Caught), // sub_140e59ac0
    Replacement::draft(0x140e59d90, "content_140e59d90", fn_140e59d90 as *const ()), // sub_140e59d90
    Replacement::draft(0x140e59e80, "content_140e59e80", fn_140e59e80 as *const ()), // sub_140e59e80
    Replacement::draft(0x140e59fd0, "content_140e59fd0", fn_140e59fd0 as *const ()), // sub_140e59fd0
    Replacement::draft(0x140e5a0c0, "content_140e5a0c0", fn_140e5a0c0 as *const ()), // sub_140e5a0c0
    Replacement::draft(0x140e5a160, "content_140e5a160", fn_140e5a160 as *const ()), // sub_140e5a160
    Replacement::draft_throws(0x140e5a2d0, "content_140e5a2d0", fn_140e5a2d0 as *const (), Throws::Caught), // sub_140e5a2d0
    Replacement::draft_throws(0x140e5a3d0, "content_140e5a3d0", fn_140e5a3d0 as *const (), Throws::Caught), // sub_140e5a3d0
    Replacement::draft_throws(0x140e5a4d0, "content_140e5a4d0", fn_140e5a4d0 as *const (), Throws::Caught), // sub_140e5a4d0
    Replacement::draft(0x140e5a6b0, "content_140e5a6b0", fn_140e5a6b0 as *const ()), // sub_140e5a6b0
    Replacement::draft(0x140e5a7d0, "content_140e5a7d0", fn_140e5a7d0 as *const ()), // sub_140e5a7d0
    Replacement::draft(0x140e5a850, "content_140e5a850", fn_140e5a850 as *const ()), // sub_140e5a850
    Replacement::draft(0x140e5a8d0, "content_140e5a8d0", fn_140e5a8d0 as *const ()), // sub_140e5a8d0
    Replacement::draft_throws(0x140e5a990, "content_140e5a990", fn_140e5a990 as *const (), Throws::Caught), // sub_140e5a990
    Replacement::draft_throws(0x140e5ab60, "content_140e5ab60", fn_140e5ab60 as *const (), Throws::Caught), // sub_140e5ab60
    Replacement::draft_throws(0x140e5ad60, "content_140e5ad60", fn_140e5ad60 as *const (), Throws::Caught), // sub_140e5ad60
    Replacement::draft(0x140e5af30, "content_140e5af30", fn_140e5af30 as *const ()), // sub_140e5af30
    Replacement::draft_throws(0x140e5afe0, "content_140e5afe0", fn_140e5afe0 as *const (), Throws::Caught), // sub_140e5afe0
    Replacement::draft(0x140e5b1b0, "content_140e5b1b0", fn_140e5b1b0 as *const ()), // sub_140e5b1b0
    Replacement::draft(0x140e5b1d0, "content_140e5b1d0", tpl_71152::<0x140e5efc0> as *const ()), // sub_140e5b1d0
    Replacement::draft_throws(0x140e5b310, "content_140e5b310", fn_140e5b310 as *const (), Throws::Caught), // sub_140e5b310
    Replacement::draft_throws(0x140e5ba00, "content_140e5ba00", fn_140e5ba00 as *const (), Throws::Caught), // sub_140e5ba00
    Replacement::draft_throws(0x140e5c0c0, "content_140e5c0c0", fn_140e5c0c0 as *const (), Throws::Caught), // sub_140e5c0c0
    Replacement::draft_throws(0x140e5c500, "content_140e5c500", fn_140e5c500 as *const (), Throws::Caught), // sub_140e5c500
    Replacement::draft_throws(0x140e5c880, "content_140e5c880", fn_140e5c880 as *const (), Throws::Caught), // sub_140e5c880
    Replacement::draft_throws(0x140e5cbb0, "content_140e5cbb0", fn_140e5cbb0 as *const (), Throws::Caught), // sub_140e5cbb0
    Replacement::draft_throws(0x140e5cec0, "content_140e5cec0", fn_140e5cec0 as *const (), Throws::Caught), // sub_140e5cec0
    Replacement::draft_throws(0x140e5d240, "content_140e5d240", fn_140e5d240 as *const (), Throws::Caught), // sub_140e5d240
    Replacement::draft_throws(0x140e5d570, "content_140e5d570", fn_140e5d570 as *const (), Throws::Caught), // sub_140e5d570
    Replacement::draft(0x140e5d880, "content_140e5d880", fn_140e5d880 as *const ()), // sub_140e5d880
    Replacement::draft(0x140e5db10, "content_140e5db10", tpl_69838::<0x140e5efc0, 0x1401e84a0> as *const ()), // sub_140e5db10
    Replacement::draft_throws(0x140e5dd30, "content_140e5dd30", fn_140e5dd30 as *const (), Throws::Caught), // sub_140e5dd30
    Replacement::draft_throws(0x140e5e3e0, "content_140e5e3e0", fn_140e5e3e0 as *const (), Throws::Caught), // sub_140e5e3e0
    Replacement::draft_throws(0x140e5ea10, "content_140e5ea10", fn_140e5ea10 as *const (), Throws::Caught), // sub_140e5ea10
    Replacement::draft(0x140e5efc0, "content_140e5efc0", fn_140e5efc0 as *const ()), // sub_140e5efc0
    Replacement::draft(0x140e5f070, "content_140e5f070", fn_140e5f070 as *const ()), // sub_140e5f070
    Replacement::draft(0x140e5f180, "content_140e5f180", fn_140e5f180 as *const ()), // sub_140e5f180
    Replacement::draft_throws(0x140e5f200, "content_140e5f200", fn_140e5f200 as *const (), Throws::Discard), // sub_140e5f200
    Replacement::draft(0x140e5f390, "content_140e5f390", fn_140e5f390 as *const ()), // sub_140e5f390
    Replacement::draft_throws(0x140e5f590, "content_140e5f590", fn_140e5f590 as *const (), Throws::Discard), // sub_140e5f590
    Replacement::draft(0x140e5f7b0, "content_140e5f7b0", fn_140e5f7b0 as *const ()), // sub_140e5f7b0
    Replacement::draft_throws(0x140e5f920, "content_140e5f920", fn_140e5f920 as *const (), Throws::Caught), // sub_140e5f920
    Replacement::draft_throws(0x140e5f9f0, "content_140e5f9f0", fn_140e5f9f0 as *const (), Throws::Caught), // sub_140e5f9f0
    Replacement::draft_throws(0x140e5fac0, "content_140e5fac0", fn_140e5fac0 as *const (), Throws::Caught), // sub_140e5fac0
    Replacement::draft(0x140e5fb90, "content_140e5fb90", fn_140e5fb90 as *const ()), // sub_140e5fb90
    Replacement::draft(0x140e5fce0, "content_140e5fce0", fn_140e5fce0 as *const ()), // sub_140e5fce0
    Replacement::draft(0x140e5feb0, "content_140e5feb0", fn_140e5feb0 as *const ()), // sub_140e5feb0
    Replacement::draft_throws(0x140e60080, "content_140e60080", fn_140e60080 as *const (), Throws::Discard), // sub_140e60080
    Replacement::draft_throws(0x140e60220, "content_140e60220", fn_140e60220 as *const (), Throws::Discard), // sub_140e60220
    Replacement::draft(0x140e602f0, "content_140e602f0", fn_140e602f0 as *const ()), // sub_140e602f0
    Replacement::draft(0x140e603e0, "content_140e603e0", fn_140e603e0 as *const ()), // sub_140e603e0
    Replacement::draft(0x140e60510, "content_140e60510", fn_140e60510 as *const ()), // sub_140e60510
    Replacement::draft(0x140e60590, "content_140e60590", fn_140e60590 as *const ()), // sub_140e60590
    Replacement::draft_throws(0x140e60770, "content_140e60770", fn_140e60770 as *const (), Throws::Discard), // sub_140e60770
    Replacement::draft(0x140e60990, "content_140e60990", fn_140e60990 as *const ()), // sub_140e60990
    Replacement::draft_throws(0x140e60ae0, "content_140e60ae0", fn_140e60ae0 as *const (), Throws::Discard), // sub_140e60ae0
    Replacement::draft(0x140e60d70, "content_140e60d70", fn_140e60d70 as *const ()), // sub_140e60d70
    Replacement::draft_throws(0x140e60eb0, "content_140e60eb0", fn_140e60eb0 as *const (), Throws::Caught), // sub_140e60eb0
    Replacement::draft_throws(0x140e611a0, "content_140e611a0", fn_140e611a0 as *const (), Throws::Caught), // sub_140e611a0
    Replacement::draft_throws(0x140e61490, "content_140e61490", fn_140e61490 as *const (), Throws::Caught), // sub_140e61490
    Replacement::draft(0x140e61780, "content_140e61780", fn_140e61780 as *const ()), // sub_140e61780
    Replacement::draft(0x140e618e0, "content_140e618e0", fn_140e618e0 as *const ()), // sub_140e618e0
    Replacement::draft(0x140e61a30, "content_140e61a30", fn_140e61a30 as *const ()), // sub_140e61a30
    Replacement::draft_throws(0x140e61b90, "content_140e61b90", fn_140e61b90 as *const (), Throws::Discard), // sub_140e61b90
    Replacement::draft_throws(0x140e61d30, "content_140e61d30", fn_140e61d30 as *const (), Throws::Caught), // sub_140e61d30
    Replacement::draft(0x140e62060, "content_140e62060", fn_140e62060 as *const ()), // sub_140e62060
    Replacement::draft(0x140e62110, "content_140e62110", fn_140e62110 as *const ()), // sub_140e62110
    Replacement::draft(0x140e621b0, "content_140e621b0", fn_140e621b0 as *const ()), // sub_140e621b0
    Replacement::draft(0x140e62260, "content_140e62260", fn_140e62260 as *const ()), // sub_140e62260
    Replacement::draft(0x140e62310, "content_140e62310", fn_140e62310 as *const ()), // sub_140e62310
    Replacement::draft(0x140e62420, "content_140e62420", fn_140e62420 as *const ()), // sub_140e62420
    Replacement::draft_throws(0x140e624d0, "content_140e624d0", tpl_7851::<0x14387fbf8, 0x14387fc18, 0x1401e9fa0, 0x14080d7a0, 0x328, 0x338, 0x58, 0x40, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140e624d0
    Replacement::draft_throws(0x140e62560, "content_140e62560", tpl_7852::<0x14387fc18, 0x14243b0b0, 0x14080d9d0, 0x338, 0x58, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const (), Throws::Discard), // sub_140e62560
    Replacement::draft_throws(0x140e625e0, "content_140e625e0", tpl_7972::<0x14387fbf8, 0x14387fc18, 0x1401e9b60, 0x14080db90, 0x328, 0x338, 0x120, 0x58, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140e625e0
    Replacement::draft_throws(0x140e62670, "content_140e62670", fn_140e62670 as *const (), Throws::Caught), // sub_140e62670
    Replacement::draft_throws(0x140e62740, "content_140e62740", fn_140e62740 as *const (), Throws::Caught), // sub_140e62740
    Replacement::draft_throws(0x140e62810, "content_140e62810", fn_140e62810 as *const (), Throws::Caught), // sub_140e62810
    Replacement::draft(0x140e628e0, "content_140e628e0", tpl_7855::<0x14387fc18, 0x14243b0b0, 0x14080e450, 0x338, 0x58, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e628e0
    Replacement::draft(0x140e62960, "content_140e62960", tpl_7856::<0x14387fbf8, 0x14387fc18, 0x1401e9970, 0x14080e690, 0x338, 0x58, 0xe0, 0x1401e9860> as *const ()), // sub_140e62960
    Replacement::draft(0x140e629d0, "content_140e629d0", tpl_7857::<0x14387fc18, 0x14243b0b0, 0x14080e8c0, 0x338, 0x58, 0xe0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e629d0
    Replacement::draft_throws(0x140e62a50, "content_140e62a50", tpl_7974::<0x14387fbf8, 0x14387fc18, 0x1401e9770, 0x14080eb00, 0x338, 0x58, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140e62a50
    Replacement::draft_throws(0x140e62ad0, "content_140e62ad0", tpl_7975::<0x14387fc18, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14080ed60, 0x58, 0x1432f8ac8, 0x1, 0x14020f120> as *const (), Throws::Discard), // sub_140e62ad0
    Replacement::draft(0x140e62ba0, "content_140e62ba0", fn_140e62ba0 as *const ()), // sub_140e62ba0
    Replacement::draft(0x140e62c70, "content_140e62c70", fn_140e62c70 as *const ()), // sub_140e62c70
    Replacement::draft(0x140e62d50, "content_140e62d50", tpl_16646::<0x14387fc00, 0x14243b0b0, 0x14081bf10, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e62d50
    Replacement::draft(0x140e62dc0, "content_140e62dc0", fn_140e62dc0 as *const ()), // sub_140e62dc0
    Replacement::draft_throws(0x140e62f00, "content_140e62f00", fn_140e62f00 as *const (), Throws::Discard), // sub_140e62f00
    Replacement::draft(0x140e630a0, "content_140e630a0", fn_140e630a0 as *const ()), // sub_140e630a0
    Replacement::draft_throws(0x140e631a0, "content_140e631a0", fn_140e631a0 as *const (), Throws::Discard), // sub_140e631a0
    Replacement::draft(0x140e63360, "content_140e63360", fn_140e63360 as *const ()), // sub_140e63360
    Replacement::draft_throws(0x140e63440, "content_140e63440", fn_140e63440 as *const (), Throws::Caught), // sub_140e63440
    Replacement::draft_throws(0x140e63680, "content_140e63680", fn_140e63680 as *const (), Throws::Caught), // sub_140e63680
    Replacement::draft_throws(0x140e638c0, "content_140e638c0", fn_140e638c0 as *const (), Throws::Caught), // sub_140e638c0
    Replacement::draft(0x140e63b00, "content_140e63b00", fn_140e63b00 as *const ()), // sub_140e63b00
    Replacement::draft(0x140e63bf0, "content_140e63bf0", fn_140e63bf0 as *const ()), // sub_140e63bf0
    Replacement::draft(0x140e63cf0, "content_140e63cf0", fn_140e63cf0 as *const ()), // sub_140e63cf0
    Replacement::draft_throws(0x140e63df0, "content_140e63df0", fn_140e63df0 as *const (), Throws::Discard), // sub_140e63df0
    Replacement::draft_throws(0x140e63f20, "content_140e63f20", fn_140e63f20 as *const (), Throws::Discard), // sub_140e63f20
    Replacement::draft(0x140e64120, "content_140e64120", fn_140e64120 as *const ()), // content::DialogueTimelineTrack::content::DialogueTimelineTrack::vfunc_0
    Replacement::draft(0x140e64170, "content_140e64170", tpl_5539::<0x8, 0x1401ea4e0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140e64170
    Replacement::draft(0x140e64240, "content_140e64240", fn_140e64240 as *const ()), // sub_140e64240
    Replacement::draft_throws(0x140e643e0, "content_140e643e0", tpl_9695::<0x20, 0x8, 0x140e64d00> as *const (), Throws::Discard), // sub_140e643e0
    Replacement::draft(0x140e64430, "content_140e64430", fn_140e64430 as *const ()), // sub_140e64430
    Replacement::draft_throws(0x140e64510, "content_140e64510", fn_140e64510 as *const (), Throws::Caught), // sub_140e64510
    Replacement::draft_throws(0x140e647e0, "content_140e647e0", fn_140e647e0 as *const (), Throws::Caught), // sub_140e647e0
    Replacement::draft_throws(0x140e64a70, "content_140e64a70", fn_140e64a70 as *const (), Throws::Caught), // sub_140e64a70
    Replacement::draft_throws(0x140e64d00, "content_140e64d00", fn_140e64d00 as *const (), Throws::Caught), // sub_140e64d00
    Replacement::draft_throws(0x140e64dd0, "content_140e64dd0", fn_140e64dd0 as *const (), Throws::Caught), // sub_140e64dd0
    Replacement::draft_throws(0x140e65350, "content_140e65350", fn_140e65350 as *const (), Throws::Caught), // sub_140e65350
    Replacement::draft_throws(0x140e658a0, "content_140e658a0", fn_140e658a0 as *const (), Throws::Caught), // sub_140e658a0
    Replacement::draft(0x140e65ce0, "content_140e65ce0", fn_140e65ce0 as *const ()), // sub_140e65ce0
    Replacement::draft_throws(0x140e65e80, "content_140e65e80", fn_140e65e80 as *const (), Throws::Caught), // sub_140e65e80
    Replacement::draft_throws(0x140e66350, "content_140e66350", fn_140e66350 as *const (), Throws::Caught), // sub_140e66350
    Replacement::draft_throws(0x140e667e0, "content_140e667e0", fn_140e667e0 as *const (), Throws::Caught), // sub_140e667e0
    Replacement::draft_throws(0x140e66c70, "content_140e66c70", fn_140e66c70 as *const (), Throws::Discard), // sub_140e66c70
    Replacement::draft(0x140e66cb0, "content_140e66cb0", tpl_17984::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140e64170> as *const ()), // sub_140e66cb0
    Replacement::draft_throws(0x140e66e60, "content_140e66e60", fn_140e66e60 as *const (), Throws::Caught), // sub_140e66e60
    Replacement::draft_throws(0x140e66f50, "content_140e66f50", fn_140e66f50 as *const (), Throws::Caught), // sub_140e66f50
    Replacement::draft_throws(0x140e67260, "content_140e67260", fn_140e67260 as *const (), Throws::Caught), // sub_140e67260
    Replacement::draft_throws(0x140e67530, "content_140e67530", fn_140e67530 as *const (), Throws::Caught), // sub_140e67530
    Replacement::draft_throws(0x140e677f0, "content_140e677f0", fn_140e677f0 as *const (), Throws::Caught), // sub_140e677f0
    Replacement::draft_throws(0x140e67cc0, "content_140e67cc0", fn_140e67cc0 as *const (), Throws::Caught), // sub_140e67cc0
    Replacement::draft_throws(0x140e68150, "content_140e68150", fn_140e68150 as *const (), Throws::Caught), // sub_140e68150
    Replacement::draft_throws(0x140e685e0, "content_140e685e0", fn_140e685e0 as *const (), Throws::Caught), // sub_140e685e0
    Replacement::draft_throws(0x140e68670, "content_140e68670", fn_140e68670 as *const (), Throws::Discard), // sub_140e68670
    Replacement::draft(0x140e686f0, "content_140e686f0", fn_140e686f0 as *const ()), // sub_140e686f0
    Replacement::draft(0x140e68800, "content_140e68800", fn_140e68800 as *const ()), // sub_140e68800
    Replacement::draft(0x140e68ad0, "content_140e68ad0", tpl_17990::<0x1401f2bc0, 0xc, 0x8, 0x1402d5c10, 0x14183acc8> as *const ()), // sub_140e68ad0
    Replacement::draft(0x140e68b90, "content_140e68b90", fn_140e68b90 as *const ()), // sub_140e68b90
    Replacement::draft(0x140e68f00, "content_140e68f00", fn_140e68f00 as *const ()), // sub_140e68f00
    Replacement::draft(0x140e69270, "content_140e69270", fn_140e69270 as *const ()), // sub_140e69270
    Replacement::draft(0x140e69420, "content_140e69420", fn_140e69420 as *const ()), // sub_140e69420
    Replacement::draft(0x140e69730, "content_140e69730", fn_140e69730 as *const ()), // sub_140e69730
    Replacement::draft(0x140e69a80, "content_140e69a80", fn_140e69a80 as *const ()), // sub_140e69a80
    Replacement::draft(0x140e69b80, "content_140e69b80", fn_140e69b80 as *const ()), // sub_140e69b80
    Replacement::draft(0x140e69c80, "content_140e69c80", fn_140e69c80 as *const ()), // sub_140e69c80
    Replacement::draft(0x140e69cf0, "content_140e69cf0", fn_140e69cf0 as *const ()), // sub_140e69cf0
    Replacement::draft(0x140e69d80, "content_140e69d80", fn_140e69d80 as *const ()), // sub_140e69d80
    Replacement::draft(0x140e6a490, "content_140e6a490", fn_140e6a490 as *const ()), // content::VersionsMetadata::content::VersionsMetadata::vfunc_0
    Replacement::draft(0x140e6a500, "content_140e6a500", tpl_2030::<0x140e6a5b0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140e6a500
    Replacement::draft(0x140e6a5b0, "content_140e6a5b0", fn_140e6a5b0 as *const ()), // sub_140e6a5b0
    Replacement::draft(0x140e6a630, "content_140e6a630", fn_140e6a630 as *const ()), // sub_140e6a630
    Replacement::draft(0x140e6a6f0, "content_140e6a6f0", fn_140e6a6f0 as *const ()), // sub_140e6a6f0
    Replacement::draft(0x140e6a7f0, "content_140e6a7f0", tpl_12891::<0x14387fc48, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x90, 0x8b, 0x8c, 0x88, 0xfffffffffffffff8> as *const ()), // sub_140e6a7f0
    Replacement::draft_throws(0x140e6a8f0, "content_140e6a8f0", tpl_71382::<0x14387fc34, 0x14387fc48, 0x1401e9fa0, 0x1426cdcb0, 0x328, 0x338, 0x20, 0x369, 0x142409b50, 0x88, 0x1401e9e70> as *const (), Throws::Discard), // sub_140e6a8f0
    Replacement::draft(0x140e6a9f0, "content_140e6a9f0", tpl_12961::<0x14387fc48, 0x14243b0b0, 0x338, 0x20, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e6a9f0
    Replacement::draft_throws(0x140e6aa70, "content_140e6aa70", tpl_71384::<0x14387fc34, 0x14387fc48, 0x1401e9b60, 0x1426cdcb0, 0x328, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0x88, 0x1401e9a20> as *const (), Throws::Discard), // sub_140e6aa70
    Replacement::draft_throws(0x140e6ab70, "content_140e6ab70", fn_140e6ab70 as *const (), Throws::Caught), // sub_140e6ab70
    Replacement::draft_throws(0x140e6af50, "content_140e6af50", fn_140e6af50 as *const (), Throws::Caught), // sub_140e6af50
    Replacement::draft_throws(0x140e6b2c0, "content_140e6b2c0", fn_140e6b2c0 as *const (), Throws::Caught), // sub_140e6b2c0
    Replacement::draft(0x140e6b620, "content_140e6b620", tpl_17887::<0x14387fc48, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e6b620
    Replacement::draft(0x140e6b6f0, "content_140e6b6f0", tpl_12962::<0x14387fc34, 0x14387fc48, 0x1401e9970, 0x338, 0xc0, 0x88, 0x1401e9860> as *const ()), // sub_140e6b6f0
    Replacement::draft(0x140e6b770, "content_140e6b770", tpl_12961::<0x14387fc48, 0x14243b0b0, 0x338, 0xc0, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e6b770
    Replacement::draft_throws(0x140e6b7f0, "content_140e6b7f0", tpl_12963::<0x14387fc34, 0x14387fc48, 0x1401e9770, 0x338, 0x1424b4cb0, 0x88, 0x1401e9590> as *const (), Throws::Discard), // sub_140e6b7f0
    Replacement::draft_throws(0x140e6b860, "content_140e6b860", tpl_17888::<0x14387fc48, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143263b20, 0x1402c4730, 0x88, 0x1432fc7a0> as *const (), Throws::Discard), // sub_140e6b860
    Replacement::draft(0x140e6b940, "content_140e6b940", fn_140e6b940 as *const ()), // sub_140e6b940
    Replacement::draft(0x140e6b9f0, "content_140e6b9f0", tpl_18006::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1c, 0x1432fca00, 0xffffffffffffffed, 0xfffffffffffffff1, 0xfffffffffffffff3, 0xffffffffffffffec> as *const ()), // sub_140e6b9f0
    Replacement::draft(0x140e6bab0, "content_140e6bab0", fn_140e6bab0 as *const ()), // sub_140e6bab0
    Replacement::draft_throws(0x140e6bbc0, "content_140e6bbc0", fn_140e6bbc0 as *const (), Throws::Discard), // sub_140e6bbc0
    Replacement::draft_throws(0x140e6bd10, "content_140e6bd10", fn_140e6bd10 as *const (), Throws::Discard), // sub_140e6bd10
    Replacement::draft_throws(0x140e6be70, "content_140e6be70", fn_140e6be70 as *const (), Throws::Discard), // sub_140e6be70
    Replacement::draft(0x140e6c010, "content_140e6c010", fn_140e6c010 as *const ()), // sub_140e6c010
    Replacement::draft_throws(0x140e6c140, "content_140e6c140", fn_140e6c140 as *const (), Throws::Caught), // sub_140e6c140
    Replacement::draft_throws(0x140e6c220, "content_140e6c220", fn_140e6c220 as *const (), Throws::Caught), // sub_140e6c220
    Replacement::draft_throws(0x140e6c300, "content_140e6c300", fn_140e6c300 as *const (), Throws::Caught), // sub_140e6c300
    Replacement::draft(0x140e6c3e0, "content_140e6c3e0", fn_140e6c3e0 as *const ()), // sub_140e6c3e0
    Replacement::draft(0x140e6c4b0, "content_140e6c4b0", fn_140e6c4b0 as *const ()), // sub_140e6c4b0
    Replacement::draft(0x140e6c600, "content_140e6c600", fn_140e6c600 as *const ()), // sub_140e6c600
    Replacement::draft_throws(0x140e6c760, "content_140e6c760", fn_140e6c760 as *const (), Throws::Discard), // sub_140e6c760
    Replacement::draft_throws(0x140e6c890, "content_140e6c890", fn_140e6c890 as *const (), Throws::Discard), // sub_140e6c890
    Replacement::draft(0x140e6c960, "content_140e6c960", fn_140e6c960 as *const ()), // sub_140e6c960
    Replacement::draft(0x140e6ca70, "content_140e6ca70", fn_140e6ca70 as *const ()), // sub_140e6ca70
    Replacement::draft(0x140e6cbe0, "content_140e6cbe0", fn_140e6cbe0 as *const ()), // sub_140e6cbe0
    Replacement::draft(0x140e6cc70, "content_140e6cc70", fn_140e6cc70 as *const ()), // sub_140e6cc70
    Replacement::draft_throws(0x140e6cdc0, "content_140e6cdc0", fn_140e6cdc0 as *const (), Throws::Discard), // sub_140e6cdc0
    Replacement::draft(0x140e6d040, "content_140e6d040", fn_140e6d040 as *const ()), // sub_140e6d040
    Replacement::draft_throws(0x140e6d320, "content_140e6d320", fn_140e6d320 as *const (), Throws::Discard), // sub_140e6d320
    Replacement::draft(0x140e6d630, "content_140e6d630", fn_140e6d630 as *const ()), // sub_140e6d630
    Replacement::draft_throws(0x140e6d7d0, "content_140e6d7d0", fn_140e6d7d0 as *const (), Throws::Caught), // sub_140e6d7d0
    Replacement::draft_throws(0x140e6d960, "content_140e6d960", fn_140e6d960 as *const (), Throws::Caught), // sub_140e6d960
    Replacement::draft_throws(0x140e6daf0, "content_140e6daf0", fn_140e6daf0 as *const (), Throws::Caught), // sub_140e6daf0
    Replacement::draft(0x140e6dc80, "content_140e6dc80", fn_140e6dc80 as *const ()), // sub_140e6dc80
    Replacement::draft(0x140e6de60, "content_140e6de60", fn_140e6de60 as *const ()), // sub_140e6de60
    Replacement::draft(0x140e6e0a0, "content_140e6e0a0", fn_140e6e0a0 as *const ()), // sub_140e6e0a0
    Replacement::draft_throws(0x140e6e2b0, "content_140e6e2b0", fn_140e6e2b0 as *const (), Throws::Discard), // sub_140e6e2b0
    Replacement::draft_throws(0x140e6e4d0, "content_140e6e4d0", fn_140e6e4d0 as *const (), Throws::Caught), // sub_140e6e4d0
    Replacement::draft(0x140e6e6e0, "content_140e6e6e0", fn_140e6e6e0 as *const ()), // sub_140e6e6e0
    Replacement::draft(0x140e6e7d0, "content_140e6e7d0", fn_140e6e7d0 as *const ()), // sub_140e6e7d0
    Replacement::draft(0x140e6e8f0, "content_140e6e8f0", fn_140e6e8f0 as *const ()), // sub_140e6e8f0
    Replacement::draft(0x140e6e9d0, "content_140e6e9d0", fn_140e6e9d0 as *const ()), // sub_140e6e9d0
    Replacement::draft_throws(0x140e6ec20, "content_140e6ec20", fn_140e6ec20 as *const (), Throws::Discard), // sub_140e6ec20
    Replacement::draft(0x140e6ee70, "content_140e6ee70", fn_140e6ee70 as *const ()), // sub_140e6ee70
    Replacement::draft_throws(0x140e6f000, "content_140e6f000", fn_140e6f000 as *const (), Throws::Discard), // sub_140e6f000
    Replacement::draft(0x140e6f280, "content_140e6f280", fn_140e6f280 as *const ()), // sub_140e6f280
    Replacement::draft_throws(0x140e6f3e0, "content_140e6f3e0", fn_140e6f3e0 as *const (), Throws::Caught), // sub_140e6f3e0
    Replacement::draft_throws(0x140e6f890, "content_140e6f890", fn_140e6f890 as *const (), Throws::Caught), // sub_140e6f890
    Replacement::draft_throws(0x140e6fe20, "content_140e6fe20", fn_140e6fe20 as *const (), Throws::Caught), // sub_140e6fe20
    Replacement::draft(0x140e70380, "content_140e70380", fn_140e70380 as *const ()), // sub_140e70380
    Replacement::draft(0x140e70540, "content_140e70540", fn_140e70540 as *const ()), // sub_140e70540
    Replacement::draft(0x140e706c0, "content_140e706c0", fn_140e706c0 as *const ()), // sub_140e706c0
    Replacement::draft_throws(0x140e70860, "content_140e70860", fn_140e70860 as *const (), Throws::Discard), // sub_140e70860
    Replacement::draft_throws(0x140e709e0, "content_140e709e0", fn_140e709e0 as *const (), Throws::Discard), // sub_140e709e0
    Replacement::draft(0x140e70b80, "content_140e70b80", tpl_1204::<0x1402183c0, 0x20, 0x140e71010, 0x10, 0x140e70f60, 0x1430b6bd0, 0x141834d30> as *const ()), // content::ConversionMetadata::content::ConversionMetadata::vfunc_0
    Replacement::draft(0x140e70be0, "content_140e70be0", fn_140e70be0 as *const ()), // content::DependencyMetadata::content::DependencyMetadata::vfunc_0
    Replacement::draft(0x140e70c70, "content_140e70c70", tpl_18041::<0x3, 0x4, 0x8, 0x14240aa00, 0x108, 0x10c> as *const ()), // sub_140e70c70
    Replacement::draft(0x140e70cd0, "content_140e70cd0", fn_140e70cd0 as *const ()), // sub_140e70cd0
    Replacement::draft(0x140e70de0, "content_140e70de0", fn_140e70de0 as *const ()), // sub_140e70de0
    Replacement::draft(0x140e70ec0, "content_140e70ec0", fn_140e70ec0 as *const ()), // sub_140e70ec0
    Replacement::draft(0x140e70f60, "content_140e70f60", tpl_2030::<0x140e710c0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140e70f60
    Replacement::draft(0x140e710c0, "content_140e710c0", fn_140e710c0 as *const ()), // sub_140e710c0
    Replacement::draft(0x140e71130, "content_140e71130", fn_140e71130 as *const ()), // sub_140e71130
    Replacement::draft_throws(0x140e71300, "content_140e71300", fn_140e71300 as *const (), Throws::Caught), // sub_140e71300
    Replacement::draft_throws(0x140e71550, "content_140e71550", fn_140e71550 as *const (), Throws::Caught), // sub_140e71550
    Replacement::draft_throws(0x140e717a0, "content_140e717a0", fn_140e717a0 as *const (), Throws::Caught), // sub_140e717a0
    Replacement::draft_throws(0x140e719f0, "content_140e719f0", fn_140e719f0 as *const (), Throws::Caught), // sub_140e719f0
    Replacement::draft_throws(0x140e71c40, "content_140e71c40", fn_140e71c40 as *const (), Throws::Caught), // sub_140e71c40
    Replacement::draft_throws(0x140e71e90, "content_140e71e90", fn_140e71e90 as *const (), Throws::Caught), // sub_140e71e90
    Replacement::draft(0x140e720a0, "content_140e720a0", fn_140e720a0 as *const ()), // sub_140e720a0
    Replacement::draft(0x140e721d0, "content_140e721d0", fn_140e721d0 as *const ()), // sub_140e721d0
    Replacement::draft(0x140e72300, "content_140e72300", fn_140e72300 as *const ()), // sub_140e72300
    Replacement::draft(0x140e723d0, "content_140e723d0", fn_140e723d0 as *const ()), // sub_140e723d0
    Replacement::draft(0x140e72530, "content_140e72530", fn_140e72530 as *const ()), // sub_140e72530
    Replacement::draft(0x140e72700, "content_140e72700", fn_140e72700 as *const ()), // sub_140e72700
    Replacement::draft_throws(0x140e72990, "content_140e72990", fn_140e72990 as *const (), Throws::Caught), // sub_140e72990
    Replacement::draft(0x140e72b10, "content_140e72b10", fn_140e72b10 as *const ()), // sub_140e72b10
    Replacement::draft_throws(0x140e72bb0, "content_140e72bb0", fn_140e72bb0 as *const (), Throws::Caught), // sub_140e72bb0
    Replacement::draft_throws(0x140e72d00, "content_140e72d00", fn_140e72d00 as *const (), Throws::Caught), // sub_140e72d00
    Replacement::draft(0x140e72f70, "content_140e72f70", fn_140e72f70 as *const ()), // sub_140e72f70
    Replacement::draft_throws(0x140e73010, "content_140e73010", fn_140e73010 as *const (), Throws::Caught), // sub_140e73010
    Replacement::draft_throws(0x140e732e0, "content_140e732e0", fn_140e732e0 as *const (), Throws::Caught), // sub_140e732e0
    Replacement::draft_throws(0x140e735e0, "content_140e735e0", fn_140e735e0 as *const (), Throws::Caught), // sub_140e735e0
    Replacement::draft_throws(0x140e73870, "content_140e73870", fn_140e73870 as *const (), Throws::Caught), // sub_140e73870
    Replacement::draft_throws(0x140e73b30, "content_140e73b30", fn_140e73b30 as *const (), Throws::Caught), // sub_140e73b30
    Replacement::draft_throws(0x140e73dc0, "content_140e73dc0", fn_140e73dc0 as *const (), Throws::Caught), // sub_140e73dc0
    Replacement::draft(0x140e74080, "content_140e74080", fn_140e74080 as *const ()), // sub_140e74080
    Replacement::draft(0x140e74120, "content_140e74120", fn_140e74120 as *const ()), // sub_140e74120
    Replacement::draft(0x140e74320, "content_140e74320", fn_140e74320 as *const ()), // sub_140e74320
    Replacement::draft_throws(0x140e74590, "content_140e74590", fn_140e74590 as *const (), Throws::Caught), // sub_140e74590
    Replacement::draft_throws(0x140e74660, "content_140e74660", fn_140e74660 as *const (), Throws::Caught), // sub_140e74660
    Replacement::draft_throws(0x140e74750, "content_140e74750", fn_140e74750 as *const (), Throws::Caught), // sub_140e74750
    Replacement::draft_throws(0x140e74de0, "content_140e74de0", fn_140e74de0 as *const (), Throws::Caught), // sub_140e74de0
    Replacement::draft_throws(0x140e75430, "content_140e75430", fn_140e75430 as *const (), Throws::Caught), // sub_140e75430
    Replacement::draft_throws(0x140e75870, "content_140e75870", fn_140e75870 as *const (), Throws::Caught), // sub_140e75870
    Replacement::draft_throws(0x140e75b90, "content_140e75b90", fn_140e75b90 as *const (), Throws::Caught), // sub_140e75b90
    Replacement::draft_throws(0x140e75eb0, "content_140e75eb0", fn_140e75eb0 as *const (), Throws::Caught), // sub_140e75eb0
    Replacement::draft_throws(0x140e76350, "content_140e76350", fn_140e76350 as *const (), Throws::Caught), // sub_140e76350
    Replacement::draft_throws(0x140e767c0, "content_140e767c0", fn_140e767c0 as *const (), Throws::Caught), // sub_140e767c0
    Replacement::draft_throws(0x140e76c20, "content_140e76c20", fn_140e76c20 as *const (), Throws::Caught), // sub_140e76c20
    Replacement::draft(0x140e76cf0, "content_140e76cf0", fn_140e76cf0 as *const ()), // sub_140e76cf0
    Replacement::draft(0x140e76da0, "content_140e76da0", tpl_18072::<0x8, 0x14240ab10> as *const ()), // sub_140e76da0
    Replacement::draft_throws(0x140e76de0, "content_140e76de0", fn_140e76de0 as *const (), Throws::Caught), // sub_140e76de0
    Replacement::draft(0x140e76fb0, "content_140e76fb0", fn_140e76fb0 as *const ()), // sub_140e76fb0
    Replacement::draft(0x140e77000, "content_140e77000", fn_140e77000 as *const ()), // sub_140e77000
    Replacement::draft(0x140e771b0, "content_140e771b0", fn_140e771b0 as *const ()), // sub_140e771b0
    Replacement::draft_throws(0x140e773d0, "content_140e773d0", fn_140e773d0 as *const (), Throws::Caught), // sub_140e773d0
    Replacement::draft_throws(0x140e774d0, "content_140e774d0", fn_140e774d0 as *const (), Throws::Caught), // sub_140e774d0
    Replacement::draft_throws(0x140e77700, "content_140e77700", fn_140e77700 as *const (), Throws::Caught), // sub_140e77700
    Replacement::draft_throws(0x140e77840, "content_140e77840", fn_140e77840 as *const (), Throws::Caught), // sub_140e77840
    Replacement::draft_throws(0x140e77e20, "content_140e77e20", fn_140e77e20 as *const (), Throws::Caught), // sub_140e77e20
    Replacement::draft_throws(0x140e782f0, "content_140e782f0", fn_140e782f0 as *const (), Throws::Caught), // sub_140e782f0
    Replacement::draft_throws(0x140e78a30, "content_140e78a30", fn_140e78a30 as *const (), Throws::Caught), // sub_140e78a30
    Replacement::draft_throws(0x140e78ec0, "content_140e78ec0", fn_140e78ec0 as *const (), Throws::Caught), // sub_140e78ec0
    Replacement::draft_throws(0x140e79570, "content_140e79570", fn_140e79570 as *const (), Throws::Caught), // sub_140e79570
    Replacement::draft(0x140e79a00, "content_140e79a00", tpl_9423::<0x143881ec0, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e79a00
    Replacement::draft_throws(0x140e79aa0, "content_140e79aa0", tpl_9424::<0x143881ec8, 0x143881ec0, 0x1401e9fa0, 0x328, 0x338, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140e79aa0
    Replacement::draft(0x140e79b20, "content_140e79b20", tpl_9428::<0x143881ec0, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e79b20
    Replacement::draft(0x140e79b90, "content_140e79b90", tpl_9425::<0x143881ec0, 0x14243b0b0, 0x338, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e79b90
    Replacement::draft_throws(0x140e79c00, "content_140e79c00", tpl_9426::<0x143881ec8, 0x143881ec0, 0x1401e9b60, 0x328, 0x338, 0x120, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140e79c00
    Replacement::draft(0x140e79c90, "content_140e79c90", tpl_9416::<0x143881ec0, 0x14243b0b0, 0x338, 0x88, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e79c90
    Replacement::draft_throws(0x140e79d00, "content_140e79d00", tpl_9429::<0x143881ec8, 0x143881ec0, 0x1401e9770, 0x338, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140e79d00
    Replacement::draft_throws(0x140e79d70, "content_140e79d70", fn_140e79d70 as *const (), Throws::Caught), // sub_140e79d70
    Replacement::draft_throws(0x140e7a240, "content_140e7a240", fn_140e7a240 as *const (), Throws::Caught), // sub_140e7a240
    Replacement::draft_throws(0x140e7a6d0, "content_140e7a6d0", fn_140e7a6d0 as *const (), Throws::Caught), // sub_140e7a6d0
    Replacement::draft_throws(0x140e7ab60, "content_140e7ab60", fn_140e7ab60 as *const (), Throws::Caught), // sub_140e7ab60
    Replacement::draft_throws(0x140e7ac60, "content_140e7ac60", fn_140e7ac60 as *const (), Throws::Caught), // sub_140e7ac60
    Replacement::draft_throws(0x140e7ad20, "content_140e7ad20", fn_140e7ad20 as *const (), Throws::Caught), // sub_140e7ad20
    Replacement::draft_throws(0x140e7ade0, "content_140e7ade0", fn_140e7ade0 as *const (), Throws::Caught), // sub_140e7ade0
    Replacement::draft_throws(0x140e7aeb0, "content_140e7aeb0", fn_140e7aeb0 as *const (), Throws::Caught), // sub_140e7aeb0
    Replacement::draft_throws(0x140e7af80, "content_140e7af80", fn_140e7af80 as *const (), Throws::Caught), // sub_140e7af80
    Replacement::draft_throws(0x140e7b050, "content_140e7b050", tpl_9430::<0x143881ec0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432fc630, 0x1402c4aa0> as *const (), Throws::Discard), // sub_140e7b050
    Replacement::draft_throws(0x140e7b100, "content_140e7b100", fn_140e7b100 as *const (), Throws::Caught), // sub_140e7b100
    Replacement::draft_throws(0x140e7b380, "content_140e7b380", fn_140e7b380 as *const (), Throws::Caught), // sub_140e7b380
    Replacement::draft_throws(0x140e7b600, "content_140e7b600", fn_140e7b600 as *const (), Throws::Caught), // sub_140e7b600
    Replacement::draft_throws(0x140e7b880, "content_140e7b880", fn_140e7b880 as *const (), Throws::Caught), // sub_140e7b880
    Replacement::draft(0x140e7baf0, "content_140e7baf0", tpl_18041::<0x3, 0x4, 0x8, 0x14240a730, 0x108, 0x10c> as *const ()), // sub_140e7baf0
    Replacement::draft_throws(0x140e7bb50, "content_140e7bb50", fn_140e7bb50 as *const (), Throws::Caught), // sub_140e7bb50
    Replacement::draft_throws(0x140e7c120, "content_140e7c120", fn_140e7c120 as *const (), Throws::Caught), // sub_140e7c120
    Replacement::draft_throws(0x140e7c6c0, "content_140e7c6c0", fn_140e7c6c0 as *const (), Throws::Caught), // sub_140e7c6c0
    Replacement::draft(0x140e7ca30, "content_140e7ca30", tpl_2003::<0x1432fd890, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::AreaLightComponent::content::AreaLightComponent::vfunc_18
    Replacement::draft(0x140e7cb40, "content_140e7cb40", tpl_11330::<0x1432fd860, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::AttachmentBoneComponent::content::AttachmentBoneComponent::vfunc_18
    Replacement::draft(0x140e7cc50, "content_140e7cc50", tpl_11357::<0x1432fd838, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::BlockerComponent::content::BlockerComponent::vfunc_18
    Replacement::draft(0x140e7cd50, "content_140e7cd50", fn_140e7cd50 as *const ()), // sub_140e7cd50
    Replacement::draft(0x140e7cfc0, "content_140e7cfc0", tpl_4773::<0x1432fd948, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::CollapsedLodComponent::content::CollapsedLodComponent::vfunc_18
    Replacement::draft(0x140e7d160, "content_140e7d160", fn_140e7d160 as *const ()), // content::DeprecatedComponent::content::DeprecatedComponent::vfunc_10
    Replacement::draft(0x140e7d1c0, "content_140e7d1c0", fn_140e7d1c0 as *const ()), // content::DeprecatedComponent::content::DeprecatedComponent::vfunc_4
    Replacement::draft(0x140e7d260, "content_140e7d260", fn_140e7d260 as *const ()), // content::JumpPointComponent::content::JumpPointComponent::vfunc_4
    Replacement::draft(0x140e7d290, "content_140e7d290", tpl_11330::<0x1432fd748, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::JumpPointComponent::content::JumpPointComponent::vfunc_18
    Replacement::draft(0x140e7d3e0, "content_140e7d3e0", fn_140e7d3e0 as *const ()), // content::OBBVolumeComponent::content::OBBVolumeComponent::vfunc_10
    Replacement::draft(0x140e7d430, "content_140e7d430", tpl_2003::<0x1432fd700, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::OBBVolumeComponent::content::OBBVolumeComponent::vfunc_18
    Replacement::draft(0x140e7d540, "content_140e7d540", fn_140e7d540 as *const ()), // content::QuestNodeComponent::content::QuestNodeComponent::vfunc_18
    Replacement::draft(0x140e7d7b0, "content_140e7d7b0", fn_140e7d7b0 as *const ()), // content::SplineComponent::content::SplineComponent::vfunc_18
    Replacement::draft(0x140e7db70, "content_140e7db70", tpl_3541::<0x1432fd540, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::SplinePointComponent::content::SplinePointComponent::vfunc_18
    Replacement::draft(0x140e7dc90, "content_140e7dc90", tpl_11336::<0x1432fd4d8, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::StreamingGraphNodeComponent::content::StreamingGraphNodeComponent::vfunc_18
    Replacement::draft(0x140e7dda0, "content_140e7dda0", fn_140e7dda0 as *const ()), // content::StreamingVolumeComponent::content::StreamingVolumeComponent::vfunc_18
    Replacement::draft(0x140e7df60, "content_140e7df60", tpl_16424::<0x1432fd648, 0x142409b50, 0x8, 0xc, 0x1, 0x1401f22d0, 0x3, 0x4, 0x14240a730, 0x14240ab10> as *const ()), // content::TriggerOldStyleScriptComponent::content::TriggerOldStyleScriptComponent::vfunc_18
    Replacement::draft(0x140e7e070, "content_140e7e070", fn_140e7e070 as *const ()), // content::WhiteboxComponent::content::WhiteboxComponent::vfunc_18
    Replacement::draft(0x140e7e1b0, "content_140e7e1b0", fn_140e7e1b0 as *const ()), // sub_140e7e1b0
    Replacement::draft(0x140e7e260, "content_140e7e260", fn_140e7e260 as *const ()), // sub_140e7e260
    Replacement::draft(0x140e7e390, "content_140e7e390", fn_140e7e390 as *const ()), // sub_140e7e390
    Replacement::draft_throws(0x140e7e460, "content_140e7e460", fn_140e7e460 as *const (), Throws::Discard), // sub_140e7e460
    Replacement::draft(0x140e7e550, "content_140e7e550", tpl_12375::<0x14387fd78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0x50, 0x80, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e7e550
    Replacement::draft_throws(0x140e7e5e0, "content_140e7e5e0", fn_140e7e5e0 as *const (), Throws::Discard), // sub_140e7e5e0
    Replacement::draft_throws(0x140e7e6d0, "content_140e7e6d0", fn_140e7e6d0 as *const (), Throws::Caught), // sub_140e7e6d0
    Replacement::draft_throws(0x140e7e7c0, "content_140e7e7c0", fn_140e7e7c0 as *const (), Throws::Caught), // sub_140e7e7c0
    Replacement::draft_throws(0x140e7e8b0, "content_140e7e8b0", fn_140e7e8b0 as *const (), Throws::Caught), // sub_140e7e8b0
    Replacement::draft(0x140e7eb10, "content_140e7eb10", fn_140e7eb10 as *const ()), // sub_140e7eb10
    Replacement::draft(0x140e7ebb0, "content_140e7ebb0", tpl_70858::<0x14387fcc8, 0x14387fd78, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x50, 0x120, 0x1401e9860> as *const ()), // sub_140e7ebb0
    Replacement::draft(0x140e7ec30, "content_140e7ec30", tpl_12375::<0x14387fd78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x50, 0x120, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e7ec30
    Replacement::draft_throws(0x140e7ecc0, "content_140e7ecc0", fn_140e7ecc0 as *const (), Throws::Discard), // sub_140e7ecc0
    Replacement::draft_throws(0x140e7ed40, "content_140e7ed40", fn_140e7ed40 as *const (), Throws::Discard), // sub_140e7ed40
    Replacement::draft(0x140e7ee70, "content_140e7ee70", fn_140e7ee70 as *const ()), // sub_140e7ee70
    Replacement::draft(0x140e7ef20, "content_140e7ef20", fn_140e7ef20 as *const ()), // sub_140e7ef20
    Replacement::draft(0x140e7efe0, "content_140e7efe0", tpl_6748::<0x14387fd38, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e7efe0
    Replacement::draft(0x140e7f050, "content_140e7f050", fn_140e7f050 as *const ()), // sub_140e7f050
    Replacement::draft_throws(0x140e7f1c0, "content_140e7f1c0", fn_140e7f1c0 as *const (), Throws::Discard), // sub_140e7f1c0
    Replacement::draft(0x140e7f340, "content_140e7f340", fn_140e7f340 as *const ()), // sub_140e7f340
    Replacement::draft_throws(0x140e7f420, "content_140e7f420", fn_140e7f420 as *const (), Throws::Discard), // sub_140e7f420
    Replacement::draft(0x140e7f5b0, "content_140e7f5b0", tpl_18119::<0x8, 0xe, 0xc, 0xd, 0xf, 0x14240aa00> as *const ()), // sub_140e7f5b0
    Replacement::draft_throws(0x140e7f620, "content_140e7f620", fn_140e7f620 as *const (), Throws::Caught), // sub_140e7f620
    Replacement::draft_throws(0x140e7fad0, "content_140e7fad0", fn_140e7fad0 as *const (), Throws::Caught), // sub_140e7fad0
    Replacement::draft_throws(0x140e7ff00, "content_140e7ff00", fn_140e7ff00 as *const (), Throws::Caught), // sub_140e7ff00
    Replacement::draft(0x140e80320, "content_140e80320", fn_140e80320 as *const ()), // sub_140e80320
    Replacement::draft(0x140e803c0, "content_140e803c0", fn_140e803c0 as *const ()), // sub_140e803c0
    Replacement::draft(0x140e804a0, "content_140e804a0", fn_140e804a0 as *const ()), // sub_140e804a0
    Replacement::draft_throws(0x140e80590, "content_140e80590", fn_140e80590 as *const (), Throws::Discard), // sub_140e80590
    Replacement::draft_throws(0x140e80680, "content_140e80680", fn_140e80680 as *const (), Throws::Discard), // sub_140e80680
    Replacement::draft(0x140e808a0, "content_140e808a0", fn_140e808a0 as *const ()), // sub_140e808a0
    Replacement::draft(0x140e809b0, "content_140e809b0", fn_140e809b0 as *const ()), // sub_140e809b0
    Replacement::draft(0x140e80b10, "content_140e80b10", fn_140e80b10 as *const ()), // sub_140e80b10
    Replacement::draft(0x140e80bd0, "content_140e80bd0", fn_140e80bd0 as *const ()), // sub_140e80bd0
    Replacement::draft_throws(0x140e80dc0, "content_140e80dc0", fn_140e80dc0 as *const (), Throws::Discard), // sub_140e80dc0
    Replacement::draft(0x140e80fc0, "content_140e80fc0", fn_140e80fc0 as *const ()), // sub_140e80fc0
    Replacement::draft_throws(0x140e81110, "content_140e81110", fn_140e81110 as *const (), Throws::Discard), // sub_140e81110
    Replacement::draft(0x140e81330, "content_140e81330", fn_140e81330 as *const ()), // sub_140e81330
    Replacement::draft_throws(0x140e813f0, "content_140e813f0", fn_140e813f0 as *const (), Throws::Caught), // sub_140e813f0
    Replacement::draft_throws(0x140e81780, "content_140e81780", fn_140e81780 as *const (), Throws::Caught), // sub_140e81780
    Replacement::draft_throws(0x140e81ad0, "content_140e81ad0", fn_140e81ad0 as *const (), Throws::Caught), // sub_140e81ad0
    Replacement::draft(0x140e81e30, "content_140e81e30", fn_140e81e30 as *const ()), // sub_140e81e30
    Replacement::draft(0x140e81f90, "content_140e81f90", fn_140e81f90 as *const ()), // sub_140e81f90
    Replacement::draft(0x140e82110, "content_140e82110", fn_140e82110 as *const ()), // sub_140e82110
    Replacement::draft_throws(0x140e822a0, "content_140e822a0", fn_140e822a0 as *const (), Throws::Discard), // sub_140e822a0
    Replacement::draft_throws(0x140e823e0, "content_140e823e0", fn_140e823e0 as *const (), Throws::Caught), // sub_140e823e0
    Replacement::draft(0x140e82690, "content_140e82690", fn_140e82690 as *const ()), // sub_140e82690
    Replacement::draft(0x140e82770, "content_140e82770", fn_140e82770 as *const ()), // sub_140e82770
    Replacement::draft(0x140e82810, "content_140e82810", fn_140e82810 as *const ()), // sub_140e82810
    Replacement::draft(0x140e828c0, "content_140e828c0", tpl_6766::<0x14387fd88, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e828c0
    Replacement::draft(0x140e82930, "content_140e82930", fn_140e82930 as *const ()), // sub_140e82930
    Replacement::draft_throws(0x140e82ad0, "content_140e82ad0", fn_140e82ad0 as *const (), Throws::Discard), // sub_140e82ad0
    Replacement::draft(0x140e82c40, "content_140e82c40", fn_140e82c40 as *const ()), // sub_140e82c40
    Replacement::draft_throws(0x140e82da0, "content_140e82da0", fn_140e82da0 as *const (), Throws::Discard), // sub_140e82da0
    Replacement::draft(0x140e82f60, "content_140e82f60", fn_140e82f60 as *const ()), // sub_140e82f60
    Replacement::draft_throws(0x140e83050, "content_140e83050", fn_140e83050 as *const (), Throws::Caught), // sub_140e83050
    Replacement::draft_throws(0x140e83160, "content_140e83160", fn_140e83160 as *const (), Throws::Caught), // sub_140e83160
    Replacement::draft_throws(0x140e83270, "content_140e83270", fn_140e83270 as *const (), Throws::Caught), // sub_140e83270
    Replacement::draft(0x140e83380, "content_140e83380", fn_140e83380 as *const ()), // sub_140e83380
    Replacement::draft(0x140e83480, "content_140e83480", fn_140e83480 as *const ()), // sub_140e83480
    Replacement::draft(0x140e835a0, "content_140e835a0", fn_140e835a0 as *const ()), // sub_140e835a0
    Replacement::draft_throws(0x140e836c0, "content_140e836c0", fn_140e836c0 as *const (), Throws::Discard), // sub_140e836c0
    Replacement::draft_throws(0x140e837e0, "content_140e837e0", fn_140e837e0 as *const (), Throws::Discard), // sub_140e837e0
    Replacement::draft(0x140e83910, "content_140e83910", fn_140e83910 as *const ()), // sub_140e83910
    Replacement::draft(0x140e839f0, "content_140e839f0", fn_140e839f0 as *const ()), // sub_140e839f0
    Replacement::draft(0x140e83a70, "content_140e83a70", tpl_6708::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x143302570, 0x28, 0x20> as *const ()), // sub_140e83a70
    Replacement::draft(0x140e83b10, "content_140e83b10", tpl_6709::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x20, 0x143302570, 0xffffffffffffffe8, 0xfffffffffffffff8, 0x9, 0xd> as *const ()), // sub_140e83b10
    Replacement::draft(0x140e83bc0, "content_140e83bc0", tpl_12060::<0x14387fd30, 0x14243b0b0, 0x1406e01b0, 0x140e97a60, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e83bc0
    Replacement::draft(0x140e83c30, "content_140e83c30", fn_140e83c30 as *const ()), // sub_140e83c30
    Replacement::draft_throws(0x140e83dc0, "content_140e83dc0", fn_140e83dc0 as *const (), Throws::Discard), // sub_140e83dc0
    Replacement::draft(0x140e83f40, "content_140e83f40", fn_140e83f40 as *const ()), // sub_140e83f40
    Replacement::draft_throws(0x140e84020, "content_140e84020", fn_140e84020 as *const (), Throws::Discard), // sub_140e84020
    Replacement::draft(0x140e841c0, "content_140e841c0", tpl_18119::<0x8, 0xc, 0xd, 0xe, 0xf, 0x1404b9990> as *const ()), // sub_140e841c0
    Replacement::draft_throws(0x140e84230, "content_140e84230", fn_140e84230 as *const (), Throws::Caught), // sub_140e84230
    Replacement::draft_throws(0x140e84550, "content_140e84550", fn_140e84550 as *const (), Throws::Caught), // sub_140e84550
    Replacement::draft_throws(0x140e84840, "content_140e84840", fn_140e84840 as *const (), Throws::Caught), // sub_140e84840
    Replacement::draft(0x140e84b30, "content_140e84b30", fn_140e84b30 as *const ()), // sub_140e84b30
    Replacement::draft(0x140e84bd0, "content_140e84bd0", fn_140e84bd0 as *const ()), // sub_140e84bd0
    Replacement::draft(0x140e84cb0, "content_140e84cb0", fn_140e84cb0 as *const ()), // sub_140e84cb0
    Replacement::draft_throws(0x140e84da0, "content_140e84da0", fn_140e84da0 as *const (), Throws::Discard), // sub_140e84da0
    Replacement::draft_throws(0x140e84ea0, "content_140e84ea0", fn_140e84ea0 as *const (), Throws::Caught), // sub_140e84ea0
    Replacement::draft(0x140e850f0, "content_140e850f0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143302e50> as *const ()), // sub_140e850f0
    Replacement::draft(0x140e85150, "content_140e85150", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143302e50> as *const ()), // sub_140e85150
    Replacement::draft_throws(0x140e85250, "content_140e85250", fn_140e85250 as *const (), Throws::Caught), // sub_140e85250
    Replacement::draft_throws(0x140e852e0, "content_140e852e0", tpl_6763::<0x14387fd58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140e852e0
    Replacement::draft(0x140e85390, "content_140e85390", fn_140e85390 as *const ()), // sub_140e85390
    Replacement::draft(0x140e85490, "content_140e85490", fn_140e85490 as *const ()), // sub_140e85490
    Replacement::draft(0x140e856b0, "content_140e856b0", fn_140e856b0 as *const ()), // sub_140e856b0
    Replacement::draft_throws(0x140e85850, "content_140e85850", fn_140e85850 as *const (), Throws::Discard), // sub_140e85850
    Replacement::draft(0x140e859d0, "content_140e859d0", tpl_18168::<0x14387fc90, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0x1a0, 0x48, 0x90, 0x118, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140e859d0
    Replacement::draft_throws(0x140e85a80, "content_140e85a80", fn_140e85a80 as *const (), Throws::Discard), // sub_140e85a80
    Replacement::draft(0x140e85c00, "content_140e85c00", fn_140e85c00 as *const ()), // sub_140e85c00
    Replacement::draft_throws(0x140e85c80, "content_140e85c80", fn_140e85c80 as *const (), Throws::Caught), // sub_140e85c80
    Replacement::draft_throws(0x140e86240, "content_140e86240", fn_140e86240 as *const (), Throws::Caught), // sub_140e86240
    Replacement::draft_throws(0x140e86770, "content_140e86770", fn_140e86770 as *const (), Throws::Caught), // sub_140e86770
    Replacement::draft(0x140e86c70, "content_140e86c70", fn_140e86c70 as *const ()), // sub_140e86c70
    Replacement::draft(0x140e86d60, "content_140e86d60", fn_140e86d60 as *const ()), // sub_140e86d60
    Replacement::draft(0x140e86e10, "content_140e86e10", tpl_18168::<0x14387fc90, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x1a0, 0xe8, 0x90, 0x118, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140e86e10
    Replacement::draft_throws(0x140e86ed0, "content_140e86ed0", fn_140e86ed0 as *const (), Throws::Discard), // sub_140e86ed0
    Replacement::draft_throws(0x140e86f80, "content_140e86f80", fn_140e86f80 as *const (), Throws::Discard), // sub_140e86f80
    Replacement::draft(0x140e87120, "content_140e87120", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143300c48> as *const ()), // sub_140e87120
    Replacement::draft(0x140e87180, "content_140e87180", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143300c48> as *const ()), // sub_140e87180
    Replacement::draft_throws(0x140e87280, "content_140e87280", fn_140e87280 as *const (), Throws::Caught), // sub_140e87280
    Replacement::draft_throws(0x140e87310, "content_140e87310", tpl_6762::<0x14387fca0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140e87310
    Replacement::draft(0x140e873c0, "content_140e873c0", fn_140e873c0 as *const ()), // sub_140e873c0
    Replacement::draft(0x140e874a0, "content_140e874a0", fn_140e874a0 as *const ()), // sub_140e874a0
    Replacement::draft(0x140e87570, "content_140e87570", fn_140e87570 as *const ()), // sub_140e87570
    Replacement::draft(0x140e87930, "content_140e87930", fn_140e87930 as *const ()), // sub_140e87930
    Replacement::draft_throws(0x140e87d50, "content_140e87d50", fn_140e87d50 as *const (), Throws::Discard), // sub_140e87d50
    Replacement::draft(0x140e87fe0, "content_140e87fe0", fn_140e87fe0 as *const ()), // sub_140e87fe0
    Replacement::draft_throws(0x140e882b0, "content_140e882b0", fn_140e882b0 as *const (), Throws::Discard), // sub_140e882b0
    Replacement::draft(0x140e88650, "content_140e88650", fn_140e88650 as *const ()), // sub_140e88650
    Replacement::draft_throws(0x140e886d0, "content_140e886d0", fn_140e886d0 as *const (), Throws::Caught), // sub_140e886d0
    Replacement::draft_throws(0x140e88890, "content_140e88890", fn_140e88890 as *const (), Throws::Caught), // sub_140e88890
    Replacement::draft_throws(0x140e88a50, "content_140e88a50", fn_140e88a50 as *const (), Throws::Caught), // sub_140e88a50
    Replacement::draft(0x140e88c10, "content_140e88c10", fn_140e88c10 as *const ()), // sub_140e88c10
    Replacement::draft(0x140e88e90, "content_140e88e90", fn_140e88e90 as *const ()), // sub_140e88e90
    Replacement::draft(0x140e89140, "content_140e89140", fn_140e89140 as *const ()), // sub_140e89140
    Replacement::draft_throws(0x140e89410, "content_140e89410", fn_140e89410 as *const (), Throws::Discard), // sub_140e89410
    Replacement::draft_throws(0x140e896a0, "content_140e896a0", fn_140e896a0 as *const (), Throws::Discard), // sub_140e896a0
    Replacement::draft(0x140e89880, "content_140e89880", fn_140e89880 as *const ()), // sub_140e89880
    Replacement::draft(0x140e89b20, "content_140e89b20", fn_140e89b20 as *const ()), // sub_140e89b20
    Replacement::draft(0x140e89dc0, "content_140e89dc0", fn_140e89dc0 as *const ()), // sub_140e89dc0
    Replacement::draft(0x140e89e80, "content_140e89e80", fn_140e89e80 as *const ()), // sub_140e89e80
    Replacement::draft(0x140e89fa0, "content_140e89fa0", fn_140e89fa0 as *const ()), // sub_140e89fa0
    Replacement::draft_throws(0x140e8a180, "content_140e8a180", fn_140e8a180 as *const (), Throws::Discard), // sub_140e8a180
    Replacement::draft(0x140e8a320, "content_140e8a320", fn_140e8a320 as *const ()), // sub_140e8a320
    Replacement::draft_throws(0x140e8a410, "content_140e8a410", fn_140e8a410 as *const (), Throws::Discard), // sub_140e8a410
    Replacement::draft(0x140e8a590, "content_140e8a590", fn_140e8a590 as *const ()), // sub_140e8a590
    Replacement::draft_throws(0x140e8a640, "content_140e8a640", fn_140e8a640 as *const (), Throws::Caught), // sub_140e8a640
    Replacement::draft_throws(0x140e8a720, "content_140e8a720", fn_140e8a720 as *const (), Throws::Caught), // sub_140e8a720
    Replacement::draft_throws(0x140e8a800, "content_140e8a800", fn_140e8a800 as *const (), Throws::Caught), // sub_140e8a800
    Replacement::draft(0x140e8a8e0, "content_140e8a8e0", fn_140e8a8e0 as *const ()), // sub_140e8a8e0
    Replacement::draft(0x140e8a9e0, "content_140e8a9e0", fn_140e8a9e0 as *const ()), // sub_140e8a9e0
    Replacement::draft(0x140e8aa90, "content_140e8aa90", fn_140e8aa90 as *const ()), // sub_140e8aa90
    Replacement::draft_throws(0x140e8ab60, "content_140e8ab60", fn_140e8ab60 as *const (), Throws::Discard), // sub_140e8ab60
    Replacement::draft_throws(0x140e8ac10, "content_140e8ac10", fn_140e8ac10 as *const (), Throws::Caught), // sub_140e8ac10
    Replacement::draft(0x140e8ae00, "content_140e8ae00", fn_140e8ae00 as *const ()), // sub_140e8ae00
    Replacement::draft(0x140e8aec0, "content_140e8aec0", fn_140e8aec0 as *const ()), // sub_140e8aec0
    Replacement::draft(0x140e8af90, "content_140e8af90", tpl_6748::<0x14387fd90, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e8af90
    Replacement::draft(0x140e8b000, "content_140e8b000", fn_140e8b000 as *const ()), // sub_140e8b000
    Replacement::draft_throws(0x140e8b130, "content_140e8b130", fn_140e8b130 as *const (), Throws::Discard), // sub_140e8b130
    Replacement::draft(0x140e8b2d0, "content_140e8b2d0", fn_140e8b2d0 as *const ()), // sub_140e8b2d0
    Replacement::draft_throws(0x140e8b3c0, "content_140e8b3c0", fn_140e8b3c0 as *const (), Throws::Discard), // sub_140e8b3c0
    Replacement::draft(0x140e8b520, "content_140e8b520", fn_140e8b520 as *const ()), // sub_140e8b520
    Replacement::draft_throws(0x140e8b5a0, "content_140e8b5a0", fn_140e8b5a0 as *const (), Throws::Caught), // sub_140e8b5a0
    Replacement::draft_throws(0x140e8b990, "content_140e8b990", fn_140e8b990 as *const (), Throws::Caught), // sub_140e8b990
    Replacement::draft_throws(0x140e8bd50, "content_140e8bd50", fn_140e8bd50 as *const (), Throws::Caught), // sub_140e8bd50
    Replacement::draft(0x140e8bed0, "content_140e8bed0", fn_140e8bed0 as *const ()), // sub_140e8bed0
    Replacement::draft(0x140e8bf90, "content_140e8bf90", fn_140e8bf90 as *const ()), // sub_140e8bf90
    Replacement::draft(0x140e8c050, "content_140e8c050", fn_140e8c050 as *const ()), // sub_140e8c050
    Replacement::draft_throws(0x140e8c120, "content_140e8c120", fn_140e8c120 as *const (), Throws::Discard), // sub_140e8c120
    Replacement::draft_throws(0x140e8c200, "content_140e8c200", fn_140e8c200 as *const (), Throws::Discard), // sub_140e8c200
    Replacement::draft(0x140e8c3d0, "content_140e8c3d0", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143302820> as *const ()), // sub_140e8c3d0
    Replacement::draft(0x140e8c430, "content_140e8c430", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143302820> as *const ()), // sub_140e8c430
    Replacement::draft_throws(0x140e8c530, "content_140e8c530", fn_140e8c530 as *const (), Throws::Caught), // sub_140e8c530
    Replacement::draft_throws(0x140e8c5c0, "content_140e8c5c0", tpl_6762::<0x14387fca8, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140e8c5c0
    Replacement::draft(0x140e8c670, "content_140e8c670", tpl_11457::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x1433013d0, 0x8> as *const ()), // sub_140e8c670
    Replacement::draft(0x140e8c700, "content_140e8c700", tpl_11458::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433013d0, 0xfffffffffffffff0, 0xfffffffffffffff8, 0x18> as *const ()), // sub_140e8c700
    Replacement::draft(0x140e8c820, "content_140e8c820", tpl_11459::<0x14387fc78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e8c820
    Replacement::draft_throws(0x140e8c8d0, "content_140e8c8d0", fn_140e8c8d0 as *const (), Throws::Discard), // sub_140e8c8d0
    Replacement::draft(0x140e8c990, "content_140e8c990", tpl_11461::<0x14387fc78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x38, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e8c990
    Replacement::draft_throws(0x140e8ca30, "content_140e8ca30", fn_140e8ca30 as *const (), Throws::Discard), // sub_140e8ca30
    Replacement::draft_throws(0x140e8cae0, "content_140e8cae0", fn_140e8cae0 as *const (), Throws::Caught), // sub_140e8cae0
    Replacement::draft_throws(0x140e8ccf0, "content_140e8ccf0", fn_140e8ccf0 as *const (), Throws::Caught), // sub_140e8ccf0
    Replacement::draft_throws(0x140e8ced0, "content_140e8ced0", fn_140e8ced0 as *const (), Throws::Caught), // sub_140e8ced0
    Replacement::draft(0x140e8d0a0, "content_140e8d0a0", fn_140e8d0a0 as *const ()), // sub_140e8d0a0
    Replacement::draft(0x140e8d120, "content_140e8d120", tpl_11467::<0x14387fc9c, 0x14387fc78, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xd8, 0xc, 0x10, 0x1401e9860> as *const ()), // sub_140e8d120
    Replacement::draft(0x140e8d1c0, "content_140e8d1c0", tpl_11461::<0x14387fc78, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xd8, 0xc, 0x10, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e8d1c0
    Replacement::draft_throws(0x140e8d270, "content_140e8d270", tpl_11468::<0x14387fc9c, 0x14387fc78, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x120, 0x10, 0x1401e9590> as *const (), Throws::Discard), // sub_140e8d270
    Replacement::draft_throws(0x140e8d2f0, "content_140e8d2f0", fn_140e8d2f0 as *const (), Throws::Discard), // sub_140e8d2f0
    Replacement::draft_throws(0x140e8d3f0, "content_140e8d3f0", tpl_14580::<0x1434268d0, 0x14329e980, 0x8, 0x1426cdcb0, 0x1432fdd70, 0x1402c4e80> as *const (), Throws::Discard), // sub_140e8d3f0
    Replacement::draft(0x140e8d460, "content_140e8d460", fn_140e8d460 as *const ()), // sub_140e8d460
    Replacement::draft(0x140e8d510, "content_140e8d510", fn_140e8d510 as *const ()), // sub_140e8d510
    Replacement::draft(0x140e8d5e0, "content_140e8d5e0", tpl_14861::<0x14387fce0, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e8d5e0
    Replacement::draft(0x140e8d650, "content_140e8d650", fn_140e8d650 as *const ()), // sub_140e8d650
    Replacement::draft_throws(0x140e8d740, "content_140e8d740", fn_140e8d740 as *const (), Throws::Discard), // sub_140e8d740
    Replacement::draft(0x140e8d940, "content_140e8d940", fn_140e8d940 as *const ()), // sub_140e8d940
    Replacement::draft_throws(0x140e8daa0, "content_140e8daa0", fn_140e8daa0 as *const (), Throws::Discard), // sub_140e8daa0
    Replacement::draft(0x140e8dbb0, "content_140e8dbb0", fn_140e8dbb0 as *const ()), // sub_140e8dbb0
    Replacement::draft_throws(0x140e8dc80, "content_140e8dc80", fn_140e8dc80 as *const (), Throws::Caught), // sub_140e8dc80
    Replacement::draft_throws(0x140e8ddf0, "content_140e8ddf0", fn_140e8ddf0 as *const (), Throws::Caught), // sub_140e8ddf0
    Replacement::draft_throws(0x140e8df60, "content_140e8df60", fn_140e8df60 as *const (), Throws::Caught), // sub_140e8df60
    Replacement::draft(0x140e8e0d0, "content_140e8e0d0", fn_140e8e0d0 as *const ()), // sub_140e8e0d0
    Replacement::draft(0x140e8e200, "content_140e8e200", fn_140e8e200 as *const ()), // sub_140e8e200
    Replacement::draft(0x140e8e330, "content_140e8e330", fn_140e8e330 as *const ()), // sub_140e8e330
    Replacement::draft_throws(0x140e8e470, "content_140e8e470", fn_140e8e470 as *const (), Throws::Discard), // sub_140e8e470
    Replacement::draft_throws(0x140e8e5e0, "content_140e8e5e0", fn_140e8e5e0 as *const (), Throws::Discard), // sub_140e8e5e0
    Replacement::draft(0x140e8e780, "content_140e8e780", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1433010b8, 0x8> as *const ()), // sub_140e8e780
    Replacement::draft(0x140e8e800, "content_140e8e800", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433010b8> as *const ()), // sub_140e8e800
    Replacement::draft(0x140e8e890, "content_140e8e890", fn_140e8e890 as *const ()), // sub_140e8e890
    Replacement::draft(0x140e8e970, "content_140e8e970", tpl_18234::<0x14387fcd0, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x8, 0x140559ea0, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140e8e970
    Replacement::draft_throws(0x140e8ea20, "content_140e8ea20", tpl_18235::<0x14387fcf4, 0x14387fcd0, 0x1401e9fa0, 0x1406e02b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x8, 0x140557390, 0x1401e9e70> as *const (), Throws::Discard), // sub_140e8ea20
    Replacement::draft(0x140e8eac0, "content_140e8eac0", tpl_18234::<0x14387fcd0, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x8, 0x14055a030, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140e8eac0
    Replacement::draft_throws(0x140e8eb70, "content_140e8eb70", tpl_18235::<0x14387fcf4, 0x14387fcd0, 0x1401e9b60, 0x1406e03a0, 0x318, 0x328, 0x320, 0x1406e3600, 0x8, 0x1405575f0, 0x1401e9a20> as *const (), Throws::Discard), // sub_140e8eb70
    Replacement::draft_throws(0x140e8ec10, "content_140e8ec10", fn_140e8ec10 as *const (), Throws::Caught), // sub_140e8ec10
    Replacement::draft_throws(0x140e8ecc0, "content_140e8ecc0", fn_140e8ecc0 as *const (), Throws::Caught), // sub_140e8ecc0
    Replacement::draft_throws(0x140e8ed70, "content_140e8ed70", fn_140e8ed70 as *const (), Throws::Caught), // sub_140e8ed70
    Replacement::draft(0x140e8ee20, "content_140e8ee20", fn_140e8ee20 as *const ()), // sub_140e8ee20
    Replacement::draft(0x140e8eed0, "content_140e8eed0", fn_140e8eed0 as *const ()), // sub_140e8eed0
    Replacement::draft(0x140e8efa0, "content_140e8efa0", tpl_18234::<0x14387fcd0, 0x14243b0b0, 0x1406e01b0, 0x318, 0x328, 0x320, 0x1406e3600, 0x8, 0x14055a1a0, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140e8efa0
    Replacement::draft_throws(0x140e8f050, "content_140e8f050", fn_140e8f050 as *const (), Throws::Discard), // sub_140e8f050
    Replacement::draft_throws(0x140e8f0f0, "content_140e8f0f0", fn_140e8f0f0 as *const (), Throws::Discard), // sub_140e8f0f0
    Replacement::draft(0x140e8f1c0, "content_140e8f1c0", fn_140e8f1c0 as *const ()), // sub_140e8f1c0
    Replacement::draft(0x140e8f260, "content_140e8f260", fn_140e8f260 as *const ()), // sub_140e8f260
    Replacement::draft(0x140e8f320, "content_140e8f320", tpl_7312::<0x14387fd50, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e8f320
    Replacement::draft(0x140e8f390, "content_140e8f390", fn_140e8f390 as *const ()), // sub_140e8f390
    Replacement::draft_throws(0x140e8f4a0, "content_140e8f4a0", fn_140e8f4a0 as *const (), Throws::Discard), // sub_140e8f4a0
    Replacement::draft(0x140e8f590, "content_140e8f590", tpl_7320::<0x14387fd50, 0x14243b0b0, 0x1406e01b0, 0x338, 0x10, 0x38, 0x14, 0x18, 0x48, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140e8f590
    Replacement::draft_throws(0x140e8f630, "content_140e8f630", fn_140e8f630 as *const (), Throws::Discard), // sub_140e8f630
    Replacement::draft_throws(0x140e8f730, "content_140e8f730", fn_140e8f730 as *const (), Throws::Caught), // sub_140e8f730
    Replacement::draft_throws(0x140e8fa10, "content_140e8fa10", fn_140e8fa10 as *const (), Throws::Caught), // sub_140e8fa10
    Replacement::draft_throws(0x140e8fcb0, "content_140e8fcb0", fn_140e8fcb0 as *const (), Throws::Caught), // sub_140e8fcb0
    Replacement::draft(0x140e8ff50, "content_140e8ff50", fn_140e8ff50 as *const ()), // sub_140e8ff50
    Replacement::draft(0x140e8ffd0, "content_140e8ffd0", fn_140e8ffd0 as *const ()), // sub_140e8ffd0
    Replacement::draft(0x140e90080, "content_140e90080", fn_140e90080 as *const ()), // sub_140e90080
    Replacement::draft_throws(0x140e90130, "content_140e90130", fn_140e90130 as *const (), Throws::Discard), // sub_140e90130
    Replacement::draft_throws(0x140e901e0, "content_140e901e0", fn_140e901e0 as *const (), Throws::Discard), // sub_140e901e0
    Replacement::draft(0x140e90390, "content_140e90390", fn_140e90390 as *const ()), // sub_140e90390
    Replacement::draft(0x140e90490, "content_140e90490", tpl_6687::<0x8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433000d8> as *const ()), // sub_140e90490
    Replacement::draft(0x140e904f0, "content_140e904f0", tpl_65756::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1433000d8> as *const ()), // sub_140e904f0
    Replacement::draft_throws(0x140e905f0, "content_140e905f0", fn_140e905f0 as *const (), Throws::Caught), // sub_140e905f0
    Replacement::draft_throws(0x140e90680, "content_140e90680", tpl_6763::<0x14387fd00, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x14387d9c8> as *const (), Throws::Discard), // sub_140e90680
    Replacement::draft(0x140e90730, "content_140e90730", tpl_8850::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0xc, 0x1432ff4e0, 0x14, 0x8> as *const ()), // sub_140e90730
    Replacement::draft(0x140e907c0, "content_140e907c0", tpl_16830::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ff4e0, 0x14, 0x18> as *const ()), // sub_140e907c0
    Replacement::draft(0x140e90870, "content_140e90870", tpl_6766::<0x14387fd48, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e90870
    Replacement::draft(0x140e908e0, "content_140e908e0", fn_140e908e0 as *const ()), // sub_140e908e0
    Replacement::draft_throws(0x140e90a20, "content_140e90a20", fn_140e90a20 as *const (), Throws::Discard), // sub_140e90a20
    Replacement::draft(0x140e90b80, "content_140e90b80", fn_140e90b80 as *const ()), // sub_140e90b80
    Replacement::draft_throws(0x140e90c60, "content_140e90c60", fn_140e90c60 as *const (), Throws::Discard), // sub_140e90c60
    Replacement::draft_throws(0x140e90df0, "content_140e90df0", fn_140e90df0 as *const (), Throws::Caught), // sub_140e90df0
    Replacement::draft_throws(0x140e91150, "content_140e91150", fn_140e91150 as *const (), Throws::Caught), // sub_140e91150
    Replacement::draft_throws(0x140e91470, "content_140e91470", fn_140e91470 as *const (), Throws::Caught), // sub_140e91470
    Replacement::draft(0x140e91790, "content_140e91790", fn_140e91790 as *const ()), // sub_140e91790
    Replacement::draft(0x140e91810, "content_140e91810", fn_140e91810 as *const ()), // sub_140e91810
    Replacement::draft(0x140e918f0, "content_140e918f0", fn_140e918f0 as *const ()), // sub_140e918f0
    Replacement::draft_throws(0x140e919e0, "content_140e919e0", fn_140e919e0 as *const (), Throws::Discard), // sub_140e919e0
    Replacement::draft_throws(0x140e91ae0, "content_140e91ae0", fn_140e91ae0 as *const (), Throws::Discard), // sub_140e91ae0
    Replacement::draft(0x140e91d00, "content_140e91d00", tpl_8992::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x143301a68, 0x8> as *const ()), // sub_140e91d00
    Replacement::draft(0x140e91d80, "content_140e91d80", fn_140e91d80 as *const ()), // sub_140e91d80
    Replacement::draft(0x140e91e20, "content_140e91e20", tpl_6766::<0x14387fce8, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e91e20
    Replacement::draft(0x140e91e90, "content_140e91e90", fn_140e91e90 as *const ()), // sub_140e91e90
    Replacement::draft_throws(0x140e91f70, "content_140e91f70", fn_140e91f70 as *const (), Throws::Discard), // sub_140e91f70
    Replacement::draft(0x140e92060, "content_140e92060", tpl_18267::<0x14387fce8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x80, 0x9, 0xa, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e92060
    Replacement::draft_throws(0x140e92110, "content_140e92110", fn_140e92110 as *const (), Throws::Discard), // sub_140e92110
    Replacement::draft_throws(0x140e92200, "content_140e92200", fn_140e92200 as *const (), Throws::Caught), // sub_140e92200
    Replacement::draft_throws(0x140e92340, "content_140e92340", fn_140e92340 as *const (), Throws::Caught), // sub_140e92340
    Replacement::draft_throws(0x140e92480, "content_140e92480", fn_140e92480 as *const (), Throws::Caught), // sub_140e92480
    Replacement::draft(0x140e925c0, "content_140e925c0", fn_140e925c0 as *const ()), // sub_140e925c0
    Replacement::draft(0x140e92660, "content_140e92660", fn_140e92660 as *const ()), // sub_140e92660
    Replacement::draft(0x140e92700, "content_140e92700", tpl_18267::<0x14387fce8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x120, 0x9, 0xa, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e92700
    Replacement::draft_throws(0x140e927b0, "content_140e927b0", fn_140e927b0 as *const (), Throws::Discard), // sub_140e927b0
    Replacement::draft_throws(0x140e92860, "content_140e92860", fn_140e92860 as *const (), Throws::Discard), // sub_140e92860
    Replacement::draft(0x140e929b0, "content_140e929b0", fn_140e929b0 as *const ()), // sub_140e929b0
    Replacement::draft(0x140e92a60, "content_140e92a60", fn_140e92a60 as *const ()), // sub_140e92a60
    Replacement::draft(0x140e92b10, "content_140e92b10", tpl_6748::<0x14387fc88, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e92b10
    Replacement::draft(0x140e92b80, "content_140e92b80", fn_140e92b80 as *const ()), // sub_140e92b80
    Replacement::draft_throws(0x140e92df0, "content_140e92df0", fn_140e92df0 as *const (), Throws::Discard), // sub_140e92df0
    Replacement::draft(0x140e93030, "content_140e93030", fn_140e93030 as *const ()), // sub_140e93030
    Replacement::draft_throws(0x140e93190, "content_140e93190", fn_140e93190 as *const (), Throws::Discard), // sub_140e93190
    Replacement::draft(0x140e93420, "content_140e93420", fn_140e93420 as *const ()), // sub_140e93420
    Replacement::draft_throws(0x140e934e0, "content_140e934e0", fn_140e934e0 as *const (), Throws::Caught), // sub_140e934e0
    Replacement::draft_throws(0x140e93a50, "content_140e93a50", fn_140e93a50 as *const (), Throws::Caught), // sub_140e93a50
    Replacement::draft_throws(0x140e93f50, "content_140e93f50", fn_140e93f50 as *const (), Throws::Caught), // sub_140e93f50
    Replacement::draft(0x140e94450, "content_140e94450", fn_140e94450 as *const ()), // sub_140e94450
    Replacement::draft(0x140e94510, "content_140e94510", fn_140e94510 as *const ()), // sub_140e94510
    Replacement::draft(0x140e946a0, "content_140e946a0", fn_140e946a0 as *const ()), // sub_140e946a0
    Replacement::draft_throws(0x140e94840, "content_140e94840", fn_140e94840 as *const (), Throws::Discard), // sub_140e94840
    Replacement::draft_throws(0x140e949d0, "content_140e949d0", fn_140e949d0 as *const (), Throws::Caught), // sub_140e949d0
    Replacement::draft(0x140e94e80, "content_140e94e80", fn_140e94e80 as *const ()), // sub_140e94e80
    Replacement::draft(0x140e94f60, "content_140e94f60", fn_140e94f60 as *const ()), // sub_140e94f60
    Replacement::draft(0x140e95230, "content_140e95230", fn_140e95230 as *const ()), // sub_140e95230
    Replacement::draft(0x140e952d0, "content_140e952d0", tpl_4999::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1432ff388, 0x8> as *const ()), // sub_140e952d0
    Replacement::draft(0x140e95350, "content_140e95350", tpl_6331::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432ff388> as *const ()), // sub_140e95350
    Replacement::draft(0x140e953e0, "content_140e953e0", tpl_9031::<0x14387fd18, 0x14243b0b0, 0x1406e01b0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e953e0
    Replacement::draft(0x140e95450, "content_140e95450", fn_140e95450 as *const ()), // sub_140e95450
    Replacement::draft_throws(0x140e95530, "content_140e95530", fn_140e95530 as *const (), Throws::Discard), // sub_140e95530
    Replacement::draft(0x140e955f0, "content_140e955f0", tpl_14955::<0x14387fd18, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x48, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e955f0
    Replacement::draft_throws(0x140e95680, "content_140e95680", fn_140e95680 as *const (), Throws::Discard), // sub_140e95680
    Replacement::draft_throws(0x140e95750, "content_140e95750", fn_140e95750 as *const (), Throws::Caught), // sub_140e95750
    Replacement::draft_throws(0x140e95a90, "content_140e95a90", fn_140e95a90 as *const (), Throws::Caught), // sub_140e95a90
    Replacement::draft_throws(0x140e95d70, "content_140e95d70", fn_140e95d70 as *const (), Throws::Caught), // sub_140e95d70
    Replacement::draft(0x140e96040, "content_140e96040", fn_140e96040 as *const ()), // sub_140e96040
    Replacement::draft(0x140e960b0, "content_140e960b0", fn_140e960b0 as *const ()), // sub_140e960b0
    Replacement::draft(0x140e96150, "content_140e96150", fn_140e96150 as *const ()), // sub_140e96150
    Replacement::draft_throws(0x140e96200, "content_140e96200", fn_140e96200 as *const (), Throws::Discard), // sub_140e96200
    Replacement::draft_throws(0x140e96290, "content_140e96290", fn_140e96290 as *const (), Throws::Discard), // sub_140e96290
    Replacement::draft(0x140e96390, "content_140e96390", tpl_14709::<0xa0, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0x28, 0x48, 0x68, 0x88, 0x1432fff80> as *const ()), // sub_140e96390
    Replacement::draft(0x140e96450, "content_140e96450", tpl_14710::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x1432fff80, 0xfffffffffffffff4, 0x14, 0x34, 0x54, 0x74> as *const ()), // sub_140e96450
    Replacement::draft(0x140e965a0, "content_140e965a0", tpl_12411::<0x14387fcb8, 0x14243b0b0, 0x1406e01b0, 0xb, 0x10, 0xc, 0x8, 0x338, 0x1401ee7c0, 0x90, 0x1426d4580, 0x1> as *const ()), // sub_140e965a0
    Replacement::draft_throws(0x140e96690, "content_140e96690", fn_140e96690 as *const (), Throws::Discard), // sub_140e96690
    Replacement::draft(0x140e96770, "content_140e96770", tpl_12375::<0x14387fcb8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0x20, 0x90, 0x48, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e96770
    Replacement::draft_throws(0x140e96800, "content_140e96800", fn_140e96800 as *const (), Throws::Discard), // sub_140e96800
    Replacement::draft_throws(0x140e968f0, "content_140e968f0", fn_140e968f0 as *const (), Throws::Caught), // sub_140e968f0
    Replacement::draft_throws(0x140e96b90, "content_140e96b90", fn_140e96b90 as *const (), Throws::Caught), // sub_140e96b90
    Replacement::draft_throws(0x140e96e00, "content_140e96e00", fn_140e96e00 as *const (), Throws::Caught), // sub_140e96e00
    Replacement::draft(0x140e97060, "content_140e97060", fn_140e97060 as *const ()), // sub_140e97060
    Replacement::draft(0x140e97100, "content_140e97100", tpl_70858::<0x14387fd80, 0x14387fcb8, 0x1401e9970, 0x1406e1480, 0x338, 0x8, 0xc0, 0x90, 0xe8, 0x1401e9860> as *const ()), // sub_140e97100
    Replacement::draft(0x140e97180, "content_140e97180", tpl_12375::<0x14387fcb8, 0x14243b0b0, 0x1406e01b0, 0x338, 0x8, 0xc0, 0x90, 0xe8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140e97180
    Replacement::draft_throws(0x140e97210, "content_140e97210", tpl_70894::<0x14387fd80, 0x14387fcb8, 0x1401e9770, 0x1406e1570, 0x338, 0x8, 0x1424b4cb0, 0x90, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140e97210
    Replacement::draft_throws(0x140e972a0, "content_140e972a0", fn_140e972a0 as *const (), Throws::Discard), // sub_140e972a0
    Replacement::draft(0x140e97430, "content_140e97430", fn_140e97430 as *const ()), // content::QuestNodeComponent::content::QuestNodeComponent::vfunc_0
    Replacement::draft(0x140e97490, "content_140e97490", fn_140e97490 as *const ()), // content::SplinePointComponent::content::SplinePointComponent::vfunc_0
    Replacement::draft(0x140e97530, "content_140e97530", tpl_13661::<0x1401f2bc0, 0x10, 0x143084e28, 0x141834d30> as *const ()), // content::CollapsedLodComponent::content::CollapsedLodComponent::vfunc_0
    Replacement::draft(0x140e97580, "content_140e97580", fn_140e97580 as *const ()), // content::DeprecatedComponent::content::DeprecatedComponent::vfunc_0
    Replacement::draft(0x140e97630, "content_140e97630", fn_140e97630 as *const ()), // content::JumpPointComponent::content::JumpPointComponent::vfunc_0
    Replacement::draft(0x140e97680, "content_140e97680", tpl_18312::<0x8, 0x140e97ef0, 0xc, 0x18, 0x30, 0xffffffffffffffd0, 0xffffffffffffffd8, 0xffffffffffffffc4, 0xffffffffffffffcc, 0xffffffffffffffdc, 0xffffffffffffffe4> as *const ()), // sub_140e97680
    Replacement::draft(0x140e97760, "content_140e97760", tpl_18313::<0x140e9df70, 0x14183acc8> as *const ()), // sub_140e97760
    Replacement::draft(0x140e97850, "content_140e97850", fn_140e97850 as *const ()), // sub_140e97850
    Replacement::draft_throws(0x140e97930, "content_140e97930", tpl_12664::<0x20, 0x8, 0x140e9b1d0> as *const (), Throws::Discard), // sub_140e97930
    Replacement::draft_throws(0x140e97990, "content_140e97990", tpl_838::<0x140e9b570> as *const (), Throws::Discard), // sub_140e97990
    Replacement::draft_throws(0x140e979b0, "content_140e979b0", tpl_838::<0x140e9bb40> as *const (), Throws::Discard), // sub_140e979b0
    Replacement::draft_throws(0x140e979d0, "content_140e979d0", tpl_838::<0x140e9c0d0> as *const (), Throws::Discard), // sub_140e979d0
    Replacement::draft(0x140e97a60, "content_140e97a60", fn_140e97a60 as *const ()), // sub_140e97a60
    Replacement::draft(0x140e97ac0, "content_140e97ac0", fn_140e97ac0 as *const ()), // sub_140e97ac0
    Replacement::draft_throws(0x140e97c50, "content_140e97c50", fn_140e97c50 as *const (), Throws::Caught), // sub_140e97c50
    Replacement::draft_throws(0x140e97e20, "content_140e97e20", fn_140e97e20 as *const (), Throws::Discard), // sub_140e97e20
    Replacement::draft_throws(0x140e97e40, "content_140e97e40", fn_140e97e40 as *const (), Throws::Discard), // sub_140e97e40
    Replacement::draft(0x140e97e60, "content_140e97e60", fn_140e97e60 as *const ()), // sub_140e97e60
    Replacement::draft(0x140e97ef0, "content_140e97ef0", tpl_76689::<0x14024a9e0, 0x1425371a0, 0x142533200> as *const ()), // sub_140e97ef0
    Replacement::draft(0x140e98030, "content_140e98030", fn_140e98030 as *const ()), // sub_140e98030
    Replacement::draft(0x140e98100, "content_140e98100", fn_140e98100 as *const ()), // sub_140e98100
    Replacement::draft_throws(0x140e98240, "content_140e98240", fn_140e98240 as *const (), Throws::Caught), // sub_140e98240
    Replacement::draft_throws(0x140e98a10, "content_140e98a10", fn_140e98a10 as *const (), Throws::Caught), // sub_140e98a10
    Replacement::draft_throws(0x140e990e0, "content_140e990e0", fn_140e990e0 as *const (), Throws::Caught), // sub_140e990e0
    Replacement::draft_throws(0x140e99720, "content_140e99720", fn_140e99720 as *const (), Throws::Caught), // sub_140e99720
    Replacement::draft_throws(0x140e99a80, "content_140e99a80", fn_140e99a80 as *const (), Throws::Caught), // sub_140e99a80
    Replacement::draft(0x140e99de0, "content_140e99de0", fn_140e99de0 as *const ()), // sub_140e99de0
    Replacement::draft_throws(0x140e9a3b0, "content_140e9a3b0", fn_140e9a3b0 as *const (), Throws::Caught), // sub_140e9a3b0
    Replacement::draft_throws(0x140e9a8b0, "content_140e9a8b0", fn_140e9a8b0 as *const (), Throws::Caught), // sub_140e9a8b0
    Replacement::draft_throws(0x140e9ad50, "content_140e9ad50", fn_140e9ad50 as *const (), Throws::Caught), // sub_140e9ad50
    Replacement::draft_throws(0x140e9b1d0, "content_140e9b1d0", fn_140e9b1d0 as *const (), Throws::Caught), // sub_140e9b1d0
    Replacement::draft_throws(0x140e9b2a0, "content_140e9b2a0", fn_140e9b2a0 as *const (), Throws::Caught), // sub_140e9b2a0
    Replacement::draft_throws(0x140e9b570, "content_140e9b570", fn_140e9b570 as *const (), Throws::Caught), // sub_140e9b570
    Replacement::draft_throws(0x140e9b8a0, "content_140e9b8a0", fn_140e9b8a0 as *const (), Throws::Caught), // sub_140e9b8a0
    Replacement::draft_throws(0x140e9bb40, "content_140e9bb40", fn_140e9bb40 as *const (), Throws::Caught), // sub_140e9bb40
    Replacement::draft_throws(0x140e9be40, "content_140e9be40", fn_140e9be40 as *const (), Throws::Caught), // sub_140e9be40
    Replacement::draft_throws(0x140e9c0d0, "content_140e9c0d0", fn_140e9c0d0 as *const (), Throws::Caught), // sub_140e9c0d0
    Replacement::draft_throws(0x140e9c3d0, "content_140e9c3d0", fn_140e9c3d0 as *const (), Throws::Caught), // sub_140e9c3d0
    Replacement::draft_throws(0x140e9c6a0, "content_140e9c6a0", fn_140e9c6a0 as *const (), Throws::Caught), // sub_140e9c6a0
    Replacement::draft_throws(0x140e9c970, "content_140e9c970", fn_140e9c970 as *const (), Throws::Caught), // sub_140e9c970
    Replacement::draft_throws(0x140e9cdc0, "content_140e9cdc0", fn_140e9cdc0 as *const (), Throws::Caught), // sub_140e9cdc0
    Replacement::draft_throws(0x140e9d1e0, "content_140e9d1e0", fn_140e9d1e0 as *const (), Throws::Caught), // sub_140e9d1e0
    Replacement::draft_throws(0x140e9d600, "content_140e9d600", fn_140e9d600 as *const (), Throws::Caught), // sub_140e9d600
    Replacement::draft(0x140e9d6d0, "content_140e9d6d0", fn_140e9d6d0 as *const ()), // sub_140e9d6d0
    Replacement::draft_throws(0x140e9d710, "content_140e9d710", fn_140e9d710 as *const (), Throws::Caught), // sub_140e9d710
    Replacement::draft_throws(0x140e9da00, "content_140e9da00", fn_140e9da00 as *const (), Throws::Caught), // sub_140e9da00
    Replacement::draft_throws(0x140e9dcc0, "content_140e9dcc0", fn_140e9dcc0 as *const (), Throws::Caught), // sub_140e9dcc0
    Replacement::draft(0x140e9df70, "content_140e9df70", fn_140e9df70 as *const ()), // sub_140e9df70
    Replacement::draft_throws(0x140e9dff0, "content_140e9dff0", fn_140e9dff0 as *const (), Throws::Caught), // sub_140e9dff0
    Replacement::draft_throws(0x140e9e390, "content_140e9e390", fn_140e9e390 as *const (), Throws::Caught), // sub_140e9e390
    Replacement::draft_throws(0x140e9e8c0, "content_140e9e8c0", fn_140e9e8c0 as *const (), Throws::Caught), // sub_140e9e8c0
    Replacement::draft_throws(0x140e9ed90, "content_140e9ed90", fn_140e9ed90 as *const (), Throws::Caught), // sub_140e9ed90
    Replacement::draft_throws(0x140e9f220, "content_140e9f220", fn_140e9f220 as *const (), Throws::Caught), // sub_140e9f220
    Replacement::draft_throws(0x140e9f6b0, "content_140e9f6b0", fn_140e9f6b0 as *const (), Throws::Caught), // sub_140e9f6b0
    Replacement::draft_throws(0x140e9fb80, "content_140e9fb80", fn_140e9fb80 as *const (), Throws::Caught), // sub_140e9fb80
    Replacement::draft_throws(0x140ea0010, "content_140ea0010", fn_140ea0010 as *const (), Throws::Caught), // sub_140ea0010
    Replacement::draft_throws(0x140ea04a0, "content_140ea04a0", fn_140ea04a0 as *const (), Throws::Caught), // sub_140ea04a0
    Replacement::draft_throws(0x140ea0960, "content_140ea0960", fn_140ea0960 as *const (), Throws::Caught), // sub_140ea0960
    Replacement::draft_throws(0x140ea0de0, "content_140ea0de0", fn_140ea0de0 as *const (), Throws::Caught), // sub_140ea0de0
    Replacement::draft(0x140ea1260, "content_140ea1260", tpl_1060::<0x14387fea0, 0x14243b0b0, 0x318, 0x328, 0x320, 0x8, 0x4, 0x38, 0xfffffffffffffff8, 0xffffffffffffffff> as *const ()), // sub_140ea1260
    Replacement::draft(0x140ea12f0, "content_140ea12f0", fn_140ea12f0 as *const ()), // sub_140ea12f0
    Replacement::draft_throws(0x140ea1570, "content_140ea1570", fn_140ea1570 as *const (), Throws::Caught), // sub_140ea1570
    Replacement::draft_throws(0x140ea1840, "content_140ea1840", fn_140ea1840 as *const (), Throws::Caught), // sub_140ea1840
    Replacement::draft_throws(0x140ea1b10, "content_140ea1b10", fn_140ea1b10 as *const (), Throws::Caught), // sub_140ea1b10
    Replacement::draft_throws(0x140ea1de0, "content_140ea1de0", fn_140ea1de0 as *const (), Throws::Caught), // sub_140ea1de0
    Replacement::draft(0x140ea1fd0, "content_140ea1fd0", fn_140ea1fd0 as *const ()), // sub_140ea1fd0
    Replacement::draft_throws(0x140ea20d0, "content_140ea20d0", fn_140ea20d0 as *const (), Throws::Caught), // sub_140ea20d0
    Replacement::draft_throws(0x140ea2340, "content_140ea2340", fn_140ea2340 as *const (), Throws::Caught), // sub_140ea2340
    Replacement::draft_throws(0x140ea25a0, "content_140ea25a0", fn_140ea25a0 as *const (), Throws::Caught), // sub_140ea25a0
    Replacement::draft_throws(0x140ea2800, "content_140ea2800", fn_140ea2800 as *const (), Throws::Caught), // sub_140ea2800
    Replacement::draft_throws(0x140ea29f0, "content_140ea29f0", fn_140ea29f0 as *const (), Throws::Caught), // sub_140ea29f0
    Replacement::draft_throws(0x140ea2ed0, "content_140ea2ed0", fn_140ea2ed0 as *const (), Throws::Discard), // sub_140ea2ed0
    Replacement::draft(0x140ea3130, "content_140ea3130", fn_140ea3130 as *const ()), // sub_140ea3130
    Replacement::draft(0x140ea3220, "content_140ea3220", fn_140ea3220 as *const ()), // sub_140ea3220
    Replacement::draft_throws(0x140ea33a0, "content_140ea33a0", fn_140ea33a0 as *const (), Throws::Discard), // sub_140ea33a0
    Replacement::draft(0x140ea3630, "content_140ea3630", fn_140ea3630 as *const ()), // sub_140ea3630
    Replacement::draft_throws(0x140ea36f0, "content_140ea36f0", fn_140ea36f0 as *const (), Throws::Caught), // sub_140ea36f0
    Replacement::draft_throws(0x140ea3cf0, "content_140ea3cf0", fn_140ea3cf0 as *const (), Throws::Caught), // sub_140ea3cf0
    Replacement::draft_throws(0x140ea42c0, "content_140ea42c0", fn_140ea42c0 as *const (), Throws::Caught), // sub_140ea42c0
    Replacement::draft(0x140ea4610, "content_140ea4610", fn_140ea4610 as *const ()), // sub_140ea4610
    Replacement::draft(0x140ea47a0, "content_140ea47a0", fn_140ea47a0 as *const ()), // sub_140ea47a0
    Replacement::draft_throws(0x140ea4940, "content_140ea4940", fn_140ea4940 as *const (), Throws::Discard), // sub_140ea4940
    Replacement::draft_throws(0x140ea4ae0, "content_140ea4ae0", fn_140ea4ae0 as *const (), Throws::Discard), // sub_140ea4ae0
    Replacement::draft(0x140ea4bc0, "content_140ea4bc0", fn_140ea4bc0 as *const ()), // sub_140ea4bc0
    Replacement::draft(0x140ea4c50, "content_140ea4c50", fn_140ea4c50 as *const ()), // sub_140ea4c50
    Replacement::draft_throws(0x140ea4ce0, "content_140ea4ce0", fn_140ea4ce0 as *const (), Throws::Discard), // sub_140ea4ce0
    Replacement::draft_throws(0x140ea4de0, "content_140ea4de0", fn_140ea4de0 as *const (), Throws::Caught), // sub_140ea4de0
    Replacement::draft_throws(0x140ea5370, "content_140ea5370", fn_140ea5370 as *const (), Throws::Caught), // sub_140ea5370
    Replacement::draft_throws(0x140ea58c0, "content_140ea58c0", fn_140ea58c0 as *const (), Throws::Caught), // sub_140ea58c0
    Replacement::draft(0x140ea5c10, "content_140ea5c10", fn_140ea5c10 as *const ()), // sub_140ea5c10
    Replacement::draft(0x140ea5cb0, "content_140ea5cb0", fn_140ea5cb0 as *const ()), // sub_140ea5cb0
    Replacement::draft_throws(0x140ea5d60, "content_140ea5d60", fn_140ea5d60 as *const (), Throws::Discard), // sub_140ea5d60
    Replacement::draft_throws(0x140ea5e10, "content_140ea5e10", fn_140ea5e10 as *const (), Throws::Discard), // sub_140ea5e10
    Replacement::draft_throws(0x140ea61c0, "content_140ea61c0", fn_140ea61c0 as *const (), Throws::Discard), // sub_140ea61c0
    Replacement::draft_throws(0x140ea6300, "content_140ea6300", fn_140ea6300 as *const (), Throws::Caught), // sub_140ea6300
    Replacement::draft_throws(0x140ea6900, "content_140ea6900", fn_140ea6900 as *const (), Throws::Caught), // sub_140ea6900
    Replacement::draft_throws(0x140ea6e70, "content_140ea6e70", fn_140ea6e70 as *const (), Throws::Caught), // sub_140ea6e70
    Replacement::draft_throws(0x140ea73c0, "content_140ea73c0", fn_140ea73c0 as *const (), Throws::Caught), // sub_140ea73c0
    Replacement::draft_throws(0x140ea7650, "content_140ea7650", fn_140ea7650 as *const (), Throws::Caught), // sub_140ea7650
    Replacement::draft_throws(0x140ea78b0, "content_140ea78b0", fn_140ea78b0 as *const (), Throws::Caught), // sub_140ea78b0
    Replacement::draft(0x140ea7b00, "content_140ea7b00", fn_140ea7b00 as *const ()), // sub_140ea7b00
    Replacement::draft(0x140ea7d00, "content_140ea7d00", fn_140ea7d00 as *const ()), // sub_140ea7d00
    Replacement::draft(0x140ea7e30, "content_140ea7e30", tpl_2030::<0x140ea8010, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140ea7e30
    Replacement::draft(0x140ea7f90, "content_140ea7f90", fn_140ea7f90 as *const ()), // sub_140ea7f90
    Replacement::draft(0x140ea8010, "content_140ea8010", fn_140ea8010 as *const ()), // sub_140ea8010
    Replacement::draft(0x140ea80a0, "content_140ea80a0", fn_140ea80a0 as *const ()), // sub_140ea80a0
    Replacement::draft(0x140ea8120, "content_140ea8120", fn_140ea8120 as *const ()), // sub_140ea8120
    Replacement::draft(0x140ea8400, "content_140ea8400", fn_140ea8400 as *const ()), // sub_140ea8400
    Replacement::draft(0x140ea84d0, "content_140ea84d0", fn_140ea84d0 as *const ()), // sub_140ea84d0
    Replacement::draft(0x140ea87d0, "content_140ea87d0", fn_140ea87d0 as *const ()), // sub_140ea87d0
    Replacement::draft_throws(0x140ea94b0, "content_140ea94b0", fn_140ea94b0 as *const (), Throws::Caught), // sub_140ea94b0
    Replacement::draft_throws(0x140eaaf60, "content_140eaaf60", fn_140eaaf60 as *const (), Throws::Discard), // sub_140eaaf60
    Replacement::draft_throws(0x140eac180, "content_140eac180", fn_140eac180 as *const (), Throws::Caught), // sub_140eac180
    Replacement::draft(0x140eae100, "content_140eae100", fn_140eae100 as *const ()), // sub_140eae100
    Replacement::draft_throws(0x140eaebb0, "content_140eaebb0", fn_140eaebb0 as *const (), Throws::Caught), // sub_140eaebb0
    Replacement::draft_throws(0x140eb0aa0, "content_140eb0aa0", fn_140eb0aa0 as *const (), Throws::Caught), // sub_140eb0aa0
    Replacement::draft_throws(0x140eb2960, "content_140eb2960", fn_140eb2960 as *const (), Throws::Caught), // sub_140eb2960
    Replacement::draft(0x140eb47e0, "content_140eb47e0", fn_140eb47e0 as *const ()), // sub_140eb47e0
    Replacement::draft(0x140eb5420, "content_140eb5420", fn_140eb5420 as *const ()), // sub_140eb5420
    Replacement::draft(0x140eb6020, "content_140eb6020", fn_140eb6020 as *const ()), // sub_140eb6020
    Replacement::draft_throws(0x140eb6c30, "content_140eb6c30", fn_140eb6c30 as *const (), Throws::Caught), // sub_140eb6c30
    Replacement::draft_throws(0x140eb7a70, "content_140eb7a70", fn_140eb7a70 as *const (), Throws::Caught), // sub_140eb7a70
    Replacement::draft(0x140eb9000, "content_140eb9000", fn_140eb9000 as *const ()), // sub_140eb9000
    Replacement::draft_throws(0x140eb95e0, "content_140eb95e0", fn_140eb95e0 as *const (), Throws::Caught), // sub_140eb95e0
    Replacement::draft(0x140eba380, "content_140eba380", fn_140eba380 as *const ()), // sub_140eba380
    Replacement::draft(0x140eba920, "content_140eba920", fn_140eba920 as *const ()), // sub_140eba920
    Replacement::draft(0x140eba9c0, "content_140eba9c0", fn_140eba9c0 as *const ()), // sub_140eba9c0
    Replacement::draft(0x140ebab10, "content_140ebab10", fn_140ebab10 as *const ()), // sub_140ebab10
    Replacement::draft_throws(0x140ebacf0, "content_140ebacf0", fn_140ebacf0 as *const (), Throws::Discard), // sub_140ebacf0
    Replacement::draft_throws(0x140ebae60, "content_140ebae60", fn_140ebae60 as *const (), Throws::Discard), // sub_140ebae60
    Replacement::draft_throws(0x140ebb010, "content_140ebb010", fn_140ebb010 as *const (), Throws::Discard), // sub_140ebb010
    Replacement::draft(0x140ebb180, "content_140ebb180", fn_140ebb180 as *const ()), // sub_140ebb180
    Replacement::draft_throws(0x140ebb230, "content_140ebb230", fn_140ebb230 as *const (), Throws::Caught), // sub_140ebb230
    Replacement::draft_throws(0x140ebb300, "content_140ebb300", fn_140ebb300 as *const (), Throws::Caught), // sub_140ebb300
    Replacement::draft_throws(0x140ebb3d0, "content_140ebb3d0", fn_140ebb3d0 as *const (), Throws::Caught), // sub_140ebb3d0
    Replacement::draft(0x140ebb4a0, "content_140ebb4a0", fn_140ebb4a0 as *const ()), // sub_140ebb4a0
    Replacement::draft(0x140ebb640, "content_140ebb640", fn_140ebb640 as *const ()), // sub_140ebb640
    Replacement::draft(0x140ebb7e0, "content_140ebb7e0", fn_140ebb7e0 as *const ()), // sub_140ebb7e0
    Replacement::draft_throws(0x140ebb980, "content_140ebb980", fn_140ebb980 as *const (), Throws::Discard), // sub_140ebb980
    Replacement::draft_throws(0x140ebbb10, "content_140ebbb10", fn_140ebbb10 as *const (), Throws::Discard), // sub_140ebbb10
    Replacement::draft(0x140ebbbd0, "content_140ebbbd0", tpl_18389::<0x8, 0x1426cdcb0, 0x140ec8550> as *const ()), // sub_140ebbbd0
    Replacement::draft(0x140ebbc50, "content_140ebbc50", tpl_18389::<0x8, 0x143303928, 0x140ec8570> as *const ()), // sub_140ebbc50
    Replacement::draft(0x140ebbcd0, "content_140ebbcd0", fn_140ebbcd0 as *const ()), // sub_140ebbcd0
    Replacement::draft(0x140ebbdb0, "content_140ebbdb0", fn_140ebbdb0 as *const ()), // sub_140ebbdb0
    Replacement::draft(0x140ebbec0, "content_140ebbec0", fn_140ebbec0 as *const ()), // sub_140ebbec0
    Replacement::draft(0x140ebbf80, "content_140ebbf80", fn_140ebbf80 as *const ()), // sub_140ebbf80
    Replacement::draft_throws(0x140ebc090, "content_140ebc090", fn_140ebc090 as *const (), Throws::Discard), // sub_140ebc090
    Replacement::draft(0x140ebc1b0, "content_140ebc1b0", tpl_18394::<0x14387fdf0, 0x14243b0b0, 0x338, 0x20, 0x108, 0x210, 0x140ec28d0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ebc1b0
    Replacement::draft_throws(0x140ebc240, "content_140ebc240", fn_140ebc240 as *const (), Throws::Discard), // sub_140ebc240
    Replacement::draft_throws(0x140ebc350, "content_140ebc350", fn_140ebc350 as *const (), Throws::Caught), // sub_140ebc350
    Replacement::draft_throws(0x140ebc430, "content_140ebc430", fn_140ebc430 as *const (), Throws::Caught), // sub_140ebc430
    Replacement::draft_throws(0x140ebc510, "content_140ebc510", fn_140ebc510 as *const (), Throws::Caught), // sub_140ebc510
    Replacement::draft(0x140ebc5f0, "content_140ebc5f0", fn_140ebc5f0 as *const ()), // sub_140ebc5f0
    Replacement::draft(0x140ebc720, "content_140ebc720", fn_140ebc720 as *const ()), // sub_140ebc720
    Replacement::draft(0x140ebc7b0, "content_140ebc7b0", tpl_18394::<0x14387fdf0, 0x14243b0b0, 0x338, 0xc0, 0x108, 0x210, 0x140ec2c50, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ebc7b0
    Replacement::draft_throws(0x140ebc850, "content_140ebc850", fn_140ebc850 as *const (), Throws::Discard), // sub_140ebc850
    Replacement::draft_throws(0x140ebc930, "content_140ebc930", fn_140ebc930 as *const (), Throws::Discard), // sub_140ebc930
    Replacement::draft(0x140ebca30, "content_140ebca30", fn_140ebca30 as *const ()), // sub_140ebca30
    Replacement::draft_throws(0x140ebcac0, "content_140ebcac0", fn_140ebcac0 as *const (), Throws::Caught), // sub_140ebcac0
    Replacement::draft_throws(0x140ebcb40, "content_140ebcb40", tpl_649::<0x14387fd98, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140ebcb40
    Replacement::draft(0x140ebcbc0, "content_140ebcbc0", fn_140ebcbc0 as *const ()), // sub_140ebcbc0
    Replacement::draft(0x140ebcc40, "content_140ebcc40", fn_140ebcc40 as *const ()), // sub_140ebcc40
    Replacement::draft(0x140ebcd30, "content_140ebcd30", fn_140ebcd30 as *const ()), // sub_140ebcd30
    Replacement::draft(0x140ebce00, "content_140ebce00", fn_140ebce00 as *const ()), // sub_140ebce00
    Replacement::draft(0x140ebcf20, "content_140ebcf20", fn_140ebcf20 as *const ()), // sub_140ebcf20
    Replacement::draft_throws(0x140ebd250, "content_140ebd250", fn_140ebd250 as *const (), Throws::Discard), // sub_140ebd250
    Replacement::draft_throws(0x140ebd4c0, "content_140ebd4c0", fn_140ebd4c0 as *const (), Throws::Discard), // sub_140ebd4c0
    Replacement::draft_throws(0x140ebd650, "content_140ebd650", fn_140ebd650 as *const (), Throws::Discard), // sub_140ebd650
    Replacement::draft_throws(0x140ebd8d0, "content_140ebd8d0", fn_140ebd8d0 as *const (), Throws::Discard), // sub_140ebd8d0
    Replacement::draft_throws(0x140ebda30, "content_140ebda30", fn_140ebda30 as *const (), Throws::Caught), // sub_140ebda30
    Replacement::draft_throws(0x140ebdbf0, "content_140ebdbf0", fn_140ebdbf0 as *const (), Throws::Caught), // sub_140ebdbf0
    Replacement::draft_throws(0x140ebddb0, "content_140ebddb0", fn_140ebddb0 as *const (), Throws::Caught), // sub_140ebddb0
    Replacement::draft(0x140ebdf70, "content_140ebdf70", fn_140ebdf70 as *const ()), // sub_140ebdf70
    Replacement::draft(0x140ebe190, "content_140ebe190", fn_140ebe190 as *const ()), // sub_140ebe190
    Replacement::draft(0x140ebe3e0, "content_140ebe3e0", fn_140ebe3e0 as *const ()), // sub_140ebe3e0
    Replacement::draft_throws(0x140ebe640, "content_140ebe640", fn_140ebe640 as *const (), Throws::Discard), // sub_140ebe640
    Replacement::draft_throws(0x140ebe8d0, "content_140ebe8d0", fn_140ebe8d0 as *const (), Throws::Discard), // sub_140ebe8d0
    Replacement::draft(0x140ebeaf0, "content_140ebeaf0", fn_140ebeaf0 as *const ()), // sub_140ebeaf0
    Replacement::draft(0x140ebeb90, "content_140ebeb90", fn_140ebeb90 as *const ()), // sub_140ebeb90
    Replacement::draft(0x140ebec40, "content_140ebec40", fn_140ebec40 as *const ()), // sub_140ebec40
    Replacement::draft(0x140ebed30, "content_140ebed30", tpl_636::<0x14387fdd0, 0x14243b0b0, 0x14387fd98, 0x8, 0xfffffffffffffff8, 0x328, 0xffffffffffffffff> as *const ()), // sub_140ebed30
    Replacement::draft(0x140ebedc0, "content_140ebedc0", fn_140ebedc0 as *const ()), // sub_140ebedc0
    Replacement::draft_throws(0x140ebeeb0, "content_140ebeeb0", fn_140ebeeb0 as *const (), Throws::Discard), // sub_140ebeeb0
    Replacement::draft(0x140ebef80, "content_140ebef80", tpl_694::<0x14387fdd0, 0x14243b0b0, 0x14387fd98, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0x20> as *const ()), // sub_140ebef80
    Replacement::draft_throws(0x140ebf030, "content_140ebf030", fn_140ebf030 as *const (), Throws::Discard), // sub_140ebf030
    Replacement::draft_throws(0x140ebf100, "content_140ebf100", fn_140ebf100 as *const (), Throws::Caught), // sub_140ebf100
    Replacement::draft_throws(0x140ebf1b0, "content_140ebf1b0", fn_140ebf1b0 as *const (), Throws::Caught), // sub_140ebf1b0
    Replacement::draft_throws(0x140ebf260, "content_140ebf260", fn_140ebf260 as *const (), Throws::Caught), // sub_140ebf260
    Replacement::draft(0x140ebf310, "content_140ebf310", fn_140ebf310 as *const ()), // sub_140ebf310
    Replacement::draft(0x140ebf3e0, "content_140ebf3e0", fn_140ebf3e0 as *const ()), // sub_140ebf3e0
    Replacement::draft(0x140ebf470, "content_140ebf470", tpl_694::<0x14387fdd0, 0x14243b0b0, 0x14387fd98, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328, 0x338, 0xc0> as *const ()), // sub_140ebf470
    Replacement::draft_throws(0x140ebf520, "content_140ebf520", fn_140ebf520 as *const (), Throws::Discard), // sub_140ebf520
    Replacement::draft_throws(0x140ebf5b0, "content_140ebf5b0", fn_140ebf5b0 as *const (), Throws::Discard), // sub_140ebf5b0
    Replacement::draft(0x140ebf690, "content_140ebf690", fn_140ebf690 as *const ()), // sub_140ebf690
    Replacement::draft(0x140ebf750, "content_140ebf750", fn_140ebf750 as *const ()), // sub_140ebf750
    Replacement::draft(0x140ebf830, "content_140ebf830", tpl_18422::<0x14387fdc0, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x108, 0x140e71130, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ebf830
    Replacement::draft_throws(0x140ebf8f0, "content_140ebf8f0", fn_140ebf8f0 as *const (), Throws::Discard), // sub_140ebf8f0
    Replacement::draft(0x140ebf9e0, "content_140ebf9e0", fn_140ebf9e0 as *const ()), // sub_140ebf9e0
    Replacement::draft_throws(0x140ebfab0, "content_140ebfab0", fn_140ebfab0 as *const (), Throws::Discard), // sub_140ebfab0
    Replacement::draft(0x140ebfbc0, "content_140ebfbc0", fn_140ebfbc0 as *const ()), // sub_140ebfbc0
    Replacement::draft_throws(0x140ebfc40, "content_140ebfc40", fn_140ebfc40 as *const (), Throws::Caught), // sub_140ebfc40
    Replacement::draft_throws(0x140ebfd00, "content_140ebfd00", fn_140ebfd00 as *const (), Throws::Caught), // sub_140ebfd00
    Replacement::draft_throws(0x140ebfdc0, "content_140ebfdc0", fn_140ebfdc0 as *const (), Throws::Caught), // sub_140ebfdc0
    Replacement::draft(0x140ebfe80, "content_140ebfe80", fn_140ebfe80 as *const ()), // sub_140ebfe80
    Replacement::draft(0x140ebff50, "content_140ebff50", fn_140ebff50 as *const ()), // sub_140ebff50
    Replacement::draft(0x140ec0020, "content_140ec0020", fn_140ec0020 as *const ()), // sub_140ec0020
    Replacement::draft_throws(0x140ec0100, "content_140ec0100", fn_140ec0100 as *const (), Throws::Discard), // sub_140ec0100
    Replacement::draft_throws(0x140ec01c0, "content_140ec01c0", fn_140ec01c0 as *const (), Throws::Discard), // sub_140ec01c0
    Replacement::draft(0x140ec0290, "content_140ec0290", fn_140ec0290 as *const ()), // sub_140ec0290
    Replacement::draft(0x140ec1570, "content_140ec1570", fn_140ec1570 as *const ()), // content::UIProperty::content::UIProperty::vfunc_0
    Replacement::draft(0x140ec1640, "content_140ec1640", fn_140ec1640 as *const ()), // content::UIPropertyRef::content::UIPropertyRef::vfunc_0
    Replacement::draft_throws(0x140ec1680, "unknown_140ec1680", fn_140ec1680 as *const (), Throws::Discard), // sub_140ec1680
    Replacement::draft_throws(0x140ec1730, "unknown_140ec1730", tpl_65670::<0x142442de0, 0x143a43ef0, 0x28, 0x143243118, 0x40, 0x1424b0060, 0x68> as *const (), Throws::Discard), // sub_140ec1730
    Replacement::draft(0x140ec1810, "unknown_140ec1810", fn_140ec1810 as *const ()), // sub_140ec1810
    Replacement::draft(0x140ec1970, "unknown_140ec1970", fn_140ec1970 as *const ()), // sub_140ec1970
    Replacement::draft(0x140ec1a40, "unknown_140ec1a40", fn_140ec1a40 as *const ()), // sub_140ec1a40
    Replacement::draft(0x140ec1b10, "unknown_140ec1b10", fn_140ec1b10 as *const ()), // sub_140ec1b10
    Replacement::draft(0x140ec1bb0, "unknown_140ec1bb0", fn_140ec1bb0 as *const ()), // sub_140ec1bb0
    Replacement::draft(0x140ec1ca0, "unknown_140ec1ca0", fn_140ec1ca0 as *const ()), // sub_140ec1ca0
    Replacement::draft_throws(0x140ec1d30, "unknown_140ec1d30", fn_140ec1d30 as *const (), Throws::Caught), // sub_140ec1d30
    Replacement::draft_throws(0x140ec1e80, "unknown_140ec1e80", fn_140ec1e80 as *const (), Throws::Caught), // sub_140ec1e80
    Replacement::draft_throws(0x140ec1fd0, "unknown_140ec1fd0", fn_140ec1fd0 as *const (), Throws::Caught), // sub_140ec1fd0
    Replacement::draft(0x140ec2110, "unknown_140ec2110", fn_140ec2110 as *const ()), // sub_140ec2110
    Replacement::draft(0x140ec21f0, "unknown_140ec21f0", fn_140ec21f0 as *const ()), // sub_140ec21f0
    Replacement::draft_throws(0x140ec2270, "unknown_140ec2270", fn_140ec2270 as *const (), Throws::Caught), // sub_140ec2270
    Replacement::draft_throws(0x140ec2430, "unknown_140ec2430", fn_140ec2430 as *const (), Throws::Caught), // sub_140ec2430
    Replacement::draft(0x140ec2660, "unknown_140ec2660", fn_140ec2660 as *const ()), // sub_140ec2660
    Replacement::draft(0x140ec2710, "unknown_140ec2710", fn_140ec2710 as *const ()), // sub_140ec2710
    Replacement::draft(0x140ec28d0, "unknown_140ec28d0", fn_140ec28d0 as *const ()), // sub_140ec28d0
    Replacement::draft(0x140ec2950, "unknown_140ec2950", fn_140ec2950 as *const ()), // sub_140ec2950
    Replacement::draft(0x140ec2b10, "unknown_140ec2b10", fn_140ec2b10 as *const ()), // sub_140ec2b10
    Replacement::draft(0x140ec2bd0, "unknown_140ec2bd0", tpl_76826::<0x8, 0x338, 0xe8, 0x40> as *const ()), // sub_140ec2bd0
    Replacement::draft(0x140ec2c50, "unknown_140ec2c50", tpl_76826::<0x8, 0x338, 0xe8, 0x38> as *const ()), // sub_140ec2c50
    Replacement::draft(0x140ec2d10, "unknown_140ec2d10", fn_140ec2d10 as *const ()), // sub_140ec2d10
    Replacement::draft_throws(0x140ec2d90, "unknown_140ec2d90", fn_140ec2d90 as *const (), Throws::Caught), // sub_140ec2d90
    Replacement::draft_throws(0x140ec3090, "unknown_140ec3090", fn_140ec3090 as *const (), Throws::Discard), // sub_140ec3090
    Replacement::draft_throws(0x140ec32b0, "unknown_140ec32b0", fn_140ec32b0 as *const (), Throws::Caught), // sub_140ec32b0
    Replacement::draft_throws(0x140ec35a0, "unknown_140ec35a0", fn_140ec35a0 as *const (), Throws::Caught), // sub_140ec35a0
    Replacement::draft_throws(0x140ec39b0, "unknown_140ec39b0", fn_140ec39b0 as *const (), Throws::Caught), // sub_140ec39b0
    Replacement::draft_throws(0x140ec4470, "unknown_140ec4470", fn_140ec4470 as *const (), Throws::Caught), // sub_140ec4470
    Replacement::draft_throws(0x140ec4f00, "unknown_140ec4f00", fn_140ec4f00 as *const (), Throws::Caught), // sub_140ec4f00
    Replacement::draft_throws(0x140ec5980, "unknown_140ec5980", fn_140ec5980 as *const (), Throws::Caught), // sub_140ec5980
    Replacement::draft(0x140ec5c40, "unknown_140ec5c40", fn_140ec5c40 as *const ()), // sub_140ec5c40
    Replacement::draft(0x140ec5cd0, "unknown_140ec5cd0", fn_140ec5cd0 as *const ()), // sub_140ec5cd0
    Replacement::draft(0x140ec5d50, "unknown_140ec5d50", fn_140ec5d50 as *const ()), // sub_140ec5d50
    Replacement::draft(0x140ec5e30, "unknown_140ec5e30", fn_140ec5e30 as *const ()), // sub_140ec5e30
    Replacement::draft(0x140ec5f00, "unknown_140ec5f00", fn_140ec5f00 as *const ()), // sub_140ec5f00
    Replacement::draft(0x140ec6120, "unknown_140ec6120", fn_140ec6120 as *const ()), // sub_140ec6120
    Replacement::draft_throws(0x140ec6340, "unknown_140ec6340", fn_140ec6340 as *const (), Throws::Caught), // sub_140ec6340
    Replacement::draft_throws(0x140ec6630, "unknown_140ec6630", fn_140ec6630 as *const (), Throws::Caught), // sub_140ec6630
    Replacement::draft_throws(0x140ec6be0, "unknown_140ec6be0", fn_140ec6be0 as *const (), Throws::Caught), // sub_140ec6be0
    Replacement::draft_throws(0x140ec6ea0, "unknown_140ec6ea0", fn_140ec6ea0 as *const (), Throws::Caught), // sub_140ec6ea0
    Replacement::draft_throws(0x140ec7410, "unknown_140ec7410", fn_140ec7410 as *const (), Throws::Caught), // sub_140ec7410
    Replacement::draft_throws(0x140ec76d0, "unknown_140ec76d0", fn_140ec76d0 as *const (), Throws::Caught), // sub_140ec76d0
    Replacement::draft_throws(0x140ec7b10, "unknown_140ec7b10", fn_140ec7b10 as *const (), Throws::Caught), // sub_140ec7b10
    Replacement::draft_throws(0x140ec7c50, "unknown_140ec7c50", fn_140ec7c50 as *const (), Throws::Caught), // sub_140ec7c50
    Replacement::draft_throws(0x140ec7f30, "unknown_140ec7f30", fn_140ec7f30 as *const (), Throws::Caught), // sub_140ec7f30
    Replacement::draft_throws(0x140ec81e0, "unknown_140ec81e0", fn_140ec81e0 as *const (), Throws::Caught), // sub_140ec81e0
    Replacement::draft_throws(0x140ec8480, "unknown_140ec8480", fn_140ec8480 as *const (), Throws::Caught), // sub_140ec8480
    Replacement::draft_throws(0x140ec8590, "unknown_140ec8590", fn_140ec8590 as *const (), Throws::Caught), // sub_140ec8590
    Replacement::draft_throws(0x140ec8870, "unknown_140ec8870", fn_140ec8870 as *const (), Throws::Caught), // sub_140ec8870
    Replacement::draft_throws(0x140ec8b20, "unknown_140ec8b20", fn_140ec8b20 as *const (), Throws::Caught), // sub_140ec8b20
    Replacement::draft_throws(0x140ec8dd0, "unknown_140ec8dd0", fn_140ec8dd0 as *const (), Throws::Caught), // sub_140ec8dd0
    Replacement::draft(0x140ec8ea0, "unknown_140ec8ea0", tpl_8671::<0x20> as *const ()), // sub_140ec8ea0
    Replacement::draft(0x140ec8ec0, "unknown_140ec8ec0", tpl_8671::<0x10> as *const ()), // sub_140ec8ec0
    Replacement::draft(0x140ec8ee0, "unknown_140ec8ee0", fn_140ec8ee0 as *const ()), // sub_140ec8ee0
    Replacement::draft_throws(0x140ec9140, "unknown_140ec9140", fn_140ec9140 as *const (), Throws::Discard), // sub_140ec9140
    Replacement::draft_throws(0x140ec93d0, "unknown_140ec93d0", fn_140ec93d0 as *const (), Throws::Caught), // sub_140ec93d0
    Replacement::draft_throws(0x140ec9510, "unknown_140ec9510", fn_140ec9510 as *const (), Throws::Caught), // sub_140ec9510
    Replacement::draft_throws(0x140ec9bb0, "unknown_140ec9bb0", fn_140ec9bb0 as *const (), Throws::Caught), // sub_140ec9bb0
    Replacement::draft_throws(0x140eca200, "unknown_140eca200", fn_140eca200 as *const (), Throws::Caught), // sub_140eca200
    Replacement::draft(0x140eca850, "unknown_140eca850", fn_140eca850 as *const ()), // sub_140eca850
    Replacement::draft_throws(0x140ecaa90, "unknown_140ecaa90", fn_140ecaa90 as *const (), Throws::Caught), // sub_140ecaa90
    Replacement::draft(0x140ecad10, "unknown_140ecad10", tpl_4433::<0x19, 0x140ec5cd0, 0x20, 0x140ecae60, 0x10, 0x18, 0x8, 0x140ecad10> as *const ()), // sub_140ecad10
    Replacement::draft(0x140ecadd0, "unknown_140ecadd0", fn_140ecadd0 as *const ()), // sub_140ecadd0
    Replacement::draft(0x140ecae60, "unknown_140ecae60", fn_140ecae60 as *const ()), // sub_140ecae60
    Replacement::draft(0x140ecaf00, "unknown_140ecaf00", fn_140ecaf00 as *const ()), // sub_140ecaf00
    Replacement::draft(0x140ecaf40, "unknown_140ecaf40", fn_140ecaf40 as *const ()), // sub_140ecaf40
    Replacement::draft(0x140ecaf80, "unknown_140ecaf80", fn_140ecaf80 as *const ()), // sub_140ecaf80
    Replacement::draft_throws(0x140ecafc0, "unknown_140ecafc0", fn_140ecafc0 as *const (), Throws::Discard), // sub_140ecafc0
    Replacement::draft(0x140ecb140, "unknown_140ecb140", fn_140ecb140 as *const ()), // sub_140ecb140
    Replacement::draft(0x140ecb1d0, "unknown_140ecb1d0", fn_140ecb1d0 as *const ()), // sub_140ecb1d0
    Replacement::draft(0x140ecb260, "unknown_140ecb260", fn_140ecb260 as *const ()), // sub_140ecb260
    Replacement::draft(0x140ecb3a0, "unknown_140ecb3a0", fn_140ecb3a0 as *const ()), // sub_140ecb3a0
    Replacement::draft_throws(0x140ecb510, "unknown_140ecb510", fn_140ecb510 as *const (), Throws::Discard), // sub_140ecb510
    Replacement::draft(0x140ecb640, "unknown_140ecb640", fn_140ecb640 as *const ()), // sub_140ecb640
    Replacement::draft(0x140ecb740, "unknown_140ecb740", fn_140ecb740 as *const ()), // sub_140ecb740
    Replacement::draft_throws(0x140ecb880, "unknown_140ecb880", fn_140ecb880 as *const (), Throws::Discard), // sub_140ecb880
    Replacement::draft(0x140ecb9d0, "unknown_140ecb9d0", fn_140ecb9d0 as *const ()), // sub_140ecb9d0
    Replacement::draft_throws(0x140ecbc10, "unknown_140ecbc10", fn_140ecbc10 as *const (), Throws::Caught), // sub_140ecbc10
    Replacement::draft_throws(0x140ecc0d0, "unknown_140ecc0d0", fn_140ecc0d0 as *const (), Throws::Caught), // sub_140ecc0d0
    Replacement::draft_throws(0x140ecc390, "unknown_140ecc390", fn_140ecc390 as *const (), Throws::Caught), // sub_140ecc390
    Replacement::draft_throws(0x140ecc9d0, "unknown_140ecc9d0", fn_140ecc9d0 as *const (), Throws::Caught), // sub_140ecc9d0
    Replacement::draft_throws(0x140eccc60, "unknown_140eccc60", fn_140eccc60 as *const (), Throws::Caught), // sub_140eccc60
    Replacement::draft_throws(0x140ecd200, "unknown_140ecd200", fn_140ecd200 as *const (), Throws::Caught), // sub_140ecd200
    Replacement::draft(0x140ecd490, "unknown_140ecd490", fn_140ecd490 as *const ()), // sub_140ecd490
    Replacement::draft(0x140ecd590, "unknown_140ecd590", fn_140ecd590 as *const ()), // sub_140ecd590
    Replacement::draft_throws(0x140ecd6a0, "unknown_140ecd6a0", fn_140ecd6a0 as *const (), Throws::Discard), // sub_140ecd6a0
    Replacement::draft_throws(0x140ecd790, "unknown_140ecd790", fn_140ecd790 as *const (), Throws::Caught), // sub_140ecd790
    Replacement::draft_throws(0x140ecdc60, "unknown_140ecdc60", fn_140ecdc60 as *const (), Throws::Caught), // sub_140ecdc60
    Replacement::draft_throws(0x140ece0f0, "unknown_140ece0f0", fn_140ece0f0 as *const (), Throws::Caught), // sub_140ece0f0
    Replacement::draft_throws(0x140ece580, "unknown_140ece580", fn_140ece580 as *const (), Throws::Caught), // sub_140ece580
    Replacement::draft_throws(0x140ecea50, "unknown_140ecea50", fn_140ecea50 as *const (), Throws::Caught), // sub_140ecea50
    Replacement::draft_throws(0x140eceee0, "unknown_140eceee0", fn_140eceee0 as *const (), Throws::Caught), // sub_140eceee0
    Replacement::draft_throws(0x140ecf370, "unknown_140ecf370", fn_140ecf370 as *const (), Throws::Caught), // sub_140ecf370
    Replacement::draft_throws(0x140ecfc70, "unknown_140ecfc70", fn_140ecfc70 as *const (), Throws::Discard), // sub_140ecfc70
    Replacement::draft_throws(0x140ed0690, "unknown_140ed0690", fn_140ed0690 as *const (), Throws::Caught), // sub_140ed0690
    Replacement::draft_throws(0x140ed0fe0, "unknown_140ed0fe0", fn_140ed0fe0 as *const (), Throws::Caught), // sub_140ed0fe0
    Replacement::draft_throws(0x140ed1590, "unknown_140ed1590", fn_140ed1590 as *const (), Throws::Caught), // sub_140ed1590
    Replacement::draft_throws(0x140ed1fe0, "unknown_140ed1fe0", fn_140ed1fe0 as *const (), Throws::Caught), // sub_140ed1fe0
    Replacement::draft_throws(0x140ed2a30, "unknown_140ed2a30", fn_140ed2a30 as *const (), Throws::Caught), // sub_140ed2a30
    Replacement::draft_throws(0x140ed34a0, "unknown_140ed34a0", fn_140ed34a0 as *const (), Throws::Caught), // sub_140ed34a0
    Replacement::draft(0x140ed3640, "unknown_140ed3640", fn_140ed3640 as *const ()), // sub_140ed3640
    Replacement::draft_throws(0x140ed3820, "unknown_140ed3820", fn_140ed3820 as *const (), Throws::Caught), // sub_140ed3820
    Replacement::draft_throws(0x140ed38e0, "unknown_140ed38e0", fn_140ed38e0 as *const (), Throws::Caught), // sub_140ed38e0
    Replacement::draft_throws(0x140ed39a0, "unknown_140ed39a0", fn_140ed39a0 as *const (), Throws::Caught), // sub_140ed39a0
    Replacement::draft_throws(0x140ed3a60, "unknown_140ed3a60", fn_140ed3a60 as *const (), Throws::Discard), // sub_140ed3a60
    Replacement::draft_throws(0x140ed3b20, "unknown_140ed3b20", fn_140ed3b20 as *const (), Throws::Caught), // sub_140ed3b20
    Replacement::draft_throws(0x140ed3c60, "unknown_140ed3c60", fn_140ed3c60 as *const (), Throws::Caught), // sub_140ed3c60
    Replacement::draft_throws(0x140ed3da0, "unknown_140ed3da0", fn_140ed3da0 as *const (), Throws::Caught), // sub_140ed3da0
    Replacement::draft_throws(0x140ed3ee0, "unknown_140ed3ee0", fn_140ed3ee0 as *const (), Throws::Caught), // sub_140ed3ee0
    Replacement::draft(0x140ed40c0, "unknown_140ed40c0", fn_140ed40c0 as *const ()), // sub_140ed40c0
    Replacement::draft(0x140ed4200, "unknown_140ed4200", tpl_18500::<0x14183acc2, 0x14309f824, 0x1401ee280, 0x140ed7f70> as *const ()), // sub_140ed4200
    Replacement::draft(0x140ed4340, "unknown_140ed4340", tpl_18500::<0x14183acc2, 0x14309f824, 0x1401ee280, 0x140ed83b0> as *const ()), // sub_140ed4340
    Replacement::draft_throws(0x140ed4480, "unknown_140ed4480", fn_140ed4480 as *const (), Throws::Caught), // sub_140ed4480
    Replacement::draft_throws(0x140ed4660, "unknown_140ed4660", fn_140ed4660 as *const (), Throws::Discard), // sub_140ed4660
    Replacement::draft_throws(0x140ed4830, "unknown_140ed4830", fn_140ed4830 as *const (), Throws::Discard), // sub_140ed4830
    Replacement::draft(0x140ed49a0, "unknown_140ed49a0", fn_140ed49a0 as *const ()), // sub_140ed49a0
    Replacement::draft(0x140ed5300, "unknown_140ed5300", fn_140ed5300 as *const ()), // sub_140ed5300
    Replacement::draft(0x140ed5440, "unknown_140ed5440", fn_140ed5440 as *const ()), // sub_140ed5440
    Replacement::draft(0x140ed55d0, "unknown_140ed55d0", fn_140ed55d0 as *const ()), // sub_140ed55d0
    Replacement::draft(0x140ed5f30, "unknown_140ed5f30", tpl_855::<0x140ea7f90, 0x1425371a0, 0x142533200, 0x140ec5f00, 0x140ecadd0> as *const ()), // sub_140ed5f30
    Replacement::draft(0x140ed6090, "unknown_140ed6090", fn_140ed6090 as *const ()), // sub_140ed6090
    Replacement::draft(0x140ed60f0, "unknown_140ed60f0", fn_140ed60f0 as *const ()), // sub_140ed60f0
    Replacement::draft_throws(0x140ed6160, "unknown_140ed6160", fn_140ed6160 as *const (), Throws::Discard), // sub_140ed6160
    Replacement::draft_throws(0x140ed61f0, "unknown_140ed61f0", fn_140ed61f0 as *const (), Throws::Discard), // sub_140ed61f0
    Replacement::draft_throws(0x140ed62b0, "unknown_140ed62b0", fn_140ed62b0 as *const (), Throws::Discard), // sub_140ed62b0
    Replacement::draft_throws(0x140ed6320, "unknown_140ed6320", fn_140ed6320 as *const (), Throws::Caught), // sub_140ed6320
    Replacement::draft_throws(0x140ed65e0, "unknown_140ed65e0", fn_140ed65e0 as *const (), Throws::Caught), // sub_140ed65e0
    Replacement::draft_throws(0x140ed68a0, "unknown_140ed68a0", fn_140ed68a0 as *const (), Throws::Caught), // sub_140ed68a0
    Replacement::draft_throws(0x140ed6b60, "unknown_140ed6b60", fn_140ed6b60 as *const (), Throws::Caught), // sub_140ed6b60
    Replacement::draft_throws(0x140ed6f00, "unknown_140ed6f00", fn_140ed6f00 as *const (), Throws::Caught), // sub_140ed6f00
    Replacement::draft_throws(0x140ed7370, "unknown_140ed7370", fn_140ed7370 as *const (), Throws::Caught), // sub_140ed7370
    Replacement::draft_throws(0x140ed77b0, "unknown_140ed77b0", fn_140ed77b0 as *const (), Throws::Caught), // sub_140ed77b0
    Replacement::draft(0x140ed7b00, "unknown_140ed7b00", fn_140ed7b00 as *const ()), // sub_140ed7b00
    Replacement::draft(0x140ed7f70, "unknown_140ed7f70", fn_140ed7f70 as *const ()), // sub_140ed7f70
    Replacement::draft(0x140ed83b0, "unknown_140ed83b0", fn_140ed83b0 as *const ()), // sub_140ed83b0
    Replacement::draft_throws(0x140ed8700, "unknown_140ed8700", fn_140ed8700 as *const (), Throws::Caught), // sub_140ed8700
    Replacement::draft_throws(0x140ed89c0, "unknown_140ed89c0", fn_140ed89c0 as *const (), Throws::Caught), // sub_140ed89c0
    Replacement::draft_throws(0x140ed8c80, "unknown_140ed8c80", fn_140ed8c80 as *const (), Throws::Caught), // sub_140ed8c80
    Replacement::draft_throws(0x140ed8f40, "unknown_140ed8f40", fn_140ed8f40 as *const (), Throws::Caught), // sub_140ed8f40
    Replacement::draft_throws(0x140ed9200, "unknown_140ed9200", fn_140ed9200 as *const (), Throws::Caught), // sub_140ed9200
    Replacement::draft(0x140ed94c0, "unknown_140ed94c0", tpl_1663::<0x8, 0x140ec5c40, 0x1401e66f0> as *const ()), // sub_140ed94c0
    Replacement::draft_throws(0x140ed9500, "unknown_140ed9500", fn_140ed9500 as *const (), Throws::Caught), // sub_140ed9500
    Replacement::draft_throws(0x140ed9960, "unknown_140ed9960", fn_140ed9960 as *const (), Throws::Caught), // sub_140ed9960
    Replacement::draft_throws(0x140ed9da0, "unknown_140ed9da0", fn_140ed9da0 as *const (), Throws::Caught), // sub_140ed9da0
    Replacement::draft_throws(0x140eda1d0, "unknown_140eda1d0", fn_140eda1d0 as *const (), Throws::Caught), // sub_140eda1d0
    Replacement::draft_throws(0x140eda5e0, "unknown_140eda5e0", fn_140eda5e0 as *const (), Throws::Caught), // sub_140eda5e0
    Replacement::draft_throws(0x140edaa00, "unknown_140edaa00", fn_140edaa00 as *const (), Throws::Caught), // sub_140edaa00
    Replacement::draft_throws(0x140edae00, "unknown_140edae00", fn_140edae00 as *const (), Throws::Caught), // sub_140edae00
    Replacement::draft_throws(0x140edaed0, "unknown_140edaed0", fn_140edaed0 as *const (), Throws::Caught), // sub_140edaed0
    Replacement::draft_throws(0x140edafa0, "unknown_140edafa0", fn_140edafa0 as *const (), Throws::Caught), // sub_140edafa0
    Replacement::draft_throws(0x140edb6b0, "unknown_140edb6b0", fn_140edb6b0 as *const (), Throws::Caught), // sub_140edb6b0
    Replacement::draft_throws(0x140edbd80, "unknown_140edbd80", fn_140edbd80 as *const (), Throws::Caught), // sub_140edbd80
    Replacement::draft(0x140edc1c0, "unknown_140edc1c0", tpl_12555::<0x14387fe18, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0xc, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140edc1c0
    Replacement::draft(0x140edc2a0, "unknown_140edc2a0", tpl_18530::<0x14387fe28, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140edc2a0
    Replacement::draft_throws(0x140edc370, "unknown_140edc370", fn_140edc370 as *const (), Throws::Caught), // sub_140edc370
    Replacement::draft_throws(0x140edc840, "unknown_140edc840", fn_140edc840 as *const (), Throws::Caught), // sub_140edc840
    Replacement::draft_throws(0x140edcd10, "unknown_140edcd10", fn_140edcd10 as *const (), Throws::Caught), // sub_140edcd10
    Replacement::draft_throws(0x140edd1a0, "unknown_140edd1a0", fn_140edd1a0 as *const (), Throws::Caught), // sub_140edd1a0
    Replacement::draft_throws(0x140edd620, "unknown_140edd620", fn_140edd620 as *const (), Throws::Caught), // sub_140edd620
    Replacement::draft_throws(0x140eddab0, "unknown_140eddab0", fn_140eddab0 as *const (), Throws::Caught), // sub_140eddab0
    Replacement::draft_throws(0x140eddf30, "unknown_140eddf30", tpl_71031::<0x14387fe20, 0x14387fe18, 0x1401e9fa0, 0x328, 0x338, 0x38, 0x4, 0x8, 0x369, 0xc, 0x1401e9e70> as *const (), Throws::Discard), // sub_140eddf30
    Replacement::draft(0x140ede000, "unknown_140ede000", tpl_18531::<0x14387fe18, 0x14243b0b0, 0x338, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ede000
    Replacement::draft_throws(0x140ede090, "unknown_140ede090", fn_140ede090 as *const (), Throws::Discard), // sub_140ede090
    Replacement::draft(0x140ede140, "unknown_140ede140", fn_140ede140 as *const ()), // sub_140ede140
    Replacement::draft(0x140ede1c0, "unknown_140ede1c0", tpl_12556::<0x14387fe18, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ede1c0
    Replacement::draft(0x140ede260, "unknown_140ede260", tpl_12961::<0x14387fe28, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ede260
    Replacement::draft_throws(0x140ede2e0, "unknown_140ede2e0", tpl_12557::<0x14387fe20, 0x14387fe18, 0x1401e9b60, 0x328, 0x338, 0x120, 0x8, 0x369, 0xc, 0x4, 0x1401e9a20> as *const (), Throws::Discard), // sub_140ede2e0
    Replacement::draft_throws(0x140ede3b0, "unknown_140ede3b0", fn_140ede3b0 as *const (), Throws::Discard), // sub_140ede3b0
    Replacement::draft_throws(0x140ede470, "unknown_140ede470", fn_140ede470 as *const (), Throws::Caught), // sub_140ede470
    Replacement::draft_throws(0x140ede6b0, "unknown_140ede6b0", fn_140ede6b0 as *const (), Throws::Caught), // sub_140ede6b0
    Replacement::draft_throws(0x140ede900, "unknown_140ede900", fn_140ede900 as *const (), Throws::Caught), // sub_140ede900
    Replacement::draft_throws(0x140edeb40, "unknown_140edeb40", fn_140edeb40 as *const (), Throws::Caught), // sub_140edeb40
    Replacement::draft_throws(0x140eded90, "unknown_140eded90", fn_140eded90 as *const (), Throws::Caught), // sub_140eded90
    Replacement::draft_throws(0x140edefd0, "unknown_140edefd0", fn_140edefd0 as *const (), Throws::Caught), // sub_140edefd0
    Replacement::draft(0x140edf220, "unknown_140edf220", tpl_12562::<0x14387fe20, 0x14387fe18, 0x1401e9970, 0x338, 0xd8, 0x4, 0x8, 0xc, 0x1401e9860> as *const ()), // sub_140edf220
    Replacement::draft(0x140edf2c0, "unknown_140edf2c0", tpl_12962::<0x14387fe0c, 0x14387fe28, 0x1401e9970, 0x338, 0xd8, 0x4, 0x1401e9860> as *const ()), // sub_140edf2c0
    Replacement::draft(0x140edf330, "unknown_140edf330", tpl_12556::<0x14387fe18, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140edf330
    Replacement::draft(0x140edf3e0, "unknown_140edf3e0", tpl_12961::<0x14387fe28, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140edf3e0
    Replacement::draft_throws(0x140edf460, "unknown_140edf460", tpl_12563::<0x14387fe20, 0x14387fe18, 0x1401e9770, 0x338, 0x120, 0x8, 0xc, 0x4, 0x1401e9590> as *const (), Throws::Discard), // sub_140edf460
    Replacement::draft_throws(0x140edf4f0, "unknown_140edf4f0", fn_140edf4f0 as *const (), Throws::Discard), // sub_140edf4f0
    Replacement::draft_throws(0x140edf580, "unknown_140edf580", fn_140edf580 as *const (), Throws::Caught), // sub_140edf580
    Replacement::draft_throws(0x140edf770, "unknown_140edf770", fn_140edf770 as *const (), Throws::Caught), // sub_140edf770
    Replacement::draft_throws(0x140edf960, "unknown_140edf960", fn_140edf960 as *const (), Throws::Caught), // sub_140edf960
    Replacement::draft_throws(0x140ee0170, "unknown_140ee0170", fn_140ee0170 as *const (), Throws::Caught), // sub_140ee0170
    Replacement::draft_throws(0x140ee06e0, "unknown_140ee06e0", fn_140ee06e0 as *const (), Throws::Caught), // sub_140ee06e0
    Replacement::draft_throws(0x140ee0e80, "unknown_140ee0e80", fn_140ee0e80 as *const (), Throws::Caught), // sub_140ee0e80
    Replacement::draft_throws(0x140ee13b0, "unknown_140ee13b0", fn_140ee13b0 as *const (), Throws::Caught), // sub_140ee13b0
    Replacement::draft_throws(0x140ee18e0, "unknown_140ee18e0", fn_140ee18e0 as *const (), Throws::Caught), // sub_140ee18e0
    Replacement::draft_throws(0x140ee1c30, "unknown_140ee1c30", tpl_18545::<0x14387fe18, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x20, 0x4, 0x8, 0x14329fb30, 0x1413c41a0, 0x1433038c8, 0x14020f240> as *const (), Throws::Discard), // sub_140ee1c30
    Replacement::draft_throws(0x140ee1d40, "unknown_140ee1d40", fn_140ee1d40 as *const (), Throws::Discard), // sub_140ee1d40
    Replacement::draft(0x140ee1e40, "unknown_140ee1e40", fn_140ee1e40 as *const ()), // sub_140ee1e40
    Replacement::draft_throws(0x140ee20d0, "unknown_140ee20d0", fn_140ee20d0 as *const (), Throws::Caught), // sub_140ee20d0
    Replacement::draft_throws(0x140ee22f0, "unknown_140ee22f0", fn_140ee22f0 as *const (), Throws::Caught), // sub_140ee22f0
    Replacement::draft_throws(0x140ee2400, "unknown_140ee2400", fn_140ee2400 as *const (), Throws::Caught), // sub_140ee2400
    Replacement::draft_throws(0x140ee25f0, "unknown_140ee25f0", fn_140ee25f0 as *const (), Throws::Caught), // sub_140ee25f0
    Replacement::draft_throws(0x140ee2700, "unknown_140ee2700", fn_140ee2700 as *const (), Throws::Caught), // sub_140ee2700
    Replacement::draft_throws(0x140ee28f0, "unknown_140ee28f0", fn_140ee28f0 as *const (), Throws::Caught), // sub_140ee28f0
    Replacement::draft(0x140ee2a00, "unknown_140ee2a00", fn_140ee2a00 as *const ()), // sub_140ee2a00
    Replacement::draft(0x140ee2a70, "unknown_140ee2a70", fn_140ee2a70 as *const ()), // sub_140ee2a70
    Replacement::draft_throws(0x140ee2ae0, "unknown_140ee2ae0", tpl_18554::<0x14329fb30, 0x1402c4e80, 0xc, 0x1433038c8, 0x14020f270> as *const (), Throws::Discard), // sub_140ee2ae0
    Replacement::draft(0x140ee2b50, "unknown_140ee2b50", fn_140ee2b50 as *const ()), // sub_140ee2b50
    Replacement::draft(0x140ee2be0, "unknown_140ee2be0", fn_140ee2be0 as *const ()), // sub_140ee2be0
    Replacement::draft(0x140ee2c90, "unknown_140ee2c90", fn_140ee2c90 as *const ()), // sub_140ee2c90
    Replacement::draft(0x140ee2d80, "unknown_140ee2d80", fn_140ee2d80 as *const ()), // sub_140ee2d80
    Replacement::draft_throws(0x140ee3120, "unknown_140ee3120", fn_140ee3120 as *const (), Throws::Caught), // sub_140ee3120
    Replacement::draft(0x140ee3370, "unknown_140ee3370", fn_140ee3370 as *const ()), // sub_140ee3370
    Replacement::draft_throws(0x140ee3560, "unknown_140ee3560", fn_140ee3560 as *const (), Throws::Caught), // sub_140ee3560
    Replacement::draft(0x140ee38e0, "unknown_140ee38e0", fn_140ee38e0 as *const ()), // sub_140ee38e0
    Replacement::draft_throws(0x140ee39d0, "unknown_140ee39d0", fn_140ee39d0 as *const (), Throws::Caught), // sub_140ee39d0
    Replacement::draft_throws(0x140ee3ae0, "unknown_140ee3ae0", fn_140ee3ae0 as *const (), Throws::Caught), // sub_140ee3ae0
    Replacement::draft_throws(0x140ee3bf0, "unknown_140ee3bf0", fn_140ee3bf0 as *const (), Throws::Caught), // sub_140ee3bf0
    Replacement::draft(0x140ee3d00, "unknown_140ee3d00", fn_140ee3d00 as *const ()), // sub_140ee3d00
    Replacement::draft(0x140ee3fe0, "unknown_140ee3fe0", fn_140ee3fe0 as *const ()), // sub_140ee3fe0
    Replacement::draft(0x140ee41d0, "unknown_140ee41d0", fn_140ee41d0 as *const ()), // sub_140ee41d0
    Replacement::draft_throws(0x140ee43a0, "unknown_140ee43a0", fn_140ee43a0 as *const (), Throws::Caught), // sub_140ee43a0
    Replacement::draft_throws(0x140ee4550, "unknown_140ee4550", fn_140ee4550 as *const (), Throws::Discard), // sub_140ee4550
    Replacement::draft(0x140ee4660, "unknown_140ee4660", fn_140ee4660 as *const ()), // sub_140ee4660
    Replacement::draft(0x140ee4720, "unknown_140ee4720", fn_140ee4720 as *const ()), // sub_140ee4720
    Replacement::draft(0x140ee4810, "unknown_140ee4810", fn_140ee4810 as *const ()), // sub_140ee4810
    Replacement::draft(0x140ee4890, "unknown_140ee4890", fn_140ee4890 as *const ()), // sub_140ee4890
    Replacement::draft_throws(0x140ee4930, "unknown_140ee4930", fn_140ee4930 as *const (), Throws::Discard), // sub_140ee4930
    Replacement::draft(0x140ee49e0, "unknown_140ee49e0", fn_140ee49e0 as *const ()), // sub_140ee49e0
    Replacement::draft_throws(0x140ee4aa0, "unknown_140ee4aa0", fn_140ee4aa0 as *const (), Throws::Discard), // sub_140ee4aa0
    Replacement::draft(0x140ee4b60, "unknown_140ee4b60", fn_140ee4b60 as *const ()), // sub_140ee4b60
    Replacement::draft(0x140ee4c60, "unknown_140ee4c60", fn_140ee4c60 as *const ()), // sub_140ee4c60
    Replacement::draft_throws(0x140ee4ce0, "unknown_140ee4ce0", fn_140ee4ce0 as *const (), Throws::Discard), // sub_140ee4ce0
    Replacement::draft(0x140ee4dd0, "unknown_140ee4dd0", fn_140ee4dd0 as *const ()), // sub_140ee4dd0
    Replacement::draft(0x140ee4e70, "unknown_140ee4e70", fn_140ee4e70 as *const ()), // sub_140ee4e70
    Replacement::draft(0x140ee4f20, "unknown_140ee4f20", fn_140ee4f20 as *const ()), // sub_140ee4f20
    Replacement::draft_throws(0x140ee5070, "unknown_140ee5070", fn_140ee5070 as *const (), Throws::Caught), // sub_140ee5070
    Replacement::draft_throws(0x140ee53d0, "unknown_140ee53d0", fn_140ee53d0 as *const (), Throws::Caught), // sub_140ee53d0
    Replacement::draft_throws(0x140ee5800, "unknown_140ee5800", fn_140ee5800 as *const (), Throws::Caught), // sub_140ee5800
    Replacement::draft_throws(0x140ee5b20, "unknown_140ee5b20", fn_140ee5b20 as *const (), Throws::Caught), // sub_140ee5b20
    Replacement::draft_throws(0x140ee5f20, "unknown_140ee5f20", fn_140ee5f20 as *const (), Throws::Caught), // sub_140ee5f20
    Replacement::draft_throws(0x140ee6230, "unknown_140ee6230", fn_140ee6230 as *const (), Throws::Caught), // sub_140ee6230
    Replacement::draft_throws(0x140ee6630, "unknown_140ee6630", fn_140ee6630 as *const (), Throws::Caught), // sub_140ee6630
    Replacement::draft_throws(0x140ee6ce0, "unknown_140ee6ce0", fn_140ee6ce0 as *const (), Throws::Caught), // sub_140ee6ce0
    Replacement::draft_throws(0x140ee7310, "unknown_140ee7310", fn_140ee7310 as *const (), Throws::Caught), // sub_140ee7310
    Replacement::draft(0x140ee78d0, "unknown_140ee78d0", fn_140ee78d0 as *const ()), // sub_140ee78d0
    Replacement::draft(0x140ee7930, "unknown_140ee7930", tpl_2030::<0x140ee79e0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140ee7930
    Replacement::draft(0x140ee79e0, "unknown_140ee79e0", fn_140ee79e0 as *const ()), // sub_140ee79e0
    Replacement::draft(0x140ee7a50, "unknown_140ee7a50", fn_140ee7a50 as *const ()), // sub_140ee7a50
    Replacement::draft(0x140ee7ae0, "unknown_140ee7ae0", fn_140ee7ae0 as *const ()), // sub_140ee7ae0
    Replacement::draft(0x140ee7b70, "unknown_140ee7b70", fn_140ee7b70 as *const ()), // sub_140ee7b70
    Replacement::draft_throws(0x140ee7c50, "unknown_140ee7c50", fn_140ee7c50 as *const (), Throws::Discard), // sub_140ee7c50
    Replacement::draft(0x140ee7d60, "unknown_140ee7d60", tpl_18586::<0x14387fe70, 0x14243b0b0, 0x338, 0x38, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ee7d60
    Replacement::draft_throws(0x140ee7e30, "unknown_140ee7e30", fn_140ee7e30 as *const (), Throws::Discard), // sub_140ee7e30
    Replacement::draft_throws(0x140ee7f20, "unknown_140ee7f20", fn_140ee7f20 as *const (), Throws::Caught), // sub_140ee7f20
    Replacement::draft_throws(0x140ee8150, "unknown_140ee8150", fn_140ee8150 as *const (), Throws::Caught), // sub_140ee8150
    Replacement::draft_throws(0x140ee8450, "unknown_140ee8450", fn_140ee8450 as *const (), Throws::Caught), // sub_140ee8450
    Replacement::draft(0x140ee8730, "unknown_140ee8730", fn_140ee8730 as *const ()), // sub_140ee8730
    Replacement::draft(0x140ee87d0, "unknown_140ee87d0", fn_140ee87d0 as *const ()), // sub_140ee87d0
    Replacement::draft(0x140ee88b0, "unknown_140ee88b0", tpl_18586::<0x14387fe70, 0x14243b0b0, 0x338, 0xd8, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140ee88b0
    Replacement::draft_throws(0x140ee8990, "unknown_140ee8990", fn_140ee8990 as *const (), Throws::Discard), // sub_140ee8990
    Replacement::draft_throws(0x140ee8a30, "unknown_140ee8a30", fn_140ee8a30 as *const (), Throws::Discard), // sub_140ee8a30
    Replacement::draft(0x140ee8b60, "unknown_140ee8b60", fn_140ee8b60 as *const ()), // sub_140ee8b60
    Replacement::draft(0x140ee8bf0, "unknown_140ee8bf0", fn_140ee8bf0 as *const ()), // sub_140ee8bf0
    Replacement::draft(0x140ee8c90, "unknown_140ee8c90", fn_140ee8c90 as *const ()), // sub_140ee8c90
    Replacement::draft(0x140ee8d40, "unknown_140ee8d40", fn_140ee8d40 as *const ()), // sub_140ee8d40
    Replacement::draft(0x140ee8e30, "unknown_140ee8e30", fn_140ee8e30 as *const ()), // sub_140ee8e30
    Replacement::draft_throws(0x140ee9020, "unknown_140ee9020", fn_140ee9020 as *const (), Throws::Discard), // sub_140ee9020
    Replacement::draft_throws(0x140ee9210, "unknown_140ee9210", fn_140ee9210 as *const (), Throws::Discard), // sub_140ee9210
    Replacement::draft_throws(0x140ee9340, "unknown_140ee9340", fn_140ee9340 as *const (), Throws::Discard), // sub_140ee9340
    Replacement::draft(0x140ee9540, "unknown_140ee9540", fn_140ee9540 as *const ()), // sub_140ee9540
    Replacement::draft_throws(0x140ee9650, "unknown_140ee9650", fn_140ee9650 as *const (), Throws::Caught), // sub_140ee9650
    Replacement::draft_throws(0x140ee9a60, "unknown_140ee9a60", fn_140ee9a60 as *const (), Throws::Caught), // sub_140ee9a60
    Replacement::draft_throws(0x140ee9e40, "unknown_140ee9e40", fn_140ee9e40 as *const (), Throws::Caught), // sub_140ee9e40
    Replacement::draft(0x140eea210, "unknown_140eea210", fn_140eea210 as *const ()), // sub_140eea210
    Replacement::draft(0x140eea380, "unknown_140eea380", fn_140eea380 as *const ()), // sub_140eea380
    Replacement::draft(0x140eea4b0, "unknown_140eea4b0", fn_140eea4b0 as *const ()), // sub_140eea4b0
    Replacement::draft_throws(0x140eea5e0, "unknown_140eea5e0", fn_140eea5e0 as *const (), Throws::Discard), // sub_140eea5e0
    Replacement::draft_throws(0x140eea710, "unknown_140eea710", fn_140eea710 as *const (), Throws::Caught), // sub_140eea710
    Replacement::draft(0x140eea920, "unknown_140eea920", fn_140eea920 as *const ()), // sub_140eea920
    Replacement::draft(0x140eea9c0, "unknown_140eea9c0", fn_140eea9c0 as *const ()), // sub_140eea9c0
    Replacement::draft_throws(0x140eeaab0, "unknown_140eeaab0", fn_140eeaab0 as *const (), Throws::Discard), // sub_140eeaab0
    Replacement::draft(0x140eeab70, "unknown_140eeab70", fn_140eeab70 as *const ()), // sub_140eeab70
    Replacement::draft_throws(0x140eeabf0, "unknown_140eeabf0", fn_140eeabf0 as *const (), Throws::Caught), // sub_140eeabf0
    Replacement::draft_throws(0x140eeac90, "unknown_140eeac90", fn_140eeac90 as *const (), Throws::Caught), // sub_140eeac90
    Replacement::draft_throws(0x140eead30, "unknown_140eead30", fn_140eead30 as *const (), Throws::Caught), // sub_140eead30
    Replacement::draft(0x140eeadd0, "unknown_140eeadd0", fn_140eeadd0 as *const ()), // sub_140eeadd0
    Replacement::draft(0x140eeae90, "unknown_140eeae90", fn_140eeae90 as *const ()), // sub_140eeae90
    Replacement::draft(0x140eeaf40, "unknown_140eeaf40", fn_140eeaf40 as *const ()), // sub_140eeaf40
    Replacement::draft_throws(0x140eeaff0, "unknown_140eeaff0", fn_140eeaff0 as *const (), Throws::Discard), // sub_140eeaff0
    Replacement::draft_throws(0x140eeb0a0, "unknown_140eeb0a0", tpl_17357::<0x14387fe50, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1433054a8, 0x140eef820> as *const (), Throws::Discard), // sub_140eeb0a0
    Replacement::draft(0x140eeb140, "unknown_140eeb140", fn_140eeb140 as *const ()), // sub_140eeb140
    Replacement::draft(0x140eeb200, "unknown_140eeb200", fn_140eeb200 as *const ()), // sub_140eeb200
    Replacement::draft(0x140eeb300, "unknown_140eeb300", tpl_12891::<0x14387fe58, 0x14243b0b0, 0x3, 0x8, 0x4, 0x338, 0x1401ee7c0, 0x110, 0x10b, 0x10c, 0x108, 0xfffffffffffffff8> as *const ()), // sub_140eeb300
    Replacement::draft_throws(0x140eeb400, "unknown_140eeb400", tpl_71382::<0x14387fe68, 0x14387fe58, 0x1401e9fa0, 0x1426cdcb0, 0x328, 0x338, 0x20, 0x369, 0x142409b50, 0x108, 0x1401e9e70> as *const (), Throws::Discard), // sub_140eeb400
    Replacement::draft(0x140eeb500, "unknown_140eeb500", tpl_12961::<0x14387fe58, 0x14243b0b0, 0x338, 0x20, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140eeb500
    Replacement::draft_throws(0x140eeb580, "unknown_140eeb580", tpl_71384::<0x14387fe68, 0x14387fe58, 0x1401e9b60, 0x1426cdcb0, 0x328, 0x338, 0x1424b4da0, 0x369, 0x142409b50, 0x108, 0x1401e9a20> as *const (), Throws::Discard), // sub_140eeb580
    Replacement::draft_throws(0x140eeb680, "unknown_140eeb680", fn_140eeb680 as *const (), Throws::Caught), // sub_140eeb680
    Replacement::draft_throws(0x140eeb900, "unknown_140eeb900", fn_140eeb900 as *const (), Throws::Caught), // sub_140eeb900
    Replacement::draft_throws(0x140eebb40, "unknown_140eebb40", fn_140eebb40 as *const (), Throws::Caught), // sub_140eebb40
    Replacement::draft(0x140eebd70, "unknown_140eebd70", tpl_17887::<0x14387fe58, 0x14243b0b0, 0x338, 0x1426cdcb0, 0x142409b50, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140eebd70
    Replacement::draft(0x140eebe40, "unknown_140eebe40", tpl_12962::<0x14387fe68, 0x14387fe58, 0x1401e9970, 0x338, 0xc0, 0x108, 0x1401e9860> as *const ()), // sub_140eebe40
    Replacement::draft(0x140eebec0, "unknown_140eebec0", tpl_12961::<0x14387fe58, 0x14243b0b0, 0x338, 0xc0, 0x108, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140eebec0
    Replacement::draft_throws(0x140eebf40, "unknown_140eebf40", tpl_12963::<0x14387fe68, 0x14387fe58, 0x1401e9770, 0x338, 0x1424b4cb0, 0x108, 0x1401e9590> as *const (), Throws::Discard), // sub_140eebf40
    Replacement::draft_throws(0x140eebfb0, "unknown_140eebfb0", tpl_17888::<0x14387fe58, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1426cdcb0, 0x143305510, 0x1402c4730, 0x108, 0x1432a26e8> as *const (), Throws::Discard), // sub_140eebfb0
    Replacement::draft(0x140eec090, "unknown_140eec090", fn_140eec090 as *const ()), // sub_140eec090
    Replacement::draft(0x140eec130, "unknown_140eec130", fn_140eec130 as *const ()), // sub_140eec130
    Replacement::draft(0x140eec1d0, "unknown_140eec1d0", fn_140eec1d0 as *const ()), // sub_140eec1d0
    Replacement::draft(0x140eec2a0, "unknown_140eec2a0", fn_140eec2a0 as *const ()), // sub_140eec2a0
    Replacement::draft(0x140eec430, "unknown_140eec430", fn_140eec430 as *const ()), // sub_140eec430
    Replacement::draft_throws(0x140eec6e0, "unknown_140eec6e0", fn_140eec6e0 as *const (), Throws::Discard), // sub_140eec6e0
    Replacement::draft_throws(0x140eec8a0, "unknown_140eec8a0", fn_140eec8a0 as *const (), Throws::Discard), // sub_140eec8a0
    Replacement::draft_throws(0x140eec9a0, "unknown_140eec9a0", fn_140eec9a0 as *const (), Throws::Discard), // sub_140eec9a0
    Replacement::draft_throws(0x140eecb70, "unknown_140eecb70", fn_140eecb70 as *const (), Throws::Discard), // sub_140eecb70
    Replacement::draft_throws(0x140eecc00, "unknown_140eecc00", fn_140eecc00 as *const (), Throws::Caught), // sub_140eecc00
    Replacement::draft_throws(0x140eecef0, "unknown_140eecef0", fn_140eecef0 as *const (), Throws::Caught), // sub_140eecef0
    Replacement::draft_throws(0x140eed1b0, "unknown_140eed1b0", fn_140eed1b0 as *const (), Throws::Caught), // sub_140eed1b0
    Replacement::draft(0x140eed460, "unknown_140eed460", fn_140eed460 as *const ()), // sub_140eed460
    Replacement::draft(0x140eed6b0, "unknown_140eed6b0", fn_140eed6b0 as *const ()), // sub_140eed6b0
    Replacement::draft(0x140eed830, "unknown_140eed830", fn_140eed830 as *const ()), // sub_140eed830
    Replacement::draft_throws(0x140eed9c0, "unknown_140eed9c0", fn_140eed9c0 as *const (), Throws::Discard), // sub_140eed9c0
    Replacement::draft_throws(0x140eedb30, "unknown_140eedb30", fn_140eedb30 as *const (), Throws::Discard), // sub_140eedb30
    Replacement::draft(0x140eedc90, "unknown_140eedc90", fn_140eedc90 as *const ()), // sub_140eedc90
    Replacement::draft(0x140eedd20, "unknown_140eedd20", fn_140eedd20 as *const ()), // sub_140eedd20
    Replacement::draft(0x140eeddb0, "unknown_140eeddb0", fn_140eeddb0 as *const ()), // sub_140eeddb0
    Replacement::draft_throws(0x140eedea0, "unknown_140eedea0", fn_140eedea0 as *const (), Throws::Discard), // sub_140eedea0
    Replacement::draft_throws(0x140eedf60, "unknown_140eedf60", fn_140eedf60 as *const (), Throws::Discard), // sub_140eedf60
    Replacement::draft_throws(0x140eee010, "unknown_140eee010", fn_140eee010 as *const (), Throws::Caught), // sub_140eee010
    Replacement::draft_throws(0x140eee0b0, "unknown_140eee0b0", fn_140eee0b0 as *const (), Throws::Caught), // sub_140eee0b0
    Replacement::draft_throws(0x140eee150, "unknown_140eee150", fn_140eee150 as *const (), Throws::Caught), // sub_140eee150
    Replacement::draft(0x140eee1f0, "unknown_140eee1f0", fn_140eee1f0 as *const ()), // sub_140eee1f0
    Replacement::draft(0x140eee2b0, "unknown_140eee2b0", fn_140eee2b0 as *const ()), // sub_140eee2b0
    Replacement::draft(0x140eee360, "unknown_140eee360", fn_140eee360 as *const ()), // sub_140eee360
    Replacement::draft_throws(0x140eee410, "unknown_140eee410", fn_140eee410 as *const (), Throws::Discard), // sub_140eee410
    Replacement::draft_throws(0x140eee4c0, "unknown_140eee4c0", tpl_17357::<0x14387fe78, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x1432f8478, 0x140eefc20> as *const (), Throws::Discard), // sub_140eee4c0
    Replacement::draft(0x140eee560, "unknown_140eee560", fn_140eee560 as *const ()), // sub_140eee560
    Replacement::draft_throws(0x140eee630, "unknown_140eee630", fn_140eee630 as *const (), Throws::Caught), // sub_140eee630
    Replacement::draft(0x140eee800, "unknown_140eee800", fn_140eee800 as *const ()), // sub_140eee800
    Replacement::draft_throws(0x140eee8c0, "unknown_140eee8c0", fn_140eee8c0 as *const (), Throws::Caught), // sub_140eee8c0
    Replacement::draft_throws(0x140eeeb10, "unknown_140eeeb10", fn_140eeeb10 as *const (), Throws::Caught), // sub_140eeeb10
    Replacement::draft_throws(0x140eeed60, "unknown_140eeed60", fn_140eeed60 as *const (), Throws::Caught), // sub_140eeed60
    Replacement::draft_throws(0x140eeefc0, "unknown_140eeefc0", fn_140eeefc0 as *const (), Throws::Caught), // sub_140eeefc0
    Replacement::draft_throws(0x140eef220, "unknown_140eef220", fn_140eef220 as *const (), Throws::Caught), // sub_140eef220
    Replacement::draft_throws(0x140eef480, "unknown_140eef480", fn_140eef480 as *const (), Throws::Caught), // sub_140eef480
    Replacement::draft_throws(0x140eef630, "unknown_140eef630", fn_140eef630 as *const (), Throws::Caught), // sub_140eef630
    Replacement::draft_throws(0x140eef820, "unknown_140eef820", fn_140eef820 as *const (), Throws::Discard), // sub_140eef820
    Replacement::draft_throws(0x140eef880, "unknown_140eef880", fn_140eef880 as *const (), Throws::Caught), // sub_140eef880
    Replacement::draft_throws(0x140eefa30, "unknown_140eefa30", fn_140eefa30 as *const (), Throws::Caught), // sub_140eefa30
    Replacement::draft(0x140eefc60, "unknown_140eefc60", fn_140eefc60 as *const ()), // sub_140eefc60
    Replacement::draft(0x140eefe80, "unknown_140eefe80", fn_140eefe80 as *const ()), // sub_140eefe80
    Replacement::draft(0x140eeff50, "unknown_140eeff50", fn_140eeff50 as *const ()), // sub_140eeff50
    Replacement::draft(0x140ef0240, "unknown_140ef0240", fn_140ef0240 as *const ()), // sub_140ef0240
    Replacement::draft(0x140ef0360, "unknown_140ef0360", fn_140ef0360 as *const ()), // sub_140ef0360
    Replacement::draft_throws(0x140ef03c0, "unknown_140ef03c0", fn_140ef03c0 as *const (), Throws::Caught), // sub_140ef03c0
    Replacement::draft_throws(0x140ef0c30, "unknown_140ef0c30", fn_140ef0c30 as *const (), Throws::Caught), // sub_140ef0c30
    Replacement::draft_throws(0x140ef12b0, "unknown_140ef12b0", fn_140ef12b0 as *const (), Throws::Caught), // sub_140ef12b0
    Replacement::draft_throws(0x140ef16f0, "unknown_140ef16f0", fn_140ef16f0 as *const (), Throws::Caught), // sub_140ef16f0
    Replacement::draft_throws(0x140ef19f0, "unknown_140ef19f0", fn_140ef19f0 as *const (), Throws::Caught), // sub_140ef19f0
    Replacement::draft_throws(0x140ef1cb0, "unknown_140ef1cb0", fn_140ef1cb0 as *const (), Throws::Caught), // sub_140ef1cb0
    Replacement::draft_throws(0x140ef1f50, "unknown_140ef1f50", fn_140ef1f50 as *const (), Throws::Caught), // sub_140ef1f50
    Replacement::draft_throws(0x140ef2020, "unknown_140ef2020", fn_140ef2020 as *const (), Throws::Discard), // sub_140ef2020
    Replacement::draft_throws(0x140ef2240, "unknown_140ef2240", fn_140ef2240 as *const (), Throws::Caught), // sub_140ef2240
    Replacement::draft_throws(0x140ef28e0, "unknown_140ef28e0", fn_140ef28e0 as *const (), Throws::Caught), // sub_140ef28e0
    Replacement::draft_throws(0x140ef2f30, "unknown_140ef2f30", fn_140ef2f30 as *const (), Throws::Caught), // sub_140ef2f30
    Replacement::draft_throws(0x140ef3580, "unknown_140ef3580", fn_140ef3580 as *const (), Throws::Caught), // sub_140ef3580
    Replacement::draft_throws(0x140ef3780, "unknown_140ef3780", fn_140ef3780 as *const (), Throws::Caught), // sub_140ef3780
    Replacement::draft_throws(0x140ef3a60, "unknown_140ef3a60", fn_140ef3a60 as *const (), Throws::Caught), // sub_140ef3a60
    Replacement::draft_throws(0x140ef3d10, "unknown_140ef3d10", fn_140ef3d10 as *const (), Throws::Caught), // sub_140ef3d10
    Replacement::draft_throws(0x140ef3fb0, "unknown_140ef3fb0", fn_140ef3fb0 as *const (), Throws::Caught), // sub_140ef3fb0
    Replacement::draft(0x140ef4080, "unknown_140ef4080", tpl_4433::<0x19, 0x14083a7e0, 0x1c, 0x140ef4140, 0x10, 0x18, 0x8, 0x140ef4080> as *const ()), // sub_140ef4080
    Replacement::draft(0x140ef4140, "unknown_140ef4140", fn_140ef4140 as *const ()), // sub_140ef4140
    Replacement::draft(0x140ef41c0, "unknown_140ef41c0", fn_140ef41c0 as *const ()), // sub_140ef41c0
    Replacement::draft(0x140ef4200, "unknown_140ef4200", fn_140ef4200 as *const ()), // sub_140ef4200
    Replacement::draft_throws(0x140ef4260, "unknown_140ef4260", fn_140ef4260 as *const (), Throws::Discard), // sub_140ef4260
    Replacement::draft(0x140ef43c0, "unknown_140ef43c0", fn_140ef43c0 as *const ()), // sub_140ef43c0
    Replacement::draft_throws(0x140ef48e0, "unknown_140ef48e0", fn_140ef48e0 as *const (), Throws::Caught), // sub_140ef48e0
    Replacement::draft_throws(0x140ef4db0, "unknown_140ef4db0", fn_140ef4db0 as *const (), Throws::Caught), // sub_140ef4db0
    Replacement::draft_throws(0x140ef5240, "unknown_140ef5240", fn_140ef5240 as *const (), Throws::Caught), // sub_140ef5240
    Replacement::draft_throws(0x140ef56d0, "unknown_140ef56d0", fn_140ef56d0 as *const (), Throws::Discard), // sub_140ef56d0
    Replacement::draft_throws(0x140ef5fc0, "unknown_140ef5fc0", fn_140ef5fc0 as *const (), Throws::Discard), // sub_140ef5fc0
    Replacement::draft_throws(0x140ef69b0, "unknown_140ef69b0", fn_140ef69b0 as *const (), Throws::Discard), // sub_140ef69b0
    Replacement::draft_throws(0x140ef7390, "unknown_140ef7390", fn_140ef7390 as *const (), Throws::Caught), // sub_140ef7390
    Replacement::draft_throws(0x140ef7d70, "unknown_140ef7d70", fn_140ef7d70 as *const (), Throws::Caught), // sub_140ef7d70
    Replacement::draft_throws(0x140ef8750, "unknown_140ef8750", fn_140ef8750 as *const (), Throws::Caught), // sub_140ef8750
    Replacement::draft_throws(0x140ef9150, "unknown_140ef9150", fn_140ef9150 as *const (), Throws::Caught), // sub_140ef9150
    Replacement::draft_throws(0x140ef9620, "unknown_140ef9620", fn_140ef9620 as *const (), Throws::Caught), // sub_140ef9620
    Replacement::draft_throws(0x140ef9ab0, "unknown_140ef9ab0", fn_140ef9ab0 as *const (), Throws::Caught), // sub_140ef9ab0
    Replacement::draft(0x140ef9f50, "unknown_140ef9f50", fn_140ef9f50 as *const ()), // sub_140ef9f50
    Replacement::draft(0x140efa110, "unknown_140efa110", fn_140efa110 as *const ()), // sub_140efa110
    Replacement::draft_throws(0x140efa360, "unknown_140efa360", fn_140efa360 as *const (), Throws::Caught), // sub_140efa360
    Replacement::draft_throws(0x140efa4a0, "unknown_140efa4a0", fn_140efa4a0 as *const (), Throws::Caught), // sub_140efa4a0
    Replacement::draft_throws(0x140efa5e0, "unknown_140efa5e0", fn_140efa5e0 as *const (), Throws::Caught), // sub_140efa5e0
    Replacement::draft_throws(0x140efa720, "unknown_140efa720", fn_140efa720 as *const (), Throws::Caught), // sub_140efa720
    Replacement::draft_throws(0x140efa910, "unknown_140efa910", fn_140efa910 as *const (), Throws::Discard), // sub_140efa910
    Replacement::draft_throws(0x140efaad0, "unknown_140efaad0", fn_140efaad0 as *const (), Throws::Caught), // sub_140efaad0
    Replacement::draft_throws(0x140efac10, "unknown_140efac10", fn_140efac10 as *const (), Throws::Caught), // sub_140efac10
    Replacement::draft_throws(0x140efad50, "unknown_140efad50", fn_140efad50 as *const (), Throws::Caught), // sub_140efad50
    Replacement::draft_throws(0x140efae90, "unknown_140efae90", fn_140efae90 as *const (), Throws::Caught), // sub_140efae90
    Replacement::draft_throws(0x140efb080, "unknown_140efb080", fn_140efb080 as *const (), Throws::Discard), // sub_140efb080
    Replacement::draft_throws(0x140efb100, "unknown_140efb100", fn_140efb100 as *const (), Throws::Caught), // sub_140efb100
    Replacement::draft_throws(0x140efb570, "unknown_140efb570", fn_140efb570 as *const (), Throws::Caught), // sub_140efb570
    Replacement::draft_throws(0x140efb9a0, "unknown_140efb9a0", fn_140efb9a0 as *const (), Throws::Caught), // sub_140efb9a0
    Replacement::draft_throws(0x140efbcf0, "unknown_140efbcf0", fn_140efbcf0 as *const (), Throws::Caught), // sub_140efbcf0
    Replacement::draft_throws(0x140efc160, "unknown_140efc160", fn_140efc160 as *const (), Throws::Caught), // sub_140efc160
    Replacement::draft_throws(0x140efc590, "unknown_140efc590", fn_140efc590 as *const (), Throws::Caught), // sub_140efc590
    Replacement::draft(0x140efc8e0, "unknown_140efc8e0", tpl_1663::<0x8, 0x14083fed0, 0x1401e66f0> as *const ()), // sub_140efc8e0
    Replacement::draft(0x140efc920, "unknown_140efc920", fn_140efc920 as *const ()), // sub_140efc920
    Replacement::draft(0x140efca00, "unknown_140efca00", fn_140efca00 as *const ()), // sub_140efca00
    Replacement::draft(0x140efca70, "unknown_140efca70", fn_140efca70 as *const ()), // sub_140efca70
    Replacement::draft(0x140efcb00, "unknown_140efcb00", fn_140efcb00 as *const ()), // sub_140efcb00
    Replacement::draft_throws(0x140efcc90, "unknown_140efcc90", fn_140efcc90 as *const (), Throws::Discard), // sub_140efcc90
    Replacement::draft(0x140efce20, "unknown_140efce20", fn_140efce20 as *const ()), // sub_140efce20
    Replacement::draft_throws(0x140efcf00, "unknown_140efcf00", fn_140efcf00 as *const (), Throws::Discard), // sub_140efcf00
    Replacement::draft(0x140efd080, "unknown_140efd080", fn_140efd080 as *const ()), // sub_140efd080
    Replacement::draft_throws(0x140efd130, "unknown_140efd130", fn_140efd130 as *const (), Throws::Caught), // sub_140efd130
    Replacement::draft_throws(0x140efd6c0, "unknown_140efd6c0", fn_140efd6c0 as *const (), Throws::Caught), // sub_140efd6c0
    Replacement::draft_throws(0x140efdbc0, "unknown_140efdbc0", fn_140efdbc0 as *const (), Throws::Caught), // sub_140efdbc0
    Replacement::draft(0x140efe090, "unknown_140efe090", fn_140efe090 as *const ()), // sub_140efe090
    Replacement::draft(0x140efe200, "unknown_140efe200", fn_140efe200 as *const ()), // sub_140efe200
    Replacement::draft_throws(0x140efe320, "unknown_140efe320", fn_140efe320 as *const (), Throws::Discard), // sub_140efe320
    Replacement::draft_throws(0x140efe440, "unknown_140efe440", fn_140efe440 as *const (), Throws::Discard), // sub_140efe440
    Replacement::draft(0x140efe560, "unknown_140efe560", fn_140efe560 as *const ()), // sub_140efe560
    Replacement::draft(0x140efe5f0, "unknown_140efe5f0", fn_140efe5f0 as *const ()), // sub_140efe5f0
    Replacement::draft_throws(0x140efe720, "unknown_140efe720", fn_140efe720 as *const (), Throws::Discard), // sub_140efe720
    Replacement::draft(0x140efe9c0, "unknown_140efe9c0", fn_140efe9c0 as *const ()), // sub_140efe9c0
    Replacement::draft(0x140efea40, "unknown_140efea40", fn_140efea40 as *const ()), // sub_140efea40
    Replacement::draft_throws(0x140efeac0, "unknown_140efeac0", fn_140efeac0 as *const (), Throws::Discard), // sub_140efeac0
    Replacement::draft(0x140efecd0, "unknown_140efecd0", fn_140efecd0 as *const ()), // sub_140efecd0
    Replacement::draft(0x140efed70, "unknown_140efed70", fn_140efed70 as *const ()), // sub_140efed70
    Replacement::draft(0x140efeea0, "unknown_140efeea0", fn_140efeea0 as *const ()), // sub_140efeea0
    Replacement::draft(0x140efef60, "unknown_140efef60", fn_140efef60 as *const ()), // sub_140efef60
    Replacement::draft_throws(0x140eff3c0, "unknown_140eff3c0", fn_140eff3c0 as *const (), Throws::Discard), // sub_140eff3c0
    Replacement::draft(0x140eff7c0, "unknown_140eff7c0", fn_140eff7c0 as *const ()), // sub_140eff7c0
    Replacement::draft_throws(0x140eff9e0, "unknown_140eff9e0", fn_140eff9e0 as *const (), Throws::Discard), // sub_140eff9e0
    Replacement::draft(0x140effed0, "unknown_140effed0", fn_140effed0 as *const ()), // sub_140effed0
    Replacement::draft_throws(0x140f00040, "unknown_140f00040", fn_140f00040 as *const (), Throws::Caught), // sub_140f00040
    Replacement::draft_throws(0x140f004f0, "unknown_140f004f0", fn_140f004f0 as *const (), Throws::Caught), // sub_140f004f0
    Replacement::draft_throws(0x140f009a0, "unknown_140f009a0", fn_140f009a0 as *const (), Throws::Caught), // sub_140f009a0
    Replacement::draft(0x140f00e50, "unknown_140f00e50", fn_140f00e50 as *const ()), // sub_140f00e50
    Replacement::draft(0x140f01070, "unknown_140f01070", fn_140f01070 as *const ()), // sub_140f01070
    Replacement::draft(0x140f012c0, "unknown_140f012c0", fn_140f012c0 as *const ()), // sub_140f012c0
    Replacement::draft_throws(0x140f01520, "unknown_140f01520", fn_140f01520 as *const (), Throws::Discard), // sub_140f01520
    Replacement::draft_throws(0x140f01820, "unknown_140f01820", fn_140f01820 as *const (), Throws::Discard), // sub_140f01820
    Replacement::draft(0x140f01ca0, "unknown_140f01ca0", fn_140f01ca0 as *const ()), // sub_140f01ca0
    Replacement::draft(0x140f01ea0, "unknown_140f01ea0", fn_140f01ea0 as *const ()), // sub_140f01ea0
    Replacement::draft_throws(0x140f01fd0, "unknown_140f01fd0", fn_140f01fd0 as *const (), Throws::Discard), // sub_140f01fd0
    Replacement::draft(0x140f020d0, "unknown_140f020d0", tpl_18711::<0x14387fe80, 0x14243b0b0, 0x338, 0x48, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f020d0
    Replacement::draft_throws(0x140f02170, "unknown_140f02170", fn_140f02170 as *const (), Throws::Discard), // sub_140f02170
    Replacement::draft_throws(0x140f02290, "unknown_140f02290", fn_140f02290 as *const (), Throws::Caught), // sub_140f02290
    Replacement::draft_throws(0x140f023f0, "unknown_140f023f0", fn_140f023f0 as *const (), Throws::Caught), // sub_140f023f0
    Replacement::draft_throws(0x140f02550, "unknown_140f02550", fn_140f02550 as *const (), Throws::Caught), // sub_140f02550
    Replacement::draft(0x140f026b0, "unknown_140f026b0", tpl_18714::<0x14387fe80, 0x14243b0b0, 0x338, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f026b0
    Replacement::draft(0x140f02750, "unknown_140f02750", fn_140f02750 as *const ()), // sub_140f02750
    Replacement::draft(0x140f027f0, "unknown_140f027f0", tpl_18711::<0x14387fe80, 0x14243b0b0, 0x338, 0xe8, 0x4, 0x8, 0xc, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f027f0
    Replacement::draft_throws(0x140f028a0, "unknown_140f028a0", fn_140f028a0 as *const (), Throws::Discard), // sub_140f028a0
    Replacement::draft_throws(0x140f02960, "unknown_140f02960", fn_140f02960 as *const (), Throws::Discard), // sub_140f02960
    Replacement::draft(0x140f02ab0, "unknown_140f02ab0", fn_140f02ab0 as *const ()), // sub_140f02ab0
    Replacement::draft_throws(0x140f02bc0, "unknown_140f02bc0", fn_140f02bc0 as *const (), Throws::Caught), // sub_140f02bc0
    Replacement::draft_throws(0x140f02e80, "unknown_140f02e80", fn_140f02e80 as *const (), Throws::Caught), // sub_140f02e80
    Replacement::draft_throws(0x140f03110, "unknown_140f03110", fn_140f03110 as *const (), Throws::Caught), // sub_140f03110
    Replacement::draft(0x140f033b0, "unknown_140f033b0", fn_140f033b0 as *const ()), // sub_140f033b0
    Replacement::draft(0x140f03410, "unknown_140f03410", tpl_2030::<0x1404c1730, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f03410
    Replacement::draft(0x140f034c0, "unknown_140f034c0", fn_140f034c0 as *const ()), // sub_140f034c0
    Replacement::draft(0x140f03570, "unknown_140f03570", fn_140f03570 as *const ()), // sub_140f03570
    Replacement::draft(0x140f03640, "unknown_140f03640", fn_140f03640 as *const ()), // sub_140f03640
    Replacement::draft(0x140f036d0, "unknown_140f036d0", fn_140f036d0 as *const ()), // sub_140f036d0
    Replacement::draft_throws(0x140f038d0, "unknown_140f038d0", fn_140f038d0 as *const (), Throws::Discard), // sub_140f038d0
    Replacement::draft(0x140f03aa0, "unknown_140f03aa0", fn_140f03aa0 as *const ()), // sub_140f03aa0
    Replacement::draft_throws(0x140f03bb0, "unknown_140f03bb0", fn_140f03bb0 as *const (), Throws::Caught), // sub_140f03bb0
    Replacement::draft(0x140f03e00, "unknown_140f03e00", fn_140f03e00 as *const ()), // sub_140f03e00
    Replacement::draft_throws(0x140f03e80, "unknown_140f03e80", fn_140f03e80 as *const (), Throws::Caught), // sub_140f03e80
    Replacement::draft_throws(0x140f04360, "unknown_140f04360", fn_140f04360 as *const (), Throws::Caught), // sub_140f04360
    Replacement::draft_throws(0x140f047c0, "unknown_140f047c0", fn_140f047c0 as *const (), Throws::Caught), // sub_140f047c0
    Replacement::draft(0x140f04a20, "unknown_140f04a20", fn_140f04a20 as *const ()), // sub_140f04a20
    Replacement::draft(0x140f04bc0, "unknown_140f04bc0", fn_140f04bc0 as *const ()), // sub_140f04bc0
    Replacement::draft(0x140f04cc0, "unknown_140f04cc0", fn_140f04cc0 as *const ()), // sub_140f04cc0
    Replacement::draft_throws(0x140f04dc0, "unknown_140f04dc0", fn_140f04dc0 as *const (), Throws::Discard), // sub_140f04dc0
    Replacement::draft_throws(0x140f04ec0, "unknown_140f04ec0", fn_140f04ec0 as *const (), Throws::Discard), // sub_140f04ec0
    Replacement::draft(0x140f05030, "unknown_140f05030", fn_140f05030 as *const ()), // sub_140f05030
    Replacement::draft(0x140f050d0, "unknown_140f050d0", fn_140f050d0 as *const ()), // sub_140f050d0
    Replacement::draft(0x140f05180, "unknown_140f05180", fn_140f05180 as *const ()), // sub_140f05180
    Replacement::draft_throws(0x140f05290, "unknown_140f05290", fn_140f05290 as *const (), Throws::Discard), // sub_140f05290
    Replacement::draft(0x140f05340, "unknown_140f05340", fn_140f05340 as *const ()), // sub_140f05340
    Replacement::draft_throws(0x140f05430, "unknown_140f05430", fn_140f05430 as *const (), Throws::Discard), // sub_140f05430
    Replacement::draft(0x140f054f0, "unknown_140f054f0", fn_140f054f0 as *const ()), // sub_140f054f0
    Replacement::draft_throws(0x140f055c0, "unknown_140f055c0", fn_140f055c0 as *const (), Throws::Caught), // sub_140f055c0
    Replacement::draft_throws(0x140f056a0, "unknown_140f056a0", fn_140f056a0 as *const (), Throws::Caught), // sub_140f056a0
    Replacement::draft_throws(0x140f05780, "unknown_140f05780", fn_140f05780 as *const (), Throws::Caught), // sub_140f05780
    Replacement::draft(0x140f05860, "unknown_140f05860", fn_140f05860 as *const ()), // sub_140f05860
    Replacement::draft(0x140f05990, "unknown_140f05990", fn_140f05990 as *const ()), // sub_140f05990
    Replacement::draft(0x140f05aa0, "unknown_140f05aa0", fn_140f05aa0 as *const ()), // sub_140f05aa0
    Replacement::draft_throws(0x140f05b90, "unknown_140f05b90", fn_140f05b90 as *const (), Throws::Discard), // sub_140f05b90
    Replacement::draft_throws(0x140f05c80, "unknown_140f05c80", fn_140f05c80 as *const (), Throws::Discard), // sub_140f05c80
    Replacement::draft(0x140f05d50, "unknown_140f05d50", fn_140f05d50 as *const ()), // sub_140f05d50
    Replacement::draft(0x140f05df0, "unknown_140f05df0", fn_140f05df0 as *const ()), // sub_140f05df0
    Replacement::draft(0x140f05ec0, "unknown_140f05ec0", fn_140f05ec0 as *const ()), // sub_140f05ec0
    Replacement::draft_throws(0x140f05fb0, "unknown_140f05fb0", fn_140f05fb0 as *const (), Throws::Discard), // sub_140f05fb0
    Replacement::draft(0x140f061a0, "unknown_140f061a0", fn_140f061a0 as *const ()), // sub_140f061a0
    Replacement::draft(0x140f06320, "unknown_140f06320", fn_140f06320 as *const ()), // sub_140f06320
    Replacement::draft_throws(0x140f065b0, "unknown_140f065b0", fn_140f065b0 as *const (), Throws::Discard), // sub_140f065b0
    Replacement::draft(0x140f06760, "unknown_140f06760", fn_140f06760 as *const ()), // sub_140f06760
    Replacement::draft_throws(0x140f06900, "unknown_140f06900", fn_140f06900 as *const (), Throws::Discard), // sub_140f06900
    Replacement::draft_throws(0x140f06ab0, "unknown_140f06ab0", tpl_997::<0x140f06ac0, 0x140f06ab9> as *const (), Throws::Discard), // sub_140f06ab0
    Replacement::draft_throws(0x140f06ac0, "unknown_140f06ac0", fn_140f06ac0 as *const (), Throws::Discard), // sub_140f06ac0
    Replacement::draft(0x140f06b10, "unknown_140f06b10", fn_140f06b10 as *const ()), // sub_140f06b10
    Replacement::draft(0x140f06c60, "unknown_140f06c60", fn_140f06c60 as *const ()), // sub_140f06c60
    Replacement::draft(0x140f06df0, "unknown_140f06df0", fn_140f06df0 as *const ()), // sub_140f06df0
    Replacement::draft_throws(0x140f06f90, "unknown_140f06f90", fn_140f06f90 as *const (), Throws::Discard), // sub_140f06f90
    Replacement::draft_throws(0x140f07120, "unknown_140f07120", tpl_997::<0x140f07130, 0x140f07129> as *const (), Throws::Discard), // sub_140f07120
    Replacement::draft_throws(0x140f07130, "unknown_140f07130", fn_140f07130 as *const (), Throws::Discard), // sub_140f07130
    Replacement::draft(0x140f07180, "unknown_140f07180", tpl_18758::<0x168, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140f0a980> as *const ()), // sub_140f07180
    Replacement::draft(0x140f07210, "unknown_140f07210", fn_140f07210 as *const ()), // sub_140f07210
    Replacement::draft(0x140f072c0, "unknown_140f072c0", fn_140f072c0 as *const ()), // sub_140f072c0
    Replacement::draft_throws(0x140f074f0, "unknown_140f074f0", fn_140f074f0 as *const (), Throws::Discard), // sub_140f074f0
    Replacement::draft(0x140f07780, "unknown_140f07780", fn_140f07780 as *const ()), // sub_140f07780
    Replacement::draft_throws(0x140f078a0, "unknown_140f078a0", fn_140f078a0 as *const (), Throws::Discard), // sub_140f078a0
    Replacement::draft(0x140f07be0, "unknown_140f07be0", fn_140f07be0 as *const ()), // sub_140f07be0
    Replacement::draft_throws(0x140f07cf0, "unknown_140f07cf0", fn_140f07cf0 as *const (), Throws::Caught), // sub_140f07cf0
    Replacement::draft_throws(0x140f08270, "unknown_140f08270", fn_140f08270 as *const (), Throws::Caught), // sub_140f08270
    Replacement::draft_throws(0x140f08790, "unknown_140f08790", fn_140f08790 as *const (), Throws::Caught), // sub_140f08790
    Replacement::draft(0x140f08db0, "unknown_140f08db0", fn_140f08db0 as *const ()), // sub_140f08db0
    Replacement::draft(0x140f08f80, "unknown_140f08f80", fn_140f08f80 as *const ()), // sub_140f08f80
    Replacement::draft(0x140f090a0, "unknown_140f090a0", fn_140f090a0 as *const ()), // sub_140f090a0
    Replacement::draft_throws(0x140f091d0, "unknown_140f091d0", fn_140f091d0 as *const (), Throws::Discard), // sub_140f091d0
    Replacement::draft_throws(0x140f09370, "unknown_140f09370", fn_140f09370 as *const (), Throws::Discard), // sub_140f09370
    Replacement::draft(0x140f09630, "unknown_140f09630", fn_140f09630 as *const ()), // sub_140f09630
    Replacement::draft_throws(0x140f09700, "unknown_140f09700", fn_140f09700 as *const (), Throws::Discard), // sub_140f09700
    Replacement::draft(0x140f097b0, "unknown_140f097b0", tpl_77161::<0x14387fed0, 0x14243b0b0, 0x338, 0x48, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f097b0
    Replacement::draft_throws(0x140f09830, "unknown_140f09830", fn_140f09830 as *const (), Throws::Discard), // sub_140f09830
    Replacement::draft_throws(0x140f098f0, "unknown_140f098f0", fn_140f098f0 as *const (), Throws::Caught), // sub_140f098f0
    Replacement::draft_throws(0x140f099f0, "unknown_140f099f0", fn_140f099f0 as *const (), Throws::Caught), // sub_140f099f0
    Replacement::draft_throws(0x140f09af0, "unknown_140f09af0", fn_140f09af0 as *const (), Throws::Caught), // sub_140f09af0
    Replacement::draft(0x140f09bf0, "unknown_140f09bf0", fn_140f09bf0 as *const ()), // sub_140f09bf0
    Replacement::draft(0x140f09c70, "unknown_140f09c70", fn_140f09c70 as *const ()), // sub_140f09c70
    Replacement::draft(0x140f09cf0, "unknown_140f09cf0", tpl_77161::<0x14387fed0, 0x14243b0b0, 0x338, 0xe8, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f09cf0
    Replacement::draft_throws(0x140f09d80, "unknown_140f09d80", fn_140f09d80 as *const (), Throws::Discard), // sub_140f09d80
    Replacement::draft_throws(0x140f09e10, "unknown_140f09e10", fn_140f09e10 as *const (), Throws::Discard), // sub_140f09e10
    Replacement::draft(0x140f09f00, "unknown_140f09f00", tpl_3532::<0x14387fef0, 0x14243b0b0, 0x340, 0x20, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f09f00
    Replacement::draft(0x140f09f90, "unknown_140f09f90", fn_140f09f90 as *const ()), // sub_140f09f90
    Replacement::draft_throws(0x140f0a070, "unknown_140f0a070", fn_140f0a070 as *const (), Throws::Discard), // sub_140f0a070
    Replacement::draft(0x140f0a140, "unknown_140f0a140", fn_140f0a140 as *const ()), // sub_140f0a140
    Replacement::draft_throws(0x140f0a1d0, "unknown_140f0a1d0", fn_140f0a1d0 as *const (), Throws::Discard), // sub_140f0a1d0
    Replacement::draft_throws(0x140f0a2c0, "unknown_140f0a2c0", fn_140f0a2c0 as *const (), Throws::Caught), // sub_140f0a2c0
    Replacement::draft_throws(0x140f0a3c0, "unknown_140f0a3c0", fn_140f0a3c0 as *const (), Throws::Caught), // sub_140f0a3c0
    Replacement::draft_throws(0x140f0a4c0, "unknown_140f0a4c0", fn_140f0a4c0 as *const (), Throws::Caught), // sub_140f0a4c0
    Replacement::draft(0x140f0a5c0, "unknown_140f0a5c0", fn_140f0a5c0 as *const ()), // sub_140f0a5c0
    Replacement::draft(0x140f0a640, "unknown_140f0a640", fn_140f0a640 as *const ()), // sub_140f0a640
    Replacement::draft(0x140f0a6e0, "unknown_140f0a6e0", fn_140f0a6e0 as *const ()), // sub_140f0a6e0
    Replacement::draft_throws(0x140f0a790, "unknown_140f0a790", fn_140f0a790 as *const (), Throws::Discard), // sub_140f0a790
    Replacement::draft_throws(0x140f0a840, "unknown_140f0a840", fn_140f0a840 as *const (), Throws::Discard), // sub_140f0a840
    Replacement::draft(0x140f0a930, "unknown_140f0a930", fn_140f0a930 as *const ()), // sub_140f0a930
    Replacement::draft(0x140f0a980, "unknown_140f0a980", fn_140f0a980 as *const ()), // sub_140f0a980
    Replacement::draft(0x140f0aa40, "unknown_140f0aa40", fn_140f0aa40 as *const ()), // sub_140f0aa40
    Replacement::draft_throws(0x140f0aaf0, "unknown_140f0aaf0", fn_140f0aaf0 as *const (), Throws::Caught), // sub_140f0aaf0
    Replacement::draft(0x140f0acc0, "unknown_140f0acc0", fn_140f0acc0 as *const ()), // sub_140f0acc0
    Replacement::draft(0x140f0ad50, "unknown_140f0ad50", fn_140f0ad50 as *const ()), // sub_140f0ad50
    Replacement::draft_throws(0x140f0adc0, "unknown_140f0adc0", fn_140f0adc0 as *const (), Throws::Caught), // sub_140f0adc0
    Replacement::draft_throws(0x140f0af70, "unknown_140f0af70", fn_140f0af70 as *const (), Throws::Caught), // sub_140f0af70
    Replacement::draft(0x140f0b1a0, "unknown_140f0b1a0", fn_140f0b1a0 as *const ()), // sub_140f0b1a0
    Replacement::draft(0x140f0b230, "unknown_140f0b230", fn_140f0b230 as *const ()), // sub_140f0b230
    Replacement::draft_throws(0x140f0b2e0, "unknown_140f0b2e0", fn_140f0b2e0 as *const (), Throws::Caught), // sub_140f0b2e0
    Replacement::draft_throws(0x140f0b310, "unknown_140f0b310", fn_140f0b310 as *const (), Throws::Caught), // sub_140f0b310
    Replacement::draft(0x140f0b340, "unknown_140f0b340", fn_140f0b340 as *const ()), // sub_140f0b340
    Replacement::draft(0x140f0b470, "unknown_140f0b470", fn_140f0b470 as *const ()), // sub_140f0b470
    Replacement::draft_throws(0x140f0b6d0, "unknown_140f0b6d0", fn_140f0b6d0 as *const (), Throws::Caught), // sub_140f0b6d0
    Replacement::draft_throws(0x140f0b9b0, "unknown_140f0b9b0", fn_140f0b9b0 as *const (), Throws::Caught), // sub_140f0b9b0
    Replacement::draft_throws(0x140f0bc60, "unknown_140f0bc60", fn_140f0bc60 as *const (), Throws::Caught), // sub_140f0bc60
    Replacement::draft_throws(0x140f0bf00, "unknown_140f0bf00", fn_140f0bf00 as *const (), Throws::Caught), // sub_140f0bf00
    Replacement::draft(0x140f0c000, "unknown_140f0c000", fn_140f0c000 as *const ()), // sub_140f0c000
    Replacement::draft_throws(0x140f0c2d0, "unknown_140f0c2d0", fn_140f0c2d0 as *const (), Throws::Caught), // sub_140f0c2d0
    Replacement::draft(0x140f0c350, "unknown_140f0c350", fn_140f0c350 as *const ()), // sub_140f0c350
    Replacement::draft_throws(0x140f0c420, "unknown_140f0c420", fn_140f0c420 as *const (), Throws::Caught), // sub_140f0c420
    Replacement::draft_throws(0x140f0c8f0, "unknown_140f0c8f0", fn_140f0c8f0 as *const (), Throws::Caught), // sub_140f0c8f0
    Replacement::draft_throws(0x140f0cd80, "unknown_140f0cd80", fn_140f0cd80 as *const (), Throws::Caught), // sub_140f0cd80
    Replacement::draft_throws(0x140f0d210, "unknown_140f0d210", fn_140f0d210 as *const (), Throws::Caught), // sub_140f0d210
    Replacement::draft_throws(0x140f0d350, "unknown_140f0d350", fn_140f0d350 as *const (), Throws::Caught), // sub_140f0d350
    Replacement::draft_throws(0x140f0d490, "unknown_140f0d490", fn_140f0d490 as *const (), Throws::Caught), // sub_140f0d490
    Replacement::draft_throws(0x140f0d5d0, "unknown_140f0d5d0", fn_140f0d5d0 as *const (), Throws::Caught), // sub_140f0d5d0
    Replacement::draft(0x140f0d7c0, "unknown_140f0d7c0", fn_140f0d7c0 as *const ()), // sub_140f0d7c0
    Replacement::draft_throws(0x140f0d840, "unknown_140f0d840", fn_140f0d840 as *const (), Throws::Caught), // sub_140f0d840
    Replacement::draft_throws(0x140f0dcb0, "unknown_140f0dcb0", fn_140f0dcb0 as *const (), Throws::Caught), // sub_140f0dcb0
    Replacement::draft_throws(0x140f0e0e0, "unknown_140f0e0e0", fn_140f0e0e0 as *const (), Throws::Caught), // sub_140f0e0e0
    Replacement::draft(0x140f0e430, "unknown_140f0e430", tpl_2030::<0x140f0e650, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f0e430
    Replacement::draft(0x140f0e4f0, "unknown_140f0e4f0", tpl_2030::<0x140f0e6c0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f0e4f0
    Replacement::draft(0x140f0e650, "unknown_140f0e650", tpl_16124::<0x8, 0x140f0e5a0> as *const ()), // sub_140f0e650
    Replacement::draft(0x140f0e6c0, "unknown_140f0e6c0", tpl_16124::<0x8, 0x1401f2bc0> as *const ()), // sub_140f0e6c0
    Replacement::draft(0x140f0e730, "unknown_140f0e730", fn_140f0e730 as *const ()), // sub_140f0e730
    Replacement::draft(0x140f0e7d0, "unknown_140f0e7d0", fn_140f0e7d0 as *const ()), // sub_140f0e7d0
    Replacement::draft_throws(0x140f0e940, "unknown_140f0e940", fn_140f0e940 as *const (), Throws::Discard), // sub_140f0e940
    Replacement::draft(0x140f0ea00, "unknown_140f0ea00", fn_140f0ea00 as *const ()), // sub_140f0ea00
    Replacement::draft_throws(0x140f0ea90, "unknown_140f0ea90", fn_140f0ea90 as *const (), Throws::Discard), // sub_140f0ea90
    Replacement::draft_throws(0x140f0eb60, "unknown_140f0eb60", fn_140f0eb60 as *const (), Throws::Caught), // sub_140f0eb60
    Replacement::draft_throws(0x140f0ec70, "unknown_140f0ec70", fn_140f0ec70 as *const (), Throws::Caught), // sub_140f0ec70
    Replacement::draft_throws(0x140f0ed80, "unknown_140f0ed80", fn_140f0ed80 as *const (), Throws::Caught), // sub_140f0ed80
    Replacement::draft(0x140f0ee90, "unknown_140f0ee90", fn_140f0ee90 as *const ()), // sub_140f0ee90
    Replacement::draft(0x140f0ef50, "unknown_140f0ef50", fn_140f0ef50 as *const ()), // sub_140f0ef50
    Replacement::draft(0x140f0f000, "unknown_140f0f000", fn_140f0f000 as *const ()), // sub_140f0f000
    Replacement::draft_throws(0x140f0f0c0, "unknown_140f0f0c0", fn_140f0f0c0 as *const (), Throws::Discard), // sub_140f0f0c0
    Replacement::draft_throws(0x140f0f1b0, "unknown_140f0f1b0", fn_140f0f1b0 as *const (), Throws::Discard), // sub_140f0f1b0
    Replacement::draft(0x140f0f2b0, "unknown_140f0f2b0", fn_140f0f2b0 as *const ()), // sub_140f0f2b0
    Replacement::draft(0x140f0f350, "unknown_140f0f350", fn_140f0f350 as *const ()), // sub_140f0f350
    Replacement::draft(0x140f0f400, "unknown_140f0f400", fn_140f0f400 as *const ()), // sub_140f0f400
    Replacement::draft(0x140f0f530, "unknown_140f0f530", fn_140f0f530 as *const ()), // sub_140f0f530
    Replacement::draft_throws(0x140f0f930, "unknown_140f0f930", fn_140f0f930 as *const (), Throws::Discard), // sub_140f0f930
    Replacement::draft(0x140f0fbc0, "unknown_140f0fbc0", fn_140f0fbc0 as *const ()), // sub_140f0fbc0
    Replacement::draft_throws(0x140f0fe10, "unknown_140f0fe10", fn_140f0fe10 as *const (), Throws::Discard), // sub_140f0fe10
    Replacement::draft(0x140f100c0, "unknown_140f100c0", fn_140f100c0 as *const ()), // sub_140f100c0
    Replacement::draft_throws(0x140f10240, "unknown_140f10240", fn_140f10240 as *const (), Throws::Caught), // sub_140f10240
    Replacement::draft_throws(0x140f103a0, "unknown_140f103a0", fn_140f103a0 as *const (), Throws::Caught), // sub_140f103a0
    Replacement::draft_throws(0x140f10500, "unknown_140f10500", fn_140f10500 as *const (), Throws::Caught), // sub_140f10500
    Replacement::draft(0x140f10660, "unknown_140f10660", fn_140f10660 as *const ()), // sub_140f10660
    Replacement::draft(0x140f108a0, "unknown_140f108a0", fn_140f108a0 as *const ()), // sub_140f108a0
    Replacement::draft(0x140f10ae0, "unknown_140f10ae0", fn_140f10ae0 as *const ()), // sub_140f10ae0
    Replacement::draft_throws(0x140f10d40, "unknown_140f10d40", fn_140f10d40 as *const (), Throws::Discard), // sub_140f10d40
    Replacement::draft_throws(0x140f10fb0, "unknown_140f10fb0", fn_140f10fb0 as *const (), Throws::Discard), // sub_140f10fb0
    Replacement::draft(0x140f11100, "unknown_140f11100", tpl_18530::<0x14387ff78, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x4, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f11100
    Replacement::draft_throws(0x140f111d0, "unknown_140f111d0", fn_140f111d0 as *const (), Throws::Discard), // sub_140f111d0
    Replacement::draft(0x140f11280, "unknown_140f11280", tpl_9506::<0x14387ff78, 0x14243b0b0, 0x338, 0x48, 0x4, 0x38, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f11280
    Replacement::draft_throws(0x140f11300, "unknown_140f11300", fn_140f11300 as *const (), Throws::Discard), // sub_140f11300
    Replacement::draft_throws(0x140f113c0, "unknown_140f113c0", fn_140f113c0 as *const (), Throws::Caught), // sub_140f113c0
    Replacement::draft_throws(0x140f114d0, "unknown_140f114d0", fn_140f114d0 as *const (), Throws::Caught), // sub_140f114d0
    Replacement::draft_throws(0x140f115e0, "unknown_140f115e0", fn_140f115e0 as *const (), Throws::Caught), // sub_140f115e0
    Replacement::draft(0x140f116f0, "unknown_140f116f0", fn_140f116f0 as *const ()), // sub_140f116f0
    Replacement::draft(0x140f11760, "unknown_140f11760", tpl_9437::<0x14387ff50, 0x14387ff78, 0x1401e9970, 0x338, 0xe8, 0x4, 0xd8, 0x1401e9860> as *const ()), // sub_140f11760
    Replacement::draft(0x140f117d0, "unknown_140f117d0", tpl_9506::<0x14387ff78, 0x14243b0b0, 0x338, 0xe8, 0x4, 0xd8, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f117d0
    Replacement::draft_throws(0x140f11850, "unknown_140f11850", fn_140f11850 as *const (), Throws::Discard), // sub_140f11850
    Replacement::draft_throws(0x140f118d0, "unknown_140f118d0", fn_140f118d0 as *const (), Throws::Discard), // sub_140f118d0
    Replacement::draft(0x140f119c0, "unknown_140f119c0", tpl_9462::<0x14387ff60, 0x14243b0b0, 0x338, 0x1426d4580, 0x1, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f119c0
    Replacement::draft_throws(0x140f11a60, "unknown_140f11a60", tpl_9463::<0x14387ff3c, 0x14387ff60, 0x1401e9fa0, 0x143427660, 0x328, 0x338, 0x140715850, 0x369, 0x8, 0xc, 0x24, 0x18, 0x1401e9e70> as *const (), Throws::Discard), // sub_140f11a60
    Replacement::draft(0x140f11b30, "unknown_140f11b30", tpl_9464::<0x14387ff60, 0x14243b0b0, 0x338, 0x140715850, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f11b30
    Replacement::draft_throws(0x140f11ba0, "unknown_140f11ba0", tpl_9465::<0x14387ff3c, 0x14387ff60, 0x1401e9b60, 0x143427660, 0x328, 0x338, 0x120, 0x8, 0xc, 0x14, 0x18, 0x20, 0x1401e9a20> as *const (), Throws::Discard), // sub_140f11ba0
    Replacement::draft_throws(0x140f11cc0, "unknown_140f11cc0", fn_140f11cc0 as *const (), Throws::Caught), // sub_140f11cc0
    Replacement::draft_throws(0x140f11d90, "unknown_140f11d90", fn_140f11d90 as *const (), Throws::Caught), // sub_140f11d90
    Replacement::draft_throws(0x140f11e60, "unknown_140f11e60", fn_140f11e60 as *const (), Throws::Caught), // sub_140f11e60
    Replacement::draft(0x140f12070, "unknown_140f12070", tpl_9468::<0x14387ff60, 0x14243b0b0, 0x143427660, 0x338, 0x8, 0xc, 0x24, 0x18, 0x2c, 0x14, 0x20, 0xfffffffffffffff8> as *const ()), // sub_140f12070
    Replacement::draft(0x140f12140, "unknown_140f12140", tpl_9464::<0x14387ff60, 0x14243b0b0, 0x338, 0x140715760, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140f12140
    Replacement::draft_throws(0x140f121b0, "unknown_140f121b0", tpl_9469::<0x14387ff3c, 0x14387ff60, 0x1401e9770, 0x338, 0x14, 0xc, 0x8, 0x120, 0x18, 0x20, 0x24, 0x2c, 0x1401e9590> as *const (), Throws::Discard), // sub_140f121b0
    Replacement::draft_throws(0x140f12250, "unknown_140f12250", tpl_9470::<0x14387ff60, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x20, 0x143428b00, 0x143427660, 0x1404a4e70, 0x24, 0x143307eb8, 0x1413c48e0> as *const (), Throws::Discard), // sub_140f12250
    Replacement::draft(0x140f12350, "unknown_140f12350", tpl_5163::<0x100, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14183acc2, 0x140f123e0> as *const ()), // sub_140f12350
    Replacement::draft(0x140f123e0, "unknown_140f123e0", fn_140f123e0 as *const ()), // sub_140f123e0
    Replacement::draft(0x140f12490, "unknown_140f12490", tpl_64337::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14183acc2, 0x140f123e0> as *const ()), // sub_140f12490
    Replacement::draft(0x140f12540, "unknown_140f12540", fn_140f12540 as *const ()), // sub_140f12540
    Replacement::draft(0x140f12720, "unknown_140f12720", fn_140f12720 as *const ()), // sub_140f12720
    Replacement::draft_throws(0x140f12c50, "unknown_140f12c50", fn_140f12c50 as *const (), Throws::Caught), // sub_140f12c50
    Replacement::draft(0x140f13300, "unknown_140f13300", fn_140f13300 as *const ()), // sub_140f13300
    Replacement::draft_throws(0x140f13700, "unknown_140f13700", fn_140f13700 as *const (), Throws::Caught), // sub_140f13700
    Replacement::draft(0x140f13de0, "unknown_140f13de0", fn_140f13de0 as *const ()), // sub_140f13de0
    Replacement::draft_throws(0x140f13f90, "unknown_140f13f90", fn_140f13f90 as *const (), Throws::Caught), // sub_140f13f90
    Replacement::draft_throws(0x140f148f0, "unknown_140f148f0", fn_140f148f0 as *const (), Throws::Caught), // sub_140f148f0
    Replacement::draft_throws(0x140f15150, "unknown_140f15150", fn_140f15150 as *const (), Throws::Caught), // sub_140f15150
    Replacement::draft(0x140f15970, "unknown_140f15970", fn_140f15970 as *const ()), // sub_140f15970
    Replacement::draft(0x140f15de0, "unknown_140f15de0", fn_140f15de0 as *const ()), // sub_140f15de0
    Replacement::draft(0x140f16350, "unknown_140f16350", fn_140f16350 as *const ()), // sub_140f16350
    Replacement::draft_throws(0x140f168d0, "unknown_140f168d0", fn_140f168d0 as *const (), Throws::Caught), // sub_140f168d0
    Replacement::draft_throws(0x140f16cf0, "unknown_140f16cf0", fn_140f16cf0 as *const (), Throws::Caught), // sub_140f16cf0
    Replacement::draft_throws(0x140f17380, "unknown_140f17380", fn_140f17380 as *const (), Throws::Discard), // sub_140f17380
    Replacement::draft(0x140f176d0, "unknown_140f176d0", fn_140f176d0 as *const ()), // sub_140f176d0
    Replacement::draft(0x140f17770, "unknown_140f17770", fn_140f17770 as *const ()), // sub_140f17770
    Replacement::draft(0x140f178b0, "unknown_140f178b0", fn_140f178b0 as *const ()), // sub_140f178b0
    Replacement::draft(0x140f17980, "unknown_140f17980", fn_140f17980 as *const ()), // sub_140f17980
    Replacement::draft_throws(0x140f17f00, "std_140f17f00", fn_140f17f00 as *const (), Throws::Discard), // sub_140f17f00
    Replacement::draft(0x140f184c0, "unknown_140f184c0", fn_140f184c0 as *const ()), // sub_140f184c0
    Replacement::draft_throws(0x140f18820, "std_140f18820", fn_140f18820 as *const (), Throws::Discard), // sub_140f18820
    Replacement::draft(0x140f18ee0, "unknown_140f18ee0", fn_140f18ee0 as *const ()), // sub_140f18ee0
    Replacement::draft_throws(0x140f190e0, "unknown_140f190e0", fn_140f190e0 as *const (), Throws::Caught), // sub_140f190e0
    Replacement::draft_throws(0x140f19c40, "unknown_140f19c40", fn_140f19c40 as *const (), Throws::Caught), // sub_140f19c40
    Replacement::draft_throws(0x140f1a770, "unknown_140f1a770", fn_140f1a770 as *const (), Throws::Caught), // sub_140f1a770
    Replacement::draft(0x140f1b290, "std_140f1b290", fn_140f1b290 as *const ()), // sub_140f1b290
    Replacement::draft(0x140f1b450, "unknown_140f1b450", fn_140f1b450 as *const ()), // sub_140f1b450
    Replacement::draft(0x140f1b7e0, "unknown_140f1b7e0", fn_140f1b7e0 as *const ()), // sub_140f1b7e0
    Replacement::draft_throws(0x140f1bb80, "unknown_140f1bb80", fn_140f1bb80 as *const (), Throws::Discard), // sub_140f1bb80
    Replacement::draft_throws(0x140f1bfb0, "unknown_140f1bfb0", fn_140f1bfb0 as *const (), Throws::Caught), // sub_140f1bfb0
    Replacement::draft(0x140f1ca20, "unknown_140f1ca20", tpl_9044::<0x78, 0x18, 0x1c, 0x10, 0x14020bcb0, 0x8, 0xc> as *const ()), // sub_140f1ca20
    Replacement::draft_throws(0x140f1cac0, "unknown_140f1cac0", fn_140f1cac0 as *const (), Throws::Discard), // sub_140f1cac0
    Replacement::draft(0x140f1cff0, "unknown_140f1cff0", fn_140f1cff0 as *const ()), // sub_140f1cff0
    Replacement::draft(0x140f1d050, "unknown_140f1d050", fn_140f1d050 as *const ()), // sub_140f1d050
    Replacement::draft(0x140f1d170, "unknown_140f1d170", fn_140f1d170 as *const ()), // sub_140f1d170
    Replacement::draft(0x140f1d250, "unknown_140f1d250", fn_140f1d250 as *const ()), // sub_140f1d250
    Replacement::draft(0x140f1d330, "unknown_140f1d330", fn_140f1d330 as *const ()), // sub_140f1d330
    Replacement::draft(0x140f1d430, "unknown_140f1d430", fn_140f1d430 as *const ()), // sub_140f1d430
    Replacement::draft(0x140f1d4e0, "unknown_140f1d4e0", fn_140f1d4e0 as *const ()), // sub_140f1d4e0
    Replacement::draft_throws(0x140f1d6f0, "unknown_140f1d6f0", fn_140f1d6f0 as *const (), Throws::Discard), // sub_140f1d6f0
    Replacement::draft(0x140f1d890, "unknown_140f1d890", fn_140f1d890 as *const ()), // sub_140f1d890
    Replacement::draft_throws(0x140f1d9e0, "unknown_140f1d9e0", fn_140f1d9e0 as *const (), Throws::Discard), // sub_140f1d9e0
    Replacement::draft(0x140f1db70, "unknown_140f1db70", fn_140f1db70 as *const ()), // sub_140f1db70
    Replacement::draft_throws(0x140f1dc80, "unknown_140f1dc80", fn_140f1dc80 as *const (), Throws::Caught), // sub_140f1dc80
    Replacement::draft_throws(0x140f1dee0, "unknown_140f1dee0", fn_140f1dee0 as *const (), Throws::Caught), // sub_140f1dee0
    Replacement::draft(0x140f1e130, "unknown_140f1e130", fn_140f1e130 as *const ()), // sub_140f1e130
    Replacement::draft(0x140f1e280, "unknown_140f1e280", fn_140f1e280 as *const ()), // sub_140f1e280
    Replacement::draft_throws(0x140f1e3d0, "unknown_140f1e3d0", fn_140f1e3d0 as *const (), Throws::Discard), // sub_140f1e3d0
    Replacement::draft_throws(0x140f1e500, "unknown_140f1e500", fn_140f1e500 as *const (), Throws::Discard), // sub_140f1e500
    Replacement::draft(0x140f1e680, "unknown_140f1e680", tpl_18877::<0x141dc0550, 0x141dc05c0, 0xa0, 0x1426cdcb0, 0x98, 0x140f226b0> as *const ()), // sub_140f1e680
    Replacement::draft(0x140f1e710, "unknown_140f1e710", tpl_18877::<0x141dc0550, 0x141dc05c0, 0xa0, 0x14307ead0, 0x98, 0x140f226d0> as *const ()), // sub_140f1e710
    Replacement::draft(0x140f1e7a0, "unknown_140f1e7a0", tpl_16361::<0x140f0e650, 0x8, 0xc, 0x140f1f3f0, 0x140f1e870> as *const ()), // sub_140f1e7a0
    Replacement::draft(0x140f1e870, "unknown_140f1e870", fn_140f1e870 as *const ()), // sub_140f1e870
    Replacement::draft(0x140f1e940, "unknown_140f1e940", tpl_16361::<0x140f0e6c0, 0x8, 0xc, 0x140f1f810, 0x140244c00> as *const ()), // sub_140f1e940
    Replacement::draft(0x140f1ea10, "unknown_140f1ea10", fn_140f1ea10 as *const ()), // sub_140f1ea10
    Replacement::draft(0x140f1ead0, "unknown_140f1ead0", fn_140f1ead0 as *const ()), // sub_140f1ead0
    Replacement::draft_throws(0x140f1eb70, "unknown_140f1eb70", tpl_2506::<0x328, 0x338, 0x120, 0x140f1f280, 0x140f1fd00, 0x369> as *const (), Throws::Discard), // sub_140f1eb70
    Replacement::draft(0x140f1ed50, "unknown_140f1ed50", fn_140f1ed50 as *const ()), // sub_140f1ed50
    Replacement::draft_throws(0x140f1ee00, "unknown_140f1ee00", fn_140f1ee00 as *const (), Throws::Caught), // sub_140f1ee00
    Replacement::draft_throws(0x140f1efb0, "unknown_140f1efb0", fn_140f1efb0 as *const (), Throws::Caught), // sub_140f1efb0
    Replacement::draft(0x140f1f1d0, "unknown_140f1f1d0", tpl_16370::<0x8, 0xc, 0x140f1f3f0, 0x140f0e5a0> as *const ()), // sub_140f1f1d0
    Replacement::draft(0x140f1f280, "unknown_140f1f280", fn_140f1f280 as *const ()), // sub_140f1f280
    Replacement::draft(0x140f1f340, "unknown_140f1f340", tpl_16370::<0x8, 0xc, 0x140f1f810, 0x1401f2bc0> as *const ()), // sub_140f1f340
    Replacement::draft(0x140f1f3f0, "unknown_140f1f3f0", tpl_16373::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140f26a30, 0x140f0e5a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f1f3f0
    Replacement::draft(0x140f1f5f0, "unknown_140f1f5f0", fn_140f1f5f0 as *const ()), // sub_140f1f5f0
    Replacement::draft(0x140f1f810, "unknown_140f1f810", tpl_16373::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1401ff410, 0x1401f2bc0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f1f810
    Replacement::draft_throws(0x140f1fa10, "unknown_140f1fa10", fn_140f1fa10 as *const (), Throws::Caught), // sub_140f1fa10
    Replacement::draft_throws(0x140f1fd00, "unknown_140f1fd00", fn_140f1fd00 as *const (), Throws::Caught), // sub_140f1fd00
    Replacement::draft(0x140f1fff0, "unknown_140f1fff0", fn_140f1fff0 as *const ()), // sub_140f1fff0
    Replacement::draft_throws(0x140f200c0, "unknown_140f200c0", fn_140f200c0 as *const (), Throws::Caught), // sub_140f200c0
    Replacement::draft_throws(0x140f20390, "unknown_140f20390", fn_140f20390 as *const (), Throws::Caught), // sub_140f20390
    Replacement::draft_throws(0x140f20830, "unknown_140f20830", fn_140f20830 as *const (), Throws::Caught), // sub_140f20830
    Replacement::draft_throws(0x140f20b00, "unknown_140f20b00", fn_140f20b00 as *const (), Throws::Caught), // sub_140f20b00
    Replacement::draft_throws(0x140f20d90, "unknown_140f20d90", fn_140f20d90 as *const (), Throws::Caught), // sub_140f20d90
    Replacement::draft_throws(0x140f21200, "unknown_140f21200", fn_140f21200 as *const (), Throws::Caught), // sub_140f21200
    Replacement::draft_throws(0x140f21490, "unknown_140f21490", fn_140f21490 as *const (), Throws::Caught), // sub_140f21490
    Replacement::draft_throws(0x140f21720, "unknown_140f21720", fn_140f21720 as *const (), Throws::Caught), // sub_140f21720
    Replacement::draft_throws(0x140f21b80, "unknown_140f21b80", fn_140f21b80 as *const (), Throws::Caught), // sub_140f21b80
    Replacement::draft_throws(0x140f21e10, "unknown_140f21e10", fn_140f21e10 as *const (), Throws::Caught), // sub_140f21e10
    Replacement::draft_throws(0x140f21ee0, "unknown_140f21ee0", fn_140f21ee0 as *const (), Throws::Caught), // sub_140f21ee0
    Replacement::draft_throws(0x140f21fb0, "unknown_140f21fb0", fn_140f21fb0 as *const (), Throws::Caught), // sub_140f21fb0
    Replacement::draft_throws(0x140f22080, "unknown_140f22080", fn_140f22080 as *const (), Throws::Caught), // sub_140f22080
    Replacement::draft_throws(0x140f22330, "unknown_140f22330", fn_140f22330 as *const (), Throws::Caught), // sub_140f22330
    Replacement::draft_throws(0x140f225e0, "unknown_140f225e0", fn_140f225e0 as *const (), Throws::Caught), // sub_140f225e0
    Replacement::draft(0x140f226f0, "unknown_140f226f0", tpl_16383::<0x8, 0x1401f2bc0> as *const ()), // sub_140f226f0
    Replacement::draft(0x140f22730, "unknown_140f22730", tpl_16383::<0x8, 0x140f0e5a0> as *const ()), // sub_140f22730
    Replacement::draft_throws(0x140f22770, "unknown_140f22770", fn_140f22770 as *const (), Throws::Caught), // sub_140f22770
    Replacement::draft_throws(0x140f22c40, "unknown_140f22c40", fn_140f22c40 as *const (), Throws::Caught), // sub_140f22c40
    Replacement::draft_throws(0x140f23110, "unknown_140f23110", fn_140f23110 as *const (), Throws::Caught), // sub_140f23110
    Replacement::draft_throws(0x140f235e0, "unknown_140f235e0", fn_140f235e0 as *const (), Throws::Caught), // sub_140f235e0
    Replacement::draft_throws(0x140f23a70, "unknown_140f23a70", fn_140f23a70 as *const (), Throws::Caught), // sub_140f23a70
    Replacement::draft_throws(0x140f23f00, "unknown_140f23f00", fn_140f23f00 as *const (), Throws::Caught), // sub_140f23f00
    Replacement::draft_throws(0x140f24390, "unknown_140f24390", fn_140f24390 as *const (), Throws::Caught), // sub_140f24390
    Replacement::draft_throws(0x140f24820, "unknown_140f24820", fn_140f24820 as *const (), Throws::Caught), // sub_140f24820
    Replacement::draft_throws(0x140f24cb0, "unknown_140f24cb0", fn_140f24cb0 as *const (), Throws::Caught), // sub_140f24cb0
    Replacement::draft_throws(0x140f25140, "unknown_140f25140", tpl_18891::<0x20, 0x8, 0x140f26230> as *const (), Throws::Discard), // sub_140f25140
    Replacement::draft_throws(0x140f25180, "unknown_140f25180", tpl_18891::<0x20, 0x8, 0x140f26500> as *const (), Throws::Discard), // sub_140f25180
    Replacement::draft_throws(0x140f251c0, "unknown_140f251c0", fn_140f251c0 as *const (), Throws::Caught), // sub_140f251c0
    Replacement::draft_throws(0x140f25650, "unknown_140f25650", fn_140f25650 as *const (), Throws::Caught), // sub_140f25650
    Replacement::draft_throws(0x140f25af0, "unknown_140f25af0", fn_140f25af0 as *const (), Throws::Caught), // sub_140f25af0
    Replacement::draft_throws(0x140f25d60, "unknown_140f25d60", fn_140f25d60 as *const (), Throws::Caught), // sub_140f25d60
    Replacement::draft_throws(0x140f25fd0, "unknown_140f25fd0", fn_140f25fd0 as *const (), Throws::Caught), // sub_140f25fd0
    Replacement::draft_throws(0x140f26230, "unknown_140f26230", fn_140f26230 as *const (), Throws::Caught), // sub_140f26230
    Replacement::draft_throws(0x140f26310, "unknown_140f26310", fn_140f26310 as *const (), Throws::Caught), // sub_140f26310
    Replacement::draft_throws(0x140f26500, "unknown_140f26500", fn_140f26500 as *const (), Throws::Caught), // sub_140f26500
    Replacement::draft_throws(0x140f265d0, "unknown_140f265d0", fn_140f265d0 as *const (), Throws::Caught), // sub_140f265d0
    Replacement::draft_throws(0x140f26710, "unknown_140f26710", fn_140f26710 as *const (), Throws::Caught), // sub_140f26710
    Replacement::draft_throws(0x140f26850, "unknown_140f26850", fn_140f26850 as *const (), Throws::Caught), // sub_140f26850
    Replacement::draft(0x140f26a30, "unknown_140f26a30", fn_140f26a30 as *const ()), // sub_140f26a30
    Replacement::draft_throws(0x140f26bd0, "unknown_140f26bd0", fn_140f26bd0 as *const (), Throws::Caught), // sub_140f26bd0
    Replacement::draft_throws(0x140f26e90, "unknown_140f26e90", fn_140f26e90 as *const (), Throws::Caught), // sub_140f26e90
    Replacement::draft_throws(0x140f27430, "unknown_140f27430", fn_140f27430 as *const (), Throws::Caught), // sub_140f27430
    Replacement::draft_throws(0x140f276c0, "unknown_140f276c0", fn_140f276c0 as *const (), Throws::Caught), // sub_140f276c0
    Replacement::draft_throws(0x140f27c30, "unknown_140f27c30", fn_140f27c30 as *const (), Throws::Caught), // sub_140f27c30
    Replacement::draft_throws(0x140f27ec0, "unknown_140f27ec0", fn_140f27ec0 as *const (), Throws::Caught), // sub_140f27ec0
    Replacement::draft_throws(0x140f28210, "unknown_140f28210", fn_140f28210 as *const (), Throws::Caught), // sub_140f28210
    Replacement::draft_throws(0x140f28640, "unknown_140f28640", fn_140f28640 as *const (), Throws::Caught), // sub_140f28640
    Replacement::draft_throws(0x140f28990, "unknown_140f28990", fn_140f28990 as *const (), Throws::Caught), // sub_140f28990
    Replacement::draft_throws(0x140f28e90, "unknown_140f28e90", fn_140f28e90 as *const (), Throws::Caught), // sub_140f28e90
    Replacement::draft_throws(0x140f29340, "unknown_140f29340", fn_140f29340 as *const (), Throws::Caught), // sub_140f29340
    Replacement::draft(0x140f297f0, "unknown_140f297f0", fn_140f297f0 as *const ()), // sub_140f297f0
    Replacement::draft_throws(0x140f29970, "unknown_140f29970", tpl_1480::<0x1428a43d0, 0x1410d6ca0, 0x140f2a7e0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140f29970
    Replacement::draft(0x140f29c10, "unknown_140f29c10", tpl_1098::<0x143cc5300, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f29c10
    Replacement::draft(0x140f29f50, "unknown_140f29f50", tpl_1436::<0x58, 0x60, 0x158, 0x143cc5300, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140f29f50
    Replacement::draft(0x140f2a030, "unknown_140f2a030", tpl_1094::<0x143cc5168, 0x140f2ab10, 0x14020e8f0> as *const ()), // sub_140f2a030
    Replacement::draft_throws(0x140f2a230, "coregame_140f2a230", tpl_18900::<0x143cc5304, 0x143cc51c8, 0x1411e9f50, 0x140f2bdd0, 0x143cc52b8, 0x1420af120, 0x1420a28e0, 0x140f2b670, 0x140f2acb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143309996, 0x142646960, 0x143cc51d8, 0x143cc51e0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::wrinkle::streamIn>
    Replacement::draft_throws(0x140f2a500, "coregame_140f2a500", tpl_5126::<0x143cc51c0, 0x143cc52e0, 0x1411e9f50, 0x140f2bce0, 0x143cc5164, 0x1420af120, 0x1420a28e0, 0x140f2b860, 0x140f2aed0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143309cd6, 0x142646960, 0x143cc52f0, 0x143cc52f8, 0x143cc52fc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::wrinkle::streamOut>
    Replacement::draft_throws(0x140f2a7e0, "coregame_140f2a7e0", fn_140f2a7e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::wrinkle::update>
    Replacement::draft(0x140f2ab10, "unknown_140f2ab10", fn_140f2ab10 as *const ()), // sub_140f2ab10
    Replacement::draft_throws(0x140f2acb0, "unknown_140f2acb0", fn_140f2acb0 as *const (), Throws::Caught), // sub_140f2acb0
    Replacement::draft_throws(0x140f2ade0, "unknown_140f2ade0", tpl_5101::<0x18, 0x20, 0x40, 0x8, 0x141dc13a0> as *const (), Throws::Discard), // sub_140f2ade0
    Replacement::draft(0x140f2aed0, "unknown_140f2aed0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140f29f50> as *const ()), // sub_140f2aed0
    Replacement::draft(0x140f2af70, "unknown_140f2af70", tpl_5026::<0x88, 0x10088, 0x580e0, 0x143c900d8, 0x140f2b110, 0x140f2b020> as *const ()), // sub_140f2af70
    Replacement::draft(0x140f2b020, "unknown_140f2b020", tpl_5027::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141dc1140> as *const ()), // sub_140f2b020
    Replacement::draft(0x140f2c300, "coregame_140f2c300", fn_140f2c300 as *const ()), // ecs::system::ecs::system::wrapper<coregame::legacy_world::updateSoundManager>
    Replacement::draft(0x140f2c570, "coregame_140f2c570", fn_140f2c570 as *const ()), // ecs::system::ecs::system::wrapper<coregame::legacy_world::updateSignedDistanceField>
    Replacement::draft(0x140f2c810, "coregame_140f2c810", fn_140f2c810 as *const ()), // ecs::system::ecs::system::wrapper<coregame::legacy_world::updateRendererVariableSystems>
    Replacement::draft(0x140f2ca30, "coregame_140f2ca30", fn_140f2ca30 as *const ()), // sub_140f2ca30
    Replacement::draft(0x140f2cbb0, "coregame_140f2cbb0", fn_140f2cbb0 as *const ()), // sub_140f2cbb0
    Replacement::draft(0x140f2cbd0, "coregame_140f2cbd0", fn_140f2cbd0 as *const ()), // sub_140f2cbd0
    Replacement::draft(0x140f2cc30, "coregame_140f2cc30", tpl_650::<0x10> as *const ()), // sub_140f2cc30
    Replacement::draft(0x140f2cc40, "coregame_140f2cc40", fn_140f2cc40 as *const ()), // sub_140f2cc40
    Replacement::draft_throws(0x140f2ccb0, "coregame_140f2ccb0", tpl_1698::<0x143cc55a8, 0x143cc54a0, 0x1411e9f50, 0x140f2d390, 0x140f2daa0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14330a726, 0x142646960, 0x143cc54c0, 0x143cc54c2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::world::updateLegacyWorld>
    Replacement::draft(0x140f2cef0, "coregame_140f2cef0", fn_140f2cef0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::world::processLayerLoadRequests>
    Replacement::draft(0x140f2d140, "unknown_140f2d140", fn_140f2d140 as *const ()), // sub_140f2d140
    Replacement::draft(0x140f2d1a0, "unknown_140f2d1a0", fn_140f2d1a0 as *const ()), // sub_140f2d1a0
    Replacement::draft(0x140f2d210, "unknown_140f2d210", fn_140f2d210 as *const ()), // sub_140f2d210
    Replacement::draft_throws(0x140f2d390, "unknown_140f2d390", fn_140f2d390 as *const (), Throws::Discard), // sub_140f2d390
    Replacement::draft(0x140f2d800, "unknown_140f2d800", tpl_4419::<0x143c8ce9c, 0x140215610, 0x143cc5498, 0x143c8d0c0, 0x143c8d170, 0x143c8d174, 0x1420a28e0> as *const ()), // sub_140f2d800
    Replacement::draft_throws(0x140f2d980, "unknown_140f2d980", fn_140f2d980 as *const (), Throws::Discard), // sub_140f2d980
    Replacement::draft_throws(0x140f2dc00, "unknown_140f2dc00", tpl_1562::<0x143c8d170, 0x143c8d0c0, 0x143cc5498, 0x580e0, 0x143c8d174, 0x143c8ce9c, 0x141dc1f90> as *const (), Throws::Discard), // sub_140f2dc00
    Replacement::draft(0x140f2dc60, "unknown_140f2dc60", fn_140f2dc60 as *const ()), // sub_140f2dc60
    Replacement::draft(0x140f2de50, "unknown_140f2de50", fn_140f2de50 as *const ()), // sub_140f2de50
    Replacement::draft(0x140f2e060, "unknown_140f2e060", fn_140f2e060 as *const ()), // sub_140f2e060
    Replacement::draft(0x140f2e0f0, "unknown_140f2e0f0", fn_140f2e0f0 as *const ()), // sub_140f2e0f0
    Replacement::draft(0x140f2e320, "unknown_140f2e320", fn_140f2e320 as *const ()), // sub_140f2e320
    Replacement::draft(0x140f2e5c0, "unknown_140f2e5c0", fn_140f2e5c0 as *const ()), // sub_140f2e5c0
    Replacement::draft(0x140f2e6c0, "unknown_140f2e6c0", fn_140f2e6c0 as *const ()), // sub_140f2e6c0
    Replacement::draft(0x140f2e9a0, "unknown_140f2e9a0", tpl_18926::<0x78, 0x10, 0x18, 0x141e849c0, 0xffffffffffffffa0, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140f2e9a0
    Replacement::draft_throws(0x140f2ea40, "unknown_140f2ea40", fn_140f2ea40 as *const (), Throws::Caught), // sub_140f2ea40
    Replacement::draft_throws(0x140f2f1e0, "unknown_140f2f1e0", fn_140f2f1e0 as *const (), Throws::Caught), // sub_140f2f1e0
    Replacement::draft(0x140f2f290, "unknown_140f2f290", fn_140f2f290 as *const ()), // sub_140f2f290
    Replacement::draft(0x140f2f340, "unknown_140f2f340", fn_140f2f340 as *const ()), // sub_140f2f340
    Replacement::draft(0x140f2f390, "unknown_140f2f390", fn_140f2f390 as *const ()), // sub_140f2f390
    Replacement::draft(0x140f2f430, "unknown_140f2f430", fn_140f2f430 as *const ()), // sub_140f2f430
    Replacement::draft(0x140f2f490, "unknown_140f2f490", fn_140f2f490 as *const ()), // sub_140f2f490
    Replacement::draft(0x140f2f5c0, "unknown_140f2f5c0", fn_140f2f5c0 as *const ()), // sub_140f2f5c0
    Replacement::draft(0x140f2f750, "unknown_140f2f750", fn_140f2f750 as *const ()), // sub_140f2f750
    Replacement::draft(0x140f2f8a0, "wind_local_140f2f8a0", tpl_1764::<0x143cc55ac, 0x143cc55b0, 0x140f30c90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14330b0e6, 0x142646960, 0x143cc55d0, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<wind_local::updateWindControlData>
    Replacement::draft_throws(0x140f2faa0, "wind_local_140f2faa0", tpl_1584::<0x143cc55d4, 0x143cc55d8, 0x1411e9f50, 0x140f31770, 0x140f31a50, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14330b276, 0x142646960, 0x143cc55f8, 0x143cc5600, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<wind_local::sortWindBoxPriorityOrderFixed>
    Replacement::draft(0x140f2fcf0, "wind_local_140f2fcf0", fn_140f2fcf0 as *const ()), // ecs::system::ecs::system::wrapper<wind_local::updateWindDebugger>
    Replacement::draft(0x140f2ffd0, "wind_local_140f2ffd0", tpl_5528::<0x143cc5628, 0x143cc5630, 0x1411e9f50, 0x143cc56c8, 0x140215610, 0x1420a28e0, 0x140f31b20, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14330b626, 0x142646960, 0x143cc5650, 0x143cc5658, 0x143cc565c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<wind_local::sortWindBoxPriorityOrderVariable>
    Replacement::draft(0x140f30270, "wind_local_140f30270", fn_140f30270 as *const ()), // ecs::system::ecs::system::wrapper<wind_local::removedWindEntity>
    Replacement::draft(0x140f30560, "unknown_140f30560", tpl_1526::<0x1428a5938, 0x1410d6ca0, 0x140f30270, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140f30560
    Replacement::draft(0x140f30800, "unknown_140f30800", tpl_1363::<0x30, 0x143c8d694, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x140f3084d> as *const ()), // sub_140f30800
    Replacement::draft(0x140f30850, "unknown_140f30850", fn_140f30850 as *const ()), // sub_140f30850
    Replacement::draft(0x140f30ab0, "unknown_140f30ab0", tpl_1095::<0x143cc5688, 0x14020e8f0> as *const ()), // sub_140f30ab0
    Replacement::draft(0x140f30c90, "unknown_140f30c90", tpl_4416::<0x1411e9f50, 0x143cc56c8, 0x140215610, 0x143cc56cc, 0x143c8ce88, 0x1420a28e0, 0x140f31a10, 0x1401de410> as *const ()), // sub_140f30c90
    Replacement::draft(0x140f30e40, "unknown_140f30e40", fn_140f30e40 as *const ()), // sub_140f30e40
    Replacement::draft(0x140f31040, "unknown_140f31040", fn_140f31040 as *const ()), // sub_140f31040
    Replacement::draft(0x140f31330, "unknown_140f31330", fn_140f31330 as *const ()), // sub_140f31330
    Replacement::draft(0x140f31470, "unknown_140f31470", fn_140f31470 as *const ()), // sub_140f31470
    Replacement::draft_throws(0x140f31770, "unknown_140f31770", fn_140f31770 as *const (), Throws::Discard), // sub_140f31770
    Replacement::draft(0x140f318f0, "unknown_140f318f0", fn_140f318f0 as *const ()), // sub_140f318f0
    Replacement::draft_throws(0x140f31a50, "unknown_140f31a50", fn_140f31a50 as *const (), Throws::Discard), // sub_140f31a50
    Replacement::draft(0x140f31b40, "unknown_140f31b40", tpl_18942::<0x143cc5688, 0x5e308, 0x143cc56c8, 0x580e0, 0x10, 0x8, 0x141dc45a0, 0x1401de830> as *const ()), // sub_140f31b40
    Replacement::draft(0x140f31c00, "unknown_140f31c00", tpl_1991::<0x140f31eb0, 0x14330b800, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140f31c00
    Replacement::draft(0x140f31c70, "unknown_140f31c70", fn_140f31c70 as *const ()), // sub_140f31c70
    Replacement::draft(0x140f31cf0, "unknown_140f31cf0", fn_140f31cf0 as *const ()), // sub_140f31cf0
    Replacement::draft(0x140f31eb0, "unknown_140f31eb0", tpl_1997::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14330b800, 0x8> as *const ()), // sub_140f31eb0
    Replacement::draft(0x140f32090, "unknown_140f32090", tpl_5912::<0xfffffffffffffffe, 0x140f2f430, 0x142442de0> as *const ()), // sub_140f32090
    Replacement::draft(0x140f320e0, "unknown_140f320e0", fn_140f320e0 as *const ()), // sub_140f320e0
    Replacement::draft(0x140f322d0, "unknown_140f322d0", fn_140f322d0 as *const ()), // sub_140f322d0
    Replacement::draft(0x140f32560, "unknown_140f32560", tpl_18946::<0x143cc5814, 0x580e0, 0x141dc6870> as *const ()), // sub_140f32560
    Replacement::draft_throws(0x140f32580, "coregame_140f32580", tpl_5850::<0x143cc5978, 0x143cc58a0, 0x1411e9f50, 0x140f34a80, 0x143cc5854, 0x1420af120, 0x143c900d8, 0x140215610, 0x1420a28e0, 0x140f341c0, 0x140f339b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14330c3d6, 0x142646960, 0x143cc58c0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::orthographic_view::update>
    Replacement::draft_throws(0x140f32880, "coregame_140f32880", tpl_1075::<0x143cc585c, 0x143cc58d8, 0x1411e9f50, 0x140f34940, 0x143cc59c0, 0x1420af120, 0x143c900d8, 0x140215610, 0x1420a28e0, 0x140f34390, 0x140f33b00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14330c666, 0x142646960, 0x143cc58f8, 0x143cc58fa, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::perspective_view::update>
    Replacement::draft(0x140f32b90, "unknown_140f32b90", tpl_3270::<0x143cc597c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f32b90
    Replacement::draft(0x140f32ed0, "unknown_140f32ed0", tpl_1094::<0x143cc5858, 0x140f33670, 0x14020e8f0> as *const ()), // sub_140f32ed0
    Replacement::draft(0x140f330d0, "unknown_140f330d0", tpl_3270::<0x143cc58cc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f330d0
    Replacement::draft(0x140f33410, "unknown_140f33410", tpl_1094::<0x143cc5938, 0x140f33830, 0x14020e8f0> as *const ()), // sub_140f33410
    Replacement::draft(0x140f33670, "unknown_140f33670", fn_140f33670 as *const ()), // sub_140f33670
    Replacement::draft(0x140f33830, "unknown_140f33830", fn_140f33830 as *const ()), // sub_140f33830
    Replacement::draft(0x140f339b0, "unknown_140f339b0", tpl_60899::<0x143c900d8, 0x140f34000, 0x141dc76a0> as *const ()), // sub_140f339b0
    Replacement::draft(0x140f33b00, "unknown_140f33b00", tpl_60899::<0x143c900d8, 0x140f33ea0, 0x141dc7200> as *const ()), // sub_140f33b00
    Replacement::draft(0x140f33c50, "unknown_140f33c50", fn_140f33c50 as *const ()), // sub_140f33c50
    Replacement::draft(0x140f34160, "unknown_140f34160", tpl_18950::<0x300, 0x14330bca8> as *const ()), // sub_140f34160
    Replacement::draft(0x140f34190, "unknown_140f34190", tpl_18950::<0x2f0, 0x14330bca8> as *const ()), // sub_140f34190
    Replacement::draft(0x140f341c0, "unknown_140f341c0", tpl_60914::<0x143c900d8, 0x140f34000, 0x141dc76a0> as *const ()), // sub_140f341c0
    Replacement::draft(0x140f34390, "unknown_140f34390", tpl_60914::<0x143c900d8, 0x140f33ea0, 0x141dc7200> as *const ()), // sub_140f34390
    Replacement::draft(0x140f34940, "unknown_140f34940", tpl_18951::<0x143cc58c4, 0x143cc5868, 0x8, 0x10, 0x18, 0x141834a78, 0x143cc5938, 0x143c8f6d8, 0x143c8d120, 0x143cc58cc, 0x143cc5888, 0x143cc5898, 0x141834a0c> as *const ()), // sub_140f34940
    Replacement::draft(0x140f34a80, "unknown_140f34a80", tpl_18951::<0x143cc58c8, 0x143cc5940, 0x8, 0x10, 0x18, 0x141834a78, 0x143cc5858, 0x143c8d120, 0x143c8f6d8, 0x143cc597c, 0x143cc5960, 0x143cc5970, 0x141834a0c> as *const ()), // sub_140f34a80
    Replacement::draft_throws(0x140f34bc0, "unknown_140f34bc0", tpl_4541::<0x143cc59c0, 0x140f34940, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140f34bc0
    Replacement::draft_throws(0x140f34c50, "unknown_140f34c50", tpl_4541::<0x143cc5854, 0x140f34a80, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140f34c50
    Replacement::draft(0x140f34ce0, "unknown_140f34ce0", fn_140f34ce0 as *const ()), // sub_140f34ce0
    Replacement::draft(0x140f34e50, "unknown_140f34e50", fn_140f34e50 as *const ()), // sub_140f34e50
    Replacement::draft_throws(0x140f34e80, "unknown_140f34e80", fn_140f34e80 as *const (), Throws::Caught), // sub_140f34e80
    Replacement::draft_throws(0x140f35640, "unknown_140f35640", fn_140f35640 as *const (), Throws::Caught), // sub_140f35640
    Replacement::draft(0x140f357f0, "unknown_140f357f0", fn_140f357f0 as *const ()), // sub_140f357f0
    Replacement::draft(0x140f35950, "std_140f35950", fn_140f35950 as *const ()), // std::_Ref_count_obj_alloc3<rend::ImageCaptureRequest,r::Allocator<rend::ImageCaptureRequest,0>>::std::_Ref_count_obj_all
    Replacement::draft(0x140f359d0, "std_140f359d0", tpl_18958::<0x14330c900, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<rend::ImageCaptureRequest,r::Allocator<rend::ImageCaptureRequest,0>>::std::_Ref_count_obj_all
    Replacement::draft(0x140f35a00, "unknown_140f35a00", fn_140f35a00 as *const ()), // sub_140f35a00
    Replacement::draft(0x140f35a30, "unknown_140f35a30", tpl_18960::<0x20, 0x142442de0> as *const ()), // sub_140f35a30
    Replacement::draft(0x140f35a50, "unknown_140f35a50", fn_140f35a50 as *const ()), // sub_140f35a50
    Replacement::draft(0x140f35aa0, "unknown_140f35aa0", fn_140f35aa0 as *const ()), // sub_140f35aa0
    Replacement::draft_throws(0x140f35c10, "unknown_140f35c10", fn_140f35c10 as *const (), Throws::Caught), // sub_140f35c10
    Replacement::draft_throws(0x140f35e50, "unknown_140f35e50", fn_140f35e50 as *const (), Throws::Discard), // sub_140f35e50
    Replacement::draft(0x140f35f60, "unknown_140f35f60", fn_140f35f60 as *const ()), // sub_140f35f60
    Replacement::draft_throws(0x140f35fe0, "unknown_140f35fe0", fn_140f35fe0 as *const (), Throws::Discard), // sub_140f35fe0
    Replacement::draft(0x140f36050, "unknown_140f36050", fn_140f36050 as *const ()), // sub_140f36050
    Replacement::draft_throws(0x140f36190, "coregame_140f36190", fn_140f36190 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::update>
    Replacement::draft_throws(0x140f36770, "unknown_140f36770", fn_140f36770 as *const (), Throws::Caught), // sub_140f36770
    Replacement::draft(0x140f36da0, "unknown_140f36da0", fn_140f36da0 as *const ()), // sub_140f36da0
    Replacement::draft(0x140f36ed0, "unknown_140f36ed0", fn_140f36ed0 as *const ()), // sub_140f36ed0
    Replacement::draft_throws(0x140f37160, "unknown_140f37160", tpl_18971::<0xfffffffffffffffe, 0x140f35e50, 0x142442de0> as *const (), Throws::Discard), // sub_140f37160
    Replacement::draft(0x140f371b0, "unknown_140f371b0", fn_140f371b0 as *const ()), // sub_140f371b0
    Replacement::draft(0x140f37240, "unknown_140f37240", fn_140f37240 as *const ()), // sub_140f37240
    Replacement::draft_throws(0x140f372c0, "unknown_140f372c0", fn_140f372c0 as *const (), Throws::Caught), // sub_140f372c0
    Replacement::draft_throws(0x140f37350, "unknown_140f37350", fn_140f37350 as *const (), Throws::Caught), // sub_140f37350
    Replacement::draft(0x140f373c0, "unknown_140f373c0", fn_140f373c0 as *const ()), // sub_140f373c0
    Replacement::draft(0x140f37580, "unknown_140f37580", tpl_18977::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e340, 0x140f39b90, 0x143107164, 0x140f39d50, 0x14330d1a0, 0x140f39f10, 0x14330d054> as *const ()), // sub_140f37580
    Replacement::draft(0x140f37730, "unknown_140f37730", tpl_18977::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e458, 0x140f3a0d0, 0x143107164, 0x140f3a290, 0x14330d1a0, 0x140f3a450, 0x14330d054> as *const ()), // sub_140f37730
    Replacement::draft(0x140f378e0, "unknown_140f378e0", tpl_18977::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330d910, 0x140f3a610, 0x143107164, 0x140f3a7d0, 0x14330d1a0, 0x140f3a990, 0x14330d054> as *const ()), // sub_140f378e0
    Replacement::draft(0x140f37a90, "unknown_140f37a90", tpl_18978::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330dfb8, 0x140f3ab50, 0x143107164, 0x140f3ad10, 0x14330d1a0> as *const ()), // sub_140f37a90
    Replacement::draft(0x140f37bf0, "unknown_140f37bf0", tpl_18978::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330ddd8, 0x140f3aed0, 0x143107164, 0x140f3b090, 0x14330d1a0> as *const ()), // sub_140f37bf0
    Replacement::draft(0x140f37d50, "unknown_140f37d50", tpl_18979::<0x30, 0x10, 0x18, 0x141deed60, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140f37d50
    Replacement::draft(0x140f37df0, "unknown_140f37df0", tpl_18977::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e7a0, 0x140f3b950, 0x143107164, 0x140f3bb10, 0x14330d1a0, 0x140f3bcd0, 0x14330d054> as *const ()), // sub_140f37df0
    Replacement::draft(0x140f37fa0, "unknown_140f37fa0", tpl_18978::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e0f8, 0x140f3be90, 0x143107164, 0x140f3c050, 0x14330d1a0> as *const ()), // sub_140f37fa0
    Replacement::draft(0x140f38100, "unknown_140f38100", tpl_18977::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330d780, 0x140f3c3b0, 0x1431a72a0, 0x140f3c570, 0x143107164, 0x140f3c730, 0x14330d1a0> as *const ()), // sub_140f38100
    Replacement::draft(0x140f382b0, "unknown_140f382b0", tpl_18978::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330d550, 0x140f3c8f0, 0x143107164, 0x140f3cab0, 0x14330d1a0> as *const ()), // sub_140f382b0
    Replacement::draft(0x140f38410, "unknown_140f38410", tpl_18979::<0x30, 0x10, 0x18, 0x141dee330, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140f38410
    Replacement::draft(0x140f384b0, "unknown_140f384b0", tpl_18978::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e840, 0x140f3d370, 0x143107164, 0x140f3d530, 0x14330d1a0> as *const ()), // sub_140f384b0
    Replacement::draft(0x140f38610, "unknown_140f38610", tpl_18977::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330d960, 0x140f3d890, 0x14330d19c, 0x140f3da50, 0x143107164, 0x140f3dc10, 0x14330d1a0> as *const ()), // sub_140f38610
    Replacement::draft(0x140f387c0, "unknown_140f387c0", tpl_18978::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e198, 0x140f3ddd0, 0x143107164, 0x140f3df90, 0x14330d1a0> as *const ()), // sub_140f387c0
    Replacement::draft(0x140f38920, "unknown_140f38920", fn_140f38920 as *const ()), // sub_140f38920
    Replacement::draft(0x140f38ae0, "unknown_140f38ae0", tpl_18981::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e2f0, 0x140f3c210> as *const ()), // sub_140f38ae0
    Replacement::draft(0x140f38c50, "unknown_140f38c50", tpl_18978::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e660, 0x140f3e9d0, 0x1431a72a0, 0x140f3eb90, 0x143078c04> as *const ()), // sub_140f38c50
    Replacement::draft(0x140f38db0, "unknown_140f38db0", tpl_18979::<0x30, 0x10, 0x18, 0x141ded800, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140f38db0
    Replacement::draft(0x140f38e50, "unknown_140f38e50", tpl_18982::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e058> as *const ()), // sub_140f38e50
    Replacement::draft(0x140f38ee0, "unknown_140f38ee0", tpl_18981::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e8e0, 0x140f3d6f0> as *const ()), // sub_140f38ee0
    Replacement::draft(0x140f39050, "unknown_140f39050", tpl_18983::<0x20, 0x10, 0x18, 0x14330daa0, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140f39050
    Replacement::draft(0x140f390f0, "unknown_140f390f0", tpl_18978::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330db90, 0x140f3f0b0, 0x1431a72a0, 0x140f3f270, 0x14330cf60> as *const ()), // sub_140f390f0
    Replacement::draft(0x140f39250, "unknown_140f39250", fn_140f39250 as *const ()), // sub_140f39250
    Replacement::draft(0x140f39410, "unknown_140f39410", tpl_18982::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14330df68> as *const ()), // sub_140f39410
    Replacement::draft(0x140f394a0, "unknown_140f394a0", tpl_18983::<0x20, 0x10, 0x18, 0x14330e288, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140f394a0
    Replacement::draft(0x140f39540, "unknown_140f39540", tpl_18983::<0x20, 0x10, 0x18, 0x14330dc30, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140f39540
    Replacement::draft(0x140f395e0, "unknown_140f395e0", tpl_18979::<0x30, 0x10, 0x18, 0x141decfc0, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0> as *const ()), // sub_140f395e0
    Replacement::draft(0x140f39680, "unknown_140f39680", tpl_18985::<0x8, 0x141dce330, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f39680
    Replacement::draft(0x140f39750, "unknown_140f39750", tpl_477::<0x8, 0x142442de0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f39750
    Replacement::draft(0x140f39830, "unknown_140f39830", tpl_18986::<0x141dd1c90, 0x14330d138, 0x1425371a0, 0x142533200> as *const ()), // sub_140f39830
    Replacement::draft(0x140f399f0, "unknown_140f399f0", tpl_18987::<0x141dd1c90, 0x1425371a0, 0x142533200> as *const ()), // sub_140f399f0
    Replacement::draft(0x140f39b90, "unknown_140f39b90", tpl_18986::<0x141dd1ec0, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f39b90
    Replacement::draft(0x140f39d50, "unknown_140f39d50", tpl_18986::<0x141dd1ec0, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f39d50
    Replacement::draft(0x140f39f10, "unknown_140f39f10", tpl_18986::<0x141dd1ec0, 0x14330d054, 0x1425371a0, 0x142533200> as *const ()), // sub_140f39f10
    Replacement::draft(0x140f3a0d0, "unknown_140f3a0d0", tpl_18986::<0x141dd20f0, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3a0d0
    Replacement::draft(0x140f3a290, "unknown_140f3a290", tpl_18986::<0x141dd20f0, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3a290
    Replacement::draft(0x140f3a450, "unknown_140f3a450", tpl_18986::<0x141dd20f0, 0x14330d054, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3a450
    Replacement::draft(0x140f3a610, "unknown_140f3a610", tpl_18986::<0x141dd2320, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3a610
    Replacement::draft(0x140f3a7d0, "unknown_140f3a7d0", tpl_18986::<0x141dd2320, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3a7d0
    Replacement::draft(0x140f3a990, "unknown_140f3a990", tpl_18986::<0x141dd2320, 0x14330d054, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3a990
    Replacement::draft(0x140f3ab50, "unknown_140f3ab50", tpl_18986::<0x141dd2550, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3ab50
    Replacement::draft(0x140f3ad10, "unknown_140f3ad10", tpl_18986::<0x141dd2550, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3ad10
    Replacement::draft(0x140f3aed0, "unknown_140f3aed0", tpl_18986::<0x141dd2780, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3aed0
    Replacement::draft(0x140f3b090, "unknown_140f3b090", tpl_18986::<0x141dd2780, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3b090
    Replacement::draft(0x140f3b250, "unknown_140f3b250", tpl_18986::<0x141dd29b0, 0x14330d19c, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3b250
    Replacement::draft(0x140f3b410, "unknown_140f3b410", tpl_18986::<0x141dd29b0, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3b410
    Replacement::draft(0x140f3b5d0, "unknown_140f3b5d0", tpl_18986::<0x141dd29b0, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3b5d0
    Replacement::draft(0x140f3b790, "unknown_140f3b790", tpl_18986::<0x141dd29b0, 0x14330d054, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3b790
    Replacement::draft(0x140f3b950, "unknown_140f3b950", tpl_18986::<0x141dd2be0, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3b950
    Replacement::draft(0x140f3bb10, "unknown_140f3bb10", tpl_18986::<0x141dd2be0, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3bb10
    Replacement::draft(0x140f3bcd0, "unknown_140f3bcd0", tpl_18986::<0x141dd2be0, 0x14330d054, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3bcd0
    Replacement::draft(0x140f3be90, "unknown_140f3be90", tpl_18986::<0x141dd2e10, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3be90
    Replacement::draft(0x140f3c050, "unknown_140f3c050", tpl_18986::<0x141dd2e10, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3c050
    Replacement::draft(0x140f3c210, "unknown_140f3c210", tpl_18988::<0x141dd3040, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3c210
    Replacement::draft(0x140f3c3b0, "unknown_140f3c3b0", tpl_18986::<0x141dd3270, 0x1431a72a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3c3b0
    Replacement::draft(0x140f3c570, "unknown_140f3c570", tpl_18986::<0x141dd3270, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3c570
    Replacement::draft(0x140f3c730, "unknown_140f3c730", tpl_18986::<0x141dd3270, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3c730
    Replacement::draft(0x140f3c8f0, "unknown_140f3c8f0", tpl_18986::<0x141dd34a0, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3c8f0
    Replacement::draft(0x140f3cab0, "unknown_140f3cab0", tpl_18986::<0x141dd34a0, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3cab0
    Replacement::draft(0x140f3cc70, "unknown_140f3cc70", tpl_18986::<0x141dd36d0, 0x14330d19c, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3cc70
    Replacement::draft(0x140f3ce30, "unknown_140f3ce30", tpl_18986::<0x141dd36d0, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3ce30
    Replacement::draft(0x140f3cff0, "unknown_140f3cff0", tpl_18986::<0x141dd36d0, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3cff0
    Replacement::draft(0x140f3d1b0, "unknown_140f3d1b0", tpl_18986::<0x141dd36d0, 0x14330d054, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3d1b0
    Replacement::draft(0x140f3d370, "unknown_140f3d370", tpl_18986::<0x141dd3900, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3d370
    Replacement::draft(0x140f3d530, "unknown_140f3d530", tpl_18986::<0x141dd3900, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3d530
    Replacement::draft(0x140f3d6f0, "unknown_140f3d6f0", tpl_18988::<0x141dd3b30, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3d6f0
    Replacement::draft(0x140f3d890, "unknown_140f3d890", tpl_18986::<0x141dd3d60, 0x14330d19c, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3d890
    Replacement::draft(0x140f3da50, "unknown_140f3da50", tpl_18986::<0x141dd3d60, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3da50
    Replacement::draft(0x140f3dc10, "unknown_140f3dc10", tpl_18986::<0x141dd3d60, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3dc10
    Replacement::draft(0x140f3ddd0, "unknown_140f3ddd0", tpl_18986::<0x141dd3f90, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3ddd0
    Replacement::draft(0x140f3df90, "unknown_140f3df90", tpl_18986::<0x141dd3f90, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3df90
    Replacement::draft(0x140f3e150, "unknown_140f3e150", tpl_18987::<0x141dd41c0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3e150
    Replacement::draft(0x140f3e2f0, "unknown_140f3e2f0", tpl_18986::<0x141dd43f0, 0x1431a72a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3e2f0
    Replacement::draft(0x140f3e4b0, "unknown_140f3e4b0", tpl_18986::<0x141dd43f0, 0x14330cfa0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3e4b0
    Replacement::draft(0x140f3e670, "unknown_140f3e670", tpl_18987::<0x141dd43f0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3e670
    Replacement::draft(0x140f3e810, "unknown_140f3e810", tpl_18986::<0x141dd43f0, 0x14330cf18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3e810
    Replacement::draft(0x140f3e9d0, "unknown_140f3e9d0", tpl_18986::<0x141dd4620, 0x1431a72a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3e9d0
    Replacement::draft(0x140f3eb90, "unknown_140f3eb90", tpl_18986::<0x141dd4620, 0x143078c04, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3eb90
    Replacement::draft(0x140f3ed50, "unknown_140f3ed50", tpl_18987::<0x141dd4850, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3ed50
    Replacement::draft(0x140f3eef0, "unknown_140f3eef0", tpl_18986::<0x141dd4850, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3eef0
    Replacement::draft(0x140f3f0b0, "unknown_140f3f0b0", tpl_18986::<0x141dd4a80, 0x1431a72a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3f0b0
    Replacement::draft(0x140f3f270, "unknown_140f3f270", tpl_18986::<0x141dd4a80, 0x14330cf60, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3f270
    Replacement::draft(0x140f3f430, "unknown_140f3f430", tpl_18986::<0x141dd4cb0, 0x14330cf50, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3f430
    Replacement::draft(0x140f3f5f0, "unknown_140f3f5f0", tpl_18987::<0x141dd4cb0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3f5f0
    Replacement::draft(0x140f3f790, "unknown_140f3f790", tpl_18986::<0x141dd4cb0, 0x14330cecc, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3f790
    Replacement::draft(0x140f3f950, "unknown_140f3f950", tpl_18987::<0x141dd4ee0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3f950
    Replacement::draft(0x140f3faf0, "unknown_140f3faf0", tpl_18986::<0x141dd4ee0, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3faf0
    Replacement::draft(0x140f3fcb0, "unknown_140f3fcb0", tpl_18987::<0x141dd5110, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3fcb0
    Replacement::draft(0x140f3fe50, "unknown_140f3fe50", tpl_18986::<0x141dd5110, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f3fe50
    Replacement::draft(0x140f40010, "unknown_140f40010", tpl_18986::<0x141dd5110, 0x14330d054, 0x1425371a0, 0x142533200> as *const ()), // sub_140f40010
    Replacement::draft(0x140f401d0, "unknown_140f401d0", tpl_18987::<0x141dd5340, 0x1425371a0, 0x142533200> as *const ()), // sub_140f401d0
    Replacement::draft(0x140f40370, "unknown_140f40370", tpl_18986::<0x141dd5340, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f40370
    Replacement::draft(0x140f40530, "unknown_140f40530", tpl_18988::<0x141dd5570, 0x1425371a0, 0x142533200> as *const ()), // sub_140f40530
    Replacement::draft(0x140f406d0, "unknown_140f406d0", tpl_18986::<0x141dd57a0, 0x1431a72a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f406d0
    Replacement::draft(0x140f40890, "unknown_140f40890", tpl_18986::<0x141dd57a0, 0x143078c04, 0x1425371a0, 0x142533200> as *const ()), // sub_140f40890
    Replacement::draft(0x140f40a50, "unknown_140f40a50", tpl_18987::<0x141dd59d0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f40a50
    Replacement::draft(0x140f40bf0, "unknown_140f40bf0", tpl_18986::<0x141dd59d0, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f40bf0
    Replacement::draft(0x140f40db0, "unknown_140f40db0", tpl_18987::<0x141dd5c00, 0x1425371a0, 0x142533200> as *const ()), // sub_140f40db0
    Replacement::draft(0x140f40f50, "unknown_140f40f50", tpl_18986::<0x141dd5c00, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f40f50
    Replacement::draft(0x140f41110, "unknown_140f41110", tpl_18986::<0x141dd5c00, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f41110
    Replacement::draft(0x140f412d0, "unknown_140f412d0", tpl_18986::<0x141dd5c00, 0x14330d054, 0x1425371a0, 0x142533200> as *const ()), // sub_140f412d0
    Replacement::draft(0x140f41490, "unknown_140f41490", tpl_18987::<0x141dd5e30, 0x1425371a0, 0x142533200> as *const ()), // sub_140f41490
    Replacement::draft(0x140f41630, "unknown_140f41630", tpl_18986::<0x141dd5e30, 0x143107164, 0x1425371a0, 0x142533200> as *const ()), // sub_140f41630
    Replacement::draft(0x140f417f0, "unknown_140f417f0", tpl_18986::<0x141dd5e30, 0x14330d1a0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f417f0
    Replacement::draft(0x140f419b0, "unknown_140f419b0", tpl_18989::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e4f8, 0x14330e610, 0x1431a72a0, 0x140f3f950, 0x14330d19c, 0x140f3faf0, 0x14330d1a0> as *const ()), // sub_140f419b0
    Replacement::draft(0x140f41bd0, "unknown_140f41bd0", tpl_18990::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14330d898, 0x141dec6a0> as *const ()), // sub_140f41bd0
    Replacement::draft(0x140f41cc0, "unknown_140f41cc0", tpl_18989::<0x142535070, 0x1426456c8, 0x1426453a0, 0x14330e390, 0x14330d298, 0x1431a72a0, 0x140f401d0, 0x14330d19c, 0x140f40370, 0x14330d1a0> as *const ()), // sub_140f41cc0
    Replacement::draft(0x140f41ee0, "unknown_140f41ee0", fn_140f41ee0 as *const ()), // sub_140f41ee0
    Replacement::draft(0x140f42090, "unknown_140f42090", fn_140f42090 as *const ()), // sub_140f42090
    Replacement::draft(0x140f42240, "unknown_140f42240", fn_140f42240 as *const ()), // sub_140f42240
    Replacement::draft(0x140f42450, "unknown_140f42450", tpl_18990::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14330e980, 0x141debd90> as *const ()), // sub_140f42450
    Replacement::draft(0x140f42540, "unknown_140f42540", tpl_18990::<0x40, 0x10, 0x18, 0xffffffffffffffd8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14330def0, 0x141debb70> as *const ()), // sub_140f42540
    Replacement::draft(0x140f42630, "unknown_140f42630", fn_140f42630 as *const ()), // sub_140f42630
    Replacement::draft(0x140f426c0, "unknown_140f426c0", tpl_3605::<0x19, 0x10, 0x140f426c0, 0x28, 0x140f39680, 0x1403e8060> as *const ()), // sub_140f426c0
    Replacement::draft(0x140f42740, "unknown_140f42740", tpl_77368::<0x141dd5e30, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f42740
    Replacement::draft(0x140f42890, "unknown_140f42890", tpl_77368::<0x141dd5c00, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f42890
    Replacement::draft(0x140f429e0, "unknown_140f429e0", tpl_77368::<0x141dd59d0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f429e0
    Replacement::draft(0x140f42b30, "unknown_140f42b30", tpl_77368::<0x141dd57a0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f42b30
    Replacement::draft(0x140f42c80, "unknown_140f42c80", tpl_77369::<0x141dd5570, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f42c80
    Replacement::draft(0x140f42dd0, "unknown_140f42dd0", tpl_77368::<0x141dd5340, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f42dd0
    Replacement::draft(0x140f42f20, "unknown_140f42f20", tpl_77368::<0x141dd5110, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f42f20
    Replacement::draft(0x140f43070, "unknown_140f43070", tpl_77368::<0x141dd4ee0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43070
    Replacement::draft(0x140f431c0, "unknown_140f431c0", tpl_77368::<0x141dd4cb0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f431c0
    Replacement::draft(0x140f43310, "unknown_140f43310", tpl_77368::<0x141dd4a80, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43310
    Replacement::draft(0x140f43460, "unknown_140f43460", tpl_77368::<0x141dd4850, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43460
    Replacement::draft(0x140f435b0, "unknown_140f435b0", tpl_77368::<0x141dd4620, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f435b0
    Replacement::draft(0x140f43700, "unknown_140f43700", tpl_77368::<0x141dd43f0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43700
    Replacement::draft(0x140f43850, "unknown_140f43850", tpl_77368::<0x141dd41c0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43850
    Replacement::draft(0x140f439a0, "unknown_140f439a0", tpl_77368::<0x141dd3f90, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f439a0
    Replacement::draft(0x140f43af0, "unknown_140f43af0", tpl_77368::<0x141dd3d60, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43af0
    Replacement::draft(0x140f43c40, "unknown_140f43c40", tpl_77369::<0x141dd3b30, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43c40
    Replacement::draft(0x140f43d90, "unknown_140f43d90", tpl_77368::<0x141dd3900, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43d90
    Replacement::draft(0x140f43ee0, "unknown_140f43ee0", tpl_77368::<0x141dd36d0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f43ee0
    Replacement::draft(0x140f44030, "unknown_140f44030", tpl_77368::<0x141dd34a0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44030
    Replacement::draft(0x140f44180, "unknown_140f44180", tpl_77368::<0x141dd3270, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44180
    Replacement::draft(0x140f442d0, "unknown_140f442d0", tpl_77369::<0x141dd3040, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f442d0
    Replacement::draft(0x140f44420, "unknown_140f44420", tpl_77368::<0x141dd2e10, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44420
    Replacement::draft(0x140f44570, "unknown_140f44570", tpl_77368::<0x141dd2be0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44570
    Replacement::draft(0x140f446c0, "unknown_140f446c0", tpl_77368::<0x141dd29b0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f446c0
    Replacement::draft(0x140f44810, "unknown_140f44810", tpl_77368::<0x141dd2780, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44810
    Replacement::draft(0x140f44960, "unknown_140f44960", tpl_77368::<0x141dd2550, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44960
    Replacement::draft(0x140f44ab0, "unknown_140f44ab0", tpl_77368::<0x141dd2320, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44ab0
    Replacement::draft(0x140f44c00, "unknown_140f44c00", tpl_77368::<0x141dd20f0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44c00
    Replacement::draft(0x140f44d50, "unknown_140f44d50", tpl_77368::<0x141dd1ec0, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44d50
    Replacement::draft(0x140f44ea0, "unknown_140f44ea0", tpl_77368::<0x141dd1c90, 0x14183acc8, 0x1425371a0, 0x142533200> as *const ()), // sub_140f44ea0
    Replacement::draft(0x140f44ff0, "unknown_140f44ff0", fn_140f44ff0 as *const ()), // sub_140f44ff0
    Replacement::draft(0x140f45160, "unknown_140f45160", tpl_475::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140f39750> as *const ()), // sub_140f45160
    Replacement::draft(0x140f452d0, "unknown_140f452d0", fn_140f452d0 as *const ()), // sub_140f452d0
    Replacement::draft(0x140f45520, "unknown_140f45520", fn_140f45520 as *const ()), // sub_140f45520
    Replacement::draft(0x140f45570, "unknown_140f45570", fn_140f45570 as *const ()), // sub_140f45570
    Replacement::draft(0x140f455e0, "cohtml_140f455e0", fn_140f455e0 as *const ()), // sub_140f455e0
    Replacement::draft(0x140f457d0, "unknown_140f457d0", fn_140f457d0 as *const ()), // sub_140f457d0
    Replacement::draft(0x140f45840, "unknown_140f45840", fn_140f45840 as *const ()), // sub_140f45840
    Replacement::draft(0x140f459c0, "unknown_140f459c0", fn_140f459c0 as *const ()), // sub_140f459c0
    Replacement::draft(0x140f45aa0, "unknown_140f45aa0", tpl_5148::<0xc, 0x8, 0x1403f3e60, 0x3, 0x4, 0x14240a730> as *const ()), // sub_140f45aa0
    Replacement::draft(0x140f45b70, "cohtml_140f45b70", tpl_1676::<0x142646d58, 0x14309f708, 0x14330eb40, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSettingsGroup,r::Allocator<coregame::UIUserSettings:
    Replacement::draft(0x140f45bd0, "cohtml_140f45bd0", tpl_1677::<0x18, 0x140f47490> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSettingsGroup,r::Allocator<coregame::UIUserSettings:
    Replacement::draft(0x140f45c00, "cohtml_140f45c00", tpl_1678::<0x8, 0x230, 0x18, 0x140f47490> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSettingsGroup,r::Allocator<coregame::UIUserSettings:
    Replacement::draft(0x140f45c50, "cohtml_140f45c50", tpl_1677::<0x18, 0x140f475d0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSettingsGroup,r::Allocator<coregame::UIUserSettings:
    Replacement::draft(0x140f45c80, "cohtml_140f45c80", tpl_1679::<0x8, 0x90, 0x18, 0x140f475d0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSettingsGroup,r::Allocator<coregame::UIUserSettings:
    Replacement::draft(0x140f45cd0, "cohtml_140f45cd0", tpl_61376::<0x78, 0x70, 0x140f475d0, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSettingsGroup,r::Allocator<coregame::UIUserSettings:
    Replacement::draft(0x140f45d90, "cohtml_140f45d90", tpl_61377::<0x140264460, 0x140f4a510, 0x141df49f0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSettingsGroup,r::Allocator<coregame::UIUserSettings:
    Replacement::draft(0x140f45e90, "cohtml_140f45e90", tpl_1676::<0x142646d58, 0x14309f708, 0x14330ec50, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSetting,r::Allocator<coregame::UIUserSettings::UserS
    Replacement::draft(0x140f45ef0, "cohtml_140f45ef0", tpl_1677::<0x18, 0x140f47690> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSetting,r::Allocator<coregame::UIUserSettings::UserS
    Replacement::draft(0x140f45f20, "cohtml_140f45f20", tpl_1678::<0x8, 0x230, 0x18, 0x140f47690> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSetting,r::Allocator<coregame::UIUserSettings::UserS
    Replacement::draft(0x140f45f70, "cohtml_140f45f70", tpl_1677::<0x18, 0x140f477f0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSetting,r::Allocator<coregame::UIUserSettings::UserS
    Replacement::draft(0x140f45fa0, "cohtml_140f45fa0", tpl_1679::<0x8, 0x90, 0x18, 0x140f477f0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSetting,r::Allocator<coregame::UIUserSettings::UserS
    Replacement::draft(0x140f45ff0, "cohtml_140f45ff0", tpl_61376::<0x78, 0x70, 0x140f477f0, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSetting,r::Allocator<coregame::UIUserSettings::UserS
    Replacement::draft(0x140f460b0, "cohtml_140f460b0", tpl_61377::<0x140264460, 0x140f4a520, 0x141df4ae0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::UserSetting,r::Allocator<coregame::UIUserSettings::UserS
    Replacement::draft(0x140f461b0, "cohtml_140f461b0", tpl_1676::<0x142646d58, 0x14309f708, 0x14330eab8, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<bool coregame::UIUserSettings::UserSettingsGroup::*>::cohtml::FieldProperty<bool coregame::UIUserS
    Replacement::draft(0x140f46210, "cohtml_140f46210", tpl_1676::<0x142646d58, 0x14309f708, 0x14330f090, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::InplaceString<32> coregame::UIUserSettings::UserSettingsGroup::*>::cohtml::FieldProperty<r::Inp
    Replacement::draft(0x140f46270, "cohtml_140f46270", tpl_1676::<0x142646d58, 0x14309f708, 0x14330ede8, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::DynamicArray<r::InplaceString<32>,r::Allocator<r::InplaceString<32>,0>> coregame::UIUserSetting
    Replacement::draft(0x140f462d0, "cohtml_140f462d0", tpl_1677::<0x18, 0x1403f3cb0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<r::InplaceString<32>,r::Allocator<r::InplaceString<32>,0>> coregame::UIUserSetting
    Replacement::draft(0x140f46300, "cohtml_140f46300", tpl_1678::<0x8, 0x230, 0x18, 0x1403f3cb0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<r::InplaceString<32>,r::Allocator<r::InplaceString<32>,0>> coregame::UIUserSetting
    Replacement::draft(0x140f46350, "cohtml_140f46350", tpl_1677::<0x18, 0x140f478a0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<r::InplaceString<32>,r::Allocator<r::InplaceString<32>,0>> coregame::UIUserSetting
    Replacement::draft(0x140f46380, "cohtml_140f46380", tpl_1679::<0x8, 0x90, 0x18, 0x140f478a0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<r::InplaceString<32>,r::Allocator<r::InplaceString<32>,0>> coregame::UIUserSetting
    Replacement::draft(0x140f463d0, "cohtml_140f463d0", tpl_61376::<0x78, 0x70, 0x140f478a0, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<r::DynamicArray<r::InplaceString<32>,r::Allocator<r::InplaceString<32>,0>> coregame::UIUserSetting
    Replacement::draft(0x140f46490, "cohtml_140f46490", fn_140f46490 as *const ()), // cohtml::FieldProperty<r::DynamicArray<r::InplaceString<32>,r::Allocator<r::InplaceString<32>,0>> coregame::UIUserSetting
    Replacement::draft(0x140f46530, "cohtml_140f46530", tpl_1676::<0x142646d58, 0x14309f708, 0x14330ebc8, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<bool coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<bool coregame::UIUserSetting
    Replacement::draft(0x140f46590, "cohtml_140f46590", tpl_1676::<0x142646d58, 0x14309f708, 0x14330ed60, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::PossibleValue,r::Allocator<coregame::UIUserSettings::Pos
    Replacement::draft(0x140f465f0, "cohtml_140f465f0", tpl_1677::<0x18, 0x140f47980> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::PossibleValue,r::Allocator<coregame::UIUserSettings::Pos
    Replacement::draft(0x140f46620, "cohtml_140f46620", tpl_1678::<0x8, 0x230, 0x18, 0x140f47980> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::PossibleValue,r::Allocator<coregame::UIUserSettings::Pos
    Replacement::draft(0x140f46670, "cohtml_140f46670", tpl_1677::<0x18, 0x140f47ac0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::PossibleValue,r::Allocator<coregame::UIUserSettings::Pos
    Replacement::draft(0x140f466a0, "cohtml_140f466a0", tpl_1679::<0x8, 0x90, 0x18, 0x140f47ac0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::PossibleValue,r::Allocator<coregame::UIUserSettings::Pos
    Replacement::draft(0x140f466f0, "cohtml_140f466f0", tpl_61376::<0x78, 0x70, 0x140f47ac0, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::PossibleValue,r::Allocator<coregame::UIUserSettings::Pos
    Replacement::draft(0x140f467b0, "cohtml_140f467b0", tpl_61377::<0x140264460, 0x140f4a540, 0x141df4d40> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UIUserSettings::PossibleValue,r::Allocator<coregame::UIUserSettings::Pos
    Replacement::draft(0x140f468b0, "cohtml_140f468b0", tpl_1676::<0x142646d58, 0x14309f708, 0x14330ee70, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<coregame::UIUserSettings::UserSettingValue coregame::UIUserSettings::UserSetting::*>::cohtml::Fiel
    Replacement::draft(0x140f46910, "cohtml_140f46910", tpl_1681::<0x18, 0x38, 0x141df4e50, 0x48> as *const ()), // cohtml::FieldProperty<coregame::UIUserSettings::UserSettingValue coregame::UIUserSettings::UserSetting::*>::cohtml::Fiel
    Replacement::draft(0x140f46960, "cohtml_140f46960", tpl_1682::<0x8, 0x230, 0x18, 0x38, 0x141df4e50, 0x48> as *const ()), // cohtml::FieldProperty<coregame::UIUserSettings::UserSettingValue coregame::UIUserSettings::UserSetting::*>::cohtml::Fiel
    Replacement::draft(0x140f469d0, "cohtml_140f469d0", tpl_1681::<0x18, 0x28, 0x141df4e50, 0x48> as *const ()), // cohtml::FieldProperty<coregame::UIUserSettings::UserSettingValue coregame::UIUserSettings::UserSetting::*>::cohtml::Fiel
    Replacement::draft(0x140f46a20, "cohtml_140f46a20", tpl_1683::<0x8, 0x90, 0x18, 0x28, 0x141df4e50, 0x48> as *const ()), // cohtml::FieldProperty<coregame::UIUserSettings::UserSettingValue coregame::UIUserSettings::UserSetting::*>::cohtml::Fiel
    Replacement::draft(0x140f46a90, "cohtml_140f46a90", tpl_1676::<0x142646d58, 0x14309f708, 0x14330f008, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<unsigned int coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft(0x140f46af0, "cohtml_140f46af0", tpl_1757::<0x18, 0x208> as *const ()), // cohtml::FieldProperty<unsigned int coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft(0x140f46b20, "cohtml_140f46b20", tpl_1758::<0x8, 0x230, 0x18, 0x208> as *const ()), // cohtml::FieldProperty<unsigned int coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft(0x140f46b80, "cohtml_140f46b80", tpl_19001::<0x18, 0x108> as *const ()), // cohtml::FieldProperty<unsigned int coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft(0x140f46bb0, "cohtml_140f46bb0", tpl_19002::<0x8, 0x90, 0x18, 0x108> as *const ()), // cohtml::FieldProperty<unsigned int coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft(0x140f46c00, "cohtml_140f46c00", tpl_19003::<0x78, 0x142646c48> as *const ()), // cohtml::FieldProperty<unsigned int coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft(0x140f46c30, "cohtml_140f46c30", fn_140f46c30 as *const ()), // cohtml::FieldProperty<unsigned int coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft(0x140f46c60, "cohtml_140f46c60", fn_140f46c60 as *const ()), // cohtml::FieldProperty<unsigned int coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft(0x140f46c80, "cohtml_140f46c80", tpl_1676::<0x142646d58, 0x14309f708, 0x14330ea30, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::InplaceString<32> coregame::UIUserSettings::UserSetting::*>::cohtml::FieldProperty<r::InplaceSt
    Replacement::draft(0x140f46ce0, "cohtml_140f46ce0", tpl_1676::<0x142646d58, 0x14309f708, 0x14330e9a8, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::InplaceString<32> coregame::UIUserSettings::PossibleValue::*>::cohtml::FieldProperty<r::Inplace
    Replacement::draft(0x140f46d40, "cohtml_140f46d40", fn_140f46d40 as *const ()), // cohtml::FieldProperty<r::InplaceString<32> coregame::UIUserSettings::UserSettingsGroup::*>::cohtml::FieldProperty<r::Inp
    Replacement::draft(0x140f46db0, "cohtml_140f46db0", tpl_1676::<0x142646d58, 0x14309f708, 0x14330ef80, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<coregame::UIUserSettings::UserSettingValue coregame::UIUserSettings::PossibleValue::*>::cohtml::Fi
    Replacement::draft(0x140f46e10, "cohtml_140f46e10", tpl_61379::<0x78, 0x70, 0x28, 0x141df4e50, 0x48, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<coregame::UIUserSettings::UserSettingValue coregame::UIUserSettings::UserSetting::*>::cohtml::Fiel
    Replacement::draft(0x140f46ef0, "cohtml_140f46ef0", tpl_1676::<0x142646d58, 0x14309f708, 0x14330ecd8, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<bool coregame::UIUserSettings::UserSettingValue::*>::cohtml::FieldProperty<bool coregame::UIUserSe
    Replacement::draft(0x140f46f50, "cohtml_140f46f50", tpl_1676::<0x142646d58, 0x14309f708, 0x14330eef8, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<double coregame::UIUserSettings::UserSettingValue::*>::cohtml::FieldProperty<double coregame::UIUs
    Replacement::draft(0x140f46fb0, "cohtml_140f46fb0", tpl_1757::<0x18, 0x1c0> as *const ()), // cohtml::FieldProperty<double coregame::JSBoxedValue::*>::cohtml::FieldProperty<double coregame::JSBoxedValue::*>::vfunc_
    Replacement::draft(0x140f46fe0, "cohtml_140f46fe0", tpl_1758::<0x8, 0x230, 0x18, 0x1c0> as *const ()), // cohtml::FieldProperty<double coregame::JSBoxedValue::*>::cohtml::FieldProperty<double coregame::JSBoxedValue::*>::vfunc_
    Replacement::draft(0x140f47040, "cohtml_140f47040", fn_140f47040 as *const ()), // cohtml::FieldProperty<double coregame::JSBoxedValue::*>::cohtml::FieldProperty<double coregame::JSBoxedValue::*>::vfunc_
    Replacement::draft(0x140f47070, "cohtml_140f47070", fn_140f47070 as *const ()), // cohtml::FieldProperty<double coregame::JSBoxedValue::*>::cohtml::FieldProperty<double coregame::JSBoxedValue::*>::vfunc_
    Replacement::draft(0x140f470d0, "cohtml_140f470d0", fn_140f470d0 as *const ()), // cohtml::FieldProperty<double coregame::JSBoxedValue::*>::cohtml::FieldProperty<double coregame::JSBoxedValue::*>::vfunc_
    Replacement::draft(0x140f47110, "cohtml_140f47110", fn_140f47110 as *const ()), // cohtml::FieldProperty<double coregame::JSBoxedValue::*>::cohtml::FieldProperty<double coregame::JSBoxedValue::*>::vfunc_
    Replacement::draft(0x140f47140, "cohtml_140f47140", fn_140f47140 as *const ()), // cohtml::FieldProperty<double coregame::JSBoxedValue::*>::cohtml::FieldProperty<double coregame::JSBoxedValue::*>::vfunc_
    Replacement::draft(0x140f47180, "cohtml_140f47180", fn_140f47180 as *const ()), // sub_140f47180
    Replacement::draft(0x140f47270, "cohtml_140f47270", fn_140f47270 as *const ()), // sub_140f47270
    Replacement::draft(0x140f47490, "cohtml_140f47490", fn_140f47490 as *const ()), // sub_140f47490
    Replacement::draft(0x140f475d0, "cohtml_140f475d0", fn_140f475d0 as *const ()), // sub_140f475d0
    Replacement::draft(0x140f47690, "cohtml_140f47690", fn_140f47690 as *const ()), // sub_140f47690
    Replacement::draft(0x140f477f0, "cohtml_140f477f0", fn_140f477f0 as *const ()), // sub_140f477f0
    Replacement::draft(0x140f478a0, "cohtml_140f478a0", fn_140f478a0 as *const ()), // sub_140f478a0
    Replacement::draft(0x140f47980, "cohtml_140f47980", fn_140f47980 as *const ()), // sub_140f47980
    Replacement::draft(0x140f47ac0, "cohtml_140f47ac0", fn_140f47ac0 as *const ()), // sub_140f47ac0
    Replacement::draft(0x140f47b80, "cohtml_140f47b80", fn_140f47b80 as *const ()), // sub_140f47b80
    Replacement::draft(0x140f47d70, "cohtml_140f47d70", tpl_1026::<0x1401ea690, 0x8, 0xc> as *const ()), // sub_140f47d70
    Replacement::draft(0x140f47dc0, "cohtml_140f47dc0", fn_140f47dc0 as *const ()), // sub_140f47dc0
    Replacement::draft(0x140f47e70, "cohtml_140f47e70", fn_140f47e70 as *const ()), // sub_140f47e70
    Replacement::draft(0x140f47fb0, "cohtml_140f47fb0", fn_140f47fb0 as *const ()), // sub_140f47fb0
    Replacement::draft(0x140f48060, "cohtml_140f48060", fn_140f48060 as *const ()), // sub_140f48060
    Replacement::draft(0x140f48140, "cohtml_140f48140", fn_140f48140 as *const ()), // sub_140f48140
    Replacement::draft(0x140f48350, "cohtml_140f48350", tpl_1689::<0x28, 0x141df4980, 0x48> as *const ()), // sub_140f48350
    Replacement::draft(0x140f48390, "cohtml_140f48390", fn_140f48390 as *const ()), // sub_140f48390
    Replacement::draft(0x140f48a90, "cohtml_140f48a90", fn_140f48a90 as *const ()), // sub_140f48a90
    Replacement::draft(0x140f48cb0, "cohtml_140f48cb0", fn_140f48cb0 as *const ()), // sub_140f48cb0
    Replacement::draft(0x140f49100, "cohtml_140f49100", fn_140f49100 as *const ()), // sub_140f49100
    Replacement::draft(0x140f49160, "cohtml_140f49160", fn_140f49160 as *const ()), // sub_140f49160
    Replacement::draft(0x140f491a0, "cohtml_140f491a0", fn_140f491a0 as *const ()), // sub_140f491a0
    Replacement::draft(0x140f49200, "cohtml_140f49200", fn_140f49200 as *const ()), // sub_140f49200
    Replacement::draft(0x140f49240, "cohtml_140f49240", fn_140f49240 as *const ()), // sub_140f49240
    Replacement::draft(0x140f492a0, "cohtml_140f492a0", fn_140f492a0 as *const ()), // sub_140f492a0
    Replacement::draft(0x140f492b0, "cohtml_140f492b0", fn_140f492b0 as *const ()), // sub_140f492b0
    Replacement::draft(0x140f49310, "cohtml_140f49310", fn_140f49310 as *const ()), // sub_140f49310
    Replacement::draft(0x140f49350, "cohtml_140f49350", fn_140f49350 as *const ()), // sub_140f49350
    Replacement::draft(0x140f495b0, "cohtml_140f495b0", fn_140f495b0 as *const ()), // sub_140f495b0
    Replacement::draft(0x140f497e0, "cohtml_140f497e0", fn_140f497e0 as *const ()), // sub_140f497e0
    Replacement::draft(0x140f49860, "cohtml_140f49860", fn_140f49860 as *const ()), // sub_140f49860
    Replacement::draft(0x140f49aa0, "cohtml_140f49aa0", fn_140f49aa0 as *const ()), // sub_140f49aa0
    Replacement::draft(0x140f49bf0, "cohtml_140f49bf0", fn_140f49bf0 as *const ()), // sub_140f49bf0
    Replacement::draft(0x140f49c50, "cohtml_140f49c50", fn_140f49c50 as *const ()), // sub_140f49c50
    Replacement::draft(0x140f4a300, "cohtml_140f4a300", fn_140f4a300 as *const ()), // sub_140f4a300
    Replacement::draft(0x140f4a510, "cohtml_140f4a510", tpl_19036 as *const ()), // sub_140f4a510
    Replacement::draft(0x140f4a520, "cohtml_140f4a520", fn_140f4a520 as *const ()), // sub_140f4a520
    Replacement::draft(0x140f4a530, "cohtml_140f4a530", tpl_19038 as *const ()), // sub_140f4a530
    Replacement::draft(0x140f4a540, "cohtml_140f4a540", fn_140f4a540 as *const ()), // sub_140f4a540
    Replacement::draft(0x140f4a550, "cohtml_140f4a550", fn_140f4a550 as *const ()), // sub_140f4a550
    Replacement::draft(0x140f4a640, "cohtml_140f4a640", fn_140f4a640 as *const ()), // sub_140f4a640
    Replacement::draft(0x140f4a690, "cohtml_140f4a690", fn_140f4a690 as *const ()), // sub_140f4a690
    Replacement::draft(0x140f4a6d0, "cohtml_140f4a6d0", fn_140f4a6d0 as *const ()), // sub_140f4a6d0
    Replacement::draft(0x140f4a6f0, "cohtml_140f4a6f0", fn_140f4a6f0 as *const ()), // sub_140f4a6f0
    Replacement::draft(0x140f4a710, "cohtml_140f4a710", fn_140f4a710 as *const ()), // sub_140f4a710
    Replacement::draft_throws(0x140f4a720, "cohtml_140f4a720", tpl_13418::<0x140f4af00> as *const (), Throws::Discard), // sub_140f4a720
    Replacement::draft(0x140f4a730, "cohtml_140f4a730", fn_140f4a730 as *const ()), // sub_140f4a730
    Replacement::draft(0x140f4a8f0, "cohtml_140f4a8f0", tpl_1676::<0x142646d58, 0x14309f708, 0x14330f490, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UISubtitles::SubtitleUIData,r::Allocator<coregame::UISubtitles::Subtitle
    Replacement::draft(0x140f4a950, "cohtml_140f4a950", tpl_1677::<0x18, 0x140f4b1f0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UISubtitles::SubtitleUIData,r::Allocator<coregame::UISubtitles::Subtitle
    Replacement::draft(0x140f4a980, "cohtml_140f4a980", tpl_1678::<0x8, 0x230, 0x18, 0x140f4b1f0> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UISubtitles::SubtitleUIData,r::Allocator<coregame::UISubtitles::Subtitle
    Replacement::draft(0x140f4a9d0, "cohtml_140f4a9d0", tpl_1677::<0x18, 0x140f4b330> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UISubtitles::SubtitleUIData,r::Allocator<coregame::UISubtitles::Subtitle
    Replacement::draft(0x140f4aa00, "cohtml_140f4aa00", tpl_1679::<0x8, 0x90, 0x18, 0x140f4b330> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UISubtitles::SubtitleUIData,r::Allocator<coregame::UISubtitles::Subtitle
    Replacement::draft(0x140f4aa50, "cohtml_140f4aa50", tpl_61376::<0x78, 0x70, 0x140f4b330, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UISubtitles::SubtitleUIData,r::Allocator<coregame::UISubtitles::Subtitle
    Replacement::draft(0x140f4ab10, "cohtml_140f4ab10", tpl_61377::<0x140264460, 0x140f4b8d0, 0x141df7040> as *const ()), // cohtml::FieldProperty<r::DynamicArray<coregame::UISubtitles::SubtitleUIData,r::Allocator<coregame::UISubtitles::Subtitle
    Replacement::draft(0x140f4ac10, "cohtml_140f4ac10", tpl_1676::<0x142646d58, 0x14309f708, 0x14330f408, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::InplaceString<32> coregame::UISubtitles::SubtitleUIData::*>::cohtml::FieldProperty<r::InplaceSt
    Replacement::draft(0x140f4ac70, "cohtml_140f4ac70", tpl_1676::<0x142646d58, 0x14309f708, 0x14330f5a0, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::InplaceWString<64> coregame::UISubtitles::SubtitleUIData::*>::cohtml::FieldProperty<r::InplaceW
    Replacement::draft(0x140f4acd0, "cohtml_140f4acd0", tpl_1677::<0x18, 0x140f4b3f0> as *const ()), // cohtml::FieldProperty<r::InplaceWString<64> coregame::UISubtitles::SubtitleUIData::*>::cohtml::FieldProperty<r::InplaceW
    Replacement::draft(0x140f4ad00, "cohtml_140f4ad00", tpl_1678::<0x8, 0x230, 0x18, 0x140f4b3f0> as *const ()), // cohtml::FieldProperty<r::InplaceWString<64> coregame::UISubtitles::SubtitleUIData::*>::cohtml::FieldProperty<r::InplaceW
    Replacement::draft(0x140f4ad50, "cohtml_140f4ad50", fn_140f4ad50 as *const ()), // cohtml::FieldProperty<r::InplaceWString<64> coregame::UISubtitles::SubtitleUIData::*>::cohtml::FieldProperty<r::InplaceW
    Replacement::draft(0x140f4ad80, "cohtml_140f4ad80", fn_140f4ad80 as *const ()), // cohtml::FieldProperty<r::InplaceWString<64> coregame::UISubtitles::SubtitleUIData::*>::cohtml::FieldProperty<r::InplaceW
    Replacement::draft(0x140f4add0, "cohtml_140f4add0", fn_140f4add0 as *const ()), // cohtml::FieldProperty<r::InplaceWString<64> coregame::UISubtitles::SubtitleUIData::*>::cohtml::FieldProperty<r::InplaceW
    Replacement::draft(0x140f4aea0, "cohtml_140f4aea0", tpl_1676::<0x142646d58, 0x14309f708, 0x14330f518, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<unsigned int coregame::UISubtitles::SubtitleUIData::*>::cohtml::FieldProperty<unsigned int coregam
    Replacement::draft_throws(0x140f4af00, "cohtml_140f4af00", fn_140f4af00 as *const (), Throws::Discard), // sub_140f4af00
    Replacement::draft(0x140f4af70, "cohtml_140f4af70", fn_140f4af70 as *const ()), // sub_140f4af70
    Replacement::draft(0x140f4b170, "cohtml_140f4b170", fn_140f4b170 as *const ()), // sub_140f4b170
    Replacement::draft(0x140f4b1f0, "cohtml_140f4b1f0", fn_140f4b1f0 as *const ()), // sub_140f4b1f0
    Replacement::draft(0x140f4b330, "cohtml_140f4b330", fn_140f4b330 as *const ()), // sub_140f4b330
    Replacement::draft(0x140f4b3f0, "cohtml_140f4b3f0", fn_140f4b3f0 as *const ()), // sub_140f4b3f0
    Replacement::draft(0x140f4b580, "cohtml_140f4b580", fn_140f4b580 as *const ()), // sub_140f4b580
    Replacement::draft(0x140f4b640, "unknown_140f4b640", fn_140f4b640 as *const ()), // sub_140f4b640
    Replacement::draft(0x140f4b6f0, "unknown_140f4b6f0", fn_140f4b6f0 as *const ()), // sub_140f4b6f0
    Replacement::draft(0x140f4b760, "unknown_140f4b760", tpl_1689::<0x28, 0x141df6fd0, 0x48> as *const ()), // sub_140f4b760
    Replacement::draft(0x140f4b7a0, "unknown_140f4b7a0", fn_140f4b7a0 as *const ()), // sub_140f4b7a0
    Replacement::draft(0x140f4b800, "unknown_140f4b800", fn_140f4b800 as *const ()), // sub_140f4b800
    Replacement::draft(0x140f4b840, "unknown_140f4b840", fn_140f4b840 as *const ()), // sub_140f4b840
    Replacement::draft(0x140f4b8d0, "unknown_140f4b8d0", fn_140f4b8d0 as *const ()), // sub_140f4b8d0
    Replacement::draft(0x140f4b8e0, "unknown_140f4b8e0", fn_140f4b8e0 as *const ()), // sub_140f4b8e0
    Replacement::draft(0x140f4b950, "unknown_140f4b950", fn_140f4b950 as *const ()), // sub_140f4b950
    Replacement::draft_throws(0x140f4b9f0, "unknown_140f4b9f0", fn_140f4b9f0 as *const (), Throws::Caught), // sub_140f4b9f0
    Replacement::draft_throws(0x140f4be00, "detail_140f4be00", fn_140f4be00 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<detail::stateStacksToFactDictionarySystem>
    Replacement::draft_throws(0x140f4c090, "detail_140f4c090", fn_140f4c090 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<detail::broadcastLuaStackEventsSystem>
    Replacement::draft(0x140f4c320, "unknown_140f4c320", tpl_1094::<0x143c8cecc, 0x140f4c710, 0x14020e8f0> as *const ()), // sub_140f4c320
    Replacement::draft(0x140f4c520, "unknown_140f4c520", fn_140f4c520 as *const ()), // sub_140f4c520
    Replacement::draft(0x140f4c710, "unknown_140f4c710", fn_140f4c710 as *const ()), // sub_140f4c710
    Replacement::draft_throws(0x140f4c890, "unknown_140f4c890", tpl_6367::<0x143c8ceb4, 0x20, 0x8, 0x141df8580> as *const (), Throws::Discard), // sub_140f4c890
    Replacement::draft(0x140f4c8f0, "unknown_140f4c8f0", tpl_1217::<0x143c8ced4, 0x20, 0x8, 0x141df80d0> as *const ()), // sub_140f4c8f0
    Replacement::draft(0x140f4c950, "unknown_140f4c950", tpl_1955::<0x140, 0x1401ed490> as *const ()), // sub_140f4c950
    Replacement::draft(0x140f4c980, "unknown_140f4c980", fn_140f4c980 as *const ()), // sub_140f4c980
    Replacement::draft_throws(0x140f4d380, "unknown_140f4d380", fn_140f4d380 as *const (), Throws::Discard), // sub_140f4d380
    Replacement::draft_throws(0x140f4d700, "anonymous_namespace_140f4d700", fn_140f4d700 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::simpleNavigationSystem>
    Replacement::draft_throws(0x140f4d9c0, "anonymous_namespace_140f4d9c0", fn_140f4d9c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::activeCellCleanupSystem>
    Replacement::draft(0x140f4dc60, "anonymous_namespace_140f4dc60", fn_140f4dc60 as *const ()), // sub_140f4dc60
    Replacement::draft(0x140f4dd70, "anonymous_namespace_140f4dd70", fn_140f4dd70 as *const ()), // sub_140f4dd70
    Replacement::draft(0x140f4ddd0, "anonymous_namespace_140f4ddd0", fn_140f4ddd0 as *const ()), // sub_140f4ddd0
    Replacement::draft(0x140f4dfb0, "anonymous_namespace_140f4dfb0", tpl_1094::<0x143c90338, 0x140f4e5c0, 0x14020e8f0> as *const ()), // sub_140f4dfb0
    Replacement::draft(0x140f4e1b0, "anonymous_namespace_140f4e1b0", tpl_1094::<0x143c90330, 0x140f4e750, 0x14020e8f0> as *const ()), // sub_140f4e1b0
    Replacement::draft(0x140f4e3b0, "anonymous_namespace_140f4e3b0", fn_140f4e3b0 as *const ()), // sub_140f4e3b0
    Replacement::draft(0x140f4e5c0, "anonymous_namespace_140f4e5c0", fn_140f4e5c0 as *const ()), // sub_140f4e5c0
    Replacement::draft(0x140f4e750, "anonymous_namespace_140f4e750", fn_140f4e750 as *const ()), // sub_140f4e750
    Replacement::draft_throws(0x140f4e8d0, "anonymous_namespace_140f4e8d0", tpl_19074::<0x143c90804, 0x143c8ceb4, 0x143c90338, 0x142489ee0, 0x140f4ea40> as *const (), Throws::Discard), // sub_140f4e8d0
    Replacement::draft_throws(0x140f4ea40, "anonymous_namespace_140f4ea40", tpl_19075::<0x18, 0x20, 0x8, 0x30, 0x1402515f0, 0x140f4c9d0> as *const (), Throws::Discard), // sub_140f4ea40
    Replacement::draft_throws(0x140f4ecd0, "anonymous_namespace_140f4ecd0", fn_140f4ecd0 as *const (), Throws::Discard), // sub_140f4ecd0
    Replacement::draft_throws(0x140f4ee00, "anonymous_namespace_140f4ee00", tpl_1331::<0x8, 0x140258bd0, 0x143c905d4, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_140f4ee00
    Replacement::draft(0x140f4eeb0, "anonymous_namespace_140f4eeb0", tpl_3658::<0x3, 0x6c, 0x28, 0x14240ab10> as *const ()), // sub_140f4eeb0
    Replacement::draft(0x140f4ef00, "anonymous_namespace_140f4ef00", tpl_4996::<0x3c, 0x14240ab10> as *const ()), // sub_140f4ef00
    Replacement::draft_throws(0x140f4f120, "anonymous_namespace_140f4f120", fn_140f4f120 as *const (), Throws::Discard), // sub_140f4f120
    Replacement::draft(0x140f4f430, "anonymous_namespace_140f4f430", fn_140f4f430 as *const ()), // sub_140f4f430
    Replacement::draft(0x140f4f4f0, "anonymous_namespace_140f4f4f0", fn_140f4f4f0 as *const ()), // sub_140f4f4f0
    Replacement::draft(0x140f4f5b0, "anonymous_namespace_140f4f5b0", fn_140f4f5b0 as *const ()), // sub_140f4f5b0
    Replacement::draft_throws(0x140f4f5c0, "anonymous_namespace_140f4f5c0", tpl_3734::<0x8, 0x140f51db0, 0x14241c190> as *const (), Throws::Discard), // sub_140f4f5c0
    Replacement::draft_throws(0x140f4f5f0, "anonymous_namespace_140f4f5f0", tpl_3734::<0x8, 0x140f51e50, 0x14241c190> as *const (), Throws::Discard), // sub_140f4f5f0
    Replacement::draft_throws(0x140f4f620, "anonymous_namespace_140f4f620", fn_140f4f620 as *const (), Throws::Discard), // sub_140f4f620
    Replacement::draft(0x140f4f8a0, "anonymous_namespace_140f4f8a0", tpl_64921::<0x10, 0x18, 0x143c90324, 0x1806c, 0x18058, 0x142489ee0, 0x18068, 0x143cc5d90, 0x18050, 0x20, 0x38> as *const ()), // sub_140f4f8a0
    Replacement::draft_throws(0x140f4fa10, "anonymous_namespace_140f4fa10", fn_140f4fa10 as *const (), Throws::Caught), // sub_140f4fa10
    Replacement::draft(0x140f503c0, "anonymous_namespace_140f503c0", tpl_1171::<0x58, 0x60, 0x14384e7d8, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140f503c0
    Replacement::draft(0x140f504a0, "anonymous_namespace_140f504a0", tpl_1171::<0x58, 0x60, 0x143cc5cb4, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140f504a0
    Replacement::draft(0x140f50580, "anonymous_namespace_140f50580", tpl_1171::<0x58, 0x60, 0x143c90324, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140f50580
    Replacement::draft_throws(0x140f50660, "anonymous_namespace_140f50660", fn_140f50660 as *const (), Throws::Discard), // sub_140f50660
    Replacement::draft_throws(0x140f509e0, "anonymous_namespace_140f509e0", tpl_6402::<0x143cc5e0c, 0x143cc5e48, 0x140f51bb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433114f6, 0x142646960, 0x143cc5e68, 0x143cc5e78, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::detail::gridNavigationSystem>
    Replacement::draft_throws(0x140f50bf0, "anonymous_namespace_140f50bf0", tpl_5011::<0x143cc5d5c, 0x143cc5d00, 0x1411e9f50, 0x140f53b40, 0x143cc5d40, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140f52d30, 0x140f525b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143310d66, 0x142646960, 0x143cc5d20, 0x143cc5d30, 0x143cc5d38, 0x143cc5d3c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::detail::destroyPendingEntitiesFromGrid>
    Replacement::draft_throws(0x140f50ee0, "anonymous_namespace_140f50ee0", tpl_3060::<0x143cc5cc0, 0x143cc5e10, 0x1411e9f50, 0x140f52620, 0x140f52df0, 0x140f528d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143310f16, 0x142646960, 0x143cc5e30, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::detail::applyGridChanges>
    Replacement::draft(0x140f51120, "unknown_140f51120", fn_140f51120 as *const ()), // sub_140f51120
    Replacement::draft(0x140f51300, "unknown_140f51300", tpl_1094::<0x143c90324, 0x140f51ed0, 0x14020e8f0> as *const ()), // sub_140f51300
    Replacement::draft(0x140f51500, "unknown_140f51500", tpl_1095::<0x143cc5d90, 0x14020e8f0> as *const ()), // sub_140f51500
    Replacement::draft_throws(0x140f516e0, "unknown_140f516e0", fn_140f516e0 as *const (), Throws::Discard), // sub_140f516e0
    Replacement::draft(0x140f519b0, "unknown_140f519b0", tpl_1094::<0x143c90318, 0x140f521c0, 0x14020e8f0> as *const ()), // sub_140f519b0
    Replacement::draft_throws(0x140f51bb0, "unknown_140f51bb0", tpl_5182::<0x1411e9f50, 0x140f539a0, 0x143cc5e08, 0x1420af120, 0x140f53a50, 0x143cc5d94, 0x140215750, 0x14024e190, 0x140f4ecd0, 0x1420a28e0, 0x140f52b40, 0x140f52340> as *const (), Throws::Discard), // sub_140f51bb0
    Replacement::draft_throws(0x140f51db0, "unknown_140f51db0", fn_140f51db0 as *const (), Throws::Discard), // sub_140f51db0
    Replacement::draft_throws(0x140f51e50, "unknown_140f51e50", tpl_19083::<0x19, 0x10, 0x140f51e50, 0x1c, 0x14240ab10, 0x14241c190> as *const (), Throws::Discard), // sub_140f51e50
    Replacement::draft(0x140f51ed0, "unknown_140f51ed0", fn_140f51ed0 as *const ()), // sub_140f51ed0
    Replacement::draft_throws(0x140f52060, "unknown_140f52060", fn_140f52060 as *const (), Throws::Discard), // sub_140f52060
    Replacement::draft(0x140f521c0, "unknown_140f521c0", fn_140f521c0 as *const ()), // sub_140f521c0
    Replacement::draft_throws(0x140f52340, "unknown_140f52340", tpl_19074::<0x143cc5d94, 0x143c8ceb4, 0x143c90318, 0x142489ee0, 0x140f524b0> as *const (), Throws::Discard), // sub_140f52340
    Replacement::draft_throws(0x140f524b0, "unknown_140f524b0", tpl_19075::<0x18, 0x20, 0x8, 0x30, 0x140f4f5b0, 0x140f4fa10> as *const (), Throws::Discard), // sub_140f524b0
    Replacement::draft(0x140f525b0, "unknown_140f525b0", fn_140f525b0 as *const ()), // sub_140f525b0
    Replacement::draft_throws(0x140f52620, "unknown_140f52620", fn_140f52620 as *const (), Throws::Discard), // sub_140f52620
    Replacement::draft_throws(0x140f528d0, "unknown_140f528d0", fn_140f528d0 as *const (), Throws::Discard), // sub_140f528d0
    Replacement::draft(0x140f52a30, "unknown_140f52a30", tpl_19090::<0x8, 0x20, 0x143cc5e08, 0x143cc5e40, 0x143cc5cb0> as *const ()), // sub_140f52a30
    Replacement::draft(0x140f52b10, "unknown_140f52b10", tpl_4996::<0x48, 0x14240ab10> as *const ()), // sub_140f52b10
    Replacement::draft_throws(0x140f52b40, "unknown_140f52b40", fn_140f52b40 as *const (), Throws::Discard), // sub_140f52b40
    Replacement::draft(0x140f52d30, "unknown_140f52d30", fn_140f52d30 as *const ()), // sub_140f52d30
    Replacement::draft_throws(0x140f52df0, "unknown_140f52df0", fn_140f52df0 as *const (), Throws::Discard), // sub_140f52df0
    Replacement::draft(0x140f53530, "unknown_140f53530", fn_140f53530 as *const ()), // sub_140f53530
    Replacement::draft(0x140f53d00, "unknown_140f53d00", fn_140f53d00 as *const ()), // sub_140f53d00
    Replacement::draft(0x140f53f00, "unknown_140f53f00", fn_140f53f00 as *const ()), // sub_140f53f00
    Replacement::draft(0x140f541b0, "unknown_140f541b0", fn_140f541b0 as *const ()), // sub_140f541b0
    Replacement::draft(0x140f54ac0, "unknown_140f54ac0", fn_140f54ac0 as *const ()), // sub_140f54ac0
    Replacement::draft(0x140f54b10, "unknown_140f54b10", fn_140f54b10 as *const ()), // sub_140f54b10
    Replacement::draft(0x140f54ce0, "unknown_140f54ce0", fn_140f54ce0 as *const ()), // sub_140f54ce0
    Replacement::draft(0x140f54f90, "unknown_140f54f90", fn_140f54f90 as *const ()), // sub_140f54f90
    Replacement::draft(0x140f55790, "unknown_140f55790", fn_140f55790 as *const ()), // sub_140f55790
    Replacement::draft_throws(0x140f55de0, "anonymous_namespace_140f55de0", tpl_1578::<0x143cc5eac, 0x143cc5eb0, 0x1411e9f50, 0x140f56030, 0x140f561e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143311bb6, 0x142646960, 0x143cc5ed0, 0x143cc5ed4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::menuModeSystem>
    Replacement::draft_throws(0x140f56030, "unknown_140f56030", fn_140f56030 as *const (), Throws::Discard), // sub_140f56030
    Replacement::draft(0x140f561e0, "unknown_140f561e0", tpl_19100::<0x8, 0x20, 0x143cc5ef4, 0x143cc5ef0, 0x143c8ceb4, 0x141e009b0> as *const ()), // sub_140f561e0
    Replacement::draft(0x140f562c0, "unknown_140f562c0", tpl_19101::<0x143cc5ed8, 0x143cc5ee0, 0x8, 0x10, 0x18, 0x141834a78, 0x143c90318, 0x141834a0c> as *const ()), // sub_140f562c0
    Replacement::draft(0x140f56370, "unknown_140f56370", tpl_19101::<0x143c8a484, 0x143c8a488, 0x8, 0x10, 0x18, 0x141834a78, 0x143c90338, 0x141834a0c> as *const ()), // sub_140f56370
    Replacement::draft_throws(0x140f56420, "unknown_140f56420", tpl_4541::<0x143cc5ef0, 0x140f562c0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140f56420
    Replacement::draft_throws(0x140f564b0, "unknown_140f564b0", tpl_4541::<0x143cc5ef4, 0x140f56370, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_140f564b0
    Replacement::draft(0x140f56540, "unknown_140f56540", fn_140f56540 as *const ()), // sub_140f56540
    Replacement::draft(0x140f56580, "unknown_140f56580", fn_140f56580 as *const ()), // sub_140f56580
    Replacement::draft(0x140f56610, "unknown_140f56610", tpl_2030::<0x140f56580, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f56610
    Replacement::draft(0x140f566c0, "unknown_140f566c0", tpl_61235::<0x1401e7c10, 0x143c90338, 0x142489ee0, 0x1426d01e0> as *const ()), // sub_140f566c0
    Replacement::draft(0x140f56980, "unknown_140f56980", fn_140f56980 as *const ()), // sub_140f56980
    Replacement::draft(0x140f56c40, "unknown_140f56c40", fn_140f56c40 as *const ()), // sub_140f56c40
    Replacement::draft(0x140f56d80, "cohtml_140f56d80", tpl_1676::<0x142646d58, 0x14309f708, 0x143311fe0, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<ui::PageResource::NavigationLayout coregame::UIStateModuleBindings::NavigationSetCellArguments::*>
    Replacement::draft(0x140f56de0, "cohtml_140f56de0", fn_140f56de0 as *const ()), // cohtml::FieldProperty<ui::PageResource::BindingType coregame::JSBoxedValue::*>::cohtml::FieldProperty<ui::PageResource::
    Replacement::draft(0x140f56e30, "cohtml_140f56e30", fn_140f56e30 as *const ()), // cohtml::FieldProperty<ui::PageResource::BindingType coregame::JSBoxedValue::*>::cohtml::FieldProperty<ui::PageResource::
    Replacement::draft(0x140f56ea0, "cohtml_140f56ea0", tpl_19001::<0x18, 0x110> as *const ()), // cohtml::FieldProperty<int coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::FieldProperty<int cor
    Replacement::draft(0x140f56ed0, "cohtml_140f56ed0", tpl_19002::<0x8, 0x90, 0x18, 0x110> as *const ()), // cohtml::FieldProperty<int coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::FieldProperty<int cor
    Replacement::draft(0x140f56f20, "cohtml_140f56f20", tpl_1676::<0x142646d58, 0x14309f708, 0x143312068, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<bool coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::FieldProperty<bool c
    Replacement::draft(0x140f56f80, "cohtml_140f56f80", tpl_1676::<0x142646d58, 0x14309f708, 0x1433120f0, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<int coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::FieldProperty<int cor
    Replacement::draft(0x140f56fe0, "cohtml_140f56fe0", tpl_1757::<0x18, 0x210> as *const ()), // cohtml::FieldProperty<int coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::FieldProperty<int cor
    Replacement::draft(0x140f57010, "cohtml_140f57010", tpl_1758::<0x8, 0x230, 0x18, 0x210> as *const ()), // cohtml::FieldProperty<int coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::FieldProperty<int cor
    Replacement::draft(0x140f57070, "cohtml_140f57070", tpl_19003::<0x78, 0x142646d88> as *const ()), // cohtml::FieldProperty<int coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::FieldProperty<int cor
    Replacement::draft(0x140f570a0, "cohtml_140f570a0", fn_140f570a0 as *const ()), // cohtml::FieldProperty<int coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::FieldProperty<int cor
    Replacement::draft(0x140f570d0, "cohtml_140f570d0", tpl_1676::<0x142646d58, 0x14309f708, 0x143312178, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::InplaceString<32> coregame::UIStateModuleBindings::NavigationSetCellArguments::*>::cohtml::Fiel
    Replacement::draft(0x140f57130, "cohtml_140f57130", fn_140f57130 as *const ()), // sub_140f57130
    Replacement::draft(0x140f573d0, "cohtml_140f573d0", tpl_3819::<0x140f57560> as *const ()), // cohtml::EventHandler<void (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>,int)>::cohtml::EventH
    Replacement::draft(0x140f573e0, "cohtml_140f573e0", fn_140f573e0 as *const ()), // cohtml::EventHandler<void (coregame::UIStateModuleBindings::NavigationBindings::*)(const coregame::UIStateModuleBindings
    Replacement::draft(0x140f57490, "cohtml_140f57490", tpl_3819::<0x140f57790> as *const ()), // cohtml::EventHandler<void (coregame::UIStateModuleBindings::NavigationBindings::*)(r::InplaceString<32>,int,ui::PageReso
    Replacement::draft(0x140f574a0, "cohtml_140f574a0", tpl_3819::<0x140f579e0> as *const ()), // cohtml::EventHandler<int (coregame::UIStateModuleBindings::NavigationBindings::*)(r::InplaceString<32>,ui::PageResource:
    Replacement::draft(0x140f574b0, "cohtml_140f574b0", tpl_3819::<0x140f57c30> as *const ()), // cohtml::EventHandler<void (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>,bool)>::cohtml::Event
    Replacement::draft(0x140f574c0, "cohtml_140f574c0", tpl_3819::<0x140f57e60> as *const ()), // cohtml::EventHandler<void (coregame::UIStateModuleBindings::NavigationBindings::*)(r::InplaceString<32>,r::InplaceString
    Replacement::draft(0x140f574d0, "cohtml_140f574d0", tpl_3819::<0x140f582b0> as *const ()), // cohtml::EventHandler<void (coregame::UIStateModuleBindings::NavigationBindings::*)(r::InplaceString<32>,r::InplaceString
    Replacement::draft(0x140f574e0, "cohtml_140f574e0", tpl_3819::<0x140f58700> as *const ()), // cohtml::EventHandler<r::DynamicArray<r::InplaceString<32>,r::Allocator<r::InplaceString<32>,0>> (coregame::UIStateModule
    Replacement::draft(0x140f574f0, "cohtml_140f574f0", tpl_3819::<0x140f58930> as *const ()), // cohtml::EventHandler<r::InplaceString<32> (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>,int)>
    Replacement::draft(0x140f57500, "cohtml_140f57500", tpl_3819::<0x140f58b80> as *const ()), // cohtml::EventHandler<r::InplaceString<32> (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>)>::co
    Replacement::draft(0x140f57510, "cohtml_140f57510", tpl_3819::<0x140f58da0> as *const ()), // cohtml::EventHandler<bool (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>)>::cohtml::EventHandl
    Replacement::draft(0x140f57520, "cohtml_140f57520", tpl_3819::<0x140f58fd0> as *const ()), // cohtml::EventHandler<int (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>,r::InplaceString<32>)>
    Replacement::draft(0x140f57530, "cohtml_140f57530", tpl_3819::<0x140f593b0> as *const ()), // cohtml::EventHandler<bool (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>,r::InplaceString<32>)
    Replacement::draft(0x140f57540, "cohtml_140f57540", tpl_3819::<0x140f59790> as *const ()), // cohtml::EventHandler<void (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>,r::InplaceString<32>)
    Replacement::draft(0x140f57550, "cohtml_140f57550", tpl_3819::<0x140f59b40> as *const ()), // cohtml::EventHandler<void (coregame::UIStateModuleBindings::StackBindings::*)(r::InplaceString<32>)>::cohtml::EventHandl
    Replacement::draft(0x140f57560, "unknown_140f57560", fn_140f57560 as *const ()), // sub_140f57560
    Replacement::draft(0x140f57790, "unknown_140f57790", fn_140f57790 as *const ()), // sub_140f57790
    Replacement::draft(0x140f579e0, "unknown_140f579e0", fn_140f579e0 as *const ()), // sub_140f579e0
    Replacement::draft(0x140f57c30, "unknown_140f57c30", fn_140f57c30 as *const ()), // sub_140f57c30
    Replacement::draft(0x140f57e60, "unknown_140f57e60", fn_140f57e60 as *const ()), // sub_140f57e60
    Replacement::draft(0x140f582b0, "unknown_140f582b0", fn_140f582b0 as *const ()), // sub_140f582b0
    Replacement::draft(0x140f58700, "unknown_140f58700", fn_140f58700 as *const ()), // sub_140f58700
    Replacement::draft(0x140f58930, "unknown_140f58930", fn_140f58930 as *const ()), // sub_140f58930
    Replacement::draft(0x140f58b80, "unknown_140f58b80", fn_140f58b80 as *const ()), // sub_140f58b80
    Replacement::draft(0x140f58da0, "unknown_140f58da0", fn_140f58da0 as *const ()), // sub_140f58da0
    Replacement::draft(0x140f58fd0, "unknown_140f58fd0", fn_140f58fd0 as *const ()), // sub_140f58fd0
    Replacement::draft(0x140f593b0, "unknown_140f593b0", fn_140f593b0 as *const ()), // sub_140f593b0
    Replacement::draft(0x140f59790, "unknown_140f59790", fn_140f59790 as *const ()), // sub_140f59790
    Replacement::draft(0x140f59b40, "unknown_140f59b40", fn_140f59b40 as *const ()), // sub_140f59b40
    Replacement::draft(0x140f59d40, "unknown_140f59d40", fn_140f59d40 as *const ()), // sub_140f59d40
    Replacement::draft(0x140f59dd0, "unknown_140f59dd0", fn_140f59dd0 as *const ()), // sub_140f59dd0
    Replacement::draft(0x140f59e50, "unknown_140f59e50", fn_140f59e50 as *const ()), // sub_140f59e50
    Replacement::draft(0x140f59eb0, "unknown_140f59eb0", fn_140f59eb0 as *const ()), // sub_140f59eb0
    Replacement::draft(0x140f59f40, "unknown_140f59f40", fn_140f59f40 as *const ()), // sub_140f59f40
    Replacement::draft_throws(0x140f59ff0, "coregame_140f59ff0", fn_140f59ff0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_page::addPages>
    Replacement::draft(0x140f5a300, "coregame_140f5a300", fn_140f5a300 as *const ()), // ecs::system::ecs::system::wrapper<coregame::ui_page::cleanUp>
    Replacement::draft(0x140f5a5c0, "coregame_140f5a5c0", tpl_1094::<0x143cc5f7c, 0x140f5b260, 0x14020e8f0> as *const ()), // sub_140f5a5c0
    Replacement::draft(0x140f5a7c0, "coregame_140f5a7c0", tpl_1095::<0x143cc5f80, 0x14020e8f0> as *const ()), // sub_140f5a7c0
    Replacement::draft(0x140f5a9a0, "coregame_140f5a9a0", tpl_1436::<0x58, 0x60, 0x158, 0x143cc5f84, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140f5a9a0
    Replacement::draft(0x140f5aa80, "coregame_140f5aa80", tpl_1094::<0x143cc5f84, 0x140366c40, 0x14020e8f0> as *const ()), // sub_140f5aa80
    Replacement::draft_throws(0x140f5acb0, "coregame_140f5acb0", tpl_18900::<0x143cc5f88, 0x143cc5f90, 0x1411e9f50, 0x140f5bf60, 0x143cc611c, 0x1420af120, 0x1420a28e0, 0x140f5bae0, 0x140f5b450, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433129f6, 0x142646960, 0x143cc5fa0, 0x143cc5fa8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_page::streamIn>
    Replacement::draft_throws(0x140f5af80, "coregame_140f5af80", tpl_5126::<0x143cc5fac, 0x143cc5fb0, 0x1411e9f50, 0x140f5be70, 0x143cc6118, 0x1420af120, 0x1420a28e0, 0x140f5bbf0, 0x140f5b510, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143312866, 0x142646960, 0x143cc5fc0, 0x143cc5fc8, 0x143cc5fcc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_page::streamOut>
    Replacement::draft(0x140f5b260, "unknown_140f5b260", fn_140f5b260 as *const ()), // sub_140f5b260
    Replacement::draft(0x140f5b450, "unknown_140f5b450", tpl_1126::<0x142407c80, 0x580f0, 0x141e069e0> as *const ()), // sub_140f5b450
    Replacement::draft(0x140f5b510, "unknown_140f5b510", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140f5a9a0> as *const ()), // sub_140f5b510
    Replacement::draft_throws(0x140f5b5b0, "unknown_140f5b5b0", fn_140f5b5b0 as *const (), Throws::Caught), // sub_140f5b5b0
    Replacement::draft_throws(0x140f5b650, "unknown_140f5b650", fn_140f5b650 as *const (), Throws::Caught), // sub_140f5b650
    Replacement::draft(0x140f5b7f0, "unknown_140f5b7f0", tpl_3188::<0x140f5bcd0, 0x143313278, 0x140497a00, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140f5b7f0
    Replacement::draft(0x140f5bae0, "unknown_140f5bae0", tpl_1130::<0x142407c80, 0x141e069e0> as *const ()), // sub_140f5bae0
    Replacement::draft(0x140f5bbf0, "unknown_140f5bbf0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140f5a9a0> as *const ()), // sub_140f5bbf0
    Replacement::draft(0x140f5bcd0, "unknown_140f5bcd0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143313278, 0x8, 0x20> as *const ()), // sub_140f5bcd0
    Replacement::draft(0x140f5c120, "unknown_140f5c120", fn_140f5c120 as *const ()), // sub_140f5c120
    Replacement::draft(0x140f5c2f0, "unknown_140f5c2f0", fn_140f5c2f0 as *const ()), // sub_140f5c2f0
    Replacement::draft(0x140f5c3a0, "unknown_140f5c3a0", fn_140f5c3a0 as *const ()), // sub_140f5c3a0
    Replacement::draft(0x140f5c3f0, "unknown_140f5c3f0", fn_140f5c3f0 as *const ()), // sub_140f5c3f0
    Replacement::draft(0x140f5c470, "unknown_140f5c470", fn_140f5c470 as *const ()), // sub_140f5c470
    Replacement::draft(0x140f5c5f0, "unknown_140f5c5f0", fn_140f5c5f0 as *const ()), // sub_140f5c5f0
    Replacement::draft(0x140f5c7f0, "unknown_140f5c7f0", fn_140f5c7f0 as *const ()), // sub_140f5c7f0
    Replacement::draft_throws(0x140f5c860, "unknown_140f5c860", fn_140f5c860 as *const (), Throws::Discard), // sub_140f5c860
    Replacement::draft_throws(0x140f5c9d0, "unknown_140f5c9d0", tpl_1730::<0x1402174f0, 0x14021a880, 0x1424b16a0, 0x140267700, 0x1401f1a80> as *const (), Throws::Discard), // sub_140f5c9d0
    Replacement::draft_throws(0x140f5cb40, "unknown_140f5cb40", fn_140f5cb40 as *const (), Throws::Discard), // sub_140f5cb40
    Replacement::draft_throws(0x140f5cca0, "unknown_140f5cca0", fn_140f5cca0 as *const (), Throws::Discard), // sub_140f5cca0
    Replacement::draft(0x140f5ce00, "unknown_140f5ce00", tpl_1526::<0x1428acc90, 0x1410d6ca0, 0x140f5d0a0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140f5ce00
    Replacement::draft(0x140f5d0a0, "coregame_140f5d0a0", tpl_19142::<0x143cc61a8, 0x143cc61f0, 0x1411e9f50, 0x143c8ce84, 0x140215610, 0x143c8ce88, 0x1420a28e0, 0x140f5d470, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143313416, 0x142646960, 0x143cc6210, 0x143cc6220, 0x143cc6228, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::ui_input_binding::updateInputBindingEnvironment>
    Replacement::draft(0x140f5d390, "unknown_140f5d390", fn_140f5d390 as *const ()), // sub_140f5d390
    Replacement::draft_throws(0x140f5d470, "unknown_140f5d470", tpl_1704::<0x143c8ce88, 0x143c8ce84, 0x580e0, 0x141e09160> as *const (), Throws::Discard), // sub_140f5d470
    Replacement::draft(0x140f5d4a0, "unknown_140f5d4a0", fn_140f5d4a0 as *const ()), // sub_140f5d4a0
    Replacement::draft(0x140f5d4f0, "unknown_140f5d4f0", fn_140f5d4f0 as *const ()), // sub_140f5d4f0
    Replacement::draft(0x140f5d5e0, "unknown_140f5d5e0", fn_140f5d5e0 as *const ()), // sub_140f5d5e0
    Replacement::draft(0x140f5d780, "unknown_140f5d780", fn_140f5d780 as *const ()), // sub_140f5d780
    Replacement::draft(0x140f5da40, "unknown_140f5da40", fn_140f5da40 as *const ()), // sub_140f5da40
    Replacement::draft(0x140f5dc80, "unknown_140f5dc80", fn_140f5dc80 as *const ()), // sub_140f5dc80
    Replacement::draft(0x140f5dec0, "cohtml_140f5dec0", fn_140f5dec0 as *const ()), // cohtml::EventHandler<void (coregame::UIFactBinding::*)(double)>::cohtml::EventHandler<void (coregame::UIFactBinding::*)(
    Replacement::draft(0x140f5df20, "cohtml_140f5df20", fn_140f5df20 as *const ()), // cohtml::EventHandler<double (coregame::UIGameTimerModuleBindings::*)(double)>::cohtml::EventHandler<double (coregame::UI
    Replacement::draft_throws(0x140f5dfb0, "unknown_140f5dfb0", fn_140f5dfb0 as *const (), Throws::Discard), // sub_140f5dfb0
    Replacement::draft(0x140f5e110, "unknown_140f5e110", tpl_64921::<0x10, 0x18, 0x143c9032c, 0x1806c, 0x18058, 0x142489ee0, 0x18068, 0x143c9031c, 0x18050, 0x20, 0x38> as *const ()), // sub_140f5e110
    Replacement::draft_throws(0x140f5e1f0, "unknown_140f5e1f0", fn_140f5e1f0 as *const (), Throws::Caught), // sub_140f5e1f0
    Replacement::draft_throws(0x140f5e320, "anonymous_namespace_140f5e320", fn_140f5e320 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::updateUIGameTimers>
    Replacement::draft(0x140f5e6a0, "unknown_140f5e6a0", tpl_1369::<0x143c9032c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f5e6a0
    Replacement::draft(0x140f5e9e0, "unknown_140f5e9e0", tpl_1095::<0x14384e7dc, 0x14020e8f0> as *const ()), // sub_140f5e9e0
    Replacement::draft(0x140f5ebc0, "unknown_140f5ebc0", tpl_19153::<0x88, 0x10088, 0x141e0c0e0, 0x141e0c4d0> as *const ()), // sub_140f5ebc0
    Replacement::draft(0x140f5ec40, "unknown_140f5ec40", fn_140f5ec40 as *const ()), // sub_140f5ec40
    Replacement::draft(0x140f5eec0, "unknown_140f5eec0", tpl_19154::<0x1425371a0, 0x142533200, 0x140265ba0, 0x140246c40, 0x1401de830> as *const ()), // sub_140f5eec0
    Replacement::draft(0x140f5f1c0, "coregame_140f5f1c0", fn_140f5f1c0 as *const ()), // coregame::ui_events::detail::EventHandler::coregame::ui_events::detail::EventHandler::vfunc_1
    Replacement::draft(0x140f5fe20, "coregame_140f5fe20", fn_140f5fe20 as *const ()), // sub_140f5fe20
    Replacement::draft(0x140f600f0, "coregame_140f600f0", fn_140f600f0 as *const ()), // coregame::ui_events::detail::EventHandler::coregame::ui_events::detail::EventHandler::vfunc_0
    Replacement::draft(0x140f60170, "coregame_140f60170", tpl_19156::<0x3, 0x4, 0x8, 0x14240aa00, 0x28, 0x14053f5f0> as *const ()), // sub_140f60170
    Replacement::draft_throws(0x140f60210, "coregame_140f60210", fn_140f60210 as *const (), Throws::Discard), // sub_140f60210
    Replacement::draft(0x140f60310, "coregame_140f60310", fn_140f60310 as *const ()), // sub_140f60310
    Replacement::draft(0x140f603e0, "coregame_140f603e0", tpl_808::<0x140f60490, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f603e0
    Replacement::draft(0x140f60490, "coregame_140f60490", tpl_19159::<0x8, 0x28, 0x1401eedc0, 0x14240ab10> as *const ()), // sub_140f60490
    Replacement::draft(0x140f60500, "coregame_140f60500", fn_140f60500 as *const ()), // sub_140f60500
    Replacement::draft(0x140f60660, "coregame_140f60660", fn_140f60660 as *const ()), // sub_140f60660
    Replacement::draft(0x140f60880, "coregame_140f60880", fn_140f60880 as *const ()), // sub_140f60880
    Replacement::draft(0x140f608e0, "coregame_140f608e0", fn_140f608e0 as *const ()), // sub_140f608e0
    Replacement::draft(0x140f60a10, "coregame_140f60a10", fn_140f60a10 as *const ()), // sub_140f60a10
    Replacement::draft(0x140f60aa0, "coregame_140f60aa0", tpl_19165::<0x140f60a10> as *const ()), // sub_140f60aa0
    Replacement::draft(0x140f60b00, "coregame_140f60b00", tpl_2030::<0x140f60bb0, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140f60b00
    Replacement::draft(0x140f60bb0, "coregame_140f60bb0", fn_140f60bb0 as *const ()), // sub_140f60bb0
    Replacement::draft(0x140f60c40, "coregame_140f60c40", fn_140f60c40 as *const ()), // sub_140f60c40
    Replacement::draft(0x140f60c90, "coregame_140f60c90", fn_140f60c90 as *const ()), // ecs::system::ecs::system::wrapper<coregame::ui_events::system::cleanUpFixed>
    Replacement::draft(0x140f60f60, "coregame_140f60f60", fn_140f60f60 as *const ()), // ecs::system::ecs::system::wrapper<coregame::ui_events::system::handleLuaCallbacks>
    Replacement::draft(0x140f61220, "coregame_140f61220", tpl_6373::<0x143cc6430, 0x143cc6438, 0x1411e9f50, 0x143c8ce6c, 0x140215610, 0x143c8ce94, 0x1420a28e0, 0x140f62540, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143314526, 0x142646960, 0x143cc6458, 0x143cc6460, 0x143cc6464, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::ui_events::system::cleanUpVariable>
    Replacement::draft_throws(0x140f614f0, "coregame_140f614f0", tpl_3068::<0x143cc6468, 0x143cc6470, 0x1411e9f50, 0x140f62bf0, 0x143cc6638, 0x1420af120, 0x1420a28e0, 0x140f62570, 0x140f622c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433145f6, 0x142646960, 0x143cc6490, 0x143cc64a0, 0x143cc64a4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_events::system::handleUIPageRegistration>
    Replacement::draft(0x140f617e0, "coregame_140f617e0", fn_140f617e0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::ui_events::system::handleTextureViewPageRegistration>
    Replacement::draft_throws(0x140f61ae0, "coregame_140f61ae0", tpl_3274::<0x143cc64f0, 0x143cc64f8, 0x1411e9f50, 0x140f62b40, 0x143cc6634, 0x1420af120, 0x143c8ce6c, 0x140215610, 0x1420a28e0, 0x140f62700, 0x140f623d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433140c6, 0x142646960, 0x143cc6518, 0x143cc6528, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_events::system::collectUIPageEvents>
    Replacement::draft(0x140f61df0, "coregame_140f61df0", tpl_19142::<0x143cc652c, 0x143cc6530, 0x1411e9f50, 0x143c8ce6c, 0x140215610, 0x143c9c824, 0x1420a28e0, 0x140f62870, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143314226, 0x142646960, 0x143cc6550, 0x143cc6560, 0x143cc6568, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::ui_events::system::collectTextureViewPageEvents>
    Replacement::draft(0x140f620e0, "unknown_140f620e0", tpl_1095::<0x143cc656c, 0x14020e8f0> as *const ()), // sub_140f620e0
    Replacement::draft_throws(0x140f622c0, "unknown_140f622c0", fn_140f622c0 as *const (), Throws::Caught), // sub_140f622c0
    Replacement::draft_throws(0x140f623d0, "unknown_140f623d0", fn_140f623d0 as *const (), Throws::Discard), // sub_140f623d0
    Replacement::draft(0x140f624e0, "unknown_140f624e0", fn_140f624e0 as *const ()), // sub_140f624e0
    Replacement::draft(0x140f62540, "unknown_140f62540", fn_140f62540 as *const ()), // sub_140f62540
    Replacement::draft_throws(0x140f62570, "unknown_140f62570", fn_140f62570 as *const (), Throws::Caught), // sub_140f62570
    Replacement::draft_throws(0x140f62700, "unknown_140f62700", tpl_62839::<0x143c8ce6c, 0x143cc5f84, 0x142489ee0, 0x141e0dc50> as *const (), Throws::Discard), // sub_140f62700
    Replacement::draft(0x140f62990, "unknown_140f62990", fn_140f62990 as *const ()), // sub_140f62990
    Replacement::draft(0x140f62ce0, "unknown_140f62ce0", tpl_60956::<0x19, 0x10, 0x140f62ce0, 0x28, 0x140f60b00, 0x1401e66f0> as *const ()), // sub_140f62ce0
    Replacement::draft(0x140f62f30, "unknown_140f62f30", fn_140f62f30 as *const ()), // sub_140f62f30
    Replacement::draft(0x140f62ff0, "unknown_140f62ff0", tpl_1663::<0x8, 0x140f62ce0, 0x1401e66f0> as *const ()), // sub_140f62ff0
    Replacement::draft(0x140f63030, "unknown_140f63030", fn_140f63030 as *const ()), // sub_140f63030
    Replacement::draft(0x140f63250, "unknown_140f63250", fn_140f63250 as *const ()), // sub_140f63250
    Replacement::draft(0x140f632c0, "unknown_140f632c0", fn_140f632c0 as *const ()), // sub_140f632c0
    Replacement::draft(0x140f63330, "coregame_140f63330", tpl_19177::<0x140f632c0, 0x141834d30> as *const ()), // coregame::UIFactBinding::coregame::UIFactBinding::vfunc_0
    Replacement::draft(0x140f63370, "coregame_140f63370", fn_140f63370 as *const ()), // sub_140f63370
    Replacement::draft(0x140f633a0, "coregame_140f633a0", fn_140f633a0 as *const ()), // coregame::UIFactListBase::coregame::UIFactListBase::vfunc_0
    Replacement::draft(0x140f633f0, "cohtml_140f633f0", tpl_19180::<0x14240ab10, 0x14309f708> as *const ()), // sub_140f633f0
    Replacement::draft(0x140f63420, "coregame_140f63420", fn_140f63420 as *const ()), // sub_140f63420
    Replacement::draft(0x140f634f0, "coregame_140f634f0", tpl_19177::<0x140f63530, 0x141834d30> as *const ()), // coregame::UIFactList::coregame::UIFactList::vfunc_0
    Replacement::draft(0x140f63530, "coregame_140f63530", fn_140f63530 as *const ()), // sub_140f63530
    Replacement::draft(0x140f635a0, "coregame_140f635a0", fn_140f635a0 as *const ()), // coregame::UIObjectFactList::coregame::UIObjectFactList::vfunc_0
    Replacement::draft(0x140f63640, "coregame_140f63640", tpl_1149::<0x8, 0x140f65730, 0x1401e66f0> as *const ()), // sub_140f63640
    Replacement::draft(0x140f63680, "coregame_140f63680", fn_140f63680 as *const ()), // sub_140f63680
    Replacement::draft(0x140f636c0, "coregame_140f636c0", fn_140f636c0 as *const ()), // sub_140f636c0
    Replacement::draft(0x140f63720, "coregame_140f63720", fn_140f63720 as *const ()), // sub_140f63720
    Replacement::draft_throws(0x140f63800, "coregame_140f63800", fn_140f63800 as *const (), Throws::Discard), // sub_140f63800
    Replacement::draft(0x140f63900, "coregame_140f63900", fn_140f63900 as *const ()), // sub_140f63900
    Replacement::draft(0x140f63990, "coregame_140f63990", fn_140f63990 as *const ()), // sub_140f63990
    Replacement::draft(0x140f63a00, "coregame_140f63a00", fn_140f63a00 as *const ()), // sub_140f63a00
    Replacement::draft(0x140f63ac0, "coregame_140f63ac0", tpl_19186::<0x14240ab10, 0x14309f708, 0x142646d40, 0x1401de410> as *const ()), // coregame::UIBaseDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Templa
    Replacement::draft(0x140f63b20, "coregame_140f63b20", fn_140f63b20 as *const ()), // sub_140f63b20
    Replacement::draft(0x140f63ba0, "coregame_140f63ba0", fn_140f63ba0 as *const ()), // coregame::UIDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Template<f
    Replacement::draft(0x140f63c50, "coregame_140f63c50", tpl_1675::<0x48> as *const ()), // coregame::UIDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Template<f
    Replacement::draft_throws(0x140f63c60, "coregame_140f63c60", fn_140f63c60 as *const (), Throws::Discard), // coregame::UIBaseDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Templa
    Replacement::draft_throws(0x140f63dd0, "coregame_140f63dd0", fn_140f63dd0 as *const (), Throws::Discard), // coregame::UIBaseDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Templa
    Replacement::draft(0x140f63f30, "coregame_140f63f30", fn_140f63f30 as *const ()), // coregame::UIBaseDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Templa
    Replacement::draft(0x140f63f80, "cohtml_140f63f80", fn_140f63f80 as *const ()), // cohtml::ConcretePropertyReadOnly<const char* (coregame::UIFactBinding::*)(void)>::cohtml::ConcretePropertyReadOnly<const
    Replacement::draft_throws(0x140f63f90, "coregame_140f63f90", fn_140f63f90 as *const (), Throws::Caught), // coregame::UIBaseDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Templa
    Replacement::draft_throws(0x140f64030, "coregame_140f64030", fn_140f64030 as *const (), Throws::Caught), // coregame::UIBaseDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Templa
    Replacement::draft_throws(0x140f640d0, "unknown_140f640d0", fn_140f640d0 as *const (), Throws::Discard), // sub_140f640d0
    Replacement::draft_throws(0x140f64250, "std_140f64250", fn_140f64250 as *const (), Throws::Discard), // sub_140f64250
    Replacement::draft(0x140f643d0, "unknown_140f643d0", fn_140f643d0 as *const ()), // sub_140f643d0
    Replacement::draft(0x140f64610, "unknown_140f64610", fn_140f64610 as *const ()), // sub_140f64610
    Replacement::draft_throws(0x140f64800, "unknown_140f64800", fn_140f64800 as *const (), Throws::Caught), // sub_140f64800
    Replacement::draft(0x140f649d0, "unknown_140f649d0", fn_140f649d0 as *const ()), // sub_140f649d0
    Replacement::draft(0x140f64b50, "unknown_140f64b50", fn_140f64b50 as *const ()), // sub_140f64b50
    Replacement::draft_throws(0x140f64dd0, "unknown_140f64dd0", fn_140f64dd0 as *const (), Throws::Caught), // sub_140f64dd0
    Replacement::draft_throws(0x140f65400, "unknown_140f65400", fn_140f65400 as *const (), Throws::Caught), // sub_140f65400
    Replacement::draft_throws(0x140f65590, "unknown_140f65590", fn_140f65590 as *const (), Throws::Caught), // sub_140f65590
    Replacement::draft(0x140f65730, "unknown_140f65730", fn_140f65730 as *const ()), // sub_140f65730
    Replacement::draft(0x140f657c0, "cohtml_140f657c0", tpl_1676::<0x142646d58, 0x14309f708, 0x143315070, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactObject,r::Allocator<coregame::UIFactObject,0>> coregame::UIObjectFactLis
    Replacement::draft(0x140f65820, "cohtml_140f65820", tpl_1677::<0x18, 0x140f692e0> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactObject,r::Allocator<coregame::UIFactObject,0>> coregame::UIObjectFactLis
    Replacement::draft(0x140f65850, "cohtml_140f65850", tpl_1678::<0x8, 0x230, 0x18, 0x140f692e0> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactObject,r::Allocator<coregame::UIFactObject,0>> coregame::UIObjectFactLis
    Replacement::draft(0x140f658a0, "cohtml_140f658a0", tpl_1677::<0x18, 0x140f69390> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactObject,r::Allocator<coregame::UIFactObject,0>> coregame::UIObjectFactLis
    Replacement::draft(0x140f658d0, "cohtml_140f658d0", tpl_1679::<0x8, 0x90, 0x18, 0x140f69390> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactObject,r::Allocator<coregame::UIFactObject,0>> coregame::UIObjectFactLis
    Replacement::draft(0x140f65920, "cohtml_140f65920", tpl_61376::<0x78, 0x70, 0x140f69390, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactObject,r::Allocator<coregame::UIFactObject,0>> coregame::UIObjectFactLis
    Replacement::draft(0x140f659e0, "cohtml_140f659e0", tpl_1676::<0x142646d58, 0x14309f708, 0x143315558, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactBinding,r::Allocator<coregame::UIFactBinding,0>> coregame::UIFactList::*
    Replacement::draft(0x140f65a40, "cohtml_140f65a40", tpl_1677::<0x18, 0x140f69410> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactBinding,r::Allocator<coregame::UIFactBinding,0>> coregame::UIFactList::*
    Replacement::draft(0x140f65a70, "cohtml_140f65a70", tpl_1678::<0x8, 0x230, 0x18, 0x140f69410> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactBinding,r::Allocator<coregame::UIFactBinding,0>> coregame::UIFactList::*
    Replacement::draft(0x140f65ac0, "cohtml_140f65ac0", tpl_1677::<0x18, 0x140f694c0> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactBinding,r::Allocator<coregame::UIFactBinding,0>> coregame::UIFactList::*
    Replacement::draft(0x140f65af0, "cohtml_140f65af0", tpl_1679::<0x8, 0x90, 0x18, 0x140f694c0> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactBinding,r::Allocator<coregame::UIFactBinding,0>> coregame::UIFactList::*
    Replacement::draft(0x140f65b40, "cohtml_140f65b40", tpl_61376::<0x78, 0x70, 0x140f694c0, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<std::list<coregame::UIFactBinding,r::Allocator<coregame::UIFactBinding,0>> coregame::UIFactList::*
    Replacement::draft(0x140f65c00, "cohtml_140f65c00", fn_140f65c00 as *const ()), // cohtml::ConcretePropertyReadOnly<const char* (coregame::UIFactBinding::*)(void)>::cohtml::ConcretePropertyReadOnly<const
    Replacement::draft(0x140f65c10, "cohtml_140f65c10", tpl_19200::<0x142646d58, 0x14309f708, 0x143315708, 0x8, 0x10, 0x18> as *const ()), // cohtml::ConcretePropertyReadOnly<const r::InplaceString<64>& (coregame::UIFactBinding::*)(void)>::cohtml::ConcreteProper
    Replacement::draft(0x140f65c70, "cohtml_140f65c70", fn_140f65c70 as *const ()), // cohtml::ConcretePropertyReadOnly<const r::InplaceString<64>& (coregame::UIFactBinding::*)(void)>::cohtml::ConcreteProper
    Replacement::draft(0x140f65ce0, "cohtml_140f65ce0", fn_140f65ce0 as *const ()), // cohtml::ConcretePropertyReadOnly<const r::InplaceString<64>& (coregame::UIFactBinding::*)(void)>::cohtml::ConcreteProper
    Replacement::draft(0x140f65d70, "cohtml_140f65d70", tpl_19200::<0x142646d58, 0x14309f708, 0x1433154a8, 0x8, 0x10, 0x18> as *const ()), // cohtml::ConcretePropertyReadOnly<const char* (coregame::UIFactBinding::*)(void)>::cohtml::ConcretePropertyReadOnly<const
    Replacement::draft(0x140f65dd0, "cohtml_140f65dd0", fn_140f65dd0 as *const ()), // cohtml::ConcretePropertyReadOnly<const char* (coregame::UIFactBinding::*)(void)>::cohtml::ConcretePropertyReadOnly<const
    Replacement::draft(0x140f65e10, "cohtml_140f65e10", fn_140f65e10 as *const ()), // cohtml::ConcretePropertyReadOnly<const char* (coregame::UIFactBinding::*)(void)>::cohtml::ConcretePropertyReadOnly<const
    Replacement::draft(0x140f65e70, "cohtml_140f65e70", fn_140f65e70 as *const ()), // cohtml::ConcretePropertyReadOnly<const char* (coregame::UIFactBinding::*)(void)>::cohtml::ConcretePropertyReadOnly<const
    Replacement::draft(0x140f65ea0, "cohtml_140f65ea0", tpl_1676::<0x142646d58, 0x14309f708, 0x1433155e0, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<float m::Vector4Template<float>::*>::cohtml::FieldProperty<float m::Vector4Template<float>::*>::vf
    Replacement::draft(0x140f65f00, "cohtml_140f65f00", tpl_1757::<0x18, 0x1c8> as *const ()), // cohtml::FieldProperty<float m::Vector3Template<float>::*>::cohtml::FieldProperty<float m::Vector3Template<float>::*>::vf
    Replacement::draft(0x140f65f30, "cohtml_140f65f30", tpl_1758::<0x8, 0x230, 0x18, 0x1c8> as *const ()), // cohtml::FieldProperty<float m::Vector3Template<float>::*>::cohtml::FieldProperty<float m::Vector3Template<float>::*>::vf
    Replacement::draft(0x140f65f90, "cohtml_140f65f90", fn_140f65f90 as *const ()), // cohtml::FieldProperty<float m::Vector3Template<float>::*>::cohtml::FieldProperty<float m::Vector3Template<float>::*>::vf
    Replacement::draft(0x140f65fc0, "cohtml_140f65fc0", fn_140f65fc0 as *const ()), // cohtml::FieldProperty<float m::Vector3Template<float>::*>::cohtml::FieldProperty<float m::Vector3Template<float>::*>::vf
    Replacement::draft(0x140f66020, "cohtml_140f66020", tpl_1676::<0x142646d58, 0x14309f708, 0x143314fc0, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<float m::Vector3Template<float>::*>::cohtml::FieldProperty<float m::Vector3Template<float>::*>::vf
    Replacement::draft(0x140f66080, "cohtml_140f66080", tpl_1676::<0x142646d58, 0x14309f708, 0x143314dd0, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<float m::Vector2Template<float>::*>::cohtml::FieldProperty<float m::Vector2Template<float>::*>::vf
    Replacement::draft(0x140f660e0, "cohtml_140f660e0", fn_140f660e0 as *const ()), // cohtml::FieldProperty<float m::Vector3Template<float>::*>::cohtml::FieldProperty<float m::Vector3Template<float>::*>::vf
    Replacement::draft(0x140f66120, "cohtml_140f66120", fn_140f66120 as *const ()), // cohtml::FieldProperty<float m::Vector3Template<float>::*>::cohtml::FieldProperty<float m::Vector3Template<float>::*>::vf
    Replacement::draft(0x140f66140, "cohtml_140f66140", fn_140f66140 as *const ()), // cohtml::FieldProperty<float m::Vector3Template<float>::*>::cohtml::FieldProperty<float m::Vector3Template<float>::*>::vf
    Replacement::draft(0x140f66180, "cohtml_140f66180", tpl_1676::<0x142646d58, 0x14309f708, 0x143315148, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<r::InplaceString<64> coregame::JSBoxedValue::*>::cohtml::FieldProperty<r::InplaceString<64> corega
    Replacement::draft(0x140f661e0, "cohtml_140f661e0", tpl_1676::<0x142646d58, 0x14309f708, 0x143314d48, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<m::Vector4Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector4Template<flo
    Replacement::draft(0x140f66240, "cohtml_140f66240", tpl_1681::<0x18, 0x38, 0x141e16e60, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector4Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector4Template<flo
    Replacement::draft(0x140f66290, "cohtml_140f66290", tpl_1682::<0x8, 0x230, 0x18, 0x38, 0x141e16e60, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector4Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector4Template<flo
    Replacement::draft(0x140f66300, "cohtml_140f66300", tpl_1681::<0x18, 0x28, 0x141e16e60, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector4Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector4Template<flo
    Replacement::draft(0x140f66350, "cohtml_140f66350", tpl_1683::<0x8, 0x90, 0x18, 0x28, 0x141e16e60, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector4Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector4Template<flo
    Replacement::draft(0x140f663c0, "cohtml_140f663c0", tpl_61379::<0x78, 0x70, 0x28, 0x141e16e60, 0x48, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<m::Vector4Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector4Template<flo
    Replacement::draft(0x140f664a0, "cohtml_140f664a0", tpl_1676::<0x142646d58, 0x14309f708, 0x143314f38, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<m::Vector3Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector3Template<flo
    Replacement::draft(0x140f66500, "cohtml_140f66500", tpl_1681::<0x18, 0x38, 0x141e16d80, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector3Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector3Template<flo
    Replacement::draft(0x140f66550, "cohtml_140f66550", tpl_1682::<0x8, 0x230, 0x18, 0x38, 0x141e16d80, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector3Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector3Template<flo
    Replacement::draft(0x140f665c0, "cohtml_140f665c0", tpl_1681::<0x18, 0x28, 0x141e16d80, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector3Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector3Template<flo
    Replacement::draft(0x140f66610, "cohtml_140f66610", tpl_1683::<0x8, 0x90, 0x18, 0x28, 0x141e16d80, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector3Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector3Template<flo
    Replacement::draft(0x140f66680, "cohtml_140f66680", tpl_61379::<0x78, 0x70, 0x28, 0x141e16d80, 0x48, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<m::Vector3Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector3Template<flo
    Replacement::draft(0x140f66760, "cohtml_140f66760", tpl_1676::<0x142646d58, 0x14309f708, 0x143315680, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<m::Vector2Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector2Template<flo
    Replacement::draft(0x140f667c0, "cohtml_140f667c0", tpl_1681::<0x18, 0x38, 0x141e16cd0, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector2Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector2Template<flo
    Replacement::draft(0x140f66810, "cohtml_140f66810", tpl_1682::<0x8, 0x230, 0x18, 0x38, 0x141e16cd0, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector2Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector2Template<flo
    Replacement::draft(0x140f66880, "cohtml_140f66880", tpl_1681::<0x18, 0x28, 0x141e16cd0, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector2Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector2Template<flo
    Replacement::draft(0x140f668d0, "cohtml_140f668d0", tpl_1683::<0x8, 0x90, 0x18, 0x28, 0x141e16cd0, 0x48> as *const ()), // cohtml::FieldProperty<m::Vector2Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector2Template<flo
    Replacement::draft(0x140f66940, "cohtml_140f66940", tpl_61379::<0x78, 0x70, 0x28, 0x141e16cd0, 0x48, 0x40, 0x10> as *const ()), // cohtml::FieldProperty<m::Vector2Template<float> coregame::JSBoxedValue::*>::cohtml::FieldProperty<m::Vector2Template<flo
    Replacement::draft(0x140f66a20, "cohtml_140f66a20", tpl_1676::<0x142646d58, 0x14309f708, 0x1433151f8, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<double coregame::JSBoxedValue::*>::cohtml::FieldProperty<double coregame::JSBoxedValue::*>::vfunc_
    Replacement::draft(0x140f66a80, "cohtml_140f66a80", tpl_1676::<0x142646d58, 0x14309f708, 0x143315808, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<bool coregame::JSBoxedValue::*>::cohtml::FieldProperty<bool coregame::JSBoxedValue::*>::vfunc_5
    Replacement::draft(0x140f66ae0, "cohtml_140f66ae0", tpl_1676::<0x142646d58, 0x14309f708, 0x1433153a8, 0x8, 0x10, 0x18> as *const ()), // cohtml::FieldProperty<ui::PageResource::BindingType coregame::JSBoxedValue::*>::cohtml::FieldProperty<ui::PageResource::
    Replacement::draft(0x140f66b40, "unknown_140f66b40", fn_140f66b40 as *const ()), // sub_140f66b40
    Replacement::draft(0x140f66b80, "unknown_140f66b80", fn_140f66b80 as *const ()), // sub_140f66b80
    Replacement::draft(0x140f66c00, "unknown_140f66c00", fn_140f66c00 as *const ()), // sub_140f66c00
    Replacement::draft(0x140f66f20, "unknown_140f66f20", fn_140f66f20 as *const ()), // sub_140f66f20
    Replacement::draft(0x140f67080, "coregame_140f67080", fn_140f67080 as *const ()), // coregame::UIDynamicProperty<std::variant<bool,double,r::InplaceString<64>,m::Vector2Template<float>,m::Vector3Template<f
    Replacement::draft(0x140f670e0, "unknown_140f670e0", fn_140f670e0 as *const ()), // sub_140f670e0
    Replacement::draft(0x140f67140, "unknown_140f67140", fn_140f67140 as *const ()), // sub_140f67140
    Replacement::draft(0x140f671d0, "unknown_140f671d0", fn_140f671d0 as *const ()), // sub_140f671d0
    Replacement::draft(0x140f672b0, "unknown_140f672b0", fn_140f672b0 as *const ()), // sub_140f672b0
    Replacement::draft(0x140f67380, "unknown_140f67380", fn_140f67380 as *const ()), // sub_140f67380
    Replacement::draft(0x140f673f0, "unknown_140f673f0", fn_140f673f0 as *const ()), // sub_140f673f0
    Replacement::draft(0x140f67450, "unknown_140f67450", fn_140f67450 as *const ()), // sub_140f67450
    Replacement::draft(0x140f674a0, "unknown_140f674a0", fn_140f674a0 as *const ()), // sub_140f674a0
    Replacement::draft(0x140f67590, "unknown_140f67590", fn_140f67590 as *const ()), // sub_140f67590
    Replacement::draft(0x140f67600, "unknown_140f67600", fn_140f67600 as *const ()), // sub_140f67600
    Replacement::draft(0x140f67840, "unknown_140f67840", fn_140f67840 as *const ()), // sub_140f67840
    Replacement::draft(0x140f67a70, "unknown_140f67a70", fn_140f67a70 as *const ()), // sub_140f67a70
    Replacement::draft(0x140f67cb0, "unknown_140f67cb0", fn_140f67cb0 as *const ()), // sub_140f67cb0
    Replacement::draft(0x140f67ef0, "unknown_140f67ef0", fn_140f67ef0 as *const ()), // sub_140f67ef0
    Replacement::draft_throws(0x140f68130, "unknown_140f68130", fn_140f68130 as *const (), Throws::Discard), // sub_140f68130
    Replacement::draft_throws(0x140f68300, "unknown_140f68300", fn_140f68300 as *const (), Throws::Caught), // sub_140f68300
    Replacement::draft_throws(0x140f684d0, "unknown_140f684d0", fn_140f684d0 as *const (), Throws::Caught), // sub_140f684d0
    Replacement::draft_throws(0x140f686a0, "unknown_140f686a0", fn_140f686a0 as *const (), Throws::Caught), // sub_140f686a0
    Replacement::draft_throws(0x140f68870, "unknown_140f68870", fn_140f68870 as *const (), Throws::Caught), // sub_140f68870
    Replacement::draft_throws(0x140f68a40, "unknown_140f68a40", fn_140f68a40 as *const (), Throws::Caught), // sub_140f68a40
    Replacement::draft(0x140f68c10, "unknown_140f68c10", fn_140f68c10 as *const ()), // sub_140f68c10
    Replacement::draft(0x140f68f70, "unknown_140f68f70", tpl_3605::<0x19, 0x10, 0x140f68f70, 0x28, 0x140f672b0, 0x1401e66f0> as *const ()), // sub_140f68f70
    Replacement::draft(0x140f68ff0, "unknown_140f68ff0", tpl_60956::<0x19, 0x10, 0x140f68ff0, 0x20, 0x140f6be00, 0x1401e66f0> as *const ()), // sub_140f68ff0
    Replacement::draft(0x140f69280, "unknown_140f69280", fn_140f69280 as *const ()), // sub_140f69280
    Replacement::draft(0x140f692e0, "unknown_140f692e0", tpl_77609::<0x240, 0x140f66c00, 0x248, 0x38, 0x10, 0x141e15710, 0x48, 0x250> as *const ()), // sub_140f692e0
    Replacement::draft(0x140f69390, "unknown_140f69390", tpl_77610::<0x8, 0x150, 0x28, 0x10, 0x141e15710, 0x48, 0x158> as *const ()), // sub_140f69390
    Replacement::draft(0x140f69410, "cohtml_140f69410", tpl_77609::<0x240, 0x140f66f20, 0x248, 0x38, 0x10, 0x141e16720, 0x48, 0x250> as *const ()), // sub_140f69410
    Replacement::draft(0x140f694c0, "cohtml_140f694c0", tpl_77610::<0x8, 0x150, 0x28, 0x10, 0x141e16720, 0x48, 0x158> as *const ()), // sub_140f694c0
    Replacement::draft(0x140f69540, "unknown_140f69540", fn_140f69540 as *const ()), // sub_140f69540
    Replacement::draft(0x140f696f0, "cohtml_140f696f0", fn_140f696f0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&)>::cohtml::Event
    Replacement::draft(0x140f697a0, "cohtml_140f697a0", fn_140f697a0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const m::Vector4
    Replacement::draft(0x140f698b0, "cohtml_140f698b0", fn_140f698b0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const m::Vector3
    Replacement::draft(0x140f699c0, "cohtml_140f699c0", fn_140f699c0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const m::Vector2
    Replacement::draft(0x140f69ae0, "cohtml_140f69ae0", fn_140f69ae0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f69bd0, "cohtml_140f69bd0", fn_140f69bd0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f69cc0, "cohtml_140f69cc0", fn_140f69cc0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,double)>::cohtml
    Replacement::draft(0x140f69da0, "cohtml_140f69da0", fn_140f69da0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,bool)>::cohtml::
    Replacement::draft(0x140f69e80, "cohtml_140f69e80", tpl_3819::<0x140f6aa70> as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::DynamicArray<coregame::JSBoxedValue
    Replacement::draft(0x140f69e90, "cohtml_140f69e90", fn_140f69e90 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&)>::cohtml::EventHandler<void (coregame::UIDa
    Replacement::draft(0x140f69f10, "cohtml_140f69f10", fn_140f69f10 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f6a060, "cohtml_140f6a060", fn_140f6a060 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f6a1b0, "cohtml_140f6a1b0", fn_140f6a1b0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f6a310, "cohtml_140f6a310", fn_140f6a310 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f6a440, "cohtml_140f6a440", fn_140f6a440 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f6a570, "cohtml_140f6a570", fn_140f6a570 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f6a6a0, "cohtml_140f6a6a0", fn_140f6a6a0 as *const ()), // cohtml::EventHandler<void (coregame::UIData::*)(const r::InplaceString<32>&,const r::InplaceString<32>&,const r::Inplace
    Replacement::draft(0x140f6a7d0, "cohtml_140f6a7d0", fn_140f6a7d0 as *const ()), // cohtml::EventHandler<void (coregame::UIFactBinding::*)(const r::InplaceString<64>&)>::cohtml::EventHandler<void (coregam
    Replacement::draft(0x140f6a850, "cohtml_140f6a850", fn_140f6a850 as *const ()), // cohtml::EventHandler<void (coregame::UIFactBinding::*)(bool)>::cohtml::EventHandler<void (coregame::UIFactBinding::*)(bo
    Replacement::draft(0x140f6a8b0, "cohtml_140f6a8b0", fn_140f6a8b0 as *const ()), // cohtml::EventHandler<void (coregame::UIFactBinding::*)(const m::Vector4Template<float>&)>::cohtml::EventHandler<void (co
    Replacement::draft(0x140f6a940, "cohtml_140f6a940", fn_140f6a940 as *const ()), // cohtml::EventHandler<void (coregame::UIFactBinding::*)(const m::Vector3Template<float>&)>::cohtml::EventHandler<void (co
    Replacement::draft(0x140f6a9e0, "cohtml_140f6a9e0", fn_140f6a9e0 as *const ()), // cohtml::EventHandler<void (coregame::UIFactBinding::*)(const m::Vector2Template<float>&)>::cohtml::EventHandler<void (co
    Replacement::draft(0x140f6aa70, "cohtml_140f6aa70", fn_140f6aa70 as *const ()), // sub_140f6aa70
    Replacement::draft(0x140f6abe0, "unknown_140f6abe0", fn_140f6abe0 as *const ()), // sub_140f6abe0
    Replacement::draft(0x140f6ad70, "unknown_140f6ad70", fn_140f6ad70 as *const ()), // sub_140f6ad70
    Replacement::draft(0x140f6ae80, "unknown_140f6ae80", fn_140f6ae80 as *const ()), // sub_140f6ae80
    Replacement::draft_throws(0x140f6b150, "unknown_140f6b150", fn_140f6b150 as *const (), Throws::Discard), // sub_140f6b150
    Replacement::draft_throws(0x140f6b320, "unknown_140f6b320", fn_140f6b320 as *const (), Throws::Discard), // sub_140f6b320
    Replacement::draft_throws(0x140f6b4d0, "unknown_140f6b4d0", fn_140f6b4d0 as *const (), Throws::Discard), // sub_140f6b4d0
    Replacement::draft_throws(0x140f6b6a0, "unknown_140f6b6a0", fn_140f6b6a0 as *const (), Throws::Discard), // sub_140f6b6a0
    Replacement::draft_throws(0x140f6b870, "unknown_140f6b870", fn_140f6b870 as *const (), Throws::Discard), // sub_140f6b870
    Replacement::draft(0x140f6ba40, "unknown_140f6ba40", tpl_1689::<0x28, 0x141e16720, 0x48> as *const ()), // sub_140f6ba40
    Replacement::draft(0x140f6ba80, "unknown_140f6ba80", tpl_1689::<0x28, 0x141e16640, 0x48> as *const ()), // sub_140f6ba80
    Replacement::draft(0x140f6bac0, "unknown_140f6bac0", tpl_1689::<0x28, 0x141e166b0, 0x48> as *const ()), // sub_140f6bac0
    Replacement::draft(0x140f6bb00, "unknown_140f6bb00", tpl_1689::<0x28, 0x141e165d0, 0x48> as *const ()), // sub_140f6bb00
    Replacement::draft(0x140f6bb40, "unknown_140f6bb40", tpl_1689::<0x28, 0x141e15ca0, 0x48> as *const ()), // sub_140f6bb40
    Replacement::draft(0x140f6bb80, "unknown_140f6bb80", fn_140f6bb80 as *const ()), // sub_140f6bb80
    Replacement::draft(0x140f6bbf0, "unknown_140f6bbf0", fn_140f6bbf0 as *const ()), // sub_140f6bbf0
    Replacement::draft(0x140f6be00, "unknown_140f6be00", fn_140f6be00 as *const ()), // sub_140f6be00
    Replacement::draft(0x140f6be70, "unknown_140f6be70", fn_140f6be70 as *const ()), // sub_140f6be70
    Replacement::draft(0x140f6beb0, "unknown_140f6beb0", fn_140f6beb0 as *const ()), // sub_140f6beb0
    Replacement::draft(0x140f6c020, "unknown_140f6c020", fn_140f6c020 as *const ()), // sub_140f6c020
    Replacement::draft_throws(0x140f6c060, "coregame_140f6c060", tpl_6046::<0x143cc67cc, 0x143cc67d0, 0x1411e9f50, 0x140f6c500, 0x140f6c7a0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143315a86, 0x142646960, 0x143cc67f0, 0x143cc67f8, 0x143cc67fc, 0x143cc67fe, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_bundle_flow::update_input::system>
    Replacement::draft_throws(0x140f6c2c0, "coregame_140f6c2c0", tpl_5893::<0x143cc6800, 0x143cc6808, 0x1411e9f50, 0x140f6c650, 0x140f6c840, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143315cf6, 0x142646960, 0x143cc6828, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_bundle_flow::sync_ui_state::system>
    Replacement::draft_throws(0x140f6c500, "coregame_140f6c500", tpl_19258::<0x14024e320, 0x143c8ce88, 0x140215610, 0x143c8ce90, 0x143c8cea8, 0x143c8cea4, 0x1420a28e0> as *const (), Throws::Discard), // sub_140f6c500
    Replacement::draft_throws(0x140f6c650, "coregame_140f6c650", tpl_19258::<0x14024e320, 0x143c8ce88, 0x140215610, 0x143cc6838, 0x143c8ce90, 0x143c8cea4, 0x1420a28e0> as *const (), Throws::Discard), // sub_140f6c650
    Replacement::draft_throws(0x140f6c7a0, "coregame_140f6c7a0", tpl_19259::<0x143c8ced4, 0x143c8ce90, 0x143c8cea8, 0x20, 0x143c8ce88, 0x8, 0x580e0, 0x143c8cea4, 0x141e17250> as *const (), Throws::Discard), // sub_140f6c7a0
    Replacement::draft_throws(0x140f6c840, "coregame_140f6c840", tpl_19259::<0x143c8ced4, 0x143cc6838, 0x143c8ce90, 0x20, 0x143c8ce88, 0x8, 0x580e0, 0x143c8cea4, 0x141e175c0> as *const (), Throws::Discard), // sub_140f6c840
    Replacement::draft(0x140f6c8e0, "coregame_140f6c8e0", tpl_5510::<0x143c8d120, 0x142489ee0, 0x143cc6a20, 0x143cc69b0> as *const ()), // sub_140f6c8e0
    Replacement::draft(0x140f6c9e0, "coregame_140f6c9e0", fn_140f6c9e0 as *const ()), // sub_140f6c9e0
    Replacement::draft_throws(0x140f6cae0, "coregame_140f6cae0", tpl_1480::<0x1428af490, 0x1410d6ca0, 0x140f6d020, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140f6cae0
    Replacement::draft_throws(0x140f6cd80, "coregame_140f6cd80", tpl_1480::<0x1428af490, 0x1410d6ca0, 0x140f6d240, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140f6cd80
    Replacement::draft_throws(0x140f6d020, "coregame_140f6d020", tpl_1770::<0x143cc6a28, 0x143cc6900, 0x140f6d570, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143316686, 0x142646960, 0x143cc6920, 0x143cc6930, 0x143cc6934, 0x143cc6936, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_audio_caption::setCurrentAudioCaptionData>
    Replacement::draft_throws(0x140f6d240, "coregame_140f6d240", fn_140f6d240 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ui_audio_caption::syncCurrentAudioCaptionDataToUI>
    Replacement::draft_throws(0x140f6d570, "unknown_140f6d570", fn_140f6d570 as *const (), Throws::Discard), // sub_140f6d570
    Replacement::draft(0x140f6d800, "unknown_140f6d800", fn_140f6d800 as *const ()), // sub_140f6d800
    Replacement::draft(0x140f6e4c0, "unknown_140f6e4c0", fn_140f6e4c0 as *const ()), // sub_140f6e4c0
    Replacement::draft_throws(0x140f6e500, "coregame_140f6e500", tpl_5091::<0x143cc7134, 0x143cc6df8, 0x140f708e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433196c6, 0x142646960, 0x143cc6e18, 0x143cc6e28, 0x143cc6e2c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger_scripting::updateTransitionTrigger>
    Replacement::draft_throws(0x140f6e710, "coregame_140f6e710", tpl_3464::<0x143cc70e4, 0x143cc6f48, 0x1411e9f50, 0x140f71600, 0x140f73040, 0x140f717d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143319206, 0x142646960, 0x143cc6f68, 0x143cc6f78, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger_scripting::updateCollisionTrigger>
    Replacement::draft_throws(0x140f6e960, "coregame_140f6e960", fn_140f6e960 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger_scripting::updateVisibilityTrigger>
    Replacement::draft_throws(0x140f6eca0, "coregame_140f6eca0", tpl_3395::<0x143cc7390, 0x143cc72a8, 0x140f70b10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143319b76, 0x142646960, 0x143cc72c8, 0x143cc72d0, 0x143cc72d2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger_scripting::updateVisible>
    Replacement::draft_throws(0x140f6eeb0, "coregame_140f6eeb0", tpl_19267::<0x143cc6cb0, 0x143cc6c08, 0x140f70ce0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143318a06, 0x142646960, 0x143cc6c28, 0x143cc6c38, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger_scripting::updateVisibleCenter>
    Replacement::draft_throws(0x140f6f0c0, "coregame_140f6f0c0", tpl_3061::<0x143cc7354, 0x143cc7358, 0x1411e9f50, 0x140f74e20, 0x143cc6e6c, 0x1420af120, 0x1420a28e0, 0x140f736a0, 0x140f71c80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143318836, 0x142646960, 0x143cc7378, 0x143cc7380, 0x143cc7384, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger_scripting::applyAutoDisable>
    Replacement::draft_throws(0x140f6f3b0, "coregame_140f6f3b0", fn_140f6f3b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger_scripting::updateCameraTriggerCollider>
    Replacement::draft(0x140f6f6f0, "unknown_140f6f6f0", tpl_1094::<0x143cc6f80, 0x140f70fe0, 0x14020e8f0> as *const ()), // sub_140f6f6f0
    Replacement::draft(0x140f6f8f0, "unknown_140f6f8f0", tpl_1095::<0x143cc6db8, 0x14020e8f0> as *const ()), // sub_140f6f8f0
    Replacement::draft(0x140f6fad0, "unknown_140f6fad0", tpl_1095::<0x143cc6cf0, 0x14020e8f0> as *const ()), // sub_140f6fad0
    Replacement::draft(0x140f6fcb0, "unknown_140f6fcb0", tpl_1095::<0x143cc6cfc, 0x14020e8f0> as *const ()), // sub_140f6fcb0
    Replacement::draft(0x140f6fe90, "unknown_140f6fe90", tpl_3074::<0x143cc6d1c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f6fe90
    Replacement::draft(0x140f70160, "unknown_140f70160", tpl_1094::<0x143cc6d44, 0x140f71170, 0x14020e8f0> as *const ()), // sub_140f70160
    Replacement::draft(0x140f70360, "unknown_140f70360", tpl_1094::<0x143cc6d04, 0x140f71300, 0x14020e8f0> as *const ()), // sub_140f70360
    Replacement::draft(0x140f70560, "unknown_140f70560", fn_140f70560 as *const ()), // sub_140f70560
    Replacement::draft_throws(0x140f708e0, "unknown_140f708e0", tpl_3083::<0x1411e9f50, 0x140f74780, 0x143cc7140, 0x1420af120, 0x140f74640, 0x143cc6e70, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x140f72e70, 0x140f71490> as *const (), Throws::Discard), // sub_140f708e0
    Replacement::draft_throws(0x140f70b10, "unknown_140f70b10", tpl_1405::<0x1411e9f50, 0x140f74b40, 0x143cc70ec, 0x1420af120, 0x14036fad0, 0x143c8f63c, 0x140215610, 0x143c8ceec, 0x1420a28e0, 0x140f73330, 0x140f71a20> as *const (), Throws::Discard), // sub_140f70b10
    Replacement::draft_throws(0x140f70ce0, "unknown_140f70ce0", tpl_1405::<0x1411e9f50, 0x140f74ce0, 0x143cc6cf4, 0x1420af120, 0x14036fad0, 0x143c8f63c, 0x140215610, 0x143c8ceec, 0x1420a28e0, 0x140f73530, 0x140f71b50> as *const (), Throws::Discard), // sub_140f70ce0
    Replacement::draft(0x140f70ee0, "unknown_140f70ee0", tpl_19270::<0x8, 0x140f6fcb0, 0x140f6fe90> as *const ()), // sub_140f70ee0
    Replacement::draft(0x140f70fe0, "unknown_140f70fe0", tpl_4469::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140f72b50> as *const ()), // sub_140f70fe0
    Replacement::draft(0x140f71170, "unknown_140f71170", fn_140f71170 as *const ()), // sub_140f71170
    Replacement::draft(0x140f71300, "unknown_140f71300", tpl_4469::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140f72ce0> as *const ()), // sub_140f71300
    Replacement::draft(0x140f71490, "unknown_140f71490", fn_140f71490 as *const ()), // sub_140f71490
    Replacement::draft_throws(0x140f71600, "unknown_140f71600", tpl_5076::<0x140f748c0, 0x143cc6cf8, 0x1420af120, 0x140f721b0, 0x140f74460, 0x143cc6d08, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0> as *const (), Throws::Discard), // sub_140f71600
    Replacement::draft(0x140f717d0, "unknown_140f717d0", fn_140f717d0 as *const ()), // sub_140f717d0
    Replacement::draft(0x140f71910, "unknown_140f71910", fn_140f71910 as *const ()), // sub_140f71910
    Replacement::draft_throws(0x140f71a20, "unknown_140f71a20", fn_140f71a20 as *const (), Throws::Discard), // sub_140f71a20
    Replacement::draft_throws(0x140f71b50, "unknown_140f71b50", fn_140f71b50 as *const (), Throws::Discard), // sub_140f71b50
    Replacement::draft(0x140f71c80, "unknown_140f71c80", tpl_19272::<0x88, 0x10088, 0x140f723e0, 0x141e1b980> as *const ()), // sub_140f71c80
    Replacement::draft(0x140f71d00, "unknown_140f71d00", tpl_3141::<0x88, 0x10088, 0x143c94b44, 0x20, 0x8, 0x143c8f63c, 0x580e0, 0x140f71f40, 0x140f71e10> as *const ()), // sub_140f71d00
    Replacement::draft(0x140f71e10, "unknown_140f71e10", tpl_3142::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141e1a670> as *const ()), // sub_140f71e10
    Replacement::draft_throws(0x140f721b0, "unknown_140f721b0", tpl_1330::<0x8, 0x140f74550, 0x143cc7388, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140f721b0
    Replacement::draft(0x140f72b50, "unknown_140f72b50", fn_140f72b50 as *const ()), // sub_140f72b50
    Replacement::draft(0x140f72ce0, "unknown_140f72ce0", fn_140f72ce0 as *const ()), // sub_140f72ce0
    Replacement::draft(0x140f72e70, "unknown_140f72e70", fn_140f72e70 as *const ()), // sub_140f72e70
    Replacement::draft(0x140f731d0, "unknown_140f731d0", fn_140f731d0 as *const ()), // sub_140f731d0
    Replacement::draft_throws(0x140f73330, "unknown_140f73330", fn_140f73330 as *const (), Throws::Discard), // sub_140f73330
    Replacement::draft_throws(0x140f73530, "unknown_140f73530", fn_140f73530 as *const (), Throws::Discard), // sub_140f73530
    Replacement::draft(0x140f736a0, "unknown_140f736a0", tpl_77668::<0x143c8d87c, 0x142489ee0, 0x143cc6d1c, 0x143cc7394, 0x141e1b980> as *const ()), // sub_140f736a0
    Replacement::draft(0x140f738a0, "unknown_140f738a0", tpl_62570::<0x143c94b44, 0x143c8f63c, 0x140f72000, 0x141e1a670> as *const ()), // sub_140f738a0
    Replacement::draft(0x140f756e0, "unknown_140f756e0", fn_140f756e0 as *const ()), // sub_140f756e0
    Replacement::draft(0x140f75750, "unknown_140f75750", fn_140f75750 as *const ()), // sub_140f75750
    Replacement::draft(0x140f757b0, "unknown_140f757b0", fn_140f757b0 as *const ()), // sub_140f757b0
    Replacement::draft(0x140f75820, "unknown_140f75820", tpl_16098::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8> as *const ()), // sub_140f75820
    Replacement::draft(0x140f758a0, "unknown_140f758a0", fn_140f758a0 as *const ()), // sub_140f758a0
    Replacement::draft(0x140f75a50, "unknown_140f75a50", fn_140f75a50 as *const ()), // sub_140f75a50
    Replacement::draft_throws(0x140f75a80, "coregame_140f75a80", tpl_1910::<0x143cc78e0, 0x143cc76c0, 0x1411e9f50, 0x140f7acd0, 0x143cc76ac, 0x1420af120, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x140f79560, 0x140f78b10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331b606, 0x142646960, 0x143cc76d0, 0x143cc76d8, 0x143cc76dc, 0x143cc76de, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger::moveTrigger>
    Replacement::draft_throws(0x140f75da0, "coregame_140f75da0", tpl_3085::<0x143cc78c8, 0x143cc7780, 0x1411e9f50, 0x140f7ade0, 0x143cc76f4, 0x1420af120, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x140f79790, 0x140f78bc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331b926, 0x142646960, 0x143cc77a0, 0x143cc77a4, 0x143cc77a6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger::applyTriggerEnabled>
    Replacement::draft_throws(0x140f760c0, "coregame_140f760c0", tpl_4129::<0x143cc78d4, 0x143cc75d0, 0x1411e9f50, 0x140f7aed0, 0x143cc7708, 0x1420af120, 0x1420a28e0, 0x140f79990, 0x140f78c60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331b806, 0x142646960, 0x143cc75f0, 0x143cc75f4, 0x143cc75f6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger::clearTriggerObjects>
    Replacement::draft_throws(0x140f763b0, "coregame_140f763b0", tpl_1467::<0x143cc76b4, 0x143cc74d8, 0x1411e9f50, 0x140f7af80, 0x143cc7454, 0x1420af120, 0x1420a28e0, 0x140f79ac0, 0x140f78d40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331b116, 0x142646960, 0x143cc74f8, 0x143cc7500, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger::updateTriggerFromPhysics>
    Replacement::draft_throws(0x140f76690, "coregame_140f76690", tpl_4485::<0x143cc739c, 0x143cc7530, 0x1411e9f50, 0x140f7b0c0, 0x143cc78d8, 0x1420af120, 0x1420a28e0, 0x140f79b90, 0x140f78dc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331aef6, 0x142646960, 0x143cc7550, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger::postEventsOnHide>
    Replacement::draft_throws(0x140f76960, "coregame_140f76960", tpl_3683::<0x143cc76a0, 0x143cc7750, 0x1411e9f50, 0x140f7b200, 0x143cc78cc, 0x1420af120, 0x1420a28e0, 0x140f79d90, 0x140f78e40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331b436, 0x142646960, 0x143cc7770, 0x143cc7778, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger::postEventsOnStreamOut>
    Replacement::draft(0x140f76c40, "coregame_140f76c40", tpl_1764::<0x143cc76e4, 0x143cc7458, 0x140f77c30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331b346, 0x142646960, 0x143cc7478, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::trigger::removedTrigger>
    Replacement::draft(0x140f76e40, "coregame_140f76e40", tpl_1526::<0x1428b0720, 0x1410d6ca0, 0x140f76c40, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140f76e40
    Replacement::draft(0x140f770e0, "coregame_140f770e0", tpl_1094::<0x143cc744c, 0x140f784c0, 0x14020e8f0> as *const ()), // sub_140f770e0
    Replacement::draft(0x140f772e0, "coregame_140f772e0", tpl_1094::<0x143cc6d00, 0x140f78640, 0x14020e8f0> as *const ()), // sub_140f772e0
    Replacement::draft(0x140f774e0, "coregame_140f774e0", tpl_1402::<0x143cc76a4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f774e0
    Replacement::draft(0x140f77830, "coregame_140f77830", tpl_1094::<0x143cc7394, 0x140f787c0, 0x14020e8f0> as *const ()), // sub_140f77830
    Replacement::draft(0x140f77a30, "coregame_140f77a30", tpl_1094::<0x143cb0ae8, 0x140f78970, 0x14020e8f0> as *const ()), // sub_140f77a30
    Replacement::draft(0x140f77c30, "coregame_140f77c30", tpl_5024::<0x1411e9f50, 0x140f7a1a0, 0x143cc6d00, 0x1401fc320, 0x143c8f63c, 0x140215610, 0x143cada18, 0x1420a28e0, 0x140f79f40, 0x1401de410> as *const ()), // sub_140f77c30
    Replacement::draft_throws(0x140f77dd0, "coregame_140f77dd0", tpl_19279::<0x143cc76a8, 0x143cc73a8, 0x1411e9f50, 0x140f7b310, 0x143cc78d0, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140f7a980, 0x140f7a000, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331a8a6, 0x142646960, 0x143cc73b8, 0x143cc73c0, 0x143cc73c4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger::streamOut>
    Replacement::draft_throws(0x140f780c0, "coregame_140f780c0", fn_140f780c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::trigger::streamIn>
    Replacement::draft(0x140f78430, "unknown_140f78430", fn_140f78430 as *const ()), // sub_140f78430
    Replacement::draft(0x140f784c0, "unknown_140f784c0", tpl_2731::<0x1401e6d50, 0x140f79470, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_140f784c0
    Replacement::draft(0x140f78640, "unknown_140f78640", tpl_2731::<0x1401e6d50, 0x140f794d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_140f78640
    Replacement::draft(0x140f787c0, "unknown_140f787c0", fn_140f787c0 as *const ()), // sub_140f787c0
    Replacement::draft(0x140f78970, "unknown_140f78970", fn_140f78970 as *const ()), // sub_140f78970
    Replacement::draft(0x140f78b10, "unknown_140f78b10", fn_140f78b10 as *const ()), // sub_140f78b10
    Replacement::draft(0x140f78bc0, "unknown_140f78bc0", tpl_77679::<0x88, 0x10088, 0x143c8f63c, 0x580e0, 0x140f79240, 0x141e1efc0> as *const ()), // sub_140f78bc0
    Replacement::draft(0x140f78c60, "unknown_140f78c60", fn_140f78c60 as *const ()), // sub_140f78c60
    Replacement::draft(0x140f78d40, "unknown_140f78d40", tpl_19285::<0x88, 0x10088, 0x140f790e0, 0x141e1ebf0> as *const ()), // sub_140f78d40
    Replacement::draft(0x140f78dc0, "unknown_140f78dc0", tpl_19272::<0x88, 0x10088, 0x140f78fb0, 0x141e1eac0> as *const ()), // sub_140f78dc0
    Replacement::draft(0x140f78e40, "unknown_140f78e40", tpl_19286::<0x88, 0x10088, 0x140f78eb0, 0x141e1e9b0> as *const ()), // sub_140f78e40
    Replacement::draft(0x140f79470, "unknown_140f79470", fn_140f79470 as *const ()), // sub_140f79470
    Replacement::draft(0x140f794d0, "unknown_140f794d0", tpl_1955::<0x8, 0x140f756e0> as *const ()), // sub_140f794d0
    Replacement::draft(0x140f79500, "unknown_140f79500", tpl_19288::<0x7, 0x90, 0x60, 0x1403b8050, 0x30> as *const ()), // sub_140f79500
    Replacement::draft(0x140f79990, "unknown_140f79990", fn_140f79990 as *const ()), // sub_140f79990
    Replacement::draft(0x140f79ac0, "unknown_140f79ac0", tpl_77683::<0x8, 0x88, 0x10088, 0x140f790e0, 0x141e1ebf0> as *const ()), // sub_140f79ac0
    Replacement::draft(0x140f79b90, "unknown_140f79b90", tpl_77668::<0x143cb0ae8, 0x142489ee0, 0x143cc7394, 0x143c8d87c, 0x141e1eac0> as *const ()), // sub_140f79b90
    Replacement::draft(0x140f79d90, "unknown_140f79d90", tpl_77684::<0x143cb0ae8, 0x142489ee0, 0x143cc7394, 0x141e1e9b0> as *const ()), // sub_140f79d90
    Replacement::draft(0x140f79f40, "unknown_140f79f40", tpl_5049::<0x143cc6d00, 0x5e308, 0x580e0, 0x143c8f63c, 0x143cada18, 0x10, 0x8, 0x18, 0x141e1e2e0, 0x140f7a690> as *const ()), // sub_140f79f40
    Replacement::draft(0x140f7a000, "unknown_140f7a000", tpl_1898::<0x88, 0x10088, 0x141e1e3f0> as *const ()), // sub_140f7a000
    Replacement::draft_throws(0x140f7a090, "unknown_140f7a090", fn_140f7a090 as *const (), Throws::Discard), // sub_140f7a090
    Replacement::draft(0x140f7a1a0, "unknown_140f7a1a0", tpl_3188::<0x140f7ac30, 0x14331c578, 0x140f7a690, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140f7a1a0
    Replacement::draft(0x140f7a220, "unknown_140f7a220", tpl_4348::<0x10, 0x18, 0x140f7a690> as *const ()), // sub_140f7a220
    Replacement::draft(0x140f7a550, "unknown_140f7a550", fn_140f7a550 as *const ()), // sub_140f7a550
    Replacement::draft(0x140f7a690, "unknown_140f7a690", fn_140f7a690 as *const ()), // sub_140f7a690
    Replacement::draft(0x140f7a980, "unknown_140f7a980", tpl_61552::<0x8, 0x88, 0x10088, 0x141e1e3f0> as *const ()), // sub_140f7a980
    Replacement::draft_throws(0x140f7aa70, "unknown_140f7aa70", fn_140f7aa70 as *const (), Throws::Discard), // sub_140f7aa70
    Replacement::draft(0x140f7ac30, "unknown_140f7ac30", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14331c578, 0x8, 0x20> as *const ()), // sub_140f7ac30
    Replacement::draft(0x140f7b400, "unknown_140f7b400", fn_140f7b400 as *const ()), // sub_140f7b400
    Replacement::draft(0x140f7ba40, "unknown_140f7ba40", tpl_3045::<0x30, 0x143cc7954, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140f7bac2> as *const ()), // sub_140f7ba40
    Replacement::draft(0x140f7bad0, "unknown_140f7bad0", fn_140f7bad0 as *const ()), // sub_140f7bad0
    Replacement::draft(0x140f7bb60, "unknown_140f7bb60", tpl_60953::<0x1426467e8, 0x143cc7b78, 0x142489ee0, 0x140f7bbdc> as *const ()), // sub_140f7bb60
    Replacement::draft(0x140f7bd10, "unknown_140f7bd10", fn_140f7bd10 as *const ()), // sub_140f7bd10
    Replacement::draft(0x140f7bd90, "unknown_140f7bd90", tpl_61197::<0x143c8d074, 0x142489ee0, 0x143c933f0, 0x143c93358, 0x143c92bcc, 0x1426467e8, 0x140f7bf23> as *const ()), // sub_140f7bd90
    Replacement::draft_throws(0x140f7bf30, "coregame_140f7bf30", tpl_1176::<0x143cc7958, 0x143cc7960, 0x140f804a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331ee46, 0x142646960, 0x143cc7980, 0x143cc7990, 0x143cc7992, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::invalidateChildrenWhenRemoved>
    Replacement::draft_throws(0x140f7c140, "coregame_140f7c140", fn_140f7c140 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::invalidateParentWhenRemoved>
    Replacement::draft_throws(0x140f7c410, "coregame_140f7c410", tpl_1102::<0x143cc79cc, 0x143cc79d0, 0x1411e9f50, 0x140f83930, 0x143cc7f24, 0x1420af120, 0x1420a28e0, 0x140f81d30, 0x140f80e80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331e796, 0x142646960, 0x143cc79f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::overrideSpawningTransforms>
    Replacement::draft_throws(0x140f7c6f0, "coregame_140f7c6f0", tpl_3281::<0x143cc7a00, 0x143cc7a08, 0x1411e9f50, 0x140f83840, 0x143cc7f20, 0x1420af120, 0x1420a28e0, 0x140f81f50, 0x140f80fa0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331eca6, 0x142646960, 0x143cc7a28, 0x143cc7a30, 0x143cc7a34, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::overrideSpawningParents>
    Replacement::draft_throws(0x140f7c9e0, "coregame_140f7c9e0", tpl_6402::<0x143cc7a38, 0x143cc7a40, 0x140f80670, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331eb26, 0x142646960, 0x143cc7a60, 0x143cc7a70, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::queueUninitializedToUnresolved>
    Replacement::draft_throws(0x140f7cbf0, "coregame_140f7cbf0", tpl_6046::<0x143cc7a74, 0x143cc7a78, 0x1411e9f50, 0x140f81090, 0x140f821a0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331e0d6, 0x142646960, 0x143cc7a98, 0x143cc7aa0, 0x143cc7aa4, 0x143cc7aa6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::resolveParentsAndChildren>
    Replacement::draft_throws(0x140f7ce50, "coregame_140f7ce50", tpl_5273::<0x143cc7aa8, 0x143cc7ab0, 0x1411e9f50, 0x140f81220, 0x140f82270, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331d816, 0x142646960, 0x143cc7ad0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::applyHierarchy>
    Replacement::draft_throws(0x140f7d090, "coregame_140f7d090", fn_140f7d090 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::clearTeleported>
    Replacement::draft_throws(0x140f7d3a0, "coregame_140f7d3a0", tpl_4804::<0x143cc7b00, 0x143cc7b08, 0x140f80830, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331e3d6, 0x142646960, 0x143cc7b28, 0x143cc7b30, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::transform::snapPreviousTransforms>
    Replacement::draft(0x140f7d5b0, "unknown_140f7d5b0", tpl_60953::<0x1426467e8, 0x143cc7954, 0x142489ee0, 0x140f7d62c> as *const ()), // sub_140f7d5b0
    Replacement::draft_throws(0x140f7d760, "unknown_140f7d760", tpl_19296::<0xc, 0x8, 0x14241c0a0, 0x14183acc8, 0x14241c190> as *const (), Throws::Discard), // sub_140f7d760
    Replacement::draft(0x140f7da90, "unknown_140f7da90", tpl_77694::<0x1426467e8, 0x140f80b60, 0x140f7dab6> as *const ()), // sub_140f7da90
    Replacement::draft(0x140f7dd90, "unknown_140f7dd90", tpl_1402::<0x143c933f0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7dd90
    Replacement::draft(0x140f7e0e0, "unknown_140f7e0e0", tpl_1402::<0x143c8d120, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7e0e0
    Replacement::draft(0x140f7e430, "unknown_140f7e430", tpl_1402::<0x143c93358, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7e430
    Replacement::draft(0x140f7e780, "unknown_140f7e780", tpl_1402::<0x143c8f6d8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7e780
    Replacement::draft(0x140f7ead0, "unknown_140f7ead0", tpl_1368::<0x143c8dbd4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7ead0
    Replacement::draft(0x140f7ee00, "unknown_140f7ee00", tpl_1466::<0x143c8d074, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7ee00
    Replacement::draft(0x140f7f0d0, "unknown_140f7f0d0", tpl_1368::<0x143cc7b78, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7f0d0
    Replacement::draft(0x140f7f400, "unknown_140f7f400", tpl_1097::<0x143c92bcc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7f400
    Replacement::draft(0x140f7f730, "unknown_140f7f730", tpl_1094::<0x143cc7954, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_140f7f730
    Replacement::draft(0x140f7f930, "unknown_140f7f930", tpl_1095::<0x143cc7b34, 0x14020e8f0> as *const ()), // sub_140f7f930
    Replacement::draft(0x140f7fb10, "unknown_140f7fb10", tpl_1368::<0x143c92b0c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7fb10
    Replacement::draft(0x140f7fe40, "unknown_140f7fe40", tpl_1402::<0x143c93198, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f7fe40
    Replacement::draft(0x140f80190, "unknown_140f80190", tpl_61019::<0x143c92bcc, 0x142489ee0, 0x143c933f0, 0x143c8d120, 0x1426467e8, 0x140f802f3> as *const ()), // sub_140f80190
    Replacement::draft(0x140f80300, "unknown_140f80300", tpl_61019::<0x143c92bcc, 0x142489ee0, 0x143c93358, 0x143c8f6d8, 0x1426467e8, 0x140f80463> as *const ()), // sub_140f80300
    Replacement::draft_throws(0x140f804a0, "unknown_140f804a0", tpl_3370::<0x1411e9f50, 0x140f824c0, 0x143cc7954, 0x1401fc320, 0x140f83b90, 0x143cc7f2c, 0x1420af120, 0x140215750, 0x1420a28e0, 0x140f81b30, 0x1401de410> as *const (), Throws::Discard), // sub_140f804a0
    Replacement::draft_throws(0x140f80670, "unknown_140f80670", tpl_3011::<0x1411e9f50, 0x140f83790, 0x143cc7f1c, 0x1420af120, 0x140215750, 0x143cc7c88, 0x140215610, 0x1420a28e0, 0x140f82100, 0x1401de410> as *const (), Throws::Discard), // sub_140f80670
    Replacement::draft_throws(0x140f80830, "unknown_140f80830", tpl_3011::<0x1411e9f50, 0x140f830c0, 0x143cc7f08, 0x1420af120, 0x140215750, 0x143cc7c84, 0x140215610, 0x1420a28e0, 0x140f823d0, 0x1401de410> as *const (), Throws::Discard), // sub_140f80830
    Replacement::draft(0x140f809f0, "unknown_140f809f0", tpl_61019::<0x143c92bcc, 0x142489ee0, 0x143c8dbd4, 0x143cc7b78, 0x1426467e8, 0x140f80b53> as *const ()), // sub_140f809f0
    Replacement::draft(0x140f80e80, "unknown_140f80e80", tpl_1113::<0x88, 0x10088, 0x142407c80, 0x140f815d0, 0x580f0, 0x141e21ea0> as *const ()), // sub_140f80e80
    Replacement::draft(0x140f80fa0, "unknown_140f80fa0", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x140f814d0, 0x580f0, 0x141e21d60> as *const ()), // sub_140f80fa0
    Replacement::draft_throws(0x140f81090, "unknown_140f81090", tpl_5922::<0x140f83680, 0x143cc7f18, 0x1420af120, 0x140215750, 0x140f81700, 0x143cc7c88, 0x140215610, 0x143c8d08c, 0x1420a28e0> as *const (), Throws::Discard), // sub_140f81090
    Replacement::draft_throws(0x140f81220, "unknown_140f81220", fn_140f81220 as *const (), Throws::Discard), // sub_140f81220
    Replacement::draft_throws(0x140f81700, "unknown_140f81700", tpl_1330::<0x8, 0x140f83a40, 0x143cc7f28, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140f81700
    Replacement::draft(0x140f81b30, "unknown_140f81b30", tpl_19300::<0x143cc7954, 0x5e308, 0x143cc7f2c, 0x20, 0x8, 0x10, 0x18, 0x141e22160, 0x140f82730> as *const ()), // sub_140f81b30
    Replacement::draft(0x140f81c30, "unknown_140f81c30", tpl_19300::<0x143c92bcc, 0x5e308, 0x143cc7f28, 0x20, 0x8, 0x10, 0x18, 0x141e22020, 0x140489dd0> as *const ()), // sub_140f81c30
    Replacement::draft(0x140f81d30, "unknown_140f81d30", tpl_1128::<0x142407c80, 0x143c94ef8, 0x142489ee0, 0x143c933f0, 0x143c93358, 0x141e21ea0> as *const ()), // sub_140f81d30
    Replacement::draft(0x140f81f50, "unknown_140f81f50", tpl_3213::<0x142407c80, 0x143c94b28, 0x142489ee0, 0x143c8dbd4, 0x141e21d60> as *const ()), // sub_140f81f50
    Replacement::draft(0x140f82100, "unknown_140f82100", tpl_19301::<0x143cc7f1c, 0x20, 0x8, 0x143cc7c88, 0x580e0, 0x142407c80, 0x580f0, 0x141e21a60> as *const ()), // sub_140f82100
    Replacement::draft_throws(0x140f821a0, "unknown_140f821a0", tpl_5925::<0x8, 0x20, 0x143cc7f18, 0x143cc7f28, 0x580e0, 0x143cc7c88, 0x143c8d08c, 0x141e21320> as *const (), Throws::Discard), // sub_140f821a0
    Replacement::draft(0x140f82370, "unknown_140f82370", tpl_4148::<0x143c8d0a4, 0x20, 0x8, 0x141e22480> as *const ()), // sub_140f82370
    Replacement::draft(0x140f823d0, "unknown_140f823d0", tpl_3028::<0x143cc7f08, 0x20, 0x8, 0x143cc7c84, 0x580e0, 0x141e22260> as *const ()), // sub_140f823d0
    Replacement::draft(0x140f82440, "unknown_140f82440", tpl_3188::<0x140f83af0, 0x14331f898, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140f82440
    Replacement::draft(0x140f824c0, "unknown_140f824c0", tpl_3188::<0x140f83c40, 0x14331f870, 0x140f82730, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140f824c0
    Replacement::draft(0x140f82540, "unknown_140f82540", tpl_4348::<0x18, 0x20, 0x140f82730> as *const ()), // sub_140f82540
    Replacement::draft(0x140f82730, "unknown_140f82730", fn_140f82730 as *const ()), // sub_140f82730
    Replacement::draft(0x140f82850, "unknown_140f82850", fn_140f82850 as *const ()), // sub_140f82850
    Replacement::draft(0x140f834c0, "unknown_140f834c0", fn_140f834c0 as *const ()), // sub_140f834c0
    Replacement::draft(0x140f83af0, "unknown_140f83af0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14331f898, 0x8, 0x20> as *const ()), // sub_140f83af0
    Replacement::draft(0x140f83c40, "unknown_140f83c40", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14331f870, 0x8, 0x20> as *const ()), // sub_140f83c40
    Replacement::draft(0x140f83ce0, "unknown_140f83ce0", tpl_19154::<0x1425371a0, 0x142533200, 0x1403d79a0, 0x1403c7e70, 0x1401de830> as *const ()), // sub_140f83ce0
    Replacement::draft(0x140f83e40, "unknown_140f83e40", fn_140f83e40 as *const ()), // sub_140f83e40
    Replacement::draft(0x140f83f40, "unknown_140f83f40", fn_140f83f40 as *const ()), // sub_140f83f40
    Replacement::draft(0x140f84590, "coregame_140f84590", fn_140f84590 as *const ()), // ecs::system::ecs::system::wrapper<coregame::time::updateWallClock>
    Replacement::draft(0x140f84840, "coregame_140f84840", fn_140f84840 as *const ()), // ecs::system::ecs::system::wrapper<coregame::time::updateRealWorldTime>
    Replacement::draft(0x140f84ac0, "coregame_140f84ac0", fn_140f84ac0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::time::updateScaling>
    Replacement::draft(0x140f84cd0, "coregame_140f84cd0", fn_140f84cd0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::time::updateDeltas>
    Replacement::draft(0x140f84f20, "coregame_140f84f20", tpl_19313::<0x143cc8320, 0x143cc8328, 0x1411e9f50, 0x140f85910, 0x140f85cf0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331fe26, 0x142646960, 0x143cc8348, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::time::updateInterpolation>
    Replacement::draft(0x140f85160, "coregame_140f85160", tpl_19313::<0x143cc834c, 0x143cc8350, 0x1411e9f50, 0x140f85a80, 0x140f85d70, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14331fa56, 0x142646960, 0x143cc8370, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::time::advanceVariableTime>
    Replacement::draft(0x140f853a0, "coregame_140f853a0", fn_140f853a0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::time::advanceFixedTime>
    Replacement::draft(0x140f85620, "coregame_140f85620", fn_140f85620 as *const ()), // sub_140f85620
    Replacement::draft(0x140f857a0, "coregame_140f857a0", fn_140f857a0 as *const ()), // sub_140f857a0
    Replacement::draft(0x140f85910, "coregame_140f85910", fn_140f85910 as *const ()), // sub_140f85910
    Replacement::draft(0x140f85a80, "coregame_140f85a80", tpl_19318::<0x143c904a0, 0x140215610, 0x143c900d8, 0x143c8d910, 0x143cc839c, 0x1420a28e0> as *const ()), // sub_140f85a80
    Replacement::draft(0x140f85bf0, "coregame_140f85bf0", fn_140f85bf0 as *const ()), // sub_140f85bf0
    Replacement::draft(0x140f85c30, "coregame_140f85c30", fn_140f85c30 as *const ()), // sub_140f85c30
    Replacement::draft(0x140f85c90, "coregame_140f85c90", fn_140f85c90 as *const ()), // sub_140f85c90
    Replacement::draft(0x140f85cf0, "coregame_140f85cf0", fn_140f85cf0 as *const ()), // sub_140f85cf0
    Replacement::draft(0x140f85d70, "coregame_140f85d70", fn_140f85d70 as *const ()), // sub_140f85d70
    Replacement::draft(0x140f85dd0, "coregame_140f85dd0", fn_140f85dd0 as *const ()), // sub_140f85dd0
    Replacement::draft(0x140f85e00, "coregame_140f85e00", fn_140f85e00 as *const ()), // sub_140f85e00
    Replacement::draft_throws(0x140f85e30, "coregame_140f85e30", fn_140f85e30 as *const (), Throws::Caught), // sub_140f85e30
    Replacement::draft(0x140f85fc0, "coregame_140f85fc0", fn_140f85fc0 as *const ()), // sub_140f85fc0
    Replacement::draft(0x140f86120, "coregame_140f86120", fn_140f86120 as *const ()), // sub_140f86120
    Replacement::draft(0x140f86250, "coregame_140f86250", tpl_4028::<0x30, 0x143c8d0a0, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140f862d3> as *const ()), // sub_140f86250
    Replacement::draft_throws(0x140f862e0, "coregame_140f862e0", fn_140f862e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_video::applyTimelineVideo>
    Replacement::draft(0x140f866a0, "coregame_140f866a0", tpl_5530::<0x143cc8680, 0x143cc8688, 0x1411e9f50, 0x143cc86c0, 0x140215610, 0x1420a28e0, 0x140f86c00, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143320496, 0x142646960, 0x143cc86a8, 0x143cc86b8, 0x143cc86bc, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::timeline_video::handleGameSeverStateChanged>
    Replacement::draft(0x140f86950, "coregame_140f86950", fn_140f86950 as *const ()), // sub_140f86950
    Replacement::draft(0x140f86b60, "coregame_140f86b60", fn_140f86b60 as *const ()), // sub_140f86b60
    Replacement::draft(0x140f86c20, "coregame_140f86c20", fn_140f86c20 as *const ()), // sub_140f86c20
    Replacement::draft_throws(0x140f86d50, "coregame_140f86d50", fn_140f86d50 as *const (), Throws::Caught), // sub_140f86d50
    Replacement::draft_throws(0x140f87010, "coregame_140f87010", tpl_5091::<0x143cc87ac, 0x143cc8888, 0x140f87480, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143320896, 0x142646960, 0x143cc88a8, 0x143cc88b8, 0x143cc88bc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_transform::applyTimelineTransform>
    Replacement::draft(0x140f87220, "coregame_140f87220", fn_140f87220 as *const ()), // sub_140f87220
    Replacement::draft_throws(0x140f87480, "coregame_140f87480", fn_140f87480 as *const (), Throws::Discard), // sub_140f87480
    Replacement::draft(0x140f876f0, "coregame_140f876f0", tpl_19100::<0x8, 0x20, 0x143cc88c4, 0x143c94b44, 0x143c94b30, 0x141e29140> as *const ()), // sub_140f876f0
    Replacement::draft(0x140f87910, "coregame_140f87910", tpl_4170::<0x141e27fc0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141e28e40, 0x142442de0> as *const ()), // sub_140f87910
    Replacement::draft(0x140f87ac0, "coregame_140f87ac0", tpl_61021::<0x30, 0x40, 0x48, 0x140f87b80, 0x8, 0x10, 0x18, 0x20, 0x28, 0x1426467e8, 0x140f87b7a> as *const ()), // sub_140f87ac0
    Replacement::draft(0x140f87cd0, "coregame_140f87cd0", tpl_1360::<0x30, 0x143cc893c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140f87d53> as *const ()), // sub_140f87cd0
    Replacement::draft_throws(0x140f87d60, "coregame_140f87d60", tpl_3456::<0x143cc88c8, 0x143cc88d0, 0x1411e9f50, 0x140f884b0, 0x140f88760, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433213d6, 0x142646960, 0x143cc88f0, 0x143cc88f8, 0x143cc88fc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_sound::applyTimelineSounds>
    Replacement::draft_throws(0x140f87fb0, "coregame_140f87fb0", tpl_6548::<0x143cc8900, 0x143cc8908, 0x1411e9f50, 0x140f88a10, 0x143cc8a30, 0x1420af120, 0x140215750, 0x1420a28e0, 0x140f887c0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143321956, 0x142646960, 0x143cc8928, 0x143cc8938, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_sound::updateTimelineAudioSync>
    Replacement::draft(0x140f882c0, "unknown_140f882c0", fn_140f882c0 as *const ()), // sub_140f882c0
    Replacement::draft_throws(0x140f884b0, "unknown_140f884b0", fn_140f884b0 as *const (), Throws::Discard), // sub_140f884b0
    Replacement::draft_throws(0x140f88760, "unknown_140f88760", tpl_5128::<0x143cc8a34, 0x20, 0x8, 0x141e2b450> as *const (), Throws::Discard), // sub_140f88760
    Replacement::draft(0x140f887c0, "unknown_140f887c0", tpl_4148::<0x143cc8a30, 0x20, 0x8, 0x141e2b1a0> as *const ()), // sub_140f887c0
    Replacement::draft(0x140f88c10, "unknown_140f88c10", tpl_4170::<0x141e2a1a0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141e2ad60, 0x142442de0> as *const ()), // sub_140f88c10
    Replacement::draft(0x140f88e50, "unknown_140f88e50", fn_140f88e50 as *const ()), // sub_140f88e50
    Replacement::draft(0x140f88e90, "unknown_140f88e90", fn_140f88e90 as *const ()), // sub_140f88e90
    Replacement::draft(0x140f88f50, "unknown_140f88f50", fn_140f88f50 as *const ()), // sub_140f88f50
    Replacement::draft_throws(0x140f89140, "coregame_140f89140", fn_140f89140 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_scripting::applyTimelineScripting>
    Replacement::draft(0x140f89480, "coregame_140f89480", tpl_1217::<0x143c96518, 0x20, 0x8, 0x141e2cef0> as *const ()), // sub_140f89480
    Replacement::draft_throws(0x140f894e0, "coregame_140f894e0", tpl_4193::<0x143cc8b58, 0x143cc8b60, 0x1411e9f50, 0x140f898d0, 0x140f89ac0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143321f16, 0x142646960, 0x143cc8b80, 0x143cc8b90, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_particle::applyTimelineParticles>
    Replacement::draft(0x140f89730, "unknown_140f89730", fn_140f89730 as *const ()), // sub_140f89730
    Replacement::draft_throws(0x140f898d0, "unknown_140f898d0", fn_140f898d0 as *const (), Throws::Discard), // sub_140f898d0
    Replacement::draft_throws(0x140f89ac0, "unknown_140f89ac0", fn_140f89ac0 as *const (), Throws::Discard), // sub_140f89ac0
    Replacement::draft(0x140f89d40, "unknown_140f89d40", tpl_4170::<0x141e2d600, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141e2def0, 0x142442de0> as *const ()), // sub_140f89d40
    Replacement::draft_throws(0x140f89ef0, "coregame_140f89ef0", fn_140f89ef0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_parameter_curve::applyTimelineParameterCurve>
    Replacement::draft(0x140f8a130, "coregame_140f8a130", fn_140f8a130 as *const ()), // sub_140f8a130
    Replacement::draft(0x140f8a380, "coregame_140f8a380", fn_140f8a380 as *const ()), // sub_140f8a380
    Replacement::draft(0x140f8a3f0, "coregame_140f8a3f0", fn_140f8a3f0 as *const ()), // sub_140f8a3f0
    Replacement::draft_throws(0x140f8a470, "coregame_140f8a470", fn_140f8a470 as *const (), Throws::Discard), // sub_140f8a470
    Replacement::draft_throws(0x140f8a6a0, "coregame_140f8a6a0", tpl_4147::<0x143c9e06c, 0x143cc8d44, 0x20, 0x143c90de0, 0x8, 0x580e0, 0x143c8ce80, 0x141e300c0> as *const (), Throws::Discard), // sub_140f8a6a0
    Replacement::draft(0x140f8a730, "coregame_140f8a730", tpl_4170::<0x141e2efe0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141e2fd10, 0x142442de0> as *const ()), // sub_140f8a730
    Replacement::draft(0x140f8a850, "coregame_140f8a850", fn_140f8a850 as *const ()), // sub_140f8a850
    Replacement::draft(0x140f8a960, "coregame_140f8a960", fn_140f8a960 as *const ()), // sub_140f8a960
    Replacement::draft(0x140f8aa80, "coregame_140f8aa80", fn_140f8aa80 as *const ()), // sub_140f8aa80
    Replacement::draft(0x140f8ac90, "coregame_140f8ac90", fn_140f8ac90 as *const ()), // sub_140f8ac90
    Replacement::draft(0x140f8ad00, "coregame_140f8ad00", fn_140f8ad00 as *const ()), // sub_140f8ad00
    Replacement::draft(0x140f8b470, "coregame_140f8b470", fn_140f8b470 as *const ()), // sub_140f8b470
    Replacement::draft_throws(0x140f8b4e0, "coregame_140f8b4e0", tpl_3367::<0x143cc8d54, 0x143cc8d58, 0x140f8f230, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143325c96, 0x142646960, 0x143cc8d78, 0x143cc8d80, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::updateGlobalTimeScale>
    Replacement::draft_throws(0x140f8b6f0, "coregame_140f8b6f0", fn_140f8b6f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::clearTimelineEvents>
    Replacement::draft(0x140f8b960, "coregame_140f8b960", tpl_19357::<0x143cc8db0, 0x143cc8db8, 0x1411e9f50, 0x140f93e90, 0x143c8d0b0, 0x1401fc320, 0x143cc8efc, 0x140215610, 0x1420a28e0, 0x140f93290, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143325996, 0x142646960, 0x143cc8dd8, 0x143cc8de8, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::timeline::moveRemovedTimelinesOwnership>
    Replacement::draft_throws(0x140f8bc50, "coregame_140f8bc50", fn_140f8bc50 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::sendTimelineEndEvents>
    Replacement::draft_throws(0x140f8bf30, "coregame_140f8bf30", tpl_1428::<0x143cc8e1c, 0x143cc8e20, 0x1411e9f50, 0x140f91ea0, 0x140f935c0, 0x140f92060, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433253f6, 0x142646960, 0x143cc8e40, 0x143cc8e44, 0x143cc8e46, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::updateStateMachine>
    Replacement::draft_throws(0x140f8c180, "coregame_140f8c180", fn_140f8c180 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::sendTimelineStartEvents>
    Replacement::draft_throws(0x140f8c430, "coregame_140f8c430", tpl_3654::<0x143cc8e7c, 0x143cc8e80, 0x1411e9f50, 0x140f922c0, 0x140f939e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143324e56, 0x142646960, 0x143cc8ea0, 0x143cc8ea8, 0x143cc8eaa, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::evaluateTimelineEvents>
    Replacement::draft(0x140f8c680, "coregame_140f8c680", fn_140f8c680 as *const ()), // ecs::system::ecs::system::wrapper<coregame::timeline::skippingUpdate>
    Replacement::draft(0x140f8c910, "coregame_140f8c910", fn_140f8c910 as *const ()), // ecs::system::ecs::system::wrapper<coregame::timeline::skippingFading>
    Replacement::draft(0x140f8cbd0, "coregame_140f8cbd0", tpl_13706::<0x8, 0x1401ff210> as *const ()), // sub_140f8cbd0
    Replacement::draft(0x140f8cc10, "coregame_140f8cc10", fn_140f8cc10 as *const ()), // sub_140f8cc10
    Replacement::draft(0x140f8cd60, "coregame_140f8cd60", fn_140f8cd60 as *const ()), // sub_140f8cd60
    Replacement::draft(0x140f8d1f0, "coregame_140f8d1f0", fn_140f8d1f0 as *const ()), // sub_140f8d1f0
    Replacement::draft(0x140f8d3f0, "coregame_140f8d3f0", tpl_1094::<0x143c8d0b0, 0x140f91590, 0x14020e8f0> as *const ()), // sub_140f8d3f0
    Replacement::draft(0x140f8d5f0, "coregame_140f8d5f0", tpl_1094::<0x143cc8d4c, 0x140f91710, 0x14020e8f0> as *const ()), // sub_140f8d5f0
    Replacement::draft(0x140f8d7f0, "coregame_140f8d7f0", tpl_1094::<0x143cc8fc0, 0x14068b330, 0x14020e8f0> as *const ()), // sub_140f8d7f0
    Replacement::draft(0x140f8d9f0, "coregame_140f8d9f0", tpl_1368::<0x143cc8d50, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f8d9f0
    Replacement::draft(0x140f8dd20, "coregame_140f8dd20", tpl_3077::<0x143cc8d48, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f8dd20
    Replacement::draft(0x140f8e050, "coregame_140f8e050", tpl_3406::<0x143c8d0a0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f8e050
    Replacement::draft(0x140f8e3a0, "coregame_140f8e3a0", tpl_3270::<0x143cc893c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f8e3a0
    Replacement::draft(0x140f8e6e0, "coregame_140f8e6e0", tpl_3278::<0x143c8d024, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f8e6e0
    Replacement::draft(0x140f8ea20, "coregame_140f8ea20", tpl_5819::<0x143cc8fc4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140f8ea20
    Replacement::draft(0x140f8ed70, "coregame_140f8ed70", tpl_1094::<0x143c932ac, 0x140f918a0, 0x14020e8f0> as *const ()), // sub_140f8ed70
    Replacement::draft(0x140f8ef70, "coregame_140f8ef70", tpl_1094::<0x143cc8fc8, 0x140f91b10, 0x14020e8f0> as *const ()), // sub_140f8ef70
    Replacement::draft(0x140f8f170, "coregame_140f8f170", tpl_19365::<0x8, 0x140f8ed70, 0x141e319e0> as *const ()), // sub_140f8f170
    Replacement::draft(0x140f8f1d0, "coregame_140f8f1d0", tpl_19365::<0x8, 0x140f8ef70, 0x141e319e0> as *const ()), // sub_140f8f1d0
    Replacement::draft_throws(0x140f8f230, "coregame_140f8f230", tpl_3011::<0x1411e9f50, 0x140f95310, 0x143cc93c4, 0x1420af120, 0x140215750, 0x143cc83a4, 0x140215610, 0x1420a28e0, 0x140f93210, 0x1401de410> as *const (), Throws::Discard), // sub_140f8f230
    Replacement::draft(0x140f8f3f0, "coregame_140f8f3f0", tpl_19366::<0x1401e7c10, 0x140f90a40, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_140f8f3f0
    Replacement::draft_throws(0x140f8f600, "coregame_140f8f600", tpl_19367::<0x143cc8fcc, 0x143cc8fd0, 0x140f924b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143324c76, 0x142646960, 0x143cc8ff0, 0x143cc9010, 0x143cc9020, 0x143cc9024, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::TimelineComponent>>
    Replacement::draft(0x140f8f830, "coregame_140f8f830", fn_140f8f830 as *const ()), // sub_140f8f830
    Replacement::draft_throws(0x140f8fa40, "coregame_140f8fa40", tpl_6486::<0x143cc9028, 0x143cc9030, 0x140f92670, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143324646, 0x142646960, 0x143cc9050, 0x143cc9070, 0x143cc9080, 0x143cc9088, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class contentcore::TimelineCompone
    Replacement::draft_throws(0x140f8fc70, "coregame_140f8fc70", tpl_1963::<0x143cc908c, 0x143cc9090, 0x1411e9f50, 0x140f95b20, 0x143cc9638, 0x1420af120, 0x1420a28e0, 0x140f94ae0, 0x140f93af0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143324836, 0x142646960, 0x143cc90a0, 0x143cc90a8, 0x143cc90ac, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::streamOut>
    Replacement::draft_throws(0x140f8ff50, "coregame_140f8ff50", fn_140f8ff50 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::wedStreamIn>
    Replacement::draft_throws(0x140f90280, "coregame_140f90280", tpl_19370::<0x143cc90d8, 0x143cc90e0, 0x1411e9f50, 0x140f958a0, 0x143cc9630, 0x1420af120, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x140f94df0, 0x140f93d50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433243c6, 0x142646960, 0x143cc9100, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline::nwedStreamIn>
    Replacement::draft(0x140f90580, "unknown_140f90580", fn_140f90580 as *const ()), // sub_140f90580
    Replacement::draft_throws(0x140f90b40, "unknown_140f90b40", fn_140f90b40 as *const (), Throws::Discard), // sub_140f90b40
    Replacement::draft(0x140f90bd0, "unknown_140f90bd0", fn_140f90bd0 as *const ()), // sub_140f90bd0
    Replacement::draft(0x140f90c40, "unknown_140f90c40", fn_140f90c40 as *const ()), // sub_140f90c40
    Replacement::draft(0x140f90dd0, "unknown_140f90dd0", fn_140f90dd0 as *const ()), // sub_140f90dd0
    Replacement::draft(0x140f91150, "unknown_140f91150", fn_140f91150 as *const ()), // sub_140f91150
    Replacement::draft(0x140f91590, "unknown_140f91590", tpl_19376::<0x1401e6d50, 0x140f931e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1401ff270> as *const ()), // sub_140f91590
    Replacement::draft(0x140f91710, "unknown_140f91710", tpl_4469::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140f8ad00> as *const ()), // sub_140f91710
    Replacement::draft(0x140f918a0, "unknown_140f918a0", fn_140f918a0 as *const ()), // sub_140f918a0
    Replacement::draft(0x140f91b10, "unknown_140f91b10", fn_140f91b10 as *const ()), // sub_140f91b10
    Replacement::draft(0x140f91d40, "unknown_140f91d40", tpl_62821::<0x143c8d910, 0x143c8ceec, 0x140f92890, 0x141e358e0> as *const ()), // sub_140f91d40
    Replacement::draft_throws(0x140f91ea0, "unknown_140f91ea0", fn_140f91ea0 as *const (), Throws::Discard), // sub_140f91ea0
    Replacement::draft(0x140f92060, "unknown_140f92060", fn_140f92060 as *const ()), // sub_140f92060
    Replacement::draft(0x140f92170, "unknown_140f92170", tpl_62759::<0x88, 0x10088, 0x143c8ceec, 0x580e0, 0x140f92890, 0x141e35860> as *const ()), // sub_140f92170
    Replacement::draft_throws(0x140f922c0, "unknown_140f922c0", fn_140f922c0 as *const (), Throws::Discard), // sub_140f922c0
    Replacement::draft_throws(0x140f924b0, "unknown_140f924b0", tpl_3011::<0x1411e9f50, 0x140f95da0, 0x143cc9640, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140f94a00, 0x1401de410> as *const (), Throws::Discard), // sub_140f924b0
    Replacement::draft_throws(0x140f92670, "unknown_140f92670", tpl_3011::<0x1411e9f50, 0x140f95cb0, 0x143cc963c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140f94a70, 0x1401de410> as *const (), Throws::Discard), // sub_140f92670
    Replacement::draft(0x140f92830, "unknown_140f92830", fn_140f92830 as *const ()), // sub_140f92830
    Replacement::draft_throws(0x140f929c0, "unknown_140f929c0", tpl_1331::<0x8, 0x140f95160, 0x143cc93c0, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_140f929c0
    Replacement::draft_throws(0x140f92a70, "unknown_140f92a70", fn_140f92a70 as *const (), Throws::Discard), // sub_140f92a70
    Replacement::draft(0x140f930a0, "unknown_140f930a0", fn_140f930a0 as *const ()), // sub_140f930a0
    Replacement::draft(0x140f931e0, "unknown_140f931e0", tpl_1955::<0xc0, 0x1401ff210> as *const ()), // sub_140f931e0
    Replacement::draft(0x140f93210, "unknown_140f93210", tpl_3028::<0x143cc93c4, 0x20, 0x8, 0x143cc83a4, 0x580e0, 0x141e35a80> as *const ()), // sub_140f93210
    Replacement::draft(0x140f93290, "unknown_140f93290", tpl_3172::<0x143c8d0b0, 0x5e308, 0x143cc8efc, 0x580e0, 0x10, 0x8, 0x18, 0x141e361e0, 0x140f942c0> as *const ()), // sub_140f93290
    Replacement::draft(0x140f93340, "unknown_140f93340", tpl_62838::<0x143c8d910, 0x143c8ceec, 0x143c8d160, 0x142489ee0, 0x143c8d0b0, 0x143c8d0a0, 0x141e358e0> as *const ()), // sub_140f93340
    Replacement::draft(0x140f935c0, "unknown_140f935c0", fn_140f935c0 as *const ()), // sub_140f935c0
    Replacement::draft(0x140f93790, "unknown_140f93790", tpl_62765::<0x143c8ceec, 0x143c8d160, 0x142489ee0, 0x143c8d0b0, 0x143c8d0a0, 0x141e35860> as *const ()), // sub_140f93790
    Replacement::draft(0x140f93af0, "unknown_140f93af0", tpl_1127::<0x142407c80, 0x143cc8fc4, 0x142489ee0, 0x141e31b20> as *const ()), // sub_140f93af0
    Replacement::draft_throws(0x140f93c00, "unknown_140f93c00", fn_140f93c00 as *const (), Throws::Caught), // sub_140f93c00
    Replacement::draft_throws(0x140f93d50, "unknown_140f93d50", fn_140f93d50 as *const (), Throws::Caught), // sub_140f93d50
    Replacement::draft(0x140f93e90, "unknown_140f93e90", tpl_3188::<0x140f95270, 0x143322720, 0x140f942c0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140f93e90
    Replacement::draft(0x140f94150, "unknown_140f94150", fn_140f94150 as *const ()), // sub_140f94150
    Replacement::draft(0x140f942c0, "unknown_140f942c0", fn_140f942c0 as *const ()), // sub_140f942c0
    Replacement::draft(0x140f94420, "unknown_140f94420", fn_140f94420 as *const ()), // sub_140f94420
    Replacement::draft(0x140f944e0, "unknown_140f944e0", fn_140f944e0 as *const ()), // sub_140f944e0
    Replacement::draft(0x140f945a0, "unknown_140f945a0", fn_140f945a0 as *const ()), // sub_140f945a0
    Replacement::draft(0x140f94a00, "unknown_140f94a00", tpl_3028::<0x143cc9640, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x140f8f3f0> as *const ()), // sub_140f94a00
    Replacement::draft(0x140f94a70, "unknown_140f94a70", tpl_3028::<0x143cc963c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x140f8f830> as *const ()), // sub_140f94a70
    Replacement::draft(0x140f94ae0, "unknown_140f94ae0", tpl_1131::<0x142407c80, 0x143cc8fc4, 0x142489ee0, 0x141e31b20> as *const ()), // sub_140f94ae0
    Replacement::draft_throws(0x140f94c40, "unknown_140f94c40", fn_140f94c40 as *const (), Throws::Caught), // sub_140f94c40
    Replacement::draft_throws(0x140f94df0, "unknown_140f94df0", fn_140f94df0 as *const (), Throws::Caught), // sub_140f94df0
    Replacement::draft(0x140f95270, "unknown_140f95270", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143322720, 0x8, 0x20> as *const ()), // sub_140f95270
    Replacement::draft(0x140f95580, "unknown_140f95580", fn_140f95580 as *const ()), // sub_140f95580
    Replacement::draft(0x140f95730, "unknown_140f95730", fn_140f95730 as *const ()), // sub_140f95730
    Replacement::draft_throws(0x140f95760, "unknown_140f95760", fn_140f95760 as *const (), Throws::Discard), // sub_140f95760
    Replacement::draft(0x140f95b20, "unknown_140f95b20", fn_140f95b20 as *const ()), // sub_140f95b20
    Replacement::draft(0x140f95e90, "unknown_140f95e90", fn_140f95e90 as *const ()), // sub_140f95e90
    Replacement::draft(0x140f96030, "unknown_140f96030", fn_140f96030 as *const ()), // sub_140f96030
    Replacement::draft(0x140f96120, "unknown_140f96120", fn_140f96120 as *const ()), // sub_140f96120
    Replacement::draft_throws(0x140f96330, "unknown_140f96330", fn_140f96330 as *const (), Throws::Discard), // sub_140f96330
    Replacement::draft(0x140f963d0, "unknown_140f963d0", fn_140f963d0 as *const ()), // sub_140f963d0
    Replacement::draft(0x140f96710, "unknown_140f96710", tpl_19401::<0x8, 0x140f963d0> as *const ()), // sub_140f96710
    Replacement::draft(0x140f96730, "unknown_140f96730", tpl_19401::<0x8, 0x1402c0d50> as *const ()), // sub_140f96730
    Replacement::draft_throws(0x140f96a20, "unknown_140f96a20", fn_140f96a20 as *const (), Throws::Discard), // sub_140f96a20
    Replacement::draft(0x140f96b80, "unknown_140f96b80", fn_140f96b80 as *const ()), // sub_140f96b80
    Replacement::draft_throws(0x140f96ea0, "unknown_140f96ea0", fn_140f96ea0 as *const (), Throws::Discard), // sub_140f96ea0
    Replacement::draft_throws(0x140f97020, "unknown_140f97020", fn_140f97020 as *const (), Throws::Discard), // sub_140f97020
    Replacement::draft(0x140f97180, "unknown_140f97180", tpl_60443::<0x143c9f930, 0x142489ee0, 0xc03, 0x8, 0x10, 0x1426467e8, 0x140f9723a> as *const ()), // sub_140f97180
    Replacement::draft_throws(0x140f97240, "coregame_140f97240", fn_140f97240 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_gfxgraph::applyTimelineGfxGraph>
    Replacement::draft_throws(0x140f974e0, "coregame_140f974e0", fn_140f974e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_gfxgraph_variable::update>
    Replacement::draft(0x140f977d0, "coregame_140f977d0", fn_140f977d0 as *const ()), // sub_140f977d0
    Replacement::draft(0x140f979a0, "coregame_140f979a0", fn_140f979a0 as *const ()), // sub_140f979a0
    Replacement::draft_throws(0x140f97a00, "coregame_140f97a00", tpl_3146::<0x143c8a8b0, 0x143c8a8bc, 0x10, 0x1420a25b0, 0x141834a78, 0x143c90de0, 0x143c8a8b8, 0x143c8a8c0, 0x141834a0c> as *const (), Throws::Discard), // sub_140f97a00
    Replacement::draft_throws(0x140f97ae0, "coregame_140f97ae0", tpl_1330::<0x8, 0x140f97ca0, 0x143cc9a60, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140f97ae0
    Replacement::draft(0x140f97bc0, "coregame_140f97bc0", tpl_1217::<0x143cc9a60, 0x20, 0x8, 0x141e390c0> as *const ()), // sub_140f97bc0
    Replacement::draft(0x140f97c20, "coregame_140f97c20", tpl_4431::<0x143cc9a60, 0x20, 0x143c90de0, 0x8, 0x580e0, 0x143cc9a20, 0x141e389f0> as *const ()), // sub_140f97c20
    Replacement::draft(0x140f97d50, "coregame_140f97d50", tpl_4170::<0x141e37ac0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141e38770, 0x142442de0> as *const ()), // sub_140f97d50
    Replacement::draft(0x140f97e70, "coregame_140f97e70", fn_140f97e70 as *const ()), // sub_140f97e70
    Replacement::draft_throws(0x140f97f90, "coregame_140f97f90", tpl_3405::<0x143cc9aa8, 0x143cc9ab0, 0x1411e9f50, 0x140f98b80, 0x143cc9b68, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x140f98810, 0x140f98500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143326d66, 0x142646960, 0x143cc9ad0, 0x143cc9ae0, 0x143cc9ae2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_early_exit::sendEarlyExitEvents>
    Replacement::draft_throws(0x140f982b0, "coregame_140f982b0", tpl_6176::<0x143cc9ae4, 0x143cc9ae8, 0x1411e9f50, 0x140f985a0, 0x140f98a10, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143326b86, 0x142646960, 0x143cc9b08, 0x143cc9b18, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_early_exit::sendEarlyExitStartEvents>
    Replacement::draft(0x140f98500, "coregame_140f98500", tpl_77679::<0x88, 0x10088, 0x143c8ceec, 0x580e0, 0x140f98710, 0x141e3a170> as *const ()), // sub_140f98500
    Replacement::draft_throws(0x140f985a0, "coregame_140f985a0", tpl_19411::<0x140f98a90, 0x143cc9b64, 0x1420af120, 0x140215750, 0x143c90de0, 0x140215610, 0x143c8ceec, 0x1420a28e0> as *const (), Throws::Discard), // sub_140f985a0
    Replacement::draft(0x140f98810, "coregame_140f98810", fn_140f98810 as *const ()), // sub_140f98810
    Replacement::draft(0x140f98a10, "coregame_140f98a10", tpl_4431::<0x143cc9b64, 0x20, 0x143c90de0, 0x8, 0x580e0, 0x143c8ceec, 0x141e39f70> as *const ()), // sub_140f98a10
    Replacement::draft_throws(0x140f98d90, "coregame_140f98d90", tpl_19412::<0x143cc9be8, 0x143cc9bf0, 0x1411e9f50, 0x140f991d0, 0x140f99400, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143327116, 0x142646960, 0x143cc9c10, 0x143cc9c20, 0x143cc9c22, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_dialogue::applyTimelineDialogue>
    Replacement::draft(0x140f98fe0, "coregame_140f98fe0", fn_140f98fe0 as *const ()), // sub_140f98fe0
    Replacement::draft_throws(0x140f991d0, "coregame_140f991d0", fn_140f991d0 as *const (), Throws::Discard), // sub_140f991d0
    Replacement::draft_throws(0x140f99400, "coregame_140f99400", fn_140f99400 as *const (), Throws::Discard), // sub_140f99400
    Replacement::draft(0x140f99590, "coregame_140f99590", tpl_4170::<0x141e3a520, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141e3aee0, 0x142442de0> as *const ()), // sub_140f99590
    Replacement::draft(0x140f99740, "coregame_140f99740", fn_140f99740 as *const ()), // sub_140f99740
    Replacement::draft(0x140f99750, "coregame_140f99750", fn_140f99750 as *const ()), // sub_140f99750
    Replacement::draft_throws(0x140f999e0, "coregame_140f999e0", tpl_1438::<0x143cc9d10, 0x143cc9d18, 0x1411e9f50, 0x140f9d0d0, 0x143cc9f74, 0x1420af120, 0x1420a28e0, 0x140f9c2d0, 0x140f9afa0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143329356, 0x142646960, 0x143cc9d38, 0x143cc9d40, 0x143cc9d42, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timelinecamera::clean_up::system>
    Replacement::draft_throws(0x140f99cd0, "coregame_140f99cd0", tpl_1296::<0x143cc9d44, 0x143cc9d48, 0x1411e9f50, 0x140f9b0a0, 0x140f9c420, 0x140f9b260, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143328b96, 0x142646960, 0x143cc9d68, 0x143cc9d78, 0x143cc9d80, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timelinecamera::handle_timeline_actions::system>
    Replacement::draft_throws(0x140f99f30, "coregame_140f99f30", tpl_1290::<0x143cc9d84, 0x143cc9d88, 0x1411e9f50, 0x140f9b5f0, 0x140f9c670, 0x140f9b840, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143328376, 0x142646960, 0x143cc9da8, 0x143cc9db8, 0x143cc9dbc, 0x143cc9dbe, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timelinecamera::update_active_events::system>
    Replacement::draft_throws(0x140f9a190, "coregame_140f9a190", tpl_19267::<0x143cc9dc0, 0x143cc9dc8, 0x140f9ac50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143327f96, 0x142646960, 0x143cc9de8, 0x143cc9df8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timelinecamera::update_cameras::system>
    Replacement::draft(0x140f9a3a0, "coregame_140f9a3a0", tpl_1094::<0x143c997a8, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_140f9a3a0
    Replacement::draft(0x140f9a5a0, "coregame_140f9a5a0", fn_140f9a5a0 as *const ()), // sub_140f9a5a0
    Replacement::draft(0x140f9a920, "coregame_140f9a920", fn_140f9a920 as *const ()), // sub_140f9a920
    Replacement::draft_throws(0x140f9ac50, "coregame_140f9ac50", tpl_5553::<0x1411e9f50, 0x140f9cba0, 0x143cc9f68, 0x1420af120, 0x14036fad0, 0x14036e720, 0x143c8d910, 0x140215610, 0x143c900d8, 0x1420a28e0, 0x140f9c8b0, 0x140f9bbb0> as *const (), Throws::Discard), // sub_140f9ac50
    Replacement::draft(0x140f9afa0, "coregame_140f9afa0", fn_140f9afa0 as *const ()), // sub_140f9afa0
    Replacement::draft_throws(0x140f9b0a0, "coregame_140f9b0a0", fn_140f9b0a0 as *const (), Throws::Discard), // sub_140f9b0a0
    Replacement::draft_throws(0x140f9b260, "coregame_140f9b260", tpl_2993::<0x140f9b390, 0x140f9c0a0, 0x140f9b4a0> as *const (), Throws::Discard), // sub_140f9b260
    Replacement::draft(0x140f9b390, "coregame_140f9b390", tpl_3101::<0x8, 0x20, 0x143cc9f28, 0x143c8d910, 0x143c94b44, 0x143c94b30, 0x580e0, 0x143cc9dfc, 0x143c90de0> as *const ()), // sub_140f9b390
    Replacement::draft_throws(0x140f9b4a0, "coregame_140f9b4a0", fn_140f9b4a0 as *const (), Throws::Discard), // sub_140f9b4a0
    Replacement::draft_throws(0x140f9b5f0, "coregame_140f9b5f0", fn_140f9b5f0 as *const (), Throws::Discard), // sub_140f9b5f0
    Replacement::draft(0x140f9b840, "coregame_140f9b840", fn_140f9b840 as *const ()), // sub_140f9b840
    Replacement::draft(0x140f9b900, "coregame_140f9b900", fn_140f9b900 as *const ()), // sub_140f9b900
    Replacement::draft(0x140f9ba50, "coregame_140f9ba50", fn_140f9ba50 as *const ()), // sub_140f9ba50
    Replacement::draft(0x140f9bbb0, "coregame_140f9bbb0", fn_140f9bbb0 as *const ()), // sub_140f9bbb0
    Replacement::draft(0x140f9bd00, "coregame_140f9bd00", tpl_5199::<0x18, 0x20, 0x30, 0x8, 0x10, 0x28, 0x38, 0x141e3d8f0> as *const ()), // sub_140f9bd00
    Replacement::draft_throws(0x140f9c1f0, "coregame_140f9c1f0", tpl_1330::<0x8, 0x140f9cdf0, 0x143cc9f28, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140f9c1f0
    Replacement::draft(0x140f9c2d0, "coregame_140f9c2d0", fn_140f9c2d0 as *const ()), // sub_140f9c2d0
    Replacement::draft_throws(0x140f9c420, "coregame_140f9c420", fn_140f9c420 as *const (), Throws::Discard), // sub_140f9c420
    Replacement::draft(0x140f9c670, "coregame_140f9c670", fn_140f9c670 as *const ()), // sub_140f9c670
    Replacement::draft(0x140f9c8b0, "coregame_140f9c8b0", fn_140f9c8b0 as *const ()), // sub_140f9c8b0
    Replacement::draft(0x140f9cdf0, "coregame_140f9cdf0", fn_140f9cdf0 as *const ()), // sub_140f9cdf0
    Replacement::draft(0x140f9d180, "coregame_140f9d180", tpl_4170::<0x1402c0d50, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_140f9d180
    Replacement::draft(0x140f9d570, "coregame_140f9d570", fn_140f9d570 as *const ()), // sub_140f9d570
    Replacement::draft(0x140f9d600, "coregame_140f9d600", tpl_3045::<0x30, 0x143cca0f8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140f9d682> as *const ()), // sub_140f9d600
    Replacement::draft(0x140f9d690, "coregame_140f9d690", tpl_3045::<0x30, 0x143cca0fc, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140f9d712> as *const ()), // sub_140f9d690
    Replacement::draft(0x140f9d720, "coregame_140f9d720", fn_140f9d720 as *const ()), // sub_140f9d720
    Replacement::draft(0x140f9d970, "coregame_140f9d970", tpl_3942::<0x30, 0x143cca0f8, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x140f9d9c0> as *const ()), // sub_140f9d970
    Replacement::draft(0x140f9d9d0, "coregame_140f9d9d0", tpl_60953::<0x1426467e8, 0x143cca1f8, 0x142489ee0, 0x140f9da4c> as *const ()), // sub_140f9d9d0
    Replacement::draft_throws(0x140f9db80, "coregame_140f9db80", tpl_1584::<0x143cca100, 0x143cca108, 0x1411e9f50, 0x140f9ea60, 0x140f9f320, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143329ef6, 0x142646960, 0x143cca128, 0x143cca130, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_animation_clip::update>
    Replacement::draft_throws(0x140f9ddd0, "coregame_140f9ddd0", tpl_3653::<0x143cca134, 0x143cca138, 0x1411e9f50, 0x140f9ec70, 0x140f9f3f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143329ba6, 0x142646960, 0x143cca158, 0x143cca160, 0x143cca164, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_animated_movement::update>
    Replacement::draft_throws(0x140f9e020, "coregame_140f9e020", tpl_6046::<0x143cca168, 0x143cca170, 0x1411e9f50, 0x140f9ee50, 0x140f9f480, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143329a06, 0x142646960, 0x143cca190, 0x143cca198, 0x143cca19c, 0x143cca19e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_animation_parameter::update>
    Replacement::draft_throws(0x140f9e280, "coregame_140f9e280", tpl_1533::<0x143cca1a0, 0x143cca1a8, 0x1411e9f50, 0x140f9efc0, 0x140f9f500, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433295e6, 0x142646960, 0x143cca1c8, 0x143cca1d0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::timeline_animation_source::update>
    Replacement::draft(0x140f9e4d0, "unknown_140f9e4d0", fn_140f9e4d0 as *const ()), // sub_140f9e4d0
    Replacement::draft(0x140f9e6d0, "unknown_140f9e6d0", tpl_19431::<0x141e42ef0, 0x1425371a0, 0x142533200> as *const ()), // sub_140f9e6d0
    Replacement::draft(0x140f9e860, "unknown_140f9e860", tpl_19431::<0x141e43010, 0x1425371a0, 0x142533200> as *const ()), // sub_140f9e860
    Replacement::draft(0x140f9e9f0, "unknown_140f9e9f0", fn_140f9e9f0 as *const ()), // sub_140f9e9f0
    Replacement::draft_throws(0x140f9ea60, "unknown_140f9ea60", fn_140f9ea60 as *const (), Throws::Discard), // sub_140f9ea60
    Replacement::draft_throws(0x140f9ec70, "unknown_140f9ec70", fn_140f9ec70 as *const (), Throws::Discard), // sub_140f9ec70
    Replacement::draft_throws(0x140f9ee50, "unknown_140f9ee50", tpl_19411::<0x140f9fd00, 0x143cca3bc, 0x1420af120, 0x140215750, 0x143c90de0, 0x140215610, 0x143cca1f4, 0x1420a28e0> as *const (), Throws::Discard), // sub_140f9ee50
    Replacement::draft_throws(0x140f9efc0, "unknown_140f9efc0", tpl_19411::<0x140f9fb40, 0x143cca3b8, 0x1420af120, 0x140215750, 0x143c90de0, 0x140215610, 0x143c90e64, 0x1420a28e0> as *const (), Throws::Discard), // sub_140f9efc0
    Replacement::draft(0x140fa0450, "unknown_140fa0450", fn_140fa0450 as *const ()), // sub_140fa0450
    Replacement::draft(0x140fa0930, "unknown_140fa0930", fn_140fa0930 as *const ()), // sub_140fa0930
    Replacement::draft(0x140fa0980, "unknown_140fa0980", fn_140fa0980 as *const ()), // sub_140fa0980
    Replacement::draft(0x140fa0a90, "unknown_140fa0a90", fn_140fa0a90 as *const ()), // sub_140fa0a90
    Replacement::draft(0x140fa0bb0, "unknown_140fa0bb0", fn_140fa0bb0 as *const ()), // sub_140fa0bb0
    Replacement::draft_throws(0x140fa0c50, "unknown_140fa0c50", tpl_8839::<0x143880318, 0x143880320, 0x1401e9fa0, 0x328, 0x338, 0x28, 0x40, 0x369, 0x1401e9e70> as *const (), Throws::Discard), // sub_140fa0c50
    Replacement::draft(0x140fa0cd0, "unknown_140fa0cd0", tpl_19442::<0x143880320, 0x14243b0b0, 0x338, 0x28, 0x40, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140fa0cd0
    Replacement::draft_throws(0x140fa0d40, "unknown_140fa0d40", tpl_8841::<0x143880318, 0x143880320, 0x1401e9b60, 0x328, 0x338, 0x120, 0x28, 0x369, 0x1401e9a20> as *const (), Throws::Discard), // sub_140fa0d40
    Replacement::draft_throws(0x140fa0dd0, "unknown_140fa0dd0", fn_140fa0dd0 as *const (), Throws::Caught), // sub_140fa0dd0
    Replacement::draft_throws(0x140fa0e90, "unknown_140fa0e90", fn_140fa0e90 as *const (), Throws::Caught), // sub_140fa0e90
    Replacement::draft_throws(0x140fa0f50, "unknown_140fa0f50", fn_140fa0f50 as *const (), Throws::Caught), // sub_140fa0f50
    Replacement::draft(0x140fa1010, "unknown_140fa1010", fn_140fa1010 as *const ()), // sub_140fa1010
    Replacement::draft(0x140fa1080, "unknown_140fa1080", tpl_9561::<0x143880318, 0x143880320, 0x1401e9970, 0x338, 0x28, 0xe0, 0x1401e9860> as *const ()), // sub_140fa1080
    Replacement::draft(0x140fa10f0, "unknown_140fa10f0", tpl_19442::<0x143880320, 0x14243b0b0, 0x338, 0x28, 0xe0, 0x8, 0xfffffffffffffff8, 0xffffffffffffffff, 0x328> as *const ()), // sub_140fa10f0
    Replacement::draft_throws(0x140fa1160, "unknown_140fa1160", tpl_8848::<0x143880318, 0x143880320, 0x1401e9770, 0x338, 0x28, 0x120, 0x1401e9590> as *const (), Throws::Discard), // sub_140fa1160
    Replacement::draft_throws(0x140fa11d0, "unknown_140fa11d0", tpl_16042::<0x143880320, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x28, 0x14332b290, 0x1, 0x14020f120> as *const (), Throws::Discard), // sub_140fa11d0
    Replacement::draft(0x140fa1290, "unknown_140fa1290", tpl_1171::<0x58, 0x60, 0x143cc5590, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140fa1290
    Replacement::draft(0x140fa1370, "unknown_140fa1370", tpl_1095::<0x143cc5590, 0x14020e8f0> as *const ()), // sub_140fa1370
    Replacement::draft(0x140fa1550, "unknown_140fa1550", fn_140fa1550 as *const ()), // sub_140fa1550
    Replacement::draft(0x140fa15e0, "anonymous_namespace_140fa15e0", tpl_5783::<0x143cca5fc, 0x143cca600, 0x1411e9f50, 0x140fa2650, 0x143c95490, 0x1401fc320, 0x143c9f9c0, 0x140215610, 0x1420a28e0, 0x140fa2420, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332b856, 0x142646960, 0x143cca620, 0x143cca628, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::entityTagsRemoved>
    Replacement::draft_throws(0x140fa18d0, "anonymous_namespace_140fa18d0", tpl_19446::<0x143cca62c, 0x143cca630, 0x1411e9f50, 0x140fa2ec0, 0x14384e888, 0x1420af120, 0x143c9f9c0, 0x140215610, 0x1420a28e0, 0x140fa24d0, 0x140fa22b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332b6a6, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::initTags>
    Replacement::draft(0x140fa1bc0, "unknown_140fa1bc0", tpl_1095::<0x14384e83c, 0x14020e8f0> as *const ()), // sub_140fa1bc0
    Replacement::draft(0x140fa1da0, "unknown_140fa1da0", tpl_1094::<0x143c95490, 0x140fa2130, 0x14020e8f0> as *const ()), // sub_140fa1da0
    Replacement::draft(0x140fa1fd0, "unknown_140fa1fd0", tpl_19447::<0x140fa3050> as *const ()), // sub_140fa1fd0
    Replacement::draft(0x140fa2130, "unknown_140fa2130", tpl_3092::<0x1401e6d50, 0x140fa23f0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14037e1e0> as *const ()), // sub_140fa2130
    Replacement::draft(0x140fa22b0, "unknown_140fa22b0", tpl_4168::<0x143c9f9c0, 0x142407c80, 0x143c95490, 0x142489ee0, 0x141e497b0> as *const ()), // sub_140fa22b0
    Replacement::draft(0x140fa23f0, "unknown_140fa23f0", tpl_1955::<0x30, 0x14037e270> as *const ()), // sub_140fa23f0
    Replacement::draft(0x140fa2420, "unknown_140fa2420", tpl_3172::<0x143c95490, 0x5e308, 0x143c9f9c0, 0x580e0, 0x10, 0x8, 0x18, 0x141e493a0, 0x140fa2a70> as *const ()), // sub_140fa2420
    Replacement::draft(0x140fa24d0, "unknown_140fa24d0", tpl_4169::<0x143c9f9c0, 0x142407c80, 0x143c95490, 0x142489ee0, 0x141e497b0> as *const ()), // sub_140fa24d0
    Replacement::draft(0x140fa2650, "unknown_140fa2650", tpl_3188::<0x140fa2fb0, 0x14332b8f8, 0x140fa2a70, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fa2650
    Replacement::draft(0x140fa2910, "unknown_140fa2910", fn_140fa2910 as *const ()), // sub_140fa2910
    Replacement::draft(0x140fa2a70, "unknown_140fa2a70", tpl_19450::<0x1401de830, 0x14037e270, 0x1425371a0, 0x142533200> as *const ()), // sub_140fa2a70
    Replacement::draft(0x140fa2c80, "unknown_140fa2c80", fn_140fa2c80 as *const ()), // sub_140fa2c80
    Replacement::draft(0x140fa2fb0, "unknown_140fa2fb0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14332b8f8, 0x8, 0x20> as *const ()), // sub_140fa2fb0
    Replacement::draft(0x140fa3050, "unknown_140fa3050", tpl_381::<0x140fa31b0, 0x140fa3430> as *const ()), // sub_140fa3050
    Replacement::draft(0x140fa31b0, "unknown_140fa31b0", fn_140fa31b0 as *const ()), // sub_140fa31b0
    Replacement::draft(0x140fa3430, "unknown_140fa3430", fn_140fa3430 as *const ()), // sub_140fa3430
    Replacement::draft(0x140fa3760, "unknown_140fa3760", fn_140fa3760 as *const ()), // sub_140fa3760
    Replacement::draft(0x140fa3a00, "coregame_140fa3a00", tpl_15235::<0x14332bab0, 0x20, 0x141834d30> as *const ()), // coregame::SubtitleInfo::coregame::SubtitleInfo::vfunc_0
    Replacement::draft(0x140fa3a40, "coregame_140fa3a40", fn_140fa3a40 as *const ()), // sub_140fa3a40
    Replacement::draft(0x140fa3ac0, "coregame_140fa3ac0", fn_140fa3ac0 as *const ()), // sub_140fa3ac0
    Replacement::draft_throws(0x140fa3c40, "coregame_140fa3c40", fn_140fa3c40 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::updateSubtitles>
    Replacement::draft(0x140fa3fa0, "coregame_140fa3fa0", fn_140fa3fa0 as *const ()), // sub_140fa3fa0
    Replacement::draft(0x140fa4110, "coregame_140fa4110", fn_140fa4110 as *const ()), // sub_140fa4110
    Replacement::draft(0x140fa4310, "coregame_140fa4310", fn_140fa4310 as *const ()), // sub_140fa4310
    Replacement::draft(0x140fa43d0, "coregame_140fa43d0", fn_140fa43d0 as *const ()), // sub_140fa43d0
    Replacement::draft(0x140fa44b0, "coregame_140fa44b0", fn_140fa44b0 as *const ()), // sub_140fa44b0
    Replacement::draft(0x140fa46d0, "coregame_140fa46d0", fn_140fa46d0 as *const ()), // sub_140fa46d0
    Replacement::draft(0x140fa4750, "coregame_140fa4750", fn_140fa4750 as *const ()), // sub_140fa4750
    Replacement::draft(0x140fa49d0, "coregame_140fa49d0", tpl_867::<0x8, 0x140fa4a80, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140fa49d0
    Replacement::draft(0x140fa4a80, "coregame_140fa4a80", fn_140fa4a80 as *const ()), // sub_140fa4a80
    Replacement::draft(0x140fa4ae0, "coregame_140fa4ae0", fn_140fa4ae0 as *const ()), // sub_140fa4ae0
    Replacement::draft_throws(0x140fa4b80, "coregame_140fa4b80", fn_140fa4b80 as *const (), Throws::Discard), // sub_140fa4b80
    Replacement::draft_throws(0x140fa59b0, "coregame_140fa59b0", fn_140fa59b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streamingVolumeChunkingSystem>
    Replacement::draft_throws(0x140fa5ca0, "unknown_140fa5ca0", fn_140fa5ca0 as *const (), Throws::Discard), // sub_140fa5ca0
    Replacement::draft(0x140fa5d10, "unknown_140fa5d10", fn_140fa5d10 as *const ()), // sub_140fa5d10
    Replacement::draft(0x140fa5d90, "unknown_140fa5d90", fn_140fa5d90 as *const ()), // sub_140fa5d90
    Replacement::draft(0x140fa6070, "unknown_140fa6070", fn_140fa6070 as *const ()), // sub_140fa6070
    Replacement::draft(0x140fa6170, "unknown_140fa6170", fn_140fa6170 as *const ()), // sub_140fa6170
    Replacement::draft(0x140fa6270, "unknown_140fa6270", fn_140fa6270 as *const ()), // sub_140fa6270
    Replacement::draft_throws(0x140fa6520, "unknown_140fa6520", fn_140fa6520 as *const (), Throws::Discard), // sub_140fa6520
    Replacement::draft(0x140fa65e0, "unknown_140fa65e0", fn_140fa65e0 as *const ()), // sub_140fa65e0
    Replacement::draft(0x140fa6780, "unknown_140fa6780", fn_140fa6780 as *const ()), // sub_140fa6780
    Replacement::draft(0x140fa6850, "unknown_140fa6850", tpl_19154::<0x1425371a0, 0x142533200, 0x140fa6fc0, 0x140348530, 0x1401de830> as *const ()), // sub_140fa6850
    Replacement::draft(0x140fa6fc0, "unknown_140fa6fc0", fn_140fa6fc0 as *const ()), // sub_140fa6fc0
    Replacement::draft(0x140fa7080, "unknown_140fa7080", fn_140fa7080 as *const ()), // sub_140fa7080
    Replacement::draft_throws(0x140fa7360, "coregame_140fa7360", fn_140fa7360 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::ignoreStreamingBroadphase>
    Replacement::draft_throws(0x140fa7690, "coregame_140fa7690", tpl_6250::<0x143ccabb8, 0x143ccaab8, 0x1411e9f50, 0x140faa180, 0x143ccac18, 0x1420af120, 0x143c8d0c0, 0x140215610, 0x1420a28e0, 0x140fa9750, 0x140fa91d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332cd06, 0x142646960, 0x143ccaad8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::streamInOnLoad>
    Replacement::draft_throws(0x140fa79a0, "coregame_140fa79a0", tpl_1394::<0x143ccad08, 0x143ccaca8, 0x140fa8790, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332cf26, 0x142646960, 0x143ccacc8, 0x143ccaccc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::streamByDistance>
    Replacement::draft(0x140fa7bb0, "unknown_140fa7bb0", tpl_1094::<0x143ccaca0, 0x140fa8c20, 0x14020e8f0> as *const ()), // sub_140fa7bb0
    Replacement::draft(0x140fa7db0, "unknown_140fa7db0", tpl_1096::<0x143ccac00, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fa7db0
    Replacement::draft(0x140fa80e0, "unknown_140fa80e0", tpl_1466::<0x143ccad2c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fa80e0
    Replacement::draft(0x140fa83b0, "unknown_140fa83b0", tpl_1094::<0x143ccac04, 0x140492c90, 0x14020e8f0> as *const ()), // sub_140fa83b0
    Replacement::draft(0x140fa85b0, "unknown_140fa85b0", tpl_1095::<0x143ccad0c, 0x14020e8f0> as *const ()), // sub_140fa85b0
    Replacement::draft_throws(0x140fa8790, "unknown_140fa8790", tpl_3697::<0x1411e9f50, 0x140faa040, 0x143ccab88, 0x1420af120, 0x140215750, 0x143c8d014, 0x140215610, 0x1420a28e0, 0x140fa9920, 0x1401de410> as *const (), Throws::Discard), // sub_140fa8790
    Replacement::draft(0x140fa8970, "unknown_140fa8970", tpl_1095::<0x143ccabfc, 0x14020e8f0> as *const ()), // sub_140fa8970
    Replacement::draft(0x140fa8b50, "unknown_140fa8b50", fn_140fa8b50 as *const ()), // sub_140fa8b50
    Replacement::draft(0x140fa8c20, "unknown_140fa8c20", fn_140fa8c20 as *const ()), // sub_140fa8c20
    Replacement::draft_throws(0x140fa8db0, "unknown_140fa8db0", tpl_19474::<0x140fa8ed0, 0x143ccaca0, 0x142489ee0, 0xc03, 0x140fa90d0, 0x14065a1a0> as *const (), Throws::Discard), // sub_140fa8db0
    Replacement::draft_throws(0x140fa8ed0, "unknown_140fa8ed0", fn_140fa8ed0 as *const (), Throws::Discard), // sub_140fa8ed0
    Replacement::draft(0x140fa90d0, "unknown_140fa90d0", tpl_77866::<0x18, 0x20, 0x28, 0x1c8, 0x1e0, 0x1d0, 0x141e4fae0> as *const ()), // sub_140fa90d0
    Replacement::draft(0x140fa91d0, "unknown_140fa91d0", tpl_5377::<0x143c8d0c0, 0x142407c80, 0x140fa92d0, 0x580f0, 0x141e4fce0> as *const ()), // sub_140fa91d0
    Replacement::draft_throws(0x140fa93d0, "unknown_140fa93d0", tpl_1331::<0x8, 0x140faa280, 0x143ccac08, 0x20, 0x1420af120, 0x10, 0x60, 0x80, 0xa0> as *const (), Throws::Discard), // sub_140fa93d0
    Replacement::draft_throws(0x140fa9480, "unknown_140fa9480", tpl_1330::<0x8, 0x140faa280, 0x143ccac08, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140fa9480
    Replacement::draft(0x140fa9560, "unknown_140fa9560", tpl_5367::<0x7, 0x18, 0x8> as *const ()), // sub_140fa9560
    Replacement::draft_throws(0x140fa9580, "unknown_140fa9580", tpl_19476::<0x140fa8ed0, 0x143ccaca0, 0x142489ee0, 0x141e4fae0, 0x14065a1a0> as *const (), Throws::Discard), // sub_140fa9580
    Replacement::draft(0x140fa9750, "unknown_140fa9750", tpl_5378::<0x143c8d0c0, 0x142407c80, 0x143ccaca0, 0x142489ee0, 0x143c8d108, 0x141e4fce0> as *const ()), // sub_140fa9750
    Replacement::draft(0x140fa9920, "unknown_140fa9920", tpl_19301::<0x143ccab88, 0x20, 0x8, 0x143c8d014, 0x580e0, 0x142407c80, 0x580f0, 0x141e4fe60> as *const ()), // sub_140fa9920
    Replacement::draft(0x140faa660, "unknown_140faa660", fn_140faa660 as *const ()), // sub_140faa660
    Replacement::draft(0x140faa710, "unknown_140faa710", fn_140faa710 as *const ()), // sub_140faa710
    Replacement::draft_throws(0x140faa870, "unknown_140faa870", tpl_1480::<0x1428bce00, 0x1410d6ca0, 0x140fab100, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140faa870
    Replacement::draft(0x140faab10, "unknown_140faab10", tpl_1526::<0x1428bce00, 0x1410d6ca0, 0x140fab430, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140faab10
    Replacement::draft_throws(0x140faadb0, "unknown_140faadb0", tpl_1480::<0x1428bce00, 0x1410d6ca0, 0x140fab6b0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140faadb0
    Replacement::draft_throws(0x140fab050, "unknown_140fab050", fn_140fab050 as *const (), Throws::Discard), // sub_140fab050
    Replacement::draft_throws(0x140fab100, "coregame_140fab100", fn_140fab100 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::updateStreamingPositionsUsingCamera>
    Replacement::draft(0x140fab430, "coregame_140fab430", fn_140fab430 as *const ()), // ecs::system::ecs::system::wrapper<coregame::streaming::pushToRenderer>
    Replacement::draft_throws(0x140fab6b0, "coregame_140fab6b0", fn_140fab6b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::updateStreamingPositions>
    Replacement::draft(0x140fab9a0, "unknown_140fab9a0", fn_140fab9a0 as *const ()), // sub_140fab9a0
    Replacement::draft_throws(0x140fabb10, "unknown_140fabb10", tpl_1330::<0x8, 0x140fac130, 0x143ccae34, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140fabb10
    Replacement::draft_throws(0x140fabbf0, "unknown_140fabbf0", fn_140fabbf0 as *const (), Throws::Discard), // sub_140fabbf0
    Replacement::draft_throws(0x140fabd50, "unknown_140fabd50", fn_140fabd50 as *const (), Throws::Discard), // sub_140fabd50
    Replacement::draft(0x140fabe90, "unknown_140fabe90", fn_140fabe90 as *const ()), // sub_140fabe90
    Replacement::draft(0x140fac1e0, "unknown_140fac1e0", fn_140fac1e0 as *const ()), // sub_140fac1e0
    Replacement::draft(0x140fac290, "unknown_140fac290", fn_140fac290 as *const ()), // sub_140fac290
    Replacement::draft(0x140fac400, "unknown_140fac400", fn_140fac400 as *const ()), // sub_140fac400
    Replacement::draft(0x140fac500, "unknown_140fac500", fn_140fac500 as *const ()), // sub_140fac500
    Replacement::draft(0x140fac650, "unknown_140fac650", fn_140fac650 as *const ()), // sub_140fac650
    Replacement::draft(0x140fac710, "unknown_140fac710", fn_140fac710 as *const ()), // sub_140fac710
    Replacement::draft(0x140fac880, "unknown_140fac880", fn_140fac880 as *const ()), // sub_140fac880
    Replacement::draft_throws(0x140fac990, "unknown_140fac990", fn_140fac990 as *const (), Throws::Discard), // sub_140fac990
    Replacement::draft(0x140fac9f0, "unknown_140fac9f0", fn_140fac9f0 as *const ()), // sub_140fac9f0
    Replacement::draft(0x140facab0, "unknown_140facab0", fn_140facab0 as *const ()), // sub_140facab0
    Replacement::draft_throws(0x140facb90, "unknown_140facb90", fn_140facb90 as *const (), Throws::Caught), // sub_140facb90
    Replacement::draft_throws(0x140facc20, "unknown_140facc20", tpl_1908::<0x143880408, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290> as *const (), Throws::Discard), // sub_140facc20
    Replacement::draft(0x140facca0, "unknown_140facca0", fn_140facca0 as *const ()), // sub_140facca0
    Replacement::draft_throws(0x140face60, "coregame_140face60", tpl_1583::<0x143ccaec4, 0x143ccaed0, 0x1411e9f50, 0x140fad9d0, 0x140fadcd0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332e856, 0x142646960, 0x143ccaef0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::updateGraphManager>
    Replacement::draft(0x140fad0a0, "unknown_140fad0a0", fn_140fad0a0 as *const ()), // sub_140fad0a0
    Replacement::draft(0x140fad160, "unknown_140fad160", fn_140fad160 as *const ()), // sub_140fad160
    Replacement::draft(0x140fad220, "unknown_140fad220", fn_140fad220 as *const ()), // sub_140fad220
    Replacement::draft_throws(0x140fad2f0, "unknown_140fad2f0", fn_140fad2f0 as *const (), Throws::Discard), // sub_140fad2f0
    Replacement::draft_throws(0x140fad3a0, "unknown_140fad3a0", fn_140fad3a0 as *const (), Throws::Discard), // sub_140fad3a0
    Replacement::draft(0x140fad520, "unknown_140fad520", fn_140fad520 as *const ()), // sub_140fad520
    Replacement::draft(0x140fad6b0, "unknown_140fad6b0", fn_140fad6b0 as *const ()), // sub_140fad6b0
    Replacement::draft(0x140fad830, "unknown_140fad830", fn_140fad830 as *const ()), // sub_140fad830
    Replacement::draft_throws(0x140fad9d0, "unknown_140fad9d0", fn_140fad9d0 as *const (), Throws::Discard), // sub_140fad9d0
    Replacement::draft_throws(0x140fadbf0, "unknown_140fadbf0", tpl_1330::<0x8, 0x140fade70, 0x143ccaec8, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140fadbf0
    Replacement::draft_throws(0x140fadcd0, "unknown_140fadcd0", fn_140fadcd0 as *const (), Throws::Discard), // sub_140fadcd0
    Replacement::draft(0x140fae130, "unknown_140fae130", fn_140fae130 as *const ()), // sub_140fae130
    Replacement::draft(0x140faeff0, "unknown_140faeff0", fn_140faeff0 as *const ()), // sub_140faeff0
    Replacement::draft_throws(0x140faf0a0, "coregame_140faf0a0", fn_140faf0a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::processAddedOrRemovedBroadphaseVolumes>
    Replacement::draft(0x140faf3a0, "coregame_140faf3a0", tpl_5528::<0x143ccaf3c, 0x143ccaf40, 0x1411e9f50, 0x143ccb2b0, 0x140215610, 0x1420a28e0, 0x140fb3c70, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332edc6, 0x142646960, 0x143ccaf60, 0x143ccaf68, 0x143ccaf6c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::streaming::clearAddedOrRemovedFlag>
    Replacement::draft_throws(0x140faf640, "coregame_140faf640", tpl_3395::<0x143ccaf70, 0x143ccaf78, 0x140fb12e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332f126, 0x142646960, 0x143ccaf98, 0x143ccafa0, 0x143ccafa2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::addVolumeToBroadphase>
    Replacement::draft(0x140faf850, "coregame_140faf850", tpl_3459::<0x143ccafa4, 0x143ccafa8, 0x140fb14a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332f016, 0x142646960, 0x143ccafc8, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::streaming::removeVolumeFromBroadphase>
    Replacement::draft_throws(0x140fafa50, "coregame_140fafa50", tpl_1173::<0x143ccafd8, 0x143ccafe0, 0x1411e9f50, 0x140fb1f60, 0x140fb3f20, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332fb96, 0x142646960, 0x143ccb000, 0x143ccb010, 0x143ccb014, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::updateBroadphaseVolumeInclusions>
    Replacement::draft_throws(0x140fafca0, "coregame_140fafca0", fn_140fafca0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::addToBroadphase>
    Replacement::draft_throws(0x140fb0000, "coregame_140fb0000", fn_140fb0000 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::removeFromBroadphase>
    Replacement::draft_throws(0x140fb0330, "coregame_140fb0330", tpl_1583::<0x143ccb07c, 0x143ccb080, 0x1411e9f50, 0x140fb2690, 0x140fb4390, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14332ff26, 0x142646960, 0x143ccb0a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::streamInBroadphase>
    Replacement::draft_throws(0x140fb0570, "coregame_140fb0570", tpl_1793::<0x143ccb0a8, 0x143ccb0b0, 0x140fb1640, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143330cd6, 0x142646960, 0x143ccb0d0, 0x143ccb0d8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::streamOutBroadphase>
    Replacement::draft_throws(0x140fb0780, "coregame_140fb0780", fn_140fb0780 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::dirtyChangedTransforms>
    Replacement::draft(0x140fb0a60, "unknown_140fb0a60", fn_140fb0a60 as *const ()), // sub_140fb0a60
    Replacement::draft(0x140fb0ae0, "unknown_140fb0ae0", fn_140fb0ae0 as *const ()), // sub_140fb0ae0
    Replacement::draft(0x140fb0ba0, "unknown_140fb0ba0", fn_140fb0ba0 as *const ()), // sub_140fb0ba0
    Replacement::draft(0x140fb0db0, "unknown_140fb0db0", tpl_1436::<0x58, 0x60, 0x158, 0x143c8d694, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140fb0db0
    Replacement::draft(0x140fb0e90, "unknown_140fb0e90", fn_140fb0e90 as *const ()), // sub_140fb0e90
    Replacement::draft(0x140fb0ed0, "unknown_140fb0ed0", tpl_1094::<0x143ccb12c, 0x140fb1c90, 0x14020e8f0> as *const ()), // sub_140fb0ed0
    Replacement::draft(0x140fb10d0, "unknown_140fb10d0", tpl_1095::<0x143ccb128, 0x14020e8f0> as *const ()), // sub_140fb10d0
    Replacement::draft_throws(0x140fb12e0, "unknown_140fb12e0", tpl_5552::<0x1411e9f50, 0x140fb5c20, 0x143ccb678, 0x1420af120, 0x143c8d080, 0x140215610, 0x143ccb2b0, 0x1420a28e0, 0x140fb3c90, 0x140fb1e70> as *const (), Throws::Discard), // sub_140fb12e0
    Replacement::draft(0x140fb14a0, "unknown_140fb14a0", tpl_1988::<0x1411e9f50, 0x140fb4b40, 0x143ccb12c, 0x1401fc320, 0x143c8d080, 0x140215610, 0x143ccb2b0, 0x1420a28e0, 0x140fb3e60, 0x1401de410> as *const ()), // sub_140fb14a0
    Replacement::draft_throws(0x140fb1640, "unknown_140fb1640", tpl_3697::<0x1411e9f50, 0x140fb53a0, 0x143ccb5e8, 0x1420af120, 0x140215750, 0x143c8d080, 0x140215610, 0x1420a28e0, 0x140fb4410, 0x1401de410> as *const (), Throws::Discard), // sub_140fb1640
    Replacement::draft(0x140fb17f0, "unknown_140fb17f0", fn_140fb17f0 as *const ()), // sub_140fb17f0
    Replacement::draft(0x140fb1c90, "unknown_140fb1c90", fn_140fb1c90 as *const ()), // sub_140fb1c90
    Replacement::draft(0x140fb1e70, "unknown_140fb1e70", tpl_19516::<0x88, 0x10088, 0x143c8d080, 0x580e0, 0x143ccb2b0, 0x142407c80, 0x140fb37e0, 0x580f0, 0x141e56440> as *const ()), // sub_140fb1e70
    Replacement::draft_throws(0x140fb1f60, "unknown_140fb1f60", tpl_19411::<0x140fb5950, 0x143ccb670, 0x1420af120, 0x140215750, 0x143c8d080, 0x140215610, 0x143c8d028, 0x1420a28e0> as *const (), Throws::Discard), // sub_140fb1f60
    Replacement::draft_throws(0x140fb20d0, "unknown_140fb20d0", fn_140fb20d0 as *const (), Throws::Caught), // sub_140fb20d0
    Replacement::draft_throws(0x140fb2220, "unknown_140fb2220", fn_140fb2220 as *const (), Throws::Discard), // sub_140fb2220
    Replacement::draft_throws(0x140fb23d0, "unknown_140fb23d0", fn_140fb23d0 as *const (), Throws::Discard), // sub_140fb23d0
    Replacement::draft_throws(0x140fb24d0, "unknown_140fb24d0", fn_140fb24d0 as *const (), Throws::Discard), // sub_140fb24d0
    Replacement::draft_throws(0x140fb2690, "unknown_140fb2690", tpl_19411::<0x140fb54e0, 0x143ccb5ec, 0x1420af120, 0x140215750, 0x143c8d080, 0x140215610, 0x143c9a2d0, 0x1420a28e0> as *const (), Throws::Discard), // sub_140fb2690
    Replacement::draft(0x140fb2800, "unknown_140fb2800", tpl_1466::<0x143ccb110, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fb2800
    Replacement::draft(0x140fb2ad0, "unknown_140fb2ad0", tpl_1466::<0x143ccb118, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fb2ad0
    Replacement::draft(0x140fb2da0, "unknown_140fb2da0", tpl_3074::<0x143ccb11c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fb2da0
    Replacement::draft(0x140fb3070, "unknown_140fb3070", tpl_4899::<0x143ccb114, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fb3070
    Replacement::draft(0x140fb33c0, "unknown_140fb33c0", tpl_4390::<0x143ccb124, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fb33c0
    Replacement::draft_throws(0x140fb3700, "unknown_140fb3700", tpl_1330::<0x8, 0x140fb5b70, 0x143ccb674, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140fb3700
    Replacement::draft(0x140fb38e0, "unknown_140fb38e0", fn_140fb38e0 as *const ()), // sub_140fb38e0
    Replacement::draft(0x140fb3ad0, "unknown_140fb3ad0", fn_140fb3ad0 as *const ()), // sub_140fb3ad0
    Replacement::draft(0x140fb3b90, "unknown_140fb3b90", tpl_19288::<0x7, 0x30, 0x20, 0x1401de830, 0x10> as *const ()), // sub_140fb3b90
    Replacement::draft(0x140fb3bf0, "unknown_140fb3bf0", tpl_4431::<0x143ccb674, 0x20, 0x143ccb2b0, 0x8, 0x580e0, 0x143c8d028, 0x141e56120> as *const ()), // sub_140fb3bf0
    Replacement::draft(0x140fb3c70, "unknown_140fb3c70", fn_140fb3c70 as *const ()), // sub_140fb3c70
    Replacement::draft(0x140fb3c90, "unknown_140fb3c90", tpl_19523::<0x143c8d080, 0x143ccb2b0, 0x142407c80, 0x143c8d120, 0x142489ee0, 0x143ccb12c, 0x141e56440> as *const ()), // sub_140fb3c90
    Replacement::draft(0x140fb3e60, "unknown_140fb3e60", tpl_5049::<0x143ccb12c, 0x5e308, 0x580e0, 0x143c8d080, 0x143ccb2b0, 0x10, 0x8, 0x18, 0x141e563c0, 0x140fb4f90> as *const ()), // sub_140fb3e60
    Replacement::draft_throws(0x140fb3f20, "unknown_140fb3f20", tpl_5944::<0x143ccb670, 0x20, 0x143c8d080, 0x8, 0x580e0, 0x143c8d028, 0x141e57a60> as *const (), Throws::Discard), // sub_140fb3f20
    Replacement::draft_throws(0x140fb3fa0, "unknown_140fb3fa0", fn_140fb3fa0 as *const (), Throws::Caught), // sub_140fb3fa0
    Replacement::draft_throws(0x140fb4130, "unknown_140fb4130", fn_140fb4130 as *const (), Throws::Discard), // sub_140fb4130
    Replacement::draft_throws(0x140fb4390, "unknown_140fb4390", tpl_5944::<0x143ccb5ec, 0x20, 0x143c8d080, 0x8, 0x580e0, 0x143c9a2d0, 0x141e571b0> as *const (), Throws::Discard), // sub_140fb4390
    Replacement::draft(0x140fb4410, "unknown_140fb4410", tpl_19301::<0x143ccb5e8, 0x20, 0x8, 0x143c8d080, 0x580e0, 0x142407c80, 0x580f0, 0x141e56ff0> as *const ()), // sub_140fb4410
    Replacement::draft(0x140fb44b0, "unknown_140fb44b0", tpl_4431::<0x143ccb674, 0x20, 0x143cc7c80, 0x8, 0x580e0, 0x143c8d028, 0x141e56dd0> as *const ()), // sub_140fb44b0
    Replacement::draft_throws(0x140fb4530, "unknown_140fb4530", tpl_4037::<0x143ccb10c, 0x143ccb684, 0x143ccb688, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143ccb114, 0x143ccb110, 0x143ccb698, 0x143ccb6a0, 0x141834a0c> as *const (), Throws::Discard), // sub_140fb4530
    Replacement::draft(0x140fb4b40, "unknown_140fb4b40", tpl_3188::<0x140fb5ad0, 0x143333000, 0x140fb4f90, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fb4b40
    Replacement::draft(0x140fb4bc0, "unknown_140fb4bc0", tpl_4348::<0x10, 0x18, 0x140fb4f90> as *const ()), // sub_140fb4bc0
    Replacement::draft(0x140fb4dd0, "unknown_140fb4dd0", fn_140fb4dd0 as *const ()), // sub_140fb4dd0
    Replacement::draft(0x140fb4f90, "unknown_140fb4f90", fn_140fb4f90 as *const ()), // sub_140fb4f90
    Replacement::draft(0x140fb5ad0, "unknown_140fb5ad0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143333000, 0x8, 0x20> as *const ()), // sub_140fb5ad0
    Replacement::draft(0x140fb5d20, "unknown_140fb5d20", fn_140fb5d20 as *const ()), // sub_140fb5d20
    Replacement::draft(0x140fb5ee0, "unknown_140fb5ee0", fn_140fb5ee0 as *const ()), // sub_140fb5ee0
    Replacement::draft(0x140fb6110, "unknown_140fb6110", fn_140fb6110 as *const ()), // sub_140fb6110
    Replacement::draft(0x140fb6360, "unknown_140fb6360", fn_140fb6360 as *const ()), // sub_140fb6360
    Replacement::draft(0x140fb68e0, "unknown_140fb68e0", fn_140fb68e0 as *const ()), // sub_140fb68e0
    Replacement::draft(0x140fb6a90, "unknown_140fb6a90", fn_140fb6a90 as *const ()), // sub_140fb6a90
    Replacement::draft(0x140fb6d80, "unknown_140fb6d80", fn_140fb6d80 as *const ()), // sub_140fb6d80
    Replacement::draft(0x140fb6db0, "unknown_140fb6db0", fn_140fb6db0 as *const ()), // sub_140fb6db0
    Replacement::draft_throws(0x140fb6e50, "unknown_140fb6e50", fn_140fb6e50 as *const (), Throws::Discard), // sub_140fb6e50
    Replacement::draft(0x140fb6eb0, "unknown_140fb6eb0", tpl_6045::<0x30, 0x143c9a4e0, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140fb6f2e> as *const ()), // sub_140fb6eb0
    Replacement::draft(0x140fb6f30, "unknown_140fb6f30", fn_140fb6f30 as *const ()), // sub_140fb6f30
    Replacement::draft(0x140fb6fd0, "unknown_140fb6fd0", fn_140fb6fd0 as *const ()), // sub_140fb6fd0
    Replacement::draft(0x140fb7020, "unknown_140fb7020", tpl_19540::<0x80, 0x1410d3420, 0x70, 0x140c345c0, 0x8, 0x1410d3a00, 0x142442de0> as *const ()), // sub_140fb7020
    Replacement::draft(0x140fb7060, "unknown_140fb7060", tpl_19541::<0x140afc400, 0x1425371a0, 0x142533200, 0x1430782a0> as *const ()), // sub_140fb7060
    Replacement::draft(0x140fb7380, "unknown_140fb7380", tpl_60953::<0x1426467e8, 0x143ccb120, 0x142489ee0, 0x140fb73fc> as *const ()), // sub_140fb7380
    Replacement::draft(0x140fb7e20, "unknown_140fb7e20", fn_140fb7e20 as *const ()), // sub_140fb7e20
    Replacement::draft(0x140fb7ed0, "unknown_140fb7ed0", fn_140fb7ed0 as *const ()), // sub_140fb7ed0
    Replacement::draft(0x140fb7fe0, "unknown_140fb7fe0", fn_140fb7fe0 as *const ()), // sub_140fb7fe0
    Replacement::draft_throws(0x140fb8060, "unknown_140fb8060", fn_140fb8060 as *const (), Throws::Caught), // sub_140fb8060
    Replacement::draft(0x140fb8100, "unknown_140fb8100", fn_140fb8100 as *const ()), // sub_140fb8100
    Replacement::draft(0x140fb81c0, "coregame_140fb81c0", tpl_1537::<0x143ccb94c, 0x143ccb950, 0x1411e9f50, 0x143c9a2d0, 0x140215610, 0x1420a28e0, 0x140fbaa80, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143333336, 0x142646960, 0x143ccb970, 0x143ccb978, 0x143ccb97c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::streaming::unacquireResourceBatches>
    Replacement::draft_throws(0x140fb8460, "coregame_140fb8460", fn_140fb8460 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::removeEntitiesFromPreviousBatches>
    Replacement::draft_throws(0x140fb87c0, "coregame_140fb87c0", fn_140fb87c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::startGatheringResources>
    Replacement::draft_throws(0x140fb8ab0, "coregame_140fb8ab0", fn_140fb8ab0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::startLoadingResourceBatches>
    Replacement::draft(0x140fb8da0, "coregame_140fb8da0", fn_140fb8da0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::streaming::processLoadingResourceBatches>
    Replacement::draft_throws(0x140fb9040, "coregame_140fb9040", tpl_3653::<0x143ccba6c, 0x143ccba70, 0x1411e9f50, 0x140fba120, 0x140fbac80, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433337d6, 0x142646960, 0x143ccba90, 0x143ccba98, 0x143ccba9c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::streamInFinishedBatches>
    Replacement::draft_throws(0x140fb9290, "coregame_140fb9290", tpl_19412::<0x143ccbaa0, 0x143ccbaa8, 0x1411e9f50, 0x140fba380, 0x140fbadb0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143333f16, 0x142646960, 0x143ccbac8, 0x143ccbad8, 0x143ccbada, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::streaming::updateBatchesToLegacyManagers>
    Replacement::draft(0x140fb94e0, "coregame_140fb94e0", fn_140fb94e0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::streaming::processRequests>
    Replacement::draft(0x140fb97b0, "unknown_140fb97b0", fn_140fb97b0 as *const ()), // sub_140fb97b0
    Replacement::draft(0x140fb98b0, "unknown_140fb98b0", fn_140fb98b0 as *const ()), // sub_140fb98b0
    Replacement::draft(0x140fb9980, "unknown_140fb9980", tpl_60530::<0x1426467e8, 0x143c9a620, 0x142489ee0, 0x143ccb120, 0x140fb99a9> as *const ()), // sub_140fb9980
    Replacement::draft(0x140fb9b40, "unknown_140fb9b40", fn_140fb9b40 as *const ()), // sub_140fb9b40
    Replacement::draft_throws(0x140fb9bd0, "unknown_140fb9bd0", fn_140fb9bd0 as *const (), Throws::Discard), // sub_140fb9bd0
    Replacement::draft(0x140fb9d30, "unknown_140fb9d30", fn_140fb9d30 as *const ()), // sub_140fb9d30
    Replacement::draft(0x140fb9f90, "unknown_140fb9f90", fn_140fb9f90 as *const ()), // sub_140fb9f90
    Replacement::draft_throws(0x140fba120, "unknown_140fba120", fn_140fba120 as *const (), Throws::Discard), // sub_140fba120
    Replacement::draft_throws(0x140fba380, "unknown_140fba380", tpl_4425::<0x140fbaf70, 0x143ccbc40, 0x1420af120, 0x140215750, 0x143c8d170, 0x140215610, 0x143ccbb08, 0x1420a28e0> as *const (), Throws::Discard), // sub_140fba380
    Replacement::draft_throws(0x140fba500, "unknown_140fba500", tpl_1330::<0x8, 0x140fbb3f0, 0x143ccbc50, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140fba500
    Replacement::draft(0x140fba5e0, "unknown_140fba5e0", fn_140fba5e0 as *const ()), // sub_140fba5e0
    Replacement::draft(0x140fba720, "unknown_140fba720", fn_140fba720 as *const ()), // sub_140fba720
    Replacement::draft(0x140fbaaa0, "unknown_140fbaaa0", fn_140fbaaa0 as *const ()), // sub_140fbaaa0
    Replacement::draft_throws(0x140fbab10, "unknown_140fbab10", tpl_1804::<0x143ccbc50, 0x580e0, 0x20, 0x8, 0x143c9a718, 0x143c9a2d0, 0x141e5b990> as *const (), Throws::Discard), // sub_140fbab10
    Replacement::draft_throws(0x140fbab90, "unknown_140fbab90", fn_140fbab90 as *const (), Throws::Discard), // sub_140fbab90
    Replacement::draft_throws(0x140fbadb0, "unknown_140fbadb0", tpl_5944::<0x143ccbc40, 0x20, 0x143c8d170, 0x8, 0x580e0, 0x143ccbb08, 0x141e59da0> as *const (), Throws::Discard), // sub_140fbadb0
    Replacement::draft(0x140fbae60, "unknown_140fbae60", fn_140fbae60 as *const ()), // sub_140fbae60
    Replacement::draft(0x140fbb590, "unknown_140fbb590", tpl_381::<0x140fbb6f0, 0x140fbba70> as *const ()), // sub_140fbb590
    Replacement::draft(0x140fbb6f0, "unknown_140fbb6f0", fn_140fbb6f0 as *const ()), // sub_140fbb6f0
    Replacement::draft(0x140fbba70, "unknown_140fbba70", fn_140fbba70 as *const ()), // sub_140fbba70
    Replacement::draft(0x140fbbc40, "unknown_140fbbc40", fn_140fbbc40 as *const ()), // sub_140fbbc40
    Replacement::draft(0x140fbbd50, "unknown_140fbbd50", tpl_5962::<0x140fb9b40, 0x140fbcae0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x140267680, 0x1401e66f0, 0x142442de0> as *const ()), // sub_140fbbd50
    Replacement::draft(0x140fbbee0, "unknown_140fbbee0", fn_140fbbee0 as *const ()), // sub_140fbbee0
    Replacement::draft(0x140fbc120, "unknown_140fbc120", tpl_19567::<0x140fbc520, 0x140fb7060, 0x1403daa30, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1403c7e70, 0x1401de830, 0x142442de0> as *const ()), // sub_140fbc120
    Replacement::draft(0x140fbc2d0, "unknown_140fbc2d0", fn_140fbc2d0 as *const ()), // sub_140fbc2d0
    Replacement::draft(0x140fbc390, "unknown_140fbc390", fn_140fbc390 as *const ()), // sub_140fbc390
    Replacement::draft(0x140fbc450, "unknown_140fbc450", fn_140fbc450 as *const ()), // sub_140fbc450
    Replacement::draft(0x140fbc520, "unknown_140fbc520", fn_140fbc520 as *const ()), // sub_140fbc520
    Replacement::draft(0x140fbcb90, "unknown_140fbcb90", fn_140fbcb90 as *const ()), // sub_140fbcb90
    Replacement::draft(0x140fbcc70, "unknown_140fbcc70", tpl_1663::<0x8, 0x140fb9b40, 0x1401e66f0> as *const ()), // sub_140fbcc70
    Replacement::draft(0x140fbccb0, "unknown_140fbccb0", fn_140fbccb0 as *const ()), // sub_140fbccb0
    Replacement::draft(0x140fbcf60, "unknown_140fbcf60", fn_140fbcf60 as *const ()), // sub_140fbcf60
    Replacement::draft(0x140fbdf50, "unknown_140fbdf50", fn_140fbdf50 as *const ()), // sub_140fbdf50
    Replacement::draft(0x140fbdfc0, "unknown_140fbdfc0", fn_140fbdfc0 as *const ()), // sub_140fbdfc0
    Replacement::draft(0x140fbdfd0, "unknown_140fbdfd0", fn_140fbdfd0 as *const ()), // sub_140fbdfd0
    Replacement::draft(0x140fbe040, "unknown_140fbe040", tpl_170::<0x140fbf8d0> as *const ()), // sub_140fbe040
    Replacement::draft(0x140fbe050, "std_140fbe050", fn_140fbe050 as *const ()), // sub_140fbe050
    Replacement::draft(0x140fbe080, "unknown_140fbe080", fn_140fbe080 as *const ()), // sub_140fbe080
    Replacement::draft(0x140fbe190, "unknown_140fbe190", fn_140fbe190 as *const ()), // sub_140fbe190
    Replacement::draft(0x140fbe2f0, "unknown_140fbe2f0", fn_140fbe2f0 as *const ()), // sub_140fbe2f0
    Replacement::draft(0x140fbe3b0, "unknown_140fbe3b0", fn_140fbe3b0 as *const ()), // sub_140fbe3b0
    Replacement::draft(0x140fbe480, "unknown_140fbe480", fn_140fbe480 as *const ()), // sub_140fbe480
    Replacement::draft_throws(0x140fbe560, "coregame_140fbe560", tpl_3061::<0x143ccbf38, 0x143ccbf40, 0x1411e9f50, 0x140fc1b40, 0x143ccc048, 0x1420af120, 0x1420a28e0, 0x140fc0e30, 0x140fc0d20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143334cc6, 0x142646960, 0x143ccbf60, 0x143ccbf68, 0x143ccbf6c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spawning::poll_instantation::system>
    Replacement::draft_throws(0x140fbe850, "coregame_140fbe850", fn_140fbe850 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spawning::handle_despawns::system>
    Replacement::draft(0x140fbeb50, "coregame_140fbeb50", fn_140fbeb50 as *const ()), // ecs::system::ecs::system::wrapper<coregame::spawning::handle_spawn_requests::system>
    Replacement::draft_throws(0x140fbedf0, "sparsehash_140fbedf0", fn_140fbedf0 as *const (), Throws::Discard), // sub_140fbedf0
    Replacement::draft_throws(0x140fbefa0, "sparsehash_140fbefa0", tpl_77984::<0x140fbf340, 0x140fbf1e0, 0x140227190, 0x1401e2020, 0x143426fc8, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x140fbf1d2> as *const (), Throws::Discard), // sub_140fbefa0
    Replacement::draft_throws(0x140fbf340, "unknown_140fbf340", tpl_1271::<0x1401e2020, 0x143426fc8, 0x140fbf4b0> as *const (), Throws::Discard), // sub_140fbf340
    Replacement::draft(0x140fbf7d0, "unknown_140fbf7d0", fn_140fbf7d0 as *const ()), // sub_140fbf7d0
    Replacement::draft(0x140fbf8d0, "unknown_140fbf8d0", tpl_13347::<0x8, 0x141f7c390, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140fbf8d0
    Replacement::draft(0x140fbf9a0, "unknown_140fbf9a0", fn_140fbf9a0 as *const ()), // sub_140fbf9a0
    Replacement::draft_throws(0x140fbfa70, "unknown_140fbfa70", fn_140fbfa70 as *const (), Throws::Discard), // sub_140fbfa70
    Replacement::draft(0x140fbfaf0, "unknown_140fbfaf0", tpl_1094::<0x143c9320c, 0x140fc07e0, 0x14020e8f0> as *const ()), // sub_140fbfaf0
    Replacement::draft(0x140fbfcf0, "unknown_140fbfcf0", tpl_1094::<0x143ccbfdc, 0x140fc09c0, 0x14020e8f0> as *const ()), // sub_140fbfcf0
    Replacement::draft(0x140fbfef0, "unknown_140fbfef0", tpl_1094::<0x143c8ea68, 0x140fa8c20, 0x14020e8f0> as *const ()), // sub_140fbfef0
    Replacement::draft(0x140fc00f0, "unknown_140fc00f0", tpl_1436::<0x58, 0x60, 0x158, 0x143ccbfdc, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140fc00f0
    Replacement::draft(0x140fc01d0, "std_140fc01d0", fn_140fc01d0 as *const ()), // std::_Ref_count_resource<coregame::spawning::detail::UnacquiringResourceBatch*,r::detail::Destroyer<coregame::spawning::
    Replacement::draft(0x140fc0200, "std_140fc0200", tpl_965::<0x8, 0x14388f708, 0x142646348, 0x10> as *const ()), // std::_Ref_count_resource<coregame::spawning::detail::UnacquiringResourceBatch*,r::detail::Destroyer<coregame::spawning::
    Replacement::draft(0x140fc0230, "unknown_140fc0230", fn_140fc0230 as *const ()), // sub_140fc0230
    Replacement::draft(0x140fc0390, "unknown_140fc0390", fn_140fc0390 as *const ()), // sub_140fc0390
    Replacement::draft(0x140fc07e0, "unknown_140fc07e0", fn_140fc07e0 as *const ()), // sub_140fc07e0
    Replacement::draft(0x140fc09c0, "unknown_140fc09c0", fn_140fc09c0 as *const ()), // sub_140fc09c0
    Replacement::draft(0x140fc0b50, "unknown_140fc0b50", fn_140fc0b50 as *const ()), // sub_140fc0b50
    Replacement::draft(0x140fc0d20, "unknown_140fc0d20", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140fc00f0> as *const ()), // sub_140fc0d20
    Replacement::draft(0x140fc0dc0, "unknown_140fc0dc0", tpl_5213::<0x38, 0x141f7c390> as *const ()), // sub_140fc0dc0
    Replacement::draft(0x140fc0e00, "unknown_140fc0e00", tpl_1955::<0x10, 0x140203870> as *const ()), // sub_140fc0e00
    Replacement::draft(0x140fc0e30, "unknown_140fc0e30", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140fc00f0> as *const ()), // sub_140fc0e30
    Replacement::draft(0x140fc0f10, "unknown_140fc0f10", fn_140fc0f10 as *const ()), // sub_140fc0f10
    Replacement::draft_throws(0x140fc1040, "unknown_140fc1040", fn_140fc1040 as *const (), Throws::Caught), // sub_140fc1040
    Replacement::draft(0x140fc1090, "unknown_140fc1090", tpl_3188::<0x140fc1aa0, 0x1433352b8, 0x140fc15c0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fc1090
    Replacement::draft(0x140fc1110, "unknown_140fc1110", fn_140fc1110 as *const ()), // sub_140fc1110
    Replacement::draft(0x140fc1200, "unknown_140fc1200", tpl_4348::<0x18, 0x20, 0x140fc15c0> as *const ()), // sub_140fc1200
    Replacement::draft(0x140fc1400, "unknown_140fc1400", fn_140fc1400 as *const ()), // sub_140fc1400
    Replacement::draft(0x140fc15c0, "unknown_140fc15c0", fn_140fc15c0 as *const ()), // sub_140fc15c0
    Replacement::draft(0x140fc16e0, "unknown_140fc16e0", fn_140fc16e0 as *const ()), // sub_140fc16e0
    Replacement::draft(0x140fc17b0, "unknown_140fc17b0", fn_140fc17b0 as *const ()), // sub_140fc17b0
    Replacement::draft(0x140fc1a30, "unknown_140fc1a30", fn_140fc1a30 as *const ()), // sub_140fc1a30
    Replacement::draft(0x140fc1aa0, "unknown_140fc1aa0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433352b8, 0x8, 0x20> as *const ()), // sub_140fc1aa0
    Replacement::draft(0x140fc1c20, "unknown_140fc1c20", tpl_381::<0x140fc1d80, 0x140fc20d0> as *const ()), // sub_140fc1c20
    Replacement::draft(0x140fc1d80, "unknown_140fc1d80", fn_140fc1d80 as *const ()), // sub_140fc1d80
    Replacement::draft(0x140fc20d0, "unknown_140fc20d0", fn_140fc20d0 as *const ()), // sub_140fc20d0
    Replacement::draft(0x140fc2400, "unknown_140fc2400", fn_140fc2400 as *const ()), // sub_140fc2400
    Replacement::draft(0x140fc2510, "unknown_140fc2510", fn_140fc2510 as *const ()), // sub_140fc2510
    Replacement::draft_throws(0x140fc26e0, "global_140fc26e0", fn_140fc26e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<shutdownDistantLoops>
    Replacement::draft(0x140fc29f0, "unknown_140fc29f0", tpl_1526::<0x1428bded8, 0x1410d6ca0, 0x140fc2fd0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140fc29f0
    Replacement::draft(0x140fc2c90, "unknown_140fc2c90", tpl_1369::<0x143ccc0e8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fc2c90
    Replacement::draft(0x140fc2fd0, "coregame_140fc2fd0", tpl_19618::<0x143ccc0ec, 0x143ccc0f0, 0x1411e9f50, 0x140fc35d0, 0x143ccc0e8, 0x1401fc320, 0x1420a28e0, 0x140fc3450, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143335856, 0x142646960, 0x143ccc110, 0x143ccc118, 0x143ccc11c, 0x143ccc11e, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::spatial_sound::query_removed::system>
    Replacement::draft(0x140fc3290, "unknown_140fc3290", tpl_6237::<0x20, 0x143ccc144, 0x8, 0x143c8d07c, 0x142407c80, 0x580f0, 0x141e61da0> as *const ()), // sub_140fc3290
    Replacement::draft_throws(0x140fc3450, "unknown_140fc3450", tpl_19619::<0x143ccc0e8, 0x5e308, 0x10, 0x8, 0x18, 0x141e61960, 0x140489dd0> as *const (), Throws::Discard), // sub_140fc3450
    Replacement::draft(0x140fc35d0, "unknown_140fc35d0", tpl_3188::<0x140fc36c0, 0x1433358f8, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fc35d0
    Replacement::draft(0x140fc3650, "unknown_140fc3650", tpl_19620::<0x8, 0x140489dd0> as *const ()), // sub_140fc3650
    Replacement::draft(0x140fc36c0, "unknown_140fc36c0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433358f8, 0x8, 0x20> as *const ()), // sub_140fc36c0
    Replacement::draft(0x140fc3820, "unknown_140fc3820", fn_140fc3820 as *const ()), // sub_140fc3820
    Replacement::draft_throws(0x140fc3940, "coregame_140fc3940", tpl_1584::<0x143ccc18c, 0x143ccc190, 0x1411e9f50, 0x140fc47d0, 0x140fc4940, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143335956, 0x142646960, 0x143ccc1b0, 0x143ccc1b8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::module::spatial::updateExisting>
    Replacement::draft_throws(0x140fc3b90, "coregame_140fc3b90", fn_140fc3b90 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::module::spatial::insertNewEntities>
    Replacement::draft(0x140fc3ee0, "coregame_140fc3ee0", tpl_1537::<0x143ccc1f0, 0x143ccc1f8, 0x1411e9f50, 0x143ccc290, 0x140215610, 0x1420a28e0, 0x140fc4a60, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143335b46, 0x142646960, 0x143ccc218, 0x143ccc220, 0x143ccc224, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::module::spatial::executeBucketMoves>
    Replacement::draft(0x140fc4180, "coregame_140fc4180", tpl_1095::<0x143ccc228, 0x14020e8f0> as *const ()), // sub_140fc4180
    Replacement::draft(0x140fc4360, "coregame_140fc4360", fn_140fc4360 as *const ()), // ecs::system::ecs::system::wrapper<coregame::module::spatial::cleanup>
    Replacement::draft(0x140fc4640, "unknown_140fc4640", fn_140fc4640 as *const ()), // sub_140fc4640
    Replacement::draft_throws(0x140fc47d0, "unknown_140fc47d0", fn_140fc47d0 as *const (), Throws::Discard), // sub_140fc47d0
    Replacement::draft(0x140fc4940, "unknown_140fc4940", tpl_1560::<0x143ccc2e8, 0x580e0, 0x20, 0x8, 0x143ccc290, 0x143cc7c80, 0x141e62590> as *const ()), // sub_140fc4940
    Replacement::draft(0x140fc49c0, "unknown_140fc49c0", fn_140fc49c0 as *const ()), // sub_140fc49c0
    Replacement::draft(0x140fc4b70, "unknown_140fc4b70", tpl_18942::<0x143ccc228, 0x5e308, 0x143ccc290, 0x580e0, 0x10, 0x8, 0x141e62250, 0x1401de830> as *const ()), // sub_140fc4b70
    Replacement::draft(0x140fc4e20, "unknown_140fc4e20", tpl_1991::<0x140fc4f50, 0x143336320, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fc4e20
    Replacement::draft(0x140fc4ee0, "unknown_140fc4ee0", fn_140fc4ee0 as *const ()), // sub_140fc4ee0
    Replacement::draft(0x140fc4f50, "unknown_140fc4f50", tpl_1997::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143336320, 0x8> as *const ()), // sub_140fc4f50
    Replacement::draft(0x140fc4ff0, "unknown_140fc4ff0", fn_140fc4ff0 as *const ()), // sub_140fc4ff0
    Replacement::draft(0x140fc5050, "unknown_140fc5050", fn_140fc5050 as *const ()), // sub_140fc5050
    Replacement::draft(0x140fc5270, "unknown_140fc5270", fn_140fc5270 as *const ()), // sub_140fc5270
    Replacement::draft(0x140fc5340, "unknown_140fc5340", fn_140fc5340 as *const ()), // sub_140fc5340
    Replacement::draft(0x140fc5520, "unknown_140fc5520", fn_140fc5520 as *const ()), // sub_140fc5520
    Replacement::draft(0x140fc5740, "unknown_140fc5740", fn_140fc5740 as *const ()), // sub_140fc5740
    Replacement::draft(0x140fc5a40, "unknown_140fc5a40", fn_140fc5a40 as *const ()), // sub_140fc5a40
    Replacement::draft(0x140fc5d40, "unknown_140fc5d40", fn_140fc5d40 as *const ()), // sub_140fc5d40
    Replacement::draft(0x140fc5e40, "unknown_140fc5e40", fn_140fc5e40 as *const ()), // sub_140fc5e40
    Replacement::draft(0x140fc5ef0, "unknown_140fc5ef0", tpl_1171::<0x58, 0x60, 0x143ccc410, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140fc5ef0
    Replacement::draft_throws(0x140fc5fd0, "coregame_140fc5fd0", fn_140fc5fd0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::updateActiveStatus>
    Replacement::draft_throws(0x140fc63a0, "coregame_140fc63a0", fn_140fc63a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::updateSoundSpawner>
    Replacement::draft_throws(0x140fc6740, "coregame_140fc6740", tpl_1480::<0x1428be048, 0x1410d6ca0, 0x140fc7ee0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140fc6740
    Replacement::draft(0x140fc69e0, "coregame_140fc69e0", tpl_1096::<0x143ccc404, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fc69e0
    Replacement::draft(0x140fc6d10, "coregame_140fc6d10", tpl_3077::<0x143ccc408, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fc6d10
    Replacement::draft(0x140fc7040, "coregame_140fc7040", tpl_1094::<0x143ccc40c, 0x140fc8350, 0x14020e8f0> as *const ()), // sub_140fc7040
    Replacement::draft(0x140fc7240, "coregame_140fc7240", tpl_1095::<0x143ccc410, 0x14020e8f0> as *const ()), // sub_140fc7240
    Replacement::draft(0x140fc7420, "coregame_140fc7420", tpl_1094::<0x143ccc418, 0x140fc8500, 0x14020e8f0> as *const ()), // sub_140fc7420
    Replacement::draft(0x140fc7620, "coregame_140fc7620", tpl_1466::<0x143ccc41c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fc7620
    Replacement::draft_throws(0x140fc78f0, "coregame_140fc78f0", tpl_19637::<0x143ccc420, 0x143ccc428, 0x1411e9f50, 0x140fcabc0, 0x143ccc81c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140fc9d10, 0x140fc93e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143337096, 0x142646960, 0x143ccc438, 0x143ccc440, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::streamIn>
    Replacement::draft_throws(0x140fc7be0, "coregame_140fc7be0", tpl_19638::<0x143ccc444, 0x143ccc448, 0x1411e9f50, 0x140fcaab0, 0x143ccc818, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140fc9ed0, 0x140fc9510, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143336e66, 0x142646960, 0x143ccc458, 0x143ccc460, 0x143ccc462, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::streamOut>
    Replacement::draft_throws(0x140fc7ee0, "coregame_140fc7ee0", tpl_5490::<0x143ccc464, 0x143ccc468, 0x1411e9f50, 0x140fca4f0, 0x143ccc40c, 0x1401fc320, 0x14027c6a0, 0x1420a28e0, 0x140fca030, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433374c6, 0x142646960, 0x143ccc488, 0x143ccc48c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::removedSoundSpawner>
    Replacement::draft(0x140fc81a0, "unknown_140fc81a0", tpl_4160::<0x8, 0x140fc7420, 0x140fc7620> as *const ()), // sub_140fc81a0
    Replacement::draft(0x140fc8350, "unknown_140fc8350", fn_140fc8350 as *const ()), // sub_140fc8350
    Replacement::draft(0x140fc8500, "unknown_140fc8500", fn_140fc8500 as *const ()), // sub_140fc8500
    Replacement::draft_throws(0x140fc8730, "unknown_140fc8730", tpl_5996::<0x88, 0x10088, 0x140fc8d40, 0x140fc87e0> as *const (), Throws::Discard), // sub_140fc8730
    Replacement::draft_throws(0x140fc87e0, "unknown_140fc87e0", tpl_3284::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141e651e0> as *const (), Throws::Discard), // sub_140fc87e0
    Replacement::draft_throws(0x140fc88f0, "unknown_140fc88f0", tpl_19641::<0x88, 0x10088, 0x140fc8aa0, 0x140fc8990> as *const (), Throws::Discard), // sub_140fc88f0
    Replacement::draft_throws(0x140fc8990, "unknown_140fc8990", tpl_4138::<0x18, 0x20, 0x40, 0x8, 0x141e64300> as *const (), Throws::Discard), // sub_140fc8990
    Replacement::draft(0x140fc8d40, "unknown_140fc8d40", tpl_5037::<0x8, 0x10, 0x18, 0x140fc8df0, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50> as *const ()), // sub_140fc8d40
    Replacement::draft(0x140fc8df0, "unknown_140fc8df0", tpl_5038::<0x143ccc40c, 0x142489ee0, 0x143cc6a20, 0x143ccc418, 0x143c8d120, 0x143c8fa04, 0x143ccc41c, 0x143ccc410> as *const ()), // sub_140fc8df0
    Replacement::draft(0x140fc8f90, "unknown_140fc8f90", fn_140fc8f90 as *const ()), // sub_140fc8f90
    Replacement::draft_throws(0x140fc8fd0, "unknown_140fc8fd0", fn_140fc8fd0 as *const (), Throws::Discard), // sub_140fc8fd0
    Replacement::draft_throws(0x140fc91d0, "unknown_140fc91d0", tpl_78029::<0x140fc8b80, 0x141e64300> as *const (), Throws::Discard), // sub_140fc91d0
    Replacement::draft(0x140fc93e0, "unknown_140fc93e0", tpl_62998::<0x140fc95f0, 0x141e659b0> as *const ()), // sub_140fc93e0
    Replacement::draft(0x140fc9510, "unknown_140fc9510", tpl_78030::<0x88, 0x10088, 0x143ccc410, 0x180a8, 0x18090, 0x142489ee0, 0x141e65860> as *const ()), // sub_140fc9510
    Replacement::draft(0x140fc9d10, "unknown_140fc9d10", tpl_62999::<0x140fc95f0, 0x141e659b0> as *const ()), // sub_140fc9d10
    Replacement::draft(0x140fc9ed0, "unknown_140fc9ed0", tpl_78031::<0x143ccc410, 0x142489ee0, 0x141e65860> as *const ()), // sub_140fc9ed0
    Replacement::draft_throws(0x140fca030, "unknown_140fca030", tpl_5499::<0x143ccc40c, 0x5e308, 0x10, 0x8, 0x18, 0x141e64240, 0x140fca8f0> as *const (), Throws::Discard), // sub_140fca030
    Replacement::draft(0x140fca0d0, "unknown_140fca0d0", fn_140fca0d0 as *const ()), // sub_140fca0d0
    Replacement::draft(0x140fca330, "unknown_140fca330", fn_140fca330 as *const ()), // sub_140fca330
    Replacement::draft(0x140fca4f0, "unknown_140fca4f0", tpl_3188::<0x140fcaa10, 0x143338688, 0x140fca8f0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fca4f0
    Replacement::draft(0x140fca760, "unknown_140fca760", fn_140fca760 as *const ()), // sub_140fca760
    Replacement::draft(0x140fca8f0, "unknown_140fca8f0", fn_140fca8f0 as *const ()), // sub_140fca8f0
    Replacement::draft(0x140fcaa10, "unknown_140fcaa10", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143338688, 0x8, 0x20> as *const ()), // sub_140fcaa10
    Replacement::draft(0x140fcabc0, "unknown_140fcabc0", fn_140fcabc0 as *const ()), // sub_140fcabc0
    Replacement::draft(0x140fcafa0, "unknown_140fcafa0", fn_140fcafa0 as *const ()), // sub_140fcafa0
    Replacement::draft(0x140fcb050, "coregame_140fcb050", fn_140fcb050 as *const ()), // coregame::SoundManager::coregame::SoundManager::vfunc_0
    Replacement::draft(0x140fcb150, "unknown_140fcb150", fn_140fcb150 as *const ()), // sub_140fcb150
    Replacement::draft_throws(0x140fcb2b0, "std_140fcb2b0", fn_140fcb2b0 as *const (), Throws::Discard), // sub_140fcb2b0
    Replacement::draft(0x140fcb5e0, "std_140fcb5e0", fn_140fcb5e0 as *const ()), // sub_140fcb5e0
    Replacement::draft(0x140fcb650, "std_140fcb650", tpl_1::<0x14388f848> as *const ()), // std::_Func_impl_no_alloc<int (*)(int,void*),int,int,void*>::std::_Func_impl_no_alloc<int (*)(int,void*),int,int,void*>::
    Replacement::draft(0x140fcb660, "std_140fcb660", fn_140fcb660 as *const ()), // std::_Func_impl_no_alloc<int (*)(int,void*),int,int,void*>::std::_Func_impl_no_alloc<int (*)(int,void*),int,int,void*>::
    Replacement::draft(0x140fcb670, "std_140fcb670", tpl_550::<0x143338778, 0x8> as *const ()), // std::_Func_impl_no_alloc<int (*)(int,void*),int,int,void*>::std::_Func_impl_no_alloc<int (*)(int,void*),int,int,void*>::
    Replacement::draft(0x140fcb690, "std_140fcb690", tpl_1::<0x14388f868> as *const ()), // std::_Func_impl_no_alloc<int (*)(int,int),int,int,int>::std::_Func_impl_no_alloc<int (*)(int,int),int,int,int>::vfunc_3
    Replacement::draft(0x140fcb6a0, "std_140fcb6a0", fn_140fcb6a0 as *const ()), // std::_Func_impl_no_alloc<int (*)(int,int),int,int,int>::std::_Func_impl_no_alloc<int (*)(int,int),int,int,int>::vfunc_2
    Replacement::draft(0x140fcb6b0, "std_140fcb6b0", tpl_550::<0x143338740, 0x8> as *const ()), // std::_Func_impl_no_alloc<int (*)(int,int),int,int,int>::std::_Func_impl_no_alloc<int (*)(int,int),int,int,int>::vfunc_0
    Replacement::draft(0x140fcb6d0, "std_140fcb6d0", tpl_1::<0x14388f820> as *const ()), // std::_Func_impl_no_alloc<int (*)(int,r::WString&),int,int,r::WString&>::std::_Func_impl_no_alloc<int (*)(int,r::WString&
    Replacement::draft(0x140fcb6e0, "std_140fcb6e0", fn_140fcb6e0 as *const ()), // std::_Func_impl_no_alloc<int (*)(int,r::WString&),int,int,r::WString&>::std::_Func_impl_no_alloc<int (*)(int,r::WString&
    Replacement::draft(0x140fcb6f0, "std_140fcb6f0", tpl_550::<0x1433387b0, 0x8> as *const ()), // std::_Func_impl_no_alloc<int (*)(int,r::WString&),int,int,r::WString&>::std::_Func_impl_no_alloc<int (*)(int,r::WString&
    Replacement::draft(0x140fcb710, "unknown_140fcb710", tpl_4842::<0x14397da68, 0x143c8d08c, 0x8, 0x580e0, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x143cba178, 0x180a8, 0x18090, 0x142489ee0, 0x140fce320, 0x1426467e8, 0x140fcb7f5> as *const ()), // sub_140fcb710
    Replacement::draft(0x140fcb810, "unknown_140fcb810", tpl_19657::<0x14397da68, 0x143c8d08c, 0x1401fa730, 0x143cc69b0, 0x142489ee0, 0x1426467e8, 0x140fcb933> as *const ()), // sub_140fcb810
    Replacement::draft(0x140fcb950, "unknown_140fcb950", fn_140fcb950 as *const ()), // sub_140fcb950
    Replacement::draft(0x140fcba90, "unknown_140fcba90", fn_140fcba90 as *const ()), // sub_140fcba90
    Replacement::draft_throws(0x140fcbbd0, "coregame_140fcbbd0", fn_140fcbbd0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::updateActivation>
    Replacement::draft_throws(0x140fcbf30, "coregame_140fcbf30", tpl_1480::<0x1428be4c0, 0x1410d6ca0, 0x140fcdf00, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140fcbf30
    Replacement::draft(0x140fcc1d0, "coregame_140fcc1d0", tpl_1094::<0x143cc69b0, 0x140fce3b0, 0x14020e8f0> as *const ()), // sub_140fcc1d0
    Replacement::draft(0x140fcc3d0, "coregame_140fcc3d0", tpl_1095::<0x143ccc8e4, 0x14020e8f0> as *const ()), // sub_140fcc3d0
    Replacement::draft(0x140fcc5b0, "coregame_140fcc5b0", tpl_1095::<0x143cc6a14, 0x14020e8f0> as *const ()), // sub_140fcc5b0
    Replacement::draft(0x140fcc790, "coregame_140fcc790", tpl_3077::<0x143ccc8e8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fcc790
    Replacement::draft(0x140fccac0, "coregame_140fccac0", tpl_1094::<0x143cc6a20, 0x140fce550, 0x14020e8f0> as *const ()), // sub_140fccac0
    Replacement::draft(0x140fcccc0, "coregame_140fcccc0", tpl_1466::<0x143ccc8ec, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fcccc0
    Replacement::draft(0x140fccf90, "coregame_140fccf90", tpl_1095::<0x143cc6a80, 0x14020e8f0> as *const ()), // sub_140fccf90
    Replacement::draft(0x140fcd170, "coregame_140fcd170", fn_140fcd170 as *const ()), // sub_140fcd170
    Replacement::draft_throws(0x140fcd380, "coregame_140fcd380", tpl_19662::<0x143ccc8f0, 0x143ccc900, 0x140fce9c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143338826, 0x142646960, 0x143ccc920, 0x143ccc940, 0x143ccc950, 0x143ccc952, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::SoundComponent>>
    Replacement::draft_throws(0x140fcd5b0, "coregame_140fcd5b0", tpl_19663::<0x143ccc954, 0x143ccc958, 0x1411e9f50, 0x140fd08b0, 0x143cccc70, 0x1420af120, 0x14027c6a0, 0x143cada18, 0x140215610, 0x1420a28e0, 0x140fcf9e0, 0x140fcf100, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433391b6, 0x142646960, 0x143ccc968, 0x143ccc970, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::streamIn>
    Replacement::draft_throws(0x140fcd8d0, "coregame_140fcd8d0", tpl_5529::<0x143ccc974, 0x143ccc978, 0x1411e9f50, 0x140fd0770, 0x143cccc6c, 0x1420af120, 0x14027c6a0, 0x143cada18, 0x140215610, 0x1420a28e0, 0x140fcfbc0, 0x140fcf250, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143338f06, 0x142646960, 0x143ccc998, 0x143ccc9a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::streamInWithoutAudioSource>
    Replacement::draft_throws(0x140fcdc00, "coregame_140fcdc00", tpl_19638::<0x143ccc9a4, 0x143ccc9a8, 0x1411e9f50, 0x140fd0660, 0x143cccc68, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140fcfd40, 0x140fcf360, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143339636, 0x142646960, 0x143ccc9b8, 0x143ccc9c0, 0x143ccc9c2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::streamOut>
    Replacement::draft_throws(0x140fcdf00, "coregame_140fcdf00", fn_140fcdf00 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::audio::removedSoundEmitter>
    Replacement::draft(0x140fce320, "unknown_140fce320", tpl_4008::<0x30, 0x143cba178, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x140fce3a3> as *const ()), // sub_140fce320
    Replacement::draft(0x140fce3b0, "unknown_140fce3b0", tpl_19665::<0x1401e6d50, 0x140470810, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_140fce3b0
    Replacement::draft(0x140fce550, "unknown_140fce550", fn_140fce550 as *const ()), // sub_140fce550
    Replacement::draft(0x140fce7f0, "unknown_140fce7f0", tpl_19667::<0x88, 0x10088, 0x140fceb80, 0x140fce8a0> as *const ()), // sub_140fce7f0
    Replacement::draft(0x140fce8a0, "unknown_140fce8a0", tpl_19668::<0x18, 0x20, 0x40, 0x60, 0x8, 0x141e69330> as *const ()), // sub_140fce8a0
    Replacement::draft_throws(0x140fce9c0, "unknown_140fce9c0", tpl_3011::<0x1411e9f50, 0x140fd0a30, 0x143cccc74, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140fcf970, 0x1401de410> as *const (), Throws::Discard), // sub_140fce9c0
    Replacement::draft(0x140fceb80, "unknown_140fceb80", fn_140fceb80 as *const ()), // sub_140fceb80
    Replacement::draft(0x140fcec70, "unknown_140fcec70", fn_140fcec70 as *const ()), // sub_140fcec70
    Replacement::draft(0x140fcee80, "unknown_140fcee80", tpl_1124::<0x148> as *const ()), // sub_140fcee80
    Replacement::draft(0x140fceec0, "unknown_140fceec0", fn_140fceec0 as *const ()), // sub_140fceec0
    Replacement::draft_throws(0x140fcf100, "unknown_140fcf100", tpl_78054::<0x143cada18, 0x140fcf540, 0x141e69d00> as *const (), Throws::Discard), // sub_140fcf100
    Replacement::draft_throws(0x140fcf250, "unknown_140fcf250", tpl_78055::<0x143cada18, 0x143cc6a20, 0x142489ee0, 0x141e69ae0> as *const (), Throws::Discard), // sub_140fcf250
    Replacement::draft(0x140fcf360, "unknown_140fcf360", tpl_64022::<0x88, 0x10088, 0x140fcf440, 0x141e695f0> as *const ()), // sub_140fcf360
    Replacement::draft(0x140fcf970, "unknown_140fcf970", tpl_3028::<0x143cccc74, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x140fcd170> as *const ()), // sub_140fcf970
    Replacement::draft_throws(0x140fcf9e0, "unknown_140fcf9e0", tpl_78056::<0x143cada18, 0x140fcf540, 0x141e69d00> as *const (), Throws::Discard), // sub_140fcf9e0
    Replacement::draft_throws(0x140fcfbc0, "unknown_140fcfbc0", tpl_78057::<0x143cada18, 0x143cc6a20, 0x142489ee0, 0x141e69ae0> as *const (), Throws::Discard), // sub_140fcfbc0
    Replacement::draft(0x140fcfd40, "unknown_140fcfd40", tpl_64023::<0x143cc69b0, 0x142489ee0, 0x143cc6a20, 0x141e695f0> as *const ()), // sub_140fcfd40
    Replacement::draft(0x140fcfef0, "unknown_140fcfef0", fn_140fcfef0 as *const ()), // sub_140fcfef0
    Replacement::draft(0x140fcffa0, "unknown_140fcffa0", fn_140fcffa0 as *const ()), // sub_140fcffa0
    Replacement::draft(0x140fd01f0, "unknown_140fd01f0", tpl_3188::<0x140fd05c0, 0x14333a710, 0x140650800, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fd01f0
    Replacement::draft(0x140fd0270, "unknown_140fd0270", tpl_4348::<0x10, 0x18, 0x140650800> as *const ()), // sub_140fd0270
    Replacement::draft(0x140fd0440, "unknown_140fd0440", fn_140fd0440 as *const ()), // sub_140fd0440
    Replacement::draft(0x140fd05c0, "unknown_140fd05c0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14333a710, 0x8, 0x20> as *const ()), // sub_140fd05c0
    Replacement::draft(0x140fd0fe0, "unknown_140fd0fe0", fn_140fd0fe0 as *const ()), // sub_140fd0fe0
    Replacement::draft_throws(0x140fd11c0, "unknown_140fd11c0", tpl_1480::<0x1428be5f8, 0x1410d6ca0, 0x140fd1c90, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_140fd11c0
    Replacement::draft_throws(0x140fd1460, "unknown_140fd1460", tpl_1480::<0x1428be5f8, 0x1410d6ca0, 0x140fd1f90, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_140fd1460
    Replacement::draft(0x140fd1700, "unknown_140fd1700", tpl_1097::<0x143cccd90, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fd1700
    Replacement::draft(0x140fd1a30, "unknown_140fd1a30", tpl_1095::<0x143cccd8c, 0x14020e8f0> as *const ()), // sub_140fd1a30
    Replacement::draft(0x140fd1c10, "unknown_140fd1c10", tpl_19676::<0x8, 0x140fd1700, 0x140fd1a30> as *const ()), // sub_140fd1c10
    Replacement::draft_throws(0x140fd1c90, "coregame_140fd1c90", tpl_3086::<0x143cccd94, 0x143cccd98, 0x1411e9f50, 0x140fd2ec0, 0x143cccea8, 0x1420af120, 0x143c8d08c, 0x140215610, 0x1420a28e0, 0x140fd2920, 0x140fd22f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333a7b6, 0x142646960, 0x143cccdb8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::skeletonslave::attachToParent>
    Replacement::draft_throws(0x140fd1f90, "coregame_140fd1f90", fn_140fd1f90 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::skeletonslave::teleportSlave>
    Replacement::draft(0x140fd22f0, "unknown_140fd22f0", tpl_5377::<0x143c8d08c, 0x142407c80, 0x140fd2640, 0x580f0, 0x141e6b0d0> as *const ()), // sub_140fd22f0
    Replacement::draft(0x140fd23f0, "unknown_140fd23f0", tpl_61053::<0x88, 0x10088, 0x143cccea0, 0x20, 0x8, 0x140fd2540, 0x141e6aed0> as *const ()), // sub_140fd23f0
    Replacement::draft(0x140fd2920, "unknown_140fd2920", tpl_5378::<0x143c8d08c, 0x142407c80, 0x143c8dbd4, 0x142489ee0, 0x143cccd90, 0x141e6b0d0> as *const ()), // sub_140fd2920
    Replacement::draft(0x140fd2af0, "unknown_140fd2af0", tpl_61071::<0x143cccea0, 0x143cccd90, 0x142489ee0, 0x143c8d074, 0x141e6aed0> as *const ()), // sub_140fd2af0
    Replacement::draft(0x140fd3170, "unknown_140fd3170", tpl_10084::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8> as *const ()), // sub_140fd3170
    Replacement::draft(0x140fd31f0, "unknown_140fd31f0", tpl_2312::<0x18, 0x10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x8> as *const ()), // sub_140fd31f0
    Replacement::draft_throws(0x140fd3360, "coregame_140fd3360", tpl_5175::<0x143ccd048, 0x143ccd118, 0x140fd4180, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333b276, 0x142646960, 0x143ccd138, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::skeleton::gatherResources>
    Replacement::draft(0x140fd3560, "coregame_140fd3560", tpl_1094::<0x143cccfe0, 0x14068b330, 0x14020e8f0> as *const ()), // sub_140fd3560
    Replacement::draft(0x140fd3760, "coregame_140fd3760", tpl_1094::<0x143c8ec20, 0x140fd4380, 0x14020e8f0> as *const ()), // sub_140fd3760
    Replacement::draft(0x140fd3960, "coregame_140fd3960", tpl_1368::<0x143c9a318, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fd3960
    Replacement::draft_throws(0x140fd3c90, "coregame_140fd3c90", fn_140fd3c90 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::skeleton::streamIn>
    Replacement::draft_throws(0x140fd3ea0, "coregame_140fd3ea0", tpl_1963::<0x143ccd04c, 0x143ccd078, 0x1411e9f50, 0x140fd4bd0, 0x143cccfd4, 0x1420af120, 0x1420a28e0, 0x140fd49d0, 0x140fd46f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333b436, 0x142646960, 0x143ccd088, 0x143ccd090, 0x143ccd094, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::skeleton::streamOut>
    Replacement::draft_throws(0x140fd4180, "unknown_140fd4180", tpl_3011::<0x1411e9f50, 0x140fd4ae0, 0x143ccd020, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140fd47b0, 0x1401de410> as *const (), Throws::Discard), // sub_140fd4180
    Replacement::draft(0x140fd4340, "unknown_140fd4340", tpl_19679::<0x8, 0x140fd3960> as *const ()), // sub_140fd4340
    Replacement::draft(0x140fd4380, "unknown_140fd4380", tpl_19680::<0x1401e6d50, 0x140fd46c0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_140fd4380
    Replacement::draft_throws(0x140fd4510, "unknown_140fd4510", tpl_3697::<0x1411e9f50, 0x140fd4cc0, 0x143cccfd0, 0x1420af120, 0x140215750, 0x143cada18, 0x140215610, 0x1420a28e0, 0x140fd4920, 0x1401de410> as *const (), Throws::Discard), // sub_140fd4510
    Replacement::draft(0x140fd46c0, "unknown_140fd46c0", tpl_1955::<0x10, 0x1402359b0> as *const ()), // sub_140fd46c0
    Replacement::draft(0x140fd46f0, "unknown_140fd46f0", tpl_1126::<0x142407c80, 0x580f0, 0x141e6b9e0> as *const ()), // sub_140fd46f0
    Replacement::draft(0x140fd47b0, "unknown_140fd47b0", tpl_3028::<0x143ccd020, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141e6b7c0> as *const ()), // sub_140fd47b0
    Replacement::draft_throws(0x140fd4920, "unknown_140fd4920", fn_140fd4920 as *const (), Throws::Caught), // sub_140fd4920
    Replacement::draft(0x140fd49d0, "unknown_140fd49d0", tpl_1130::<0x142407c80, 0x141e6b9e0> as *const ()), // sub_140fd49d0
    Replacement::draft(0x140fd4fa0, "coregame_140fd4fa0", fn_140fd4fa0 as *const ()), // coregame::CoreProgramFlow::ActionStartLoadingWorldParameters::coregame::CoreProgramFlow::ActionStartLoadingWorldParamete
    Replacement::draft(0x140fd5070, "coregame_140fd5070", fn_140fd5070 as *const ()), // coregame::CoreProgramFlow::ActionStartLoadingEpisodeParameters::coregame::CoreProgramFlow::ActionStartLoadingEpisodePara
    Replacement::draft(0x140fd5100, "coregame_140fd5100", fn_140fd5100 as *const ()), // coregame::CoreProgramFlow::ActionStartReloadingEpisodeParameters::coregame::CoreProgramFlow::ActionStartReloadingEpisode
    Replacement::draft(0x140fd5170, "coregame_140fd5170", fn_140fd5170 as *const ()), // coregame::CoreProgramFlow::ActionRequestLoadWorldParameters::coregame::CoreProgramFlow::ActionRequestLoadWorldParameters
    Replacement::draft(0x140fd51d0, "coregame_140fd51d0", fn_140fd51d0 as *const ()), // sub_140fd51d0
    Replacement::draft(0x140fd5210, "coregame_140fd5210", tpl_3538::<0x14240ab10, 0x8, 0x143079e70, 0x141834d30> as *const ()), // coregame::CoreProgramFlow::ActionStartLoadingEpisodeParameters::coregame::CoreProgramFlow::ActionStartLoadingEpisodePara
    Replacement::draft(0x140fd5260, "coregame_140fd5260", fn_140fd5260 as *const ()), // sub_140fd5260
    Replacement::draft(0x140fd5310, "coregame_140fd5310", fn_140fd5310 as *const ()), // sub_140fd5310
    Replacement::draft_throws(0x140fd53d0, "coregame_140fd53d0", fn_140fd53d0 as *const (), Throws::Discard), // sub_140fd53d0
    Replacement::draft(0x140fd5530, "coregame_140fd5530", fn_140fd5530 as *const ()), // sub_140fd5530
    Replacement::draft(0x140fd56d0, "coregame_140fd56d0", tpl_1526::<0x1428bf040, 0x1410d6ca0, 0x140fd6780, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_140fd56d0
    Replacement::draft(0x140fd5970, "coregame_140fd5970", tpl_1094::<0x143ccd1c4, 0x140fd6b60, 0x14020e8f0> as *const ()), // sub_140fd5970
    Replacement::draft(0x140fd5b70, "coregame_140fd5b70", tpl_1094::<0x143ccd1c8, 0x140fd6ce0, 0x14020e8f0> as *const ()), // sub_140fd5b70
    Replacement::draft_throws(0x140fd5d70, "coregame_140fd5d70", tpl_5492::<0x143ccd1cc, 0x143ccd1d0, 0x1411e9f50, 0x140fd7c10, 0x143ccd3a0, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140fd7430, 0x140fd70a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333c0c6, 0x142646960, 0x143ccd1e0, 0x143ccd1e8, 0x143ccd1ec, 0x143ccd1ee, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::sdf_volume::streamIn>
    Replacement::draft_throws(0x140fd6050, "coregame_140fd6050", tpl_1896::<0x143ccd1f0, 0x143ccd1f8, 0x1411e9f50, 0x140fd7b20, 0x143ccd39c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x140fd7580, 0x140fd71a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333c426, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::sdf_volume::streamOut>
    Replacement::draft(0x140fd6330, "coregame_140fd6330", tpl_3593::<0x1401e7c10, 0x140fd6a60, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_140fd6330
    Replacement::draft_throws(0x140fd6540, "coregame_140fd6540", tpl_19691::<0x143ccd218, 0x143ccd220, 0x140fd6e70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333c5f6, 0x142646960, 0x143ccd240, 0x143ccd260, 0x143ccd270, 0x143ccd274, 0x143ccd276, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::SDFVolumeComponent>
    Replacement::draft(0x140fd6780, "coregame_140fd6780", fn_140fd6780 as *const ()), // ecs::system::ecs::system::wrapper<coregame::sdf_volume::removeSDFVolume>
    Replacement::draft(0x140fd6b60, "unknown_140fd6b60", tpl_2731::<0x1401e6d50, 0x140fd7030, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_140fd6b60
    Replacement::draft(0x140fd6ce0, "unknown_140fd6ce0", fn_140fd6ce0 as *const ()), // sub_140fd6ce0
    Replacement::draft_throws(0x140fd6e70, "unknown_140fd6e70", tpl_3011::<0x1411e9f50, 0x140fd7a30, 0x143ccd398, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140fd7670, 0x1401de410> as *const (), Throws::Discard), // sub_140fd6e70
    Replacement::draft(0x140fd7030, "unknown_140fd7030", fn_140fd7030 as *const ()), // sub_140fd7030
    Replacement::draft_throws(0x140fd70a0, "unknown_140fd70a0", tpl_62933::<0x143ccd1c8, 0x142489ee0, 0x141e6d420> as *const (), Throws::Discard), // sub_140fd70a0
    Replacement::draft(0x140fd71a0, "unknown_140fd71a0", tpl_1898::<0x88, 0x10088, 0x141e6d2f0> as *const ()), // sub_140fd71a0
    Replacement::draft_throws(0x140fd7430, "unknown_140fd7430", tpl_62934::<0x143ccd1c8, 0x142489ee0, 0x141e6d420> as *const (), Throws::Discard), // sub_140fd7430
    Replacement::draft(0x140fd7580, "unknown_140fd7580", tpl_61552::<0x8, 0x88, 0x10088, 0x141e6d2f0> as *const ()), // sub_140fd7580
    Replacement::draft(0x140fd7670, "unknown_140fd7670", tpl_3028::<0x143ccd398, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x140fd6330> as *const ()), // sub_140fd7670
    Replacement::draft(0x140fd76e0, "unknown_140fd76e0", tpl_19695::<0x143ccd1c4, 0x5e308, 0x10, 0x8, 0x18, 0x141e6d000, 0x140fd7860> as *const ()), // sub_140fd76e0
    Replacement::draft(0x140fd7770, "unknown_140fd7770", tpl_3188::<0x140fd7990, 0x14333ccb8, 0x140fd7860, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fd7770
    Replacement::draft(0x140fd77f0, "unknown_140fd77f0", tpl_19620::<0x8, 0x140fd7860> as *const ()), // sub_140fd77f0
    Replacement::draft(0x140fd7860, "unknown_140fd7860", fn_140fd7860 as *const ()), // sub_140fd7860
    Replacement::draft(0x140fd7990, "unknown_140fd7990", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14333ccb8, 0x8, 0x20> as *const ()), // sub_140fd7990
    Replacement::draft_throws(0x140fd7ec0, "coregame_140fd7ec0", fn_140fd7ec0 as *const (), Throws::Discard), // coregame::ScrollingTextWindow::coregame::ScrollingTextWindow::vfunc_0
    Replacement::draft(0x140fd8010, "unknown_140fd8010", fn_140fd8010 as *const ()), // sub_140fd8010
    Replacement::draft(0x140fd8080, "unknown_140fd8080", fn_140fd8080 as *const ()), // sub_140fd8080
    Replacement::draft(0x140fd80f0, "unknown_140fd80f0", tpl_64713::<0x19, 0x10, 0x140fd80f0, 0x124, 0x14240ab10, 0x1c, 0x1401e66f0> as *const ()), // sub_140fd80f0
    Replacement::draft(0x140fd8180, "unknown_140fd8180", fn_140fd8180 as *const ()), // sub_140fd8180
    Replacement::draft(0x140fd82b0, "unknown_140fd82b0", tpl_1808::<0x8, 0x14240ab10, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140fd82b0
    Replacement::draft(0x140fd8380, "unknown_140fd8380", tpl_5539::<0x8, 0x140fd8850, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140fd8380
    Replacement::draft(0x140fd8450, "unknown_140fd8450", fn_140fd8450 as *const ()), // sub_140fd8450
    Replacement::draft(0x140fd8670, "unknown_140fd8670", fn_140fd8670 as *const ()), // sub_140fd8670
    Replacement::draft(0x140fd8810, "unknown_140fd8810", tpl_1819::<0x8, 0x14240ab10> as *const ()), // sub_140fd8810
    Replacement::draft(0x140fd8850, "unknown_140fd8850", fn_140fd8850 as *const ()), // sub_140fd8850
    Replacement::draft(0x140fd8920, "concurrency_140fd8920", tpl_390::<0x1426680a0, 0x8, 0x142646338, 0x1426683c0> as *const ()), // sub_140fd8920
    Replacement::draft(0x140fd8970, "unknown_140fd8970", tpl_390::<0x1426680a0, 0x8, 0x142646338, 0x14333cd00> as *const ()), // sub_140fd8970
    Replacement::draft(0x140fd89c0, "unknown_140fd89c0", tpl_402::<0x1426680a0, 0x8, 0x142646338, 0x14333cd00> as *const ()), // sub_140fd89c0
    Replacement::draft(0x140fd8a00, "unknown_140fd8a00", tpl_402::<0x1426680a0, 0x8, 0x142646338, 0x1426683c0> as *const ()), // sub_140fd8a00
    Replacement::draft_throws(0x140fd8a40, "coregame_140fd8a40", fn_140fd8a40 as *const (), Throws::Discard), // sub_140fd8a40
    Replacement::draft(0x140fd8ac0, "coregame_140fd8ac0", fn_140fd8ac0 as *const ()), // sub_140fd8ac0
    Replacement::draft(0x140fd8cb0, "coregame_140fd8cb0", fn_140fd8cb0 as *const ()), // sub_140fd8cb0
    Replacement::draft(0x140fd9030, "coregame_140fd9030", fn_140fd9030 as *const ()), // sub_140fd9030
    Replacement::draft_throws(0x140fd90b0, "coregame_140fd90b0", tpl_4804::<0x143cce4c8, 0x143cce4d0, 0x140fdc500, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333d276, 0x142646960, 0x143cce4f0, 0x143cce4f8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::scattering::handleRemovedRegistry>
    Replacement::draft_throws(0x140fd92c0, "coregame_140fd92c0", tpl_3592::<0x143cce4fc, 0x143cce500, 0x1411e9f50, 0x140fe14d0, 0x143cce9cc, 0x1420af120, 0x140fde980, 0x1420a28e0, 0x140fdf1f0, 0x140fde220, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333cdb6, 0x142646960, 0x143cce520, 0x143cce528, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::scattering::cleanupOldChildren>
    Replacement::draft_throws(0x140fd95c0, "coregame_140fd95c0", tpl_1480::<0x1428bf290, 0x1410d6ca0, 0x140fdcc10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140fd95c0
    Replacement::draft_throws(0x140fd9860, "coregame_140fd9860", fn_140fd9860 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::scattering::registerNewEntities>
    Replacement::draft(0x140fd9bc0, "coregame_140fd9bc0", tpl_1095::<0x143cce568, 0x14020e8f0> as *const ()), // sub_140fd9bc0
    Replacement::draft(0x140fd9da0, "coregame_140fd9da0", tpl_1095::<0x143cce56c, 0x14020e8f0> as *const ()), // sub_140fd9da0
    Replacement::draft(0x140fd9f80, "coregame_140fd9f80", tpl_1095::<0x143cce570, 0x14020e8f0> as *const ()), // sub_140fd9f80
    Replacement::draft(0x140fda160, "coregame_140fda160", tpl_1095::<0x143cce574, 0x14020e8f0> as *const ()), // sub_140fda160
    Replacement::draft(0x140fda340, "coregame_140fda340", tpl_1369::<0x143c8fa04, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fda340
    Replacement::draft(0x140fda680, "coregame_140fda680", tpl_1095::<0x143c8d694, 0x14020e8f0> as *const ()), // sub_140fda680
    Replacement::draft(0x140fda860, "coregame_140fda860", tpl_3077::<0x143c8d87c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fda860
    Replacement::draft(0x140fdab90, "coregame_140fdab90", tpl_1368::<0x143c9a620, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fdab90
    Replacement::draft(0x140fdaec0, "coregame_140fdaec0", tpl_1369::<0x143cce578, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fdaec0
    Replacement::draft(0x140fdb200, "coregame_140fdb200", tpl_1095::<0x143cce57c, 0x14020e8f0> as *const ()), // sub_140fdb200
    Replacement::draft(0x140fdb3e0, "coregame_140fdb3e0", tpl_1094::<0x143cce580, 0x140fdd500, 0x14020e8f0> as *const ()), // sub_140fdb3e0
    Replacement::draft(0x140fdb5e0, "coregame_140fdb5e0", fn_140fdb5e0 as *const ()), // sub_140fdb5e0
    Replacement::draft(0x140fdb7e0, "coregame_140fdb7e0", tpl_1094::<0x143cce55c, 0x140fdd680, 0x14020e8f0> as *const ()), // sub_140fdb7e0
    Replacement::draft(0x140fdb9e0, "coregame_140fdb9e0", fn_140fdb9e0 as *const ()), // sub_140fdb9e0
    Replacement::draft(0x140fdbcf0, "coregame_140fdbcf0", tpl_1095::<0x143cce584, 0x14020e8f0> as *const ()), // sub_140fdbcf0
    Replacement::draft(0x140fdbed0, "coregame_140fdbed0", tpl_1094::<0x143cce588, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_140fdbed0
    Replacement::draft(0x140fdc0d0, "coregame_140fdc0d0", tpl_1094::<0x143cce560, 0x140fddf20, 0x14020e8f0> as *const ()), // sub_140fdc0d0
    Replacement::draft(0x140fdc2d0, "coregame_140fdc2d0", tpl_1094::<0x143cce564, 0x140fde0a0, 0x14020e8f0> as *const ()), // sub_140fdc2d0
    Replacement::draft_throws(0x140fdc500, "coregame_140fdc500", tpl_4807::<0x1411e9f50, 0x140fdff50, 0x143cce55c, 0x1401fc320, 0x140fe1630, 0x143cce9d0, 0x1420af120, 0x140215750, 0x140fde980, 0x1420a28e0, 0x140fdf050, 0x1401de410> as *const (), Throws::Discard), // sub_140fdc500
    Replacement::draft_throws(0x140fdc6f0, "coregame_140fdc6f0", tpl_4388::<0x143cce58c, 0x143cce590, 0x1411e9f50, 0x140fe1d60, 0x143ccec44, 0x1420af120, 0x1420a28e0, 0x140fe0f90, 0x140fdf3a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333db26, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::scattering::streamOut>
    Replacement::draft_throws(0x140fdc9b0, "coregame_140fdc9b0", tpl_1484::<0x143cce5b0, 0x143cce5b8, 0x1411e9f50, 0x140fdf460, 0x140fe10a0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333dd16, 0x142646960, 0x143cce5c8, 0x143cce5d0, 0x143cce5d4, 0x143cce5d6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::scattering::streamIn>
    Replacement::draft_throws(0x140fdcc10, "coregame_140fdcc10", tpl_3062::<0x143cce5d8, 0x143cce5e0, 0x1411e9f50, 0x140fdf5d0, 0x140fe1160, 0x140fdf7b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333d4b6, 0x142646960, 0x143cce600, 0x143cce608, 0x143cce60c, 0x143cce60e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::scattering::streamChildrenByDistance>
    Replacement::draft(0x140fdce70, "coregame_140fdce70", tpl_19710::<0x1401e7c10, 0x140fdd400, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_140fdce70
    Replacement::draft_throws(0x140fdd080, "coregame_140fdd080", tpl_6195::<0x143cce610, 0x143cce620, 0x140fde4e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333d936, 0x142646960, 0x143cce640, 0x143cce660, 0x143cce670, 0x143cce678, 0x143cce67a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::ScatterPointsCompon
    Replacement::draft_throws(0x140fdd2c0, "unknown_140fdd2c0", fn_140fdd2c0 as *const (), Throws::Discard), // sub_140fdd2c0
    Replacement::draft(0x140fdd500, "unknown_140fdd500", tpl_4368::<0x1401e6d50, 0x140fdee30, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140fdea60> as *const ()), // sub_140fdd500
    Replacement::draft(0x140fdd680, "unknown_140fdd680", fn_140fdd680 as *const ()), // sub_140fdd680
    Replacement::draft(0x140fdd820, "unknown_140fdd820", fn_140fdd820 as *const ()), // sub_140fdd820
    Replacement::draft(0x140fdda20, "unknown_140fdda20", fn_140fdda20 as *const ()), // sub_140fdda20
    Replacement::draft(0x140fddf20, "unknown_140fddf20", fn_140fddf20 as *const ()), // sub_140fddf20
    Replacement::draft(0x140fde0a0, "unknown_140fde0a0", tpl_19376::<0x1401e6d50, 0x140fdf020, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140fde8d0> as *const ()), // sub_140fde0a0
    Replacement::draft(0x140fde220, "unknown_140fde220", tpl_4341::<0x143cce9c4, 0x142407c80, 0x143cce588, 0x142489ee0, 0x141e71750> as *const ()), // sub_140fde220
    Replacement::draft_throws(0x140fde380, "unknown_140fde380", fn_140fde380 as *const (), Throws::Discard), // sub_140fde380
    Replacement::draft_throws(0x140fde4e0, "unknown_140fde4e0", tpl_3011::<0x1411e9f50, 0x140fe1870, 0x143ccec38, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140fe1370, 0x1401de410> as *const (), Throws::Discard), // sub_140fde4e0
    Replacement::draft(0x140fde6a0, "unknown_140fde6a0", fn_140fde6a0 as *const ()), // sub_140fde6a0
    Replacement::draft(0x140fde8d0, "unknown_140fde8d0", fn_140fde8d0 as *const ()), // sub_140fde8d0
    Replacement::draft_throws(0x140fde980, "unknown_140fde980", tpl_1330::<0x8, 0x140fe1580, 0x143cce9c4, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140fde980
    Replacement::draft(0x140fdea60, "unknown_140fdea60", fn_140fdea60 as *const ()), // sub_140fdea60
    Replacement::draft(0x140fdecf0, "unknown_140fdecf0", fn_140fdecf0 as *const ()), // sub_140fdecf0
    Replacement::draft(0x140fdee30, "unknown_140fdee30", tpl_1955::<0x10, 0x140fd8d10> as *const ()), // sub_140fdee30
    Replacement::draft(0x140fdee60, "unknown_140fdee60", tpl_1955::<0x20, 0x1401de830> as *const ()), // sub_140fdee60
    Replacement::draft(0x140fdee90, "unknown_140fdee90", fn_140fdee90 as *const ()), // sub_140fdee90
    Replacement::draft(0x140fdf020, "unknown_140fdf020", tpl_1955::<0xc0, 0x140fd8cb0> as *const ()), // sub_140fdf020
    Replacement::draft(0x140fdf050, "unknown_140fdf050", fn_140fdf050 as *const ()), // sub_140fdf050
    Replacement::draft(0x140fdf1f0, "unknown_140fdf1f0", tpl_4344::<0x143cce9c4, 0x142407c80, 0x143cce588, 0x142489ee0, 0x141e71750> as *const ()), // sub_140fdf1f0
    Replacement::draft(0x140fdf3a0, "unknown_140fdf3a0", tpl_1126::<0x142407c80, 0x580f0, 0x141e728e0> as *const ()), // sub_140fdf3a0
    Replacement::draft_throws(0x140fdf460, "unknown_140fdf460", fn_140fdf460 as *const (), Throws::Discard), // sub_140fdf460
    Replacement::draft_throws(0x140fdf5d0, "unknown_140fdf5d0", tpl_1196::<0x140fe1a40, 0x143ccec3c, 0x1420af120, 0x140fe1960, 0x143cceb78, 0x140215750, 0x143c8d014, 0x140215610, 0x143c8d018, 0x1420a28e0> as *const (), Throws::Discard), // sub_140fdf5d0
    Replacement::draft_throws(0x140fdf7b0, "unknown_140fdf7b0", tpl_3398::<0x140fdf8d0, 0x140fe0150, 0x140fdf990> as *const (), Throws::Discard), // sub_140fdf7b0
    Replacement::draft(0x140fdf8d0, "unknown_140fdf8d0", tpl_3114::<0x143cceb78, 0x20, 0x8, 0x143c8d014, 0x580e0, 0x143c8d018, 0x142407c80, 0x580f0, 0x10, 0x18, 0x28> as *const ()), // sub_140fdf8d0
    Replacement::draft_throws(0x140fdf990, "unknown_140fdf990", tpl_1323::<0x18, 0x20, 0x40, 0x8, 0x10, 0x28, 0x141e71db0> as *const (), Throws::Discard), // sub_140fdf990
    Replacement::draft_throws(0x140fdfab0, "unknown_140fdfab0", fn_140fdfab0 as *const (), Throws::Discard), // sub_140fdfab0
    Replacement::draft(0x140fdff50, "unknown_140fdff50", tpl_3188::<0x140fe16e0, 0x14333f878, 0x140fe0480, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140fdff50
    Replacement::draft(0x140fdffd0, "unknown_140fdffd0", tpl_4348::<0x38, 0x40, 0x140fe0480> as *const ()), // sub_140fdffd0
    Replacement::draft(0x140fe02f0, "unknown_140fe02f0", fn_140fe02f0 as *const ()), // sub_140fe02f0
    Replacement::draft(0x140fe0480, "unknown_140fe0480", fn_140fe0480 as *const ()), // sub_140fe0480
    Replacement::draft(0x140fe0f90, "unknown_140fe0f90", tpl_1130::<0x142407c80, 0x141e728e0> as *const ()), // sub_140fe0f90
    Replacement::draft_throws(0x140fe10a0, "unknown_140fe10a0", fn_140fe10a0 as *const (), Throws::Caught), // sub_140fe10a0
    Replacement::draft_throws(0x140fe1160, "unknown_140fe1160", tpl_62748::<0x140fdf8d0, 0x140fe0150, 0x141e71db0> as *const (), Throws::Discard), // sub_140fe1160
    Replacement::draft(0x140fe1370, "unknown_140fe1370", tpl_3028::<0x143ccec38, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x140fdce70> as *const ()), // sub_140fe1370
    Replacement::draft(0x140fe16e0, "unknown_140fe16e0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14333f878, 0x8, 0x20> as *const ()), // sub_140fe16e0
    Replacement::draft(0x140fe1bc0, "unknown_140fe1bc0", fn_140fe1bc0 as *const ()), // sub_140fe1bc0
    Replacement::draft(0x140fe21c0, "unknown_140fe21c0", fn_140fe21c0 as *const ()), // sub_140fe21c0
    Replacement::draft(0x140fe25a0, "unknown_140fe25a0", fn_140fe25a0 as *const ()), // sub_140fe25a0
    Replacement::draft(0x140fe2640, "unknown_140fe2640", fn_140fe2640 as *const ()), // sub_140fe2640
    Replacement::draft(0x140fe27b0, "unknown_140fe27b0", fn_140fe27b0 as *const ()), // sub_140fe27b0
    Replacement::draft(0x140fe2810, "unknown_140fe2810", fn_140fe2810 as *const ()), // sub_140fe2810
    Replacement::draft(0x140fe28d0, "unknown_140fe28d0", tpl_19735::<0x140fe3890, 0x30, 0x140fe37c0, 0x4, 0x14240ab10> as *const ()), // sub_140fe28d0
    Replacement::draft(0x140fe2920, "unknown_140fe2920", tpl_19736::<0x20, 0x28, 0x8, 0x142442de0, 0x18, 0x10, 0x140fe3a40> as *const ()), // sub_140fe2920
    Replacement::draft(0x140fe29a0, "unknown_140fe29a0", fn_140fe29a0 as *const ()), // sub_140fe29a0
    Replacement::draft(0x140fe2a10, "unknown_140fe2a10", fn_140fe2a10 as *const ()), // sub_140fe2a10
    Replacement::draft(0x140fe2ad0, "unknown_140fe2ad0", fn_140fe2ad0 as *const ()), // sub_140fe2ad0
    Replacement::draft(0x140fe2be0, "unknown_140fe2be0", fn_140fe2be0 as *const ()), // sub_140fe2be0
    Replacement::draft(0x140fe2d80, "unknown_140fe2d80", fn_140fe2d80 as *const ()), // sub_140fe2d80
    Replacement::draft(0x140fe2df0, "coregame_140fe2df0", tpl_6567::<0x143ccedb4, 0x143ccedb8, 0x1411e9f50, 0x143ccee14, 0x140215610, 0x143ccee18, 0x1420a28e0, 0x140fe48d0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333fad6, 0x142646960, 0x143ccedd8, 0x143ccede0, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::savegames::processLoadRequests>
    Replacement::draft(0x140fe30c0, "coregame_140fe30c0", tpl_6567::<0x143ccede4, 0x143ccede8, 0x1411e9f50, 0x143ccee1c, 0x140215610, 0x143c90e64, 0x1420a28e0, 0x140fe4900, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14333fba6, 0x142646960, 0x143ccee08, 0x143ccee10, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::savegames::processSaveRequests>
    Replacement::draft(0x140fe3390, "coregame_140fe3390", fn_140fe3390 as *const ()), // sub_140fe3390
    Replacement::draft(0x140fe3510, "coregame_140fe3510", fn_140fe3510 as *const ()), // sub_140fe3510
    Replacement::draft(0x140fe35f0, "coregame_140fe35f0", fn_140fe35f0 as *const ()), // sub_140fe35f0
    Replacement::draft(0x140fe3660, "coregame_140fe3660", fn_140fe3660 as *const ()), // sub_140fe3660
    Replacement::draft(0x140fe3740, "coregame_140fe3740", fn_140fe3740 as *const ()), // sub_140fe3740
    Replacement::draft(0x140fe37c0, "coregame_140fe37c0", fn_140fe37c0 as *const ()), // sub_140fe37c0
    Replacement::draft(0x140fe3890, "coregame_140fe3890", fn_140fe3890 as *const ()), // sub_140fe3890
    Replacement::draft(0x140fe3970, "coregame_140fe3970", tpl_2443::<0x8, 0x1401fc280, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140fe3970
    Replacement::draft(0x140fe3a40, "coregame_140fe3a40", fn_140fe3a40 as *const ()), // sub_140fe3a40
    Replacement::draft(0x140fe3b10, "coregame_140fe3b10", fn_140fe3b10 as *const ()), // sub_140fe3b10
    Replacement::draft(0x140fe3b70, "coregame_140fe3b70", fn_140fe3b70 as *const ()), // sub_140fe3b70
    Replacement::draft(0x140fe3d80, "coregame_140fe3d80", fn_140fe3d80 as *const ()), // sub_140fe3d80
    Replacement::draft(0x140fe3f00, "coregame_140fe3f00", fn_140fe3f00 as *const ()), // sub_140fe3f00
    Replacement::draft(0x140fe4080, "coregame_140fe4080", fn_140fe4080 as *const ()), // sub_140fe4080
    Replacement::draft(0x140fe42b0, "coregame_140fe42b0", fn_140fe42b0 as *const ()), // sub_140fe42b0
    Replacement::draft(0x140fe4440, "coregame_140fe4440", fn_140fe4440 as *const ()), // sub_140fe4440
    Replacement::draft(0x140fe4540, "coregame_140fe4540", fn_140fe4540 as *const ()), // sub_140fe4540
    Replacement::draft(0x140fe4620, "coregame_140fe4620", fn_140fe4620 as *const ()), // sub_140fe4620
    Replacement::draft(0x140fe47c0, "coregame_140fe47c0", fn_140fe47c0 as *const ()), // sub_140fe47c0
    Replacement::draft(0x140fe4930, "coregame_140fe4930", fn_140fe4930 as *const ()), // sub_140fe4930
    Replacement::draft(0x140fe4990, "coregame_140fe4990", fn_140fe4990 as *const ()), // sub_140fe4990
    Replacement::draft(0x140fe49d0, "coregame_140fe49d0", fn_140fe49d0 as *const ()), // sub_140fe49d0
    Replacement::draft(0x140fe4ac0, "coregame_140fe4ac0", tpl_4710::<0xfffffffffffffffe, 0x190, 0x140fe4540, 0x140fe4440, 0x142442de0> as *const ()), // sub_140fe4ac0
    Replacement::draft(0x140fe4b30, "coregame_140fe4b30", tpl_4710::<0xfffffffffffffffe, 0x2d0, 0x14037e270, 0x140fe49d0, 0x142442de0> as *const ()), // sub_140fe4b30
    Replacement::draft(0x140fe4ba0, "coregame_140fe4ba0", tpl_5912::<0xfffffffffffffffe, 0x140fe47c0, 0x142442de0> as *const ()), // sub_140fe4ba0
    Replacement::draft(0x140fe4bf0, "coregame_140fe4bf0", fn_140fe4bf0 as *const ()), // sub_140fe4bf0
    Replacement::draft(0x140fe4cb0, "coregame_140fe4cb0", fn_140fe4cb0 as *const ()), // sub_140fe4cb0
    Replacement::draft(0x140fe4d00, "coregame_140fe4d00", fn_140fe4d00 as *const ()), // sub_140fe4d00
    Replacement::draft(0x140fe4d50, "coregame_140fe4d50", fn_140fe4d50 as *const ()), // sub_140fe4d50
    Replacement::draft_throws(0x140fe4da0, "coregame_140fe4da0", fn_140fe4da0 as *const (), Throws::Discard), // coregame::ResourceManager::coregame::ResourceManager::vfunc_0
    Replacement::draft(0x140fe4e00, "unknown_140fe4e00", fn_140fe4e00 as *const ()), // sub_140fe4e00
    Replacement::draft(0x140fe4e60, "unknown_140fe4e60", tpl_1098::<0x143ccef28, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fe4e60
    Replacement::draft(0x140fe51a0, "unknown_140fe51a0", tpl_19768::<0x8, 0x10, 0x140fe4e60> as *const ()), // sub_140fe51a0
    Replacement::draft(0x140fe52e0, "unknown_140fe52e0", fn_140fe52e0 as *const ()), // sub_140fe52e0
    Replacement::draft_throws(0x140fe5570, "unknown_140fe5570", fn_140fe5570 as *const (), Throws::Discard), // sub_140fe5570
    Replacement::draft_throws(0x140fe55a0, "coregame_140fe55a0", fn_140fe55a0 as *const (), Throws::Discard), // coregame::Renderer::coregame::Renderer::vfunc_0
    Replacement::draft(0x140fe5680, "coregame_140fe5680", fn_140fe5680 as *const ()), // sub_140fe5680
    Replacement::draft(0x140fe57f0, "coregame_140fe57f0", fn_140fe57f0 as *const ()), // sub_140fe57f0
    Replacement::draft(0x140fe5960, "coregame_140fe5960", fn_140fe5960 as *const ()), // sub_140fe5960
    Replacement::draft(0x140fe5ad0, "coregame_140fe5ad0", fn_140fe5ad0 as *const ()), // sub_140fe5ad0
    Replacement::draft(0x140fe5c40, "coregame_140fe5c40", tpl_19776::<0x142407c80, 0x143c4aa00, 0x1426455a8, 0x142645598, 0x141e78c50, 0x142645580> as *const ()), // sub_140fe5c40
    Replacement::draft(0x140fe5d80, "coregame_140fe5d80", fn_140fe5d80 as *const ()), // sub_140fe5d80
    Replacement::draft(0x140fe5ef0, "coregame_140fe5ef0", fn_140fe5ef0 as *const ()), // sub_140fe5ef0
    Replacement::draft(0x140fe6060, "coregame_140fe6060", fn_140fe6060 as *const ()), // sub_140fe6060
    Replacement::draft(0x140fe62c0, "coregame_140fe62c0", fn_140fe62c0 as *const ()), // sub_140fe62c0
    Replacement::draft(0x140fe64b0, "coregame_140fe64b0", tpl_19776::<0x142407c80, 0x143c4aa00, 0x1426455a8, 0x142645598, 0x141e78c30, 0x142645580> as *const ()), // sub_140fe64b0
    Replacement::draft(0x140fe65f0, "coregame_140fe65f0", fn_140fe65f0 as *const ()), // sub_140fe65f0
    Replacement::draft(0x140fe6760, "coregame_140fe6760", fn_140fe6760 as *const ()), // sub_140fe6760
    Replacement::draft(0x140fe68d0, "coregame_140fe68d0", tpl_19783::<0x8, 0x140fbd5e0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140fe68d0
    Replacement::draft(0x140fe69a0, "coregame_140fe69a0", tpl_19784::<0x142407c80, 0x143c4aa00, 0x1424069a0, 0x1426455a8, 0x142645598, 0x141e78c10, 0x142645580> as *const ()), // sub_140fe69a0
    Replacement::draft(0x140fe6af0, "coregame_140fe6af0", fn_140fe6af0 as *const ()), // sub_140fe6af0
    Replacement::draft(0x140fe6ca0, "coregame_140fe6ca0", tpl_19776::<0x142407c80, 0x143c4aa00, 0x1426455a8, 0x142645598, 0x141e78bf0, 0x142645580> as *const ()), // sub_140fe6ca0
    Replacement::draft(0x140fe6de0, "coregame_140fe6de0", fn_140fe6de0 as *const ()), // sub_140fe6de0
    Replacement::draft(0x140fe6fa0, "coregame_140fe6fa0", fn_140fe6fa0 as *const ()), // sub_140fe6fa0
    Replacement::draft(0x140fe7130, "coregame_140fe7130", fn_140fe7130 as *const ()), // sub_140fe7130
    Replacement::draft_throws(0x140fe72e0, "coregame_140fe72e0", fn_140fe72e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::rbf::update>
    Replacement::draft_throws(0x140fe75b0, "coregame_140fe75b0", tpl_4703::<0x143ccefd8, 0x143ccefe0, 0x140fe9250, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143340716, 0x142646960, 0x143cceff0, 0x143cceff8, 0x143cceffc, 0x143cceffe, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::rbf::gatherResources>
    Replacement::draft(0x140fe77c0, "coregame_140fe77c0", tpl_1094::<0x143ccf000, 0x14068b330, 0x14020e8f0> as *const ()), // sub_140fe77c0
    Replacement::draft(0x140fe79c0, "coregame_140fe79c0", tpl_1094::<0x143ccf004, 0x140fe94b0, 0x14020e8f0> as *const ()), // sub_140fe79c0
    Replacement::draft(0x140fe7bc0, "coregame_140fe7bc0", tpl_1094::<0x143ccf008, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_140fe7bc0
    Replacement::draft(0x140fe7dc0, "coregame_140fe7dc0", tpl_1436::<0x58, 0x60, 0x158, 0x143ccf000, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140fe7dc0
    Replacement::draft(0x140fe7ea0, "coregame_140fe7ea0", tpl_1436::<0x58, 0x60, 0x158, 0x143ccf004, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140fe7ea0
    Replacement::draft(0x140fe7f80, "coregame_140fe7f80", tpl_1436::<0x58, 0x60, 0x158, 0x143ccf008, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_140fe7f80
    Replacement::draft(0x140fe8060, "coregame_140fe8060", tpl_1097::<0x143ccf00c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fe8060
    Replacement::draft(0x140fe8390, "coregame_140fe8390", tpl_1368::<0x143c9a2d4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fe8390
    Replacement::draft_throws(0x140fe86c0, "coregame_140fe86c0", fn_140fe86c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::rbf::streamIn>
    Replacement::draft_throws(0x140fe89c0, "coregame_140fe89c0", tpl_1881::<0x143ccf030, 0x143ccf038, 0x1411e9f50, 0x140feadd0, 0x143ccf2e8, 0x1420af120, 0x1420a28e0, 0x140fea6a0, 0x140fe9ce0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433402b6, 0x142646960, 0x143ccf058, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::rbf::streamOutResource>
    Replacement::draft_throws(0x140fe8c90, "coregame_140fe8c90", tpl_18900::<0x143ccf05c, 0x143ccf060, 0x1411e9f50, 0x140feace0, 0x143ccf2e4, 0x1420af120, 0x1420a28e0, 0x140fea780, 0x140fe9d80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143340ed6, 0x142646960, 0x143ccf070, 0x143ccf078, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::rbf::streamOutRBF>
    Replacement::draft_throws(0x140fe8f60, "coregame_140fe8f60", tpl_19790::<0x143ccf07c, 0x143ccf080, 0x1411e9f50, 0x140feabf0, 0x143ccf2e0, 0x1420af120, 0x1420a28e0, 0x140fea860, 0x140fe9e20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143340d46, 0x142646960, 0x143ccf090, 0x143ccf098, 0x143ccf09c, 0x143ccf09e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::rbf::streamOutScales>
    Replacement::draft_throws(0x140fe9250, "unknown_140fe9250", tpl_3011::<0x1411e9f50, 0x140fea940, 0x143ccf1b4, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140fe9ec0, 0x1401de410> as *const (), Throws::Discard), // sub_140fe9250
    Replacement::draft(0x140fe9410, "unknown_140fe9410", tpl_19791::<0x8, 0x140fe8060, 0x140fe8390> as *const ()), // sub_140fe9410
    Replacement::draft(0x140fe94b0, "unknown_140fe94b0", fn_140fe94b0 as *const ()), // sub_140fe94b0
    Replacement::draft(0x140fe99b0, "unknown_140fe99b0", fn_140fe99b0 as *const ()), // sub_140fe99b0
    Replacement::draft(0x140fe9a00, "unknown_140fe9a00", fn_140fe9a00 as *const ()), // sub_140fe9a00
    Replacement::draft_throws(0x140fe9ba0, "unknown_140fe9ba0", fn_140fe9ba0 as *const (), Throws::Caught), // sub_140fe9ba0
    Replacement::draft(0x140fe9ce0, "unknown_140fe9ce0", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140fe7dc0> as *const ()), // sub_140fe9ce0
    Replacement::draft(0x140fe9d80, "unknown_140fe9d80", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140fe7ea0> as *const ()), // sub_140fe9d80
    Replacement::draft(0x140fe9e20, "unknown_140fe9e20", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x140fe7f80> as *const ()), // sub_140fe9e20
    Replacement::draft(0x140fe9ec0, "unknown_140fe9ec0", tpl_3028::<0x143ccf1b4, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141e7b110> as *const ()), // sub_140fe9ec0
    Replacement::draft_throws(0x140fea450, "unknown_140fea450", fn_140fea450 as *const (), Throws::Caught), // sub_140fea450
    Replacement::draft(0x140fea6a0, "unknown_140fea6a0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140fe7dc0> as *const ()), // sub_140fea6a0
    Replacement::draft(0x140fea780, "unknown_140fea780", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140fe7ea0> as *const ()), // sub_140fea780
    Replacement::draft(0x140fea860, "unknown_140fea860", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x140fe7f80> as *const ()), // sub_140fea860
    Replacement::draft(0x140feaa30, "unknown_140feaa30", fn_140feaa30 as *const ()), // sub_140feaa30
    Replacement::draft(0x140feb360, "unknown_140feb360", fn_140feb360 as *const ()), // sub_140feb360
    Replacement::draft(0x140feb3c0, "unknown_140feb3c0", fn_140feb3c0 as *const ()), // sub_140feb3c0
    Replacement::draft_throws(0x140feb460, "unknown_140feb460", fn_140feb460 as *const (), Throws::Discard), // sub_140feb460
    Replacement::draft(0x140feb4d0, "unknown_140feb4d0", fn_140feb4d0 as *const ()), // sub_140feb4d0
    Replacement::draft(0x140feb540, "unknown_140feb540", fn_140feb540 as *const ()), // sub_140feb540
    Replacement::draft(0x140feb5c0, "unknown_140feb5c0", fn_140feb5c0 as *const ()), // sub_140feb5c0
    Replacement::draft_throws(0x140feb7b0, "coregame_140feb7b0", tpl_19802::<0x143ccf400, 0x143ccf408, 0x1411e9f50, 0x140ff3130, 0x143ccf998, 0x1420af120, 0x140215750, 0x1420a28e0, 0x140ff0fa0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143342836, 0x142646960, 0x143ccf418, 0x143ccf420, 0x143ccf424, 0x143ccf426, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ragdoll::loadProfile>
    Replacement::draft_throws(0x140febac0, "coregame_140febac0", fn_140febac0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ragdoll::updateTrackedRecoveryBodies>
    Replacement::draft_throws(0x140febdc0, "coregame_140febdc0", fn_140febdc0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ragdoll::detectPenetrationForRecovery>
    Replacement::draft_throws(0x140fec0a0, "coregame_140fec0a0", fn_140fec0a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ragdoll::applyRecovery>
    Replacement::draft_throws(0x140fec380, "coregame_140fec380", tpl_19806::<0x143ccf4c4, 0x143ccf4c8, 0x1411e9f50, 0x140ff2c50, 0x143ccf988, 0x1420af120, 0x14036d240, 0x1420a28e0, 0x140ff16d0, 0x140fef920, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143343506, 0x142646960, 0x143ccf4e8, 0x143ccf4ec, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ragdoll::updateBoneMapping>
    Replacement::draft_throws(0x140fec670, "coregame_140fec670", tpl_1896::<0x143ccf4f0, 0x143ccf4f8, 0x1411e9f50, 0x140ff2b10, 0x143ccf984, 0x1420af120, 0x140ff0a40, 0x1420a28e0, 0x140ff18d0, 0x140fefab0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143343226, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ragdoll::applyProfile>
    Replacement::draft_throws(0x140fec950, "coregame_140fec950", tpl_3591::<0x143ccf518, 0x143ccf520, 0x1411e9f50, 0x140ff2a00, 0x143ccf980, 0x1420af120, 0x140ff0a40, 0x1420a28e0, 0x140ff1a90, 0x140fefbe0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143342f86, 0x142646960, 0x143ccf540, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ragdoll::applyCollisionLayers>
    Replacement::draft_throws(0x140fecc40, "coregame_140fecc40", fn_140fecc40 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::ragdoll::applyDriveTargets>
    Replacement::draft(0x140fecfc0, "unknown_140fecfc0", fn_140fecfc0 as *const ()), // sub_140fecfc0
    Replacement::draft(0x140fed120, "unknown_140fed120", fn_140fed120 as *const ()), // sub_140fed120
    Replacement::draft(0x140fed320, "unknown_140fed320", fn_140fed320 as *const ()), // sub_140fed320
    Replacement::draft(0x140fed510, "unknown_140fed510", tpl_1368::<0x143ccf580, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140fed510
    Replacement::draft(0x140fed840, "unknown_140fed840", tpl_1094::<0x143ccf57c, 0x140fee8d0, 0x14020e8f0> as *const ()), // sub_140fed840
    Replacement::draft(0x140feda40, "unknown_140feda40", tpl_1094::<0x143ccf578, 0x14068b330, 0x14020e8f0> as *const ()), // sub_140feda40
    Replacement::draft(0x140fedc40, "unknown_140fedc40", tpl_1094::<0x143ccf584, 0x140feea60, 0x14020e8f0> as *const ()), // sub_140fedc40
    Replacement::draft(0x140fede40, "unknown_140fede40", tpl_1094::<0x143ccf588, 0x140feec00, 0x14020e8f0> as *const ()), // sub_140fede40
    Replacement::draft(0x140fee040, "unknown_140fee040", tpl_1094::<0x143ccf58c, 0x140feed80, 0x14020e8f0> as *const ()), // sub_140fee040
    Replacement::draft(0x140fee240, "unknown_140fee240", tpl_1094::<0x143ccf590, 0x140feef00, 0x14020e8f0> as *const ()), // sub_140fee240
    Replacement::draft(0x140fee440, "unknown_140fee440", tpl_1094::<0x143ccf594, 0x140fef0a0, 0x14020e8f0> as *const ()), // sub_140fee440
    Replacement::draft(0x140fee640, "unknown_140fee640", tpl_1094::<0x143ccf598, 0x140fef220, 0x14020e8f0> as *const ()), // sub_140fee640
    Replacement::draft(0x140fee8d0, "unknown_140fee8d0", fn_140fee8d0 as *const ()), // sub_140fee8d0
    Replacement::draft(0x140feea60, "unknown_140feea60", tpl_19812::<0x1401e6d50, 0x140244ce0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143290408, 0x14240aa00> as *const ()), // sub_140feea60
    Replacement::draft(0x140feec00, "unknown_140feec00", tpl_5321::<0x1401e6d50, 0x140ff0ec0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140ff0040> as *const ()), // sub_140feec00
    Replacement::draft(0x140feed80, "unknown_140feed80", tpl_4368::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140330bc0> as *const ()), // sub_140feed80
    Replacement::draft(0x140feef00, "unknown_140feef00", fn_140feef00 as *const ()), // sub_140feef00
    Replacement::draft(0x140fef0a0, "unknown_140fef0a0", fn_140fef0a0 as *const ()), // sub_140fef0a0
    Replacement::draft(0x140fef220, "unknown_140fef220", fn_140fef220 as *const ()), // sub_140fef220
    Replacement::draft_throws(0x140fef3b0, "unknown_140fef3b0", tpl_3803::<0x88, 0x10088, 0x143c9d164, 0x20, 0x8, 0x140ff0c50, 0x140fef4a0> as *const (), Throws::Discard), // sub_140fef3b0
    Replacement::draft_throws(0x140fef4a0, "unknown_140fef4a0", tpl_3804::<0x18, 0x20, 0x8, 0x141e80d40> as *const (), Throws::Discard), // sub_140fef4a0
    Replacement::draft(0x140fef580, "unknown_140fef580", tpl_3801::<0x88, 0x10088, 0x143c9d164, 0x20, 0x8, 0x140ff0b20, 0x140fef670> as *const ()), // sub_140fef580
    Replacement::draft(0x140fef670, "unknown_140fef670", tpl_3802::<0x18, 0x20, 0x30, 0x8, 0x28, 0x141e80830> as *const ()), // sub_140fef670
    Replacement::draft(0x140fef750, "unknown_140fef750", tpl_3801::<0x88, 0x10088, 0x143c9d164, 0x20, 0x8, 0x140ff0910, 0x140fef840> as *const ()), // sub_140fef750
    Replacement::draft(0x140fef840, "unknown_140fef840", tpl_3802::<0x18, 0x20, 0x30, 0x8, 0x28, 0x141e813b0> as *const ()), // sub_140fef840
    Replacement::draft_throws(0x140fef920, "unknown_140fef920", tpl_4104::<0x88, 0x10088, 0x140ff06f0, 0x140fef9c0> as *const (), Throws::Discard), // sub_140fef920
    Replacement::draft_throws(0x140fef9c0, "unknown_140fef9c0", tpl_4105::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141e801f0> as *const (), Throws::Discard), // sub_140fef9c0
    Replacement::draft(0x140fefab0, "unknown_140fefab0", tpl_62998::<0x140ff0590, 0x141e800a0> as *const ()), // sub_140fefab0
    Replacement::draft_throws(0x140fefbe0, "unknown_140fefbe0", tpl_78188::<0x88, 0x10088, 0x140ff0460, 0x141e7ece0> as *const (), Throws::Discard), // sub_140fefbe0
    Replacement::draft_throws(0x140fefd00, "unknown_140fefd00", fn_140fefd00 as *const (), Throws::Discard), // sub_140fefd00
    Replacement::draft_throws(0x140fefdb0, "unknown_140fefdb0", tpl_19817::<0x18, 0x20, 0x40, 0x60, 0x8, 0x141e7ee70> as *const (), Throws::Discard), // sub_140fefdb0
    Replacement::draft(0x140fefed0, "unknown_140fefed0", fn_140fefed0 as *const ()), // sub_140fefed0
    Replacement::draft(0x140ff0040, "unknown_140ff0040", fn_140ff0040 as *const ()), // sub_140ff0040
    Replacement::draft(0x140ff00c0, "unknown_140ff00c0", fn_140ff00c0 as *const ()), // sub_140ff00c0
    Replacement::draft_throws(0x140ff0a40, "unknown_140ff0a40", tpl_1498::<0x143c8b330, 0x143c8b33c, 0x10, 0x1420a25b0, 0x141834a78, 0x143c8f63c, 0x143c8b338, 0x143c8b340, 0x141834a0c> as *const (), Throws::Discard), // sub_140ff0a40
    Replacement::draft_throws(0x140ff0db0, "unknown_140ff0db0", tpl_1330::<0x8, 0x14041d8d0, 0x143c9d164, 0x20, 0x1420af120, 0x10, 0xc, 0x140215750, 0x40> as *const (), Throws::Discard), // sub_140ff0db0
    Replacement::draft(0x140ff0e90, "unknown_140ff0e90", tpl_1955::<0x98, 0x140feb4d0> as *const ()), // sub_140ff0e90
    Replacement::draft(0x140ff0ec0, "unknown_140ff0ec0", fn_140ff0ec0 as *const ()), // sub_140ff0ec0
    Replacement::draft(0x140ff0f20, "unknown_140ff0f20", fn_140ff0f20 as *const ()), // sub_140ff0f20
    Replacement::draft_throws(0x140ff0fa0, "unknown_140ff0fa0", tpl_5214::<0x143ccf998, 0x20, 0x8, 0x141e815f0> as *const (), Throws::Discard), // sub_140ff0fa0
    Replacement::draft_throws(0x140ff1000, "unknown_140ff1000", tpl_63098::<0x143c9d164, 0x140ff0c50, 0x141e80d40> as *const (), Throws::Discard), // sub_140ff1000
    Replacement::draft(0x140ff11d0, "unknown_140ff11d0", tpl_63097::<0x143c9d164, 0x143ccf588, 0x142489ee0, 0x143c8fab8, 0x143c914bc, 0x141e80830> as *const ()), // sub_140ff11d0
    Replacement::draft(0x140ff1450, "unknown_140ff1450", tpl_63097::<0x143c9d164, 0x143ccf588, 0x142489ee0, 0x143ccf58c, 0x143c914bc, 0x141e813b0> as *const ()), // sub_140ff1450
    Replacement::draft_throws(0x140ff16d0, "unknown_140ff16d0", tpl_63343::<0x140ff07a0, 0x141e801f0> as *const (), Throws::Discard), // sub_140ff16d0
    Replacement::draft(0x140ff18d0, "unknown_140ff18d0", tpl_62999::<0x140ff0590, 0x141e800a0> as *const ()), // sub_140ff18d0
    Replacement::draft_throws(0x140ff1a90, "unknown_140ff1a90", tpl_78195::<0x143c914bc, 0x142489ee0, 0x143ccf58c, 0x143ccf57c, 0x141e7ece0> as *const (), Throws::Discard), // sub_140ff1a90
    Replacement::draft_throws(0x140ff1cb0, "unknown_140ff1cb0", fn_140ff1cb0 as *const (), Throws::Discard), // sub_140ff1cb0
    Replacement::draft(0x140ff39e0, "unknown_140ff39e0", fn_140ff39e0 as *const ()), // sub_140ff39e0
    Replacement::draft(0x140ff3a40, "unknown_140ff3a40", fn_140ff3a40 as *const ()), // sub_140ff3a40
    Replacement::draft_throws(0x140ff3bb0, "coregame_140ff3bb0", fn_140ff3bb0 as *const (), Throws::Discard), // coregame::QuestManager::coregame::QuestManager::vfunc_0
    Replacement::draft(0x140ff3d00, "coregame_140ff3d00", tpl_19827::<0x8, 0x140ff47c0> as *const ()), // sub_140ff3d00
    Replacement::draft(0x140ff3d70, "coregame_140ff3d70", tpl_2030::<0x140ff3d00, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_140ff3d70
    Replacement::draft_throws(0x140ff3e20, "coregame_140ff3e20", fn_140ff3e20 as *const (), Throws::Caught), // sub_140ff3e20
    Replacement::draft(0x140ff4170, "coregame_140ff4170", fn_140ff4170 as *const ()), // sub_140ff4170
    Replacement::draft_throws(0x140ff41c0, "coregame_140ff41c0", fn_140ff41c0 as *const (), Throws::Caught), // sub_140ff41c0
    Replacement::draft(0x140ff4570, "coregame_140ff4570", fn_140ff4570 as *const ()), // sub_140ff4570
    Replacement::draft(0x140ff46b0, "coregame_140ff46b0", fn_140ff46b0 as *const ()), // sub_140ff46b0
    Replacement::draft(0x140ff47c0, "coregame_140ff47c0", fn_140ff47c0 as *const ()), // sub_140ff47c0
    Replacement::draft_throws(0x140ff4820, "coregame_140ff4820", fn_140ff4820 as *const (), Throws::Discard), // sub_140ff4820
    Replacement::draft(0x140ff4980, "coregame_140ff4980", fn_140ff4980 as *const ()), // sub_140ff4980
    Replacement::draft(0x140ff4a30, "coregame_140ff4a30", fn_140ff4a30 as *const ()), // sub_140ff4a30
    Replacement::draft(0x140ff4ac0, "coregame_140ff4ac0", fn_140ff4ac0 as *const ()), // sub_140ff4ac0
    Replacement::draft_throws(0x140ff4bb0, "coregame_140ff4bb0", fn_140ff4bb0 as *const (), Throws::Discard), // sub_140ff4bb0
    Replacement::draft(0x140ff4cb0, "coregame_140ff4cb0", fn_140ff4cb0 as *const ()), // sub_140ff4cb0
    Replacement::draft_throws(0x140ff4ef0, "coregame_140ff4ef0", fn_140ff4ef0 as *const (), Throws::Discard), // coregame::QuestLoader::coregame::QuestLoader::vfunc_4
    Replacement::draft(0x140ff4fd0, "coregame_140ff4fd0", fn_140ff4fd0 as *const ()), // sub_140ff4fd0
    Replacement::draft_throws(0x140ff5030, "coregame_140ff5030", fn_140ff5030 as *const (), Throws::Discard), // sub_140ff5030
    Replacement::draft(0x140ff50c0, "coregame_140ff50c0", fn_140ff50c0 as *const ()), // sub_140ff50c0
    Replacement::draft_throws(0x140ff51d0, "coregame_140ff51d0", tpl_19843::<0x1438807c8, 0x142417470, 0x140ff58f0, 0x142645790, 0x1426455a8, 0x142645598, 0x142645868, 0x142442de0, 0x143ccfbd0, 0x140ff5bd0> as *const (), Throws::Discard), // sub_140ff51d0
    Replacement::draft_throws(0x140ff5370, "coregame_140ff5370", fn_140ff5370 as *const (), Throws::Caught), // sub_140ff5370
    Replacement::draft_throws(0x140ff5400, "coregame_140ff5400", fn_140ff5400 as *const (), Throws::Caught), // sub_140ff5400
    Replacement::draft_throws(0x140ff5530, "coregame_140ff5530", tpl_19843::<0x1438807c9, 0x142417470, 0x140ff57e0, 0x142645790, 0x1426455a8, 0x142645598, 0x142645868, 0x142442de0, 0x143ccfbe0, 0x140ff5a00> as *const (), Throws::Discard), // sub_140ff5530
    Replacement::draft(0x140ff56d0, "coregame_140ff56d0", fn_140ff56d0 as *const ()), // coregame::QuestContentLoader::coregame::QuestContentLoader::vfunc_0
    Replacement::draft_throws(0x140ff5740, "coregame_140ff5740", fn_140ff5740 as *const (), Throws::Discard), // coregame::EntityContentLoader::coregame::EntityContentLoader::vfunc_0
    Replacement::draft_throws(0x140ff57e0, "coregame_140ff57e0", tpl_19848::<0x143ccfbe0, 0x8, 0x19, 0x20, 0x10, 0x143ccfbe8, 0x14025f9d0, 0x28, 0x30, 0x18, 0x140208500, 0x140200130, 0x140ff58e5> as *const (), Throws::Discard), // sub_140ff57e0
    Replacement::draft_throws(0x140ff58f0, "coregame_140ff58f0", tpl_19848::<0x143ccfbd0, 0x8, 0x19, 0x20, 0x10, 0x143ccfbd8, 0x14025f9d0, 0x28, 0x30, 0x18, 0x140208500, 0x140200130, 0x140ff59f5> as *const (), Throws::Discard), // sub_140ff58f0
    Replacement::draft(0x140ff5a00, "coregame_140ff5a00", tpl_19849::<0x143ccfbe0, 0x140246bb0, 0x1401e66f0, 0x143ccfbe8, 0x1403f7860> as *const ()), // sub_140ff5a00
    Replacement::draft(0x140ff5bd0, "coregame_140ff5bd0", tpl_19849::<0x143ccfbd0, 0x140246bb0, 0x1401e66f0, 0x143ccfbd8, 0x1403f7860> as *const ()), // sub_140ff5bd0
    Replacement::draft(0x140ff5dc0, "coregame_140ff5dc0", fn_140ff5dc0 as *const ()), // sub_140ff5dc0
    Replacement::draft(0x140ff5e90, "coregame_140ff5e90", fn_140ff5e90 as *const ()), // sub_140ff5e90
    Replacement::draft(0x140ff5f60, "coregame_140ff5f60", fn_140ff5f60 as *const ()), // sub_140ff5f60
    Replacement::draft(0x140ff6030, "coregame_140ff6030", fn_140ff6030 as *const ()), // sub_140ff6030
    Replacement::draft_throws(0x140ff6140, "coregame_140ff6140", fn_140ff6140 as *const (), Throws::Caught), // sub_140ff6140
    Replacement::draft(0x140ff68f0, "coregame_140ff68f0", fn_140ff68f0 as *const ()), // sub_140ff68f0
    Replacement::draft_throws(0x140ff6dd0, "coregame_140ff6dd0", fn_140ff6dd0 as *const (), Throws::Caught), // sub_140ff6dd0
    Replacement::draft(0x140ff7580, "coregame_140ff7580", fn_140ff7580 as *const ()), // sub_140ff7580
    Replacement::draft(0x140ff7700, "coregame_140ff7700", fn_140ff7700 as *const ()), // sub_140ff7700
    Replacement::draft(0x140ff7790, "coregame_140ff7790", fn_140ff7790 as *const ()), // sub_140ff7790
    Replacement::draft_throws(0x140ff78b0, "coregame_140ff78b0", fn_140ff78b0 as *const (), Throws::Caught), // sub_140ff78b0
    Replacement::draft_throws(0x140ff7940, "coregame_140ff7940", tpl_601::<0x1438807d0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143880820, 0x143881ed0> as *const (), Throws::Discard), // sub_140ff7940
    Replacement::draft(0x140ff7a20, "coregame_140ff7a20", fn_140ff7a20 as *const ()), // sub_140ff7a20
    Replacement::draft(0x140ff7ad0, "coregame_140ff7ad0", fn_140ff7ad0 as *const ()), // coregame::ProgramFlowServerStateUnloadingLevel::coregame::ProgramFlowServerStateUnloadingLevel::vfunc_0
    Replacement::draft(0x140ff7b70, "coregame_140ff7b70", tpl_750::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143345738, 0x8> as *const ()), // sub_140ff7b70
    Replacement::draft(0x140ff7c00, "coregame_140ff7c00", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143345738, 0x18> as *const ()), // sub_140ff7c00
    Replacement::draft_throws(0x140ff7d20, "coregame_140ff7d20", fn_140ff7d20 as *const (), Throws::Caught), // sub_140ff7d20
    Replacement::draft_throws(0x140ff7db0, "coregame_140ff7db0", tpl_601::<0x1438807e0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143880820, 0x143881ed0> as *const (), Throws::Discard), // sub_140ff7db0
    Replacement::draft(0x140ff7ec0, "coregame_140ff7ec0", tpl_750::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143345848, 0x8> as *const ()), // sub_140ff7ec0
    Replacement::draft(0x140ff7f50, "coregame_140ff7f50", tpl_6722::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x143345848, 0x18> as *const ()), // sub_140ff7f50
    Replacement::draft_throws(0x140ff8070, "coregame_140ff8070", fn_140ff8070 as *const (), Throws::Caught), // sub_140ff8070
    Replacement::draft_throws(0x140ff8100, "coregame_140ff8100", tpl_601::<0x1438807f0, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143880820, 0x143881ed0> as *const (), Throws::Discard), // sub_140ff8100
    Replacement::draft(0x140ff8210, "coregame_140ff8210", tpl_2384::<0x8, 0x28, 0x20, 0x10, 0x18, 0x1401f2fe0> as *const ()), // sub_140ff8210
    Replacement::draft(0x140ff8290, "coregame_140ff8290", fn_140ff8290 as *const ()), // sub_140ff8290
    Replacement::draft(0x140ff8330, "coregame_140ff8330", fn_140ff8330 as *const ()), // sub_140ff8330
    Replacement::draft_throws(0x140ff8450, "coregame_140ff8450", fn_140ff8450 as *const (), Throws::Caught), // sub_140ff8450
    Replacement::draft_throws(0x140ff84e0, "coregame_140ff84e0", tpl_601::<0x143880800, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143880820, 0x143881ed0> as *const (), Throws::Discard), // sub_140ff84e0
    Replacement::draft(0x140ff85c0, "coregame_140ff85c0", tpl_12588::<0x1401e5470, 0x143407c40, 0x141834d30> as *const ()), // coregame::ProgramFlowServerStatePlatformServicesLogon::coregame::ProgramFlowServerStatePlatformServicesLogon::vfunc_0
    Replacement::draft(0x140ff8640, "coregame_140ff8640", fn_140ff8640 as *const ()), // sub_140ff8640
    Replacement::draft(0x140ff86d0, "coregame_140ff86d0", fn_140ff86d0 as *const ()), // sub_140ff86d0
    Replacement::draft_throws(0x140ff87e0, "coregame_140ff87e0", fn_140ff87e0 as *const (), Throws::Caught), // sub_140ff87e0
    Replacement::draft_throws(0x140ff8870, "coregame_140ff8870", tpl_601::<0x143880810, 0x14307aa58, 0x1413c3940, 0x14307a9d0, 0x14307a9e0, 0x1423fe030, 0x142481290, 0x143880820, 0x143881ed0> as *const (), Throws::Discard), // sub_140ff8870
    Replacement::draft(0x140ff8990, "unknown_140ff8990", tpl_750::<0x20, 0x10, 0x18, 0xfffffffffffffff8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14307a308, 0x8> as *const ()), // sub_140ff8990
    Replacement::draft(0x140ff8a20, "unknown_140ff8a20", tpl_11905::<0x10, 0x142535360, 0x1426456c8, 0x1426453a0, 0x8, 0xc, 0x14307a308, 0x18> as *const ()), // sub_140ff8a20
    Replacement::draft(0x140ff8ad0, "unknown_140ff8ad0", fn_140ff8ad0 as *const ()), // sub_140ff8ad0
    Replacement::draft(0x140ff8b70, "unknown_140ff8b70", fn_140ff8b70 as *const ()), // sub_140ff8b70
    Replacement::draft_throws(0x140ff8db0, "unknown_140ff8db0", fn_140ff8db0 as *const (), Throws::Discard), // sub_140ff8db0
    Replacement::draft(0x140ff8fa0, "unknown_140ff8fa0", fn_140ff8fa0 as *const ()), // sub_140ff8fa0
    Replacement::draft_throws(0x140ff90b0, "unknown_140ff90b0", fn_140ff90b0 as *const (), Throws::Discard), // sub_140ff90b0
    Replacement::draft(0x140ff92a0, "unknown_140ff92a0", fn_140ff92a0 as *const ()), // sub_140ff92a0
    Replacement::draft_throws(0x140ff9340, "unknown_140ff9340", fn_140ff9340 as *const (), Throws::Caught), // sub_140ff9340
    Replacement::draft_throws(0x140ff9a30, "unknown_140ff9a30", fn_140ff9a30 as *const (), Throws::Caught), // sub_140ff9a30
    Replacement::draft_throws(0x140ffa060, "unknown_140ffa060", fn_140ffa060 as *const (), Throws::Caught), // sub_140ffa060
    Replacement::draft(0x140ffa660, "unknown_140ffa660", fn_140ffa660 as *const ()), // sub_140ffa660
    Replacement::draft(0x140ffa7d0, "unknown_140ffa7d0", fn_140ffa7d0 as *const ()), // sub_140ffa7d0
    Replacement::draft(0x140ffa8e0, "unknown_140ffa8e0", fn_140ffa8e0 as *const ()), // sub_140ffa8e0
    Replacement::draft_throws(0x140ffaa10, "unknown_140ffaa10", fn_140ffaa10 as *const (), Throws::Discard), // sub_140ffaa10
    Replacement::draft_throws(0x140ffab20, "unknown_140ffab20", fn_140ffab20 as *const (), Throws::Discard), // sub_140ffab20
    Replacement::draft(0x140fface0, "unknown_140fface0", tpl_1095::<0x143c92bc8, 0x14020e8f0> as *const ()), // sub_140fface0
    Replacement::draft(0x140ffaec0, "unknown_140ffaec0", tpl_1466::<0x143ccfc30, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140ffaec0
    Replacement::draft(0x140ffb190, "unknown_140ffb190", tpl_1466::<0x143cb8c4c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_140ffb190
    Replacement::draft(0x140ffb580, "unknown_140ffb580", fn_140ffb580 as *const ()), // sub_140ffb580
    Replacement::draft(0x140ffb860, "unknown_140ffb860", tpl_19878::<0x168, 0x10, 0x18, 0x142535070, 0x1426456c8, 0x1426453a0, 0x142153640> as *const ()), // sub_140ffb860
    Replacement::draft_throws(0x140ffb8f0, "unknown_140ffb8f0", tpl_1480::<0x1428d10b0, 0x1410d6ca0, 0x140ffcef0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140ffb8f0
    Replacement::draft_throws(0x140ffbb90, "unknown_140ffbb90", tpl_1480::<0x1428d10b0, 0x1410d6ca0, 0x140ffd1c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_140ffbb90
    Replacement::draft(0x140ffbe30, "unknown_140ffbe30", tpl_1094::<0x143cb9034, 0x140ffd630, 0x14020e8f0> as *const ()), // sub_140ffbe30
    Replacement::draft(0x140ffc030, "unknown_140ffc030", tpl_1094::<0x143c93078, 0x140ffd7b0, 0x14020e8f0> as *const ()), // sub_140ffc030
    Replacement::draft(0x140ffc230, "unknown_140ffc230", tpl_1094::<0x143ccfc38, 0x140ffd930, 0x14020e8f0> as *const ()), // sub_140ffc230
    Replacement::draft(0x140ffc430, "unknown_140ffc430", tpl_3593::<0x1401e7c10, 0x140ffd530, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_140ffc430
    Replacement::draft_throws(0x140ffc640, "coregame_140ffc640", tpl_19879::<0x143ccfc3c, 0x143ccfc40, 0x140ffdac0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143346986, 0x142646960, 0x143ccfc60, 0x143ccfc80, 0x143ccfc90, 0x143ccfc98, 0x143ccfc9c, 0x143ccfc9e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::CollisionResourceCo
    Replacement::draft_throws(0x140ffc890, "coregame_140ffc890", tpl_19880::<0x143ccfca0, 0x143ccfca8, 0x1411e9f50, 0x140ffed10, 0x143ccfe28, 0x1420af120, 0x140215750, 0x14027c6a0, 0x1420a28e0, 0x140ffde70, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143347096, 0x142646960, 0x143ccfcc8, 0x143ccfccc, 0x143ccfcce, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_resources::streamOut>
    Replacement::draft_throws(0x140ffcbc0, "coregame_140ffcbc0", tpl_19881::<0x143ccfcd0, 0x143ccfcd8, 0x1411e9f50, 0x140ffec10, 0x143ccfe24, 0x1420af120, 0x140215750, 0x14027c6a0, 0x1420a28e0, 0x140ffdf00, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143347286, 0x142646960, 0x143ccfcf8, 0x143ccfcfc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_resources::streamIn>
    Replacement::draft_throws(0x140ffcef0, "coregame_140ffcef0", tpl_19882::<0x143ccfd00, 0x143ccfd08, 0x1411e9f50, 0x140ffe120, 0x143c93078, 0x1401fc320, 0x140ff0a40, 0x1420a28e0, 0x140ffdf60, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433474b6, 0x142646960, 0x143ccfd28, 0x143ccfd30, 0x143ccfd34, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_resources::removedTemplate>
    Replacement::draft_throws(0x140ffd1c0, "coregame_140ffd1c0", tpl_19882::<0x143ccfd38, 0x143ccfd40, 0x1411e9f50, 0x140ffe0a0, 0x143cb9034, 0x1401fc320, 0x140ff0a40, 0x1420a28e0, 0x140ffe000, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143347626, 0x142646960, 0x143ccfd60, 0x143ccfd68, 0x143ccfd6c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_resources::removedResource>
    Replacement::draft(0x140ffd490, "unknown_140ffd490", tpl_19883::<0x8, 0x140ffc230> as *const ()), // sub_140ffd490
    Replacement::draft(0x140ffd630, "unknown_140ffd630", fn_140ffd630 as *const ()), // sub_140ffd630
    Replacement::draft(0x140ffd7b0, "unknown_140ffd7b0", tpl_1191::<0x1401e6d50, 0x140ffdcd0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1403482f0> as *const ()), // sub_140ffd7b0
    Replacement::draft(0x140ffd930, "unknown_140ffd930", fn_140ffd930 as *const ()), // sub_140ffd930
    Replacement::draft_throws(0x140ffdac0, "unknown_140ffdac0", tpl_3011::<0x1411e9f50, 0x140ffee00, 0x143ccfe2c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x140ffde00, 0x1401de410> as *const (), Throws::Discard), // sub_140ffdac0
    Replacement::draft(0x140ffdc80, "unknown_140ffdc80", tpl_6490::<0x10, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_140ffdc80
    Replacement::draft(0x140ffdcd0, "unknown_140ffdcd0", tpl_1955::<0x50, 0x140348440> as *const ()), // sub_140ffdcd0
    Replacement::draft(0x140ffde00, "unknown_140ffde00", tpl_3028::<0x143ccfe2c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x140ffc430> as *const ()), // sub_140ffde00
    Replacement::draft(0x140ffde70, "unknown_140ffde70", fn_140ffde70 as *const ()), // sub_140ffde70
    Replacement::draft_throws(0x140ffdf00, "unknown_140ffdf00", tpl_5128::<0x143ccfe24, 0x20, 0x8, 0x141e8db00> as *const (), Throws::Discard), // sub_140ffdf00
    Replacement::draft(0x140ffdf60, "unknown_140ffdf60", tpl_6144::<0x143c93078, 0x5e308, 0x10, 0x8, 0x18, 0x141e8d820, 0x140ffe720> as *const ()), // sub_140ffdf60
    Replacement::draft(0x140ffe000, "unknown_140ffe000", tpl_6144::<0x143cb9034, 0x5e308, 0x10, 0x8, 0x18, 0x141e8d790, 0x140ffe9b0> as *const ()), // sub_140ffe000
    Replacement::draft(0x140ffe0a0, "unknown_140ffe0a0", tpl_3188::<0x140ffead0, 0x143347770, 0x140ffe9b0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140ffe0a0
    Replacement::draft(0x140ffe120, "unknown_140ffe120", tpl_3188::<0x140ffeb70, 0x143347798, 0x140ffe720, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_140ffe120
    Replacement::draft(0x140ffe5b0, "unknown_140ffe5b0", fn_140ffe5b0 as *const ()), // sub_140ffe5b0
    Replacement::draft(0x140ffe720, "unknown_140ffe720", fn_140ffe720 as *const ()), // sub_140ffe720
    Replacement::draft(0x140ffe840, "unknown_140ffe840", fn_140ffe840 as *const ()), // sub_140ffe840
    Replacement::draft(0x140ffe9b0, "unknown_140ffe9b0", fn_140ffe9b0 as *const ()), // sub_140ffe9b0
    Replacement::draft(0x140ffead0, "unknown_140ffead0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143347770, 0x8, 0x20> as *const ()), // sub_140ffead0
    Replacement::draft(0x140ffeb70, "unknown_140ffeb70", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143347798, 0x8, 0x20> as *const ()), // sub_140ffeb70
    Replacement::draft(0x140fff0a0, "unknown_140fff0a0", fn_140fff0a0 as *const ()), // sub_140fff0a0
    Replacement::draft(0x140fff250, "unknown_140fff250", fn_140fff250 as *const ()), // sub_140fff250
    Replacement::draft_throws(0x140fffb50, "coregame_140fffb50", tpl_4835::<0x143ccfef0, 0x143ccfef8, 0x1411e9f50, 0x141002840, 0x143cd0158, 0x1420af120, 0x14036d240, 0x1420a28e0, 0x141001b40, 0x141000f70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143348356, 0x142646960, 0x143ccff18, 0x143ccff20, 0x143ccff24, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_render::updateShouldUpdate>
    Replacement::draft_throws(0x140fffe50, "coregame_140fffe50", fn_140fffe50 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_render::updateRenderTransforms>
    Replacement::draft_throws(0x1410001e0, "coregame_1410001e0", tpl_6068::<0x143ccff64, 0x143ccff68, 0x1411e9f50, 0x141002930, 0x143cd015c, 0x1420af120, 0x14036d240, 0x1420a28e0, 0x1410020d0, 0x141001240, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143348c86, 0x142646960, 0x143ccff88, 0x143ccff98, 0x143ccff9c, 0x143ccff9e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_render::updateInvisibilityNoSkinning>
    Replacement::draft(0x1410004c0, "coregame_1410004c0", tpl_6012::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x8, 0xc, 0x143349710> as *const ()), // sub_1410004c0
    Replacement::draft(0x141000590, "coregame_141000590", tpl_1094::<0x143ccffa0, 0x141000df0, 0x14020e8f0> as *const ()), // sub_141000590
    Replacement::draft(0x141000790, "coregame_141000790", tpl_19900::<0x1e, 0x8, 0x140fdb200> as *const ()), // sub_141000790
    Replacement::draft_throws(0x1410007c0, "coregame_1410007c0", tpl_19901::<0x143ccffa4, 0x143ccffa8, 0x1411e9f50, 0x141002be0, 0x143cd029c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141002380, 0x141001fb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433477f6, 0x142646960, 0x143ccffc8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_render::streamOut>
    Replacement::draft_throws(0x141000ab0, "coregame_141000ab0", tpl_19902::<0x143ccffcc, 0x143ccffd0, 0x1411e9f50, 0x141002a70, 0x143cd0298, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141002470, 0x141002040, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433479c6, 0x142646960, 0x143ccfff0, 0x143ccfff2, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_render::streamIn>
    Replacement::draft(0x141000da0, "std_141000da0", tpl_6559::<0x1401de830> as *const ()), // std::_Ref_count_obj_alloc3<r::DynamicArray<rend::BoneMatrixOverride,r::Allocator<rend::BoneMatrixOverride,0>>,r::Allocat
    Replacement::draft(0x141000dc0, "std_141000dc0", tpl_6019::<0x143349710, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<r::DynamicArray<m::Matrix4x3Template<float>,r::Allocator<m::Matrix4x3Template<float>,0>>,r::A
    Replacement::draft(0x141000df0, "unknown_141000df0", fn_141000df0 as *const ()), // sub_141000df0
    Replacement::draft(0x141000f70, "unknown_141000f70", tpl_64022::<0x88, 0x10088, 0x1410017b0, 0x141e8fc70> as *const ()), // sub_141000f70
    Replacement::draft(0x141001240, "unknown_141001240", tpl_19153::<0x88, 0x10088, 0x1410018b0, 0x141e8ece0> as *const ()), // sub_141001240
    Replacement::draft(0x1410012c0, "unknown_1410012c0", fn_1410012c0 as *const ()), // sub_1410012c0
    Replacement::draft(0x141001510, "unknown_141001510", fn_141001510 as *const ()), // sub_141001510
    Replacement::draft(0x1410017b0, "unknown_1410017b0", tpl_3492::<0x143c914bc, 0x142489ee0, 0x143ccffa0> as *const ()), // sub_1410017b0
    Replacement::draft(0x1410018b0, "unknown_1410018b0", tpl_3492::<0x143c8dbc8, 0x142489ee0, 0x143c914bc> as *const ()), // sub_1410018b0
    Replacement::draft(0x1410019b0, "unknown_1410019b0", tpl_19908::<0x1401f2bc0, 0x140e97ef0> as *const ()), // sub_1410019b0
    Replacement::draft(0x141001ad0, "unknown_141001ad0", fn_141001ad0 as *const ()), // sub_141001ad0
    Replacement::draft(0x141001b40, "unknown_141001b40", tpl_64023::<0x143c914bc, 0x142489ee0, 0x143ccffa0, 0x141e8fc70> as *const ()), // sub_141001b40
    Replacement::draft_throws(0x141001cf0, "unknown_141001cf0", fn_141001cf0 as *const (), Throws::Discard), // sub_141001cf0
    Replacement::draft(0x141001fb0, "unknown_141001fb0", tpl_1898::<0x88, 0x10088, 0x141e8ed90> as *const ()), // sub_141001fb0
    Replacement::draft(0x141002040, "unknown_141002040", tpl_1898::<0x88, 0x10088, 0x141e8ef90> as *const ()), // sub_141002040
    Replacement::draft(0x1410020d0, "unknown_1410020d0", fn_1410020d0 as *const ()), // sub_1410020d0
    Replacement::draft(0x141002380, "unknown_141002380", tpl_61552::<0x8, 0x88, 0x10088, 0x141e8ed90> as *const ()), // sub_141002380
    Replacement::draft(0x141002470, "unknown_141002470", tpl_61552::<0x8, 0x88, 0x10088, 0x141e8ef90> as *const ()), // sub_141002470
    Replacement::draft(0x141002560, "unknown_141002560", fn_141002560 as *const ()), // sub_141002560
    Replacement::draft(0x141002fa0, "unknown_141002fa0", fn_141002fa0 as *const ()), // sub_141002fa0
    Replacement::draft_throws(0x141003010, "physics_141003010", fn_141003010 as *const (), Throws::Discard), // physics::RaycastIgnoreCallback<std::function<bool(const physics::RayCastIgnoreData&)>>::physics::RaycastIgnoreCallback<s
    Replacement::draft(0x141003030, "physics_141003030", tpl_19913::<0x8, 0x38, 0x20, 0x143195628, 0x141834d30> as *const ()), // physics::RaycastIgnoreCallback<std::function<bool(const physics::RayCastIgnoreData&)>>::physics::RaycastIgnoreCallback<s
    Replacement::draft_throws(0x1410030a0, "unknown_1410030a0", fn_1410030a0 as *const (), Throws::Discard), // sub_1410030a0
    Replacement::draft(0x141003170, "unknown_141003170", tpl_1171::<0x58, 0x60, 0x143ccf774, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141003170
    Replacement::draft(0x141003250, "unknown_141003250", tpl_1171::<0x58, 0x60, 0x143ccf778, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141003250
    Replacement::draft(0x141003330, "unknown_141003330", tpl_1171::<0x58, 0x60, 0x143cd0578, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141003330
    Replacement::draft_throws(0x141003410, "coregame_141003410", tpl_6039::<0x143cd07a0, 0x143cd0360, 0x1411e9f50, 0x141007a50, 0x143cd0530, 0x1420af120, 0x14036d240, 0x1420a28e0, 0x1410066d0, 0x141005920, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334a4f6, 0x142646960, 0x143cd0380, 0x143cd0384, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_animation::mapBones>
    Replacement::draft_throws(0x141003710, "coregame_141003710", fn_141003710 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_animation::calculateBoneLocalTransforms>
    Replacement::draft_throws(0x141003a10, "coregame_141003a10", tpl_4989::<0x143cd0404, 0x143cd0838, 0x1411e9f50, 0x141007700, 0x143cd074c, 0x1420af120, 0x140ff0a40, 0x1420a28e0, 0x141006a90, 0x141005b70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143349b56, 0x142646960, 0x143cd0858, 0x143cd0868, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_animation::applySkeletonToPhysics>
    Replacement::draft_throws(0x141003d10, "coregame_141003d10", tpl_4989::<0x143cd0824, 0x143cd05b8, 0x1411e9f50, 0x1410075c0, 0x143cd0754, 0x1420af120, 0x14036d240, 0x1420a28e0, 0x141006ca0, 0x141005d20, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143349806, 0x142646960, 0x143cd05d8, 0x143cd05e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_animation::applyPhysicsToSkeleton>
    Replacement::draft(0x141004010, "coregame_141004010", fn_141004010 as *const ()), // sub_141004010
    Replacement::draft(0x141004300, "coregame_141004300", tpl_1094::<0x143ccf774, 0x1410055b0, 0x14020e8f0> as *const ()), // sub_141004300
    Replacement::draft(0x141004500, "coregame_141004500", tpl_1094::<0x143ccf778, 0x141005750, 0x14020e8f0> as *const ()), // sub_141004500
    Replacement::draft(0x141004700, "coregame_141004700", tpl_3072::<0x143cd0578, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141004700
    Replacement::draft_throws(0x141004a50, "coregame_141004a50", tpl_4129::<0x143cd0618, 0x143cd0878, 0x1411e9f50, 0x141007c90, 0x143cd0538, 0x1420af120, 0x1420a28e0, 0x141007270, 0x141006e90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334a036, 0x142646960, 0x143cd0898, 0x143cd089c, 0x143cd089e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_animation::streamOut>
    Replacement::draft_throws(0x141004d40, "coregame_141004d40", tpl_3408::<0x143cd0654, 0x143cd07e8, 0x1411e9f50, 0x141007b60, 0x143cd08a0, 0x1420af120, 0x1420a28e0, 0x141007360, 0x141006f40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334b516, 0x142646960, 0x143cd0808, 0x143cd0818, 0x143cd081c, 0x143cd081e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_animation::streamOutIfPhysicsRemoved>
    Replacement::draft_throws(0x141005040, "coregame_141005040", tpl_1886::<0x143cd082c, 0x143cd05f0, 0x1411e9f50, 0x141007dc0, 0x143cd053c, 0x1420af120, 0x1420a28e0, 0x141007470, 0x141007000, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334b1f6, 0x142646960, 0x143cd0610, 0x143cd0614, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_animation::streamIn>
    Replacement::draft(0x141005320, "unknown_141005320", fn_141005320 as *const ()), // sub_141005320
    Replacement::draft(0x1410055b0, "unknown_1410055b0", fn_1410055b0 as *const ()), // sub_1410055b0
    Replacement::draft(0x141005750, "unknown_141005750", fn_141005750 as *const ()), // sub_141005750
    Replacement::draft(0x141005920, "unknown_141005920", tpl_64030::<0x88, 0x10088, 0x141006450, 0x141e948e0> as *const ()), // sub_141005920
    Replacement::draft(0x141005a40, "unknown_141005a40", tpl_62531::<0x88, 0x10088, 0x1410062f0, 0x141e94750> as *const ()), // sub_141005a40
    Replacement::draft(0x141005b70, "unknown_141005b70", tpl_3559::<0x88, 0x10088, 0x141006050, 0x141005c10> as *const ()), // sub_141005b70
    Replacement::draft(0x141005c10, "unknown_141005c10", tpl_3560::<0x18, 0x20, 0x40, 0x8, 0x141e941a0> as *const ()), // sub_141005c10
    Replacement::draft(0x141005d20, "unknown_141005d20", tpl_3319::<0x40, 0x48, 0x10048, 0x141005f00, 0x141005e10> as *const ()), // sub_141005d20
    Replacement::draft(0x141005e10, "unknown_141005e10", tpl_1916::<0x18, 0x20, 0x40, 0x8, 0x141e93eb0> as *const ()), // sub_141005e10
    Replacement::draft(0x141006580, "unknown_141006580", fn_141006580 as *const ()), // sub_141006580
    Replacement::draft(0x141006650, "unknown_141006650", tpl_1955::<0x20, 0x1401f2bc0> as *const ()), // sub_141006650
    Replacement::draft(0x141006680, "unknown_141006680", tpl_4521::<0x7, 0x30, 0x10, 0x1401de830> as *const ()), // sub_141006680
    Replacement::draft(0x1410066d0, "unknown_1410066d0", tpl_64033::<0x143ccf774, 0x142489ee0, 0x143c914bc, 0x143c8ec20, 0x141e948e0> as *const ()), // sub_1410066d0
    Replacement::draft(0x1410068f0, "unknown_1410068f0", tpl_62567::<0x1410062f0, 0x141e94750> as *const ()), // sub_1410068f0
    Replacement::draft(0x141006a90, "unknown_141006a90", tpl_62907::<0x141006130, 0x141e941a0> as *const ()), // sub_141006a90
    Replacement::draft(0x141006ca0, "unknown_141006ca0", tpl_62687::<0x141005f00, 0x141e93eb0> as *const ()), // sub_141006ca0
    Replacement::draft(0x141006e90, "unknown_141006e90", tpl_19920::<0x142407c80, 0x141e938f0> as *const ()), // sub_141006e90
    Replacement::draft(0x141006f40, "unknown_141006f40", tpl_1126::<0x142407c80, 0x580f0, 0x141e93890> as *const ()), // sub_141006f40
    Replacement::draft(0x141007000, "unknown_141007000", fn_141007000 as *const ()), // sub_141007000
    Replacement::draft(0x141007110, "unknown_141007110", fn_141007110 as *const ()), // sub_141007110
    Replacement::draft(0x141007270, "unknown_141007270", tpl_19923::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141e938f0> as *const ()), // sub_141007270
    Replacement::draft(0x141007360, "unknown_141007360", tpl_1130::<0x142407c80, 0x141e93890> as *const ()), // sub_141007360
    Replacement::draft(0x141007470, "unknown_141007470", fn_141007470 as *const ()), // sub_141007470
    Replacement::draft(0x141007700, "unknown_141007700", fn_141007700 as *const ()), // sub_141007700
    Replacement::draft(0x141008330, "unknown_141008330", fn_141008330 as *const ()), // sub_141008330
    Replacement::draft(0x1410083b0, "unknown_1410083b0", fn_1410083b0 as *const ()), // sub_1410083b0
    Replacement::draft(0x141008420, "unknown_141008420", tpl_19928::<0x8, 0x141010820, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141008420
    Replacement::draft(0x141008c00, "unknown_141008c00", fn_141008c00 as *const ()), // sub_141008c00
    Replacement::draft(0x141008e00, "unknown_141008e00", tpl_1171::<0x58, 0x60, 0x143c93398, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141008e00
    Replacement::draft(0x141008ee0, "unknown_141008ee0", fn_141008ee0 as *const ()), // sub_141008ee0
    Replacement::draft_throws(0x141008f40, "coregame_141008f40", fn_141008f40 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::initGlobal>
    Replacement::draft_throws(0x1410091c0, "coregame_1410091c0", tpl_4804::<0x143cd08d0, 0x143cd08d8, 0x14100e8c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334c296, 0x142646960, 0x143cd08f8, 0x143cd0900, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::applyKeyframedTransforms>
    Replacement::draft_throws(0x1410093d0, "coregame_1410093d0", fn_1410093d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::simulate>
    Replacement::draft_throws(0x141009670, "coregame_141009670", tpl_5091::<0x143cd0924, 0x143cd0928, 0x14100ea90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334ded6, 0x142646960, 0x143cd0948, 0x143cd0958, 0x143cd095c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::applyPhysicsToRigidBodyTransforms>
    Replacement::draft_throws(0x141009880, "coregame_141009880", tpl_4834::<0x143cd0960, 0x143cd0968, 0x1411e9f50, 0x1410161d0, 0x143cd12bc, 0x1420af120, 0x140ff0a40, 0x1420a28e0, 0x141012a20, 0x1410115d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334dc46, 0x142646960, 0x143cd0988, 0x143cd0990, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::applyLayerAndSystemGroup>
    Replacement::draft_throws(0x141009b80, "coregame_141009b80", tpl_5275::<0x143cd0994, 0x143cd0998, 0x1411e9f50, 0x1410162e0, 0x143cd12c0, 0x1420af120, 0x140ff0a40, 0x1420a28e0, 0x141012c20, 0x141011650, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334e806, 0x142646960, 0x143cd09b8, 0x143cd09bc, 0x143cd09be, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::applyKeyframedState>
    Replacement::draft_throws(0x141009e80, "coregame_141009e80", tpl_1533::<0x143cd09c0, 0x143cd09c8, 0x1411e9f50, 0x1410116d0, 0x141012e20, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334e216, 0x142646960, 0x143cd09e8, 0x143cd09f0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::applySceneAddsAndRemoves>
    Replacement::draft_throws(0x14100a0d0, "coregame_14100a0d0", tpl_5374::<0x143cd09f4, 0x143cd09f8, 0x1411e9f50, 0x141015ca0, 0x143cd12ac, 0x1420af120, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x141012ea0, 0x141011860, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334c9b6, 0x142646960, 0x143cd0a18, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::applyBuoyancy>
    Replacement::draft_throws(0x14100a3d0, "coregame_14100a3d0", tpl_1793::<0x143cd0a1c, 0x143cd0a20, 0x14100ec60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334c7a6, 0x142646960, 0x143cd0a40, 0x143cd0a48, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::updateAwakeComponents>
    Replacement::draft_throws(0x14100a5e0, "coregame_14100a5e0", fn_14100a5e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::applyPhysicsToSimpleObjectTransform>
    Replacement::draft_throws(0x14100a8f0, "coregame_14100a8f0", fn_14100a8f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::explicitApplyPhysicsToObjectTransform>
    Replacement::draft_throws(0x14100ac10, "coregame_14100ac10", tpl_19935::<0x143cd0acc, 0x143cd0ad0, 0x14100ee10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433500e6, 0x142646960, 0x143cd0af0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::removedPhysics>
    Replacement::draft(0x14100ae10, "coregame_14100ae10", tpl_6431::<0x143cd0af4, 0x143cd0af8, 0x1411e9f50, 0x141014350, 0x143c93398, 0x1401fc320, 0x143cd0f1c, 0x140215610, 0x1420a28e0, 0x141013700, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334fff6, 0x142646960, 0x143cd0b18, 0x143cd0b20, 0x143cd0b24, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::physics::removedPhysicsPackageState>
    Replacement::draft_throws(0x14100b100, "coregame_14100b100", tpl_1480::<0x1428d2460, 0x1410d6ca0, 0x14100ac10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14100b100
    Replacement::draft(0x14100b3a0, "coregame_14100b3a0", tpl_1526::<0x1428d2460, 0x1410d6ca0, 0x14100ae10, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_14100b3a0
    Replacement::draft(0x14100b640, "coregame_14100b640", fn_14100b640 as *const ()), // ecs::system::ecs::system::wrapper<coregame::physics::processCollisionPackagesToDestruct>
    Replacement::draft_throws(0x14100b890, "coregame_14100b890", tpl_1480::<0x1428d2460, 0x1410d6ca0, 0x14100fec0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14100b890
    Replacement::draft(0x14100bb30, "coregame_14100bb30", tpl_867::<0x8, 0x1410109c0, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14100bb30
    Replacement::draft(0x14100bbe0, "coregame_14100bbe0", fn_14100bbe0 as *const ()), // sub_14100bbe0
    Replacement::draft(0x14100bd70, "coregame_14100bd70", tpl_19938::<0x8, 0x141010a30, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14100bd70
    Replacement::draft(0x14100be30, "coregame_14100be30", tpl_3077::<0x143c8fab8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14100be30
    Replacement::draft(0x14100c160, "coregame_14100c160", tpl_1094::<0x143c914bc, 0x141010c30, 0x14020e8f0> as *const ()), // sub_14100c160
    Replacement::draft(0x14100c360, "coregame_14100c360", tpl_1171::<0x58, 0x60, 0x143c914bc, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_14100c360
    Replacement::draft(0x14100c440, "coregame_14100c440", tpl_77694::<0x1426467e8, 0x141010de0, 0x14100c466> as *const ()), // sub_14100c440
    Replacement::draft(0x14100c5b0, "coregame_14100c5b0", tpl_1095::<0x143c930bc, 0x14020e8f0> as *const ()), // sub_14100c5b0
    Replacement::draft(0x14100c790, "coregame_14100c790", tpl_3072::<0x143cd0b70, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14100c790
    Replacement::draft(0x14100cae0, "coregame_14100cae0", tpl_3077::<0x143c954e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14100cae0
    Replacement::draft(0x14100ce10, "coregame_14100ce10", tpl_1466::<0x143ccf820, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14100ce10
    Replacement::draft(0x14100d0e0, "coregame_14100d0e0", tpl_1466::<0x143c954e4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14100d0e0
    Replacement::draft(0x14100d3b0, "coregame_14100d3b0", tpl_1466::<0x143cd08a4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14100d3b0
    Replacement::draft(0x14100d680, "coregame_14100d680", tpl_1466::<0x143cd0b6c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14100d680
    Replacement::draft(0x14100d950, "coregame_14100d950", fn_14100d950 as *const ()), // sub_14100d950
    Replacement::draft(0x14100dbb0, "coregame_14100dbb0", tpl_1094::<0x143cd0b74, 0x141010f50, 0x14020e8f0> as *const ()), // sub_14100dbb0
    Replacement::draft(0x14100ddb0, "coregame_14100ddb0", tpl_1097::<0x143c939f8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14100ddb0
    Replacement::draft(0x14100e0e0, "coregame_14100e0e0", tpl_1094::<0x143cd0b68, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_14100e0e0
    Replacement::draft(0x14100e2e0, "coregame_14100e2e0", tpl_1094::<0x143cd0bb8, 0x141011120, 0x14020e8f0> as *const ()), // sub_14100e2e0
    Replacement::draft(0x14100e4e0, "coregame_14100e4e0", tpl_1094::<0x143cd0bbc, 0x1410112a0, 0x14020e8f0> as *const ()), // sub_14100e4e0
    Replacement::draft(0x14100e6e0, "coregame_14100e6e0", tpl_1403::<0x143cd0bc0, 0x141011450, 0x14020e8f0> as *const ()), // sub_14100e6e0
    Replacement::draft_throws(0x14100e8c0, "coregame_14100e8c0", fn_14100e8c0 as *const (), Throws::Discard), // sub_14100e8c0
    Replacement::draft_throws(0x14100ea90, "coregame_14100ea90", fn_14100ea90 as *const (), Throws::Discard), // sub_14100ea90
    Replacement::draft_throws(0x14100ec60, "coregame_14100ec60", tpl_3697::<0x1411e9f50, 0x141015b90, 0x143cd12a8, 0x1420af120, 0x140215750, 0x143c8f63c, 0x140215610, 0x1420a28e0, 0x141013060, 0x1401de410> as *const (), Throws::Discard), // sub_14100ec60
    Replacement::draft_throws(0x14100ee10, "coregame_14100ee10", fn_14100ee10 as *const (), Throws::Discard), // sub_14100ee10
    Replacement::draft_throws(0x14100efd0, "coregame_14100efd0", tpl_3592::<0x143cd0bc4, 0x143cd0bc8, 0x1411e9f50, 0x141017cf0, 0x143cd175c, 0x1420af120, 0x140ff0a40, 0x1420a28e0, 0x141015160, 0x1410137b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143350536, 0x142646960, 0x143cd0be8, 0x143cd0bf0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics_system_group::streamIn>
    Replacement::draft_throws(0x14100f2d0, "coregame_14100f2d0", fn_14100f2d0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::streamOut>
    Replacement::draft_throws(0x14100f520, "coregame_14100f520", fn_14100f520 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::streamInTemplate>
    Replacement::draft_throws(0x14100f840, "coregame_14100f840", tpl_5273::<0x143cd0c44, 0x143cd0c48, 0x1411e9f50, 0x141013c50, 0x1410155f0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334f906, 0x142646960, 0x143cd0c68, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::physics::streamInResource>
    Replacement::draft(0x14100fa80, "coregame_14100fa80", tpl_19366::<0x1401e7c10, 0x1410108c0, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_14100fa80
    Replacement::draft_throws(0x14100fc90, "coregame_14100fc90", tpl_6486::<0x143cd0c6c, 0x143cd0c70, 0x141011d30, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14334f716, 0x142646960, 0x143cd0c90, 0x143cd0cb0, 0x143cd0cc0, 0x143cd0cc8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::CollapsedLodCompone
    Replacement::draft_throws(0x14100fec0, "coregame_14100fec0", fn_14100fec0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::collapsed_lod_physics::removed>
    Replacement::draft_throws(0x1410101a0, "coregame_1410101a0", tpl_5993::<0x143cd0cfc, 0x143cd0d00, 0x1411e9f50, 0x1410173a0, 0x143cd174c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141015780, 0x141013de0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143350b56, 0x142646960, 0x143cd0d20, 0x143cd0d28, 0x143cd0d2a, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::collapsed_lod_physics::streamOut>
    Replacement::draft_throws(0x1410104a0, "coregame_1410104a0", fn_1410104a0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::collapsed_lod_physics::streamIn>
    Replacement::draft(0x141010820, "unknown_141010820", fn_141010820 as *const ()), // sub_141010820
    Replacement::draft(0x1410109c0, "unknown_1410109c0", fn_1410109c0 as *const ()), // sub_1410109c0
    Replacement::draft(0x141010a30, "unknown_141010a30", fn_141010a30 as *const ()), // sub_141010a30
    Replacement::draft(0x141010a90, "unknown_141010a90", fn_141010a90 as *const ()), // sub_141010a90
    Replacement::draft(0x141010c30, "unknown_141010c30", fn_141010c30 as *const ()), // sub_141010c30
    Replacement::draft(0x141010f50, "unknown_141010f50", fn_141010f50 as *const ()), // sub_141010f50
    Replacement::draft(0x141011120, "unknown_141011120", tpl_4368::<0x1401e6d50, 0x141012880, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x1410125f0> as *const ()), // sub_141011120
    Replacement::draft(0x1410112a0, "unknown_1410112a0", fn_1410112a0 as *const ()), // sub_1410112a0
    Replacement::draft(0x141011450, "unknown_141011450", tpl_1407::<0x1401e6d50, 0x14023f540, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143295e70> as *const ()), // sub_141011450
    Replacement::draft_throws(0x1410115d0, "unknown_1410115d0", tpl_19954::<0x88, 0x10088, 0x141012120, 0x141e98c80> as *const (), Throws::Discard), // sub_1410115d0
    Replacement::draft(0x141011650, "unknown_141011650", fn_141011650 as *const ()), // sub_141011650
    Replacement::draft_throws(0x1410116d0, "unknown_1410116d0", fn_1410116d0 as *const (), Throws::Discard), // sub_1410116d0
    Replacement::draft_throws(0x141011860, "unknown_141011860", tpl_62718::<0x88, 0x10088, 0x143c8f63c, 0x580e0, 0x141011ef0, 0x141e96c50> as *const (), Throws::Discard), // sub_141011860
    Replacement::draft(0x141011960, "unknown_141011960", fn_141011960 as *const ()), // sub_141011960
    Replacement::draft(0x141011aa0, "unknown_141011aa0", fn_141011aa0 as *const ()), // sub_141011aa0
    Replacement::draft(0x141011bf0, "unknown_141011bf0", tpl_1547::<0x143cd0f1c, 0x140215610, 0x143c8f63c, 0x143c8ce8c, 0x143c8ce94, 0x1420a28e0> as *const ()), // sub_141011bf0
    Replacement::draft_throws(0x141011d30, "unknown_141011d30", tpl_3011::<0x1411e9f50, 0x141017530, 0x143cd1750, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141015670, 0x1401de410> as *const (), Throws::Discard), // sub_141011d30
    Replacement::draft(0x1410125f0, "unknown_1410125f0", fn_1410125f0 as *const ()), // sub_1410125f0
    Replacement::draft(0x141012720, "unknown_141012720", fn_141012720 as *const ()), // sub_141012720
    Replacement::draft(0x141014350, "unknown_141014350", tpl_3188::<0x141015a50, 0x143354a00, 0x141014d20, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141014350
    Replacement::draft(0x1410143d0, "unknown_1410143d0", tpl_3188::<0x141015af0, 0x143354a28, 0x141014be0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410143d0
    Replacement::draft(0x141014450, "unknown_141014450", tpl_4348::<0x18, 0x20, 0x141014be0> as *const ()), // sub_141014450
    Replacement::draft(0x141014a50, "unknown_141014a50", fn_141014a50 as *const ()), // sub_141014a50
    Replacement::draft(0x141014be0, "unknown_141014be0", fn_141014be0 as *const ()), // sub_141014be0
    Replacement::draft(0x141014d20, "unknown_141014d20", fn_141014d20 as *const ()), // sub_141014d20
    Replacement::draft(0x141015160, "unknown_141015160", tpl_61552::<0x8, 0x88, 0x10088, 0x141e9abf0> as *const ()), // sub_141015160
    Replacement::draft_throws(0x141015250, "unknown_141015250", fn_141015250 as *const (), Throws::Discard), // sub_141015250
    Replacement::draft_throws(0x141015340, "unknown_141015340", fn_141015340 as *const (), Throws::Discard), // sub_141015340
    Replacement::draft_throws(0x1410155f0, "unknown_1410155f0", tpl_19963::<0x143cd0e18, 0x20, 0x143cd0f18, 0x8, 0x580e0, 0x143c8ce94, 0x141e9a210> as *const (), Throws::Discard), // sub_1410155f0
    Replacement::draft(0x141015670, "unknown_141015670", tpl_3028::<0x143cd1750, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x14100fa80> as *const ()), // sub_141015670
    Replacement::draft_throws(0x1410156e0, "unknown_1410156e0", fn_1410156e0 as *const (), Throws::Discard), // sub_1410156e0
    Replacement::draft(0x141015780, "unknown_141015780", tpl_61552::<0x8, 0x88, 0x10088, 0x141e96f80> as *const ()), // sub_141015780
    Replacement::draft_throws(0x141015870, "unknown_141015870", fn_141015870 as *const (), Throws::Discard), // sub_141015870
    Replacement::draft(0x141015a50, "unknown_141015a50", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143354a00, 0x8, 0x20> as *const ()), // sub_141015a50
    Replacement::draft(0x141015af0, "unknown_141015af0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143354a28, 0x8, 0x20> as *const ()), // sub_141015af0
    Replacement::draft(0x141016010, "unknown_141016010", fn_141016010 as *const ()), // sub_141016010
    Replacement::draft(0x1410163f0, "unknown_1410163f0", fn_1410163f0 as *const ()), // sub_1410163f0
    Replacement::draft(0x1410166e0, "unknown_1410166e0", fn_1410166e0 as *const ()), // sub_1410166e0
    Replacement::draft(0x1410169b0, "unknown_1410169b0", tpl_3188::<0x141017490, 0x1433549a8, 0x141016d40, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410169b0
    Replacement::draft(0x141016a30, "unknown_141016a30", tpl_4348::<0x10, 0x18, 0x141016d40> as *const ()), // sub_141016a30
    Replacement::draft(0x141016be0, "unknown_141016be0", tpl_4820::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1410125f0, 0x142442de0> as *const ()), // sub_141016be0
    Replacement::draft(0x141016d40, "unknown_141016d40", tpl_4821::<0x1401de830, 0x141008420, 0x1425371a0, 0x142533200> as *const ()), // sub_141016d40
    Replacement::draft(0x141016e60, "unknown_141016e60", fn_141016e60 as *const ()), // sub_141016e60
    Replacement::draft(0x141016ff0, "unknown_141016ff0", fn_141016ff0 as *const ()), // sub_141016ff0
    Replacement::draft(0x141017490, "unknown_141017490", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433549a8, 0x8, 0x20> as *const ()), // sub_141017490
    Replacement::draft(0x141017620, "unknown_141017620", tpl_19979::<0x143cd1340, 0x143cd133c, 0x143c8d694, 0x143c914bc, 0x143cd0b70, 0x143c93198, 0x143c8d120, 0x143c8f6e4, 0x143cd0b74, 0x143c8fa04, 0x143c939f8, 0x143c8d160, 0x143c93398, 0x143cd08a4, 0x143c93078, 0x143cb9034, 0x141834a0c, 0x141834a78> as *const ()), // sub_141017620
    Replacement::draft(0x141017900, "unknown_141017900", tpl_19979::<0x143cd13f0, 0x143cd13ec, 0x143c8d694, 0x143c914bc, 0x143cd0b70, 0x143c93198, 0x143c8d120, 0x143c8f6e4, 0x143cd0b74, 0x143c8fa04, 0x143c939f8, 0x143c8d160, 0x143c93398, 0x143cd08a4, 0x143cb9034, 0x143c93078, 0x141834a0c, 0x141834a78> as *const ()), // sub_141017900
    Replacement::draft(0x141018660, "unknown_141018660", tpl_2029::<0x8, 0x14240ab10, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141018660
    Replacement::draft_throws(0x141018770, "unknown_141018770", fn_141018770 as *const (), Throws::Caught), // sub_141018770
    Replacement::draft(0x141018f20, "unknown_141018f20", fn_141018f20 as *const ()), // sub_141018f20
    Replacement::draft_throws(0x141019370, "unknown_141019370", fn_141019370 as *const (), Throws::Caught), // sub_141019370
    Replacement::draft(0x141019b20, "unknown_141019b20", fn_141019b20 as *const ()), // sub_141019b20
    Replacement::draft_throws(0x141019e30, "unknown_141019e30", fn_141019e30 as *const (), Throws::Caught), // sub_141019e30
    Replacement::draft_throws(0x14101a5e0, "unknown_14101a5e0", fn_14101a5e0 as *const (), Throws::Caught), // sub_14101a5e0
    Replacement::draft(0x14101ad90, "unknown_14101ad90", fn_14101ad90 as *const ()), // sub_14101ad90
    Replacement::draft_throws(0x14101b4d0, "unknown_14101b4d0", fn_14101b4d0 as *const (), Throws::Caught), // sub_14101b4d0
    Replacement::draft_throws(0x14101bc80, "unknown_14101bc80", fn_14101bc80 as *const (), Throws::Caught), // sub_14101bc80
    Replacement::draft_throws(0x14101c430, "unknown_14101c430", fn_14101c430 as *const (), Throws::Caught), // sub_14101c430
    Replacement::draft(0x14101cbe0, "unknown_14101cbe0", fn_14101cbe0 as *const ()), // sub_14101cbe0
    Replacement::draft_throws(0x14101ceb0, "unknown_14101ceb0", fn_14101ceb0 as *const (), Throws::Caught), // sub_14101ceb0
    Replacement::draft_throws(0x14101d660, "unknown_14101d660", fn_14101d660 as *const (), Throws::Caught), // sub_14101d660
    Replacement::draft(0x14101de10, "unknown_14101de10", fn_14101de10 as *const ()), // sub_14101de10
    Replacement::draft_throws(0x14101e0d0, "unknown_14101e0d0", fn_14101e0d0 as *const (), Throws::Caught), // sub_14101e0d0
    Replacement::draft_throws(0x14101e870, "unknown_14101e870", fn_14101e870 as *const (), Throws::Caught), // sub_14101e870
    Replacement::draft(0x14101f0a0, "unknown_14101f0a0", fn_14101f0a0 as *const ()), // sub_14101f0a0
    Replacement::draft(0x14101f160, "unknown_14101f160", fn_14101f160 as *const ()), // sub_14101f160
    Replacement::draft(0x14101f1a0, "unknown_14101f1a0", fn_14101f1a0 as *const ()), // sub_14101f1a0
    Replacement::draft(0x14101f270, "unknown_14101f270", fn_14101f270 as *const ()), // sub_14101f270
    Replacement::draft(0x14101f340, "unknown_14101f340", fn_14101f340 as *const ()), // sub_14101f340
    Replacement::draft(0x14101f400, "unknown_14101f400", fn_14101f400 as *const ()), // sub_14101f400
    Replacement::draft(0x14101f4d0, "unknown_14101f4d0", tpl_19999::<0x14, 0x20, 0x143a08300, 0x14248b0c0, 0x18, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x1401ea4e0, 0x10, 0x140e64170, 0x142442c80, 0x8, 0x142442de0> as *const ()), // sub_14101f4d0
    Replacement::draft(0x14101f5a0, "unknown_14101f5a0", fn_14101f5a0 as *const ()), // sub_14101f5a0
    Replacement::draft(0x14101f690, "unknown_14101f690", fn_14101f690 as *const ()), // sub_14101f690
    Replacement::draft(0x14101f790, "unknown_14101f790", tpl_20001::<0x30, 0x48, 0x40, 0x38, 0x10, 0x14240ab10> as *const ()), // sub_14101f790
    Replacement::draft(0x14101f880, "unknown_14101f880", fn_14101f880 as *const ()), // sub_14101f880
    Replacement::draft(0x14101f8b0, "unknown_14101f8b0", tpl_365::<0x14101fa20> as *const ()), // sub_14101f8b0
    Replacement::draft_throws(0x14101f8c0, "unknown_14101f8c0", fn_14101f8c0 as *const (), Throws::Caught), // sub_14101f8c0
    Replacement::draft(0x14101f960, "unknown_14101f960", fn_14101f960 as *const ()), // sub_14101f960
    Replacement::draft(0x14101fa20, "unknown_14101fa20", tpl_20006::<0x8, 0x141020670, 0xffffffffffffffff, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200, 0x10> as *const ()), // sub_14101fa20
    Replacement::draft(0x14101fb00, "unknown_14101fb00", fn_14101fb00 as *const ()), // sub_14101fb00
    Replacement::draft(0x14101fcc0, "unknown_14101fcc0", fn_14101fcc0 as *const ()), // sub_14101fcc0
    Replacement::draft(0x14101fd30, "unknown_14101fd30", fn_14101fd30 as *const ()), // sub_14101fd30
    Replacement::draft(0x14101fe40, "unknown_14101fe40", fn_14101fe40 as *const ()), // sub_14101fe40
    Replacement::draft_throws(0x14101ff20, "unknown_14101ff20", fn_14101ff20 as *const (), Throws::Discard), // sub_14101ff20
    Replacement::draft(0x14101ffb0, "unknown_14101ffb0", fn_14101ffb0 as *const ()), // sub_14101ffb0
    Replacement::draft(0x1410201d0, "unknown_1410201d0", tpl_170::<0x141020670> as *const ()), // sub_1410201d0
    Replacement::draft_throws(0x1410201e0, "unknown_1410201e0", tpl_78361::<0x8, 0x141020670, 0x10, 0x141025f50, 0x1410261f0> as *const (), Throws::Discard), // sub_1410201e0
    Replacement::draft(0x141020290, "unknown_141020290", fn_141020290 as *const ()), // sub_141020290
    Replacement::draft_throws(0x1410203e0, "unknown_1410203e0", fn_1410203e0 as *const (), Throws::Caught), // sub_1410203e0
    Replacement::draft(0x141020540, "unknown_141020540", fn_141020540 as *const ()), // sub_141020540
    Replacement::draft(0x141020600, "unknown_141020600", fn_141020600 as *const ()), // sub_141020600
    Replacement::draft(0x141020670, "unknown_141020670", fn_141020670 as *const ()), // sub_141020670
    Replacement::draft_throws(0x141020760, "unknown_141020760", fn_141020760 as *const (), Throws::Caught), // sub_141020760
    Replacement::draft(0x141020e50, "unknown_141020e50", fn_141020e50 as *const ()), // sub_141020e50
    Replacement::draft_throws(0x141020fd0, "std_141020fd0", fn_141020fd0 as *const (), Throws::Discard), // sub_141020fd0
    Replacement::draft(0x141021110, "unknown_141021110", fn_141021110 as *const ()), // sub_141021110
    Replacement::draft(0x1410211a0, "unknown_1410211a0", fn_1410211a0 as *const ()), // sub_1410211a0
    Replacement::draft(0x141021330, "sparsehash_141021330", fn_141021330 as *const ()), // sub_141021330
    Replacement::draft(0x141021430, "unknown_141021430", tpl_170::<0x141025cd0> as *const ()), // sub_141021430
    Replacement::draft_throws(0x141021440, "unknown_141021440", fn_141021440 as *const (), Throws::Caught), // sub_141021440
    Replacement::draft_throws(0x141021bc0, "unknown_141021bc0", fn_141021bc0 as *const (), Throws::Caught), // sub_141021bc0
    Replacement::draft(0x141022330, "unknown_141022330", fn_141022330 as *const ()), // sub_141022330
    Replacement::draft_throws(0x1410225f0, "unknown_1410225f0", fn_1410225f0 as *const (), Throws::Caught), // sub_1410225f0
    Replacement::draft(0x141022d60, "unknown_141022d60", fn_141022d60 as *const ()), // sub_141022d60
    Replacement::draft_throws(0x141023270, "unknown_141023270", fn_141023270 as *const (), Throws::Caught), // sub_141023270
    Replacement::draft(0x141023a70, "unknown_141023a70", fn_141023a70 as *const ()), // sub_141023a70
    Replacement::draft_throws(0x141023d30, "unknown_141023d30", fn_141023d30 as *const (), Throws::Caught), // sub_141023d30
    Replacement::draft(0x1410244e0, "unknown_1410244e0", fn_1410244e0 as *const ()), // sub_1410244e0
    Replacement::draft_throws(0x141024a10, "unknown_141024a10", fn_141024a10 as *const (), Throws::Caught), // sub_141024a10
    Replacement::draft(0x141025250, "unknown_141025250", fn_141025250 as *const ()), // sub_141025250
    Replacement::draft_throws(0x141025510, "unknown_141025510", fn_141025510 as *const (), Throws::Caught), // sub_141025510
    Replacement::draft(0x141025840, "unknown_141025840", tpl_20031::<0x141020670> as *const ()), // sub_141025840
    Replacement::draft(0x141025880, "unknown_141025880", fn_141025880 as *const ()), // sub_141025880
    Replacement::draft_throws(0x141025920, "sparsehash_141025920", fn_141025920 as *const (), Throws::Discard), // sub_141025920
    Replacement::draft_throws(0x141025a40, "sparsehash_141025a40", fn_141025a40 as *const (), Throws::Caught), // sub_141025a40
    Replacement::draft(0x141025cd0, "unknown_141025cd0", fn_141025cd0 as *const ()), // sub_141025cd0
    Replacement::draft(0x141025da0, "unknown_141025da0", tpl_78389::<0x14, 0x20, 0x143a08300, 0x14248b0c0, 0x18, 0xfffffffffffffff8, 0xfffffffffffffff0, 0xc8, 0x14240ab10, 0x142442c80, 0x8, 0x142442de0> as *const ()), // sub_141025da0
    Replacement::draft(0x141025e80, "unknown_141025e80", tpl_20036::<0x14, 0x20, 0x143a08300, 0x14248b0c0, 0x18, 0xfffffffffffffff8, 0xfffffffffffffff0, 0x10, 0x1401de830, 0x142442c80, 0x8, 0x142442de0> as *const ()), // sub_141025e80
    Replacement::draft_throws(0x141025f50, "unknown_141025f50", tpl_20037::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1410261f0, 0x141026590, 0x141020670, 0x1425371a0, 0x142533200, 0x1402032e0, 0x1410261e0> as *const (), Throws::Discard), // sub_141025f50
    Replacement::draft(0x1410261f0, "unknown_1410261f0", tpl_20038::<0x141026590> as *const ()), // sub_1410261f0
    Replacement::draft(0x141026250, "unknown_141026250", tpl_20039::<0x8, 0x14240ab10> as *const ()), // sub_141026250
    Replacement::draft(0x141026290, "unknown_141026290", tpl_16383::<0x8, 0x141020670> as *const ()), // sub_141026290
    Replacement::draft_throws(0x1410262d0, "unknown_1410262d0", fn_1410262d0 as *const (), Throws::Caught), // sub_1410262d0
    Replacement::draft_throws(0x141026430, "unknown_141026430", fn_141026430 as *const (), Throws::Caught), // sub_141026430
    Replacement::draft(0x141026590, "unknown_141026590", fn_141026590 as *const ()), // sub_141026590
    Replacement::draft(0x141026600, "unknown_141026600", fn_141026600 as *const ()), // sub_141026600
    Replacement::draft_throws(0x141026650, "coregame_141026650", fn_141026650 as *const (), Throws::Discard), // coregame::PersistentEpisodeBlock::coregame::PersistentEpisodeBlock::vfunc_0
    Replacement::draft_throws(0x1410267b0, "coregame_1410267b0", fn_1410267b0 as *const (), Throws::Caught), // sub_1410267b0
    Replacement::draft_throws(0x141026980, "coregame_141026980", fn_141026980 as *const (), Throws::Caught), // sub_141026980
    Replacement::draft(0x141026b40, "coregame_141026b40", fn_141026b40 as *const ()), // sub_141026b40
    Replacement::draft(0x141026ce0, "coregame_141026ce0", fn_141026ce0 as *const ()), // sub_141026ce0
    Replacement::draft_throws(0x141026e80, "coregame_141026e80", fn_141026e80 as *const (), Throws::Discard), // sub_141026e80
    Replacement::draft(0x141027200, "coregame_141027200", tpl_20046::<0x140f0d7c0> as *const ()), // sub_141027200
    Replacement::draft(0x141027260, "coregame_141027260", fn_141027260 as *const ()), // sub_141027260
    Replacement::draft(0x141027330, "coregame_141027330", fn_141027330 as *const ()), // coregame::PersistentContentBlock::coregame::PersistentContentBlock::vfunc_5
    Replacement::draft(0x141027350, "coregame_141027350", fn_141027350 as *const ()), // coregame::PersistentContentBlock::coregame::PersistentContentBlock::vfunc_0
    Replacement::draft(0x1410275e0, "coregame_1410275e0", fn_1410275e0 as *const ()), // sub_1410275e0
    Replacement::draft(0x141027670, "coregame_141027670", fn_141027670 as *const ()), // sub_141027670
    Replacement::draft_throws(0x1410277e0, "coregame_1410277e0", tpl_1480::<0x1428d4698, 0x1410d6ca0, 0x141029300, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_1410277e0
    Replacement::draft(0x141027a80, "coregame_141027a80", tpl_1526::<0x1428d4698, 0x1410d6ca0, 0x141029550, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_141027a80
    Replacement::draft(0x141027d20, "coregame_141027d20", tpl_64515::<0x1426467e8, 0x141029e20, 0x141027d99> as *const ()), // sub_141027d20
    Replacement::draft(0x141027ef0, "coregame_141027ef0", tpl_1094::<0x143cd1a00, 0x141029940, 0x14020e8f0> as *const ()), // sub_141027ef0
    Replacement::draft(0x1410280f0, "coregame_1410280f0", tpl_1097::<0x143cc8b94, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410280f0
    Replacement::draft(0x141028420, "coregame_141028420", tpl_1094::<0x143cd1a04, 0x14068b330, 0x14020e8f0> as *const ()), // sub_141028420
    Replacement::draft(0x141028620, "coregame_141028620", tpl_1094::<0x143cd19fc, 0x141029ad0, 0x14020e8f0> as *const ()), // sub_141028620
    Replacement::draft(0x141028820, "coregame_141028820", tpl_20052::<0x143427080, 0x10, 0x8, 0x141027ef0, 0x1410280f0> as *const ()), // sub_141028820
    Replacement::draft(0x1410288c0, "coregame_1410288c0", tpl_3593::<0x1401e7c10, 0x141029840, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_1410288c0
    Replacement::draft_throws(0x141028ad0, "coregame_141028ad0", tpl_20053::<0x143cd1a08, 0x143cd1a10, 0x141029c60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143355786, 0x142646960, 0x143cd1a30, 0x143cd1a50, 0x143cd1a60, 0x143cd1a68, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::ParticleSystemCompo
    Replacement::draft_throws(0x141028d00, "coregame_141028d00", fn_141028d00 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::particle::streamIn>
    Replacement::draft_throws(0x141029020, "coregame_141029020", tpl_3552::<0x143cd1a90, 0x143cd1a98, 0x1411e9f50, 0x14102b510, 0x143cd1cb4, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x14102a990, 0x14102a0e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143354d46, 0x142646960, 0x143cd1aa8, 0x143cd1ab0, 0x143cd1ab4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::particle::streamOut>
    Replacement::draft_throws(0x141029300, "coregame_141029300", tpl_1578::<0x143cd1ab8, 0x143cd1ac0, 0x1411e9f50, 0x14102a170, 0x14102aa80, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143354fd6, 0x142646960, 0x143cd1ae0, 0x143cd1ae4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::particle::updateAllEntities>
    Replacement::draft(0x141029550, "coregame_141029550", fn_141029550 as *const ()), // ecs::system::ecs::system::wrapper<coregame::particle::remove>
    Replacement::draft(0x141029940, "unknown_141029940", fn_141029940 as *const ()), // sub_141029940
    Replacement::draft(0x141029ad0, "unknown_141029ad0", fn_141029ad0 as *const ()), // sub_141029ad0
    Replacement::draft_throws(0x141029c60, "unknown_141029c60", tpl_3011::<0x1411e9f50, 0x14102b780, 0x143cd1cbc, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14102a7a0, 0x1401de410> as *const (), Throws::Discard), // sub_141029c60
    Replacement::draft(0x141029f70, "unknown_141029f70", tpl_3157::<0x7, 0x18, 0x142442de0> as *const ()), // sub_141029f70
    Replacement::draft_throws(0x141029fd0, "unknown_141029fd0", tpl_78055::<0x143cada18, 0x143cd1a00, 0x142489ee0, 0x141ea2240> as *const (), Throws::Discard), // sub_141029fd0
    Replacement::draft(0x14102a0e0, "unknown_14102a0e0", tpl_1898::<0x88, 0x10088, 0x141ea2040> as *const ()), // sub_14102a0e0
    Replacement::draft_throws(0x14102a170, "unknown_14102a170", fn_14102a170 as *const (), Throws::Discard), // sub_14102a170
    Replacement::draft(0x14102a7a0, "unknown_14102a7a0", tpl_3028::<0x143cd1cbc, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x1410288c0> as *const ()), // sub_14102a7a0
    Replacement::draft_throws(0x14102a810, "unknown_14102a810", tpl_78057::<0x143cada18, 0x143cd1a00, 0x142489ee0, 0x141ea2240> as *const (), Throws::Discard), // sub_14102a810
    Replacement::draft(0x14102a990, "unknown_14102a990", tpl_61552::<0x8, 0x88, 0x10088, 0x141ea2040> as *const ()), // sub_14102a990
    Replacement::draft_throws(0x14102aa80, "unknown_14102aa80", fn_14102aa80 as *const (), Throws::Discard), // sub_14102aa80
    Replacement::draft(0x14102ab30, "unknown_14102ab30", tpl_3172::<0x143cd19fc, 0x5e308, 0x143cada18, 0x580e0, 0x10, 0x8, 0x18, 0x141ea0b20, 0x14102b070> as *const ()), // sub_14102ab30
    Replacement::draft(0x14102abe0, "unknown_14102abe0", tpl_3188::<0x14102b1a0, 0x143356a48, 0x14102b070, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14102abe0
    Replacement::draft(0x14102ae20, "unknown_14102ae20", fn_14102ae20 as *const ()), // sub_14102ae20
    Replacement::draft(0x14102b070, "unknown_14102b070", fn_14102b070 as *const ()), // sub_14102b070
    Replacement::draft(0x14102b1a0, "unknown_14102b1a0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143356a48, 0x8, 0x20> as *const ()), // sub_14102b1a0
    Replacement::draft(0x14102bb40, "unknown_14102bb40", fn_14102bb40 as *const ()), // sub_14102bb40
    Replacement::draft(0x14102bb90, "unknown_14102bb90", tpl_20064::<0xa8, 0xb0, 0x8, 0x142442de0, 0xa0, 0x98, 0x50, 0x14240ab10> as *const ()), // sub_14102bb90
    Replacement::draft(0x14102bc30, "unknown_14102bc30", tpl_20065::<0x14102bc60> as *const ()), // sub_14102bc30
    Replacement::draft(0x14102bc60, "unknown_14102bc60", fn_14102bc60 as *const ()), // sub_14102bc60
    Replacement::draft(0x14102bdb0, "unknown_14102bdb0", fn_14102bdb0 as *const ()), // sub_14102bdb0
    Replacement::draft(0x14102bdf0, "unknown_14102bdf0", fn_14102bdf0 as *const ()), // sub_14102bdf0
    Replacement::draft(0x14102be20, "unknown_14102be20", fn_14102be20 as *const ()), // sub_14102be20
    Replacement::draft(0x14102be70, "unknown_14102be70", fn_14102be70 as *const ()), // sub_14102be70
    Replacement::draft(0x14102c1d0, "unknown_14102c1d0", fn_14102c1d0 as *const ()), // sub_14102c1d0
    Replacement::draft_throws(0x14102c290, "unknown_14102c290", fn_14102c290 as *const (), Throws::Caught), // sub_14102c290
    Replacement::draft_throws(0x14102c600, "unknown_14102c600", fn_14102c600 as *const (), Throws::Caught), // sub_14102c600
    Replacement::draft_throws(0x14102c8f0, "unknown_14102c8f0", fn_14102c8f0 as *const (), Throws::Discard), // sub_14102c8f0
    Replacement::draft(0x14102ca60, "unknown_14102ca60", fn_14102ca60 as *const ()), // sub_14102ca60
    Replacement::draft(0x14102caf0, "unknown_14102caf0", fn_14102caf0 as *const ()), // sub_14102caf0
    Replacement::draft(0x14102cbe0, "unknown_14102cbe0", fn_14102cbe0 as *const ()), // sub_14102cbe0
    Replacement::draft_throws(0x14102cce0, "unknown_14102cce0", fn_14102cce0 as *const (), Throws::Discard), // sub_14102cce0
    Replacement::draft(0x14102cd50, "unknown_14102cd50", tpl_20079::<0x28, 0x10, 0x20, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x143356b18, 0x1402bf830, 0x143356b48> as *const ()), // sub_14102cd50
    Replacement::draft(0x14102ce10, "unknown_14102ce10", fn_14102ce10 as *const ()), // sub_14102ce10
    Replacement::draft(0x14102cf00, "unknown_14102cf00", fn_14102cf00 as *const ()), // sub_14102cf00
    Replacement::draft_throws(0x14102d040, "unknown_14102d040", fn_14102d040 as *const (), Throws::Discard), // sub_14102d040
    Replacement::draft_throws(0x14102d1d0, "unknown_14102d1d0", fn_14102d1d0 as *const (), Throws::Caught), // sub_14102d1d0
    Replacement::draft(0x14102d320, "unknown_14102d320", fn_14102d320 as *const ()), // sub_14102d320
    Replacement::draft(0x14102d540, "unknown_14102d540", fn_14102d540 as *const ()), // sub_14102d540
    Replacement::draft(0x14102d750, "unknown_14102d750", tpl_20086::<0x14102da30, 0x1425371a0, 0x142533200, 0x142442de0, 0x14102d540, 0x1401f3c00> as *const ()), // sub_14102d750
    Replacement::draft(0x14102da30, "unknown_14102da30", tpl_2871::<0x38, 0x10, 0x18, 0xffffffffffffffe0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143356aa0, 0x8, 0x9, 0x11, 0x1401f3d90, 0x143356ad0> as *const ()), // sub_14102da30
    Replacement::draft(0x14102db10, "unknown_14102db10", fn_14102db10 as *const ()), // sub_14102db10
    Replacement::draft_throws(0x14102db70, "unknown_14102db70", tpl_4037::<0x143cd1e08, 0x143cd1dcc, 0x143cd1e10, 0x1420aeb60, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143c8f7a0, 0x143c8d87c, 0x143cd1e20, 0x143cd1e28, 0x141834a0c> as *const (), Throws::Discard), // sub_14102db70
    Replacement::draft(0x14102dcd0, "coregame_14102dcd0", fn_14102dcd0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::audio::updateMusicCallbacks>
    Replacement::draft(0x14102dee0, "coregame_14102dee0", fn_14102dee0 as *const ()), // sub_14102dee0
    Replacement::draft(0x14102e040, "coregame_14102e040", fn_14102e040 as *const ()), // sub_14102e040
    Replacement::draft(0x14102e0b0, "coregame_14102e0b0", fn_14102e0b0 as *const ()), // sub_14102e0b0
    Replacement::draft(0x14102e300, "coregame_14102e300", tpl_1765::<0x1411e9f50, 0x143cd1e98, 0x140215610, 0x143c90e64, 0x143c8ceec, 0x1420a28e0, 0x14102e480, 0x1401de410> as *const ()), // sub_14102e300
    Replacement::draft(0x14102e4c0, "coregame_14102e4c0", tpl_78436::<0x14102e6e0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x14102e5c0, 0x142442de0> as *const ()), // sub_14102e4c0
    Replacement::draft(0x14102e5c0, "coregame_14102e5c0", tpl_2030::<0x14102e670, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_14102e5c0
    Replacement::draft(0x14102e670, "coregame_14102e670", fn_14102e670 as *const ()), // sub_14102e670
    Replacement::draft(0x14102e6e0, "coregame_14102e6e0", fn_14102e6e0 as *const ()), // sub_14102e6e0
    Replacement::draft(0x14102e8c0, "coregame_14102e8c0", fn_14102e8c0 as *const ()), // sub_14102e8c0
    Replacement::draft_throws(0x14102ea60, "coregame_14102ea60", fn_14102ea60 as *const (), Throws::Discard), // sub_14102ea60
    Replacement::draft(0x14102ec70, "coregame_14102ec70", fn_14102ec70 as *const ()), // sub_14102ec70
    Replacement::draft(0x14102ed20, "coregame_14102ed20", fn_14102ed20 as *const ()), // sub_14102ed20
    Replacement::draft(0x14102ee80, "coregame_14102ee80", fn_14102ee80 as *const ()), // sub_14102ee80
    Replacement::draft(0x14102eee0, "coregame_14102eee0", fn_14102eee0 as *const ()), // sub_14102eee0
    Replacement::draft(0x14102f030, "coregame_14102f030", fn_14102f030 as *const ()), // sub_14102f030
    Replacement::draft(0x14102f330, "coregame_14102f330", fn_14102f330 as *const ()), // sub_14102f330
    Replacement::draft_throws(0x14102f3e0, "coregame_14102f3e0", fn_14102f3e0 as *const (), Throws::Discard), // sub_14102f3e0
    Replacement::draft_throws(0x14102fef0, "coregame_14102fef0", fn_14102fef0 as *const (), Throws::Discard), // sub_14102fef0
    Replacement::draft(0x141030a40, "coregame_141030a40", fn_141030a40 as *const ()), // ecs::system::ecs::system::wrapper<coregame::movementmodel::initDatabase>
    Replacement::draft_throws(0x141030cd0, "coregame_141030cd0", tpl_4485::<0x143cd4068, 0x143cd26b8, 0x1411e9f50, 0x141037d90, 0x143cd3014, 0x1420af120, 0x1420a28e0, 0x141036c80, 0x141035bb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335b576, 0x142646960, 0x143cd26d8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::movementmodel::teleported>
    Replacement::draft(0x141030fa0, "coregame_141030fa0", fn_141030fa0 as *const ()), // sub_141030fa0
    Replacement::draft_throws(0x141031430, "coregame_141031430", fn_141031430 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::movementmodel::update_pre_animation::system<class ecs::Entity<class ecs::acc
    Replacement::draft(0x141031730, "coregame_141031730", fn_141031730 as *const ()), // sub_141031730
    Replacement::draft_throws(0x1410317d0, "coregame_1410317d0", tpl_78451::<0x143cd1ea0, 0x143cd2f5c, 0x1411e9f50, 0x1410379c0, 0x143cd3010, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x141037090, 0x141035f50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433591f6, 0x142646960, 0x14183acbc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::movementmodel::update_post_animation::system<class ecs::Entity<class ecs::ac
    Replacement::draft_throws(0x141031ad0, "coregame_141031ad0", fn_141031ad0 as *const (), Throws::Discard), // sub_141031ad0
    Replacement::draft_throws(0x141032120, "coregame_141032120", fn_141032120 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::movementmodel::update_pre_animation_navigation_agent::system<class ecs::Enti
    Replacement::draft_throws(0x141032420, "coregame_141032420", tpl_78451::<0x143cd38d0, 0x143cd25bc, 0x1411e9f50, 0x1410375f0, 0x143cd2fe0, 0x1420af120, 0x143c8d910, 0x140215610, 0x1420a28e0, 0x141037090, 0x141035f50, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143356f96, 0x142646960, 0x14183acbc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::movementmodel::update_post_animation::system<class ecs::Entity<class ecs::ac
    Replacement::draft(0x141032720, "unknown_141032720", tpl_20099::<0x143c97ad0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141032720
    Replacement::draft(0x141032a80, "unknown_141032a80", tpl_1435::<0x143cd3888, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141032a80
    Replacement::draft(0x141032dd0, "unknown_141032dd0", fn_141032dd0 as *const ()), // sub_141032dd0
    Replacement::draft(0x141033110, "unknown_141033110", tpl_3072::<0x143c8e68c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141033110
    Replacement::draft(0x141033490, "unknown_141033490", fn_141033490 as *const ()), // sub_141033490
    Replacement::draft(0x141033ec0, "unknown_141033ec0", fn_141033ec0 as *const ()), // sub_141033ec0
    Replacement::draft(0x141034650, "unknown_141034650", fn_141034650 as *const ()), // sub_141034650
    Replacement::draft(0x141034f20, "unknown_141034f20", fn_141034f20 as *const ()), // sub_141034f20
    Replacement::draft(0x1410357a0, "unknown_1410357a0", fn_1410357a0 as *const ()), // sub_1410357a0
    Replacement::draft(0x141035bb0, "unknown_141035bb0", tpl_1889::<0x40, 0x48, 0x10048, 0x141036b10, 0x141035cb0> as *const ()), // sub_141035bb0
    Replacement::draft(0x141035cb0, "unknown_141035cb0", tpl_1890::<0x18, 0x20, 0x40, 0x8, 0x141ea8e90> as *const ()), // sub_141035cb0
    Replacement::draft(0x141035d90, "unknown_141035d90", fn_141035d90 as *const ()), // sub_141035d90
    Replacement::draft(0x141035e40, "unknown_141035e40", tpl_3560::<0x18, 0x20, 0x40, 0x8, 0x141030fa0> as *const ()), // sub_141035e40
    Replacement::draft(0x141035f50, "unknown_141035f50", tpl_5032::<0x88, 0x10088, 0x580e0, 0x143c8d910, 0x141036300, 0x141036010> as *const ()), // sub_141035f50
    Replacement::draft(0x141036010, "unknown_141036010", tpl_5033::<0x18, 0x20, 0x50, 0x40, 0x48, 0x8, 0x141031730> as *const ()), // sub_141036010
    Replacement::draft_throws(0x141036120, "unknown_141036120", fn_141036120 as *const (), Throws::Discard), // sub_141036120
    Replacement::draft_throws(0x1410361e0, "unknown_1410361e0", tpl_19817::<0x18, 0x20, 0x40, 0x60, 0x8, 0x141031ad0> as *const (), Throws::Discard), // sub_1410361e0
    Replacement::draft(0x141036c80, "unknown_141036c80", tpl_61544::<0x141036b10, 0x141ea8e90> as *const ()), // sub_141036c80
    Replacement::draft(0x141036e60, "unknown_141036e60", fn_141036e60 as *const ()), // sub_141036e60
    Replacement::draft(0x141037090, "unknown_141037090", tpl_64227::<0x143c8d910, 0x1410363c0, 0x141031730> as *const ()), // sub_141037090
    Replacement::draft_throws(0x1410372a0, "unknown_1410372a0", fn_1410372a0 as *const (), Throws::Discard), // sub_1410372a0
    Replacement::draft(0x1410375f0, "unknown_1410375f0", fn_1410375f0 as *const ()), // sub_1410375f0
    Replacement::draft(0x1410377b0, "unknown_1410377b0", tpl_5834::<0x143cd305c, 0x143cd3ff0, 0x143c97ad0, 0x143c8e68c, 0x143cc25cc, 0x143c8d120, 0x143c9f438, 0x143c8d074, 0x143cd3888, 0x143c9b928, 0x143c91f1c, 0x143cd4010, 0x143cd4030, 0x143cd4050, 0x141834a0c, 0x141834a78> as *const ()), // sub_1410377b0
    Replacement::draft(0x1410379c0, "unknown_1410379c0", fn_1410379c0 as *const ()), // sub_1410379c0
    Replacement::draft(0x141037b80, "unknown_141037b80", tpl_6071::<0x143cd1e9c, 0x143cd2f70, 0x143cc25cc, 0x143c97ad0, 0x143c8e68c, 0x143c8d120, 0x143c9f438, 0x143ca7420, 0x143cd3888, 0x143c9b928, 0x143c91f1c, 0x143cd2f90, 0x143cd2fb0, 0x143cd2fd0, 0x141834a0c, 0x141834a78> as *const ()), // sub_141037b80
    Replacement::draft(0x1410381a0, "unknown_1410381a0", fn_1410381a0 as *const ()), // sub_1410381a0
    Replacement::draft(0x1410382d0, "unknown_1410382d0", fn_1410382d0 as *const ()), // sub_1410382d0
    Replacement::draft(0x1410386a0, "unknown_1410386a0", fn_1410386a0 as *const ()), // sub_1410386a0
    Replacement::draft(0x1410387f0, "unknown_1410387f0", tpl_5084::<0x143c8d87c, 0x142489ee0, 0x143c8dbc8, 0x143c8e070, 0x143cd4478, 0x143c9c6e4, 0x143cc515c, 0x143c8d120, 0x143c8e2e8, 0x143c8de70, 0x143c9a338, 0x143c95490, 0x143c930bc> as *const ()), // sub_1410387f0
    Replacement::draft(0x141039060, "unknown_141039060", tpl_60443::<0x143c8ec20, 0x142489ee0, 0xc03, 0x8, 0x10, 0x1426467e8, 0x14103911a> as *const ()), // sub_141039060
    Replacement::draft(0x141039120, "unknown_141039120", fn_141039120 as *const ()), // sub_141039120
    Replacement::draft(0x1410392b0, "unknown_1410392b0", fn_1410392b0 as *const ()), // sub_1410392b0
    Replacement::draft(0x1410393f0, "unknown_1410393f0", fn_1410393f0 as *const ()), // sub_1410393f0
    Replacement::draft(0x1410394f0, "unknown_1410394f0", fn_1410394f0 as *const ()), // sub_1410394f0
    Replacement::draft(0x141039660, "unknown_141039660", fn_141039660 as *const ()), // sub_141039660
    Replacement::draft(0x1410397e0, "unknown_1410397e0", fn_1410397e0 as *const ()), // sub_1410397e0
    Replacement::draft(0x141039950, "unknown_141039950", fn_141039950 as *const ()), // sub_141039950
    Replacement::draft(0x141039ac0, "unknown_141039ac0", fn_141039ac0 as *const ()), // sub_141039ac0
    Replacement::draft(0x141039c40, "unknown_141039c40", fn_141039c40 as *const ()), // sub_141039c40
    Replacement::draft(0x141039dc0, "unknown_141039dc0", fn_141039dc0 as *const ()), // sub_141039dc0
    Replacement::draft(0x141039f40, "unknown_141039f40", fn_141039f40 as *const ()), // sub_141039f40
    Replacement::draft(0x14103a750, "unknown_14103a750", fn_14103a750 as *const ()), // sub_14103a750
    Replacement::draft(0x14103a8d0, "unknown_14103a8d0", fn_14103a8d0 as *const ()), // sub_14103a8d0
    Replacement::draft_throws(0x14103a930, "coregame_14103a930", fn_14103a930 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::gatherResources>
    Replacement::draft(0x14103ab20, "coregame_14103ab20", tpl_5879::<0x143cd5140, 0x143cd42b8, 0x1411e9f50, 0x141047a60, 0x143c8f7a0, 0x1401fc320, 0x143ccef2c, 0x140215610, 0x1420a28e0, 0x1410468d0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335e0b6, 0x142646960, 0x143cd42d8, 0x143cd42dc, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::mesh::destructRenderObjects>
    Replacement::draft_throws(0x14103ae10, "coregame_14103ae10", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141042b40, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103ae10
    Replacement::draft_throws(0x14103b0b0, "coregame_14103b0b0", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141042e40, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103b0b0
    Replacement::draft_throws(0x14103b350, "coregame_14103b350", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141043160, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103b350
    Replacement::draft_throws(0x14103b5f0, "coregame_14103b5f0", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141043440, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103b5f0
    Replacement::draft_throws(0x14103b890, "coregame_14103b890", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141043760, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103b890
    Replacement::draft_throws(0x14103bb30, "coregame_14103bb30", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141043a60, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103bb30
    Replacement::draft_throws(0x14103bdd0, "coregame_14103bdd0", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141043d60, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103bdd0
    Replacement::draft_throws(0x14103c070, "coregame_14103c070", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x1410440b0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103c070
    Replacement::draft_throws(0x14103c310, "coregame_14103c310", tpl_1480::<0x1428d8ac8, 0x1410d6ca0, 0x1410443b0, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14103c310
    Replacement::draft_throws(0x14103c5b0, "coregame_14103c5b0", tpl_1480::<0x1428d8ac8, 0x1410d6ca0, 0x141044690, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14103c5b0
    Replacement::draft_throws(0x14103c850, "coregame_14103c850", tpl_1480::<0x1428d8ac8, 0x1410d6ca0, 0x141044990, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14103c850
    Replacement::draft_throws(0x14103caf0, "coregame_14103caf0", tpl_1480::<0x1428d8ac8, 0x1410d6ca0, 0x141044ca0, 0x143823a08, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x1438239f8> as *const (), Throws::Discard), // sub_14103caf0
    Replacement::draft_throws(0x14103cd90, "coregame_14103cd90", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141044fb0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103cd90
    Replacement::draft_throws(0x14103d030, "coregame_14103d030", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x1410451c0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103d030
    Replacement::draft_throws(0x14103d2d0, "coregame_14103d2d0", tpl_1480::<0x1428d8a98, 0x1410d6ca0, 0x141045410, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const (), Throws::Discard), // sub_14103d2d0
    Replacement::draft(0x14103d570, "coregame_14103d570", tpl_1466::<0x143cd4468, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103d570
    Replacement::draft(0x14103d840, "coregame_14103d840", tpl_1466::<0x143cd48a8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103d840
    Replacement::draft(0x14103db10, "coregame_14103db10", tpl_1402::<0x143c900a8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103db10
    Replacement::draft(0x14103de60, "coregame_14103de60", tpl_1466::<0x143cd4dcc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103de60
    Replacement::draft(0x14103e130, "coregame_14103e130", tpl_1094::<0x143c8f7a0, 0x141045a90, 0x14020e8f0> as *const ()), // sub_14103e130
    Replacement::draft(0x14103e330, "coregame_14103e330", tpl_1094::<0x143cac928, 0x141045c20, 0x14020e8f0> as *const ()), // sub_14103e330
    Replacement::draft(0x14103e530, "coregame_14103e530", tpl_20099::<0x143c8f6dc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103e530
    Replacement::draft(0x14103e890, "coregame_14103e890", tpl_1094::<0x143cd445c, 0x141045dc0, 0x14020e8f0> as *const ()), // sub_14103e890
    Replacement::draft(0x14103ea90, "coregame_14103ea90", fn_14103ea90 as *const ()), // sub_14103ea90
    Replacement::draft(0x14103eb60, "coregame_14103eb60", tpl_1095::<0x143cd4fa0, 0x14020e8f0> as *const ()), // sub_14103eb60
    Replacement::draft(0x14103ed40, "coregame_14103ed40", tpl_1094::<0x143cd4470, 0x14068b330, 0x14020e8f0> as *const ()), // sub_14103ed40
    Replacement::draft(0x14103ef40, "coregame_14103ef40", tpl_1094::<0x143cd4970, 0x141045f60, 0x14020e8f0> as *const ()), // sub_14103ef40
    Replacement::draft(0x14103f140, "coregame_14103f140", tpl_4899::<0x143c8e2e8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103f140
    Replacement::draft(0x14103f490, "coregame_14103f490", tpl_4389::<0x143c8de70, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103f490
    Replacement::draft(0x14103f7e0, "coregame_14103f7e0", tpl_3077::<0x143c8e070, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103f7e0
    Replacement::draft(0x14103fb10, "coregame_14103fb10", tpl_1466::<0x143c9c6e4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103fb10
    Replacement::draft(0x14103fde0, "coregame_14103fde0", tpl_3077::<0x143c8dbc8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14103fde0
    Replacement::draft(0x141040110, "coregame_141040110", tpl_1368::<0x143c9a338, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141040110
    Replacement::draft(0x141040440, "coregame_141040440", tpl_4324::<0x143cd501c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141040440
    Replacement::draft(0x141040780, "coregame_141040780", tpl_3074::<0x143cd4478, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141040780
    Replacement::draft(0x141040a50, "coregame_141040a50", tpl_1098::<0x143cc515c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141040a50
    Replacement::draft_throws(0x141040dc0, "coregame_141040dc0", tpl_3011::<0x1411e9f50, 0x14104b500, 0x143cd5154, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141046380, 0x1401de410> as *const (), Throws::Discard), // sub_141040dc0
    Replacement::draft_throws(0x141040f80, "coregame_141040f80", fn_141040f80 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamOutMesh>
    Replacement::draft_throws(0x1410411d0, "coregame_1410411d0", tpl_1404::<0x143cd5144, 0x143cd4208, 0x1411e9f50, 0x14104c3a0, 0x143cd4748, 0x1420af120, 0x1420a28e0, 0x1410495e0, 0x141046560, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335ec06, 0x142646960, 0x143cd4228, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamOutResource>
    Replacement::draft_throws(0x1410414a0, "coregame_1410414a0", tpl_1404::<0x143cd4754, 0x143cd4e70, 0x1411e9f50, 0x14104c490, 0x143cd44fc, 0x1420af120, 0x1420a28e0, 0x1410496d0, 0x141046610, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335ea66, 0x142646960, 0x143cd4e90, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamOutSkinning>
    Replacement::draft_throws(0x141041770, "coregame_141041770", tpl_3061::<0x143cd49e8, 0x143cd41a8, 0x1411e9f50, 0x14104c580, 0x143cd4464, 0x1420af120, 0x1420a28e0, 0x1410497c0, 0x1410466c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335e8b6, 0x142646960, 0x143cd41c8, 0x143cd41d0, 0x143cd41d4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamOutUseGeometryTransform>
    Replacement::draft_throws(0x141041a60, "coregame_141041a60", tpl_3393::<0x143cd4fb0, 0x143cd4a10, 0x1411e9f50, 0x14104c670, 0x143cd5310, 0x1420af120, 0x1420a28e0, 0x1410498b0, 0x141046770, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335e6f6, 0x142646960, 0x143cd4a30, 0x143cd4a40, 0x143cd4a48, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamOutGeometryTransformUpdatesEnabled>
    Replacement::draft_throws(0x141041d50, "coregame_141041d50", tpl_4129::<0x143cd5078, 0x143cd4b80, 0x1411e9f50, 0x14104c760, 0x143cd4454, 0x1420af120, 0x1420a28e0, 0x1410499a0, 0x141046820, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335e556, 0x142646960, 0x143cd4ba0, 0x143cd4ba4, 0x143cd4ba6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamOutSkeletonBones>
    Replacement::draft_throws(0x141042040, "coregame_141042040", tpl_19370::<0x143cd4790, 0x143cd4ed0, 0x1411e9f50, 0x14104c850, 0x143cd4ecc, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x141049a90, 0x141046980, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335e336, 0x142646960, 0x143cd4ef0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamInResource>
    Replacement::draft_throws(0x141042340, "coregame_141042340", fn_141042340 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamIn>
    Replacement::draft_throws(0x141042580, "coregame_141042580", tpl_1467::<0x143cd4f20, 0x143cd4418, 0x1411e9f50, 0x14104c950, 0x143cd5264, 0x1420af120, 0x1420a28e0, 0x141049ce0, 0x141046c60, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433601e6, 0x142646960, 0x143cd4438, 0x143cd4440, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamInSkeletonBoneMapping>
    Replacement::draft_throws(0x141042860, "coregame_141042860", tpl_3683::<0x143cd513c, 0x143cd4838, 0x1411e9f50, 0x14104ca80, 0x143cd4448, 0x1420af120, 0x1420a28e0, 0x141049dd0, 0x141046d10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335ffe6, 0x142646960, 0x143cd4858, 0x143cd4860, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::streamInSlaveBoneMapping>
    Replacement::draft_throws(0x141042b40, "coregame_141042b40", tpl_19370::<0x143cd4760, 0x143cd4650, 0x1411e9f50, 0x14104cb80, 0x143cd4fa4, 0x1420af120, 0x143c900d8, 0x140215610, 0x1420a28e0, 0x141049ec0, 0x141046dc0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335fbb6, 0x142646960, 0x143cd4670, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::interpolateBones>
    Replacement::draft_throws(0x141042e40, "coregame_141042e40", tpl_4322::<0x143cd4bb0, 0x143cd4c70, 0x1411e9f50, 0x14104cd90, 0x143cd4604, 0x1420af120, 0x141047eb0, 0x143c900d8, 0x140215610, 0x1420a28e0, 0x14104a070, 0x141046e80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335f726, 0x142646960, 0x143cd4c90, 0x143cd4c98, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::interpolateBonesForSlave>
    Replacement::draft_throws(0x141043160, "coregame_141043160", tpl_1437::<0x143cd526c, 0x143cd4508, 0x1411e9f50, 0x14104cf50, 0x143cd4648, 0x1420af120, 0x1420a28e0, 0x14104a270, 0x141046fa0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335f4c6, 0x142646960, 0x143cd4528, 0x143cd4530, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::finalizeGeometryTransform>
    Replacement::draft_throws(0x141043440, "coregame_141043440", fn_141043440 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::finalizeBones>
    Replacement::draft_throws(0x141043760, "coregame_141043760", tpl_1101::<0x143cd4920, 0x143cd4538, 0x1411e9f50, 0x14104d190, 0x143cd4450, 0x1420af120, 0x1420a28e0, 0x14104a620, 0x1410470b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14335f086, 0x142646960, 0x143cd4558, 0x143cd4560, 0x143cd4564, 0x143cd4566, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::updateDirtySkeletonBoneMapping>
    Replacement::draft_throws(0x141043a60, "coregame_141043a60", tpl_4834::<0x143cd4868, 0x143cd5238, 0x1411e9f50, 0x14104d2d0, 0x143cd4644, 0x1420af120, 0x141047eb0, 0x1420a28e0, 0x14104a820, 0x141047130, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143361d16, 0x142646960, 0x143cd5258, 0x143cd5260, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::updateDirtySlaveBoneMapping>
    Replacement::draft_throws(0x141043d60, "coregame_141043d60", tpl_4481::<0x143cd42b0, 0x143cd53f0, 0x1411e9f50, 0x14104d3e0, 0x143cd4d40, 0x1420af120, 0x143c900d8, 0x140215610, 0x143ccef2c, 0x1420a28e0, 0x14104aa80, 0x141047210, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143361786, 0x142646960, 0x143cd5410, 0x143cd5414, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::interpolateTransforms>
    Replacement::draft_throws(0x1410440b0, "coregame_1410440b0", fn_1410440b0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::applyHide>
    Replacement::draft_throws(0x1410443b0, "coregame_1410443b0", tpl_1437::<0x143cd4a08, 0x143cd41d8, 0x1411e9f50, 0x14104bc30, 0x143cd4458, 0x1420af120, 0x1420a28e0, 0x14104ad20, 0x141047300, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433612b6, 0x142646960, 0x143cd41f8, 0x143cd4200, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::setGeometryTransformBones>
    Replacement::draft_throws(0x141044690, "coregame_141044690", tpl_4108::<0x143cd45fc, 0x143cd4570, 0x1411e9f50, 0x14104baf0, 0x143cd4f24, 0x1420af120, 0x141047eb0, 0x1420a28e0, 0x14104af30, 0x141047420, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143360fa6, 0x142646960, 0x143cd4590, 0x143cd45a0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::setGeometryTransformBonesForSlave>
    Replacement::draft_throws(0x141044990, "coregame_141044990", tpl_19802::<0x143cd4f64, 0x143cd45d8, 0x1411e9f50, 0x14104ba00, 0x143cd5084, 0x1420af120, 0x140215750, 0x1420a28e0, 0x14104b1a0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143360e36, 0x142646960, 0x143cd45e8, 0x143cd45f0, 0x143cd45f4, 0x143cd45f6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::teleportMeshes>
    Replacement::draft_throws(0x141044ca0, "coregame_141044ca0", fn_141044ca0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::updateMeshFlagFromScript>
    Replacement::draft_throws(0x141044fb0, "coregame_141044fb0", tpl_1793::<0x143cd4fac, 0x143cd4120, 0x1410460f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143360b06, 0x142646960, 0x143cd4140, 0x143cd4148, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::updateFromGameParameters>
    Replacement::draft_throws(0x1410451c0, "coregame_1410451c0", tpl_1578::<0x143cd4dc0, 0x143cd45b0, 0x1411e9f50, 0x1410475e0, 0x14104b2e0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143362556, 0x142646960, 0x143cd45d0, 0x143cd45d4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::updateParameterBlends>
    Replacement::draft_throws(0x141045410, "coregame_141045410", fn_141045410 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::mesh::updateMaterialsToRenderer>
    Replacement::draft(0x141045720, "std_141045720", tpl_18960::<0x10, 0x142442de0> as *const ()), // std::_Ref_count_obj_alloc3<rend::GeometryTransformBoneData,r::Allocator<rend::GeometryTransformBoneData,0>>::std::_Ref_c
    Replacement::draft(0x141045740, "std_141045740", tpl_6019::<0x143364f78, 0x141834d30> as *const ()), // std::_Ref_count_obj_alloc3<rend::GeometryTransformBoneData,r::Allocator<rend::GeometryTransformBoneData,0>>::std::_Ref_c
    Replacement::draft(0x141045770, "unknown_141045770", tpl_60953::<0x1426467e8, 0x143c8f7a0, 0x142489ee0, 0x1410457ec> as *const ()), // sub_141045770
    Replacement::draft(0x141045920, "unknown_141045920", tpl_61019::<0x143cd501c, 0x142489ee0, 0x143c8de70, 0x143c8e070, 0x1426467e8, 0x141045a83> as *const ()), // sub_141045920
    Replacement::draft(0x141045a90, "unknown_141045a90", fn_141045a90 as *const ()), // sub_141045a90
    Replacement::draft(0x141045c20, "unknown_141045c20", fn_141045c20 as *const ()), // sub_141045c20
    Replacement::draft(0x141045dc0, "unknown_141045dc0", fn_141045dc0 as *const ()), // sub_141045dc0
    Replacement::draft(0x141045f60, "unknown_141045f60", fn_141045f60 as *const ()), // sub_141045f60
    Replacement::draft_throws(0x1410460f0, "unknown_1410460f0", tpl_3697::<0x1411e9f50, 0x14104b840, 0x143cd49ec, 0x1420af120, 0x140215750, 0x143c8ce80, 0x140215610, 0x1420a28e0, 0x14104b260, 0x1401de410> as *const (), Throws::Discard), // sub_1410460f0
    Replacement::draft(0x1410462a0, "unknown_1410462a0", tpl_6490::<0x18, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_1410462a0
    Replacement::draft(0x1410462f0, "unknown_1410462f0", fn_1410462f0 as *const ()), // sub_1410462f0
    Replacement::draft(0x141046340, "unknown_141046340", fn_141046340 as *const ()), // sub_141046340
    Replacement::draft(0x141046380, "unknown_141046380", tpl_3028::<0x143cd5154, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141eac250> as *const ()), // sub_141046380
    Replacement::draft_throws(0x1410463f0, "unknown_1410463f0", tpl_4746::<0x14104c260, 0x143cd4738, 0x1420af120, 0x140215750, 0x143ccef2c, 0x140215610, 0x143c8ce8c, 0x1420a28e0> as *const (), Throws::Discard), // sub_1410463f0
    Replacement::draft(0x141046560, "unknown_141046560", tpl_19920::<0x142407c80, 0x141eadd50> as *const ()), // sub_141046560
    Replacement::draft(0x141046610, "unknown_141046610", tpl_19920::<0x142407c80, 0x141ead030> as *const ()), // sub_141046610
    Replacement::draft(0x1410466c0, "unknown_1410466c0", tpl_19920::<0x142407c80, 0x141eacf00> as *const ()), // sub_1410466c0
    Replacement::draft(0x141046770, "unknown_141046770", tpl_19920::<0x142407c80, 0x141eacdd0> as *const ()), // sub_141046770
    Replacement::draft(0x141046820, "unknown_141046820", tpl_19920::<0x142407c80, 0x141eacca0> as *const ()), // sub_141046820
    Replacement::draft(0x1410468d0, "unknown_1410468d0", tpl_3172::<0x143c8f7a0, 0x5e308, 0x143ccef2c, 0x580e0, 0x10, 0x8, 0x18, 0x141eac9b0, 0x141048e30> as *const ()), // sub_1410468d0
    Replacement::draft_throws(0x141046980, "unknown_141046980", fn_141046980 as *const (), Throws::Caught), // sub_141046980
    Replacement::draft_throws(0x141046ab0, "unknown_141046ab0", fn_141046ab0 as *const (), Throws::Discard), // sub_141046ab0
    Replacement::draft(0x141046c60, "unknown_141046c60", tpl_19920::<0x142407c80, 0x141eadbf0> as *const ()), // sub_141046c60
    Replacement::draft(0x141046d10, "unknown_141046d10", tpl_19920::<0x142407c80, 0x141eada90> as *const ()), // sub_141046d10
    Replacement::draft_throws(0x141046e80, "unknown_141046e80", fn_141046e80 as *const (), Throws::Discard), // sub_141046e80
    Replacement::draft(0x141046fa0, "unknown_141046fa0", tpl_19286::<0x88, 0x10088, 0x1410482f0, 0x141eb0580> as *const ()), // sub_141046fa0
    Replacement::draft(0x141047010, "unknown_141047010", tpl_77679::<0x88, 0x10088, 0x143ccef2c, 0x580e0, 0x1410481f0, 0x141eb0210> as *const ()), // sub_141047010
    Replacement::draft(0x141047130, "unknown_141047130", fn_141047130 as *const ()), // sub_141047130
    Replacement::draft(0x141047210, "unknown_141047210", fn_141047210 as *const ()), // sub_141047210
    Replacement::draft(0x141047300, "unknown_141047300", tpl_62730::<0x88, 0x10088, 0x141047d80, 0x141eb09b0> as *const ()), // sub_141047300
    Replacement::draft(0x141047420, "unknown_141047420", tpl_1781::<0x88, 0x10088, 0x143cd4460, 0x20, 0x8, 0x141047c50, 0x141047500> as *const ()), // sub_141047420
    Replacement::draft(0x141047500, "unknown_141047500", tpl_1782::<0x18, 0x20, 0x30, 0x10, 0x28, 0x8, 0x141eb08e0> as *const ()), // sub_141047500
    Replacement::draft_throws(0x1410475e0, "unknown_1410475e0", tpl_5909::<0x14104b730, 0x143cd4750, 0x1420af120, 0x140215750, 0x143cd53a4, 0x140215610, 0x143c904a0, 0x143c8d904, 0x1420a28e0> as *const (), Throws::Discard), // sub_1410475e0
    Replacement::draft(0x141047a60, "unknown_141047a60", tpl_3188::<0x14104b460, 0x143365010, 0x141048e30, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141047a60
    Replacement::draft(0x141048cc0, "unknown_141048cc0", fn_141048cc0 as *const ()), // sub_141048cc0
    Replacement::draft(0x141048e30, "unknown_141048e30", fn_141048e30 as *const ()), // sub_141048e30
    Replacement::draft_throws(0x14104d800, "unknown_14104d800", tpl_4541::<0x143cd4460, 0x14104beb0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104d800
    Replacement::draft_throws(0x14104d890, "unknown_14104d890", tpl_4541::<0x143cd4c40, 0x14104b5f0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104d890
    Replacement::draft_throws(0x14104d920, "unknown_14104d920", tpl_4541::<0x143cd4750, 0x14104b730, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104d920
    Replacement::draft_throws(0x14104d9b0, "unknown_14104d9b0", tpl_4541::<0x143cd49ec, 0x14104b840, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104d9b0
    Replacement::draft_throws(0x14104da40, "unknown_14104da40", tpl_4541::<0x143cd446c, 0x14104b8f0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104da40
    Replacement::draft_throws(0x14104dad0, "unknown_14104dad0", tpl_4541::<0x143cd5084, 0x14104ba00, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104dad0
    Replacement::draft_throws(0x14104db60, "unknown_14104db60", tpl_4541::<0x143cd4f24, 0x14104baf0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104db60
    Replacement::draft_throws(0x14104dbf0, "unknown_14104dbf0", tpl_4541::<0x143cd4458, 0x14104bc30, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104dbf0
    Replacement::draft_throws(0x14104dc80, "unknown_14104dc80", tpl_4541::<0x143cd4bac, 0x14104bd70, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104dc80
    Replacement::draft_throws(0x14104dd10, "unknown_14104dd10", tpl_4541::<0x143cd4738, 0x14104c260, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104dd10
    Replacement::draft_throws(0x14104dda0, "unknown_14104dda0", tpl_4541::<0x143cd4748, 0x14104c3a0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104dda0
    Replacement::draft_throws(0x14104de30, "unknown_14104de30", tpl_4541::<0x143cd44fc, 0x14104c490, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104de30
    Replacement::draft_throws(0x14104dec0, "unknown_14104dec0", tpl_4541::<0x143cd4464, 0x14104c580, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104dec0
    Replacement::draft_throws(0x14104df50, "unknown_14104df50", tpl_4541::<0x143cd5310, 0x14104c670, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104df50
    Replacement::draft_throws(0x14104dfe0, "unknown_14104dfe0", tpl_4541::<0x143cd4454, 0x14104c760, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104dfe0
    Replacement::draft_throws(0x14104e070, "unknown_14104e070", tpl_4541::<0x143cd4ecc, 0x14104c850, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e070
    Replacement::draft_throws(0x14104e100, "unknown_14104e100", tpl_4541::<0x143cd5264, 0x14104c950, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e100
    Replacement::draft_throws(0x14104e190, "unknown_14104e190", tpl_4541::<0x143cd4448, 0x14104ca80, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e190
    Replacement::draft_throws(0x14104e220, "unknown_14104e220", tpl_4541::<0x143cd4fa4, 0x14104cb80, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e220
    Replacement::draft_throws(0x14104e2b0, "unknown_14104e2b0", tpl_4541::<0x143cd4604, 0x14104cd90, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e2b0
    Replacement::draft_throws(0x14104e340, "unknown_14104e340", tpl_4541::<0x143cd4648, 0x14104cf50, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e340
    Replacement::draft_throws(0x14104e3d0, "unknown_14104e3d0", tpl_4541::<0x143cd4474, 0x14104d080, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e3d0
    Replacement::draft_throws(0x14104e460, "unknown_14104e460", tpl_4541::<0x143cd4450, 0x14104d190, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e460
    Replacement::draft_throws(0x14104e4f0, "unknown_14104e4f0", tpl_4541::<0x143cd4644, 0x14104d2d0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e4f0
    Replacement::draft_throws(0x14104e580, "unknown_14104e580", tpl_4541::<0x143cd4d40, 0x14104d3e0, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190> as *const (), Throws::Discard), // sub_14104e580
    Replacement::draft_throws(0x14104e810, "unknown_14104e810", fn_14104e810 as *const (), Throws::Discard), // sub_14104e810
    Replacement::draft(0x14104eb20, "unknown_14104eb20", tpl_365::<0x14104ed20> as *const ()), // sub_14104eb20
    Replacement::draft_throws(0x14104eb30, "unknown_14104eb30", tpl_20152::<0x14104f120, 0x38, 0x14104efd0, 0x18, 0x141051540, 0x1410510f0, 0x141051320, 0x20> as *const (), Throws::Discard), // sub_14104eb30
    Replacement::draft(0x14104ebe0, "unknown_14104ebe0", fn_14104ebe0 as *const ()), // sub_14104ebe0
    Replacement::draft(0x14104ed20, "unknown_14104ed20", tpl_20006::<0x8, 0x14104f970, 0xffffffffffffffff, 0x100, 0x108, 0x4, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200, 0x10> as *const ()), // sub_14104ed20
    Replacement::draft_throws(0x14104ee00, "unknown_14104ee00", tpl_20152::<0x14104f120, 0x38, 0x14104efd0, 0x18, 0x141051be0, 0x141051790, 0x1410519c0, 0x20> as *const (), Throws::Discard), // sub_14104ee00
    Replacement::draft(0x14104eeb0, "unknown_14104eeb0", fn_14104eeb0 as *const ()), // sub_14104eeb0
    Replacement::draft(0x14104efd0, "unknown_14104efd0", fn_14104efd0 as *const ()), // sub_14104efd0
    Replacement::draft_throws(0x14104f0a0, "unknown_14104f0a0", tpl_20156::<0x8, 0x10, 0x18, 0x20, 0xffffffffffffffff, 0x14104f3c0> as *const (), Throws::Discard), // sub_14104f0a0
    Replacement::draft(0x14104f120, "unknown_14104f120", fn_14104f120 as *const ()), // sub_14104f120
    Replacement::draft_throws(0x14104f330, "unknown_14104f330", tpl_20156::<0x8, 0x10, 0x18, 0x20, 0xffffffffffffffff, 0x14104f6f0> as *const (), Throws::Discard), // sub_14104f330
    Replacement::draft(0x14104f3b0, "unknown_14104f3b0", tpl_170::<0x14104f970> as *const ()), // sub_14104f3b0
    Replacement::draft_throws(0x14104f3c0, "unknown_14104f3c0", tpl_78361::<0x8, 0x14104f970, 0x10, 0x141051fe0, 0x141052280> as *const (), Throws::Discard), // sub_14104f3c0
    Replacement::draft(0x14104f470, "sparsehash_14104f470", fn_14104f470 as *const ()), // sub_14104f470
    Replacement::draft_throws(0x14104f510, "sparsehash_14104f510", fn_14104f510 as *const (), Throws::Caught), // sub_14104f510
    Replacement::draft(0x14104f640, "sparsehash_14104f640", fn_14104f640 as *const ()), // sub_14104f640
    Replacement::draft_throws(0x14104f6f0, "sparsehash_14104f6f0", tpl_78361::<0x8, 0x14104f970, 0x10, 0x1410522e0, 0x141052580> as *const (), Throws::Discard), // sub_14104f6f0
    Replacement::draft(0x14104f7a0, "sparsehash_14104f7a0", fn_14104f7a0 as *const ()), // sub_14104f7a0
    Replacement::draft_throws(0x14104f840, "sparsehash_14104f840", fn_14104f840 as *const (), Throws::Caught), // sub_14104f840
    Replacement::draft(0x14104f970, "sparsehash_14104f970", fn_14104f970 as *const ()), // sub_14104f970
    Replacement::draft_throws(0x14104fa20, "sparsehash_14104fa20", fn_14104fa20 as *const (), Throws::Discard), // sub_14104fa20
    Replacement::draft_throws(0x141050090, "sparsehash_141050090", fn_141050090 as *const (), Throws::Discard), // sub_141050090
    Replacement::draft(0x141050750, "sparsehash_141050750", fn_141050750 as *const ()), // sub_141050750
    Replacement::draft_throws(0x141050880, "std_141050880", tpl_20162::<0x14104f3c0, 0x141050ce0> as *const (), Throws::Discard), // sub_141050880
    Replacement::draft(0x1410509c0, "sparsehash_1410509c0", fn_1410509c0 as *const ()), // sub_1410509c0
    Replacement::draft_throws(0x141050b20, "std_141050b20", tpl_20162::<0x14104f6f0, 0x141050f30> as *const (), Throws::Discard), // sub_141050b20
    Replacement::draft(0x141050c60, "sparsehash_141050c60", fn_141050c60 as *const ()), // sub_141050c60
    Replacement::draft(0x141050ce0, "sparsehash_141050ce0", fn_141050ce0 as *const ()), // sub_141050ce0
    Replacement::draft(0x141050e30, "sparsehash_141050e30", fn_141050e30 as *const ()), // sub_141050e30
    Replacement::draft(0x141050f30, "sparsehash_141050f30", fn_141050f30 as *const ()), // sub_141050f30
    Replacement::draft(0x1410510b0, "sparsehash_1410510b0", tpl_20031::<0x14104f970> as *const ()), // sub_1410510b0
    Replacement::draft_throws(0x1410510f0, "sparsehash_1410510f0", tpl_78525::<0x14104f120, 0x141020600, 0x141051320> as *const (), Throws::Discard), // sub_1410510f0
    Replacement::draft_throws(0x141051320, "sparsehash_141051320", tpl_78526::<0x14104f640, 0x141050750, 0x141020600, 0x14307aa70, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x141051537> as *const (), Throws::Discard), // sub_141051320
    Replacement::draft_throws(0x141051540, "sparsehash_141051540", tpl_78527::<0x143426fc8, 0x14104f510, 0x14104f470, 0x14104ed20, 0x1401e2020, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x141051786> as *const (), Throws::Discard), // sub_141051540
    Replacement::draft_throws(0x141051790, "sparsehash_141051790", tpl_78525::<0x14104f120, 0x141020600, 0x1410519c0> as *const (), Throws::Discard), // sub_141051790
    Replacement::draft_throws(0x1410519c0, "sparsehash_1410519c0", tpl_78526::<0x14104f640, 0x1410509c0, 0x141020600, 0x14307aa70, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x141051bd7> as *const (), Throws::Discard), // sub_1410519c0
    Replacement::draft_throws(0x141051be0, "sparsehash_141051be0", tpl_78527::<0x143426fc8, 0x14104f840, 0x14104f7a0, 0x14104ed20, 0x1401e2020, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x141051e26> as *const (), Throws::Discard), // sub_141051be0
    Replacement::draft(0x141051e30, "sparsehash_141051e30", fn_141051e30 as *const ()), // sub_141051e30
    Replacement::draft_throws(0x141051fe0, "sparsehash_141051fe0", tpl_20037::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x141052280, 0x141052620, 0x14104f970, 0x1425371a0, 0x142533200, 0x1402032e0, 0x141052270> as *const (), Throws::Discard), // sub_141051fe0
    Replacement::draft(0x141052280, "sparsehash_141052280", tpl_20038::<0x141052620> as *const ()), // sub_141052280
    Replacement::draft_throws(0x1410522e0, "sparsehash_1410522e0", tpl_20037::<0x142537280, 0x142535070, 0x1426456c8, 0x1426453a0, 0x141052580, 0x141052680, 0x14104f970, 0x1425371a0, 0x142533200, 0x1402032e0, 0x141052570> as *const (), Throws::Discard), // sub_1410522e0
    Replacement::draft(0x141052580, "sparsehash_141052580", tpl_20038::<0x141052680> as *const ()), // sub_141052580
    Replacement::draft(0x1410525e0, "sparsehash_1410525e0", tpl_16383::<0x8, 0x14104f970> as *const ()), // sub_1410525e0
    Replacement::draft(0x141052620, "sparsehash_141052620", fn_141052620 as *const ()), // sub_141052620
    Replacement::draft(0x141052680, "sparsehash_141052680", fn_141052680 as *const ()), // sub_141052680
    Replacement::draft(0x141052720, "sparsehash_141052720", fn_141052720 as *const ()), // sub_141052720
    Replacement::draft_throws(0x1410527b0, "sparsehash_1410527b0", fn_1410527b0 as *const (), Throws::Discard), // sub_1410527b0
    Replacement::draft_throws(0x141052860, "sparsehash_141052860", fn_141052860 as *const (), Throws::Discard), // sub_141052860
    Replacement::draft(0x141052900, "sparsehash_141052900", fn_141052900 as *const ()), // sub_141052900
    Replacement::draft(0x141052910, "sparsehash_141052910", fn_141052910 as *const ()), // sub_141052910
    Replacement::draft(0x1410529c0, "sparsehash_1410529c0", tpl_422::<0x70, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_1410529c0
    Replacement::draft(0x141052a50, "sparsehash_141052a50", fn_141052a50 as *const ()), // sub_141052a50
    Replacement::draft(0x141052b70, "sparsehash_141052b70", tpl_60622::<0x10, 0x18, 0x143c9a620, 0x142489ee0, 0xc03, 0x143cd558c, 0x20, 0x38, 0x40> as *const ()), // sub_141052b70
    Replacement::draft(0x141052c70, "sparsehash_141052c70", fn_141052c70 as *const ()), // sub_141052c70
    Replacement::draft(0x141052dc0, "sparsehash_141052dc0", fn_141052dc0 as *const ()), // sub_141052dc0
    Replacement::draft(0x141052f00, "unknown_141052f00", fn_141052f00 as *const ()), // sub_141052f00
    Replacement::draft_throws(0x141053080, "unknown_141053080", fn_141053080 as *const (), Throws::Discard), // sub_141053080
    Replacement::draft(0x1410531f0, "unknown_1410531f0", fn_1410531f0 as *const ()), // sub_1410531f0
    Replacement::draft_throws(0x141053350, "unknown_141053350", fn_141053350 as *const (), Throws::Discard), // sub_141053350
    Replacement::draft_throws(0x141053650, "coregame_141053650", tpl_3004::<0x143cd56a4, 0x143cd54c0, 0x141055050, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143365426, 0x142646960, 0x143cd54e0, 0x143cd54e4, 0x143cd54e6, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_systems::gatherResources>
    Replacement::draft(0x141053860, "coregame_141053860", tpl_2959::<0x143cd5550, 0x143cd5528, 0x1411e9f50, 0x141055ae0, 0x1410560b0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433655e6, 0x142646960, 0x143cd5548, 0x143cd554c, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::lua_systems::unregisterGlue>
    Replacement::draft_throws(0x141053ab0, "coregame_141053ab0", fn_141053ab0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_systems::processPendingRegistrations>
    Replacement::draft_throws(0x141053e10, "sparsehash_141053e10", fn_141053e10 as *const (), Throws::Discard), // sub_141053e10
    Replacement::draft_throws(0x141053ff0, "sparsehash_141053ff0", fn_141053ff0 as *const (), Throws::Discard), // sub_141053ff0
    Replacement::draft_throws(0x141054150, "sparsehash_141054150", fn_141054150 as *const (), Throws::Discard), // sub_141054150
    Replacement::draft_throws(0x141054390, "sparsehash_141054390", tpl_77984::<0x140fbf340, 0x1410531f0, 0x140227190, 0x1401e2020, 0x143426fc8, 0x1430790d8, 0x1401df630, 0x1437fcf88, 0x14183acce, 0x1410545c2> as *const (), Throws::Discard), // sub_141054390
    Replacement::draft(0x1410545d0, "unknown_1410545d0", tpl_1368::<0x143cd55e8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410545d0
    Replacement::draft(0x141054900, "unknown_141054900", tpl_1094::<0x143cd558c, 0x1410557d0, 0x14020e8f0> as *const ()), // sub_141054900
    Replacement::draft(0x141054b00, "unknown_141054b00", tpl_1095::<0x143cd5584, 0x14020e8f0> as *const ()), // sub_141054b00
    Replacement::draft(0x141054ce0, "unknown_141054ce0", tpl_1094::<0x143cd565c, 0x141055950, 0x14020e8f0> as *const ()), // sub_141054ce0
    Replacement::draft(0x141054ee0, "unknown_141054ee0", tpl_1436::<0x58, 0x60, 0x158, 0x143cd565c, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141054ee0
    Replacement::draft(0x141054fc0, "unknown_141054fc0", tpl_20183::<0x10, 0x8, 0x141c51430, 0x141054900, 0x142442de0, 0x141054b00> as *const ()), // sub_141054fc0
    Replacement::draft_throws(0x141055050, "unknown_141055050", tpl_3011::<0x1411e9f50, 0x141056d20, 0x143cd5598, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x141055ea0, 0x1401de410> as *const (), Throws::Discard), // sub_141055050
    Replacement::draft_throws(0x141055210, "coregame_141055210", tpl_1437::<0x143cd56ec, 0x143cd5628, 0x1411e9f50, 0x141057010, 0x143cd5588, 0x1420af120, 0x1420a28e0, 0x141056a40, 0x141055f10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143365996, 0x142646960, 0x143cd5648, 0x143cd5650, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_systems::streamOutResources>
    Replacement::draft_throws(0x1410554f0, "coregame_1410554f0", tpl_3683::<0x143cd5590, 0x143cd56f8, 0x1411e9f50, 0x141056f10, 0x143cd5594, 0x1420af120, 0x1420a28e0, 0x141056b20, 0x141055fa0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143365b46, 0x142646960, 0x143cd5718, 0x143cd5720, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_systems::streamInResources>
    Replacement::draft(0x1410557d0, "unknown_1410557d0", tpl_2731::<0x1401e6d50, 0x141055e30, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_1410557d0
    Replacement::draft(0x141055950, "unknown_141055950", tpl_4210::<0x1401e6d50, 0x141055e70, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060> as *const ()), // sub_141055950
    Replacement::draft(0x141055ae0, "unknown_141055ae0", fn_141055ae0 as *const ()), // sub_141055ae0
    Replacement::draft(0x141055c40, "unknown_141055c40", tpl_19516::<0x88, 0x10088, 0x143cd5810, 0x580e0, 0x143cd56e4, 0x142407c80, 0x141055d30, 0x580f0, 0x141ec09a0> as *const ()), // sub_141055c40
    Replacement::draft(0x141055e30, "unknown_141055e30", fn_141055e30 as *const ()), // sub_141055e30
    Replacement::draft(0x141055e70, "unknown_141055e70", tpl_1955::<0x10, 0x1403d1260> as *const ()), // sub_141055e70
    Replacement::draft(0x141055ea0, "unknown_141055ea0", tpl_3028::<0x143cd5598, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141ec05e0> as *const ()), // sub_141055ea0
    Replacement::draft(0x141055f10, "unknown_141055f10", tpl_4533::<0x10088, 0x142407c80, 0x580f0, 0x141054ee0> as *const ()), // sub_141055f10
    Replacement::draft_throws(0x141055fa0, "unknown_141055fa0", fn_141055fa0 as *const (), Throws::Caught), // sub_141055fa0
    Replacement::draft_throws(0x1410560b0, "unknown_1410560b0", fn_1410560b0 as *const (), Throws::Caught), // sub_1410560b0
    Replacement::draft(0x141056210, "unknown_141056210", tpl_19523::<0x143cd5810, 0x143cd56e4, 0x142407c80, 0x143cd558c, 0x142489ee0, 0x143c8d160, 0x141ec09a0> as *const ()), // sub_141056210
    Replacement::draft(0x1410564e0, "unknown_1410564e0", tpl_3188::<0x141056c80, 0x1433667f8, 0x140489dd0, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_1410564e0
    Replacement::draft(0x141056890, "unknown_141056890", tpl_20187::<0x1425371a0, 0x142533200, 0x141057100, 0x140215e80, 0x1401de830> as *const ()), // sub_141056890
    Replacement::draft(0x141056a40, "unknown_141056a40", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141054ee0> as *const ()), // sub_141056a40
    Replacement::draft_throws(0x141056b20, "unknown_141056b20", fn_141056b20 as *const (), Throws::Caught), // sub_141056b20
    Replacement::draft(0x141056c80, "unknown_141056c80", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x1433667f8, 0x8, 0x20> as *const ()), // sub_141056c80
    Replacement::draft(0x141057100, "unknown_141057100", fn_141057100 as *const ()), // sub_141057100
    Replacement::draft(0x141057460, "unknown_141057460", fn_141057460 as *const ()), // sub_141057460
    Replacement::draft_throws(0x141057660, "unknown_141057660", fn_141057660 as *const (), Throws::Caught), // sub_141057660
    Replacement::draft(0x141057ad0, "unknown_141057ad0", fn_141057ad0 as *const ()), // sub_141057ad0
    Replacement::draft(0x141057af0, "unknown_141057af0", tpl_20192::<0x48, 0x10, 0x40, 0x142537280, 0x1426456c8, 0x1426453a0, 0x18, 0x1433668c8, 0x14248cd50> as *const ()), // sub_141057af0
    Replacement::draft(0x141057ba0, "unknown_141057ba0", fn_141057ba0 as *const ()), // sub_141057ba0
    Replacement::draft(0x141057c00, "unknown_141057c00", fn_141057c00 as *const ()), // sub_141057c00
    Replacement::draft(0x141057e50, "unknown_141057e50", fn_141057e50 as *const ()), // sub_141057e50
    Replacement::draft(0x141058110, "unknown_141058110", fn_141058110 as *const ()), // sub_141058110
    Replacement::draft(0x141058310, "unknown_141058310", fn_141058310 as *const ()), // sub_141058310
    Replacement::draft_throws(0x141058390, "coregame_141058390", tpl_1394::<0x143cd5a68, 0x143cd60d0, 0x14105cc10, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143366af6, 0x142646960, 0x143cd60f0, 0x143cd60f4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::gatherResources>
    Replacement::draft(0x1410585a0, "coregame_1410585a0", fn_1410585a0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::lua_script::collectGarbage>
    Replacement::draft(0x141058830, "coregame_141058830", tpl_1526::<0x1428dbeb8, 0x1410d6ca0, 0x14105d3b0, 0x143823638, 0x14020e980, 0x14020afa0, 0x1420a6b70, 0x143823628> as *const ()), // sub_141058830
    Replacement::draft_throws(0x141058ad0, "coregame_141058ad0", fn_141058ad0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::luaScriptRemoved>
    Replacement::draft_throws(0x141058e70, "coregame_141058e70", tpl_3281::<0x143cd64e8, 0x143cd5a88, 0x1411e9f50, 0x141062f00, 0x143cd60c8, 0x1420af120, 0x1420a28e0, 0x14105f7e0, 0x14105e3c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433677a6, 0x142646960, 0x143cd5aa8, 0x143cd5ab0, 0x143cd5ab4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::updatePendingResources>
    Replacement::draft(0x141059160, "coregame_141059160", fn_141059160 as *const ()), // ecs::system::ecs::system::wrapper<coregame::lua_script::initLuaScript>
    Replacement::draft_throws(0x1410593b0, "coregame_1410593b0", tpl_4905::<0x143cd5904, 0x143cd5d18, 0x14105d670, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433679f6, 0x142646960, 0x143cd5d38, 0x143cd5d48, 0x143cd5d50, 0x143cd5d52, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::processVariableStreamingStateChanges>
    Replacement::draft_throws(0x1410595d0, "coregame_1410595d0", tpl_19267::<0x143cd5c4c, 0x143cd5ac0, 0x14105d830, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143367c46, 0x142646960, 0x143cd5ae0, 0x143cd5af0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::luaVariableStreamOutEvents>
    Replacement::draft_throws(0x1410597e0, "coregame_1410597e0", tpl_3691::<0x143cd5ff0, 0x143cd59b8, 0x1411e9f50, 0x141062c00, 0x143cd5d54, 0x1420af120, 0x1420a28e0, 0x14105fd80, 0x14105e710, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143367eb6, 0x142646960, 0x143cd59d8, 0x143cd59e8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::luaVariableStepStreamInEvents>
    Replacement::draft_throws(0x141059ac0, "coregame_141059ac0", tpl_3682::<0x143cd5b30, 0x143cd64c0, 0x1411e9f50, 0x141062b00, 0x143cd6538, 0x1420af120, 0x1420a28e0, 0x14105fee0, 0x14105e820, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433680c6, 0x142646960, 0x143cd64e0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::luaStreamInEvents>
    Replacement::draft_throws(0x141059da0, "coregame_141059da0", tpl_1176::<0x143cd5bb0, 0x143cd5c88, 0x14105d9f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433682b6, 0x142646960, 0x143cd5ca8, 0x143cd5cb8, 0x143cd5cba, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::processStreamingStateChanges>
    Replacement::draft_throws(0x141059fb0, "coregame_141059fb0", tpl_1793::<0x143cd5c54, 0x143cd6428, 0x14105dbb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143368546, 0x142646960, 0x143cd6448, 0x143cd6450, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::luaStreamOutEvents>
    Replacement::draft(0x14105a1c0, "coregame_14105a1c0", tpl_4203::<0x143cd6320, 0x143cd6580, 0x1411e9f50, 0x14105eb50, 0x141060470, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143366f36, 0x142646960, 0x143cd65a0, 0x143cd65a4, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::lua_script::luaFixedUpdate>
    Replacement::draft(0x14105a400, "coregame_14105a400", fn_14105a400 as *const ()), // ecs::system::ecs::system::wrapper<coregame::lua_script::variableUpdateLuaScript>
    Replacement::draft(0x14105a650, "unknown_14105a650", fn_14105a650 as *const ()), // sub_14105a650
    Replacement::draft(0x14105a920, "sparsehash_14105a920", fn_14105a920 as *const ()), // sub_14105a920
    Replacement::draft_throws(0x14105a960, "sparsehash_14105a960", fn_14105a960 as *const (), Throws::Discard), // sub_14105a960
    Replacement::draft_throws(0x14105aae0, "unknown_14105aae0", tpl_20204::<0x140fbf340, 0x140fbf1e0, 0x140227190> as *const (), Throws::Discard), // sub_14105aae0
    Replacement::draft(0x14105abe0, "unknown_14105abe0", fn_14105abe0 as *const ()), // sub_14105abe0
    Replacement::draft(0x14105ac30, "unknown_14105ac30", tpl_1094::<0x143cd5ff8, 0x14105dd70, 0x14020e8f0> as *const ()), // sub_14105ac30
    Replacement::draft(0x14105ae30, "unknown_14105ae30", tpl_1095::<0x143cd5ccc, 0x14020e8f0> as *const ()), // sub_14105ae30
    Replacement::draft(0x14105b010, "unknown_14105b010", tpl_1094::<0x143cd5a78, 0x14105df10, 0x14020e8f0> as *const ()), // sub_14105b010
    Replacement::draft(0x14105b210, "unknown_14105b210", tpl_1094::<0x143cd5d90, 0x14105e0a0, 0x14020e8f0> as *const ()), // sub_14105b210
    Replacement::draft(0x14105b410, "unknown_14105b410", tpl_1466::<0x143cd6100, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14105b410
    Replacement::draft(0x14105b6e0, "unknown_14105b6e0", tpl_3074::<0x143cd5ee0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14105b6e0
    Replacement::draft(0x14105b9b0, "unknown_14105b9b0", tpl_1466::<0x143cd5f8c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14105b9b0
    Replacement::draft(0x14105bc80, "unknown_14105bc80", tpl_1094::<0x143cd5af8, 0x1410557d0, 0x14020e8f0> as *const ()), // sub_14105bc80
    Replacement::draft(0x14105be80, "unknown_14105be80", tpl_1094::<0x143cd6578, 0x14068b330, 0x14020e8f0> as *const ()), // sub_14105be80
    Replacement::draft(0x14105c080, "unknown_14105c080", tpl_1094::<0x143cb96dc, 0x14105e220, 0x14020e8f0> as *const ()), // sub_14105c080
    Replacement::draft(0x14105c280, "unknown_14105c280", tpl_3077::<0x143cb95b0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14105c280
    Replacement::draft(0x14105c5b0, "unknown_14105c5b0", tpl_1094::<0x143cd653c, 0x140fdd500, 0x14020e8f0> as *const ()), // sub_14105c5b0
    Replacement::draft(0x14105c7b0, "unknown_14105c7b0", fn_14105c7b0 as *const ()), // sub_14105c7b0
    Replacement::draft(0x14105c9b0, "unknown_14105c9b0", tpl_1094::<0x143cd600c, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_14105c9b0
    Replacement::draft_throws(0x14105cc10, "unknown_14105cc10", tpl_3011::<0x1411e9f50, 0x141063090, 0x143cd5c48, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14105f460, 0x1401de410> as *const (), Throws::Discard), // sub_14105cc10
    Replacement::draft_throws(0x14105cdd0, "coregame_14105cdd0", tpl_3683::<0x143cd5ba8, 0x143cd5dc0, 0x1411e9f50, 0x1410636e0, 0x143cd5a7c, 0x1420af120, 0x1420a28e0, 0x141062270, 0x14105f4f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143367236, 0x142646960, 0x143cd5de0, 0x143cd5de8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::streamOutResources>
    Replacement::draft_throws(0x14105d0b0, "coregame_14105d0b0", tpl_3086::<0x143cd5f20, 0x143cd5990, 0x1411e9f50, 0x1410635e0, 0x143cd5ba4, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x1410623d0, 0x14105f600, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143367446, 0x142646960, 0x143cd59b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_script::streamInResources>
    Replacement::draft(0x14105d3b0, "coregame_14105d3b0", tpl_19618::<0x143cd5cc4, 0x143cd5b70, 0x1411e9f50, 0x141063180, 0x143cd600c, 0x1401fc320, 0x1420a28e0, 0x141062550, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143367666, 0x142646960, 0x143cd5b90, 0x143cd5b98, 0x143cd5b9c, 0x143cd5b9e, 0x141834a0c> as *const ()), // ecs::system::ecs::system::wrapper<coregame::lua_script::removedDialogueResources>
    Replacement::draft_throws(0x14105d670, "unknown_14105d670", tpl_5552::<0x1411e9f50, 0x141062e00, 0x143cd5a6c, 0x1420af120, 0x143cd5d94, 0x140215610, 0x143cd6014, 0x1420a28e0, 0x14105f9d0, 0x14105e4b0> as *const (), Throws::Discard), // sub_14105d670
    Replacement::draft_throws(0x14105d830, "unknown_14105d830", tpl_5552::<0x1411e9f50, 0x141062d00, 0x143cd6004, 0x1420af120, 0x143cd5fe4, 0x140215610, 0x143cd6014, 0x1420a28e0, 0x14105fbd0, 0x14105e5d0> as *const (), Throws::Discard), // sub_14105d830
    Replacement::draft_throws(0x14105d9f0, "unknown_14105d9f0", tpl_5552::<0x1411e9f50, 0x1410629c0, 0x143cd6008, 0x1420af120, 0x143cd5d94, 0x140215610, 0x143cd6014, 0x1420a28e0, 0x14105fff0, 0x14105e8e0> as *const (), Throws::Discard), // sub_14105d9f0
    Replacement::draft_throws(0x14105dbb0, "unknown_14105dbb0", tpl_5552::<0x1411e9f50, 0x1410628b0, 0x143cd60fc, 0x1420af120, 0x143cd5fe4, 0x140215610, 0x143cd6014, 0x1420a28e0, 0x141060270, 0x14105ea40> as *const (), Throws::Discard), // sub_14105dbb0
    Replacement::draft(0x14105dd70, "unknown_14105dd70", fn_14105dd70 as *const ()), // sub_14105dd70
    Replacement::draft(0x14105df10, "unknown_14105df10", tpl_4469::<0x1401e6d50, 0x140469d40, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x140330e60> as *const ()), // sub_14105df10
    Replacement::draft(0x14105e0a0, "unknown_14105e0a0", tpl_3092::<0x1401e6d50, 0x14105f3e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14105eeb0> as *const ()), // sub_14105e0a0
    Replacement::draft(0x14105e220, "unknown_14105e220", fn_14105e220 as *const ()), // sub_14105e220
    Replacement::draft(0x14105e3c0, "unknown_14105e3c0", tpl_1968::<0x88, 0x10088, 0x142407c80, 0x14105f290, 0x580f0, 0x141ec7750> as *const ()), // sub_14105e3c0
    Replacement::draft(0x14105e4b0, "unknown_14105e4b0", tpl_63708::<0x88, 0x10088, 0x580e0, 0x143cd6014, 0x143cd5d94, 0x14105f190, 0x141ec67d0> as *const ()), // sub_14105e4b0
    Replacement::draft(0x14105e5d0, "unknown_14105e5d0", tpl_5561::<0x143cd5fe4, 0x143cd6014, 0x142407c80, 0x143cd5ff8, 0x142489ee0, 0x141ec6660> as *const ()), // sub_14105e5d0
    Replacement::draft(0x14105e710, "unknown_14105e710", tpl_1127::<0x142407c80, 0x143cd5ff8, 0x142489ee0, 0x141ec6cc0> as *const ()), // sub_14105e710
    Replacement::draft(0x14105e820, "unknown_14105e820", tpl_1126::<0x142407c80, 0x580f0, 0x141ec6d50> as *const ()), // sub_14105e820
    Replacement::draft(0x14105e8e0, "unknown_14105e8e0", tpl_62821::<0x143cd6014, 0x143cd5d94, 0x14105f060, 0x141ec6a90> as *const ()), // sub_14105e8e0
    Replacement::draft(0x14105ea40, "unknown_14105ea40", tpl_6205::<0x143cd5fe4, 0x143cd6014, 0x142407c80, 0x14105ef60, 0x141ec6880> as *const ()), // sub_14105ea40
    Replacement::draft(0x14105eb50, "unknown_14105eb50", tpl_20208::<0x143cd5658, 0x140215610, 0x143cd5a70, 0x143cd5af4, 0x143c8ceec, 0x143cd5810, 0x143cd56e4, 0x1420a28e0> as *const ()), // sub_14105eb50
    Replacement::draft(0x14105ed00, "unknown_14105ed00", tpl_20208::<0x143cd5658, 0x140215610, 0x143c8ceec, 0x143cd5a70, 0x143cd5d94, 0x143cd5fe4, 0x143cd56e4, 0x1420a28e0> as *const ()), // sub_14105ed00
    Replacement::draft(0x14105eeb0, "unknown_14105eeb0", fn_14105eeb0 as *const ()), // sub_14105eeb0
    Replacement::draft(0x14105f390, "unknown_14105f390", tpl_6490::<0x20, 0x24, 0x28, 0x14241d6f0> as *const ()), // sub_14105f390
    Replacement::draft(0x14105f3e0, "unknown_14105f3e0", tpl_5680::<0x7, 0x30, 0x20, 0x1401de830, 0x14037e270> as *const ()), // sub_14105f3e0
    Replacement::draft(0x14105f430, "unknown_14105f430", tpl_4996::<0x28, 0x14240ab10> as *const ()), // sub_14105f430
    Replacement::draft(0x14105f460, "unknown_14105f460", tpl_3028::<0x143cd5c48, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x141ec6230> as *const ()), // sub_14105f460
    Replacement::draft(0x14105f4f0, "unknown_14105f4f0", tpl_1127::<0x142407c80, 0x143cd600c, 0x142489ee0, 0x141ec58c0> as *const ()), // sub_14105f4f0
    Replacement::draft_throws(0x14105f600, "unknown_14105f600", fn_14105f600 as *const (), Throws::Caught), // sub_14105f600
    Replacement::draft_throws(0x14105f740, "unknown_14105f740", fn_14105f740 as *const (), Throws::Caught), // sub_14105f740
    Replacement::draft(0x14105f7e0, "unknown_14105f7e0", tpl_3213::<0x142407c80, 0x143cd6578, 0x142489ee0, 0x143cd5ff8, 0x141ec7750> as *const ()), // sub_14105f7e0
    Replacement::draft_throws(0x14105f990, "unknown_14105f990", fn_14105f990 as *const (), Throws::Caught), // sub_14105f990
    Replacement::draft(0x14105f9d0, "unknown_14105f9d0", tpl_63711::<0x143cd6014, 0x143cd5d94, 0x143cd5f8c, 0x142489ee0, 0x143cd5ff8, 0x141ec67d0> as *const ()), // sub_14105f9d0
    Replacement::draft(0x14105fbd0, "unknown_14105fbd0", tpl_5599::<0x143cd5fe4, 0x143cd6014, 0x142407c80, 0x143cd5ff8, 0x142489ee0, 0x141ec6660> as *const ()), // sub_14105fbd0
    Replacement::draft(0x14105fd80, "unknown_14105fd80", tpl_1131::<0x142407c80, 0x143cd5ff8, 0x142489ee0, 0x141ec6cc0> as *const ()), // sub_14105fd80
    Replacement::draft(0x14105fee0, "unknown_14105fee0", tpl_1130::<0x142407c80, 0x141ec6d50> as *const ()), // sub_14105fee0
    Replacement::draft(0x141060270, "unknown_141060270", tpl_6211::<0x143cd5fe4, 0x143cd6014, 0x142407c80, 0x143cd5ee0, 0x142489ee0, 0x143cd5ff8, 0x141ec6880> as *const ()), // sub_141060270
    Replacement::draft_throws(0x141060470, "unknown_141060470", fn_141060470 as *const (), Throws::Discard), // sub_141060470
    Replacement::draft_throws(0x1410604e0, "unknown_1410604e0", fn_1410604e0 as *const (), Throws::Discard), // sub_1410604e0
    Replacement::draft(0x141060790, "unknown_141060790", tpl_3188::<0x141062ff0, 0x14336a568, 0x141060b60, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141060790
    Replacement::draft(0x1410609e0, "unknown_1410609e0", fn_1410609e0 as *const ()), // sub_1410609e0
    Replacement::draft(0x141060b60, "unknown_141060b60", fn_141060b60 as *const ()), // sub_141060b60
    Replacement::draft(0x141061540, "unknown_141061540", fn_141061540 as *const ()), // sub_141061540
    Replacement::draft(0x141061570, "unknown_141061570", fn_141061570 as *const ()), // sub_141061570
    Replacement::draft(0x1410617c0, "unknown_1410617c0", tpl_5912::<0xfffffffffffffffe, 0x1410638b0, 0x142442de0> as *const ()), // sub_1410617c0
    Replacement::draft(0x141061810, "unknown_141061810", tpl_5284::<0x1402c0a70, 0x140244c00, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x140260f30, 0x142442de0> as *const ()), // sub_141061810
    Replacement::draft(0x141061950, "unknown_141061950", tpl_2885::<0x140225740, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_141061950
    Replacement::draft(0x141061a90, "unknown_141061a90", tpl_78436::<0x141063ca0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x141063220, 0x142442de0> as *const ()), // sub_141061a90
    Replacement::draft(0x141061b90, "unknown_141061b90", fn_141061b90 as *const ()), // sub_141061b90
    Replacement::draft_throws(0x141061d00, "unknown_141061d00", fn_141061d00 as *const (), Throws::Caught), // sub_141061d00
    Replacement::draft(0x1410620c0, "unknown_1410620c0", tpl_20187::<0x1425371a0, 0x142533200, 0x141064020, 0x140215e80, 0x1401de830> as *const ()), // sub_1410620c0
    Replacement::draft(0x141062270, "unknown_141062270", tpl_1131::<0x142407c80, 0x143cd600c, 0x142489ee0, 0x141ec58c0> as *const ()), // sub_141062270
    Replacement::draft_throws(0x1410623d0, "unknown_1410623d0", fn_1410623d0 as *const (), Throws::Caught), // sub_1410623d0
    Replacement::draft_throws(0x141062550, "unknown_141062550", fn_141062550 as *const (), Throws::Caught), // sub_141062550
    Replacement::draft_throws(0x1410625e0, "unknown_1410625e0", tpl_1365::<0x143cd65a8, 0x143cd5f38, 0x143cd5f28, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cd5ccc, 0x141834a0c> as *const (), Throws::Discard), // sub_1410625e0
    Replacement::draft_throws(0x141062710, "unknown_141062710", fn_141062710 as *const (), Throws::Discard), // sub_141062710
    Replacement::draft(0x141062ff0, "unknown_141062ff0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14336a568, 0x8, 0x20> as *const ()), // sub_141062ff0
    Replacement::draft(0x141063180, "unknown_141063180", tpl_3188::<0x141063540, 0x14336a5e0, 0x140f82730, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_141063180
    Replacement::draft(0x141063200, "unknown_141063200", tpl_19620::<0x8, 0x140f82730> as *const ()), // sub_141063200
    Replacement::draft(0x141063220, "unknown_141063220", tpl_2030::<0x141063320, 0xc, 0xffffffffffffffff, 0x108, 0x4, 0x100, 0xe, 0x20, 0x18, 0x1425371a0, 0x142533200> as *const ()), // sub_141063220
    Replacement::draft(0x141063320, "unknown_141063320", fn_141063320 as *const ()), // sub_141063320
    Replacement::draft(0x141063390, "unknown_141063390", fn_141063390 as *const ()), // sub_141063390
    Replacement::draft(0x141063540, "unknown_141063540", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x14336a5e0, 0x8, 0x20> as *const ()), // sub_141063540
    Replacement::draft(0x1410637f0, "unknown_1410637f0", fn_1410637f0 as *const ()), // sub_1410637f0
    Replacement::draft_throws(0x141063830, "unknown_141063830", fn_141063830 as *const (), Throws::Discard), // sub_141063830
    Replacement::draft(0x1410638b0, "unknown_1410638b0", fn_1410638b0 as *const ()), // sub_1410638b0
    Replacement::draft(0x141063b60, "unknown_141063b60", fn_141063b60 as *const ()), // sub_141063b60
    Replacement::draft(0x141063ca0, "unknown_141063ca0", fn_141063ca0 as *const ()), // sub_141063ca0
    Replacement::draft(0x141063e10, "unknown_141063e10", fn_141063e10 as *const ()), // sub_141063e10
    Replacement::draft(0x141063ef0, "unknown_141063ef0", fn_141063ef0 as *const ()), // sub_141063ef0
    Replacement::draft(0x141063f40, "unknown_141063f40", fn_141063f40 as *const ()), // sub_141063f40
    Replacement::draft(0x141064020, "unknown_141064020", fn_141064020 as *const ()), // sub_141064020
    Replacement::draft(0x141064150, "unknown_141064150", fn_141064150 as *const ()), // sub_141064150
    Replacement::draft(0x141064820, "unknown_141064820", fn_141064820 as *const ()), // sub_141064820
    Replacement::draft(0x141064860, "unknown_141064860", fn_141064860 as *const ()), // sub_141064860
    Replacement::draft(0x141064d40, "unknown_141064d40", fn_141064d40 as *const ()), // sub_141064d40
    Replacement::draft(0x141064e80, "unknown_141064e80", fn_141064e80 as *const ()), // sub_141064e80
    Replacement::draft(0x141064f80, "unknown_141064f80", fn_141064f80 as *const ()), // sub_141064f80
    Replacement::draft(0x141065020, "unknown_141065020", fn_141065020 as *const ()), // sub_141065020
    Replacement::draft(0x141065190, "unknown_141065190", fn_141065190 as *const ()), // sub_141065190
    Replacement::draft(0x1410652e0, "unknown_1410652e0", fn_1410652e0 as *const ()), // sub_1410652e0
    Replacement::draft(0x1410654f0, "unknown_1410654f0", fn_1410654f0 as *const ()), // sub_1410654f0
    Replacement::draft(0x141065580, "unknown_141065580", fn_141065580 as *const ()), // sub_141065580
    Replacement::draft(0x141065600, "unknown_141065600", tpl_20244::<0x8, 0x4, 0xc, 0x14, 0x10, 0x18, 0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30> as *const ()), // sub_141065600
    Replacement::draft_throws(0x141065680, "unknown_141065680", tpl_6277::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65b8, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14310a3b4, 0x141ef4820, 0x1410657e1, 0x141065815, 0x141065827, 0x14106583c, 0x1410657e1, 0x141065815, 0x141065827, 0x14106583c> as *const (), Throws::Discard), // sub_141065680
    Replacement::draft_throws(0x141065840, "unknown_141065840", tpl_20245::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65c0, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a758, 0x141ef4820, 0x1410659a1, 0x1410659d5, 0x1410659e7, 0x1410659fc, 0x1410659a1, 0x1410659d5, 0x1410659e7, 0x1410659fc> as *const (), Throws::Discard), // sub_141065840
    Replacement::draft_throws(0x141065a00, "unknown_141065a00", tpl_6276::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65c8, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14310a3b4, 0x141ef4820, 0x141065b61, 0x141065b95, 0x141065ba7, 0x141065bbc, 0x141065b61, 0x141065b95, 0x141065ba7, 0x141065bbc> as *const (), Throws::Discard), // sub_141065a00
    Replacement::draft_throws(0x141065bc0, "unknown_141065bc0", tpl_20246::<0x142119010, 0x142118e10, 0x1401fa730, 0x143ca9e48, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14310a3b4, 0x141ef4820, 0x141065d25, 0x141065d59, 0x141065d6b, 0x141065d80, 0x141065d25, 0x141065d59, 0x141065d6b, 0x141065d80> as *const (), Throws::Discard), // sub_141065bc0
    Replacement::draft_throws(0x141065d90, "unknown_141065d90", tpl_20246::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65cc, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14310a3b4, 0x141ef4820, 0x141065ef5, 0x141065f29, 0x141065f3b, 0x141065f50, 0x141065ef5, 0x141065f29, 0x141065f3b, 0x141065f50> as *const (), Throws::Discard), // sub_141065d90
    Replacement::draft_throws(0x141065f60, "unknown_141065f60", tpl_20246::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65d0, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a758, 0x141ef4820, 0x1410660c5, 0x1410660f9, 0x14106610b, 0x141066120, 0x1410660c5, 0x1410660f9, 0x14106610b, 0x141066120> as *const (), Throws::Discard), // sub_141065f60
    Replacement::draft_throws(0x141066130, "unknown_141066130", tpl_20246::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd501c, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a770, 0x141ef4820, 0x141066295, 0x1410662c9, 0x1410662db, 0x1410662f0, 0x141066295, 0x1410662c9, 0x1410662db, 0x1410662f0> as *const (), Throws::Discard), // sub_141066130
    Replacement::draft_throws(0x141066300, "unknown_141066300", tpl_6277::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65b0, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a768, 0x141ef4820, 0x141066461, 0x141066495, 0x1410664a7, 0x1410664bc, 0x141066461, 0x141066495, 0x1410664a7, 0x1410664bc> as *const (), Throws::Discard), // sub_141066300
    Replacement::draft_throws(0x1410664c0, "unknown_1410664c0", tpl_6277::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65b4, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1432bb974, 0x141ef4820, 0x141066621, 0x141066655, 0x141066667, 0x14106667c, 0x141066621, 0x141066655, 0x141066667, 0x14106667c> as *const (), Throws::Discard), // sub_1410664c0
    Replacement::draft_throws(0x141066680, "unknown_141066680", tpl_6277::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65d4, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x1432bb974, 0x141ef4820, 0x1410667e1, 0x141066815, 0x141066827, 0x14106683c, 0x1410667e1, 0x141066815, 0x141066827, 0x14106683c> as *const (), Throws::Discard), // sub_141066680
    Replacement::draft_throws(0x141066840, "unknown_141066840", tpl_20247::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65d8, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a790, 0x141ef4820, 0x1410669a1, 0x1410669d5, 0x1410669e7, 0x1410669fc, 0x1410669a1, 0x1410669d5, 0x1410669e7, 0x1410669fc> as *const (), Throws::Discard), // sub_141066840
    Replacement::draft_throws(0x141066a00, "unknown_141066a00", tpl_6274::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cc597c, 0x142489ee0, 0x141069ac0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a778, 0x141ef4820, 0x141066b16, 0x141066b4a, 0x141066b5c, 0x141066b71, 0x141066b16, 0x141066b4a, 0x141066b5c, 0x141066b71> as *const (), Throws::Discard), // sub_141066a00
    Replacement::draft_throws(0x141066b80, "unknown_141066b80", tpl_6274::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cc58cc, 0x142489ee0, 0x141069b50, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a6b8, 0x141ef4820, 0x141066c96, 0x141066cca, 0x141066cdc, 0x141066cf1, 0x141066c96, 0x141066cca, 0x141066cdc, 0x141066cf1> as *const (), Throws::Discard), // sub_141066b80
    Replacement::draft_throws(0x141066d00, "unknown_141066d00", tpl_20248::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c9f930, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a6a8, 0x141ef4820, 0x141066e65, 0x141066e99, 0x141066eab, 0x141066ec0, 0x141066e65, 0x141066e99, 0x141066eab, 0x141066ec0> as *const (), Throws::Discard), // sub_141066d00
    Replacement::draft_throws(0x141066ed0, "unknown_141066ed0", tpl_20247::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cc8b94, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a6e0, 0x141ef4820, 0x141067031, 0x141067065, 0x141067077, 0x14106708c, 0x141067031, 0x141067065, 0x141067077, 0x14106708c> as *const (), Throws::Discard), // sub_141066ed0
    Replacement::draft_throws(0x141067090, "unknown_141067090", fn_141067090 as *const (), Throws::Discard), // sub_141067090
    Replacement::draft_throws(0x1410671b0, "unknown_1410671b0", tpl_20245::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8e070, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a770, 0x141ef4820, 0x141067311, 0x141067345, 0x141067357, 0x14106736c, 0x141067311, 0x141067345, 0x141067357, 0x14106736c> as *const (), Throws::Discard), // sub_1410671b0
    Replacement::draft_throws(0x141067370, "unknown_141067370", tpl_20248::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8de70, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a770, 0x141ef4820, 0x1410674d5, 0x141067509, 0x14106751b, 0x141067530, 0x1410674d5, 0x141067509, 0x14106751b, 0x141067530> as *const (), Throws::Discard), // sub_141067370
    Replacement::draft_throws(0x141067540, "unknown_141067540", tpl_6274::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8dbc8, 0x142489ee0, 0x1402744c0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a770, 0x141ef4820, 0x141067656, 0x14106768a, 0x14106769c, 0x1410676b1, 0x141067656, 0x14106768a, 0x14106769c, 0x1410676b1> as *const (), Throws::Discard), // sub_141067540
    Replacement::draft_throws(0x1410676c0, "unknown_1410676c0", tpl_20250::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd65dc, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336a720, 0x141ef4820, 0x14106781d, 0x141067851, 0x141067863, 0x141067878, 0x14106781d, 0x141067851, 0x141067863, 0x141067878> as *const (), Throws::Discard), // sub_1410676c0
    Replacement::draft(0x141067880, "unknown_141067880", tpl_3872::<0x78, 0x8, 0x38, 0x30, 0x40, 0x1401fa730, 0x58, 0x141068b10> as *const ()), // sub_141067880
    Replacement::draft_throws(0x141067920, "unknown_141067920", fn_141067920 as *const (), Throws::Discard), // sub_141067920
    Replacement::draft_throws(0x141067ae0, "unknown_141067ae0", fn_141067ae0 as *const (), Throws::Discard), // sub_141067ae0
    Replacement::draft_throws(0x141067cb0, "unknown_141067cb0", tpl_6274::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8cf5c, 0x142489ee0, 0x1404c71e0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336ab40, 0x141ef4820, 0x141067dc6, 0x141067dfa, 0x141067e0c, 0x141067e21, 0x141067dc6, 0x141067dfa, 0x141067e0c, 0x141067e21> as *const (), Throws::Discard), // sub_141067cb0
    Replacement::draft(0x141067e30, "unknown_141067e30", fn_141067e30 as *const ()), // sub_141067e30
    Replacement::draft(0x141067fa0, "unknown_141067fa0", fn_141067fa0 as *const ()), // sub_141067fa0
    Replacement::draft(0x141068150, "unknown_141068150", fn_141068150 as *const ()), // sub_141068150
    Replacement::draft(0x1410682c0, "unknown_1410682c0", fn_1410682c0 as *const ()), // sub_1410682c0
    Replacement::draft(0x141068320, "unknown_141068320", fn_141068320 as *const ()), // sub_141068320
    Replacement::draft(0x141068560, "unknown_141068560", fn_141068560 as *const ()), // sub_141068560
    Replacement::draft(0x141068860, "unknown_141068860", fn_141068860 as *const ()), // sub_141068860
    Replacement::draft(0x141068920, "unknown_141068920", fn_141068920 as *const ()), // sub_141068920
    Replacement::draft(0x1410689e0, "unknown_1410689e0", tpl_3936::<0x30, 0x143c914d0, 0x180a8, 0x18090, 0x142489ee0, 0x140348b30, 0x1426467e8, 0x141068a46> as *const ()), // sub_1410689e0
    Replacement::draft(0x141068a50, "unknown_141068a50", tpl_63210::<0x30, 0x143c8e2e8, 0x180a8, 0x18090, 0x142489ee0, 0x180ac, 0x18098, 0x40, 0x1426467e8, 0x141068b0b> as *const ()), // sub_141068a50
    Replacement::draft(0x141068b10, "unknown_141068b10", tpl_3938::<0x30, 0x143cd5ff8, 0x180a8, 0x18090, 0x142489ee0, 0x141069820, 0x1426467e8, 0x141068b7b> as *const ()), // sub_141068b10
    Replacement::draft(0x141068b80, "unknown_141068b80", fn_141068b80 as *const ()), // sub_141068b80
    Replacement::draft(0x141068ce0, "unknown_141068ce0", fn_141068ce0 as *const ()), // sub_141068ce0
    Replacement::draft(0x141068d00, "unknown_141068d00", fn_141068d00 as *const ()), // sub_141068d00
    Replacement::draft(0x141068da0, "unknown_141068da0", fn_141068da0 as *const ()), // sub_141068da0
    Replacement::draft(0x141069060, "unknown_141069060", fn_141069060 as *const ()), // sub_141069060
    Replacement::draft(0x1410692a0, "unknown_1410692a0", fn_1410692a0 as *const ()), // sub_1410692a0
    Replacement::draft(0x141069300, "unknown_141069300", fn_141069300 as *const ()), // sub_141069300
    Replacement::draft(0x141069820, "unknown_141069820", tpl_3351::<0x30, 0x143cd5ff8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1410698a2> as *const ()), // sub_141069820
    Replacement::draft(0x1410698b0, "unknown_1410698b0", fn_1410698b0 as *const ()), // sub_1410698b0
    Replacement::draft(0x1410699d0, "unknown_1410699d0", fn_1410699d0 as *const ()), // sub_1410699d0
    Replacement::draft(0x141069ac0, "unknown_141069ac0", tpl_1360::<0x30, 0x143cc597c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x141069b43> as *const ()), // sub_141069ac0
    Replacement::draft(0x141069b50, "unknown_141069b50", tpl_1360::<0x30, 0x143cc58cc, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x141069bd3> as *const ()), // sub_141069b50
    Replacement::draft(0x141069be0, "unknown_141069be0", tpl_4008::<0x30, 0x143c900a8, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x141069c63> as *const ()), // sub_141069be0
    Replacement::draft(0x14106a9c0, "unknown_14106a9c0", fn_14106a9c0 as *const ()), // sub_14106a9c0
    Replacement::draft(0x14106aa20, "unknown_14106aa20", fn_14106aa20 as *const ()), // sub_14106aa20
    Replacement::draft_throws(0x14106aa70, "unknown_14106aa70", fn_14106aa70 as *const (), Throws::Discard), // sub_14106aa70
    Replacement::draft_throws(0x14106ab80, "unknown_14106ab80", fn_14106ab80 as *const (), Throws::Discard), // sub_14106ab80
    Replacement::draft_throws(0x14106ace0, "unknown_14106ace0", fn_14106ace0 as *const (), Throws::Discard), // sub_14106ace0
    Replacement::draft(0x14106ad20, "unknown_14106ad20", fn_14106ad20 as *const ()), // sub_14106ad20
    Replacement::draft(0x14106ae30, "unknown_14106ae30", fn_14106ae30 as *const ()), // sub_14106ae30
    Replacement::draft(0x14106afe0, "unknown_14106afe0", tpl_63210::<0x30, 0x143c8f628, 0x180a8, 0x18090, 0x142489ee0, 0x180ac, 0x18098, 0x40, 0x1426467e8, 0x14106b09b> as *const ()), // sub_14106afe0
    Replacement::draft_throws(0x14106b0a0, "unknown_14106b0a0", tpl_20250::<0x142119010, 0x142118e10, 0x1401fa730, 0x143ccf820, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x143366820, 0x141ef4820, 0x14106b1fd, 0x14106b231, 0x14106b243, 0x14106b258, 0x14106b1fd, 0x14106b231, 0x14106b243, 0x14106b258> as *const (), Throws::Discard), // sub_14106b0a0
    Replacement::draft_throws(0x14106b260, "unknown_14106b260", tpl_20245::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c954e0, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x143366820, 0x141ef4820, 0x14106b3c1, 0x14106b3f5, 0x14106b407, 0x14106b41c, 0x14106b3c1, 0x14106b3f5, 0x14106b407, 0x14106b41c> as *const (), Throws::Discard), // sub_14106b260
    Replacement::draft(0x14106b420, "unknown_14106b420", fn_14106b420 as *const ()), // sub_14106b420
    Replacement::draft_throws(0x14106b4c0, "anonymous_namespace_14106b4c0", fn_14106b4c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::processLuaRaycasts>
    Replacement::draft_throws(0x14106b810, "anonymous_namespace_14106b810", tpl_19267::<0x143cd6a8c, 0x143cd6620, 0x14106caf0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14336d8d6, 0x142646960, 0x143cd6640, 0x143cd6650, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::applyPhysicsResetsFromLua>
    Replacement::draft_throws(0x14106ba20, "anonymous_namespace_14106ba20", fn_14106ba20 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::assignSystemGroupsFromLua>
    Replacement::draft_throws(0x14106bd60, "anonymous_namespace_14106bd60", fn_14106bd60 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::processContactReports>
    Replacement::draft_throws(0x14106c0f0, "anonymous_namespace_14106c0f0", tpl_3202::<0x143cd69c4, 0x143cd6988, 0x1411e9f50, 0x14106e8f0, 0x143cd66e4, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x14106d9f0, 0x14106d380, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14336dfa6, 0x142646960, 0x143cd69a8, 0x143cd69b8, 0x143cd69bc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::updatePhysicsEventsStreamedOut>
    Replacement::draft_throws(0x14106c420, "anonymous_namespace_14106c420", tpl_4197::<0x143cd66e0, 0x143cd6a10, 0x1411e9f50, 0x14106e7e0, 0x143cd68fc, 0x1420af120, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x14106dbb0, 0x14106d480, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14336e396, 0x142646960, 0x143cd6a30, 0x143cd6a40, 0x143cd6a44, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<`anonymous-namespace'::updatePhysicsEventsStreamedIn>
    Replacement::draft_throws(0x14106c740, "unknown_14106c740", fn_14106c740 as *const (), Throws::Discard), // sub_14106c740
    Replacement::draft(0x14106c890, "unknown_14106c890", fn_14106c890 as *const ()), // sub_14106c890
    Replacement::draft(0x14106ca80, "unknown_14106ca80", tpl_3936::<0x30, 0x143c914bc, 0x180a8, 0x18090, 0x142489ee0, 0x1404a83c0, 0x1426467e8, 0x14106cae6> as *const ()), // sub_14106ca80
    Replacement::draft_throws(0x14106caf0, "unknown_14106caf0", fn_14106caf0 as *const (), Throws::Discard), // sub_14106caf0
    Replacement::draft(0x14106cce0, "unknown_14106cce0", fn_14106cce0 as *const ()), // sub_14106cce0
    Replacement::draft(0x14106cec0, "unknown_14106cec0", fn_14106cec0 as *const ()), // sub_14106cec0
    Replacement::draft(0x14106cf30, "unknown_14106cf30", fn_14106cf30 as *const ()), // sub_14106cf30
    Replacement::draft(0x14106d310, "unknown_14106d310", fn_14106d310 as *const ()), // sub_14106d310
    Replacement::draft(0x14106d380, "unknown_14106d380", tpl_64057::<0x88, 0x10088, 0x143c8ceec, 0x580e0, 0x14106d700, 0x141ee7fc0> as *const ()), // sub_14106d380
    Replacement::draft(0x14106d480, "unknown_14106d480", tpl_62759::<0x88, 0x10088, 0x143c8ceec, 0x580e0, 0x14106d5d0, 0x141ee8030> as *const ()), // sub_14106d480
    Replacement::draft(0x14106d810, "unknown_14106d810", fn_14106d810 as *const ()), // sub_14106d810
    Replacement::draft_throws(0x14106d940, "unknown_14106d940", fn_14106d940 as *const (), Throws::Discard), // sub_14106d940
    Replacement::draft(0x14106d9f0, "unknown_14106d9f0", tpl_64074::<0x143c8ceec, 0x143c8d160, 0x142489ee0, 0x143cd68a0, 0x141ee7fc0> as *const ()), // sub_14106d9f0
    Replacement::draft(0x14106dbb0, "unknown_14106dbb0", tpl_62765::<0x143c8ceec, 0x143c914bc, 0x142489ee0, 0x143c8d160, 0x143cd68a0, 0x141ee8030> as *const ()), // sub_14106dbb0
    Replacement::draft_throws(0x14106de00, "unknown_14106de00", tpl_20284::<0x14106df60, 0x14106e1d0> as *const (), Throws::Discard), // sub_14106de00
    Replacement::draft(0x14106df60, "unknown_14106df60", fn_14106df60 as *const ()), // sub_14106df60
    Replacement::draft_throws(0x14106e1d0, "unknown_14106e1d0", fn_14106e1d0 as *const (), Throws::Discard), // sub_14106e1d0
    Replacement::draft(0x14106e340, "unknown_14106e340", fn_14106e340 as *const ()), // sub_14106e340
    Replacement::draft(0x14106e4c0, "unknown_14106e4c0", tpl_78436::<0x14106eda0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x140260f30, 0x142442de0> as *const ()), // sub_14106e4c0
    Replacement::draft(0x14106e5c0, "unknown_14106e5c0", tpl_78436::<0x14043b8d0, 0x142537280, 0x1426456c8, 0x1426453a0, 0x140260f30, 0x142442de0> as *const ()), // sub_14106e5c0
    Replacement::draft(0x14106e6c0, "unknown_14106e6c0", tpl_4170::<0x14106ed10, 0x142537280, 0x1426456c8, 0x1426453a0, 0x1401f2bc0, 0x142442de0> as *const ()), // sub_14106e6c0
    Replacement::draft(0x14106eab0, "unknown_14106eab0", fn_14106eab0 as *const ()), // sub_14106eab0
    Replacement::draft(0x14106ed10, "unknown_14106ed10", fn_14106ed10 as *const ()), // sub_14106ed10
    Replacement::draft(0x14106eda0, "unknown_14106eda0", fn_14106eda0 as *const ()), // sub_14106eda0
    Replacement::draft(0x14106ee10, "unknown_14106ee10", fn_14106ee10 as *const ()), // sub_14106ee10
    Replacement::draft(0x14106ef80, "unknown_14106ef80", fn_14106ef80 as *const ()), // sub_14106ef80
    Replacement::draft_throws(0x14106f3b0, "coregame_14106f3b0", tpl_1176::<0x143cd6c3c, 0x143cd6bb8, 0x14106f700, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14336f1b6, 0x142646960, 0x143cd6bd8, 0x143cd6be8, 0x143cd6bea, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::lua_fact::detail::updateLuaFactListeners>
    Replacement::draft_throws(0x14106f700, "unknown_14106f700", fn_14106f700 as *const (), Throws::Discard), // sub_14106f700
    Replacement::draft_throws(0x14106f940, "unknown_14106f940", tpl_6367::<0x143cd6ba8, 0x20, 0x8, 0x141eec2e0> as *const (), Throws::Discard), // sub_14106f940
    Replacement::draft(0x14106f9a0, "unknown_14106f9a0", tpl_2885::<0x1402c0c50, 0x142537280, 0x1426456c8, 0x1426453a0, 0x140260f30, 0x142442de0> as *const ()), // sub_14106f9a0
    Replacement::draft_throws(0x14106fc60, "unknown_14106fc60", fn_14106fc60 as *const (), Throws::Caught), // sub_14106fc60
    Replacement::draft_throws(0x1410701d0, "unknown_1410701d0", fn_1410701d0 as *const (), Throws::Discard), // sub_1410701d0
    Replacement::draft(0x1410704e0, "unknown_1410704e0", fn_1410704e0 as *const ()), // sub_1410704e0
    Replacement::draft_throws(0x141070570, "unknown_141070570", fn_141070570 as *const (), Throws::Caught), // sub_141070570
    Replacement::draft(0x141070600, "unknown_141070600", fn_141070600 as *const ()), // sub_141070600
    Replacement::draft(0x141070640, "unknown_141070640", fn_141070640 as *const ()), // sub_141070640
    Replacement::draft(0x141070820, "unknown_141070820", fn_141070820 as *const ()), // sub_141070820
    Replacement::draft_throws(0x141070940, "unknown_141070940", fn_141070940 as *const (), Throws::Discard), // sub_141070940
    Replacement::draft(0x141070ad0, "unknown_141070ad0", fn_141070ad0 as *const ()), // sub_141070ad0
    Replacement::draft(0x141070c00, "unknown_141070c00", fn_141070c00 as *const ()), // sub_141070c00
    Replacement::draft(0x141070d50, "unknown_141070d50", fn_141070d50 as *const ()), // sub_141070d50
    Replacement::draft(0x141070ef0, "unknown_141070ef0", fn_141070ef0 as *const ()), // sub_141070ef0
    Replacement::draft(0x141071220, "unknown_141071220", fn_141071220 as *const ()), // sub_141071220
    Replacement::draft(0x141071440, "unknown_141071440", fn_141071440 as *const ()), // sub_141071440
    Replacement::draft(0x141071610, "unknown_141071610", fn_141071610 as *const ()), // sub_141071610
    Replacement::draft(0x1410718c0, "unknown_1410718c0", fn_1410718c0 as *const ()), // sub_1410718c0
    Replacement::draft(0x141071a10, "unknown_141071a10", fn_141071a10 as *const ()), // sub_141071a10
    Replacement::draft(0x141071c30, "unknown_141071c30", fn_141071c30 as *const ()), // sub_141071c30
    Replacement::draft(0x141072170, "unknown_141072170", fn_141072170 as *const ()), // sub_141072170
    Replacement::draft(0x1410721e0, "unknown_1410721e0", fn_1410721e0 as *const ()), // sub_1410721e0
    Replacement::draft(0x1410722e0, "unknown_1410722e0", fn_1410722e0 as *const ()), // sub_1410722e0
    Replacement::draft(0x141072480, "unknown_141072480", fn_141072480 as *const ()), // sub_141072480
    Replacement::draft(0x1410724f0, "unknown_1410724f0", fn_1410724f0 as *const ()), // sub_1410724f0
    Replacement::draft_throws(0x141072680, "unknown_141072680", fn_141072680 as *const (), Throws::Caught), // sub_141072680
    Replacement::draft(0x1410727b0, "unknown_1410727b0", fn_1410727b0 as *const ()), // sub_1410727b0
    Replacement::draft_throws(0x1410727d0, "unknown_1410727d0", fn_1410727d0 as *const (), Throws::Caught), // sub_1410727d0
    Replacement::draft(0x141072900, "unknown_141072900", fn_141072900 as *const ()), // sub_141072900
    Replacement::draft_throws(0x1410729e0, "unknown_1410729e0", tpl_6277::<0x142119010, 0x142118e10, 0x1401fa730, 0x143ccbf30, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336f910, 0x141ef4820, 0x141072b41, 0x141072b75, 0x141072b87, 0x141072b9c, 0x141072b41, 0x141072b75, 0x141072b87, 0x141072b9c> as *const (), Throws::Discard), // sub_1410729e0
    Replacement::draft_throws(0x141072ba0, "unknown_141072ba0", tpl_20247::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8dbd4, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14310c350, 0x141ef4820, 0x141072d01, 0x141072d35, 0x141072d47, 0x141072d5c, 0x141072d01, 0x141072d35, 0x141072d47, 0x141072d5c> as *const (), Throws::Discard), // sub_141072ba0
    Replacement::draft_throws(0x141072d60, "unknown_141072d60", tpl_20312::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cc7394, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336f908, 0x141ef4820, 0x141072ec5, 0x141072ef9, 0x141072f0b, 0x141072f20, 0x141072ec5, 0x141072ef9, 0x141072f0b, 0x141072f20> as *const (), Throws::Discard), // sub_141072d60
    Replacement::draft_throws(0x141072f30, "unknown_141072f30", tpl_20312::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cc6d44, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336f8f0, 0x141ef4820, 0x141073095, 0x1410730c9, 0x1410730db, 0x1410730f0, 0x141073095, 0x1410730c9, 0x1410730db, 0x1410730f0> as *const (), Throws::Discard), // sub_141072f30
    Replacement::draft_throws(0x141073100, "unknown_141073100", tpl_6274::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd6cc8, 0x142489ee0, 0x141075010, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336f8e0, 0x141ef4820, 0x141073216, 0x14107324a, 0x14107325c, 0x141073271, 0x141073216, 0x14107324a, 0x14107325c, 0x141073271> as *const (), Throws::Discard), // sub_141073100
    Replacement::draft_throws(0x141073280, "unknown_141073280", tpl_6274::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8d0a0, 0x142489ee0, 0x140f86250, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336f7e0, 0x141ef4820, 0x141073396, 0x1410733ca, 0x1410733dc, 0x1410733f1, 0x141073396, 0x1410733ca, 0x1410733dc, 0x1410733f1> as *const (), Throws::Discard), // sub_141073280
    Replacement::draft_throws(0x141073400, "unknown_141073400", tpl_6274::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cc893c, 0x142489ee0, 0x140f87cd0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336f7c8, 0x141ef4820, 0x141073516, 0x14107354a, 0x14107355c, 0x141073571, 0x141073516, 0x14107354a, 0x14107355c, 0x141073571> as *const (), Throws::Discard), // sub_141073400
    Replacement::draft_throws(0x141073580, "unknown_141073580", tpl_20312::<0x142119010, 0x142118e10, 0x1401fa730, 0x143c8d024, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336f7e0, 0x141ef4820, 0x1410736e5, 0x141073719, 0x14107372b, 0x141073740, 0x1410736e5, 0x141073719, 0x14107372b, 0x141073740> as *const (), Throws::Discard), // sub_141073580
    Replacement::draft_throws(0x141073750, "unknown_141073750", tpl_20247::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd6c54, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x14336f7b0, 0x141ef4820, 0x1410738b1, 0x1410738e5, 0x1410738f7, 0x14107390c, 0x1410738b1, 0x1410738e5, 0x1410738f7, 0x14107390c> as *const (), Throws::Discard), // sub_141073750
    Replacement::draft_throws(0x141073910, "unknown_141073910", fn_141073910 as *const (), Throws::Discard), // sub_141073910
    Replacement::draft(0x141073a90, "unknown_141073a90", tpl_3045::<0x30, 0x143cd6d0c, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x141073b12> as *const ()), // sub_141073a90
    Replacement::draft_throws(0x141073b20, "unknown_141073b20", fn_141073b20 as *const (), Throws::Discard), // sub_141073b20
    Replacement::draft_throws(0x141073c60, "unknown_141073c60", fn_141073c60 as *const (), Throws::Caught), // sub_141073c60
    Replacement::draft(0x141073d90, "unknown_141073d90", fn_141073d90 as *const ()), // sub_141073d90
    Replacement::draft(0x141073db0, "unknown_141073db0", tpl_3163::<0x38, 0x8, 0x141074b00> as *const ()), // sub_141073db0
    Replacement::draft(0x141073df0, "unknown_141073df0", fn_141073df0 as *const ()), // sub_141073df0
    Replacement::draft(0x141073f40, "unknown_141073f40", fn_141073f40 as *const ()), // sub_141073f40
    Replacement::draft(0x141073f60, "unknown_141073f60", fn_141073f60 as *const ()), // sub_141073f60
    Replacement::draft_throws(0x141073f80, "unknown_141073f80", tpl_3880::<0x78, 0x143c8d0c0, 0x580e0, 0x143110698, 0x142116b30, 0x141073fb6> as *const (), Throws::Discard), // sub_141073f80
    Replacement::draft_throws(0x141073fc0, "luaspawning_141073fc0", fn_141073fc0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<luaspawning::listen_for_spawn_events::system>
    Replacement::draft_throws(0x141074330, "unknown_141074330", fn_141074330 as *const (), Throws::Discard), // sub_141074330
    Replacement::draft_throws(0x141074490, "unknown_141074490", fn_141074490 as *const (), Throws::Discard), // sub_141074490
    Replacement::draft_throws(0x141074520, "unknown_141074520", fn_141074520 as *const (), Throws::Discard), // sub_141074520
    Replacement::draft(0x1410745c0, "unknown_1410745c0", tpl_1466::<0x143cd68a0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410745c0
    Replacement::draft(0x141074890, "unknown_141074890", fn_141074890 as *const ()), // sub_141074890
    Replacement::draft(0x141074b00, "unknown_141074b00", tpl_3077::<0x143cd6c5c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141074b00
    Replacement::draft(0x141074e30, "unknown_141074e30", tpl_64022::<0x88, 0x10088, 0x141074f10, 0x141f0a610> as *const ()), // sub_141074e30
    Replacement::draft(0x141075010, "unknown_141075010", fn_141075010 as *const ()), // sub_141075010
    Replacement::draft(0x1410750a0, "unknown_1410750a0", tpl_64023::<0x143c8d160, 0x142489ee0, 0x143cd6c5c, 0x141f0a610> as *const ()), // sub_1410750a0
    Replacement::draft(0x141075ad0, "unknown_141075ad0", fn_141075ad0 as *const ()), // sub_141075ad0
    Replacement::draft(0x141075c10, "unknown_141075c10", fn_141075c10 as *const ()), // sub_141075c10
    Replacement::draft(0x141075c80, "unknown_141075c80", tpl_4781::<0x8, 0x10, 0x14024d8b0, 0x18, 0x30, 0x38, 0x1401e5730> as *const ()), // sub_141075c80
    Replacement::draft(0x141075cf0, "unknown_141075cf0", tpl_5914::<0x10, 0x38, 0x8, 0x141075dc0, 0x18, 0x1401de830, 0x1401e66f0, 0x20, 0x1401e6780> as *const ()), // sub_141075cf0
    Replacement::draft(0x141075dc0, "std_141075dc0", fn_141075dc0 as *const ()), // sub_141075dc0
    Replacement::draft_throws(0x141075fc0, "unknown_141075fc0", tpl_6276::<0x142119010, 0x142118e10, 0x1401fa730, 0x143cd6e28, 0x142489ee0, 0x1426467e8, 0x1406574b0, 0x1406573b0, 0x142116bc0, 0x1421158a0, 0x143372ec0, 0x141ef4820, 0x141076121, 0x141076155, 0x141076167, 0x14107617c, 0x141076121, 0x141076155, 0x141076167, 0x14107617c> as *const (), Throws::Discard), // sub_141075fc0
    Replacement::draft(0x141076180, "unknown_141076180", fn_141076180 as *const ()), // sub_141076180
    Replacement::draft_throws(0x1410763d0, "unknown_1410763d0", tpl_847::<0x143cd6e24, 0x143cd6e30, 0x143cd6e38, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143ccc0e8, 0x141834a0c> as *const (), Throws::Discard), // sub_1410763d0
    Replacement::draft(0x141077df0, "unknown_141077df0", fn_141077df0 as *const ()), // sub_141077df0
    Replacement::draft(0x141077ec0, "unknown_141077ec0", fn_141077ec0 as *const ()), // sub_141077ec0
    Replacement::draft(0x141078050, "unknown_141078050", fn_141078050 as *const ()), // sub_141078050
    Replacement::draft(0x1410780f0, "unknown_1410780f0", fn_1410780f0 as *const ()), // sub_1410780f0
    Replacement::draft(0x141078150, "unknown_141078150", tpl_3045::<0x30, 0x143c8ec20, 0x180ac, 0x18098, 0x142489ee0, 0x40, 0x1426467e8, 0x1410781d2> as *const ()), // sub_141078150
    Replacement::draft(0x1410781e0, "unknown_1410781e0", fn_1410781e0 as *const ()), // sub_1410781e0
    Replacement::draft(0x141078290, "unknown_141078290", tpl_20343::<0x4, 0x14183acbc, 0x8, 0xc> as *const ()), // sub_141078290
    Replacement::draft(0x141078300, "unknown_141078300", tpl_1171::<0x58, 0x60, 0x143c8de68, 0x158, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141078300
    Replacement::draft(0x1410783e0, "unknown_1410783e0", fn_1410783e0 as *const ()), // sub_1410783e0
    Replacement::draft(0x141078560, "unknown_141078560", tpl_3942::<0x30, 0x143c8dd14, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x1410785b0> as *const ()), // sub_141078560
    Replacement::draft(0x1410785c0, "unknown_1410785c0", tpl_63595::<0x141078700, 0x1426467e8, 0x1410786fe> as *const ()), // sub_1410785c0
    Replacement::draft(0x141078930, "unknown_141078930", fn_141078930 as *const ()), // sub_141078930
    Replacement::draft(0x1410789a0, "unknown_1410789a0", fn_1410789a0 as *const ()), // sub_1410789a0
    Replacement::draft(0x141078a00, "unknown_141078a00", fn_141078a00 as *const ()), // sub_141078a00
    Replacement::draft(0x141078b50, "unknown_141078b50", tpl_60953::<0x1426467e8, 0x143cbe8e4, 0x142489ee0, 0x141078bcc> as *const ()), // sub_141078b50
    Replacement::draft_throws(0x141078d00, "coregame_141078d00", tpl_3062::<0x143cd724c, 0x143cd7668, 0x1411e9f50, 0x14107c2a0, 0x14107d730, 0x14107c490, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433755d6, 0x142646960, 0x143cd7688, 0x143cd7690, 0x143cd7694, 0x143cd7696, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::logicanimation::sync_slaves::execute>
    Replacement::draft_throws(0x141078f60, "coregame_141078f60", tpl_6402::<0x143cd73e8, 0x143cd73b0, 0x14107ab70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143374f46, 0x142646960, 0x143cd73d0, 0x143cd73e0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::logicanimation::sync_attachments::execute>
    Replacement::draft_throws(0x141079170, "coregame_141079170", tpl_3464::<0x143cd72d0, 0x143cd6ed8, 0x1411e9f50, 0x14107c7f0, 0x14107dbc0, 0x14107c9f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143374186, 0x142646960, 0x143cd6ef8, 0x143cd6f08, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::logicanimation::start_processing::execute>
    Replacement::draft_throws(0x1410793c0, "coregame_1410793c0", tpl_3004::<0x143cd7180, 0x143cd6e78, 0x14107ada0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143374016, 0x142646960, 0x143cd6e98, 0x143cd6e9c, 0x143cd6e9e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::logicanimation::stateRemoved>
    Replacement::draft(0x1410795d0, "coregame_1410795d0", fn_1410795d0 as *const ()), // sub_1410795d0
    Replacement::draft(0x1410796b0, "coregame_1410796b0", tpl_60953::<0x1426467e8, 0x143cbe480, 0x142489ee0, 0x14107972c> as *const ()), // sub_1410796b0
    Replacement::draft(0x141079860, "coregame_141079860", tpl_1094::<0x143cd71f8, 0x14068b330, 0x14020e8f0> as *const ()), // sub_141079860
    Replacement::draft(0x141079a60, "coregame_141079a60", tpl_1094::<0x143cd6f20, 0x14107bc80, 0x14020e8f0> as *const ()), // sub_141079a60
    Replacement::draft(0x141079c60, "coregame_141079c60", tpl_1094::<0x143cd6f24, 0x14107be00, 0x14020e8f0> as *const ()), // sub_141079c60
    Replacement::draft(0x141079e60, "coregame_141079e60", tpl_3080::<0x143c8de68, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141079e60
    Replacement::draft(0x14107a1a0, "coregame_14107a1a0", tpl_1363::<0x30, 0x143c9b928, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x14107a1ed> as *const ()), // sub_14107a1a0
    Replacement::draft(0x14107a1f0, "coregame_14107a1f0", tpl_1363::<0x30, 0x143ca7420, 0x180a8, 0x18090, 0x142489ee0, 0x1426467e8, 0x14107a23d> as *const ()), // sub_14107a1f0
    Replacement::draft(0x14107a240, "coregame_14107a240", tpl_1094::<0x143cd7348, 0x14107bf80, 0x14020e8f0> as *const ()), // sub_14107a240
    Replacement::draft(0x14107a440, "coregame_14107a440", tpl_1094::<0x143cbe480, 0x14107c120, 0x14020e8f0> as *const ()), // sub_14107a440
    Replacement::draft(0x14107a640, "coregame_14107a640", tpl_1097::<0x143cbeb80, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14107a640
    Replacement::draft(0x14107a970, "coregame_14107a970", tpl_1094::<0x143cca1f8, 0x1404493b0, 0x14020e8f0> as *const ()), // sub_14107a970
    Replacement::draft_throws(0x14107ab70, "coregame_14107ab70", tpl_3083::<0x1411e9f50, 0x14107fa70, 0x143cd7788, 0x1420af120, 0x14107f8a0, 0x143cd73f0, 0x140215750, 0x143c8ceec, 0x140215610, 0x1420a28e0, 0x14107d970, 0x14107c5f0> as *const (), Throws::Discard), // sub_14107ab70
    Replacement::draft_throws(0x14107ada0, "coregame_14107ada0", tpl_3370::<0x1411e9f50, 0x14107e180, 0x143cd6f20, 0x1401fc320, 0x14107f340, 0x143cd6fbc, 0x1420af120, 0x140215750, 0x1420a28e0, 0x14107deb0, 0x1401de410> as *const (), Throws::Discard), // sub_14107ada0
    Replacement::draft_throws(0x14107af70, "coregame_14107af70", tpl_1075::<0x143cd6e74, 0x143cd6f98, 0x1411e9f50, 0x141080150, 0x143cd6f38, 0x1420af120, 0x143cada18, 0x140215610, 0x1420a28e0, 0x14107ec60, 0x14107dfb0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143373d96, 0x142646960, 0x143cd6fb8, 0x143cd6fba, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::logicanimation::streamIn>
    Replacement::draft_throws(0x14107b280, "coregame_14107b280", tpl_4485::<0x143cd72c8, 0x143cd76d8, 0x1411e9f50, 0x141080060, 0x143cd6f2c, 0x1420af120, 0x1420a28e0, 0x14107ee30, 0x14107e0c0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143374da6, 0x142646960, 0x143cd76f8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::logicanimation::streamOut>
    Replacement::draft(0x14107b550, "coregame_14107b550", tpl_20349::<0x1401e7c10, 0x14107b9c0, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_14107b550
    Replacement::draft_throws(0x14107b760, "coregame_14107b760", tpl_20053::<0x143cd6e70, 0x143cd7350, 0x14107cc90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143374bb6, 0x142646960, 0x143cd7370, 0x143cd7390, 0x143cd73a0, 0x143cd73a8, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::LogicAnimationCompo
    Replacement::draft(0x14107bac0, "unknown_14107bac0", tpl_60530::<0x1426467e8, 0x143cd71f8, 0x142489ee0, 0x143cbe480, 0x14107bae9> as *const ()), // sub_14107bac0
    Replacement::draft(0x14107bc80, "unknown_14107bc80", tpl_3090::<0x1401e6d50, 0x14107d630, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14107ce50> as *const ()), // sub_14107bc80
    Replacement::draft(0x14107be00, "unknown_14107be00", fn_14107be00 as *const ()), // sub_14107be00
    Replacement::draft(0x14107bf80, "unknown_14107bf80", fn_14107bf80 as *const ()), // sub_14107bf80
    Replacement::draft(0x14107c120, "unknown_14107c120", fn_14107c120 as *const ()), // sub_14107c120
    Replacement::draft_throws(0x14107c2a0, "unknown_14107c2a0", tpl_4853::<0x14107fdd0, 0x143cd7188, 0x1420af120, 0x14107fce0, 0x143cd6ff8, 0x140215750, 0x14107fbf0, 0x143cd6f34, 0x1420a28e0> as *const (), Throws::Discard), // sub_14107c2a0
    Replacement::draft(0x14107c490, "unknown_14107c490", tpl_64043::<0x143cd6ff8, 0x143cd6f34, 0x14107d530, 0x141f1f480> as *const ()), // sub_14107c490
    Replacement::draft(0x14107c5f0, "unknown_14107c5f0", fn_14107c5f0 as *const ()), // sub_14107c5f0
    Replacement::draft(0x14107c6f0, "unknown_14107c6f0", tpl_4943::<0x18, 0x20, 0x40, 0x48, 0x8, 0x141f1eb30> as *const ()), // sub_14107c6f0
    Replacement::draft_throws(0x14107c7f0, "unknown_14107c7f0", tpl_3039::<0x14107f6e0, 0x143cd6f1c, 0x1420af120, 0x14107f490, 0x143cd72bc, 0x140215750, 0x14036fad0, 0x143c8ceec, 0x140215610, 0x143cd72d4, 0x1420a28e0> as *const (), Throws::Discard), // sub_14107c7f0
    Replacement::draft_throws(0x14107c9f0, "unknown_14107c9f0", fn_14107c9f0 as *const (), Throws::Discard), // sub_14107c9f0
    Replacement::draft_throws(0x14107cb40, "unknown_14107cb40", fn_14107cb40 as *const (), Throws::Discard), // sub_14107cb40
    Replacement::draft_throws(0x14107cc90, "unknown_14107cc90", tpl_3011::<0x1411e9f50, 0x14107ff70, 0x143cd778c, 0x1420af120, 0x140215750, 0x143c9a718, 0x140215610, 0x1420a28e0, 0x14107ef40, 0x1401de410> as *const (), Throws::Discard), // sub_14107cc90
    Replacement::draft(0x14107ce50, "unknown_14107ce50", fn_14107ce50 as *const ()), // sub_14107ce50
    Replacement::draft(0x14107cf50, "unknown_14107cf50", fn_14107cf50 as *const ()), // sub_14107cf50
    Replacement::draft(0x14107d630, "unknown_14107d630", fn_14107d630 as *const ()), // sub_14107d630
    Replacement::draft(0x14107d690, "unknown_14107d690", fn_14107d690 as *const ()), // sub_14107d690
    Replacement::draft(0x14107d6f0, "unknown_14107d6f0", tpl_5213::<0x38, 0x1403b8050> as *const ()), // sub_14107d6f0
    Replacement::draft(0x14107d730, "unknown_14107d730", tpl_64047::<0x143cd6ff8, 0x143cd6f34, 0x143cbe8e4, 0x142489ee0, 0x143c8dd14, 0x141f1f480> as *const ()), // sub_14107d730
    Replacement::draft(0x14107d970, "unknown_14107d970", fn_14107d970 as *const ()), // sub_14107d970
    Replacement::draft_throws(0x14107dbc0, "unknown_14107dbc0", fn_14107dbc0 as *const (), Throws::Discard), // sub_14107dbc0
    Replacement::draft(0x14107deb0, "unknown_14107deb0", tpl_19300::<0x143cd6f20, 0x5e308, 0x143cd6fbc, 0x20, 0x8, 0x10, 0x18, 0x141f1f890, 0x14107e720> as *const ()), // sub_14107deb0
    Replacement::draft_throws(0x14107dfb0, "unknown_14107dfb0", fn_14107dfb0 as *const (), Throws::Caught), // sub_14107dfb0
    Replacement::draft(0x14107e0c0, "unknown_14107e0c0", tpl_1126::<0x142407c80, 0x580f0, 0x141f1fc10> as *const ()), // sub_14107e0c0
    Replacement::draft(0x14107e180, "unknown_14107e180", tpl_3188::<0x14107f3f0, 0x143377828, 0x14107e720, 0x8, 0x1401de830, 0x142442de0> as *const ()), // sub_14107e180
    Replacement::draft(0x14107e200, "unknown_14107e200", tpl_4348::<0x18, 0x20, 0x14107e720> as *const ()), // sub_14107e200
    Replacement::draft(0x14107e5b0, "unknown_14107e5b0", fn_14107e5b0 as *const ()), // sub_14107e5b0
    Replacement::draft(0x14107e720, "unknown_14107e720", fn_14107e720 as *const ()), // sub_14107e720
    Replacement::draft_throws(0x14107ec60, "unknown_14107ec60", fn_14107ec60 as *const (), Throws::Caught), // sub_14107ec60
    Replacement::draft(0x14107ee30, "unknown_14107ee30", tpl_1130::<0x142407c80, 0x141f1fc10> as *const ()), // sub_14107ee30
    Replacement::draft(0x14107ef40, "unknown_14107ef40", tpl_3028::<0x143cd778c, 0x20, 0x8, 0x143c9a718, 0x580e0, 0x14107b550> as *const ()), // sub_14107ef40
    Replacement::draft_throws(0x14107efb0, "unknown_14107efb0", tpl_2918::<0x143cd6f28, 0x143cd71fc, 0x143cd7238, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cbeb80, 0x141834a0c> as *const (), Throws::Discard), // sub_14107efb0
    Replacement::draft_throws(0x14107f0e0, "unknown_14107f0e0", tpl_2918::<0x143cd7108, 0x143cd762c, 0x143cd72a8, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cbe8e4, 0x141834a0c> as *const (), Throws::Discard), // sub_14107f0e0
    Replacement::draft_throws(0x14107f210, "unknown_14107f210", tpl_2918::<0x143cd773c, 0x143cd6f94, 0x143cd7138, 0x1420aeb60, 0x8, 0x1401ed0c0, 0x14241c190, 0x141834a78, 0x143cbe480, 0x141834a0c> as *const (), Throws::Discard), // sub_14107f210
    Replacement::draft(0x14107f3f0, "unknown_14107f3f0", tpl_3194::<0x30, 0x10, 0x18, 0xffffffffffffffe8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x143377828, 0x8, 0x20> as *const ()), // sub_14107f3f0
    Replacement::draft(0x14107f490, "unknown_14107f490", fn_14107f490 as *const ()), // sub_14107f490
    Replacement::draft(0x14107f6e0, "unknown_14107f6e0", fn_14107f6e0 as *const ()), // sub_14107f6e0
    Replacement::draft(0x14107f8a0, "unknown_14107f8a0", fn_14107f8a0 as *const ()), // sub_14107f8a0
    Replacement::draft(0x14107fec0, "unknown_14107fec0", fn_14107fec0 as *const ()), // sub_14107fec0
    Replacement::draft_throws(0x1410808c0, "lod_1410808c0", fn_1410808c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<lod::update>
    Replacement::draft(0x141080c90, "lod_141080c90", tpl_5819::<0x143cd7878, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141080c90
    Replacement::draft(0x141080fe0, "lod_141080fe0", tpl_1436::<0x58, 0x60, 0x158, 0x143cd7878, 0x8, 0x14020e8f0, 0x10, 0x170> as *const ()), // sub_141080fe0
    Replacement::draft(0x1410810c0, "lod_1410810c0", tpl_3077::<0x143cc52b4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410810c0
    Replacement::draft_throws(0x1410813f0, "lod_1410813f0", fn_1410813f0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<lod::streamIn>
    Replacement::draft_throws(0x1410816c0, "lod_1410816c0", fn_1410816c0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<lod::streamOut>
    Replacement::draft(0x1410819a0, "unknown_1410819a0", tpl_3280::<0x8, 0x1410810c0> as *const ()), // sub_1410819a0
    Replacement::draft(0x1410819e0, "unknown_1410819e0", fn_1410819e0 as *const ()), // sub_1410819e0
    Replacement::draft(0x141081cf0, "unknown_141081cf0", fn_141081cf0 as *const ()), // sub_141081cf0
    Replacement::draft(0x141081e70, "unknown_141081e70", tpl_1126::<0x142407c80, 0x580f0, 0x141f22a00> as *const ()), // sub_141081e70
    Replacement::draft(0x141081f30, "unknown_141081f30", tpl_1450::<0x88, 0x10088, 0x142407c80, 0x10, 0x141080fe0> as *const ()), // sub_141081f30
    Replacement::draft(0x1410820c0, "unknown_1410820c0", tpl_1130::<0x142407c80, 0x141f22a00> as *const ()), // sub_1410820c0
    Replacement::draft(0x1410821d0, "unknown_1410821d0", tpl_1452::<0x8, 0x142407c80, 0x88, 0x10088, 0x580f0, 0x141080fe0> as *const ()), // sub_1410821d0
    Replacement::draft(0x1410822b0, "unknown_1410822b0", fn_1410822b0 as *const ()), // sub_1410822b0
    Replacement::draft_throws(0x141082800, "coregame_141082800", fn_141082800 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::local_time_scale::update::fixedSystem>
    Replacement::draft_throws(0x141082aa0, "coregame_141082aa0", fn_141082aa0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::local_time_scale::update::variableSystem>
    Replacement::draft(0x141082d60, "unknown_141082d60", tpl_1094::<0x143cd7aa4, 0x141083610, 0x14020e8f0> as *const ()), // sub_141082d60
    Replacement::draft(0x141082f60, "unknown_141082f60", tpl_1099::<0x143cd7aac, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_141082f60
    Replacement::draft(0x1410832a0, "unknown_1410832a0", tpl_1099::<0x143cd7ab0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_1410832a0
    Replacement::draft(0x141083610, "unknown_141083610", tpl_3212::<0x1401e6d50, 0x140272840, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143288528> as *const ()), // sub_141083610
    Replacement::draft(0x141083790, "unknown_141083790", tpl_64057::<0x88, 0x10088, 0x143c8d910, 0x580e0, 0x141083990, 0x141f23420> as *const ()), // sub_141083790
    Replacement::draft(0x141083890, "unknown_141083890", tpl_64057::<0x88, 0x10088, 0x143c904a0, 0x580e0, 0x141083990, 0x141f233b0> as *const ()), // sub_141083890
    Replacement::draft_throws(0x141083a90, "unknown_141083a90", tpl_1331::<0x8, 0x1410841a0, 0x143cd7b98, 0x20, 0x1420af120, 0x10, 0xa8, 0xc8, 0xe8> as *const (), Throws::Discard), // sub_141083a90
    Replacement::draft(0x141083b40, "unknown_141083b40", tpl_64074::<0x143c8d910, 0x143cd7aac, 0x142489ee0, 0x143cd7ab0, 0x141f23420> as *const ()), // sub_141083b40
    Replacement::draft(0x141083d00, "unknown_141083d00", tpl_64074::<0x143c904a0, 0x143cd7aac, 0x142489ee0, 0x143cd7ab0, 0x141f233b0> as *const ()), // sub_141083d00
    Replacement::draft_throws(0x141084320, "unknown_141084320", fn_141084320 as *const (), Throws::Discard), // sub_141084320
    Replacement::draft_throws(0x1410843c0, "unknown_1410843c0", tpl_1148::<0x8, 0xc, 0x14241c190> as *const (), Throws::Discard), // sub_1410843c0
    Replacement::draft(0x141084f10, "unknown_141084f10", fn_141084f10 as *const ()), // sub_141084f10
    Replacement::draft(0x141085020, "unknown_141085020", fn_141085020 as *const ()), // sub_141085020
    Replacement::draft(0x141085190, "unknown_141085190", fn_141085190 as *const ()), // sub_141085190
    Replacement::draft(0x141085300, "unknown_141085300", fn_141085300 as *const ()), // sub_141085300
    Replacement::draft(0x141085470, "unknown_141085470", fn_141085470 as *const ()), // sub_141085470
    Replacement::draft(0x1410855e0, "unknown_1410855e0", fn_1410855e0 as *const ()), // sub_1410855e0
    Replacement::draft(0x141085770, "unknown_141085770", fn_141085770 as *const ()), // sub_141085770
    Replacement::draft(0x1410858e0, "unknown_1410858e0", fn_1410858e0 as *const ()), // sub_1410858e0
    Replacement::draft(0x141085a50, "unknown_141085a50", fn_141085a50 as *const ()), // sub_141085a50
    Replacement::draft(0x141085bc0, "unknown_141085bc0", fn_141085bc0 as *const ()), // sub_141085bc0
    Replacement::draft(0x141085d30, "unknown_141085d30", fn_141085d30 as *const ()), // sub_141085d30
    Replacement::draft(0x141085ea0, "unknown_141085ea0", fn_141085ea0 as *const ()), // sub_141085ea0
    Replacement::draft(0x141086010, "unknown_141086010", fn_141086010 as *const ()), // sub_141086010
    Replacement::draft(0x141086180, "unknown_141086180", fn_141086180 as *const ()), // sub_141086180
    Replacement::draft(0x1410862f0, "unknown_1410862f0", fn_1410862f0 as *const ()), // sub_1410862f0
    Replacement::draft(0x1410864a0, "unknown_1410864a0", fn_1410864a0 as *const ()), // sub_1410864a0
    Replacement::draft(0x1410866d0, "unknown_1410866d0", fn_1410866d0 as *const ()), // sub_1410866d0
    Replacement::draft(0x141086840, "unknown_141086840", fn_141086840 as *const ()), // sub_141086840
    Replacement::draft(0x1410869b0, "unknown_1410869b0", fn_1410869b0 as *const ()), // sub_1410869b0
    Replacement::draft(0x141086b30, "unknown_141086b30", fn_141086b30 as *const ()), // sub_141086b30
    Replacement::draft(0x141086cb0, "unknown_141086cb0", fn_141086cb0 as *const ()), // sub_141086cb0
    Replacement::draft(0x141086e20, "unknown_141086e20", fn_141086e20 as *const ()), // sub_141086e20
    Replacement::draft(0x141086f90, "unknown_141086f90", fn_141086f90 as *const ()), // sub_141086f90
    Replacement::draft(0x141087130, "unknown_141087130", fn_141087130 as *const ()), // sub_141087130
    Replacement::draft(0x1410872a0, "unknown_1410872a0", fn_1410872a0 as *const ()), // sub_1410872a0
    Replacement::draft_throws(0x141087420, "coregame_141087420", tpl_1963::<0x143cd7e24, 0x143cd87a0, 0x1411e9f50, 0x141099690, 0x143cd8530, 0x1420af120, 0x1420a28e0, 0x141093d40, 0x141092240, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143378e16, 0x142646960, 0x143cd87b0, 0x143cd87b8, 0x143cd87bc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::light::copyPrevious>
    Replacement::draft_throws(0x141087700, "coregame_141087700", tpl_1404::<0x143cd8834, 0x143cd7ba0, 0x1411e9f50, 0x1410995a0, 0x143cd8b18, 0x1420af120, 0x1420a28e0, 0x141093ee0, 0x141092320, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337aa76, 0x142646960, 0x143cd7bc0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spotlight::copyPrevious>
    Replacement::draft_throws(0x1410879d0, "coregame_1410879d0", tpl_3367::<0x143cd8910, 0x143cd8c08, 0x14108ef40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337abe6, 0x142646960, 0x143cd8c28, 0x143cd8c30, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::light::updateFromGameParameters>
    Replacement::draft_throws(0x141087be0, "coregame_141087be0", tpl_5818::<0x143cd7c60, 0x143cd8f68, 0x1411e9f50, 0x141099350, 0x143cd89c0, 0x1420af120, 0x143c904a0, 0x140215610, 0x143c8d904, 0x1420a28e0, 0x1410940f0, 0x141092400, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337adf6, 0x142646960, 0x143cd8f88, 0x143cd8f8c, 0x143cd8f8e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::light::updateParameterBlends>
    Replacement::draft_throws(0x141087f30, "coregame_141087f30", fn_141087f30 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::flicker::updateFlicker>
    Replacement::draft_throws(0x141088260, "coregame_141088260", tpl_4436::<0x143cd8838, 0x143cd7f38, 0x14108f0f0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337a3b6, 0x142646960, 0x143cd7f58, 0x143cd7f60, 0x143cd7f64, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spotlight::updateFromGameParameters>
    Replacement::draft_throws(0x141088480, "coregame_141088480", fn_141088480 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spotlight::updateParameterBlends>
    Replacement::draft_throws(0x1410887d0, "coregame_1410887d0", tpl_4359::<0x143cd8584, 0x143cd8bb0, 0x14108f2a0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337a796, 0x142646960, 0x143cd8bd0, 0x143cd8bd8, 0x143cd8bdc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dirlight::updateFromGameParameters>
    Replacement::draft_throws(0x1410889e0, "coregame_1410889e0", fn_1410889e0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::pointlight::updateHidden>
    Replacement::draft_throws(0x141088cf0, "coregame_141088cf0", fn_141088cf0 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spotlight::updateHidden>
    Replacement::draft_throws(0x141088ff0, "coregame_141088ff0", tpl_1881::<0x143cd7be4, 0x143cd8230, 0x1411e9f50, 0x141098cc0, 0x143cd8cd8, 0x1420af120, 0x1420a28e0, 0x1410949c0, 0x1410928d0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x1433798d6, 0x142646960, 0x143cd8250, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dirlight::updateHidden>
    Replacement::draft_throws(0x1410892c0, "coregame_1410892c0", tpl_19880::<0x143cd7eec, 0x143cd8e20, 0x1411e9f50, 0x141098b20, 0x143cd85c4, 0x1420af120, 0x140215750, 0x1410933c0, 0x1420a28e0, 0x141094b60, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143379a56, 0x142646960, 0x143cd8e40, 0x143cd8e44, 0x143cd8e46, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::pointlight::updateToRenderer>
    Replacement::draft_throws(0x1410895f0, "coregame_1410895f0", tpl_19881::<0x143cd8c90, 0x143cd82b8, 0x1411e9f50, 0x1410988e0, 0x143cd8950, 0x1420af120, 0x140215750, 0x1410933c0, 0x1420a28e0, 0x141094bc0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143379e86, 0x142646960, 0x143cd82d8, 0x143cd82dc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spotlight::updateToRenderer>
    Replacement::draft_throws(0x141089920, "coregame_141089920", tpl_19806::<0x143cd8e54, 0x143cd83b0, 0x1411e9f50, 0x1410986d0, 0x143cd85c0, 0x1420af120, 0x1410933c0, 0x1420a28e0, 0x141094c20, 0x1410929b0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143378f86, 0x142646960, 0x143cd83d0, 0x143cd83d4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dirlight::updateToRenderer>
    Replacement::draft_throws(0x141089c10, "coregame_141089c10", tpl_3744::<0x143cd8e98, 0x143cd88e8, 0x1411e9f50, 0x141095fd0, 0x143cd852c, 0x1401fc320, 0x14027c6a0, 0x1420a28e0, 0x1410955c0, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143379476, 0x142646960, 0x143cd8908, 0x143cd890c, 0x143cd890e, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::pointlight::removePointLight>
    Replacement::draft_throws(0x141089ee0, "coregame_141089ee0", tpl_5490::<0x143cd8628, 0x143cd84b8, 0x1411e9f50, 0x141095f50, 0x143cd8a20, 0x1401fc320, 0x14027c6a0, 0x1420a28e0, 0x141095660, 0x1401de410, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x143379586, 0x142646960, 0x143cd84d8, 0x143cd84dc, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spotlight::removeSpotLight>
    Replacement::draft(0x14108a1a0, "coregame_14108a1a0", fn_14108a1a0 as *const ()), // ecs::system::ecs::system::wrapper<coregame::spotlight::removeSpotLightResources>
    Replacement::draft_throws(0x14108a460, "coregame_14108a460", fn_14108a460 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dirlight::removeDirectionalLight>
    Replacement::draft_throws(0x14108a730, "coregame_14108a730", fn_14108a730 as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dirlight::updateActiveLightToRenderer>
    Replacement::draft_throws(0x14108ac70, "coregame_14108ac70", fn_14108ac70 as *const (), Throws::Discard), // sub_14108ac70
    Replacement::draft_throws(0x14108ada0, "coregame_14108ada0", fn_14108ada0 as *const (), Throws::Discard), // sub_14108ada0
    Replacement::draft(0x14108aef0, "coregame_14108aef0", fn_14108aef0 as *const ()), // sub_14108aef0
    Replacement::draft(0x14108b060, "coregame_14108b060", tpl_1094::<0x143cd65b0, 0x141091840, 0x14020e8f0> as *const ()), // sub_14108b060
    Replacement::draft(0x14108b260, "coregame_14108b260", tpl_1096::<0x143cd8830, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108b260
    Replacement::draft(0x14108b590, "coregame_14108b590", tpl_1094::<0x143cd8490, 0x14068b330, 0x14020e8f0> as *const ()), // sub_14108b590
    Replacement::draft(0x14108b790, "coregame_14108b790", tpl_3080::<0x143cd852c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108b790
    Replacement::draft(0x14108bad0, "coregame_14108bad0", tpl_1402::<0x143c9d2f0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108bad0
    Replacement::draft(0x14108be20, "coregame_14108be20", tpl_1402::<0x143cd65b8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108be20
    Replacement::draft(0x14108c170, "coregame_14108c170", tpl_1094::<0x143cd7f8c, 0x1410919e0, 0x14020e8f0> as *const ()), // sub_14108c170
    Replacement::draft(0x14108c370, "coregame_14108c370", tpl_1098::<0x143cd65bc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108c370
    Replacement::draft(0x14108c6b0, "coregame_14108c6b0", tpl_1096::<0x143cd848c, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108c6b0
    Replacement::draft(0x14108c9e0, "coregame_14108c9e0", tpl_4324::<0x143cd65cc, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108c9e0
    Replacement::draft(0x14108cd20, "coregame_14108cd20", tpl_1435::<0x143cd65c8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108cd20
    Replacement::draft(0x14108d070, "coregame_14108d070", tpl_4324::<0x143ca9e48, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108d070
    Replacement::draft(0x14108d3b0, "coregame_14108d3b0", tpl_1094::<0x143cd8cc8, 0x141091c00, 0x14020e8f0> as *const ()), // sub_14108d3b0
    Replacement::draft(0x14108d5b0, "coregame_14108d5b0", tpl_4434::<0x143cd8a20, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108d5b0
    Replacement::draft(0x14108d900, "coregame_14108d900", tpl_1094::<0x143cd7f88, 0x141091db0, 0x14020e8f0> as *const ()), // sub_14108d900
    Replacement::draft(0x14108db00, "coregame_14108db00", tpl_3080::<0x143c9f8e8, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108db00
    Replacement::draft(0x14108de40, "coregame_14108de40", tpl_1096::<0x143cd65c0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108de40
    Replacement::draft(0x14108e170, "coregame_14108e170", tpl_4324::<0x143cd65d0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108e170
    Replacement::draft(0x14108e4b0, "coregame_14108e4b0", tpl_1094::<0x143cd7fa0, 0x14068b330, 0x14020e8f0> as *const ()), // sub_14108e4b0
    Replacement::draft(0x14108e6b0, "coregame_14108e6b0", tpl_4899::<0x143cd82e0, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108e6b0
    Replacement::draft(0x14108ea00, "coregame_14108ea00", tpl_1094::<0x143cd8954, 0x141092080, 0x14020e8f0> as *const ()), // sub_14108ea00
    Replacement::draft(0x14108ec00, "coregame_14108ec00", tpl_1099::<0x143cd65c4, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x14020e8f0> as *const ()), // sub_14108ec00
    Replacement::draft_throws(0x14108ef40, "coregame_14108ef40", tpl_3697::<0x1411e9f50, 0x141099490, 0x143cd89c4, 0x1420af120, 0x140215750, 0x143c8ce80, 0x140215610, 0x1420a28e0, 0x141094080, 0x1401de410> as *const (), Throws::Discard), // sub_14108ef40
    Replacement::draft_throws(0x14108f0f0, "coregame_14108f0f0", tpl_3697::<0x1411e9f50, 0x141099150, 0x143cd7f98, 0x1420af120, 0x140215750, 0x143c8ce80, 0x140215610, 0x1420a28e0, 0x141094590, 0x1401de410> as *const (), Throws::Discard), // sub_14108f0f0
    Replacement::draft_throws(0x14108f2a0, "coregame_14108f2a0", tpl_3697::<0x1411e9f50, 0x141098f90, 0x143cd7fa8, 0x1420af120, 0x140215750, 0x143c8ce80, 0x140215610, 0x1420a28e0, 0x141094890, 0x1401de410> as *const (), Throws::Discard), // sub_14108f2a0
    Replacement::draft_throws(0x14108f450, "coregame_14108f450", tpl_1896::<0x143cd7c64, 0x143cd8be0, 0x1411e9f50, 0x14109a290, 0x143cd7f94, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141097950, 0x141094e70, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337c2d6, 0x142646960, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::pointlight::streamOut>
    Replacement::draft_throws(0x14108f730, "coregame_14108f730", tpl_5492::<0x143cd8630, 0x143cd7f68, 0x1411e9f50, 0x14109a190, 0x143cd7fa4, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141097a40, 0x141094f00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337c516, 0x142646960, 0x143cd7f78, 0x143cd7f80, 0x143cd7f84, 0x143cd7f86, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spotlight::streamOut>
    Replacement::draft_throws(0x14108fa10, "coregame_14108fa10", tpl_3552::<0x143cd83dc, 0x143cd8e68, 0x1411e9f50, 0x14109a090, 0x143cd7f9c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141097b30, 0x141094f90, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337c756, 0x142646960, 0x143cd8e78, 0x143cd8e80, 0x143cd8e84, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dirlight::streamOut>
    Replacement::draft_throws(0x14108fcf0, "coregame_14108fcf0", tpl_5492::<0x143cd8cc4, 0x143cd8a28, 0x1411e9f50, 0x141099e50, 0x143cd8670, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141097c20, 0x141095020, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337c996, 0x142646960, 0x143cd8a38, 0x143cd8a40, 0x143cd8a44, 0x143cd8a46, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::pointlight::streamIn>
    Replacement::draft_throws(0x14108ffd0, "coregame_14108ffd0", tpl_1963::<0x143cd8cdc, 0x143cd8af8, 0x1411e9f50, 0x141099c10, 0x143cd8f24, 0x1420af120, 0x1420a28e0, 0x141097e20, 0x1410951e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337b6a6, 0x142646960, 0x143cd8b08, 0x143cd8b10, 0x143cd8b14, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::spotlight::streamIn>
    Replacement::draft_throws(0x1410902b0, "coregame_1410902b0", tpl_19279::<0x143cd8d3c, 0x143cd8ad8, 0x1411e9f50, 0x141099a50, 0x143cd8e8c, 0x1420af120, 0x14027c6a0, 0x1420a28e0, 0x141098020, 0x1410953e0, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337bb06, 0x142646960, 0x143cd8ae8, 0x143cd8af0, 0x143cd8af4, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::dirlight::streamIn>
    Replacement::draft(0x1410905a0, "coregame_1410905a0", tpl_20410::<0x1401e7c10, 0x141091590, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_1410905a0
    Replacement::draft_throws(0x1410907b0, "coregame_1410907b0", tpl_20411::<0x143cd8a48, 0x143cd8260, 0x141092b80, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337bf16, 0x142646960, 0x143cd8280, 0x143cd82a0, 0x143cd82b0, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::PointLightComponent
    Replacement::draft(0x1410909d0, "coregame_1410909d0", tpl_19710::<0x1401e7c10, 0x141091490, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_1410909d0
    Replacement::draft_throws(0x141090be0, "coregame_141090be0", tpl_19691::<0x143cd8ba0, 0x143cd85d0, 0x141092d40, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337c0f6, 0x142646960, 0x143cd85f0, 0x143cd8610, 0x143cd8620, 0x143cd8624, 0x143cd8626, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::SpotLightComponent>
    Replacement::draft(0x141090e20, "coregame_141090e20", tpl_20412::<0x1401e7c10, 0x141091390, 0x141e5c1b0, 0x1401de830> as *const ()), // sub_141090e20
    Replacement::draft_throws(0x141091030, "coregame_141091030", tpl_20413::<0x143cd7cc0, 0x143cd8dc0, 0x141092f00, 0x143c8d510, 0x14020afa0, 0x1420a6b70, 0x141834a78, 0x14337b306, 0x142646960, 0x143cd8de0, 0x143cd8e00, 0x143cd8e10, 0x143cd8e18, 0x143cd8e1c, 0x141834a0c> as *const (), Throws::Discard), // ecs::system::ecs::system::wrapper<coregame::system::gather_resources_from_osp::update<class content::DirectionalLightCom
    Replacement::draft(0x141091300, "unknown_141091300", tpl_20414::<0x8, 0x14108b060, 0x14108b260> as *const ()), // sub_141091300
    Replacement::draft(0x141091370, "unknown_141091370", fn_141091370 as *const ()), // sub_141091370
    Replacement::draft(0x141091690, "unknown_141091690", tpl_60953::<0x1426467e8, 0x143cd65c4, 0x142489ee0, 0x14109170c> as *const ()), // sub_141091690
    Replacement::draft(0x141091840, "unknown_141091840", tpl_4134::<0x1401e6d50, 0x140285d50, 0x142535070, 0x1426456c8, 0x1426453a0, 0x140213060, 0x143288b58> as *const ()), // sub_141091840
    Replacement::draft(0x1410919e0, "unknown_1410919e0", fn_1410919e0 as *const ()), // sub_1410919e0
];
